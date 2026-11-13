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
    C0(Bytes, Bytes, Adt1, Adt2, Adt6, Adt8),
}

#[derive(Clone)]
pub enum Adt1 {
    C0,
    C1,
    C2,
    C3,
    C4,
    C5,
}

#[derive(Clone)]
pub enum Adt2 {
    C0(Adt3),
    C1(u64),
    C2(Adt4, Adt3),
    C3(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>, List<u64>),
    C4(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C5(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C6(u64, u64, Bytes, std::rc::Rc<Adt2>, std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C7(Bytes, std::rc::Rc<Adt2>),
    C8,
    C9(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C10(u64, u64, Bytes, std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
}

#[derive(Clone)]
pub enum Adt3 {
    C0(u64),
    C1(Bytes),
    C2(std::rc::Rc<Adt3>),
    C3(std::rc::Rc<Adt3>),
    C4(std::rc::Rc<Adt3>, std::rc::Rc<Adt3>),
    C5,
    C6(std::rc::Rc<Adt3>, std::rc::Rc<Adt3>),
}

#[derive(Clone)]
pub enum Adt4 {
    C0(Adt5),
    C1(Bytes, Bytes, Bytes, Bytes, Bytes),
    C2(Bytes, Bytes, Bytes),
    C3(Bytes, Bytes, Bytes),
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
}

#[derive(Clone)]
pub enum Adt6 {
    C0(Bytes, Adt2, Adt2),
    C1(Adt2, Adt7),
}

#[derive(Clone)]
pub enum Adt7 {
    C0(std::rc::Rc<Adt7>),
    C1(List<u64>),
    C2(Adt2),
    C3(Adt2),
    C4(Adt2, std::rc::Rc<Adt7>),
    C5,
    C6(Adt2),
    C7,
    C8,
    C9(u64, Adt2, std::rc::Rc<Adt7>),
    C10(Option<u64>, std::rc::Rc<Adt7>),
    C11(Option<u64>, Adt2),
    C12(std::rc::Rc<Adt7>),
    C13(Adt2, std::rc::Rc<Adt7>),
    C14(Adt2, std::rc::Rc<Adt7>),
    C15(Adt4, Adt2, std::rc::Rc<Adt7>),
    C16,
    C17(std::rc::Rc<Adt7>, std::rc::Rc<Adt7>),
    C18(bool, Adt2, std::rc::Rc<Adt7>),
    C19(Bytes, Bytes, List<(u64, Bytes)>, std::rc::Rc<Adt7>, std::rc::Rc<Adt7>),
    C20(Adt2, Adt2, std::rc::Rc<Adt7>),
}

#[derive(Clone)]
pub enum Adt8 {
    C0,
    C1(List<Bytes>),
    C2(List<Bytes>),
}

#[derive(Clone)]
pub enum Adt9 {
    C0(bool),
    C1(bool, u64),
    C2(Bytes),
    C3(std::rc::Rc<Adt9>),
    C4(std::rc::Rc<Adt9>),
    C5,
    C6(std::rc::Rc<Adt9>, std::rc::Rc<Adt9>),
    C7(Bytes, std::rc::Rc<Adt9>, std::rc::Rc<Adt9>),
}

#[derive(Clone)]
pub enum Adt10 {
    C0(List<(u64, u64)>, u64, bool),
}

#[derive(Clone)]
pub enum Adt11 {
    C0(u64),
    C1(u64),
}

#[derive(Clone)]
pub enum Fn0 {
    F33(Bytes),
    F79(u64, Bytes),
    F86(Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn0::F33(k0) => f33(k0.clone(), p0),
            Fn0::F79(k0, k1) => Ok(f79(*k0, k1.clone(), p0)),
            Fn0::F86(k0) => f86(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F36,
    F106,
}

impl Fn1 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn1::F36 => f36(p0, p1),
            Fn1::F106 => f106(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F39,
}

impl Fn2 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, Bytes)) -> R<List<Bytes>> {
        match self {
            Fn2::F39 => f39(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F52,
}

impl Fn3 {
    pub fn apply(&self, p0: Adt9, p1: Bytes) -> Adt9 {
        match self {
            Fn3::F52 => f52(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F58(Bytes),
}

impl Fn4 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Bytes) -> Option<Bytes> {
        match self {
            Fn4::F58(k0) => f58(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F64(u64),
}

impl Fn5 {
    pub fn apply(&self, p0: Option<u64>, p1: (u64, u64)) -> Option<u64> {
        match self {
            Fn5::F64(k0) => f64(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F68,
    F73,
}

impl Fn6 {
    pub fn apply(&self, p0: Adt9, p1: u64) -> Adt9 {
        match self {
            Fn6::F68 => f68(p0, p1),
            Fn6::F73 => f73(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F72,
}

impl Fn7 {
    pub fn apply(&self, p0: (List<u64>, Adt10), p1: u64) -> R<(List<u64>, Adt10)> {
        match self {
            Fn7::F72 => f72(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F75,
}

impl Fn8 {
    pub fn apply(&self, p0: List<u64>, p1: (u64, Bytes)) -> List<u64> {
        match self {
            Fn8::F75 => f75(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F77,
}

impl Fn9 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn9::F77 => f77(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F83,
}

impl Fn10 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn10::F83 => f83(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F85,
}

impl Fn11 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
        match self {
            Fn11::F85 => f85(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F88,
}

impl Fn12 {
    pub fn apply(&self, p0: List<u64>, p1: u64) -> List<u64> {
        match self {
            Fn12::F88 => f88(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F94,
}

impl Fn13 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn13::F94 => f94(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F100,
}

impl Fn14 {
    pub fn apply(&self, p0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
        match self {
            Fn14::F100 => f100(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F105,
}

impl Fn15 {
    pub fn apply(&self, p0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
        match self {
            Fn15::F105 => f105(p0),
        }
    }
}

pub fn f0(v0: Adt0) -> R<Bytes> {
    f1({
        let (h1, _) = f2(v0, f3())?;
        h1
    })
}

pub fn f1(v0: Adt9) -> R<Bytes> {
    f4(v0)
}

pub fn f2(v0: Adt0, v1: Adt10) -> R<(Adt9, Adt10)> {
    let v2: (Adt9, Adt10) = f5(match v0.clone() {
        Adt0::C0(_, _, _, h2, _, _) => {
            h2
        }
    }, v1)?;
    let v3: (Adt9, Adt10) = f6(match v0.clone() {
        Adt0::C0(_, _, _, _, h3, _) => {
            h3
        }
    }, {
        let (_, h4) = v2;
        h4
    })?;
    Ok((f7(f8(Str::lit("component"), Adt9::C2(match v0.clone() {
        Adt0::C0(h5, _, _, _, _, _) => {
            h5
        }
    }), f8(Str::lit("lean_name"), Adt9::C2(match v0.clone() {
        Adt0::C0(_, h6, _, _, _, _) => {
            h6
        }
    }), f8(Str::lit("kind"), f9(match v0.clone() {
        Adt0::C0(_, _, h7, _, _, _) => {
            h7
        }
    }), f8(Str::lit("params"), f10({
        let (h8, _) = v2;
        h8
    }), f8(Str::lit("body"), {
        let (h9, _) = v3;
        h9
    }, f8(Str::lit("policy"), f11(match v0 {
        Adt0::C0(_, _, _, _, _, h10) => {
            h10
        }
    }), f12()))))))), {
        let (_, h11) = v3;
        h11
    }))
}

pub fn f3() -> Adt10 {
    Adt10::C0(List::<(u64, u64)>::nil(), 0u64, false)
}

pub fn f4(v0: Adt9) -> R<Bytes> {
    let m12 = v0;
    match m12 {
        Adt9::C0(v1) => {
            if v1 {
                Ok(Bytes::lit(&[0x74, 0x72, 0x75, 0x65]))
            } else {
                Ok(Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]))
            }
        }
        Adt9::C1(v2, v3) => {
            f13(v2, v3)
        }
        Adt9::C2(v4) => {
            f14(v4)
        }
        Adt9::C3(r13) => {
            let v5 = (*r13).clone();
            Ok(f15(f16(v5)?))
        }
        Adt9::C4(r14) => {
            let v6 = (*r14).clone();
            f17(f18(v6)?)
        }
        Adt9::C5 => {
            Ok(Bytes::lit(&[]))
        }
        Adt9::C6(_, _) => {
            Ok(Bytes::lit(&[]))
        }
        Adt9::C7(_, _, _) => {
            Ok(Bytes::lit(&[]))
        }
    }
}

pub fn f5(v0: Adt2, v1: Adt10) -> R<(Adt9, Adt10)> {
    let m15 = v0;
    match m15 {
        Adt2::C0(v2) => {
            Ok((Adt9::C4(std::rc::Rc::new(f8(Str::lit("k"), f19(Str::lit("sort")), f8(Str::lit("u"), f20(v2), Adt9::C5)))), v1))
        }
        Adt2::C1(v3) => {
            let v4: (Adt11, Adt10) = f21(v1, v3)?;
            Ok((f22({
                let (h16, _) = v4;
                h16
            }), {
                let (_, h17) = v4;
                h17
            }))
        }
        Adt2::C2(v5, v6) => {
            Ok((f23(v5, v6)?, v1))
        }
        Adt2::C3(r50, r51, v9) => {
            let v7 = (*r50).clone();
            let v8 = (*r51).clone();
            let v10: (Adt9, Adt10) = f5(v7, v1)?;
            let v11: (Adt9, Adt10) = f5(v8, {
                let (_, h18) = v10;
                h18
            })?;
            Ok((f24({
                let (h19, _) = v10;
                h19
            }, {
                let (h20, _) = v11;
                h20
            }, v9), {
                let (_, h21) = v11;
                h21
            }))
        }
        Adt2::C4(r52, r53) => {
            let v12 = (*r52).clone();
            let v13 = (*r53).clone();
            let v14: (Adt9, Adt10) = f5(v12, v1)?;
            let v15: (Adt9, Adt10) = f5(v13, {
                let (_, h22) = v14;
                h22
            })?;
            Ok((Adt9::C4(std::rc::Rc::new(f8(Str::lit("k"), f19(Str::lit("pi")), f8(Str::lit("b"), Adt9::C3(std::rc::Rc::new({
                let (h23, _) = v14;
                h23
            })), f8(Str::lit("v"), {
                let (h24, _) = v15;
                h24
            }, Adt9::C5))))), {
                let (_, h25) = v15;
                h25
            }))
        }
        Adt2::C5(r54, r55) => {
            let v16 = (*r54).clone();
            let v17 = (*r55).clone();
            let v18: (Adt9, Adt10) = f5(v16, v1)?;
            let v19: (Adt9, Adt10) = f5(v17, {
                let (_, h26) = v18;
                h26
            })?;
            Ok((Adt9::C4(std::rc::Rc::new(f8(Str::lit("k"), f19(Str::lit("lam")), f8(Str::lit("b"), Adt9::C3(std::rc::Rc::new({
                let (h27, _) = v18;
                h27
            })), f8(Str::lit("v"), {
                let (h28, _) = v19;
                h28
            }, Adt9::C5))))), {
                let (_, h29) = v19;
                h29
            }))
        }
        Adt2::C6(v20, v21, v22, r56, r57, r58) => {
            let v23 = (*r56).clone();
            let v24 = (*r57).clone();
            let v25 = (*r58).clone();
            let v26: (Adt9, Adt10) = f5(v23, v1)?;
            let v27: (Adt9, Adt10) = f5(v24, {
                let (_, h30) = v26;
                h30
            })?;
            let v28: (u64, Adt10) = f25({
                let (_, h31) = v27;
                h31
            }, v20)?;
            let v29: (Adt9, Adt10) = f5(v25, {
                let (_, h32) = v28;
                h32
            })?;
            Ok((Adt9::C4(std::rc::Rc::new(f8(Str::lit("k"), f19(Str::lit("let")), f8(Str::lit("b"), f26({
                let (h33, _) = v28;
                h33
            }, v21, v22, {
                let (h34, _) = v26;
                h34
            }), f8(Str::lit("x"), {
                let (h35, _) = v27;
                h35
            }, f8(Str::lit("v"), {
                let (h36, _) = v29;
                h36
            }, Adt9::C5)))))), {
                let (_, h37) = v29;
                h37
            }))
        }
        Adt2::C7(v30, r59) => {
            let v31 = (*r59).clone();
            let v32: (Adt9, Adt10) = f5(v31, v1)?;
            Ok((Adt9::C4(std::rc::Rc::new(f8(Str::lit("k"), f19(Str::lit("nat")), f8(Str::lit("d"), Adt9::C2(v30), f8(Str::lit("t"), {
                let (h38, _) = v32;
                h38
            }, Adt9::C5))))), {
                let (_, h39) = v32;
                h39
            }))
        }
        Adt2::C8 => {
            Ok((Adt9::C5, v1))
        }
        Adt2::C9(r60, r61) => {
            let v33 = (*r60).clone();
            let v34 = (*r61).clone();
            let v35: (Adt9, Adt10) = f5(v33, v1)?;
            let v36: (Adt9, Adt10) = f5(v34, {
                let (_, h40) = v35;
                h40
            })?;
            Ok((Adt9::C6(std::rc::Rc::new({
                let (h41, _) = v35;
                h41
            }), std::rc::Rc::new({
                let (h42, _) = v36;
                h42
            })), {
                let (_, h43) = v36;
                h43
            }))
        }
        Adt2::C10(v37, v38, v39, r62, r63) => {
            let v40 = (*r62).clone();
            let v41 = (*r63).clone();
            let v42: (Adt9, Adt10) = f5(v40, v1)?;
            let v43: (u64, Adt10) = f25({
                let (_, h44) = v42;
                h44
            }, v37)?;
            let v44: (Adt9, Adt10) = f5(v41, {
                let (_, h45) = v43;
                h45
            })?;
            Ok((Adt9::C6(std::rc::Rc::new(f26({
                let (h46, _) = v43;
                h46
            }, v38, v39, {
                let (h47, _) = v42;
                h47
            })), std::rc::Rc::new({
                let (h48, _) = v44;
                h48
            })), {
                let (_, h49) = v44;
                h49
            }))
        }
    }
}

pub fn f6(v0: Adt6, v1: Adt10) -> R<(Adt9, Adt10)> {
    let m64 = v0;
    match m64 {
        Adt6::C0(v2, v3, v4) => {
            let v5: (Adt9, Adt10) = f5(v3, v1)?;
            let v6: (Adt9, Adt10) = f5(v4, {
                let (_, h65) = v5;
                h65
            })?;
            Ok((f27(Str::lit("definition"), f8(Str::lit("entry"), Adt9::C2(v2), f8(Str::lit("t"), {
                let (h66, _) = v5;
                h66
            }, f8(Str::lit("v"), {
                let (h67, _) = v6;
                h67
            }, f12())))), {
                let (_, h68) = v6;
                h68
            }))
        }
        Adt6::C1(v7, v8) => {
            let v9: (Adt9, Adt10) = f5(v7, v1)?;
            let v10: (Adt9, Adt10) = f28(v8, {
                let (_, h69) = v9;
                h69
            })?;
            Ok((f27(Str::lit("theorem"), f8(Str::lit("s"), {
                let (h70, _) = v9;
                h70
            }, f8(Str::lit("p"), {
                let (h71, _) = v10;
                h71
            }, f12()))), {
                let (_, h72) = v10;
                h72
            }))
        }
    }
}

pub fn f7(v0: Adt9) -> Adt9 {
    Adt9::C4(std::rc::Rc::new(v0))
}

pub fn f8(v0: Str, v1: Adt9, v2: Adt9) -> Adt9 {
    Adt9::C7(f29(v0), std::rc::Rc::new(v1), std::rc::Rc::new(v2))
}

pub fn f9(v0: Adt1) -> Adt9 {
    let m73 = v0;
    match m73 {
        Adt1::C0 => {
            f19(Str::lit("typedefinition"))
        }
        Adt1::C1 => {
            f19(Str::lit("termdefinition"))
        }
        Adt1::C2 => {
            f19(Str::lit("predicatedefinition"))
        }
        Adt1::C3 => {
            f19(Str::lit("theorem"))
        }
        Adt1::C4 => {
            f19(Str::lit("lemma"))
        }
        Adt1::C5 => {
            f19(Str::lit("corollary"))
        }
    }
}

pub fn f10(v0: Adt9) -> Adt9 {
    Adt9::C3(std::rc::Rc::new(v0))
}

pub fn f11(v0: Adt8) -> Adt9 {
    let m74 = v0;
    match m74 {
        Adt8::C0 => {
            f7(f8(Str::lit("kind"), f19(Str::lit("none")), f8(Str::lit("axioms"), f10(f12()), f12())))
        }
        Adt8::C1(v1) => {
            f7(f8(Str::lit("kind"), f19(Str::lit("allow")), f8(Str::lit("axioms"), f10(f30(v1)), f12())))
        }
        Adt8::C2(v2) => {
            f7(f8(Str::lit("kind"), f19(Str::lit("exact")), f8(Str::lit("axioms"), f10(f30(v2)), f12())))
        }
    }
}

pub fn f12() -> Adt9 {
    Adt9::C5
}

pub fn f13(v0: bool, v1: u64) -> R<Bytes> {
    if if v0 {
        let a75 = 0u64;
        let a76 = v1;
        nat_lt(a75, a76)
    } else {
        false
    } {
        let a77 = Bytes::lit(&[0x2d]);
        let a78 = f31(v1)?;
        Ok(append_bytes(a77, a78))
    } else {
        f31(v1)
    }
}

pub fn f14(v0: Bytes) -> R<Bytes> {
    let a84 = {
        let a82 = Bytes::lit(&[0x22]);
        let a83 = {
            let (_, h79) = {
                let (h80, _) = f32(Fn0::F33(v0.clone()), {
                    let a81 = v0;
                    length_bytes(a81)
                }, (0u64, Bytes::lit(&[])))?;
                h80
            };
            h79
        };
        append_bytes(a82, a83)
    };
    let a85 = Bytes::lit(&[0x22]);
    Ok(append_bytes(a84, a85))
}

pub fn f15(v0: List<Bytes>) -> Bytes {
    let a88 = {
        let a86 = Bytes::lit(&[0x5b]);
        let a87 = f34(v0, Bytes::lit(&[0x2c]));
        append_bytes(a86, a87)
    };
    let a89 = Bytes::lit(&[0x5d]);
    append_bytes(a88, a89)
}

pub fn f16(v0: Adt9) -> R<List<Bytes>> {
    let m90 = v0;
    match m90 {
        Adt9::C0(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt9::C1(_, _) => {
            Ok(List::<Bytes>::nil())
        }
        Adt9::C2(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt9::C3(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt9::C4(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt9::C5 => {
            Ok(List::<Bytes>::nil())
        }
        Adt9::C6(r91, r92) => {
            let v7 = (*r91).clone();
            let v8 = (*r92).clone();
            Ok(List::cons(f4(v7)?, f16(v8)?))
        }
        Adt9::C7(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f17(v0: List<(Bytes, Bytes)>) -> R<Bytes> {
    let v1: List<(Bytes, Bytes)> = f35(Fn1::F36, List::<(Bytes, Bytes)>::nil(), f37(v0)?);
    let v2: List<Bytes> = f38(Fn2::F39, List::<Bytes>::nil(), v1)?;
    let a95 = {
        let a93 = Bytes::lit(&[0x7b]);
        let a94 = f34(v2, Bytes::lit(&[0x2c]));
        append_bytes(a93, a94)
    };
    let a96 = Bytes::lit(&[0x7d]);
    Ok(append_bytes(a95, a96))
}

pub fn f18(v0: Adt9) -> R<List<(Bytes, Bytes)>> {
    let m97 = v0;
    match m97 {
        Adt9::C0(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt9::C1(_, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt9::C2(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt9::C3(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt9::C4(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt9::C5 => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt9::C6(_, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt9::C7(v9, r98, r99) => {
            let v10 = (*r98).clone();
            let v11 = (*r99).clone();
            Ok(List::cons((v9, f4(v10)?), f18(v11)?))
        }
    }
}

pub fn f19(v0: Str) -> Adt9 {
    Adt9::C2(f29(v0))
}

pub fn f20(v0: Adt3) -> Adt9 {
    let m100 = v0;
    match m100 {
        Adt3::C0(v1) => {
            Adt9::C4(std::rc::Rc::new(f8(Str::lit("num"), f40(v1), Adt9::C5)))
        }
        Adt3::C1(v2) => {
            Adt9::C4(std::rc::Rc::new(f8(Str::lit("var"), Adt9::C2(v2), Adt9::C5)))
        }
        Adt3::C2(r101) => {
            let v3 = (*r101).clone();
            Adt9::C4(std::rc::Rc::new(f8(Str::lit("succ"), f20(v3), Adt9::C5)))
        }
        Adt3::C3(r102) => {
            let v4 = (*r102).clone();
            Adt9::C4(std::rc::Rc::new(f8(Str::lit("max"), Adt9::C3(std::rc::Rc::new(f20(v4))), Adt9::C5)))
        }
        Adt3::C4(r103, r104) => {
            let v5 = (*r103).clone();
            let v6 = (*r104).clone();
            Adt9::C4(std::rc::Rc::new(f8(Str::lit("imax"), Adt9::C3(std::rc::Rc::new(Adt9::C6(std::rc::Rc::new(f20(v5)), std::rc::Rc::new(Adt9::C6(std::rc::Rc::new(f20(v6)), std::rc::Rc::new(Adt9::C5)))))), Adt9::C5)))
        }
        Adt3::C5 => {
            Adt9::C5
        }
        Adt3::C6(r105, r106) => {
            let v7 = (*r105).clone();
            let v8 = (*r106).clone();
            Adt9::C6(std::rc::Rc::new(f20(v7)), std::rc::Rc::new(f20(v8)))
        }
    }
}

pub fn f21(v0: Adt10, v1: u64) -> R<(Adt11, Adt10)> {
    let m108 = f41(match v0.clone() {
        Adt10::C0(h107, _, _) => {
            h107
        }
    }, v1);
    match m108 {
        None => {
            if match v0.clone() {
                Adt10::C0(_, _, h109) => {
                    h109
                }
            } {
                Ok((Adt11::C1(v1), v0))
            } else {
                let v3: (u64, Adt10) = f42(v0, v1)?;
                Ok((Adt11::C0({
                    let (h110, _) = v3;
                    h110
                }), {
                    let (_, h111) = v3;
                    h111
                }))
            }
        }
        Some(v2) => {
            Ok((Adt11::C0(v2), v0))
        }
    }
}

pub fn f22(v0: Adt11) -> Adt9 {
    let m112 = v0;
    match m112 {
        Adt11::C0(v1) => {
            Adt9::C4(std::rc::Rc::new(f8(Str::lit("k"), f19(Str::lit("local")), f8(Str::lit("id"), f40(v1), Adt9::C5))))
        }
        Adt11::C1(v2) => {
            Adt9::C4(std::rc::Rc::new(f8(Str::lit("k"), f19(Str::lit("local")), f8(Str::lit("free"), f40(v2), Adt9::C5))))
        }
    }
}

pub fn f23(v0: Adt4, v1: Adt3) -> R<Adt9> {
    let v2: Adt9 = if f43(v1.clone()) {
        Adt9::C5
    } else {
        f8(Str::lit("u"), Adt9::C3(std::rc::Rc::new(f20(v1))), Adt9::C5)
    };
    Ok(Adt9::C4(std::rc::Rc::new(f8(Str::lit("k"), f19(Str::lit("global")), f8(Str::lit("g"), f44(v0)?, v2)))))
}

pub fn f24(v0: Adt9, v1: Adt9, v2: List<u64>) -> Adt9 {
    let v5: Adt9 = {
        let m113 = v2.clone();
        match m113.uncons() {
            None => {
                Adt9::C5
            }
            Some((_, _)) => {
                f8(Str::lit("i"), Adt9::C3(std::rc::Rc::new(f45(v2))), Adt9::C5)
            }
        }
    };
    Adt9::C4(std::rc::Rc::new(f8(Str::lit("k"), f19(Str::lit("app")), f8(Str::lit("f"), v0, f8(Str::lit("a"), Adt9::C3(std::rc::Rc::new(v1)), v5)))))
}

pub fn f25(v0: Adt10, v1: u64) -> R<(u64, Adt10)> {
    let m115 = f41(match v0.clone() {
        Adt10::C0(h114, _, _) => {
            h114
        }
    }, v1);
    match m115 {
        None => {
            f42(v0, v1)
        }
        Some(v2) => {
            Ok((v2, v0))
        }
    }
}

pub fn f26(v0: u64, v1: u64, v2: Bytes, v3: Adt9) -> Adt9 {
    Adt9::C4(std::rc::Rc::new(f8(Str::lit("id"), f40(v0), f8(Str::lit("m"), f46(v1), f8(Str::lit("s"), Adt9::C2(v2), f8(Str::lit("t"), v3, Adt9::C5))))))
}

pub fn f27(v0: Str, v1: Adt9) -> Adt9 {
    f7(f8(Str::lit("k"), f19(v0), v1))
}

pub fn f28(v0: Adt7, v1: Adt10) -> R<(Adt9, Adt10)> {
    let m116 = v0;
    match m116 {
        Adt7::C0(r179) => {
            let v2 = (*r179).clone();
            let v3: (Adt9, Adt10) = f28(v2, v1)?;
            Ok((f27(Str::lit("seq"), f8(Str::lit("s"), f10({
                let (h117, _) = v3;
                h117
            }), f12())), {
                let (_, h118) = v3;
                h118
            }))
        }
        Adt7::C1(v4) => {
            let v5: (Adt9, Adt10) = f47(v4, v1)?;
            Ok((f27(Str::lit("intro"), f8(Str::lit("l"), f10({
                let (h119, _) = v5;
                h119
            }), f12())), {
                let (_, h120) = v5;
                h120
            }))
        }
        Adt7::C2(v6) => {
            let v7: (Adt9, Adt10) = f48(Str::lit("t"), v6, v1, f12())?;
            Ok((f27(Str::lit("exact"), {
                let (h121, _) = v7;
                h121
            }), {
                let (_, h122) = v7;
                h122
            }))
        }
        Adt7::C3(v8) => {
            let v9: (Adt9, Adt10) = f48(Str::lit("t"), v8, v1, f12())?;
            Ok((f27(Str::lit("apply-one"), {
                let (h123, _) = v9;
                h123
            }), {
                let (_, h124) = v9;
                h124
            }))
        }
        Adt7::C4(v10, r180) => {
            let v11 = (*r180).clone();
            let v12: (Adt9, Adt10) = f5(v10, v1)?;
            let v13: (Adt9, Adt10) = f28(v11, {
                let (_, h125) = v12;
                h125
            })?;
            Ok((f27(Str::lit("apply"), f8(Str::lit("f"), {
                let (h126, _) = v12;
                h126
            }, f8(Str::lit("p"), f10({
                let (h127, _) = v13;
                h127
            }), f12()))), {
                let (_, h128) = v13;
                h128
            }))
        }
        Adt7::C5 => {
            Ok((f27(Str::lit("rfl"), f12()), v1))
        }
        Adt7::C6(v14) => {
            let v15: (Adt9, Adt10) = f48(Str::lit("t"), v14, v1, f12())?;
            Ok((f27(Str::lit("witness"), {
                let (h129, _) = v15;
                h129
            }), {
                let (_, h130) = v15;
                h130
            }))
        }
        Adt7::C7 => {
            Ok((f27(Str::lit("left"), f12()), v1))
        }
        Adt7::C8 => {
            Ok((f27(Str::lit("right"), f12()), v1))
        }
        Adt7::C9(v16, v17, r181) => {
            let v18 = (*r181).clone();
            let v19: (Adt9, Adt10) = f5(v17, v1)?;
            let v20: (u64, Adt10) = f25({
                let (_, h131) = v19;
                h131
            }, v16)?;
            let v21: (Adt9, Adt10) = f28(v18, {
                let (_, h132) = v20;
                h132
            })?;
            Ok((f27(Str::lit("have"), f8(Str::lit("l"), f40({
                let (h133, _) = v20;
                h133
            }), f8(Str::lit("p"), {
                let (h134, _) = v19;
                h134
            }, f8(Str::lit("q"), {
                let (h135, _) = v21;
                h135
            }, f12())))), {
                let (_, h136) = v21;
                h136
            }))
        }
        Adt7::C10(v22, r182) => {
            let v23 = (*r182).clone();
            let v24: (Adt9, Adt10) = f49(v22, v1)?;
            let v25: (Adt9, Adt10) = f28(v23, {
                let (_, h137) = v24;
                h137
            })?;
            Ok((f27(Str::lit("rw"), f8(Str::lit("at"), {
                let (h138, _) = v24;
                h138
            }, f8(Str::lit("r"), f10({
                let (h139, _) = v25;
                h139
            }), f12()))), {
                let (_, h140) = v25;
                h140
            }))
        }
        Adt7::C11(v26, v27) => {
            let v28: (Adt9, Adt10) = f49(v26, v1)?;
            let v29: (Adt9, Adt10) = f5(v27, {
                let (_, h141) = v28;
                h141
            })?;
            Ok((f27(Str::lit("simp-only"), f8(Str::lit("at"), {
                let (h142, _) = v28;
                h142
            }, f8(Str::lit("r"), f10({
                let (h143, _) = v29;
                h143
            }), f12()))), {
                let (_, h144) = v29;
                h144
            }))
        }
        Adt7::C12(r183) => {
            let v30 = (*r183).clone();
            let v31: (Adt9, Adt10) = f28(v30, v1)?;
            Ok((f27(Str::lit("constructor"), f8(Str::lit("b"), f10({
                let (h145, _) = v31;
                h145
            }), f12())), {
                let (_, h146) = v31;
                h146
            }))
        }
        Adt7::C13(v32, r184) => {
            let v33 = (*r184).clone();
            let v34: (Adt9, Adt10) = f5(v32, v1)?;
            let v35: (Adt9, Adt10) = f28(v33, {
                let (_, h147) = v34;
                h147
            })?;
            Ok((f27(Str::lit("cases"), f8(Str::lit("t"), {
                let (h148, _) = v34;
                h148
            }, f8(Str::lit("c"), f10({
                let (h149, _) = v35;
                h149
            }), f12()))), {
                let (_, h150) = v35;
                h150
            }))
        }
        Adt7::C14(v36, r185) => {
            let v37 = (*r185).clone();
            let v38: (Adt9, Adt10) = f5(v36, v1)?;
            let v39: (Adt9, Adt10) = f28(v37, {
                let (_, h151) = v38;
                h151
            })?;
            Ok((f27(Str::lit("induction"), f8(Str::lit("t"), {
                let (h152, _) = v38;
                h152
            }, f8(Str::lit("c"), f10({
                let (h153, _) = v39;
                h153
            }), f12()))), {
                let (_, h154) = v39;
                h154
            }))
        }
        Adt7::C15(v40, v41, r186) => {
            let v42 = (*r186).clone();
            let v43: (Adt9, Adt10) = f5(v41, v1)?;
            let v44: (Adt9, Adt10) = f28(v42, {
                let (_, h155) = v43;
                h155
            })?;
            Ok((f27(Str::lit("calc"), f8(Str::lit("rel"), f23(v40, Adt3::C5)?, f8(Str::lit("s"), {
                let (h156, _) = v43;
                h156
            }, f8(Str::lit("steps"), f10({
                let (h157, _) = v44;
                h157
            }), f12())))), {
                let (_, h158) = v44;
                h158
            }))
        }
        Adt7::C16 => {
            Ok((f12(), v1))
        }
        Adt7::C17(r187, r188) => {
            let v45 = (*r187).clone();
            let v46 = (*r188).clone();
            let v47: (Adt9, Adt10) = f28(v45, v1)?;
            let v48: (Adt9, Adt10) = f28(v46, {
                let (_, h159) = v47;
                h159
            })?;
            Ok((Adt9::C6(std::rc::Rc::new({
                let (h160, _) = v47;
                h160
            }), std::rc::Rc::new({
                let (h161, _) = v48;
                h161
            })), {
                let (_, h162) = v48;
                h162
            }))
        }
        Adt7::C18(v49, v50, r189) => {
            let v51 = (*r189).clone();
            let v52: (Adt9, Adt10) = f5(v50, v1)?;
            let v53: (Adt9, Adt10) = f28(v51, {
                let (_, h163) = v52;
                h163
            })?;
            Ok((Adt9::C6(std::rc::Rc::new(f7(f8(Str::lit("rev"), Adt9::C0(v49), f8(Str::lit("t"), {
                let (h164, _) = v52;
                h164
            }, f12())))), std::rc::Rc::new({
                let (h165, _) = v53;
                h165
            })), {
                let (_, h166) = v53;
                h166
            }))
        }
        Adt7::C19(v54, v55, v56, r190, r191) => {
            let v57 = (*r190).clone();
            let v58 = (*r191).clone();
            let v59: (Adt9, Adt10) = f47(f50(v56), v1)?;
            let v60: (Adt9, Adt10) = f28(v57, {
                let (_, h167) = v59;
                h167
            })?;
            let v61: (Adt9, Adt10) = f28(v58, {
                let (_, h168) = v60;
                h168
            })?;
            Ok((Adt9::C6(std::rc::Rc::new(f7(f8(Str::lit("ctor"), Adt9::C2(v54), f8(Str::lit("lean"), Adt9::C2(v55), f8(Str::lit("b"), f10({
                let (h169, _) = v59;
                h169
            }), f8(Str::lit("p"), {
                let (h170, _) = v60;
                h170
            }, f12())))))), std::rc::Rc::new({
                let (h171, _) = v61;
                h171
            })), {
                let (_, h172) = v61;
                h172
            }))
        }
        Adt7::C20(v62, v63, r192) => {
            let v64 = (*r192).clone();
            let v65: (Adt9, Adt10) = f5(v62, v1)?;
            let v66: (Adt9, Adt10) = f5(v63, {
                let (_, h173) = v65;
                h173
            })?;
            let v67: (Adt9, Adt10) = f28(v64, {
                let (_, h174) = v66;
                h174
            })?;
            Ok((Adt9::C6(std::rc::Rc::new(f7(f8(Str::lit("t"), {
                let (h175, _) = v65;
                h175
            }, f8(Str::lit("p"), {
                let (h176, _) = v66;
                h176
            }, f12())))), std::rc::Rc::new({
                let (h177, _) = v67;
                h177
            })), {
                let (_, h178) = v67;
                h178
            }))
        }
    }
}

pub fn f29(v0: Str) -> Bytes {
    let a193 = v0;
    utf8_encode(a193)
}

pub fn f30(v0: List<Bytes>) -> Adt9 {
    f51(Fn3::F52, f12(), f53(v0))
}

pub fn f31(v0: u64) -> R<Bytes> {
    f54(f55(), 10u64, v0)
}

pub fn f32(v0: Fn0, p198: u64, p199: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m197 = (p198, p199);
    {
        let mut m197 = m197;
        loop {
            let step = {
                let (v1, v2) = m197;
                let m194 = v1;
                if m194 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m194 - 1;
                    let m196 = {
                        let c195: Fn0 = v0.clone();
                        c195.apply(v2.clone())?
                    };
                    match m196 {
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
                Ok(next) => { m197 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f33(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f56(v0, v1)
}

pub fn f34(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m200 = f57(Fn4::F58(v1), None::<Bytes>, v0);
    match m200 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f35(v0: Fn1, p204: List<(Bytes, Bytes)>, p205: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m203 = (p204, p205);
    {
        let mut m203 = m203;
        loop {
            let step = {
                let (v1, v2) = m203;
                let m201 = v2;
                match m201.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c202: Fn1 = v0.clone();
                            c202.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m203 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f36(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    f59(v0, v1)
}

pub fn f37(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    f60(v0)
}

pub fn f38(v0: Fn2, p209: List<Bytes>, p210: List<(Bytes, Bytes)>) -> R<List<Bytes>> {
    let m208 = (p209, p210);
    {
        let mut m208 = m208;
        loop {
            let step = {
                let (v1, v2) = m208;
                let m206 = v2;
                match m206.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(({
                            let c207: Fn2 = v0.clone();
                            c207.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m208 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f39(v0: List<Bytes>, v1: (Bytes, Bytes)) -> R<List<Bytes>> {
    Ok(List::cons(f61(v1)?, v0))
}

pub fn f40(v0: u64) -> Adt9 {
    Adt9::C1(false, f62(v0))
}

pub fn f41(v0: List<(u64, u64)>, v1: u64) -> Option<u64> {
    f63(Fn5::F64(v1), None::<u64>, v0)
}

pub fn f42(v0: Adt10, v1: u64) -> R<(u64, Adt10)> {
    Ok((match v0.clone() {
        Adt10::C0(_, h211, _) => {
            h211
        }
    }, Adt10::C0(List::cons((v1, match v0.clone() {
        Adt10::C0(_, h212, _) => {
            h212
        }
    }), match v0.clone() {
        Adt10::C0(h213, _, _) => {
            h213
        }
    }), {
        let a215 = match v0.clone() {
            Adt10::C0(_, h214, _) => {
                h214
            }
        };
        let a216 = 1u64;
        nat_add(a215, a216)?
    }, match v0 {
        Adt10::C0(_, _, h217) => {
            h217
        }
    })))
}

pub fn f43(v0: Adt3) -> bool {
    let m218 = v0;
    match m218 {
        Adt3::C0(_) => {
            false
        }
        Adt3::C1(_) => {
            false
        }
        Adt3::C2(_) => {
            false
        }
        Adt3::C3(_) => {
            false
        }
        Adt3::C4(_, _) => {
            false
        }
        Adt3::C5 => {
            true
        }
        Adt3::C6(_, _) => {
            false
        }
    }
}

pub fn f44(v0: Adt4) -> R<Adt9> {
    let m219 = v0;
    match m219 {
        Adt4::C0(v1) => {
            Ok(Adt9::C4(std::rc::Rc::new(f8(Str::lit("kind"), f19(Str::lit("core")), f8(Str::lit("name"), Adt9::C2(f65(v1)), Adt9::C5)))))
        }
        Adt4::C1(_, v3, v4, v5, v6) => {
            Ok(Adt9::C4(std::rc::Rc::new(f8(Str::lit("kind"), f19(Str::lit("external")), f8(Str::lit("entry"), Adt9::C2(v3), f8(Str::lit("lean_module"), Adt9::C2(v4), f8(Str::lit("lean_name"), Adt9::C2(v5), f8(Str::lit("signature_sha256"), Adt9::C2(f66(v6)?), Adt9::C5))))))))
        }
        Adt4::C2(v7, v8, v9) => {
            Ok(Adt9::C4(std::rc::Rc::new(f8(Str::lit("kind"), f19(Str::lit("document")), f8(Str::lit("module"), Adt9::C2(v7), f8(Str::lit("component"), Adt9::C2(v8), f8(Str::lit("lean_name"), Adt9::C2(v9), Adt9::C5)))))))
        }
        Adt4::C3(_, v11, v12) => {
            Ok(Adt9::C4(std::rc::Rc::new(f8(Str::lit("kind"), f19(Str::lit("defined")), f8(Str::lit("entry"), Adt9::C2(v11), f8(Str::lit("signature_sha256"), Adt9::C2(f66(v12)?), Adt9::C5))))))
        }
    }
}

pub fn f45(v0: List<u64>) -> Adt9 {
    f67(Fn6::F68, Adt9::C5, f69(v0))
}

pub fn f46(v0: u64) -> Adt9 {
    Adt9::C2(f70(v0))
}

pub fn f47(v0: List<u64>, v1: Adt10) -> R<(Adt9, Adt10)> {
    let v2: (List<u64>, Adt10) = f71(Fn7::F72, (List::<u64>::nil(), v1), v0)?;
    Ok((f67(Fn6::F73, f12(), {
        let (h220, _) = v2;
        h220
    }), {
        let (_, h221) = v2;
        h221
    }))
}

pub fn f48(v0: Str, v1: Adt2, v2: Adt10, v3: Adt9) -> R<(Adt9, Adt10)> {
    let v4: (Adt9, Adt10) = f5(v1, v2)?;
    Ok((f8(v0, {
        let (h222, _) = v4;
        h222
    }, v3), {
        let (_, h223) = v4;
        h223
    }))
}

pub fn f49(v0: Option<u64>, v1: Adt10) -> R<(Adt9, Adt10)> {
    let m224 = v0;
    match m224 {
        None => {
            Ok((f19(Str::lit("goal")), v1))
        }
        Some(v2) => {
            let v3: (Adt11, Adt10) = f21(v1, v2)?;
            Ok(({
                let m226 = {
                    let (h225, _) = v3;
                    h225
                };
                match m226 {
                    Adt11::C0(v4) => {
                        f40(v4)
                    }
                    Adt11::C1(v5) => {
                        f40(v5)
                    }
                }
            }, {
                let (_, h227) = v3;
                h227
            }))
        }
    }
}

pub fn f50(v0: List<(u64, Bytes)>) -> List<u64> {
    f69(f74(Fn8::F75, List::<u64>::nil(), v0))
}

pub fn f51(v0: Fn3, p231: Adt9, p232: List<Bytes>) -> Adt9 {
    let m230 = (p231, p232);
    {
        let mut m230 = m230;
        loop {
            let step = {
                let (v1, v2) = m230;
                let m228 = v2;
                match m228.uncons() {
                    None => {
                        Err::<(Adt9, List<Bytes>), Adt9>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt9, List<Bytes>), Adt9>(({
                            let c229: Fn3 = v0.clone();
                            c229.apply(v1, v3)
                        }, v4))
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

pub fn f52(v0: Adt9, v1: Bytes) -> Adt9 {
    Adt9::C6(std::rc::Rc::new(Adt9::C2(v1)), std::rc::Rc::new(v0))
}

pub fn f53(v0: List<Bytes>) -> List<Bytes> {
    f76(Fn9::F77, List::<Bytes>::nil(), v0)
}

pub fn f54(v0: Bytes, v1: u64, v2: u64) -> R<Bytes> {
    let a233 = v2;
    let a234 = 0u64;
    if nat_eq(a233, a234) {
        Ok(f78(v0, 0u64))
    } else {
        let (_, h235) = {
            let (h236, _) = f32(Fn0::F79(v1, v0), v2, (v2, Bytes::lit(&[])))?;
            h236
        };
        Ok(h235)
    }
}

pub fn f55() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f56(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let m240 = {
        let a238 = v0.clone();
        let a239 = {
            let (h237, _) = v1.clone();
            h237
        };
        index_bytes(a238, a239)
    };
    match m240 {
        None => {
            Ok(None::<(u64, Bytes)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a242 = {
                    let (h241, _) = v1.clone();
                    h241
                };
                let a243 = 1u64;
                nat_add(a242, a243)?
            }, {
                let a246 = {
                    let (_, h244) = v1;
                    h244
                };
                let a247 = f80(v0, {
                    let (h245, _) = v1;
                    h245
                }, v2)?;
                append_bytes(a246, a247)
            })))
        }
    }
}

pub fn f57(v0: Fn4, p251: Option<Bytes>, p252: List<Bytes>) -> Option<Bytes> {
    let m250 = (p251, p252);
    {
        let mut m250 = m250;
        loop {
            let step = {
                let (v1, v2) = m250;
                let m248 = v2;
                match m248.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(({
                            let c249: Fn4 = v0.clone();
                            c249.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m250 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f58(v0: Bytes, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m253 = v1;
    match m253 {
        None => {
            Some(v2)
        }
        Some(v3) => {
            Some({
                let a256 = {
                    let a254 = v3;
                    let a255 = v0;
                    append_bytes(a254, a255)
                };
                let a257 = v2;
                append_bytes(a256, a257)
            })
        }
    }
}

pub fn f59(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    let m258 = v0.clone();
    match m258.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, v3)) => {
            let a261 = {
                let (h259, _) = v2;
                h259
            };
            let a262 = {
                let (h260, _) = v1.clone();
                h260
            };
            if equal(a261, a262) {
                List::cons(v1, v3)
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f60(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v1: List<List<(Bytes, Bytes)>> = f81(f82(Fn10::F83, List::<List<(Bytes, Bytes)>>::nil(), v0.clone()));
    let m267 = {
        let (h263, _) = f84(Fn11::F85, {
            let a265 = {
                let a264 = v0;
                length_list(a264)
            };
            let a266 = 1u64;
            nat_add(a265, a266)?
        }, v1)?;
        h263
    };
    match m267.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f61(v0: (Bytes, Bytes)) -> R<Bytes> {
    let a272 = {
        let a269 = f14({
            let (h268, _) = v0;
            h268
        })?;
        let a270 = Bytes::lit(&[0x3a]);
        append_bytes(a269, a270)
    };
    let a273 = {
        let (_, h271) = v0;
        h271
    };
    Ok(append_bytes(a272, a273))
}

pub fn f62(v0: u64) -> u64 {
    let a274 = v0;
    let a275 = 9223372036854775807u64;
    if nat_le(a274, a275) {
        v0
    } else {
        9223372036854775807u64
    }
}

pub fn f63(v0: Fn5, p279: Option<u64>, p280: List<(u64, u64)>) -> Option<u64> {
    let m278 = (p279, p280);
    {
        let mut m278 = m278;
        loop {
            let step = {
                let (v1, v2) = m278;
                let m276 = v2;
                match m276.uncons() {
                    None => {
                        Err::<(Option<u64>, List<(u64, u64)>), Option<u64>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<u64>, List<(u64, u64)>), Option<u64>>(({
                            let c277: Fn5 = v0.clone();
                            c277.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m278 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f64(v0: u64, v1: Option<u64>, v2: (u64, u64)) -> Option<u64> {
    let a282 = {
        let (h281, _) = v2;
        h281
    };
    let a283 = v0;
    if nat_eq(a282, a283) {
        Some({
            let (_, h284) = v2;
            h284
        })
    } else {
        v1
    }
}

pub fn f65(v0: Adt5) -> Bytes {
    let m285 = v0;
    match m285 {
        Adt5::C0 => {
            f29(Str::lit("eq"))
        }
        Adt5::C1 => {
            f29(Str::lit("and"))
        }
        Adt5::C2 => {
            f29(Str::lit("or"))
        }
        Adt5::C3 => {
            f29(Str::lit("not"))
        }
        Adt5::C4 => {
            f29(Str::lit("iff"))
        }
        Adt5::C5 => {
            f29(Str::lit("exists"))
        }
        Adt5::C6 => {
            f29(Str::lit("exists-unique"))
        }
    }
}

pub fn f66(v0: Bytes) -> R<Bytes> {
    let (_, h286) = {
        let (h287, _) = f32(Fn0::F86(v0.clone()), {
            let a288 = v0;
            length_bytes(a288)
        }, (0u64, Bytes::lit(&[])))?;
        h287
    };
    Ok(h286)
}

pub fn f67(v0: Fn6, p292: Adt9, p293: List<u64>) -> Adt9 {
    let m291 = (p292, p293);
    {
        let mut m291 = m291;
        loop {
            let step = {
                let (v1, v2) = m291;
                let m289 = v2;
                match m289.uncons() {
                    None => {
                        Err::<(Adt9, List<u64>), Adt9>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt9, List<u64>), Adt9>(({
                            let c290: Fn6 = v0.clone();
                            c290.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m291 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f68(v0: Adt9, v1: u64) -> Adt9 {
    Adt9::C6(std::rc::Rc::new(f40(v1)), std::rc::Rc::new(v0))
}

pub fn f69(v0: List<u64>) -> List<u64> {
    f87(Fn12::F88, List::<u64>::nil(), v0)
}

pub fn f70(v0: u64) -> Bytes {
    let a294 = v0;
    let a295 = 1u64;
    if nat_eq(a294, a295) {
        f89(Str::lit("implicit"))
    } else {
        let a296 = v0;
        let a297 = 2u64;
        if nat_eq(a296, a297) {
            f89(Str::lit("instance"))
        } else {
            f89(Str::lit("explicit"))
        }
    }
}

pub fn f71(v0: Fn7, p301: (List<u64>, Adt10), p302: List<u64>) -> R<(List<u64>, Adt10)> {
    let m300 = (p301, p302);
    {
        let mut m300 = m300;
        loop {
            let step = {
                let (v1, v2) = m300;
                let m298 = v2;
                match m298.uncons() {
                    None => {
                        Err::<((List<u64>, Adt10), List<u64>), R<(List<u64>, Adt10)>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<((List<u64>, Adt10), List<u64>), R<(List<u64>, Adt10)>>(({
                            let c299: Fn7 = v0.clone();
                            c299.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m300 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f72(v0: (List<u64>, Adt10), v1: u64) -> R<(List<u64>, Adt10)> {
    let v2: (u64, Adt10) = f25({
        let (_, h303) = v0;
        h303
    }, v1)?;
    Ok((List::cons({
        let (h304, _) = v2;
        h304
    }, {
        let (h305, _) = v0;
        h305
    }), {
        let (_, h306) = v2;
        h306
    }))
}

pub fn f73(v0: Adt9, v1: u64) -> Adt9 {
    Adt9::C6(std::rc::Rc::new(f40(v1)), std::rc::Rc::new(v0))
}

pub fn f74(v0: Fn8, p310: List<u64>, p311: List<(u64, Bytes)>) -> List<u64> {
    let m309 = (p310, p311);
    {
        let mut m309 = m309;
        loop {
            let step = {
                let (v1, v2) = m309;
                let m307 = v2;
                match m307.uncons() {
                    None => {
                        Err::<(List<u64>, List<(u64, Bytes)>), List<u64>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<u64>, List<(u64, Bytes)>), List<u64>>(({
                            let c308: Fn8 = v0.clone();
                            c308.apply(v1, v3)
                        }, v4))
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

pub fn f75(v0: List<u64>, v1: (u64, Bytes)) -> List<u64> {
    List::cons({
        let (h312, _) = v1;
        h312
    }, v0)
}

pub fn f76(v0: Fn9, p316: List<Bytes>, p317: List<Bytes>) -> List<Bytes> {
    let m315 = (p316, p317);
    {
        let mut m315 = m315;
        loop {
            let step = {
                let (v1, v2) = m315;
                let m313 = v2;
                match m313.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c314: Fn9 = v0.clone();
                            c314.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m315 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f77(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f78(v0: Bytes, v1: u64) -> Bytes {
    let m321 = {
        let a318 = v0;
        let a319 = v1;
        let a320 = 1u64;
        slice_bytes(a318, a319, a320)
    };
    match m321 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f79(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f90(v1, v0, v2)
}

pub fn f80(v0: Bytes, v1: u64, v2: u8) -> R<Bytes> {
    let v4: u64 = {
        let v3: u8 = v2;
        let a366 = {
            let a342 = {
                let a330 = {
                    let a324 = {
                        let a322 = v3;
                        let a323 = 128u8;
                        fixed_u8::bit_and(a322, a323)
                    };
                    let a325 = 128u8;
                    if equal(a324, a325) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a331 = {
                    let a328 = {
                        let a326 = v3;
                        let a327 = 64u8;
                        fixed_u8::bit_and(a326, a327)
                    };
                    let a329 = 64u8;
                    if equal(a328, a329) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a330, a331)?
            };
            let a343 = {
                let a340 = {
                    let a334 = {
                        let a332 = v3;
                        let a333 = 32u8;
                        fixed_u8::bit_and(a332, a333)
                    };
                    let a335 = 32u8;
                    if equal(a334, a335) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a341 = {
                    let a338 = {
                        let a336 = v3;
                        let a337 = 16u8;
                        fixed_u8::bit_and(a336, a337)
                    };
                    let a339 = 16u8;
                    if equal(a338, a339) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a340, a341)?
            };
            nat_add(a342, a343)?
        };
        let a367 = {
            let a364 = {
                let a352 = {
                    let a346 = {
                        let a344 = v3;
                        let a345 = 8u8;
                        fixed_u8::bit_and(a344, a345)
                    };
                    let a347 = 8u8;
                    if equal(a346, a347) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a353 = {
                    let a350 = {
                        let a348 = v3;
                        let a349 = 4u8;
                        fixed_u8::bit_and(a348, a349)
                    };
                    let a351 = 4u8;
                    if equal(a350, a351) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a352, a353)?
            };
            let a365 = {
                let a362 = {
                    let a356 = {
                        let a354 = v3;
                        let a355 = 2u8;
                        fixed_u8::bit_and(a354, a355)
                    };
                    let a357 = 2u8;
                    if equal(a356, a357) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a363 = {
                    let a360 = {
                        let a358 = v3;
                        let a359 = 1u8;
                        fixed_u8::bit_and(a358, a359)
                    };
                    let a361 = 1u8;
                    if equal(a360, a361) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a362, a363)?
            };
            nat_add(a364, a365)?
        };
        nat_add(a366, a367)?
    };
    let a368 = v4;
    let a369 = 34u64;
    if nat_eq(a368, a369) {
        Ok(Bytes::lit(&[0x5c, 0x22]))
    } else {
        let a370 = v4;
        let a371 = 92u64;
        if nat_eq(a370, a371) {
            Ok(Bytes::lit(&[0x5c, 0x5c]))
        } else {
            let a372 = v4;
            let a373 = 8u64;
            if nat_eq(a372, a373) {
                Ok(Bytes::lit(&[0x5c, 0x62]))
            } else {
                let a374 = v4;
                let a375 = 12u64;
                if nat_eq(a374, a375) {
                    Ok(Bytes::lit(&[0x5c, 0x66]))
                } else {
                    let a376 = v4;
                    let a377 = 10u64;
                    if nat_eq(a376, a377) {
                        Ok(Bytes::lit(&[0x5c, 0x6e]))
                    } else {
                        let a378 = v4;
                        let a379 = 13u64;
                        if nat_eq(a378, a379) {
                            Ok(Bytes::lit(&[0x5c, 0x72]))
                        } else {
                            let a380 = v4;
                            let a381 = 9u64;
                            if nat_eq(a380, a381) {
                                Ok(Bytes::lit(&[0x5c, 0x74]))
                            } else {
                                let a382 = v4;
                                let a383 = 32u64;
                                if nat_lt(a382, a383) {
                                    let a384 = Bytes::lit(&[0x5c, 0x75, 0x30, 0x30]);
                                    let a385 = f91(v2)?;
                                    Ok(append_bytes(a384, a385))
                                } else {
                                    Ok(f92(v0, v1, 1u64))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f81(v0: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    f93(Fn13::F94, List::<List<(Bytes, Bytes)>>::nil(), v0)
}

pub fn f82(v0: Fn10, p389: List<List<(Bytes, Bytes)>>, p390: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    let m388 = (p389, p390);
    {
        let mut m388 = m388;
        loop {
            let step = {
                let (v1, v2) = m388;
                let m386 = v2;
                match m386.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(({
                            let c387: Fn10 = v0.clone();
                            c387.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m388 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f83(v0: List<List<(Bytes, Bytes)>>, v1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
    List::cons(List::cons(v1, List::<(Bytes, Bytes)>::nil()), v0)
}

pub fn f84(v0: Fn11, p395: u64, p396: List<List<(Bytes, Bytes)>>) -> R<(List<List<(Bytes, Bytes)>>, bool)> {
    let m394 = (p395, p396);
    {
        let mut m394 = m394;
        loop {
            let step = {
                let (v1, v2) = m394;
                let m391 = v1;
                if m391 == 0 {
                    Err::<(u64, List<List<(Bytes, Bytes)>>), R<(List<List<(Bytes, Bytes)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m391 - 1;
                    let m393 = {
                        let c392: Fn11 = v0.clone();
                        c392.apply(v2.clone())?
                    };
                    match m393 {
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
                Ok(next) => { m394 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f85(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    f95(v0)
}

pub fn f86(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f96(v0, v1)
}

pub fn f87(v0: Fn12, p400: List<u64>, p401: List<u64>) -> List<u64> {
    let m399 = (p400, p401);
    {
        let mut m399 = m399;
        loop {
            let step = {
                let (v1, v2) = m399;
                let m397 = v2;
                match m397.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>(({
                            let c398: Fn12 = v0.clone();
                            c398.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m399 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f88(v0: List<u64>, v1: u64) -> List<u64> {
    List::cons(v1, v0)
}

pub fn f89(v0: Str) -> Bytes {
    let a402 = v0;
    utf8_encode(a402)
}

pub fn f90(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h403, _) = v2;
        h403
    };
    let a404 = v3;
    let a405 = 0u64;
    if nat_eq(a404, a405) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a406 = v3;
            let a407 = v1;
            let a408 = 0u64;
            nat_quot(a406, a407, a408)
        }, {
            let a413 = f78(v0, {
                let a409 = v3;
                let a410 = v1;
                let a411 = 0u64;
                nat_rem(a409, a410, a411)
            });
            let a414 = {
                let (_, h412) = v2;
                h412
            };
            append_bytes(a413, a414)
        }))
    }
}

pub fn f91(v0: u8) -> R<Bytes> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a459 = {
            let a435 = {
                let a423 = {
                    let a417 = {
                        let a415 = v1;
                        let a416 = 128u8;
                        fixed_u8::bit_and(a415, a416)
                    };
                    let a418 = 128u8;
                    if equal(a417, a418) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a424 = {
                    let a421 = {
                        let a419 = v1;
                        let a420 = 64u8;
                        fixed_u8::bit_and(a419, a420)
                    };
                    let a422 = 64u8;
                    if equal(a421, a422) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a423, a424)?
            };
            let a436 = {
                let a433 = {
                    let a427 = {
                        let a425 = v1;
                        let a426 = 32u8;
                        fixed_u8::bit_and(a425, a426)
                    };
                    let a428 = 32u8;
                    if equal(a427, a428) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a434 = {
                    let a431 = {
                        let a429 = v1;
                        let a430 = 16u8;
                        fixed_u8::bit_and(a429, a430)
                    };
                    let a432 = 16u8;
                    if equal(a431, a432) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a433, a434)?
            };
            nat_add(a435, a436)?
        };
        let a460 = {
            let a457 = {
                let a445 = {
                    let a439 = {
                        let a437 = v1;
                        let a438 = 8u8;
                        fixed_u8::bit_and(a437, a438)
                    };
                    let a440 = 8u8;
                    if equal(a439, a440) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a446 = {
                    let a443 = {
                        let a441 = v1;
                        let a442 = 4u8;
                        fixed_u8::bit_and(a441, a442)
                    };
                    let a444 = 4u8;
                    if equal(a443, a444) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a445, a446)?
            };
            let a458 = {
                let a455 = {
                    let a449 = {
                        let a447 = v1;
                        let a448 = 2u8;
                        fixed_u8::bit_and(a447, a448)
                    };
                    let a450 = 2u8;
                    if equal(a449, a450) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a456 = {
                    let a453 = {
                        let a451 = v1;
                        let a452 = 1u8;
                        fixed_u8::bit_and(a451, a452)
                    };
                    let a454 = 1u8;
                    if equal(a453, a454) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a455, a456)?
            };
            nat_add(a457, a458)?
        };
        nat_add(a459, a460)?
    };
    let a467 = f78(f97(), {
        let a461 = v2;
        let a462 = 16u64;
        let a463 = 0u64;
        nat_quot(a461, a462, a463)
    });
    let a468 = f78(f97(), {
        let a464 = v2;
        let a465 = 16u64;
        let a466 = 0u64;
        nat_rem(a464, a465, a466)
    });
    Ok(append_bytes(a467, a468))
}

pub fn f92(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m472 = {
        let a469 = v0;
        let a470 = v1;
        let a471 = v2;
        slice_bytes(a469, a470, a471)
    };
    match m472 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f93(v0: Fn13, p476: List<List<(Bytes, Bytes)>>, p477: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    let m475 = (p476, p477);
    {
        let mut m475 = m475;
        loop {
            let step = {
                let (v1, v2) = m475;
                let m473 = v2;
                match m473.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(({
                            let c474: Fn13 = v0.clone();
                            c474.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m475 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f94(v0: List<List<(Bytes, Bytes)>>, v1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    List::cons(v1, v0)
}

pub fn f95(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    let a479 = {
        let a478 = v0.clone();
        length_list(a478)
    };
    let a480 = 1u64;
    if nat_le(a479, a480) {
        Ok(None::<List<List<(Bytes, Bytes)>>>)
    } else {
        Ok(Some(f98(v0)?))
    }
}

pub fn f96(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let m484 = {
        let a482 = v0;
        let a483 = {
            let (h481, _) = v1.clone();
            h481
        };
        index_bytes(a482, a483)
    };
    match m484 {
        None => {
            Ok(None::<(u64, Bytes)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a486 = {
                    let (h485, _) = v1;
                    h485
                };
                let a487 = 1u64;
                nat_add(a486, a487)?
            }, {
                let a489 = {
                    let (_, h488) = v1;
                    h488
                };
                let a490 = f91(v2)?;
                append_bytes(a489, a490)
            })))
        }
    }
}

pub fn f97() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66])
}

pub fn f98(v0: List<List<(Bytes, Bytes)>>) -> R<List<List<(Bytes, Bytes)>>> {
    Ok(f81({
        let (_, h491) = {
            let (h492, _) = f99(Fn14::F100, {
                let a494 = {
                    let a493 = v0.clone();
                    length_list(a493)
                };
                let a495 = 1u64;
                nat_add(a494, a495)?
            }, (v0, List::<List<(Bytes, Bytes)>>::nil()))?;
            h492
        };
        h491
    }))
}

pub fn f99(v0: Fn14, p500: u64, p501: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)> {
    let m499 = (p500, p501);
    {
        let mut m499 = m499;
        loop {
            let step = {
                let (v1, v2) = m499;
                let m496 = v1;
                if m496 == 0 {
                    Err::<(u64, (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)), R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m496 - 1;
                    let m498 = {
                        let c497: Fn14 = v0.clone();
                        c497.apply(v2.clone())?
                    };
                    match m498 {
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
                Ok(next) => { m499 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f100(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    f101(v0)
}

pub fn f101(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    let v1: List<List<(Bytes, Bytes)>> = {
        let (h502, _) = v0;
        h502
    };
    let v2: List<List<(Bytes, Bytes)>> = {
        let (_, h503) = v0;
        h503
    };
    let m504 = v1;
    match m504.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>)
        }
        Some((v3, v4)) => {
            let m505 = v4;
            match m505.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Bytes)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f102(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f102(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v2: u64 = {
        let a510 = {
            let a508 = {
                let a506 = v0.clone();
                length_list(a506)
            };
            let a509 = {
                let a507 = v1.clone();
                length_list(a507)
            };
            nat_add(a508, a509)?
        };
        let a511 = 1u64;
        nat_add(a510, a511)?
    };
    Ok(f103({
        let (_, h512) = {
            let (_, h513) = {
                let (h514, _) = f104(Fn15::F105, v2, (v0, (v1, List::<(Bytes, Bytes)>::nil())));
                h514
            };
            h513
        };
        h512
    }))
}

pub fn f103(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f35(Fn1::F106, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f104(v0: Fn15, p519: u64, p520: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool) {
    let m518 = (p519, p520);
    {
        let mut m518 = m518;
        loop {
            let step = {
                let (v1, v2) = m518;
                let m515 = v1;
                if m515 == 0 {
                    Err::<(u64, (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))), ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool)>((v2, false))
                } else {
                    let v3 = m515 - 1;
                    let m517 = {
                        let c516: Fn15 = v0.clone();
                        c516.apply(v2.clone())
                    };
                    match m517 {
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
                Ok(next) => { m518 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f105(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    f107(v0)
}

pub fn f106(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn f107(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    let v1: List<(Bytes, Bytes)> = {
        let (h521, _) = v0.clone();
        h521
    };
    let v2: List<(Bytes, Bytes)> = {
        let (h522, _) = {
            let (_, h523) = v0.clone();
            h523
        };
        h522
    };
    let v3: List<(Bytes, Bytes)> = {
        let (_, h524) = {
            let (_, h525) = v0;
            h525
        };
        h524
    };
    let m526 = v1.clone();
    match m526.uncons() {
        None => {
            let m527 = v2;
            match m527.uncons() {
                None => {
                    None::<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Bytes)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m528 = v2.clone();
            match m528.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Bytes)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f108({
                        let (h529, _) = v8.clone();
                        h529
                    }, {
                        let (h530, _) = v6.clone();
                        h530
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

pub fn f108(v0: Bytes, v1: Bytes) -> bool {
    let a535 = {
        let a531 = v0;
        let a532 = v1;
        compare_bytes(a531, a532)
    };
    let a536 = {
        let a533 = Bytes::lit(&[]);
        let a534 = Bytes::lit(&[0x01]);
        compare_bytes(a533, a534)
    };
    equal(a535, a536)
}

pub fn run(p0: Adt0) -> R<Bytes> {
    f0(p0)
}
