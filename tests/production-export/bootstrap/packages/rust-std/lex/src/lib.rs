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
    C0(Adt1, u64, bool, u64),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(List<Adt2>),
    C1,
    C2,
    C3(Bytes),
    C4(Bytes),
}

#[derive(Clone)]
pub enum Adt2 {
    C0(Bytes, bool),
}

#[derive(Clone)]
pub enum Adt3 {
    C0(u64, u64, bool, u64, List<Adt0>, Option<Bytes>),
}

#[derive(Clone)]
pub enum Adt4 {
    C0(u64, u64, u64, bool),
}

#[derive(Clone)]
pub enum Adt5 {
    C0(u64, u64, bool, bool),
}

#[derive(Clone)]
pub enum Adt6 {
    C0(u64, List<Adt2>, bool, Option<Bytes>),
}

#[derive(Clone)]
pub enum Adt7 {
    C0(u64, bool, bool, bool),
}

#[derive(Clone)]
pub enum Fn0 {
    F2(u64, Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: Adt3) -> R<Option<Adt3>> {
        match self {
            Fn0::F2(k0, k1) => f2(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F6,
}

impl Fn1 {
    pub fn apply(&self, p0: List<Adt0>, p1: Adt0) -> List<Adt0> {
        match self {
            Fn1::F6 => f6(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F19(u64, Bytes),
    F48(Bytes),
    F60(Bytes),
    F66(Bytes),
}

impl Fn2 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn2::F19(k0, k1) => f19(*k0, k1.clone(), p0),
            Fn2::F48(k0) => f48(k0.clone(), p0),
            Fn2::F60(k0) => f60(k0.clone(), p0),
            Fn2::F66(k0) => f66(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F21(u64, Bytes),
}

impl Fn3 {
    pub fn apply(&self, p0: Adt4) -> R<Option<Adt4>> {
        match self {
            Fn3::F21(k0, k1) => f21(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F37(Bytes, u64),
}

impl Fn4 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn4::F37(k0, k1) => f37(k0.clone(), *k1, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F41(u64, Bytes),
    F67(u64, u64, Bytes),
}

impl Fn5 {
    pub fn apply(&self, p0: Adt5) -> R<Option<Adt5>> {
        match self {
            Fn5::F41(k0, k1) => f41(*k0, k1.clone(), p0),
            Fn5::F67(k0, k1, k2) => f67(*k0, *k1, k2.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F57(u64, Bytes),
}

impl Fn6 {
    pub fn apply(&self, p0: (u64, (u64, bool))) -> R<Option<(u64, (u64, bool))>> {
        match self {
            Fn6::F57(k0, k1) => f57(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F59(u64, u64, Bytes),
}

impl Fn7 {
    pub fn apply(&self, p0: (u64, (bool, bool))) -> R<Option<(u64, (bool, bool))>> {
        match self {
            Fn7::F59(k0, k1, k2) => f59(*k0, *k1, k2.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F64(Bytes),
}

impl Fn8 {
    pub fn apply(&self, p0: Adt6) -> R<Option<Adt6>> {
        match self {
            Fn8::F64(k0) => f64(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F75,
}

impl Fn9 {
    pub fn apply(&self, p0: List<Adt2>, p1: Adt2) -> List<Adt2> {
        match self {
            Fn9::F75 => f75(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F83(Bytes),
}

impl Fn10 {
    pub fn apply(&self, p0: Adt7) -> R<Option<Adt7>> {
        match self {
            Fn10::F83(k0) => f83(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F85(u64, u64, Bytes),
}

impl Fn11 {
    pub fn apply(&self, p0: (u64, bool)) -> R<Option<(u64, bool)>> {
        match self {
            Fn11::F85(k0, k1, k2) => f85(*k0, *k1, k2.clone(), p0),
        }
    }
}

pub fn f0(v0: Bytes) -> R<Result<List<Adt0>, Bytes>> {
    let v1: u64 = {
        let a1 = v0.clone();
        length_bytes(a1)
    };
    let v2: Adt3 = {
        let (h2, _) = f1(Fn0::F2(v1, v0), {
            let a3 = v1;
            let a4 = 1u64;
            nat_add(a3, a4)?
        }, Adt3::C0(0u64, 0u64, true, 0u64, List::<Adt0>::nil(), None::<Bytes>))?;
        h2
    };
    let m6 = match v2.clone() {
        Adt3::C0(_, _, _, _, _, h5) => {
            h5
        }
    };
    match m6 {
        None => {
            let v4: List<Adt0> = f3(match v2 {
                Adt3::C0(_, _, _, _, h7, _) => {
                    h7
                }
            });
            Ok(Ok::<List<Adt0>, Bytes>(v4))
        }
        Some(v3) => {
            Ok(Err::<List<Adt0>, Bytes>(v3))
        }
    }
}

pub fn f1(v0: Fn0, p12: u64, p13: Adt3) -> R<(Adt3, bool)> {
    let m11 = (p12, p13);
    {
        let mut m11 = m11;
        loop {
            let step = {
                let (v1, v2) = m11;
                let m8 = v1;
                if m8 == 0 {
                    Err::<(u64, Adt3), R<(Adt3, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m8 - 1;
                    let m10 = {
                        let c9: Fn0 = v0.clone();
                        c9.apply(v2.clone())?
                    };
                    match m10 {
                        None => {
                            Err::<(u64, Adt3), R<(Adt3, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt3), R<(Adt3, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m11 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f2(v0: u64, v1: Bytes, v2: Adt3) -> R<Option<Adt3>> {
    f4(v1, v0, v2)
}

pub fn f3(v0: List<Adt0>) -> List<Adt0> {
    f5(Fn1::F6, List::<Adt0>::nil(), v0)
}

pub fn f4(v0: Bytes, v1: u64, v2: Adt3) -> R<Option<Adt3>> {
    let m15 = match v2.clone() {
        Adt3::C0(_, _, _, _, _, h14) => {
            h14
        }
    };
    match m15 {
        None => {
            let v4: Option<Adt3> = f7(v0, match v2.clone() {
                Adt3::C0(h16, _, _, _, _, _) => {
                    h16
                }
            }, match v2.clone() {
                Adt3::C0(_, h17, _, _, _, _) => {
                    h17
                }
            }, match v2.clone() {
                Adt3::C0(_, _, h18, _, _, _) => {
                    h18
                }
            }, match v2.clone() {
                Adt3::C0(_, _, _, h19, _, _) => {
                    h19
                }
            }, match v2.clone() {
                Adt3::C0(_, _, _, _, h20, _) => {
                    h20
                }
            })?;
            let a22 = match v2 {
                Adt3::C0(h21, _, _, _, _, _) => {
                    h21
                }
            };
            let a23 = v1;
            if nat_lt(a22, a23) {
                Ok(v4)
            } else {
                Ok(None::<Adt3>)
            }
        }
        Some(_) => {
            Ok(None::<Adt3>)
        }
    }
}

pub fn f5(v0: Fn1, p27: List<Adt0>, p28: List<Adt0>) -> List<Adt0> {
    let m26 = (p27, p28);
    {
        let mut m26 = m26;
        loop {
            let step = {
                let (v1, v2) = m26;
                let m24 = v2;
                match m24.uncons() {
                    None => {
                        Err::<(List<Adt0>, List<Adt0>), List<Adt0>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt0>, List<Adt0>), List<Adt0>>(({
                            let c25: Fn1 = v0.clone();
                            c25.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m26 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f6(v0: List<Adt0>, v1: Adt0) -> List<Adt0> {
    List::cons(v1, v0)
}

pub fn f7(v0: Bytes, v1: u64, v2: u64, v3: bool, v4: u64, v5: List<Adt0>) -> R<Option<Adt3>> {
    let v6: u8 = f8(v0.clone(), v1);
    let v7: bool = f9(v0.clone(), {
        let a29 = v1;
        let a30 = 1u64;
        nat_add(a29, a30)?
    }, 45u8);
    let v8: u64 = f10(v0.clone(), v1)?;
    let v9: Adt4 = f11(v0.clone(), {
        let a31 = v1;
        let a32 = 2u64;
        nat_add(a31, a32)?
    }, v2)?;
    let v10: Adt3 = f12(v0.clone(), v6, v1, v2, v3, v4, v5.clone())?;
    let a33 = v6;
    let a34 = 10u8;
    if equal(a33, a34) {
        Ok(Some(Adt3::C0({
            let a35 = v1;
            let a36 = 1u64;
            nat_add(a35, a36)?
        }, {
            let a37 = v2;
            let a38 = 1u64;
            nat_add(a37, a38)?
        }, true, {
            let a39 = v4;
            let a40 = 1u64;
            nat_add(a39, a40)?
        }, v5, None::<Bytes>)))
    } else {
        let a41 = v6;
        let a42 = 32u8;
        if if equal(a41, a42) {
            true
        } else {
            let a43 = v6;
            let a44 = 9u8;
            if equal(a43, a44) {
                true
            } else {
                let a45 = v6;
                let a46 = 13u8;
                equal(a45, a46)
            }
        } {
            Ok(Some(Adt3::C0({
                let a47 = v1;
                let a48 = 1u64;
                nat_add(a47, a48)?
            }, v2, v3, {
                let a49 = v4;
                let a50 = 1u64;
                nat_add(a49, a50)?
            }, v5, None::<Bytes>)))
        } else {
            let m51 = f13(v0.clone(), v1)?;
            match m51 {
                None => {
                    let a52 = v6;
                    let a53 = 45u8;
                    if if equal(a52, a53) {
                        v7
                    } else {
                        false
                    } {
                        Ok(Some(Adt3::C0(v8, v2, v3, {
                            let a54 = v4;
                            let a55 = f16(v0, v1, v8)?;
                            nat_add(a54, a55)?
                        }, v5, None::<Bytes>)))
                    } else {
                        let a56 = v6;
                        let a57 = 47u8;
                        if if equal(a56, a57) {
                            v7
                        } else {
                            false
                        } {
                            if match v9.clone() {
                                Adt4::C0(_, _, _, h58) => {
                                    h58
                                }
                            } {
                                Ok(Some(f15(v1, v2, v3, v4, v5, {
                                    let a59 = Str::lit("a comment is not closed");
                                    utf8_encode(a59)
                                })))
                            } else {
                                Ok(Some(Adt3::C0(match v9.clone() {
                                    Adt4::C0(h60, _, _, _) => {
                                        h60
                                    }
                                }, match v9.clone() {
                                    Adt4::C0(_, h61, _, _) => {
                                        h61
                                    }
                                }, v3, {
                                    let a63 = v4;
                                    let a64 = f16(v0, v1, match v9 {
                                        Adt4::C0(h62, _, _, _) => {
                                            h62
                                        }
                                    })?;
                                    nat_add(a63, a64)?
                                }, v5, None::<Bytes>)))
                            }
                        } else {
                            Ok(Some(v10))
                        }
                    }
                }
                Some(v11) => {
                    let v12: Bytes = f14(v11);
                    Ok(Some(f15(v1, v2, v3, v4, v5, v12)))
                }
            }
        }
    }
}

pub fn f8(v0: Bytes, v1: u64) -> u8 {
    let m65 = f17(v0, v1);
    match m65 {
        None => {
            0u8
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f9(v0: Bytes, v1: u64, v2: u8) -> bool {
    let m66 = f17(v0, v1);
    match m66 {
        None => {
            false
        }
        Some(v3) => {
            let a67 = v3;
            let a68 = v2;
            equal(a67, a68)
        }
    }
}

pub fn f10(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = {
        let a69 = v0.clone();
        length_bytes(a69)
    };
    let (h70, _) = f18(Fn2::F19(v2, v0), {
        let a71 = v2;
        let a72 = v1;
        nat_sub(a71, a72)
    }, v1)?;
    Ok(h70)
}

pub fn f11(v0: Bytes, v1: u64, v2: u64) -> R<Adt4> {
    let v3: u64 = {
        let a73 = v0.clone();
        length_bytes(a73)
    };
    let (h74, _) = f20(Fn3::F21(v3, v0), {
        let a77 = {
            let a75 = v3;
            let a76 = v1;
            nat_sub(a75, a76)
        };
        let a78 = 1u64;
        nat_add(a77, a78)?
    }, Adt4::C0(v1, v2, 1u64, false))?;
    Ok(h74)
}

pub fn f12(v0: Bytes, v1: u8, v2: u64, v3: u64, v4: bool, v5: u64, v6: List<Adt0>) -> R<Adt3> {
    let v7: Adt5 = f22(v0.clone(), {
        let a79 = v2;
        let a80 = 1u64;
        nat_add(a79, a80)?
    }, v3)?;
    let v8: Adt3 = if match v7.clone() {
        Adt5::C0(_, _, _, h81) => {
            h81
        }
    } {
        f15(v2, v3, false, v5, v6.clone(), {
            let a82 = Str::lit("a string is not closed");
            utf8_encode(a82)
        })
    } else {
        f23(match v7.clone() {
            Adt5::C0(h83, _, _, _) => {
                h83
            }
        }, match v7.clone() {
            Adt5::C0(_, h84, _, _) => {
                h84
            }
        }, v5, f16(v0.clone(), v2, match v7 {
            Adt5::C0(h85, _, _, _) => {
                h85
            }
        })?, Adt1::C2, v3, v4, v6.clone())?
    };
    let v10: Adt3 = {
        let m86 = f24(v0.clone(), v2)?;
        match m86 {
            None => {
                f15(v2, v3, false, v5, v6.clone(), {
                    let a87 = Str::lit("a character literal is not closed");
                    utf8_encode(a87)
                })
            }
            Some(v9) => {
                f23(v9, v3, v5, f16(v0.clone(), v2, v9)?, Adt1::C2, v3, v4, v6.clone())?
            }
        }
    };
    let v13: Adt3 = {
        let m88 = f25(v0.clone(), v2)?;
        match m88 {
            Ok(v12) => {
                f23(v12, v3, v5, f16(v0.clone(), v2, v12)?, Adt1::C1, v3, v4, v6.clone())?
            }
            Err(v11) => {
                f15(v2, v3, false, v5, v6.clone(), v11)
            }
        }
    };
    let v14: u64 = f26(v0.clone(), {
        let a89 = v2;
        let a90 = 1u64;
        nat_add(a89, a90)?
    })?;
    let v15: u64 = f27(v0.clone(), {
        let a93 = {
            let a91 = v2;
            let a92 = 1u64;
            nat_add(a91, a92)?
        };
        let a94 = v14;
        nat_add(a93, a94)?
    })?;
    let v16: Adt3 = f23(v15, v3, v5, f16(v0.clone(), v2, v15)?, Adt1::C3(f28(v0.clone(), v2, {
        let a95 = v15;
        let a96 = v2;
        nat_sub(a95, a96)
    })), v3, v4, v6.clone())?;
    let v17: bool = f29(v0.clone(), v2)?;
    let v18: Adt3 = f30(v0.clone(), v2, v3, v4, v5, v6.clone())?;
    let v19: u64 = f31(v0.clone(), v2);
    let v20: Adt3 = f23({
        let a97 = v2;
        let a98 = v19;
        nat_add(a97, a98)?
    }, v3, v5, 1u64, Adt1::C4(f28(v0, v2, v19)), v3, v4, v6)?;
    let v21: bool = f32(v1);
    let a99 = v1;
    let a100 = 34u8;
    if equal(a99, a100) {
        Ok(v8)
    } else {
        let a101 = v1;
        let a102 = 39u8;
        if equal(a101, a102) {
            Ok(v10)
        } else {
            if v21 {
                Ok(v13)
            } else {
                let a103 = v1;
                let a104 = 35u8;
                if if equal(a103, a104) {
                    let a105 = 0u64;
                    let a106 = v14;
                    nat_lt(a105, a106)
                } else {
                    false
                } {
                    Ok(v16)
                } else {
                    if v17 {
                        Ok(v18)
                    } else {
                        Ok(v20)
                    }
                }
            }
        }
    }
}

pub fn f13(v0: Bytes, v1: u64) -> R<Option<u8>> {
    let a108 = v1;
    let a109 = {
        let a107 = v0.clone();
        length_bytes(a107)
    };
    if nat_lt(a108, a109) {
        let v2: u8 = f8(v0.clone(), v1);
        if f33(v2, 0u8, 31u8) {
            Ok(Some(v2))
        } else {
            let a110 = v2;
            let a111 = 127u8;
            if equal(a110, a111) {
                Ok(Some(v2))
            } else {
                let a112 = v2;
                let a113 = 194u8;
                if equal(a112, a113) {
                    if f34(v0.clone(), {
                        let a114 = v1;
                        let a115 = 1u64;
                        nat_add(a114, a115)?
                    }, 128u8, 159u8) {
                        Ok(Some(f8(v0, {
                            let a116 = v1;
                            let a117 = 1u64;
                            nat_add(a116, a117)?
                        })))
                    } else {
                        Ok(None::<u8>)
                    }
                } else {
                    Ok(None::<u8>)
                }
            }
        }
    } else {
        Ok(None::<u8>)
    }
}

pub fn f14(v0: u8) -> Bytes {
    let v2: u8 = {
        let m120 = {
            let a118 = v0;
            let a119 = 4u32;
            fixed_u8::shift_right(a118, a119)
        };
        match m120 {
            None => {
                0u8
            }
            Some(v1) => {
                v1
            }
        }
    };
    let v3: u8 = {
        let a121 = v0;
        let a122 = 15u8;
        fixed_u8::bit_and(a121, a122)
    };
    let v4: Bytes = f35(v2);
    let v5: Bytes = f35(v3);
    let a129 = {
        let a126 = {
            let a123 = Str::lit("a control character U+00");
            utf8_encode(a123)
        };
        let a127 = {
            let a124 = v4;
            let a125 = v5;
            append_bytes(a124, a125)
        };
        append_bytes(a126, a127)
    };
    let a130 = {
        let a128 = Str::lit(" in the text");
        utf8_encode(a128)
    };
    append_bytes(a129, a130)
}

pub fn f15(v0: u64, v1: u64, v2: bool, v3: u64, v4: List<Adt0>, v5: Bytes) -> Adt3 {
    Adt3::C0(v0, v1, v2, v3, v4, Some(v5))
}

pub fn f16(v0: Bytes, v1: u64, v2: u64) -> R<u64> {
    let (_, h131) = {
        let (h132, _) = f36(Fn4::F37(v0, v2), {
            let a133 = v2;
            let a134 = v1;
            nat_sub(a133, a134)
        }, (v1, 0u64))?;
        h132
    };
    Ok(h131)
}

pub fn f17(v0: Bytes, v1: u64) -> Option<u8> {
    let a135 = v0;
    let a136 = v1;
    index_bytes(a135, a136)
}

pub fn f18(v0: Fn2, p141: u64, p142: u64) -> R<(u64, bool)> {
    let m140 = (p141, p142);
    {
        let mut m140 = m140;
        loop {
            let step = {
                let (v1, v2) = m140;
                let m137 = v1;
                if m137 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m137 - 1;
                    let m139 = {
                        let c138: Fn2 = v0.clone();
                        c138.apply(v2)?
                    };
                    match m139 {
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
                Ok(next) => { m140 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f19(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f38(v1, v0, v2)
}

pub fn f20(v0: Fn3, p147: u64, p148: Adt4) -> R<(Adt4, bool)> {
    let m146 = (p147, p148);
    {
        let mut m146 = m146;
        loop {
            let step = {
                let (v1, v2) = m146;
                let m143 = v1;
                if m143 == 0 {
                    Err::<(u64, Adt4), R<(Adt4, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m143 - 1;
                    let m145 = {
                        let c144: Fn3 = v0.clone();
                        c144.apply(v2.clone())?
                    };
                    match m145 {
                        None => {
                            Err::<(u64, Adt4), R<(Adt4, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt4), R<(Adt4, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m146 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f21(v0: u64, v1: Bytes, v2: Adt4) -> R<Option<Adt4>> {
    f39(v1, v0, v2)
}

pub fn f22(v0: Bytes, v1: u64, v2: u64) -> R<Adt5> {
    let v3: u64 = {
        let a149 = v0.clone();
        length_bytes(a149)
    };
    let (h150, _) = f40(Fn5::F41(v3, v0), {
        let a153 = {
            let a151 = v3;
            let a152 = v1;
            nat_sub(a151, a152)
        };
        let a154 = 1u64;
        nat_add(a153, a154)?
    }, Adt5::C0(v1, v2, false, false))?;
    Ok(h150)
}

pub fn f23(v0: u64, v1: u64, v2: u64, v3: u64, v4: Adt1, v5: u64, v6: bool, v7: List<Adt0>) -> R<Adt3> {
    Ok(Adt3::C0(v0, v1, false, {
        let a155 = v2;
        let a156 = v3;
        nat_add(a155, a156)?
    }, List::cons(Adt0::C0(v4, v5, v6, v2), v7), None::<Bytes>))
}

pub fn f24(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = {
        let a157 = v0.clone();
        length_bytes(a157)
    };
    let v3: u64 = {
        let a158 = v1;
        let a159 = 1u64;
        nat_add(a158, a159)?
    };
    let v4: u8 = f8(v0.clone(), v3);
    let v5: u64 = {
        let a160 = v3;
        let a161 = 1u64;
        nat_add(a160, a161)?
    };
    let v6: u8 = f8(v0.clone(), v5);
    let v7: u64 = {
        let a162 = v5;
        let a163 = f31(v0.clone(), v5);
        nat_add(a162, a163)?
    };
    let v8: Option<u64> = f42(v0.clone(), v7, 2u64)?;
    let v9: Option<u64> = f42(v0.clone(), v7, 4u64)?;
    let v10: u64 = f31(v0.clone(), v3);
    let v11: Option<u64> = {
        let a164 = v3;
        let a165 = v2;
        if nat_lt(a164, a165) {
            let a166 = v4;
            let a167 = 92u8;
            if equal(a166, a167) {
                let a168 = v5;
                let a169 = v2;
                if nat_lt(a168, a169) {
                    let a170 = v6;
                    let a171 = 120u8;
                    if equal(a170, a171) {
                        v8
                    } else {
                        let a172 = v6;
                        let a173 = 117u8;
                        if equal(a172, a173) {
                            v9
                        } else {
                            Some(v7)
                        }
                    }
                } else {
                    None::<u64>
                }
            } else {
                let a174 = v4;
                let a175 = 39u8;
                if equal(a174, a175) {
                    None::<u64>
                } else {
                    let a176 = v4;
                    let a177 = 10u8;
                    if equal(a176, a177) {
                        None::<u64>
                    } else {
                        Some({
                            let a178 = v3;
                            let a179 = v10;
                            nat_add(a178, a179)?
                        })
                    }
                }
            }
        } else {
            None::<u64>
        }
    };
    let m180 = v11;
    match m180 {
        None => {
            Ok(None::<u64>)
        }
        Some(v12) => {
            let v13: bool = f9(v0, v12, 39u8);
            if v13 {
                Ok(Some({
                    let a181 = v12;
                    let a182 = 1u64;
                    nat_add(a181, a182)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f25(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let v2: u8 = f8(v0.clone(), v1);
    let v3: u8 = f8(v0.clone(), {
        let a183 = v1;
        let a184 = 1u64;
        nat_add(a183, a184)?
    });
    let v4: u64 = {
        let a185 = v2;
        let a186 = 48u8;
        if equal(a185, a186) {
            let a187 = v3;
            let a188 = 120u8;
            if equal(a187, a188) {
                16u64
            } else {
                let a189 = v3;
                let a190 = 88u8;
                if equal(a189, a190) {
                    16u64
                } else {
                    let a191 = v3;
                    let a192 = 98u8;
                    if equal(a191, a192) {
                        2u64
                    } else {
                        let a193 = v3;
                        let a194 = 66u8;
                        if equal(a193, a194) {
                            2u64
                        } else {
                            let a195 = v3;
                            let a196 = 111u8;
                            if equal(a195, a196) {
                                8u64
                            } else {
                                let a197 = v3;
                                let a198 = 79u8;
                                if equal(a197, a198) {
                                    8u64
                                } else {
                                    10u64
                                }
                            }
                        }
                    }
                }
            }
        } else {
            10u64
        }
    };
    let v5: Result<u64, Bytes> = f43(v0.clone(), {
        let a199 = v1;
        let a200 = 2u64;
        nat_add(a199, a200)?
    }, v4, true)?;
    let v6: u64 = f44(v0.clone(), v1)?;
    let v9: Result<u64, Bytes> = {
        let m203 = f43(v0.clone(), {
            let a201 = v1;
            let a202 = 1u64;
            nat_add(a201, a202)?
        }, 10u64, false)?;
        match m203 {
            Ok(v8) => {
                f45(v0.clone(), v8)?
            }
            Err(v7) => {
                Err::<u64, Bytes>(v7)
            }
        }
    };
    let a206 = {
        let a204 = v4;
        let a205 = 10u64;
        nat_eq(a204, a205)
    };
    if bool_not(a206) {
        Ok(v5)
    } else {
        let a209 = {
            let a207 = v2;
            let a208 = 48u8;
            equal(a207, a208)
        };
        if if bool_not(a209) {
            let a210 = 0u64;
            let a211 = v1;
            if nat_lt(a210, a211) {
                f9(v0.clone(), {
                    let a212 = v1;
                    let a213 = 1u64;
                    nat_sub(a212, a213)
                }, 46u8)
            } else {
                false
            }
        } else {
            false
        } {
            if if f9(v0.clone(), v6, 95u8) {
                true
            } else {
                if f9(v0.clone(), v6, 101u8) {
                    true
                } else {
                    f9(v0, v6, 69u8)
                }
            } {
                Ok(Err::<u64, Bytes>({
                    let a214 = Str::lit("a field index is followed by what could continue a number");
                    utf8_encode(a214)
                }))
            } else {
                Ok(Ok::<u64, Bytes>(v6))
            }
        } else {
            Ok(v9)
        }
    }
}

pub fn f26(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u8 = f8(v0.clone(), v1);
    let v3: bool = f46(v2);
    let v4: bool = {
        let a215 = v2;
        let a216 = 95u8;
        equal(a215, a216)
    };
    let v5: u64 = f47(v0.clone(), v1)?;
    let a218 = v1;
    let a219 = {
        let a217 = v0;
        length_bytes(a217)
    };
    if nat_lt(a218, a219) {
        if v3 {
            Ok(1u64)
        } else {
            if v4 {
                Ok(1u64)
            } else {
                Ok(v5)
            }
        }
    } else {
        Ok(0u64)
    }
}

pub fn f27(v0: Bytes, v1: u64) -> R<u64> {
    let (h220, _) = f18(Fn2::F48(v0.clone()), {
        let a222 = {
            let a221 = v0;
            length_bytes(a221)
        };
        let a223 = v1;
        nat_sub(a222, a223)
    }, v1)?;
    Ok(h220)
}

pub fn f28(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m227 = {
        let a224 = v0;
        let a225 = v1;
        let a226 = v2;
        slice_bytes(a224, a225, a226)
    };
    match m227 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f29(v0: Bytes, v1: u64) -> R<bool> {
    let v2: u64 = f26(v0.clone(), v1)?;
    let v3: bool = f49(v0, v1)?;
    let a228 = 0u64;
    let a229 = v2;
    if nat_lt(a228, a229) {
        Ok(true)
    } else {
        Ok(v3)
    }
}

pub fn f30(v0: Bytes, v1: u64, v2: u64, v3: bool, v4: u64, v5: List<Adt0>) -> R<Adt3> {
    let m230 = f50(v0.clone(), v1)?;
    match m230 {
        Ok(v7) => {
            let m232 = {
                let (h231, _) = v7.clone();
                h231
            };
            match m232.uncons() {
                None => {
                    Ok(Adt3::C0({
                        let (_, h233) = v7;
                        h233
                    }, v2, false, v4, v5, None::<Bytes>))
                }
                Some((v8, v9)) => {
                    let m234 = v9;
                    match m234.uncons() {
                        None => {
                            let v10: u64 = f51(v0.clone(), {
                                let (_, h235) = v7.clone();
                                h235
                            })?;
                            let v11: Adt5 = f52(v0.clone(), {
                                let a239 = {
                                    let a237 = {
                                        let (_, h236) = v7.clone();
                                        h236
                                    };
                                    let a238 = v10;
                                    nat_add(a237, a238)?
                                };
                                let a240 = 1u64;
                                nat_add(a239, a240)?
                            }, v2, v10)?;
                            let v12: Adt3 = if match v11.clone() {
                                Adt5::C0(_, _, _, h241) => {
                                    h241
                                }
                            } {
                                f15(v1, v2, false, v4, v5.clone(), {
                                    let a242 = Str::lit("a raw string is not closed");
                                    utf8_encode(a242)
                                })
                            } else {
                                f23(match v11.clone() {
                                    Adt5::C0(h243, _, _, _) => {
                                        h243
                                    }
                                }, match v11.clone() {
                                    Adt5::C0(_, h244, _, _) => {
                                        h244
                                    }
                                }, v4, f16(v0.clone(), v1, match v11 {
                                    Adt5::C0(h245, _, _, _) => {
                                        h245
                                    }
                                })?, Adt1::C2, v2, v3, v5.clone())?
                            };
                            let v13: Adt3 = f15(v1, v2, false, v4, v5.clone(), {
                                let a246 = Str::lit("an interpolated string holds code");
                                utf8_encode(a246)
                            });
                            let v14: Adt3 = f23({
                                let (_, h247) = v7.clone();
                                h247
                            }, v2, v4, f16(v0.clone(), v1, {
                                let (_, h248) = v7.clone();
                                h248
                            })?, Adt1::C0({
                                let (h249, _) = v7.clone();
                                h249
                            }), v2, v3, v5)?;
                            let v15: bool = f53(match v8.clone() {
                                Adt2::C0(h250, _) => {
                                    h250
                                }
                            }, {
                                let a251 = Str::lit("!");
                                utf8_encode(a251)
                            });
                            let v16: bool = f9(v0.clone(), {
                                let (_, h252) = v7.clone();
                                h252
                            }, 34u8);
                            let v17: bool = f9(v0, {
                                let a254 = {
                                    let (_, h253) = v7;
                                    h253
                                };
                                let a255 = v10;
                                nat_add(a254, a255)?
                            }, 34u8);
                            let a257 = match v8.clone() {
                                Adt2::C0(_, h256) => {
                                    h256
                                }
                            };
                            if if bool_not(a257) {
                                let a260 = match v8.clone() {
                                    Adt2::C0(h258, _) => {
                                        h258
                                    }
                                };
                                let a261 = {
                                    let a259 = Str::lit("r");
                                    utf8_encode(a259)
                                };
                                if equal(a260, a261) {
                                    v17
                                } else {
                                    false
                                }
                            } else {
                                false
                            } {
                                Ok(v12)
                            } else {
                                let a263 = match v8 {
                                    Adt2::C0(_, h262) => {
                                        h262
                                    }
                                };
                                if if bool_not(a263) {
                                    if v15 {
                                        v16
                                    } else {
                                        false
                                    }
                                } else {
                                    false
                                } {
                                    Ok(v13)
                                } else {
                                    Ok(v14)
                                }
                            }
                        }
                        Some((_, _)) => {
                            f23({
                                let (_, h264) = v7.clone();
                                h264
                            }, v2, v4, f16(v0, v1, {
                                let (_, h265) = v7;
                                h265
                            })?, Adt1::C0({
                                let (h266, _) = v7;
                                h266
                            }), v2, v3, v5)
                        }
                    }
                }
            }
        }
        Err(v6) => {
            Ok(f15(v1, v2, false, v4, v5, v6))
        }
    }
}

pub fn f31(v0: Bytes, v1: u64) -> u64 {
    let v2: u8 = f8(v0, v1);
    let v3: bool = f33(v2, 0u8, 127u8);
    if v3 {
        1u64
    } else {
        let v4: bool = f33(v2, 128u8, 223u8);
        if v4 {
            2u64
        } else {
            let v5: bool = f33(v2, 224u8, 239u8);
            if v5 {
                3u64
            } else {
                4u64
            }
        }
    }
}

pub fn f32(v0: u8) -> bool {
    f33(v0, 48u8, 57u8)
}

pub fn f33(v0: u8, v1: u8, v2: u8) -> bool {
    let m269 = {
        let a267 = v0;
        let a268 = v1;
        fixed_u8::checked_sub(a267, a268)
    };
    match m269 {
        None => {
            false
        }
        Some(_) => {
            let m272 = {
                let a270 = v2;
                let a271 = v0;
                fixed_u8::checked_sub(a270, a271)
            };
            match m272 {
                None => {
                    false
                }
                Some(_) => {
                    true
                }
            }
        }
    }
}

pub fn f34(v0: Bytes, v1: u64, v2: u8, v3: u8) -> bool {
    let m273 = f17(v0, v1);
    match m273 {
        None => {
            false
        }
        Some(v4) => {
            f33(v4, v2, v3)
        }
    }
}

pub fn f35(v0: u8) -> Bytes {
    let v1: Bytes = {
        let a274 = Str::lit("0123456789ABCDEF");
        utf8_encode(a274)
    };
    let v2: u64 = {
        let a275 = v0;
        let a276 = 0u8;
        if equal(a275, a276) {
            0u64
        } else {
            let a277 = v0;
            let a278 = 1u8;
            if equal(a277, a278) {
                1u64
            } else {
                let a279 = v0;
                let a280 = 2u8;
                if equal(a279, a280) {
                    2u64
                } else {
                    let a281 = v0;
                    let a282 = 3u8;
                    if equal(a281, a282) {
                        3u64
                    } else {
                        let a283 = v0;
                        let a284 = 4u8;
                        if equal(a283, a284) {
                            4u64
                        } else {
                            let a285 = v0;
                            let a286 = 5u8;
                            if equal(a285, a286) {
                                5u64
                            } else {
                                let a287 = v0;
                                let a288 = 6u8;
                                if equal(a287, a288) {
                                    6u64
                                } else {
                                    let a289 = v0;
                                    let a290 = 7u8;
                                    if equal(a289, a290) {
                                        7u64
                                    } else {
                                        let a291 = v0;
                                        let a292 = 8u8;
                                        if equal(a291, a292) {
                                            8u64
                                        } else {
                                            let a293 = v0;
                                            let a294 = 9u8;
                                            if equal(a293, a294) {
                                                9u64
                                            } else {
                                                let a295 = v0;
                                                let a296 = 10u8;
                                                if equal(a295, a296) {
                                                    10u64
                                                } else {
                                                    let a297 = v0;
                                                    let a298 = 11u8;
                                                    if equal(a297, a298) {
                                                        11u64
                                                    } else {
                                                        let a299 = v0;
                                                        let a300 = 12u8;
                                                        if equal(a299, a300) {
                                                            12u64
                                                        } else {
                                                            let a301 = v0;
                                                            let a302 = 13u8;
                                                            if equal(a301, a302) {
                                                                13u64
                                                            } else {
                                                                let a303 = v0;
                                                                let a304 = 14u8;
                                                                if equal(a303, a304) {
                                                                    14u64
                                                                } else {
                                                                    15u64
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    };
    f28(v1, v2, 1u64)
}

pub fn f36(v0: Fn4, p309: u64, p310: (u64, u64)) -> R<((u64, u64), bool)> {
    let m308 = (p309, p310);
    {
        let mut m308 = m308;
        loop {
            let step = {
                let (v1, v2) = m308;
                let m305 = v1;
                if m305 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m305 - 1;
                    let m307 = {
                        let c306: Fn4 = v0.clone();
                        c306.apply(v2)?
                    };
                    match m307 {
                        None => {
                            Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (u64, u64)), R<((u64, u64), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m308 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f37(v0: Bytes, v1: u64, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    f54(v0, v1, v2)
}

pub fn f38(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let a311 = v2;
    let a312 = v1;
    if nat_lt(a311, a312) {
        let v3: bool = f9(v0, v2, 10u8);
        if v3 {
            Ok(None::<u64>)
        } else {
            Ok(Some({
                let a313 = v2;
                let a314 = 1u64;
                nat_add(a313, a314)?
            }))
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f39(v0: Bytes, v1: u64, v2: Adt4) -> R<Option<Adt4>> {
    let v3: u64 = match v2.clone() {
        Adt4::C0(h315, _, _, _) => {
            h315
        }
    };
    let v4: u64 = match v2.clone() {
        Adt4::C0(_, h316, _, _) => {
            h316
        }
    };
    let v5: u64 = match v2.clone() {
        Adt4::C0(_, _, h317, _) => {
            h317
        }
    };
    let a318 = v5;
    let a319 = 0u64;
    if nat_eq(a318, a319) {
        Ok(None::<Adt4>)
    } else {
        if match v2 {
            Adt4::C0(_, _, _, h320) => {
                h320
            }
        } {
            Ok(None::<Adt4>)
        } else {
            let a321 = v3;
            let a322 = v1;
            if nat_lt(a321, a322) {
                let v6: u8 = f8(v0.clone(), v3);
                let a323 = v6;
                let a324 = 47u8;
                if if equal(a323, a324) {
                    f9(v0.clone(), {
                        let a325 = v3;
                        let a326 = 1u64;
                        nat_add(a325, a326)?
                    }, 45u8)
                } else {
                    false
                } {
                    Ok(Some(Adt4::C0({
                        let a327 = v3;
                        let a328 = 2u64;
                        nat_add(a327, a328)?
                    }, v4, {
                        let a329 = v5;
                        let a330 = 1u64;
                        nat_add(a329, a330)?
                    }, false)))
                } else {
                    let a331 = v6;
                    let a332 = 45u8;
                    if if equal(a331, a332) {
                        f9(v0.clone(), {
                            let a333 = v3;
                            let a334 = 1u64;
                            nat_add(a333, a334)?
                        }, 47u8)
                    } else {
                        false
                    } {
                        Ok(Some(Adt4::C0({
                            let a335 = v3;
                            let a336 = 2u64;
                            nat_add(a335, a336)?
                        }, v4, {
                            let a337 = v5;
                            let a338 = 1u64;
                            nat_sub(a337, a338)
                        }, false)))
                    } else {
                        let a339 = v6;
                        let a340 = 10u8;
                        if equal(a339, a340) {
                            Ok(Some(Adt4::C0({
                                let a341 = v3;
                                let a342 = 1u64;
                                nat_add(a341, a342)?
                            }, {
                                let a343 = v4;
                                let a344 = 1u64;
                                nat_add(a343, a344)?
                            }, v5, false)))
                        } else {
                            Ok(Some(Adt4::C0({
                                let a345 = v3;
                                let a346 = f31(v0, v3);
                                nat_add(a345, a346)?
                            }, v4, v5, false)))
                        }
                    }
                }
            } else {
                Ok(Some(Adt4::C0(v3, v4, v5, true)))
            }
        }
    }
}

pub fn f40(v0: Fn5, p351: u64, p352: Adt5) -> R<(Adt5, bool)> {
    let m350 = (p351, p352);
    {
        let mut m350 = m350;
        loop {
            let step = {
                let (v1, v2) = m350;
                let m347 = v1;
                if m347 == 0 {
                    Err::<(u64, Adt5), R<(Adt5, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m347 - 1;
                    let m349 = {
                        let c348: Fn5 = v0.clone();
                        c348.apply(v2.clone())?
                    };
                    match m349 {
                        None => {
                            Err::<(u64, Adt5), R<(Adt5, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt5), R<(Adt5, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m350 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f41(v0: u64, v1: Bytes, v2: Adt5) -> R<Option<Adt5>> {
    f55(v1, v0, v2)
}

pub fn f42(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let v3: (u64, (u64, bool)) = {
        let (h353, _) = f56(Fn6::F57(v2, v0), v2, (v1, (0u64, true)))?;
        h353
    };
    let (_, h354) = {
        let (_, h355) = v3;
        h355
    };
    if h354 {
        Ok(Some({
            let (h356, _) = v3;
            h356
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f43(v0: Bytes, v1: u64, v2: u64, v3: bool) -> R<Result<u64, Bytes>> {
    let v4: u64 = {
        let a357 = v0.clone();
        length_bytes(a357)
    };
    let v5: (u64, (bool, bool)) = {
        let (h358, _) = f58(Fn7::F59(v4, v2, v0), {
            let a361 = {
                let a359 = v4;
                let a360 = v1;
                nat_sub(a359, a360)
            };
            let a362 = 1u64;
            nat_add(a361, a362)?
        }, (v1, (v3, false)))?;
        h358
    };
    let (h363, _) = {
        let (_, h364) = v5;
        h364
    };
    if h363 {
        Ok(Err::<u64, Bytes>({
            let a365 = Str::lit("a number has a digit separator or a radix prefix with no digit after it");
            utf8_encode(a365)
        }))
    } else {
        Ok(Ok::<u64, Bytes>({
            let (h366, _) = v5;
            h366
        }))
    }
}

pub fn f44(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = {
        let a367 = v0.clone();
        length_bytes(a367)
    };
    let (h368, _) = f18(Fn2::F60(v0), {
        let a369 = v2;
        let a370 = v1;
        nat_sub(a369, a370)
    }, v1)?;
    Ok(h368)
}

pub fn f45(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let v2: bool = f9(v0.clone(), v1, 46u8);
    let v3: bool = f9(v0.clone(), {
        let a371 = v1;
        let a372 = 1u64;
        nat_add(a371, a372)?
    }, 46u8);
    let v4: bool = f34(v0.clone(), {
        let a373 = v1;
        let a374 = 1u64;
        nat_add(a373, a374)?
    }, 48u8, 57u8);
    let v5: bool = f9(v0.clone(), v1, 101u8);
    let v6: bool = f9(v0.clone(), v1, 69u8);
    let v7: Result<u64, Bytes> = f61(v0.clone(), {
        let a375 = v1;
        let a376 = 1u64;
        nat_add(a375, a376)?
    }, true)?;
    let v8: Result<u64, Bytes> = f61(v0.clone(), v1, false)?;
    let v11: Result<u64, Bytes> = {
        let m379 = f43(v0.clone(), {
            let a377 = v1;
            let a378 = 1u64;
            nat_add(a377, a378)?
        }, 10u64, false)?;
        match m379 {
            Ok(v10) => {
                f61(v0, v10, false)?
            }
            Err(v9) => {
                Err::<u64, Bytes>(v9)
            }
        }
    };
    if if v2 {
        v3
    } else {
        false
    } {
        Ok(Ok::<u64, Bytes>(v1))
    } else {
        if v2 {
            if v4 {
                Ok(v11)
            } else {
                Ok(v7)
            }
        } else {
            if if v5 {
                true
            } else {
                v6
            } {
                Ok(v8)
            } else {
                Ok(Ok::<u64, Bytes>(v1))
            }
        }
    }
}

pub fn f46(v0: u8) -> bool {
    let v1: bool = f33(v0, 97u8, 122u8);
    let v2: bool = f33(v0, 65u8, 90u8);
    if v1 {
        true
    } else {
        v2
    }
}

pub fn f47(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u8 = f8(v0.clone(), v1);
    let v3: u8 = f8(v0.clone(), {
        let a380 = v1;
        let a381 = 1u64;
        nat_add(a380, a381)?
    });
    let v4: u8 = f8(v0.clone(), {
        let a382 = v1;
        let a383 = 2u64;
        nat_add(a382, a383)?
    });
    let v5: u8 = f8(v0, {
        let a384 = v1;
        let a385 = 3u64;
        nat_add(a384, a385)?
    });
    let a386 = v2;
    let a387 = 206u8;
    if equal(a386, a387) {
        if f33(v3, 145u8, 169u8) {
            let a388 = v3;
            let a389 = 160u8;
            if equal(a388, a389) {
                Ok(0u64)
            } else {
                let a390 = v3;
                let a391 = 163u8;
                if equal(a390, a391) {
                    Ok(0u64)
                } else {
                    Ok(2u64)
                }
            }
        } else {
            if f33(v3, 177u8, 191u8) {
                let a392 = v3;
                let a393 = 187u8;
                if equal(a392, a393) {
                    Ok(0u64)
                } else {
                    Ok(2u64)
                }
            } else {
                Ok(0u64)
            }
        }
    } else {
        let a394 = v2;
        let a395 = 207u8;
        if equal(a394, a395) {
            if f33(v3, 128u8, 187u8) {
                Ok(2u64)
            } else {
                Ok(0u64)
            }
        } else {
            let a396 = v2;
            let a397 = 225u8;
            if equal(a396, a397) {
                if f33(v3, 188u8, 191u8) {
                    if f33(v4, 128u8, 191u8) {
                        let a398 = v3;
                        let a399 = 191u8;
                        if equal(a398, a399) {
                            let a400 = v4;
                            let a401 = 191u8;
                            if equal(a400, a401) {
                                Ok(0u64)
                            } else {
                                Ok(3u64)
                            }
                        } else {
                            Ok(3u64)
                        }
                    } else {
                        Ok(0u64)
                    }
                } else {
                    Ok(0u64)
                }
            } else {
                let a402 = v2;
                let a403 = 226u8;
                if equal(a402, a403) {
                    let a404 = v3;
                    let a405 = 132u8;
                    if equal(a404, a405) {
                        if f33(v4, 128u8, 191u8) {
                            Ok(3u64)
                        } else {
                            Ok(0u64)
                        }
                    } else {
                        let a406 = v3;
                        let a407 = 133u8;
                        if equal(a406, a407) {
                            if f33(v4, 128u8, 143u8) {
                                Ok(3u64)
                            } else {
                                Ok(0u64)
                            }
                        } else {
                            Ok(0u64)
                        }
                    }
                } else {
                    let a408 = v2;
                    let a409 = 240u8;
                    if equal(a408, a409) {
                        let a410 = v3;
                        let a411 = 157u8;
                        if equal(a410, a411) {
                            let a412 = v4;
                            let a413 = 146u8;
                            if equal(a412, a413) {
                                if f33(v5, 156u8, 191u8) {
                                    Ok(4u64)
                                } else {
                                    Ok(0u64)
                                }
                            } else {
                                if f33(v4, 147u8, 149u8) {
                                    if f33(v5, 128u8, 191u8) {
                                        Ok(4u64)
                                    } else {
                                        Ok(0u64)
                                    }
                                } else {
                                    let a414 = v4;
                                    let a415 = 150u8;
                                    if equal(a414, a415) {
                                        if f33(v5, 128u8, 159u8) {
                                            Ok(4u64)
                                        } else {
                                            Ok(0u64)
                                        }
                                    } else {
                                        Ok(0u64)
                                    }
                                }
                            }
                        } else {
                            Ok(0u64)
                        }
                    } else {
                        Ok(0u64)
                    }
                }
            }
        }
    }
}

pub fn f48(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f62(v0, v1)
}

pub fn f49(v0: Bytes, v1: u64) -> R<bool> {
    let v2: bool = f9(v0.clone(), v1, 194u8);
    let v3: bool = f9(v0, {
        let a416 = v1;
        let a417 = 1u64;
        nat_add(a416, a417)?
    }, 171u8);
    if v2 {
        Ok(v3)
    } else {
        Ok(false)
    }
}

pub fn f50(v0: Bytes, v1: u64) -> R<Result<(List<Adt2>, u64), Bytes>> {
    let v2: u64 = {
        let a418 = v0.clone();
        length_bytes(a418)
    };
    let v3: Adt6 = {
        let (h419, _) = f63(Fn8::F64(v0), {
            let a422 = {
                let a420 = v2;
                let a421 = v1;
                nat_sub(a420, a421)
            };
            let a423 = 1u64;
            nat_add(a422, a423)?
        }, Adt6::C0(v1, List::<Adt2>::nil(), false, None::<Bytes>))?;
        h419
    };
    let m425 = match v3.clone() {
        Adt6::C0(_, _, _, h424) => {
            h424
        }
    };
    match m425 {
        None => {
            let v5: List<Adt2> = f65(match v3.clone() {
                Adt6::C0(_, h426, _, _) => {
                    h426
                }
            });
            Ok(Ok::<(List<Adt2>, u64), Bytes>((v5, match v3 {
                Adt6::C0(h427, _, _, _) => {
                    h427
                }
            })))
        }
        Some(v4) => {
            Ok(Err::<(List<Adt2>, u64), Bytes>(v4))
        }
    }
}

pub fn f51(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = {
        let a428 = v0.clone();
        length_bytes(a428)
    };
    let v3: u64 = {
        let (h429, _) = f18(Fn2::F66(v0), {
            let a430 = v2;
            let a431 = v1;
            nat_sub(a430, a431)
        }, v1)?;
        h429
    };
    let a432 = v3;
    let a433 = v1;
    Ok(nat_sub(a432, a433))
}

pub fn f52(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt5> {
    let v4: u64 = {
        let a434 = v0.clone();
        length_bytes(a434)
    };
    let (h435, _) = f40(Fn5::F67(v3, v4, v0), {
        let a438 = {
            let a436 = v4;
            let a437 = v1;
            nat_sub(a436, a437)
        };
        let a439 = 1u64;
        nat_add(a438, a439)?
    }, Adt5::C0(v1, v2, false, false))?;
    Ok(h435)
}

pub fn f53(v0: Bytes, v1: Bytes) -> bool {
    let v2: u64 = {
        let a440 = v0.clone();
        length_bytes(a440)
    };
    let v3: u64 = {
        let a441 = v1.clone();
        length_bytes(a441)
    };
    let a442 = v3;
    let a443 = v2;
    if nat_le(a442, a443) {
        let m449 = {
            let a446 = v0;
            let a447 = {
                let a444 = v2;
                let a445 = v3;
                nat_sub(a444, a445)
            };
            let a448 = v3;
            slice_bytes(a446, a447, a448)
        };
        match m449 {
            None => {
                false
            }
            Some(v4) => {
                let a450 = v4;
                let a451 = v1;
                equal(a450, a451)
            }
        }
    } else {
        false
    }
}

pub fn f54(v0: Bytes, v1: u64, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v3: u64 = {
        let (h452, _) = v2;
        h452
    };
    let v4: u64 = {
        let (_, h453) = v2;
        h453
    };
    let a454 = v3;
    let a455 = v1;
    if nat_lt(a454, a455) {
        let v5: u64 = f31(v0, v3);
        Ok(Some(({
            let a456 = v3;
            let a457 = v5;
            nat_add(a456, a457)?
        }, {
            let a458 = v4;
            let a459 = 1u64;
            nat_add(a458, a459)?
        })))
    } else {
        Ok(None::<(u64, u64)>)
    }
}

pub fn f55(v0: Bytes, v1: u64, v2: Adt5) -> R<Option<Adt5>> {
    let v3: u64 = match v2.clone() {
        Adt5::C0(h460, _, _, _) => {
            h460
        }
    };
    let v4: u64 = match v2.clone() {
        Adt5::C0(_, h461, _, _) => {
            h461
        }
    };
    if match v2.clone() {
        Adt5::C0(_, _, h462, _) => {
            h462
        }
    } {
        Ok(None::<Adt5>)
    } else {
        if match v2 {
            Adt5::C0(_, _, _, h463) => {
                h463
            }
        } {
            Ok(None::<Adt5>)
        } else {
            let a464 = v3;
            let a465 = v1;
            if nat_lt(a464, a465) {
                let v5: u8 = f8(v0.clone(), v3);
                let a466 = v5;
                let a467 = 92u8;
                if equal(a466, a467) {
                    let a470 = {
                        let a468 = v3;
                        let a469 = 1u64;
                        nat_add(a468, a469)?
                    };
                    let a471 = v1;
                    if nat_lt(a470, a471) {
                        let v6: u64 = f31(v0, {
                            let a472 = v3;
                            let a473 = 1u64;
                            nat_add(a472, a473)?
                        });
                        Ok(Some(Adt5::C0({
                            let a476 = {
                                let a474 = v3;
                                let a475 = 1u64;
                                nat_add(a474, a475)?
                            };
                            let a477 = v6;
                            nat_add(a476, a477)?
                        }, v4, false, false)))
                    } else {
                        Ok(Some(Adt5::C0(v1, v4, false, true)))
                    }
                } else {
                    let a478 = v5;
                    let a479 = 34u8;
                    if equal(a478, a479) {
                        Ok(Some(Adt5::C0({
                            let a480 = v3;
                            let a481 = 1u64;
                            nat_add(a480, a481)?
                        }, v4, true, false)))
                    } else {
                        let a482 = v5;
                        let a483 = 10u8;
                        if equal(a482, a483) {
                            Ok(Some(Adt5::C0({
                                let a484 = v3;
                                let a485 = 1u64;
                                nat_add(a484, a485)?
                            }, {
                                let a486 = v4;
                                let a487 = 1u64;
                                nat_add(a486, a487)?
                            }, false, false)))
                        } else {
                            let v7: u64 = f31(v0, v3);
                            Ok(Some(Adt5::C0({
                                let a488 = v3;
                                let a489 = v7;
                                nat_add(a488, a489)?
                            }, v4, false, false)))
                        }
                    }
                }
            } else {
                Ok(Some(Adt5::C0(v3, v4, false, true)))
            }
        }
    }
}

pub fn f56(v0: Fn6, p494: u64, p495: (u64, (u64, bool))) -> R<((u64, (u64, bool)), bool)> {
    let m493 = (p494, p495);
    {
        let mut m493 = m493;
        loop {
            let step = {
                let (v1, v2) = m493;
                let m490 = v1;
                if m490 == 0 {
                    Err::<(u64, (u64, (u64, bool))), R<((u64, (u64, bool)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m490 - 1;
                    let m492 = {
                        let c491: Fn6 = v0.clone();
                        c491.apply(v2)?
                    };
                    match m492 {
                        None => {
                            Err::<(u64, (u64, (u64, bool))), R<((u64, (u64, bool)), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (u64, (u64, bool))), R<((u64, (u64, bool)), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m493 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f57(v0: u64, v1: Bytes, v2: (u64, (u64, bool))) -> R<Option<(u64, (u64, bool))>> {
    f68(v1.clone(), {
        let a496 = v1;
        length_bytes(a496)
    }, v0, v2)
}

pub fn f58(v0: Fn7, p501: u64, p502: (u64, (bool, bool))) -> R<((u64, (bool, bool)), bool)> {
    let m500 = (p501, p502);
    {
        let mut m500 = m500;
        loop {
            let step = {
                let (v1, v2) = m500;
                let m497 = v1;
                if m497 == 0 {
                    Err::<(u64, (u64, (bool, bool))), R<((u64, (bool, bool)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m497 - 1;
                    let m499 = {
                        let c498: Fn7 = v0.clone();
                        c498.apply(v2)?
                    };
                    match m499 {
                        None => {
                            Err::<(u64, (u64, (bool, bool))), R<((u64, (bool, bool)), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (u64, (bool, bool))), R<((u64, (bool, bool)), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m500 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f59(v0: u64, v1: u64, v2: Bytes, v3: (u64, (bool, bool))) -> R<Option<(u64, (bool, bool))>> {
    f69(v2, v0, v1, v3)
}

pub fn f60(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f70(v0, v1)
}

pub fn f61(v0: Bytes, v1: u64, v2: bool) -> R<Result<u64, Bytes>> {
    let v3: bool = f9(v0.clone(), v1, 101u8);
    let v4: bool = f9(v0.clone(), v1, 69u8);
    let v5: Result<u64, Bytes> = f71(v0.clone(), v1)?;
    let v6: bool = f29(v0, v1)?;
    if if v3 {
        true
    } else {
        v4
    } {
        Ok(v5)
    } else {
        if if v2 {
            v6
        } else {
            false
        } {
            Ok(Err::<u64, Bytes>({
                let a503 = Str::lit("an identifier follows a decimal point");
                utf8_encode(a503)
            }))
        } else {
            Ok(Ok::<u64, Bytes>(v1))
        }
    }
}

pub fn f62(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f72(v0, v1)?;
    let a504 = 0u64;
    let a505 = v2;
    if nat_lt(a504, a505) {
        Ok(Some({
            let a506 = v1;
            let a507 = v2;
            nat_add(a506, a507)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f63(v0: Fn8, p512: u64, p513: Adt6) -> R<(Adt6, bool)> {
    let m511 = (p512, p513);
    {
        let mut m511 = m511;
        loop {
            let step = {
                let (v1, v2) = m511;
                let m508 = v1;
                if m508 == 0 {
                    Err::<(u64, Adt6), R<(Adt6, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m508 - 1;
                    let m510 = {
                        let c509: Fn8 = v0.clone();
                        c509.apply(v2.clone())?
                    };
                    match m510 {
                        None => {
                            Err::<(u64, Adt6), R<(Adt6, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt6), R<(Adt6, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m511 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f64(v0: Bytes, v1: Adt6) -> R<Option<Adt6>> {
    f73(v0, v1)
}

pub fn f65(v0: List<Adt2>) -> List<Adt2> {
    f74(Fn9::F75, List::<Adt2>::nil(), v0)
}

pub fn f66(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f76(v0, v1)
}

pub fn f67(v0: u64, v1: u64, v2: Bytes, v3: Adt5) -> R<Option<Adt5>> {
    f77(v2, v1, v0, v3)
}

pub fn f68(v0: Bytes, v1: u64, v2: u64, v3: (u64, (u64, bool))) -> R<Option<(u64, (u64, bool))>> {
    let v4: u64 = {
        let (h514, _) = v3;
        h514
    };
    let v5: u64 = {
        let (h515, _) = {
            let (_, h516) = v3;
            h516
        };
        h515
    };
    let v6: bool = {
        let (_, h517) = {
            let (_, h518) = v3;
            h518
        };
        h517
    };
    if v6 {
        let a519 = v5;
        let a520 = v2;
        if nat_lt(a519, a520) {
            let a521 = v4;
            let a522 = v1;
            if nat_lt(a521, a522) {
                let v7: u64 = f31(v0, v4);
                Ok(Some(({
                    let a523 = v4;
                    let a524 = v7;
                    nat_add(a523, a524)?
                }, ({
                    let a525 = v5;
                    let a526 = 1u64;
                    nat_add(a525, a526)?
                }, true))))
            } else {
                Ok(Some((v4, ({
                    let a527 = v5;
                    let a528 = 1u64;
                    nat_add(a527, a528)?
                }, false))))
            }
        } else {
            Ok(None::<(u64, (u64, bool))>)
        }
    } else {
        Ok(None::<(u64, (u64, bool))>)
    }
}

pub fn f69(v0: Bytes, v1: u64, v2: u64, v3: (u64, (bool, bool))) -> R<Option<(u64, (bool, bool))>> {
    let v4: u64 = {
        let (h529, _) = v3;
        h529
    };
    let v5: bool = {
        let (h530, _) = {
            let (_, h531) = v3;
            h531
        };
        h530
    };
    let v6: bool = {
        let (_, h532) = {
            let (_, h533) = v3;
            h533
        };
        h532
    };
    if v6 {
        Ok(None::<(u64, (bool, bool))>)
    } else {
        let a534 = v4;
        let a535 = v1;
        if nat_lt(a534, a535) {
            let v7: u8 = f8(v0, v4);
            let a536 = v7;
            let a537 = 95u8;
            if equal(a536, a537) {
                Ok(Some(({
                    let a538 = v4;
                    let a539 = 1u64;
                    nat_add(a538, a539)?
                }, (true, false))))
            } else {
                let v8: bool = f78(v7, v2);
                if v8 {
                    Ok(Some(({
                        let a540 = v4;
                        let a541 = 1u64;
                        nat_add(a540, a541)?
                    }, (false, false))))
                } else {
                    Ok(Some((v4, (v5, true))))
                }
            }
        } else {
            Ok(Some((v4, (v5, true))))
        }
    }
}

pub fn f70(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: bool = f34(v0, v1, 48u8, 57u8);
    if v2 {
        Ok(Some({
            let a542 = v1;
            let a543 = 1u64;
            nat_add(a542, a543)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f71(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let v2: u64 = {
        let a544 = v1;
        let a545 = 1u64;
        nat_add(a544, a545)?
    };
    let v3: bool = f9(v0.clone(), v2, 43u8);
    let v4: bool = f9(v0.clone(), v2, 45u8);
    let v5: u64 = if v3 {
        let a546 = v2;
        let a547 = 1u64;
        nat_add(a546, a547)?
    } else {
        if v4 {
            let a548 = v2;
            let a549 = 1u64;
            nat_add(a548, a549)?
        } else {
            v2
        }
    };
    let v6: bool = f34(v0.clone(), v5, 48u8, 57u8);
    let v7: Result<u64, Bytes> = f43(v0, v5, 10u64, false)?;
    if v6 {
        Ok(v7)
    } else {
        Ok(Err::<u64, Bytes>({
            let a550 = Str::lit("a scientific literal has no exponent digits");
            utf8_encode(a550)
        }))
    }
}

pub fn f72(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u8 = f8(v0.clone(), v1);
    let v3: bool = f46(v2);
    let v4: bool = f32(v2);
    let v5: u64 = f47(v0.clone(), v1)?;
    let v6: u64 = f79(v0.clone(), v1)?;
    let a552 = v1;
    let a553 = {
        let a551 = v0;
        length_bytes(a551)
    };
    if nat_lt(a552, a553) {
        if v3 {
            Ok(1u64)
        } else {
            if v4 {
                Ok(1u64)
            } else {
                let a554 = v2;
                let a555 = 95u8;
                if equal(a554, a555) {
                    Ok(1u64)
                } else {
                    let a556 = v2;
                    let a557 = 39u8;
                    if equal(a556, a557) {
                        Ok(1u64)
                    } else {
                        let a558 = v2;
                        let a559 = 33u8;
                        if equal(a558, a559) {
                            Ok(1u64)
                        } else {
                            let a560 = v2;
                            let a561 = 63u8;
                            if equal(a560, a561) {
                                Ok(1u64)
                            } else {
                                let a562 = 0u64;
                                let a563 = v5;
                                if nat_lt(a562, a563) {
                                    Ok(v5)
                                } else {
                                    Ok(v6)
                                }
                            }
                        }
                    }
                }
            }
        }
    } else {
        Ok(0u64)
    }
}

pub fn f73(v0: Bytes, v1: Adt6) -> R<Option<Adt6>> {
    let v2: bool = f49(v0.clone(), match v1.clone() {
        Adt6::C0(h564, _, _, _) => {
            h564
        }
    })?;
    let v3: Result<(Adt2, u64), Bytes> = f80(v0.clone(), match v1.clone() {
        Adt6::C0(h565, _, _, _) => {
            h565
        }
    })?;
    let v4: u64 = f27(v0.clone(), {
        let a568 = match v1.clone() {
            Adt6::C0(h566, _, _, _) => {
                h566
            }
        };
        let a569 = f26(v0.clone(), match v1.clone() {
            Adt6::C0(h567, _, _, _) => {
                h567
            }
        })?;
        nat_add(a568, a569)?
    })?;
    let v5: Result<(Adt2, u64), Bytes> = Ok::<(Adt2, u64), Bytes>((Adt2::C0(f28(v0.clone(), match v1.clone() {
        Adt6::C0(h570, _, _, _) => {
            h570
        }
    }, {
        let a572 = v4;
        let a573 = match v1.clone() {
            Adt6::C0(h571, _, _, _) => {
                h571
            }
        };
        nat_sub(a572, a573)
    }), false), v4));
    if match v1.clone() {
        Adt6::C0(_, _, h574, _) => {
            h574
        }
    } {
        Ok(None::<Adt6>)
    } else {
        let m575 = if v2 {
            v3
        } else {
            v5
        };
        match m575 {
            Ok(v7) => {
                let v8: bool = f9(v0.clone(), {
                    let (_, h578) = v7.clone();
                    h578
                }, 46u8);
                let v9: bool = f29(v0, {
                    let a580 = {
                        let (_, h579) = v7.clone();
                        h579
                    };
                    let a581 = 1u64;
                    nat_add(a580, a581)?
                })?;
                if if v8 {
                    v9
                } else {
                    false
                } {
                    Ok(Some(Adt6::C0({
                        let a583 = {
                            let (_, h582) = v7;
                            h582
                        };
                        let a584 = 1u64;
                        nat_add(a583, a584)?
                    }, List::cons({
                        let (h585, _) = v7;
                        h585
                    }, match v1 {
                        Adt6::C0(_, h586, _, _) => {
                            h586
                        }
                    }), false, None::<Bytes>)))
                } else {
                    Ok(Some(Adt6::C0({
                        let (_, h587) = v7;
                        h587
                    }, List::cons({
                        let (h588, _) = v7;
                        h588
                    }, match v1 {
                        Adt6::C0(_, h589, _, _) => {
                            h589
                        }
                    }), true, None::<Bytes>)))
                }
            }
            Err(v6) => {
                Ok(Some(Adt6::C0(match v1.clone() {
                    Adt6::C0(h576, _, _, _) => {
                        h576
                    }
                }, match v1 {
                    Adt6::C0(_, h577, _, _) => {
                        h577
                    }
                }, true, Some(v6))))
            }
        }
    }
}

pub fn f74(v0: Fn9, p593: List<Adt2>, p594: List<Adt2>) -> List<Adt2> {
    let m592 = (p593, p594);
    {
        let mut m592 = m592;
        loop {
            let step = {
                let (v1, v2) = m592;
                let m590 = v2;
                match m590.uncons() {
                    None => {
                        Err::<(List<Adt2>, List<Adt2>), List<Adt2>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt2>, List<Adt2>), List<Adt2>>(({
                            let c591: Fn9 = v0.clone();
                            c591.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m592 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f75(v0: List<Adt2>, v1: Adt2) -> List<Adt2> {
    List::cons(v1, v0)
}

pub fn f76(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: bool = f9(v0, v1, 35u8);
    if v2 {
        Ok(Some({
            let a595 = v1;
            let a596 = 1u64;
            nat_add(a595, a596)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f77(v0: Bytes, v1: u64, v2: u64, v3: Adt5) -> R<Option<Adt5>> {
    let v4: u64 = match v3.clone() {
        Adt5::C0(h597, _, _, _) => {
            h597
        }
    };
    let v5: u64 = match v3.clone() {
        Adt5::C0(_, h598, _, _) => {
            h598
        }
    };
    if match v3.clone() {
        Adt5::C0(_, _, h599, _) => {
            h599
        }
    } {
        Ok(None::<Adt5>)
    } else {
        if match v3 {
            Adt5::C0(_, _, _, h600) => {
                h600
            }
        } {
            Ok(None::<Adt5>)
        } else {
            let a601 = v4;
            let a602 = v1;
            if nat_lt(a601, a602) {
                let v6: u8 = f8(v0.clone(), v4);
                let a603 = v6;
                let a604 = 34u8;
                if equal(a603, a604) {
                    let v7: bool = f81(v0, {
                        let a605 = v4;
                        let a606 = 1u64;
                        nat_add(a605, a606)?
                    }, v2)?;
                    if v7 {
                        Ok(Some(Adt5::C0({
                            let a609 = {
                                let a607 = v4;
                                let a608 = 1u64;
                                nat_add(a607, a608)?
                            };
                            let a610 = v2;
                            nat_add(a609, a610)?
                        }, v5, true, false)))
                    } else {
                        Ok(Some(Adt5::C0({
                            let a611 = v4;
                            let a612 = 1u64;
                            nat_add(a611, a612)?
                        }, v5, false, false)))
                    }
                } else {
                    let a613 = v6;
                    let a614 = 10u8;
                    if equal(a613, a614) {
                        Ok(Some(Adt5::C0({
                            let a615 = v4;
                            let a616 = 1u64;
                            nat_add(a615, a616)?
                        }, {
                            let a617 = v5;
                            let a618 = 1u64;
                            nat_add(a617, a618)?
                        }, false, false)))
                    } else {
                        let v8: u64 = f31(v0, v4);
                        Ok(Some(Adt5::C0({
                            let a619 = v4;
                            let a620 = v8;
                            nat_add(a619, a620)?
                        }, v5, false, false)))
                    }
                }
            } else {
                Ok(Some(Adt5::C0(v4, v5, false, true)))
            }
        }
    }
}

pub fn f78(v0: u8, v1: u64) -> bool {
    let v2: bool = f33(v0, 48u8, 49u8);
    let v3: bool = f33(v0, 48u8, 55u8);
    let v4: bool = f33(v0, 48u8, 57u8);
    let v5: bool = f33(v0, 97u8, 102u8);
    let v6: bool = f33(v0, 65u8, 70u8);
    let a621 = v1;
    let a622 = 2u64;
    if nat_eq(a621, a622) {
        v2
    } else {
        let a623 = v1;
        let a624 = 8u64;
        if nat_eq(a623, a624) {
            v3
        } else {
            let a625 = v1;
            let a626 = 16u64;
            if nat_eq(a625, a626) {
                if v4 {
                    true
                } else {
                    if v5 {
                        true
                    } else {
                        v6
                    }
                }
            } else {
                v4
            }
        }
    }
}

pub fn f79(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u8 = f8(v0.clone(), v1);
    let v3: u8 = f8(v0.clone(), {
        let a627 = v1;
        let a628 = 1u64;
        nat_add(a627, a628)?
    });
    let v4: u8 = f8(v0, {
        let a629 = v1;
        let a630 = 2u64;
        nat_add(a629, a630)?
    });
    let a631 = v2;
    let a632 = 225u8;
    if equal(a631, a632) {
        let a633 = v3;
        let a634 = 181u8;
        if equal(a633, a634) {
            if f33(v4, 162u8, 170u8) {
                Ok(3u64)
            } else {
                Ok(0u64)
            }
        } else {
            Ok(0u64)
        }
    } else {
        let a635 = v2;
        let a636 = 226u8;
        if equal(a635, a636) {
            let a637 = v3;
            let a638 = 130u8;
            if equal(a637, a638) {
                if f33(v4, 128u8, 137u8) {
                    Ok(3u64)
                } else {
                    if f33(v4, 144u8, 156u8) {
                        Ok(3u64)
                    } else {
                        Ok(0u64)
                    }
                }
            } else {
                Ok(0u64)
            }
        } else {
            Ok(0u64)
        }
    }
}

pub fn f80(v0: Bytes, v1: u64) -> R<Result<(Adt2, u64), Bytes>> {
    let v2: u64 = {
        let a639 = v1;
        let a640 = 2u64;
        nat_add(a639, a640)?
    };
    let v3: Adt7 = {
        let (h641, _) = f82(Fn10::F83(v0.clone()), {
            let a645 = {
                let a643 = {
                    let a642 = v0.clone();
                    length_bytes(a642)
                };
                let a644 = v2;
                nat_sub(a643, a644)
            };
            let a646 = 1u64;
            nat_add(a645, a646)?
        }, Adt7::C0(v2, false, false, true))?;
        h641
    };
    let v4: Bytes = f28(v0, v2, {
        let a648 = match v3.clone() {
            Adt7::C0(h647, _, _, _) => {
                h647
            }
        };
        let a649 = v2;
        nat_sub(a648, a649)
    });
    if match v3.clone() {
        Adt7::C0(_, h650, _, _) => {
            h650
        }
    } {
        let a652 = 0u64;
        let a653 = {
            let a651 = v4.clone();
            length_bytes(a651)
        };
        if if nat_lt(a652, a653) {
            match v3.clone() {
                Adt7::C0(_, _, _, h654) => {
                    h654
                }
            }
        } else {
            false
        } {
            Ok(Ok::<(Adt2, u64), Bytes>((Adt2::C0(v4, true), {
                let a656 = match v3 {
                    Adt7::C0(h655, _, _, _) => {
                        h655
                    }
                };
                let a657 = 2u64;
                nat_add(a656, a657)?
            })))
        } else {
            Ok(Err::<(Adt2, u64), Bytes>({
                let a662 = {
                    let a659 = {
                        let a658 = Str::lit("the quoted name `");
                        utf8_encode(a658)
                    };
                    let a660 = v4;
                    append_bytes(a659, a660)
                };
                let a663 = {
                    let a661 = Str::lit("` is not a name");
                    utf8_encode(a661)
                };
                append_bytes(a662, a663)
            }))
        }
    } else {
        Ok(Err::<(Adt2, u64), Bytes>({
            let a664 = Str::lit("a quoted name is not closed");
            utf8_encode(a664)
        }))
    }
}

pub fn f81(v0: Bytes, v1: u64, v2: u64) -> R<bool> {
    let (_, h665) = {
        let (h666, _) = f84(Fn11::F85(v2, v1, v0), v2, (0u64, true))?;
        h666
    };
    Ok(h665)
}

pub fn f82(v0: Fn10, p671: u64, p672: Adt7) -> R<(Adt7, bool)> {
    let m670 = (p671, p672);
    {
        let mut m670 = m670;
        loop {
            let step = {
                let (v1, v2) = m670;
                let m667 = v1;
                if m667 == 0 {
                    Err::<(u64, Adt7), R<(Adt7, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m667 - 1;
                    let m669 = {
                        let c668: Fn10 = v0.clone();
                        c668.apply(v2.clone())?
                    };
                    match m669 {
                        None => {
                            Err::<(u64, Adt7), R<(Adt7, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt7), R<(Adt7, bool)>>((v3, v4))
                        }
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

pub fn f83(v0: Bytes, v1: Adt7) -> R<Option<Adt7>> {
    f86(v0.clone(), {
        let a673 = v0;
        length_bytes(a673)
    }, v1)
}

pub fn f84(v0: Fn11, p678: u64, p679: (u64, bool)) -> R<((u64, bool), bool)> {
    let m677 = (p678, p679);
    {
        let mut m677 = m677;
        loop {
            let step = {
                let (v1, v2) = m677;
                let m674 = v1;
                if m674 == 0 {
                    Err::<(u64, (u64, bool)), R<((u64, bool), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m674 - 1;
                    let m676 = {
                        let c675: Fn11 = v0.clone();
                        c675.apply(v2)?
                    };
                    match m676 {
                        None => {
                            Err::<(u64, (u64, bool)), R<((u64, bool), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (u64, bool)), R<((u64, bool), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m677 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f85(v0: u64, v1: u64, v2: Bytes, v3: (u64, bool)) -> R<Option<(u64, bool)>> {
    f87(v2, v1, v0, v3)
}

pub fn f86(v0: Bytes, v1: u64, v2: Adt7) -> R<Option<Adt7>> {
    let v3: u64 = match v2.clone() {
        Adt7::C0(h680, _, _, _) => {
            h680
        }
    };
    let v4: bool = match v2.clone() {
        Adt7::C0(_, _, _, h681) => {
            h681
        }
    };
    if match v2.clone() {
        Adt7::C0(_, h682, _, _) => {
            h682
        }
    } {
        Ok(None::<Adt7>)
    } else {
        if match v2 {
            Adt7::C0(_, _, h683, _) => {
                h683
            }
        } {
            Ok(None::<Adt7>)
        } else {
            let a684 = v3;
            let a685 = v1;
            if nat_lt(a684, a685) {
                let v5: bool = f9(v0.clone(), v3, 10u8);
                if v5 {
                    Ok(Some(Adt7::C0(v3, false, true, v4)))
                } else {
                    let v6: bool = f88(v0.clone(), v3)?;
                    if v6 {
                        Ok(Some(Adt7::C0(v3, true, false, v4)))
                    } else {
                        let v7: u64 = f72(v0.clone(), v3)?;
                        let a686 = 0u64;
                        let a687 = v7;
                        if nat_lt(a686, a687) {
                            Ok(Some(Adt7::C0({
                                let a688 = v3;
                                let a689 = v7;
                                nat_add(a688, a689)?
                            }, false, false, v4)))
                        } else {
                            let v8: bool = f9(v0.clone(), v3, 46u8);
                            if v8 {
                                Ok(Some(Adt7::C0({
                                    let a690 = v3;
                                    let a691 = 1u64;
                                    nat_add(a690, a691)?
                                }, false, false, v4)))
                            } else {
                                let v9: u64 = f31(v0, v3);
                                Ok(Some(Adt7::C0({
                                    let a692 = v3;
                                    let a693 = v9;
                                    nat_add(a692, a693)?
                                }, false, false, false)))
                            }
                        }
                    }
                }
            } else {
                Ok(Some(Adt7::C0(v3, false, true, v4)))
            }
        }
    }
}

pub fn f87(v0: Bytes, v1: u64, v2: u64, v3: (u64, bool)) -> R<Option<(u64, bool)>> {
    let v4: u64 = {
        let (h694, _) = v3;
        h694
    };
    let v5: bool = {
        let (_, h695) = v3;
        h695
    };
    if v5 {
        let a696 = v4;
        let a697 = v2;
        if nat_lt(a696, a697) {
            let v6: bool = f9(v0, {
                let a698 = v1;
                let a699 = v4;
                nat_add(a698, a699)?
            }, 35u8);
            Ok(Some(({
                let a700 = v4;
                let a701 = 1u64;
                nat_add(a700, a701)?
            }, v6)))
        } else {
            Ok(None::<(u64, bool)>)
        }
    } else {
        Ok(None::<(u64, bool)>)
    }
}

pub fn f88(v0: Bytes, v1: u64) -> R<bool> {
    let v2: bool = f9(v0.clone(), v1, 194u8);
    let v3: bool = f9(v0, {
        let a702 = v1;
        let a703 = 1u64;
        nat_add(a702, a703)?
    }, 187u8);
    if v2 {
        Ok(v3)
    } else {
        Ok(false)
    }
}

pub fn run(p0: Bytes) -> R<Result<List<Adt0>, Bytes>> {
    f0(p0)
}
