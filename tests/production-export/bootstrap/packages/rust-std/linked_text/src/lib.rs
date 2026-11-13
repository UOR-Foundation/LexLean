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
    C0(Bytes, Bytes, Bytes, Bytes, Option<Bytes>, List<Bytes>, List<Bytes>, List<(Bytes, Bytes)>, List<Adt1>, Adt6, Option<Bytes>, Option<Adt12>, Option<Adt12>),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(Bytes, Bytes),
    C1(Adt2),
    C2(Bytes),
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
    C0(Bytes, List<Adt1>, Adt2, std::rc::Rc<Adt6>),
    C1(Adt7),
    C2,
    C3(std::rc::Rc<Adt6>, std::rc::Rc<Adt6>),
}

#[derive(Clone)]
pub enum Adt7 {
    C0(Bytes, Bytes, Adt8, Adt2, Adt9, Adt11),
}

#[derive(Clone)]
pub enum Adt8 {
    C0,
    C1,
    C2,
    C3,
    C4,
    C5,
}

#[derive(Clone)]
pub enum Adt9 {
    C0(Bytes, Adt2, Adt2),
    C1(Adt2, Adt10),
}

#[derive(Clone)]
pub enum Adt10 {
    C0(std::rc::Rc<Adt10>),
    C1(List<u64>),
    C2(Adt2),
    C3(Adt2),
    C4(Adt2, std::rc::Rc<Adt10>),
    C5,
    C6(Adt2),
    C7,
    C8,
    C9(u64, Adt2, std::rc::Rc<Adt10>),
    C10(Option<u64>, std::rc::Rc<Adt10>),
    C11(Option<u64>, Adt2),
    C12(std::rc::Rc<Adt10>),
    C13(Adt2, std::rc::Rc<Adt10>),
    C14(Adt2, std::rc::Rc<Adt10>),
    C15(Adt4, Adt2, std::rc::Rc<Adt10>),
    C16,
    C17(std::rc::Rc<Adt10>, std::rc::Rc<Adt10>),
    C18(bool, Adt2, std::rc::Rc<Adt10>),
    C19(Bytes, Bytes, List<(u64, Bytes)>, std::rc::Rc<Adt10>, std::rc::Rc<Adt10>),
    C20(Adt2, Adt2, std::rc::Rc<Adt10>),
}

#[derive(Clone)]
pub enum Adt11 {
    C0,
    C1(List<Bytes>),
    C2(List<Bytes>),
}

#[derive(Clone)]
pub enum Adt12 {
    C0(bool),
    C1(bool, u64),
    C2(Bytes),
    C3(std::rc::Rc<Adt12>),
    C4(std::rc::Rc<Adt12>),
    C5,
    C6(std::rc::Rc<Adt12>, std::rc::Rc<Adt12>),
    C7(Bytes, std::rc::Rc<Adt12>, std::rc::Rc<Adt12>),
}

#[derive(Clone)]
pub enum Adt13 {
    C0(List<(u64, u64)>, u64, bool),
}

#[derive(Clone)]
pub enum Adt14 {
    C0(u64),
    C1(u64),
}

#[derive(Clone)]
pub enum Fn0 {
    F10,
}

