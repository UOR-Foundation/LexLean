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
    C0(u64, u64, u64, Option<(u64, u64)>),
}

#[derive(Clone)]
pub enum Fn0 {
    F5(Bytes),
    F12(Bytes),
    F29(Bytes),
    F30(u64, Bytes),
    F31(Bytes),
    F33(Bytes),
    F35(Bytes, u64, Bytes, u64),
}

impl Fn0 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn0::F5(k0) => f5(k0.clone(), p0),
            Fn0::F12(k0) => f12(k0.clone(), p0),
            Fn0::F29(k0) => f29(k0.clone(), p0),
            Fn0::F30(k0, k1) => f30(*k0, k1.clone(), p0),
            Fn0::F31(k0) => f31(k0.clone(), p0),
            Fn0::F33(k0) => f33(k0.clone(), p0),
            Fn0::F35(k0, k1, k2, k3) => f35(k0.clone(), *k1, k2.clone(), *k3, p0),
        }
    }
}

pub fn f0(v0: Bytes) -> R<Option<Adt0>> {
    let m1 = f1(v0.clone())?;
    match m1 {
        None => {
            Ok(None::<Adt0>)
        }
        Some(v1) => {
            let v2: u64 = {
                let (h2, _) = v1;
                h2
            };
            let v3: u64 = {
                let (h3, _) = {
                    let (_, h4) = v1;
                    h4
                };
                h3
            };
            let m7 = f2(v0.clone(), v2, {
                let (_, h5) = {
                    let (_, h6) = v1;
                    h6
                };
                h5
            })?;
            match m7 {
                None => {
                    Ok(None::<Adt0>)
                }
                Some(v4) => {
                    let a8 = v4;
                    let a9 = v3;
                    if nat_lt(a8, a9) {
                        Ok(None::<Adt0>)
                    } else {
                        Ok(Some(Adt0::C0(v2, v3, v4, f3(v0, v2, v3, v4)?)))
                    }
                }
            }
        }
    }
}

pub fn f1(v0: Bytes) -> R<Option<(u64, (u64, u64))>> {
    let v1: u64 = {
        let (h10, _) = f4(Fn0::F5(v0.clone()), {
            let a12 = {
                let a11 = v0.clone();
                length_bytes(a11)
            };
            let a13 = 1u64;
            nat_add(a12, a13)?
        }, 0u64)?;
        h10
    };
    let m14 = f6(v0.clone(), v1)?;
    match m14 {
        None => {
            Ok(None::<(u64, (u64, u64))>)
        }
        Some(v2) => {
            if f7(v0.clone(), v2, f8(Str::lit("\\begin"))) {
                let m16 = f9(v0.clone(), {
                    let (_, h15) = v2;
                    h15
                })?;
                match m16 {
                    None => {
                        Ok(None::<(u64, (u64, u64))>)
                    }
                    Some(v3) => {
                        let m17 = f10(f11(v0, v3));
                        match m17 {
                            None => {
                                Ok(None::<(u64, (u64, u64))>)
                            }
                            Some(v4) => {
                                Ok(Some((v4, ({
                                    let a19 = {
                                        let (_, h18) = v3;
                                        h18
                                    };
                                    let a20 = 1u64;
                                    nat_add(a19, a20)?
                                }, {
                                    let (_, h21) = v2;
                                    h21
                                }))))
                            }
                        }
                    }
                }
            } else {
                Ok(None::<(u64, (u64, u64))>)
            }
        }
    }
}

