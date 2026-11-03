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
    C0(u64, u64, u64, u64, u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt3 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt4 {
    C0(u64, Bytes, Bytes, bool),
}

#[derive(Clone)]
pub enum Adt5 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt6 {
    C0(u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt7 {
    C0(bool, bool, List<(Bytes, Bytes)>),
}

#[derive(Clone)]
pub enum Adt8 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt9 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32, Adt8, u64),
}

#[derive(Clone)]
pub enum Adt10 {
    C0(u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt11 {
    C0(u64, u64, u64, u64, u64, u64, u64, u64, Adt10, u64),
}

#[derive(Clone)]
pub enum Adt12 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt13 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt14 {
    C0(Adt3, u64, List<Adt3>),
}

#[derive(Clone)]
pub enum Adt15 {
    C0(u32, u32, u32, u32),
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
    F11(Bytes),
    F91(Bytes),
    F181(u64, u64),
}

impl Fn1 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn1::F11(k0) => f11(k0.clone(), p0),
            Fn1::F91(k0) => f91(k0.clone(), p0),
            Fn1::F181(k0, k1) => f181(*k0, *k1, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F42(Bytes, u64, u64),
}

impl Fn2 {
    pub fn apply(&self, p0: (u64, Adt5)) -> R<(u64, Adt5)> {
        match self {
            Fn2::F42(k0, k1, k2) => f42(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F47(Bytes, u64, u64),
}

impl Fn3 {
    pub fn apply(&self, p0: (u64, Adt2)) -> R<(u64, Adt2)> {
        match self {
            Fn3::F47(k0, k1, k2) => f47(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F53,
}

impl Fn4 {
    pub fn apply(&self, p0: (List<Adt3>, Adt3)) -> (List<Adt3>, Adt3) {
        match self {
            Fn4::F53 => f53(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F56(Bytes),
}

impl Fn5 {
    pub fn apply(&self, p0: Adt4) -> R<Option<Adt4>> {
        match self {
            Fn5::F56(k0) => f56(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F59(Bytes),
}

impl Fn6 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn6::F59(k0) => f59(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F70(Bytes, u64, u64, u64),
    F149,
}

impl Fn7 {
    pub fn apply(&self, p0: (u64, Adt6)) -> R<(u64, Adt6)> {
        match self {
            Fn7::F70(k0, k1, k2, k3) => f70(k0.clone(), *k1, *k2, *k3, p0),
            Fn7::F149 => f149(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F77(Bytes, u64),
}

impl Fn8 {
    pub fn apply(&self, p0: (u64, List<Adt3>)) -> R<(u64, List<Adt3>)> {
        match self {
            Fn8::F77(k0, k1) => f77(k0.clone(), *k1, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F87(Bytes, Bytes),
}

impl Fn9 {
    pub fn apply(&self, p0: Adt7, p1: (Bytes, Bytes)) -> Adt7 {
        match self {
            Fn9::F87(k0, k1) => f87(k0.clone(), k1.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F93,
    F94,
}

impl Fn10 {
    pub fn apply(&self, p0: Option<Bytes>, p1: (Bytes, Bytes)) -> R<Option<Bytes>> {
        match self {
            Fn10::F93 => f93(p0, p1),
            Fn10::F94 => Ok(f94(p0, p1)),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F96,
}

impl Fn11 {
    pub fn apply(&self, p0: Adt9) -> R<Adt9> {
        match self {
            Fn11::F96 => f96(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F101,
}

impl Fn12 {
    pub fn apply(&self, p0: Adt11) -> R<Adt11> {
        match self {
            Fn12::F101 => f101(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F106,
}

impl Fn13 {
    pub fn apply(&self, p0: (Adt13, Adt12)) -> (Adt13, Adt12) {
        match self {
            Fn13::F106 => f106(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F108(u64, Bytes, u64),
}

impl Fn14 {
    pub fn apply(&self, p0: (u64, Adt3)) -> R<(u64, Adt3)> {
        match self {
            Fn14::F108(k0, k1, k2) => f108(*k0, k1.clone(), *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F112,
}

impl Fn15 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn15::F112 => f112(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F120,
}

impl Fn16 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn16::F120 => f120(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F135,
}

impl Fn17 {
    pub fn apply(&self, p0: Adt14) -> Adt14 {
        match self {
            Fn17::F135 => f135(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F159,
}

impl Fn18 {
    pub fn apply(&self, p0: (u64, (u64, u64))) -> (u64, (u64, u64)) {
        match self {
            Fn18::F159 => f159(p0),
        }
    }
}

pub fn f0(v0: Bytes, v1: Bytes) -> R<Option<Bytes>> {
    let m1 = f1(v1)?;
    match m1 {
        Ok(v2) => {
            f2(v0, v2)
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

pub fn f2(v0: Bytes, v1: Bytes) -> R<Option<Bytes>> {
    let m17 = f7(v0.clone(), v1)?;
    match m17 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v2) => {
            Ok(Some({
                let a18 = f8(v0);
                let a19 = v2;
                append_bytes(a18, a19)
            }))
        }
    }
}

pub fn f3(v0: Fn0, p24: u64, p25: Adt0) -> R<(Adt0, bool)> {
    let m23 = (p24, p25);
    {
        let mut m23 = m23;
        loop {
            let step = {
                let (v1, v2) = m23;
                let m20 = v1;
                if m20 == 0 {
                    Err::<(u64, Adt0), R<(Adt0, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m20 - 1;
                    let m22 = {
                        let c21: Fn0 = v0.clone();
                        c21.apply(v2.clone())?
                    };
                    match m22 {
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
                Ok(next) => { m23 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f4(v0: Bytes, v1: Adt0) -> R<Option<Adt0>> {
    f9(v0, v1)
}

pub fn f5(v0: Bytes, v1: u64) -> R<u64> {
    let (h26, _) = f10(Fn1::F11(v0.clone()), {
        let a27 = v0;
        length_bytes(a27)
    }, v1)?;
    Ok(h26)
}

pub fn f6(v0: Str) -> Bytes {
    let a28 = v0;
    utf8_encode(a28)
}

pub fn f7(v0: Bytes, v1: Bytes) -> R<Option<Bytes>> {
    if f12(v0.clone(), Str::lit("sha256")) {
        Ok(Some(f13(v1)?))
    } else {
        if f12(v0.clone(), Str::lit("sha3-256")) {
            Ok(Some(f14(v1)?))
        } else {
            if f12(v0.clone(), Str::lit("keccak256")) {
                Ok(Some(f15(v1)?))
            } else {
                if f12(v0.clone(), Str::lit("sha512")) {
                    Ok(Some(f16(v1)?))
                } else {
                    if f12(v0, Str::lit("blake3")) {
                        Ok(Some(f17(v1)?))
                    } else {
                        Ok(None::<Bytes>)
                    }
                }
            }
        }
    }
}

pub fn f8(v0: Bytes) -> Bytes {
    let a29 = v0;
    let a30 = f6(Str::lit(":"));
    append_bytes(a29, a30)
}

pub fn f9(v0: Bytes, v1: Adt0) -> R<Option<Adt0>> {
    let a33 = 0u64;
    let a34 = {
        let a32 = match v1.clone() {
            Adt0::C0(_, _, _, h31) => {
                h31
            }
        };
        length_bytes(a32)
    };
    if nat_lt(a33, a34) {
        Ok(None::<Adt0>)
    } else {
        let m36 = match v1.clone() {
            Adt0::C0(_, _, h35, _) => {
                h35
            }
        };
        match m36 {
            None => {
                let v3: u64 = f5(v0.clone(), match v1.clone() {
                    Adt0::C0(h37, _, _, _) => {
                        h37
                    }
                })?;
                let v4: u64 = f18(v0.clone(), v3)?;
                let m39 = match v1.clone() {
                    Adt0::C0(_, h38, _, _) => {
                        h38
                    }
                };
                match m39.uncons() {
                    None => {
                        Ok(Some(f19(v1, v0, v3, v4)?))
                    }
                    Some((v5, v6)) => {
                        let v7: u64 = match v5.clone() {
                            Adt1::C0(_, h40, _, _) => {
                                h40
                            }
                        };
                        let v8: bool = if match v5.clone() {
                            Adt1::C0(h41, _, _, _) => {
                                h41
                            }
                        } {
                            let a42 = v7;
                            let a43 = 2u64;
                            nat_eq(a42, a43)
                        } else {
                            let a44 = v7;
                            let a45 = 2u64;
                            if nat_eq(a44, a45) {
                                true
                            } else {
                                let a46 = v7;
                                let a47 = 0u64;
                                if nat_eq(a46, a47) {
                                    let a50 = {
                                        let a48 = v4;
                                        let a49 = 93u64;
                                        nat_eq(a48, a49)
                                    };
                                    bool_not(a50)
                                } else {
                                    false
                                }
                            }
                        };
                        if v8 {
                            Ok(Some(f19(v1, v0, v3, v4)?))
                        } else {
                            Ok(Some(f20(v1, v0, v5, v6, v3, v4)?))
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

pub fn f10(v0: Fn1, p55: u64, p56: u64) -> R<(u64, bool)> {
    let m54 = (p55, p56);
    {
        let mut m54 = m54;
        loop {
            let step = {
                let (v1, v2) = m54;
                let m51 = v1;
                if m51 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m51 - 1;
                    let m53 = {
                        let c52: Fn1 = v0.clone();
                        c52.apply(v2)?
                    };
                    match m53 {
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
                Ok(next) => { m54 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f11(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f21(f18(v0, v1)?) {
        Ok(Some({
            let a57 = v1;
            let a58 = 1u64;
            nat_add(a57, a58)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f12(v0: Bytes, v1: Str) -> bool {
    let a59 = v0;
    let a60 = f6(v1);
    equal(a59, a60)
}

pub fn f13(v0: Bytes) -> R<Bytes> {
    Ok(f22(f23(v0)?))
}

pub fn f14(v0: Bytes) -> R<Bytes> {
    f24(v0, 6u64)
}

pub fn f15(v0: Bytes) -> R<Bytes> {
    f24(v0, 1u64)
}

pub fn f16(v0: Bytes) -> R<Bytes> {
    let v1: Adt2 = f25(v0)?;
    let a81 = f26(match v1.clone() {
        Adt2::C0(h61, _, _, _, _, _, _, _) => {
            h61
        }
    });
    let a82 = {
        let a79 = f26(match v1.clone() {
            Adt2::C0(_, h62, _, _, _, _, _, _) => {
                h62
            }
        });
        let a80 = {
            let a77 = f26(match v1.clone() {
                Adt2::C0(_, _, h63, _, _, _, _, _) => {
                    h63
                }
            });
            let a78 = {
                let a75 = f26(match v1.clone() {
                    Adt2::C0(_, _, _, h64, _, _, _, _) => {
                        h64
                    }
                });
                let a76 = {
                    let a73 = f26(match v1.clone() {
                        Adt2::C0(_, _, _, _, h65, _, _, _) => {
                            h65
                        }
                    });
                    let a74 = {
                        let a71 = f26(match v1.clone() {
                            Adt2::C0(_, _, _, _, _, h66, _, _) => {
                                h66
                            }
                        });
                        let a72 = {
                            let a69 = f26(match v1.clone() {
                                Adt2::C0(_, _, _, _, _, _, h67, _) => {
                                    h67
                                }
                            });
                            let a70 = f26(match v1 {
                                Adt2::C0(_, _, _, _, _, _, _, h68) => {
                                    h68
                                }
                            });
                            append_bytes(a69, a70)
                        };
                        append_bytes(a71, a72)
                    };
                    append_bytes(a73, a74)
                };
                append_bytes(a75, a76)
            };
            append_bytes(a77, a78)
        };
        append_bytes(a79, a80)
    };
    Ok(append_bytes(a81, a82))
}

pub fn f17(v0: Bytes) -> R<Bytes> {
    let v1: Adt3 = f27(v0)?;
    let a103 = f28(match v1.clone() {
        Adt3::C0(h83, _, _, _, _, _, _, _) => {
            h83
        }
    });
    let a104 = {
        let a101 = f28(match v1.clone() {
            Adt3::C0(_, h84, _, _, _, _, _, _) => {
                h84
            }
        });
        let a102 = {
            let a99 = f28(match v1.clone() {
                Adt3::C0(_, _, h85, _, _, _, _, _) => {
                    h85
                }
            });
            let a100 = {
                let a97 = f28(match v1.clone() {
                    Adt3::C0(_, _, _, h86, _, _, _, _) => {
                        h86
                    }
                });
                let a98 = {
                    let a95 = f28(match v1.clone() {
                        Adt3::C0(_, _, _, _, h87, _, _, _) => {
                            h87
                        }
                    });
                    let a96 = {
                        let a93 = f28(match v1.clone() {
                            Adt3::C0(_, _, _, _, _, h88, _, _) => {
                                h88
                            }
                        });
                        let a94 = {
                            let a91 = f28(match v1.clone() {
                                Adt3::C0(_, _, _, _, _, _, h89, _) => {
                                    h89
                                }
                            });
                            let a92 = f28(match v1 {
                                Adt3::C0(_, _, _, _, _, _, _, h90) => {
                                    h90
                                }
                            });
                            append_bytes(a91, a92)
                        };
                        append_bytes(a93, a94)
                    };
                    append_bytes(a95, a96)
                };
                append_bytes(a97, a98)
            };
            append_bytes(a99, a100)
        };
        append_bytes(a101, a102)
    };
    Ok(append_bytes(a103, a104))
}

pub fn f18(v0: Bytes, v1: u64) -> R<u64> {
    let m107 = {
        let a105 = v0;
        let a106 = v1;
        index_bytes(a105, a106)
    };
    match m107 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a152 = {
                let a128 = {
                    let a116 = {
                        let a110 = {
                            let a108 = v3;
                            let a109 = 128u8;
                            fixed_u8::bit_and(a108, a109)
                        };
                        let a111 = 128u8;
                        if equal(a110, a111) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a117 = {
                        let a114 = {
                            let a112 = v3;
                            let a113 = 64u8;
                            fixed_u8::bit_and(a112, a113)
                        };
                        let a115 = 64u8;
                        if equal(a114, a115) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a116, a117)?
                };
                let a129 = {
                    let a126 = {
                        let a120 = {
                            let a118 = v3;
                            let a119 = 32u8;
                            fixed_u8::bit_and(a118, a119)
                        };
                        let a121 = 32u8;
                        if equal(a120, a121) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a127 = {
                        let a124 = {
                            let a122 = v3;
                            let a123 = 16u8;
                            fixed_u8::bit_and(a122, a123)
                        };
                        let a125 = 16u8;
                        if equal(a124, a125) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a126, a127)?
                };
                nat_add(a128, a129)?
            };
            let a153 = {
                let a150 = {
                    let a138 = {
                        let a132 = {
                            let a130 = v3;
                            let a131 = 8u8;
                            fixed_u8::bit_and(a130, a131)
                        };
                        let a133 = 8u8;
                        if equal(a132, a133) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a139 = {
                        let a136 = {
                            let a134 = v3;
                            let a135 = 4u8;
                            fixed_u8::bit_and(a134, a135)
                        };
                        let a137 = 4u8;
                        if equal(a136, a137) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a138, a139)?
                };
                let a151 = {
                    let a148 = {
                        let a142 = {
                            let a140 = v3;
                            let a141 = 2u8;
                            fixed_u8::bit_and(a140, a141)
                        };
                        let a143 = 2u8;
                        if equal(a142, a143) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a149 = {
                        let a146 = {
                            let a144 = v3;
                            let a145 = 1u8;
                            fixed_u8::bit_and(a144, a145)
                        };
                        let a147 = 1u8;
                        if equal(a146, a147) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a148, a149)?
                };
                nat_add(a150, a151)?
            };
            nat_add(a152, a153)
        }
    }
}

pub fn f19(v0: Adt0, v1: Bytes, v2: u64, v3: u64) -> R<Adt0> {
    let a154 = v3;
    let a155 = 123u64;
    if nat_eq(a154, a155) {
        Ok(f29(v0, true, {
            let a156 = v2;
            let a157 = 1u64;
            nat_add(a156, a157)?
        }))
    } else {
        let a158 = v3;
        let a159 = 91u64;
        if nat_eq(a158, a159) {
            Ok(f29(v0, false, {
                let a160 = v2;
                let a161 = 1u64;
                nat_add(a160, a161)?
            }))
        } else {
            let a162 = v3;
            let a163 = 34u64;
            if nat_eq(a162, a163) {
                let v4: Adt4 = f30(v1, v2)?;
                if match v4.clone() {
                    Adt4::C0(_, _, _, h164) => {
                        h164
                    }
                } {
                    Ok(f31(v0.clone(), match v0 {
                        Adt0::C0(_, h165, _, _) => {
                            h165
                        }
                    }, f32(match v4.clone() {
                        Adt4::C0(_, h166, _, _) => {
                            h166
                        }
                    })?, match v4 {
                        Adt4::C0(h167, _, _, _) => {
                            h167
                        }
                    }))
                } else {
                    Ok(f33(v0, match v4 {
                        Adt4::C0(_, _, h168, _) => {
                            h168
                        }
                    }))
                }
            } else {
                let a169 = v3;
                let a170 = 116u64;
                if nat_eq(a169, a170) {
                    f34(v0, v1, v2, Bytes::lit(&[0x74, 0x72, 0x75, 0x65]), Bytes::lit(&[0x74, 0x72, 0x75, 0x65]))
                } else {
                    let a171 = v3;
                    let a172 = 102u64;
                    if nat_eq(a171, a172) {
                        f34(v0, v1, v2, Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]), Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]))
                    } else {
                        let a173 = v3;
                        let a174 = 110u64;
                        if nat_eq(a173, a174) {
                            f34(v0, v1, v2, Bytes::lit(&[0x6e, 0x75, 0x6c, 0x6c]), Bytes::lit(&[0x6e, 0x75, 0x6c, 0x6c]))
                        } else {
                            let a175 = v3;
                            let a176 = 45u64;
                            if nat_eq(a175, a176) {
                                f35(v0, v1, v2)
                            } else {
                                let a177 = 47u64;
                                let a178 = v3;
                                if nat_lt(a177, a178) {
                                    let a179 = v3;
                                    let a180 = 58u64;
                                    if nat_lt(a179, a180) {
                                        f35(v0, v1, v2)
                                    } else {
                                        Ok(f33(v0, f6(Str::lit("unexpected input"))))
                                    }
                                } else {
                                    Ok(f33(v0, f6(Str::lit("unexpected input"))))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f20(v0: Adt0, v1: Bytes, v2: Adt1, v3: List<Adt1>, v4: u64, v5: u64) -> R<Adt0> {
    let v6: u64 = match v2.clone() {
        Adt1::C0(_, h181, _, _) => {
            h181
        }
    };
    if match v2.clone() {
        Adt1::C0(h182, _, _, _) => {
            h182
        }
    } {
        let a183 = v5;
        let a184 = 125u64;
        if nat_eq(a183, a184) {
            let a185 = v6;
            let a186 = 0u64;
            if nat_eq(a185, a186) {
                f36(v0, v2, v3, {
                    let a187 = v4;
                    let a188 = 1u64;
                    nat_add(a187, a188)?
                })
            } else {
                let a189 = v6;
                let a190 = 3u64;
                if nat_eq(a189, a190) {
                    f36(v0, v2, v3, {
                        let a191 = v4;
                        let a192 = 1u64;
                        nat_add(a191, a192)?
                    })
                } else {
                    Ok(f33(v0, f6(Str::lit("unexpected input"))))
                }
            }
        } else {
            let a193 = v5;
            let a194 = 34u64;
            if nat_eq(a193, a194) {
                let a195 = v6;
                let a196 = 0u64;
                if nat_eq(a195, a196) {
                    f37(v0, v1, v2, v3, v4)
                } else {
                    let a197 = v6;
                    let a198 = 4u64;
                    if nat_eq(a197, a198) {
                        f37(v0, v1, v2, v3, v4)
                    } else {
                        Ok(f33(v0, f6(Str::lit("unexpected input"))))
                    }
                }
            } else {
                let a199 = v5;
                let a200 = 58u64;
                if nat_eq(a199, a200) {
                    let a201 = v6;
                    let a202 = 1u64;
                    if nat_eq(a201, a202) {
                        Ok(f38(v0, f39(v2.clone(), 2u64, match v2 {
                            Adt1::C0(_, _, _, h203) => {
                                h203
                            }
                        }), v3, {
                            let a204 = v4;
                            let a205 = 1u64;
                            nat_add(a204, a205)?
                        }))
                    } else {
                        Ok(f33(v0, f6(Str::lit("unexpected input"))))
                    }
                } else {
                    let a206 = v5;
                    let a207 = 44u64;
                    if nat_eq(a206, a207) {
                        let a208 = v6;
                        let a209 = 3u64;
                        if nat_eq(a208, a209) {
                            Ok(f38(v0, f39(v2, 4u64, Bytes::lit(&[])), v3, {
                                let a210 = v4;
                                let a211 = 1u64;
                                nat_add(a210, a211)?
                            }))
                        } else {
                            Ok(f33(v0, f6(Str::lit("unexpected input"))))
                        }
                    } else {
                        Ok(f33(v0, f6(Str::lit("unexpected input"))))
                    }
                }
            }
        }
    } else {
        let a212 = v5;
        let a213 = 93u64;
        if nat_eq(a212, a213) {
            f36(v0, v2, v3, {
                let a214 = v4;
                let a215 = 1u64;
                nat_add(a214, a215)?
            })
        } else {
            let a216 = v5;
            let a217 = 44u64;
            if nat_eq(a216, a217) {
                let a218 = v6;
                let a219 = 1u64;
                if nat_eq(a218, a219) {
                    Ok(f38(v0, f39(v2, 2u64, Bytes::lit(&[])), v3, {
                        let a220 = v4;
                        let a221 = 1u64;
                        nat_add(a220, a221)?
                    }))
                } else {
                    Ok(f33(v0, f6(Str::lit("unexpected input"))))
                }
            } else {
                Ok(f33(v0, f6(Str::lit("unexpected input"))))
            }
        }
    }
}

pub fn f21(v0: u64) -> bool {
    let a222 = v0;
    let a223 = 32u64;
    if nat_eq(a222, a223) {
        true
    } else {
        let a224 = v0;
        let a225 = 10u64;
        if nat_eq(a224, a225) {
            true
        } else {
            let a226 = v0;
            let a227 = 13u64;
            if nat_eq(a226, a227) {
                true
            } else {
                let a228 = v0;
                let a229 = 9u64;
                nat_eq(a228, a229)
            }
        }
    }
}

pub fn f22(v0: Adt5) -> Bytes {
    let a250 = f40(match v0.clone() {
        Adt5::C0(h230, _, _, _, _, _, _, _) => {
            h230
        }
    });
    let a251 = {
        let a248 = f40(match v0.clone() {
            Adt5::C0(_, h231, _, _, _, _, _, _) => {
                h231
            }
        });
        let a249 = {
            let a246 = f40(match v0.clone() {
                Adt5::C0(_, _, h232, _, _, _, _, _) => {
                    h232
                }
            });
            let a247 = {
                let a244 = f40(match v0.clone() {
                    Adt5::C0(_, _, _, h233, _, _, _, _) => {
                        h233
                    }
                });
                let a245 = {
                    let a242 = f40(match v0.clone() {
                        Adt5::C0(_, _, _, _, h234, _, _, _) => {
                            h234
                        }
                    });
                    let a243 = {
                        let a240 = f40(match v0.clone() {
                            Adt5::C0(_, _, _, _, _, h235, _, _) => {
                                h235
                            }
                        });
                        let a241 = {
                            let a238 = f40(match v0.clone() {
                                Adt5::C0(_, _, _, _, _, _, h236, _) => {
                                    h236
                                }
                            });
                            let a239 = f40(match v0 {
                                Adt5::C0(_, _, _, _, _, _, _, h237) => {
                                    h237
                                }
                            });
                            append_bytes(a238, a239)
                        };
                        append_bytes(a240, a241)
                    };
                    append_bytes(a242, a243)
                };
                append_bytes(a244, a245)
            };
            append_bytes(a246, a247)
        };
        append_bytes(a248, a249)
    };
    append_bytes(a250, a251)
}

pub fn f23(v0: Bytes) -> R<Adt5> {
    let v1: u64 = {
        let a252 = v0.clone();
        length_bytes(a252)
    };
    let v2: u64 = {
        let a258 = {
            let a255 = {
                let a253 = v1;
                let a254 = 72u64;
                nat_add(a253, a254)?
            };
            let a256 = 64u64;
            let a257 = 0u64;
            nat_quot(a255, a256, a257)
        };
        let a259 = 64u64;
        nat_mul(a258, a259)?
    };
    let (_, h260) = f41(Fn2::F42(v0, v1, v2), {
        let a261 = v2;
        let a262 = 64u64;
        let a263 = 0u64;
        nat_quot(a261, a262, a263)
    }, (0u64, f43()))?;
    Ok(h260)
}

pub fn f24(v0: Bytes, v1: u64) -> R<Bytes> {
    let v2: Adt6 = f44(v0, v1)?;
    let a272 = f45(match v2.clone() {
        Adt6::C0(h264, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h264
        }
    });
    let a273 = {
        let a270 = f45(match v2.clone() {
            Adt6::C0(_, h265, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h265
            }
        });
        let a271 = {
            let a268 = f45(match v2.clone() {
                Adt6::C0(_, _, h266, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h266
                }
            });
            let a269 = f45(match v2 {
                Adt6::C0(_, _, _, h267, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h267
                }
            });
            append_bytes(a268, a269)
        };
        append_bytes(a270, a271)
    };
    Ok(append_bytes(a272, a273))
}

pub fn f25(v0: Bytes) -> R<Adt2> {
    let v1: u64 = {
        let a274 = v0.clone();
        length_bytes(a274)
    };
    let v2: u64 = {
        let a280 = {
            let a277 = {
                let a275 = v1;
                let a276 = 144u64;
                nat_add(a275, a276)?
            };
            let a278 = 128u64;
            let a279 = 0u64;
            nat_quot(a277, a278, a279)
        };
        let a281 = 128u64;
        nat_mul(a280, a281)?
    };
    let (_, h282) = f46(Fn3::F47(v0, v1, v2), {
        let a283 = v2;
        let a284 = 128u64;
        let a285 = 0u64;
        nat_quot(a283, a284, a285)
    }, (0u64, f48()))?;
    Ok(h282)
}

pub fn f26(v0: u64) -> Bytes {
    let a314 = f49(v0, 60u32);
    let a315 = {
        let a312 = f49(v0, 56u32);
        let a313 = {
            let a310 = f49(v0, 52u32);
            let a311 = {
                let a308 = f49(v0, 48u32);
                let a309 = {
                    let a306 = f49(v0, 44u32);
                    let a307 = {
                        let a304 = f49(v0, 40u32);
                        let a305 = {
                            let a302 = f49(v0, 36u32);
                            let a303 = {
                                let a300 = f49(v0, 32u32);
                                let a301 = {
                                    let a298 = f49(v0, 28u32);
                                    let a299 = {
                                        let a296 = f49(v0, 24u32);
                                        let a297 = {
                                            let a294 = f49(v0, 20u32);
                                            let a295 = {
                                                let a292 = f49(v0, 16u32);
                                                let a293 = {
                                                    let a290 = f49(v0, 12u32);
                                                    let a291 = {
                                                        let a288 = f49(v0, 8u32);
                                                        let a289 = {
                                                            let a286 = f49(v0, 4u32);
                                                            let a287 = f49(v0, 0u32);
                                                            append_bytes(a286, a287)
                                                        };
                                                        append_bytes(a288, a289)
                                                    };
                                                    append_bytes(a290, a291)
                                                };
                                                append_bytes(a292, a293)
                                            };
                                            append_bytes(a294, a295)
                                        };
                                        append_bytes(a296, a297)
                                    };
                                    append_bytes(a298, a299)
                                };
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
    };
    append_bytes(a314, a315)
}

pub fn f27(v0: Bytes) -> R<Adt3> {
    let v1: u64 = {
        let a316 = v0.clone();
        length_bytes(a316)
    };
    let v2: u64 = {
        let a317 = v1;
        let a318 = 0u64;
        if nat_eq(a317, a318) {
            1u64
        } else {
            let a321 = {
                let a319 = v1;
                let a320 = 1023u64;
                nat_add(a319, a320)?
            };
            let a322 = 1024u64;
            let a323 = 0u64;
            nat_quot(a321, a322, a323)
        }
    };
    let v3: u64 = {
        let a324 = v2;
        let a325 = 1u64;
        nat_sub(a324, a325)
    };
    let m327 = {
        let (_, h326) = f50(v0.clone(), v1, v3)?;
        h326
    };
    match m327.uncons() {
        None => {
            f51(v0, v1, v3, true)
        }
        Some((v4, v5)) => {
            let (_, h328) = f52(Fn4::F53, 64u64, (List::cons(v4, v5), f51(v0, v1, v3, false)?));
            Ok(h328)
        }
    }
}

pub fn f28(v0: u32) -> Bytes {
    let a341 = {
        let a329 = f54(v0, 4u32);
        let a330 = f54(v0, 0u32);
        append_bytes(a329, a330)
    };
    let a342 = {
        let a339 = {
            let a331 = f54(v0, 12u32);
            let a332 = f54(v0, 8u32);
            append_bytes(a331, a332)
        };
        let a340 = {
            let a337 = {
                let a333 = f54(v0, 20u32);
                let a334 = f54(v0, 16u32);
                append_bytes(a333, a334)
            };
            let a338 = {
                let a335 = f54(v0, 28u32);
                let a336 = f54(v0, 24u32);
                append_bytes(a335, a336)
            };
            append_bytes(a337, a338)
        };
        append_bytes(a339, a340)
    };
    append_bytes(a341, a342)
}

pub fn f29(v0: Adt0, v1: bool, v2: u64) -> Adt0 {
    Adt0::C0(v2, List::cons(Adt1::C0(v1, 0u64, List::<(Bytes, Bytes)>::nil(), Bytes::lit(&[])), match v0 {
        Adt0::C0(_, h343, _, _) => {
            h343
        }
    }), None::<Bytes>, Bytes::lit(&[]))
}

pub fn f30(v0: Bytes, v1: u64) -> R<Adt4> {
    let (h344, _) = f55(Fn5::F56(v0.clone()), {
        let a346 = {
            let a345 = v0;
            length_bytes(a345)
        };
        let a347 = 2u64;
        nat_add(a346, a347)?
    }, Adt4::C0({
        let a348 = v1;
        let a349 = 1u64;
        nat_add(a348, a349)?
    }, Bytes::lit(&[]), Bytes::lit(&[]), false))?;
    Ok(h344)
}

pub fn f31(v0: Adt0, v1: List<Adt1>, v2: Bytes, v3: u64) -> Adt0 {
    let m350 = v1;
    match m350.uncons() {
        None => {
            Adt0::C0(v3, List::<Adt1>::nil(), Some(v2), Bytes::lit(&[]))
        }
        Some((v4, v5)) => {
            if match v4.clone() {
                Adt1::C0(h351, _, _, _) => {
                    h351
                }
            } {
                let m354 = f57(match v4.clone() {
                    Adt1::C0(_, _, _, h352) => {
                        h352
                    }
                }, v2, match v4 {
                    Adt1::C0(_, _, h353, _) => {
                        h353
                    }
                });
                match m354 {
                    None => {
                        f33(v0, f6(Str::lit("duplicate member name")))
                    }
                    Some(v6) => {
                        Adt0::C0(v3, List::cons(Adt1::C0(true, 3u64, v6, Bytes::lit(&[])), v5), None::<Bytes>, Bytes::lit(&[]))
                    }
                }
            } else {
                Adt0::C0(v3, List::cons(Adt1::C0(false, 1u64, List::cons((Bytes::lit(&[]), v2), match v4 {
                    Adt1::C0(_, _, h355, _) => {
                        h355
                    }
                }), Bytes::lit(&[])), v5), None::<Bytes>, Bytes::lit(&[]))
            }
        }
    }
}

pub fn f32(v0: Bytes) -> R<Bytes> {
    let a363 = Bytes::lit(&[0x22]);
    let a364 = {
        let a361 = {
            let (_, h356) = {
                let (h357, _) = f58(Fn6::F59(v0.clone()), {
                    let a359 = {
                        let a358 = v0;
                        length_bytes(a358)
                    };
                    let a360 = 1u64;
                    nat_add(a359, a360)?
                }, (0u64, Bytes::lit(&[])))?;
                h357
            };
            h356
        };
        let a362 = Bytes::lit(&[0x22]);
        append_bytes(a361, a362)
    };
    Ok(append_bytes(a363, a364))
}

pub fn f33(v0: Adt0, v1: Bytes) -> Adt0 {
    Adt0::C0(match v0.clone() {
        Adt0::C0(h365, _, _, _) => {
            h365
        }
    }, match v0 {
        Adt0::C0(_, h366, _, _) => {
            h366
        }
    }, None::<Bytes>, v1)
}

pub fn f34(v0: Adt0, v1: Bytes, v2: u64, v3: Bytes, v4: Bytes) -> R<Adt0> {
    if f60(v1, v3.clone(), v2) {
        Ok(f31(v0.clone(), match v0 {
            Adt0::C0(_, h367, _, _) => {
                h367
            }
        }, v4, {
            let a369 = v2;
            let a370 = {
                let a368 = v3;
                length_bytes(a368)
            };
            nat_add(a369, a370)?
        }))
    } else {
        Ok(f33(v0, f6(Str::lit("bad literal"))))
    }
}

pub fn f35(v0: Adt0, v1: Bytes, v2: u64) -> R<Adt0> {
    let v3: bool = {
        let a371 = f18(v1.clone(), v2)?;
        let a372 = 45u64;
        nat_eq(a371, a372)
    };
    let v4: u64 = if v3 {
        let a373 = v2;
        let a374 = 1u64;
        nat_add(a373, a374)?
    } else {
        v2
    };
    let v5: u64 = f18(v1.clone(), v4)?;
    let a375 = v5;
    let a376 = 48u64;
    if nat_eq(a375, a376) {
        let v6: u64 = {
            let a377 = v4;
            let a378 = 1u64;
            nat_add(a377, a378)?
        };
        f61(v0, v1, v2, v6, v3)
    } else {
        let a379 = 48u64;
        let a380 = v5;
        if nat_lt(a379, a380) {
            let a381 = v5;
            let a382 = 58u64;
            if nat_lt(a381, a382) {
                f61(v0, v1.clone(), v2, f62(v1, {
                    let a383 = v4;
                    let a384 = 1u64;
                    nat_add(a383, a384)?
                })?, false)
            } else {
                Ok(f33(v0, f6(Str::lit("bad number"))))
            }
        } else {
            Ok(f33(v0, f6(Str::lit("bad number"))))
        }
    }
}

pub fn f36(v0: Adt0, v1: Adt1, v2: List<Adt1>, v3: u64) -> R<Adt0> {
    if match v1.clone() {
        Adt1::C0(h385, _, _, _) => {
            h385
        }
    } {
        Ok(f31(v0, v2, {
            let a389 = Bytes::lit(&[0x7b]);
            let a390 = {
                let a387 = f63(match v1 {
                    Adt1::C0(_, _, h386, _) => {
                        h386
                    }
                })?;
                let a388 = Bytes::lit(&[0x7d]);
                append_bytes(a387, a388)
            };
            append_bytes(a389, a390)
        }, v3))
    } else {
        Ok(f31(v0, v2, {
            let a394 = Bytes::lit(&[0x5b]);
            let a395 = {
                let a392 = f64(match v1 {
                    Adt1::C0(_, _, h391, _) => {
                        h391
                    }
                })?;
                let a393 = Bytes::lit(&[0x5d]);
                append_bytes(a392, a393)
            };
            append_bytes(a394, a395)
        }, v3))
    }
}

pub fn f37(v0: Adt0, v1: Bytes, v2: Adt1, v3: List<Adt1>, v4: u64) -> R<Adt0> {
    let v5: Adt4 = f30(v1, v4)?;
    if match v5.clone() {
        Adt4::C0(_, _, _, h396) => {
            h396
        }
    } {
        Ok(f38(v0, f39(v2, 1u64, match v5.clone() {
            Adt4::C0(_, h397, _, _) => {
                h397
            }
        }), v3, match v5 {
            Adt4::C0(h398, _, _, _) => {
                h398
            }
        }))
    } else {
        Ok(f33(v0, match v5 {
            Adt4::C0(_, _, h399, _) => {
                h399
            }
        }))
    }
}

pub fn f38(_: Adt0, v1: Adt1, v2: List<Adt1>, v3: u64) -> Adt0 {
    Adt0::C0(v3, List::cons(v1, v2), None::<Bytes>, Bytes::lit(&[]))
}

pub fn f39(v0: Adt1, v1: u64, v2: Bytes) -> Adt1 {
    Adt1::C0(match v0.clone() {
        Adt1::C0(h400, _, _, _) => {
            h400
        }
    }, v1, match v0 {
        Adt1::C0(_, _, h401, _) => {
            h401
        }
    }, v2)
}

pub fn f40(v0: u32) -> Bytes {
    let a430 = f65({
        let a402 = f66(v0, 28u32);
        let a403 = 15u32;
        fixed_u32::bit_and(a402, a403)
    });
    let a431 = {
        let a428 = f65({
            let a404 = f66(v0, 24u32);
            let a405 = 15u32;
            fixed_u32::bit_and(a404, a405)
        });
        let a429 = {
            let a426 = f65({
                let a406 = f66(v0, 20u32);
                let a407 = 15u32;
                fixed_u32::bit_and(a406, a407)
            });
            let a427 = {
                let a424 = f65({
                    let a408 = f66(v0, 16u32);
                    let a409 = 15u32;
                    fixed_u32::bit_and(a408, a409)
                });
                let a425 = {
                    let a422 = f65({
                        let a410 = f66(v0, 12u32);
                        let a411 = 15u32;
                        fixed_u32::bit_and(a410, a411)
                    });
                    let a423 = {
                        let a420 = f65({
                            let a412 = f66(v0, 8u32);
                            let a413 = 15u32;
                            fixed_u32::bit_and(a412, a413)
                        });
                        let a421 = {
                            let a418 = f65({
                                let a414 = f66(v0, 4u32);
                                let a415 = 15u32;
                                fixed_u32::bit_and(a414, a415)
                            });
                            let a419 = f65({
                                let a416 = v0;
                                let a417 = 15u32;
                                fixed_u32::bit_and(a416, a417)
                            });
                            append_bytes(a418, a419)
                        };
                        append_bytes(a420, a421)
                    };
                    append_bytes(a422, a423)
                };
                append_bytes(a424, a425)
            };
            append_bytes(a426, a427)
        };
        append_bytes(a428, a429)
    };
    append_bytes(a430, a431)
}

pub fn f41(v0: Fn2, p435: u64, p436: (u64, Adt5)) -> R<(u64, Adt5)> {
    let m434 = (p435, p436);
    {
        let mut m434 = m434;
        loop {
            let step = {
                let (v1, v2) = m434;
                let m432 = v1;
                if m432 == 0 {
                    Err::<(u64, (u64, Adt5)), R<(u64, Adt5)>>(Ok(v2))
                } else {
                    let v3 = m432 - 1;
                    Ok::<(u64, (u64, Adt5)), R<(u64, Adt5)>>((v3, {
                        let c433: Fn2 = v0.clone();
                        c433.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m434 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f42(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt5)) -> R<(u64, Adt5)> {
    Ok(({
        let a438 = {
            let (h437, _) = v3.clone();
            h437
        };
        let a439 = 64u64;
        nat_add(a438, a439)?
    }, f67({
        let (_, h440) = v3;
        h440
    }, f68(v0, v1, v2, {
        let (h441, _) = v3;
        h441
    })?)?))
}

pub fn f43() -> Adt5 {
    Adt5::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f44(v0: Bytes, v1: u64) -> R<Adt6> {
    let v2: u64 = {
        let a442 = v0.clone();
        length_bytes(a442)
    };
    let v3: u64 = {
        let a448 = {
            let a446 = {
                let a443 = v2;
                let a444 = 136u64;
                let a445 = 0u64;
                nat_quot(a443, a444, a445)
            };
            let a447 = 1u64;
            nat_add(a446, a447)?
        };
        let a449 = 136u64;
        nat_mul(a448, a449)?
    };
    let (_, h450) = f69(Fn7::F70(v0, v2, v1, v3), {
        let a451 = v3;
        let a452 = 136u64;
        let a453 = 0u64;
        nat_quot(a451, a452, a453)
    }, (0u64, f71()))?;
    Ok(h450)
}

pub fn f45(v0: u64) -> Bytes {
    let a482 = {
        let a454 = f49(v0, 4u32);
        let a455 = f49(v0, 0u32);
        append_bytes(a454, a455)
    };
    let a483 = {
        let a480 = {
            let a456 = f49(v0, 12u32);
            let a457 = f49(v0, 8u32);
            append_bytes(a456, a457)
        };
        let a481 = {
            let a478 = {
                let a458 = f49(v0, 20u32);
                let a459 = f49(v0, 16u32);
                append_bytes(a458, a459)
            };
            let a479 = {
                let a476 = {
                    let a460 = f49(v0, 28u32);
                    let a461 = f49(v0, 24u32);
                    append_bytes(a460, a461)
                };
                let a477 = {
                    let a474 = {
                        let a462 = f49(v0, 36u32);
                        let a463 = f49(v0, 32u32);
                        append_bytes(a462, a463)
                    };
                    let a475 = {
                        let a472 = {
                            let a464 = f49(v0, 44u32);
                            let a465 = f49(v0, 40u32);
                            append_bytes(a464, a465)
                        };
                        let a473 = {
                            let a470 = {
                                let a466 = f49(v0, 52u32);
                                let a467 = f49(v0, 48u32);
                                append_bytes(a466, a467)
                            };
                            let a471 = {
                                let a468 = f49(v0, 60u32);
                                let a469 = f49(v0, 56u32);
                                append_bytes(a468, a469)
                            };
                            append_bytes(a470, a471)
                        };
                        append_bytes(a472, a473)
                    };
                    append_bytes(a474, a475)
                };
                append_bytes(a476, a477)
            };
            append_bytes(a478, a479)
        };
        append_bytes(a480, a481)
    };
    append_bytes(a482, a483)
}

pub fn f46(v0: Fn3, p487: u64, p488: (u64, Adt2)) -> R<(u64, Adt2)> {
    let m486 = (p487, p488);
    {
        let mut m486 = m486;
        loop {
            let step = {
                let (v1, v2) = m486;
                let m484 = v1;
                if m484 == 0 {
                    Err::<(u64, (u64, Adt2)), R<(u64, Adt2)>>(Ok(v2))
                } else {
                    let v3 = m484 - 1;
                    Ok::<(u64, (u64, Adt2)), R<(u64, Adt2)>>((v3, {
                        let c485: Fn3 = v0.clone();
                        c485.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m486 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f47(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt2)) -> R<(u64, Adt2)> {
    Ok(({
        let a490 = {
            let (h489, _) = v3.clone();
            h489
        };
        let a491 = 128u64;
        nat_add(a490, a491)?
    }, f72({
        let (_, h492) = v3;
        h492
    }, f73(v0, v1, v2, {
        let (h493, _) = v3;
        h493
    })?)?))
}

pub fn f48() -> Adt2 {
    Adt2::C0(7640891576956012808u64, 13503953896175478587u64, 4354685564936845355u64, 11912009170470909681u64, 5840696475078001361u64, 11170449401992604703u64, 2270897969802886507u64, 6620516959819538809u64)
}

pub fn f49(v0: u64, v1: u32) -> Bytes {
    f65(f74(f75(v0, v1)))
}

pub fn f50(v0: Bytes, v1: u64, v2: u64) -> R<(u64, List<Adt3>)> {
    f76(Fn8::F77(v0, v1), v2, (0u64, List::<Adt3>::nil()))
}

pub fn f51(v0: Bytes, v1: u64, v2: u64, v3: bool) -> R<Adt3> {
    let v4: u64 = {
        let a494 = 1024u64;
        let a495 = v2;
        nat_mul(a494, a495)?
    };
    let v5: u64 = f78(v1, v4);
    let v6: u64 = f79(v5)?;
    let v7: u64 = {
        let a496 = v6;
        let a497 = 1u64;
        nat_sub(a496, a497)
    };
    Ok(f80(f81(v0.clone(), v4, v2, v7)?, f82(v0, {
        let a500 = v4;
        let a501 = {
            let a498 = 64u64;
            let a499 = v7;
            nat_mul(a498, a499)?
        };
        nat_add(a500, a501)?
    })?, v2, f83({
        let a504 = v5;
        let a505 = {
            let a502 = 64u64;
            let a503 = v7;
            nat_mul(a502, a503)?
        };
        nat_sub(a504, a505)
    }), {
        let a508 = {
            let a506 = v7;
            let a507 = 0u64;
            if nat_eq(a506, a507) {
                3u32
            } else {
                2u32
            }
        };
        let a509 = if v3 {
            8u32
        } else {
            0u32
        };
        fixed_u32::bit_or(a508, a509)
    }))
}

pub fn f52(v0: Fn4, p513: u64, p514: (List<Adt3>, Adt3)) -> (List<Adt3>, Adt3) {
    let m512 = (p513, p514);
    {
        let mut m512 = m512;
        loop {
            let step = {
                let (v1, v2) = m512;
                let m510 = v1;
                if m510 == 0 {
                    Err::<(u64, (List<Adt3>, Adt3)), (List<Adt3>, Adt3)>(v2)
                } else {
                    let v3 = m510 - 1;
                    Ok::<(u64, (List<Adt3>, Adt3)), (List<Adt3>, Adt3)>((v3, {
                        let c511: Fn4 = v0.clone();
                        c511.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m512 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f53(v0: (List<Adt3>, Adt3)) -> (List<Adt3>, Adt3) {
    f84(v0)
}

pub fn f54(v0: u32, v1: u32) -> Bytes {
    f65(f66(v0, v1))
}

pub fn f55(v0: Fn5, p519: u64, p520: Adt4) -> R<(Adt4, bool)> {
    let m518 = (p519, p520);
    {
        let mut m518 = m518;
        loop {
            let step = {
                let (v1, v2) = m518;
                let m515 = v1;
                if m515 == 0 {
                    Err::<(u64, Adt4), R<(Adt4, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m515 - 1;
                    let m517 = {
                        let c516: Fn5 = v0.clone();
                        c516.apply(v2.clone())?
                    };
                    match m517 {
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
                Ok(next) => { m518 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f56(v0: Bytes, v1: Adt4) -> R<Option<Adt4>> {
    f85(v0, v1)
}

pub fn f57(v0: Bytes, v1: Bytes, v2: List<(Bytes, Bytes)>) -> Option<List<(Bytes, Bytes)>> {
    let v3: Adt7 = f86(Fn9::F87(v0.clone(), v1.clone()), Adt7::C0(false, false, List::<(Bytes, Bytes)>::nil()), v2);
    if match v3.clone() {
        Adt7::C0(_, h521, _) => {
            h521
        }
    } {
        None::<List<(Bytes, Bytes)>>
    } else {
        if match v3.clone() {
            Adt7::C0(h522, _, _) => {
                h522
            }
        } {
            Some(f88(match v3 {
                Adt7::C0(_, _, h523) => {
                    h523
                }
            }))
        } else {
            Some(f88(List::cons((v0, v1), match v3 {
                Adt7::C0(_, _, h524) => {
                    h524
                }
            })))
        }
    }
}

pub fn f58(v0: Fn6, p529: u64, p530: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m528 = (p529, p530);
    {
        let mut m528 = m528;
        loop {
            let step = {
                let (v1, v2) = m528;
                let m525 = v1;
                if m525 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m525 - 1;
                    let m527 = {
                        let c526: Fn6 = v0.clone();
                        c526.apply(v2.clone())?
                    };
                    match m527 {
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
                Ok(next) => { m528 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f59(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f89(v0, v1)
}

pub fn f60(v0: Bytes, v1: Bytes, v2: u64) -> bool {
    let m535 = {
        let a532 = v0;
        let a533 = v2;
        let a534 = {
            let a531 = v1.clone();
            length_bytes(a531)
        };
        slice_bytes(a532, a533, a534)
    };
    match m535 {
        None => {
            false
        }
        Some(v3) => {
            let a536 = v3;
            let a537 = v1;
            equal(a536, a537)
        }
    }
}

pub fn f61(v0: Adt0, v1: Bytes, v2: u64, v3: u64, v4: bool) -> R<Adt0> {
    let v5: u64 = f18(v1.clone(), v3)?;
    let a538 = v5;
    let a539 = 46u64;
    if nat_eq(a538, a539) {
        Ok(f33(v0, f6(Str::lit("number outside the integer subset"))))
    } else {
        let a540 = v5;
        let a541 = 101u64;
        if nat_eq(a540, a541) {
            Ok(f33(v0, f6(Str::lit("number outside the integer subset"))))
        } else {
            let a542 = v5;
            let a543 = 69u64;
            if nat_eq(a542, a543) {
                Ok(f33(v0, f6(Str::lit("number outside the integer subset"))))
            } else {
                if v4 {
                    Ok(f33(v0, f6(Str::lit("number outside the integer subset"))))
                } else {
                    Ok(f31(v0.clone(), match v0 {
                        Adt0::C0(_, h544, _, _) => {
                            h544
                        }
                    }, f90(v1, v2, {
                        let a545 = v3;
                        let a546 = v2;
                        nat_sub(a545, a546)
                    }), v3))
                }
            }
        }
    }
}

pub fn f62(v0: Bytes, v1: u64) -> R<u64> {
    let (h547, _) = f10(Fn1::F91(v0.clone()), {
        let a548 = v0;
        length_bytes(a548)
    }, v1)?;
    Ok(h547)
}

pub fn f63(v0: List<(Bytes, Bytes)>) -> R<Bytes> {
    let m549 = f92(Fn10::F93, None::<Bytes>, v0)?;
    match m549 {
        None => {
            Ok(Bytes::lit(&[]))
        }
        Some(v1) => {
            Ok(v1)
        }
    }
}

pub fn f64(v0: List<(Bytes, Bytes)>) -> R<Bytes> {
    let m550 = f92(Fn10::F94, None::<Bytes>, v0)?;
    match m550 {
        None => {
            Ok(Bytes::lit(&[]))
        }
        Some(v1) => {
            Ok(v1)
        }
    }
}

pub fn f65(v0: u32) -> Bytes {
    let a553 = {
        let a551 = v0;
        let a552 = 8u32;
        fixed_u32::bit_and(a551, a552)
    };
    let a554 = 0u32;
    if equal(a553, a554) {
        let a557 = {
            let a555 = v0;
            let a556 = 4u32;
            fixed_u32::bit_and(a555, a556)
        };
        let a558 = 0u32;
        if equal(a557, a558) {
            let a561 = {
                let a559 = v0;
                let a560 = 2u32;
                fixed_u32::bit_and(a559, a560)
            };
            let a562 = 0u32;
            if equal(a561, a562) {
                let a565 = {
                    let a563 = v0;
                    let a564 = 1u32;
                    fixed_u32::bit_and(a563, a564)
                };
                let a566 = 0u32;
                if equal(a565, a566) {
                    Bytes::lit(&[0x30])
                } else {
                    Bytes::lit(&[0x31])
                }
            } else {
                let a569 = {
                    let a567 = v0;
                    let a568 = 1u32;
                    fixed_u32::bit_and(a567, a568)
                };
                let a570 = 0u32;
                if equal(a569, a570) {
                    Bytes::lit(&[0x32])
                } else {
                    Bytes::lit(&[0x33])
                }
            }
        } else {
            let a573 = {
                let a571 = v0;
                let a572 = 2u32;
                fixed_u32::bit_and(a571, a572)
            };
            let a574 = 0u32;
            if equal(a573, a574) {
                let a577 = {
                    let a575 = v0;
                    let a576 = 1u32;
                    fixed_u32::bit_and(a575, a576)
                };
                let a578 = 0u32;
                if equal(a577, a578) {
                    Bytes::lit(&[0x34])
                } else {
                    Bytes::lit(&[0x35])
                }
            } else {
                let a581 = {
                    let a579 = v0;
                    let a580 = 1u32;
                    fixed_u32::bit_and(a579, a580)
                };
                let a582 = 0u32;
                if equal(a581, a582) {
                    Bytes::lit(&[0x36])
                } else {
                    Bytes::lit(&[0x37])
                }
            }
        }
    } else {
        let a585 = {
            let a583 = v0;
            let a584 = 4u32;
            fixed_u32::bit_and(a583, a584)
        };
        let a586 = 0u32;
        if equal(a585, a586) {
            let a589 = {
                let a587 = v0;
                let a588 = 2u32;
                fixed_u32::bit_and(a587, a588)
            };
            let a590 = 0u32;
            if equal(a589, a590) {
                let a593 = {
                    let a591 = v0;
                    let a592 = 1u32;
                    fixed_u32::bit_and(a591, a592)
                };
                let a594 = 0u32;
                if equal(a593, a594) {
                    Bytes::lit(&[0x38])
                } else {
                    Bytes::lit(&[0x39])
                }
            } else {
                let a597 = {
                    let a595 = v0;
                    let a596 = 1u32;
                    fixed_u32::bit_and(a595, a596)
                };
                let a598 = 0u32;
                if equal(a597, a598) {
                    Bytes::lit(&[0x61])
                } else {
                    Bytes::lit(&[0x62])
                }
            }
        } else {
            let a601 = {
                let a599 = v0;
                let a600 = 2u32;
                fixed_u32::bit_and(a599, a600)
            };
            let a602 = 0u32;
            if equal(a601, a602) {
                let a605 = {
                    let a603 = v0;
                    let a604 = 1u32;
                    fixed_u32::bit_and(a603, a604)
                };
                let a606 = 0u32;
                if equal(a605, a606) {
                    Bytes::lit(&[0x63])
                } else {
                    Bytes::lit(&[0x64])
                }
            } else {
                let a609 = {
                    let a607 = v0;
                    let a608 = 1u32;
                    fixed_u32::bit_and(a607, a608)
                };
                let a610 = 0u32;
                if equal(a609, a610) {
                    Bytes::lit(&[0x65])
                } else {
                    Bytes::lit(&[0x66])
                }
            }
        }
    }
}

pub fn f66(v0: u32, v1: u32) -> u32 {
    let m613 = {
        let a611 = v0;
        let a612 = v1;
        fixed_u32::shift_right(a611, a612)
    };
    match m613 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f67(v0: Adt5, v1: Adt8) -> R<Adt5> {
    let v2: Adt9 = f95(Fn11::F96, 64u64, Adt9::C0(match v0.clone() {
        Adt5::C0(h614, _, _, _, _, _, _, _) => {
            h614
        }
    }, match v0.clone() {
        Adt5::C0(_, h615, _, _, _, _, _, _) => {
            h615
        }
    }, match v0.clone() {
        Adt5::C0(_, _, h616, _, _, _, _, _) => {
            h616
        }
    }, match v0.clone() {
        Adt5::C0(_, _, _, h617, _, _, _, _) => {
            h617
        }
    }, match v0.clone() {
        Adt5::C0(_, _, _, _, h618, _, _, _) => {
            h618
        }
    }, match v0.clone() {
        Adt5::C0(_, _, _, _, _, h619, _, _) => {
            h619
        }
    }, match v0.clone() {
        Adt5::C0(_, _, _, _, _, _, h620, _) => {
            h620
        }
    }, match v0.clone() {
        Adt5::C0(_, _, _, _, _, _, _, h621) => {
            h621
        }
    }, v1, 0u64))?;
    Ok(Adt5::C0(f97(match v0.clone() {
        Adt5::C0(h622, _, _, _, _, _, _, _) => {
            h622
        }
    }, match v2.clone() {
        Adt9::C0(h623, _, _, _, _, _, _, _, _, _) => {
            h623
        }
    }), f97(match v0.clone() {
        Adt5::C0(_, h624, _, _, _, _, _, _) => {
            h624
        }
    }, match v2.clone() {
        Adt9::C0(_, h625, _, _, _, _, _, _, _, _) => {
            h625
        }
    }), f97(match v0.clone() {
        Adt5::C0(_, _, h626, _, _, _, _, _) => {
            h626
        }
    }, match v2.clone() {
        Adt9::C0(_, _, h627, _, _, _, _, _, _, _) => {
            h627
        }
    }), f97(match v0.clone() {
        Adt5::C0(_, _, _, h628, _, _, _, _) => {
            h628
        }
    }, match v2.clone() {
        Adt9::C0(_, _, _, h629, _, _, _, _, _, _) => {
            h629
        }
    }), f97(match v0.clone() {
        Adt5::C0(_, _, _, _, h630, _, _, _) => {
            h630
        }
    }, match v2.clone() {
        Adt9::C0(_, _, _, _, h631, _, _, _, _, _) => {
            h631
        }
    }), f97(match v0.clone() {
        Adt5::C0(_, _, _, _, _, h632, _, _) => {
            h632
        }
    }, match v2.clone() {
        Adt9::C0(_, _, _, _, _, h633, _, _, _, _) => {
            h633
        }
    }), f97(match v0.clone() {
        Adt5::C0(_, _, _, _, _, _, h634, _) => {
            h634
        }
    }, match v2.clone() {
        Adt9::C0(_, _, _, _, _, _, h635, _, _, _) => {
            h635
        }
    }), f97(match v0 {
        Adt5::C0(_, _, _, _, _, _, _, h636) => {
            h636
        }
    }, match v2 {
        Adt9::C0(_, _, _, _, _, _, _, h637, _, _) => {
            h637
        }
    })))
}

pub fn f68(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt8> {
    Ok(Adt8::C0(f98(v0.clone(), v1, v2, v3)?, f98(v0.clone(), v1, v2, {
        let a638 = v3;
        let a639 = 4u64;
        nat_add(a638, a639)?
    })?, f98(v0.clone(), v1, v2, {
        let a640 = v3;
        let a641 = 8u64;
        nat_add(a640, a641)?
    })?, f98(v0.clone(), v1, v2, {
        let a642 = v3;
        let a643 = 12u64;
        nat_add(a642, a643)?
    })?, f98(v0.clone(), v1, v2, {
        let a644 = v3;
        let a645 = 16u64;
        nat_add(a644, a645)?
    })?, f98(v0.clone(), v1, v2, {
        let a646 = v3;
        let a647 = 20u64;
        nat_add(a646, a647)?
    })?, f98(v0.clone(), v1, v2, {
        let a648 = v3;
        let a649 = 24u64;
        nat_add(a648, a649)?
    })?, f98(v0.clone(), v1, v2, {
        let a650 = v3;
        let a651 = 28u64;
        nat_add(a650, a651)?
    })?, f98(v0.clone(), v1, v2, {
        let a652 = v3;
        let a653 = 32u64;
        nat_add(a652, a653)?
    })?, f98(v0.clone(), v1, v2, {
        let a654 = v3;
        let a655 = 36u64;
        nat_add(a654, a655)?
    })?, f98(v0.clone(), v1, v2, {
        let a656 = v3;
        let a657 = 40u64;
        nat_add(a656, a657)?
    })?, f98(v0.clone(), v1, v2, {
        let a658 = v3;
        let a659 = 44u64;
        nat_add(a658, a659)?
    })?, f98(v0.clone(), v1, v2, {
        let a660 = v3;
        let a661 = 48u64;
        nat_add(a660, a661)?
    })?, f98(v0.clone(), v1, v2, {
        let a662 = v3;
        let a663 = 52u64;
        nat_add(a662, a663)?
    })?, f98(v0.clone(), v1, v2, {
        let a664 = v3;
        let a665 = 56u64;
        nat_add(a664, a665)?
    })?, f98(v0, v1, v2, {
        let a666 = v3;
        let a667 = 60u64;
        nat_add(a666, a667)?
    })?))
}

pub fn f69(v0: Fn7, p671: u64, p672: (u64, Adt6)) -> R<(u64, Adt6)> {
    let m670 = (p671, p672);
    {
        let mut m670 = m670;
        loop {
            let step = {
                let (v1, v2) = m670;
                let m668 = v1;
                if m668 == 0 {
                    Err::<(u64, (u64, Adt6)), R<(u64, Adt6)>>(Ok(v2))
                } else {
                    let v3 = m668 - 1;
                    Ok::<(u64, (u64, Adt6)), R<(u64, Adt6)>>((v3, {
                        let c669: Fn7 = v0.clone();
                        c669.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m670 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f70(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: (u64, Adt6)) -> R<(u64, Adt6)> {
    Ok(({
        let a674 = {
            let (h673, _) = v4.clone();
            h673
        };
        let a675 = 136u64;
        nat_add(a674, a675)?
    }, f99(v0, v1, v3, v2, {
        let (_, h676) = v4;
        h676
    }, {
        let (h677, _) = v4;
        h677
    })?))
}

pub fn f71() -> Adt6 {
    Adt6::C0(0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64)
}

pub fn f72(v0: Adt2, v1: Adt10) -> R<Adt2> {
    let v2: Adt11 = f100(Fn12::F101, 80u64, Adt11::C0(match v0.clone() {
        Adt2::C0(h678, _, _, _, _, _, _, _) => {
            h678
        }
    }, match v0.clone() {
        Adt2::C0(_, h679, _, _, _, _, _, _) => {
            h679
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h680, _, _, _, _, _) => {
            h680
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h681, _, _, _, _) => {
            h681
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h682, _, _, _) => {
            h682
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h683, _, _) => {
            h683
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h684, _) => {
            h684
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h685) => {
            h685
        }
    }, v1, 0u64))?;
    Ok(Adt2::C0(f102(match v0.clone() {
        Adt2::C0(h686, _, _, _, _, _, _, _) => {
            h686
        }
    }, match v2.clone() {
        Adt11::C0(h687, _, _, _, _, _, _, _, _, _) => {
            h687
        }
    }), f102(match v0.clone() {
        Adt2::C0(_, h688, _, _, _, _, _, _) => {
            h688
        }
    }, match v2.clone() {
        Adt11::C0(_, h689, _, _, _, _, _, _, _, _) => {
            h689
        }
    }), f102(match v0.clone() {
        Adt2::C0(_, _, h690, _, _, _, _, _) => {
            h690
        }
    }, match v2.clone() {
        Adt11::C0(_, _, h691, _, _, _, _, _, _, _) => {
            h691
        }
    }), f102(match v0.clone() {
        Adt2::C0(_, _, _, h692, _, _, _, _) => {
            h692
        }
    }, match v2.clone() {
        Adt11::C0(_, _, _, h693, _, _, _, _, _, _) => {
            h693
        }
    }), f102(match v0.clone() {
        Adt2::C0(_, _, _, _, h694, _, _, _) => {
            h694
        }
    }, match v2.clone() {
        Adt11::C0(_, _, _, _, h695, _, _, _, _, _) => {
            h695
        }
    }), f102(match v0.clone() {
        Adt2::C0(_, _, _, _, _, h696, _, _) => {
            h696
        }
    }, match v2.clone() {
        Adt11::C0(_, _, _, _, _, h697, _, _, _, _) => {
            h697
        }
    }), f102(match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h698, _) => {
            h698
        }
    }, match v2.clone() {
        Adt11::C0(_, _, _, _, _, _, h699, _, _, _) => {
            h699
        }
    }), f102(match v0 {
        Adt2::C0(_, _, _, _, _, _, _, h700) => {
            h700
        }
    }, match v2 {
        Adt11::C0(_, _, _, _, _, _, _, h701, _, _) => {
            h701
        }
    })))
}

pub fn f73(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt10> {
    Ok(Adt10::C0(f103(v0.clone(), v1, v2, {
        let a702 = v3;
        let a703 = 0u64;
        nat_add(a702, a703)?
    })?, f103(v0.clone(), v1, v2, {
        let a704 = v3;
        let a705 = 8u64;
        nat_add(a704, a705)?
    })?, f103(v0.clone(), v1, v2, {
        let a706 = v3;
        let a707 = 16u64;
        nat_add(a706, a707)?
    })?, f103(v0.clone(), v1, v2, {
        let a708 = v3;
        let a709 = 24u64;
        nat_add(a708, a709)?
    })?, f103(v0.clone(), v1, v2, {
        let a710 = v3;
        let a711 = 32u64;
        nat_add(a710, a711)?
    })?, f103(v0.clone(), v1, v2, {
        let a712 = v3;
        let a713 = 40u64;
        nat_add(a712, a713)?
    })?, f103(v0.clone(), v1, v2, {
        let a714 = v3;
        let a715 = 48u64;
        nat_add(a714, a715)?
    })?, f103(v0.clone(), v1, v2, {
        let a716 = v3;
        let a717 = 56u64;
        nat_add(a716, a717)?
    })?, f103(v0.clone(), v1, v2, {
        let a718 = v3;
        let a719 = 64u64;
        nat_add(a718, a719)?
    })?, f103(v0.clone(), v1, v2, {
        let a720 = v3;
        let a721 = 72u64;
        nat_add(a720, a721)?
    })?, f103(v0.clone(), v1, v2, {
        let a722 = v3;
        let a723 = 80u64;
        nat_add(a722, a723)?
    })?, f103(v0.clone(), v1, v2, {
        let a724 = v3;
        let a725 = 88u64;
        nat_add(a724, a725)?
    })?, f103(v0.clone(), v1, v2, {
        let a726 = v3;
        let a727 = 96u64;
        nat_add(a726, a727)?
    })?, f103(v0.clone(), v1, v2, {
        let a728 = v3;
        let a729 = 104u64;
        nat_add(a728, a729)?
    })?, f103(v0.clone(), v1, v2, {
        let a730 = v3;
        let a731 = 112u64;
        nat_add(a730, a731)?
    })?, f103(v0, v1, v2, {
        let a732 = v3;
        let a733 = 120u64;
        nat_add(a732, a733)?
    })?))
}

pub fn f74(v0: u64) -> u32 {
    let m737 = {
        let a736 = {
            let a734 = v0;
            let a735 = 4294967295u64;
            fixed_u64::bit_and(a734, a735)
        };
        fixed_u32::convert(i128::from(a736))
    };
    match m737 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f75(v0: u64, v1: u32) -> u64 {
    let m740 = {
        let a738 = v0;
        let a739 = v1;
        fixed_u64::shift_right(a738, a739)
    };
    match m740 {
        None => {
            0u64
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f76(v0: Fn8, p744: u64, p745: (u64, List<Adt3>)) -> R<(u64, List<Adt3>)> {
    let m743 = (p744, p745);
    {
        let mut m743 = m743;
        loop {
            let step = {
                let (v1, v2) = m743;
                let m741 = v1;
                if m741 == 0 {
                    Err::<(u64, (u64, List<Adt3>)), R<(u64, List<Adt3>)>>(Ok(v2))
                } else {
                    let v3 = m741 - 1;
                    Ok::<(u64, (u64, List<Adt3>)), R<(u64, List<Adt3>)>>((v3, {
                        let c742: Fn8 = v0.clone();
                        c742.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m743 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f77(v0: Bytes, v1: u64, v2: (u64, List<Adt3>)) -> R<(u64, List<Adt3>)> {
    f104(v2.clone(), f51(v0, v1, {
        let (h746, _) = v2;
        h746
    }, false)?)
}

pub fn f78(v0: u64, v1: u64) -> u64 {
    let a749 = 1024u64;
    let a750 = {
        let a747 = v0;
        let a748 = v1;
        nat_sub(a747, a748)
    };
    if nat_lt(a749, a750) {
        1024u64
    } else {
        let a751 = v0;
        let a752 = v1;
        nat_sub(a751, a752)
    }
}

pub fn f79(v0: u64) -> R<u64> {
    let a753 = v0;
    let a754 = 0u64;
    if nat_eq(a753, a754) {
        Ok(1u64)
    } else {
        let a757 = {
            let a755 = v0;
            let a756 = 63u64;
            nat_add(a755, a756)?
        };
        let a758 = 64u64;
        let a759 = 0u64;
        Ok(nat_quot(a757, a758, a759))
    }
}

pub fn f80(v0: Adt3, v1: Adt12, v2: u64, v3: u32, v4: u32) -> Adt3 {
    let v5: Adt13 = {
        let (h760, _) = f105(Fn13::F106, 7u64, (Adt13::C0(match v0.clone() {
            Adt3::C0(h761, _, _, _, _, _, _, _) => {
                h761
            }
        }, match v0.clone() {
            Adt3::C0(_, h762, _, _, _, _, _, _) => {
                h762
            }
        }, match v0.clone() {
            Adt3::C0(_, _, h763, _, _, _, _, _) => {
                h763
            }
        }, match v0.clone() {
            Adt3::C0(_, _, _, h764, _, _, _, _) => {
                h764
            }
        }, match v0.clone() {
            Adt3::C0(_, _, _, _, h765, _, _, _) => {
                h765
            }
        }, match v0.clone() {
            Adt3::C0(_, _, _, _, _, h766, _, _) => {
                h766
            }
        }, match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, h767, _) => {
                h767
            }
        }, match v0 {
            Adt3::C0(_, _, _, _, _, _, _, h768) => {
                h768
            }
        }, 1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, f83({
            let a769 = v2;
            let a770 = 4294967296u64;
            let a771 = 0u64;
            nat_rem(a769, a770, a771)
        }), f83({
            let a772 = v2;
            let a773 = 4294967296u64;
            let a774 = 0u64;
            nat_quot(a772, a773, a774)
        }), v3, v4), v1));
        h760
    };
    Adt3::C0({
        let a777 = match v5.clone() {
            Adt13::C0(h775, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h775
            }
        };
        let a778 = match v5.clone() {
            Adt13::C0(_, _, _, _, _, _, _, _, h776, _, _, _, _, _, _, _) => {
                h776
            }
        };
        fixed_u32::bit_xor(a777, a778)
    }, {
        let a781 = match v5.clone() {
            Adt13::C0(_, h779, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h779
            }
        };
        let a782 = match v5.clone() {
            Adt13::C0(_, _, _, _, _, _, _, _, _, h780, _, _, _, _, _, _) => {
                h780
            }
        };
        fixed_u32::bit_xor(a781, a782)
    }, {
        let a785 = match v5.clone() {
            Adt13::C0(_, _, h783, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h783
            }
        };
        let a786 = match v5.clone() {
            Adt13::C0(_, _, _, _, _, _, _, _, _, _, h784, _, _, _, _, _) => {
                h784
            }
        };
        fixed_u32::bit_xor(a785, a786)
    }, {
        let a789 = match v5.clone() {
            Adt13::C0(_, _, _, h787, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h787
            }
        };
        let a790 = match v5.clone() {
            Adt13::C0(_, _, _, _, _, _, _, _, _, _, _, h788, _, _, _, _) => {
                h788
            }
        };
        fixed_u32::bit_xor(a789, a790)
    }, {
        let a793 = match v5.clone() {
            Adt13::C0(_, _, _, _, h791, _, _, _, _, _, _, _, _, _, _, _) => {
                h791
            }
        };
        let a794 = match v5.clone() {
            Adt13::C0(_, _, _, _, _, _, _, _, _, _, _, _, h792, _, _, _) => {
                h792
            }
        };
        fixed_u32::bit_xor(a793, a794)
    }, {
        let a797 = match v5.clone() {
            Adt13::C0(_, _, _, _, _, h795, _, _, _, _, _, _, _, _, _, _) => {
                h795
            }
        };
        let a798 = match v5.clone() {
            Adt13::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h796, _, _) => {
                h796
            }
        };
        fixed_u32::bit_xor(a797, a798)
    }, {
        let a801 = match v5.clone() {
            Adt13::C0(_, _, _, _, _, _, h799, _, _, _, _, _, _, _, _, _) => {
                h799
            }
        };
        let a802 = match v5.clone() {
            Adt13::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h800, _) => {
                h800
            }
        };
        fixed_u32::bit_xor(a801, a802)
    }, {
        let a805 = match v5.clone() {
            Adt13::C0(_, _, _, _, _, _, _, h803, _, _, _, _, _, _, _, _) => {
                h803
            }
        };
        let a806 = match v5 {
            Adt13::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h804) => {
                h804
            }
        };
        fixed_u32::bit_xor(a805, a806)
    })
}

pub fn f81(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt3> {
    let (_, h807) = f107(Fn14::F108(v2, v0, v1), v3, (0u64, f109()))?;
    Ok(h807)
}

pub fn f82(v0: Bytes, v1: u64) -> R<Adt12> {
    Ok(Adt12::C0(f110(v0.clone(), {
        let a808 = v1;
        let a809 = 0u64;
        nat_add(a808, a809)?
    })?, f110(v0.clone(), {
        let a810 = v1;
        let a811 = 4u64;
        nat_add(a810, a811)?
    })?, f110(v0.clone(), {
        let a812 = v1;
        let a813 = 8u64;
        nat_add(a812, a813)?
    })?, f110(v0.clone(), {
        let a814 = v1;
        let a815 = 12u64;
        nat_add(a814, a815)?
    })?, f110(v0.clone(), {
        let a816 = v1;
        let a817 = 16u64;
        nat_add(a816, a817)?
    })?, f110(v0.clone(), {
        let a818 = v1;
        let a819 = 20u64;
        nat_add(a818, a819)?
    })?, f110(v0.clone(), {
        let a820 = v1;
        let a821 = 24u64;
        nat_add(a820, a821)?
    })?, f110(v0.clone(), {
        let a822 = v1;
        let a823 = 28u64;
        nat_add(a822, a823)?
    })?, f110(v0.clone(), {
        let a824 = v1;
        let a825 = 32u64;
        nat_add(a824, a825)?
    })?, f110(v0.clone(), {
        let a826 = v1;
        let a827 = 36u64;
        nat_add(a826, a827)?
    })?, f110(v0.clone(), {
        let a828 = v1;
        let a829 = 40u64;
        nat_add(a828, a829)?
    })?, f110(v0.clone(), {
        let a830 = v1;
        let a831 = 44u64;
        nat_add(a830, a831)?
    })?, f110(v0.clone(), {
        let a832 = v1;
        let a833 = 48u64;
        nat_add(a832, a833)?
    })?, f110(v0.clone(), {
        let a834 = v1;
        let a835 = 52u64;
        nat_add(a834, a835)?
    })?, f110(v0.clone(), {
        let a836 = v1;
        let a837 = 56u64;
        nat_add(a836, a837)?
    })?, f110(v0, {
        let a838 = v1;
        let a839 = 60u64;
        nat_add(a838, a839)?
    })?))
}

pub fn f83(v0: u64) -> u32 {
    let (_, h840) = {
        let (_, h841) = f111(Fn15::F112, 32u64, (v0, (1u32, 0u32)));
        h841
    };
    h840
}

pub fn f84(v0: (List<Adt3>, Adt3)) -> (List<Adt3>, Adt3) {
    let m843 = {
        let (h842, _) = v0.clone();
        h842
    };
    match m843.uncons() {
        None => {
            v0
        }
        Some((v1, v2)) => {
            (v2.clone(), f113(v1, {
                let (_, h844) = v0;
                h844
            }, {
                let m845 = v2;
                match m845.uncons() {
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

pub fn f85(v0: Bytes, v1: Adt4) -> R<Option<Adt4>> {
    if match v1.clone() {
        Adt4::C0(_, _, _, h846) => {
            h846
        }
    } {
        Ok(None::<Adt4>)
    } else {
        let a849 = 0u64;
        let a850 = {
            let a848 = match v1.clone() {
                Adt4::C0(_, _, h847, _) => {
                    h847
                }
            };
            length_bytes(a848)
        };
        if nat_lt(a849, a850) {
            Ok(None::<Adt4>)
        } else {
            let v2: u64 = f18(v0.clone(), match v1.clone() {
                Adt4::C0(h851, _, _, _) => {
                    h851
                }
            })?;
            let a852 = v2;
            let a853 = 256u64;
            if nat_eq(a852, a853) {
                Ok(f114(v1, f6(Str::lit("unterminated string"))))
            } else {
                let a854 = v2;
                let a855 = 34u64;
                if nat_eq(a854, a855) {
                    Ok(Some(Adt4::C0({
                        let a857 = match v1.clone() {
                            Adt4::C0(h856, _, _, _) => {
                                h856
                            }
                        };
                        let a858 = 1u64;
                        nat_add(a857, a858)?
                    }, match v1 {
                        Adt4::C0(_, h859, _, _) => {
                            h859
                        }
                    }, Bytes::lit(&[]), true)))
                } else {
                    let a860 = v2;
                    let a861 = 92u64;
                    if nat_eq(a860, a861) {
                        f115(v0, v1)
                    } else {
                        let a862 = v2;
                        let a863 = 32u64;
                        if nat_lt(a862, a863) {
                            Ok(f114(v1, f6(Str::lit("control character in string"))))
                        } else {
                            let a864 = v2;
                            let a865 = 128u64;
                            if nat_lt(a864, a865) {
                                f116(v1, v2, 1u64)
                            } else {
                                f117(v0, v1, v2)
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f86(v0: Fn9, p869: Adt7, p870: List<(Bytes, Bytes)>) -> Adt7 {
    let m868 = (p869, p870);
    {
        let mut m868 = m868;
        loop {
            let step = {
                let (v1, v2) = m868;
                let m866 = v2;
                match m866.uncons() {
                    None => {
                        Err::<(Adt7, List<(Bytes, Bytes)>), Adt7>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt7, List<(Bytes, Bytes)>), Adt7>(({
                            let c867: Fn9 = v0.clone();
                            c867.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m868 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f87(v0: Bytes, v1: Bytes, v2: Adt7, v3: (Bytes, Bytes)) -> Adt7 {
    if match v2.clone() {
        Adt7::C0(h871, _, _) => {
            h871
        }
    } {
        Adt7::C0(true, match v2.clone() {
            Adt7::C0(_, h872, _) => {
                h872
            }
        }, List::cons(v3, match v2 {
            Adt7::C0(_, _, h873) => {
                h873
            }
        }))
    } else {
        let a875 = v0.clone();
        let a876 = {
            let (h874, _) = v3.clone();
            h874
        };
        if equal(a875, a876) {
            Adt7::C0(true, true, List::cons(v3, match v2 {
                Adt7::C0(_, _, h877) => {
                    h877
                }
            }))
        } else {
            if f118(v0.clone(), {
                let (h878, _) = v3.clone();
                h878
            }) {
                Adt7::C0(true, false, List::cons(v3, List::cons((v0, v1), match v2 {
                    Adt7::C0(_, _, h879) => {
                        h879
                    }
                })))
            } else {
                Adt7::C0(false, false, List::cons(v3, match v2 {
                    Adt7::C0(_, _, h880) => {
                        h880
                    }
                }))
            }
        }
    }
}

pub fn f88(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f119(Fn16::F120, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f89(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let v2: u64 = {
        let (h881, _) = v1;
        h881
    };
    let v3: u64 = f18(v0, v2)?;
    let a882 = v3;
    let a883 = 256u64;
    if nat_eq(a882, a883) {
        Ok(None::<(u64, Bytes)>)
    } else {
        Ok(Some(({
            let a884 = v2;
            let a885 = 1u64;
            nat_add(a884, a885)?
        }, {
            let a913 = {
                let (_, h886) = v1;
                h886
            };
            let a914 = {
                let a887 = v3;
                let a888 = 34u64;
                if nat_eq(a887, a888) {
                    Bytes::lit(&[0x5c, 0x22])
                } else {
                    let a889 = v3;
                    let a890 = 92u64;
                    if nat_eq(a889, a890) {
                        Bytes::lit(&[0x5c, 0x5c])
                    } else {
                        let a891 = v3;
                        let a892 = 8u64;
                        if nat_eq(a891, a892) {
                            Bytes::lit(&[0x5c, 0x62])
                        } else {
                            let a893 = v3;
                            let a894 = 9u64;
                            if nat_eq(a893, a894) {
                                Bytes::lit(&[0x5c, 0x74])
                            } else {
                                let a895 = v3;
                                let a896 = 10u64;
                                if nat_eq(a895, a896) {
                                    Bytes::lit(&[0x5c, 0x6e])
                                } else {
                                    let a897 = v3;
                                    let a898 = 12u64;
                                    if nat_eq(a897, a898) {
                                        Bytes::lit(&[0x5c, 0x66])
                                    } else {
                                        let a899 = v3;
                                        let a900 = 13u64;
                                        if nat_eq(a899, a900) {
                                            Bytes::lit(&[0x5c, 0x72])
                                        } else {
                                            let a901 = v3;
                                            let a902 = 32u64;
                                            if nat_lt(a901, a902) {
                                                let a911 = Bytes::lit(&[0x5c, 0x75, 0x30, 0x30]);
                                                let a912 = {
                                                    let a909 = f90(Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66]), {
                                                        let a903 = v3;
                                                        let a904 = 16u64;
                                                        let a905 = 0u64;
                                                        nat_quot(a903, a904, a905)
                                                    }, 1u64);
                                                    let a910 = f90(Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66]), {
                                                        let a906 = v3;
                                                        let a907 = 16u64;
                                                        let a908 = 0u64;
                                                        nat_rem(a906, a907, a908)
                                                    }, 1u64);
                                                    append_bytes(a909, a910)
                                                };
                                                append_bytes(a911, a912)
                                            } else {
                                                f121(v3)
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            };
            append_bytes(a913, a914)
        })))
    }
}

pub fn f90(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m918 = {
        let a915 = v0;
        let a916 = v1;
        let a917 = v2;
        slice_bytes(a915, a916, a917)
    };
    match m918 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f91(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f18(v0, v1)?;
    let a919 = 47u64;
    let a920 = v2;
    if nat_lt(a919, a920) {
        let a921 = v2;
        let a922 = 58u64;
        if nat_lt(a921, a922) {
            Ok(Some({
                let a923 = v1;
                let a924 = 1u64;
                nat_add(a923, a924)?
            }))
        } else {
            Ok(None::<u64>)
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f92(v0: Fn10, p928: Option<Bytes>, p929: List<(Bytes, Bytes)>) -> R<Option<Bytes>> {
    let m927 = (p928, p929);
    {
        let mut m927 = m927;
        loop {
            let step = {
                let (v1, v2) = m927;
                let m925 = v2;
                match m925.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<(Bytes, Bytes)>), R<Option<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<(Bytes, Bytes)>), R<Option<Bytes>>>(({
                            let c926: Fn10 = v0.clone();
                            c926.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m927 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f93(v0: Option<Bytes>, v1: (Bytes, Bytes)) -> R<Option<Bytes>> {
    let v2: Bytes = {
        let a934 = f32({
            let (h930, _) = v1;
            h930
        })?;
        let a935 = {
            let a932 = Bytes::lit(&[0x3a]);
            let a933 = {
                let (_, h931) = v1;
                h931
            };
            append_bytes(a932, a933)
        };
        append_bytes(a934, a935)
    };
    let m936 = v0;
    match m936 {
        None => {
            Ok(Some(v2))
        }
        Some(v3) => {
            Ok(Some({
                let a939 = v3;
                let a940 = {
                    let a937 = Bytes::lit(&[0x2c]);
                    let a938 = v2;
                    append_bytes(a937, a938)
                };
                append_bytes(a939, a940)
            }))
        }
    }
}

pub fn f94(v0: Option<Bytes>, v1: (Bytes, Bytes)) -> Option<Bytes> {
    let m941 = v0;
    match m941 {
        None => {
            Some({
                let (_, h942) = v1;
                h942
            })
        }
        Some(v2) => {
            Some({
                let a946 = {
                    let (_, h943) = v1;
                    h943
                };
                let a947 = {
                    let a944 = Bytes::lit(&[0x2c]);
                    let a945 = v2;
                    append_bytes(a944, a945)
                };
                append_bytes(a946, a947)
            })
        }
    }
}

pub fn f95(v0: Fn11, p951: u64, p952: Adt9) -> R<Adt9> {
    let m950 = (p951, p952);
    {
        let mut m950 = m950;
        loop {
            let step = {
                let (v1, v2) = m950;
                let m948 = v1;
                if m948 == 0 {
                    Err::<(u64, Adt9), R<Adt9>>(Ok(v2))
                } else {
                    let v3 = m948 - 1;
                    Ok::<(u64, Adt9), R<Adt9>>((v3, {
                        let c949: Fn11 = v0.clone();
                        c949.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m950 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f96(v0: Adt9) -> R<Adt9> {
    f122(v0)
}

pub fn f97(v0: u32, v1: u32) -> u32 {
    let m955 = {
        let a953 = f123(v0);
        let a954 = f123(v1);
        fixed_u64::checked_add(a953, a954)
    };
    match m955 {
        None => {
            0u32
        }
        Some(v2) => {
            f74(v2)
        }
    }
}

pub fn f98(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u32> {
    let a958 = {
        let a956 = v3;
        let a957 = 8u64;
        nat_add(a956, a957)?
    };
    let a959 = v2;
    if nat_eq(a958, a959) {
        Ok(f83({
            let a962 = {
                let a960 = v1;
                let a961 = 8u64;
                nat_mul(a960, a961)?
            };
            let a963 = 4294967296u64;
            let a964 = 0u64;
            nat_quot(a962, a963, a964)
        }))
    } else {
        let a967 = {
            let a965 = v3;
            let a966 = 4u64;
            nat_add(a965, a966)?
        };
        let a968 = v2;
        if nat_eq(a967, a968) {
            Ok(f83({
                let a971 = {
                    let a969 = v1;
                    let a970 = 8u64;
                    nat_mul(a969, a970)?
                };
                let a972 = 4294967296u64;
                let a973 = 0u64;
                nat_rem(a971, a972, a973)
            }))
        } else {
            Ok(f124(f125(v0.clone(), v1, v3), f125(v0.clone(), v1, {
                let a974 = v3;
                let a975 = 1u64;
                nat_add(a974, a975)?
            }), f125(v0.clone(), v1, {
                let a976 = v3;
                let a977 = 2u64;
                nat_add(a976, a977)?
            }), f125(v0, v1, {
                let a978 = v3;
                let a979 = 3u64;
                nat_add(a978, a979)?
            })))
        }
    }
}

pub fn f99(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: Adt6, v5: u64) -> R<Adt6> {
    f126(Adt6::C0({
        let a983 = match v4.clone() {
            Adt6::C0(h980, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h980
            }
        };
        let a984 = f127(v0.clone(), v1, v2, v3, {
            let a981 = v5;
            let a982 = 0u64;
            nat_add(a981, a982)?
        })?;
        fixed_u64::bit_xor(a983, a984)
    }, {
        let a988 = match v4.clone() {
            Adt6::C0(_, h985, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h985
            }
        };
        let a989 = f127(v0.clone(), v1, v2, v3, {
            let a986 = v5;
            let a987 = 8u64;
            nat_add(a986, a987)?
        })?;
        fixed_u64::bit_xor(a988, a989)
    }, {
        let a993 = match v4.clone() {
            Adt6::C0(_, _, h990, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h990
            }
        };
        let a994 = f127(v0.clone(), v1, v2, v3, {
            let a991 = v5;
            let a992 = 16u64;
            nat_add(a991, a992)?
        })?;
        fixed_u64::bit_xor(a993, a994)
    }, {
        let a998 = match v4.clone() {
            Adt6::C0(_, _, _, h995, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h995
            }
        };
        let a999 = f127(v0.clone(), v1, v2, v3, {
            let a996 = v5;
            let a997 = 24u64;
            nat_add(a996, a997)?
        })?;
        fixed_u64::bit_xor(a998, a999)
    }, {
        let a1003 = match v4.clone() {
            Adt6::C0(_, _, _, _, h1000, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1000
            }
        };
        let a1004 = f127(v0.clone(), v1, v2, v3, {
            let a1001 = v5;
            let a1002 = 32u64;
            nat_add(a1001, a1002)?
        })?;
        fixed_u64::bit_xor(a1003, a1004)
    }, {
        let a1008 = match v4.clone() {
            Adt6::C0(_, _, _, _, _, h1005, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1005
            }
        };
        let a1009 = f127(v0.clone(), v1, v2, v3, {
            let a1006 = v5;
            let a1007 = 40u64;
            nat_add(a1006, a1007)?
        })?;
        fixed_u64::bit_xor(a1008, a1009)
    }, {
        let a1013 = match v4.clone() {
            Adt6::C0(_, _, _, _, _, _, h1010, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1010
            }
        };
        let a1014 = f127(v0.clone(), v1, v2, v3, {
            let a1011 = v5;
            let a1012 = 48u64;
            nat_add(a1011, a1012)?
        })?;
        fixed_u64::bit_xor(a1013, a1014)
    }, {
        let a1018 = match v4.clone() {
            Adt6::C0(_, _, _, _, _, _, _, h1015, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1015
            }
        };
        let a1019 = f127(v0.clone(), v1, v2, v3, {
            let a1016 = v5;
            let a1017 = 56u64;
            nat_add(a1016, a1017)?
        })?;
        fixed_u64::bit_xor(a1018, a1019)
    }, {
        let a1023 = match v4.clone() {
            Adt6::C0(_, _, _, _, _, _, _, _, h1020, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1020
            }
        };
        let a1024 = f127(v0.clone(), v1, v2, v3, {
            let a1021 = v5;
            let a1022 = 64u64;
            nat_add(a1021, a1022)?
        })?;
        fixed_u64::bit_xor(a1023, a1024)
    }, {
        let a1028 = match v4.clone() {
            Adt6::C0(_, _, _, _, _, _, _, _, _, h1025, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1025
            }
        };
        let a1029 = f127(v0.clone(), v1, v2, v3, {
            let a1026 = v5;
            let a1027 = 72u64;
            nat_add(a1026, a1027)?
        })?;
        fixed_u64::bit_xor(a1028, a1029)
    }, {
        let a1033 = match v4.clone() {
            Adt6::C0(_, _, _, _, _, _, _, _, _, _, h1030, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1030
            }
        };
        let a1034 = f127(v0.clone(), v1, v2, v3, {
            let a1031 = v5;
            let a1032 = 80u64;
            nat_add(a1031, a1032)?
        })?;
        fixed_u64::bit_xor(a1033, a1034)
    }, {
        let a1038 = match v4.clone() {
            Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, h1035, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1035
            }
        };
        let a1039 = f127(v0.clone(), v1, v2, v3, {
            let a1036 = v5;
            let a1037 = 88u64;
            nat_add(a1036, a1037)?
        })?;
        fixed_u64::bit_xor(a1038, a1039)
    }, {
        let a1043 = match v4.clone() {
            Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, h1040, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1040
            }
        };
        let a1044 = f127(v0.clone(), v1, v2, v3, {
            let a1041 = v5;
            let a1042 = 96u64;
            nat_add(a1041, a1042)?
        })?;
        fixed_u64::bit_xor(a1043, a1044)
    }, {
        let a1048 = match v4.clone() {
            Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1045, _, _, _, _, _, _, _, _, _, _, _) => {
                h1045
            }
        };
        let a1049 = f127(v0.clone(), v1, v2, v3, {
            let a1046 = v5;
            let a1047 = 104u64;
            nat_add(a1046, a1047)?
        })?;
        fixed_u64::bit_xor(a1048, a1049)
    }, {
        let a1053 = match v4.clone() {
            Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1050, _, _, _, _, _, _, _, _, _, _) => {
                h1050
            }
        };
        let a1054 = f127(v0.clone(), v1, v2, v3, {
            let a1051 = v5;
            let a1052 = 112u64;
            nat_add(a1051, a1052)?
        })?;
        fixed_u64::bit_xor(a1053, a1054)
    }, {
        let a1058 = match v4.clone() {
            Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1055, _, _, _, _, _, _, _, _, _) => {
                h1055
            }
        };
        let a1059 = f127(v0.clone(), v1, v2, v3, {
            let a1056 = v5;
            let a1057 = 120u64;
            nat_add(a1056, a1057)?
        })?;
        fixed_u64::bit_xor(a1058, a1059)
    }, {
        let a1063 = match v4.clone() {
            Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1060, _, _, _, _, _, _, _, _) => {
                h1060
            }
        };
        let a1064 = f127(v0, v1, v2, v3, {
            let a1061 = v5;
            let a1062 = 128u64;
            nat_add(a1061, a1062)?
        })?;
        fixed_u64::bit_xor(a1063, a1064)
    }, match v4.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1065, _, _, _, _, _, _, _) => {
            h1065
        }
    }, match v4.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1066, _, _, _, _, _, _) => {
            h1066
        }
    }, match v4.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1067, _, _, _, _, _) => {
            h1067
        }
    }, match v4.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1068, _, _, _, _) => {
            h1068
        }
    }, match v4.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1069, _, _, _) => {
            h1069
        }
    }, match v4.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1070, _, _) => {
            h1070
        }
    }, match v4.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1071, _) => {
            h1071
        }
    }, match v4 {
        Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1072) => {
            h1072
        }
    }))
}

pub fn f100(v0: Fn12, p1076: u64, p1077: Adt11) -> R<Adt11> {
    let m1075 = (p1076, p1077);
    {
        let mut m1075 = m1075;
        loop {
            let step = {
                let (v1, v2) = m1075;
                let m1073 = v1;
                if m1073 == 0 {
                    Err::<(u64, Adt11), R<Adt11>>(Ok(v2))
                } else {
                    let v3 = m1073 - 1;
                    Ok::<(u64, Adt11), R<Adt11>>((v3, {
                        let c1074: Fn12 = v0.clone();
                        c1074.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m1075 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f101(v0: Adt11) -> R<Adt11> {
    f128(v0)
}

pub fn f102(v0: u64, v1: u64) -> u64 {
    let v2: u64 = 4294967295u64;
    let v3: u64 = f129({
        let a1078 = v0;
        let a1079 = v2;
        fixed_u64::bit_and(a1078, a1079)
    }, {
        let a1080 = v1;
        let a1081 = v2;
        fixed_u64::bit_and(a1080, a1081)
    });
    let v4: u64 = f129(f129(f75(v0, 32u32), f75(v1, 32u32)), f75(v3, 32u32));
    let a1086 = f130({
        let a1082 = v4;
        let a1083 = v2;
        fixed_u64::bit_and(a1082, a1083)
    }, 32u32);
    let a1087 = {
        let a1084 = v3;
        let a1085 = v2;
        fixed_u64::bit_and(a1084, a1085)
    };
    fixed_u64::bit_or(a1086, a1087)
}

pub fn f103(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a1090 = {
        let a1088 = v3;
        let a1089 = 16u64;
        nat_add(a1088, a1089)?
    };
    let a1091 = v2;
    if nat_eq(a1090, a1091) {
        Ok(0u64)
    } else {
        let a1094 = {
            let a1092 = v3;
            let a1093 = 8u64;
            nat_add(a1092, a1093)?
        };
        let a1095 = v2;
        if nat_eq(a1094, a1095) {
            Ok(f131({
                let a1096 = v1;
                let a1097 = 8u64;
                nat_mul(a1096, a1097)?
            }))
        } else {
            Ok(f132(f133(v0.clone(), v1, {
                let a1098 = v3;
                let a1099 = 0u64;
                nat_add(a1098, a1099)?
            }), f133(v0.clone(), v1, {
                let a1100 = v3;
                let a1101 = 1u64;
                nat_add(a1100, a1101)?
            }), f133(v0.clone(), v1, {
                let a1102 = v3;
                let a1103 = 2u64;
                nat_add(a1102, a1103)?
            }), f133(v0.clone(), v1, {
                let a1104 = v3;
                let a1105 = 3u64;
                nat_add(a1104, a1105)?
            }), f133(v0.clone(), v1, {
                let a1106 = v3;
                let a1107 = 4u64;
                nat_add(a1106, a1107)?
            }), f133(v0.clone(), v1, {
                let a1108 = v3;
                let a1109 = 5u64;
                nat_add(a1108, a1109)?
            }), f133(v0.clone(), v1, {
                let a1110 = v3;
                let a1111 = 6u64;
                nat_add(a1110, a1111)?
            }), f133(v0, v1, {
                let a1112 = v3;
                let a1113 = 7u64;
                nat_add(a1112, a1113)?
            })))
        }
    }
}

pub fn f104(v0: (u64, List<Adt3>), v1: Adt3) -> R<(u64, List<Adt3>)> {
    let v2: Adt14 = f134(Fn17::F135, 40u64, Adt14::C0(v1, {
        let a1115 = {
            let (h1114, _) = v0.clone();
            h1114
        };
        let a1116 = 1u64;
        nat_add(a1115, a1116)?
    }, {
        let (_, h1117) = v0;
        h1117
    }));
    Ok(({
        let a1119 = {
            let (h1118, _) = v0;
            h1118
        };
        let a1120 = 1u64;
        nat_add(a1119, a1120)?
    }, List::cons(match v2.clone() {
        Adt14::C0(h1121, _, _) => {
            h1121
        }
    }, match v2 {
        Adt14::C0(_, _, h1122) => {
            h1122
        }
    })))
}

pub fn f105(v0: Fn13, p1126: u64, p1127: (Adt13, Adt12)) -> (Adt13, Adt12) {
    let m1125 = (p1126, p1127);
    {
        let mut m1125 = m1125;
        loop {
            let step = {
                let (v1, v2) = m1125;
                let m1123 = v1;
                if m1123 == 0 {
                    Err::<(u64, (Adt13, Adt12)), (Adt13, Adt12)>(v2)
                } else {
                    let v3 = m1123 - 1;
                    Ok::<(u64, (Adt13, Adt12)), (Adt13, Adt12)>((v3, {
                        let c1124: Fn13 = v0.clone();
                        c1124.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m1125 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f106(v0: (Adt13, Adt12)) -> (Adt13, Adt12) {
    f136(v0)
}

pub fn f107(v0: Fn14, p1131: u64, p1132: (u64, Adt3)) -> R<(u64, Adt3)> {
    let m1130 = (p1131, p1132);
    {
        let mut m1130 = m1130;
        loop {
            let step = {
                let (v1, v2) = m1130;
                let m1128 = v1;
                if m1128 == 0 {
                    Err::<(u64, (u64, Adt3)), R<(u64, Adt3)>>(Ok(v2))
                } else {
                    let v3 = m1128 - 1;
                    Ok::<(u64, (u64, Adt3)), R<(u64, Adt3)>>((v3, {
                        let c1129: Fn14 = v0.clone();
                        c1129.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m1130 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f108(v0: u64, v1: Bytes, v2: u64, v3: (u64, Adt3)) -> R<(u64, Adt3)> {
    Ok(({
        let a1134 = {
            let (h1133, _) = v3.clone();
            h1133
        };
        let a1135 = 1u64;
        nat_add(a1134, a1135)?
    }, f80({
        let (_, h1136) = v3.clone();
        h1136
    }, f82(v1, {
        let a1140 = v2;
        let a1141 = {
            let a1138 = 64u64;
            let a1139 = {
                let (h1137, _) = v3.clone();
                h1137
            };
            nat_mul(a1138, a1139)?
        };
        nat_add(a1140, a1141)?
    })?, v0, 64u32, {
        let a1143 = {
            let (h1142, _) = v3;
            h1142
        };
        let a1144 = 0u64;
        if nat_eq(a1143, a1144) {
            1u32
        } else {
            0u32
        }
    })))
}

pub fn f109() -> Adt3 {
    Adt3::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f110(v0: Bytes, v1: u64) -> R<u32> {
    let a1155 = f137(v0.clone(), v1);
    let a1156 = {
        let a1153 = f138(f137(v0.clone(), {
            let a1145 = v1;
            let a1146 = 1u64;
            nat_add(a1145, a1146)?
        }), 8u32);
        let a1154 = {
            let a1151 = f138(f137(v0.clone(), {
                let a1147 = v1;
                let a1148 = 2u64;
                nat_add(a1147, a1148)?
            }), 16u32);
            let a1152 = f138(f137(v0, {
                let a1149 = v1;
                let a1150 = 3u64;
                nat_add(a1149, a1150)?
            }), 24u32);
            fixed_u32::bit_or(a1151, a1152)
        };
        fixed_u32::bit_or(a1153, a1154)
    };
    Ok(fixed_u32::bit_or(a1155, a1156))
}

pub fn f111(v0: Fn15, p1160: u64, p1161: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m1159 = (p1160, p1161);
    {
        let mut m1159 = m1159;
        loop {
            let step = {
                let (v1, v2) = m1159;
                let m1157 = v1;
                if m1157 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m1157 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c1158: Fn15 = v0.clone();
                        c1158.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m1159 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f112(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h1162, _) = v0;
        h1162
    };
    let v2: u32 = {
        let (h1163, _) = {
            let (_, h1164) = v0;
            h1164
        };
        h1163
    };
    let v3: u32 = {
        let (_, h1165) = {
            let (_, h1166) = v0;
            h1166
        };
        h1165
    };
    ({
        let a1167 = v1;
        let a1168 = 2u64;
        let a1169 = 0u64;
        nat_quot(a1167, a1168, a1169)
    }, (f138(v2, 1u32), {
        let a1173 = {
            let a1170 = v1;
            let a1171 = 2u64;
            let a1172 = 0u64;
            nat_rem(a1170, a1171, a1172)
        };
        let a1174 = 1u64;
        if nat_eq(a1173, a1174) {
            let a1175 = v3;
            let a1176 = v2;
            fixed_u32::bit_or(a1175, a1176)
        } else {
            v3
        }
    }))
}

pub fn f113(v0: Adt3, v1: Adt3, v2: bool) -> Adt3 {
    f80(f109(), f139(v0, v1), 0u64, 64u32, {
        let a1177 = 4u32;
        let a1178 = if v2 {
            8u32
        } else {
            0u32
        };
        fixed_u32::bit_or(a1177, a1178)
    })
}

pub fn f114(v0: Adt4, v1: Bytes) -> Option<Adt4> {
    Some(Adt4::C0(match v0.clone() {
        Adt4::C0(h1179, _, _, _) => {
            h1179
        }
    }, match v0 {
        Adt4::C0(_, h1180, _, _) => {
            h1180
        }
    }, v1, false))
}

pub fn f115(v0: Bytes, v1: Adt4) -> R<Option<Adt4>> {
    let v2: u64 = match v1.clone() {
        Adt4::C0(h1181, _, _, _) => {
            h1181
        }
    };
    let v3: u64 = f18(v0.clone(), {
        let a1182 = v2;
        let a1183 = 1u64;
        nat_add(a1182, a1183)?
    })?;
    let v4: u64 = f140(v3);
    let a1184 = v4;
    let a1185 = 256u64;
    if nat_lt(a1184, a1185) {
        f116(v1, v4, 2u64)
    } else {
        let a1186 = v3;
        let a1187 = 117u64;
        if nat_eq(a1186, a1187) {
            let v5: u64 = f141(v0, {
                let a1188 = v2;
                let a1189 = 2u64;
                nat_add(a1188, a1189)?
            })?;
            let a1190 = v5;
            let a1191 = 65536u64;
            if nat_lt(a1190, a1191) {
                f116(v1, v5, 6u64)
            } else {
                Ok(f114(v1, f6(Str::lit("bad unicode escape"))))
            }
        } else {
            Ok(f114(v1, f6(Str::lit("bad escape"))))
        }
    }
}

pub fn f116(v0: Adt4, v1: u64, v2: u64) -> R<Option<Adt4>> {
    let m1193 = f142(match v0.clone() {
        Adt4::C0(_, h1192, _, _) => {
            h1192
        }
    }, v1)?;
    match m1193 {
        None => {
            Ok(f114(v0, f6(Str::lit("string outside the NFC subset"))))
        }
        Some(v3) => {
            Ok(Some(Adt4::C0({
                let a1195 = match v0 {
                    Adt4::C0(h1194, _, _, _) => {
                        h1194
                    }
                };
                let a1196 = v2;
                nat_add(a1195, a1196)?
            }, v3, Bytes::lit(&[]), false)))
        }
    }
}

pub fn f117(v0: Bytes, v1: Adt4, v2: u64) -> R<Option<Adt4>> {
    let v3: u64 = f18(v0, {
        let a1198 = match v1.clone() {
            Adt4::C0(h1197, _, _, _) => {
                h1197
            }
        };
        let a1199 = 1u64;
        nat_add(a1198, a1199)?
    })?;
    let a1200 = 127u64;
    let a1201 = v3;
    if nat_lt(a1200, a1201) {
        let a1202 = v3;
        let a1203 = 192u64;
        if nat_lt(a1202, a1203) {
            let a1204 = v2;
            let a1205 = 204u64;
            if nat_eq(a1204, a1205) {
                f116(v1, {
                    let a1208 = 768u64;
                    let a1209 = {
                        let a1206 = v3;
                        let a1207 = 128u64;
                        nat_sub(a1206, a1207)
                    };
                    nat_add(a1208, a1209)?
                }, 2u64)
            } else {
                let a1210 = v2;
                let a1211 = 194u64;
                if nat_eq(a1210, a1211) {
                    f116(v1, {
                        let a1216 = {
                            let a1212 = 2u64;
                            let a1213 = 64u64;
                            nat_mul(a1212, a1213)?
                        };
                        let a1217 = {
                            let a1214 = v3;
                            let a1215 = 128u64;
                            nat_sub(a1214, a1215)
                        };
                        nat_add(a1216, a1217)?
                    }, 2u64)
                } else {
                    let a1218 = v2;
                    let a1219 = 195u64;
                    if nat_eq(a1218, a1219) {
                        f116(v1, {
                            let a1224 = {
                                let a1220 = 3u64;
                                let a1221 = 64u64;
                                nat_mul(a1220, a1221)?
                            };
                            let a1225 = {
                                let a1222 = v3;
                                let a1223 = 128u64;
                                nat_sub(a1222, a1223)
                            };
                            nat_add(a1224, a1225)?
                        }, 2u64)
                    } else {
                        Ok(f114(v1, f6(Str::lit("string outside the NFC subset"))))
                    }
                }
            }
        } else {
            Ok(f114(v1, f6(Str::lit("invalid UTF-8"))))
        }
    } else {
        Ok(f114(v1, f6(Str::lit("invalid UTF-8"))))
    }
}

pub fn f118(v0: Bytes, v1: Bytes) -> bool {
    let a1230 = {
        let a1226 = v0;
        let a1227 = v1;
        compare_bytes(a1226, a1227)
    };
    let a1231 = {
        let a1228 = Bytes::lit(&[]);
        let a1229 = Bytes::lit(&[0x01]);
        compare_bytes(a1228, a1229)
    };
    equal(a1230, a1231)
}

pub fn f119(v0: Fn16, p1235: List<(Bytes, Bytes)>, p1236: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m1234 = (p1235, p1236);
    {
        let mut m1234 = m1234;
        loop {
            let step = {
                let (v1, v2) = m1234;
                let m1232 = v2;
                match m1232.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c1233: Fn16 = v0.clone();
                            c1233.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1234 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f120(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn f121(v0: u64) -> Bytes {
    f90(Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff]), v0, 1u64)
}

pub fn f122(v0: Adt9) -> R<Adt9> {
    let v1: u32 = f143(match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, _, h1237) => {
            h1237
        }
    });
    let v2: u32 = f97(match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, h1238, _, _) => {
            h1238
        }
    }, f97(f144(match v0.clone() {
        Adt9::C0(_, _, _, _, h1239, _, _, _, _, _) => {
            h1239
        }
    }), f97(f145(match v0.clone() {
        Adt9::C0(_, _, _, _, h1240, _, _, _, _, _) => {
            h1240
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, h1241, _, _, _, _) => {
            h1241
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, h1242, _, _, _) => {
            h1242
        }
    }), f97(v1, {
        let m1245 = match v0.clone() {
            Adt9::C0(_, _, _, _, _, _, _, _, h1244, _) => {
                h1244
            }
        };
        match m1245 {
            Adt8::C0(h1243, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1243
            }
        }
    }))));
    let v3: u32 = f97(f146(match v0.clone() {
        Adt9::C0(h1246, _, _, _, _, _, _, _, _, _) => {
            h1246
        }
    }), f147(match v0.clone() {
        Adt9::C0(h1247, _, _, _, _, _, _, _, _, _) => {
            h1247
        }
    }, match v0.clone() {
        Adt9::C0(_, h1248, _, _, _, _, _, _, _, _) => {
            h1248
        }
    }, match v0.clone() {
        Adt9::C0(_, _, h1249, _, _, _, _, _, _, _) => {
            h1249
        }
    }));
    Ok(Adt9::C0(f97(v2, v3), match v0.clone() {
        Adt9::C0(h1250, _, _, _, _, _, _, _, _, _) => {
            h1250
        }
    }, match v0.clone() {
        Adt9::C0(_, h1251, _, _, _, _, _, _, _, _) => {
            h1251
        }
    }, match v0.clone() {
        Adt9::C0(_, _, h1252, _, _, _, _, _, _, _) => {
            h1252
        }
    }, f97(match v0.clone() {
        Adt9::C0(_, _, _, h1253, _, _, _, _, _, _) => {
            h1253
        }
    }, v2), match v0.clone() {
        Adt9::C0(_, _, _, _, h1254, _, _, _, _, _) => {
            h1254
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, h1255, _, _, _, _) => {
            h1255
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, h1256, _, _, _) => {
            h1256
        }
    }, f148(match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, h1257, _) => {
            h1257
        }
    }), {
        let a1259 = match v0 {
            Adt9::C0(_, _, _, _, _, _, _, _, _, h1258) => {
                h1258
            }
        };
        let a1260 = 1u64;
        nat_add(a1259, a1260)?
    }))
}

pub fn f123(v0: u32) -> u64 {
    let m1262 = {
        let a1261 = v0;
        fixed_u64::convert(i128::from(a1261))
    };
    match m1262 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f124(v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
    let a1267 = f138(v0, 24u32);
    let a1268 = {
        let a1265 = f138(v1, 16u32);
        let a1266 = {
            let a1263 = f138(v2, 8u32);
            let a1264 = v3;
            fixed_u32::bit_or(a1263, a1264)
        };
        fixed_u32::bit_or(a1265, a1266)
    };
    fixed_u32::bit_or(a1267, a1268)
}

pub fn f125(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a1269 = v2;
    let a1270 = v1;
    if nat_lt(a1269, a1270) {
        let m1273 = {
            let a1271 = v0;
            let a1272 = v2;
            index_bytes(a1271, a1272)
        };
        match m1273 {
            None => {
                0u32
            }
            Some(v3) => {
                let m1275 = {
                    let a1274 = v3;
                    fixed_u32::convert(i128::from(a1274))
                };
                match m1275 {
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
        let a1276 = v2;
        let a1277 = v1;
        if nat_eq(a1276, a1277) {
            128u32
        } else {
            0u32
        }
    }
}

pub fn f126(v0: Adt6) -> R<Adt6> {
    let (_, h1278) = f69(Fn7::F149, 24u64, (0u64, v0))?;
    Ok(h1278)
}

pub fn f127(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    Ok(f150(f151(v0.clone(), v1, v2, v3, {
        let a1279 = v4;
        let a1280 = 0u64;
        nat_add(a1279, a1280)?
    })?, f151(v0.clone(), v1, v2, v3, {
        let a1281 = v4;
        let a1282 = 1u64;
        nat_add(a1281, a1282)?
    })?, f151(v0.clone(), v1, v2, v3, {
        let a1283 = v4;
        let a1284 = 2u64;
        nat_add(a1283, a1284)?
    })?, f151(v0.clone(), v1, v2, v3, {
        let a1285 = v4;
        let a1286 = 3u64;
        nat_add(a1285, a1286)?
    })?, f151(v0.clone(), v1, v2, v3, {
        let a1287 = v4;
        let a1288 = 4u64;
        nat_add(a1287, a1288)?
    })?, f151(v0.clone(), v1, v2, v3, {
        let a1289 = v4;
        let a1290 = 5u64;
        nat_add(a1289, a1290)?
    })?, f151(v0.clone(), v1, v2, v3, {
        let a1291 = v4;
        let a1292 = 6u64;
        nat_add(a1291, a1292)?
    })?, f151(v0, v1, v2, v3, {
        let a1293 = v4;
        let a1294 = 7u64;
        nat_add(a1293, a1294)?
    })?))
}

pub fn f128(v0: Adt11) -> R<Adt11> {
    let v1: u64 = f152(match v0.clone() {
        Adt11::C0(_, _, _, _, _, _, _, _, _, h1295) => {
            h1295
        }
    });
    let v2: u64 = f102(match v0.clone() {
        Adt11::C0(_, _, _, _, _, _, _, h1296, _, _) => {
            h1296
        }
    }, f102(f153(match v0.clone() {
        Adt11::C0(_, _, _, _, h1297, _, _, _, _, _) => {
            h1297
        }
    }), f102(f154(match v0.clone() {
        Adt11::C0(_, _, _, _, h1298, _, _, _, _, _) => {
            h1298
        }
    }, match v0.clone() {
        Adt11::C0(_, _, _, _, _, h1299, _, _, _, _) => {
            h1299
        }
    }, match v0.clone() {
        Adt11::C0(_, _, _, _, _, _, h1300, _, _, _) => {
            h1300
        }
    }), f102(v1, {
        let m1303 = match v0.clone() {
            Adt11::C0(_, _, _, _, _, _, _, _, h1302, _) => {
                h1302
            }
        };
        match m1303 {
            Adt10::C0(h1301, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1301
            }
        }
    }))));
    let v3: u64 = f102(f155(match v0.clone() {
        Adt11::C0(h1304, _, _, _, _, _, _, _, _, _) => {
            h1304
        }
    }), f156(match v0.clone() {
        Adt11::C0(h1305, _, _, _, _, _, _, _, _, _) => {
            h1305
        }
    }, match v0.clone() {
        Adt11::C0(_, h1306, _, _, _, _, _, _, _, _) => {
            h1306
        }
    }, match v0.clone() {
        Adt11::C0(_, _, h1307, _, _, _, _, _, _, _) => {
            h1307
        }
    }));
    Ok(Adt11::C0(f102(v2, v3), match v0.clone() {
        Adt11::C0(h1308, _, _, _, _, _, _, _, _, _) => {
            h1308
        }
    }, match v0.clone() {
        Adt11::C0(_, h1309, _, _, _, _, _, _, _, _) => {
            h1309
        }
    }, match v0.clone() {
        Adt11::C0(_, _, h1310, _, _, _, _, _, _, _) => {
            h1310
        }
    }, f102(match v0.clone() {
        Adt11::C0(_, _, _, h1311, _, _, _, _, _, _) => {
            h1311
        }
    }, v2), match v0.clone() {
        Adt11::C0(_, _, _, _, h1312, _, _, _, _, _) => {
            h1312
        }
    }, match v0.clone() {
        Adt11::C0(_, _, _, _, _, h1313, _, _, _, _) => {
            h1313
        }
    }, match v0.clone() {
        Adt11::C0(_, _, _, _, _, _, h1314, _, _, _) => {
            h1314
        }
    }, f157(match v0.clone() {
        Adt11::C0(_, _, _, _, _, _, _, _, h1315, _) => {
            h1315
        }
    }), {
        let a1317 = match v0 {
            Adt11::C0(_, _, _, _, _, _, _, _, _, h1316) => {
                h1316
            }
        };
        let a1318 = 1u64;
        nat_add(a1317, a1318)?
    }))
}

pub fn f129(v0: u64, v1: u64) -> u64 {
    let m1321 = {
        let a1319 = v0;
        let a1320 = v1;
        fixed_u64::checked_add(a1319, a1320)
    };
    match m1321 {
        None => {
            0u64
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f130(v0: u64, v1: u32) -> u64 {
    let m1324 = {
        let a1322 = v0;
        let a1323 = v1;
        fixed_u64::shift_left(a1322, a1323)
    };
    match m1324 {
        None => {
            0u64
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f131(v0: u64) -> u64 {
    let (_, h1325) = {
        let (_, h1326) = f158(Fn18::F159, 64u64, (v0, (1u64, 0u64)));
        h1326
    };
    h1325
}

pub fn f132(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> u64 {
    let a1339 = f130(v0, 56u32);
    let a1340 = {
        let a1337 = f130(v1, 48u32);
        let a1338 = {
            let a1335 = f130(v2, 40u32);
            let a1336 = {
                let a1333 = f130(v3, 32u32);
                let a1334 = {
                    let a1331 = f130(v4, 24u32);
                    let a1332 = {
                        let a1329 = f130(v5, 16u32);
                        let a1330 = {
                            let a1327 = f130(v6, 8u32);
                            let a1328 = v7;
                            fixed_u64::bit_or(a1327, a1328)
                        };
                        fixed_u64::bit_or(a1329, a1330)
                    };
                    fixed_u64::bit_or(a1331, a1332)
                };
                fixed_u64::bit_or(a1333, a1334)
            };
            fixed_u64::bit_or(a1335, a1336)
        };
        fixed_u64::bit_or(a1337, a1338)
    };
    fixed_u64::bit_or(a1339, a1340)
}

pub fn f133(v0: Bytes, v1: u64, v2: u64) -> u64 {
    let a1341 = v2;
    let a1342 = v1;
    if nat_lt(a1341, a1342) {
        f160(v0, v2)
    } else {
        let a1343 = v2;
        let a1344 = v1;
        if nat_eq(a1343, a1344) {
            128u64
        } else {
            0u64
        }
    }
}

pub fn f134(v0: Fn17, p1348: u64, p1349: Adt14) -> Adt14 {
    let m1347 = (p1348, p1349);
    {
        let mut m1347 = m1347;
        loop {
            let step = {
                let (v1, v2) = m1347;
                let m1345 = v1;
                if m1345 == 0 {
                    Err::<(u64, Adt14), Adt14>(v2)
                } else {
                    let v3 = m1345 - 1;
                    Ok::<(u64, Adt14), Adt14>((v3, {
                        let c1346: Fn17 = v0.clone();
                        c1346.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m1347 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f135(v0: Adt14) -> Adt14 {
    f161(v0)
}

pub fn f136(v0: (Adt13, Adt12)) -> (Adt13, Adt12) {
    (f162(f163({
        let (h1350, _) = v0.clone();
        h1350
    }, {
        let (_, h1351) = v0.clone();
        h1351
    }), {
        let (_, h1352) = v0.clone();
        h1352
    }), f164({
        let (_, h1353) = v0;
        h1353
    }))
}

pub fn f137(v0: Bytes, v1: u64) -> u32 {
    let m1356 = {
        let a1354 = v0;
        let a1355 = v1;
        index_bytes(a1354, a1355)
    };
    match m1356 {
        None => {
            0u32
        }
        Some(v2) => {
            let m1358 = {
                let a1357 = v2;
                fixed_u32::convert(i128::from(a1357))
            };
            match m1358 {
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

pub fn f138(v0: u32, v1: u32) -> u32 {
    let m1361 = {
        let a1359 = v0;
        let a1360 = v1;
        fixed_u32::shift_left(a1359, a1360)
    };
    match m1361 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f139(v0: Adt3, v1: Adt3) -> Adt12 {
    Adt12::C0(match v0.clone() {
        Adt3::C0(h1362, _, _, _, _, _, _, _) => {
            h1362
        }
    }, match v0.clone() {
        Adt3::C0(_, h1363, _, _, _, _, _, _) => {
            h1363
        }
    }, match v0.clone() {
        Adt3::C0(_, _, h1364, _, _, _, _, _) => {
            h1364
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, h1365, _, _, _, _) => {
            h1365
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, h1366, _, _, _) => {
            h1366
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, h1367, _, _) => {
            h1367
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, h1368, _) => {
            h1368
        }
    }, match v0 {
        Adt3::C0(_, _, _, _, _, _, _, h1369) => {
            h1369
        }
    }, match v1.clone() {
        Adt3::C0(h1370, _, _, _, _, _, _, _) => {
            h1370
        }
    }, match v1.clone() {
        Adt3::C0(_, h1371, _, _, _, _, _, _) => {
            h1371
        }
    }, match v1.clone() {
        Adt3::C0(_, _, h1372, _, _, _, _, _) => {
            h1372
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, h1373, _, _, _, _) => {
            h1373
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h1374, _, _, _) => {
            h1374
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h1375, _, _) => {
            h1375
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h1376, _) => {
            h1376
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h1377) => {
            h1377
        }
    })
}

pub fn f140(v0: u64) -> u64 {
    let a1378 = v0;
    let a1379 = 34u64;
    if nat_eq(a1378, a1379) {
        34u64
    } else {
        let a1380 = v0;
        let a1381 = 92u64;
        if nat_eq(a1380, a1381) {
            92u64
        } else {
            let a1382 = v0;
            let a1383 = 47u64;
            if nat_eq(a1382, a1383) {
                47u64
            } else {
                let a1384 = v0;
                let a1385 = 98u64;
                if nat_eq(a1384, a1385) {
                    8u64
                } else {
                    let a1386 = v0;
                    let a1387 = 102u64;
                    if nat_eq(a1386, a1387) {
                        12u64
                    } else {
                        let a1388 = v0;
                        let a1389 = 110u64;
                        if nat_eq(a1388, a1389) {
                            10u64
                        } else {
                            let a1390 = v0;
                            let a1391 = 114u64;
                            if nat_eq(a1390, a1391) {
                                13u64
                            } else {
                                let a1392 = v0;
                                let a1393 = 116u64;
                                if nat_eq(a1392, a1393) {
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

pub fn f141(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f165(f18(v0.clone(), v1)?);
    let v3: u64 = f165(f18(v0.clone(), {
        let a1394 = v1;
        let a1395 = 1u64;
        nat_add(a1394, a1395)?
    })?);
    let v4: u64 = f165(f18(v0.clone(), {
        let a1396 = v1;
        let a1397 = 2u64;
        nat_add(a1396, a1397)?
    })?);
    let v5: u64 = f165(f18(v0, {
        let a1398 = v1;
        let a1399 = 3u64;
        nat_add(a1398, a1399)?
    })?);
    let a1400 = v2;
    let a1401 = 16u64;
    if nat_lt(a1400, a1401) {
        let a1402 = v3;
        let a1403 = 16u64;
        if nat_lt(a1402, a1403) {
            let a1404 = v4;
            let a1405 = 16u64;
            if nat_lt(a1404, a1405) {
                let a1406 = v5;
                let a1407 = 16u64;
                if nat_lt(a1406, a1407) {
                    let a1418 = {
                        let a1416 = {
                            let a1412 = {
                                let a1408 = v2;
                                let a1409 = 4096u64;
                                nat_mul(a1408, a1409)?
                            };
                            let a1413 = {
                                let a1410 = v3;
                                let a1411 = 256u64;
                                nat_mul(a1410, a1411)?
                            };
                            nat_add(a1412, a1413)?
                        };
                        let a1417 = {
                            let a1414 = v4;
                            let a1415 = 16u64;
                            nat_mul(a1414, a1415)?
                        };
                        nat_add(a1416, a1417)?
                    };
                    let a1419 = v5;
                    nat_add(a1418, a1419)
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

pub fn f142(v0: Bytes, v1: u64) -> R<Option<Bytes>> {
    let a1420 = v1;
    let a1421 = 128u64;
    if nat_lt(a1420, a1421) {
        Ok(Some({
            let a1422 = v0;
            let a1423 = f121(v1);
            append_bytes(a1422, a1423)
        }))
    } else {
        let a1424 = v1;
        let a1425 = 256u64;
        if nat_lt(a1424, a1425) {
            Ok(Some({
                let a1426 = v0;
                let a1427 = f166(v1)?;
                append_bytes(a1426, a1427)
            }))
        } else {
            let a1428 = 767u64;
            let a1429 = v1;
            if nat_lt(a1428, a1429) {
                let a1430 = v1;
                let a1431 = 808u64;
                if nat_lt(a1430, a1431) {
                    let v2: u64 = {
                        let a1432 = v0.clone();
                        length_bytes(a1432)
                    };
                    let v3: u64 = {
                        let a1433 = 0u64;
                        let a1434 = v2;
                        if nat_lt(a1433, a1434) {
                            f18(v0.clone(), {
                                let a1435 = v2;
                                let a1436 = 1u64;
                                nat_sub(a1435, a1436)
                            })?
                        } else {
                            256u64
                        }
                    };
                    let v4: u64 = {
                        let a1437 = v3;
                        let a1438 = 128u64;
                        if nat_lt(a1437, a1438) {
                            f167({
                                let a1439 = v1;
                                let a1440 = 768u64;
                                nat_sub(a1439, a1440)
                            }, v3)?
                        } else {
                            0u64
                        }
                    };
                    let a1441 = 0u64;
                    let a1442 = v4;
                    if nat_lt(a1441, a1442) {
                        Ok(Some({
                            let a1445 = f90(v0, 0u64, {
                                let a1443 = v2;
                                let a1444 = 1u64;
                                nat_sub(a1443, a1444)
                            });
                            let a1446 = f166(v4)?;
                            append_bytes(a1445, a1446)
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

pub fn f143(v0: u64) -> u32 {
    let a1447 = v0;
    let a1448 = 32u64;
    if nat_lt(a1447, a1448) {
        let a1449 = v0;
        let a1450 = 16u64;
        if nat_lt(a1449, a1450) {
            let a1451 = v0;
            let a1452 = 8u64;
            if nat_lt(a1451, a1452) {
                let a1453 = v0;
                let a1454 = 4u64;
                if nat_lt(a1453, a1454) {
                    let a1455 = v0;
                    let a1456 = 2u64;
                    if nat_lt(a1455, a1456) {
                        let a1457 = v0;
                        let a1458 = 1u64;
                        if nat_lt(a1457, a1458) {
                            1116352408u32
                        } else {
                            1899447441u32
                        }
                    } else {
                        let a1459 = v0;
                        let a1460 = 3u64;
                        if nat_lt(a1459, a1460) {
                            3049323471u32
                        } else {
                            3921009573u32
                        }
                    }
                } else {
                    let a1461 = v0;
                    let a1462 = 6u64;
                    if nat_lt(a1461, a1462) {
                        let a1463 = v0;
                        let a1464 = 5u64;
                        if nat_lt(a1463, a1464) {
                            961987163u32
                        } else {
                            1508970993u32
                        }
                    } else {
                        let a1465 = v0;
                        let a1466 = 7u64;
                        if nat_lt(a1465, a1466) {
                            2453635748u32
                        } else {
                            2870763221u32
                        }
                    }
                }
            } else {
                let a1467 = v0;
                let a1468 = 12u64;
                if nat_lt(a1467, a1468) {
                    let a1469 = v0;
                    let a1470 = 10u64;
                    if nat_lt(a1469, a1470) {
                        let a1471 = v0;
                        let a1472 = 9u64;
                        if nat_lt(a1471, a1472) {
                            3624381080u32
                        } else {
                            310598401u32
                        }
                    } else {
                        let a1473 = v0;
                        let a1474 = 11u64;
                        if nat_lt(a1473, a1474) {
                            607225278u32
                        } else {
                            1426881987u32
                        }
                    }
                } else {
                    let a1475 = v0;
                    let a1476 = 14u64;
                    if nat_lt(a1475, a1476) {
                        let a1477 = v0;
                        let a1478 = 13u64;
                        if nat_lt(a1477, a1478) {
                            1925078388u32
                        } else {
                            2162078206u32
                        }
                    } else {
                        let a1479 = v0;
                        let a1480 = 15u64;
                        if nat_lt(a1479, a1480) {
                            2614888103u32
                        } else {
                            3248222580u32
                        }
                    }
                }
            }
        } else {
            let a1481 = v0;
            let a1482 = 24u64;
            if nat_lt(a1481, a1482) {
                let a1483 = v0;
                let a1484 = 20u64;
                if nat_lt(a1483, a1484) {
                    let a1485 = v0;
                    let a1486 = 18u64;
                    if nat_lt(a1485, a1486) {
                        let a1487 = v0;
                        let a1488 = 17u64;
                        if nat_lt(a1487, a1488) {
                            3835390401u32
                        } else {
                            4022224774u32
                        }
                    } else {
                        let a1489 = v0;
                        let a1490 = 19u64;
                        if nat_lt(a1489, a1490) {
                            264347078u32
                        } else {
                            604807628u32
                        }
                    }
                } else {
                    let a1491 = v0;
                    let a1492 = 22u64;
                    if nat_lt(a1491, a1492) {
                        let a1493 = v0;
                        let a1494 = 21u64;
                        if nat_lt(a1493, a1494) {
                            770255983u32
                        } else {
                            1249150122u32
                        }
                    } else {
                        let a1495 = v0;
                        let a1496 = 23u64;
                        if nat_lt(a1495, a1496) {
                            1555081692u32
                        } else {
                            1996064986u32
                        }
                    }
                }
            } else {
                let a1497 = v0;
                let a1498 = 28u64;
                if nat_lt(a1497, a1498) {
                    let a1499 = v0;
                    let a1500 = 26u64;
                    if nat_lt(a1499, a1500) {
                        let a1501 = v0;
                        let a1502 = 25u64;
                        if nat_lt(a1501, a1502) {
                            2554220882u32
                        } else {
                            2821834349u32
                        }
                    } else {
                        let a1503 = v0;
                        let a1504 = 27u64;
                        if nat_lt(a1503, a1504) {
                            2952996808u32
                        } else {
                            3210313671u32
                        }
                    }
                } else {
                    let a1505 = v0;
                    let a1506 = 30u64;
                    if nat_lt(a1505, a1506) {
                        let a1507 = v0;
                        let a1508 = 29u64;
                        if nat_lt(a1507, a1508) {
                            3336571891u32
                        } else {
                            3584528711u32
                        }
                    } else {
                        let a1509 = v0;
                        let a1510 = 31u64;
                        if nat_lt(a1509, a1510) {
                            113926993u32
                        } else {
                            338241895u32
                        }
                    }
                }
            }
        }
    } else {
        let a1511 = v0;
        let a1512 = 48u64;
        if nat_lt(a1511, a1512) {
            let a1513 = v0;
            let a1514 = 40u64;
            if nat_lt(a1513, a1514) {
                let a1515 = v0;
                let a1516 = 36u64;
                if nat_lt(a1515, a1516) {
                    let a1517 = v0;
                    let a1518 = 34u64;
                    if nat_lt(a1517, a1518) {
                        let a1519 = v0;
                        let a1520 = 33u64;
                        if nat_lt(a1519, a1520) {
                            666307205u32
                        } else {
                            773529912u32
                        }
                    } else {
                        let a1521 = v0;
                        let a1522 = 35u64;
                        if nat_lt(a1521, a1522) {
                            1294757372u32
                        } else {
                            1396182291u32
                        }
                    }
                } else {
                    let a1523 = v0;
                    let a1524 = 38u64;
                    if nat_lt(a1523, a1524) {
                        let a1525 = v0;
                        let a1526 = 37u64;
                        if nat_lt(a1525, a1526) {
                            1695183700u32
                        } else {
                            1986661051u32
                        }
                    } else {
                        let a1527 = v0;
                        let a1528 = 39u64;
                        if nat_lt(a1527, a1528) {
                            2177026350u32
                        } else {
                            2456956037u32
                        }
                    }
                }
            } else {
                let a1529 = v0;
                let a1530 = 44u64;
                if nat_lt(a1529, a1530) {
                    let a1531 = v0;
                    let a1532 = 42u64;
                    if nat_lt(a1531, a1532) {
                        let a1533 = v0;
                        let a1534 = 41u64;
                        if nat_lt(a1533, a1534) {
                            2730485921u32
                        } else {
                            2820302411u32
                        }
                    } else {
                        let a1535 = v0;
                        let a1536 = 43u64;
                        if nat_lt(a1535, a1536) {
                            3259730800u32
                        } else {
                            3345764771u32
                        }
                    }
                } else {
                    let a1537 = v0;
                    let a1538 = 46u64;
                    if nat_lt(a1537, a1538) {
                        let a1539 = v0;
                        let a1540 = 45u64;
                        if nat_lt(a1539, a1540) {
                            3516065817u32
                        } else {
                            3600352804u32
                        }
                    } else {
                        let a1541 = v0;
                        let a1542 = 47u64;
                        if nat_lt(a1541, a1542) {
                            4094571909u32
                        } else {
                            275423344u32
                        }
                    }
                }
            }
        } else {
            let a1543 = v0;
            let a1544 = 56u64;
            if nat_lt(a1543, a1544) {
                let a1545 = v0;
                let a1546 = 52u64;
                if nat_lt(a1545, a1546) {
                    let a1547 = v0;
                    let a1548 = 50u64;
                    if nat_lt(a1547, a1548) {
                        let a1549 = v0;
                        let a1550 = 49u64;
                        if nat_lt(a1549, a1550) {
                            430227734u32
                        } else {
                            506948616u32
                        }
                    } else {
                        let a1551 = v0;
                        let a1552 = 51u64;
                        if nat_lt(a1551, a1552) {
                            659060556u32
                        } else {
                            883997877u32
                        }
                    }
                } else {
                    let a1553 = v0;
                    let a1554 = 54u64;
                    if nat_lt(a1553, a1554) {
                        let a1555 = v0;
                        let a1556 = 53u64;
                        if nat_lt(a1555, a1556) {
                            958139571u32
                        } else {
                            1322822218u32
                        }
                    } else {
                        let a1557 = v0;
                        let a1558 = 55u64;
                        if nat_lt(a1557, a1558) {
                            1537002063u32
                        } else {
                            1747873779u32
                        }
                    }
                }
            } else {
                let a1559 = v0;
                let a1560 = 60u64;
                if nat_lt(a1559, a1560) {
                    let a1561 = v0;
                    let a1562 = 58u64;
                    if nat_lt(a1561, a1562) {
                        let a1563 = v0;
                        let a1564 = 57u64;
                        if nat_lt(a1563, a1564) {
                            1955562222u32
                        } else {
                            2024104815u32
                        }
                    } else {
                        let a1565 = v0;
                        let a1566 = 59u64;
                        if nat_lt(a1565, a1566) {
                            2227730452u32
                        } else {
                            2361852424u32
                        }
                    }
                } else {
                    let a1567 = v0;
                    let a1568 = 62u64;
                    if nat_lt(a1567, a1568) {
                        let a1569 = v0;
                        let a1570 = 61u64;
                        if nat_lt(a1569, a1570) {
                            2428436474u32
                        } else {
                            2756734187u32
                        }
                    } else {
                        let a1571 = v0;
                        let a1572 = 63u64;
                        if nat_lt(a1571, a1572) {
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

pub fn f144(v0: u32) -> u32 {
    let a1575 = {
        let a1573 = f168(v0, 6u32, 26u32);
        let a1574 = f168(v0, 11u32, 21u32);
        fixed_u32::bit_xor(a1573, a1574)
    };
    let a1576 = f168(v0, 25u32, 7u32);
    fixed_u32::bit_xor(a1575, a1576)
}

pub fn f145(v0: u32, v1: u32, v2: u32) -> u32 {
    let a1582 = {
        let a1577 = v0;
        let a1578 = v1;
        fixed_u32::bit_and(a1577, a1578)
    };
    let a1583 = {
        let a1580 = {
            let a1579 = v0;
            fixed_u32::bit_not(a1579)
        };
        let a1581 = v2;
        fixed_u32::bit_and(a1580, a1581)
    };
    fixed_u32::bit_xor(a1582, a1583)
}

pub fn f146(v0: u32) -> u32 {
    let a1586 = {
        let a1584 = f168(v0, 2u32, 30u32);
        let a1585 = f168(v0, 13u32, 19u32);
        fixed_u32::bit_xor(a1584, a1585)
    };
    let a1587 = f168(v0, 22u32, 10u32);
    fixed_u32::bit_xor(a1586, a1587)
}

pub fn f147(v0: u32, v1: u32, v2: u32) -> u32 {
    let a1596 = {
        let a1592 = {
            let a1588 = v0;
            let a1589 = v1;
            fixed_u32::bit_and(a1588, a1589)
        };
        let a1593 = {
            let a1590 = v0;
            let a1591 = v2;
            fixed_u32::bit_and(a1590, a1591)
        };
        fixed_u32::bit_xor(a1592, a1593)
    };
    let a1597 = {
        let a1594 = v1;
        let a1595 = v2;
        fixed_u32::bit_and(a1594, a1595)
    };
    fixed_u32::bit_xor(a1596, a1597)
}

pub fn f148(v0: Adt8) -> Adt8 {
    Adt8::C0(match v0.clone() {
        Adt8::C0(_, h1598, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1598
        }
    }, match v0.clone() {
        Adt8::C0(_, _, h1599, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1599
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, h1600, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1600
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, h1601, _, _, _, _, _, _, _, _, _, _, _) => {
            h1601
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, _, h1602, _, _, _, _, _, _, _, _, _, _) => {
            h1602
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, h1603, _, _, _, _, _, _, _, _, _) => {
            h1603
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, h1604, _, _, _, _, _, _, _, _) => {
            h1604
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, h1605, _, _, _, _, _, _, _) => {
            h1605
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, h1606, _, _, _, _, _, _) => {
            h1606
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, h1607, _, _, _, _, _) => {
            h1607
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, h1608, _, _, _, _) => {
            h1608
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, h1609, _, _, _) => {
            h1609
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1610, _, _) => {
            h1610
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1611, _) => {
            h1611
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1612) => {
            h1612
        }
    }, f97(f169(match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1613, _) => {
            h1613
        }
    }), f97(match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, h1614, _, _, _, _, _, _) => {
            h1614
        }
    }, f97(f170(match v0.clone() {
        Adt8::C0(_, h1615, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1615
        }
    }), match v0 {
        Adt8::C0(h1616, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1616
        }
    }))))
}

pub fn f149(v0: (u64, Adt6)) -> R<(u64, Adt6)> {
    f171(v0)
}

pub fn f150(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> u64 {
    let a1629 = f130(v7, 56u32);
    let a1630 = {
        let a1627 = f130(v6, 48u32);
        let a1628 = {
            let a1625 = f130(v5, 40u32);
            let a1626 = {
                let a1623 = f130(v4, 32u32);
                let a1624 = {
                    let a1621 = f130(v3, 24u32);
                    let a1622 = {
                        let a1619 = f130(v2, 16u32);
                        let a1620 = {
                            let a1617 = f130(v1, 8u32);
                            let a1618 = v0;
                            fixed_u64::bit_or(a1617, a1618)
                        };
                        fixed_u64::bit_or(a1619, a1620)
                    };
                    fixed_u64::bit_or(a1621, a1622)
                };
                fixed_u64::bit_or(a1623, a1624)
            };
            fixed_u64::bit_or(a1625, a1626)
        };
        fixed_u64::bit_or(a1627, a1628)
    };
    fixed_u64::bit_or(a1629, a1630)
}

pub fn f151(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    let v5: u64 = {
        let a1631 = v4;
        let a1632 = v1;
        if nat_lt(a1631, a1632) {
            f160(v0, v4)
        } else {
            let a1633 = v4;
            let a1634 = v1;
            if nat_eq(a1633, a1634) {
                v3
            } else {
                0u64
            }
        }
    };
    let a1637 = {
        let a1635 = v4;
        let a1636 = 1u64;
        nat_add(a1635, a1636)?
    };
    let a1638 = v2;
    if nat_eq(a1637, a1638) {
        let a1639 = v5;
        let a1640 = 128u64;
        Ok(fixed_u64::bit_or(a1639, a1640))
    } else {
        Ok(v5)
    }
}

pub fn f152(v0: u64) -> u64 {
    let a1641 = v0;
    let a1642 = 40u64;
    if nat_lt(a1641, a1642) {
        let a1643 = v0;
        let a1644 = 20u64;
        if nat_lt(a1643, a1644) {
            let a1645 = v0;
            let a1646 = 10u64;
            if nat_lt(a1645, a1646) {
                let a1647 = v0;
                let a1648 = 5u64;
                if nat_lt(a1647, a1648) {
                    let a1649 = v0;
                    let a1650 = 2u64;
                    if nat_lt(a1649, a1650) {
                        let a1651 = v0;
                        let a1652 = 1u64;
                        if nat_lt(a1651, a1652) {
                            4794697086780616226u64
                        } else {
                            8158064640168781261u64
                        }
                    } else {
                        let a1653 = v0;
                        let a1654 = 3u64;
                        if nat_lt(a1653, a1654) {
                            13096744586834688815u64
                        } else {
                            let a1655 = v0;
                            let a1656 = 4u64;
                            if nat_lt(a1655, a1656) {
                                16840607885511220156u64
                            } else {
                                4131703408338449720u64
                            }
                        }
                    }
                } else {
                    let a1657 = v0;
                    let a1658 = 7u64;
                    if nat_lt(a1657, a1658) {
                        let a1659 = v0;
                        let a1660 = 6u64;
                        if nat_lt(a1659, a1660) {
                            6480981068601479193u64
                        } else {
                            10538285296894168987u64
                        }
                    } else {
                        let a1661 = v0;
                        let a1662 = 8u64;
                        if nat_lt(a1661, a1662) {
                            12329834152419229976u64
                        } else {
                            let a1663 = v0;
                            let a1664 = 9u64;
                            if nat_lt(a1663, a1664) {
                                15566598209576043074u64
                            } else {
                                1334009975649890238u64
                            }
                        }
                    }
                }
            } else {
                let a1665 = v0;
                let a1666 = 15u64;
                if nat_lt(a1665, a1666) {
                    let a1667 = v0;
                    let a1668 = 12u64;
                    if nat_lt(a1667, a1668) {
                        let a1669 = v0;
                        let a1670 = 11u64;
                        if nat_lt(a1669, a1670) {
                            2608012711638119052u64
                        } else {
                            6128411473006802146u64
                        }
                    } else {
                        let a1671 = v0;
                        let a1672 = 13u64;
                        if nat_lt(a1671, a1672) {
                            8268148722764581231u64
                        } else {
                            let a1673 = v0;
                            let a1674 = 14u64;
                            if nat_lt(a1673, a1674) {
                                9286055187155687089u64
                            } else {
                                11230858885718282805u64
                            }
                        }
                    }
                } else {
                    let a1675 = v0;
                    let a1676 = 17u64;
                    if nat_lt(a1675, a1676) {
                        let a1677 = v0;
                        let a1678 = 16u64;
                        if nat_lt(a1677, a1678) {
                            13951009754708518548u64
                        } else {
                            16472876342353939154u64
                        }
                    } else {
                        let a1679 = v0;
                        let a1680 = 18u64;
                        if nat_lt(a1679, a1680) {
                            17275323862435702243u64
                        } else {
                            let a1681 = v0;
                            let a1682 = 19u64;
                            if nat_lt(a1681, a1682) {
                                1135362057144423861u64
                            } else {
                                2597628984639134821u64
                            }
                        }
                    }
                }
            }
        } else {
            let a1683 = v0;
            let a1684 = 30u64;
            if nat_lt(a1683, a1684) {
                let a1685 = v0;
                let a1686 = 25u64;
                if nat_lt(a1685, a1686) {
                    let a1687 = v0;
                    let a1688 = 22u64;
                    if nat_lt(a1687, a1688) {
                        let a1689 = v0;
                        let a1690 = 21u64;
                        if nat_lt(a1689, a1690) {
                            3308224258029322869u64
                        } else {
                            5365058923640841347u64
                        }
                    } else {
                        let a1691 = v0;
                        let a1692 = 23u64;
                        if nat_lt(a1691, a1692) {
                            6679025012923562964u64
                        } else {
                            let a1693 = v0;
                            let a1694 = 24u64;
                            if nat_lt(a1693, a1694) {
                                8573033837759648693u64
                            } else {
                                10970295158949994411u64
                            }
                        }
                    }
                } else {
                    let a1695 = v0;
                    let a1696 = 27u64;
                    if nat_lt(a1695, a1696) {
                        let a1697 = v0;
                        let a1698 = 26u64;
                        if nat_lt(a1697, a1698) {
                            12119686244451234320u64
                        } else {
                            12683024718118986047u64
                        }
                    } else {
                        let a1699 = v0;
                        let a1700 = 28u64;
                        if nat_lt(a1699, a1700) {
                            13788192230050041572u64
                        } else {
                            let a1701 = v0;
                            let a1702 = 29u64;
                            if nat_lt(a1701, a1702) {
                                14330467153632333762u64
                            } else {
                                15395433587784984357u64
                            }
                        }
                    }
                }
            } else {
                let a1703 = v0;
                let a1704 = 35u64;
                if nat_lt(a1703, a1704) {
                    let a1705 = v0;
                    let a1706 = 32u64;
                    if nat_lt(a1705, a1706) {
                        let a1707 = v0;
                        let a1708 = 31u64;
                        if nat_lt(a1707, a1708) {
                            489312712824947311u64
                        } else {
                            1452737877330783856u64
                        }
                    } else {
                        let a1709 = v0;
                        let a1710 = 33u64;
                        if nat_lt(a1709, a1710) {
                            2861767655752347644u64
                        } else {
                            let a1711 = v0;
                            let a1712 = 34u64;
                            if nat_lt(a1711, a1712) {
                                3322285676063803686u64
                            } else {
                                5560940570517711597u64
                            }
                        }
                    }
                } else {
                    let a1713 = v0;
                    let a1714 = 37u64;
                    if nat_lt(a1713, a1714) {
                        let a1715 = v0;
                        let a1716 = 36u64;
                        if nat_lt(a1715, a1716) {
                            5996557281743188959u64
                        } else {
                            7280758554555802590u64
                        }
                    } else {
                        let a1717 = v0;
                        let a1718 = 38u64;
                        if nat_lt(a1717, a1718) {
                            8532644243296465576u64
                        } else {
                            let a1719 = v0;
                            let a1720 = 39u64;
                            if nat_lt(a1719, a1720) {
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
        let a1721 = v0;
        let a1722 = 60u64;
        if nat_lt(a1721, a1722) {
            let a1723 = v0;
            let a1724 = 50u64;
            if nat_lt(a1723, a1724) {
                let a1725 = v0;
                let a1726 = 45u64;
                if nat_lt(a1725, a1726) {
                    let a1727 = v0;
                    let a1728 = 42u64;
                    if nat_lt(a1727, a1728) {
                        let a1729 = v0;
                        let a1730 = 41u64;
                        if nat_lt(a1729, a1730) {
                            11727347734174303076u64
                        } else {
                            12113106623233404929u64
                        }
                    } else {
                        let a1731 = v0;
                        let a1732 = 43u64;
                        if nat_lt(a1731, a1732) {
                            14000437183269869457u64
                        } else {
                            let a1733 = v0;
                            let a1734 = 44u64;
                            if nat_lt(a1733, a1734) {
                                14369950271660146224u64
                            } else {
                                15101387698204529176u64
                            }
                        }
                    }
                } else {
                    let a1735 = v0;
                    let a1736 = 47u64;
                    if nat_lt(a1735, a1736) {
                        let a1737 = v0;
                        let a1738 = 46u64;
                        if nat_lt(a1737, a1738) {
                            15463397548674623760u64
                        } else {
                            17586052441742319658u64
                        }
                    } else {
                        let a1739 = v0;
                        let a1740 = 48u64;
                        if nat_lt(a1739, a1740) {
                            1182934255886127544u64
                        } else {
                            let a1741 = v0;
                            let a1742 = 49u64;
                            if nat_lt(a1741, a1742) {
                                1847814050463011016u64
                            } else {
                                2177327727835720531u64
                            }
                        }
                    }
                }
            } else {
                let a1743 = v0;
                let a1744 = 55u64;
                if nat_lt(a1743, a1744) {
                    let a1745 = v0;
                    let a1746 = 52u64;
                    if nat_lt(a1745, a1746) {
                        let a1747 = v0;
                        let a1748 = 51u64;
                        if nat_lt(a1747, a1748) {
                            2830643537854262169u64
                        } else {
                            3796741975233480872u64
                        }
                    } else {
                        let a1749 = v0;
                        let a1750 = 53u64;
                        if nat_lt(a1749, a1750) {
                            4115178125766777443u64
                        } else {
                            let a1751 = v0;
                            let a1752 = 54u64;
                            if nat_lt(a1751, a1752) {
                                5681478168544905931u64
                            } else {
                                6601373596472566643u64
                            }
                        }
                    }
                } else {
                    let a1753 = v0;
                    let a1754 = 57u64;
                    if nat_lt(a1753, a1754) {
                        let a1755 = v0;
                        let a1756 = 56u64;
                        if nat_lt(a1755, a1756) {
                            7507060721942968483u64
                        } else {
                            8399075790359081724u64
                        }
                    } else {
                        let a1757 = v0;
                        let a1758 = 58u64;
                        if nat_lt(a1757, a1758) {
                            8693463985226723168u64
                        } else {
                            let a1759 = v0;
                            let a1760 = 59u64;
                            if nat_lt(a1759, a1760) {
                                9568029438360202098u64
                            } else {
                                10144078919501101548u64
                            }
                        }
                    }
                }
            }
        } else {
            let a1761 = v0;
            let a1762 = 70u64;
            if nat_lt(a1761, a1762) {
                let a1763 = v0;
                let a1764 = 65u64;
                if nat_lt(a1763, a1764) {
                    let a1765 = v0;
                    let a1766 = 62u64;
                    if nat_lt(a1765, a1766) {
                        let a1767 = v0;
                        let a1768 = 61u64;
                        if nat_lt(a1767, a1768) {
                            10430055236837252648u64
                        } else {
                            11840083180663258601u64
                        }
                    } else {
                        let a1769 = v0;
                        let a1770 = 63u64;
                        if nat_lt(a1769, a1770) {
                            13761210420658862357u64
                        } else {
                            let a1771 = v0;
                            let a1772 = 64u64;
                            if nat_lt(a1771, a1772) {
                                14299343276471374635u64
                            } else {
                                14566680578165727644u64
                            }
                        }
                    }
                } else {
                    let a1773 = v0;
                    let a1774 = 67u64;
                    if nat_lt(a1773, a1774) {
                        let a1775 = v0;
                        let a1776 = 66u64;
                        if nat_lt(a1775, a1776) {
                            15097957966210449927u64
                        } else {
                            16922976911328602910u64
                        }
                    } else {
                        let a1777 = v0;
                        let a1778 = 68u64;
                        if nat_lt(a1777, a1778) {
                            17689382322260857208u64
                        } else {
                            let a1779 = v0;
                            let a1780 = 69u64;
                            if nat_lt(a1779, a1780) {
                                500013540394364858u64
                            } else {
                                748580250866718886u64
                            }
                        }
                    }
                }
            } else {
                let a1781 = v0;
                let a1782 = 75u64;
                if nat_lt(a1781, a1782) {
                    let a1783 = v0;
                    let a1784 = 72u64;
                    if nat_lt(a1783, a1784) {
                        let a1785 = v0;
                        let a1786 = 71u64;
                        if nat_lt(a1785, a1786) {
                            1242879168328830382u64
                        } else {
                            1977374033974150939u64
                        }
                    } else {
                        let a1787 = v0;
                        let a1788 = 73u64;
                        if nat_lt(a1787, a1788) {
                            2944078676154940804u64
                        } else {
                            let a1789 = v0;
                            let a1790 = 74u64;
                            if nat_lt(a1789, a1790) {
                                3659926193048069267u64
                            } else {
                                4368137639120453308u64
                            }
                        }
                    }
                } else {
                    let a1791 = v0;
                    let a1792 = 77u64;
                    if nat_lt(a1791, a1792) {
                        let a1793 = v0;
                        let a1794 = 76u64;
                        if nat_lt(a1793, a1794) {
                            4836135668995329356u64
                        } else {
                            5532061633213252278u64
                        }
                    } else {
                        let a1795 = v0;
                        let a1796 = 78u64;
                        if nat_lt(a1795, a1796) {
                            6448918945643986474u64
                        } else {
                            let a1797 = v0;
                            let a1798 = 79u64;
                            if nat_lt(a1797, a1798) {
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

pub fn f153(v0: u64) -> u64 {
    let a1801 = {
        let a1799 = f172(v0, 14u32, 50u32);
        let a1800 = f172(v0, 18u32, 46u32);
        fixed_u64::bit_xor(a1799, a1800)
    };
    let a1802 = f172(v0, 41u32, 23u32);
    fixed_u64::bit_xor(a1801, a1802)
}

pub fn f154(v0: u64, v1: u64, v2: u64) -> u64 {
    let a1808 = {
        let a1803 = v0;
        let a1804 = v1;
        fixed_u64::bit_and(a1803, a1804)
    };
    let a1809 = {
        let a1806 = {
            let a1805 = v0;
            fixed_u64::bit_not(a1805)
        };
        let a1807 = v2;
        fixed_u64::bit_and(a1806, a1807)
    };
    fixed_u64::bit_xor(a1808, a1809)
}

pub fn f155(v0: u64) -> u64 {
    let a1812 = {
        let a1810 = f172(v0, 28u32, 36u32);
        let a1811 = f172(v0, 34u32, 30u32);
        fixed_u64::bit_xor(a1810, a1811)
    };
    let a1813 = f172(v0, 39u32, 25u32);
    fixed_u64::bit_xor(a1812, a1813)
}

pub fn f156(v0: u64, v1: u64, v2: u64) -> u64 {
    let a1822 = {
        let a1818 = {
            let a1814 = v0;
            let a1815 = v1;
            fixed_u64::bit_and(a1814, a1815)
        };
        let a1819 = {
            let a1816 = v0;
            let a1817 = v2;
            fixed_u64::bit_and(a1816, a1817)
        };
        fixed_u64::bit_xor(a1818, a1819)
    };
    let a1823 = {
        let a1820 = v1;
        let a1821 = v2;
        fixed_u64::bit_and(a1820, a1821)
    };
    fixed_u64::bit_xor(a1822, a1823)
}

pub fn f157(v0: Adt10) -> Adt10 {
    Adt10::C0(match v0.clone() {
        Adt10::C0(_, h1824, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1824
        }
    }, match v0.clone() {
        Adt10::C0(_, _, h1825, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1825
        }
    }, match v0.clone() {
        Adt10::C0(_, _, _, h1826, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1826
        }
    }, match v0.clone() {
        Adt10::C0(_, _, _, _, h1827, _, _, _, _, _, _, _, _, _, _, _) => {
            h1827
        }
    }, match v0.clone() {
        Adt10::C0(_, _, _, _, _, h1828, _, _, _, _, _, _, _, _, _, _) => {
            h1828
        }
    }, match v0.clone() {
        Adt10::C0(_, _, _, _, _, _, h1829, _, _, _, _, _, _, _, _, _) => {
            h1829
        }
    }, match v0.clone() {
        Adt10::C0(_, _, _, _, _, _, _, h1830, _, _, _, _, _, _, _, _) => {
            h1830
        }
    }, match v0.clone() {
        Adt10::C0(_, _, _, _, _, _, _, _, h1831, _, _, _, _, _, _, _) => {
            h1831
        }
    }, match v0.clone() {
        Adt10::C0(_, _, _, _, _, _, _, _, _, h1832, _, _, _, _, _, _) => {
            h1832
        }
    }, match v0.clone() {
        Adt10::C0(_, _, _, _, _, _, _, _, _, _, h1833, _, _, _, _, _) => {
            h1833
        }
    }, match v0.clone() {
        Adt10::C0(_, _, _, _, _, _, _, _, _, _, _, h1834, _, _, _, _) => {
            h1834
        }
    }, match v0.clone() {
        Adt10::C0(_, _, _, _, _, _, _, _, _, _, _, _, h1835, _, _, _) => {
            h1835
        }
    }, match v0.clone() {
        Adt10::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1836, _, _) => {
            h1836
        }
    }, match v0.clone() {
        Adt10::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1837, _) => {
            h1837
        }
    }, match v0.clone() {
        Adt10::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1838) => {
            h1838
        }
    }, f102(f173(match v0.clone() {
        Adt10::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1839, _) => {
            h1839
        }
    }), f102(match v0.clone() {
        Adt10::C0(_, _, _, _, _, _, _, _, _, h1840, _, _, _, _, _, _) => {
            h1840
        }
    }, f102(f174(match v0.clone() {
        Adt10::C0(_, h1841, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1841
        }
    }), match v0 {
        Adt10::C0(h1842, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1842
        }
    }))))
}

pub fn f158(v0: Fn18, p1846: u64, p1847: (u64, (u64, u64))) -> (u64, (u64, u64)) {
    let m1845 = (p1846, p1847);
    {
        let mut m1845 = m1845;
        loop {
            let step = {
                let (v1, v2) = m1845;
                let m1843 = v1;
                if m1843 == 0 {
                    Err::<(u64, (u64, (u64, u64))), (u64, (u64, u64))>(v2)
                } else {
                    let v3 = m1843 - 1;
                    Ok::<(u64, (u64, (u64, u64))), (u64, (u64, u64))>((v3, {
                        let c1844: Fn18 = v0.clone();
                        c1844.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m1845 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f159(v0: (u64, (u64, u64))) -> (u64, (u64, u64)) {
    let v1: u64 = {
        let (h1848, _) = v0;
        h1848
    };
    let v2: u64 = {
        let (h1849, _) = {
            let (_, h1850) = v0;
            h1850
        };
        h1849
    };
    let v3: u64 = {
        let (_, h1851) = {
            let (_, h1852) = v0;
            h1852
        };
        h1851
    };
    ({
        let a1853 = v1;
        let a1854 = 2u64;
        let a1855 = 0u64;
        nat_quot(a1853, a1854, a1855)
    }, (f130(v2, 1u32), {
        let a1859 = {
            let a1856 = v1;
            let a1857 = 2u64;
            let a1858 = 0u64;
            nat_rem(a1856, a1857, a1858)
        };
        let a1860 = 1u64;
        if nat_eq(a1859, a1860) {
            let a1861 = v3;
            let a1862 = v2;
            fixed_u64::bit_or(a1861, a1862)
        } else {
            v3
        }
    }))
}

pub fn f160(v0: Bytes, v1: u64) -> u64 {
    let m1865 = {
        let a1863 = v0;
        let a1864 = v1;
        index_bytes(a1863, a1864)
    };
    match m1865 {
        None => {
            0u64
        }
        Some(v2) => {
            let m1867 = {
                let a1866 = v2;
                fixed_u64::convert(i128::from(a1866))
            };
            match m1867 {
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

pub fn f161(v0: Adt14) -> Adt14 {
    let a1872 = {
        let a1869 = match v0.clone() {
            Adt14::C0(_, h1868, _) => {
                h1868
            }
        };
        let a1870 = 2u64;
        let a1871 = 0u64;
        nat_rem(a1869, a1870, a1871)
    };
    let a1873 = 0u64;
    if nat_eq(a1872, a1873) {
        let m1875 = match v0.clone() {
            Adt14::C0(_, _, h1874) => {
                h1874
            }
        };
        match m1875.uncons() {
            None => {
                v0
            }
            Some((v1, v2)) => {
                Adt14::C0(f113(v1, match v0.clone() {
                    Adt14::C0(h1876, _, _) => {
                        h1876
                    }
                }, false), {
                    let a1878 = match v0 {
                        Adt14::C0(_, h1877, _) => {
                            h1877
                        }
                    };
                    let a1879 = 2u64;
                    let a1880 = 0u64;
                    nat_quot(a1878, a1879, a1880)
                }, v2)
            }
        }
    } else {
        v0
    }
}

pub fn f162(v0: Adt13, v1: Adt12) -> Adt13 {
    let v2: Adt15 = f175(match v0.clone() {
        Adt13::C0(h1881, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1881
        }
    }, match v0.clone() {
        Adt13::C0(_, _, _, _, _, h1882, _, _, _, _, _, _, _, _, _, _) => {
            h1882
        }
    }, match v0.clone() {
        Adt13::C0(_, _, _, _, _, _, _, _, _, _, h1883, _, _, _, _, _) => {
            h1883
        }
    }, match v0.clone() {
        Adt13::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1884) => {
            h1884
        }
    }, match v1.clone() {
        Adt12::C0(_, _, _, _, _, _, _, _, h1885, _, _, _, _, _, _, _) => {
            h1885
        }
    }, match v1.clone() {
        Adt12::C0(_, _, _, _, _, _, _, _, _, h1886, _, _, _, _, _, _) => {
            h1886
        }
    });
    let v3: Adt15 = f175(match v0.clone() {
        Adt13::C0(_, h1887, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1887
        }
    }, match v0.clone() {
        Adt13::C0(_, _, _, _, _, _, h1888, _, _, _, _, _, _, _, _, _) => {
            h1888
        }
    }, match v0.clone() {
        Adt13::C0(_, _, _, _, _, _, _, _, _, _, _, h1889, _, _, _, _) => {
            h1889
        }
    }, match v0.clone() {
        Adt13::C0(_, _, _, _, _, _, _, _, _, _, _, _, h1890, _, _, _) => {
            h1890
        }
    }, match v1.clone() {
        Adt12::C0(_, _, _, _, _, _, _, _, _, _, h1891, _, _, _, _, _) => {
            h1891
        }
    }, match v1.clone() {
        Adt12::C0(_, _, _, _, _, _, _, _, _, _, _, h1892, _, _, _, _) => {
            h1892
        }
    });
    let v4: Adt15 = f175(match v0.clone() {
        Adt13::C0(_, _, h1893, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1893
        }
    }, match v0.clone() {
        Adt13::C0(_, _, _, _, _, _, _, h1894, _, _, _, _, _, _, _, _) => {
            h1894
        }
    }, match v0.clone() {
        Adt13::C0(_, _, _, _, _, _, _, _, h1895, _, _, _, _, _, _, _) => {
            h1895
        }
    }, match v0.clone() {
        Adt13::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1896, _, _) => {
            h1896
        }
    }, match v1.clone() {
        Adt12::C0(_, _, _, _, _, _, _, _, _, _, _, _, h1897, _, _, _) => {
            h1897
        }
    }, match v1.clone() {
        Adt12::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1898, _, _) => {
            h1898
        }
    });
    let v5: Adt15 = f175(match v0.clone() {
        Adt13::C0(_, _, _, h1899, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1899
        }
    }, match v0.clone() {
        Adt13::C0(_, _, _, _, h1900, _, _, _, _, _, _, _, _, _, _, _) => {
            h1900
        }
    }, match v0.clone() {
        Adt13::C0(_, _, _, _, _, _, _, _, _, h1901, _, _, _, _, _, _) => {
            h1901
        }
    }, match v0 {
        Adt13::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1902, _) => {
            h1902
        }
    }, match v1.clone() {
        Adt12::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1903, _) => {
            h1903
        }
    }, match v1 {
        Adt12::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1904) => {
            h1904
        }
    });
    Adt13::C0(match v2.clone() {
        Adt15::C0(h1905, _, _, _) => {
            h1905
        }
    }, match v3.clone() {
        Adt15::C0(h1906, _, _, _) => {
            h1906
        }
    }, match v4.clone() {
        Adt15::C0(h1907, _, _, _) => {
            h1907
        }
    }, match v5.clone() {
        Adt15::C0(h1908, _, _, _) => {
            h1908
        }
    }, match v5.clone() {
        Adt15::C0(_, h1909, _, _) => {
            h1909
        }
    }, match v2.clone() {
        Adt15::C0(_, h1910, _, _) => {
            h1910
        }
    }, match v3.clone() {
        Adt15::C0(_, h1911, _, _) => {
            h1911
        }
    }, match v4.clone() {
        Adt15::C0(_, h1912, _, _) => {
            h1912
        }
    }, match v4.clone() {
        Adt15::C0(_, _, h1913, _) => {
            h1913
        }
    }, match v5.clone() {
        Adt15::C0(_, _, h1914, _) => {
            h1914
        }
    }, match v2.clone() {
        Adt15::C0(_, _, h1915, _) => {
            h1915
        }
    }, match v3.clone() {
        Adt15::C0(_, _, h1916, _) => {
            h1916
        }
    }, match v3 {
        Adt15::C0(_, _, _, h1917) => {
            h1917
        }
    }, match v4 {
        Adt15::C0(_, _, _, h1918) => {
            h1918
        }
    }, match v5 {
        Adt15::C0(_, _, _, h1919) => {
            h1919
        }
    }, match v2 {
        Adt15::C0(_, _, _, h1920) => {
            h1920
        }
    })
}

pub fn f163(v0: Adt13, v1: Adt12) -> Adt13 {
    let v2: Adt15 = f175(match v0.clone() {
        Adt13::C0(h1921, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1921
        }
    }, match v0.clone() {
        Adt13::C0(_, _, _, _, h1922, _, _, _, _, _, _, _, _, _, _, _) => {
            h1922
        }
    }, match v0.clone() {
        Adt13::C0(_, _, _, _, _, _, _, _, h1923, _, _, _, _, _, _, _) => {
            h1923
        }
    }, match v0.clone() {
        Adt13::C0(_, _, _, _, _, _, _, _, _, _, _, _, h1924, _, _, _) => {
            h1924
        }
    }, match v1.clone() {
        Adt12::C0(h1925, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1925
        }
    }, match v1.clone() {
        Adt12::C0(_, h1926, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1926
        }
    });
    let v3: Adt15 = f175(match v0.clone() {
        Adt13::C0(_, h1927, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1927
        }
    }, match v0.clone() {
        Adt13::C0(_, _, _, _, _, h1928, _, _, _, _, _, _, _, _, _, _) => {
            h1928
        }
    }, match v0.clone() {
        Adt13::C0(_, _, _, _, _, _, _, _, _, h1929, _, _, _, _, _, _) => {
            h1929
        }
    }, match v0.clone() {
        Adt13::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1930, _, _) => {
            h1930
        }
    }, match v1.clone() {
        Adt12::C0(_, _, h1931, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1931
        }
    }, match v1.clone() {
        Adt12::C0(_, _, _, h1932, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1932
        }
    });
    let v4: Adt15 = f175(match v0.clone() {
        Adt13::C0(_, _, h1933, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1933
        }
    }, match v0.clone() {
        Adt13::C0(_, _, _, _, _, _, h1934, _, _, _, _, _, _, _, _, _) => {
            h1934
        }
    }, match v0.clone() {
        Adt13::C0(_, _, _, _, _, _, _, _, _, _, h1935, _, _, _, _, _) => {
            h1935
        }
    }, match v0.clone() {
        Adt13::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1936, _) => {
            h1936
        }
    }, match v1.clone() {
        Adt12::C0(_, _, _, _, h1937, _, _, _, _, _, _, _, _, _, _, _) => {
            h1937
        }
    }, match v1.clone() {
        Adt12::C0(_, _, _, _, _, h1938, _, _, _, _, _, _, _, _, _, _) => {
            h1938
        }
    });
    let v5: Adt15 = f175(match v0.clone() {
        Adt13::C0(_, _, _, h1939, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1939
        }
    }, match v0.clone() {
        Adt13::C0(_, _, _, _, _, _, _, h1940, _, _, _, _, _, _, _, _) => {
            h1940
        }
    }, match v0.clone() {
        Adt13::C0(_, _, _, _, _, _, _, _, _, _, _, h1941, _, _, _, _) => {
            h1941
        }
    }, match v0 {
        Adt13::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1942) => {
            h1942
        }
    }, match v1.clone() {
        Adt12::C0(_, _, _, _, _, _, h1943, _, _, _, _, _, _, _, _, _) => {
            h1943
        }
    }, match v1 {
        Adt12::C0(_, _, _, _, _, _, _, h1944, _, _, _, _, _, _, _, _) => {
            h1944
        }
    });
    Adt13::C0(match v2.clone() {
        Adt15::C0(h1945, _, _, _) => {
            h1945
        }
    }, match v3.clone() {
        Adt15::C0(h1946, _, _, _) => {
            h1946
        }
    }, match v4.clone() {
        Adt15::C0(h1947, _, _, _) => {
            h1947
        }
    }, match v5.clone() {
        Adt15::C0(h1948, _, _, _) => {
            h1948
        }
    }, match v2.clone() {
        Adt15::C0(_, h1949, _, _) => {
            h1949
        }
    }, match v3.clone() {
        Adt15::C0(_, h1950, _, _) => {
            h1950
        }
    }, match v4.clone() {
        Adt15::C0(_, h1951, _, _) => {
            h1951
        }
    }, match v5.clone() {
        Adt15::C0(_, h1952, _, _) => {
            h1952
        }
    }, match v2.clone() {
        Adt15::C0(_, _, h1953, _) => {
            h1953
        }
    }, match v3.clone() {
        Adt15::C0(_, _, h1954, _) => {
            h1954
        }
    }, match v4.clone() {
        Adt15::C0(_, _, h1955, _) => {
            h1955
        }
    }, match v5.clone() {
        Adt15::C0(_, _, h1956, _) => {
            h1956
        }
    }, match v2 {
        Adt15::C0(_, _, _, h1957) => {
            h1957
        }
    }, match v3 {
        Adt15::C0(_, _, _, h1958) => {
            h1958
        }
    }, match v4 {
        Adt15::C0(_, _, _, h1959) => {
            h1959
        }
    }, match v5 {
        Adt15::C0(_, _, _, h1960) => {
            h1960
        }
    })
}

pub fn f164(v0: Adt12) -> Adt12 {
    Adt12::C0(match v0.clone() {
        Adt12::C0(_, _, h1961, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1961
        }
    }, match v0.clone() {
        Adt12::C0(_, _, _, _, _, _, h1962, _, _, _, _, _, _, _, _, _) => {
            h1962
        }
    }, match v0.clone() {
        Adt12::C0(_, _, _, h1963, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1963
        }
    }, match v0.clone() {
        Adt12::C0(_, _, _, _, _, _, _, _, _, _, h1964, _, _, _, _, _) => {
            h1964
        }
    }, match v0.clone() {
        Adt12::C0(_, _, _, _, _, _, _, h1965, _, _, _, _, _, _, _, _) => {
            h1965
        }
    }, match v0.clone() {
        Adt12::C0(h1966, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1966
        }
    }, match v0.clone() {
        Adt12::C0(_, _, _, _, h1967, _, _, _, _, _, _, _, _, _, _, _) => {
            h1967
        }
    }, match v0.clone() {
        Adt12::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1968, _, _) => {
            h1968
        }
    }, match v0.clone() {
        Adt12::C0(_, h1969, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1969
        }
    }, match v0.clone() {
        Adt12::C0(_, _, _, _, _, _, _, _, _, _, _, h1970, _, _, _, _) => {
            h1970
        }
    }, match v0.clone() {
        Adt12::C0(_, _, _, _, _, _, _, _, _, _, _, _, h1971, _, _, _) => {
            h1971
        }
    }, match v0.clone() {
        Adt12::C0(_, _, _, _, _, h1972, _, _, _, _, _, _, _, _, _, _) => {
            h1972
        }
    }, match v0.clone() {
        Adt12::C0(_, _, _, _, _, _, _, _, _, h1973, _, _, _, _, _, _) => {
            h1973
        }
    }, match v0.clone() {
        Adt12::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1974, _) => {
            h1974
        }
    }, match v0.clone() {
        Adt12::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1975) => {
            h1975
        }
    }, match v0 {
        Adt12::C0(_, _, _, _, _, _, _, _, h1976, _, _, _, _, _, _, _) => {
            h1976
        }
    })
}

pub fn f165(v0: u64) -> u64 {
    let a1977 = 47u64;
    let a1978 = v0;
    if nat_lt(a1977, a1978) {
        let a1979 = v0;
        let a1980 = 58u64;
        if nat_lt(a1979, a1980) {
            let a1981 = v0;
            let a1982 = 48u64;
            nat_sub(a1981, a1982)
        } else {
            let a1983 = 64u64;
            let a1984 = v0;
            if nat_lt(a1983, a1984) {
                let a1985 = v0;
                let a1986 = 71u64;
                if nat_lt(a1985, a1986) {
                    let a1987 = v0;
                    let a1988 = 55u64;
                    nat_sub(a1987, a1988)
                } else {
                    let a1989 = 96u64;
                    let a1990 = v0;
                    if nat_lt(a1989, a1990) {
                        let a1991 = v0;
                        let a1992 = 103u64;
                        if nat_lt(a1991, a1992) {
                            let a1993 = v0;
                            let a1994 = 87u64;
                            nat_sub(a1993, a1994)
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

pub fn f166(v0: u64) -> R<Bytes> {
    let a2005 = f121({
        let a1998 = 192u64;
        let a1999 = {
            let a1995 = v0;
            let a1996 = 64u64;
            let a1997 = 0u64;
            nat_quot(a1995, a1996, a1997)
        };
        nat_add(a1998, a1999)?
    });
    let a2006 = f121({
        let a2003 = 128u64;
        let a2004 = {
            let a2000 = v0;
            let a2001 = 64u64;
            let a2002 = 0u64;
            nat_rem(a2000, a2001, a2002)
        };
        nat_add(a2003, a2004)?
    });
    Ok(append_bytes(a2005, a2006))
}

pub fn f167(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f176(v0, v1)?;
    let a2007 = v2;
    let a2008 = 53u64;
    if nat_lt(a2007, a2008) {
        f18(Bytes::lit(&[0x00, 0x41, 0xc0, 0x00, 0x45, 0xc8, 0x00, 0x49, 0xcc, 0x00, 0x4f, 0xd2, 0x00, 0x55, 0xd9, 0x00, 0x61, 0xe0, 0x00, 0x65, 0xe8, 0x00, 0x69, 0xec, 0x00, 0x6f, 0xf2, 0x00, 0x75, 0xf9, 0x01, 0x41, 0xc1, 0x01, 0x45, 0xc9, 0x01, 0x49, 0xcd, 0x01, 0x4f, 0xd3, 0x01, 0x55, 0xda, 0x01, 0x59, 0xdd, 0x01, 0x61, 0xe1, 0x01, 0x65, 0xe9, 0x01, 0x69, 0xed, 0x01, 0x6f, 0xf3, 0x01, 0x75, 0xfa, 0x01, 0x79, 0xfd, 0x02, 0x41, 0xc2, 0x02, 0x45, 0xca, 0x02, 0x49, 0xce, 0x02, 0x4f, 0xd4, 0x02, 0x55, 0xdb, 0x02, 0x61, 0xe2, 0x02, 0x65, 0xea, 0x02, 0x69, 0xee, 0x02, 0x6f, 0xf4, 0x02, 0x75, 0xfb, 0x03, 0x41, 0xc3, 0x03, 0x4e, 0xd1, 0x03, 0x4f, 0xd5, 0x03, 0x61, 0xe3, 0x03, 0x6e, 0xf1, 0x03, 0x6f, 0xf5, 0x08, 0x41, 0xc4, 0x08, 0x45, 0xcb, 0x08, 0x49, 0xcf, 0x08, 0x4f, 0xd6, 0x08, 0x55, 0xdc, 0x08, 0x61, 0xe4, 0x08, 0x65, 0xeb, 0x08, 0x69, 0xef, 0x08, 0x6f, 0xf6, 0x08, 0x75, 0xfc, 0x08, 0x79, 0xff, 0x0a, 0x41, 0xc5, 0x0a, 0x61, 0xe5, 0x27, 0x43, 0xc7, 0x27, 0x63, 0xe7]), {
            let a2011 = {
                let a2009 = v2;
                let a2010 = 3u64;
                nat_mul(a2009, a2010)?
            };
            let a2012 = 2u64;
            nat_add(a2011, a2012)?
        })
    } else {
        Ok(0u64)
    }
}

pub fn f168(v0: u32, v1: u32, v2: u32) -> u32 {
    let a2013 = f66(v0, v1);
    let a2014 = f138(v0, v2);
    fixed_u32::bit_or(a2013, a2014)
}

pub fn f169(v0: u32) -> u32 {
    let a2017 = {
        let a2015 = f168(v0, 17u32, 15u32);
        let a2016 = f168(v0, 19u32, 13u32);
        fixed_u32::bit_xor(a2015, a2016)
    };
    let a2018 = f66(v0, 10u32);
    fixed_u32::bit_xor(a2017, a2018)
}

pub fn f170(v0: u32) -> u32 {
    let a2021 = {
        let a2019 = f168(v0, 7u32, 25u32);
        let a2020 = f168(v0, 18u32, 14u32);
        fixed_u32::bit_xor(a2019, a2020)
    };
    let a2022 = f66(v0, 3u32);
    fixed_u32::bit_xor(a2021, a2022)
}

pub fn f171(v0: (u64, Adt6)) -> R<(u64, Adt6)> {
    Ok(({
        let a2024 = {
            let (h2023, _) = v0.clone();
            h2023
        };
        let a2025 = 1u64;
        nat_add(a2024, a2025)?
    }, f177(f178(f179(f180({
        let (_, h2026) = v0;
        h2026
    }))), {
        let (h2027, _) = v0;
        h2027
    })))
}

pub fn f172(v0: u64, v1: u32, v2: u32) -> u64 {
    let a2028 = f75(v0, v1);
    let a2029 = f130(v0, v2);
    fixed_u64::bit_or(a2028, a2029)
}

pub fn f173(v0: u64) -> u64 {
    let a2032 = {
        let a2030 = f172(v0, 19u32, 45u32);
        let a2031 = f172(v0, 61u32, 3u32);
        fixed_u64::bit_xor(a2030, a2031)
    };
    let a2033 = f75(v0, 6u32);
    fixed_u64::bit_xor(a2032, a2033)
}

pub fn f174(v0: u64) -> u64 {
    let a2036 = {
        let a2034 = f172(v0, 1u32, 63u32);
        let a2035 = f172(v0, 8u32, 56u32);
        fixed_u64::bit_xor(a2034, a2035)
    };
    let a2037 = f75(v0, 7u32);
    fixed_u64::bit_xor(a2036, a2037)
}

pub fn f175(v0: u32, v1: u32, v2: u32, v3: u32, v4: u32, v5: u32) -> Adt15 {
    let v6: u32 = f97(f97(v0, v1), v4);
    let v7: u32 = f168({
        let a2038 = v3;
        let a2039 = v6;
        fixed_u32::bit_xor(a2038, a2039)
    }, 16u32, 16u32);
    let v8: u32 = f97(v2, v7);
    let v9: u32 = f168({
        let a2040 = v1;
        let a2041 = v8;
        fixed_u32::bit_xor(a2040, a2041)
    }, 12u32, 20u32);
    let v10: u32 = f97(f97(v6, v9), v5);
    let v11: u32 = f168({
        let a2042 = v7;
        let a2043 = v10;
        fixed_u32::bit_xor(a2042, a2043)
    }, 8u32, 24u32);
    let v12: u32 = f97(v8, v11);
    let v13: u32 = f168({
        let a2044 = v9;
        let a2045 = v12;
        fixed_u32::bit_xor(a2044, a2045)
    }, 7u32, 25u32);
    Adt15::C0(v10, v13, v12, v11)
}

pub fn f176(v0: u64, v1: u64) -> R<u64> {
    let (h2046, _) = f10(Fn1::F181(v1, v0), 54u64, 0u64)?;
    Ok(h2046)
}

pub fn f177(v0: Adt6, v1: u64) -> Adt6 {
    let v2: u64 = f182(v1);
    Adt6::C0({
        let a2048 = match v0.clone() {
            Adt6::C0(h2047, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2047
            }
        };
        let a2049 = v2;
        fixed_u64::bit_xor(a2048, a2049)
    }, match v0.clone() {
        Adt6::C0(_, h2050, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2050
        }
    }, match v0.clone() {
        Adt6::C0(_, _, h2051, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2051
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, h2052, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2052
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, h2053, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2053
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, h2054, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2054
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, h2055, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2055
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, h2056, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2056
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, h2057, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2057
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, h2058, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2058
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, _, h2059, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2059
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, h2060, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2060
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, h2061, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2061
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h2062, _, _, _, _, _, _, _, _, _, _, _) => {
            h2062
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2063, _, _, _, _, _, _, _, _, _, _) => {
            h2063
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2064, _, _, _, _, _, _, _, _, _) => {
            h2064
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2065, _, _, _, _, _, _, _, _) => {
            h2065
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2066, _, _, _, _, _, _, _) => {
            h2066
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2067, _, _, _, _, _, _) => {
            h2067
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2068, _, _, _, _, _) => {
            h2068
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2069, _, _, _, _) => {
            h2069
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2070, _, _, _) => {
            h2070
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2071, _, _) => {
            h2071
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2072, _) => {
            h2072
        }
    }, match v0 {
        Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2073) => {
            h2073
        }
    })
}

pub fn f178(v0: Adt6) -> Adt6 {
    Adt6::C0({
        let a2080 = match v0.clone() {
            Adt6::C0(h2074, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2074
            }
        };
        let a2081 = {
            let a2078 = {
                let a2076 = match v0.clone() {
                    Adt6::C0(_, h2075, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h2075
                    }
                };
                fixed_u64::bit_not(a2076)
            };
            let a2079 = match v0.clone() {
                Adt6::C0(_, _, h2077, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h2077
                }
            };
            fixed_u64::bit_and(a2078, a2079)
        };
        fixed_u64::bit_xor(a2080, a2081)
    }, {
        let a2088 = match v0.clone() {
            Adt6::C0(_, h2082, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2082
            }
        };
        let a2089 = {
            let a2086 = {
                let a2084 = match v0.clone() {
                    Adt6::C0(_, _, h2083, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h2083
                    }
                };
                fixed_u64::bit_not(a2084)
            };
            let a2087 = match v0.clone() {
                Adt6::C0(_, _, _, h2085, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h2085
                }
            };
            fixed_u64::bit_and(a2086, a2087)
        };
        fixed_u64::bit_xor(a2088, a2089)
    }, {
        let a2096 = match v0.clone() {
            Adt6::C0(_, _, h2090, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2090
            }
        };
        let a2097 = {
            let a2094 = {
                let a2092 = match v0.clone() {
                    Adt6::C0(_, _, _, h2091, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h2091
                    }
                };
                fixed_u64::bit_not(a2092)
            };
            let a2095 = match v0.clone() {
                Adt6::C0(_, _, _, _, h2093, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h2093
                }
            };
            fixed_u64::bit_and(a2094, a2095)
        };
        fixed_u64::bit_xor(a2096, a2097)
    }, {
        let a2104 = match v0.clone() {
            Adt6::C0(_, _, _, h2098, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2098
            }
        };
        let a2105 = {
            let a2102 = {
                let a2100 = match v0.clone() {
                    Adt6::C0(_, _, _, _, h2099, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h2099
                    }
                };
                fixed_u64::bit_not(a2100)
            };
            let a2103 = match v0.clone() {
                Adt6::C0(h2101, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h2101
                }
            };
            fixed_u64::bit_and(a2102, a2103)
        };
        fixed_u64::bit_xor(a2104, a2105)
    }, {
        let a2112 = match v0.clone() {
            Adt6::C0(_, _, _, _, h2106, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2106
            }
        };
        let a2113 = {
            let a2110 = {
                let a2108 = match v0.clone() {
                    Adt6::C0(h2107, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h2107
                    }
                };
                fixed_u64::bit_not(a2108)
            };
            let a2111 = match v0.clone() {
                Adt6::C0(_, h2109, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h2109
                }
            };
            fixed_u64::bit_and(a2110, a2111)
        };
        fixed_u64::bit_xor(a2112, a2113)
    }, {
        let a2120 = match v0.clone() {
            Adt6::C0(_, _, _, _, _, h2114, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2114
            }
        };
        let a2121 = {
            let a2118 = {
                let a2116 = match v0.clone() {
                    Adt6::C0(_, _, _, _, _, _, h2115, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h2115
                    }
                };
                fixed_u64::bit_not(a2116)
            };
            let a2119 = match v0.clone() {
                Adt6::C0(_, _, _, _, _, _, _, h2117, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h2117
                }
            };
            fixed_u64::bit_and(a2118, a2119)
        };
        fixed_u64::bit_xor(a2120, a2121)
    }, {
        let a2128 = match v0.clone() {
            Adt6::C0(_, _, _, _, _, _, h2122, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2122
            }
        };
        let a2129 = {
            let a2126 = {
                let a2124 = match v0.clone() {
                    Adt6::C0(_, _, _, _, _, _, _, h2123, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h2123
                    }
                };
                fixed_u64::bit_not(a2124)
            };
            let a2127 = match v0.clone() {
                Adt6::C0(_, _, _, _, _, _, _, _, h2125, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h2125
                }
            };
            fixed_u64::bit_and(a2126, a2127)
        };
        fixed_u64::bit_xor(a2128, a2129)
    }, {
        let a2136 = match v0.clone() {
            Adt6::C0(_, _, _, _, _, _, _, h2130, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2130
            }
        };
        let a2137 = {
            let a2134 = {
                let a2132 = match v0.clone() {
                    Adt6::C0(_, _, _, _, _, _, _, _, h2131, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h2131
                    }
                };
                fixed_u64::bit_not(a2132)
            };
            let a2135 = match v0.clone() {
                Adt6::C0(_, _, _, _, _, _, _, _, _, h2133, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h2133
                }
            };
            fixed_u64::bit_and(a2134, a2135)
        };
        fixed_u64::bit_xor(a2136, a2137)
    }, {
        let a2144 = match v0.clone() {
            Adt6::C0(_, _, _, _, _, _, _, _, h2138, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2138
            }
        };
        let a2145 = {
            let a2142 = {
                let a2140 = match v0.clone() {
                    Adt6::C0(_, _, _, _, _, _, _, _, _, h2139, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h2139
                    }
                };
                fixed_u64::bit_not(a2140)
            };
            let a2143 = match v0.clone() {
                Adt6::C0(_, _, _, _, _, h2141, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h2141
                }
            };
            fixed_u64::bit_and(a2142, a2143)
        };
        fixed_u64::bit_xor(a2144, a2145)
    }, {
        let a2152 = match v0.clone() {
            Adt6::C0(_, _, _, _, _, _, _, _, _, h2146, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2146
            }
        };
        let a2153 = {
            let a2150 = {
                let a2148 = match v0.clone() {
                    Adt6::C0(_, _, _, _, _, h2147, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h2147
                    }
                };
                fixed_u64::bit_not(a2148)
            };
            let a2151 = match v0.clone() {
                Adt6::C0(_, _, _, _, _, _, h2149, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h2149
                }
            };
            fixed_u64::bit_and(a2150, a2151)
        };
        fixed_u64::bit_xor(a2152, a2153)
    }, {
        let a2160 = match v0.clone() {
            Adt6::C0(_, _, _, _, _, _, _, _, _, _, h2154, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2154
            }
        };
        let a2161 = {
            let a2158 = {
                let a2156 = match v0.clone() {
                    Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, h2155, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h2155
                    }
                };
                fixed_u64::bit_not(a2156)
            };
            let a2159 = match v0.clone() {
                Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, h2157, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h2157
                }
            };
            fixed_u64::bit_and(a2158, a2159)
        };
        fixed_u64::bit_xor(a2160, a2161)
    }, {
        let a2168 = match v0.clone() {
            Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, h2162, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2162
            }
        };
        let a2169 = {
            let a2166 = {
                let a2164 = match v0.clone() {
                    Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, h2163, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h2163
                    }
                };
                fixed_u64::bit_not(a2164)
            };
            let a2167 = match v0.clone() {
                Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h2165, _, _, _, _, _, _, _, _, _, _, _) => {
                    h2165
                }
            };
            fixed_u64::bit_and(a2166, a2167)
        };
        fixed_u64::bit_xor(a2168, a2169)
    }, {
        let a2176 = match v0.clone() {
            Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, h2170, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2170
            }
        };
        let a2177 = {
            let a2174 = {
                let a2172 = match v0.clone() {
                    Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h2171, _, _, _, _, _, _, _, _, _, _, _) => {
                        h2171
                    }
                };
                fixed_u64::bit_not(a2172)
            };
            let a2175 = match v0.clone() {
                Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2173, _, _, _, _, _, _, _, _, _, _) => {
                    h2173
                }
            };
            fixed_u64::bit_and(a2174, a2175)
        };
        fixed_u64::bit_xor(a2176, a2177)
    }, {
        let a2184 = match v0.clone() {
            Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h2178, _, _, _, _, _, _, _, _, _, _, _) => {
                h2178
            }
        };
        let a2185 = {
            let a2182 = {
                let a2180 = match v0.clone() {
                    Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2179, _, _, _, _, _, _, _, _, _, _) => {
                        h2179
                    }
                };
                fixed_u64::bit_not(a2180)
            };
            let a2183 = match v0.clone() {
                Adt6::C0(_, _, _, _, _, _, _, _, _, _, h2181, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h2181
                }
            };
            fixed_u64::bit_and(a2182, a2183)
        };
        fixed_u64::bit_xor(a2184, a2185)
    }, {
        let a2192 = match v0.clone() {
            Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2186, _, _, _, _, _, _, _, _, _, _) => {
                h2186
            }
        };
        let a2193 = {
            let a2190 = {
                let a2188 = match v0.clone() {
                    Adt6::C0(_, _, _, _, _, _, _, _, _, _, h2187, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h2187
                    }
                };
                fixed_u64::bit_not(a2188)
            };
            let a2191 = match v0.clone() {
                Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, h2189, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h2189
                }
            };
            fixed_u64::bit_and(a2190, a2191)
        };
        fixed_u64::bit_xor(a2192, a2193)
    }, {
        let a2200 = match v0.clone() {
            Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2194, _, _, _, _, _, _, _, _, _) => {
                h2194
            }
        };
        let a2201 = {
            let a2198 = {
                let a2196 = match v0.clone() {
                    Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2195, _, _, _, _, _, _, _, _) => {
                        h2195
                    }
                };
                fixed_u64::bit_not(a2196)
            };
            let a2199 = match v0.clone() {
                Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2197, _, _, _, _, _, _, _) => {
                    h2197
                }
            };
            fixed_u64::bit_and(a2198, a2199)
        };
        fixed_u64::bit_xor(a2200, a2201)
    }, {
        let a2208 = match v0.clone() {
            Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2202, _, _, _, _, _, _, _, _) => {
                h2202
            }
        };
        let a2209 = {
            let a2206 = {
                let a2204 = match v0.clone() {
                    Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2203, _, _, _, _, _, _, _) => {
                        h2203
                    }
                };
                fixed_u64::bit_not(a2204)
            };
            let a2207 = match v0.clone() {
                Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2205, _, _, _, _, _, _) => {
                    h2205
                }
            };
            fixed_u64::bit_and(a2206, a2207)
        };
        fixed_u64::bit_xor(a2208, a2209)
    }, {
        let a2216 = match v0.clone() {
            Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2210, _, _, _, _, _, _, _) => {
                h2210
            }
        };
        let a2217 = {
            let a2214 = {
                let a2212 = match v0.clone() {
                    Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2211, _, _, _, _, _, _) => {
                        h2211
                    }
                };
                fixed_u64::bit_not(a2212)
            };
            let a2215 = match v0.clone() {
                Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2213, _, _, _, _, _) => {
                    h2213
                }
            };
            fixed_u64::bit_and(a2214, a2215)
        };
        fixed_u64::bit_xor(a2216, a2217)
    }, {
        let a2224 = match v0.clone() {
            Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2218, _, _, _, _, _, _) => {
                h2218
            }
        };
        let a2225 = {
            let a2222 = {
                let a2220 = match v0.clone() {
                    Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2219, _, _, _, _, _) => {
                        h2219
                    }
                };
                fixed_u64::bit_not(a2220)
            };
            let a2223 = match v0.clone() {
                Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2221, _, _, _, _, _, _, _, _, _) => {
                    h2221
                }
            };
            fixed_u64::bit_and(a2222, a2223)
        };
        fixed_u64::bit_xor(a2224, a2225)
    }, {
        let a2232 = match v0.clone() {
            Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2226, _, _, _, _, _) => {
                h2226
            }
        };
        let a2233 = {
            let a2230 = {
                let a2228 = match v0.clone() {
                    Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2227, _, _, _, _, _, _, _, _, _) => {
                        h2227
                    }
                };
                fixed_u64::bit_not(a2228)
            };
            let a2231 = match v0.clone() {
                Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2229, _, _, _, _, _, _, _, _) => {
                    h2229
                }
            };
            fixed_u64::bit_and(a2230, a2231)
        };
        fixed_u64::bit_xor(a2232, a2233)
    }, {
        let a2240 = match v0.clone() {
            Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2234, _, _, _, _) => {
                h2234
            }
        };
        let a2241 = {
            let a2238 = {
                let a2236 = match v0.clone() {
                    Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2235, _, _, _) => {
                        h2235
                    }
                };
                fixed_u64::bit_not(a2236)
            };
            let a2239 = match v0.clone() {
                Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2237, _, _) => {
                    h2237
                }
            };
            fixed_u64::bit_and(a2238, a2239)
        };
        fixed_u64::bit_xor(a2240, a2241)
    }, {
        let a2248 = match v0.clone() {
            Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2242, _, _, _) => {
                h2242
            }
        };
        let a2249 = {
            let a2246 = {
                let a2244 = match v0.clone() {
                    Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2243, _, _) => {
                        h2243
                    }
                };
                fixed_u64::bit_not(a2244)
            };
            let a2247 = match v0.clone() {
                Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2245, _) => {
                    h2245
                }
            };
            fixed_u64::bit_and(a2246, a2247)
        };
        fixed_u64::bit_xor(a2248, a2249)
    }, {
        let a2256 = match v0.clone() {
            Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2250, _, _) => {
                h2250
            }
        };
        let a2257 = {
            let a2254 = {
                let a2252 = match v0.clone() {
                    Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2251, _) => {
                        h2251
                    }
                };
                fixed_u64::bit_not(a2252)
            };
            let a2255 = match v0.clone() {
                Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2253) => {
                    h2253
                }
            };
            fixed_u64::bit_and(a2254, a2255)
        };
        fixed_u64::bit_xor(a2256, a2257)
    }, {
        let a2264 = match v0.clone() {
            Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2258, _) => {
                h2258
            }
        };
        let a2265 = {
            let a2262 = {
                let a2260 = match v0.clone() {
                    Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2259) => {
                        h2259
                    }
                };
                fixed_u64::bit_not(a2260)
            };
            let a2263 = match v0.clone() {
                Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2261, _, _, _, _) => {
                    h2261
                }
            };
            fixed_u64::bit_and(a2262, a2263)
        };
        fixed_u64::bit_xor(a2264, a2265)
    }, {
        let a2272 = match v0.clone() {
            Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2266) => {
                h2266
            }
        };
        let a2273 = {
            let a2270 = {
                let a2268 = match v0.clone() {
                    Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2267, _, _, _, _) => {
                        h2267
                    }
                };
                fixed_u64::bit_not(a2268)
            };
            let a2271 = match v0 {
                Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2269, _, _, _) => {
                    h2269
                }
            };
            fixed_u64::bit_and(a2270, a2271)
        };
        fixed_u64::bit_xor(a2272, a2273)
    })
}

pub fn f179(v0: Adt6) -> Adt6 {
    Adt6::C0(match v0.clone() {
        Adt6::C0(h2274, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2274
        }
    }, f183(match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, h2275, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2275
        }
    }, 44u32, 20u32), f183(match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, h2276, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2276
        }
    }, 43u32, 21u32), f183(match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2277, _, _, _, _, _, _) => {
            h2277
        }
    }, 21u32, 43u32), f183(match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2278) => {
            h2278
        }
    }, 14u32, 50u32), f183(match v0.clone() {
        Adt6::C0(_, _, _, h2279, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2279
        }
    }, 28u32, 36u32), f183(match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, h2280, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2280
        }
    }, 20u32, 44u32), f183(match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, _, h2281, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2281
        }
    }, 3u32, 61u32), f183(match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2282, _, _, _, _, _, _, _, _) => {
            h2282
        }
    }, 45u32, 19u32), f183(match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2283, _, _) => {
            h2283
        }
    }, 61u32, 3u32), f183(match v0.clone() {
        Adt6::C0(_, h2284, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2284
        }
    }, 1u32, 63u32), f183(match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, h2285, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2285
        }
    }, 6u32, 58u32), f183(match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h2286, _, _, _, _, _, _, _, _, _, _, _) => {
            h2286
        }
    }, 25u32, 39u32), f183(match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2287, _, _, _, _, _) => {
            h2287
        }
    }, 8u32, 56u32), f183(match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2288, _, _, _, _) => {
            h2288
        }
    }, 18u32, 46u32), f183(match v0.clone() {
        Adt6::C0(_, _, _, _, h2289, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2289
        }
    }, 27u32, 37u32), f183(match v0.clone() {
        Adt6::C0(_, _, _, _, _, h2290, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2290
        }
    }, 36u32, 28u32), f183(match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, h2291, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2291
        }
    }, 10u32, 54u32), f183(match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2292, _, _, _, _, _, _, _) => {
            h2292
        }
    }, 15u32, 49u32), f183(match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2293, _) => {
            h2293
        }
    }, 56u32, 8u32), f183(match v0.clone() {
        Adt6::C0(_, _, h2294, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2294
        }
    }, 62u32, 2u32), f183(match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, h2295, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2295
        }
    }, 55u32, 9u32), f183(match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2296, _, _, _, _, _, _, _, _, _, _) => {
            h2296
        }
    }, 39u32, 25u32), f183(match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2297, _, _, _, _, _, _, _, _, _) => {
            h2297
        }
    }, 41u32, 23u32), f183(match v0 {
        Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2298, _, _, _) => {
            h2298
        }
    }, 2u32, 62u32))
}

pub fn f180(v0: Adt6) -> Adt6 {
    let v1: u64 = {
        let a2310 = {
            let a2307 = {
                let a2304 = {
                    let a2301 = match v0.clone() {
                        Adt6::C0(h2299, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h2299
                        }
                    };
                    let a2302 = match v0.clone() {
                        Adt6::C0(_, _, _, _, _, h2300, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h2300
                        }
                    };
                    fixed_u64::bit_xor(a2301, a2302)
                };
                let a2305 = match v0.clone() {
                    Adt6::C0(_, _, _, _, _, _, _, _, _, _, h2303, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h2303
                    }
                };
                fixed_u64::bit_xor(a2304, a2305)
            };
            let a2308 = match v0.clone() {
                Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2306, _, _, _, _, _, _, _, _, _) => {
                    h2306
                }
            };
            fixed_u64::bit_xor(a2307, a2308)
        };
        let a2311 = match v0.clone() {
            Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2309, _, _, _, _) => {
                h2309
            }
        };
        fixed_u64::bit_xor(a2310, a2311)
    };
    let v2: u64 = {
        let a2323 = {
            let a2320 = {
                let a2317 = {
                    let a2314 = match v0.clone() {
                        Adt6::C0(_, h2312, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h2312
                        }
                    };
                    let a2315 = match v0.clone() {
                        Adt6::C0(_, _, _, _, _, _, h2313, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h2313
                        }
                    };
                    fixed_u64::bit_xor(a2314, a2315)
                };
                let a2318 = match v0.clone() {
                    Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, h2316, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h2316
                    }
                };
                fixed_u64::bit_xor(a2317, a2318)
            };
            let a2321 = match v0.clone() {
                Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2319, _, _, _, _, _, _, _, _) => {
                    h2319
                }
            };
            fixed_u64::bit_xor(a2320, a2321)
        };
        let a2324 = match v0.clone() {
            Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2322, _, _, _) => {
                h2322
            }
        };
        fixed_u64::bit_xor(a2323, a2324)
    };
    let v3: u64 = {
        let a2336 = {
            let a2333 = {
                let a2330 = {
                    let a2327 = match v0.clone() {
                        Adt6::C0(_, _, h2325, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h2325
                        }
                    };
                    let a2328 = match v0.clone() {
                        Adt6::C0(_, _, _, _, _, _, _, h2326, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h2326
                        }
                    };
                    fixed_u64::bit_xor(a2327, a2328)
                };
                let a2331 = match v0.clone() {
                    Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, h2329, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h2329
                    }
                };
                fixed_u64::bit_xor(a2330, a2331)
            };
            let a2334 = match v0.clone() {
                Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2332, _, _, _, _, _, _, _) => {
                    h2332
                }
            };
            fixed_u64::bit_xor(a2333, a2334)
        };
        let a2337 = match v0.clone() {
            Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2335, _, _) => {
                h2335
            }
        };
        fixed_u64::bit_xor(a2336, a2337)
    };
    let v4: u64 = {
        let a2349 = {
            let a2346 = {
                let a2343 = {
                    let a2340 = match v0.clone() {
                        Adt6::C0(_, _, _, h2338, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h2338
                        }
                    };
                    let a2341 = match v0.clone() {
                        Adt6::C0(_, _, _, _, _, _, _, _, h2339, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h2339
                        }
                    };
                    fixed_u64::bit_xor(a2340, a2341)
                };
                let a2344 = match v0.clone() {
                    Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h2342, _, _, _, _, _, _, _, _, _, _, _) => {
                        h2342
                    }
                };
                fixed_u64::bit_xor(a2343, a2344)
            };
            let a2347 = match v0.clone() {
                Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2345, _, _, _, _, _, _) => {
                    h2345
                }
            };
            fixed_u64::bit_xor(a2346, a2347)
        };
        let a2350 = match v0.clone() {
            Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2348, _) => {
                h2348
            }
        };
        fixed_u64::bit_xor(a2349, a2350)
    };
    let v5: u64 = {
        let a2362 = {
            let a2359 = {
                let a2356 = {
                    let a2353 = match v0.clone() {
                        Adt6::C0(_, _, _, _, h2351, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h2351
                        }
                    };
                    let a2354 = match v0.clone() {
                        Adt6::C0(_, _, _, _, _, _, _, _, _, h2352, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h2352
                        }
                    };
                    fixed_u64::bit_xor(a2353, a2354)
                };
                let a2357 = match v0.clone() {
                    Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2355, _, _, _, _, _, _, _, _, _, _) => {
                        h2355
                    }
                };
                fixed_u64::bit_xor(a2356, a2357)
            };
            let a2360 = match v0.clone() {
                Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2358, _, _, _, _, _) => {
                    h2358
                }
            };
            fixed_u64::bit_xor(a2359, a2360)
        };
        let a2363 = match v0.clone() {
            Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2361) => {
                h2361
            }
        };
        fixed_u64::bit_xor(a2362, a2363)
    };
    let v6: u64 = {
        let a2364 = v5;
        let a2365 = f183(v2, 1u32, 63u32);
        fixed_u64::bit_xor(a2364, a2365)
    };
    let v7: u64 = {
        let a2366 = v1;
        let a2367 = f183(v3, 1u32, 63u32);
        fixed_u64::bit_xor(a2366, a2367)
    };
    let v8: u64 = {
        let a2368 = v2;
        let a2369 = f183(v4, 1u32, 63u32);
        fixed_u64::bit_xor(a2368, a2369)
    };
    let v9: u64 = {
        let a2370 = v3;
        let a2371 = f183(v5, 1u32, 63u32);
        fixed_u64::bit_xor(a2370, a2371)
    };
    let v10: u64 = {
        let a2372 = v4;
        let a2373 = f183(v1, 1u32, 63u32);
        fixed_u64::bit_xor(a2372, a2373)
    };
    Adt6::C0({
        let a2375 = match v0.clone() {
            Adt6::C0(h2374, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2374
            }
        };
        let a2376 = v6;
        fixed_u64::bit_xor(a2375, a2376)
    }, {
        let a2378 = match v0.clone() {
            Adt6::C0(_, h2377, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2377
            }
        };
        let a2379 = v7;
        fixed_u64::bit_xor(a2378, a2379)
    }, {
        let a2381 = match v0.clone() {
            Adt6::C0(_, _, h2380, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2380
            }
        };
        let a2382 = v8;
        fixed_u64::bit_xor(a2381, a2382)
    }, {
        let a2384 = match v0.clone() {
            Adt6::C0(_, _, _, h2383, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2383
            }
        };
        let a2385 = v9;
        fixed_u64::bit_xor(a2384, a2385)
    }, {
        let a2387 = match v0.clone() {
            Adt6::C0(_, _, _, _, h2386, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2386
            }
        };
        let a2388 = v10;
        fixed_u64::bit_xor(a2387, a2388)
    }, {
        let a2390 = match v0.clone() {
            Adt6::C0(_, _, _, _, _, h2389, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2389
            }
        };
        let a2391 = v6;
        fixed_u64::bit_xor(a2390, a2391)
    }, {
        let a2393 = match v0.clone() {
            Adt6::C0(_, _, _, _, _, _, h2392, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2392
            }
        };
        let a2394 = v7;
        fixed_u64::bit_xor(a2393, a2394)
    }, {
        let a2396 = match v0.clone() {
            Adt6::C0(_, _, _, _, _, _, _, h2395, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2395
            }
        };
        let a2397 = v8;
        fixed_u64::bit_xor(a2396, a2397)
    }, {
        let a2399 = match v0.clone() {
            Adt6::C0(_, _, _, _, _, _, _, _, h2398, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2398
            }
        };
        let a2400 = v9;
        fixed_u64::bit_xor(a2399, a2400)
    }, {
        let a2402 = match v0.clone() {
            Adt6::C0(_, _, _, _, _, _, _, _, _, h2401, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2401
            }
        };
        let a2403 = v10;
        fixed_u64::bit_xor(a2402, a2403)
    }, {
        let a2405 = match v0.clone() {
            Adt6::C0(_, _, _, _, _, _, _, _, _, _, h2404, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2404
            }
        };
        let a2406 = v6;
        fixed_u64::bit_xor(a2405, a2406)
    }, {
        let a2408 = match v0.clone() {
            Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, h2407, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2407
            }
        };
        let a2409 = v7;
        fixed_u64::bit_xor(a2408, a2409)
    }, {
        let a2411 = match v0.clone() {
            Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, h2410, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2410
            }
        };
        let a2412 = v8;
        fixed_u64::bit_xor(a2411, a2412)
    }, {
        let a2414 = match v0.clone() {
            Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h2413, _, _, _, _, _, _, _, _, _, _, _) => {
                h2413
            }
        };
        let a2415 = v9;
        fixed_u64::bit_xor(a2414, a2415)
    }, {
        let a2417 = match v0.clone() {
            Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2416, _, _, _, _, _, _, _, _, _, _) => {
                h2416
            }
        };
        let a2418 = v10;
        fixed_u64::bit_xor(a2417, a2418)
    }, {
        let a2420 = match v0.clone() {
            Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2419, _, _, _, _, _, _, _, _, _) => {
                h2419
            }
        };
        let a2421 = v6;
        fixed_u64::bit_xor(a2420, a2421)
    }, {
        let a2423 = match v0.clone() {
            Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2422, _, _, _, _, _, _, _, _) => {
                h2422
            }
        };
        let a2424 = v7;
        fixed_u64::bit_xor(a2423, a2424)
    }, {
        let a2426 = match v0.clone() {
            Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2425, _, _, _, _, _, _, _) => {
                h2425
            }
        };
        let a2427 = v8;
        fixed_u64::bit_xor(a2426, a2427)
    }, {
        let a2429 = match v0.clone() {
            Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2428, _, _, _, _, _, _) => {
                h2428
            }
        };
        let a2430 = v9;
        fixed_u64::bit_xor(a2429, a2430)
    }, {
        let a2432 = match v0.clone() {
            Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2431, _, _, _, _, _) => {
                h2431
            }
        };
        let a2433 = v10;
        fixed_u64::bit_xor(a2432, a2433)
    }, {
        let a2435 = match v0.clone() {
            Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2434, _, _, _, _) => {
                h2434
            }
        };
        let a2436 = v6;
        fixed_u64::bit_xor(a2435, a2436)
    }, {
        let a2438 = match v0.clone() {
            Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2437, _, _, _) => {
                h2437
            }
        };
        let a2439 = v7;
        fixed_u64::bit_xor(a2438, a2439)
    }, {
        let a2441 = match v0.clone() {
            Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2440, _, _) => {
                h2440
            }
        };
        let a2442 = v8;
        fixed_u64::bit_xor(a2441, a2442)
    }, {
        let a2444 = match v0.clone() {
            Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2443, _) => {
                h2443
            }
        };
        let a2445 = v9;
        fixed_u64::bit_xor(a2444, a2445)
    }, {
        let a2447 = match v0 {
            Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2446) => {
                h2446
            }
        };
        let a2448 = v10;
        fixed_u64::bit_xor(a2447, a2448)
    })
}

pub fn f181(v0: u64, v1: u64, v2: u64) -> R<Option<u64>> {
    let v3: Bytes = Bytes::lit(&[0x00, 0x41, 0xc0, 0x00, 0x45, 0xc8, 0x00, 0x49, 0xcc, 0x00, 0x4f, 0xd2, 0x00, 0x55, 0xd9, 0x00, 0x61, 0xe0, 0x00, 0x65, 0xe8, 0x00, 0x69, 0xec, 0x00, 0x6f, 0xf2, 0x00, 0x75, 0xf9, 0x01, 0x41, 0xc1, 0x01, 0x45, 0xc9, 0x01, 0x49, 0xcd, 0x01, 0x4f, 0xd3, 0x01, 0x55, 0xda, 0x01, 0x59, 0xdd, 0x01, 0x61, 0xe1, 0x01, 0x65, 0xe9, 0x01, 0x69, 0xed, 0x01, 0x6f, 0xf3, 0x01, 0x75, 0xfa, 0x01, 0x79, 0xfd, 0x02, 0x41, 0xc2, 0x02, 0x45, 0xca, 0x02, 0x49, 0xce, 0x02, 0x4f, 0xd4, 0x02, 0x55, 0xdb, 0x02, 0x61, 0xe2, 0x02, 0x65, 0xea, 0x02, 0x69, 0xee, 0x02, 0x6f, 0xf4, 0x02, 0x75, 0xfb, 0x03, 0x41, 0xc3, 0x03, 0x4e, 0xd1, 0x03, 0x4f, 0xd5, 0x03, 0x61, 0xe3, 0x03, 0x6e, 0xf1, 0x03, 0x6f, 0xf5, 0x08, 0x41, 0xc4, 0x08, 0x45, 0xcb, 0x08, 0x49, 0xcf, 0x08, 0x4f, 0xd6, 0x08, 0x55, 0xdc, 0x08, 0x61, 0xe4, 0x08, 0x65, 0xeb, 0x08, 0x69, 0xef, 0x08, 0x6f, 0xf6, 0x08, 0x75, 0xfc, 0x08, 0x79, 0xff, 0x0a, 0x41, 0xc5, 0x0a, 0x61, 0xe5, 0x27, 0x43, 0xc7, 0x27, 0x63, 0xe7]);
    let a2449 = v2;
    let a2450 = 53u64;
    if nat_lt(a2449, a2450) {
        let a2453 = f18(v3.clone(), {
            let a2451 = v2;
            let a2452 = 3u64;
            nat_mul(a2451, a2452)?
        })?;
        let a2454 = v1;
        if nat_eq(a2453, a2454) {
            let a2459 = f18(v3, {
                let a2457 = {
                    let a2455 = v2;
                    let a2456 = 3u64;
                    nat_mul(a2455, a2456)?
                };
                let a2458 = 1u64;
                nat_add(a2457, a2458)?
            })?;
            let a2460 = v0;
            if nat_eq(a2459, a2460) {
                Ok(None::<u64>)
            } else {
                Ok(Some({
                    let a2461 = v2;
                    let a2462 = 1u64;
                    nat_add(a2461, a2462)?
                }))
            }
        } else {
            Ok(Some({
                let a2463 = v2;
                let a2464 = 1u64;
                nat_add(a2463, a2464)?
            }))
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f182(v0: u64) -> u64 {
    let a2465 = v0;
    let a2466 = 12u64;
    if nat_lt(a2465, a2466) {
        let a2467 = v0;
        let a2468 = 6u64;
        if nat_lt(a2467, a2468) {
            let a2469 = v0;
            let a2470 = 3u64;
            if nat_lt(a2469, a2470) {
                let a2471 = v0;
                let a2472 = 1u64;
                if nat_lt(a2471, a2472) {
                    1u64
                } else {
                    let a2473 = v0;
                    let a2474 = 2u64;
                    if nat_lt(a2473, a2474) {
                        32898u64
                    } else {
                        9223372036854808714u64
                    }
                }
            } else {
                let a2475 = v0;
                let a2476 = 4u64;
                if nat_lt(a2475, a2476) {
                    9223372039002292224u64
                } else {
                    let a2477 = v0;
                    let a2478 = 5u64;
                    if nat_lt(a2477, a2478) {
                        32907u64
                    } else {
                        2147483649u64
                    }
                }
            }
        } else {
            let a2479 = v0;
            let a2480 = 9u64;
            if nat_lt(a2479, a2480) {
                let a2481 = v0;
                let a2482 = 7u64;
                if nat_lt(a2481, a2482) {
                    9223372039002292353u64
                } else {
                    let a2483 = v0;
                    let a2484 = 8u64;
                    if nat_lt(a2483, a2484) {
                        9223372036854808585u64
                    } else {
                        138u64
                    }
                }
            } else {
                let a2485 = v0;
                let a2486 = 10u64;
                if nat_lt(a2485, a2486) {
                    136u64
                } else {
                    let a2487 = v0;
                    let a2488 = 11u64;
                    if nat_lt(a2487, a2488) {
                        2147516425u64
                    } else {
                        2147483658u64
                    }
                }
            }
        }
    } else {
        let a2489 = v0;
        let a2490 = 18u64;
        if nat_lt(a2489, a2490) {
            let a2491 = v0;
            let a2492 = 15u64;
            if nat_lt(a2491, a2492) {
                let a2493 = v0;
                let a2494 = 13u64;
                if nat_lt(a2493, a2494) {
                    2147516555u64
                } else {
                    let a2495 = v0;
                    let a2496 = 14u64;
                    if nat_lt(a2495, a2496) {
                        9223372036854775947u64
                    } else {
                        9223372036854808713u64
                    }
                }
            } else {
                let a2497 = v0;
                let a2498 = 16u64;
                if nat_lt(a2497, a2498) {
                    9223372036854808579u64
                } else {
                    let a2499 = v0;
                    let a2500 = 17u64;
                    if nat_lt(a2499, a2500) {
                        9223372036854808578u64
                    } else {
                        9223372036854775936u64
                    }
                }
            }
        } else {
            let a2501 = v0;
            let a2502 = 21u64;
            if nat_lt(a2501, a2502) {
                let a2503 = v0;
                let a2504 = 19u64;
                if nat_lt(a2503, a2504) {
                    32778u64
                } else {
                    let a2505 = v0;
                    let a2506 = 20u64;
                    if nat_lt(a2505, a2506) {
                        9223372039002259466u64
                    } else {
                        9223372039002292353u64
                    }
                }
            } else {
                let a2507 = v0;
                let a2508 = 22u64;
                if nat_lt(a2507, a2508) {
                    9223372036854808704u64
                } else {
                    let a2509 = v0;
                    let a2510 = 23u64;
                    if nat_lt(a2509, a2510) {
                        2147483649u64
                    } else {
                        9223372039002292232u64
                    }
                }
            }
        }
    }
}

pub fn f183(v0: u64, v1: u32, v2: u32) -> u64 {
    let a2511 = f130(v0, v1);
    let a2512 = f75(v0, v2);
    fixed_u64::bit_or(a2511, a2512)
}
