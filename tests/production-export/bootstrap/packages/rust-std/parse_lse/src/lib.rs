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
    C0(u64, Bytes),
}

#[derive(Clone)]
pub enum Adt3 {
    C0,
    C1,
    C2(Bytes),
}

#[derive(Clone)]
pub enum Adt4 {
    C0(u64, u64, bool, List<Adt3>, Option<Adt2>),
}

#[derive(Clone)]
pub enum Adt5 {
    C0(Bytes),
    C1(std::rc::Rc<Adt5>),
    C2,
    C3(std::rc::Rc<Adt5>, std::rc::Rc<Adt5>),
}

#[derive(Clone)]
pub enum Adt6 {
    C0(u64, List<List<Adt5>>, Option<Adt5>, Option<Adt2>, bool),
}

#[derive(Clone)]
pub enum Adt7 {
    C0(List<Adt8>, List<Adt0>, Option<Adt2>),
}

#[derive(Clone)]
pub enum Adt8 {
    C0(Adt5),
    C1(Adt5),
}

#[derive(Clone)]
pub enum Adt9 {
    C0(u64, Bytes, Adt5),
}

#[derive(Clone)]
pub enum Adt10 {
    C0(List<Adt11>, List<Adt1>, Option<Adt2>),
}

#[derive(Clone)]
pub enum Adt11 {
    C0(Adt5),
    C1(Bytes, u64),
}

#[derive(Clone)]
pub enum Adt12 {
    C0(u64, bool, bool, bool),
}

