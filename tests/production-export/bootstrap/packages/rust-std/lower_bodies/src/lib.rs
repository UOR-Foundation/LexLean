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
    C0(Adt0, Adt8, Adt0, Adt9),
}

#[derive(Clone)]
pub enum Adt8 {
    C0,
    C1(Adt0, Adt0, std::rc::Rc<Adt8>),
}

#[derive(Clone)]
pub enum Adt9 {
    C0,
    C1(Adt0, u64, Adt0, std::rc::Rc<Adt9>),
}

#[derive(Clone)]
pub enum Adt10 {
    C0(Adt11, u64),
}

#[derive(Clone)]
pub enum Adt11 {
    C0(Adt12, Adt15),
    C1(Adt16, Adt17, Adt15),
    C2(Adt18, std::rc::Rc<Adt11>, Adt15),
    C3(Adt19, std::rc::Rc<Adt11>, bool, Adt15),
    C4(Adt17, std::rc::Rc<Adt11>, bool, Adt15),
    C5(Adt21, std::rc::Rc<Adt11>, Adt15),
    C6(std::rc::Rc<Adt11>, std::rc::Rc<Adt11>, Adt15),
    C7(std::rc::Rc<Adt11>, std::rc::Rc<Adt11>, std::rc::Rc<Adt11>, Adt15),
    C8(std::rc::Rc<Adt11>, std::rc::Rc<Adt11>, Adt15),
    C9(std::rc::Rc<Adt11>),
    C10(Adt27, Adt17, Adt15),
    C11(std::rc::Rc<Adt11>, std::rc::Rc<Adt11>),
    C12,
    C13(std::rc::Rc<Adt11>, std::rc::Rc<Adt11>),
    C14(Adt28, bool, Adt22, std::rc::Rc<Adt11>, Adt15, std::rc::Rc<Adt11>),
    C15(Adt28, std::rc::Rc<Adt11>, std::rc::Rc<Adt11>),
    C16(Adt17, std::rc::Rc<Adt11>),
}

#[derive(Clone)]
pub enum Adt12 {
    C0,
    C1(bool),
    C2(u64),
    C3(bool, u64),
    C4(Adt13, bool, u64),
    C5(Bytes),
    C6(Bytes),
    C7(Adt14),
    C8,
}

#[derive(Clone)]
pub enum Adt13 {
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
pub enum Adt14 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt15 {
    C0(Bytes, Option<Adt13>),
}

#[derive(Clone)]
pub enum Adt16 {
    C0,
    C1,
    C2,
    C3,
    C4,
}

#[derive(Clone)]
pub enum Adt17 {
    C0(u64),
    C1(u64),
    C2(u64),
    C3(u64),
    C4(u64),
    C5(u64),
    C6(u64),
    C7(u64),
    C8(u64),
    C9(Bytes),
}

#[derive(Clone)]
pub enum Adt18 {
    C0,
    C1,
    C2,
    C3,
}

#[derive(Clone)]
pub enum Adt19 {
    C0(u64),
    C1(Adt20),
    C2(u64, Option<Adt13>),
}

#[derive(Clone)]
pub enum Adt20 {
    C0(u64, Option<Adt13>),
}

#[derive(Clone)]
pub enum Adt21 {
    C0(Adt22),
    C1,
    C2(Adt22, Adt22),
    C3(Adt22, Adt22),
    C4(u64, u64),
    C5(u64, u64),
    C6,
    C7(Adt22),
}

#[derive(Clone)]
pub enum Adt22 {
    C0(Adt23),
    C1(Adt13),
    C2(Adt24, std::rc::Rc<Adt22>),
    C3(Adt25, std::rc::Rc<Adt22>, std::rc::Rc<Adt22>),
    C4(Adt26, u64),
    C5,
    C6(std::rc::Rc<Adt22>, std::rc::Rc<Adt22>),
}

#[derive(Clone)]
pub enum Adt23 {
    C0,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
}

#[derive(Clone)]
pub enum Adt24 {
    C0,
    C1,
    C2,
    C3,
    C4,
}

#[derive(Clone)]
pub enum Adt25 {
    C0,
    C1,
}

#[derive(Clone)]
pub enum Adt26 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt27 {
    C0,
    C1,
    C2,
    C3,
}

#[derive(Clone)]
pub enum Adt28 {
    C0(Adt29),
    C1(Adt17),
    C2(std::rc::Rc<Adt28>),
    C3(Adt30, std::rc::Rc<Adt28>),
    C4(u64, u64, std::rc::Rc<Adt28>),
    C5,
    C6(std::rc::Rc<Adt28>, std::rc::Rc<Adt28>),
    C7(Adt14),
}

#[derive(Clone)]
pub enum Adt29 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt30 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt31 {
    C0(Bytes, Adt0, List<List<u64>>, Adt0, Adt0, Adt2),
}

#[derive(Clone)]
pub enum Adt32 {
    C0(Adt31, Adt0, Adt8, Adt0, Adt0, Adt0, List<bool>, Adt9, Adt0),
}

#[derive(Clone)]
pub enum Adt33 {
    C0(List<(u64, u64)>, List<Bytes>, List<(Adt17, u64)>, u64),
}

#[derive(Clone)]
pub enum Adt34 {
    C0(Adt0, Adt21, bool),
}

#[derive(Clone)]
pub enum Adt35 {
    C0(Adt11, Adt28, u64),
}

#[derive(Clone)]
pub enum Fn0 {
    F120(Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn0::F120(k0) => f120(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F168(Bytes),
}

impl Fn1 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn1::F168(k0) => f168(k0.clone(), p0),
        }
    }
}

pub fn f0(v0: Bytes, v1: Adt0, v2: List<List<u64>>, v3: Adt0, v4: Adt0, v5: Adt2, v6: Adt0, v7: Adt0, v8: List<bool>, v9: List<bool>, v10: Adt7, v11: u64) -> R<Adt10> {
    let v12: Adt31 = Adt31::C0(v0, v1.clone(), v2, v3.clone(), v4.clone(), v5.clone());
    let v13: Adt32 = Adt32::C0(v12, match v10.clone() {
        Adt7::C0(h1, _, _, _) => {
            h1
        }
    }, match v10.clone() {
        Adt7::C0(_, h2, _, _) => {
            h2
        }
    }, v1, v3, v4, v9, match v10.clone() {
        Adt7::C0(_, _, _, h3) => {
            h3
        }
    }, match v10 {
        Adt7::C0(_, _, h4, _) => {
            h4
        }
    });
    f1(v13, v5, v6, v7, v8, 0u64, v11)
}

pub fn f1(v0: Adt32, v1: Adt2, v2: Adt0, v3: Adt0, v4: List<bool>, v5: u64, v6: u64) -> R<Adt10> {
    let m5 = v1;
    match m5 {
        Adt2::C0(_, _) => {
            Ok(f2(Adt11::C12, v6))
        }
        Adt2::C1(_) => {
            Ok(f2(Adt11::C12, v6))
        }
        Adt2::C2(_, _, _, _) => {
            Ok(f2(Adt11::C12, v6))
        }
        Adt2::C3(_, _, _) => {
            Ok(f2(Adt11::C12, v6))
        }
        Adt2::C4(_, _, _) => {
            Ok(f2(Adt11::C12, v6))
        }
        Adt2::C5(_, _, _) => {
            Ok(f2(Adt11::C12, v6))
        }
        Adt2::C6(_, _) => {
            Ok(f2(Adt11::C12, v6))
        }
        Adt2::C7(_, _) => {
            Ok(f2(Adt11::C12, v6))
        }
        Adt2::C8(_, _) => {
            Ok(f2(Adt11::C12, v6))
        }
        Adt2::C9(_, _) => {
            Ok(f2(Adt11::C12, v6))
        }
        Adt2::C10(_) => {
            Ok(f2(Adt11::C12, v6))
        }
        Adt2::C11(_) => {
            Ok(f2(Adt11::C12, v6))
        }
        Adt2::C12(_, _) => {
            Ok(f2(Adt11::C12, v6))
        }
        Adt2::C13 => {
            Ok(f2(Adt11::C12, v6))
        }
        Adt2::C14(r18, r19) => {
            let v35 = (*r18).clone();
            let v36 = (*r19).clone();
            if f3(v4.clone(), v5) {
                let v37: Adt10 = f1(v0, v36, f4(v2), f4(v3), v4, {
                    let a6 = v5;
                    let a7 = 1u64;
                    nat_add(a6, a7)?
                }, v6)?;
                Ok(f2(Adt11::C13(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(match v37.clone() {
                    Adt10::C0(h8, _) => {
                        h8
                    }
                })), match v37 {
                    Adt10::C0(_, h9) => {
                        h9
                    }
                }))
            } else {
                let v38: List<(u64, Adt0)> = f5(f6(match v0.clone() {
                    Adt32::C0(h10, _, _, _, _, _, _, _, _) => {
                        h10
                    }
                }, v5), f7(match v0.clone() {
                    Adt32::C0(h11, _, _, _, _, _, _, _, _) => {
                        h11
                    }
                }, v5), List::<(u64, Adt0)>::nil());
                let v39: Adt10 = f8(v0.clone(), v35.clone(), f9(v2.clone()), f9(v3.clone()), v38, Adt0::C14, 0u64, f10(v35), v6)?;
                let v40: Adt10 = f1(v0, v36, f4(v2), f4(v3), v4, {
                    let a12 = v5;
                    let a13 = 1u64;
                    nat_add(a12, a13)?
                }, match v39.clone() {
                    Adt10::C0(_, h14) => {
                        h14
                    }
                })?;
                Ok(f2(Adt11::C13(std::rc::Rc::new(f11(match v39 {
                    Adt10::C0(h15, _) => {
                        h15
                    }
                })), std::rc::Rc::new(match v40.clone() {
                    Adt10::C0(h16, _) => {
                        h16
                    }
                })), match v40 {
                    Adt10::C0(_, h17) => {
                        h17
                    }
                }))
            }
        }
        Adt2::C15(_, _, _) => {
            Ok(f2(Adt11::C12, v6))
        }
    }
}

pub fn f2(v0: Adt11, v1: u64) -> Adt10 {
    Adt10::C0(v0, v1)
}

