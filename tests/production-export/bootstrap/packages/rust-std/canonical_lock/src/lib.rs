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
    C0(Bytes, Bytes, Bytes, List<(Bytes, Bytes)>, List<Adt1>, List<Adt2>, Option<Adt3>),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(Bytes, Bytes, Bytes, Bytes, Bytes, Bytes, Bytes, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt2 {
    C0(Bytes, Bytes, Bytes, Bytes, Bytes, Bytes, Bytes, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt3 {
    C0(Bytes, Bytes, List<Bytes>, Bytes, List<Bytes>, Bytes, List<(Bytes, Bytes)>),
}

#[derive(Clone)]
pub enum Fn0 {
    F8,
}

impl Fn0 {
    pub fn apply(&self, p0: Bytes, p1: Adt1) -> R<Bytes> {
        match self {
            Fn0::F8 => f8(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F11,
}

impl Fn1 {
    pub fn apply(&self, p0: Bytes, p1: Adt2) -> R<Bytes> {
        match self {
            Fn1::F11 => f11(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F18(Bytes),
}

impl Fn2 {
    pub fn apply(&self, p0: Bytes, p1: (Bytes, Bytes)) -> R<Bytes> {
        match self {
            Fn2::F18(k0) => f18(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F22,
}

impl Fn3 {
    pub fn apply(&self, p0: List<Adt1>, p1: (Bytes, Adt1)) -> List<Adt1> {
        match self {
            Fn3::F22 => f22(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F26,
}

impl Fn4 {
    pub fn apply(&self, p0: List<(Bytes, Adt1)>, p1: Adt1) -> R<List<(Bytes, Adt1)>> {
        match self {
            Fn4::F26 => f26(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F29,
}

impl Fn5 {
    pub fn apply(&self, p0: List<Adt2>, p1: (Bytes, Adt2)) -> List<Adt2> {
        match self {
            Fn5::F29 => f29(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F33,
}

impl Fn6 {
    pub fn apply(&self, p0: List<(Bytes, Adt2)>, p1: Adt2) -> List<(Bytes, Adt2)> {
        match self {
            Fn6::F33 => f33(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F37(Bytes),
    F38(Bytes),
}

impl Fn7 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn7::F37(k0) => f37(k0.clone(), p0),
            Fn7::F38(k0) => f38(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F40,
}

impl Fn8 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, (Bytes, Bytes))) -> List<(Bytes, Bytes)> {
        match self {
            Fn8::F40 => f40(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F44,
}

impl Fn9 {
    pub fn apply(&self, p0: List<(Bytes, (Bytes, Bytes))>, p1: (Bytes, Bytes)) -> List<(Bytes, (Bytes, Bytes))> {
        match self {
            Fn9::F44 => f44(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F46,
}

impl Fn10 {
    pub fn apply(&self, p0: List<(Bytes, Adt1)>, p1: (Bytes, Adt1)) -> List<(Bytes, Adt1)> {
        match self {
            Fn10::F46 => f46(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F49,
}

impl Fn11 {
    pub fn apply(&self, p0: List<List<(Bytes, Adt1)>>, p1: (Bytes, Adt1)) -> List<List<(Bytes, Adt1)>> {
        match self {
            Fn11::F49 => f49(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F51,
}

impl Fn12 {
    pub fn apply(&self, p0: List<List<(Bytes, Adt1)>>) -> R<Option<List<List<(Bytes, Adt1)>>>> {
        match self {
            Fn12::F51 => f51(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F54,
}

impl Fn13 {
    pub fn apply(&self, p0: List<(Bytes, Adt2)>, p1: (Bytes, Adt2)) -> List<(Bytes, Adt2)> {
        match self {
            Fn13::F54 => f54(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F57,
}

impl Fn14 {
    pub fn apply(&self, p0: List<List<(Bytes, Adt2)>>, p1: (Bytes, Adt2)) -> List<List<(Bytes, Adt2)>> {
        match self {
            Fn14::F57 => f57(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F59,
}

impl Fn15 {
    pub fn apply(&self, p0: List<List<(Bytes, Adt2)>>) -> R<Option<List<List<(Bytes, Adt2)>>>> {
        match self {
            Fn15::F59 => f59(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F63,
    F82,
    F109,
}

impl Fn16 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> R<List<Bytes>> {
        match self {
            Fn16::F63 => f63(p0, p1),
            Fn16::F82 => Ok(f82(p0, p1)),
            Fn16::F109 => Ok(f109(p0, p1)),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F67,
}

impl Fn17 {
    pub fn apply(&self, p0: List<(Bytes, (Bytes, Bytes))>, p1: (Bytes, (Bytes, Bytes))) -> List<(Bytes, (Bytes, Bytes))> {
        match self {
            Fn17::F67 => f67(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F70,
}

impl Fn18 {
    pub fn apply(&self, p0: List<List<(Bytes, (Bytes, Bytes))>>, p1: (Bytes, (Bytes, Bytes))) -> List<List<(Bytes, (Bytes, Bytes))>> {
        match self {
            Fn18::F70 => f70(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F72,
}

impl Fn19 {
    pub fn apply(&self, p0: List<List<(Bytes, (Bytes, Bytes))>>) -> R<Option<List<List<(Bytes, (Bytes, Bytes))>>>> {
        match self {
            Fn19::F72 => f72(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn20 {
    F74,
}

impl Fn20 {
    pub fn apply(&self, p0: List<List<(Bytes, Adt1)>>, p1: List<(Bytes, Adt1)>) -> List<List<(Bytes, Adt1)>> {
        match self {
            Fn20::F74 => f74(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn21 {
    F78,
}

impl Fn21 {
    pub fn apply(&self, p0: List<List<(Bytes, Adt2)>>, p1: List<(Bytes, Adt2)>) -> List<List<(Bytes, Adt2)>> {
        match self {
            Fn21::F78 => f78(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn22 {
    F81(Bytes),
}

impl Fn22 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Bytes) -> Option<Bytes> {
        match self {
            Fn22::F81(k0) => f81(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn23 {
    F87,
}

impl Fn23 {
    pub fn apply(&self, p0: List<List<(Bytes, (Bytes, Bytes))>>, p1: List<(Bytes, (Bytes, Bytes))>) -> List<List<(Bytes, (Bytes, Bytes))>> {
        match self {
            Fn23::F87 => f87(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn24 {
    F98,
}

impl Fn24 {
    pub fn apply(&self, p0: (List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>)) -> R<Option<(List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>)>> {
        match self {
            Fn24::F98 => f98(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn25 {
    F100(u8, Bytes),
}

impl Fn25 {
    pub fn apply(&self, p0: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
        match self {
            Fn25::F100(k0, k1) => f100(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn26 {
    F104,
}

impl Fn26 {
    pub fn apply(&self, p0: (List<List<(Bytes, Adt2)>>, List<List<(Bytes, Adt2)>>)) -> R<Option<(List<List<(Bytes, Adt2)>>, List<List<(Bytes, Adt2)>>)>> {
        match self {
            Fn26::F104 => f104(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn27 {
    F106,
}

impl Fn27 {
    pub fn apply(&self, p0: (List<List<(Bytes, (Bytes, Bytes))>>, List<List<(Bytes, (Bytes, Bytes))>>)) -> R<Option<(List<List<(Bytes, (Bytes, Bytes))>>, List<List<(Bytes, (Bytes, Bytes))>>)>> {
        match self {
            Fn27::F106 => f106(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn28 {
    F116,
}

impl Fn28 {
    pub fn apply(&self, p0: (List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>))) -> Option<(List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>))> {
        match self {
            Fn28::F116 => f116(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn29 {
    F118,
}

impl Fn29 {
    pub fn apply(&self, p0: (List<(Bytes, Adt2)>, (List<(Bytes, Adt2)>, List<(Bytes, Adt2)>))) -> Option<(List<(Bytes, Adt2)>, (List<(Bytes, Adt2)>, List<(Bytes, Adt2)>))> {
        match self {
            Fn29::F118 => f118(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn30 {
    F120,
}

impl Fn30 {
    pub fn apply(&self, p0: (List<(Bytes, (Bytes, Bytes))>, (List<(Bytes, (Bytes, Bytes))>, List<(Bytes, (Bytes, Bytes))>))) -> Option<(List<(Bytes, (Bytes, Bytes))>, (List<(Bytes, (Bytes, Bytes))>, List<(Bytes, (Bytes, Bytes))>))> {
        match self {
            Fn30::F120 => f120(p0),
        }
    }
}

pub fn f0(v0: Adt0) -> R<Bytes> {
    let v1: Bytes = {
        let a2 = match v0.clone() {
            Adt0::C0(h1, _, _, _, _, _, _) => {
                h1
            }
        };
        let a3 = f1(Str::lit("1.2"));
        if equal(a2, a3) {
            f1(Str::lit("lexlean/lock/2"))
        } else {
            f1(Str::lit("lexlean/lock/1"))
        }
    };
    let v2: Bytes = f2(f2(f2(f2(f3(f1(Str::lit("spec")), v1)?, f3(f1(Str::lit("language")), match v0.clone() {
        Adt0::C0(h4, _, _, _, _, _, _) => {
            h4
        }
    })?), f4(f1(Str::lit("compiler_semantics")), match v0.clone() {
        Adt0::C0(_, h5, _, _, _, _, _) => {
            h5
        }
    })?), f4(f1(Str::lit("project_config_sha256")), match v0.clone() {
        Adt0::C0(_, _, h6, _, _, _, _) => {
            h6
        }
    })?), f3(f1(Str::lit("lean_toolchain")), f5())?);
    let v3: Bytes = f6(f1(Str::lit("[[workspace_file]]")), match v0.clone() {
        Adt0::C0(_, _, _, h7, _, _, _) => {
            h7
        }
    })?;
    let v4: Bytes = f7(Fn0::F8, Bytes::lit(&[]), f9(match v0.clone() {
        Adt0::C0(_, _, _, _, h8, _, _) => {
            h8
        }
    })?)?;
    let v5: Bytes = f10(Fn1::F11, Bytes::lit(&[]), f12(match v0.clone() {
        Adt0::C0(_, _, _, _, _, h9, _) => {
            h9
        }
    })?)?;
    let v7: Bytes = {
        let m11 = match v0 {
            Adt0::C0(_, _, _, _, _, _, h10) => {
                h10
            }
        };
        match m11 {
            None => {
                Bytes::lit(&[])
            }
            Some(v6) => {
                f13(v6)?
            }
        }
    };
    Ok(f2(f2(f2(f2(v2, v3), v4), v5), v7))
}

pub fn f1(v0: Str) -> Bytes {
    let a12 = v0;
    utf8_encode(a12)
}

pub fn f2(v0: Bytes, v1: Bytes) -> Bytes {
    let a13 = v0;
    let a14 = v1;
    append_bytes(a13, a14)
}

pub fn f3(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f14(v0, f15(v1)?))
}

pub fn f4(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f14(v0, f15(f16(v1)?)?))
}

pub fn f5() -> Bytes {
    f1(Str::lit("leanprover/lean4:v4.32.2"))
}

pub fn f6(v0: Bytes, v1: List<(Bytes, Bytes)>) -> R<Bytes> {
    f17(Fn2::F18(v0), Bytes::lit(&[]), f19(v1)?)
}

pub fn f7(v0: Fn0, p18: Bytes, p19: List<Adt1>) -> R<Bytes> {
    let m17 = (p18, p19);
    {
        let mut m17 = m17;
        loop {
            let step = {
                let (v1, v2) = m17;
                let m15 = v2;
                match m15.uncons() {
                    None => {
                        Err::<(Bytes, List<Adt1>), R<Bytes>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Bytes, List<Adt1>), R<Bytes>>(({
                            let c16: Fn0 = v0.clone();
                            c16.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m17 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f8(v0: Bytes, v1: Adt1) -> R<Bytes> {
    Ok(f2(v0, f20(v1)?))
}

pub fn f9(v0: List<Adt1>) -> R<List<Adt1>> {
    Ok(f21(Fn3::F22, List::<Adt1>::nil(), f23(f24(f23(f25(Fn4::F26, List::<(Bytes, Adt1)>::nil(), v0)?))?)))
}

pub fn f10(v0: Fn1, p23: Bytes, p24: List<Adt2>) -> R<Bytes> {
    let m22 = (p23, p24);
    {
        let mut m22 = m22;
        loop {
            let step = {
                let (v1, v2) = m22;
                let m20 = v2;
                match m20.uncons() {
                    None => {
                        Err::<(Bytes, List<Adt2>), R<Bytes>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Bytes, List<Adt2>), R<Bytes>>(({
                            let c21: Fn1 = v0.clone();
                            c21.apply(v1, v3)?
                        }, v4))
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

pub fn f11(v0: Bytes, v1: Adt2) -> R<Bytes> {
    Ok(f2(v0, f27(v1)?))
}

pub fn f12(v0: List<Adt2>) -> R<List<Adt2>> {
    Ok(f28(Fn5::F29, List::<Adt2>::nil(), f30(f31(f30(f32(Fn6::F33, List::<(Bytes, Adt2)>::nil(), v0)))?)))
}

pub fn f13(v0: Adt3) -> R<Bytes> {
    Ok(f2(f2(f2(f2(f2(f2(f2(f34(f1(Str::lit("[pdf]"))), f3(f1(Str::lit("program")), match v0.clone() {
        Adt3::C0(h25, _, _, _, _, _, _) => {
            h25
        }
    })?), f4(f1(Str::lit("program_sha256")), match v0.clone() {
        Adt3::C0(_, h26, _, _, _, _, _) => {
            h26
        }
    })?), f14(f1(Str::lit("version_argv")), f35(match v0.clone() {
        Adt3::C0(_, _, h27, _, _, _, _) => {
            h27
        }
    })?)), f4(f1(Str::lit("version_stdout_sha256")), match v0.clone() {
        Adt3::C0(_, _, _, h28, _, _, _) => {
            h28
        }
    })?), f14(f1(Str::lit("compile_argv")), f35(match v0.clone() {
        Adt3::C0(_, _, _, _, h29, _, _) => {
            h29
        }
    })?)), f3(f1(Str::lit("output")), match v0.clone() {
        Adt3::C0(_, _, _, _, _, h30, _) => {
            h30
        }
    })?), f6(f1(Str::lit("[[pdf.resource]]")), match v0 {
        Adt3::C0(_, _, _, _, _, _, h31) => {
            h31
        }
    })?))
}

pub fn f14(v0: Bytes, v1: Bytes) -> Bytes {
    f2(f2(f2(v0, f1(Str::lit(" = "))), v1), Bytes::lit(&[0x0a]))
}

pub fn f15(v0: Bytes) -> R<Bytes> {
    Ok(f2(f2(Bytes::lit(&[0x22]), {
        let (_, h32) = {
            let (h33, _) = f36(Fn7::F37(v0.clone()), {
                let a35 = {
                    let a34 = v0;
                    length_bytes(a34)
                };
                let a36 = 1u64;
                nat_add(a35, a36)?
            }, (0u64, Bytes::lit(&[])))?;
            h33
        };
        h32
    }), Bytes::lit(&[0x22])))
}

pub fn f16(v0: Bytes) -> R<Bytes> {
    let (_, h37) = {
        let (h38, _) = f36(Fn7::F38(v0.clone()), {
            let a39 = v0;
            length_bytes(a39)
        }, (0u64, Bytes::lit(&[])))?;
        h38
    };
    Ok(h37)
}

pub fn f17(v0: Fn2, p43: Bytes, p44: List<(Bytes, Bytes)>) -> R<Bytes> {
    let m42 = (p43, p44);
    {
        let mut m42 = m42;
        loop {
            let step = {
                let (v1, v2) = m42;
                let m40 = v2;
                match m40.uncons() {
                    None => {
                        Err::<(Bytes, List<(Bytes, Bytes)>), R<Bytes>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Bytes, List<(Bytes, Bytes)>), R<Bytes>>(({
                            let c41: Fn2 = v0.clone();
                            c41.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m42 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f18(v0: Bytes, v1: Bytes, v2: (Bytes, Bytes)) -> R<Bytes> {
    Ok(f2(v1, f2(f2(f34(v0), f3(f1(Str::lit("path")), {
        let (h45, _) = v2;
        h45
    })?), f4(f1(Str::lit("sha256")), {
        let (_, h46) = v2;
        h46
    })?)))
}

pub fn f19(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    Ok(f39(Fn8::F40, List::<(Bytes, Bytes)>::nil(), f41(f42(f41(f43(Fn9::F44, List::<(Bytes, (Bytes, Bytes))>::nil(), v0)))?)))
}

pub fn f20(v0: Adt1) -> R<Bytes> {
    Ok(f2(f2(f2(f2(f2(f2(f2(f2(f34(f1(Str::lit("[[package]]"))), f3(f1(Str::lit("id")), match v0.clone() {
        Adt1::C0(h47, _, _, _, _, _, _, _) => {
            h47
        }
    })?), f3(f1(Str::lit("version")), match v0.clone() {
        Adt1::C0(_, h48, _, _, _, _, _, _) => {
            h48
        }
    })?), f3(f1(Str::lit("kind")), match v0.clone() {
        Adt1::C0(_, _, h49, _, _, _, _, _) => {
            h49
        }
    })?), f3(f1(Str::lit("source")), match v0.clone() {
        Adt1::C0(_, _, _, h50, _, _, _, _) => {
            h50
        }
    })?), f3(f1(Str::lit("revision")), match v0.clone() {
        Adt1::C0(_, _, _, _, h51, _, _, _) => {
            h51
        }
    })?), f4(f1(Str::lit("tree_sha256")), match v0.clone() {
        Adt1::C0(_, _, _, _, _, h52, _, _) => {
            h52
        }
    })?), f4(f1(Str::lit("manifest_sha256")), match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h53, _) => {
            h53
        }
    })?), f14(f1(Str::lit("imports")), f35(match v0 {
        Adt1::C0(_, _, _, _, _, _, _, h54) => {
            h54
        }
    })?)))
}

pub fn f21(v0: Fn3, p58: List<Adt1>, p59: List<(Bytes, Adt1)>) -> List<Adt1> {
    let m57 = (p58, p59);
    {
        let mut m57 = m57;
        loop {
            let step = {
                let (v1, v2) = m57;
                let m55 = v2;
                match m55.uncons() {
                    None => {
                        Err::<(List<Adt1>, List<(Bytes, Adt1)>), List<Adt1>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt1>, List<(Bytes, Adt1)>), List<Adt1>>(({
                            let c56: Fn3 = v0.clone();
                            c56.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m57 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f22(v0: List<Adt1>, v1: (Bytes, Adt1)) -> List<Adt1> {
    List::cons({
        let (_, h60) = v1;
        h60
    }, v0)
}

pub fn f23(v0: List<(Bytes, Adt1)>) -> List<(Bytes, Adt1)> {
    f45(Fn10::F46, List::<(Bytes, Adt1)>::nil(), v0)
}

pub fn f24(v0: List<(Bytes, Adt1)>) -> R<List<(Bytes, Adt1)>> {
    let v1: List<List<(Bytes, Adt1)>> = f47(f48(Fn11::F49, List::<List<(Bytes, Adt1)>>::nil(), v0.clone()));
    let m65 = {
        let (h61, _) = f50(Fn12::F51, {
            let a63 = {
                let a62 = v0;
                length_list(a62)
            };
            let a64 = 1u64;
            nat_add(a63, a64)?
        }, v1)?;
        h61
    };
    match m65.uncons() {
        None => {
            Ok(List::<(Bytes, Adt1)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f25(v0: Fn4, p69: List<(Bytes, Adt1)>, p70: List<Adt1>) -> R<List<(Bytes, Adt1)>> {
    let m68 = (p69, p70);
    {
        let mut m68 = m68;
        loop {
            let step = {
                let (v1, v2) = m68;
                let m66 = v2;
                match m66.uncons() {
                    None => {
                        Err::<(List<(Bytes, Adt1)>, List<Adt1>), R<List<(Bytes, Adt1)>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Adt1)>, List<Adt1>), R<List<(Bytes, Adt1)>>>(({
                            let c67: Fn4 = v0.clone();
                            c67.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m68 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f26(v0: List<(Bytes, Adt1)>, v1: Adt1) -> R<List<(Bytes, Adt1)>> {
    Ok(List::cons((f52(v1.clone())?, v1), v0))
}

pub fn f27(v0: Adt2) -> R<Bytes> {
    Ok(f2(f2(f2(f2(f2(f2(f2(f2(f34(f1(Str::lit("[[source_package]]"))), f3(f1(Str::lit("id")), match v0.clone() {
        Adt2::C0(h71, _, _, _, _, _, _, _) => {
            h71
        }
    })?), f3(f1(Str::lit("version")), match v0.clone() {
        Adt2::C0(_, h72, _, _, _, _, _, _) => {
            h72
        }
    })?), f3(f1(Str::lit("kind")), match v0.clone() {
        Adt2::C0(_, _, h73, _, _, _, _, _) => {
            h73
        }
    })?), f3(f1(Str::lit("source")), match v0.clone() {
        Adt2::C0(_, _, _, h74, _, _, _, _) => {
            h74
        }
    })?), f3(f1(Str::lit("module_prefix")), match v0.clone() {
        Adt2::C0(_, _, _, _, h75, _, _, _) => {
            h75
        }
    })?), f4(f1(Str::lit("tree_sha256")), match v0.clone() {
        Adt2::C0(_, _, _, _, _, h76, _, _) => {
            h76
        }
    })?), f4(f1(Str::lit("manifest_sha256")), match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h77, _) => {
            h77
        }
    })?), f14(f1(Str::lit("imports")), f35(match v0 {
        Adt2::C0(_, _, _, _, _, _, _, h78) => {
            h78
        }
    })?)))
}

pub fn f28(v0: Fn5, p82: List<Adt2>, p83: List<(Bytes, Adt2)>) -> List<Adt2> {
    let m81 = (p82, p83);
    {
        let mut m81 = m81;
        loop {
            let step = {
                let (v1, v2) = m81;
                let m79 = v2;
                match m79.uncons() {
                    None => {
                        Err::<(List<Adt2>, List<(Bytes, Adt2)>), List<Adt2>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt2>, List<(Bytes, Adt2)>), List<Adt2>>(({
                            let c80: Fn5 = v0.clone();
                            c80.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m81 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f29(v0: List<Adt2>, v1: (Bytes, Adt2)) -> List<Adt2> {
    List::cons({
        let (_, h84) = v1;
        h84
    }, v0)
}

pub fn f30(v0: List<(Bytes, Adt2)>) -> List<(Bytes, Adt2)> {
    f53(Fn13::F54, List::<(Bytes, Adt2)>::nil(), v0)
}

pub fn f31(v0: List<(Bytes, Adt2)>) -> R<List<(Bytes, Adt2)>> {
    let v1: List<List<(Bytes, Adt2)>> = f55(f56(Fn14::F57, List::<List<(Bytes, Adt2)>>::nil(), v0.clone()));
    let m89 = {
        let (h85, _) = f58(Fn15::F59, {
            let a87 = {
                let a86 = v0;
                length_list(a86)
            };
            let a88 = 1u64;
            nat_add(a87, a88)?
        }, v1)?;
        h85
    };
    match m89.uncons() {
        None => {
            Ok(List::<(Bytes, Adt2)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f32(v0: Fn6, p93: List<(Bytes, Adt2)>, p94: List<Adt2>) -> List<(Bytes, Adt2)> {
    let m92 = (p93, p94);
    {
        let mut m92 = m92;
        loop {
            let step = {
                let (v1, v2) = m92;
                let m90 = v2;
                match m90.uncons() {
                    None => {
                        Err::<(List<(Bytes, Adt2)>, List<Adt2>), List<(Bytes, Adt2)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Adt2)>, List<Adt2>), List<(Bytes, Adt2)>>(({
                            let c91: Fn6 = v0.clone();
                            c91.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m92 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f33(v0: List<(Bytes, Adt2)>, v1: Adt2) -> List<(Bytes, Adt2)> {
    List::cons((match v1.clone() {
        Adt2::C0(h95, _, _, _, _, _, _, _) => {
            h95
        }
    }, v1), v0)
}

pub fn f34(v0: Bytes) -> Bytes {
    f2(f2(Bytes::lit(&[0x0a]), v0), Bytes::lit(&[0x0a]))
}

pub fn f35(v0: List<Bytes>) -> R<Bytes> {
    Ok(f2(f2(f1(Str::lit("[")), f60(f61(f62(Fn16::F63, List::<Bytes>::nil(), v0)?)?, f1(Str::lit(", ")))), f1(Str::lit("]"))))
}

pub fn f36(v0: Fn7, p100: u64, p101: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m99 = (p100, p101);
    {
        let mut m99 = m99;
        loop {
            let step = {
                let (v1, v2) = m99;
                let m96 = v1;
                if m96 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m96 - 1;
                    let m98 = {
                        let c97: Fn7 = v0.clone();
                        c97.apply(v2.clone())?
                    };
                    match m98 {
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
                Ok(next) => { m99 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f37(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f64(v0, v1)
}

pub fn f38(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f65(v0, v1)
}

pub fn f39(v0: Fn8, p105: List<(Bytes, Bytes)>, p106: List<(Bytes, (Bytes, Bytes))>) -> List<(Bytes, Bytes)> {
    let m104 = (p105, p106);
    {
        let mut m104 = m104;
        loop {
            let step = {
                let (v1, v2) = m104;
                let m102 = v2;
                match m102.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, (Bytes, Bytes))>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, (Bytes, Bytes))>), List<(Bytes, Bytes)>>(({
                            let c103: Fn8 = v0.clone();
                            c103.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m104 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f40(v0: List<(Bytes, Bytes)>, v1: (Bytes, (Bytes, Bytes))) -> List<(Bytes, Bytes)> {
    List::cons({
        let (_, h107) = v1;
        h107
    }, v0)
}

pub fn f41(v0: List<(Bytes, (Bytes, Bytes))>) -> List<(Bytes, (Bytes, Bytes))> {
    f66(Fn17::F67, List::<(Bytes, (Bytes, Bytes))>::nil(), v0)
}

pub fn f42(v0: List<(Bytes, (Bytes, Bytes))>) -> R<List<(Bytes, (Bytes, Bytes))>> {
    let v1: List<List<(Bytes, (Bytes, Bytes))>> = f68(f69(Fn18::F70, List::<List<(Bytes, (Bytes, Bytes))>>::nil(), v0.clone()));
    let m112 = {
        let (h108, _) = f71(Fn19::F72, {
            let a110 = {
                let a109 = v0;
                length_list(a109)
            };
            let a111 = 1u64;
            nat_add(a110, a111)?
        }, v1)?;
        h108
    };
    match m112.uncons() {
        None => {
            Ok(List::<(Bytes, (Bytes, Bytes))>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f43(v0: Fn9, p116: List<(Bytes, (Bytes, Bytes))>, p117: List<(Bytes, Bytes)>) -> List<(Bytes, (Bytes, Bytes))> {
    let m115 = (p116, p117);
    {
        let mut m115 = m115;
        loop {
            let step = {
                let (v1, v2) = m115;
                let m113 = v2;
                match m113.uncons() {
                    None => {
                        Err::<(List<(Bytes, (Bytes, Bytes))>, List<(Bytes, Bytes)>), List<(Bytes, (Bytes, Bytes))>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, (Bytes, Bytes))>, List<(Bytes, Bytes)>), List<(Bytes, (Bytes, Bytes))>>(({
                            let c114: Fn9 = v0.clone();
                            c114.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m115 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f44(v0: List<(Bytes, (Bytes, Bytes))>, v1: (Bytes, Bytes)) -> List<(Bytes, (Bytes, Bytes))> {
    List::cons(({
        let (h118, _) = v1.clone();
        h118
    }, v1), v0)
}

pub fn f45(v0: Fn10, p122: List<(Bytes, Adt1)>, p123: List<(Bytes, Adt1)>) -> List<(Bytes, Adt1)> {
    let m121 = (p122, p123);
    {
        let mut m121 = m121;
        loop {
            let step = {
                let (v1, v2) = m121;
                let m119 = v2;
                match m119.uncons() {
                    None => {
                        Err::<(List<(Bytes, Adt1)>, List<(Bytes, Adt1)>), List<(Bytes, Adt1)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Adt1)>, List<(Bytes, Adt1)>), List<(Bytes, Adt1)>>(({
                            let c120: Fn10 = v0.clone();
                            c120.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m121 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f46(v0: List<(Bytes, Adt1)>, v1: (Bytes, Adt1)) -> List<(Bytes, Adt1)> {
    List::cons(v1, v0)
}

pub fn f47(v0: List<List<(Bytes, Adt1)>>) -> List<List<(Bytes, Adt1)>> {
    f73(Fn20::F74, List::<List<(Bytes, Adt1)>>::nil(), v0)
}

pub fn f48(v0: Fn11, p127: List<List<(Bytes, Adt1)>>, p128: List<(Bytes, Adt1)>) -> List<List<(Bytes, Adt1)>> {
    let m126 = (p127, p128);
    {
        let mut m126 = m126;
        loop {
            let step = {
                let (v1, v2) = m126;
                let m124 = v2;
                match m124.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Adt1)>>, List<(Bytes, Adt1)>), List<List<(Bytes, Adt1)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Adt1)>>, List<(Bytes, Adt1)>), List<List<(Bytes, Adt1)>>>(({
                            let c125: Fn11 = v0.clone();
                            c125.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m126 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f49(v0: List<List<(Bytes, Adt1)>>, v1: (Bytes, Adt1)) -> List<List<(Bytes, Adt1)>> {
    List::cons(List::cons(v1, List::<(Bytes, Adt1)>::nil()), v0)
}

pub fn f50(v0: Fn12, p133: u64, p134: List<List<(Bytes, Adt1)>>) -> R<(List<List<(Bytes, Adt1)>>, bool)> {
    let m132 = (p133, p134);
    {
        let mut m132 = m132;
        loop {
            let step = {
                let (v1, v2) = m132;
                let m129 = v1;
                if m129 == 0 {
                    Err::<(u64, List<List<(Bytes, Adt1)>>), R<(List<List<(Bytes, Adt1)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m129 - 1;
                    let m131 = {
                        let c130: Fn12 = v0.clone();
                        c130.apply(v2.clone())?
                    };
                    match m131 {
                        None => {
                            Err::<(u64, List<List<(Bytes, Adt1)>>), R<(List<List<(Bytes, Adt1)>>, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, List<List<(Bytes, Adt1)>>), R<(List<List<(Bytes, Adt1)>>, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m132 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f51(v0: List<List<(Bytes, Adt1)>>) -> R<Option<List<List<(Bytes, Adt1)>>>> {
    f75(v0)
}

pub fn f52(v0: Adt1) -> R<Bytes> {
    Ok(f2(f76(match v0.clone() {
        Adt1::C0(h135, _, _, _, _, _, _, _) => {
            h135
        }
    })?, f76(match v0 {
        Adt1::C0(_, h136, _, _, _, _, _, _) => {
            h136
        }
    })?))
}

pub fn f53(v0: Fn13, p140: List<(Bytes, Adt2)>, p141: List<(Bytes, Adt2)>) -> List<(Bytes, Adt2)> {
    let m139 = (p140, p141);
    {
        let mut m139 = m139;
        loop {
            let step = {
                let (v1, v2) = m139;
                let m137 = v2;
                match m137.uncons() {
                    None => {
                        Err::<(List<(Bytes, Adt2)>, List<(Bytes, Adt2)>), List<(Bytes, Adt2)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Adt2)>, List<(Bytes, Adt2)>), List<(Bytes, Adt2)>>(({
                            let c138: Fn13 = v0.clone();
                            c138.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m139 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f54(v0: List<(Bytes, Adt2)>, v1: (Bytes, Adt2)) -> List<(Bytes, Adt2)> {
    List::cons(v1, v0)
}

pub fn f55(v0: List<List<(Bytes, Adt2)>>) -> List<List<(Bytes, Adt2)>> {
    f77(Fn21::F78, List::<List<(Bytes, Adt2)>>::nil(), v0)
}

pub fn f56(v0: Fn14, p145: List<List<(Bytes, Adt2)>>, p146: List<(Bytes, Adt2)>) -> List<List<(Bytes, Adt2)>> {
    let m144 = (p145, p146);
    {
        let mut m144 = m144;
        loop {
            let step = {
                let (v1, v2) = m144;
                let m142 = v2;
                match m142.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Adt2)>>, List<(Bytes, Adt2)>), List<List<(Bytes, Adt2)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Adt2)>>, List<(Bytes, Adt2)>), List<List<(Bytes, Adt2)>>>(({
                            let c143: Fn14 = v0.clone();
                            c143.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m144 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f57(v0: List<List<(Bytes, Adt2)>>, v1: (Bytes, Adt2)) -> List<List<(Bytes, Adt2)>> {
    List::cons(List::cons(v1, List::<(Bytes, Adt2)>::nil()), v0)
}

pub fn f58(v0: Fn15, p151: u64, p152: List<List<(Bytes, Adt2)>>) -> R<(List<List<(Bytes, Adt2)>>, bool)> {
    let m150 = (p151, p152);
    {
        let mut m150 = m150;
        loop {
            let step = {
                let (v1, v2) = m150;
                let m147 = v1;
                if m147 == 0 {
                    Err::<(u64, List<List<(Bytes, Adt2)>>), R<(List<List<(Bytes, Adt2)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m147 - 1;
                    let m149 = {
                        let c148: Fn15 = v0.clone();
                        c148.apply(v2.clone())?
                    };
                    match m149 {
                        None => {
                            Err::<(u64, List<List<(Bytes, Adt2)>>), R<(List<List<(Bytes, Adt2)>>, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, List<List<(Bytes, Adt2)>>), R<(List<List<(Bytes, Adt2)>>, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m150 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f59(v0: List<List<(Bytes, Adt2)>>) -> R<Option<List<List<(Bytes, Adt2)>>>> {
    f79(v0)
}

pub fn f60(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m153 = f80(Fn22::F81(v1), None::<Bytes>, v0);
    match m153 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f61(v0: List<Bytes>) -> R<List<Bytes>> {
    f62(Fn16::F82, List::<Bytes>::nil(), v0)
}

pub fn f62(v0: Fn16, p157: List<Bytes>, p158: List<Bytes>) -> R<List<Bytes>> {
    let m156 = (p157, p158);
    {
        let mut m156 = m156;
        loop {
            let step = {
                let (v1, v2) = m156;
                let m154 = v2;
                match m154.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), R<List<Bytes>>>(({
                            let c155: Fn16 = v0.clone();
                            c155.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m156 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f63(v0: List<Bytes>, v1: Bytes) -> R<List<Bytes>> {
    Ok(List::cons(f15(v1)?, v0))
}

pub fn f64(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let m160 = f83(v0, {
        let (h159, _) = v1.clone();
        h159
    });
    match m160 {
        None => {
            Ok(None::<(u64, Bytes)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a162 = {
                    let (h161, _) = v1;
                    h161
                };
                let a163 = 1u64;
                nat_add(a162, a163)?
            }, f2({
                let (_, h164) = v1;
                h164
            }, f84({
                let v3: u8 = v2;
                let a209 = {
                    let a185 = {
                        let a173 = {
                            let a167 = {
                                let a165 = v3;
                                let a166 = 128u8;
                                fixed_u8::bit_and(a165, a166)
                            };
                            let a168 = 128u8;
                            if equal(a167, a168) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a174 = {
                            let a171 = {
                                let a169 = v3;
                                let a170 = 64u8;
                                fixed_u8::bit_and(a169, a170)
                            };
                            let a172 = 64u8;
                            if equal(a171, a172) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a173, a174)?
                    };
                    let a186 = {
                        let a183 = {
                            let a177 = {
                                let a175 = v3;
                                let a176 = 32u8;
                                fixed_u8::bit_and(a175, a176)
                            };
                            let a178 = 32u8;
                            if equal(a177, a178) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a184 = {
                            let a181 = {
                                let a179 = v3;
                                let a180 = 16u8;
                                fixed_u8::bit_and(a179, a180)
                            };
                            let a182 = 16u8;
                            if equal(a181, a182) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a183, a184)?
                    };
                    nat_add(a185, a186)?
                };
                let a210 = {
                    let a207 = {
                        let a195 = {
                            let a189 = {
                                let a187 = v3;
                                let a188 = 8u8;
                                fixed_u8::bit_and(a187, a188)
                            };
                            let a190 = 8u8;
                            if equal(a189, a190) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a196 = {
                            let a193 = {
                                let a191 = v3;
                                let a192 = 4u8;
                                fixed_u8::bit_and(a191, a192)
                            };
                            let a194 = 4u8;
                            if equal(a193, a194) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a195, a196)?
                    };
                    let a208 = {
                        let a205 = {
                            let a199 = {
                                let a197 = v3;
                                let a198 = 2u8;
                                fixed_u8::bit_and(a197, a198)
                            };
                            let a200 = 2u8;
                            if equal(a199, a200) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a206 = {
                            let a203 = {
                                let a201 = v3;
                                let a202 = 1u8;
                                fixed_u8::bit_and(a201, a202)
                            };
                            let a204 = 1u8;
                            if equal(a203, a204) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a205, a206)?
                    };
                    nat_add(a207, a208)?
                };
                nat_add(a209, a210)?
            })))))
        }
    }
}

pub fn f65(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let m214 = {
        let a212 = v0;
        let a213 = {
            let (h211, _) = v1.clone();
            h211
        };
        index_bytes(a212, a213)
    };
    match m214 {
        None => {
            Ok(None::<(u64, Bytes)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a216 = {
                    let (h215, _) = v1;
                    h215
                };
                let a217 = 1u64;
                nat_add(a216, a217)?
            }, {
                let a219 = {
                    let (_, h218) = v1;
                    h218
                };
                let a220 = f85(v2)?;
                append_bytes(a219, a220)
            })))
        }
    }
}

pub fn f66(v0: Fn17, p224: List<(Bytes, (Bytes, Bytes))>, p225: List<(Bytes, (Bytes, Bytes))>) -> List<(Bytes, (Bytes, Bytes))> {
    let m223 = (p224, p225);
    {
        let mut m223 = m223;
        loop {
            let step = {
                let (v1, v2) = m223;
                let m221 = v2;
                match m221.uncons() {
                    None => {
                        Err::<(List<(Bytes, (Bytes, Bytes))>, List<(Bytes, (Bytes, Bytes))>), List<(Bytes, (Bytes, Bytes))>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, (Bytes, Bytes))>, List<(Bytes, (Bytes, Bytes))>), List<(Bytes, (Bytes, Bytes))>>(({
                            let c222: Fn17 = v0.clone();
                            c222.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m223 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f67(v0: List<(Bytes, (Bytes, Bytes))>, v1: (Bytes, (Bytes, Bytes))) -> List<(Bytes, (Bytes, Bytes))> {
    List::cons(v1, v0)
}

pub fn f68(v0: List<List<(Bytes, (Bytes, Bytes))>>) -> List<List<(Bytes, (Bytes, Bytes))>> {
    f86(Fn23::F87, List::<List<(Bytes, (Bytes, Bytes))>>::nil(), v0)
}

pub fn f69(v0: Fn18, p229: List<List<(Bytes, (Bytes, Bytes))>>, p230: List<(Bytes, (Bytes, Bytes))>) -> List<List<(Bytes, (Bytes, Bytes))>> {
    let m228 = (p229, p230);
    {
        let mut m228 = m228;
        loop {
            let step = {
                let (v1, v2) = m228;
                let m226 = v2;
                match m226.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, (Bytes, Bytes))>>, List<(Bytes, (Bytes, Bytes))>), List<List<(Bytes, (Bytes, Bytes))>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, (Bytes, Bytes))>>, List<(Bytes, (Bytes, Bytes))>), List<List<(Bytes, (Bytes, Bytes))>>>(({
                            let c227: Fn18 = v0.clone();
                            c227.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m228 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f70(v0: List<List<(Bytes, (Bytes, Bytes))>>, v1: (Bytes, (Bytes, Bytes))) -> List<List<(Bytes, (Bytes, Bytes))>> {
    List::cons(List::cons(v1, List::<(Bytes, (Bytes, Bytes))>::nil()), v0)
}

pub fn f71(v0: Fn19, p235: u64, p236: List<List<(Bytes, (Bytes, Bytes))>>) -> R<(List<List<(Bytes, (Bytes, Bytes))>>, bool)> {
    let m234 = (p235, p236);
    {
        let mut m234 = m234;
        loop {
            let step = {
                let (v1, v2) = m234;
                let m231 = v1;
                if m231 == 0 {
                    Err::<(u64, List<List<(Bytes, (Bytes, Bytes))>>), R<(List<List<(Bytes, (Bytes, Bytes))>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m231 - 1;
                    let m233 = {
                        let c232: Fn19 = v0.clone();
                        c232.apply(v2.clone())?
                    };
                    match m233 {
                        None => {
                            Err::<(u64, List<List<(Bytes, (Bytes, Bytes))>>), R<(List<List<(Bytes, (Bytes, Bytes))>>, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, List<List<(Bytes, (Bytes, Bytes))>>), R<(List<List<(Bytes, (Bytes, Bytes))>>, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m234 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f72(v0: List<List<(Bytes, (Bytes, Bytes))>>) -> R<Option<List<List<(Bytes, (Bytes, Bytes))>>>> {
    f88(v0)
}

pub fn f73(v0: Fn20, p240: List<List<(Bytes, Adt1)>>, p241: List<List<(Bytes, Adt1)>>) -> List<List<(Bytes, Adt1)>> {
    let m239 = (p240, p241);
    {
        let mut m239 = m239;
        loop {
            let step = {
                let (v1, v2) = m239;
                let m237 = v2;
                match m237.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>), List<List<(Bytes, Adt1)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>), List<List<(Bytes, Adt1)>>>(({
                            let c238: Fn20 = v0.clone();
                            c238.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m239 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f74(v0: List<List<(Bytes, Adt1)>>, v1: List<(Bytes, Adt1)>) -> List<List<(Bytes, Adt1)>> {
    List::cons(v1, v0)
}

pub fn f75(v0: List<List<(Bytes, Adt1)>>) -> R<Option<List<List<(Bytes, Adt1)>>>> {
    let a243 = {
        let a242 = v0.clone();
        length_list(a242)
    };
    let a244 = 1u64;
    if nat_le(a243, a244) {
        Ok(None::<List<List<(Bytes, Adt1)>>>)
    } else {
        Ok(Some(f89(v0)?))
    }
}

pub fn f76(v0: Bytes) -> R<Bytes> {
    Ok(f2(f60(f90(v0, 0u8)?, Bytes::lit(&[0x00, 0x01])), Bytes::lit(&[0x00, 0x00])))
}

pub fn f77(v0: Fn21, p248: List<List<(Bytes, Adt2)>>, p249: List<List<(Bytes, Adt2)>>) -> List<List<(Bytes, Adt2)>> {
    let m247 = (p248, p249);
    {
        let mut m247 = m247;
        loop {
            let step = {
                let (v1, v2) = m247;
                let m245 = v2;
                match m245.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Adt2)>>, List<List<(Bytes, Adt2)>>), List<List<(Bytes, Adt2)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Adt2)>>, List<List<(Bytes, Adt2)>>), List<List<(Bytes, Adt2)>>>(({
                            let c246: Fn21 = v0.clone();
                            c246.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m247 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f78(v0: List<List<(Bytes, Adt2)>>, v1: List<(Bytes, Adt2)>) -> List<List<(Bytes, Adt2)>> {
    List::cons(v1, v0)
}

pub fn f79(v0: List<List<(Bytes, Adt2)>>) -> R<Option<List<List<(Bytes, Adt2)>>>> {
    let a251 = {
        let a250 = v0.clone();
        length_list(a250)
    };
    let a252 = 1u64;
    if nat_le(a251, a252) {
        Ok(None::<List<List<(Bytes, Adt2)>>>)
    } else {
        Ok(Some(f91(v0)?))
    }
}

pub fn f80(v0: Fn22, p256: Option<Bytes>, p257: List<Bytes>) -> Option<Bytes> {
    let m255 = (p256, p257);
    {
        let mut m255 = m255;
        loop {
            let step = {
                let (v1, v2) = m255;
                let m253 = v2;
                match m253.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(({
                            let c254: Fn22 = v0.clone();
                            c254.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m255 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f81(v0: Bytes, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m258 = v1;
    match m258 {
        None => {
            Some(v2)
        }
        Some(v3) => {
            Some({
                let a261 = {
                    let a259 = v3;
                    let a260 = v0;
                    append_bytes(a259, a260)
                };
                let a262 = v2;
                append_bytes(a261, a262)
            })
        }
    }
}

pub fn f82(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f83(v0: Bytes, v1: u64) -> Option<u8> {
    let a263 = v0;
    let a264 = v1;
    index_bytes(a263, a264)
}

pub fn f84(v0: u64) -> Bytes {
    let a265 = v0;
    let a266 = 34u64;
    if nat_eq(a265, a266) {
        Bytes::lit(&[0x5c, 0x22])
    } else {
        let a267 = v0;
        let a268 = 92u64;
        if nat_eq(a267, a268) {
            Bytes::lit(&[0x5c, 0x5c])
        } else {
            let a269 = v0;
            let a270 = 32u64;
            if if nat_lt(a269, a270) {
                true
            } else {
                let a271 = v0;
                let a272 = 127u64;
                nat_eq(a271, a272)
            } {
                f2(f2(Bytes::lit(&[0x5c, 0x75, 0x30, 0x30]), f92({
                    let a273 = v0;
                    let a274 = 16u64;
                    let a275 = 0u64;
                    nat_quot(a273, a274, a275)
                })), f92({
                    let a276 = v0;
                    let a277 = 16u64;
                    let a278 = 0u64;
                    nat_rem(a276, a277, a278)
                }))
            } else {
                f93(f94(), v0)
            }
        }
    }
}

pub fn f85(v0: u8) -> R<Bytes> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a323 = {
            let a299 = {
                let a287 = {
                    let a281 = {
                        let a279 = v1;
                        let a280 = 128u8;
                        fixed_u8::bit_and(a279, a280)
                    };
                    let a282 = 128u8;
                    if equal(a281, a282) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a288 = {
                    let a285 = {
                        let a283 = v1;
                        let a284 = 64u8;
                        fixed_u8::bit_and(a283, a284)
                    };
                    let a286 = 64u8;
                    if equal(a285, a286) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a287, a288)?
            };
            let a300 = {
                let a297 = {
                    let a291 = {
                        let a289 = v1;
                        let a290 = 32u8;
                        fixed_u8::bit_and(a289, a290)
                    };
                    let a292 = 32u8;
                    if equal(a291, a292) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a298 = {
                    let a295 = {
                        let a293 = v1;
                        let a294 = 16u8;
                        fixed_u8::bit_and(a293, a294)
                    };
                    let a296 = 16u8;
                    if equal(a295, a296) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a297, a298)?
            };
            nat_add(a299, a300)?
        };
        let a324 = {
            let a321 = {
                let a309 = {
                    let a303 = {
                        let a301 = v1;
                        let a302 = 8u8;
                        fixed_u8::bit_and(a301, a302)
                    };
                    let a304 = 8u8;
                    if equal(a303, a304) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a310 = {
                    let a307 = {
                        let a305 = v1;
                        let a306 = 4u8;
                        fixed_u8::bit_and(a305, a306)
                    };
                    let a308 = 4u8;
                    if equal(a307, a308) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a309, a310)?
            };
            let a322 = {
                let a319 = {
                    let a313 = {
                        let a311 = v1;
                        let a312 = 2u8;
                        fixed_u8::bit_and(a311, a312)
                    };
                    let a314 = 2u8;
                    if equal(a313, a314) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a320 = {
                    let a317 = {
                        let a315 = v1;
                        let a316 = 1u8;
                        fixed_u8::bit_and(a315, a316)
                    };
                    let a318 = 1u8;
                    if equal(a317, a318) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a319, a320)?
            };
            nat_add(a321, a322)?
        };
        nat_add(a323, a324)?
    };
    let a331 = f93(f95(), {
        let a325 = v2;
        let a326 = 16u64;
        let a327 = 0u64;
        nat_quot(a325, a326, a327)
    });
    let a332 = f93(f95(), {
        let a328 = v2;
        let a329 = 16u64;
        let a330 = 0u64;
        nat_rem(a328, a329, a330)
    });
    Ok(append_bytes(a331, a332))
}

pub fn f86(v0: Fn23, p336: List<List<(Bytes, (Bytes, Bytes))>>, p337: List<List<(Bytes, (Bytes, Bytes))>>) -> List<List<(Bytes, (Bytes, Bytes))>> {
    let m335 = (p336, p337);
    {
        let mut m335 = m335;
        loop {
            let step = {
                let (v1, v2) = m335;
                let m333 = v2;
                match m333.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, (Bytes, Bytes))>>, List<List<(Bytes, (Bytes, Bytes))>>), List<List<(Bytes, (Bytes, Bytes))>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, (Bytes, Bytes))>>, List<List<(Bytes, (Bytes, Bytes))>>), List<List<(Bytes, (Bytes, Bytes))>>>(({
                            let c334: Fn23 = v0.clone();
                            c334.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m335 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f87(v0: List<List<(Bytes, (Bytes, Bytes))>>, v1: List<(Bytes, (Bytes, Bytes))>) -> List<List<(Bytes, (Bytes, Bytes))>> {
    List::cons(v1, v0)
}

pub fn f88(v0: List<List<(Bytes, (Bytes, Bytes))>>) -> R<Option<List<List<(Bytes, (Bytes, Bytes))>>>> {
    let a339 = {
        let a338 = v0.clone();
        length_list(a338)
    };
    let a340 = 1u64;
    if nat_le(a339, a340) {
        Ok(None::<List<List<(Bytes, (Bytes, Bytes))>>>)
    } else {
        Ok(Some(f96(v0)?))
    }
}

pub fn f89(v0: List<List<(Bytes, Adt1)>>) -> R<List<List<(Bytes, Adt1)>>> {
    Ok(f47({
        let (_, h341) = {
            let (h342, _) = f97(Fn24::F98, {
                let a344 = {
                    let a343 = v0.clone();
                    length_list(a343)
                };
                let a345 = 1u64;
                nat_add(a344, a345)?
            }, (v0, List::<List<(Bytes, Adt1)>>::nil()))?;
            h342
        };
        h341
    }))
}

pub fn f90(v0: Bytes, v1: u8) -> R<List<Bytes>> {
    let v2: u64 = {
        let a346 = v0.clone();
        length_bytes(a346)
    };
    let v3: (u64, (u64, List<Bytes>)) = {
        let (h347, _) = f99(Fn25::F100(v1, v0.clone()), v2, (0u64, (0u64, List::<Bytes>::nil())))?;
        h347
    };
    let v4: u64 = {
        let (h348, _) = {
            let (_, h349) = v3.clone();
            h349
        };
        h348
    };
    f101(List::cons(f102(v0, v4, {
        let a350 = v2;
        let a351 = v4;
        nat_sub(a350, a351)
    }), {
        let (_, h352) = {
            let (_, h353) = v3;
            h353
        };
        h352
    }))
}

pub fn f91(v0: List<List<(Bytes, Adt2)>>) -> R<List<List<(Bytes, Adt2)>>> {
    Ok(f55({
        let (_, h354) = {
            let (h355, _) = f103(Fn26::F104, {
                let a357 = {
                    let a356 = v0.clone();
                    length_list(a356)
                };
                let a358 = 1u64;
                nat_add(a357, a358)?
            }, (v0, List::<List<(Bytes, Adt2)>>::nil()))?;
            h355
        };
        h354
    }))
}

pub fn f92(v0: u64) -> Bytes {
    f93(Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46]), v0)
}

pub fn f93(v0: Bytes, v1: u64) -> Bytes {
    let m362 = {
        let a359 = v0;
        let a360 = v1;
        let a361 = 1u64;
        slice_bytes(a359, a360, a361)
    };
    match m362 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f94() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f95() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66])
}

pub fn f96(v0: List<List<(Bytes, (Bytes, Bytes))>>) -> R<List<List<(Bytes, (Bytes, Bytes))>>> {
    Ok(f68({
        let (_, h363) = {
            let (h364, _) = f105(Fn27::F106, {
                let a366 = {
                    let a365 = v0.clone();
                    length_list(a365)
                };
                let a367 = 1u64;
                nat_add(a366, a367)?
            }, (v0, List::<List<(Bytes, (Bytes, Bytes))>>::nil()))?;
            h364
        };
        h363
    }))
}

pub fn f97(v0: Fn24, p372: u64, p373: (List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>)) -> R<((List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>), bool)> {
    let m371 = (p372, p373);
    {
        let mut m371 = m371;
        loop {
            let step = {
                let (v1, v2) = m371;
                let m368 = v1;
                if m368 == 0 {
                    Err::<(u64, (List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>)), R<((List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m368 - 1;
                    let m370 = {
                        let c369: Fn24 = v0.clone();
                        c369.apply(v2.clone())?
                    };
                    match m370 {
                        None => {
                            Err::<(u64, (List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>)), R<((List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>)), R<((List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m371 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f98(v0: (List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>)) -> R<Option<(List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>)>> {
    f107(v0)
}

pub fn f99(v0: Fn25, p378: u64, p379: (u64, (u64, List<Bytes>))) -> R<((u64, (u64, List<Bytes>)), bool)> {
    let m377 = (p378, p379);
    {
        let mut m377 = m377;
        loop {
            let step = {
                let (v1, v2) = m377;
                let m374 = v1;
                if m374 == 0 {
                    Err::<(u64, (u64, (u64, List<Bytes>))), R<((u64, (u64, List<Bytes>)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m374 - 1;
                    let m376 = {
                        let c375: Fn25 = v0.clone();
                        c375.apply(v2.clone())?
                    };
                    match m376 {
                        None => {
                            Err::<(u64, (u64, (u64, List<Bytes>))), R<((u64, (u64, List<Bytes>)), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (u64, (u64, List<Bytes>))), R<((u64, (u64, List<Bytes>)), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m377 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f100(v0: u8, v1: Bytes, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f108(v1, v0, v2)
}

pub fn f101(v0: List<Bytes>) -> R<List<Bytes>> {
    f62(Fn16::F109, List::<Bytes>::nil(), v0)
}

pub fn f102(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m383 = {
        let a380 = v0;
        let a381 = v1;
        let a382 = v2;
        slice_bytes(a380, a381, a382)
    };
    match m383 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f103(v0: Fn26, p388: u64, p389: (List<List<(Bytes, Adt2)>>, List<List<(Bytes, Adt2)>>)) -> R<((List<List<(Bytes, Adt2)>>, List<List<(Bytes, Adt2)>>), bool)> {
    let m387 = (p388, p389);
    {
        let mut m387 = m387;
        loop {
            let step = {
                let (v1, v2) = m387;
                let m384 = v1;
                if m384 == 0 {
                    Err::<(u64, (List<List<(Bytes, Adt2)>>, List<List<(Bytes, Adt2)>>)), R<((List<List<(Bytes, Adt2)>>, List<List<(Bytes, Adt2)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m384 - 1;
                    let m386 = {
                        let c385: Fn26 = v0.clone();
                        c385.apply(v2.clone())?
                    };
                    match m386 {
                        None => {
                            Err::<(u64, (List<List<(Bytes, Adt2)>>, List<List<(Bytes, Adt2)>>)), R<((List<List<(Bytes, Adt2)>>, List<List<(Bytes, Adt2)>>), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (List<List<(Bytes, Adt2)>>, List<List<(Bytes, Adt2)>>)), R<((List<List<(Bytes, Adt2)>>, List<List<(Bytes, Adt2)>>), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m387 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f104(v0: (List<List<(Bytes, Adt2)>>, List<List<(Bytes, Adt2)>>)) -> R<Option<(List<List<(Bytes, Adt2)>>, List<List<(Bytes, Adt2)>>)>> {
    f110(v0)
}

pub fn f105(v0: Fn27, p394: u64, p395: (List<List<(Bytes, (Bytes, Bytes))>>, List<List<(Bytes, (Bytes, Bytes))>>)) -> R<((List<List<(Bytes, (Bytes, Bytes))>>, List<List<(Bytes, (Bytes, Bytes))>>), bool)> {
    let m393 = (p394, p395);
    {
        let mut m393 = m393;
        loop {
            let step = {
                let (v1, v2) = m393;
                let m390 = v1;
                if m390 == 0 {
                    Err::<(u64, (List<List<(Bytes, (Bytes, Bytes))>>, List<List<(Bytes, (Bytes, Bytes))>>)), R<((List<List<(Bytes, (Bytes, Bytes))>>, List<List<(Bytes, (Bytes, Bytes))>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m390 - 1;
                    let m392 = {
                        let c391: Fn27 = v0.clone();
                        c391.apply(v2.clone())?
                    };
                    match m392 {
                        None => {
                            Err::<(u64, (List<List<(Bytes, (Bytes, Bytes))>>, List<List<(Bytes, (Bytes, Bytes))>>)), R<((List<List<(Bytes, (Bytes, Bytes))>>, List<List<(Bytes, (Bytes, Bytes))>>), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (List<List<(Bytes, (Bytes, Bytes))>>, List<List<(Bytes, (Bytes, Bytes))>>)), R<((List<List<(Bytes, (Bytes, Bytes))>>, List<List<(Bytes, (Bytes, Bytes))>>), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m393 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f106(v0: (List<List<(Bytes, (Bytes, Bytes))>>, List<List<(Bytes, (Bytes, Bytes))>>)) -> R<Option<(List<List<(Bytes, (Bytes, Bytes))>>, List<List<(Bytes, (Bytes, Bytes))>>)>> {
    f111(v0)
}

pub fn f107(v0: (List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>)) -> R<Option<(List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>)>> {
    let v1: List<List<(Bytes, Adt1)>> = {
        let (h396, _) = v0;
        h396
    };
    let v2: List<List<(Bytes, Adt1)>> = {
        let (_, h397) = v0;
        h397
    };
    let m398 = v1;
    match m398.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>)>)
        }
        Some((v3, v4)) => {
            let m399 = v4;
            match m399.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Adt1)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f112(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f108(v0: Bytes, v1: u8, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v3: u64 = {
        let (h400, _) = v2.clone();
        h400
    };
    let v4: u64 = {
        let (h401, _) = {
            let (_, h402) = v2.clone();
            h402
        };
        h401
    };
    let v5: List<Bytes> = {
        let (_, h403) = {
            let (_, h404) = v2;
            h404
        };
        h403
    };
    let m405 = f83(v0.clone(), v3);
    match m405 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v6) => {
            let a406 = v6;
            let a407 = v1;
            if equal(a406, a407) {
                Ok(Some(({
                    let a408 = v3;
                    let a409 = 1u64;
                    nat_add(a408, a409)?
                }, ({
                    let a410 = v3;
                    let a411 = 1u64;
                    nat_add(a410, a411)?
                }, List::cons(f102(v0, v4, {
                    let a412 = v3;
                    let a413 = v4;
                    nat_sub(a412, a413)
                }), v5)))))
            } else {
                Ok(Some(({
                    let a414 = v3;
                    let a415 = 1u64;
                    nat_add(a414, a415)?
                }, (v4, v5))))
            }
        }
    }
}

pub fn f109(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f110(v0: (List<List<(Bytes, Adt2)>>, List<List<(Bytes, Adt2)>>)) -> R<Option<(List<List<(Bytes, Adt2)>>, List<List<(Bytes, Adt2)>>)>> {
    let v1: List<List<(Bytes, Adt2)>> = {
        let (h416, _) = v0;
        h416
    };
    let v2: List<List<(Bytes, Adt2)>> = {
        let (_, h417) = v0;
        h417
    };
    let m418 = v1;
    match m418.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Adt2)>>, List<List<(Bytes, Adt2)>>)>)
        }
        Some((v3, v4)) => {
            let m419 = v4;
            match m419.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Adt2)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f113(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f111(v0: (List<List<(Bytes, (Bytes, Bytes))>>, List<List<(Bytes, (Bytes, Bytes))>>)) -> R<Option<(List<List<(Bytes, (Bytes, Bytes))>>, List<List<(Bytes, (Bytes, Bytes))>>)>> {
    let v1: List<List<(Bytes, (Bytes, Bytes))>> = {
        let (h420, _) = v0;
        h420
    };
    let v2: List<List<(Bytes, (Bytes, Bytes))>> = {
        let (_, h421) = v0;
        h421
    };
    let m422 = v1;
    match m422.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, (Bytes, Bytes))>>, List<List<(Bytes, (Bytes, Bytes))>>)>)
        }
        Some((v3, v4)) => {
            let m423 = v4;
            match m423.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, (Bytes, Bytes))>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f114(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f112(v0: List<(Bytes, Adt1)>, v1: List<(Bytes, Adt1)>) -> R<List<(Bytes, Adt1)>> {
    let v2: u64 = {
        let a428 = {
            let a426 = {
                let a424 = v0.clone();
                length_list(a424)
            };
            let a427 = {
                let a425 = v1.clone();
                length_list(a425)
            };
            nat_add(a426, a427)?
        };
        let a429 = 1u64;
        nat_add(a428, a429)?
    };
    Ok(f23({
        let (_, h430) = {
            let (_, h431) = {
                let (h432, _) = f115(Fn28::F116, v2, (v0, (v1, List::<(Bytes, Adt1)>::nil())));
                h432
            };
            h431
        };
        h430
    }))
}

pub fn f113(v0: List<(Bytes, Adt2)>, v1: List<(Bytes, Adt2)>) -> R<List<(Bytes, Adt2)>> {
    let v2: u64 = {
        let a437 = {
            let a435 = {
                let a433 = v0.clone();
                length_list(a433)
            };
            let a436 = {
                let a434 = v1.clone();
                length_list(a434)
            };
            nat_add(a435, a436)?
        };
        let a438 = 1u64;
        nat_add(a437, a438)?
    };
    Ok(f30({
        let (_, h439) = {
            let (_, h440) = {
                let (h441, _) = f117(Fn29::F118, v2, (v0, (v1, List::<(Bytes, Adt2)>::nil())));
                h441
            };
            h440
        };
        h439
    }))
}

pub fn f114(v0: List<(Bytes, (Bytes, Bytes))>, v1: List<(Bytes, (Bytes, Bytes))>) -> R<List<(Bytes, (Bytes, Bytes))>> {
    let v2: u64 = {
        let a446 = {
            let a444 = {
                let a442 = v0.clone();
                length_list(a442)
            };
            let a445 = {
                let a443 = v1.clone();
                length_list(a443)
            };
            nat_add(a444, a445)?
        };
        let a447 = 1u64;
        nat_add(a446, a447)?
    };
    Ok(f41({
        let (_, h448) = {
            let (_, h449) = {
                let (h450, _) = f119(Fn30::F120, v2, (v0, (v1, List::<(Bytes, (Bytes, Bytes))>::nil())));
                h450
            };
            h449
        };
        h448
    }))
}

pub fn f115(v0: Fn28, p455: u64, p456: (List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>))) -> ((List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>)), bool) {
    let m454 = (p455, p456);
    {
        let mut m454 = m454;
        loop {
            let step = {
                let (v1, v2) = m454;
                let m451 = v1;
                if m451 == 0 {
                    Err::<(u64, (List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>))), ((List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>)), bool)>((v2, false))
                } else {
                    let v3 = m451 - 1;
                    let m453 = {
                        let c452: Fn28 = v0.clone();
                        c452.apply(v2.clone())
                    };
                    match m453 {
                        None => {
                            Err::<(u64, (List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>))), ((List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>)), bool)>((v2, true))
                        }
                        Some(v4) => {
                            Ok::<(u64, (List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>))), ((List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>)), bool)>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m454 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f116(v0: (List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>))) -> Option<(List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>))> {
    f121(v0)
}

pub fn f117(v0: Fn29, p461: u64, p462: (List<(Bytes, Adt2)>, (List<(Bytes, Adt2)>, List<(Bytes, Adt2)>))) -> ((List<(Bytes, Adt2)>, (List<(Bytes, Adt2)>, List<(Bytes, Adt2)>)), bool) {
    let m460 = (p461, p462);
    {
        let mut m460 = m460;
        loop {
            let step = {
                let (v1, v2) = m460;
                let m457 = v1;
                if m457 == 0 {
                    Err::<(u64, (List<(Bytes, Adt2)>, (List<(Bytes, Adt2)>, List<(Bytes, Adt2)>))), ((List<(Bytes, Adt2)>, (List<(Bytes, Adt2)>, List<(Bytes, Adt2)>)), bool)>((v2, false))
                } else {
                    let v3 = m457 - 1;
                    let m459 = {
                        let c458: Fn29 = v0.clone();
                        c458.apply(v2.clone())
                    };
                    match m459 {
                        None => {
                            Err::<(u64, (List<(Bytes, Adt2)>, (List<(Bytes, Adt2)>, List<(Bytes, Adt2)>))), ((List<(Bytes, Adt2)>, (List<(Bytes, Adt2)>, List<(Bytes, Adt2)>)), bool)>((v2, true))
                        }
                        Some(v4) => {
                            Ok::<(u64, (List<(Bytes, Adt2)>, (List<(Bytes, Adt2)>, List<(Bytes, Adt2)>))), ((List<(Bytes, Adt2)>, (List<(Bytes, Adt2)>, List<(Bytes, Adt2)>)), bool)>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m460 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f118(v0: (List<(Bytes, Adt2)>, (List<(Bytes, Adt2)>, List<(Bytes, Adt2)>))) -> Option<(List<(Bytes, Adt2)>, (List<(Bytes, Adt2)>, List<(Bytes, Adt2)>))> {
    f122(v0)
}

pub fn f119(v0: Fn30, p467: u64, p468: (List<(Bytes, (Bytes, Bytes))>, (List<(Bytes, (Bytes, Bytes))>, List<(Bytes, (Bytes, Bytes))>))) -> ((List<(Bytes, (Bytes, Bytes))>, (List<(Bytes, (Bytes, Bytes))>, List<(Bytes, (Bytes, Bytes))>)), bool) {
    let m466 = (p467, p468);
    {
        let mut m466 = m466;
        loop {
            let step = {
                let (v1, v2) = m466;
                let m463 = v1;
                if m463 == 0 {
                    Err::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<(Bytes, (Bytes, Bytes))>, List<(Bytes, (Bytes, Bytes))>))), ((List<(Bytes, (Bytes, Bytes))>, (List<(Bytes, (Bytes, Bytes))>, List<(Bytes, (Bytes, Bytes))>)), bool)>((v2, false))
                } else {
                    let v3 = m463 - 1;
                    let m465 = {
                        let c464: Fn30 = v0.clone();
                        c464.apply(v2.clone())
                    };
                    match m465 {
                        None => {
                            Err::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<(Bytes, (Bytes, Bytes))>, List<(Bytes, (Bytes, Bytes))>))), ((List<(Bytes, (Bytes, Bytes))>, (List<(Bytes, (Bytes, Bytes))>, List<(Bytes, (Bytes, Bytes))>)), bool)>((v2, true))
                        }
                        Some(v4) => {
                            Ok::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<(Bytes, (Bytes, Bytes))>, List<(Bytes, (Bytes, Bytes))>))), ((List<(Bytes, (Bytes, Bytes))>, (List<(Bytes, (Bytes, Bytes))>, List<(Bytes, (Bytes, Bytes))>)), bool)>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m466 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f120(v0: (List<(Bytes, (Bytes, Bytes))>, (List<(Bytes, (Bytes, Bytes))>, List<(Bytes, (Bytes, Bytes))>))) -> Option<(List<(Bytes, (Bytes, Bytes))>, (List<(Bytes, (Bytes, Bytes))>, List<(Bytes, (Bytes, Bytes))>))> {
    f123(v0)
}

pub fn f121(v0: (List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>))) -> Option<(List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>))> {
    let v1: List<(Bytes, Adt1)> = {
        let (h469, _) = v0.clone();
        h469
    };
    let v2: List<(Bytes, Adt1)> = {
        let (h470, _) = {
            let (_, h471) = v0.clone();
            h471
        };
        h470
    };
    let v3: List<(Bytes, Adt1)> = {
        let (_, h472) = {
            let (_, h473) = v0;
            h473
        };
        h472
    };
    let m474 = v1.clone();
    match m474.uncons() {
        None => {
            let m475 = v2;
            match m475.uncons() {
                None => {
                    None::<(List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Adt1)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m476 = v2.clone();
            match m476.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Adt1)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f124({
                        let (h477, _) = v8.clone();
                        h477
                    }, {
                        let (h478, _) = v6.clone();
                        h478
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

pub fn f122(v0: (List<(Bytes, Adt2)>, (List<(Bytes, Adt2)>, List<(Bytes, Adt2)>))) -> Option<(List<(Bytes, Adt2)>, (List<(Bytes, Adt2)>, List<(Bytes, Adt2)>))> {
    let v1: List<(Bytes, Adt2)> = {
        let (h479, _) = v0.clone();
        h479
    };
    let v2: List<(Bytes, Adt2)> = {
        let (h480, _) = {
            let (_, h481) = v0.clone();
            h481
        };
        h480
    };
    let v3: List<(Bytes, Adt2)> = {
        let (_, h482) = {
            let (_, h483) = v0;
            h483
        };
        h482
    };
    let m484 = v1.clone();
    match m484.uncons() {
        None => {
            let m485 = v2;
            match m485.uncons() {
                None => {
                    None::<(List<(Bytes, Adt2)>, (List<(Bytes, Adt2)>, List<(Bytes, Adt2)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Adt2)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m486 = v2.clone();
            match m486.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Adt2)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f124({
                        let (h487, _) = v8.clone();
                        h487
                    }, {
                        let (h488, _) = v6.clone();
                        h488
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

pub fn f123(v0: (List<(Bytes, (Bytes, Bytes))>, (List<(Bytes, (Bytes, Bytes))>, List<(Bytes, (Bytes, Bytes))>))) -> Option<(List<(Bytes, (Bytes, Bytes))>, (List<(Bytes, (Bytes, Bytes))>, List<(Bytes, (Bytes, Bytes))>))> {
    let v1: List<(Bytes, (Bytes, Bytes))> = {
        let (h489, _) = v0.clone();
        h489
    };
    let v2: List<(Bytes, (Bytes, Bytes))> = {
        let (h490, _) = {
            let (_, h491) = v0.clone();
            h491
        };
        h490
    };
    let v3: List<(Bytes, (Bytes, Bytes))> = {
        let (_, h492) = {
            let (_, h493) = v0;
            h493
        };
        h492
    };
    let m494 = v1.clone();
    match m494.uncons() {
        None => {
            let m495 = v2;
            match m495.uncons() {
                None => {
                    None::<(List<(Bytes, (Bytes, Bytes))>, (List<(Bytes, (Bytes, Bytes))>, List<(Bytes, (Bytes, Bytes))>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, (Bytes, Bytes))>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m496 = v2.clone();
            match m496.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, (Bytes, Bytes))>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f124({
                        let (h497, _) = v8.clone();
                        h497
                    }, {
                        let (h498, _) = v6.clone();
                        h498
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

pub fn f124(v0: Bytes, v1: Bytes) -> bool {
    let a503 = {
        let a499 = v0;
        let a500 = v1;
        compare_bytes(a499, a500)
    };
    let a504 = {
        let a501 = Bytes::lit(&[]);
        let a502 = Bytes::lit(&[0x01]);
        compare_bytes(a501, a502)
    };
    equal(a503, a504)
}

pub fn run(p0: Adt0) -> R<Bytes> {
    f0(p0)
}