impl Fn0 {
    pub fn apply(&self, p0: List<Adt12>, p1: (Bytes, Adt0)) -> R<List<Adt12>> {
        match self {
            Fn0::F10 => f10(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F22,
    F62,
}

impl Fn1 {
    pub fn apply(&self, p0: Adt12, p1: Adt12) -> Adt12 {
        match self {
            Fn1::F22 => f22(p0, p1),
            Fn1::F62 => f62(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F26,
}

impl Fn2 {
    pub fn apply(&self, p0: List<(Bytes, Adt0)>, p1: (Bytes, Adt0)) -> List<(Bytes, Adt0)> {
        match self {
            Fn2::F26 => f26(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F29,
}

impl Fn3 {
    pub fn apply(&self, p0: List<List<(Bytes, Adt0)>>, p1: (Bytes, Adt0)) -> List<List<(Bytes, Adt0)>> {
        match self {
            Fn3::F29 => f29(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F31,
}

impl Fn4 {
    pub fn apply(&self, p0: List<List<(Bytes, Adt0)>>) -> R<Option<List<List<(Bytes, Adt0)>>>> {
        match self {
            Fn4::F31 => f31(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F34(Bytes),
    F71(u64, Bytes),
    F137(Bytes),
}

impl Fn5 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn5::F34(k0) => f34(k0.clone(), p0),
            Fn5::F71(k0, k1) => Ok(f71(*k0, k1.clone(), p0)),
            Fn5::F137(k0) => f137(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F37,
    F156,
}

impl Fn6 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn6::F37 => f37(p0, p1),
            Fn6::F156 => f156(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F40,
}

impl Fn7 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, Bytes)) -> R<List<Bytes>> {
        match self {
            Fn7::F40 => f40(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F42,
}

impl Fn8 {
    pub fn apply(&self, p0: List<Adt12>, p1: Adt12) -> List<Adt12> {
        match self {
            Fn8::F42 => f42(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F50,
}

impl Fn9 {
    pub fn apply(&self, p0: List<List<(Bytes, Adt0)>>, p1: List<(Bytes, Adt0)>) -> List<List<(Bytes, Adt0)>> {
        match self {
            Fn9::F50 => f50(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F56(Bytes),
}

impl Fn10 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Bytes) -> Option<Bytes> {
        match self {
            Fn10::F56(k0) => f56(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F61,
}

impl Fn11 {
    pub fn apply(&self, p0: (List<Adt12>, Adt13), p1: Adt1) -> R<(List<Adt12>, Adt13)> {
        match self {
            Fn11::F61 => f61(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F66,
}

impl Fn12 {
    pub fn apply(&self, p0: List<Adt12>, p1: Bytes) -> List<Adt12> {
        match self {
            Fn12::F66 => f66(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F75,
}

impl Fn13 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn13::F75 => f75(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F77,
}

impl Fn14 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
        match self {
            Fn14::F77 => f77(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F93,
}

impl Fn15 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn15::F93 => f93(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F95(Adt0),
}

impl Fn16 {
    pub fn apply(&self, p0: Adt12, p1: (Bytes, Bytes)) -> Adt12 {
        match self {
            Fn16::F95(k0) => f95(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F97,
}

impl Fn17 {
    pub fn apply(&self, p0: (List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>)) -> R<Option<(List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>)>> {
        match self {
            Fn17::F97 => f97(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F102,
}

impl Fn18 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn18::F102 => f102(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F120(u64),
}

impl Fn19 {
    pub fn apply(&self, p0: Option<u64>, p1: (u64, u64)) -> Option<u64> {
        match self {
            Fn19::F120(k0) => f120(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn20 {
    F124,
    F143,
}

impl Fn20 {
    pub fn apply(&self, p0: Adt12, p1: u64) -> Adt12 {
        match self {
            Fn20::F124 => f124(p0, p1),
            Fn20::F143 => f143(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn21 {
    F132,
}

impl Fn21 {
    pub fn apply(&self, p0: Adt12, p1: Bytes) -> Adt12 {
        match self {
            Fn21::F132 => f132(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn22 {
    F136,
}

impl Fn22 {
    pub fn apply(&self, p0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
        match self {
            Fn22::F136 => f136(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn23 {
    F139,
}

impl Fn23 {
    pub fn apply(&self, p0: List<u64>, p1: u64) -> List<u64> {
        match self {
            Fn23::F139 => f139(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn24 {
    F142,
}

impl Fn24 {
    pub fn apply(&self, p0: (List<u64>, Adt13), p1: u64) -> R<(List<u64>, Adt13)> {
        match self {
            Fn24::F142 => f142(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn25 {
    F145,
}

impl Fn25 {
    pub fn apply(&self, p0: List<u64>, p1: (u64, Bytes)) -> List<u64> {
        match self {
            Fn25::F145 => f145(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn26 {
    F147,
}

impl Fn26 {
    pub fn apply(&self, p0: (List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>))) -> Option<(List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>))> {
        match self {
            Fn26::F147 => f147(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn27 {
    F155,
}

impl Fn27 {
    pub fn apply(&self, p0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
        match self {
            Fn27::F155 => f155(p0),
        }
    }
}

pub fn f0(v0: List<(Bytes, Adt0)>) -> R<Bytes> {
    f1(f2(v0)?)
}

pub fn f1(v0: Adt12) -> R<Bytes> {
    f3(v0)
}

pub fn f2(v0: List<(Bytes, Adt0)>) -> R<Adt12> {
    Ok(f4(f5(Str::lit("spec"), f6(Str::lit("lexlean/linked-ir/1")), f5(Str::lit("modules"), f7(f8(f9(Fn0::F10, List::<Adt12>::nil(), f11(f12(v0)?))?)), f13()))))
}

pub fn f3(v0: Adt12) -> R<Bytes> {
    let m1 = v0;
    match m1 {
        Adt12::C0(v1) => {
            if v1 {
                Ok(Bytes::lit(&[0x74, 0x72, 0x75, 0x65]))
            } else {
                Ok(Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]))
            }
        }
        Adt12::C1(v2, v3) => {
            f14(v2, v3)
        }
        Adt12::C2(v4) => {
            f15(v4)
        }
        Adt12::C3(r2) => {
            let v5 = (*r2).clone();
            Ok(f16(f17(v5)?))
        }
        Adt12::C4(r3) => {
            let v6 = (*r3).clone();
            f18(f19(v6)?)
        }
        Adt12::C5 => {
            Ok(Bytes::lit(&[]))
        }
        Adt12::C6(_, _) => {
            Ok(Bytes::lit(&[]))
        }
        Adt12::C7(_, _, _) => {
            Ok(Bytes::lit(&[]))
        }
    }
}

pub fn f4(v0: Adt12) -> Adt12 {
    Adt12::C4(std::rc::Rc::new(v0))
}

pub fn f5(v0: Str, v1: Adt12, v2: Adt12) -> Adt12 {
    Adt12::C7(f20(v0), std::rc::Rc::new(v1), std::rc::Rc::new(v2))
}

pub fn f6(v0: Str) -> Adt12 {
    Adt12::C2(f20(v0))
}

pub fn f7(v0: Adt12) -> Adt12 {
    Adt12::C3(std::rc::Rc::new(v0))
}

pub fn f8(v0: List<Adt12>) -> Adt12 {
    f21(Fn1::F22, f13(), f23(v0))
}

pub fn f9(v0: Fn0, p7: List<Adt12>, p8: List<(Bytes, Adt0)>) -> R<List<Adt12>> {
    let m6 = (p7, p8);
    {
        let mut m6 = m6;
        loop {
            let step = {
                let (v1, v2) = m6;
                let m4 = v2;
                match m4.uncons() {
                    None => {
                        Err::<(List<Adt12>, List<(Bytes, Adt0)>), R<List<Adt12>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt12>, List<(Bytes, Adt0)>), R<List<Adt12>>>(({
                            let c5: Fn0 = v0.clone();
                            c5.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m6 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f10(v0: List<Adt12>, v1: (Bytes, Adt0)) -> R<List<Adt12>> {
    Ok(List::cons(f24({
        let (_, h9) = v1;
        h9
    })?, v0))
}

pub fn f11(v0: List<(Bytes, Adt0)>) -> List<(Bytes, Adt0)> {
    f25(Fn2::F26, List::<(Bytes, Adt0)>::nil(), v0)
}

pub fn f12(v0: List<(Bytes, Adt0)>) -> R<List<(Bytes, Adt0)>> {
    let v1: List<List<(Bytes, Adt0)>> = f27(f28(Fn3::F29, List::<List<(Bytes, Adt0)>>::nil(), v0.clone()));
    let m14 = {
        let (h10, _) = f30(Fn4::F31, {
            let a12 = {
                let a11 = v0;
                length_list(a11)
            };
            let a13 = 1u64;
            nat_add(a12, a13)?
        }, v1)?;
        h10
    };
    match m14.uncons() {
        None => {
            Ok(List::<(Bytes, Adt0)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f13() -> Adt12 {
    Adt12::C5
}

pub fn f14(v0: bool, v1: u64) -> R<Bytes> {
    if if v0 {
        let a15 = 0u64;
        let a16 = v1;
        nat_lt(a15, a16)
    } else {
        false
    } {
        let a17 = Bytes::lit(&[0x2d]);
        let a18 = f32(v1)?;
        Ok(append_bytes(a17, a18))
    } else {
        f32(v1)
    }
}

pub fn f15(v0: Bytes) -> R<Bytes> {
    let a24 = {
        let a22 = Bytes::lit(&[0x22]);
        let a23 = {
            let (_, h19) = {
                let (h20, _) = f33(Fn5::F34(v0.clone()), {
                    let a21 = v0;
                    length_bytes(a21)
                }, (0u64, Bytes::lit(&[])))?;
                h20
            };
            h19
        };
        append_bytes(a22, a23)
    };
    let a25 = Bytes::lit(&[0x22]);
    Ok(append_bytes(a24, a25))
}

pub fn f16(v0: List<Bytes>) -> Bytes {
    let a28 = {
        let a26 = Bytes::lit(&[0x5b]);
        let a27 = f35(v0, Bytes::lit(&[0x2c]));
        append_bytes(a26, a27)
    };
    let a29 = Bytes::lit(&[0x5d]);
    append_bytes(a28, a29)
}

pub fn f17(v0: Adt12) -> R<List<Bytes>> {
    let m30 = v0;
    match m30 {
        Adt12::C0(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt12::C1(_, _) => {
            Ok(List::<Bytes>::nil())
        }
        Adt12::C2(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt12::C3(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt12::C4(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt12::C5 => {
            Ok(List::<Bytes>::nil())
        }
        Adt12::C6(r31, r32) => {
            let v7 = (*r31).clone();
            let v8 = (*r32).clone();
            Ok(List::cons(f3(v7)?, f17(v8)?))
        }
        Adt12::C7(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f18(v0: List<(Bytes, Bytes)>) -> R<Bytes> {
    let v1: List<(Bytes, Bytes)> = f36(Fn6::F37, List::<(Bytes, Bytes)>::nil(), f38(v0)?);
    let v2: List<Bytes> = f39(Fn7::F40, List::<Bytes>::nil(), v1)?;
    let a35 = {
        let a33 = Bytes::lit(&[0x7b]);
        let a34 = f35(v2, Bytes::lit(&[0x2c]));
        append_bytes(a33, a34)
    };
    let a36 = Bytes::lit(&[0x7d]);
    Ok(append_bytes(a35, a36))
}

pub fn f19(v0: Adt12) -> R<List<(Bytes, Bytes)>> {
    let m37 = v0;
    match m37 {
        Adt12::C0(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt12::C1(_, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt12::C2(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt12::C3(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt12::C4(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt12::C5 => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt12::C6(_, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt12::C7(v9, r38, r39) => {
            let v10 = (*r38).clone();
            let v11 = (*r39).clone();
            Ok(List::cons((v9, f3(v10)?), f19(v11)?))
        }
    }
}

pub fn f20(v0: Str) -> Bytes {
    let a40 = v0;
    utf8_encode(a40)
}

pub fn f21(v0: Fn1, p44: Adt12, p45: List<Adt12>) -> Adt12 {
    let m43 = (p44, p45);
    {
        let mut m43 = m43;
        loop {
            let step = {
                let (v1, v2) = m43;
                let m41 = v2;
                match m41.uncons() {
                    None => {
                        Err::<(Adt12, List<Adt12>), Adt12>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt12, List<Adt12>), Adt12>(({
                            let c42: Fn1 = v0.clone();
                            c42.apply(v1, v3)
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

pub fn f22(v0: Adt12, v1: Adt12) -> Adt12 {
    Adt12::C6(std::rc::Rc::new(v1), std::rc::Rc::new(v0))
}

pub fn f23(v0: List<Adt12>) -> List<Adt12> {
    f41(Fn8::F42, List::<Adt12>::nil(), v0)
}

pub fn f24(v0: Adt0) -> R<Adt12> {
    let v1: (Adt12, Adt13) = f43(match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, _, h46, _, _, _, _) => {
            h46
        }
    }, f44())?;
    let v2: (Adt12, Adt13) = f45(match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, _, _, h47, _, _, _) => {
            h47
        }
    }, {
        let (_, h48) = v1;
        h48
    })?;
    Ok(f4(f5(Str::lit("name"), f46(match v0.clone() {
        Adt0::C0(h49, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h49
        }
    }), f5(Str::lit("lean_module"), f46(match v0.clone() {
        Adt0::C0(_, h50, _, _, _, _, _, _, _, _, _, _, _) => {
            h50
        }
    }), f5(Str::lit("glossary"), f47(match v0.clone() {
        Adt0::C0(_, _, _, _, _, h51, _, _, _, _, _, _, _) => {
            h51
        }
    }), f5(Str::lit("imports"), f47(match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, h52, _, _, _, _, _, _) => {
            h52
        }
    }), f5(Str::lit("title"), {
        let (h53, _) = v1;
        h53
    }, f5(Str::lit("blocks"), f7({
        let (h54, _) = v2;
        h54
    }), f48(v0)))))))))
}

pub fn f25(v0: Fn2, p58: List<(Bytes, Adt0)>, p59: List<(Bytes, Adt0)>) -> List<(Bytes, Adt0)> {
    let m57 = (p58, p59);
    {
        let mut m57 = m57;
        loop {
            let step = {
                let (v1, v2) = m57;
                let m55 = v2;
                match m55.uncons() {
                    None => {
                        Err::<(List<(Bytes, Adt0)>, List<(Bytes, Adt0)>), List<(Bytes, Adt0)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Adt0)>, List<(Bytes, Adt0)>), List<(Bytes, Adt0)>>(({
                            let c56: Fn2 = v0.clone();
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

pub fn f26(v0: List<(Bytes, Adt0)>, v1: (Bytes, Adt0)) -> List<(Bytes, Adt0)> {
    List::cons(v1, v0)
}

pub fn f27(v0: List<List<(Bytes, Adt0)>>) -> List<List<(Bytes, Adt0)>> {
    f49(Fn9::F50, List::<List<(Bytes, Adt0)>>::nil(), v0)
}

pub fn f28(v0: Fn3, p63: List<List<(Bytes, Adt0)>>, p64: List<(Bytes, Adt0)>) -> List<List<(Bytes, Adt0)>> {
    let m62 = (p63, p64);
    {
        let mut m62 = m62;
        loop {
            let step = {
                let (v1, v2) = m62;
                let m60 = v2;
                match m60.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Adt0)>>, List<(Bytes, Adt0)>), List<List<(Bytes, Adt0)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Adt0)>>, List<(Bytes, Adt0)>), List<List<(Bytes, Adt0)>>>(({
                            let c61: Fn3 = v0.clone();
                            c61.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m62 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f29(v0: List<List<(Bytes, Adt0)>>, v1: (Bytes, Adt0)) -> List<List<(Bytes, Adt0)>> {
    List::cons(List::cons(v1, List::<(Bytes, Adt0)>::nil()), v0)
}

pub fn f30(v0: Fn4, p69: u64, p70: List<List<(Bytes, Adt0)>>) -> R<(List<List<(Bytes, Adt0)>>, bool)> {
    let m68 = (p69, p70);
    {
        let mut m68 = m68;
        loop {
            let step = {
                let (v1, v2) = m68;
                let m65 = v1;
                if m65 == 0 {
                    Err::<(u64, List<List<(Bytes, Adt0)>>), R<(List<List<(Bytes, Adt0)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m65 - 1;
                    let m67 = {
                        let c66: Fn4 = v0.clone();
                        c66.apply(v2.clone())?
                    };
                    match m67 {
                        None => {
                            Err::<(u64, List<List<(Bytes, Adt0)>>), R<(List<List<(Bytes, Adt0)>>, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, List<List<(Bytes, Adt0)>>), R<(List<List<(Bytes, Adt0)>>, bool)>>((v3, v4))
                        }
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

pub fn f31(v0: List<List<(Bytes, Adt0)>>) -> R<Option<List<List<(Bytes, Adt0)>>>> {
    f51(v0)
}

pub fn f32(v0: u64) -> R<Bytes> {
    f52(f53(), 10u64, v0)
}

pub fn f33(v0: Fn5, p75: u64, p76: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m74 = (p75, p76);
    {
        let mut m74 = m74;
        loop {
            let step = {
                let (v1, v2) = m74;
                let m71 = v1;
                if m71 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m71 - 1;
                    let m73 = {
                        let c72: Fn5 = v0.clone();
                        c72.apply(v2.clone())?
                    };
                    match m73 {
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
                Ok(next) => { m74 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f34(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f54(v0, v1)
}

pub fn f35(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m77 = f55(Fn10::F56(v1), None::<Bytes>, v0);
    match m77 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f36(v0: Fn6, p81: List<(Bytes, Bytes)>, p82: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m80 = (p81, p82);
    {
        let mut m80 = m80;
        loop {
            let step = {
                let (v1, v2) = m80;
                let m78 = v2;
                match m78.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c79: Fn6 = v0.clone();
                            c79.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m80 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f37(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    f57(v0, v1)
}

pub fn f38(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    f58(v0)
}

pub fn f39(v0: Fn7, p86: List<Bytes>, p87: List<(Bytes, Bytes)>) -> R<List<Bytes>> {
    let m85 = (p86, p87);
    {
        let mut m85 = m85;
        loop {
            let step = {
                let (v1, v2) = m85;
                let m83 = v2;
                match m83.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(({
                            let c84: Fn7 = v0.clone();
                            c84.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m85 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f40(v0: List<Bytes>, v1: (Bytes, Bytes)) -> R<List<Bytes>> {
    Ok(List::cons(f59(v1)?, v0))
}

pub fn f41(v0: Fn8, p91: List<Adt12>, p92: List<Adt12>) -> List<Adt12> {
    let m90 = (p91, p92);
    {
        let mut m90 = m90;
        loop {
            let step = {
                let (v1, v2) = m90;
                let m88 = v2;
                match m88.uncons() {
                    None => {
                        Err::<(List<Adt12>, List<Adt12>), List<Adt12>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt12>, List<Adt12>), List<Adt12>>(({
                            let c89: Fn8 = v0.clone();
                            c89.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m90 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f42(v0: List<Adt12>, v1: Adt12) -> List<Adt12> {
    List::cons(v1, v0)
}

pub fn f43(v0: List<Adt1>, v1: Adt13) -> R<(Adt12, Adt13)> {
    let v2: (List<Adt12>, Adt13) = f60(Fn11::F61, (List::<Adt12>::nil(), v1), v0)?;
    Ok((f7(f21(Fn1::F62, f13(), {
        let (h93, _) = v2;
        h93
    })), {
        let (_, h94) = v2;
        h94
    }))
}

pub fn f44() -> Adt13 {
    Adt13::C0(List::<(u64, u64)>::nil(), 0u64, false)
}

pub fn f45(v0: Adt6, v1: Adt13) -> R<(Adt12, Adt13)> {
    let m95 = v0;
    match m95 {
        Adt6::C0(v2, v3, v4, r108) => {
            let v5 = (*r108).clone();
            let v6: (Adt12, Adt13) = f43(v3, v1)?;
            let v7: (Adt12, Adt13) = f63(v4, {
                let (_, h96) = v6;
                h96
            })?;
            let v8: (Adt12, Adt13) = f45(v5, {
                let (_, h97) = v7;
                h97
            })?;
            Ok((f4(f5(Str::lit("k"), f6(Str::lit("section")), f5(Str::lit("component"), f46(v2), f5(Str::lit("heading"), {
                let (h98, _) = v6;
                h98
            }, f5(Str::lit("params"), f7({
                let (h99, _) = v7;
                h99
            }), f5(Str::lit("blocks"), f7({
                let (h100, _) = v8;
                h100
            }), f13())))))), {
                let (_, h101) = v8;
                h101
            }))
        }
        Adt6::C1(v9) => {
            let v10: (Adt12, Adt13) = f64(v9, v1)?;
            Ok((f4(f5(Str::lit("k"), f6(Str::lit("declaration")), f5(Str::lit("d"), {
                let (h102, _) = v10;
                h102
            }, f13()))), {
                let (_, h103) = v10;
                h103
            }))
        }
        Adt6::C2 => {
            Ok((f13(), v1))
        }
        Adt6::C3(r109, r110) => {
            let v11 = (*r109).clone();
            let v12 = (*r110).clone();
            let v13: (Adt12, Adt13) = f45(v11, v1)?;
            let v14: (Adt12, Adt13) = f45(v12, {
                let (_, h104) = v13;
                h104
            })?;
            Ok((Adt12::C6(std::rc::Rc::new({
                let (h105, _) = v13;
                h105
            }), std::rc::Rc::new({
                let (h106, _) = v14;
                h106
            })), {
                let (_, h107) = v14;
                h107
            }))
        }
    }
}

pub fn f46(v0: Bytes) -> Adt12 {
    Adt12::C2(v0)
}

pub fn f47(v0: List<Bytes>) -> Adt12 {
    f7(f8(f65(Fn12::F66, List::<Adt12>::nil(), f67(v0))))
}

pub fn f48(v0: Adt0) -> Adt12 {
    let v2: Adt12 = {
        let m112 = match v0.clone() {
            Adt0::C0(_, _, _, _, _, _, _, _, _, _, _, _, h111) => {
                h111
            }
        };
        match m112 {
            None => {
                f13()
            }
            Some(v1) => {
                f5(Str::lit("tooling"), v1, f13())
            }
        }
    };
    let v4: Adt12 = {
        let m114 = match v0.clone() {
            Adt0::C0(_, _, _, _, _, _, _, _, _, _, _, h113, _) => {
                h113
            }
        };
        match m114 {
            None => {
                v2
            }
            Some(v3) => {
                f5(Str::lit("semantic"), v3, v2)
            }
        }
    };
    let v6: Adt12 = {
        let m116 = match v0.clone() {
            Adt0::C0(_, _, _, _, _, _, _, _, _, _, h115, _, _) => {
                h115
            }
        };
        match m116 {
            None => {
                v4
            }
            Some(v5) => {
                f5(Str::lit("core"), f46(v5), v4)
            }
        }
    };
    let v7: Adt12 = f68(v0.clone());
    let v19: Adt12 = {
        let m117 = v7.clone();
        match m117 {
            Adt12::C0(_) => {
                v6
            }
            Adt12::C1(_, _) => {
                v6
            }
            Adt12::C2(_) => {
                v6
            }
            Adt12::C3(_) => {
                v6
            }
            Adt12::C4(_) => {
                v6
            }
            Adt12::C5 => {
                v6
            }
            Adt12::C6(_, _) => {
                v6
            }
            Adt12::C7(_, _, _) => {
                f5(Str::lit("import_lean"), f4(v7), v6)
            }
        }
    };
    let m119 = match v0 {
        Adt0::C0(_, _, _, _, h118, _, _, _, _, _, _, _, _) => {
            h118
        }
    };
    match m119 {
        None => {
            v19
        }
        Some(v20) => {
            f5(Str::lit("origin"), f46(v20), v19)
        }
    }
}

pub fn f49(v0: Fn9, p123: List<List<(Bytes, Adt0)>>, p124: List<List<(Bytes, Adt0)>>) -> List<List<(Bytes, Adt0)>> {
    let m122 = (p123, p124);
    {
        let mut m122 = m122;
        loop {
            let step = {
                let (v1, v2) = m122;
                let m120 = v2;
                match m120.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>), List<List<(Bytes, Adt0)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>), List<List<(Bytes, Adt0)>>>(({
                            let c121: Fn9 = v0.clone();
                            c121.apply(v1, v3)
                        }, v4))
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

pub fn f50(v0: List<List<(Bytes, Adt0)>>, v1: List<(Bytes, Adt0)>) -> List<List<(Bytes, Adt0)>> {
    List::cons(v1, v0)
}

pub fn f51(v0: List<List<(Bytes, Adt0)>>) -> R<Option<List<List<(Bytes, Adt0)>>>> {
    let a126 = {
        let a125 = v0.clone();
        length_list(a125)
    };
    let a127 = 1u64;
    if nat_le(a126, a127) {
        Ok(None::<List<List<(Bytes, Adt0)>>>)
    } else {
        Ok(Some(f69(v0)?))
    }
}

pub fn f52(v0: Bytes, v1: u64, v2: u64) -> R<Bytes> {
    let a128 = v2;
    let a129 = 0u64;
    if nat_eq(a128, a129) {
        Ok(f70(v0, 0u64))
    } else {
        let (_, h130) = {
            let (h131, _) = f33(Fn5::F71(v1, v0), v2, (v2, Bytes::lit(&[])))?;
            h131
        };
        Ok(h130)
    }
}

pub fn f53() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f54(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let m135 = {
        let a133 = v0.clone();
        let a134 = {
            let (h132, _) = v1.clone();
            h132
        };
        index_bytes(a133, a134)
    };
    match m135 {
        None => {
            Ok(None::<(u64, Bytes)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a137 = {
                    let (h136, _) = v1.clone();
                    h136
                };
                let a138 = 1u64;
                nat_add(a137, a138)?
            }, {
                let a141 = {
                    let (_, h139) = v1;
                    h139
                };
                let a142 = f72(v0, {
                    let (h140, _) = v1;
                    h140
                }, v2)?;
                append_bytes(a141, a142)
            })))
        }
    }
}

pub fn f55(v0: Fn10, p146: Option<Bytes>, p147: List<Bytes>) -> Option<Bytes> {
    let m145 = (p146, p147);
    {
        let mut m145 = m145;
        loop {
            let step = {
                let (v1, v2) = m145;
                let m143 = v2;
                match m143.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(({
                            let c144: Fn10 = v0.clone();
                            c144.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m145 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f56(v0: Bytes, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m148 = v1;
    match m148 {
        None => {
            Some(v2)
        }
        Some(v3) => {
            Some({
                let a151 = {
                    let a149 = v3;
                    let a150 = v0;
                    append_bytes(a149, a150)
                };
                let a152 = v2;
                append_bytes(a151, a152)
            })
        }
    }
}

pub fn f57(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    let m153 = v0.clone();
    match m153.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, v3)) => {
            let a156 = {
                let (h154, _) = v2;
                h154
            };
            let a157 = {
                let (h155, _) = v1.clone();
                h155
            };
            if equal(a156, a157) {
                List::cons(v1, v3)
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f58(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v1: List<List<(Bytes, Bytes)>> = f73(f74(Fn13::F75, List::<List<(Bytes, Bytes)>>::nil(), v0.clone()));
    let m162 = {
        let (h158, _) = f76(Fn14::F77, {
            let a160 = {
                let a159 = v0;
                length_list(a159)
            };
            let a161 = 1u64;
            nat_add(a160, a161)?
        }, v1)?;
        h158
    };
    match m162.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f59(v0: (Bytes, Bytes)) -> R<Bytes> {
    let a167 = {
        let a164 = f15({
            let (h163, _) = v0;
            h163
        })?;
        let a165 = Bytes::lit(&[0x3a]);
        append_bytes(a164, a165)
    };
    let a168 = {
        let (_, h166) = v0;
        h166
    };
    Ok(append_bytes(a167, a168))
}

pub fn f60(v0: Fn11, p172: (List<Adt12>, Adt13), p173: List<Adt1>) -> R<(List<Adt12>, Adt13)> {
    let m171 = (p172, p173);
    {
        let mut m171 = m171;
        loop {
            let step = {
                let (v1, v2) = m171;
                let m169 = v2;
                match m169.uncons() {
                    None => {
                        Err::<((List<Adt12>, Adt13), List<Adt1>), R<(List<Adt12>, Adt13)>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<((List<Adt12>, Adt13), List<Adt1>), R<(List<Adt12>, Adt13)>>(({
                            let c170: Fn11 = v0.clone();
                            c170.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m171 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f61(v0: (List<Adt12>, Adt13), v1: Adt1) -> R<(List<Adt12>, Adt13)> {
    let v2: (Adt12, Adt13) = f78(v1, {
        let (_, h174) = v0;
        h174
    })?;
    Ok((List::cons({
        let (h175, _) = v2;
        h175
    }, {
        let (h176, _) = v0;
        h176
    }), {
        let (_, h177) = v2;
        h177
    }))
}

pub fn f62(v0: Adt12, v1: Adt12) -> Adt12 {
    Adt12::C6(std::rc::Rc::new(v1), std::rc::Rc::new(v0))
}

pub fn f63(v0: Adt2, v1: Adt13) -> R<(Adt12, Adt13)> {
    let m178 = v0;
    match m178 {
        Adt2::C0(v2) => {
            Ok((Adt12::C4(std::rc::Rc::new(f5(Str::lit("k"), f6(Str::lit("sort")), f5(Str::lit("u"), f79(v2), Adt12::C5)))), v1))
        }
        Adt2::C1(v3) => {
            let v4: (Adt14, Adt13) = f80(v1, v3)?;
            Ok((f81({
                let (h179, _) = v4;
                h179
            }), {
                let (_, h180) = v4;
                h180
            }))
        }
        Adt2::C2(v5, v6) => {
            Ok((f82(v5, v6)?, v1))
        }
        Adt2::C3(r213, r214, v9) => {
            let v7 = (*r213).clone();
            let v8 = (*r214).clone();
            let v10: (Adt12, Adt13) = f63(v7, v1)?;
            let v11: (Adt12, Adt13) = f63(v8, {
                let (_, h181) = v10;
                h181
            })?;
            Ok((f83({
                let (h182, _) = v10;
                h182
            }, {
                let (h183, _) = v11;
                h183
            }, v9), {
                let (_, h184) = v11;
                h184
            }))
        }
        Adt2::C4(r215, r216) => {
            let v12 = (*r215).clone();
            let v13 = (*r216).clone();
            let v14: (Adt12, Adt13) = f63(v12, v1)?;
            let v15: (Adt12, Adt13) = f63(v13, {
                let (_, h185) = v14;
                h185
            })?;
            Ok((Adt12::C4(std::rc::Rc::new(f5(Str::lit("k"), f6(Str::lit("pi")), f5(Str::lit("b"), Adt12::C3(std::rc::Rc::new({
                let (h186, _) = v14;
                h186
            })), f5(Str::lit("v"), {
                let (h187, _) = v15;
                h187
            }, Adt12::C5))))), {
                let (_, h188) = v15;
                h188
            }))
        }
        Adt2::C5(r217, r218) => {
            let v16 = (*r217).clone();
            let v17 = (*r218).clone();
            let v18: (Adt12, Adt13) = f63(v16, v1)?;
            let v19: (Adt12, Adt13) = f63(v17, {
                let (_, h189) = v18;
                h189
            })?;
            Ok((Adt12::C4(std::rc::Rc::new(f5(Str::lit("k"), f6(Str::lit("lam")), f5(Str::lit("b"), Adt12::C3(std::rc::Rc::new({
                let (h190, _) = v18;
                h190
            })), f5(Str::lit("v"), {
                let (h191, _) = v19;
                h191
            }, Adt12::C5))))), {
                let (_, h192) = v19;
                h192
            }))
        }
        Adt2::C6(v20, v21, v22, r219, r220, r221) => {
            let v23 = (*r219).clone();
            let v24 = (*r220).clone();
            let v25 = (*r221).clone();
            let v26: (Adt12, Adt13) = f63(v23, v1)?;
            let v27: (Adt12, Adt13) = f63(v24, {
                let (_, h193) = v26;
                h193
            })?;
            let v28: (u64, Adt13) = f84({
                let (_, h194) = v27;
                h194
            }, v20)?;
            let v29: (Adt12, Adt13) = f63(v25, {
                let (_, h195) = v28;
                h195
            })?;
            Ok((Adt12::C4(std::rc::Rc::new(f5(Str::lit("k"), f6(Str::lit("let")), f5(Str::lit("b"), f85({
                let (h196, _) = v28;
                h196
            }, v21, v22, {
                let (h197, _) = v26;
                h197
            }), f5(Str::lit("x"), {
                let (h198, _) = v27;
                h198
            }, f5(Str::lit("v"), {
                let (h199, _) = v29;
                h199
            }, Adt12::C5)))))), {
                let (_, h200) = v29;
                h200
            }))
        }
        Adt2::C7(v30, r222) => {
            let v31 = (*r222).clone();
            let v32: (Adt12, Adt13) = f63(v31, v1)?;
            Ok((Adt12::C4(std::rc::Rc::new(f5(Str::lit("k"), f6(Str::lit("nat")), f5(Str::lit("d"), Adt12::C2(v30), f5(Str::lit("t"), {
                let (h201, _) = v32;
                h201
            }, Adt12::C5))))), {
                let (_, h202) = v32;
                h202
            }))
        }
        Adt2::C8 => {
            Ok((Adt12::C5, v1))
        }
        Adt2::C9(r223, r224) => {
            let v33 = (*r223).clone();
            let v34 = (*r224).clone();
            let v35: (Adt12, Adt13) = f63(v33, v1)?;
            let v36: (Adt12, Adt13) = f63(v34, {
                let (_, h203) = v35;
                h203
            })?;
            Ok((Adt12::C6(std::rc::Rc::new({
                let (h204, _) = v35;
                h204
            }), std::rc::Rc::new({
                let (h205, _) = v36;
                h205
            })), {
                let (_, h206) = v36;
                h206
            }))
        }
        Adt2::C10(v37, v38, v39, r225, r226) => {
            let v40 = (*r225).clone();
            let v41 = (*r226).clone();
            let v42: (Adt12, Adt13) = f63(v40, v1)?;
            let v43: (u64, Adt13) = f84({
                let (_, h207) = v42;
                h207
            }, v37)?;
            let v44: (Adt12, Adt13) = f63(v41, {
                let (_, h208) = v43;
                h208
            })?;
            Ok((Adt12::C6(std::rc::Rc::new(f85({
                let (h209, _) = v43;
                h209
            }, v38, v39, {
                let (h210, _) = v42;
                h210
            })), std::rc::Rc::new({
                let (h211, _) = v44;
                h211
            })), {
                let (_, h212) = v44;
                h212
            }))
        }
    }
}

pub fn f64(v0: Adt7, v1: Adt13) -> R<(Adt12, Adt13)> {
    let v2: (Adt12, Adt13) = f63(match v0.clone() {
        Adt7::C0(_, _, _, h227, _, _) => {
            h227
        }
    }, v1)?;
    let v3: (Adt12, Adt13) = f86(match v0.clone() {
        Adt7::C0(_, _, _, _, h228, _) => {
            h228
        }
    }, {
        let (_, h229) = v2;
        h229
    })?;
    Ok((f87(f5(Str::lit("component"), Adt12::C2(match v0.clone() {
        Adt7::C0(h230, _, _, _, _, _) => {
            h230
        }
    }), f5(Str::lit("lean_name"), Adt12::C2(match v0.clone() {
        Adt7::C0(_, h231, _, _, _, _) => {
            h231
        }
    }), f5(Str::lit("kind"), f88(match v0.clone() {
        Adt7::C0(_, _, h232, _, _, _) => {
            h232
        }
    }), f5(Str::lit("params"), f89({
        let (h233, _) = v2;
        h233
    }), f5(Str::lit("body"), {
        let (h234, _) = v3;
        h234
    }, f5(Str::lit("policy"), f90(match v0 {
        Adt7::C0(_, _, _, _, _, h235) => {
            h235
        }
    }), f91()))))))), {
        let (_, h236) = v3;
        h236
    }))
}

pub fn f65(v0: Fn12, p240: List<Adt12>, p241: List<Bytes>) -> List<Adt12> {
    let m239 = (p240, p241);
    {
        let mut m239 = m239;
        loop {
            let step = {
                let (v1, v2) = m239;
                let m237 = v2;
                match m237.uncons() {
                    None => {
                        Err::<(List<Adt12>, List<Bytes>), List<Adt12>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt12>, List<Bytes>), List<Adt12>>(({
                            let c238: Fn12 = v0.clone();
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

pub fn f66(v0: List<Adt12>, v1: Bytes) -> List<Adt12> {
    List::cons(f46(v1), v0)
}

pub fn f67(v0: List<Bytes>) -> List<Bytes> {
    f92(Fn15::F93, List::<Bytes>::nil(), v0)
}

pub fn f68(v0: Adt0) -> Adt12 {
    f94(Fn16::F95(v0.clone()), f13(), match v0 {
        Adt0::C0(_, _, _, _, _, _, _, h242, _, _, _, _, _) => {
            h242
        }
    })
}

pub fn f69(v0: List<List<(Bytes, Adt0)>>) -> R<List<List<(Bytes, Adt0)>>> {
    Ok(f27({
        let (_, h243) = {
            let (h244, _) = f96(Fn17::F97, {
                let a246 = {
                    let a245 = v0.clone();
                    length_list(a245)
                };
                let a247 = 1u64;
                nat_add(a246, a247)?
            }, (v0, List::<List<(Bytes, Adt0)>>::nil()))?;
            h244
        };
        h243
    }))
}

pub fn f70(v0: Bytes, v1: u64) -> Bytes {
    let m251 = {
        let a248 = v0;
        let a249 = v1;
        let a250 = 1u64;
        slice_bytes(a248, a249, a250)
    };
    match m251 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f71(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f98(v1, v0, v2)
}

pub fn f72(v0: Bytes, v1: u64, v2: u8) -> R<Bytes> {
    let v4: u64 = {
        let v3: u8 = v2;
        let a296 = {
            let a272 = {
                let a260 = {
                    let a254 = {
                        let a252 = v3;
                        let a253 = 128u8;
                        fixed_u8::bit_and(a252, a253)
                    };
                    let a255 = 128u8;
                    if equal(a254, a255) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a261 = {
                    let a258 = {
                        let a256 = v3;
                        let a257 = 64u8;
                        fixed_u8::bit_and(a256, a257)
                    };
                    let a259 = 64u8;
                    if equal(a258, a259) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a260, a261)?
            };
            let a273 = {
                let a270 = {
                    let a264 = {
                        let a262 = v3;
                        let a263 = 32u8;
                        fixed_u8::bit_and(a262, a263)
                    };
                    let a265 = 32u8;
                    if equal(a264, a265) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a271 = {
                    let a268 = {
                        let a266 = v3;
                        let a267 = 16u8;
                        fixed_u8::bit_and(a266, a267)
                    };
                    let a269 = 16u8;
                    if equal(a268, a269) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a270, a271)?
            };
            nat_add(a272, a273)?
        };
        let a297 = {
            let a294 = {
                let a282 = {
                    let a276 = {
                        let a274 = v3;
                        let a275 = 8u8;
                        fixed_u8::bit_and(a274, a275)
                    };
                    let a277 = 8u8;
                    if equal(a276, a277) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a283 = {
                    let a280 = {
                        let a278 = v3;
                        let a279 = 4u8;
                        fixed_u8::bit_and(a278, a279)
                    };
                    let a281 = 4u8;
                    if equal(a280, a281) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a282, a283)?
            };
            let a295 = {
                let a292 = {
                    let a286 = {
                        let a284 = v3;
                        let a285 = 2u8;
                        fixed_u8::bit_and(a284, a285)
                    };
                    let a287 = 2u8;
                    if equal(a286, a287) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a293 = {
                    let a290 = {
                        let a288 = v3;
                        let a289 = 1u8;
                        fixed_u8::bit_and(a288, a289)
                    };
                    let a291 = 1u8;
                    if equal(a290, a291) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a292, a293)?
            };
            nat_add(a294, a295)?
        };
        nat_add(a296, a297)?
    };
    let a298 = v4;
    let a299 = 34u64;
    if nat_eq(a298, a299) {
        Ok(Bytes::lit(&[0x5c, 0x22]))
    } else {
        let a300 = v4;
        let a301 = 92u64;
        if nat_eq(a300, a301) {
            Ok(Bytes::lit(&[0x5c, 0x5c]))
        } else {
            let a302 = v4;
            let a303 = 8u64;
            if nat_eq(a302, a303) {
                Ok(Bytes::lit(&[0x5c, 0x62]))
            } else {
                let a304 = v4;
                let a305 = 12u64;
                if nat_eq(a304, a305) {
                    Ok(Bytes::lit(&[0x5c, 0x66]))
                } else {
                    let a306 = v4;
                    let a307 = 10u64;
                    if nat_eq(a306, a307) {
                        Ok(Bytes::lit(&[0x5c, 0x6e]))
                    } else {
                        let a308 = v4;
                        let a309 = 13u64;
                        if nat_eq(a308, a309) {
                            Ok(Bytes::lit(&[0x5c, 0x72]))
                        } else {
                            let a310 = v4;
                            let a311 = 9u64;
                            if nat_eq(a310, a311) {
                                Ok(Bytes::lit(&[0x5c, 0x74]))
                            } else {
                                let a312 = v4;
                                let a313 = 32u64;
                                if nat_lt(a312, a313) {
                                    let a314 = Bytes::lit(&[0x5c, 0x75, 0x30, 0x30]);
                                    let a315 = f99(v2)?;
                                    Ok(append_bytes(a314, a315))
                                } else {
                                    Ok(f100(v0, v1, 1u64))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f73(v0: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    f101(Fn18::F102, List::<List<(Bytes, Bytes)>>::nil(), v0)
}

pub fn f74(v0: Fn13, p319: List<List<(Bytes, Bytes)>>, p320: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    let m318 = (p319, p320);
    {
        let mut m318 = m318;
        loop {
            let step = {
                let (v1, v2) = m318;
                let m316 = v2;
                match m316.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(({
                            let c317: Fn13 = v0.clone();
                            c317.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m318 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f75(v0: List<List<(Bytes, Bytes)>>, v1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
    List::cons(List::cons(v1, List::<(Bytes, Bytes)>::nil()), v0)
}

pub fn f76(v0: Fn14, p325: u64, p326: List<List<(Bytes, Bytes)>>) -> R<(List<List<(Bytes, Bytes)>>, bool)> {
    let m324 = (p325, p326);
    {
        let mut m324 = m324;
        loop {
            let step = {
                let (v1, v2) = m324;
                let m321 = v1;
                if m321 == 0 {
                    Err::<(u64, List<List<(Bytes, Bytes)>>), R<(List<List<(Bytes, Bytes)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m321 - 1;
                    let m323 = {
                        let c322: Fn14 = v0.clone();
                        c322.apply(v2.clone())?
                    };
                    match m323 {
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
                Ok(next) => { m324 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f77(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    f103(v0)
}

pub fn f78(v0: Adt1, v1: Adt13) -> R<(Adt12, Adt13)> {
    let m327 = v0;
    match m327 {
        Adt1::C0(v2, v3) => {
            Ok((f4(f5(Str::lit("k"), f6(Str::lit("word")), f5(Str::lit("entry"), f46(v2), f5(Str::lit("form"), f46(v3), f13())))), v1))
        }
        Adt1::C1(v4) => {
            let v5: (Adt12, Adt13) = f63(v4, v1)?;
            Ok((f4(f5(Str::lit("k"), f6(Str::lit("math")), f5(Str::lit("t"), {
                let (h328, _) = v5;
                h328
            }, f13()))), {
                let (_, h329) = v5;
                h329
            }))
        }
        Adt1::C2(v6) => {
            Ok((f4(f5(Str::lit("k"), f6(Str::lit("punct")), f5(Str::lit("entry"), f46(v6), f13()))), v1))
        }
    }
}

pub fn f79(v0: Adt3) -> Adt12 {
    let m330 = v0;
    match m330 {
        Adt3::C0(v1) => {
            Adt12::C4(std::rc::Rc::new(f5(Str::lit("num"), f104(v1), Adt12::C5)))
        }
        Adt3::C1(v2) => {
            Adt12::C4(std::rc::Rc::new(f5(Str::lit("var"), Adt12::C2(v2), Adt12::C5)))
        }
        Adt3::C2(r331) => {
            let v3 = (*r331).clone();
            Adt12::C4(std::rc::Rc::new(f5(Str::lit("succ"), f79(v3), Adt12::C5)))
        }
        Adt3::C3(r332) => {
            let v4 = (*r332).clone();
            Adt12::C4(std::rc::Rc::new(f5(Str::lit("max"), Adt12::C3(std::rc::Rc::new(f79(v4))), Adt12::C5)))
        }
        Adt3::C4(r333, r334) => {
            let v5 = (*r333).clone();
            let v6 = (*r334).clone();
            Adt12::C4(std::rc::Rc::new(f5(Str::lit("imax"), Adt12::C3(std::rc::Rc::new(Adt12::C6(std::rc::Rc::new(f79(v5)), std::rc::Rc::new(Adt12::C6(std::rc::Rc::new(f79(v6)), std::rc::Rc::new(Adt12::C5)))))), Adt12::C5)))
        }
        Adt3::C5 => {
            Adt12::C5
        }
        Adt3::C6(r335, r336) => {
            let v7 = (*r335).clone();
            let v8 = (*r336).clone();
            Adt12::C6(std::rc::Rc::new(f79(v7)), std::rc::Rc::new(f79(v8)))
        }
    }
}

pub fn f80(v0: Adt13, v1: u64) -> R<(Adt14, Adt13)> {
    let m338 = f105(match v0.clone() {
        Adt13::C0(h337, _, _) => {
            h337
        }
    }, v1);
    match m338 {
        None => {
            if match v0.clone() {
                Adt13::C0(_, _, h339) => {
                    h339
                }
            } {
                Ok((Adt14::C1(v1), v0))
            } else {
                let v3: (u64, Adt13) = f106(v0, v1)?;
                Ok((Adt14::C0({
                    let (h340, _) = v3;
                    h340
                }), {
                    let (_, h341) = v3;
                    h341
                }))
            }
        }
        Some(v2) => {
            Ok((Adt14::C0(v2), v0))
        }
    }
}

pub fn f81(v0: Adt14) -> Adt12 {
    let m342 = v0;
    match m342 {
        Adt14::C0(v1) => {
            Adt12::C4(std::rc::Rc::new(f5(Str::lit("k"), f6(Str::lit("local")), f5(Str::lit("id"), f104(v1), Adt12::C5))))
        }
        Adt14::C1(v2) => {
            Adt12::C4(std::rc::Rc::new(f5(Str::lit("k"), f6(Str::lit("local")), f5(Str::lit("free"), f104(v2), Adt12::C5))))
        }
    }
}

pub fn f82(v0: Adt4, v1: Adt3) -> R<Adt12> {
    let v2: Adt12 = if f107(v1.clone()) {
        Adt12::C5
    } else {
        f5(Str::lit("u"), Adt12::C3(std::rc::Rc::new(f79(v1))), Adt12::C5)
    };
    Ok(Adt12::C4(std::rc::Rc::new(f5(Str::lit("k"), f6(Str::lit("global")), f5(Str::lit("g"), f108(v0)?, v2)))))
}

pub fn f83(v0: Adt12, v1: Adt12, v2: List<u64>) -> Adt12 {
    let v5: Adt12 = {
        let m343 = v2.clone();
        match m343.uncons() {
            None => {
                Adt12::C5
            }
            Some((_, _)) => {
                f5(Str::lit("i"), Adt12::C3(std::rc::Rc::new(f109(v2))), Adt12::C5)
            }
        }
    };
    Adt12::C4(std::rc::Rc::new(f5(Str::lit("k"), f6(Str::lit("app")), f5(Str::lit("f"), v0, f5(Str::lit("a"), Adt12::C3(std::rc::Rc::new(v1)), v5)))))
}

pub fn f84(v0: Adt13, v1: u64) -> R<(u64, Adt13)> {
    let m345 = f105(match v0.clone() {
        Adt13::C0(h344, _, _) => {
            h344
        }
    }, v1);
    match m345 {
        None => {
            f106(v0, v1)
        }
        Some(v2) => {
            Ok((v2, v0))
        }
    }
}

pub fn f85(v0: u64, v1: u64, v2: Bytes, v3: Adt12) -> Adt12 {
    Adt12::C4(std::rc::Rc::new(f5(Str::lit("id"), f104(v0), f5(Str::lit("m"), f110(v1), f5(Str::lit("s"), Adt12::C2(v2), f5(Str::lit("t"), v3, Adt12::C5))))))
}

pub fn f86(v0: Adt9, v1: Adt13) -> R<(Adt12, Adt13)> {
    let m346 = v0;
    match m346 {
        Adt9::C0(v2, v3, v4) => {
            let v5: (Adt12, Adt13) = f63(v3, v1)?;
            let v6: (Adt12, Adt13) = f63(v4, {
                let (_, h347) = v5;
                h347
            })?;
            Ok((f111(Str::lit("definition"), f5(Str::lit("entry"), Adt12::C2(v2), f5(Str::lit("t"), {
                let (h348, _) = v5;
                h348
            }, f5(Str::lit("v"), {
                let (h349, _) = v6;
                h349
            }, f91())))), {
                let (_, h350) = v6;
                h350
            }))
        }
        Adt9::C1(v7, v8) => {
            let v9: (Adt12, Adt13) = f63(v7, v1)?;
            let v10: (Adt12, Adt13) = f112(v8, {
                let (_, h351) = v9;
                h351
            })?;
            Ok((f111(Str::lit("theorem"), f5(Str::lit("s"), {
                let (h352, _) = v9;
                h352
            }, f5(Str::lit("p"), {
                let (h353, _) = v10;
                h353
            }, f91()))), {
                let (_, h354) = v10;
                h354
            }))
        }
    }
}

pub fn f87(v0: Adt12) -> Adt12 {
    Adt12::C4(std::rc::Rc::new(v0))
}

pub fn f88(v0: Adt8) -> Adt12 {
    let m355 = v0;
    match m355 {
        Adt8::C0 => {
            f6(Str::lit("typedefinition"))
        }
        Adt8::C1 => {
            f6(Str::lit("termdefinition"))
        }
        Adt8::C2 => {
            f6(Str::lit("predicatedefinition"))
        }
        Adt8::C3 => {
            f6(Str::lit("theorem"))
        }
        Adt8::C4 => {
            f6(Str::lit("lemma"))
        }
        Adt8::C5 => {
            f6(Str::lit("corollary"))
        }
    }
}

pub fn f89(v0: Adt12) -> Adt12 {
    Adt12::C3(std::rc::Rc::new(v0))
}

pub fn f90(v0: Adt11) -> Adt12 {
    let m356 = v0;
    match m356 {
        Adt11::C0 => {
            f87(f5(Str::lit("kind"), f6(Str::lit("none")), f5(Str::lit("axioms"), f89(f91()), f91())))
        }
        Adt11::C1(v1) => {
            f87(f5(Str::lit("kind"), f6(Str::lit("allow")), f5(Str::lit("axioms"), f89(f113(v1)), f91())))
        }
        Adt11::C2(v2) => {
            f87(f5(Str::lit("kind"), f6(Str::lit("exact")), f5(Str::lit("axioms"), f89(f113(v2)), f91())))
        }
    }
}

pub fn f91() -> Adt12 {
    Adt12::C5
}

pub fn f92(v0: Fn15, p360: List<Bytes>, p361: List<Bytes>) -> List<Bytes> {
    let m359 = (p360, p361);
    {
        let mut m359 = m359;
        loop {
            let step = {
                let (v1, v2) = m359;
                let m357 = v2;
                match m357.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c358: Fn15 = v0.clone();
                            c358.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m359 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f93(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f94(v0: Fn16, p365: Adt12, p366: List<(Bytes, Bytes)>) -> Adt12 {
    let m364 = (p365, p366);
    {
        let mut m364 = m364;
        loop {
            let step = {
                let (v1, v2) = m364;
                let m362 = v2;
                match m362.uncons() {
                    None => {
                        Err::<(Adt12, List<(Bytes, Bytes)>), Adt12>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt12, List<(Bytes, Bytes)>), Adt12>(({
                            let c363: Fn16 = v0.clone();
                            c363.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m364 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f95(v0: Adt0, v1: Adt12, v2: (Bytes, Bytes)) -> Adt12 {
    let a369 = {
        let (_, h367) = v2.clone();
        h367
    };
    let a370 = f114(v0, {
        let (h368, _) = v2.clone();
        h368
    });
    if equal(a369, a370) {
        v1
    } else {
        Adt12::C7({
            let (h371, _) = v2;
            h371
        }, std::rc::Rc::new(f46({
            let (_, h372) = v2;
            h372
        })), std::rc::Rc::new(v1))
    }
}

pub fn f96(v0: Fn17, p377: u64, p378: (List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>)) -> R<((List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>), bool)> {
    let m376 = (p377, p378);
    {
        let mut m376 = m376;
        loop {
            let step = {
                let (v1, v2) = m376;
                let m373 = v1;
                if m373 == 0 {
                    Err::<(u64, (List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>)), R<((List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m373 - 1;
                    let m375 = {
                        let c374: Fn17 = v0.clone();
                        c374.apply(v2.clone())?
                    };
                    match m375 {
                        None => {
                            Err::<(u64, (List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>)), R<((List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>)), R<((List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m376 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f97(v0: (List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>)) -> R<Option<(List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>)>> {
    f115(v0)
}

pub fn f98(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h379, _) = v2;
        h379
    };
    let a380 = v3;
    let a381 = 0u64;
    if nat_eq(a380, a381) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a382 = v3;
            let a383 = v1;
            let a384 = 0u64;
            nat_quot(a382, a383, a384)
        }, {
            let a389 = f70(v0, {
                let a385 = v3;
                let a386 = v1;
                let a387 = 0u64;
                nat_rem(a385, a386, a387)
            });
            let a390 = {
                let (_, h388) = v2;
                h388
            };
            append_bytes(a389, a390)
        }))
    }
}

pub fn f99(v0: u8) -> R<Bytes> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a435 = {
            let a411 = {
                let a399 = {
                    let a393 = {
                        let a391 = v1;
                        let a392 = 128u8;
                        fixed_u8::bit_and(a391, a392)
                    };
                    let a394 = 128u8;
                    if equal(a393, a394) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a400 = {
                    let a397 = {
                        let a395 = v1;
                        let a396 = 64u8;
                        fixed_u8::bit_and(a395, a396)
                    };
                    let a398 = 64u8;
                    if equal(a397, a398) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a399, a400)?
            };
            let a412 = {
                let a409 = {
                    let a403 = {
                        let a401 = v1;
                        let a402 = 32u8;
                        fixed_u8::bit_and(a401, a402)
                    };
                    let a404 = 32u8;
                    if equal(a403, a404) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a410 = {
                    let a407 = {
                        let a405 = v1;
                        let a406 = 16u8;
                        fixed_u8::bit_and(a405, a406)
                    };
                    let a408 = 16u8;
                    if equal(a407, a408) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a409, a410)?
            };
            nat_add(a411, a412)?
        };
        let a436 = {
            let a433 = {
                let a421 = {
                    let a415 = {
                        let a413 = v1;
                        let a414 = 8u8;
                        fixed_u8::bit_and(a413, a414)
                    };
                    let a416 = 8u8;
                    if equal(a415, a416) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a422 = {
                    let a419 = {
                        let a417 = v1;
                        let a418 = 4u8;
                        fixed_u8::bit_and(a417, a418)
                    };
                    let a420 = 4u8;
                    if equal(a419, a420) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a421, a422)?
            };
            let a434 = {
                let a431 = {
                    let a425 = {
                        let a423 = v1;
                        let a424 = 2u8;
                        fixed_u8::bit_and(a423, a424)
                    };
                    let a426 = 2u8;
                    if equal(a425, a426) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a432 = {
                    let a429 = {
                        let a427 = v1;
                        let a428 = 1u8;
                        fixed_u8::bit_and(a427, a428)
                    };
                    let a430 = 1u8;
                    if equal(a429, a430) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a431, a432)?
            };
            nat_add(a433, a434)?
        };
        nat_add(a435, a436)?
    };
    let a443 = f70(f116(), {
        let a437 = v2;
        let a438 = 16u64;
        let a439 = 0u64;
        nat_quot(a437, a438, a439)
    });
    let a444 = f70(f116(), {
        let a440 = v2;
        let a441 = 16u64;
        let a442 = 0u64;
        nat_rem(a440, a441, a442)
    });
    Ok(append_bytes(a443, a444))
}

pub fn f100(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m448 = {
        let a445 = v0;
        let a446 = v1;
        let a447 = v2;
        slice_bytes(a445, a446, a447)
    };
    match m448 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f101(v0: Fn18, p452: List<List<(Bytes, Bytes)>>, p453: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    let m451 = (p452, p453);
    {
        let mut m451 = m451;
        loop {
            let step = {
                let (v1, v2) = m451;
                let m449 = v2;
                match m449.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(({
                            let c450: Fn18 = v0.clone();
                            c450.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m451 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f102(v0: List<List<(Bytes, Bytes)>>, v1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    List::cons(v1, v0)
}

pub fn f103(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    let a455 = {
        let a454 = v0.clone();
        length_list(a454)
    };
    let a456 = 1u64;
    if nat_le(a455, a456) {
        Ok(None::<List<List<(Bytes, Bytes)>>>)
    } else {
        Ok(Some(f117(v0)?))
    }
}

pub fn f104(v0: u64) -> Adt12 {
    Adt12::C1(false, f118(v0))
}

pub fn f105(v0: List<(u64, u64)>, v1: u64) -> Option<u64> {
    f119(Fn19::F120(v1), None::<u64>, v0)
}

pub fn f106(v0: Adt13, v1: u64) -> R<(u64, Adt13)> {
    Ok((match v0.clone() {
        Adt13::C0(_, h457, _) => {
            h457
        }
    }, Adt13::C0(List::cons((v1, match v0.clone() {
        Adt13::C0(_, h458, _) => {
            h458
        }
    }), match v0.clone() {
        Adt13::C0(h459, _, _) => {
            h459
        }
    }), {
        let a461 = match v0.clone() {
            Adt13::C0(_, h460, _) => {
                h460
            }
        };
        let a462 = 1u64;
        nat_add(a461, a462)?
    }, match v0 {
        Adt13::C0(_, _, h463) => {
            h463
        }
    })))
}

pub fn f107(v0: Adt3) -> bool {
    let m464 = v0;
    match m464 {
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

pub fn f108(v0: Adt4) -> R<Adt12> {
    let m465 = v0;
    match m465 {
        Adt4::C0(v1) => {
            Ok(Adt12::C4(std::rc::Rc::new(f5(Str::lit("kind"), f6(Str::lit("core")), f5(Str::lit("name"), Adt12::C2(f121(v1)), Adt12::C5)))))
        }
        Adt4::C1(_, v3, v4, v5, v6) => {
            Ok(Adt12::C4(std::rc::Rc::new(f5(Str::lit("kind"), f6(Str::lit("external")), f5(Str::lit("entry"), Adt12::C2(v3), f5(Str::lit("lean_module"), Adt12::C2(v4), f5(Str::lit("lean_name"), Adt12::C2(v5), f5(Str::lit("signature_sha256"), Adt12::C2(f122(v6)?), Adt12::C5))))))))
        }
        Adt4::C2(v7, v8, v9) => {
            Ok(Adt12::C4(std::rc::Rc::new(f5(Str::lit("kind"), f6(Str::lit("document")), f5(Str::lit("module"), Adt12::C2(v7), f5(Str::lit("component"), Adt12::C2(v8), f5(Str::lit("lean_name"), Adt12::C2(v9), Adt12::C5)))))))
        }
        Adt4::C3(_, v11, v12) => {
            Ok(Adt12::C4(std::rc::Rc::new(f5(Str::lit("kind"), f6(Str::lit("defined")), f5(Str::lit("entry"), Adt12::C2(v11), f5(Str::lit("signature_sha256"), Adt12::C2(f122(v12)?), Adt12::C5))))))
        }
    }
}

pub fn f109(v0: List<u64>) -> Adt12 {
    f123(Fn20::F124, Adt12::C5, f125(v0))
}

pub fn f110(v0: u64) -> Adt12 {
    Adt12::C2(f126(v0))
}

pub fn f111(v0: Str, v1: Adt12) -> Adt12 {
    f87(f5(Str::lit("k"), f6(v0), v1))
}

pub fn f112(v0: Adt10, v1: Adt13) -> R<(Adt12, Adt13)> {
    let m466 = v0;
    match m466 {
        Adt10::C0(r529) => {
            let v2 = (*r529).clone();
            let v3: (Adt12, Adt13) = f112(v2, v1)?;
            Ok((f111(Str::lit("seq"), f5(Str::lit("s"), f89({
                let (h467, _) = v3;
                h467
            }), f91())), {
                let (_, h468) = v3;
                h468
            }))
        }
        Adt10::C1(v4) => {
            let v5: (Adt12, Adt13) = f127(v4, v1)?;
            Ok((f111(Str::lit("intro"), f5(Str::lit("l"), f89({
                let (h469, _) = v5;
                h469
            }), f91())), {
                let (_, h470) = v5;
                h470
            }))
        }
        Adt10::C2(v6) => {
            let v7: (Adt12, Adt13) = f128(Str::lit("t"), v6, v1, f91())?;
            Ok((f111(Str::lit("exact"), {
                let (h471, _) = v7;
                h471
            }), {
                let (_, h472) = v7;
                h472
            }))
        }
        Adt10::C3(v8) => {
            let v9: (Adt12, Adt13) = f128(Str::lit("t"), v8, v1, f91())?;
            Ok((f111(Str::lit("apply-one"), {
                let (h473, _) = v9;
                h473
            }), {
                let (_, h474) = v9;
                h474
            }))
        }
        Adt10::C4(v10, r530) => {
            let v11 = (*r530).clone();
            let v12: (Adt12, Adt13) = f63(v10, v1)?;
            let v13: (Adt12, Adt13) = f112(v11, {
                let (_, h475) = v12;
                h475
            })?;
            Ok((f111(Str::lit("apply"), f5(Str::lit("f"), {
                let (h476, _) = v12;
                h476
            }, f5(Str::lit("p"), f89({
                let (h477, _) = v13;
                h477
            }), f91()))), {
                let (_, h478) = v13;
                h478
            }))
        }
        Adt10::C5 => {
            Ok((f111(Str::lit("rfl"), f91()), v1))
        }
        Adt10::C6(v14) => {
            let v15: (Adt12, Adt13) = f128(Str::lit("t"), v14, v1, f91())?;
            Ok((f111(Str::lit("witness"), {
                let (h479, _) = v15;
                h479
            }), {
                let (_, h480) = v15;
                h480
            }))
        }
        Adt10::C7 => {
            Ok((f111(Str::lit("left"), f91()), v1))
        }
        Adt10::C8 => {
            Ok((f111(Str::lit("right"), f91()), v1))
        }
        Adt10::C9(v16, v17, r531) => {
            let v18 = (*r531).clone();
            let v19: (Adt12, Adt13) = f63(v17, v1)?;
            let v20: (u64, Adt13) = f84({
                let (_, h481) = v19;
                h481
            }, v16)?;
            let v21: (Adt12, Adt13) = f112(v18, {
                let (_, h482) = v20;
                h482
            })?;
            Ok((f111(Str::lit("have"), f5(Str::lit("l"), f104({
                let (h483, _) = v20;
                h483
            }), f5(Str::lit("p"), {
                let (h484, _) = v19;
                h484
            }, f5(Str::lit("q"), {
                let (h485, _) = v21;
                h485
            }, f91())))), {
                let (_, h486) = v21;
                h486
            }))
        }
        Adt10::C10(v22, r532) => {
            let v23 = (*r532).clone();
            let v24: (Adt12, Adt13) = f129(v22, v1)?;
            let v25: (Adt12, Adt13) = f112(v23, {
                let (_, h487) = v24;
                h487
            })?;
            Ok((f111(Str::lit("rw"), f5(Str::lit("at"), {
                let (h488, _) = v24;
                h488
            }, f5(Str::lit("r"), f89({
                let (h489, _) = v25;
                h489
            }), f91()))), {
                let (_, h490) = v25;
                h490
            }))
        }
        Adt10::C11(v26, v27) => {
            let v28: (Adt12, Adt13) = f129(v26, v1)?;
            let v29: (Adt12, Adt13) = f63(v27, {
                let (_, h491) = v28;
                h491
            })?;
            Ok((f111(Str::lit("simp-only"), f5(Str::lit("at"), {
                let (h492, _) = v28;
                h492
            }, f5(Str::lit("r"), f89({
                let (h493, _) = v29;
                h493
            }), f91()))), {
                let (_, h494) = v29;
                h494
            }))
        }
        Adt10::C12(r533) => {
            let v30 = (*r533).clone();
            let v31: (Adt12, Adt13) = f112(v30, v1)?;
            Ok((f111(Str::lit("constructor"), f5(Str::lit("b"), f89({
                let (h495, _) = v31;
                h495
            }), f91())), {
                let (_, h496) = v31;
                h496
            }))
        }
        Adt10::C13(v32, r534) => {
            let v33 = (*r534).clone();
            let v34: (Adt12, Adt13) = f63(v32, v1)?;
            let v35: (Adt12, Adt13) = f112(v33, {
                let (_, h497) = v34;
                h497
            })?;
            Ok((f111(Str::lit("cases"), f5(Str::lit("t"), {
                let (h498, _) = v34;
                h498
            }, f5(Str::lit("c"), f89({
                let (h499, _) = v35;
                h499
            }), f91()))), {
                let (_, h500) = v35;
                h500
            }))
        }
        Adt10::C14(v36, r535) => {
            let v37 = (*r535).clone();
            let v38: (Adt12, Adt13) = f63(v36, v1)?;
            let v39: (Adt12, Adt13) = f112(v37, {
                let (_, h501) = v38;
                h501
            })?;
            Ok((f111(Str::lit("induction"), f5(Str::lit("t"), {
                let (h502, _) = v38;
                h502
            }, f5(Str::lit("c"), f89({
                let (h503, _) = v39;
                h503
            }), f91()))), {
                let (_, h504) = v39;
                h504
            }))
        }
        Adt10::C15(v40, v41, r536) => {
            let v42 = (*r536).clone();
            let v43: (Adt12, Adt13) = f63(v41, v1)?;
            let v44: (Adt12, Adt13) = f112(v42, {
                let (_, h505) = v43;
                h505
            })?;
            Ok((f111(Str::lit("calc"), f5(Str::lit("rel"), f82(v40, Adt3::C5)?, f5(Str::lit("s"), {
                let (h506, _) = v43;
                h506
            }, f5(Str::lit("steps"), f89({
                let (h507, _) = v44;
                h507
            }), f91())))), {
                let (_, h508) = v44;
                h508
            }))
        }
        Adt10::C16 => {
            Ok((f91(), v1))
        }
        Adt10::C17(r537, r538) => {
            let v45 = (*r537).clone();
            let v46 = (*r538).clone();
            let v47: (Adt12, Adt13) = f112(v45, v1)?;
            let v48: (Adt12, Adt13) = f112(v46, {
                let (_, h509) = v47;
                h509
            })?;
            Ok((Adt12::C6(std::rc::Rc::new({
                let (h510, _) = v47;
                h510
            }), std::rc::Rc::new({
                let (h511, _) = v48;
                h511
            })), {
                let (_, h512) = v48;
                h512
            }))
        }
        Adt10::C18(v49, v50, r539) => {
            let v51 = (*r539).clone();
            let v52: (Adt12, Adt13) = f63(v50, v1)?;
            let v53: (Adt12, Adt13) = f112(v51, {
                let (_, h513) = v52;
                h513
            })?;
            Ok((Adt12::C6(std::rc::Rc::new(f87(f5(Str::lit("rev"), Adt12::C0(v49), f5(Str::lit("t"), {
                let (h514, _) = v52;
                h514
            }, f91())))), std::rc::Rc::new({
                let (h515, _) = v53;
                h515
            })), {
                let (_, h516) = v53;
                h516
            }))
        }
        Adt10::C19(v54, v55, v56, r540, r541) => {
            let v57 = (*r540).clone();
            let v58 = (*r541).clone();
            let v59: (Adt12, Adt13) = f127(f130(v56), v1)?;
            let v60: (Adt12, Adt13) = f112(v57, {
                let (_, h517) = v59;
                h517
            })?;
            let v61: (Adt12, Adt13) = f112(v58, {
                let (_, h518) = v60;
                h518
            })?;
            Ok((Adt12::C6(std::rc::Rc::new(f87(f5(Str::lit("ctor"), Adt12::C2(v54), f5(Str::lit("lean"), Adt12::C2(v55), f5(Str::lit("b"), f89({
                let (h519, _) = v59;
                h519
            }), f5(Str::lit("p"), {
                let (h520, _) = v60;
                h520
            }, f91())))))), std::rc::Rc::new({
                let (h521, _) = v61;
                h521
            })), {
                let (_, h522) = v61;
                h522
            }))
        }
        Adt10::C20(v62, v63, r542) => {
            let v64 = (*r542).clone();
            let v65: (Adt12, Adt13) = f63(v62, v1)?;
            let v66: (Adt12, Adt13) = f63(v63, {
                let (_, h523) = v65;
                h523
            })?;
            let v67: (Adt12, Adt13) = f112(v64, {
                let (_, h524) = v66;
                h524
            })?;
            Ok((Adt12::C6(std::rc::Rc::new(f87(f5(Str::lit("t"), {
                let (h525, _) = v65;
                h525
            }, f5(Str::lit("p"), {
                let (h526, _) = v66;
                h526
            }, f91())))), std::rc::Rc::new({
                let (h527, _) = v67;
                h527
            })), {
                let (_, h528) = v67;
                h528
            }))
        }
    }
}

pub fn f113(v0: List<Bytes>) -> Adt12 {
    f131(Fn21::F132, f91(), f67(v0))
}

pub fn f114(v0: Adt0, v1: Bytes) -> Bytes {
    let v2: Bytes = match v0.clone() {
        Adt0::C0(_, h543, _, _, _, _, _, _, _, _, _, _, _) => {
            h543
        }
    };
    let v3: u64 = {
        let a545 = match v0.clone() {
            Adt0::C0(h544, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h544
            }
        };
        length_bytes(a545)
    };
    let v4: u64 = {
        let a546 = v2.clone();
        length_bytes(a546)
    };
    let v5: Bytes = if f133(v2.clone(), match v0 {
        Adt0::C0(h547, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h547
        }
    }) {
        f100(v2, 0u64, {
            let a548 = v4;
            let a549 = v3;
            nat_sub(a548, a549)
        })
    } else {
        Bytes::lit(&[])
    };
    let a550 = v5;
    let a551 = v1;
    append_bytes(a550, a551)
}

pub fn f115(v0: (List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>)) -> R<Option<(List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>)>> {
    let v1: List<List<(Bytes, Adt0)>> = {
        let (h552, _) = v0;
        h552
    };
    let v2: List<List<(Bytes, Adt0)>> = {
        let (_, h553) = v0;
        h553
    };
    let m554 = v1;
    match m554.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>)>)
        }
        Some((v3, v4)) => {
            let m555 = v4;
            match m555.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Adt0)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f134(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f116() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66])
}

pub fn f117(v0: List<List<(Bytes, Bytes)>>) -> R<List<List<(Bytes, Bytes)>>> {
    Ok(f73({
        let (_, h556) = {
            let (h557, _) = f135(Fn22::F136, {
                let a559 = {
                    let a558 = v0.clone();
                    length_list(a558)
                };
                let a560 = 1u64;
                nat_add(a559, a560)?
            }, (v0, List::<List<(Bytes, Bytes)>>::nil()))?;
            h557
        };
        h556
    }))
}

pub fn f118(v0: u64) -> u64 {
    let a561 = v0;
    let a562 = 9223372036854775807u64;
    if nat_le(a561, a562) {
        v0
    } else {
        9223372036854775807u64
    }
}

pub fn f119(v0: Fn19, p566: Option<u64>, p567: List<(u64, u64)>) -> Option<u64> {
    let m565 = (p566, p567);
    {
        let mut m565 = m565;
        loop {
            let step = {
                let (v1, v2) = m565;
                let m563 = v2;
                match m563.uncons() {
                    None => {
                        Err::<(Option<u64>, List<(u64, u64)>), Option<u64>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<u64>, List<(u64, u64)>), Option<u64>>(({
                            let c564: Fn19 = v0.clone();
                            c564.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m565 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f120(v0: u64, v1: Option<u64>, v2: (u64, u64)) -> Option<u64> {
    let a569 = {
        let (h568, _) = v2;
        h568
    };
    let a570 = v0;
    if nat_eq(a569, a570) {
        Some({
            let (_, h571) = v2;
            h571
        })
    } else {
        v1
    }
}

pub fn f121(v0: Adt5) -> Bytes {
    let m572 = v0;
    match m572 {
        Adt5::C0 => {
            f20(Str::lit("eq"))
        }
        Adt5::C1 => {
            f20(Str::lit("and"))
        }
        Adt5::C2 => {
            f20(Str::lit("or"))
        }
        Adt5::C3 => {
            f20(Str::lit("not"))
        }
        Adt5::C4 => {
            f20(Str::lit("iff"))
        }
        Adt5::C5 => {
            f20(Str::lit("exists"))
        }
        Adt5::C6 => {
            f20(Str::lit("exists-unique"))
        }
    }
}

pub fn f122(v0: Bytes) -> R<Bytes> {
    let (_, h573) = {
        let (h574, _) = f33(Fn5::F137(v0.clone()), {
            let a575 = v0;
            length_bytes(a575)
        }, (0u64, Bytes::lit(&[])))?;
        h574
    };
    Ok(h573)
}

pub fn f123(v0: Fn20, p579: Adt12, p580: List<u64>) -> Adt12 {
    let m578 = (p579, p580);
    {
        let mut m578 = m578;
        loop {
            let step = {
                let (v1, v2) = m578;
                let m576 = v2;
                match m576.uncons() {
                    None => {
                        Err::<(Adt12, List<u64>), Adt12>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt12, List<u64>), Adt12>(({
                            let c577: Fn20 = v0.clone();
                            c577.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m578 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f124(v0: Adt12, v1: u64) -> Adt12 {
    Adt12::C6(std::rc::Rc::new(f104(v1)), std::rc::Rc::new(v0))
}

pub fn f125(v0: List<u64>) -> List<u64> {
    f138(Fn23::F139, List::<u64>::nil(), v0)
}

pub fn f126(v0: u64) -> Bytes {
    let a581 = v0;
    let a582 = 1u64;
    if nat_eq(a581, a582) {
        f140(Str::lit("implicit"))
    } else {
        let a583 = v0;
        let a584 = 2u64;
        if nat_eq(a583, a584) {
            f140(Str::lit("instance"))
        } else {
            f140(Str::lit("explicit"))
        }
    }
}

pub fn f127(v0: List<u64>, v1: Adt13) -> R<(Adt12, Adt13)> {
    let v2: (List<u64>, Adt13) = f141(Fn24::F142, (List::<u64>::nil(), v1), v0)?;
    Ok((f123(Fn20::F143, f91(), {
        let (h585, _) = v2;
        h585
    }), {
        let (_, h586) = v2;
        h586
    }))
}

pub fn f128(v0: Str, v1: Adt2, v2: Adt13, v3: Adt12) -> R<(Adt12, Adt13)> {
    let v4: (Adt12, Adt13) = f63(v1, v2)?;
    Ok((f5(v0, {
        let (h587, _) = v4;
        h587
    }, v3), {
        let (_, h588) = v4;
        h588
    }))
}

pub fn f129(v0: Option<u64>, v1: Adt13) -> R<(Adt12, Adt13)> {
    let m589 = v0;
    match m589 {
        None => {
            Ok((f6(Str::lit("goal")), v1))
        }
        Some(v2) => {
            let v3: (Adt14, Adt13) = f80(v1, v2)?;
            Ok(({
                let m591 = {
                    let (h590, _) = v3;
                    h590
                };
                match m591 {
                    Adt14::C0(v4) => {
                        f104(v4)
                    }
                    Adt14::C1(v5) => {
                        f104(v5)
                    }
                }
            }, {
                let (_, h592) = v3;
                h592
            }))
        }
    }
}

pub fn f130(v0: List<(u64, Bytes)>) -> List<u64> {
    f125(f144(Fn25::F145, List::<u64>::nil(), v0))
}

pub fn f131(v0: Fn21, p596: Adt12, p597: List<Bytes>) -> Adt12 {
    let m595 = (p596, p597);
    {
        let mut m595 = m595;
        loop {
            let step = {
                let (v1, v2) = m595;
                let m593 = v2;
                match m593.uncons() {
                    None => {
                        Err::<(Adt12, List<Bytes>), Adt12>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt12, List<Bytes>), Adt12>(({
                            let c594: Fn21 = v0.clone();
                            c594.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m595 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f132(v0: Adt12, v1: Bytes) -> Adt12 {
    Adt12::C6(std::rc::Rc::new(Adt12::C2(v1)), std::rc::Rc::new(v0))
}

pub fn f133(v0: Bytes, v1: Bytes) -> bool {
    let v2: u64 = {
        let a598 = v0.clone();
        length_bytes(a598)
    };
    let v3: u64 = {
        let a599 = v1.clone();
        length_bytes(a599)
    };
    let a600 = v3;
    let a601 = v2;
    if nat_le(a600, a601) {
        let m607 = {
            let a604 = v0;
            let a605 = {
                let a602 = v2;
                let a603 = v3;
                nat_sub(a602, a603)
            };
            let a606 = v3;
            slice_bytes(a604, a605, a606)
        };
        match m607 {
            None => {
                false
            }
            Some(v4) => {
                let a608 = v4;
                let a609 = v1;
                equal(a608, a609)
            }
        }
    } else {
        false
    }
}

pub fn f134(v0: List<(Bytes, Adt0)>, v1: List<(Bytes, Adt0)>) -> R<List<(Bytes, Adt0)>> {
    let v2: u64 = {
        let a614 = {
            let a612 = {
                let a610 = v0.clone();
                length_list(a610)
            };
            let a613 = {
                let a611 = v1.clone();
                length_list(a611)
            };
            nat_add(a612, a613)?
        };
        let a615 = 1u64;
        nat_add(a614, a615)?
    };
    Ok(f11({
        let (_, h616) = {
            let (_, h617) = {
                let (h618, _) = f146(Fn26::F147, v2, (v0, (v1, List::<(Bytes, Adt0)>::nil())));
                h618
            };
            h617
        };
        h616
    }))
}

pub fn f135(v0: Fn22, p623: u64, p624: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)> {
    let m622 = (p623, p624);
    {
        let mut m622 = m622;
        loop {
            let step = {
                let (v1, v2) = m622;
                let m619 = v1;
                if m619 == 0 {
                    Err::<(u64, (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)), R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m619 - 1;
                    let m621 = {
                        let c620: Fn22 = v0.clone();
                        c620.apply(v2.clone())?
                    };
                    match m621 {
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
                Ok(next) => { m622 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f136(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    f148(v0)
}

pub fn f137(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f149(v0, v1)
}

pub fn f138(v0: Fn23, p628: List<u64>, p629: List<u64>) -> List<u64> {
    let m627 = (p628, p629);
    {
        let mut m627 = m627;
        loop {
            let step = {
                let (v1, v2) = m627;
                let m625 = v2;
                match m625.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>(({
                            let c626: Fn23 = v0.clone();
                            c626.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m627 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f139(v0: List<u64>, v1: u64) -> List<u64> {
    List::cons(v1, v0)
}

pub fn f140(v0: Str) -> Bytes {
    let a630 = v0;
    utf8_encode(a630)
}

pub fn f141(v0: Fn24, p634: (List<u64>, Adt13), p635: List<u64>) -> R<(List<u64>, Adt13)> {
    let m633 = (p634, p635);
    {
        let mut m633 = m633;
        loop {
            let step = {
                let (v1, v2) = m633;
                let m631 = v2;
                match m631.uncons() {
                    None => {
                        Err::<((List<u64>, Adt13), List<u64>), R<(List<u64>, Adt13)>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<((List<u64>, Adt13), List<u64>), R<(List<u64>, Adt13)>>(({
                            let c632: Fn24 = v0.clone();
                            c632.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m633 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f142(v0: (List<u64>, Adt13), v1: u64) -> R<(List<u64>, Adt13)> {
    let v2: (u64, Adt13) = f84({
        let (_, h636) = v0;
        h636
    }, v1)?;
    Ok((List::cons({
        let (h637, _) = v2;
        h637
    }, {
        let (h638, _) = v0;
        h638
    }), {
        let (_, h639) = v2;
        h639
    }))
}

pub fn f143(v0: Adt12, v1: u64) -> Adt12 {
    Adt12::C6(std::rc::Rc::new(f104(v1)), std::rc::Rc::new(v0))
}

pub fn f144(v0: Fn25, p643: List<u64>, p644: List<(u64, Bytes)>) -> List<u64> {
    let m642 = (p643, p644);
    {
        let mut m642 = m642;
        loop {
            let step = {
                let (v1, v2) = m642;
                let m640 = v2;
                match m640.uncons() {
                    None => {
                        Err::<(List<u64>, List<(u64, Bytes)>), List<u64>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<u64>, List<(u64, Bytes)>), List<u64>>(({
                            let c641: Fn25 = v0.clone();
                            c641.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m642 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f145(v0: List<u64>, v1: (u64, Bytes)) -> List<u64> {
    List::cons({
        let (h645, _) = v1;
        h645
    }, v0)
}

pub fn f146(v0: Fn26, p650: u64, p651: (List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>))) -> ((List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>)), bool) {
    let m649 = (p650, p651);
    {
        let mut m649 = m649;
        loop {
            let step = {
                let (v1, v2) = m649;
                let m646 = v1;
                if m646 == 0 {
                    Err::<(u64, (List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>))), ((List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>)), bool)>((v2, false))
                } else {
                    let v3 = m646 - 1;
                    let m648 = {
                        let c647: Fn26 = v0.clone();
                        c647.apply(v2.clone())
                    };
                    match m648 {
                        None => {
                            Err::<(u64, (List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>))), ((List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>)), bool)>((v2, true))
                        }
                        Some(v4) => {
                            Ok::<(u64, (List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>))), ((List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>)), bool)>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m649 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f147(v0: (List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>))) -> Option<(List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>))> {
    f150(v0)
}

pub fn f148(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    let v1: List<List<(Bytes, Bytes)>> = {
        let (h652, _) = v0;
        h652
    };
    let v2: List<List<(Bytes, Bytes)>> = {
        let (_, h653) = v0;
        h653
    };
    let m654 = v1;
    match m654.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>)
        }
        Some((v3, v4)) => {
            let m655 = v4;
            match m655.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Bytes)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f151(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f149(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let m659 = {
        let a657 = v0;
        let a658 = {
            let (h656, _) = v1.clone();
            h656
        };
        index_bytes(a657, a658)
    };
    match m659 {
        None => {
            Ok(None::<(u64, Bytes)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a661 = {
                    let (h660, _) = v1;
                    h660
                };
                let a662 = 1u64;
                nat_add(a661, a662)?
            }, {
                let a664 = {
                    let (_, h663) = v1;
                    h663
                };
                let a665 = f99(v2)?;
                append_bytes(a664, a665)
            })))
        }
    }
}

pub fn f150(v0: (List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>))) -> Option<(List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>))> {
    let v1: List<(Bytes, Adt0)> = {
        let (h666, _) = v0.clone();
        h666
    };
    let v2: List<(Bytes, Adt0)> = {
        let (h667, _) = {
            let (_, h668) = v0.clone();
            h668
        };
        h667
    };
    let v3: List<(Bytes, Adt0)> = {
        let (_, h669) = {
            let (_, h670) = v0;
            h670
        };
        h669
    };
    let m671 = v1.clone();
    match m671.uncons() {
        None => {
            let m672 = v2;
            match m672.uncons() {
                None => {
                    None::<(List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Adt0)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m673 = v2.clone();
            match m673.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Adt0)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f152({
                        let (h674, _) = v8.clone();
                        h674
                    }, {
                        let (h675, _) = v6.clone();
                        h675
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

pub fn f151(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v2: u64 = {
        let a680 = {
            let a678 = {
                let a676 = v0.clone();
                length_list(a676)
            };
            let a679 = {
                let a677 = v1.clone();
                length_list(a677)
            };
            nat_add(a678, a679)?
        };
        let a681 = 1u64;
        nat_add(a680, a681)?
    };
    Ok(f153({
        let (_, h682) = {
            let (_, h683) = {
                let (h684, _) = f154(Fn27::F155, v2, (v0, (v1, List::<(Bytes, Bytes)>::nil())));
                h684
            };
            h683
        };
        h682
    }))
}

pub fn f152(v0: Bytes, v1: Bytes) -> bool {
    let a689 = {
        let a685 = v0;
        let a686 = v1;
        compare_bytes(a685, a686)
    };
    let a690 = {
        let a687 = Bytes::lit(&[]);
        let a688 = Bytes::lit(&[0x01]);
        compare_bytes(a687, a688)
    };
    equal(a689, a690)
}

pub fn f153(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f36(Fn6::F156, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f154(v0: Fn27, p695: u64, p696: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool) {
    let m694 = (p695, p696);
    {
        let mut m694 = m694;
        loop {
            let step = {
                let (v1, v2) = m694;
                let m691 = v1;
                if m691 == 0 {
                    Err::<(u64, (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))), ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool)>((v2, false))
                } else {
                    let v3 = m691 - 1;
                    let m693 = {
                        let c692: Fn27 = v0.clone();
                        c692.apply(v2.clone())
                    };
                    match m693 {
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
                Ok(next) => { m694 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f155(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    f157(v0)
}

pub fn f156(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn f157(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    let v1: List<(Bytes, Bytes)> = {
        let (h697, _) = v0.clone();
        h697
    };
    let v2: List<(Bytes, Bytes)> = {
        let (h698, _) = {
            let (_, h699) = v0.clone();
            h699
        };
        h698
    };
    let v3: List<(Bytes, Bytes)> = {
        let (_, h700) = {
            let (_, h701) = v0;
            h701
        };
        h700
    };
    let m702 = v1.clone();
    match m702.uncons() {
        None => {
            let m703 = v2;
            match m703.uncons() {
                None => {
                    None::<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Bytes)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m704 = v2.clone();
            match m704.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Bytes)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f152({
                        let (h705, _) = v8.clone();
                        h705
                    }, {
                        let (h706, _) = v6.clone();
                        h706
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

pub fn run(p0: List<(Bytes, Adt0)>) -> R<Bytes> {
    f0(p0)
}