pub fn f3(v0: List<bool>, v1: u64) -> bool {
    let m20 = f12(v0, v1);
    match m20 {
        None => {
            false
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f4(v0: Adt0) -> Adt0 {
    f13(f14(f14(v0)))
}

pub fn f5(v0: List<u64>, v1: Adt0, v2: List<(u64, Adt0)>) -> List<(u64, Adt0)> {
    let m21 = v0;
    match m21.uncons() {
        None => {
            v2
        }
        Some((v3, v4)) => {
            let v5: List<(u64, Adt0)> = f5(v4, f14(v1.clone()), f15(v3, f13(v1.clone()), v2.clone()));
            if f16(v1) {
                v2
            } else {
                v5
            }
        }
    }
}

pub fn f6(v0: Adt31, v1: u64) -> List<u64> {
    let m23 = f17(match v0 {
        Adt31::C0(_, _, h22, _, _, _) => {
            h22
        }
    }, v1);
    match m23 {
        None => {
            List::<u64>::nil()
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f7(v0: Adt31, v1: u64) -> Adt0 {
    let m24 = f18(v0, v1);
    match m24 {
        None => {
            Adt0::C14
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f8(v0: Adt32, v1: Adt2, v2: Adt0, v3: Adt0, v4: List<(u64, Adt0)>, v5: Adt0, v6: u64, v7: Adt15, v8: u64) -> R<Adt10> {
    let m25 = v1;
    match m25 {
        Adt2::C0(v9, v10) => {
            Ok(f2(f19(match v0.clone() {
                Adt32::C0(_, h26, _, _, _, _, _, _, _) => {
                    h26
                }
            }, match v0.clone() {
                Adt32::C0(_, _, h27, _, _, _, _, _, _) => {
                    h27
                }
            }, match v0 {
                Adt32::C0(_, _, _, h28, _, _, _, _, _) => {
                    h28
                }
            }, Adt0::C14, 0u64, v10, v9)?, v8))
        }
        Adt2::C1(v11) => {
            let v12: Adt0 = f20(v4, v11);
            Ok(f2(if f21(v12.clone(), Adt0::C0) {
                Adt11::C0(Adt12::C0, v7)
            } else {
                Adt11::C1(if f22(v12) {
                    Adt16::C2
                } else {
                    Adt16::C1
                }, Adt17::C0(v11), v7)
            }, v8))
        }
        Adt2::C2(v13, v14, r152, r153) => {
            let v15 = (*r152).clone();
            let v16 = (*r153).clone();
            let v17: Adt10 = f8(v0.clone(), v15.clone(), f9(v2.clone()), f9(v3.clone()), v4.clone(), Adt0::C14, 0u64, f10(v15), v8)?;
            let v18: Adt10 = f8(v0.clone(), v16.clone(), f4(v2), f4(v3), f15(v13, v14.clone(), v4), Adt0::C14, 0u64, f10(v16.clone()), match v17.clone() {
                Adt10::C0(_, h29) => {
                    h29
                }
            })?;
            let v19: Adt11 = f11(match v18.clone() {
                Adt10::C0(h30, _) => {
                    h30
                }
            });
            Ok(f2(Adt11::C9(std::rc::Rc::new(Adt11::C11(std::rc::Rc::new(Adt11::C14(f23(v13, v14.clone(), v16), true, f24(match v0 {
                Adt32::C0(_, h31, _, _, _, _, _, _, _) => {
                    h31
                }
            }, v14)?, std::rc::Rc::new(match v17 {
                Adt10::C0(h32, _) => {
                    h32
                }
            }), v7, std::rc::Rc::new(f25(v19.clone())))), std::rc::Rc::new(f26(v19))))), match v18 {
                Adt10::C0(_, h33) => {
                    h33
                }
            }))
        }
        Adt2::C3(r154, r155, r156) => {
            let v20 = (*r154).clone();
            let v21 = (*r155).clone();
            let v22 = (*r156).clone();
            let v23: Adt10 = f8(v0.clone(), v20.clone(), f9(v2.clone()), f9(v3.clone()), v4.clone(), Adt0::C14, 0u64, f10(v20), v8)?;
            let v24: Adt11 = f11(match v23.clone() {
                Adt10::C0(h34, _) => {
                    h34
                }
            });
            let v25: Adt10 = f8(v0.clone(), v21.clone(), f4(v2.clone()), f4(v3.clone()), v4.clone(), Adt0::C14, 0u64, f10(v21), match v23 {
                Adt10::C0(_, h35) => {
                    h35
                }
            })?;
            let v26: Adt10 = f8(v0, v22.clone(), f27(v2), f27(v3), v4, Adt0::C14, 0u64, f10(v22), match v25.clone() {
                Adt10::C0(_, h36) => {
                    h36
                }
            })?;
            let v27: Adt10 = f28(f26(v24.clone()), f11(match v25 {
                Adt10::C0(h37, _) => {
                    h37
                }
            }), f11(match v26.clone() {
                Adt10::C0(h38, _) => {
                    h38
                }
            }), v7.clone(), match v26 {
                Adt10::C0(_, h39) => {
                    h39
                }
            })?;
            let v28: Adt11 = f29(Adt11::C11(std::rc::Rc::new(f25(v24)), std::rc::Rc::new(Adt11::C0(Adt12::C0, v7))), match v27.clone() {
                Adt10::C0(h40, _) => {
                    h40
                }
            });
            Ok(f2(if f30(f25(v28.clone())) {
                f26(v28)
            } else {
                Adt11::C9(std::rc::Rc::new(v28))
            }, match v27 {
                Adt10::C0(_, h41) => {
                    h41
                }
            }))
        }
        Adt2::C4(_, r157, r158) => {
            let v30 = (*r157).clone();
            let v31 = (*r158).clone();
            let v32: Adt0 = f31(v2.clone());
            let v33: Adt10 = f8(v0.clone(), v30.clone(), f9(v2.clone()), f9(v3.clone()), v4.clone(), Adt0::C14, 0u64, f10(v30), v8)?;
            if f21(v32.clone(), Adt0::C0) {
                let v34: Adt10 = f8(v0, v31.clone(), f4(v2), f4(v3), v4, Adt0::C14, 3u64, f10(v31), match v33.clone() {
                    Adt10::C0(_, h42) => {
                        h42
                    }
                })?;
                let v35: Adt11 = f11(match v34.clone() {
                    Adt10::C0(h43, _) => {
                        h43
                    }
                });
                Ok(f2(Adt11::C9(std::rc::Rc::new(Adt11::C11(std::rc::Rc::new(Adt11::C14(Adt28::C0(Adt29::C1), false, f32(0u64), std::rc::Rc::new(match v33 {
                    Adt10::C0(h44, _) => {
                        h44
                    }
                }), f33(Adt5::C11), std::rc::Rc::new(f25(v35.clone())))), std::rc::Rc::new(f26(v35))))), match v34 {
                    Adt10::C0(_, h45) => {
                        h45
                    }
                }))
            } else {
                let v36: Adt17 = Adt17::C1({
                    let a47 = match v33.clone() {
                        Adt10::C0(_, h46) => {
                            h46
                        }
                    };
                    let a48 = 1u64;
                    nat_add(a47, a48)?
                });
                let v37: Adt10 = f8(v0.clone(), v31.clone(), f4(v2), f4(v3), v4, v32.clone(), 2u64, f10(v31.clone()), {
                    let a50 = match v33.clone() {
                        Adt10::C0(_, h49) => {
                            h49
                        }
                    };
                    let a51 = 1u64;
                    nat_add(a50, a51)?
                })?;
                let v38: Adt10 = f34(v0, v36.clone(), v32, v31, match v37.clone() {
                    Adt10::C0(h52, _) => {
                        h52
                    }
                }, match v37 {
                    Adt10::C0(_, h53) => {
                        h53
                    }
                })?;
                if f35(match v38.clone() {
                    Adt10::C0(h54, _) => {
                        h54
                    }
                }, v36.clone()) {
                    Ok(f2(match v33 {
                        Adt10::C0(h55, _) => {
                            h55
                        }
                    }, match v38 {
                        Adt10::C0(_, h56) => {
                            h56
                        }
                    }))
                } else {
                    let v39: Adt11 = f11(match v38.clone() {
                        Adt10::C0(h57, _) => {
                            h57
                        }
                    });
                    Ok(f2(Adt11::C9(std::rc::Rc::new(Adt11::C11(std::rc::Rc::new(Adt11::C14(Adt28::C1(v36), false, f32(0u64), std::rc::Rc::new(match v33 {
                        Adt10::C0(h58, _) => {
                            h58
                        }
                    }), v7, std::rc::Rc::new(f25(v39.clone())))), std::rc::Rc::new(f26(v39))))), match v38 {
                        Adt10::C0(_, h59) => {
                            h59
                        }
                    }))
                }
            }
        }
        Adt2::C5(v40, v41, r159) => {
            let v42 = (*r159).clone();
            let v43: Adt10 = f8(v0.clone(), v42.clone(), f9(v2), f9(v3), v4, Adt0::C14, 1u64, f10(v42), v8)?;
            f36(match v0.clone() {
                Adt32::C0(_, h60, _, _, _, _, _, _, _) => {
                    h60
                }
            }, match v0.clone() {
                Adt32::C0(_, _, h61, _, _, _, _, _, _) => {
                    h61
                }
            }, match v0 {
                Adt32::C0(_, _, _, h62, _, _, _, _, _) => {
                    h62
                }
            }, v40.clone(), v41, match v43.clone() {
                Adt10::C0(h63, _) => {
                    h63
                }
            }, f33(v40), match v43 {
                Adt10::C0(_, h64) => {
                    h64
                }
            })
        }
        Adt2::C6(v44, r160) => {
            let v45 = (*r160).clone();
            let v46: Adt10 = f8(v0.clone(), v45.clone(), f9(v2), f9(v3), v4, Adt0::C14, 1u64, f10(v45), v8)?;
            let v47: Adt10 = f37(match v46.clone() {
                Adt10::C0(h65, _) => {
                    h65
                }
            }, f38(v0.clone(), v44), v7.clone(), match v46 {
                Adt10::C0(_, h66) => {
                    h66
                }
            })?;
            Ok(f2(f39(f25(match v47.clone() {
                Adt10::C0(h67, _) => {
                    h67
                }
            }), Adt11::C3(Adt19::C0(v44), std::rc::Rc::new(f26(match v47.clone() {
                Adt10::C0(h68, _) => {
                    h68
                }
            })), f3(match v0 {
                Adt32::C0(_, _, _, _, _, _, h69, _, _) => {
                    h69
                }
            }, v44), v7)), match v47 {
                Adt10::C0(_, h70) => {
                    h70
                }
            }))
        }
        Adt2::C7(v48, r161) => {
            let v49 = (*r161).clone();
            let v50: Adt10 = f8(v0.clone(), v49.clone(), f9(v2), f9(v3), v4, Adt0::C14, 1u64, f10(v49.clone()), v8)?;
            let v51: u64 = f40(v49)?;
            let v52: Adt0 = f41(f38(v0.clone(), v48), v51);
            let v53: Adt0 = Adt0::C13(std::rc::Rc::new(f42(f38(v0.clone(), v48), v51)), std::rc::Rc::new(f43(v0.clone(), v48)));
            let v54: u64 = f44(match v0.clone() {
                Adt32::C0(_, h71, _, _, _, _, _, _, _) => {
                    h71
                }
            }, v53.clone(), 0u64)?;
            let v55: Adt10 = f37(match v50.clone() {
                Adt10::C0(h72, _) => {
                    h72
                }
            }, v52.clone(), v7.clone(), match v50 {
                Adt10::C0(_, h73) => {
                    h73
                }
            })?;
            Ok(f2(f39(f25(match v55.clone() {
                Adt10::C0(h74, _) => {
                    h74
                }
            }), Adt11::C5(Adt21::C5(v54, v48), std::rc::Rc::new(f45(match v0.clone() {
                Adt32::C0(_, h75, _, _, _, _, _, _, _) => {
                    h75
                }
            }, match v0 {
                Adt32::C0(_, _, h76, _, _, _, _, _, _) => {
                    h76
                }
            }, v53, f26(match v55.clone() {
                Adt10::C0(h77, _) => {
                    h77
                }
            }), v52)), v7)), match v55 {
                Adt10::C0(_, h78) => {
                    h78
                }
            }))
        }
        Adt2::C8(r162, r163) => {
            let v56 = (*r162).clone();
            let v57 = (*r163).clone();
            let v58: Adt0 = f31(v2.clone());
            let v59: (Adt0, Adt0) = f46(v58.clone());
            let v60: u64 = f44(match v0.clone() {
                Adt32::C0(_, h79, _, _, _, _, _, _, _) => {
                    h79
                }
            }, v58.clone(), 0u64)?;
            let v61: Adt17 = Adt17::C3({
                let a80 = v8;
                let a81 = 1u64;
                nat_add(a80, a81)?
            });
            let v62: Adt10 = f8(v0.clone(), v56.clone(), f9(v2.clone()), f9(v3.clone()), v4.clone(), Adt0::C14, 0u64, f10(v56), {
                let a82 = v8;
                let a83 = 1u64;
                nat_add(a82, a83)?
            })?;
            let v63: Adt10 = f8(v0.clone(), v57.clone(), f4(v2), f4(v3), v4, Adt0::C14, 1u64, f10(v57), match v62.clone() {
                Adt10::C0(_, h84) => {
                    h84
                }
            })?;
            let v64: Adt10 = f37(match v63.clone() {
                Adt10::C0(h85, _) => {
                    h85
                }
            }, {
                let (h86, _) = v59;
                h86
            }, v7.clone(), match v63 {
                Adt10::C0(_, h87) => {
                    h87
                }
            })?;
            Ok(f2(Adt11::C9(std::rc::Rc::new(Adt11::C11(std::rc::Rc::new(Adt11::C14(Adt28::C1(v61.clone()), true, Adt22::C4(Adt26::C1, v60), std::rc::Rc::new(match v62 {
                Adt10::C0(h88, _) => {
                    h88
                }
            }), v7.clone(), std::rc::Rc::new(f25(match v64.clone() {
                Adt10::C0(h89, _) => {
                    h89
                }
            })))), std::rc::Rc::new(Adt11::C4(v61, std::rc::Rc::new(f26(match v64.clone() {
                Adt10::C0(h90, _) => {
                    h90
                }
            })), f47(v58, match v0.clone() {
                Adt32::C0(_, _, _, _, _, _, h91, _, _) => {
                    h91
                }
            }, match v0 {
                Adt32::C0(_, _, _, _, _, _, _, h92, _) => {
                    h92
                }
            }), v7))))), match v64 {
                Adt10::C0(_, h93) => {
                    h93
                }
            }))
        }
        Adt2::C9(v65, r164) => {
            let v66 = (*r164).clone();
            let v67: Adt0 = f48(v66.clone(), f9(v3.clone()));
            let v68: Adt10 = f8(v0, v66.clone(), f9(v2), f9(v3), v4, Adt0::C14, 1u64, f10(v66), v8)?;
            let v69: u64 = f49(v65.clone(), v67.clone());
            let v70: Adt15 = f50(v65.clone(), v67.clone());
            let v71: Adt10 = f51(match v68.clone() {
                Adt10::C0(h94, _) => {
                    h94
                }
            }, {
                let a95 = v69;
                let a96 = 31u64;
                nat_eq(a95, a96)
            }, v70.clone(), v7, match v68 {
                Adt10::C0(_, h97) => {
                    h97
                }
            })?;
            Ok(f2(Adt11::C9(std::rc::Rc::new(Adt11::C11(std::rc::Rc::new(f25(match v71.clone() {
                Adt10::C0(h98, _) => {
                    h98
                }
            })), std::rc::Rc::new(Adt11::C3(Adt19::C1(Adt20::C0(v69, f52(v65, v67))), std::rc::Rc::new(f26(match v71.clone() {
                Adt10::C0(h99, _) => {
                    h99
                }
            })), f53(v69), v70))))), match v71 {
                Adt10::C0(_, h100) => {
                    h100
                }
            }))
        }
        Adt2::C10(r165) => {
            let v72 = (*r165).clone();
            let v73: Adt17 = Adt17::C5({
                let a101 = v8;
                let a102 = 1u64;
                nat_add(a101, a102)?
            });
            let v74: Adt10 = f8(v0, v72.clone(), f9(v2), f9(v3), v4, Adt0::C14, 0u64, f10(v72), {
                let a103 = v8;
                let a104 = 1u64;
                nat_add(a103, a104)?
            })?;
            Ok(f2(Adt11::C9(std::rc::Rc::new(Adt11::C11(std::rc::Rc::new(Adt11::C14(Adt28::C2(std::rc::Rc::new(Adt28::C6(std::rc::Rc::new(Adt28::C1(v73.clone())), std::rc::Rc::new(Adt28::C6(std::rc::Rc::new(Adt28::C0(Adt29::C0)), std::rc::Rc::new(Adt28::C5)))))), false, f32(0u64), std::rc::Rc::new(match v74.clone() {
                Adt10::C0(h105, _) => {
                    h105
                }
            }), v7.clone(), std::rc::Rc::new(Adt11::C12))), std::rc::Rc::new(Adt11::C1(Adt16::C0, v73, v7))))), match v74 {
                Adt10::C0(_, h106) => {
                    h106
                }
            }))
        }
        Adt2::C11(r166) => {
            let v75 = (*r166).clone();
            let v76: Adt17 = Adt17::C5({
                let a107 = v8;
                let a108 = 1u64;
                nat_add(a107, a108)?
            });
            let v77: Adt10 = f8(v0, v75.clone(), f9(v2), f9(v3), v4, Adt0::C14, 0u64, f10(v75), {
                let a109 = v8;
                let a110 = 1u64;
                nat_add(a109, a110)?
            })?;
            Ok(f2(Adt11::C9(std::rc::Rc::new(Adt11::C11(std::rc::Rc::new(Adt11::C14(Adt28::C2(std::rc::Rc::new(Adt28::C6(std::rc::Rc::new(Adt28::C0(Adt29::C0)), std::rc::Rc::new(Adt28::C6(std::rc::Rc::new(Adt28::C1(v76.clone())), std::rc::Rc::new(Adt28::C5)))))), false, f32(0u64), std::rc::Rc::new(match v77.clone() {
                Adt10::C0(h111, _) => {
                    h111
                }
            }), v7.clone(), std::rc::Rc::new(Adt11::C12))), std::rc::Rc::new(Adt11::C1(Adt16::C0, v76, v7))))), match v77 {
                Adt10::C0(_, h112) => {
                    h112
                }
            }))
        }
        Adt2::C12(r167, v79) => {
            let v78 = (*r167).clone();
            let m113 = f54(f31(f9(v3.clone())));
            match m113 {
                None => {
                    Ok(f2(Adt11::C12, v8))
                }
                Some(v80) => {
                    let m115 = f55(match v0.clone() {
                        Adt32::C0(_, _, _, h114, _, _, _, _, _) => {
                            h114
                        }
                    }, v80, 0u64);
                    match m115 {
                        None => {
                            Ok(f2(Adt11::C12, v8))
                        }
                        Some(v81) => {
                            let m116 = f56(v81.clone(), v79);
                            match m116 {
                                None => {
                                    Ok(f2(Adt11::C12, v8))
                                }
                                Some(v82) => {
                                    let v83: Adt17 = Adt17::C5({
                                        let a117 = v8;
                                        let a118 = 1u64;
                                        nat_add(a117, a118)?
                                    });
                                    let v84: Adt11 = if f57(match v0.clone() {
                                        Adt32::C0(_, h119, _, _, _, _, _, _, _) => {
                                            h119
                                        }
                                    }, match v0.clone() {
                                        Adt32::C0(_, _, h120, _, _, _, _, _, _) => {
                                            h120
                                        }
                                    }, Adt0::C12(v80), v82) {
                                        Adt11::C1(Adt16::C4, v83.clone(), Adt15::C0({
                                            let a121 = Str::lit("indirection");
                                            utf8_encode(a121)
                                        }, None::<Adt13>))
                                    } else {
                                        Adt11::C1(Adt16::C0, v83.clone(), v7.clone())
                                    };
                                    let v85: Adt10 = f8(v0, v78.clone(), f9(v2), f9(v3), v4, Adt0::C14, 0u64, f10(v78), {
                                        let a122 = v8;
                                        let a123 = 1u64;
                                        nat_add(a122, a123)?
                                    })?;
                                    let v86: Adt28 = Adt28::C4(v80, 0u64, std::rc::Rc::new(f58(v81, v79, v83, 0u64)?));
                                    if f59(match v85.clone() {
                                        Adt10::C0(h124, _) => {
                                            h124
                                        }
                                    }) {
                                        Ok(f2(Adt11::C8(std::rc::Rc::new(match v85.clone() {
                                            Adt10::C0(h125, _) => {
                                                h125
                                            }
                                        }), std::rc::Rc::new(Adt11::C15(v86, std::rc::Rc::new(Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(v84))), std::rc::Rc::new(Adt11::C12))), v7), match v85 {
                                            Adt10::C0(_, h126) => {
                                                h126
                                            }
                                        }))
                                    } else {
                                        let v87: Adt17 = Adt17::C1({
                                            let a128 = match v85.clone() {
                                                Adt10::C0(_, h127) => {
                                                    h127
                                                }
                                            };
                                            let a129 = 1u64;
                                            nat_add(a128, a129)?
                                        });
                                        Ok(f2(Adt11::C9(std::rc::Rc::new(Adt11::C11(std::rc::Rc::new(Adt11::C14(Adt28::C1(v87.clone()), false, f32(0u64), std::rc::Rc::new(match v85.clone() {
                                            Adt10::C0(h130, _) => {
                                                h130
                                            }
                                        }), v7.clone(), std::rc::Rc::new(Adt11::C12))), std::rc::Rc::new(Adt11::C8(std::rc::Rc::new(Adt11::C1(Adt16::C0, v87, v7.clone())), std::rc::Rc::new(Adt11::C15(v86, std::rc::Rc::new(Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(v84))), std::rc::Rc::new(Adt11::C12))), v7))))), {
                                            let a132 = match v85 {
                                                Adt10::C0(_, h131) => {
                                                    h131
                                                }
                                            };
                                            let a133 = 1u64;
                                            nat_add(a132, a133)?
                                        }))
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Adt2::C13 => {
            Ok(f2(Adt11::C12, v8))
        }
        Adt2::C14(r168, r169) => {
            let v88 = (*r168).clone();
            let v89 = (*r169).clone();
            let a134 = v6;
            let a135 = 3u64;
            if nat_eq(a134, a135) {
                f8(v0, v88.clone(), f9(v2), f9(v3), v4, Adt0::C14, 5u64, f10(v88), v8)
            } else {
                let a136 = v6;
                let a137 = 2u64;
                if nat_eq(a136, a137) {
                    let v90: Adt10 = f8(v0.clone(), v88.clone(), f9(v2.clone()), f9(v3.clone()), v4.clone(), v5.clone(), 4u64, f10(v88), v8)?;
                    let v91: Adt10 = f8(v0, v89.clone(), f4(v2), f4(v3), v4, v5, 2u64, f10(v89), match v90.clone() {
                        Adt10::C0(_, h138) => {
                            h138
                        }
                    })?;
                    Ok(f2(Adt11::C13(std::rc::Rc::new(match v90 {
                        Adt10::C0(h139, _) => {
                            h139
                        }
                    }), std::rc::Rc::new(match v91.clone() {
                        Adt10::C0(h140, _) => {
                            h140
                        }
                    })), match v91 {
                        Adt10::C0(_, h141) => {
                            h141
                        }
                    }))
                } else {
                    let v92: Adt10 = f8(v0.clone(), v88.clone(), f9(v2.clone()), f9(v3.clone()), v4.clone(), Adt0::C14, 0u64, f10(v88), v8)?;
                    let v93: Adt10 = f8(v0, v89.clone(), f4(v2), f4(v3), v4, v5, 1u64, f10(v89), match v92.clone() {
                        Adt10::C0(_, h142) => {
                            h142
                        }
                    })?;
                    Ok(f2(Adt11::C13(std::rc::Rc::new(match v92 {
                        Adt10::C0(h143, _) => {
                            h143
                        }
                    }), std::rc::Rc::new(match v93.clone() {
                        Adt10::C0(h144, _) => {
                            h144
                        }
                    })), match v93 {
                        Adt10::C0(_, h145) => {
                            h145
                        }
                    }))
                }
            }
        }
        Adt2::C15(v94, v95, r170) => {
            let v96 = (*r170).clone();
            let a146 = v6;
            let a147 = 5u64;
            if nat_eq(a146, a147) {
                f8(v0, v96.clone(), f9(v2), f9(v3), v4, Adt0::C14, 0u64, f10(v96), v8)
            } else {
                let v97: Adt0 = f60(match v0.clone() {
                    Adt32::C0(_, _, _, h148, _, _, _, _, _) => {
                        h148
                    }
                }, v5, v94);
                if f61(v97.clone(), match v0.clone() {
                    Adt32::C0(_, _, _, _, _, _, _, _, h149) => {
                        h149
                    }
                }) {
                    let v98: Adt10 = f8(v0, v96.clone(), f9(v2), f9(v3), f5(v95.clone(), v97.clone(), v4), Adt0::C14, 0u64, f10(v96.clone()), v8)?;
                    Ok(f2(Adt11::C15(Adt28::C2(std::rc::Rc::new(f62(v95, v97, v96))), std::rc::Rc::new(f11(match v98.clone() {
                        Adt10::C0(h150, _) => {
                            h150
                        }
                    })), std::rc::Rc::new(Adt11::C12)), match v98 {
                        Adt10::C0(_, h151) => {
                            h151
                        }
                    }))
                } else {
                    Ok(f2(Adt11::C12, v8))
                }
            }
        }
    }
}

pub fn f9(v0: Adt0) -> Adt0 {
    f13(f14(v0))
}

pub fn f10(v0: Adt2) -> Adt15 {
    Adt15::C0({
        let a172 = {
            let a171 = Str::lit("expr:");
            utf8_encode(a171)
        };
        let a173 = f63(v0);
        append_bytes(a172, a173)
    }, None::<Adt13>)
}

pub fn f11(v0: Adt11) -> Adt11 {
    f64(v0.clone(), v0)
}

pub fn f12(p177: List<bool>, p178: u64) -> Option<bool> {
    let m176 = (p177, p178);
    {
        let mut m176 = m176;
        loop {
            let step = {
                let (v0, v1) = m176;
                let m174 = v0;
                match m174.uncons() {
                    None => {
                        Err::<(List<bool>, u64), Option<bool>>(None::<bool>)
                    }
                    Some((v2, v3)) => {
                        let m175 = v1;
                        if m175 == 0 {
                            Err::<(List<bool>, u64), Option<bool>>(Some(v2))
                        } else {
                            let v4 = m175 - 1;
                            Ok::<(List<bool>, u64), Option<bool>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m176 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f13(v0: Adt0) -> Adt0 {
    let m179 = f65(v0);
    match m179 {
        None => {
            Adt0::C0
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f14(v0: Adt0) -> Adt0 {
    let m180 = f66(v0);
    match m180 {
        None => {
            Adt0::C0
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f15(v0: u64, v1: Adt0, v2: List<(u64, Adt0)>) -> List<(u64, Adt0)> {
    List::cons((v0, v1), v2)
}

pub fn f16(v0: Adt0) -> bool {
    let a181 = f67(v0);
    let a182 = 14u64;
    nat_eq(a181, a182)
}

pub fn f17(p186: List<List<u64>>, p187: u64) -> Option<List<u64>> {
    let m185 = (p186, p187);
    {
        let mut m185 = m185;
        loop {
            let step = {
                let (v0, v1) = m185;
                let m183 = v0;
                match m183.uncons() {
                    None => {
                        Err::<(List<List<u64>>, u64), Option<List<u64>>>(None::<List<u64>>)
                    }
                    Some((v2, v3)) => {
                        let m184 = v1;
                        if m184 == 0 {
                            Err::<(List<List<u64>>, u64), Option<List<u64>>>(Some(v2))
                        } else {
                            let v4 = m184 - 1;
                            Ok::<(List<List<u64>>, u64), Option<List<u64>>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m185 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f18(v0: Adt31, v1: u64) -> Option<Adt0> {
    f68(match v0 {
        Adt31::C0(_, _, _, h188, _, _) => {
            h188
        }
    }, v1)
}

pub fn f19(v0: Adt0, v1: Adt8, v2: Adt0, v3: Adt0, v4: u64, v5: Adt3, v6: Adt0) -> R<Adt11> {
    let m189 = v5;
    match m189 {
        Adt3::C0 => {
            Ok(Adt11::C0(Adt12::C0, f69({
                let a190 = Str::lit("value:unit");
                utf8_encode(a190)
            }, v6)))
        }
        Adt3::C1(v7) => {
            Ok(Adt11::C0(Adt12::C1(v7), f69({
                let a191 = Str::lit("value:bool");
                utf8_encode(a191)
            }, v6)))
        }
        Adt3::C2(v8) => {
            let m192 = f70(v8)?;
            match m192 {
                None => {
                    Ok(f71(0u64))
                }
                Some(v9) => {
                    Ok(Adt11::C0(Adt12::C2(v9), f69({
                        let a193 = Str::lit("value:nat");
                        utf8_encode(a193)
                    }, v6)))
                }
            }
        }
        Adt3::C3(v10) => {
            let m194 = f72(v10)?;
            match m194 {
                None => {
                    Ok(f71(0u64))
                }
                Some(v11) => {
                    Ok(Adt11::C0(Adt12::C3({
                        let (h195, _) = v11;
                        h195
                    }, {
                        let (_, h196) = v11;
                        h196
                    }), f69({
                        let a197 = Str::lit("value:int");
                        utf8_encode(a197)
                    }, v6)))
                }
            }
        }
        Adt3::C4(v12) => {
            f73(v12, v6, {
                let a198 = Str::lit("value:u8");
                utf8_encode(a198)
            })
        }
        Adt3::C5(v13) => {
            f73(v13, v6, {
                let a199 = Str::lit("value:u16");
                utf8_encode(a199)
            })
        }
        Adt3::C6(v14) => {
            f73(v14, v6, {
                let a200 = Str::lit("value:u32");
                utf8_encode(a200)
            })
        }
        Adt3::C7(v15) => {
            f73(v15, v6, {
                let a201 = Str::lit("value:u64");
                utf8_encode(a201)
            })
        }
        Adt3::C8(v16) => {
            f73(v16, v6, {
                let a202 = Str::lit("value:i8");
                utf8_encode(a202)
            })
        }
        Adt3::C9(v17) => {
            f73(v17, v6, {
                let a203 = Str::lit("value:i16");
                utf8_encode(a203)
            })
        }
        Adt3::C10(v18) => {
            f73(v18, v6, {
                let a204 = Str::lit("value:i32");
                utf8_encode(a204)
            })
        }
        Adt3::C11(v19) => {
            f73(v19, v6, {
                let a205 = Str::lit("value:i64");
                utf8_encode(a205)
            })
        }
        Adt3::C12(v20) => {
            Ok(Adt11::C0(Adt12::C5(v20), f69({
                let a206 = Str::lit("value:string");
                utf8_encode(a206)
            }, v6)))
        }
        Adt3::C13(v21) => {
            let m207 = f74(v21)?;
            match m207 {
                None => {
                    Ok(f71(0u64))
                }
                Some(v22) => {
                    Ok(Adt11::C0(Adt12::C6(v22), f69({
                        let a208 = Str::lit("value:bytes");
                        utf8_encode(a208)
                    }, v6)))
                }
            }
        }
        Adt3::C14(v23) => {
            Ok(Adt11::C0(Adt12::C7({
                let m209 = v23;
                match m209 {
                    Adt4::C0 => {
                        Adt14::C0
                    }
                    Adt4::C1 => {
                        Adt14::C1
                    }
                    Adt4::C2 => {
                        Adt14::C2
                    }
                }
            }), f69({
                let a210 = Str::lit("value:ordering");
                utf8_encode(a210)
            }, v6)))
        }
        Adt3::C15 => {
            let v24: Adt0 = f13(v6.clone());
            Ok(Adt11::C5(Adt21::C0(f24(v0, v24)?), std::rc::Rc::new(Adt11::C12), f69({
                let a211 = Str::lit("value:none");
                utf8_encode(a211)
            }, v6)))
        }
        Adt3::C16(r232) => {
            let v25 = (*r232).clone();
            let v26: Adt0 = f13(v6.clone());
            let v27: Adt11 = f19(v0, v1, v2, v3, 0u64, v25, v26)?;
            if f75(v27.clone()) {
                Ok(f71(0u64))
            } else {
                Ok(Adt11::C5(Adt21::C1, std::rc::Rc::new(Adt11::C13(std::rc::Rc::new(v27), std::rc::Rc::new(Adt11::C12))), f69({
                    let a212 = Str::lit("value:some");
                    utf8_encode(a212)
                }, v6)))
            }
        }
        Adt3::C17(r233) => {
            let v28 = (*r233).clone();
            let v29: Adt11 = f19(v0.clone(), v1, v2, v3, 0u64, v28, f13(v6.clone()))?;
            if f75(v29.clone()) {
                Ok(f71(0u64))
            } else {
                Ok(Adt11::C5(Adt21::C2(f24(v0.clone(), f13(v6.clone()))?, f24(v0, f14(v6.clone()))?), std::rc::Rc::new(Adt11::C13(std::rc::Rc::new(v29), std::rc::Rc::new(Adt11::C12))), f69({
                    let a213 = Str::lit("value:ok");
                    utf8_encode(a213)
                }, v6)))
            }
        }
        Adt3::C18(r234) => {
            let v30 = (*r234).clone();
            let v31: Adt11 = f19(v0.clone(), v1, v2, v3, 0u64, v30, f14(v6.clone()))?;
            if f75(v31.clone()) {
                Ok(f71(0u64))
            } else {
                Ok(Adt11::C5(Adt21::C3(f24(v0.clone(), f13(v6.clone()))?, f24(v0, f14(v6.clone()))?), std::rc::Rc::new(Adt11::C13(std::rc::Rc::new(v31), std::rc::Rc::new(Adt11::C12))), f69({
                    let a214 = Str::lit("value:error");
                    utf8_encode(a214)
                }, v6)))
            }
        }
        Adt3::C19(r235) => {
            let v32 = (*r235).clone();
            f19(v0, v1, v2, v3, 1u64, v32, f13(v6))
        }
        Adt3::C20(r236, r237) => {
            let v33 = (*r236).clone();
            let v34 = (*r237).clone();
            let v35: Adt11 = f19(v0.clone(), v1.clone(), v2.clone(), v3.clone(), 0u64, v33, f13(v6.clone()))?;
            if f75(v35.clone()) {
                Ok(f71(0u64))
            } else {
                let v36: Adt11 = f19(v0, v1, v2, v3, 0u64, v34, f14(v6.clone()))?;
                if f75(v36.clone()) {
                    Ok(f71(0u64))
                } else {
                    Ok(Adt11::C6(std::rc::Rc::new(v35), std::rc::Rc::new(v36), f69({
                        let a215 = Str::lit("value:pair");
                        utf8_encode(a215)
                    }, v6)))
                }
            }
        }
        Adt3::C21(v37, r238) => {
            let v38 = (*r238).clone();
            let v39: u64 = f76(v6.clone());
            let m216 = f55(v2.clone(), v39, v37);
            match m216 {
                None => {
                    Ok(f71(0u64))
                }
                Some(v40) => {
                    let v41: Adt11 = f19(v0, v1, v2, Adt0::C12(v39), 2u64, v38, v40)?;
                    if f75(v41.clone()) {
                        Ok(f71(0u64))
                    } else {
                        Ok(Adt11::C5(Adt21::C4(v39, v37), std::rc::Rc::new(v41), f69({
                            let a217 = Str::lit("value:adt");
                            utf8_encode(a217)
                        }, v6)))
                    }
                }
            }
        }
        Adt3::C22(_, _) => {
            Ok(f71(0u64))
        }
        Adt3::C23 => {
            let a218 = v4;
            let a219 = 1u64;
            if nat_eq(a218, a219) {
                Ok(Adt11::C5(Adt21::C7(f24(v0, v6)?), std::rc::Rc::new(Adt11::C12), Adt15::C0({
                    let a220 = Str::lit("value:list");
                    utf8_encode(a220)
                }, None::<Adt13>)))
            } else {
                let a221 = v4;
                let a222 = 2u64;
                if nat_eq(a221, a222) {
                    Ok(Adt11::C12)
                } else {
                    Ok(f71(0u64))
                }
            }
        }
        Adt3::C24(r239, r240) => {
            let v44 = (*r239).clone();
            let v45 = (*r240).clone();
            let a223 = v4;
            let a224 = 1u64;
            if nat_eq(a223, a224) {
                let v46: Adt11 = f19(v0.clone(), v1.clone(), v2.clone(), v3.clone(), 0u64, v44, v6.clone())?;
                if f75(v46.clone()) {
                    Ok(f71(0u64))
                } else {
                    let v47: Adt11 = f19(v0, v1, v2, v3, 1u64, v45, v6)?;
                    if f75(v47.clone()) {
                        Ok(f71(0u64))
                    } else {
                        Ok(Adt11::C5(Adt21::C6, std::rc::Rc::new(Adt11::C13(std::rc::Rc::new(v46), std::rc::Rc::new(Adt11::C13(std::rc::Rc::new(v47), std::rc::Rc::new(Adt11::C12))))), Adt15::C0({
                            let a225 = Str::lit("value:list");
                            utf8_encode(a225)
                        }, None::<Adt13>)))
                    }
                }
            } else {
                let a226 = v4;
                let a227 = 2u64;
                if nat_eq(a226, a227) {
                    let m228 = v6;
                    match m228 {
                        Adt0::C0 => {
                            Ok(Adt11::C12)
                        }
                        Adt0::C1 => {
                            Ok(Adt11::C12)
                        }
                        Adt0::C2 => {
                            Ok(Adt11::C12)
                        }
                        Adt0::C3 => {
                            Ok(Adt11::C12)
                        }
                        Adt0::C4(_) => {
                            Ok(Adt11::C12)
                        }
                        Adt0::C5 => {
                            Ok(Adt11::C12)
                        }
                        Adt0::C6 => {
                            Ok(Adt11::C12)
                        }
                        Adt0::C7 => {
                            Ok(Adt11::C12)
                        }
                        Adt0::C8(_) => {
                            Ok(Adt11::C12)
                        }
                        Adt0::C9(_, _) => {
                            Ok(Adt11::C12)
                        }
                        Adt0::C10(_) => {
                            Ok(Adt11::C12)
                        }
                        Adt0::C11(_, _) => {
                            Ok(Adt11::C12)
                        }
                        Adt0::C12(_) => {
                            Ok(Adt11::C12)
                        }
                        Adt0::C13(_, _) => {
                            Ok(Adt11::C12)
                        }
                        Adt0::C14 => {
                            Ok(Adt11::C12)
                        }
                        Adt0::C15(r230, r231) => {
                            let v58 = (*r230).clone();
                            let v59 = (*r231).clone();
                            let v60: Adt11 = f19(v0.clone(), v1.clone(), v2.clone(), v3.clone(), 0u64, v44, v58.clone())?;
                            if f75(v60.clone()) {
                                Ok(f71(0u64))
                            } else {
                                let v61: Adt11 = f19(v0.clone(), v1.clone(), v2, v3.clone(), 2u64, v45, v59)?;
                                if f75(v61.clone()) {
                                    Ok(f71(0u64))
                                } else {
                                    Ok(Adt11::C13(std::rc::Rc::new(if f57(v0, v1, v3, v58) {
                                        Adt11::C2(Adt18::C1, std::rc::Rc::new(v60), Adt15::C0({
                                            let a229 = Str::lit("indirection");
                                            utf8_encode(a229)
                                        }, None::<Adt13>))
                                    } else {
                                        v60
                                    }), std::rc::Rc::new(v61)))
                                }
                            }
                        }
                    }
                } else {
                    Ok(f71(0u64))
                }
            }
        }
    }
}

pub fn f20(p247: List<(u64, Adt0)>, v1: u64) -> Adt0 {
    let m246 = p247;
    {
        let mut m246 = m246;
        loop {
            let step = {
                let v0 = m246;
                let m241 = v0;
                match m241.uncons() {
                    None => {
                        Err::<List<(u64, Adt0)>, Adt0>(Adt0::C14)
                    }
                    Some((v2, v3)) => {
                        let a243 = {
                            let (h242, _) = v2;
                            h242
                        };
                        let a244 = v1;
                        if nat_eq(a243, a244) {
                            let (_, h245) = v2;
                            Err::<List<(u64, Adt0)>, Adt0>(h245)
                        } else {
                            Ok::<List<(u64, Adt0)>, Adt0>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m246 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f21(v0: Adt0, v1: Adt0) -> bool {
    let m248 = v0;
    match m248 {
        Adt0::C0 => {
            let a249 = f67(v1);
            let a250 = 0u64;
            nat_eq(a249, a250)
        }
        Adt0::C1 => {
            let a251 = f67(v1);
            let a252 = 1u64;
            nat_eq(a251, a252)
        }
        Adt0::C2 => {
            let a253 = f67(v1);
            let a254 = 2u64;
            nat_eq(a253, a254)
        }
        Adt0::C3 => {
            let a255 = f67(v1);
            let a256 = 3u64;
            nat_eq(a255, a256)
        }
        Adt0::C4(v2) => {
            let a265 = f67(v1.clone());
            let a266 = 4u64;
            if nat_eq(a265, a266) {
                let a267 = f77(v2);
                let a268 = f77(f78(v1));
                nat_eq(a267, a268)
            } else {
                false
            }
        }
        Adt0::C5 => {
            let a257 = f67(v1);
            let a258 = 5u64;
            nat_eq(a257, a258)
        }
        Adt0::C6 => {
            let a259 = f67(v1);
            let a260 = 6u64;
            nat_eq(a259, a260)
        }
        Adt0::C7 => {
            let a261 = f67(v1);
            let a262 = 7u64;
            nat_eq(a261, a262)
        }
        Adt0::C8(r285) => {
            let v3 = (*r285).clone();
            let v4: bool = f21(v3, f13(v1.clone()));
            let a269 = f67(v1);
            let a270 = 8u64;
            if nat_eq(a269, a270) {
                v4
            } else {
                false
            }
        }
        Adt0::C9(r286, r287) => {
            let v5 = (*r286).clone();
            let v6 = (*r287).clone();
            let v7: bool = f21(v5, f13(v1.clone()));
            let v8: bool = f21(v6, f14(v1.clone()));
            let a271 = f67(v1);
            let a272 = 9u64;
            if nat_eq(a271, a272) {
                if v7 {
                    v8
                } else {
                    false
                }
            } else {
                false
            }
        }
        Adt0::C10(r288) => {
            let v9 = (*r288).clone();
            let v10: bool = f21(v9, f13(v1.clone()));
            let a273 = f67(v1);
            let a274 = 10u64;
            if nat_eq(a273, a274) {
                v10
            } else {
                false
            }
        }
        Adt0::C11(r289, r290) => {
            let v11 = (*r289).clone();
            let v12 = (*r290).clone();
            let v13: bool = f21(v11, f13(v1.clone()));
            let v14: bool = f21(v12, f14(v1.clone()));
            let a275 = f67(v1);
            let a276 = 11u64;
            if nat_eq(a275, a276) {
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
            let a277 = f67(v1.clone());
            let a278 = 12u64;
            if nat_eq(a277, a278) {
                let a279 = v15;
                let a280 = f76(v1);
                nat_eq(a279, a280)
            } else {
                false
            }
        }
        Adt0::C13(r291, r292) => {
            let v16 = (*r291).clone();
            let v17 = (*r292).clone();
            let v18: bool = f21(v16, f13(v1.clone()));
            let v19: bool = f21(v17, f14(v1.clone()));
            let a281 = f67(v1);
            let a282 = 13u64;
            if nat_eq(a281, a282) {
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
            let a263 = f67(v1);
            let a264 = 14u64;
            nat_eq(a263, a264)
        }
        Adt0::C15(r293, r294) => {
            let v20 = (*r293).clone();
            let v21 = (*r294).clone();
            let v22: bool = f21(v20, f13(v1.clone()));
            let v23: bool = f21(v21, f14(v1.clone()));
            let a283 = f67(v1);
            let a284 = 15u64;
            if nat_eq(a283, a284) {
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

pub fn f22(v0: Adt0) -> bool {
    let m295 = v0;
    match m295 {
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
            false
        }
        Adt0::C6 => {
            false
        }
        Adt0::C7 => {
            true
        }
        Adt0::C8(r296) => {
            let v2 = (*r296).clone();
            f22(v2)
        }
        Adt0::C9(r297, r298) => {
            let v3 = (*r297).clone();
            let v4 = (*r298).clone();
            if f22(v3) {
                f22(v4)
            } else {
                false
            }
        }
        Adt0::C10(_) => {
            false
        }
        Adt0::C11(r299, r300) => {
            let v6 = (*r299).clone();
            let v7 = (*r300).clone();
            if f22(v6) {
                f22(v7)
            } else {
                false
            }
        }
        Adt0::C12(_) => {
            false
        }
        Adt0::C13(_, _) => {
            false
        }
        Adt0::C14 => {
            false
        }
        Adt0::C15(_, _) => {
            false
        }
    }
}

pub fn f23(v0: u64, v1: Adt0, v2: Adt2) -> Adt28 {
    if if f21(v1, Adt0::C0) {
        true
    } else {
        let a301 = f79(v2, v0);
        bool_not(a301)
    } {
        Adt28::C0(Adt29::C0)
    } else {
        Adt28::C1(Adt17::C0(v0))
    }
}

pub fn f24(v0: Adt0, v1: Adt0) -> R<Adt22> {
    let m302 = v1;
    match m302 {
        Adt0::C0 => {
            Ok(Adt22::C0(Adt23::C0))
        }
        Adt0::C1 => {
            Ok(Adt22::C0(Adt23::C1))
        }
        Adt0::C2 => {
            Ok(Adt22::C0(Adt23::C2))
        }
        Adt0::C3 => {
            Ok(Adt22::C0(Adt23::C3))
        }
        Adt0::C4(v2) => {
            Ok(Adt22::C1(f80(v2)))
        }
        Adt0::C5 => {
            Ok(Adt22::C0(Adt23::C5))
        }
        Adt0::C6 => {
            Ok(Adt22::C0(Adt23::C6))
        }
        Adt0::C7 => {
            Ok(Adt22::C0(Adt23::C4))
        }
        Adt0::C8(r303) => {
            let v3 = (*r303).clone();
            Ok(Adt22::C2(Adt24::C0, std::rc::Rc::new(f24(v0, v3)?)))
        }
        Adt0::C9(r304, r305) => {
            let v4 = (*r304).clone();
            let v5 = (*r305).clone();
            Ok(Adt22::C3(Adt25::C0, std::rc::Rc::new(f24(v0.clone(), v4)?), std::rc::Rc::new(f24(v0, v5)?)))
        }
        Adt0::C10(r306) => {
            let v6 = (*r306).clone();
            Ok(Adt22::C2(Adt24::C1, std::rc::Rc::new(f24(v0, v6)?)))
        }
        Adt0::C11(r307, r308) => {
            let v7 = (*r307).clone();
            let v8 = (*r308).clone();
            Ok(Adt22::C3(Adt25::C1, std::rc::Rc::new(f24(v0.clone(), v7)?), std::rc::Rc::new(f24(v0, v8)?)))
        }
        Adt0::C12(v9) => {
            Ok(Adt22::C4(Adt26::C0, v9))
        }
        Adt0::C13(r309, r310) => {
            let v10 = (*r309).clone();
            let v11 = (*r310).clone();
            Ok(Adt22::C4(Adt26::C1, f44(v0, Adt0::C13(std::rc::Rc::new(v10), std::rc::Rc::new(v11)), 0u64)?))
        }
        Adt0::C14 => {
            Ok(Adt22::C5)
        }
        Adt0::C15(r311, r312) => {
            let v12 = (*r311).clone();
            let v13 = (*r312).clone();
            Ok(Adt22::C6(std::rc::Rc::new(f24(v0.clone(), v12)?), std::rc::Rc::new(f24(v0, v13)?)))
        }
    }
}

pub fn f25(v0: Adt11) -> Adt11 {
    let m313 = v0;
    match m313 {
        Adt11::C0(_, _) => {
            Adt11::C12
        }
        Adt11::C1(_, _, _) => {
            Adt11::C12
        }
        Adt11::C2(_, _, _) => {
            Adt11::C12
        }
        Adt11::C3(_, _, _, _) => {
            Adt11::C12
        }
        Adt11::C4(_, _, _, _) => {
            Adt11::C12
        }
        Adt11::C5(_, _, _) => {
            Adt11::C12
        }
        Adt11::C6(_, _, _) => {
            Adt11::C12
        }
        Adt11::C7(_, _, _, _) => {
            Adt11::C12
        }
        Adt11::C8(_, _, _) => {
            Adt11::C12
        }
        Adt11::C9(_) => {
            Adt11::C12
        }
        Adt11::C10(_, _, _) => {
            Adt11::C12
        }
        Adt11::C11(r314, _) => {
            let v34 = (*r314).clone();
            f81(v34)
        }
        Adt11::C12 => {
            Adt11::C12
        }
        Adt11::C13(_, _) => {
            Adt11::C12
        }
        Adt11::C14(_, _, _, _, _, _) => {
            Adt11::C12
        }
        Adt11::C15(_, _, _) => {
            Adt11::C12
        }
        Adt11::C16(_, _) => {
            Adt11::C12
        }
    }
}

pub fn f26(v0: Adt11) -> Adt11 {
    let m315 = v0;
    match m315 {
        Adt11::C0(_, _) => {
            Adt11::C12
        }
        Adt11::C1(_, _, _) => {
            Adt11::C12
        }
        Adt11::C2(_, _, _) => {
            Adt11::C12
        }
        Adt11::C3(_, _, _, _) => {
            Adt11::C12
        }
        Adt11::C4(_, _, _, _) => {
            Adt11::C12
        }
        Adt11::C5(_, _, _) => {
            Adt11::C12
        }
        Adt11::C6(_, _, _) => {
            Adt11::C12
        }
        Adt11::C7(_, _, _, _) => {
            Adt11::C12
        }
        Adt11::C8(_, _, _) => {
            Adt11::C12
        }
        Adt11::C9(_) => {
            Adt11::C12
        }
        Adt11::C10(_, _, _) => {
            Adt11::C12
        }
        Adt11::C11(_, r316) => {
            let v35 = (*r316).clone();
            f81(v35)
        }
        Adt11::C12 => {
            Adt11::C12
        }
        Adt11::C13(_, _) => {
            Adt11::C12
        }
        Adt11::C14(_, _, _, _, _, _) => {
            Adt11::C12
        }
        Adt11::C15(_, _, _) => {
            Adt11::C12
        }
        Adt11::C16(_, _) => {
            Adt11::C12
        }
    }
}

pub fn f27(v0: Adt0) -> Adt0 {
    f13(f14(f14(f14(v0))))
}

pub fn f28(v0: Adt11, v1: Adt11, v2: Adt11, v3: Adt15, v4: u64) -> R<Adt10> {
    let v5: Adt10 = if if f82(v2.clone()) {
        if f30(f25(v1.clone())) {
            f83(f26(v1.clone()))
        } else {
            false
        }
    } else {
        false
    } {
        f84(v1, v4)?
    } else {
        f2(v1, v4)
    };
    let v6: Adt11 = match v5.clone() {
        Adt10::C0(h317, _) => {
            h317
        }
    };
    let v7: u64 = match v5 {
        Adt10::C0(_, h318) => {
            h318
        }
    };
    let a319 = f85(v6.clone());
    let a320 = 1u64;
    if if nat_eq(a319, a320) {
        let a321 = f85(v2.clone());
        let a322 = 2u64;
        nat_eq(a321, a322)
    } else {
        false
    } {
        Ok(f2(v0, v7))
    } else {
        let a323 = f85(v6.clone());
        let a324 = 2u64;
        if if nat_eq(a323, a324) {
            let a325 = f85(v2.clone());
            let a326 = 1u64;
            nat_eq(a325, a326)
        } else {
            false
        } {
            Ok(f2(f86(v0, v3), v7))
        } else {
            if f87(v6.clone(), v2.clone())? {
                Ok(f2(Adt11::C9(std::rc::Rc::new(Adt11::C11(std::rc::Rc::new(Adt11::C14(Adt28::C0(Adt29::C0), false, f32(0u64), std::rc::Rc::new(v0), v3, std::rc::Rc::new(f25(v6.clone())))), std::rc::Rc::new(f26(v6))))), v7))
            } else {
                let a327 = 0u64;
                let a328 = f88(v0.clone());
                if nat_lt(a327, a328) {
                    Ok(f2(Adt11::C9(std::rc::Rc::new(Adt11::C11(std::rc::Rc::new(Adt11::C14(Adt28::C1(Adt17::C1({
                        let a329 = v7;
                        let a330 = 1u64;
                        nat_add(a329, a330)?
                    })), false, f32(0u64), std::rc::Rc::new(v0), v3.clone(), std::rc::Rc::new(Adt11::C12))), std::rc::Rc::new(Adt11::C7(std::rc::Rc::new(Adt11::C1(Adt16::C0, Adt17::C1({
                        let a331 = v7;
                        let a332 = 1u64;
                        nat_add(a331, a332)?
                    }), v3.clone())), std::rc::Rc::new(v6), std::rc::Rc::new(v2), v3))))), {
                        let a333 = v7;
                        let a334 = 1u64;
                        nat_add(a333, a334)?
                    }))
                } else {
                    Ok(f2(Adt11::C7(std::rc::Rc::new(v0), std::rc::Rc::new(v6), std::rc::Rc::new(v2), v3), v7))
                }
            }
        }
    }
}

pub fn f29(v0: Adt11, v1: Adt11) -> Adt11 {
    f89(v0, v1.clone(), v1)
}

pub fn f30(v0: Adt11) -> bool {
    let m335 = v0;
    match m335 {
        Adt11::C0(_, _) => {
            false
        }
        Adt11::C1(_, _, _) => {
            false
        }
        Adt11::C2(_, _, _) => {
            false
        }
        Adt11::C3(_, _, _, _) => {
            false
        }
        Adt11::C4(_, _, _, _) => {
            false
        }
        Adt11::C5(_, _, _) => {
            false
        }
        Adt11::C6(_, _, _) => {
            false
        }
        Adt11::C7(_, _, _, _) => {
            false
        }
        Adt11::C8(_, _, _) => {
            false
        }
        Adt11::C9(_) => {
            false
        }
        Adt11::C10(_, _, _) => {
            false
        }
        Adt11::C11(_, _) => {
            false
        }
        Adt11::C12 => {
            true
        }
        Adt11::C13(_, _) => {
            false
        }
        Adt11::C14(_, _, _, _, _, _) => {
            false
        }
        Adt11::C15(_, _, _) => {
            false
        }
        Adt11::C16(_, _) => {
            false
        }
    }
}

pub fn f31(v0: Adt0) -> Adt0 {
    f13(v0)
}

pub fn f32(_: u64) -> Adt22 {
    Adt22::C0(Adt23::C0)
}

pub fn f33(v0: Adt5) -> Adt15 {
    Adt15::C0({
        let a337 = {
            let a336 = Str::lit("shape:");
            utf8_encode(a336)
        };
        let a338 = f90(v0);
        append_bytes(a337, a338)
    }, None::<Adt13>)
}

pub fn f34(v0: Adt32, v1: Adt17, v2: Adt0, v3: Adt2, v4: Adt11, v5: u64) -> R<Adt10> {
    let m339 = v2;
    match m339 {
        Adt0::C0 => {
            Ok(f2(Adt11::C12, v5))
        }
        Adt0::C1 => {
            f91(v1, v3, v4, v5)
        }
        Adt0::C2 => {
            f92(v1, v3, v4, v5)
        }
        Adt0::C3 => {
            Ok(f2(Adt11::C12, v5))
        }
        Adt0::C4(_) => {
            Ok(f2(Adt11::C12, v5))
        }
        Adt0::C5 => {
            Ok(f2(Adt11::C12, v5))
        }
        Adt0::C6 => {
            Ok(f2(Adt11::C12, v5))
        }
        Adt0::C7 => {
            f93(v1, v3, v4, v5)
        }
        Adt0::C8(_) => {
            f94(v1, v3, v4, v5)
        }
        Adt0::C9(_, _) => {
            f95(v1, v3, v4, v5)
        }
        Adt0::C10(_) => {
            f96(v1, v3, v4, v5)
        }
        Adt0::C11(_, _) => {
            f97(v1, v3, v4, v5)
        }
        Adt0::C12(v13) => {
            f98(match v0.clone() {
                Adt32::C0(_, h340, _, _, _, _, _, _, _) => {
                    h340
                }
            }, match v0.clone() {
                Adt32::C0(_, _, h341, _, _, _, _, _, _) => {
                    h341
                }
            }, match v0 {
                Adt32::C0(_, _, _, h342, _, _, _, _, _) => {
                    h342
                }
            }, v1, v13, v3, v4, v5)
        }
        Adt0::C13(_, _) => {
            Ok(f2(Adt11::C12, v5))
        }
        Adt0::C14 => {
            Ok(f2(Adt11::C12, v5))
        }
        Adt0::C15(_, _) => {
            Ok(f2(Adt11::C12, v5))
        }
    }
}

pub fn f35(v0: Adt11, v1: Adt17) -> bool {
    let m343 = v0;
    match m343 {
        Adt11::C0(_, _) => {
            false
        }
        Adt11::C1(v4, v5, _) => {
            let m344 = v4;
            match m344 {
                Adt16::C0 => {
                    f99(v5, v1)
                }
                Adt16::C1 => {
                    false
                }
                Adt16::C2 => {
                    false
                }
                Adt16::C3 => {
                    false
                }
                Adt16::C4 => {
                    false
                }
            }
        }
        Adt11::C2(_, _, _) => {
            false
        }
        Adt11::C3(_, _, _, _) => {
            false
        }
        Adt11::C4(_, _, _, _) => {
            false
        }
        Adt11::C5(_, _, _) => {
            false
        }
        Adt11::C6(_, _, _) => {
            false
        }
        Adt11::C7(_, _, _, _) => {
            false
        }
        Adt11::C8(_, _, _) => {
            false
        }
        Adt11::C9(_) => {
            false
        }
        Adt11::C10(_, _, _) => {
            false
        }
        Adt11::C11(_, _) => {
            false
        }
        Adt11::C12 => {
            false
        }
        Adt11::C13(_, _) => {
            false
        }
        Adt11::C14(_, _, _, _, _, _) => {
            false
        }
        Adt11::C15(_, _, _) => {
            false
        }
        Adt11::C16(_, _) => {
            false
        }
    }
}

pub fn f36(v0: Adt0, v1: Adt8, v2: Adt0, v3: Adt5, v4: Adt0, v5: Adt11, v6: Adt15, v7: u64) -> R<Adt10> {
    let m345 = v3.clone();
    match m345 {
        Adt5::C0 => {
            let m346 = v4;
            match m346 {
                Adt0::C0 => {
                    Ok(f2(Adt11::C12, v7))
                }
                Adt0::C1 => {
                    Ok(f2(Adt11::C12, v7))
                }
                Adt0::C2 => {
                    Ok(f2(Adt11::C12, v7))
                }
                Adt0::C3 => {
                    Ok(f2(Adt11::C12, v7))
                }
                Adt0::C4(_) => {
                    Ok(f2(Adt11::C12, v7))
                }
                Adt0::C5 => {
                    Ok(f2(Adt11::C12, v7))
                }
                Adt0::C6 => {
                    Ok(f2(Adt11::C12, v7))
                }
                Adt0::C7 => {
                    Ok(f2(Adt11::C12, v7))
                }
                Adt0::C8(r347) => {
                    let v9 = (*r347).clone();
                    Ok(f2(Adt11::C5(Adt21::C0(f24(v0, v9)?), std::rc::Rc::new(Adt11::C12), v6), v7))
                }
                Adt0::C9(_, _) => {
                    Ok(f2(Adt11::C12, v7))
                }
                Adt0::C10(_) => {
                    Ok(f2(Adt11::C12, v7))
                }
                Adt0::C11(_, _) => {
                    Ok(f2(Adt11::C12, v7))
                }
                Adt0::C12(_) => {
                    Ok(f2(Adt11::C12, v7))
                }
                Adt0::C13(_, _) => {
                    Ok(f2(Adt11::C12, v7))
                }
                Adt0::C14 => {
                    Ok(f2(Adt11::C12, v7))
                }
                Adt0::C15(_, _) => {
                    Ok(f2(Adt11::C12, v7))
                }
            }
        }
        Adt5::C1 => {
            f100(v0, v1, v2, v3, v4, v5, v6, v7)
        }
        Adt5::C2 => {
            f100(v0, v1, v2, v3, v4, v5, v6, v7)
        }
        Adt5::C3 => {
            f100(v0, v1, v2, v3, v4, v5, v6, v7)
        }
        Adt5::C4 => {
            let m348 = v4;
            match m348 {
                Adt0::C0 => {
                    Ok(f2(Adt11::C12, v7))
                }
                Adt0::C1 => {
                    Ok(f2(Adt11::C12, v7))
                }
                Adt0::C2 => {
                    Ok(f2(Adt11::C12, v7))
                }
                Adt0::C3 => {
                    Ok(f2(Adt11::C12, v7))
                }
                Adt0::C4(_) => {
                    Ok(f2(Adt11::C12, v7))
                }
                Adt0::C5 => {
                    Ok(f2(Adt11::C12, v7))
                }
                Adt0::C6 => {
                    Ok(f2(Adt11::C12, v7))
                }
                Adt0::C7 => {
                    Ok(f2(Adt11::C12, v7))
                }
                Adt0::C8(_) => {
                    Ok(f2(Adt11::C12, v7))
                }
                Adt0::C9(_, _) => {
                    Ok(f2(Adt11::C12, v7))
                }
                Adt0::C10(r349) => {
                    let v24 = (*r349).clone();
                    Ok(f2(Adt11::C5(Adt21::C7(f24(v0, v24)?), std::rc::Rc::new(Adt11::C12), v6), v7))
                }
                Adt0::C11(_, _) => {
                    Ok(f2(Adt11::C12, v7))
                }
                Adt0::C12(_) => {
                    Ok(f2(Adt11::C12, v7))
                }
                Adt0::C13(_, _) => {
                    Ok(f2(Adt11::C12, v7))
                }
                Adt0::C14 => {
                    Ok(f2(Adt11::C12, v7))
                }
                Adt0::C15(_, _) => {
                    Ok(f2(Adt11::C12, v7))
                }
            }
        }
        Adt5::C5 => {
            f100(v0, v1, v2, v3, v4, v5, v6, v7)
        }
        Adt5::C6 => {
            Ok(f2(Adt11::C0(f101(v3), v6), v7))
        }
        Adt5::C7 => {
            Ok(f2(Adt11::C3(Adt19::C1(Adt20::C0(8u64, None::<Adt13>)), std::rc::Rc::new(v5), true, v6), v7))
        }
        Adt5::C8 => {
            Ok(f2(Adt11::C6(std::rc::Rc::new(f102(v5.clone(), 0u64)), std::rc::Rc::new(f102(v5, 1u64)), v6), v7))
        }
        Adt5::C9 => {
            Ok(f2(Adt11::C0(f101(v3), v6), v7))
        }
        Adt5::C10 => {
            Ok(f2(Adt11::C0(f101(v3), v6), v7))
        }
        Adt5::C11 => {
            Ok(f2(Adt11::C0(f101(v3), v6), v7))
        }
        Adt5::C12 => {
            Ok(f2(Adt11::C0(f101(v3), v6), v7))
        }
        Adt5::C13 => {
            Ok(f2(Adt11::C0(f101(v3), v6), v7))
        }
        Adt5::C14 => {
            Ok(f2(Adt11::C0(f101(v3), v6), v7))
        }
        Adt5::C15(_) => {
            f100(v0, v1, v2, v3, v4, v5, v6, v7)
        }
    }
}

pub fn f37(v0: Adt11, v1: Adt0, v2: Adt15, v3: u64) -> R<Adt10> {
    if f103(v0.clone(), v1.clone()) {
        f104(v0, v1, v2, v3)
    } else {
        Ok(f2(Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(v0)), v3))
    }
}

pub fn f38(v0: Adt32, v1: u64) -> Adt0 {
    let m351 = f56(match v0 {
        Adt32::C0(_, _, _, _, h350, _, _, _, _) => {
            h350
        }
    }, v1);
    match m351 {
        None => {
            Adt0::C14
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f39(v0: Adt11, v1: Adt11) -> Adt11 {
    if f30(v0.clone()) {
        v1
    } else {
        let v2: Adt11 = f11(v1);
        Adt11::C9(std::rc::Rc::new(Adt11::C11(std::rc::Rc::new(f105(v0, f25(v2.clone()))), std::rc::Rc::new(f26(v2)))))
    }
}

pub fn f40(v0: Adt2) -> R<u64> {
    let m352 = v0;
    match m352 {
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
        Adt2::C14(_, r355) => {
            let v33 = (*r355).clone();
            let v34: u64 = f40(v33)?;
            let a353 = 1u64;
            let a354 = v34;
            nat_add(a353, a354)
        }
        Adt2::C15(_, _, _) => {
            Ok(0u64)
        }
    }
}

pub fn f41(v0: Adt0, v1: u64) -> Adt0 {
    let m356 = v0;
    match m356 {
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
        Adt0::C15(r358, r359) => {
            let v12 = (*r358).clone();
            let v13 = (*r359).clone();
            let m357 = v1;
            if m357 == 0 {
                Adt0::C14
            } else {
                let v14 = m357 - 1;
                Adt0::C15(std::rc::Rc::new(v12), std::rc::Rc::new(f41(v13, v14)))
            }
        }
    }
}

pub fn f42(p365: Adt0, p366: u64) -> Adt0 {
    let m364 = (p365, p366);
    {
        let mut m364 = m364;
        loop {
            let step = {
                let (v0, v1) = m364;
                let m360 = v0;
                match m360 {
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
                    Adt0::C15(r362, r363) => {
                        let v12 = (*r362).clone();
                        let v13 = (*r363).clone();
                        let m361 = v1;
                        if m361 == 0 {
                            Err::<(Adt0, u64), Adt0>(Adt0::C15(std::rc::Rc::new(v12), std::rc::Rc::new(v13)))
                        } else {
                            let v14 = m361 - 1;
                            Ok::<(Adt0, u64), Adt0>((v13, v14))
                        }
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

pub fn f43(v0: Adt32, v1: u64) -> Adt0 {
    let m368 = f56(match v0 {
        Adt32::C0(_, _, _, _, _, h367, _, _, _) => {
            h367
        }
    }, v1);
    match m368 {
        None => {
            Adt0::C14
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f44(p375: Adt0, v1: Adt0, p376: u64) -> R<u64> {
    let m374 = (p375, p376);
    {
        let mut m374 = m374;
        loop {
            let step = {
                let (v0, v2) = m374;
                let m369 = v0;
                match m369 {
                    Adt0::C0 => {
                        Err::<(Adt0, u64), R<u64>>(Ok(v2))
                    }
                    Adt0::C1 => {
                        Err::<(Adt0, u64), R<u64>>(Ok(v2))
                    }
                    Adt0::C2 => {
                        Err::<(Adt0, u64), R<u64>>(Ok(v2))
                    }
                    Adt0::C3 => {
                        Err::<(Adt0, u64), R<u64>>(Ok(v2))
                    }
                    Adt0::C4(_) => {
                        Err::<(Adt0, u64), R<u64>>(Ok(v2))
                    }
                    Adt0::C5 => {
                        Err::<(Adt0, u64), R<u64>>(Ok(v2))
                    }
                    Adt0::C6 => {
                        Err::<(Adt0, u64), R<u64>>(Ok(v2))
                    }
                    Adt0::C7 => {
                        Err::<(Adt0, u64), R<u64>>(Ok(v2))
                    }
                    Adt0::C8(_) => {
                        Err::<(Adt0, u64), R<u64>>(Ok(v2))
                    }
                    Adt0::C9(_, _) => {
                        Err::<(Adt0, u64), R<u64>>(Ok(v2))
                    }
                    Adt0::C10(_) => {
                        Err::<(Adt0, u64), R<u64>>(Ok(v2))
                    }
                    Adt0::C11(_, _) => {
                        Err::<(Adt0, u64), R<u64>>(Ok(v2))
                    }
                    Adt0::C12(_) => {
                        Err::<(Adt0, u64), R<u64>>(Ok(v2))
                    }
                    Adt0::C13(_, _) => {
                        Err::<(Adt0, u64), R<u64>>(Ok(v2))
                    }
                    Adt0::C14 => {
                        Err::<(Adt0, u64), R<u64>>(Ok(v2))
                    }
                    Adt0::C15(r372, r373) => {
                        let v13 = (*r372).clone();
                        let v14 = (*r373).clone();
                        if f21(v13, v1.clone()) {
                            Err::<(Adt0, u64), R<u64>>(Ok(v2))
                        } else {
                            Ok::<(Adt0, u64), R<u64>>((v14, {
                                let a370 = v2;
                                let a371 = 1u64;
                                nat_add(a370, a371)?
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m374 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f45(v0: Adt0, v1: Adt8, v2: Adt0, v3: Adt11, v4: Adt0) -> Adt11 {
    let m377 = v3;
    match m377 {
        Adt11::C0(_, _) => {
            Adt11::C12
        }
        Adt11::C1(_, _, _) => {
            Adt11::C12
        }
        Adt11::C2(_, _, _) => {
            Adt11::C12
        }
        Adt11::C3(_, _, _, _) => {
            Adt11::C12
        }
        Adt11::C4(_, _, _, _) => {
            Adt11::C12
        }
        Adt11::C5(_, _, _) => {
            Adt11::C12
        }
        Adt11::C6(_, _, _) => {
            Adt11::C12
        }
        Adt11::C7(_, _, _, _) => {
            Adt11::C12
        }
        Adt11::C8(_, _, _) => {
            Adt11::C12
        }
        Adt11::C9(_) => {
            Adt11::C12
        }
        Adt11::C10(_, _, _) => {
            Adt11::C12
        }
        Adt11::C11(_, _) => {
            Adt11::C12
        }
        Adt11::C12 => {
            Adt11::C12
        }
        Adt11::C13(r379, r380) => {
            let v40 = (*r379).clone();
            let v41 = (*r380).clone();
            if f106(v4.clone()) {
                Adt11::C13(std::rc::Rc::new(if f57(v0.clone(), v1.clone(), v2.clone(), f107(v4.clone())) {
                    Adt11::C2(Adt18::C1, std::rc::Rc::new(v40), Adt15::C0({
                        let a378 = Str::lit("indirection");
                        utf8_encode(a378)
                    }, None::<Adt13>))
                } else {
                    v40
                }), std::rc::Rc::new(f45(v0, v1, v2, v41, f108(v4))))
            } else {
                Adt11::C12
            }
        }
        Adt11::C14(_, _, _, _, _, _) => {
            Adt11::C12
        }
        Adt11::C15(_, _, _) => {
            Adt11::C12
        }
        Adt11::C16(_, _) => {
            Adt11::C12
        }
    }
}

pub fn f46(v0: Adt0) -> (Adt0, Adt0) {
    (f13(v0.clone()), f14(v0))
}

pub fn f47(v0: Adt0, v1: List<bool>, p384: Adt9) -> bool {
    let m383 = p384;
    {
        let mut m383 = m383;
        loop {
            let step = {
                let v2 = m383;
                let m381 = v2;
                match m381 {
                    Adt9::C0 => {
                        Err::<Adt9, bool>(false)
                    }
                    Adt9::C1(v3, v4, _, r382) => {
                        let v6 = (*r382).clone();
                        if f21(v3, v0.clone()) {
                            if f3(v1.clone(), v4) {
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
                Ok(next) => { m383 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f48(v0: Adt2, v1: Adt0) -> Adt0 {
    let m385 = v0;
    match m385 {
        Adt2::C0(_, _) => {
            Adt0::C0
        }
        Adt2::C1(_) => {
            Adt0::C0
        }
        Adt2::C2(_, _, _, _) => {
            Adt0::C0
        }
        Adt2::C3(_, _, _) => {
            Adt0::C0
        }
        Adt2::C4(_, _, _) => {
            Adt0::C0
        }
        Adt2::C5(_, _, _) => {
            Adt0::C0
        }
        Adt2::C6(_, _) => {
            Adt0::C0
        }
        Adt2::C7(_, _) => {
            Adt0::C0
        }
        Adt2::C8(_, _) => {
            Adt0::C0
        }
        Adt2::C9(_, _) => {
            Adt0::C0
        }
        Adt2::C10(_) => {
            Adt0::C0
        }
        Adt2::C11(_) => {
            Adt0::C0
        }
        Adt2::C12(_, _) => {
            Adt0::C0
        }
        Adt2::C13 => {
            Adt0::C0
        }
        Adt2::C14(_, _) => {
            f31(f9(v1))
        }
        Adt2::C15(_, _, _) => {
            Adt0::C0
        }
    }
}

pub fn f49(v0: Adt6, v1: Adt0) -> u64 {
    let m386 = v0;
    match m386 {
        Adt6::C0 => {
            0u64
        }
        Adt6::C1 => {
            1u64
        }
        Adt6::C2 => {
            2u64
        }
        Adt6::C3 => {
            3u64
        }
        Adt6::C4 => {
            4u64
        }
        Adt6::C5 => {
            5u64
        }
        Adt6::C6 => {
            6u64
        }
        Adt6::C7 => {
            7u64
        }
        Adt6::C8 => {
            9u64
        }
        Adt6::C9 => {
            10u64
        }
        Adt6::C10 => {
            11u64
        }
        Adt6::C11 => {
            12u64
        }
        Adt6::C12 => {
            13u64
        }
        Adt6::C13 => {
            14u64
        }
        Adt6::C14 => {
            20u64
        }
        Adt6::C15 => {
            21u64
        }
        Adt6::C16 => {
            22u64
        }
        Adt6::C17 => {
            24u64
        }
        Adt6::C18 => {
            23u64
        }
        Adt6::C19 => {
            25u64
        }
        Adt6::C20 => {
            26u64
        }
        Adt6::C21 => {
            27u64
        }
        Adt6::C22 => {
            28u64
        }
        Adt6::C23 => {
            29u64
        }
        Adt6::C24 => {
            30u64
        }
        Adt6::C25 => {
            18u64
        }
        Adt6::C26 => {
            15u64
        }
        Adt6::C27 => {
            16u64
        }
        Adt6::C28 => {
            17u64
        }
        Adt6::C29 => {
            f109(v1, 32u64, 33u64, 32u64)
        }
        Adt6::C30 => {
            f109(v1, 34u64, 35u64, 36u64)
        }
        Adt6::C31 => {
            f109(v1, 37u64, 38u64, 37u64)
        }
        Adt6::C32 => {
            f109(v1, 39u64, 40u64, 39u64)
        }
        Adt6::C33 => {
            41u64
        }
        Adt6::C34 => {
            42u64
        }
        Adt6::C35 => {
            43u64
        }
        Adt6::C36 => {
            44u64
        }
        Adt6::C37 => {
            45u64
        }
        Adt6::C38 => {
            let m387 = v1;
            match m387 {
                Adt0::C0 => {
                    47u64
                }
                Adt0::C1 => {
                    47u64
                }
                Adt0::C2 => {
                    47u64
                }
                Adt0::C3 => {
                    46u64
                }
                Adt0::C4(_) => {
                    47u64
                }
                Adt0::C5 => {
                    47u64
                }
                Adt0::C6 => {
                    47u64
                }
                Adt0::C7 => {
                    47u64
                }
                Adt0::C8(_) => {
                    47u64
                }
                Adt0::C9(_, _) => {
                    47u64
                }
                Adt0::C10(_) => {
                    47u64
                }
                Adt0::C11(_, _) => {
                    47u64
                }
                Adt0::C12(_) => {
                    47u64
                }
                Adt0::C13(_, _) => {
                    47u64
                }
                Adt0::C14 => {
                    47u64
                }
                Adt0::C15(_, _) => {
                    47u64
                }
            }
        }
        Adt6::C39 => {
            19u64
        }
        Adt6::C40(_) => {
            31u64
        }
        Adt6::C41(v15) => {
            let m388 = v15;
            match m388 {
                Adt0::C0 => {
                    49u64
                }
                Adt0::C1 => {
                    49u64
                }
                Adt0::C2 => {
                    49u64
                }
                Adt0::C3 => {
                    48u64
                }
                Adt0::C4(_) => {
                    49u64
                }
                Adt0::C5 => {
                    49u64
                }
                Adt0::C6 => {
                    49u64
                }
                Adt0::C7 => {
                    49u64
                }
                Adt0::C8(_) => {
                    49u64
                }
                Adt0::C9(_, _) => {
                    49u64
                }
                Adt0::C10(_) => {
                    49u64
                }
                Adt0::C11(_, _) => {
                    49u64
                }
                Adt0::C12(_) => {
                    49u64
                }
                Adt0::C13(_, _) => {
                    49u64
                }
                Adt0::C14 => {
                    49u64
                }
                Adt0::C15(_, _) => {
                    49u64
                }
            }
        }
    }
}

pub fn f50(v0: Adt6, v1: Adt0) -> Adt15 {
    Adt15::C0({
        let a390 = {
            let a389 = Str::lit("prim:");
            utf8_encode(a389)
        };
        let a391 = f110(v0.clone());
        append_bytes(a390, a391)
    }, f52(v0, v1))
}

pub fn f51(v0: Adt11, v1: bool, v2: Adt15, v3: Adt15, v4: u64) -> R<Adt10> {
    let m392 = v0;
    match m392 {
        Adt11::C0(_, _) => {
            Ok(f2(Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(Adt11::C12)), v4))
        }
        Adt11::C1(_, _, _) => {
            Ok(f2(Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(Adt11::C12)), v4))
        }
        Adt11::C2(_, _, _) => {
            Ok(f2(Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(Adt11::C12)), v4))
        }
        Adt11::C3(_, _, _, _) => {
            Ok(f2(Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(Adt11::C12)), v4))
        }
        Adt11::C4(_, _, _, _) => {
            Ok(f2(Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(Adt11::C12)), v4))
        }
        Adt11::C5(_, _, _) => {
            Ok(f2(Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(Adt11::C12)), v4))
        }
        Adt11::C6(_, _, _) => {
            Ok(f2(Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(Adt11::C12)), v4))
        }
        Adt11::C7(_, _, _, _) => {
            Ok(f2(Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(Adt11::C12)), v4))
        }
        Adt11::C8(_, _, _) => {
            Ok(f2(Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(Adt11::C12)), v4))
        }
        Adt11::C9(_) => {
            Ok(f2(Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(Adt11::C12)), v4))
        }
        Adt11::C10(_, _, _) => {
            Ok(f2(Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(Adt11::C12)), v4))
        }
        Adt11::C11(_, _) => {
            Ok(f2(Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(Adt11::C12)), v4))
        }
        Adt11::C12 => {
            Ok(f2(Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(Adt11::C12)), v4))
        }
        Adt11::C13(r405, r406) => {
            let v40 = (*r405).clone();
            let v41 = (*r406).clone();
            let v42: Adt10 = f51(v41, v1, v2.clone(), v3.clone(), {
                let a393 = v4;
                let a394 = 1u64;
                nat_add(a393, a394)?
            })?;
            Ok(f2(Adt11::C11(std::rc::Rc::new(Adt11::C14(Adt28::C1(Adt17::C2({
                let a395 = v4;
                let a396 = 1u64;
                nat_add(a395, a396)?
            })), false, f32(0u64), std::rc::Rc::new(v40), v3.clone(), std::rc::Rc::new(f25(match v42.clone() {
                Adt10::C0(h397, _) => {
                    h397
                }
            })))), std::rc::Rc::new(Adt11::C13(std::rc::Rc::new(if v1 {
                Adt11::C2(Adt18::C2, std::rc::Rc::new(Adt11::C1(Adt16::C0, Adt17::C2({
                    let a398 = v4;
                    let a399 = 1u64;
                    nat_add(a398, a399)?
                }), v3)), Adt15::C0(match v2 {
                    Adt15::C0(h400, _) => {
                        h400
                    }
                }, None::<Adt13>))
            } else {
                Adt11::C1(Adt16::C0, Adt17::C2({
                    let a401 = v4;
                    let a402 = 1u64;
                    nat_add(a401, a402)?
                }), v3)
            }), std::rc::Rc::new(f26(match v42.clone() {
                Adt10::C0(h403, _) => {
                    h403
                }
            }))))), match v42 {
                Adt10::C0(_, h404) => {
                    h404
                }
            }))
        }
        Adt11::C14(_, _, _, _, _, _) => {
            Ok(f2(Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(Adt11::C12)), v4))
        }
        Adt11::C15(_, _, _) => {
            Ok(f2(Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(Adt11::C12)), v4))
        }
        Adt11::C16(_, _) => {
            Ok(f2(Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(Adt11::C12)), v4))
        }
    }
}

pub fn f52(v0: Adt6, v1: Adt0) -> Option<Adt13> {
    let m407 = v0;
    match m407 {
        Adt6::C0 => {
            None::<Adt13>
        }
        Adt6::C1 => {
            None::<Adt13>
        }
        Adt6::C2 => {
            None::<Adt13>
        }
        Adt6::C3 => {
            None::<Adt13>
        }
        Adt6::C4 => {
            None::<Adt13>
        }
        Adt6::C5 => {
            None::<Adt13>
        }
        Adt6::C6 => {
            None::<Adt13>
        }
        Adt6::C7 => {
            None::<Adt13>
        }
        Adt6::C8 => {
            None::<Adt13>
        }
        Adt6::C9 => {
            None::<Adt13>
        }
        Adt6::C10 => {
            None::<Adt13>
        }
        Adt6::C11 => {
            None::<Adt13>
        }
        Adt6::C12 => {
            None::<Adt13>
        }
        Adt6::C13 => {
            None::<Adt13>
        }
        Adt6::C14 => {
            f111(v1)
        }
        Adt6::C15 => {
            f111(v1)
        }
        Adt6::C16 => {
            f111(v1)
        }
        Adt6::C17 => {
            f111(v1)
        }
        Adt6::C18 => {
            f111(v1)
        }
        Adt6::C19 => {
            f111(v1)
        }
        Adt6::C20 => {
            f111(v1)
        }
        Adt6::C21 => {
            f111(v1)
        }
        Adt6::C22 => {
            f111(v1)
        }
        Adt6::C23 => {
            f111(v1)
        }
        Adt6::C24 => {
            f111(v1)
        }
        Adt6::C25 => {
            None::<Adt13>
        }
        Adt6::C26 => {
            None::<Adt13>
        }
        Adt6::C27 => {
            None::<Adt13>
        }
        Adt6::C28 => {
            None::<Adt13>
        }
        Adt6::C29 => {
            None::<Adt13>
        }
        Adt6::C30 => {
            None::<Adt13>
        }
        Adt6::C31 => {
            None::<Adt13>
        }
        Adt6::C32 => {
            None::<Adt13>
        }
        Adt6::C33 => {
            None::<Adt13>
        }
        Adt6::C34 => {
            None::<Adt13>
        }
        Adt6::C35 => {
            None::<Adt13>
        }
        Adt6::C36 => {
            None::<Adt13>
        }
        Adt6::C37 => {
            None::<Adt13>
        }
        Adt6::C38 => {
            f111(v1)
        }
        Adt6::C39 => {
            None::<Adt13>
        }
        Adt6::C40(v2) => {
            Some(f80(v2))
        }
        Adt6::C41(v3) => {
            f111(v3)
        }
    }
}

pub fn f53(v0: u64) -> bool {
    let a408 = v0;
    let a409 = 0u64;
    if nat_eq(a408, a409) {
        true
    } else {
        let a410 = v0;
        let a411 = 2u64;
        if nat_eq(a410, a411) {
            true
        } else {
            let a412 = v0;
            let a413 = 8u64;
            if nat_eq(a412, a413) {
                true
            } else {
                let a414 = v0;
                let a415 = 48u64;
                if nat_eq(a414, a415) {
                    true
                } else {
                    let a416 = 7u64;
                    let a417 = v0;
                    if nat_lt(a416, a417) {
                        let a418 = v0;
                        let a419 = 14u64;
                        nat_lt(a418, a419)
                    } else {
                        false
                    }
                }
            }
        }
    }
}

pub fn f54(v0: Adt0) -> Option<u64> {
    let m420 = v0;
    match m420 {
        Adt0::C0 => {
            None::<u64>
        }
        Adt0::C1 => {
            None::<u64>
        }
        Adt0::C2 => {
            None::<u64>
        }
        Adt0::C3 => {
            None::<u64>
        }
        Adt0::C4(_) => {
            None::<u64>
        }
        Adt0::C5 => {
            None::<u64>
        }
        Adt0::C6 => {
            None::<u64>
        }
        Adt0::C7 => {
            None::<u64>
        }
        Adt0::C8(_) => {
            None::<u64>
        }
        Adt0::C9(_, _) => {
            None::<u64>
        }
        Adt0::C10(_) => {
            None::<u64>
        }
        Adt0::C11(_, _) => {
            None::<u64>
        }
        Adt0::C12(v8) => {
            Some(v8)
        }
        Adt0::C13(_, _) => {
            None::<u64>
        }
        Adt0::C14 => {
            None::<u64>
        }
        Adt0::C15(_, _) => {
            None::<u64>
        }
    }
}

pub fn f55(v0: Adt0, v1: u64, v2: u64) -> Option<Adt0> {
    let m421 = f56(v0, v1);
    match m421 {
        None => {
            None::<Adt0>
        }
        Some(v3) => {
            f56(v3, v2)
        }
    }
}

pub fn f56(p427: Adt0, p428: u64) -> Option<Adt0> {
    let m426 = (p427, p428);
    {
        let mut m426 = m426;
        loop {
            let step = {
                let (v0, v1) = m426;
                let m422 = v0;
                match m422 {
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
                    Adt0::C15(r424, r425) => {
                        let v12 = (*r424).clone();
                        let v13 = (*r425).clone();
                        let m423 = v1;
                        if m423 == 0 {
                            Err::<(Adt0, u64), Option<Adt0>>(Some(v12))
                        } else {
                            let v14 = m423 - 1;
                            Ok::<(Adt0, u64), Option<Adt0>>((v13, v14))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m426 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f57(v0: Adt0, v1: Adt8, v2: Adt0, v3: Adt0) -> bool {
    f112(v2, v1, f113(v0, Adt0::C14, v3))
}

pub fn f58(v0: Adt0, v1: u64, v2: Adt17, v3: u64) -> R<Adt28> {
    let m429 = v0;
    match m429 {
        Adt0::C0 => {
            Ok(Adt28::C5)
        }
        Adt0::C1 => {
            Ok(Adt28::C5)
        }
        Adt0::C2 => {
            Ok(Adt28::C5)
        }
        Adt0::C3 => {
            Ok(Adt28::C5)
        }
        Adt0::C4(_) => {
            Ok(Adt28::C5)
        }
        Adt0::C5 => {
            Ok(Adt28::C5)
        }
        Adt0::C6 => {
            Ok(Adt28::C5)
        }
        Adt0::C7 => {
            Ok(Adt28::C5)
        }
        Adt0::C8(_) => {
            Ok(Adt28::C5)
        }
        Adt0::C9(_, _) => {
            Ok(Adt28::C5)
        }
        Adt0::C10(_) => {
            Ok(Adt28::C5)
        }
        Adt0::C11(_, _) => {
            Ok(Adt28::C5)
        }
        Adt0::C12(_) => {
            Ok(Adt28::C5)
        }
        Adt0::C13(_, _) => {
            Ok(Adt28::C5)
        }
        Adt0::C14 => {
            Ok(Adt28::C5)
        }
        Adt0::C15(_, r434) => {
            let v15 = (*r434).clone();
            Ok(Adt28::C6(std::rc::Rc::new({
                let a430 = v3;
                let a431 = v1;
                if nat_eq(a430, a431) {
                    Adt28::C1(v2.clone())
                } else {
                    Adt28::C0(Adt29::C0)
                }
            }), std::rc::Rc::new(f58(v15, v1, v2, {
                let a432 = v3;
                let a433 = 1u64;
                nat_add(a432, a433)?
            })?)))
        }
    }
}

pub fn f59(v0: Adt11) -> bool {
    let m435 = v0;
    match m435 {
        Adt11::C0(_, _) => {
            false
        }
        Adt11::C1(v3, _, _) => {
            let m436 = v3;
            match m436 {
                Adt16::C0 => {
                    true
                }
                Adt16::C1 => {
                    true
                }
                Adt16::C2 => {
                    true
                }
                Adt16::C3 => {
                    false
                }
                Adt16::C4 => {
                    false
                }
            }
        }
        Adt11::C2(_, _, _) => {
            false
        }
        Adt11::C3(_, _, _, _) => {
            false
        }
        Adt11::C4(_, _, _, _) => {
            false
        }
        Adt11::C5(_, _, _) => {
            false
        }
        Adt11::C6(_, _, _) => {
            false
        }
        Adt11::C7(_, _, _, _) => {
            false
        }
        Adt11::C8(_, _, _) => {
            false
        }
        Adt11::C9(_) => {
            false
        }
        Adt11::C10(_, _, _) => {
            false
        }
        Adt11::C11(_, _) => {
            false
        }
        Adt11::C12 => {
            false
        }
        Adt11::C13(_, _) => {
            false
        }
        Adt11::C14(_, _, _, _, _, _) => {
            false
        }
        Adt11::C15(_, _, _) => {
            false
        }
        Adt11::C16(_, _) => {
            false
        }
    }
}

pub fn f60(v0: Adt0, v1: Adt0, v2: Adt5) -> Adt0 {
    let m437 = v2;
    match m437 {
        Adt5::C0 => {
            Adt0::C14
        }
        Adt5::C1 => {
            Adt0::C15(std::rc::Rc::new(f13(v1)), std::rc::Rc::new(Adt0::C14))
        }
        Adt5::C2 => {
            Adt0::C15(std::rc::Rc::new(f13(v1)), std::rc::Rc::new(Adt0::C14))
        }
        Adt5::C3 => {
            Adt0::C15(std::rc::Rc::new(f14(v1)), std::rc::Rc::new(Adt0::C14))
        }
        Adt5::C4 => {
            Adt0::C14
        }
        Adt5::C5 => {
            Adt0::C15(std::rc::Rc::new(f13(v1.clone())), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(v1), std::rc::Rc::new(Adt0::C14))))
        }
        Adt5::C6 => {
            Adt0::C14
        }
        Adt5::C7 => {
            Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C14))
        }
        Adt5::C8 => {
            Adt0::C15(std::rc::Rc::new(f13(v1.clone())), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(f14(v1)), std::rc::Rc::new(Adt0::C14))))
        }
        Adt5::C9 => {
            Adt0::C14
        }
        Adt5::C10 => {
            Adt0::C14
        }
        Adt5::C11 => {
            Adt0::C14
        }
        Adt5::C12 => {
            Adt0::C14
        }
        Adt5::C13 => {
            Adt0::C14
        }
        Adt5::C14 => {
            Adt0::C14
        }
        Adt5::C15(v3) => {
            let m438 = f54(v1);
            match m438 {
                None => {
                    Adt0::C14
                }
                Some(v4) => {
                    let m439 = f55(v0, v4, v3);
                    match m439 {
                        None => {
                            Adt0::C14
                        }
                        Some(v5) => {
                            v5
                        }
                    }
                }
            }
        }
    }
}

pub fn f61(p444: Adt0, v1: Adt0) -> bool {
    let m443 = p444;
    {
        let mut m443 = m443;
        loop {
            let step = {
                let v0 = m443;
                let m440 = v0;
                match m440 {
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
                    Adt0::C15(r441, r442) => {
                        let v12 = (*r441).clone();
                        let v13 = (*r442).clone();
                        if f114(v12, v1.clone()) {
                            Ok::<Adt0, bool>(v13)
                        } else {
                            Err::<Adt0, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m443 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f62(v0: List<u64>, v1: Adt0, v2: Adt2) -> Adt28 {
    let m445 = v0;
    match m445.uncons() {
        None => {
            Adt28::C5
        }
        Some((v3, v4)) => {
            let m446 = v1;
            match m446 {
                Adt0::C0 => {
                    Adt28::C5
                }
                Adt0::C1 => {
                    Adt28::C5
                }
                Adt0::C2 => {
                    Adt28::C5
                }
                Adt0::C3 => {
                    Adt28::C5
                }
                Adt0::C4(_) => {
                    Adt28::C5
                }
                Adt0::C5 => {
                    Adt28::C5
                }
                Adt0::C6 => {
                    Adt28::C5
                }
                Adt0::C7 => {
                    Adt28::C5
                }
                Adt0::C8(_) => {
                    Adt28::C5
                }
                Adt0::C9(_, _) => {
                    Adt28::C5
                }
                Adt0::C10(_) => {
                    Adt28::C5
                }
                Adt0::C11(_, _) => {
                    Adt28::C5
                }
                Adt0::C12(_) => {
                    Adt28::C5
                }
                Adt0::C13(_, _) => {
                    Adt28::C5
                }
                Adt0::C14 => {
                    Adt28::C5
                }
                Adt0::C15(r447, r448) => {
                    let v15 = (*r447).clone();
                    let v16 = (*r448).clone();
                    Adt28::C6(std::rc::Rc::new(f23(v3, v15, v2.clone())), std::rc::Rc::new(f62(v4, v16, v2)))
                }
            }
        }
    }
}

pub fn f63(v0: Adt2) -> Bytes {
    let m449 = v0;
    match m449 {
        Adt2::C0(_, _) => {
            let a450 = Str::lit("value");
            utf8_encode(a450)
        }
        Adt2::C1(_) => {
            let a451 = Str::lit("var");
            utf8_encode(a451)
        }
        Adt2::C2(_, _, _, _) => {
            let a452 = Str::lit("let");
            utf8_encode(a452)
        }
        Adt2::C3(_, _, _) => {
            let a453 = Str::lit("cond");
            utf8_encode(a453)
        }
        Adt2::C4(_, _, _) => {
            let a454 = Str::lit("match");
            utf8_encode(a454)
        }
        Adt2::C5(_, _, _) => {
            let a455 = Str::lit("build");
            utf8_encode(a455)
        }
        Adt2::C6(_, _) => {
            let a456 = Str::lit("call");
            utf8_encode(a456)
        }
        Adt2::C7(_, _) => {
            let a457 = Str::lit("closure");
            utf8_encode(a457)
        }
        Adt2::C8(_, _) => {
            let a458 = Str::lit("apply");
            utf8_encode(a458)
        }
        Adt2::C9(_, _) => {
            let a459 = Str::lit("prim");
            utf8_encode(a459)
        }
        Adt2::C10(_) => {
            let a460 = Str::lit("first");
            utf8_encode(a460)
        }
        Adt2::C11(_) => {
            let a461 = Str::lit("second");
            utf8_encode(a461)
        }
        Adt2::C12(_, _) => {
            let a462 = Str::lit("field");
            utf8_encode(a462)
        }
        Adt2::C13 => {
            let a463 = Str::lit("");
            utf8_encode(a463)
        }
        Adt2::C14(_, _) => {
            let a464 = Str::lit("");
            utf8_encode(a464)
        }
        Adt2::C15(_, _, _) => {
            let a465 = Str::lit("");
            utf8_encode(a465)
        }
    }
}

pub fn f64(v0: Adt11, v1: Adt11) -> Adt11 {
    let m466 = v0;
    match m466 {
        Adt11::C0(_, _) => {
            Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(v1))
        }
        Adt11::C1(_, _, _) => {
            Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(v1))
        }
        Adt11::C2(_, _, _) => {
            Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(v1))
        }
        Adt11::C3(_, _, _, _) => {
            Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(v1))
        }
        Adt11::C4(_, _, _, _) => {
            Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(v1))
        }
        Adt11::C5(_, _, _) => {
            Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(v1))
        }
        Adt11::C6(_, _, _) => {
            Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(v1))
        }
        Adt11::C7(_, _, _, _) => {
            Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(v1))
        }
        Adt11::C8(_, _, _) => {
            Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(v1))
        }
        Adt11::C9(r467) => {
            let v31 = (*r467).clone();
            f81(v31)
        }
        Adt11::C10(_, _, _) => {
            Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(v1))
        }
        Adt11::C11(_, _) => {
            Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(v1))
        }
        Adt11::C12 => {
            Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(v1))
        }
        Adt11::C13(_, _) => {
            Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(v1))
        }
        Adt11::C14(_, _, _, _, _, _) => {
            Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(v1))
        }
        Adt11::C15(_, _, _) => {
            Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(v1))
        }
        Adt11::C16(_, _) => {
            Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(v1))
        }
    }
}

pub fn f65(v0: Adt0) -> Option<Adt0> {
    let m468 = v0;
    match m468 {
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
        Adt0::C8(r469) => {
            let v2 = (*r469).clone();
            Some(v2)
        }
        Adt0::C9(r470, _) => {
            let v3 = (*r470).clone();
            Some(v3)
        }
        Adt0::C10(r471) => {
            let v5 = (*r471).clone();
            Some(v5)
        }
        Adt0::C11(r472, _) => {
            let v6 = (*r472).clone();
            Some(v6)
        }
        Adt0::C12(_) => {
            None::<Adt0>
        }
        Adt0::C13(r473, _) => {
            let v9 = (*r473).clone();
            Some(v9)
        }
        Adt0::C14 => {
            None::<Adt0>
        }
        Adt0::C15(r474, _) => {
            let v11 = (*r474).clone();
            Some(v11)
        }
    }
}

pub fn f66(v0: Adt0) -> Option<Adt0> {
    let m475 = v0;
    match m475 {
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
        Adt0::C9(_, r476) => {
            let v4 = (*r476).clone();
            Some(v4)
        }
        Adt0::C10(_) => {
            None::<Adt0>
        }
        Adt0::C11(_, r477) => {
            let v7 = (*r477).clone();
            Some(v7)
        }
        Adt0::C12(_) => {
            None::<Adt0>
        }
        Adt0::C13(_, r478) => {
            let v10 = (*r478).clone();
            Some(v10)
        }
        Adt0::C14 => {
            None::<Adt0>
        }
        Adt0::C15(_, r479) => {
            let v12 = (*r479).clone();
            Some(v12)
        }
    }
}

pub fn f67(v0: Adt0) -> u64 {
    let m480 = v0;
    match m480 {
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

pub fn f68(p486: Adt0, p487: u64) -> Option<Adt0> {
    let m485 = (p486, p487);
    {
        let mut m485 = m485;
        loop {
            let step = {
                let (v0, v1) = m485;
                let m481 = v0;
                match m481 {
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
                    Adt0::C15(r483, r484) => {
                        let v12 = (*r483).clone();
                        let v13 = (*r484).clone();
                        let m482 = v1;
                        if m482 == 0 {
                            Err::<(Adt0, u64), Option<Adt0>>(Some(v12))
                        } else {
                            let v14 = m482 - 1;
                            Ok::<(Adt0, u64), Option<Adt0>>((v13, v14))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m485 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f69(v0: Bytes, v1: Adt0) -> Adt15 {
    Adt15::C0(v0, f115(v1))
}

pub fn f70(v0: Bytes) -> R<Option<u64>> {
    let m488 = f116(v0, false)?;
    match m488 {
        None => {
            Ok(None::<u64>)
        }
        Some(v1) => {
            Ok(Some({
                let (_, h489) = v1;
                h489
            }))
        }
    }
}

pub fn f71(_: u64) -> Adt11 {
    Adt11::C0(Adt12::C8, Adt15::C0({
        let a490 = Str::lit("");
        utf8_encode(a490)
    }, None::<Adt13>))
}

pub fn f72(v0: Bytes) -> R<Option<(bool, u64)>> {
    let m491 = f116(v0, true)?;
    match m491 {
        None => {
            Ok(None::<(bool, u64)>)
        }
        Some(v1) => {
            let (h492, _) = v1;
            if h492 {
                let a494 = {
                    let (_, h493) = v1;
                    h493
                };
                let a495 = 9223372036854775809u64;
                if nat_lt(a494, a495) {
                    Ok(Some(({
                        let a497 = 0u64;
                        let a498 = {
                            let (_, h496) = v1;
                            h496
                        };
                        nat_lt(a497, a498)
                    }, {
                        let (_, h499) = v1;
                        h499
                    })))
                } else {
                    Ok(None::<(bool, u64)>)
                }
            } else {
                let a501 = {
                    let (_, h500) = v1;
                    h500
                };
                let a502 = 9223372036854775808u64;
                if nat_lt(a501, a502) {
                    Ok(Some((false, {
                        let (_, h503) = v1;
                        h503
                    })))
                } else {
                    Ok(None::<(bool, u64)>)
                }
            }
        }
    }
}

pub fn f73(v0: Bytes, v1: Adt0, v2: Bytes) -> R<Adt11> {
    let m504 = f117(v1.clone());
    match m504 {
        None => {
            Ok(f71(0u64))
        }
        Some(v3) => {
            let m505 = f118(v0)?;
            match m505 {
                None => {
                    Ok(f71(0u64))
                }
                Some(v4) => {
                    Ok(Adt11::C0(Adt12::C4(f80(v3), {
                        let (h506, _) = v4;
                        h506
                    }, {
                        let (_, h507) = v4;
                        h507
                    }), f69(v2, v1)))
                }
            }
        }
    }
}

pub fn f74(v0: Bytes) -> R<Option<Bytes>> {
    let v1: u64 = {
        let a508 = v0.clone();
        length_bytes(a508)
    };
    let v2: ((u64, Bytes), bool) = f119(Fn0::F120(v0), v1, (0u64, {
        let a509 = Str::lit("");
        utf8_encode(a509)
    }))?;
    let a512 = {
        let (h510, _) = {
            let (h511, _) = v2.clone();
            h511
        };
        h510
    };
    let a513 = v1;
    if nat_eq(a512, a513) {
        Ok(Some({
            let (_, h514) = {
                let (h515, _) = v2;
                h515
            };
            h514
        }))
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f75(v0: Adt11) -> bool {
    let m516 = v0;
    match m516 {
        Adt11::C0(v1, _) => {
            let m517 = v1;
            match m517 {
                Adt12::C0 => {
                    false
                }
                Adt12::C1(_) => {
                    false
                }
                Adt12::C2(_) => {
                    false
                }
                Adt12::C3(_, _) => {
                    false
                }
                Adt12::C4(_, _, _) => {
                    false
                }
                Adt12::C5(_) => {
                    false
                }
                Adt12::C6(_) => {
                    false
                }
                Adt12::C7(_) => {
                    false
                }
                Adt12::C8 => {
                    true
                }
            }
        }
        Adt11::C1(_, _, _) => {
            false
        }
        Adt11::C2(_, _, _) => {
            false
        }
        Adt11::C3(_, _, _, _) => {
            false
        }
        Adt11::C4(_, _, _, _) => {
            false
        }
        Adt11::C5(_, _, _) => {
            false
        }
        Adt11::C6(_, _, _) => {
            false
        }
        Adt11::C7(_, _, _, _) => {
            false
        }
        Adt11::C8(_, _, _) => {
            false
        }
        Adt11::C9(_) => {
            false
        }
        Adt11::C10(_, _, _) => {
            false
        }
        Adt11::C11(_, _) => {
            false
        }
        Adt11::C12 => {
            false
        }
        Adt11::C13(_, _) => {
            false
        }
        Adt11::C14(_, _, _, _, _, _) => {
            false
        }
        Adt11::C15(_, _, _) => {
            false
        }
        Adt11::C16(_, _) => {
            false
        }
    }
}

pub fn f76(v0: Adt0) -> u64 {
    let m518 = v0;
    match m518 {
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

pub fn f77(v0: Adt1) -> u64 {
    let m519 = v0;
    match m519 {
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

pub fn f78(v0: Adt0) -> Adt1 {
    let m520 = v0;
    match m520 {
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

pub fn f79(v0: Adt2, v1: u64) -> bool {
    let m521 = v0;
    match m521 {
        Adt2::C0(_, _) => {
            false
        }
        Adt2::C1(v4) => {
            let a522 = v4;
            let a523 = v1;
            nat_eq(a522, a523)
        }
        Adt2::C2(v5, _, r526, r527) => {
            let v7 = (*r526).clone();
            let v8 = (*r527).clone();
            if f79(v7, v1) {
                true
            } else {
                let a524 = v5;
                let a525 = v1;
                if nat_eq(a524, a525) {
                    false
                } else {
                    f79(v8, v1)
                }
            }
        }
        Adt2::C3(r528, r529, r530) => {
            let v9 = (*r528).clone();
            let v10 = (*r529).clone();
            let v11 = (*r530).clone();
            if f79(v9, v1) {
                true
            } else {
                if f79(v10, v1) {
                    true
                } else {
                    f79(v11, v1)
                }
            }
        }
        Adt2::C4(_, r531, r532) => {
            let v13 = (*r531).clone();
            let v14 = (*r532).clone();
            if f79(v13, v1) {
                true
            } else {
                f79(v14, v1)
            }
        }
        Adt2::C5(_, _, r533) => {
            let v17 = (*r533).clone();
            f79(v17, v1)
        }
        Adt2::C6(_, r534) => {
            let v19 = (*r534).clone();
            f79(v19, v1)
        }
        Adt2::C7(_, r535) => {
            let v21 = (*r535).clone();
            f79(v21, v1)
        }
        Adt2::C8(r536, r537) => {
            let v22 = (*r536).clone();
            let v23 = (*r537).clone();
            if f79(v22, v1) {
                true
            } else {
                f79(v23, v1)
            }
        }
        Adt2::C9(_, r538) => {
            let v25 = (*r538).clone();
            f79(v25, v1)
        }
        Adt2::C10(r539) => {
            let v26 = (*r539).clone();
            f79(v26, v1)
        }
        Adt2::C11(r540) => {
            let v27 = (*r540).clone();
            f79(v27, v1)
        }
        Adt2::C12(r541, _) => {
            let v28 = (*r541).clone();
            f79(v28, v1)
        }
        Adt2::C13 => {
            false
        }
        Adt2::C14(r542, r543) => {
            let v30 = (*r542).clone();
            let v31 = (*r543).clone();
            if f79(v30, v1) {
                true
            } else {
                f79(v31, v1)
            }
        }
        Adt2::C15(_, v33, r544) => {
            let v34 = (*r544).clone();
            if f121(v33, v1) {
                false
            } else {
                f79(v34, v1)
            }
        }
    }
}

pub fn f80(v0: Adt1) -> Adt13 {
    let m545 = v0;
    match m545 {
        Adt1::C0 => {
            Adt13::C0
        }
        Adt1::C1 => {
            Adt13::C1
        }
        Adt1::C2 => {
            Adt13::C2
        }
        Adt1::C3 => {
            Adt13::C3
        }
        Adt1::C4 => {
            Adt13::C4
        }
        Adt1::C5 => {
            Adt13::C5
        }
        Adt1::C6 => {
            Adt13::C6
        }
        Adt1::C7 => {
            Adt13::C7
        }
    }
}

pub fn f81(v0: Adt11) -> Adt11 {
    v0
}

pub fn f82(v0: Adt11) -> bool {
    if f30(f25(v0.clone())) {
        f122(f26(v0))
    } else {
        false
    }
}

pub fn f83(v0: Adt11) -> bool {
    let m546 = v0;
    match m546 {
        Adt11::C0(_, _) => {
            false
        }
        Adt11::C1(_, _, _) => {
            false
        }
        Adt11::C2(_, _, _) => {
            false
        }
        Adt11::C3(_, _, _, _) => {
            false
        }
        Adt11::C4(_, _, _, _) => {
            false
        }
        Adt11::C5(_, _, _) => {
            false
        }
        Adt11::C6(_, _, _) => {
            false
        }
        Adt11::C7(_, _, r547, _) => {
            let v25 = (*r547).clone();
            f82(v25)
        }
        Adt11::C8(_, _, _) => {
            false
        }
        Adt11::C9(_) => {
            false
        }
        Adt11::C10(_, _, _) => {
            false
        }
        Adt11::C11(_, _) => {
            false
        }
        Adt11::C12 => {
            false
        }
        Adt11::C13(_, _) => {
            false
        }
        Adt11::C14(_, _, _, _, _, _) => {
            false
        }
        Adt11::C15(_, _, _) => {
            false
        }
        Adt11::C16(_, _) => {
            false
        }
    }
}

pub fn f84(v0: Adt11, v1: u64) -> R<Adt10> {
    let m548 = f26(v0.clone());
    match m548 {
        Adt11::C0(_, _) => {
            Ok(f2(v0, v1))
        }
        Adt11::C1(_, _, _) => {
            Ok(f2(v0, v1))
        }
        Adt11::C2(_, _, _) => {
            Ok(f2(v0, v1))
        }
        Adt11::C3(_, _, _, _) => {
            Ok(f2(v0, v1))
        }
        Adt11::C4(_, _, _, _) => {
            Ok(f2(v0, v1))
        }
        Adt11::C5(_, _, _) => {
            Ok(f2(v0, v1))
        }
        Adt11::C6(_, _, _) => {
            Ok(f2(v0, v1))
        }
        Adt11::C7(r553, r554, r555, v27) => {
            let v24 = (*r553).clone();
            let v25 = (*r554).clone();
            let v26 = (*r555).clone();
            let v28: Adt17 = Adt17::C1({
                let a549 = v1;
                let a550 = 1u64;
                nat_add(a549, a550)?
            });
            Ok(f2(Adt11::C11(std::rc::Rc::new(f105(f25(v0), Adt11::C14(Adt28::C1(v28.clone()), false, f32(0u64), std::rc::Rc::new(v24), v27.clone(), std::rc::Rc::new(Adt11::C12)))), std::rc::Rc::new(Adt11::C7(std::rc::Rc::new(Adt11::C1(Adt16::C0, v28, v27.clone())), std::rc::Rc::new(v25), std::rc::Rc::new(v26), v27))), {
                let a551 = v1;
                let a552 = 1u64;
                nat_add(a551, a552)?
            }))
        }
        Adt11::C8(_, _, _) => {
            Ok(f2(v0, v1))
        }
        Adt11::C9(_) => {
            Ok(f2(v0, v1))
        }
        Adt11::C10(_, _, _) => {
            Ok(f2(v0, v1))
        }
        Adt11::C11(_, _) => {
            Ok(f2(v0, v1))
        }
        Adt11::C12 => {
            Ok(f2(v0, v1))
        }
        Adt11::C13(_, _) => {
            Ok(f2(v0, v1))
        }
        Adt11::C14(_, _, _, _, _, _) => {
            Ok(f2(v0, v1))
        }
        Adt11::C15(_, _, _) => {
            Ok(f2(v0, v1))
        }
        Adt11::C16(_, _) => {
            Ok(f2(v0, v1))
        }
    }
}

pub fn f85(v0: Adt11) -> u64 {
    if f30(f25(v0.clone())) {
        f88(f26(v0))
    } else {
        0u64
    }
}

pub fn f86(v0: Adt11, v1: Adt15) -> Adt11 {
    f123(v0.clone(), v0, v1)
}

pub fn f87(v0: Adt11, v1: Adt11) -> R<bool> {
    let v2: Adt33 = f124(v0, f125(0u64))?;
    let v3: Adt33 = f124(v1, f125(0u64))?;
    if f126(match v2.clone() {
        Adt33::C0(h556, _, _, _) => {
            h556
        }
    }, match v3.clone() {
        Adt33::C0(h557, _, _, _) => {
            h557
        }
    }) {
        Ok(f127(match v2 {
            Adt33::C0(_, h558, _, _) => {
                h558
            }
        }, match v3 {
            Adt33::C0(_, h559, _, _) => {
                h559
            }
        }))
    } else {
        Ok(false)
    }
}

pub fn f88(v0: Adt11) -> u64 {
    let m560 = v0;
    match m560 {
        Adt11::C0(v1, _) => {
            let m561 = v1;
            match m561 {
                Adt12::C0 => {
                    0u64
                }
                Adt12::C1(v3) => {
                    if v3 {
                        1u64
                    } else {
                        2u64
                    }
                }
                Adt12::C2(_) => {
                    0u64
                }
                Adt12::C3(_, _) => {
                    0u64
                }
                Adt12::C4(_, _, _) => {
                    0u64
                }
                Adt12::C5(_) => {
                    0u64
                }
                Adt12::C6(_) => {
                    0u64
                }
                Adt12::C7(_) => {
                    0u64
                }
                Adt12::C8 => {
                    0u64
                }
            }
        }
        Adt11::C1(_, _, _) => {
            0u64
        }
        Adt11::C2(_, _, _) => {
            0u64
        }
        Adt11::C3(_, _, _, _) => {
            0u64
        }
        Adt11::C4(_, _, _, _) => {
            0u64
        }
        Adt11::C5(_, _, _) => {
            0u64
        }
        Adt11::C6(_, _, _) => {
            0u64
        }
        Adt11::C7(_, _, _, _) => {
            0u64
        }
        Adt11::C8(_, _, _) => {
            0u64
        }
        Adt11::C9(_) => {
            0u64
        }
        Adt11::C10(_, _, _) => {
            0u64
        }
        Adt11::C11(_, _) => {
            0u64
        }
        Adt11::C12 => {
            0u64
        }
        Adt11::C13(_, _) => {
            0u64
        }
        Adt11::C14(_, _, _, _, _, _) => {
            0u64
        }
        Adt11::C15(_, _, _) => {
            0u64
        }
        Adt11::C16(_, _) => {
            0u64
        }
    }
}

pub fn f89(v0: Adt11, v1: Adt11, v2: Adt11) -> Adt11 {
    let m562 = v1;
    match m562 {
        Adt11::C0(_, _) => {
            Adt11::C11(std::rc::Rc::new(f25(v0)), std::rc::Rc::new(v2))
        }
        Adt11::C1(_, _, _) => {
            Adt11::C11(std::rc::Rc::new(f25(v0)), std::rc::Rc::new(v2))
        }
        Adt11::C2(_, _, _) => {
            Adt11::C11(std::rc::Rc::new(f25(v0)), std::rc::Rc::new(v2))
        }
        Adt11::C3(_, _, _, _) => {
            Adt11::C11(std::rc::Rc::new(f25(v0)), std::rc::Rc::new(v2))
        }
        Adt11::C4(_, _, _, _) => {
            Adt11::C11(std::rc::Rc::new(f25(v0)), std::rc::Rc::new(v2))
        }
        Adt11::C5(_, _, _) => {
            Adt11::C11(std::rc::Rc::new(f25(v0)), std::rc::Rc::new(v2))
        }
        Adt11::C6(_, _, _) => {
            Adt11::C11(std::rc::Rc::new(f25(v0)), std::rc::Rc::new(v2))
        }
        Adt11::C7(_, _, _, _) => {
            Adt11::C11(std::rc::Rc::new(f25(v0)), std::rc::Rc::new(v2))
        }
        Adt11::C8(_, _, _) => {
            Adt11::C11(std::rc::Rc::new(f25(v0)), std::rc::Rc::new(v2))
        }
        Adt11::C9(r563) => {
            let v32 = (*r563).clone();
            Adt11::C11(std::rc::Rc::new(f105(f25(v0), f25(v32.clone()))), std::rc::Rc::new(f26(v32)))
        }
        Adt11::C10(_, _, _) => {
            Adt11::C11(std::rc::Rc::new(f25(v0)), std::rc::Rc::new(v2))
        }
        Adt11::C11(_, _) => {
            Adt11::C11(std::rc::Rc::new(f25(v0)), std::rc::Rc::new(v2))
        }
        Adt11::C12 => {
            Adt11::C11(std::rc::Rc::new(f25(v0)), std::rc::Rc::new(v2))
        }
        Adt11::C13(_, _) => {
            Adt11::C11(std::rc::Rc::new(f25(v0)), std::rc::Rc::new(v2))
        }
        Adt11::C14(_, _, _, _, _, _) => {
            Adt11::C11(std::rc::Rc::new(f25(v0)), std::rc::Rc::new(v2))
        }
        Adt11::C15(_, _, _) => {
            Adt11::C11(std::rc::Rc::new(f25(v0)), std::rc::Rc::new(v2))
        }
        Adt11::C16(_, _) => {
            Adt11::C11(std::rc::Rc::new(f25(v0)), std::rc::Rc::new(v2))
        }
    }
}

pub fn f90(v0: Adt5) -> Bytes {
    let m564 = v0;
    match m564 {
        Adt5::C0 => {
            let a565 = Str::lit("none");
            utf8_encode(a565)
        }
        Adt5::C1 => {
            let a566 = Str::lit("some");
            utf8_encode(a566)
        }
        Adt5::C2 => {
            let a567 = Str::lit("ok");
            utf8_encode(a567)
        }
        Adt5::C3 => {
            let a568 = Str::lit("error");
            utf8_encode(a568)
        }
        Adt5::C4 => {
            let a569 = Str::lit("nil");
            utf8_encode(a569)
        }
        Adt5::C5 => {
            let a570 = Str::lit("cons");
            utf8_encode(a570)
        }
        Adt5::C6 => {
            let a571 = Str::lit("zero");
            utf8_encode(a571)
        }
        Adt5::C7 => {
            let a572 = Str::lit("succ");
            utf8_encode(a572)
        }
        Adt5::C8 => {
            let a573 = Str::lit("pair");
            utf8_encode(a573)
        }
        Adt5::C9 => {
            let a574 = Str::lit("true");
            utf8_encode(a574)
        }
        Adt5::C10 => {
            let a575 = Str::lit("false");
            utf8_encode(a575)
        }
        Adt5::C11 => {
            let a576 = Str::lit("unit");
            utf8_encode(a576)
        }
        Adt5::C12 => {
            let a577 = Str::lit("lt");
            utf8_encode(a577)
        }
        Adt5::C13 => {
            let a578 = Str::lit("eq");
            utf8_encode(a578)
        }
        Adt5::C14 => {
            let a579 = Str::lit("gt");
            utf8_encode(a579)
        }
        Adt5::C15(_) => {
            let a580 = Str::lit("adt");
            utf8_encode(a580)
        }
    }
}

pub fn f91(v0: Adt17, v1: Adt2, v2: Adt11, v3: u64) -> R<Adt10> {
    let m581 = f128(v1.clone(), v2.clone(), 9u64)?;
    match m581 {
        None => {
            Ok(f2(Adt11::C12, v3))
        }
        Some(v4) => {
            let m582 = f128(v1, v2, 10u64)?;
            match m582 {
                None => {
                    Ok(f2(Adt11::C12, v3))
                }
                Some(v5) => {
                    f28(f129(v0), {
                        let (_, h583) = v4;
                        h583
                    }, {
                        let (_, h584) = v5;
                        h584
                    }, f33(Adt5::C9), v3)
                }
            }
        }
    }
}

pub fn f92(v0: Adt17, v1: Adt2, v2: Adt11, v3: u64) -> R<Adt10> {
    let m585 = f128(v1.clone(), v2.clone(), 6u64)?;
    match m585 {
        None => {
            Ok(f2(Adt11::C12, v3))
        }
        Some(v4) => {
            let m586 = f128(v1, v2, 7u64)?;
            match m586 {
                None => {
                    Ok(f2(Adt11::C12, v3))
                }
                Some(v5) => {
                    let v6: Adt11 = {
                        let (_, h587) = v5;
                        h587
                    };
                    let v7: Adt28 = f130({
                        let (h588, _) = v5;
                        h588
                    }, 0u64);
                    let v8: Adt11 = if f131(v7.clone()) {
                        v6
                    } else {
                        Adt11::C11(std::rc::Rc::new(Adt11::C14(v7, false, f32(0u64), std::rc::Rc::new(Adt11::C10(Adt27::C3, v0.clone(), Adt15::C0({
                            let a589 = Str::lit("");
                            utf8_encode(a589)
                        }, None::<Adt13>))), f33(Adt5::C7), std::rc::Rc::new(f25(v6.clone())))), std::rc::Rc::new(f26(v6)))
                    };
                    f28(Adt11::C10(Adt27::C0, v0, f33(Adt5::C6)), {
                        let (_, h590) = v4;
                        h590
                    }, v8, f33(Adt5::C6), v3)
                }
            }
        }
    }
}

pub fn f93(v0: Adt17, v1: Adt2, v2: Adt11, v3: u64) -> R<Adt10> {
    let v5: Adt11 = {
        let m591 = f128(v1.clone(), v2.clone(), 14u64)?;
        match m591 {
            None => {
                Adt11::C12
            }
            Some(v4) => {
                Adt11::C15(Adt28::C7(Adt14::C2), std::rc::Rc::new({
                    let (_, h592) = v4;
                    h592
                }), std::rc::Rc::new(Adt11::C12))
            }
        }
    };
    let v7: Adt11 = {
        let m593 = f128(v1.clone(), v2.clone(), 13u64)?;
        match m593 {
            None => {
                v5
            }
            Some(v6) => {
                Adt11::C15(Adt28::C7(Adt14::C1), std::rc::Rc::new({
                    let (_, h594) = v6;
                    h594
                }), std::rc::Rc::new(v5))
            }
        }
    };
    let v9: Adt11 = {
        let m595 = f128(v1, v2, 12u64)?;
        match m595 {
            None => {
                v7
            }
            Some(v8) => {
                Adt11::C15(Adt28::C7(Adt14::C0), std::rc::Rc::new({
                    let (_, h596) = v8;
                    h596
                }), std::rc::Rc::new(v7))
            }
        }
    };
    Ok(f2(f132(v9, f129(v0), f33(Adt5::C12))?, v3))
}

pub fn f94(v0: Adt17, v1: Adt2, v2: Adt11, v3: u64) -> R<Adt10> {
    let v7: Adt11 = {
        let v5: Adt11 = {
            let m597 = f128(v1.clone(), v2.clone(), 1u64)?;
            match m597 {
                None => {
                    Adt11::C12
                }
                Some(v4) => {
                    Adt11::C15(Adt28::C3(Adt30::C0, std::rc::Rc::new(f130({
                        let (h598, _) = v4;
                        h598
                    }, 0u64))), std::rc::Rc::new({
                        let (_, h599) = v4;
                        h599
                    }), std::rc::Rc::new(Adt11::C12))
                }
            }
        };
        let m600 = f128(v1, v2, 0u64)?;
        match m600 {
            None => {
                v5
            }
            Some(v6) => {
                Adt11::C15(Adt28::C0(Adt29::C2), std::rc::Rc::new({
                    let (_, h601) = v6;
                    h601
                }), std::rc::Rc::new(v5))
            }
        }
    };
    Ok(f2(f132(v7, f129(v0), f33(Adt5::C0))?, v3))
}

pub fn f95(v0: Adt17, v1: Adt2, v2: Adt11, v3: u64) -> R<Adt10> {
    let v7: Adt11 = {
        let v5: Adt11 = {
            let m602 = f128(v1.clone(), v2.clone(), 3u64)?;
            match m602 {
                None => {
                    Adt11::C12
                }
                Some(v4) => {
                    Adt11::C15(Adt28::C3(Adt30::C2, std::rc::Rc::new(f130({
                        let (h603, _) = v4;
                        h603
                    }, 0u64))), std::rc::Rc::new({
                        let (_, h604) = v4;
                        h604
                    }), std::rc::Rc::new(Adt11::C12))
                }
            }
        };
        let m605 = f128(v1, v2, 2u64)?;
        match m605 {
            None => {
                v5
            }
            Some(v6) => {
                Adt11::C15(Adt28::C3(Adt30::C1, std::rc::Rc::new(f130({
                    let (h606, _) = v6;
                    h606
                }, 0u64))), std::rc::Rc::new({
                    let (_, h607) = v6;
                    h607
                }), std::rc::Rc::new(v5))
            }
        }
    };
    Ok(f2(f132(v7, f129(v0), f33(Adt5::C2))?, v3))
}

pub fn f96(v0: Adt17, v1: Adt2, v2: Adt11, v3: u64) -> R<Adt10> {
    let v7: Adt11 = {
        let v5: Adt11 = {
            let m608 = f128(v1.clone(), v2.clone(), 5u64)?;
            match m608 {
                None => {
                    Adt11::C12
                }
                Some(v4) => {
                    Adt11::C15(Adt28::C3(Adt30::C0, std::rc::Rc::new(Adt28::C2(std::rc::Rc::new(Adt28::C6(std::rc::Rc::new(f130({
                        let (h609, _) = v4.clone();
                        h609
                    }, 0u64)), std::rc::Rc::new(Adt28::C6(std::rc::Rc::new(f130({
                        let (h610, _) = v4;
                        h610
                    }, 1u64)), std::rc::Rc::new(Adt28::C5)))))))), std::rc::Rc::new({
                        let (_, h611) = v4;
                        h611
                    }), std::rc::Rc::new(Adt11::C12))
                }
            }
        };
        let m612 = f128(v1, v2, 4u64)?;
        match m612 {
            None => {
                v5
            }
            Some(v6) => {
                Adt11::C15(Adt28::C0(Adt29::C2), std::rc::Rc::new({
                    let (_, h613) = v6;
                    h613
                }), std::rc::Rc::new(v5))
            }
        }
    };
    Ok(f2(Adt11::C8(std::rc::Rc::new(Adt11::C10(Adt27::C2, v0, Adt15::C0({
        let a614 = Str::lit("");
        utf8_encode(a614)
    }, None::<Adt13>))), std::rc::Rc::new(v7), f33(Adt5::C4)), v3))
}

pub fn f97(v0: Adt17, v1: Adt2, v2: Adt11, v3: u64) -> R<Adt10> {
    let m615 = f128(v1, v2, 8u64)?;
    match m615 {
        None => {
            Ok(f2(Adt11::C12, v3))
        }
        Some(v4) => {
            let v5: Adt11 = {
                let (_, h616) = v4.clone();
                h616
            };
            Ok(f2(Adt11::C9(std::rc::Rc::new(Adt11::C11(std::rc::Rc::new(Adt11::C14(Adt28::C2(std::rc::Rc::new(Adt28::C6(std::rc::Rc::new(f130({
                let (h617, _) = v4.clone();
                h617
            }, 0u64)), std::rc::Rc::new(Adt28::C6(std::rc::Rc::new(f130({
                let (h618, _) = v4;
                h618
            }, 1u64)), std::rc::Rc::new(Adt28::C5)))))), false, f32(0u64), std::rc::Rc::new(f129(v0)), f33(Adt5::C8), std::rc::Rc::new(f25(v5.clone())))), std::rc::Rc::new(f26(v5))))), v3))
        }
    }
}

pub fn f98(v0: Adt0, v1: Adt8, v2: Adt0, v3: Adt17, v4: u64, v5: Adt2, v6: Adt11, v7: u64) -> R<Adt10> {
    let v8: Adt10 = f133(v0, v1, v2.clone(), v4, 0u64, f134(v2, v4)?, v5, v6, v7)?;
    Ok(f2(f132(match v8.clone() {
        Adt10::C0(h619, _) => {
            h619
        }
    }, f129(v3), f33(Adt5::C15(0u64)))?, match v8 {
        Adt10::C0(_, h620) => {
            h620
        }
    }))
}

pub fn f99(v0: Adt17, v1: Adt17) -> bool {
    let a621 = f135(v0.clone());
    let a622 = f135(v1.clone());
    if nat_eq(a621, a622) {
        let a623 = f136(v0);
        let a624 = f136(v1);
        nat_eq(a623, a624)
    } else {
        false
    }
}

pub fn f100(v0: Adt0, v1: Adt8, v2: Adt0, v3: Adt5, v4: Adt0, v5: Adt11, v6: Adt15, v7: u64) -> R<Adt10> {
    let v8: Adt34 = f137(v0.clone(), v2, v3, v4.clone())?;
    let v9: Adt10 = f37(v5, match v8.clone() {
        Adt34::C0(h625, _, _) => {
            h625
        }
    }, v6.clone(), v7)?;
    Ok(f2(f39(f25(match v9.clone() {
        Adt10::C0(h626, _) => {
            h626
        }
    }), Adt11::C5(match v8.clone() {
        Adt34::C0(_, h627, _) => {
            h627
        }
    }, std::rc::Rc::new(if match v8.clone() {
        Adt34::C0(_, _, h628) => {
            h628
        }
    } {
        f45(v0, v1, v4, f26(match v9.clone() {
            Adt10::C0(h629, _) => {
                h629
            }
        }), match v8 {
            Adt34::C0(h630, _, _) => {
                h630
            }
        })
    } else {
        f26(match v9.clone() {
            Adt10::C0(h631, _) => {
                h631
            }
        })
    }), v6)), match v9 {
        Adt10::C0(_, h632) => {
            h632
        }
    }))
}

pub fn f101(v0: Adt5) -> Adt12 {
    let m633 = v0;
    match m633 {
        Adt5::C0 => {
            Adt12::C0
        }
        Adt5::C1 => {
            Adt12::C0
        }
        Adt5::C2 => {
            Adt12::C0
        }
        Adt5::C3 => {
            Adt12::C0
        }
        Adt5::C4 => {
            Adt12::C0
        }
        Adt5::C5 => {
            Adt12::C0
        }
        Adt5::C6 => {
            Adt12::C2(0u64)
        }
        Adt5::C7 => {
            Adt12::C0
        }
        Adt5::C8 => {
            Adt12::C0
        }
        Adt5::C9 => {
            Adt12::C1(true)
        }
        Adt5::C10 => {
            Adt12::C1(false)
        }
        Adt5::C11 => {
            Adt12::C0
        }
        Adt5::C12 => {
            Adt12::C7(Adt14::C0)
        }
        Adt5::C13 => {
            Adt12::C7(Adt14::C1)
        }
        Adt5::C14 => {
            Adt12::C7(Adt14::C2)
        }
        Adt5::C15(_) => {
            Adt12::C0
        }
    }
}

pub fn f102(p639: Adt11, p640: u64) -> Adt11 {
    let m638 = (p639, p640);
    {
        let mut m638 = m638;
        loop {
            let step = {
                let (v0, v1) = m638;
                let m634 = v0;
                match m634 {
                    Adt11::C0(_, _) => {
                        Err::<(Adt11, u64), Adt11>(Adt11::C12)
                    }
                    Adt11::C1(_, _, _) => {
                        Err::<(Adt11, u64), Adt11>(Adt11::C12)
                    }
                    Adt11::C2(_, _, _) => {
                        Err::<(Adt11, u64), Adt11>(Adt11::C12)
                    }
                    Adt11::C3(_, _, _, _) => {
                        Err::<(Adt11, u64), Adt11>(Adt11::C12)
                    }
                    Adt11::C4(_, _, _, _) => {
                        Err::<(Adt11, u64), Adt11>(Adt11::C12)
                    }
                    Adt11::C5(_, _, _) => {
                        Err::<(Adt11, u64), Adt11>(Adt11::C12)
                    }
                    Adt11::C6(_, _, _) => {
                        Err::<(Adt11, u64), Adt11>(Adt11::C12)
                    }
                    Adt11::C7(_, _, _, _) => {
                        Err::<(Adt11, u64), Adt11>(Adt11::C12)
                    }
                    Adt11::C8(_, _, _) => {
                        Err::<(Adt11, u64), Adt11>(Adt11::C12)
                    }
                    Adt11::C9(_) => {
                        Err::<(Adt11, u64), Adt11>(Adt11::C12)
                    }
                    Adt11::C10(_, _, _) => {
                        Err::<(Adt11, u64), Adt11>(Adt11::C12)
                    }
                    Adt11::C11(_, _) => {
                        Err::<(Adt11, u64), Adt11>(Adt11::C12)
                    }
                    Adt11::C12 => {
                        Err::<(Adt11, u64), Adt11>(Adt11::C12)
                    }
                    Adt11::C13(r636, r637) => {
                        let v37 = (*r636).clone();
                        let v38 = (*r637).clone();
                        let m635 = v1;
                        if m635 == 0 {
                            Err::<(Adt11, u64), Adt11>(f81(v37))
                        } else {
                            let v39 = m635 - 1;
                            Ok::<(Adt11, u64), Adt11>((v38, v39))
                        }
                    }
                    Adt11::C14(_, _, _, _, _, _) => {
                        Err::<(Adt11, u64), Adt11>(Adt11::C12)
                    }
                    Adt11::C15(_, _, _) => {
                        Err::<(Adt11, u64), Adt11>(Adt11::C12)
                    }
                    Adt11::C16(_, _) => {
                        Err::<(Adt11, u64), Adt11>(Adt11::C12)
                    }
                }
            };
            match step {
                Ok(next) => { m638 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f103(p646: Adt11, p647: Adt0) -> bool {
    let m645 = (p646, p647);
    {
        let mut m645 = m645;
        loop {
            let step = {
                let (v0, v1) = m645;
                let m641 = v0;
                match m641 {
                    Adt11::C0(_, _) => {
                        Err::<(Adt11, Adt0), bool>(false)
                    }
                    Adt11::C1(_, _, _) => {
                        Err::<(Adt11, Adt0), bool>(false)
                    }
                    Adt11::C2(_, _, _) => {
                        Err::<(Adt11, Adt0), bool>(false)
                    }
                    Adt11::C3(_, _, _, _) => {
                        Err::<(Adt11, Adt0), bool>(false)
                    }
                    Adt11::C4(_, _, _, _) => {
                        Err::<(Adt11, Adt0), bool>(false)
                    }
                    Adt11::C5(_, _, _) => {
                        Err::<(Adt11, Adt0), bool>(false)
                    }
                    Adt11::C6(_, _, _) => {
                        Err::<(Adt11, Adt0), bool>(false)
                    }
                    Adt11::C7(_, _, _, _) => {
                        Err::<(Adt11, Adt0), bool>(false)
                    }
                    Adt11::C8(_, _, _) => {
                        Err::<(Adt11, Adt0), bool>(false)
                    }
                    Adt11::C9(_) => {
                        Err::<(Adt11, Adt0), bool>(false)
                    }
                    Adt11::C10(_, _, _) => {
                        Err::<(Adt11, Adt0), bool>(false)
                    }
                    Adt11::C11(_, _) => {
                        Err::<(Adt11, Adt0), bool>(false)
                    }
                    Adt11::C12 => {
                        Err::<(Adt11, Adt0), bool>(false)
                    }
                    Adt11::C13(r643, r644) => {
                        let v37 = (*r643).clone();
                        let v38 = (*r644).clone();
                        if f106(v1.clone()) {
                            if if f21(f107(v1.clone()), Adt0::C0) {
                                let a642 = f122(v37);
                                bool_not(a642)
                            } else {
                                false
                            } {
                                Err::<(Adt11, Adt0), bool>(true)
                            } else {
                                Ok::<(Adt11, Adt0), bool>((v38, f108(v1)))
                            }
                        } else {
                            Err::<(Adt11, Adt0), bool>(false)
                        }
                    }
                    Adt11::C14(_, _, _, _, _, _) => {
                        Err::<(Adt11, Adt0), bool>(false)
                    }
                    Adt11::C15(_, _, _) => {
                        Err::<(Adt11, Adt0), bool>(false)
                    }
                    Adt11::C16(_, _) => {
                        Err::<(Adt11, Adt0), bool>(false)
                    }
                }
            };
            match step {
                Ok(next) => { m645 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f104(v0: Adt11, v1: Adt0, v2: Adt15, v3: u64) -> R<Adt10> {
    let m648 = v0;
    match m648 {
        Adt11::C0(_, _) => {
            Ok(f2(Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(Adt11::C12)), v3))
        }
        Adt11::C1(_, _, _) => {
            Ok(f2(Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(Adt11::C12)), v3))
        }
        Adt11::C2(_, _, _) => {
            Ok(f2(Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(Adt11::C12)), v3))
        }
        Adt11::C3(_, _, _, _) => {
            Ok(f2(Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(Adt11::C12)), v3))
        }
        Adt11::C4(_, _, _, _) => {
            Ok(f2(Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(Adt11::C12)), v3))
        }
        Adt11::C5(_, _, _) => {
            Ok(f2(Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(Adt11::C12)), v3))
        }
        Adt11::C6(_, _, _) => {
            Ok(f2(Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(Adt11::C12)), v3))
        }
        Adt11::C7(_, _, _, _) => {
            Ok(f2(Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(Adt11::C12)), v3))
        }
        Adt11::C8(_, _, _) => {
            Ok(f2(Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(Adt11::C12)), v3))
        }
        Adt11::C9(_) => {
            Ok(f2(Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(Adt11::C12)), v3))
        }
        Adt11::C10(_, _, _) => {
            Ok(f2(Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(Adt11::C12)), v3))
        }
        Adt11::C11(_, _) => {
            Ok(f2(Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(Adt11::C12)), v3))
        }
        Adt11::C12 => {
            Ok(f2(Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(Adt11::C12)), v3))
        }
        Adt11::C13(r658, r659) => {
            let v39 = (*r658).clone();
            let v40 = (*r659).clone();
            if f106(v1.clone()) {
                let v41: bool = f21(f107(v1.clone()), Adt0::C0);
                let v42: Adt10 = f104(v40, f108(v1), v2.clone(), if v41 {
                    v3
                } else {
                    let a649 = v3;
                    let a650 = 1u64;
                    nat_add(a649, a650)?
                })?;
                Ok(f2(Adt11::C11(std::rc::Rc::new(Adt11::C14(if v41 {
                    Adt28::C0(Adt29::C1)
                } else {
                    Adt28::C1(Adt17::C2({
                        let a651 = v3;
                        let a652 = 1u64;
                        nat_add(a651, a652)?
                    }))
                }, false, f32(0u64), std::rc::Rc::new(v39), v2.clone(), std::rc::Rc::new(f25(match v42.clone() {
                    Adt10::C0(h653, _) => {
                        h653
                    }
                })))), std::rc::Rc::new(Adt11::C13(std::rc::Rc::new(if v41 {
                    Adt11::C0(Adt12::C0, v2)
                } else {
                    Adt11::C1(Adt16::C0, Adt17::C2({
                        let a654 = v3;
                        let a655 = 1u64;
                        nat_add(a654, a655)?
                    }), v2)
                }), std::rc::Rc::new(f26(match v42.clone() {
                    Adt10::C0(h656, _) => {
                        h656
                    }
                }))))), match v42 {
                    Adt10::C0(_, h657) => {
                        h657
                    }
                }))
            } else {
                Ok(f2(Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(Adt11::C12)), v3))
            }
        }
        Adt11::C14(_, _, _, _, _, _) => {
            Ok(f2(Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(Adt11::C12)), v3))
        }
        Adt11::C15(_, _, _) => {
            Ok(f2(Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(Adt11::C12)), v3))
        }
        Adt11::C16(_, _) => {
            Ok(f2(Adt11::C11(std::rc::Rc::new(Adt11::C12), std::rc::Rc::new(Adt11::C12)), v3))
        }
    }
}

pub fn f105(v0: Adt11, v1: Adt11) -> Adt11 {
    let m660 = v0;
    match m660 {
        Adt11::C0(_, _) => {
            v1
        }
        Adt11::C1(_, _, _) => {
            v1
        }
        Adt11::C2(_, _, _) => {
            v1
        }
        Adt11::C3(_, _, _, _) => {
            v1
        }
        Adt11::C4(_, _, _, _) => {
            v1
        }
        Adt11::C5(_, _, _) => {
            v1
        }
        Adt11::C6(_, _, _) => {
            v1
        }
        Adt11::C7(_, _, _, _) => {
            v1
        }
        Adt11::C8(_, _, _) => {
            v1
        }
        Adt11::C9(_) => {
            v1
        }
        Adt11::C10(_, _, _) => {
            v1
        }
        Adt11::C11(_, _) => {
            v1
        }
        Adt11::C12 => {
            v1
        }
        Adt11::C13(_, _) => {
            v1
        }
        Adt11::C14(v39, v40, v41, r661, v43, r662) => {
            let v42 = (*r661).clone();
            let v44 = (*r662).clone();
            Adt11::C14(v39, v40, v41, std::rc::Rc::new(v42), v43, std::rc::Rc::new(f105(v44, v1)))
        }
        Adt11::C15(_, _, _) => {
            v1
        }
        Adt11::C16(_, _) => {
            v1
        }
    }
}

pub fn f106(v0: Adt0) -> bool {
    let a663 = f67(v0);
    let a664 = 15u64;
    nat_eq(a663, a664)
}

pub fn f107(v0: Adt0) -> Adt0 {
    f13(v0)
}

pub fn f108(v0: Adt0) -> Adt0 {
    f14(v0)
}

pub fn f109(v0: Adt0, v1: u64, v2: u64, v3: u64) -> u64 {
    let m665 = v0;
    match m665 {
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
            v3
        }
        Adt0::C6 => {
            v2
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
        Adt0::C15(_, _) => {
            v1
        }
    }
}

pub fn f110(v0: Adt6) -> Bytes {
    let m666 = v0;
    match m666 {
        Adt6::C0 => {
            let a667 = Str::lit("nat_add");
            utf8_encode(a667)
        }
        Adt6::C1 => {
            let a668 = Str::lit("nat_sub");
            utf8_encode(a668)
        }
        Adt6::C2 => {
            let a669 = Str::lit("nat_mul");
            utf8_encode(a669)
        }
        Adt6::C3 => {
            let a670 = Str::lit("nat_quot");
            utf8_encode(a670)
        }
        Adt6::C4 => {
            let a671 = Str::lit("nat_rem");
            utf8_encode(a671)
        }
        Adt6::C5 => {
            let a672 = Str::lit("nat_eq");
            utf8_encode(a672)
        }
        Adt6::C6 => {
            let a673 = Str::lit("nat_le");
            utf8_encode(a673)
        }
        Adt6::C7 => {
            let a674 = Str::lit("nat_lt");
            utf8_encode(a674)
        }
        Adt6::C8 => {
            let a675 = Str::lit("int_add");
            utf8_encode(a675)
        }
        Adt6::C9 => {
            let a676 = Str::lit("int_sub");
            utf8_encode(a676)
        }
        Adt6::C10 => {
            let a677 = Str::lit("int_mul");
            utf8_encode(a677)
        }
        Adt6::C11 => {
            let a678 = Str::lit("int_neg");
            utf8_encode(a678)
        }
        Adt6::C12 => {
            let a679 = Str::lit("int_quot");
            utf8_encode(a679)
        }
        Adt6::C13 => {
            let a680 = Str::lit("int_rem");
            utf8_encode(a680)
        }
        Adt6::C14 => {
            let a681 = Str::lit("checked_add");
            utf8_encode(a681)
        }
        Adt6::C15 => {
            let a682 = Str::lit("checked_sub");
            utf8_encode(a682)
        }
        Adt6::C16 => {
            let a683 = Str::lit("checked_mul");
            utf8_encode(a683)
        }
        Adt6::C17 => {
            let a684 = Str::lit("checked_neg");
            utf8_encode(a684)
        }
        Adt6::C18 => {
            let a685 = Str::lit("checked_quot");
            utf8_encode(a685)
        }
        Adt6::C19 => {
            let a686 = Str::lit("bit_and");
            utf8_encode(a686)
        }
        Adt6::C20 => {
            let a687 = Str::lit("bit_or");
            utf8_encode(a687)
        }
        Adt6::C21 => {
            let a688 = Str::lit("bit_xor");
            utf8_encode(a688)
        }
        Adt6::C22 => {
            let a689 = Str::lit("bit_not");
            utf8_encode(a689)
        }
        Adt6::C23 => {
            let a690 = Str::lit("shift_left");
            utf8_encode(a690)
        }
        Adt6::C24 => {
            let a691 = Str::lit("shift_right");
            utf8_encode(a691)
        }
        Adt6::C25 => {
            let a692 = Str::lit("equal");
            utf8_encode(a692)
        }
        Adt6::C26 => {
            let a693 = Str::lit("bool_not");
            utf8_encode(a693)
        }
        Adt6::C27 => {
            let a694 = Str::lit("bool_and");
            utf8_encode(a694)
        }
        Adt6::C28 => {
            let a695 = Str::lit("bool_or");
            utf8_encode(a695)
        }
        Adt6::C29 => {
            let a696 = Str::lit("append");
            utf8_encode(a696)
        }
        Adt6::C30 => {
            let a697 = Str::lit("length");
            utf8_encode(a697)
        }
        Adt6::C31 => {
            let a698 = Str::lit("index");
            utf8_encode(a698)
        }
        Adt6::C32 => {
            let a699 = Str::lit("slice");
            utf8_encode(a699)
        }
        Adt6::C33 => {
            let a700 = Str::lit("utf8_encode");
            utf8_encode(a700)
        }
        Adt6::C34 => {
            let a701 = Str::lit("utf8_decode");
            utf8_encode(a701)
        }
        Adt6::C35 => {
            let a702 = Str::lit("compare_bytes");
            utf8_encode(a702)
        }
        Adt6::C36 => {
            let a703 = Str::lit("split_exact");
            utf8_encode(a703)
        }
        Adt6::C37 => {
            let a704 = Str::lit("join");
            utf8_encode(a704)
        }
        Adt6::C38 => {
            let a705 = Str::lit("format_decimal");
            utf8_encode(a705)
        }
        Adt6::C39 => {
            let a706 = Str::lit("compare");
            utf8_encode(a706)
        }
        Adt6::C40(_) => {
            let a707 = Str::lit("convert");
            utf8_encode(a707)
        }
        Adt6::C41(_) => {
            let a708 = Str::lit("parse_decimal");
            utf8_encode(a708)
        }
    }
}

pub fn f111(v0: Adt0) -> Option<Adt13> {
    let m709 = v0;
    match m709 {
        Adt0::C0 => {
            None::<Adt13>
        }
        Adt0::C1 => {
            None::<Adt13>
        }
        Adt0::C2 => {
            None::<Adt13>
        }
        Adt0::C3 => {
            None::<Adt13>
        }
        Adt0::C4(v1) => {
            Some(f80(v1))
        }
        Adt0::C5 => {
            None::<Adt13>
        }
        Adt0::C6 => {
            None::<Adt13>
        }
        Adt0::C7 => {
            None::<Adt13>
        }
        Adt0::C8(_) => {
            None::<Adt13>
        }
        Adt0::C9(_, _) => {
            None::<Adt13>
        }
        Adt0::C10(_) => {
            None::<Adt13>
        }
        Adt0::C11(_, _) => {
            None::<Adt13>
        }
        Adt0::C12(_) => {
            None::<Adt13>
        }
        Adt0::C13(_, _) => {
            None::<Adt13>
        }
        Adt0::C14 => {
            None::<Adt13>
        }
        Adt0::C15(_, _) => {
            None::<Adt13>
        }
    }
}

pub fn f112(v0: Adt0, v1: Adt8, p714: Adt0) -> bool {
    let m713 = p714;
    {
        let mut m713 = m713;
        loop {
            let step = {
                let v2 = m713;
                let m710 = v2;
                match m710 {
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
                    Adt0::C15(r711, r712) => {
                        let v13 = (*r711).clone();
                        let v14 = (*r712).clone();
                        if f21(v13.clone(), v0.clone()) {
                            Err::<Adt0, bool>(true)
                        } else {
                            if f138(v1.clone(), v13, v0.clone()) {
                                Err::<Adt0, bool>(true)
                            } else {
                                Ok::<Adt0, bool>(v14)
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m713 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f113(v0: Adt0, v1: Adt0, v2: Adt0) -> Adt0 {
    let m715 = v2;
    match m715 {
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
        Adt0::C8(r716) => {
            let v4 = (*r716).clone();
            f113(v0, v1, v4)
        }
        Adt0::C9(r717, r718) => {
            let v5 = (*r717).clone();
            let v6 = (*r718).clone();
            f113(v0.clone(), f113(v0, v1, v5), v6)
        }
        Adt0::C10(_) => {
            v1
        }
        Adt0::C11(r719, r720) => {
            let v8 = (*r719).clone();
            let v9 = (*r720).clone();
            f113(v0.clone(), f113(v0, v1, v8), v9)
        }
        Adt0::C12(v10) => {
            f139(v1, Adt0::C12(v10))
        }
        Adt0::C13(r721, r722) => {
            let v11 = (*r721).clone();
            let v12 = (*r722).clone();
            if f140(v0, Adt0::C13(std::rc::Rc::new(v11.clone()), std::rc::Rc::new(v12.clone()))) {
                f139(v1, Adt0::C13(std::rc::Rc::new(v11), std::rc::Rc::new(v12)))
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

pub fn f114(v0: Adt0, v1: Adt0) -> bool {
    let m723 = v0;
    match m723 {
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
        Adt0::C9(r724, r725) => {
            let v4 = (*r724).clone();
            let v5 = (*r725).clone();
            if f114(v4, v1.clone()) {
                true
            } else {
                f114(v5, v1)
            }
        }
        Adt0::C10(_) => {
            true
        }
        Adt0::C11(r726, r727) => {
            let v7 = (*r726).clone();
            let v8 = (*r727).clone();
            if f114(v7, v1.clone()) {
                f114(v8, v1)
            } else {
                false
            }
        }
        Adt0::C12(v9) => {
            f140(v1, Adt0::C12(v9))
        }
        Adt0::C13(r728, r729) => {
            let v10 = (*r728).clone();
            let v11 = (*r729).clone();
            f140(v1, Adt0::C13(std::rc::Rc::new(v10), std::rc::Rc::new(v11)))
        }
        Adt0::C14 => {
            true
        }
        Adt0::C15(_, _) => {
            true
        }
    }
}

pub fn f115(v0: Adt0) -> Option<Adt13> {
    let m730 = v0;
    match m730 {
        Adt0::C0 => {
            None::<Adt13>
        }
        Adt0::C1 => {
            None::<Adt13>
        }
        Adt0::C2 => {
            None::<Adt13>
        }
        Adt0::C3 => {
            None::<Adt13>
        }
        Adt0::C4(v1) => {
            Some(f80(v1))
        }
        Adt0::C5 => {
            None::<Adt13>
        }
        Adt0::C6 => {
            None::<Adt13>
        }
        Adt0::C7 => {
            None::<Adt13>
        }
        Adt0::C8(_) => {
            None::<Adt13>
        }
        Adt0::C9(_, _) => {
            None::<Adt13>
        }
        Adt0::C10(_) => {
            None::<Adt13>
        }
        Adt0::C11(_, _) => {
            None::<Adt13>
        }
        Adt0::C12(_) => {
            None::<Adt13>
        }
        Adt0::C13(_, _) => {
            None::<Adt13>
        }
        Adt0::C14 => {
            None::<Adt13>
        }
        Adt0::C15(_, _) => {
            None::<Adt13>
        }
    }
}

pub fn f116(v0: Bytes, v1: bool) -> R<Option<(bool, u64)>> {
    let m731 = f141(v0.clone(), 0u64);
    match m731 {
        None => {
            Ok(None::<(bool, u64)>)
        }
        Some(v2) => {
            if if v1 {
                let a732 = v2;
                let a733 = 45u8;
                equal(a732, a733)
            } else {
                false
            } {
                let m734 = f142(v0, 1u64)?;
                match m734 {
                    None => {
                        Ok(None::<(bool, u64)>)
                    }
                    Some(v3) => {
                        Ok(Some((true, v3)))
                    }
                }
            } else {
                let m735 = f142(v0, 0u64)?;
                match m735 {
                    None => {
                        Ok(None::<(bool, u64)>)
                    }
                    Some(v4) => {
                        Ok(Some((false, v4)))
                    }
                }
            }
        }
    }
}

pub fn f117(v0: Adt0) -> Option<Adt1> {
    let m736 = v0;
    match m736 {
        Adt0::C0 => {
            None::<Adt1>
        }
        Adt0::C1 => {
            None::<Adt1>
        }
        Adt0::C2 => {
            None::<Adt1>
        }
        Adt0::C3 => {
            None::<Adt1>
        }
        Adt0::C4(v1) => {
            Some(v1)
        }
        Adt0::C5 => {
            None::<Adt1>
        }
        Adt0::C6 => {
            None::<Adt1>
        }
        Adt0::C7 => {
            None::<Adt1>
        }
        Adt0::C8(_) => {
            None::<Adt1>
        }
        Adt0::C9(_, _) => {
            None::<Adt1>
        }
        Adt0::C10(_) => {
            None::<Adt1>
        }
        Adt0::C11(_, _) => {
            None::<Adt1>
        }
        Adt0::C12(_) => {
            None::<Adt1>
        }
        Adt0::C13(_, _) => {
            None::<Adt1>
        }
        Adt0::C14 => {
            None::<Adt1>
        }
        Adt0::C15(_, _) => {
            None::<Adt1>
        }
    }
}

pub fn f118(v0: Bytes) -> R<Option<(bool, u64)>> {
    let m737 = f116(v0, true)?;
    match m737 {
        None => {
            Ok(None::<(bool, u64)>)
        }
        Some(v1) => {
            Ok(Some(({
                let (h738, _) = v1;
                if h738 {
                    let a740 = 0u64;
                    let a741 = {
                        let (_, h739) = v1;
                        h739
                    };
                    nat_lt(a740, a741)
                } else {
                    false
                }
            }, {
                let (_, h742) = v1;
                h742
            })))
        }
    }
}

pub fn f119(v0: Fn0, p747: u64, p748: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m746 = (p747, p748);
    {
        let mut m746 = m746;
        loop {
            let step = {
                let (v1, v2) = m746;
                let m743 = v1;
                if m743 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m743 - 1;
                    let m745 = {
                        let c744: Fn0 = v0.clone();
                        c744.apply(v2.clone())?
                    };
                    match m745 {
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
                Ok(next) => { m746 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f120(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f143(v0, v1)
}

pub fn f121(p753: List<u64>, v1: u64) -> bool {
    let m752 = p753;
    {
        let mut m752 = m752;
        loop {
            let step = {
                let v0 = m752;
                let m749 = v0;
                match m749.uncons() {
                    None => {
                        Err::<List<u64>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a750 = v2;
                        let a751 = v1;
                        if nat_eq(a750, a751) {
                            Err::<List<u64>, bool>(true)
                        } else {
                            Ok::<List<u64>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m752 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f122(v0: Adt11) -> bool {
    let m754 = v0;
    match m754 {
        Adt11::C0(v1, _) => {
            let m755 = v1;
            match m755 {
                Adt12::C0 => {
                    true
                }
                Adt12::C1(_) => {
                    false
                }
                Adt12::C2(_) => {
                    false
                }
                Adt12::C3(_, _) => {
                    false
                }
                Adt12::C4(_, _, _) => {
                    false
                }
                Adt12::C5(_) => {
                    false
                }
                Adt12::C6(_) => {
                    false
                }
                Adt12::C7(_) => {
                    false
                }
                Adt12::C8 => {
                    false
                }
            }
        }
        Adt11::C1(_, _, _) => {
            false
        }
        Adt11::C2(_, _, _) => {
            false
        }
        Adt11::C3(_, _, _, _) => {
            false
        }
        Adt11::C4(_, _, _, _) => {
            false
        }
        Adt11::C5(_, _, _) => {
            false
        }
        Adt11::C6(_, _, _) => {
            false
        }
        Adt11::C7(_, _, _, _) => {
            false
        }
        Adt11::C8(_, _, _) => {
            false
        }
        Adt11::C9(_) => {
            false
        }
        Adt11::C10(_, _, _) => {
            false
        }
        Adt11::C11(_, _) => {
            false
        }
        Adt11::C12 => {
            false
        }
        Adt11::C13(_, _) => {
            false
        }
        Adt11::C14(_, _, _, _, _, _) => {
            false
        }
        Adt11::C15(_, _, _) => {
            false
        }
        Adt11::C16(_, _) => {
            false
        }
    }
}

pub fn f123(v0: Adt11, v1: Adt11, v2: Adt15) -> Adt11 {
    let m756 = v0;
    match m756 {
        Adt11::C0(_, _) => {
            let a757 = f88(v1.clone());
            let a758 = 1u64;
            if nat_eq(a757, a758) {
                Adt11::C0(Adt12::C1(false), v2)
            } else {
                let a759 = f88(v1.clone());
                let a760 = 2u64;
                if nat_eq(a759, a760) {
                    Adt11::C0(Adt12::C1(true), v2)
                } else {
                    Adt11::C2(Adt18::C0, std::rc::Rc::new(v1), v2)
                }
            }
        }
        Adt11::C1(_, _, _) => {
            Adt11::C2(Adt18::C0, std::rc::Rc::new(v1), v2)
        }
        Adt11::C2(v8, r767, _) => {
            let v9 = (*r767).clone();
            let a761 = f144(v8);
            let a762 = 0u64;
            if nat_eq(a761, a762) {
                v9
            } else {
                Adt11::C2(Adt18::C0, std::rc::Rc::new(v1), v2)
            }
        }
        Adt11::C3(_, _, _, _) => {
            Adt11::C2(Adt18::C0, std::rc::Rc::new(v1), v2)
        }
        Adt11::C4(_, _, _, _) => {
            Adt11::C2(Adt18::C0, std::rc::Rc::new(v1), v2)
        }
        Adt11::C5(_, _, _) => {
            Adt11::C2(Adt18::C0, std::rc::Rc::new(v1), v2)
        }
        Adt11::C6(_, _, _) => {
            Adt11::C2(Adt18::C0, std::rc::Rc::new(v1), v2)
        }
        Adt11::C7(_, _, _, _) => {
            Adt11::C2(Adt18::C0, std::rc::Rc::new(v1), v2)
        }
        Adt11::C8(_, _, _) => {
            Adt11::C2(Adt18::C0, std::rc::Rc::new(v1), v2)
        }
        Adt11::C9(_) => {
            Adt11::C2(Adt18::C0, std::rc::Rc::new(v1), v2)
        }
        Adt11::C10(v33, v34, v35) => {
            let a763 = f145(v33.clone());
            let a764 = 0u64;
            if nat_eq(a763, a764) {
                Adt11::C10(Adt27::C1, v34, v35)
            } else {
                let a765 = f145(v33);
                let a766 = 1u64;
                if nat_eq(a765, a766) {
                    Adt11::C10(Adt27::C0, v34, v35)
                } else {
                    Adt11::C2(Adt18::C0, std::rc::Rc::new(v1), v2)
                }
            }
        }
        Adt11::C11(_, _) => {
            Adt11::C2(Adt18::C0, std::rc::Rc::new(v1), v2)
        }
        Adt11::C12 => {
            Adt11::C2(Adt18::C0, std::rc::Rc::new(v1), v2)
        }
        Adt11::C13(_, _) => {
            Adt11::C2(Adt18::C0, std::rc::Rc::new(v1), v2)
        }
        Adt11::C14(_, _, _, _, _, _) => {
            Adt11::C2(Adt18::C0, std::rc::Rc::new(v1), v2)
        }
        Adt11::C15(_, _, _) => {
            Adt11::C2(Adt18::C0, std::rc::Rc::new(v1), v2)
        }
        Adt11::C16(_, _) => {
            Adt11::C2(Adt18::C0, std::rc::Rc::new(v1), v2)
        }
    }
}

pub fn f124(v0: Adt11, v1: Adt33) -> R<Adt33> {
    let m768 = v0;
    match m768 {
        Adt11::C0(v2, _) => {
            Ok(f146(v2, f147(v1, 1u64, 0u64)))
        }
        Adt11::C1(v4, v5, _) => {
            f148(v5, f147(v1, 2u64, f149(v4)))
        }
        Adt11::C2(v7, r769, _) => {
            let v8 = (*r769).clone();
            f124(v8, f147(v1, 3u64, f144(v7)))
        }
        Adt11::C3(v10, r770, v12, _) => {
            let v11 = (*r770).clone();
            f124(v11, f150(v10, f147(v1, 4u64, if v12 {
                1u64
            } else {
                0u64
            }))?)
        }
        Adt11::C4(v14, r771, v16, _) => {
            let v15 = (*r771).clone();
            f124(v15, f148(v14, f147(v1, 5u64, if v16 {
                1u64
            } else {
                0u64
            }))?)
        }
        Adt11::C5(v18, r772, _) => {
            let v19 = (*r772).clone();
            f124(v19, f151(v18, f147(v1, 6u64, 0u64)))
        }
        Adt11::C6(r773, r774, _) => {
            let v21 = (*r773).clone();
            let v22 = (*r774).clone();
            f124(v22, f124(v21, f147(v1, 7u64, 0u64))?)
        }
        Adt11::C7(r775, r776, r777, _) => {
            let v24 = (*r775).clone();
            let v25 = (*r776).clone();
            let v26 = (*r777).clone();
            f124(v26, f124(v25, f124(v24, f147(v1, 8u64, 0u64))?)?)
        }
        Adt11::C8(r778, r779, _) => {
            let v28 = (*r778).clone();
            let v29 = (*r779).clone();
            f124(v29, f124(v28, f147(v1, 9u64, 0u64))?)
        }
        Adt11::C9(r780) => {
            let v31 = (*r780).clone();
            f124(v31, v1)
        }
        Adt11::C10(v32, v33, _) => {
            f148(v33, f147(v1, 10u64, f145(v32)))
        }
        Adt11::C11(r781, r782) => {
            let v35 = (*r781).clone();
            let v36 = (*r782).clone();
            f124(v36, f147(f124(v35, f147(v1, 11u64, 0u64))?, 12u64, 0u64))
        }
        Adt11::C12 => {
            Ok(f147(v1, 13u64, 0u64))
        }
        Adt11::C13(r783, r784) => {
            let v37 = (*r783).clone();
            let v38 = (*r784).clone();
            f124(v38, f124(v37, f147(v1, 14u64, 0u64))?)
        }
        Adt11::C14(v39, v40, v41, r785, _, r786) => {
            let v42 = (*r785).clone();
            let v44 = (*r786).clone();
            f124(v44, f152(v39, f124(v42, if v40 {
                f153(v41, f147(v1, 15u64, 1u64))
            } else {
                f147(v1, 15u64, 0u64)
            })?)?)
        }
        Adt11::C15(v45, r787, r788) => {
            let v46 = (*r787).clone();
            let v47 = (*r788).clone();
            f124(v47, f124(v46, f152(v45, f147(v1, 16u64, 0u64))?)?)
        }
        Adt11::C16(v48, r789) => {
            let v49 = (*r789).clone();
            f124(v49, f148(v48, f147(v1, 17u64, 0u64))?)
        }
    }
}

pub fn f125(_: u64) -> Adt33 {
    Adt33::C0(List::<(u64, u64)>::nil(), List::<Bytes>::nil(), List::<(Adt17, u64)>::nil(), 0u64)
}

pub fn f126(p802: List<(u64, u64)>, p803: List<(u64, u64)>) -> bool {
    let m801 = (p802, p803);
    {
        let mut m801 = m801;
        loop {
            let step = {
                let (v0, v1) = m801;
                let m790 = v0;
                match m790.uncons() {
                    None => {
                        let m791 = v1;
                        match m791.uncons() {
                            None => {
                                Err::<(List<(u64, u64)>, List<(u64, u64)>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<(u64, u64)>, List<(u64, u64)>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m792 = v1;
                        match m792.uncons() {
                            None => {
                                Err::<(List<(u64, u64)>, List<(u64, u64)>), bool>(false)
                            }
                            Some((v6, v7)) => {
                                let a795 = {
                                    let (h793, _) = v4;
                                    h793
                                };
                                let a796 = {
                                    let (h794, _) = v6;
                                    h794
                                };
                                if if nat_eq(a795, a796) {
                                    let a799 = {
                                        let (_, h797) = v4;
                                        h797
                                    };
                                    let a800 = {
                                        let (_, h798) = v6;
                                        h798
                                    };
                                    nat_eq(a799, a800)
                                } else {
                                    false
                                } {
                                    Ok::<(List<(u64, u64)>, List<(u64, u64)>), bool>((v5, v7))
                                } else {
                                    Err::<(List<(u64, u64)>, List<(u64, u64)>), bool>(false)
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m801 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f127(p810: List<Bytes>, p811: List<Bytes>) -> bool {
    let m809 = (p810, p811);
    {
        let mut m809 = m809;
        loop {
            let step = {
                let (v0, v1) = m809;
                let m804 = v0;
                match m804.uncons() {
                    None => {
                        let m805 = v1;
                        match m805.uncons() {
                            None => {
                                Err::<(List<Bytes>, List<Bytes>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<Bytes>, List<Bytes>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m806 = v1;
                        match m806.uncons() {
                            None => {
                                Err::<(List<Bytes>, List<Bytes>), bool>(false)
                            }
                            Some((v6, v7)) => {
                                let a807 = v4;
                                let a808 = v6;
                                if equal(a807, a808) {
                                    Ok::<(List<Bytes>, List<Bytes>), bool>((v5, v7))
                                } else {
                                    Err::<(List<Bytes>, List<Bytes>), bool>(false)
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m809 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f128(v0: Adt2, v1: Adt11, v2: u64) -> R<Option<(Adt28, Adt11)>> {
    let m812 = v0;
    match m812 {
        Adt2::C0(_, _) => {
            Ok(None::<(Adt28, Adt11)>)
        }
        Adt2::C1(_) => {
            Ok(None::<(Adt28, Adt11)>)
        }
        Adt2::C2(_, _, _, _) => {
            Ok(None::<(Adt28, Adt11)>)
        }
        Adt2::C3(_, _, _) => {
            Ok(None::<(Adt28, Adt11)>)
        }
        Adt2::C4(_, _, _) => {
            Ok(None::<(Adt28, Adt11)>)
        }
        Adt2::C5(_, _, _) => {
            Ok(None::<(Adt28, Adt11)>)
        }
        Adt2::C6(_, _) => {
            Ok(None::<(Adt28, Adt11)>)
        }
        Adt2::C7(_, _) => {
            Ok(None::<(Adt28, Adt11)>)
        }
        Adt2::C8(_, _) => {
            Ok(None::<(Adt28, Adt11)>)
        }
        Adt2::C9(_, _) => {
            Ok(None::<(Adt28, Adt11)>)
        }
        Adt2::C10(_) => {
            Ok(None::<(Adt28, Adt11)>)
        }
        Adt2::C11(_) => {
            Ok(None::<(Adt28, Adt11)>)
        }
        Adt2::C12(_, _) => {
            Ok(None::<(Adt28, Adt11)>)
        }
        Adt2::C13 => {
            Ok(None::<(Adt28, Adt11)>)
        }
        Adt2::C14(r816, r817) => {
            let v31 = (*r816).clone();
            let v32 = (*r817).clone();
            let m813 = f128(v32, f154(v1.clone()), v2)?;
            match m813 {
                None => {
                    let a814 = f155(v31)?;
                    let a815 = v2;
                    if if nat_eq(a814, a815) {
                        f156(f157(v1.clone()))
                    } else {
                        false
                    } {
                        Ok(Some((f158(f157(v1.clone())), f159(f157(v1)))))
                    } else {
                        Ok(None::<(Adt28, Adt11)>)
                    }
                }
                Some(v33) => {
                    Ok(Some(v33))
                }
            }
        }
        Adt2::C15(_, _, _) => {
            Ok(None::<(Adt28, Adt11)>)
        }
    }
}

pub fn f129(v0: Adt17) -> Adt11 {
    Adt11::C1(Adt16::C0, v0, Adt15::C0({
        let a818 = Str::lit("expr:match");
        utf8_encode(a818)
    }, None::<Adt13>))
}

pub fn f130(p825: Adt28, p826: u64) -> Adt28 {
    let m824 = (p825, p826);
    {
        let mut m824 = m824;
        loop {
            let step = {
                let (v0, v1) = m824;
                let m819 = v0;
                match m819 {
                    Adt28::C0(_) => {
                        Err::<(Adt28, u64), Adt28>(Adt28::C0(Adt29::C0))
                    }
                    Adt28::C1(_) => {
                        Err::<(Adt28, u64), Adt28>(Adt28::C0(Adt29::C0))
                    }
                    Adt28::C2(r821) => {
                        let v4 = (*r821).clone();
                        Ok::<(Adt28, u64), Adt28>((v4, v1))
                    }
                    Adt28::C3(_, _) => {
                        Err::<(Adt28, u64), Adt28>(Adt28::C0(Adt29::C0))
                    }
                    Adt28::C4(_, _, _) => {
                        Err::<(Adt28, u64), Adt28>(Adt28::C0(Adt29::C0))
                    }
                    Adt28::C5 => {
                        Err::<(Adt28, u64), Adt28>(Adt28::C0(Adt29::C0))
                    }
                    Adt28::C6(r822, r823) => {
                        let v10 = (*r822).clone();
                        let v11 = (*r823).clone();
                        let m820 = v1;
                        if m820 == 0 {
                            Err::<(Adt28, u64), Adt28>(f160(v10))
                        } else {
                            let v12 = m820 - 1;
                            Ok::<(Adt28, u64), Adt28>((v11, v12))
                        }
                    }
                    Adt28::C7(_) => {
                        Err::<(Adt28, u64), Adt28>(Adt28::C0(Adt29::C0))
                    }
                }
            };
            match step {
                Ok(next) => { m824 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f131(v0: Adt28) -> bool {
    let m827 = v0;
    match m827 {
        Adt28::C0(v1) => {
            let m828 = v1;
            match m828 {
                Adt29::C0 => {
                    true
                }
                Adt29::C1 => {
                    false
                }
                Adt29::C2 => {
                    false
                }
            }
        }
        Adt28::C1(_) => {
            false
        }
        Adt28::C2(_) => {
            false
        }
        Adt28::C3(_, _) => {
            false
        }
        Adt28::C4(_, _, _) => {
            false
        }
        Adt28::C5 => {
            false
        }
        Adt28::C6(_, _) => {
            false
        }
        Adt28::C7(_) => {
            false
        }
    }
}

pub fn f132(v0: Adt11, v1: Adt11, v2: Adt15) -> R<Adt11> {
    let a829 = f30(v0.clone());
    if if bool_not(a829) {
        f161(v0.clone())?
    } else {
        false
    } {
        Ok(v1)
    } else {
        Ok(Adt11::C8(std::rc::Rc::new(v1), std::rc::Rc::new(v0), v2))
    }
}

pub fn f133(v0: Adt0, v1: Adt8, v2: Adt0, v3: u64, v4: u64, v5: u64, v6: Adt2, v7: Adt11, v8: u64) -> R<Adt10> {
    let m830 = v5;
    if m830 == 0 {
        Ok(f2(Adt11::C12, v8))
    } else {
        let v9 = m830 - 1;
        let m833 = f128(v6.clone(), v7.clone(), {
            let a831 = 16u64;
            let a832 = v4;
            nat_add(a831, a832)?
        })?;
        match m833 {
            None => {
                f133(v0, v1, v2, v3, {
                    let a846 = v4;
                    let a847 = 1u64;
                    nat_add(a846, a847)?
                }, v9, v6, v7, v8)
            }
            Some(v10) => {
                let m834 = f55(v2.clone(), v3, v4);
                match m834 {
                    None => {
                        f133(v0, v1, v2, v3, {
                            let a844 = v4;
                            let a845 = 1u64;
                            nat_add(a844, a845)?
                        }, v9, v6, v7, v8)
                    }
                    Some(v11) => {
                        let v12: Adt35 = f162(v0.clone(), v1.clone(), v3, f163({
                            let (h835, _) = v10;
                            h835
                        }), v11, v8)?;
                        let v13: Adt11 = {
                            let (_, h836) = v10;
                            h836
                        };
                        let v14: Adt10 = f133(v0, v1, v2, v3, {
                            let a837 = v4;
                            let a838 = 1u64;
                            nat_add(a837, a838)?
                        }, v9, v6, v7, match v12.clone() {
                            Adt35::C0(_, _, h839) => {
                                h839
                            }
                        })?;
                        Ok(f2(Adt11::C15(Adt28::C4(v3, v4, std::rc::Rc::new(match v12.clone() {
                            Adt35::C0(_, h840, _) => {
                                h840
                            }
                        })), std::rc::Rc::new(Adt11::C11(std::rc::Rc::new(f105(match v12 {
                            Adt35::C0(h841, _, _) => {
                                h841
                            }
                        }, f25(v13.clone()))), std::rc::Rc::new(f26(v13)))), std::rc::Rc::new(match v14.clone() {
                            Adt10::C0(h842, _) => {
                                h842
                            }
                        })), match v14 {
                            Adt10::C0(_, h843) => {
                                h843
                            }
                        }))
                    }
                }
            }
        }
    }
}

pub fn f134(v0: Adt0, v1: u64) -> R<u64> {
    let m848 = f56(v0, v1);
    match m848 {
        None => {
            Ok(0u64)
        }
        Some(v2) => {
            f164(v2)
        }
    }
}

pub fn f135(v0: Adt17) -> u64 {
    let m849 = v0;
    match m849 {
        Adt17::C0(_) => {
            0u64
        }
        Adt17::C1(_) => {
            1u64
        }
        Adt17::C2(_) => {
            2u64
        }
        Adt17::C3(_) => {
            3u64
        }
        Adt17::C4(_) => {
            4u64
        }
        Adt17::C5(_) => {
            5u64
        }
        Adt17::C6(_) => {
            6u64
        }
        Adt17::C7(_) => {
            7u64
        }
        Adt17::C8(_) => {
            8u64
        }
        Adt17::C9(_) => {
            9u64
        }
    }
}

pub fn f136(v0: Adt17) -> u64 {
    let m850 = v0;
    match m850 {
        Adt17::C0(v1) => {
            v1
        }
        Adt17::C1(v2) => {
            v2
        }
        Adt17::C2(v3) => {
            v3
        }
        Adt17::C3(v4) => {
            v4
        }
        Adt17::C4(v5) => {
            v5
        }
        Adt17::C5(v6) => {
            v6
        }
        Adt17::C6(v7) => {
            v7
        }
        Adt17::C7(v8) => {
            v8
        }
        Adt17::C8(v9) => {
            v9
        }
        Adt17::C9(_) => {
            0u64
        }
    }
}

pub fn f137(v0: Adt0, v1: Adt0, v2: Adt5, v3: Adt0) -> R<Adt34> {
    let m851 = v2;
    match m851 {
        Adt5::C0 => {
            Ok(Adt34::C0(Adt0::C14, Adt21::C6, false))
        }
        Adt5::C1 => {
            Ok(Adt34::C0(Adt0::C15(std::rc::Rc::new(f13(v3)), std::rc::Rc::new(Adt0::C14)), Adt21::C1, false))
        }
        Adt5::C2 => {
            Ok(Adt34::C0(Adt0::C15(std::rc::Rc::new(f13(v3.clone())), std::rc::Rc::new(Adt0::C14)), Adt21::C2(f24(v0.clone(), f13(v3.clone()))?, f24(v0, f14(v3))?), false))
        }
        Adt5::C3 => {
            Ok(Adt34::C0(Adt0::C15(std::rc::Rc::new(f14(v3.clone())), std::rc::Rc::new(Adt0::C14)), Adt21::C3(f24(v0.clone(), f13(v3.clone()))?, f24(v0, f14(v3))?), false))
        }
        Adt5::C4 => {
            Ok(Adt34::C0(Adt0::C14, Adt21::C6, false))
        }
        Adt5::C5 => {
            Ok(Adt34::C0(Adt0::C15(std::rc::Rc::new(f13(v3.clone())), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(v3), std::rc::Rc::new(Adt0::C14)))), Adt21::C6, false))
        }
        Adt5::C6 => {
            Ok(Adt34::C0(Adt0::C14, Adt21::C6, false))
        }
        Adt5::C7 => {
            Ok(Adt34::C0(Adt0::C14, Adt21::C6, false))
        }
        Adt5::C8 => {
            Ok(Adt34::C0(Adt0::C14, Adt21::C6, false))
        }
        Adt5::C9 => {
            Ok(Adt34::C0(Adt0::C14, Adt21::C6, false))
        }
        Adt5::C10 => {
            Ok(Adt34::C0(Adt0::C14, Adt21::C6, false))
        }
        Adt5::C11 => {
            Ok(Adt34::C0(Adt0::C14, Adt21::C6, false))
        }
        Adt5::C12 => {
            Ok(Adt34::C0(Adt0::C14, Adt21::C6, false))
        }
        Adt5::C13 => {
            Ok(Adt34::C0(Adt0::C14, Adt21::C6, false))
        }
        Adt5::C14 => {
            Ok(Adt34::C0(Adt0::C14, Adt21::C6, false))
        }
        Adt5::C15(v4) => {
            Ok(Adt34::C0(f165(v1, f76(v3.clone()), v4), Adt21::C4(f76(v3), v4), true))
        }
    }
}

pub fn f138(p855: Adt8, v1: Adt0, v2: Adt0) -> bool {
    let m854 = p855;
    {
        let mut m854 = m854;
        loop {
            let step = {
                let v0 = m854;
                let m852 = v0;
                match m852 {
                    Adt8::C0 => {
                        Err::<Adt8, bool>(false)
                    }
                    Adt8::C1(v3, v4, r853) => {
                        let v5 = (*r853).clone();
                        if f21(v3, v1.clone()) {
                            if f21(v4, v2.clone()) {
                                Err::<Adt8, bool>(true)
                            } else {
                                Ok::<Adt8, bool>(v5)
                            }
                        } else {
                            Ok::<Adt8, bool>(v5)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m854 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f139(v0: Adt0, v1: Adt0) -> Adt0 {
    if f140(v0.clone(), v1.clone()) {
        v0
    } else {
        f166(v0, Adt0::C15(std::rc::Rc::new(v1), std::rc::Rc::new(Adt0::C14)))
    }
}

pub fn f140(p860: Adt0, v1: Adt0) -> bool {
    let m859 = p860;
    {
        let mut m859 = m859;
        loop {
            let step = {
                let v0 = m859;
                let m856 = v0;
                match m856 {
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
                    Adt0::C15(r857, r858) => {
                        let v12 = (*r857).clone();
                        let v13 = (*r858).clone();
                        if f21(v12, v1.clone()) {
                            Err::<Adt0, bool>(true)
                        } else {
                            Ok::<Adt0, bool>(v13)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m859 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f141(v0: Bytes, v1: u64) -> Option<u8> {
    let a861 = v0;
    let a862 = v1;
    index_bytes(a861, a862)
}

pub fn f142(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = {
        let a863 = v0.clone();
        length_bytes(a863)
    };
    let a864 = v1;
    let a865 = v2;
    if nat_lt(a864, a865) {
        let v3: ((u64, u64), bool) = f167(Fn1::F168(v0), {
            let a866 = v2;
            let a867 = v1;
            nat_sub(a866, a867)
        }, (v1, 0u64))?;
        let a870 = {
            let (h868, _) = {
                let (h869, _) = v3;
                h869
            };
            h868
        };
        let a871 = v2;
        if nat_eq(a870, a871) {
            Ok(Some({
                let (_, h872) = {
                    let (h873, _) = v3;
                    h873
                };
                h872
            }))
        } else {
            Ok(None::<u64>)
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f143(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let m875 = f141(v0.clone(), {
        let (h874, _) = v1.clone();
        h874
    });
    match m875 {
        None => {
            Ok(None::<(u64, Bytes)>)
        }
        Some(v2) => {
            let m879 = f141(v0, {
                let a877 = {
                    let (h876, _) = v1.clone();
                    h876
                };
                let a878 = 1u64;
                nat_add(a877, a878)?
            });
            match m879 {
                None => {
                    Ok(None::<(u64, Bytes)>)
                }
                Some(v3) => {
                    let m880 = f169(v2, v3)?;
                    match m880 {
                        None => {
                            Ok(None::<(u64, Bytes)>)
                        }
                        Some(v4) => {
                            Ok(Some(({
                                let a882 = {
                                    let (h881, _) = v1;
                                    h881
                                };
                                let a883 = 2u64;
                                nat_add(a882, a883)?
                            }, {
                                let a885 = {
                                    let (_, h884) = v1;
                                    h884
                                };
                                let a886 = f170(f171(0u64), v4, 1u64);
                                append_bytes(a885, a886)
                            })))
                        }
                    }
                }
            }
        }
    }
}

pub fn f144(v0: Adt18) -> u64 {
    let m887 = v0;
    match m887 {
        Adt18::C0 => {
            0u64
        }
        Adt18::C1 => {
            1u64
        }
        Adt18::C2 => {
            2u64
        }
        Adt18::C3 => {
            3u64
        }
    }
}

pub fn f145(v0: Adt27) -> u64 {
    let m888 = v0;
    match m888 {
        Adt27::C0 => {
            0u64
        }
        Adt27::C1 => {
            1u64
        }
        Adt27::C2 => {
            2u64
        }
        Adt27::C3 => {
            3u64
        }
    }
}

pub fn f146(v0: Adt12, v1: Adt33) -> Adt33 {
    let m889 = v0;
    match m889 {
        Adt12::C0 => {
            f147(v1, 0u64, 0u64)
        }
        Adt12::C1(v2) => {
            f147(v1, 1u64, if v2 {
                1u64
            } else {
                0u64
            })
        }
        Adt12::C2(v3) => {
            f147(v1, 2u64, v3)
        }
        Adt12::C3(v4, v5) => {
            f147(f147(v1, 3u64, if v4 {
                1u64
            } else {
                0u64
            }), 4u64, v5)
        }
        Adt12::C4(v6, v7, v8) => {
            f147(f147(f147(v1, 5u64, f172(v6)), 3u64, if v7 {
                1u64
            } else {
                0u64
            }), 4u64, v8)
        }
        Adt12::C5(v9) => {
            f173(v1, 6u64, v9)
        }
        Adt12::C6(v10) => {
            f173(v1, 7u64, v10)
        }
        Adt12::C7(v11) => {
            f147(v1, 8u64, f174(v11))
        }
        Adt12::C8 => {
            f147(v1, 9u64, 0u64)
        }
    }
}

pub fn f147(v0: Adt33, v1: u64, v2: u64) -> Adt33 {
    Adt33::C0(List::cons((v1, v2), match v0.clone() {
        Adt33::C0(h890, _, _, _) => {
            h890
        }
    }), match v0.clone() {
        Adt33::C0(_, h891, _, _) => {
            h891
        }
    }, match v0.clone() {
        Adt33::C0(_, _, h892, _) => {
            h892
        }
    }, match v0 {
        Adt33::C0(_, _, _, h893) => {
            h893
        }
    })
}

pub fn f148(v0: Adt17, v1: Adt33) -> R<Adt33> {
    let m895 = f175(match v1.clone() {
        Adt33::C0(_, _, h894, _) => {
            h894
        }
    }, v0.clone());
    match m895 {
        None => {
            Ok(f147(v1, {
                let a898 = 60u64;
                let a899 = f135(v0.clone());
                nat_add(a898, a899)?
            }, f136(v0)))
        }
        Some(v2) => {
            Ok(f147(f147(v1, {
                let a896 = 100u64;
                let a897 = f135(v0);
                nat_add(a896, a897)?
            }, v2), 99u64, 0u64))
        }
    }
}

pub fn f149(v0: Adt16) -> u64 {
    let m900 = v0;
    match m900 {
        Adt16::C0 => {
            0u64
        }
        Adt16::C1 => {
            1u64
        }
        Adt16::C2 => {
            0u64
        }
        Adt16::C3 => {
            2u64
        }
        Adt16::C4 => {
            3u64
        }
    }
}

pub fn f150(v0: Adt19, v1: Adt33) -> R<Adt33> {
    let m901 = v0;
    match m901 {
        Adt19::C0(v2) => {
            Ok(f147(v1, 50u64, v2))
        }
        Adt19::C1(v3) => {
            Ok(f147(f147(v1, 51u64, match v3.clone() {
                Adt20::C0(h902, _) => {
                    h902
                }
            }), 52u64, f176(match v3 {
                Adt20::C0(_, h903) => {
                    h903
                }
            })?))
        }
        Adt19::C2(v4, v5) => {
            Ok(f147(f147(v1, 53u64, v4), 54u64, f176(v5)?))
        }
    }
}

pub fn f151(v0: Adt21, v1: Adt33) -> Adt33 {
    let m904 = v0;
    match m904 {
        Adt21::C0(v2) => {
            f153(v2, f147(v1, 40u64, 0u64))
        }
        Adt21::C1 => {
            f147(v1, 41u64, 0u64)
        }
        Adt21::C2(v3, v4) => {
            f153(v4, f153(v3, f147(v1, 42u64, 0u64)))
        }
        Adt21::C3(v5, v6) => {
            f153(v6, f153(v5, f147(v1, 43u64, 0u64)))
        }
        Adt21::C4(v7, v8) => {
            f147(f147(v1, 44u64, v7), 45u64, v8)
        }
        Adt21::C5(v9, v10) => {
            f147(f147(v1, 46u64, v9), 47u64, v10)
        }
        Adt21::C6 => {
            f147(v1, 48u64, 0u64)
        }
        Adt21::C7(v11) => {
            f153(v11, f147(v1, 49u64, 0u64))
        }
    }
}

pub fn f152(v0: Adt28, v1: Adt33) -> R<Adt33> {
    let m905 = v0;
    match m905 {
        Adt28::C0(v2) => {
            Ok(f147(v1, 80u64, f177(v2)))
        }
        Adt28::C1(v3) => {
            Ok(Adt33::C0(List::cons(({
                let a906 = 90u64;
                let a907 = f135(v3.clone());
                nat_add(a906, a907)?
            }, 0u64), match v1.clone() {
                Adt33::C0(h908, _, _, _) => {
                    h908
                }
            }), match v1.clone() {
                Adt33::C0(_, h909, _, _) => {
                    h909
                }
            }, List::cons((v3, match v1.clone() {
                Adt33::C0(_, _, _, h910) => {
                    h910
                }
            }), match v1.clone() {
                Adt33::C0(_, _, h911, _) => {
                    h911
                }
            }), {
                let a913 = match v1 {
                    Adt33::C0(_, _, _, h912) => {
                        h912
                    }
                };
                let a914 = 1u64;
                nat_add(a913, a914)?
            }))
        }
        Adt28::C2(r915) => {
            let v4 = (*r915).clone();
            f152(v4, f147(v1, 81u64, 0u64))
        }
        Adt28::C3(v5, r916) => {
            let v6 = (*r916).clone();
            f152(v6, f147(v1, 82u64, f178(v5)))
        }
        Adt28::C4(v7, v8, r917) => {
            let v9 = (*r917).clone();
            f152(v9, f147(f147(v1, 83u64, v7), 84u64, v8))
        }
        Adt28::C5 => {
            Ok(f147(v1, 85u64, 0u64))
        }
        Adt28::C6(r918, r919) => {
            let v10 = (*r918).clone();
            let v11 = (*r919).clone();
            f152(v11, f152(v10, f147(v1, 86u64, 0u64))?)
        }
        Adt28::C7(v12) => {
            Ok(f147(v1, 87u64, f174(v12)))
        }
    }
}

pub fn f153(v0: Adt22, v1: Adt33) -> Adt33 {
    let m920 = v0;
    match m920 {
        Adt22::C0(v2) => {
            f147(v1, 30u64, f179(v2))
        }
        Adt22::C1(v3) => {
            f147(v1, 31u64, f172(v3))
        }
        Adt22::C2(v4, r923) => {
            let v5 = (*r923).clone();
            f153(v5, f147(v1, 32u64, f180(v4)))
        }
        Adt22::C3(v6, r924, r925) => {
            let v7 = (*r924).clone();
            let v8 = (*r925).clone();
            f153(v8, f153(v7, f147(v1, 33u64, {
                let m921 = v6;
                match m921 {
                    Adt25::C0 => {
                        0u64
                    }
                    Adt25::C1 => {
                        1u64
                    }
                }
            })))
        }
        Adt22::C4(v9, v10) => {
            f147(f147(v1, 34u64, {
                let m922 = v9;
                match m922 {
                    Adt26::C0 => {
                        0u64
                    }
                    Adt26::C1 => {
                        1u64
                    }
                    Adt26::C2 => {
                        2u64
                    }
                }
            }), 35u64, v10)
        }
        Adt22::C5 => {
            f147(v1, 36u64, 0u64)
        }
        Adt22::C6(r926, r927) => {
            let v11 = (*r926).clone();
            let v12 = (*r927).clone();
            f153(v12, f153(v11, f147(v1, 37u64, 0u64)))
        }
    }
}

pub fn f154(v0: Adt11) -> Adt11 {
    let m928 = v0;
    match m928 {
        Adt11::C0(_, _) => {
            Adt11::C12
        }
        Adt11::C1(_, _, _) => {
            Adt11::C12
        }
        Adt11::C2(_, _, _) => {
            Adt11::C12
        }
        Adt11::C3(_, _, _, _) => {
            Adt11::C12
        }
        Adt11::C4(_, _, _, _) => {
            Adt11::C12
        }
        Adt11::C5(_, _, _) => {
            Adt11::C12
        }
        Adt11::C6(_, _, _) => {
            Adt11::C12
        }
        Adt11::C7(_, _, _, _) => {
            Adt11::C12
        }
        Adt11::C8(_, _, _) => {
            Adt11::C12
        }
        Adt11::C9(_) => {
            Adt11::C12
        }
        Adt11::C10(_, _, _) => {
            Adt11::C12
        }
        Adt11::C11(_, _) => {
            Adt11::C12
        }
        Adt11::C12 => {
            Adt11::C12
        }
        Adt11::C13(_, r929) => {
            let v37 = (*r929).clone();
            f81(v37)
        }
        Adt11::C14(_, _, _, _, _, _) => {
            Adt11::C12
        }
        Adt11::C15(_, _, _) => {
            Adt11::C12
        }
        Adt11::C16(_, _) => {
            Adt11::C12
        }
    }
}

pub fn f155(v0: Adt2) -> R<u64> {
    let m930 = v0;
    match m930 {
        Adt2::C0(_, _) => {
            Ok(9999u64)
        }
        Adt2::C1(_) => {
            Ok(9999u64)
        }
        Adt2::C2(_, _, _, _) => {
            Ok(9999u64)
        }
        Adt2::C3(_, _, _) => {
            Ok(9999u64)
        }
        Adt2::C4(_, _, _) => {
            Ok(9999u64)
        }
        Adt2::C5(_, _, _) => {
            Ok(9999u64)
        }
        Adt2::C6(_, _) => {
            Ok(9999u64)
        }
        Adt2::C7(_, _) => {
            Ok(9999u64)
        }
        Adt2::C8(_, _) => {
            Ok(9999u64)
        }
        Adt2::C9(_, _) => {
            Ok(9999u64)
        }
        Adt2::C10(_) => {
            Ok(9999u64)
        }
        Adt2::C11(_) => {
            Ok(9999u64)
        }
        Adt2::C12(_, _) => {
            Ok(9999u64)
        }
        Adt2::C13 => {
            Ok(9999u64)
        }
        Adt2::C14(_, _) => {
            Ok(9999u64)
        }
        Adt2::C15(v31, _, _) => {
            f181(v31)
        }
    }
}

pub fn f156(v0: Adt11) -> bool {
    let m931 = v0;
    match m931 {
        Adt11::C0(_, _) => {
            false
        }
        Adt11::C1(_, _, _) => {
            false
        }
        Adt11::C2(_, _, _) => {
            false
        }
        Adt11::C3(_, _, _, _) => {
            false
        }
        Adt11::C4(_, _, _, _) => {
            false
        }
        Adt11::C5(_, _, _) => {
            false
        }
        Adt11::C6(_, _, _) => {
            false
        }
        Adt11::C7(_, _, _, _) => {
            false
        }
        Adt11::C8(_, _, _) => {
            false
        }
        Adt11::C9(_) => {
            false
        }
        Adt11::C10(_, _, _) => {
            false
        }
        Adt11::C11(_, _) => {
            false
        }
        Adt11::C12 => {
            false
        }
        Adt11::C13(_, _) => {
            false
        }
        Adt11::C14(_, _, _, _, _, _) => {
            false
        }
        Adt11::C15(_, _, _) => {
            true
        }
        Adt11::C16(_, _) => {
            false
        }
    }
}

pub fn f157(v0: Adt11) -> Adt11 {
    let m932 = v0;
    match m932 {
        Adt11::C0(_, _) => {
            Adt11::C12
        }
        Adt11::C1(_, _, _) => {
            Adt11::C12
        }
        Adt11::C2(_, _, _) => {
            Adt11::C12
        }
        Adt11::C3(_, _, _, _) => {
            Adt11::C12
        }
        Adt11::C4(_, _, _, _) => {
            Adt11::C12
        }
        Adt11::C5(_, _, _) => {
            Adt11::C12
        }
        Adt11::C6(_, _, _) => {
            Adt11::C12
        }
        Adt11::C7(_, _, _, _) => {
            Adt11::C12
        }
        Adt11::C8(_, _, _) => {
            Adt11::C12
        }
        Adt11::C9(_) => {
            Adt11::C12
        }
        Adt11::C10(_, _, _) => {
            Adt11::C12
        }
        Adt11::C11(_, _) => {
            Adt11::C12
        }
        Adt11::C12 => {
            Adt11::C12
        }
        Adt11::C13(r933, _) => {
            let v36 = (*r933).clone();
            f81(v36)
        }
        Adt11::C14(_, _, _, _, _, _) => {
            Adt11::C12
        }
        Adt11::C15(_, _, _) => {
            Adt11::C12
        }
        Adt11::C16(_, _) => {
            Adt11::C12
        }
    }
}

pub fn f158(v0: Adt11) -> Adt28 {
    let m934 = v0;
    match m934 {
        Adt11::C0(_, _) => {
            Adt28::C5
        }
        Adt11::C1(_, _, _) => {
            Adt28::C5
        }
        Adt11::C2(_, _, _) => {
            Adt28::C5
        }
        Adt11::C3(_, _, _, _) => {
            Adt28::C5
        }
        Adt11::C4(_, _, _, _) => {
            Adt28::C5
        }
        Adt11::C5(_, _, _) => {
            Adt28::C5
        }
        Adt11::C6(_, _, _) => {
            Adt28::C5
        }
        Adt11::C7(_, _, _, _) => {
            Adt28::C5
        }
        Adt11::C8(_, _, _) => {
            Adt28::C5
        }
        Adt11::C9(_) => {
            Adt28::C5
        }
        Adt11::C10(_, _, _) => {
            Adt28::C5
        }
        Adt11::C11(_, _) => {
            Adt28::C5
        }
        Adt11::C12 => {
            Adt28::C5
        }
        Adt11::C13(_, _) => {
            Adt28::C5
        }
        Adt11::C14(_, _, _, _, _, _) => {
            Adt28::C5
        }
        Adt11::C15(v44, _, _) => {
            f160(v44)
        }
        Adt11::C16(_, _) => {
            Adt28::C5
        }
    }
}

pub fn f159(v0: Adt11) -> Adt11 {
    let m935 = v0;
    match m935 {
        Adt11::C0(_, _) => {
            Adt11::C12
        }
        Adt11::C1(_, _, _) => {
            Adt11::C12
        }
        Adt11::C2(_, _, _) => {
            Adt11::C12
        }
        Adt11::C3(_, _, _, _) => {
            Adt11::C12
        }
        Adt11::C4(_, _, _, _) => {
            Adt11::C12
        }
        Adt11::C5(_, _, _) => {
            Adt11::C12
        }
        Adt11::C6(_, _, _) => {
            Adt11::C12
        }
        Adt11::C7(_, _, _, _) => {
            Adt11::C12
        }
        Adt11::C8(_, _, _) => {
            Adt11::C12
        }
        Adt11::C9(_) => {
            Adt11::C12
        }
        Adt11::C10(_, _, _) => {
            Adt11::C12
        }
        Adt11::C11(_, _) => {
            Adt11::C12
        }
        Adt11::C12 => {
            Adt11::C12
        }
        Adt11::C13(_, _) => {
            Adt11::C12
        }
        Adt11::C14(_, _, _, _, _, _) => {
            Adt11::C12
        }
        Adt11::C15(_, r936, _) => {
            let v45 = (*r936).clone();
            f81(v45)
        }
        Adt11::C16(_, _) => {
            Adt11::C12
        }
    }
}

pub fn f160(v0: Adt28) -> Adt28 {
    v0
}

pub fn f161(p941: Adt11) -> R<bool> {
    let m940 = p941;
    {
        let mut m940 = m940;
        loop {
            let step = {
                let v0 = m940;
                let m937 = v0;
                match m937 {
                    Adt11::C0(_, _) => {
                        Err::<Adt11, R<bool>>(Ok(false))
                    }
                    Adt11::C1(_, _, _) => {
                        Err::<Adt11, R<bool>>(Ok(false))
                    }
                    Adt11::C2(_, _, _) => {
                        Err::<Adt11, R<bool>>(Ok(false))
                    }
                    Adt11::C3(_, _, _, _) => {
                        Err::<Adt11, R<bool>>(Ok(false))
                    }
                    Adt11::C4(_, _, _, _) => {
                        Err::<Adt11, R<bool>>(Ok(false))
                    }
                    Adt11::C5(_, _, _) => {
                        Err::<Adt11, R<bool>>(Ok(false))
                    }
                    Adt11::C6(_, _, _) => {
                        Err::<Adt11, R<bool>>(Ok(false))
                    }
                    Adt11::C7(_, _, _, _) => {
                        Err::<Adt11, R<bool>>(Ok(false))
                    }
                    Adt11::C8(_, _, _) => {
                        Err::<Adt11, R<bool>>(Ok(false))
                    }
                    Adt11::C9(_) => {
                        Err::<Adt11, R<bool>>(Ok(false))
                    }
                    Adt11::C10(_, _, _) => {
                        Err::<Adt11, R<bool>>(Ok(false))
                    }
                    Adt11::C11(_, _) => {
                        Err::<Adt11, R<bool>>(Ok(false))
                    }
                    Adt11::C12 => {
                        Err::<Adt11, R<bool>>(Ok(true))
                    }
                    Adt11::C13(_, _) => {
                        Err::<Adt11, R<bool>>(Ok(false))
                    }
                    Adt11::C14(_, _, _, _, _, _) => {
                        Err::<Adt11, R<bool>>(Ok(false))
                    }
                    Adt11::C15(v44, r938, r939) => {
                        let v45 = (*r938).clone();
                        let v46 = (*r939).clone();
                        if f182(v44, v45)? {
                            Ok::<Adt11, R<bool>>(v46)
                        } else {
                            Err::<Adt11, R<bool>>(Ok(false))
                        }
                    }
                    Adt11::C16(_, _) => {
                        Err::<Adt11, R<bool>>(Ok(false))
                    }
                }
            };
            match step {
                Ok(next) => { m940 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f162(v0: Adt0, v1: Adt8, v2: u64, v3: Adt28, v4: Adt0, v5: u64) -> R<Adt35> {
    let m942 = v3;
    match m942 {
        Adt28::C0(_) => {
            Ok(Adt35::C0(Adt11::C12, Adt28::C5, v5))
        }
        Adt28::C1(_) => {
            Ok(Adt35::C0(Adt11::C12, Adt28::C5, v5))
        }
        Adt28::C2(_) => {
            Ok(Adt35::C0(Adt11::C12, Adt28::C5, v5))
        }
        Adt28::C3(_, _) => {
            Ok(Adt35::C0(Adt11::C12, Adt28::C5, v5))
        }
        Adt28::C4(_, _, _) => {
            Ok(Adt35::C0(Adt11::C12, Adt28::C5, v5))
        }
        Adt28::C5 => {
            Ok(Adt35::C0(Adt11::C12, Adt28::C5, v5))
        }
        Adt28::C6(r953, r954) => {
            let v14 = (*r953).clone();
            let v15 = (*r954).clone();
            if f106(v4.clone()) {
                let v16: bool = if f131(v14.clone()) {
                    false
                } else {
                    f57(v0.clone(), v1.clone(), Adt0::C12(v2), f107(v4.clone()))
                };
                let v17: Adt17 = Adt17::C4({
                    let a943 = v5;
                    let a944 = 1u64;
                    nat_add(a943, a944)?
                });
                let v18: Adt35 = f162(v0, v1, v2, v15, f108(v4), if v16 {
                    let a945 = v5;
                    let a946 = 1u64;
                    nat_add(a945, a946)?
                } else {
                    v5
                })?;
                Ok(Adt35::C0(if v16 {
                    Adt11::C14(v14.clone(), false, f32(0u64), std::rc::Rc::new(Adt11::C1(Adt16::C4, v17.clone(), Adt15::C0({
                        let a947 = Str::lit("indirection");
                        utf8_encode(a947)
                    }, None::<Adt13>))), Adt15::C0({
                        let a948 = Str::lit("indirection");
                        utf8_encode(a948)
                    }, None::<Adt13>), std::rc::Rc::new(match v18.clone() {
                        Adt35::C0(h949, _, _) => {
                            h949
                        }
                    }))
                } else {
                    match v18.clone() {
                        Adt35::C0(h950, _, _) => {
                            h950
                        }
                    }
                }, Adt28::C6(std::rc::Rc::new(if v16 {
                    Adt28::C1(v17)
                } else {
                    v14
                }), std::rc::Rc::new(match v18.clone() {
                    Adt35::C0(_, h951, _) => {
                        h951
                    }
                })), match v18 {
                    Adt35::C0(_, _, h952) => {
                        h952
                    }
                }))
            } else {
                Ok(Adt35::C0(Adt11::C12, Adt28::C5, v5))
            }
        }
        Adt28::C7(_) => {
            Ok(Adt35::C0(Adt11::C12, Adt28::C5, v5))
        }
    }
}

pub fn f163(v0: Adt28) -> Adt28 {
    let m955 = v0;
    match m955 {
        Adt28::C0(_) => {
            Adt28::C5
        }
        Adt28::C1(_) => {
            Adt28::C5
        }
        Adt28::C2(r956) => {
            let v3 = (*r956).clone();
            f160(v3)
        }
        Adt28::C3(_, _) => {
            Adt28::C5
        }
        Adt28::C4(_, _, _) => {
            Adt28::C5
        }
        Adt28::C5 => {
            Adt28::C5
        }
        Adt28::C6(_, _) => {
            Adt28::C5
        }
        Adt28::C7(_) => {
            Adt28::C5
        }
    }
}

pub fn f164(v0: Adt0) -> R<u64> {
    let m957 = v0;
    match m957 {
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
        Adt0::C15(_, r960) => {
            let v12 = (*r960).clone();
            let v13: u64 = f164(v12)?;
            let a958 = 1u64;
            let a959 = v13;
            nat_add(a958, a959)
        }
    }
}

pub fn f165(v0: Adt0, v1: u64, v2: u64) -> Adt0 {
    let m961 = f55(v0, v1, v2);
    match m961 {
        None => {
            Adt0::C14
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f166(v0: Adt0, v1: Adt0) -> Adt0 {
    let m962 = v0;
    match m962 {
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
        Adt0::C15(r963, r964) => {
            let v12 = (*r963).clone();
            let v13 = (*r964).clone();
            Adt0::C15(std::rc::Rc::new(v12), std::rc::Rc::new(f166(v13, v1)))
        }
    }
}

pub fn f167(v0: Fn1, p969: u64, p970: (u64, u64)) -> R<((u64, u64), bool)> {
    let m968 = (p969, p970);
    {
        let mut m968 = m968;
        loop {
            let step = {
                let (v1, v2) = m968;
                let m965 = v1;
                if m965 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m965 - 1;
                    let m967 = {
                        let c966: Fn1 = v0.clone();
                        c966.apply(v2)?
                    };
                    match m967 {
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
                Ok(next) => { m968 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f168(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f183(v0, v1)
}

pub fn f169(v0: u8, v1: u8) -> R<Option<u64>> {
    let m971 = f184(v0)?;
    match m971 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            let m972 = f184(v1)?;
            match m972 {
                None => {
                    Ok(None::<u64>)
                }
                Some(v3) => {
                    Ok(Some({
                        let a975 = {
                            let a973 = v2;
                            let a974 = 16u64;
                            nat_mul(a973, a974)?
                        };
                        let a976 = v3;
                        nat_add(a975, a976)?
                    }))
                }
            }
        }
    }
}

pub fn f170(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m980 = {
        let a977 = v0;
        let a978 = v1;
        let a979 = v2;
        slice_bytes(a977, a978, a979)
    };
    match m980 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f171(_: u64) -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f172(v0: Adt13) -> u64 {
    let m981 = v0;
    match m981 {
        Adt13::C0 => {
            0u64
        }
        Adt13::C1 => {
            1u64
        }
        Adt13::C2 => {
            2u64
        }
        Adt13::C3 => {
            3u64
        }
        Adt13::C4 => {
            4u64
        }
        Adt13::C5 => {
            5u64
        }
        Adt13::C6 => {
            6u64
        }
        Adt13::C7 => {
            7u64
        }
    }
}

pub fn f173(v0: Adt33, v1: u64, v2: Bytes) -> Adt33 {
    Adt33::C0(List::cons((v1, {
        let a982 = v2.clone();
        length_bytes(a982)
    }), match v0.clone() {
        Adt33::C0(h983, _, _, _) => {
            h983
        }
    }), List::cons(v2, match v0.clone() {
        Adt33::C0(_, h984, _, _) => {
            h984
        }
    }), match v0.clone() {
        Adt33::C0(_, _, h985, _) => {
            h985
        }
    }, match v0 {
        Adt33::C0(_, _, _, h986) => {
            h986
        }
    })
}

pub fn f174(v0: Adt14) -> u64 {
    let m987 = v0;
    match m987 {
        Adt14::C0 => {
            0u64
        }
        Adt14::C1 => {
            1u64
        }
        Adt14::C2 => {
            2u64
        }
    }
}

pub fn f175(p992: List<(Adt17, u64)>, v1: Adt17) -> Option<u64> {
    let m991 = p992;
    {
        let mut m991 = m991;
        loop {
            let step = {
                let v0 = m991;
                let m988 = v0;
                match m988.uncons() {
                    None => {
                        Err::<List<(Adt17, u64)>, Option<u64>>(None::<u64>)
                    }
                    Some((v2, v3)) => {
                        if f99({
                            let (h989, _) = v2;
                            h989
                        }, v1.clone()) {
                            Err::<List<(Adt17, u64)>, Option<u64>>(Some({
                                let (_, h990) = v2;
                                h990
                            }))
                        } else {
                            Ok::<List<(Adt17, u64)>, Option<u64>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m991 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f176(v0: Option<Adt13>) -> R<u64> {
    let m993 = v0;
    match m993 {
        None => {
            Ok(0u64)
        }
        Some(v1) => {
            let a994 = 1u64;
            let a995 = f172(v1);
            nat_add(a994, a995)
        }
    }
}

pub fn f177(v0: Adt29) -> u64 {
    let m996 = v0;
    match m996 {
        Adt29::C0 => {
            0u64
        }
        Adt29::C1 => {
            1u64
        }
        Adt29::C2 => {
            2u64
        }
    }
}

pub fn f178(v0: Adt30) -> u64 {
    let m997 = v0;
    match m997 {
        Adt30::C0 => {
            0u64
        }
        Adt30::C1 => {
            1u64
        }
        Adt30::C2 => {
            2u64
        }
    }
}

pub fn f179(v0: Adt23) -> u64 {
    let m998 = v0;
    match m998 {
        Adt23::C0 => {
            0u64
        }
        Adt23::C1 => {
            1u64
        }
        Adt23::C2 => {
            2u64
        }
        Adt23::C3 => {
            3u64
        }
        Adt23::C4 => {
            4u64
        }
        Adt23::C5 => {
            5u64
        }
        Adt23::C6 => {
            6u64
        }
        Adt23::C7 => {
            7u64
        }
        Adt23::C8 => {
            8u64
        }
    }
}

pub fn f180(v0: Adt24) -> u64 {
    let m999 = v0;
    match m999 {
        Adt24::C0 => {
            0u64
        }
        Adt24::C1 => {
            1u64
        }
        Adt24::C2 => {
            2u64
        }
        Adt24::C3 => {
            3u64
        }
        Adt24::C4 => {
            4u64
        }
    }
}

pub fn f181(v0: Adt5) -> R<u64> {
    let m1000 = v0;
    match m1000 {
        Adt5::C0 => {
            Ok(0u64)
        }
        Adt5::C1 => {
            Ok(1u64)
        }
        Adt5::C2 => {
            Ok(2u64)
        }
        Adt5::C3 => {
            Ok(3u64)
        }
        Adt5::C4 => {
            Ok(4u64)
        }
        Adt5::C5 => {
            Ok(5u64)
        }
        Adt5::C6 => {
            Ok(6u64)
        }
        Adt5::C7 => {
            Ok(7u64)
        }
        Adt5::C8 => {
            Ok(8u64)
        }
        Adt5::C9 => {
            Ok(9u64)
        }
        Adt5::C10 => {
            Ok(10u64)
        }
        Adt5::C11 => {
            Ok(11u64)
        }
        Adt5::C12 => {
            Ok(12u64)
        }
        Adt5::C13 => {
            Ok(13u64)
        }
        Adt5::C14 => {
            Ok(14u64)
        }
        Adt5::C15(v1) => {
            let a1001 = 16u64;
            let a1002 = v1;
            nat_add(a1001, a1002)
        }
    }
}

pub fn f182(v0: Adt28, v1: Adt11) -> R<bool> {
    if f30(f25(v1.clone())) {
        f185(v0, f26(v1))
    } else {
        Ok(false)
    }
}

pub fn f183(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m1004 = f141(v0, {
        let (h1003, _) = v1;
        h1003
    });
    match m1004 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            let m1005 = f186(v2)?;
            match m1005 {
                None => {
                    Ok(None::<(u64, u64)>)
                }
                Some(v3) => {
                    Ok(Some(({
                        let a1007 = {
                            let (h1006, _) = v1;
                            h1006
                        };
                        let a1008 = 1u64;
                        nat_add(a1007, a1008)?
                    }, {
                        let a1012 = {
                            let a1010 = {
                                let (_, h1009) = v1;
                                h1009
                            };
                            let a1011 = 10u64;
                            nat_mul(a1010, a1011)?
                        };
                        let a1013 = v3;
                        nat_add(a1012, a1013)?
                    })))
                }
            }
        }
    }
}

pub fn f184(v0: u8) -> R<Option<u64>> {
    let m1014 = f186(v0)?;
    match m1014 {
        None => {
            let v3: u64 = {
                let v2: u8 = v0;
                let a1059 = {
                    let a1035 = {
                        let a1023 = {
                            let a1017 = {
                                let a1015 = v2;
                                let a1016 = 128u8;
                                fixed_u8::bit_and(a1015, a1016)
                            };
                            let a1018 = 128u8;
                            if equal(a1017, a1018) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a1024 = {
                            let a1021 = {
                                let a1019 = v2;
                                let a1020 = 64u8;
                                fixed_u8::bit_and(a1019, a1020)
                            };
                            let a1022 = 64u8;
                            if equal(a1021, a1022) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1023, a1024)?
                    };
                    let a1036 = {
                        let a1033 = {
                            let a1027 = {
                                let a1025 = v2;
                                let a1026 = 32u8;
                                fixed_u8::bit_and(a1025, a1026)
                            };
                            let a1028 = 32u8;
                            if equal(a1027, a1028) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a1034 = {
                            let a1031 = {
                                let a1029 = v2;
                                let a1030 = 16u8;
                                fixed_u8::bit_and(a1029, a1030)
                            };
                            let a1032 = 16u8;
                            if equal(a1031, a1032) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1033, a1034)?
                    };
                    nat_add(a1035, a1036)?
                };
                let a1060 = {
                    let a1057 = {
                        let a1045 = {
                            let a1039 = {
                                let a1037 = v2;
                                let a1038 = 8u8;
                                fixed_u8::bit_and(a1037, a1038)
                            };
                            let a1040 = 8u8;
                            if equal(a1039, a1040) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a1046 = {
                            let a1043 = {
                                let a1041 = v2;
                                let a1042 = 4u8;
                                fixed_u8::bit_and(a1041, a1042)
                            };
                            let a1044 = 4u8;
                            if equal(a1043, a1044) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1045, a1046)?
                    };
                    let a1058 = {
                        let a1055 = {
                            let a1049 = {
                                let a1047 = v2;
                                let a1048 = 2u8;
                                fixed_u8::bit_and(a1047, a1048)
                            };
                            let a1050 = 2u8;
                            if equal(a1049, a1050) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a1056 = {
                            let a1053 = {
                                let a1051 = v2;
                                let a1052 = 1u8;
                                fixed_u8::bit_and(a1051, a1052)
                            };
                            let a1054 = 1u8;
                            if equal(a1053, a1054) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1055, a1056)?
                    };
                    nat_add(a1057, a1058)?
                };
                nat_add(a1059, a1060)?
            };
            let a1061 = v3;
            let a1062 = 97u64;
            if nat_lt(a1061, a1062) {
                Ok(None::<u64>)
            } else {
                let a1063 = v3;
                let a1064 = 103u64;
                if nat_lt(a1063, a1064) {
                    Ok(Some({
                        let a1065 = v3;
                        let a1066 = 87u64;
                        nat_sub(a1065, a1066)
                    }))
                } else {
                    Ok(None::<u64>)
                }
            }
        }
        Some(v1) => {
            Ok(Some(v1))
        }
    }
}

pub fn f185(v0: Adt28, v1: Adt11) -> R<bool> {
    let m1067 = v0;
    match m1067 {
        Adt28::C0(v2) => {
            let a1068 = f177(v2);
            let a1069 = 2u64;
            if nat_eq(a1068, a1069) {
                let a1070 = f187(v1);
                let a1071 = 0u64;
                Ok(nat_eq(a1070, a1071))
            } else {
                Ok(false)
            }
        }
        Adt28::C1(_) => {
            Ok(false)
        }
        Adt28::C2(_) => {
            Ok(false)
        }
        Adt28::C3(v5, r1076) => {
            let v6 = (*r1076).clone();
            let a1074 = f187(v1.clone());
            let a1075 = {
                let a1072 = 1u64;
                let a1073 = f178(v5);
                nat_add(a1072, a1073)?
            };
            if nat_eq(a1074, a1075) {
                Ok(f188(v6, f189(v1)))
            } else {
                Ok(false)
            }
        }
        Adt28::C4(v7, v8, r1077) => {
            let v9 = (*r1077).clone();
            if f190(v1.clone(), v7, v8) {
                Ok(f191(v9, f189(v1)))
            } else {
                Ok(false)
            }
        }
        Adt28::C5 => {
            Ok(false)
        }
        Adt28::C6(_, _) => {
            Ok(false)
        }
        Adt28::C7(v12) => {
            Ok(f192(v1, v12))
        }
    }
}

pub fn f186(v0: u8) -> R<Option<u64>> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a1122 = {
            let a1098 = {
                let a1086 = {
                    let a1080 = {
                        let a1078 = v1;
                        let a1079 = 128u8;
                        fixed_u8::bit_and(a1078, a1079)
                    };
                    let a1081 = 128u8;
                    if equal(a1080, a1081) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a1087 = {
                    let a1084 = {
                        let a1082 = v1;
                        let a1083 = 64u8;
                        fixed_u8::bit_and(a1082, a1083)
                    };
                    let a1085 = 64u8;
                    if equal(a1084, a1085) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a1086, a1087)?
            };
            let a1099 = {
                let a1096 = {
                    let a1090 = {
                        let a1088 = v1;
                        let a1089 = 32u8;
                        fixed_u8::bit_and(a1088, a1089)
                    };
                    let a1091 = 32u8;
                    if equal(a1090, a1091) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a1097 = {
                    let a1094 = {
                        let a1092 = v1;
                        let a1093 = 16u8;
                        fixed_u8::bit_and(a1092, a1093)
                    };
                    let a1095 = 16u8;
                    if equal(a1094, a1095) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a1096, a1097)?
            };
            nat_add(a1098, a1099)?
        };
        let a1123 = {
            let a1120 = {
                let a1108 = {
                    let a1102 = {
                        let a1100 = v1;
                        let a1101 = 8u8;
                        fixed_u8::bit_and(a1100, a1101)
                    };
                    let a1103 = 8u8;
                    if equal(a1102, a1103) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a1109 = {
                    let a1106 = {
                        let a1104 = v1;
                        let a1105 = 4u8;
                        fixed_u8::bit_and(a1104, a1105)
                    };
                    let a1107 = 4u8;
                    if equal(a1106, a1107) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a1108, a1109)?
            };
            let a1121 = {
                let a1118 = {
                    let a1112 = {
                        let a1110 = v1;
                        let a1111 = 2u8;
                        fixed_u8::bit_and(a1110, a1111)
                    };
                    let a1113 = 2u8;
                    if equal(a1112, a1113) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a1119 = {
                    let a1116 = {
                        let a1114 = v1;
                        let a1115 = 1u8;
                        fixed_u8::bit_and(a1114, a1115)
                    };
                    let a1117 = 1u8;
                    if equal(a1116, a1117) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a1118, a1119)?
            };
            nat_add(a1120, a1121)?
        };
        nat_add(a1122, a1123)?
    };
    let a1124 = v2;
    let a1125 = 48u64;
    if nat_lt(a1124, a1125) {
        Ok(None::<u64>)
    } else {
        let a1126 = v2;
        let a1127 = 58u64;
        if nat_lt(a1126, a1127) {
            Ok(Some({
                let a1128 = v2;
                let a1129 = 48u64;
                nat_sub(a1128, a1129)
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f187(v0: Adt11) -> u64 {
    let m1130 = v0;
    match m1130 {
        Adt11::C0(_, _) => {
            99u64
        }
        Adt11::C1(_, _, _) => {
            99u64
        }
        Adt11::C2(_, _, _) => {
            99u64
        }
        Adt11::C3(_, _, _, _) => {
            99u64
        }
        Adt11::C4(_, _, _, _) => {
            99u64
        }
        Adt11::C5(v17, _, _) => {
            f193(v17)
        }
        Adt11::C6(_, _, _) => {
            99u64
        }
        Adt11::C7(_, _, _, _) => {
            99u64
        }
        Adt11::C8(_, _, _) => {
            99u64
        }
        Adt11::C9(_) => {
            99u64
        }
        Adt11::C10(_, _, _) => {
            99u64
        }
        Adt11::C11(_, _) => {
            99u64
        }
        Adt11::C12 => {
            99u64
        }
        Adt11::C13(_, _) => {
            99u64
        }
        Adt11::C14(_, _, _, _, _, _) => {
            99u64
        }
        Adt11::C15(_, _, _) => {
            99u64
        }
        Adt11::C16(_, _) => {
            99u64
        }
    }
}

pub fn f188(v0: Adt28, v1: Adt11) -> bool {
    let m1131 = v1;
    match m1131 {
        Adt11::C0(_, _) => {
            false
        }
        Adt11::C1(_, _, _) => {
            false
        }
        Adt11::C2(_, _, _) => {
            false
        }
        Adt11::C3(_, _, _, _) => {
            false
        }
        Adt11::C4(_, _, _, _) => {
            false
        }
        Adt11::C5(_, _, _) => {
            false
        }
        Adt11::C6(_, _, _) => {
            false
        }
        Adt11::C7(_, _, _, _) => {
            false
        }
        Adt11::C8(_, _, _) => {
            false
        }
        Adt11::C9(_) => {
            false
        }
        Adt11::C10(_, _, _) => {
            false
        }
        Adt11::C11(_, _) => {
            false
        }
        Adt11::C12 => {
            false
        }
        Adt11::C13(r1132, r1133) => {
            let v37 = (*r1132).clone();
            let v38 = (*r1133).clone();
            if f30(v38) {
                f194(v0, v37)
            } else {
                false
            }
        }
        Adt11::C14(_, _, _, _, _, _) => {
            false
        }
        Adt11::C15(_, _, _) => {
            false
        }
        Adt11::C16(_, _) => {
            false
        }
    }
}

pub fn f189(v0: Adt11) -> Adt11 {
    let m1134 = v0;
    match m1134 {
        Adt11::C0(_, _) => {
            Adt11::C12
        }
        Adt11::C1(_, _, _) => {
            Adt11::C12
        }
        Adt11::C2(_, _, _) => {
            Adt11::C12
        }
        Adt11::C3(_, _, _, _) => {
            Adt11::C12
        }
        Adt11::C4(_, _, _, _) => {
            Adt11::C12
        }
        Adt11::C5(_, r1135, _) => {
            let v18 = (*r1135).clone();
            f81(v18)
        }
        Adt11::C6(_, _, _) => {
            Adt11::C12
        }
        Adt11::C7(_, _, _, _) => {
            Adt11::C12
        }
        Adt11::C8(_, _, _) => {
            Adt11::C12
        }
        Adt11::C9(_) => {
            Adt11::C12
        }
        Adt11::C10(_, _, _) => {
            Adt11::C12
        }
        Adt11::C11(_, _) => {
            Adt11::C12
        }
        Adt11::C12 => {
            Adt11::C12
        }
        Adt11::C13(_, _) => {
            Adt11::C12
        }
        Adt11::C14(_, _, _, _, _, _) => {
            Adt11::C12
        }
        Adt11::C15(_, _, _) => {
            Adt11::C12
        }
        Adt11::C16(_, _) => {
            Adt11::C12
        }
    }
}

pub fn f190(v0: Adt11, v1: u64, v2: u64) -> bool {
    let m1136 = v0;
    match m1136 {
        Adt11::C0(_, _) => {
            false
        }
        Adt11::C1(_, _, _) => {
            false
        }
        Adt11::C2(_, _, _) => {
            false
        }
        Adt11::C3(_, _, _, _) => {
            false
        }
        Adt11::C4(_, _, _, _) => {
            false
        }
        Adt11::C5(v19, _, _) => {
            let m1137 = v19;
            match m1137 {
                Adt21::C0(_) => {
                    false
                }
                Adt21::C1 => {
                    false
                }
                Adt21::C2(_, _) => {
                    false
                }
                Adt21::C3(_, _) => {
                    false
                }
                Adt21::C4(v27, v28) => {
                    let a1138 = v1;
                    let a1139 = v27;
                    if nat_eq(a1138, a1139) {
                        let a1140 = v2;
                        let a1141 = v28;
                        nat_eq(a1140, a1141)
                    } else {
                        false
                    }
                }
                Adt21::C5(_, _) => {
                    false
                }
                Adt21::C6 => {
                    false
                }
                Adt21::C7(_) => {
                    false
                }
            }
        }
        Adt11::C6(_, _, _) => {
            false
        }
        Adt11::C7(_, _, _, _) => {
            false
        }
        Adt11::C8(_, _, _) => {
            false
        }
        Adt11::C9(_) => {
            false
        }
        Adt11::C10(_, _, _) => {
            false
        }
        Adt11::C11(_, _) => {
            false
        }
        Adt11::C12 => {
            false
        }
        Adt11::C13(_, _) => {
            false
        }
        Adt11::C14(_, _, _, _, _, _) => {
            false
        }
        Adt11::C15(_, _, _) => {
            false
        }
        Adt11::C16(_, _) => {
            false
        }
    }
}

pub fn f191(p1149: Adt28, p1150: Adt11) -> bool {
    let m1148 = (p1149, p1150);
    {
        let mut m1148 = m1148;
        loop {
            let step = {
                let (v0, v1) = m1148;
                let m1142 = v0;
                match m1142 {
                    Adt28::C0(_) => {
                        Err::<(Adt28, Adt11), bool>(false)
                    }
                    Adt28::C1(_) => {
                        Err::<(Adt28, Adt11), bool>(false)
                    }
                    Adt28::C2(_) => {
                        Err::<(Adt28, Adt11), bool>(false)
                    }
                    Adt28::C3(_, _) => {
                        Err::<(Adt28, Adt11), bool>(false)
                    }
                    Adt28::C4(_, _, _) => {
                        Err::<(Adt28, Adt11), bool>(false)
                    }
                    Adt28::C5 => {
                        Err::<(Adt28, Adt11), bool>(f30(v1))
                    }
                    Adt28::C6(r1146, r1147) => {
                        let v10 = (*r1146).clone();
                        let v11 = (*r1147).clone();
                        let m1143 = v1;
                        match m1143 {
                            Adt11::C0(_, _) => {
                                Err::<(Adt28, Adt11), bool>(false)
                            }
                            Adt11::C1(_, _, _) => {
                                Err::<(Adt28, Adt11), bool>(false)
                            }
                            Adt11::C2(_, _, _) => {
                                Err::<(Adt28, Adt11), bool>(false)
                            }
                            Adt11::C3(_, _, _, _) => {
                                Err::<(Adt28, Adt11), bool>(false)
                            }
                            Adt11::C4(_, _, _, _) => {
                                Err::<(Adt28, Adt11), bool>(false)
                            }
                            Adt11::C5(_, _, _) => {
                                Err::<(Adt28, Adt11), bool>(false)
                            }
                            Adt11::C6(_, _, _) => {
                                Err::<(Adt28, Adt11), bool>(false)
                            }
                            Adt11::C7(_, _, _, _) => {
                                Err::<(Adt28, Adt11), bool>(false)
                            }
                            Adt11::C8(_, _, _) => {
                                Err::<(Adt28, Adt11), bool>(false)
                            }
                            Adt11::C9(_) => {
                                Err::<(Adt28, Adt11), bool>(false)
                            }
                            Adt11::C10(_, _, _) => {
                                Err::<(Adt28, Adt11), bool>(false)
                            }
                            Adt11::C11(_, _) => {
                                Err::<(Adt28, Adt11), bool>(false)
                            }
                            Adt11::C12 => {
                                Err::<(Adt28, Adt11), bool>(false)
                            }
                            Adt11::C13(r1144, r1145) => {
                                let v47 = (*r1144).clone();
                                let v48 = (*r1145).clone();
                                if f194(v10, v47) {
                                    Ok::<(Adt28, Adt11), bool>((v11, v48))
                                } else {
                                    Err::<(Adt28, Adt11), bool>(false)
                                }
                            }
                            Adt11::C14(_, _, _, _, _, _) => {
                                Err::<(Adt28, Adt11), bool>(false)
                            }
                            Adt11::C15(_, _, _) => {
                                Err::<(Adt28, Adt11), bool>(false)
                            }
                            Adt11::C16(_, _) => {
                                Err::<(Adt28, Adt11), bool>(false)
                            }
                        }
                    }
                    Adt28::C7(_) => {
                        Err::<(Adt28, Adt11), bool>(false)
                    }
                }
            };
            match step {
                Ok(next) => { m1148 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f192(v0: Adt11, v1: Adt14) -> bool {
    let m1151 = v0;
    match m1151 {
        Adt11::C0(v2, _) => {
            let m1152 = v2;
            match m1152 {
                Adt12::C0 => {
                    false
                }
                Adt12::C1(_) => {
                    false
                }
                Adt12::C2(_) => {
                    false
                }
                Adt12::C3(_, _) => {
                    false
                }
                Adt12::C4(_, _, _) => {
                    false
                }
                Adt12::C5(_) => {
                    false
                }
                Adt12::C6(_) => {
                    false
                }
                Adt12::C7(v13) => {
                    f195(v1, v13)
                }
                Adt12::C8 => {
                    false
                }
            }
        }
        Adt11::C1(_, _, _) => {
            false
        }
        Adt11::C2(_, _, _) => {
            false
        }
        Adt11::C3(_, _, _, _) => {
            false
        }
        Adt11::C4(_, _, _, _) => {
            false
        }
        Adt11::C5(_, _, _) => {
            false
        }
        Adt11::C6(_, _, _) => {
            false
        }
        Adt11::C7(_, _, _, _) => {
            false
        }
        Adt11::C8(_, _, _) => {
            false
        }
        Adt11::C9(_) => {
            false
        }
        Adt11::C10(_, _, _) => {
            false
        }
        Adt11::C11(_, _) => {
            false
        }
        Adt11::C12 => {
            false
        }
        Adt11::C13(_, _) => {
            false
        }
        Adt11::C14(_, _, _, _, _, _) => {
            false
        }
        Adt11::C15(_, _, _) => {
            false
        }
        Adt11::C16(_, _) => {
            false
        }
    }
}

pub fn f193(v0: Adt21) -> u64 {
    let m1153 = v0;
    match m1153 {
        Adt21::C0(_) => {
            0u64
        }
        Adt21::C1 => {
            1u64
        }
        Adt21::C2(_, _) => {
            2u64
        }
        Adt21::C3(_, _) => {
            3u64
        }
        Adt21::C4(_, _) => {
            4u64
        }
        Adt21::C5(_, _) => {
            5u64
        }
        Adt21::C6 => {
            6u64
        }
        Adt21::C7(_) => {
            7u64
        }
    }
}

pub fn f194(v0: Adt28, v1: Adt11) -> bool {
    let m1154 = v0;
    match m1154 {
        Adt28::C0(v2) => {
            let a1155 = f177(v2);
            let a1156 = 0u64;
            if nat_eq(a1155, a1156) {
                f122(v1)
            } else {
                false
            }
        }
        Adt28::C1(v3) => {
            let m1157 = v1;
            match m1157 {
                Adt11::C0(_, _) => {
                    false
                }
                Adt11::C1(v6, v7, _) => {
                    let a1158 = f149(v6);
                    let a1159 = 0u64;
                    if nat_eq(a1158, a1159) {
                        f99(v3, v7)
                    } else {
                        false
                    }
                }
                Adt11::C2(_, _, _) => {
                    false
                }
                Adt11::C3(_, _, _, _) => {
                    false
                }
                Adt11::C4(_, _, _, _) => {
                    false
                }
                Adt11::C5(_, _, _) => {
                    false
                }
                Adt11::C6(_, _, _) => {
                    false
                }
                Adt11::C7(_, _, _, _) => {
                    false
                }
                Adt11::C8(_, _, _) => {
                    false
                }
                Adt11::C9(_) => {
                    false
                }
                Adt11::C10(_, _, _) => {
                    false
                }
                Adt11::C11(_, _) => {
                    false
                }
                Adt11::C12 => {
                    false
                }
                Adt11::C13(_, _) => {
                    false
                }
                Adt11::C14(_, _, _, _, _, _) => {
                    false
                }
                Adt11::C15(_, _, _) => {
                    false
                }
                Adt11::C16(_, _) => {
                    false
                }
            }
        }
        Adt28::C2(_) => {
            false
        }
        Adt28::C3(_, _) => {
            false
        }
        Adt28::C4(_, _, _) => {
            false
        }
        Adt28::C5 => {
            false
        }
        Adt28::C6(_, _) => {
            false
        }
        Adt28::C7(_) => {
            false
        }
    }
}

pub fn f195(v0: Adt14, v1: Adt14) -> bool {
    let a1160 = f174(v0);
    let a1161 = f174(v1);
    nat_eq(a1160, a1161)
}

pub fn run(p0: Bytes, p1: Adt0, p2: List<List<u64>>, p3: Adt0, p4: Adt0, p5: Adt2, p6: Adt0, p7: Adt0, p8: List<bool>, p9: List<bool>, p10: Adt7, p11: u64) -> R<Adt10> {
    f0(p0, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11)
}
