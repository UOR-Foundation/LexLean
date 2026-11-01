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
    F12(Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn0::F12(k0) => f12(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F19(Bytes, u64, u64),
}

impl Fn1 {
    pub fn apply(&self, p0: (u64, Adt0)) -> R<(u64, Adt0)> {
        match self {
            Fn1::F19(k0, k1, k2) => f19(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F27,
}

impl Fn2 {
    pub fn apply(&self, p0: Adt2) -> R<Adt2> {
        match self {
            Fn2::F27 => f27(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F43,
}

impl Fn3 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn3::F43 => f43(p0),
        }
    }
}

pub fn f0(v0: Bytes) -> R<Option<Bytes>> {
    if f1(v0.clone())? {
        Ok(Some(f2({
            let a1 = f3(Str::lit("sha256:"));
            let a2 = f4(v0);
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
        if f5(v0.clone(), f3(Str::lit("sha256:"))) {
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
    let a8 = f3(Str::lit("sha256:"));
    let a9 = f7(v0)?;
    Ok(append_bytes(a8, a9))
}

pub fn f3(v0: Str) -> Bytes {
    let a10 = v0;
    utf8_encode(a10)
}

pub fn f4(v0: Bytes) -> Bytes {
    let v1: Bytes = f8(v0.clone(), 7u64, 16u64);
    let v2: Bytes = f8(v0.clone(), 23u64, 16u64);
    let v3: Bytes = f8(v0.clone(), 39u64, 16u64);
    let v4: Bytes = f8(v0, 55u64, 16u64);
    let v5: Bytes = f9(v1.clone(), v2.clone());
    let v6: Bytes = f10(v1, v2);
    let v7: Bytes = f9(v3.clone(), v4.clone());
    let v8: Bytes = f10(v3, v4);
    let v9: Bytes = f9(v5.clone(), v7.clone());
    let v10: Bytes = f10(v5, v7);
    let v11: Bytes = f9(v6.clone(), v8.clone());
    let v12: Bytes = f10(v6, v8);
    let v13: Bytes = f9(v11.clone(), v10.clone());
    let v14: Bytes = f10(v11, v10);
    let a15 = v9;
    let a16 = {
        let a13 = v13;
        let a14 = {
            let a11 = v14;
            let a12 = v12;
            append_bytes(a11, a12)
        };
        append_bytes(a13, a14)
    };
    append_bytes(a15, a16)
}

pub fn f5(v0: Bytes, v1: Bytes) -> bool {
    let m21 = {
        let a18 = v0;
        let a19 = 0u64;
        let a20 = {
            let a17 = v1.clone();
            length_bytes(a17)
        };
        slice_bytes(a18, a19, a20)
    };
    match m21 {
        None => {
            false
        }
        Some(v2) => {
            let a22 = v2;
            let a23 = v1;
            equal(a22, a23)
        }
    }
}

pub fn f6(v0: Bytes, v1: u64) -> R<u64> {
    let (h24, _) = f11(Fn0::F12(v0.clone()), {
        let a25 = v0;
        length_bytes(a25)
    }, v1)?;
    Ok(h24)
}

pub fn f7(v0: Bytes) -> R<Bytes> {
    Ok(f13(f14(v0)?))
}

pub fn f8(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m29 = {
        let a26 = v0;
        let a27 = v1;
        let a28 = v2;
        slice_bytes(a26, a27, a28)
    };
    match m29 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f9(v0: Bytes, v1: Bytes) -> Bytes {
    if f15(v1.clone(), v0.clone()) {
        v1
    } else {
        v0
    }
}

pub fn f10(v0: Bytes, v1: Bytes) -> Bytes {
    if f15(v1.clone(), v0.clone()) {
        v0
    } else {
        v1
    }
}

pub fn f11(v0: Fn0, p34: u64, p35: u64) -> R<(u64, bool)> {
    let m33 = (p34, p35);
    {
        let mut m33 = m33;
        loop {
            let step = {
                let (v1, v2) = m33;
                let m30 = v1;
                if m30 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m30 - 1;
                    let m32 = {
                        let c31: Fn0 = v0.clone();
                        c31.apply(v2)?
                    };
                    match m32 {
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
                Ok(next) => { m33 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f12(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f16(v0, v1)? {
        Ok(Some({
            let a36 = v1;
            let a37 = 1u64;
            nat_add(a36, a37)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f13(v0: Adt0) -> Bytes {
    let a58 = f17(match v0.clone() {
        Adt0::C0(h38, _, _, _, _, _, _, _) => {
            h38
        }
    });
    let a59 = {
        let a56 = f17(match v0.clone() {
            Adt0::C0(_, h39, _, _, _, _, _, _) => {
                h39
            }
        });
        let a57 = {
            let a54 = f17(match v0.clone() {
                Adt0::C0(_, _, h40, _, _, _, _, _) => {
                    h40
                }
            });
            let a55 = {
                let a52 = f17(match v0.clone() {
                    Adt0::C0(_, _, _, h41, _, _, _, _) => {
                        h41
                    }
                });
                let a53 = {
                    let a50 = f17(match v0.clone() {
                        Adt0::C0(_, _, _, _, h42, _, _, _) => {
                            h42
                        }
                    });
                    let a51 = {
                        let a48 = f17(match v0.clone() {
                            Adt0::C0(_, _, _, _, _, h43, _, _) => {
                                h43
                            }
                        });
                        let a49 = {
                            let a46 = f17(match v0.clone() {
                                Adt0::C0(_, _, _, _, _, _, h44, _) => {
                                    h44
                                }
                            });
                            let a47 = f17(match v0 {
                                Adt0::C0(_, _, _, _, _, _, _, h45) => {
                                    h45
                                }
                            });
                            append_bytes(a46, a47)
                        };
                        append_bytes(a48, a49)
                    };
                    append_bytes(a50, a51)
                };
                append_bytes(a52, a53)
            };
            append_bytes(a54, a55)
        };
        append_bytes(a56, a57)
    };
    append_bytes(a58, a59)
}

pub fn f14(v0: Bytes) -> R<Adt0> {
    let v1: u64 = {
        let a60 = v0.clone();
        length_bytes(a60)
    };
    let v2: u64 = {
        let a66 = {
            let a63 = {
                let a61 = v1;
                let a62 = 72u64;
                nat_add(a61, a62)?
            };
            let a64 = 64u64;
            let a65 = 0u64;
            nat_quot(a63, a64, a65)
        };
        let a67 = 64u64;
        nat_mul(a66, a67)?
    };
    let (_, h68) = f18(Fn1::F19(v0, v1, v2), {
        let a69 = v2;
        let a70 = 64u64;
        let a71 = 0u64;
        nat_quot(a69, a70, a71)
    }, (0u64, f20()))?;
    Ok(h68)
}

pub fn f15(v0: Bytes, v1: Bytes) -> bool {
    let a76 = {
        let a72 = v0;
        let a73 = v1;
        compare_bytes(a72, a73)
    };
    let a77 = {
        let a74 = Bytes::lit(&[]);
        let a75 = Bytes::lit(&[0x01]);
        compare_bytes(a74, a75)
    };
    equal(a76, a77)
}

pub fn f16(v0: Bytes, v1: u64) -> R<bool> {
    let v2: u64 = f21(v0, v1)?;
    let a78 = 47u64;
    let a79 = v2;
    if nat_lt(a78, a79) {
        let a80 = v2;
        let a81 = 58u64;
        if nat_lt(a80, a81) {
            Ok(true)
        } else {
            let a82 = 96u64;
            let a83 = v2;
            if nat_lt(a82, a83) {
                let a84 = v2;
                let a85 = 103u64;
                Ok(nat_lt(a84, a85))
            } else {
                Ok(false)
            }
        }
    } else {
        Ok(false)
    }
}

pub fn f17(v0: u32) -> Bytes {
    let a114 = f22({
        let a86 = f23(v0, 28u32);
        let a87 = 15u32;
        fixed_u32::bit_and(a86, a87)
    });
    let a115 = {
        let a112 = f22({
            let a88 = f23(v0, 24u32);
            let a89 = 15u32;
            fixed_u32::bit_and(a88, a89)
        });
        let a113 = {
            let a110 = f22({
                let a90 = f23(v0, 20u32);
                let a91 = 15u32;
                fixed_u32::bit_and(a90, a91)
            });
            let a111 = {
                let a108 = f22({
                    let a92 = f23(v0, 16u32);
                    let a93 = 15u32;
                    fixed_u32::bit_and(a92, a93)
                });
                let a109 = {
                    let a106 = f22({
                        let a94 = f23(v0, 12u32);
                        let a95 = 15u32;
                        fixed_u32::bit_and(a94, a95)
                    });
                    let a107 = {
                        let a104 = f22({
                            let a96 = f23(v0, 8u32);
                            let a97 = 15u32;
                            fixed_u32::bit_and(a96, a97)
                        });
                        let a105 = {
                            let a102 = f22({
                                let a98 = f23(v0, 4u32);
                                let a99 = 15u32;
                                fixed_u32::bit_and(a98, a99)
                            });
                            let a103 = f22({
                                let a100 = v0;
                                let a101 = 15u32;
                                fixed_u32::bit_and(a100, a101)
                            });
                            append_bytes(a102, a103)
                        };
                        append_bytes(a104, a105)
                    };
                    append_bytes(a106, a107)
                };
                append_bytes(a108, a109)
            };
            append_bytes(a110, a111)
        };
        append_bytes(a112, a113)
    };
    append_bytes(a114, a115)
}

pub fn f18(v0: Fn1, p119: u64, p120: (u64, Adt0)) -> R<(u64, Adt0)> {
    let m118 = (p119, p120);
    {
        let mut m118 = m118;
        loop {
            let step = {
                let (v1, v2) = m118;
                let m116 = v1;
                if m116 == 0 {
                    Err::<(u64, (u64, Adt0)), R<(u64, Adt0)>>(Ok(v2))
                } else {
                    let v3 = m116 - 1;
                    Ok::<(u64, (u64, Adt0)), R<(u64, Adt0)>>((v3, {
                        let c117: Fn1 = v0.clone();
                        c117.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m118 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f19(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt0)) -> R<(u64, Adt0)> {
    Ok(({
        let a122 = {
            let (h121, _) = v3.clone();
            h121
        };
        let a123 = 64u64;
        nat_add(a122, a123)?
    }, f24({
        let (_, h124) = v3;
        h124
    }, f25(v0, v1, v2, {
        let (h125, _) = v3;
        h125
    })?)?))
}

pub fn f20() -> Adt0 {
    Adt0::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f21(v0: Bytes, v1: u64) -> R<u64> {
    let m128 = {
        let a126 = v0;
        let a127 = v1;
        index_bytes(a126, a127)
    };
    match m128 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a173 = {
                let a149 = {
                    let a137 = {
                        let a131 = {
                            let a129 = v3;
                            let a130 = 128u8;
                            fixed_u8::bit_and(a129, a130)
                        };
                        let a132 = 128u8;
                        if equal(a131, a132) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a138 = {
                        let a135 = {
                            let a133 = v3;
                            let a134 = 64u8;
                            fixed_u8::bit_and(a133, a134)
                        };
                        let a136 = 64u8;
                        if equal(a135, a136) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a137, a138)?
                };
                let a150 = {
                    let a147 = {
                        let a141 = {
                            let a139 = v3;
                            let a140 = 32u8;
                            fixed_u8::bit_and(a139, a140)
                        };
                        let a142 = 32u8;
                        if equal(a141, a142) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a148 = {
                        let a145 = {
                            let a143 = v3;
                            let a144 = 16u8;
                            fixed_u8::bit_and(a143, a144)
                        };
                        let a146 = 16u8;
                        if equal(a145, a146) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a147, a148)?
                };
                nat_add(a149, a150)?
            };
            let a174 = {
                let a171 = {
                    let a159 = {
                        let a153 = {
                            let a151 = v3;
                            let a152 = 8u8;
                            fixed_u8::bit_and(a151, a152)
                        };
                        let a154 = 8u8;
                        if equal(a153, a154) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a160 = {
                        let a157 = {
                            let a155 = v3;
                            let a156 = 4u8;
                            fixed_u8::bit_and(a155, a156)
                        };
                        let a158 = 4u8;
                        if equal(a157, a158) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a159, a160)?
                };
                let a172 = {
                    let a169 = {
                        let a163 = {
                            let a161 = v3;
                            let a162 = 2u8;
                            fixed_u8::bit_and(a161, a162)
                        };
                        let a164 = 2u8;
                        if equal(a163, a164) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a170 = {
                        let a167 = {
                            let a165 = v3;
                            let a166 = 1u8;
                            fixed_u8::bit_and(a165, a166)
                        };
                        let a168 = 1u8;
                        if equal(a167, a168) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a169, a170)?
                };
                nat_add(a171, a172)?
            };
            nat_add(a173, a174)
        }
    }
}

pub fn f22(v0: u32) -> Bytes {
    let a177 = {
        let a175 = v0;
        let a176 = 8u32;
        fixed_u32::bit_and(a175, a176)
    };
    let a178 = 0u32;
    if equal(a177, a178) {
        let a181 = {
            let a179 = v0;
            let a180 = 4u32;
            fixed_u32::bit_and(a179, a180)
        };
        let a182 = 0u32;
        if equal(a181, a182) {
            let a185 = {
                let a183 = v0;
                let a184 = 2u32;
                fixed_u32::bit_and(a183, a184)
            };
            let a186 = 0u32;
            if equal(a185, a186) {
                let a189 = {
                    let a187 = v0;
                    let a188 = 1u32;
                    fixed_u32::bit_and(a187, a188)
                };
                let a190 = 0u32;
                if equal(a189, a190) {
                    Bytes::lit(&[0x30])
                } else {
                    Bytes::lit(&[0x31])
                }
            } else {
                let a193 = {
                    let a191 = v0;
                    let a192 = 1u32;
                    fixed_u32::bit_and(a191, a192)
                };
                let a194 = 0u32;
                if equal(a193, a194) {
                    Bytes::lit(&[0x32])
                } else {
                    Bytes::lit(&[0x33])
                }
            }
        } else {
            let a197 = {
                let a195 = v0;
                let a196 = 2u32;
                fixed_u32::bit_and(a195, a196)
            };
            let a198 = 0u32;
            if equal(a197, a198) {
                let a201 = {
                    let a199 = v0;
                    let a200 = 1u32;
                    fixed_u32::bit_and(a199, a200)
                };
                let a202 = 0u32;
                if equal(a201, a202) {
                    Bytes::lit(&[0x34])
                } else {
                    Bytes::lit(&[0x35])
                }
            } else {
                let a205 = {
                    let a203 = v0;
                    let a204 = 1u32;
                    fixed_u32::bit_and(a203, a204)
                };
                let a206 = 0u32;
                if equal(a205, a206) {
                    Bytes::lit(&[0x36])
                } else {
                    Bytes::lit(&[0x37])
                }
            }
        }
    } else {
        let a209 = {
            let a207 = v0;
            let a208 = 4u32;
            fixed_u32::bit_and(a207, a208)
        };
        let a210 = 0u32;
        if equal(a209, a210) {
            let a213 = {
                let a211 = v0;
                let a212 = 2u32;
                fixed_u32::bit_and(a211, a212)
            };
            let a214 = 0u32;
            if equal(a213, a214) {
                let a217 = {
                    let a215 = v0;
                    let a216 = 1u32;
                    fixed_u32::bit_and(a215, a216)
                };
                let a218 = 0u32;
                if equal(a217, a218) {
                    Bytes::lit(&[0x38])
                } else {
                    Bytes::lit(&[0x39])
                }
            } else {
                let a221 = {
                    let a219 = v0;
                    let a220 = 1u32;
                    fixed_u32::bit_and(a219, a220)
                };
                let a222 = 0u32;
                if equal(a221, a222) {
                    Bytes::lit(&[0x61])
                } else {
                    Bytes::lit(&[0x62])
                }
            }
        } else {
            let a225 = {
                let a223 = v0;
                let a224 = 2u32;
                fixed_u32::bit_and(a223, a224)
            };
            let a226 = 0u32;
            if equal(a225, a226) {
                let a229 = {
                    let a227 = v0;
                    let a228 = 1u32;
                    fixed_u32::bit_and(a227, a228)
                };
                let a230 = 0u32;
                if equal(a229, a230) {
                    Bytes::lit(&[0x63])
                } else {
                    Bytes::lit(&[0x64])
                }
            } else {
                let a233 = {
                    let a231 = v0;
                    let a232 = 1u32;
                    fixed_u32::bit_and(a231, a232)
                };
                let a234 = 0u32;
                if equal(a233, a234) {
                    Bytes::lit(&[0x65])
                } else {
                    Bytes::lit(&[0x66])
                }
            }
        }
    }
}

pub fn f23(v0: u32, v1: u32) -> u32 {
    let m237 = {
        let a235 = v0;
        let a236 = v1;
        fixed_u32::shift_right(a235, a236)
    };
    match m237 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f24(v0: Adt0, v1: Adt1) -> R<Adt0> {
    let v2: Adt2 = f26(Fn2::F27, 64u64, Adt2::C0(match v0.clone() {
        Adt0::C0(h238, _, _, _, _, _, _, _) => {
            h238
        }
    }, match v0.clone() {
        Adt0::C0(_, h239, _, _, _, _, _, _) => {
            h239
        }
    }, match v0.clone() {
        Adt0::C0(_, _, h240, _, _, _, _, _) => {
            h240
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, h241, _, _, _, _) => {
            h241
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, h242, _, _, _) => {
            h242
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, h243, _, _) => {
            h243
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, h244, _) => {
            h244
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, h245) => {
            h245
        }
    }, v1, 0u64))?;
    Ok(Adt0::C0(f28(match v0.clone() {
        Adt0::C0(h246, _, _, _, _, _, _, _) => {
            h246
        }
    }, match v2.clone() {
        Adt2::C0(h247, _, _, _, _, _, _, _, _, _) => {
            h247
        }
    }), f28(match v0.clone() {
        Adt0::C0(_, h248, _, _, _, _, _, _) => {
            h248
        }
    }, match v2.clone() {
        Adt2::C0(_, h249, _, _, _, _, _, _, _, _) => {
            h249
        }
    }), f28(match v0.clone() {
        Adt0::C0(_, _, h250, _, _, _, _, _) => {
            h250
        }
    }, match v2.clone() {
        Adt2::C0(_, _, h251, _, _, _, _, _, _, _) => {
            h251
        }
    }), f28(match v0.clone() {
        Adt0::C0(_, _, _, h252, _, _, _, _) => {
            h252
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, h253, _, _, _, _, _, _) => {
            h253
        }
    }), f28(match v0.clone() {
        Adt0::C0(_, _, _, _, h254, _, _, _) => {
            h254
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, _, h255, _, _, _, _, _) => {
            h255
        }
    }), f28(match v0.clone() {
        Adt0::C0(_, _, _, _, _, h256, _, _) => {
            h256
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, _, _, h257, _, _, _, _) => {
            h257
        }
    }), f28(match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, h258, _) => {
            h258
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, _, _, _, h259, _, _, _) => {
            h259
        }
    }), f28(match v0 {
        Adt0::C0(_, _, _, _, _, _, _, h260) => {
            h260
        }
    }, match v2 {
        Adt2::C0(_, _, _, _, _, _, _, h261, _, _) => {
            h261
        }
    })))
}

pub fn f25(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt1> {
    Ok(Adt1::C0(f29(v0.clone(), v1, v2, v3)?, f29(v0.clone(), v1, v2, {
        let a262 = v3;
        let a263 = 4u64;
        nat_add(a262, a263)?
    })?, f29(v0.clone(), v1, v2, {
        let a264 = v3;
        let a265 = 8u64;
        nat_add(a264, a265)?
    })?, f29(v0.clone(), v1, v2, {
        let a266 = v3;
        let a267 = 12u64;
        nat_add(a266, a267)?
    })?, f29(v0.clone(), v1, v2, {
        let a268 = v3;
        let a269 = 16u64;
        nat_add(a268, a269)?
    })?, f29(v0.clone(), v1, v2, {
        let a270 = v3;
        let a271 = 20u64;
        nat_add(a270, a271)?
    })?, f29(v0.clone(), v1, v2, {
        let a272 = v3;
        let a273 = 24u64;
        nat_add(a272, a273)?
    })?, f29(v0.clone(), v1, v2, {
        let a274 = v3;
        let a275 = 28u64;
        nat_add(a274, a275)?
    })?, f29(v0.clone(), v1, v2, {
        let a276 = v3;
        let a277 = 32u64;
        nat_add(a276, a277)?
    })?, f29(v0.clone(), v1, v2, {
        let a278 = v3;
        let a279 = 36u64;
        nat_add(a278, a279)?
    })?, f29(v0.clone(), v1, v2, {
        let a280 = v3;
        let a281 = 40u64;
        nat_add(a280, a281)?
    })?, f29(v0.clone(), v1, v2, {
        let a282 = v3;
        let a283 = 44u64;
        nat_add(a282, a283)?
    })?, f29(v0.clone(), v1, v2, {
        let a284 = v3;
        let a285 = 48u64;
        nat_add(a284, a285)?
    })?, f29(v0.clone(), v1, v2, {
        let a286 = v3;
        let a287 = 52u64;
        nat_add(a286, a287)?
    })?, f29(v0.clone(), v1, v2, {
        let a288 = v3;
        let a289 = 56u64;
        nat_add(a288, a289)?
    })?, f29(v0, v1, v2, {
        let a290 = v3;
        let a291 = 60u64;
        nat_add(a290, a291)?
    })?))
}

pub fn f26(v0: Fn2, p295: u64, p296: Adt2) -> R<Adt2> {
    let m294 = (p295, p296);
    {
        let mut m294 = m294;
        loop {
            let step = {
                let (v1, v2) = m294;
                let m292 = v1;
                if m292 == 0 {
                    Err::<(u64, Adt2), R<Adt2>>(Ok(v2))
                } else {
                    let v3 = m292 - 1;
                    Ok::<(u64, Adt2), R<Adt2>>((v3, {
                        let c293: Fn2 = v0.clone();
                        c293.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m294 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f27(v0: Adt2) -> R<Adt2> {
    f30(v0)
}

pub fn f28(v0: u32, v1: u32) -> u32 {
    let m299 = {
        let a297 = f31(v0);
        let a298 = f31(v1);
        fixed_u64::checked_add(a297, a298)
    };
    match m299 {
        None => {
            0u32
        }
        Some(v2) => {
            f32(v2)
        }
    }
}

pub fn f29(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u32> {
    let a302 = {
        let a300 = v3;
        let a301 = 8u64;
        nat_add(a300, a301)?
    };
    let a303 = v2;
    if nat_eq(a302, a303) {
        Ok(f33({
            let a306 = {
                let a304 = v1;
                let a305 = 8u64;
                nat_mul(a304, a305)?
            };
            let a307 = 4294967296u64;
            let a308 = 0u64;
            nat_quot(a306, a307, a308)
        }))
    } else {
        let a311 = {
            let a309 = v3;
            let a310 = 4u64;
            nat_add(a309, a310)?
        };
        let a312 = v2;
        if nat_eq(a311, a312) {
            Ok(f33({
                let a315 = {
                    let a313 = v1;
                    let a314 = 8u64;
                    nat_mul(a313, a314)?
                };
                let a316 = 4294967296u64;
                let a317 = 0u64;
                nat_rem(a315, a316, a317)
            }))
        } else {
            Ok(f34(f35(v0.clone(), v1, v3), f35(v0.clone(), v1, {
                let a318 = v3;
                let a319 = 1u64;
                nat_add(a318, a319)?
            }), f35(v0.clone(), v1, {
                let a320 = v3;
                let a321 = 2u64;
                nat_add(a320, a321)?
            }), f35(v0, v1, {
                let a322 = v3;
                let a323 = 3u64;
                nat_add(a322, a323)?
            })))
        }
    }
}

pub fn f30(v0: Adt2) -> R<Adt2> {
    let v1: u32 = f36(match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h324) => {
            h324
        }
    });
    let v2: u32 = f28(match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h325, _, _) => {
            h325
        }
    }, f28(f37(match v0.clone() {
        Adt2::C0(_, _, _, _, h326, _, _, _, _, _) => {
            h326
        }
    }), f28(f38(match v0.clone() {
        Adt2::C0(_, _, _, _, h327, _, _, _, _, _) => {
            h327
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h328, _, _, _, _) => {
            h328
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h329, _, _, _) => {
            h329
        }
    }), f28(v1, {
        let m332 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, h331, _) => {
                h331
            }
        };
        match m332 {
            Adt1::C0(h330, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h330
            }
        }
    }))));
    let v3: u32 = f28(f39(match v0.clone() {
        Adt2::C0(h333, _, _, _, _, _, _, _, _, _) => {
            h333
        }
    }), f40(match v0.clone() {
        Adt2::C0(h334, _, _, _, _, _, _, _, _, _) => {
            h334
        }
    }, match v0.clone() {
        Adt2::C0(_, h335, _, _, _, _, _, _, _, _) => {
            h335
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h336, _, _, _, _, _, _, _) => {
            h336
        }
    }));
    Ok(Adt2::C0(f28(v2, v3), match v0.clone() {
        Adt2::C0(h337, _, _, _, _, _, _, _, _, _) => {
            h337
        }
    }, match v0.clone() {
        Adt2::C0(_, h338, _, _, _, _, _, _, _, _) => {
            h338
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h339, _, _, _, _, _, _, _) => {
            h339
        }
    }, f28(match v0.clone() {
        Adt2::C0(_, _, _, h340, _, _, _, _, _, _) => {
            h340
        }
    }, v2), match v0.clone() {
        Adt2::C0(_, _, _, _, h341, _, _, _, _, _) => {
            h341
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h342, _, _, _, _) => {
            h342
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h343, _, _, _) => {
            h343
        }
    }, f41(match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, h344, _) => {
            h344
        }
    }), {
        let a346 = match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h345) => {
                h345
            }
        };
        let a347 = 1u64;
        nat_add(a346, a347)?
    }))
}

pub fn f31(v0: u32) -> u64 {
    let m349 = {
        let a348 = v0;
        fixed_u64::convert(i128::from(a348))
    };
    match m349 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f32(v0: u64) -> u32 {
    let m353 = {
        let a352 = {
            let a350 = v0;
            let a351 = 4294967295u64;
            fixed_u64::bit_and(a350, a351)
        };
        fixed_u32::convert(i128::from(a352))
    };
    match m353 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f33(v0: u64) -> u32 {
    let (_, h354) = {
        let (_, h355) = f42(Fn3::F43, 32u64, (v0, (1u32, 0u32)));
        h355
    };
    h354
}

pub fn f34(v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
    let a360 = f44(v0, 24u32);
    let a361 = {
        let a358 = f44(v1, 16u32);
        let a359 = {
            let a356 = f44(v2, 8u32);
            let a357 = v3;
            fixed_u32::bit_or(a356, a357)
        };
        fixed_u32::bit_or(a358, a359)
    };
    fixed_u32::bit_or(a360, a361)
}

pub fn f35(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a362 = v2;
    let a363 = v1;
    if nat_lt(a362, a363) {
        let m366 = {
            let a364 = v0;
            let a365 = v2;
            index_bytes(a364, a365)
        };
        match m366 {
            None => {
                0u32
            }
            Some(v3) => {
                let m368 = {
                    let a367 = v3;
                    fixed_u32::convert(i128::from(a367))
                };
                match m368 {
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
        let a369 = v2;
        let a370 = v1;
        if nat_eq(a369, a370) {
            128u32
        } else {
            0u32
        }
    }
}

pub fn f36(v0: u64) -> u32 {
    let a371 = v0;
    let a372 = 32u64;
    if nat_lt(a371, a372) {
        let a373 = v0;
        let a374 = 16u64;
        if nat_lt(a373, a374) {
            let a375 = v0;
            let a376 = 8u64;
            if nat_lt(a375, a376) {
                let a377 = v0;
                let a378 = 4u64;
                if nat_lt(a377, a378) {
                    let a379 = v0;
                    let a380 = 2u64;
                    if nat_lt(a379, a380) {
                        let a381 = v0;
                        let a382 = 1u64;
                        if nat_lt(a381, a382) {
                            1116352408u32
                        } else {
                            1899447441u32
                        }
                    } else {
                        let a383 = v0;
                        let a384 = 3u64;
                        if nat_lt(a383, a384) {
                            3049323471u32
                        } else {
                            3921009573u32
                        }
                    }
                } else {
                    let a385 = v0;
                    let a386 = 6u64;
                    if nat_lt(a385, a386) {
                        let a387 = v0;
                        let a388 = 5u64;
                        if nat_lt(a387, a388) {
                            961987163u32
                        } else {
                            1508970993u32
                        }
                    } else {
                        let a389 = v0;
                        let a390 = 7u64;
                        if nat_lt(a389, a390) {
                            2453635748u32
                        } else {
                            2870763221u32
                        }
                    }
                }
            } else {
                let a391 = v0;
                let a392 = 12u64;
                if nat_lt(a391, a392) {
                    let a393 = v0;
                    let a394 = 10u64;
                    if nat_lt(a393, a394) {
                        let a395 = v0;
                        let a396 = 9u64;
                        if nat_lt(a395, a396) {
                            3624381080u32
                        } else {
                            310598401u32
                        }
                    } else {
                        let a397 = v0;
                        let a398 = 11u64;
                        if nat_lt(a397, a398) {
                            607225278u32
                        } else {
                            1426881987u32
                        }
                    }
                } else {
                    let a399 = v0;
                    let a400 = 14u64;
                    if nat_lt(a399, a400) {
                        let a401 = v0;
                        let a402 = 13u64;
                        if nat_lt(a401, a402) {
                            1925078388u32
                        } else {
                            2162078206u32
                        }
                    } else {
                        let a403 = v0;
                        let a404 = 15u64;
                        if nat_lt(a403, a404) {
                            2614888103u32
                        } else {
                            3248222580u32
                        }
                    }
                }
            }
        } else {
            let a405 = v0;
            let a406 = 24u64;
            if nat_lt(a405, a406) {
                let a407 = v0;
                let a408 = 20u64;
                if nat_lt(a407, a408) {
                    let a409 = v0;
                    let a410 = 18u64;
                    if nat_lt(a409, a410) {
                        let a411 = v0;
                        let a412 = 17u64;
                        if nat_lt(a411, a412) {
                            3835390401u32
                        } else {
                            4022224774u32
                        }
                    } else {
                        let a413 = v0;
                        let a414 = 19u64;
                        if nat_lt(a413, a414) {
                            264347078u32
                        } else {
                            604807628u32
                        }
                    }
                } else {
                    let a415 = v0;
                    let a416 = 22u64;
                    if nat_lt(a415, a416) {
                        let a417 = v0;
                        let a418 = 21u64;
                        if nat_lt(a417, a418) {
                            770255983u32
                        } else {
                            1249150122u32
                        }
                    } else {
                        let a419 = v0;
                        let a420 = 23u64;
                        if nat_lt(a419, a420) {
                            1555081692u32
                        } else {
                            1996064986u32
                        }
                    }
                }
            } else {
                let a421 = v0;
                let a422 = 28u64;
                if nat_lt(a421, a422) {
                    let a423 = v0;
                    let a424 = 26u64;
                    if nat_lt(a423, a424) {
                        let a425 = v0;
                        let a426 = 25u64;
                        if nat_lt(a425, a426) {
                            2554220882u32
                        } else {
                            2821834349u32
                        }
                    } else {
                        let a427 = v0;
                        let a428 = 27u64;
                        if nat_lt(a427, a428) {
                            2952996808u32
                        } else {
                            3210313671u32
                        }
                    }
                } else {
                    let a429 = v0;
                    let a430 = 30u64;
                    if nat_lt(a429, a430) {
                        let a431 = v0;
                        let a432 = 29u64;
                        if nat_lt(a431, a432) {
                            3336571891u32
                        } else {
                            3584528711u32
                        }
                    } else {
                        let a433 = v0;
                        let a434 = 31u64;
                        if nat_lt(a433, a434) {
                            113926993u32
                        } else {
                            338241895u32
                        }
                    }
                }
            }
        }
    } else {
        let a435 = v0;
        let a436 = 48u64;
        if nat_lt(a435, a436) {
            let a437 = v0;
            let a438 = 40u64;
            if nat_lt(a437, a438) {
                let a439 = v0;
                let a440 = 36u64;
                if nat_lt(a439, a440) {
                    let a441 = v0;
                    let a442 = 34u64;
                    if nat_lt(a441, a442) {
                        let a443 = v0;
                        let a444 = 33u64;
                        if nat_lt(a443, a444) {
                            666307205u32
                        } else {
                            773529912u32
                        }
                    } else {
                        let a445 = v0;
                        let a446 = 35u64;
                        if nat_lt(a445, a446) {
                            1294757372u32
                        } else {
                            1396182291u32
                        }
                    }
                } else {
                    let a447 = v0;
                    let a448 = 38u64;
                    if nat_lt(a447, a448) {
                        let a449 = v0;
                        let a450 = 37u64;
                        if nat_lt(a449, a450) {
                            1695183700u32
                        } else {
                            1986661051u32
                        }
                    } else {
                        let a451 = v0;
                        let a452 = 39u64;
                        if nat_lt(a451, a452) {
                            2177026350u32
                        } else {
                            2456956037u32
                        }
                    }
                }
            } else {
                let a453 = v0;
                let a454 = 44u64;
                if nat_lt(a453, a454) {
                    let a455 = v0;
                    let a456 = 42u64;
                    if nat_lt(a455, a456) {
                        let a457 = v0;
                        let a458 = 41u64;
                        if nat_lt(a457, a458) {
                            2730485921u32
                        } else {
                            2820302411u32
                        }
                    } else {
                        let a459 = v0;
                        let a460 = 43u64;
                        if nat_lt(a459, a460) {
                            3259730800u32
                        } else {
                            3345764771u32
                        }
                    }
                } else {
                    let a461 = v0;
                    let a462 = 46u64;
                    if nat_lt(a461, a462) {
                        let a463 = v0;
                        let a464 = 45u64;
                        if nat_lt(a463, a464) {
                            3516065817u32
                        } else {
                            3600352804u32
                        }
                    } else {
                        let a465 = v0;
                        let a466 = 47u64;
                        if nat_lt(a465, a466) {
                            4094571909u32
                        } else {
                            275423344u32
                        }
                    }
                }
            }
        } else {
            let a467 = v0;
            let a468 = 56u64;
            if nat_lt(a467, a468) {
                let a469 = v0;
                let a470 = 52u64;
                if nat_lt(a469, a470) {
                    let a471 = v0;
                    let a472 = 50u64;
                    if nat_lt(a471, a472) {
                        let a473 = v0;
                        let a474 = 49u64;
                        if nat_lt(a473, a474) {
                            430227734u32
                        } else {
                            506948616u32
                        }
                    } else {
                        let a475 = v0;
                        let a476 = 51u64;
                        if nat_lt(a475, a476) {
                            659060556u32
                        } else {
                            883997877u32
                        }
                    }
                } else {
                    let a477 = v0;
                    let a478 = 54u64;
                    if nat_lt(a477, a478) {
                        let a479 = v0;
                        let a480 = 53u64;
                        if nat_lt(a479, a480) {
                            958139571u32
                        } else {
                            1322822218u32
                        }
                    } else {
                        let a481 = v0;
                        let a482 = 55u64;
                        if nat_lt(a481, a482) {
                            1537002063u32
                        } else {
                            1747873779u32
                        }
                    }
                }
            } else {
                let a483 = v0;
                let a484 = 60u64;
                if nat_lt(a483, a484) {
                    let a485 = v0;
                    let a486 = 58u64;
                    if nat_lt(a485, a486) {
                        let a487 = v0;
                        let a488 = 57u64;
                        if nat_lt(a487, a488) {
                            1955562222u32
                        } else {
                            2024104815u32
                        }
                    } else {
                        let a489 = v0;
                        let a490 = 59u64;
                        if nat_lt(a489, a490) {
                            2227730452u32
                        } else {
                            2361852424u32
                        }
                    }
                } else {
                    let a491 = v0;
                    let a492 = 62u64;
                    if nat_lt(a491, a492) {
                        let a493 = v0;
                        let a494 = 61u64;
                        if nat_lt(a493, a494) {
                            2428436474u32
                        } else {
                            2756734187u32
                        }
                    } else {
                        let a495 = v0;
                        let a496 = 63u64;
                        if nat_lt(a495, a496) {
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

pub fn f37(v0: u32) -> u32 {
    let a499 = {
        let a497 = f45(v0, 6u32, 26u32);
        let a498 = f45(v0, 11u32, 21u32);
        fixed_u32::bit_xor(a497, a498)
    };
    let a500 = f45(v0, 25u32, 7u32);
    fixed_u32::bit_xor(a499, a500)
}

pub fn f38(v0: u32, v1: u32, v2: u32) -> u32 {
    let a506 = {
        let a501 = v0;
        let a502 = v1;
        fixed_u32::bit_and(a501, a502)
    };
    let a507 = {
        let a504 = {
            let a503 = v0;
            fixed_u32::bit_not(a503)
        };
        let a505 = v2;
        fixed_u32::bit_and(a504, a505)
    };
    fixed_u32::bit_xor(a506, a507)
}

pub fn f39(v0: u32) -> u32 {
    let a510 = {
        let a508 = f45(v0, 2u32, 30u32);
        let a509 = f45(v0, 13u32, 19u32);
        fixed_u32::bit_xor(a508, a509)
    };
    let a511 = f45(v0, 22u32, 10u32);
    fixed_u32::bit_xor(a510, a511)
}

pub fn f40(v0: u32, v1: u32, v2: u32) -> u32 {
    let a520 = {
        let a516 = {
            let a512 = v0;
            let a513 = v1;
            fixed_u32::bit_and(a512, a513)
        };
        let a517 = {
            let a514 = v0;
            let a515 = v2;
            fixed_u32::bit_and(a514, a515)
        };
        fixed_u32::bit_xor(a516, a517)
    };
    let a521 = {
        let a518 = v1;
        let a519 = v2;
        fixed_u32::bit_and(a518, a519)
    };
    fixed_u32::bit_xor(a520, a521)
}

pub fn f41(v0: Adt1) -> Adt1 {
    Adt1::C0(match v0.clone() {
        Adt1::C0(_, h522, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h522
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h523, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h523
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h524, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h524
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h525, _, _, _, _, _, _, _, _, _, _, _) => {
            h525
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, h526, _, _, _, _, _, _, _, _, _, _) => {
            h526
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h527, _, _, _, _, _, _, _, _, _) => {
            h527
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h528, _, _, _, _, _, _, _, _) => {
            h528
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, h529, _, _, _, _, _, _, _) => {
            h529
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, h530, _, _, _, _, _, _) => {
            h530
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, h531, _, _, _, _, _) => {
            h531
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, h532, _, _, _, _) => {
            h532
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, h533, _, _, _) => {
            h533
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h534, _, _) => {
            h534
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h535, _) => {
            h535
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h536) => {
            h536
        }
    }, f28(f46(match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h537, _) => {
            h537
        }
    }), f28(match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, h538, _, _, _, _, _, _) => {
            h538
        }
    }, f28(f47(match v0.clone() {
        Adt1::C0(_, h539, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h539
        }
    }), match v0 {
        Adt1::C0(h540, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h540
        }
    }))))
}

pub fn f42(v0: Fn3, p544: u64, p545: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m543 = (p544, p545);
    {
        let mut m543 = m543;
        loop {
            let step = {
                let (v1, v2) = m543;
                let m541 = v1;
                if m541 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m541 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c542: Fn3 = v0.clone();
                        c542.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m543 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f43(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h546, _) = v0;
        h546
    };
    let v2: u32 = {
        let (h547, _) = {
            let (_, h548) = v0;
            h548
        };
        h547
    };
    let v3: u32 = {
        let (_, h549) = {
            let (_, h550) = v0;
            h550
        };
        h549
    };
    ({
        let a551 = v1;
        let a552 = 2u64;
        let a553 = 0u64;
        nat_quot(a551, a552, a553)
    }, (f44(v2, 1u32), {
        let a557 = {
            let a554 = v1;
            let a555 = 2u64;
            let a556 = 0u64;
            nat_rem(a554, a555, a556)
        };
        let a558 = 1u64;
        if nat_eq(a557, a558) {
            let a559 = v3;
            let a560 = v2;
            fixed_u32::bit_or(a559, a560)
        } else {
            v3
        }
    }))
}

pub fn f44(v0: u32, v1: u32) -> u32 {
    let m563 = {
        let a561 = v0;
        let a562 = v1;
        fixed_u32::shift_left(a561, a562)
    };
    match m563 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f45(v0: u32, v1: u32, v2: u32) -> u32 {
    let a564 = f23(v0, v1);
    let a565 = f44(v0, v2);
    fixed_u32::bit_or(a564, a565)
}

pub fn f46(v0: u32) -> u32 {
    let a568 = {
        let a566 = f45(v0, 17u32, 15u32);
        let a567 = f45(v0, 19u32, 13u32);
        fixed_u32::bit_xor(a566, a567)
    };
    let a569 = f23(v0, 10u32);
    fixed_u32::bit_xor(a568, a569)
}

pub fn f47(v0: u32) -> u32 {
    let a572 = {
        let a570 = f45(v0, 7u32, 25u32);
        let a571 = f45(v0, 18u32, 14u32);
        fixed_u32::bit_xor(a570, a571)
    };
    let a573 = f23(v0, 3u32);
    fixed_u32::bit_xor(a572, a573)
}
