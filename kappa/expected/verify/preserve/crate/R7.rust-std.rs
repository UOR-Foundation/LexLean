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
    F28(Bytes, u64, u64),
}

impl Fn1 {
    pub fn apply(&self, p0: (u64, Adt2)) -> R<(u64, Adt2)> {
        match self {
            Fn1::F28(k0, k1, k2) => f28(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F33(Bytes, u64, u64),
}

impl Fn2 {
    pub fn apply(&self, p0: (u64, Adt0)) -> R<(u64, Adt0)> {
        match self {
            Fn2::F33(k0, k1, k2) => f33(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F39,
}

impl Fn3 {
    pub fn apply(&self, p0: (List<Adt1>, Adt1)) -> (List<Adt1>, Adt1) {
        match self {
            Fn3::F39 => f39(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F46(Bytes, u64, u64, u64),
    F104,
}

impl Fn4 {
    pub fn apply(&self, p0: (u64, Adt3)) -> R<(u64, Adt3)> {
        match self {
            Fn4::F46(k0, k1, k2, k3) => f46(k0.clone(), *k1, *k2, *k3, p0),
            Fn4::F104 => f104(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F53(Bytes, u64),
}

impl Fn5 {
    pub fn apply(&self, p0: (u64, List<Adt1>)) -> R<(u64, List<Adt1>)> {
        match self {
            Fn5::F53(k0, k1) => f53(k0.clone(), *k1, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F62,
}

impl Fn6 {
    pub fn apply(&self, p0: Adt5) -> R<Adt5> {
        match self {
            Fn6::F62 => f62(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F67,
}

impl Fn7 {
    pub fn apply(&self, p0: Adt7) -> R<Adt7> {
        match self {
            Fn7::F67 => f67(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F72,
}

impl Fn8 {
    pub fn apply(&self, p0: (Adt9, Adt8)) -> (Adt9, Adt8) {
        match self {
            Fn8::F72 => f72(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F74(u64, Bytes, u64),
}

impl Fn9 {
    pub fn apply(&self, p0: (u64, Adt1)) -> R<(u64, Adt1)> {
        match self {
            Fn9::F74(k0, k1, k2) => f74(*k0, k1.clone(), *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F78,
}

impl Fn10 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn10::F78 => f78(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F93,
}

impl Fn11 {
    pub fn apply(&self, p0: Adt10) -> Adt10 {
        match self {
            Fn11::F93 => f93(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F114,
}

impl Fn12 {
    pub fn apply(&self, p0: (u64, (u64, u64))) -> (u64, (u64, u64)) {
        match self {
            Fn12::F114 => f114(p0),
        }
    }
}

pub fn f0(v0: Bytes, v1: Bytes) -> R<Option<Bytes>> {
    if f1(v0.clone(), v1.clone())? {
        f2(v0, v1)
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f1(v0: Bytes, v1: Bytes) -> R<bool> {
    let a1 = 0u64;
    let a2 = f3(v0.clone());
    if nat_lt(a1, a2) {
        let a7 = {
            let a3 = v1.clone();
            length_bytes(a3)
        };
        let a8 = {
            let a5 = {
                let a4 = f4(v0.clone());
                length_bytes(a4)
            };
            let a6 = f3(v0.clone());
            nat_add(a5, a6)?
        };
        if nat_eq(a7, a8) {
            if f5(v1.clone(), f4(v0.clone())) {
                let a11 = f6(v1.clone(), {
                    let a9 = f4(v0);
                    length_bytes(a9)
                })?;
                let a12 = {
                    let a10 = v1;
                    length_bytes(a10)
                };
                Ok(nat_eq(a11, a12))
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

pub fn f2(v0: Bytes, v1: Bytes) -> R<Option<Bytes>> {
    let m13 = f7(v0.clone(), v1)?;
    match m13 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v2) => {
            Ok(Some({
                let a14 = f4(v0);
                let a15 = v2;
                append_bytes(a14, a15)
            }))
        }
    }
}

pub fn f3(v0: Bytes) -> u64 {
    if f8(v0.clone(), Str::lit("sha256")) {
        64u64
    } else {
        if f8(v0.clone(), Str::lit("sha3-256")) {
            64u64
        } else {
            if f8(v0.clone(), Str::lit("keccak256")) {
                64u64
            } else {
                if f8(v0.clone(), Str::lit("sha512")) {
                    128u64
                } else {
                    if f8(v0, Str::lit("blake3")) {
                        64u64
                    } else {
                        0u64
                    }
                }
            }
        }
    }
}

pub fn f4(v0: Bytes) -> Bytes {
    let a16 = v0;
    let a17 = f9(Str::lit(":"));
    append_bytes(a16, a17)
}

pub fn f5(v0: Bytes, v1: Bytes) -> bool {
    let m22 = {
        let a19 = v0;
        let a20 = 0u64;
        let a21 = {
            let a18 = v1.clone();
            length_bytes(a18)
        };
        slice_bytes(a19, a20, a21)
    };
    match m22 {
        None => {
            false
        }
        Some(v2) => {
            let a23 = v2;
            let a24 = v1;
            equal(a23, a24)
        }
    }
}

pub fn f6(v0: Bytes, v1: u64) -> R<u64> {
    let (h25, _) = f10(Fn0::F11(v0.clone()), {
        let a26 = v0;
        length_bytes(a26)
    }, v1)?;
    Ok(h25)
}

pub fn f7(v0: Bytes, v1: Bytes) -> R<Option<Bytes>> {
    if f8(v0.clone(), Str::lit("sha256")) {
        Ok(Some(f12(v1)?))
    } else {
        if f8(v0.clone(), Str::lit("sha3-256")) {
            Ok(Some(f13(v1)?))
        } else {
            if f8(v0.clone(), Str::lit("keccak256")) {
                Ok(Some(f14(v1)?))
            } else {
                if f8(v0.clone(), Str::lit("sha512")) {
                    Ok(Some(f15(v1)?))
                } else {
                    if f8(v0, Str::lit("blake3")) {
                        Ok(Some(f16(v1)?))
                    } else {
                        Ok(None::<Bytes>)
                    }
                }
            }
        }
    }
}

pub fn f8(v0: Bytes, v1: Str) -> bool {
    let a27 = v0;
    let a28 = f9(v1);
    equal(a27, a28)
}

pub fn f9(v0: Str) -> Bytes {
    let a29 = v0;
    utf8_encode(a29)
}

pub fn f10(v0: Fn0, p34: u64, p35: u64) -> R<(u64, bool)> {
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

pub fn f11(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f17(v0, v1)? {
        Ok(Some({
            let a36 = v1;
            let a37 = 1u64;
            nat_add(a36, a37)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f12(v0: Bytes) -> R<Bytes> {
    Ok(f18(f19(v0)?))
}

pub fn f13(v0: Bytes) -> R<Bytes> {
    f20(v0, 6u64)
}

pub fn f14(v0: Bytes) -> R<Bytes> {
    f20(v0, 1u64)
}

pub fn f15(v0: Bytes) -> R<Bytes> {
    let v1: Adt0 = f21(v0)?;
    let a58 = f22(match v1.clone() {
        Adt0::C0(h38, _, _, _, _, _, _, _) => {
            h38
        }
    });
    let a59 = {
        let a56 = f22(match v1.clone() {
            Adt0::C0(_, h39, _, _, _, _, _, _) => {
                h39
            }
        });
        let a57 = {
            let a54 = f22(match v1.clone() {
                Adt0::C0(_, _, h40, _, _, _, _, _) => {
                    h40
                }
            });
            let a55 = {
                let a52 = f22(match v1.clone() {
                    Adt0::C0(_, _, _, h41, _, _, _, _) => {
                        h41
                    }
                });
                let a53 = {
                    let a50 = f22(match v1.clone() {
                        Adt0::C0(_, _, _, _, h42, _, _, _) => {
                            h42
                        }
                    });
                    let a51 = {
                        let a48 = f22(match v1.clone() {
                            Adt0::C0(_, _, _, _, _, h43, _, _) => {
                                h43
                            }
                        });
                        let a49 = {
                            let a46 = f22(match v1.clone() {
                                Adt0::C0(_, _, _, _, _, _, h44, _) => {
                                    h44
                                }
                            });
                            let a47 = f22(match v1 {
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
    Ok(append_bytes(a58, a59))
}

pub fn f16(v0: Bytes) -> R<Bytes> {
    let v1: Adt1 = f23(v0)?;
    let a80 = f24(match v1.clone() {
        Adt1::C0(h60, _, _, _, _, _, _, _) => {
            h60
        }
    });
    let a81 = {
        let a78 = f24(match v1.clone() {
            Adt1::C0(_, h61, _, _, _, _, _, _) => {
                h61
            }
        });
        let a79 = {
            let a76 = f24(match v1.clone() {
                Adt1::C0(_, _, h62, _, _, _, _, _) => {
                    h62
                }
            });
            let a77 = {
                let a74 = f24(match v1.clone() {
                    Adt1::C0(_, _, _, h63, _, _, _, _) => {
                        h63
                    }
                });
                let a75 = {
                    let a72 = f24(match v1.clone() {
                        Adt1::C0(_, _, _, _, h64, _, _, _) => {
                            h64
                        }
                    });
                    let a73 = {
                        let a70 = f24(match v1.clone() {
                            Adt1::C0(_, _, _, _, _, h65, _, _) => {
                                h65
                            }
                        });
                        let a71 = {
                            let a68 = f24(match v1.clone() {
                                Adt1::C0(_, _, _, _, _, _, h66, _) => {
                                    h66
                                }
                            });
                            let a69 = f24(match v1 {
                                Adt1::C0(_, _, _, _, _, _, _, h67) => {
                                    h67
                                }
                            });
                            append_bytes(a68, a69)
                        };
                        append_bytes(a70, a71)
                    };
                    append_bytes(a72, a73)
                };
                append_bytes(a74, a75)
            };
            append_bytes(a76, a77)
        };
        append_bytes(a78, a79)
    };
    Ok(append_bytes(a80, a81))
}

pub fn f17(v0: Bytes, v1: u64) -> R<bool> {
    let v2: u64 = f25(v0, v1)?;
    let a82 = 47u64;
    let a83 = v2;
    if nat_lt(a82, a83) {
        let a84 = v2;
        let a85 = 58u64;
        if nat_lt(a84, a85) {
            Ok(true)
        } else {
            let a86 = 96u64;
            let a87 = v2;
            if nat_lt(a86, a87) {
                let a88 = v2;
                let a89 = 103u64;
                Ok(nat_lt(a88, a89))
            } else {
                Ok(false)
            }
        }
    } else {
        Ok(false)
    }
}

pub fn f18(v0: Adt2) -> Bytes {
    let a110 = f26(match v0.clone() {
        Adt2::C0(h90, _, _, _, _, _, _, _) => {
            h90
        }
    });
    let a111 = {
        let a108 = f26(match v0.clone() {
            Adt2::C0(_, h91, _, _, _, _, _, _) => {
                h91
            }
        });
        let a109 = {
            let a106 = f26(match v0.clone() {
                Adt2::C0(_, _, h92, _, _, _, _, _) => {
                    h92
                }
            });
            let a107 = {
                let a104 = f26(match v0.clone() {
                    Adt2::C0(_, _, _, h93, _, _, _, _) => {
                        h93
                    }
                });
                let a105 = {
                    let a102 = f26(match v0.clone() {
                        Adt2::C0(_, _, _, _, h94, _, _, _) => {
                            h94
                        }
                    });
                    let a103 = {
                        let a100 = f26(match v0.clone() {
                            Adt2::C0(_, _, _, _, _, h95, _, _) => {
                                h95
                            }
                        });
                        let a101 = {
                            let a98 = f26(match v0.clone() {
                                Adt2::C0(_, _, _, _, _, _, h96, _) => {
                                    h96
                                }
                            });
                            let a99 = f26(match v0 {
                                Adt2::C0(_, _, _, _, _, _, _, h97) => {
                                    h97
                                }
                            });
                            append_bytes(a98, a99)
                        };
                        append_bytes(a100, a101)
                    };
                    append_bytes(a102, a103)
                };
                append_bytes(a104, a105)
            };
            append_bytes(a106, a107)
        };
        append_bytes(a108, a109)
    };
    append_bytes(a110, a111)
}

pub fn f19(v0: Bytes) -> R<Adt2> {
    let v1: u64 = {
        let a112 = v0.clone();
        length_bytes(a112)
    };
    let v2: u64 = {
        let a118 = {
            let a115 = {
                let a113 = v1;
                let a114 = 72u64;
                nat_add(a113, a114)?
            };
            let a116 = 64u64;
            let a117 = 0u64;
            nat_quot(a115, a116, a117)
        };
        let a119 = 64u64;
        nat_mul(a118, a119)?
    };
    let (_, h120) = f27(Fn1::F28(v0, v1, v2), {
        let a121 = v2;
        let a122 = 64u64;
        let a123 = 0u64;
        nat_quot(a121, a122, a123)
    }, (0u64, f29()))?;
    Ok(h120)
}

pub fn f20(v0: Bytes, v1: u64) -> R<Bytes> {
    let v2: Adt3 = f30(v0, v1)?;
    let a132 = f31(match v2.clone() {
        Adt3::C0(h124, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h124
        }
    });
    let a133 = {
        let a130 = f31(match v2.clone() {
            Adt3::C0(_, h125, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h125
            }
        });
        let a131 = {
            let a128 = f31(match v2.clone() {
                Adt3::C0(_, _, h126, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h126
                }
            });
            let a129 = f31(match v2 {
                Adt3::C0(_, _, _, h127, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h127
                }
            });
            append_bytes(a128, a129)
        };
        append_bytes(a130, a131)
    };
    Ok(append_bytes(a132, a133))
}

pub fn f21(v0: Bytes) -> R<Adt0> {
    let v1: u64 = {
        let a134 = v0.clone();
        length_bytes(a134)
    };
    let v2: u64 = {
        let a140 = {
            let a137 = {
                let a135 = v1;
                let a136 = 144u64;
                nat_add(a135, a136)?
            };
            let a138 = 128u64;
            let a139 = 0u64;
            nat_quot(a137, a138, a139)
        };
        let a141 = 128u64;
        nat_mul(a140, a141)?
    };
    let (_, h142) = f32(Fn2::F33(v0, v1, v2), {
        let a143 = v2;
        let a144 = 128u64;
        let a145 = 0u64;
        nat_quot(a143, a144, a145)
    }, (0u64, f34()))?;
    Ok(h142)
}

pub fn f22(v0: u64) -> Bytes {
    let a174 = f35(v0, 60u32);
    let a175 = {
        let a172 = f35(v0, 56u32);
        let a173 = {
            let a170 = f35(v0, 52u32);
            let a171 = {
                let a168 = f35(v0, 48u32);
                let a169 = {
                    let a166 = f35(v0, 44u32);
                    let a167 = {
                        let a164 = f35(v0, 40u32);
                        let a165 = {
                            let a162 = f35(v0, 36u32);
                            let a163 = {
                                let a160 = f35(v0, 32u32);
                                let a161 = {
                                    let a158 = f35(v0, 28u32);
                                    let a159 = {
                                        let a156 = f35(v0, 24u32);
                                        let a157 = {
                                            let a154 = f35(v0, 20u32);
                                            let a155 = {
                                                let a152 = f35(v0, 16u32);
                                                let a153 = {
                                                    let a150 = f35(v0, 12u32);
                                                    let a151 = {
                                                        let a148 = f35(v0, 8u32);
                                                        let a149 = {
                                                            let a146 = f35(v0, 4u32);
                                                            let a147 = f35(v0, 0u32);
                                                            append_bytes(a146, a147)
                                                        };
                                                        append_bytes(a148, a149)
                                                    };
                                                    append_bytes(a150, a151)
                                                };
                                                append_bytes(a152, a153)
                                            };
                                            append_bytes(a154, a155)
                                        };
                                        append_bytes(a156, a157)
                                    };
                                    append_bytes(a158, a159)
                                };
                                append_bytes(a160, a161)
                            };
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

pub fn f23(v0: Bytes) -> R<Adt1> {
    let v1: u64 = {
        let a176 = v0.clone();
        length_bytes(a176)
    };
    let v2: u64 = {
        let a177 = v1;
        let a178 = 0u64;
        if nat_eq(a177, a178) {
            1u64
        } else {
            let a181 = {
                let a179 = v1;
                let a180 = 1023u64;
                nat_add(a179, a180)?
            };
            let a182 = 1024u64;
            let a183 = 0u64;
            nat_quot(a181, a182, a183)
        }
    };
    let v3: u64 = {
        let a184 = v2;
        let a185 = 1u64;
        nat_sub(a184, a185)
    };
    let m187 = {
        let (_, h186) = f36(v0.clone(), v1, v3)?;
        h186
    };
    match m187.uncons() {
        None => {
            f37(v0, v1, v3, true)
        }
        Some((v4, v5)) => {
            let (_, h188) = f38(Fn3::F39, 64u64, (List::cons(v4, v5), f37(v0, v1, v3, false)?));
            Ok(h188)
        }
    }
}

pub fn f24(v0: u32) -> Bytes {
    let a201 = {
        let a189 = f40(v0, 4u32);
        let a190 = f40(v0, 0u32);
        append_bytes(a189, a190)
    };
    let a202 = {
        let a199 = {
            let a191 = f40(v0, 12u32);
            let a192 = f40(v0, 8u32);
            append_bytes(a191, a192)
        };
        let a200 = {
            let a197 = {
                let a193 = f40(v0, 20u32);
                let a194 = f40(v0, 16u32);
                append_bytes(a193, a194)
            };
            let a198 = {
                let a195 = f40(v0, 28u32);
                let a196 = f40(v0, 24u32);
                append_bytes(a195, a196)
            };
            append_bytes(a197, a198)
        };
        append_bytes(a199, a200)
    };
    append_bytes(a201, a202)
}

pub fn f25(v0: Bytes, v1: u64) -> R<u64> {
    let m205 = {
        let a203 = v0;
        let a204 = v1;
        index_bytes(a203, a204)
    };
    match m205 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a250 = {
                let a226 = {
                    let a214 = {
                        let a208 = {
                            let a206 = v3;
                            let a207 = 128u8;
                            fixed_u8::bit_and(a206, a207)
                        };
                        let a209 = 128u8;
                        if equal(a208, a209) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a215 = {
                        let a212 = {
                            let a210 = v3;
                            let a211 = 64u8;
                            fixed_u8::bit_and(a210, a211)
                        };
                        let a213 = 64u8;
                        if equal(a212, a213) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a214, a215)?
                };
                let a227 = {
                    let a224 = {
                        let a218 = {
                            let a216 = v3;
                            let a217 = 32u8;
                            fixed_u8::bit_and(a216, a217)
                        };
                        let a219 = 32u8;
                        if equal(a218, a219) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a225 = {
                        let a222 = {
                            let a220 = v3;
                            let a221 = 16u8;
                            fixed_u8::bit_and(a220, a221)
                        };
                        let a223 = 16u8;
                        if equal(a222, a223) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a224, a225)?
                };
                nat_add(a226, a227)?
            };
            let a251 = {
                let a248 = {
                    let a236 = {
                        let a230 = {
                            let a228 = v3;
                            let a229 = 8u8;
                            fixed_u8::bit_and(a228, a229)
                        };
                        let a231 = 8u8;
                        if equal(a230, a231) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a237 = {
                        let a234 = {
                            let a232 = v3;
                            let a233 = 4u8;
                            fixed_u8::bit_and(a232, a233)
                        };
                        let a235 = 4u8;
                        if equal(a234, a235) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a236, a237)?
                };
                let a249 = {
                    let a246 = {
                        let a240 = {
                            let a238 = v3;
                            let a239 = 2u8;
                            fixed_u8::bit_and(a238, a239)
                        };
                        let a241 = 2u8;
                        if equal(a240, a241) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a247 = {
                        let a244 = {
                            let a242 = v3;
                            let a243 = 1u8;
                            fixed_u8::bit_and(a242, a243)
                        };
                        let a245 = 1u8;
                        if equal(a244, a245) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a246, a247)?
                };
                nat_add(a248, a249)?
            };
            nat_add(a250, a251)
        }
    }
}

pub fn f26(v0: u32) -> Bytes {
    let a280 = f41({
        let a252 = f42(v0, 28u32);
        let a253 = 15u32;
        fixed_u32::bit_and(a252, a253)
    });
    let a281 = {
        let a278 = f41({
            let a254 = f42(v0, 24u32);
            let a255 = 15u32;
            fixed_u32::bit_and(a254, a255)
        });
        let a279 = {
            let a276 = f41({
                let a256 = f42(v0, 20u32);
                let a257 = 15u32;
                fixed_u32::bit_and(a256, a257)
            });
            let a277 = {
                let a274 = f41({
                    let a258 = f42(v0, 16u32);
                    let a259 = 15u32;
                    fixed_u32::bit_and(a258, a259)
                });
                let a275 = {
                    let a272 = f41({
                        let a260 = f42(v0, 12u32);
                        let a261 = 15u32;
                        fixed_u32::bit_and(a260, a261)
                    });
                    let a273 = {
                        let a270 = f41({
                            let a262 = f42(v0, 8u32);
                            let a263 = 15u32;
                            fixed_u32::bit_and(a262, a263)
                        });
                        let a271 = {
                            let a268 = f41({
                                let a264 = f42(v0, 4u32);
                                let a265 = 15u32;
                                fixed_u32::bit_and(a264, a265)
                            });
                            let a269 = f41({
                                let a266 = v0;
                                let a267 = 15u32;
                                fixed_u32::bit_and(a266, a267)
                            });
                            append_bytes(a268, a269)
                        };
                        append_bytes(a270, a271)
                    };
                    append_bytes(a272, a273)
                };
                append_bytes(a274, a275)
            };
            append_bytes(a276, a277)
        };
        append_bytes(a278, a279)
    };
    append_bytes(a280, a281)
}

pub fn f27(v0: Fn1, p285: u64, p286: (u64, Adt2)) -> R<(u64, Adt2)> {
    let m284 = (p285, p286);
    {
        let mut m284 = m284;
        loop {
            let step = {
                let (v1, v2) = m284;
                let m282 = v1;
                if m282 == 0 {
                    Err::<(u64, (u64, Adt2)), R<(u64, Adt2)>>(Ok(v2))
                } else {
                    let v3 = m282 - 1;
                    Ok::<(u64, (u64, Adt2)), R<(u64, Adt2)>>((v3, {
                        let c283: Fn1 = v0.clone();
                        c283.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m284 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f28(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt2)) -> R<(u64, Adt2)> {
    Ok(({
        let a288 = {
            let (h287, _) = v3.clone();
            h287
        };
        let a289 = 64u64;
        nat_add(a288, a289)?
    }, f43({
        let (_, h290) = v3;
        h290
    }, f44(v0, v1, v2, {
        let (h291, _) = v3;
        h291
    })?)?))
}

pub fn f29() -> Adt2 {
    Adt2::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f30(v0: Bytes, v1: u64) -> R<Adt3> {
    let v2: u64 = {
        let a292 = v0.clone();
        length_bytes(a292)
    };
    let v3: u64 = {
        let a298 = {
            let a296 = {
                let a293 = v2;
                let a294 = 136u64;
                let a295 = 0u64;
                nat_quot(a293, a294, a295)
            };
            let a297 = 1u64;
            nat_add(a296, a297)?
        };
        let a299 = 136u64;
        nat_mul(a298, a299)?
    };
    let (_, h300) = f45(Fn4::F46(v0, v2, v1, v3), {
        let a301 = v3;
        let a302 = 136u64;
        let a303 = 0u64;
        nat_quot(a301, a302, a303)
    }, (0u64, f47()))?;
    Ok(h300)
}

pub fn f31(v0: u64) -> Bytes {
    let a332 = {
        let a304 = f35(v0, 4u32);
        let a305 = f35(v0, 0u32);
        append_bytes(a304, a305)
    };
    let a333 = {
        let a330 = {
            let a306 = f35(v0, 12u32);
            let a307 = f35(v0, 8u32);
            append_bytes(a306, a307)
        };
        let a331 = {
            let a328 = {
                let a308 = f35(v0, 20u32);
                let a309 = f35(v0, 16u32);
                append_bytes(a308, a309)
            };
            let a329 = {
                let a326 = {
                    let a310 = f35(v0, 28u32);
                    let a311 = f35(v0, 24u32);
                    append_bytes(a310, a311)
                };
                let a327 = {
                    let a324 = {
                        let a312 = f35(v0, 36u32);
                        let a313 = f35(v0, 32u32);
                        append_bytes(a312, a313)
                    };
                    let a325 = {
                        let a322 = {
                            let a314 = f35(v0, 44u32);
                            let a315 = f35(v0, 40u32);
                            append_bytes(a314, a315)
                        };
                        let a323 = {
                            let a320 = {
                                let a316 = f35(v0, 52u32);
                                let a317 = f35(v0, 48u32);
                                append_bytes(a316, a317)
                            };
                            let a321 = {
                                let a318 = f35(v0, 60u32);
                                let a319 = f35(v0, 56u32);
                                append_bytes(a318, a319)
                            };
                            append_bytes(a320, a321)
                        };
                        append_bytes(a322, a323)
                    };
                    append_bytes(a324, a325)
                };
                append_bytes(a326, a327)
            };
            append_bytes(a328, a329)
        };
        append_bytes(a330, a331)
    };
    append_bytes(a332, a333)
}

pub fn f32(v0: Fn2, p337: u64, p338: (u64, Adt0)) -> R<(u64, Adt0)> {
    let m336 = (p337, p338);
    {
        let mut m336 = m336;
        loop {
            let step = {
                let (v1, v2) = m336;
                let m334 = v1;
                if m334 == 0 {
                    Err::<(u64, (u64, Adt0)), R<(u64, Adt0)>>(Ok(v2))
                } else {
                    let v3 = m334 - 1;
                    Ok::<(u64, (u64, Adt0)), R<(u64, Adt0)>>((v3, {
                        let c335: Fn2 = v0.clone();
                        c335.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m336 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f33(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt0)) -> R<(u64, Adt0)> {
    Ok(({
        let a340 = {
            let (h339, _) = v3.clone();
            h339
        };
        let a341 = 128u64;
        nat_add(a340, a341)?
    }, f48({
        let (_, h342) = v3;
        h342
    }, f49(v0, v1, v2, {
        let (h343, _) = v3;
        h343
    })?)?))
}

pub fn f34() -> Adt0 {
    Adt0::C0(7640891576956012808u64, 13503953896175478587u64, 4354685564936845355u64, 11912009170470909681u64, 5840696475078001361u64, 11170449401992604703u64, 2270897969802886507u64, 6620516959819538809u64)
}

pub fn f35(v0: u64, v1: u32) -> Bytes {
    f41(f50(f51(v0, v1)))
}

pub fn f36(v0: Bytes, v1: u64, v2: u64) -> R<(u64, List<Adt1>)> {
    f52(Fn5::F53(v0, v1), v2, (0u64, List::<Adt1>::nil()))
}

pub fn f37(v0: Bytes, v1: u64, v2: u64, v3: bool) -> R<Adt1> {
    let v4: u64 = {
        let a344 = 1024u64;
        let a345 = v2;
        nat_mul(a344, a345)?
    };
    let v5: u64 = f54(v1, v4);
    let v6: u64 = f55(v5)?;
    let v7: u64 = {
        let a346 = v6;
        let a347 = 1u64;
        nat_sub(a346, a347)
    };
    Ok(f56(f57(v0.clone(), v4, v2, v7)?, f58(v0, {
        let a350 = v4;
        let a351 = {
            let a348 = 64u64;
            let a349 = v7;
            nat_mul(a348, a349)?
        };
        nat_add(a350, a351)?
    })?, v2, f59({
        let a354 = v5;
        let a355 = {
            let a352 = 64u64;
            let a353 = v7;
            nat_mul(a352, a353)?
        };
        nat_sub(a354, a355)
    }), {
        let a358 = {
            let a356 = v7;
            let a357 = 0u64;
            if nat_eq(a356, a357) {
                3u32
            } else {
                2u32
            }
        };
        let a359 = if v3 {
            8u32
        } else {
            0u32
        };
        fixed_u32::bit_or(a358, a359)
    }))
}

pub fn f38(v0: Fn3, p363: u64, p364: (List<Adt1>, Adt1)) -> (List<Adt1>, Adt1) {
    let m362 = (p363, p364);
    {
        let mut m362 = m362;
        loop {
            let step = {
                let (v1, v2) = m362;
                let m360 = v1;
                if m360 == 0 {
                    Err::<(u64, (List<Adt1>, Adt1)), (List<Adt1>, Adt1)>(v2)
                } else {
                    let v3 = m360 - 1;
                    Ok::<(u64, (List<Adt1>, Adt1)), (List<Adt1>, Adt1)>((v3, {
                        let c361: Fn3 = v0.clone();
                        c361.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m362 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f39(v0: (List<Adt1>, Adt1)) -> (List<Adt1>, Adt1) {
    f60(v0)
}

pub fn f40(v0: u32, v1: u32) -> Bytes {
    f41(f42(v0, v1))
}

pub fn f41(v0: u32) -> Bytes {
    let a367 = {
        let a365 = v0;
        let a366 = 8u32;
        fixed_u32::bit_and(a365, a366)
    };
    let a368 = 0u32;
    if equal(a367, a368) {
        let a371 = {
            let a369 = v0;
            let a370 = 4u32;
            fixed_u32::bit_and(a369, a370)
        };
        let a372 = 0u32;
        if equal(a371, a372) {
            let a375 = {
                let a373 = v0;
                let a374 = 2u32;
                fixed_u32::bit_and(a373, a374)
            };
            let a376 = 0u32;
            if equal(a375, a376) {
                let a379 = {
                    let a377 = v0;
                    let a378 = 1u32;
                    fixed_u32::bit_and(a377, a378)
                };
                let a380 = 0u32;
                if equal(a379, a380) {
                    Bytes::lit(&[0x30])
                } else {
                    Bytes::lit(&[0x31])
                }
            } else {
                let a383 = {
                    let a381 = v0;
                    let a382 = 1u32;
                    fixed_u32::bit_and(a381, a382)
                };
                let a384 = 0u32;
                if equal(a383, a384) {
                    Bytes::lit(&[0x32])
                } else {
                    Bytes::lit(&[0x33])
                }
            }
        } else {
            let a387 = {
                let a385 = v0;
                let a386 = 2u32;
                fixed_u32::bit_and(a385, a386)
            };
            let a388 = 0u32;
            if equal(a387, a388) {
                let a391 = {
                    let a389 = v0;
                    let a390 = 1u32;
                    fixed_u32::bit_and(a389, a390)
                };
                let a392 = 0u32;
                if equal(a391, a392) {
                    Bytes::lit(&[0x34])
                } else {
                    Bytes::lit(&[0x35])
                }
            } else {
                let a395 = {
                    let a393 = v0;
                    let a394 = 1u32;
                    fixed_u32::bit_and(a393, a394)
                };
                let a396 = 0u32;
                if equal(a395, a396) {
                    Bytes::lit(&[0x36])
                } else {
                    Bytes::lit(&[0x37])
                }
            }
        }
    } else {
        let a399 = {
            let a397 = v0;
            let a398 = 4u32;
            fixed_u32::bit_and(a397, a398)
        };
        let a400 = 0u32;
        if equal(a399, a400) {
            let a403 = {
                let a401 = v0;
                let a402 = 2u32;
                fixed_u32::bit_and(a401, a402)
            };
            let a404 = 0u32;
            if equal(a403, a404) {
                let a407 = {
                    let a405 = v0;
                    let a406 = 1u32;
                    fixed_u32::bit_and(a405, a406)
                };
                let a408 = 0u32;
                if equal(a407, a408) {
                    Bytes::lit(&[0x38])
                } else {
                    Bytes::lit(&[0x39])
                }
            } else {
                let a411 = {
                    let a409 = v0;
                    let a410 = 1u32;
                    fixed_u32::bit_and(a409, a410)
                };
                let a412 = 0u32;
                if equal(a411, a412) {
                    Bytes::lit(&[0x61])
                } else {
                    Bytes::lit(&[0x62])
                }
            }
        } else {
            let a415 = {
                let a413 = v0;
                let a414 = 2u32;
                fixed_u32::bit_and(a413, a414)
            };
            let a416 = 0u32;
            if equal(a415, a416) {
                let a419 = {
                    let a417 = v0;
                    let a418 = 1u32;
                    fixed_u32::bit_and(a417, a418)
                };
                let a420 = 0u32;
                if equal(a419, a420) {
                    Bytes::lit(&[0x63])
                } else {
                    Bytes::lit(&[0x64])
                }
            } else {
                let a423 = {
                    let a421 = v0;
                    let a422 = 1u32;
                    fixed_u32::bit_and(a421, a422)
                };
                let a424 = 0u32;
                if equal(a423, a424) {
                    Bytes::lit(&[0x65])
                } else {
                    Bytes::lit(&[0x66])
                }
            }
        }
    }
}

pub fn f42(v0: u32, v1: u32) -> u32 {
    let m427 = {
        let a425 = v0;
        let a426 = v1;
        fixed_u32::shift_right(a425, a426)
    };
    match m427 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f43(v0: Adt2, v1: Adt4) -> R<Adt2> {
    let v2: Adt5 = f61(Fn6::F62, 64u64, Adt5::C0(match v0.clone() {
        Adt2::C0(h428, _, _, _, _, _, _, _) => {
            h428
        }
    }, match v0.clone() {
        Adt2::C0(_, h429, _, _, _, _, _, _) => {
            h429
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h430, _, _, _, _, _) => {
            h430
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h431, _, _, _, _) => {
            h431
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h432, _, _, _) => {
            h432
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h433, _, _) => {
            h433
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h434, _) => {
            h434
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h435) => {
            h435
        }
    }, v1, 0u64))?;
    Ok(Adt2::C0(f63(match v0.clone() {
        Adt2::C0(h436, _, _, _, _, _, _, _) => {
            h436
        }
    }, match v2.clone() {
        Adt5::C0(h437, _, _, _, _, _, _, _, _, _) => {
            h437
        }
    }), f63(match v0.clone() {
        Adt2::C0(_, h438, _, _, _, _, _, _) => {
            h438
        }
    }, match v2.clone() {
        Adt5::C0(_, h439, _, _, _, _, _, _, _, _) => {
            h439
        }
    }), f63(match v0.clone() {
        Adt2::C0(_, _, h440, _, _, _, _, _) => {
            h440
        }
    }, match v2.clone() {
        Adt5::C0(_, _, h441, _, _, _, _, _, _, _) => {
            h441
        }
    }), f63(match v0.clone() {
        Adt2::C0(_, _, _, h442, _, _, _, _) => {
            h442
        }
    }, match v2.clone() {
        Adt5::C0(_, _, _, h443, _, _, _, _, _, _) => {
            h443
        }
    }), f63(match v0.clone() {
        Adt2::C0(_, _, _, _, h444, _, _, _) => {
            h444
        }
    }, match v2.clone() {
        Adt5::C0(_, _, _, _, h445, _, _, _, _, _) => {
            h445
        }
    }), f63(match v0.clone() {
        Adt2::C0(_, _, _, _, _, h446, _, _) => {
            h446
        }
    }, match v2.clone() {
        Adt5::C0(_, _, _, _, _, h447, _, _, _, _) => {
            h447
        }
    }), f63(match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h448, _) => {
            h448
        }
    }, match v2.clone() {
        Adt5::C0(_, _, _, _, _, _, h449, _, _, _) => {
            h449
        }
    }), f63(match v0 {
        Adt2::C0(_, _, _, _, _, _, _, h450) => {
            h450
        }
    }, match v2 {
        Adt5::C0(_, _, _, _, _, _, _, h451, _, _) => {
            h451
        }
    })))
}

pub fn f44(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt4> {
    Ok(Adt4::C0(f64(v0.clone(), v1, v2, v3)?, f64(v0.clone(), v1, v2, {
        let a452 = v3;
        let a453 = 4u64;
        nat_add(a452, a453)?
    })?, f64(v0.clone(), v1, v2, {
        let a454 = v3;
        let a455 = 8u64;
        nat_add(a454, a455)?
    })?, f64(v0.clone(), v1, v2, {
        let a456 = v3;
        let a457 = 12u64;
        nat_add(a456, a457)?
    })?, f64(v0.clone(), v1, v2, {
        let a458 = v3;
        let a459 = 16u64;
        nat_add(a458, a459)?
    })?, f64(v0.clone(), v1, v2, {
        let a460 = v3;
        let a461 = 20u64;
        nat_add(a460, a461)?
    })?, f64(v0.clone(), v1, v2, {
        let a462 = v3;
        let a463 = 24u64;
        nat_add(a462, a463)?
    })?, f64(v0.clone(), v1, v2, {
        let a464 = v3;
        let a465 = 28u64;
        nat_add(a464, a465)?
    })?, f64(v0.clone(), v1, v2, {
        let a466 = v3;
        let a467 = 32u64;
        nat_add(a466, a467)?
    })?, f64(v0.clone(), v1, v2, {
        let a468 = v3;
        let a469 = 36u64;
        nat_add(a468, a469)?
    })?, f64(v0.clone(), v1, v2, {
        let a470 = v3;
        let a471 = 40u64;
        nat_add(a470, a471)?
    })?, f64(v0.clone(), v1, v2, {
        let a472 = v3;
        let a473 = 44u64;
        nat_add(a472, a473)?
    })?, f64(v0.clone(), v1, v2, {
        let a474 = v3;
        let a475 = 48u64;
        nat_add(a474, a475)?
    })?, f64(v0.clone(), v1, v2, {
        let a476 = v3;
        let a477 = 52u64;
        nat_add(a476, a477)?
    })?, f64(v0.clone(), v1, v2, {
        let a478 = v3;
        let a479 = 56u64;
        nat_add(a478, a479)?
    })?, f64(v0, v1, v2, {
        let a480 = v3;
        let a481 = 60u64;
        nat_add(a480, a481)?
    })?))
}

pub fn f45(v0: Fn4, p485: u64, p486: (u64, Adt3)) -> R<(u64, Adt3)> {
    let m484 = (p485, p486);
    {
        let mut m484 = m484;
        loop {
            let step = {
                let (v1, v2) = m484;
                let m482 = v1;
                if m482 == 0 {
                    Err::<(u64, (u64, Adt3)), R<(u64, Adt3)>>(Ok(v2))
                } else {
                    let v3 = m482 - 1;
                    Ok::<(u64, (u64, Adt3)), R<(u64, Adt3)>>((v3, {
                        let c483: Fn4 = v0.clone();
                        c483.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m484 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f46(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: (u64, Adt3)) -> R<(u64, Adt3)> {
    Ok(({
        let a488 = {
            let (h487, _) = v4.clone();
            h487
        };
        let a489 = 136u64;
        nat_add(a488, a489)?
    }, f65(v0, v1, v3, v2, {
        let (_, h490) = v4;
        h490
    }, {
        let (h491, _) = v4;
        h491
    })?))
}

pub fn f47() -> Adt3 {
    Adt3::C0(0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64)
}

pub fn f48(v0: Adt0, v1: Adt6) -> R<Adt0> {
    let v2: Adt7 = f66(Fn7::F67, 80u64, Adt7::C0(match v0.clone() {
        Adt0::C0(h492, _, _, _, _, _, _, _) => {
            h492
        }
    }, match v0.clone() {
        Adt0::C0(_, h493, _, _, _, _, _, _) => {
            h493
        }
    }, match v0.clone() {
        Adt0::C0(_, _, h494, _, _, _, _, _) => {
            h494
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, h495, _, _, _, _) => {
            h495
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, h496, _, _, _) => {
            h496
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, h497, _, _) => {
            h497
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, h498, _) => {
            h498
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, h499) => {
            h499
        }
    }, v1, 0u64))?;
    Ok(Adt0::C0(f68(match v0.clone() {
        Adt0::C0(h500, _, _, _, _, _, _, _) => {
            h500
        }
    }, match v2.clone() {
        Adt7::C0(h501, _, _, _, _, _, _, _, _, _) => {
            h501
        }
    }), f68(match v0.clone() {
        Adt0::C0(_, h502, _, _, _, _, _, _) => {
            h502
        }
    }, match v2.clone() {
        Adt7::C0(_, h503, _, _, _, _, _, _, _, _) => {
            h503
        }
    }), f68(match v0.clone() {
        Adt0::C0(_, _, h504, _, _, _, _, _) => {
            h504
        }
    }, match v2.clone() {
        Adt7::C0(_, _, h505, _, _, _, _, _, _, _) => {
            h505
        }
    }), f68(match v0.clone() {
        Adt0::C0(_, _, _, h506, _, _, _, _) => {
            h506
        }
    }, match v2.clone() {
        Adt7::C0(_, _, _, h507, _, _, _, _, _, _) => {
            h507
        }
    }), f68(match v0.clone() {
        Adt0::C0(_, _, _, _, h508, _, _, _) => {
            h508
        }
    }, match v2.clone() {
        Adt7::C0(_, _, _, _, h509, _, _, _, _, _) => {
            h509
        }
    }), f68(match v0.clone() {
        Adt0::C0(_, _, _, _, _, h510, _, _) => {
            h510
        }
    }, match v2.clone() {
        Adt7::C0(_, _, _, _, _, h511, _, _, _, _) => {
            h511
        }
    }), f68(match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, h512, _) => {
            h512
        }
    }, match v2.clone() {
        Adt7::C0(_, _, _, _, _, _, h513, _, _, _) => {
            h513
        }
    }), f68(match v0 {
        Adt0::C0(_, _, _, _, _, _, _, h514) => {
            h514
        }
    }, match v2 {
        Adt7::C0(_, _, _, _, _, _, _, h515, _, _) => {
            h515
        }
    })))
}

pub fn f49(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt6> {
    Ok(Adt6::C0(f69(v0.clone(), v1, v2, {
        let a516 = v3;
        let a517 = 0u64;
        nat_add(a516, a517)?
    })?, f69(v0.clone(), v1, v2, {
        let a518 = v3;
        let a519 = 8u64;
        nat_add(a518, a519)?
    })?, f69(v0.clone(), v1, v2, {
        let a520 = v3;
        let a521 = 16u64;
        nat_add(a520, a521)?
    })?, f69(v0.clone(), v1, v2, {
        let a522 = v3;
        let a523 = 24u64;
        nat_add(a522, a523)?
    })?, f69(v0.clone(), v1, v2, {
        let a524 = v3;
        let a525 = 32u64;
        nat_add(a524, a525)?
    })?, f69(v0.clone(), v1, v2, {
        let a526 = v3;
        let a527 = 40u64;
        nat_add(a526, a527)?
    })?, f69(v0.clone(), v1, v2, {
        let a528 = v3;
        let a529 = 48u64;
        nat_add(a528, a529)?
    })?, f69(v0.clone(), v1, v2, {
        let a530 = v3;
        let a531 = 56u64;
        nat_add(a530, a531)?
    })?, f69(v0.clone(), v1, v2, {
        let a532 = v3;
        let a533 = 64u64;
        nat_add(a532, a533)?
    })?, f69(v0.clone(), v1, v2, {
        let a534 = v3;
        let a535 = 72u64;
        nat_add(a534, a535)?
    })?, f69(v0.clone(), v1, v2, {
        let a536 = v3;
        let a537 = 80u64;
        nat_add(a536, a537)?
    })?, f69(v0.clone(), v1, v2, {
        let a538 = v3;
        let a539 = 88u64;
        nat_add(a538, a539)?
    })?, f69(v0.clone(), v1, v2, {
        let a540 = v3;
        let a541 = 96u64;
        nat_add(a540, a541)?
    })?, f69(v0.clone(), v1, v2, {
        let a542 = v3;
        let a543 = 104u64;
        nat_add(a542, a543)?
    })?, f69(v0.clone(), v1, v2, {
        let a544 = v3;
        let a545 = 112u64;
        nat_add(a544, a545)?
    })?, f69(v0, v1, v2, {
        let a546 = v3;
        let a547 = 120u64;
        nat_add(a546, a547)?
    })?))
}

pub fn f50(v0: u64) -> u32 {
    let m551 = {
        let a550 = {
            let a548 = v0;
            let a549 = 4294967295u64;
            fixed_u64::bit_and(a548, a549)
        };
        fixed_u32::convert(i128::from(a550))
    };
    match m551 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f51(v0: u64, v1: u32) -> u64 {
    let m554 = {
        let a552 = v0;
        let a553 = v1;
        fixed_u64::shift_right(a552, a553)
    };
    match m554 {
        None => {
            0u64
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f52(v0: Fn5, p558: u64, p559: (u64, List<Adt1>)) -> R<(u64, List<Adt1>)> {
    let m557 = (p558, p559);
    {
        let mut m557 = m557;
        loop {
            let step = {
                let (v1, v2) = m557;
                let m555 = v1;
                if m555 == 0 {
                    Err::<(u64, (u64, List<Adt1>)), R<(u64, List<Adt1>)>>(Ok(v2))
                } else {
                    let v3 = m555 - 1;
                    Ok::<(u64, (u64, List<Adt1>)), R<(u64, List<Adt1>)>>((v3, {
                        let c556: Fn5 = v0.clone();
                        c556.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m557 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f53(v0: Bytes, v1: u64, v2: (u64, List<Adt1>)) -> R<(u64, List<Adt1>)> {
    f70(v2.clone(), f37(v0, v1, {
        let (h560, _) = v2;
        h560
    }, false)?)
}

pub fn f54(v0: u64, v1: u64) -> u64 {
    let a563 = 1024u64;
    let a564 = {
        let a561 = v0;
        let a562 = v1;
        nat_sub(a561, a562)
    };
    if nat_lt(a563, a564) {
        1024u64
    } else {
        let a565 = v0;
        let a566 = v1;
        nat_sub(a565, a566)
    }
}

pub fn f55(v0: u64) -> R<u64> {
    let a567 = v0;
    let a568 = 0u64;
    if nat_eq(a567, a568) {
        Ok(1u64)
    } else {
        let a571 = {
            let a569 = v0;
            let a570 = 63u64;
            nat_add(a569, a570)?
        };
        let a572 = 64u64;
        let a573 = 0u64;
        Ok(nat_quot(a571, a572, a573))
    }
}

pub fn f56(v0: Adt1, v1: Adt8, v2: u64, v3: u32, v4: u32) -> Adt1 {
    let v5: Adt9 = {
        let (h574, _) = f71(Fn8::F72, 7u64, (Adt9::C0(match v0.clone() {
            Adt1::C0(h575, _, _, _, _, _, _, _) => {
                h575
            }
        }, match v0.clone() {
            Adt1::C0(_, h576, _, _, _, _, _, _) => {
                h576
            }
        }, match v0.clone() {
            Adt1::C0(_, _, h577, _, _, _, _, _) => {
                h577
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, h578, _, _, _, _) => {
                h578
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, h579, _, _, _) => {
                h579
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, h580, _, _) => {
                h580
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h581, _) => {
                h581
            }
        }, match v0 {
            Adt1::C0(_, _, _, _, _, _, _, h582) => {
                h582
            }
        }, 1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, f59({
            let a583 = v2;
            let a584 = 4294967296u64;
            let a585 = 0u64;
            nat_rem(a583, a584, a585)
        }), f59({
            let a586 = v2;
            let a587 = 4294967296u64;
            let a588 = 0u64;
            nat_quot(a586, a587, a588)
        }), v3, v4), v1));
        h574
    };
    Adt1::C0({
        let a591 = match v5.clone() {
            Adt9::C0(h589, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h589
            }
        };
        let a592 = match v5.clone() {
            Adt9::C0(_, _, _, _, _, _, _, _, h590, _, _, _, _, _, _, _) => {
                h590
            }
        };
        fixed_u32::bit_xor(a591, a592)
    }, {
        let a595 = match v5.clone() {
            Adt9::C0(_, h593, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h593
            }
        };
        let a596 = match v5.clone() {
            Adt9::C0(_, _, _, _, _, _, _, _, _, h594, _, _, _, _, _, _) => {
                h594
            }
        };
        fixed_u32::bit_xor(a595, a596)
    }, {
        let a599 = match v5.clone() {
            Adt9::C0(_, _, h597, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h597
            }
        };
        let a600 = match v5.clone() {
            Adt9::C0(_, _, _, _, _, _, _, _, _, _, h598, _, _, _, _, _) => {
                h598
            }
        };
        fixed_u32::bit_xor(a599, a600)
    }, {
        let a603 = match v5.clone() {
            Adt9::C0(_, _, _, h601, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h601
            }
        };
        let a604 = match v5.clone() {
            Adt9::C0(_, _, _, _, _, _, _, _, _, _, _, h602, _, _, _, _) => {
                h602
            }
        };
        fixed_u32::bit_xor(a603, a604)
    }, {
        let a607 = match v5.clone() {
            Adt9::C0(_, _, _, _, h605, _, _, _, _, _, _, _, _, _, _, _) => {
                h605
            }
        };
        let a608 = match v5.clone() {
            Adt9::C0(_, _, _, _, _, _, _, _, _, _, _, _, h606, _, _, _) => {
                h606
            }
        };
        fixed_u32::bit_xor(a607, a608)
    }, {
        let a611 = match v5.clone() {
            Adt9::C0(_, _, _, _, _, h609, _, _, _, _, _, _, _, _, _, _) => {
                h609
            }
        };
        let a612 = match v5.clone() {
            Adt9::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h610, _, _) => {
                h610
            }
        };
        fixed_u32::bit_xor(a611, a612)
    }, {
        let a615 = match v5.clone() {
            Adt9::C0(_, _, _, _, _, _, h613, _, _, _, _, _, _, _, _, _) => {
                h613
            }
        };
        let a616 = match v5.clone() {
            Adt9::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h614, _) => {
                h614
            }
        };
        fixed_u32::bit_xor(a615, a616)
    }, {
        let a619 = match v5.clone() {
            Adt9::C0(_, _, _, _, _, _, _, h617, _, _, _, _, _, _, _, _) => {
                h617
            }
        };
        let a620 = match v5 {
            Adt9::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h618) => {
                h618
            }
        };
        fixed_u32::bit_xor(a619, a620)
    })
}

pub fn f57(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt1> {
    let (_, h621) = f73(Fn9::F74(v2, v0, v1), v3, (0u64, f75()))?;
    Ok(h621)
}

pub fn f58(v0: Bytes, v1: u64) -> R<Adt8> {
    Ok(Adt8::C0(f76(v0.clone(), {
        let a622 = v1;
        let a623 = 0u64;
        nat_add(a622, a623)?
    })?, f76(v0.clone(), {
        let a624 = v1;
        let a625 = 4u64;
        nat_add(a624, a625)?
    })?, f76(v0.clone(), {
        let a626 = v1;
        let a627 = 8u64;
        nat_add(a626, a627)?
    })?, f76(v0.clone(), {
        let a628 = v1;
        let a629 = 12u64;
        nat_add(a628, a629)?
    })?, f76(v0.clone(), {
        let a630 = v1;
        let a631 = 16u64;
        nat_add(a630, a631)?
    })?, f76(v0.clone(), {
        let a632 = v1;
        let a633 = 20u64;
        nat_add(a632, a633)?
    })?, f76(v0.clone(), {
        let a634 = v1;
        let a635 = 24u64;
        nat_add(a634, a635)?
    })?, f76(v0.clone(), {
        let a636 = v1;
        let a637 = 28u64;
        nat_add(a636, a637)?
    })?, f76(v0.clone(), {
        let a638 = v1;
        let a639 = 32u64;
        nat_add(a638, a639)?
    })?, f76(v0.clone(), {
        let a640 = v1;
        let a641 = 36u64;
        nat_add(a640, a641)?
    })?, f76(v0.clone(), {
        let a642 = v1;
        let a643 = 40u64;
        nat_add(a642, a643)?
    })?, f76(v0.clone(), {
        let a644 = v1;
        let a645 = 44u64;
        nat_add(a644, a645)?
    })?, f76(v0.clone(), {
        let a646 = v1;
        let a647 = 48u64;
        nat_add(a646, a647)?
    })?, f76(v0.clone(), {
        let a648 = v1;
        let a649 = 52u64;
        nat_add(a648, a649)?
    })?, f76(v0.clone(), {
        let a650 = v1;
        let a651 = 56u64;
        nat_add(a650, a651)?
    })?, f76(v0, {
        let a652 = v1;
        let a653 = 60u64;
        nat_add(a652, a653)?
    })?))
}

pub fn f59(v0: u64) -> u32 {
    let (_, h654) = {
        let (_, h655) = f77(Fn10::F78, 32u64, (v0, (1u32, 0u32)));
        h655
    };
    h654
}

pub fn f60(v0: (List<Adt1>, Adt1)) -> (List<Adt1>, Adt1) {
    let m657 = {
        let (h656, _) = v0.clone();
        h656
    };
    match m657.uncons() {
        None => {
            v0
        }
        Some((v1, v2)) => {
            (v2.clone(), f79(v1, {
                let (_, h658) = v0;
                h658
            }, {
                let m659 = v2;
                match m659.uncons() {
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

pub fn f61(v0: Fn6, p663: u64, p664: Adt5) -> R<Adt5> {
    let m662 = (p663, p664);
    {
        let mut m662 = m662;
        loop {
            let step = {
                let (v1, v2) = m662;
                let m660 = v1;
                if m660 == 0 {
                    Err::<(u64, Adt5), R<Adt5>>(Ok(v2))
                } else {
                    let v3 = m660 - 1;
                    Ok::<(u64, Adt5), R<Adt5>>((v3, {
                        let c661: Fn6 = v0.clone();
                        c661.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m662 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f62(v0: Adt5) -> R<Adt5> {
    f80(v0)
}

pub fn f63(v0: u32, v1: u32) -> u32 {
    let m667 = {
        let a665 = f81(v0);
        let a666 = f81(v1);
        fixed_u64::checked_add(a665, a666)
    };
    match m667 {
        None => {
            0u32
        }
        Some(v2) => {
            f50(v2)
        }
    }
}

pub fn f64(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u32> {
    let a670 = {
        let a668 = v3;
        let a669 = 8u64;
        nat_add(a668, a669)?
    };
    let a671 = v2;
    if nat_eq(a670, a671) {
        Ok(f59({
            let a674 = {
                let a672 = v1;
                let a673 = 8u64;
                nat_mul(a672, a673)?
            };
            let a675 = 4294967296u64;
            let a676 = 0u64;
            nat_quot(a674, a675, a676)
        }))
    } else {
        let a679 = {
            let a677 = v3;
            let a678 = 4u64;
            nat_add(a677, a678)?
        };
        let a680 = v2;
        if nat_eq(a679, a680) {
            Ok(f59({
                let a683 = {
                    let a681 = v1;
                    let a682 = 8u64;
                    nat_mul(a681, a682)?
                };
                let a684 = 4294967296u64;
                let a685 = 0u64;
                nat_rem(a683, a684, a685)
            }))
        } else {
            Ok(f82(f83(v0.clone(), v1, v3), f83(v0.clone(), v1, {
                let a686 = v3;
                let a687 = 1u64;
                nat_add(a686, a687)?
            }), f83(v0.clone(), v1, {
                let a688 = v3;
                let a689 = 2u64;
                nat_add(a688, a689)?
            }), f83(v0, v1, {
                let a690 = v3;
                let a691 = 3u64;
                nat_add(a690, a691)?
            })))
        }
    }
}

pub fn f65(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: Adt3, v5: u64) -> R<Adt3> {
    f84(Adt3::C0({
        let a695 = match v4.clone() {
            Adt3::C0(h692, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h692
            }
        };
        let a696 = f85(v0.clone(), v1, v2, v3, {
            let a693 = v5;
            let a694 = 0u64;
            nat_add(a693, a694)?
        })?;
        fixed_u64::bit_xor(a695, a696)
    }, {
        let a700 = match v4.clone() {
            Adt3::C0(_, h697, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h697
            }
        };
        let a701 = f85(v0.clone(), v1, v2, v3, {
            let a698 = v5;
            let a699 = 8u64;
            nat_add(a698, a699)?
        })?;
        fixed_u64::bit_xor(a700, a701)
    }, {
        let a705 = match v4.clone() {
            Adt3::C0(_, _, h702, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h702
            }
        };
        let a706 = f85(v0.clone(), v1, v2, v3, {
            let a703 = v5;
            let a704 = 16u64;
            nat_add(a703, a704)?
        })?;
        fixed_u64::bit_xor(a705, a706)
    }, {
        let a710 = match v4.clone() {
            Adt3::C0(_, _, _, h707, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h707
            }
        };
        let a711 = f85(v0.clone(), v1, v2, v3, {
            let a708 = v5;
            let a709 = 24u64;
            nat_add(a708, a709)?
        })?;
        fixed_u64::bit_xor(a710, a711)
    }, {
        let a715 = match v4.clone() {
            Adt3::C0(_, _, _, _, h712, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h712
            }
        };
        let a716 = f85(v0.clone(), v1, v2, v3, {
            let a713 = v5;
            let a714 = 32u64;
            nat_add(a713, a714)?
        })?;
        fixed_u64::bit_xor(a715, a716)
    }, {
        let a720 = match v4.clone() {
            Adt3::C0(_, _, _, _, _, h717, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h717
            }
        };
        let a721 = f85(v0.clone(), v1, v2, v3, {
            let a718 = v5;
            let a719 = 40u64;
            nat_add(a718, a719)?
        })?;
        fixed_u64::bit_xor(a720, a721)
    }, {
        let a725 = match v4.clone() {
            Adt3::C0(_, _, _, _, _, _, h722, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h722
            }
        };
        let a726 = f85(v0.clone(), v1, v2, v3, {
            let a723 = v5;
            let a724 = 48u64;
            nat_add(a723, a724)?
        })?;
        fixed_u64::bit_xor(a725, a726)
    }, {
        let a730 = match v4.clone() {
            Adt3::C0(_, _, _, _, _, _, _, h727, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h727
            }
        };
        let a731 = f85(v0.clone(), v1, v2, v3, {
            let a728 = v5;
            let a729 = 56u64;
            nat_add(a728, a729)?
        })?;
        fixed_u64::bit_xor(a730, a731)
    }, {
        let a735 = match v4.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, h732, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h732
            }
        };
        let a736 = f85(v0.clone(), v1, v2, v3, {
            let a733 = v5;
            let a734 = 64u64;
            nat_add(a733, a734)?
        })?;
        fixed_u64::bit_xor(a735, a736)
    }, {
        let a740 = match v4.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, h737, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h737
            }
        };
        let a741 = f85(v0.clone(), v1, v2, v3, {
            let a738 = v5;
            let a739 = 72u64;
            nat_add(a738, a739)?
        })?;
        fixed_u64::bit_xor(a740, a741)
    }, {
        let a745 = match v4.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, h742, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h742
            }
        };
        let a746 = f85(v0.clone(), v1, v2, v3, {
            let a743 = v5;
            let a744 = 80u64;
            nat_add(a743, a744)?
        })?;
        fixed_u64::bit_xor(a745, a746)
    }, {
        let a750 = match v4.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, h747, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h747
            }
        };
        let a751 = f85(v0.clone(), v1, v2, v3, {
            let a748 = v5;
            let a749 = 88u64;
            nat_add(a748, a749)?
        })?;
        fixed_u64::bit_xor(a750, a751)
    }, {
        let a755 = match v4.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, h752, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h752
            }
        };
        let a756 = f85(v0.clone(), v1, v2, v3, {
            let a753 = v5;
            let a754 = 96u64;
            nat_add(a753, a754)?
        })?;
        fixed_u64::bit_xor(a755, a756)
    }, {
        let a760 = match v4.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h757, _, _, _, _, _, _, _, _, _, _, _) => {
                h757
            }
        };
        let a761 = f85(v0.clone(), v1, v2, v3, {
            let a758 = v5;
            let a759 = 104u64;
            nat_add(a758, a759)?
        })?;
        fixed_u64::bit_xor(a760, a761)
    }, {
        let a765 = match v4.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h762, _, _, _, _, _, _, _, _, _, _) => {
                h762
            }
        };
        let a766 = f85(v0.clone(), v1, v2, v3, {
            let a763 = v5;
            let a764 = 112u64;
            nat_add(a763, a764)?
        })?;
        fixed_u64::bit_xor(a765, a766)
    }, {
        let a770 = match v4.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h767, _, _, _, _, _, _, _, _, _) => {
                h767
            }
        };
        let a771 = f85(v0.clone(), v1, v2, v3, {
            let a768 = v5;
            let a769 = 120u64;
            nat_add(a768, a769)?
        })?;
        fixed_u64::bit_xor(a770, a771)
    }, {
        let a775 = match v4.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h772, _, _, _, _, _, _, _, _) => {
                h772
            }
        };
        let a776 = f85(v0, v1, v2, v3, {
            let a773 = v5;
            let a774 = 128u64;
            nat_add(a773, a774)?
        })?;
        fixed_u64::bit_xor(a775, a776)
    }, match v4.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h777, _, _, _, _, _, _, _) => {
            h777
        }
    }, match v4.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h778, _, _, _, _, _, _) => {
            h778
        }
    }, match v4.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h779, _, _, _, _, _) => {
            h779
        }
    }, match v4.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h780, _, _, _, _) => {
            h780
        }
    }, match v4.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h781, _, _, _) => {
            h781
        }
    }, match v4.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h782, _, _) => {
            h782
        }
    }, match v4.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h783, _) => {
            h783
        }
    }, match v4 {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h784) => {
            h784
        }
    }))
}

pub fn f66(v0: Fn7, p788: u64, p789: Adt7) -> R<Adt7> {
    let m787 = (p788, p789);
    {
        let mut m787 = m787;
        loop {
            let step = {
                let (v1, v2) = m787;
                let m785 = v1;
                if m785 == 0 {
                    Err::<(u64, Adt7), R<Adt7>>(Ok(v2))
                } else {
                    let v3 = m785 - 1;
                    Ok::<(u64, Adt7), R<Adt7>>((v3, {
                        let c786: Fn7 = v0.clone();
                        c786.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m787 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f67(v0: Adt7) -> R<Adt7> {
    f86(v0)
}

pub fn f68(v0: u64, v1: u64) -> u64 {
    let v2: u64 = 4294967295u64;
    let v3: u64 = f87({
        let a790 = v0;
        let a791 = v2;
        fixed_u64::bit_and(a790, a791)
    }, {
        let a792 = v1;
        let a793 = v2;
        fixed_u64::bit_and(a792, a793)
    });
    let v4: u64 = f87(f87(f51(v0, 32u32), f51(v1, 32u32)), f51(v3, 32u32));
    let a798 = f88({
        let a794 = v4;
        let a795 = v2;
        fixed_u64::bit_and(a794, a795)
    }, 32u32);
    let a799 = {
        let a796 = v3;
        let a797 = v2;
        fixed_u64::bit_and(a796, a797)
    };
    fixed_u64::bit_or(a798, a799)
}

pub fn f69(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a802 = {
        let a800 = v3;
        let a801 = 16u64;
        nat_add(a800, a801)?
    };
    let a803 = v2;
    if nat_eq(a802, a803) {
        Ok(0u64)
    } else {
        let a806 = {
            let a804 = v3;
            let a805 = 8u64;
            nat_add(a804, a805)?
        };
        let a807 = v2;
        if nat_eq(a806, a807) {
            Ok(f89({
                let a808 = v1;
                let a809 = 8u64;
                nat_mul(a808, a809)?
            }))
        } else {
            Ok(f90(f91(v0.clone(), v1, {
                let a810 = v3;
                let a811 = 0u64;
                nat_add(a810, a811)?
            }), f91(v0.clone(), v1, {
                let a812 = v3;
                let a813 = 1u64;
                nat_add(a812, a813)?
            }), f91(v0.clone(), v1, {
                let a814 = v3;
                let a815 = 2u64;
                nat_add(a814, a815)?
            }), f91(v0.clone(), v1, {
                let a816 = v3;
                let a817 = 3u64;
                nat_add(a816, a817)?
            }), f91(v0.clone(), v1, {
                let a818 = v3;
                let a819 = 4u64;
                nat_add(a818, a819)?
            }), f91(v0.clone(), v1, {
                let a820 = v3;
                let a821 = 5u64;
                nat_add(a820, a821)?
            }), f91(v0.clone(), v1, {
                let a822 = v3;
                let a823 = 6u64;
                nat_add(a822, a823)?
            }), f91(v0, v1, {
                let a824 = v3;
                let a825 = 7u64;
                nat_add(a824, a825)?
            })))
        }
    }
}

pub fn f70(v0: (u64, List<Adt1>), v1: Adt1) -> R<(u64, List<Adt1>)> {
    let v2: Adt10 = f92(Fn11::F93, 40u64, Adt10::C0(v1, {
        let a827 = {
            let (h826, _) = v0.clone();
            h826
        };
        let a828 = 1u64;
        nat_add(a827, a828)?
    }, {
        let (_, h829) = v0;
        h829
    }));
    Ok(({
        let a831 = {
            let (h830, _) = v0;
            h830
        };
        let a832 = 1u64;
        nat_add(a831, a832)?
    }, List::cons(match v2.clone() {
        Adt10::C0(h833, _, _) => {
            h833
        }
    }, match v2 {
        Adt10::C0(_, _, h834) => {
            h834
        }
    })))
}

pub fn f71(v0: Fn8, p838: u64, p839: (Adt9, Adt8)) -> (Adt9, Adt8) {
    let m837 = (p838, p839);
    {
        let mut m837 = m837;
        loop {
            let step = {
                let (v1, v2) = m837;
                let m835 = v1;
                if m835 == 0 {
                    Err::<(u64, (Adt9, Adt8)), (Adt9, Adt8)>(v2)
                } else {
                    let v3 = m835 - 1;
                    Ok::<(u64, (Adt9, Adt8)), (Adt9, Adt8)>((v3, {
                        let c836: Fn8 = v0.clone();
                        c836.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m837 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f72(v0: (Adt9, Adt8)) -> (Adt9, Adt8) {
    f94(v0)
}

pub fn f73(v0: Fn9, p843: u64, p844: (u64, Adt1)) -> R<(u64, Adt1)> {
    let m842 = (p843, p844);
    {
        let mut m842 = m842;
        loop {
            let step = {
                let (v1, v2) = m842;
                let m840 = v1;
                if m840 == 0 {
                    Err::<(u64, (u64, Adt1)), R<(u64, Adt1)>>(Ok(v2))
                } else {
                    let v3 = m840 - 1;
                    Ok::<(u64, (u64, Adt1)), R<(u64, Adt1)>>((v3, {
                        let c841: Fn9 = v0.clone();
                        c841.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m842 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f74(v0: u64, v1: Bytes, v2: u64, v3: (u64, Adt1)) -> R<(u64, Adt1)> {
    Ok(({
        let a846 = {
            let (h845, _) = v3.clone();
            h845
        };
        let a847 = 1u64;
        nat_add(a846, a847)?
    }, f56({
        let (_, h848) = v3.clone();
        h848
    }, f58(v1, {
        let a852 = v2;
        let a853 = {
            let a850 = 64u64;
            let a851 = {
                let (h849, _) = v3.clone();
                h849
            };
            nat_mul(a850, a851)?
        };
        nat_add(a852, a853)?
    })?, v0, 64u32, {
        let a855 = {
            let (h854, _) = v3;
            h854
        };
        let a856 = 0u64;
        if nat_eq(a855, a856) {
            1u32
        } else {
            0u32
        }
    })))
}

pub fn f75() -> Adt1 {
    Adt1::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f76(v0: Bytes, v1: u64) -> R<u32> {
    let a867 = f95(v0.clone(), v1);
    let a868 = {
        let a865 = f96(f95(v0.clone(), {
            let a857 = v1;
            let a858 = 1u64;
            nat_add(a857, a858)?
        }), 8u32);
        let a866 = {
            let a863 = f96(f95(v0.clone(), {
                let a859 = v1;
                let a860 = 2u64;
                nat_add(a859, a860)?
            }), 16u32);
            let a864 = f96(f95(v0, {
                let a861 = v1;
                let a862 = 3u64;
                nat_add(a861, a862)?
            }), 24u32);
            fixed_u32::bit_or(a863, a864)
        };
        fixed_u32::bit_or(a865, a866)
    };
    Ok(fixed_u32::bit_or(a867, a868))
}

pub fn f77(v0: Fn10, p872: u64, p873: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m871 = (p872, p873);
    {
        let mut m871 = m871;
        loop {
            let step = {
                let (v1, v2) = m871;
                let m869 = v1;
                if m869 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m869 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c870: Fn10 = v0.clone();
                        c870.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m871 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f78(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h874, _) = v0;
        h874
    };
    let v2: u32 = {
        let (h875, _) = {
            let (_, h876) = v0;
            h876
        };
        h875
    };
    let v3: u32 = {
        let (_, h877) = {
            let (_, h878) = v0;
            h878
        };
        h877
    };
    ({
        let a879 = v1;
        let a880 = 2u64;
        let a881 = 0u64;
        nat_quot(a879, a880, a881)
    }, (f96(v2, 1u32), {
        let a885 = {
            let a882 = v1;
            let a883 = 2u64;
            let a884 = 0u64;
            nat_rem(a882, a883, a884)
        };
        let a886 = 1u64;
        if nat_eq(a885, a886) {
            let a887 = v3;
            let a888 = v2;
            fixed_u32::bit_or(a887, a888)
        } else {
            v3
        }
    }))
}

pub fn f79(v0: Adt1, v1: Adt1, v2: bool) -> Adt1 {
    f56(f75(), f97(v0, v1), 0u64, 64u32, {
        let a889 = 4u32;
        let a890 = if v2 {
            8u32
        } else {
            0u32
        };
        fixed_u32::bit_or(a889, a890)
    })
}

pub fn f80(v0: Adt5) -> R<Adt5> {
    let v1: u32 = f98(match v0.clone() {
        Adt5::C0(_, _, _, _, _, _, _, _, _, h891) => {
            h891
        }
    });
    let v2: u32 = f63(match v0.clone() {
        Adt5::C0(_, _, _, _, _, _, _, h892, _, _) => {
            h892
        }
    }, f63(f99(match v0.clone() {
        Adt5::C0(_, _, _, _, h893, _, _, _, _, _) => {
            h893
        }
    }), f63(f100(match v0.clone() {
        Adt5::C0(_, _, _, _, h894, _, _, _, _, _) => {
            h894
        }
    }, match v0.clone() {
        Adt5::C0(_, _, _, _, _, h895, _, _, _, _) => {
            h895
        }
    }, match v0.clone() {
        Adt5::C0(_, _, _, _, _, _, h896, _, _, _) => {
            h896
        }
    }), f63(v1, {
        let m899 = match v0.clone() {
            Adt5::C0(_, _, _, _, _, _, _, _, h898, _) => {
                h898
            }
        };
        match m899 {
            Adt4::C0(h897, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h897
            }
        }
    }))));
    let v3: u32 = f63(f101(match v0.clone() {
        Adt5::C0(h900, _, _, _, _, _, _, _, _, _) => {
            h900
        }
    }), f102(match v0.clone() {
        Adt5::C0(h901, _, _, _, _, _, _, _, _, _) => {
            h901
        }
    }, match v0.clone() {
        Adt5::C0(_, h902, _, _, _, _, _, _, _, _) => {
            h902
        }
    }, match v0.clone() {
        Adt5::C0(_, _, h903, _, _, _, _, _, _, _) => {
            h903
        }
    }));
    Ok(Adt5::C0(f63(v2, v3), match v0.clone() {
        Adt5::C0(h904, _, _, _, _, _, _, _, _, _) => {
            h904
        }
    }, match v0.clone() {
        Adt5::C0(_, h905, _, _, _, _, _, _, _, _) => {
            h905
        }
    }, match v0.clone() {
        Adt5::C0(_, _, h906, _, _, _, _, _, _, _) => {
            h906
        }
    }, f63(match v0.clone() {
        Adt5::C0(_, _, _, h907, _, _, _, _, _, _) => {
            h907
        }
    }, v2), match v0.clone() {
        Adt5::C0(_, _, _, _, h908, _, _, _, _, _) => {
            h908
        }
    }, match v0.clone() {
        Adt5::C0(_, _, _, _, _, h909, _, _, _, _) => {
            h909
        }
    }, match v0.clone() {
        Adt5::C0(_, _, _, _, _, _, h910, _, _, _) => {
            h910
        }
    }, f103(match v0.clone() {
        Adt5::C0(_, _, _, _, _, _, _, _, h911, _) => {
            h911
        }
    }), {
        let a913 = match v0 {
            Adt5::C0(_, _, _, _, _, _, _, _, _, h912) => {
                h912
            }
        };
        let a914 = 1u64;
        nat_add(a913, a914)?
    }))
}

pub fn f81(v0: u32) -> u64 {
    let m916 = {
        let a915 = v0;
        fixed_u64::convert(i128::from(a915))
    };
    match m916 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f82(v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
    let a921 = f96(v0, 24u32);
    let a922 = {
        let a919 = f96(v1, 16u32);
        let a920 = {
            let a917 = f96(v2, 8u32);
            let a918 = v3;
            fixed_u32::bit_or(a917, a918)
        };
        fixed_u32::bit_or(a919, a920)
    };
    fixed_u32::bit_or(a921, a922)
}

pub fn f83(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a923 = v2;
    let a924 = v1;
    if nat_lt(a923, a924) {
        let m927 = {
            let a925 = v0;
            let a926 = v2;
            index_bytes(a925, a926)
        };
        match m927 {
            None => {
                0u32
            }
            Some(v3) => {
                let m929 = {
                    let a928 = v3;
                    fixed_u32::convert(i128::from(a928))
                };
                match m929 {
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
        let a930 = v2;
        let a931 = v1;
        if nat_eq(a930, a931) {
            128u32
        } else {
            0u32
        }
    }
}

pub fn f84(v0: Adt3) -> R<Adt3> {
    let (_, h932) = f45(Fn4::F104, 24u64, (0u64, v0))?;
    Ok(h932)
}

pub fn f85(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    Ok(f105(f106(v0.clone(), v1, v2, v3, {
        let a933 = v4;
        let a934 = 0u64;
        nat_add(a933, a934)?
    })?, f106(v0.clone(), v1, v2, v3, {
        let a935 = v4;
        let a936 = 1u64;
        nat_add(a935, a936)?
    })?, f106(v0.clone(), v1, v2, v3, {
        let a937 = v4;
        let a938 = 2u64;
        nat_add(a937, a938)?
    })?, f106(v0.clone(), v1, v2, v3, {
        let a939 = v4;
        let a940 = 3u64;
        nat_add(a939, a940)?
    })?, f106(v0.clone(), v1, v2, v3, {
        let a941 = v4;
        let a942 = 4u64;
        nat_add(a941, a942)?
    })?, f106(v0.clone(), v1, v2, v3, {
        let a943 = v4;
        let a944 = 5u64;
        nat_add(a943, a944)?
    })?, f106(v0.clone(), v1, v2, v3, {
        let a945 = v4;
        let a946 = 6u64;
        nat_add(a945, a946)?
    })?, f106(v0, v1, v2, v3, {
        let a947 = v4;
        let a948 = 7u64;
        nat_add(a947, a948)?
    })?))
}

pub fn f86(v0: Adt7) -> R<Adt7> {
    let v1: u64 = f107(match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, h949) => {
            h949
        }
    });
    let v2: u64 = f68(match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, h950, _, _) => {
            h950
        }
    }, f68(f108(match v0.clone() {
        Adt7::C0(_, _, _, _, h951, _, _, _, _, _) => {
            h951
        }
    }), f68(f109(match v0.clone() {
        Adt7::C0(_, _, _, _, h952, _, _, _, _, _) => {
            h952
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, h953, _, _, _, _) => {
            h953
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, h954, _, _, _) => {
            h954
        }
    }), f68(v1, {
        let m957 = match v0.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, h956, _) => {
                h956
            }
        };
        match m957 {
            Adt6::C0(h955, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h955
            }
        }
    }))));
    let v3: u64 = f68(f110(match v0.clone() {
        Adt7::C0(h958, _, _, _, _, _, _, _, _, _) => {
            h958
        }
    }), f111(match v0.clone() {
        Adt7::C0(h959, _, _, _, _, _, _, _, _, _) => {
            h959
        }
    }, match v0.clone() {
        Adt7::C0(_, h960, _, _, _, _, _, _, _, _) => {
            h960
        }
    }, match v0.clone() {
        Adt7::C0(_, _, h961, _, _, _, _, _, _, _) => {
            h961
        }
    }));
    Ok(Adt7::C0(f68(v2, v3), match v0.clone() {
        Adt7::C0(h962, _, _, _, _, _, _, _, _, _) => {
            h962
        }
    }, match v0.clone() {
        Adt7::C0(_, h963, _, _, _, _, _, _, _, _) => {
            h963
        }
    }, match v0.clone() {
        Adt7::C0(_, _, h964, _, _, _, _, _, _, _) => {
            h964
        }
    }, f68(match v0.clone() {
        Adt7::C0(_, _, _, h965, _, _, _, _, _, _) => {
            h965
        }
    }, v2), match v0.clone() {
        Adt7::C0(_, _, _, _, h966, _, _, _, _, _) => {
            h966
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, h967, _, _, _, _) => {
            h967
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, h968, _, _, _) => {
            h968
        }
    }, f112(match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, h969, _) => {
            h969
        }
    }), {
        let a971 = match v0 {
            Adt7::C0(_, _, _, _, _, _, _, _, _, h970) => {
                h970
            }
        };
        let a972 = 1u64;
        nat_add(a971, a972)?
    }))
}

pub fn f87(v0: u64, v1: u64) -> u64 {
    let m975 = {
        let a973 = v0;
        let a974 = v1;
        fixed_u64::checked_add(a973, a974)
    };
    match m975 {
        None => {
            0u64
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f88(v0: u64, v1: u32) -> u64 {
    let m978 = {
        let a976 = v0;
        let a977 = v1;
        fixed_u64::shift_left(a976, a977)
    };
    match m978 {
        None => {
            0u64
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f89(v0: u64) -> u64 {
    let (_, h979) = {
        let (_, h980) = f113(Fn12::F114, 64u64, (v0, (1u64, 0u64)));
        h980
    };
    h979
}

pub fn f90(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> u64 {
    let a993 = f88(v0, 56u32);
    let a994 = {
        let a991 = f88(v1, 48u32);
        let a992 = {
            let a989 = f88(v2, 40u32);
            let a990 = {
                let a987 = f88(v3, 32u32);
                let a988 = {
                    let a985 = f88(v4, 24u32);
                    let a986 = {
                        let a983 = f88(v5, 16u32);
                        let a984 = {
                            let a981 = f88(v6, 8u32);
                            let a982 = v7;
                            fixed_u64::bit_or(a981, a982)
                        };
                        fixed_u64::bit_or(a983, a984)
                    };
                    fixed_u64::bit_or(a985, a986)
                };
                fixed_u64::bit_or(a987, a988)
            };
            fixed_u64::bit_or(a989, a990)
        };
        fixed_u64::bit_or(a991, a992)
    };
    fixed_u64::bit_or(a993, a994)
}

pub fn f91(v0: Bytes, v1: u64, v2: u64) -> u64 {
    let a995 = v2;
    let a996 = v1;
    if nat_lt(a995, a996) {
        f115(v0, v2)
    } else {
        let a997 = v2;
        let a998 = v1;
        if nat_eq(a997, a998) {
            128u64
        } else {
            0u64
        }
    }
}

pub fn f92(v0: Fn11, p1002: u64, p1003: Adt10) -> Adt10 {
    let m1001 = (p1002, p1003);
    {
        let mut m1001 = m1001;
        loop {
            let step = {
                let (v1, v2) = m1001;
                let m999 = v1;
                if m999 == 0 {
                    Err::<(u64, Adt10), Adt10>(v2)
                } else {
                    let v3 = m999 - 1;
                    Ok::<(u64, Adt10), Adt10>((v3, {
                        let c1000: Fn11 = v0.clone();
                        c1000.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m1001 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f93(v0: Adt10) -> Adt10 {
    f116(v0)
}

pub fn f94(v0: (Adt9, Adt8)) -> (Adt9, Adt8) {
    (f117(f118({
        let (h1004, _) = v0.clone();
        h1004
    }, {
        let (_, h1005) = v0.clone();
        h1005
    }), {
        let (_, h1006) = v0.clone();
        h1006
    }), f119({
        let (_, h1007) = v0;
        h1007
    }))
}

pub fn f95(v0: Bytes, v1: u64) -> u32 {
    let m1010 = {
        let a1008 = v0;
        let a1009 = v1;
        index_bytes(a1008, a1009)
    };
    match m1010 {
        None => {
            0u32
        }
        Some(v2) => {
            let m1012 = {
                let a1011 = v2;
                fixed_u32::convert(i128::from(a1011))
            };
            match m1012 {
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

pub fn f96(v0: u32, v1: u32) -> u32 {
    let m1015 = {
        let a1013 = v0;
        let a1014 = v1;
        fixed_u32::shift_left(a1013, a1014)
    };
    match m1015 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f97(v0: Adt1, v1: Adt1) -> Adt8 {
    Adt8::C0(match v0.clone() {
        Adt1::C0(h1016, _, _, _, _, _, _, _) => {
            h1016
        }
    }, match v0.clone() {
        Adt1::C0(_, h1017, _, _, _, _, _, _) => {
            h1017
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h1018, _, _, _, _, _) => {
            h1018
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h1019, _, _, _, _) => {
            h1019
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h1020, _, _, _) => {
            h1020
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, h1021, _, _) => {
            h1021
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h1022, _) => {
            h1022
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, _, _, _, h1023) => {
            h1023
        }
    }, match v1.clone() {
        Adt1::C0(h1024, _, _, _, _, _, _, _) => {
            h1024
        }
    }, match v1.clone() {
        Adt1::C0(_, h1025, _, _, _, _, _, _) => {
            h1025
        }
    }, match v1.clone() {
        Adt1::C0(_, _, h1026, _, _, _, _, _) => {
            h1026
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, h1027, _, _, _, _) => {
            h1027
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, h1028, _, _, _) => {
            h1028
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, h1029, _, _) => {
            h1029
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, h1030, _) => {
            h1030
        }
    }, match v1 {
        Adt1::C0(_, _, _, _, _, _, _, h1031) => {
            h1031
        }
    })
}

pub fn f98(v0: u64) -> u32 {
    let a1032 = v0;
    let a1033 = 32u64;
    if nat_lt(a1032, a1033) {
        let a1034 = v0;
        let a1035 = 16u64;
        if nat_lt(a1034, a1035) {
            let a1036 = v0;
            let a1037 = 8u64;
            if nat_lt(a1036, a1037) {
                let a1038 = v0;
                let a1039 = 4u64;
                if nat_lt(a1038, a1039) {
                    let a1040 = v0;
                    let a1041 = 2u64;
                    if nat_lt(a1040, a1041) {
                        let a1042 = v0;
                        let a1043 = 1u64;
                        if nat_lt(a1042, a1043) {
                            1116352408u32
                        } else {
                            1899447441u32
                        }
                    } else {
                        let a1044 = v0;
                        let a1045 = 3u64;
                        if nat_lt(a1044, a1045) {
                            3049323471u32
                        } else {
                            3921009573u32
                        }
                    }
                } else {
                    let a1046 = v0;
                    let a1047 = 6u64;
                    if nat_lt(a1046, a1047) {
                        let a1048 = v0;
                        let a1049 = 5u64;
                        if nat_lt(a1048, a1049) {
                            961987163u32
                        } else {
                            1508970993u32
                        }
                    } else {
                        let a1050 = v0;
                        let a1051 = 7u64;
                        if nat_lt(a1050, a1051) {
                            2453635748u32
                        } else {
                            2870763221u32
                        }
                    }
                }
            } else {
                let a1052 = v0;
                let a1053 = 12u64;
                if nat_lt(a1052, a1053) {
                    let a1054 = v0;
                    let a1055 = 10u64;
                    if nat_lt(a1054, a1055) {
                        let a1056 = v0;
                        let a1057 = 9u64;
                        if nat_lt(a1056, a1057) {
                            3624381080u32
                        } else {
                            310598401u32
                        }
                    } else {
                        let a1058 = v0;
                        let a1059 = 11u64;
                        if nat_lt(a1058, a1059) {
                            607225278u32
                        } else {
                            1426881987u32
                        }
                    }
                } else {
                    let a1060 = v0;
                    let a1061 = 14u64;
                    if nat_lt(a1060, a1061) {
                        let a1062 = v0;
                        let a1063 = 13u64;
                        if nat_lt(a1062, a1063) {
                            1925078388u32
                        } else {
                            2162078206u32
                        }
                    } else {
                        let a1064 = v0;
                        let a1065 = 15u64;
                        if nat_lt(a1064, a1065) {
                            2614888103u32
                        } else {
                            3248222580u32
                        }
                    }
                }
            }
        } else {
            let a1066 = v0;
            let a1067 = 24u64;
            if nat_lt(a1066, a1067) {
                let a1068 = v0;
                let a1069 = 20u64;
                if nat_lt(a1068, a1069) {
                    let a1070 = v0;
                    let a1071 = 18u64;
                    if nat_lt(a1070, a1071) {
                        let a1072 = v0;
                        let a1073 = 17u64;
                        if nat_lt(a1072, a1073) {
                            3835390401u32
                        } else {
                            4022224774u32
                        }
                    } else {
                        let a1074 = v0;
                        let a1075 = 19u64;
                        if nat_lt(a1074, a1075) {
                            264347078u32
                        } else {
                            604807628u32
                        }
                    }
                } else {
                    let a1076 = v0;
                    let a1077 = 22u64;
                    if nat_lt(a1076, a1077) {
                        let a1078 = v0;
                        let a1079 = 21u64;
                        if nat_lt(a1078, a1079) {
                            770255983u32
                        } else {
                            1249150122u32
                        }
                    } else {
                        let a1080 = v0;
                        let a1081 = 23u64;
                        if nat_lt(a1080, a1081) {
                            1555081692u32
                        } else {
                            1996064986u32
                        }
                    }
                }
            } else {
                let a1082 = v0;
                let a1083 = 28u64;
                if nat_lt(a1082, a1083) {
                    let a1084 = v0;
                    let a1085 = 26u64;
                    if nat_lt(a1084, a1085) {
                        let a1086 = v0;
                        let a1087 = 25u64;
                        if nat_lt(a1086, a1087) {
                            2554220882u32
                        } else {
                            2821834349u32
                        }
                    } else {
                        let a1088 = v0;
                        let a1089 = 27u64;
                        if nat_lt(a1088, a1089) {
                            2952996808u32
                        } else {
                            3210313671u32
                        }
                    }
                } else {
                    let a1090 = v0;
                    let a1091 = 30u64;
                    if nat_lt(a1090, a1091) {
                        let a1092 = v0;
                        let a1093 = 29u64;
                        if nat_lt(a1092, a1093) {
                            3336571891u32
                        } else {
                            3584528711u32
                        }
                    } else {
                        let a1094 = v0;
                        let a1095 = 31u64;
                        if nat_lt(a1094, a1095) {
                            113926993u32
                        } else {
                            338241895u32
                        }
                    }
                }
            }
        }
    } else {
        let a1096 = v0;
        let a1097 = 48u64;
        if nat_lt(a1096, a1097) {
            let a1098 = v0;
            let a1099 = 40u64;
            if nat_lt(a1098, a1099) {
                let a1100 = v0;
                let a1101 = 36u64;
                if nat_lt(a1100, a1101) {
                    let a1102 = v0;
                    let a1103 = 34u64;
                    if nat_lt(a1102, a1103) {
                        let a1104 = v0;
                        let a1105 = 33u64;
                        if nat_lt(a1104, a1105) {
                            666307205u32
                        } else {
                            773529912u32
                        }
                    } else {
                        let a1106 = v0;
                        let a1107 = 35u64;
                        if nat_lt(a1106, a1107) {
                            1294757372u32
                        } else {
                            1396182291u32
                        }
                    }
                } else {
                    let a1108 = v0;
                    let a1109 = 38u64;
                    if nat_lt(a1108, a1109) {
                        let a1110 = v0;
                        let a1111 = 37u64;
                        if nat_lt(a1110, a1111) {
                            1695183700u32
                        } else {
                            1986661051u32
                        }
                    } else {
                        let a1112 = v0;
                        let a1113 = 39u64;
                        if nat_lt(a1112, a1113) {
                            2177026350u32
                        } else {
                            2456956037u32
                        }
                    }
                }
            } else {
                let a1114 = v0;
                let a1115 = 44u64;
                if nat_lt(a1114, a1115) {
                    let a1116 = v0;
                    let a1117 = 42u64;
                    if nat_lt(a1116, a1117) {
                        let a1118 = v0;
                        let a1119 = 41u64;
                        if nat_lt(a1118, a1119) {
                            2730485921u32
                        } else {
                            2820302411u32
                        }
                    } else {
                        let a1120 = v0;
                        let a1121 = 43u64;
                        if nat_lt(a1120, a1121) {
                            3259730800u32
                        } else {
                            3345764771u32
                        }
                    }
                } else {
                    let a1122 = v0;
                    let a1123 = 46u64;
                    if nat_lt(a1122, a1123) {
                        let a1124 = v0;
                        let a1125 = 45u64;
                        if nat_lt(a1124, a1125) {
                            3516065817u32
                        } else {
                            3600352804u32
                        }
                    } else {
                        let a1126 = v0;
                        let a1127 = 47u64;
                        if nat_lt(a1126, a1127) {
                            4094571909u32
                        } else {
                            275423344u32
                        }
                    }
                }
            }
        } else {
            let a1128 = v0;
            let a1129 = 56u64;
            if nat_lt(a1128, a1129) {
                let a1130 = v0;
                let a1131 = 52u64;
                if nat_lt(a1130, a1131) {
                    let a1132 = v0;
                    let a1133 = 50u64;
                    if nat_lt(a1132, a1133) {
                        let a1134 = v0;
                        let a1135 = 49u64;
                        if nat_lt(a1134, a1135) {
                            430227734u32
                        } else {
                            506948616u32
                        }
                    } else {
                        let a1136 = v0;
                        let a1137 = 51u64;
                        if nat_lt(a1136, a1137) {
                            659060556u32
                        } else {
                            883997877u32
                        }
                    }
                } else {
                    let a1138 = v0;
                    let a1139 = 54u64;
                    if nat_lt(a1138, a1139) {
                        let a1140 = v0;
                        let a1141 = 53u64;
                        if nat_lt(a1140, a1141) {
                            958139571u32
                        } else {
                            1322822218u32
                        }
                    } else {
                        let a1142 = v0;
                        let a1143 = 55u64;
                        if nat_lt(a1142, a1143) {
                            1537002063u32
                        } else {
                            1747873779u32
                        }
                    }
                }
            } else {
                let a1144 = v0;
                let a1145 = 60u64;
                if nat_lt(a1144, a1145) {
                    let a1146 = v0;
                    let a1147 = 58u64;
                    if nat_lt(a1146, a1147) {
                        let a1148 = v0;
                        let a1149 = 57u64;
                        if nat_lt(a1148, a1149) {
                            1955562222u32
                        } else {
                            2024104815u32
                        }
                    } else {
                        let a1150 = v0;
                        let a1151 = 59u64;
                        if nat_lt(a1150, a1151) {
                            2227730452u32
                        } else {
                            2361852424u32
                        }
                    }
                } else {
                    let a1152 = v0;
                    let a1153 = 62u64;
                    if nat_lt(a1152, a1153) {
                        let a1154 = v0;
                        let a1155 = 61u64;
                        if nat_lt(a1154, a1155) {
                            2428436474u32
                        } else {
                            2756734187u32
                        }
                    } else {
                        let a1156 = v0;
                        let a1157 = 63u64;
                        if nat_lt(a1156, a1157) {
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

pub fn f99(v0: u32) -> u32 {
    let a1160 = {
        let a1158 = f120(v0, 6u32, 26u32);
        let a1159 = f120(v0, 11u32, 21u32);
        fixed_u32::bit_xor(a1158, a1159)
    };
    let a1161 = f120(v0, 25u32, 7u32);
    fixed_u32::bit_xor(a1160, a1161)
}

pub fn f100(v0: u32, v1: u32, v2: u32) -> u32 {
    let a1167 = {
        let a1162 = v0;
        let a1163 = v1;
        fixed_u32::bit_and(a1162, a1163)
    };
    let a1168 = {
        let a1165 = {
            let a1164 = v0;
            fixed_u32::bit_not(a1164)
        };
        let a1166 = v2;
        fixed_u32::bit_and(a1165, a1166)
    };
    fixed_u32::bit_xor(a1167, a1168)
}

pub fn f101(v0: u32) -> u32 {
    let a1171 = {
        let a1169 = f120(v0, 2u32, 30u32);
        let a1170 = f120(v0, 13u32, 19u32);
        fixed_u32::bit_xor(a1169, a1170)
    };
    let a1172 = f120(v0, 22u32, 10u32);
    fixed_u32::bit_xor(a1171, a1172)
}

pub fn f102(v0: u32, v1: u32, v2: u32) -> u32 {
    let a1181 = {
        let a1177 = {
            let a1173 = v0;
            let a1174 = v1;
            fixed_u32::bit_and(a1173, a1174)
        };
        let a1178 = {
            let a1175 = v0;
            let a1176 = v2;
            fixed_u32::bit_and(a1175, a1176)
        };
        fixed_u32::bit_xor(a1177, a1178)
    };
    let a1182 = {
        let a1179 = v1;
        let a1180 = v2;
        fixed_u32::bit_and(a1179, a1180)
    };
    fixed_u32::bit_xor(a1181, a1182)
}

pub fn f103(v0: Adt4) -> Adt4 {
    Adt4::C0(match v0.clone() {
        Adt4::C0(_, h1183, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1183
        }
    }, match v0.clone() {
        Adt4::C0(_, _, h1184, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1184
        }
    }, match v0.clone() {
        Adt4::C0(_, _, _, h1185, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1185
        }
    }, match v0.clone() {
        Adt4::C0(_, _, _, _, h1186, _, _, _, _, _, _, _, _, _, _, _) => {
            h1186
        }
    }, match v0.clone() {
        Adt4::C0(_, _, _, _, _, h1187, _, _, _, _, _, _, _, _, _, _) => {
            h1187
        }
    }, match v0.clone() {
        Adt4::C0(_, _, _, _, _, _, h1188, _, _, _, _, _, _, _, _, _) => {
            h1188
        }
    }, match v0.clone() {
        Adt4::C0(_, _, _, _, _, _, _, h1189, _, _, _, _, _, _, _, _) => {
            h1189
        }
    }, match v0.clone() {
        Adt4::C0(_, _, _, _, _, _, _, _, h1190, _, _, _, _, _, _, _) => {
            h1190
        }
    }, match v0.clone() {
        Adt4::C0(_, _, _, _, _, _, _, _, _, h1191, _, _, _, _, _, _) => {
            h1191
        }
    }, match v0.clone() {
        Adt4::C0(_, _, _, _, _, _, _, _, _, _, h1192, _, _, _, _, _) => {
            h1192
        }
    }, match v0.clone() {
        Adt4::C0(_, _, _, _, _, _, _, _, _, _, _, h1193, _, _, _, _) => {
            h1193
        }
    }, match v0.clone() {
        Adt4::C0(_, _, _, _, _, _, _, _, _, _, _, _, h1194, _, _, _) => {
            h1194
        }
    }, match v0.clone() {
        Adt4::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1195, _, _) => {
            h1195
        }
    }, match v0.clone() {
        Adt4::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1196, _) => {
            h1196
        }
    }, match v0.clone() {
        Adt4::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1197) => {
            h1197
        }
    }, f63(f121(match v0.clone() {
        Adt4::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1198, _) => {
            h1198
        }
    }), f63(match v0.clone() {
        Adt4::C0(_, _, _, _, _, _, _, _, _, h1199, _, _, _, _, _, _) => {
            h1199
        }
    }, f63(f122(match v0.clone() {
        Adt4::C0(_, h1200, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1200
        }
    }), match v0 {
        Adt4::C0(h1201, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1201
        }
    }))))
}

pub fn f104(v0: (u64, Adt3)) -> R<(u64, Adt3)> {
    f123(v0)
}

pub fn f105(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> u64 {
    let a1214 = f88(v7, 56u32);
    let a1215 = {
        let a1212 = f88(v6, 48u32);
        let a1213 = {
            let a1210 = f88(v5, 40u32);
            let a1211 = {
                let a1208 = f88(v4, 32u32);
                let a1209 = {
                    let a1206 = f88(v3, 24u32);
                    let a1207 = {
                        let a1204 = f88(v2, 16u32);
                        let a1205 = {
                            let a1202 = f88(v1, 8u32);
                            let a1203 = v0;
                            fixed_u64::bit_or(a1202, a1203)
                        };
                        fixed_u64::bit_or(a1204, a1205)
                    };
                    fixed_u64::bit_or(a1206, a1207)
                };
                fixed_u64::bit_or(a1208, a1209)
            };
            fixed_u64::bit_or(a1210, a1211)
        };
        fixed_u64::bit_or(a1212, a1213)
    };
    fixed_u64::bit_or(a1214, a1215)
}

pub fn f106(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    let v5: u64 = {
        let a1216 = v4;
        let a1217 = v1;
        if nat_lt(a1216, a1217) {
            f115(v0, v4)
        } else {
            let a1218 = v4;
            let a1219 = v1;
            if nat_eq(a1218, a1219) {
                v3
            } else {
                0u64
            }
        }
    };
    let a1222 = {
        let a1220 = v4;
        let a1221 = 1u64;
        nat_add(a1220, a1221)?
    };
    let a1223 = v2;
    if nat_eq(a1222, a1223) {
        let a1224 = v5;
        let a1225 = 128u64;
        Ok(fixed_u64::bit_or(a1224, a1225))
    } else {
        Ok(v5)
    }
}

pub fn f107(v0: u64) -> u64 {
    let a1226 = v0;
    let a1227 = 40u64;
    if nat_lt(a1226, a1227) {
        let a1228 = v0;
        let a1229 = 20u64;
        if nat_lt(a1228, a1229) {
            let a1230 = v0;
            let a1231 = 10u64;
            if nat_lt(a1230, a1231) {
                let a1232 = v0;
                let a1233 = 5u64;
                if nat_lt(a1232, a1233) {
                    let a1234 = v0;
                    let a1235 = 2u64;
                    if nat_lt(a1234, a1235) {
                        let a1236 = v0;
                        let a1237 = 1u64;
                        if nat_lt(a1236, a1237) {
                            4794697086780616226u64
                        } else {
                            8158064640168781261u64
                        }
                    } else {
                        let a1238 = v0;
                        let a1239 = 3u64;
                        if nat_lt(a1238, a1239) {
                            13096744586834688815u64
                        } else {
                            let a1240 = v0;
                            let a1241 = 4u64;
                            if nat_lt(a1240, a1241) {
                                16840607885511220156u64
                            } else {
                                4131703408338449720u64
                            }
                        }
                    }
                } else {
                    let a1242 = v0;
                    let a1243 = 7u64;
                    if nat_lt(a1242, a1243) {
                        let a1244 = v0;
                        let a1245 = 6u64;
                        if nat_lt(a1244, a1245) {
                            6480981068601479193u64
                        } else {
                            10538285296894168987u64
                        }
                    } else {
                        let a1246 = v0;
                        let a1247 = 8u64;
                        if nat_lt(a1246, a1247) {
                            12329834152419229976u64
                        } else {
                            let a1248 = v0;
                            let a1249 = 9u64;
                            if nat_lt(a1248, a1249) {
                                15566598209576043074u64
                            } else {
                                1334009975649890238u64
                            }
                        }
                    }
                }
            } else {
                let a1250 = v0;
                let a1251 = 15u64;
                if nat_lt(a1250, a1251) {
                    let a1252 = v0;
                    let a1253 = 12u64;
                    if nat_lt(a1252, a1253) {
                        let a1254 = v0;
                        let a1255 = 11u64;
                        if nat_lt(a1254, a1255) {
                            2608012711638119052u64
                        } else {
                            6128411473006802146u64
                        }
                    } else {
                        let a1256 = v0;
                        let a1257 = 13u64;
                        if nat_lt(a1256, a1257) {
                            8268148722764581231u64
                        } else {
                            let a1258 = v0;
                            let a1259 = 14u64;
                            if nat_lt(a1258, a1259) {
                                9286055187155687089u64
                            } else {
                                11230858885718282805u64
                            }
                        }
                    }
                } else {
                    let a1260 = v0;
                    let a1261 = 17u64;
                    if nat_lt(a1260, a1261) {
                        let a1262 = v0;
                        let a1263 = 16u64;
                        if nat_lt(a1262, a1263) {
                            13951009754708518548u64
                        } else {
                            16472876342353939154u64
                        }
                    } else {
                        let a1264 = v0;
                        let a1265 = 18u64;
                        if nat_lt(a1264, a1265) {
                            17275323862435702243u64
                        } else {
                            let a1266 = v0;
                            let a1267 = 19u64;
                            if nat_lt(a1266, a1267) {
                                1135362057144423861u64
                            } else {
                                2597628984639134821u64
                            }
                        }
                    }
                }
            }
        } else {
            let a1268 = v0;
            let a1269 = 30u64;
            if nat_lt(a1268, a1269) {
                let a1270 = v0;
                let a1271 = 25u64;
                if nat_lt(a1270, a1271) {
                    let a1272 = v0;
                    let a1273 = 22u64;
                    if nat_lt(a1272, a1273) {
                        let a1274 = v0;
                        let a1275 = 21u64;
                        if nat_lt(a1274, a1275) {
                            3308224258029322869u64
                        } else {
                            5365058923640841347u64
                        }
                    } else {
                        let a1276 = v0;
                        let a1277 = 23u64;
                        if nat_lt(a1276, a1277) {
                            6679025012923562964u64
                        } else {
                            let a1278 = v0;
                            let a1279 = 24u64;
                            if nat_lt(a1278, a1279) {
                                8573033837759648693u64
                            } else {
                                10970295158949994411u64
                            }
                        }
                    }
                } else {
                    let a1280 = v0;
                    let a1281 = 27u64;
                    if nat_lt(a1280, a1281) {
                        let a1282 = v0;
                        let a1283 = 26u64;
                        if nat_lt(a1282, a1283) {
                            12119686244451234320u64
                        } else {
                            12683024718118986047u64
                        }
                    } else {
                        let a1284 = v0;
                        let a1285 = 28u64;
                        if nat_lt(a1284, a1285) {
                            13788192230050041572u64
                        } else {
                            let a1286 = v0;
                            let a1287 = 29u64;
                            if nat_lt(a1286, a1287) {
                                14330467153632333762u64
                            } else {
                                15395433587784984357u64
                            }
                        }
                    }
                }
            } else {
                let a1288 = v0;
                let a1289 = 35u64;
                if nat_lt(a1288, a1289) {
                    let a1290 = v0;
                    let a1291 = 32u64;
                    if nat_lt(a1290, a1291) {
                        let a1292 = v0;
                        let a1293 = 31u64;
                        if nat_lt(a1292, a1293) {
                            489312712824947311u64
                        } else {
                            1452737877330783856u64
                        }
                    } else {
                        let a1294 = v0;
                        let a1295 = 33u64;
                        if nat_lt(a1294, a1295) {
                            2861767655752347644u64
                        } else {
                            let a1296 = v0;
                            let a1297 = 34u64;
                            if nat_lt(a1296, a1297) {
                                3322285676063803686u64
                            } else {
                                5560940570517711597u64
                            }
                        }
                    }
                } else {
                    let a1298 = v0;
                    let a1299 = 37u64;
                    if nat_lt(a1298, a1299) {
                        let a1300 = v0;
                        let a1301 = 36u64;
                        if nat_lt(a1300, a1301) {
                            5996557281743188959u64
                        } else {
                            7280758554555802590u64
                        }
                    } else {
                        let a1302 = v0;
                        let a1303 = 38u64;
                        if nat_lt(a1302, a1303) {
                            8532644243296465576u64
                        } else {
                            let a1304 = v0;
                            let a1305 = 39u64;
                            if nat_lt(a1304, a1305) {
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
        let a1306 = v0;
        let a1307 = 60u64;
        if nat_lt(a1306, a1307) {
            let a1308 = v0;
            let a1309 = 50u64;
            if nat_lt(a1308, a1309) {
                let a1310 = v0;
                let a1311 = 45u64;
                if nat_lt(a1310, a1311) {
                    let a1312 = v0;
                    let a1313 = 42u64;
                    if nat_lt(a1312, a1313) {
                        let a1314 = v0;
                        let a1315 = 41u64;
                        if nat_lt(a1314, a1315) {
                            11727347734174303076u64
                        } else {
                            12113106623233404929u64
                        }
                    } else {
                        let a1316 = v0;
                        let a1317 = 43u64;
                        if nat_lt(a1316, a1317) {
                            14000437183269869457u64
                        } else {
                            let a1318 = v0;
                            let a1319 = 44u64;
                            if nat_lt(a1318, a1319) {
                                14369950271660146224u64
                            } else {
                                15101387698204529176u64
                            }
                        }
                    }
                } else {
                    let a1320 = v0;
                    let a1321 = 47u64;
                    if nat_lt(a1320, a1321) {
                        let a1322 = v0;
                        let a1323 = 46u64;
                        if nat_lt(a1322, a1323) {
                            15463397548674623760u64
                        } else {
                            17586052441742319658u64
                        }
                    } else {
                        let a1324 = v0;
                        let a1325 = 48u64;
                        if nat_lt(a1324, a1325) {
                            1182934255886127544u64
                        } else {
                            let a1326 = v0;
                            let a1327 = 49u64;
                            if nat_lt(a1326, a1327) {
                                1847814050463011016u64
                            } else {
                                2177327727835720531u64
                            }
                        }
                    }
                }
            } else {
                let a1328 = v0;
                let a1329 = 55u64;
                if nat_lt(a1328, a1329) {
                    let a1330 = v0;
                    let a1331 = 52u64;
                    if nat_lt(a1330, a1331) {
                        let a1332 = v0;
                        let a1333 = 51u64;
                        if nat_lt(a1332, a1333) {
                            2830643537854262169u64
                        } else {
                            3796741975233480872u64
                        }
                    } else {
                        let a1334 = v0;
                        let a1335 = 53u64;
                        if nat_lt(a1334, a1335) {
                            4115178125766777443u64
                        } else {
                            let a1336 = v0;
                            let a1337 = 54u64;
                            if nat_lt(a1336, a1337) {
                                5681478168544905931u64
                            } else {
                                6601373596472566643u64
                            }
                        }
                    }
                } else {
                    let a1338 = v0;
                    let a1339 = 57u64;
                    if nat_lt(a1338, a1339) {
                        let a1340 = v0;
                        let a1341 = 56u64;
                        if nat_lt(a1340, a1341) {
                            7507060721942968483u64
                        } else {
                            8399075790359081724u64
                        }
                    } else {
                        let a1342 = v0;
                        let a1343 = 58u64;
                        if nat_lt(a1342, a1343) {
                            8693463985226723168u64
                        } else {
                            let a1344 = v0;
                            let a1345 = 59u64;
                            if nat_lt(a1344, a1345) {
                                9568029438360202098u64
                            } else {
                                10144078919501101548u64
                            }
                        }
                    }
                }
            }
        } else {
            let a1346 = v0;
            let a1347 = 70u64;
            if nat_lt(a1346, a1347) {
                let a1348 = v0;
                let a1349 = 65u64;
                if nat_lt(a1348, a1349) {
                    let a1350 = v0;
                    let a1351 = 62u64;
                    if nat_lt(a1350, a1351) {
                        let a1352 = v0;
                        let a1353 = 61u64;
                        if nat_lt(a1352, a1353) {
                            10430055236837252648u64
                        } else {
                            11840083180663258601u64
                        }
                    } else {
                        let a1354 = v0;
                        let a1355 = 63u64;
                        if nat_lt(a1354, a1355) {
                            13761210420658862357u64
                        } else {
                            let a1356 = v0;
                            let a1357 = 64u64;
                            if nat_lt(a1356, a1357) {
                                14299343276471374635u64
                            } else {
                                14566680578165727644u64
                            }
                        }
                    }
                } else {
                    let a1358 = v0;
                    let a1359 = 67u64;
                    if nat_lt(a1358, a1359) {
                        let a1360 = v0;
                        let a1361 = 66u64;
                        if nat_lt(a1360, a1361) {
                            15097957966210449927u64
                        } else {
                            16922976911328602910u64
                        }
                    } else {
                        let a1362 = v0;
                        let a1363 = 68u64;
                        if nat_lt(a1362, a1363) {
                            17689382322260857208u64
                        } else {
                            let a1364 = v0;
                            let a1365 = 69u64;
                            if nat_lt(a1364, a1365) {
                                500013540394364858u64
                            } else {
                                748580250866718886u64
                            }
                        }
                    }
                }
            } else {
                let a1366 = v0;
                let a1367 = 75u64;
                if nat_lt(a1366, a1367) {
                    let a1368 = v0;
                    let a1369 = 72u64;
                    if nat_lt(a1368, a1369) {
                        let a1370 = v0;
                        let a1371 = 71u64;
                        if nat_lt(a1370, a1371) {
                            1242879168328830382u64
                        } else {
                            1977374033974150939u64
                        }
                    } else {
                        let a1372 = v0;
                        let a1373 = 73u64;
                        if nat_lt(a1372, a1373) {
                            2944078676154940804u64
                        } else {
                            let a1374 = v0;
                            let a1375 = 74u64;
                            if nat_lt(a1374, a1375) {
                                3659926193048069267u64
                            } else {
                                4368137639120453308u64
                            }
                        }
                    }
                } else {
                    let a1376 = v0;
                    let a1377 = 77u64;
                    if nat_lt(a1376, a1377) {
                        let a1378 = v0;
                        let a1379 = 76u64;
                        if nat_lt(a1378, a1379) {
                            4836135668995329356u64
                        } else {
                            5532061633213252278u64
                        }
                    } else {
                        let a1380 = v0;
                        let a1381 = 78u64;
                        if nat_lt(a1380, a1381) {
                            6448918945643986474u64
                        } else {
                            let a1382 = v0;
                            let a1383 = 79u64;
                            if nat_lt(a1382, a1383) {
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

pub fn f108(v0: u64) -> u64 {
    let a1386 = {
        let a1384 = f124(v0, 14u32, 50u32);
        let a1385 = f124(v0, 18u32, 46u32);
        fixed_u64::bit_xor(a1384, a1385)
    };
    let a1387 = f124(v0, 41u32, 23u32);
    fixed_u64::bit_xor(a1386, a1387)
}

pub fn f109(v0: u64, v1: u64, v2: u64) -> u64 {
    let a1393 = {
        let a1388 = v0;
        let a1389 = v1;
        fixed_u64::bit_and(a1388, a1389)
    };
    let a1394 = {
        let a1391 = {
            let a1390 = v0;
            fixed_u64::bit_not(a1390)
        };
        let a1392 = v2;
        fixed_u64::bit_and(a1391, a1392)
    };
    fixed_u64::bit_xor(a1393, a1394)
}

pub fn f110(v0: u64) -> u64 {
    let a1397 = {
        let a1395 = f124(v0, 28u32, 36u32);
        let a1396 = f124(v0, 34u32, 30u32);
        fixed_u64::bit_xor(a1395, a1396)
    };
    let a1398 = f124(v0, 39u32, 25u32);
    fixed_u64::bit_xor(a1397, a1398)
}

pub fn f111(v0: u64, v1: u64, v2: u64) -> u64 {
    let a1407 = {
        let a1403 = {
            let a1399 = v0;
            let a1400 = v1;
            fixed_u64::bit_and(a1399, a1400)
        };
        let a1404 = {
            let a1401 = v0;
            let a1402 = v2;
            fixed_u64::bit_and(a1401, a1402)
        };
        fixed_u64::bit_xor(a1403, a1404)
    };
    let a1408 = {
        let a1405 = v1;
        let a1406 = v2;
        fixed_u64::bit_and(a1405, a1406)
    };
    fixed_u64::bit_xor(a1407, a1408)
}

pub fn f112(v0: Adt6) -> Adt6 {
    Adt6::C0(match v0.clone() {
        Adt6::C0(_, h1409, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1409
        }
    }, match v0.clone() {
        Adt6::C0(_, _, h1410, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1410
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, h1411, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1411
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, h1412, _, _, _, _, _, _, _, _, _, _, _) => {
            h1412
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, h1413, _, _, _, _, _, _, _, _, _, _) => {
            h1413
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, h1414, _, _, _, _, _, _, _, _, _) => {
            h1414
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, h1415, _, _, _, _, _, _, _, _) => {
            h1415
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, h1416, _, _, _, _, _, _, _) => {
            h1416
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, h1417, _, _, _, _, _, _) => {
            h1417
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, _, h1418, _, _, _, _, _) => {
            h1418
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, h1419, _, _, _, _) => {
            h1419
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, h1420, _, _, _) => {
            h1420
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1421, _, _) => {
            h1421
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1422, _) => {
            h1422
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1423) => {
            h1423
        }
    }, f68(f125(match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1424, _) => {
            h1424
        }
    }), f68(match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, h1425, _, _, _, _, _, _) => {
            h1425
        }
    }, f68(f126(match v0.clone() {
        Adt6::C0(_, h1426, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1426
        }
    }), match v0 {
        Adt6::C0(h1427, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1427
        }
    }))))
}

pub fn f113(v0: Fn12, p1431: u64, p1432: (u64, (u64, u64))) -> (u64, (u64, u64)) {
    let m1430 = (p1431, p1432);
    {
        let mut m1430 = m1430;
        loop {
            let step = {
                let (v1, v2) = m1430;
                let m1428 = v1;
                if m1428 == 0 {
                    Err::<(u64, (u64, (u64, u64))), (u64, (u64, u64))>(v2)
                } else {
                    let v3 = m1428 - 1;
                    Ok::<(u64, (u64, (u64, u64))), (u64, (u64, u64))>((v3, {
                        let c1429: Fn12 = v0.clone();
                        c1429.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m1430 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f114(v0: (u64, (u64, u64))) -> (u64, (u64, u64)) {
    let v1: u64 = {
        let (h1433, _) = v0;
        h1433
    };
    let v2: u64 = {
        let (h1434, _) = {
            let (_, h1435) = v0;
            h1435
        };
        h1434
    };
    let v3: u64 = {
        let (_, h1436) = {
            let (_, h1437) = v0;
            h1437
        };
        h1436
    };
    ({
        let a1438 = v1;
        let a1439 = 2u64;
        let a1440 = 0u64;
        nat_quot(a1438, a1439, a1440)
    }, (f88(v2, 1u32), {
        let a1444 = {
            let a1441 = v1;
            let a1442 = 2u64;
            let a1443 = 0u64;
            nat_rem(a1441, a1442, a1443)
        };
        let a1445 = 1u64;
        if nat_eq(a1444, a1445) {
            let a1446 = v3;
            let a1447 = v2;
            fixed_u64::bit_or(a1446, a1447)
        } else {
            v3
        }
    }))
}

pub fn f115(v0: Bytes, v1: u64) -> u64 {
    let m1450 = {
        let a1448 = v0;
        let a1449 = v1;
        index_bytes(a1448, a1449)
    };
    match m1450 {
        None => {
            0u64
        }
        Some(v2) => {
            let m1452 = {
                let a1451 = v2;
                fixed_u64::convert(i128::from(a1451))
            };
            match m1452 {
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

pub fn f116(v0: Adt10) -> Adt10 {
    let a1457 = {
        let a1454 = match v0.clone() {
            Adt10::C0(_, h1453, _) => {
                h1453
            }
        };
        let a1455 = 2u64;
        let a1456 = 0u64;
        nat_rem(a1454, a1455, a1456)
    };
    let a1458 = 0u64;
    if nat_eq(a1457, a1458) {
        let m1460 = match v0.clone() {
            Adt10::C0(_, _, h1459) => {
                h1459
            }
        };
        match m1460.uncons() {
            None => {
                v0
            }
            Some((v1, v2)) => {
                Adt10::C0(f79(v1, match v0.clone() {
                    Adt10::C0(h1461, _, _) => {
                        h1461
                    }
                }, false), {
                    let a1463 = match v0 {
                        Adt10::C0(_, h1462, _) => {
                            h1462
                        }
                    };
                    let a1464 = 2u64;
                    let a1465 = 0u64;
                    nat_quot(a1463, a1464, a1465)
                }, v2)
            }
        }
    } else {
        v0
    }
}

pub fn f117(v0: Adt9, v1: Adt8) -> Adt9 {
    let v2: Adt11 = f127(match v0.clone() {
        Adt9::C0(h1466, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1466
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, h1467, _, _, _, _, _, _, _, _, _, _) => {
            h1467
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, _, _, h1468, _, _, _, _, _) => {
            h1468
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1469) => {
            h1469
        }
    }, match v1.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, h1470, _, _, _, _, _, _, _) => {
            h1470
        }
    }, match v1.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, h1471, _, _, _, _, _, _) => {
            h1471
        }
    });
    let v3: Adt11 = f127(match v0.clone() {
        Adt9::C0(_, h1472, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1472
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, h1473, _, _, _, _, _, _, _, _, _) => {
            h1473
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, _, _, _, h1474, _, _, _, _) => {
            h1474
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, _, _, _, _, h1475, _, _, _) => {
            h1475
        }
    }, match v1.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, h1476, _, _, _, _, _) => {
            h1476
        }
    }, match v1.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, h1477, _, _, _, _) => {
            h1477
        }
    });
    let v4: Adt11 = f127(match v0.clone() {
        Adt9::C0(_, _, h1478, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1478
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, h1479, _, _, _, _, _, _, _, _) => {
            h1479
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, h1480, _, _, _, _, _, _, _) => {
            h1480
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1481, _, _) => {
            h1481
        }
    }, match v1.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, h1482, _, _, _) => {
            h1482
        }
    }, match v1.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1483, _, _) => {
            h1483
        }
    });
    let v5: Adt11 = f127(match v0.clone() {
        Adt9::C0(_, _, _, h1484, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1484
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, h1485, _, _, _, _, _, _, _, _, _, _, _) => {
            h1485
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, _, h1486, _, _, _, _, _, _) => {
            h1486
        }
    }, match v0 {
        Adt9::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1487, _) => {
            h1487
        }
    }, match v1.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1488, _) => {
            h1488
        }
    }, match v1 {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1489) => {
            h1489
        }
    });
    Adt9::C0(match v2.clone() {
        Adt11::C0(h1490, _, _, _) => {
            h1490
        }
    }, match v3.clone() {
        Adt11::C0(h1491, _, _, _) => {
            h1491
        }
    }, match v4.clone() {
        Adt11::C0(h1492, _, _, _) => {
            h1492
        }
    }, match v5.clone() {
        Adt11::C0(h1493, _, _, _) => {
            h1493
        }
    }, match v5.clone() {
        Adt11::C0(_, h1494, _, _) => {
            h1494
        }
    }, match v2.clone() {
        Adt11::C0(_, h1495, _, _) => {
            h1495
        }
    }, match v3.clone() {
        Adt11::C0(_, h1496, _, _) => {
            h1496
        }
    }, match v4.clone() {
        Adt11::C0(_, h1497, _, _) => {
            h1497
        }
    }, match v4.clone() {
        Adt11::C0(_, _, h1498, _) => {
            h1498
        }
    }, match v5.clone() {
        Adt11::C0(_, _, h1499, _) => {
            h1499
        }
    }, match v2.clone() {
        Adt11::C0(_, _, h1500, _) => {
            h1500
        }
    }, match v3.clone() {
        Adt11::C0(_, _, h1501, _) => {
            h1501
        }
    }, match v3 {
        Adt11::C0(_, _, _, h1502) => {
            h1502
        }
    }, match v4 {
        Adt11::C0(_, _, _, h1503) => {
            h1503
        }
    }, match v5 {
        Adt11::C0(_, _, _, h1504) => {
            h1504
        }
    }, match v2 {
        Adt11::C0(_, _, _, h1505) => {
            h1505
        }
    })
}

pub fn f118(v0: Adt9, v1: Adt8) -> Adt9 {
    let v2: Adt11 = f127(match v0.clone() {
        Adt9::C0(h1506, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1506
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, h1507, _, _, _, _, _, _, _, _, _, _, _) => {
            h1507
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, h1508, _, _, _, _, _, _, _) => {
            h1508
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, _, _, _, _, h1509, _, _, _) => {
            h1509
        }
    }, match v1.clone() {
        Adt8::C0(h1510, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1510
        }
    }, match v1.clone() {
        Adt8::C0(_, h1511, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1511
        }
    });
    let v3: Adt11 = f127(match v0.clone() {
        Adt9::C0(_, h1512, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1512
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, h1513, _, _, _, _, _, _, _, _, _, _) => {
            h1513
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, _, h1514, _, _, _, _, _, _) => {
            h1514
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1515, _, _) => {
            h1515
        }
    }, match v1.clone() {
        Adt8::C0(_, _, h1516, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1516
        }
    }, match v1.clone() {
        Adt8::C0(_, _, _, h1517, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1517
        }
    });
    let v4: Adt11 = f127(match v0.clone() {
        Adt9::C0(_, _, h1518, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1518
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, h1519, _, _, _, _, _, _, _, _, _) => {
            h1519
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, _, _, h1520, _, _, _, _, _) => {
            h1520
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1521, _) => {
            h1521
        }
    }, match v1.clone() {
        Adt8::C0(_, _, _, _, h1522, _, _, _, _, _, _, _, _, _, _, _) => {
            h1522
        }
    }, match v1.clone() {
        Adt8::C0(_, _, _, _, _, h1523, _, _, _, _, _, _, _, _, _, _) => {
            h1523
        }
    });
    let v5: Adt11 = f127(match v0.clone() {
        Adt9::C0(_, _, _, h1524, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1524
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, h1525, _, _, _, _, _, _, _, _) => {
            h1525
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, _, _, _, h1526, _, _, _, _) => {
            h1526
        }
    }, match v0 {
        Adt9::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1527) => {
            h1527
        }
    }, match v1.clone() {
        Adt8::C0(_, _, _, _, _, _, h1528, _, _, _, _, _, _, _, _, _) => {
            h1528
        }
    }, match v1 {
        Adt8::C0(_, _, _, _, _, _, _, h1529, _, _, _, _, _, _, _, _) => {
            h1529
        }
    });
    Adt9::C0(match v2.clone() {
        Adt11::C0(h1530, _, _, _) => {
            h1530
        }
    }, match v3.clone() {
        Adt11::C0(h1531, _, _, _) => {
            h1531
        }
    }, match v4.clone() {
        Adt11::C0(h1532, _, _, _) => {
            h1532
        }
    }, match v5.clone() {
        Adt11::C0(h1533, _, _, _) => {
            h1533
        }
    }, match v2.clone() {
        Adt11::C0(_, h1534, _, _) => {
            h1534
        }
    }, match v3.clone() {
        Adt11::C0(_, h1535, _, _) => {
            h1535
        }
    }, match v4.clone() {
        Adt11::C0(_, h1536, _, _) => {
            h1536
        }
    }, match v5.clone() {
        Adt11::C0(_, h1537, _, _) => {
            h1537
        }
    }, match v2.clone() {
        Adt11::C0(_, _, h1538, _) => {
            h1538
        }
    }, match v3.clone() {
        Adt11::C0(_, _, h1539, _) => {
            h1539
        }
    }, match v4.clone() {
        Adt11::C0(_, _, h1540, _) => {
            h1540
        }
    }, match v5.clone() {
        Adt11::C0(_, _, h1541, _) => {
            h1541
        }
    }, match v2 {
        Adt11::C0(_, _, _, h1542) => {
            h1542
        }
    }, match v3 {
        Adt11::C0(_, _, _, h1543) => {
            h1543
        }
    }, match v4 {
        Adt11::C0(_, _, _, h1544) => {
            h1544
        }
    }, match v5 {
        Adt11::C0(_, _, _, h1545) => {
            h1545
        }
    })
}

pub fn f119(v0: Adt8) -> Adt8 {
    Adt8::C0(match v0.clone() {
        Adt8::C0(_, _, h1546, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1546
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, h1547, _, _, _, _, _, _, _, _, _) => {
            h1547
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, h1548, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1548
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, h1549, _, _, _, _, _) => {
            h1549
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, h1550, _, _, _, _, _, _, _, _) => {
            h1550
        }
    }, match v0.clone() {
        Adt8::C0(h1551, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1551
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, h1552, _, _, _, _, _, _, _, _, _, _, _) => {
            h1552
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1553, _, _) => {
            h1553
        }
    }, match v0.clone() {
        Adt8::C0(_, h1554, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1554
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, h1555, _, _, _, _) => {
            h1555
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, h1556, _, _, _) => {
            h1556
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, _, h1557, _, _, _, _, _, _, _, _, _, _) => {
            h1557
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, h1558, _, _, _, _, _, _) => {
            h1558
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1559, _) => {
            h1559
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1560) => {
            h1560
        }
    }, match v0 {
        Adt8::C0(_, _, _, _, _, _, _, _, h1561, _, _, _, _, _, _, _) => {
            h1561
        }
    })
}

pub fn f120(v0: u32, v1: u32, v2: u32) -> u32 {
    let a1562 = f42(v0, v1);
    let a1563 = f96(v0, v2);
    fixed_u32::bit_or(a1562, a1563)
}

pub fn f121(v0: u32) -> u32 {
    let a1566 = {
        let a1564 = f120(v0, 17u32, 15u32);
        let a1565 = f120(v0, 19u32, 13u32);
        fixed_u32::bit_xor(a1564, a1565)
    };
    let a1567 = f42(v0, 10u32);
    fixed_u32::bit_xor(a1566, a1567)
}

pub fn f122(v0: u32) -> u32 {
    let a1570 = {
        let a1568 = f120(v0, 7u32, 25u32);
        let a1569 = f120(v0, 18u32, 14u32);
        fixed_u32::bit_xor(a1568, a1569)
    };
    let a1571 = f42(v0, 3u32);
    fixed_u32::bit_xor(a1570, a1571)
}

pub fn f123(v0: (u64, Adt3)) -> R<(u64, Adt3)> {
    Ok(({
        let a1573 = {
            let (h1572, _) = v0.clone();
            h1572
        };
        let a1574 = 1u64;
        nat_add(a1573, a1574)?
    }, f128(f129(f130(f131({
        let (_, h1575) = v0;
        h1575
    }))), {
        let (h1576, _) = v0;
        h1576
    })))
}

pub fn f124(v0: u64, v1: u32, v2: u32) -> u64 {
    let a1577 = f51(v0, v1);
    let a1578 = f88(v0, v2);
    fixed_u64::bit_or(a1577, a1578)
}

pub fn f125(v0: u64) -> u64 {
    let a1581 = {
        let a1579 = f124(v0, 19u32, 45u32);
        let a1580 = f124(v0, 61u32, 3u32);
        fixed_u64::bit_xor(a1579, a1580)
    };
    let a1582 = f51(v0, 6u32);
    fixed_u64::bit_xor(a1581, a1582)
}

pub fn f126(v0: u64) -> u64 {
    let a1585 = {
        let a1583 = f124(v0, 1u32, 63u32);
        let a1584 = f124(v0, 8u32, 56u32);
        fixed_u64::bit_xor(a1583, a1584)
    };
    let a1586 = f51(v0, 7u32);
    fixed_u64::bit_xor(a1585, a1586)
}

pub fn f127(v0: u32, v1: u32, v2: u32, v3: u32, v4: u32, v5: u32) -> Adt11 {
    let v6: u32 = f63(f63(v0, v1), v4);
    let v7: u32 = f120({
        let a1587 = v3;
        let a1588 = v6;
        fixed_u32::bit_xor(a1587, a1588)
    }, 16u32, 16u32);
    let v8: u32 = f63(v2, v7);
    let v9: u32 = f120({
        let a1589 = v1;
        let a1590 = v8;
        fixed_u32::bit_xor(a1589, a1590)
    }, 12u32, 20u32);
    let v10: u32 = f63(f63(v6, v9), v5);
    let v11: u32 = f120({
        let a1591 = v7;
        let a1592 = v10;
        fixed_u32::bit_xor(a1591, a1592)
    }, 8u32, 24u32);
    let v12: u32 = f63(v8, v11);
    let v13: u32 = f120({
        let a1593 = v9;
        let a1594 = v12;
        fixed_u32::bit_xor(a1593, a1594)
    }, 7u32, 25u32);
    Adt11::C0(v10, v13, v12, v11)
}

pub fn f128(v0: Adt3, v1: u64) -> Adt3 {
    let v2: u64 = f132(v1);
    Adt3::C0({
        let a1596 = match v0.clone() {
            Adt3::C0(h1595, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1595
            }
        };
        let a1597 = v2;
        fixed_u64::bit_xor(a1596, a1597)
    }, match v0.clone() {
        Adt3::C0(_, h1598, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1598
        }
    }, match v0.clone() {
        Adt3::C0(_, _, h1599, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1599
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, h1600, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1600
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, h1601, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1601
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, h1602, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1602
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, h1603, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1603
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, h1604, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1604
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, h1605, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1605
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, h1606, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1606
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, h1607, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1607
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, h1608, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1608
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, h1609, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1609
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1610, _, _, _, _, _, _, _, _, _, _, _) => {
            h1610
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1611, _, _, _, _, _, _, _, _, _, _) => {
            h1611
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1612, _, _, _, _, _, _, _, _, _) => {
            h1612
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1613, _, _, _, _, _, _, _, _) => {
            h1613
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1614, _, _, _, _, _, _, _) => {
            h1614
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1615, _, _, _, _, _, _) => {
            h1615
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1616, _, _, _, _, _) => {
            h1616
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1617, _, _, _, _) => {
            h1617
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1618, _, _, _) => {
            h1618
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1619, _, _) => {
            h1619
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1620, _) => {
            h1620
        }
    }, match v0 {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1621) => {
            h1621
        }
    })
}

pub fn f129(v0: Adt3) -> Adt3 {
    Adt3::C0({
        let a1628 = match v0.clone() {
            Adt3::C0(h1622, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1622
            }
        };
        let a1629 = {
            let a1626 = {
                let a1624 = match v0.clone() {
                    Adt3::C0(_, h1623, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1623
                    }
                };
                fixed_u64::bit_not(a1624)
            };
            let a1627 = match v0.clone() {
                Adt3::C0(_, _, h1625, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1625
                }
            };
            fixed_u64::bit_and(a1626, a1627)
        };
        fixed_u64::bit_xor(a1628, a1629)
    }, {
        let a1636 = match v0.clone() {
            Adt3::C0(_, h1630, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1630
            }
        };
        let a1637 = {
            let a1634 = {
                let a1632 = match v0.clone() {
                    Adt3::C0(_, _, h1631, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1631
                    }
                };
                fixed_u64::bit_not(a1632)
            };
            let a1635 = match v0.clone() {
                Adt3::C0(_, _, _, h1633, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1633
                }
            };
            fixed_u64::bit_and(a1634, a1635)
        };
        fixed_u64::bit_xor(a1636, a1637)
    }, {
        let a1644 = match v0.clone() {
            Adt3::C0(_, _, h1638, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1638
            }
        };
        let a1645 = {
            let a1642 = {
                let a1640 = match v0.clone() {
                    Adt3::C0(_, _, _, h1639, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1639
                    }
                };
                fixed_u64::bit_not(a1640)
            };
            let a1643 = match v0.clone() {
                Adt3::C0(_, _, _, _, h1641, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1641
                }
            };
            fixed_u64::bit_and(a1642, a1643)
        };
        fixed_u64::bit_xor(a1644, a1645)
    }, {
        let a1652 = match v0.clone() {
            Adt3::C0(_, _, _, h1646, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1646
            }
        };
        let a1653 = {
            let a1650 = {
                let a1648 = match v0.clone() {
                    Adt3::C0(_, _, _, _, h1647, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1647
                    }
                };
                fixed_u64::bit_not(a1648)
            };
            let a1651 = match v0.clone() {
                Adt3::C0(h1649, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1649
                }
            };
            fixed_u64::bit_and(a1650, a1651)
        };
        fixed_u64::bit_xor(a1652, a1653)
    }, {
        let a1660 = match v0.clone() {
            Adt3::C0(_, _, _, _, h1654, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1654
            }
        };
        let a1661 = {
            let a1658 = {
                let a1656 = match v0.clone() {
                    Adt3::C0(h1655, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1655
                    }
                };
                fixed_u64::bit_not(a1656)
            };
            let a1659 = match v0.clone() {
                Adt3::C0(_, h1657, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1657
                }
            };
            fixed_u64::bit_and(a1658, a1659)
        };
        fixed_u64::bit_xor(a1660, a1661)
    }, {
        let a1668 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, h1662, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1662
            }
        };
        let a1669 = {
            let a1666 = {
                let a1664 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h1663, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1663
                    }
                };
                fixed_u64::bit_not(a1664)
            };
            let a1667 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, h1665, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1665
                }
            };
            fixed_u64::bit_and(a1666, a1667)
        };
        fixed_u64::bit_xor(a1668, a1669)
    }, {
        let a1676 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, h1670, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1670
            }
        };
        let a1677 = {
            let a1674 = {
                let a1672 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, h1671, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1671
                    }
                };
                fixed_u64::bit_not(a1672)
            };
            let a1675 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, h1673, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1673
                }
            };
            fixed_u64::bit_and(a1674, a1675)
        };
        fixed_u64::bit_xor(a1676, a1677)
    }, {
        let a1684 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, h1678, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1678
            }
        };
        let a1685 = {
            let a1682 = {
                let a1680 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, h1679, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1679
                    }
                };
                fixed_u64::bit_not(a1680)
            };
            let a1683 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, _, h1681, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1681
                }
            };
            fixed_u64::bit_and(a1682, a1683)
        };
        fixed_u64::bit_xor(a1684, a1685)
    }, {
        let a1692 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, h1686, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1686
            }
        };
        let a1693 = {
            let a1690 = {
                let a1688 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, h1687, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1687
                    }
                };
                fixed_u64::bit_not(a1688)
            };
            let a1691 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, h1689, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1689
                }
            };
            fixed_u64::bit_and(a1690, a1691)
        };
        fixed_u64::bit_xor(a1692, a1693)
    }, {
        let a1700 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, h1694, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1694
            }
        };
        let a1701 = {
            let a1698 = {
                let a1696 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, h1695, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1695
                    }
                };
                fixed_u64::bit_not(a1696)
            };
            let a1699 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, h1697, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1697
                }
            };
            fixed_u64::bit_and(a1698, a1699)
        };
        fixed_u64::bit_xor(a1700, a1701)
    }, {
        let a1708 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, h1702, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1702
            }
        };
        let a1709 = {
            let a1706 = {
                let a1704 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, h1703, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1703
                    }
                };
                fixed_u64::bit_not(a1704)
            };
            let a1707 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, h1705, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1705
                }
            };
            fixed_u64::bit_and(a1706, a1707)
        };
        fixed_u64::bit_xor(a1708, a1709)
    }, {
        let a1716 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, h1710, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1710
            }
        };
        let a1717 = {
            let a1714 = {
                let a1712 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, h1711, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1711
                    }
                };
                fixed_u64::bit_not(a1712)
            };
            let a1715 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1713, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1713
                }
            };
            fixed_u64::bit_and(a1714, a1715)
        };
        fixed_u64::bit_xor(a1716, a1717)
    }, {
        let a1724 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, h1718, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1718
            }
        };
        let a1725 = {
            let a1722 = {
                let a1720 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1719, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1719
                    }
                };
                fixed_u64::bit_not(a1720)
            };
            let a1723 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1721, _, _, _, _, _, _, _, _, _, _) => {
                    h1721
                }
            };
            fixed_u64::bit_and(a1722, a1723)
        };
        fixed_u64::bit_xor(a1724, a1725)
    }, {
        let a1732 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1726, _, _, _, _, _, _, _, _, _, _, _) => {
                h1726
            }
        };
        let a1733 = {
            let a1730 = {
                let a1728 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1727, _, _, _, _, _, _, _, _, _, _) => {
                        h1727
                    }
                };
                fixed_u64::bit_not(a1728)
            };
            let a1731 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, _, _, h1729, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1729
                }
            };
            fixed_u64::bit_and(a1730, a1731)
        };
        fixed_u64::bit_xor(a1732, a1733)
    }, {
        let a1740 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1734, _, _, _, _, _, _, _, _, _, _) => {
                h1734
            }
        };
        let a1741 = {
            let a1738 = {
                let a1736 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, _, h1735, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1735
                    }
                };
                fixed_u64::bit_not(a1736)
            };
            let a1739 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, h1737, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1737
                }
            };
            fixed_u64::bit_and(a1738, a1739)
        };
        fixed_u64::bit_xor(a1740, a1741)
    }, {
        let a1748 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1742, _, _, _, _, _, _, _, _, _) => {
                h1742
            }
        };
        let a1749 = {
            let a1746 = {
                let a1744 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1743, _, _, _, _, _, _, _, _) => {
                        h1743
                    }
                };
                fixed_u64::bit_not(a1744)
            };
            let a1747 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1745, _, _, _, _, _, _, _) => {
                    h1745
                }
            };
            fixed_u64::bit_and(a1746, a1747)
        };
        fixed_u64::bit_xor(a1748, a1749)
    }, {
        let a1756 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1750, _, _, _, _, _, _, _, _) => {
                h1750
            }
        };
        let a1757 = {
            let a1754 = {
                let a1752 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1751, _, _, _, _, _, _, _) => {
                        h1751
                    }
                };
                fixed_u64::bit_not(a1752)
            };
            let a1755 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1753, _, _, _, _, _, _) => {
                    h1753
                }
            };
            fixed_u64::bit_and(a1754, a1755)
        };
        fixed_u64::bit_xor(a1756, a1757)
    }, {
        let a1764 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1758, _, _, _, _, _, _, _) => {
                h1758
            }
        };
        let a1765 = {
            let a1762 = {
                let a1760 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1759, _, _, _, _, _, _) => {
                        h1759
                    }
                };
                fixed_u64::bit_not(a1760)
            };
            let a1763 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1761, _, _, _, _, _) => {
                    h1761
                }
            };
            fixed_u64::bit_and(a1762, a1763)
        };
        fixed_u64::bit_xor(a1764, a1765)
    }, {
        let a1772 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1766, _, _, _, _, _, _) => {
                h1766
            }
        };
        let a1773 = {
            let a1770 = {
                let a1768 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1767, _, _, _, _, _) => {
                        h1767
                    }
                };
                fixed_u64::bit_not(a1768)
            };
            let a1771 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1769, _, _, _, _, _, _, _, _, _) => {
                    h1769
                }
            };
            fixed_u64::bit_and(a1770, a1771)
        };
        fixed_u64::bit_xor(a1772, a1773)
    }, {
        let a1780 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1774, _, _, _, _, _) => {
                h1774
            }
        };
        let a1781 = {
            let a1778 = {
                let a1776 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1775, _, _, _, _, _, _, _, _, _) => {
                        h1775
                    }
                };
                fixed_u64::bit_not(a1776)
            };
            let a1779 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1777, _, _, _, _, _, _, _, _) => {
                    h1777
                }
            };
            fixed_u64::bit_and(a1778, a1779)
        };
        fixed_u64::bit_xor(a1780, a1781)
    }, {
        let a1788 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1782, _, _, _, _) => {
                h1782
            }
        };
        let a1789 = {
            let a1786 = {
                let a1784 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1783, _, _, _) => {
                        h1783
                    }
                };
                fixed_u64::bit_not(a1784)
            };
            let a1787 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1785, _, _) => {
                    h1785
                }
            };
            fixed_u64::bit_and(a1786, a1787)
        };
        fixed_u64::bit_xor(a1788, a1789)
    }, {
        let a1796 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1790, _, _, _) => {
                h1790
            }
        };
        let a1797 = {
            let a1794 = {
                let a1792 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1791, _, _) => {
                        h1791
                    }
                };
                fixed_u64::bit_not(a1792)
            };
            let a1795 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1793, _) => {
                    h1793
                }
            };
            fixed_u64::bit_and(a1794, a1795)
        };
        fixed_u64::bit_xor(a1796, a1797)
    }, {
        let a1804 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1798, _, _) => {
                h1798
            }
        };
        let a1805 = {
            let a1802 = {
                let a1800 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1799, _) => {
                        h1799
                    }
                };
                fixed_u64::bit_not(a1800)
            };
            let a1803 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1801) => {
                    h1801
                }
            };
            fixed_u64::bit_and(a1802, a1803)
        };
        fixed_u64::bit_xor(a1804, a1805)
    }, {
        let a1812 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1806, _) => {
                h1806
            }
        };
        let a1813 = {
            let a1810 = {
                let a1808 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1807) => {
                        h1807
                    }
                };
                fixed_u64::bit_not(a1808)
            };
            let a1811 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1809, _, _, _, _) => {
                    h1809
                }
            };
            fixed_u64::bit_and(a1810, a1811)
        };
        fixed_u64::bit_xor(a1812, a1813)
    }, {
        let a1820 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1814) => {
                h1814
            }
        };
        let a1821 = {
            let a1818 = {
                let a1816 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1815, _, _, _, _) => {
                        h1815
                    }
                };
                fixed_u64::bit_not(a1816)
            };
            let a1819 = match v0 {
                Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1817, _, _, _) => {
                    h1817
                }
            };
            fixed_u64::bit_and(a1818, a1819)
        };
        fixed_u64::bit_xor(a1820, a1821)
    })
}

pub fn f130(v0: Adt3) -> Adt3 {
    Adt3::C0(match v0.clone() {
        Adt3::C0(h1822, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1822
        }
    }, f133(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, h1823, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1823
        }
    }, 44u32, 20u32), f133(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, h1824, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1824
        }
    }, 43u32, 21u32), f133(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1825, _, _, _, _, _, _) => {
            h1825
        }
    }, 21u32, 43u32), f133(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1826) => {
            h1826
        }
    }, 14u32, 50u32), f133(match v0.clone() {
        Adt3::C0(_, _, _, h1827, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1827
        }
    }, 28u32, 36u32), f133(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, h1828, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1828
        }
    }, 20u32, 44u32), f133(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, h1829, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1829
        }
    }, 3u32, 61u32), f133(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1830, _, _, _, _, _, _, _, _) => {
            h1830
        }
    }, 45u32, 19u32), f133(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1831, _, _) => {
            h1831
        }
    }, 61u32, 3u32), f133(match v0.clone() {
        Adt3::C0(_, h1832, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1832
        }
    }, 1u32, 63u32), f133(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, h1833, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1833
        }
    }, 6u32, 58u32), f133(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1834, _, _, _, _, _, _, _, _, _, _, _) => {
            h1834
        }
    }, 25u32, 39u32), f133(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1835, _, _, _, _, _) => {
            h1835
        }
    }, 8u32, 56u32), f133(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1836, _, _, _, _) => {
            h1836
        }
    }, 18u32, 46u32), f133(match v0.clone() {
        Adt3::C0(_, _, _, _, h1837, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1837
        }
    }, 27u32, 37u32), f133(match v0.clone() {
        Adt3::C0(_, _, _, _, _, h1838, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1838
        }
    }, 36u32, 28u32), f133(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, h1839, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1839
        }
    }, 10u32, 54u32), f133(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1840, _, _, _, _, _, _, _) => {
            h1840
        }
    }, 15u32, 49u32), f133(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1841, _) => {
            h1841
        }
    }, 56u32, 8u32), f133(match v0.clone() {
        Adt3::C0(_, _, h1842, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1842
        }
    }, 62u32, 2u32), f133(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, h1843, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1843
        }
    }, 55u32, 9u32), f133(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1844, _, _, _, _, _, _, _, _, _, _) => {
            h1844
        }
    }, 39u32, 25u32), f133(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1845, _, _, _, _, _, _, _, _, _) => {
            h1845
        }
    }, 41u32, 23u32), f133(match v0 {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1846, _, _, _) => {
            h1846
        }
    }, 2u32, 62u32))
}