#[derive(Clone)]
pub enum Fn0 {
    F5(u64, Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: Adt4) -> R<Option<Adt4>> {
        match self {
            Fn0::F5(k0, k1) => f5(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F9(u64, List<Adt3>),
}

impl Fn1 {
    pub fn apply(&self, p0: Adt6) -> R<Option<Adt6>> {
        match self {
            Fn1::F9(k0, k1) => f9(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F13(u64),
}

impl Fn2 {
    pub fn apply(&self, p0: Adt7) -> R<Option<Adt7>> {
        match self {
            Fn2::F13(k0) => f13(*k0, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F16,
}

impl Fn3 {
    pub fn apply(&self, p0: List<Adt3>, p1: Adt3) -> List<Adt3> {
        match self {
            Fn3::F16 => f16(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F44,
}

impl Fn4 {
    pub fn apply(&self, p0: Adt5, p1: Adt5) -> Adt5 {
        match self {
            Fn4::F44 => f44(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F47,
}

impl Fn5 {
    pub fn apply(&self, p0: (Adt5, List<Adt5>)) -> Option<(Adt5, List<Adt5>)> {
        match self {
            Fn5::F47 => f47(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F55,
}

impl Fn6 {
    pub fn apply(&self, p0: (List<Adt0>, List<Adt0>)) -> Option<(List<Adt0>, List<Adt0>)> {
        match self {
            Fn6::F55 => f55(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F57,
}

impl Fn7 {
    pub fn apply(&self, p0: Adt0, p1: Adt0) -> Adt0 {
        match self {
            Fn7::F57 => f57(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F59,
}

impl Fn8 {
    pub fn apply(&self, p0: List<Adt0>, p1: Adt0) -> List<Adt0> {
        match self {
            Fn8::F59 => f59(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F61(u64),
}

impl Fn9 {
    pub fn apply(&self, p0: (List<Adt0>, Adt0), p1: Adt5) -> R<(List<Adt0>, Adt0)> {
        match self {
            Fn9::F61(k0) => f61(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F64(u64, Bytes),
}

impl Fn10 {
    pub fn apply(&self, p0: (u64, Bytes)) -> Option<(u64, Bytes)> {
        match self {
            Fn10::F64(k0, k1) => f64(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F66,
}

impl Fn11 {
    pub fn apply(&self, p0: List<Adt5>, p1: Adt5) -> List<Adt5> {
        match self {
            Fn11::F66 => f66(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F73,
}

impl Fn12 {
    pub fn apply(&self, p0: List<Adt8>, p1: Adt5) -> List<Adt8> {
        match self {
            Fn12::F73 => f73(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F85(u64),
}

impl Fn13 {
    pub fn apply(&self, p0: Result<List<Adt8>, Adt2>, p1: Adt5) -> R<Result<List<Adt8>, Adt2>> {
        match self {
            Fn13::F85(k0) => f85(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F88(u64, Bytes),
    F103(Bytes, u64, Bytes, u64),
    F110(u64, Bytes),
    F135(Bytes),
}

impl Fn14 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn14::F88(k0, k1) => f88(*k0, k1.clone(), p0),
            Fn14::F103(k0, k1, k2, k3) => f103(k0.clone(), *k1, k2.clone(), *k3, p0),
            Fn14::F110(k0, k1) => f110(*k0, k1.clone(), p0),
            Fn14::F135(k0) => f135(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F91(u64),
}

impl Fn15 {
    pub fn apply(&self, p0: Adt10) -> R<Option<Adt10>> {
        match self {
            Fn15::F91(k0) => f91(*k0, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F96(u64),
}

impl Fn16 {
    pub fn apply(&self, p0: Result<List<Adt1>, Adt2>, p1: Adt5) -> R<Result<List<Adt1>, Adt2>> {
        match self {
            Fn16::F96(k0) => f96(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F107,
}

impl Fn17 {
    pub fn apply(&self, p0: Adt1, p1: Adt1) -> Adt1 {
        match self {
            Fn17::F107 => f107(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F109,
}

impl Fn18 {
    pub fn apply(&self, p0: List<Adt1>, p1: Adt1) -> List<Adt1> {
        match self {
            Fn18::F109 => f109(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F115(bool, u64, Bytes),
}

impl Fn19 {
    pub fn apply(&self, p0: Adt12) -> R<Option<Adt12>> {
        match self {
            Fn19::F115(k0, k1, k2) => f115(*k0, *k1, k2.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn20 {
    F125,
}

impl Fn20 {
    pub fn apply(&self, p0: List<Adt11>, p1: Adt5) -> List<Adt11> {
        match self {
            Fn20::F125 => f125(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn21 {
    F127,
}

impl Fn21 {
    pub fn apply(&self, p0: (List<Adt1>, List<Adt1>)) -> Option<(List<Adt1>, List<Adt1>)> {
        match self {
            Fn21::F127 => f127(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn22 {
    F132(Bytes),
}

impl Fn22 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn22::F132(k0) => f132(k0.clone(), p0),
        }
    }
}

pub fn f0(v0: Bytes, v1: u64) -> R<Result<Adt0, Adt2>> {
    let m1 = f1(v0)?;
    match m1 {
        Ok(v3) => {
            let m2 = f2(v3.clone(), v1)?;
            match m2 {
                Ok(v5) => {
                    f3(v5, {
                        let a3 = v3;
                        length_list(a3)
                    })
                }
                Err(v4) => {
                    Ok(Err::<Adt0, Adt2>(v4))
                }
            }
        }
        Err(v2) => {
            Ok(Err::<Adt0, Adt2>(v2))
        }
    }
}

pub fn f1(v0: Bytes) -> R<Result<List<Adt3>, Adt2>> {
    let v1: u64 = {
        let a4 = v0.clone();
        length_bytes(a4)
    };
    let v2: Adt4 = {
        let (h5, _) = f4(Fn0::F5(v1, v0.clone()), {
            let a6 = v1;
            let a7 = 1u64;
            nat_add(a6, a7)?
        }, Adt4::C0(0u64, 0u64, false, List::<Adt3>::nil(), None::<Adt2>))?;
        h5
    };
    let m9 = match v2.clone() {
        Adt4::C0(_, _, _, _, h8) => {
            h8
        }
    };
    match m9 {
        None => {
            Ok(Ok::<List<Adt3>, Adt2>(f6(f7(v0, v2))))
        }
        Some(v3) => {
            Ok(Err::<List<Adt3>, Adt2>(v3))
        }
    }
}

pub fn f2(v0: List<Adt3>, v1: u64) -> R<Result<Adt5, Adt2>> {
    let v2: Adt6 = {
        let (h10, _) = f8(Fn1::F9(v1, v0.clone()), {
            let a12 = {
                let a11 = v0;
                length_list(a11)
            };
            let a13 = 2u64;
            nat_add(a12, a13)?
        }, Adt6::C0(0u64, List::<List<Adt5>>::nil(), None::<Adt5>, None::<Adt2>, false))?;
        h10
    };
    let m15 = match v2.clone() {
        Adt6::C0(_, _, _, h14, _) => {
            h14
        }
    };
    match m15 {
        None => {
            let m17 = match v2 {
                Adt6::C0(_, _, h16, _, _) => {
                    h16
                }
            };
            match m17 {
                None => {
                    Ok(Err::<Adt5, Adt2>(f10(f11(Str::lit("unexpected end of expression")))))
                }
                Some(v4) => {
                    Ok(Ok::<Adt5, Adt2>(v4))
                }
            }
        }
        Some(v3) => {
            Ok(Err::<Adt5, Adt2>(v3))
        }
    }
}

pub fn f3(v0: Adt5, v1: u64) -> R<Result<Adt0, Adt2>> {
    let v2: Adt7 = {
        let (h18, _) = f12(Fn2::F13(v1), {
            let a21 = {
                let a19 = v1;
                let a20 = 2u64;
                nat_mul(a19, a20)?
            };
            let a22 = 2u64;
            nat_add(a21, a22)?
        }, Adt7::C0(List::cons(Adt8::C0(v0), List::<Adt8>::nil()), List::<Adt0>::nil(), None::<Adt2>))?;
        h18
    };
    let m24 = match v2.clone() {
        Adt7::C0(_, _, h23) => {
            h23
        }
    };
    match m24 {
        None => {
            let m26 = match v2 {
                Adt7::C0(_, h25, _) => {
                    h25
                }
            };
            match m26.uncons() {
                None => {
                    Ok(Err::<Adt0, Adt2>(f10(f11(Str::lit("malformed expression")))))
                }
                Some((v4, v5)) => {
                    let m27 = v5;
                    match m27.uncons() {
                        None => {
                            Ok(Ok::<Adt0, Adt2>(v4))
                        }
                        Some((_, _)) => {
                            Ok(Err::<Adt0, Adt2>(f10(f11(Str::lit("malformed expression")))))
                        }
                    }
                }
            }
        }
        Some(v3) => {
            Ok(Err::<Adt0, Adt2>(v3))
        }
    }
}

pub fn f4(v0: Fn0, p32: u64, p33: Adt4) -> R<(Adt4, bool)> {
    let m31 = (p32, p33);
    {
        let mut m31 = m31;
        loop {
            let step = {
                let (v1, v2) = m31;
                let m28 = v1;
                if m28 == 0 {
                    Err::<(u64, Adt4), R<(Adt4, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m28 - 1;
                    let m30 = {
                        let c29: Fn0 = v0.clone();
                        c29.apply(v2.clone())?
                    };
                    match m30 {
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
                Ok(next) => { m31 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f5(v0: u64, v1: Bytes, v2: Adt4) -> R<Option<Adt4>> {
    f14(v1, v0, v2)
}

pub fn f6(v0: List<Adt3>) -> List<Adt3> {
    f15(Fn3::F16, List::<Adt3>::nil(), v0)
}

pub fn f7(v0: Bytes, v1: Adt4) -> List<Adt3> {
    if match v1.clone() {
        Adt4::C0(_, _, h34, _, _) => {
            h34
        }
    } {
        List::cons(Adt3::C2(f17(v0, match v1.clone() {
            Adt4::C0(_, h35, _, _, _) => {
                h35
            }
        }, {
            let a38 = match v1.clone() {
                Adt4::C0(h36, _, _, _, _) => {
                    h36
                }
            };
            let a39 = match v1.clone() {
                Adt4::C0(_, h37, _, _, _) => {
                    h37
                }
            };
            nat_sub(a38, a39)
        })), match v1 {
            Adt4::C0(_, _, _, h40, _) => {
                h40
            }
        })
    } else {
        match v1 {
            Adt4::C0(_, _, _, h41, _) => {
                h41
            }
        }
    }
}

pub fn f8(v0: Fn1, p46: u64, p47: Adt6) -> R<(Adt6, bool)> {
    let m45 = (p46, p47);
    {
        let mut m45 = m45;
        loop {
            let step = {
                let (v1, v2) = m45;
                let m42 = v1;
                if m42 == 0 {
                    Err::<(u64, Adt6), R<(Adt6, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m42 - 1;
                    let m44 = {
                        let c43: Fn1 = v0.clone();
                        c43.apply(v2.clone())?
                    };
                    match m44 {
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
                Ok(next) => { m45 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f9(v0: u64, v1: List<Adt3>, v2: Adt6) -> R<Option<Adt6>> {
    f18(v1, v0, v2)
}

pub fn f10(v0: Bytes) -> Adt2 {
    Adt2::C0(0u64, v0)
}

pub fn f11(v0: Str) -> Bytes {
    let a48 = v0;
    utf8_encode(a48)
}

pub fn f12(v0: Fn2, p53: u64, p54: Adt7) -> R<(Adt7, bool)> {
    let m52 = (p53, p54);
    {
        let mut m52 = m52;
        loop {
            let step = {
                let (v1, v2) = m52;
                let m49 = v1;
                if m49 == 0 {
                    Err::<(u64, Adt7), R<(Adt7, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m49 - 1;
                    let m51 = {
                        let c50: Fn2 = v0.clone();
                        c50.apply(v2.clone())?
                    };
                    match m51 {
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
                Ok(next) => { m52 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f13(v0: u64, v1: Adt7) -> R<Option<Adt7>> {
    f19(v0, v1)
}

pub fn f14(v0: Bytes, v1: u64, v2: Adt4) -> R<Option<Adt4>> {
    let m56 = match v2.clone() {
        Adt4::C0(_, _, _, _, h55) => {
            h55
        }
    };
    match m56 {
        None => {
            let v4: u64 = match v2.clone() {
                Adt4::C0(h57, _, _, _, _) => {
                    h57
                }
            };
            let a58 = v4;
            let a59 = v1;
            if nat_lt(a58, a59) {
                let v5: u64 = f20(v0.clone(), v4)?;
                let a60 = v5;
                let a61 = 40u64;
                if nat_eq(a60, a61) {
                    Ok(Some(Adt4::C0({
                        let a62 = v4;
                        let a63 = 1u64;
                        nat_add(a62, a63)?
                    }, match v2.clone() {
                        Adt4::C0(_, h64, _, _, _) => {
                            h64
                        }
                    }, false, List::cons(Adt3::C0, f7(v0, v2)), None::<Adt2>)))
                } else {
                    let a65 = v5;
                    let a66 = 41u64;
                    if nat_eq(a65, a66) {
                        Ok(Some(Adt4::C0({
                            let a67 = v4;
                            let a68 = 1u64;
                            nat_add(a67, a68)?
                        }, match v2.clone() {
                            Adt4::C0(_, h69, _, _, _) => {
                                h69
                            }
                        }, false, List::cons(Adt3::C1, f7(v0, v2)), None::<Adt2>)))
                    } else {
                        let a70 = v5;
                        let a71 = 32u64;
                        if if nat_eq(a70, a71) {
                            true
                        } else {
                            let a72 = v5;
                            let a73 = 10u64;
                            nat_eq(a72, a73)
                        } {
                            Ok(Some(Adt4::C0({
                                let a74 = v4;
                                let a75 = 1u64;
                                nat_add(a74, a75)?
                            }, match v2.clone() {
                                Adt4::C0(_, h76, _, _, _) => {
                                    h76
                                }
                            }, false, f7(v0, v2), None::<Adt2>)))
                        } else {
                            let a77 = v5;
                            let a78 = 32u64;
                            if if nat_lt(a77, a78) {
                                true
                            } else {
                                let a79 = v5;
                                let a80 = 127u64;
                                nat_eq(a79, a80)
                            } {
                                Ok(Some(Adt4::C0(v4, match v2.clone() {
                                    Adt4::C0(_, h81, _, _, _) => {
                                        h81
                                    }
                                }, match v2.clone() {
                                    Adt4::C0(_, _, h82, _, _) => {
                                        h82
                                    }
                                }, match v2 {
                                    Adt4::C0(_, _, _, h83, _) => {
                                        h83
                                    }
                                }, Some(f10(f11(Str::lit("whitespace is one or more ASCII spaces or LF")))))))
                            } else {
                                if match v2.clone() {
                                    Adt4::C0(_, _, h84, _, _) => {
                                        h84
                                    }
                                } {
                                    Ok(Some(Adt4::C0({
                                        let a85 = v4;
                                        let a86 = 1u64;
                                        nat_add(a85, a86)?
                                    }, match v2.clone() {
                                        Adt4::C0(_, h87, _, _, _) => {
                                            h87
                                        }
                                    }, true, match v2 {
                                        Adt4::C0(_, _, _, h88, _) => {
                                            h88
                                        }
                                    }, None::<Adt2>)))
                                } else {
                                    Ok(Some(Adt4::C0({
                                        let a89 = v4;
                                        let a90 = 1u64;
                                        nat_add(a89, a90)?
                                    }, v4, true, match v2 {
                                        Adt4::C0(_, _, _, h91, _) => {
                                            h91
                                        }
                                    }, None::<Adt2>)))
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

pub fn f15(v0: Fn3, p95: List<Adt3>, p96: List<Adt3>) -> List<Adt3> {
    let m94 = (p95, p96);
    {
        let mut m94 = m94;
        loop {
            let step = {
                let (v1, v2) = m94;
                let m92 = v2;
                match m92.uncons() {
                    None => {
                        Err::<(List<Adt3>, List<Adt3>), List<Adt3>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt3>, List<Adt3>), List<Adt3>>(({
                            let c93: Fn3 = v0.clone();
                            c93.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m94 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f16(v0: List<Adt3>, v1: Adt3) -> List<Adt3> {
    List::cons(v1, v0)
}

pub fn f17(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m100 = {
        let a97 = v0;
        let a98 = v1;
        let a99 = v2;
        slice_bytes(a97, a98, a99)
    };
    match m100 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f18(v0: List<Adt3>, v1: u64, v2: Adt6) -> R<Option<Adt6>> {
    let m102 = match v2.clone() {
        Adt6::C0(_, _, _, h101, _) => {
            h101
        }
    };
    match m102 {
        None => {
            if match v2.clone() {
                Adt6::C0(_, _, _, _, h103) => {
                    h103
                }
            } {
                Ok(None::<Adt6>)
            } else {
                let m107 = {
                    let a105 = v0;
                    let a106 = match v2.clone() {
                        Adt6::C0(h104, _, _, _, _) => {
                            h104
                        }
                    };
                    index_list(a105, a106)
                };
                match m107 {
                    None => {
                        let m109 = match v2.clone() {
                            Adt6::C0(_, h108, _, _, _) => {
                                h108
                            }
                        };
                        match m109.uncons() {
                            None => {
                                let m111 = match v2.clone() {
                                    Adt6::C0(_, _, h110, _, _) => {
                                        h110
                                    }
                                };
                                match m111 {
                                    None => {
                                        Ok(Some(f21(v2, f10(f11(Str::lit("unexpected end of expression"))))))
                                    }
                                    Some(_) => {
                                        Ok(Some(Adt6::C0(match v2.clone() {
                                            Adt6::C0(h112, _, _, _, _) => {
                                                h112
                                            }
                                        }, List::<List<Adt5>>::nil(), match v2 {
                                            Adt6::C0(_, _, h113, _, _) => {
                                                h113
                                            }
                                        }, None::<Adt2>, true)))
                                    }
                                }
                            }
                            Some((_, _)) => {
                                Ok(Some(f21(v2, f10(f11(Str::lit("unexpected end of expression"))))))
                            }
                        }
                    }
                    Some(v7) => {
                        let m115 = match v2.clone() {
                            Adt6::C0(_, _, h114, _, _) => {
                                h114
                            }
                        };
                        match m115 {
                            None => {
                                Ok(Some(f22(v1, v2, v7)?))
                            }
                            Some(_) => {
                                Ok(Some(f21(v2, f10(f11(Str::lit("trailing tokens after the expression"))))))
                            }
                        }
                    }
                }
            }
        }
        Some(_) => {
            Ok(None::<Adt6>)
        }
    }
}

pub fn f19(v0: u64, v1: Adt7) -> R<Option<Adt7>> {
    let m117 = match v1.clone() {
        Adt7::C0(_, _, h116) => {
            h116
        }
    };
    match m117 {
        None => {
            let m119 = match v1.clone() {
                Adt7::C0(h118, _, _) => {
                    h118
                }
            };
            match m119.uncons() {
                None => {
                    Ok(None::<Adt7>)
                }
                Some((v3, v4)) => {
                    let m120 = v3;
                    match m120 {
                        Adt8::C0(v5) => {
                            Ok(Some(f23(v1, v4, v5, v0)?))
                        }
                        Adt8::C1(v6) => {
                            Ok(Some(f24(v1, v4, v6, v0)?))
                        }
                    }
                }
            }
        }
        Some(_) => {
            Ok(None::<Adt7>)
        }
    }
}

pub fn f20(v0: Bytes, v1: u64) -> R<u64> {
    let m121 = f25(v0, v1);
    match m121 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a166 = {
                let a142 = {
                    let a130 = {
                        let a124 = {
                            let a122 = v3;
                            let a123 = 128u8;
                            fixed_u8::bit_and(a122, a123)
                        };
                        let a125 = 128u8;
                        if equal(a124, a125) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a131 = {
                        let a128 = {
                            let a126 = v3;
                            let a127 = 64u8;
                            fixed_u8::bit_and(a126, a127)
                        };
                        let a129 = 64u8;
                        if equal(a128, a129) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a130, a131)?
                };
                let a143 = {
                    let a140 = {
                        let a134 = {
                            let a132 = v3;
                            let a133 = 32u8;
                            fixed_u8::bit_and(a132, a133)
                        };
                        let a135 = 32u8;
                        if equal(a134, a135) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a141 = {
                        let a138 = {
                            let a136 = v3;
                            let a137 = 16u8;
                            fixed_u8::bit_and(a136, a137)
                        };
                        let a139 = 16u8;
                        if equal(a138, a139) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a140, a141)?
                };
                nat_add(a142, a143)?
            };
            let a167 = {
                let a164 = {
                    let a152 = {
                        let a146 = {
                            let a144 = v3;
                            let a145 = 8u8;
                            fixed_u8::bit_and(a144, a145)
                        };
                        let a147 = 8u8;
                        if equal(a146, a147) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a153 = {
                        let a150 = {
                            let a148 = v3;
                            let a149 = 4u8;
                            fixed_u8::bit_and(a148, a149)
                        };
                        let a151 = 4u8;
                        if equal(a150, a151) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a152, a153)?
                };
                let a165 = {
                    let a162 = {
                        let a156 = {
                            let a154 = v3;
                            let a155 = 2u8;
                            fixed_u8::bit_and(a154, a155)
                        };
                        let a157 = 2u8;
                        if equal(a156, a157) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a163 = {
                        let a160 = {
                            let a158 = v3;
                            let a159 = 1u8;
                            fixed_u8::bit_and(a158, a159)
                        };
                        let a161 = 1u8;
                        if equal(a160, a161) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a162, a163)?
                };
                nat_add(a164, a165)?
            };
            nat_add(a166, a167)
        }
    }
}

pub fn f21(v0: Adt6, v1: Adt2) -> Adt6 {
    Adt6::C0(match v0.clone() {
        Adt6::C0(h168, _, _, _, _) => {
            h168
        }
    }, match v0.clone() {
        Adt6::C0(_, h169, _, _, _) => {
            h169
        }
    }, match v0 {
        Adt6::C0(_, _, h170, _, _) => {
            h170
        }
    }, Some(v1), false)
}

pub fn f22(v0: u64, v1: Adt6, v2: Adt3) -> R<Adt6> {
    let m171 = v2;
    match m171 {
        Adt3::C0 => {
            let a174 = {
                let a173 = match v1.clone() {
                    Adt6::C0(_, h172, _, _, _) => {
                        h172
                    }
                };
                length_list(a173)
            };
            let a175 = v0;
            if nat_lt(a174, a175) {
                Ok(Adt6::C0({
                    let a177 = match v1.clone() {
                        Adt6::C0(h176, _, _, _, _) => {
                            h176
                        }
                    };
                    let a178 = 1u64;
                    nat_add(a177, a178)?
                }, List::cons(List::<Adt5>::nil(), match v1.clone() {
                    Adt6::C0(_, h179, _, _, _) => {
                        h179
                    }
                }), match v1 {
                    Adt6::C0(_, _, h180, _, _) => {
                        h180
                    }
                }, None::<Adt2>, false))
            } else {
                Ok(f21(v1, Adt2::C0(1u64, f26(f11(Str::lit("max_scope_depth exceeded: configured ")), f27(v0)))))
            }
        }
        Adt3::C1 => {
            let m182 = match v1.clone() {
                Adt6::C0(_, h181, _, _, _) => {
                    h181
                }
            };
            match m182.uncons() {
                None => {
                    Ok(f21(v1, f10(f11(Str::lit("unbalanced `)`")))))
                }
                Some((v3, v4)) => {
                    f28(v1, Adt5::C1(std::rc::Rc::new(f29(v3))), v4)
                }
            }
        }
        Adt3::C2(v5) => {
            f28(v1.clone(), Adt5::C0(v5), match v1 {
                Adt6::C0(_, h183, _, _, _) => {
                    h183
                }
            })
        }
    }
}

pub fn f23(v0: Adt7, v1: List<Adt8>, v2: Adt5, v3: u64) -> R<Adt7> {
    let m184 = v2.clone();
    match m184 {
        Adt5::C0(_) => {
            Ok(f31(v0, f10(f26(f11(Str::lit("expected `(`, found ")), f33(v2)))))
        }
        Adt5::C1(r187) => {
            let v4 = (*r187).clone();
            let m185 = f30(v4, v3)?;
            match m185.uncons() {
                None => {
                    Ok(f31(v0, f10(f11(Str::lit("expected an LSE form")))))
                }
                Some((v5, v6)) => {
                    let m186 = v5;
                    match m186 {
                        Adt5::C0(v7) => {
                            f32(v0, v1, v2, v7, v6, v3)
                        }
                        Adt5::C1(_) => {
                            Ok(f31(v0, f10(f11(Str::lit("expected an LSE form")))))
                        }
                        Adt5::C2 => {
                            Ok(f31(v0, f10(f11(Str::lit("expected an LSE form")))))
                        }
                        Adt5::C3(_, _) => {
                            Ok(f31(v0, f10(f11(Str::lit("expected an LSE form")))))
                        }
                    }
                }
            }
        }
        Adt5::C2 => {
            Ok(f31(v0, f10(f26(f11(Str::lit("expected `(`, found ")), f33(v2)))))
        }
        Adt5::C3(_, _) => {
            Ok(f31(v0, f10(f26(f11(Str::lit("expected `(`, found ")), f33(v2)))))
        }
    }
}

pub fn f24(v0: Adt7, v1: List<Adt8>, v2: Adt5, v3: u64) -> R<Adt7> {
    let m188 = v2;
    match m188 {
        Adt5::C0(_) => {
            Ok(f31(v0, f10(f11(Str::lit("expected a list")))))
        }
        Adt5::C1(r216) => {
            let v4 = (*r216).clone();
            let v5: List<Adt5> = f30(v4, v3)?;
            let m189 = v5;
            match m189.uncons() {
                None => {
                    Ok(f31(v0, f10(f11(Str::lit("expected a list")))))
                }
                Some((v6, v7)) => {
                    let m190 = v6;
                    match m190 {
                        Adt5::C0(v8) => {
                            let a191 = v8.clone();
                            let a192 = f11(Str::lit("app"));
                            if equal(a191, a192) {
                                let v9: (List<Adt0>, List<Adt0>) = f34(match v0.clone() {
                                    Adt7::C0(_, h193, _) => {
                                        h193
                                    }
                                }, {
                                    let a194 = v7;
                                    length_list(a194)
                                });
                                let m196 = {
                                    let (h195, _) = v9;
                                    h195
                                };
                                match m196.uncons() {
                                    None => {
                                        Ok(f31(v0, f10(f11(Str::lit("missing function")))))
                                    }
                                    Some((v10, v11)) => {
                                        Ok(Adt7::C0(v1, List::cons(Adt0::C4(std::rc::Rc::new(v10), std::rc::Rc::new(f35(v11))), {
                                            let (_, h197) = v9;
                                            h197
                                        }), None::<Adt2>))
                                    }
                                }
                            } else {
                                if f36(v8.clone()) {
                                    let m198 = f37(v7, 0u64);
                                    match m198 {
                                        Adt5::C0(_) => {
                                            Ok(f31(v0, f10(f11(Str::lit("expected a binder list")))))
                                        }
                                        Adt5::C1(r208) => {
                                            let v12 = (*r208).clone();
                                            let v13: List<Adt5> = f30(v12, v3)?;
                                            let v14: (List<Adt0>, List<Adt0>) = f34(match v0.clone() {
                                                Adt7::C0(_, h199, _) => {
                                                    h199
                                                }
                                            }, {
                                                let a201 = {
                                                    let a200 = v13.clone();
                                                    length_list(a200)
                                                };
                                                let a202 = 1u64;
                                                nat_add(a201, a202)?
                                            });
                                            let m204 = f38({
                                                let (h203, _) = v14;
                                                h203
                                            });
                                            match m204.uncons() {
                                                None => {
                                                    Ok(f31(v0, f10(f11(Str::lit("missing body")))))
                                                }
                                                Some((v15, v16)) => {
                                                    let v17: Adt0 = f39(v13, f38(v16), v3)?;
                                                    Ok(Adt7::C0(v1, List::cons({
                                                        let a205 = v8;
                                                        let a206 = f11(Str::lit("pi"));
                                                        if equal(a205, a206) {
                                                            Adt0::C5(std::rc::Rc::new(v17), std::rc::Rc::new(v15))
                                                        } else {
                                                            Adt0::C6(std::rc::Rc::new(v17), std::rc::Rc::new(v15))
                                                        }
                                                    }, {
                                                        let (_, h207) = v14;
                                                        h207
                                                    }), None::<Adt2>))
                                                }
                                            }
                                        }
                                        Adt5::C2 => {
                                            Ok(f31(v0, f10(f11(Str::lit("expected a binder list")))))
                                        }
                                        Adt5::C3(_, _) => {
                                            Ok(f31(v0, f10(f11(Str::lit("expected a binder list")))))
                                        }
                                    }
                                } else {
                                    let v21: (List<Adt0>, List<Adt0>) = f34(match v0.clone() {
                                        Adt7::C0(_, h209, _) => {
                                            h209
                                        }
                                    }, 3u64);
                                    let m211 = {
                                        let (h210, _) = v21;
                                        h210
                                    };
                                    match m211.uncons() {
                                        None => {
                                            Ok(f31(v0, f10(f11(Str::lit("missing type")))))
                                        }
                                        Some((v22, v23)) => {
                                            let m212 = v23;
                                            match m212.uncons() {
                                                None => {
                                                    Ok(f31(v0, f10(f11(Str::lit("missing value")))))
                                                }
                                                Some((v24, v25)) => {
                                                    let m213 = v25;
                                                    match m213.uncons() {
                                                        None => {
                                                            Ok(f31(v0, f10(f11(Str::lit("missing body")))))
                                                        }
                                                        Some((v26, _)) => {
                                                            let m214 = f40(f37(v7, 0u64), f11(Str::lit("a local ID")), v3);
                                                            match m214 {
                                                                Ok(v28) => {
                                                                    Ok(Adt7::C0(v1, List::cons(Adt0::C7(v28, std::rc::Rc::new(v22), std::rc::Rc::new(v24), std::rc::Rc::new(v26)), {
                                                                        let (_, h215) = v21;
                                                                        h215
                                                                    }), None::<Adt2>))
                                                                }
                                                                Err(v29) => {
                                                                    Ok(f31(v0, v29))
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
                        Adt5::C1(_) => {
                            Ok(f31(v0, f10(f11(Str::lit("expected a form, found a list")))))
                        }
                        Adt5::C2 => {
                            Ok(f31(v0, f10(f11(Str::lit("expected a form, found a list")))))
                        }
                        Adt5::C3(_, _) => {
                            Ok(f31(v0, f10(f11(Str::lit("expected a form, found a list")))))
                        }
                    }
                }
            }
        }
        Adt5::C2 => {
            Ok(f31(v0, f10(f11(Str::lit("expected a list")))))
        }
        Adt5::C3(_, _) => {
            Ok(f31(v0, f10(f11(Str::lit("expected a list")))))
        }
    }
}

pub fn f25(v0: Bytes, v1: u64) -> Option<u8> {
    let a217 = v0;
    let a218 = v1;
    index_bytes(a217, a218)
}

pub fn f26(v0: Bytes, v1: Bytes) -> Bytes {
    let a219 = v0;
    let a220 = v1;
    append_bytes(a219, a220)
}

pub fn f27(v0: u64) -> Bytes {
    f41(f42(), 10u64, v0)
}

pub fn f28(v0: Adt6, v1: Adt5, v2: List<List<Adt5>>) -> R<Adt6> {
    let m221 = v2;
    match m221.uncons() {
        None => {
            Ok(Adt6::C0({
                let a223 = match v0 {
                    Adt6::C0(h222, _, _, _, _) => {
                        h222
                    }
                };
                let a224 = 1u64;
                nat_add(a223, a224)?
            }, List::<List<Adt5>>::nil(), Some(v1), None::<Adt2>, false))
        }
        Some((v3, v4)) => {
            Ok(Adt6::C0({
                let a226 = match v0.clone() {
                    Adt6::C0(h225, _, _, _, _) => {
                        h225
                    }
                };
                let a227 = 1u64;
                nat_add(a226, a227)?
            }, List::cons(List::cons(v1, v3), v4), match v0 {
                Adt6::C0(_, _, h228, _, _) => {
                    h228
                }
            }, None::<Adt2>, false))
        }
    }
}

pub fn f29(v0: List<Adt5>) -> Adt5 {
    f43(Fn4::F44, Adt5::C2, v0)
}

pub fn f30(v0: Adt5, v1: u64) -> R<List<Adt5>> {
    Ok(f45({
        let (_, h229) = {
            let (h230, _) = f46(Fn5::F47, {
                let a231 = v1;
                let a232 = 1u64;
                nat_add(a231, a232)?
            }, (v0, List::<Adt5>::nil()));
            h230
        };
        h229
    }))
}

pub fn f31(v0: Adt7, v1: Adt2) -> Adt7 {
    Adt7::C0(match v0.clone() {
        Adt7::C0(h233, _, _) => {
            h233
        }
    }, match v0 {
        Adt7::C0(_, h234, _) => {
            h234
        }
    }, Some(v1))
}

pub fn f32(v0: Adt7, v1: List<Adt8>, v2: Adt5, v3: Bytes, v4: List<Adt5>, v5: u64) -> R<Adt7> {
    if f48(v3.clone()) {
        let m235 = f49(v3, v4, v5)?;
        match m235 {
            Ok(v6) => {
                Ok(Adt7::C0(v1, List::cons(v6, match v0 {
                    Adt7::C0(_, h236, _) => {
                        h236
                    }
                }), None::<Adt2>))
            }
            Err(v7) => {
                Ok(f31(v0, v7))
            }
        }
    } else {
        let a237 = v3.clone();
        let a238 = f11(Str::lit("app"));
        if equal(a237, a238) {
            let a240 = {
                let a239 = v4.clone();
                length_list(a239)
            };
            let a241 = 2u64;
            if nat_lt(a240, a241) {
                Ok(f31(v0, f10(f11(Str::lit("`app` has at least a function and one argument")))))
            } else {
                Ok(Adt7::C0(f50(v4, List::cons(Adt8::C1(v2), v1)), match v0 {
                    Adt7::C0(_, h242, _) => {
                        h242
                    }
                }, None::<Adt2>))
            }
        } else {
            if f36(v3.clone()) {
                f51(v0, v1, v2, v3, v4, v5)
            } else {
                let a243 = v3.clone();
                let a244 = f11(Str::lit("let"));
                if equal(a243, a244) {
                    f52(v0, v1, v2, v4, v5)
                } else {
                    Ok(f31(v0, f10(f26(f53(v3), f11(Str::lit(" is not an LSE form"))))))
                }
            }
        }
    }
}

pub fn f33(v0: Adt5) -> Bytes {
    let m245 = v0;
    match m245 {
        Adt5::C0(v1) => {
            f53(v1)
        }
        Adt5::C1(r249) => {
            let v2 = (*r249).clone();
            let m246 = v2;
            match m246 {
                Adt5::C0(_) => {
                    f11(Str::lit("a list"))
                }
                Adt5::C1(_) => {
                    f11(Str::lit("a list"))
                }
                Adt5::C2 => {
                    f11(Str::lit("a list"))
                }
                Adt5::C3(r248, _) => {
                    let v3 = (*r248).clone();
                    let m247 = v3;
                    match m247 {
                        Adt5::C0(v5) => {
                            f26(f26(f11(Str::lit("`(")), v5), f11(Str::lit(" ...)`")))
                        }
                        Adt5::C1(_) => {
                            f11(Str::lit("a list"))
                        }
                        Adt5::C2 => {
                            f11(Str::lit("a list"))
                        }
                        Adt5::C3(_, _) => {
                            f11(Str::lit("a list"))
                        }
                    }
                }
            }
        }
        Adt5::C2 => {
            f11(Str::lit("a list"))
        }
        Adt5::C3(_, _) => {
            f11(Str::lit("a list"))
        }
    }
}

pub fn f34(v0: List<Adt0>, v1: u64) -> (List<Adt0>, List<Adt0>) {
    let (h250, _) = f54(Fn6::F55, v1, (List::<Adt0>::nil(), v0));
    h250
}

pub fn f35(v0: List<Adt0>) -> Adt0 {
    f56(Fn7::F57, Adt0::C9, f38(v0))
}

pub fn f36(v0: Bytes) -> bool {
    let a251 = v0.clone();
    let a252 = f11(Str::lit("pi"));
    if equal(a251, a252) {
        true
    } else {
        let a253 = v0;
        let a254 = f11(Str::lit("lam"));
        equal(a253, a254)
    }
}

pub fn f37(v0: List<Adt5>, v1: u64) -> Adt5 {
    let m257 = {
        let a255 = v0;
        let a256 = v1;
        index_list(a255, a256)
    };
    match m257 {
        None => {
            Adt5::C2
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f38(v0: List<Adt0>) -> List<Adt0> {
    f58(Fn8::F59, List::<Adt0>::nil(), v0)
}

pub fn f39(v0: List<Adt5>, v1: List<Adt0>, v2: u64) -> R<Adt0> {
    let (_, h258) = f60(Fn9::F61(v2), (f38(v1), Adt0::C9), f45(v0))?;
    Ok(h258)
}

pub fn f40(v0: Adt5, v1: Bytes, _: u64) -> Result<Bytes, Adt2> {
    let m259 = v0.clone();
    match m259 {
        Adt5::C0(v3) => {
            Ok::<Bytes, Adt2>(v3)
        }
        Adt5::C1(_) => {
            Err::<Bytes, Adt2>(f10(f26(f26(f26(f11(Str::lit("expected ")), v1), f11(Str::lit(", found "))), f33(v0))))
        }
        Adt5::C2 => {
            Err::<Bytes, Adt2>(f10(f26(f26(f26(f11(Str::lit("expected ")), v1), f11(Str::lit(", found "))), f33(v0))))
        }
        Adt5::C3(_, _) => {
            Err::<Bytes, Adt2>(f10(f26(f26(f26(f11(Str::lit("expected ")), v1), f11(Str::lit(", found "))), f33(v0))))
        }
    }
}

pub fn f41(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let a260 = v2;
    let a261 = 0u64;
    if nat_eq(a260, a261) {
        f62(v0, 0u64)
    } else {
        let (_, h262) = {
            let (h263, _) = f63(Fn10::F64(v1, v0), v2, (v2, Bytes::lit(&[])));
            h263
        };
        h262
    }
}

pub fn f42() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f43(v0: Fn4, p267: Adt5, p268: List<Adt5>) -> Adt5 {
    let m266 = (p267, p268);
    {
        let mut m266 = m266;
        loop {
            let step = {
                let (v1, v2) = m266;
                let m264 = v2;
                match m264.uncons() {
                    None => {
                        Err::<(Adt5, List<Adt5>), Adt5>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt5, List<Adt5>), Adt5>(({
                            let c265: Fn4 = v0.clone();
                            c265.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m266 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f44(v0: Adt5, v1: Adt5) -> Adt5 {
    Adt5::C3(std::rc::Rc::new(v1), std::rc::Rc::new(v0))
}

pub fn f45(v0: List<Adt5>) -> List<Adt5> {
    f65(Fn11::F66, List::<Adt5>::nil(), v0)
}

pub fn f46(v0: Fn5, p273: u64, p274: (Adt5, List<Adt5>)) -> ((Adt5, List<Adt5>), bool) {
    let m272 = (p273, p274);
    {
        let mut m272 = m272;
        loop {
            let step = {
                let (v1, v2) = m272;
                let m269 = v1;
                if m269 == 0 {
                    Err::<(u64, (Adt5, List<Adt5>)), ((Adt5, List<Adt5>), bool)>((v2, false))
                } else {
                    let v3 = m269 - 1;
                    let m271 = {
                        let c270: Fn5 = v0.clone();
                        c270.apply(v2.clone())
                    };
                    match m271 {
                        None => {
                            Err::<(u64, (Adt5, List<Adt5>)), ((Adt5, List<Adt5>), bool)>((v2, true))
                        }
                        Some(v4) => {
                            Ok::<(u64, (Adt5, List<Adt5>)), ((Adt5, List<Adt5>), bool)>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m272 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f47(v0: (Adt5, List<Adt5>)) -> Option<(Adt5, List<Adt5>)> {
    f67(v0)
}

pub fn f48(v0: Bytes) -> bool {
    let a275 = v0.clone();
    let a276 = f11(Str::lit("sort"));
    if equal(a275, a276) {
        true
    } else {
        let a277 = v0.clone();
        let a278 = f11(Str::lit("const"));
        if equal(a277, a278) {
            true
        } else {
            let a279 = v0.clone();
            let a280 = f11(Str::lit("local"));
            if equal(a279, a280) {
                true
            } else {
                let a281 = v0;
                let a282 = f11(Str::lit("nat"));
                equal(a281, a282)
            }
        }
    }
}

pub fn f49(v0: Bytes, v1: List<Adt5>, v2: u64) -> R<Result<Adt0, Adt2>> {
    let a283 = v0.clone();
    let a284 = f11(Str::lit("sort"));
    if equal(a283, a284) {
        f68(v1, v2)
    } else {
        let a285 = v0.clone();
        let a286 = f11(Str::lit("const"));
        if equal(a285, a286) {
            f69(v1, v2)
        } else {
            let a287 = v0.clone();
            let a288 = f11(Str::lit("local"));
            if equal(a287, a288) {
                f70(v1)
            } else {
                let a289 = v0.clone();
                let a290 = f11(Str::lit("nat"));
                if equal(a289, a290) {
                    f71(v1)
                } else {
                    Ok(Err::<Adt0, Adt2>(f10(f26(f53(v0), f11(Str::lit(" is not an LSE form"))))))
                }
            }
        }
    }
}

pub fn f50(v0: List<Adt5>, v1: List<Adt8>) -> List<Adt8> {
    f72(Fn12::F73, v1, f45(v0))
}

pub fn f51(v0: Adt7, v1: List<Adt8>, v2: Adt5, v3: Bytes, v4: List<Adt5>, v5: u64) -> R<Adt7> {
    let a292 = {
        let a291 = v4.clone();
        length_list(a291)
    };
    let a293 = 2u64;
    if nat_eq(a292, a293) {
        let m294 = f37(v4.clone(), 0u64);
        match m294 {
            Adt5::C0(_) => {
                Ok(f31(v0, f10(f11(Str::lit("expected a binder list")))))
            }
            Adt5::C1(r298) => {
                let v6 = (*r298).clone();
                let v7: List<Adt5> = f30(v6, v5)?;
                let m295 = v7.clone();
                match m295.uncons() {
                    None => {
                        Ok(f31(v0, f10(f26(f53(v3), f11(Str::lit(" has at least one binder"))))))
                    }
                    Some((_, _)) => {
                        let m296 = f74(v7, List::cons(Adt8::C0(f37(v4, 1u64)), List::cons(Adt8::C1(v2), v1)), v5)?;
                        match m296 {
                            Ok(v10) => {
                                Ok(Adt7::C0(v10, match v0 {
                                    Adt7::C0(_, h297, _) => {
                                        h297
                                    }
                                }, None::<Adt2>))
                            }
                            Err(v11) => {
                                Ok(f31(v0, v11))
                            }
                        }
                    }
                }
            }
            Adt5::C2 => {
                Ok(f31(v0, f10(f11(Str::lit("expected a binder list")))))
            }
            Adt5::C3(_, _) => {
                Ok(f31(v0, f10(f11(Str::lit("expected a binder list")))))
            }
        }
    } else {
        Ok(f31(v0, f10(f26(f26(f26(f26(f11(Str::lit("`")), v3.clone()), f11(Str::lit("` is `("))), v3), f11(Str::lit(" (binders...) body)`"))))))
    }
}

pub fn f52(v0: Adt7, v1: List<Adt8>, v2: Adt5, v3: List<Adt5>, v4: u64) -> R<Adt7> {
    let a300 = {
        let a299 = v3.clone();
        length_list(a299)
    };
    let a301 = 4u64;
    if nat_eq(a300, a301) {
        let m302 = f40(f37(v3.clone(), 0u64), f11(Str::lit("a local ID")), v4);
        match m302 {
            Ok(v5) => {
                if f75(v5.clone())? {
                    Ok(Adt7::C0(List::cons(Adt8::C0(f37(v3.clone(), 1u64)), List::cons(Adt8::C0(f37(v3.clone(), 2u64)), List::cons(Adt8::C0(f37(v3, 3u64)), List::cons(Adt8::C1(v2), v1)))), match v0 {
                        Adt7::C0(_, h303, _) => {
                            h303
                        }
                    }, None::<Adt2>))
                } else {
                    Ok(f31(v0, f10(f26(f53(v5), f11(Str::lit(" is not a valid local ID"))))))
                }
            }
            Err(v6) => {
                Ok(f31(v0, v6))
            }
        }
    } else {
        Ok(f31(v0, f10(f11(Str::lit("`let` is `(let name type value body)`")))))
    }
}

pub fn f53(v0: Bytes) -> Bytes {
    f26(f26(f11(Str::lit("`")), v0), f11(Str::lit("`")))
}

pub fn f54(v0: Fn6, p308: u64, p309: (List<Adt0>, List<Adt0>)) -> ((List<Adt0>, List<Adt0>), bool) {
    let m307 = (p308, p309);
    {
        let mut m307 = m307;
        loop {
            let step = {
                let (v1, v2) = m307;
                let m304 = v1;
                if m304 == 0 {
                    Err::<(u64, (List<Adt0>, List<Adt0>)), ((List<Adt0>, List<Adt0>), bool)>((v2, false))
                } else {
                    let v3 = m304 - 1;
                    let m306 = {
                        let c305: Fn6 = v0.clone();
                        c305.apply(v2.clone())
                    };
                    match m306 {
                        None => {
                            Err::<(u64, (List<Adt0>, List<Adt0>)), ((List<Adt0>, List<Adt0>), bool)>((v2, true))
                        }
                        Some(v4) => {
                            Ok::<(u64, (List<Adt0>, List<Adt0>)), ((List<Adt0>, List<Adt0>), bool)>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m307 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f55(v0: (List<Adt0>, List<Adt0>)) -> Option<(List<Adt0>, List<Adt0>)> {
    let m311 = {
        let (_, h310) = v0;
        h310
    };
    match m311.uncons() {
        None => {
            None::<(List<Adt0>, List<Adt0>)>
        }
        Some((v1, v2)) => {
            Some((List::cons(v1, {
                let (h312, _) = v0;
                h312
            }), v2))
        }
    }
}

pub fn f56(v0: Fn7, p316: Adt0, p317: List<Adt0>) -> Adt0 {
    let m315 = (p316, p317);
    {
        let mut m315 = m315;
        loop {
            let step = {
                let (v1, v2) = m315;
                let m313 = v2;
                match m313.uncons() {
                    None => {
                        Err::<(Adt0, List<Adt0>), Adt0>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt0, List<Adt0>), Adt0>(({
                            let c314: Fn7 = v0.clone();
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

pub fn f57(v0: Adt0, v1: Adt0) -> Adt0 {
    Adt0::C10(std::rc::Rc::new(v1), std::rc::Rc::new(v0))
}

pub fn f58(v0: Fn8, p321: List<Adt0>, p322: List<Adt0>) -> List<Adt0> {
    let m320 = (p321, p322);
    {
        let mut m320 = m320;
        loop {
            let step = {
                let (v1, v2) = m320;
                let m318 = v2;
                match m318.uncons() {
                    None => {
                        Err::<(List<Adt0>, List<Adt0>), List<Adt0>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt0>, List<Adt0>), List<Adt0>>(({
                            let c319: Fn8 = v0.clone();
                            c319.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m320 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f59(v0: List<Adt0>, v1: Adt0) -> List<Adt0> {
    List::cons(v1, v0)
}

pub fn f60(v0: Fn9, p326: (List<Adt0>, Adt0), p327: List<Adt5>) -> R<(List<Adt0>, Adt0)> {
    let m325 = (p326, p327);
    {
        let mut m325 = m325;
        loop {
            let step = {
                let (v1, v2) = m325;
                let m323 = v2;
                match m323.uncons() {
                    None => {
                        Err::<((List<Adt0>, Adt0), List<Adt5>), R<(List<Adt0>, Adt0)>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<((List<Adt0>, Adt0), List<Adt5>), R<(List<Adt0>, Adt0)>>(({
                            let c324: Fn9 = v0.clone();
                            c324.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m325 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f61(v0: u64, v1: (List<Adt0>, Adt0), v2: Adt5) -> R<(List<Adt0>, Adt0)> {
    let m329 = {
        let (h328, _) = v1.clone();
        h328
    };
    match m329.uncons() {
        None => {
            Ok(v1)
        }
        Some((v3, v4)) => {
            let m330 = f76(v2, v0)?;
            match m330 {
                Ok(v5) => {
                    Ok((v4, Adt0::C11(match v5.clone() {
                        Adt9::C0(h331, _, _) => {
                            h331
                        }
                    }, match v5 {
                        Adt9::C0(_, h332, _) => {
                            h332
                        }
                    }, std::rc::Rc::new(v3), std::rc::Rc::new({
                        let (_, h333) = v1;
                        h333
                    }))))
                }
                Err(_) => {
                    Ok((v4, {
                        let (_, h334) = v1;
                        h334
                    }))
                }
            }
        }
    }
}

pub fn f62(v0: Bytes, v1: u64) -> Bytes {
    let m338 = {
        let a335 = v0;
        let a336 = v1;
        let a337 = 1u64;
        slice_bytes(a335, a336, a337)
    };
    match m338 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f63(v0: Fn10, p343: u64, p344: (u64, Bytes)) -> ((u64, Bytes), bool) {
    let m342 = (p343, p344);
    {
        let mut m342 = m342;
        loop {
            let step = {
                let (v1, v2) = m342;
                let m339 = v1;
                if m339 == 0 {
                    Err::<(u64, (u64, Bytes)), ((u64, Bytes), bool)>((v2, false))
                } else {
                    let v3 = m339 - 1;
                    let m341 = {
                        let c340: Fn10 = v0.clone();
                        c340.apply(v2.clone())
                    };
                    match m341 {
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
                Ok(next) => { m342 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f64(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f77(v1, v0, v2)
}

pub fn f65(v0: Fn11, p348: List<Adt5>, p349: List<Adt5>) -> List<Adt5> {
    let m347 = (p348, p349);
    {
        let mut m347 = m347;
        loop {
            let step = {
                let (v1, v2) = m347;
                let m345 = v2;
                match m345.uncons() {
                    None => {
                        Err::<(List<Adt5>, List<Adt5>), List<Adt5>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt5>, List<Adt5>), List<Adt5>>(({
                            let c346: Fn11 = v0.clone();
                            c346.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m347 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f66(v0: List<Adt5>, v1: Adt5) -> List<Adt5> {
    List::cons(v1, v0)
}

pub fn f67(v0: (Adt5, List<Adt5>)) -> Option<(Adt5, List<Adt5>)> {
    let m351 = {
        let (h350, _) = v0;
        h350
    };
    match m351 {
        Adt5::C0(_) => {
            None::<(Adt5, List<Adt5>)>
        }
        Adt5::C1(_) => {
            None::<(Adt5, List<Adt5>)>
        }
        Adt5::C2 => {
            None::<(Adt5, List<Adt5>)>
        }
        Adt5::C3(r353, r354) => {
            let v3 = (*r353).clone();
            let v4 = (*r354).clone();
            Some((v4, List::cons(v3, {
                let (_, h352) = v0;
                h352
            })))
        }
    }
}

pub fn f68(v0: List<Adt5>, v1: u64) -> R<Result<Adt0, Adt2>> {
    let a356 = {
        let a355 = v0.clone();
        length_list(a355)
    };
    let a357 = 1u64;
    if nat_eq(a356, a357) {
        let m358 = f37(v0, 0u64);
        match m358 {
            Adt5::C0(v2) => {
                let a359 = v2;
                let a360 = f11(Str::lit("prop"));
                if equal(a359, a360) {
                    Ok(Ok::<Adt0, Adt2>(Adt0::C0))
                } else {
                    Ok(Err::<Adt0, Adt2>(f78()))
                }
            }
            Adt5::C1(r368) => {
                let v3 = (*r368).clone();
                let v4: List<Adt5> = f30(v3, v1)?;
                let a362 = {
                    let a361 = v4.clone();
                    length_list(a361)
                };
                let a363 = 2u64;
                if nat_eq(a362, a363) {
                    let m364 = f37(v4.clone(), 0u64);
                    match m364 {
                        Adt5::C0(v5) => {
                            let a365 = v5;
                            let a366 = f11(Str::lit("type"));
                            if equal(a365, a366) {
                                let m367 = f79(f37(v4, 1u64), v1)?;
                                match m367 {
                                    Ok(v6) => {
                                        Ok(Ok::<Adt0, Adt2>(Adt0::C1(v6)))
                                    }
                                    Err(v7) => {
                                        Ok(Err::<Adt0, Adt2>(v7))
                                    }
                                }
                            } else {
                                Ok(Err::<Adt0, Adt2>(f78()))
                            }
                        }
                        Adt5::C1(_) => {
                            Ok(Err::<Adt0, Adt2>(f78()))
                        }
                        Adt5::C2 => {
                            Ok(Err::<Adt0, Adt2>(f78()))
                        }
                        Adt5::C3(_, _) => {
                            Ok(Err::<Adt0, Adt2>(f78()))
                        }
                    }
                } else {
                    Ok(Err::<Adt0, Adt2>(f78()))
                }
            }
            Adt5::C2 => {
                Ok(Err::<Adt0, Adt2>(f78()))
            }
            Adt5::C3(_, _) => {
                Ok(Err::<Adt0, Adt2>(f78()))
            }
        }
    } else {
        Ok(Err::<Adt0, Adt2>(f78()))
    }
}

pub fn f69(v0: List<Adt5>, v1: u64) -> R<Result<Adt0, Adt2>> {
    let v2: u64 = {
        let a369 = v0.clone();
        length_list(a369)
    };
    let m370 = f37(v0.clone(), 0u64);
    match m370 {
        Adt5::C0(v3) => {
            let a371 = v2;
            let a372 = 1u64;
            if nat_eq(a371, a372) {
                let m373 = f80(v3)?;
                match m373 {
                    Ok(v4) => {
                        Ok(Ok::<Adt0, Adt2>(Adt0::C2({
                            let (h374, _) = v4;
                            h374
                        }, {
                            let (_, h375) = v4;
                            h375
                        }, Adt1::C5)))
                    }
                    Err(v5) => {
                        Ok(Err::<Adt0, Adt2>(v5))
                    }
                }
            } else {
                let a376 = v2;
                let a377 = 2u64;
                if nat_eq(a376, a377) {
                    let m378 = f37(v0, 1u64);
                    match m378 {
                        Adt5::C0(_) => {
                            Ok(Err::<Adt0, Adt2>(f81()))
                        }
                        Adt5::C1(r384) => {
                            let v6 = (*r384).clone();
                            let v7: List<Adt5> = f30(v6, v1)?;
                            let m379 = v7.clone();
                            match m379.uncons() {
                                None => {
                                    Ok(Err::<Adt0, Adt2>(f81()))
                                }
                                Some((_, _)) => {
                                    let m380 = f80(v3)?;
                                    match m380 {
                                        Ok(v10) => {
                                            let m381 = f82(v7, v1)?;
                                            match m381 {
                                                Ok(v11) => {
                                                    Ok(Ok::<Adt0, Adt2>(Adt0::C2({
                                                        let (h382, _) = v10;
                                                        h382
                                                    }, {
                                                        let (_, h383) = v10;
                                                        h383
                                                    }, v11)))
                                                }
                                                Err(v12) => {
                                                    Ok(Err::<Adt0, Adt2>(v12))
                                                }
                                            }
                                        }
                                        Err(v13) => {
                                            Ok(Err::<Adt0, Adt2>(v13))
                                        }
                                    }
                                }
                            }
                        }
                        Adt5::C2 => {
                            Ok(Err::<Adt0, Adt2>(f81()))
                        }
                        Adt5::C3(_, _) => {
                            Ok(Err::<Adt0, Adt2>(f81()))
                        }
                    }
                } else {
                    Ok(Err::<Adt0, Adt2>(f81()))
                }
            }
        }
        Adt5::C1(_) => {
            Ok(Err::<Adt0, Adt2>(f81()))
        }
        Adt5::C2 => {
            Ok(Err::<Adt0, Adt2>(f81()))
        }
        Adt5::C3(_, _) => {
            Ok(Err::<Adt0, Adt2>(f81()))
        }
    }
}

pub fn f70(v0: List<Adt5>) -> R<Result<Adt0, Adt2>> {
    let a386 = {
        let a385 = v0.clone();
        length_list(a385)
    };
    let a387 = 1u64;
    if nat_eq(a386, a387) {
        let m388 = f37(v0, 0u64);
        match m388 {
            Adt5::C0(v1) => {
                if f75(v1.clone())? {
                    Ok(Ok::<Adt0, Adt2>(Adt0::C3(v1)))
                } else {
                    Ok(Err::<Adt0, Adt2>(f10(f26(f53(v1), f11(Str::lit(" is not a valid local ID"))))))
                }
            }
            Adt5::C1(_) => {
                Ok(Err::<Adt0, Adt2>(f10(f11(Str::lit("a local is `(local id)`")))))
            }
            Adt5::C2 => {
                Ok(Err::<Adt0, Adt2>(f10(f11(Str::lit("a local is `(local id)`")))))
            }
            Adt5::C3(_, _) => {
                Ok(Err::<Adt0, Adt2>(f10(f11(Str::lit("a local is `(local id)`")))))
            }
        }
    } else {
        Ok(Err::<Adt0, Adt2>(f10(f11(Str::lit("a local is `(local id)`")))))
    }
}

pub fn f71(v0: List<Adt5>) -> R<Result<Adt0, Adt2>> {
    let a390 = {
        let a389 = v0.clone();
        length_list(a389)
    };
    let a391 = 1u64;
    if nat_eq(a390, a391) {
        let m392 = f37(v0, 0u64);
        match m392 {
            Adt5::C0(v1) => {
                let m393 = f83(v1.clone())?;
                match m393 {
                    None => {
                        Ok(Ok::<Adt0, Adt2>(Adt0::C8(v1)))
                    }
                    Some(v2) => {
                        Ok(Err::<Adt0, Adt2>(v2))
                    }
                }
            }
            Adt5::C1(_) => {
                Ok(Err::<Adt0, Adt2>(f10(f11(Str::lit("a nat is `(nat decimal)`")))))
            }
            Adt5::C2 => {
                Ok(Err::<Adt0, Adt2>(f10(f11(Str::lit("a nat is `(nat decimal)`")))))
            }
            Adt5::C3(_, _) => {
                Ok(Err::<Adt0, Adt2>(f10(f11(Str::lit("a nat is `(nat decimal)`")))))
            }
        }
    } else {
        Ok(Err::<Adt0, Adt2>(f10(f11(Str::lit("a nat is `(nat decimal)`")))))
    }
}

pub fn f72(v0: Fn12, p397: List<Adt8>, p398: List<Adt5>) -> List<Adt8> {
    let m396 = (p397, p398);
    {
        let mut m396 = m396;
        loop {
            let step = {
                let (v1, v2) = m396;
                let m394 = v2;
                match m394.uncons() {
                    None => {
                        Err::<(List<Adt8>, List<Adt5>), List<Adt8>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt8>, List<Adt5>), List<Adt8>>(({
                            let c395: Fn12 = v0.clone();
                            c395.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m396 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f73(v0: List<Adt8>, v1: Adt5) -> List<Adt8> {
    List::cons(Adt8::C0(v1), v0)
}

pub fn f74(v0: List<Adt5>, v1: List<Adt8>, v2: u64) -> R<Result<List<Adt8>, Adt2>> {
    f84(Fn13::F85(v2), Ok::<List<Adt8>, Adt2>(v1), f45(v0))
}

pub fn f75(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a399 = v0.clone();
        length_bytes(a399)
    };
    let a400 = v1;
    let a401 = 0u64;
    if nat_eq(a400, a401) {
        Ok(false)
    } else {
        if f86(f20(v0.clone(), 0u64)?) {
            let a403 = {
                let (h402, _) = f87(Fn14::F88(v1, v0), v1, 1u64)?;
                h402
            };
            let a404 = v1;
            Ok(nat_eq(a403, a404))
        } else {
            Ok(false)
        }
    }
}

pub fn f76(v0: Adt5, v1: u64) -> R<Result<Adt9, Adt2>> {
    let m405 = v0.clone();
    match m405 {
        Adt5::C0(_) => {
            Ok(Err::<Adt9, Adt2>(f10(f26(f11(Str::lit("expected a binder, found ")), f33(v0)))))
        }
        Adt5::C1(r412) => {
            let v2 = (*r412).clone();
            let v3: List<Adt5> = f30(v2, v1)?;
            let a407 = {
                let a406 = v3.clone();
                length_list(a406)
            };
            let a408 = 3u64;
            if nat_eq(a407, a408) {
                let m409 = f40(f37(v3.clone(), 0u64), f11(Str::lit("a binder mode")), v1);
                match m409 {
                    Ok(v4) => {
                        let m410 = f89(v4.clone());
                        match m410 {
                            None => {
                                Ok(Err::<Adt9, Adt2>(f10(f26(f53(v4), f11(Str::lit(" is not a binder mode"))))))
                            }
                            Some(v5) => {
                                let m411 = f40(f37(v3.clone(), 1u64), f11(Str::lit("a local ID")), v1);
                                match m411 {
                                    Ok(v6) => {
                                        if f75(v6.clone())? {
                                            Ok(Ok::<Adt9, Adt2>(Adt9::C0(v5, v6, f37(v3, 2u64))))
                                        } else {
                                            Ok(Err::<Adt9, Adt2>(f10(f26(f53(v6), f11(Str::lit(" is not a valid local ID"))))))
                                        }
                                    }
                                    Err(v7) => {
                                        Ok(Err::<Adt9, Adt2>(v7))
                                    }
                                }
                            }
                        }
                    }
                    Err(v8) => {
                        Ok(Err::<Adt9, Adt2>(v8))
                    }
                }
            } else {
                Ok(Err::<Adt9, Adt2>(f10(f11(Str::lit("a binder is `(mode name type)`")))))
            }
        }
        Adt5::C2 => {
            Ok(Err::<Adt9, Adt2>(f10(f26(f11(Str::lit("expected a binder, found ")), f33(v0)))))
        }
        Adt5::C3(_, _) => {
            Ok(Err::<Adt9, Adt2>(f10(f26(f11(Str::lit("expected a binder, found ")), f33(v0)))))
        }
    }
}

pub fn f77(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h413, _) = v2;
        h413
    };
    let a414 = v3;
    let a415 = 0u64;
    if nat_eq(a414, a415) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a416 = v3;
            let a417 = v1;
            let a418 = 0u64;
            nat_quot(a416, a417, a418)
        }, {
            let a423 = f62(v0, {
                let a419 = v3;
                let a420 = v1;
                let a421 = 0u64;
                nat_rem(a419, a420, a421)
            });
            let a424 = {
                let (_, h422) = v2;
                h422
            };
            append_bytes(a423, a424)
        }))
    }
}

pub fn f78() -> Adt2 {
    f10(f11(Str::lit("a sort is `(sort prop)` or `(sort (type u))`")))
}

pub fn f79(v0: Adt5, v1: u64) -> R<Result<Adt1, Adt2>> {
    let v2: Adt10 = {
        let (h425, _) = f90(Fn15::F91(v1), {
            let a428 = {
                let a426 = v1;
                let a427 = 2u64;
                nat_mul(a426, a427)?
            };
            let a429 = 2u64;
            nat_add(a428, a429)?
        }, Adt10::C0(List::cons(Adt11::C0(v0), List::<Adt11>::nil()), List::<Adt1>::nil(), None::<Adt2>))?;
        h425
    };
    let m431 = match v2.clone() {
        Adt10::C0(_, _, h430) => {
            h430
        }
    };
    match m431 {
        None => {
            let m433 = match v2 {
                Adt10::C0(_, h432, _) => {
                    h432
                }
            };
            match m433.uncons() {
                None => {
                    Ok(Err::<Adt1, Adt2>(f10(f11(Str::lit("expected a universe")))))
                }
                Some((v4, _)) => {
                    Ok(Ok::<Adt1, Adt2>(v4))
                }
            }
        }
        Some(v3) => {
            Ok(Err::<Adt1, Adt2>(v3))
        }
    }
}

pub fn f80(v0: Bytes) -> R<Result<(Bytes, Bytes), Adt2>> {
    let m434 = f92(v0.clone(), f11(Str::lit("::")))?;
    match m434 {
        None => {
            Ok(Err::<(Bytes, Bytes), Adt2>(f10(f26(f53(v0), f11(Str::lit(" is not a qualified `package::entry` ID"))))))
        }
        Some(v1) => {
            let v2: Bytes = f17(v0.clone(), 0u64, v1);
            let v3: Bytes = f17(v0.clone(), {
                let a435 = v1;
                let a436 = 2u64;
                nat_add(a435, a436)?
            }, {
                let a440 = {
                    let a437 = v0;
                    length_bytes(a437)
                };
                let a441 = {
                    let a438 = v1;
                    let a439 = 2u64;
                    nat_add(a438, a439)?
                };
                nat_sub(a440, a441)
            });
            if f93(v2.clone())? {
                if f94(v3.clone())? {
                    Ok(Ok::<(Bytes, Bytes), Adt2>((v2, v3)))
                } else {
                    Ok(Err::<(Bytes, Bytes), Adt2>(f10(f26(f53(v3), f11(Str::lit(" is not a valid local entry ID"))))))
                }
            } else {
                Ok(Err::<(Bytes, Bytes), Adt2>(f10(f26(f53(v2), f11(Str::lit(" is not a valid package ID"))))))
            }
        }
    }
}

pub fn f81() -> Adt2 {
    f10(f11(Str::lit("a const is `(const qualified-id [universes])`")))
}

pub fn f82(v0: List<Adt5>, v1: u64) -> R<Result<Adt1, Adt2>> {
    let m442 = f95(Fn16::F96(v1), Ok::<List<Adt1>, Adt2>(List::<Adt1>::nil()), v0)?;
    match m442 {
        Ok(v2) => {
            Ok(Ok::<Adt1, Adt2>(f97(f98(v2))))
        }
        Err(v3) => {
            Ok(Err::<Adt1, Adt2>(v3))
        }
    }
}

pub fn f83(v0: Bytes) -> R<Option<Adt2>> {
    let v1: u64 = {
        let a443 = v0.clone();
        length_bytes(a443)
    };
    let a444 = v1;
    let a445 = 0u64;
    if if nat_eq(a444, a445) {
        true
    } else {
        !f99(v0.clone())?
    } {
        Ok(Some(f10(f26(f53(v0), f11(Str::lit(" is not a decimal"))))))
    } else {
        let a446 = 1u64;
        let a447 = v1;
        if if nat_lt(a446, a447) {
            let a448 = f20(v0.clone(), 0u64)?;
            let a449 = 48u64;
            nat_eq(a448, a449)
        } else {
            false
        } {
            Ok(Some(f10(f26(f53(v0), f11(Str::lit(" has a redundant leading zero"))))))
        } else {
            let a450 = 20u64;
            let a451 = v1;
            if if nat_lt(a450, a451) {
                true
            } else {
                let a452 = v1;
                let a453 = 20u64;
                if nat_eq(a452, a453) {
                    let a458 = {
                        let a454 = v0.clone();
                        let a455 = f11(Str::lit("18446744073709551615"));
                        compare_bytes(a454, a455)
                    };
                    let a459 = {
                        let a456 = Bytes::lit(&[0x01]);
                        let a457 = Bytes::lit(&[]);
                        compare_bytes(a456, a457)
                    };
                    equal(a458, a459)
                } else {
                    false
                }
            } {
                Ok(Some(f10(f26(f53(v0), f11(Str::lit(" does not fit 64 bits"))))))
            } else {
                Ok(None::<Adt2>)
            }
        }
    }
}

pub fn f84(v0: Fn13, p463: Result<List<Adt8>, Adt2>, p464: List<Adt5>) -> R<Result<List<Adt8>, Adt2>> {
    let m462 = (p463, p464);
    {
        let mut m462 = m462;
        loop {
            let step = {
                let (v1, v2) = m462;
                let m460 = v2;
                match m460.uncons() {
                    None => {
                        Err::<(Result<List<Adt8>, Adt2>, List<Adt5>), R<Result<List<Adt8>, Adt2>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Result<List<Adt8>, Adt2>, List<Adt5>), R<Result<List<Adt8>, Adt2>>>(({
                            let c461: Fn13 = v0.clone();
                            c461.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m462 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f85(v0: u64, v1: Result<List<Adt8>, Adt2>, v2: Adt5) -> R<Result<List<Adt8>, Adt2>> {
    let m465 = v1;
    match m465 {
        Ok(v3) => {
            let m466 = f76(v2, v0)?;
            match m466 {
                Ok(v4) => {
                    Ok(Ok::<List<Adt8>, Adt2>(List::cons(Adt8::C0(match v4 {
                        Adt9::C0(_, _, h467) => {
                            h467
                        }
                    }), v3)))
                }
                Err(v5) => {
                    Ok(Err::<List<Adt8>, Adt2>(v5))
                }
            }
        }
        Err(v6) => {
            Ok(Err::<List<Adt8>, Adt2>(v6))
        }
    }
}

pub fn f86(v0: u64) -> bool {
    if f100(v0) {
        true
    } else {
        let a468 = v0;
        let a469 = 65u64;
        if nat_lt(a468, a469) {
            false
        } else {
            let a470 = v0;
            let a471 = 90u64;
            nat_le(a470, a471)
        }
    }
}

pub fn f87(v0: Fn14, p476: u64, p477: u64) -> R<(u64, bool)> {
    let m475 = (p476, p477);
    {
        let mut m475 = m475;
        loop {
            let step = {
                let (v1, v2) = m475;
                let m472 = v1;
                if m472 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m472 - 1;
                    let m474 = {
                        let c473: Fn14 = v0.clone();
                        c473.apply(v2)?
                    };
                    match m474 {
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
                Ok(next) => { m475 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f88(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f101(v1, v0, v2)
}

pub fn f89(v0: Bytes) -> Option<u64> {
    let a478 = v0.clone();
    let a479 = f11(Str::lit("explicit"));
    if equal(a478, a479) {
        Some(0u64)
    } else {
        let a480 = v0.clone();
        let a481 = f11(Str::lit("implicit"));
        if equal(a480, a481) {
            Some(1u64)
        } else {
            let a482 = v0;
            let a483 = f11(Str::lit("instance"));
            if equal(a482, a483) {
                Some(2u64)
            } else {
                None::<u64>
            }
        }
    }
}

pub fn f90(v0: Fn15, p488: u64, p489: Adt10) -> R<(Adt10, bool)> {
    let m487 = (p488, p489);
    {
        let mut m487 = m487;
        loop {
            let step = {
                let (v1, v2) = m487;
                let m484 = v1;
                if m484 == 0 {
                    Err::<(u64, Adt10), R<(Adt10, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m484 - 1;
                    let m486 = {
                        let c485: Fn15 = v0.clone();
                        c485.apply(v2.clone())?
                    };
                    match m486 {
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
                Ok(next) => { m487 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f91(v0: u64, v1: Adt10) -> R<Option<Adt10>> {
    f102(v0, v1)
}

pub fn f92(v0: Bytes, v1: Bytes) -> R<Option<u64>> {
    let v2: u64 = {
        let a490 = v0.clone();
        length_bytes(a490)
    };
    let v3: u64 = {
        let a491 = v1.clone();
        length_bytes(a491)
    };
    let v4: u64 = {
        let (h492, _) = f87(Fn14::F103(v1.clone(), v3, v0.clone(), v2), {
            let a493 = v2;
            let a494 = 1u64;
            nat_add(a493, a494)?
        }, 0u64)?;
        h492
    };
    if f104(v0, v1, v4) {
        Ok(Some(v4))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f93(v0: Bytes) -> R<bool> {
    f105(v0, false)
}

pub fn f94(v0: Bytes) -> R<bool> {
    f105(v0, true)
}

pub fn f95(v0: Fn16, p498: Result<List<Adt1>, Adt2>, p499: List<Adt5>) -> R<Result<List<Adt1>, Adt2>> {
    let m497 = (p498, p499);
    {
        let mut m497 = m497;
        loop {
            let step = {
                let (v1, v2) = m497;
                let m495 = v2;
                match m495.uncons() {
                    None => {
                        Err::<(Result<List<Adt1>, Adt2>, List<Adt5>), R<Result<List<Adt1>, Adt2>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Result<List<Adt1>, Adt2>, List<Adt5>), R<Result<List<Adt1>, Adt2>>>(({
                            let c496: Fn16 = v0.clone();
                            c496.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m497 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f96(v0: u64, v1: Result<List<Adt1>, Adt2>, v2: Adt5) -> R<Result<List<Adt1>, Adt2>> {
    let m500 = v1;
    match m500 {
        Ok(v3) => {
            let m501 = f79(v2, v0)?;
            match m501 {
                Ok(v4) => {
                    Ok(Ok::<List<Adt1>, Adt2>(List::cons(v4, v3)))
                }
                Err(v5) => {
                    Ok(Err::<List<Adt1>, Adt2>(v5))
                }
            }
        }
        Err(v6) => {
            Ok(Err::<List<Adt1>, Adt2>(v6))
        }
    }
}

pub fn f97(v0: List<Adt1>) -> Adt1 {
    f106(Fn17::F107, Adt1::C5, f98(v0))
}

pub fn f98(v0: List<Adt1>) -> List<Adt1> {
    f108(Fn18::F109, List::<Adt1>::nil(), v0)
}

pub fn f99(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a502 = v0.clone();
        length_bytes(a502)
    };
    let a506 = {
        let (h503, _) = f87(Fn14::F110(v1, v0), {
            let a504 = v1;
            let a505 = 1u64;
            nat_add(a504, a505)?
        }, 0u64)?;
        h503
    };
    let a507 = v1;
    Ok(nat_eq(a506, a507))
}

pub fn f100(v0: u64) -> bool {
    let a508 = v0;
    let a509 = 97u64;
    if nat_lt(a508, a509) {
        false
    } else {
        let a510 = v0;
        let a511 = 122u64;
        nat_le(a510, a511)
    }
}

pub fn f101(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let a512 = v2;
    let a513 = v1;
    if nat_lt(a512, a513) {
        if f111(f20(v0, v2)?) {
            Ok(Some({
                let a514 = v2;
                let a515 = 1u64;
                nat_add(a514, a515)?
            }))
        } else {
            Ok(None::<u64>)
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f102(v0: u64, v1: Adt10) -> R<Option<Adt10>> {
    let m517 = match v1.clone() {
        Adt10::C0(_, _, h516) => {
            h516
        }
    };
    match m517 {
        None => {
            let m519 = match v1.clone() {
                Adt10::C0(h518, _, _) => {
                    h518
                }
            };
            match m519.uncons() {
                None => {
                    Ok(None::<Adt10>)
                }
                Some((v3, v4)) => {
                    let m520 = v3;
                    match m520 {
                        Adt11::C0(v5) => {
                            Ok(Some(f112(v0, v1, v4, v5)?))
                        }
                        Adt11::C1(v6, v7) => {
                            Ok(Some(f113(v1, v4, v6, v7)))
                        }
                    }
                }
            }
        }
        Some(_) => {
            Ok(None::<Adt10>)
        }
    }
}

pub fn f103(v0: Bytes, v1: u64, v2: Bytes, v3: u64, v4: u64) -> R<Option<u64>> {
    if f104(v2, v0, v4) {
        Ok(None::<u64>)
    } else {
        let a523 = {
            let a521 = v4;
            let a522 = v1;
            nat_add(a521, a522)?
        };
        let a524 = v3;
        if nat_lt(a523, a524) {
            Ok(Some({
                let a525 = v4;
                let a526 = 1u64;
                nat_add(a525, a526)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f104(v0: Bytes, v1: Bytes, v2: u64) -> bool {
    let m531 = {
        let a528 = v0;
        let a529 = v2;
        let a530 = {
            let a527 = v1.clone();
            length_bytes(a527)
        };
        slice_bytes(a528, a529, a530)
    };
    match m531 {
        None => {
            false
        }
        Some(v3) => {
            let a532 = v3;
            let a533 = v1;
            equal(a532, a533)
        }
    }
}

pub fn f105(v0: Bytes, v1: bool) -> R<bool> {
    let v2: u64 = {
        let a534 = v0.clone();
        length_bytes(a534)
    };
    let v3: Adt12 = {
        let (h535, _) = f114(Fn19::F115(v1, v2, v0), {
            let a536 = v2;
            let a537 = 1u64;
            nat_add(a536, a537)?
        }, Adt12::C0(0u64, true, true, true))?;
        h535
    };
    if match v3.clone() {
        Adt12::C0(_, _, _, h538) => {
            h538
        }
    } {
        Ok(!match v3 {
            Adt12::C0(_, h539, _, _) => {
                h539
            }
        })
    } else {
        Ok(false)
    }
}

pub fn f106(v0: Fn17, p543: Adt1, p544: List<Adt1>) -> Adt1 {
    let m542 = (p543, p544);
    {
        let mut m542 = m542;
        loop {
            let step = {
                let (v1, v2) = m542;
                let m540 = v2;
                match m540.uncons() {
                    None => {
                        Err::<(Adt1, List<Adt1>), Adt1>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt1, List<Adt1>), Adt1>(({
                            let c541: Fn17 = v0.clone();
                            c541.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m542 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f107(v0: Adt1, v1: Adt1) -> Adt1 {
    Adt1::C6(std::rc::Rc::new(v1), std::rc::Rc::new(v0))
}

pub fn f108(v0: Fn18, p548: List<Adt1>, p549: List<Adt1>) -> List<Adt1> {
    let m547 = (p548, p549);
    {
        let mut m547 = m547;
        loop {
            let step = {
                let (v1, v2) = m547;
                let m545 = v2;
                match m545.uncons() {
                    None => {
                        Err::<(List<Adt1>, List<Adt1>), List<Adt1>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt1>, List<Adt1>), List<Adt1>>(({
                            let c546: Fn18 = v0.clone();
                            c546.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m547 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f109(v0: List<Adt1>, v1: Adt1) -> List<Adt1> {
    List::cons(v1, v0)
}

pub fn f110(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f116(v1, v0, v2)
}

pub fn f111(v0: u64) -> bool {
    if f86(v0) {
        true
    } else {
        f117(v0)
    }
}

pub fn f112(v0: u64, v1: Adt10, v2: List<Adt11>, v3: Adt5) -> R<Adt10> {
    let m550 = v3;
    match m550 {
        Adt5::C0(v4) => {
            if f99(v4.clone())? {
                let m551 = f83(v4.clone())?;
                match m551 {
                    None => {
                        Ok(Adt10::C0(v2, List::cons(Adt1::C0(f119(v4)?), match v1 {
                            Adt10::C0(_, h552, _) => {
                                h552
                            }
                        }), None::<Adt2>))
                    }
                    Some(v5) => {
                        Ok(f118(v1, v5))
                    }
                }
            } else {
                if f75(v4.clone())? {
                    Ok(Adt10::C0(v2, List::cons(Adt1::C1(v4), match v1 {
                        Adt10::C0(_, h553, _) => {
                            h553
                        }
                    }), None::<Adt2>))
                } else {
                    Ok(f118(v1, f10(f26(f53(v4), f11(Str::lit(" is not a universe"))))))
                }
            }
        }
        Adt5::C1(r576) => {
            let v6 = (*r576).clone();
            let v7: List<Adt5> = f30(v6, v0)?;
            let m554 = v7;
            match m554.uncons() {
                None => {
                    Ok(f118(v1, f10(f11(Str::lit("expected a universe form")))))
                }
                Some((v8, v9)) => {
                    let m555 = v8;
                    match m555 {
                        Adt5::C0(v10) => {
                            let v11: u64 = {
                                let a556 = v9.clone();
                                length_list(a556)
                            };
                            let v12: bool = {
                                let a557 = v10.clone();
                                let a558 = f11(Str::lit("succ"));
                                if equal(a557, a558) {
                                    true
                                } else {
                                    let a559 = v10.clone();
                                    let a560 = f11(Str::lit("max"));
                                    if equal(a559, a560) {
                                        true
                                    } else {
                                        let a561 = v10.clone();
                                        let a562 = f11(Str::lit("imax"));
                                        equal(a561, a562)
                                    }
                                }
                            };
                            let v13: bool = {
                                let a563 = v10.clone();
                                let a564 = f11(Str::lit("succ"));
                                if equal(a563, a564) {
                                    let a565 = v11;
                                    let a566 = 1u64;
                                    nat_eq(a565, a566)
                                } else {
                                    let a567 = v10.clone();
                                    let a568 = f11(Str::lit("max"));
                                    if equal(a567, a568) {
                                        let a569 = 2u64;
                                        let a570 = v11;
                                        nat_le(a569, a570)
                                    } else {
                                        let a571 = v10.clone();
                                        let a572 = f11(Str::lit("imax"));
                                        if equal(a571, a572) {
                                            let a573 = v11;
                                            let a574 = 2u64;
                                            nat_eq(a573, a574)
                                        } else {
                                            false
                                        }
                                    }
                                }
                            };
                            if v13 {
                                Ok(Adt10::C0(f120(v9, List::cons(Adt11::C1(v10, v11), v2)), match v1 {
                                    Adt10::C0(_, h575, _) => {
                                        h575
                                    }
                                }, None::<Adt2>))
                            } else {
                                if v12 {
                                    Ok(f118(v1, f10(f26(f26(f11(Str::lit("`(")), v10), f11(Str::lit(" ...)` has the wrong arity"))))))
                                } else {
                                    Ok(f118(v1, f10(f26(f53(v10), f11(Str::lit(" is not a universe form"))))))
                                }
                            }
                        }
                        Adt5::C1(_) => {
                            Ok(f118(v1, f10(f11(Str::lit("expected a universe form")))))
                        }
                        Adt5::C2 => {
                            Ok(f118(v1, f10(f11(Str::lit("expected a universe form")))))
                        }
                        Adt5::C3(_, _) => {
                            Ok(f118(v1, f10(f11(Str::lit("expected a universe form")))))
                        }
                    }
                }
            }
        }
        Adt5::C2 => {
            Ok(f118(v1, f10(f11(Str::lit("expected a universe form")))))
        }
        Adt5::C3(_, _) => {
            Ok(f118(v1, f10(f11(Str::lit("expected a universe form")))))
        }
    }
}

pub fn f113(v0: Adt10, v1: List<Adt11>, v2: Bytes, v3: u64) -> Adt10 {
    let v4: (List<Adt1>, List<Adt1>) = f121(match v0 {
        Adt10::C0(_, h577, _) => {
            h577
        }
    }, v3);
    let v5: List<Adt1> = {
        let (h578, _) = v4;
        h578
    };
    let v12: Adt1 = {
        let a579 = v2.clone();
        let a580 = f11(Str::lit("succ"));
        if equal(a579, a580) {
            let m581 = v5;
            match m581.uncons() {
                None => {
                    Adt1::C5
                }
                Some((v6, _)) => {
                    Adt1::C2(std::rc::Rc::new(v6))
                }
            }
        } else {
            let a582 = v2;
            let a583 = f11(Str::lit("imax"));
            if equal(a582, a583) {
                let m584 = v5;
                match m584.uncons() {
                    None => {
                        Adt1::C5
                    }
                    Some((v8, v9)) => {
                        let m585 = v9;
                        match m585.uncons() {
                            None => {
                                Adt1::C5
                            }
                            Some((v10, _)) => {
                                Adt1::C4(std::rc::Rc::new(v8), std::rc::Rc::new(v10))
                            }
                        }
                    }
                }
            } else {
                Adt1::C3(std::rc::Rc::new(f97(v5)))
            }
        }
    };
    Adt10::C0(v1, List::cons(v12, {
        let (_, h586) = v4;
        h586
    }), None::<Adt2>)
}

pub fn f114(v0: Fn19, p591: u64, p592: Adt12) -> R<(Adt12, bool)> {
    let m590 = (p591, p592);
    {
        let mut m590 = m590;
        loop {
            let step = {
                let (v1, v2) = m590;
                let m587 = v1;
                if m587 == 0 {
                    Err::<(u64, Adt12), R<(Adt12, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m587 - 1;
                    let m589 = {
                        let c588: Fn19 = v0.clone();
                        c588.apply(v2.clone())?
                    };
                    match m589 {
                        None => {
                            Err::<(u64, Adt12), R<(Adt12, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt12), R<(Adt12, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m590 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f115(v0: bool, v1: u64, v2: Bytes, v3: Adt12) -> R<Option<Adt12>> {
    f122(v2, v1, v0, v3)
}

pub fn f116(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let a593 = v2;
    let a594 = v1;
    if nat_lt(a593, a594) {
        if f117(f20(v0, v2)?) {
            Ok(Some({
                let a595 = v2;
                let a596 = 1u64;
                nat_add(a595, a596)?
            }))
        } else {
            Ok(None::<u64>)
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f117(v0: u64) -> bool {
    let a597 = v0;
    let a598 = 48u64;
    if nat_lt(a597, a598) {
        false
    } else {
        let a599 = v0;
        let a600 = 57u64;
        nat_le(a599, a600)
    }
}

pub fn f118(v0: Adt10, v1: Adt2) -> Adt10 {
    Adt10::C0(match v0.clone() {
        Adt10::C0(h601, _, _) => {
            h601
        }
    }, match v0 {
        Adt10::C0(_, h602, _) => {
            h602
        }
    }, Some(v1))
}

pub fn f119(v0: Bytes) -> R<u64> {
    let m603 = f123(v0)?;
    match m603 {
        None => {
            Ok(0u64)
        }
        Some(v1) => {
            Ok(v1)
        }
    }
}

pub fn f120(v0: List<Adt5>, v1: List<Adt11>) -> List<Adt11> {
    f124(Fn20::F125, v1, f45(v0))
}

pub fn f121(v0: List<Adt1>, v1: u64) -> (List<Adt1>, List<Adt1>) {
    let (h604, _) = f126(Fn21::F127, v1, (List::<Adt1>::nil(), v0));
    h604
}

pub fn f122(v0: Bytes, v1: u64, v2: bool, v3: Adt12) -> R<Option<Adt12>> {
    if match v3.clone() {
        Adt12::C0(_, _, _, h605) => {
            h605
        }
    } {
        let a607 = match v3.clone() {
            Adt12::C0(h606, _, _, _) => {
                h606
            }
        };
        let a608 = v1;
        if nat_lt(a607, a608) {
            let v4: u64 = f20(v0, match v3.clone() {
                Adt12::C0(h609, _, _, _) => {
                    h609
                }
            })?;
            let a610 = v4;
            let a611 = 46u64;
            if nat_eq(a610, a611) {
                if match v3.clone() {
                    Adt12::C0(_, h612, _, _) => {
                        h612
                    }
                } {
                    Ok(f128(v3))
                } else {
                    f129(v3, true, false)
                }
            } else {
                if match v3.clone() {
                    Adt12::C0(_, h613, _, _) => {
                        h613
                    }
                } {
                    if f100(v4) {
                        f129(v3.clone(), false, match v3 {
                            Adt12::C0(_, _, h614, _) => {
                                h614
                            }
                        })
                    } else {
                        Ok(f128(v3))
                    }
                } else {
                    if if f100(v4) {
                        true
                    } else {
                        f117(v4)
                    } {
                        f129(v3.clone(), false, match v3 {
                            Adt12::C0(_, _, h615, _) => {
                                h615
                            }
                        })
                    } else {
                        let a616 = v4;
                        let a617 = 45u64;
                        if if nat_eq(a616, a617) {
                            if v2 {
                                true
                            } else {
                                !match v3.clone() {
                                    Adt12::C0(_, _, h618, _) => {
                                        h618
                                    }
                                }
                            }
                        } else {
                            false
                        } {
                            f129(v3.clone(), false, match v3 {
                                Adt12::C0(_, _, h619, _) => {
                                    h619
                                }
                            })
                        } else {
                            Ok(f128(v3))
                        }
                    }
                }
            }
        } else {
            Ok(None::<Adt12>)
        }
    } else {
        Ok(None::<Adt12>)
    }
}

pub fn f123(v0: Bytes) -> R<Option<u64>> {
    if f130(v0.clone())? {
        Ok(Some({
            let (_, h620) = {
                let (h621, _) = f131(Fn22::F132(v0.clone()), {
                    let a622 = v0;
                    length_bytes(a622)
                }, (0u64, 0u64))?;
                h621
            };
            h620
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f124(v0: Fn20, p626: List<Adt11>, p627: List<Adt5>) -> List<Adt11> {
    let m625 = (p626, p627);
    {
        let mut m625 = m625;
        loop {
            let step = {
                let (v1, v2) = m625;
                let m623 = v2;
                match m623.uncons() {
                    None => {
                        Err::<(List<Adt11>, List<Adt5>), List<Adt11>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt11>, List<Adt5>), List<Adt11>>(({
                            let c624: Fn20 = v0.clone();
                            c624.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m625 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f125(v0: List<Adt11>, v1: Adt5) -> List<Adt11> {
    List::cons(Adt11::C0(v1), v0)
}

pub fn f126(v0: Fn21, p632: u64, p633: (List<Adt1>, List<Adt1>)) -> ((List<Adt1>, List<Adt1>), bool) {
    let m631 = (p632, p633);
    {
        let mut m631 = m631;
        loop {
            let step = {
                let (v1, v2) = m631;
                let m628 = v1;
                if m628 == 0 {
                    Err::<(u64, (List<Adt1>, List<Adt1>)), ((List<Adt1>, List<Adt1>), bool)>((v2, false))
                } else {
                    let v3 = m628 - 1;
                    let m630 = {
                        let c629: Fn21 = v0.clone();
                        c629.apply(v2.clone())
                    };
                    match m630 {
                        None => {
                            Err::<(u64, (List<Adt1>, List<Adt1>)), ((List<Adt1>, List<Adt1>), bool)>((v2, true))
                        }
                        Some(v4) => {
                            Ok::<(u64, (List<Adt1>, List<Adt1>)), ((List<Adt1>, List<Adt1>), bool)>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m631 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f127(v0: (List<Adt1>, List<Adt1>)) -> Option<(List<Adt1>, List<Adt1>)> {
    let m635 = {
        let (_, h634) = v0;
        h634
    };
    match m635.uncons() {
        None => {
            None::<(List<Adt1>, List<Adt1>)>
        }
        Some((v1, v2)) => {
            Some((List::cons(v1, {
                let (h636, _) = v0;
                h636
            }), v2))
        }
    }
}

pub fn f128(v0: Adt12) -> Option<Adt12> {
    Some(Adt12::C0(match v0.clone() {
        Adt12::C0(h637, _, _, _) => {
            h637
        }
    }, match v0.clone() {
        Adt12::C0(_, h638, _, _) => {
            h638
        }
    }, match v0 {
        Adt12::C0(_, _, h639, _) => {
            h639
        }
    }, false))
}

pub fn f129(v0: Adt12, v1: bool, v2: bool) -> R<Option<Adt12>> {
    Ok(Some(Adt12::C0({
        let a641 = match v0 {
            Adt12::C0(h640, _, _, _) => {
                h640
            }
        };
        let a642 = 1u64;
        nat_add(a641, a642)?
    }, v1, v2, true)))
}

pub fn f130(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a643 = v0.clone();
        length_bytes(a643)
    };
    let a644 = 0u64;
    let a645 = v1;
    if nat_lt(a644, a645) {
        let a646 = f133(v0.clone())?;
        let a647 = v1;
        if nat_eq(a646, a647) {
            let a648 = 1u64;
            let a649 = v1;
            if nat_lt(a648, a649) {
                let m652 = {
                    let a650 = v0;
                    let a651 = 0u64;
                    index_bytes(a650, a651)
                };
                match m652 {
                    None => {
                        Ok(false)
                    }
                    Some(v2) => {
                        let a653 = v2;
                        let a654 = 48u8;
                        Ok(!equal(a653, a654))
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

pub fn f131(v0: Fn22, p659: u64, p660: (u64, u64)) -> R<((u64, u64), bool)> {
    let m658 = (p659, p660);
    {
        let mut m658 = m658;
        loop {
            let step = {
                let (v1, v2) = m658;
                let m655 = v1;
                if m655 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m655 - 1;
                    let m657 = {
                        let c656: Fn22 = v0.clone();
                        c656.apply(v2)?
                    };
                    match m657 {
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
                Ok(next) => { m658 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f132(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f134(v0, v1)
}

pub fn f133(v0: Bytes) -> R<u64> {
    let (h661, _) = f87(Fn14::F135(v0.clone()), {
        let a662 = v0;
        length_bytes(a662)
    }, 0u64)?;
    Ok(h661)
}

pub fn f134(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m666 = {
        let a664 = v0;
        let a665 = {
            let (h663, _) = v1;
            h663
        };
        index_bytes(a664, a665)
    };
    match m666 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a668 = {
                    let (h667, _) = v1;
                    h667
                };
                let a669 = 1u64;
                nat_add(a668, a669)?
            }, {
                let a673 = {
                    let a671 = {
                        let (_, h670) = v1;
                        h670
                    };
                    let a672 = 10u64;
                    nat_mul(a671, a672)?
                };
                let a674 = f136(v2)?;
                nat_add(a673, a674)?
            })))
        }
    }
}

pub fn f135(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m677 = {
        let a675 = v0;
        let a676 = v1;
        index_bytes(a675, a676)
    };
    match m677 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f137(v2)? {
                Ok(Some({
                    let a678 = v1;
                    let a679 = 1u64;
                    nat_add(a678, a679)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f136(v0: u8) -> R<u64> {
    let a726 = {
        let v1: u8 = v0;
        let a724 = {
            let a700 = {
                let a688 = {
                    let a682 = {
                        let a680 = v1;
                        let a681 = 128u8;
                        fixed_u8::bit_and(a680, a681)
                    };
                    let a683 = 128u8;
                    if equal(a682, a683) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a689 = {
                    let a686 = {
                        let a684 = v1;
                        let a685 = 64u8;
                        fixed_u8::bit_and(a684, a685)
                    };
                    let a687 = 64u8;
                    if equal(a686, a687) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a688, a689)?
            };
            let a701 = {
                let a698 = {
                    let a692 = {
                        let a690 = v1;
                        let a691 = 32u8;
                        fixed_u8::bit_and(a690, a691)
                    };
                    let a693 = 32u8;
                    if equal(a692, a693) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a699 = {
                    let a696 = {
                        let a694 = v1;
                        let a695 = 16u8;
                        fixed_u8::bit_and(a694, a695)
                    };
                    let a697 = 16u8;
                    if equal(a696, a697) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a698, a699)?
            };
            nat_add(a700, a701)?
        };
        let a725 = {
            let a722 = {
                let a710 = {
                    let a704 = {
                        let a702 = v1;
                        let a703 = 8u8;
                        fixed_u8::bit_and(a702, a703)
                    };
                    let a705 = 8u8;
                    if equal(a704, a705) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a711 = {
                    let a708 = {
                        let a706 = v1;
                        let a707 = 4u8;
                        fixed_u8::bit_and(a706, a707)
                    };
                    let a709 = 4u8;
                    if equal(a708, a709) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a710, a711)?
            };
            let a723 = {
                let a720 = {
                    let a714 = {
                        let a712 = v1;
                        let a713 = 2u8;
                        fixed_u8::bit_and(a712, a713)
                    };
                    let a715 = 2u8;
                    if equal(a714, a715) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a721 = {
                    let a718 = {
                        let a716 = v1;
                        let a717 = 1u8;
                        fixed_u8::bit_and(a716, a717)
                    };
                    let a719 = 1u8;
                    if equal(a718, a719) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a720, a721)?
            };
            nat_add(a722, a723)?
        };
        nat_add(a724, a725)?
    };
    let a727 = 48u64;
    Ok(nat_sub(a726, a727))
}

pub fn f137(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a772 = {
            let a748 = {
                let a736 = {
                    let a730 = {
                        let a728 = v1;
                        let a729 = 128u8;
                        fixed_u8::bit_and(a728, a729)
                    };
                    let a731 = 128u8;
                    if equal(a730, a731) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a737 = {
                    let a734 = {
                        let a732 = v1;
                        let a733 = 64u8;
                        fixed_u8::bit_and(a732, a733)
                    };
                    let a735 = 64u8;
                    if equal(a734, a735) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a736, a737)?
            };
            let a749 = {
                let a746 = {
                    let a740 = {
                        let a738 = v1;
                        let a739 = 32u8;
                        fixed_u8::bit_and(a738, a739)
                    };
                    let a741 = 32u8;
                    if equal(a740, a741) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a747 = {
                    let a744 = {
                        let a742 = v1;
                        let a743 = 16u8;
                        fixed_u8::bit_and(a742, a743)
                    };
                    let a745 = 16u8;
                    if equal(a744, a745) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a746, a747)?
            };
            nat_add(a748, a749)?
        };
        let a773 = {
            let a770 = {
                let a758 = {
                    let a752 = {
                        let a750 = v1;
                        let a751 = 8u8;
                        fixed_u8::bit_and(a750, a751)
                    };
                    let a753 = 8u8;
                    if equal(a752, a753) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a759 = {
                    let a756 = {
                        let a754 = v1;
                        let a755 = 4u8;
                        fixed_u8::bit_and(a754, a755)
                    };
                    let a757 = 4u8;
                    if equal(a756, a757) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a758, a759)?
            };
            let a771 = {
                let a768 = {
                    let a762 = {
                        let a760 = v1;
                        let a761 = 2u8;
                        fixed_u8::bit_and(a760, a761)
                    };
                    let a763 = 2u8;
                    if equal(a762, a763) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a769 = {
                    let a766 = {
                        let a764 = v1;
                        let a765 = 1u8;
                        fixed_u8::bit_and(a764, a765)
                    };
                    let a767 = 1u8;
                    if equal(a766, a767) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a768, a769)?
            };
            nat_add(a770, a771)?
        };
        nat_add(a772, a773)?
    };
    let a774 = 48u64;
    let a775 = v2;
    if nat_le(a774, a775) {
        let a776 = v2;
        let a777 = 57u64;
        Ok(nat_le(a776, a777))
    } else {
        Ok(false)
    }
}

pub fn run(p0: Bytes, p1: u64) -> R<Result<Adt0, Adt2>> {
    f0(p0, p1)
}
