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
    F16(u64, Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn0::F16(k0, k1) => f16(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F18(Bytes),
}

impl Fn1 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn1::F18(k0) => f18(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F38(Bytes, u64, u64),
}

impl Fn2 {
    pub fn apply(&self, p0: (u64, Adt2)) -> R<(u64, Adt2)> {
        match self {
            Fn2::F38(k0, k1, k2) => f38(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F43(Bytes, u64, u64),
}

impl Fn3 {
    pub fn apply(&self, p0: (u64, Adt0)) -> R<(u64, Adt0)> {
        match self {
            Fn3::F43(k0, k1, k2) => f43(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F49,
}

impl Fn4 {
    pub fn apply(&self, p0: (List<Adt1>, Adt1)) -> (List<Adt1>, Adt1) {
        match self {
            Fn4::F49 => f49(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F57(Bytes, u64, u64, u64),
    F115,
}

impl Fn5 {
    pub fn apply(&self, p0: (u64, Adt3)) -> R<(u64, Adt3)> {
        match self {
            Fn5::F57(k0, k1, k2, k3) => f57(k0.clone(), *k1, *k2, *k3, p0),
            Fn5::F115 => f115(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F64(Bytes, u64),
}

impl Fn6 {
    pub fn apply(&self, p0: (u64, List<Adt1>)) -> R<(u64, List<Adt1>)> {
        match self {
            Fn6::F64(k0, k1) => f64(k0.clone(), *k1, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F73,
}

impl Fn7 {
    pub fn apply(&self, p0: Adt5) -> R<Adt5> {
        match self {
            Fn7::F73 => f73(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F78,
}

impl Fn8 {
    pub fn apply(&self, p0: Adt7) -> R<Adt7> {
        match self {
            Fn8::F78 => f78(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F83,
}

impl Fn9 {
    pub fn apply(&self, p0: (Adt9, Adt8)) -> (Adt9, Adt8) {
        match self {
            Fn9::F83 => f83(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F85(u64, Bytes, u64),
}

impl Fn10 {
    pub fn apply(&self, p0: (u64, Adt1)) -> R<(u64, Adt1)> {
        match self {
            Fn10::F85(k0, k1, k2) => f85(*k0, k1.clone(), *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F89,
}

impl Fn11 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn11::F89 => f89(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F104,
}

impl Fn12 {
    pub fn apply(&self, p0: Adt10) -> Adt10 {
        match self {
            Fn12::F104 => f104(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F125,
}

impl Fn13 {
    pub fn apply(&self, p0: (u64, (u64, u64))) -> (u64, (u64, u64)) {
        match self {
            Fn13::F125 => f125(p0),
        }
    }
}

pub fn f0(v0: Bytes, v1: Bytes) -> R<Option<Bytes>> {
    if f1(v0.clone(), v1.clone())? {
        let v2: u64 = {
            let a1 = f2(v0.clone());
            length_bytes(a1)
        };
        f3(v0.clone(), {
            let a2 = f2(v0.clone());
            let a3 = f4(f5(v1.clone(), v2, f6(v0.clone())), f7(v1, v2, f6(v0))?);
            append_bytes(a2, a3)
        })
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f1(v0: Bytes, v1: Bytes) -> R<bool> {
    let a4 = 0u64;
    let a5 = f6(v0.clone());
    if nat_lt(a4, a5) {
        let a10 = {
            let a6 = v1.clone();
            length_bytes(a6)
        };
        let a11 = {
            let a8 = {
                let a7 = f2(v0.clone());
                length_bytes(a7)
            };
            let a9 = f6(v0.clone());
            nat_add(a8, a9)?
        };
        if nat_eq(a10, a11) {
            if f8(v1.clone(), f2(v0.clone())) {
                let a14 = f9(v1.clone(), {
                    let a12 = f2(v0);
                    length_bytes(a12)
                })?;
                let a15 = {
                    let a13 = v1;
                    length_bytes(a13)
                };
                Ok(nat_eq(a14, a15))
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

pub fn f2(v0: Bytes) -> Bytes {
    let a16 = v0;
    let a17 = f10(Str::lit(":"));
    append_bytes(a16, a17)
}

pub fn f3(v0: Bytes, v1: Bytes) -> R<Option<Bytes>> {
    let m18 = f11(v0.clone(), v1)?;
    match m18 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v2) => {
            Ok(Some({
                let a19 = f2(v0);
                let a20 = v2;
                append_bytes(a19, a20)
            }))
        }
    }
}

pub fn f4(v0: Bytes, v1: Bytes) -> Bytes {
    if f12(v1.clone(), v0.clone()) {
        v1
    } else {
        v0
    }
}

pub fn f5(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    f13(v0, v1, v2)
}

pub fn f6(v0: Bytes) -> u64 {
    if f14(v0.clone(), Str::lit("sha256")) {
        64u64
    } else {
        if f14(v0.clone(), Str::lit("sha3-256")) {
            64u64
        } else {
            if f14(v0.clone(), Str::lit("keccak256")) {
                64u64
            } else {
                if f14(v0.clone(), Str::lit("sha512")) {
                    128u64
                } else {
                    if f14(v0, Str::lit("blake3")) {
                        64u64
                    } else {
                        0u64
                    }
                }
            }
        }
    }
}

pub fn f7(v0: Bytes, v1: u64, v2: u64) -> R<Bytes> {
    let v3: Bytes = f5(v0, v1, v2);
    let (_, h21) = {
        let (h22, _) = f15(Fn0::F16(v2, v3), {
            let a23 = v2;
            let a24 = 1u64;
            nat_add(a23, a24)?
        }, (0u64, Bytes::lit(&[])))?;
        h22
    };
    Ok(h21)
}

pub fn f8(v0: Bytes, v1: Bytes) -> bool {
    let m29 = {
        let a26 = v0;
        let a27 = 0u64;
        let a28 = {
            let a25 = v1.clone();
            length_bytes(a25)
        };
        slice_bytes(a26, a27, a28)
    };
    match m29 {
        None => {
            false
        }
        Some(v2) => {
            let a30 = v2;
            let a31 = v1;
            equal(a30, a31)
        }
    }
}

pub fn f9(v0: Bytes, v1: u64) -> R<u64> {
    let (h32, _) = f17(Fn1::F18(v0.clone()), {
        let a33 = v0;
        length_bytes(a33)
    }, v1)?;
    Ok(h32)
}

pub fn f10(v0: Str) -> Bytes {
    let a34 = v0;
    utf8_encode(a34)
}

pub fn f11(v0: Bytes, v1: Bytes) -> R<Option<Bytes>> {
    if f14(v0.clone(), Str::lit("sha256")) {
        Ok(Some(f19(v1)?))
    } else {
        if f14(v0.clone(), Str::lit("sha3-256")) {
            Ok(Some(f20(v1)?))
        } else {
            if f14(v0.clone(), Str::lit("keccak256")) {
                Ok(Some(f21(v1)?))
            } else {
                if f14(v0.clone(), Str::lit("sha512")) {
                    Ok(Some(f22(v1)?))
                } else {
                    if f14(v0, Str::lit("blake3")) {
                        Ok(Some(f23(v1)?))
                    } else {
                        Ok(None::<Bytes>)
                    }
                }
            }
        }
    }
}

pub fn f12(v0: Bytes, v1: Bytes) -> bool {
    let a39 = {
        let a35 = v0;
        let a36 = v1;
        compare_bytes(a35, a36)
    };
    let a40 = {
        let a37 = Bytes::lit(&[]);
        let a38 = Bytes::lit(&[0x01]);
        compare_bytes(a37, a38)
    };
    equal(a39, a40)
}

pub fn f13(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m44 = {
        let a41 = v0;
        let a42 = v1;
        let a43 = v2;
        slice_bytes(a41, a42, a43)
    };
    match m44 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f14(v0: Bytes, v1: Str) -> bool {
    let a45 = v0;
    let a46 = f10(v1);
    equal(a45, a46)
}

pub fn f15(v0: Fn0, p51: u64, p52: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m50 = (p51, p52);
    {
        let mut m50 = m50;
        loop {
            let step = {
                let (v1, v2) = m50;
                let m47 = v1;
                if m47 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m47 - 1;
                    let m49 = {
                        let c48: Fn0 = v0.clone();
                        c48.apply(v2.clone())?
                    };
                    match m49 {
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
                Ok(next) => { m50 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f16(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f24(v0, v1, v2)
}

pub fn f17(v0: Fn1, p57: u64, p58: u64) -> R<(u64, bool)> {
    let m56 = (p57, p58);
    {
        let mut m56 = m56;
        loop {
            let step = {
                let (v1, v2) = m56;
                let m53 = v1;
                if m53 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m53 - 1;
                    let m55 = {
                        let c54: Fn1 = v0.clone();
                        c54.apply(v2)?
                    };
                    match m55 {
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
                Ok(next) => { m56 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f18(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f25(v0, v1)? {
        Ok(Some({
            let a59 = v1;
            let a60 = 1u64;
            nat_add(a59, a60)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f19(v0: Bytes) -> R<Bytes> {
    Ok(f26(f27(v0)?))
}

pub fn f20(v0: Bytes) -> R<Bytes> {
    f28(v0, 6u64)
}

pub fn f21(v0: Bytes) -> R<Bytes> {
    f28(v0, 1u64)
}

pub fn f22(v0: Bytes) -> R<Bytes> {
    let v1: Adt0 = f29(v0)?;
    let a81 = f30(match v1.clone() {
        Adt0::C0(h61, _, _, _, _, _, _, _) => {
            h61
        }
    });
    let a82 = {
        let a79 = f30(match v1.clone() {
            Adt0::C0(_, h62, _, _, _, _, _, _) => {
                h62
            }
        });
        let a80 = {
            let a77 = f30(match v1.clone() {
                Adt0::C0(_, _, h63, _, _, _, _, _) => {
                    h63
                }
            });
            let a78 = {
                let a75 = f30(match v1.clone() {
                    Adt0::C0(_, _, _, h64, _, _, _, _) => {
                        h64
                    }
                });
                let a76 = {
                    let a73 = f30(match v1.clone() {
                        Adt0::C0(_, _, _, _, h65, _, _, _) => {
                            h65
                        }
                    });
                    let a74 = {
                        let a71 = f30(match v1.clone() {
                            Adt0::C0(_, _, _, _, _, h66, _, _) => {
                                h66
                            }
                        });
                        let a72 = {
                            let a69 = f30(match v1.clone() {
                                Adt0::C0(_, _, _, _, _, _, h67, _) => {
                                    h67
                                }
                            });
                            let a70 = f30(match v1 {
                                Adt0::C0(_, _, _, _, _, _, _, h68) => {
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

pub fn f23(v0: Bytes) -> R<Bytes> {
    let v1: Adt1 = f31(v0)?;
    let a103 = f32(match v1.clone() {
        Adt1::C0(h83, _, _, _, _, _, _, _) => {
            h83
        }
    });
    let a104 = {
        let a101 = f32(match v1.clone() {
            Adt1::C0(_, h84, _, _, _, _, _, _) => {
                h84
            }
        });
        let a102 = {
            let a99 = f32(match v1.clone() {
                Adt1::C0(_, _, h85, _, _, _, _, _) => {
                    h85
                }
            });
            let a100 = {
                let a97 = f32(match v1.clone() {
                    Adt1::C0(_, _, _, h86, _, _, _, _) => {
                        h86
                    }
                });
                let a98 = {
                    let a95 = f32(match v1.clone() {
                        Adt1::C0(_, _, _, _, h87, _, _, _) => {
                            h87
                        }
                    });
                    let a96 = {
                        let a93 = f32(match v1.clone() {
                            Adt1::C0(_, _, _, _, _, h88, _, _) => {
                                h88
                            }
                        });
                        let a94 = {
                            let a91 = f32(match v1.clone() {
                                Adt1::C0(_, _, _, _, _, _, h89, _) => {
                                    h89
                                }
                            });
                            let a92 = f32(match v1 {
                                Adt1::C0(_, _, _, _, _, _, _, h90) => {
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

pub fn f24(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let a106 = {
        let (h105, _) = v2.clone();
        h105
    };
    let a107 = v0;
    if nat_lt(a106, a107) {
        Ok(Some(({
            let a109 = {
                let (h108, _) = v2.clone();
                h108
            };
            let a110 = 1u64;
            nat_add(a109, a110)?
        }, {
            let a115 = {
                let (_, h111) = v2;
                h111
            };
            let a116 = f33({
                let a113 = 15u64;
                let a114 = f34(f35(v1, {
                    let (h112, _) = v2;
                    h112
                })?);
                nat_sub(a113, a114)
            })?;
            append_bytes(a115, a116)
        })))
    } else {
        Ok(None::<(u64, Bytes)>)
    }
}

pub fn f25(v0: Bytes, v1: u64) -> R<bool> {
    let v2: u64 = f35(v0, v1)?;
    let a117 = 47u64;
    let a118 = v2;
    if nat_lt(a117, a118) {
        let a119 = v2;
        let a120 = 58u64;
        if nat_lt(a119, a120) {
            Ok(true)
        } else {
            let a121 = 96u64;
            let a122 = v2;
            if nat_lt(a121, a122) {
                let a123 = v2;
                let a124 = 103u64;
                Ok(nat_lt(a123, a124))
            } else {
                Ok(false)
            }
        }
    } else {
        Ok(false)
    }
}

pub fn f26(v0: Adt2) -> Bytes {
    let a145 = f36(match v0.clone() {
        Adt2::C0(h125, _, _, _, _, _, _, _) => {
            h125
        }
    });
    let a146 = {
        let a143 = f36(match v0.clone() {
            Adt2::C0(_, h126, _, _, _, _, _, _) => {
                h126
            }
        });
        let a144 = {
            let a141 = f36(match v0.clone() {
                Adt2::C0(_, _, h127, _, _, _, _, _) => {
                    h127
                }
            });
            let a142 = {
                let a139 = f36(match v0.clone() {
                    Adt2::C0(_, _, _, h128, _, _, _, _) => {
                        h128
                    }
                });
                let a140 = {
                    let a137 = f36(match v0.clone() {
                        Adt2::C0(_, _, _, _, h129, _, _, _) => {
                            h129
                        }
                    });
                    let a138 = {
                        let a135 = f36(match v0.clone() {
                            Adt2::C0(_, _, _, _, _, h130, _, _) => {
                                h130
                            }
                        });
                        let a136 = {
                            let a133 = f36(match v0.clone() {
                                Adt2::C0(_, _, _, _, _, _, h131, _) => {
                                    h131
                                }
                            });
                            let a134 = f36(match v0 {
                                Adt2::C0(_, _, _, _, _, _, _, h132) => {
                                    h132
                                }
                            });
                            append_bytes(a133, a134)
                        };
                        append_bytes(a135, a136)
                    };
                    append_bytes(a137, a138)
                };
                append_bytes(a139, a140)
            };
            append_bytes(a141, a142)
        };
        append_bytes(a143, a144)
    };
    append_bytes(a145, a146)
}

pub fn f27(v0: Bytes) -> R<Adt2> {
    let v1: u64 = {
        let a147 = v0.clone();
        length_bytes(a147)
    };
    let v2: u64 = {
        let a153 = {
            let a150 = {
                let a148 = v1;
                let a149 = 72u64;
                nat_add(a148, a149)?
            };
            let a151 = 64u64;
            let a152 = 0u64;
            nat_quot(a150, a151, a152)
        };
        let a154 = 64u64;
        nat_mul(a153, a154)?
    };
    let (_, h155) = f37(Fn2::F38(v0, v1, v2), {
        let a156 = v2;
        let a157 = 64u64;
        let a158 = 0u64;
        nat_quot(a156, a157, a158)
    }, (0u64, f39()))?;
    Ok(h155)
}

pub fn f28(v0: Bytes, v1: u64) -> R<Bytes> {
    let v2: Adt3 = f40(v0, v1)?;
    let a167 = f41(match v2.clone() {
        Adt3::C0(h159, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h159
        }
    });
    let a168 = {
        let a165 = f41(match v2.clone() {
            Adt3::C0(_, h160, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h160
            }
        });
        let a166 = {
            let a163 = f41(match v2.clone() {
                Adt3::C0(_, _, h161, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h161
                }
            });
            let a164 = f41(match v2 {
                Adt3::C0(_, _, _, h162, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h162
                }
            });
            append_bytes(a163, a164)
        };
        append_bytes(a165, a166)
    };
    Ok(append_bytes(a167, a168))
}

pub fn f29(v0: Bytes) -> R<Adt0> {
    let v1: u64 = {
        let a169 = v0.clone();
        length_bytes(a169)
    };
    let v2: u64 = {
        let a175 = {
            let a172 = {
                let a170 = v1;
                let a171 = 144u64;
                nat_add(a170, a171)?
            };
            let a173 = 128u64;
            let a174 = 0u64;
            nat_quot(a172, a173, a174)
        };
        let a176 = 128u64;
        nat_mul(a175, a176)?
    };
    let (_, h177) = f42(Fn3::F43(v0, v1, v2), {
        let a178 = v2;
        let a179 = 128u64;
        let a180 = 0u64;
        nat_quot(a178, a179, a180)
    }, (0u64, f44()))?;
    Ok(h177)
}

pub fn f30(v0: u64) -> Bytes {
    let a209 = f45(v0, 60u32);
    let a210 = {
        let a207 = f45(v0, 56u32);
        let a208 = {
            let a205 = f45(v0, 52u32);
            let a206 = {
                let a203 = f45(v0, 48u32);
                let a204 = {
                    let a201 = f45(v0, 44u32);
                    let a202 = {
                        let a199 = f45(v0, 40u32);
                        let a200 = {
                            let a197 = f45(v0, 36u32);
                            let a198 = {
                                let a195 = f45(v0, 32u32);
                                let a196 = {
                                    let a193 = f45(v0, 28u32);
                                    let a194 = {
                                        let a191 = f45(v0, 24u32);
                                        let a192 = {
                                            let a189 = f45(v0, 20u32);
                                            let a190 = {
                                                let a187 = f45(v0, 16u32);
                                                let a188 = {
                                                    let a185 = f45(v0, 12u32);
                                                    let a186 = {
                                                        let a183 = f45(v0, 8u32);
                                                        let a184 = {
                                                            let a181 = f45(v0, 4u32);
                                                            let a182 = f45(v0, 0u32);
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
                                    };
                                    append_bytes(a193, a194)
                                };
                                append_bytes(a195, a196)
                            };
                            append_bytes(a197, a198)
                        };
                        append_bytes(a199, a200)
                    };
                    append_bytes(a201, a202)
                };
                append_bytes(a203, a204)
            };
            append_bytes(a205, a206)
        };
        append_bytes(a207, a208)
    };
    append_bytes(a209, a210)
}

pub fn f31(v0: Bytes) -> R<Adt1> {
    let v1: u64 = {
        let a211 = v0.clone();
        length_bytes(a211)
    };
    let v2: u64 = {
        let a212 = v1;
        let a213 = 0u64;
        if nat_eq(a212, a213) {
            1u64
        } else {
            let a216 = {
                let a214 = v1;
                let a215 = 1023u64;
                nat_add(a214, a215)?
            };
            let a217 = 1024u64;
            let a218 = 0u64;
            nat_quot(a216, a217, a218)
        }
    };
    let v3: u64 = {
        let a219 = v2;
        let a220 = 1u64;
        nat_sub(a219, a220)
    };
    let m222 = {
        let (_, h221) = f46(v0.clone(), v1, v3)?;
        h221
    };
    match m222.uncons() {
        None => {
            f47(v0, v1, v3, true)
        }
        Some((v4, v5)) => {
            let (_, h223) = f48(Fn4::F49, 64u64, (List::cons(v4, v5), f47(v0, v1, v3, false)?));
            Ok(h223)
        }
    }
}

pub fn f32(v0: u32) -> Bytes {
    let a236 = {
        let a224 = f50(v0, 4u32);
        let a225 = f50(v0, 0u32);
        append_bytes(a224, a225)
    };
    let a237 = {
        let a234 = {
            let a226 = f50(v0, 12u32);
            let a227 = f50(v0, 8u32);
            append_bytes(a226, a227)
        };
        let a235 = {
            let a232 = {
                let a228 = f50(v0, 20u32);
                let a229 = f50(v0, 16u32);
                append_bytes(a228, a229)
            };
            let a233 = {
                let a230 = f50(v0, 28u32);
                let a231 = f50(v0, 24u32);
                append_bytes(a230, a231)
            };
            append_bytes(a232, a233)
        };
        append_bytes(a234, a235)
    };
    append_bytes(a236, a237)
}

pub fn f33(v0: u64) -> R<Bytes> {
    Ok(f51({
        let a238 = v0;
        let a239 = 10u64;
        if nat_lt(a238, a239) {
            let a240 = 48u64;
            let a241 = v0;
            nat_add(a240, a241)?
        } else {
            let a242 = 87u64;
            let a243 = v0;
            nat_add(a242, a243)?
        }
    }))
}

pub fn f34(v0: u64) -> u64 {
    let a244 = 47u64;
    let a245 = v0;
    if nat_lt(a244, a245) {
        let a246 = v0;
        let a247 = 58u64;
        if nat_lt(a246, a247) {
            let a248 = v0;
            let a249 = 48u64;
            nat_sub(a248, a249)
        } else {
            let a250 = 64u64;
            let a251 = v0;
            if nat_lt(a250, a251) {
                let a252 = v0;
                let a253 = 71u64;
                if nat_lt(a252, a253) {
                    let a254 = v0;
                    let a255 = 55u64;
                    nat_sub(a254, a255)
                } else {
                    let a256 = 96u64;
                    let a257 = v0;
                    if nat_lt(a256, a257) {
                        let a258 = v0;
                        let a259 = 103u64;
                        if nat_lt(a258, a259) {
                            let a260 = v0;
                            let a261 = 87u64;
                            nat_sub(a260, a261)
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

pub fn f35(v0: Bytes, v1: u64) -> R<u64> {
    let m264 = {
        let a262 = v0;
        let a263 = v1;
        index_bytes(a262, a263)
    };
    match m264 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a309 = {
                let a285 = {
                    let a273 = {
                        let a267 = {
                            let a265 = v3;
                            let a266 = 128u8;
                            fixed_u8::bit_and(a265, a266)
                        };
                        let a268 = 128u8;
                        if equal(a267, a268) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a274 = {
                        let a271 = {
                            let a269 = v3;
                            let a270 = 64u8;
                            fixed_u8::bit_and(a269, a270)
                        };
                        let a272 = 64u8;
                        if equal(a271, a272) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a273, a274)?
                };
                let a286 = {
                    let a283 = {
                        let a277 = {
                            let a275 = v3;
                            let a276 = 32u8;
                            fixed_u8::bit_and(a275, a276)
                        };
                        let a278 = 32u8;
                        if equal(a277, a278) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a284 = {
                        let a281 = {
                            let a279 = v3;
                            let a280 = 16u8;
                            fixed_u8::bit_and(a279, a280)
                        };
                        let a282 = 16u8;
                        if equal(a281, a282) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a283, a284)?
                };
                nat_add(a285, a286)?
            };
            let a310 = {
                let a307 = {
                    let a295 = {
                        let a289 = {
                            let a287 = v3;
                            let a288 = 8u8;
                            fixed_u8::bit_and(a287, a288)
                        };
                        let a290 = 8u8;
                        if equal(a289, a290) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a296 = {
                        let a293 = {
                            let a291 = v3;
                            let a292 = 4u8;
                            fixed_u8::bit_and(a291, a292)
                        };
                        let a294 = 4u8;
                        if equal(a293, a294) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a295, a296)?
                };
                let a308 = {
                    let a305 = {
                        let a299 = {
                            let a297 = v3;
                            let a298 = 2u8;
                            fixed_u8::bit_and(a297, a298)
                        };
                        let a300 = 2u8;
                        if equal(a299, a300) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a306 = {
                        let a303 = {
                            let a301 = v3;
                            let a302 = 1u8;
                            fixed_u8::bit_and(a301, a302)
                        };
                        let a304 = 1u8;
                        if equal(a303, a304) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a305, a306)?
                };
                nat_add(a307, a308)?
            };
            nat_add(a309, a310)
        }
    }
}

pub fn f36(v0: u32) -> Bytes {
    let a339 = f52({
        let a311 = f53(v0, 28u32);
        let a312 = 15u32;
        fixed_u32::bit_and(a311, a312)
    });
    let a340 = {
        let a337 = f52({
            let a313 = f53(v0, 24u32);
            let a314 = 15u32;
            fixed_u32::bit_and(a313, a314)
        });
        let a338 = {
            let a335 = f52({
                let a315 = f53(v0, 20u32);
                let a316 = 15u32;
                fixed_u32::bit_and(a315, a316)
            });
            let a336 = {
                let a333 = f52({
                    let a317 = f53(v0, 16u32);
                    let a318 = 15u32;
                    fixed_u32::bit_and(a317, a318)
                });
                let a334 = {
                    let a331 = f52({
                        let a319 = f53(v0, 12u32);
                        let a320 = 15u32;
                        fixed_u32::bit_and(a319, a320)
                    });
                    let a332 = {
                        let a329 = f52({
                            let a321 = f53(v0, 8u32);
                            let a322 = 15u32;
                            fixed_u32::bit_and(a321, a322)
                        });
                        let a330 = {
                            let a327 = f52({
                                let a323 = f53(v0, 4u32);
                                let a324 = 15u32;
                                fixed_u32::bit_and(a323, a324)
                            });
                            let a328 = f52({
                                let a325 = v0;
                                let a326 = 15u32;
                                fixed_u32::bit_and(a325, a326)
                            });
                            append_bytes(a327, a328)
                        };
                        append_bytes(a329, a330)
                    };
                    append_bytes(a331, a332)
                };
                append_bytes(a333, a334)
            };
            append_bytes(a335, a336)
        };
        append_bytes(a337, a338)
    };
    append_bytes(a339, a340)
}

pub fn f37(v0: Fn2, p344: u64, p345: (u64, Adt2)) -> R<(u64, Adt2)> {
    let m343 = (p344, p345);
    {
        let mut m343 = m343;
        loop {
            let step = {
                let (v1, v2) = m343;
                let m341 = v1;
                if m341 == 0 {
                    Err::<(u64, (u64, Adt2)), R<(u64, Adt2)>>(Ok(v2))
                } else {
                    let v3 = m341 - 1;
                    Ok::<(u64, (u64, Adt2)), R<(u64, Adt2)>>((v3, {
                        let c342: Fn2 = v0.clone();
                        c342.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m343 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f38(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt2)) -> R<(u64, Adt2)> {
    Ok(({
        let a347 = {
            let (h346, _) = v3.clone();
            h346
        };
        let a348 = 64u64;
        nat_add(a347, a348)?
    }, f54({
        let (_, h349) = v3;
        h349
    }, f55(v0, v1, v2, {
        let (h350, _) = v3;
        h350
    })?)?))
}

pub fn f39() -> Adt2 {
    Adt2::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f40(v0: Bytes, v1: u64) -> R<Adt3> {
    let v2: u64 = {
        let a351 = v0.clone();
        length_bytes(a351)
    };
    let v3: u64 = {
        let a357 = {
            let a355 = {
                let a352 = v2;
                let a353 = 136u64;
                let a354 = 0u64;
                nat_quot(a352, a353, a354)
            };
            let a356 = 1u64;
            nat_add(a355, a356)?
        };
        let a358 = 136u64;
        nat_mul(a357, a358)?
    };
    let (_, h359) = f56(Fn5::F57(v0, v2, v1, v3), {
        let a360 = v3;
        let a361 = 136u64;
        let a362 = 0u64;
        nat_quot(a360, a361, a362)
    }, (0u64, f58()))?;
    Ok(h359)
}

pub fn f41(v0: u64) -> Bytes {
    let a391 = {
        let a363 = f45(v0, 4u32);
        let a364 = f45(v0, 0u32);
        append_bytes(a363, a364)
    };
    let a392 = {
        let a389 = {
            let a365 = f45(v0, 12u32);
            let a366 = f45(v0, 8u32);
            append_bytes(a365, a366)
        };
        let a390 = {
            let a387 = {
                let a367 = f45(v0, 20u32);
                let a368 = f45(v0, 16u32);
                append_bytes(a367, a368)
            };
            let a388 = {
                let a385 = {
                    let a369 = f45(v0, 28u32);
                    let a370 = f45(v0, 24u32);
                    append_bytes(a369, a370)
                };
                let a386 = {
                    let a383 = {
                        let a371 = f45(v0, 36u32);
                        let a372 = f45(v0, 32u32);
                        append_bytes(a371, a372)
                    };
                    let a384 = {
                        let a381 = {
                            let a373 = f45(v0, 44u32);
                            let a374 = f45(v0, 40u32);
                            append_bytes(a373, a374)
                        };
                        let a382 = {
                            let a379 = {
                                let a375 = f45(v0, 52u32);
                                let a376 = f45(v0, 48u32);
                                append_bytes(a375, a376)
                            };
                            let a380 = {
                                let a377 = f45(v0, 60u32);
                                let a378 = f45(v0, 56u32);
                                append_bytes(a377, a378)
                            };
                            append_bytes(a379, a380)
                        };
                        append_bytes(a381, a382)
                    };
                    append_bytes(a383, a384)
                };
                append_bytes(a385, a386)
            };
            append_bytes(a387, a388)
        };
        append_bytes(a389, a390)
    };
    append_bytes(a391, a392)
}

pub fn f42(v0: Fn3, p396: u64, p397: (u64, Adt0)) -> R<(u64, Adt0)> {
    let m395 = (p396, p397);
    {
        let mut m395 = m395;
        loop {
            let step = {
                let (v1, v2) = m395;
                let m393 = v1;
                if m393 == 0 {
                    Err::<(u64, (u64, Adt0)), R<(u64, Adt0)>>(Ok(v2))
                } else {
                    let v3 = m393 - 1;
                    Ok::<(u64, (u64, Adt0)), R<(u64, Adt0)>>((v3, {
                        let c394: Fn3 = v0.clone();
                        c394.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m395 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f43(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt0)) -> R<(u64, Adt0)> {
    Ok(({
        let a399 = {
            let (h398, _) = v3.clone();
            h398
        };
        let a400 = 128u64;
        nat_add(a399, a400)?
    }, f59({
        let (_, h401) = v3;
        h401
    }, f60(v0, v1, v2, {
        let (h402, _) = v3;
        h402
    })?)?))
}

pub fn f44() -> Adt0 {
    Adt0::C0(7640891576956012808u64, 13503953896175478587u64, 4354685564936845355u64, 11912009170470909681u64, 5840696475078001361u64, 11170449401992604703u64, 2270897969802886507u64, 6620516959819538809u64)
}

pub fn f45(v0: u64, v1: u32) -> Bytes {
    f52(f61(f62(v0, v1)))
}

pub fn f46(v0: Bytes, v1: u64, v2: u64) -> R<(u64, List<Adt1>)> {
    f63(Fn6::F64(v0, v1), v2, (0u64, List::<Adt1>::nil()))
}

pub fn f47(v0: Bytes, v1: u64, v2: u64, v3: bool) -> R<Adt1> {
    let v4: u64 = {
        let a403 = 1024u64;
        let a404 = v2;
        nat_mul(a403, a404)?
    };
    let v5: u64 = f65(v1, v4);
    let v6: u64 = f66(v5)?;
    let v7: u64 = {
        let a405 = v6;
        let a406 = 1u64;
        nat_sub(a405, a406)
    };
    Ok(f67(f68(v0.clone(), v4, v2, v7)?, f69(v0, {
        let a409 = v4;
        let a410 = {
            let a407 = 64u64;
            let a408 = v7;
            nat_mul(a407, a408)?
        };
        nat_add(a409, a410)?
    })?, v2, f70({
        let a413 = v5;
        let a414 = {
            let a411 = 64u64;
            let a412 = v7;
            nat_mul(a411, a412)?
        };
        nat_sub(a413, a414)
    }), {
        let a417 = {
            let a415 = v7;
            let a416 = 0u64;
            if nat_eq(a415, a416) {
                3u32
            } else {
                2u32
            }
        };
        let a418 = if v3 {
            8u32
        } else {
            0u32
        };
        fixed_u32::bit_or(a417, a418)
    }))
}

pub fn f48(v0: Fn4, p422: u64, p423: (List<Adt1>, Adt1)) -> (List<Adt1>, Adt1) {
    let m421 = (p422, p423);
    {
        let mut m421 = m421;
        loop {
            let step = {
                let (v1, v2) = m421;
                let m419 = v1;
                if m419 == 0 {
                    Err::<(u64, (List<Adt1>, Adt1)), (List<Adt1>, Adt1)>(v2)
                } else {
                    let v3 = m419 - 1;
                    Ok::<(u64, (List<Adt1>, Adt1)), (List<Adt1>, Adt1)>((v3, {
                        let c420: Fn4 = v0.clone();
                        c420.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m421 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f49(v0: (List<Adt1>, Adt1)) -> (List<Adt1>, Adt1) {
    f71(v0)
}

pub fn f50(v0: u32, v1: u32) -> Bytes {
    f52(f53(v0, v1))
}

pub fn f51(v0: u64) -> Bytes {
    f13(Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff]), v0, 1u64)
}

pub fn f52(v0: u32) -> Bytes {
    let a426 = {
        let a424 = v0;
        let a425 = 8u32;
        fixed_u32::bit_and(a424, a425)
    };
    let a427 = 0u32;
    if equal(a426, a427) {
        let a430 = {
            let a428 = v0;
            let a429 = 4u32;
            fixed_u32::bit_and(a428, a429)
        };
        let a431 = 0u32;
        if equal(a430, a431) {
            let a434 = {
                let a432 = v0;
                let a433 = 2u32;
                fixed_u32::bit_and(a432, a433)
            };
            let a435 = 0u32;
            if equal(a434, a435) {
                let a438 = {
                    let a436 = v0;
                    let a437 = 1u32;
                    fixed_u32::bit_and(a436, a437)
                };
                let a439 = 0u32;
                if equal(a438, a439) {
                    Bytes::lit(&[0x30])
                } else {
                    Bytes::lit(&[0x31])
                }
            } else {
                let a442 = {
                    let a440 = v0;
                    let a441 = 1u32;
                    fixed_u32::bit_and(a440, a441)
                };
                let a443 = 0u32;
                if equal(a442, a443) {
                    Bytes::lit(&[0x32])
                } else {
                    Bytes::lit(&[0x33])
                }
            }
        } else {
            let a446 = {
                let a444 = v0;
                let a445 = 2u32;
                fixed_u32::bit_and(a444, a445)
            };
            let a447 = 0u32;
            if equal(a446, a447) {
                let a450 = {
                    let a448 = v0;
                    let a449 = 1u32;
                    fixed_u32::bit_and(a448, a449)
                };
                let a451 = 0u32;
                if equal(a450, a451) {
                    Bytes::lit(&[0x34])
                } else {
                    Bytes::lit(&[0x35])
                }
            } else {
                let a454 = {
                    let a452 = v0;
                    let a453 = 1u32;
                    fixed_u32::bit_and(a452, a453)
                };
                let a455 = 0u32;
                if equal(a454, a455) {
                    Bytes::lit(&[0x36])
                } else {
                    Bytes::lit(&[0x37])
                }
            }
        }
    } else {
        let a458 = {
            let a456 = v0;
            let a457 = 4u32;
            fixed_u32::bit_and(a456, a457)
        };
        let a459 = 0u32;
        if equal(a458, a459) {
            let a462 = {
                let a460 = v0;
                let a461 = 2u32;
                fixed_u32::bit_and(a460, a461)
            };
            let a463 = 0u32;
            if equal(a462, a463) {
                let a466 = {
                    let a464 = v0;
                    let a465 = 1u32;
                    fixed_u32::bit_and(a464, a465)
                };
                let a467 = 0u32;
                if equal(a466, a467) {
                    Bytes::lit(&[0x38])
                } else {
                    Bytes::lit(&[0x39])
                }
            } else {
                let a470 = {
                    let a468 = v0;
                    let a469 = 1u32;
                    fixed_u32::bit_and(a468, a469)
                };
                let a471 = 0u32;
                if equal(a470, a471) {
                    Bytes::lit(&[0x61])
                } else {
                    Bytes::lit(&[0x62])
                }
            }
        } else {
            let a474 = {
                let a472 = v0;
                let a473 = 2u32;
                fixed_u32::bit_and(a472, a473)
            };
            let a475 = 0u32;
            if equal(a474, a475) {
                let a478 = {
                    let a476 = v0;
                    let a477 = 1u32;
                    fixed_u32::bit_and(a476, a477)
                };
                let a479 = 0u32;
                if equal(a478, a479) {
                    Bytes::lit(&[0x63])
                } else {
                    Bytes::lit(&[0x64])
                }
            } else {
                let a482 = {
                    let a480 = v0;
                    let a481 = 1u32;
                    fixed_u32::bit_and(a480, a481)
                };
                let a483 = 0u32;
                if equal(a482, a483) {
                    Bytes::lit(&[0x65])
                } else {
                    Bytes::lit(&[0x66])
                }
            }
        }
    }
}

pub fn f53(v0: u32, v1: u32) -> u32 {
    let m486 = {
        let a484 = v0;
        let a485 = v1;
        fixed_u32::shift_right(a484, a485)
    };
    match m486 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f54(v0: Adt2, v1: Adt4) -> R<Adt2> {
    let v2: Adt5 = f72(Fn7::F73, 64u64, Adt5::C0(match v0.clone() {
        Adt2::C0(h487, _, _, _, _, _, _, _) => {
            h487
        }
    }, match v0.clone() {
        Adt2::C0(_, h488, _, _, _, _, _, _) => {
            h488
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h489, _, _, _, _, _) => {
            h489
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h490, _, _, _, _) => {
            h490
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h491, _, _, _) => {
            h491
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h492, _, _) => {
            h492
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h493, _) => {
            h493
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h494) => {
            h494
        }
    }, v1, 0u64))?;
    Ok(Adt2::C0(f74(match v0.clone() {
        Adt2::C0(h495, _, _, _, _, _, _, _) => {
            h495
        }
    }, match v2.clone() {
        Adt5::C0(h496, _, _, _, _, _, _, _, _, _) => {
            h496
        }
    }), f74(match v0.clone() {
        Adt2::C0(_, h497, _, _, _, _, _, _) => {
            h497
        }
    }, match v2.clone() {
        Adt5::C0(_, h498, _, _, _, _, _, _, _, _) => {
            h498
        }
    }), f74(match v0.clone() {
        Adt2::C0(_, _, h499, _, _, _, _, _) => {
            h499
        }
    }, match v2.clone() {
        Adt5::C0(_, _, h500, _, _, _, _, _, _, _) => {
            h500
        }
    }), f74(match v0.clone() {
        Adt2::C0(_, _, _, h501, _, _, _, _) => {
            h501
        }
    }, match v2.clone() {
        Adt5::C0(_, _, _, h502, _, _, _, _, _, _) => {
            h502
        }
    }), f74(match v0.clone() {
        Adt2::C0(_, _, _, _, h503, _, _, _) => {
            h503
        }
    }, match v2.clone() {
        Adt5::C0(_, _, _, _, h504, _, _, _, _, _) => {
            h504
        }
    }), f74(match v0.clone() {
        Adt2::C0(_, _, _, _, _, h505, _, _) => {
            h505
        }
    }, match v2.clone() {
        Adt5::C0(_, _, _, _, _, h506, _, _, _, _) => {
            h506
        }
    }), f74(match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h507, _) => {
            h507
        }
    }, match v2.clone() {
        Adt5::C0(_, _, _, _, _, _, h508, _, _, _) => {
            h508
        }
    }), f74(match v0 {
        Adt2::C0(_, _, _, _, _, _, _, h509) => {
            h509
        }
    }, match v2 {
        Adt5::C0(_, _, _, _, _, _, _, h510, _, _) => {
            h510
        }
    })))
}

pub fn f55(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt4> {
    Ok(Adt4::C0(f75(v0.clone(), v1, v2, v3)?, f75(v0.clone(), v1, v2, {
        let a511 = v3;
        let a512 = 4u64;
        nat_add(a511, a512)?
    })?, f75(v0.clone(), v1, v2, {
        let a513 = v3;
        let a514 = 8u64;
        nat_add(a513, a514)?
    })?, f75(v0.clone(), v1, v2, {
        let a515 = v3;
        let a516 = 12u64;
        nat_add(a515, a516)?
    })?, f75(v0.clone(), v1, v2, {
        let a517 = v3;
        let a518 = 16u64;
        nat_add(a517, a518)?
    })?, f75(v0.clone(), v1, v2, {
        let a519 = v3;
        let a520 = 20u64;
        nat_add(a519, a520)?
    })?, f75(v0.clone(), v1, v2, {
        let a521 = v3;
        let a522 = 24u64;
        nat_add(a521, a522)?
    })?, f75(v0.clone(), v1, v2, {
        let a523 = v3;
        let a524 = 28u64;
        nat_add(a523, a524)?
    })?, f75(v0.clone(), v1, v2, {
        let a525 = v3;
        let a526 = 32u64;
        nat_add(a525, a526)?
    })?, f75(v0.clone(), v1, v2, {
        let a527 = v3;
        let a528 = 36u64;
        nat_add(a527, a528)?
    })?, f75(v0.clone(), v1, v2, {
        let a529 = v3;
        let a530 = 40u64;
        nat_add(a529, a530)?
    })?, f75(v0.clone(), v1, v2, {
        let a531 = v3;
        let a532 = 44u64;
        nat_add(a531, a532)?
    })?, f75(v0.clone(), v1, v2, {
        let a533 = v3;
        let a534 = 48u64;
        nat_add(a533, a534)?
    })?, f75(v0.clone(), v1, v2, {
        let a535 = v3;
        let a536 = 52u64;
        nat_add(a535, a536)?
    })?, f75(v0.clone(), v1, v2, {
        let a537 = v3;
        let a538 = 56u64;
        nat_add(a537, a538)?
    })?, f75(v0, v1, v2, {
        let a539 = v3;
        let a540 = 60u64;
        nat_add(a539, a540)?
    })?))
}

pub fn f56(v0: Fn5, p544: u64, p545: (u64, Adt3)) -> R<(u64, Adt3)> {
    let m543 = (p544, p545);
    {
        let mut m543 = m543;
        loop {
            let step = {
                let (v1, v2) = m543;
                let m541 = v1;
                if m541 == 0 {
                    Err::<(u64, (u64, Adt3)), R<(u64, Adt3)>>(Ok(v2))
                } else {
                    let v3 = m541 - 1;
                    Ok::<(u64, (u64, Adt3)), R<(u64, Adt3)>>((v3, {
                        let c542: Fn5 = v0.clone();
                        c542.apply(v2)?
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

pub fn f57(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: (u64, Adt3)) -> R<(u64, Adt3)> {
    Ok(({
        let a547 = {
            let (h546, _) = v4.clone();
            h546
        };
        let a548 = 136u64;
        nat_add(a547, a548)?
    }, f76(v0, v1, v3, v2, {
        let (_, h549) = v4;
        h549
    }, {
        let (h550, _) = v4;
        h550
    })?))
}

pub fn f58() -> Adt3 {
    Adt3::C0(0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64)
}

pub fn f59(v0: Adt0, v1: Adt6) -> R<Adt0> {
    let v2: Adt7 = f77(Fn8::F78, 80u64, Adt7::C0(match v0.clone() {
        Adt0::C0(h551, _, _, _, _, _, _, _) => {
            h551
        }
    }, match v0.clone() {
        Adt0::C0(_, h552, _, _, _, _, _, _) => {
            h552
        }
    }, match v0.clone() {
        Adt0::C0(_, _, h553, _, _, _, _, _) => {
            h553
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, h554, _, _, _, _) => {
            h554
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, h555, _, _, _) => {
            h555
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, h556, _, _) => {
            h556
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, h557, _) => {
            h557
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, h558) => {
            h558
        }
    }, v1, 0u64))?;
    Ok(Adt0::C0(f79(match v0.clone() {
        Adt0::C0(h559, _, _, _, _, _, _, _) => {
            h559
        }
    }, match v2.clone() {
        Adt7::C0(h560, _, _, _, _, _, _, _, _, _) => {
            h560
        }
    }), f79(match v0.clone() {
        Adt0::C0(_, h561, _, _, _, _, _, _) => {
            h561
        }
    }, match v2.clone() {
        Adt7::C0(_, h562, _, _, _, _, _, _, _, _) => {
            h562
        }
    }), f79(match v0.clone() {
        Adt0::C0(_, _, h563, _, _, _, _, _) => {
            h563
        }
    }, match v2.clone() {
        Adt7::C0(_, _, h564, _, _, _, _, _, _, _) => {
            h564
        }
    }), f79(match v0.clone() {
        Adt0::C0(_, _, _, h565, _, _, _, _) => {
            h565
        }
    }, match v2.clone() {
        Adt7::C0(_, _, _, h566, _, _, _, _, _, _) => {
            h566
        }
    }), f79(match v0.clone() {
        Adt0::C0(_, _, _, _, h567, _, _, _) => {
            h567
        }
    }, match v2.clone() {
        Adt7::C0(_, _, _, _, h568, _, _, _, _, _) => {
            h568
        }
    }), f79(match v0.clone() {
        Adt0::C0(_, _, _, _, _, h569, _, _) => {
            h569
        }
    }, match v2.clone() {
        Adt7::C0(_, _, _, _, _, h570, _, _, _, _) => {
            h570
        }
    }), f79(match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, h571, _) => {
            h571
        }
    }, match v2.clone() {
        Adt7::C0(_, _, _, _, _, _, h572, _, _, _) => {
            h572
        }
    }), f79(match v0 {
        Adt0::C0(_, _, _, _, _, _, _, h573) => {
            h573
        }
    }, match v2 {
        Adt7::C0(_, _, _, _, _, _, _, h574, _, _) => {
            h574
        }
    })))
}

pub fn f60(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt6> {
    Ok(Adt6::C0(f80(v0.clone(), v1, v2, {
        let a575 = v3;
        let a576 = 0u64;
        nat_add(a575, a576)?
    })?, f80(v0.clone(), v1, v2, {
        let a577 = v3;
        let a578 = 8u64;
        nat_add(a577, a578)?
    })?, f80(v0.clone(), v1, v2, {
        let a579 = v3;
        let a580 = 16u64;
        nat_add(a579, a580)?
    })?, f80(v0.clone(), v1, v2, {
        let a581 = v3;
        let a582 = 24u64;
        nat_add(a581, a582)?
    })?, f80(v0.clone(), v1, v2, {
        let a583 = v3;
        let a584 = 32u64;
        nat_add(a583, a584)?
    })?, f80(v0.clone(), v1, v2, {
        let a585 = v3;
        let a586 = 40u64;
        nat_add(a585, a586)?
    })?, f80(v0.clone(), v1, v2, {
        let a587 = v3;
        let a588 = 48u64;
        nat_add(a587, a588)?
    })?, f80(v0.clone(), v1, v2, {
        let a589 = v3;
        let a590 = 56u64;
        nat_add(a589, a590)?
    })?, f80(v0.clone(), v1, v2, {
        let a591 = v3;
        let a592 = 64u64;
        nat_add(a591, a592)?
    })?, f80(v0.clone(), v1, v2, {
        let a593 = v3;
        let a594 = 72u64;
        nat_add(a593, a594)?
    })?, f80(v0.clone(), v1, v2, {
        let a595 = v3;
        let a596 = 80u64;
        nat_add(a595, a596)?
    })?, f80(v0.clone(), v1, v2, {
        let a597 = v3;
        let a598 = 88u64;
        nat_add(a597, a598)?
    })?, f80(v0.clone(), v1, v2, {
        let a599 = v3;
        let a600 = 96u64;
        nat_add(a599, a600)?
    })?, f80(v0.clone(), v1, v2, {
        let a601 = v3;
        let a602 = 104u64;
        nat_add(a601, a602)?
    })?, f80(v0.clone(), v1, v2, {
        let a603 = v3;
        let a604 = 112u64;
        nat_add(a603, a604)?
    })?, f80(v0, v1, v2, {
        let a605 = v3;
        let a606 = 120u64;
        nat_add(a605, a606)?
    })?))
}

pub fn f61(v0: u64) -> u32 {
    let m610 = {
        let a609 = {
            let a607 = v0;
            let a608 = 4294967295u64;
            fixed_u64::bit_and(a607, a608)
        };
        fixed_u32::convert(i128::from(a609))
    };
    match m610 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f62(v0: u64, v1: u32) -> u64 {
    let m613 = {
        let a611 = v0;
        let a612 = v1;
        fixed_u64::shift_right(a611, a612)
    };
    match m613 {
        None => {
            0u64
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f63(v0: Fn6, p617: u64, p618: (u64, List<Adt1>)) -> R<(u64, List<Adt1>)> {
    let m616 = (p617, p618);
    {
        let mut m616 = m616;
        loop {
            let step = {
                let (v1, v2) = m616;
                let m614 = v1;
                if m614 == 0 {
                    Err::<(u64, (u64, List<Adt1>)), R<(u64, List<Adt1>)>>(Ok(v2))
                } else {
                    let v3 = m614 - 1;
                    Ok::<(u64, (u64, List<Adt1>)), R<(u64, List<Adt1>)>>((v3, {
                        let c615: Fn6 = v0.clone();
                        c615.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m616 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f64(v0: Bytes, v1: u64, v2: (u64, List<Adt1>)) -> R<(u64, List<Adt1>)> {
    f81(v2.clone(), f47(v0, v1, {
        let (h619, _) = v2;
        h619
    }, false)?)
}

pub fn f65(v0: u64, v1: u64) -> u64 {
    let a622 = 1024u64;
    let a623 = {
        let a620 = v0;
        let a621 = v1;
        nat_sub(a620, a621)
    };
    if nat_lt(a622, a623) {
        1024u64
    } else {
        let a624 = v0;
        let a625 = v1;
        nat_sub(a624, a625)
    }
}

pub fn f66(v0: u64) -> R<u64> {
    let a626 = v0;
    let a627 = 0u64;
    if nat_eq(a626, a627) {
        Ok(1u64)
    } else {
        let a630 = {
            let a628 = v0;
            let a629 = 63u64;
            nat_add(a628, a629)?
        };
        let a631 = 64u64;
        let a632 = 0u64;
        Ok(nat_quot(a630, a631, a632))
    }
}

pub fn f67(v0: Adt1, v1: Adt8, v2: u64, v3: u32, v4: u32) -> Adt1 {
    let v5: Adt9 = {
        let (h633, _) = f82(Fn9::F83, 7u64, (Adt9::C0(match v0.clone() {
            Adt1::C0(h634, _, _, _, _, _, _, _) => {
                h634
            }
        }, match v0.clone() {
            Adt1::C0(_, h635, _, _, _, _, _, _) => {
                h635
            }
        }, match v0.clone() {
            Adt1::C0(_, _, h636, _, _, _, _, _) => {
                h636
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, h637, _, _, _, _) => {
                h637
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, h638, _, _, _) => {
                h638
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, h639, _, _) => {
                h639
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h640, _) => {
                h640
            }
        }, match v0 {
            Adt1::C0(_, _, _, _, _, _, _, h641) => {
                h641
            }
        }, 1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, f70({
            let a642 = v2;
            let a643 = 4294967296u64;
            let a644 = 0u64;
            nat_rem(a642, a643, a644)
        }), f70({
            let a645 = v2;
            let a646 = 4294967296u64;
            let a647 = 0u64;
            nat_quot(a645, a646, a647)
        }), v3, v4), v1));
        h633
    };
    Adt1::C0({
        let a650 = match v5.clone() {
            Adt9::C0(h648, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h648
            }
        };
        let a651 = match v5.clone() {
            Adt9::C0(_, _, _, _, _, _, _, _, h649, _, _, _, _, _, _, _) => {
                h649
            }
        };
        fixed_u32::bit_xor(a650, a651)
    }, {
        let a654 = match v5.clone() {
            Adt9::C0(_, h652, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h652
            }
        };
        let a655 = match v5.clone() {
            Adt9::C0(_, _, _, _, _, _, _, _, _, h653, _, _, _, _, _, _) => {
                h653
            }
        };
        fixed_u32::bit_xor(a654, a655)
    }, {
        let a658 = match v5.clone() {
            Adt9::C0(_, _, h656, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h656
            }
        };
        let a659 = match v5.clone() {
            Adt9::C0(_, _, _, _, _, _, _, _, _, _, h657, _, _, _, _, _) => {
                h657
            }
        };
        fixed_u32::bit_xor(a658, a659)
    }, {
        let a662 = match v5.clone() {
            Adt9::C0(_, _, _, h660, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h660
            }
        };
        let a663 = match v5.clone() {
            Adt9::C0(_, _, _, _, _, _, _, _, _, _, _, h661, _, _, _, _) => {
                h661
            }
        };
        fixed_u32::bit_xor(a662, a663)
    }, {
        let a666 = match v5.clone() {
            Adt9::C0(_, _, _, _, h664, _, _, _, _, _, _, _, _, _, _, _) => {
                h664
            }
        };
        let a667 = match v5.clone() {
            Adt9::C0(_, _, _, _, _, _, _, _, _, _, _, _, h665, _, _, _) => {
                h665
            }
        };
        fixed_u32::bit_xor(a666, a667)
    }, {
        let a670 = match v5.clone() {
            Adt9::C0(_, _, _, _, _, h668, _, _, _, _, _, _, _, _, _, _) => {
                h668
            }
        };
        let a671 = match v5.clone() {
            Adt9::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h669, _, _) => {
                h669
            }
        };
        fixed_u32::bit_xor(a670, a671)
    }, {
        let a674 = match v5.clone() {
            Adt9::C0(_, _, _, _, _, _, h672, _, _, _, _, _, _, _, _, _) => {
                h672
            }
        };
        let a675 = match v5.clone() {
            Adt9::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h673, _) => {
                h673
            }
        };
        fixed_u32::bit_xor(a674, a675)
    }, {
        let a678 = match v5.clone() {
            Adt9::C0(_, _, _, _, _, _, _, h676, _, _, _, _, _, _, _, _) => {
                h676
            }
        };
        let a679 = match v5 {
            Adt9::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h677) => {
                h677
            }
        };
        fixed_u32::bit_xor(a678, a679)
    })
}

pub fn f68(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt1> {
    let (_, h680) = f84(Fn10::F85(v2, v0, v1), v3, (0u64, f86()))?;
    Ok(h680)
}

pub fn f69(v0: Bytes, v1: u64) -> R<Adt8> {
    Ok(Adt8::C0(f87(v0.clone(), {
        let a681 = v1;
        let a682 = 0u64;
        nat_add(a681, a682)?
    })?, f87(v0.clone(), {
        let a683 = v1;
        let a684 = 4u64;
        nat_add(a683, a684)?
    })?, f87(v0.clone(), {
        let a685 = v1;
        let a686 = 8u64;
        nat_add(a685, a686)?
    })?, f87(v0.clone(), {
        let a687 = v1;
        let a688 = 12u64;
        nat_add(a687, a688)?
    })?, f87(v0.clone(), {
        let a689 = v1;
        let a690 = 16u64;
        nat_add(a689, a690)?
    })?, f87(v0.clone(), {
        let a691 = v1;
        let a692 = 20u64;
        nat_add(a691, a692)?
    })?, f87(v0.clone(), {
        let a693 = v1;
        let a694 = 24u64;
        nat_add(a693, a694)?
    })?, f87(v0.clone(), {
        let a695 = v1;
        let a696 = 28u64;
        nat_add(a695, a696)?
    })?, f87(v0.clone(), {
        let a697 = v1;
        let a698 = 32u64;
        nat_add(a697, a698)?
    })?, f87(v0.clone(), {
        let a699 = v1;
        let a700 = 36u64;
        nat_add(a699, a700)?
    })?, f87(v0.clone(), {
        let a701 = v1;
        let a702 = 40u64;
        nat_add(a701, a702)?
    })?, f87(v0.clone(), {
        let a703 = v1;
        let a704 = 44u64;
        nat_add(a703, a704)?
    })?, f87(v0.clone(), {
        let a705 = v1;
        let a706 = 48u64;
        nat_add(a705, a706)?
    })?, f87(v0.clone(), {
        let a707 = v1;
        let a708 = 52u64;
        nat_add(a707, a708)?
    })?, f87(v0.clone(), {
        let a709 = v1;
        let a710 = 56u64;
        nat_add(a709, a710)?
    })?, f87(v0, {
        let a711 = v1;
        let a712 = 60u64;
        nat_add(a711, a712)?
    })?))
}

pub fn f70(v0: u64) -> u32 {
    let (_, h713) = {
        let (_, h714) = f88(Fn11::F89, 32u64, (v0, (1u32, 0u32)));
        h714
    };
    h713
}

pub fn f71(v0: (List<Adt1>, Adt1)) -> (List<Adt1>, Adt1) {
    let m716 = {
        let (h715, _) = v0.clone();
        h715
    };
    match m716.uncons() {
        None => {
            v0
        }
        Some((v1, v2)) => {
            (v2.clone(), f90(v1, {
                let (_, h717) = v0;
                h717
            }, {
                let m718 = v2;
                match m718.uncons() {
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

pub fn f72(v0: Fn7, p722: u64, p723: Adt5) -> R<Adt5> {
    let m721 = (p722, p723);
    {
        let mut m721 = m721;
        loop {
            let step = {
                let (v1, v2) = m721;
                let m719 = v1;
                if m719 == 0 {
                    Err::<(u64, Adt5), R<Adt5>>(Ok(v2))
                } else {
                    let v3 = m719 - 1;
                    Ok::<(u64, Adt5), R<Adt5>>((v3, {
                        let c720: Fn7 = v0.clone();
                        c720.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m721 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f73(v0: Adt5) -> R<Adt5> {
    f91(v0)
}

pub fn f74(v0: u32, v1: u32) -> u32 {
    let m726 = {
        let a724 = f92(v0);
        let a725 = f92(v1);
        fixed_u64::checked_add(a724, a725)
    };
    match m726 {
        None => {
            0u32
        }
        Some(v2) => {
            f61(v2)
        }
    }
}

pub fn f75(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u32> {
    let a729 = {
        let a727 = v3;
        let a728 = 8u64;
        nat_add(a727, a728)?
    };
    let a730 = v2;
    if nat_eq(a729, a730) {
        Ok(f70({
            let a733 = {
                let a731 = v1;
                let a732 = 8u64;
                nat_mul(a731, a732)?
            };
            let a734 = 4294967296u64;
            let a735 = 0u64;
            nat_quot(a733, a734, a735)
        }))
    } else {
        let a738 = {
            let a736 = v3;
            let a737 = 4u64;
            nat_add(a736, a737)?
        };
        let a739 = v2;
        if nat_eq(a738, a739) {
            Ok(f70({
                let a742 = {
                    let a740 = v1;
                    let a741 = 8u64;
                    nat_mul(a740, a741)?
                };
                let a743 = 4294967296u64;
                let a744 = 0u64;
                nat_rem(a742, a743, a744)
            }))
        } else {
            Ok(f93(f94(v0.clone(), v1, v3), f94(v0.clone(), v1, {
                let a745 = v3;
                let a746 = 1u64;
                nat_add(a745, a746)?
            }), f94(v0.clone(), v1, {
                let a747 = v3;
                let a748 = 2u64;
                nat_add(a747, a748)?
            }), f94(v0, v1, {
                let a749 = v3;
                let a750 = 3u64;
                nat_add(a749, a750)?
            })))
        }
    }
}

pub fn f76(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: Adt3, v5: u64) -> R<Adt3> {
    f95(Adt3::C0({
        let a754 = match v4.clone() {
            Adt3::C0(h751, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h751
            }
        };
        let a755 = f96(v0.clone(), v1, v2, v3, {
            let a752 = v5;
            let a753 = 0u64;
            nat_add(a752, a753)?
        })?;
        fixed_u64::bit_xor(a754, a755)
    }, {
        let a759 = match v4.clone() {
            Adt3::C0(_, h756, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h756
            }
        };
        let a760 = f96(v0.clone(), v1, v2, v3, {
            let a757 = v5;
            let a758 = 8u64;
            nat_add(a757, a758)?
        })?;
        fixed_u64::bit_xor(a759, a760)
    }, {
        let a764 = match v4.clone() {
            Adt3::C0(_, _, h761, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h761
            }
        };
        let a765 = f96(v0.clone(), v1, v2, v3, {
            let a762 = v5;
            let a763 = 16u64;
            nat_add(a762, a763)?
        })?;
        fixed_u64::bit_xor(a764, a765)
    }, {
        let a769 = match v4.clone() {
            Adt3::C0(_, _, _, h766, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h766
            }
        };
        let a770 = f96(v0.clone(), v1, v2, v3, {
            let a767 = v5;
            let a768 = 24u64;
            nat_add(a767, a768)?
        })?;
        fixed_u64::bit_xor(a769, a770)
    }, {
        let a774 = match v4.clone() {
            Adt3::C0(_, _, _, _, h771, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h771
            }
        };
        let a775 = f96(v0.clone(), v1, v2, v3, {
            let a772 = v5;
            let a773 = 32u64;
            nat_add(a772, a773)?
        })?;
        fixed_u64::bit_xor(a774, a775)
    }, {
        let a779 = match v4.clone() {
            Adt3::C0(_, _, _, _, _, h776, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h776
            }
        };
        let a780 = f96(v0.clone(), v1, v2, v3, {
            let a777 = v5;
            let a778 = 40u64;
            nat_add(a777, a778)?
        })?;
        fixed_u64::bit_xor(a779, a780)
    }, {
        let a784 = match v4.clone() {
            Adt3::C0(_, _, _, _, _, _, h781, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h781
            }
        };
        let a785 = f96(v0.clone(), v1, v2, v3, {
            let a782 = v5;
            let a783 = 48u64;
            nat_add(a782, a783)?
        })?;
        fixed_u64::bit_xor(a784, a785)
    }, {
        let a789 = match v4.clone() {
            Adt3::C0(_, _, _, _, _, _, _, h786, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h786
            }
        };
        let a790 = f96(v0.clone(), v1, v2, v3, {
            let a787 = v5;
            let a788 = 56u64;
            nat_add(a787, a788)?
        })?;
        fixed_u64::bit_xor(a789, a790)
    }, {
        let a794 = match v4.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, h791, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h791
            }
        };
        let a795 = f96(v0.clone(), v1, v2, v3, {
            let a792 = v5;
            let a793 = 64u64;
            nat_add(a792, a793)?
        })?;
        fixed_u64::bit_xor(a794, a795)
    }, {
        let a799 = match v4.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, h796, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h796
            }
        };
        let a800 = f96(v0.clone(), v1, v2, v3, {
            let a797 = v5;
            let a798 = 72u64;
            nat_add(a797, a798)?
        })?;
        fixed_u64::bit_xor(a799, a800)
    }, {
        let a804 = match v4.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, h801, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h801
            }
        };
        let a805 = f96(v0.clone(), v1, v2, v3, {
            let a802 = v5;
            let a803 = 80u64;
            nat_add(a802, a803)?
        })?;
        fixed_u64::bit_xor(a804, a805)
    }, {
        let a809 = match v4.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, h806, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h806
            }
        };
        let a810 = f96(v0.clone(), v1, v2, v3, {
            let a807 = v5;
            let a808 = 88u64;
            nat_add(a807, a808)?
        })?;
        fixed_u64::bit_xor(a809, a810)
    }, {
        let a814 = match v4.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, h811, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h811
            }
        };
        let a815 = f96(v0.clone(), v1, v2, v3, {
            let a812 = v5;
            let a813 = 96u64;
            nat_add(a812, a813)?
        })?;
        fixed_u64::bit_xor(a814, a815)
    }, {
        let a819 = match v4.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h816, _, _, _, _, _, _, _, _, _, _, _) => {
                h816
            }
        };
        let a820 = f96(v0.clone(), v1, v2, v3, {
            let a817 = v5;
            let a818 = 104u64;
            nat_add(a817, a818)?
        })?;
        fixed_u64::bit_xor(a819, a820)
    }, {
        let a824 = match v4.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h821, _, _, _, _, _, _, _, _, _, _) => {
                h821
            }
        };
        let a825 = f96(v0.clone(), v1, v2, v3, {
            let a822 = v5;
            let a823 = 112u64;
            nat_add(a822, a823)?
        })?;
        fixed_u64::bit_xor(a824, a825)
    }, {
        let a829 = match v4.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h826, _, _, _, _, _, _, _, _, _) => {
                h826
            }
        };
        let a830 = f96(v0.clone(), v1, v2, v3, {
            let a827 = v5;
            let a828 = 120u64;
            nat_add(a827, a828)?
        })?;
        fixed_u64::bit_xor(a829, a830)
    }, {
        let a834 = match v4.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h831, _, _, _, _, _, _, _, _) => {
                h831
            }
        };
        let a835 = f96(v0, v1, v2, v3, {
            let a832 = v5;
            let a833 = 128u64;
            nat_add(a832, a833)?
        })?;
        fixed_u64::bit_xor(a834, a835)
    }, match v4.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h836, _, _, _, _, _, _, _) => {
            h836
        }
    }, match v4.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h837, _, _, _, _, _, _) => {
            h837
        }
    }, match v4.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h838, _, _, _, _, _) => {
            h838
        }
    }, match v4.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h839, _, _, _, _) => {
            h839
        }
    }, match v4.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h840, _, _, _) => {
            h840
        }
    }, match v4.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h841, _, _) => {
            h841
        }
    }, match v4.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h842, _) => {
            h842
        }
    }, match v4 {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h843) => {
            h843
        }
    }))
}

pub fn f77(v0: Fn8, p847: u64, p848: Adt7) -> R<Adt7> {
    let m846 = (p847, p848);
    {
        let mut m846 = m846;
        loop {
            let step = {
                let (v1, v2) = m846;
                let m844 = v1;
                if m844 == 0 {
                    Err::<(u64, Adt7), R<Adt7>>(Ok(v2))
                } else {
                    let v3 = m844 - 1;
                    Ok::<(u64, Adt7), R<Adt7>>((v3, {
                        let c845: Fn8 = v0.clone();
                        c845.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m846 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f78(v0: Adt7) -> R<Adt7> {
    f97(v0)
}

pub fn f79(v0: u64, v1: u64) -> u64 {
    let v2: u64 = 4294967295u64;
    let v3: u64 = f98({
        let a849 = v0;
        let a850 = v2;
        fixed_u64::bit_and(a849, a850)
    }, {
        let a851 = v1;
        let a852 = v2;
        fixed_u64::bit_and(a851, a852)
    });
    let v4: u64 = f98(f98(f62(v0, 32u32), f62(v1, 32u32)), f62(v3, 32u32));
    let a857 = f99({
        let a853 = v4;
        let a854 = v2;
        fixed_u64::bit_and(a853, a854)
    }, 32u32);
    let a858 = {
        let a855 = v3;
        let a856 = v2;
        fixed_u64::bit_and(a855, a856)
    };
    fixed_u64::bit_or(a857, a858)
}

pub fn f80(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a861 = {
        let a859 = v3;
        let a860 = 16u64;
        nat_add(a859, a860)?
    };
    let a862 = v2;
    if nat_eq(a861, a862) {
        Ok(0u64)
    } else {
        let a865 = {
            let a863 = v3;
            let a864 = 8u64;
            nat_add(a863, a864)?
        };
        let a866 = v2;
        if nat_eq(a865, a866) {
            Ok(f100({
                let a867 = v1;
                let a868 = 8u64;
                nat_mul(a867, a868)?
            }))
        } else {
            Ok(f101(f102(v0.clone(), v1, {
                let a869 = v3;
                let a870 = 0u64;
                nat_add(a869, a870)?
            }), f102(v0.clone(), v1, {
                let a871 = v3;
                let a872 = 1u64;
                nat_add(a871, a872)?
            }), f102(v0.clone(), v1, {
                let a873 = v3;
                let a874 = 2u64;
                nat_add(a873, a874)?
            }), f102(v0.clone(), v1, {
                let a875 = v3;
                let a876 = 3u64;
                nat_add(a875, a876)?
            }), f102(v0.clone(), v1, {
                let a877 = v3;
                let a878 = 4u64;
                nat_add(a877, a878)?
            }), f102(v0.clone(), v1, {
                let a879 = v3;
                let a880 = 5u64;
                nat_add(a879, a880)?
            }), f102(v0.clone(), v1, {
                let a881 = v3;
                let a882 = 6u64;
                nat_add(a881, a882)?
            }), f102(v0, v1, {
                let a883 = v3;
                let a884 = 7u64;
                nat_add(a883, a884)?
            })))
        }
    }
}

pub fn f81(v0: (u64, List<Adt1>), v1: Adt1) -> R<(u64, List<Adt1>)> {
    let v2: Adt10 = f103(Fn12::F104, 40u64, Adt10::C0(v1, {
        let a886 = {
            let (h885, _) = v0.clone();
            h885
        };
        let a887 = 1u64;
        nat_add(a886, a887)?
    }, {
        let (_, h888) = v0;
        h888
    }));
    Ok(({
        let a890 = {
            let (h889, _) = v0;
            h889
        };
        let a891 = 1u64;
        nat_add(a890, a891)?
    }, List::cons(match v2.clone() {
        Adt10::C0(h892, _, _) => {
            h892
        }
    }, match v2 {
        Adt10::C0(_, _, h893) => {
            h893
        }
    })))
}

pub fn f82(v0: Fn9, p897: u64, p898: (Adt9, Adt8)) -> (Adt9, Adt8) {
    let m896 = (p897, p898);
    {
        let mut m896 = m896;
        loop {
            let step = {
                let (v1, v2) = m896;
                let m894 = v1;
                if m894 == 0 {
                    Err::<(u64, (Adt9, Adt8)), (Adt9, Adt8)>(v2)
                } else {
                    let v3 = m894 - 1;
                    Ok::<(u64, (Adt9, Adt8)), (Adt9, Adt8)>((v3, {
                        let c895: Fn9 = v0.clone();
                        c895.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m896 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f83(v0: (Adt9, Adt8)) -> (Adt9, Adt8) {
    f105(v0)
}

pub fn f84(v0: Fn10, p902: u64, p903: (u64, Adt1)) -> R<(u64, Adt1)> {
    let m901 = (p902, p903);
    {
        let mut m901 = m901;
        loop {
            let step = {
                let (v1, v2) = m901;
                let m899 = v1;
                if m899 == 0 {
                    Err::<(u64, (u64, Adt1)), R<(u64, Adt1)>>(Ok(v2))
                } else {
                    let v3 = m899 - 1;
                    Ok::<(u64, (u64, Adt1)), R<(u64, Adt1)>>((v3, {
                        let c900: Fn10 = v0.clone();
                        c900.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m901 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f85(v0: u64, v1: Bytes, v2: u64, v3: (u64, Adt1)) -> R<(u64, Adt1)> {
    Ok(({
        let a905 = {
            let (h904, _) = v3.clone();
            h904
        };
        let a906 = 1u64;
        nat_add(a905, a906)?
    }, f67({
        let (_, h907) = v3.clone();
        h907
    }, f69(v1, {
        let a911 = v2;
        let a912 = {
            let a909 = 64u64;
            let a910 = {
                let (h908, _) = v3.clone();
                h908
            };
            nat_mul(a909, a910)?
        };
        nat_add(a911, a912)?
    })?, v0, 64u32, {
        let a914 = {
            let (h913, _) = v3;
            h913
        };
        let a915 = 0u64;
        if nat_eq(a914, a915) {
            1u32
        } else {
            0u32
        }
    })))
}

pub fn f86() -> Adt1 {
    Adt1::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f87(v0: Bytes, v1: u64) -> R<u32> {
    let a926 = f106(v0.clone(), v1);
    let a927 = {
        let a924 = f107(f106(v0.clone(), {
            let a916 = v1;
            let a917 = 1u64;
            nat_add(a916, a917)?
        }), 8u32);
        let a925 = {
            let a922 = f107(f106(v0.clone(), {
                let a918 = v1;
                let a919 = 2u64;
                nat_add(a918, a919)?
            }), 16u32);
            let a923 = f107(f106(v0, {
                let a920 = v1;
                let a921 = 3u64;
                nat_add(a920, a921)?
            }), 24u32);
            fixed_u32::bit_or(a922, a923)
        };
        fixed_u32::bit_or(a924, a925)
    };
    Ok(fixed_u32::bit_or(a926, a927))
}

pub fn f88(v0: Fn11, p931: u64, p932: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m930 = (p931, p932);
    {
        let mut m930 = m930;
        loop {
            let step = {
                let (v1, v2) = m930;
                let m928 = v1;
                if m928 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m928 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c929: Fn11 = v0.clone();
                        c929.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m930 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f89(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h933, _) = v0;
        h933
    };
    let v2: u32 = {
        let (h934, _) = {
            let (_, h935) = v0;
            h935
        };
        h934
    };
    let v3: u32 = {
        let (_, h936) = {
            let (_, h937) = v0;
            h937
        };
        h936
    };
    ({
        let a938 = v1;
        let a939 = 2u64;
        let a940 = 0u64;
        nat_quot(a938, a939, a940)
    }, (f107(v2, 1u32), {
        let a944 = {
            let a941 = v1;
            let a942 = 2u64;
            let a943 = 0u64;
            nat_rem(a941, a942, a943)
        };
        let a945 = 1u64;
        if nat_eq(a944, a945) {
            let a946 = v3;
            let a947 = v2;
            fixed_u32::bit_or(a946, a947)
        } else {
            v3
        }
    }))
}

pub fn f90(v0: Adt1, v1: Adt1, v2: bool) -> Adt1 {
    f67(f86(), f108(v0, v1), 0u64, 64u32, {
        let a948 = 4u32;
        let a949 = if v2 {
            8u32
        } else {
            0u32
        };
        fixed_u32::bit_or(a948, a949)
    })
}

pub fn f91(v0: Adt5) -> R<Adt5> {
    let v1: u32 = f109(match v0.clone() {
        Adt5::C0(_, _, _, _, _, _, _, _, _, h950) => {
            h950
        }
    });
    let v2: u32 = f74(match v0.clone() {
        Adt5::C0(_, _, _, _, _, _, _, h951, _, _) => {
            h951
        }
    }, f74(f110(match v0.clone() {
        Adt5::C0(_, _, _, _, h952, _, _, _, _, _) => {
            h952
        }
    }), f74(f111(match v0.clone() {
        Adt5::C0(_, _, _, _, h953, _, _, _, _, _) => {
            h953
        }
    }, match v0.clone() {
        Adt5::C0(_, _, _, _, _, h954, _, _, _, _) => {
            h954
        }
    }, match v0.clone() {
        Adt5::C0(_, _, _, _, _, _, h955, _, _, _) => {
            h955
        }
    }), f74(v1, {
        let m958 = match v0.clone() {
            Adt5::C0(_, _, _, _, _, _, _, _, h957, _) => {
                h957
            }
        };
        match m958 {
            Adt4::C0(h956, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h956
            }
        }
    }))));
    let v3: u32 = f74(f112(match v0.clone() {
        Adt5::C0(h959, _, _, _, _, _, _, _, _, _) => {
            h959
        }
    }), f113(match v0.clone() {
        Adt5::C0(h960, _, _, _, _, _, _, _, _, _) => {
            h960
        }
    }, match v0.clone() {
        Adt5::C0(_, h961, _, _, _, _, _, _, _, _) => {
            h961
        }
    }, match v0.clone() {
        Adt5::C0(_, _, h962, _, _, _, _, _, _, _) => {
            h962
        }
    }));
    Ok(Adt5::C0(f74(v2, v3), match v0.clone() {
        Adt5::C0(h963, _, _, _, _, _, _, _, _, _) => {
            h963
        }
    }, match v0.clone() {
        Adt5::C0(_, h964, _, _, _, _, _, _, _, _) => {
            h964
        }
    }, match v0.clone() {
        Adt5::C0(_, _, h965, _, _, _, _, _, _, _) => {
            h965
        }
    }, f74(match v0.clone() {
        Adt5::C0(_, _, _, h966, _, _, _, _, _, _) => {
            h966
        }
    }, v2), match v0.clone() {
        Adt5::C0(_, _, _, _, h967, _, _, _, _, _) => {
            h967
        }
    }, match v0.clone() {
        Adt5::C0(_, _, _, _, _, h968, _, _, _, _) => {
            h968
        }
    }, match v0.clone() {
        Adt5::C0(_, _, _, _, _, _, h969, _, _, _) => {
            h969
        }
    }, f114(match v0.clone() {
        Adt5::C0(_, _, _, _, _, _, _, _, h970, _) => {
            h970
        }
    }), {
        let a972 = match v0 {
            Adt5::C0(_, _, _, _, _, _, _, _, _, h971) => {
                h971
            }
        };
        let a973 = 1u64;
        nat_add(a972, a973)?
    }))
}

pub fn f92(v0: u32) -> u64 {
    let m975 = {
        let a974 = v0;
        fixed_u64::convert(i128::from(a974))
    };
    match m975 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f93(v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
    let a980 = f107(v0, 24u32);
    let a981 = {
        let a978 = f107(v1, 16u32);
        let a979 = {
            let a976 = f107(v2, 8u32);
            let a977 = v3;
            fixed_u32::bit_or(a976, a977)
        };
        fixed_u32::bit_or(a978, a979)
    };
    fixed_u32::bit_or(a980, a981)
}

pub fn f94(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a982 = v2;
    let a983 = v1;
    if nat_lt(a982, a983) {
        let m986 = {
            let a984 = v0;
            let a985 = v2;
            index_bytes(a984, a985)
        };
        match m986 {
            None => {
                0u32
            }
            Some(v3) => {
                let m988 = {
                    let a987 = v3;
                    fixed_u32::convert(i128::from(a987))
                };
                match m988 {
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
        let a989 = v2;
        let a990 = v1;
        if nat_eq(a989, a990) {
            128u32
        } else {
            0u32
        }
    }
}

pub fn f95(v0: Adt3) -> R<Adt3> {
    let (_, h991) = f56(Fn5::F115, 24u64, (0u64, v0))?;
    Ok(h991)
}

pub fn f96(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    Ok(f116(f117(v0.clone(), v1, v2, v3, {
        let a992 = v4;
        let a993 = 0u64;
        nat_add(a992, a993)?
    })?, f117(v0.clone(), v1, v2, v3, {
        let a994 = v4;
        let a995 = 1u64;
        nat_add(a994, a995)?
    })?, f117(v0.clone(), v1, v2, v3, {
        let a996 = v4;
        let a997 = 2u64;
        nat_add(a996, a997)?
    })?, f117(v0.clone(), v1, v2, v3, {
        let a998 = v4;
        let a999 = 3u64;
        nat_add(a998, a999)?
    })?, f117(v0.clone(), v1, v2, v3, {
        let a1000 = v4;
        let a1001 = 4u64;
        nat_add(a1000, a1001)?
    })?, f117(v0.clone(), v1, v2, v3, {
        let a1002 = v4;
        let a1003 = 5u64;
        nat_add(a1002, a1003)?
    })?, f117(v0.clone(), v1, v2, v3, {
        let a1004 = v4;
        let a1005 = 6u64;
        nat_add(a1004, a1005)?
    })?, f117(v0, v1, v2, v3, {
        let a1006 = v4;
        let a1007 = 7u64;
        nat_add(a1006, a1007)?
    })?))
}

pub fn f97(v0: Adt7) -> R<Adt7> {
    let v1: u64 = f118(match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, h1008) => {
            h1008
        }
    });
    let v2: u64 = f79(match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, h1009, _, _) => {
            h1009
        }
    }, f79(f119(match v0.clone() {
        Adt7::C0(_, _, _, _, h1010, _, _, _, _, _) => {
            h1010
        }
    }), f79(f120(match v0.clone() {
        Adt7::C0(_, _, _, _, h1011, _, _, _, _, _) => {
            h1011
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, h1012, _, _, _, _) => {
            h1012
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, h1013, _, _, _) => {
            h1013
        }
    }), f79(v1, {
        let m1016 = match v0.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, h1015, _) => {
                h1015
            }
        };
        match m1016 {
            Adt6::C0(h1014, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1014
            }
        }
    }))));
    let v3: u64 = f79(f121(match v0.clone() {
        Adt7::C0(h1017, _, _, _, _, _, _, _, _, _) => {
            h1017
        }
    }), f122(match v0.clone() {
        Adt7::C0(h1018, _, _, _, _, _, _, _, _, _) => {
            h1018
        }
    }, match v0.clone() {
        Adt7::C0(_, h1019, _, _, _, _, _, _, _, _) => {
            h1019
        }
    }, match v0.clone() {
        Adt7::C0(_, _, h1020, _, _, _, _, _, _, _) => {
            h1020
        }
    }));
    Ok(Adt7::C0(f79(v2, v3), match v0.clone() {
        Adt7::C0(h1021, _, _, _, _, _, _, _, _, _) => {
            h1021
        }
    }, match v0.clone() {
        Adt7::C0(_, h1022, _, _, _, _, _, _, _, _) => {
            h1022
        }
    }, match v0.clone() {
        Adt7::C0(_, _, h1023, _, _, _, _, _, _, _) => {
            h1023
        }
    }, f79(match v0.clone() {
        Adt7::C0(_, _, _, h1024, _, _, _, _, _, _) => {
            h1024
        }
    }, v2), match v0.clone() {
        Adt7::C0(_, _, _, _, h1025, _, _, _, _, _) => {
            h1025
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, h1026, _, _, _, _) => {
            h1026
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, h1027, _, _, _) => {
            h1027
        }
    }, f123(match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, h1028, _) => {
            h1028
        }
    }), {
        let a1030 = match v0 {
            Adt7::C0(_, _, _, _, _, _, _, _, _, h1029) => {
                h1029
            }
        };
        let a1031 = 1u64;
        nat_add(a1030, a1031)?
    }))
}

pub fn f98(v0: u64, v1: u64) -> u64 {
    let m1034 = {
        let a1032 = v0;
        let a1033 = v1;
        fixed_u64::checked_add(a1032, a1033)
    };
    match m1034 {
        None => {
            0u64
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f99(v0: u64, v1: u32) -> u64 {
    let m1037 = {
        let a1035 = v0;
        let a1036 = v1;
        fixed_u64::shift_left(a1035, a1036)
    };
    match m1037 {
        None => {
            0u64
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f100(v0: u64) -> u64 {
    let (_, h1038) = {
        let (_, h1039) = f124(Fn13::F125, 64u64, (v0, (1u64, 0u64)));
        h1039
    };
    h1038
}

pub fn f101(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> u64 {
    let a1052 = f99(v0, 56u32);
    let a1053 = {
        let a1050 = f99(v1, 48u32);
        let a1051 = {
            let a1048 = f99(v2, 40u32);
            let a1049 = {
                let a1046 = f99(v3, 32u32);
                let a1047 = {
                    let a1044 = f99(v4, 24u32);
                    let a1045 = {
                        let a1042 = f99(v5, 16u32);
                        let a1043 = {
                            let a1040 = f99(v6, 8u32);
                            let a1041 = v7;
                            fixed_u64::bit_or(a1040, a1041)
                        };
                        fixed_u64::bit_or(a1042, a1043)
                    };
                    fixed_u64::bit_or(a1044, a1045)
                };
                fixed_u64::bit_or(a1046, a1047)
            };
            fixed_u64::bit_or(a1048, a1049)
        };
        fixed_u64::bit_or(a1050, a1051)
    };
    fixed_u64::bit_or(a1052, a1053)
}

pub fn f102(v0: Bytes, v1: u64, v2: u64) -> u64 {
    let a1054 = v2;
    let a1055 = v1;
    if nat_lt(a1054, a1055) {
        f126(v0, v2)
    } else {
        let a1056 = v2;
        let a1057 = v1;
        if nat_eq(a1056, a1057) {
            128u64
        } else {
            0u64
        }
    }
}

pub fn f103(v0: Fn12, p1061: u64, p1062: Adt10) -> Adt10 {
    let m1060 = (p1061, p1062);
    {
        let mut m1060 = m1060;
        loop {
            let step = {
                let (v1, v2) = m1060;
                let m1058 = v1;
                if m1058 == 0 {
                    Err::<(u64, Adt10), Adt10>(v2)
                } else {
                    let v3 = m1058 - 1;
                    Ok::<(u64, Adt10), Adt10>((v3, {
                        let c1059: Fn12 = v0.clone();
                        c1059.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m1060 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f104(v0: Adt10) -> Adt10 {
    f127(v0)
}

pub fn f105(v0: (Adt9, Adt8)) -> (Adt9, Adt8) {
    (f128(f129({
        let (h1063, _) = v0.clone();
        h1063
    }, {
        let (_, h1064) = v0.clone();
        h1064
    }), {
        let (_, h1065) = v0.clone();
        h1065
    }), f130({
        let (_, h1066) = v0;
        h1066
    }))
}

pub fn f106(v0: Bytes, v1: u64) -> u32 {
    let m1069 = {
        let a1067 = v0;
        let a1068 = v1;
        index_bytes(a1067, a1068)
    };
    match m1069 {
        None => {
            0u32
        }
        Some(v2) => {
            let m1071 = {
                let a1070 = v2;
                fixed_u32::convert(i128::from(a1070))
            };
            match m1071 {
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

pub fn f107(v0: u32, v1: u32) -> u32 {
    let m1074 = {
        let a1072 = v0;
        let a1073 = v1;
        fixed_u32::shift_left(a1072, a1073)
    };
    match m1074 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f108(v0: Adt1, v1: Adt1) -> Adt8 {
    Adt8::C0(match v0.clone() {
        Adt1::C0(h1075, _, _, _, _, _, _, _) => {
            h1075
        }
    }, match v0.clone() {
        Adt1::C0(_, h1076, _, _, _, _, _, _) => {
            h1076
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h1077, _, _, _, _, _) => {
            h1077
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h1078, _, _, _, _) => {
            h1078
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h1079, _, _, _) => {
            h1079
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, h1080, _, _) => {
            h1080
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h1081, _) => {
            h1081
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, _, _, _, h1082) => {
            h1082
        }
    }, match v1.clone() {
        Adt1::C0(h1083, _, _, _, _, _, _, _) => {
            h1083
        }
    }, match v1.clone() {
        Adt1::C0(_, h1084, _, _, _, _, _, _) => {
            h1084
        }
    }, match v1.clone() {
        Adt1::C0(_, _, h1085, _, _, _, _, _) => {
            h1085
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, h1086, _, _, _, _) => {
            h1086
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, h1087, _, _, _) => {
            h1087
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, h1088, _, _) => {
            h1088
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, h1089, _) => {
            h1089
        }
    }, match v1 {
        Adt1::C0(_, _, _, _, _, _, _, h1090) => {
            h1090
        }
    })
}

pub fn f109(v0: u64) -> u32 {
    let a1091 = v0;
    let a1092 = 32u64;
    if nat_lt(a1091, a1092) {
        let a1093 = v0;
        let a1094 = 16u64;
        if nat_lt(a1093, a1094) {
            let a1095 = v0;
            let a1096 = 8u64;
            if nat_lt(a1095, a1096) {
                let a1097 = v0;
                let a1098 = 4u64;
                if nat_lt(a1097, a1098) {
                    let a1099 = v0;
                    let a1100 = 2u64;
                    if nat_lt(a1099, a1100) {
                        let a1101 = v0;
                        let a1102 = 1u64;
                        if nat_lt(a1101, a1102) {
                            1116352408u32
                        } else {
                            1899447441u32
                        }
                    } else {
                        let a1103 = v0;
                        let a1104 = 3u64;
                        if nat_lt(a1103, a1104) {
                            3049323471u32
                        } else {
                            3921009573u32
                        }
                    }
                } else {
                    let a1105 = v0;
                    let a1106 = 6u64;
                    if nat_lt(a1105, a1106) {
                        let a1107 = v0;
                        let a1108 = 5u64;
                        if nat_lt(a1107, a1108) {
                            961987163u32
                        } else {
                            1508970993u32
                        }
                    } else {
                        let a1109 = v0;
                        let a1110 = 7u64;
                        if nat_lt(a1109, a1110) {
                            2453635748u32
                        } else {
                            2870763221u32
                        }
                    }
                }
            } else {
                let a1111 = v0;
                let a1112 = 12u64;
                if nat_lt(a1111, a1112) {
                    let a1113 = v0;
                    let a1114 = 10u64;
                    if nat_lt(a1113, a1114) {
                        let a1115 = v0;
                        let a1116 = 9u64;
                        if nat_lt(a1115, a1116) {
                            3624381080u32
                        } else {
                            310598401u32
                        }
                    } else {
                        let a1117 = v0;
                        let a1118 = 11u64;
                        if nat_lt(a1117, a1118) {
                            607225278u32
                        } else {
                            1426881987u32
                        }
                    }
                } else {
                    let a1119 = v0;
                    let a1120 = 14u64;
                    if nat_lt(a1119, a1120) {
                        let a1121 = v0;
                        let a1122 = 13u64;
                        if nat_lt(a1121, a1122) {
                            1925078388u32
                        } else {
                            2162078206u32
                        }
                    } else {
                        let a1123 = v0;
                        let a1124 = 15u64;
                        if nat_lt(a1123, a1124) {
                            2614888103u32
                        } else {
                            3248222580u32
                        }
                    }
                }
            }
        } else {
            let a1125 = v0;
            let a1126 = 24u64;
            if nat_lt(a1125, a1126) {
                let a1127 = v0;
                let a1128 = 20u64;
                if nat_lt(a1127, a1128) {
                    let a1129 = v0;
                    let a1130 = 18u64;
                    if nat_lt(a1129, a1130) {
                        let a1131 = v0;
                        let a1132 = 17u64;
                        if nat_lt(a1131, a1132) {
                            3835390401u32
                        } else {
                            4022224774u32
                        }
                    } else {
                        let a1133 = v0;
                        let a1134 = 19u64;
                        if nat_lt(a1133, a1134) {
                            264347078u32
                        } else {
                            604807628u32
                        }
                    }
                } else {
                    let a1135 = v0;
                    let a1136 = 22u64;
                    if nat_lt(a1135, a1136) {
                        let a1137 = v0;
                        let a1138 = 21u64;
                        if nat_lt(a1137, a1138) {
                            770255983u32
                        } else {
                            1249150122u32
                        }
                    } else {
                        let a1139 = v0;
                        let a1140 = 23u64;
                        if nat_lt(a1139, a1140) {
                            1555081692u32
                        } else {
                            1996064986u32
                        }
                    }
                }
            } else {
                let a1141 = v0;
                let a1142 = 28u64;
                if nat_lt(a1141, a1142) {
                    let a1143 = v0;
                    let a1144 = 26u64;
                    if nat_lt(a1143, a1144) {
                        let a1145 = v0;
                        let a1146 = 25u64;
                        if nat_lt(a1145, a1146) {
                            2554220882u32
                        } else {
                            2821834349u32
                        }
                    } else {
                        let a1147 = v0;
                        let a1148 = 27u64;
                        if nat_lt(a1147, a1148) {
                            2952996808u32
                        } else {
                            3210313671u32
                        }
                    }
                } else {
                    let a1149 = v0;
                    let a1150 = 30u64;
                    if nat_lt(a1149, a1150) {
                        let a1151 = v0;
                        let a1152 = 29u64;
                        if nat_lt(a1151, a1152) {
                            3336571891u32
                        } else {
                            3584528711u32
                        }
                    } else {
                        let a1153 = v0;
                        let a1154 = 31u64;
                        if nat_lt(a1153, a1154) {
                            113926993u32
                        } else {
                            338241895u32
                        }
                    }
                }
            }
        }
    } else {
        let a1155 = v0;
        let a1156 = 48u64;
        if nat_lt(a1155, a1156) {
            let a1157 = v0;
            let a1158 = 40u64;
            if nat_lt(a1157, a1158) {
                let a1159 = v0;
                let a1160 = 36u64;
                if nat_lt(a1159, a1160) {
                    let a1161 = v0;
                    let a1162 = 34u64;
                    if nat_lt(a1161, a1162) {
                        let a1163 = v0;
                        let a1164 = 33u64;
                        if nat_lt(a1163, a1164) {
                            666307205u32
                        } else {
                            773529912u32
                        }
                    } else {
                        let a1165 = v0;
                        let a1166 = 35u64;
                        if nat_lt(a1165, a1166) {
                            1294757372u32
                        } else {
                            1396182291u32
                        }
                    }
                } else {
                    let a1167 = v0;
                    let a1168 = 38u64;
                    if nat_lt(a1167, a1168) {
                        let a1169 = v0;
                        let a1170 = 37u64;
                        if nat_lt(a1169, a1170) {
                            1695183700u32
                        } else {
                            1986661051u32
                        }
                    } else {
                        let a1171 = v0;
                        let a1172 = 39u64;
                        if nat_lt(a1171, a1172) {
                            2177026350u32
                        } else {
                            2456956037u32
                        }
                    }
                }
            } else {
                let a1173 = v0;
                let a1174 = 44u64;
                if nat_lt(a1173, a1174) {
                    let a1175 = v0;
                    let a1176 = 42u64;
                    if nat_lt(a1175, a1176) {
                        let a1177 = v0;
                        let a1178 = 41u64;
                        if nat_lt(a1177, a1178) {
                            2730485921u32
                        } else {
                            2820302411u32
                        }
                    } else {
                        let a1179 = v0;
                        let a1180 = 43u64;
                        if nat_lt(a1179, a1180) {
                            3259730800u32
                        } else {
                            3345764771u32
                        }
                    }
                } else {
                    let a1181 = v0;
                    let a1182 = 46u64;
                    if nat_lt(a1181, a1182) {
                        let a1183 = v0;
                        let a1184 = 45u64;
                        if nat_lt(a1183, a1184) {
                            3516065817u32
                        } else {
                            3600352804u32
                        }
                    } else {
                        let a1185 = v0;
                        let a1186 = 47u64;
                        if nat_lt(a1185, a1186) {
                            4094571909u32
                        } else {
                            275423344u32
                        }
                    }
                }
            }
        } else {
            let a1187 = v0;
            let a1188 = 56u64;
            if nat_lt(a1187, a1188) {
                let a1189 = v0;
                let a1190 = 52u64;
                if nat_lt(a1189, a1190) {
                    let a1191 = v0;
                    let a1192 = 50u64;
                    if nat_lt(a1191, a1192) {
                        let a1193 = v0;
                        let a1194 = 49u64;
                        if nat_lt(a1193, a1194) {
                            430227734u32
                        } else {
                            506948616u32
                        }
                    } else {
                        let a1195 = v0;
                        let a1196 = 51u64;
                        if nat_lt(a1195, a1196) {
                            659060556u32
                        } else {
                            883997877u32
                        }
                    }
                } else {
                    let a1197 = v0;
                    let a1198 = 54u64;
                    if nat_lt(a1197, a1198) {
                        let a1199 = v0;
                        let a1200 = 53u64;
                        if nat_lt(a1199, a1200) {
                            958139571u32
                        } else {
                            1322822218u32
                        }
                    } else {
                        let a1201 = v0;
                        let a1202 = 55u64;
                        if nat_lt(a1201, a1202) {
                            1537002063u32
                        } else {
                            1747873779u32
                        }
                    }
                }
            } else {
                let a1203 = v0;
                let a1204 = 60u64;
                if nat_lt(a1203, a1204) {
                    let a1205 = v0;
                    let a1206 = 58u64;
                    if nat_lt(a1205, a1206) {
                        let a1207 = v0;
                        let a1208 = 57u64;
                        if nat_lt(a1207, a1208) {
                            1955562222u32
                        } else {
                            2024104815u32
                        }
                    } else {
                        let a1209 = v0;
                        let a1210 = 59u64;
                        if nat_lt(a1209, a1210) {
                            2227730452u32
                        } else {
                            2361852424u32
                        }
                    }
                } else {
                    let a1211 = v0;
                    let a1212 = 62u64;
                    if nat_lt(a1211, a1212) {
                        let a1213 = v0;
                        let a1214 = 61u64;
                        if nat_lt(a1213, a1214) {
                            2428436474u32
                        } else {
                            2756734187u32
                        }
                    } else {
                        let a1215 = v0;
                        let a1216 = 63u64;
                        if nat_lt(a1215, a1216) {
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

pub fn f110(v0: u32) -> u32 {
    let a1219 = {
        let a1217 = f131(v0, 6u32, 26u32);
        let a1218 = f131(v0, 11u32, 21u32);
        fixed_u32::bit_xor(a1217, a1218)
    };
    let a1220 = f131(v0, 25u32, 7u32);
    fixed_u32::bit_xor(a1219, a1220)
}

pub fn f111(v0: u32, v1: u32, v2: u32) -> u32 {
    let a1226 = {
        let a1221 = v0;
        let a1222 = v1;
        fixed_u32::bit_and(a1221, a1222)
    };
    let a1227 = {
        let a1224 = {
            let a1223 = v0;
            fixed_u32::bit_not(a1223)
        };
        let a1225 = v2;
        fixed_u32::bit_and(a1224, a1225)
    };
    fixed_u32::bit_xor(a1226, a1227)
}

pub fn f112(v0: u32) -> u32 {
    let a1230 = {
        let a1228 = f131(v0, 2u32, 30u32);
        let a1229 = f131(v0, 13u32, 19u32);
        fixed_u32::bit_xor(a1228, a1229)
    };
    let a1231 = f131(v0, 22u32, 10u32);
    fixed_u32::bit_xor(a1230, a1231)
}

pub fn f113(v0: u32, v1: u32, v2: u32) -> u32 {
    let a1240 = {
        let a1236 = {
            let a1232 = v0;
            let a1233 = v1;
            fixed_u32::bit_and(a1232, a1233)
        };
        let a1237 = {
            let a1234 = v0;
            let a1235 = v2;
            fixed_u32::bit_and(a1234, a1235)
        };
        fixed_u32::bit_xor(a1236, a1237)
    };
    let a1241 = {
        let a1238 = v1;
        let a1239 = v2;
        fixed_u32::bit_and(a1238, a1239)
    };
    fixed_u32::bit_xor(a1240, a1241)
}

pub fn f114(v0: Adt4) -> Adt4 {
    Adt4::C0(match v0.clone() {
        Adt4::C0(_, h1242, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1242
        }
    }, match v0.clone() {
        Adt4::C0(_, _, h1243, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1243
        }
    }, match v0.clone() {
        Adt4::C0(_, _, _, h1244, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1244
        }
    }, match v0.clone() {
        Adt4::C0(_, _, _, _, h1245, _, _, _, _, _, _, _, _, _, _, _) => {
            h1245
        }
    }, match v0.clone() {
        Adt4::C0(_, _, _, _, _, h1246, _, _, _, _, _, _, _, _, _, _) => {
            h1246
        }
    }, match v0.clone() {
        Adt4::C0(_, _, _, _, _, _, h1247, _, _, _, _, _, _, _, _, _) => {
            h1247
        }
    }, match v0.clone() {
        Adt4::C0(_, _, _, _, _, _, _, h1248, _, _, _, _, _, _, _, _) => {
            h1248
        }
    }, match v0.clone() {
        Adt4::C0(_, _, _, _, _, _, _, _, h1249, _, _, _, _, _, _, _) => {
            h1249
        }
    }, match v0.clone() {
        Adt4::C0(_, _, _, _, _, _, _, _, _, h1250, _, _, _, _, _, _) => {
            h1250
        }
    }, match v0.clone() {
        Adt4::C0(_, _, _, _, _, _, _, _, _, _, h1251, _, _, _, _, _) => {
            h1251
        }
    }, match v0.clone() {
        Adt4::C0(_, _, _, _, _, _, _, _, _, _, _, h1252, _, _, _, _) => {
            h1252
        }
    }, match v0.clone() {
        Adt4::C0(_, _, _, _, _, _, _, _, _, _, _, _, h1253, _, _, _) => {
            h1253
        }
    }, match v0.clone() {
        Adt4::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1254, _, _) => {
            h1254
        }
    }, match v0.clone() {
        Adt4::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1255, _) => {
            h1255
        }
    }, match v0.clone() {
        Adt4::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1256) => {
            h1256
        }
    }, f74(f132(match v0.clone() {
        Adt4::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1257, _) => {
            h1257
        }
    }), f74(match v0.clone() {
        Adt4::C0(_, _, _, _, _, _, _, _, _, h1258, _, _, _, _, _, _) => {
            h1258
        }
    }, f74(f133(match v0.clone() {
        Adt4::C0(_, h1259, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1259
        }
    }), match v0 {
        Adt4::C0(h1260, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1260
        }
    }))))
}

pub fn f115(v0: (u64, Adt3)) -> R<(u64, Adt3)> {
    f134(v0)
}

pub fn f116(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> u64 {
    let a1273 = f99(v7, 56u32);
    let a1274 = {
        let a1271 = f99(v6, 48u32);
        let a1272 = {
            let a1269 = f99(v5, 40u32);
            let a1270 = {
                let a1267 = f99(v4, 32u32);
                let a1268 = {
                    let a1265 = f99(v3, 24u32);
                    let a1266 = {
                        let a1263 = f99(v2, 16u32);
                        let a1264 = {
                            let a1261 = f99(v1, 8u32);
                            let a1262 = v0;
                            fixed_u64::bit_or(a1261, a1262)
                        };
                        fixed_u64::bit_or(a1263, a1264)
                    };
                    fixed_u64::bit_or(a1265, a1266)
                };
                fixed_u64::bit_or(a1267, a1268)
            };
            fixed_u64::bit_or(a1269, a1270)
        };
        fixed_u64::bit_or(a1271, a1272)
    };
    fixed_u64::bit_or(a1273, a1274)
}

pub fn f117(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    let v5: u64 = {
        let a1275 = v4;
        let a1276 = v1;
        if nat_lt(a1275, a1276) {
            f126(v0, v4)
        } else {
            let a1277 = v4;
            let a1278 = v1;
            if nat_eq(a1277, a1278) {
                v3
            } else {
                0u64
            }
        }
    };
    let a1281 = {
        let a1279 = v4;
        let a1280 = 1u64;
        nat_add(a1279, a1280)?
    };
    let a1282 = v2;
    if nat_eq(a1281, a1282) {
        let a1283 = v5;
        let a1284 = 128u64;
        Ok(fixed_u64::bit_or(a1283, a1284))
    } else {
        Ok(v5)
    }
}

pub fn f118(v0: u64) -> u64 {
    let a1285 = v0;
    let a1286 = 40u64;
    if nat_lt(a1285, a1286) {
        let a1287 = v0;
        let a1288 = 20u64;
        if nat_lt(a1287, a1288) {
            let a1289 = v0;
            let a1290 = 10u64;
            if nat_lt(a1289, a1290) {
                let a1291 = v0;
                let a1292 = 5u64;
                if nat_lt(a1291, a1292) {
                    let a1293 = v0;
                    let a1294 = 2u64;
                    if nat_lt(a1293, a1294) {
                        let a1295 = v0;
                        let a1296 = 1u64;
                        if nat_lt(a1295, a1296) {
                            4794697086780616226u64
                        } else {
                            8158064640168781261u64
                        }
                    } else {
                        let a1297 = v0;
                        let a1298 = 3u64;
                        if nat_lt(a1297, a1298) {
                            13096744586834688815u64
                        } else {
                            let a1299 = v0;
                            let a1300 = 4u64;
                            if nat_lt(a1299, a1300) {
                                16840607885511220156u64
                            } else {
                                4131703408338449720u64
                            }
                        }
                    }
                } else {
                    let a1301 = v0;
                    let a1302 = 7u64;
                    if nat_lt(a1301, a1302) {
                        let a1303 = v0;
                        let a1304 = 6u64;
                        if nat_lt(a1303, a1304) {
                            6480981068601479193u64
                        } else {
                            10538285296894168987u64
                        }
                    } else {
                        let a1305 = v0;
                        let a1306 = 8u64;
                        if nat_lt(a1305, a1306) {
                            12329834152419229976u64
                        } else {
                            let a1307 = v0;
                            let a1308 = 9u64;
                            if nat_lt(a1307, a1308) {
                                15566598209576043074u64
                            } else {
                                1334009975649890238u64
                            }
                        }
                    }
                }
            } else {
                let a1309 = v0;
                let a1310 = 15u64;
                if nat_lt(a1309, a1310) {
                    let a1311 = v0;
                    let a1312 = 12u64;
                    if nat_lt(a1311, a1312) {
                        let a1313 = v0;
                        let a1314 = 11u64;
                        if nat_lt(a1313, a1314) {
                            2608012711638119052u64
                        } else {
                            6128411473006802146u64
                        }
                    } else {
                        let a1315 = v0;
                        let a1316 = 13u64;
                        if nat_lt(a1315, a1316) {
                            8268148722764581231u64
                        } else {
                            let a1317 = v0;
                            let a1318 = 14u64;
                            if nat_lt(a1317, a1318) {
                                9286055187155687089u64
                            } else {
                                11230858885718282805u64
                            }
                        }
                    }
                } else {
                    let a1319 = v0;
                    let a1320 = 17u64;
                    if nat_lt(a1319, a1320) {
                        let a1321 = v0;
                        let a1322 = 16u64;
                        if nat_lt(a1321, a1322) {
                            13951009754708518548u64
                        } else {
                            16472876342353939154u64
                        }
                    } else {
                        let a1323 = v0;
                        let a1324 = 18u64;
                        if nat_lt(a1323, a1324) {
                            17275323862435702243u64
                        } else {
                            let a1325 = v0;
                            let a1326 = 19u64;
                            if nat_lt(a1325, a1326) {
                                1135362057144423861u64
                            } else {
                                2597628984639134821u64
                            }
                        }
                    }
                }
            }
        } else {
            let a1327 = v0;
            let a1328 = 30u64;
            if nat_lt(a1327, a1328) {
                let a1329 = v0;
                let a1330 = 25u64;
                if nat_lt(a1329, a1330) {
                    let a1331 = v0;
                    let a1332 = 22u64;
                    if nat_lt(a1331, a1332) {
                        let a1333 = v0;
                        let a1334 = 21u64;
                        if nat_lt(a1333, a1334) {
                            3308224258029322869u64
                        } else {
                            5365058923640841347u64
                        }
                    } else {
                        let a1335 = v0;
                        let a1336 = 23u64;
                        if nat_lt(a1335, a1336) {
                            6679025012923562964u64
                        } else {
                            let a1337 = v0;
                            let a1338 = 24u64;
                            if nat_lt(a1337, a1338) {
                                8573033837759648693u64
                            } else {
                                10970295158949994411u64
                            }
                        }
                    }
                } else {
                    let a1339 = v0;
                    let a1340 = 27u64;
                    if nat_lt(a1339, a1340) {
                        let a1341 = v0;
                        let a1342 = 26u64;
                        if nat_lt(a1341, a1342) {
                            12119686244451234320u64
                        } else {
                            12683024718118986047u64
                        }
                    } else {
                        let a1343 = v0;
                        let a1344 = 28u64;
                        if nat_lt(a1343, a1344) {
                            13788192230050041572u64
                        } else {
                            let a1345 = v0;
                            let a1346 = 29u64;
                            if nat_lt(a1345, a1346) {
                                14330467153632333762u64
                            } else {
                                15395433587784984357u64
                            }
                        }
                    }
                }
            } else {
                let a1347 = v0;
                let a1348 = 35u64;
                if nat_lt(a1347, a1348) {
                    let a1349 = v0;
                    let a1350 = 32u64;
                    if nat_lt(a1349, a1350) {
                        let a1351 = v0;
                        let a1352 = 31u64;
                        if nat_lt(a1351, a1352) {
                            489312712824947311u64
                        } else {
                            1452737877330783856u64
                        }
                    } else {
                        let a1353 = v0;
                        let a1354 = 33u64;
                        if nat_lt(a1353, a1354) {
                            2861767655752347644u64
                        } else {
                            let a1355 = v0;
                            let a1356 = 34u64;
                            if nat_lt(a1355, a1356) {
                                3322285676063803686u64
                            } else {
                                5560940570517711597u64
                            }
                        }
                    }
                } else {
                    let a1357 = v0;
                    let a1358 = 37u64;
                    if nat_lt(a1357, a1358) {
                        let a1359 = v0;
                        let a1360 = 36u64;
                        if nat_lt(a1359, a1360) {
                            5996557281743188959u64
                        } else {
                            7280758554555802590u64
                        }
                    } else {
                        let a1361 = v0;
                        let a1362 = 38u64;
                        if nat_lt(a1361, a1362) {
                            8532644243296465576u64
                        } else {
                            let a1363 = v0;
                            let a1364 = 39u64;
                            if nat_lt(a1363, a1364) {
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
        let a1365 = v0;
        let a1366 = 60u64;
        if nat_lt(a1365, a1366) {
            let a1367 = v0;
            let a1368 = 50u64;
            if nat_lt(a1367, a1368) {
                let a1369 = v0;
                let a1370 = 45u64;
                if nat_lt(a1369, a1370) {
                    let a1371 = v0;
                    let a1372 = 42u64;
                    if nat_lt(a1371, a1372) {
                        let a1373 = v0;
                        let a1374 = 41u64;
                        if nat_lt(a1373, a1374) {
                            11727347734174303076u64
                        } else {
                            12113106623233404929u64
                        }
                    } else {
                        let a1375 = v0;
                        let a1376 = 43u64;
                        if nat_lt(a1375, a1376) {
                            14000437183269869457u64
                        } else {
                            let a1377 = v0;
                            let a1378 = 44u64;
                            if nat_lt(a1377, a1378) {
                                14369950271660146224u64
                            } else {
                                15101387698204529176u64
                            }
                        }
                    }
                } else {
                    let a1379 = v0;
                    let a1380 = 47u64;
                    if nat_lt(a1379, a1380) {
                        let a1381 = v0;
                        let a1382 = 46u64;
                        if nat_lt(a1381, a1382) {
                            15463397548674623760u64
                        } else {
                            17586052441742319658u64
                        }
                    } else {
                        let a1383 = v0;
                        let a1384 = 48u64;
                        if nat_lt(a1383, a1384) {
                            1182934255886127544u64
                        } else {
                            let a1385 = v0;
                            let a1386 = 49u64;
                            if nat_lt(a1385, a1386) {
                                1847814050463011016u64
                            } else {
                                2177327727835720531u64
                            }
                        }
                    }
                }
            } else {
                let a1387 = v0;
                let a1388 = 55u64;
                if nat_lt(a1387, a1388) {
                    let a1389 = v0;
                    let a1390 = 52u64;
                    if nat_lt(a1389, a1390) {
                        let a1391 = v0;
                        let a1392 = 51u64;
                        if nat_lt(a1391, a1392) {
                            2830643537854262169u64
                        } else {
                            3796741975233480872u64
                        }
                    } else {
                        let a1393 = v0;
                        let a1394 = 53u64;
                        if nat_lt(a1393, a1394) {
                            4115178125766777443u64
                        } else {
                            let a1395 = v0;
                            let a1396 = 54u64;
                            if nat_lt(a1395, a1396) {
                                5681478168544905931u64
                            } else {
                                6601373596472566643u64
                            }
                        }
                    }
                } else {
                    let a1397 = v0;
                    let a1398 = 57u64;
                    if nat_lt(a1397, a1398) {
                        let a1399 = v0;
                        let a1400 = 56u64;
                        if nat_lt(a1399, a1400) {
                            7507060721942968483u64
                        } else {
                            8399075790359081724u64
                        }
                    } else {
                        let a1401 = v0;
                        let a1402 = 58u64;
                        if nat_lt(a1401, a1402) {
                            8693463985226723168u64
                        } else {
                            let a1403 = v0;
                            let a1404 = 59u64;
                            if nat_lt(a1403, a1404) {
                                9568029438360202098u64
                            } else {
                                10144078919501101548u64
                            }
                        }
                    }
                }
            }
        } else {
            let a1405 = v0;
            let a1406 = 70u64;
            if nat_lt(a1405, a1406) {
                let a1407 = v0;
                let a1408 = 65u64;
                if nat_lt(a1407, a1408) {
                    let a1409 = v0;
                    let a1410 = 62u64;
                    if nat_lt(a1409, a1410) {
                        let a1411 = v0;
                        let a1412 = 61u64;
                        if nat_lt(a1411, a1412) {
                            10430055236837252648u64
                        } else {
                            11840083180663258601u64
                        }
                    } else {
                        let a1413 = v0;
                        let a1414 = 63u64;
                        if nat_lt(a1413, a1414) {
                            13761210420658862357u64
                        } else {
                            let a1415 = v0;
                            let a1416 = 64u64;
                            if nat_lt(a1415, a1416) {
                                14299343276471374635u64
                            } else {
                                14566680578165727644u64
                            }
                        }
                    }
                } else {
                    let a1417 = v0;
                    let a1418 = 67u64;
                    if nat_lt(a1417, a1418) {
                        let a1419 = v0;
                        let a1420 = 66u64;
                        if nat_lt(a1419, a1420) {
                            15097957966210449927u64
                        } else {
                            16922976911328602910u64
                        }
                    } else {
                        let a1421 = v0;
                        let a1422 = 68u64;
                        if nat_lt(a1421, a1422) {
                            17689382322260857208u64
                        } else {
                            let a1423 = v0;
                            let a1424 = 69u64;
                            if nat_lt(a1423, a1424) {
                                500013540394364858u64
                            } else {
                                748580250866718886u64
                            }
                        }
                    }
                }
            } else {
                let a1425 = v0;
                let a1426 = 75u64;
                if nat_lt(a1425, a1426) {
                    let a1427 = v0;
                    let a1428 = 72u64;
                    if nat_lt(a1427, a1428) {
                        let a1429 = v0;
                        let a1430 = 71u64;
                        if nat_lt(a1429, a1430) {
                            1242879168328830382u64
                        } else {
                            1977374033974150939u64
                        }
                    } else {
                        let a1431 = v0;
                        let a1432 = 73u64;
                        if nat_lt(a1431, a1432) {
                            2944078676154940804u64
                        } else {
                            let a1433 = v0;
                            let a1434 = 74u64;
                            if nat_lt(a1433, a1434) {
                                3659926193048069267u64
                            } else {
                                4368137639120453308u64
                            }
                        }
                    }
                } else {
                    let a1435 = v0;
                    let a1436 = 77u64;
                    if nat_lt(a1435, a1436) {
                        let a1437 = v0;
                        let a1438 = 76u64;
                        if nat_lt(a1437, a1438) {
                            4836135668995329356u64
                        } else {
                            5532061633213252278u64
                        }
                    } else {
                        let a1439 = v0;
                        let a1440 = 78u64;
                        if nat_lt(a1439, a1440) {
                            6448918945643986474u64
                        } else {
                            let a1441 = v0;
                            let a1442 = 79u64;
                            if nat_lt(a1441, a1442) {
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

pub fn f119(v0: u64) -> u64 {
    let a1445 = {
        let a1443 = f135(v0, 14u32, 50u32);
        let a1444 = f135(v0, 18u32, 46u32);
        fixed_u64::bit_xor(a1443, a1444)
    };
    let a1446 = f135(v0, 41u32, 23u32);
    fixed_u64::bit_xor(a1445, a1446)
}

pub fn f120(v0: u64, v1: u64, v2: u64) -> u64 {
    let a1452 = {
        let a1447 = v0;
        let a1448 = v1;
        fixed_u64::bit_and(a1447, a1448)
    };
    let a1453 = {
        let a1450 = {
            let a1449 = v0;
            fixed_u64::bit_not(a1449)
        };
        let a1451 = v2;
        fixed_u64::bit_and(a1450, a1451)
    };
    fixed_u64::bit_xor(a1452, a1453)
}

pub fn f121(v0: u64) -> u64 {
    let a1456 = {
        let a1454 = f135(v0, 28u32, 36u32);
        let a1455 = f135(v0, 34u32, 30u32);
        fixed_u64::bit_xor(a1454, a1455)
    };
    let a1457 = f135(v0, 39u32, 25u32);
    fixed_u64::bit_xor(a1456, a1457)
}

pub fn f122(v0: u64, v1: u64, v2: u64) -> u64 {
    let a1466 = {
        let a1462 = {
            let a1458 = v0;
            let a1459 = v1;
            fixed_u64::bit_and(a1458, a1459)
        };
        let a1463 = {
            let a1460 = v0;
            let a1461 = v2;
            fixed_u64::bit_and(a1460, a1461)
        };
        fixed_u64::bit_xor(a1462, a1463)
    };
    let a1467 = {
        let a1464 = v1;
        let a1465 = v2;
        fixed_u64::bit_and(a1464, a1465)
    };
    fixed_u64::bit_xor(a1466, a1467)
}

pub fn f123(v0: Adt6) -> Adt6 {
    Adt6::C0(match v0.clone() {
        Adt6::C0(_, h1468, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1468
        }
    }, match v0.clone() {
        Adt6::C0(_, _, h1469, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1469
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, h1470, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1470
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, h1471, _, _, _, _, _, _, _, _, _, _, _) => {
            h1471
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, h1472, _, _, _, _, _, _, _, _, _, _) => {
            h1472
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, h1473, _, _, _, _, _, _, _, _, _) => {
            h1473
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, h1474, _, _, _, _, _, _, _, _) => {
            h1474
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, h1475, _, _, _, _, _, _, _) => {
            h1475
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, h1476, _, _, _, _, _, _) => {
            h1476
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, _, h1477, _, _, _, _, _) => {
            h1477
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, h1478, _, _, _, _) => {
            h1478
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, h1479, _, _, _) => {
            h1479
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1480, _, _) => {
            h1480
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1481, _) => {
            h1481
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1482) => {
            h1482
        }
    }, f79(f136(match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1483, _) => {
            h1483
        }
    }), f79(match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, h1484, _, _, _, _, _, _) => {
            h1484
        }
    }, f79(f137(match v0.clone() {
        Adt6::C0(_, h1485, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1485
        }
    }), match v0 {
        Adt6::C0(h1486, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1486
        }
    }))))
}

pub fn f124(v0: Fn13, p1490: u64, p1491: (u64, (u64, u64))) -> (u64, (u64, u64)) {
    let m1489 = (p1490, p1491);
    {
        let mut m1489 = m1489;
        loop {
            let step = {
                let (v1, v2) = m1489;
                let m1487 = v1;
                if m1487 == 0 {
                    Err::<(u64, (u64, (u64, u64))), (u64, (u64, u64))>(v2)
                } else {
                    let v3 = m1487 - 1;
                    Ok::<(u64, (u64, (u64, u64))), (u64, (u64, u64))>((v3, {
                        let c1488: Fn13 = v0.clone();
                        c1488.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m1489 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f125(v0: (u64, (u64, u64))) -> (u64, (u64, u64)) {
    let v1: u64 = {
        let (h1492, _) = v0;
        h1492
    };
    let v2: u64 = {
        let (h1493, _) = {
            let (_, h1494) = v0;
            h1494
        };
        h1493
    };
    let v3: u64 = {
        let (_, h1495) = {
            let (_, h1496) = v0;
            h1496
        };
        h1495
    };
    ({
        let a1497 = v1;
        let a1498 = 2u64;
        let a1499 = 0u64;
        nat_quot(a1497, a1498, a1499)
    }, (f99(v2, 1u32), {
        let a1503 = {
            let a1500 = v1;
            let a1501 = 2u64;
            let a1502 = 0u64;
            nat_rem(a1500, a1501, a1502)
        };
        let a1504 = 1u64;
        if nat_eq(a1503, a1504) {
            let a1505 = v3;
            let a1506 = v2;
            fixed_u64::bit_or(a1505, a1506)
        } else {
            v3
        }
    }))
}

pub fn f126(v0: Bytes, v1: u64) -> u64 {
    let m1509 = {
        let a1507 = v0;
        let a1508 = v1;
        index_bytes(a1507, a1508)
    };
    match m1509 {
        None => {
            0u64
        }
        Some(v2) => {
            let m1511 = {
                let a1510 = v2;
                fixed_u64::convert(i128::from(a1510))
            };
            match m1511 {
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

pub fn f127(v0: Adt10) -> Adt10 {
    let a1516 = {
        let a1513 = match v0.clone() {
            Adt10::C0(_, h1512, _) => {
                h1512
            }
        };
        let a1514 = 2u64;
        let a1515 = 0u64;
        nat_rem(a1513, a1514, a1515)
    };
    let a1517 = 0u64;
    if nat_eq(a1516, a1517) {
        let m1519 = match v0.clone() {
            Adt10::C0(_, _, h1518) => {
                h1518
            }
        };
        match m1519.uncons() {
            None => {
                v0
            }
            Some((v1, v2)) => {
                Adt10::C0(f90(v1, match v0.clone() {
                    Adt10::C0(h1520, _, _) => {
                        h1520
                    }
                }, false), {
                    let a1522 = match v0 {
                        Adt10::C0(_, h1521, _) => {
                            h1521
                        }
                    };
                    let a1523 = 2u64;
                    let a1524 = 0u64;
                    nat_quot(a1522, a1523, a1524)
                }, v2)
            }
        }
    } else {
        v0
    }
}

pub fn f128(v0: Adt9, v1: Adt8) -> Adt9 {
    let v2: Adt11 = f138(match v0.clone() {
        Adt9::C0(h1525, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1525
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, h1526, _, _, _, _, _, _, _, _, _, _) => {
            h1526
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, _, _, h1527, _, _, _, _, _) => {
            h1527
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1528) => {
            h1528
        }
    }, match v1.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, h1529, _, _, _, _, _, _, _) => {
            h1529
        }
    }, match v1.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, h1530, _, _, _, _, _, _) => {
            h1530
        }
    });
    let v3: Adt11 = f138(match v0.clone() {
        Adt9::C0(_, h1531, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1531
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, h1532, _, _, _, _, _, _, _, _, _) => {
            h1532
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, _, _, _, h1533, _, _, _, _) => {
            h1533
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, _, _, _, _, h1534, _, _, _) => {
            h1534
        }
    }, match v1.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, h1535, _, _, _, _, _) => {
            h1535
        }
    }, match v1.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, h1536, _, _, _, _) => {
            h1536
        }
    });
    let v4: Adt11 = f138(match v0.clone() {
        Adt9::C0(_, _, h1537, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1537
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, h1538, _, _, _, _, _, _, _, _) => {
            h1538
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, h1539, _, _, _, _, _, _, _) => {
            h1539
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1540, _, _) => {
            h1540
        }
    }, match v1.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, h1541, _, _, _) => {
            h1541
        }
    }, match v1.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1542, _, _) => {
            h1542
        }
    });
    let v5: Adt11 = f138(match v0.clone() {
        Adt9::C0(_, _, _, h1543, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1543
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, h1544, _, _, _, _, _, _, _, _, _, _, _) => {
            h1544
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, _, h1545, _, _, _, _, _, _) => {
            h1545
        }
    }, match v0 {
        Adt9::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1546, _) => {
            h1546
        }
    }, match v1.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1547, _) => {
            h1547
        }
    }, match v1 {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1548) => {
            h1548
        }
    });
    Adt9::C0(match v2.clone() {
        Adt11::C0(h1549, _, _, _) => {
            h1549
        }
    }, match v3.clone() {
        Adt11::C0(h1550, _, _, _) => {
            h1550
        }
    }, match v4.clone() {
        Adt11::C0(h1551, _, _, _) => {
            h1551
        }
    }, match v5.clone() {
        Adt11::C0(h1552, _, _, _) => {
            h1552
        }
    }, match v5.clone() {
        Adt11::C0(_, h1553, _, _) => {
            h1553
        }
    }, match v2.clone() {
        Adt11::C0(_, h1554, _, _) => {
            h1554
        }
    }, match v3.clone() {
        Adt11::C0(_, h1555, _, _) => {
            h1555
        }
    }, match v4.clone() {
        Adt11::C0(_, h1556, _, _) => {
            h1556
        }
    }, match v4.clone() {
        Adt11::C0(_, _, h1557, _) => {
            h1557
        }
    }, match v5.clone() {
        Adt11::C0(_, _, h1558, _) => {
            h1558
        }
    }, match v2.clone() {
        Adt11::C0(_, _, h1559, _) => {
            h1559
        }
    }, match v3.clone() {
        Adt11::C0(_, _, h1560, _) => {
            h1560
        }
    }, match v3 {
        Adt11::C0(_, _, _, h1561) => {
            h1561
        }
    }, match v4 {
        Adt11::C0(_, _, _, h1562) => {
            h1562
        }
    }, match v5 {
        Adt11::C0(_, _, _, h1563) => {
            h1563
        }
    }, match v2 {
        Adt11::C0(_, _, _, h1564) => {
            h1564
        }
    })
}

pub fn f129(v0: Adt9, v1: Adt8) -> Adt9 {
    let v2: Adt11 = f138(match v0.clone() {
        Adt9::C0(h1565, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1565
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, h1566, _, _, _, _, _, _, _, _, _, _, _) => {
            h1566
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, h1567, _, _, _, _, _, _, _) => {
            h1567
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, _, _, _, _, h1568, _, _, _) => {
            h1568
        }
    }, match v1.clone() {
        Adt8::C0(h1569, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1569
        }
    }, match v1.clone() {
        Adt8::C0(_, h1570, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1570
        }
    });
    let v3: Adt11 = f138(match v0.clone() {
        Adt9::C0(_, h1571, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1571
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, h1572, _, _, _, _, _, _, _, _, _, _) => {
            h1572
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, _, h1573, _, _, _, _, _, _) => {
            h1573
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1574, _, _) => {
            h1574
        }
    }, match v1.clone() {
        Adt8::C0(_, _, h1575, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1575
        }
    }, match v1.clone() {
        Adt8::C0(_, _, _, h1576, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1576
        }
    });
    let v4: Adt11 = f138(match v0.clone() {
        Adt9::C0(_, _, h1577, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1577
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, h1578, _, _, _, _, _, _, _, _, _) => {
            h1578
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, _, _, h1579, _, _, _, _, _) => {
            h1579
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1580, _) => {
            h1580
        }
    }, match v1.clone() {
        Adt8::C0(_, _, _, _, h1581, _, _, _, _, _, _, _, _, _, _, _) => {
            h1581
        }
    }, match v1.clone() {
        Adt8::C0(_, _, _, _, _, h1582, _, _, _, _, _, _, _, _, _, _) => {
            h1582
        }
    });
    let v5: Adt11 = f138(match v0.clone() {
        Adt9::C0(_, _, _, h1583, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1583
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, h1584, _, _, _, _, _, _, _, _) => {
            h1584
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, _, _, _, h1585, _, _, _, _) => {
            h1585
        }
    }, match v0 {
        Adt9::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1586) => {
            h1586
        }
    }, match v1.clone() {
        Adt8::C0(_, _, _, _, _, _, h1587, _, _, _, _, _, _, _, _, _) => {
            h1587
        }
    }, match v1 {
        Adt8::C0(_, _, _, _, _, _, _, h1588, _, _, _, _, _, _, _, _) => {
            h1588
        }
    });
    Adt9::C0(match v2.clone() {
        Adt11::C0(h1589, _, _, _) => {
            h1589
        }
    }, match v3.clone() {
        Adt11::C0(h1590, _, _, _) => {
            h1590
        }
    }, match v4.clone() {
        Adt11::C0(h1591, _, _, _) => {
            h1591
        }
    }, match v5.clone() {
        Adt11::C0(h1592, _, _, _) => {
            h1592
        }
    }, match v2.clone() {
        Adt11::C0(_, h1593, _, _) => {
            h1593
        }
    }, match v3.clone() {
        Adt11::C0(_, h1594, _, _) => {
            h1594
        }
    }, match v4.clone() {
        Adt11::C0(_, h1595, _, _) => {
            h1595
        }
    }, match v5.clone() {
        Adt11::C0(_, h1596, _, _) => {
            h1596
        }
    }, match v2.clone() {
        Adt11::C0(_, _, h1597, _) => {
            h1597
        }
    }, match v3.clone() {
        Adt11::C0(_, _, h1598, _) => {
            h1598
        }
    }, match v4.clone() {
        Adt11::C0(_, _, h1599, _) => {
            h1599
        }
    }, match v5.clone() {
        Adt11::C0(_, _, h1600, _) => {
            h1600
        }
    }, match v2 {
        Adt11::C0(_, _, _, h1601) => {
            h1601
        }
    }, match v3 {
        Adt11::C0(_, _, _, h1602) => {
            h1602
        }
    }, match v4 {
        Adt11::C0(_, _, _, h1603) => {
            h1603
        }
    }, match v5 {
        Adt11::C0(_, _, _, h1604) => {
            h1604
        }
    })
}

pub fn f130(v0: Adt8) -> Adt8 {
    Adt8::C0(match v0.clone() {
        Adt8::C0(_, _, h1605, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1605
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, h1606, _, _, _, _, _, _, _, _, _) => {
            h1606
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, h1607, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1607
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, h1608, _, _, _, _, _) => {
            h1608
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, h1609, _, _, _, _, _, _, _, _) => {
            h1609
        }
    }, match v0.clone() {
        Adt8::C0(h1610, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1610
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, h1611, _, _, _, _, _, _, _, _, _, _, _) => {
            h1611
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1612, _, _) => {
            h1612
        }
    }, match v0.clone() {
        Adt8::C0(_, h1613, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1613
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, h1614, _, _, _, _) => {
            h1614
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, h1615, _, _, _) => {
            h1615
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, _, h1616, _, _, _, _, _, _, _, _, _, _) => {
            h1616
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, h1617, _, _, _, _, _, _) => {
            h1617
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1618, _) => {
            h1618
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1619) => {
            h1619
        }
    }, match v0 {
        Adt8::C0(_, _, _, _, _, _, _, _, h1620, _, _, _, _, _, _, _) => {
            h1620
        }
    })
}

pub fn f131(v0: u32, v1: u32, v2: u32) -> u32 {
    let a1621 = f53(v0, v1);
    let a1622 = f107(v0, v2);
    fixed_u32::bit_or(a1621, a1622)
}

pub fn f132(v0: u32) -> u32 {
    let a1625 = {
        let a1623 = f131(v0, 17u32, 15u32);
        let a1624 = f131(v0, 19u32, 13u32);
        fixed_u32::bit_xor(a1623, a1624)
    };
    let a1626 = f53(v0, 10u32);
    fixed_u32::bit_xor(a1625, a1626)
}

pub fn f133(v0: u32) -> u32 {
    let a1629 = {
        let a1627 = f131(v0, 7u32, 25u32);
        let a1628 = f131(v0, 18u32, 14u32);
        fixed_u32::bit_xor(a1627, a1628)
    };
    let a1630 = f53(v0, 3u32);
    fixed_u32::bit_xor(a1629, a1630)
}

pub fn f134(v0: (u64, Adt3)) -> R<(u64, Adt3)> {
    Ok(({
        let a1632 = {
            let (h1631, _) = v0.clone();
            h1631
        };
        let a1633 = 1u64;
        nat_add(a1632, a1633)?
    }, f139(f140(f141(f142({
        let (_, h1634) = v0;
        h1634
    }))), {
        let (h1635, _) = v0;
        h1635
    })))
}

pub fn f135(v0: u64, v1: u32, v2: u32) -> u64 {
    let a1636 = f62(v0, v1);
    let a1637 = f99(v0, v2);
    fixed_u64::bit_or(a1636, a1637)
}

pub fn f136(v0: u64) -> u64 {
    let a1640 = {
        let a1638 = f135(v0, 19u32, 45u32);
        let a1639 = f135(v0, 61u32, 3u32);
        fixed_u64::bit_xor(a1638, a1639)
    };
    let a1641 = f62(v0, 6u32);
    fixed_u64::bit_xor(a1640, a1641)
}

pub fn f137(v0: u64) -> u64 {
    let a1644 = {
        let a1642 = f135(v0, 1u32, 63u32);
        let a1643 = f135(v0, 8u32, 56u32);
        fixed_u64::bit_xor(a1642, a1643)
    };
    let a1645 = f62(v0, 7u32);
    fixed_u64::bit_xor(a1644, a1645)
}

pub fn f138(v0: u32, v1: u32, v2: u32, v3: u32, v4: u32, v5: u32) -> Adt11 {
    let v6: u32 = f74(f74(v0, v1), v4);
    let v7: u32 = f131({
        let a1646 = v3;
        let a1647 = v6;
        fixed_u32::bit_xor(a1646, a1647)
    }, 16u32, 16u32);
    let v8: u32 = f74(v2, v7);
    let v9: u32 = f131({
        let a1648 = v1;
        let a1649 = v8;
        fixed_u32::bit_xor(a1648, a1649)
    }, 12u32, 20u32);
    let v10: u32 = f74(f74(v6, v9), v5);
    let v11: u32 = f131({
        let a1650 = v7;
        let a1651 = v10;
        fixed_u32::bit_xor(a1650, a1651)
    }, 8u32, 24u32);
    let v12: u32 = f74(v8, v11);
    let v13: u32 = f131({
        let a1652 = v9;
        let a1653 = v12;
        fixed_u32::bit_xor(a1652, a1653)
    }, 7u32, 25u32);
    Adt11::C0(v10, v13, v12, v11)
}

pub fn f139(v0: Adt3, v1: u64) -> Adt3 {
    let v2: u64 = f143(v1);
    Adt3::C0({
        let a1655 = match v0.clone() {
            Adt3::C0(h1654, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1654
            }
        };
        let a1656 = v2;
        fixed_u64::bit_xor(a1655, a1656)
    }, match v0.clone() {
        Adt3::C0(_, h1657, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1657
        }
    }, match v0.clone() {
        Adt3::C0(_, _, h1658, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1658
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, h1659, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1659
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, h1660, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1660
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, h1661, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1661
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, h1662, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1662
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, h1663, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1663
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, h1664, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1664
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, h1665, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1665
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, h1666, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1666
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, h1667, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1667
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, h1668, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1668
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1669, _, _, _, _, _, _, _, _, _, _, _) => {
            h1669
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1670, _, _, _, _, _, _, _, _, _, _) => {
            h1670
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1671, _, _, _, _, _, _, _, _, _) => {
            h1671
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1672, _, _, _, _, _, _, _, _) => {
            h1672
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1673, _, _, _, _, _, _, _) => {
            h1673
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1674, _, _, _, _, _, _) => {
            h1674
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1675, _, _, _, _, _) => {
            h1675
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1676, _, _, _, _) => {
            h1676
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1677, _, _, _) => {
            h1677
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1678, _, _) => {
            h1678
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1679, _) => {
            h1679
        }
    }, match v0 {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1680) => {
            h1680
        }
    })
}

pub fn f140(v0: Adt3) -> Adt3 {
    Adt3::C0({
        let a1687 = match v0.clone() {
            Adt3::C0(h1681, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1681
            }
        };
        let a1688 = {
            let a1685 = {
                let a1683 = match v0.clone() {
                    Adt3::C0(_, h1682, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1682
                    }
                };
                fixed_u64::bit_not(a1683)
            };
            let a1686 = match v0.clone() {
                Adt3::C0(_, _, h1684, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1684
                }
            };
            fixed_u64::bit_and(a1685, a1686)
        };
        fixed_u64::bit_xor(a1687, a1688)
    }, {
        let a1695 = match v0.clone() {
            Adt3::C0(_, h1689, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1689
            }
        };
        let a1696 = {
            let a1693 = {
                let a1691 = match v0.clone() {
                    Adt3::C0(_, _, h1690, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1690
                    }
                };
                fixed_u64::bit_not(a1691)
            };
            let a1694 = match v0.clone() {
                Adt3::C0(_, _, _, h1692, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1692
                }
            };
            fixed_u64::bit_and(a1693, a1694)
        };
        fixed_u64::bit_xor(a1695, a1696)
    }, {
        let a1703 = match v0.clone() {
            Adt3::C0(_, _, h1697, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1697
            }
        };
        let a1704 = {
            let a1701 = {
                let a1699 = match v0.clone() {
                    Adt3::C0(_, _, _, h1698, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1698
                    }
                };
                fixed_u64::bit_not(a1699)
            };
            let a1702 = match v0.clone() {
                Adt3::C0(_, _, _, _, h1700, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1700
                }
            };
            fixed_u64::bit_and(a1701, a1702)
        };
        fixed_u64::bit_xor(a1703, a1704)
    }, {
        let a1711 = match v0.clone() {
            Adt3::C0(_, _, _, h1705, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1705
            }
        };
        let a1712 = {
            let a1709 = {
                let a1707 = match v0.clone() {
                    Adt3::C0(_, _, _, _, h1706, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1706
                    }
                };
                fixed_u64::bit_not(a1707)
            };
            let a1710 = match v0.clone() {
                Adt3::C0(h1708, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1708
                }
            };
            fixed_u64::bit_and(a1709, a1710)
        };
        fixed_u64::bit_xor(a1711, a1712)
    }, {
        let a1719 = match v0.clone() {
            Adt3::C0(_, _, _, _, h1713, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1713
            }
        };
        let a1720 = {
            let a1717 = {
                let a1715 = match v0.clone() {
                    Adt3::C0(h1714, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1714
                    }
                };
                fixed_u64::bit_not(a1715)
            };
            let a1718 = match v0.clone() {
                Adt3::C0(_, h1716, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1716
                }
            };
            fixed_u64::bit_and(a1717, a1718)
        };
        fixed_u64::bit_xor(a1719, a1720)
    }, {
        let a1727 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, h1721, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1721
            }
        };
        let a1728 = {
            let a1725 = {
                let a1723 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h1722, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1722
                    }
                };
                fixed_u64::bit_not(a1723)
            };
            let a1726 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, h1724, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1724
                }
            };
            fixed_u64::bit_and(a1725, a1726)
        };
        fixed_u64::bit_xor(a1727, a1728)
    }, {
        let a1735 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, h1729, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1729
            }
        };
        let a1736 = {
            let a1733 = {
                let a1731 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, h1730, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1730
                    }
                };
                fixed_u64::bit_not(a1731)
            };
            let a1734 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, h1732, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1732
                }
            };
            fixed_u64::bit_and(a1733, a1734)
        };
        fixed_u64::bit_xor(a1735, a1736)
    }, {
        let a1743 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, h1737, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1737
            }
        };
        let a1744 = {
            let a1741 = {
                let a1739 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, h1738, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1738
                    }
                };
                fixed_u64::bit_not(a1739)
            };
            let a1742 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, _, h1740, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1740
                }
            };
            fixed_u64::bit_and(a1741, a1742)
        };
        fixed_u64::bit_xor(a1743, a1744)
    }, {
        let a1751 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, h1745, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1745
            }
        };
        let a1752 = {
            let a1749 = {
                let a1747 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, h1746, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1746
                    }
                };
                fixed_u64::bit_not(a1747)
            };
            let a1750 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, h1748, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1748
                }
            };
            fixed_u64::bit_and(a1749, a1750)
        };
        fixed_u64::bit_xor(a1751, a1752)
    }, {
        let a1759 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, h1753, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1753
            }
        };
        let a1760 = {
            let a1757 = {
                let a1755 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, h1754, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1754
                    }
                };
                fixed_u64::bit_not(a1755)
            };
            let a1758 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, h1756, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1756
                }
            };
            fixed_u64::bit_and(a1757, a1758)
        };
        fixed_u64::bit_xor(a1759, a1760)
    }, {
        let a1767 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, h1761, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1761
            }
        };
        let a1768 = {
            let a1765 = {
                let a1763 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, h1762, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1762
                    }
                };
                fixed_u64::bit_not(a1763)
            };
            let a1766 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, h1764, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1764
                }
            };
            fixed_u64::bit_and(a1765, a1766)
        };
        fixed_u64::bit_xor(a1767, a1768)
    }, {
        let a1775 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, h1769, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1769
            }
        };
        let a1776 = {
            let a1773 = {
                let a1771 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, h1770, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1770
                    }
                };
                fixed_u64::bit_not(a1771)
            };
            let a1774 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1772, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1772
                }
            };
            fixed_u64::bit_and(a1773, a1774)
        };
        fixed_u64::bit_xor(a1775, a1776)
    }, {
        let a1783 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, h1777, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1777
            }
        };
        let a1784 = {
            let a1781 = {
                let a1779 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1778, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1778
                    }
                };
                fixed_u64::bit_not(a1779)
            };
            let a1782 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1780, _, _, _, _, _, _, _, _, _, _) => {
                    h1780
                }
            };
            fixed_u64::bit_and(a1781, a1782)
        };
        fixed_u64::bit_xor(a1783, a1784)
    }, {
        let a1791 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1785, _, _, _, _, _, _, _, _, _, _, _) => {
                h1785
            }
        };
        let a1792 = {
            let a1789 = {
                let a1787 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1786, _, _, _, _, _, _, _, _, _, _) => {
                        h1786
                    }
                };
                fixed_u64::bit_not(a1787)
            };
            let a1790 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, _, _, h1788, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1788
                }
            };
            fixed_u64::bit_and(a1789, a1790)
        };
        fixed_u64::bit_xor(a1791, a1792)
    }, {
        let a1799 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1793, _, _, _, _, _, _, _, _, _, _) => {
                h1793
            }
        };
        let a1800 = {
            let a1797 = {
                let a1795 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, _, h1794, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1794
                    }
                };
                fixed_u64::bit_not(a1795)
            };
            let a1798 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, h1796, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1796
                }
            };
            fixed_u64::bit_and(a1797, a1798)
        };
        fixed_u64::bit_xor(a1799, a1800)
    }, {
        let a1807 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1801, _, _, _, _, _, _, _, _, _) => {
                h1801
            }
        };
        let a1808 = {
            let a1805 = {
                let a1803 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1802, _, _, _, _, _, _, _, _) => {
                        h1802
                    }
                };
                fixed_u64::bit_not(a1803)
            };
            let a1806 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1804, _, _, _, _, _, _, _) => {
                    h1804
                }
            };
            fixed_u64::bit_and(a1805, a1806)
        };
        fixed_u64::bit_xor(a1807, a1808)
    }, {
        let a1815 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1809, _, _, _, _, _, _, _, _) => {
                h1809
            }
        };
        let a1816 = {
            let a1813 = {
                let a1811 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1810, _, _, _, _, _, _, _) => {
                        h1810
                    }
                };
                fixed_u64::bit_not(a1811)
            };
            let a1814 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1812, _, _, _, _, _, _) => {
                    h1812
                }
            };
            fixed_u64::bit_and(a1813, a1814)
        };
        fixed_u64::bit_xor(a1815, a1816)
    }, {
        let a1823 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1817, _, _, _, _, _, _, _) => {
                h1817
            }
        };
        let a1824 = {
            let a1821 = {
                let a1819 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1818, _, _, _, _, _, _) => {
                        h1818
                    }
                };
                fixed_u64::bit_not(a1819)
            };
            let a1822 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1820, _, _, _, _, _) => {
                    h1820
                }
            };
            fixed_u64::bit_and(a1821, a1822)
        };
        fixed_u64::bit_xor(a1823, a1824)
    }, {
        let a1831 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1825, _, _, _, _, _, _) => {
                h1825
            }
        };
        let a1832 = {
            let a1829 = {
                let a1827 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1826, _, _, _, _, _) => {
                        h1826
                    }
                };
                fixed_u64::bit_not(a1827)
            };
            let a1830 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1828, _, _, _, _, _, _, _, _, _) => {
                    h1828
                }
            };
            fixed_u64::bit_and(a1829, a1830)
        };
        fixed_u64::bit_xor(a1831, a1832)
    }, {
        let a1839 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1833, _, _, _, _, _) => {
                h1833
            }
        };
        let a1840 = {
            let a1837 = {
                let a1835 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1834, _, _, _, _, _, _, _, _, _) => {
                        h1834
                    }
                };
                fixed_u64::bit_not(a1835)
            };
            let a1838 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1836, _, _, _, _, _, _, _, _) => {
                    h1836
                }
            };
            fixed_u64::bit_and(a1837, a1838)
        };
        fixed_u64::bit_xor(a1839, a1840)
    }, {
        let a1847 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1841, _, _, _, _) => {
                h1841
            }
        };
        let a1848 = {
            let a1845 = {
                let a1843 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1842, _, _, _) => {
                        h1842
                    }
                };
                fixed_u64::bit_not(a1843)
            };
            let a1846 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1844, _, _) => {
                    h1844
                }
            };
            fixed_u64::bit_and(a1845, a1846)
        };
        fixed_u64::bit_xor(a1847, a1848)
    }, {
        let a1855 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1849, _, _, _) => {
                h1849
            }
        };
        let a1856 = {
            let a1853 = {
                let a1851 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1850, _, _) => {
                        h1850
                    }
                };
                fixed_u64::bit_not(a1851)
            };
            let a1854 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1852, _) => {
                    h1852
                }
            };
            fixed_u64::bit_and(a1853, a1854)
        };
        fixed_u64::bit_xor(a1855, a1856)
    }, {
        let a1863 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1857, _, _) => {
                h1857
            }
        };
        let a1864 = {
            let a1861 = {
                let a1859 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1858, _) => {
                        h1858
                    }
                };
                fixed_u64::bit_not(a1859)
            };
            let a1862 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1860) => {
                    h1860
                }
            };
            fixed_u64::bit_and(a1861, a1862)
        };
        fixed_u64::bit_xor(a1863, a1864)
    }, {
        let a1871 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1865, _) => {
                h1865
            }
        };
        let a1872 = {
            let a1869 = {
                let a1867 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1866) => {
                        h1866
                    }
                };
                fixed_u64::bit_not(a1867)
            };
            let a1870 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1868, _, _, _, _) => {
                    h1868
                }
            };
            fixed_u64::bit_and(a1869, a1870)
        };
        fixed_u64::bit_xor(a1871, a1872)
    }, {
        let a1879 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1873) => {
                h1873
            }
        };
        let a1880 = {
            let a1877 = {
                let a1875 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1874, _, _, _, _) => {
                        h1874
                    }
                };
                fixed_u64::bit_not(a1875)
            };
            let a1878 = match v0 {
                Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1876, _, _, _) => {
                    h1876
                }
            };
            fixed_u64::bit_and(a1877, a1878)
        };
        fixed_u64::bit_xor(a1879, a1880)
    })
}

pub fn f141(v0: Adt3) -> Adt3 {
    Adt3::C0(match v0.clone() {
        Adt3::C0(h1881, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1881
        }
    }, f144(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, h1882, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1882
        }
    }, 44u32, 20u32), f144(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, h1883, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1883
        }
    }, 43u32, 21u32), f144(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1884, _, _, _, _, _, _) => {
            h1884
        }
    }, 21u32, 43u32), f144(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1885) => {
            h1885
        }
    }, 14u32, 50u32), f144(match v0.clone() {
        Adt3::C0(_, _, _, h1886, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1886
        }
    }, 28u32, 36u32), f144(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, h1887, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1887
        }
    }, 20u32, 44u32), f144(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, h1888, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1888
        }
    }, 3u32, 61u32), f144(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1889, _, _, _, _, _, _, _, _) => {
            h1889
        }
    }, 45u32, 19u32), f144(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1890, _, _) => {
            h1890
        }
    }, 61u32, 3u32), f144(match v0.clone() {
        Adt3::C0(_, h1891, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1891
        }
    }, 1u32, 63u32), f144(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, h1892, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1892
        }
    }, 6u32, 58u32), f144(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1893, _, _, _, _, _, _, _, _, _, _, _) => {
            h1893
        }
    }, 25u32, 39u32), f144(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1894, _, _, _, _, _) => {
            h1894
        }
    }, 8u32, 56u32), f144(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1895, _, _, _, _) => {
            h1895
        }
    }, 18u32, 46u32), f144(match v0.clone() {
        Adt3::C0(_, _, _, _, h1896, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1896
        }
    }, 27u32, 37u32), f144(match v0.clone() {
        Adt3::C0(_, _, _, _, _, h1897, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1897
        }
    }, 36u32, 28u32), f144(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, h1898, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1898
        }
    }, 10u32, 54u32), f144(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1899, _, _, _, _, _, _, _) => {
            h1899
        }
    }, 15u32, 49u32), f144(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1900, _) => {
            h1900
        }
    }, 56u32, 8u32), f144(match v0.clone() {
        Adt3::C0(_, _, h1901, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1901
        }
    }, 62u32, 2u32), f144(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, h1902, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1902
        }
    }, 55u32, 9u32), f144(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1903, _, _, _, _, _, _, _, _, _, _) => {
            h1903
        }
    }, 39u32, 25u32), f144(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1904, _, _, _, _, _, _, _, _, _) => {
            h1904
        }
    }, 41u32, 23u32), f144(match v0 {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1905, _, _, _) => {
            h1905
        }
    }, 2u32, 62u32))
}

pub fn f142(v0: Adt3) -> Adt3 {
    let v1: u64 = {
        let a1917 = {
            let a1914 = {
                let a1911 = {
                    let a1908 = match v0.clone() {
                        Adt3::C0(h1906, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1906
                        }
                    };
                    let a1909 = match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h1907, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1907
                        }
                    };
                    fixed_u64::bit_xor(a1908, a1909)
                };
                let a1912 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, _, h1910, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1910
                    }
                };
                fixed_u64::bit_xor(a1911, a1912)
            };
            let a1915 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1913, _, _, _, _, _, _, _, _, _) => {
                    h1913
                }
            };
            fixed_u64::bit_xor(a1914, a1915)
        };
        let a1918 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1916, _, _, _, _) => {
                h1916
            }
        };
        fixed_u64::bit_xor(a1917, a1918)
    };
    let v2: u64 = {
        let a1930 = {
            let a1927 = {
                let a1924 = {
                    let a1921 = match v0.clone() {
                        Adt3::C0(_, h1919, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1919
                        }
                    };
                    let a1922 = match v0.clone() {
                        Adt3::C0(_, _, _, _, _, _, h1920, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1920
                        }
                    };
                    fixed_u64::bit_xor(a1921, a1922)
                };
                let a1925 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, h1923, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1923
                    }
                };
                fixed_u64::bit_xor(a1924, a1925)
            };
            let a1928 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1926, _, _, _, _, _, _, _, _) => {
                    h1926
                }
            };
            fixed_u64::bit_xor(a1927, a1928)
        };
        let a1931 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1929, _, _, _) => {
                h1929
            }
        };
        fixed_u64::bit_xor(a1930, a1931)
    };
    let v3: u64 = {
        let a1943 = {
            let a1940 = {
                let a1937 = {
                    let a1934 = match v0.clone() {
                        Adt3::C0(_, _, h1932, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1932
                        }
                    };
                    let a1935 = match v0.clone() {
                        Adt3::C0(_, _, _, _, _, _, _, h1933, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1933
                        }
                    };
                    fixed_u64::bit_xor(a1934, a1935)
                };
                let a1938 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, h1936, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1936
                    }
                };
                fixed_u64::bit_xor(a1937, a1938)
            };
            let a1941 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1939, _, _, _, _, _, _, _) => {
                    h1939
                }
            };
            fixed_u64::bit_xor(a1940, a1941)
        };
        let a1944 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1942, _, _) => {
                h1942
            }
        };
        fixed_u64::bit_xor(a1943, a1944)
    };
    let v4: u64 = {
        let a1956 = {
            let a1953 = {
                let a1950 = {
                    let a1947 = match v0.clone() {
                        Adt3::C0(_, _, _, h1945, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1945
                        }
                    };
                    let a1948 = match v0.clone() {
                        Adt3::C0(_, _, _, _, _, _, _, _, h1946, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1946
                        }
                    };
                    fixed_u64::bit_xor(a1947, a1948)
                };
                let a1951 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1949, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1949
                    }
                };
                fixed_u64::bit_xor(a1950, a1951)
            };
            let a1954 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1952, _, _, _, _, _, _) => {
                    h1952
                }
            };
            fixed_u64::bit_xor(a1953, a1954)
        };
        let a1957 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1955, _) => {
                h1955
            }
        };
        fixed_u64::bit_xor(a1956, a1957)
    };
    let v5: u64 = {
        let a1969 = {
            let a1966 = {
                let a1963 = {
                    let a1960 = match v0.clone() {
                        Adt3::C0(_, _, _, _, h1958, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1958
                        }
                    };
                    let a1961 = match v0.clone() {
                        Adt3::C0(_, _, _, _, _, _, _, _, _, h1959, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1959
                        }
                    };
                    fixed_u64::bit_xor(a1960, a1961)
                };
                let a1964 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1962, _, _, _, _, _, _, _, _, _, _) => {
                        h1962
                    }
                };
                fixed_u64::bit_xor(a1963, a1964)
            };
            let a1967 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1965, _, _, _, _, _) => {
                    h1965
                }
            };
            fixed_u64::bit_xor(a1966, a1967)
        };
        let a1970 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1968) => {
                h1968
            }
        };
        fixed_u64::bit_xor(a1969, a1970)
    };
    let v6: u64 = {
        let a1971 = v5;
        let a1972 = f144(v2, 1u32, 63u32);
        fixed_u64::bit_xor(a1971, a1972)
    };
    let v7: u64 = {
        let a1973 = v1;
        let a1974 = f144(v3, 1u32, 63u32);
        fixed_u64::bit_xor(a1973, a1974)
    };
    let v8: u64 = {
        let a1975 = v2;
        let a1976 = f144(v4, 1u32, 63u32);
        fixed_u64::bit_xor(a1975, a1976)
    };
    let v9: u64 = {
        let a1977 = v3;
        let a1978 = f144(v5, 1u32, 63u32);
        fixed_u64::bit_xor(a1977, a1978)
    };
    let v10: u64 = {
        let a1979 = v4;
        let a1980 = f144(v1, 1u32, 63u32);
        fixed_u64::bit_xor(a1979, a1980)
    };
    Adt3::C0({
        let a1982 = match v0.clone() {
            Adt3::C0(h1981, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1981
            }
        };
        let a1983 = v6;
        fixed_u64::bit_xor(a1982, a1983)
    }, {
        let a1985 = match v0.clone() {
            Adt3::C0(_, h1984, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1984
            }
        };
        let a1986 = v7;
        fixed_u64::bit_xor(a1985, a1986)
    }, {
        let a1988 = match v0.clone() {
            Adt3::C0(_, _, h1987, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1987
            }
        };
        let a1989 = v8;
        fixed_u64::bit_xor(a1988, a1989)
    }, {
        let a1991 = match v0.clone() {
            Adt3::C0(_, _, _, h1990, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1990
            }
        };
        let a1992 = v9;
        fixed_u64::bit_xor(a1991, a1992)
    }, {
        let a1994 = match v0.clone() {
            Adt3::C0(_, _, _, _, h1993, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1993
            }
        };
        let a1995 = v10;
        fixed_u64::bit_xor(a1994, a1995)
    }, {
        let a1997 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, h1996, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1996
            }
        };
        let a1998 = v6;
        fixed_u64::bit_xor(a1997, a1998)
    }, {
        let a2000 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, h1999, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1999
            }
        };
        let a2001 = v7;
        fixed_u64::bit_xor(a2000, a2001)
    }, {
        let a2003 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, h2002, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2002
            }
        };
        let a2004 = v8;
        fixed_u64::bit_xor(a2003, a2004)
    }, {
        let a2006 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, h2005, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2005
            }
        };
        let a2007 = v9;
        fixed_u64::bit_xor(a2006, a2007)
    }, {
        let a2009 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, h2008, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2008
            }
        };
        let a2010 = v10;
        fixed_u64::bit_xor(a2009, a2010)
    }, {
        let a2012 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, h2011, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2011
            }
        };
        let a2013 = v6;
        fixed_u64::bit_xor(a2012, a2013)
    }, {
        let a2015 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, h2014, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2014
            }
        };
        let a2016 = v7;
        fixed_u64::bit_xor(a2015, a2016)
    }, {
        let a2018 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, h2017, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2017
            }
        };
        let a2019 = v8;
        fixed_u64::bit_xor(a2018, a2019)
    }, {
        let a2021 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h2020, _, _, _, _, _, _, _, _, _, _, _) => {
                h2020
            }
        };
        let a2022 = v9;
        fixed_u64::bit_xor(a2021, a2022)
    }, {
        let a2024 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2023, _, _, _, _, _, _, _, _, _, _) => {
                h2023
            }
        };
        let a2025 = v10;
        fixed_u64::bit_xor(a2024, a2025)
    }, {
        let a2027 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2026, _, _, _, _, _, _, _, _, _) => {
                h2026
            }
        };
        let a2028 = v6;
        fixed_u64::bit_xor(a2027, a2028)
    }, {
        let a2030 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2029, _, _, _, _, _, _, _, _) => {
                h2029
            }
        };
        let a2031 = v7;
        fixed_u64::bit_xor(a2030, a2031)
    }, {
        let a2033 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2032, _, _, _, _, _, _, _) => {
                h2032
            }
        };
        let a2034 = v8;
        fixed_u64::bit_xor(a2033, a2034)
    }, {
        let a2036 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2035, _, _, _, _, _, _) => {
                h2035
            }
        };
        let a2037 = v9;
        fixed_u64::bit_xor(a2036, a2037)
    }, {
        let a2039 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2038, _, _, _, _, _) => {
                h2038
            }
        };
        let a2040 = v10;
        fixed_u64::bit_xor(a2039, a2040)
    }, {
        let a2042 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2041, _, _, _, _) => {
                h2041
            }
        };
        let a2043 = v6;
        fixed_u64::bit_xor(a2042, a2043)
    }, {
        let a2045 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2044, _, _, _) => {
                h2044
            }
        };
        let a2046 = v7;
        fixed_u64::bit_xor(a2045, a2046)
    }, {
        let a2048 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2047, _, _) => {
                h2047
            }
        };
        let a2049 = v8;
        fixed_u64::bit_xor(a2048, a2049)
    }, {
        let a2051 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2050, _) => {
                h2050
            }
        };
        let a2052 = v9;
        fixed_u64::bit_xor(a2051, a2052)
    }, {
        let a2054 = match v0 {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2053) => {
                h2053
            }
        };
        let a2055 = v10;
        fixed_u64::bit_xor(a2054, a2055)
    })
}