pub fn f131(v0: Adt3) -> Adt3 {
    let v1: u64 = {
        let a1858 = {
            let a1855 = {
                let a1852 = {
                    let a1849 = match v0.clone() {
                        Adt3::C0(h1847, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1847
                        }
                    };
                    let a1850 = match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h1848, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1848
                        }
                    };
                    fixed_u64::bit_xor(a1849, a1850)
                };
                let a1853 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, _, h1851, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1851
                    }
                };
                fixed_u64::bit_xor(a1852, a1853)
            };
            let a1856 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1854, _, _, _, _, _, _, _, _, _) => {
                    h1854
                }
            };
            fixed_u64::bit_xor(a1855, a1856)
        };
        let a1859 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1857, _, _, _, _) => {
                h1857
            }
        };
        fixed_u64::bit_xor(a1858, a1859)
    };
    let v2: u64 = {
        let a1871 = {
            let a1868 = {
                let a1865 = {
                    let a1862 = match v0.clone() {
                        Adt3::C0(_, h1860, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1860
                        }
                    };
                    let a1863 = match v0.clone() {
                        Adt3::C0(_, _, _, _, _, _, h1861, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1861
                        }
                    };
                    fixed_u64::bit_xor(a1862, a1863)
                };
                let a1866 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, h1864, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1864
                    }
                };
                fixed_u64::bit_xor(a1865, a1866)
            };
            let a1869 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1867, _, _, _, _, _, _, _, _) => {
                    h1867
                }
            };
            fixed_u64::bit_xor(a1868, a1869)
        };
        let a1872 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1870, _, _, _) => {
                h1870
            }
        };
        fixed_u64::bit_xor(a1871, a1872)
    };
    let v3: u64 = {
        let a1884 = {
            let a1881 = {
                let a1878 = {
                    let a1875 = match v0.clone() {
                        Adt3::C0(_, _, h1873, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1873
                        }
                    };
                    let a1876 = match v0.clone() {
                        Adt3::C0(_, _, _, _, _, _, _, h1874, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1874
                        }
                    };
                    fixed_u64::bit_xor(a1875, a1876)
                };
                let a1879 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, h1877, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1877
                    }
                };
                fixed_u64::bit_xor(a1878, a1879)
            };
            let a1882 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1880, _, _, _, _, _, _, _) => {
                    h1880
                }
            };
            fixed_u64::bit_xor(a1881, a1882)
        };
        let a1885 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1883, _, _) => {
                h1883
            }
        };
        fixed_u64::bit_xor(a1884, a1885)
    };
    let v4: u64 = {
        let a1897 = {
            let a1894 = {
                let a1891 = {
                    let a1888 = match v0.clone() {
                        Adt3::C0(_, _, _, h1886, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1886
                        }
                    };
                    let a1889 = match v0.clone() {
                        Adt3::C0(_, _, _, _, _, _, _, _, h1887, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1887
                        }
                    };
                    fixed_u64::bit_xor(a1888, a1889)
                };
                let a1892 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1890, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1890
                    }
                };
                fixed_u64::bit_xor(a1891, a1892)
            };
            let a1895 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1893, _, _, _, _, _, _) => {
                    h1893
                }
            };
            fixed_u64::bit_xor(a1894, a1895)
        };
        let a1898 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1896, _) => {
                h1896
            }
        };
        fixed_u64::bit_xor(a1897, a1898)
    };
    let v5: u64 = {
        let a1910 = {
            let a1907 = {
                let a1904 = {
                    let a1901 = match v0.clone() {
                        Adt3::C0(_, _, _, _, h1899, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1899
                        }
                    };
                    let a1902 = match v0.clone() {
                        Adt3::C0(_, _, _, _, _, _, _, _, _, h1900, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1900
                        }
                    };
                    fixed_u64::bit_xor(a1901, a1902)
                };
                let a1905 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1903, _, _, _, _, _, _, _, _, _, _) => {
                        h1903
                    }
                };
                fixed_u64::bit_xor(a1904, a1905)
            };
            let a1908 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1906, _, _, _, _, _) => {
                    h1906
                }
            };
            fixed_u64::bit_xor(a1907, a1908)
        };
        let a1911 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1909) => {
                h1909
            }
        };
        fixed_u64::bit_xor(a1910, a1911)
    };
    let v6: u64 = {
        let a1912 = v5;
        let a1913 = f133(v2, 1u32, 63u32);
        fixed_u64::bit_xor(a1912, a1913)
    };
    let v7: u64 = {
        let a1914 = v1;
        let a1915 = f133(v3, 1u32, 63u32);
        fixed_u64::bit_xor(a1914, a1915)
    };
    let v8: u64 = {
        let a1916 = v2;
        let a1917 = f133(v4, 1u32, 63u32);
        fixed_u64::bit_xor(a1916, a1917)
    };
    let v9: u64 = {
        let a1918 = v3;
        let a1919 = f133(v5, 1u32, 63u32);
        fixed_u64::bit_xor(a1918, a1919)
    };
    let v10: u64 = {
        let a1920 = v4;
        let a1921 = f133(v1, 1u32, 63u32);
        fixed_u64::bit_xor(a1920, a1921)
    };
    Adt3::C0({
        let a1923 = match v0.clone() {
            Adt3::C0(h1922, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1922
            }
        };
        let a1924 = v6;
        fixed_u64::bit_xor(a1923, a1924)
    }, {
        let a1926 = match v0.clone() {
            Adt3::C0(_, h1925, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1925
            }
        };
        let a1927 = v7;
        fixed_u64::bit_xor(a1926, a1927)
    }, {
        let a1929 = match v0.clone() {
            Adt3::C0(_, _, h1928, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1928
            }
        };
        let a1930 = v8;
        fixed_u64::bit_xor(a1929, a1930)
    }, {
        let a1932 = match v0.clone() {
            Adt3::C0(_, _, _, h1931, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1931
            }
        };
        let a1933 = v9;
        fixed_u64::bit_xor(a1932, a1933)
    }, {
        let a1935 = match v0.clone() {
            Adt3::C0(_, _, _, _, h1934, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1934
            }
        };
        let a1936 = v10;
        fixed_u64::bit_xor(a1935, a1936)
    }, {
        let a1938 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, h1937, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1937
            }
        };
        let a1939 = v6;
        fixed_u64::bit_xor(a1938, a1939)
    }, {
        let a1941 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, h1940, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1940
            }
        };
        let a1942 = v7;
        fixed_u64::bit_xor(a1941, a1942)
    }, {
        let a1944 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, h1943, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1943
            }
        };
        let a1945 = v8;
        fixed_u64::bit_xor(a1944, a1945)
    }, {
        let a1947 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, h1946, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1946
            }
        };
        let a1948 = v9;
        fixed_u64::bit_xor(a1947, a1948)
    }, {
        let a1950 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, h1949, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1949
            }
        };
        let a1951 = v10;
        fixed_u64::bit_xor(a1950, a1951)
    }, {
        let a1953 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, h1952, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1952
            }
        };
        let a1954 = v6;
        fixed_u64::bit_xor(a1953, a1954)
    }, {
        let a1956 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, h1955, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1955
            }
        };
        let a1957 = v7;
        fixed_u64::bit_xor(a1956, a1957)
    }, {
        let a1959 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, h1958, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1958
            }
        };
        let a1960 = v8;
        fixed_u64::bit_xor(a1959, a1960)
    }, {
        let a1962 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1961, _, _, _, _, _, _, _, _, _, _, _) => {
                h1961
            }
        };
        let a1963 = v9;
        fixed_u64::bit_xor(a1962, a1963)
    }, {
        let a1965 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1964, _, _, _, _, _, _, _, _, _, _) => {
                h1964
            }
        };
        let a1966 = v10;
        fixed_u64::bit_xor(a1965, a1966)
    }, {
        let a1968 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1967, _, _, _, _, _, _, _, _, _) => {
                h1967
            }
        };
        let a1969 = v6;
        fixed_u64::bit_xor(a1968, a1969)
    }, {
        let a1971 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1970, _, _, _, _, _, _, _, _) => {
                h1970
            }
        };
        let a1972 = v7;
        fixed_u64::bit_xor(a1971, a1972)
    }, {
        let a1974 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1973, _, _, _, _, _, _, _) => {
                h1973
            }
        };
        let a1975 = v8;
        fixed_u64::bit_xor(a1974, a1975)
    }, {
        let a1977 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1976, _, _, _, _, _, _) => {
                h1976
            }
        };
        let a1978 = v9;
        fixed_u64::bit_xor(a1977, a1978)
    }, {
        let a1980 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1979, _, _, _, _, _) => {
                h1979
            }
        };
        let a1981 = v10;
        fixed_u64::bit_xor(a1980, a1981)
    }, {
        let a1983 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1982, _, _, _, _) => {
                h1982
            }
        };
        let a1984 = v6;
        fixed_u64::bit_xor(a1983, a1984)
    }, {
        let a1986 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1985, _, _, _) => {
                h1985
            }
        };
        let a1987 = v7;
        fixed_u64::bit_xor(a1986, a1987)
    }, {
        let a1989 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1988, _, _) => {
                h1988
            }
        };
        let a1990 = v8;
        fixed_u64::bit_xor(a1989, a1990)
    }, {
        let a1992 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1991, _) => {
                h1991
            }
        };
        let a1993 = v9;
        fixed_u64::bit_xor(a1992, a1993)
    }, {
        let a1995 = match v0 {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1994) => {
                h1994
            }
        };
        let a1996 = v10;
        fixed_u64::bit_xor(a1995, a1996)
    })
}

