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
    C0,
    C1(Adt1),
    C2(Bytes, Bytes, Adt1),
    C3(Bytes),
    C4(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
    C5(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
    C6(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
    C7(Bytes, std::rc::Rc<Adt0>, std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
    C8(Bytes),
    C9,
    C10(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
    C11(u64, Bytes, std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(u64),
    C1(Bytes),
    C2(std::rc::Rc<Adt1>),
    C3(std::rc::Rc<Adt1>),
    C4(std::rc::Rc<Adt1>, std::rc::Rc<Adt1>),
    C5,
    C6(std::rc::Rc<Adt1>, std::rc::Rc<Adt1>),
}

#[derive(Clone)]
pub enum Adt2 {
    C0(Bytes, Bytes, u64, Adt0, bool),
}

#[derive(Clone)]
pub enum Adt3 {
    C0(Adt0, List<Option<Adt0>>),
}

#[derive(Clone)]
pub enum Adt4 {
    C0,
    C1(std::rc::Rc<Adt4>),
}

#[derive(Clone)]
pub enum Adt5 {
    C0(Adt0, Option<Bytes>, Adt6),
}

#[derive(Clone)]
pub enum Adt6 {
    C0(List<(Bytes, Adt0)>, u64, List<Bytes>, u64, bool, List<Adt2>, u64),
}

#[derive(Clone)]
pub enum Adt7 {
    C0(Adt0, Adt6),
}

#[derive(Clone)]
pub enum Adt8 {
    C0(Option<Bytes>, Adt6),
}

#[derive(Clone)]
pub enum Adt9 {
    C0(u64, Bytes, Adt0),
}

#[derive(Clone)]
pub enum Adt10 {
    C0(List<Adt9>, Adt6),
}

#[derive(Clone)]
pub enum Adt11 {
    C0(List<Adt9>, Adt0),
}

#[derive(Clone)]
pub enum Adt12 {
    C0(Adt0, List<(Bytes, Bytes)>, bool, Adt6),
}

#[derive(Clone)]
pub enum Fn0 {
    F6,
}

impl Fn0 {
    pub fn apply(&self, p0: Adt4) -> Option<Adt4> {
        match self {
            Fn0::F6 => f6(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F35(Bytes, Bytes),
    F37(Bytes, Bytes),
}

impl Fn1 {
    pub fn apply(&self, p0: Option<Adt2>, p1: Adt2) -> Option<Adt2> {
        match self {
            Fn1::F35(k0, k1) => f35(k0.clone(), k1.clone(), p0, p1),
            Fn1::F37(k0, k1) => f37(k0.clone(), k1.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F40(Bytes),
    F92(Bytes),
}

impl Fn2 {
    pub fn apply(&self, p0: Option<Adt0>, p1: (Bytes, Adt0)) -> Option<Adt0> {
        match self {
            Fn2::F40(k0) => f40(k0.clone(), p0, p1),
            Fn2::F92(k0) => f92(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F55,
    F109,
}

impl Fn3 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn3::F55 => f55(p0, p1),
            Fn3::F109 => f109(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F73,
    F108(Bytes),
}

impl Fn4 {
    pub fn apply(&self, p0: bool, p1: Bytes) -> R<bool> {
        match self {
            Fn4::F73 => f73(p0, p1),
            Fn4::F108(k0) => Ok(f108(k0.clone(), p0, p1)),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F76(u64, Bytes),
}

impl Fn5 {
    pub fn apply(&self, p0: (u64, Bytes)) -> Option<(u64, Bytes)> {
        match self {
            Fn5::F76(k0, k1) => f76(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F86,
}

impl Fn6 {
    pub fn apply(&self, p0: Adt0, p1: Adt9) -> Adt0 {
        match self {
            Fn6::F86 => f86(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F100(Bytes),
}

impl Fn7 {
    pub fn apply(&self, p0: Option<Bytes>, p1: (Bytes, Bytes)) -> Option<Bytes> {
        match self {
            Fn7::F100(k0) => f100(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F103,
    F105,
    F106,
}

impl Fn8 {
    pub fn apply(&self, p0: List<Adt9>, p1: Adt9) -> List<Adt9> {
        match self {
            Fn8::F103 => f103(p0, p1),
            Fn8::F105 => f105(p0, p1),
            Fn8::F106 => f106(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F116,
}

impl Fn9 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn9::F116 => f116(p0, p1),
        }
    }
}

pub fn f0(v0: Adt0, v1: List<Adt2>, v2: u64, v3: u64) -> R<Result<Adt3, Bytes>> {
    let v4: Adt4 = f1(v3);
    let v5: Adt5 = f2(v4.clone(), f3(v1, v2), v0, List::<(Bytes, Adt0)>::nil())?;
    f4(v4, v5)
}

pub fn f1(v0: u64) -> Adt4 {
    let (h1, _) = f5(Fn0::F6, v0, Adt4::C0);
    h1
}

pub fn f2(v0: Adt4, v1: Adt6, v2: Adt0, v3: List<(Bytes, Adt0)>) -> R<Adt5> {
    let m2 = v0;
    match m2 {
        Adt4::C0 => {
            Ok(f7(f8(v1), Bytes::lit(&[])))
        }
        Adt4::C1(r7) => {
            let v4 = (*r7).clone();
            let v5: Adt5 = f9(v4.clone(), v1, v2.clone(), v3)?;
            let m4 = match v5.clone() {
                Adt5::C0(_, h3, _) => {
                    h3
                }
            };
            match m4 {
                None => {
                    f10(v4, match v5.clone() {
                        Adt5::C0(_, _, h5) => {
                            h5
                        }
                    }, v2, match v5 {
                        Adt5::C0(h6, _, _) => {
                            h6
                        }
                    })
                }
                Some(_) => {
                    Ok(v5)
                }
            }
        }
    }
}

pub fn f3(v0: List<Adt2>, v1: u64) -> Adt6 {
    Adt6::C0(List::<(Bytes, Adt0)>::nil(), 0u64, List::<Bytes>::nil(), 0u64, false, v0, v1)
}

pub fn f4(v0: Adt4, v1: Adt5) -> R<Result<Adt3, Bytes>> {
    let m10 = match v1.clone() {
        Adt5::C0(_, _, h9) => {
            h9
        }
    };
    if match m10 {
        Adt6::C0(_, _, _, _, h8, _, _) => {
            h8
        }
    } {
        Ok(Err::<Adt3, Bytes>(f11(Str::lit("the checker's bound on nesting is exhausted"))))
    } else {
        let m12 = match v1.clone() {
            Adt5::C0(_, h11, _) => {
                h11
            }
        };
        match m12 {
            None => {
                f12(v0, match v1.clone() {
                    Adt5::C0(_, _, h13) => {
                        h13
                    }
                }, match v1 {
                    Adt5::C0(h14, _, _) => {
                        h14
                    }
                })
            }
            Some(v2) => {
                Ok(Err::<Adt3, Bytes>(v2))
            }
        }
    }
}

pub fn f5(v0: Fn0, p19: u64, p20: Adt4) -> (Adt4, bool) {
    let m18 = (p19, p20);
    {
        let mut m18 = m18;
        loop {
            let step = {
                let (v1, v2) = m18;
                let m15 = v1;
                if m15 == 0 {
                    Err::<(u64, Adt4), (Adt4, bool)>((v2, false))
                } else {
                    let v3 = m15 - 1;
                    let m17 = {
                        let c16: Fn0 = v0.clone();
                        c16.apply(v2.clone())
                    };
                    match m17 {
                        None => {
                            Err::<(u64, Adt4), (Adt4, bool)>((v2, true))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt4), (Adt4, bool)>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m18 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f6(v0: Adt4) -> Option<Adt4> {
    Some(Adt4::C1(std::rc::Rc::new(v0)))
}

pub fn f7(v0: Adt6, v1: Bytes) -> Adt5 {
    Adt5::C0(Adt0::C0, Some(v1), v0)
}

pub fn f8(v0: Adt6) -> Adt6 {
    f13(v0.clone(), match v0.clone() {
        Adt6::C0(h21, _, _, _, _, _, _) => {
            h21
        }
    }, match v0.clone() {
        Adt6::C0(_, h22, _, _, _, _, _) => {
            h22
        }
    }, match v0.clone() {
        Adt6::C0(_, _, h23, _, _, _, _) => {
            h23
        }
    }, match v0 {
        Adt6::C0(_, _, _, h24, _, _, _) => {
            h24
        }
    }, true)
}

pub fn f9(v0: Adt4, v1: Adt6, v2: Adt0, v3: List<(Bytes, Adt0)>) -> R<Adt5> {
    let m25 = v0;
    match m25 {
        Adt4::C0 => {
            Ok(f7(f8(v1), Bytes::lit(&[])))
        }
        Adt4::C1(r66) => {
            let v4 = (*r66).clone();
            let m26 = v2.clone();
            match m26 {
                Adt0::C0 => {
                    Ok(f14(Adt0::C1(Adt1::C0(1u64)), v1))
                }
                Adt0::C1(v5) => {
                    Ok(f14(Adt0::C1(Adt1::C2(std::rc::Rc::new(v5))), v1))
                }
                Adt0::C2(v6, v7, _) => {
                    let v9: (u64, bool) = f15(v1.clone(), v6.clone(), v7.clone());
                    let v10: u64 = {
                        let (h27, _) = v9;
                        h27
                    };
                    let a28 = v10;
                    let a29 = 0u64;
                    if nat_eq(a28, a29) {
                        Ok(f7(v1, f16(f16(f16(f16(f11(Str::lit("`")), v6), f11(Str::lit("::"))), v7), f11(Str::lit("` does not resolve")))))
                    } else {
                        let a30 = v10;
                        let a31 = 1u64;
                        if nat_eq(a30, a31) {
                            f17(v1)
                        } else {
                            let a32 = v10;
                            let a33 = 2u64;
                            if nat_eq(a32, a33) {
                                Ok(f7(v1, f16(f16(f16(f16(f11(Str::lit("`")), v6), f11(Str::lit("::"))), v7), f11(Str::lit("` has no signature and cannot appear in an expression")))))
                            } else {
                                Ok(f14(f18(v1.clone(), v6, v7), v1))
                            }
                        }
                    }
                }
                Adt0::C3(v11) => {
                    if f19(v11.clone())? {
                        f17(v1)
                    } else {
                        let m34 = f20(v3, v11.clone());
                        match m34 {
                            None => {
                                Ok(f7(v1, f16(f16(f11(Str::lit("local `")), v11), f11(Str::lit("` is not in scope")))))
                            }
                            Some(v12) => {
                                Ok(f14(v12, v1))
                            }
                        }
                    }
                }
                Adt0::C4(r57, r58) => {
                    let v15 = (*r57).clone();
                    let v16 = (*r58).clone();
                    let v17: u64 = f22(v15.clone());
                    let a37 = v17;
                    let a38 = 2u64;
                    if if nat_lt(a37, a38) {
                        true
                    } else {
                        let a39 = v17;
                        let a40 = 8u64;
                        nat_eq(a39, a40)
                    } {
                        let v18: Bytes = f23(v2)?;
                        let v19: Bytes = f23(v15)?;
                        Ok(f7(v1, f16(f16(f16(v18, f11(Str::lit(" applies "))), v19), f11(Str::lit(", which is not a function")))))
                    } else {
                        let v20: Adt5 = f9(v4.clone(), v1, v15.clone(), v3.clone())?;
                        let m42 = match v20.clone() {
                            Adt5::C0(_, h41, _) => {
                                h41
                            }
                        };
                        match m42 {
                            None => {
                                let v22: List<Adt0> = f24(v4.clone(), v16);
                                f25(v4, match v20.clone() {
                                    Adt5::C0(_, _, h43) => {
                                        h43
                                    }
                                }, v2, v15, v22.clone(), {
                                    let a44 = v22;
                                    length_list(a44)
                                }, match v20 {
                                    Adt5::C0(h45, _, _) => {
                                        h45
                                    }
                                }, 0u64, v3)
                            }
                            Some(_) => {
                                Ok(v20)
                            }
                        }
                    }
                }
                Adt0::C5(r59, r60) => {
                    let v23 = (*r59).clone();
                    let v24 = (*r60).clone();
                    f26(v4, v1, v23.clone(), v24, v3, false, v23)
                }
                Adt0::C6(r61, r62) => {
                    let v25 = (*r61).clone();
                    let v26 = (*r62).clone();
                    f26(v4, v1, v25.clone(), v26, v3, true, v25)
                }
                Adt0::C7(v27, r63, r64, r65) => {
                    let v28 = (*r63).clone();
                    let v29 = (*r64).clone();
                    let v30 = (*r65).clone();
                    let v31: Adt5 = f2(v4.clone(), v1, v28.clone(), v3.clone())?;
                    let m47 = match v31.clone() {
                        Adt5::C0(_, h46, _) => {
                            h46
                        }
                    };
                    match m47 {
                        None => {
                            let v33: Adt5 = f9(v4.clone(), match v31 {
                                Adt5::C0(_, _, h48) => {
                                    h48
                                }
                            }, v29, v3.clone())?;
                            let m50 = match v33.clone() {
                                Adt5::C0(_, h49, _) => {
                                    h49
                                }
                            };
                            match m50 {
                                None => {
                                    let v35: Adt8 = f27(v4.clone(), match v33.clone() {
                                        Adt5::C0(_, _, h51) => {
                                            h51
                                        }
                                    }, v28.clone(), match v33 {
                                        Adt5::C0(h52, _, _) => {
                                            h52
                                        }
                                    })?;
                                    let m54 = match v35.clone() {
                                        Adt8::C0(h53, _) => {
                                            h53
                                        }
                                    };
                                    match m54 {
                                        None => {
                                            f9(v4, match v35 {
                                                Adt8::C0(_, h56) => {
                                                    h56
                                                }
                                            }, v30, List::cons((v27, v28), v3))
                                        }
                                        Some(v36) => {
                                            Ok(f7(match v35 {
                                                Adt8::C0(_, h55) => {
                                                    h55
                                                }
                                            }, v36))
                                        }
                                    }
                                }
                                Some(_) => {
                                    Ok(v33)
                                }
                            }
                        }
                        Some(_) => {
                            Ok(v31)
                        }
                    }
                }
                Adt0::C8(_) => {
                    let v14: Adt7 = f21(v1)?;
                    Ok(f14(match v14.clone() {
                        Adt7::C0(h35, _) => {
                            h35
                        }
                    }, match v14 {
                        Adt7::C0(_, h36) => {
                            h36
                        }
                    }))
                }
                Adt0::C9 => {
                    Ok(f14(Adt0::C0, v1))
                }
                Adt0::C10(_, _) => {
                    Ok(f14(Adt0::C0, v1))
                }
                Adt0::C11(_, _, _, _) => {
                    Ok(f14(Adt0::C0, v1))
                }
            }
        }
    }
}

pub fn f10(v0: Adt4, v1: Adt6, v2: Adt0, v3: Adt0) -> R<Adt5> {
    let v4: Adt7 = f28(v0.clone(), v1, v3)?;
    let v5: Adt0 = match v4.clone() {
        Adt7::C0(h67, _) => {
            h67
        }
    };
    let v6: Adt6 = match v4 {
        Adt7::C0(_, h68) => {
            h68
        }
    };
    let v7: u64 = f22(v2.clone());
    let a69 = v7;
    let a70 = 8u64;
    if if nat_eq(a69, a70) {
        true
    } else {
        let a71 = v7;
        let a72 = 6u64;
        nat_eq(a71, a72)
    } {
        let v8: Bytes = f23(v2)?;
        Ok(f7(v6, f16(v8, f11(Str::lit(" is a term, not a type")))))
    } else {
        let a73 = f22(v5.clone());
        let a74 = 2u64;
        if nat_lt(a73, a74) {
            Ok(f14(v5, v6))
        } else {
            let m75 = f29(v5.clone())?;
            match m75 {
                None => {
                    if f30(v0, v6.clone(), v5.clone()) {
                        Ok(f14(v5, v6))
                    } else {
                        let v10: Bytes = f23(v2)?;
                        let v11: Bytes = f23(v5)?;
                        Ok(f7(v6, f16(f16(f16(v10, f11(Str::lit(" is not a type: its type is "))), v11), Bytes::lit(&[]))))
                    }
                }
                Some(_) => {
                    Ok(f14(v5, v6))
                }
            }
        }
    }
}

pub fn f11(v0: Str) -> Bytes {
    let a76 = v0;
    utf8_encode(a76)
}

pub fn f12(v0: Adt4, v1: Adt6, v2: Adt0) -> R<Result<Adt3, Bytes>> {
    let v3: Adt7 = f28(v0.clone(), v1.clone(), v2)?;
    let v4: (List<Option<Adt0>>, Adt6) = f31(v0, match v3.clone() {
        Adt7::C0(_, h77) => {
            h77
        }
    }, f32(match v1 {
        Adt6::C0(_, _, h78, _, _, _, _) => {
            h78
        }
    }))?;
    let m80 = f33(v4.clone());
    if match m80 {
        Adt6::C0(_, _, _, _, h79, _, _) => {
            h79
        }
    } {
        Ok(Err::<Adt3, Bytes>(f11(Str::lit("the checker's bound on nesting is exhausted"))))
    } else {
        Ok(Ok::<Adt3, Bytes>(Adt3::C0(match v3 {
            Adt7::C0(h81, _) => {
                h81
            }
        }, {
            let (h82, _) = v4;
            h82
        })))
    }
}

pub fn f13(v0: Adt6, v1: List<(Bytes, Adt0)>, v2: u64, v3: List<Bytes>, v4: u64, v5: bool) -> Adt6 {
    Adt6::C0(v1, v2, v3, v4, v5, match v0.clone() {
        Adt6::C0(_, _, _, _, _, h83, _) => {
            h83
        }
    }, match v0 {
        Adt6::C0(_, _, _, _, _, _, h84) => {
            h84
        }
    })
}

pub fn f14(v0: Adt0, v1: Adt6) -> Adt5 {
    Adt5::C0(v0, None::<Bytes>, v1)
}

pub fn f15(v0: Adt6, v1: Bytes, v2: Bytes) -> (u64, bool) {
    let m86 = f34(Fn1::F35(v2, v1), None::<Adt2>, match v0.clone() {
        Adt6::C0(_, _, _, _, _, h85, _) => {
            h85
        }
    });
    match m86 {
        None => {
            (match v0 {
                Adt6::C0(_, _, _, _, _, _, h89) => {
                    h89
                }
            }, false)
        }
        Some(v3) => {
            (match v3.clone() {
                Adt2::C0(_, _, h87, _, _) => {
                    h87
                }
            }, match v3 {
                Adt2::C0(_, _, _, _, h88) => {
                    h88
                }
            })
        }
    }
}

pub fn f16(v0: Bytes, v1: Bytes) -> Bytes {
    let a90 = v0;
    let a91 = v1;
    append_bytes(a90, a91)
}

pub fn f17(v0: Adt6) -> R<Adt5> {
    let v1: Adt7 = f36(v0)?;
    Ok(f14(match v1.clone() {
        Adt7::C0(h92, _) => {
            h92
        }
    }, match v1 {
        Adt7::C0(_, h93) => {
            h93
        }
    }))
}

pub fn f18(v0: Adt6, v1: Bytes, v2: Bytes) -> Adt0 {
    let m95 = f34(Fn1::F37(v2, v1), None::<Adt2>, match v0 {
        Adt6::C0(_, _, _, _, _, h94, _) => {
            h94
        }
    });
    match m95 {
        None => {
            Adt0::C0
        }
        Some(v3) => {
            match v3 {
                Adt2::C0(_, _, _, h96, _) => {
                    h96
                }
            }
        }
    }
}

pub fn f19(v0: Bytes) -> R<bool> {
    let a97 = f38(v0, 0u64)?;
    let a98 = 63u64;
    Ok(nat_eq(a97, a98))
}

pub fn f20(v0: List<(Bytes, Adt0)>, v1: Bytes) -> Option<Adt0> {
    f39(Fn2::F40(v1), None::<Adt0>, v0)
}

pub fn f21(v0: Adt6) -> R<Adt7> {
    let v1: Adt7 = f36(v0.clone())?;
    let v2: Adt6 = match v1.clone() {
        Adt7::C0(_, h99) => {
            h99
        }
    };
    Ok(Adt7::C0(match v1 {
        Adt7::C0(h100, _) => {
            h100
        }
    }, f13(v2.clone(), match v2.clone() {
        Adt6::C0(h101, _, _, _, _, _, _) => {
            h101
        }
    }, match v2.clone() {
        Adt6::C0(_, h102, _, _, _, _, _) => {
            h102
        }
    }, List::cons(f16(f11(Str::lit("?")), f41(match v0 {
        Adt6::C0(_, h103, _, _, _, _, _) => {
            h103
        }
    })), match v2.clone() {
        Adt6::C0(_, _, h104, _, _, _, _) => {
            h104
        }
    }), match v2.clone() {
        Adt6::C0(_, _, _, h105, _, _, _) => {
            h105
        }
    }, match v2 {
        Adt6::C0(_, _, _, _, h106, _, _) => {
            h106
        }
    })))
}

pub fn f22(v0: Adt0) -> u64 {
    let m107 = v0;
    match m107 {
        Adt0::C0 => {
            0u64
        }
        Adt0::C1(_) => {
            1u64
        }
        Adt0::C2(_, _, _) => {
            2u64
        }
        Adt0::C3(_) => {
            3u64
        }
        Adt0::C4(_, _) => {
            4u64
        }
        Adt0::C5(_, _) => {
            5u64
        }
        Adt0::C6(_, _) => {
            6u64
        }
        Adt0::C7(_, _, _, _) => {
            7u64
        }
        Adt0::C8(_) => {
            8u64
        }
        Adt0::C9 => {
            9u64
        }
        Adt0::C10(_, _) => {
            9u64
        }
        Adt0::C11(_, _, _, _) => {
            9u64
        }
    }
}

pub fn f23(v0: Adt0) -> R<Bytes> {
    Ok(f16(f16(f11(Str::lit("`")), f42(v0, false)?), f11(Str::lit("`"))))
}

pub fn f24(v0: Adt4, v1: Adt0) -> List<Adt0> {
    let m108 = v0;
    match m108 {
        Adt4::C0 => {
            List::<Adt0>::nil()
        }
        Adt4::C1(r112) => {
            let v2 = (*r112).clone();
            let m109 = v1;
            match m109 {
                Adt0::C0 => {
                    List::<Adt0>::nil()
                }
                Adt0::C1(_) => {
                    List::<Adt0>::nil()
                }
                Adt0::C2(_, _, _) => {
                    List::<Adt0>::nil()
                }
                Adt0::C3(_) => {
                    List::<Adt0>::nil()
                }
                Adt0::C4(_, _) => {
                    List::<Adt0>::nil()
                }
                Adt0::C5(_, _) => {
                    List::<Adt0>::nil()
                }
                Adt0::C6(_, _) => {
                    List::<Adt0>::nil()
                }
                Adt0::C7(_, _, _, _) => {
                    List::<Adt0>::nil()
                }
                Adt0::C8(_) => {
                    List::<Adt0>::nil()
                }
                Adt0::C9 => {
                    List::<Adt0>::nil()
                }
                Adt0::C10(r110, r111) => {
                    let v3 = (*r110).clone();
                    let v4 = (*r111).clone();
                    List::cons(v3, f24(v2, v4))
                }
                Adt0::C11(_, _, _, _) => {
                    List::<Adt0>::nil()
                }
            }
        }
    }
}

pub fn f25(v0: Adt4, v1: Adt6, v2: Adt0, v3: Adt0, v4: List<Adt0>, v5: u64, v6: Adt0, v7: u64, v8: List<(Bytes, Adt0)>) -> R<Adt5> {
    let m113 = v0;
    match m113 {
        Adt4::C0 => {
            Ok(f7(f8(v1), Bytes::lit(&[])))
        }
        Adt4::C1(r120) => {
            let v9 = (*r120).clone();
            let m114 = v4.clone();
            match m114.uncons() {
                None => {
                    Ok(f14(v6, v1))
                }
                Some((_, _)) => {
                    let v12: Adt7 = f28(v9.clone(), v1, v6)?;
                    let v13: Adt0 = match v12.clone() {
                        Adt7::C0(h115, _) => {
                            h115
                        }
                    };
                    let v14: Adt6 = match v12 {
                        Adt7::C0(_, h116) => {
                            h116
                        }
                    };
                    let m117 = v13.clone();
                    match m117 {
                        Adt0::C0 => {
                            f45(v9, v14, v2, v3, v4, v5, v7, v13, v8)
                        }
                        Adt0::C1(_) => {
                            f45(v9, v14, v2, v3, v4, v5, v7, v13, v8)
                        }
                        Adt0::C2(_, _, _) => {
                            f45(v9, v14, v2, v3, v4, v5, v7, v13, v8)
                        }
                        Adt0::C3(_) => {
                            f45(v9, v14, v2, v3, v4, v5, v7, v13, v8)
                        }
                        Adt0::C4(_, _) => {
                            f45(v9, v14, v2, v3, v4, v5, v7, v13, v8)
                        }
                        Adt0::C5(r118, r119) => {
                            let v15 = (*r118).clone();
                            let v16 = (*r119).clone();
                            f43(v9.clone(), v14, v2, v3, f44(v9, v15), v16, v4, v5, v7, v8)
                        }
                        Adt0::C6(_, _) => {
                            f45(v9, v14, v2, v3, v4, v5, v7, v13, v8)
                        }
                        Adt0::C7(_, _, _, _) => {
                            f45(v9, v14, v2, v3, v4, v5, v7, v13, v8)
                        }
                        Adt0::C8(_) => {
                            f45(v9, v14, v2, v3, v4, v5, v7, v13, v8)
                        }
                        Adt0::C9 => {
                            f45(v9, v14, v2, v3, v4, v5, v7, v13, v8)
                        }
                        Adt0::C10(_, _) => {
                            f45(v9, v14, v2, v3, v4, v5, v7, v13, v8)
                        }
                        Adt0::C11(_, _, _, _) => {
                            f45(v9, v14, v2, v3, v4, v5, v7, v13, v8)
                        }
                    }
                }
            }
        }
    }
}

pub fn f26(v0: Adt4, v1: Adt6, v2: Adt0, v3: Adt0, v4: List<(Bytes, Adt0)>, v5: bool, v6: Adt0) -> R<Adt5> {
    let m121 = v0;
    match m121 {
        Adt4::C0 => {
            Ok(f7(f8(v1), Bytes::lit(&[])))
        }
        Adt4::C1(r135) => {
            let v7 = (*r135).clone();
            let m122 = v2;
            match m122 {
                Adt0::C0 => {
                    f2(v7, v1, v3, v4)
                }
                Adt0::C1(_) => {
                    f2(v7, v1, v3, v4)
                }
                Adt0::C2(_, _, _) => {
                    f2(v7, v1, v3, v4)
                }
                Adt0::C3(_) => {
                    f2(v7, v1, v3, v4)
                }
                Adt0::C4(_, _) => {
                    f2(v7, v1, v3, v4)
                }
                Adt0::C5(_, _) => {
                    f2(v7, v1, v3, v4)
                }
                Adt0::C6(_, _) => {
                    f2(v7, v1, v3, v4)
                }
                Adt0::C7(_, _, _, _) => {
                    f2(v7, v1, v3, v4)
                }
                Adt0::C8(_) => {
                    f2(v7, v1, v3, v4)
                }
                Adt0::C9 => {
                    if v5 {
                        let v15: Adt5 = f9(v7, v1, v3, v4)?;
                        let m130 = match v15.clone() {
                            Adt5::C0(_, h129, _) => {
                                h129
                            }
                        };
                        match m130 {
                            None => {
                                Ok(f14(Adt0::C5(std::rc::Rc::new(v6), std::rc::Rc::new(match v15.clone() {
                                    Adt5::C0(h131, _, _) => {
                                        h131
                                    }
                                })), match v15 {
                                    Adt5::C0(_, _, h132) => {
                                        h132
                                    }
                                }))
                            }
                            Some(_) => {
                                Ok(v15)
                            }
                        }
                    } else {
                        f2(v7, v1, v3, v4)
                    }
                }
                Adt0::C10(_, _) => {
                    f2(v7, v1, v3, v4)
                }
                Adt0::C11(_, v9, r133, r134) => {
                    let v10 = (*r133).clone();
                    let v11 = (*r134).clone();
                    let v12: Adt5 = f2(v7.clone(), v1, v10.clone(), v4.clone())?;
                    let m124 = match v12.clone() {
                        Adt5::C0(_, h123, _) => {
                            h123
                        }
                    };
                    match m124 {
                        None => {
                            let v14: Adt5 = f26(v7, match v12 {
                                Adt5::C0(_, _, h125) => {
                                    h125
                                }
                            }, v11, v3, List::cons((v9, v10), v4), v5, v6)?;
                            Ok(Adt5::C0(match v14.clone() {
                                Adt5::C0(h126, _, _) => {
                                    h126
                                }
                            }, match v14.clone() {
                                Adt5::C0(_, h127, _) => {
                                    h127
                                }
                            }, match v14 {
                                Adt5::C0(_, _, h128) => {
                                    h128
                                }
                            }))
                        }
                        Some(_) => {
                            Ok(v12)
                        }
                    }
                }
            }
        }
    }
}

pub fn f27(v0: Adt4, v1: Adt6, v2: Adt0, v3: Adt0) -> R<Adt8> {
    let m136 = v0;
    match m136 {
        Adt4::C0 => {
            Ok(f46(f8(v1)))
        }
        Adt4::C1(r147) => {
            let v4 = (*r147).clone();
            let v5: Adt7 = f28(v4.clone(), v1, v2)?;
            let v6: Adt7 = f28(v4.clone(), match v5.clone() {
                Adt7::C0(_, h137) => {
                    h137
                }
            }, v3)?;
            let v7: Adt0 = match v5 {
                Adt7::C0(h138, _) => {
                    h138
                }
            };
            let v8: Adt0 = match v6.clone() {
                Adt7::C0(h139, _) => {
                    h139
                }
            };
            let v9: Adt6 = match v6 {
                Adt7::C0(_, h140) => {
                    h140
                }
            };
            let m141 = f29(v7.clone())?;
            match m141 {
                None => {
                    let m146 = f29(v8.clone())?;
                    match m146 {
                        None => {
                            f49(v4, v9, v7, v8)
                        }
                        Some(v11) => {
                            Ok(f46(f48(v9, v11, v7)))
                        }
                    }
                }
                Some(v10) => {
                    let a142 = f47(v8.clone());
                    let a143 = v10.clone();
                    if equal(a142, a143) {
                        let a144 = f22(v8.clone());
                        let a145 = 3u64;
                        if nat_eq(a144, a145) {
                            Ok(f46(v9))
                        } else {
                            Ok(f46(f48(v9, v10, v8)))
                        }
                    } else {
                        Ok(f46(f48(v9, v10, v8)))
                    }
                }
            }
        }
    }
}

pub fn f28(v0: Adt4, v1: Adt6, v2: Adt0) -> R<Adt7> {
    let m149 = f50(v0, match v1.clone() {
        Adt6::C0(h148, _, _, _, _, _, _) => {
            h148
        }
    }, v2.clone())?;
    match m149 {
        None => {
            Ok(Adt7::C0(v2, f8(v1)))
        }
        Some(v3) => {
            Ok(Adt7::C0(v3, v1))
        }
    }
}

pub fn f29(v0: Adt0) -> R<Option<Bytes>> {
    let m150 = v0;
    match m150 {
        Adt0::C0 => {
            Ok(None::<Bytes>)
        }
        Adt0::C1(_) => {
            Ok(None::<Bytes>)
        }
        Adt0::C2(_, _, _) => {
            Ok(None::<Bytes>)
        }
        Adt0::C3(v1) => {
            if f19(v1.clone())? {
                Ok(Some(v1))
            } else {
                Ok(None::<Bytes>)
            }
        }
        Adt0::C4(_, _) => {
            Ok(None::<Bytes>)
        }
        Adt0::C5(_, _) => {
            Ok(None::<Bytes>)
        }
        Adt0::C6(_, _) => {
            Ok(None::<Bytes>)
        }
        Adt0::C7(_, _, _, _) => {
            Ok(None::<Bytes>)
        }
        Adt0::C8(_) => {
            Ok(None::<Bytes>)
        }
        Adt0::C9 => {
            Ok(None::<Bytes>)
        }
        Adt0::C10(_, _) => {
            Ok(None::<Bytes>)
        }
        Adt0::C11(_, _, _, _) => {
            Ok(None::<Bytes>)
        }
    }
}

pub fn f30(p156: Adt4, v1: Adt6, p157: Adt0) -> bool {
    let m155 = (p156, p157);
    {
        let mut m155 = m155;
        loop {
            let step = {
                let (v0, v2) = m155;
                let m151 = v0;
                match m151 {
                    Adt4::C0 => {
                        Err::<(Adt4, Adt0), bool>(false)
                    }
                    Adt4::C1(r154) => {
                        let v3 = (*r154).clone();
                        let m152 = v2;
                        match m152 {
                            Adt0::C0 => {
                                Err::<(Adt4, Adt0), bool>(false)
                            }
                            Adt0::C1(_) => {
                                Err::<(Adt4, Adt0), bool>(false)
                            }
                            Adt0::C2(v4, v5, _) => {
                                Err::<(Adt4, Adt0), bool>(f51(v1.clone(), v4, v5))
                            }
                            Adt0::C3(_) => {
                                Err::<(Adt4, Adt0), bool>(false)
                            }
                            Adt0::C4(r153, _) => {
                                let v7 = (*r153).clone();
                                Ok::<(Adt4, Adt0), bool>((v3, v7))
                            }
                            Adt0::C5(_, _) => {
                                Err::<(Adt4, Adt0), bool>(false)
                            }
                            Adt0::C6(_, _) => {
                                Err::<(Adt4, Adt0), bool>(false)
                            }
                            Adt0::C7(_, _, _, _) => {
                                Err::<(Adt4, Adt0), bool>(false)
                            }
                            Adt0::C8(_) => {
                                Err::<(Adt4, Adt0), bool>(false)
                            }
                            Adt0::C9 => {
                                Err::<(Adt4, Adt0), bool>(false)
                            }
                            Adt0::C10(_, _) => {
                                Err::<(Adt4, Adt0), bool>(false)
                            }
                            Adt0::C11(_, _, _, _) => {
                                Err::<(Adt4, Adt0), bool>(false)
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m155 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f31(v0: Adt4, v1: Adt6, v2: List<Bytes>) -> R<(List<Option<Adt0>>, Adt6)> {
    let m158 = v0;
    match m158 {
        Adt4::C0 => {
            Ok((List::<Option<Adt0>>::nil(), f8(v1)))
        }
        Adt4::C1(r166) => {
            let v3 = (*r166).clone();
            let m159 = v2;
            match m159.uncons() {
                None => {
                    Ok((List::<Option<Adt0>>::nil(), v1))
                }
                Some((v4, v5)) => {
                    let v6: Adt7 = f28(v3.clone(), v1, Adt0::C3(v4))?;
                    let v7: (bool, Adt6) = f52(v3.clone(), match v6.clone() {
                        Adt7::C0(_, h160) => {
                            h160
                        }
                    }, match v6.clone() {
                        Adt7::C0(h161, _) => {
                            h161
                        }
                    })?;
                    let v8: (List<Option<Adt0>>, Adt6) = f31(v3, {
                        let (_, h162) = v7.clone();
                        h162
                    }, v5)?;
                    Ok((List::cons(if f53(v7) {
                        None::<Adt0>
                    } else {
                        Some(match v6 {
                            Adt7::C0(h163, _) => {
                                h163
                            }
                        })
                    }, {
                        let (h164, _) = v8;
                        h164
                    }), {
                        let (_, h165) = v8;
                        h165
                    }))
                }
            }
        }
    }
}

pub fn f32(v0: List<Bytes>) -> List<Bytes> {
    f54(Fn3::F55, List::<Bytes>::nil(), v0)
}

pub fn f33(v0: (List<Option<Adt0>>, Adt6)) -> Adt6 {
    let (_, h167) = v0;
    h167
}

pub fn f34(v0: Fn1, p171: Option<Adt2>, p172: List<Adt2>) -> Option<Adt2> {
    let m170 = (p171, p172);
    {
        let mut m170 = m170;
        loop {
            let step = {
                let (v1, v2) = m170;
                let m168 = v2;
                match m168.uncons() {
                    None => {
                        Err::<(Option<Adt2>, List<Adt2>), Option<Adt2>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Adt2>, List<Adt2>), Option<Adt2>>(({
                            let c169: Fn1 = v0.clone();
                            c169.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m170 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f35(v0: Bytes, v1: Bytes, v2: Option<Adt2>, v3: Adt2) -> Option<Adt2> {
    let m173 = v2.clone();
    match m173 {
        None => {
            let a175 = match v3.clone() {
                Adt2::C0(h174, _, _, _, _) => {
                    h174
                }
            };
            let a176 = v1;
            if if equal(a175, a176) {
                let a178 = match v3.clone() {
                    Adt2::C0(_, h177, _, _, _) => {
                        h177
                    }
                };
                let a179 = v0;
                equal(a178, a179)
            } else {
                false
            } {
                Some(v3)
            } else {
                None::<Adt2>
            }
        }
        Some(_) => {
            v2
        }
    }
}

pub fn f36(v0: Adt6) -> R<Adt7> {
    Ok(Adt7::C0(Adt0::C3(f16(f11(Str::lit("?")), f41(match v0.clone() {
        Adt6::C0(_, h180, _, _, _, _, _) => {
            h180
        }
    }))), f13(v0.clone(), match v0.clone() {
        Adt6::C0(h181, _, _, _, _, _, _) => {
            h181
        }
    }, {
        let a183 = match v0.clone() {
            Adt6::C0(_, h182, _, _, _, _, _) => {
                h182
            }
        };
        let a184 = 1u64;
        nat_add(a183, a184)?
    }, match v0.clone() {
        Adt6::C0(_, _, h185, _, _, _, _) => {
            h185
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, h186, _, _, _) => {
            h186
        }
    }, match v0 {
        Adt6::C0(_, _, _, _, h187, _, _) => {
            h187
        }
    })))
}

pub fn f37(v0: Bytes, v1: Bytes, v2: Option<Adt2>, v3: Adt2) -> Option<Adt2> {
    let m188 = v2.clone();
    match m188 {
        None => {
            let a190 = match v3.clone() {
                Adt2::C0(h189, _, _, _, _) => {
                    h189
                }
            };
            let a191 = v1;
            if if equal(a190, a191) {
                let a193 = match v3.clone() {
                    Adt2::C0(_, h192, _, _, _) => {
                        h192
                    }
                };
                let a194 = v0;
                equal(a193, a194)
            } else {
                false
            } {
                Some(v3)
            } else {
                None::<Adt2>
            }
        }
        Some(_) => {
            v2
        }
    }
}

pub fn f38(v0: Bytes, v1: u64) -> R<u64> {
    let m195 = f56(v0, v1);
    match m195 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a240 = {
                let a216 = {
                    let a204 = {
                        let a198 = {
                            let a196 = v3;
                            let a197 = 128u8;
                            fixed_u8::bit_and(a196, a197)
                        };
                        let a199 = 128u8;
                        if equal(a198, a199) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a205 = {
                        let a202 = {
                            let a200 = v3;
                            let a201 = 64u8;
                            fixed_u8::bit_and(a200, a201)
                        };
                        let a203 = 64u8;
                        if equal(a202, a203) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a204, a205)?
                };
                let a217 = {
                    let a214 = {
                        let a208 = {
                            let a206 = v3;
                            let a207 = 32u8;
                            fixed_u8::bit_and(a206, a207)
                        };
                        let a209 = 32u8;
                        if equal(a208, a209) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a215 = {
                        let a212 = {
                            let a210 = v3;
                            let a211 = 16u8;
                            fixed_u8::bit_and(a210, a211)
                        };
                        let a213 = 16u8;
                        if equal(a212, a213) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a214, a215)?
                };
                nat_add(a216, a217)?
            };
            let a241 = {
                let a238 = {
                    let a226 = {
                        let a220 = {
                            let a218 = v3;
                            let a219 = 8u8;
                            fixed_u8::bit_and(a218, a219)
                        };
                        let a221 = 8u8;
                        if equal(a220, a221) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a227 = {
                        let a224 = {
                            let a222 = v3;
                            let a223 = 4u8;
                            fixed_u8::bit_and(a222, a223)
                        };
                        let a225 = 4u8;
                        if equal(a224, a225) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a226, a227)?
                };
                let a239 = {
                    let a236 = {
                        let a230 = {
                            let a228 = v3;
                            let a229 = 2u8;
                            fixed_u8::bit_and(a228, a229)
                        };
                        let a231 = 2u8;
                        if equal(a230, a231) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a237 = {
                        let a234 = {
                            let a232 = v3;
                            let a233 = 1u8;
                            fixed_u8::bit_and(a232, a233)
                        };
                        let a235 = 1u8;
                        if equal(a234, a235) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a236, a237)?
                };
                nat_add(a238, a239)?
            };
            nat_add(a240, a241)
        }
    }
}

pub fn f39(v0: Fn2, p245: Option<Adt0>, p246: List<(Bytes, Adt0)>) -> Option<Adt0> {
    let m244 = (p245, p246);
    {
        let mut m244 = m244;
        loop {
            let step = {
                let (v1, v2) = m244;
                let m242 = v2;
                match m242.uncons() {
                    None => {
                        Err::<(Option<Adt0>, List<(Bytes, Adt0)>), Option<Adt0>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Adt0>, List<(Bytes, Adt0)>), Option<Adt0>>(({
                            let c243: Fn2 = v0.clone();
                            c243.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m244 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f40(v0: Bytes, v1: Option<Adt0>, v2: (Bytes, Adt0)) -> Option<Adt0> {
    let m247 = v1.clone();
    match m247 {
        None => {
            let a249 = {
                let (h248, _) = v2;
                h248
            };
            let a250 = v0;
            if equal(a249, a250) {
                Some({
                    let (_, h251) = v2;
                    h251
                })
            } else {
                None::<Adt0>
            }
        }
        Some(_) => {
            v1
        }
    }
}

pub fn f41(v0: u64) -> Bytes {
    f57(f58(), 10u64, v0)
}

pub fn f42(v0: Adt0, v1: bool) -> R<Bytes> {
    let (h252, _) = f59(v0, v1, List::<(Bytes, Bytes)>::nil(), 0u64)?;
    Ok(h252)
}

pub fn f43(v0: Adt4, v1: Adt6, v2: Adt0, v3: Adt0, v4: List<Adt9>, v5: Adt0, v6: List<Adt0>, v7: u64, v8: u64, v9: List<(Bytes, Adt0)>) -> R<Adt5> {
    let m253 = v0;
    match m253 {
        Adt4::C0 => {
            Ok(f7(f8(v1), Bytes::lit(&[])))
        }
        Adt4::C1(r272) => {
            let v10 = (*r272).clone();
            let m254 = v4.clone();
            match m254.uncons() {
                None => {
                    f25(v10, v1, v2, v3, v6, v7, v5, v8, v9)
                }
                Some((v11, v12)) => {
                    let a256 = match v11.clone() {
                        Adt9::C0(h255, _, _) => {
                            h255
                        }
                    };
                    let a257 = 0u64;
                    if nat_eq(a256, a257) {
                        let m258 = v6;
                        match m258.uncons() {
                            None => {
                                Ok(f14(Adt0::C5(std::rc::Rc::new(f60(v4)), std::rc::Rc::new(v5)), v1))
                            }
                            Some((v13, v14)) => {
                                let v15: Adt5 = f9(v10.clone(), v1, v13.clone(), v9.clone())?;
                                let m260 = match v15.clone() {
                                    Adt5::C0(_, h259, _) => {
                                        h259
                                    }
                                };
                                match m260 {
                                    None => {
                                        let v17: Adt8 = f27(v10.clone(), match v15.clone() {
                                            Adt5::C0(_, _, h261) => {
                                                h261
                                            }
                                        }, match v11.clone() {
                                            Adt9::C0(_, _, h262) => {
                                                h262
                                            }
                                        }, match v15 {
                                            Adt5::C0(h263, _, _) => {
                                                h263
                                            }
                                        })?;
                                        let m265 = match v17.clone() {
                                            Adt8::C0(h264, _) => {
                                                h264
                                            }
                                        };
                                        match m265 {
                                            None => {
                                                f61(v10, match v17 {
                                                    Adt8::C0(_, h267) => {
                                                        h267
                                                    }
                                                }, v2, v3, v11, v13, v12, v5, v14, v7, {
                                                    let a268 = v8;
                                                    let a269 = 1u64;
                                                    nat_add(a268, a269)?
                                                }, v9)
                                            }
                                            Some(v18) => {
                                                Ok(f7(match v17 {
                                                    Adt8::C0(_, h266) => {
                                                        h266
                                                    }
                                                }, v18))
                                            }
                                        }
                                    }
                                    Some(_) => {
                                        Ok(v15)
                                    }
                                }
                            }
                        }
                    } else {
                        let v19: Adt7 = f36(v1)?;
                        f61(v10, match v19.clone() {
                            Adt7::C0(_, h270) => {
                                h270
                            }
                        }, v2, v3, v11, match v19 {
                            Adt7::C0(h271, _) => {
                                h271
                            }
                        }, v12, v5, v6, v7, v8, v9)
                    }
                }
            }
        }
    }
}

pub fn f44(v0: Adt4, v1: Adt0) -> List<Adt9> {
    let m273 = v0;
    match m273 {
        Adt4::C0 => {
            List::<Adt9>::nil()
        }
        Adt4::C1(r277) => {
            let v2 = (*r277).clone();
            let m274 = v1;
            match m274 {
                Adt0::C0 => {
                    List::<Adt9>::nil()
                }
                Adt0::C1(_) => {
                    List::<Adt9>::nil()
                }
                Adt0::C2(_, _, _) => {
                    List::<Adt9>::nil()
                }
                Adt0::C3(_) => {
                    List::<Adt9>::nil()
                }
                Adt0::C4(_, _) => {
                    List::<Adt9>::nil()
                }
                Adt0::C5(_, _) => {
                    List::<Adt9>::nil()
                }
                Adt0::C6(_, _) => {
                    List::<Adt9>::nil()
                }
                Adt0::C7(_, _, _, _) => {
                    List::<Adt9>::nil()
                }
                Adt0::C8(_) => {
                    List::<Adt9>::nil()
                }
                Adt0::C9 => {
                    List::<Adt9>::nil()
                }
                Adt0::C10(_, _) => {
                    List::<Adt9>::nil()
                }
                Adt0::C11(v3, v4, r275, r276) => {
                    let v5 = (*r275).clone();
                    let v6 = (*r276).clone();
                    List::cons(Adt9::C0(v3, v4, v5), f44(v2, v6))
                }
            }
        }
    }
}

pub fn f45(v0: Adt4, v1: Adt6, v2: Adt0, v3: Adt0, v4: List<Adt0>, v5: u64, v6: u64, v7: Adt0, v8: List<(Bytes, Adt0)>) -> R<Adt5> {
    let m278 = v0;
    match m278 {
        Adt4::C0 => {
            Ok(f7(f8(v1), Bytes::lit(&[])))
        }
        Adt4::C1(r280) => {
            let v9 = (*r280).clone();
            let m279 = f29(v7.clone())?;
            match m279 {
                None => {
                    if f30(v9.clone(), v1.clone(), v7.clone()) {
                        f62(v9, v1, v4, v8)
                    } else {
                        let v11: Bytes = f23(v2)?;
                        let v12: Bytes = f23(v3)?;
                        let v13: Bytes = f23(v7)?;
                        Ok(f7(v1, f16(f16(f16(f16(f16(f16(f16(f16(f16(v11, f11(Str::lit(" supplies "))), f41(v5)), f11(Str::lit(" explicit argument"))), f63(v5)), f11(Str::lit(" but "))), v12), f11(Str::lit(" accepts "))), f41(v6)), f16(f16(f11(Str::lit(": after them its type is ")), v13), f11(Str::lit(", not a function"))))))
                    }
                }
                Some(_) => {
                    f62(v9, v1, v4, v8)
                }
            }
        }
    }
}

pub fn f46(v0: Adt6) -> Adt8 {
    Adt8::C0(None::<Bytes>, v0)
}

pub fn f47(v0: Adt0) -> Bytes {
    let m281 = v0;
    match m281 {
        Adt0::C0 => {
            Bytes::lit(&[])
        }
        Adt0::C1(_) => {
            Bytes::lit(&[])
        }
        Adt0::C2(v1, v2, _) => {
            f16(f16(v1, f11(Str::lit("::"))), v2)
        }
        Adt0::C3(v4) => {
            v4
        }
        Adt0::C4(_, _) => {
            Bytes::lit(&[])
        }
        Adt0::C5(_, _) => {
            Bytes::lit(&[])
        }
        Adt0::C6(_, _) => {
            Bytes::lit(&[])
        }
        Adt0::C7(_, _, _, _) => {
            Bytes::lit(&[])
        }
        Adt0::C8(v5) => {
            v5
        }
        Adt0::C9 => {
            Bytes::lit(&[])
        }
        Adt0::C10(_, _) => {
            Bytes::lit(&[])
        }
        Adt0::C11(_, _, _, _) => {
            Bytes::lit(&[])
        }
    }
}

pub fn f48(v0: Adt6, v1: Bytes, v2: Adt0) -> Adt6 {
    f13(v0.clone(), List::cons((v1, v2), match v0.clone() {
        Adt6::C0(h282, _, _, _, _, _, _) => {
            h282
        }
    }), match v0.clone() {
        Adt6::C0(_, h283, _, _, _, _, _) => {
            h283
        }
    }, match v0.clone() {
        Adt6::C0(_, _, h284, _, _, _, _) => {
            h284
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, h285, _, _, _) => {
            h285
        }
    }, match v0 {
        Adt6::C0(_, _, _, _, h286, _, _) => {
            h286
        }
    })
}

pub fn f49(v0: Adt4, v1: Adt6, v2: Adt0, v3: Adt0) -> R<Adt8> {
    let m287 = v0;
    match m287 {
        Adt4::C0 => {
            Ok(f46(f8(v1)))
        }
        Adt4::C1(r305) => {
            let v4 = (*r305).clone();
            let v5: u64 = f64(v4.clone(), v2.clone(), v3.clone());
            let a288 = v5;
            let a289 = 0u64;
            if nat_eq(a288, a289) {
                Ok(f46(v1))
            } else {
                let a290 = v5;
                let a291 = 1u64;
                if nat_eq(a290, a291) {
                    if if f30(v4.clone(), v1.clone(), v2.clone()) {
                        true
                    } else {
                        f30(v4.clone(), v1.clone(), v3.clone())
                    } {
                        Ok(f46(v1))
                    } else {
                        let v6: (Adt0, Adt0) = f65(v2);
                        let v7: (Adt0, Adt0) = f65(v3);
                        let v8: Adt8 = f27(v4.clone(), v1, {
                            let (h292, _) = v6;
                            h292
                        }, {
                            let (h293, _) = v7;
                            h293
                        })?;
                        let m295 = match v8.clone() {
                            Adt8::C0(h294, _) => {
                                h294
                            }
                        };
                        match m295 {
                            None => {
                                f66(v4, match v8 {
                                    Adt8::C0(_, h296) => {
                                        h296
                                    }
                                }, {
                                    let (_, h297) = v6;
                                    h297
                                }, {
                                    let (_, h298) = v7;
                                    h298
                                })
                            }
                            Some(_) => {
                                Ok(v8)
                            }
                        }
                    }
                } else {
                    let a299 = v5;
                    let a300 = 2u64;
                    if nat_eq(a299, a300) {
                        f67(v4, v1, v2, v3)
                    } else {
                        if if f30(v4.clone(), v1.clone(), v2.clone()) {
                            true
                        } else {
                            f30(v4.clone(), v1.clone(), v3.clone())
                        } {
                            Ok(f46(v1))
                        } else {
                            let v10: (bool, Adt6) = f52(v4.clone(), v1, v2.clone())?;
                            if f53(v10.clone()) {
                                Ok(f46({
                                    let (_, h301) = v10;
                                    h301
                                }))
                            } else {
                                let v11: (bool, Adt6) = f52(v4, {
                                    let (_, h302) = v10;
                                    h302
                                }, v3.clone())?;
                                if f53(v11.clone()) {
                                    Ok(f46({
                                        let (_, h303) = v11;
                                        h303
                                    }))
                                } else {
                                    let v12: Bytes = f23(v2)?;
                                    let v13: Bytes = f23(v3)?;
                                    Ok(f68({
                                        let (_, h304) = v11;
                                        h304
                                    }, f16(f16(f16(f11(Str::lit("expected ")), v12), f11(Str::lit(", found "))), v13)))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f50(v0: Adt4, v1: List<(Bytes, Adt0)>, v2: Adt0) -> R<Option<Adt0>> {
    let m306 = v0;
    match m306 {
        Adt4::C0 => {
            Ok(None::<Adt0>)
        }
        Adt4::C1(r341) => {
            let v3 = (*r341).clone();
            let m307 = v2;
            match m307 {
                Adt0::C0 => {
                    Ok(Some(Adt0::C0))
                }
                Adt0::C1(v29) => {
                    Ok(Some(Adt0::C1(v29)))
                }
                Adt0::C2(v30, v31, v32) => {
                    Ok(Some(Adt0::C2(v30, v31, v32)))
                }
                Adt0::C3(v4) => {
                    if f19(v4.clone())? {
                        let m308 = f69(v1.clone(), v4.clone());
                        match m308 {
                            None => {
                                Ok(Some(Adt0::C3(v4)))
                            }
                            Some(v5) => {
                                f50(v3, v1, v5)
                            }
                        }
                    } else {
                        Ok(Some(Adt0::C3(v4)))
                    }
                }
                Adt0::C4(r328, r329) => {
                    let v6 = (*r328).clone();
                    let v7 = (*r329).clone();
                    let m309 = f70(f50(v3.clone(), v1.clone(), v6)?, f50(v3, v1, v7)?);
                    match m309 {
                        None => {
                            Ok(None::<Adt0>)
                        }
                        Some(v8) => {
                            Ok(Some(Adt0::C4(std::rc::Rc::new({
                                let (h310, _) = v8;
                                h310
                            }), std::rc::Rc::new({
                                let (_, h311) = v8;
                                h311
                            }))))
                        }
                    }
                }
                Adt0::C5(r330, r331) => {
                    let v12 = (*r330).clone();
                    let v13 = (*r331).clone();
                    let m315 = f70(f50(v3.clone(), v1.clone(), v12)?, f50(v3, v1, v13)?);
                    match m315 {
                        None => {
                            Ok(None::<Adt0>)
                        }
                        Some(v14) => {
                            Ok(Some(Adt0::C5(std::rc::Rc::new({
                                let (h316, _) = v14;
                                h316
                            }), std::rc::Rc::new({
                                let (_, h317) = v14;
                                h317
                            }))))
                        }
                    }
                }
                Adt0::C6(r332, r333) => {
                    let v15 = (*r332).clone();
                    let v16 = (*r333).clone();
                    let m318 = f70(f50(v3.clone(), v1.clone(), v15)?, f50(v3, v1, v16)?);
                    match m318 {
                        None => {
                            Ok(None::<Adt0>)
                        }
                        Some(v17) => {
                            Ok(Some(Adt0::C6(std::rc::Rc::new({
                                let (h319, _) = v17;
                                h319
                            }), std::rc::Rc::new({
                                let (_, h320) = v17;
                                h320
                            }))))
                        }
                    }
                }
                Adt0::C7(v23, r334, r335, r336) => {
                    let v24 = (*r334).clone();
                    let v25 = (*r335).clone();
                    let v26 = (*r336).clone();
                    let m324 = f70(f50(v3.clone(), v1.clone(), v24)?, f50(v3.clone(), v1.clone(), v25)?);
                    match m324 {
                        None => {
                            Ok(None::<Adt0>)
                        }
                        Some(v27) => {
                            let m325 = f50(v3, v1, v26)?;
                            match m325 {
                                None => {
                                    Ok(None::<Adt0>)
                                }
                                Some(v28) => {
                                    Ok(Some(Adt0::C7(v23, std::rc::Rc::new({
                                        let (h326, _) = v27;
                                        h326
                                    }), std::rc::Rc::new({
                                        let (_, h327) = v27;
                                        h327
                                    }), std::rc::Rc::new(v28))))
                                }
                            }
                        }
                    }
                }
                Adt0::C8(v33) => {
                    Ok(Some(Adt0::C8(v33)))
                }
                Adt0::C9 => {
                    Ok(Some(Adt0::C9))
                }
                Adt0::C10(r337, r338) => {
                    let v9 = (*r337).clone();
                    let v10 = (*r338).clone();
                    let m312 = f70(f50(v3.clone(), v1.clone(), v9)?, f50(v3, v1, v10)?);
                    match m312 {
                        None => {
                            Ok(None::<Adt0>)
                        }
                        Some(v11) => {
                            Ok(Some(Adt0::C10(std::rc::Rc::new({
                                let (h313, _) = v11;
                                h313
                            }), std::rc::Rc::new({
                                let (_, h314) = v11;
                                h314
                            }))))
                        }
                    }
                }
                Adt0::C11(v18, v19, r339, r340) => {
                    let v20 = (*r339).clone();
                    let v21 = (*r340).clone();
                    let m321 = f70(f50(v3.clone(), v1.clone(), v20)?, f50(v3, v1, v21)?);
                    match m321 {
                        None => {
                            Ok(None::<Adt0>)
                        }
                        Some(v22) => {
                            Ok(Some(Adt0::C11(v18, v19, std::rc::Rc::new({
                                let (h322, _) = v22;
                                h322
                            }), std::rc::Rc::new({
                                let (_, h323) = v22;
                                h323
                            }))))
                        }
                    }
                }
            }
        }
    }
}

pub fn f51(v0: Adt6, v1: Bytes, v2: Bytes) -> bool {
    let v3: (u64, bool) = f15(v0, v1, v2);
    let a343 = {
        let (h342, _) = v3;
        h342
    };
    let a344 = 1u64;
    if nat_eq(a343, a344) {
        true
    } else {
        let a346 = {
            let (h345, _) = v3;
            h345
        };
        let a347 = 3u64;
        if nat_eq(a346, a347) {
            let (_, h348) = v3;
            h348
        } else {
            false
        }
    }
}

pub fn f52(v0: Adt4, v1: Adt6, v2: Adt0) -> R<(bool, Adt6)> {
    let v3: Adt7 = f28(v0.clone(), v1, v2)?;
    let m350 = f71(v0, match v3.clone() {
        Adt7::C0(h349, _) => {
            h349
        }
    }, List::<Bytes>::nil())?;
    match m350 {
        None => {
            Ok((true, f8(match v3 {
                Adt7::C0(_, h352) => {
                    h352
                }
            })))
        }
        Some(v4) => {
            Ok((f72(Fn4::F73, false, v4)?, match v3 {
                Adt7::C0(_, h351) => {
                    h351
                }
            }))
        }
    }
}

pub fn f53(v0: (bool, Adt6)) -> bool {
    let (h353, _) = v0;
    h353
}

pub fn f54(v0: Fn3, p357: List<Bytes>, p358: List<Bytes>) -> List<Bytes> {
    let m356 = (p357, p358);
    {
        let mut m356 = m356;
        loop {
            let step = {
                let (v1, v2) = m356;
                let m354 = v2;
                match m354.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c355: Fn3 = v0.clone();
                            c355.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m356 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f55(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f56(v0: Bytes, v1: u64) -> Option<u8> {
    let a359 = v0;
    let a360 = v1;
    index_bytes(a359, a360)
}

pub fn f57(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let a361 = v2;
    let a362 = 0u64;
    if nat_eq(a361, a362) {
        f74(v0, 0u64)
    } else {
        let (_, h363) = {
            let (h364, _) = f75(Fn5::F76(v1, v0), v2, (v2, Bytes::lit(&[])));
            h364
        };
        h363
    }
}

pub fn f58() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f59(v0: Adt0, v1: bool, v2: List<(Bytes, Bytes)>, v3: u64) -> R<(Bytes, u64)> {
    let m365 = v0;
    match m365 {
        Adt0::C0 => {
            Ok((f77(Str::lit("(sort prop)")), v3))
        }
        Adt0::C1(v4) => {
            Ok((f78(f78(f77(Str::lit("(sort (type ")), f79(v4)), f77(Str::lit("))"))), v3))
        }
        Adt0::C2(v5, v6, v7) => {
            Ok((f80(v5, v6, v7), v3))
        }
        Adt0::C3(v8) => {
            Ok((f78(f78(f77(Str::lit("(local ")), if v1 {
                f81(v2, v8)
            } else {
                v8
            }), f77(Str::lit(")"))), v3))
        }
        Adt0::C4(r393, r394) => {
            let v9 = (*r393).clone();
            let v10 = (*r394).clone();
            let v11: (Bytes, u64) = f59(v9, v1, v2.clone(), v3)?;
            let v12: (Bytes, u64) = f82(v10, v1, v2, {
                let (_, h366) = v11;
                h366
            })?;
            Ok((f78(f78(f78(f77(Str::lit("(app ")), {
                let (h367, _) = v11;
                h367
            }), {
                let (h368, _) = v12;
                h368
            }), f77(Str::lit(")"))), {
                let (_, h369) = v12;
                h369
            }))
        }
        Adt0::C5(r395, r396) => {
            let v13 = (*r395).clone();
            let v14 = (*r396).clone();
            let v15: ((Bytes, List<(Bytes, Bytes)>), u64) = f83(v13, v1, v2, v3)?;
            let v16: (Bytes, u64) = f59(v14, v1, {
                let (_, h370) = {
                    let (h371, _) = v15.clone();
                    h371
                };
                h370
            }, {
                let (_, h372) = v15;
                h372
            })?;
            Ok((f78(f78(f78(f78(f77(Str::lit("(pi (")), {
                let (h373, _) = {
                    let (h374, _) = v15;
                    h374
                };
                h373
            }), f77(Str::lit(") "))), {
                let (h375, _) = v16;
                h375
            }), f77(Str::lit(")"))), {
                let (_, h376) = v16;
                h376
            }))
        }
        Adt0::C6(r397, r398) => {
            let v17 = (*r397).clone();
            let v18 = (*r398).clone();
            let v19: ((Bytes, List<(Bytes, Bytes)>), u64) = f83(v17, v1, v2, v3)?;
            let v20: (Bytes, u64) = f59(v18, v1, {
                let (_, h377) = {
                    let (h378, _) = v19.clone();
                    h378
                };
                h377
            }, {
                let (_, h379) = v19;
                h379
            })?;
            Ok((f78(f78(f78(f78(f77(Str::lit("(lam (")), {
                let (h380, _) = {
                    let (h381, _) = v19;
                    h381
                };
                h380
            }), f77(Str::lit(") "))), {
                let (h382, _) = v20;
                h382
            }), f77(Str::lit(")"))), {
                let (_, h383) = v20;
                h383
            }))
        }
        Adt0::C7(v21, r399, r400, r401) => {
            let v22 = (*r399).clone();
            let v23 = (*r400).clone();
            let v24 = (*r401).clone();
            let v25: (Bytes, u64) = f84(v21.clone(), v1, v3)?;
            let v26: List<(Bytes, Bytes)> = List::cons((v21, {
                let (h384, _) = v25.clone();
                h384
            }), v2);
            let v27: (Bytes, u64) = f59(v22, v1, v26.clone(), {
                let (_, h385) = v25;
                h385
            })?;
            let v28: (Bytes, u64) = f59(v23, v1, v26.clone(), {
                let (_, h386) = v27;
                h386
            })?;
            let v29: (Bytes, u64) = f59(v24, v1, v26, {
                let (_, h387) = v28;
                h387
            })?;
            Ok((f78(f78(f78(f78(f78(f78(f78(f77(Str::lit("(let ")), {
                let (h388, _) = v25;
                h388
            }), f77(Str::lit(" "))), {
                let (h389, _) = v27;
                h389
            }), f77(Str::lit(" "))), {
                let (h390, _) = v28;
                h390
            }), f77(Str::lit(" "))), f78({
                let (h391, _) = v29;
                h391
            }, f77(Str::lit(")")))), {
                let (_, h392) = v29;
                h392
            }))
        }
        Adt0::C8(v30) => {
            Ok((f78(f78(f77(Str::lit("(nat ")), v30), f77(Str::lit(")"))), v3))
        }
        Adt0::C9 => {
            Ok((Bytes::lit(&[]), v3))
        }
        Adt0::C10(_, _) => {
            Ok((Bytes::lit(&[]), v3))
        }
        Adt0::C11(_, _, _, _) => {
            Ok((Bytes::lit(&[]), v3))
        }
    }
}

pub fn f60(v0: List<Adt9>) -> Adt0 {
    f85(Fn6::F86, Adt0::C9, f87(v0))
}

pub fn f61(v0: Adt4, v1: Adt6, v2: Adt0, v3: Adt0, v4: Adt9, v5: Adt0, v6: List<Adt9>, v7: Adt0, v8: List<Adt0>, v9: u64, v10: u64, v11: List<(Bytes, Adt0)>) -> R<Adt5> {
    let m402 = v0;
    match m402 {
        Adt4::C0 => {
            Ok(f7(f8(v1), Bytes::lit(&[])))
        }
        Adt4::C1(r409) => {
            let v12 = (*r409).clone();
            let v13: Adt10 = f88(v12.clone(), v1, v6, match v4.clone() {
                Adt9::C0(_, h403, _) => {
                    h403
                }
            }, v5.clone())?;
            let v14: Adt7 = f89(v12.clone(), match v13.clone() {
                Adt10::C0(_, h404) => {
                    h404
                }
            }, v7, match v4 {
                Adt9::C0(_, h405, _) => {
                    h405
                }
            }, v5)?;
            f43(v12, match v14.clone() {
                Adt7::C0(_, h406) => {
                    h406
                }
            }, v2, v3, match v13 {
                Adt10::C0(h407, _) => {
                    h407
                }
            }, match v14 {
                Adt7::C0(h408, _) => {
                    h408
                }
            }, v8, v9, v10, v11)
        }
    }
}

pub fn f62(v0: Adt4, v1: Adt6, v2: List<Adt0>, v3: List<(Bytes, Adt0)>) -> R<Adt5> {
    let m410 = v0;
    match m410 {
        Adt4::C0 => {
            Ok(f7(f8(v1), Bytes::lit(&[])))
        }
        Adt4::C1(r418) => {
            let v4 = (*r418).clone();
            let m411 = v2;
            match m411.uncons() {
                None => {
                    f17(v1)
                }
                Some((v5, v6)) => {
                    let v7: Adt5 = f9(v4.clone(), v1, v5, v3.clone())?;
                    let m413 = match v7.clone() {
                        Adt5::C0(_, h412, _) => {
                            h412
                        }
                    };
                    match m413 {
                        None => {
                            let v9: Adt5 = f62(v4, match v7 {
                                Adt5::C0(_, _, h414) => {
                                    h414
                                }
                            }, v6, v3)?;
                            Ok(Adt5::C0(match v9.clone() {
                                Adt5::C0(h415, _, _) => {
                                    h415
                                }
                            }, match v9.clone() {
                                Adt5::C0(_, h416, _) => {
                                    h416
                                }
                            }, match v9 {
                                Adt5::C0(_, _, h417) => {
                                    h417
                                }
                            }))
                        }
                        Some(_) => {
                            Ok(v7)
                        }
                    }
                }
            }
        }
    }
}

pub fn f63(v0: u64) -> Bytes {
    let a419 = v0;
    let a420 = 1u64;
    if nat_eq(a419, a420) {
        Bytes::lit(&[])
    } else {
        f11(Str::lit("s"))
    }
}

pub fn f64(v0: Adt4, v1: Adt0, v2: Adt0) -> u64 {
    let v3: u64 = f22(v1.clone());
    let a421 = v3;
    let a422 = f22(v2.clone());
    if nat_eq(a421, a422) {
        let a423 = v3;
        let a424 = 2u64;
        if nat_lt(a423, a424) {
            0u64
        } else {
            let a425 = v3;
            let a426 = 2u64;
            if if nat_eq(a425, a426) {
                true
            } else {
                let a427 = v3;
                let a428 = 3u64;
                if nat_eq(a427, a428) {
                    true
                } else {
                    let a429 = v3;
                    let a430 = 8u64;
                    nat_eq(a429, a430)
                }
            } {
                let a431 = f47(v1);
                let a432 = f47(v2);
                if equal(a431, a432) {
                    0u64
                } else {
                    3u64
                }
            } else {
                let a433 = v3;
                let a434 = 4u64;
                if nat_eq(a433, a434) {
                    let m435 = v1;
                    match m435 {
                        Adt0::C0 => {
                            3u64
                        }
                        Adt0::C1(_) => {
                            3u64
                        }
                        Adt0::C2(_, _, _) => {
                            3u64
                        }
                        Adt0::C3(_) => {
                            3u64
                        }
                        Adt0::C4(_, r442) => {
                            let v5 = (*r442).clone();
                            let m436 = v2;
                            match m436 {
                                Adt0::C0 => {
                                    3u64
                                }
                                Adt0::C1(_) => {
                                    3u64
                                }
                                Adt0::C2(_, _, _) => {
                                    3u64
                                }
                                Adt0::C3(_) => {
                                    3u64
                                }
                                Adt0::C4(_, r441) => {
                                    let v7 = (*r441).clone();
                                    let a439 = {
                                        let a437 = f24(v0.clone(), v5);
                                        length_list(a437)
                                    };
                                    let a440 = {
                                        let a438 = f24(v0, v7);
                                        length_list(a438)
                                    };
                                    if nat_eq(a439, a440) {
                                        1u64
                                    } else {
                                        3u64
                                    }
                                }
                                Adt0::C5(_, _) => {
                                    3u64
                                }
                                Adt0::C6(_, _) => {
                                    3u64
                                }
                                Adt0::C7(_, _, _, _) => {
                                    3u64
                                }
                                Adt0::C8(_) => {
                                    3u64
                                }
                                Adt0::C9 => {
                                    3u64
                                }
                                Adt0::C10(_, _) => {
                                    3u64
                                }
                                Adt0::C11(_, _, _, _) => {
                                    3u64
                                }
                            }
                        }
                        Adt0::C5(_, _) => {
                            3u64
                        }
                        Adt0::C6(_, _) => {
                            3u64
                        }
                        Adt0::C7(_, _, _, _) => {
                            3u64
                        }
                        Adt0::C8(_) => {
                            3u64
                        }
                        Adt0::C9 => {
                            3u64
                        }
                        Adt0::C10(_, _) => {
                            3u64
                        }
                        Adt0::C11(_, _, _, _) => {
                            3u64
                        }
                    }
                } else {
                    let a443 = v3;
                    let a444 = 5u64;
                    if if nat_eq(a443, a444) {
                        true
                    } else {
                        let a445 = v3;
                        let a446 = 6u64;
                        nat_eq(a445, a446)
                    } {
                        2u64
                    } else {
                        3u64
                    }
                }
            }
        }
    } else {
        3u64
    }
}

pub fn f65(v0: Adt0) -> (Adt0, Adt0) {
    let m447 = v0;
    match m447 {
        Adt0::C0 => {
            (Adt0::C9, Adt0::C9)
        }
        Adt0::C1(_) => {
            (Adt0::C9, Adt0::C9)
        }
        Adt0::C2(_, _, _) => {
            (Adt0::C9, Adt0::C9)
        }
        Adt0::C3(_) => {
            (Adt0::C9, Adt0::C9)
        }
        Adt0::C4(r448, r449) => {
            let v1 = (*r448).clone();
            let v2 = (*r449).clone();
            (v1, v2)
        }
        Adt0::C5(_, _) => {
            (Adt0::C9, Adt0::C9)
        }
        Adt0::C6(_, _) => {
            (Adt0::C9, Adt0::C9)
        }
        Adt0::C7(_, _, _, _) => {
            (Adt0::C9, Adt0::C9)
        }
        Adt0::C8(_) => {
            (Adt0::C9, Adt0::C9)
        }
        Adt0::C9 => {
            (Adt0::C9, Adt0::C9)
        }
        Adt0::C10(_, _) => {
            (Adt0::C9, Adt0::C9)
        }
        Adt0::C11(_, _, _, _) => {
            (Adt0::C9, Adt0::C9)
        }
    }
}

pub fn f66(p462: Adt4, p463: Adt6, p464: Adt0, p465: Adt0) -> R<Adt8> {
    let m461 = (p462, (p463, (p464, p465)));
    {
        let mut m461 = m461;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m461;
                let m450 = v0;
                match m450 {
                    Adt4::C0 => {
                        Err::<(Adt4, (Adt6, (Adt0, Adt0))), R<Adt8>>(Ok(f46(f8(v1))))
                    }
                    Adt4::C1(r460) => {
                        let v4 = (*r460).clone();
                        let m451 = v2;
                        match m451 {
                            Adt0::C0 => {
                                Err::<(Adt4, (Adt6, (Adt0, Adt0))), R<Adt8>>(Ok(f46(v1)))
                            }
                            Adt0::C1(_) => {
                                Err::<(Adt4, (Adt6, (Adt0, Adt0))), R<Adt8>>(Ok(f46(v1)))
                            }
                            Adt0::C2(_, _, _) => {
                                Err::<(Adt4, (Adt6, (Adt0, Adt0))), R<Adt8>>(Ok(f46(v1)))
                            }
                            Adt0::C3(_) => {
                                Err::<(Adt4, (Adt6, (Adt0, Adt0))), R<Adt8>>(Ok(f46(v1)))
                            }
                            Adt0::C4(_, _) => {
                                Err::<(Adt4, (Adt6, (Adt0, Adt0))), R<Adt8>>(Ok(f46(v1)))
                            }
                            Adt0::C5(_, _) => {
                                Err::<(Adt4, (Adt6, (Adt0, Adt0))), R<Adt8>>(Ok(f46(v1)))
                            }
                            Adt0::C6(_, _) => {
                                Err::<(Adt4, (Adt6, (Adt0, Adt0))), R<Adt8>>(Ok(f46(v1)))
                            }
                            Adt0::C7(_, _, _, _) => {
                                Err::<(Adt4, (Adt6, (Adt0, Adt0))), R<Adt8>>(Ok(f46(v1)))
                            }
                            Adt0::C8(_) => {
                                Err::<(Adt4, (Adt6, (Adt0, Adt0))), R<Adt8>>(Ok(f46(v1)))
                            }
                            Adt0::C9 => {
                                Err::<(Adt4, (Adt6, (Adt0, Adt0))), R<Adt8>>(Ok(f46(v1)))
                            }
                            Adt0::C10(r458, r459) => {
                                let v5 = (*r458).clone();
                                let v6 = (*r459).clone();
                                let m452 = v3;
                                match m452 {
                                    Adt0::C0 => {
                                        Err::<(Adt4, (Adt6, (Adt0, Adt0))), R<Adt8>>(Ok(f46(v1)))
                                    }
                                    Adt0::C1(_) => {
                                        Err::<(Adt4, (Adt6, (Adt0, Adt0))), R<Adt8>>(Ok(f46(v1)))
                                    }
                                    Adt0::C2(_, _, _) => {
                                        Err::<(Adt4, (Adt6, (Adt0, Adt0))), R<Adt8>>(Ok(f46(v1)))
                                    }
                                    Adt0::C3(_) => {
                                        Err::<(Adt4, (Adt6, (Adt0, Adt0))), R<Adt8>>(Ok(f46(v1)))
                                    }
                                    Adt0::C4(_, _) => {
                                        Err::<(Adt4, (Adt6, (Adt0, Adt0))), R<Adt8>>(Ok(f46(v1)))
                                    }
                                    Adt0::C5(_, _) => {
                                        Err::<(Adt4, (Adt6, (Adt0, Adt0))), R<Adt8>>(Ok(f46(v1)))
                                    }
                                    Adt0::C6(_, _) => {
                                        Err::<(Adt4, (Adt6, (Adt0, Adt0))), R<Adt8>>(Ok(f46(v1)))
                                    }
                                    Adt0::C7(_, _, _, _) => {
                                        Err::<(Adt4, (Adt6, (Adt0, Adt0))), R<Adt8>>(Ok(f46(v1)))
                                    }
                                    Adt0::C8(_) => {
                                        Err::<(Adt4, (Adt6, (Adt0, Adt0))), R<Adt8>>(Ok(f46(v1)))
                                    }
                                    Adt0::C9 => {
                                        Err::<(Adt4, (Adt6, (Adt0, Adt0))), R<Adt8>>(Ok(f46(v1)))
                                    }
                                    Adt0::C10(r456, r457) => {
                                        let v7 = (*r456).clone();
                                        let v8 = (*r457).clone();
                                        let v9: Adt8 = f27(v4.clone(), v1, v5, v7)?;
                                        let m454 = match v9.clone() {
                                            Adt8::C0(h453, _) => {
                                                h453
                                            }
                                        };
                                        match m454 {
                                            None => {
                                                Ok::<(Adt4, (Adt6, (Adt0, Adt0))), R<Adt8>>((v4, (match v9 {
                                                    Adt8::C0(_, h455) => {
                                                        h455
                                                    }
                                                }, (v6, v8))))
                                            }
                                            Some(_) => {
                                                Err::<(Adt4, (Adt6, (Adt0, Adt0))), R<Adt8>>(Ok(v9))
                                            }
                                        }
                                    }
                                    Adt0::C11(_, _, _, _) => {
                                        Err::<(Adt4, (Adt6, (Adt0, Adt0))), R<Adt8>>(Ok(f46(v1)))
                                    }
                                }
                            }
                            Adt0::C11(_, _, _, _) => {
                                Err::<(Adt4, (Adt6, (Adt0, Adt0))), R<Adt8>>(Ok(f46(v1)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m461 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f67(v0: Adt4, v1: Adt6, v2: Adt0, v3: Adt0) -> R<Adt8> {
    let m466 = v0;
    match m466 {
        Adt4::C0 => {
            Ok(f46(f8(v1)))
        }
        Adt4::C1(r479) => {
            let v4 = (*r479).clone();
            let v5: bool = {
                let a467 = f22(v2.clone());
                let a468 = 5u64;
                nat_eq(a467, a468)
            };
            let v6: Adt11 = f90(v4.clone(), v2.clone(), v5);
            let v7: Adt11 = f90(v4.clone(), v3.clone(), v5);
            let v8: u64 = {
                let a470 = match v6.clone() {
                    Adt11::C0(h469, _) => {
                        h469
                    }
                };
                length_list(a470)
            };
            let v9: u64 = {
                let a472 = match v7.clone() {
                    Adt11::C0(h471, _) => {
                        h471
                    }
                };
                length_list(a472)
            };
            let a473 = v8;
            let a474 = v9;
            if nat_eq(a473, a474) {
                f91(v4, v1, match v6.clone() {
                    Adt11::C0(h475, _) => {
                        h475
                    }
                }, match v7.clone() {
                    Adt11::C0(h476, _) => {
                        h476
                    }
                }, match v7 {
                    Adt11::C0(_, h477) => {
                        h477
                    }
                }, match v6 {
                    Adt11::C0(_, h478) => {
                        h478
                    }
                }, v2, v3)
            } else {
                let v10: Bytes = f23(v2)?;
                let v11: Bytes = f23(v3)?;
                Ok(f68(v1, f16(f16(f16(f16(f16(f16(v10, f11(Str::lit(" binds "))), f41(v8)), f11(Str::lit(" arguments but "))), v11), f11(Str::lit(" binds "))), f41(v9))))
            }
        }
    }
}

pub fn f68(v0: Adt6, v1: Bytes) -> Adt8 {
    Adt8::C0(Some(v1), v0)
}

pub fn f69(v0: List<(Bytes, Adt0)>, v1: Bytes) -> Option<Adt0> {
    f39(Fn2::F92(v1), None::<Adt0>, v0)
}

pub fn f70(v0: Option<Adt0>, v1: Option<Adt0>) -> Option<(Adt0, Adt0)> {
    let m480 = v0;
    match m480 {
        None => {
            None::<(Adt0, Adt0)>
        }
        Some(v2) => {
            let m481 = v1;
            match m481 {
                None => {
                    None::<(Adt0, Adt0)>
                }
                Some(v3) => {
                    Some((v2, v3))
                }
            }
        }
    }
}

pub fn f71(v0: Adt4, v1: Adt0, v2: List<Bytes>) -> R<Option<List<Bytes>>> {
    let m482 = v0;
    match m482 {
        Adt4::C0 => {
            Ok(None::<List<Bytes>>)
        }
        Adt4::C1(r495) => {
            let v3 = (*r495).clone();
            let m483 = v1;
            match m483 {
                Adt0::C0 => {
                    Ok(Some(List::<Bytes>::nil()))
                }
                Adt0::C1(_) => {
                    Ok(Some(List::<Bytes>::nil()))
                }
                Adt0::C2(_, _, _) => {
                    Ok(Some(List::<Bytes>::nil()))
                }
                Adt0::C3(v4) => {
                    if f93(v2, v4.clone())? {
                        Ok(Some(List::<Bytes>::nil()))
                    } else {
                        Ok(Some(List::cons(v4, List::<Bytes>::nil())))
                    }
                }
                Adt0::C4(r484, r485) => {
                    let v5 = (*r484).clone();
                    let v6 = (*r485).clone();
                    Ok(f94(f71(v3.clone(), v5, v2.clone())?, f71(v3, v6, v2)?))
                }
                Adt0::C5(r486, r487) => {
                    let v9 = (*r486).clone();
                    let v10 = (*r487).clone();
                    f95(v3, v9, v2, v10)
                }
                Adt0::C6(r488, r489) => {
                    let v11 = (*r488).clone();
                    let v12 = (*r489).clone();
                    f95(v3, v11, v2, v12)
                }
                Adt0::C7(v13, r490, r491, r492) => {
                    let v14 = (*r490).clone();
                    let v15 = (*r491).clone();
                    let v16 = (*r492).clone();
                    Ok(f94(f94(f71(v3.clone(), v14, v2.clone())?, f71(v3.clone(), v15, v2.clone())?), f71(v3, v16, List::cons(v13, v2))?))
                }
                Adt0::C8(_) => {
                    Ok(Some(List::<Bytes>::nil()))
                }
                Adt0::C9 => {
                    Ok(Some(List::<Bytes>::nil()))
                }
                Adt0::C10(r493, r494) => {
                    let v7 = (*r493).clone();
                    let v8 = (*r494).clone();
                    Ok(f94(f71(v3.clone(), v7, v2.clone())?, f71(v3, v8, v2)?))
                }
                Adt0::C11(_, _, _, _) => {
                    Ok(Some(List::<Bytes>::nil()))
                }
            }
        }
    }
}

pub fn f72(v0: Fn4, p499: bool, p500: List<Bytes>) -> R<bool> {
    let m498 = (p499, p500);
    {
        let mut m498 = m498;
        loop {
            let step = {
                let (v1, v2) = m498;
                let m496 = v2;
                match m496.uncons() {
                    None => {
                        Err::<(bool, List<Bytes>), R<bool>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<Bytes>), R<bool>>(({
                            let c497: Fn4 = v0.clone();
                            c497.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m498 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f73(v0: bool, v1: Bytes) -> R<bool> {
    if v0 {
        Ok(true)
    } else {
        f19(v1)
    }
}

pub fn f74(v0: Bytes, v1: u64) -> Bytes {
    let m504 = {
        let a501 = v0;
        let a502 = v1;
        let a503 = 1u64;
        slice_bytes(a501, a502, a503)
    };
    match m504 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f75(v0: Fn5, p509: u64, p510: (u64, Bytes)) -> ((u64, Bytes), bool) {
    let m508 = (p509, p510);
    {
        let mut m508 = m508;
        loop {
            let step = {
                let (v1, v2) = m508;
                let m505 = v1;
                if m505 == 0 {
                    Err::<(u64, (u64, Bytes)), ((u64, Bytes), bool)>((v2, false))
                } else {
                    let v3 = m505 - 1;
                    let m507 = {
                        let c506: Fn5 = v0.clone();
                        c506.apply(v2.clone())
                    };
                    match m507 {
                        None => {
                            Err::<(u64, (u64, Bytes)), ((u64, Bytes), bool)>((v2, true))
                        }
                        Some(v4) => {
                            Ok::<(u64, (u64, Bytes)), ((u64, Bytes), bool)>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m508 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f76(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f96(v1, v0, v2)
}

pub fn f77(v0: Str) -> Bytes {
    let a511 = v0;
    utf8_encode(a511)
}

pub fn f78(v0: Bytes, v1: Bytes) -> Bytes {
    let a512 = v0;
    let a513 = v1;
    append_bytes(a512, a513)
}

pub fn f79(v0: Adt1) -> Bytes {
    let m514 = v0;
    match m514 {
        Adt1::C0(v1) => {
            f41(v1)
        }
        Adt1::C1(v2) => {
            v2
        }
        Adt1::C2(r515) => {
            let v3 = (*r515).clone();
            f78(f78(f77(Str::lit("(succ ")), f79(v3)), f77(Str::lit(")")))
        }
        Adt1::C3(r516) => {
            let v4 = (*r516).clone();
            f78(f78(f77(Str::lit("(max")), f97(v4)), f77(Str::lit(")")))
        }
        Adt1::C4(r517, r518) => {
            let v5 = (*r517).clone();
            let v6 = (*r518).clone();
            f78(f78(f78(f78(f77(Str::lit("(imax ")), f79(v5)), f77(Str::lit(" "))), f79(v6)), f77(Str::lit(")")))
        }
        Adt1::C5 => {
            Bytes::lit(&[])
        }
        Adt1::C6(_, _) => {
            Bytes::lit(&[])
        }
    }
}

pub fn f80(v0: Bytes, v1: Bytes, v2: Adt1) -> Bytes {
    let v3: Bytes = f78(f78(f78(f77(Str::lit("(const ")), v0), f77(Str::lit("::"))), v1);
    let m519 = v2.clone();
    match m519 {
        Adt1::C0(_) => {
            f78(v3, f77(Str::lit(")")))
        }
        Adt1::C1(_) => {
            f78(v3, f77(Str::lit(")")))
        }
        Adt1::C2(_) => {
            f78(v3, f77(Str::lit(")")))
        }
        Adt1::C3(_) => {
            f78(v3, f77(Str::lit(")")))
        }
        Adt1::C4(_, _) => {
            f78(v3, f77(Str::lit(")")))
        }
        Adt1::C5 => {
            f78(v3, f77(Str::lit(")")))
        }
        Adt1::C6(_, _) => {
            let v6: Bytes = f97(v2);
            f78(f78(f78(v3, f77(Str::lit(" ("))), f98(v6.clone(), 1u64, {
                let a521 = {
                    let a520 = v6;
                    length_bytes(a520)
                };
                let a522 = 1u64;
                nat_sub(a521, a522)
            })), f77(Str::lit("))")))
        }
    }
}

pub fn f81(v0: List<(Bytes, Bytes)>, v1: Bytes) -> Bytes {
    let m523 = f99(Fn7::F100(v1.clone()), None::<Bytes>, v0);
    match m523 {
        None => {
            v1
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f82(v0: Adt0, v1: bool, v2: List<(Bytes, Bytes)>, v3: u64) -> R<(Bytes, u64)> {
    let m524 = v0;
    match m524 {
        Adt0::C0 => {
            Ok((Bytes::lit(&[]), v3))
        }
        Adt0::C1(_) => {
            Ok((Bytes::lit(&[]), v3))
        }
        Adt0::C2(_, _, _) => {
            Ok((Bytes::lit(&[]), v3))
        }
        Adt0::C3(_) => {
            Ok((Bytes::lit(&[]), v3))
        }
        Adt0::C4(_, _) => {
            Ok((Bytes::lit(&[]), v3))
        }
        Adt0::C5(_, _) => {
            Ok((Bytes::lit(&[]), v3))
        }
        Adt0::C6(_, _) => {
            Ok((Bytes::lit(&[]), v3))
        }
        Adt0::C7(_, _, _, _) => {
            Ok((Bytes::lit(&[]), v3))
        }
        Adt0::C8(_) => {
            Ok((Bytes::lit(&[]), v3))
        }
        Adt0::C9 => {
            Ok((Bytes::lit(&[]), v3))
        }
        Adt0::C10(r529, r530) => {
            let v4 = (*r529).clone();
            let v5 = (*r530).clone();
            let v6: (Bytes, u64) = f59(v4, v1, v2.clone(), v3)?;
            let v7: (Bytes, u64) = f82(v5, v1, v2, {
                let (_, h525) = v6;
                h525
            })?;
            Ok((f78(f78(f77(Str::lit(" ")), {
                let (h526, _) = v6;
                h526
            }), {
                let (h527, _) = v7;
                h527
            }), {
                let (_, h528) = v7;
                h528
            }))
        }
        Adt0::C11(_, _, _, _) => {
            Ok((Bytes::lit(&[]), v3))
        }
    }
}

pub fn f83(v0: Adt0, v1: bool, v2: List<(Bytes, Bytes)>, v3: u64) -> R<((Bytes, List<(Bytes, Bytes)>), u64)> {
    let m531 = v0;
    match m531 {
        Adt0::C0 => {
            Ok(((Bytes::lit(&[]), v2), v3))
        }
        Adt0::C1(_) => {
            Ok(((Bytes::lit(&[]), v2), v3))
        }
        Adt0::C2(_, _, _) => {
            Ok(((Bytes::lit(&[]), v2), v3))
        }
        Adt0::C3(_) => {
            Ok(((Bytes::lit(&[]), v2), v3))
        }
        Adt0::C4(_, _) => {
            Ok(((Bytes::lit(&[]), v2), v3))
        }
        Adt0::C5(_, _) => {
            Ok(((Bytes::lit(&[]), v2), v3))
        }
        Adt0::C6(_, _) => {
            Ok(((Bytes::lit(&[]), v2), v3))
        }
        Adt0::C7(_, _, _, _) => {
            Ok(((Bytes::lit(&[]), v2), v3))
        }
        Adt0::C8(_) => {
            Ok(((Bytes::lit(&[]), v2), v3))
        }
        Adt0::C9 => {
            Ok(((Bytes::lit(&[]), v2), v3))
        }
        Adt0::C10(_, _) => {
            Ok(((Bytes::lit(&[]), v2), v3))
        }
        Adt0::C11(v4, v5, r554, r555) => {
            let v6 = (*r554).clone();
            let v7 = (*r555).clone();
            let v8: (Bytes, u64) = f84(v5.clone(), v1, v3)?;
            let v9: (Bytes, u64) = f59(v6, v1, v2.clone(), {
                let (_, h532) = v8.clone();
                h532
            })?;
            let v10: Bytes = f78(f78(f78(f78(f78(f78(f77(Str::lit("(")), f101(v4)), f77(Str::lit(" "))), {
                let (h533, _) = v8.clone();
                h533
            }), f77(Str::lit(" "))), {
                let (h534, _) = v9;
                h534
            }), f77(Str::lit(")")));
            let v11: List<(Bytes, Bytes)> = List::cons((v5, {
                let (h535, _) = v8;
                h535
            }), v2);
            let m536 = v7.clone();
            match m536 {
                Adt0::C0 => {
                    Ok(((v10, v11), {
                        let (_, h544) = v9;
                        h544
                    }))
                }
                Adt0::C1(_) => {
                    Ok(((v10, v11), {
                        let (_, h545) = v9;
                        h545
                    }))
                }
                Adt0::C2(_, _, _) => {
                    Ok(((v10, v11), {
                        let (_, h546) = v9;
                        h546
                    }))
                }
                Adt0::C3(_) => {
                    Ok(((v10, v11), {
                        let (_, h547) = v9;
                        h547
                    }))
                }
                Adt0::C4(_, _) => {
                    Ok(((v10, v11), {
                        let (_, h548) = v9;
                        h548
                    }))
                }
                Adt0::C5(_, _) => {
                    Ok(((v10, v11), {
                        let (_, h549) = v9;
                        h549
                    }))
                }
                Adt0::C6(_, _) => {
                    Ok(((v10, v11), {
                        let (_, h550) = v9;
                        h550
                    }))
                }
                Adt0::C7(_, _, _, _) => {
                    Ok(((v10, v11), {
                        let (_, h551) = v9;
                        h551
                    }))
                }
                Adt0::C8(_) => {
                    Ok(((v10, v11), {
                        let (_, h552) = v9;
                        h552
                    }))
                }
                Adt0::C9 => {
                    Ok(((v10, v11), {
                        let (_, h537) = v9;
                        h537
                    }))
                }
                Adt0::C10(_, _) => {
                    Ok(((v10, v11), {
                        let (_, h553) = v9;
                        h553
                    }))
                }
                Adt0::C11(_, _, _, _) => {
                    let v16: ((Bytes, List<(Bytes, Bytes)>), u64) = f83(v7, v1, v11, {
                        let (_, h538) = v9;
                        h538
                    })?;
                    Ok(((f78(f78(v10, f77(Str::lit(" "))), {
                        let (h539, _) = {
                            let (h540, _) = v16.clone();
                            h540
                        };
                        h539
                    }), {
                        let (_, h541) = {
                            let (h542, _) = v16;
                            h542
                        };
                        h541
                    }), {
                        let (_, h543) = v16;
                        h543
                    }))
                }
            }
        }
    }
}

pub fn f84(v0: Bytes, v1: bool, v2: u64) -> R<(Bytes, u64)> {
    if v1 {
        Ok((f78(f77(Str::lit("x")), f41(v2)), {
            let a556 = v2;
            let a557 = 1u64;
            nat_add(a556, a557)?
        }))
    } else {
        Ok((v0, v2))
    }
}

pub fn f85(v0: Fn6, p561: Adt0, p562: List<Adt9>) -> Adt0 {
    let m560 = (p561, p562);
    {
        let mut m560 = m560;
        loop {
            let step = {
                let (v1, v2) = m560;
                let m558 = v2;
                match m558.uncons() {
                    None => {
                        Err::<(Adt0, List<Adt9>), Adt0>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt0, List<Adt9>), Adt0>(({
                            let c559: Fn6 = v0.clone();
                            c559.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m560 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f86(v0: Adt0, v1: Adt9) -> Adt0 {
    Adt0::C11(match v1.clone() {
        Adt9::C0(h563, _, _) => {
            h563
        }
    }, match v1.clone() {
        Adt9::C0(_, h564, _) => {
            h564
        }
    }, std::rc::Rc::new(match v1 {
        Adt9::C0(_, _, h565) => {
            h565
        }
    }), std::rc::Rc::new(v0))
}

pub fn f87(v0: List<Adt9>) -> List<Adt9> {
    f102(Fn8::F103, List::<Adt9>::nil(), v0)
}

pub fn f88(v0: Adt4, v1: Adt6, v2: List<Adt9>, v3: Bytes, v4: Adt0) -> R<Adt10> {
    let m566 = v0;
    match m566 {
        Adt4::C0 => {
            Ok(Adt10::C0(v2, f8(v1)))
        }
        Adt4::C1(r575) => {
            let v5 = (*r575).clone();
            let m567 = v2;
            match m567.uncons() {
                None => {
                    Ok(Adt10::C0(List::<Adt9>::nil(), v1))
                }
                Some((v6, v7)) => {
                    let v8: Adt7 = f89(v5.clone(), v1, match v6.clone() {
                        Adt9::C0(_, _, h568) => {
                            h568
                        }
                    }, v3.clone(), v4.clone())?;
                    let v9: Adt10 = f88(v5, match v8.clone() {
                        Adt7::C0(_, h569) => {
                            h569
                        }
                    }, v7, v3, v4)?;
                    Ok(Adt10::C0(List::cons(Adt9::C0(match v6.clone() {
                        Adt9::C0(h570, _, _) => {
                            h570
                        }
                    }, match v6 {
                        Adt9::C0(_, h571, _) => {
                            h571
                        }
                    }, match v8 {
                        Adt7::C0(h572, _) => {
                            h572
                        }
                    }), match v9.clone() {
                        Adt10::C0(h573, _) => {
                            h573
                        }
                    }), match v9 {
                        Adt10::C0(_, h574) => {
                            h574
                        }
                    }))
                }
            }
        }
    }
}

pub fn f89(v0: Adt4, v1: Adt6, v2: Adt0, v3: Bytes, v4: Adt0) -> R<Adt7> {
    let m576 = f71(v0.clone(), v4.clone(), List::<Bytes>::nil())?;
    match m576 {
        None => {
            Ok(Adt7::C0(v2, f8(v1)))
        }
        Some(v5) => {
            f104(v0, v1, v2, v3, v4, v5)
        }
    }
}

pub fn f90(v0: Adt4, v1: Adt0, v2: bool) -> Adt11 {
    let m577 = v0;
    match m577 {
        Adt4::C0 => {
            Adt11::C0(List::<Adt9>::nil(), v1)
        }
        Adt4::C1(r596) => {
            let v3 = (*r596).clone();
            let m578 = v1;
            match m578 {
                Adt0::C0 => {
                    Adt11::C0(List::<Adt9>::nil(), Adt0::C0)
                }
                Adt0::C1(v10) => {
                    Adt11::C0(List::<Adt9>::nil(), Adt0::C1(v10))
                }
                Adt0::C2(v11, v12, v13) => {
                    Adt11::C0(List::<Adt9>::nil(), Adt0::C2(v11, v12, v13))
                }
                Adt0::C3(v14) => {
                    Adt11::C0(List::<Adt9>::nil(), Adt0::C3(v14))
                }
                Adt0::C4(r583, r584) => {
                    let v15 = (*r583).clone();
                    let v16 = (*r584).clone();
                    Adt11::C0(List::<Adt9>::nil(), Adt0::C4(std::rc::Rc::new(v15), std::rc::Rc::new(v16)))
                }
                Adt0::C5(r585, r586) => {
                    let v4 = (*r585).clone();
                    let v5 = (*r586).clone();
                    if v2 {
                        let v6: Adt11 = f90(v3.clone(), v5, v2);
                        Adt11::C0(f102(Fn8::F105, match v6.clone() {
                            Adt11::C0(h579, _) => {
                                h579
                            }
                        }, f87(f44(v3, v4))), match v6 {
                            Adt11::C0(_, h580) => {
                                h580
                            }
                        })
                    } else {
                        Adt11::C0(List::<Adt9>::nil(), Adt0::C5(std::rc::Rc::new(v4), std::rc::Rc::new(v5)))
                    }
                }
                Adt0::C6(r587, r588) => {
                    let v7 = (*r587).clone();
                    let v8 = (*r588).clone();
                    if v2 {
                        Adt11::C0(List::<Adt9>::nil(), Adt0::C6(std::rc::Rc::new(v7), std::rc::Rc::new(v8)))
                    } else {
                        let v9: Adt11 = f90(v3.clone(), v8, v2);
                        Adt11::C0(f102(Fn8::F106, match v9.clone() {
                            Adt11::C0(h581, _) => {
                                h581
                            }
                        }, f87(f44(v3, v7))), match v9 {
                            Adt11::C0(_, h582) => {
                                h582
                            }
                        })
                    }
                }
                Adt0::C7(v17, r589, r590, r591) => {
                    let v18 = (*r589).clone();
                    let v19 = (*r590).clone();
                    let v20 = (*r591).clone();
                    Adt11::C0(List::<Adt9>::nil(), Adt0::C7(v17, std::rc::Rc::new(v18), std::rc::Rc::new(v19), std::rc::Rc::new(v20)))
                }
                Adt0::C8(v21) => {
                    Adt11::C0(List::<Adt9>::nil(), Adt0::C8(v21))
                }
                Adt0::C9 => {
                    Adt11::C0(List::<Adt9>::nil(), Adt0::C9)
                }
                Adt0::C10(r592, r593) => {
                    let v22 = (*r592).clone();
                    let v23 = (*r593).clone();
                    Adt11::C0(List::<Adt9>::nil(), Adt0::C10(std::rc::Rc::new(v22), std::rc::Rc::new(v23)))
                }
                Adt0::C11(v24, v25, r594, r595) => {
                    let v26 = (*r594).clone();
                    let v27 = (*r595).clone();
                    Adt11::C0(List::<Adt9>::nil(), Adt0::C11(v24, v25, std::rc::Rc::new(v26), std::rc::Rc::new(v27)))
                }
            }
        }
    }
}

pub fn f91(v0: Adt4, v1: Adt6, v2: List<Adt9>, v3: List<Adt9>, v4: Adt0, v5: Adt0, v6: Adt0, v7: Adt0) -> R<Adt8> {
    let m597 = v0;
    match m597 {
        Adt4::C0 => {
            Ok(f46(f8(v1)))
        }
        Adt4::C1(r622) => {
            let v8 = (*r622).clone();
            let m598 = v2;
            match m598.uncons() {
                None => {
                    f27(v8, v1, v5, v4)
                }
                Some((v9, v10)) => {
                    let m599 = v3;
                    match m599.uncons() {
                        None => {
                            f27(v8, v1, v5, v4)
                        }
                        Some((v11, v12)) => {
                            let a602 = match v9.clone() {
                                Adt9::C0(h600, _, _) => {
                                    h600
                                }
                            };
                            let a603 = match v11.clone() {
                                Adt9::C0(h601, _, _) => {
                                    h601
                                }
                            };
                            if nat_eq(a602, a603) {
                                let v13: Adt8 = f27(v8.clone(), v1, match v9.clone() {
                                    Adt9::C0(_, _, h604) => {
                                        h604
                                    }
                                }, match v11.clone() {
                                    Adt9::C0(_, _, h605) => {
                                        h605
                                    }
                                })?;
                                let m607 = match v13.clone() {
                                    Adt8::C0(h606, _) => {
                                        h606
                                    }
                                };
                                match m607 {
                                    None => {
                                        let v15: Adt0 = Adt0::C3(match v9 {
                                            Adt9::C0(_, h608, _) => {
                                                h608
                                            }
                                        });
                                        let v16: Adt10 = f88(v8.clone(), match v13 {
                                            Adt8::C0(_, h609) => {
                                                h609
                                            }
                                        }, v12, match v11.clone() {
                                            Adt9::C0(_, h610, _) => {
                                                h610
                                            }
                                        }, v15.clone())?;
                                        let v17: Adt7 = f89(v8.clone(), match v16.clone() {
                                            Adt10::C0(_, h611) => {
                                                h611
                                            }
                                        }, v4, match v11 {
                                            Adt9::C0(_, h612, _) => {
                                                h612
                                            }
                                        }, v15)?;
                                        let v18: Adt8 = f91(v8, match v17.clone() {
                                            Adt7::C0(_, h613) => {
                                                h613
                                            }
                                        }, v10, match v16 {
                                            Adt10::C0(h614, _) => {
                                                h614
                                            }
                                        }, match v17 {
                                            Adt7::C0(h615, _) => {
                                                h615
                                            }
                                        }, v5, v6, v7)?;
                                        Ok(Adt8::C0(match v18.clone() {
                                            Adt8::C0(h616, _) => {
                                                h616
                                            }
                                        }, match v18 {
                                            Adt8::C0(_, h617) => {
                                                h617
                                            }
                                        }))
                                    }
                                    Some(_) => {
                                        Ok(v13)
                                    }
                                }
                            } else {
                                let v19: Bytes = f23(v6)?;
                                let v20: Bytes = f23(v7)?;
                                Ok(f68(v1, f16(f16(f16(f16(f16(f16(f16(f16(f16(f16(f16(f11(Str::lit("binder `")), match v9.clone() {
                                    Adt9::C0(_, h618, _) => {
                                        h618
                                    }
                                }), f11(Str::lit("` is "))), f107(match v9 {
                                    Adt9::C0(h619, _, _) => {
                                        h619
                                    }
                                })), f11(Str::lit(" in "))), v19), f11(Str::lit(" but `"))), match v11.clone() {
                                    Adt9::C0(_, h620, _) => {
                                        h620
                                    }
                                }), f11(Str::lit("` is "))), f107(match v11 {
                                    Adt9::C0(h621, _, _) => {
                                        h621
                                    }
                                })), f11(Str::lit(" in "))), v20)))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f92(v0: Bytes, v1: Option<Adt0>, v2: (Bytes, Adt0)) -> Option<Adt0> {
    let m623 = v1.clone();
    match m623 {
        None => {
            let a625 = {
                let (h624, _) = v2;
                h624
            };
            let a626 = v0;
            if equal(a625, a626) {
                Some({
                    let (_, h627) = v2;
                    h627
                })
            } else {
                None::<Adt0>
            }
        }
        Some(_) => {
            v1
        }
    }
}

pub fn f93(v0: List<Bytes>, v1: Bytes) -> R<bool> {
    f72(Fn4::F108(v1), false, v0)
}

pub fn f94(v0: Option<List<Bytes>>, v1: Option<List<Bytes>>) -> Option<List<Bytes>> {
    let m628 = v0;
    match m628 {
        None => {
            None::<List<Bytes>>
        }
        Some(v2) => {
            let m629 = v1;
            match m629 {
                None => {
                    None::<List<Bytes>>
                }
                Some(v3) => {
                    Some(f54(Fn3::F109, v3, v2))
                }
            }
        }
    }
}

pub fn f95(v0: Adt4, v1: Adt0, v2: List<Bytes>, v3: Adt0) -> R<Option<List<Bytes>>> {
    let m630 = v0;
    match m630 {
        Adt4::C0 => {
            Ok(None::<List<Bytes>>)
        }
        Adt4::C1(r634) => {
            let v4 = (*r634).clone();
            let m631 = v1;
            match m631 {
                Adt0::C0 => {
                    f71(v4, v3, v2)
                }
                Adt0::C1(_) => {
                    f71(v4, v3, v2)
                }
                Adt0::C2(_, _, _) => {
                    f71(v4, v3, v2)
                }
                Adt0::C3(_) => {
                    f71(v4, v3, v2)
                }
                Adt0::C4(_, _) => {
                    f71(v4, v3, v2)
                }
                Adt0::C5(_, _) => {
                    f71(v4, v3, v2)
                }
                Adt0::C6(_, _) => {
                    f71(v4, v3, v2)
                }
                Adt0::C7(_, _, _, _) => {
                    f71(v4, v3, v2)
                }
                Adt0::C8(_) => {
                    f71(v4, v3, v2)
                }
                Adt0::C9 => {
                    f71(v4, v3, v2)
                }
                Adt0::C10(_, _) => {
                    f71(v4, v3, v2)
                }
                Adt0::C11(_, v6, r632, r633) => {
                    let v7 = (*r632).clone();
                    let v8 = (*r633).clone();
                    Ok(f94(f71(v4.clone(), v7, v2.clone())?, f95(v4, v8, List::cons(v6, v2), v3)?))
                }
            }
        }
    }
}

pub fn f96(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h635, _) = v2;
        h635
    };
    let a636 = v3;
    let a637 = 0u64;
    if nat_eq(a636, a637) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a638 = v3;
            let a639 = v1;
            let a640 = 0u64;
            nat_quot(a638, a639, a640)
        }, {
            let a645 = f74(v0, {
                let a641 = v3;
                let a642 = v1;
                let a643 = 0u64;
                nat_rem(a641, a642, a643)
            });
            let a646 = {
                let (_, h644) = v2;
                h644
            };
            append_bytes(a645, a646)
        }))
    }
}

pub fn f97(v0: Adt1) -> Bytes {
    let m647 = v0;
    match m647 {
        Adt1::C0(_) => {
            Bytes::lit(&[])
        }
        Adt1::C1(_) => {
            Bytes::lit(&[])
        }
        Adt1::C2(_) => {
            Bytes::lit(&[])
        }
        Adt1::C3(_) => {
            Bytes::lit(&[])
        }
        Adt1::C4(_, _) => {
            Bytes::lit(&[])
        }
        Adt1::C5 => {
            Bytes::lit(&[])
        }
        Adt1::C6(r648, r649) => {
            let v7 = (*r648).clone();
            let v8 = (*r649).clone();
            f78(f78(f77(Str::lit(" ")), f79(v7)), f97(v8))
        }
    }
}

pub fn f98(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m653 = {
        let a650 = v0;
        let a651 = v1;
        let a652 = v2;
        slice_bytes(a650, a651, a652)
    };
    match m653 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f99(v0: Fn7, p657: Option<Bytes>, p658: List<(Bytes, Bytes)>) -> Option<Bytes> {
    let m656 = (p657, p658);
    {
        let mut m656 = m656;
        loop {
            let step = {
                let (v1, v2) = m656;
                let m654 = v2;
                match m654.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<(Bytes, Bytes)>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<(Bytes, Bytes)>), Option<Bytes>>(({
                            let c655: Fn7 = v0.clone();
                            c655.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m656 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f100(v0: Bytes, v1: Option<Bytes>, v2: (Bytes, Bytes)) -> Option<Bytes> {
    let m659 = v1.clone();
    match m659 {
        None => {
            let a661 = {
                let (h660, _) = v2;
                h660
            };
            let a662 = v0;
            if equal(a661, a662) {
                Some({
                    let (_, h663) = v2;
                    h663
                })
            } else {
                None::<Bytes>
            }
        }
        Some(_) => {
            v1
        }
    }
}

pub fn f101(v0: u64) -> Bytes {
    let a664 = v0;
    let a665 = 1u64;
    if nat_eq(a664, a665) {
        f77(Str::lit("implicit"))
    } else {
        let a666 = v0;
        let a667 = 2u64;
        if nat_eq(a666, a667) {
            f77(Str::lit("instance"))
        } else {
            f77(Str::lit("explicit"))
        }
    }
}

pub fn f102(v0: Fn8, p671: List<Adt9>, p672: List<Adt9>) -> List<Adt9> {
    let m670 = (p671, p672);
    {
        let mut m670 = m670;
        loop {
            let step = {
                let (v1, v2) = m670;
                let m668 = v2;
                match m668.uncons() {
                    None => {
                        Err::<(List<Adt9>, List<Adt9>), List<Adt9>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt9>, List<Adt9>), List<Adt9>>(({
                            let c669: Fn8 = v0.clone();
                            c669.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m670 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f103(v0: List<Adt9>, v1: Adt9) -> List<Adt9> {
    List::cons(v1, v0)
}

pub fn f104(v0: Adt4, v1: Adt6, v2: Adt0, v3: Bytes, v4: Adt0, v5: List<Bytes>) -> R<Adt7> {
    let m673 = v0;
    match m673 {
        Adt4::C0 => {
            Ok(Adt7::C0(v2, f8(v1)))
        }
        Adt4::C1(r725) => {
            let v6 = (*r725).clone();
            let m674 = v2;
            match m674 {
                Adt0::C0 => {
                    Ok(Adt7::C0(Adt0::C0, v1))
                }
                Adt0::C1(v38) => {
                    Ok(Adt7::C0(Adt0::C1(v38), v1))
                }
                Adt0::C2(v39, v40, v41) => {
                    Ok(Adt7::C0(Adt0::C2(v39, v40, v41), v1))
                }
                Adt0::C3(v7) => {
                    let a675 = v7.clone();
                    let a676 = v3;
                    if equal(a675, a676) {
                        Ok(Adt7::C0(v4, v1))
                    } else {
                        Ok(Adt7::C0(Adt0::C3(v7), v1))
                    }
                }
                Adt0::C4(r712, r713) => {
                    let v8 = (*r712).clone();
                    let v9 = (*r713).clone();
                    let v10: Adt7 = f104(v6.clone(), v1, v8, v3.clone(), v4.clone(), v5.clone())?;
                    let v11: Adt7 = f104(v6, match v10.clone() {
                        Adt7::C0(_, h677) => {
                            h677
                        }
                    }, v9, v3, v4, v5)?;
                    Ok(Adt7::C0(Adt0::C4(std::rc::Rc::new(match v10 {
                        Adt7::C0(h678, _) => {
                            h678
                        }
                    }), std::rc::Rc::new(match v11.clone() {
                        Adt7::C0(h679, _) => {
                            h679
                        }
                    })), match v11 {
                        Adt7::C0(_, h680) => {
                            h680
                        }
                    }))
                }
                Adt0::C5(r714, r715) => {
                    let v16 = (*r714).clone();
                    let v17 = (*r715).clone();
                    let v18: Adt12 = f110(v6.clone(), v1, v16, v3.clone(), v4.clone(), v5.clone(), List::<(Bytes, Bytes)>::nil(), false)?;
                    let v19: Adt7 = f111(v6, v18.clone(), v17, v3, v4, v5)?;
                    Ok(Adt7::C0(Adt0::C5(std::rc::Rc::new(match v18 {
                        Adt12::C0(h685, _, _, _) => {
                            h685
                        }
                    }), std::rc::Rc::new(match v19.clone() {
                        Adt7::C0(h686, _) => {
                            h686
                        }
                    })), match v19 {
                        Adt7::C0(_, h687) => {
                            h687
                        }
                    }))
                }
                Adt0::C6(r716, r717) => {
                    let v20 = (*r716).clone();
                    let v21 = (*r717).clone();
                    let v22: Adt12 = f110(v6.clone(), v1, v20, v3.clone(), v4.clone(), v5.clone(), List::<(Bytes, Bytes)>::nil(), false)?;
                    let v23: Adt7 = f111(v6, v22.clone(), v21, v3, v4, v5)?;
                    Ok(Adt7::C0(Adt0::C6(std::rc::Rc::new(match v22 {
                        Adt12::C0(h688, _, _, _) => {
                            h688
                        }
                    }), std::rc::Rc::new(match v23.clone() {
                        Adt7::C0(h689, _) => {
                            h689
                        }
                    })), match v23 {
                        Adt7::C0(_, h690) => {
                            h690
                        }
                    }))
                }
                Adt0::C7(v24, r718, r719, r720) => {
                    let v25 = (*r718).clone();
                    let v26 = (*r719).clone();
                    let v27 = (*r720).clone();
                    let v28: Adt7 = f104(v6.clone(), v1, v25, v3.clone(), v4.clone(), v5.clone())?;
                    let v29: Adt7 = f104(v6.clone(), match v28.clone() {
                        Adt7::C0(_, h691) => {
                            h691
                        }
                    }, v26, v3.clone(), v4.clone(), v5.clone())?;
                    let a692 = v24.clone();
                    let a693 = v3.clone();
                    if equal(a692, a693) {
                        Ok(Adt7::C0(Adt0::C7(v24, std::rc::Rc::new(match v28 {
                            Adt7::C0(h694, _) => {
                                h694
                            }
                        }), std::rc::Rc::new(match v29.clone() {
                            Adt7::C0(h695, _) => {
                                h695
                            }
                        }), std::rc::Rc::new(v27)), match v29 {
                            Adt7::C0(_, h696) => {
                                h696
                            }
                        }))
                    } else {
                        if f93(v5.clone(), v24.clone())? {
                            let v30: (Bytes, Adt6) = f112(match v29.clone() {
                                Adt7::C0(_, h697) => {
                                    h697
                                }
                            }, v24.clone())?;
                            let v31: Adt7 = f104(v6.clone(), {
                                let (_, h698) = v30.clone();
                                h698
                            }, v27, v24, Adt0::C3({
                                let (h699, _) = v30.clone();
                                h699
                            }), List::<Bytes>::nil())?;
                            let v32: Adt7 = f104(v6, match v31.clone() {
                                Adt7::C0(_, h700) => {
                                    h700
                                }
                            }, match v31 {
                                Adt7::C0(h701, _) => {
                                    h701
                                }
                            }, v3, v4, v5)?;
                            Ok(Adt7::C0(Adt0::C7({
                                let (h702, _) = v30;
                                h702
                            }, std::rc::Rc::new(match v28 {
                                Adt7::C0(h703, _) => {
                                    h703
                                }
                            }), std::rc::Rc::new(match v29 {
                                Adt7::C0(h704, _) => {
                                    h704
                                }
                            }), std::rc::Rc::new(match v32.clone() {
                                Adt7::C0(h705, _) => {
                                    h705
                                }
                            })), match v32 {
                                Adt7::C0(_, h706) => {
                                    h706
                                }
                            }))
                        } else {
                            let v33: Adt7 = f104(v6, match v29.clone() {
                                Adt7::C0(_, h707) => {
                                    h707
                                }
                            }, v27, v3, v4, v5)?;
                            Ok(Adt7::C0(Adt0::C7(v24, std::rc::Rc::new(match v28 {
                                Adt7::C0(h708, _) => {
                                    h708
                                }
                            }), std::rc::Rc::new(match v29 {
                                Adt7::C0(h709, _) => {
                                    h709
                                }
                            }), std::rc::Rc::new(match v33.clone() {
                                Adt7::C0(h710, _) => {
                                    h710
                                }
                            })), match v33 {
                                Adt7::C0(_, h711) => {
                                    h711
                                }
                            }))
                        }
                    }
                }
                Adt0::C8(v42) => {
                    Ok(Adt7::C0(Adt0::C8(v42), v1))
                }
                Adt0::C9 => {
                    Ok(Adt7::C0(Adt0::C9, v1))
                }
                Adt0::C10(r721, r722) => {
                    let v12 = (*r721).clone();
                    let v13 = (*r722).clone();
                    let v14: Adt7 = f104(v6.clone(), v1, v12, v3.clone(), v4.clone(), v5.clone())?;
                    let v15: Adt7 = f104(v6, match v14.clone() {
                        Adt7::C0(_, h681) => {
                            h681
                        }
                    }, v13, v3, v4, v5)?;
                    Ok(Adt7::C0(Adt0::C10(std::rc::Rc::new(match v14 {
                        Adt7::C0(h682, _) => {
                            h682
                        }
                    }), std::rc::Rc::new(match v15.clone() {
                        Adt7::C0(h683, _) => {
                            h683
                        }
                    })), match v15 {
                        Adt7::C0(_, h684) => {
                            h684
                        }
                    }))
                }
                Adt0::C11(v34, v35, r723, r724) => {
                    let v36 = (*r723).clone();
                    let v37 = (*r724).clone();
                    Ok(Adt7::C0(Adt0::C11(v34, v35, std::rc::Rc::new(v36), std::rc::Rc::new(v37)), v1))
                }
            }
        }
    }
}

pub fn f105(v0: List<Adt9>, v1: Adt9) -> List<Adt9> {
    List::cons(v1, v0)
}

pub fn f106(v0: List<Adt9>, v1: Adt9) -> List<Adt9> {
    List::cons(v1, v0)
}

pub fn f107(v0: u64) -> Bytes {
    f101(v0)
}

pub fn f108(v0: Bytes, v1: bool, v2: Bytes) -> bool {
    if v1 {
        true
    } else {
        let a726 = v2;
        let a727 = v0;
        equal(a726, a727)
    }
}

pub fn f109(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f110(v0: Adt4, v1: Adt6, v2: Adt0, v3: Bytes, v4: Adt0, v5: List<Bytes>, v6: List<(Bytes, Bytes)>, v7: bool) -> R<Adt12> {
    let m728 = v0;
    match m728 {
        Adt4::C0 => {
            Ok(Adt12::C0(v2, v6, v7, f8(v1)))
        }
        Adt4::C1(r755) => {
            let v8 = (*r755).clone();
            let m729 = v2.clone();
            match m729 {
                Adt0::C0 => {
                    Ok(Adt12::C0(Adt0::C9, v6, v7, v1))
                }
                Adt0::C1(_) => {
                    Ok(Adt12::C0(Adt0::C9, v6, v7, v1))
                }
                Adt0::C2(_, _, _) => {
                    Ok(Adt12::C0(Adt0::C9, v6, v7, v1))
                }
                Adt0::C3(_) => {
                    Ok(Adt12::C0(Adt0::C9, v6, v7, v1))
                }
                Adt0::C4(_, _) => {
                    Ok(Adt12::C0(Adt0::C9, v6, v7, v1))
                }
                Adt0::C5(_, _) => {
                    Ok(Adt12::C0(Adt0::C9, v6, v7, v1))
                }
                Adt0::C6(_, _) => {
                    Ok(Adt12::C0(Adt0::C9, v6, v7, v1))
                }
                Adt0::C7(_, _, _, _) => {
                    Ok(Adt12::C0(Adt0::C9, v6, v7, v1))
                }
                Adt0::C8(_) => {
                    Ok(Adt12::C0(Adt0::C9, v6, v7, v1))
                }
                Adt0::C9 => {
                    Ok(Adt12::C0(Adt0::C9, v6, v7, v1))
                }
                Adt0::C10(_, _) => {
                    Ok(Adt12::C0(Adt0::C9, v6, v7, v1))
                }
                Adt0::C11(v9, v10, r753, r754) => {
                    let v11 = (*r753).clone();
                    let v12 = (*r754).clone();
                    let v13: Adt7 = f111(v8.clone(), Adt12::C0(v2, v6.clone(), v7, v1), v11, v3.clone(), v4.clone(), v5.clone())?;
                    let v14: Adt7 = v13;
                    let a730 = v10.clone();
                    let a731 = v3.clone();
                    if equal(a730, a731) {
                        let v15: Adt12 = f110(v8, match v14.clone() {
                            Adt7::C0(_, h732) => {
                                h732
                            }
                        }, v12, v3, v4, v5, v6, true)?;
                        Ok(Adt12::C0(Adt0::C11(v9, v10, std::rc::Rc::new(match v14 {
                            Adt7::C0(h733, _) => {
                                h733
                            }
                        }), std::rc::Rc::new(match v15.clone() {
                            Adt12::C0(h734, _, _, _) => {
                                h734
                            }
                        })), match v15.clone() {
                            Adt12::C0(_, h735, _, _) => {
                                h735
                            }
                        }, match v15.clone() {
                            Adt12::C0(_, _, h736, _) => {
                                h736
                            }
                        }, match v15 {
                            Adt12::C0(_, _, _, h737) => {
                                h737
                            }
                        }))
                    } else {
                        if f93(v5.clone(), v10.clone())? {
                            let v16: (Bytes, Adt6) = f112(match v14.clone() {
                                Adt7::C0(_, h738) => {
                                    h738
                                }
                            }, v10.clone())?;
                            let v17: Adt12 = f110(v8, {
                                let (_, h739) = v16.clone();
                                h739
                            }, v12, v3, v4, v5, List::cons((v10, {
                                let (h740, _) = v16.clone();
                                h740
                            }), v6), v7)?;
                            Ok(Adt12::C0(Adt0::C11(v9, {
                                let (h741, _) = v16;
                                h741
                            }, std::rc::Rc::new(match v14 {
                                Adt7::C0(h742, _) => {
                                    h742
                                }
                            }), std::rc::Rc::new(match v17.clone() {
                                Adt12::C0(h743, _, _, _) => {
                                    h743
                                }
                            })), match v17.clone() {
                                Adt12::C0(_, h744, _, _) => {
                                    h744
                                }
                            }, match v17.clone() {
                                Adt12::C0(_, _, h745, _) => {
                                    h745
                                }
                            }, match v17 {
                                Adt12::C0(_, _, _, h746) => {
                                    h746
                                }
                            }))
                        } else {
                            let v18: Adt12 = f110(v8, match v14.clone() {
                                Adt7::C0(_, h747) => {
                                    h747
                                }
                            }, v12, v3, v4, v5, v6, v7)?;
                            Ok(Adt12::C0(Adt0::C11(v9, v10, std::rc::Rc::new(match v14 {
                                Adt7::C0(h748, _) => {
                                    h748
                                }
                            }), std::rc::Rc::new(match v18.clone() {
                                Adt12::C0(h749, _, _, _) => {
                                    h749
                                }
                            })), match v18.clone() {
                                Adt12::C0(_, h750, _, _) => {
                                    h750
                                }
                            }, match v18.clone() {
                                Adt12::C0(_, _, h751, _) => {
                                    h751
                                }
                            }, match v18 {
                                Adt12::C0(_, _, _, h752) => {
                                    h752
                                }
                            }))
                        }
                    }
                }
            }
        }
    }
}

pub fn f111(v0: Adt4, v1: Adt12, v2: Adt0, v3: Bytes, v4: Adt0, v5: List<Bytes>) -> R<Adt7> {
    let m756 = v0;
    match m756 {
        Adt4::C0 => {
            Ok(Adt7::C0(v2, f8(match v1 {
                Adt12::C0(_, _, _, h757) => {
                    h757
                }
            })))
        }
        Adt4::C1(r763) => {
            let v6 = (*r763).clone();
            let v7: Adt7 = f113(v6.clone(), match v1.clone() {
                Adt12::C0(_, _, _, h758) => {
                    h758
                }
            }, v2, f114(match v1.clone() {
                Adt12::C0(_, h759, _, _) => {
                    h759
                }
            }))?;
            if match v1 {
                Adt12::C0(_, _, h760, _) => {
                    h760
                }
            } {
                Ok(v7)
            } else {
                f104(v6, match v7.clone() {
                    Adt7::C0(_, h761) => {
                        h761
                    }
                }, match v7 {
                    Adt7::C0(h762, _) => {
                        h762
                    }
                }, v3, v4, v5)
            }
        }
    }
}

pub fn f112(v0: Adt6, v1: Bytes) -> R<(Bytes, Adt6)> {
    let v2: u64 = {
        let a765 = match v0.clone() {
            Adt6::C0(_, _, _, h764, _, _, _) => {
                h764
            }
        };
        let a766 = 1u64;
        nat_add(a765, a766)?
    };
    Ok((f16(f16(v1, Bytes::lit(&[0x23])), f41(v2)), f13(v0.clone(), match v0.clone() {
        Adt6::C0(h767, _, _, _, _, _, _) => {
            h767
        }
    }, match v0.clone() {
        Adt6::C0(_, h768, _, _, _, _, _) => {
            h768
        }
    }, match v0.clone() {
        Adt6::C0(_, _, h769, _, _, _, _) => {
            h769
        }
    }, v2, match v0 {
        Adt6::C0(_, _, _, _, h770, _, _) => {
            h770
        }
    })))
}

pub fn f113(p779: Adt4, p780: Adt6, p781: Adt0, p782: List<(Bytes, Bytes)>) -> R<Adt7> {
    let m778 = (p779, (p780, (p781, p782)));
    {
        let mut m778 = m778;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m778;
                let m771 = v0;
                match m771 {
                    Adt4::C0 => {
                        Err::<(Adt4, (Adt6, (Adt0, List<(Bytes, Bytes)>))), R<Adt7>>(Ok(Adt7::C0(v2, f8(v1))))
                    }
                    Adt4::C1(r777) => {
                        let v4 = (*r777).clone();
                        let m772 = v3;
                        match m772.uncons() {
                            None => {
                                Err::<(Adt4, (Adt6, (Adt0, List<(Bytes, Bytes)>))), R<Adt7>>(Ok(Adt7::C0(v2, v1)))
                            }
                            Some((v5, v6)) => {
                                let v7: Adt7 = f104(v4.clone(), v1, v2, {
                                    let (h773, _) = v5;
                                    h773
                                }, Adt0::C3({
                                    let (_, h774) = v5;
                                    h774
                                }), List::<Bytes>::nil())?;
                                Ok::<(Adt4, (Adt6, (Adt0, List<(Bytes, Bytes)>))), R<Adt7>>((v4, (match v7.clone() {
                                    Adt7::C0(_, h775) => {
                                        h775
                                    }
                                }, (match v7 {
                                    Adt7::C0(h776, _) => {
                                        h776
                                    }
                                }, v6))))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m778 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f114(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f115(Fn9::F116, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f115(v0: Fn9, p786: List<(Bytes, Bytes)>, p787: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m785 = (p786, p787);
    {
        let mut m785 = m785;
        loop {
            let step = {
                let (v1, v2) = m785;
                let m783 = v2;
                match m783.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c784: Fn9 = v0.clone();
                            c784.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m785 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f116(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn run(p0: Adt0, p1: List<Adt2>, p2: u64, p3: u64) -> R<Result<Adt3, Bytes>> {
    f0(p0, p1, p2, p3)
}
