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
    F15(Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn0::F15(k0) => f15(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F32(Bytes, u64, u64),
}

impl Fn1 {
    pub fn apply(&self, p0: (u64, Adt2)) -> R<(u64, Adt2)> {
        match self {
            Fn1::F32(k0, k1, k2) => f32(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F37(Bytes, u64, u64),
}

impl Fn2 {
    pub fn apply(&self, p0: (u64, Adt0)) -> R<(u64, Adt0)> {
        match self {
            Fn2::F37(k0, k1, k2) => f37(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F43,
}

impl Fn3 {
    pub fn apply(&self, p0: (List<Adt1>, Adt1)) -> (List<Adt1>, Adt1) {
        match self {
            Fn3::F43 => f43(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F50(Bytes, u64, u64, u64),
    F108,
}

impl Fn4 {
    pub fn apply(&self, p0: (u64, Adt3)) -> R<(u64, Adt3)> {
        match self {
            Fn4::F50(k0, k1, k2, k3) => f50(k0.clone(), *k1, *k2, *k3, p0),
            Fn4::F108 => f108(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F57(Bytes, u64),
}

impl Fn5 {
    pub fn apply(&self, p0: (u64, List<Adt1>)) -> R<(u64, List<Adt1>)> {
        match self {
            Fn5::F57(k0, k1) => f57(k0.clone(), *k1, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F66,
}

impl Fn6 {
    pub fn apply(&self, p0: Adt5) -> R<Adt5> {
        match self {
            Fn6::F66 => f66(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F71,
}

impl Fn7 {
    pub fn apply(&self, p0: Adt7) -> R<Adt7> {
        match self {
            Fn7::F71 => f71(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F76,
}

impl Fn8 {
    pub fn apply(&self, p0: (Adt9, Adt8)) -> (Adt9, Adt8) {
        match self {
            Fn8::F76 => f76(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F78(u64, Bytes, u64),
}

impl Fn9 {
    pub fn apply(&self, p0: (u64, Adt1)) -> R<(u64, Adt1)> {
        match self {
            Fn9::F78(k0, k1, k2) => f78(*k0, k1.clone(), *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F82,
}

impl Fn10 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn10::F82 => f82(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F97,
}

impl Fn11 {
    pub fn apply(&self, p0: Adt10) -> Adt10 {
        match self {
            Fn11::F97 => f97(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F118,
}

impl Fn12 {
    pub fn apply(&self, p0: (u64, (u64, u64))) -> (u64, (u64, u64)) {
        match self {
            Fn12::F118 => f118(p0),
        }
    }
}

pub fn f0(v0: Bytes, v1: Bytes) -> R<Option<Bytes>> {
    f1(f2(Str::lit("sha256")), v0, v1)
}

pub fn f1(v0: Bytes, v1: Bytes, v2: Bytes) -> R<Option<Bytes>> {
    if f3(v0.clone(), v1.clone())? {
        if f3(v0.clone(), v2.clone())? {
            f4(v0, {
                let a1 = f5(v1.clone(), v2.clone());
                let a2 = f6(v1, v2);
                append_bytes(a1, a2)
            })
        } else {
            Ok(None::<Bytes>)
        }
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f2(v0: Str) -> Bytes {
    let a3 = v0;
    utf8_encode(a3)
}

pub fn f3(v0: Bytes, v1: Bytes) -> R<bool> {
    let a4 = 0u64;
    let a5 = f7(v0.clone());
    if nat_lt(a4, a5) {
        let a10 = {
            let a6 = v1.clone();
            length_bytes(a6)
        };
        let a11 = {
            let a8 = {
                let a7 = f8(v0.clone());
                length_bytes(a7)
            };
            let a9 = f7(v0.clone());
            nat_add(a8, a9)?
        };
        if nat_eq(a10, a11) {
            if f9(v1.clone(), f8(v0.clone())) {
                let a14 = f10(v1.clone(), {
                    let a12 = f8(v0);
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

pub fn f4(v0: Bytes, v1: Bytes) -> R<Option<Bytes>> {
    let m16 = f11(v0.clone(), v1)?;
    match m16 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v2) => {
            Ok(Some({
                let a17 = f8(v0);
                let a18 = v2;
                append_bytes(a17, a18)
            }))
        }
    }
}

pub fn f5(v0: Bytes, v1: Bytes) -> Bytes {
    if f12(v1.clone(), v0.clone()) {
        v1
    } else {
        v0
    }
}

pub fn f6(v0: Bytes, v1: Bytes) -> Bytes {
    if f12(v1.clone(), v0.clone()) {
        v0
    } else {
        v1
    }
}

pub fn f7(v0: Bytes) -> u64 {
    if f13(v0.clone(), Str::lit("sha256")) {
        64u64
    } else {
        if f13(v0.clone(), Str::lit("sha3-256")) {
            64u64
        } else {
            if f13(v0.clone(), Str::lit("keccak256")) {
                64u64
            } else {
                if f13(v0.clone(), Str::lit("sha512")) {
                    128u64
                } else {
                    if f13(v0, Str::lit("blake3")) {
                        64u64
                    } else {
                        0u64
                    }
                }
            }
        }
    }
}

pub fn f8(v0: Bytes) -> Bytes {
    let a19 = v0;
    let a20 = f2(Str::lit(":"));
    append_bytes(a19, a20)
}

pub fn f9(v0: Bytes, v1: Bytes) -> bool {
    let m25 = {
        let a22 = v0;
        let a23 = 0u64;
        let a24 = {
            let a21 = v1.clone();
            length_bytes(a21)
        };
        slice_bytes(a22, a23, a24)
    };
    match m25 {
        None => {
            false
        }
        Some(v2) => {
            let a26 = v2;
            let a27 = v1;
            equal(a26, a27)
        }
    }
}

pub fn f10(v0: Bytes, v1: u64) -> R<u64> {
    let (h28, _) = f14(Fn0::F15(v0.clone()), {
        let a29 = v0;
        length_bytes(a29)
    }, v1)?;
    Ok(h28)
}

pub fn f11(v0: Bytes, v1: Bytes) -> R<Option<Bytes>> {
    if f13(v0.clone(), Str::lit("sha256")) {
        Ok(Some(f16(v1)?))
    } else {
        if f13(v0.clone(), Str::lit("sha3-256")) {
            Ok(Some(f17(v1)?))
        } else {
            if f13(v0.clone(), Str::lit("keccak256")) {
                Ok(Some(f18(v1)?))
            } else {
                if f13(v0.clone(), Str::lit("sha512")) {
                    Ok(Some(f19(v1)?))
                } else {
                    if f13(v0, Str::lit("blake3")) {
                        Ok(Some(f20(v1)?))
                    } else {
                        Ok(None::<Bytes>)
                    }
                }
            }
        }
    }
}

pub fn f12(v0: Bytes, v1: Bytes) -> bool {
    let a34 = {
        let a30 = v0;
        let a31 = v1;
        compare_bytes(a30, a31)
    };
    let a35 = {
        let a32 = Bytes::lit(&[]);
        let a33 = Bytes::lit(&[0x01]);
        compare_bytes(a32, a33)
    };
    equal(a34, a35)
}

pub fn f13(v0: Bytes, v1: Str) -> bool {
    let a36 = v0;
    let a37 = f2(v1);
    equal(a36, a37)
}

pub fn f14(v0: Fn0, p42: u64, p43: u64) -> R<(u64, bool)> {
    let m41 = (p42, p43);
    {
        let mut m41 = m41;
        loop {
            let step = {
                let (v1, v2) = m41;
                let m38 = v1;
                if m38 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m38 - 1;
                    let m40 = {
                        let c39: Fn0 = v0.clone();
                        c39.apply(v2)?
                    };
                    match m40 {
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
                Ok(next) => { m41 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f15(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f21(v0, v1)? {
        Ok(Some({
            let a44 = v1;
            let a45 = 1u64;
            nat_add(a44, a45)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f16(v0: Bytes) -> R<Bytes> {
    Ok(f22(f23(v0)?))
}

pub fn f17(v0: Bytes) -> R<Bytes> {
    f24(v0, 6u64)
}

pub fn f18(v0: Bytes) -> R<Bytes> {
    f24(v0, 1u64)
}

pub fn f19(v0: Bytes) -> R<Bytes> {
    let v1: Adt0 = f25(v0)?;
    let a66 = f26(match v1.clone() {
        Adt0::C0(h46, _, _, _, _, _, _, _) => {
            h46
        }
    });
    let a67 = {
        let a64 = f26(match v1.clone() {
            Adt0::C0(_, h47, _, _, _, _, _, _) => {
                h47
            }
        });
        let a65 = {
            let a62 = f26(match v1.clone() {
                Adt0::C0(_, _, h48, _, _, _, _, _) => {
                    h48
                }
            });
            let a63 = {
                let a60 = f26(match v1.clone() {
                    Adt0::C0(_, _, _, h49, _, _, _, _) => {
                        h49
                    }
                });
                let a61 = {
                    let a58 = f26(match v1.clone() {
                        Adt0::C0(_, _, _, _, h50, _, _, _) => {
                            h50
                        }
                    });
                    let a59 = {
                        let a56 = f26(match v1.clone() {
                            Adt0::C0(_, _, _, _, _, h51, _, _) => {
                                h51
                            }
                        });
                        let a57 = {
                            let a54 = f26(match v1.clone() {
                                Adt0::C0(_, _, _, _, _, _, h52, _) => {
                                    h52
                                }
                            });
                            let a55 = f26(match v1 {
                                Adt0::C0(_, _, _, _, _, _, _, h53) => {
                                    h53
                                }
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
    Ok(append_bytes(a66, a67))
}

pub fn f20(v0: Bytes) -> R<Bytes> {
    let v1: Adt1 = f27(v0)?;
    let a88 = f28(match v1.clone() {
        Adt1::C0(h68, _, _, _, _, _, _, _) => {
            h68
        }
    });
    let a89 = {
        let a86 = f28(match v1.clone() {
            Adt1::C0(_, h69, _, _, _, _, _, _) => {
                h69
            }
        });
        let a87 = {
            let a84 = f28(match v1.clone() {
                Adt1::C0(_, _, h70, _, _, _, _, _) => {
                    h70
                }
            });
            let a85 = {
                let a82 = f28(match v1.clone() {
                    Adt1::C0(_, _, _, h71, _, _, _, _) => {
                        h71
                    }
                });
                let a83 = {
                    let a80 = f28(match v1.clone() {
                        Adt1::C0(_, _, _, _, h72, _, _, _) => {
                            h72
                        }
                    });
                    let a81 = {
                        let a78 = f28(match v1.clone() {
                            Adt1::C0(_, _, _, _, _, h73, _, _) => {
                                h73
                            }
                        });
                        let a79 = {
                            let a76 = f28(match v1.clone() {
                                Adt1::C0(_, _, _, _, _, _, h74, _) => {
                                    h74
                                }
                            });
                            let a77 = f28(match v1 {
                                Adt1::C0(_, _, _, _, _, _, _, h75) => {
                                    h75
                                }
                            });
                            append_bytes(a76, a77)
                        };
                        append_bytes(a78, a79)
                    };
                    append_bytes(a80, a81)
                };
                append_bytes(a82, a83)
            };
            append_bytes(a84, a85)
        };
        append_bytes(a86, a87)
    };
    Ok(append_bytes(a88, a89))
}

pub fn f21(v0: Bytes, v1: u64) -> R<bool> {
    let v2: u64 = f29(v0, v1)?;
    let a90 = 47u64;
    let a91 = v2;
    if nat_lt(a90, a91) {
        let a92 = v2;
        let a93 = 58u64;
        if nat_lt(a92, a93) {
            Ok(true)
        } else {
            let a94 = 96u64;
            let a95 = v2;
            if nat_lt(a94, a95) {
                let a96 = v2;
                let a97 = 103u64;
                Ok(nat_lt(a96, a97))
            } else {
                Ok(false)
            }
        }
    } else {
        Ok(false)
    }
}

pub fn f22(v0: Adt2) -> Bytes {
    let a118 = f30(match v0.clone() {
        Adt2::C0(h98, _, _, _, _, _, _, _) => {
            h98
        }
    });
    let a119 = {
        let a116 = f30(match v0.clone() {
            Adt2::C0(_, h99, _, _, _, _, _, _) => {
                h99
            }
        });
        let a117 = {
            let a114 = f30(match v0.clone() {
                Adt2::C0(_, _, h100, _, _, _, _, _) => {
                    h100
                }
            });
            let a115 = {
                let a112 = f30(match v0.clone() {
                    Adt2::C0(_, _, _, h101, _, _, _, _) => {
                        h101
                    }
                });
                let a113 = {
                    let a110 = f30(match v0.clone() {
                        Adt2::C0(_, _, _, _, h102, _, _, _) => {
                            h102
                        }
                    });
                    let a111 = {
                        let a108 = f30(match v0.clone() {
                            Adt2::C0(_, _, _, _, _, h103, _, _) => {
                                h103
                            }
                        });
                        let a109 = {
                            let a106 = f30(match v0.clone() {
                                Adt2::C0(_, _, _, _, _, _, h104, _) => {
                                    h104
                                }
                            });
                            let a107 = f30(match v0 {
                                Adt2::C0(_, _, _, _, _, _, _, h105) => {
                                    h105
                                }
                            });
                            append_bytes(a106, a107)
                        };
                        append_bytes(a108, a109)
                    };
                    append_bytes(a110, a111)
                };
                append_bytes(a112, a113)
            };
            append_bytes(a114, a115)
        };
        append_bytes(a116, a117)
    };
    append_bytes(a118, a119)
}

pub fn f23(v0: Bytes) -> R<Adt2> {
    let v1: u64 = {
        let a120 = v0.clone();
        length_bytes(a120)
    };
    let v2: u64 = {
        let a126 = {
            let a123 = {
                let a121 = v1;
                let a122 = 72u64;
                nat_add(a121, a122)?
            };
            let a124 = 64u64;
            let a125 = 0u64;
            nat_quot(a123, a124, a125)
        };
        let a127 = 64u64;
        nat_mul(a126, a127)?
    };
    let (_, h128) = f31(Fn1::F32(v0, v1, v2), {
        let a129 = v2;
        let a130 = 64u64;
        let a131 = 0u64;
        nat_quot(a129, a130, a131)
    }, (0u64, f33()))?;
    Ok(h128)
}

pub fn f24(v0: Bytes, v1: u64) -> R<Bytes> {
    let v2: Adt3 = f34(v0, v1)?;
    let a140 = f35(match v2.clone() {
        Adt3::C0(h132, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h132
        }
    });
    let a141 = {
        let a138 = f35(match v2.clone() {
            Adt3::C0(_, h133, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h133
            }
        });
        let a139 = {
            let a136 = f35(match v2.clone() {
                Adt3::C0(_, _, h134, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h134
                }
            });
            let a137 = f35(match v2 {
                Adt3::C0(_, _, _, h135, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h135
                }
            });
            append_bytes(a136, a137)
        };
        append_bytes(a138, a139)
    };
    Ok(append_bytes(a140, a141))
}

pub fn f25(v0: Bytes) -> R<Adt0> {
    let v1: u64 = {
        let a142 = v0.clone();
        length_bytes(a142)
    };
    let v2: u64 = {
        let a148 = {
            let a145 = {
                let a143 = v1;
                let a144 = 144u64;
                nat_add(a143, a144)?
            };
            let a146 = 128u64;
            let a147 = 0u64;
            nat_quot(a145, a146, a147)
        };
        let a149 = 128u64;
        nat_mul(a148, a149)?
    };
    let (_, h150) = f36(Fn2::F37(v0, v1, v2), {
        let a151 = v2;
        let a152 = 128u64;
        let a153 = 0u64;
        nat_quot(a151, a152, a153)
    }, (0u64, f38()))?;
    Ok(h150)
}

pub fn f26(v0: u64) -> Bytes {
    let a182 = f39(v0, 60u32);
    let a183 = {
        let a180 = f39(v0, 56u32);
        let a181 = {
            let a178 = f39(v0, 52u32);
            let a179 = {
                let a176 = f39(v0, 48u32);
                let a177 = {
                    let a174 = f39(v0, 44u32);
                    let a175 = {
                        let a172 = f39(v0, 40u32);
                        let a173 = {
                            let a170 = f39(v0, 36u32);
                            let a171 = {
                                let a168 = f39(v0, 32u32);
                                let a169 = {
                                    let a166 = f39(v0, 28u32);
                                    let a167 = {
                                        let a164 = f39(v0, 24u32);
                                        let a165 = {
                                            let a162 = f39(v0, 20u32);
                                            let a163 = {
                                                let a160 = f39(v0, 16u32);
                                                let a161 = {
                                                    let a158 = f39(v0, 12u32);
                                                    let a159 = {
                                                        let a156 = f39(v0, 8u32);
                                                        let a157 = {
                                                            let a154 = f39(v0, 4u32);
                                                            let a155 = f39(v0, 0u32);
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
                };
                append_bytes(a176, a177)
            };
            append_bytes(a178, a179)
        };
        append_bytes(a180, a181)
    };
    append_bytes(a182, a183)
}

pub fn f27(v0: Bytes) -> R<Adt1> {
    let v1: u64 = {
        let a184 = v0.clone();
        length_bytes(a184)
    };
    let v2: u64 = {
        let a185 = v1;
        let a186 = 0u64;
        if nat_eq(a185, a186) {
            1u64
        } else {
            let a189 = {
                let a187 = v1;
                let a188 = 1023u64;
                nat_add(a187, a188)?
            };
            let a190 = 1024u64;
            let a191 = 0u64;
            nat_quot(a189, a190, a191)
        }
    };
    let v3: u64 = {
        let a192 = v2;
        let a193 = 1u64;
        nat_sub(a192, a193)
    };
    let m195 = {
        let (_, h194) = f40(v0.clone(), v1, v3)?;
        h194
    };
    match m195.uncons() {
        None => {
            f41(v0, v1, v3, true)
        }
        Some((v4, v5)) => {
            let (_, h196) = f42(Fn3::F43, 64u64, (List::cons(v4, v5), f41(v0, v1, v3, false)?));
            Ok(h196)
        }
    }
}

pub fn f28(v0: u32) -> Bytes {
    let a209 = {
        let a197 = f44(v0, 4u32);
        let a198 = f44(v0, 0u32);
        append_bytes(a197, a198)
    };
    let a210 = {
        let a207 = {
            let a199 = f44(v0, 12u32);
            let a200 = f44(v0, 8u32);
            append_bytes(a199, a200)
        };
        let a208 = {
            let a205 = {
                let a201 = f44(v0, 20u32);
                let a202 = f44(v0, 16u32);
                append_bytes(a201, a202)
            };
            let a206 = {
                let a203 = f44(v0, 28u32);
                let a204 = f44(v0, 24u32);
                append_bytes(a203, a204)
            };
            append_bytes(a205, a206)
        };
        append_bytes(a207, a208)
    };
    append_bytes(a209, a210)
}

pub fn f29(v0: Bytes, v1: u64) -> R<u64> {
    let m213 = {
        let a211 = v0;
        let a212 = v1;
        index_bytes(a211, a212)
    };
    match m213 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a258 = {
                let a234 = {
                    let a222 = {
                        let a216 = {
                            let a214 = v3;
                            let a215 = 128u8;
                            fixed_u8::bit_and(a214, a215)
                        };
                        let a217 = 128u8;
                        if equal(a216, a217) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a223 = {
                        let a220 = {
                            let a218 = v3;
                            let a219 = 64u8;
                            fixed_u8::bit_and(a218, a219)
                        };
                        let a221 = 64u8;
                        if equal(a220, a221) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a222, a223)?
                };
                let a235 = {
                    let a232 = {
                        let a226 = {
                            let a224 = v3;
                            let a225 = 32u8;
                            fixed_u8::bit_and(a224, a225)
                        };
                        let a227 = 32u8;
                        if equal(a226, a227) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a233 = {
                        let a230 = {
                            let a228 = v3;
                            let a229 = 16u8;
                            fixed_u8::bit_and(a228, a229)
                        };
                        let a231 = 16u8;
                        if equal(a230, a231) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a232, a233)?
                };
                nat_add(a234, a235)?
            };
            let a259 = {
                let a256 = {
                    let a244 = {
                        let a238 = {
                            let a236 = v3;
                            let a237 = 8u8;
                            fixed_u8::bit_and(a236, a237)
                        };
                        let a239 = 8u8;
                        if equal(a238, a239) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a245 = {
                        let a242 = {
                            let a240 = v3;
                            let a241 = 4u8;
                            fixed_u8::bit_and(a240, a241)
                        };
                        let a243 = 4u8;
                        if equal(a242, a243) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a244, a245)?
                };
                let a257 = {
                    let a254 = {
                        let a248 = {
                            let a246 = v3;
                            let a247 = 2u8;
                            fixed_u8::bit_and(a246, a247)
                        };
                        let a249 = 2u8;
                        if equal(a248, a249) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a255 = {
                        let a252 = {
                            let a250 = v3;
                            let a251 = 1u8;
                            fixed_u8::bit_and(a250, a251)
                        };
                        let a253 = 1u8;
                        if equal(a252, a253) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a254, a255)?
                };
                nat_add(a256, a257)?
            };
            nat_add(a258, a259)
        }
    }
}

pub fn f30(v0: u32) -> Bytes {
    let a288 = f45({
        let a260 = f46(v0, 28u32);
        let a261 = 15u32;
        fixed_u32::bit_and(a260, a261)
    });
    let a289 = {
        let a286 = f45({
            let a262 = f46(v0, 24u32);
            let a263 = 15u32;
            fixed_u32::bit_and(a262, a263)
        });
        let a287 = {
            let a284 = f45({
                let a264 = f46(v0, 20u32);
                let a265 = 15u32;
                fixed_u32::bit_and(a264, a265)
            });
            let a285 = {
                let a282 = f45({
                    let a266 = f46(v0, 16u32);
                    let a267 = 15u32;
                    fixed_u32::bit_and(a266, a267)
                });
                let a283 = {
                    let a280 = f45({
                        let a268 = f46(v0, 12u32);
                        let a269 = 15u32;
                        fixed_u32::bit_and(a268, a269)
                    });
                    let a281 = {
                        let a278 = f45({
                            let a270 = f46(v0, 8u32);
                            let a271 = 15u32;
                            fixed_u32::bit_and(a270, a271)
                        });
                        let a279 = {
                            let a276 = f45({
                                let a272 = f46(v0, 4u32);
                                let a273 = 15u32;
                                fixed_u32::bit_and(a272, a273)
                            });
                            let a277 = f45({
                                let a274 = v0;
                                let a275 = 15u32;
                                fixed_u32::bit_and(a274, a275)
                            });
                            append_bytes(a276, a277)
                        };
                        append_bytes(a278, a279)
                    };
                    append_bytes(a280, a281)
                };
                append_bytes(a282, a283)
            };
            append_bytes(a284, a285)
        };
        append_bytes(a286, a287)
    };
    append_bytes(a288, a289)
}

pub fn f31(v0: Fn1, p293: u64, p294: (u64, Adt2)) -> R<(u64, Adt2)> {
    let m292 = (p293, p294);
    {
        let mut m292 = m292;
        loop {
            let step = {
                let (v1, v2) = m292;
                let m290 = v1;
                if m290 == 0 {
                    Err::<(u64, (u64, Adt2)), R<(u64, Adt2)>>(Ok(v2))
                } else {
                    let v3 = m290 - 1;
                    Ok::<(u64, (u64, Adt2)), R<(u64, Adt2)>>((v3, {
                        let c291: Fn1 = v0.clone();
                        c291.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m292 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f32(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt2)) -> R<(u64, Adt2)> {
    Ok(({
        let a296 = {
            let (h295, _) = v3.clone();
            h295
        };
        let a297 = 64u64;
        nat_add(a296, a297)?
    }, f47({
        let (_, h298) = v3;
        h298
    }, f48(v0, v1, v2, {
        let (h299, _) = v3;
        h299
    })?)?))
}

pub fn f33() -> Adt2 {
    Adt2::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f34(v0: Bytes, v1: u64) -> R<Adt3> {
    let v2: u64 = {
        let a300 = v0.clone();
        length_bytes(a300)
    };
    let v3: u64 = {
        let a306 = {
            let a304 = {
                let a301 = v2;
                let a302 = 136u64;
                let a303 = 0u64;
                nat_quot(a301, a302, a303)
            };
            let a305 = 1u64;
            nat_add(a304, a305)?
        };
        let a307 = 136u64;
        nat_mul(a306, a307)?
    };
    let (_, h308) = f49(Fn4::F50(v0, v2, v1, v3), {
        let a309 = v3;
        let a310 = 136u64;
        let a311 = 0u64;
        nat_quot(a309, a310, a311)
    }, (0u64, f51()))?;
    Ok(h308)
}

pub fn f35(v0: u64) -> Bytes {
    let a340 = {
        let a312 = f39(v0, 4u32);
        let a313 = f39(v0, 0u32);
        append_bytes(a312, a313)
    };
    let a341 = {
        let a338 = {
            let a314 = f39(v0, 12u32);
            let a315 = f39(v0, 8u32);
            append_bytes(a314, a315)
        };
        let a339 = {
            let a336 = {
                let a316 = f39(v0, 20u32);
                let a317 = f39(v0, 16u32);
                append_bytes(a316, a317)
            };
            let a337 = {
                let a334 = {
                    let a318 = f39(v0, 28u32);
                    let a319 = f39(v0, 24u32);
                    append_bytes(a318, a319)
                };
                let a335 = {
                    let a332 = {
                        let a320 = f39(v0, 36u32);
                        let a321 = f39(v0, 32u32);
                        append_bytes(a320, a321)
                    };
                    let a333 = {
                        let a330 = {
                            let a322 = f39(v0, 44u32);
                            let a323 = f39(v0, 40u32);
                            append_bytes(a322, a323)
                        };
                        let a331 = {
                            let a328 = {
                                let a324 = f39(v0, 52u32);
                                let a325 = f39(v0, 48u32);
                                append_bytes(a324, a325)
                            };
                            let a329 = {
                                let a326 = f39(v0, 60u32);
                                let a327 = f39(v0, 56u32);
                                append_bytes(a326, a327)
                            };
                            append_bytes(a328, a329)
                        };
                        append_bytes(a330, a331)
                    };
                    append_bytes(a332, a333)
                };
                append_bytes(a334, a335)
            };
            append_bytes(a336, a337)
        };
        append_bytes(a338, a339)
    };
    append_bytes(a340, a341)
}

pub fn f36(v0: Fn2, p345: u64, p346: (u64, Adt0)) -> R<(u64, Adt0)> {
    let m344 = (p345, p346);
    {
        let mut m344 = m344;
        loop {
            let step = {
                let (v1, v2) = m344;
                let m342 = v1;
                if m342 == 0 {
                    Err::<(u64, (u64, Adt0)), R<(u64, Adt0)>>(Ok(v2))
                } else {
                    let v3 = m342 - 1;
                    Ok::<(u64, (u64, Adt0)), R<(u64, Adt0)>>((v3, {
                        let c343: Fn2 = v0.clone();
                        c343.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m344 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f37(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt0)) -> R<(u64, Adt0)> {
    Ok(({
        let a348 = {
            let (h347, _) = v3.clone();
            h347
        };
        let a349 = 128u64;
        nat_add(a348, a349)?
    }, f52({
        let (_, h350) = v3;
        h350
    }, f53(v0, v1, v2, {
        let (h351, _) = v3;
        h351
    })?)?))
}

pub fn f38() -> Adt0 {
    Adt0::C0(7640891576956012808u64, 13503953896175478587u64, 4354685564936845355u64, 11912009170470909681u64, 5840696475078001361u64, 11170449401992604703u64, 2270897969802886507u64, 6620516959819538809u64)
}

pub fn f39(v0: u64, v1: u32) -> Bytes {
    f45(f54(f55(v0, v1)))
}

pub fn f40(v0: Bytes, v1: u64, v2: u64) -> R<(u64, List<Adt1>)> {
    f56(Fn5::F57(v0, v1), v2, (0u64, List::<Adt1>::nil()))
}

pub fn f41(v0: Bytes, v1: u64, v2: u64, v3: bool) -> R<Adt1> {
    let v4: u64 = {
        let a352 = 1024u64;
        let a353 = v2;
        nat_mul(a352, a353)?
    };
    let v5: u64 = f58(v1, v4);
    let v6: u64 = f59(v5)?;
    let v7: u64 = {
        let a354 = v6;
        let a355 = 1u64;
        nat_sub(a354, a355)
    };
    Ok(f60(f61(v0.clone(), v4, v2, v7)?, f62(v0, {
        let a358 = v4;
        let a359 = {
            let a356 = 64u64;
            let a357 = v7;
            nat_mul(a356, a357)?
        };
        nat_add(a358, a359)?
    })?, v2, f63({
        let a362 = v5;
        let a363 = {
            let a360 = 64u64;
            let a361 = v7;
            nat_mul(a360, a361)?
        };
        nat_sub(a362, a363)
    }), {
        let a366 = {
            let a364 = v7;
            let a365 = 0u64;
            if nat_eq(a364, a365) {
                3u32
            } else {
                2u32
            }
        };
        let a367 = if v3 {
            8u32
        } else {
            0u32
        };
        fixed_u32::bit_or(a366, a367)
    }))
}

pub fn f42(v0: Fn3, p371: u64, p372: (List<Adt1>, Adt1)) -> (List<Adt1>, Adt1) {
    let m370 = (p371, p372);
    {
        let mut m370 = m370;
        loop {
            let step = {
                let (v1, v2) = m370;
                let m368 = v1;
                if m368 == 0 {
                    Err::<(u64, (List<Adt1>, Adt1)), (List<Adt1>, Adt1)>(v2)
                } else {
                    let v3 = m368 - 1;
                    Ok::<(u64, (List<Adt1>, Adt1)), (List<Adt1>, Adt1)>((v3, {
                        let c369: Fn3 = v0.clone();
                        c369.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m370 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f43(v0: (List<Adt1>, Adt1)) -> (List<Adt1>, Adt1) {
    f64(v0)
}

pub fn f44(v0: u32, v1: u32) -> Bytes {
    f45(f46(v0, v1))
}

pub fn f45(v0: u32) -> Bytes {
    let a375 = {
        let a373 = v0;
        let a374 = 8u32;
        fixed_u32::bit_and(a373, a374)
    };
    let a376 = 0u32;
    if equal(a375, a376) {
        let a379 = {
            let a377 = v0;
            let a378 = 4u32;
            fixed_u32::bit_and(a377, a378)
        };
        let a380 = 0u32;
        if equal(a379, a380) {
            let a383 = {
                let a381 = v0;
                let a382 = 2u32;
                fixed_u32::bit_and(a381, a382)
            };
            let a384 = 0u32;
            if equal(a383, a384) {
                let a387 = {
                    let a385 = v0;
                    let a386 = 1u32;
                    fixed_u32::bit_and(a385, a386)
                };
                let a388 = 0u32;
                if equal(a387, a388) {
                    Bytes::lit(&[0x30])
                } else {
                    Bytes::lit(&[0x31])
                }
            } else {
                let a391 = {
                    let a389 = v0;
                    let a390 = 1u32;
                    fixed_u32::bit_and(a389, a390)
                };
                let a392 = 0u32;
                if equal(a391, a392) {
                    Bytes::lit(&[0x32])
                } else {
                    Bytes::lit(&[0x33])
                }
            }
        } else {
            let a395 = {
                let a393 = v0;
                let a394 = 2u32;
                fixed_u32::bit_and(a393, a394)
            };
            let a396 = 0u32;
            if equal(a395, a396) {
                let a399 = {
                    let a397 = v0;
                    let a398 = 1u32;
                    fixed_u32::bit_and(a397, a398)
                };
                let a400 = 0u32;
                if equal(a399, a400) {
                    Bytes::lit(&[0x34])
                } else {
                    Bytes::lit(&[0x35])
                }
            } else {
                let a403 = {
                    let a401 = v0;
                    let a402 = 1u32;
                    fixed_u32::bit_and(a401, a402)
                };
                let a404 = 0u32;
                if equal(a403, a404) {
                    Bytes::lit(&[0x36])
                } else {
                    Bytes::lit(&[0x37])
                }
            }
        }
    } else {
        let a407 = {
            let a405 = v0;
            let a406 = 4u32;
            fixed_u32::bit_and(a405, a406)
        };
        let a408 = 0u32;
        if equal(a407, a408) {
            let a411 = {
                let a409 = v0;
                let a410 = 2u32;
                fixed_u32::bit_and(a409, a410)
            };
            let a412 = 0u32;
            if equal(a411, a412) {
                let a415 = {
                    let a413 = v0;
                    let a414 = 1u32;
                    fixed_u32::bit_and(a413, a414)
                };
                let a416 = 0u32;
                if equal(a415, a416) {
                    Bytes::lit(&[0x38])
                } else {
                    Bytes::lit(&[0x39])
                }
            } else {
                let a419 = {
                    let a417 = v0;
                    let a418 = 1u32;
                    fixed_u32::bit_and(a417, a418)
                };
                let a420 = 0u32;
                if equal(a419, a420) {
                    Bytes::lit(&[0x61])
                } else {
                    Bytes::lit(&[0x62])
                }
            }
        } else {
            let a423 = {
                let a421 = v0;
                let a422 = 2u32;
                fixed_u32::bit_and(a421, a422)
            };
            let a424 = 0u32;
            if equal(a423, a424) {
                let a427 = {
                    let a425 = v0;
                    let a426 = 1u32;
                    fixed_u32::bit_and(a425, a426)
                };
                let a428 = 0u32;
                if equal(a427, a428) {
                    Bytes::lit(&[0x63])
                } else {
                    Bytes::lit(&[0x64])
                }
            } else {
                let a431 = {
                    let a429 = v0;
                    let a430 = 1u32;
                    fixed_u32::bit_and(a429, a430)
                };
                let a432 = 0u32;
                if equal(a431, a432) {
                    Bytes::lit(&[0x65])
                } else {
                    Bytes::lit(&[0x66])
                }
            }
        }
    }
}

pub fn f46(v0: u32, v1: u32) -> u32 {
    let m435 = {
        let a433 = v0;
        let a434 = v1;
        fixed_u32::shift_right(a433, a434)
    };
    match m435 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f47(v0: Adt2, v1: Adt4) -> R<Adt2> {
    let v2: Adt5 = f65(Fn6::F66, 64u64, Adt5::C0(match v0.clone() {
        Adt2::C0(h436, _, _, _, _, _, _, _) => {
            h436
        }
    }, match v0.clone() {
        Adt2::C0(_, h437, _, _, _, _, _, _) => {
            h437
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h438, _, _, _, _, _) => {
            h438
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h439, _, _, _, _) => {
            h439
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h440, _, _, _) => {
            h440
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h441, _, _) => {
            h441
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h442, _) => {
            h442
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h443) => {
            h443
        }
    }, v1, 0u64))?;
    Ok(Adt2::C0(f67(match v0.clone() {
        Adt2::C0(h444, _, _, _, _, _, _, _) => {
            h444
        }
    }, match v2.clone() {
        Adt5::C0(h445, _, _, _, _, _, _, _, _, _) => {
            h445
        }
    }), f67(match v0.clone() {
        Adt2::C0(_, h446, _, _, _, _, _, _) => {
            h446
        }
    }, match v2.clone() {
        Adt5::C0(_, h447, _, _, _, _, _, _, _, _) => {
            h447
        }
    }), f67(match v0.clone() {
        Adt2::C0(_, _, h448, _, _, _, _, _) => {
            h448
        }
    }, match v2.clone() {
        Adt5::C0(_, _, h449, _, _, _, _, _, _, _) => {
            h449
        }
    }), f67(match v0.clone() {
        Adt2::C0(_, _, _, h450, _, _, _, _) => {
            h450
        }
    }, match v2.clone() {
        Adt5::C0(_, _, _, h451, _, _, _, _, _, _) => {
            h451
        }
    }), f67(match v0.clone() {
        Adt2::C0(_, _, _, _, h452, _, _, _) => {
            h452
        }
    }, match v2.clone() {
        Adt5::C0(_, _, _, _, h453, _, _, _, _, _) => {
            h453
        }
    }), f67(match v0.clone() {
        Adt2::C0(_, _, _, _, _, h454, _, _) => {
            h454
        }
    }, match v2.clone() {
        Adt5::C0(_, _, _, _, _, h455, _, _, _, _) => {
            h455
        }
    }), f67(match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h456, _) => {
            h456
        }
    }, match v2.clone() {
        Adt5::C0(_, _, _, _, _, _, h457, _, _, _) => {
            h457
        }
    }), f67(match v0 {
        Adt2::C0(_, _, _, _, _, _, _, h458) => {
            h458
        }
    }, match v2 {
        Adt5::C0(_, _, _, _, _, _, _, h459, _, _) => {
            h459
        }
    })))
}

pub fn f48(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt4> {
    Ok(Adt4::C0(f68(v0.clone(), v1, v2, v3)?, f68(v0.clone(), v1, v2, {
        let a460 = v3;
        let a461 = 4u64;
        nat_add(a460, a461)?
    })?, f68(v0.clone(), v1, v2, {
        let a462 = v3;
        let a463 = 8u64;
        nat_add(a462, a463)?
    })?, f68(v0.clone(), v1, v2, {
        let a464 = v3;
        let a465 = 12u64;
        nat_add(a464, a465)?
    })?, f68(v0.clone(), v1, v2, {
        let a466 = v3;
        let a467 = 16u64;
        nat_add(a466, a467)?
    })?, f68(v0.clone(), v1, v2, {
        let a468 = v3;
        let a469 = 20u64;
        nat_add(a468, a469)?
    })?, f68(v0.clone(), v1, v2, {
        let a470 = v3;
        let a471 = 24u64;
        nat_add(a470, a471)?
    })?, f68(v0.clone(), v1, v2, {
        let a472 = v3;
        let a473 = 28u64;
        nat_add(a472, a473)?
    })?, f68(v0.clone(), v1, v2, {
        let a474 = v3;
        let a475 = 32u64;
        nat_add(a474, a475)?
    })?, f68(v0.clone(), v1, v2, {
        let a476 = v3;
        let a477 = 36u64;
        nat_add(a476, a477)?
    })?, f68(v0.clone(), v1, v2, {
        let a478 = v3;
        let a479 = 40u64;
        nat_add(a478, a479)?
    })?, f68(v0.clone(), v1, v2, {
        let a480 = v3;
        let a481 = 44u64;
        nat_add(a480, a481)?
    })?, f68(v0.clone(), v1, v2, {
        let a482 = v3;
        let a483 = 48u64;
        nat_add(a482, a483)?
    })?, f68(v0.clone(), v1, v2, {
        let a484 = v3;
        let a485 = 52u64;
        nat_add(a484, a485)?
    })?, f68(v0.clone(), v1, v2, {
        let a486 = v3;
        let a487 = 56u64;
        nat_add(a486, a487)?
    })?, f68(v0, v1, v2, {
        let a488 = v3;
        let a489 = 60u64;
        nat_add(a488, a489)?
    })?))
}

pub fn f49(v0: Fn4, p493: u64, p494: (u64, Adt3)) -> R<(u64, Adt3)> {
    let m492 = (p493, p494);
    {
        let mut m492 = m492;
        loop {
            let step = {
                let (v1, v2) = m492;
                let m490 = v1;
                if m490 == 0 {
                    Err::<(u64, (u64, Adt3)), R<(u64, Adt3)>>(Ok(v2))
                } else {
                    let v3 = m490 - 1;
                    Ok::<(u64, (u64, Adt3)), R<(u64, Adt3)>>((v3, {
                        let c491: Fn4 = v0.clone();
                        c491.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m492 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f50(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: (u64, Adt3)) -> R<(u64, Adt3)> {
    Ok(({
        let a496 = {
            let (h495, _) = v4.clone();
            h495
        };
        let a497 = 136u64;
        nat_add(a496, a497)?
    }, f69(v0, v1, v3, v2, {
        let (_, h498) = v4;
        h498
    }, {
        let (h499, _) = v4;
        h499
    })?))
}

pub fn f51() -> Adt3 {
    Adt3::C0(0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64)
}

pub fn f52(v0: Adt0, v1: Adt6) -> R<Adt0> {
    let v2: Adt7 = f70(Fn7::F71, 80u64, Adt7::C0(match v0.clone() {
        Adt0::C0(h500, _, _, _, _, _, _, _) => {
            h500
        }
    }, match v0.clone() {
        Adt0::C0(_, h501, _, _, _, _, _, _) => {
            h501
        }
    }, match v0.clone() {
        Adt0::C0(_, _, h502, _, _, _, _, _) => {
            h502
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, h503, _, _, _, _) => {
            h503
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, h504, _, _, _) => {
            h504
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, h505, _, _) => {
            h505
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, h506, _) => {
            h506
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, h507) => {
            h507
        }
    }, v1, 0u64))?;
    Ok(Adt0::C0(f72(match v0.clone() {
        Adt0::C0(h508, _, _, _, _, _, _, _) => {
            h508
        }
    }, match v2.clone() {
        Adt7::C0(h509, _, _, _, _, _, _, _, _, _) => {
            h509
        }
    }), f72(match v0.clone() {
        Adt0::C0(_, h510, _, _, _, _, _, _) => {
            h510
        }
    }, match v2.clone() {
        Adt7::C0(_, h511, _, _, _, _, _, _, _, _) => {
            h511
        }
    }), f72(match v0.clone() {
        Adt0::C0(_, _, h512, _, _, _, _, _) => {
            h512
        }
    }, match v2.clone() {
        Adt7::C0(_, _, h513, _, _, _, _, _, _, _) => {
            h513
        }
    }), f72(match v0.clone() {
        Adt0::C0(_, _, _, h514, _, _, _, _) => {
            h514
        }
    }, match v2.clone() {
        Adt7::C0(_, _, _, h515, _, _, _, _, _, _) => {
            h515
        }
    }), f72(match v0.clone() {
        Adt0::C0(_, _, _, _, h516, _, _, _) => {
            h516
        }
    }, match v2.clone() {
        Adt7::C0(_, _, _, _, h517, _, _, _, _, _) => {
            h517
        }
    }), f72(match v0.clone() {
        Adt0::C0(_, _, _, _, _, h518, _, _) => {
            h518
        }
    }, match v2.clone() {
        Adt7::C0(_, _, _, _, _, h519, _, _, _, _) => {
            h519
        }
    }), f72(match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, h520, _) => {
            h520
        }
    }, match v2.clone() {
        Adt7::C0(_, _, _, _, _, _, h521, _, _, _) => {
            h521
        }
    }), f72(match v0 {
        Adt0::C0(_, _, _, _, _, _, _, h522) => {
            h522
        }
    }, match v2 {
        Adt7::C0(_, _, _, _, _, _, _, h523, _, _) => {
            h523
        }
    })))
}

pub fn f53(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt6> {
    Ok(Adt6::C0(f73(v0.clone(), v1, v2, {
        let a524 = v3;
        let a525 = 0u64;
        nat_add(a524, a525)?
    })?, f73(v0.clone(), v1, v2, {
        let a526 = v3;
        let a527 = 8u64;
        nat_add(a526, a527)?
    })?, f73(v0.clone(), v1, v2, {
        let a528 = v3;
        let a529 = 16u64;
        nat_add(a528, a529)?
    })?, f73(v0.clone(), v1, v2, {
        let a530 = v3;
        let a531 = 24u64;
        nat_add(a530, a531)?
    })?, f73(v0.clone(), v1, v2, {
        let a532 = v3;
        let a533 = 32u64;
        nat_add(a532, a533)?
    })?, f73(v0.clone(), v1, v2, {
        let a534 = v3;
        let a535 = 40u64;
        nat_add(a534, a535)?
    })?, f73(v0.clone(), v1, v2, {
        let a536 = v3;
        let a537 = 48u64;
        nat_add(a536, a537)?
    })?, f73(v0.clone(), v1, v2, {
        let a538 = v3;
        let a539 = 56u64;
        nat_add(a538, a539)?
    })?, f73(v0.clone(), v1, v2, {
        let a540 = v3;
        let a541 = 64u64;
        nat_add(a540, a541)?
    })?, f73(v0.clone(), v1, v2, {
        let a542 = v3;
        let a543 = 72u64;
        nat_add(a542, a543)?
    })?, f73(v0.clone(), v1, v2, {
        let a544 = v3;
        let a545 = 80u64;
        nat_add(a544, a545)?
    })?, f73(v0.clone(), v1, v2, {
        let a546 = v3;
        let a547 = 88u64;
        nat_add(a546, a547)?
    })?, f73(v0.clone(), v1, v2, {
        let a548 = v3;
        let a549 = 96u64;
        nat_add(a548, a549)?
    })?, f73(v0.clone(), v1, v2, {
        let a550 = v3;
        let a551 = 104u64;
        nat_add(a550, a551)?
    })?, f73(v0.clone(), v1, v2, {
        let a552 = v3;
        let a553 = 112u64;
        nat_add(a552, a553)?
    })?, f73(v0, v1, v2, {
        let a554 = v3;
        let a555 = 120u64;
        nat_add(a554, a555)?
    })?))
}

pub fn f54(v0: u64) -> u32 {
    let m559 = {
        let a558 = {
            let a556 = v0;
            let a557 = 4294967295u64;
            fixed_u64::bit_and(a556, a557)
        };
        fixed_u32::convert(i128::from(a558))
    };
    match m559 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f55(v0: u64, v1: u32) -> u64 {
    let m562 = {
        let a560 = v0;
        let a561 = v1;
        fixed_u64::shift_right(a560, a561)
    };
    match m562 {
        None => {
            0u64
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f56(v0: Fn5, p566: u64, p567: (u64, List<Adt1>)) -> R<(u64, List<Adt1>)> {
    let m565 = (p566, p567);
    {
        let mut m565 = m565;
        loop {
            let step = {
                let (v1, v2) = m565;
                let m563 = v1;
                if m563 == 0 {
                    Err::<(u64, (u64, List<Adt1>)), R<(u64, List<Adt1>)>>(Ok(v2))
                } else {
                    let v3 = m563 - 1;
                    Ok::<(u64, (u64, List<Adt1>)), R<(u64, List<Adt1>)>>((v3, {
                        let c564: Fn5 = v0.clone();
                        c564.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m565 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f57(v0: Bytes, v1: u64, v2: (u64, List<Adt1>)) -> R<(u64, List<Adt1>)> {
    f74(v2.clone(), f41(v0, v1, {
        let (h568, _) = v2;
        h568
    }, false)?)
}

pub fn f58(v0: u64, v1: u64) -> u64 {
    let a571 = 1024u64;
    let a572 = {
        let a569 = v0;
        let a570 = v1;
        nat_sub(a569, a570)
    };
    if nat_lt(a571, a572) {
        1024u64
    } else {
        let a573 = v0;
        let a574 = v1;
        nat_sub(a573, a574)
    }
}

pub fn f59(v0: u64) -> R<u64> {
    let a575 = v0;
    let a576 = 0u64;
    if nat_eq(a575, a576) {
        Ok(1u64)
    } else {
        let a579 = {
            let a577 = v0;
            let a578 = 63u64;
            nat_add(a577, a578)?
        };
        let a580 = 64u64;
        let a581 = 0u64;
        Ok(nat_quot(a579, a580, a581))
    }
}

pub fn f60(v0: Adt1, v1: Adt8, v2: u64, v3: u32, v4: u32) -> Adt1 {
    let v5: Adt9 = {
        let (h582, _) = f75(Fn8::F76, 7u64, (Adt9::C0(match v0.clone() {
            Adt1::C0(h583, _, _, _, _, _, _, _) => {
                h583
            }
        }, match v0.clone() {
            Adt1::C0(_, h584, _, _, _, _, _, _) => {
                h584
            }
        }, match v0.clone() {
            Adt1::C0(_, _, h585, _, _, _, _, _) => {
                h585
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, h586, _, _, _, _) => {
                h586
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, h587, _, _, _) => {
                h587
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, h588, _, _) => {
                h588
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h589, _) => {
                h589
            }
        }, match v0 {
            Adt1::C0(_, _, _, _, _, _, _, h590) => {
                h590
            }
        }, 1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, f63({
            let a591 = v2;
            let a592 = 4294967296u64;
            let a593 = 0u64;
            nat_rem(a591, a592, a593)
        }), f63({
            let a594 = v2;
            let a595 = 4294967296u64;
            let a596 = 0u64;
            nat_quot(a594, a595, a596)
        }), v3, v4), v1));
        h582
    };
    Adt1::C0({
        let a599 = match v5.clone() {
            Adt9::C0(h597, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h597
            }
        };
        let a600 = match v5.clone() {
            Adt9::C0(_, _, _, _, _, _, _, _, h598, _, _, _, _, _, _, _) => {
                h598
            }
        };
        fixed_u32::bit_xor(a599, a600)
    }, {
        let a603 = match v5.clone() {
            Adt9::C0(_, h601, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h601
            }
        };
        let a604 = match v5.clone() {
            Adt9::C0(_, _, _, _, _, _, _, _, _, h602, _, _, _, _, _, _) => {
                h602
            }
        };
        fixed_u32::bit_xor(a603, a604)
    }, {
        let a607 = match v5.clone() {
            Adt9::C0(_, _, h605, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h605
            }
        };
        let a608 = match v5.clone() {
            Adt9::C0(_, _, _, _, _, _, _, _, _, _, h606, _, _, _, _, _) => {
                h606
            }
        };
        fixed_u32::bit_xor(a607, a608)
    }, {
        let a611 = match v5.clone() {
            Adt9::C0(_, _, _, h609, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h609
            }
        };
        let a612 = match v5.clone() {
            Adt9::C0(_, _, _, _, _, _, _, _, _, _, _, h610, _, _, _, _) => {
                h610
            }
        };
        fixed_u32::bit_xor(a611, a612)
    }, {
        let a615 = match v5.clone() {
            Adt9::C0(_, _, _, _, h613, _, _, _, _, _, _, _, _, _, _, _) => {
                h613
            }
        };
        let a616 = match v5.clone() {
            Adt9::C0(_, _, _, _, _, _, _, _, _, _, _, _, h614, _, _, _) => {
                h614
            }
        };
        fixed_u32::bit_xor(a615, a616)
    }, {
        let a619 = match v5.clone() {
            Adt9::C0(_, _, _, _, _, h617, _, _, _, _, _, _, _, _, _, _) => {
                h617
            }
        };
        let a620 = match v5.clone() {
            Adt9::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h618, _, _) => {
                h618
            }
        };
        fixed_u32::bit_xor(a619, a620)
    }, {
        let a623 = match v5.clone() {
            Adt9::C0(_, _, _, _, _, _, h621, _, _, _, _, _, _, _, _, _) => {
                h621
            }
        };
        let a624 = match v5.clone() {
            Adt9::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h622, _) => {
                h622
            }
        };
        fixed_u32::bit_xor(a623, a624)
    }, {
        let a627 = match v5.clone() {
            Adt9::C0(_, _, _, _, _, _, _, h625, _, _, _, _, _, _, _, _) => {
                h625
            }
        };
        let a628 = match v5 {
            Adt9::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h626) => {
                h626
            }
        };
        fixed_u32::bit_xor(a627, a628)
    })
}

pub fn f61(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt1> {
    let (_, h629) = f77(Fn9::F78(v2, v0, v1), v3, (0u64, f79()))?;
    Ok(h629)
}

pub fn f62(v0: Bytes, v1: u64) -> R<Adt8> {
    Ok(Adt8::C0(f80(v0.clone(), {
        let a630 = v1;
        let a631 = 0u64;
        nat_add(a630, a631)?
    })?, f80(v0.clone(), {
        let a632 = v1;
        let a633 = 4u64;
        nat_add(a632, a633)?
    })?, f80(v0.clone(), {
        let a634 = v1;
        let a635 = 8u64;
        nat_add(a634, a635)?
    })?, f80(v0.clone(), {
        let a636 = v1;
        let a637 = 12u64;
        nat_add(a636, a637)?
    })?, f80(v0.clone(), {
        let a638 = v1;
        let a639 = 16u64;
        nat_add(a638, a639)?
    })?, f80(v0.clone(), {
        let a640 = v1;
        let a641 = 20u64;
        nat_add(a640, a641)?
    })?, f80(v0.clone(), {
        let a642 = v1;
        let a643 = 24u64;
        nat_add(a642, a643)?
    })?, f80(v0.clone(), {
        let a644 = v1;
        let a645 = 28u64;
        nat_add(a644, a645)?
    })?, f80(v0.clone(), {
        let a646 = v1;
        let a647 = 32u64;
        nat_add(a646, a647)?
    })?, f80(v0.clone(), {
        let a648 = v1;
        let a649 = 36u64;
        nat_add(a648, a649)?
    })?, f80(v0.clone(), {
        let a650 = v1;
        let a651 = 40u64;
        nat_add(a650, a651)?
    })?, f80(v0.clone(), {
        let a652 = v1;
        let a653 = 44u64;
        nat_add(a652, a653)?
    })?, f80(v0.clone(), {
        let a654 = v1;
        let a655 = 48u64;
        nat_add(a654, a655)?
    })?, f80(v0.clone(), {
        let a656 = v1;
        let a657 = 52u64;
        nat_add(a656, a657)?
    })?, f80(v0.clone(), {
        let a658 = v1;
        let a659 = 56u64;
        nat_add(a658, a659)?
    })?, f80(v0, {
        let a660 = v1;
        let a661 = 60u64;
        nat_add(a660, a661)?
    })?))
}

pub fn f63(v0: u64) -> u32 {
    let (_, h662) = {
        let (_, h663) = f81(Fn10::F82, 32u64, (v0, (1u32, 0u32)));
        h663
    };
    h662
}

pub fn f64(v0: (List<Adt1>, Adt1)) -> (List<Adt1>, Adt1) {
    let m665 = {
        let (h664, _) = v0.clone();
        h664
    };
    match m665.uncons() {
        None => {
            v0
        }
        Some((v1, v2)) => {
            (v2.clone(), f83(v1, {
                let (_, h666) = v0;
                h666
            }, {
                let m667 = v2;
                match m667.uncons() {
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

pub fn f65(v0: Fn6, p671: u64, p672: Adt5) -> R<Adt5> {
    let m670 = (p671, p672);
    {
        let mut m670 = m670;
        loop {
            let step = {
                let (v1, v2) = m670;
                let m668 = v1;
                if m668 == 0 {
                    Err::<(u64, Adt5), R<Adt5>>(Ok(v2))
                } else {
                    let v3 = m668 - 1;
                    Ok::<(u64, Adt5), R<Adt5>>((v3, {
                        let c669: Fn6 = v0.clone();
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

pub fn f66(v0: Adt5) -> R<Adt5> {
    f84(v0)
}

pub fn f67(v0: u32, v1: u32) -> u32 {
    let m675 = {
        let a673 = f85(v0);
        let a674 = f85(v1);
        fixed_u64::checked_add(a673, a674)
    };
    match m675 {
        None => {
            0u32
        }
        Some(v2) => {
            f54(v2)
        }
    }
}

pub fn f68(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u32> {
    let a678 = {
        let a676 = v3;
        let a677 = 8u64;
        nat_add(a676, a677)?
    };
    let a679 = v2;
    if nat_eq(a678, a679) {
        Ok(f63({
            let a682 = {
                let a680 = v1;
                let a681 = 8u64;
                nat_mul(a680, a681)?
            };
            let a683 = 4294967296u64;
            let a684 = 0u64;
            nat_quot(a682, a683, a684)
        }))
    } else {
        let a687 = {
            let a685 = v3;
            let a686 = 4u64;
            nat_add(a685, a686)?
        };
        let a688 = v2;
        if nat_eq(a687, a688) {
            Ok(f63({
                let a691 = {
                    let a689 = v1;
                    let a690 = 8u64;
                    nat_mul(a689, a690)?
                };
                let a692 = 4294967296u64;
                let a693 = 0u64;
                nat_rem(a691, a692, a693)
            }))
        } else {
            Ok(f86(f87(v0.clone(), v1, v3), f87(v0.clone(), v1, {
                let a694 = v3;
                let a695 = 1u64;
                nat_add(a694, a695)?
            }), f87(v0.clone(), v1, {
                let a696 = v3;
                let a697 = 2u64;
                nat_add(a696, a697)?
            }), f87(v0, v1, {
                let a698 = v3;
                let a699 = 3u64;
                nat_add(a698, a699)?
            })))
        }
    }
}

pub fn f69(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: Adt3, v5: u64) -> R<Adt3> {
    f88(Adt3::C0({
        let a703 = match v4.clone() {
            Adt3::C0(h700, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h700
            }
        };
        let a704 = f89(v0.clone(), v1, v2, v3, {
            let a701 = v5;
            let a702 = 0u64;
            nat_add(a701, a702)?
        })?;
        fixed_u64::bit_xor(a703, a704)
    }, {
        let a708 = match v4.clone() {
            Adt3::C0(_, h705, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h705
            }
        };
        let a709 = f89(v0.clone(), v1, v2, v3, {
            let a706 = v5;
            let a707 = 8u64;
            nat_add(a706, a707)?
        })?;
        fixed_u64::bit_xor(a708, a709)
    }, {
        let a713 = match v4.clone() {
            Adt3::C0(_, _, h710, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h710
            }
        };
        let a714 = f89(v0.clone(), v1, v2, v3, {
            let a711 = v5;
            let a712 = 16u64;
            nat_add(a711, a712)?
        })?;
        fixed_u64::bit_xor(a713, a714)
    }, {
        let a718 = match v4.clone() {
            Adt3::C0(_, _, _, h715, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h715
            }
        };
        let a719 = f89(v0.clone(), v1, v2, v3, {
            let a716 = v5;
            let a717 = 24u64;
            nat_add(a716, a717)?
        })?;
        fixed_u64::bit_xor(a718, a719)
    }, {
        let a723 = match v4.clone() {
            Adt3::C0(_, _, _, _, h720, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h720
            }
        };
        let a724 = f89(v0.clone(), v1, v2, v3, {
            let a721 = v5;
            let a722 = 32u64;
            nat_add(a721, a722)?
        })?;
        fixed_u64::bit_xor(a723, a724)
    }, {
        let a728 = match v4.clone() {
            Adt3::C0(_, _, _, _, _, h725, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h725
            }
        };
        let a729 = f89(v0.clone(), v1, v2, v3, {
            let a726 = v5;
            let a727 = 40u64;
            nat_add(a726, a727)?
        })?;
        fixed_u64::bit_xor(a728, a729)
    }, {
        let a733 = match v4.clone() {
            Adt3::C0(_, _, _, _, _, _, h730, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h730
            }
        };
        let a734 = f89(v0.clone(), v1, v2, v3, {
            let a731 = v5;
            let a732 = 48u64;
            nat_add(a731, a732)?
        })?;
        fixed_u64::bit_xor(a733, a734)
    }, {
        let a738 = match v4.clone() {
            Adt3::C0(_, _, _, _, _, _, _, h735, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h735
            }
        };
        let a739 = f89(v0.clone(), v1, v2, v3, {
            let a736 = v5;
            let a737 = 56u64;
            nat_add(a736, a737)?
        })?;
        fixed_u64::bit_xor(a738, a739)
    }, {
        let a743 = match v4.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, h740, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h740
            }
        };
        let a744 = f89(v0.clone(), v1, v2, v3, {
            let a741 = v5;
            let a742 = 64u64;
            nat_add(a741, a742)?
        })?;
        fixed_u64::bit_xor(a743, a744)
    }, {
        let a748 = match v4.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, h745, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h745
            }
        };
        let a749 = f89(v0.clone(), v1, v2, v3, {
            let a746 = v5;
            let a747 = 72u64;
            nat_add(a746, a747)?
        })?;
        fixed_u64::bit_xor(a748, a749)
    }, {
        let a753 = match v4.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, h750, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h750
            }
        };
        let a754 = f89(v0.clone(), v1, v2, v3, {
            let a751 = v5;
            let a752 = 80u64;
            nat_add(a751, a752)?
        })?;
        fixed_u64::bit_xor(a753, a754)
    }, {
        let a758 = match v4.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, h755, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h755
            }
        };
        let a759 = f89(v0.clone(), v1, v2, v3, {
            let a756 = v5;
            let a757 = 88u64;
            nat_add(a756, a757)?
        })?;
        fixed_u64::bit_xor(a758, a759)
    }, {
        let a763 = match v4.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, h760, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h760
            }
        };
        let a764 = f89(v0.clone(), v1, v2, v3, {
            let a761 = v5;
            let a762 = 96u64;
            nat_add(a761, a762)?
        })?;
        fixed_u64::bit_xor(a763, a764)
    }, {
        let a768 = match v4.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h765, _, _, _, _, _, _, _, _, _, _, _) => {
                h765
            }
        };
        let a769 = f89(v0.clone(), v1, v2, v3, {
            let a766 = v5;
            let a767 = 104u64;
            nat_add(a766, a767)?
        })?;
        fixed_u64::bit_xor(a768, a769)
    }, {
        let a773 = match v4.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h770, _, _, _, _, _, _, _, _, _, _) => {
                h770
            }
        };
        let a774 = f89(v0.clone(), v1, v2, v3, {
            let a771 = v5;
            let a772 = 112u64;
            nat_add(a771, a772)?
        })?;
        fixed_u64::bit_xor(a773, a774)
    }, {
        let a778 = match v4.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h775, _, _, _, _, _, _, _, _, _) => {
                h775
            }
        };
        let a779 = f89(v0.clone(), v1, v2, v3, {
            let a776 = v5;
            let a777 = 120u64;
            nat_add(a776, a777)?
        })?;
        fixed_u64::bit_xor(a778, a779)
    }, {
        let a783 = match v4.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h780, _, _, _, _, _, _, _, _) => {
                h780
            }
        };
        let a784 = f89(v0, v1, v2, v3, {
            let a781 = v5;
            let a782 = 128u64;
            nat_add(a781, a782)?
        })?;
        fixed_u64::bit_xor(a783, a784)
    }, match v4.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h785, _, _, _, _, _, _, _) => {
            h785
        }
    }, match v4.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h786, _, _, _, _, _, _) => {
            h786
        }
    }, match v4.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h787, _, _, _, _, _) => {
            h787
        }
    }, match v4.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h788, _, _, _, _) => {
            h788
        }
    }, match v4.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h789, _, _, _) => {
            h789
        }
    }, match v4.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h790, _, _) => {
            h790
        }
    }, match v4.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h791, _) => {
            h791
        }
    }, match v4 {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h792) => {
            h792
        }
    }))
}

pub fn f70(v0: Fn7, p796: u64, p797: Adt7) -> R<Adt7> {
    let m795 = (p796, p797);
    {
        let mut m795 = m795;
        loop {
            let step = {
                let (v1, v2) = m795;
                let m793 = v1;
                if m793 == 0 {
                    Err::<(u64, Adt7), R<Adt7>>(Ok(v2))
                } else {
                    let v3 = m793 - 1;
                    Ok::<(u64, Adt7), R<Adt7>>((v3, {
                        let c794: Fn7 = v0.clone();
                        c794.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m795 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f71(v0: Adt7) -> R<Adt7> {
    f90(v0)
}

pub fn f72(v0: u64, v1: u64) -> u64 {
    let v2: u64 = 4294967295u64;
    let v3: u64 = f91({
        let a798 = v0;
        let a799 = v2;
        fixed_u64::bit_and(a798, a799)
    }, {
        let a800 = v1;
        let a801 = v2;
        fixed_u64::bit_and(a800, a801)
    });
    let v4: u64 = f91(f91(f55(v0, 32u32), f55(v1, 32u32)), f55(v3, 32u32));
    let a806 = f92({
        let a802 = v4;
        let a803 = v2;
        fixed_u64::bit_and(a802, a803)
    }, 32u32);
    let a807 = {
        let a804 = v3;
        let a805 = v2;
        fixed_u64::bit_and(a804, a805)
    };
    fixed_u64::bit_or(a806, a807)
}

pub fn f73(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a810 = {
        let a808 = v3;
        let a809 = 16u64;
        nat_add(a808, a809)?
    };
    let a811 = v2;
    if nat_eq(a810, a811) {
        Ok(0u64)
    } else {
        let a814 = {
            let a812 = v3;
            let a813 = 8u64;
            nat_add(a812, a813)?
        };
        let a815 = v2;
        if nat_eq(a814, a815) {
            Ok(f93({
                let a816 = v1;
                let a817 = 8u64;
                nat_mul(a816, a817)?
            }))
        } else {
            Ok(f94(f95(v0.clone(), v1, {
                let a818 = v3;
                let a819 = 0u64;
                nat_add(a818, a819)?
            }), f95(v0.clone(), v1, {
                let a820 = v3;
                let a821 = 1u64;
                nat_add(a820, a821)?
            }), f95(v0.clone(), v1, {
                let a822 = v3;
                let a823 = 2u64;
                nat_add(a822, a823)?
            }), f95(v0.clone(), v1, {
                let a824 = v3;
                let a825 = 3u64;
                nat_add(a824, a825)?
            }), f95(v0.clone(), v1, {
                let a826 = v3;
                let a827 = 4u64;
                nat_add(a826, a827)?
            }), f95(v0.clone(), v1, {
                let a828 = v3;
                let a829 = 5u64;
                nat_add(a828, a829)?
            }), f95(v0.clone(), v1, {
                let a830 = v3;
                let a831 = 6u64;
                nat_add(a830, a831)?
            }), f95(v0, v1, {
                let a832 = v3;
                let a833 = 7u64;
                nat_add(a832, a833)?
            })))
        }
    }
}

pub fn f74(v0: (u64, List<Adt1>), v1: Adt1) -> R<(u64, List<Adt1>)> {
    let v2: Adt10 = f96(Fn11::F97, 40u64, Adt10::C0(v1, {
        let a835 = {
            let (h834, _) = v0.clone();
            h834
        };
        let a836 = 1u64;
        nat_add(a835, a836)?
    }, {
        let (_, h837) = v0;
        h837
    }));
    Ok(({
        let a839 = {
            let (h838, _) = v0;
            h838
        };
        let a840 = 1u64;
        nat_add(a839, a840)?
    }, List::cons(match v2.clone() {
        Adt10::C0(h841, _, _) => {
            h841
        }
    }, match v2 {
        Adt10::C0(_, _, h842) => {
            h842
        }
    })))
}

pub fn f75(v0: Fn8, p846: u64, p847: (Adt9, Adt8)) -> (Adt9, Adt8) {
    let m845 = (p846, p847);
    {
        let mut m845 = m845;
        loop {
            let step = {
                let (v1, v2) = m845;
                let m843 = v1;
                if m843 == 0 {
                    Err::<(u64, (Adt9, Adt8)), (Adt9, Adt8)>(v2)
                } else {
                    let v3 = m843 - 1;
                    Ok::<(u64, (Adt9, Adt8)), (Adt9, Adt8)>((v3, {
                        let c844: Fn8 = v0.clone();
                        c844.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m845 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f76(v0: (Adt9, Adt8)) -> (Adt9, Adt8) {
    f98(v0)
}

pub fn f77(v0: Fn9, p851: u64, p852: (u64, Adt1)) -> R<(u64, Adt1)> {
    let m850 = (p851, p852);
    {
        let mut m850 = m850;
        loop {
            let step = {
                let (v1, v2) = m850;
                let m848 = v1;
                if m848 == 0 {
                    Err::<(u64, (u64, Adt1)), R<(u64, Adt1)>>(Ok(v2))
                } else {
                    let v3 = m848 - 1;
                    Ok::<(u64, (u64, Adt1)), R<(u64, Adt1)>>((v3, {
                        let c849: Fn9 = v0.clone();
                        c849.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m850 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f78(v0: u64, v1: Bytes, v2: u64, v3: (u64, Adt1)) -> R<(u64, Adt1)> {
    Ok(({
        let a854 = {
            let (h853, _) = v3.clone();
            h853
        };
        let a855 = 1u64;
        nat_add(a854, a855)?
    }, f60({
        let (_, h856) = v3.clone();
        h856
    }, f62(v1, {
        let a860 = v2;
        let a861 = {
            let a858 = 64u64;
            let a859 = {
                let (h857, _) = v3.clone();
                h857
            };
            nat_mul(a858, a859)?
        };
        nat_add(a860, a861)?
    })?, v0, 64u32, {
        let a863 = {
            let (h862, _) = v3;
            h862
        };
        let a864 = 0u64;
        if nat_eq(a863, a864) {
            1u32
        } else {
            0u32
        }
    })))
}

pub fn f79() -> Adt1 {
    Adt1::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f80(v0: Bytes, v1: u64) -> R<u32> {
    let a875 = f99(v0.clone(), v1);
    let a876 = {
        let a873 = f100(f99(v0.clone(), {
            let a865 = v1;
            let a866 = 1u64;
            nat_add(a865, a866)?
        }), 8u32);
        let a874 = {
            let a871 = f100(f99(v0.clone(), {
                let a867 = v1;
                let a868 = 2u64;
                nat_add(a867, a868)?
            }), 16u32);
            let a872 = f100(f99(v0, {
                let a869 = v1;
                let a870 = 3u64;
                nat_add(a869, a870)?
            }), 24u32);
            fixed_u32::bit_or(a871, a872)
        };
        fixed_u32::bit_or(a873, a874)
    };
    Ok(fixed_u32::bit_or(a875, a876))
}

pub fn f81(v0: Fn10, p880: u64, p881: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m879 = (p880, p881);
    {
        let mut m879 = m879;
        loop {
            let step = {
                let (v1, v2) = m879;
                let m877 = v1;
                if m877 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m877 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c878: Fn10 = v0.clone();
                        c878.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m879 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f82(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h882, _) = v0;
        h882
    };
    let v2: u32 = {
        let (h883, _) = {
            let (_, h884) = v0;
            h884
        };
        h883
    };
    let v3: u32 = {
        let (_, h885) = {
            let (_, h886) = v0;
            h886
        };
        h885
    };
    ({
        let a887 = v1;
        let a888 = 2u64;
        let a889 = 0u64;
        nat_quot(a887, a888, a889)
    }, (f100(v2, 1u32), {
        let a893 = {
            let a890 = v1;
            let a891 = 2u64;
            let a892 = 0u64;
            nat_rem(a890, a891, a892)
        };
        let a894 = 1u64;
        if nat_eq(a893, a894) {
            let a895 = v3;
            let a896 = v2;
            fixed_u32::bit_or(a895, a896)
        } else {
            v3
        }
    }))
}

pub fn f83(v0: Adt1, v1: Adt1, v2: bool) -> Adt1 {
    f60(f79(), f101(v0, v1), 0u64, 64u32, {
        let a897 = 4u32;
        let a898 = if v2 {
            8u32
        } else {
            0u32
        };
        fixed_u32::bit_or(a897, a898)
    })
}

pub fn f84(v0: Adt5) -> R<Adt5> {
    let v1: u32 = f102(match v0.clone() {
        Adt5::C0(_, _, _, _, _, _, _, _, _, h899) => {
            h899
        }
    });
    let v2: u32 = f67(match v0.clone() {
        Adt5::C0(_, _, _, _, _, _, _, h900, _, _) => {
            h900
        }
    }, f67(f103(match v0.clone() {
        Adt5::C0(_, _, _, _, h901, _, _, _, _, _) => {
            h901
        }
    }), f67(f104(match v0.clone() {
        Adt5::C0(_, _, _, _, h902, _, _, _, _, _) => {
            h902
        }
    }, match v0.clone() {
        Adt5::C0(_, _, _, _, _, h903, _, _, _, _) => {
            h903
        }
    }, match v0.clone() {
        Adt5::C0(_, _, _, _, _, _, h904, _, _, _) => {
            h904
        }
    }), f67(v1, {
        let m907 = match v0.clone() {
            Adt5::C0(_, _, _, _, _, _, _, _, h906, _) => {
                h906
            }
        };
        match m907 {
            Adt4::C0(h905, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h905
            }
        }
    }))));
    let v3: u32 = f67(f105(match v0.clone() {
        Adt5::C0(h908, _, _, _, _, _, _, _, _, _) => {
            h908
        }
    }), f106(match v0.clone() {
        Adt5::C0(h909, _, _, _, _, _, _, _, _, _) => {
            h909
        }
    }, match v0.clone() {
        Adt5::C0(_, h910, _, _, _, _, _, _, _, _) => {
            h910
        }
    }, match v0.clone() {
        Adt5::C0(_, _, h911, _, _, _, _, _, _, _) => {
            h911
        }
    }));
    Ok(Adt5::C0(f67(v2, v3), match v0.clone() {
        Adt5::C0(h912, _, _, _, _, _, _, _, _, _) => {
            h912
        }
    }, match v0.clone() {
        Adt5::C0(_, h913, _, _, _, _, _, _, _, _) => {
            h913
        }
    }, match v0.clone() {
        Adt5::C0(_, _, h914, _, _, _, _, _, _, _) => {
            h914
        }
    }, f67(match v0.clone() {
        Adt5::C0(_, _, _, h915, _, _, _, _, _, _) => {
            h915
        }
    }, v2), match v0.clone() {
        Adt5::C0(_, _, _, _, h916, _, _, _, _, _) => {
            h916
        }
    }, match v0.clone() {
        Adt5::C0(_, _, _, _, _, h917, _, _, _, _) => {
            h917
        }
    }, match v0.clone() {
        Adt5::C0(_, _, _, _, _, _, h918, _, _, _) => {
            h918
        }
    }, f107(match v0.clone() {
        Adt5::C0(_, _, _, _, _, _, _, _, h919, _) => {
            h919
        }
    }), {
        let a921 = match v0 {
            Adt5::C0(_, _, _, _, _, _, _, _, _, h920) => {
                h920
            }
        };
        let a922 = 1u64;
        nat_add(a921, a922)?
    }))
}

pub fn f85(v0: u32) -> u64 {
    let m924 = {
        let a923 = v0;
        fixed_u64::convert(i128::from(a923))
    };
    match m924 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f86(v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
    let a929 = f100(v0, 24u32);
    let a930 = {
        let a927 = f100(v1, 16u32);
        let a928 = {
            let a925 = f100(v2, 8u32);
            let a926 = v3;
            fixed_u32::bit_or(a925, a926)
        };
        fixed_u32::bit_or(a927, a928)
    };
    fixed_u32::bit_or(a929, a930)
}

pub fn f87(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a931 = v2;
    let a932 = v1;
    if nat_lt(a931, a932) {
        let m935 = {
            let a933 = v0;
            let a934 = v2;
            index_bytes(a933, a934)
        };
        match m935 {
            None => {
                0u32
            }
            Some(v3) => {
                let m937 = {
                    let a936 = v3;
                    fixed_u32::convert(i128::from(a936))
                };
                match m937 {
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
        let a938 = v2;
        let a939 = v1;
        if nat_eq(a938, a939) {
            128u32
        } else {
            0u32
        }
    }
}

pub fn f88(v0: Adt3) -> R<Adt3> {
    let (_, h940) = f49(Fn4::F108, 24u64, (0u64, v0))?;
    Ok(h940)
}

pub fn f89(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    Ok(f109(f110(v0.clone(), v1, v2, v3, {
        let a941 = v4;
        let a942 = 0u64;
        nat_add(a941, a942)?
    })?, f110(v0.clone(), v1, v2, v3, {
        let a943 = v4;
        let a944 = 1u64;
        nat_add(a943, a944)?
    })?, f110(v0.clone(), v1, v2, v3, {
        let a945 = v4;
        let a946 = 2u64;
        nat_add(a945, a946)?
    })?, f110(v0.clone(), v1, v2, v3, {
        let a947 = v4;
        let a948 = 3u64;
        nat_add(a947, a948)?
    })?, f110(v0.clone(), v1, v2, v3, {
        let a949 = v4;
        let a950 = 4u64;
        nat_add(a949, a950)?
    })?, f110(v0.clone(), v1, v2, v3, {
        let a951 = v4;
        let a952 = 5u64;
        nat_add(a951, a952)?
    })?, f110(v0.clone(), v1, v2, v3, {
        let a953 = v4;
        let a954 = 6u64;
        nat_add(a953, a954)?
    })?, f110(v0, v1, v2, v3, {
        let a955 = v4;
        let a956 = 7u64;
        nat_add(a955, a956)?
    })?))
}

pub fn f90(v0: Adt7) -> R<Adt7> {
    let v1: u64 = f111(match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, h957) => {
            h957
        }
    });
    let v2: u64 = f72(match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, h958, _, _) => {
            h958
        }
    }, f72(f112(match v0.clone() {
        Adt7::C0(_, _, _, _, h959, _, _, _, _, _) => {
            h959
        }
    }), f72(f113(match v0.clone() {
        Adt7::C0(_, _, _, _, h960, _, _, _, _, _) => {
            h960
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, h961, _, _, _, _) => {
            h961
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, h962, _, _, _) => {
            h962
        }
    }), f72(v1, {
        let m965 = match v0.clone() {
            Adt7::C0(_, _, _, _, _, _, _, _, h964, _) => {
                h964
            }
        };
        match m965 {
            Adt6::C0(h963, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h963
            }
        }
    }))));
    let v3: u64 = f72(f114(match v0.clone() {
        Adt7::C0(h966, _, _, _, _, _, _, _, _, _) => {
            h966
        }
    }), f115(match v0.clone() {
        Adt7::C0(h967, _, _, _, _, _, _, _, _, _) => {
            h967
        }
    }, match v0.clone() {
        Adt7::C0(_, h968, _, _, _, _, _, _, _, _) => {
            h968
        }
    }, match v0.clone() {
        Adt7::C0(_, _, h969, _, _, _, _, _, _, _) => {
            h969
        }
    }));
    Ok(Adt7::C0(f72(v2, v3), match v0.clone() {
        Adt7::C0(h970, _, _, _, _, _, _, _, _, _) => {
            h970
        }
    }, match v0.clone() {
        Adt7::C0(_, h971, _, _, _, _, _, _, _, _) => {
            h971
        }
    }, match v0.clone() {
        Adt7::C0(_, _, h972, _, _, _, _, _, _, _) => {
            h972
        }
    }, f72(match v0.clone() {
        Adt7::C0(_, _, _, h973, _, _, _, _, _, _) => {
            h973
        }
    }, v2), match v0.clone() {
        Adt7::C0(_, _, _, _, h974, _, _, _, _, _) => {
            h974
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, h975, _, _, _, _) => {
            h975
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, h976, _, _, _) => {
            h976
        }
    }, f116(match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, h977, _) => {
            h977
        }
    }), {
        let a979 = match v0 {
            Adt7::C0(_, _, _, _, _, _, _, _, _, h978) => {
                h978
            }
        };
        let a980 = 1u64;
        nat_add(a979, a980)?
    }))
}

pub fn f91(v0: u64, v1: u64) -> u64 {
    let m983 = {
        let a981 = v0;
        let a982 = v1;
        fixed_u64::checked_add(a981, a982)
    };
    match m983 {
        None => {
            0u64
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f92(v0: u64, v1: u32) -> u64 {
    let m986 = {
        let a984 = v0;
        let a985 = v1;
        fixed_u64::shift_left(a984, a985)
    };
    match m986 {
        None => {
            0u64
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f93(v0: u64) -> u64 {
    let (_, h987) = {
        let (_, h988) = f117(Fn12::F118, 64u64, (v0, (1u64, 0u64)));
        h988
    };
    h987
}

pub fn f94(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> u64 {
    let a1001 = f92(v0, 56u32);
    let a1002 = {
        let a999 = f92(v1, 48u32);
        let a1000 = {
            let a997 = f92(v2, 40u32);
            let a998 = {
                let a995 = f92(v3, 32u32);
                let a996 = {
                    let a993 = f92(v4, 24u32);
                    let a994 = {
                        let a991 = f92(v5, 16u32);
                        let a992 = {
                            let a989 = f92(v6, 8u32);
                            let a990 = v7;
                            fixed_u64::bit_or(a989, a990)
                        };
                        fixed_u64::bit_or(a991, a992)
                    };
                    fixed_u64::bit_or(a993, a994)
                };
                fixed_u64::bit_or(a995, a996)
            };
            fixed_u64::bit_or(a997, a998)
        };
        fixed_u64::bit_or(a999, a1000)
    };
    fixed_u64::bit_or(a1001, a1002)
}

pub fn f95(v0: Bytes, v1: u64, v2: u64) -> u64 {
    let a1003 = v2;
    let a1004 = v1;
    if nat_lt(a1003, a1004) {
        f119(v0, v2)
    } else {
        let a1005 = v2;
        let a1006 = v1;
        if nat_eq(a1005, a1006) {
            128u64
        } else {
            0u64
        }
    }
}

pub fn f96(v0: Fn11, p1010: u64, p1011: Adt10) -> Adt10 {
    let m1009 = (p1010, p1011);
    {
        let mut m1009 = m1009;
        loop {
            let step = {
                let (v1, v2) = m1009;
                let m1007 = v1;
                if m1007 == 0 {
                    Err::<(u64, Adt10), Adt10>(v2)
                } else {
                    let v3 = m1007 - 1;
                    Ok::<(u64, Adt10), Adt10>((v3, {
                        let c1008: Fn11 = v0.clone();
                        c1008.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m1009 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f97(v0: Adt10) -> Adt10 {
    f120(v0)
}

pub fn f98(v0: (Adt9, Adt8)) -> (Adt9, Adt8) {
    (f121(f122({
        let (h1012, _) = v0.clone();
        h1012
    }, {
        let (_, h1013) = v0.clone();
        h1013
    }), {
        let (_, h1014) = v0.clone();
        h1014
    }), f123({
        let (_, h1015) = v0;
        h1015
    }))
}

pub fn f99(v0: Bytes, v1: u64) -> u32 {
    let m1018 = {
        let a1016 = v0;
        let a1017 = v1;
        index_bytes(a1016, a1017)
    };
    match m1018 {
        None => {
            0u32
        }
        Some(v2) => {
            let m1020 = {
                let a1019 = v2;
                fixed_u32::convert(i128::from(a1019))
            };
            match m1020 {
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

pub fn f100(v0: u32, v1: u32) -> u32 {
    let m1023 = {
        let a1021 = v0;
        let a1022 = v1;
        fixed_u32::shift_left(a1021, a1022)
    };
    match m1023 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f101(v0: Adt1, v1: Adt1) -> Adt8 {
    Adt8::C0(match v0.clone() {
        Adt1::C0(h1024, _, _, _, _, _, _, _) => {
            h1024
        }
    }, match v0.clone() {
        Adt1::C0(_, h1025, _, _, _, _, _, _) => {
            h1025
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h1026, _, _, _, _, _) => {
            h1026
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h1027, _, _, _, _) => {
            h1027
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h1028, _, _, _) => {
            h1028
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, h1029, _, _) => {
            h1029
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h1030, _) => {
            h1030
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, _, _, _, h1031) => {
            h1031
        }
    }, match v1.clone() {
        Adt1::C0(h1032, _, _, _, _, _, _, _) => {
            h1032
        }
    }, match v1.clone() {
        Adt1::C0(_, h1033, _, _, _, _, _, _) => {
            h1033
        }
    }, match v1.clone() {
        Adt1::C0(_, _, h1034, _, _, _, _, _) => {
            h1034
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, h1035, _, _, _, _) => {
            h1035
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, h1036, _, _, _) => {
            h1036
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, h1037, _, _) => {
            h1037
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, h1038, _) => {
            h1038
        }
    }, match v1 {
        Adt1::C0(_, _, _, _, _, _, _, h1039) => {
            h1039
        }
    })
}

pub fn f102(v0: u64) -> u32 {
    let a1040 = v0;
    let a1041 = 32u64;
    if nat_lt(a1040, a1041) {
        let a1042 = v0;
        let a1043 = 16u64;
        if nat_lt(a1042, a1043) {
            let a1044 = v0;
            let a1045 = 8u64;
            if nat_lt(a1044, a1045) {
                let a1046 = v0;
                let a1047 = 4u64;
                if nat_lt(a1046, a1047) {
                    let a1048 = v0;
                    let a1049 = 2u64;
                    if nat_lt(a1048, a1049) {
                        let a1050 = v0;
                        let a1051 = 1u64;
                        if nat_lt(a1050, a1051) {
                            1116352408u32
                        } else {
                            1899447441u32
                        }
                    } else {
                        let a1052 = v0;
                        let a1053 = 3u64;
                        if nat_lt(a1052, a1053) {
                            3049323471u32
                        } else {
                            3921009573u32
                        }
                    }
                } else {
                    let a1054 = v0;
                    let a1055 = 6u64;
                    if nat_lt(a1054, a1055) {
                        let a1056 = v0;
                        let a1057 = 5u64;
                        if nat_lt(a1056, a1057) {
                            961987163u32
                        } else {
                            1508970993u32
                        }
                    } else {
                        let a1058 = v0;
                        let a1059 = 7u64;
                        if nat_lt(a1058, a1059) {
                            2453635748u32
                        } else {
                            2870763221u32
                        }
                    }
                }
            } else {
                let a1060 = v0;
                let a1061 = 12u64;
                if nat_lt(a1060, a1061) {
                    let a1062 = v0;
                    let a1063 = 10u64;
                    if nat_lt(a1062, a1063) {
                        let a1064 = v0;
                        let a1065 = 9u64;
                        if nat_lt(a1064, a1065) {
                            3624381080u32
                        } else {
                            310598401u32
                        }
                    } else {
                        let a1066 = v0;
                        let a1067 = 11u64;
                        if nat_lt(a1066, a1067) {
                            607225278u32
                        } else {
                            1426881987u32
                        }
                    }
                } else {
                    let a1068 = v0;
                    let a1069 = 14u64;
                    if nat_lt(a1068, a1069) {
                        let a1070 = v0;
                        let a1071 = 13u64;
                        if nat_lt(a1070, a1071) {
                            1925078388u32
                        } else {
                            2162078206u32
                        }
                    } else {
                        let a1072 = v0;
                        let a1073 = 15u64;
                        if nat_lt(a1072, a1073) {
                            2614888103u32
                        } else {
                            3248222580u32
                        }
                    }
                }
            }
        } else {
            let a1074 = v0;
            let a1075 = 24u64;
            if nat_lt(a1074, a1075) {
                let a1076 = v0;
                let a1077 = 20u64;
                if nat_lt(a1076, a1077) {
                    let a1078 = v0;
                    let a1079 = 18u64;
                    if nat_lt(a1078, a1079) {
                        let a1080 = v0;
                        let a1081 = 17u64;
                        if nat_lt(a1080, a1081) {
                            3835390401u32
                        } else {
                            4022224774u32
                        }
                    } else {
                        let a1082 = v0;
                        let a1083 = 19u64;
                        if nat_lt(a1082, a1083) {
                            264347078u32
                        } else {
                            604807628u32
                        }
                    }
                } else {
                    let a1084 = v0;
                    let a1085 = 22u64;
                    if nat_lt(a1084, a1085) {
                        let a1086 = v0;
                        let a1087 = 21u64;
                        if nat_lt(a1086, a1087) {
                            770255983u32
                        } else {
                            1249150122u32
                        }
                    } else {
                        let a1088 = v0;
                        let a1089 = 23u64;
                        if nat_lt(a1088, a1089) {
                            1555081692u32
                        } else {
                            1996064986u32
                        }
                    }
                }
            } else {
                let a1090 = v0;
                let a1091 = 28u64;
                if nat_lt(a1090, a1091) {
                    let a1092 = v0;
                    let a1093 = 26u64;
                    if nat_lt(a1092, a1093) {
                        let a1094 = v0;
                        let a1095 = 25u64;
                        if nat_lt(a1094, a1095) {
                            2554220882u32
                        } else {
                            2821834349u32
                        }
                    } else {
                        let a1096 = v0;
                        let a1097 = 27u64;
                        if nat_lt(a1096, a1097) {
                            2952996808u32
                        } else {
                            3210313671u32
                        }
                    }
                } else {
                    let a1098 = v0;
                    let a1099 = 30u64;
                    if nat_lt(a1098, a1099) {
                        let a1100 = v0;
                        let a1101 = 29u64;
                        if nat_lt(a1100, a1101) {
                            3336571891u32
                        } else {
                            3584528711u32
                        }
                    } else {
                        let a1102 = v0;
                        let a1103 = 31u64;
                        if nat_lt(a1102, a1103) {
                            113926993u32
                        } else {
                            338241895u32
                        }
                    }
                }
            }
        }
    } else {
        let a1104 = v0;
        let a1105 = 48u64;
        if nat_lt(a1104, a1105) {
            let a1106 = v0;
            let a1107 = 40u64;
            if nat_lt(a1106, a1107) {
                let a1108 = v0;
                let a1109 = 36u64;
                if nat_lt(a1108, a1109) {
                    let a1110 = v0;
                    let a1111 = 34u64;
                    if nat_lt(a1110, a1111) {
                        let a1112 = v0;
                        let a1113 = 33u64;
                        if nat_lt(a1112, a1113) {
                            666307205u32
                        } else {
                            773529912u32
                        }
                    } else {
                        let a1114 = v0;
                        let a1115 = 35u64;
                        if nat_lt(a1114, a1115) {
                            1294757372u32
                        } else {
                            1396182291u32
                        }
                    }
                } else {
                    let a1116 = v0;
                    let a1117 = 38u64;
                    if nat_lt(a1116, a1117) {
                        let a1118 = v0;
                        let a1119 = 37u64;
                        if nat_lt(a1118, a1119) {
                            1695183700u32
                        } else {
                            1986661051u32
                        }
                    } else {
                        let a1120 = v0;
                        let a1121 = 39u64;
                        if nat_lt(a1120, a1121) {
                            2177026350u32
                        } else {
                            2456956037u32
                        }
                    }
                }
            } else {
                let a1122 = v0;
                let a1123 = 44u64;
                if nat_lt(a1122, a1123) {
                    let a1124 = v0;
                    let a1125 = 42u64;
                    if nat_lt(a1124, a1125) {
                        let a1126 = v0;
                        let a1127 = 41u64;
                        if nat_lt(a1126, a1127) {
                            2730485921u32
                        } else {
                            2820302411u32
                        }
                    } else {
                        let a1128 = v0;
                        let a1129 = 43u64;
                        if nat_lt(a1128, a1129) {
                            3259730800u32
                        } else {
                            3345764771u32
                        }
                    }
                } else {
                    let a1130 = v0;
                    let a1131 = 46u64;
                    if nat_lt(a1130, a1131) {
                        let a1132 = v0;
                        let a1133 = 45u64;
                        if nat_lt(a1132, a1133) {
                            3516065817u32
                        } else {
                            3600352804u32
                        }
                    } else {
                        let a1134 = v0;
                        let a1135 = 47u64;
                        if nat_lt(a1134, a1135) {
                            4094571909u32
                        } else {
                            275423344u32
                        }
                    }
                }
            }
        } else {
            let a1136 = v0;
            let a1137 = 56u64;
            if nat_lt(a1136, a1137) {
                let a1138 = v0;
                let a1139 = 52u64;
                if nat_lt(a1138, a1139) {
                    let a1140 = v0;
                    let a1141 = 50u64;
                    if nat_lt(a1140, a1141) {
                        let a1142 = v0;
                        let a1143 = 49u64;
                        if nat_lt(a1142, a1143) {
                            430227734u32
                        } else {
                            506948616u32
                        }
                    } else {
                        let a1144 = v0;
                        let a1145 = 51u64;
                        if nat_lt(a1144, a1145) {
                            659060556u32
                        } else {
                            883997877u32
                        }
                    }
                } else {
                    let a1146 = v0;
                    let a1147 = 54u64;
                    if nat_lt(a1146, a1147) {
                        let a1148 = v0;
                        let a1149 = 53u64;
                        if nat_lt(a1148, a1149) {
                            958139571u32
                        } else {
                            1322822218u32
                        }
                    } else {
                        let a1150 = v0;
                        let a1151 = 55u64;
                        if nat_lt(a1150, a1151) {
                            1537002063u32
                        } else {
                            1747873779u32
                        }
                    }
                }
            } else {
                let a1152 = v0;
                let a1153 = 60u64;
                if nat_lt(a1152, a1153) {
                    let a1154 = v0;
                    let a1155 = 58u64;
                    if nat_lt(a1154, a1155) {
                        let a1156 = v0;
                        let a1157 = 57u64;
                        if nat_lt(a1156, a1157) {
                            1955562222u32
                        } else {
                            2024104815u32
                        }
                    } else {
                        let a1158 = v0;
                        let a1159 = 59u64;
                        if nat_lt(a1158, a1159) {
                            2227730452u32
                        } else {
                            2361852424u32
                        }
                    }
                } else {
                    let a1160 = v0;
                    let a1161 = 62u64;
                    if nat_lt(a1160, a1161) {
                        let a1162 = v0;
                        let a1163 = 61u64;
                        if nat_lt(a1162, a1163) {
                            2428436474u32
                        } else {
                            2756734187u32
                        }
                    } else {
                        let a1164 = v0;
                        let a1165 = 63u64;
                        if nat_lt(a1164, a1165) {
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

pub fn f103(v0: u32) -> u32 {
    let a1168 = {
        let a1166 = f124(v0, 6u32, 26u32);
        let a1167 = f124(v0, 11u32, 21u32);
        fixed_u32::bit_xor(a1166, a1167)
    };
    let a1169 = f124(v0, 25u32, 7u32);
    fixed_u32::bit_xor(a1168, a1169)
}

pub fn f104(v0: u32, v1: u32, v2: u32) -> u32 {
    let a1175 = {
        let a1170 = v0;
        let a1171 = v1;
        fixed_u32::bit_and(a1170, a1171)
    };
    let a1176 = {
        let a1173 = {
            let a1172 = v0;
            fixed_u32::bit_not(a1172)
        };
        let a1174 = v2;
        fixed_u32::bit_and(a1173, a1174)
    };
    fixed_u32::bit_xor(a1175, a1176)
}

pub fn f105(v0: u32) -> u32 {
    let a1179 = {
        let a1177 = f124(v0, 2u32, 30u32);
        let a1178 = f124(v0, 13u32, 19u32);
        fixed_u32::bit_xor(a1177, a1178)
    };
    let a1180 = f124(v0, 22u32, 10u32);
    fixed_u32::bit_xor(a1179, a1180)
}

pub fn f106(v0: u32, v1: u32, v2: u32) -> u32 {
    let a1189 = {
        let a1185 = {
            let a1181 = v0;
            let a1182 = v1;
            fixed_u32::bit_and(a1181, a1182)
        };
        let a1186 = {
            let a1183 = v0;
            let a1184 = v2;
            fixed_u32::bit_and(a1183, a1184)
        };
        fixed_u32::bit_xor(a1185, a1186)
    };
    let a1190 = {
        let a1187 = v1;
        let a1188 = v2;
        fixed_u32::bit_and(a1187, a1188)
    };
    fixed_u32::bit_xor(a1189, a1190)
}

pub fn f107(v0: Adt4) -> Adt4 {
    Adt4::C0(match v0.clone() {
        Adt4::C0(_, h1191, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1191
        }
    }, match v0.clone() {
        Adt4::C0(_, _, h1192, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1192
        }
    }, match v0.clone() {
        Adt4::C0(_, _, _, h1193, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1193
        }
    }, match v0.clone() {
        Adt4::C0(_, _, _, _, h1194, _, _, _, _, _, _, _, _, _, _, _) => {
            h1194
        }
    }, match v0.clone() {
        Adt4::C0(_, _, _, _, _, h1195, _, _, _, _, _, _, _, _, _, _) => {
            h1195
        }
    }, match v0.clone() {
        Adt4::C0(_, _, _, _, _, _, h1196, _, _, _, _, _, _, _, _, _) => {
            h1196
        }
    }, match v0.clone() {
        Adt4::C0(_, _, _, _, _, _, _, h1197, _, _, _, _, _, _, _, _) => {
            h1197
        }
    }, match v0.clone() {
        Adt4::C0(_, _, _, _, _, _, _, _, h1198, _, _, _, _, _, _, _) => {
            h1198
        }
    }, match v0.clone() {
        Adt4::C0(_, _, _, _, _, _, _, _, _, h1199, _, _, _, _, _, _) => {
            h1199
        }
    }, match v0.clone() {
        Adt4::C0(_, _, _, _, _, _, _, _, _, _, h1200, _, _, _, _, _) => {
            h1200
        }
    }, match v0.clone() {
        Adt4::C0(_, _, _, _, _, _, _, _, _, _, _, h1201, _, _, _, _) => {
            h1201
        }
    }, match v0.clone() {
        Adt4::C0(_, _, _, _, _, _, _, _, _, _, _, _, h1202, _, _, _) => {
            h1202
        }
    }, match v0.clone() {
        Adt4::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1203, _, _) => {
            h1203
        }
    }, match v0.clone() {
        Adt4::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1204, _) => {
            h1204
        }
    }, match v0.clone() {
        Adt4::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1205) => {
            h1205
        }
    }, f67(f125(match v0.clone() {
        Adt4::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1206, _) => {
            h1206
        }
    }), f67(match v0.clone() {
        Adt4::C0(_, _, _, _, _, _, _, _, _, h1207, _, _, _, _, _, _) => {
            h1207
        }
    }, f67(f126(match v0.clone() {
        Adt4::C0(_, h1208, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1208
        }
    }), match v0 {
        Adt4::C0(h1209, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1209
        }
    }))))
}

pub fn f108(v0: (u64, Adt3)) -> R<(u64, Adt3)> {
    f127(v0)
}

pub fn f109(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> u64 {
    let a1222 = f92(v7, 56u32);
    let a1223 = {
        let a1220 = f92(v6, 48u32);
        let a1221 = {
            let a1218 = f92(v5, 40u32);
            let a1219 = {
                let a1216 = f92(v4, 32u32);
                let a1217 = {
                    let a1214 = f92(v3, 24u32);
                    let a1215 = {
                        let a1212 = f92(v2, 16u32);
                        let a1213 = {
                            let a1210 = f92(v1, 8u32);
                            let a1211 = v0;
                            fixed_u64::bit_or(a1210, a1211)
                        };
                        fixed_u64::bit_or(a1212, a1213)
                    };
                    fixed_u64::bit_or(a1214, a1215)
                };
                fixed_u64::bit_or(a1216, a1217)
            };
            fixed_u64::bit_or(a1218, a1219)
        };
        fixed_u64::bit_or(a1220, a1221)
    };
    fixed_u64::bit_or(a1222, a1223)
}

pub fn f110(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    let v5: u64 = {
        let a1224 = v4;
        let a1225 = v1;
        if nat_lt(a1224, a1225) {
            f119(v0, v4)
        } else {
            let a1226 = v4;
            let a1227 = v1;
            if nat_eq(a1226, a1227) {
                v3
            } else {
                0u64
            }
        }
    };
    let a1230 = {
        let a1228 = v4;
        let a1229 = 1u64;
        nat_add(a1228, a1229)?
    };
    let a1231 = v2;
    if nat_eq(a1230, a1231) {
        let a1232 = v5;
        let a1233 = 128u64;
        Ok(fixed_u64::bit_or(a1232, a1233))
    } else {
        Ok(v5)
    }
}

pub fn f111(v0: u64) -> u64 {
    let a1234 = v0;
    let a1235 = 40u64;
    if nat_lt(a1234, a1235) {
        let a1236 = v0;
        let a1237 = 20u64;
        if nat_lt(a1236, a1237) {
            let a1238 = v0;
            let a1239 = 10u64;
            if nat_lt(a1238, a1239) {
                let a1240 = v0;
                let a1241 = 5u64;
                if nat_lt(a1240, a1241) {
                    let a1242 = v0;
                    let a1243 = 2u64;
                    if nat_lt(a1242, a1243) {
                        let a1244 = v0;
                        let a1245 = 1u64;
                        if nat_lt(a1244, a1245) {
                            4794697086780616226u64
                        } else {
                            8158064640168781261u64
                        }
                    } else {
                        let a1246 = v0;
                        let a1247 = 3u64;
                        if nat_lt(a1246, a1247) {
                            13096744586834688815u64
                        } else {
                            let a1248 = v0;
                            let a1249 = 4u64;
                            if nat_lt(a1248, a1249) {
                                16840607885511220156u64
                            } else {
                                4131703408338449720u64
                            }
                        }
                    }
                } else {
                    let a1250 = v0;
                    let a1251 = 7u64;
                    if nat_lt(a1250, a1251) {
                        let a1252 = v0;
                        let a1253 = 6u64;
                        if nat_lt(a1252, a1253) {
                            6480981068601479193u64
                        } else {
                            10538285296894168987u64
                        }
                    } else {
                        let a1254 = v0;
                        let a1255 = 8u64;
                        if nat_lt(a1254, a1255) {
                            12329834152419229976u64
                        } else {
                            let a1256 = v0;
                            let a1257 = 9u64;
                            if nat_lt(a1256, a1257) {
                                15566598209576043074u64
                            } else {
                                1334009975649890238u64
                            }
                        }
                    }
                }
            } else {
                let a1258 = v0;
                let a1259 = 15u64;
                if nat_lt(a1258, a1259) {
                    let a1260 = v0;
                    let a1261 = 12u64;
                    if nat_lt(a1260, a1261) {
                        let a1262 = v0;
                        let a1263 = 11u64;
                        if nat_lt(a1262, a1263) {
                            2608012711638119052u64
                        } else {
                            6128411473006802146u64
                        }
                    } else {
                        let a1264 = v0;
                        let a1265 = 13u64;
                        if nat_lt(a1264, a1265) {
                            8268148722764581231u64
                        } else {
                            let a1266 = v0;
                            let a1267 = 14u64;
                            if nat_lt(a1266, a1267) {
                                9286055187155687089u64
                            } else {
                                11230858885718282805u64
                            }
                        }
                    }
                } else {
                    let a1268 = v0;
                    let a1269 = 17u64;
                    if nat_lt(a1268, a1269) {
                        let a1270 = v0;
                        let a1271 = 16u64;
                        if nat_lt(a1270, a1271) {
                            13951009754708518548u64
                        } else {
                            16472876342353939154u64
                        }
                    } else {
                        let a1272 = v0;
                        let a1273 = 18u64;
                        if nat_lt(a1272, a1273) {
                            17275323862435702243u64
                        } else {
                            let a1274 = v0;
                            let a1275 = 19u64;
                            if nat_lt(a1274, a1275) {
                                1135362057144423861u64
                            } else {
                                2597628984639134821u64
                            }
                        }
                    }
                }
            }
        } else {
            let a1276 = v0;
            let a1277 = 30u64;
            if nat_lt(a1276, a1277) {
                let a1278 = v0;
                let a1279 = 25u64;
                if nat_lt(a1278, a1279) {
                    let a1280 = v0;
                    let a1281 = 22u64;
                    if nat_lt(a1280, a1281) {
                        let a1282 = v0;
                        let a1283 = 21u64;
                        if nat_lt(a1282, a1283) {
                            3308224258029322869u64
                        } else {
                            5365058923640841347u64
                        }
                    } else {
                        let a1284 = v0;
                        let a1285 = 23u64;
                        if nat_lt(a1284, a1285) {
                            6679025012923562964u64
                        } else {
                            let a1286 = v0;
                            let a1287 = 24u64;
                            if nat_lt(a1286, a1287) {
                                8573033837759648693u64
                            } else {
                                10970295158949994411u64
                            }
                        }
                    }
                } else {
                    let a1288 = v0;
                    let a1289 = 27u64;
                    if nat_lt(a1288, a1289) {
                        let a1290 = v0;
                        let a1291 = 26u64;
                        if nat_lt(a1290, a1291) {
                            12119686244451234320u64
                        } else {
                            12683024718118986047u64
                        }
                    } else {
                        let a1292 = v0;
                        let a1293 = 28u64;
                        if nat_lt(a1292, a1293) {
                            13788192230050041572u64
                        } else {
                            let a1294 = v0;
                            let a1295 = 29u64;
                            if nat_lt(a1294, a1295) {
                                14330467153632333762u64
                            } else {
                                15395433587784984357u64
                            }
                        }
                    }
                }
            } else {
                let a1296 = v0;
                let a1297 = 35u64;
                if nat_lt(a1296, a1297) {
                    let a1298 = v0;
                    let a1299 = 32u64;
                    if nat_lt(a1298, a1299) {
                        let a1300 = v0;
                        let a1301 = 31u64;
                        if nat_lt(a1300, a1301) {
                            489312712824947311u64
                        } else {
                            1452737877330783856u64
                        }
                    } else {
                        let a1302 = v0;
                        let a1303 = 33u64;
                        if nat_lt(a1302, a1303) {
                            2861767655752347644u64
                        } else {
                            let a1304 = v0;
                            let a1305 = 34u64;
                            if nat_lt(a1304, a1305) {
                                3322285676063803686u64
                            } else {
                                5560940570517711597u64
                            }
                        }
                    }
                } else {
                    let a1306 = v0;
                    let a1307 = 37u64;
                    if nat_lt(a1306, a1307) {
                        let a1308 = v0;
                        let a1309 = 36u64;
                        if nat_lt(a1308, a1309) {
                            5996557281743188959u64
                        } else {
                            7280758554555802590u64
                        }
                    } else {
                        let a1310 = v0;
                        let a1311 = 38u64;
                        if nat_lt(a1310, a1311) {
                            8532644243296465576u64
                        } else {
                            let a1312 = v0;
                            let a1313 = 39u64;
                            if nat_lt(a1312, a1313) {
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
        let a1314 = v0;
        let a1315 = 60u64;
        if nat_lt(a1314, a1315) {
            let a1316 = v0;
            let a1317 = 50u64;
            if nat_lt(a1316, a1317) {
                let a1318 = v0;
                let a1319 = 45u64;
                if nat_lt(a1318, a1319) {
                    let a1320 = v0;
                    let a1321 = 42u64;
                    if nat_lt(a1320, a1321) {
                        let a1322 = v0;
                        let a1323 = 41u64;
                        if nat_lt(a1322, a1323) {
                            11727347734174303076u64
                        } else {
                            12113106623233404929u64
                        }
                    } else {
                        let a1324 = v0;
                        let a1325 = 43u64;
                        if nat_lt(a1324, a1325) {
                            14000437183269869457u64
                        } else {
                            let a1326 = v0;
                            let a1327 = 44u64;
                            if nat_lt(a1326, a1327) {
                                14369950271660146224u64
                            } else {
                                15101387698204529176u64
                            }
                        }
                    }
                } else {
                    let a1328 = v0;
                    let a1329 = 47u64;
                    if nat_lt(a1328, a1329) {
                        let a1330 = v0;
                        let a1331 = 46u64;
                        if nat_lt(a1330, a1331) {
                            15463397548674623760u64
                        } else {
                            17586052441742319658u64
                        }
                    } else {
                        let a1332 = v0;
                        let a1333 = 48u64;
                        if nat_lt(a1332, a1333) {
                            1182934255886127544u64
                        } else {
                            let a1334 = v0;
                            let a1335 = 49u64;
                            if nat_lt(a1334, a1335) {
                                1847814050463011016u64
                            } else {
                                2177327727835720531u64
                            }
                        }
                    }
                }
            } else {
                let a1336 = v0;
                let a1337 = 55u64;
                if nat_lt(a1336, a1337) {
                    let a1338 = v0;
                    let a1339 = 52u64;
                    if nat_lt(a1338, a1339) {
                        let a1340 = v0;
                        let a1341 = 51u64;
                        if nat_lt(a1340, a1341) {
                            2830643537854262169u64
                        } else {
                            3796741975233480872u64
                        }
                    } else {
                        let a1342 = v0;
                        let a1343 = 53u64;
                        if nat_lt(a1342, a1343) {
                            4115178125766777443u64
                        } else {
                            let a1344 = v0;
                            let a1345 = 54u64;
                            if nat_lt(a1344, a1345) {
                                5681478168544905931u64
                            } else {
                                6601373596472566643u64
                            }
                        }
                    }
                } else {
                    let a1346 = v0;
                    let a1347 = 57u64;
                    if nat_lt(a1346, a1347) {
                        let a1348 = v0;
                        let a1349 = 56u64;
                        if nat_lt(a1348, a1349) {
                            7507060721942968483u64
                        } else {
                            8399075790359081724u64
                        }
                    } else {
                        let a1350 = v0;
                        let a1351 = 58u64;
                        if nat_lt(a1350, a1351) {
                            8693463985226723168u64
                        } else {
                            let a1352 = v0;
                            let a1353 = 59u64;
                            if nat_lt(a1352, a1353) {
                                9568029438360202098u64
                            } else {
                                10144078919501101548u64
                            }
                        }
                    }
                }
            }
        } else {
            let a1354 = v0;
            let a1355 = 70u64;
            if nat_lt(a1354, a1355) {
                let a1356 = v0;
                let a1357 = 65u64;
                if nat_lt(a1356, a1357) {
                    let a1358 = v0;
                    let a1359 = 62u64;
                    if nat_lt(a1358, a1359) {
                        let a1360 = v0;
                        let a1361 = 61u64;
                        if nat_lt(a1360, a1361) {
                            10430055236837252648u64
                        } else {
                            11840083180663258601u64
                        }
                    } else {
                        let a1362 = v0;
                        let a1363 = 63u64;
                        if nat_lt(a1362, a1363) {
                            13761210420658862357u64
                        } else {
                            let a1364 = v0;
                            let a1365 = 64u64;
                            if nat_lt(a1364, a1365) {
                                14299343276471374635u64
                            } else {
                                14566680578165727644u64
                            }
                        }
                    }
                } else {
                    let a1366 = v0;
                    let a1367 = 67u64;
                    if nat_lt(a1366, a1367) {
                        let a1368 = v0;
                        let a1369 = 66u64;
                        if nat_lt(a1368, a1369) {
                            15097957966210449927u64
                        } else {
                            16922976911328602910u64
                        }
                    } else {
                        let a1370 = v0;
                        let a1371 = 68u64;
                        if nat_lt(a1370, a1371) {
                            17689382322260857208u64
                        } else {
                            let a1372 = v0;
                            let a1373 = 69u64;
                            if nat_lt(a1372, a1373) {
                                500013540394364858u64
                            } else {
                                748580250866718886u64
                            }
                        }
                    }
                }
            } else {
                let a1374 = v0;
                let a1375 = 75u64;
                if nat_lt(a1374, a1375) {
                    let a1376 = v0;
                    let a1377 = 72u64;
                    if nat_lt(a1376, a1377) {
                        let a1378 = v0;
                        let a1379 = 71u64;
                        if nat_lt(a1378, a1379) {
                            1242879168328830382u64
                        } else {
                            1977374033974150939u64
                        }
                    } else {
                        let a1380 = v0;
                        let a1381 = 73u64;
                        if nat_lt(a1380, a1381) {
                            2944078676154940804u64
                        } else {
                            let a1382 = v0;
                            let a1383 = 74u64;
                            if nat_lt(a1382, a1383) {
                                3659926193048069267u64
                            } else {
                                4368137639120453308u64
                            }
                        }
                    }
                } else {
                    let a1384 = v0;
                    let a1385 = 77u64;
                    if nat_lt(a1384, a1385) {
                        let a1386 = v0;
                        let a1387 = 76u64;
                        if nat_lt(a1386, a1387) {
                            4836135668995329356u64
                        } else {
                            5532061633213252278u64
                        }
                    } else {
                        let a1388 = v0;
                        let a1389 = 78u64;
                        if nat_lt(a1388, a1389) {
                            6448918945643986474u64
                        } else {
                            let a1390 = v0;
                            let a1391 = 79u64;
                            if nat_lt(a1390, a1391) {
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

pub fn f112(v0: u64) -> u64 {
    let a1394 = {
        let a1392 = f128(v0, 14u32, 50u32);
        let a1393 = f128(v0, 18u32, 46u32);
        fixed_u64::bit_xor(a1392, a1393)
    };
    let a1395 = f128(v0, 41u32, 23u32);
    fixed_u64::bit_xor(a1394, a1395)
}

pub fn f113(v0: u64, v1: u64, v2: u64) -> u64 {
    let a1401 = {
        let a1396 = v0;
        let a1397 = v1;
        fixed_u64::bit_and(a1396, a1397)
    };
    let a1402 = {
        let a1399 = {
            let a1398 = v0;
            fixed_u64::bit_not(a1398)
        };
        let a1400 = v2;
        fixed_u64::bit_and(a1399, a1400)
    };
    fixed_u64::bit_xor(a1401, a1402)
}

pub fn f114(v0: u64) -> u64 {
    let a1405 = {
        let a1403 = f128(v0, 28u32, 36u32);
        let a1404 = f128(v0, 34u32, 30u32);
        fixed_u64::bit_xor(a1403, a1404)
    };
    let a1406 = f128(v0, 39u32, 25u32);
    fixed_u64::bit_xor(a1405, a1406)
}

pub fn f115(v0: u64, v1: u64, v2: u64) -> u64 {
    let a1415 = {
        let a1411 = {
            let a1407 = v0;
            let a1408 = v1;
            fixed_u64::bit_and(a1407, a1408)
        };
        let a1412 = {
            let a1409 = v0;
            let a1410 = v2;
            fixed_u64::bit_and(a1409, a1410)
        };
        fixed_u64::bit_xor(a1411, a1412)
    };
    let a1416 = {
        let a1413 = v1;
        let a1414 = v2;
        fixed_u64::bit_and(a1413, a1414)
    };
    fixed_u64::bit_xor(a1415, a1416)
}

pub fn f116(v0: Adt6) -> Adt6 {
    Adt6::C0(match v0.clone() {
        Adt6::C0(_, h1417, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1417
        }
    }, match v0.clone() {
        Adt6::C0(_, _, h1418, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1418
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, h1419, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1419
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, h1420, _, _, _, _, _, _, _, _, _, _, _) => {
            h1420
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, h1421, _, _, _, _, _, _, _, _, _, _) => {
            h1421
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, h1422, _, _, _, _, _, _, _, _, _) => {
            h1422
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, h1423, _, _, _, _, _, _, _, _) => {
            h1423
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, h1424, _, _, _, _, _, _, _) => {
            h1424
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, h1425, _, _, _, _, _, _) => {
            h1425
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, _, h1426, _, _, _, _, _) => {
            h1426
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, h1427, _, _, _, _) => {
            h1427
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, h1428, _, _, _) => {
            h1428
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1429, _, _) => {
            h1429
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1430, _) => {
            h1430
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1431) => {
            h1431
        }
    }, f72(f129(match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1432, _) => {
            h1432
        }
    }), f72(match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, _, _, h1433, _, _, _, _, _, _) => {
            h1433
        }
    }, f72(f130(match v0.clone() {
        Adt6::C0(_, h1434, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1434
        }
    }), match v0 {
        Adt6::C0(h1435, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1435
        }
    }))))
}

pub fn f117(v0: Fn12, p1439: u64, p1440: (u64, (u64, u64))) -> (u64, (u64, u64)) {
    let m1438 = (p1439, p1440);
    {
        let mut m1438 = m1438;
        loop {
            let step = {
                let (v1, v2) = m1438;
                let m1436 = v1;
                if m1436 == 0 {
                    Err::<(u64, (u64, (u64, u64))), (u64, (u64, u64))>(v2)
                } else {
                    let v3 = m1436 - 1;
                    Ok::<(u64, (u64, (u64, u64))), (u64, (u64, u64))>((v3, {
                        let c1437: Fn12 = v0.clone();
                        c1437.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m1438 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f118(v0: (u64, (u64, u64))) -> (u64, (u64, u64)) {
    let v1: u64 = {
        let (h1441, _) = v0;
        h1441
    };
    let v2: u64 = {
        let (h1442, _) = {
            let (_, h1443) = v0;
            h1443
        };
        h1442
    };
    let v3: u64 = {
        let (_, h1444) = {
            let (_, h1445) = v0;
            h1445
        };
        h1444
    };
    ({
        let a1446 = v1;
        let a1447 = 2u64;
        let a1448 = 0u64;
        nat_quot(a1446, a1447, a1448)
    }, (f92(v2, 1u32), {
        let a1452 = {
            let a1449 = v1;
            let a1450 = 2u64;
            let a1451 = 0u64;
            nat_rem(a1449, a1450, a1451)
        };
        let a1453 = 1u64;
        if nat_eq(a1452, a1453) {
            let a1454 = v3;
            let a1455 = v2;
            fixed_u64::bit_or(a1454, a1455)
        } else {
            v3
        }
    }))
}

pub fn f119(v0: Bytes, v1: u64) -> u64 {
    let m1458 = {
        let a1456 = v0;
        let a1457 = v1;
        index_bytes(a1456, a1457)
    };
    match m1458 {
        None => {
            0u64
        }
        Some(v2) => {
            let m1460 = {
                let a1459 = v2;
                fixed_u64::convert(i128::from(a1459))
            };
            match m1460 {
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

pub fn f120(v0: Adt10) -> Adt10 {
    let a1465 = {
        let a1462 = match v0.clone() {
            Adt10::C0(_, h1461, _) => {
                h1461
            }
        };
        let a1463 = 2u64;
        let a1464 = 0u64;
        nat_rem(a1462, a1463, a1464)
    };
    let a1466 = 0u64;
    if nat_eq(a1465, a1466) {
        let m1468 = match v0.clone() {
            Adt10::C0(_, _, h1467) => {
                h1467
            }
        };
        match m1468.uncons() {
            None => {
                v0
            }
            Some((v1, v2)) => {
                Adt10::C0(f83(v1, match v0.clone() {
                    Adt10::C0(h1469, _, _) => {
                        h1469
                    }
                }, false), {
                    let a1471 = match v0 {
                        Adt10::C0(_, h1470, _) => {
                            h1470
                        }
                    };
                    let a1472 = 2u64;
                    let a1473 = 0u64;
                    nat_quot(a1471, a1472, a1473)
                }, v2)
            }
        }
    } else {
        v0
    }
}

pub fn f121(v0: Adt9, v1: Adt8) -> Adt9 {
    let v2: Adt11 = f131(match v0.clone() {
        Adt9::C0(h1474, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1474
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, h1475, _, _, _, _, _, _, _, _, _, _) => {
            h1475
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, _, _, h1476, _, _, _, _, _) => {
            h1476
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1477) => {
            h1477
        }
    }, match v1.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, h1478, _, _, _, _, _, _, _) => {
            h1478
        }
    }, match v1.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, h1479, _, _, _, _, _, _) => {
            h1479
        }
    });
    let v3: Adt11 = f131(match v0.clone() {
        Adt9::C0(_, h1480, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1480
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, h1481, _, _, _, _, _, _, _, _, _) => {
            h1481
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, _, _, _, h1482, _, _, _, _) => {
            h1482
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, _, _, _, _, h1483, _, _, _) => {
            h1483
        }
    }, match v1.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, h1484, _, _, _, _, _) => {
            h1484
        }
    }, match v1.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, h1485, _, _, _, _) => {
            h1485
        }
    });
    let v4: Adt11 = f131(match v0.clone() {
        Adt9::C0(_, _, h1486, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1486
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, h1487, _, _, _, _, _, _, _, _) => {
            h1487
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, h1488, _, _, _, _, _, _, _) => {
            h1488
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1489, _, _) => {
            h1489
        }
    }, match v1.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, h1490, _, _, _) => {
            h1490
        }
    }, match v1.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1491, _, _) => {
            h1491
        }
    });
    let v5: Adt11 = f131(match v0.clone() {
        Adt9::C0(_, _, _, h1492, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1492
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, h1493, _, _, _, _, _, _, _, _, _, _, _) => {
            h1493
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, _, h1494, _, _, _, _, _, _) => {
            h1494
        }
    }, match v0 {
        Adt9::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1495, _) => {
            h1495
        }
    }, match v1.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1496, _) => {
            h1496
        }
    }, match v1 {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1497) => {
            h1497
        }
    });
    Adt9::C0(match v2.clone() {
        Adt11::C0(h1498, _, _, _) => {
            h1498
        }
    }, match v3.clone() {
        Adt11::C0(h1499, _, _, _) => {
            h1499
        }
    }, match v4.clone() {
        Adt11::C0(h1500, _, _, _) => {
            h1500
        }
    }, match v5.clone() {
        Adt11::C0(h1501, _, _, _) => {
            h1501
        }
    }, match v5.clone() {
        Adt11::C0(_, h1502, _, _) => {
            h1502
        }
    }, match v2.clone() {
        Adt11::C0(_, h1503, _, _) => {
            h1503
        }
    }, match v3.clone() {
        Adt11::C0(_, h1504, _, _) => {
            h1504
        }
    }, match v4.clone() {
        Adt11::C0(_, h1505, _, _) => {
            h1505
        }
    }, match v4.clone() {
        Adt11::C0(_, _, h1506, _) => {
            h1506
        }
    }, match v5.clone() {
        Adt11::C0(_, _, h1507, _) => {
            h1507
        }
    }, match v2.clone() {
        Adt11::C0(_, _, h1508, _) => {
            h1508
        }
    }, match v3.clone() {
        Adt11::C0(_, _, h1509, _) => {
            h1509
        }
    }, match v3 {
        Adt11::C0(_, _, _, h1510) => {
            h1510
        }
    }, match v4 {
        Adt11::C0(_, _, _, h1511) => {
            h1511
        }
    }, match v5 {
        Adt11::C0(_, _, _, h1512) => {
            h1512
        }
    }, match v2 {
        Adt11::C0(_, _, _, h1513) => {
            h1513
        }
    })
}

pub fn f122(v0: Adt9, v1: Adt8) -> Adt9 {
    let v2: Adt11 = f131(match v0.clone() {
        Adt9::C0(h1514, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1514
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, h1515, _, _, _, _, _, _, _, _, _, _, _) => {
            h1515
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, h1516, _, _, _, _, _, _, _) => {
            h1516
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, _, _, _, _, h1517, _, _, _) => {
            h1517
        }
    }, match v1.clone() {
        Adt8::C0(h1518, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1518
        }
    }, match v1.clone() {
        Adt8::C0(_, h1519, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1519
        }
    });
    let v3: Adt11 = f131(match v0.clone() {
        Adt9::C0(_, h1520, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1520
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, h1521, _, _, _, _, _, _, _, _, _, _) => {
            h1521
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, _, h1522, _, _, _, _, _, _) => {
            h1522
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1523, _, _) => {
            h1523
        }
    }, match v1.clone() {
        Adt8::C0(_, _, h1524, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1524
        }
    }, match v1.clone() {
        Adt8::C0(_, _, _, h1525, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1525
        }
    });
    let v4: Adt11 = f131(match v0.clone() {
        Adt9::C0(_, _, h1526, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1526
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, h1527, _, _, _, _, _, _, _, _, _) => {
            h1527
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, _, _, h1528, _, _, _, _, _) => {
            h1528
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1529, _) => {
            h1529
        }
    }, match v1.clone() {
        Adt8::C0(_, _, _, _, h1530, _, _, _, _, _, _, _, _, _, _, _) => {
            h1530
        }
    }, match v1.clone() {
        Adt8::C0(_, _, _, _, _, h1531, _, _, _, _, _, _, _, _, _, _) => {
            h1531
        }
    });
    let v5: Adt11 = f131(match v0.clone() {
        Adt9::C0(_, _, _, h1532, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1532
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, h1533, _, _, _, _, _, _, _, _) => {
            h1533
        }
    }, match v0.clone() {
        Adt9::C0(_, _, _, _, _, _, _, _, _, _, _, h1534, _, _, _, _) => {
            h1534
        }
    }, match v0 {
        Adt9::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1535) => {
            h1535
        }
    }, match v1.clone() {
        Adt8::C0(_, _, _, _, _, _, h1536, _, _, _, _, _, _, _, _, _) => {
            h1536
        }
    }, match v1 {
        Adt8::C0(_, _, _, _, _, _, _, h1537, _, _, _, _, _, _, _, _) => {
            h1537
        }
    });
    Adt9::C0(match v2.clone() {
        Adt11::C0(h1538, _, _, _) => {
            h1538
        }
    }, match v3.clone() {
        Adt11::C0(h1539, _, _, _) => {
            h1539
        }
    }, match v4.clone() {
        Adt11::C0(h1540, _, _, _) => {
            h1540
        }
    }, match v5.clone() {
        Adt11::C0(h1541, _, _, _) => {
            h1541
        }
    }, match v2.clone() {
        Adt11::C0(_, h1542, _, _) => {
            h1542
        }
    }, match v3.clone() {
        Adt11::C0(_, h1543, _, _) => {
            h1543
        }
    }, match v4.clone() {
        Adt11::C0(_, h1544, _, _) => {
            h1544
        }
    }, match v5.clone() {
        Adt11::C0(_, h1545, _, _) => {
            h1545
        }
    }, match v2.clone() {
        Adt11::C0(_, _, h1546, _) => {
            h1546
        }
    }, match v3.clone() {
        Adt11::C0(_, _, h1547, _) => {
            h1547
        }
    }, match v4.clone() {
        Adt11::C0(_, _, h1548, _) => {
            h1548
        }
    }, match v5.clone() {
        Adt11::C0(_, _, h1549, _) => {
            h1549
        }
    }, match v2 {
        Adt11::C0(_, _, _, h1550) => {
            h1550
        }
    }, match v3 {
        Adt11::C0(_, _, _, h1551) => {
            h1551
        }
    }, match v4 {
        Adt11::C0(_, _, _, h1552) => {
            h1552
        }
    }, match v5 {
        Adt11::C0(_, _, _, h1553) => {
            h1553
        }
    })
}

pub fn f123(v0: Adt8) -> Adt8 {
    Adt8::C0(match v0.clone() {
        Adt8::C0(_, _, h1554, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1554
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, h1555, _, _, _, _, _, _, _, _, _) => {
            h1555
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, h1556, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1556
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, h1557, _, _, _, _, _) => {
            h1557
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, h1558, _, _, _, _, _, _, _, _) => {
            h1558
        }
    }, match v0.clone() {
        Adt8::C0(h1559, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1559
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, h1560, _, _, _, _, _, _, _, _, _, _, _) => {
            h1560
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1561, _, _) => {
            h1561
        }
    }, match v0.clone() {
        Adt8::C0(_, h1562, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1562
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, h1563, _, _, _, _) => {
            h1563
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, h1564, _, _, _) => {
            h1564
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, _, h1565, _, _, _, _, _, _, _, _, _, _) => {
            h1565
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, h1566, _, _, _, _, _, _) => {
            h1566
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1567, _) => {
            h1567
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1568) => {
            h1568
        }
    }, match v0 {
        Adt8::C0(_, _, _, _, _, _, _, _, h1569, _, _, _, _, _, _, _) => {
            h1569
        }
    })
}

pub fn f124(v0: u32, v1: u32, v2: u32) -> u32 {
    let a1570 = f46(v0, v1);
    let a1571 = f100(v0, v2);
    fixed_u32::bit_or(a1570, a1571)
}

pub fn f125(v0: u32) -> u32 {
    let a1574 = {
        let a1572 = f124(v0, 17u32, 15u32);
        let a1573 = f124(v0, 19u32, 13u32);
        fixed_u32::bit_xor(a1572, a1573)
    };
    let a1575 = f46(v0, 10u32);
    fixed_u32::bit_xor(a1574, a1575)
}

pub fn f126(v0: u32) -> u32 {
    let a1578 = {
        let a1576 = f124(v0, 7u32, 25u32);
        let a1577 = f124(v0, 18u32, 14u32);
        fixed_u32::bit_xor(a1576, a1577)
    };
    let a1579 = f46(v0, 3u32);
    fixed_u32::bit_xor(a1578, a1579)
}

pub fn f127(v0: (u64, Adt3)) -> R<(u64, Adt3)> {
    Ok(({
        let a1581 = {
            let (h1580, _) = v0.clone();
            h1580
        };
        let a1582 = 1u64;
        nat_add(a1581, a1582)?
    }, f132(f133(f134(f135({
        let (_, h1583) = v0;
        h1583
    }))), {
        let (h1584, _) = v0;
        h1584
    })))
}

pub fn f128(v0: u64, v1: u32, v2: u32) -> u64 {
    let a1585 = f55(v0, v1);
    let a1586 = f92(v0, v2);
    fixed_u64::bit_or(a1585, a1586)
}

pub fn f129(v0: u64) -> u64 {
    let a1589 = {
        let a1587 = f128(v0, 19u32, 45u32);
        let a1588 = f128(v0, 61u32, 3u32);
        fixed_u64::bit_xor(a1587, a1588)
    };
    let a1590 = f55(v0, 6u32);
    fixed_u64::bit_xor(a1589, a1590)
}

pub fn f130(v0: u64) -> u64 {
    let a1593 = {
        let a1591 = f128(v0, 1u32, 63u32);
        let a1592 = f128(v0, 8u32, 56u32);
        fixed_u64::bit_xor(a1591, a1592)
    };
    let a1594 = f55(v0, 7u32);
    fixed_u64::bit_xor(a1593, a1594)
}

pub fn f131(v0: u32, v1: u32, v2: u32, v3: u32, v4: u32, v5: u32) -> Adt11 {
    let v6: u32 = f67(f67(v0, v1), v4);
    let v7: u32 = f124({
        let a1595 = v3;
        let a1596 = v6;
        fixed_u32::bit_xor(a1595, a1596)
    }, 16u32, 16u32);
    let v8: u32 = f67(v2, v7);
    let v9: u32 = f124({
        let a1597 = v1;
        let a1598 = v8;
        fixed_u32::bit_xor(a1597, a1598)
    }, 12u32, 20u32);
    let v10: u32 = f67(f67(v6, v9), v5);
    let v11: u32 = f124({
        let a1599 = v7;
        let a1600 = v10;
        fixed_u32::bit_xor(a1599, a1600)
    }, 8u32, 24u32);
    let v12: u32 = f67(v8, v11);
    let v13: u32 = f124({
        let a1601 = v9;
        let a1602 = v12;
        fixed_u32::bit_xor(a1601, a1602)
    }, 7u32, 25u32);
    Adt11::C0(v10, v13, v12, v11)
}

pub fn f132(v0: Adt3, v1: u64) -> Adt3 {
    let v2: u64 = f136(v1);
    Adt3::C0({
        let a1604 = match v0.clone() {
            Adt3::C0(h1603, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1603
            }
        };
        let a1605 = v2;
        fixed_u64::bit_xor(a1604, a1605)
    }, match v0.clone() {
        Adt3::C0(_, h1606, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1606
        }
    }, match v0.clone() {
        Adt3::C0(_, _, h1607, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1607
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, h1608, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1608
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, h1609, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1609
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, h1610, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1610
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, h1611, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1611
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, h1612, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1612
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, h1613, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1613
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, h1614, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1614
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, h1615, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1615
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, h1616, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1616
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, h1617, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1617
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1618, _, _, _, _, _, _, _, _, _, _, _) => {
            h1618
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1619, _, _, _, _, _, _, _, _, _, _) => {
            h1619
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1620, _, _, _, _, _, _, _, _, _) => {
            h1620
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1621, _, _, _, _, _, _, _, _) => {
            h1621
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1622, _, _, _, _, _, _, _) => {
            h1622
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1623, _, _, _, _, _, _) => {
            h1623
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1624, _, _, _, _, _) => {
            h1624
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1625, _, _, _, _) => {
            h1625
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1626, _, _, _) => {
            h1626
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1627, _, _) => {
            h1627
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1628, _) => {
            h1628
        }
    }, match v0 {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1629) => {
            h1629
        }
    })
}

pub fn f133(v0: Adt3) -> Adt3 {
    Adt3::C0({
        let a1636 = match v0.clone() {
            Adt3::C0(h1630, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1630
            }
        };
        let a1637 = {
            let a1634 = {
                let a1632 = match v0.clone() {
                    Adt3::C0(_, h1631, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1631
                    }
                };
                fixed_u64::bit_not(a1632)
            };
            let a1635 = match v0.clone() {
                Adt3::C0(_, _, h1633, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1633
                }
            };
            fixed_u64::bit_and(a1634, a1635)
        };
        fixed_u64::bit_xor(a1636, a1637)
    }, {
        let a1644 = match v0.clone() {
            Adt3::C0(_, h1638, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1638
            }
        };
        let a1645 = {
            let a1642 = {
                let a1640 = match v0.clone() {
                    Adt3::C0(_, _, h1639, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1639
                    }
                };
                fixed_u64::bit_not(a1640)
            };
            let a1643 = match v0.clone() {
                Adt3::C0(_, _, _, h1641, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1641
                }
            };
            fixed_u64::bit_and(a1642, a1643)
        };
        fixed_u64::bit_xor(a1644, a1645)
    }, {
        let a1652 = match v0.clone() {
            Adt3::C0(_, _, h1646, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1646
            }
        };
        let a1653 = {
            let a1650 = {
                let a1648 = match v0.clone() {
                    Adt3::C0(_, _, _, h1647, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1647
                    }
                };
                fixed_u64::bit_not(a1648)
            };
            let a1651 = match v0.clone() {
                Adt3::C0(_, _, _, _, h1649, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1649
                }
            };
            fixed_u64::bit_and(a1650, a1651)
        };
        fixed_u64::bit_xor(a1652, a1653)
    }, {
        let a1660 = match v0.clone() {
            Adt3::C0(_, _, _, h1654, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1654
            }
        };
        let a1661 = {
            let a1658 = {
                let a1656 = match v0.clone() {
                    Adt3::C0(_, _, _, _, h1655, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1655
                    }
                };
                fixed_u64::bit_not(a1656)
            };
            let a1659 = match v0.clone() {
                Adt3::C0(h1657, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1657
                }
            };
            fixed_u64::bit_and(a1658, a1659)
        };
        fixed_u64::bit_xor(a1660, a1661)
    }, {
        let a1668 = match v0.clone() {
            Adt3::C0(_, _, _, _, h1662, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1662
            }
        };
        let a1669 = {
            let a1666 = {
                let a1664 = match v0.clone() {
                    Adt3::C0(h1663, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1663
                    }
                };
                fixed_u64::bit_not(a1664)
            };
            let a1667 = match v0.clone() {
                Adt3::C0(_, h1665, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1665
                }
            };
            fixed_u64::bit_and(a1666, a1667)
        };
        fixed_u64::bit_xor(a1668, a1669)
    }, {
        let a1676 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, h1670, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1670
            }
        };
        let a1677 = {
            let a1674 = {
                let a1672 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h1671, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1671
                    }
                };
                fixed_u64::bit_not(a1672)
            };
            let a1675 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, h1673, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1673
                }
            };
            fixed_u64::bit_and(a1674, a1675)
        };
        fixed_u64::bit_xor(a1676, a1677)
    }, {
        let a1684 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, h1678, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1678
            }
        };
        let a1685 = {
            let a1682 = {
                let a1680 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, h1679, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1679
                    }
                };
                fixed_u64::bit_not(a1680)
            };
            let a1683 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, h1681, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1681
                }
            };
            fixed_u64::bit_and(a1682, a1683)
        };
        fixed_u64::bit_xor(a1684, a1685)
    }, {
        let a1692 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, h1686, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1686
            }
        };
        let a1693 = {
            let a1690 = {
                let a1688 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, h1687, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1687
                    }
                };
                fixed_u64::bit_not(a1688)
            };
            let a1691 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, _, h1689, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1689
                }
            };
            fixed_u64::bit_and(a1690, a1691)
        };
        fixed_u64::bit_xor(a1692, a1693)
    }, {
        let a1700 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, h1694, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1694
            }
        };
        let a1701 = {
            let a1698 = {
                let a1696 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, h1695, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1695
                    }
                };
                fixed_u64::bit_not(a1696)
            };
            let a1699 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, h1697, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1697
                }
            };
            fixed_u64::bit_and(a1698, a1699)
        };
        fixed_u64::bit_xor(a1700, a1701)
    }, {
        let a1708 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, h1702, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1702
            }
        };
        let a1709 = {
            let a1706 = {
                let a1704 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, h1703, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1703
                    }
                };
                fixed_u64::bit_not(a1704)
            };
            let a1707 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, h1705, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1705
                }
            };
            fixed_u64::bit_and(a1706, a1707)
        };
        fixed_u64::bit_xor(a1708, a1709)
    }, {
        let a1716 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, h1710, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1710
            }
        };
        let a1717 = {
            let a1714 = {
                let a1712 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, h1711, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1711
                    }
                };
                fixed_u64::bit_not(a1712)
            };
            let a1715 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, h1713, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1713
                }
            };
            fixed_u64::bit_and(a1714, a1715)
        };
        fixed_u64::bit_xor(a1716, a1717)
    }, {
        let a1724 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, h1718, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1718
            }
        };
        let a1725 = {
            let a1722 = {
                let a1720 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, h1719, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1719
                    }
                };
                fixed_u64::bit_not(a1720)
            };
            let a1723 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1721, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1721
                }
            };
            fixed_u64::bit_and(a1722, a1723)
        };
        fixed_u64::bit_xor(a1724, a1725)
    }, {
        let a1732 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, h1726, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1726
            }
        };
        let a1733 = {
            let a1730 = {
                let a1728 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1727, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1727
                    }
                };
                fixed_u64::bit_not(a1728)
            };
            let a1731 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1729, _, _, _, _, _, _, _, _, _, _) => {
                    h1729
                }
            };
            fixed_u64::bit_and(a1730, a1731)
        };
        fixed_u64::bit_xor(a1732, a1733)
    }, {
        let a1740 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1734, _, _, _, _, _, _, _, _, _, _, _) => {
                h1734
            }
        };
        let a1741 = {
            let a1738 = {
                let a1736 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1735, _, _, _, _, _, _, _, _, _, _) => {
                        h1735
                    }
                };
                fixed_u64::bit_not(a1736)
            };
            let a1739 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, _, _, h1737, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1737
                }
            };
            fixed_u64::bit_and(a1738, a1739)
        };
        fixed_u64::bit_xor(a1740, a1741)
    }, {
        let a1748 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1742, _, _, _, _, _, _, _, _, _, _) => {
                h1742
            }
        };
        let a1749 = {
            let a1746 = {
                let a1744 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, _, h1743, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1743
                    }
                };
                fixed_u64::bit_not(a1744)
            };
            let a1747 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, h1745, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1745
                }
            };
            fixed_u64::bit_and(a1746, a1747)
        };
        fixed_u64::bit_xor(a1748, a1749)
    }, {
        let a1756 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1750, _, _, _, _, _, _, _, _, _) => {
                h1750
            }
        };
        let a1757 = {
            let a1754 = {
                let a1752 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1751, _, _, _, _, _, _, _, _) => {
                        h1751
                    }
                };
                fixed_u64::bit_not(a1752)
            };
            let a1755 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1753, _, _, _, _, _, _, _) => {
                    h1753
                }
            };
            fixed_u64::bit_and(a1754, a1755)
        };
        fixed_u64::bit_xor(a1756, a1757)
    }, {
        let a1764 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1758, _, _, _, _, _, _, _, _) => {
                h1758
            }
        };
        let a1765 = {
            let a1762 = {
                let a1760 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1759, _, _, _, _, _, _, _) => {
                        h1759
                    }
                };
                fixed_u64::bit_not(a1760)
            };
            let a1763 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1761, _, _, _, _, _, _) => {
                    h1761
                }
            };
            fixed_u64::bit_and(a1762, a1763)
        };
        fixed_u64::bit_xor(a1764, a1765)
    }, {
        let a1772 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1766, _, _, _, _, _, _, _) => {
                h1766
            }
        };
        let a1773 = {
            let a1770 = {
                let a1768 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1767, _, _, _, _, _, _) => {
                        h1767
                    }
                };
                fixed_u64::bit_not(a1768)
            };
            let a1771 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1769, _, _, _, _, _) => {
                    h1769
                }
            };
            fixed_u64::bit_and(a1770, a1771)
        };
        fixed_u64::bit_xor(a1772, a1773)
    }, {
        let a1780 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1774, _, _, _, _, _, _) => {
                h1774
            }
        };
        let a1781 = {
            let a1778 = {
                let a1776 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1775, _, _, _, _, _) => {
                        h1775
                    }
                };
                fixed_u64::bit_not(a1776)
            };
            let a1779 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1777, _, _, _, _, _, _, _, _, _) => {
                    h1777
                }
            };
            fixed_u64::bit_and(a1778, a1779)
        };
        fixed_u64::bit_xor(a1780, a1781)
    }, {
        let a1788 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1782, _, _, _, _, _) => {
                h1782
            }
        };
        let a1789 = {
            let a1786 = {
                let a1784 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1783, _, _, _, _, _, _, _, _, _) => {
                        h1783
                    }
                };
                fixed_u64::bit_not(a1784)
            };
            let a1787 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1785, _, _, _, _, _, _, _, _) => {
                    h1785
                }
            };
            fixed_u64::bit_and(a1786, a1787)
        };
        fixed_u64::bit_xor(a1788, a1789)
    }, {
        let a1796 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1790, _, _, _, _) => {
                h1790
            }
        };
        let a1797 = {
            let a1794 = {
                let a1792 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1791, _, _, _) => {
                        h1791
                    }
                };
                fixed_u64::bit_not(a1792)
            };
            let a1795 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1793, _, _) => {
                    h1793
                }
            };
            fixed_u64::bit_and(a1794, a1795)
        };
        fixed_u64::bit_xor(a1796, a1797)
    }, {
        let a1804 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1798, _, _, _) => {
                h1798
            }
        };
        let a1805 = {
            let a1802 = {
                let a1800 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1799, _, _) => {
                        h1799
                    }
                };
                fixed_u64::bit_not(a1800)
            };
            let a1803 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1801, _) => {
                    h1801
                }
            };
            fixed_u64::bit_and(a1802, a1803)
        };
        fixed_u64::bit_xor(a1804, a1805)
    }, {
        let a1812 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1806, _, _) => {
                h1806
            }
        };
        let a1813 = {
            let a1810 = {
                let a1808 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1807, _) => {
                        h1807
                    }
                };
                fixed_u64::bit_not(a1808)
            };
            let a1811 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1809) => {
                    h1809
                }
            };
            fixed_u64::bit_and(a1810, a1811)
        };
        fixed_u64::bit_xor(a1812, a1813)
    }, {
        let a1820 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1814, _) => {
                h1814
            }
        };
        let a1821 = {
            let a1818 = {
                let a1816 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1815) => {
                        h1815
                    }
                };
                fixed_u64::bit_not(a1816)
            };
            let a1819 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1817, _, _, _, _) => {
                    h1817
                }
            };
            fixed_u64::bit_and(a1818, a1819)
        };
        fixed_u64::bit_xor(a1820, a1821)
    }, {
        let a1828 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1822) => {
                h1822
            }
        };
        let a1829 = {
            let a1826 = {
                let a1824 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1823, _, _, _, _) => {
                        h1823
                    }
                };
                fixed_u64::bit_not(a1824)
            };
            let a1827 = match v0 {
                Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1825, _, _, _) => {
                    h1825
                }
            };
            fixed_u64::bit_and(a1826, a1827)
        };
        fixed_u64::bit_xor(a1828, a1829)
    })
}

pub fn f134(v0: Adt3) -> Adt3 {
    Adt3::C0(match v0.clone() {
        Adt3::C0(h1830, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1830
        }
    }, f137(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, h1831, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1831
        }
    }, 44u32, 20u32), f137(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, h1832, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1832
        }
    }, 43u32, 21u32), f137(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1833, _, _, _, _, _, _) => {
            h1833
        }
    }, 21u32, 43u32), f137(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1834) => {
            h1834
        }
    }, 14u32, 50u32), f137(match v0.clone() {
        Adt3::C0(_, _, _, h1835, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1835
        }
    }, 28u32, 36u32), f137(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, h1836, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1836
        }
    }, 20u32, 44u32), f137(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, h1837, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1837
        }
    }, 3u32, 61u32), f137(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1838, _, _, _, _, _, _, _, _) => {
            h1838
        }
    }, 45u32, 19u32), f137(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1839, _, _) => {
            h1839
        }
    }, 61u32, 3u32), f137(match v0.clone() {
        Adt3::C0(_, h1840, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1840
        }
    }, 1u32, 63u32), f137(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, h1841, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1841
        }
    }, 6u32, 58u32), f137(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1842, _, _, _, _, _, _, _, _, _, _, _) => {
            h1842
        }
    }, 25u32, 39u32), f137(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1843, _, _, _, _, _) => {
            h1843
        }
    }, 8u32, 56u32), f137(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1844, _, _, _, _) => {
            h1844
        }
    }, 18u32, 46u32), f137(match v0.clone() {
        Adt3::C0(_, _, _, _, h1845, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1845
        }
    }, 27u32, 37u32), f137(match v0.clone() {
        Adt3::C0(_, _, _, _, _, h1846, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1846
        }
    }, 36u32, 28u32), f137(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, h1847, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1847
        }
    }, 10u32, 54u32), f137(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1848, _, _, _, _, _, _, _) => {
            h1848
        }
    }, 15u32, 49u32), f137(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1849, _) => {
            h1849
        }
    }, 56u32, 8u32), f137(match v0.clone() {
        Adt3::C0(_, _, h1850, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1850
        }
    }, 62u32, 2u32), f137(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, h1851, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1851
        }
    }, 55u32, 9u32), f137(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1852, _, _, _, _, _, _, _, _, _, _) => {
            h1852
        }
    }, 39u32, 25u32), f137(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1853, _, _, _, _, _, _, _, _, _) => {
            h1853
        }
    }, 41u32, 23u32), f137(match v0 {
        Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1854, _, _, _) => {
            h1854
        }
    }, 2u32, 62u32))
}

