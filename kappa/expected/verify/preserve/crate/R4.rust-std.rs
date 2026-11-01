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
    pub fn apply(&self, p0: (u64, Adt0)) -> R<(u64, Adt0)> {
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
    pub fn apply(&self, p0: Adt2) -> R<Adt2> {
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

pub fn f0(v0: Bytes) -> R<Option<Bytes>> {
    if f1(v0.clone())? {
        Ok(Some(f2(v0)?))
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f1(v0: Bytes) -> R<bool> {
    let a2 = {
        let a1 = v0.clone();
        length_bytes(a1)
    };
    let a3 = 71u64;
    if nat_eq(a2, a3) {
        if f3(v0.clone(), f4(Str::lit("sha256:"))) {
            let a4 = f5(v0, 7u64)?;
            let a5 = 71u64;
            Ok(nat_eq(a4, a5))
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f2(v0: Bytes) -> R<Bytes> {
    let a6 = f4(Str::lit("sha256:"));
    let a7 = f6(v0)?;
    Ok(append_bytes(a6, a7))
}

pub fn f3(v0: Bytes, v1: Bytes) -> bool {
    let m12 = {
        let a9 = v0;
        let a10 = 0u64;
        let a11 = {
            let a8 = v1.clone();
            length_bytes(a8)
        };
        slice_bytes(a9, a10, a11)
    };
    match m12 {
        None => {
            false
        }
        Some(v2) => {
            let a13 = v2;
            let a14 = v1;
            equal(a13, a14)
        }
    }
}

pub fn f4(v0: Str) -> Bytes {
    let a15 = v0;
    utf8_encode(a15)
}

pub fn f5(v0: Bytes, v1: u64) -> R<u64> {
    let (h16, _) = f7(Fn0::F8(v0.clone()), {
        let a17 = v0;
        length_bytes(a17)
    }, v1)?;
    Ok(h16)
}

pub fn f6(v0: Bytes) -> R<Bytes> {
    Ok(f9(f10(v0)?))
}

pub fn f7(v0: Fn0, p22: u64, p23: u64) -> R<(u64, bool)> {
    let m21 = (p22, p23);
    {
        let mut m21 = m21;
        loop {
            let step = {
                let (v1, v2) = m21;
                let m18 = v1;
                if m18 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m18 - 1;
                    let m20 = {
                        let c19: Fn0 = v0.clone();
                        c19.apply(v2)?
                    };
                    match m20 {
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
                Ok(next) => { m21 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f8(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f11(v0, v1)? {
        Ok(Some({
            let a24 = v1;
            let a25 = 1u64;
            nat_add(a24, a25)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f9(v0: Adt0) -> Bytes {
    let a46 = f12(match v0.clone() {
        Adt0::C0(h26, _, _, _, _, _, _, _) => {
            h26
        }
    });
    let a47 = {
        let a44 = f12(match v0.clone() {
            Adt0::C0(_, h27, _, _, _, _, _, _) => {
                h27
            }
        });
        let a45 = {
            let a42 = f12(match v0.clone() {
                Adt0::C0(_, _, h28, _, _, _, _, _) => {
                    h28
                }
            });
            let a43 = {
                let a40 = f12(match v0.clone() {
                    Adt0::C0(_, _, _, h29, _, _, _, _) => {
                        h29
                    }
                });
                let a41 = {
                    let a38 = f12(match v0.clone() {
                        Adt0::C0(_, _, _, _, h30, _, _, _) => {
                            h30
                        }
                    });
                    let a39 = {
                        let a36 = f12(match v0.clone() {
                            Adt0::C0(_, _, _, _, _, h31, _, _) => {
                                h31
                            }
                        });
                        let a37 = {
                            let a34 = f12(match v0.clone() {
                                Adt0::C0(_, _, _, _, _, _, h32, _) => {
                                    h32
                                }
                            });
                            let a35 = f12(match v0 {
                                Adt0::C0(_, _, _, _, _, _, _, h33) => {
                                    h33
                                }
                            });
                            append_bytes(a34, a35)
                        };
                        append_bytes(a36, a37)
                    };
                    append_bytes(a38, a39)
                };
                append_bytes(a40, a41)
            };
            append_bytes(a42, a43)
        };
        append_bytes(a44, a45)
    };
    append_bytes(a46, a47)
}

pub fn f10(v0: Bytes) -> R<Adt0> {
    let v1: u64 = {
        let a48 = v0.clone();
        length_bytes(a48)
    };
    let v2: u64 = {
        let a54 = {
            let a51 = {
                let a49 = v1;
                let a50 = 72u64;
                nat_add(a49, a50)?
            };
            let a52 = 64u64;
            let a53 = 0u64;
            nat_quot(a51, a52, a53)
        };
        let a55 = 64u64;
        nat_mul(a54, a55)?
    };
    let (_, h56) = f13(Fn1::F14(v0, v1, v2), {
        let a57 = v2;
        let a58 = 64u64;
        let a59 = 0u64;
        nat_quot(a57, a58, a59)
    }, (0u64, f15()))?;
    Ok(h56)
}

pub fn f11(v0: Bytes, v1: u64) -> R<bool> {
    let v2: u64 = f16(v0, v1)?;
    let a60 = 47u64;
    let a61 = v2;
    if nat_lt(a60, a61) {
        let a62 = v2;
        let a63 = 58u64;
        if nat_lt(a62, a63) {
            Ok(true)
        } else {
            let a64 = 96u64;
            let a65 = v2;
            if nat_lt(a64, a65) {
                let a66 = v2;
                let a67 = 103u64;
                Ok(nat_lt(a66, a67))
            } else {
                Ok(false)
            }
        }
    } else {
        Ok(false)
    }
}

pub fn f12(v0: u32) -> Bytes {
    let a96 = f17({
        let a68 = f18(v0, 28u32);
        let a69 = 15u32;
        fixed_u32::bit_and(a68, a69)
    });
    let a97 = {
        let a94 = f17({
            let a70 = f18(v0, 24u32);
            let a71 = 15u32;
            fixed_u32::bit_and(a70, a71)
        });
        let a95 = {
            let a92 = f17({
                let a72 = f18(v0, 20u32);
                let a73 = 15u32;
                fixed_u32::bit_and(a72, a73)
            });
            let a93 = {
                let a90 = f17({
                    let a74 = f18(v0, 16u32);
                    let a75 = 15u32;
                    fixed_u32::bit_and(a74, a75)
                });
                let a91 = {
                    let a88 = f17({
                        let a76 = f18(v0, 12u32);
                        let a77 = 15u32;
                        fixed_u32::bit_and(a76, a77)
                    });
                    let a89 = {
                        let a86 = f17({
                            let a78 = f18(v0, 8u32);
                            let a79 = 15u32;
                            fixed_u32::bit_and(a78, a79)
                        });
                        let a87 = {
                            let a84 = f17({
                                let a80 = f18(v0, 4u32);
                                let a81 = 15u32;
                                fixed_u32::bit_and(a80, a81)
                            });
                            let a85 = f17({
                                let a82 = v0;
                                let a83 = 15u32;
                                fixed_u32::bit_and(a82, a83)
                            });
                            append_bytes(a84, a85)
                        };
                        append_bytes(a86, a87)
                    };
                    append_bytes(a88, a89)
                };
                append_bytes(a90, a91)
            };
            append_bytes(a92, a93)
        };
        append_bytes(a94, a95)
    };
    append_bytes(a96, a97)
}

pub fn f13(v0: Fn1, p101: u64, p102: (u64, Adt0)) -> R<(u64, Adt0)> {
    let m100 = (p101, p102);
    {
        let mut m100 = m100;
        loop {
            let step = {
                let (v1, v2) = m100;
                let m98 = v1;
                if m98 == 0 {
                    Err::<(u64, (u64, Adt0)), R<(u64, Adt0)>>(Ok(v2))
                } else {
                    let v3 = m98 - 1;
                    Ok::<(u64, (u64, Adt0)), R<(u64, Adt0)>>((v3, {
                        let c99: Fn1 = v0.clone();
                        c99.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m100 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f14(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt0)) -> R<(u64, Adt0)> {
    Ok(({
        let a104 = {
            let (h103, _) = v3.clone();
            h103
        };
        let a105 = 64u64;
        nat_add(a104, a105)?
    }, f19({
        let (_, h106) = v3;
        h106
    }, f20(v0, v1, v2, {
        let (h107, _) = v3;
        h107
    })?)?))
}

pub fn f15() -> Adt0 {
    Adt0::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f16(v0: Bytes, v1: u64) -> R<u64> {
    let m110 = {
        let a108 = v0;
        let a109 = v1;
        index_bytes(a108, a109)
    };
    match m110 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a155 = {
                let a131 = {
                    let a119 = {
                        let a113 = {
                            let a111 = v3;
                            let a112 = 128u8;
                            fixed_u8::bit_and(a111, a112)
                        };
                        let a114 = 128u8;
                        if equal(a113, a114) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a120 = {
                        let a117 = {
                            let a115 = v3;
                            let a116 = 64u8;
                            fixed_u8::bit_and(a115, a116)
                        };
                        let a118 = 64u8;
                        if equal(a117, a118) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a119, a120)?
                };
                let a132 = {
                    let a129 = {
                        let a123 = {
                            let a121 = v3;
                            let a122 = 32u8;
                            fixed_u8::bit_and(a121, a122)
                        };
                        let a124 = 32u8;
                        if equal(a123, a124) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a130 = {
                        let a127 = {
                            let a125 = v3;
                            let a126 = 16u8;
                            fixed_u8::bit_and(a125, a126)
                        };
                        let a128 = 16u8;
                        if equal(a127, a128) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a129, a130)?
                };
                nat_add(a131, a132)?
            };
            let a156 = {
                let a153 = {
                    let a141 = {
                        let a135 = {
                            let a133 = v3;
                            let a134 = 8u8;
                            fixed_u8::bit_and(a133, a134)
                        };
                        let a136 = 8u8;
                        if equal(a135, a136) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a142 = {
                        let a139 = {
                            let a137 = v3;
                            let a138 = 4u8;
                            fixed_u8::bit_and(a137, a138)
                        };
                        let a140 = 4u8;
                        if equal(a139, a140) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a141, a142)?
                };
                let a154 = {
                    let a151 = {
                        let a145 = {
                            let a143 = v3;
                            let a144 = 2u8;
                            fixed_u8::bit_and(a143, a144)
                        };
                        let a146 = 2u8;
                        if equal(a145, a146) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a152 = {
                        let a149 = {
                            let a147 = v3;
                            let a148 = 1u8;
                            fixed_u8::bit_and(a147, a148)
                        };
                        let a150 = 1u8;
                        if equal(a149, a150) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a151, a152)?
                };
                nat_add(a153, a154)?
            };
            nat_add(a155, a156)
        }
    }
}

pub fn f17(v0: u32) -> Bytes {
    let a159 = {
        let a157 = v0;
        let a158 = 8u32;
        fixed_u32::bit_and(a157, a158)
    };
    let a160 = 0u32;
    if equal(a159, a160) {
        let a163 = {
            let a161 = v0;
            let a162 = 4u32;
            fixed_u32::bit_and(a161, a162)
        };
        let a164 = 0u32;
        if equal(a163, a164) {
            let a167 = {
                let a165 = v0;
                let a166 = 2u32;
                fixed_u32::bit_and(a165, a166)
            };
            let a168 = 0u32;
            if equal(a167, a168) {
                let a171 = {
                    let a169 = v0;
                    let a170 = 1u32;
                    fixed_u32::bit_and(a169, a170)
                };
                let a172 = 0u32;
                if equal(a171, a172) {
                    Bytes::lit(&[0x30])
                } else {
                    Bytes::lit(&[0x31])
                }
            } else {
                let a175 = {
                    let a173 = v0;
                    let a174 = 1u32;
                    fixed_u32::bit_and(a173, a174)
                };
                let a176 = 0u32;
                if equal(a175, a176) {
                    Bytes::lit(&[0x32])
                } else {
                    Bytes::lit(&[0x33])
                }
            }
        } else {
            let a179 = {
                let a177 = v0;
                let a178 = 2u32;
                fixed_u32::bit_and(a177, a178)
            };
            let a180 = 0u32;
            if equal(a179, a180) {
                let a183 = {
                    let a181 = v0;
                    let a182 = 1u32;
                    fixed_u32::bit_and(a181, a182)
                };
                let a184 = 0u32;
                if equal(a183, a184) {
                    Bytes::lit(&[0x34])
                } else {
                    Bytes::lit(&[0x35])
                }
            } else {
                let a187 = {
                    let a185 = v0;
                    let a186 = 1u32;
                    fixed_u32::bit_and(a185, a186)
                };
                let a188 = 0u32;
                if equal(a187, a188) {
                    Bytes::lit(&[0x36])
                } else {
                    Bytes::lit(&[0x37])
                }
            }
        }
    } else {
        let a191 = {
            let a189 = v0;
            let a190 = 4u32;
            fixed_u32::bit_and(a189, a190)
        };
        let a192 = 0u32;
        if equal(a191, a192) {
            let a195 = {
                let a193 = v0;
                let a194 = 2u32;
                fixed_u32::bit_and(a193, a194)
            };
            let a196 = 0u32;
            if equal(a195, a196) {
                let a199 = {
                    let a197 = v0;
                    let a198 = 1u32;
                    fixed_u32::bit_and(a197, a198)
                };
                let a200 = 0u32;
                if equal(a199, a200) {
                    Bytes::lit(&[0x38])
                } else {
                    Bytes::lit(&[0x39])
                }
            } else {
                let a203 = {
                    let a201 = v0;
                    let a202 = 1u32;
                    fixed_u32::bit_and(a201, a202)
                };
                let a204 = 0u32;
                if equal(a203, a204) {
                    Bytes::lit(&[0x61])
                } else {
                    Bytes::lit(&[0x62])
                }
            }
        } else {
            let a207 = {
                let a205 = v0;
                let a206 = 2u32;
                fixed_u32::bit_and(a205, a206)
            };
            let a208 = 0u32;
            if equal(a207, a208) {
                let a211 = {
                    let a209 = v0;
                    let a210 = 1u32;
                    fixed_u32::bit_and(a209, a210)
                };
                let a212 = 0u32;
                if equal(a211, a212) {
                    Bytes::lit(&[0x63])
                } else {
                    Bytes::lit(&[0x64])
                }
            } else {
                let a215 = {
                    let a213 = v0;
                    let a214 = 1u32;
                    fixed_u32::bit_and(a213, a214)
                };
                let a216 = 0u32;
                if equal(a215, a216) {
                    Bytes::lit(&[0x65])
                } else {
                    Bytes::lit(&[0x66])
                }
            }
        }
    }
}

pub fn f18(v0: u32, v1: u32) -> u32 {
    let m219 = {
        let a217 = v0;
        let a218 = v1;
        fixed_u32::shift_right(a217, a218)
    };
    match m219 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f19(v0: Adt0, v1: Adt1) -> R<Adt0> {
    let v2: Adt2 = f21(Fn2::F22, 64u64, Adt2::C0(match v0.clone() {
        Adt0::C0(h220, _, _, _, _, _, _, _) => {
            h220
        }
    }, match v0.clone() {
        Adt0::C0(_, h221, _, _, _, _, _, _) => {
            h221
        }
    }, match v0.clone() {
        Adt0::C0(_, _, h222, _, _, _, _, _) => {
            h222
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, h223, _, _, _, _) => {
            h223
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, h224, _, _, _) => {
            h224
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, h225, _, _) => {
            h225
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, h226, _) => {
            h226
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, h227) => {
            h227
        }
    }, v1, 0u64))?;
    Ok(Adt0::C0(f23(match v0.clone() {
        Adt0::C0(h228, _, _, _, _, _, _, _) => {
            h228
        }
    }, match v2.clone() {
        Adt2::C0(h229, _, _, _, _, _, _, _, _, _) => {
            h229
        }
    }), f23(match v0.clone() {
        Adt0::C0(_, h230, _, _, _, _, _, _) => {
            h230
        }
    }, match v2.clone() {
        Adt2::C0(_, h231, _, _, _, _, _, _, _, _) => {
            h231
        }
    }), f23(match v0.clone() {
        Adt0::C0(_, _, h232, _, _, _, _, _) => {
            h232
        }
    }, match v2.clone() {
        Adt2::C0(_, _, h233, _, _, _, _, _, _, _) => {
            h233
        }
    }), f23(match v0.clone() {
        Adt0::C0(_, _, _, h234, _, _, _, _) => {
            h234
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, h235, _, _, _, _, _, _) => {
            h235
        }
    }), f23(match v0.clone() {
        Adt0::C0(_, _, _, _, h236, _, _, _) => {
            h236
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, _, h237, _, _, _, _, _) => {
            h237
        }
    }), f23(match v0.clone() {
        Adt0::C0(_, _, _, _, _, h238, _, _) => {
            h238
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, _, _, h239, _, _, _, _) => {
            h239
        }
    }), f23(match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, h240, _) => {
            h240
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, _, _, _, h241, _, _, _) => {
            h241
        }
    }), f23(match v0 {
        Adt0::C0(_, _, _, _, _, _, _, h242) => {
            h242
        }
    }, match v2 {
        Adt2::C0(_, _, _, _, _, _, _, h243, _, _) => {
            h243
        }
    })))
}

pub fn f20(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt1> {
    Ok(Adt1::C0(f24(v0.clone(), v1, v2, v3)?, f24(v0.clone(), v1, v2, {
        let a244 = v3;
        let a245 = 4u64;
        nat_add(a244, a245)?
    })?, f24(v0.clone(), v1, v2, {
        let a246 = v3;
        let a247 = 8u64;
        nat_add(a246, a247)?
    })?, f24(v0.clone(), v1, v2, {
        let a248 = v3;
        let a249 = 12u64;
        nat_add(a248, a249)?
    })?, f24(v0.clone(), v1, v2, {
        let a250 = v3;
        let a251 = 16u64;
        nat_add(a250, a251)?
    })?, f24(v0.clone(), v1, v2, {
        let a252 = v3;
        let a253 = 20u64;
        nat_add(a252, a253)?
    })?, f24(v0.clone(), v1, v2, {
        let a254 = v3;
        let a255 = 24u64;
        nat_add(a254, a255)?
    })?, f24(v0.clone(), v1, v2, {
        let a256 = v3;
        let a257 = 28u64;
        nat_add(a256, a257)?
    })?, f24(v0.clone(), v1, v2, {
        let a258 = v3;
        let a259 = 32u64;
        nat_add(a258, a259)?
    })?, f24(v0.clone(), v1, v2, {
        let a260 = v3;
        let a261 = 36u64;
        nat_add(a260, a261)?
    })?, f24(v0.clone(), v1, v2, {
        let a262 = v3;
        let a263 = 40u64;
        nat_add(a262, a263)?
    })?, f24(v0.clone(), v1, v2, {
        let a264 = v3;
        let a265 = 44u64;
        nat_add(a264, a265)?
    })?, f24(v0.clone(), v1, v2, {
        let a266 = v3;
        let a267 = 48u64;
        nat_add(a266, a267)?
    })?, f24(v0.clone(), v1, v2, {
        let a268 = v3;
        let a269 = 52u64;
        nat_add(a268, a269)?
    })?, f24(v0.clone(), v1, v2, {
        let a270 = v3;
        let a271 = 56u64;
        nat_add(a270, a271)?
    })?, f24(v0, v1, v2, {
        let a272 = v3;
        let a273 = 60u64;
        nat_add(a272, a273)?
    })?))
}

pub fn f21(v0: Fn2, p277: u64, p278: Adt2) -> R<Adt2> {
    let m276 = (p277, p278);
    {
        let mut m276 = m276;
        loop {
            let step = {
                let (v1, v2) = m276;
                let m274 = v1;
                if m274 == 0 {
                    Err::<(u64, Adt2), R<Adt2>>(Ok(v2))
                } else {
                    let v3 = m274 - 1;
                    Ok::<(u64, Adt2), R<Adt2>>((v3, {
                        let c275: Fn2 = v0.clone();
                        c275.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m276 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f22(v0: Adt2) -> R<Adt2> {
    f25(v0)
}

pub fn f23(v0: u32, v1: u32) -> u32 {
    let m281 = {
        let a279 = f26(v0);
        let a280 = f26(v1);
        fixed_u64::checked_add(a279, a280)
    };
    match m281 {
        None => {
            0u32
        }
        Some(v2) => {
            f27(v2)
        }
    }
}

pub fn f24(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u32> {
    let a284 = {
        let a282 = v3;
        let a283 = 8u64;
        nat_add(a282, a283)?
    };
    let a285 = v2;
    if nat_eq(a284, a285) {
        Ok(f28({
            let a288 = {
                let a286 = v1;
                let a287 = 8u64;
                nat_mul(a286, a287)?
            };
            let a289 = 4294967296u64;
            let a290 = 0u64;
            nat_quot(a288, a289, a290)
        }))
    } else {
        let a293 = {
            let a291 = v3;
            let a292 = 4u64;
            nat_add(a291, a292)?
        };
        let a294 = v2;
        if nat_eq(a293, a294) {
            Ok(f28({
                let a297 = {
                    let a295 = v1;
                    let a296 = 8u64;
                    nat_mul(a295, a296)?
                };
                let a298 = 4294967296u64;
                let a299 = 0u64;
                nat_rem(a297, a298, a299)
            }))
        } else {
            Ok(f29(f30(v0.clone(), v1, v3), f30(v0.clone(), v1, {
                let a300 = v3;
                let a301 = 1u64;
                nat_add(a300, a301)?
            }), f30(v0.clone(), v1, {
                let a302 = v3;
                let a303 = 2u64;
                nat_add(a302, a303)?
            }), f30(v0, v1, {
                let a304 = v3;
                let a305 = 3u64;
                nat_add(a304, a305)?
            })))
        }
    }
}

pub fn f25(v0: Adt2) -> R<Adt2> {
    let v1: u32 = f31(match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h306) => {
            h306
        }
    });
    let v2: u32 = f23(match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h307, _, _) => {
            h307
        }
    }, f23(f32(match v0.clone() {
        Adt2::C0(_, _, _, _, h308, _, _, _, _, _) => {
            h308
        }
    }), f23(f33(match v0.clone() {
        Adt2::C0(_, _, _, _, h309, _, _, _, _, _) => {
            h309
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h310, _, _, _, _) => {
            h310
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h311, _, _, _) => {
            h311
        }
    }), f23(v1, {
        let m314 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, h313, _) => {
                h313
            }
        };
        match m314 {
            Adt1::C0(h312, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h312
            }
        }
    }))));
    let v3: u32 = f23(f34(match v0.clone() {
        Adt2::C0(h315, _, _, _, _, _, _, _, _, _) => {
            h315
        }
    }), f35(match v0.clone() {
        Adt2::C0(h316, _, _, _, _, _, _, _, _, _) => {
            h316
        }
    }, match v0.clone() {
        Adt2::C0(_, h317, _, _, _, _, _, _, _, _) => {
            h317
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h318, _, _, _, _, _, _, _) => {
            h318
        }
    }));
    Ok(Adt2::C0(f23(v2, v3), match v0.clone() {
        Adt2::C0(h319, _, _, _, _, _, _, _, _, _) => {
            h319
        }
    }, match v0.clone() {
        Adt2::C0(_, h320, _, _, _, _, _, _, _, _) => {
            h320
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h321, _, _, _, _, _, _, _) => {
            h321
        }
    }, f23(match v0.clone() {
        Adt2::C0(_, _, _, h322, _, _, _, _, _, _) => {
            h322
        }
    }, v2), match v0.clone() {
        Adt2::C0(_, _, _, _, h323, _, _, _, _, _) => {
            h323
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h324, _, _, _, _) => {
            h324
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h325, _, _, _) => {
            h325
        }
    }, f36(match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, h326, _) => {
            h326
        }
    }), {
        let a328 = match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h327) => {
                h327
            }
        };
        let a329 = 1u64;
        nat_add(a328, a329)?
    }))
}

pub fn f26(v0: u32) -> u64 {
    let m331 = {
        let a330 = v0;
        fixed_u64::convert(i128::from(a330))
    };
    match m331 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f27(v0: u64) -> u32 {
    let m335 = {
        let a334 = {
            let a332 = v0;
            let a333 = 4294967295u64;
            fixed_u64::bit_and(a332, a333)
        };
        fixed_u32::convert(i128::from(a334))
    };
    match m335 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f28(v0: u64) -> u32 {
    let (_, h336) = {
        let (_, h337) = f37(Fn3::F38, 32u64, (v0, (1u32, 0u32)));
        h337
    };
    h336
}

pub fn f29(v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
    let a342 = f39(v0, 24u32);
    let a343 = {
        let a340 = f39(v1, 16u32);
        let a341 = {
            let a338 = f39(v2, 8u32);
            let a339 = v3;
            fixed_u32::bit_or(a338, a339)
        };
        fixed_u32::bit_or(a340, a341)
    };
    fixed_u32::bit_or(a342, a343)
}

pub fn f30(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a344 = v2;
    let a345 = v1;
    if nat_lt(a344, a345) {
        let m348 = {
            let a346 = v0;
            let a347 = v2;
            index_bytes(a346, a347)
        };
        match m348 {
            None => {
                0u32
            }
            Some(v3) => {
                let m350 = {
                    let a349 = v3;
                    fixed_u32::convert(i128::from(a349))
                };
                match m350 {
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
        let a351 = v2;
        let a352 = v1;
        if nat_eq(a351, a352) {
            128u32
        } else {
            0u32
        }
    }
}

pub fn f31(v0: u64) -> u32 {
    let a353 = v0;
    let a354 = 32u64;
    if nat_lt(a353, a354) {
        let a355 = v0;
        let a356 = 16u64;
        if nat_lt(a355, a356) {
            let a357 = v0;
            let a358 = 8u64;
            if nat_lt(a357, a358) {
                let a359 = v0;
                let a360 = 4u64;
                if nat_lt(a359, a360) {
                    let a361 = v0;
                    let a362 = 2u64;
                    if nat_lt(a361, a362) {
                        let a363 = v0;
                        let a364 = 1u64;
                        if nat_lt(a363, a364) {
                            1116352408u32
                        } else {
                            1899447441u32
                        }
                    } else {
                        let a365 = v0;
                        let a366 = 3u64;
                        if nat_lt(a365, a366) {
                            3049323471u32
                        } else {
                            3921009573u32
                        }
                    }
                } else {
                    let a367 = v0;
                    let a368 = 6u64;
                    if nat_lt(a367, a368) {
                        let a369 = v0;
                        let a370 = 5u64;
                        if nat_lt(a369, a370) {
                            961987163u32
                        } else {
                            1508970993u32
                        }
                    } else {
                        let a371 = v0;
                        let a372 = 7u64;
                        if nat_lt(a371, a372) {
                            2453635748u32
                        } else {
                            2870763221u32
                        }
                    }
                }
            } else {
                let a373 = v0;
                let a374 = 12u64;
                if nat_lt(a373, a374) {
                    let a375 = v0;
                    let a376 = 10u64;
                    if nat_lt(a375, a376) {
                        let a377 = v0;
                        let a378 = 9u64;
                        if nat_lt(a377, a378) {
                            3624381080u32
                        } else {
                            310598401u32
                        }
                    } else {
                        let a379 = v0;
                        let a380 = 11u64;
                        if nat_lt(a379, a380) {
                            607225278u32
                        } else {
                            1426881987u32
                        }
                    }
                } else {
                    let a381 = v0;
                    let a382 = 14u64;
                    if nat_lt(a381, a382) {
                        let a383 = v0;
                        let a384 = 13u64;
                        if nat_lt(a383, a384) {
                            1925078388u32
                        } else {
                            2162078206u32
                        }
                    } else {
                        let a385 = v0;
                        let a386 = 15u64;
                        if nat_lt(a385, a386) {
                            2614888103u32
                        } else {
                            3248222580u32
                        }
                    }
                }
            }
        } else {
            let a387 = v0;
            let a388 = 24u64;
            if nat_lt(a387, a388) {
                let a389 = v0;
                let a390 = 20u64;
                if nat_lt(a389, a390) {
                    let a391 = v0;
                    let a392 = 18u64;
                    if nat_lt(a391, a392) {
                        let a393 = v0;
                        let a394 = 17u64;
                        if nat_lt(a393, a394) {
                            3835390401u32
                        } else {
                            4022224774u32
                        }
                    } else {
                        let a395 = v0;
                        let a396 = 19u64;
                        if nat_lt(a395, a396) {
                            264347078u32
                        } else {
                            604807628u32
                        }
                    }
                } else {
                    let a397 = v0;
                    let a398 = 22u64;
                    if nat_lt(a397, a398) {
                        let a399 = v0;
                        let a400 = 21u64;
                        if nat_lt(a399, a400) {
                            770255983u32
                        } else {
                            1249150122u32
                        }
                    } else {
                        let a401 = v0;
                        let a402 = 23u64;
                        if nat_lt(a401, a402) {
                            1555081692u32
                        } else {
                            1996064986u32
                        }
                    }
                }
            } else {
                let a403 = v0;
                let a404 = 28u64;
                if nat_lt(a403, a404) {
                    let a405 = v0;
                    let a406 = 26u64;
                    if nat_lt(a405, a406) {
                        let a407 = v0;
                        let a408 = 25u64;
                        if nat_lt(a407, a408) {
                            2554220882u32
                        } else {
                            2821834349u32
                        }
                    } else {
                        let a409 = v0;
                        let a410 = 27u64;
                        if nat_lt(a409, a410) {
                            2952996808u32
                        } else {
                            3210313671u32
                        }
                    }
                } else {
                    let a411 = v0;
                    let a412 = 30u64;
                    if nat_lt(a411, a412) {
                        let a413 = v0;
                        let a414 = 29u64;
                        if nat_lt(a413, a414) {
                            3336571891u32
                        } else {
                            3584528711u32
                        }
                    } else {
                        let a415 = v0;
                        let a416 = 31u64;
                        if nat_lt(a415, a416) {
                            113926993u32
                        } else {
                            338241895u32
                        }
                    }
                }
            }
        }
    } else {
        let a417 = v0;
        let a418 = 48u64;
        if nat_lt(a417, a418) {
            let a419 = v0;
            let a420 = 40u64;
            if nat_lt(a419, a420) {
                let a421 = v0;
                let a422 = 36u64;
                if nat_lt(a421, a422) {
                    let a423 = v0;
                    let a424 = 34u64;
                    if nat_lt(a423, a424) {
                        let a425 = v0;
                        let a426 = 33u64;
                        if nat_lt(a425, a426) {
                            666307205u32
                        } else {
                            773529912u32
                        }
                    } else {
                        let a427 = v0;
                        let a428 = 35u64;
                        if nat_lt(a427, a428) {
                            1294757372u32
                        } else {
                            1396182291u32
                        }
                    }
                } else {
                    let a429 = v0;
                    let a430 = 38u64;
                    if nat_lt(a429, a430) {
                        let a431 = v0;
                        let a432 = 37u64;
                        if nat_lt(a431, a432) {
                            1695183700u32
                        } else {
                            1986661051u32
                        }
                    } else {
                        let a433 = v0;
                        let a434 = 39u64;
                        if nat_lt(a433, a434) {
                            2177026350u32
                        } else {
                            2456956037u32
                        }
                    }
                }
            } else {
                let a435 = v0;
                let a436 = 44u64;
                if nat_lt(a435, a436) {
                    let a437 = v0;
                    let a438 = 42u64;
                    if nat_lt(a437, a438) {
                        let a439 = v0;
                        let a440 = 41u64;
                        if nat_lt(a439, a440) {
                            2730485921u32
                        } else {
                            2820302411u32
                        }
                    } else {
                        let a441 = v0;
                        let a442 = 43u64;
                        if nat_lt(a441, a442) {
                            3259730800u32
                        } else {
                            3345764771u32
                        }
                    }
                } else {
                    let a443 = v0;
                    let a444 = 46u64;
                    if nat_lt(a443, a444) {
                        let a445 = v0;
                        let a446 = 45u64;
                        if nat_lt(a445, a446) {
                            3516065817u32
                        } else {
                            3600352804u32
                        }
                    } else {
                        let a447 = v0;
                        let a448 = 47u64;
                        if nat_lt(a447, a448) {
                            4094571909u32
                        } else {
                            275423344u32
                        }
                    }
                }
            }
        } else {
            let a449 = v0;
            let a450 = 56u64;
            if nat_lt(a449, a450) {
                let a451 = v0;
                let a452 = 52u64;
                if nat_lt(a451, a452) {
                    let a453 = v0;
                    let a454 = 50u64;
                    if nat_lt(a453, a454) {
                        let a455 = v0;
                        let a456 = 49u64;
                        if nat_lt(a455, a456) {
                            430227734u32
                        } else {
                            506948616u32
                        }
                    } else {
                        let a457 = v0;
                        let a458 = 51u64;
                        if nat_lt(a457, a458) {
                            659060556u32
                        } else {
                            883997877u32
                        }
                    }
                } else {
                    let a459 = v0;
                    let a460 = 54u64;
                    if nat_lt(a459, a460) {
                        let a461 = v0;
                        let a462 = 53u64;
                        if nat_lt(a461, a462) {
                            958139571u32
                        } else {
                            1322822218u32
                        }
                    } else {
                        let a463 = v0;
                        let a464 = 55u64;
                        if nat_lt(a463, a464) {
                            1537002063u32
                        } else {
                            1747873779u32
                        }
                    }
                }
            } else {
                let a465 = v0;
                let a466 = 60u64;
                if nat_lt(a465, a466) {
                    let a467 = v0;
                    let a468 = 58u64;
                    if nat_lt(a467, a468) {
                        let a469 = v0;
                        let a470 = 57u64;
                        if nat_lt(a469, a470) {
                            1955562222u32
                        } else {
                            2024104815u32
                        }
                    } else {
                        let a471 = v0;
                        let a472 = 59u64;
                        if nat_lt(a471, a472) {
                            2227730452u32
                        } else {
                            2361852424u32
                        }
                    }
                } else {
                    let a473 = v0;
                    let a474 = 62u64;
                    if nat_lt(a473, a474) {
                        let a475 = v0;
                        let a476 = 61u64;
                        if nat_lt(a475, a476) {
                            2428436474u32
                        } else {
                            2756734187u32
                        }
                    } else {
                        let a477 = v0;
                        let a478 = 63u64;
                        if nat_lt(a477, a478) {
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
    let a481 = {
        let a479 = f40(v0, 6u32, 26u32);
        let a480 = f40(v0, 11u32, 21u32);
        fixed_u32::bit_xor(a479, a480)
    };
    let a482 = f40(v0, 25u32, 7u32);
    fixed_u32::bit_xor(a481, a482)
}

pub fn f33(v0: u32, v1: u32, v2: u32) -> u32 {
    let a488 = {
        let a483 = v0;
        let a484 = v1;
        fixed_u32::bit_and(a483, a484)
    };
    let a489 = {
        let a486 = {
            let a485 = v0;
            fixed_u32::bit_not(a485)
        };
        let a487 = v2;
        fixed_u32::bit_and(a486, a487)
    };
    fixed_u32::bit_xor(a488, a489)
}

pub fn f34(v0: u32) -> u32 {
    let a492 = {
        let a490 = f40(v0, 2u32, 30u32);
        let a491 = f40(v0, 13u32, 19u32);
        fixed_u32::bit_xor(a490, a491)
    };
    let a493 = f40(v0, 22u32, 10u32);
    fixed_u32::bit_xor(a492, a493)
}

pub fn f35(v0: u32, v1: u32, v2: u32) -> u32 {
    let a502 = {
        let a498 = {
            let a494 = v0;
            let a495 = v1;
            fixed_u32::bit_and(a494, a495)
        };
        let a499 = {
            let a496 = v0;
            let a497 = v2;
            fixed_u32::bit_and(a496, a497)
        };
        fixed_u32::bit_xor(a498, a499)
    };
    let a503 = {
        let a500 = v1;
        let a501 = v2;
        fixed_u32::bit_and(a500, a501)
    };
    fixed_u32::bit_xor(a502, a503)
}

pub fn f36(v0: Adt1) -> Adt1 {
    Adt1::C0(match v0.clone() {
        Adt1::C0(_, h504, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h504
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h505, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h505
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h506, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h506
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h507, _, _, _, _, _, _, _, _, _, _, _) => {
            h507
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, h508, _, _, _, _, _, _, _, _, _, _) => {
            h508
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h509, _, _, _, _, _, _, _, _, _) => {
            h509
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h510, _, _, _, _, _, _, _, _) => {
            h510
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, h511, _, _, _, _, _, _, _) => {
            h511
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, h512, _, _, _, _, _, _) => {
            h512
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, h513, _, _, _, _, _) => {
            h513
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, h514, _, _, _, _) => {
            h514
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, h515, _, _, _) => {
            h515
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h516, _, _) => {
            h516
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h517, _) => {
            h517
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h518) => {
            h518
        }
    }, f23(f41(match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h519, _) => {
            h519
        }
    }), f23(match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, h520, _, _, _, _, _, _) => {
            h520
        }
    }, f23(f42(match v0.clone() {
        Adt1::C0(_, h521, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h521
        }
    }), match v0 {
        Adt1::C0(h522, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h522
        }
    }))))
}

pub fn f37(v0: Fn3, p526: u64, p527: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m525 = (p526, p527);
    {
        let mut m525 = m525;
        loop {
            let step = {
                let (v1, v2) = m525;
                let m523 = v1;
                if m523 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m523 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c524: Fn3 = v0.clone();
                        c524.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m525 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f38(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h528, _) = v0;
        h528
    };
    let v2: u32 = {
        let (h529, _) = {
            let (_, h530) = v0;
            h530
        };
        h529
    };
    let v3: u32 = {
        let (_, h531) = {
            let (_, h532) = v0;
            h532
        };
        h531
    };
    ({
        let a533 = v1;
        let a534 = 2u64;
        let a535 = 0u64;
        nat_quot(a533, a534, a535)
    }, (f39(v2, 1u32), {
        let a539 = {
            let a536 = v1;
            let a537 = 2u64;
            let a538 = 0u64;
            nat_rem(a536, a537, a538)
        };
        let a540 = 1u64;
        if nat_eq(a539, a540) {
            let a541 = v3;
            let a542 = v2;
            fixed_u32::bit_or(a541, a542)
        } else {
            v3
        }
    }))
}

pub fn f39(v0: u32, v1: u32) -> u32 {
    let m545 = {
        let a543 = v0;
        let a544 = v1;
        fixed_u32::shift_left(a543, a544)
    };
    match m545 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f40(v0: u32, v1: u32, v2: u32) -> u32 {
    let a546 = f18(v0, v1);
    let a547 = f39(v0, v2);
    fixed_u32::bit_or(a546, a547)
}

pub fn f41(v0: u32) -> u32 {
    let a550 = {
        let a548 = f40(v0, 17u32, 15u32);
        let a549 = f40(v0, 19u32, 13u32);
        fixed_u32::bit_xor(a548, a549)
    };
    let a551 = f18(v0, 10u32);
    fixed_u32::bit_xor(a550, a551)
}

pub fn f42(v0: u32) -> u32 {
    let a554 = {
        let a552 = f40(v0, 7u32, 25u32);
        let a553 = f40(v0, 18u32, 14u32);
        fixed_u32::bit_xor(a552, a553)
    };
    let a555 = f18(v0, 3u32);
    fixed_u32::bit_xor(a554, a555)
}
