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
    C1,
    C2,
    C3,
    C4(Adt1),
    C5,
    C6,
    C7,
    C8(std::rc::Rc<Adt0>),
    C9(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
    C10(std::rc::Rc<Adt0>),
    C11(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
    C12(u64),
    C13(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
    C14,
    C15(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
}

#[derive(Clone)]
pub enum Adt1 {
    C0,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
}

#[derive(Clone)]
pub enum Adt2 {
    C0(Adt0, Adt3),
    C1(u64),
    C2(u64, Adt0, std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C3(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C4(Adt0, std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C5(Adt5, Adt0, std::rc::Rc<Adt2>),
    C6(u64, std::rc::Rc<Adt2>),
    C7(u64, std::rc::Rc<Adt2>),
    C8(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C9(Adt6, std::rc::Rc<Adt2>),
    C10(std::rc::Rc<Adt2>),
    C11(std::rc::Rc<Adt2>),
    C12(std::rc::Rc<Adt2>, u64),
    C13,
    C14(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C15(Adt5, List<u64>, std::rc::Rc<Adt2>),
}

#[derive(Clone)]
pub enum Adt3 {
    C0,
    C1(bool),
    C2(Bytes),
    C3(Bytes),
    C4(Bytes),
    C5(Bytes),
    C6(Bytes),
    C7(Bytes),
    C8(Bytes),
    C9(Bytes),
    C10(Bytes),
    C11(Bytes),
    C12(Bytes),
    C13(Bytes),
    C14(Adt4),
    C15,
    C16(std::rc::Rc<Adt3>),
    C17(std::rc::Rc<Adt3>),
    C18(std::rc::Rc<Adt3>),
    C19(std::rc::Rc<Adt3>),
    C20(std::rc::Rc<Adt3>, std::rc::Rc<Adt3>),
    C21(u64, std::rc::Rc<Adt3>),
    C22(u64, std::rc::Rc<Adt3>),
    C23,
    C24(std::rc::Rc<Adt3>, std::rc::Rc<Adt3>),
}

#[derive(Clone)]
pub enum Adt4 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt5 {
    C0,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    C9,
    C10,
    C11,
    C12,
    C13,
    C14,
    C15(u64),
}

#[derive(Clone)]
pub enum Adt6 {
    C0,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    C9,
    C10,
    C11,
    C12,
    C13,
    C14,
    C15,
    C16,
    C17,
    C18,
    C19,
    C20,
    C21,
    C22,
    C23,
    C24,
    C25,
    C26,
    C27,
    C28,
    C29,
    C30,
    C31,
    C32,
    C33,
    C34,
    C35,
    C36,
    C37,
    C38,
    C39,
    C40(Adt1),
    C41(Adt0),
}

#[derive(Clone)]
pub enum Adt7 {
    C0(Bytes, Adt0, List<List<u64>>, Adt0, Adt0, Adt2),
}

#[derive(Clone)]
pub enum Adt8 {
    C0(Adt0, Adt9),
}

#[derive(Clone)]
pub enum Adt9 {
    C0,
    C1(Adt0, u64, Adt0, std::rc::Rc<Adt9>),
}

#[derive(Clone)]
pub enum Adt10 {
    C0,
    C1(Adt0, Adt0, std::rc::Rc<Adt10>),
}

#[derive(Clone)]
pub enum Adt11 {
    C0(List<bool>, Adt9),
}

#[derive(Clone)]
pub enum Fn0 {
    F10(Adt2, Adt10, Adt0, Adt0, Adt7, Adt0),
}

impl Fn0 {
    pub fn apply(&self, p0: Adt11) -> R<Option<Adt11>> {
        match self {
            Fn0::F10(k0, k1, k2, k3, k4, k5) => f10(k0.clone(), k1.clone(), k2.clone(), k3.clone(), k4.clone(), k5.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F21,
}

impl Fn1 {
    pub fn apply(&self, p0: Adt10) -> R<Option<Adt10>> {
        match self {
            Fn1::F21 => f21(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F51(Adt9, Adt10, Adt0, Adt0, Adt7),
}

impl Fn2 {
    pub fn apply(&self, p0: Adt0) -> R<Option<Adt0>> {
        match self {
            Fn2::F51(k0, k1, k2, k3, k4) => f51(k0.clone(), k1.clone(), k2.clone(), k3.clone(), k4.clone(), p0),
        }
    }
}

pub fn f0(v0: Bytes, v1: Adt0, v2: List<List<u64>>, v3: Adt0, v4: Adt0, v5: Adt2, v6: Adt0) -> R<List<bool>> {
    let v7: Adt7 = Adt7::C0(v0, v1.clone(), v2, v3.clone(), v4.clone(), v5.clone());
    let v8: u64 = f1(v5.clone())?;
    let v9: Adt0 = f2(f2(f2(Adt0::C14, v3), v4), v1.clone());
    let v10: Adt8 = f3(v7.clone(), Adt8::C0(v9, Adt9::C0), v5.clone(), 0u64, List::<bool>::nil(), v6.clone())?;
    let v11: Adt0 = match v10.clone() {
        Adt8::C0(h1, _) => {
            h1
        }
    };
    let v12: Adt0 = f4(f5(f6(v1)?), v11.clone());
    let v13: Adt10 = f7(v7.clone(), v11.clone(), match v10.clone() {
        Adt8::C0(_, h2) => {
            h2
        }
    }, v12.clone(), Adt10::C0);
    let v14: Adt10 = f8(v13, {
        let a5 = 1u64;
        let a6 = {
            let a3 = f6(v12.clone())?;
            let a4 = f6(v12.clone())?;
            nat_mul(a3, a4)?
        };
        nat_add(a5, a6)?
    })?;
    let v15: Adt11 = {
        let (h7, _) = f9(Fn0::F10(v5, v14, v11, v12, v7, v6), {
            let a8 = 1u64;
            let a9 = v8;
            nat_add(a8, a9)?
        }, Adt11::C0(f11(v8), match v10 {
            Adt8::C0(_, h10) => {
                h10
            }
        }))?;
        h7
    };
    match v15 {
        Adt11::C0(h11, _) => {
            Ok(h11)
        }
    }
}

pub fn f1(v0: Adt2) -> R<u64> {
    let m12 = v0;
    match m12 {
        Adt2::C0(_, _) => {
            Ok(0u64)
        }
        Adt2::C1(_) => {
            Ok(0u64)
        }
        Adt2::C2(_, _, _, _) => {
            Ok(0u64)
        }
        Adt2::C3(_, _, _) => {
            Ok(0u64)
        }
        Adt2::C4(_, _, _) => {
            Ok(0u64)
        }
        Adt2::C5(_, _, _) => {
            Ok(0u64)
        }
        Adt2::C6(_, _) => {
            Ok(0u64)
        }
        Adt2::C7(_, _) => {
            Ok(0u64)
        }
        Adt2::C8(_, _) => {
            Ok(0u64)
        }
        Adt2::C9(_, _) => {
            Ok(0u64)
        }
        Adt2::C10(_) => {
            Ok(0u64)
        }
        Adt2::C11(_) => {
            Ok(0u64)
        }
        Adt2::C12(_, _) => {
            Ok(0u64)
        }
        Adt2::C13 => {
            Ok(0u64)
        }
        Adt2::C14(_, r15) => {
            let v33 = (*r15).clone();
            let v34: u64 = f1(v33)?;
            let a13 = 1u64;
            let a14 = v34;
            nat_add(a13, a14)
        }
        Adt2::C15(_, _, _) => {
            Ok(0u64)
        }
    }
}

pub fn f2(v0: Adt0, v1: Adt0) -> Adt0 {
    let m16 = v1;
    match m16 {
        Adt0::C0 => {
            v0
        }
        Adt0::C1 => {
            v0
        }
        Adt0::C2 => {
            v0
        }
        Adt0::C3 => {
            v0
        }
        Adt0::C4(_) => {
            v0
        }
        Adt0::C5 => {
            v0
        }
        Adt0::C6 => {
            v0
        }
        Adt0::C7 => {
            v0
        }
        Adt0::C8(r17) => {
            let v3 = (*r17).clone();
            f2(v0, v3)
        }
        Adt0::C9(r18, r19) => {
            let v4 = (*r18).clone();
            let v5 = (*r19).clone();
            f2(f2(v0, v4), v5)
        }
        Adt0::C10(r20) => {
            let v6 = (*r20).clone();
            f2(v0, v6)
        }
        Adt0::C11(r21, r22) => {
            let v7 = (*r21).clone();
            let v8 = (*r22).clone();
            f2(f2(v0, v7), v8)
        }
        Adt0::C12(_) => {
            v0
        }
        Adt0::C13(r23, r24) => {
            let v10 = (*r23).clone();
            let v11 = (*r24).clone();
            let v12: Adt0 = f12(v0, Adt0::C13(std::rc::Rc::new(v10.clone()), std::rc::Rc::new(v11.clone())));
            f2(f2(v12, v10), v11)
        }
        Adt0::C14 => {
            v0
        }
        Adt0::C15(r25, r26) => {
            let v13 = (*r25).clone();
            let v14 = (*r26).clone();
            f2(f2(v0, v13), v14)
        }
    }
}

pub fn f3(v0: Adt7, p37: Adt8, p38: Adt2, p39: u64, v4: List<bool>, p40: Adt0) -> R<Adt8> {
    let m36 = (p37, (p38, (p39, p40)));
    {
        let mut m36 = m36;
        loop {
            let step = {
                let (v1, (v2, (v3, v5))) = m36;
                let m27 = v2;
                match m27 {
                    Adt2::C0(_, _) => {
                        Err::<(Adt8, (Adt2, (u64, Adt0))), R<Adt8>>(Ok(v1))
                    }
                    Adt2::C1(_) => {
                        Err::<(Adt8, (Adt2, (u64, Adt0))), R<Adt8>>(Ok(v1))
                    }
                    Adt2::C2(_, _, _, _) => {
                        Err::<(Adt8, (Adt2, (u64, Adt0))), R<Adt8>>(Ok(v1))
                    }
                    Adt2::C3(_, _, _) => {
                        Err::<(Adt8, (Adt2, (u64, Adt0))), R<Adt8>>(Ok(v1))
                    }
                    Adt2::C4(_, _, _) => {
                        Err::<(Adt8, (Adt2, (u64, Adt0))), R<Adt8>>(Ok(v1))
                    }
                    Adt2::C5(_, _, _) => {
                        Err::<(Adt8, (Adt2, (u64, Adt0))), R<Adt8>>(Ok(v1))
                    }
                    Adt2::C6(_, _) => {
                        Err::<(Adt8, (Adt2, (u64, Adt0))), R<Adt8>>(Ok(v1))
                    }
                    Adt2::C7(_, _) => {
                        Err::<(Adt8, (Adt2, (u64, Adt0))), R<Adt8>>(Ok(v1))
                    }
                    Adt2::C8(_, _) => {
                        Err::<(Adt8, (Adt2, (u64, Adt0))), R<Adt8>>(Ok(v1))
                    }
                    Adt2::C9(_, _) => {
                        Err::<(Adt8, (Adt2, (u64, Adt0))), R<Adt8>>(Ok(v1))
                    }
                    Adt2::C10(_) => {
                        Err::<(Adt8, (Adt2, (u64, Adt0))), R<Adt8>>(Ok(v1))
                    }
                    Adt2::C11(_) => {
                        Err::<(Adt8, (Adt2, (u64, Adt0))), R<Adt8>>(Ok(v1))
                    }
                    Adt2::C12(_, _) => {
                        Err::<(Adt8, (Adt2, (u64, Adt0))), R<Adt8>>(Ok(v1))
                    }
                    Adt2::C13 => {
                        Err::<(Adt8, (Adt2, (u64, Adt0))), R<Adt8>>(Ok(v1))
                    }
                    Adt2::C14(r34, r35) => {
                        let v34 = (*r34).clone();
                        let v35 = (*r35).clone();
                        let v37: bool = {
                            let m28 = f13(v4.clone(), v3);
                            match m28 {
                                None => {
                                    true
                                }
                                Some(v36) => {
                                    let a29 = v36;
                                    bool_not(a29)
                                }
                            }
                        };
                        let v38: Adt8 = Adt8::C0(f14(v0.clone(), match v1.clone() {
                            Adt8::C0(h30, _) => {
                                h30
                            }
                        }, v3), match v1 {
                            Adt8::C0(_, h31) => {
                                h31
                            }
                        });
                        let v39: Adt8 = if v37 {
                            f15(v0.clone(), v38, v34, f16(v5.clone()))?
                        } else {
                            v38
                        };
                        Ok::<(Adt8, (Adt2, (u64, Adt0))), R<Adt8>>((v39, (v35, ({
                            let a32 = v3;
                            let a33 = 1u64;
                            nat_add(a32, a33)?
                        }, f17(v5)))))
                    }
                    Adt2::C15(_, _, _) => {
                        Err::<(Adt8, (Adt2, (u64, Adt0))), R<Adt8>>(Ok(v1))
                    }
                }
            };
            match step {
                Ok(next) => { m36 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f4(v0: Adt0, v1: Adt0) -> Adt0 {
    let m41 = v0;
    match m41 {
        Adt0::C0 => {
            v1
        }
        Adt0::C1 => {
            v1
        }
        Adt0::C2 => {
            v1
        }
        Adt0::C3 => {
            v1
        }
        Adt0::C4(_) => {
            v1
        }
        Adt0::C5 => {
            v1
        }
        Adt0::C6 => {
            v1
        }
        Adt0::C7 => {
            v1
        }
        Adt0::C8(_) => {
            v1
        }
        Adt0::C9(_, _) => {
            v1
        }
        Adt0::C10(_) => {
            v1
        }
        Adt0::C11(_, _) => {
            v1
        }
        Adt0::C12(_) => {
            v1
        }
        Adt0::C13(_, _) => {
            v1
        }
        Adt0::C14 => {
            v1
        }
        Adt0::C15(r42, r43) => {
            let v12 = (*r42).clone();
            let v13 = (*r43).clone();
            Adt0::C15(std::rc::Rc::new(v12), std::rc::Rc::new(f4(v13, v1)))
        }
    }
}

pub fn f5(v0: u64) -> Adt0 {
    let m44 = v0;
    if m44 == 0 {
        Adt0::C14
    } else {
        let v1 = m44 - 1;
        Adt0::C15(std::rc::Rc::new(Adt0::C12(v1)), std::rc::Rc::new(f5(v1)))
    }
}

pub fn f6(v0: Adt0) -> R<u64> {
    let m45 = v0;
    match m45 {
        Adt0::C0 => {
            Ok(0u64)
        }
        Adt0::C1 => {
            Ok(0u64)
        }
        Adt0::C2 => {
            Ok(0u64)
        }
        Adt0::C3 => {
            Ok(0u64)
        }
        Adt0::C4(_) => {
            Ok(0u64)
        }
        Adt0::C5 => {
            Ok(0u64)
        }
        Adt0::C6 => {
            Ok(0u64)
        }
        Adt0::C7 => {
            Ok(0u64)
        }
        Adt0::C8(_) => {
            Ok(0u64)
        }
        Adt0::C9(_, _) => {
            Ok(0u64)
        }
        Adt0::C10(_) => {
            Ok(0u64)
        }
        Adt0::C11(_, _) => {
            Ok(0u64)
        }
        Adt0::C12(_) => {
            Ok(0u64)
        }
        Adt0::C13(_, _) => {
            Ok(0u64)
        }
        Adt0::C14 => {
            Ok(0u64)
        }
        Adt0::C15(_, r48) => {
            let v12 = (*r48).clone();
            let v13: u64 = f6(v12)?;
            let a46 = 1u64;
            let a47 = v13;
            nat_add(a46, a47)
        }
    }
}

pub fn f7(v0: Adt7, v1: Adt0, v2: Adt9, p53: Adt0, p54: Adt10) -> Adt10 {
    let m52 = (p53, p54);
    {
        let mut m52 = m52;
        loop {
            let step = {
                let (v3, v4) = m52;
                let m49 = v3;
                match m49 {
                    Adt0::C0 => {
                        Err::<(Adt0, Adt10), Adt10>(v4)
                    }
                    Adt0::C1 => {
                        Err::<(Adt0, Adt10), Adt10>(v4)
                    }
                    Adt0::C2 => {
                        Err::<(Adt0, Adt10), Adt10>(v4)
                    }
                    Adt0::C3 => {
                        Err::<(Adt0, Adt10), Adt10>(v4)
                    }
                    Adt0::C4(_) => {
                        Err::<(Adt0, Adt10), Adt10>(v4)
                    }
                    Adt0::C5 => {
                        Err::<(Adt0, Adt10), Adt10>(v4)
                    }
                    Adt0::C6 => {
                        Err::<(Adt0, Adt10), Adt10>(v4)
                    }
                    Adt0::C7 => {
                        Err::<(Adt0, Adt10), Adt10>(v4)
                    }
                    Adt0::C8(_) => {
                        Err::<(Adt0, Adt10), Adt10>(v4)
                    }
                    Adt0::C9(_, _) => {
                        Err::<(Adt0, Adt10), Adt10>(v4)
                    }
                    Adt0::C10(_) => {
                        Err::<(Adt0, Adt10), Adt10>(v4)
                    }
                    Adt0::C11(_, _) => {
                        Err::<(Adt0, Adt10), Adt10>(v4)
                    }
                    Adt0::C12(_) => {
                        Err::<(Adt0, Adt10), Adt10>(v4)
                    }
                    Adt0::C13(_, _) => {
                        Err::<(Adt0, Adt10), Adt10>(v4)
                    }
                    Adt0::C14 => {
                        Err::<(Adt0, Adt10), Adt10>(v4)
                    }
                    Adt0::C15(r50, r51) => {
                        let v15 = (*r50).clone();
                        let v16 = (*r51).clone();
                        Ok::<(Adt0, Adt10), Adt10>((v16, f18(v1.clone(), v15.clone(), f19(v0.clone(), v2.clone(), v15), v4)))
                    }
                }
            };
            match step {
                Ok(next) => { m52 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f8(v0: Adt10, v1: u64) -> R<Adt10> {
    let (h55, _) = f20(Fn1::F21, v1, v0)?;
    Ok(h55)
}

pub fn f9(v0: Fn0, p60: u64, p61: Adt11) -> R<(Adt11, bool)> {
    let m59 = (p60, p61);
    {
        let mut m59 = m59;
        loop {
            let step = {
                let (v1, v2) = m59;
                let m56 = v1;
                if m56 == 0 {
                    Err::<(u64, Adt11), R<(Adt11, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m56 - 1;
                    let m58 = {
                        let c57: Fn0 = v0.clone();
                        c57.apply(v2.clone())?
                    };
                    match m58 {
                        None => {
                            Err::<(u64, Adt11), R<(Adt11, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt11), R<(Adt11, bool)>>((v3, v4))
                        }
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

pub fn f10(v0: Adt2, v1: Adt10, v2: Adt0, v3: Adt0, v4: Adt7, v5: Adt0, v6: Adt11) -> R<Option<Adt11>> {
    f22(v4, v0, v2, v1, v3, v5, v6)
}

pub fn f11(v0: u64) -> List<bool> {
    let m62 = v0;
    if m62 == 0 {
        List::<bool>::nil()
    } else {
        let v1 = m62 - 1;
        List::cons(false, f11(v1))
    }
}

pub fn f12(v0: Adt0, v1: Adt0) -> Adt0 {
    if f23(v0.clone(), v1.clone()) {
        v0
    } else {
        f4(v0, Adt0::C15(std::rc::Rc::new(v1), std::rc::Rc::new(Adt0::C14)))
    }
}

pub fn f13(p66: List<bool>, p67: u64) -> Option<bool> {
    let m65 = (p66, p67);
    {
        let mut m65 = m65;
        loop {
            let step = {
                let (v0, v1) = m65;
                let m63 = v0;
                match m63.uncons() {
                    None => {
                        Err::<(List<bool>, u64), Option<bool>>(None::<bool>)
                    }
                    Some((v2, v3)) => {
                        let m64 = v1;
                        if m64 == 0 {
                            Err::<(List<bool>, u64), Option<bool>>(Some(v2))
                        } else {
                            let v4 = m64 - 1;
                            Ok::<(List<bool>, u64), Option<bool>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m65 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f14(v0: Adt7, v1: Adt0, v2: u64) -> Adt0 {
    f2(f24(v1, f25(v0.clone(), v2)), f26(v0, v2))
}

pub fn f15(v0: Adt7, v1: Adt8, v2: Adt2, v3: Adt0) -> R<Adt8> {
    let m68 = v2;
    match m68 {
        Adt2::C0(v4, _) => {
            Ok(Adt8::C0(f2(match v1.clone() {
                Adt8::C0(h69, _) => {
                    h69
                }
            }, v4), match v1 {
                Adt8::C0(_, h70) => {
                    h70
                }
            }))
        }
        Adt2::C1(_) => {
            Ok(v1)
        }
        Adt2::C2(_, v8, r79, r80) => {
            let v9 = (*r79).clone();
            let v10 = (*r80).clone();
            let v11: Adt8 = Adt8::C0(f2(match v1.clone() {
                Adt8::C0(h71, _) => {
                    h71
                }
            }, v8), match v1 {
                Adt8::C0(_, h72) => {
                    h72
                }
            });
            let v12: Adt8 = f15(v0.clone(), v11, v9, f16(v3.clone()))?;
            f15(v0, v12, v10, f17(v3))
        }
        Adt2::C3(r81, r82, r83) => {
            let v13 = (*r81).clone();
            let v14 = (*r82).clone();
            let v15 = (*r83).clone();
            let v16: Adt8 = f15(v0.clone(), v1, v13, f16(v3.clone()))?;
            let v17: Adt8 = f15(v0.clone(), v16, v14, f17(v3.clone()))?;
            f15(v0, v17, v15, f27(v3))
        }
        Adt2::C4(v18, r84, r85) => {
            let v19 = (*r84).clone();
            let v20 = (*r85).clone();
            let v21: Adt8 = Adt8::C0(f2(match v1.clone() {
                Adt8::C0(h73, _) => {
                    h73
                }
            }, v18), match v1 {
                Adt8::C0(_, h74) => {
                    h74
                }
            });
            let v22: Adt8 = f15(v0.clone(), v21, v19, f16(v3.clone()))?;
            f15(v0, v22, v20, f17(v3))
        }
        Adt2::C5(_, v24, r86) => {
            let v25 = (*r86).clone();
            let v26: Adt8 = Adt8::C0(f2(match v1.clone() {
                Adt8::C0(h75, _) => {
                    h75
                }
            }, v24), match v1 {
                Adt8::C0(_, h76) => {
                    h76
                }
            });
            f15(v0, v26, v25, f16(v3))
        }
        Adt2::C6(_, r87) => {
            let v28 = (*r87).clone();
            f15(v0, v1, v28, f16(v3))
        }
        Adt2::C7(v29, r88) => {
            let v30 = (*r88).clone();
            let v31: Adt0 = f25(v0.clone(), v29);
            let v32: u64 = f1(v30.clone())?;
            let v33: Adt0 = Adt0::C13(std::rc::Rc::new(f28(v31.clone(), v32)), std::rc::Rc::new(f26(v0.clone(), v29)));
            let v34: Adt8 = Adt8::C0(f2(match v1.clone() {
                Adt8::C0(h77, _) => {
                    h77
                }
            }, v33.clone()), f29(match v1 {
                Adt8::C0(_, h78) => {
                    h78
                }
            }, v33, v29, f30(v31, v32)));
            f15(v0, v34, v30, f16(v3))
        }
        Adt2::C8(r89, r90) => {
            let v35 = (*r89).clone();
            let v36 = (*r90).clone();
            let v37: Adt8 = f15(v0.clone(), v1, v35, f16(v3.clone()))?;
            f15(v0, v37, v36, f17(v3))
        }
        Adt2::C9(_, r91) => {
            let v39 = (*r91).clone();
            f15(v0, v1, v39, f16(v3))
        }
        Adt2::C10(r92) => {
            let v40 = (*r92).clone();
            f15(v0, v1, v40, f16(v3))
        }
        Adt2::C11(r93) => {
            let v41 = (*r93).clone();
            f15(v0, v1, v41, f16(v3))
        }
        Adt2::C12(r94, _) => {
            let v42 = (*r94).clone();
            f15(v0, v1, v42, f16(v3))
        }
        Adt2::C13 => {
            Ok(v1)
        }
        Adt2::C14(r95, r96) => {
            let v44 = (*r95).clone();
            let v45 = (*r96).clone();
            let v46: Adt8 = f15(v0.clone(), v1, v44, f16(v3.clone()))?;
            f15(v0, v46, v45, f17(v3))
        }
        Adt2::C15(_, _, r97) => {
            let v49 = (*r97).clone();
            f15(v0, v1, v49, f16(v3))
        }
    }
}

pub fn f16(v0: Adt0) -> Adt0 {
    f31(f32(v0))
}

pub fn f17(v0: Adt0) -> Adt0 {
    f31(f32(f32(v0)))
}

pub fn f18(v0: Adt0, v1: Adt0, p102: Adt0, p103: Adt10) -> Adt10 {
    let m101 = (p102, p103);
    {
        let mut m101 = m101;
        loop {
            let step = {
                let (v2, v3) = m101;
                let m98 = v2;
                match m98 {
                    Adt0::C0 => {
                        Err::<(Adt0, Adt10), Adt10>(v3)
                    }
                    Adt0::C1 => {
                        Err::<(Adt0, Adt10), Adt10>(v3)
                    }
                    Adt0::C2 => {
                        Err::<(Adt0, Adt10), Adt10>(v3)
                    }
                    Adt0::C3 => {
                        Err::<(Adt0, Adt10), Adt10>(v3)
                    }
                    Adt0::C4(_) => {
                        Err::<(Adt0, Adt10), Adt10>(v3)
                    }
                    Adt0::C5 => {
                        Err::<(Adt0, Adt10), Adt10>(v3)
                    }
                    Adt0::C6 => {
                        Err::<(Adt0, Adt10), Adt10>(v3)
                    }
                    Adt0::C7 => {
                        Err::<(Adt0, Adt10), Adt10>(v3)
                    }
                    Adt0::C8(_) => {
                        Err::<(Adt0, Adt10), Adt10>(v3)
                    }
                    Adt0::C9(_, _) => {
                        Err::<(Adt0, Adt10), Adt10>(v3)
                    }
                    Adt0::C10(_) => {
                        Err::<(Adt0, Adt10), Adt10>(v3)
                    }
                    Adt0::C11(_, _) => {
                        Err::<(Adt0, Adt10), Adt10>(v3)
                    }
                    Adt0::C12(_) => {
                        Err::<(Adt0, Adt10), Adt10>(v3)
                    }
                    Adt0::C13(_, _) => {
                        Err::<(Adt0, Adt10), Adt10>(v3)
                    }
                    Adt0::C14 => {
                        Err::<(Adt0, Adt10), Adt10>(v3)
                    }
                    Adt0::C15(r99, r100) => {
                        let v14 = (*r99).clone();
                        let v15 = (*r100).clone();
                        Ok::<(Adt0, Adt10), Adt10>((v15, f33(v0.clone(), v1.clone(), v14, v3)))
                    }
                }
            };
            match step {
                Ok(next) => { m101 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f19(v0: Adt7, v1: Adt9, v2: Adt0) -> Adt0 {
    let m104 = v2;
    match m104 {
        Adt0::C0 => {
            Adt0::C14
        }
        Adt0::C1 => {
            Adt0::C14
        }
        Adt0::C2 => {
            Adt0::C14
        }
        Adt0::C3 => {
            Adt0::C14
        }
        Adt0::C4(_) => {
            Adt0::C14
        }
        Adt0::C5 => {
            Adt0::C14
        }
        Adt0::C6 => {
            Adt0::C14
        }
        Adt0::C7 => {
            Adt0::C14
        }
        Adt0::C8(_) => {
            Adt0::C14
        }
        Adt0::C9(_, _) => {
            Adt0::C14
        }
        Adt0::C10(_) => {
            Adt0::C14
        }
        Adt0::C11(_, _) => {
            Adt0::C14
        }
        Adt0::C12(v10) => {
            let m105 = f34(v0, v10);
            match m105 {
                None => {
                    Adt0::C14
                }
                Some(v11) => {
                    v11
                }
            }
        }
        Adt0::C13(r106, r107) => {
            let v12 = (*r106).clone();
            let v13 = (*r107).clone();
            f35(v1, Adt0::C13(std::rc::Rc::new(v12), std::rc::Rc::new(v13)))
        }
        Adt0::C14 => {
            Adt0::C14
        }
        Adt0::C15(_, _) => {
            Adt0::C14
        }
    }
}

pub fn f20(v0: Fn1, p112: u64, p113: Adt10) -> R<(Adt10, bool)> {
    let m111 = (p112, p113);
    {
        let mut m111 = m111;
        loop {
            let step = {
                let (v1, v2) = m111;
                let m108 = v1;
                if m108 == 0 {
                    Err::<(u64, Adt10), R<(Adt10, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m108 - 1;
                    let m110 = {
                        let c109: Fn1 = v0.clone();
                        c109.apply(v2.clone())?
                    };
                    match m110 {
                        None => {
                            Err::<(u64, Adt10), R<(Adt10, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt10), R<(Adt10, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m111 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f21(v0: Adt10) -> R<Option<Adt10>> {
    f36(v0)
}

pub fn f22(v0: Adt7, v1: Adt2, v2: Adt0, v3: Adt10, v4: Adt0, v5: Adt0, v6: Adt11) -> R<Option<Adt11>> {
    let v7: Adt0 = f37(v0.clone(), v2.clone(), match v6.clone() {
        Adt11::C0(_, h114) => {
            h114
        }
    }, v3, v4)?;
    let v8: List<bool> = f38(match v0.clone() {
        Adt7::C0(_, _, _, h115, _, _) => {
            h115
        }
    }, v7);
    if f39(v8.clone(), match v6 {
        Adt11::C0(h116, _) => {
            h116
        }
    }) {
        Ok(None::<Adt11>)
    } else {
        Ok(Some(Adt11::C0(v8.clone(), {
            let m118 = f3(v0, Adt8::C0(v2, Adt9::C0), v1, 0u64, v8, v5)?;
            match m118 {
                Adt8::C0(_, h117) => {
                    h117
                }
            }
        })))
    }
}

pub fn f23(p123: Adt0, v1: Adt0) -> bool {
    let m122 = p123;
    {
        let mut m122 = m122;
        loop {
            let step = {
                let v0 = m122;
                let m119 = v0;
                match m119 {
                    Adt0::C0 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C1 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C2 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C3 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C4(_) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C5 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C6 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C7 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C8(_) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C9(_, _) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C10(_) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C11(_, _) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C12(_) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C13(_, _) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C14 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C15(r120, r121) => {
                        let v12 = (*r120).clone();
                        let v13 = (*r121).clone();
                        if f40(v12, v1.clone()) {
                            Err::<Adt0, bool>(true)
                        } else {
                            Ok::<Adt0, bool>(v13)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m122 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f24(p128: Adt0, p129: Adt0) -> Adt0 {
    let m127 = (p128, p129);
    {
        let mut m127 = m127;
        loop {
            let step = {
                let (v0, v1) = m127;
                let m124 = v1;
                match m124 {
                    Adt0::C0 => {
                        Err::<(Adt0, Adt0), Adt0>(v0)
                    }
                    Adt0::C1 => {
                        Err::<(Adt0, Adt0), Adt0>(v0)
                    }
                    Adt0::C2 => {
                        Err::<(Adt0, Adt0), Adt0>(v0)
                    }
                    Adt0::C3 => {
                        Err::<(Adt0, Adt0), Adt0>(v0)
                    }
                    Adt0::C4(_) => {
                        Err::<(Adt0, Adt0), Adt0>(v0)
                    }
                    Adt0::C5 => {
                        Err::<(Adt0, Adt0), Adt0>(v0)
                    }
                    Adt0::C6 => {
                        Err::<(Adt0, Adt0), Adt0>(v0)
                    }
                    Adt0::C7 => {
                        Err::<(Adt0, Adt0), Adt0>(v0)
                    }
                    Adt0::C8(_) => {
                        Err::<(Adt0, Adt0), Adt0>(v0)
                    }
                    Adt0::C9(_, _) => {
                        Err::<(Adt0, Adt0), Adt0>(v0)
                    }
                    Adt0::C10(_) => {
                        Err::<(Adt0, Adt0), Adt0>(v0)
                    }
                    Adt0::C11(_, _) => {
                        Err::<(Adt0, Adt0), Adt0>(v0)
                    }
                    Adt0::C12(_) => {
                        Err::<(Adt0, Adt0), Adt0>(v0)
                    }
                    Adt0::C13(_, _) => {
                        Err::<(Adt0, Adt0), Adt0>(v0)
                    }
                    Adt0::C14 => {
                        Err::<(Adt0, Adt0), Adt0>(v0)
                    }
                    Adt0::C15(r125, r126) => {
                        let v2 = (*r125).clone();
                        let v3 = (*r126).clone();
                        Ok::<(Adt0, Adt0), Adt0>((f2(v0, v2), v3))
                    }
                }
            };
            match step {
                Ok(next) => { m127 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f25(v0: Adt7, v1: u64) -> Adt0 {
    let m130 = f41(v0, v1);
    match m130 {
        None => {
            Adt0::C14
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f26(v0: Adt7, v1: u64) -> Adt0 {
    let m132 = f42(match v0 {
        Adt7::C0(_, _, _, _, h131, _) => {
            h131
        }
    }, v1);
    match m132 {
        None => {
            Adt0::C0
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f27(v0: Adt0) -> Adt0 {
    f31(f32(f32(f32(v0))))
}

pub fn f28(p138: Adt0, p139: u64) -> Adt0 {
    let m137 = (p138, p139);
    {
        let mut m137 = m137;
        loop {
            let step = {
                let (v0, v1) = m137;
                let m133 = v0;
                match m133 {
                    Adt0::C0 => {
                        Err::<(Adt0, u64), Adt0>(Adt0::C14)
                    }
                    Adt0::C1 => {
                        Err::<(Adt0, u64), Adt0>(Adt0::C14)
                    }
                    Adt0::C2 => {
                        Err::<(Adt0, u64), Adt0>(Adt0::C14)
                    }
                    Adt0::C3 => {
                        Err::<(Adt0, u64), Adt0>(Adt0::C14)
                    }
                    Adt0::C4(_) => {
                        Err::<(Adt0, u64), Adt0>(Adt0::C14)
                    }
                    Adt0::C5 => {
                        Err::<(Adt0, u64), Adt0>(Adt0::C14)
                    }
                    Adt0::C6 => {
                        Err::<(Adt0, u64), Adt0>(Adt0::C14)
                    }
                    Adt0::C7 => {
                        Err::<(Adt0, u64), Adt0>(Adt0::C14)
                    }
                    Adt0::C8(_) => {
                        Err::<(Adt0, u64), Adt0>(Adt0::C14)
                    }
                    Adt0::C9(_, _) => {
                        Err::<(Adt0, u64), Adt0>(Adt0::C14)
                    }
                    Adt0::C10(_) => {
                        Err::<(Adt0, u64), Adt0>(Adt0::C14)
                    }
                    Adt0::C11(_, _) => {
                        Err::<(Adt0, u64), Adt0>(Adt0::C14)
                    }
                    Adt0::C12(_) => {
                        Err::<(Adt0, u64), Adt0>(Adt0::C14)
                    }
                    Adt0::C13(_, _) => {
                        Err::<(Adt0, u64), Adt0>(Adt0::C14)
                    }
                    Adt0::C14 => {
                        Err::<(Adt0, u64), Adt0>(Adt0::C14)
                    }
                    Adt0::C15(r135, r136) => {
                        let v12 = (*r135).clone();
                        let v13 = (*r136).clone();
                        let m134 = v1;
                        if m134 == 0 {
                            Err::<(Adt0, u64), Adt0>(Adt0::C15(std::rc::Rc::new(v12), std::rc::Rc::new(v13)))
                        } else {
                            let v14 = m134 - 1;
                            Ok::<(Adt0, u64), Adt0>((v13, v14))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m137 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f29(v0: Adt9, v1: Adt0, v2: u64, v3: Adt0) -> Adt9 {
    if f43(v0.clone(), v1.clone(), v2) {
        v0
    } else {
        Adt9::C1(v1, v2, v3, std::rc::Rc::new(v0))
    }
}

pub fn f30(v0: Adt0, v1: u64) -> Adt0 {
    let m140 = v0;
    match m140 {
        Adt0::C0 => {
            Adt0::C14
        }
        Adt0::C1 => {
            Adt0::C14
        }
        Adt0::C2 => {
            Adt0::C14
        }
        Adt0::C3 => {
            Adt0::C14
        }
        Adt0::C4(_) => {
            Adt0::C14
        }
        Adt0::C5 => {
            Adt0::C14
        }
        Adt0::C6 => {
            Adt0::C14
        }
        Adt0::C7 => {
            Adt0::C14
        }
        Adt0::C8(_) => {
            Adt0::C14
        }
        Adt0::C9(_, _) => {
            Adt0::C14
        }
        Adt0::C10(_) => {
            Adt0::C14
        }
        Adt0::C11(_, _) => {
            Adt0::C14
        }
        Adt0::C12(_) => {
            Adt0::C14
        }
        Adt0::C13(_, _) => {
            Adt0::C14
        }
        Adt0::C14 => {
            Adt0::C14
        }
        Adt0::C15(r142, r143) => {
            let v12 = (*r142).clone();
            let v13 = (*r143).clone();
            let m141 = v1;
            if m141 == 0 {
                Adt0::C14
            } else {
                let v14 = m141 - 1;
                Adt0::C15(std::rc::Rc::new(v12), std::rc::Rc::new(f30(v13, v14)))
            }
        }
    }
}

pub fn f31(v0: Adt0) -> Adt0 {
    let m144 = f44(v0);
    match m144 {
        None => {
            Adt0::C0
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f32(v0: Adt0) -> Adt0 {
    let m145 = f45(v0);
    match m145 {
        None => {
            Adt0::C0
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f33(v0: Adt0, v1: Adt0, p150: Adt0, p151: Adt10) -> Adt10 {
    let m149 = (p150, p151);
    {
        let mut m149 = m149;
        loop {
            let step = {
                let (v2, v3) = m149;
                let m146 = v2;
                match m146 {
                    Adt0::C0 => {
                        Err::<(Adt0, Adt10), Adt10>(v3)
                    }
                    Adt0::C1 => {
                        Err::<(Adt0, Adt10), Adt10>(v3)
                    }
                    Adt0::C2 => {
                        Err::<(Adt0, Adt10), Adt10>(v3)
                    }
                    Adt0::C3 => {
                        Err::<(Adt0, Adt10), Adt10>(v3)
                    }
                    Adt0::C4(_) => {
                        Err::<(Adt0, Adt10), Adt10>(v3)
                    }
                    Adt0::C5 => {
                        Err::<(Adt0, Adt10), Adt10>(v3)
                    }
                    Adt0::C6 => {
                        Err::<(Adt0, Adt10), Adt10>(v3)
                    }
                    Adt0::C7 => {
                        Err::<(Adt0, Adt10), Adt10>(v3)
                    }
                    Adt0::C8(_) => {
                        Err::<(Adt0, Adt10), Adt10>(v3)
                    }
                    Adt0::C9(_, _) => {
                        Err::<(Adt0, Adt10), Adt10>(v3)
                    }
                    Adt0::C10(_) => {
                        Err::<(Adt0, Adt10), Adt10>(v3)
                    }
                    Adt0::C11(_, _) => {
                        Err::<(Adt0, Adt10), Adt10>(v3)
                    }
                    Adt0::C12(_) => {
                        Err::<(Adt0, Adt10), Adt10>(v3)
                    }
                    Adt0::C13(_, _) => {
                        Err::<(Adt0, Adt10), Adt10>(v3)
                    }
                    Adt0::C14 => {
                        Err::<(Adt0, Adt10), Adt10>(v3)
                    }
                    Adt0::C15(r147, r148) => {
                        let v14 = (*r147).clone();
                        let v15 = (*r148).clone();
                        Ok::<(Adt0, Adt10), Adt10>((v15, f46(v1.clone(), f47(v0.clone(), Adt0::C14, v14), v3)))
                    }
                }
            };
            match step {
                Ok(next) => { m149 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f34(v0: Adt7, v1: u64) -> Option<Adt0> {
    f42(match v0 {
        Adt7::C0(_, h152, _, _, _, _) => {
            h152
        }
    }, v1)
}

pub fn f35(v0: Adt9, v1: Adt0) -> Adt0 {
    let m153 = v0;
    match m153 {
        Adt9::C0 => {
            Adt0::C14
        }
        Adt9::C1(v2, _, v4, r154) => {
            let v5 = (*r154).clone();
            if f40(v2, v1.clone()) {
                Adt0::C15(std::rc::Rc::new(v4), std::rc::Rc::new(f35(v5, v1)))
            } else {
                f35(v5, v1)
            }
        }
    }
}

pub fn f36(v0: Adt10) -> R<Option<Adt10>> {
    let v1: Adt10 = f48(v0.clone(), v0.clone(), v0.clone());
    let a155 = f49(v1.clone())?;
    let a156 = f49(v0)?;
    if nat_eq(a155, a156) {
        Ok(None::<Adt10>)
    } else {
        Ok(Some(v1))
    }
}

pub fn f37(v0: Adt7, v1: Adt0, v2: Adt9, v3: Adt10, v4: Adt0) -> R<Adt0> {
    let (h157, _) = f50(Fn2::F51(v2, v3, v1, v4.clone(), v0), {
        let a158 = 1u64;
        let a159 = f6(v4)?;
        nat_add(a158, a159)?
    }, Adt0::C14)?;
    Ok(h157)
}

pub fn f38(v0: Adt0, v1: Adt0) -> List<bool> {
    let m160 = v0;
    match m160 {
        Adt0::C0 => {
            List::<bool>::nil()
        }
        Adt0::C1 => {
            List::<bool>::nil()
        }
        Adt0::C2 => {
            List::<bool>::nil()
        }
        Adt0::C3 => {
            List::<bool>::nil()
        }
        Adt0::C4(_) => {
            List::<bool>::nil()
        }
        Adt0::C5 => {
            List::<bool>::nil()
        }
        Adt0::C6 => {
            List::<bool>::nil()
        }
        Adt0::C7 => {
            List::<bool>::nil()
        }
        Adt0::C8(_) => {
            List::<bool>::nil()
        }
        Adt0::C9(_, _) => {
            List::<bool>::nil()
        }
        Adt0::C10(_) => {
            List::<bool>::nil()
        }
        Adt0::C11(_, _) => {
            List::<bool>::nil()
        }
        Adt0::C12(_) => {
            List::<bool>::nil()
        }
        Adt0::C13(_, _) => {
            List::<bool>::nil()
        }
        Adt0::C14 => {
            List::<bool>::nil()
        }
        Adt0::C15(r161, r162) => {
            let v12 = (*r161).clone();
            let v13 = (*r162).clone();
            List::cons(f52(v12, v1.clone()), f38(v13, v1))
        }
    }
}

pub fn f39(p169: List<bool>, p170: List<bool>) -> bool {
    let m168 = (p169, p170);
    {
        let mut m168 = m168;
        loop {
            let step = {
                let (v0, v1) = m168;
                let m163 = v0;
                match m163.uncons() {
                    None => {
                        let m164 = v1;
                        match m164.uncons() {
                            None => {
                                Err::<(List<bool>, List<bool>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<bool>, List<bool>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m165 = v1;
                        match m165.uncons() {
                            None => {
                                Err::<(List<bool>, List<bool>), bool>(false)
                            }
                            Some((v6, v7)) => {
                                let a166 = v4;
                                let a167 = v6;
                                if equal(a166, a167) {
                                    Ok::<(List<bool>, List<bool>), bool>((v5, v7))
                                } else {
                                    Err::<(List<bool>, List<bool>), bool>(false)
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m168 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f40(v0: Adt0, v1: Adt0) -> bool {
    let m171 = v0;
    match m171 {
        Adt0::C0 => {
            let a172 = f53(v1);
            let a173 = 0u64;
            nat_eq(a172, a173)
        }
        Adt0::C1 => {
            let a174 = f53(v1);
            let a175 = 1u64;
            nat_eq(a174, a175)
        }
        Adt0::C2 => {
            let a176 = f53(v1);
            let a177 = 2u64;
            nat_eq(a176, a177)
        }
        Adt0::C3 => {
            let a178 = f53(v1);
            let a179 = 3u64;
            nat_eq(a178, a179)
        }
        Adt0::C4(v2) => {
            let a188 = f53(v1.clone());
            let a189 = 4u64;
            if nat_eq(a188, a189) {
                let a190 = f54(v2);
                let a191 = f54(f55(v1));
                nat_eq(a190, a191)
            } else {
                false
            }
        }
        Adt0::C5 => {
            let a180 = f53(v1);
            let a181 = 5u64;
            nat_eq(a180, a181)
        }
        Adt0::C6 => {
            let a182 = f53(v1);
            let a183 = 6u64;
            nat_eq(a182, a183)
        }
        Adt0::C7 => {
            let a184 = f53(v1);
            let a185 = 7u64;
            nat_eq(a184, a185)
        }
        Adt0::C8(r208) => {
            let v3 = (*r208).clone();
            let v4: bool = f40(v3, f31(v1.clone()));
            let a192 = f53(v1);
            let a193 = 8u64;
            if nat_eq(a192, a193) {
                v4
            } else {
                false
            }
        }
        Adt0::C9(r209, r210) => {
            let v5 = (*r209).clone();
            let v6 = (*r210).clone();
            let v7: bool = f40(v5, f31(v1.clone()));
            let v8: bool = f40(v6, f32(v1.clone()));
            let a194 = f53(v1);
            let a195 = 9u64;
            if nat_eq(a194, a195) {
                if v7 {
                    v8
                } else {
                    false
                }
            } else {
                false
            }
        }
        Adt0::C10(r211) => {
            let v9 = (*r211).clone();
            let v10: bool = f40(v9, f31(v1.clone()));
            let a196 = f53(v1);
            let a197 = 10u64;
            if nat_eq(a196, a197) {
                v10
            } else {
                false
            }
        }
        Adt0::C11(r212, r213) => {
            let v11 = (*r212).clone();
            let v12 = (*r213).clone();
            let v13: bool = f40(v11, f31(v1.clone()));
            let v14: bool = f40(v12, f32(v1.clone()));
            let a198 = f53(v1);
            let a199 = 11u64;
            if nat_eq(a198, a199) {
                if v13 {
                    v14
                } else {
                    false
                }
            } else {
                false
            }
        }
        Adt0::C12(v15) => {
            let a200 = f53(v1.clone());
            let a201 = 12u64;
            if nat_eq(a200, a201) {
                let a202 = v15;
                let a203 = f56(v1);
                nat_eq(a202, a203)
            } else {
                false
            }
        }
        Adt0::C13(r214, r215) => {
            let v16 = (*r214).clone();
            let v17 = (*r215).clone();
            let v18: bool = f40(v16, f31(v1.clone()));
            let v19: bool = f40(v17, f32(v1.clone()));
            let a204 = f53(v1);
            let a205 = 13u64;
            if nat_eq(a204, a205) {
                if v18 {
                    v19
                } else {
                    false
                }
            } else {
                false
            }
        }
        Adt0::C14 => {
            let a186 = f53(v1);
            let a187 = 14u64;
            nat_eq(a186, a187)
        }
        Adt0::C15(r216, r217) => {
            let v20 = (*r216).clone();
            let v21 = (*r217).clone();
            let v22: bool = f40(v20, f31(v1.clone()));
            let v23: bool = f40(v21, f32(v1.clone()));
            let a206 = f53(v1);
            let a207 = 15u64;
            if nat_eq(a206, a207) {
                if v22 {
                    v23
                } else {
                    false
                }
            } else {
                false
            }
        }
    }
}

pub fn f41(v0: Adt7, v1: u64) -> Option<Adt0> {
    f42(match v0 {
        Adt7::C0(_, _, _, h218, _, _) => {
            h218
        }
    }, v1)
}

pub fn f42(p224: Adt0, p225: u64) -> Option<Adt0> {
    let m223 = (p224, p225);
    {
        let mut m223 = m223;
        loop {
            let step = {
                let (v0, v1) = m223;
                let m219 = v0;
                match m219 {
                    Adt0::C0 => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C1 => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C2 => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C3 => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C4(_) => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C5 => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C6 => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C7 => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C8(_) => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C9(_, _) => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C10(_) => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C11(_, _) => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C12(_) => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C13(_, _) => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C14 => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C15(r221, r222) => {
                        let v12 = (*r221).clone();
                        let v13 = (*r222).clone();
                        let m220 = v1;
                        if m220 == 0 {
                            Err::<(Adt0, u64), Option<Adt0>>(Some(v12))
                        } else {
                            let v14 = m220 - 1;
                            Ok::<(Adt0, u64), Option<Adt0>>((v13, v14))
                        }
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

pub fn f43(p231: Adt9, v1: Adt0, v2: u64) -> bool {
    let m230 = p231;
    {
        let mut m230 = m230;
        loop {
            let step = {
                let v0 = m230;
                let m226 = v0;
                match m226 {
                    Adt9::C0 => {
                        Err::<Adt9, bool>(false)
                    }
                    Adt9::C1(v3, v4, _, r229) => {
                        let v6 = (*r229).clone();
                        let a227 = v4;
                        let a228 = v2;
                        if nat_eq(a227, a228) {
                            if f40(v3, v1.clone()) {
                                Err::<Adt9, bool>(true)
                            } else {
                                Ok::<Adt9, bool>(v6)
                            }
                        } else {
                            Ok::<Adt9, bool>(v6)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m230 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f44(v0: Adt0) -> Option<Adt0> {
    let m232 = v0;
    match m232 {
        Adt0::C0 => {
            None::<Adt0>
        }
        Adt0::C1 => {
            None::<Adt0>
        }
        Adt0::C2 => {
            None::<Adt0>
        }
        Adt0::C3 => {
            None::<Adt0>
        }
        Adt0::C4(_) => {
            None::<Adt0>
        }
        Adt0::C5 => {
            None::<Adt0>
        }
        Adt0::C6 => {
            None::<Adt0>
        }
        Adt0::C7 => {
            None::<Adt0>
        }
        Adt0::C8(r233) => {
            let v2 = (*r233).clone();
            Some(v2)
        }
        Adt0::C9(r234, _) => {
            let v3 = (*r234).clone();
            Some(v3)
        }
        Adt0::C10(r235) => {
            let v5 = (*r235).clone();
            Some(v5)
        }
        Adt0::C11(r236, _) => {
            let v6 = (*r236).clone();
            Some(v6)
        }
        Adt0::C12(_) => {
            None::<Adt0>
        }
        Adt0::C13(r237, _) => {
            let v9 = (*r237).clone();
            Some(v9)
        }
        Adt0::C14 => {
            None::<Adt0>
        }
        Adt0::C15(r238, _) => {
            let v11 = (*r238).clone();
            Some(v11)
        }
    }
}

pub fn f45(v0: Adt0) -> Option<Adt0> {
    let m239 = v0;
    match m239 {
        Adt0::C0 => {
            None::<Adt0>
        }
        Adt0::C1 => {
            None::<Adt0>
        }
        Adt0::C2 => {
            None::<Adt0>
        }
        Adt0::C3 => {
            None::<Adt0>
        }
        Adt0::C4(_) => {
            None::<Adt0>
        }
        Adt0::C5 => {
            None::<Adt0>
        }
        Adt0::C6 => {
            None::<Adt0>
        }
        Adt0::C7 => {
            None::<Adt0>
        }
        Adt0::C8(_) => {
            None::<Adt0>
        }
        Adt0::C9(_, r240) => {
            let v4 = (*r240).clone();
            Some(v4)
        }
        Adt0::C10(_) => {
            None::<Adt0>
        }
        Adt0::C11(_, r241) => {
            let v7 = (*r241).clone();
            Some(v7)
        }
        Adt0::C12(_) => {
            None::<Adt0>
        }
        Adt0::C13(_, r242) => {
            let v10 = (*r242).clone();
            Some(v10)
        }
        Adt0::C14 => {
            None::<Adt0>
        }
        Adt0::C15(_, r243) => {
            let v12 = (*r243).clone();
            Some(v12)
        }
    }
}

pub fn f46(v0: Adt0, p248: Adt0, p249: Adt10) -> Adt10 {
    let m247 = (p248, p249);
    {
        let mut m247 = m247;
        loop {
            let step = {
                let (v1, v2) = m247;
                let m244 = v1;
                match m244 {
                    Adt0::C0 => {
                        Err::<(Adt0, Adt10), Adt10>(v2)
                    }
                    Adt0::C1 => {
                        Err::<(Adt0, Adt10), Adt10>(v2)
                    }
                    Adt0::C2 => {
                        Err::<(Adt0, Adt10), Adt10>(v2)
                    }
                    Adt0::C3 => {
                        Err::<(Adt0, Adt10), Adt10>(v2)
                    }
                    Adt0::C4(_) => {
                        Err::<(Adt0, Adt10), Adt10>(v2)
                    }
                    Adt0::C5 => {
                        Err::<(Adt0, Adt10), Adt10>(v2)
                    }
                    Adt0::C6 => {
                        Err::<(Adt0, Adt10), Adt10>(v2)
                    }
                    Adt0::C7 => {
                        Err::<(Adt0, Adt10), Adt10>(v2)
                    }
                    Adt0::C8(_) => {
                        Err::<(Adt0, Adt10), Adt10>(v2)
                    }
                    Adt0::C9(_, _) => {
                        Err::<(Adt0, Adt10), Adt10>(v2)
                    }
                    Adt0::C10(_) => {
                        Err::<(Adt0, Adt10), Adt10>(v2)
                    }
                    Adt0::C11(_, _) => {
                        Err::<(Adt0, Adt10), Adt10>(v2)
                    }
                    Adt0::C12(_) => {
                        Err::<(Adt0, Adt10), Adt10>(v2)
                    }
                    Adt0::C13(_, _) => {
                        Err::<(Adt0, Adt10), Adt10>(v2)
                    }
                    Adt0::C14 => {
                        Err::<(Adt0, Adt10), Adt10>(v2)
                    }
                    Adt0::C15(r245, r246) => {
                        let v13 = (*r245).clone();
                        let v14 = (*r246).clone();
                        Ok::<(Adt0, Adt10), Adt10>((v14, f57(v2, v0.clone(), v13)))
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

pub fn f47(v0: Adt0, v1: Adt0, v2: Adt0) -> Adt0 {
    let m250 = v2;
    match m250 {
        Adt0::C0 => {
            v1
        }
        Adt0::C1 => {
            v1
        }
        Adt0::C2 => {
            v1
        }
        Adt0::C3 => {
            v1
        }
        Adt0::C4(_) => {
            v1
        }
        Adt0::C5 => {
            v1
        }
        Adt0::C6 => {
            v1
        }
        Adt0::C7 => {
            v1
        }
        Adt0::C8(r251) => {
            let v4 = (*r251).clone();
            f47(v0, v1, v4)
        }
        Adt0::C9(r252, r253) => {
            let v5 = (*r252).clone();
            let v6 = (*r253).clone();
            f47(v0.clone(), f47(v0, v1, v5), v6)
        }
        Adt0::C10(_) => {
            v1
        }
        Adt0::C11(r254, r255) => {
            let v8 = (*r254).clone();
            let v9 = (*r255).clone();
            f47(v0.clone(), f47(v0, v1, v8), v9)
        }
        Adt0::C12(v10) => {
            f12(v1, Adt0::C12(v10))
        }
        Adt0::C13(r256, r257) => {
            let v11 = (*r256).clone();
            let v12 = (*r257).clone();
            if f23(v0, Adt0::C13(std::rc::Rc::new(v11.clone()), std::rc::Rc::new(v12.clone()))) {
                f12(v1, Adt0::C13(std::rc::Rc::new(v11), std::rc::Rc::new(v12)))
            } else {
                v1
            }
        }
        Adt0::C14 => {
            v1
        }
        Adt0::C15(_, _) => {
            v1
        }
    }
}

pub fn f48(v0: Adt10, p261: Adt10, p262: Adt10) -> Adt10 {
    let m260 = (p261, p262);
    {
        let mut m260 = m260;
        loop {
            let step = {
                let (v1, v2) = m260;
                let m258 = v1;
                match m258 {
                    Adt10::C0 => {
                        Err::<(Adt10, Adt10), Adt10>(v2)
                    }
                    Adt10::C1(v3, v4, r259) => {
                        let v5 = (*r259).clone();
                        Ok::<(Adt10, Adt10), Adt10>((v5, f58(v0.clone(), v4, v3, v2)))
                    }
                }
            };
            match step {
                Ok(next) => { m260 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f49(v0: Adt10) -> R<u64> {
    let m263 = v0;
    match m263 {
        Adt10::C0 => {
            Ok(0u64)
        }
        Adt10::C1(_, _, r266) => {
            let v3 = (*r266).clone();
            let a264 = 1u64;
            let a265 = f49(v3)?;
            nat_add(a264, a265)
        }
    }
}

pub fn f50(v0: Fn2, p271: u64, p272: Adt0) -> R<(Adt0, bool)> {
    let m270 = (p271, p272);
    {
        let mut m270 = m270;
        loop {
            let step = {
                let (v1, v2) = m270;
                let m267 = v1;
                if m267 == 0 {
                    Err::<(u64, Adt0), R<(Adt0, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m267 - 1;
                    let m269 = {
                        let c268: Fn2 = v0.clone();
                        c268.apply(v2.clone())?
                    };
                    match m269 {
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
                Ok(next) => { m270 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f51(v0: Adt9, v1: Adt10, v2: Adt0, v3: Adt0, v4: Adt7, v5: Adt0) -> R<Option<Adt0>> {
    f59(v4, v2, v0, v1, v3, v5)
}

pub fn f52(p277: Adt0, v1: Adt0) -> bool {
    let m276 = p277;
    {
        let mut m276 = m276;
        loop {
            let step = {
                let v0 = m276;
                let m273 = v0;
                match m273 {
                    Adt0::C0 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C1 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C2 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C3 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C4(_) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C5 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C6 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C7 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C8(_) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C9(_, _) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C10(_) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C11(_, _) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C12(_) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C13(_, _) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C14 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C15(r274, r275) => {
                        let v12 = (*r274).clone();
                        let v13 = (*r275).clone();
                        if f60(v12, v1.clone()) {
                            Ok::<Adt0, bool>(v13)
                        } else {
                            Err::<Adt0, bool>(true)
                        }
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

pub fn f53(v0: Adt0) -> u64 {
    let m278 = v0;
    match m278 {
        Adt0::C0 => {
            0u64
        }
        Adt0::C1 => {
            1u64
        }
        Adt0::C2 => {
            2u64
        }
        Adt0::C3 => {
            3u64
        }
        Adt0::C4(_) => {
            4u64
        }
        Adt0::C5 => {
            5u64
        }
        Adt0::C6 => {
            6u64
        }
        Adt0::C7 => {
            7u64
        }
        Adt0::C8(_) => {
            8u64
        }
        Adt0::C9(_, _) => {
            9u64
        }
        Adt0::C10(_) => {
            10u64
        }
        Adt0::C11(_, _) => {
            11u64
        }
        Adt0::C12(_) => {
            12u64
        }
        Adt0::C13(_, _) => {
            13u64
        }
        Adt0::C14 => {
            14u64
        }
        Adt0::C15(_, _) => {
            15u64
        }
    }
}

pub fn f54(v0: Adt1) -> u64 {
    let m279 = v0;
    match m279 {
        Adt1::C0 => {
            0u64
        }
        Adt1::C1 => {
            1u64
        }
        Adt1::C2 => {
            2u64
        }
        Adt1::C3 => {
            3u64
        }
        Adt1::C4 => {
            4u64
        }
        Adt1::C5 => {
            5u64
        }
        Adt1::C6 => {
            6u64
        }
        Adt1::C7 => {
            7u64
        }
    }
}

pub fn f55(v0: Adt0) -> Adt1 {
    let m280 = v0;
    match m280 {
        Adt0::C0 => {
            Adt1::C0
        }
        Adt0::C1 => {
            Adt1::C0
        }
        Adt0::C2 => {
            Adt1::C0
        }
        Adt0::C3 => {
            Adt1::C0
        }
        Adt0::C4(v1) => {
            v1
        }
        Adt0::C5 => {
            Adt1::C0
        }
        Adt0::C6 => {
            Adt1::C0
        }
        Adt0::C7 => {
            Adt1::C0
        }
        Adt0::C8(_) => {
            Adt1::C0
        }
        Adt0::C9(_, _) => {
            Adt1::C0
        }
        Adt0::C10(_) => {
            Adt1::C0
        }
        Adt0::C11(_, _) => {
            Adt1::C0
        }
        Adt0::C12(_) => {
            Adt1::C0
        }
        Adt0::C13(_, _) => {
            Adt1::C0
        }
        Adt0::C14 => {
            Adt1::C0
        }
        Adt0::C15(_, _) => {
            Adt1::C0
        }
    }
}

pub fn f56(v0: Adt0) -> u64 {
    let m281 = v0;
    match m281 {
        Adt0::C0 => {
            0u64
        }
        Adt0::C1 => {
            0u64
        }
        Adt0::C2 => {
            0u64
        }
        Adt0::C3 => {
            0u64
        }
        Adt0::C4(_) => {
            0u64
        }
        Adt0::C5 => {
            0u64
        }
        Adt0::C6 => {
            0u64
        }
        Adt0::C7 => {
            0u64
        }
        Adt0::C8(_) => {
            0u64
        }
        Adt0::C9(_, _) => {
            0u64
        }
        Adt0::C10(_) => {
            0u64
        }
        Adt0::C11(_, _) => {
            0u64
        }
        Adt0::C12(v8) => {
            v8
        }
        Adt0::C13(_, _) => {
            0u64
        }
        Adt0::C14 => {
            0u64
        }
        Adt0::C15(_, _) => {
            0u64
        }
    }
}

pub fn f57(v0: Adt10, v1: Adt0, v2: Adt0) -> Adt10 {
    if f61(v0.clone(), v1.clone(), v2.clone()) {
        v0
    } else {
        Adt10::C1(v1, v2, std::rc::Rc::new(v0))
    }
}

pub fn f58(p285: Adt10, v1: Adt0, v2: Adt0, p286: Adt10) -> Adt10 {
    let m284 = (p285, p286);
    {
        let mut m284 = m284;
        loop {
            let step = {
                let (v0, v3) = m284;
                let m282 = v0;
                match m282 {
                    Adt10::C0 => {
                        Err::<(Adt10, Adt10), Adt10>(v3)
                    }
                    Adt10::C1(v4, v5, r283) => {
                        let v6 = (*r283).clone();
                        Ok::<(Adt10, Adt10), Adt10>((v6, if f40(v4, v1.clone()) {
                            f57(v3, v2.clone(), v5)
                        } else {
                            v3
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m284 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f59(v0: Adt7, v1: Adt0, v2: Adt9, v3: Adt10, v4: Adt0, v5: Adt0) -> R<Option<Adt0>> {
    let v6: Adt0 = f62(v0, v1, v2, v3, v4, v5.clone());
    let a287 = f6(v6.clone())?;
    let a288 = f6(v5)?;
    if nat_eq(a287, a288) {
        Ok(None::<Adt0>)
    } else {
        Ok(Some(v6))
    }
}

pub fn f60(v0: Adt0, v1: Adt0) -> bool {
    let m289 = v0;
    match m289 {
        Adt0::C0 => {
            true
        }
        Adt0::C1 => {
            true
        }
        Adt0::C2 => {
            true
        }
        Adt0::C3 => {
            true
        }
        Adt0::C4(_) => {
            true
        }
        Adt0::C5 => {
            true
        }
        Adt0::C6 => {
            true
        }
        Adt0::C7 => {
            true
        }
        Adt0::C8(_) => {
            true
        }
        Adt0::C9(r290, r291) => {
            let v4 = (*r290).clone();
            let v5 = (*r291).clone();
            if f60(v4, v1.clone()) {
                true
            } else {
                f60(v5, v1)
            }
        }
        Adt0::C10(_) => {
            true
        }
        Adt0::C11(r292, r293) => {
            let v7 = (*r292).clone();
            let v8 = (*r293).clone();
            if f60(v7, v1.clone()) {
                f60(v8, v1)
            } else {
                false
            }
        }
        Adt0::C12(v9) => {
            f23(v1, Adt0::C12(v9))
        }
        Adt0::C13(r294, r295) => {
            let v10 = (*r294).clone();
            let v11 = (*r295).clone();
            f23(v1, Adt0::C13(std::rc::Rc::new(v10), std::rc::Rc::new(v11)))
        }
        Adt0::C14 => {
            true
        }
        Adt0::C15(_, _) => {
            true
        }
    }
}

pub fn f61(p299: Adt10, v1: Adt0, v2: Adt0) -> bool {
    let m298 = p299;
    {
        let mut m298 = m298;
        loop {
            let step = {
                let v0 = m298;
                let m296 = v0;
                match m296 {
                    Adt10::C0 => {
                        Err::<Adt10, bool>(false)
                    }
                    Adt10::C1(v3, v4, r297) => {
                        let v5 = (*r297).clone();
                        if f40(v3, v1.clone()) {
                            if f40(v4, v2.clone()) {
                                Err::<Adt10, bool>(true)
                            } else {
                                Ok::<Adt10, bool>(v5)
                            }
                        } else {
                            Ok::<Adt10, bool>(v5)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m298 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f62(v0: Adt7, v1: Adt0, v2: Adt9, v3: Adt10, p304: Adt0, p305: Adt0) -> Adt0 {
    let m303 = (p304, p305);
    {
        let mut m303 = m303;
        loop {
            let step = {
                let (v4, v5) = m303;
                let m300 = v4;
                match m300 {
                    Adt0::C0 => {
                        Err::<(Adt0, Adt0), Adt0>(v5)
                    }
                    Adt0::C1 => {
                        Err::<(Adt0, Adt0), Adt0>(v5)
                    }
                    Adt0::C2 => {
                        Err::<(Adt0, Adt0), Adt0>(v5)
                    }
                    Adt0::C3 => {
                        Err::<(Adt0, Adt0), Adt0>(v5)
                    }
                    Adt0::C4(_) => {
                        Err::<(Adt0, Adt0), Adt0>(v5)
                    }
                    Adt0::C5 => {
                        Err::<(Adt0, Adt0), Adt0>(v5)
                    }
                    Adt0::C6 => {
                        Err::<(Adt0, Adt0), Adt0>(v5)
                    }
                    Adt0::C7 => {
                        Err::<(Adt0, Adt0), Adt0>(v5)
                    }
                    Adt0::C8(_) => {
                        Err::<(Adt0, Adt0), Adt0>(v5)
                    }
                    Adt0::C9(_, _) => {
                        Err::<(Adt0, Adt0), Adt0>(v5)
                    }
                    Adt0::C10(_) => {
                        Err::<(Adt0, Adt0), Adt0>(v5)
                    }
                    Adt0::C11(_, _) => {
                        Err::<(Adt0, Adt0), Adt0>(v5)
                    }
                    Adt0::C12(_) => {
                        Err::<(Adt0, Adt0), Adt0>(v5)
                    }
                    Adt0::C13(_, _) => {
                        Err::<(Adt0, Adt0), Adt0>(v5)
                    }
                    Adt0::C14 => {
                        Err::<(Adt0, Adt0), Adt0>(v5)
                    }
                    Adt0::C15(r301, r302) => {
                        let v16 = (*r301).clone();
                        let v17 = (*r302).clone();
                        Ok::<(Adt0, Adt0), Adt0>((v17, if f23(v5.clone(), v16.clone()) {
                            v5
                        } else {
                            if f63(v1.clone(), v3.clone(), v5.clone(), v16.clone(), f19(v0.clone(), v2.clone(), v16.clone())) {
                                f12(v5, v16)
                            } else {
                                v5
                            }
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m303 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f63(v0: Adt0, v1: Adt10, v2: Adt0, v3: Adt0, p310: Adt0) -> bool {
    let m309 = p310;
    {
        let mut m309 = m309;
        loop {
            let step = {
                let v4 = m309;
                let m306 = v4;
                match m306 {
                    Adt0::C0 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C1 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C2 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C3 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C4(_) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C5 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C6 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C7 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C8(_) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C9(_, _) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C10(_) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C11(_, _) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C12(_) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C13(_, _) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C14 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C15(r307, r308) => {
                        let v15 = (*r307).clone();
                        let v16 = (*r308).clone();
                        if f64(v0.clone(), v1.clone(), v2.clone(), v3.clone(), v15) {
                            Err::<Adt0, bool>(true)
                        } else {
                            Ok::<Adt0, bool>(v16)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m309 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f64(v0: Adt0, v1: Adt10, v2: Adt0, v3: Adt0, p315: Adt0) -> bool {
    let m314 = p315;
    {
        let mut m314 = m314;
        loop {
            let step = {
                let v4 = m314;
                let m311 = v4;
                match m311 {
                    Adt0::C0 => {
                        Err::<Adt0, bool>(true)
                    }
                    Adt0::C1 => {
                        Err::<Adt0, bool>(true)
                    }
                    Adt0::C2 => {
                        Err::<Adt0, bool>(true)
                    }
                    Adt0::C3 => {
                        Err::<Adt0, bool>(true)
                    }
                    Adt0::C4(_) => {
                        Err::<Adt0, bool>(true)
                    }
                    Adt0::C5 => {
                        Err::<Adt0, bool>(true)
                    }
                    Adt0::C6 => {
                        Err::<Adt0, bool>(true)
                    }
                    Adt0::C7 => {
                        Err::<Adt0, bool>(true)
                    }
                    Adt0::C8(_) => {
                        Err::<Adt0, bool>(true)
                    }
                    Adt0::C9(_, _) => {
                        Err::<Adt0, bool>(true)
                    }
                    Adt0::C10(_) => {
                        Err::<Adt0, bool>(true)
                    }
                    Adt0::C11(_, _) => {
                        Err::<Adt0, bool>(true)
                    }
                    Adt0::C12(_) => {
                        Err::<Adt0, bool>(true)
                    }
                    Adt0::C13(_, _) => {
                        Err::<Adt0, bool>(true)
                    }
                    Adt0::C14 => {
                        Err::<Adt0, bool>(true)
                    }
                    Adt0::C15(r312, r313) => {
                        let v15 = (*r312).clone();
                        let v16 = (*r313).clone();
                        if if f65(v0.clone(), v1.clone(), v3.clone(), v15.clone()) {
                            true
                        } else {
                            f60(v15, v2.clone())
                        } {
                            Ok::<Adt0, bool>(v16)
                        } else {
                            Err::<Adt0, bool>(false)
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

pub fn f65(v0: Adt0, v1: Adt10, v2: Adt0, v3: Adt0) -> bool {
    f66(v2, v1, f47(v0, Adt0::C14, v3))
}

pub fn f66(v0: Adt0, v1: Adt10, p320: Adt0) -> bool {
    let m319 = p320;
    {
        let mut m319 = m319;
        loop {
            let step = {
                let v2 = m319;
                let m316 = v2;
                match m316 {
                    Adt0::C0 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C1 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C2 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C3 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C4(_) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C5 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C6 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C7 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C8(_) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C9(_, _) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C10(_) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C11(_, _) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C12(_) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C13(_, _) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C14 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C15(r317, r318) => {
                        let v13 = (*r317).clone();
                        let v14 = (*r318).clone();
                        if f40(v13.clone(), v0.clone()) {
                            Err::<Adt0, bool>(true)
                        } else {
                            if f61(v1.clone(), v13, v0.clone()) {
                                Err::<Adt0, bool>(true)
                            } else {
                                Ok::<Adt0, bool>(v14)
                            }
                        }
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

pub fn run(p0: Bytes, p1: Adt0, p2: List<List<u64>>, p3: Adt0, p4: Adt0, p5: Adt2, p6: Adt0) -> R<List<bool>> {
    f0(p0, p1, p2, p3, p4, p5, p6)
}
