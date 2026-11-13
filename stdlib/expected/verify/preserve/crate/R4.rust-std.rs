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
    C0(bool),
    C1(bool, u64),
    C2(Bytes),
    C3(std::rc::Rc<Adt0>),
    C4(std::rc::Rc<Adt0>),
    C5,
    C6(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
    C7(Bytes, std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
}

#[derive(Clone)]
pub enum Fn0 {
    F10(Bytes),
    F26(u64, Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn0::F10(k0) => f10(k0.clone(), p0),
            Fn0::F26(k0, k1) => Ok(f26(*k0, k1.clone(), p0)),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F13,
    F48,
}

impl Fn1 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn1::F13 => f13(p0, p1),
            Fn1::F48 => f48(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F16,
}

impl Fn2 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, Bytes)) -> R<List<Bytes>> {
        match self {
            Fn2::F16 => f16(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F21(Bytes),
}

impl Fn3 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Bytes) -> Option<Bytes> {
        match self {
            Fn3::F21(k0) => f21(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F30,
}

impl Fn4 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn4::F30 => f30(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F32,
}

impl Fn5 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
        match self {
            Fn5::F32 => f32(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F37,
}

impl Fn6 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn6::F37 => f37(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F42,
}

impl Fn7 {
    pub fn apply(&self, p0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
        match self {
            Fn7::F42 => f42(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F47,
}

impl Fn8 {
    pub fn apply(&self, p0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
        match self {
            Fn8::F47 => f47(p0),
        }
    }
}

pub fn f0(v0: Adt0) -> R<Bytes> {
    f1(v0)
}

pub fn f1(v0: Adt0) -> R<Bytes> {
    let m1 = v0;
    match m1 {
        Adt0::C0(v1) => {
            if v1 {
                Ok(Bytes::lit(&[0x74, 0x72, 0x75, 0x65]))
            } else {
                Ok(Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]))
            }
        }
        Adt0::C1(v2, v3) => {
            f2(v2, v3)
        }
        Adt0::C2(v4) => {
            f3(v4)
        }
        Adt0::C3(r2) => {
            let v5 = (*r2).clone();
            Ok(f4(f5(v5)?))
        }
        Adt0::C4(r3) => {
            let v6 = (*r3).clone();
            f6(f7(v6)?)
        }
        Adt0::C5 => {
            Ok(Bytes::lit(&[]))
        }
        Adt0::C6(_, _) => {
            Ok(Bytes::lit(&[]))
        }
        Adt0::C7(_, _, _) => {
            Ok(Bytes::lit(&[]))
        }
    }
}

pub fn f2(v0: bool, v1: u64) -> R<Bytes> {
    if if v0 {
        let a4 = 0u64;
        let a5 = v1;
        nat_lt(a4, a5)
    } else {
        false
    } {
        let a6 = Bytes::lit(&[0x2d]);
        let a7 = f8(v1)?;
        Ok(append_bytes(a6, a7))
    } else {
        f8(v1)
    }
}

pub fn f3(v0: Bytes) -> R<Bytes> {
    let a13 = {
        let a11 = Bytes::lit(&[0x22]);
        let a12 = {
            let (_, h8) = {
                let (h9, _) = f9(Fn0::F10(v0.clone()), {
                    let a10 = v0;
                    length_bytes(a10)
                }, (0u64, Bytes::lit(&[])))?;
                h9
            };
            h8
        };
        append_bytes(a11, a12)
    };
    let a14 = Bytes::lit(&[0x22]);
    Ok(append_bytes(a13, a14))
}

pub fn f4(v0: List<Bytes>) -> Bytes {
    let a17 = {
        let a15 = Bytes::lit(&[0x5b]);
        let a16 = f11(v0, Bytes::lit(&[0x2c]));
        append_bytes(a15, a16)
    };
    let a18 = Bytes::lit(&[0x5d]);
    append_bytes(a17, a18)
}

pub fn f5(v0: Adt0) -> R<List<Bytes>> {
    let m19 = v0;
    match m19 {
        Adt0::C0(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt0::C1(_, _) => {
            Ok(List::<Bytes>::nil())
        }
        Adt0::C2(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt0::C3(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt0::C4(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt0::C5 => {
            Ok(List::<Bytes>::nil())
        }
        Adt0::C6(r20, r21) => {
            let v7 = (*r20).clone();
            let v8 = (*r21).clone();
            Ok(List::cons(f1(v7)?, f5(v8)?))
        }
        Adt0::C7(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f6(v0: List<(Bytes, Bytes)>) -> R<Bytes> {
    let v1: List<(Bytes, Bytes)> = f12(Fn1::F13, List::<(Bytes, Bytes)>::nil(), f14(v0)?);
    let v2: List<Bytes> = f15(Fn2::F16, List::<Bytes>::nil(), v1)?;
    let a24 = {
        let a22 = Bytes::lit(&[0x7b]);
        let a23 = f11(v2, Bytes::lit(&[0x2c]));
        append_bytes(a22, a23)
    };
    let a25 = Bytes::lit(&[0x7d]);
    Ok(append_bytes(a24, a25))
}

pub fn f7(v0: Adt0) -> R<List<(Bytes, Bytes)>> {
    let m26 = v0;
    match m26 {
        Adt0::C0(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt0::C1(_, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt0::C2(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt0::C3(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt0::C4(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt0::C5 => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt0::C6(_, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt0::C7(v9, r27, r28) => {
            let v10 = (*r27).clone();
            let v11 = (*r28).clone();
            Ok(List::cons((v9, f1(v10)?), f7(v11)?))
        }
    }
}

pub fn f8(v0: u64) -> R<Bytes> {
    f17(f18(), 10u64, v0)
}

pub fn f9(v0: Fn0, p33: u64, p34: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m32 = (p33, p34);
    {
        let mut m32 = m32;
        loop {
            let step = {
                let (v1, v2) = m32;
                let m29 = v1;
                if m29 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m29 - 1;
                    let m31 = {
                        let c30: Fn0 = v0.clone();
                        c30.apply(v2.clone())?
                    };
                    match m31 {
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
                Ok(next) => { m32 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f10(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f19(v0, v1)
}

pub fn f11(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m35 = f20(Fn3::F21(v1), None::<Bytes>, v0);
    match m35 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f12(v0: Fn1, p39: List<(Bytes, Bytes)>, p40: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m38 = (p39, p40);
    {
        let mut m38 = m38;
        loop {
            let step = {
                let (v1, v2) = m38;
                let m36 = v2;
                match m36.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c37: Fn1 = v0.clone();
                            c37.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m38 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f13(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    f22(v0, v1)
}

pub fn f14(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    f23(v0)
}

pub fn f15(v0: Fn2, p44: List<Bytes>, p45: List<(Bytes, Bytes)>) -> R<List<Bytes>> {
    let m43 = (p44, p45);
    {
        let mut m43 = m43;
        loop {
            let step = {
                let (v1, v2) = m43;
                let m41 = v2;
                match m41.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(({
                            let c42: Fn2 = v0.clone();
                            c42.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m43 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f16(v0: List<Bytes>, v1: (Bytes, Bytes)) -> R<List<Bytes>> {
    Ok(List::cons(f24(v1)?, v0))
}

pub fn f17(v0: Bytes, v1: u64, v2: u64) -> R<Bytes> {
    let a46 = v2;
    let a47 = 0u64;
    if nat_eq(a46, a47) {
        Ok(f25(v0, 0u64))
    } else {
        let (_, h48) = {
            let (h49, _) = f9(Fn0::F26(v1, v0), v2, (v2, Bytes::lit(&[])))?;
            h49
        };
        Ok(h48)
    }
}

pub fn f18() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f19(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let m53 = {
        let a51 = v0.clone();
        let a52 = {
            let (h50, _) = v1.clone();
            h50
        };
        index_bytes(a51, a52)
    };
    match m53 {
        None => {
            Ok(None::<(u64, Bytes)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a55 = {
                    let (h54, _) = v1.clone();
                    h54
                };
                let a56 = 1u64;
                nat_add(a55, a56)?
            }, {
                let a59 = {
                    let (_, h57) = v1;
                    h57
                };
                let a60 = f27(v0, {
                    let (h58, _) = v1;
                    h58
                }, v2)?;
                append_bytes(a59, a60)
            })))
        }
    }
}

pub fn f20(v0: Fn3, p64: Option<Bytes>, p65: List<Bytes>) -> Option<Bytes> {
    let m63 = (p64, p65);
    {
        let mut m63 = m63;
        loop {
            let step = {
                let (v1, v2) = m63;
                let m61 = v2;
                match m61.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(({
                            let c62: Fn3 = v0.clone();
                            c62.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m63 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f21(v0: Bytes, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m66 = v1;
    match m66 {
        None => {
            Some(v2)
        }
        Some(v3) => {
            Some({
                let a69 = {
                    let a67 = v3;
                    let a68 = v0;
                    append_bytes(a67, a68)
                };
                let a70 = v2;
                append_bytes(a69, a70)
            })
        }
    }
}

pub fn f22(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    let m71 = v0.clone();
    match m71.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, v3)) => {
            let a74 = {
                let (h72, _) = v2;
                h72
            };
            let a75 = {
                let (h73, _) = v1.clone();
                h73
            };
            if equal(a74, a75) {
                List::cons(v1, v3)
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f23(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v1: List<List<(Bytes, Bytes)>> = f28(f29(Fn4::F30, List::<List<(Bytes, Bytes)>>::nil(), v0.clone()));
    let m80 = {
        let (h76, _) = f31(Fn5::F32, {
            let a78 = {
                let a77 = v0;
                length_list(a77)
            };
            let a79 = 1u64;
            nat_add(a78, a79)?
        }, v1)?;
        h76
    };
    match m80.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f24(v0: (Bytes, Bytes)) -> R<Bytes> {
    let a85 = {
        let a82 = f3({
            let (h81, _) = v0;
            h81
        })?;
        let a83 = Bytes::lit(&[0x3a]);
        append_bytes(a82, a83)
    };
    let a86 = {
        let (_, h84) = v0;
        h84
    };
    Ok(append_bytes(a85, a86))
}

pub fn f25(v0: Bytes, v1: u64) -> Bytes {
    let m90 = {
        let a87 = v0;
        let a88 = v1;
        let a89 = 1u64;
        slice_bytes(a87, a88, a89)
    };
    match m90 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f26(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f33(v1, v0, v2)
}

pub fn f27(v0: Bytes, v1: u64, v2: u8) -> R<Bytes> {
    let v4: u64 = {
        let v3: u8 = v2;
        let a135 = {
            let a111 = {
                let a99 = {
                    let a93 = {
                        let a91 = v3;
                        let a92 = 128u8;
                        fixed_u8::bit_and(a91, a92)
                    };
                    let a94 = 128u8;
                    if equal(a93, a94) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a100 = {
                    let a97 = {
                        let a95 = v3;
                        let a96 = 64u8;
                        fixed_u8::bit_and(a95, a96)
                    };
                    let a98 = 64u8;
                    if equal(a97, a98) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a99, a100)?
            };
            let a112 = {
                let a109 = {
                    let a103 = {
                        let a101 = v3;
                        let a102 = 32u8;
                        fixed_u8::bit_and(a101, a102)
                    };
                    let a104 = 32u8;
                    if equal(a103, a104) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a110 = {
                    let a107 = {
                        let a105 = v3;
                        let a106 = 16u8;
                        fixed_u8::bit_and(a105, a106)
                    };
                    let a108 = 16u8;
                    if equal(a107, a108) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a109, a110)?
            };
            nat_add(a111, a112)?
        };
        let a136 = {
            let a133 = {
                let a121 = {
                    let a115 = {
                        let a113 = v3;
                        let a114 = 8u8;
                        fixed_u8::bit_and(a113, a114)
                    };
                    let a116 = 8u8;
                    if equal(a115, a116) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a122 = {
                    let a119 = {
                        let a117 = v3;
                        let a118 = 4u8;
                        fixed_u8::bit_and(a117, a118)
                    };
                    let a120 = 4u8;
                    if equal(a119, a120) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a121, a122)?
            };
            let a134 = {
                let a131 = {
                    let a125 = {
                        let a123 = v3;
                        let a124 = 2u8;
                        fixed_u8::bit_and(a123, a124)
                    };
                    let a126 = 2u8;
                    if equal(a125, a126) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a132 = {
                    let a129 = {
                        let a127 = v3;
                        let a128 = 1u8;
                        fixed_u8::bit_and(a127, a128)
                    };
                    let a130 = 1u8;
                    if equal(a129, a130) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a131, a132)?
            };
            nat_add(a133, a134)?
        };
        nat_add(a135, a136)?
    };
    let a137 = v4;
    let a138 = 34u64;
    if nat_eq(a137, a138) {
        Ok(Bytes::lit(&[0x5c, 0x22]))
    } else {
        let a139 = v4;
        let a140 = 92u64;
        if nat_eq(a139, a140) {
            Ok(Bytes::lit(&[0x5c, 0x5c]))
        } else {
            let a141 = v4;
            let a142 = 8u64;
            if nat_eq(a141, a142) {
                Ok(Bytes::lit(&[0x5c, 0x62]))
            } else {
                let a143 = v4;
                let a144 = 12u64;
                if nat_eq(a143, a144) {
                    Ok(Bytes::lit(&[0x5c, 0x66]))
                } else {
                    let a145 = v4;
                    let a146 = 10u64;
                    if nat_eq(a145, a146) {
                        Ok(Bytes::lit(&[0x5c, 0x6e]))
                    } else {
                        let a147 = v4;
                        let a148 = 13u64;
                        if nat_eq(a147, a148) {
                            Ok(Bytes::lit(&[0x5c, 0x72]))
                        } else {
                            let a149 = v4;
                            let a150 = 9u64;
                            if nat_eq(a149, a150) {
                                Ok(Bytes::lit(&[0x5c, 0x74]))
                            } else {
                                let a151 = v4;
                                let a152 = 32u64;
                                if nat_lt(a151, a152) {
                                    let a153 = Bytes::lit(&[0x5c, 0x75, 0x30, 0x30]);
                                    let a154 = f34(v2)?;
                                    Ok(append_bytes(a153, a154))
                                } else {
                                    Ok(f35(v0, v1, 1u64))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f28(v0: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    f36(Fn6::F37, List::<List<(Bytes, Bytes)>>::nil(), v0)
}

pub fn f29(v0: Fn4, p158: List<List<(Bytes, Bytes)>>, p159: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    let m157 = (p158, p159);
    {
        let mut m157 = m157;
        loop {
            let step = {
                let (v1, v2) = m157;
                let m155 = v2;
                match m155.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(({
                            let c156: Fn4 = v0.clone();
                            c156.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m157 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f30(v0: List<List<(Bytes, Bytes)>>, v1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
    List::cons(List::cons(v1, List::<(Bytes, Bytes)>::nil()), v0)
}

pub fn f31(v0: Fn5, p164: u64, p165: List<List<(Bytes, Bytes)>>) -> R<(List<List<(Bytes, Bytes)>>, bool)> {
    let m163 = (p164, p165);
    {
        let mut m163 = m163;
        loop {
            let step = {
                let (v1, v2) = m163;
                let m160 = v1;
                if m160 == 0 {
                    Err::<(u64, List<List<(Bytes, Bytes)>>), R<(List<List<(Bytes, Bytes)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m160 - 1;
                    let m162 = {
                        let c161: Fn5 = v0.clone();
                        c161.apply(v2.clone())?
                    };
                    match m162 {
                        None => {
                            Err::<(u64, List<List<(Bytes, Bytes)>>), R<(List<List<(Bytes, Bytes)>>, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, List<List<(Bytes, Bytes)>>), R<(List<List<(Bytes, Bytes)>>, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m163 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f32(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    f38(v0)
}

pub fn f33(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h166, _) = v2;
        h166
    };
    let a167 = v3;
    let a168 = 0u64;
    if nat_eq(a167, a168) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a169 = v3;
            let a170 = v1;
            let a171 = 0u64;
            nat_quot(a169, a170, a171)
        }, {
            let a176 = f25(v0, {
                let a172 = v3;
                let a173 = v1;
                let a174 = 0u64;
                nat_rem(a172, a173, a174)
            });
            let a177 = {
                let (_, h175) = v2;
                h175
            };
            append_bytes(a176, a177)
        }))
    }
}

pub fn f34(v0: u8) -> R<Bytes> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a222 = {
            let a198 = {
                let a186 = {
                    let a180 = {
                        let a178 = v1;
                        let a179 = 128u8;
                        fixed_u8::bit_and(a178, a179)
                    };
                    let a181 = 128u8;
                    if equal(a180, a181) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a187 = {
                    let a184 = {
                        let a182 = v1;
                        let a183 = 64u8;
                        fixed_u8::bit_and(a182, a183)
                    };
                    let a185 = 64u8;
                    if equal(a184, a185) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a186, a187)?
            };
            let a199 = {
                let a196 = {
                    let a190 = {
                        let a188 = v1;
                        let a189 = 32u8;
                        fixed_u8::bit_and(a188, a189)
                    };
                    let a191 = 32u8;
                    if equal(a190, a191) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a197 = {
                    let a194 = {
                        let a192 = v1;
                        let a193 = 16u8;
                        fixed_u8::bit_and(a192, a193)
                    };
                    let a195 = 16u8;
                    if equal(a194, a195) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a196, a197)?
            };
            nat_add(a198, a199)?
        };
        let a223 = {
            let a220 = {
                let a208 = {
                    let a202 = {
                        let a200 = v1;
                        let a201 = 8u8;
                        fixed_u8::bit_and(a200, a201)
                    };
                    let a203 = 8u8;
                    if equal(a202, a203) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a209 = {
                    let a206 = {
                        let a204 = v1;
                        let a205 = 4u8;
                        fixed_u8::bit_and(a204, a205)
                    };
                    let a207 = 4u8;
                    if equal(a206, a207) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a208, a209)?
            };
            let a221 = {
                let a218 = {
                    let a212 = {
                        let a210 = v1;
                        let a211 = 2u8;
                        fixed_u8::bit_and(a210, a211)
                    };
                    let a213 = 2u8;
                    if equal(a212, a213) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a219 = {
                    let a216 = {
                        let a214 = v1;
                        let a215 = 1u8;
                        fixed_u8::bit_and(a214, a215)
                    };
                    let a217 = 1u8;
                    if equal(a216, a217) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a218, a219)?
            };
            nat_add(a220, a221)?
        };
        nat_add(a222, a223)?
    };
    let a230 = f25(f39(), {
        let a224 = v2;
        let a225 = 16u64;
        let a226 = 0u64;
        nat_quot(a224, a225, a226)
    });
    let a231 = f25(f39(), {
        let a227 = v2;
        let a228 = 16u64;
        let a229 = 0u64;
        nat_rem(a227, a228, a229)
    });
    Ok(append_bytes(a230, a231))
}

pub fn f35(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m235 = {
        let a232 = v0;
        let a233 = v1;
        let a234 = v2;
        slice_bytes(a232, a233, a234)
    };
    match m235 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f36(v0: Fn6, p239: List<List<(Bytes, Bytes)>>, p240: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    let m238 = (p239, p240);
    {
        let mut m238 = m238;
        loop {
            let step = {
                let (v1, v2) = m238;
                let m236 = v2;
                match m236.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(({
                            let c237: Fn6 = v0.clone();
                            c237.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m238 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f37(v0: List<List<(Bytes, Bytes)>>, v1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    List::cons(v1, v0)
}

pub fn f38(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    let a242 = {
        let a241 = v0.clone();
        length_list(a241)
    };
    let a243 = 1u64;
    if nat_le(a242, a243) {
        Ok(None::<List<List<(Bytes, Bytes)>>>)
    } else {
        Ok(Some(f40(v0)?))
    }
}

pub fn f39() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66])
}

pub fn f40(v0: List<List<(Bytes, Bytes)>>) -> R<List<List<(Bytes, Bytes)>>> {
    Ok(f28({
        let (_, h244) = {
            let (h245, _) = f41(Fn7::F42, {
                let a247 = {
                    let a246 = v0.clone();
                    length_list(a246)
                };
                let a248 = 1u64;
                nat_add(a247, a248)?
            }, (v0, List::<List<(Bytes, Bytes)>>::nil()))?;
            h245
        };
        h244
    }))
}

pub fn f41(v0: Fn7, p253: u64, p254: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)> {
    let m252 = (p253, p254);
    {
        let mut m252 = m252;
        loop {
            let step = {
                let (v1, v2) = m252;
                let m249 = v1;
                if m249 == 0 {
                    Err::<(u64, (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)), R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m249 - 1;
                    let m251 = {
                        let c250: Fn7 = v0.clone();
                        c250.apply(v2.clone())?
                    };
                    match m251 {
                        None => {
                            Err::<(u64, (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)), R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)), R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m252 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f42(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    f43(v0)
}

pub fn f43(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    let v1: List<List<(Bytes, Bytes)>> = {
        let (h255, _) = v0;
        h255
    };
    let v2: List<List<(Bytes, Bytes)>> = {
        let (_, h256) = v0;
        h256
    };
    let m257 = v1;
    match m257.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>)
        }
        Some((v3, v4)) => {
            let m258 = v4;
            match m258.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Bytes)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f44(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f44(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v2: u64 = {
        let a263 = {
            let a261 = {
                let a259 = v0.clone();
                length_list(a259)
            };
            let a262 = {
                let a260 = v1.clone();
                length_list(a260)
            };
            nat_add(a261, a262)?
        };
        let a264 = 1u64;
        nat_add(a263, a264)?
    };
    Ok(f45({
        let (_, h265) = {
            let (_, h266) = {
                let (h267, _) = f46(Fn8::F47, v2, (v0, (v1, List::<(Bytes, Bytes)>::nil())));
                h267
            };
            h266
        };
        h265
    }))
}

pub fn f45(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f12(Fn1::F48, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f46(v0: Fn8, p272: u64, p273: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool) {
    let m271 = (p272, p273);
    {
        let mut m271 = m271;
        loop {
            let step = {
                let (v1, v2) = m271;
                let m268 = v1;
                if m268 == 0 {
                    Err::<(u64, (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))), ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool)>((v2, false))
                } else {
                    let v3 = m268 - 1;
                    let m270 = {
                        let c269: Fn8 = v0.clone();
                        c269.apply(v2.clone())
                    };
                    match m270 {
                        None => {
                            Err::<(u64, (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))), ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool)>((v2, true))
                        }
                        Some(v4) => {
                            Ok::<(u64, (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))), ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool)>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m271 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f47(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    f49(v0)
}

pub fn f48(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn f49(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    let v1: List<(Bytes, Bytes)> = {
        let (h274, _) = v0.clone();
        h274
    };
    let v2: List<(Bytes, Bytes)> = {
        let (h275, _) = {
            let (_, h276) = v0.clone();
            h276
        };
        h275
    };
    let v3: List<(Bytes, Bytes)> = {
        let (_, h277) = {
            let (_, h278) = v0;
            h278
        };
        h277
    };
    let m279 = v1.clone();
    match m279.uncons() {
        None => {
            let m280 = v2;
            match m280.uncons() {
                None => {
                    None::<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Bytes)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m281 = v2.clone();
            match m281.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Bytes)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f50({
                        let (h282, _) = v8.clone();
                        h282
                    }, {
                        let (h283, _) = v6.clone();
                        h283
                    }) {
                        Some((v1, (v9, List::cons(v8, v3))))
                    } else {
                        Some((v7, (v2, List::cons(v6, v3))))
                    }
                }
            }
        }
    }
}

pub fn f50(v0: Bytes, v1: Bytes) -> bool {
    let a288 = {
        let a284 = v0;
        let a285 = v1;
        compare_bytes(a284, a285)
    };
    let a289 = {
        let a286 = Bytes::lit(&[]);
        let a287 = Bytes::lit(&[0x01]);
        compare_bytes(a286, a287)
    };
    equal(a288, a289)
}
