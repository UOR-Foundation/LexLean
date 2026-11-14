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
    C0(Option<Bytes>, Adt6),
}

#[derive(Clone)]
pub enum Adt8 {
    C0(Adt0, Adt6),
}

#[derive(Clone)]
pub enum Adt9 {
    C0(u64, Bytes, Adt0),
}

#[derive(Clone)]
pub enum Adt10 {
    C0(List<Adt9>, Adt0),
}

#[derive(Clone)]
pub enum Adt11 {
    C0(List<Adt9>, Adt6),
}

#[derive(Clone)]
pub enum Adt12 {
    C0(Adt0, List<(Bytes, Bytes)>, bool, Adt6),
}

#[derive(Clone)]
pub enum Fn0 {
    F12,
}

impl Fn0 {
    pub fn apply(&self, p0: Adt4) -> Option<Adt4> {
        match self {
            Fn0::F12 => f12(p0),
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
    F67,
    F109,
}

impl Fn3 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn3::F67 => f67(p0, p1),
            Fn3::F109 => f109(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F75,
    F108(Bytes),
}

impl Fn4 {
    pub fn apply(&self, p0: bool, p1: Bytes) -> R<bool> {
        match self {
            Fn4::F75 => f75(p0, p1),
            Fn4::F108(k0) => Ok(f108(k0.clone(), p0, p1)),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F86(u64, Bytes),
}

impl Fn5 {
    pub fn apply(&self, p0: (u64, Bytes)) -> Option<(u64, Bytes)> {
        match self {
            Fn5::F86(k0, k1) => f86(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F88,
}

impl Fn6 {
    pub fn apply(&self, p0: Adt0, p1: Adt9) -> Adt0 {
        match self {
            Fn6::F88 => f88(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F94,
    F95,
    F106,
}

impl Fn7 {
    pub fn apply(&self, p0: List<Adt9>, p1: Adt9) -> List<Adt9> {
        match self {
            Fn7::F94 => f94(p0, p1),
            Fn7::F95 => f95(p0, p1),
            Fn7::F106 => f106(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F103(Bytes),
}

impl Fn8 {
    pub fn apply(&self, p0: Option<Bytes>, p1: (Bytes, Bytes)) -> Option<Bytes> {
        match self {
            Fn8::F103(k0) => f103(k0.clone(), p0, p1),
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

pub fn f0(v0: Adt0, v1: Option<Adt0>, v2: List<Adt2>, v3: u64, v4: u64) -> R<Result<Adt3, Bytes>> {
    let v5: Adt4 = f1(v4);
    let v6: Adt5 = f2(v5.clone(), f3(v2, v3), v0.clone(), List::<(Bytes, Adt0)>::nil())?;
    let m1 = v1;
    match m1 {
        None => {
            f4(v5, v6)
        }
        Some(v7) => {
            let m4 = match v6.clone() {
                Adt5::C0(_, _, h3) => {
                    h3
                }
            };
            if if match m4 {
                Adt6::C0(_, _, _, _, h2, _, _) => {
                    h2
                }
            } {
                true
            } else {
                let m6 = match v6.clone() {
                    Adt5::C0(_, h5, _) => {
                        h5
                    }
                };
                match m6 {
                    None => {
                        false
                    }
                    Some(_) => {
                        true
                    }
                }
            } {
                f4(v5, v6)
            } else {
                let v9: Adt7 = f5(v5.clone(), match v6.clone() {
                    Adt5::C0(_, _, h7) => {
                        h7
                    }
                }, v7.clone(), match v6.clone() {
                    Adt5::C0(h8, _, _) => {
                        h8
                    }
                })?;
                let m10 = match v9.clone() {
                    Adt7::C0(h9, _) => {
                        h9
                    }
                };
                match m10 {
                    None => {
                        f4(v5, f10(match v6 {
                            Adt5::C0(h12, _, _) => {
                                h12
                            }
                        }, match v9 {
                            Adt7::C0(_, h13) => {
                                h13
                            }
                        }))
                    }
                    Some(v10) => {
                        let v11: Bytes = f6(v0)?;
                        let v12: Bytes = f6(v7)?;
                        f4(v5, f7(match v9 {
                            Adt7::C0(_, h11) => {
                                h11
                            }
                        }, f8(f8(f8(f8(f8(f9(Str::lit("defined value ")), v11), f9(Str::lit(" does not match the signature "))), v12), f9(Str::lit(": "))), v10)))
                    }
                }
            }
        }
    }
}

pub fn f1(v0: u64) -> Adt4 {
    let (h14, _) = f11(Fn0::F12, v0, Adt4::C0);
    h14
}

pub fn f2(v0: Adt4, v1: Adt6, v2: Adt0, v3: List<(Bytes, Adt0)>) -> R<Adt5> {
    let m15 = v0;
    match m15 {
        Adt4::C0 => {
            Ok(f7(f13(v1), Bytes::lit(&[])))
        }
        Adt4::C1(r56) => {
            let v4 = (*r56).clone();
            let m16 = v2.clone();
            match m16 {
                Adt0::C0 => {
                    Ok(f10(Adt0::C1(Adt1::C0(1u64)), v1))
                }
                Adt0::C1(v5) => {
                    Ok(f10(Adt0::C1(Adt1::C2(std::rc::Rc::new(v5))), v1))
                }
                Adt0::C2(v6, v7, _) => {
                    let v9: (u64, bool) = f14(v1.clone(), v6.clone(), v7.clone());
                    let v10: u64 = {
                        let (h17, _) = v9;
                        h17
                    };
                    let a18 = v10;
                    let a19 = 0u64;
                    if nat_eq(a18, a19) {
                        Ok(f7(v1, f8(f8(f8(f8(f9(Str::lit("`")), v6), f9(Str::lit("::"))), v7), f9(Str::lit("` does not resolve")))))
                    } else {
                        let a20 = v10;
                        let a21 = 1u64;
                        if nat_eq(a20, a21) {
                            f15(v1)
                        } else {
                            let a22 = v10;
                            let a23 = 2u64;
                            if nat_eq(a22, a23) {
                                Ok(f7(v1, f8(f8(f8(f8(f9(Str::lit("`")), v6), f9(Str::lit("::"))), v7), f9(Str::lit("` has no signature and cannot appear in an expression")))))
                            } else {
                                Ok(f10(f16(v1.clone(), v6, v7), v1))
                            }
                        }
                    }
                }
                Adt0::C3(v11) => {
                    if f17(v11.clone())? {
                        f15(v1)
                    } else {
                        let m24 = f18(v3, v11.clone());
                        match m24 {
                            None => {
                                Ok(f7(v1, f8(f8(f9(Str::lit("local `")), v11), f9(Str::lit("` is not in scope")))))
                            }
                            Some(v12) => {
                                Ok(f10(v12, v1))
                            }
                        }
                    }
                }
                Adt0::C4(r47, r48) => {
                    let v15 = (*r47).clone();
                    let v16 = (*r48).clone();
                    let v17: u64 = f20(v15.clone());
                    let a27 = v17;
                    let a28 = 2u64;
                    if if nat_lt(a27, a28) {
                        true
                    } else {
                        let a29 = v17;
                        let a30 = 8u64;
                        nat_eq(a29, a30)
                    } {
                        let v18: Bytes = f6(v2)?;
                        let v19: Bytes = f6(v15)?;
                        Ok(f7(v1, f8(f8(f8(v18, f9(Str::lit(" applies "))), v19), f9(Str::lit(", which is not a function")))))
                    } else {
                        let v20: Adt5 = f2(v4.clone(), v1, v15.clone(), v3.clone())?;
                        let m32 = match v20.clone() {
                            Adt5::C0(_, h31, _) => {
                                h31
                            }
                        };
                        match m32 {
                            None => {
                                let v22: List<Adt0> = f21(v4.clone(), v16);
                                f22(v4, match v20.clone() {
                                    Adt5::C0(_, _, h33) => {
                                        h33
                                    }
                                }, v2, v15, v22.clone(), {
                                    let a34 = v22;
                                    length_list(a34)
                                }, match v20 {
                                    Adt5::C0(h35, _, _) => {
                                        h35
                                    }
                                }, 0u64, v3)
                            }
                            Some(_) => {
                                Ok(v20)
                            }
                        }
                    }
                }
                Adt0::C5(r49, r50) => {
                    let v23 = (*r49).clone();
                    let v24 = (*r50).clone();
                    f23(v4, v1, v23.clone(), v24, v3, false, v23)
                }
                Adt0::C6(r51, r52) => {
                    let v25 = (*r51).clone();
                    let v26 = (*r52).clone();
                    f23(v4, v1, v25.clone(), v26, v3, true, v25)
                }
                Adt0::C7(v27, r53, r54, r55) => {
                    let v28 = (*r53).clone();
                    let v29 = (*r54).clone();
                    let v30 = (*r55).clone();
                    let v31: Adt5 = f24(v4.clone(), v1, v28.clone(), v3.clone())?;
                    let m37 = match v31.clone() {
                        Adt5::C0(_, h36, _) => {
                            h36
                        }
                    };
                    match m37 {
                        None => {
                            let v33: Adt5 = f2(v4.clone(), match v31 {
                                Adt5::C0(_, _, h38) => {
                                    h38
                                }
                            }, v29, v3.clone())?;
                            let m40 = match v33.clone() {
                                Adt5::C0(_, h39, _) => {
                                    h39
                                }
                            };
                            match m40 {
                                None => {
                                    let v35: Adt7 = f5(v4.clone(), match v33.clone() {
                                        Adt5::C0(_, _, h41) => {
                                            h41
                                        }
                                    }, v28.clone(), match v33 {
                                        Adt5::C0(h42, _, _) => {
                                            h42
                                        }
                                    })?;
                                    let m44 = match v35.clone() {
                                        Adt7::C0(h43, _) => {
                                            h43
                                        }
                                    };
                                    match m44 {
                                        None => {
                                            f2(v4, match v35 {
                                                Adt7::C0(_, h46) => {
                                                    h46
                                                }
                                            }, v30, List::cons((v27, v28), v3))
                                        }
                                        Some(v36) => {
                                            Ok(f7(match v35 {
                                                Adt7::C0(_, h45) => {
                                                    h45
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
                    let v14: Adt8 = f19(v1)?;
                    Ok(f10(match v14.clone() {
                        Adt8::C0(h25, _) => {
                            h25
                        }
                    }, match v14 {
                        Adt8::C0(_, h26) => {
                            h26
                        }
                    }))
                }
                Adt0::C9 => {
                    Ok(f10(Adt0::C0, v1))
                }
                Adt0::C10(_, _) => {
                    Ok(f10(Adt0::C0, v1))
                }
                Adt0::C11(_, _, _, _) => {
                    Ok(f10(Adt0::C0, v1))
                }
            }
        }
    }
}

pub fn f3(v0: List<Adt2>, v1: u64) -> Adt6 {
    Adt6::C0(List::<(Bytes, Adt0)>::nil(), 0u64, List::<Bytes>::nil(), 0u64, false, v0, v1)
}

pub fn f4(v0: Adt4, v1: Adt5) -> R<Result<Adt3, Bytes>> {
    let m59 = match v1.clone() {
        Adt5::C0(_, _, h58) => {
            h58
        }
    };
    if match m59 {
        Adt6::C0(_, _, _, _, h57, _, _) => {
            h57
        }
    } {
        Ok(Err::<Adt3, Bytes>(f9(Str::lit("the checker's bound on nesting is exhausted"))))
    } else {
        let m61 = match v1.clone() {
            Adt5::C0(_, h60, _) => {
                h60
            }
        };
        match m61 {
            None => {
                f25(v0, match v1.clone() {
                    Adt5::C0(_, _, h62) => {
                        h62
                    }
                }, match v1 {
                    Adt5::C0(h63, _, _) => {
                        h63
                    }
                })
            }
            Some(v2) => {
                Ok(Err::<Adt3, Bytes>(v2))
            }
        }
    }
}

pub fn f5(v0: Adt4, v1: Adt6, v2: Adt0, v3: Adt0) -> R<Adt7> {
    let m64 = v0;
    match m64 {
        Adt4::C0 => {
            Ok(f26(f13(v1)))
        }
        Adt4::C1(r75) => {
            let v4 = (*r75).clone();
            let v5: Adt8 = f27(v4.clone(), v1, v2)?;
            let v6: Adt8 = f27(v4.clone(), match v5.clone() {
                Adt8::C0(_, h65) => {
                    h65
                }
            }, v3)?;
            let v7: Adt0 = match v5 {
                Adt8::C0(h66, _) => {
                    h66
                }
            };
            let v8: Adt0 = match v6.clone() {
                Adt8::C0(h67, _) => {
                    h67
                }
            };
            let v9: Adt6 = match v6 {
                Adt8::C0(_, h68) => {
                    h68
                }
            };
            let m69 = f28(v7.clone())?;
            match m69 {
                None => {
                    let m74 = f28(v8.clone())?;
                    match m74 {
                        None => {
                            f31(v4, v9, v7, v8)
                        }
                        Some(v11) => {
                            Ok(f26(f30(v9, v11, v7)))
                        }
                    }
                }
                Some(v10) => {
                    let a70 = f29(v8.clone());
                    let a71 = v10.clone();
                    if equal(a70, a71) {
                        let a72 = f20(v8.clone());
                        let a73 = 3u64;
                        if nat_eq(a72, a73) {
                            Ok(f26(v9))
                        } else {
                            Ok(f26(f30(v9, v10, v8)))
                        }
                    } else {
                        Ok(f26(f30(v9, v10, v8)))
                    }
                }
            }
        }
    }
}

pub fn f6(v0: Adt0) -> R<Bytes> {
    Ok(f8(f8(f9(Str::lit("`")), f32(v0, false)?), f9(Str::lit("`"))))
}

pub fn f7(v0: Adt6, v1: Bytes) -> Adt5 {
    Adt5::C0(Adt0::C0, Some(v1), v0)
}

pub fn f8(v0: Bytes, v1: Bytes) -> Bytes {
    let a76 = v0;
    let a77 = v1;
    append_bytes(a76, a77)
}

pub fn f9(v0: Str) -> Bytes {
    let a78 = v0;
    utf8_encode(a78)
}

pub fn f10(v0: Adt0, v1: Adt6) -> Adt5 {
    Adt5::C0(v0, None::<Bytes>, v1)
}

pub fn f11(v0: Fn0, p83: u64, p84: Adt4) -> (Adt4, bool) {
    let m82 = (p83, p84);
    {
        let mut m82 = m82;
        loop {
            let step = {
                let (v1, v2) = m82;
                let m79 = v1;
                if m79 == 0 {
                    Err::<(u64, Adt4), (Adt4, bool)>((v2, false))
                } else {
                    let v3 = m79 - 1;
                    let m81 = {
                        let c80: Fn0 = v0.clone();
                        c80.apply(v2.clone())
                    };
                    match m81 {
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
                Ok(next) => { m82 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f12(v0: Adt4) -> Option<Adt4> {
    Some(Adt4::C1(std::rc::Rc::new(v0)))
}

pub fn f13(v0: Adt6) -> Adt6 {
    f33(v0.clone(), match v0.clone() {
        Adt6::C0(h85, _, _, _, _, _, _) => {
            h85
        }
    }, match v0.clone() {
        Adt6::C0(_, h86, _, _, _, _, _) => {
            h86
        }
    }, match v0.clone() {
        Adt6::C0(_, _, h87, _, _, _, _) => {
            h87
        }
    }, match v0 {
        Adt6::C0(_, _, _, h88, _, _, _) => {
            h88
        }
    }, true)
}

pub fn f14(v0: Adt6, v1: Bytes, v2: Bytes) -> (u64, bool) {
    let m90 = f34(Fn1::F35(v2, v1), None::<Adt2>, match v0.clone() {
        Adt6::C0(_, _, _, _, _, h89, _) => {
            h89
        }
    });
    match m90 {
        None => {
            (match v0 {
                Adt6::C0(_, _, _, _, _, _, h93) => {
                    h93
                }
            }, false)
        }
        Some(v3) => {
            (match v3.clone() {
                Adt2::C0(_, _, h91, _, _) => {
                    h91
                }
            }, match v3 {
                Adt2::C0(_, _, _, _, h92) => {
                    h92
                }
            })
        }
    }
}

pub fn f15(v0: Adt6) -> R<Adt5> {
    let v1: Adt8 = f36(v0)?;
    Ok(f10(match v1.clone() {
        Adt8::C0(h94, _) => {
            h94
        }
    }, match v1 {
        Adt8::C0(_, h95) => {
            h95
        }
    }))
}

pub fn f16(v0: Adt6, v1: Bytes, v2: Bytes) -> Adt0 {
    let m97 = f34(Fn1::F37(v2, v1), None::<Adt2>, match v0 {
        Adt6::C0(_, _, _, _, _, h96, _) => {
            h96
        }
    });
    match m97 {
        None => {
            Adt0::C0
        }
        Some(v3) => {
            match v3 {
                Adt2::C0(_, _, _, h98, _) => {
                    h98
                }
            }
        }
    }
}

pub fn f17(v0: Bytes) -> R<bool> {
    let a99 = f38(v0, 0u64)?;
    let a100 = 63u64;
    Ok(nat_eq(a99, a100))
}

pub fn f18(v0: List<(Bytes, Adt0)>, v1: Bytes) -> Option<Adt0> {
    f39(Fn2::F40(v1), None::<Adt0>, v0)
}

pub fn f19(v0: Adt6) -> R<Adt8> {
    let v1: Adt8 = f36(v0.clone())?;
    let v2: Adt6 = match v1.clone() {
        Adt8::C0(_, h101) => {
            h101
        }
    };
    Ok(Adt8::C0(match v1 {
        Adt8::C0(h102, _) => {
            h102
        }
    }, f33(v2.clone(), match v2.clone() {
        Adt6::C0(h103, _, _, _, _, _, _) => {
            h103
        }
    }, match v2.clone() {
        Adt6::C0(_, h104, _, _, _, _, _) => {
            h104
        }
    }, List::cons(f8(f9(Str::lit("?")), f41(match v0 {
        Adt6::C0(_, h105, _, _, _, _, _) => {
            h105
        }
    })), match v2.clone() {
        Adt6::C0(_, _, h106, _, _, _, _) => {
            h106
        }
    }), match v2.clone() {
        Adt6::C0(_, _, _, h107, _, _, _) => {
            h107
        }
    }, match v2 {
        Adt6::C0(_, _, _, _, h108, _, _) => {
            h108
        }
    })))
}

pub fn f20(v0: Adt0) -> u64 {
    let m109 = v0;
    match m109 {
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

pub fn f21(v0: Adt4, v1: Adt0) -> List<Adt0> {
    let m110 = v0;
    match m110 {
        Adt4::C0 => {
            List::<Adt0>::nil()
        }
        Adt4::C1(r114) => {
            let v2 = (*r114).clone();
            let m111 = v1;
            match m111 {
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
                Adt0::C10(r112, r113) => {
                    let v3 = (*r112).clone();
                    let v4 = (*r113).clone();
                    List::cons(v3, f21(v2, v4))
                }
                Adt0::C11(_, _, _, _) => {
                    List::<Adt0>::nil()
                }
            }
        }
    }
}

pub fn f22(v0: Adt4, v1: Adt6, v2: Adt0, v3: Adt0, v4: List<Adt0>, v5: u64, v6: Adt0, v7: u64, v8: List<(Bytes, Adt0)>) -> R<Adt5> {
    let m115 = v0;
    match m115 {
        Adt4::C0 => {
            Ok(f7(f13(v1), Bytes::lit(&[])))
        }
        Adt4::C1(r122) => {
            let v9 = (*r122).clone();
            let m116 = v4.clone();
            match m116.uncons() {
                None => {
                    Ok(f10(v6, v1))
                }
                Some((_, _)) => {
                    let v12: Adt8 = f27(v9.clone(), v1, v6)?;
                    let v13: Adt0 = match v12.clone() {
                        Adt8::C0(h117, _) => {
                            h117
                        }
                    };
                    let v14: Adt6 = match v12 {
                        Adt8::C0(_, h118) => {
                            h118
                        }
                    };
                    let m119 = v13.clone();
                    match m119 {
                        Adt0::C0 => {
                            f44(v9, v14, v2, v3, v4, v5, v7, v13, v8)
                        }
                        Adt0::C1(_) => {
                            f44(v9, v14, v2, v3, v4, v5, v7, v13, v8)
                        }
                        Adt0::C2(_, _, _) => {
                            f44(v9, v14, v2, v3, v4, v5, v7, v13, v8)
                        }
                        Adt0::C3(_) => {
                            f44(v9, v14, v2, v3, v4, v5, v7, v13, v8)
                        }
                        Adt0::C4(_, _) => {
                            f44(v9, v14, v2, v3, v4, v5, v7, v13, v8)
                        }
                        Adt0::C5(r120, r121) => {
                            let v15 = (*r120).clone();
                            let v16 = (*r121).clone();
                            f42(v9.clone(), v14, v2, v3, f43(v9, v15), v16, v4, v5, v7, v8)
                        }
                        Adt0::C6(_, _) => {
                            f44(v9, v14, v2, v3, v4, v5, v7, v13, v8)
                        }
                        Adt0::C7(_, _, _, _) => {
                            f44(v9, v14, v2, v3, v4, v5, v7, v13, v8)
                        }
                        Adt0::C8(_) => {
                            f44(v9, v14, v2, v3, v4, v5, v7, v13, v8)
                        }
                        Adt0::C9 => {
                            f44(v9, v14, v2, v3, v4, v5, v7, v13, v8)
                        }
                        Adt0::C10(_, _) => {
                            f44(v9, v14, v2, v3, v4, v5, v7, v13, v8)
                        }
                        Adt0::C11(_, _, _, _) => {
                            f44(v9, v14, v2, v3, v4, v5, v7, v13, v8)
                        }
                    }
                }
            }
        }
    }
}

pub fn f23(v0: Adt4, v1: Adt6, v2: Adt0, v3: Adt0, v4: List<(Bytes, Adt0)>, v5: bool, v6: Adt0) -> R<Adt5> {
    let m123 = v0;
    match m123 {
        Adt4::C0 => {
            Ok(f7(f13(v1), Bytes::lit(&[])))
        }
        Adt4::C1(r137) => {
            let v7 = (*r137).clone();
            let m124 = v2;
            match m124 {
                Adt0::C0 => {
                    f24(v7, v1, v3, v4)
                }
                Adt0::C1(_) => {
                    f24(v7, v1, v3, v4)
                }
                Adt0::C2(_, _, _) => {
                    f24(v7, v1, v3, v4)
                }
                Adt0::C3(_) => {
                    f24(v7, v1, v3, v4)
                }
                Adt0::C4(_, _) => {
                    f24(v7, v1, v3, v4)
                }
                Adt0::C5(_, _) => {
                    f24(v7, v1, v3, v4)
                }
                Adt0::C6(_, _) => {
                    f24(v7, v1, v3, v4)
                }
                Adt0::C7(_, _, _, _) => {
                    f24(v7, v1, v3, v4)
                }
                Adt0::C8(_) => {
                    f24(v7, v1, v3, v4)
                }
                Adt0::C9 => {
                    if v5 {
                        let v15: Adt5 = f2(v7, v1, v3, v4)?;
                        let m132 = match v15.clone() {
                            Adt5::C0(_, h131, _) => {
                                h131
                            }
                        };
                        match m132 {
                            None => {
                                Ok(f10(Adt0::C5(std::rc::Rc::new(v6), std::rc::Rc::new(match v15.clone() {
                                    Adt5::C0(h133, _, _) => {
                                        h133
                                    }
                                })), match v15 {
                                    Adt5::C0(_, _, h134) => {
                                        h134
                                    }
                                }))
                            }
                            Some(_) => {
                                Ok(v15)
                            }
                        }
                    } else {
                        f24(v7, v1, v3, v4)
                    }
                }
                Adt0::C10(_, _) => {
                    f24(v7, v1, v3, v4)
                }
                Adt0::C11(_, v9, r135, r136) => {
                    let v10 = (*r135).clone();
                    let v11 = (*r136).clone();
                    let v12: Adt5 = f24(v7.clone(), v1, v10.clone(), v4.clone())?;
                    let m126 = match v12.clone() {
                        Adt5::C0(_, h125, _) => {
                            h125
                        }
                    };
                    match m126 {
                        None => {
                            let v14: Adt5 = f23(v7, match v12 {
                                Adt5::C0(_, _, h127) => {
                                    h127
                                }
                            }, v11, v3, List::cons((v9, v10), v4), v5, v6)?;
                            Ok(Adt5::C0(match v14.clone() {
                                Adt5::C0(h128, _, _) => {
                                    h128
                                }
                            }, match v14.clone() {
                                Adt5::C0(_, h129, _) => {
                                    h129
                                }
                            }, match v14 {
                                Adt5::C0(_, _, h130) => {
                                    h130
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

pub fn f24(v0: Adt4, v1: Adt6, v2: Adt0, v3: List<(Bytes, Adt0)>) -> R<Adt5> {
    let m138 = v0;
    match m138 {
        Adt4::C0 => {
            Ok(f7(f13(v1), Bytes::lit(&[])))
        }
        Adt4::C1(r143) => {
            let v4 = (*r143).clone();
            let v5: Adt5 = f2(v4.clone(), v1, v2.clone(), v3)?;
            let m140 = match v5.clone() {
                Adt5::C0(_, h139, _) => {
                    h139
                }
            };
            match m140 {
                None => {
                    f45(v4, match v5.clone() {
                        Adt5::C0(_, _, h141) => {
                            h141
                        }
                    }, v2, match v5 {
                        Adt5::C0(h142, _, _) => {
                            h142
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

pub fn f25(v0: Adt4, v1: Adt6, v2: Adt0) -> R<Result<Adt3, Bytes>> {
    let v3: Adt8 = f27(v0.clone(), v1.clone(), v2)?;
    let v4: (List<Option<Adt0>>, Adt6) = f46(v0, match v3.clone() {
        Adt8::C0(_, h144) => {
            h144
        }
    }, f47(match v1 {
        Adt6::C0(_, _, h145, _, _, _, _) => {
            h145
        }
    }))?;
    let m147 = f48(v4.clone());
    if match m147 {
        Adt6::C0(_, _, _, _, h146, _, _) => {
            h146
        }
    } {
        Ok(Err::<Adt3, Bytes>(f9(Str::lit("the checker's bound on nesting is exhausted"))))
    } else {
        Ok(Ok::<Adt3, Bytes>(Adt3::C0(match v3 {
            Adt8::C0(h148, _) => {
                h148
            }
        }, {
            let (h149, _) = v4;
            h149
        })))
    }
}

pub fn f26(v0: Adt6) -> Adt7 {
    Adt7::C0(None::<Bytes>, v0)
}

pub fn f27(v0: Adt4, v1: Adt6, v2: Adt0) -> R<Adt8> {
    let m151 = f49(v0, match v1.clone() {
        Adt6::C0(h150, _, _, _, _, _, _) => {
            h150
        }
    }, v2.clone())?;
    match m151 {
        None => {
            Ok(Adt8::C0(v2, f13(v1)))
        }
        Some(v3) => {
            Ok(Adt8::C0(v3, v1))
        }
    }
}

pub fn f28(v0: Adt0) -> R<Option<Bytes>> {
    let m152 = v0;
    match m152 {
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
            if f17(v1.clone())? {
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

pub fn f29(v0: Adt0) -> Bytes {
    let m153 = v0;
    match m153 {
        Adt0::C0 => {
            Bytes::lit(&[])
        }
        Adt0::C1(_) => {
            Bytes::lit(&[])
        }
        Adt0::C2(v1, v2, _) => {
            f8(f8(v1, f9(Str::lit("::"))), v2)
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

pub fn f30(v0: Adt6, v1: Bytes, v2: Adt0) -> Adt6 {
    f33(v0.clone(), List::cons((v1, v2), match v0.clone() {
        Adt6::C0(h154, _, _, _, _, _, _) => {
            h154
        }
    }), match v0.clone() {
        Adt6::C0(_, h155, _, _, _, _, _) => {
            h155
        }
    }, match v0.clone() {
        Adt6::C0(_, _, h156, _, _, _, _) => {
            h156
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, h157, _, _, _) => {
            h157
        }
    }, match v0 {
        Adt6::C0(_, _, _, _, h158, _, _) => {
            h158
        }
    })
}

pub fn f31(v0: Adt4, v1: Adt6, v2: Adt0, v3: Adt0) -> R<Adt7> {
    let m159 = v0;
    match m159 {
        Adt4::C0 => {
            Ok(f26(f13(v1)))
        }
        Adt4::C1(r177) => {
            let v4 = (*r177).clone();
            let v5: u64 = f50(v4.clone(), v2.clone(), v3.clone());
            let a160 = v5;
            let a161 = 0u64;
            if nat_eq(a160, a161) {
                Ok(f26(v1))
            } else {
                let a162 = v5;
                let a163 = 1u64;
                if nat_eq(a162, a163) {
                    if if f51(v4.clone(), v1.clone(), v2.clone()) {
                        true
                    } else {
                        f51(v4.clone(), v1.clone(), v3.clone())
                    } {
                        Ok(f26(v1))
                    } else {
                        let v6: (Adt0, Adt0) = f52(v2);
                        let v7: (Adt0, Adt0) = f52(v3);
                        let v8: Adt7 = f5(v4.clone(), v1, {
                            let (h164, _) = v6;
                            h164
                        }, {
                            let (h165, _) = v7;
                            h165
                        })?;
                        let m167 = match v8.clone() {
                            Adt7::C0(h166, _) => {
                                h166
                            }
                        };
                        match m167 {
                            None => {
                                f53(v4, match v8 {
                                    Adt7::C0(_, h168) => {
                                        h168
                                    }
                                }, {
                                    let (_, h169) = v6;
                                    h169
                                }, {
                                    let (_, h170) = v7;
                                    h170
                                })
                            }
                            Some(_) => {
                                Ok(v8)
                            }
                        }
                    }
                } else {
                    let a171 = v5;
                    let a172 = 2u64;
                    if nat_eq(a171, a172) {
                        f54(v4, v1, v2, v3)
                    } else {
                        if if f51(v4.clone(), v1.clone(), v2.clone()) {
                            true
                        } else {
                            f51(v4.clone(), v1.clone(), v3.clone())
                        } {
                            Ok(f26(v1))
                        } else {
                            let v10: (bool, Adt6) = f55(v4.clone(), v1, v2.clone())?;
                            if f56(v10.clone()) {
                                Ok(f26({
                                    let (_, h173) = v10;
                                    h173
                                }))
                            } else {
                                let v11: (bool, Adt6) = f55(v4, {
                                    let (_, h174) = v10;
                                    h174
                                }, v3.clone())?;
                                if f56(v11.clone()) {
                                    Ok(f26({
                                        let (_, h175) = v11;
                                        h175
                                    }))
                                } else {
                                    let v12: Bytes = f6(v2)?;
                                    let v13: Bytes = f6(v3)?;
                                    Ok(f57({
                                        let (_, h176) = v11;
                                        h176
                                    }, f8(f8(f8(f9(Str::lit("expected ")), v12), f9(Str::lit(", found "))), v13)))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f32(v0: Adt0, v1: bool) -> R<Bytes> {
    let (h178, _) = f58(v0, v1, List::<(Bytes, Bytes)>::nil(), 0u64)?;
    Ok(h178)
}

pub fn f33(v0: Adt6, v1: List<(Bytes, Adt0)>, v2: u64, v3: List<Bytes>, v4: u64, v5: bool) -> Adt6 {
    Adt6::C0(v1, v2, v3, v4, v5, match v0.clone() {
        Adt6::C0(_, _, _, _, _, h179, _) => {
            h179
        }
    }, match v0 {
        Adt6::C0(_, _, _, _, _, _, h180) => {
            h180
        }
    })
}

pub fn f34(v0: Fn1, p184: Option<Adt2>, p185: List<Adt2>) -> Option<Adt2> {
    let m183 = (p184, p185);
    {
        let mut m183 = m183;
        loop {
            let step = {
                let (v1, v2) = m183;
                let m181 = v2;
                match m181.uncons() {
                    None => {
                        Err::<(Option<Adt2>, List<Adt2>), Option<Adt2>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Adt2>, List<Adt2>), Option<Adt2>>(({
                            let c182: Fn1 = v0.clone();
                            c182.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m183 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f35(v0: Bytes, v1: Bytes, v2: Option<Adt2>, v3: Adt2) -> Option<Adt2> {
    let m186 = v2.clone();
    match m186 {
        None => {
            let a188 = match v3.clone() {
                Adt2::C0(h187, _, _, _, _) => {
                    h187
                }
            };
            let a189 = v1;
            if if equal(a188, a189) {
                let a191 = match v3.clone() {
                    Adt2::C0(_, h190, _, _, _) => {
                        h190
                    }
                };
                let a192 = v0;
                equal(a191, a192)
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

pub fn f36(v0: Adt6) -> R<Adt8> {
    Ok(Adt8::C0(Adt0::C3(f8(f9(Str::lit("?")), f41(match v0.clone() {
        Adt6::C0(_, h193, _, _, _, _, _) => {
            h193
        }
    }))), f33(v0.clone(), match v0.clone() {
        Adt6::C0(h194, _, _, _, _, _, _) => {
            h194
        }
    }, {
        let a196 = match v0.clone() {
            Adt6::C0(_, h195, _, _, _, _, _) => {
                h195
            }
        };
        let a197 = 1u64;
        nat_add(a196, a197)?
    }, match v0.clone() {
        Adt6::C0(_, _, h198, _, _, _, _) => {
            h198
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, h199, _, _, _) => {
            h199
        }
    }, match v0 {
        Adt6::C0(_, _, _, _, h200, _, _) => {
            h200
        }
    })))
}

pub fn f37(v0: Bytes, v1: Bytes, v2: Option<Adt2>, v3: Adt2) -> Option<Adt2> {
    let m201 = v2.clone();
    match m201 {
        None => {
            let a203 = match v3.clone() {
                Adt2::C0(h202, _, _, _, _) => {
                    h202
                }
            };
            let a204 = v1;
            if if equal(a203, a204) {
                let a206 = match v3.clone() {
                    Adt2::C0(_, h205, _, _, _) => {
                        h205
                    }
                };
                let a207 = v0;
                equal(a206, a207)
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
    let m208 = f59(v0, v1);
    match m208 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a253 = {
                let a229 = {
                    let a217 = {
                        let a211 = {
                            let a209 = v3;
                            let a210 = 128u8;
                            fixed_u8::bit_and(a209, a210)
                        };
                        let a212 = 128u8;
                        if equal(a211, a212) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a218 = {
                        let a215 = {
                            let a213 = v3;
                            let a214 = 64u8;
                            fixed_u8::bit_and(a213, a214)
                        };
                        let a216 = 64u8;
                        if equal(a215, a216) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a217, a218)?
                };
                let a230 = {
                    let a227 = {
                        let a221 = {
                            let a219 = v3;
                            let a220 = 32u8;
                            fixed_u8::bit_and(a219, a220)
                        };
                        let a222 = 32u8;
                        if equal(a221, a222) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a228 = {
                        let a225 = {
                            let a223 = v3;
                            let a224 = 16u8;
                            fixed_u8::bit_and(a223, a224)
                        };
                        let a226 = 16u8;
                        if equal(a225, a226) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a227, a228)?
                };
                nat_add(a229, a230)?
            };
            let a254 = {
                let a251 = {
                    let a239 = {
                        let a233 = {
                            let a231 = v3;
                            let a232 = 8u8;
                            fixed_u8::bit_and(a231, a232)
                        };
                        let a234 = 8u8;
                        if equal(a233, a234) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a240 = {
                        let a237 = {
                            let a235 = v3;
                            let a236 = 4u8;
                            fixed_u8::bit_and(a235, a236)
                        };
                        let a238 = 4u8;
                        if equal(a237, a238) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a239, a240)?
                };
                let a252 = {
                    let a249 = {
                        let a243 = {
                            let a241 = v3;
                            let a242 = 2u8;
                            fixed_u8::bit_and(a241, a242)
                        };
                        let a244 = 2u8;
                        if equal(a243, a244) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a250 = {
                        let a247 = {
                            let a245 = v3;
                            let a246 = 1u8;
                            fixed_u8::bit_and(a245, a246)
                        };
                        let a248 = 1u8;
                        if equal(a247, a248) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a249, a250)?
                };
                nat_add(a251, a252)?
            };
            nat_add(a253, a254)
        }
    }
}

pub fn f39(v0: Fn2, p258: Option<Adt0>, p259: List<(Bytes, Adt0)>) -> Option<Adt0> {
    let m257 = (p258, p259);
    {
        let mut m257 = m257;
        loop {
            let step = {
                let (v1, v2) = m257;
                let m255 = v2;
                match m255.uncons() {
                    None => {
                        Err::<(Option<Adt0>, List<(Bytes, Adt0)>), Option<Adt0>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Adt0>, List<(Bytes, Adt0)>), Option<Adt0>>(({
                            let c256: Fn2 = v0.clone();
                            c256.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m257 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f40(v0: Bytes, v1: Option<Adt0>, v2: (Bytes, Adt0)) -> Option<Adt0> {
    let m260 = v1.clone();
    match m260 {
        None => {
            let a262 = {
                let (h261, _) = v2;
                h261
            };
            let a263 = v0;
            if equal(a262, a263) {
                Some({
                    let (_, h264) = v2;
                    h264
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
    f60(f61(), 10u64, v0)
}

pub fn f42(v0: Adt4, v1: Adt6, v2: Adt0, v3: Adt0, v4: List<Adt9>, v5: Adt0, v6: List<Adt0>, v7: u64, v8: u64, v9: List<(Bytes, Adt0)>) -> R<Adt5> {
    let m265 = v0;
    match m265 {
        Adt4::C0 => {
            Ok(f7(f13(v1), Bytes::lit(&[])))
        }
        Adt4::C1(r284) => {
            let v10 = (*r284).clone();
            let m266 = v4.clone();
            match m266.uncons() {
                None => {
                    f22(v10, v1, v2, v3, v6, v7, v5, v8, v9)
                }
                Some((v11, v12)) => {
                    let a268 = match v11.clone() {
                        Adt9::C0(h267, _, _) => {
                            h267
                        }
                    };
                    let a269 = 0u64;
                    if nat_eq(a268, a269) {
                        let m270 = v6;
                        match m270.uncons() {
                            None => {
                                Ok(f10(Adt0::C5(std::rc::Rc::new(f62(v4)), std::rc::Rc::new(v5)), v1))
                            }
                            Some((v13, v14)) => {
                                let v15: Adt5 = f2(v10.clone(), v1, v13.clone(), v9.clone())?;
                                let m272 = match v15.clone() {
                                    Adt5::C0(_, h271, _) => {
                                        h271
                                    }
                                };
                                match m272 {
                                    None => {
                                        let v17: Adt7 = f5(v10.clone(), match v15.clone() {
                                            Adt5::C0(_, _, h273) => {
                                                h273
                                            }
                                        }, match v11.clone() {
                                            Adt9::C0(_, _, h274) => {
                                                h274
                                            }
                                        }, match v15 {
                                            Adt5::C0(h275, _, _) => {
                                                h275
                                            }
                                        })?;
                                        let m277 = match v17.clone() {
                                            Adt7::C0(h276, _) => {
                                                h276
                                            }
                                        };
                                        match m277 {
                                            None => {
                                                f63(v10, match v17 {
                                                    Adt7::C0(_, h279) => {
                                                        h279
                                                    }
                                                }, v2, v3, v11, v13, v12, v5, v14, v7, {
                                                    let a280 = v8;
                                                    let a281 = 1u64;
                                                    nat_add(a280, a281)?
                                                }, v9)
                                            }
                                            Some(v18) => {
                                                Ok(f7(match v17 {
                                                    Adt7::C0(_, h278) => {
                                                        h278
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
                        let v19: Adt8 = f36(v1)?;
                        f63(v10, match v19.clone() {
                            Adt8::C0(_, h282) => {
                                h282
                            }
                        }, v2, v3, v11, match v19 {
                            Adt8::C0(h283, _) => {
                                h283
                            }
                        }, v12, v5, v6, v7, v8, v9)
                    }
                }
            }
        }
    }
}

pub fn f43(v0: Adt4, v1: Adt0) -> List<Adt9> {
    let m285 = v0;
    match m285 {
        Adt4::C0 => {
            List::<Adt9>::nil()
        }
        Adt4::C1(r289) => {
            let v2 = (*r289).clone();
            let m286 = v1;
            match m286 {
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
                Adt0::C11(v3, v4, r287, r288) => {
                    let v5 = (*r287).clone();
                    let v6 = (*r288).clone();
                    List::cons(Adt9::C0(v3, v4, v5), f43(v2, v6))
                }
            }
        }
    }
}

pub fn f44(v0: Adt4, v1: Adt6, v2: Adt0, v3: Adt0, v4: List<Adt0>, v5: u64, v6: u64, v7: Adt0, v8: List<(Bytes, Adt0)>) -> R<Adt5> {
    let m290 = v0;
    match m290 {
        Adt4::C0 => {
            Ok(f7(f13(v1), Bytes::lit(&[])))
        }
        Adt4::C1(r292) => {
            let v9 = (*r292).clone();
            let m291 = f28(v7.clone())?;
            match m291 {
                None => {
                    if f51(v9.clone(), v1.clone(), v7.clone()) {
                        f64(v9, v1, v4, v8)
                    } else {
                        let v11: Bytes = f6(v2)?;
                        let v12: Bytes = f6(v3)?;
                        let v13: Bytes = f6(v7)?;
                        Ok(f7(v1, f8(f8(f8(f8(f8(f8(f8(f8(f8(v11, f9(Str::lit(" supplies "))), f41(v5)), f9(Str::lit(" explicit argument"))), f65(v5)), f9(Str::lit(" but "))), v12), f9(Str::lit(" accepts "))), f41(v6)), f8(f8(f9(Str::lit(": after them its type is ")), v13), f9(Str::lit(", not a function"))))))
                    }
                }
                Some(_) => {
                    f64(v9, v1, v4, v8)
                }
            }
        }
    }
}

pub fn f45(v0: Adt4, v1: Adt6, v2: Adt0, v3: Adt0) -> R<Adt5> {
    let v4: Adt8 = f27(v0.clone(), v1, v3)?;
    let v5: Adt0 = match v4.clone() {
        Adt8::C0(h293, _) => {
            h293
        }
    };
    let v6: Adt6 = match v4 {
        Adt8::C0(_, h294) => {
            h294
        }
    };
    let v7: u64 = f20(v2.clone());
    let a295 = v7;
    let a296 = 8u64;
    if if nat_eq(a295, a296) {
        true
    } else {
        let a297 = v7;
        let a298 = 6u64;
        nat_eq(a297, a298)
    } {
        let v8: Bytes = f6(v2)?;
        Ok(f7(v6, f8(v8, f9(Str::lit(" is a term, not a type")))))
    } else {
        let a299 = f20(v5.clone());
        let a300 = 2u64;
        if nat_lt(a299, a300) {
            Ok(f10(v5, v6))
        } else {
            let m301 = f28(v5.clone())?;
            match m301 {
                None => {
                    if f51(v0, v6.clone(), v5.clone()) {
                        Ok(f10(v5, v6))
                    } else {
                        let v10: Bytes = f6(v2)?;
                        let v11: Bytes = f6(v5)?;
                        Ok(f7(v6, f8(f8(f8(v10, f9(Str::lit(" is not a type: its type is "))), v11), Bytes::lit(&[]))))
                    }
                }
                Some(_) => {
                    Ok(f10(v5, v6))
                }
            }
        }
    }
}

pub fn f46(v0: Adt4, v1: Adt6, v2: List<Bytes>) -> R<(List<Option<Adt0>>, Adt6)> {
    let m302 = v0;
    match m302 {
        Adt4::C0 => {
            Ok((List::<Option<Adt0>>::nil(), f13(v1)))
        }
        Adt4::C1(r310) => {
            let v3 = (*r310).clone();
            let m303 = v2;
            match m303.uncons() {
                None => {
                    Ok((List::<Option<Adt0>>::nil(), v1))
                }
                Some((v4, v5)) => {
                    let v6: Adt8 = f27(v3.clone(), v1, Adt0::C3(v4))?;
                    let v7: (bool, Adt6) = f55(v3.clone(), match v6.clone() {
                        Adt8::C0(_, h304) => {
                            h304
                        }
                    }, match v6.clone() {
                        Adt8::C0(h305, _) => {
                            h305
                        }
                    })?;
                    let v8: (List<Option<Adt0>>, Adt6) = f46(v3, {
                        let (_, h306) = v7.clone();
                        h306
                    }, v5)?;
                    Ok((List::cons(if f56(v7) {
                        None::<Adt0>
                    } else {
                        Some(match v6 {
                            Adt8::C0(h307, _) => {
                                h307
                            }
                        })
                    }, {
                        let (h308, _) = v8;
                        h308
                    }), {
                        let (_, h309) = v8;
                        h309
                    }))
                }
            }
        }
    }
}

pub fn f47(v0: List<Bytes>) -> List<Bytes> {
    f66(Fn3::F67, List::<Bytes>::nil(), v0)
}

pub fn f48(v0: (List<Option<Adt0>>, Adt6)) -> Adt6 {
    let (_, h311) = v0;
    h311
}

pub fn f49(v0: Adt4, v1: List<(Bytes, Adt0)>, v2: Adt0) -> R<Option<Adt0>> {
    let m312 = v0;
    match m312 {
        Adt4::C0 => {
            Ok(None::<Adt0>)
        }
        Adt4::C1(r347) => {
            let v3 = (*r347).clone();
            let m313 = v2;
            match m313 {
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
                    if f17(v4.clone())? {
                        let m314 = f68(v1.clone(), v4.clone());
                        match m314 {
                            None => {
                                Ok(Some(Adt0::C3(v4)))
                            }
                            Some(v5) => {
                                f49(v3, v1, v5)
                            }
                        }
                    } else {
                        Ok(Some(Adt0::C3(v4)))
                    }
                }
                Adt0::C4(r334, r335) => {
                    let v6 = (*r334).clone();
                    let v7 = (*r335).clone();
                    let m315 = f69(f49(v3.clone(), v1.clone(), v6)?, f49(v3, v1, v7)?);
                    match m315 {
                        None => {
                            Ok(None::<Adt0>)
                        }
                        Some(v8) => {
                            Ok(Some(Adt0::C4(std::rc::Rc::new({
                                let (h316, _) = v8;
                                h316
                            }), std::rc::Rc::new({
                                let (_, h317) = v8;
                                h317
                            }))))
                        }
                    }
                }
                Adt0::C5(r336, r337) => {
                    let v12 = (*r336).clone();
                    let v13 = (*r337).clone();
                    let m321 = f69(f49(v3.clone(), v1.clone(), v12)?, f49(v3, v1, v13)?);
                    match m321 {
                        None => {
                            Ok(None::<Adt0>)
                        }
                        Some(v14) => {
                            Ok(Some(Adt0::C5(std::rc::Rc::new({
                                let (h322, _) = v14;
                                h322
                            }), std::rc::Rc::new({
                                let (_, h323) = v14;
                                h323
                            }))))
                        }
                    }
                }
                Adt0::C6(r338, r339) => {
                    let v15 = (*r338).clone();
                    let v16 = (*r339).clone();
                    let m324 = f69(f49(v3.clone(), v1.clone(), v15)?, f49(v3, v1, v16)?);
                    match m324 {
                        None => {
                            Ok(None::<Adt0>)
                        }
                        Some(v17) => {
                            Ok(Some(Adt0::C6(std::rc::Rc::new({
                                let (h325, _) = v17;
                                h325
                            }), std::rc::Rc::new({
                                let (_, h326) = v17;
                                h326
                            }))))
                        }
                    }
                }
                Adt0::C7(v23, r340, r341, r342) => {
                    let v24 = (*r340).clone();
                    let v25 = (*r341).clone();
                    let v26 = (*r342).clone();
                    let m330 = f69(f49(v3.clone(), v1.clone(), v24)?, f49(v3.clone(), v1.clone(), v25)?);
                    match m330 {
                        None => {
                            Ok(None::<Adt0>)
                        }
                        Some(v27) => {
                            let m331 = f49(v3, v1, v26)?;
                            match m331 {
                                None => {
                                    Ok(None::<Adt0>)
                                }
                                Some(v28) => {
                                    Ok(Some(Adt0::C7(v23, std::rc::Rc::new({
                                        let (h332, _) = v27;
                                        h332
                                    }), std::rc::Rc::new({
                                        let (_, h333) = v27;
                                        h333
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
                Adt0::C10(r343, r344) => {
                    let v9 = (*r343).clone();
                    let v10 = (*r344).clone();
                    let m318 = f69(f49(v3.clone(), v1.clone(), v9)?, f49(v3, v1, v10)?);
                    match m318 {
                        None => {
                            Ok(None::<Adt0>)
                        }
                        Some(v11) => {
                            Ok(Some(Adt0::C10(std::rc::Rc::new({
                                let (h319, _) = v11;
                                h319
                            }), std::rc::Rc::new({
                                let (_, h320) = v11;
                                h320
                            }))))
                        }
                    }
                }
                Adt0::C11(v18, v19, r345, r346) => {
                    let v20 = (*r345).clone();
                    let v21 = (*r346).clone();
                    let m327 = f69(f49(v3.clone(), v1.clone(), v20)?, f49(v3, v1, v21)?);
                    match m327 {
                        None => {
                            Ok(None::<Adt0>)
                        }
                        Some(v22) => {
                            Ok(Some(Adt0::C11(v18, v19, std::rc::Rc::new({
                                let (h328, _) = v22;
                                h328
                            }), std::rc::Rc::new({
                                let (_, h329) = v22;
                                h329
                            }))))
                        }
                    }
                }
            }
        }
    }
}

pub fn f50(v0: Adt4, v1: Adt0, v2: Adt0) -> u64 {
    let v3: u64 = f20(v1.clone());
    let a348 = v3;
    let a349 = f20(v2.clone());
    if nat_eq(a348, a349) {
        let a350 = v3;
        let a351 = 2u64;
        if nat_lt(a350, a351) {
            0u64
        } else {
            let a352 = v3;
            let a353 = 2u64;
            if if nat_eq(a352, a353) {
                true
            } else {
                let a354 = v3;
                let a355 = 3u64;
                if nat_eq(a354, a355) {
                    true
                } else {
                    let a356 = v3;
                    let a357 = 8u64;
                    nat_eq(a356, a357)
                }
            } {
                let a358 = f29(v1);
                let a359 = f29(v2);
                if equal(a358, a359) {
                    0u64
                } else {
                    3u64
                }
            } else {
                let a360 = v3;
                let a361 = 4u64;
                if nat_eq(a360, a361) {
                    let m362 = v1;
                    match m362 {
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
                        Adt0::C4(_, r369) => {
                            let v5 = (*r369).clone();
                            let m363 = v2;
                            match m363 {
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
                                Adt0::C4(_, r368) => {
                                    let v7 = (*r368).clone();
                                    let a366 = {
                                        let a364 = f21(v0.clone(), v5);
                                        length_list(a364)
                                    };
                                    let a367 = {
                                        let a365 = f21(v0, v7);
                                        length_list(a365)
                                    };
                                    if nat_eq(a366, a367) {
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
                    let a370 = v3;
                    let a371 = 5u64;
                    if if nat_eq(a370, a371) {
                        true
                    } else {
                        let a372 = v3;
                        let a373 = 6u64;
                        nat_eq(a372, a373)
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

pub fn f51(p379: Adt4, v1: Adt6, p380: Adt0) -> bool {
    let m378 = (p379, p380);
    {
        let mut m378 = m378;
        loop {
            let step = {
                let (v0, v2) = m378;
                let m374 = v0;
                match m374 {
                    Adt4::C0 => {
                        Err::<(Adt4, Adt0), bool>(false)
                    }
                    Adt4::C1(r377) => {
                        let v3 = (*r377).clone();
                        let m375 = v2;
                        match m375 {
                            Adt0::C0 => {
                                Err::<(Adt4, Adt0), bool>(false)
                            }
                            Adt0::C1(_) => {
                                Err::<(Adt4, Adt0), bool>(false)
                            }
                            Adt0::C2(v4, v5, _) => {
                                Err::<(Adt4, Adt0), bool>(f70(v1.clone(), v4, v5))
                            }
                            Adt0::C3(_) => {
                                Err::<(Adt4, Adt0), bool>(false)
                            }
                            Adt0::C4(r376, _) => {
                                let v7 = (*r376).clone();
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
                Ok(next) => { m378 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f52(v0: Adt0) -> (Adt0, Adt0) {
    let m381 = v0;
    match m381 {
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
        Adt0::C4(r382, r383) => {
            let v1 = (*r382).clone();
            let v2 = (*r383).clone();
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

pub fn f53(p396: Adt4, p397: Adt6, p398: Adt0, p399: Adt0) -> R<Adt7> {
    let m395 = (p396, (p397, (p398, p399)));
    {
        let mut m395 = m395;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m395;
                let m384 = v0;
                match m384 {
                    Adt4::C0 => {
                        Err::<(Adt4, (Adt6, (Adt0, Adt0))), R<Adt7>>(Ok(f26(f13(v1))))
                    }
                    Adt4::C1(r394) => {
                        let v4 = (*r394).clone();
                        let m385 = v2;
                        match m385 {
                            Adt0::C0 => {
                                Err::<(Adt4, (Adt6, (Adt0, Adt0))), R<Adt7>>(Ok(f26(v1)))
                            }
                            Adt0::C1(_) => {
                                Err::<(Adt4, (Adt6, (Adt0, Adt0))), R<Adt7>>(Ok(f26(v1)))
                            }
                            Adt0::C2(_, _, _) => {
                                Err::<(Adt4, (Adt6, (Adt0, Adt0))), R<Adt7>>(Ok(f26(v1)))
                            }
                            Adt0::C3(_) => {
                                Err::<(Adt4, (Adt6, (Adt0, Adt0))), R<Adt7>>(Ok(f26(v1)))
                            }
                            Adt0::C4(_, _) => {
                                Err::<(Adt4, (Adt6, (Adt0, Adt0))), R<Adt7>>(Ok(f26(v1)))
                            }
                            Adt0::C5(_, _) => {
                                Err::<(Adt4, (Adt6, (Adt0, Adt0))), R<Adt7>>(Ok(f26(v1)))
                            }
                            Adt0::C6(_, _) => {
                                Err::<(Adt4, (Adt6, (Adt0, Adt0))), R<Adt7>>(Ok(f26(v1)))
                            }
                            Adt0::C7(_, _, _, _) => {
                                Err::<(Adt4, (Adt6, (Adt0, Adt0))), R<Adt7>>(Ok(f26(v1)))
                            }
                            Adt0::C8(_) => {
                                Err::<(Adt4, (Adt6, (Adt0, Adt0))), R<Adt7>>(Ok(f26(v1)))
                            }
                            Adt0::C9 => {
                                Err::<(Adt4, (Adt6, (Adt0, Adt0))), R<Adt7>>(Ok(f26(v1)))
                            }
                            Adt0::C10(r392, r393) => {
                                let v5 = (*r392).clone();
                                let v6 = (*r393).clone();
                                let m386 = v3;
                                match m386 {
                                    Adt0::C0 => {
                                        Err::<(Adt4, (Adt6, (Adt0, Adt0))), R<Adt7>>(Ok(f26(v1)))
                                    }
                                    Adt0::C1(_) => {
                                        Err::<(Adt4, (Adt6, (Adt0, Adt0))), R<Adt7>>(Ok(f26(v1)))
                                    }
                                    Adt0::C2(_, _, _) => {
                                        Err::<(Adt4, (Adt6, (Adt0, Adt0))), R<Adt7>>(Ok(f26(v1)))
                                    }
                                    Adt0::C3(_) => {
                                        Err::<(Adt4, (Adt6, (Adt0, Adt0))), R<Adt7>>(Ok(f26(v1)))
                                    }
                                    Adt0::C4(_, _) => {
                                        Err::<(Adt4, (Adt6, (Adt0, Adt0))), R<Adt7>>(Ok(f26(v1)))
                                    }
                                    Adt0::C5(_, _) => {
                                        Err::<(Adt4, (Adt6, (Adt0, Adt0))), R<Adt7>>(Ok(f26(v1)))
                                    }
                                    Adt0::C6(_, _) => {
                                        Err::<(Adt4, (Adt6, (Adt0, Adt0))), R<Adt7>>(Ok(f26(v1)))
                                    }
                                    Adt0::C7(_, _, _, _) => {
                                        Err::<(Adt4, (Adt6, (Adt0, Adt0))), R<Adt7>>(Ok(f26(v1)))
                                    }
                                    Adt0::C8(_) => {
                                        Err::<(Adt4, (Adt6, (Adt0, Adt0))), R<Adt7>>(Ok(f26(v1)))
                                    }
                                    Adt0::C9 => {
                                        Err::<(Adt4, (Adt6, (Adt0, Adt0))), R<Adt7>>(Ok(f26(v1)))
                                    }
                                    Adt0::C10(r390, r391) => {
                                        let v7 = (*r390).clone();
                                        let v8 = (*r391).clone();
                                        let v9: Adt7 = f5(v4.clone(), v1, v5, v7)?;
                                        let m388 = match v9.clone() {
                                            Adt7::C0(h387, _) => {
                                                h387
                                            }
                                        };
                                        match m388 {
                                            None => {
                                                Ok::<(Adt4, (Adt6, (Adt0, Adt0))), R<Adt7>>((v4, (match v9 {
                                                    Adt7::C0(_, h389) => {
                                                        h389
                                                    }
                                                }, (v6, v8))))
                                            }
                                            Some(_) => {
                                                Err::<(Adt4, (Adt6, (Adt0, Adt0))), R<Adt7>>(Ok(v9))
                                            }
                                        }
                                    }
                                    Adt0::C11(_, _, _, _) => {
                                        Err::<(Adt4, (Adt6, (Adt0, Adt0))), R<Adt7>>(Ok(f26(v1)))
                                    }
                                }
                            }
                            Adt0::C11(_, _, _, _) => {
                                Err::<(Adt4, (Adt6, (Adt0, Adt0))), R<Adt7>>(Ok(f26(v1)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m395 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f54(v0: Adt4, v1: Adt6, v2: Adt0, v3: Adt0) -> R<Adt7> {
    let m400 = v0;
    match m400 {
        Adt4::C0 => {
            Ok(f26(f13(v1)))
        }
        Adt4::C1(r413) => {
            let v4 = (*r413).clone();
            let v5: bool = {
                let a401 = f20(v2.clone());
                let a402 = 5u64;
                nat_eq(a401, a402)
            };
            let v6: Adt10 = f71(v4.clone(), v2.clone(), v5);
            let v7: Adt10 = f71(v4.clone(), v3.clone(), v5);
            let v8: u64 = {
                let a404 = match v6.clone() {
                    Adt10::C0(h403, _) => {
                        h403
                    }
                };
                length_list(a404)
            };
            let v9: u64 = {
                let a406 = match v7.clone() {
                    Adt10::C0(h405, _) => {
                        h405
                    }
                };
                length_list(a406)
            };
            let a407 = v8;
            let a408 = v9;
            if nat_eq(a407, a408) {
                f72(v4, v1, match v6.clone() {
                    Adt10::C0(h409, _) => {
                        h409
                    }
                }, match v7.clone() {
                    Adt10::C0(h410, _) => {
                        h410
                    }
                }, match v7 {
                    Adt10::C0(_, h411) => {
                        h411
                    }
                }, match v6 {
                    Adt10::C0(_, h412) => {
                        h412
                    }
                }, v2, v3)
            } else {
                let v10: Bytes = f6(v2)?;
                let v11: Bytes = f6(v3)?;
                Ok(f57(v1, f8(f8(f8(f8(f8(f8(v10, f9(Str::lit(" binds "))), f41(v8)), f9(Str::lit(" arguments but "))), v11), f9(Str::lit(" binds "))), f41(v9))))
            }
        }
    }
}

pub fn f55(v0: Adt4, v1: Adt6, v2: Adt0) -> R<(bool, Adt6)> {
    let v3: Adt8 = f27(v0.clone(), v1, v2)?;
    let m415 = f73(v0, match v3.clone() {
        Adt8::C0(h414, _) => {
            h414
        }
    }, List::<Bytes>::nil())?;
    match m415 {
        None => {
            Ok((true, f13(match v3 {
                Adt8::C0(_, h417) => {
                    h417
                }
            })))
        }
        Some(v4) => {
            Ok((f74(Fn4::F75, false, v4)?, match v3 {
                Adt8::C0(_, h416) => {
                    h416
                }
            }))
        }
    }
}

pub fn f56(v0: (bool, Adt6)) -> bool {
    let (h418, _) = v0;
    h418
}

pub fn f57(v0: Adt6, v1: Bytes) -> Adt7 {
    Adt7::C0(Some(v1), v0)
}

pub fn f58(v0: Adt0, v1: bool, v2: List<(Bytes, Bytes)>, v3: u64) -> R<(Bytes, u64)> {
    let m419 = v0;
    match m419 {
        Adt0::C0 => {
            Ok((f76(Str::lit("(sort prop)")), v3))
        }
        Adt0::C1(v4) => {
            Ok((f77(f77(f76(Str::lit("(sort (type ")), f78(v4)), f76(Str::lit("))"))), v3))
        }
        Adt0::C2(v5, v6, v7) => {
            Ok((f79(v5, v6, v7), v3))
        }
        Adt0::C3(v8) => {
            Ok((f77(f77(f76(Str::lit("(local ")), if v1 {
                f80(v2, v8)
            } else {
                v8
            }), f76(Str::lit(")"))), v3))
        }
        Adt0::C4(r447, r448) => {
            let v9 = (*r447).clone();
            let v10 = (*r448).clone();
            let v11: (Bytes, u64) = f58(v9, v1, v2.clone(), v3)?;
            let v12: (Bytes, u64) = f81(v10, v1, v2, {
                let (_, h420) = v11;
                h420
            })?;
            Ok((f77(f77(f77(f76(Str::lit("(app ")), {
                let (h421, _) = v11;
                h421
            }), {
                let (h422, _) = v12;
                h422
            }), f76(Str::lit(")"))), {
                let (_, h423) = v12;
                h423
            }))
        }
        Adt0::C5(r449, r450) => {
            let v13 = (*r449).clone();
            let v14 = (*r450).clone();
            let v15: ((Bytes, List<(Bytes, Bytes)>), u64) = f82(v13, v1, v2, v3)?;
            let v16: (Bytes, u64) = f58(v14, v1, {
                let (_, h424) = {
                    let (h425, _) = v15.clone();
                    h425
                };
                h424
            }, {
                let (_, h426) = v15;
                h426
            })?;
            Ok((f77(f77(f77(f77(f76(Str::lit("(pi (")), {
                let (h427, _) = {
                    let (h428, _) = v15;
                    h428
                };
                h427
            }), f76(Str::lit(") "))), {
                let (h429, _) = v16;
                h429
            }), f76(Str::lit(")"))), {
                let (_, h430) = v16;
                h430
            }))
        }
        Adt0::C6(r451, r452) => {
            let v17 = (*r451).clone();
            let v18 = (*r452).clone();
            let v19: ((Bytes, List<(Bytes, Bytes)>), u64) = f82(v17, v1, v2, v3)?;
            let v20: (Bytes, u64) = f58(v18, v1, {
                let (_, h431) = {
                    let (h432, _) = v19.clone();
                    h432
                };
                h431
            }, {
                let (_, h433) = v19;
                h433
            })?;
            Ok((f77(f77(f77(f77(f76(Str::lit("(lam (")), {
                let (h434, _) = {
                    let (h435, _) = v19;
                    h435
                };
                h434
            }), f76(Str::lit(") "))), {
                let (h436, _) = v20;
                h436
            }), f76(Str::lit(")"))), {
                let (_, h437) = v20;
                h437
            }))
        }
        Adt0::C7(v21, r453, r454, r455) => {
            let v22 = (*r453).clone();
            let v23 = (*r454).clone();
            let v24 = (*r455).clone();
            let v25: (Bytes, u64) = f83(v21.clone(), v1, v3)?;
            let v26: List<(Bytes, Bytes)> = List::cons((v21, {
                let (h438, _) = v25.clone();
                h438
            }), v2);
            let v27: (Bytes, u64) = f58(v22, v1, v26.clone(), {
                let (_, h439) = v25;
                h439
            })?;
            let v28: (Bytes, u64) = f58(v23, v1, v26.clone(), {
                let (_, h440) = v27;
                h440
            })?;
            let v29: (Bytes, u64) = f58(v24, v1, v26, {
                let (_, h441) = v28;
                h441
            })?;
            Ok((f77(f77(f77(f77(f77(f77(f77(f76(Str::lit("(let ")), {
                let (h442, _) = v25;
                h442
            }), f76(Str::lit(" "))), {
                let (h443, _) = v27;
                h443
            }), f76(Str::lit(" "))), {
                let (h444, _) = v28;
                h444
            }), f76(Str::lit(" "))), f77({
                let (h445, _) = v29;
                h445
            }, f76(Str::lit(")")))), {
                let (_, h446) = v29;
                h446
            }))
        }
        Adt0::C8(v30) => {
            Ok((f77(f77(f76(Str::lit("(nat ")), v30), f76(Str::lit(")"))), v3))
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

pub fn f59(v0: Bytes, v1: u64) -> Option<u8> {
    let a456 = v0;
    let a457 = v1;
    index_bytes(a456, a457)
}

pub fn f60(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let a458 = v2;
    let a459 = 0u64;
    if nat_eq(a458, a459) {
        f84(v0, 0u64)
    } else {
        let (_, h460) = {
            let (h461, _) = f85(Fn5::F86(v1, v0), v2, (v2, Bytes::lit(&[])));
            h461
        };
        h460
    }
}

pub fn f61() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f62(v0: List<Adt9>) -> Adt0 {
    f87(Fn6::F88, Adt0::C9, f89(v0))
}

pub fn f63(v0: Adt4, v1: Adt6, v2: Adt0, v3: Adt0, v4: Adt9, v5: Adt0, v6: List<Adt9>, v7: Adt0, v8: List<Adt0>, v9: u64, v10: u64, v11: List<(Bytes, Adt0)>) -> R<Adt5> {
    let m462 = v0;
    match m462 {
        Adt4::C0 => {
            Ok(f7(f13(v1), Bytes::lit(&[])))
        }
        Adt4::C1(r469) => {
            let v12 = (*r469).clone();
            let v13: Adt11 = f90(v12.clone(), v1, v6, match v4.clone() {
                Adt9::C0(_, h463, _) => {
                    h463
                }
            }, v5.clone())?;
            let v14: Adt8 = f91(v12.clone(), match v13.clone() {
                Adt11::C0(_, h464) => {
                    h464
                }
            }, v7, match v4 {
                Adt9::C0(_, h465, _) => {
                    h465
                }
            }, v5)?;
            f42(v12, match v14.clone() {
                Adt8::C0(_, h466) => {
                    h466
                }
            }, v2, v3, match v13 {
                Adt11::C0(h467, _) => {
                    h467
                }
            }, match v14 {
                Adt8::C0(h468, _) => {
                    h468
                }
            }, v8, v9, v10, v11)
        }
    }
}

pub fn f64(v0: Adt4, v1: Adt6, v2: List<Adt0>, v3: List<(Bytes, Adt0)>) -> R<Adt5> {
    let m470 = v0;
    match m470 {
        Adt4::C0 => {
            Ok(f7(f13(v1), Bytes::lit(&[])))
        }
        Adt4::C1(r478) => {
            let v4 = (*r478).clone();
            let m471 = v2;
            match m471.uncons() {
                None => {
                    f15(v1)
                }
                Some((v5, v6)) => {
                    let v7: Adt5 = f2(v4.clone(), v1, v5, v3.clone())?;
                    let m473 = match v7.clone() {
                        Adt5::C0(_, h472, _) => {
                            h472
                        }
                    };
                    match m473 {
                        None => {
                            let v9: Adt5 = f64(v4, match v7 {
                                Adt5::C0(_, _, h474) => {
                                    h474
                                }
                            }, v6, v3)?;
                            Ok(Adt5::C0(match v9.clone() {
                                Adt5::C0(h475, _, _) => {
                                    h475
                                }
                            }, match v9.clone() {
                                Adt5::C0(_, h476, _) => {
                                    h476
                                }
                            }, match v9 {
                                Adt5::C0(_, _, h477) => {
                                    h477
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

pub fn f65(v0: u64) -> Bytes {
    let a479 = v0;
    let a480 = 1u64;
    if nat_eq(a479, a480) {
        Bytes::lit(&[])
    } else {
        f9(Str::lit("s"))
    }
}

pub fn f66(v0: Fn3, p484: List<Bytes>, p485: List<Bytes>) -> List<Bytes> {
    let m483 = (p484, p485);
    {
        let mut m483 = m483;
        loop {
            let step = {
                let (v1, v2) = m483;
                let m481 = v2;
                match m481.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c482: Fn3 = v0.clone();
                            c482.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m483 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f67(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f68(v0: List<(Bytes, Adt0)>, v1: Bytes) -> Option<Adt0> {
    f39(Fn2::F92(v1), None::<Adt0>, v0)
}

pub fn f69(v0: Option<Adt0>, v1: Option<Adt0>) -> Option<(Adt0, Adt0)> {
    let m486 = v0;
    match m486 {
        None => {
            None::<(Adt0, Adt0)>
        }
        Some(v2) => {
            let m487 = v1;
            match m487 {
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

pub fn f70(v0: Adt6, v1: Bytes, v2: Bytes) -> bool {
    let v3: (u64, bool) = f14(v0, v1, v2);
    let a489 = {
        let (h488, _) = v3;
        h488
    };
    let a490 = 1u64;
    if nat_eq(a489, a490) {
        true
    } else {
        let a492 = {
            let (h491, _) = v3;
            h491
        };
        let a493 = 3u64;
        if nat_eq(a492, a493) {
            let (_, h494) = v3;
            h494
        } else {
            false
        }
    }
}

pub fn f71(v0: Adt4, v1: Adt0, v2: bool) -> Adt10 {
    let m495 = v0;
    match m495 {
        Adt4::C0 => {
            Adt10::C0(List::<Adt9>::nil(), v1)
        }
        Adt4::C1(r514) => {
            let v3 = (*r514).clone();
            let m496 = v1;
            match m496 {
                Adt0::C0 => {
                    Adt10::C0(List::<Adt9>::nil(), Adt0::C0)
                }
                Adt0::C1(v10) => {
                    Adt10::C0(List::<Adt9>::nil(), Adt0::C1(v10))
                }
                Adt0::C2(v11, v12, v13) => {
                    Adt10::C0(List::<Adt9>::nil(), Adt0::C2(v11, v12, v13))
                }
                Adt0::C3(v14) => {
                    Adt10::C0(List::<Adt9>::nil(), Adt0::C3(v14))
                }
                Adt0::C4(r501, r502) => {
                    let v15 = (*r501).clone();
                    let v16 = (*r502).clone();
                    Adt10::C0(List::<Adt9>::nil(), Adt0::C4(std::rc::Rc::new(v15), std::rc::Rc::new(v16)))
                }
                Adt0::C5(r503, r504) => {
                    let v4 = (*r503).clone();
                    let v5 = (*r504).clone();
                    if v2 {
                        let v6: Adt10 = f71(v3.clone(), v5, v2);
                        Adt10::C0(f93(Fn7::F94, match v6.clone() {
                            Adt10::C0(h497, _) => {
                                h497
                            }
                        }, f89(f43(v3, v4))), match v6 {
                            Adt10::C0(_, h498) => {
                                h498
                            }
                        })
                    } else {
                        Adt10::C0(List::<Adt9>::nil(), Adt0::C5(std::rc::Rc::new(v4), std::rc::Rc::new(v5)))
                    }
                }
                Adt0::C6(r505, r506) => {
                    let v7 = (*r505).clone();
                    let v8 = (*r506).clone();
                    if v2 {
                        Adt10::C0(List::<Adt9>::nil(), Adt0::C6(std::rc::Rc::new(v7), std::rc::Rc::new(v8)))
                    } else {
                        let v9: Adt10 = f71(v3.clone(), v8, v2);
                        Adt10::C0(f93(Fn7::F95, match v9.clone() {
                            Adt10::C0(h499, _) => {
                                h499
                            }
                        }, f89(f43(v3, v7))), match v9 {
                            Adt10::C0(_, h500) => {
                                h500
                            }
                        })
                    }
                }
                Adt0::C7(v17, r507, r508, r509) => {
                    let v18 = (*r507).clone();
                    let v19 = (*r508).clone();
                    let v20 = (*r509).clone();
                    Adt10::C0(List::<Adt9>::nil(), Adt0::C7(v17, std::rc::Rc::new(v18), std::rc::Rc::new(v19), std::rc::Rc::new(v20)))
                }
                Adt0::C8(v21) => {
                    Adt10::C0(List::<Adt9>::nil(), Adt0::C8(v21))
                }
                Adt0::C9 => {
                    Adt10::C0(List::<Adt9>::nil(), Adt0::C9)
                }
                Adt0::C10(r510, r511) => {
                    let v22 = (*r510).clone();
                    let v23 = (*r511).clone();
                    Adt10::C0(List::<Adt9>::nil(), Adt0::C10(std::rc::Rc::new(v22), std::rc::Rc::new(v23)))
                }
                Adt0::C11(v24, v25, r512, r513) => {
                    let v26 = (*r512).clone();
                    let v27 = (*r513).clone();
                    Adt10::C0(List::<Adt9>::nil(), Adt0::C11(v24, v25, std::rc::Rc::new(v26), std::rc::Rc::new(v27)))
                }
            }
        }
    }
}

pub fn f72(v0: Adt4, v1: Adt6, v2: List<Adt9>, v3: List<Adt9>, v4: Adt0, v5: Adt0, v6: Adt0, v7: Adt0) -> R<Adt7> {
    let m515 = v0;
    match m515 {
        Adt4::C0 => {
            Ok(f26(f13(v1)))
        }
        Adt4::C1(r540) => {
            let v8 = (*r540).clone();
            let m516 = v2;
            match m516.uncons() {
                None => {
                    f5(v8, v1, v5, v4)
                }
                Some((v9, v10)) => {
                    let m517 = v3;
                    match m517.uncons() {
                        None => {
                            f5(v8, v1, v5, v4)
                        }
                        Some((v11, v12)) => {
                            let a520 = match v9.clone() {
                                Adt9::C0(h518, _, _) => {
                                    h518
                                }
                            };
                            let a521 = match v11.clone() {
                                Adt9::C0(h519, _, _) => {
                                    h519
                                }
                            };
                            if nat_eq(a520, a521) {
                                let v13: Adt7 = f5(v8.clone(), v1, match v9.clone() {
                                    Adt9::C0(_, _, h522) => {
                                        h522
                                    }
                                }, match v11.clone() {
                                    Adt9::C0(_, _, h523) => {
                                        h523
                                    }
                                })?;
                                let m525 = match v13.clone() {
                                    Adt7::C0(h524, _) => {
                                        h524
                                    }
                                };
                                match m525 {
                                    None => {
                                        let v15: Adt0 = Adt0::C3(match v9 {
                                            Adt9::C0(_, h526, _) => {
                                                h526
                                            }
                                        });
                                        let v16: Adt11 = f90(v8.clone(), match v13 {
                                            Adt7::C0(_, h527) => {
                                                h527
                                            }
                                        }, v12, match v11.clone() {
                                            Adt9::C0(_, h528, _) => {
                                                h528
                                            }
                                        }, v15.clone())?;
                                        let v17: Adt8 = f91(v8.clone(), match v16.clone() {
                                            Adt11::C0(_, h529) => {
                                                h529
                                            }
                                        }, v4, match v11 {
                                            Adt9::C0(_, h530, _) => {
                                                h530
                                            }
                                        }, v15)?;
                                        let v18: Adt7 = f72(v8, match v17.clone() {
                                            Adt8::C0(_, h531) => {
                                                h531
                                            }
                                        }, v10, match v16 {
                                            Adt11::C0(h532, _) => {
                                                h532
                                            }
                                        }, match v17 {
                                            Adt8::C0(h533, _) => {
                                                h533
                                            }
                                        }, v5, v6, v7)?;
                                        Ok(Adt7::C0(match v18.clone() {
                                            Adt7::C0(h534, _) => {
                                                h534
                                            }
                                        }, match v18 {
                                            Adt7::C0(_, h535) => {
                                                h535
                                            }
                                        }))
                                    }
                                    Some(_) => {
                                        Ok(v13)
                                    }
                                }
                            } else {
                                let v19: Bytes = f6(v6)?;
                                let v20: Bytes = f6(v7)?;
                                Ok(f57(v1, f8(f8(f8(f8(f8(f8(f8(f8(f8(f8(f8(f9(Str::lit("binder `")), match v9.clone() {
                                    Adt9::C0(_, h536, _) => {
                                        h536
                                    }
                                }), f9(Str::lit("` is "))), f96(match v9 {
                                    Adt9::C0(h537, _, _) => {
                                        h537
                                    }
                                })), f9(Str::lit(" in "))), v19), f9(Str::lit(" but `"))), match v11.clone() {
                                    Adt9::C0(_, h538, _) => {
                                        h538
                                    }
                                }), f9(Str::lit("` is "))), f96(match v11 {
                                    Adt9::C0(h539, _, _) => {
                                        h539
                                    }
                                })), f9(Str::lit(" in "))), v20)))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f73(v0: Adt4, v1: Adt0, v2: List<Bytes>) -> R<Option<List<Bytes>>> {
    let m541 = v0;
    match m541 {
        Adt4::C0 => {
            Ok(None::<List<Bytes>>)
        }
        Adt4::C1(r554) => {
            let v3 = (*r554).clone();
            let m542 = v1;
            match m542 {
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
                    if f97(v2, v4.clone())? {
                        Ok(Some(List::<Bytes>::nil()))
                    } else {
                        Ok(Some(List::cons(v4, List::<Bytes>::nil())))
                    }
                }
                Adt0::C4(r543, r544) => {
                    let v5 = (*r543).clone();
                    let v6 = (*r544).clone();
                    Ok(f98(f73(v3.clone(), v5, v2.clone())?, f73(v3, v6, v2)?))
                }
                Adt0::C5(r545, r546) => {
                    let v9 = (*r545).clone();
                    let v10 = (*r546).clone();
                    f99(v3, v9, v2, v10)
                }
                Adt0::C6(r547, r548) => {
                    let v11 = (*r547).clone();
                    let v12 = (*r548).clone();
                    f99(v3, v11, v2, v12)
                }
                Adt0::C7(v13, r549, r550, r551) => {
                    let v14 = (*r549).clone();
                    let v15 = (*r550).clone();
                    let v16 = (*r551).clone();
                    Ok(f98(f98(f73(v3.clone(), v14, v2.clone())?, f73(v3.clone(), v15, v2.clone())?), f73(v3, v16, List::cons(v13, v2))?))
                }
                Adt0::C8(_) => {
                    Ok(Some(List::<Bytes>::nil()))
                }
                Adt0::C9 => {
                    Ok(Some(List::<Bytes>::nil()))
                }
                Adt0::C10(r552, r553) => {
                    let v7 = (*r552).clone();
                    let v8 = (*r553).clone();
                    Ok(f98(f73(v3.clone(), v7, v2.clone())?, f73(v3, v8, v2)?))
                }
                Adt0::C11(_, _, _, _) => {
                    Ok(Some(List::<Bytes>::nil()))
                }
            }
        }
    }
}

pub fn f74(v0: Fn4, p558: bool, p559: List<Bytes>) -> R<bool> {
    let m557 = (p558, p559);
    {
        let mut m557 = m557;
        loop {
            let step = {
                let (v1, v2) = m557;
                let m555 = v2;
                match m555.uncons() {
                    None => {
                        Err::<(bool, List<Bytes>), R<bool>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<Bytes>), R<bool>>(({
                            let c556: Fn4 = v0.clone();
                            c556.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m557 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f75(v0: bool, v1: Bytes) -> R<bool> {
    if v0 {
        Ok(true)
    } else {
        f17(v1)
    }
}

pub fn f76(v0: Str) -> Bytes {
    let a560 = v0;
    utf8_encode(a560)
}

pub fn f77(v0: Bytes, v1: Bytes) -> Bytes {
    let a561 = v0;
    let a562 = v1;
    append_bytes(a561, a562)
}

pub fn f78(v0: Adt1) -> Bytes {
    let m563 = v0;
    match m563 {
        Adt1::C0(v1) => {
            f41(v1)
        }
        Adt1::C1(v2) => {
            v2
        }
        Adt1::C2(r564) => {
            let v3 = (*r564).clone();
            f77(f77(f76(Str::lit("(succ ")), f78(v3)), f76(Str::lit(")")))
        }
        Adt1::C3(r565) => {
            let v4 = (*r565).clone();
            f77(f77(f76(Str::lit("(max")), f100(v4)), f76(Str::lit(")")))
        }
        Adt1::C4(r566, r567) => {
            let v5 = (*r566).clone();
            let v6 = (*r567).clone();
            f77(f77(f77(f77(f76(Str::lit("(imax ")), f78(v5)), f76(Str::lit(" "))), f78(v6)), f76(Str::lit(")")))
        }
        Adt1::C5 => {
            Bytes::lit(&[])
        }
        Adt1::C6(_, _) => {
            Bytes::lit(&[])
        }
    }
}

pub fn f79(v0: Bytes, v1: Bytes, v2: Adt1) -> Bytes {
    let v3: Bytes = f77(f77(f77(f76(Str::lit("(const ")), v0), f76(Str::lit("::"))), v1);
    let m568 = v2.clone();
    match m568 {
        Adt1::C0(_) => {
            f77(v3, f76(Str::lit(")")))
        }
        Adt1::C1(_) => {
            f77(v3, f76(Str::lit(")")))
        }
        Adt1::C2(_) => {
            f77(v3, f76(Str::lit(")")))
        }
        Adt1::C3(_) => {
            f77(v3, f76(Str::lit(")")))
        }
        Adt1::C4(_, _) => {
            f77(v3, f76(Str::lit(")")))
        }
        Adt1::C5 => {
            f77(v3, f76(Str::lit(")")))
        }
        Adt1::C6(_, _) => {
            let v6: Bytes = f100(v2);
            f77(f77(f77(v3, f76(Str::lit(" ("))), f101(v6.clone(), 1u64, {
                let a570 = {
                    let a569 = v6;
                    length_bytes(a569)
                };
                let a571 = 1u64;
                nat_sub(a570, a571)
            })), f76(Str::lit("))")))
        }
    }
}

pub fn f80(v0: List<(Bytes, Bytes)>, v1: Bytes) -> Bytes {
    let m572 = f102(Fn8::F103(v1.clone()), None::<Bytes>, v0);
    match m572 {
        None => {
            v1
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f81(v0: Adt0, v1: bool, v2: List<(Bytes, Bytes)>, v3: u64) -> R<(Bytes, u64)> {
    let m573 = v0;
    match m573 {
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
        Adt0::C10(r578, r579) => {
            let v4 = (*r578).clone();
            let v5 = (*r579).clone();
            let v6: (Bytes, u64) = f58(v4, v1, v2.clone(), v3)?;
            let v7: (Bytes, u64) = f81(v5, v1, v2, {
                let (_, h574) = v6;
                h574
            })?;
            Ok((f77(f77(f76(Str::lit(" ")), {
                let (h575, _) = v6;
                h575
            }), {
                let (h576, _) = v7;
                h576
            }), {
                let (_, h577) = v7;
                h577
            }))
        }
        Adt0::C11(_, _, _, _) => {
            Ok((Bytes::lit(&[]), v3))
        }
    }
}

pub fn f82(v0: Adt0, v1: bool, v2: List<(Bytes, Bytes)>, v3: u64) -> R<((Bytes, List<(Bytes, Bytes)>), u64)> {
    let m580 = v0;
    match m580 {
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
        Adt0::C11(v4, v5, r603, r604) => {
            let v6 = (*r603).clone();
            let v7 = (*r604).clone();
            let v8: (Bytes, u64) = f83(v5.clone(), v1, v3)?;
            let v9: (Bytes, u64) = f58(v6, v1, v2.clone(), {
                let (_, h581) = v8.clone();
                h581
            })?;
            let v10: Bytes = f77(f77(f77(f77(f77(f77(f76(Str::lit("(")), f104(v4)), f76(Str::lit(" "))), {
                let (h582, _) = v8.clone();
                h582
            }), f76(Str::lit(" "))), {
                let (h583, _) = v9;
                h583
            }), f76(Str::lit(")")));
            let v11: List<(Bytes, Bytes)> = List::cons((v5, {
                let (h584, _) = v8;
                h584
            }), v2);
            let m585 = v7.clone();
            match m585 {
                Adt0::C0 => {
                    Ok(((v10, v11), {
                        let (_, h593) = v9;
                        h593
                    }))
                }
                Adt0::C1(_) => {
                    Ok(((v10, v11), {
                        let (_, h594) = v9;
                        h594
                    }))
                }
                Adt0::C2(_, _, _) => {
                    Ok(((v10, v11), {
                        let (_, h595) = v9;
                        h595
                    }))
                }
                Adt0::C3(_) => {
                    Ok(((v10, v11), {
                        let (_, h596) = v9;
                        h596
                    }))
                }
                Adt0::C4(_, _) => {
                    Ok(((v10, v11), {
                        let (_, h597) = v9;
                        h597
                    }))
                }
                Adt0::C5(_, _) => {
                    Ok(((v10, v11), {
                        let (_, h598) = v9;
                        h598
                    }))
                }
                Adt0::C6(_, _) => {
                    Ok(((v10, v11), {
                        let (_, h599) = v9;
                        h599
                    }))
                }
                Adt0::C7(_, _, _, _) => {
                    Ok(((v10, v11), {
                        let (_, h600) = v9;
                        h600
                    }))
                }
                Adt0::C8(_) => {
                    Ok(((v10, v11), {
                        let (_, h601) = v9;
                        h601
                    }))
                }
                Adt0::C9 => {
                    Ok(((v10, v11), {
                        let (_, h586) = v9;
                        h586
                    }))
                }
                Adt0::C10(_, _) => {
                    Ok(((v10, v11), {
                        let (_, h602) = v9;
                        h602
                    }))
                }
                Adt0::C11(_, _, _, _) => {
                    let v16: ((Bytes, List<(Bytes, Bytes)>), u64) = f82(v7, v1, v11, {
                        let (_, h587) = v9;
                        h587
                    })?;
                    Ok(((f77(f77(v10, f76(Str::lit(" "))), {
                        let (h588, _) = {
                            let (h589, _) = v16.clone();
                            h589
                        };
                        h588
                    }), {
                        let (_, h590) = {
                            let (h591, _) = v16;
                            h591
                        };
                        h590
                    }), {
                        let (_, h592) = v16;
                        h592
                    }))
                }
            }
        }
    }
}

pub fn f83(v0: Bytes, v1: bool, v2: u64) -> R<(Bytes, u64)> {
    if v1 {
        Ok((f77(f76(Str::lit("x")), f41(v2)), {
            let a605 = v2;
            let a606 = 1u64;
            nat_add(a605, a606)?
        }))
    } else {
        Ok((v0, v2))
    }
}

pub fn f84(v0: Bytes, v1: u64) -> Bytes {
    let m610 = {
        let a607 = v0;
        let a608 = v1;
        let a609 = 1u64;
        slice_bytes(a607, a608, a609)
    };
    match m610 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f85(v0: Fn5, p615: u64, p616: (u64, Bytes)) -> ((u64, Bytes), bool) {
    let m614 = (p615, p616);
    {
        let mut m614 = m614;
        loop {
            let step = {
                let (v1, v2) = m614;
                let m611 = v1;
                if m611 == 0 {
                    Err::<(u64, (u64, Bytes)), ((u64, Bytes), bool)>((v2, false))
                } else {
                    let v3 = m611 - 1;
                    let m613 = {
                        let c612: Fn5 = v0.clone();
                        c612.apply(v2.clone())
                    };
                    match m613 {
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
                Ok(next) => { m614 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f86(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f105(v1, v0, v2)
}

pub fn f87(v0: Fn6, p620: Adt0, p621: List<Adt9>) -> Adt0 {
    let m619 = (p620, p621);
    {
        let mut m619 = m619;
        loop {
            let step = {
                let (v1, v2) = m619;
                let m617 = v2;
                match m617.uncons() {
                    None => {
                        Err::<(Adt0, List<Adt9>), Adt0>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt0, List<Adt9>), Adt0>(({
                            let c618: Fn6 = v0.clone();
                            c618.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m619 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f88(v0: Adt0, v1: Adt9) -> Adt0 {
    Adt0::C11(match v1.clone() {
        Adt9::C0(h622, _, _) => {
            h622
        }
    }, match v1.clone() {
        Adt9::C0(_, h623, _) => {
            h623
        }
    }, std::rc::Rc::new(match v1 {
        Adt9::C0(_, _, h624) => {
            h624
        }
    }), std::rc::Rc::new(v0))
}

pub fn f89(v0: List<Adt9>) -> List<Adt9> {
    f93(Fn7::F106, List::<Adt9>::nil(), v0)
}

pub fn f90(v0: Adt4, v1: Adt6, v2: List<Adt9>, v3: Bytes, v4: Adt0) -> R<Adt11> {
    let m625 = v0;
    match m625 {
        Adt4::C0 => {
            Ok(Adt11::C0(v2, f13(v1)))
        }
        Adt4::C1(r634) => {
            let v5 = (*r634).clone();
            let m626 = v2;
            match m626.uncons() {
                None => {
                    Ok(Adt11::C0(List::<Adt9>::nil(), v1))
                }
                Some((v6, v7)) => {
                    let v8: Adt8 = f91(v5.clone(), v1, match v6.clone() {
                        Adt9::C0(_, _, h627) => {
                            h627
                        }
                    }, v3.clone(), v4.clone())?;
                    let v9: Adt11 = f90(v5, match v8.clone() {
                        Adt8::C0(_, h628) => {
                            h628
                        }
                    }, v7, v3, v4)?;
                    Ok(Adt11::C0(List::cons(Adt9::C0(match v6.clone() {
                        Adt9::C0(h629, _, _) => {
                            h629
                        }
                    }, match v6 {
                        Adt9::C0(_, h630, _) => {
                            h630
                        }
                    }, match v8 {
                        Adt8::C0(h631, _) => {
                            h631
                        }
                    }), match v9.clone() {
                        Adt11::C0(h632, _) => {
                            h632
                        }
                    }), match v9 {
                        Adt11::C0(_, h633) => {
                            h633
                        }
                    }))
                }
            }
        }
    }
}

pub fn f91(v0: Adt4, v1: Adt6, v2: Adt0, v3: Bytes, v4: Adt0) -> R<Adt8> {
    let m635 = f73(v0.clone(), v4.clone(), List::<Bytes>::nil())?;
    match m635 {
        None => {
            Ok(Adt8::C0(v2, f13(v1)))
        }
        Some(v5) => {
            f107(v0, v1, v2, v3, v4, v5)
        }
    }
}

pub fn f92(v0: Bytes, v1: Option<Adt0>, v2: (Bytes, Adt0)) -> Option<Adt0> {
    let m636 = v1.clone();
    match m636 {
        None => {
            let a638 = {
                let (h637, _) = v2;
                h637
            };
            let a639 = v0;
            if equal(a638, a639) {
                Some({
                    let (_, h640) = v2;
                    h640
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

pub fn f93(v0: Fn7, p644: List<Adt9>, p645: List<Adt9>) -> List<Adt9> {
    let m643 = (p644, p645);
    {
        let mut m643 = m643;
        loop {
            let step = {
                let (v1, v2) = m643;
                let m641 = v2;
                match m641.uncons() {
                    None => {
                        Err::<(List<Adt9>, List<Adt9>), List<Adt9>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt9>, List<Adt9>), List<Adt9>>(({
                            let c642: Fn7 = v0.clone();
                            c642.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m643 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f94(v0: List<Adt9>, v1: Adt9) -> List<Adt9> {
    List::cons(v1, v0)
}

pub fn f95(v0: List<Adt9>, v1: Adt9) -> List<Adt9> {
    List::cons(v1, v0)
}

pub fn f96(v0: u64) -> Bytes {
    f104(v0)
}

pub fn f97(v0: List<Bytes>, v1: Bytes) -> R<bool> {
    f74(Fn4::F108(v1), false, v0)
}

pub fn f98(v0: Option<List<Bytes>>, v1: Option<List<Bytes>>) -> Option<List<Bytes>> {
    let m646 = v0;
    match m646 {
        None => {
            None::<List<Bytes>>
        }
        Some(v2) => {
            let m647 = v1;
            match m647 {
                None => {
                    None::<List<Bytes>>
                }
                Some(v3) => {
                    Some(f66(Fn3::F109, v3, v2))
                }
            }
        }
    }
}

pub fn f99(v0: Adt4, v1: Adt0, v2: List<Bytes>, v3: Adt0) -> R<Option<List<Bytes>>> {
    let m648 = v0;
    match m648 {
        Adt4::C0 => {
            Ok(None::<List<Bytes>>)
        }
        Adt4::C1(r652) => {
            let v4 = (*r652).clone();
            let m649 = v1;
            match m649 {
                Adt0::C0 => {
                    f73(v4, v3, v2)
                }
                Adt0::C1(_) => {
                    f73(v4, v3, v2)
                }
                Adt0::C2(_, _, _) => {
                    f73(v4, v3, v2)
                }
                Adt0::C3(_) => {
                    f73(v4, v3, v2)
                }
                Adt0::C4(_, _) => {
                    f73(v4, v3, v2)
                }
                Adt0::C5(_, _) => {
                    f73(v4, v3, v2)
                }
                Adt0::C6(_, _) => {
                    f73(v4, v3, v2)
                }
                Adt0::C7(_, _, _, _) => {
                    f73(v4, v3, v2)
                }
                Adt0::C8(_) => {
                    f73(v4, v3, v2)
                }
                Adt0::C9 => {
                    f73(v4, v3, v2)
                }
                Adt0::C10(_, _) => {
                    f73(v4, v3, v2)
                }
                Adt0::C11(_, v6, r650, r651) => {
                    let v7 = (*r650).clone();
                    let v8 = (*r651).clone();
                    Ok(f98(f73(v4.clone(), v7, v2.clone())?, f99(v4, v8, List::cons(v6, v2), v3)?))
                }
            }
        }
    }
}

pub fn f100(v0: Adt1) -> Bytes {
    let m653 = v0;
    match m653 {
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
        Adt1::C6(r654, r655) => {
            let v7 = (*r654).clone();
            let v8 = (*r655).clone();
            f77(f77(f76(Str::lit(" ")), f78(v7)), f100(v8))
        }
    }
}

pub fn f101(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m659 = {
        let a656 = v0;
        let a657 = v1;
        let a658 = v2;
        slice_bytes(a656, a657, a658)
    };
    match m659 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f102(v0: Fn8, p663: Option<Bytes>, p664: List<(Bytes, Bytes)>) -> Option<Bytes> {
    let m662 = (p663, p664);
    {
        let mut m662 = m662;
        loop {
            let step = {
                let (v1, v2) = m662;
                let m660 = v2;
                match m660.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<(Bytes, Bytes)>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<(Bytes, Bytes)>), Option<Bytes>>(({
                            let c661: Fn8 = v0.clone();
                            c661.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m662 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f103(v0: Bytes, v1: Option<Bytes>, v2: (Bytes, Bytes)) -> Option<Bytes> {
    let m665 = v1.clone();
    match m665 {
        None => {
            let a667 = {
                let (h666, _) = v2;
                h666
            };
            let a668 = v0;
            if equal(a667, a668) {
                Some({
                    let (_, h669) = v2;
                    h669
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

pub fn f104(v0: u64) -> Bytes {
    let a670 = v0;
    let a671 = 1u64;
    if nat_eq(a670, a671) {
        f76(Str::lit("implicit"))
    } else {
        let a672 = v0;
        let a673 = 2u64;
        if nat_eq(a672, a673) {
            f76(Str::lit("instance"))
        } else {
            f76(Str::lit("explicit"))
        }
    }
}

pub fn f105(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h674, _) = v2;
        h674
    };
    let a675 = v3;
    let a676 = 0u64;
    if nat_eq(a675, a676) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a677 = v3;
            let a678 = v1;
            let a679 = 0u64;
            nat_quot(a677, a678, a679)
        }, {
            let a684 = f84(v0, {
                let a680 = v3;
                let a681 = v1;
                let a682 = 0u64;
                nat_rem(a680, a681, a682)
            });
            let a685 = {
                let (_, h683) = v2;
                h683
            };
            append_bytes(a684, a685)
        }))
    }
}

pub fn f106(v0: List<Adt9>, v1: Adt9) -> List<Adt9> {
    List::cons(v1, v0)
}

pub fn f107(v0: Adt4, v1: Adt6, v2: Adt0, v3: Bytes, v4: Adt0, v5: List<Bytes>) -> R<Adt8> {
    let m686 = v0;
    match m686 {
        Adt4::C0 => {
            Ok(Adt8::C0(v2, f13(v1)))
        }
        Adt4::C1(r738) => {
            let v6 = (*r738).clone();
            let m687 = v2;
            match m687 {
                Adt0::C0 => {
                    Ok(Adt8::C0(Adt0::C0, v1))
                }
                Adt0::C1(v38) => {
                    Ok(Adt8::C0(Adt0::C1(v38), v1))
                }
                Adt0::C2(v39, v40, v41) => {
                    Ok(Adt8::C0(Adt0::C2(v39, v40, v41), v1))
                }
                Adt0::C3(v7) => {
                    let a688 = v7.clone();
                    let a689 = v3;
                    if equal(a688, a689) {
                        Ok(Adt8::C0(v4, v1))
                    } else {
                        Ok(Adt8::C0(Adt0::C3(v7), v1))
                    }
                }
                Adt0::C4(r725, r726) => {
                    let v8 = (*r725).clone();
                    let v9 = (*r726).clone();
                    let v10: Adt8 = f107(v6.clone(), v1, v8, v3.clone(), v4.clone(), v5.clone())?;
                    let v11: Adt8 = f107(v6, match v10.clone() {
                        Adt8::C0(_, h690) => {
                            h690
                        }
                    }, v9, v3, v4, v5)?;
                    Ok(Adt8::C0(Adt0::C4(std::rc::Rc::new(match v10 {
                        Adt8::C0(h691, _) => {
                            h691
                        }
                    }), std::rc::Rc::new(match v11.clone() {
                        Adt8::C0(h692, _) => {
                            h692
                        }
                    })), match v11 {
                        Adt8::C0(_, h693) => {
                            h693
                        }
                    }))
                }
                Adt0::C5(r727, r728) => {
                    let v16 = (*r727).clone();
                    let v17 = (*r728).clone();
                    let v18: Adt12 = f110(v6.clone(), v1, v16, v3.clone(), v4.clone(), v5.clone(), List::<(Bytes, Bytes)>::nil(), false)?;
                    let v19: Adt8 = f111(v6, v18.clone(), v17, v3, v4, v5)?;
                    Ok(Adt8::C0(Adt0::C5(std::rc::Rc::new(match v18 {
                        Adt12::C0(h698, _, _, _) => {
                            h698
                        }
                    }), std::rc::Rc::new(match v19.clone() {
                        Adt8::C0(h699, _) => {
                            h699
                        }
                    })), match v19 {
                        Adt8::C0(_, h700) => {
                            h700
                        }
                    }))
                }
                Adt0::C6(r729, r730) => {
                    let v20 = (*r729).clone();
                    let v21 = (*r730).clone();
                    let v22: Adt12 = f110(v6.clone(), v1, v20, v3.clone(), v4.clone(), v5.clone(), List::<(Bytes, Bytes)>::nil(), false)?;
                    let v23: Adt8 = f111(v6, v22.clone(), v21, v3, v4, v5)?;
                    Ok(Adt8::C0(Adt0::C6(std::rc::Rc::new(match v22 {
                        Adt12::C0(h701, _, _, _) => {
                            h701
                        }
                    }), std::rc::Rc::new(match v23.clone() {
                        Adt8::C0(h702, _) => {
                            h702
                        }
                    })), match v23 {
                        Adt8::C0(_, h703) => {
                            h703
                        }
                    }))
                }
                Adt0::C7(v24, r731, r732, r733) => {
                    let v25 = (*r731).clone();
                    let v26 = (*r732).clone();
                    let v27 = (*r733).clone();
                    let v28: Adt8 = f107(v6.clone(), v1, v25, v3.clone(), v4.clone(), v5.clone())?;
                    let v29: Adt8 = f107(v6.clone(), match v28.clone() {
                        Adt8::C0(_, h704) => {
                            h704
                        }
                    }, v26, v3.clone(), v4.clone(), v5.clone())?;
                    let a705 = v24.clone();
                    let a706 = v3.clone();
                    if equal(a705, a706) {
                        Ok(Adt8::C0(Adt0::C7(v24, std::rc::Rc::new(match v28 {
                            Adt8::C0(h707, _) => {
                                h707
                            }
                        }), std::rc::Rc::new(match v29.clone() {
                            Adt8::C0(h708, _) => {
                                h708
                            }
                        }), std::rc::Rc::new(v27)), match v29 {
                            Adt8::C0(_, h709) => {
                                h709
                            }
                        }))
                    } else {
                        if f97(v5.clone(), v24.clone())? {
                            let v30: (Bytes, Adt6) = f112(match v29.clone() {
                                Adt8::C0(_, h710) => {
                                    h710
                                }
                            }, v24.clone())?;
                            let v31: Adt8 = f107(v6.clone(), {
                                let (_, h711) = v30.clone();
                                h711
                            }, v27, v24, Adt0::C3({
                                let (h712, _) = v30.clone();
                                h712
                            }), List::<Bytes>::nil())?;
                            let v32: Adt8 = f107(v6, match v31.clone() {
                                Adt8::C0(_, h713) => {
                                    h713
                                }
                            }, match v31 {
                                Adt8::C0(h714, _) => {
                                    h714
                                }
                            }, v3, v4, v5)?;
                            Ok(Adt8::C0(Adt0::C7({
                                let (h715, _) = v30;
                                h715
                            }, std::rc::Rc::new(match v28 {
                                Adt8::C0(h716, _) => {
                                    h716
                                }
                            }), std::rc::Rc::new(match v29 {
                                Adt8::C0(h717, _) => {
                                    h717
                                }
                            }), std::rc::Rc::new(match v32.clone() {
                                Adt8::C0(h718, _) => {
                                    h718
                                }
                            })), match v32 {
                                Adt8::C0(_, h719) => {
                                    h719
                                }
                            }))
                        } else {
                            let v33: Adt8 = f107(v6, match v29.clone() {
                                Adt8::C0(_, h720) => {
                                    h720
                                }
                            }, v27, v3, v4, v5)?;
                            Ok(Adt8::C0(Adt0::C7(v24, std::rc::Rc::new(match v28 {
                                Adt8::C0(h721, _) => {
                                    h721
                                }
                            }), std::rc::Rc::new(match v29 {
                                Adt8::C0(h722, _) => {
                                    h722
                                }
                            }), std::rc::Rc::new(match v33.clone() {
                                Adt8::C0(h723, _) => {
                                    h723
                                }
                            })), match v33 {
                                Adt8::C0(_, h724) => {
                                    h724
                                }
                            }))
                        }
                    }
                }
                Adt0::C8(v42) => {
                    Ok(Adt8::C0(Adt0::C8(v42), v1))
                }
                Adt0::C9 => {
                    Ok(Adt8::C0(Adt0::C9, v1))
                }
                Adt0::C10(r734, r735) => {
                    let v12 = (*r734).clone();
                    let v13 = (*r735).clone();
                    let v14: Adt8 = f107(v6.clone(), v1, v12, v3.clone(), v4.clone(), v5.clone())?;
                    let v15: Adt8 = f107(v6, match v14.clone() {
                        Adt8::C0(_, h694) => {
                            h694
                        }
                    }, v13, v3, v4, v5)?;
                    Ok(Adt8::C0(Adt0::C10(std::rc::Rc::new(match v14 {
                        Adt8::C0(h695, _) => {
                            h695
                        }
                    }), std::rc::Rc::new(match v15.clone() {
                        Adt8::C0(h696, _) => {
                            h696
                        }
                    })), match v15 {
                        Adt8::C0(_, h697) => {
                            h697
                        }
                    }))
                }
                Adt0::C11(v34, v35, r736, r737) => {
                    let v36 = (*r736).clone();
                    let v37 = (*r737).clone();
                    Ok(Adt8::C0(Adt0::C11(v34, v35, std::rc::Rc::new(v36), std::rc::Rc::new(v37)), v1))
                }
            }
        }
    }
}

pub fn f108(v0: Bytes, v1: bool, v2: Bytes) -> bool {
    if v1 {
        true
    } else {
        let a739 = v2;
        let a740 = v0;
        equal(a739, a740)
    }
}

pub fn f109(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f110(v0: Adt4, v1: Adt6, v2: Adt0, v3: Bytes, v4: Adt0, v5: List<Bytes>, v6: List<(Bytes, Bytes)>, v7: bool) -> R<Adt12> {
    let m741 = v0;
    match m741 {
        Adt4::C0 => {
            Ok(Adt12::C0(v2, v6, v7, f13(v1)))
        }
        Adt4::C1(r768) => {
            let v8 = (*r768).clone();
            let m742 = v2.clone();
            match m742 {
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
                Adt0::C11(v9, v10, r766, r767) => {
                    let v11 = (*r766).clone();
                    let v12 = (*r767).clone();
                    let v13: Adt8 = f111(v8.clone(), Adt12::C0(v2, v6.clone(), v7, v1), v11, v3.clone(), v4.clone(), v5.clone())?;
                    let v14: Adt8 = v13;
                    let a743 = v10.clone();
                    let a744 = v3.clone();
                    if equal(a743, a744) {
                        let v15: Adt12 = f110(v8, match v14.clone() {
                            Adt8::C0(_, h745) => {
                                h745
                            }
                        }, v12, v3, v4, v5, v6, true)?;
                        Ok(Adt12::C0(Adt0::C11(v9, v10, std::rc::Rc::new(match v14 {
                            Adt8::C0(h746, _) => {
                                h746
                            }
                        }), std::rc::Rc::new(match v15.clone() {
                            Adt12::C0(h747, _, _, _) => {
                                h747
                            }
                        })), match v15.clone() {
                            Adt12::C0(_, h748, _, _) => {
                                h748
                            }
                        }, match v15.clone() {
                            Adt12::C0(_, _, h749, _) => {
                                h749
                            }
                        }, match v15 {
                            Adt12::C0(_, _, _, h750) => {
                                h750
                            }
                        }))
                    } else {
                        if f97(v5.clone(), v10.clone())? {
                            let v16: (Bytes, Adt6) = f112(match v14.clone() {
                                Adt8::C0(_, h751) => {
                                    h751
                                }
                            }, v10.clone())?;
                            let v17: Adt12 = f110(v8, {
                                let (_, h752) = v16.clone();
                                h752
                            }, v12, v3, v4, v5, List::cons((v10, {
                                let (h753, _) = v16.clone();
                                h753
                            }), v6), v7)?;
                            Ok(Adt12::C0(Adt0::C11(v9, {
                                let (h754, _) = v16;
                                h754
                            }, std::rc::Rc::new(match v14 {
                                Adt8::C0(h755, _) => {
                                    h755
                                }
                            }), std::rc::Rc::new(match v17.clone() {
                                Adt12::C0(h756, _, _, _) => {
                                    h756
                                }
                            })), match v17.clone() {
                                Adt12::C0(_, h757, _, _) => {
                                    h757
                                }
                            }, match v17.clone() {
                                Adt12::C0(_, _, h758, _) => {
                                    h758
                                }
                            }, match v17 {
                                Adt12::C0(_, _, _, h759) => {
                                    h759
                                }
                            }))
                        } else {
                            let v18: Adt12 = f110(v8, match v14.clone() {
                                Adt8::C0(_, h760) => {
                                    h760
                                }
                            }, v12, v3, v4, v5, v6, v7)?;
                            Ok(Adt12::C0(Adt0::C11(v9, v10, std::rc::Rc::new(match v14 {
                                Adt8::C0(h761, _) => {
                                    h761
                                }
                            }), std::rc::Rc::new(match v18.clone() {
                                Adt12::C0(h762, _, _, _) => {
                                    h762
                                }
                            })), match v18.clone() {
                                Adt12::C0(_, h763, _, _) => {
                                    h763
                                }
                            }, match v18.clone() {
                                Adt12::C0(_, _, h764, _) => {
                                    h764
                                }
                            }, match v18 {
                                Adt12::C0(_, _, _, h765) => {
                                    h765
                                }
                            }))
                        }
                    }
                }
            }
        }
    }
}

pub fn f111(v0: Adt4, v1: Adt12, v2: Adt0, v3: Bytes, v4: Adt0, v5: List<Bytes>) -> R<Adt8> {
    let m769 = v0;
    match m769 {
        Adt4::C0 => {
            Ok(Adt8::C0(v2, f13(match v1 {
                Adt12::C0(_, _, _, h770) => {
                    h770
                }
            })))
        }
        Adt4::C1(r776) => {
            let v6 = (*r776).clone();
            let v7: Adt8 = f113(v6.clone(), match v1.clone() {
                Adt12::C0(_, _, _, h771) => {
                    h771
                }
            }, v2, f114(match v1.clone() {
                Adt12::C0(_, h772, _, _) => {
                    h772
                }
            }))?;
            if match v1 {
                Adt12::C0(_, _, h773, _) => {
                    h773
                }
            } {
                Ok(v7)
            } else {
                f107(v6, match v7.clone() {
                    Adt8::C0(_, h774) => {
                        h774
                    }
                }, match v7 {
                    Adt8::C0(h775, _) => {
                        h775
                    }
                }, v3, v4, v5)
            }
        }
    }
}

pub fn f112(v0: Adt6, v1: Bytes) -> R<(Bytes, Adt6)> {
    let v2: u64 = {
        let a778 = match v0.clone() {
            Adt6::C0(_, _, _, h777, _, _, _) => {
                h777
            }
        };
        let a779 = 1u64;
        nat_add(a778, a779)?
    };
    Ok((f8(f8(v1, Bytes::lit(&[0x23])), f41(v2)), f33(v0.clone(), match v0.clone() {
        Adt6::C0(h780, _, _, _, _, _, _) => {
            h780
        }
    }, match v0.clone() {
        Adt6::C0(_, h781, _, _, _, _, _) => {
            h781
        }
    }, match v0.clone() {
        Adt6::C0(_, _, h782, _, _, _, _) => {
            h782
        }
    }, v2, match v0 {
        Adt6::C0(_, _, _, _, h783, _, _) => {
            h783
        }
    })))
}

pub fn f113(p792: Adt4, p793: Adt6, p794: Adt0, p795: List<(Bytes, Bytes)>) -> R<Adt8> {
    let m791 = (p792, (p793, (p794, p795)));
    {
        let mut m791 = m791;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m791;
                let m784 = v0;
                match m784 {
                    Adt4::C0 => {
                        Err::<(Adt4, (Adt6, (Adt0, List<(Bytes, Bytes)>))), R<Adt8>>(Ok(Adt8::C0(v2, f13(v1))))
                    }
                    Adt4::C1(r790) => {
                        let v4 = (*r790).clone();
                        let m785 = v3;
                        match m785.uncons() {
                            None => {
                                Err::<(Adt4, (Adt6, (Adt0, List<(Bytes, Bytes)>))), R<Adt8>>(Ok(Adt8::C0(v2, v1)))
                            }
                            Some((v5, v6)) => {
                                let v7: Adt8 = f107(v4.clone(), v1, v2, {
                                    let (h786, _) = v5;
                                    h786
                                }, Adt0::C3({
                                    let (_, h787) = v5;
                                    h787
                                }), List::<Bytes>::nil())?;
                                Ok::<(Adt4, (Adt6, (Adt0, List<(Bytes, Bytes)>))), R<Adt8>>((v4, (match v7.clone() {
                                    Adt8::C0(_, h788) => {
                                        h788
                                    }
                                }, (match v7 {
                                    Adt8::C0(h789, _) => {
                                        h789
                                    }
                                }, v6))))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m791 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f114(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f115(Fn9::F116, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f115(v0: Fn9, p799: List<(Bytes, Bytes)>, p800: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m798 = (p799, p800);
    {
        let mut m798 = m798;
        loop {
            let step = {
                let (v1, v2) = m798;
                let m796 = v2;
                match m796.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c797: Fn9 = v0.clone();
                            c797.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m798 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f116(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn run(p0: Adt0, p1: Option<Adt0>, p2: List<Adt2>, p3: u64, p4: u64) -> R<Result<Adt3, Bytes>> {
    f0(p0, p1, p2, p3, p4)
}