pub fn f143(v0: u64) -> u64 {
    let a2056 = v0;
    let a2057 = 12u64;
    if nat_lt(a2056, a2057) {
        let a2058 = v0;
        let a2059 = 6u64;
        if nat_lt(a2058, a2059) {
            let a2060 = v0;
            let a2061 = 3u64;
            if nat_lt(a2060, a2061) {
                let a2062 = v0;
                let a2063 = 1u64;
                if nat_lt(a2062, a2063) {
                    1u64
                } else {
                    let a2064 = v0;
                    let a2065 = 2u64;
                    if nat_lt(a2064, a2065) {
                        32898u64
                    } else {
                        9223372036854808714u64
                    }
                }
            } else {
                let a2066 = v0;
                let a2067 = 4u64;
                if nat_lt(a2066, a2067) {
                    9223372039002292224u64
                } else {
                    let a2068 = v0;
                    let a2069 = 5u64;
                    if nat_lt(a2068, a2069) {
                        32907u64
                    } else {
                        2147483649u64
                    }
                }
            }
        } else {
            let a2070 = v0;
            let a2071 = 9u64;
            if nat_lt(a2070, a2071) {
                let a2072 = v0;
                let a2073 = 7u64;
                if nat_lt(a2072, a2073) {
                    9223372039002292353u64
                } else {
                    let a2074 = v0;
                    let a2075 = 8u64;
                    if nat_lt(a2074, a2075) {
                        9223372036854808585u64
                    } else {
                        138u64
                    }
                }
            } else {
                let a2076 = v0;
                let a2077 = 10u64;
                if nat_lt(a2076, a2077) {
                    136u64
                } else {
                    let a2078 = v0;
                    let a2079 = 11u64;
                    if nat_lt(a2078, a2079) {
                        2147516425u64
                    } else {
                        2147483658u64
                    }
                }
            }
        }
    } else {
        let a2080 = v0;
        let a2081 = 18u64;
        if nat_lt(a2080, a2081) {
            let a2082 = v0;
            let a2083 = 15u64;
            if nat_lt(a2082, a2083) {
                let a2084 = v0;
                let a2085 = 13u64;
                if nat_lt(a2084, a2085) {
                    2147516555u64
                } else {
                    let a2086 = v0;
                    let a2087 = 14u64;
                    if nat_lt(a2086, a2087) {
                        9223372036854775947u64
                    } else {
                        9223372036854808713u64
                    }
                }
            } else {
                let a2088 = v0;
                let a2089 = 16u64;
                if nat_lt(a2088, a2089) {
                    9223372036854808579u64
                } else {
                    let a2090 = v0;
                    let a2091 = 17u64;
                    if nat_lt(a2090, a2091) {
                        9223372036854808578u64
                    } else {
                        9223372036854775936u64
                    }
                }
            }
        } else {
            let a2092 = v0;
            let a2093 = 21u64;
            if nat_lt(a2092, a2093) {
                let a2094 = v0;
                let a2095 = 19u64;
                if nat_lt(a2094, a2095) {
                    32778u64
                } else {
                    let a2096 = v0;
                    let a2097 = 20u64;
                    if nat_lt(a2096, a2097) {
                        9223372039002259466u64
                    } else {
                        9223372039002292353u64
                    }
                }
            } else {
                let a2098 = v0;
                let a2099 = 22u64;
                if nat_lt(a2098, a2099) {
                    9223372036854808704u64
                } else {
                    let a2100 = v0;
                    let a2101 = 23u64;
                    if nat_lt(a2100, a2101) {
                        2147483649u64
                    } else {
                        9223372039002292232u64
                    }
                }
            }
        }
    }
}

pub fn f144(v0: u64, v1: u32, v2: u32) -> u64 {
    let a2102 = f99(v0, v1);
    let a2103 = f62(v0, v2);
    fixed_u64::bit_or(a2102, a2103)
}
