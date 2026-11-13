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
    C0(Adt1, u64, u64),
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
    C8,
    C9,
    C10,
    C11(Bytes),
    C12(Bytes),
}

#[derive(Clone)]
pub enum Adt2 {
    C0(u64, Bytes),
}

#[derive(Clone)]
pub enum Adt3 {
    C0(u64, List<Adt0>, Option<Adt2>),
}

#[derive(Clone)]
pub enum Adt4 {
    C0(u64, List<Bytes>, bool, Option<Adt2>),
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
    F24(u64, Bytes),
}

impl Fn2 {
    pub fn apply(&self, p0: Adt4) -> R<Option<Adt4>> {
        match self {
            Fn2::F24(k0, k1) => f24(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F28(u64, Bytes),
    F46(u64, Bytes),
}

impl Fn3 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn3::F28(k0, k1) => f28(*k0, k1.clone(), p0),
            Fn3::F46(k0, k1) => f46(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F31,
}

impl Fn4 {
    pub fn apply(&self, p0: List<Bytes>) -> Option<List<Bytes>> {
        match self {
            Fn4::F31 => f31(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F33,
}

impl Fn5 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn5::F33 => f33(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F48,
}

impl Fn6 {
    pub fn apply(&self, p0: (Option<Bytes>, List<Bytes>), p1: Bytes) -> (Option<Bytes>, List<Bytes>) {
        match self {
            Fn6::F48 => f48(p0, p1),
        }
    }
}

pub fn f0(v0: Bytes) -> R<Result<List<Adt0>, Adt2>> {
    let v1: u64 = {
        let a1 = v0.clone();
        length_bytes(a1)
    };
    let v2: Adt3 = {
        let (h2, _) = f1(Fn0::F2(v1, v0), {
            let a3 = v1;
            let a4 = 1u64;
            nat_add(a3, a4)?
        }, Adt3::C0(0u64, List::<Adt0>::nil(), None::<Adt2>))?;
        h2
    };
    let m6 = match v2.clone() {
        Adt3::C0(_, _, h5) => {
            h5
        }
    };
    match m6 {
        None => {
            Ok(Ok::<List<Adt0>, Adt2>(f3(match v2 {
                Adt3::C0(_, h7, _) => {
                    h7
                }
            })))
        }
        Some(v3) => {
            Ok(Err::<List<Adt0>, Adt2>(v3))
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
        Adt3::C0(_, _, h14) => {
            h14
        }
    };
    match m15 {
        None => {
            let v4: u64 = match v2.clone() {
                Adt3::C0(h16, _, _) => {
                    h16
                }
            };
            let a17 = v4;
            let a18 = v1;
            if nat_lt(a17, a18) {
                let v5: u64 = f7(v0.clone(), v4)?;
                let a19 = 0u64;
                let a20 = v5;
                if nat_lt(a19, a20) {
                    Ok(Some(Adt3::C0({
                        let a21 = v4;
                        let a22 = v5;
                        nat_add(a21, a22)?
                    }, match v2 {
                        Adt3::C0(_, h23, _) => {
                            h23
                        }
                    }, None::<Adt2>)))
                } else {
                    if f8(v0.clone(), v4, 123u8) {
                        Ok(f9(v2, Adt1::C0, {
                            let a24 = v4;
                            let a25 = 1u64;
                            nat_add(a24, a25)?
                        }))
                    } else {
                        if f8(v0.clone(), v4, 125u8) {
                            Ok(f9(v2, Adt1::C1, {
                                let a26 = v4;
                                let a27 = 1u64;
                                nat_add(a26, a27)?
                            }))
                        } else {
                            if f8(v0.clone(), v4, 91u8) {
                                Ok(f9(v2, Adt1::C2, {
                                    let a28 = v4;
                                    let a29 = 1u64;
                                    nat_add(a28, a29)?
                                }))
                            } else {
                                if f8(v0.clone(), v4, 93u8) {
                                    Ok(f9(v2, Adt1::C3, {
                                        let a30 = v4;
                                        let a31 = 1u64;
                                        nat_add(a30, a31)?
                                    }))
                                } else {
                                    if f8(v0.clone(), v4, 40u8) {
                                        Ok(f9(v2, Adt1::C4, {
                                            let a32 = v4;
                                            let a33 = 1u64;
                                            nat_add(a32, a33)?
                                        }))
                                    } else {
                                        if f8(v0.clone(), v4, 41u8) {
                                            Ok(f9(v2, Adt1::C5, {
                                                let a34 = v4;
                                                let a35 = 1u64;
                                                nat_add(a34, a35)?
                                            }))
                                        } else {
                                            if f8(v0.clone(), v4, 44u8) {
                                                Ok(f9(v2, Adt1::C6, {
                                                    let a36 = v4;
                                                    let a37 = 1u64;
                                                    nat_add(a36, a37)?
                                                }))
                                            } else {
                                                if f8(v0.clone(), v4, 59u8) {
                                                    Ok(f9(v2, Adt1::C10, {
                                                        let a38 = v4;
                                                        let a39 = 1u64;
                                                        nat_add(a38, a39)?
                                                    }))
                                                } else {
                                                    if f8(v0.clone(), v4, 58u8) {
                                                        f10(v0, v2)
                                                    } else {
                                                        if f8(v0.clone(), v4, 34u8) {
                                                            f11(v0, v1, v2)
                                                        } else {
                                                            f12(v0, v1, v2)
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
            } else {
                Ok(None::<Adt3>)
            }
        }
        Some(_) => {
            Ok(None::<Adt3>)
        }
    }
}

pub fn f5(v0: Fn1, p43: List<Adt0>, p44: List<Adt0>) -> List<Adt0> {
    let m42 = (p43, p44);
    {
        let mut m42 = m42;
        loop {
            let step = {
                let (v1, v2) = m42;
                let m40 = v2;
                match m40.uncons() {
                    None => {
                        Err::<(List<Adt0>, List<Adt0>), List<Adt0>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt0>, List<Adt0>), List<Adt0>>(({
                            let c41: Fn1 = v0.clone();
                            c41.apply(v1, v3)
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

pub fn f6(v0: List<Adt0>, v1: Adt0) -> List<Adt0> {
    List::cons(v1, v0)
}

pub fn f7(v0: Bytes, v1: u64) -> R<u64> {
    if if f13(v0.clone(), v1, 9u8, 13u8) {
        true
    } else {
        f14(v0.clone(), v1, 32u8)
    } {
        Ok(1u64)
    } else {
        if f14(v0.clone(), v1, 194u8) {
            if if f14(v0.clone(), {
                let a45 = v1;
                let a46 = 1u64;
                nat_add(a45, a46)?
            }, 133u8) {
                true
            } else {
                f14(v0, {
                    let a47 = v1;
                    let a48 = 1u64;
                    nat_add(a47, a48)?
                }, 160u8)
            } {
                Ok(2u64)
            } else {
                Ok(0u64)
            }
        } else {
            if f14(v0.clone(), v1, 225u8) {
                if if f14(v0.clone(), {
                    let a49 = v1;
                    let a50 = 1u64;
                    nat_add(a49, a50)?
                }, 154u8) {
                    f14(v0, {
                        let a51 = v1;
                        let a52 = 2u64;
                        nat_add(a51, a52)?
                    }, 128u8)
                } else {
                    false
                } {
                    Ok(3u64)
                } else {
                    Ok(0u64)
                }
            } else {
                if f14(v0.clone(), v1, 226u8) {
                    if f14(v0.clone(), {
                        let a53 = v1;
                        let a54 = 1u64;
                        nat_add(a53, a54)?
                    }, 128u8) {
                        if if f13(v0.clone(), {
                            let a55 = v1;
                            let a56 = 2u64;
                            nat_add(a55, a56)?
                        }, 128u8, 138u8) {
                            true
                        } else {
                            if f14(v0.clone(), {
                                let a57 = v1;
                                let a58 = 2u64;
                                nat_add(a57, a58)?
                            }, 168u8) {
                                true
                            } else {
                                if f14(v0.clone(), {
                                    let a59 = v1;
                                    let a60 = 2u64;
                                    nat_add(a59, a60)?
                                }, 169u8) {
                                    true
                                } else {
                                    f14(v0, {
                                        let a61 = v1;
                                        let a62 = 2u64;
                                        nat_add(a61, a62)?
                                    }, 175u8)
                                }
                            }
                        } {
                            Ok(3u64)
                        } else {
                            Ok(0u64)
                        }
                    } else {
                        if if f14(v0.clone(), {
                            let a63 = v1;
                            let a64 = 1u64;
                            nat_add(a63, a64)?
                        }, 129u8) {
                            f14(v0, {
                                let a65 = v1;
                                let a66 = 2u64;
                                nat_add(a65, a66)?
                            }, 159u8)
                        } else {
                            false
                        } {
                            Ok(3u64)
                        } else {
                            Ok(0u64)
                        }
                    }
                } else {
                    if f14(v0.clone(), v1, 227u8) {
                        if if f14(v0.clone(), {
                            let a67 = v1;
                            let a68 = 1u64;
                            nat_add(a67, a68)?
                        }, 128u8) {
                            f14(v0, {
                                let a69 = v1;
                                let a70 = 2u64;
                                nat_add(a69, a70)?
                            }, 128u8)
                        } else {
                            false
                        } {
                            Ok(3u64)
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

pub fn f8(v0: Bytes, v1: u64, v2: u8) -> bool {
    let m71 = f15(v0, v1);
    match m71 {
        None => {
            false
        }
        Some(v3) => {
            let a72 = v3;
            let a73 = v2;
            equal(a72, a73)
        }
    }
}

pub fn f9(v0: Adt3, v1: Adt1, v2: u64) -> Option<Adt3> {
    Some(Adt3::C0(v2, List::cons(Adt0::C0(v1, match v0.clone() {
        Adt3::C0(h74, _, _) => {
            h74
        }
    }, v2), match v0 {
        Adt3::C0(_, h75, _) => {
            h75
        }
    }), None::<Adt2>))
}

pub fn f10(v0: Bytes, v1: Adt3) -> R<Option<Adt3>> {
    let v2: u64 = match v1.clone() {
        Adt3::C0(h76, _, _) => {
            h76
        }
    };
    if f8(v0.clone(), {
        let a77 = v2;
        let a78 = 1u64;
        nat_add(a77, a78)?
    }, 61u8) {
        Ok(f9(v1, Adt1::C8, {
            let a79 = v2;
            let a80 = 2u64;
            nat_add(a79, a80)?
        }))
    } else {
        if f8(v0, {
            let a81 = v2;
            let a82 = 1u64;
            nat_add(a81, a82)?
        }, 58u8) {
            Ok(f9(v1, Adt1::C9, {
                let a83 = v2;
                let a84 = 2u64;
                nat_add(a83, a84)?
            }))
        } else {
            Ok(f9(v1, Adt1::C7, {
                let a85 = v2;
                let a86 = 1u64;
                nat_add(a85, a86)?
            }))
        }
    }
}

pub fn f11(v0: Bytes, v1: u64, v2: Adt3) -> R<Option<Adt3>> {
    let v3: u64 = match v2.clone() {
        Adt3::C0(h87, _, _) => {
            h87
        }
    };
    let v4: Adt4 = f16(v0, v1, v3)?;
    let m89 = match v4.clone() {
        Adt4::C0(_, _, _, h88) => {
            h88
        }
    };
    match m89 {
        None => {
            if match v4.clone() {
                Adt4::C0(_, _, h91, _) => {
                    h91
                }
            } {
                Ok(f9(v2, Adt1::C11(f17(match v4.clone() {
                    Adt4::C0(_, h92, _, _) => {
                        h92
                    }
                })), match v4 {
                    Adt4::C0(h93, _, _, _) => {
                        h93
                    }
                }))
            } else {
                Ok(Some(Adt3::C0(v3, match v2 {
                    Adt3::C0(_, h94, _) => {
                        h94
                    }
                }, Some(f18(v3, f19(Str::lit("unterminated string")))))))
            }
        }
        Some(v5) => {
            Ok(Some(Adt3::C0(v3, match v2 {
                Adt3::C0(_, h90, _) => {
                    h90
                }
            }, Some(v5))))
        }
    }
}

pub fn f12(v0: Bytes, v1: u64, v2: Adt3) -> R<Option<Adt3>> {
    let v3: u64 = match v2.clone() {
        Adt3::C0(h95, _, _) => {
            h95
        }
    };
    let v4: u64 = f20(v0.clone(), v1, v3)?;
    Ok(f9(v2, Adt1::C12(f21(v0, v3, {
        let a96 = v4;
        let a97 = v3;
        nat_sub(a96, a97)
    })), v4))
}

pub fn f13(v0: Bytes, v1: u64, v2: u8, v3: u8) -> bool {
    let m98 = f15(v0, v1);
    match m98 {
        None => {
            false
        }
        Some(v4) => {
            f22(v4, v2, v3)
        }
    }
}

pub fn f14(v0: Bytes, v1: u64, v2: u8) -> bool {
    let m99 = f15(v0, v1);
    match m99 {
        None => {
            false
        }
        Some(v3) => {
            let a100 = v3;
            let a101 = v2;
            equal(a100, a101)
        }
    }
}

pub fn f15(v0: Bytes, v1: u64) -> Option<u8> {
    let a102 = v0;
    let a103 = v1;
    index_bytes(a102, a103)
}

pub fn f16(v0: Bytes, v1: u64, v2: u64) -> R<Adt4> {
    let (h104, _) = f23(Fn2::F24(v1, v0), {
        let a107 = {
            let a105 = v1;
            let a106 = v2;
            nat_sub(a105, a106)
        };
        let a108 = 1u64;
        nat_add(a107, a108)?
    }, Adt4::C0({
        let a109 = v2;
        let a110 = 1u64;
        nat_add(a109, a110)?
    }, List::<Bytes>::nil(), false, None::<Adt2>))?;
    Ok(h104)
}

pub fn f17(v0: List<Bytes>) -> Bytes {
    f25(f26(v0))
}

pub fn f18(v0: u64, v1: Bytes) -> Adt2 {
    Adt2::C0(v0, v1)
}

pub fn f19(v0: Str) -> Bytes {
    let a111 = v0;
    utf8_encode(a111)
}

pub fn f20(v0: Bytes, v1: u64, v2: u64) -> R<u64> {
    let (h112, _) = f27(Fn3::F28(v1, v0), {
        let a113 = v1;
        let a114 = v2;
        nat_sub(a113, a114)
    }, v2)?;
    Ok(h112)
}

pub fn f21(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m118 = {
        let a115 = v0;
        let a116 = v1;
        let a117 = v2;
        slice_bytes(a115, a116, a117)
    };
    match m118 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f22(v0: u8, v1: u8, v2: u8) -> bool {
    let m121 = {
        let a119 = v0;
        let a120 = v1;
        fixed_u8::checked_sub(a119, a120)
    };
    match m121 {
        None => {
            false
        }
        Some(_) => {
            let m124 = {
                let a122 = v2;
                let a123 = v0;
                fixed_u8::checked_sub(a122, a123)
            };
            match m124 {
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

pub fn f23(v0: Fn2, p129: u64, p130: Adt4) -> R<(Adt4, bool)> {
    let m128 = (p129, p130);
    {
        let mut m128 = m128;
        loop {
            let step = {
                let (v1, v2) = m128;
                let m125 = v1;
                if m125 == 0 {
                    Err::<(u64, Adt4), R<(Adt4, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m125 - 1;
                    let m127 = {
                        let c126: Fn2 = v0.clone();
                        c126.apply(v2.clone())?
                    };
                    match m127 {
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
                Ok(next) => { m128 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f24(v0: u64, v1: Bytes, v2: Adt4) -> R<Option<Adt4>> {
    f29(v1, v0, v2)
}

pub fn f25(v0: List<Bytes>) -> Bytes {
    let m133 = {
        let (h131, _) = f30(Fn4::F31, {
            let a132 = v0.clone();
            length_list(a132)
        }, v0);
        h131
    };
    match m133.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f26(v0: List<Bytes>) -> List<Bytes> {
    f32(Fn5::F33, List::<Bytes>::nil(), v0)
}

pub fn f27(v0: Fn3, p138: u64, p139: u64) -> R<(u64, bool)> {
    let m137 = (p138, p139);
    {
        let mut m137 = m137;
        loop {
            let step = {
                let (v1, v2) = m137;
                let m134 = v1;
                if m134 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m134 - 1;
                    let m136 = {
                        let c135: Fn3 = v0.clone();
                        c135.apply(v2)?
                    };
                    match m136 {
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
                Ok(next) => { m137 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f28(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f34(v1, v0, v2)
}

pub fn f29(v0: Bytes, v1: u64, v2: Adt4) -> R<Option<Adt4>> {
    let m141 = match v2.clone() {
        Adt4::C0(_, _, _, h140) => {
            h140
        }
    };
    match m141 {
        None => {
            if match v2.clone() {
                Adt4::C0(_, _, h142, _) => {
                    h142
                }
            } {
                Ok(None::<Adt4>)
            } else {
                let v4: u64 = match v2.clone() {
                    Adt4::C0(h143, _, _, _) => {
                        h143
                    }
                };
                let a144 = v4;
                let a145 = v1;
                if nat_lt(a144, a145) {
                    if f8(v0.clone(), v4, 34u8) {
                        Ok(Some(Adt4::C0({
                            let a146 = v4;
                            let a147 = 1u64;
                            nat_add(a146, a147)?
                        }, match v2 {
                            Adt4::C0(_, h148, _, _) => {
                                h148
                            }
                        }, true, None::<Adt2>)))
                    } else {
                        if f8(v0.clone(), v4, 92u8) {
                            Ok(Some(f35(v0, v1, v2)?))
                        } else {
                            let a149 = f36(v0.clone(), v4)?;
                            let a150 = 32u64;
                            if nat_lt(a149, a150) {
                                Ok(Some(f37(v2, v4, f19(Str::lit("a control character must be escaped in a string")))))
                            } else {
                                let v5: u64 = f38(v0.clone(), v1, v4)?;
                                Ok(Some(f39(v2, v5, f21(v0, v4, {
                                    let a151 = v5;
                                    let a152 = v4;
                                    nat_sub(a151, a152)
                                }))))
                            }
                        }
                    }
                } else {
                    Ok(None::<Adt4>)
                }
            }
        }
        Some(_) => {
            Ok(None::<Adt4>)
        }
    }
}

pub fn f30(v0: Fn4, p157: u64, p158: List<Bytes>) -> (List<Bytes>, bool) {
    let m156 = (p157, p158);
    {
        let mut m156 = m156;
        loop {
            let step = {
                let (v1, v2) = m156;
                let m153 = v1;
                if m153 == 0 {
                    Err::<(u64, List<Bytes>), (List<Bytes>, bool)>((v2, false))
                } else {
                    let v3 = m153 - 1;
                    let m155 = {
                        let c154: Fn4 = v0.clone();
                        c154.apply(v2.clone())
                    };
                    match m155 {
                        None => {
                            Err::<(u64, List<Bytes>), (List<Bytes>, bool)>((v2, true))
                        }
                        Some(v4) => {
                            Ok::<(u64, List<Bytes>), (List<Bytes>, bool)>((v3, v4))
                        }
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

pub fn f31(v0: List<Bytes>) -> Option<List<Bytes>> {
    f40(v0)
}

pub fn f32(v0: Fn5, p162: List<Bytes>, p163: List<Bytes>) -> List<Bytes> {
    let m161 = (p162, p163);
    {
        let mut m161 = m161;
        loop {
            let step = {
                let (v1, v2) = m161;
                let m159 = v2;
                match m159.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c160: Fn5 = v0.clone();
                            c160.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m161 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f33(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f34(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let a164 = v2;
    let a165 = v1;
    if nat_lt(a164, a165) {
        if f41(v0, v2)? {
            Ok(None::<u64>)
        } else {
            Ok(Some({
                let a166 = v2;
                let a167 = 1u64;
                nat_add(a166, a167)?
            }))
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f35(v0: Bytes, v1: u64, v2: Adt4) -> R<Adt4> {
    let v3: u64 = match v2.clone() {
        Adt4::C0(h168, _, _, _) => {
            h168
        }
    };
    let m171 = f15(v0.clone(), {
        let a169 = v3;
        let a170 = 1u64;
        nat_add(a169, a170)?
    });
    match m171 {
        None => {
            Ok(Adt4::C0(v1, match v2 {
                Adt4::C0(_, h172, _, _) => {
                    h172
                }
            }, false, None::<Adt2>))
        }
        Some(v4) => {
            let a173 = v4;
            let a174 = 34u8;
            if equal(a173, a174) {
                Ok(f39(v2, {
                    let a175 = v3;
                    let a176 = 2u64;
                    nat_add(a175, a176)?
                }, f19(Str::lit("\""))))
            } else {
                let a177 = v4;
                let a178 = 92u8;
                if equal(a177, a178) {
                    Ok(f39(v2, {
                        let a179 = v3;
                        let a180 = 2u64;
                        nat_add(a179, a180)?
                    }, f19(Str::lit("\\"))))
                } else {
                    let a181 = v4;
                    let a182 = 47u8;
                    if equal(a181, a182) {
                        Ok(f39(v2, {
                            let a183 = v3;
                            let a184 = 2u64;
                            nat_add(a183, a184)?
                        }, f19(Str::lit("/"))))
                    } else {
                        let a185 = v4;
                        let a186 = 98u8;
                        if equal(a185, a186) {
                            Ok(f39(v2, {
                                let a187 = v3;
                                let a188 = 2u64;
                                nat_add(a187, a188)?
                            }, f42(8u64)))
                        } else {
                            let a189 = v4;
                            let a190 = 102u8;
                            if equal(a189, a190) {
                                Ok(f39(v2, {
                                    let a191 = v3;
                                    let a192 = 2u64;
                                    nat_add(a191, a192)?
                                }, f42(12u64)))
                            } else {
                                let a193 = v4;
                                let a194 = 110u8;
                                if equal(a193, a194) {
                                    Ok(f39(v2, {
                                        let a195 = v3;
                                        let a196 = 2u64;
                                        nat_add(a195, a196)?
                                    }, f42(10u64)))
                                } else {
                                    let a197 = v4;
                                    let a198 = 114u8;
                                    if equal(a197, a198) {
                                        Ok(f39(v2, {
                                            let a199 = v3;
                                            let a200 = 2u64;
                                            nat_add(a199, a200)?
                                        }, f42(13u64)))
                                    } else {
                                        let a201 = v4;
                                        let a202 = 116u8;
                                        if equal(a201, a202) {
                                            Ok(f39(v2, {
                                                let a203 = v3;
                                                let a204 = 2u64;
                                                nat_add(a203, a204)?
                                            }, f42(9u64)))
                                        } else {
                                            let a205 = v4;
                                            let a206 = 117u8;
                                            if equal(a205, a206) {
                                                f43(v0, v2)
                                            } else {
                                                Ok(f37(v2, v3, f44(f44(f19(Str::lit("`\\")), f21(v0.clone(), {
                                                    let a207 = v3;
                                                    let a208 = 1u64;
                                                    nat_add(a207, a208)?
                                                }, f45(v0, {
                                                    let a209 = v3;
                                                    let a210 = 1u64;
                                                    nat_add(a209, a210)?
                                                })?)), f19(Str::lit("` is not an escape")))))
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

pub fn f36(v0: Bytes, v1: u64) -> R<u64> {
    let m211 = f15(v0, v1);
    match m211 {
        None => {
            Ok(0u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a256 = {
                let a232 = {
                    let a220 = {
                        let a214 = {
                            let a212 = v3;
                            let a213 = 128u8;
                            fixed_u8::bit_and(a212, a213)
                        };
                        let a215 = 128u8;
                        if equal(a214, a215) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a221 = {
                        let a218 = {
                            let a216 = v3;
                            let a217 = 64u8;
                            fixed_u8::bit_and(a216, a217)
                        };
                        let a219 = 64u8;
                        if equal(a218, a219) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a220, a221)?
                };
                let a233 = {
                    let a230 = {
                        let a224 = {
                            let a222 = v3;
                            let a223 = 32u8;
                            fixed_u8::bit_and(a222, a223)
                        };
                        let a225 = 32u8;
                        if equal(a224, a225) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a231 = {
                        let a228 = {
                            let a226 = v3;
                            let a227 = 16u8;
                            fixed_u8::bit_and(a226, a227)
                        };
                        let a229 = 16u8;
                        if equal(a228, a229) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a230, a231)?
                };
                nat_add(a232, a233)?
            };
            let a257 = {
                let a254 = {
                    let a242 = {
                        let a236 = {
                            let a234 = v3;
                            let a235 = 8u8;
                            fixed_u8::bit_and(a234, a235)
                        };
                        let a237 = 8u8;
                        if equal(a236, a237) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a243 = {
                        let a240 = {
                            let a238 = v3;
                            let a239 = 4u8;
                            fixed_u8::bit_and(a238, a239)
                        };
                        let a241 = 4u8;
                        if equal(a240, a241) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a242, a243)?
                };
                let a255 = {
                    let a252 = {
                        let a246 = {
                            let a244 = v3;
                            let a245 = 2u8;
                            fixed_u8::bit_and(a244, a245)
                        };
                        let a247 = 2u8;
                        if equal(a246, a247) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a253 = {
                        let a250 = {
                            let a248 = v3;
                            let a249 = 1u8;
                            fixed_u8::bit_and(a248, a249)
                        };
                        let a251 = 1u8;
                        if equal(a250, a251) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a252, a253)?
                };
                nat_add(a254, a255)?
            };
            nat_add(a256, a257)
        }
    }
}

pub fn f37(v0: Adt4, v1: u64, v2: Bytes) -> Adt4 {
    Adt4::C0(match v0.clone() {
        Adt4::C0(h258, _, _, _) => {
            h258
        }
    }, match v0 {
        Adt4::C0(_, h259, _, _) => {
            h259
        }
    }, false, Some(f18(v1, v2)))
}

pub fn f38(v0: Bytes, v1: u64, v2: u64) -> R<u64> {
    let (h260, _) = f27(Fn3::F46(v1, v0), {
        let a261 = v1;
        let a262 = v2;
        nat_sub(a261, a262)
    }, v2)?;
    Ok(h260)
}

pub fn f39(v0: Adt4, v1: u64, v2: Bytes) -> Adt4 {
    Adt4::C0(v1, List::cons(v2, match v0 {
        Adt4::C0(_, h263, _, _) => {
            h263
        }
    }), false, None::<Adt2>)
}

pub fn f40(v0: List<Bytes>) -> Option<List<Bytes>> {
    let m264 = v0.clone();
    match m264.uncons() {
        None => {
            None::<List<Bytes>>
        }
        Some((_, v2)) => {
            let m265 = v2;
            match m265.uncons() {
                None => {
                    None::<List<Bytes>>
                }
                Some((_, _)) => {
                    let v5: (Option<Bytes>, List<Bytes>) = f47(Fn6::F48, (None::<Bytes>, List::<Bytes>::nil()), v0);
                    let m267 = {
                        let (h266, _) = v5;
                        h266
                    };
                    match m267 {
                        None => {
                            Some(f26({
                                let (_, h269) = v5;
                                h269
                            }))
                        }
                        Some(v6) => {
                            Some(f26(List::cons(v6, {
                                let (_, h268) = v5;
                                h268
                            })))
                        }
                    }
                }
            }
        }
    }
}

pub fn f41(v0: Bytes, v1: u64) -> R<bool> {
    let a270 = 0u64;
    let a271 = f7(v0.clone(), v1)?;
    if nat_lt(a270, a271) {
        Ok(true)
    } else {
        Ok(f49(v0, v1))
    }
}

pub fn f42(v0: u64) -> Bytes {
    f21(Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff]), v0, 1u64)
}

pub fn f43(v0: Bytes, v1: Adt4) -> R<Adt4> {
    let v2: u64 = match v1.clone() {
        Adt4::C0(h272, _, _, _) => {
            h272
        }
    };
    let m275 = f50(v0, {
        let a273 = v2;
        let a274 = 2u64;
        nat_add(a273, a274)?
    })?;
    match m275 {
        None => {
            Ok(f37(v1, v2, f19(Str::lit("a `\\u` escape takes four hexadecimal digits"))))
        }
        Some(v3) => {
            let a276 = 55296u64;
            let a277 = v3;
            if nat_le(a276, a277) {
                let a278 = v3;
                let a279 = 57343u64;
                if nat_le(a278, a279) {
                    Ok(f37(v1, v2, f44(f44(f19(Str::lit("`\\u")), f51(v3)), f19(Str::lit("` is not a Unicode scalar value")))))
                } else {
                    Ok(f39(v1, {
                        let a280 = v2;
                        let a281 = 6u64;
                        nat_add(a280, a281)?
                    }, f52(v3)?))
                }
            } else {
                Ok(f39(v1, {
                    let a282 = v2;
                    let a283 = 6u64;
                    nat_add(a282, a283)?
                }, f52(v3)?))
            }
        }
    }
}

pub fn f44(v0: Bytes, v1: Bytes) -> Bytes {
    let a284 = v0;
    let a285 = v1;
    append_bytes(a284, a285)
}

pub fn f45(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f36(v0, v1)?;
    let a286 = v2;
    let a287 = 128u64;
    if nat_lt(a286, a287) {
        Ok(1u64)
    } else {
        let a288 = v2;
        let a289 = 224u64;
        if nat_lt(a288, a289) {
            Ok(2u64)
        } else {
            let a290 = v2;
            let a291 = 240u64;
            if nat_lt(a290, a291) {
                Ok(3u64)
            } else {
                Ok(4u64)
            }
        }
    }
}

pub fn f46(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f53(v1, v0, v2)
}

pub fn f47(v0: Fn6, p295: (Option<Bytes>, List<Bytes>), p296: List<Bytes>) -> (Option<Bytes>, List<Bytes>) {
    let m294 = (p295, p296);
    {
        let mut m294 = m294;
        loop {
            let step = {
                let (v1, v2) = m294;
                let m292 = v2;
                match m292.uncons() {
                    None => {
                        Err::<((Option<Bytes>, List<Bytes>), List<Bytes>), (Option<Bytes>, List<Bytes>)>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<((Option<Bytes>, List<Bytes>), List<Bytes>), (Option<Bytes>, List<Bytes>)>(({
                            let c293: Fn6 = v0.clone();
                            c293.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m294 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f48(v0: (Option<Bytes>, List<Bytes>), v1: Bytes) -> (Option<Bytes>, List<Bytes>) {
    f54(v0, v1)
}

pub fn f49(v0: Bytes, v1: u64) -> bool {
    if f8(v0.clone(), v1, 123u8) {
        true
    } else {
        if f8(v0.clone(), v1, 125u8) {
            true
        } else {
            if f8(v0.clone(), v1, 91u8) {
                true
            } else {
                if f8(v0.clone(), v1, 93u8) {
                    true
                } else {
                    if f8(v0.clone(), v1, 40u8) {
                        true
                    } else {
                        if f8(v0.clone(), v1, 41u8) {
                            true
                        } else {
                            if f8(v0.clone(), v1, 44u8) {
                                true
                            } else {
                                if f8(v0.clone(), v1, 58u8) {
                                    true
                                } else {
                                    if f8(v0.clone(), v1, 59u8) {
                                        true
                                    } else {
                                        f8(v0, v1, 34u8)
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

pub fn f50(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m297 = f55(v0.clone(), v1)?;
    match m297 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            let m300 = f55(v0.clone(), {
                let a298 = v1;
                let a299 = 1u64;
                nat_add(a298, a299)?
            })?;
            match m300 {
                None => {
                    Ok(None::<u64>)
                }
                Some(v3) => {
                    let m303 = f55(v0.clone(), {
                        let a301 = v1;
                        let a302 = 2u64;
                        nat_add(a301, a302)?
                    })?;
                    match m303 {
                        None => {
                            Ok(None::<u64>)
                        }
                        Some(v4) => {
                            let m306 = f55(v0, {
                                let a304 = v1;
                                let a305 = 3u64;
                                nat_add(a304, a305)?
                            })?;
                            match m306 {
                                None => {
                                    Ok(None::<u64>)
                                }
                                Some(v5) => {
                                    Ok(Some({
                                        let a317 = {
                                            let a315 = {
                                                let a313 = {
                                                    let a311 = {
                                                        let a309 = {
                                                            let a307 = v2;
                                                            let a308 = 16u64;
                                                            nat_mul(a307, a308)?
                                                        };
                                                        let a310 = v3;
                                                        nat_add(a309, a310)?
                                                    };
                                                    let a312 = 16u64;
                                                    nat_mul(a311, a312)?
                                                };
                                                let a314 = v4;
                                                nat_add(a313, a314)?
                                            };
                                            let a316 = 16u64;
                                            nat_mul(a315, a316)?
                                        };
                                        let a318 = v5;
                                        nat_add(a317, a318)?
                                    }))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f51(v0: u64) -> Bytes {
    f44(f44(f56(v0, 4096u64), f56(v0, 256u64)), f44(f56(v0, 16u64), f57(v0)))
}

pub fn f52(v0: u64) -> R<Bytes> {
    let a319 = v0;
    let a320 = 128u64;
    if nat_lt(a319, a320) {
        Ok(f42(v0))
    } else {
        let a321 = v0;
        let a322 = 2048u64;
        if nat_lt(a321, a322) {
            Ok(f44(f42({
                let a326 = 192u64;
                let a327 = {
                    let a323 = v0;
                    let a324 = 64u64;
                    let a325 = 0u64;
                    nat_quot(a323, a324, a325)
                };
                nat_add(a326, a327)?
            }), f42({
                let a331 = 128u64;
                let a332 = {
                    let a328 = v0;
                    let a329 = 64u64;
                    let a330 = 0u64;
                    nat_rem(a328, a329, a330)
                };
                nat_add(a331, a332)?
            })))
        } else {
            Ok(f44(f42({
                let a336 = 224u64;
                let a337 = {
                    let a333 = v0;
                    let a334 = 4096u64;
                    let a335 = 0u64;
                    nat_quot(a333, a334, a335)
                };
                nat_add(a336, a337)?
            }), f44(f42({
                let a344 = 128u64;
                let a345 = {
                    let a341 = {
                        let a338 = v0;
                        let a339 = 64u64;
                        let a340 = 0u64;
                        nat_quot(a338, a339, a340)
                    };
                    let a342 = 64u64;
                    let a343 = 0u64;
                    nat_rem(a341, a342, a343)
                };
                nat_add(a344, a345)?
            }), f42({
                let a349 = 128u64;
                let a350 = {
                    let a346 = v0;
                    let a347 = 64u64;
                    let a348 = 0u64;
                    nat_rem(a346, a347, a348)
                };
                nat_add(a349, a350)?
            }))))
        }
    }
}

pub fn f53(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let a351 = v2;
    let a352 = v1;
    if nat_lt(a351, a352) {
        let v3: u64 = f36(v0, v2)?;
        let a353 = v3;
        let a354 = 32u64;
        if nat_lt(a353, a354) {
            Ok(None::<u64>)
        } else {
            let a355 = v3;
            let a356 = 34u64;
            if nat_eq(a355, a356) {
                Ok(None::<u64>)
            } else {
                let a357 = v3;
                let a358 = 92u64;
                if nat_eq(a357, a358) {
                    Ok(None::<u64>)
                } else {
                    Ok(Some({
                        let a359 = v2;
                        let a360 = 1u64;
                        nat_add(a359, a360)?
                    }))
                }
            }
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f54(v0: (Option<Bytes>, List<Bytes>), v1: Bytes) -> (Option<Bytes>, List<Bytes>) {
    let m362 = {
        let (h361, _) = v0;
        h361
    };
    match m362 {
        None => {
            (Some(v1), {
                let (_, h363) = v0;
                h363
            })
        }
        Some(v2) => {
            (None::<Bytes>, List::cons({
                let a364 = v2;
                let a365 = v1;
                append_bytes(a364, a365)
            }, {
                let (_, h366) = v0;
                h366
            }))
        }
    }
}

pub fn f55(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m367 = f15(v0, v1);
    match m367 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            f58(v2)
        }
    }
}

pub fn f56(v0: u64, v1: u64) -> Bytes {
    f57({
        let a368 = v0;
        let a369 = v1;
        let a370 = 0u64;
        nat_quot(a368, a369, a370)
    })
}

pub fn f57(v0: u64) -> Bytes {
    f21(Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66]), {
        let a371 = v0;
        let a372 = 16u64;
        let a373 = 0u64;
        nat_rem(a371, a372, a373)
    }, 1u64)
}

pub fn f58(v0: u8) -> R<Option<u64>> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a418 = {
            let a394 = {
                let a382 = {
                    let a376 = {
                        let a374 = v1;
                        let a375 = 128u8;
                        fixed_u8::bit_and(a374, a375)
                    };
                    let a377 = 128u8;
                    if equal(a376, a377) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a383 = {
                    let a380 = {
                        let a378 = v1;
                        let a379 = 64u8;
                        fixed_u8::bit_and(a378, a379)
                    };
                    let a381 = 64u8;
                    if equal(a380, a381) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a382, a383)?
            };
            let a395 = {
                let a392 = {
                    let a386 = {
                        let a384 = v1;
                        let a385 = 32u8;
                        fixed_u8::bit_and(a384, a385)
                    };
                    let a387 = 32u8;
                    if equal(a386, a387) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a393 = {
                    let a390 = {
                        let a388 = v1;
                        let a389 = 16u8;
                        fixed_u8::bit_and(a388, a389)
                    };
                    let a391 = 16u8;
                    if equal(a390, a391) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a392, a393)?
            };
            nat_add(a394, a395)?
        };
        let a419 = {
            let a416 = {
                let a404 = {
                    let a398 = {
                        let a396 = v1;
                        let a397 = 8u8;
                        fixed_u8::bit_and(a396, a397)
                    };
                    let a399 = 8u8;
                    if equal(a398, a399) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a405 = {
                    let a402 = {
                        let a400 = v1;
                        let a401 = 4u8;
                        fixed_u8::bit_and(a400, a401)
                    };
                    let a403 = 4u8;
                    if equal(a402, a403) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a404, a405)?
            };
            let a417 = {
                let a414 = {
                    let a408 = {
                        let a406 = v1;
                        let a407 = 2u8;
                        fixed_u8::bit_and(a406, a407)
                    };
                    let a409 = 2u8;
                    if equal(a408, a409) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a415 = {
                    let a412 = {
                        let a410 = v1;
                        let a411 = 1u8;
                        fixed_u8::bit_and(a410, a411)
                    };
                    let a413 = 1u8;
                    if equal(a412, a413) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a414, a415)?
            };
            nat_add(a416, a417)?
        };
        nat_add(a418, a419)?
    };
    let a420 = v2;
    let a421 = 48u64;
    if nat_lt(a420, a421) {
        Ok(None::<u64>)
    } else {
        let a422 = v2;
        let a423 = 57u64;
        if nat_le(a422, a423) {
            Ok(Some({
                let a424 = v2;
                let a425 = 48u64;
                nat_sub(a424, a425)
            }))
        } else {
            let a426 = v2;
            let a427 = 65u64;
            if nat_lt(a426, a427) {
                Ok(None::<u64>)
            } else {
                let a428 = v2;
                let a429 = 70u64;
                if nat_le(a428, a429) {
                    Ok(Some({
                        let a430 = v2;
                        let a431 = 55u64;
                        nat_sub(a430, a431)
                    }))
                } else {
                    let a432 = v2;
                    let a433 = 97u64;
                    if nat_lt(a432, a433) {
                        Ok(None::<u64>)
                    } else {
                        let a434 = v2;
                        let a435 = 102u64;
                        if nat_le(a434, a435) {
                            Ok(Some({
                                let a436 = v2;
                                let a437 = 87u64;
                                nat_sub(a436, a437)
                            }))
                        } else {
                            Ok(None::<u64>)
                        }
                    }
                }
            }
        }
    }
}

pub fn run(p0: Bytes) -> R<Result<List<Adt0>, Adt2>> {
    f0(p0)
}