pub fn f2(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let v3: u64 = {
        let (h22, _) = f4(Fn0::F12(v0.clone()), {
            let a24 = {
                let a23 = v0.clone();
                length_bytes(a23)
            };
            let a25 = 1u64;
            nat_add(a24, a25)?
        }, v2)?;
        h22
    };
    let m26 = f6(v0.clone(), v3)?;
    match m26 {
        None => {
            Ok(None::<u64>)
        }
        Some(v4) => {
            if f7(v0.clone(), v4, f8(Str::lit("\\end"))) {
                let m28 = f9(v0.clone(), {
                    let (_, h27) = v4;
                    h27
                })?;
                match m28 {
                    None => {
                        Ok(None::<u64>)
                    }
                    Some(v5) => {
                        let a29 = f11(v0, v5);
                        let a30 = f13(v1);
                        if equal(a29, a30) {
                            Ok(Some({
                                let (h31, _) = v4;
                                h31
                            }))
                        } else {
                            Ok(None::<u64>)
                        }
                    }
                }
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f3(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Option<(u64, u64)>> {
    let v4: Bytes = f14(v0, v2, {
        let a32 = v3;
        let a33 = v2;
        nat_sub(a32, a33)
    });
    let v5: u64 = f15(v4.clone())?;
    let v6: Bytes = f16(v1);
    if f17(f18(v4.clone(), v5), v6.clone()) {
        let v7: u64 = {
            let a35 = v5;
            let a36 = {
                let a34 = v6;
                length_bytes(a34)
            };
            nat_add(a35, a36)?
        };
        let v8: u64 = {
            let a37 = v7;
            let a38 = f15(f18(v4.clone(), v7))?;
            nat_add(a37, a38)?
        };
        if f17(f18(v4.clone(), v8), f8(Str::lit("{"))) {
            let v9: Bytes = f18(v4, {
                let a39 = v8;
                let a40 = 1u64;
                nat_add(a39, a40)?
            });
            let v10: u64 = f19(v9.clone(), 0u64)?;
            let a41 = 0u64;
            let a42 = v10;
            if if nat_lt(a41, a42) {
                f20(v9, f8(Str::lit("}")), {
                    let a43 = v10;
                    let a44 = 1u64;
                    nat_sub(a43, a44)
                })
            } else {
                false
            } {
                Ok(Some(({
                    let a47 = {
                        let a45 = v2;
                        let a46 = v8;
                        nat_add(a45, a46)?
                    };
                    let a48 = 1u64;
                    nat_add(a47, a48)?
                }, {
                    let a55 = {
                        let a51 = {
                            let a49 = v2;
                            let a50 = v8;
                            nat_add(a49, a50)?
                        };
                        let a52 = 1u64;
                        nat_add(a51, a52)?
                    };
                    let a56 = {
                        let a53 = v10;
                        let a54 = 1u64;
                        nat_sub(a53, a54)
                    };
                    nat_add(a55, a56)?
                })))
            } else {
                Ok(None::<(u64, u64)>)
            }
        } else {
            Ok(None::<(u64, u64)>)
        }
    } else {
        Ok(None::<(u64, u64)>)
    }
}

pub fn f4(v0: Fn0, p61: u64, p62: u64) -> R<(u64, bool)> {
    let m60 = (p61, p62);
    {
        let mut m60 = m60;
        loop {
            let step = {
                let (v1, v2) = m60;
                let m57 = v1;
                if m57 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m57 - 1;
                    let m59 = {
                        let c58: Fn0 = v0.clone();
                        c58.apply(v2)?
                    };
                    match m59 {
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
                Ok(next) => { m60 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f5(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f21(v0, v1)
}

pub fn f6(v0: Bytes, v1: u64) -> R<Option<(u64, u64)>> {
    let v2: u64 = f22(v0.clone(), v1)?;
    if f23(v0.clone(), v2)? {
        if f24(v0.clone(), {
            let a63 = v2;
            let a64 = 1u64;
            nat_add(a63, a64)?
        }) {
            Ok(Some((v2, f25(v0, {
                let a65 = v2;
                let a66 = 1u64;
                nat_add(a65, a66)?
            })?)))
        } else {
            Ok(Some((v2, {
                let a67 = v2;
                let a68 = 2u64;
                nat_add(a67, a68)?
            })))
        }
    } else {
        Ok(None::<(u64, u64)>)
    }
}

pub fn f7(v0: Bytes, v1: (u64, u64), v2: Bytes) -> bool {
    let m77 = {
        let a74 = v0;
        let a75 = {
            let (h69, _) = v1;
            h69
        };
        let a76 = {
            let a72 = {
                let (_, h70) = v1;
                h70
            };
            let a73 = {
                let (h71, _) = v1;
                h71
            };
            nat_sub(a72, a73)
        };
        slice_bytes(a74, a75, a76)
    };
    match m77 {
        None => {
            false
        }
        Some(v3) => {
            let a78 = v3;
            let a79 = v2;
            equal(a78, a79)
        }
    }
}

pub fn f8(v0: Str) -> Bytes {
    let a80 = v0;
    utf8_encode(a80)
}

pub fn f9(v0: Bytes, v1: u64) -> R<Option<(u64, u64)>> {
    if f26(v0.clone(), v1, 123u8) {
        let m83 = f27(f18(v0, {
            let a81 = v1;
            let a82 = 1u64;
            nat_add(a81, a82)?
        }), f8(Str::lit("}")))?;
        match m83 {
            None => {
                Ok(None::<(u64, u64)>)
            }
            Some(v2) => {
                Ok(Some(({
                    let a84 = v1;
                    let a85 = 1u64;
                    nat_add(a84, a85)?
                }, {
                    let a88 = {
                        let a86 = v1;
                        let a87 = 1u64;
                        nat_add(a86, a87)?
                    };
                    let a89 = v2;
                    nat_add(a88, a89)?
                })))
            }
        }
    } else {
        Ok(None::<(u64, u64)>)
    }
}

pub fn f10(v0: Bytes) -> Option<u64> {
    let a90 = v0.clone();
    let a91 = f13(0u64);
    if equal(a90, a91) {
        Some(0u64)
    } else {
        let a92 = v0.clone();
        let a93 = f13(1u64);
        if equal(a92, a93) {
            Some(1u64)
        } else {
            let a94 = v0;
            let a95 = f13(2u64);
            if equal(a94, a95) {
                Some(2u64)
            } else {
                None::<u64>
            }
        }
    }
}

pub fn f11(v0: Bytes, v1: (u64, u64)) -> Bytes {
    f14(v0, {
        let (h96, _) = v1;
        h96
    }, {
        let a99 = {
            let (_, h97) = v1;
            h97
        };
        let a100 = {
            let (h98, _) = v1;
            h98
        };
        nat_sub(a99, a100)
    })
}

pub fn f12(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f28(v0, v1)
}

pub fn f13(v0: u64) -> Bytes {
    let a101 = v0;
    let a102 = 0u64;
    if nat_eq(a101, a102) {
        f8(Str::lit("semanticmodule"))
    } else {
        let a103 = v0;
        let a104 = 1u64;
        if nat_eq(a103, a104) {
            f8(Str::lit("toolingmodule"))
        } else {
            f8(Str::lit("coremodule"))
        }
    }
}

pub fn f14(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m108 = {
        let a105 = v0;
        let a106 = v1;
        let a107 = v2;
        slice_bytes(a105, a106, a107)
    };
    match m108 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f15(v0: Bytes) -> R<u64> {
    let (h109, _) = f4(Fn0::F29(v0.clone()), {
        let a110 = v0;
        length_bytes(a110)
    }, 0u64)?;
    Ok(h109)
}

pub fn f16(v0: u64) -> Bytes {
    let a111 = v0;
    let a112 = 0u64;
    if nat_eq(a111, a112) {
        f8(Str::lit("\\semanticdata"))
    } else {
        let a113 = v0;
        let a114 = 1u64;
        if nat_eq(a113, a114) {
            f8(Str::lit("\\toolingdata"))
        } else {
            f8(Str::lit("\\coredata"))
        }
    }
}

pub fn f17(v0: Bytes, v1: Bytes) -> bool {
    let m119 = {
        let a116 = v0;
        let a117 = 0u64;
        let a118 = {
            let a115 = v1.clone();
            length_bytes(a115)
        };
        slice_bytes(a116, a117, a118)
    };
    match m119 {
        None => {
            false
        }
        Some(v2) => {
            let a120 = v2;
            let a121 = v1;
            equal(a120, a121)
        }
    }
}

pub fn f18(v0: Bytes, v1: u64) -> Bytes {
    f14(v0.clone(), v1, {
        let a123 = {
            let a122 = v0;
            length_bytes(a122)
        };
        let a124 = v1;
        nat_sub(a123, a124)
    })
}

pub fn f19(v0: Bytes, v1: u64) -> R<u64> {
    let (h125, _) = f4(Fn0::F30(v1, v0.clone()), {
        let a126 = v0.clone();
        length_bytes(a126)
    }, {
        let a127 = v0;
        length_bytes(a127)
    })?;
    Ok(h125)
}

pub fn f20(v0: Bytes, v1: Bytes, v2: u64) -> bool {
    let m132 = {
        let a129 = v0;
        let a130 = v2;
        let a131 = {
            let a128 = v1.clone();
            length_bytes(a128)
        };
        slice_bytes(a129, a130, a131)
    };
    match m132 {
        None => {
            false
        }
        Some(v3) => {
            let a133 = v3;
            let a134 = v1;
            equal(a133, a134)
        }
    }
}

pub fn f21(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m135 = f6(v0.clone(), v1)?;
    match m135 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f7(v0.clone(), v2, f8(Str::lit("\\begin"))) {
                let m137 = f9(v0.clone(), {
                    let (_, h136) = v2;
                    h136
                })?;
                match m137 {
                    None => {
                        Ok(None::<u64>)
                    }
                    Some(v3) => {
                        let a138 = f11(v0, v3);
                        let a139 = f8(Str::lit("lexlean"));
                        if equal(a138, a139) {
                            Ok(Some({
                                let (_, h140) = v2;
                                h140
                            }))
                        } else {
                            Ok(None::<u64>)
                        }
                    }
                }
            } else {
                Ok(Some({
                    let (_, h141) = v2;
                    h141
                }))
            }
        }
    }
}

pub fn f22(v0: Bytes, v1: u64) -> R<u64> {
    let (h142, _) = f4(Fn0::F31(v0.clone()), {
        let a144 = {
            let a143 = v0;
            length_bytes(a143)
        };
        let a145 = 1u64;
        nat_add(a144, a145)?
    }, v1)?;
    Ok(h142)
}

pub fn f23(v0: Bytes, v1: u64) -> R<bool> {
    if f26(v0.clone(), v1, 92u8) {
        Ok(f32(v0, {
            let a146 = v1;
            let a147 = 1u64;
            nat_add(a146, a147)?
        }, 0u8, 127u8))
    } else {
        Ok(false)
    }
}

pub fn f24(v0: Bytes, v1: u64) -> bool {
    if f32(v0.clone(), v1, 65u8, 90u8) {
        true
    } else {
        f32(v0, v1, 97u8, 122u8)
    }
}

pub fn f25(v0: Bytes, v1: u64) -> R<u64> {
    let (h148, _) = f4(Fn0::F33(v0.clone()), {
        let a150 = {
            let a149 = v0;
            length_bytes(a149)
        };
        let a151 = 1u64;
        nat_add(a150, a151)?
    }, v1)?;
    Ok(h148)
}

pub fn f26(v0: Bytes, v1: u64, v2: u8) -> bool {
    let m152 = f34(v0, v1);
    match m152 {
        None => {
            false
        }
        Some(v3) => {
            let a153 = v3;
            let a154 = v2;
            equal(a153, a154)
        }
    }
}

pub fn f27(v0: Bytes, v1: Bytes) -> R<Option<u64>> {
    let v2: u64 = {
        let a155 = v0.clone();
        length_bytes(a155)
    };
    let v3: u64 = {
        let a156 = v1.clone();
        length_bytes(a156)
    };
    let v4: u64 = {
        let (h157, _) = f4(Fn0::F35(v1.clone(), v3, v0.clone(), v2), {
            let a158 = v2;
            let a159 = 1u64;
            nat_add(a158, a159)?
        }, 0u64)?;
        h157
    };
    if f20(v0, v1, v4) {
        Ok(Some(v4))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f28(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m160 = f6(v0.clone(), v1)?;
    match m160 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f7(v0, v2, f8(Str::lit("\\end"))) {
                Ok(None::<u64>)
            } else {
                Ok(Some({
                    let (_, h161) = v2;
                    h161
                }))
            }
        }
    }
}

pub fn f29(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a162 = 0u64;
    let a163 = f36(v0.clone(), v1)?;
    if nat_lt(a162, a163) {
        Ok(Some({
            let a164 = v1;
            let a165 = f36(v0, v1)?;
            nat_add(a164, a165)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f30(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    let a166 = 0u64;
    let a167 = f37(v1.clone(), v2)?;
    if if nat_lt(a166, a167) {
        let a170 = {
            let a168 = v0;
            let a169 = f37(v1.clone(), v2)?;
            nat_add(a168, a169)?
        };
        let a171 = v2;
        nat_le(a170, a171)
    } else {
        false
    } {
        Ok(Some({
            let a172 = v2;
            let a173 = f37(v1, v2)?;
            nat_sub(a172, a173)
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f31(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f23(v0.clone(), v1)? {
        Ok(None::<u64>)
    } else {
        let a175 = v1;
        let a176 = {
            let a174 = v0;
            length_bytes(a174)
        };
        if nat_lt(a175, a176) {
            Ok(Some({
                let a177 = v1;
                let a178 = 1u64;
                nat_add(a177, a178)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f32(v0: Bytes, v1: u64, v2: u8, v3: u8) -> bool {
    let m179 = f34(v0, v1);
    match m179 {
        None => {
            false
        }
        Some(v4) => {
            f38(v4, v2, v3)
        }
    }
}

pub fn f33(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f24(v0, v1) {
        Ok(Some({
            let a180 = v1;
            let a181 = 1u64;
            nat_add(a180, a181)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f34(v0: Bytes, v1: u64) -> Option<u8> {
    let a182 = v0;
    let a183 = v1;
    index_bytes(a182, a183)
}

pub fn f35(v0: Bytes, v1: u64, v2: Bytes, v3: u64, v4: u64) -> R<Option<u64>> {
    if f20(v2, v0, v4) {
        Ok(None::<u64>)
    } else {
        let a186 = {
            let a184 = v4;
            let a185 = v1;
            nat_add(a184, a185)?
        };
        let a187 = v3;
        if nat_lt(a186, a187) {
            Ok(Some({
                let a188 = v4;
                let a189 = 1u64;
                nat_add(a188, a189)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f36(v0: Bytes, v1: u64) -> R<u64> {
    if if f32(v0.clone(), v1, 9u8, 13u8) {
        true
    } else {
        f26(v0.clone(), v1, 32u8)
    } {
        Ok(1u64)
    } else {
        if f26(v0.clone(), v1, 194u8) {
            if if f26(v0.clone(), {
                let a190 = v1;
                let a191 = 1u64;
                nat_add(a190, a191)?
            }, 133u8) {
                true
            } else {
                f26(v0, {
                    let a192 = v1;
                    let a193 = 1u64;
                    nat_add(a192, a193)?
                }, 160u8)
            } {
                Ok(2u64)
            } else {
                Ok(0u64)
            }
        } else {
            if f26(v0.clone(), v1, 225u8) {
                if if f26(v0.clone(), {
                    let a194 = v1;
                    let a195 = 1u64;
                    nat_add(a194, a195)?
                }, 154u8) {
                    f26(v0, {
                        let a196 = v1;
                        let a197 = 2u64;
                        nat_add(a196, a197)?
                    }, 128u8)
                } else {
                    false
                } {
                    Ok(3u64)
                } else {
                    Ok(0u64)
                }
            } else {
                if f26(v0.clone(), v1, 226u8) {
                    if f26(v0.clone(), {
                        let a198 = v1;
                        let a199 = 1u64;
                        nat_add(a198, a199)?
                    }, 128u8) {
                        if if f32(v0.clone(), {
                            let a200 = v1;
                            let a201 = 2u64;
                            nat_add(a200, a201)?
                        }, 128u8, 138u8) {
                            true
                        } else {
                            if f26(v0.clone(), {
                                let a202 = v1;
                                let a203 = 2u64;
                                nat_add(a202, a203)?
                            }, 168u8) {
                                true
                            } else {
                                if f26(v0.clone(), {
                                    let a204 = v1;
                                    let a205 = 2u64;
                                    nat_add(a204, a205)?
                                }, 169u8) {
                                    true
                                } else {
                                    f26(v0, {
                                        let a206 = v1;
                                        let a207 = 2u64;
                                        nat_add(a206, a207)?
                                    }, 175u8)
                                }
                            }
                        } {
                            Ok(3u64)
                        } else {
                            Ok(0u64)
                        }
                    } else {
                        if if f26(v0.clone(), {
                            let a208 = v1;
                            let a209 = 1u64;
                            nat_add(a208, a209)?
                        }, 129u8) {
                            f26(v0, {
                                let a210 = v1;
                                let a211 = 2u64;
                                nat_add(a210, a211)?
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
                    if f26(v0.clone(), v1, 227u8) {
                        if if f26(v0.clone(), {
                            let a212 = v1;
                            let a213 = 1u64;
                            nat_add(a212, a213)?
                        }, 128u8) {
                            f26(v0, {
                                let a214 = v1;
                                let a215 = 2u64;
                                nat_add(a214, a215)?
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

pub fn f37(v0: Bytes, v1: u64) -> R<u64> {
    let a216 = v1;
    let a217 = 0u64;
    if nat_eq(a216, a217) {
        Ok(0u64)
    } else {
        let a220 = f36(v0.clone(), {
            let a218 = v1;
            let a219 = 1u64;
            nat_sub(a218, a219)
        })?;
        let a221 = 1u64;
        if nat_eq(a220, a221) {
            Ok(1u64)
        } else {
            let a222 = 2u64;
            let a223 = v1;
            if if nat_le(a222, a223) {
                let a226 = f36(v0.clone(), {
                    let a224 = v1;
                    let a225 = 2u64;
                    nat_sub(a224, a225)
                })?;
                let a227 = 2u64;
                nat_eq(a226, a227)
            } else {
                false
            } {
                Ok(2u64)
            } else {
                let a228 = 3u64;
                let a229 = v1;
                if if nat_le(a228, a229) {
                    let a232 = f36(v0, {
                        let a230 = v1;
                        let a231 = 3u64;
                        nat_sub(a230, a231)
                    })?;
                    let a233 = 3u64;
                    nat_eq(a232, a233)
                } else {
                    false
                } {
                    Ok(3u64)
                } else {
                    Ok(0u64)
                }
            }
        }
    }
}

pub fn f38(v0: u8, v1: u8, v2: u8) -> bool {
    let m236 = {
        let a234 = v0;
        let a235 = v1;
        fixed_u8::checked_sub(a234, a235)
    };
    match m236 {
        None => {
            false
        }
        Some(_) => {
            let m239 = {
                let a237 = v2;
                let a238 = v0;
                fixed_u8::checked_sub(a237, a238)
            };
            match m239 {
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

pub fn run(p0: Bytes) -> R<Option<Adt0>> {
    f0(p0)
}