pub fn f132(v0: u64) -> u64 {
    let a1997 = v0;
    let a1998 = 12u64;
    if nat_lt(a1997, a1998) {
        let a1999 = v0;
        let a2000 = 6u64;
        if nat_lt(a1999, a2000) {
            let a2001 = v0;
            let a2002 = 3u64;
            if nat_lt(a2001, a2002) {
                let a2003 = v0;
                let a2004 = 1u64;
                if nat_lt(a2003, a2004) {
                    1u64
                } else {
                    let a2005 = v0;
                    let a2006 = 2u64;
                    if nat_lt(a2005, a2006) {
                        32898u64
                    } else {
                        9223372036854808714u64
                    }
                }
            } else {
                let a2007 = v0;
                let a2008 = 4u64;
                if nat_lt(a2007, a2008) {
                    9223372039002292224u64
                } else {
                    let a2009 = v0;
                    let a2010 = 5u64;
                    if nat_lt(a2009, a2010) {
                        32907u64
                    } else {
                        2147483649u64
                    }
                }
            }
        } else {
            let a2011 = v0;
            let a2012 = 9u64;
            if nat_lt(a2011, a2012) {
                let a2013 = v0;
                let a2014 = 7u64;
                if nat_lt(a2013, a2014) {
                    9223372039002292353u64
                } else {
                    let a2015 = v0;
                    let a2016 = 8u64;
                    if nat_lt(a2015, a2016) {
                        9223372036854808585u64
                    } else {
                        138u64
                    }
                }
            } else {
                let a2017 = v0;
                let a2018 = 10u64;
                if nat_lt(a2017, a2018) {
                    136u64
                } else {
                    let a2019 = v0;
                    let a2020 = 11u64;
                    if nat_lt(a2019, a2020) {
                        2147516425u64
                    } else {
                        2147483658u64
                    }
                }
            }
        }
    } else {
        let a2021 = v0;
        let a2022 = 18u64;
        if nat_lt(a2021, a2022) {
            let a2023 = v0;
            let a2024 = 15u64;
            if nat_lt(a2023, a2024) {
                let a2025 = v0;
                let a2026 = 13u64;
                if nat_lt(a2025, a2026) {
                    2147516555u64
                } else {
                    let a2027 = v0;
                    let a2028 = 14u64;
                    if nat_lt(a2027, a2028) {
                        9223372036854775947u64
                    } else {
                        9223372036854808713u64
                    }
                }
            } else {
                let a2029 = v0;
                let a2030 = 16u64;
                if nat_lt(a2029, a2030) {
                    9223372036854808579u64
                } else {
                    let a2031 = v0;
                    let a2032 = 17u64;
                    if nat_lt(a2031, a2032) {
                        9223372036854808578u64
                    } else {
                        9223372036854775936u64
                    }
                }
            }
        } else {
            let a2033 = v0;
            let a2034 = 21u64;
            if nat_lt(a2033, a2034) {
                let a2035 = v0;
                let a2036 = 19u64;
                if nat_lt(a2035, a2036) {
                    32778u64
                } else {
                    let a2037 = v0;
                    let a2038 = 20u64;
                    if nat_lt(a2037, a2038) {
                        9223372039002259466u64
                    } else {
                        9223372039002292353u64
                    }
                }
            } else {
                let a2039 = v0;
                let a2040 = 22u64;
                if nat_lt(a2039, a2040) {
                    9223372036854808704u64
                } else {
                    let a2041 = v0;
                    let a2042 = 23u64;
                    if nat_lt(a2041, a2042) {
                        2147483649u64
                    } else {
                        9223372039002292232u64
                    }
                }
            }
        }
    }
}

pub fn f133(v0: u64, v1: u32, v2: u32) -> u64 {
    let a2043 = f88(v0, v1);
    let a2044 = f51(v0, v2);
    fixed_u64::bit_or(a2043, a2044)
}
