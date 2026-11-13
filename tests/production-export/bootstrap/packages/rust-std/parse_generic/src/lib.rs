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
    C0(bool),
    C1(bool, u64),
    C2(Bytes),
    C3(std::rc::Rc<Adt0>),
    C4(std::rc::Rc<Adt0>),
    C5,
    C6(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
    C7(Bytes, std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(u64, Bytes),
}

#[derive(Clone)]
pub enum Adt2 {
    C0(Adt3, u64, u64),
}

#[derive(Clone)]
pub enum Adt3 {
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
pub enum Adt4 {
    C0(u64, List<Adt2>, Option<Adt1>),
}

#[derive(Clone)]
pub enum Adt5 {
    C0(u64, u64, u64, List<Adt6>, Adt0, Option<Adt1>),
}

#[derive(Clone)]
pub enum Adt6 {
    C0(List<Adt0>),
    C1(Bytes, u64, List<(Bytes, Adt0)>),
}

#[derive(Clone)]
pub enum Adt7 {
    C0(u64, List<Bytes>, bool, Option<Adt1>),
}

#[derive(Clone)]
pub enum Adt8 {
    C0(u64, List<Bytes>),
}

#[derive(Clone)]
pub enum Fn0 {
    F4(u64, Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: Adt4) -> R<Option<Adt4>> {
        match self {
            Fn0::F4(k0, k1) => f4(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F12,
}

impl Fn1 {
    pub fn apply(&self, p0: List<Adt2>, p1: Adt2) -> List<Adt2> {
        match self {
            Fn1::F12 => f12(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F14(u64, List<Adt2>),
}

impl Fn2 {
    pub fn apply(&self, p0: Adt5) -> R<Option<Adt5>> {
        match self {
            Fn2::F14(k0, k1) => f14(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F35(u64, Bytes),
}

impl Fn3 {
    pub fn apply(&self, p0: Adt7) -> R<Option<Adt7>> {
        match self {
            Fn3::F35(k0, k1) => f35(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F39(u64, Bytes),
    F85(u64, Bytes),
    F113(Bytes),
    F114(Bytes),
    F118(Bytes),
    F137(Bytes),
}

impl Fn4 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn4::F39(k0, k1) => f39(*k0, k1.clone(), p0),
            Fn4::F85(k0, k1) => f85(*k0, k1.clone(), p0),
            Fn4::F113(k0) => f113(k0.clone(), p0),
            Fn4::F114(k0) => f114(k0.clone(), p0),
            Fn4::F118(k0) => f118(k0.clone(), p0),
            Fn4::F137(k0) => f137(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F47,
}

impl Fn5 {
    pub fn apply(&self, p0: List<Bytes>) -> Option<List<Bytes>> {
        match self {
            Fn5::F47 => f47(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F49,
}

impl Fn6 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn6::F49 => f49(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F80(Bytes),
}

impl Fn7 {
    pub fn apply(&self, p0: bool, p1: (Bytes, Adt0)) -> bool {
        match self {
            Fn7::F80(k0) => f80(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F87,
}

impl Fn8 {
    pub fn apply(&self, p0: (Option<Bytes>, List<Bytes>), p1: Bytes) -> (Option<Bytes>, List<Bytes>) {
        match self {
            Fn8::F87 => f87(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F99,
}

impl Fn9 {
    pub fn apply(&self, p0: Adt0, p1: Adt0) -> Adt0 {
        match self {
            Fn9::F99 => f99(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F101,
}

impl Fn10 {
    pub fn apply(&self, p0: Adt0, p1: (Bytes, Adt0)) -> Adt0 {
        match self {
            Fn10::F101 => f101(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F105,
}

impl Fn11 {
    pub fn apply(&self, p0: List<List<(Bytes, Adt0)>>, p1: (Bytes, Adt0)) -> List<List<(Bytes, Adt0)>> {
        match self {
            Fn11::F105 => f105(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F107,
}

impl Fn12 {
    pub fn apply(&self, p0: List<List<(Bytes, Adt0)>>) -> R<Option<List<List<(Bytes, Adt0)>>>> {
        match self {
            Fn12::F107 => f107(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F117(Bytes),
}

impl Fn13 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn13::F117(k0) => f117(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F120(u64, Bytes),
}

impl Fn14 {
    pub fn apply(&self, p0: Adt8) -> R<Option<Adt8>> {
        match self {
            Fn14::F120(k0, k1) => f120(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F122,
}

impl Fn15 {
    pub fn apply(&self, p0: List<(Bytes, Adt0)>, p1: (Bytes, Adt0)) -> List<(Bytes, Adt0)> {
        match self {
            Fn15::F122 => f122(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F124,
}

impl Fn16 {
    pub fn apply(&self, p0: List<List<(Bytes, Adt0)>>, p1: List<(Bytes, Adt0)>) -> List<List<(Bytes, Adt0)>> {
        match self {
            Fn16::F124 => f124(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F142,
}

impl Fn17 {
    pub fn apply(&self, p0: (List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>)) -> R<Option<(List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>)>> {
        match self {
            Fn17::F142 => f142(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F148,
}

impl Fn18 {
    pub fn apply(&self, p0: (List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>))) -> Option<(List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>))> {
        match self {
            Fn18::F148 => f148(p0),
        }
    }
}

pub fn f0(v0: Bytes) -> R<Result<Adt0, Adt1>> {
    let m1 = f1(v0.clone())?;
    match m1 {
        Ok(v1) => {
            f2(v1, {
                let a2 = v0;
                length_bytes(a2)
            })
        }
        Err(v2) => {
            Ok(Err::<Adt0, Adt1>(v2))
        }
    }
}

pub fn f1(v0: Bytes) -> R<Result<List<Adt2>, Adt1>> {
    let v1: u64 = {
        let a3 = v0.clone();
        length_bytes(a3)
    };
    let v2: Adt4 = {
        let (h4, _) = f3(Fn0::F4(v1, v0), {
            let a5 = v1;
            let a6 = 1u64;
            nat_add(a5, a6)?
        }, Adt4::C0(0u64, List::<Adt2>::nil(), None::<Adt1>))?;
        h4
    };
    let m8 = match v2.clone() {
        Adt4::C0(_, _, h7) => {
            h7
        }
    };
    match m8 {
        None => {
            Ok(Ok::<List<Adt2>, Adt1>(f5(match v2 {
                Adt4::C0(_, h9, _) => {
                    h9
                }
            })))
        }
        Some(v3) => {
            Ok(Err::<List<Adt2>, Adt1>(v3))
        }
    }
}

pub fn f2(v0: List<Adt2>, v1: u64) -> R<Result<Adt0, Adt1>> {
    let v2: Adt5 = f6(v0.clone(), v1, 0u64, 0u64)?;
    let m11 = match v2.clone() {
        Adt5::C0(_, _, _, _, _, h10) => {
            h10
        }
    };
    match m11 {
        None => {
            let a13 = match v2.clone() {
                Adt5::C0(_, _, h12, _, _, _) => {
                    h12
                }
            };
            let a14 = 9u64;
            if nat_eq(a13, a14) {
                let a17 = match v2.clone() {
                    Adt5::C0(h15, _, _, _, _, _) => {
                        h15
                    }
                };
                let a18 = {
                    let a16 = v0.clone();
                    length_list(a16)
                };
                if nat_lt(a17, a18) {
                    Ok(Err::<Adt0, Adt1>(f7(f8(v0, v1, match v2 {
                        Adt5::C0(h19, _, _, _, _, _) => {
                            h19
                        }
                    }), f9(Str::lit("unexpected text after the value")))))
                } else {
                    Ok(Ok::<Adt0, Adt1>(match v2 {
                        Adt5::C0(_, _, _, _, h20, _) => {
                            h20
                        }
                    }))
                }
            } else {
                Ok(Err::<Adt0, Adt1>(f7(v1, f9(Str::lit("the reader ran out of steps")))))
            }
        }
        Some(v3) => {
            Ok(Err::<Adt0, Adt1>(v3))
        }
    }
}

pub fn f3(v0: Fn0, p25: u64, p26: Adt4) -> R<(Adt4, bool)> {
    let m24 = (p25, p26);
    {
        let mut m24 = m24;
        loop {
            let step = {
                let (v1, v2) = m24;
                let m21 = v1;
                if m21 == 0 {
                    Err::<(u64, Adt4), R<(Adt4, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m21 - 1;
                    let m23 = {
                        let c22: Fn0 = v0.clone();
                        c22.apply(v2.clone())?
                    };
                    match m23 {
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
                Ok(next) => { m24 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f4(v0: u64, v1: Bytes, v2: Adt4) -> R<Option<Adt4>> {
    f10(v1, v0, v2)
}

pub fn f5(v0: List<Adt2>) -> List<Adt2> {
    f11(Fn1::F12, List::<Adt2>::nil(), v0)
}

pub fn f6(v0: List<Adt2>, v1: u64, v2: u64, v3: u64) -> R<Adt5> {
    let (h27, _) = f13(Fn2::F14(v1, v0.clone()), {
        let a31 = {
            let a29 = {
                let a28 = v0;
                length_list(a28)
            };
            let a30 = 4u64;
            nat_mul(a29, a30)?
        };
        let a32 = 8u64;
        nat_add(a31, a32)?
    }, Adt5::C0(v2, v3, 0u64, List::<Adt6>::nil(), Adt0::C5, None::<Adt1>))?;
    Ok(h27)
}

pub fn f7(v0: u64, v1: Bytes) -> Adt1 {
    Adt1::C0(v0, v1)
}

pub fn f8(v0: List<Adt2>, v1: u64, v2: u64) -> u64 {
    let m33 = f15(v0, v2);
    match m33 {
        None => {
            v1
        }
        Some(v3) => {
            match v3 {
                Adt2::C0(_, h34, _) => {
                    h34
                }
            }
        }
    }
}

pub fn f9(v0: Str) -> Bytes {
    let a35 = v0;
    utf8_encode(a35)
}

pub fn f10(v0: Bytes, v1: u64, v2: Adt4) -> R<Option<Adt4>> {
    let m37 = match v2.clone() {
        Adt4::C0(_, _, h36) => {
            h36
        }
    };
    match m37 {
        None => {
            let v4: u64 = match v2.clone() {
                Adt4::C0(h38, _, _) => {
                    h38
                }
            };
            let a39 = v4;
            let a40 = v1;
            if nat_lt(a39, a40) {
                let v5: u64 = f16(v0.clone(), v4)?;
                let a41 = 0u64;
                let a42 = v5;
                if nat_lt(a41, a42) {
                    Ok(Some(Adt4::C0({
                        let a43 = v4;
                        let a44 = v5;
                        nat_add(a43, a44)?
                    }, match v2 {
                        Adt4::C0(_, h45, _) => {
                            h45
                        }
                    }, None::<Adt1>)))
                } else {
                    if f17(v0.clone(), v4, 123u8) {
                        Ok(f18(v2, Adt3::C0, {
                            let a46 = v4;
                            let a47 = 1u64;
                            nat_add(a46, a47)?
                        }))
                    } else {
                        if f17(v0.clone(), v4, 125u8) {
                            Ok(f18(v2, Adt3::C1, {
                                let a48 = v4;
                                let a49 = 1u64;
                                nat_add(a48, a49)?
                            }))
                        } else {
                            if f17(v0.clone(), v4, 91u8) {
                                Ok(f18(v2, Adt3::C2, {
                                    let a50 = v4;
                                    let a51 = 1u64;
                                    nat_add(a50, a51)?
                                }))
                            } else {
                                if f17(v0.clone(), v4, 93u8) {
                                    Ok(f18(v2, Adt3::C3, {
                                        let a52 = v4;
                                        let a53 = 1u64;
                                        nat_add(a52, a53)?
                                    }))
                                } else {
                                    if f17(v0.clone(), v4, 40u8) {
                                        Ok(f18(v2, Adt3::C4, {
                                            let a54 = v4;
                                            let a55 = 1u64;
                                            nat_add(a54, a55)?
                                        }))
                                    } else {
                                        if f17(v0.clone(), v4, 41u8) {
                                            Ok(f18(v2, Adt3::C5, {
                                                let a56 = v4;
                                                let a57 = 1u64;
                                                nat_add(a56, a57)?
                                            }))
                                        } else {
                                            if f17(v0.clone(), v4, 44u8) {
                                                Ok(f18(v2, Adt3::C6, {
                                                    let a58 = v4;
                                                    let a59 = 1u64;
                                                    nat_add(a58, a59)?
                                                }))
                                            } else {
                                                if f17(v0.clone(), v4, 59u8) {
                                                    Ok(f18(v2, Adt3::C10, {
                                                        let a60 = v4;
                                                        let a61 = 1u64;
                                                        nat_add(a60, a61)?
                                                    }))
                                                } else {
                                                    if f17(v0.clone(), v4, 58u8) {
                                                        f19(v0, v2)
                                                    } else {
                                                        if f17(v0.clone(), v4, 34u8) {
                                                            f20(v0, v1, v2)
                                                        } else {
                                                            f21(v0, v1, v2)
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
                Ok(None::<Adt4>)
            }
        }
        Some(_) => {
            Ok(None::<Adt4>)
        }
    }
}

pub fn f11(v0: Fn1, p65: List<Adt2>, p66: List<Adt2>) -> List<Adt2> {
    let m64 = (p65, p66);
    {
        let mut m64 = m64;
        loop {
            let step = {
                let (v1, v2) = m64;
                let m62 = v2;
                match m62.uncons() {
                    None => {
                        Err::<(List<Adt2>, List<Adt2>), List<Adt2>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt2>, List<Adt2>), List<Adt2>>(({
                            let c63: Fn1 = v0.clone();
                            c63.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m64 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f12(v0: List<Adt2>, v1: Adt2) -> List<Adt2> {
    List::cons(v1, v0)
}

pub fn f13(v0: Fn2, p71: u64, p72: Adt5) -> R<(Adt5, bool)> {
    let m70 = (p71, p72);
    {
        let mut m70 = m70;
        loop {
            let step = {
                let (v1, v2) = m70;
                let m67 = v1;
                if m67 == 0 {
                    Err::<(u64, Adt5), R<(Adt5, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m67 - 1;
                    let m69 = {
                        let c68: Fn2 = v0.clone();
                        c68.apply(v2.clone())?
                    };
                    match m69 {
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
                Ok(next) => { m70 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f14(v0: u64, v1: List<Adt2>, v2: Adt5) -> R<Option<Adt5>> {
    f22(v1, v0, v2)
}

pub fn f15(v0: List<Adt2>, v1: u64) -> Option<Adt2> {
    let a73 = v0;
    let a74 = v1;
    index_list(a73, a74)
}

pub fn f16(v0: Bytes, v1: u64) -> R<u64> {
    if if f23(v0.clone(), v1, 9u8, 13u8) {
        true
    } else {
        f24(v0.clone(), v1, 32u8)
    } {
        Ok(1u64)
    } else {
        if f24(v0.clone(), v1, 194u8) {
            if if f24(v0.clone(), {
                let a75 = v1;
                let a76 = 1u64;
                nat_add(a75, a76)?
            }, 133u8) {
                true
            } else {
                f24(v0, {
                    let a77 = v1;
                    let a78 = 1u64;
                    nat_add(a77, a78)?
                }, 160u8)
            } {
                Ok(2u64)
            } else {
                Ok(0u64)
            }
        } else {
            if f24(v0.clone(), v1, 225u8) {
                if if f24(v0.clone(), {
                    let a79 = v1;
                    let a80 = 1u64;
                    nat_add(a79, a80)?
                }, 154u8) {
                    f24(v0, {
                        let a81 = v1;
                        let a82 = 2u64;
                        nat_add(a81, a82)?
                    }, 128u8)
                } else {
                    false
                } {
                    Ok(3u64)
                } else {
                    Ok(0u64)
                }
            } else {
                if f24(v0.clone(), v1, 226u8) {
                    if f24(v0.clone(), {
                        let a83 = v1;
                        let a84 = 1u64;
                        nat_add(a83, a84)?
                    }, 128u8) {
                        if if f23(v0.clone(), {
                            let a85 = v1;
                            let a86 = 2u64;
                            nat_add(a85, a86)?
                        }, 128u8, 138u8) {
                            true
                        } else {
                            if f24(v0.clone(), {
                                let a87 = v1;
                                let a88 = 2u64;
                                nat_add(a87, a88)?
                            }, 168u8) {
                                true
                            } else {
                                if f24(v0.clone(), {
                                    let a89 = v1;
                                    let a90 = 2u64;
                                    nat_add(a89, a90)?
                                }, 169u8) {
                                    true
                                } else {
                                    f24(v0, {
                                        let a91 = v1;
                                        let a92 = 2u64;
                                        nat_add(a91, a92)?
                                    }, 175u8)
                                }
                            }
                        } {
                            Ok(3u64)
                        } else {
                            Ok(0u64)
                        }
                    } else {
                        if if f24(v0.clone(), {
                            let a93 = v1;
                            let a94 = 1u64;
                            nat_add(a93, a94)?
                        }, 129u8) {
                            f24(v0, {
                                let a95 = v1;
                                let a96 = 2u64;
                                nat_add(a95, a96)?
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
                    if f24(v0.clone(), v1, 227u8) {
                        if if f24(v0.clone(), {
                            let a97 = v1;
                            let a98 = 1u64;
                            nat_add(a97, a98)?
                        }, 128u8) {
                            f24(v0, {
                                let a99 = v1;
                                let a100 = 2u64;
                                nat_add(a99, a100)?
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

pub fn f17(v0: Bytes, v1: u64, v2: u8) -> bool {
    let m101 = f25(v0, v1);
    match m101 {
        None => {
            false
        }
        Some(v3) => {
            let a102 = v3;
            let a103 = v2;
            equal(a102, a103)
        }
    }
}

pub fn f18(v0: Adt4, v1: Adt3, v2: u64) -> Option<Adt4> {
    Some(Adt4::C0(v2, List::cons(Adt2::C0(v1, match v0.clone() {
        Adt4::C0(h104, _, _) => {
            h104
        }
    }, v2), match v0 {
        Adt4::C0(_, h105, _) => {
            h105
        }
    }), None::<Adt1>))
}

pub fn f19(v0: Bytes, v1: Adt4) -> R<Option<Adt4>> {
    let v2: u64 = match v1.clone() {
        Adt4::C0(h106, _, _) => {
            h106
        }
    };
    if f17(v0.clone(), {
        let a107 = v2;
        let a108 = 1u64;
        nat_add(a107, a108)?
    }, 61u8) {
        Ok(f18(v1, Adt3::C8, {
            let a109 = v2;
            let a110 = 2u64;
            nat_add(a109, a110)?
        }))
    } else {
        if f17(v0, {
            let a111 = v2;
            let a112 = 1u64;
            nat_add(a111, a112)?
        }, 58u8) {
            Ok(f18(v1, Adt3::C9, {
                let a113 = v2;
                let a114 = 2u64;
                nat_add(a113, a114)?
            }))
        } else {
            Ok(f18(v1, Adt3::C7, {
                let a115 = v2;
                let a116 = 1u64;
                nat_add(a115, a116)?
            }))
        }
    }
}

pub fn f20(v0: Bytes, v1: u64, v2: Adt4) -> R<Option<Adt4>> {
    let v3: u64 = match v2.clone() {
        Adt4::C0(h117, _, _) => {
            h117
        }
    };
    let v4: Adt7 = f26(v0, v1, v3)?;
    let m119 = match v4.clone() {
        Adt7::C0(_, _, _, h118) => {
            h118
        }
    };
    match m119 {
        None => {
            if match v4.clone() {
                Adt7::C0(_, _, h121, _) => {
                    h121
                }
            } {
                Ok(f18(v2, Adt3::C11(f27(match v4.clone() {
                    Adt7::C0(_, h122, _, _) => {
                        h122
                    }
                })), match v4 {
                    Adt7::C0(h123, _, _, _) => {
                        h123
                    }
                }))
            } else {
                Ok(Some(Adt4::C0(v3, match v2 {
                    Adt4::C0(_, h124, _) => {
                        h124
                    }
                }, Some(f28(v3, f29(Str::lit("unterminated string")))))))
            }
        }
        Some(v5) => {
            Ok(Some(Adt4::C0(v3, match v2 {
                Adt4::C0(_, h120, _) => {
                    h120
                }
            }, Some(v5))))
        }
    }
}

pub fn f21(v0: Bytes, v1: u64, v2: Adt4) -> R<Option<Adt4>> {
    let v3: u64 = match v2.clone() {
        Adt4::C0(h125, _, _) => {
            h125
        }
    };
    let v4: u64 = f30(v0.clone(), v1, v3)?;
    Ok(f18(v2, Adt3::C12(f31(v0, v3, {
        let a126 = v4;
        let a127 = v3;
        nat_sub(a126, a127)
    })), v4))
}

pub fn f22(v0: List<Adt2>, v1: u64, v2: Adt5) -> R<Option<Adt5>> {
    let m129 = match v2.clone() {
        Adt5::C0(_, _, _, _, _, h128) => {
            h128
        }
    };
    match m129 {
        None => {
            let a131 = match v2.clone() {
                Adt5::C0(_, _, h130, _, _, _) => {
                    h130
                }
            };
            let a132 = 9u64;
            if nat_eq(a131, a132) {
                Ok(None::<Adt5>)
            } else {
                Ok(Some(f32(v0, v1, v2)?))
            }
        }
        Some(_) => {
            Ok(None::<Adt5>)
        }
    }
}

pub fn f23(v0: Bytes, v1: u64, v2: u8, v3: u8) -> bool {
    let m133 = f25(v0, v1);
    match m133 {
        None => {
            false
        }
        Some(v4) => {
            f33(v4, v2, v3)
        }
    }
}

pub fn f24(v0: Bytes, v1: u64, v2: u8) -> bool {
    let m134 = f25(v0, v1);
    match m134 {
        None => {
            false
        }
        Some(v3) => {
            let a135 = v3;
            let a136 = v2;
            equal(a135, a136)
        }
    }
}

pub fn f25(v0: Bytes, v1: u64) -> Option<u8> {
    let a137 = v0;
    let a138 = v1;
    index_bytes(a137, a138)
}

pub fn f26(v0: Bytes, v1: u64, v2: u64) -> R<Adt7> {
    let (h139, _) = f34(Fn3::F35(v1, v0), {
        let a142 = {
            let a140 = v1;
            let a141 = v2;
            nat_sub(a140, a141)
        };
        let a143 = 1u64;
        nat_add(a142, a143)?
    }, Adt7::C0({
        let a144 = v2;
        let a145 = 1u64;
        nat_add(a144, a145)?
    }, List::<Bytes>::nil(), false, None::<Adt1>))?;
    Ok(h139)
}

pub fn f27(v0: List<Bytes>) -> Bytes {
    f36(f37(v0))
}

pub fn f28(v0: u64, v1: Bytes) -> Adt1 {
    Adt1::C0(v0, v1)
}

pub fn f29(v0: Str) -> Bytes {
    let a146 = v0;
    utf8_encode(a146)
}

pub fn f30(v0: Bytes, v1: u64, v2: u64) -> R<u64> {
    let (h147, _) = f38(Fn4::F39(v1, v0), {
        let a148 = v1;
        let a149 = v2;
        nat_sub(a148, a149)
    }, v2)?;
    Ok(h147)
}

pub fn f31(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m153 = {
        let a150 = v0;
        let a151 = v1;
        let a152 = v2;
        slice_bytes(a150, a151, a152)
    };
    match m153 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f32(v0: List<Adt2>, v1: u64, v2: Adt5) -> R<Adt5> {
    let v3: u64 = match v2.clone() {
        Adt5::C0(_, _, h154, _, _, _) => {
            h154
        }
    };
    let a155 = v3;
    let a156 = 0u64;
    if nat_eq(a155, a156) {
        f40(v0, v1, v2)
    } else {
        let a157 = v3;
        let a158 = 1u64;
        if nat_eq(a157, a158) {
            f41(v0, v2)
        } else {
            let a159 = v3;
            let a160 = 2u64;
            if nat_eq(a159, a160) {
                f42(v0, v1, v2)
            } else {
                let a161 = v3;
                let a162 = 3u64;
                if nat_eq(a161, a162) {
                    f43(v0, v2)
                } else {
                    let a163 = v3;
                    let a164 = 4u64;
                    if nat_eq(a163, a164) {
                        f44(v0, v1, v2, 3u64, f9(Str::lit("expected `,` or `]`")))
                    } else {
                        f44(v0, v1, v2, 1u64, f9(Str::lit("expected `,` or `}`")))
                    }
                }
            }
        }
    }
}

pub fn f33(v0: u8, v1: u8, v2: u8) -> bool {
    let m167 = {
        let a165 = v0;
        let a166 = v1;
        fixed_u8::checked_sub(a165, a166)
    };
    match m167 {
        None => {
            false
        }
        Some(_) => {
            let m170 = {
                let a168 = v2;
                let a169 = v0;
                fixed_u8::checked_sub(a168, a169)
            };
            match m170 {
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

pub fn f34(v0: Fn3, p175: u64, p176: Adt7) -> R<(Adt7, bool)> {
    let m174 = (p175, p176);
    {
        let mut m174 = m174;
        loop {
            let step = {
                let (v1, v2) = m174;
                let m171 = v1;
                if m171 == 0 {
                    Err::<(u64, Adt7), R<(Adt7, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m171 - 1;
                    let m173 = {
                        let c172: Fn3 = v0.clone();
                        c172.apply(v2.clone())?
                    };
                    match m173 {
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
                Ok(next) => { m174 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f35(v0: u64, v1: Bytes, v2: Adt7) -> R<Option<Adt7>> {
    f45(v1, v0, v2)
}

pub fn f36(v0: List<Bytes>) -> Bytes {
    let m179 = {
        let (h177, _) = f46(Fn5::F47, {
            let a178 = v0.clone();
            length_list(a178)
        }, v0);
        h177
    };
    match m179.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f37(v0: List<Bytes>) -> List<Bytes> {
    f48(Fn6::F49, List::<Bytes>::nil(), v0)
}

pub fn f38(v0: Fn4, p184: u64, p185: u64) -> R<(u64, bool)> {
    let m183 = (p184, p185);
    {
        let mut m183 = m183;
        loop {
            let step = {
                let (v1, v2) = m183;
                let m180 = v1;
                if m180 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m180 - 1;
                    let m182 = {
                        let c181: Fn4 = v0.clone();
                        c181.apply(v2)?
                    };
                    match m182 {
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
                Ok(next) => { m183 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f39(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f50(v1, v0, v2)
}

pub fn f40(v0: List<Adt2>, v1: u64, v2: Adt5) -> R<Adt5> {
    let v3: u64 = match v2.clone() {
        Adt5::C0(h186, _, _, _, _, _) => {
            h186
        }
    };
    let a188 = 256u64;
    let a189 = match v2.clone() {
        Adt5::C0(_, h187, _, _, _, _) => {
            h187
        }
    };
    if nat_lt(a188, a189) {
        Ok(f51(v2, f8(v0, v1, v3), f9(Str::lit("nested more deeply than the notation reads"))))
    } else {
        let v4: u64 = f52(v0.clone(), v3);
        let a190 = v4;
        let a191 = 99u64;
        if nat_eq(a190, a191) {
            Ok(f51(v2, v1, f9(Str::lit("expected a value, found the end of the text"))))
        } else {
            let a192 = v4;
            let a193 = 11u64;
            if nat_eq(a192, a193) {
                Ok(f53(v2, {
                    let a194 = v3;
                    let a195 = 1u64;
                    nat_add(a194, a195)?
                }, Adt0::C2(f54(v0, v3))))
            } else {
                let a196 = v4;
                let a197 = 2u64;
                if nat_eq(a196, a197) {
                    f55(v2, {
                        let a198 = v3;
                        let a199 = 1u64;
                        nat_add(a198, a199)?
                    }, Adt6::C0(List::<Adt0>::nil()), 1u64)
                } else {
                    let a200 = v4;
                    let a201 = 0u64;
                    if nat_eq(a200, a201) {
                        f55(v2, {
                            let a202 = v3;
                            let a203 = 1u64;
                            nat_add(a202, a203)?
                        }, Adt6::C1(Bytes::lit(&[]), 0u64, List::<(Bytes, Adt0)>::nil()), 2u64)
                    } else {
                        let a204 = v4;
                        let a205 = 12u64;
                        if nat_eq(a204, a205) {
                            f56(v0.clone(), v1, v2, f8(v0, v1, v3))
                        } else {
                            Ok(f51(v2, f8(v0.clone(), v1, v3), f57(f9(Str::lit("expected a value, found ")), f58(v0, v3)?)))
                        }
                    }
                }
            }
        }
    }
}

pub fn f41(v0: List<Adt2>, v1: Adt5) -> R<Adt5> {
    let a207 = f52(v0, match v1.clone() {
        Adt5::C0(h206, _, _, _, _, _) => {
            h206
        }
    });
    let a208 = 3u64;
    if nat_eq(a207, a208) {
        f59(v1)
    } else {
        Ok(Adt5::C0(match v1.clone() {
            Adt5::C0(h209, _, _, _, _, _) => {
                h209
            }
        }, match v1.clone() {
            Adt5::C0(_, h210, _, _, _, _) => {
                h210
            }
        }, 0u64, match v1 {
            Adt5::C0(_, _, _, h211, _, _) => {
                h211
            }
        }, Adt0::C5, None::<Adt1>))
    }
}

pub fn f42(v0: List<Adt2>, v1: u64, v2: Adt5) -> R<Adt5> {
    let m213 = match v2.clone() {
        Adt5::C0(_, _, _, h212, _, _) => {
            h212
        }
    };
    match m213.uncons() {
        None => {
            Ok(f51(v2, 0u64, f9(Str::lit("no object is open"))))
        }
        Some((v3, v4)) => {
            let m214 = v3;
            match m214 {
                Adt6::C0(_) => {
                    Ok(f51(v2, 0u64, f9(Str::lit("no object is open"))))
                }
                Adt6::C1(_, _, v8) => {
                    let a216 = f52(v0.clone(), match v2.clone() {
                        Adt5::C0(h215, _, _, _, _, _) => {
                            h215
                        }
                    });
                    let a217 = 1u64;
                    if nat_eq(a216, a217) {
                        f59(v2)
                    } else {
                        f60(v0, v1, v2, v8, v4)
                    }
                }
            }
        }
    }
}

pub fn f43(v0: List<Adt2>, v1: Adt5) -> R<Adt5> {
    let m219 = match v1.clone() {
        Adt5::C0(_, _, _, h218, _, _) => {
            h218
        }
    };
    match m219.uncons() {
        None => {
            Ok(Adt5::C0(match v1.clone() {
                Adt5::C0(h220, _, _, _, _, _) => {
                    h220
                }
            }, match v1.clone() {
                Adt5::C0(_, h221, _, _, _, _) => {
                    h221
                }
            }, 9u64, List::<Adt6>::nil(), match v1 {
                Adt5::C0(_, _, _, _, h222, _) => {
                    h222
                }
            }, None::<Adt1>))
        }
        Some((v2, v3)) => {
            let m223 = v2;
            match m223 {
                Adt6::C0(v4) => {
                    f61(v0, v1.clone(), List::cons(Adt6::C0(List::cons(match v1 {
                        Adt5::C0(_, _, _, _, h224, _) => {
                            h224
                        }
                    }, v4)), v3), 1u64, 4u64)
                }
                Adt6::C1(v5, v6, v7) => {
                    if f62(v7.clone(), v5.clone()) {
                        Ok(f51(v1, v6, f57(f57(f9(Str::lit("the member `")), v5), f9(Str::lit("` is written twice")))))
                    } else {
                        f61(v0, v1.clone(), List::cons(Adt6::C1(Bytes::lit(&[]), 0u64, List::cons((v5, match v1 {
                            Adt5::C0(_, _, _, _, h225, _) => {
                                h225
                            }
                        }), v7)), v3), 2u64, 5u64)
                    }
                }
            }
        }
    }
}

pub fn f44(v0: List<Adt2>, v1: u64, v2: Adt5, v3: u64, v4: Bytes) -> R<Adt5> {
    let a227 = f52(v0.clone(), match v2.clone() {
        Adt5::C0(h226, _, _, _, _, _) => {
            h226
        }
    });
    let a228 = v3;
    if nat_eq(a227, a228) {
        f59(v2)
    } else {
        Ok(f51(v2.clone(), f8(v0, v1, match v2 {
            Adt5::C0(h229, _, _, _, _, _) => {
                h229
            }
        }), v4))
    }
}

pub fn f45(v0: Bytes, v1: u64, v2: Adt7) -> R<Option<Adt7>> {
    let m231 = match v2.clone() {
        Adt7::C0(_, _, _, h230) => {
            h230
        }
    };
    match m231 {
        None => {
            if match v2.clone() {
                Adt7::C0(_, _, h232, _) => {
                    h232
                }
            } {
                Ok(None::<Adt7>)
            } else {
                let v4: u64 = match v2.clone() {
                    Adt7::C0(h233, _, _, _) => {
                        h233
                    }
                };
                let a234 = v4;
                let a235 = v1;
                if nat_lt(a234, a235) {
                    if f17(v0.clone(), v4, 34u8) {
                        Ok(Some(Adt7::C0({
                            let a236 = v4;
                            let a237 = 1u64;
                            nat_add(a236, a237)?
                        }, match v2 {
                            Adt7::C0(_, h238, _, _) => {
                                h238
                            }
                        }, true, None::<Adt1>)))
                    } else {
                        if f17(v0.clone(), v4, 92u8) {
                            Ok(Some(f63(v0, v1, v2)?))
                        } else {
                            let a239 = f64(v0.clone(), v4)?;
                            let a240 = 32u64;
                            if nat_lt(a239, a240) {
                                Ok(Some(f65(v2, v4, f29(Str::lit("a control character must be escaped in a string")))))
                            } else {
                                let v5: u64 = f66(v0.clone(), v1, v4)?;
                                Ok(Some(f67(v2, v5, f31(v0, v4, {
                                    let a241 = v5;
                                    let a242 = v4;
                                    nat_sub(a241, a242)
                                }))))
                            }
                        }
                    }
                } else {
                    Ok(None::<Adt7>)
                }
            }
        }
        Some(_) => {
            Ok(None::<Adt7>)
        }
    }
}

pub fn f46(v0: Fn5, p247: u64, p248: List<Bytes>) -> (List<Bytes>, bool) {
    let m246 = (p247, p248);
    {
        let mut m246 = m246;
        loop {
            let step = {
                let (v1, v2) = m246;
                let m243 = v1;
                if m243 == 0 {
                    Err::<(u64, List<Bytes>), (List<Bytes>, bool)>((v2, false))
                } else {
                    let v3 = m243 - 1;
                    let m245 = {
                        let c244: Fn5 = v0.clone();
                        c244.apply(v2.clone())
                    };
                    match m245 {
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
                Ok(next) => { m246 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f47(v0: List<Bytes>) -> Option<List<Bytes>> {
    f68(v0)
}

pub fn f48(v0: Fn6, p252: List<Bytes>, p253: List<Bytes>) -> List<Bytes> {
    let m251 = (p252, p253);
    {
        let mut m251 = m251;
        loop {
            let step = {
                let (v1, v2) = m251;
                let m249 = v2;
                match m249.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c250: Fn6 = v0.clone();
                            c250.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m251 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f49(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f50(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let a254 = v2;
    let a255 = v1;
    if nat_lt(a254, a255) {
        if f69(v0, v2)? {
            Ok(None::<u64>)
        } else {
            Ok(Some({
                let a256 = v2;
                let a257 = 1u64;
                nat_add(a256, a257)?
            }))
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f51(v0: Adt5, v1: u64, v2: Bytes) -> Adt5 {
    Adt5::C0(match v0.clone() {
        Adt5::C0(h258, _, _, _, _, _) => {
            h258
        }
    }, match v0.clone() {
        Adt5::C0(_, h259, _, _, _, _) => {
            h259
        }
    }, 9u64, match v0.clone() {
        Adt5::C0(_, _, _, h260, _, _) => {
            h260
        }
    }, match v0 {
        Adt5::C0(_, _, _, _, h261, _) => {
            h261
        }
    }, Some(f7(v1, v2)))
}

pub fn f52(v0: List<Adt2>, v1: u64) -> u64 {
    let m262 = f15(v0, v1);
    match m262 {
        None => {
            99u64
        }
        Some(v2) => {
            f70(match v2 {
                Adt2::C0(h263, _, _) => {
                    h263
                }
            })
        }
    }
}

pub fn f53(v0: Adt5, v1: u64, v2: Adt0) -> Adt5 {
    Adt5::C0(v1, match v0.clone() {
        Adt5::C0(_, h264, _, _, _, _) => {
            h264
        }
    }, 3u64, match v0 {
        Adt5::C0(_, _, _, h265, _, _) => {
            h265
        }
    }, v2, None::<Adt1>)
}

pub fn f54(v0: List<Adt2>, v1: u64) -> Bytes {
    let m266 = f15(v0, v1);
    match m266 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            f71(match v2 {
                Adt2::C0(h267, _, _) => {
                    h267
                }
            })
        }
    }
}

pub fn f55(v0: Adt5, v1: u64, v2: Adt6, v3: u64) -> R<Adt5> {
    Ok(Adt5::C0(v1, {
        let a269 = match v0.clone() {
            Adt5::C0(_, h268, _, _, _, _) => {
                h268
            }
        };
        let a270 = 1u64;
        nat_add(a269, a270)?
    }, v3, List::cons(v2, match v0 {
        Adt5::C0(_, _, _, h271, _, _) => {
            h271
        }
    }), Adt0::C5, None::<Adt1>))
}

pub fn f56(v0: List<Adt2>, _: u64, v2: Adt5, v3: u64) -> R<Adt5> {
    let v4: u64 = match v2.clone() {
        Adt5::C0(h272, _, _, _, _, _) => {
            h272
        }
    };
    let v5: Bytes = f54(v0.clone(), v4);
    let a273 = v5.clone();
    let a274 = Bytes::lit(&[0x74, 0x72, 0x75, 0x65]);
    if equal(a273, a274) {
        Ok(f53(v2, {
            let a275 = v4;
            let a276 = 1u64;
            nat_add(a275, a276)?
        }, Adt0::C0(true)))
    } else {
        let a277 = v5.clone();
        let a278 = Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]);
        if equal(a277, a278) {
            Ok(f53(v2, {
                let a279 = v4;
                let a280 = 1u64;
                nat_add(a279, a280)?
            }, Adt0::C0(false)))
        } else {
            if f72(v5.clone())? {
                let m281 = f73(v5.clone())?;
                match m281 {
                    None => {
                        Ok(f51(v2, v3, f57(f57(f9(Str::lit("`")), v5), f9(Str::lit("` does not fit a 64-bit integer")))))
                    }
                    Some(v6) => {
                        Ok(f53(v2, {
                            let a282 = v4;
                            let a283 = 1u64;
                            nat_add(a282, a283)?
                        }, v6))
                    }
                }
            } else {
                if f74(v5.clone())? {
                    let a286 = f52(v0, {
                        let a284 = v4;
                        let a285 = 1u64;
                        nat_add(a284, a285)?
                    });
                    let a287 = 0u64;
                    if nat_eq(a286, a287) {
                        f55(v2, {
                            let a288 = v4;
                            let a289 = 2u64;
                            nat_add(a288, a289)?
                        }, Adt6::C1(Bytes::lit(&[]), 0u64, List::cons((f9(Str::lit("kind")), Adt0::C2(v5)), List::<(Bytes, Adt0)>::nil())), 2u64)
                    } else {
                        Ok(f53(v2, {
                            let a290 = v4;
                            let a291 = 1u64;
                            nat_add(a290, a291)?
                        }, Adt0::C2(v5)))
                    }
                } else {
                    Ok(f51(v2, v3, f57(f57(f9(Str::lit("`")), v5), f9(Str::lit("` is not a name; quote it to write a string")))))
                }
            }
        }
    }
}

pub fn f57(v0: Bytes, v1: Bytes) -> Bytes {
    let a292 = v0;
    let a293 = v1;
    append_bytes(a292, a293)
}

pub fn f58(v0: List<Adt2>, v1: u64) -> R<Bytes> {
    let m294 = f15(v0, v1);
    match m294 {
        None => {
            Ok(f9(Str::lit("the end of the text")))
        }
        Some(v2) => {
            f75(match v2 {
                Adt2::C0(h295, _, _) => {
                    h295
                }
            })
        }
    }
}

pub fn f59(v0: Adt5) -> R<Adt5> {
    let m297 = match v0.clone() {
        Adt5::C0(_, _, _, h296, _, _) => {
            h296
        }
    };
    match m297.uncons() {
        None => {
            Ok(f51(v0, 0u64, f9(Str::lit("nothing to close"))))
        }
        Some((v1, v2)) => {
            let m298 = v1;
            match m298 {
                Adt6::C0(v3) => {
                    Ok(Adt5::C0({
                        let a300 = match v0.clone() {
                            Adt5::C0(h299, _, _, _, _, _) => {
                                h299
                            }
                        };
                        let a301 = 1u64;
                        nat_add(a300, a301)?
                    }, {
                        let a303 = match v0 {
                            Adt5::C0(_, h302, _, _, _, _) => {
                                h302
                            }
                        };
                        let a304 = 1u64;
                        nat_sub(a303, a304)
                    }, 3u64, v2, Adt0::C3(std::rc::Rc::new(f76(v3))), None::<Adt1>))
                }
                Adt6::C1(_, _, v6) => {
                    Ok(Adt5::C0({
                        let a306 = match v0.clone() {
                            Adt5::C0(h305, _, _, _, _, _) => {
                                h305
                            }
                        };
                        let a307 = 1u64;
                        nat_add(a306, a307)?
                    }, {
                        let a309 = match v0 {
                            Adt5::C0(_, h308, _, _, _, _) => {
                                h308
                            }
                        };
                        let a310 = 1u64;
                        nat_sub(a309, a310)
                    }, 3u64, v2, Adt0::C4(std::rc::Rc::new(f77(f78(v6)?))), None::<Adt1>))
                }
            }
        }
    }
}

pub fn f60(v0: List<Adt2>, v1: u64, v2: Adt5, v3: List<(Bytes, Adt0)>, v4: List<Adt6>) -> R<Adt5> {
    let v5: u64 = match v2.clone() {
        Adt5::C0(h311, _, _, _, _, _) => {
            h311
        }
    };
    let v6: u64 = f52(v0.clone(), v5);
    let v7: Bytes = f54(v0.clone(), v5);
    let a312 = v6;
    let a313 = 11u64;
    if if nat_eq(a312, a313) {
        true
    } else {
        let a314 = v6;
        let a315 = 12u64;
        if nat_eq(a314, a315) {
            f74(v7.clone())?
        } else {
            false
        }
    } {
        let a318 = f52(v0.clone(), {
            let a316 = v5;
            let a317 = 1u64;
            nat_add(a316, a317)?
        });
        let a319 = 7u64;
        if nat_eq(a318, a319) {
            Ok(Adt5::C0({
                let a320 = v5;
                let a321 = 2u64;
                nat_add(a320, a321)?
            }, match v2 {
                Adt5::C0(_, h322, _, _, _, _) => {
                    h322
                }
            }, 0u64, List::cons(Adt6::C1(v7, f8(v0, v1, v5), v3), v4), Adt0::C5, None::<Adt1>))
        } else {
            Ok(f51(v2, f8(v0, v1, {
                let a323 = v5;
                let a324 = 1u64;
                nat_add(a323, a324)?
            }), f9(Str::lit("expected `:` after a member name"))))
        }
    } else {
        Ok(f51(v2, f8(v0, v1, v5), f9(Str::lit("expected a member name"))))
    }
}

pub fn f61(v0: List<Adt2>, v1: Adt5, v2: List<Adt6>, v3: u64, v4: u64) -> R<Adt5> {
    let a326 = f52(v0, match v1.clone() {
        Adt5::C0(h325, _, _, _, _, _) => {
            h325
        }
    });
    let a327 = 6u64;
    if nat_eq(a326, a327) {
        Ok(Adt5::C0({
            let a329 = match v1.clone() {
                Adt5::C0(h328, _, _, _, _, _) => {
                    h328
                }
            };
            let a330 = 1u64;
            nat_add(a329, a330)?
        }, match v1 {
            Adt5::C0(_, h331, _, _, _, _) => {
                h331
            }
        }, v3, v2, Adt0::C5, None::<Adt1>))
    } else {
        Ok(Adt5::C0(match v1.clone() {
            Adt5::C0(h332, _, _, _, _, _) => {
                h332
            }
        }, match v1 {
            Adt5::C0(_, h333, _, _, _, _) => {
                h333
            }
        }, v4, v2, Adt0::C5, None::<Adt1>))
    }
}

pub fn f62(v0: List<(Bytes, Adt0)>, v1: Bytes) -> bool {
    f79(Fn7::F80(v1), false, v0)
}

pub fn f63(v0: Bytes, v1: u64, v2: Adt7) -> R<Adt7> {
    let v3: u64 = match v2.clone() {
        Adt7::C0(h334, _, _, _) => {
            h334
        }
    };
    let m337 = f25(v0.clone(), {
        let a335 = v3;
        let a336 = 1u64;
        nat_add(a335, a336)?
    });
    match m337 {
        None => {
            Ok(Adt7::C0(v1, match v2 {
                Adt7::C0(_, h338, _, _) => {
                    h338
                }
            }, false, None::<Adt1>))
        }
        Some(v4) => {
            let a339 = v4;
            let a340 = 34u8;
            if equal(a339, a340) {
                Ok(f67(v2, {
                    let a341 = v3;
                    let a342 = 2u64;
                    nat_add(a341, a342)?
                }, f29(Str::lit("\""))))
            } else {
                let a343 = v4;
                let a344 = 92u8;
                if equal(a343, a344) {
                    Ok(f67(v2, {
                        let a345 = v3;
                        let a346 = 2u64;
                        nat_add(a345, a346)?
                    }, f29(Str::lit("\\"))))
                } else {
                    let a347 = v4;
                    let a348 = 47u8;
                    if equal(a347, a348) {
                        Ok(f67(v2, {
                            let a349 = v3;
                            let a350 = 2u64;
                            nat_add(a349, a350)?
                        }, f29(Str::lit("/"))))
                    } else {
                        let a351 = v4;
                        let a352 = 98u8;
                        if equal(a351, a352) {
                            Ok(f67(v2, {
                                let a353 = v3;
                                let a354 = 2u64;
                                nat_add(a353, a354)?
                            }, f81(8u64)))
                        } else {
                            let a355 = v4;
                            let a356 = 102u8;
                            if equal(a355, a356) {
                                Ok(f67(v2, {
                                    let a357 = v3;
                                    let a358 = 2u64;
                                    nat_add(a357, a358)?
                                }, f81(12u64)))
                            } else {
                                let a359 = v4;
                                let a360 = 110u8;
                                if equal(a359, a360) {
                                    Ok(f67(v2, {
                                        let a361 = v3;
                                        let a362 = 2u64;
                                        nat_add(a361, a362)?
                                    }, f81(10u64)))
                                } else {
                                    let a363 = v4;
                                    let a364 = 114u8;
                                    if equal(a363, a364) {
                                        Ok(f67(v2, {
                                            let a365 = v3;
                                            let a366 = 2u64;
                                            nat_add(a365, a366)?
                                        }, f81(13u64)))
                                    } else {
                                        let a367 = v4;
                                        let a368 = 116u8;
                                        if equal(a367, a368) {
                                            Ok(f67(v2, {
                                                let a369 = v3;
                                                let a370 = 2u64;
                                                nat_add(a369, a370)?
                                            }, f81(9u64)))
                                        } else {
                                            let a371 = v4;
                                            let a372 = 117u8;
                                            if equal(a371, a372) {
                                                f82(v0, v2)
                                            } else {
                                                Ok(f65(v2, v3, f83(f83(f29(Str::lit("`\\")), f31(v0.clone(), {
                                                    let a373 = v3;
                                                    let a374 = 1u64;
                                                    nat_add(a373, a374)?
                                                }, f84(v0, {
                                                    let a375 = v3;
                                                    let a376 = 1u64;
                                                    nat_add(a375, a376)?
                                                })?)), f29(Str::lit("` is not an escape")))))
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

pub fn f64(v0: Bytes, v1: u64) -> R<u64> {
    let m377 = f25(v0, v1);
    match m377 {
        None => {
            Ok(0u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a422 = {
                let a398 = {
                    let a386 = {
                        let a380 = {
                            let a378 = v3;
                            let a379 = 128u8;
                            fixed_u8::bit_and(a378, a379)
                        };
                        let a381 = 128u8;
                        if equal(a380, a381) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a387 = {
                        let a384 = {
                            let a382 = v3;
                            let a383 = 64u8;
                            fixed_u8::bit_and(a382, a383)
                        };
                        let a385 = 64u8;
                        if equal(a384, a385) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a386, a387)?
                };
                let a399 = {
                    let a396 = {
                        let a390 = {
                            let a388 = v3;
                            let a389 = 32u8;
                            fixed_u8::bit_and(a388, a389)
                        };
                        let a391 = 32u8;
                        if equal(a390, a391) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a397 = {
                        let a394 = {
                            let a392 = v3;
                            let a393 = 16u8;
                            fixed_u8::bit_and(a392, a393)
                        };
                        let a395 = 16u8;
                        if equal(a394, a395) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a396, a397)?
                };
                nat_add(a398, a399)?
            };
            let a423 = {
                let a420 = {
                    let a408 = {
                        let a402 = {
                            let a400 = v3;
                            let a401 = 8u8;
                            fixed_u8::bit_and(a400, a401)
                        };
                        let a403 = 8u8;
                        if equal(a402, a403) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a409 = {
                        let a406 = {
                            let a404 = v3;
                            let a405 = 4u8;
                            fixed_u8::bit_and(a404, a405)
                        };
                        let a407 = 4u8;
                        if equal(a406, a407) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a408, a409)?
                };
                let a421 = {
                    let a418 = {
                        let a412 = {
                            let a410 = v3;
                            let a411 = 2u8;
                            fixed_u8::bit_and(a410, a411)
                        };
                        let a413 = 2u8;
                        if equal(a412, a413) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a419 = {
                        let a416 = {
                            let a414 = v3;
                            let a415 = 1u8;
                            fixed_u8::bit_and(a414, a415)
                        };
                        let a417 = 1u8;
                        if equal(a416, a417) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a418, a419)?
                };
                nat_add(a420, a421)?
            };
            nat_add(a422, a423)
        }
    }
}

pub fn f65(v0: Adt7, v1: u64, v2: Bytes) -> Adt7 {
    Adt7::C0(match v0.clone() {
        Adt7::C0(h424, _, _, _) => {
            h424
        }
    }, match v0 {
        Adt7::C0(_, h425, _, _) => {
            h425
        }
    }, false, Some(f28(v1, v2)))
}

pub fn f66(v0: Bytes, v1: u64, v2: u64) -> R<u64> {
    let (h426, _) = f38(Fn4::F85(v1, v0), {
        let a427 = v1;
        let a428 = v2;
        nat_sub(a427, a428)
    }, v2)?;
    Ok(h426)
}

pub fn f67(v0: Adt7, v1: u64, v2: Bytes) -> Adt7 {
    Adt7::C0(v1, List::cons(v2, match v0 {
        Adt7::C0(_, h429, _, _) => {
            h429
        }
    }), false, None::<Adt1>)
}

pub fn f68(v0: List<Bytes>) -> Option<List<Bytes>> {
    let m430 = v0.clone();
    match m430.uncons() {
        None => {
            None::<List<Bytes>>
        }
        Some((_, v2)) => {
            let m431 = v2;
            match m431.uncons() {
                None => {
                    None::<List<Bytes>>
                }
                Some((_, _)) => {
                    let v5: (Option<Bytes>, List<Bytes>) = f86(Fn8::F87, (None::<Bytes>, List::<Bytes>::nil()), v0);
                    let m433 = {
                        let (h432, _) = v5;
                        h432
                    };
                    match m433 {
                        None => {
                            Some(f37({
                                let (_, h435) = v5;
                                h435
                            }))
                        }
                        Some(v6) => {
                            Some(f37(List::cons(v6, {
                                let (_, h434) = v5;
                                h434
                            })))
                        }
                    }
                }
            }
        }
    }
}

pub fn f69(v0: Bytes, v1: u64) -> R<bool> {
    let a436 = 0u64;
    let a437 = f16(v0.clone(), v1)?;
    if nat_lt(a436, a437) {
        Ok(true)
    } else {
        Ok(f88(v0, v1))
    }
}

pub fn f70(v0: Adt3) -> u64 {
    let m438 = v0;
    match m438 {
        Adt3::C0 => {
            0u64
        }
        Adt3::C1 => {
            1u64
        }
        Adt3::C2 => {
            2u64
        }
        Adt3::C3 => {
            3u64
        }
        Adt3::C4 => {
            4u64
        }
        Adt3::C5 => {
            5u64
        }
        Adt3::C6 => {
            6u64
        }
        Adt3::C7 => {
            7u64
        }
        Adt3::C8 => {
            8u64
        }
        Adt3::C9 => {
            9u64
        }
        Adt3::C10 => {
            10u64
        }
        Adt3::C11(_) => {
            11u64
        }
        Adt3::C12(_) => {
            12u64
        }
    }
}

pub fn f71(v0: Adt3) -> Bytes {
    let m439 = v0;
    match m439 {
        Adt3::C0 => {
            Bytes::lit(&[])
        }
        Adt3::C1 => {
            Bytes::lit(&[])
        }
        Adt3::C2 => {
            Bytes::lit(&[])
        }
        Adt3::C3 => {
            Bytes::lit(&[])
        }
        Adt3::C4 => {
            Bytes::lit(&[])
        }
        Adt3::C5 => {
            Bytes::lit(&[])
        }
        Adt3::C6 => {
            Bytes::lit(&[])
        }
        Adt3::C7 => {
            Bytes::lit(&[])
        }
        Adt3::C8 => {
            Bytes::lit(&[])
        }
        Adt3::C9 => {
            Bytes::lit(&[])
        }
        Adt3::C10 => {
            Bytes::lit(&[])
        }
        Adt3::C11(v1) => {
            v1
        }
        Adt3::C12(v2) => {
            v2
        }
    }
}

pub fn f72(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a440 = f89(v0.clone(), 0u64)?;
        let a441 = 45u64;
        if nat_eq(a440, a441) {
            1u64
        } else {
            0u64
        }
    };
    let a443 = v1;
    let a444 = {
        let a442 = v0.clone();
        length_bytes(a442)
    };
    if nat_lt(a443, a444) {
        let a446 = f90(v0.clone(), v1)?;
        let a447 = {
            let a445 = v0;
            length_bytes(a445)
        };
        Ok(nat_eq(a446, a447))
    } else {
        Ok(false)
    }
}

pub fn f73(v0: Bytes) -> R<Option<Adt0>> {
    let v1: bool = {
        let a448 = f89(v0.clone(), 0u64)?;
        let a449 = 45u64;
        nat_eq(a448, a449)
    };
    let v2: u64 = if v1 {
        1u64
    } else {
        0u64
    };
    let v3: u64 = f91(v0.clone(), v2)?;
    let v4: Bytes = f31(v0.clone(), v3, {
        let a451 = {
            let a450 = v0;
            length_bytes(a450)
        };
        let a452 = v3;
        nat_sub(a451, a452)
    });
    let a454 = {
        let a453 = v4.clone();
        length_bytes(a453)
    };
    let a455 = 0u64;
    if nat_eq(a454, a455) {
        Ok(Some(Adt0::C1(false, 0u64)))
    } else {
        let a457 = 19u64;
        let a458 = {
            let a456 = v4.clone();
            length_bytes(a456)
        };
        if nat_lt(a457, a458) {
            Ok(None::<Adt0>)
        } else {
            let m459 = f92(v4)?;
            match m459 {
                None => {
                    Ok(None::<Adt0>)
                }
                Some(v5) => {
                    if f93(v1, v5) {
                        Ok(Some(Adt0::C1(v1, v5)))
                    } else {
                        Ok(None::<Adt0>)
                    }
                }
            }
        }
    }
}

pub fn f74(v0: Bytes) -> R<bool> {
    let a461 = {
        let a460 = v0.clone();
        length_bytes(a460)
    };
    let a462 = 0u64;
    if nat_eq(a461, a462) {
        Ok(false)
    } else {
        if f94(f89(v0.clone(), 0u64)?) {
            let a464 = f95(v0.clone(), 1u64)?;
            let a465 = {
                let a463 = v0.clone();
                length_bytes(a463)
            };
            if nat_eq(a464, a465) {
                let a466 = f96(v0);
                Ok(bool_not(a466))
            } else {
                Ok(false)
            }
        } else {
            Ok(false)
        }
    }
}

pub fn f75(v0: Adt3) -> R<Bytes> {
    let m467 = v0;
    match m467 {
        Adt3::C0 => {
            Ok(f9(Str::lit("`{`")))
        }
        Adt3::C1 => {
            Ok(f9(Str::lit("`}`")))
        }
        Adt3::C2 => {
            Ok(f9(Str::lit("`[`")))
        }
        Adt3::C3 => {
            Ok(f9(Str::lit("`]`")))
        }
        Adt3::C4 => {
            Ok(f9(Str::lit("`(`")))
        }
        Adt3::C5 => {
            Ok(f9(Str::lit("`)`")))
        }
        Adt3::C6 => {
            Ok(f9(Str::lit("`,`")))
        }
        Adt3::C7 => {
            Ok(f9(Str::lit("`:`")))
        }
        Adt3::C8 => {
            Ok(f9(Str::lit("`:=`")))
        }
        Adt3::C9 => {
            Ok(f9(Str::lit("`::`")))
        }
        Adt3::C10 => {
            Ok(f9(Str::lit("`;`")))
        }
        Adt3::C11(v1) => {
            Ok(f57(f9(Str::lit("the string ")), f97(v1)?))
        }
        Adt3::C12(v2) => {
            Ok(f57(f57(f9(Str::lit("`")), v2), f9(Str::lit("`"))))
        }
    }
}

pub fn f76(v0: List<Adt0>) -> Adt0 {
    f98(Fn9::F99, Adt0::C5, v0)
}

pub fn f77(v0: List<(Bytes, Adt0)>) -> Adt0 {
    f100(Fn10::F101, Adt0::C5, f102(v0))
}

pub fn f78(v0: List<(Bytes, Adt0)>) -> R<List<(Bytes, Adt0)>> {
    let v1: List<List<(Bytes, Adt0)>> = f103(f104(Fn11::F105, List::<List<(Bytes, Adt0)>>::nil(), v0.clone()));
    let m472 = {
        let (h468, _) = f106(Fn12::F107, {
            let a470 = {
                let a469 = v0;
                length_list(a469)
            };
            let a471 = 1u64;
            nat_add(a470, a471)?
        }, v1)?;
        h468
    };
    match m472.uncons() {
        None => {
            Ok(List::<(Bytes, Adt0)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f79(v0: Fn7, p476: bool, p477: List<(Bytes, Adt0)>) -> bool {
    let m475 = (p476, p477);
    {
        let mut m475 = m475;
        loop {
            let step = {
                let (v1, v2) = m475;
                let m473 = v2;
                match m473.uncons() {
                    None => {
                        Err::<(bool, List<(Bytes, Adt0)>), bool>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<(Bytes, Adt0)>), bool>(({
                            let c474: Fn7 = v0.clone();
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

pub fn f80(v0: Bytes, v1: bool, v2: (Bytes, Adt0)) -> bool {
    if v1 {
        true
    } else {
        let a479 = {
            let (h478, _) = v2;
            h478
        };
        let a480 = v0;
        equal(a479, a480)
    }
}

pub fn f81(v0: u64) -> Bytes {
    f31(Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff]), v0, 1u64)
}

pub fn f82(v0: Bytes, v1: Adt7) -> R<Adt7> {
    let v2: u64 = match v1.clone() {
        Adt7::C0(h481, _, _, _) => {
            h481
        }
    };
    let m484 = f108(v0, {
        let a482 = v2;
        let a483 = 2u64;
        nat_add(a482, a483)?
    })?;
    match m484 {
        None => {
            Ok(f65(v1, v2, f29(Str::lit("a `\\u` escape takes four hexadecimal digits"))))
        }
        Some(v3) => {
            let a485 = 55296u64;
            let a486 = v3;
            if nat_le(a485, a486) {
                let a487 = v3;
                let a488 = 57343u64;
                if nat_le(a487, a488) {
                    Ok(f65(v1, v2, f83(f83(f29(Str::lit("`\\u")), f109(v3)), f29(Str::lit("` is not a Unicode scalar value")))))
                } else {
                    Ok(f67(v1, {
                        let a489 = v2;
                        let a490 = 6u64;
                        nat_add(a489, a490)?
                    }, f110(v3)?))
                }
            } else {
                Ok(f67(v1, {
                    let a491 = v2;
                    let a492 = 6u64;
                    nat_add(a491, a492)?
                }, f110(v3)?))
            }
        }
    }
}

pub fn f83(v0: Bytes, v1: Bytes) -> Bytes {
    let a493 = v0;
    let a494 = v1;
    append_bytes(a493, a494)
}

pub fn f84(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f64(v0, v1)?;
    let a495 = v2;
    let a496 = 128u64;
    if nat_lt(a495, a496) {
        Ok(1u64)
    } else {
        let a497 = v2;
        let a498 = 224u64;
        if nat_lt(a497, a498) {
            Ok(2u64)
        } else {
            let a499 = v2;
            let a500 = 240u64;
            if nat_lt(a499, a500) {
                Ok(3u64)
            } else {
                Ok(4u64)
            }
        }
    }
}

pub fn f85(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f111(v1, v0, v2)
}

pub fn f86(v0: Fn8, p504: (Option<Bytes>, List<Bytes>), p505: List<Bytes>) -> (Option<Bytes>, List<Bytes>) {
    let m503 = (p504, p505);
    {
        let mut m503 = m503;
        loop {
            let step = {
                let (v1, v2) = m503;
                let m501 = v2;
                match m501.uncons() {
                    None => {
                        Err::<((Option<Bytes>, List<Bytes>), List<Bytes>), (Option<Bytes>, List<Bytes>)>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<((Option<Bytes>, List<Bytes>), List<Bytes>), (Option<Bytes>, List<Bytes>)>(({
                            let c502: Fn8 = v0.clone();
                            c502.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m503 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f87(v0: (Option<Bytes>, List<Bytes>), v1: Bytes) -> (Option<Bytes>, List<Bytes>) {
    f112(v0, v1)
}

pub fn f88(v0: Bytes, v1: u64) -> bool {
    if f17(v0.clone(), v1, 123u8) {
        true
    } else {
        if f17(v0.clone(), v1, 125u8) {
            true
        } else {
            if f17(v0.clone(), v1, 91u8) {
                true
            } else {
                if f17(v0.clone(), v1, 93u8) {
                    true
                } else {
                    if f17(v0.clone(), v1, 40u8) {
                        true
                    } else {
                        if f17(v0.clone(), v1, 41u8) {
                            true
                        } else {
                            if f17(v0.clone(), v1, 44u8) {
                                true
                            } else {
                                if f17(v0.clone(), v1, 58u8) {
                                    true
                                } else {
                                    if f17(v0.clone(), v1, 59u8) {
                                        true
                                    } else {
                                        f17(v0, v1, 34u8)
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

pub fn f89(v0: Bytes, v1: u64) -> R<u64> {
    let m506 = f25(v0, v1);
    match m506 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a551 = {
                let a527 = {
                    let a515 = {
                        let a509 = {
                            let a507 = v3;
                            let a508 = 128u8;
                            fixed_u8::bit_and(a507, a508)
                        };
                        let a510 = 128u8;
                        if equal(a509, a510) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a516 = {
                        let a513 = {
                            let a511 = v3;
                            let a512 = 64u8;
                            fixed_u8::bit_and(a511, a512)
                        };
                        let a514 = 64u8;
                        if equal(a513, a514) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a515, a516)?
                };
                let a528 = {
                    let a525 = {
                        let a519 = {
                            let a517 = v3;
                            let a518 = 32u8;
                            fixed_u8::bit_and(a517, a518)
                        };
                        let a520 = 32u8;
                        if equal(a519, a520) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a526 = {
                        let a523 = {
                            let a521 = v3;
                            let a522 = 16u8;
                            fixed_u8::bit_and(a521, a522)
                        };
                        let a524 = 16u8;
                        if equal(a523, a524) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a525, a526)?
                };
                nat_add(a527, a528)?
            };
            let a552 = {
                let a549 = {
                    let a537 = {
                        let a531 = {
                            let a529 = v3;
                            let a530 = 8u8;
                            fixed_u8::bit_and(a529, a530)
                        };
                        let a532 = 8u8;
                        if equal(a531, a532) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a538 = {
                        let a535 = {
                            let a533 = v3;
                            let a534 = 4u8;
                            fixed_u8::bit_and(a533, a534)
                        };
                        let a536 = 4u8;
                        if equal(a535, a536) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a537, a538)?
                };
                let a550 = {
                    let a547 = {
                        let a541 = {
                            let a539 = v3;
                            let a540 = 2u8;
                            fixed_u8::bit_and(a539, a540)
                        };
                        let a542 = 2u8;
                        if equal(a541, a542) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a548 = {
                        let a545 = {
                            let a543 = v3;
                            let a544 = 1u8;
                            fixed_u8::bit_and(a543, a544)
                        };
                        let a546 = 1u8;
                        if equal(a545, a546) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a547, a548)?
                };
                nat_add(a549, a550)?
            };
            nat_add(a551, a552)
        }
    }
}

pub fn f90(v0: Bytes, v1: u64) -> R<u64> {
    let (h553, _) = f38(Fn4::F113(v0.clone()), {
        let a555 = {
            let a554 = v0;
            length_bytes(a554)
        };
        let a556 = 1u64;
        nat_add(a555, a556)?
    }, v1)?;
    Ok(h553)
}

pub fn f91(v0: Bytes, v1: u64) -> R<u64> {
    let (h557, _) = f38(Fn4::F114(v0.clone()), {
        let a559 = {
            let a558 = v0;
            length_bytes(a558)
        };
        let a560 = 1u64;
        nat_add(a559, a560)?
    }, v1)?;
    Ok(h557)
}

pub fn f92(v0: Bytes) -> R<Option<u64>> {
    if f115(v0.clone())? {
        Ok(Some({
            let (_, h561) = {
                let (h562, _) = f116(Fn13::F117(v0.clone()), {
                    let a563 = v0;
                    length_bytes(a563)
                }, (0u64, 0u64))?;
                h562
            };
            h561
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f93(v0: bool, v1: u64) -> bool {
    if v0 {
        let a564 = 0u64;
        let a565 = v1;
        if nat_lt(a564, a565) {
            let a566 = v1;
            let a567 = 9223372036854775808u64;
            nat_le(a566, a567)
        } else {
            false
        }
    } else {
        let a568 = v1;
        let a569 = 9223372036854775807u64;
        nat_le(a568, a569)
    }
}

pub fn f94(v0: u64) -> bool {
    let a570 = 64u64;
    let a571 = v0;
    if if nat_lt(a570, a571) {
        let a572 = v0;
        let a573 = 91u64;
        nat_lt(a572, a573)
    } else {
        false
    } {
        true
    } else {
        let a574 = 96u64;
        let a575 = v0;
        if if nat_lt(a574, a575) {
            let a576 = v0;
            let a577 = 123u64;
            nat_lt(a576, a577)
        } else {
            false
        } {
            true
        } else {
            let a578 = v0;
            let a579 = 95u64;
            nat_eq(a578, a579)
        }
    }
}

pub fn f95(v0: Bytes, v1: u64) -> R<u64> {
    let (h580, _) = f38(Fn4::F118(v0.clone()), {
        let a582 = {
            let a581 = v0;
            length_bytes(a581)
        };
        let a583 = 1u64;
        nat_add(a582, a583)?
    }, v1)?;
    Ok(h580)
}

pub fn f96(v0: Bytes) -> bool {
    let a584 = v0.clone();
    let a585 = Bytes::lit(&[0x74, 0x72, 0x75, 0x65]);
    if equal(a584, a585) {
        true
    } else {
        let a586 = v0;
        let a587 = Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]);
        equal(a586, a587)
    }
}

pub fn f97(v0: Bytes) -> R<Bytes> {
    let v1: u64 = {
        let a588 = v0.clone();
        length_bytes(a588)
    };
    let v2: Adt8 = {
        let (h589, _) = f119(Fn14::F120(v1, v0), {
            let a590 = v1;
            let a591 = 1u64;
            nat_add(a590, a591)?
        }, Adt8::C0(0u64, List::<Bytes>::nil()))?;
        h589
    };
    Ok(f83(f83(f29(Str::lit("\"")), f27(match v2 {
        Adt8::C0(_, h592) => {
            h592
        }
    })), f29(Str::lit("\""))))
}

pub fn f98(v0: Fn9, p596: Adt0, p597: List<Adt0>) -> Adt0 {
    let m595 = (p596, p597);
    {
        let mut m595 = m595;
        loop {
            let step = {
                let (v1, v2) = m595;
                let m593 = v2;
                match m593.uncons() {
                    None => {
                        Err::<(Adt0, List<Adt0>), Adt0>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt0, List<Adt0>), Adt0>(({
                            let c594: Fn9 = v0.clone();
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

pub fn f99(v0: Adt0, v1: Adt0) -> Adt0 {
    Adt0::C6(std::rc::Rc::new(v1), std::rc::Rc::new(v0))
}

pub fn f100(v0: Fn10, p601: Adt0, p602: List<(Bytes, Adt0)>) -> Adt0 {
    let m600 = (p601, p602);
    {
        let mut m600 = m600;
        loop {
            let step = {
                let (v1, v2) = m600;
                let m598 = v2;
                match m598.uncons() {
                    None => {
                        Err::<(Adt0, List<(Bytes, Adt0)>), Adt0>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt0, List<(Bytes, Adt0)>), Adt0>(({
                            let c599: Fn10 = v0.clone();
                            c599.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m600 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f101(v0: Adt0, v1: (Bytes, Adt0)) -> Adt0 {
    Adt0::C7({
        let (h603, _) = v1;
        h603
    }, std::rc::Rc::new({
        let (_, h604) = v1;
        h604
    }), std::rc::Rc::new(v0))
}

pub fn f102(v0: List<(Bytes, Adt0)>) -> List<(Bytes, Adt0)> {
    f121(Fn15::F122, List::<(Bytes, Adt0)>::nil(), v0)
}

pub fn f103(v0: List<List<(Bytes, Adt0)>>) -> List<List<(Bytes, Adt0)>> {
    f123(Fn16::F124, List::<List<(Bytes, Adt0)>>::nil(), v0)
}

pub fn f104(v0: Fn11, p608: List<List<(Bytes, Adt0)>>, p609: List<(Bytes, Adt0)>) -> List<List<(Bytes, Adt0)>> {
    let m607 = (p608, p609);
    {
        let mut m607 = m607;
        loop {
            let step = {
                let (v1, v2) = m607;
                let m605 = v2;
                match m605.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Adt0)>>, List<(Bytes, Adt0)>), List<List<(Bytes, Adt0)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Adt0)>>, List<(Bytes, Adt0)>), List<List<(Bytes, Adt0)>>>(({
                            let c606: Fn11 = v0.clone();
                            c606.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m607 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f105(v0: List<List<(Bytes, Adt0)>>, v1: (Bytes, Adt0)) -> List<List<(Bytes, Adt0)>> {
    List::cons(List::cons(v1, List::<(Bytes, Adt0)>::nil()), v0)
}

pub fn f106(v0: Fn12, p614: u64, p615: List<List<(Bytes, Adt0)>>) -> R<(List<List<(Bytes, Adt0)>>, bool)> {
    let m613 = (p614, p615);
    {
        let mut m613 = m613;
        loop {
            let step = {
                let (v1, v2) = m613;
                let m610 = v1;
                if m610 == 0 {
                    Err::<(u64, List<List<(Bytes, Adt0)>>), R<(List<List<(Bytes, Adt0)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m610 - 1;
                    let m612 = {
                        let c611: Fn12 = v0.clone();
                        c611.apply(v2.clone())?
                    };
                    match m612 {
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
                Ok(next) => { m613 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f107(v0: List<List<(Bytes, Adt0)>>) -> R<Option<List<List<(Bytes, Adt0)>>>> {
    f125(v0)
}

pub fn f108(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m616 = f126(v0.clone(), v1)?;
    match m616 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            let m619 = f126(v0.clone(), {
                let a617 = v1;
                let a618 = 1u64;
                nat_add(a617, a618)?
            })?;
            match m619 {
                None => {
                    Ok(None::<u64>)
                }
                Some(v3) => {
                    let m622 = f126(v0.clone(), {
                        let a620 = v1;
                        let a621 = 2u64;
                        nat_add(a620, a621)?
                    })?;
                    match m622 {
                        None => {
                            Ok(None::<u64>)
                        }
                        Some(v4) => {
                            let m625 = f126(v0, {
                                let a623 = v1;
                                let a624 = 3u64;
                                nat_add(a623, a624)?
                            })?;
                            match m625 {
                                None => {
                                    Ok(None::<u64>)
                                }
                                Some(v5) => {
                                    Ok(Some({
                                        let a636 = {
                                            let a634 = {
                                                let a632 = {
                                                    let a630 = {
                                                        let a628 = {
                                                            let a626 = v2;
                                                            let a627 = 16u64;
                                                            nat_mul(a626, a627)?
                                                        };
                                                        let a629 = v3;
                                                        nat_add(a628, a629)?
                                                    };
                                                    let a631 = 16u64;
                                                    nat_mul(a630, a631)?
                                                };
                                                let a633 = v4;
                                                nat_add(a632, a633)?
                                            };
                                            let a635 = 16u64;
                                            nat_mul(a634, a635)?
                                        };
                                        let a637 = v5;
                                        nat_add(a636, a637)?
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

pub fn f109(v0: u64) -> Bytes {
    f83(f83(f127(v0, 4096u64), f127(v0, 256u64)), f83(f127(v0, 16u64), f128(v0)))
}

pub fn f110(v0: u64) -> R<Bytes> {
    let a638 = v0;
    let a639 = 128u64;
    if nat_lt(a638, a639) {
        Ok(f81(v0))
    } else {
        let a640 = v0;
        let a641 = 2048u64;
        if nat_lt(a640, a641) {
            Ok(f83(f81({
                let a645 = 192u64;
                let a646 = {
                    let a642 = v0;
                    let a643 = 64u64;
                    let a644 = 0u64;
                    nat_quot(a642, a643, a644)
                };
                nat_add(a645, a646)?
            }), f81({
                let a650 = 128u64;
                let a651 = {
                    let a647 = v0;
                    let a648 = 64u64;
                    let a649 = 0u64;
                    nat_rem(a647, a648, a649)
                };
                nat_add(a650, a651)?
            })))
        } else {
            Ok(f83(f81({
                let a655 = 224u64;
                let a656 = {
                    let a652 = v0;
                    let a653 = 4096u64;
                    let a654 = 0u64;
                    nat_quot(a652, a653, a654)
                };
                nat_add(a655, a656)?
            }), f83(f81({
                let a663 = 128u64;
                let a664 = {
                    let a660 = {
                        let a657 = v0;
                        let a658 = 64u64;
                        let a659 = 0u64;
                        nat_quot(a657, a658, a659)
                    };
                    let a661 = 64u64;
                    let a662 = 0u64;
                    nat_rem(a660, a661, a662)
                };
                nat_add(a663, a664)?
            }), f81({
                let a668 = 128u64;
                let a669 = {
                    let a665 = v0;
                    let a666 = 64u64;
                    let a667 = 0u64;
                    nat_rem(a665, a666, a667)
                };
                nat_add(a668, a669)?
            }))))
        }
    }
}

pub fn f111(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let a670 = v2;
    let a671 = v1;
    if nat_lt(a670, a671) {
        let v3: u64 = f64(v0, v2)?;
        let a672 = v3;
        let a673 = 32u64;
        if nat_lt(a672, a673) {
            Ok(None::<u64>)
        } else {
            let a674 = v3;
            let a675 = 34u64;
            if nat_eq(a674, a675) {
                Ok(None::<u64>)
            } else {
                let a676 = v3;
                let a677 = 92u64;
                if nat_eq(a676, a677) {
                    Ok(None::<u64>)
                } else {
                    Ok(Some({
                        let a678 = v2;
                        let a679 = 1u64;
                        nat_add(a678, a679)?
                    }))
                }
            }
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f112(v0: (Option<Bytes>, List<Bytes>), v1: Bytes) -> (Option<Bytes>, List<Bytes>) {
    let m681 = {
        let (h680, _) = v0;
        h680
    };
    match m681 {
        None => {
            (Some(v1), {
                let (_, h682) = v0;
                h682
            })
        }
        Some(v2) => {
            (None::<Bytes>, List::cons({
                let a683 = v2;
                let a684 = v1;
                append_bytes(a683, a684)
            }, {
                let (_, h685) = v0;
                h685
            }))
        }
    }
}

pub fn f113(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f129(v0, v1)
}

pub fn f114(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f130(v0, v1)
}

pub fn f115(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a686 = v0.clone();
        length_bytes(a686)
    };
    let a687 = 0u64;
    let a688 = v1;
    if nat_lt(a687, a688) {
        let a689 = f131(v0.clone())?;
        let a690 = v1;
        if nat_eq(a689, a690) {
            let a691 = 1u64;
            let a692 = v1;
            if nat_lt(a691, a692) {
                let m695 = {
                    let a693 = v0;
                    let a694 = 0u64;
                    index_bytes(a693, a694)
                };
                match m695 {
                    None => {
                        Ok(false)
                    }
                    Some(v2) => {
                        let a696 = v2;
                        let a697 = 48u8;
                        Ok(!equal(a696, a697))
                    }
                }
            } else {
                Ok(true)
            }
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f116(v0: Fn13, p702: u64, p703: (u64, u64)) -> R<((u64, u64), bool)> {
    let m701 = (p702, p703);
    {
        let mut m701 = m701;
        loop {
            let step = {
                let (v1, v2) = m701;
                let m698 = v1;
                if m698 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m698 - 1;
                    let m700 = {
                        let c699: Fn13 = v0.clone();
                        c699.apply(v2)?
                    };
                    match m700 {
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
                Ok(next) => { m701 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f117(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f132(v0, v1)
}

pub fn f118(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f133(v0, v1)
}

pub fn f119(v0: Fn14, p708: u64, p709: Adt8) -> R<(Adt8, bool)> {
    let m707 = (p708, p709);
    {
        let mut m707 = m707;
        loop {
            let step = {
                let (v1, v2) = m707;
                let m704 = v1;
                if m704 == 0 {
                    Err::<(u64, Adt8), R<(Adt8, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m704 - 1;
                    let m706 = {
                        let c705: Fn14 = v0.clone();
                        c705.apply(v2.clone())?
                    };
                    match m706 {
                        None => {
                            Err::<(u64, Adt8), R<(Adt8, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt8), R<(Adt8, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m707 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f120(v0: u64, v1: Bytes, v2: Adt8) -> R<Option<Adt8>> {
    f134(v1, v0, v2)
}

pub fn f121(v0: Fn15, p713: List<(Bytes, Adt0)>, p714: List<(Bytes, Adt0)>) -> List<(Bytes, Adt0)> {
    let m712 = (p713, p714);
    {
        let mut m712 = m712;
        loop {
            let step = {
                let (v1, v2) = m712;
                let m710 = v2;
                match m710.uncons() {
                    None => {
                        Err::<(List<(Bytes, Adt0)>, List<(Bytes, Adt0)>), List<(Bytes, Adt0)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Adt0)>, List<(Bytes, Adt0)>), List<(Bytes, Adt0)>>(({
                            let c711: Fn15 = v0.clone();
                            c711.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m712 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f122(v0: List<(Bytes, Adt0)>, v1: (Bytes, Adt0)) -> List<(Bytes, Adt0)> {
    List::cons(v1, v0)
}

pub fn f123(v0: Fn16, p718: List<List<(Bytes, Adt0)>>, p719: List<List<(Bytes, Adt0)>>) -> List<List<(Bytes, Adt0)>> {
    let m717 = (p718, p719);
    {
        let mut m717 = m717;
        loop {
            let step = {
                let (v1, v2) = m717;
                let m715 = v2;
                match m715.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>), List<List<(Bytes, Adt0)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>), List<List<(Bytes, Adt0)>>>(({
                            let c716: Fn16 = v0.clone();
                            c716.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m717 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f124(v0: List<List<(Bytes, Adt0)>>, v1: List<(Bytes, Adt0)>) -> List<List<(Bytes, Adt0)>> {
    List::cons(v1, v0)
}

pub fn f125(v0: List<List<(Bytes, Adt0)>>) -> R<Option<List<List<(Bytes, Adt0)>>>> {
    let a721 = {
        let a720 = v0.clone();
        length_list(a720)
    };
    let a722 = 1u64;
    if nat_le(a721, a722) {
        Ok(None::<List<List<(Bytes, Adt0)>>>)
    } else {
        Ok(Some(f135(v0)?))
    }
}

pub fn f126(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m723 = f25(v0, v1);
    match m723 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            f136(v2)
        }
    }
}

pub fn f127(v0: u64, v1: u64) -> Bytes {
    f128({
        let a724 = v0;
        let a725 = v1;
        let a726 = 0u64;
        nat_quot(a724, a725, a726)
    })
}

pub fn f128(v0: u64) -> Bytes {
    f31(Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66]), {
        let a727 = v0;
        let a728 = 16u64;
        let a729 = 0u64;
        nat_rem(a727, a728, a729)
    }, 1u64)
}

pub fn f129(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f89(v0, v1)?;
    let a730 = 47u64;
    let a731 = v2;
    if nat_lt(a730, a731) {
        let a732 = v2;
        let a733 = 58u64;
        if nat_lt(a732, a733) {
            Ok(Some({
                let a734 = v1;
                let a735 = 1u64;
                nat_add(a734, a735)?
            }))
        } else {
            Ok(None::<u64>)
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f130(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a736 = f89(v0, v1)?;
    let a737 = 48u64;
    if nat_eq(a736, a737) {
        Ok(Some({
            let a738 = v1;
            let a739 = 1u64;
            nat_add(a738, a739)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f131(v0: Bytes) -> R<u64> {
    let (h740, _) = f38(Fn4::F137(v0.clone()), {
        let a741 = v0;
        length_bytes(a741)
    }, 0u64)?;
    Ok(h740)
}

pub fn f132(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m745 = {
        let a743 = v0;
        let a744 = {
            let (h742, _) = v1;
            h742
        };
        index_bytes(a743, a744)
    };
    match m745 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a747 = {
                    let (h746, _) = v1;
                    h746
                };
                let a748 = 1u64;
                nat_add(a747, a748)?
            }, {
                let a752 = {
                    let a750 = {
                        let (_, h749) = v1;
                        h749
                    };
                    let a751 = 10u64;
                    nat_mul(a750, a751)?
                };
                let a753 = f138(v2)?;
                nat_add(a752, a753)?
            })))
        }
    }
}

pub fn f133(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f139(f89(v0, v1)?) {
        Ok(Some({
            let a754 = v1;
            let a755 = 1u64;
            nat_add(a754, a755)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f134(v0: Bytes, v1: u64, v2: Adt8) -> R<Option<Adt8>> {
    let v3: u64 = match v2.clone() {
        Adt8::C0(h756, _) => {
            h756
        }
    };
    let a757 = v3;
    let a758 = v1;
    if nat_lt(a757, a758) {
        let v4: u64 = f64(v0.clone(), v3)?;
        let a759 = v4;
        let a760 = 32u64;
        if if nat_lt(a759, a760) {
            true
        } else {
            let a761 = v4;
            let a762 = 34u64;
            if nat_eq(a761, a762) {
                true
            } else {
                let a763 = v4;
                let a764 = 92u64;
                nat_eq(a763, a764)
            }
        } {
            Ok(Some(Adt8::C0({
                let a765 = v3;
                let a766 = 1u64;
                nat_add(a765, a766)?
            }, List::cons(f140(v4), match v2 {
                Adt8::C0(_, h767) => {
                    h767
                }
            }))))
        } else {
            let v5: u64 = f66(v0.clone(), v1, v3)?;
            Ok(Some(Adt8::C0(v5, List::cons(f31(v0, v3, {
                let a768 = v5;
                let a769 = v3;
                nat_sub(a768, a769)
            }), match v2 {
                Adt8::C0(_, h770) => {
                    h770
                }
            }))))
        }
    } else {
        Ok(None::<Adt8>)
    }
}

pub fn f135(v0: List<List<(Bytes, Adt0)>>) -> R<List<List<(Bytes, Adt0)>>> {
    Ok(f103({
        let (_, h771) = {
            let (h772, _) = f141(Fn17::F142, {
                let a774 = {
                    let a773 = v0.clone();
                    length_list(a773)
                };
                let a775 = 1u64;
                nat_add(a774, a775)?
            }, (v0, List::<List<(Bytes, Adt0)>>::nil()))?;
            h772
        };
        h771
    }))
}

pub fn f136(v0: u8) -> R<Option<u64>> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a820 = {
            let a796 = {
                let a784 = {
                    let a778 = {
                        let a776 = v1;
                        let a777 = 128u8;
                        fixed_u8::bit_and(a776, a777)
                    };
                    let a779 = 128u8;
                    if equal(a778, a779) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a785 = {
                    let a782 = {
                        let a780 = v1;
                        let a781 = 64u8;
                        fixed_u8::bit_and(a780, a781)
                    };
                    let a783 = 64u8;
                    if equal(a782, a783) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a784, a785)?
            };
            let a797 = {
                let a794 = {
                    let a788 = {
                        let a786 = v1;
                        let a787 = 32u8;
                        fixed_u8::bit_and(a786, a787)
                    };
                    let a789 = 32u8;
                    if equal(a788, a789) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a795 = {
                    let a792 = {
                        let a790 = v1;
                        let a791 = 16u8;
                        fixed_u8::bit_and(a790, a791)
                    };
                    let a793 = 16u8;
                    if equal(a792, a793) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a794, a795)?
            };
            nat_add(a796, a797)?
        };
        let a821 = {
            let a818 = {
                let a806 = {
                    let a800 = {
                        let a798 = v1;
                        let a799 = 8u8;
                        fixed_u8::bit_and(a798, a799)
                    };
                    let a801 = 8u8;
                    if equal(a800, a801) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a807 = {
                    let a804 = {
                        let a802 = v1;
                        let a803 = 4u8;
                        fixed_u8::bit_and(a802, a803)
                    };
                    let a805 = 4u8;
                    if equal(a804, a805) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a806, a807)?
            };
            let a819 = {
                let a816 = {
                    let a810 = {
                        let a808 = v1;
                        let a809 = 2u8;
                        fixed_u8::bit_and(a808, a809)
                    };
                    let a811 = 2u8;
                    if equal(a810, a811) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a817 = {
                    let a814 = {
                        let a812 = v1;
                        let a813 = 1u8;
                        fixed_u8::bit_and(a812, a813)
                    };
                    let a815 = 1u8;
                    if equal(a814, a815) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a816, a817)?
            };
            nat_add(a818, a819)?
        };
        nat_add(a820, a821)?
    };
    let a822 = v2;
    let a823 = 48u64;
    if nat_lt(a822, a823) {
        Ok(None::<u64>)
    } else {
        let a824 = v2;
        let a825 = 57u64;
        if nat_le(a824, a825) {
            Ok(Some({
                let a826 = v2;
                let a827 = 48u64;
                nat_sub(a826, a827)
            }))
        } else {
            let a828 = v2;
            let a829 = 65u64;
            if nat_lt(a828, a829) {
                Ok(None::<u64>)
            } else {
                let a830 = v2;
                let a831 = 70u64;
                if nat_le(a830, a831) {
                    Ok(Some({
                        let a832 = v2;
                        let a833 = 55u64;
                        nat_sub(a832, a833)
                    }))
                } else {
                    let a834 = v2;
                    let a835 = 97u64;
                    if nat_lt(a834, a835) {
                        Ok(None::<u64>)
                    } else {
                        let a836 = v2;
                        let a837 = 102u64;
                        if nat_le(a836, a837) {
                            Ok(Some({
                                let a838 = v2;
                                let a839 = 87u64;
                                nat_sub(a838, a839)
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

pub fn f137(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m842 = {
        let a840 = v0;
        let a841 = v1;
        index_bytes(a840, a841)
    };
    match m842 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f143(v2)? {
                Ok(Some({
                    let a843 = v1;
                    let a844 = 1u64;
                    nat_add(a843, a844)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f138(v0: u8) -> R<u64> {
    let a891 = {
        let v1: u8 = v0;
        let a889 = {
            let a865 = {
                let a853 = {
                    let a847 = {
                        let a845 = v1;
                        let a846 = 128u8;
                        fixed_u8::bit_and(a845, a846)
                    };
                    let a848 = 128u8;
                    if equal(a847, a848) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a854 = {
                    let a851 = {
                        let a849 = v1;
                        let a850 = 64u8;
                        fixed_u8::bit_and(a849, a850)
                    };
                    let a852 = 64u8;
                    if equal(a851, a852) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a853, a854)?
            };
            let a866 = {
                let a863 = {
                    let a857 = {
                        let a855 = v1;
                        let a856 = 32u8;
                        fixed_u8::bit_and(a855, a856)
                    };
                    let a858 = 32u8;
                    if equal(a857, a858) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a864 = {
                    let a861 = {
                        let a859 = v1;
                        let a860 = 16u8;
                        fixed_u8::bit_and(a859, a860)
                    };
                    let a862 = 16u8;
                    if equal(a861, a862) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a863, a864)?
            };
            nat_add(a865, a866)?
        };
        let a890 = {
            let a887 = {
                let a875 = {
                    let a869 = {
                        let a867 = v1;
                        let a868 = 8u8;
                        fixed_u8::bit_and(a867, a868)
                    };
                    let a870 = 8u8;
                    if equal(a869, a870) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a876 = {
                    let a873 = {
                        let a871 = v1;
                        let a872 = 4u8;
                        fixed_u8::bit_and(a871, a872)
                    };
                    let a874 = 4u8;
                    if equal(a873, a874) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a875, a876)?
            };
            let a888 = {
                let a885 = {
                    let a879 = {
                        let a877 = v1;
                        let a878 = 2u8;
                        fixed_u8::bit_and(a877, a878)
                    };
                    let a880 = 2u8;
                    if equal(a879, a880) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a886 = {
                    let a883 = {
                        let a881 = v1;
                        let a882 = 1u8;
                        fixed_u8::bit_and(a881, a882)
                    };
                    let a884 = 1u8;
                    if equal(a883, a884) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a885, a886)?
            };
            nat_add(a887, a888)?
        };
        nat_add(a889, a890)?
    };
    let a892 = 48u64;
    Ok(nat_sub(a891, a892))
}

pub fn f139(v0: u64) -> bool {
    if f94(v0) {
        true
    } else {
        let a893 = 47u64;
        let a894 = v0;
        if if nat_lt(a893, a894) {
            let a895 = v0;
            let a896 = 58u64;
            nat_lt(a895, a896)
        } else {
            false
        } {
            true
        } else {
            let a897 = v0;
            let a898 = 46u64;
            if nat_eq(a897, a898) {
                true
            } else {
                let a899 = v0;
                let a900 = 39u64;
                if nat_eq(a899, a900) {
                    true
                } else {
                    let a901 = v0;
                    let a902 = 47u64;
                    if nat_eq(a901, a902) {
                        true
                    } else {
                        let a903 = v0;
                        let a904 = 45u64;
                        nat_eq(a903, a904)
                    }
                }
            }
        }
    }
}

pub fn f140(v0: u64) -> Bytes {
    let a905 = v0;
    let a906 = 34u64;
    if nat_eq(a905, a906) {
        f29(Str::lit("\\\""))
    } else {
        let a907 = v0;
        let a908 = 92u64;
        if nat_eq(a907, a908) {
            f29(Str::lit("\\\\"))
        } else {
            let a909 = v0;
            let a910 = 8u64;
            if nat_eq(a909, a910) {
                f29(Str::lit("\\b"))
            } else {
                let a911 = v0;
                let a912 = 12u64;
                if nat_eq(a911, a912) {
                    f29(Str::lit("\\f"))
                } else {
                    let a913 = v0;
                    let a914 = 10u64;
                    if nat_eq(a913, a914) {
                        f29(Str::lit("\\n"))
                    } else {
                        let a915 = v0;
                        let a916 = 13u64;
                        if nat_eq(a915, a916) {
                            f29(Str::lit("\\r"))
                        } else {
                            let a917 = v0;
                            let a918 = 9u64;
                            if nat_eq(a917, a918) {
                                f29(Str::lit("\\t"))
                            } else {
                                f83(f29(Str::lit("\\u00")), f144(v0))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f141(v0: Fn17, p923: u64, p924: (List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>)) -> R<((List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>), bool)> {
    let m922 = (p923, p924);
    {
        let mut m922 = m922;
        loop {
            let step = {
                let (v1, v2) = m922;
                let m919 = v1;
                if m919 == 0 {
                    Err::<(u64, (List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>)), R<((List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m919 - 1;
                    let m921 = {
                        let c920: Fn17 = v0.clone();
                        c920.apply(v2.clone())?
                    };
                    match m921 {
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
                Ok(next) => { m922 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f142(v0: (List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>)) -> R<Option<(List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>)>> {
    f145(v0)
}

pub fn f143(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a969 = {
            let a945 = {
                let a933 = {
                    let a927 = {
                        let a925 = v1;
                        let a926 = 128u8;
                        fixed_u8::bit_and(a925, a926)
                    };
                    let a928 = 128u8;
                    if equal(a927, a928) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a934 = {
                    let a931 = {
                        let a929 = v1;
                        let a930 = 64u8;
                        fixed_u8::bit_and(a929, a930)
                    };
                    let a932 = 64u8;
                    if equal(a931, a932) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a933, a934)?
            };
            let a946 = {
                let a943 = {
                    let a937 = {
                        let a935 = v1;
                        let a936 = 32u8;
                        fixed_u8::bit_and(a935, a936)
                    };
                    let a938 = 32u8;
                    if equal(a937, a938) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a944 = {
                    let a941 = {
                        let a939 = v1;
                        let a940 = 16u8;
                        fixed_u8::bit_and(a939, a940)
                    };
                    let a942 = 16u8;
                    if equal(a941, a942) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a943, a944)?
            };
            nat_add(a945, a946)?
        };
        let a970 = {
            let a967 = {
                let a955 = {
                    let a949 = {
                        let a947 = v1;
                        let a948 = 8u8;
                        fixed_u8::bit_and(a947, a948)
                    };
                    let a950 = 8u8;
                    if equal(a949, a950) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a956 = {
                    let a953 = {
                        let a951 = v1;
                        let a952 = 4u8;
                        fixed_u8::bit_and(a951, a952)
                    };
                    let a954 = 4u8;
                    if equal(a953, a954) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a955, a956)?
            };
            let a968 = {
                let a965 = {
                    let a959 = {
                        let a957 = v1;
                        let a958 = 2u8;
                        fixed_u8::bit_and(a957, a958)
                    };
                    let a960 = 2u8;
                    if equal(a959, a960) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a966 = {
                    let a963 = {
                        let a961 = v1;
                        let a962 = 1u8;
                        fixed_u8::bit_and(a961, a962)
                    };
                    let a964 = 1u8;
                    if equal(a963, a964) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a965, a966)?
            };
            nat_add(a967, a968)?
        };
        nat_add(a969, a970)?
    };
    let a971 = 48u64;
    let a972 = v2;
    if nat_le(a971, a972) {
        let a973 = v2;
        let a974 = 57u64;
        Ok(nat_le(a973, a974))
    } else {
        Ok(false)
    }
}

pub fn f144(v0: u64) -> Bytes {
    f83(f127(v0, 16u64), f128(v0))
}

pub fn f145(v0: (List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>)) -> R<Option<(List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>)>> {
    let v1: List<List<(Bytes, Adt0)>> = {
        let (h975, _) = v0;
        h975
    };
    let v2: List<List<(Bytes, Adt0)>> = {
        let (_, h976) = v0;
        h976
    };
    let m977 = v1;
    match m977.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Adt0)>>, List<List<(Bytes, Adt0)>>)>)
        }
        Some((v3, v4)) => {
            let m978 = v4;
            match m978.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Adt0)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f146(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f146(v0: List<(Bytes, Adt0)>, v1: List<(Bytes, Adt0)>) -> R<List<(Bytes, Adt0)>> {
    let v2: u64 = {
        let a983 = {
            let a981 = {
                let a979 = v0.clone();
                length_list(a979)
            };
            let a982 = {
                let a980 = v1.clone();
                length_list(a980)
            };
            nat_add(a981, a982)?
        };
        let a984 = 1u64;
        nat_add(a983, a984)?
    };
    Ok(f102({
        let (_, h985) = {
            let (_, h986) = {
                let (h987, _) = f147(Fn18::F148, v2, (v0, (v1, List::<(Bytes, Adt0)>::nil())));
                h987
            };
            h986
        };
        h985
    }))
}

pub fn f147(v0: Fn18, p992: u64, p993: (List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>))) -> ((List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>)), bool) {
    let m991 = (p992, p993);
    {
        let mut m991 = m991;
        loop {
            let step = {
                let (v1, v2) = m991;
                let m988 = v1;
                if m988 == 0 {
                    Err::<(u64, (List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>))), ((List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>)), bool)>((v2, false))
                } else {
                    let v3 = m988 - 1;
                    let m990 = {
                        let c989: Fn18 = v0.clone();
                        c989.apply(v2.clone())
                    };
                    match m990 {
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
                Ok(next) => { m991 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f148(v0: (List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>))) -> Option<(List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>))> {
    f149(v0)
}

pub fn f149(v0: (List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>))) -> Option<(List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>))> {
    let v1: List<(Bytes, Adt0)> = {
        let (h994, _) = v0.clone();
        h994
    };
    let v2: List<(Bytes, Adt0)> = {
        let (h995, _) = {
            let (_, h996) = v0.clone();
            h996
        };
        h995
    };
    let v3: List<(Bytes, Adt0)> = {
        let (_, h997) = {
            let (_, h998) = v0;
            h998
        };
        h997
    };
    let m999 = v1.clone();
    match m999.uncons() {
        None => {
            let m1000 = v2;
            match m1000.uncons() {
                None => {
                    None::<(List<(Bytes, Adt0)>, (List<(Bytes, Adt0)>, List<(Bytes, Adt0)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Adt0)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m1001 = v2.clone();
            match m1001.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Adt0)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f150({
                        let (h1002, _) = v8.clone();
                        h1002
                    }, {
                        let (h1003, _) = v6.clone();
                        h1003
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

pub fn f150(v0: Bytes, v1: Bytes) -> bool {
    let a1008 = {
        let a1004 = v0;
        let a1005 = v1;
        compare_bytes(a1004, a1005)
    };
    let a1009 = {
        let a1006 = Bytes::lit(&[]);
        let a1007 = Bytes::lit(&[0x01]);
        compare_bytes(a1006, a1007)
    };
    equal(a1008, a1009)
}

pub fn run(p0: Bytes) -> R<Result<Adt0, Adt1>> {
    f0(p0)
}