pub fn f135(v0: Adt3) -> Adt3 {
    let v1: u64 = {
        let a1866 = {
            let a1863 = {
                let a1860 = {
                    let a1857 = match v0.clone() {
                        Adt3::C0(h1855, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1855
                        }
                    };
                    let a1858 = match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h1856, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1856
                        }
                    };
                    fixed_u64::bit_xor(a1857, a1858)
                };
                let a1861 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, _, h1859, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1859
                    }
                };
                fixed_u64::bit_xor(a1860, a1861)
            };
            let a1864 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1862, _, _, _, _, _, _, _, _, _) => {
                    h1862
                }
            };
            fixed_u64::bit_xor(a1863, a1864)
        };
        let a1867 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1865, _, _, _, _) => {
                h1865
            }
        };
        fixed_u64::bit_xor(a1866, a1867)
    };
    let v2: u64 = {
        let a1879 = {
            let a1876 = {
                let a1873 = {
                    let a1870 = match v0.clone() {
                        Adt3::C0(_, h1868, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1868
                        }
                    };
                    let a1871 = match v0.clone() {
                        Adt3::C0(_, _, _, _, _, _, h1869, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1869
                        }
                    };
                    fixed_u64::bit_xor(a1870, a1871)
                };
                let a1874 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, h1872, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1872
                    }
                };
                fixed_u64::bit_xor(a1873, a1874)
            };
            let a1877 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1875, _, _, _, _, _, _, _, _) => {
                    h1875
                }
            };
            fixed_u64::bit_xor(a1876, a1877)
        };
        let a1880 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1878, _, _, _) => {
                h1878
            }
        };
        fixed_u64::bit_xor(a1879, a1880)
    };
    let v3: u64 = {
        let a1892 = {
            let a1889 = {
                let a1886 = {
                    let a1883 = match v0.clone() {
                        Adt3::C0(_, _, h1881, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1881
                        }
                    };
                    let a1884 = match v0.clone() {
                        Adt3::C0(_, _, _, _, _, _, _, h1882, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1882
                        }
                    };
                    fixed_u64::bit_xor(a1883, a1884)
                };
                let a1887 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, h1885, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1885
                    }
                };
                fixed_u64::bit_xor(a1886, a1887)
            };
            let a1890 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1888, _, _, _, _, _, _, _) => {
                    h1888
                }
            };
            fixed_u64::bit_xor(a1889, a1890)
        };
        let a1893 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1891, _, _) => {
                h1891
            }
        };
        fixed_u64::bit_xor(a1892, a1893)
    };
    let v4: u64 = {
        let a1905 = {
            let a1902 = {
                let a1899 = {
                    let a1896 = match v0.clone() {
                        Adt3::C0(_, _, _, h1894, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1894
                        }
                    };
                    let a1897 = match v0.clone() {
                        Adt3::C0(_, _, _, _, _, _, _, _, h1895, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1895
                        }
                    };
                    fixed_u64::bit_xor(a1896, a1897)
                };
                let a1900 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1898, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1898
                    }
                };
                fixed_u64::bit_xor(a1899, a1900)
            };
            let a1903 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1901, _, _, _, _, _, _) => {
                    h1901
                }
            };
            fixed_u64::bit_xor(a1902, a1903)
        };
        let a1906 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1904, _) => {
                h1904
            }
        };
        fixed_u64::bit_xor(a1905, a1906)
    };
    let v5: u64 = {
        let a1918 = {
            let a1915 = {
                let a1912 = {
                    let a1909 = match v0.clone() {
                        Adt3::C0(_, _, _, _, h1907, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1907
                        }
                    };
                    let a1910 = match v0.clone() {
                        Adt3::C0(_, _, _, _, _, _, _, _, _, h1908, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1908
                        }
                    };
                    fixed_u64::bit_xor(a1909, a1910)
                };
                let a1913 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1911, _, _, _, _, _, _, _, _, _, _) => {
                        h1911
                    }
                };
                fixed_u64::bit_xor(a1912, a1913)
            };
            let a1916 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1914, _, _, _, _, _) => {
                    h1914
                }
            };
            fixed_u64::bit_xor(a1915, a1916)
        };
        let a1919 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1917) => {
                h1917
            }
        };
        fixed_u64::bit_xor(a1918, a1919)
    };
    let v6: u64 = {
        let a1920 = v5;
        let a1921 = f137(v2, 1u32, 63u32);
        fixed_u64::bit_xor(a1920, a1921)
    };
    let v7: u64 = {
        let a1922 = v1;
        let a1923 = f137(v3, 1u32, 63u32);
        fixed_u64::bit_xor(a1922, a1923)
    };
    let v8: u64 = {
        let a1924 = v2;
        let a1925 = f137(v4, 1u32, 63u32);
        fixed_u64::bit_xor(a1924, a1925)
    };
    let v9: u64 = {
        let a1926 = v3;
        let a1927 = f137(v5, 1u32, 63u32);
        fixed_u64::bit_xor(a1926, a1927)
    };
    let v10: u64 = {
        let a1928 = v4;
        let a1929 = f137(v1, 1u32, 63u32);
        fixed_u64::bit_xor(a1928, a1929)
    };
    Adt3::C0({
        let a1931 = match v0.clone() {
            Adt3::C0(h1930, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1930
            }
        };
        let a1932 = v6;
        fixed_u64::bit_xor(a1931, a1932)
    }, {
        let a1934 = match v0.clone() {
            Adt3::C0(_, h1933, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1933
            }
        };
        let a1935 = v7;
        fixed_u64::bit_xor(a1934, a1935)
    }, {
        let a1937 = match v0.clone() {
            Adt3::C0(_, _, h1936, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1936
            }
        };
        let a1938 = v8;
        fixed_u64::bit_xor(a1937, a1938)
    }, {
        let a1940 = match v0.clone() {
            Adt3::C0(_, _, _, h1939, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1939
            }
        };
        let a1941 = v9;
        fixed_u64::bit_xor(a1940, a1941)
    }, {
        let a1943 = match v0.clone() {
            Adt3::C0(_, _, _, _, h1942, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1942
            }
        };
        let a1944 = v10;
        fixed_u64::bit_xor(a1943, a1944)
    }, {
        let a1946 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, h1945, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1945
            }
        };
        let a1947 = v6;
        fixed_u64::bit_xor(a1946, a1947)
    }, {
        let a1949 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, h1948, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1948
            }
        };
        let a1950 = v7;
        fixed_u64::bit_xor(a1949, a1950)
    }, {
        let a1952 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, h1951, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1951
            }
        };
        let a1953 = v8;
        fixed_u64::bit_xor(a1952, a1953)
    }, {
        let a1955 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, h1954, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1954
            }
        };
        let a1956 = v9;
        fixed_u64::bit_xor(a1955, a1956)
    }, {
        let a1958 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, h1957, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1957
            }
        };
        let a1959 = v10;
        fixed_u64::bit_xor(a1958, a1959)
    }, {
        let a1961 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, h1960, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1960
            }
        };
        let a1962 = v6;
        fixed_u64::bit_xor(a1961, a1962)
    }, {
        let a1964 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, h1963, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1963
            }
        };
        let a1965 = v7;
        fixed_u64::bit_xor(a1964, a1965)
    }, {
        let a1967 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, h1966, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1966
            }
        };
        let a1968 = v8;
        fixed_u64::bit_xor(a1967, a1968)
    }, {
        let a1970 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1969, _, _, _, _, _, _, _, _, _, _, _) => {
                h1969
            }
        };
        let a1971 = v9;
        fixed_u64::bit_xor(a1970, a1971)
    }, {
        let a1973 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1972, _, _, _, _, _, _, _, _, _, _) => {
                h1972
            }
        };
        let a1974 = v10;
        fixed_u64::bit_xor(a1973, a1974)
    }, {
        let a1976 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1975, _, _, _, _, _, _, _, _, _) => {
                h1975
            }
        };
        let a1977 = v6;
        fixed_u64::bit_xor(a1976, a1977)
    }, {
        let a1979 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1978, _, _, _, _, _, _, _, _) => {
                h1978
            }
        };
        let a1980 = v7;
        fixed_u64::bit_xor(a1979, a1980)
    }, {
        let a1982 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1981, _, _, _, _, _, _, _) => {
                h1981
            }
        };
        let a1983 = v8;
        fixed_u64::bit_xor(a1982, a1983)
    }, {
        let a1985 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1984, _, _, _, _, _, _) => {
                h1984
            }
        };
        let a1986 = v9;
        fixed_u64::bit_xor(a1985, a1986)
    }, {
        let a1988 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1987, _, _, _, _, _) => {
                h1987
            }
        };
        let a1989 = v10;
        fixed_u64::bit_xor(a1988, a1989)
    }, {
        let a1991 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1990, _, _, _, _) => {
                h1990
            }
        };
        let a1992 = v6;
        fixed_u64::bit_xor(a1991, a1992)
    }, {
        let a1994 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1993, _, _, _) => {
                h1993
            }
        };
        let a1995 = v7;
        fixed_u64::bit_xor(a1994, a1995)
    }, {
        let a1997 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1996, _, _) => {
                h1996
            }
        };
        let a1998 = v8;
        fixed_u64::bit_xor(a1997, a1998)
    }, {
        let a2000 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1999, _) => {
                h1999
            }
        };
        let a2001 = v9;
        fixed_u64::bit_xor(a2000, a2001)
    }, {
        let a2003 = match v0 {
            Adt3::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2002) => {
                h2002
            }
        };
        let a2004 = v10;
        fixed_u64::bit_xor(a2003, a2004)
    })
}

