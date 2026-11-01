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
    C0(u64, Bytes, Bytes, bool),
}

#[derive(Clone)]
pub enum Adt3 {
    C0(bool, bool, List<(Bytes, Bytes)>),
}

#[derive(Clone)]
pub enum Fn0 {
    F3(Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: Adt0) -> R<Option<Adt0>> {
        match self {
            Fn0::F3(k0) => f3(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F8(Bytes),
    F40(Bytes),
    F59(u64, u64),
}

impl Fn1 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn1::F8(k0) => f8(k0.clone(), p0),
            Fn1::F40(k0) => f40(k0.clone(), p0),
            Fn1::F59(k0, k1) => f59(*k0, *k1, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F25(Bytes),
}

impl Fn2 {
    pub fn apply(&self, p0: Adt2) -> R<Option<Adt2>> {
        match self {
            Fn2::F25(k0) => f25(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F28(Bytes),
}

impl Fn3 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn3::F28(k0) => f28(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F36(Bytes, Bytes),
}

impl Fn4 {
    pub fn apply(&self, p0: Adt3, p1: (Bytes, Bytes)) -> Adt3 {
        match self {
            Fn4::F36(k0, k1) => f36(k0.clone(), k1.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F42,
    F43,
}

impl Fn5 {
    pub fn apply(&self, p0: Option<Bytes>, p1: (Bytes, Bytes)) -> R<Option<Bytes>> {
        match self {
            Fn5::F42 => f42(p0, p1),
            Fn5::F43 => Ok(f43(p0, p1)),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F50,
}

impl Fn6 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn6::F50 => f50(p0, p1),
        }
    }
}

pub fn f0(v0: Bytes) -> R<Option<Bytes>> {
    let m1 = f1(v0)?;
    match m1 {
        Ok(v1) => {
            Ok(Some(v1))
        }
        Err(_) => {
            Ok(None::<Bytes>)
        }
    }
}

pub fn f1(v0: Bytes) -> R<Result<Bytes, Bytes>> {
    let v1: Adt0 = {
        let (h2, _) = f2(Fn0::F3(v0.clone()), {
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
                Ok(Err::<Bytes, Bytes>(f5(Str::lit("unexpected end of input"))))
            }
            Some(v2) => {
                let a15 = f4(v0.clone(), match v1 {
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
                    Ok(Err::<Bytes, Bytes>(f5(Str::lit("trailing input"))))
                }
            }
        }
    }
}

pub fn f2(v0: Fn0, p21: u64, p22: Adt0) -> R<(Adt0, bool)> {
    let m20 = (p21, p22);
    {
        let mut m20 = m20;
        loop {
            let step = {
                let (v1, v2) = m20;
                let m17 = v1;
                if m17 == 0 {
                    Err::<(u64, Adt0), R<(Adt0, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m17 - 1;
                    let m19 = {
                        let c18: Fn0 = v0.clone();
                        c18.apply(v2.clone())?
                    };
                    match m19 {
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
                Ok(next) => { m20 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f3(v0: Bytes, v1: Adt0) -> R<Option<Adt0>> {
    f6(v0, v1)
}

pub fn f4(v0: Bytes, v1: u64) -> R<u64> {
    let (h23, _) = f7(Fn1::F8(v0.clone()), {
        let a24 = v0;
        length_bytes(a24)
    }, v1)?;
    Ok(h23)
}

pub fn f5(v0: Str) -> Bytes {
    let a25 = v0;
    utf8_encode(a25)
}

pub fn f6(v0: Bytes, v1: Adt0) -> R<Option<Adt0>> {
    let a28 = 0u64;
    let a29 = {
        let a27 = match v1.clone() {
            Adt0::C0(_, _, _, h26) => {
                h26
            }
        };
        length_bytes(a27)
    };
    if nat_lt(a28, a29) {
        Ok(None::<Adt0>)
    } else {
        let m31 = match v1.clone() {
            Adt0::C0(_, _, h30, _) => {
                h30
            }
        };
        match m31 {
            None => {
                let v3: u64 = f4(v0.clone(), match v1.clone() {
                    Adt0::C0(h32, _, _, _) => {
                        h32
                    }
                })?;
                let v4: u64 = f9(v0.clone(), v3)?;
                let m34 = match v1.clone() {
                    Adt0::C0(_, h33, _, _) => {
                        h33
                    }
                };
                match m34.uncons() {
                    None => {
                        Ok(Some(f10(v1, v0, v3, v4)?))
                    }
                    Some((v5, v6)) => {
                        let v7: u64 = match v5.clone() {
                            Adt1::C0(_, h35, _, _) => {
                                h35
                            }
                        };
                        let v8: bool = if match v5.clone() {
                            Adt1::C0(h36, _, _, _) => {
                                h36
                            }
                        } {
                            let a37 = v7;
                            let a38 = 2u64;
                            nat_eq(a37, a38)
                        } else {
                            let a39 = v7;
                            let a40 = 2u64;
                            if nat_eq(a39, a40) {
                                true
                            } else {
                                let a41 = v7;
                                let a42 = 0u64;
                                if nat_eq(a41, a42) {
                                    let a45 = {
                                        let a43 = v4;
                                        let a44 = 93u64;
                                        nat_eq(a43, a44)
                                    };
                                    bool_not(a45)
                                } else {
                                    false
                                }
                            }
                        };
                        if v8 {
                            Ok(Some(f10(v1, v0, v3, v4)?))
                        } else {
                            Ok(Some(f11(v1, v0, v5, v6, v3, v4)?))
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

pub fn f7(v0: Fn1, p50: u64, p51: u64) -> R<(u64, bool)> {
    let m49 = (p50, p51);
    {
        let mut m49 = m49;
        loop {
            let step = {
                let (v1, v2) = m49;
                let m46 = v1;
                if m46 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m46 - 1;
                    let m48 = {
                        let c47: Fn1 = v0.clone();
                        c47.apply(v2)?
                    };
                    match m48 {
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
                Ok(next) => { m49 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f8(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f12(f9(v0, v1)?) {
        Ok(Some({
            let a52 = v1;
            let a53 = 1u64;
            nat_add(a52, a53)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f9(v0: Bytes, v1: u64) -> R<u64> {
    let m56 = {
        let a54 = v0;
        let a55 = v1;
        index_bytes(a54, a55)
    };
    match m56 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a101 = {
                let a77 = {
                    let a65 = {
                        let a59 = {
                            let a57 = v3;
                            let a58 = 128u8;
                            fixed_u8::bit_and(a57, a58)
                        };
                        let a60 = 128u8;
                        if equal(a59, a60) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a66 = {
                        let a63 = {
                            let a61 = v3;
                            let a62 = 64u8;
                            fixed_u8::bit_and(a61, a62)
                        };
                        let a64 = 64u8;
                        if equal(a63, a64) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a65, a66)?
                };
                let a78 = {
                    let a75 = {
                        let a69 = {
                            let a67 = v3;
                            let a68 = 32u8;
                            fixed_u8::bit_and(a67, a68)
                        };
                        let a70 = 32u8;
                        if equal(a69, a70) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a76 = {
                        let a73 = {
                            let a71 = v3;
                            let a72 = 16u8;
                            fixed_u8::bit_and(a71, a72)
                        };
                        let a74 = 16u8;
                        if equal(a73, a74) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a75, a76)?
                };
                nat_add(a77, a78)?
            };
            let a102 = {
                let a99 = {
                    let a87 = {
                        let a81 = {
                            let a79 = v3;
                            let a80 = 8u8;
                            fixed_u8::bit_and(a79, a80)
                        };
                        let a82 = 8u8;
                        if equal(a81, a82) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a88 = {
                        let a85 = {
                            let a83 = v3;
                            let a84 = 4u8;
                            fixed_u8::bit_and(a83, a84)
                        };
                        let a86 = 4u8;
                        if equal(a85, a86) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a87, a88)?
                };
                let a100 = {
                    let a97 = {
                        let a91 = {
                            let a89 = v3;
                            let a90 = 2u8;
                            fixed_u8::bit_and(a89, a90)
                        };
                        let a92 = 2u8;
                        if equal(a91, a92) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a98 = {
                        let a95 = {
                            let a93 = v3;
                            let a94 = 1u8;
                            fixed_u8::bit_and(a93, a94)
                        };
                        let a96 = 1u8;
                        if equal(a95, a96) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a97, a98)?
                };
                nat_add(a99, a100)?
            };
            nat_add(a101, a102)
        }
    }
}

pub fn f10(v0: Adt0, v1: Bytes, v2: u64, v3: u64) -> R<Adt0> {
    let a103 = v3;
    let a104 = 123u64;
    if nat_eq(a103, a104) {
        Ok(f13(v0, true, {
            let a105 = v2;
            let a106 = 1u64;
            nat_add(a105, a106)?
        }))
    } else {
        let a107 = v3;
        let a108 = 91u64;
        if nat_eq(a107, a108) {
            Ok(f13(v0, false, {
                let a109 = v2;
                let a110 = 1u64;
                nat_add(a109, a110)?
            }))
        } else {
            let a111 = v3;
            let a112 = 34u64;
            if nat_eq(a111, a112) {
                let v4: Adt2 = f14(v1, v2)?;
                if match v4.clone() {
                    Adt2::C0(_, _, _, h113) => {
                        h113
                    }
                } {
                    Ok(f15(v0.clone(), match v0 {
                        Adt0::C0(_, h114, _, _) => {
                            h114
                        }
                    }, f16(match v4.clone() {
                        Adt2::C0(_, h115, _, _) => {
                            h115
                        }
                    })?, match v4 {
                        Adt2::C0(h116, _, _, _) => {
                            h116
                        }
                    }))
                } else {
                    Ok(f17(v0, match v4 {
                        Adt2::C0(_, _, h117, _) => {
                            h117
                        }
                    }))
                }
            } else {
                let a118 = v3;
                let a119 = 116u64;
                if nat_eq(a118, a119) {
                    f18(v0, v1, v2, Bytes::lit(&[0x74, 0x72, 0x75, 0x65]), Bytes::lit(&[0x74, 0x72, 0x75, 0x65]))
                } else {
                    let a120 = v3;
                    let a121 = 102u64;
                    if nat_eq(a120, a121) {
                        f18(v0, v1, v2, Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]), Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]))
                    } else {
                        let a122 = v3;
                        let a123 = 110u64;
                        if nat_eq(a122, a123) {
                            f18(v0, v1, v2, Bytes::lit(&[0x6e, 0x75, 0x6c, 0x6c]), Bytes::lit(&[0x6e, 0x75, 0x6c, 0x6c]))
                        } else {
                            let a124 = v3;
                            let a125 = 45u64;
                            if nat_eq(a124, a125) {
                                f19(v0, v1, v2)
                            } else {
                                let a126 = 47u64;
                                let a127 = v3;
                                if nat_lt(a126, a127) {
                                    let a128 = v3;
                                    let a129 = 58u64;
                                    if nat_lt(a128, a129) {
                                        f19(v0, v1, v2)
                                    } else {
                                        Ok(f17(v0, f5(Str::lit("unexpected input"))))
                                    }
                                } else {
                                    Ok(f17(v0, f5(Str::lit("unexpected input"))))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f11(v0: Adt0, v1: Bytes, v2: Adt1, v3: List<Adt1>, v4: u64, v5: u64) -> R<Adt0> {
    let v6: u64 = match v2.clone() {
        Adt1::C0(_, h130, _, _) => {
            h130
        }
    };
    if match v2.clone() {
        Adt1::C0(h131, _, _, _) => {
            h131
        }
    } {
        let a132 = v5;
        let a133 = 125u64;
        if nat_eq(a132, a133) {
            let a134 = v6;
            let a135 = 0u64;
            if nat_eq(a134, a135) {
                f20(v0, v2, v3, {
                    let a136 = v4;
                    let a137 = 1u64;
                    nat_add(a136, a137)?
                })
            } else {
                let a138 = v6;
                let a139 = 3u64;
                if nat_eq(a138, a139) {
                    f20(v0, v2, v3, {
                        let a140 = v4;
                        let a141 = 1u64;
                        nat_add(a140, a141)?
                    })
                } else {
                    Ok(f17(v0, f5(Str::lit("unexpected input"))))
                }
            }
        } else {
            let a142 = v5;
            let a143 = 34u64;
            if nat_eq(a142, a143) {
                let a144 = v6;
                let a145 = 0u64;
                if nat_eq(a144, a145) {
                    f21(v0, v1, v2, v3, v4)
                } else {
                    let a146 = v6;
                    let a147 = 4u64;
                    if nat_eq(a146, a147) {
                        f21(v0, v1, v2, v3, v4)
                    } else {
                        Ok(f17(v0, f5(Str::lit("unexpected input"))))
                    }
                }
            } else {
                let a148 = v5;
                let a149 = 58u64;
                if nat_eq(a148, a149) {
                    let a150 = v6;
                    let a151 = 1u64;
                    if nat_eq(a150, a151) {
                        Ok(f22(v0, f23(v2.clone(), 2u64, match v2 {
                            Adt1::C0(_, _, _, h152) => {
                                h152
                            }
                        }), v3, {
                            let a153 = v4;
                            let a154 = 1u64;
                            nat_add(a153, a154)?
                        }))
                    } else {
                        Ok(f17(v0, f5(Str::lit("unexpected input"))))
                    }
                } else {
                    let a155 = v5;
                    let a156 = 44u64;
                    if nat_eq(a155, a156) {
                        let a157 = v6;
                        let a158 = 3u64;
                        if nat_eq(a157, a158) {
                            Ok(f22(v0, f23(v2, 4u64, Bytes::lit(&[])), v3, {
                                let a159 = v4;
                                let a160 = 1u64;
                                nat_add(a159, a160)?
                            }))
                        } else {
                            Ok(f17(v0, f5(Str::lit("unexpected input"))))
                        }
                    } else {
                        Ok(f17(v0, f5(Str::lit("unexpected input"))))
                    }
                }
            }
        }
    } else {
        let a161 = v5;
        let a162 = 93u64;
        if nat_eq(a161, a162) {
            f20(v0, v2, v3, {
                let a163 = v4;
                let a164 = 1u64;
                nat_add(a163, a164)?
            })
        } else {
            let a165 = v5;
            let a166 = 44u64;
            if nat_eq(a165, a166) {
                let a167 = v6;
                let a168 = 1u64;
                if nat_eq(a167, a168) {
                    Ok(f22(v0, f23(v2, 2u64, Bytes::lit(&[])), v3, {
                        let a169 = v4;
                        let a170 = 1u64;
                        nat_add(a169, a170)?
                    }))
                } else {
                    Ok(f17(v0, f5(Str::lit("unexpected input"))))
                }
            } else {
                Ok(f17(v0, f5(Str::lit("unexpected input"))))
            }
        }
    }
}

pub fn f12(v0: u64) -> bool {
    let a171 = v0;
    let a172 = 32u64;
    if nat_eq(a171, a172) {
        true
    } else {
        let a173 = v0;
        let a174 = 10u64;
        if nat_eq(a173, a174) {
            true
        } else {
            let a175 = v0;
            let a176 = 13u64;
            if nat_eq(a175, a176) {
                true
            } else {
                let a177 = v0;
                let a178 = 9u64;
                nat_eq(a177, a178)
            }
        }
    }
}

pub fn f13(v0: Adt0, v1: bool, v2: u64) -> Adt0 {
    Adt0::C0(v2, List::cons(Adt1::C0(v1, 0u64, List::<(Bytes, Bytes)>::nil(), Bytes::lit(&[])), match v0 {
        Adt0::C0(_, h179, _, _) => {
            h179
        }
    }), None::<Bytes>, Bytes::lit(&[]))
}

pub fn f14(v0: Bytes, v1: u64) -> R<Adt2> {
    let (h180, _) = f24(Fn2::F25(v0.clone()), {
        let a182 = {
            let a181 = v0;
            length_bytes(a181)
        };
        let a183 = 2u64;
        nat_add(a182, a183)?
    }, Adt2::C0({
        let a184 = v1;
        let a185 = 1u64;
        nat_add(a184, a185)?
    }, Bytes::lit(&[]), Bytes::lit(&[]), false))?;
    Ok(h180)
}

pub fn f15(v0: Adt0, v1: List<Adt1>, v2: Bytes, v3: u64) -> Adt0 {
    let m186 = v1;
    match m186.uncons() {
        None => {
            Adt0::C0(v3, List::<Adt1>::nil(), Some(v2), Bytes::lit(&[]))
        }
        Some((v4, v5)) => {
            if match v4.clone() {
                Adt1::C0(h187, _, _, _) => {
                    h187
                }
            } {
                let m190 = f26(match v4.clone() {
                    Adt1::C0(_, _, _, h188) => {
                        h188
                    }
                }, v2, match v4 {
                    Adt1::C0(_, _, h189, _) => {
                        h189
                    }
                });
                match m190 {
                    None => {
                        f17(v0, f5(Str::lit("duplicate member name")))
                    }
                    Some(v6) => {
                        Adt0::C0(v3, List::cons(Adt1::C0(true, 3u64, v6, Bytes::lit(&[])), v5), None::<Bytes>, Bytes::lit(&[]))
                    }
                }
            } else {
                Adt0::C0(v3, List::cons(Adt1::C0(false, 1u64, List::cons((Bytes::lit(&[]), v2), match v4 {
                    Adt1::C0(_, _, h191, _) => {
                        h191
                    }
                }), Bytes::lit(&[])), v5), None::<Bytes>, Bytes::lit(&[]))
            }
        }
    }
}

pub fn f16(v0: Bytes) -> R<Bytes> {
    let a199 = Bytes::lit(&[0x22]);
    let a200 = {
        let a197 = {
            let (_, h192) = {
                let (h193, _) = f27(Fn3::F28(v0.clone()), {
                    let a195 = {
                        let a194 = v0;
                        length_bytes(a194)
                    };
                    let a196 = 1u64;
                    nat_add(a195, a196)?
                }, (0u64, Bytes::lit(&[])))?;
                h193
            };
            h192
        };
        let a198 = Bytes::lit(&[0x22]);
        append_bytes(a197, a198)
    };
    Ok(append_bytes(a199, a200))
}

pub fn f17(v0: Adt0, v1: Bytes) -> Adt0 {
    Adt0::C0(match v0.clone() {
        Adt0::C0(h201, _, _, _) => {
            h201
        }
    }, match v0 {
        Adt0::C0(_, h202, _, _) => {
            h202
        }
    }, None::<Bytes>, v1)
}

pub fn f18(v0: Adt0, v1: Bytes, v2: u64, v3: Bytes, v4: Bytes) -> R<Adt0> {
    if f29(v1, v3.clone(), v2) {
        Ok(f15(v0.clone(), match v0 {
            Adt0::C0(_, h203, _, _) => {
                h203
            }
        }, v4, {
            let a205 = v2;
            let a206 = {
                let a204 = v3;
                length_bytes(a204)
            };
            nat_add(a205, a206)?
        }))
    } else {
        Ok(f17(v0, f5(Str::lit("bad literal"))))
    }
}

pub fn f19(v0: Adt0, v1: Bytes, v2: u64) -> R<Adt0> {
    let v3: bool = {
        let a207 = f9(v1.clone(), v2)?;
        let a208 = 45u64;
        nat_eq(a207, a208)
    };
    let v4: u64 = if v3 {
        let a209 = v2;
        let a210 = 1u64;
        nat_add(a209, a210)?
    } else {
        v2
    };
    let v5: u64 = f9(v1.clone(), v4)?;
    let a211 = v5;
    let a212 = 48u64;
    if nat_eq(a211, a212) {
        let v6: u64 = {
            let a213 = v4;
            let a214 = 1u64;
            nat_add(a213, a214)?
        };
        f30(v0, v1, v2, v6, v3)
    } else {
        let a215 = 48u64;
        let a216 = v5;
        if nat_lt(a215, a216) {
            let a217 = v5;
            let a218 = 58u64;
            if nat_lt(a217, a218) {
                f30(v0, v1.clone(), v2, f31(v1, {
                    let a219 = v4;
                    let a220 = 1u64;
                    nat_add(a219, a220)?
                })?, false)
            } else {
                Ok(f17(v0, f5(Str::lit("bad number"))))
            }
        } else {
            Ok(f17(v0, f5(Str::lit("bad number"))))
        }
    }
}

pub fn f20(v0: Adt0, v1: Adt1, v2: List<Adt1>, v3: u64) -> R<Adt0> {
    if match v1.clone() {
        Adt1::C0(h221, _, _, _) => {
            h221
        }
    } {
        Ok(f15(v0, v2, {
            let a225 = Bytes::lit(&[0x7b]);
            let a226 = {
                let a223 = f32(match v1 {
                    Adt1::C0(_, _, h222, _) => {
                        h222
                    }
                })?;
                let a224 = Bytes::lit(&[0x7d]);
                append_bytes(a223, a224)
            };
            append_bytes(a225, a226)
        }, v3))
    } else {
        Ok(f15(v0, v2, {
            let a230 = Bytes::lit(&[0x5b]);
            let a231 = {
                let a228 = f33(match v1 {
                    Adt1::C0(_, _, h227, _) => {
                        h227
                    }
                })?;
                let a229 = Bytes::lit(&[0x5d]);
                append_bytes(a228, a229)
            };
            append_bytes(a230, a231)
        }, v3))
    }
}

pub fn f21(v0: Adt0, v1: Bytes, v2: Adt1, v3: List<Adt1>, v4: u64) -> R<Adt0> {
    let v5: Adt2 = f14(v1, v4)?;
    if match v5.clone() {
        Adt2::C0(_, _, _, h232) => {
            h232
        }
    } {
        Ok(f22(v0, f23(v2, 1u64, match v5.clone() {
            Adt2::C0(_, h233, _, _) => {
                h233
            }
        }), v3, match v5 {
            Adt2::C0(h234, _, _, _) => {
                h234
            }
        }))
    } else {
        Ok(f17(v0, match v5 {
            Adt2::C0(_, _, h235, _) => {
                h235
            }
        }))
    }
}

pub fn f22(_: Adt0, v1: Adt1, v2: List<Adt1>, v3: u64) -> Adt0 {
    Adt0::C0(v3, List::cons(v1, v2), None::<Bytes>, Bytes::lit(&[]))
}

pub fn f23(v0: Adt1, v1: u64, v2: Bytes) -> Adt1 {
    Adt1::C0(match v0.clone() {
        Adt1::C0(h236, _, _, _) => {
            h236
        }
    }, v1, match v0 {
        Adt1::C0(_, _, h237, _) => {
            h237
        }
    }, v2)
}

pub fn f24(v0: Fn2, p242: u64, p243: Adt2) -> R<(Adt2, bool)> {
    let m241 = (p242, p243);
    {
        let mut m241 = m241;
        loop {
            let step = {
                let (v1, v2) = m241;
                let m238 = v1;
                if m238 == 0 {
                    Err::<(u64, Adt2), R<(Adt2, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m238 - 1;
                    let m240 = {
                        let c239: Fn2 = v0.clone();
                        c239.apply(v2.clone())?
                    };
                    match m240 {
                        None => {
                            Err::<(u64, Adt2), R<(Adt2, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt2), R<(Adt2, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m241 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f25(v0: Bytes, v1: Adt2) -> R<Option<Adt2>> {
    f34(v0, v1)
}

pub fn f26(v0: Bytes, v1: Bytes, v2: List<(Bytes, Bytes)>) -> Option<List<(Bytes, Bytes)>> {
    let v3: Adt3 = f35(Fn4::F36(v0.clone(), v1.clone()), Adt3::C0(false, false, List::<(Bytes, Bytes)>::nil()), v2);
    if match v3.clone() {
        Adt3::C0(_, h244, _) => {
            h244
        }
    } {
        None::<List<(Bytes, Bytes)>>
    } else {
        if match v3.clone() {
            Adt3::C0(h245, _, _) => {
                h245
            }
        } {
            Some(f37(match v3 {
                Adt3::C0(_, _, h246) => {
                    h246
                }
            }))
        } else {
            Some(f37(List::cons((v0, v1), match v3 {
                Adt3::C0(_, _, h247) => {
                    h247
                }
            })))
        }
    }
}

pub fn f27(v0: Fn3, p252: u64, p253: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m251 = (p252, p253);
    {
        let mut m251 = m251;
        loop {
            let step = {
                let (v1, v2) = m251;
                let m248 = v1;
                if m248 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m248 - 1;
                    let m250 = {
                        let c249: Fn3 = v0.clone();
                        c249.apply(v2.clone())?
                    };
                    match m250 {
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
                Ok(next) => { m251 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f28(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f38(v0, v1)
}

pub fn f29(v0: Bytes, v1: Bytes, v2: u64) -> bool {
    let m258 = {
        let a255 = v0;
        let a256 = v2;
        let a257 = {
            let a254 = v1.clone();
            length_bytes(a254)
        };
        slice_bytes(a255, a256, a257)
    };
    match m258 {
        None => {
            false
        }
        Some(v3) => {
            let a259 = v3;
            let a260 = v1;
            equal(a259, a260)
        }
    }
}

pub fn f30(v0: Adt0, v1: Bytes, v2: u64, v3: u64, v4: bool) -> R<Adt0> {
    let v5: u64 = f9(v1.clone(), v3)?;
    let a261 = v5;
    let a262 = 46u64;
    if nat_eq(a261, a262) {
        Ok(f17(v0, f5(Str::lit("number outside the integer subset"))))
    } else {
        let a263 = v5;
        let a264 = 101u64;
        if nat_eq(a263, a264) {
            Ok(f17(v0, f5(Str::lit("number outside the integer subset"))))
        } else {
            let a265 = v5;
            let a266 = 69u64;
            if nat_eq(a265, a266) {
                Ok(f17(v0, f5(Str::lit("number outside the integer subset"))))
            } else {
                if v4 {
                    Ok(f17(v0, f5(Str::lit("number outside the integer subset"))))
                } else {
                    Ok(f15(v0.clone(), match v0 {
                        Adt0::C0(_, h267, _, _) => {
                            h267
                        }
                    }, f39(v1, v2, {
                        let a268 = v3;
                        let a269 = v2;
                        nat_sub(a268, a269)
                    }), v3))
                }
            }
        }
    }
}

pub fn f31(v0: Bytes, v1: u64) -> R<u64> {
    let (h270, _) = f7(Fn1::F40(v0.clone()), {
        let a271 = v0;
        length_bytes(a271)
    }, v1)?;
    Ok(h270)
}

pub fn f32(v0: List<(Bytes, Bytes)>) -> R<Bytes> {
    let m272 = f41(Fn5::F42, None::<Bytes>, v0)?;
    match m272 {
        None => {
            Ok(Bytes::lit(&[]))
        }
        Some(v1) => {
            Ok(v1)
        }
    }
}

pub fn f33(v0: List<(Bytes, Bytes)>) -> R<Bytes> {
    let m273 = f41(Fn5::F43, None::<Bytes>, v0)?;
    match m273 {
        None => {
            Ok(Bytes::lit(&[]))
        }
        Some(v1) => {
            Ok(v1)
        }
    }
}

pub fn f34(v0: Bytes, v1: Adt2) -> R<Option<Adt2>> {
    if match v1.clone() {
        Adt2::C0(_, _, _, h274) => {
            h274
        }
    } {
        Ok(None::<Adt2>)
    } else {
        let a277 = 0u64;
        let a278 = {
            let a276 = match v1.clone() {
                Adt2::C0(_, _, h275, _) => {
                    h275
                }
            };
            length_bytes(a276)
        };
        if nat_lt(a277, a278) {
            Ok(None::<Adt2>)
        } else {
            let v2: u64 = f9(v0.clone(), match v1.clone() {
                Adt2::C0(h279, _, _, _) => {
                    h279
                }
            })?;
            let a280 = v2;
            let a281 = 256u64;
            if nat_eq(a280, a281) {
                Ok(f44(v1, f5(Str::lit("unterminated string"))))
            } else {
                let a282 = v2;
                let a283 = 34u64;
                if nat_eq(a282, a283) {
                    Ok(Some(Adt2::C0({
                        let a285 = match v1.clone() {
                            Adt2::C0(h284, _, _, _) => {
                                h284
                            }
                        };
                        let a286 = 1u64;
                        nat_add(a285, a286)?
                    }, match v1 {
                        Adt2::C0(_, h287, _, _) => {
                            h287
                        }
                    }, Bytes::lit(&[]), true)))
                } else {
                    let a288 = v2;
                    let a289 = 92u64;
                    if nat_eq(a288, a289) {
                        f45(v0, v1)
                    } else {
                        let a290 = v2;
                        let a291 = 32u64;
                        if nat_lt(a290, a291) {
                            Ok(f44(v1, f5(Str::lit("control character in string"))))
                        } else {
                            let a292 = v2;
                            let a293 = 128u64;
                            if nat_lt(a292, a293) {
                                f46(v1, v2, 1u64)
                            } else {
                                f47(v0, v1, v2)
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f35(v0: Fn4, p297: Adt3, p298: List<(Bytes, Bytes)>) -> Adt3 {
    let m296 = (p297, p298);
    {
        let mut m296 = m296;
        loop {
            let step = {
                let (v1, v2) = m296;
                let m294 = v2;
                match m294.uncons() {
                    None => {
                        Err::<(Adt3, List<(Bytes, Bytes)>), Adt3>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt3, List<(Bytes, Bytes)>), Adt3>(({
                            let c295: Fn4 = v0.clone();
                            c295.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m296 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f36(v0: Bytes, v1: Bytes, v2: Adt3, v3: (Bytes, Bytes)) -> Adt3 {
    if match v2.clone() {
        Adt3::C0(h299, _, _) => {
            h299
        }
    } {
        Adt3::C0(true, match v2.clone() {
            Adt3::C0(_, h300, _) => {
                h300
            }
        }, List::cons(v3, match v2 {
            Adt3::C0(_, _, h301) => {
                h301
            }
        }))
    } else {
        let a303 = v0.clone();
        let a304 = {
            let (h302, _) = v3.clone();
            h302
        };
        if equal(a303, a304) {
            Adt3::C0(true, true, List::cons(v3, match v2 {
                Adt3::C0(_, _, h305) => {
                    h305
                }
            }))
        } else {
            if f48(v0.clone(), {
                let (h306, _) = v3.clone();
                h306
            }) {
                Adt3::C0(true, false, List::cons(v3, List::cons((v0, v1), match v2 {
                    Adt3::C0(_, _, h307) => {
                        h307
                    }
                })))
            } else {
                Adt3::C0(false, false, List::cons(v3, match v2 {
                    Adt3::C0(_, _, h308) => {
                        h308
                    }
                }))
            }
        }
    }
}

pub fn f37(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f49(Fn6::F50, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f38(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let v2: u64 = {
        let (h309, _) = v1;
        h309
    };
    let v3: u64 = f9(v0, v2)?;
    let a310 = v3;
    let a311 = 256u64;
    if nat_eq(a310, a311) {
        Ok(None::<(u64, Bytes)>)
    } else {
        Ok(Some(({
            let a312 = v2;
            let a313 = 1u64;
            nat_add(a312, a313)?
        }, {
            let a341 = {
                let (_, h314) = v1;
                h314
            };
            let a342 = {
                let a315 = v3;
                let a316 = 34u64;
                if nat_eq(a315, a316) {
                    Bytes::lit(&[0x5c, 0x22])
                } else {
                    let a317 = v3;
                    let a318 = 92u64;
                    if nat_eq(a317, a318) {
                        Bytes::lit(&[0x5c, 0x5c])
                    } else {
                        let a319 = v3;
                        let a320 = 8u64;
                        if nat_eq(a319, a320) {
                            Bytes::lit(&[0x5c, 0x62])
                        } else {
                            let a321 = v3;
                            let a322 = 9u64;
                            if nat_eq(a321, a322) {
                                Bytes::lit(&[0x5c, 0x74])
                            } else {
                                let a323 = v3;
                                let a324 = 10u64;
                                if nat_eq(a323, a324) {
                                    Bytes::lit(&[0x5c, 0x6e])
                                } else {
                                    let a325 = v3;
                                    let a326 = 12u64;
                                    if nat_eq(a325, a326) {
                                        Bytes::lit(&[0x5c, 0x66])
                                    } else {
                                        let a327 = v3;
                                        let a328 = 13u64;
                                        if nat_eq(a327, a328) {
                                            Bytes::lit(&[0x5c, 0x72])
                                        } else {
                                            let a329 = v3;
                                            let a330 = 32u64;
                                            if nat_lt(a329, a330) {
                                                let a339 = Bytes::lit(&[0x5c, 0x75, 0x30, 0x30]);
                                                let a340 = {
                                                    let a337 = f39(Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66]), {
                                                        let a331 = v3;
                                                        let a332 = 16u64;
                                                        let a333 = 0u64;
                                                        nat_quot(a331, a332, a333)
                                                    }, 1u64);
                                                    let a338 = f39(Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66]), {
                                                        let a334 = v3;
                                                        let a335 = 16u64;
                                                        let a336 = 0u64;
                                                        nat_rem(a334, a335, a336)
                                                    }, 1u64);
                                                    append_bytes(a337, a338)
                                                };
                                                append_bytes(a339, a340)
                                            } else {
                                                f51(v3)
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            };
            append_bytes(a341, a342)
        })))
    }
}

pub fn f39(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m346 = {
        let a343 = v0;
        let a344 = v1;
        let a345 = v2;
        slice_bytes(a343, a344, a345)
    };
    match m346 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f40(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f9(v0, v1)?;
    let a347 = 47u64;
    let a348 = v2;
    if nat_lt(a347, a348) {
        let a349 = v2;
        let a350 = 58u64;
        if nat_lt(a349, a350) {
            Ok(Some({
                let a351 = v1;
                let a352 = 1u64;
                nat_add(a351, a352)?
            }))
        } else {
            Ok(None::<u64>)
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f41(v0: Fn5, p356: Option<Bytes>, p357: List<(Bytes, Bytes)>) -> R<Option<Bytes>> {
    let m355 = (p356, p357);
    {
        let mut m355 = m355;
        loop {
            let step = {
                let (v1, v2) = m355;
                let m353 = v2;
                match m353.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<(Bytes, Bytes)>), R<Option<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<(Bytes, Bytes)>), R<Option<Bytes>>>(({
                            let c354: Fn5 = v0.clone();
                            c354.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m355 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f42(v0: Option<Bytes>, v1: (Bytes, Bytes)) -> R<Option<Bytes>> {
    let v2: Bytes = {
        let a362 = f16({
            let (h358, _) = v1;
            h358
        })?;
        let a363 = {
            let a360 = Bytes::lit(&[0x3a]);
            let a361 = {
                let (_, h359) = v1;
                h359
            };
            append_bytes(a360, a361)
        };
        append_bytes(a362, a363)
    };
    let m364 = v0;
    match m364 {
        None => {
            Ok(Some(v2))
        }
        Some(v3) => {
            Ok(Some({
                let a367 = v3;
                let a368 = {
                    let a365 = Bytes::lit(&[0x2c]);
                    let a366 = v2;
                    append_bytes(a365, a366)
                };
                append_bytes(a367, a368)
            }))
        }
    }
}

pub fn f43(v0: Option<Bytes>, v1: (Bytes, Bytes)) -> Option<Bytes> {
    let m369 = v0;
    match m369 {
        None => {
            Some({
                let (_, h370) = v1;
                h370
            })
        }
        Some(v2) => {
            Some({
                let a374 = {
                    let (_, h371) = v1;
                    h371
                };
                let a375 = {
                    let a372 = Bytes::lit(&[0x2c]);
                    let a373 = v2;
                    append_bytes(a372, a373)
                };
                append_bytes(a374, a375)
            })
        }
    }
}

pub fn f44(v0: Adt2, v1: Bytes) -> Option<Adt2> {
    Some(Adt2::C0(match v0.clone() {
        Adt2::C0(h376, _, _, _) => {
            h376
        }
    }, match v0 {
        Adt2::C0(_, h377, _, _) => {
            h377
        }
    }, v1, false))
}

pub fn f45(v0: Bytes, v1: Adt2) -> R<Option<Adt2>> {
    let v2: u64 = match v1.clone() {
        Adt2::C0(h378, _, _, _) => {
            h378
        }
    };
    let v3: u64 = f9(v0.clone(), {
        let a379 = v2;
        let a380 = 1u64;
        nat_add(a379, a380)?
    })?;
    let v4: u64 = f52(v3);
    let a381 = v4;
    let a382 = 256u64;
    if nat_lt(a381, a382) {
        f46(v1, v4, 2u64)
    } else {
        let a383 = v3;
        let a384 = 117u64;
        if nat_eq(a383, a384) {
            let v5: u64 = f53(v0, {
                let a385 = v2;
                let a386 = 2u64;
                nat_add(a385, a386)?
            })?;
            let a387 = v5;
            let a388 = 65536u64;
            if nat_lt(a387, a388) {
                f46(v1, v5, 6u64)
            } else {
                Ok(f44(v1, f5(Str::lit("bad unicode escape"))))
            }
        } else {
            Ok(f44(v1, f5(Str::lit("bad escape"))))
        }
    }
}

pub fn f46(v0: Adt2, v1: u64, v2: u64) -> R<Option<Adt2>> {
    let m390 = f54(match v0.clone() {
        Adt2::C0(_, h389, _, _) => {
            h389
        }
    }, v1)?;
    match m390 {
        None => {
            Ok(f44(v0, f5(Str::lit("string outside the NFC subset"))))
        }
        Some(v3) => {
            Ok(Some(Adt2::C0({
                let a392 = match v0 {
                    Adt2::C0(h391, _, _, _) => {
                        h391
                    }
                };
                let a393 = v2;
                nat_add(a392, a393)?
            }, v3, Bytes::lit(&[]), false)))
        }
    }
}

pub fn f47(v0: Bytes, v1: Adt2, v2: u64) -> R<Option<Adt2>> {
    let v3: u64 = f9(v0, {
        let a395 = match v1.clone() {
            Adt2::C0(h394, _, _, _) => {
                h394
            }
        };
        let a396 = 1u64;
        nat_add(a395, a396)?
    })?;
    let a397 = 127u64;
    let a398 = v3;
    if nat_lt(a397, a398) {
        let a399 = v3;
        let a400 = 192u64;
        if nat_lt(a399, a400) {
            let a401 = v2;
            let a402 = 204u64;
            if nat_eq(a401, a402) {
                f46(v1, {
                    let a405 = 768u64;
                    let a406 = {
                        let a403 = v3;
                        let a404 = 128u64;
                        nat_sub(a403, a404)
                    };
                    nat_add(a405, a406)?
                }, 2u64)
            } else {
                let a407 = v2;
                let a408 = 194u64;
                if nat_eq(a407, a408) {
                    f46(v1, {
                        let a413 = {
                            let a409 = 2u64;
                            let a410 = 64u64;
                            nat_mul(a409, a410)?
                        };
                        let a414 = {
                            let a411 = v3;
                            let a412 = 128u64;
                            nat_sub(a411, a412)
                        };
                        nat_add(a413, a414)?
                    }, 2u64)
                } else {
                    let a415 = v2;
                    let a416 = 195u64;
                    if nat_eq(a415, a416) {
                        f46(v1, {
                            let a421 = {
                                let a417 = 3u64;
                                let a418 = 64u64;
                                nat_mul(a417, a418)?
                            };
                            let a422 = {
                                let a419 = v3;
                                let a420 = 128u64;
                                nat_sub(a419, a420)
                            };
                            nat_add(a421, a422)?
                        }, 2u64)
                    } else {
                        Ok(f44(v1, f5(Str::lit("string outside the NFC subset"))))
                    }
                }
            }
        } else {
            Ok(f44(v1, f5(Str::lit("invalid UTF-8"))))
        }
    } else {
        Ok(f44(v1, f5(Str::lit("invalid UTF-8"))))
    }
}

pub fn f48(v0: Bytes, v1: Bytes) -> bool {
    let a427 = {
        let a423 = v0;
        let a424 = v1;
        compare_bytes(a423, a424)
    };
    let a428 = {
        let a425 = Bytes::lit(&[]);
        let a426 = Bytes::lit(&[0x01]);
        compare_bytes(a425, a426)
    };
    equal(a427, a428)
}

pub fn f49(v0: Fn6, p432: List<(Bytes, Bytes)>, p433: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m431 = (p432, p433);
    {
        let mut m431 = m431;
        loop {
            let step = {
                let (v1, v2) = m431;
                let m429 = v2;
                match m429.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c430: Fn6 = v0.clone();
                            c430.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m431 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f50(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn f51(v0: u64) -> Bytes {
    f39(Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff]), v0, 1u64)
}

pub fn f52(v0: u64) -> u64 {
    let a434 = v0;
    let a435 = 34u64;
    if nat_eq(a434, a435) {
        34u64
    } else {
        let a436 = v0;
        let a437 = 92u64;
        if nat_eq(a436, a437) {
            92u64
        } else {
            let a438 = v0;
            let a439 = 47u64;
            if nat_eq(a438, a439) {
                47u64
            } else {
                let a440 = v0;
                let a441 = 98u64;
                if nat_eq(a440, a441) {
                    8u64
                } else {
                    let a442 = v0;
                    let a443 = 102u64;
                    if nat_eq(a442, a443) {
                        12u64
                    } else {
                        let a444 = v0;
                        let a445 = 110u64;
                        if nat_eq(a444, a445) {
                            10u64
                        } else {
                            let a446 = v0;
                            let a447 = 114u64;
                            if nat_eq(a446, a447) {
                                13u64
                            } else {
                                let a448 = v0;
                                let a449 = 116u64;
                                if nat_eq(a448, a449) {
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

pub fn f53(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f55(f9(v0.clone(), v1)?);
    let v3: u64 = f55(f9(v0.clone(), {
        let a450 = v1;
        let a451 = 1u64;
        nat_add(a450, a451)?
    })?);
    let v4: u64 = f55(f9(v0.clone(), {
        let a452 = v1;
        let a453 = 2u64;
        nat_add(a452, a453)?
    })?);
    let v5: u64 = f55(f9(v0, {
        let a454 = v1;
        let a455 = 3u64;
        nat_add(a454, a455)?
    })?);
    let a456 = v2;
    let a457 = 16u64;
    if nat_lt(a456, a457) {
        let a458 = v3;
        let a459 = 16u64;
        if nat_lt(a458, a459) {
            let a460 = v4;
            let a461 = 16u64;
            if nat_lt(a460, a461) {
                let a462 = v5;
                let a463 = 16u64;
                if nat_lt(a462, a463) {
                    let a474 = {
                        let a472 = {
                            let a468 = {
                                let a464 = v2;
                                let a465 = 4096u64;
                                nat_mul(a464, a465)?
                            };
                            let a469 = {
                                let a466 = v3;
                                let a467 = 256u64;
                                nat_mul(a466, a467)?
                            };
                            nat_add(a468, a469)?
                        };
                        let a473 = {
                            let a470 = v4;
                            let a471 = 16u64;
                            nat_mul(a470, a471)?
                        };
                        nat_add(a472, a473)?
                    };
                    let a475 = v5;
                    nat_add(a474, a475)
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

pub fn f54(v0: Bytes, v1: u64) -> R<Option<Bytes>> {
    let a476 = v1;
    let a477 = 128u64;
    if nat_lt(a476, a477) {
        Ok(Some({
            let a478 = v0;
            let a479 = f51(v1);
            append_bytes(a478, a479)
        }))
    } else {
        let a480 = v1;
        let a481 = 256u64;
        if nat_lt(a480, a481) {
            Ok(Some({
                let a482 = v0;
                let a483 = f56(v1)?;
                append_bytes(a482, a483)
            }))
        } else {
            let a484 = 767u64;
            let a485 = v1;
            if nat_lt(a484, a485) {
                let a486 = v1;
                let a487 = 808u64;
                if nat_lt(a486, a487) {
                    let v2: u64 = {
                        let a488 = v0.clone();
                        length_bytes(a488)
                    };
                    let v3: u64 = {
                        let a489 = 0u64;
                        let a490 = v2;
                        if nat_lt(a489, a490) {
                            f9(v0.clone(), {
                                let a491 = v2;
                                let a492 = 1u64;
                                nat_sub(a491, a492)
                            })?
                        } else {
                            256u64
                        }
                    };
                    let v4: u64 = {
                        let a493 = v3;
                        let a494 = 128u64;
                        if nat_lt(a493, a494) {
                            f57({
                                let a495 = v1;
                                let a496 = 768u64;
                                nat_sub(a495, a496)
                            }, v3)?
                        } else {
                            0u64
                        }
                    };
                    let a497 = 0u64;
                    let a498 = v4;
                    if nat_lt(a497, a498) {
                        Ok(Some({
                            let a501 = f39(v0, 0u64, {
                                let a499 = v2;
                                let a500 = 1u64;
                                nat_sub(a499, a500)
                            });
                            let a502 = f56(v4)?;
                            append_bytes(a501, a502)
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

pub fn f55(v0: u64) -> u64 {
    let a503 = 47u64;
    let a504 = v0;
    if nat_lt(a503, a504) {
        let a505 = v0;
        let a506 = 58u64;
        if nat_lt(a505, a506) {
            let a507 = v0;
            let a508 = 48u64;
            nat_sub(a507, a508)
        } else {
            let a509 = 64u64;
            let a510 = v0;
            if nat_lt(a509, a510) {
                let a511 = v0;
                let a512 = 71u64;
                if nat_lt(a511, a512) {
                    let a513 = v0;
                    let a514 = 55u64;
                    nat_sub(a513, a514)
                } else {
                    let a515 = 96u64;
                    let a516 = v0;
                    if nat_lt(a515, a516) {
                        let a517 = v0;
                        let a518 = 103u64;
                        if nat_lt(a517, a518) {
                            let a519 = v0;
                            let a520 = 87u64;
                            nat_sub(a519, a520)
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

pub fn f56(v0: u64) -> R<Bytes> {
    let a531 = f51({
        let a524 = 192u64;
        let a525 = {
            let a521 = v0;
            let a522 = 64u64;
            let a523 = 0u64;
            nat_quot(a521, a522, a523)
        };
        nat_add(a524, a525)?
    });
    let a532 = f51({
        let a529 = 128u64;
        let a530 = {
            let a526 = v0;
            let a527 = 64u64;
            let a528 = 0u64;
            nat_rem(a526, a527, a528)
        };
        nat_add(a529, a530)?
    });
    Ok(append_bytes(a531, a532))
}

pub fn f57(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f58(v0, v1)?;
    let a533 = v2;
    let a534 = 53u64;
    if nat_lt(a533, a534) {
        f9(Bytes::lit(&[0x00, 0x41, 0xc0, 0x00, 0x45, 0xc8, 0x00, 0x49, 0xcc, 0x00, 0x4f, 0xd2, 0x00, 0x55, 0xd9, 0x00, 0x61, 0xe0, 0x00, 0x65, 0xe8, 0x00, 0x69, 0xec, 0x00, 0x6f, 0xf2, 0x00, 0x75, 0xf9, 0x01, 0x41, 0xc1, 0x01, 0x45, 0xc9, 0x01, 0x49, 0xcd, 0x01, 0x4f, 0xd3, 0x01, 0x55, 0xda, 0x01, 0x59, 0xdd, 0x01, 0x61, 0xe1, 0x01, 0x65, 0xe9, 0x01, 0x69, 0xed, 0x01, 0x6f, 0xf3, 0x01, 0x75, 0xfa, 0x01, 0x79, 0xfd, 0x02, 0x41, 0xc2, 0x02, 0x45, 0xca, 0x02, 0x49, 0xce, 0x02, 0x4f, 0xd4, 0x02, 0x55, 0xdb, 0x02, 0x61, 0xe2, 0x02, 0x65, 0xea, 0x02, 0x69, 0xee, 0x02, 0x6f, 0xf4, 0x02, 0x75, 0xfb, 0x03, 0x41, 0xc3, 0x03, 0x4e, 0xd1, 0x03, 0x4f, 0xd5, 0x03, 0x61, 0xe3, 0x03, 0x6e, 0xf1, 0x03, 0x6f, 0xf5, 0x08, 0x41, 0xc4, 0x08, 0x45, 0xcb, 0x08, 0x49, 0xcf, 0x08, 0x4f, 0xd6, 0x08, 0x55, 0xdc, 0x08, 0x61, 0xe4, 0x08, 0x65, 0xeb, 0x08, 0x69, 0xef, 0x08, 0x6f, 0xf6, 0x08, 0x75, 0xfc, 0x08, 0x79, 0xff, 0x0a, 0x41, 0xc5, 0x0a, 0x61, 0xe5, 0x27, 0x43, 0xc7, 0x27, 0x63, 0xe7]), {
            let a537 = {
                let a535 = v2;
                let a536 = 3u64;
                nat_mul(a535, a536)?
            };
            let a538 = 2u64;
            nat_add(a537, a538)?
        })
    } else {
        Ok(0u64)
    }
}

pub fn f58(v0: u64, v1: u64) -> R<u64> {
    let (h539, _) = f7(Fn1::F59(v1, v0), 54u64, 0u64)?;
    Ok(h539)
}

pub fn f59(v0: u64, v1: u64, v2: u64) -> R<Option<u64>> {
    let v3: Bytes = Bytes::lit(&[0x00, 0x41, 0xc0, 0x00, 0x45, 0xc8, 0x00, 0x49, 0xcc, 0x00, 0x4f, 0xd2, 0x00, 0x55, 0xd9, 0x00, 0x61, 0xe0, 0x00, 0x65, 0xe8, 0x00, 0x69, 0xec, 0x00, 0x6f, 0xf2, 0x00, 0x75, 0xf9, 0x01, 0x41, 0xc1, 0x01, 0x45, 0xc9, 0x01, 0x49, 0xcd, 0x01, 0x4f, 0xd3, 0x01, 0x55, 0xda, 0x01, 0x59, 0xdd, 0x01, 0x61, 0xe1, 0x01, 0x65, 0xe9, 0x01, 0x69, 0xed, 0x01, 0x6f, 0xf3, 0x01, 0x75, 0xfa, 0x01, 0x79, 0xfd, 0x02, 0x41, 0xc2, 0x02, 0x45, 0xca, 0x02, 0x49, 0xce, 0x02, 0x4f, 0xd4, 0x02, 0x55, 0xdb, 0x02, 0x61, 0xe2, 0x02, 0x65, 0xea, 0x02, 0x69, 0xee, 0x02, 0x6f, 0xf4, 0x02, 0x75, 0xfb, 0x03, 0x41, 0xc3, 0x03, 0x4e, 0xd1, 0x03, 0x4f, 0xd5, 0x03, 0x61, 0xe3, 0x03, 0x6e, 0xf1, 0x03, 0x6f, 0xf5, 0x08, 0x41, 0xc4, 0x08, 0x45, 0xcb, 0x08, 0x49, 0xcf, 0x08, 0x4f, 0xd6, 0x08, 0x55, 0xdc, 0x08, 0x61, 0xe4, 0x08, 0x65, 0xeb, 0x08, 0x69, 0xef, 0x08, 0x6f, 0xf6, 0x08, 0x75, 0xfc, 0x08, 0x79, 0xff, 0x0a, 0x41, 0xc5, 0x0a, 0x61, 0xe5, 0x27, 0x43, 0xc7, 0x27, 0x63, 0xe7]);
    let a540 = v2;
    let a541 = 53u64;
    if nat_lt(a540, a541) {
        let a544 = f9(v3.clone(), {
            let a542 = v2;
            let a543 = 3u64;
            nat_mul(a542, a543)?
        })?;
        let a545 = v1;
        if nat_eq(a544, a545) {
            let a550 = f9(v3, {
                let a548 = {
                    let a546 = v2;
                    let a547 = 3u64;
                    nat_mul(a546, a547)?
                };
                let a549 = 1u64;
                nat_add(a548, a549)?
            })?;
            let a551 = v0;
            if nat_eq(a550, a551) {
                Ok(None::<u64>)
            } else {
                Ok(Some({
                    let a552 = v2;
                    let a553 = 1u64;
                    nat_add(a552, a553)?
                }))
            }
        } else {
            Ok(Some({
                let a554 = v2;
                let a555 = 1u64;
                nat_add(a554, a555)?
            }))
        }
    } else {
        Ok(None::<u64>)
    }
}
