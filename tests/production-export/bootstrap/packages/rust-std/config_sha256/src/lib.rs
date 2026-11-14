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
    C0(Bytes, Bytes, Bytes, List<Bytes>, List<Bytes>, Bytes, Bytes, Bytes, List<Adt1>, List<Adt2>, List<Adt3>, Adt4, Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(Bytes),
    C1(Bytes, Bytes),
    C2(Bytes, Bytes, Bytes, Bytes),
}

#[derive(Clone)]
pub enum Adt2 {
    C0(Bytes, Bytes),
}

#[derive(Clone)]
pub enum Adt3 {
    C0(Bytes, Bytes, Bytes),
}

#[derive(Clone)]
pub enum Adt4 {
    C0(u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt5 {
    C0(Bytes, Bytes, List<Bytes>, Bytes, List<Bytes>, Bytes, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt6 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt7 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt8 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32, Adt7, u64),
}

#[derive(Clone)]
pub enum Fn0 {
    F11,
}

impl Fn0 {
    pub fn apply(&self, p0: Bytes, p1: Adt2) -> R<Bytes> {
        match self {
            Fn0::F11 => f11(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F13,
}

impl Fn1 {
    pub fn apply(&self, p0: Bytes, p1: Adt3) -> R<Bytes> {
        match self {
            Fn1::F13 => f13(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F19(Bytes, u64, u64),
}

impl Fn2 {
    pub fn apply(&self, p0: (u64, Adt6)) -> R<(u64, Adt6)> {
        match self {
            Fn2::F19(k0, k1, k2) => f19(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F30,
}

impl Fn3 {
    pub fn apply(&self, p0: Bytes, p1: Adt1) -> R<Bytes> {
        match self {
            Fn3::F30 => f30(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F42,
    F61,
}

impl Fn4 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> R<List<Bytes>> {
        match self {
            Fn4::F42 => f42(p0, p1),
            Fn4::F61 => Ok(f61(p0, p1)),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F47,
}

impl Fn5 {
    pub fn apply(&self, p0: List<Adt1>, p1: (Bytes, Adt1)) -> List<Adt1> {
        match self {
            Fn5::F47 => f47(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F51,
}

impl Fn6 {
    pub fn apply(&self, p0: List<(Bytes, Adt1)>, p1: Adt1) -> List<(Bytes, Adt1)> {
        match self {
            Fn6::F51 => f51(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F54,
}

impl Fn7 {
    pub fn apply(&self, p0: Adt8) -> R<Adt8> {
        match self {
            Fn7::F54 => f54(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F58(Bytes),
    F80(Bytes),
    F85(u64, Bytes),
}

impl Fn8 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn8::F58(k0) => f58(k0.clone(), p0),
            Fn8::F80(k0) => f80(k0.clone(), p0),
            Fn8::F85(k0, k1) => Ok(f85(*k0, k1.clone(), p0)),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F60(Bytes),
}

impl Fn9 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Bytes) -> Option<Bytes> {
        match self {
            Fn9::F60(k0) => f60(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F64,
}

impl Fn10 {
    pub fn apply(&self, p0: List<(Bytes, Adt1)>, p1: (Bytes, Adt1)) -> List<(Bytes, Adt1)> {
        match self {
            Fn10::F64 => f64(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F67,
}

impl Fn11 {
    pub fn apply(&self, p0: List<List<(Bytes, Adt1)>>, p1: (Bytes, Adt1)) -> List<List<(Bytes, Adt1)>> {
        match self {
            Fn11::F67 => f67(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F69,
}

impl Fn12 {
    pub fn apply(&self, p0: List<List<(Bytes, Adt1)>>) -> R<Option<List<List<(Bytes, Adt1)>>>> {
        match self {
            Fn12::F69 => f69(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F82,
}

impl Fn13 {
    pub fn apply(&self, p0: List<List<(Bytes, Adt1)>>, p1: List<(Bytes, Adt1)>) -> List<List<(Bytes, Adt1)>> {
        match self {
            Fn13::F82 => f82(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F93,
}

impl Fn14 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn14::F93 => f93(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F107,
}

impl Fn15 {
    pub fn apply(&self, p0: (List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>)) -> R<Option<(List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>)>> {
        match self {
            Fn15::F107 => f107(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F112,
}

impl Fn16 {
    pub fn apply(&self, p0: (List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>))) -> Option<(List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>))> {
        match self {
            Fn16::F112 => f112(p0),
        }
    }
}

pub fn f0(v0: Adt0) -> R<Bytes> {
    f1(f2(v0)?)
}

pub fn f1(v0: Bytes) -> R<Bytes> {
    Ok(f3(f4(v0)?))
}

pub fn f2(v0: Adt0) -> R<Bytes> {
    let v1: Bytes = f5(f5(f5(f5(f5(f5(f5(f5(f5(f6(Str::lit("spec"), f7(Str::lit("lexlean/project/1")))?, f6(Str::lit("name"), match v0.clone() {
        Adt0::C0(h1, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1
        }
    })?), f6(Str::lit("language"), match v0.clone() {
        Adt0::C0(_, h2, _, _, _, _, _, _, _, _, _, _, _) => {
            h2
        }
    })?), f6(Str::lit("module_prefix"), match v0.clone() {
        Adt0::C0(_, _, h3, _, _, _, _, _, _, _, _, _, _) => {
            h3
        }
    })?), f8(Str::lit("source_roots"), match v0.clone() {
        Adt0::C0(_, _, _, h4, _, _, _, _, _, _, _, _, _) => {
            h4
        }
    })?), f8(Str::lit("entrypoints"), match v0.clone() {
        Adt0::C0(_, _, _, _, h5, _, _, _, _, _, _, _, _) => {
            h5
        }
    })?), f6(Str::lit("build_root"), match v0.clone() {
        Adt0::C0(_, _, _, _, _, h6, _, _, _, _, _, _, _) => {
            h6
        }
    })?), f6(Str::lit("lockfile"), match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, h7, _, _, _, _, _, _) => {
            h7
        }
    })?), f6(Str::lit("lean_workspace"), match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, h8, _, _, _, _, _) => {
            h8
        }
    })?), f6(Str::lit("lean_toolchain"), f9())?);
    let v2: Bytes = f10(Fn0::F11, Bytes::lit(&[]), match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, _, _, h9, _, _, _) => {
            h9
        }
    })?;
    let v3: Bytes = f12(Fn1::F13, Bytes::lit(&[]), match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, _, _, _, h10, _, _) => {
            h10
        }
    })?;
    let v5: Bytes = {
        let m12 = match v0.clone() {
            Adt0::C0(_, _, _, _, _, _, _, _, _, _, _, _, h11) => {
                h11
            }
        };
        match m12 {
            None => {
                Bytes::lit(&[])
            }
            Some(v4) => {
                f14(v4)?
            }
        }
    };
    Ok(f5(f5(f5(f5(f5(v1, f15(match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, _, h13, _, _, _, _) => {
            h13
        }
    })?), v2), v3), f16(match v0 {
        Adt0::C0(_, _, _, _, _, _, _, _, _, _, _, h14, _) => {
            h14
        }
    })?), v5))
}

pub fn f3(v0: Adt6) -> Bytes {
    let a35 = f17(match v0.clone() {
        Adt6::C0(h15, _, _, _, _, _, _, _) => {
            h15
        }
    });
    let a36 = {
        let a33 = f17(match v0.clone() {
            Adt6::C0(_, h16, _, _, _, _, _, _) => {
                h16
            }
        });
        let a34 = {
            let a31 = f17(match v0.clone() {
                Adt6::C0(_, _, h17, _, _, _, _, _) => {
                    h17
                }
            });
            let a32 = {
                let a29 = f17(match v0.clone() {
                    Adt6::C0(_, _, _, h18, _, _, _, _) => {
                        h18
                    }
                });
                let a30 = {
                    let a27 = f17(match v0.clone() {
                        Adt6::C0(_, _, _, _, h19, _, _, _) => {
                            h19
                        }
                    });
                    let a28 = {
                        let a25 = f17(match v0.clone() {
                            Adt6::C0(_, _, _, _, _, h20, _, _) => {
                                h20
                            }
                        });
                        let a26 = {
                            let a23 = f17(match v0.clone() {
                                Adt6::C0(_, _, _, _, _, _, h21, _) => {
                                    h21
                                }
                            });
                            let a24 = f17(match v0 {
                                Adt6::C0(_, _, _, _, _, _, _, h22) => {
                                    h22
                                }
                            });
                            append_bytes(a23, a24)
                        };
                        append_bytes(a25, a26)
                    };
                    append_bytes(a27, a28)
                };
                append_bytes(a29, a30)
            };
            append_bytes(a31, a32)
        };
        append_bytes(a33, a34)
    };
    append_bytes(a35, a36)
}

pub fn f4(v0: Bytes) -> R<Adt6> {
    let v1: u64 = {
        let a37 = v0.clone();
        length_bytes(a37)
    };
    let v2: u64 = {
        let a43 = {
            let a40 = {
                let a38 = v1;
                let a39 = 72u64;
                nat_add(a38, a39)?
            };
            let a41 = 64u64;
            let a42 = 0u64;
            nat_quot(a40, a41, a42)
        };
        let a44 = 64u64;
        nat_mul(a43, a44)?
    };
    let (_, h45) = f18(Fn2::F19(v0, v1, v2), {
        let a46 = v2;
        let a47 = 64u64;
        let a48 = 0u64;
        nat_quot(a46, a47, a48)
    }, (0u64, f20()))?;
    Ok(h45)
}

pub fn f5(v0: Bytes, v1: Bytes) -> Bytes {
    let a49 = v0;
    let a50 = v1;
    append_bytes(a49, a50)
}

pub fn f6(v0: Str, v1: Bytes) -> R<Bytes> {
    f21(f7(v0), v1)
}

pub fn f7(v0: Str) -> Bytes {
    let a51 = v0;
    utf8_encode(a51)
}

pub fn f8(v0: Str, v1: List<Bytes>) -> R<Bytes> {
    Ok(f22(f7(v0), f23(v1)?))
}

pub fn f9() -> Bytes {
    f24(Str::lit("leanprover/lean4:v4.32.2"))
}

pub fn f10(v0: Fn0, p55: Bytes, p56: List<Adt2>) -> R<Bytes> {
    let m54 = (p55, p56);
    {
        let mut m54 = m54;
        loop {
            let step = {
                let (v1, v2) = m54;
                let m52 = v2;
                match m52.uncons() {
                    None => {
                        Err::<(Bytes, List<Adt2>), R<Bytes>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Bytes, List<Adt2>), R<Bytes>>(({
                            let c53: Fn0 = v0.clone();
                            c53.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m54 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f11(v0: Bytes, v1: Adt2) -> R<Bytes> {
    Ok(f5(v0, f25(v1)?))
}

pub fn f12(v0: Fn1, p60: Bytes, p61: List<Adt3>) -> R<Bytes> {
    let m59 = (p60, p61);
    {
        let mut m59 = m59;
        loop {
            let step = {
                let (v1, v2) = m59;
                let m57 = v2;
                match m57.uncons() {
                    None => {
                        Err::<(Bytes, List<Adt3>), R<Bytes>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Bytes, List<Adt3>), R<Bytes>>(({
                            let c58: Fn1 = v0.clone();
                            c58.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m59 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f13(v0: Bytes, v1: Adt3) -> R<Bytes> {
    Ok(f5(v0, f26(v1)?))
}

pub fn f14(v0: Adt5) -> R<Bytes> {
    Ok(f5(f5(f5(f5(f5(f5(f5(f5(f27(Str::lit("[pdf]")), f6(Str::lit("mode"), f7(Str::lit("external")))?), f6(Str::lit("program"), match v0.clone() {
        Adt5::C0(h62, _, _, _, _, _, _) => {
            h62
        }
    })?), f28(Str::lit("program_sha256"), match v0.clone() {
        Adt5::C0(_, h63, _, _, _, _, _) => {
            h63
        }
    })?), f8(Str::lit("version_argv"), match v0.clone() {
        Adt5::C0(_, _, h64, _, _, _, _) => {
            h64
        }
    })?), f28(Str::lit("version_stdout_sha256"), match v0.clone() {
        Adt5::C0(_, _, _, h65, _, _, _) => {
            h65
        }
    })?), f8(Str::lit("compile_argv"), match v0.clone() {
        Adt5::C0(_, _, _, _, h66, _, _) => {
            h66
        }
    })?), f6(Str::lit("output"), match v0.clone() {
        Adt5::C0(_, _, _, _, _, h67, _) => {
            h67
        }
    })?), f8(Str::lit("resources"), match v0 {
        Adt5::C0(_, _, _, _, _, _, h68) => {
            h68
        }
    })?))
}

pub fn f15(v0: List<Adt1>) -> R<Bytes> {
    let m69 = v0.clone();
    match m69.uncons() {
        None => {
            Ok(f5(f7(Str::lit("lexicon_source = []")), Bytes::lit(&[0x0a])))
        }
        Some((_, _)) => {
            f29(Fn3::F30, Bytes::lit(&[]), f31(v0)?)
        }
    }
}

pub fn f16(v0: Adt4) -> R<Bytes> {
    Ok(f5(f5(f5(f5(f5(f5(f5(f5(f5(f5(f5(f27(Str::lit("[limits]")), f32(Str::lit("max_file_bytes"), match v0.clone() {
        Adt4::C0(h70, _, _, _, _, _, _, _, _, _, _) => {
            h70
        }
    })?), f32(Str::lit("max_total_source_bytes"), match v0.clone() {
        Adt4::C0(_, h71, _, _, _, _, _, _, _, _, _) => {
            h71
        }
    })?), f32(Str::lit("max_primitive_atoms"), match v0.clone() {
        Adt4::C0(_, _, h72, _, _, _, _, _, _, _, _) => {
            h72
        }
    })?), f32(Str::lit("max_token_lattice_edges"), match v0.clone() {
        Adt4::C0(_, _, _, h73, _, _, _, _, _, _, _) => {
            h73
        }
    })?), f32(Str::lit("max_parse_states"), match v0.clone() {
        Adt4::C0(_, _, _, _, h74, _, _, _, _, _, _) => {
            h74
        }
    })?), f32(Str::lit("max_ir_nodes"), match v0.clone() {
        Adt4::C0(_, _, _, _, _, h75, _, _, _, _, _) => {
            h75
        }
    })?), f32(Str::lit("max_scope_depth"), match v0.clone() {
        Adt4::C0(_, _, _, _, _, _, h76, _, _, _, _) => {
            h76
        }
    })?), f32(Str::lit("max_import_depth"), match v0.clone() {
        Adt4::C0(_, _, _, _, _, _, _, h77, _, _, _) => {
            h77
        }
    })?), f32(Str::lit("max_diagnostics"), match v0.clone() {
        Adt4::C0(_, _, _, _, _, _, _, _, h78, _, _) => {
            h78
        }
    })?), f32(Str::lit("max_child_output_bytes"), match v0.clone() {
        Adt4::C0(_, _, _, _, _, _, _, _, _, h79, _) => {
            h79
        }
    })?), f32(Str::lit("child_timeout_ms"), match v0 {
        Adt4::C0(_, _, _, _, _, _, _, _, _, _, h80) => {
            h80
        }
    })?))
}

pub fn f17(v0: u32) -> Bytes {
    let a109 = f33({
        let a81 = f34(v0, 28u32);
        let a82 = 15u32;
        fixed_u32::bit_and(a81, a82)
    });
    let a110 = {
        let a107 = f33({
            let a83 = f34(v0, 24u32);
            let a84 = 15u32;
            fixed_u32::bit_and(a83, a84)
        });
        let a108 = {
            let a105 = f33({
                let a85 = f34(v0, 20u32);
                let a86 = 15u32;
                fixed_u32::bit_and(a85, a86)
            });
            let a106 = {
                let a103 = f33({
                    let a87 = f34(v0, 16u32);
                    let a88 = 15u32;
                    fixed_u32::bit_and(a87, a88)
                });
                let a104 = {
                    let a101 = f33({
                        let a89 = f34(v0, 12u32);
                        let a90 = 15u32;
                        fixed_u32::bit_and(a89, a90)
                    });
                    let a102 = {
                        let a99 = f33({
                            let a91 = f34(v0, 8u32);
                            let a92 = 15u32;
                            fixed_u32::bit_and(a91, a92)
                        });
                        let a100 = {
                            let a97 = f33({
                                let a93 = f34(v0, 4u32);
                                let a94 = 15u32;
                                fixed_u32::bit_and(a93, a94)
                            });
                            let a98 = f33({
                                let a95 = v0;
                                let a96 = 15u32;
                                fixed_u32::bit_and(a95, a96)
                            });
                            append_bytes(a97, a98)
                        };
                        append_bytes(a99, a100)
                    };
                    append_bytes(a101, a102)
                };
                append_bytes(a103, a104)
            };
            append_bytes(a105, a106)
        };
        append_bytes(a107, a108)
    };
    append_bytes(a109, a110)
}

pub fn f18(v0: Fn2, p114: u64, p115: (u64, Adt6)) -> R<(u64, Adt6)> {
    let m113 = (p114, p115);
    {
        let mut m113 = m113;
        loop {
            let step = {
                let (v1, v2) = m113;
                let m111 = v1;
                if m111 == 0 {
                    Err::<(u64, (u64, Adt6)), R<(u64, Adt6)>>(Ok(v2))
                } else {
                    let v3 = m111 - 1;
                    Ok::<(u64, (u64, Adt6)), R<(u64, Adt6)>>((v3, {
                        let c112: Fn2 = v0.clone();
                        c112.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m113 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f19(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt6)) -> R<(u64, Adt6)> {
    Ok(({
        let a117 = {
            let (h116, _) = v3.clone();
            h116
        };
        let a118 = 64u64;
        nat_add(a117, a118)?
    }, f35({
        let (_, h119) = v3;
        h119
    }, f36(v0, v1, v2, {
        let (h120, _) = v3;
        h120
    })?)?))
}

pub fn f20() -> Adt6 {
    Adt6::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f21(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f22(v0, f37(v1)?))
}

pub fn f22(v0: Bytes, v1: Bytes) -> Bytes {
    f38(f38(f38(v0, f24(Str::lit(" = "))), v1), Bytes::lit(&[0x0a]))
}

pub fn f23(v0: List<Bytes>) -> R<Bytes> {
    Ok(f38(f38(f24(Str::lit("[")), f39(f40(f41(Fn4::F42, List::<Bytes>::nil(), v0)?)?, f24(Str::lit(", ")))), f24(Str::lit("]"))))
}

pub fn f24(v0: Str) -> Bytes {
    let a121 = v0;
    utf8_encode(a121)
}

pub fn f25(v0: Adt2) -> R<Bytes> {
    Ok(f5(f5(f27(Str::lit("[[artifact_source]]")), f28(Str::lit("sha256"), match v0.clone() {
        Adt2::C0(h122, _) => {
            h122
        }
    })?), f6(Str::lit("path"), match v0 {
        Adt2::C0(_, h123) => {
            h123
        }
    })?))
}

pub fn f26(v0: Adt3) -> R<Bytes> {
    Ok(f5(f5(f5(f5(f27(Str::lit("[[source_package]]")), f6(Str::lit("package"), match v0.clone() {
        Adt3::C0(h124, _, _) => {
            h124
        }
    })?), f6(Str::lit("version"), match v0.clone() {
        Adt3::C0(_, h125, _) => {
            h125
        }
    })?), f6(Str::lit("kind"), f7(Str::lit("path")))?), f6(Str::lit("path"), match v0 {
        Adt3::C0(_, _, h126) => {
            h126
        }
    })?))
}

pub fn f27(v0: Str) -> Bytes {
    f43(f7(v0))
}

pub fn f28(v0: Str, v1: Bytes) -> R<Bytes> {
    f44(f7(v0), v1)
}

pub fn f29(v0: Fn3, p130: Bytes, p131: List<Adt1>) -> R<Bytes> {
    let m129 = (p130, p131);
    {
        let mut m129 = m129;
        loop {
            let step = {
                let (v1, v2) = m129;
                let m127 = v2;
                match m127.uncons() {
                    None => {
                        Err::<(Bytes, List<Adt1>), R<Bytes>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Bytes, List<Adt1>), R<Bytes>>(({
                            let c128: Fn3 = v0.clone();
                            c128.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m129 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f30(v0: Bytes, v1: Adt1) -> R<Bytes> {
    Ok(f5(v0, f45(v1)?))
}

pub fn f31(v0: List<Adt1>) -> R<List<Adt1>> {
    Ok(f46(Fn5::F47, List::<Adt1>::nil(), f48(f49(f48(f50(Fn6::F51, List::<(Bytes, Adt1)>::nil(), v0)))?)))
}

pub fn f32(v0: Str, v1: u64) -> R<Bytes> {
    Ok(f22(f7(v0), f52(v1)?))
}

pub fn f33(v0: u32) -> Bytes {
    let a134 = {
        let a132 = v0;
        let a133 = 8u32;
        fixed_u32::bit_and(a132, a133)
    };
    let a135 = 0u32;
    if equal(a134, a135) {
        let a138 = {
            let a136 = v0;
            let a137 = 4u32;
            fixed_u32::bit_and(a136, a137)
        };
        let a139 = 0u32;
        if equal(a138, a139) {
            let a142 = {
                let a140 = v0;
                let a141 = 2u32;
                fixed_u32::bit_and(a140, a141)
            };
            let a143 = 0u32;
            if equal(a142, a143) {
                let a146 = {
                    let a144 = v0;
                    let a145 = 1u32;
                    fixed_u32::bit_and(a144, a145)
                };
                let a147 = 0u32;
                if equal(a146, a147) {
                    Bytes::lit(&[0x30])
                } else {
                    Bytes::lit(&[0x31])
                }
            } else {
                let a150 = {
                    let a148 = v0;
                    let a149 = 1u32;
                    fixed_u32::bit_and(a148, a149)
                };
                let a151 = 0u32;
                if equal(a150, a151) {
                    Bytes::lit(&[0x32])
                } else {
                    Bytes::lit(&[0x33])
                }
            }
        } else {
            let a154 = {
                let a152 = v0;
                let a153 = 2u32;
                fixed_u32::bit_and(a152, a153)
            };
            let a155 = 0u32;
            if equal(a154, a155) {
                let a158 = {
                    let a156 = v0;
                    let a157 = 1u32;
                    fixed_u32::bit_and(a156, a157)
                };
                let a159 = 0u32;
                if equal(a158, a159) {
                    Bytes::lit(&[0x34])
                } else {
                    Bytes::lit(&[0x35])
                }
            } else {
                let a162 = {
                    let a160 = v0;
                    let a161 = 1u32;
                    fixed_u32::bit_and(a160, a161)
                };
                let a163 = 0u32;
                if equal(a162, a163) {
                    Bytes::lit(&[0x36])
                } else {
                    Bytes::lit(&[0x37])
                }
            }
        }
    } else {
        let a166 = {
            let a164 = v0;
            let a165 = 4u32;
            fixed_u32::bit_and(a164, a165)
        };
        let a167 = 0u32;
        if equal(a166, a167) {
            let a170 = {
                let a168 = v0;
                let a169 = 2u32;
                fixed_u32::bit_and(a168, a169)
            };
            let a171 = 0u32;
            if equal(a170, a171) {
                let a174 = {
                    let a172 = v0;
                    let a173 = 1u32;
                    fixed_u32::bit_and(a172, a173)
                };
                let a175 = 0u32;
                if equal(a174, a175) {
                    Bytes::lit(&[0x38])
                } else {
                    Bytes::lit(&[0x39])
                }
            } else {
                let a178 = {
                    let a176 = v0;
                    let a177 = 1u32;
                    fixed_u32::bit_and(a176, a177)
                };
                let a179 = 0u32;
                if equal(a178, a179) {
                    Bytes::lit(&[0x61])
                } else {
                    Bytes::lit(&[0x62])
                }
            }
        } else {
            let a182 = {
                let a180 = v0;
                let a181 = 2u32;
                fixed_u32::bit_and(a180, a181)
            };
            let a183 = 0u32;
            if equal(a182, a183) {
                let a186 = {
                    let a184 = v0;
                    let a185 = 1u32;
                    fixed_u32::bit_and(a184, a185)
                };
                let a187 = 0u32;
                if equal(a186, a187) {
                    Bytes::lit(&[0x63])
                } else {
                    Bytes::lit(&[0x64])
                }
            } else {
                let a190 = {
                    let a188 = v0;
                    let a189 = 1u32;
                    fixed_u32::bit_and(a188, a189)
                };
                let a191 = 0u32;
                if equal(a190, a191) {
                    Bytes::lit(&[0x65])
                } else {
                    Bytes::lit(&[0x66])
                }
            }
        }
    }
}

pub fn f34(v0: u32, v1: u32) -> u32 {
    let m194 = {
        let a192 = v0;
        let a193 = v1;
        fixed_u32::shift_right(a192, a193)
    };
    match m194 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f35(v0: Adt6, v1: Adt7) -> R<Adt6> {
    let v2: Adt8 = f53(Fn7::F54, 64u64, Adt8::C0(match v0.clone() {
        Adt6::C0(h195, _, _, _, _, _, _, _) => {
            h195
        }
    }, match v0.clone() {
        Adt6::C0(_, h196, _, _, _, _, _, _) => {
            h196
        }
    }, match v0.clone() {
        Adt6::C0(_, _, h197, _, _, _, _, _) => {
            h197
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, h198, _, _, _, _) => {
            h198
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, h199, _, _, _) => {
            h199
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, h200, _, _) => {
            h200
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, h201, _) => {
            h201
        }
    }, match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, _, h202) => {
            h202
        }
    }, v1, 0u64))?;
    Ok(Adt6::C0(f55(match v0.clone() {
        Adt6::C0(h203, _, _, _, _, _, _, _) => {
            h203
        }
    }, match v2.clone() {
        Adt8::C0(h204, _, _, _, _, _, _, _, _, _) => {
            h204
        }
    }), f55(match v0.clone() {
        Adt6::C0(_, h205, _, _, _, _, _, _) => {
            h205
        }
    }, match v2.clone() {
        Adt8::C0(_, h206, _, _, _, _, _, _, _, _) => {
            h206
        }
    }), f55(match v0.clone() {
        Adt6::C0(_, _, h207, _, _, _, _, _) => {
            h207
        }
    }, match v2.clone() {
        Adt8::C0(_, _, h208, _, _, _, _, _, _, _) => {
            h208
        }
    }), f55(match v0.clone() {
        Adt6::C0(_, _, _, h209, _, _, _, _) => {
            h209
        }
    }, match v2.clone() {
        Adt8::C0(_, _, _, h210, _, _, _, _, _, _) => {
            h210
        }
    }), f55(match v0.clone() {
        Adt6::C0(_, _, _, _, h211, _, _, _) => {
            h211
        }
    }, match v2.clone() {
        Adt8::C0(_, _, _, _, h212, _, _, _, _, _) => {
            h212
        }
    }), f55(match v0.clone() {
        Adt6::C0(_, _, _, _, _, h213, _, _) => {
            h213
        }
    }, match v2.clone() {
        Adt8::C0(_, _, _, _, _, h214, _, _, _, _) => {
            h214
        }
    }), f55(match v0.clone() {
        Adt6::C0(_, _, _, _, _, _, h215, _) => {
            h215
        }
    }, match v2.clone() {
        Adt8::C0(_, _, _, _, _, _, h216, _, _, _) => {
            h216
        }
    }), f55(match v0 {
        Adt6::C0(_, _, _, _, _, _, _, h217) => {
            h217
        }
    }, match v2 {
        Adt8::C0(_, _, _, _, _, _, _, h218, _, _) => {
            h218
        }
    })))
}

pub fn f36(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt7> {
    Ok(Adt7::C0(f56(v0.clone(), v1, v2, v3)?, f56(v0.clone(), v1, v2, {
        let a219 = v3;
        let a220 = 4u64;
        nat_add(a219, a220)?
    })?, f56(v0.clone(), v1, v2, {
        let a221 = v3;
        let a222 = 8u64;
        nat_add(a221, a222)?
    })?, f56(v0.clone(), v1, v2, {
        let a223 = v3;
        let a224 = 12u64;
        nat_add(a223, a224)?
    })?, f56(v0.clone(), v1, v2, {
        let a225 = v3;
        let a226 = 16u64;
        nat_add(a225, a226)?
    })?, f56(v0.clone(), v1, v2, {
        let a227 = v3;
        let a228 = 20u64;
        nat_add(a227, a228)?
    })?, f56(v0.clone(), v1, v2, {
        let a229 = v3;
        let a230 = 24u64;
        nat_add(a229, a230)?
    })?, f56(v0.clone(), v1, v2, {
        let a231 = v3;
        let a232 = 28u64;
        nat_add(a231, a232)?
    })?, f56(v0.clone(), v1, v2, {
        let a233 = v3;
        let a234 = 32u64;
        nat_add(a233, a234)?
    })?, f56(v0.clone(), v1, v2, {
        let a235 = v3;
        let a236 = 36u64;
        nat_add(a235, a236)?
    })?, f56(v0.clone(), v1, v2, {
        let a237 = v3;
        let a238 = 40u64;
        nat_add(a237, a238)?
    })?, f56(v0.clone(), v1, v2, {
        let a239 = v3;
        let a240 = 44u64;
        nat_add(a239, a240)?
    })?, f56(v0.clone(), v1, v2, {
        let a241 = v3;
        let a242 = 48u64;
        nat_add(a241, a242)?
    })?, f56(v0.clone(), v1, v2, {
        let a243 = v3;
        let a244 = 52u64;
        nat_add(a243, a244)?
    })?, f56(v0.clone(), v1, v2, {
        let a245 = v3;
        let a246 = 56u64;
        nat_add(a245, a246)?
    })?, f56(v0, v1, v2, {
        let a247 = v3;
        let a248 = 60u64;
        nat_add(a247, a248)?
    })?))
}

pub fn f37(v0: Bytes) -> R<Bytes> {
    Ok(f38(f38(Bytes::lit(&[0x22]), {
        let (_, h249) = {
            let (h250, _) = f57(Fn8::F58(v0.clone()), {
                let a252 = {
                    let a251 = v0;
                    length_bytes(a251)
                };
                let a253 = 1u64;
                nat_add(a252, a253)?
            }, (0u64, Bytes::lit(&[])))?;
            h250
        };
        h249
    }), Bytes::lit(&[0x22])))
}

pub fn f38(v0: Bytes, v1: Bytes) -> Bytes {
    let a254 = v0;
    let a255 = v1;
    append_bytes(a254, a255)
}

pub fn f39(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m256 = f59(Fn9::F60(v1), None::<Bytes>, v0);
    match m256 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f40(v0: List<Bytes>) -> R<List<Bytes>> {
    f41(Fn4::F61, List::<Bytes>::nil(), v0)
}

pub fn f41(v0: Fn4, p260: List<Bytes>, p261: List<Bytes>) -> R<List<Bytes>> {
    let m259 = (p260, p261);
    {
        let mut m259 = m259;
        loop {
            let step = {
                let (v1, v2) = m259;
                let m257 = v2;
                match m257.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), R<List<Bytes>>>(({
                            let c258: Fn4 = v0.clone();
                            c258.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m259 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f42(v0: List<Bytes>, v1: Bytes) -> R<List<Bytes>> {
    Ok(List::cons(f37(v1)?, v0))
}

pub fn f43(v0: Bytes) -> Bytes {
    f38(f38(Bytes::lit(&[0x0a]), v0), Bytes::lit(&[0x0a]))
}

pub fn f44(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f22(v0, f37(f62(v1)?)?))
}

pub fn f45(v0: Adt1) -> R<Bytes> {
    Ok(f5(f27(Str::lit("[[lexicon_source]]")), {
        let m262 = v0;
        match m262 {
            Adt1::C0(v1) => {
                f5(f6(Str::lit("package"), v1)?, f6(Str::lit("kind"), f7(Str::lit("builtin")))?)
            }
            Adt1::C1(v2, v3) => {
                f5(f5(f6(Str::lit("package"), v2)?, f6(Str::lit("kind"), f7(Str::lit("path")))?), f6(Str::lit("path"), v3)?)
            }
            Adt1::C2(v4, v5, v6, v7) => {
                f5(f5(f5(f5(f6(Str::lit("package"), v4)?, f6(Str::lit("kind"), f7(Str::lit("git")))?), f6(Str::lit("url"), v5)?), f6(Str::lit("revision"), v6)?), f6(Str::lit("subdirectory"), v7)?)
            }
        }
    }))
}

pub fn f46(v0: Fn5, p266: List<Adt1>, p267: List<(Bytes, Adt1)>) -> List<Adt1> {
    let m265 = (p266, p267);
    {
        let mut m265 = m265;
        loop {
            let step = {
                let (v1, v2) = m265;
                let m263 = v2;
                match m263.uncons() {
                    None => {
                        Err::<(List<Adt1>, List<(Bytes, Adt1)>), List<Adt1>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt1>, List<(Bytes, Adt1)>), List<Adt1>>(({
                            let c264: Fn5 = v0.clone();
                            c264.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m265 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f47(v0: List<Adt1>, v1: (Bytes, Adt1)) -> List<Adt1> {
    List::cons({
        let (_, h268) = v1;
        h268
    }, v0)
}

pub fn f48(v0: List<(Bytes, Adt1)>) -> List<(Bytes, Adt1)> {
    f63(Fn10::F64, List::<(Bytes, Adt1)>::nil(), v0)
}

pub fn f49(v0: List<(Bytes, Adt1)>) -> R<List<(Bytes, Adt1)>> {
    let v1: List<List<(Bytes, Adt1)>> = f65(f66(Fn11::F67, List::<List<(Bytes, Adt1)>>::nil(), v0.clone()));
    let m273 = {
        let (h269, _) = f68(Fn12::F69, {
            let a271 = {
                let a270 = v0;
                length_list(a270)
            };
            let a272 = 1u64;
            nat_add(a271, a272)?
        }, v1)?;
        h269
    };
    match m273.uncons() {
        None => {
            Ok(List::<(Bytes, Adt1)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f50(v0: Fn6, p277: List<(Bytes, Adt1)>, p278: List<Adt1>) -> List<(Bytes, Adt1)> {
    let m276 = (p277, p278);
    {
        let mut m276 = m276;
        loop {
            let step = {
                let (v1, v2) = m276;
                let m274 = v2;
                match m274.uncons() {
                    None => {
                        Err::<(List<(Bytes, Adt1)>, List<Adt1>), List<(Bytes, Adt1)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Adt1)>, List<Adt1>), List<(Bytes, Adt1)>>(({
                            let c275: Fn6 = v0.clone();
                            c275.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m276 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f51(v0: List<(Bytes, Adt1)>, v1: Adt1) -> List<(Bytes, Adt1)> {
    List::cons((f70(v1.clone()), v1), v0)
}

pub fn f52(v0: u64) -> R<Bytes> {
    f71(f72(), 10u64, v0)
}

pub fn f53(v0: Fn7, p282: u64, p283: Adt8) -> R<Adt8> {
    let m281 = (p282, p283);
    {
        let mut m281 = m281;
        loop {
            let step = {
                let (v1, v2) = m281;
                let m279 = v1;
                if m279 == 0 {
                    Err::<(u64, Adt8), R<Adt8>>(Ok(v2))
                } else {
                    let v3 = m279 - 1;
                    Ok::<(u64, Adt8), R<Adt8>>((v3, {
                        let c280: Fn7 = v0.clone();
                        c280.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m281 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f54(v0: Adt8) -> R<Adt8> {
    f73(v0)
}

pub fn f55(v0: u32, v1: u32) -> u32 {
    let m286 = {
        let a284 = f74(v0);
        let a285 = f74(v1);
        fixed_u64::checked_add(a284, a285)
    };
    match m286 {
        None => {
            0u32
        }
        Some(v2) => {
            f75(v2)
        }
    }
}

pub fn f56(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u32> {
    let a289 = {
        let a287 = v3;
        let a288 = 8u64;
        nat_add(a287, a288)?
    };
    let a290 = v2;
    if nat_eq(a289, a290) {
        Ok(f76({
            let a293 = {
                let a291 = v1;
                let a292 = 8u64;
                nat_mul(a291, a292)?
            };
            let a294 = 4294967296u64;
            let a295 = 0u64;
            nat_quot(a293, a294, a295)
        }))
    } else {
        let a298 = {
            let a296 = v3;
            let a297 = 4u64;
            nat_add(a296, a297)?
        };
        let a299 = v2;
        if nat_eq(a298, a299) {
            Ok(f76({
                let a302 = {
                    let a300 = v1;
                    let a301 = 8u64;
                    nat_mul(a300, a301)?
                };
                let a303 = 4294967296u64;
                let a304 = 0u64;
                nat_rem(a302, a303, a304)
            }))
        } else {
            Ok(f77(f78(v0.clone(), v1, v3), f78(v0.clone(), v1, {
                let a305 = v3;
                let a306 = 1u64;
                nat_add(a305, a306)?
            }), f78(v0.clone(), v1, {
                let a307 = v3;
                let a308 = 2u64;
                nat_add(a307, a308)?
            }), f78(v0, v1, {
                let a309 = v3;
                let a310 = 3u64;
                nat_add(a309, a310)?
            })))
        }
    }
}

pub fn f57(v0: Fn8, p315: u64, p316: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m314 = (p315, p316);
    {
        let mut m314 = m314;
        loop {
            let step = {
                let (v1, v2) = m314;
                let m311 = v1;
                if m311 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m311 - 1;
                    let m313 = {
                        let c312: Fn8 = v0.clone();
                        c312.apply(v2.clone())?
                    };
                    match m313 {
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
                Ok(next) => { m314 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f58(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f79(v0, v1)
}

pub fn f59(v0: Fn9, p320: Option<Bytes>, p321: List<Bytes>) -> Option<Bytes> {
    let m319 = (p320, p321);
    {
        let mut m319 = m319;
        loop {
            let step = {
                let (v1, v2) = m319;
                let m317 = v2;
                match m317.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(({
                            let c318: Fn9 = v0.clone();
                            c318.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m319 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f60(v0: Bytes, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m322 = v1;
    match m322 {
        None => {
            Some(v2)
        }
        Some(v3) => {
            Some({
                let a325 = {
                    let a323 = v3;
                    let a324 = v0;
                    append_bytes(a323, a324)
                };
                let a326 = v2;
                append_bytes(a325, a326)
            })
        }
    }
}

pub fn f61(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f62(v0: Bytes) -> R<Bytes> {
    let (_, h327) = {
        let (h328, _) = f57(Fn8::F80(v0.clone()), {
            let a329 = v0;
            length_bytes(a329)
        }, (0u64, Bytes::lit(&[])))?;
        h328
    };
    Ok(h327)
}

pub fn f63(v0: Fn10, p333: List<(Bytes, Adt1)>, p334: List<(Bytes, Adt1)>) -> List<(Bytes, Adt1)> {
    let m332 = (p333, p334);
    {
        let mut m332 = m332;
        loop {
            let step = {
                let (v1, v2) = m332;
                let m330 = v2;
                match m330.uncons() {
                    None => {
                        Err::<(List<(Bytes, Adt1)>, List<(Bytes, Adt1)>), List<(Bytes, Adt1)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Adt1)>, List<(Bytes, Adt1)>), List<(Bytes, Adt1)>>(({
                            let c331: Fn10 = v0.clone();
                            c331.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m332 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f64(v0: List<(Bytes, Adt1)>, v1: (Bytes, Adt1)) -> List<(Bytes, Adt1)> {
    List::cons(v1, v0)
}

pub fn f65(v0: List<List<(Bytes, Adt1)>>) -> List<List<(Bytes, Adt1)>> {
    f81(Fn13::F82, List::<List<(Bytes, Adt1)>>::nil(), v0)
}

pub fn f66(v0: Fn11, p338: List<List<(Bytes, Adt1)>>, p339: List<(Bytes, Adt1)>) -> List<List<(Bytes, Adt1)>> {
    let m337 = (p338, p339);
    {
        let mut m337 = m337;
        loop {
            let step = {
                let (v1, v2) = m337;
                let m335 = v2;
                match m335.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Adt1)>>, List<(Bytes, Adt1)>), List<List<(Bytes, Adt1)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Adt1)>>, List<(Bytes, Adt1)>), List<List<(Bytes, Adt1)>>>(({
                            let c336: Fn11 = v0.clone();
                            c336.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m337 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f67(v0: List<List<(Bytes, Adt1)>>, v1: (Bytes, Adt1)) -> List<List<(Bytes, Adt1)>> {
    List::cons(List::cons(v1, List::<(Bytes, Adt1)>::nil()), v0)
}

pub fn f68(v0: Fn12, p344: u64, p345: List<List<(Bytes, Adt1)>>) -> R<(List<List<(Bytes, Adt1)>>, bool)> {
    let m343 = (p344, p345);
    {
        let mut m343 = m343;
        loop {
            let step = {
                let (v1, v2) = m343;
                let m340 = v1;
                if m340 == 0 {
                    Err::<(u64, List<List<(Bytes, Adt1)>>), R<(List<List<(Bytes, Adt1)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m340 - 1;
                    let m342 = {
                        let c341: Fn12 = v0.clone();
                        c341.apply(v2.clone())?
                    };
                    match m342 {
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
                Ok(next) => { m343 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f69(v0: List<List<(Bytes, Adt1)>>) -> R<Option<List<List<(Bytes, Adt1)>>>> {
    f83(v0)
}

pub fn f70(v0: Adt1) -> Bytes {
    let m346 = v0;
    match m346 {
        Adt1::C0(v1) => {
            v1
        }
        Adt1::C1(v2, _) => {
            v2
        }
        Adt1::C2(v4, _, _, _) => {
            v4
        }
    }
}

pub fn f71(v0: Bytes, v1: u64, v2: u64) -> R<Bytes> {
    let a347 = v2;
    let a348 = 0u64;
    if nat_eq(a347, a348) {
        Ok(f84(v0, 0u64))
    } else {
        let (_, h349) = {
            let (h350, _) = f57(Fn8::F85(v1, v0), v2, (v2, Bytes::lit(&[])))?;
            h350
        };
        Ok(h349)
    }
}

pub fn f72() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f73(v0: Adt8) -> R<Adt8> {
    let v1: u32 = f86(match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, h351) => {
            h351
        }
    });
    let v2: u32 = f55(match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, h352, _, _) => {
            h352
        }
    }, f55(f87(match v0.clone() {
        Adt8::C0(_, _, _, _, h353, _, _, _, _, _) => {
            h353
        }
    }), f55(f88(match v0.clone() {
        Adt8::C0(_, _, _, _, h354, _, _, _, _, _) => {
            h354
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, _, h355, _, _, _, _) => {
            h355
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, h356, _, _, _) => {
            h356
        }
    }), f55(v1, {
        let m359 = match v0.clone() {
            Adt8::C0(_, _, _, _, _, _, _, _, h358, _) => {
                h358
            }
        };
        match m359 {
            Adt7::C0(h357, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h357
            }
        }
    }))));
    let v3: u32 = f55(f89(match v0.clone() {
        Adt8::C0(h360, _, _, _, _, _, _, _, _, _) => {
            h360
        }
    }), f90(match v0.clone() {
        Adt8::C0(h361, _, _, _, _, _, _, _, _, _) => {
            h361
        }
    }, match v0.clone() {
        Adt8::C0(_, h362, _, _, _, _, _, _, _, _) => {
            h362
        }
    }, match v0.clone() {
        Adt8::C0(_, _, h363, _, _, _, _, _, _, _) => {
            h363
        }
    }));
    Ok(Adt8::C0(f55(v2, v3), match v0.clone() {
        Adt8::C0(h364, _, _, _, _, _, _, _, _, _) => {
            h364
        }
    }, match v0.clone() {
        Adt8::C0(_, h365, _, _, _, _, _, _, _, _) => {
            h365
        }
    }, match v0.clone() {
        Adt8::C0(_, _, h366, _, _, _, _, _, _, _) => {
            h366
        }
    }, f55(match v0.clone() {
        Adt8::C0(_, _, _, h367, _, _, _, _, _, _) => {
            h367
        }
    }, v2), match v0.clone() {
        Adt8::C0(_, _, _, _, h368, _, _, _, _, _) => {
            h368
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, _, h369, _, _, _, _) => {
            h369
        }
    }, match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, h370, _, _, _) => {
            h370
        }
    }, f91(match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, h371, _) => {
            h371
        }
    }), {
        let a373 = match v0 {
            Adt8::C0(_, _, _, _, _, _, _, _, _, h372) => {
                h372
            }
        };
        let a374 = 1u64;
        nat_add(a373, a374)?
    }))
}

pub fn f74(v0: u32) -> u64 {
    let m376 = {
        let a375 = v0;
        fixed_u64::convert(i128::from(a375))
    };
    match m376 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f75(v0: u64) -> u32 {
    let m380 = {
        let a379 = {
            let a377 = v0;
            let a378 = 4294967295u64;
            fixed_u64::bit_and(a377, a378)
        };
        fixed_u32::convert(i128::from(a379))
    };
    match m380 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f76(v0: u64) -> u32 {
    let (_, h381) = {
        let (_, h382) = f92(Fn14::F93, 32u64, (v0, (1u32, 0u32)));
        h382
    };
    h381
}

pub fn f77(v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
    let a387 = f94(v0, 24u32);
    let a388 = {
        let a385 = f94(v1, 16u32);
        let a386 = {
            let a383 = f94(v2, 8u32);
            let a384 = v3;
            fixed_u32::bit_or(a383, a384)
        };
        fixed_u32::bit_or(a385, a386)
    };
    fixed_u32::bit_or(a387, a388)
}

pub fn f78(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a389 = v2;
    let a390 = v1;
    if nat_lt(a389, a390) {
        let m393 = {
            let a391 = v0;
            let a392 = v2;
            index_bytes(a391, a392)
        };
        match m393 {
            None => {
                0u32
            }
            Some(v3) => {
                let m395 = {
                    let a394 = v3;
                    fixed_u32::convert(i128::from(a394))
                };
                match m395 {
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
        let a396 = v2;
        let a397 = v1;
        if nat_eq(a396, a397) {
            128u32
        } else {
            0u32
        }
    }
}

pub fn f79(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let m399 = f95(v0, {
        let (h398, _) = v1.clone();
        h398
    });
    match m399 {
        None => {
            Ok(None::<(u64, Bytes)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a401 = {
                    let (h400, _) = v1;
                    h400
                };
                let a402 = 1u64;
                nat_add(a401, a402)?
            }, f38({
                let (_, h403) = v1;
                h403
            }, f96({
                let v3: u8 = v2;
                let a448 = {
                    let a424 = {
                        let a412 = {
                            let a406 = {
                                let a404 = v3;
                                let a405 = 128u8;
                                fixed_u8::bit_and(a404, a405)
                            };
                            let a407 = 128u8;
                            if equal(a406, a407) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a413 = {
                            let a410 = {
                                let a408 = v3;
                                let a409 = 64u8;
                                fixed_u8::bit_and(a408, a409)
                            };
                            let a411 = 64u8;
                            if equal(a410, a411) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a412, a413)?
                    };
                    let a425 = {
                        let a422 = {
                            let a416 = {
                                let a414 = v3;
                                let a415 = 32u8;
                                fixed_u8::bit_and(a414, a415)
                            };
                            let a417 = 32u8;
                            if equal(a416, a417) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a423 = {
                            let a420 = {
                                let a418 = v3;
                                let a419 = 16u8;
                                fixed_u8::bit_and(a418, a419)
                            };
                            let a421 = 16u8;
                            if equal(a420, a421) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a422, a423)?
                    };
                    nat_add(a424, a425)?
                };
                let a449 = {
                    let a446 = {
                        let a434 = {
                            let a428 = {
                                let a426 = v3;
                                let a427 = 8u8;
                                fixed_u8::bit_and(a426, a427)
                            };
                            let a429 = 8u8;
                            if equal(a428, a429) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a435 = {
                            let a432 = {
                                let a430 = v3;
                                let a431 = 4u8;
                                fixed_u8::bit_and(a430, a431)
                            };
                            let a433 = 4u8;
                            if equal(a432, a433) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a434, a435)?
                    };
                    let a447 = {
                        let a444 = {
                            let a438 = {
                                let a436 = v3;
                                let a437 = 2u8;
                                fixed_u8::bit_and(a436, a437)
                            };
                            let a439 = 2u8;
                            if equal(a438, a439) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a445 = {
                            let a442 = {
                                let a440 = v3;
                                let a441 = 1u8;
                                fixed_u8::bit_and(a440, a441)
                            };
                            let a443 = 1u8;
                            if equal(a442, a443) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a444, a445)?
                    };
                    nat_add(a446, a447)?
                };
                nat_add(a448, a449)?
            })))))
        }
    }
}

pub fn f80(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f97(v0, v1)
}

pub fn f81(v0: Fn13, p453: List<List<(Bytes, Adt1)>>, p454: List<List<(Bytes, Adt1)>>) -> List<List<(Bytes, Adt1)>> {
    let m452 = (p453, p454);
    {
        let mut m452 = m452;
        loop {
            let step = {
                let (v1, v2) = m452;
                let m450 = v2;
                match m450.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>), List<List<(Bytes, Adt1)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>), List<List<(Bytes, Adt1)>>>(({
                            let c451: Fn13 = v0.clone();
                            c451.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m452 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f82(v0: List<List<(Bytes, Adt1)>>, v1: List<(Bytes, Adt1)>) -> List<List<(Bytes, Adt1)>> {
    List::cons(v1, v0)
}

pub fn f83(v0: List<List<(Bytes, Adt1)>>) -> R<Option<List<List<(Bytes, Adt1)>>>> {
    let a456 = {
        let a455 = v0.clone();
        length_list(a455)
    };
    let a457 = 1u64;
    if nat_le(a456, a457) {
        Ok(None::<List<List<(Bytes, Adt1)>>>)
    } else {
        Ok(Some(f98(v0)?))
    }
}

pub fn f84(v0: Bytes, v1: u64) -> Bytes {
    let m461 = {
        let a458 = v0;
        let a459 = v1;
        let a460 = 1u64;
        slice_bytes(a458, a459, a460)
    };
    match m461 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f85(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f99(v1, v0, v2)
}

pub fn f86(v0: u64) -> u32 {
    let a462 = v0;
    let a463 = 32u64;
    if nat_lt(a462, a463) {
        let a464 = v0;
        let a465 = 16u64;
        if nat_lt(a464, a465) {
            let a466 = v0;
            let a467 = 8u64;
            if nat_lt(a466, a467) {
                let a468 = v0;
                let a469 = 4u64;
                if nat_lt(a468, a469) {
                    let a470 = v0;
                    let a471 = 2u64;
                    if nat_lt(a470, a471) {
                        let a472 = v0;
                        let a473 = 1u64;
                        if nat_lt(a472, a473) {
                            1116352408u32
                        } else {
                            1899447441u32
                        }
                    } else {
                        let a474 = v0;
                        let a475 = 3u64;
                        if nat_lt(a474, a475) {
                            3049323471u32
                        } else {
                            3921009573u32
                        }
                    }
                } else {
                    let a476 = v0;
                    let a477 = 6u64;
                    if nat_lt(a476, a477) {
                        let a478 = v0;
                        let a479 = 5u64;
                        if nat_lt(a478, a479) {
                            961987163u32
                        } else {
                            1508970993u32
                        }
                    } else {
                        let a480 = v0;
                        let a481 = 7u64;
                        if nat_lt(a480, a481) {
                            2453635748u32
                        } else {
                            2870763221u32
                        }
                    }
                }
            } else {
                let a482 = v0;
                let a483 = 12u64;
                if nat_lt(a482, a483) {
                    let a484 = v0;
                    let a485 = 10u64;
                    if nat_lt(a484, a485) {
                        let a486 = v0;
                        let a487 = 9u64;
                        if nat_lt(a486, a487) {
                            3624381080u32
                        } else {
                            310598401u32
                        }
                    } else {
                        let a488 = v0;
                        let a489 = 11u64;
                        if nat_lt(a488, a489) {
                            607225278u32
                        } else {
                            1426881987u32
                        }
                    }
                } else {
                    let a490 = v0;
                    let a491 = 14u64;
                    if nat_lt(a490, a491) {
                        let a492 = v0;
                        let a493 = 13u64;
                        if nat_lt(a492, a493) {
                            1925078388u32
                        } else {
                            2162078206u32
                        }
                    } else {
                        let a494 = v0;
                        let a495 = 15u64;
                        if nat_lt(a494, a495) {
                            2614888103u32
                        } else {
                            3248222580u32
                        }
                    }
                }
            }
        } else {
            let a496 = v0;
            let a497 = 24u64;
            if nat_lt(a496, a497) {
                let a498 = v0;
                let a499 = 20u64;
                if nat_lt(a498, a499) {
                    let a500 = v0;
                    let a501 = 18u64;
                    if nat_lt(a500, a501) {
                        let a502 = v0;
                        let a503 = 17u64;
                        if nat_lt(a502, a503) {
                            3835390401u32
                        } else {
                            4022224774u32
                        }
                    } else {
                        let a504 = v0;
                        let a505 = 19u64;
                        if nat_lt(a504, a505) {
                            264347078u32
                        } else {
                            604807628u32
                        }
                    }
                } else {
                    let a506 = v0;
                    let a507 = 22u64;
                    if nat_lt(a506, a507) {
                        let a508 = v0;
                        let a509 = 21u64;
                        if nat_lt(a508, a509) {
                            770255983u32
                        } else {
                            1249150122u32
                        }
                    } else {
                        let a510 = v0;
                        let a511 = 23u64;
                        if nat_lt(a510, a511) {
                            1555081692u32
                        } else {
                            1996064986u32
                        }
                    }
                }
            } else {
                let a512 = v0;
                let a513 = 28u64;
                if nat_lt(a512, a513) {
                    let a514 = v0;
                    let a515 = 26u64;
                    if nat_lt(a514, a515) {
                        let a516 = v0;
                        let a517 = 25u64;
                        if nat_lt(a516, a517) {
                            2554220882u32
                        } else {
                            2821834349u32
                        }
                    } else {
                        let a518 = v0;
                        let a519 = 27u64;
                        if nat_lt(a518, a519) {
                            2952996808u32
                        } else {
                            3210313671u32
                        }
                    }
                } else {
                    let a520 = v0;
                    let a521 = 30u64;
                    if nat_lt(a520, a521) {
                        let a522 = v0;
                        let a523 = 29u64;
                        if nat_lt(a522, a523) {
                            3336571891u32
                        } else {
                            3584528711u32
                        }
                    } else {
                        let a524 = v0;
                        let a525 = 31u64;
                        if nat_lt(a524, a525) {
                            113926993u32
                        } else {
                            338241895u32
                        }
                    }
                }
            }
        }
    } else {
        let a526 = v0;
        let a527 = 48u64;
        if nat_lt(a526, a527) {
            let a528 = v0;
            let a529 = 40u64;
            if nat_lt(a528, a529) {
                let a530 = v0;
                let a531 = 36u64;
                if nat_lt(a530, a531) {
                    let a532 = v0;
                    let a533 = 34u64;
                    if nat_lt(a532, a533) {
                        let a534 = v0;
                        let a535 = 33u64;
                        if nat_lt(a534, a535) {
                            666307205u32
                        } else {
                            773529912u32
                        }
                    } else {
                        let a536 = v0;
                        let a537 = 35u64;
                        if nat_lt(a536, a537) {
                            1294757372u32
                        } else {
                            1396182291u32
                        }
                    }
                } else {
                    let a538 = v0;
                    let a539 = 38u64;
                    if nat_lt(a538, a539) {
                        let a540 = v0;
                        let a541 = 37u64;
                        if nat_lt(a540, a541) {
                            1695183700u32
                        } else {
                            1986661051u32
                        }
                    } else {
                        let a542 = v0;
                        let a543 = 39u64;
                        if nat_lt(a542, a543) {
                            2177026350u32
                        } else {
                            2456956037u32
                        }
                    }
                }
            } else {
                let a544 = v0;
                let a545 = 44u64;
                if nat_lt(a544, a545) {
                    let a546 = v0;
                    let a547 = 42u64;
                    if nat_lt(a546, a547) {
                        let a548 = v0;
                        let a549 = 41u64;
                        if nat_lt(a548, a549) {
                            2730485921u32
                        } else {
                            2820302411u32
                        }
                    } else {
                        let a550 = v0;
                        let a551 = 43u64;
                        if nat_lt(a550, a551) {
                            3259730800u32
                        } else {
                            3345764771u32
                        }
                    }
                } else {
                    let a552 = v0;
                    let a553 = 46u64;
                    if nat_lt(a552, a553) {
                        let a554 = v0;
                        let a555 = 45u64;
                        if nat_lt(a554, a555) {
                            3516065817u32
                        } else {
                            3600352804u32
                        }
                    } else {
                        let a556 = v0;
                        let a557 = 47u64;
                        if nat_lt(a556, a557) {
                            4094571909u32
                        } else {
                            275423344u32
                        }
                    }
                }
            }
        } else {
            let a558 = v0;
            let a559 = 56u64;
            if nat_lt(a558, a559) {
                let a560 = v0;
                let a561 = 52u64;
                if nat_lt(a560, a561) {
                    let a562 = v0;
                    let a563 = 50u64;
                    if nat_lt(a562, a563) {
                        let a564 = v0;
                        let a565 = 49u64;
                        if nat_lt(a564, a565) {
                            430227734u32
                        } else {
                            506948616u32
                        }
                    } else {
                        let a566 = v0;
                        let a567 = 51u64;
                        if nat_lt(a566, a567) {
                            659060556u32
                        } else {
                            883997877u32
                        }
                    }
                } else {
                    let a568 = v0;
                    let a569 = 54u64;
                    if nat_lt(a568, a569) {
                        let a570 = v0;
                        let a571 = 53u64;
                        if nat_lt(a570, a571) {
                            958139571u32
                        } else {
                            1322822218u32
                        }
                    } else {
                        let a572 = v0;
                        let a573 = 55u64;
                        if nat_lt(a572, a573) {
                            1537002063u32
                        } else {
                            1747873779u32
                        }
                    }
                }
            } else {
                let a574 = v0;
                let a575 = 60u64;
                if nat_lt(a574, a575) {
                    let a576 = v0;
                    let a577 = 58u64;
                    if nat_lt(a576, a577) {
                        let a578 = v0;
                        let a579 = 57u64;
                        if nat_lt(a578, a579) {
                            1955562222u32
                        } else {
                            2024104815u32
                        }
                    } else {
                        let a580 = v0;
                        let a581 = 59u64;
                        if nat_lt(a580, a581) {
                            2227730452u32
                        } else {
                            2361852424u32
                        }
                    }
                } else {
                    let a582 = v0;
                    let a583 = 62u64;
                    if nat_lt(a582, a583) {
                        let a584 = v0;
                        let a585 = 61u64;
                        if nat_lt(a584, a585) {
                            2428436474u32
                        } else {
                            2756734187u32
                        }
                    } else {
                        let a586 = v0;
                        let a587 = 63u64;
                        if nat_lt(a586, a587) {
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

pub fn f87(v0: u32) -> u32 {
    let a590 = {
        let a588 = f100(v0, 6u32, 26u32);
        let a589 = f100(v0, 11u32, 21u32);
        fixed_u32::bit_xor(a588, a589)
    };
    let a591 = f100(v0, 25u32, 7u32);
    fixed_u32::bit_xor(a590, a591)
}

pub fn f88(v0: u32, v1: u32, v2: u32) -> u32 {
    let a597 = {
        let a592 = v0;
        let a593 = v1;
        fixed_u32::bit_and(a592, a593)
    };
    let a598 = {
        let a595 = {
            let a594 = v0;
            fixed_u32::bit_not(a594)
        };
        let a596 = v2;
        fixed_u32::bit_and(a595, a596)
    };
    fixed_u32::bit_xor(a597, a598)
}

pub fn f89(v0: u32) -> u32 {
    let a601 = {
        let a599 = f100(v0, 2u32, 30u32);
        let a600 = f100(v0, 13u32, 19u32);
        fixed_u32::bit_xor(a599, a600)
    };
    let a602 = f100(v0, 22u32, 10u32);
    fixed_u32::bit_xor(a601, a602)
}

pub fn f90(v0: u32, v1: u32, v2: u32) -> u32 {
    let a611 = {
        let a607 = {
            let a603 = v0;
            let a604 = v1;
            fixed_u32::bit_and(a603, a604)
        };
        let a608 = {
            let a605 = v0;
            let a606 = v2;
            fixed_u32::bit_and(a605, a606)
        };
        fixed_u32::bit_xor(a607, a608)
    };
    let a612 = {
        let a609 = v1;
        let a610 = v2;
        fixed_u32::bit_and(a609, a610)
    };
    fixed_u32::bit_xor(a611, a612)
}

pub fn f91(v0: Adt7) -> Adt7 {
    Adt7::C0(match v0.clone() {
        Adt7::C0(_, h613, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h613
        }
    }, match v0.clone() {
        Adt7::C0(_, _, h614, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h614
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, h615, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h615
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, h616, _, _, _, _, _, _, _, _, _, _, _) => {
            h616
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, h617, _, _, _, _, _, _, _, _, _, _) => {
            h617
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, h618, _, _, _, _, _, _, _, _, _) => {
            h618
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, h619, _, _, _, _, _, _, _, _) => {
            h619
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, h620, _, _, _, _, _, _, _) => {
            h620
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, h621, _, _, _, _, _, _) => {
            h621
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, h622, _, _, _, _, _) => {
            h622
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, h623, _, _, _, _) => {
            h623
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, h624, _, _, _) => {
            h624
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h625, _, _) => {
            h625
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h626, _) => {
            h626
        }
    }, match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h627) => {
            h627
        }
    }, f55(f101(match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h628, _) => {
            h628
        }
    }), f55(match v0.clone() {
        Adt7::C0(_, _, _, _, _, _, _, _, _, h629, _, _, _, _, _, _) => {
            h629
        }
    }, f55(f102(match v0.clone() {
        Adt7::C0(_, h630, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h630
        }
    }), match v0 {
        Adt7::C0(h631, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h631
        }
    }))))
}

pub fn f92(v0: Fn14, p635: u64, p636: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m634 = (p635, p636);
    {
        let mut m634 = m634;
        loop {
            let step = {
                let (v1, v2) = m634;
                let m632 = v1;
                if m632 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m632 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c633: Fn14 = v0.clone();
                        c633.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m634 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f93(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h637, _) = v0;
        h637
    };
    let v2: u32 = {
        let (h638, _) = {
            let (_, h639) = v0;
            h639
        };
        h638
    };
    let v3: u32 = {
        let (_, h640) = {
            let (_, h641) = v0;
            h641
        };
        h640
    };
    ({
        let a642 = v1;
        let a643 = 2u64;
        let a644 = 0u64;
        nat_quot(a642, a643, a644)
    }, (f94(v2, 1u32), {
        let a648 = {
            let a645 = v1;
            let a646 = 2u64;
            let a647 = 0u64;
            nat_rem(a645, a646, a647)
        };
        let a649 = 1u64;
        if nat_eq(a648, a649) {
            let a650 = v3;
            let a651 = v2;
            fixed_u32::bit_or(a650, a651)
        } else {
            v3
        }
    }))
}

pub fn f94(v0: u32, v1: u32) -> u32 {
    let m654 = {
        let a652 = v0;
        let a653 = v1;
        fixed_u32::shift_left(a652, a653)
    };
    match m654 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f95(v0: Bytes, v1: u64) -> Option<u8> {
    let a655 = v0;
    let a656 = v1;
    index_bytes(a655, a656)
}

pub fn f96(v0: u64) -> Bytes {
    let a657 = v0;
    let a658 = 34u64;
    if nat_eq(a657, a658) {
        Bytes::lit(&[0x5c, 0x22])
    } else {
        let a659 = v0;
        let a660 = 92u64;
        if nat_eq(a659, a660) {
            Bytes::lit(&[0x5c, 0x5c])
        } else {
            let a661 = v0;
            let a662 = 32u64;
            if if nat_lt(a661, a662) {
                true
            } else {
                let a663 = v0;
                let a664 = 127u64;
                nat_eq(a663, a664)
            } {
                f38(f38(Bytes::lit(&[0x5c, 0x75, 0x30, 0x30]), f103({
                    let a665 = v0;
                    let a666 = 16u64;
                    let a667 = 0u64;
                    nat_quot(a665, a666, a667)
                })), f103({
                    let a668 = v0;
                    let a669 = 16u64;
                    let a670 = 0u64;
                    nat_rem(a668, a669, a670)
                }))
            } else {
                f84(f104(), v0)
            }
        }
    }
}

pub fn f97(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let m674 = {
        let a672 = v0;
        let a673 = {
            let (h671, _) = v1.clone();
            h671
        };
        index_bytes(a672, a673)
    };
    match m674 {
        None => {
            Ok(None::<(u64, Bytes)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a676 = {
                    let (h675, _) = v1;
                    h675
                };
                let a677 = 1u64;
                nat_add(a676, a677)?
            }, {
                let a679 = {
                    let (_, h678) = v1;
                    h678
                };
                let a680 = f105(v2)?;
                append_bytes(a679, a680)
            })))
        }
    }
}

pub fn f98(v0: List<List<(Bytes, Adt1)>>) -> R<List<List<(Bytes, Adt1)>>> {
    Ok(f65({
        let (_, h681) = {
            let (h682, _) = f106(Fn15::F107, {
                let a684 = {
                    let a683 = v0.clone();
                    length_list(a683)
                };
                let a685 = 1u64;
                nat_add(a684, a685)?
            }, (v0, List::<List<(Bytes, Adt1)>>::nil()))?;
            h682
        };
        h681
    }))
}

pub fn f99(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h686, _) = v2;
        h686
    };
    let a687 = v3;
    let a688 = 0u64;
    if nat_eq(a687, a688) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a689 = v3;
            let a690 = v1;
            let a691 = 0u64;
            nat_quot(a689, a690, a691)
        }, {
            let a696 = f84(v0, {
                let a692 = v3;
                let a693 = v1;
                let a694 = 0u64;
                nat_rem(a692, a693, a694)
            });
            let a697 = {
                let (_, h695) = v2;
                h695
            };
            append_bytes(a696, a697)
        }))
    }
}

pub fn f100(v0: u32, v1: u32, v2: u32) -> u32 {
    let a698 = f34(v0, v1);
    let a699 = f94(v0, v2);
    fixed_u32::bit_or(a698, a699)
}

pub fn f101(v0: u32) -> u32 {
    let a702 = {
        let a700 = f100(v0, 17u32, 15u32);
        let a701 = f100(v0, 19u32, 13u32);
        fixed_u32::bit_xor(a700, a701)
    };
    let a703 = f34(v0, 10u32);
    fixed_u32::bit_xor(a702, a703)
}

pub fn f102(v0: u32) -> u32 {
    let a706 = {
        let a704 = f100(v0, 7u32, 25u32);
        let a705 = f100(v0, 18u32, 14u32);
        fixed_u32::bit_xor(a704, a705)
    };
    let a707 = f34(v0, 3u32);
    fixed_u32::bit_xor(a706, a707)
}

pub fn f103(v0: u64) -> Bytes {
    f84(Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46]), v0)
}

pub fn f104() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f105(v0: u8) -> R<Bytes> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a752 = {
            let a728 = {
                let a716 = {
                    let a710 = {
                        let a708 = v1;
                        let a709 = 128u8;
                        fixed_u8::bit_and(a708, a709)
                    };
                    let a711 = 128u8;
                    if equal(a710, a711) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a717 = {
                    let a714 = {
                        let a712 = v1;
                        let a713 = 64u8;
                        fixed_u8::bit_and(a712, a713)
                    };
                    let a715 = 64u8;
                    if equal(a714, a715) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a716, a717)?
            };
            let a729 = {
                let a726 = {
                    let a720 = {
                        let a718 = v1;
                        let a719 = 32u8;
                        fixed_u8::bit_and(a718, a719)
                    };
                    let a721 = 32u8;
                    if equal(a720, a721) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a727 = {
                    let a724 = {
                        let a722 = v1;
                        let a723 = 16u8;
                        fixed_u8::bit_and(a722, a723)
                    };
                    let a725 = 16u8;
                    if equal(a724, a725) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a726, a727)?
            };
            nat_add(a728, a729)?
        };
        let a753 = {
            let a750 = {
                let a738 = {
                    let a732 = {
                        let a730 = v1;
                        let a731 = 8u8;
                        fixed_u8::bit_and(a730, a731)
                    };
                    let a733 = 8u8;
                    if equal(a732, a733) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a739 = {
                    let a736 = {
                        let a734 = v1;
                        let a735 = 4u8;
                        fixed_u8::bit_and(a734, a735)
                    };
                    let a737 = 4u8;
                    if equal(a736, a737) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a738, a739)?
            };
            let a751 = {
                let a748 = {
                    let a742 = {
                        let a740 = v1;
                        let a741 = 2u8;
                        fixed_u8::bit_and(a740, a741)
                    };
                    let a743 = 2u8;
                    if equal(a742, a743) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a749 = {
                    let a746 = {
                        let a744 = v1;
                        let a745 = 1u8;
                        fixed_u8::bit_and(a744, a745)
                    };
                    let a747 = 1u8;
                    if equal(a746, a747) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a748, a749)?
            };
            nat_add(a750, a751)?
        };
        nat_add(a752, a753)?
    };
    let a760 = f84(f108(), {
        let a754 = v2;
        let a755 = 16u64;
        let a756 = 0u64;
        nat_quot(a754, a755, a756)
    });
    let a761 = f84(f108(), {
        let a757 = v2;
        let a758 = 16u64;
        let a759 = 0u64;
        nat_rem(a757, a758, a759)
    });
    Ok(append_bytes(a760, a761))
}

pub fn f106(v0: Fn15, p766: u64, p767: (List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>)) -> R<((List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>), bool)> {
    let m765 = (p766, p767);
    {
        let mut m765 = m765;
        loop {
            let step = {
                let (v1, v2) = m765;
                let m762 = v1;
                if m762 == 0 {
                    Err::<(u64, (List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>)), R<((List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m762 - 1;
                    let m764 = {
                        let c763: Fn15 = v0.clone();
                        c763.apply(v2.clone())?
                    };
                    match m764 {
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
                Ok(next) => { m765 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f107(v0: (List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>)) -> R<Option<(List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>)>> {
    f109(v0)
}

pub fn f108() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66])
}

pub fn f109(v0: (List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>)) -> R<Option<(List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>)>> {
    let v1: List<List<(Bytes, Adt1)>> = {
        let (h768, _) = v0;
        h768
    };
    let v2: List<List<(Bytes, Adt1)>> = {
        let (_, h769) = v0;
        h769
    };
    let m770 = v1;
    match m770.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>)>)
        }
        Some((v3, v4)) => {
            let m771 = v4;
            match m771.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Adt1)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f110(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f110(v0: List<(Bytes, Adt1)>, v1: List<(Bytes, Adt1)>) -> R<List<(Bytes, Adt1)>> {
    let v2: u64 = {
        let a776 = {
            let a774 = {
                let a772 = v0.clone();
                length_list(a772)
            };
            let a775 = {
                let a773 = v1.clone();
                length_list(a773)
            };
            nat_add(a774, a775)?
        };
        let a777 = 1u64;
        nat_add(a776, a777)?
    };
    Ok(f48({
        let (_, h778) = {
            let (_, h779) = {
                let (h780, _) = f111(Fn16::F112, v2, (v0, (v1, List::<(Bytes, Adt1)>::nil())));
                h780
            };
            h779
        };
        h778
    }))
}

pub fn f111(v0: Fn16, p785: u64, p786: (List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>))) -> ((List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>)), bool) {
    let m784 = (p785, p786);
    {
        let mut m784 = m784;
        loop {
            let step = {
                let (v1, v2) = m784;
                let m781 = v1;
                if m781 == 0 {
                    Err::<(u64, (List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>))), ((List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>)), bool)>((v2, false))
                } else {
                    let v3 = m781 - 1;
                    let m783 = {
                        let c782: Fn16 = v0.clone();
                        c782.apply(v2.clone())
                    };
                    match m783 {
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
                Ok(next) => { m784 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f112(v0: (List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>))) -> Option<(List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>))> {
    f113(v0)
}

pub fn f113(v0: (List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>))) -> Option<(List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>))> {
    let v1: List<(Bytes, Adt1)> = {
        let (h787, _) = v0.clone();
        h787
    };
    let v2: List<(Bytes, Adt1)> = {
        let (h788, _) = {
            let (_, h789) = v0.clone();
            h789
        };
        h788
    };
    let v3: List<(Bytes, Adt1)> = {
        let (_, h790) = {
            let (_, h791) = v0;
            h791
        };
        h790
    };
    let m792 = v1.clone();
    match m792.uncons() {
        None => {
            let m793 = v2;
            match m793.uncons() {
                None => {
                    None::<(List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Adt1)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m794 = v2.clone();
            match m794.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Adt1)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f114({
                        let (h795, _) = v8.clone();
                        h795
                    }, {
                        let (h796, _) = v6.clone();
                        h796
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

pub fn f114(v0: Bytes, v1: Bytes) -> bool {
    let a801 = {
        let a797 = v0;
        let a798 = v1;
        compare_bytes(a797, a798)
    };
    let a802 = {
        let a799 = Bytes::lit(&[]);
        let a800 = Bytes::lit(&[0x01]);
        compare_bytes(a799, a800)
    };
    equal(a801, a802)
}

pub fn run(p0: Adt0) -> R<Bytes> {
    f0(p0)
}