pub fn f136(v0: u64) -> u64 {
    let a2005 = v0;
    let a2006 = 12u64;
    if nat_lt(a2005, a2006) {
        let a2007 = v0;
        let a2008 = 6u64;
        if nat_lt(a2007, a2008) {
            let a2009 = v0;
            let a2010 = 3u64;
            if nat_lt(a2009, a2010) {
                let a2011 = v0;
                let a2012 = 1u64;
                if nat_lt(a2011, a2012) {
                    1u64
                } else {
                    let a2013 = v0;
                    let a2014 = 2u64;
                    if nat_lt(a2013, a2014) {
                        32898u64
                    } else {
                        9223372036854808714u64
                    }
                }
            } else {
                let a2015 = v0;
                let a2016 = 4u64;
                if nat_lt(a2015, a2016) {
                    9223372039002292224u64
                } else {
                    let a2017 = v0;
                    let a2018 = 5u64;
                    if nat_lt(a2017, a2018) {
                        32907u64
                    } else {
                        2147483649u64
                    }
                }
            }
        } else {
            let a2019 = v0;
            let a2020 = 9u64;
            if nat_lt(a2019, a2020) {
                let a2021 = v0;
                let a2022 = 7u64;
                if nat_lt(a2021, a2022) {
                    9223372039002292353u64
                } else {
                    let a2023 = v0;
                    let a2024 = 8u64;
                    if nat_lt(a2023, a2024) {
                        9223372036854808585u64
                    } else {
                        138u64
                    }
                }
            } else {
                let a2025 = v0;
                let a2026 = 10u64;
                if nat_lt(a2025, a2026) {
                    136u64
                } else {
                    let a2027 = v0;
                    let a2028 = 11u64;
                    if nat_lt(a2027, a2028) {
                        2147516425u64
                    } else {
                        2147483658u64
                    }
                }
            }
        }
    } else {
        let a2029 = v0;
        let a2030 = 18u64;
        if nat_lt(a2029, a2030) {
            let a2031 = v0;
            let a2032 = 15u64;
            if nat_lt(a2031, a2032) {
                let a2033 = v0;
                let a2034 = 13u64;
                if nat_lt(a2033, a2034) {
                    2147516555u64
                } else {
                    let a2035 = v0;
                    let a2036 = 14u64;
                    if nat_lt(a2035, a2036) {
                        9223372036854775947u64
                    } else {
                        9223372036854808713u64
                    }
                }
            } else {
                let a2037 = v0;
                let a2038 = 16u64;
                if nat_lt(a2037, a2038) {
                    9223372036854808579u64
                } else {
                    let a2039 = v0;
                    let a2040 = 17u64;
                    if nat_lt(a2039, a2040) {
                        9223372036854808578u64
                    } else {
                        9223372036854775936u64
                    }
                }
            }
        } else {
            let a2041 = v0;
            let a2042 = 21u64;
            if nat_lt(a2041, a2042) {
                let a2043 = v0;
                let a2044 = 19u64;
                if nat_lt(a2043, a2044) {
                    32778u64
                } else {
                    let a2045 = v0;
                    let a2046 = 20u64;
                    if nat_lt(a2045, a2046) {
                        9223372039002259466u64
                    } else {
                        9223372039002292353u64
                    }
                }
            } else {
                let a2047 = v0;
                let a2048 = 22u64;
                if nat_lt(a2047, a2048) {
                    9223372036854808704u64
                } else {
                    let a2049 = v0;
                    let a2050 = 23u64;
                    if nat_lt(a2049, a2050) {
                        2147483649u64
                    } else {
                        9223372039002292232u64
                    }
                }
            }
        }
    }
}

pub fn f137(v0: u64, v1: u32, v2: u32) -> u64 {
    let a2051 = f92(v0, v1);
    let a2052 = f55(v0, v2);
    fixed_u64::bit_or(a2051, a2052)
}
