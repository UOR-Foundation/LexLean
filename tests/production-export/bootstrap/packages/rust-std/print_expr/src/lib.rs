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
    C0(Adt1, Adt4),
    C1(Adt5, Adt6, Adt4),
    C2(Adt7, std::rc::Rc<Adt0>, Adt4),
    C3(Adt8, std::rc::Rc<Adt0>, bool, Adt4),
    C4(Adt6, std::rc::Rc<Adt0>, bool, Adt4),
    C5(Adt10, std::rc::Rc<Adt0>, Adt4),
    C6(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>, Adt4),
    C7(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>, std::rc::Rc<Adt0>, Adt4),
    C8(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>, Adt4),
    C9(std::rc::Rc<Adt0>),
    C10(Adt16, Adt6, Adt4),
    C11(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
    C12,
    C13(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
    C14(Adt17, bool, Adt11, std::rc::Rc<Adt0>, Adt4, std::rc::Rc<Adt0>),
    C15(Adt17, std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
    C16(Adt6, std::rc::Rc<Adt0>),
}

#[derive(Clone)]
pub enum Adt1 {
    C0,
    C1(bool),
    C2(u64),
    C3(bool, u64),
    C4(Adt2, bool, u64),
    C5(Bytes),
    C6(Bytes),
    C7(Adt3),
    C8,
}

#[derive(Clone)]
pub enum Adt2 {
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
pub enum Adt3 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt4 {
    C0(Bytes, Option<Adt2>),
}

#[derive(Clone)]
pub enum Adt5 {
    C0,
    C1,
    C2,
    C3,
    C4,
}

#[derive(Clone)]
pub enum Adt6 {
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
pub enum Adt7 {
    C0,
    C1,
    C2,
    C3,
}

#[derive(Clone)]
pub enum Adt8 {
    C0(u64),
    C1(Adt9),
    C2(u64, Option<Adt2>),
}

#[derive(Clone)]
pub enum Adt9 {
    C0(u64, Option<Adt2>),
}

#[derive(Clone)]
pub enum Adt10 {
    C0(Adt11),
    C1,
    C2(Adt11, Adt11),
    C3(Adt11, Adt11),
    C4(u64, u64),
    C5(u64, u64),
    C6,
    C7(Adt11),
}

#[derive(Clone)]
pub enum Adt11 {
    C0(Adt12),
    C1(Adt2),
    C2(Adt13, std::rc::Rc<Adt11>),
    C3(Adt14, std::rc::Rc<Adt11>, std::rc::Rc<Adt11>),
    C4(Adt15, u64),
    C5,
    C6(std::rc::Rc<Adt11>, std::rc::Rc<Adt11>),
}

#[derive(Clone)]
pub enum Adt12 {
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
pub enum Adt13 {
    C0,
    C1,
    C2,
    C3,
    C4,
}

#[derive(Clone)]
pub enum Adt14 {
    C0,
    C1,
}

#[derive(Clone)]
pub enum Adt15 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt16 {
    C0,
    C1,
    C2,
    C3,
}

#[derive(Clone)]
pub enum Adt17 {
    C0(Adt18),
    C1(Adt6),
    C2(std::rc::Rc<Adt17>),
    C3(Adt19, std::rc::Rc<Adt17>),
    C4(u64, u64, std::rc::Rc<Adt17>),
    C5,
    C6(std::rc::Rc<Adt17>, std::rc::Rc<Adt17>),
    C7(Adt3),
}

#[derive(Clone)]
pub enum Adt18 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt19 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt20 {
    C0(List<Bytes>, Bytes, u64, u64, bool),
}

#[derive(Clone)]
pub enum Fn0 {
    F28,
}

impl Fn0 {
    pub fn apply(&self, p0: List<Bytes>) -> Option<List<Bytes>> {
        match self {
            Fn0::F28 => f28(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F30,
}

impl Fn1 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn1::F30 => f30(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F45(u64),
    F54(Bytes),
    F55(Bytes),
}

impl Fn2 {
    pub fn apply(&self, p0: (u64, Adt20)) -> R<Option<(u64, Adt20)>> {
        match self {
            Fn2::F45(k0) => f45(*k0, p0),
            Fn2::F54(k0) => f54(k0.clone(), p0),
            Fn2::F55(k0) => f55(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F61,
}

impl Fn3 {
    pub fn apply(&self, p0: (Option<Bytes>, List<Bytes>), p1: Bytes) -> (Option<Bytes>, List<Bytes>) {
        match self {
            Fn3::F61 => f61(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F66(Bytes),
    F76(Bytes),
}

impl Fn4 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn4::F66(k0) => f66(k0.clone(), p0),
            Fn4::F76(k0) => f76(k0.clone(), p0),
        }
    }
}

pub fn f0(v0: u64, v1: Adt0) -> R<Result<Bytes, Bytes>> {
    Ok(f1(f2(f3(v0), v1, 0u64, 100u64, 0u64)?))
}

pub fn f1(v0: Adt20) -> Result<Bytes, Bytes> {
    if match v0.clone() {
        Adt20::C0(_, _, _, _, h1) => {
            h1
        }
    } {
        Err::<Bytes, Bytes>({
            let a2 = Str::lit("output exceeds the capacity of the buffer");
            utf8_encode(a2)
        })
    } else {
        Ok::<Bytes, Bytes>(f4(f5(List::cons(match v0.clone() {
            Adt20::C0(_, h3, _, _, _) => {
                h3
            }
        }, match v0 {
            Adt20::C0(h4, _, _, _, _) => {
                h4
            }
        }))))
    }
}

pub fn f2(v0: Adt20, v1: Adt0, v2: u64, v3: u64, v4: u64) -> R<Adt20> {
    let m5 = v1;
    match m5 {
        Adt0::C0(v5, _) => {
            f6(v0, v5)
        }
        Adt0::C1(v7, v8, _) => {
            let m6 = v7;
            match m6 {
                Adt5::C0 => {
                    f7(v0, v8)
                }
                Adt5::C1 => {
                    f8(f7(v0, v8)?, Str::lit(".clone()"))
                }
                Adt5::C2 => {
                    f7(v0, v8)
                }
                Adt5::C3 => {
                    f7(f8(v0, Str::lit("*"))?, v8)
                }
                Adt5::C4 => {
                    f8(f7(f8(v0, Str::lit("(*"))?, v8)?, Str::lit(").clone()"))
                }
            }
        }
        Adt0::C2(v10, r63, _) => {
            let v11 = (*r63).clone();
            let m7 = v10;
            match m7 {
                Adt7::C0 => {
                    let a8 = f9(v11.clone());
                    let a9 = 0u64;
                    if if nat_eq(a8, a9) {
                        true
                    } else {
                        let a10 = f9(v11.clone());
                        let a11 = 1u64;
                        if nat_eq(a10, a11) {
                            true
                        } else {
                            let a12 = f9(v11.clone());
                            let a13 = 3u64;
                            nat_eq(a12, a13)
                        }
                    } {
                        f8(f2(f8(v0, Str::lit("!("))?, v11, v2, 100u64, 0u64)?, Str::lit(")"))
                    } else {
                        f2(f8(v0, Str::lit("!"))?, v11, v2, 100u64, 0u64)
                    }
                }
                Adt7::C1 => {
                    f8(f2(f8(v0, Str::lit("std::rc::Rc::new("))?, v11, v2, 100u64, 0u64)?, Str::lit(")"))
                }
                Adt7::C2 => {
                    f8(f2(f8(v0, Str::lit("i128::from("))?, v11, v2, 100u64, 0u64)?, Str::lit(")"))
                }
                Adt7::C3 => {
                    f8(f2(f8(v0, Str::lit("Ok("))?, v11, v2, 100u64, 0u64)?, Str::lit(")"))
                }
            }
        }
        Adt0::C3(v13, r64, v15, _) => {
            let v14 = (*r64).clone();
            let v17: u64 = f10(v13.clone());
            let v22: Adt20 = {
                let m14 = v13.clone();
                match m14 {
                    Adt8::C0(v18) => {
                        f8(f11(f8(v0, Str::lit("f"))?, v18)?, Str::lit("("))?
                    }
                    Adt8::C1(v19) => {
                        f8(f12(v0, v19)?, Str::lit("("))?
                    }
                    Adt8::C2(v20, v21) => {
                        f13(v0, v20, v21)?
                    }
                }
            };
            let v23: Adt20 = f2(v22, v14, v2, v17, 0u64)?;
            let v28: Adt20 = {
                let m15 = v13;
                match m15 {
                    Adt8::C0(_) => {
                        f8(v23, Str::lit(")"))?
                    }
                    Adt8::C1(_) => {
                        f8(v23, Str::lit(")"))?
                    }
                    Adt8::C2(v26, _) => {
                        f14(v23, v26)?
                    }
                }
            };
            if v15 {
                f8(v28, Str::lit("?"))
            } else {
                Ok(v28)
            }
        }
        Adt0::C4(v29, r65, v31, _) => {
            let v30 = (*r65).clone();
            let v33: Adt20 = f8(f2(f8(f7(v0, v29)?, Str::lit(".apply("))?, v30, v2, 100u64, 0u64)?, Str::lit(")"))?;
            if v31 {
                f8(v33, Str::lit("?"))
            } else {
                Ok(v33)
            }
        }
        Adt0::C5(v34, r66, _) => {
            let v35 = (*r66).clone();
            let v37: Adt20 = f15(v0, v34.clone())?;
            let a16 = f16(v34.clone());
            let a17 = 1u64;
            if nat_eq(a16, a17) {
                f8(f2(v37, v35, v2, 100u64, 0u64)?, Str::lit(")"))
            } else {
                let a18 = f16(v34);
                let a19 = 2u64;
                if nat_eq(a18, a19) {
                    if f17(v35.clone()) {
                        f8(f2(f8(v37, Str::lit("("))?, v35, v2, 100u64, 0u64)?, Str::lit(")"))
                    } else {
                        Ok(v37)
                    }
                } else {
                    Ok(v37)
                }
            }
        }
        Adt0::C6(r67, r68, _) => {
            let v38 = (*r67).clone();
            let v39 = (*r68).clone();
            f8(f2(f8(f2(f8(v0, Str::lit("("))?, v38, v2, 100u64, 0u64)?, Str::lit(", "))?, v39, v2, 100u64, 0u64)?, Str::lit(")"))
        }
        Adt0::C7(r69, r70, r71, _) => {
            let v41 = (*r69).clone();
            let v42 = (*r70).clone();
            let v43 = (*r71).clone();
            let v45: Adt20 = f2(f8(f2(f8(v0, Str::lit("if "))?, v41, v2, 100u64, 0u64)?, Str::lit(" "))?, v42, v2, 100u64, 0u64)?;
            if f18(v43.clone()) {
                Ok(v45)
            } else {
                f2(f8(v45, Str::lit(" else "))?, v43, v2, 100u64, 0u64)
            }
        }
        Adt0::C8(r72, r73, _) => {
            let v46 = (*r72).clone();
            let v47 = (*r73).clone();
            let a20 = f19(v47.clone());
            let a21 = 15u64;
            if nat_eq(a20, a21) {
                if f20(v46.clone()) {
                    let v49: Adt20 = f2(f8(f2(f8(v0, Str::lit("{ let value = "))?, v46, v2, 100u64, 0u64)?, Str::lit("; match value {\u{a}"))?, v47, v2, 100u64, 0u64)?;
                    f8(f21(v49, v2)?, Str::lit("} }"))
                } else {
                    let v50: Adt20 = f2(f8(f2(f8(v0, Str::lit("match "))?, v46, v2, 100u64, 0u64)?, Str::lit(" {\u{a}"))?, v47, v2, 100u64, 0u64)?;
                    f8(f21(v50, v2)?, Str::lit("}"))
                }
            } else {
                f8(f2(f8(v0, Str::lit("match "))?, v46, v2, 100u64, 0u64)?, Str::lit(" {}"))
            }
        }
        Adt0::C9(r74) => {
            let v51 = (*r74).clone();
            f2(v0, v51, v2, v3, v4)
        }
        Adt0::C10(v52, v53, _) => {
            let m22 = v52;
            match m22 {
                Adt16::C0 => {
                    f8(f7(v0, v53)?, Str::lit(" == 0"))
                }
                Adt16::C1 => {
                    f8(f7(v0, v53)?, Str::lit(" != 0"))
                }
                Adt16::C2 => {
                    f8(f7(v0, v53)?, Str::lit(".uncons()"))
                }
                Adt16::C3 => {
                    f8(f7(v0, v53)?, Str::lit(" - 1"))
                }
            }
        }
        Adt0::C11(r75, r76) => {
            let v55 = (*r75).clone();
            let v56 = (*r76).clone();
            let a23 = f19(v55.clone());
            let a24 = 12u64;
            if if nat_eq(a23, a24) {
                f22(v56.clone())
            } else {
                false
            } {
                f8(v0, Str::lit("{}"))
            } else {
                let v57: Adt20 = f2(f8(v0, Str::lit("{\u{a}"))?, v55, v2, 100u64, 0u64)?;
                let v58: Adt20 = if f22(v56.clone()) {
                    v57
                } else {
                    f8(f2(f21(v57, {
                        let a25 = v2;
                        let a26 = 1u64;
                        nat_add(a25, a26)?
                    })?, v56, {
                        let a27 = v2;
                        let a28 = 1u64;
                        nat_add(a27, a28)?
                    }, 100u64, 0u64)?, Str::lit("\u{a}"))?
                };
                f8(f21(v58, v2)?, Str::lit("}"))
            }
        }
        Adt0::C12 => {
            Ok(v0)
        }
        Adt0::C13(r77, r78) => {
            let v59 = (*r77).clone();
            let v60 = (*r78).clone();
            let a29 = v4;
            let a30 = f23(v3);
            if nat_lt(a29, a30) {
                let v61: Adt20 = f2(v0, v59, v2, 100u64, 0u64)?;
                if f17(v60.clone()) {
                    let a33 = {
                        let a31 = v4;
                        let a32 = 1u64;
                        nat_add(a31, a32)?
                    };
                    let a34 = f23(v3);
                    if nat_lt(a33, a34) {
                        f2(f24(v61, v3, v4)?, v60, v2, v3, {
                            let a35 = v4;
                            let a36 = 1u64;
                            nat_add(a35, a36)?
                        })
                    } else {
                        Ok(v61)
                    }
                } else {
                    Ok(v61)
                }
            } else {
                Ok(v0)
            }
        }
        Adt0::C14(v62, v63, v64, r79, _, r80) => {
            let v65 = (*r79).clone();
            let v67 = (*r80).clone();
            let v68: Adt20 = f25(f8(f21(v0, {
                let a37 = v2;
                let a38 = 1u64;
                nat_add(a37, a38)?
            })?, Str::lit("let "))?, v62)?;
            let v69: Adt20 = if v63 {
                f26(f8(v68, Str::lit(": "))?, v64)?
            } else {
                v68
            };
            let v70: Adt20 = f8(f2(f8(v69, Str::lit(" = "))?, v65, {
                let a39 = v2;
                let a40 = 1u64;
                nat_add(a39, a40)?
            }, 100u64, 0u64)?, Str::lit(";\u{a}"))?;
            f2(v70, v67, v2, 100u64, 0u64)
        }
        Adt0::C15(v71, r81, r82) => {
            let v72 = (*r81).clone();
            let v73 = (*r82).clone();
            let v74: Adt20 = f8(f2(f8(f25(f21(v0, {
                let a41 = v2;
                let a42 = 1u64;
                nat_add(a41, a42)?
            })?, v71)?, Str::lit(" => "))?, v72, {
                let a43 = v2;
                let a44 = 1u64;
                nat_add(a43, a44)?
            }, 100u64, 0u64)?, Str::lit("\u{a}"))?;
            f2(v74, v73, v2, 100u64, 0u64)
        }
        Adt0::C16(v75, r83) => {
            let v76 = (*r83).clone();
            let v77: Adt20 = f7(f8(f21(f8(v0, Str::lit("{\u{a}"))?, {
                let a45 = v2;
                let a46 = 1u64;
                nat_add(a45, a46)?
            })?, Str::lit("let mut "))?, v75.clone())?;
            let v78: Adt20 = f8(f7(f8(v77, Str::lit(" = "))?, v75.clone())?, Str::lit(";\u{a}"))?;
            let v79: Adt20 = f8(f21(v78, {
                let a47 = v2;
                let a48 = 1u64;
                nat_add(a47, a48)?
            })?, Str::lit("loop {\u{a}"))?;
            let v80: Adt20 = f8(f2(f8(f21(v79, {
                let a49 = v2;
                let a50 = 2u64;
                nat_add(a49, a50)?
            })?, Str::lit("let step = "))?, v76, {
                let a51 = v2;
                let a52 = 2u64;
                nat_add(a51, a52)?
            }, 100u64, 0u64)?, Str::lit(";\u{a}"))?;
            let v81: Adt20 = f8(f21(v80, {
                let a53 = v2;
                let a54 = 2u64;
                nat_add(a53, a54)?
            })?, Str::lit("match step {\u{a}"))?;
            let v82: Adt20 = f8(f7(f8(f21(v81, {
                let a55 = v2;
                let a56 = 3u64;
                nat_add(a55, a56)?
            })?, Str::lit("Ok(next) => { "))?, v75)?, Str::lit(" = next; }\u{a}"))?;
            let v83: Adt20 = f8(f21(v82, {
                let a57 = v2;
                let a58 = 3u64;
                nat_add(a57, a58)?
            })?, Str::lit("Err(done) => break done,\u{a}"))?;
            f8(f21(f8(f21(f8(f21(v83, {
                let a59 = v2;
                let a60 = 2u64;
                nat_add(a59, a60)?
            })?, Str::lit("}\u{a}"))?, {
                let a61 = v2;
                let a62 = 1u64;
                nat_add(a61, a62)?
            })?, Str::lit("}\u{a}"))?, v2)?, Str::lit("}"))
        }
    }
}

pub fn f3(v0: u64) -> Adt20 {
    Adt20::C0(List::<Bytes>::nil(), Bytes::lit(&[]), 0u64, v0, false)
}

pub fn f4(v0: List<Bytes>) -> Bytes {
    let m86 = {
        let (h84, _) = f27(Fn0::F28, {
            let a85 = v0.clone();
            length_list(a85)
        }, v0);
        h84
    };
    match m86.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f5(v0: List<Bytes>) -> List<Bytes> {
    f29(Fn1::F30, List::<Bytes>::nil(), v0)
}

pub fn f6(v0: Adt20, v1: Adt1) -> R<Adt20> {
    let m87 = v1;
    match m87 {
        Adt1::C0 => {
            f8(v0, Str::lit("()"))
        }
        Adt1::C1(v2) => {
            if v2 {
                f8(v0, Str::lit("true"))
            } else {
                f8(v0, Str::lit("false"))
            }
        }
        Adt1::C2(v3) => {
            f8(f11(v0, v3)?, Str::lit("u64"))
        }
        Adt1::C3(v4, v5) => {
            if if v4 {
                let a88 = v5;
                let a89 = 9223372036854775808u64;
                nat_eq(a88, a89)
            } else {
                false
            } {
                f8(v0, Str::lit("i64::MIN"))
            } else {
                f8(f11(if v4 {
                    f8(v0, Str::lit("-"))?
                } else {
                    v0
                }, v5)?, Str::lit("i64"))
            }
        }
        Adt1::C4(v6, v7, v8) => {
            if if v7 {
                let a90 = v8;
                let a91 = f31(v6.clone());
                nat_eq(a90, a91)
            } else {
                false
            } {
                f8(f32(v0, v6)?, Str::lit("::MIN"))
            } else {
                f32(f11(if v7 {
                    f8(v0, Str::lit("-"))?
                } else {
                    v0
                }, v8)?, v6)
            }
        }
        Adt1::C5(v9) => {
            f8(f33(f8(v0, Str::lit("Str::lit("))?, v9)?, Str::lit(")"))
        }
        Adt1::C6(v10) => {
            f34(v0, v10)
        }
        Adt1::C7(v11) => {
            f35(v0, v11)
        }
        Adt1::C8 => {
            f8(v0, Str::lit("Overflow"))
        }
    }
}

pub fn f7(v0: Adt20, v1: Adt6) -> R<Adt20> {
    let m92 = v1;
    match m92 {
        Adt6::C0(v2) => {
            f11(f8(v0, Str::lit("v"))?, v2)
        }
        Adt6::C1(v3) => {
            f11(f8(v0, Str::lit("m"))?, v3)
        }
        Adt6::C2(v4) => {
            f11(f8(v0, Str::lit("a"))?, v4)
        }
        Adt6::C3(v5) => {
            f11(f8(v0, Str::lit("c"))?, v5)
        }
        Adt6::C4(v6) => {
            f11(f8(v0, Str::lit("r"))?, v6)
        }
        Adt6::C5(v7) => {
            f11(f8(v0, Str::lit("h"))?, v7)
        }
        Adt6::C6(v8) => {
            f11(f8(v0, Str::lit("k"))?, v8)
        }
        Adt6::C7(v9) => {
            f11(f8(v0, Str::lit("p"))?, v9)
        }
        Adt6::C8(v10) => {
            f11(f8(v0, Str::lit("f"))?, v10)
        }
        Adt6::C9(v11) => {
            f36(v0, v11)
        }
    }
}

pub fn f8(v0: Adt20, v1: Str) -> R<Adt20> {
    f36(v0, {
        let a93 = v1;
        utf8_encode(a93)
    })
}

pub fn f9(v0: Adt0) -> u64 {
    let m94 = v0;
    match m94 {
        Adt0::C0(_, _) => {
            1000u64
        }
        Adt0::C1(_, _, _) => {
            1000u64
        }
        Adt0::C2(_, _, _) => {
            1000u64
        }
        Adt0::C3(_, _, _, _) => {
            1000u64
        }
        Adt0::C4(_, _, _, _) => {
            1000u64
        }
        Adt0::C5(_, _, _) => {
            1000u64
        }
        Adt0::C6(_, _, _) => {
            1000u64
        }
        Adt0::C7(_, _, _, _) => {
            1000u64
        }
        Adt0::C8(_, _, _) => {
            1000u64
        }
        Adt0::C9(_) => {
            1000u64
        }
        Adt0::C10(v31, _, _) => {
            let m95 = v31;
            match m95 {
                Adt16::C0 => {
                    0u64
                }
                Adt16::C1 => {
                    1u64
                }
                Adt16::C2 => {
                    2u64
                }
                Adt16::C3 => {
                    3u64
                }
            }
        }
        Adt0::C11(_, _) => {
            1000u64
        }
        Adt0::C12 => {
            1000u64
        }
        Adt0::C13(_, _) => {
            1000u64
        }
        Adt0::C14(_, _, _, _, _, _) => {
            1000u64
        }
        Adt0::C15(_, _, _) => {
            1000u64
        }
        Adt0::C16(_, _) => {
            1000u64
        }
    }
}

pub fn f10(v0: Adt8) -> u64 {
    let m96 = v0;
    match m96 {
        Adt8::C0(_) => {
            1000u64
        }
        Adt8::C1(_) => {
            1000u64
        }
        Adt8::C2(v3, _) => {
            v3
        }
    }
}

pub fn f11(v0: Adt20, v1: u64) -> R<Adt20> {
    f37(v0, v1)
}

pub fn f12(v0: Adt20, v1: Adt9) -> R<Adt20> {
    let v2: u64 = match v1.clone() {
        Adt9::C0(h97, _) => {
            h97
        }
    };
    let m99 = match v1 {
        Adt9::C0(_, h98) => {
            h98
        }
    };
    match m99 {
        None => {
            f8(v0, {
                let a106 = v2;
                let a107 = 20u64;
                if nat_lt(a106, a107) {
                    f39(v2)
                } else {
                    f40(v2)
                }
            })
        }
        Some(v3) => {
            let a100 = v2;
            let a101 = 24u64;
            if nat_eq(a100, a101) {
                f32(f8(v0, Str::lit("checked_neg_"))?, v3)
            } else {
                let a102 = v2;
                let a103 = 47u64;
                if nat_eq(a102, a103) {
                    f32(f8(v0, Str::lit("format_"))?, v3)
                } else {
                    let a104 = v2;
                    let a105 = 49u64;
                    if nat_eq(a104, a105) {
                        f32(f8(v0, Str::lit("parse_"))?, v3)
                    } else {
                        f8(f8(f32(f8(v0, Str::lit("fixed_"))?, v3)?, Str::lit("::"))?, f38(v2))
                    }
                }
            }
        }
    }
}

pub fn f13(v0: Adt20, v1: u64, v2: Option<Adt2>) -> R<Adt20> {
    let a108 = v1;
    let a109 = 8u64;
    if nat_lt(a108, a109) {
        let m110 = v2;
        match m110 {
            None => {
                Ok(v0)
            }
            Some(v3) => {
                f8(f8(f8(f32(v0, v3)?, Str::lit("::"))?, {
                    let a111 = v1;
                    let a112 = 7u64;
                    if nat_eq(a111, a112) {
                        Str::lit("try_from")
                    } else {
                        f41(v1)
                    }
                })?, Str::lit("("))
            }
        }
    } else {
        let a113 = v1;
        let a114 = 40u64;
        if nat_eq(a113, a114) {
            let m115 = v2;
            match m115 {
                None => {
                    Ok(v0)
                }
                Some(v4) => {
                    f8(f32(f8(v0, Str::lit("Str::format_"))?, v4)?, Str::lit("("))
                }
            }
        } else {
            let a116 = v1;
            let a117 = 41u64;
            if nat_eq(a116, a117) {
                let m118 = v2;
                match m118 {
                    None => {
                        Ok(v0)
                    }
                    Some(v5) => {
                        f8(f32(f8(v0, Str::lit("Str::parse_"))?, v5)?, Str::lit("(&"))
                    }
                }
            } else {
                f8(v0, f42(v1))
            }
        }
    }
}

pub fn f14(v0: Adt20, v1: u64) -> R<Adt20> {
    let a119 = v1;
    let a120 = 8u64;
    if nat_lt(a119, a120) {
        f8(v0, {
            let a121 = v1;
            let a122 = 7u64;
            if nat_eq(a121, a122) {
                Str::lit(").ok()")
            } else {
                Str::lit(")")
            }
        })
    } else {
        f8(v0, f43(v1))
    }
}

pub fn f15(v0: Adt20, v1: Adt10) -> R<Adt20> {
    let m123 = v1;
    match m123 {
        Adt10::C0(v2) => {
            f8(f26(f8(v0, Str::lit("None::<"))?, v2)?, Str::lit(">"))
        }
        Adt10::C1 => {
            f8(v0, Str::lit("Some("))
        }
        Adt10::C2(v3, v4) => {
            f8(f26(f8(f26(f8(v0, Str::lit("Ok::<"))?, v3)?, Str::lit(", "))?, v4)?, Str::lit(">("))
        }
        Adt10::C3(v5, v6) => {
            f8(f26(f8(f26(f8(v0, Str::lit("Err::<"))?, v5)?, Str::lit(", "))?, v6)?, Str::lit(">("))
        }
        Adt10::C4(v7, v8) => {
            f11(f8(f11(f8(v0, Str::lit("Adt"))?, v7)?, Str::lit("::C"))?, v8)
        }
        Adt10::C5(v9, v10) => {
            f11(f8(f11(f8(v0, Str::lit("Fn"))?, v9)?, Str::lit("::F"))?, v10)
        }
        Adt10::C6 => {
            f8(v0, Str::lit("List::cons("))
        }
        Adt10::C7(v11) => {
            f8(f26(f8(v0, Str::lit("List::<"))?, v11)?, Str::lit(">::nil()"))
        }
    }
}

pub fn f16(v0: Adt10) -> u64 {
    let m124 = v0;
    match m124 {
        Adt10::C0(_) => {
            0u64
        }
        Adt10::C1 => {
            1u64
        }
        Adt10::C2(_, _) => {
            1u64
        }
        Adt10::C3(_, _) => {
            1u64
        }
        Adt10::C4(_, _) => {
            2u64
        }
        Adt10::C5(_, _) => {
            2u64
        }
        Adt10::C6 => {
            1u64
        }
        Adt10::C7(_) => {
            0u64
        }
    }
}

pub fn f17(v0: Adt0) -> bool {
    let a125 = f19(v0);
    let a126 = 13u64;
    nat_eq(a125, a126)
}

pub fn f18(v0: Adt0) -> bool {
    let m127 = v0;
    match m127 {
        Adt0::C0(_, _) => {
            false
        }
        Adt0::C1(_, _, _) => {
            false
        }
        Adt0::C2(_, _, _) => {
            false
        }
        Adt0::C3(_, _, _, _) => {
            false
        }
        Adt0::C4(_, _, _, _) => {
            false
        }
        Adt0::C5(_, _, _) => {
            false
        }
        Adt0::C6(_, _, _) => {
            false
        }
        Adt0::C7(_, _, _, _) => {
            false
        }
        Adt0::C8(_, _, _) => {
            false
        }
        Adt0::C9(_) => {
            false
        }
        Adt0::C10(_, _, _) => {
            false
        }
        Adt0::C11(r130, r131) => {
            let v34 = (*r130).clone();
            let v35 = (*r131).clone();
            let a128 = f19(v34);
            let a129 = 12u64;
            if nat_eq(a128, a129) {
                f22(v35)
            } else {
                false
            }
        }
        Adt0::C12 => {
            false
        }
        Adt0::C13(_, _) => {
            false
        }
        Adt0::C14(_, _, _, _, _, _) => {
            false
        }
        Adt0::C15(_, _, _) => {
            false
        }
        Adt0::C16(_, _) => {
            false
        }
    }
}

pub fn f19(v0: Adt0) -> u64 {
    let m132 = v0;
    match m132 {
        Adt0::C0(_, _) => {
            0u64
        }
        Adt0::C1(_, _, _) => {
            1u64
        }
        Adt0::C2(_, _, _) => {
            2u64
        }
        Adt0::C3(_, _, _, _) => {
            3u64
        }
        Adt0::C4(_, _, _, _) => {
            4u64
        }
        Adt0::C5(_, _, _) => {
            5u64
        }
        Adt0::C6(_, _, _) => {
            6u64
        }
        Adt0::C7(_, _, _, _) => {
            7u64
        }
        Adt0::C8(_, _, _) => {
            8u64
        }
        Adt0::C9(_) => {
            9u64
        }
        Adt0::C10(_, _, _) => {
            10u64
        }
        Adt0::C11(_, _) => {
            11u64
        }
        Adt0::C12 => {
            12u64
        }
        Adt0::C13(_, _) => {
            13u64
        }
        Adt0::C14(_, _, _, _, _, _) => {
            14u64
        }
        Adt0::C15(_, _, _) => {
            15u64
        }
        Adt0::C16(_, _) => {
            16u64
        }
    }
}

pub fn f20(p143: Adt0) -> bool {
    let m142 = p143;
    {
        let mut m142 = m142;
        loop {
            let step = {
                let v0 = m142;
                let m133 = v0;
                match m133 {
                    Adt0::C0(_, _) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C1(_, _, _) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C2(_, _, _) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C3(v9, _, _, _) => {
                        let a134 = f10(v9.clone());
                        let a135 = 16u64;
                        if nat_eq(a134, a135) {
                            Err::<Adt0, bool>(true)
                        } else {
                            let a136 = f10(v9);
                            let a137 = 25u64;
                            Err::<Adt0, bool>(nat_eq(a136, a137))
                        }
                    }
                    Adt0::C4(_, _, _, _) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C5(_, _, _) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C6(_, _, _) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C7(_, _, _, _) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C8(r140, r141, _) => {
                        let v27 = (*r140).clone();
                        let v28 = (*r141).clone();
                        let a138 = f19(v28);
                        let a139 = 15u64;
                        if nat_eq(a138, a139) {
                            Ok::<Adt0, bool>(v27)
                        } else {
                            Err::<Adt0, bool>(false)
                        }
                    }
                    Adt0::C9(_) => {
                        Err::<Adt0, bool>(true)
                    }
                    Adt0::C10(_, _, _) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C11(_, _) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C12 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C13(_, _) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C14(_, _, _, _, _, _) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C15(_, _, _) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C16(_, _) => {
                        Err::<Adt0, bool>(true)
                    }
                }
            };
            match step {
                Ok(next) => { m142 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f21(v0: Adt20, v1: u64) -> R<Adt20> {
    let (_, h144) = {
        let (h145, _) = f44(Fn2::F45(v1), v1, (0u64, v0))?;
        h145
    };
    Ok(h144)
}

pub fn f22(v0: Adt0) -> bool {
    let m146 = v0;
    match m146 {
        Adt0::C0(v1, _) => {
            let m147 = v1;
            match m147 {
                Adt1::C0 => {
                    true
                }
                Adt1::C1(_) => {
                    false
                }
                Adt1::C2(_) => {
                    false
                }
                Adt1::C3(_, _) => {
                    false
                }
                Adt1::C4(_, _, _) => {
                    false
                }
                Adt1::C5(_) => {
                    false
                }
                Adt1::C6(_) => {
                    false
                }
                Adt1::C7(_) => {
                    false
                }
                Adt1::C8 => {
                    false
                }
            }
        }
        Adt0::C1(_, _, _) => {
            false
        }
        Adt0::C2(_, _, _) => {
            false
        }
        Adt0::C3(_, _, _, _) => {
            false
        }
        Adt0::C4(_, _, _, _) => {
            false
        }
        Adt0::C5(_, _, _) => {
            false
        }
        Adt0::C6(_, _, _) => {
            false
        }
        Adt0::C7(_, _, _, _) => {
            false
        }
        Adt0::C8(_, _, _) => {
            false
        }
        Adt0::C9(_) => {
            false
        }
        Adt0::C10(_, _, _) => {
            false
        }
        Adt0::C11(_, _) => {
            false
        }
        Adt0::C12 => {
            false
        }
        Adt0::C13(_, _) => {
            false
        }
        Adt0::C14(_, _, _, _, _, _) => {
            false
        }
        Adt0::C15(_, _, _) => {
            false
        }
        Adt0::C16(_, _) => {
            false
        }
    }
}

pub fn f23(v0: u64) -> u64 {
    let a148 = v0;
    let a149 = 8u64;
    if if nat_lt(a148, a149) {
        true
    } else {
        let a150 = 44u64;
        let a151 = v0;
        nat_le(a150, a151)
    } {
        100000u64
    } else {
        let a152 = v0;
        let a153 = 11u64;
        if nat_eq(a152, a153) {
            1u64
        } else {
            let a154 = v0;
            let a155 = 24u64;
            if if nat_eq(a154, a155) {
                true
            } else {
                let a156 = v0;
                let a157 = 26u64;
                if nat_eq(a156, a157) {
                    true
                } else {
                    let a158 = v0;
                    let a159 = 31u64;
                    if nat_eq(a158, a159) {
                        true
                    } else {
                        let a160 = v0;
                        let a161 = 32u64;
                        if nat_eq(a160, a161) {
                            true
                        } else {
                            let a162 = v0;
                            let a163 = 35u64;
                            if nat_eq(a162, a163) {
                                true
                            } else {
                                let a164 = v0;
                                let a165 = 36u64;
                                if nat_eq(a164, a165) {
                                    true
                                } else {
                                    let a166 = v0;
                                    let a167 = 40u64;
                                    if nat_eq(a166, a167) {
                                        true
                                    } else {
                                        let a168 = v0;
                                        let a169 = 41u64;
                                        if nat_eq(a168, a169) {
                                            true
                                        } else {
                                            let a170 = v0;
                                            let a171 = 42u64;
                                            if nat_eq(a170, a171) {
                                                true
                                            } else {
                                                let a172 = v0;
                                                let a173 = 43u64;
                                                nat_eq(a172, a173)
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            } {
                1u64
            } else {
                let a174 = v0;
                let a175 = 15u64;
                if nat_eq(a174, a175) {
                    3u64
                } else {
                    let a176 = v0;
                    let a177 = 16u64;
                    if nat_eq(a176, a177) {
                        3u64
                    } else {
                        let a178 = v0;
                        let a179 = 20u64;
                        if nat_eq(a178, a179) {
                            3u64
                        } else {
                            let a180 = v0;
                            let a181 = 25u64;
                            if if nat_eq(a180, a181) {
                                true
                            } else {
                                let a182 = v0;
                                let a183 = 29u64;
                                if nat_eq(a182, a183) {
                                    true
                                } else {
                                    let a184 = v0;
                                    let a185 = 34u64;
                                    if nat_eq(a184, a185) {
                                        true
                                    } else {
                                        let a186 = v0;
                                        let a187 = 38u64;
                                        nat_eq(a186, a187)
                                    }
                                }
                            } {
                                3u64
                            } else {
                                2u64
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f24(v0: Adt20, v1: u64, v2: u64) -> R<Adt20> {
    let a188 = v1;
    let a189 = 8u64;
    if nat_lt(a188, a189) {
        f8(v0, Str::lit(", "))
    } else {
        let a190 = v1;
        let a191 = 100u64;
        if nat_eq(a190, a191) {
            f8(v0, Str::lit(", "))
        } else {
            let a192 = v2;
            let a193 = 0u64;
            if nat_eq(a192, a193) {
                f8(v0, f46(v1))
            } else {
                f8(v0, f47(v1))
            }
        }
    }
}

pub fn f25(v0: Adt20, v1: Adt17) -> R<Adt20> {
    let m194 = v1;
    match m194 {
        Adt17::C0(v2) => {
            f8(v0, {
                let m195 = v2;
                match m195 {
                    Adt18::C0 => {
                        Str::lit("_")
                    }
                    Adt18::C1 => {
                        Str::lit("()")
                    }
                    Adt18::C2 => {
                        Str::lit("None")
                    }
                }
            })
        }
        Adt17::C1(v3) => {
            f7(v0, v3)
        }
        Adt17::C2(r199) => {
            let v4 = (*r199).clone();
            f8(f25(f8(v0, Str::lit("("))?, v4)?, Str::lit(")"))
        }
        Adt17::C3(v5, r200) => {
            let v6 = (*r200).clone();
            f8(f25(f8(v0, {
                let m196 = v5;
                match m196 {
                    Adt19::C0 => {
                        Str::lit("Some(")
                    }
                    Adt19::C1 => {
                        Str::lit("Ok(")
                    }
                    Adt19::C2 => {
                        Str::lit("Err(")
                    }
                }
            })?, v6)?, Str::lit(")"))
        }
        Adt17::C4(v7, v8, r201) => {
            let v9 = (*r201).clone();
            let v10: Adt20 = f11(f8(f11(f8(v0, Str::lit("Adt"))?, v7)?, Str::lit("::C"))?, v8)?;
            let m197 = v9.clone();
            match m197 {
                Adt17::C0(_) => {
                    Ok(v10)
                }
                Adt17::C1(_) => {
                    Ok(v10)
                }
                Adt17::C2(_) => {
                    Ok(v10)
                }
                Adt17::C3(_, _) => {
                    Ok(v10)
                }
                Adt17::C4(_, _, _) => {
                    Ok(v10)
                }
                Adt17::C5 => {
                    Ok(v10)
                }
                Adt17::C6(_, _) => {
                    f8(f25(f8(v10, Str::lit("("))?, v9)?, Str::lit(")"))
                }
                Adt17::C7(_) => {
                    Ok(v10)
                }
            }
        }
        Adt17::C5 => {
            Ok(v0)
        }
        Adt17::C6(r202, r203) => {
            let v22 = (*r202).clone();
            let v23 = (*r203).clone();
            let m198 = v23.clone();
            match m198 {
                Adt17::C0(_) => {
                    f25(v0, v22)
                }
                Adt17::C1(_) => {
                    f25(v0, v22)
                }
                Adt17::C2(_) => {
                    f25(v0, v22)
                }
                Adt17::C3(_, _) => {
                    f25(v0, v22)
                }
                Adt17::C4(_, _, _) => {
                    f25(v0, v22)
                }
                Adt17::C5 => {
                    f25(v0, v22)
                }
                Adt17::C6(_, _) => {
                    f25(f8(f25(v0, v22)?, Str::lit(", "))?, v23)
                }
                Adt17::C7(_) => {
                    f25(v0, v22)
                }
            }
        }
        Adt17::C7(v35) => {
            f35(v0, v35)
        }
    }
}

pub fn f26(v0: Adt20, v1: Adt11) -> R<Adt20> {
    let m204 = v1;
    match m204 {
        Adt11::C0(v2) => {
            f48(v0, v2)
        }
        Adt11::C1(v3) => {
            f32(v0, v3)
        }
        Adt11::C2(v4, r208) => {
            let v5 = (*r208).clone();
            f8(f26(f8(v0, f49(v4.clone()))?, v5)?, f50(v4))
        }
        Adt11::C3(v6, r209, r210) => {
            let v7 = (*r209).clone();
            let v8 = (*r210).clone();
            let m205 = v6;
            match m205 {
                Adt14::C0 => {
                    f8(f26(f8(f26(f8(v0, Str::lit("Result<"))?, v7)?, Str::lit(", "))?, v8)?, Str::lit(">"))
                }
                Adt14::C1 => {
                    f8(f26(f8(f26(f8(v0, Str::lit("("))?, v7)?, Str::lit(", "))?, v8)?, Str::lit(")"))
                }
            }
        }
        Adt11::C4(v9, v10) => {
            let m206 = v9;
            match m206 {
                Adt15::C0 => {
                    f11(f8(v0, Str::lit("Adt"))?, v10)
                }
                Adt15::C1 => {
                    f11(f8(v0, Str::lit("Fn"))?, v10)
                }
                Adt15::C2 => {
                    f51(v0, v10)
                }
            }
        }
        Adt11::C5 => {
            Ok(v0)
        }
        Adt11::C6(r211, r212) => {
            let v11 = (*r211).clone();
            let v12 = (*r212).clone();
            let m207 = v12.clone();
            match m207 {
                Adt11::C0(_) => {
                    f26(v0, v11)
                }
                Adt11::C1(_) => {
                    f26(v0, v11)
                }
                Adt11::C2(_, _) => {
                    f26(v0, v11)
                }
                Adt11::C3(_, _, _) => {
                    f26(v0, v11)
                }
                Adt11::C4(_, _) => {
                    f26(v0, v11)
                }
                Adt11::C5 => {
                    f26(v0, v11)
                }
                Adt11::C6(_, _) => {
                    f26(f8(f26(v0, v11)?, Str::lit(", "))?, v12)
                }
            }
        }
    }
}

pub fn f27(v0: Fn0, p217: u64, p218: List<Bytes>) -> (List<Bytes>, bool) {
    let m216 = (p217, p218);
    {
        let mut m216 = m216;
        loop {
            let step = {
                let (v1, v2) = m216;
                let m213 = v1;
                if m213 == 0 {
                    Err::<(u64, List<Bytes>), (List<Bytes>, bool)>((v2, false))
                } else {
                    let v3 = m213 - 1;
                    let m215 = {
                        let c214: Fn0 = v0.clone();
                        c214.apply(v2.clone())
                    };
                    match m215 {
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
                Ok(next) => { m216 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f28(v0: List<Bytes>) -> Option<List<Bytes>> {
    f52(v0)
}

pub fn f29(v0: Fn1, p222: List<Bytes>, p223: List<Bytes>) -> List<Bytes> {
    let m221 = (p222, p223);
    {
        let mut m221 = m221;
        loop {
            let step = {
                let (v1, v2) = m221;
                let m219 = v2;
                match m219.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c220: Fn1 = v0.clone();
                            c220.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m221 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f30(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f31(v0: Adt2) -> u64 {
    let m224 = v0;
    match m224 {
        Adt2::C0 => {
            0u64
        }
        Adt2::C1 => {
            0u64
        }
        Adt2::C2 => {
            0u64
        }
        Adt2::C3 => {
            0u64
        }
        Adt2::C4 => {
            128u64
        }
        Adt2::C5 => {
            32768u64
        }
        Adt2::C6 => {
            2147483648u64
        }
        Adt2::C7 => {
            9223372036854775808u64
        }
    }
}

pub fn f32(v0: Adt20, v1: Adt2) -> R<Adt20> {
    f8(v0, f53(v1))
}

pub fn f33(v0: Adt20, v1: Bytes) -> R<Adt20> {
    f8({
        let (_, h225) = {
            let (h226, _) = f44(Fn2::F54(v1.clone()), {
                let a227 = v1;
                length_bytes(a227)
            }, (0u64, f8(v0, Str::lit("\""))?))?;
            h226
        };
        h225
    }, Str::lit("\""))
}

pub fn f34(v0: Adt20, v1: Bytes) -> R<Adt20> {
    f8({
        let (_, h228) = {
            let (h229, _) = f44(Fn2::F55(v1.clone()), {
                let a230 = v1;
                length_bytes(a230)
            }, (0u64, f8(v0, Str::lit("Bytes::lit(&["))?))?;
            h229
        };
        h228
    }, Str::lit("])"))
}

pub fn f35(v0: Adt20, v1: Adt3) -> R<Adt20> {
    f8(v0, {
        let m231 = v1;
        match m231 {
            Adt3::C0 => {
                Str::lit("Ordering::Less")
            }
            Adt3::C1 => {
                Str::lit("Ordering::Equal")
            }
            Adt3::C2 => {
                Str::lit("Ordering::Greater")
            }
        }
    })
}

pub fn f36(v0: Adt20, v1: Bytes) -> R<Adt20> {
    if match v0.clone() {
        Adt20::C0(_, _, _, _, h232) => {
            h232
        }
    } {
        Ok(v0)
    } else {
        let v2: u64 = {
            let a235 = match v0.clone() {
                Adt20::C0(_, _, h233, _, _) => {
                    h233
                }
            };
            let a236 = {
                let a234 = v1.clone();
                length_bytes(a234)
            };
            nat_add(a235, a236)?
        };
        let a238 = v2;
        let a239 = match v0.clone() {
            Adt20::C0(_, _, _, h237, _) => {
                h237
            }
        };
        if nat_le(a238, a239) {
            Ok(f56(v0.clone(), {
                let a241 = match v0 {
                    Adt20::C0(_, h240, _, _, _) => {
                        h240
                    }
                };
                let a242 = v1;
                append_bytes(a241, a242)
            }, v2))
        } else {
            Ok(f57(v0))
        }
    }
}

pub fn f37(v0: Adt20, v1: u64) -> R<Adt20> {
    f36(v0, f58(v1)?)
}

pub fn f38(v0: u64) -> Str {
    let a243 = v0;
    let a244 = 20u64;
    if nat_eq(a243, a244) {
        Str::lit("checked_add")
    } else {
        let a245 = v0;
        let a246 = 21u64;
        if nat_eq(a245, a246) {
            Str::lit("checked_sub")
        } else {
            let a247 = v0;
            let a248 = 22u64;
            if nat_eq(a247, a248) {
                Str::lit("checked_mul")
            } else {
                let a249 = v0;
                let a250 = 23u64;
                if nat_eq(a249, a250) {
                    Str::lit("checked_quot")
                } else {
                    let a251 = v0;
                    let a252 = 25u64;
                    if nat_eq(a251, a252) {
                        Str::lit("bit_and")
                    } else {
                        let a253 = v0;
                        let a254 = 26u64;
                        if nat_eq(a253, a254) {
                            Str::lit("bit_or")
                        } else {
                            let a255 = v0;
                            let a256 = 27u64;
                            if nat_eq(a255, a256) {
                                Str::lit("bit_xor")
                            } else {
                                let a257 = v0;
                                let a258 = 28u64;
                                if nat_eq(a257, a258) {
                                    Str::lit("bit_not")
                                } else {
                                    let a259 = v0;
                                    let a260 = 29u64;
                                    if nat_eq(a259, a260) {
                                        Str::lit("shift_left")
                                    } else {
                                        let a261 = v0;
                                        let a262 = 30u64;
                                        if nat_eq(a261, a262) {
                                            Str::lit("shift_right")
                                        } else {
                                            let a263 = v0;
                                            let a264 = 31u64;
                                            if nat_eq(a263, a264) {
                                                Str::lit("convert")
                                            } else {
                                                Str::lit("")
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

pub fn f39(v0: u64) -> Str {
    let a265 = v0;
    let a266 = 0u64;
    if nat_eq(a265, a266) {
        Str::lit("nat_add")
    } else {
        let a267 = v0;
        let a268 = 1u64;
        if nat_eq(a267, a268) {
            Str::lit("nat_sub")
        } else {
            let a269 = v0;
            let a270 = 2u64;
            if nat_eq(a269, a270) {
                Str::lit("nat_mul")
            } else {
                let a271 = v0;
                let a272 = 3u64;
                if nat_eq(a271, a272) {
                    Str::lit("nat_quot")
                } else {
                    let a273 = v0;
                    let a274 = 4u64;
                    if nat_eq(a273, a274) {
                        Str::lit("nat_rem")
                    } else {
                        let a275 = v0;
                        let a276 = 5u64;
                        if nat_eq(a275, a276) {
                            Str::lit("nat_eq")
                        } else {
                            let a277 = v0;
                            let a278 = 6u64;
                            if nat_eq(a277, a278) {
                                Str::lit("nat_le")
                            } else {
                                let a279 = v0;
                                let a280 = 7u64;
                                if nat_eq(a279, a280) {
                                    Str::lit("nat_lt")
                                } else {
                                    let a281 = v0;
                                    let a282 = 8u64;
                                    if nat_eq(a281, a282) {
                                        Str::lit("nat_succ")
                                    } else {
                                        let a283 = v0;
                                        let a284 = 9u64;
                                        if nat_eq(a283, a284) {
                                            Str::lit("int_add")
                                        } else {
                                            let a285 = v0;
                                            let a286 = 10u64;
                                            if nat_eq(a285, a286) {
                                                Str::lit("int_sub")
                                            } else {
                                                let a287 = v0;
                                                let a288 = 11u64;
                                                if nat_eq(a287, a288) {
                                                    Str::lit("int_mul")
                                                } else {
                                                    let a289 = v0;
                                                    let a290 = 12u64;
                                                    if nat_eq(a289, a290) {
                                                        Str::lit("int_neg")
                                                    } else {
                                                        let a291 = v0;
                                                        let a292 = 13u64;
                                                        if nat_eq(a291, a292) {
                                                            Str::lit("int_quot")
                                                        } else {
                                                            let a293 = v0;
                                                            let a294 = 14u64;
                                                            if nat_eq(a293, a294) {
                                                                Str::lit("int_rem")
                                                            } else {
                                                                let a295 = v0;
                                                                let a296 = 15u64;
                                                                if nat_eq(a295, a296) {
                                                                    Str::lit("bool_not")
                                                                } else {
                                                                    let a297 = v0;
                                                                    let a298 = 16u64;
                                                                    if nat_eq(a297, a298) {
                                                                        Str::lit("bool_and")
                                                                    } else {
                                                                        let a299 = v0;
                                                                        let a300 = 17u64;
                                                                        if nat_eq(a299, a300) {
                                                                            Str::lit("bool_or")
                                                                        } else {
                                                                            let a301 = v0;
                                                                            let a302 = 18u64;
                                                                            if nat_eq(a301, a302) {
                                                                                Str::lit("equal")
                                                                            } else {
                                                                                let a303 = v0;
                                                                                let a304 = 19u64;
                                                                                if nat_eq(a303, a304) {
                                                                                    Str::lit("compare")
                                                                                } else {
                                                                                    Str::lit("")
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
                    }
                }
            }
        }
    }
}

pub fn f40(v0: u64) -> Str {
    let a305 = v0;
    let a306 = 32u64;
    if nat_eq(a305, a306) {
        Str::lit("append_list")
    } else {
        let a307 = v0;
        let a308 = 33u64;
        if nat_eq(a307, a308) {
            Str::lit("append_bytes")
        } else {
            let a309 = v0;
            let a310 = 34u64;
            if nat_eq(a309, a310) {
                Str::lit("length_list")
            } else {
                let a311 = v0;
                let a312 = 35u64;
                if nat_eq(a311, a312) {
                    Str::lit("length_bytes")
                } else {
                    let a313 = v0;
                    let a314 = 36u64;
                    if nat_eq(a313, a314) {
                        Str::lit("length_string")
                    } else {
                        let a315 = v0;
                        let a316 = 37u64;
                        if nat_eq(a315, a316) {
                            Str::lit("index_list")
                        } else {
                            let a317 = v0;
                            let a318 = 38u64;
                            if nat_eq(a317, a318) {
                                Str::lit("index_bytes")
                            } else {
                                let a319 = v0;
                                let a320 = 39u64;
                                if nat_eq(a319, a320) {
                                    Str::lit("slice_list")
                                } else {
                                    let a321 = v0;
                                    let a322 = 40u64;
                                    if nat_eq(a321, a322) {
                                        Str::lit("slice_bytes")
                                    } else {
                                        let a323 = v0;
                                        let a324 = 41u64;
                                        if nat_eq(a323, a324) {
                                            Str::lit("utf8_encode")
                                        } else {
                                            let a325 = v0;
                                            let a326 = 42u64;
                                            if nat_eq(a325, a326) {
                                                Str::lit("utf8_decode")
                                            } else {
                                                let a327 = v0;
                                                let a328 = 43u64;
                                                if nat_eq(a327, a328) {
                                                    Str::lit("compare_bytes")
                                                } else {
                                                    let a329 = v0;
                                                    let a330 = 44u64;
                                                    if nat_eq(a329, a330) {
                                                        Str::lit("split_exact")
                                                    } else {
                                                        let a331 = v0;
                                                        let a332 = 45u64;
                                                        if nat_eq(a331, a332) {
                                                            Str::lit("join")
                                                        } else {
                                                            let a333 = v0;
                                                            let a334 = 46u64;
                                                            if nat_eq(a333, a334) {
                                                                Str::lit("format_int")
                                                            } else {
                                                                let a335 = v0;
                                                                let a336 = 48u64;
                                                                if nat_eq(a335, a336) {
                                                                    Str::lit("parse_int")
                                                                } else {
                                                                    Str::lit("")
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
    }
}

pub fn f41(v0: u64) -> Str {
    let a337 = v0;
    let a338 = 0u64;
    if nat_eq(a337, a338) {
        Str::lit("checked_add")
    } else {
        let a339 = v0;
        let a340 = 1u64;
        if nat_eq(a339, a340) {
            Str::lit("checked_sub")
        } else {
            let a341 = v0;
            let a342 = 2u64;
            if nat_eq(a341, a342) {
                Str::lit("checked_mul")
            } else {
                let a343 = v0;
                let a344 = 3u64;
                if nat_eq(a343, a344) {
                    Str::lit("checked_div")
                } else {
                    let a345 = v0;
                    let a346 = 4u64;
                    if nat_eq(a345, a346) {
                        Str::lit("checked_neg")
                    } else {
                        let a347 = v0;
                        let a348 = 5u64;
                        if nat_eq(a347, a348) {
                            Str::lit("checked_shl")
                        } else {
                            let a349 = v0;
                            let a350 = 6u64;
                            if nat_eq(a349, a350) {
                                Str::lit("checked_shr")
                            } else {
                                Str::lit("")
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f42(v0: u64) -> Str {
    let a351 = v0;
    let a352 = 42u64;
    if nat_eq(a351, a352) {
        Str::lit("Str::parse_int(&")
    } else {
        let a353 = v0;
        let a354 = 43u64;
        if nat_eq(a353, a354) {
            Str::lit("Str::format_i64(")
        } else {
            let a355 = v0;
            let a356 = 30u64;
            if nat_eq(a355, a356) {
                Str::lit("Bytes::append(")
            } else {
                let a357 = v0;
                let a358 = 31u64;
                if nat_eq(a357, a358) {
                    Str::lit("Bytes::length(&")
                } else {
                    let a359 = v0;
                    let a360 = 32u64;
                    if nat_eq(a359, a360) {
                        Str::lit("Str::length(&")
                    } else {
                        let a361 = v0;
                        let a362 = 33u64;
                        if nat_eq(a361, a362) {
                            Str::lit("Bytes::index(&")
                        } else {
                            let a363 = v0;
                            let a364 = 34u64;
                            if nat_eq(a363, a364) {
                                Str::lit("Bytes::slice(&")
                            } else {
                                let a365 = v0;
                                let a366 = 35u64;
                                if nat_eq(a365, a366) {
                                    Str::lit("Str::utf8_encode(&")
                                } else {
                                    let a367 = v0;
                                    let a368 = 36u64;
                                    if nat_eq(a367, a368) {
                                        Str::lit("Bytes::utf8_decode(&")
                                    } else {
                                        let a369 = v0;
                                        let a370 = 37u64;
                                        if nat_eq(a369, a370) {
                                            Str::lit("Bytes::compare(&")
                                        } else {
                                            let a371 = v0;
                                            let a372 = 38u64;
                                            if nat_eq(a371, a372) {
                                                Str::lit("Str::split_exact(&")
                                            } else {
                                                let a373 = v0;
                                                let a374 = 39u64;
                                                if nat_eq(a373, a374) {
                                                    Str::lit("Str::join(")
                                                } else {
                                                    let a375 = v0;
                                                    let a376 = 8u64;
                                                    if nat_eq(a375, a376) {
                                                        Str::lit("core::ops::BitAnd::bitand(")
                                                    } else {
                                                        let a377 = v0;
                                                        let a378 = 9u64;
                                                        if nat_eq(a377, a378) {
                                                            Str::lit("core::ops::BitOr::bitor(")
                                                        } else {
                                                            let a379 = v0;
                                                            let a380 = 10u64;
                                                            if nat_eq(a379, a380) {
                                                                Str::lit("core::ops::BitXor::bitxor(")
                                                            } else {
                                                                let a381 = v0;
                                                                let a382 = 11u64;
                                                                if nat_eq(a381, a382) {
                                                                    Str::lit("core::ops::Not::not(")
                                                                } else {
                                                                    let a383 = v0;
                                                                    let a384 = 12u64;
                                                                    if nat_eq(a383, a384) {
                                                                        Str::lit("core::cmp::PartialEq::eq(&")
                                                                    } else {
                                                                        let a385 = v0;
                                                                        let a386 = 13u64;
                                                                        if nat_eq(a385, a386) {
                                                                            Str::lit("core::cmp::PartialOrd::le(&")
                                                                        } else {
                                                                            let a387 = v0;
                                                                            let a388 = 14u64;
                                                                            if nat_eq(a387, a388) {
                                                                                Str::lit("core::cmp::PartialOrd::lt(&")
                                                                            } else {
                                                                                let a389 = v0;
                                                                                let a390 = 15u64;
                                                                                if nat_eq(a389, a390) {
                                                                                    Str::lit("u64::checked_rem(")
                                                                                } else {
                                                                                    let a391 = v0;
                                                                                    let a392 = 16u64;
                                                                                    if nat_eq(a391, a392) {
                                                                                        Str::lit("{ let (a, b, z): (i64, i64, i64) = (")
                                                                                    } else {
                                                                                        let a393 = v0;
                                                                                        let a394 = 17u64;
                                                                                        if nat_eq(a393, a394) {
                                                                                            Str::lit("u64::checked_add(")
                                                                                        } else {
                                                                                            let a395 = v0;
                                                                                            let a396 = 18u64;
                                                                                            if nat_eq(a395, a396) {
                                                                                                Str::lit("u64::saturating_sub(")
                                                                                            } else {
                                                                                                let a397 = v0;
                                                                                                let a398 = 19u64;
                                                                                                if nat_eq(a397, a398) {
                                                                                                    Str::lit("u64::checked_mul(")
                                                                                                } else {
                                                                                                    let a399 = v0;
                                                                                                    let a400 = 20u64;
                                                                                                    if nat_eq(a399, a400) {
                                                                                                        Str::lit("u64::checked_div(")
                                                                                                    } else {
                                                                                                        let a401 = v0;
                                                                                                        let a402 = 21u64;
                                                                                                        if nat_eq(a401, a402) {
                                                                                                            Str::lit("i64::checked_add(")
                                                                                                        } else {
                                                                                                            let a403 = v0;
                                                                                                            let a404 = 22u64;
                                                                                                            if nat_eq(a403, a404) {
                                                                                                                Str::lit("i64::checked_sub(")
                                                                                                            } else {
                                                                                                                let a405 = v0;
                                                                                                                let a406 = 23u64;
                                                                                                                if nat_eq(a405, a406) {
                                                                                                                    Str::lit("i64::checked_mul(")
                                                                                                                } else {
                                                                                                                    let a407 = v0;
                                                                                                                    let a408 = 24u64;
                                                                                                                    if nat_eq(a407, a408) {
                                                                                                                        Str::lit("i64::checked_neg(")
                                                                                                                    } else {
                                                                                                                        let a409 = v0;
                                                                                                                        let a410 = 25u64;
                                                                                                                        if nat_eq(a409, a410) {
                                                                                                                            Str::lit("{ let (a, b, z): (i64, i64, i64) = (")
                                                                                                                        } else {
                                                                                                                            let a411 = v0;
                                                                                                                            let a412 = 26u64;
                                                                                                                            if nat_eq(a411, a412) {
                                                                                                                                Str::lit("List::length(&")
                                                                                                                            } else {
                                                                                                                                let a413 = v0;
                                                                                                                                let a414 = 27u64;
                                                                                                                                if nat_eq(a413, a414) {
                                                                                                                                    Str::lit("List::append(")
                                                                                                                                } else {
                                                                                                                                    let a415 = v0;
                                                                                                                                    let a416 = 28u64;
                                                                                                                                    if nat_eq(a415, a416) {
                                                                                                                                        Str::lit("List::index(&")
                                                                                                                                    } else {
                                                                                                                                        let a417 = v0;
                                                                                                                                        let a418 = 29u64;
                                                                                                                                        if nat_eq(a417, a418) {
                                                                                                                                            Str::lit("List::slice(&")
                                                                                                                                        } else {
                                                                                                                                            Str::lit("")
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
                }
            }
        }
    }
}

pub fn f43(v0: u64) -> Str {
    let a419 = v0;
    let a420 = 16u64;
    if nat_eq(a419, a420) {
        Str::lit("); if b == 0 { z } else { a.wrapping_rem(b) } }")
    } else {
        let a421 = v0;
        let a422 = 25u64;
        if nat_eq(a421, a422) {
            Str::lit("); if b == 0 { Some(z) } else { a.checked_div(b) } }")
        } else {
            let a423 = v0;
            let a424 = 15u64;
            if nat_eq(a423, a424) {
                Str::lit(")")
            } else {
                let a425 = v0;
                let a426 = 20u64;
                if nat_eq(a425, a426) {
                    Str::lit(")")
                } else {
                    let a427 = v0;
                    let a428 = 8u64;
                    if nat_eq(a427, a428) {
                        Str::lit(")")
                    } else {
                        let a429 = v0;
                        let a430 = 9u64;
                        if nat_eq(a429, a430) {
                            Str::lit(")")
                        } else {
                            let a431 = v0;
                            let a432 = 10u64;
                            if nat_eq(a431, a432) {
                                Str::lit(")")
                            } else {
                                let a433 = v0;
                                let a434 = 11u64;
                                if nat_eq(a433, a434) {
                                    Str::lit(")")
                                } else {
                                    let a435 = v0;
                                    let a436 = 12u64;
                                    if nat_eq(a435, a436) {
                                        Str::lit(")")
                                    } else {
                                        let a437 = v0;
                                        let a438 = 13u64;
                                        if nat_eq(a437, a438) {
                                            Str::lit(")")
                                        } else {
                                            let a439 = v0;
                                            let a440 = 14u64;
                                            if nat_eq(a439, a440) {
                                                Str::lit(")")
                                            } else {
                                                let a441 = v0;
                                                let a442 = 17u64;
                                                if nat_eq(a441, a442) {
                                                    Str::lit(")")
                                                } else {
                                                    let a443 = v0;
                                                    let a444 = 18u64;
                                                    if nat_eq(a443, a444) {
                                                        Str::lit(")")
                                                    } else {
                                                        let a445 = v0;
                                                        let a446 = 19u64;
                                                        if nat_eq(a445, a446) {
                                                            Str::lit(")")
                                                        } else {
                                                            let a447 = v0;
                                                            let a448 = 21u64;
                                                            if nat_eq(a447, a448) {
                                                                Str::lit(")")
                                                            } else {
                                                                let a449 = v0;
                                                                let a450 = 22u64;
                                                                if nat_eq(a449, a450) {
                                                                    Str::lit(")")
                                                                } else {
                                                                    let a451 = v0;
                                                                    let a452 = 23u64;
                                                                    if nat_eq(a451, a452) {
                                                                        Str::lit(")")
                                                                    } else {
                                                                        let a453 = v0;
                                                                        let a454 = 24u64;
                                                                        let _ = nat_eq(a453, a454);
                                                                        Str::lit(")")
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
        }
    }
}

pub fn f44(v0: Fn2, p459: u64, p460: (u64, Adt20)) -> R<((u64, Adt20), bool)> {
    let m458 = (p459, p460);
    {
        let mut m458 = m458;
        loop {
            let step = {
                let (v1, v2) = m458;
                let m455 = v1;
                if m455 == 0 {
                    Err::<(u64, (u64, Adt20)), R<((u64, Adt20), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m455 - 1;
                    let m457 = {
                        let c456: Fn2 = v0.clone();
                        c456.apply(v2.clone())?
                    };
                    match m457 {
                        None => {
                            Err::<(u64, (u64, Adt20)), R<((u64, Adt20), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (u64, Adt20)), R<((u64, Adt20), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m458 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f45(v0: u64, v1: (u64, Adt20)) -> R<Option<(u64, Adt20)>> {
    f59(v0, v1)
}

pub fn f46(v0: u64) -> Str {
    let a461 = v0;
    let a462 = 37u64;
    if nat_eq(a461, a462) {
        Str::lit(", &")
    } else {
        let a463 = v0;
        let a464 = 38u64;
        if nat_eq(a463, a464) {
            Str::lit(", &")
        } else {
            let a465 = v0;
            let a466 = 39u64;
            if nat_eq(a465, a466) {
                Str::lit(", &")
            } else {
                let a467 = v0;
                let a468 = 12u64;
                if nat_eq(a467, a468) {
                    Str::lit(", &")
                } else {
                    let a469 = v0;
                    let a470 = 13u64;
                    if nat_eq(a469, a470) {
                        Str::lit(", &")
                    } else {
                        let a471 = v0;
                        let a472 = 14u64;
                        if nat_eq(a471, a472) {
                            Str::lit(", &")
                        } else {
                            let a473 = v0;
                            let a474 = 8u64;
                            if nat_eq(a473, a474) {
                                Str::lit(", ")
                            } else {
                                let a475 = v0;
                                let a476 = 9u64;
                                if nat_eq(a475, a476) {
                                    Str::lit(", ")
                                } else {
                                    let a477 = v0;
                                    let a478 = 10u64;
                                    if nat_eq(a477, a478) {
                                        Str::lit(", ")
                                    } else {
                                        let a479 = v0;
                                        let a480 = 11u64;
                                        if nat_eq(a479, a480) {
                                            Str::lit(", ")
                                        } else {
                                            let a481 = v0;
                                            let a482 = 15u64;
                                            if nat_eq(a481, a482) {
                                                Str::lit(", ")
                                            } else {
                                                let a483 = v0;
                                                let a484 = 16u64;
                                                if nat_eq(a483, a484) {
                                                    Str::lit(", ")
                                                } else {
                                                    let a485 = v0;
                                                    let a486 = 17u64;
                                                    if nat_eq(a485, a486) {
                                                        Str::lit(", ")
                                                    } else {
                                                        let a487 = v0;
                                                        let a488 = 18u64;
                                                        if nat_eq(a487, a488) {
                                                            Str::lit(", ")
                                                        } else {
                                                            let a489 = v0;
                                                            let a490 = 19u64;
                                                            if nat_eq(a489, a490) {
                                                                Str::lit(", ")
                                                            } else {
                                                                let a491 = v0;
                                                                let a492 = 20u64;
                                                                if nat_eq(a491, a492) {
                                                                    Str::lit(", ")
                                                                } else {
                                                                    let a493 = v0;
                                                                    let a494 = 21u64;
                                                                    if nat_eq(a493, a494) {
                                                                        Str::lit(", ")
                                                                    } else {
                                                                        let a495 = v0;
                                                                        let a496 = 22u64;
                                                                        if nat_eq(a495, a496) {
                                                                            Str::lit(", ")
                                                                        } else {
                                                                            let a497 = v0;
                                                                            let a498 = 23u64;
                                                                            if nat_eq(a497, a498) {
                                                                                Str::lit(", ")
                                                                            } else {
                                                                                let a499 = v0;
                                                                                let a500 = 24u64;
                                                                                if nat_eq(a499, a500) {
                                                                                    Str::lit(", ")
                                                                                } else {
                                                                                    let a501 = v0;
                                                                                    let a502 = 25u64;
                                                                                    let _ = nat_eq(a501, a502);
                                                                                    Str::lit(", ")
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
                    }
                }
            }
        }
    }
}

pub fn f47(v0: u64) -> Str {
    let a503 = v0;
    let a504 = 15u64;
    if nat_eq(a503, a504) {
        Str::lit(").unwrap_or(")
    } else {
        let a505 = v0;
        let a506 = 16u64;
        if nat_eq(a505, a506) {
            Str::lit(", ")
        } else {
            let a507 = v0;
            let a508 = 20u64;
            if nat_eq(a507, a508) {
                Str::lit(").unwrap_or(")
            } else {
                let a509 = v0;
                let a510 = 25u64;
                let _ = nat_eq(a509, a510);
                Str::lit(", ")
            }
        }
    }
}

pub fn f48(v0: Adt20, v1: Adt12) -> R<Adt20> {
    f8(v0, {
        let m511 = v1;
        match m511 {
            Adt12::C0 => {
                Str::lit("()")
            }
            Adt12::C1 => {
                Str::lit("bool")
            }
            Adt12::C2 => {
                Str::lit("u64")
            }
            Adt12::C3 => {
                Str::lit("i64")
            }
            Adt12::C4 => {
                Str::lit("Ordering")
            }
            Adt12::C5 => {
                Str::lit("Str")
            }
            Adt12::C6 => {
                Str::lit("Bytes")
            }
            Adt12::C7 => {
                Str::lit("Overflow")
            }
            Adt12::C8 => {
                Str::lit("i128")
            }
        }
    })
}

pub fn f49(v0: Adt13) -> Str {
    let m512 = v0;
    match m512 {
        Adt13::C0 => {
            Str::lit("Option<")
        }
        Adt13::C1 => {
            Str::lit("List<")
        }
        Adt13::C2 => {
            Str::lit("std::rc::Rc<")
        }
        Adt13::C3 => {
            Str::lit("R<")
        }
        Adt13::C4 => {
            Str::lit("&")
        }
    }
}

pub fn f50(v0: Adt13) -> Str {
    let m513 = v0;
    match m513 {
        Adt13::C0 => {
            Str::lit(">")
        }
        Adt13::C1 => {
            Str::lit(">")
        }
        Adt13::C2 => {
            Str::lit(">")
        }
        Adt13::C3 => {
            Str::lit(">")
        }
        Adt13::C4 => {
            Str::lit("")
        }
    }
}

pub fn f51(v0: Adt20, v1: u64) -> R<Adt20> {
    let a514 = v1;
    let a515 = 0u64;
    if nat_eq(a514, a515) {
        f8(v0, Str::lit("T"))
    } else {
        let a516 = v1;
        let a517 = 1u64;
        if nat_eq(a516, a517) {
            f8(v0, Str::lit("U"))
        } else {
            let a518 = v1;
            let a519 = 2u64;
            if nat_eq(a518, a519) {
                f8(v0, Str::lit("V"))
            } else {
                f11(f8(v0, Str::lit("T"))?, v1)
            }
        }
    }
}

pub fn f52(v0: List<Bytes>) -> Option<List<Bytes>> {
    let m520 = v0.clone();
    match m520.uncons() {
        None => {
            None::<List<Bytes>>
        }
        Some((_, v2)) => {
            let m521 = v2;
            match m521.uncons() {
                None => {
                    None::<List<Bytes>>
                }
                Some((_, _)) => {
                    let v5: (Option<Bytes>, List<Bytes>) = f60(Fn3::F61, (None::<Bytes>, List::<Bytes>::nil()), v0);
                    let m523 = {
                        let (h522, _) = v5;
                        h522
                    };
                    match m523 {
                        None => {
                            Some(f5({
                                let (_, h525) = v5;
                                h525
                            }))
                        }
                        Some(v6) => {
                            Some(f5(List::cons(v6, {
                                let (_, h524) = v5;
                                h524
                            })))
                        }
                    }
                }
            }
        }
    }
}

pub fn f53(v0: Adt2) -> Str {
    let m526 = v0;
    match m526 {
        Adt2::C0 => {
            Str::lit("u8")
        }
        Adt2::C1 => {
            Str::lit("u16")
        }
        Adt2::C2 => {
            Str::lit("u32")
        }
        Adt2::C3 => {
            Str::lit("u64")
        }
        Adt2::C4 => {
            Str::lit("i8")
        }
        Adt2::C5 => {
            Str::lit("i16")
        }
        Adt2::C6 => {
            Str::lit("i32")
        }
        Adt2::C7 => {
            Str::lit("i64")
        }
    }
}

pub fn f54(v0: Bytes, v1: (u64, Adt20)) -> R<Option<(u64, Adt20)>> {
    f62(v0, v1)
}

pub fn f55(v0: Bytes, v1: (u64, Adt20)) -> R<Option<(u64, Adt20)>> {
    f63(v0, v1)
}

pub fn f56(v0: Adt20, v1: Bytes, v2: u64) -> Adt20 {
    let a528 = 256u64;
    let a529 = {
        let a527 = v1.clone();
        length_bytes(a527)
    };
    if nat_le(a528, a529) {
        Adt20::C0(List::cons(v1, match v0.clone() {
            Adt20::C0(h530, _, _, _, _) => {
                h530
            }
        }), Bytes::lit(&[]), v2, match v0 {
            Adt20::C0(_, _, _, h531, _) => {
                h531
            }
        }, false)
    } else {
        Adt20::C0(match v0.clone() {
            Adt20::C0(h532, _, _, _, _) => {
                h532
            }
        }, v1, v2, match v0 {
            Adt20::C0(_, _, _, h533, _) => {
                h533
            }
        }, false)
    }
}

pub fn f57(v0: Adt20) -> Adt20 {
    Adt20::C0(match v0.clone() {
        Adt20::C0(h534, _, _, _, _) => {
            h534
        }
    }, match v0.clone() {
        Adt20::C0(_, h535, _, _, _) => {
            h535
        }
    }, match v0.clone() {
        Adt20::C0(_, _, h536, _, _) => {
            h536
        }
    }, match v0 {
        Adt20::C0(_, _, _, h537, _) => {
            h537
        }
    }, true)
}

pub fn f58(v0: u64) -> R<Bytes> {
    let v1: Bytes = {
        let a574 = f64(v0, 10000000000000000000u64);
        let a575 = {
            let a572 = f64(v0, 1000000000000000000u64);
            let a573 = {
                let a570 = f64(v0, 100000000000000000u64);
                let a571 = {
                    let a568 = f64(v0, 10000000000000000u64);
                    let a569 = {
                        let a566 = f64(v0, 1000000000000000u64);
                        let a567 = {
                            let a564 = f64(v0, 100000000000000u64);
                            let a565 = {
                                let a562 = f64(v0, 10000000000000u64);
                                let a563 = {
                                    let a560 = f64(v0, 1000000000000u64);
                                    let a561 = {
                                        let a558 = f64(v0, 100000000000u64);
                                        let a559 = {
                                            let a556 = f64(v0, 10000000000u64);
                                            let a557 = {
                                                let a554 = f64(v0, 1000000000u64);
                                                let a555 = {
                                                    let a552 = f64(v0, 100000000u64);
                                                    let a553 = {
                                                        let a550 = f64(v0, 10000000u64);
                                                        let a551 = {
                                                            let a548 = f64(v0, 1000000u64);
                                                            let a549 = {
                                                                let a546 = f64(v0, 100000u64);
                                                                let a547 = {
                                                                    let a544 = f64(v0, 10000u64);
                                                                    let a545 = {
                                                                        let a542 = f64(v0, 1000u64);
                                                                        let a543 = {
                                                                            let a540 = f64(v0, 100u64);
                                                                            let a541 = {
                                                                                let a538 = f64(v0, 10u64);
                                                                                let a539 = f64(v0, 1u64);
                                                                                append_bytes(a538, a539)
                                                                            };
                                                                            append_bytes(a540, a541)
                                                                        };
                                                                        append_bytes(a542, a543)
                                                                    };
                                                                    append_bytes(a544, a545)
                                                                };
                                                                append_bytes(a546, a547)
                                                            };
                                                            append_bytes(a548, a549)
                                                        };
                                                        append_bytes(a550, a551)
                                                    };
                                                    append_bytes(a552, a553)
                                                };
                                                append_bytes(a554, a555)
                                            };
                                            append_bytes(a556, a557)
                                        };
                                        append_bytes(a558, a559)
                                    };
                                    append_bytes(a560, a561)
                                };
                                append_bytes(a562, a563)
                            };
                            append_bytes(a564, a565)
                        };
                        append_bytes(a566, a567)
                    };
                    append_bytes(a568, a569)
                };
                append_bytes(a570, a571)
            };
            append_bytes(a572, a573)
        };
        append_bytes(a574, a575)
    };
    let v2: u64 = {
        let (h576, _) = f65(Fn4::F66(v1.clone()), {
            let a577 = v1.clone();
            length_bytes(a577)
        }, 0u64)?;
        h576
    };
    let a578 = v0;
    let a579 = 0u64;
    if nat_eq(a578, a579) {
        let a580 = Str::lit("0");
        Ok(utf8_encode(a580))
    } else {
        Ok(f67(v1.clone(), v2, {
            let a582 = {
                let a581 = v1;
                length_bytes(a581)
            };
            let a583 = v2;
            nat_sub(a582, a583)
        }))
    }
}

pub fn f59(v0: u64, v1: (u64, Adt20)) -> R<Option<(u64, Adt20)>> {
    let a585 = {
        let (h584, _) = v1.clone();
        h584
    };
    let a586 = v0;
    if nat_lt(a585, a586) {
        Ok(Some(({
            let a588 = {
                let (h587, _) = v1;
                h587
            };
            let a589 = 1u64;
            nat_add(a588, a589)?
        }, f8({
            let (_, h590) = v1;
            h590
        }, Str::lit("    "))?)))
    } else {
        Ok(None::<(u64, Adt20)>)
    }
}

pub fn f60(v0: Fn3, p594: (Option<Bytes>, List<Bytes>), p595: List<Bytes>) -> (Option<Bytes>, List<Bytes>) {
    let m593 = (p594, p595);
    {
        let mut m593 = m593;
        loop {
            let step = {
                let (v1, v2) = m593;
                let m591 = v2;
                match m591.uncons() {
                    None => {
                        Err::<((Option<Bytes>, List<Bytes>), List<Bytes>), (Option<Bytes>, List<Bytes>)>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<((Option<Bytes>, List<Bytes>), List<Bytes>), (Option<Bytes>, List<Bytes>)>(({
                            let c592: Fn3 = v0.clone();
                            c592.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m593 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f61(v0: (Option<Bytes>, List<Bytes>), v1: Bytes) -> (Option<Bytes>, List<Bytes>) {
    f68(v0, v1)
}

pub fn f62(v0: Bytes, v1: (u64, Adt20)) -> R<Option<(u64, Adt20)>> {
    let a598 = {
        let (h596, _) = v1.clone();
        h596
    };
    let a599 = {
        let a597 = v0.clone();
        length_bytes(a597)
    };
    if nat_lt(a598, a599) {
        let v2: (u64, u64) = f69(v0.clone(), {
            let (h600, _) = v1.clone();
            h600
        })?;
        let v3: u64 = {
            let (h601, _) = v2;
            h601
        };
        Ok(Some(({
            let a604 = {
                let (h602, _) = v1.clone();
                h602
            };
            let a605 = {
                let (_, h603) = v2;
                h603
            };
            nat_add(a604, a605)?
        }, {
            let a606 = v3;
            let a607 = 34u64;
            if nat_eq(a606, a607) {
                f8({
                    let (_, h608) = v1;
                    h608
                }, Str::lit("\\\""))?
            } else {
                let a609 = v3;
                let a610 = 92u64;
                if nat_eq(a609, a610) {
                    f8({
                        let (_, h611) = v1;
                        h611
                    }, Str::lit("\\\\"))?
                } else {
                    let a612 = 32u64;
                    let a613 = v3;
                    if if nat_le(a612, a613) {
                        let a614 = v3;
                        let a615 = 126u64;
                        nat_le(a614, a615)
                    } else {
                        false
                    } {
                        f36({
                            let (_, h616) = v1;
                            h616
                        }, f67(v0, {
                            let (h617, _) = v1;
                            h617
                        }, 1u64))?
                    } else {
                        f8(f70(f8({
                            let (_, h618) = v1;
                            h618
                        }, Str::lit("\\u{"))?, v3)?, Str::lit("}"))?
                    }
                }
            }
        })))
    } else {
        Ok(None::<(u64, Adt20)>)
    }
}

pub fn f63(v0: Bytes, v1: (u64, Adt20)) -> R<Option<(u64, Adt20)>> {
    let m620 = f71(v0, {
        let (h619, _) = v1.clone();
        h619
    });
    match m620 {
        None => {
            Ok(None::<(u64, Adt20)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a622 = {
                    let (h621, _) = v1.clone();
                    h621
                };
                let a623 = 1u64;
                nat_add(a622, a623)?
            }, f72(f8({
                let a625 = {
                    let (h624, _) = v1;
                    h624
                };
                let a626 = 0u64;
                if nat_eq(a625, a626) {
                    let (_, h627) = v1;
                    h627
                } else {
                    f8({
                        let (_, h628) = v1;
                        h628
                    }, Str::lit(", "))?
                }
            }, Str::lit("0x"))?, v2)?)))
        }
    }
}

pub fn f64(v0: u64, v1: u64) -> Bytes {
    f67(Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39]), {
        let a632 = {
            let a629 = v0;
            let a630 = v1;
            let a631 = 0u64;
            nat_quot(a629, a630, a631)
        };
        let a633 = 10u64;
        let a634 = 0u64;
        nat_rem(a632, a633, a634)
    }, 1u64)
}

pub fn f65(v0: Fn4, p639: u64, p640: u64) -> R<(u64, bool)> {
    let m638 = (p639, p640);
    {
        let mut m638 = m638;
        loop {
            let step = {
                let (v1, v2) = m638;
                let m635 = v1;
                if m635 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m635 - 1;
                    let m637 = {
                        let c636: Fn4 = v0.clone();
                        c636.apply(v2)?
                    };
                    match m637 {
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
                Ok(next) => { m638 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f66(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f73(v0, v1)
}

pub fn f67(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m644 = {
        let a641 = v0;
        let a642 = v1;
        let a643 = v2;
        slice_bytes(a641, a642, a643)
    };
    match m644 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f68(v0: (Option<Bytes>, List<Bytes>), v1: Bytes) -> (Option<Bytes>, List<Bytes>) {
    let m646 = {
        let (h645, _) = v0;
        h645
    };
    match m646 {
        None => {
            (Some(v1), {
                let (_, h647) = v0;
                h647
            })
        }
        Some(v2) => {
            (None::<Bytes>, List::cons({
                let a648 = v2;
                let a649 = v1;
                append_bytes(a648, a649)
            }, {
                let (_, h650) = v0;
                h650
            }))
        }
    }
}

pub fn f69(v0: Bytes, v1: u64) -> R<(u64, u64)> {
    let m651 = f71(v0.clone(), v1);
    match m651 {
        None => {
            Ok((0u64, 1u64))
        }
        Some(v2) => {
            let v4: u64 = {
                let v3: u8 = v2;
                let a696 = {
                    let a672 = {
                        let a660 = {
                            let a654 = {
                                let a652 = v3;
                                let a653 = 128u8;
                                fixed_u8::bit_and(a652, a653)
                            };
                            let a655 = 128u8;
                            if equal(a654, a655) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a661 = {
                            let a658 = {
                                let a656 = v3;
                                let a657 = 64u8;
                                fixed_u8::bit_and(a656, a657)
                            };
                            let a659 = 64u8;
                            if equal(a658, a659) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a660, a661)?
                    };
                    let a673 = {
                        let a670 = {
                            let a664 = {
                                let a662 = v3;
                                let a663 = 32u8;
                                fixed_u8::bit_and(a662, a663)
                            };
                            let a665 = 32u8;
                            if equal(a664, a665) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a671 = {
                            let a668 = {
                                let a666 = v3;
                                let a667 = 16u8;
                                fixed_u8::bit_and(a666, a667)
                            };
                            let a669 = 16u8;
                            if equal(a668, a669) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a670, a671)?
                    };
                    nat_add(a672, a673)?
                };
                let a697 = {
                    let a694 = {
                        let a682 = {
                            let a676 = {
                                let a674 = v3;
                                let a675 = 8u8;
                                fixed_u8::bit_and(a674, a675)
                            };
                            let a677 = 8u8;
                            if equal(a676, a677) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a683 = {
                            let a680 = {
                                let a678 = v3;
                                let a679 = 4u8;
                                fixed_u8::bit_and(a678, a679)
                            };
                            let a681 = 4u8;
                            if equal(a680, a681) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a682, a683)?
                    };
                    let a695 = {
                        let a692 = {
                            let a686 = {
                                let a684 = v3;
                                let a685 = 2u8;
                                fixed_u8::bit_and(a684, a685)
                            };
                            let a687 = 2u8;
                            if equal(a686, a687) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a693 = {
                            let a690 = {
                                let a688 = v3;
                                let a689 = 1u8;
                                fixed_u8::bit_and(a688, a689)
                            };
                            let a691 = 1u8;
                            if equal(a690, a691) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a692, a693)?
                    };
                    nat_add(a694, a695)?
                };
                nat_add(a696, a697)?
            };
            let a698 = v4;
            let a699 = 128u64;
            if nat_lt(a698, a699) {
                Ok((v4, 1u64))
            } else {
                let a700 = v4;
                let a701 = 224u64;
                if nat_lt(a700, a701) {
                    Ok(({
                        let a709 = {
                            let a705 = {
                                let a702 = v4;
                                let a703 = 32u64;
                                let a704 = 0u64;
                                nat_rem(a702, a703, a704)
                            };
                            let a706 = 64u64;
                            nat_mul(a705, a706)?
                        };
                        let a710 = f74(v0, {
                            let a707 = v1;
                            let a708 = 1u64;
                            nat_add(a707, a708)?
                        })?;
                        nat_add(a709, a710)?
                    }, 2u64))
                } else {
                    let a711 = v4;
                    let a712 = 240u64;
                    if nat_lt(a711, a712) {
                        Ok(({
                            let a726 = {
                                let a716 = {
                                    let a713 = v4;
                                    let a714 = 16u64;
                                    let a715 = 0u64;
                                    nat_rem(a713, a714, a715)
                                };
                                let a717 = 4096u64;
                                nat_mul(a716, a717)?
                            };
                            let a727 = {
                                let a724 = {
                                    let a720 = f74(v0.clone(), {
                                        let a718 = v1;
                                        let a719 = 1u64;
                                        nat_add(a718, a719)?
                                    })?;
                                    let a721 = 64u64;
                                    nat_mul(a720, a721)?
                                };
                                let a725 = f74(v0, {
                                    let a722 = v1;
                                    let a723 = 2u64;
                                    nat_add(a722, a723)?
                                })?;
                                nat_add(a724, a725)?
                            };
                            nat_add(a726, a727)?
                        }, 3u64))
                    } else {
                        Ok(({
                            let a747 = {
                                let a731 = {
                                    let a728 = v4;
                                    let a729 = 8u64;
                                    let a730 = 0u64;
                                    nat_rem(a728, a729, a730)
                                };
                                let a732 = 262144u64;
                                nat_mul(a731, a732)?
                            };
                            let a748 = {
                                let a745 = {
                                    let a735 = f74(v0.clone(), {
                                        let a733 = v1;
                                        let a734 = 1u64;
                                        nat_add(a733, a734)?
                                    })?;
                                    let a736 = 4096u64;
                                    nat_mul(a735, a736)?
                                };
                                let a746 = {
                                    let a743 = {
                                        let a739 = f74(v0.clone(), {
                                            let a737 = v1;
                                            let a738 = 2u64;
                                            nat_add(a737, a738)?
                                        })?;
                                        let a740 = 64u64;
                                        nat_mul(a739, a740)?
                                    };
                                    let a744 = f74(v0, {
                                        let a741 = v1;
                                        let a742 = 3u64;
                                        nat_add(a741, a742)?
                                    })?;
                                    nat_add(a743, a744)?
                                };
                                nat_add(a745, a746)?
                            };
                            nat_add(a747, a748)?
                        }, 4u64))
                    }
                }
            }
        }
    }
}

pub fn f70(v0: Adt20, v1: u64) -> R<Adt20> {
    let v2: Bytes = {
        let a761 = f75(v1, 268435456u64);
        let a762 = {
            let a759 = f75(v1, 16777216u64);
            let a760 = {
                let a757 = f75(v1, 1048576u64);
                let a758 = {
                    let a755 = f75(v1, 65536u64);
                    let a756 = {
                        let a753 = f75(v1, 4096u64);
                        let a754 = {
                            let a751 = f75(v1, 256u64);
                            let a752 = {
                                let a749 = f75(v1, 16u64);
                                let a750 = f75(v1, 1u64);
                                append_bytes(a749, a750)
                            };
                            append_bytes(a751, a752)
                        };
                        append_bytes(a753, a754)
                    };
                    append_bytes(a755, a756)
                };
                append_bytes(a757, a758)
            };
            append_bytes(a759, a760)
        };
        append_bytes(a761, a762)
    };
    let v3: u64 = {
        let (h763, _) = f65(Fn4::F76(v2.clone()), 7u64, 0u64)?;
        h763
    };
    f36(v0, f67(v2, v3, {
        let a764 = 8u64;
        let a765 = v3;
        nat_sub(a764, a765)
    }))
}

pub fn f71(v0: Bytes, v1: u64) -> Option<u8> {
    let a766 = v0;
    let a767 = v1;
    index_bytes(a766, a767)
}

pub fn f72(v0: Adt20, v1: u8) -> R<Adt20> {
    let v3: u64 = {
        let v2: u8 = v1;
        let a812 = {
            let a788 = {
                let a776 = {
                    let a770 = {
                        let a768 = v2;
                        let a769 = 128u8;
                        fixed_u8::bit_and(a768, a769)
                    };
                    let a771 = 128u8;
                    if equal(a770, a771) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a777 = {
                    let a774 = {
                        let a772 = v2;
                        let a773 = 64u8;
                        fixed_u8::bit_and(a772, a773)
                    };
                    let a775 = 64u8;
                    if equal(a774, a775) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a776, a777)?
            };
            let a789 = {
                let a786 = {
                    let a780 = {
                        let a778 = v2;
                        let a779 = 32u8;
                        fixed_u8::bit_and(a778, a779)
                    };
                    let a781 = 32u8;
                    if equal(a780, a781) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a787 = {
                    let a784 = {
                        let a782 = v2;
                        let a783 = 16u8;
                        fixed_u8::bit_and(a782, a783)
                    };
                    let a785 = 16u8;
                    if equal(a784, a785) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a786, a787)?
            };
            nat_add(a788, a789)?
        };
        let a813 = {
            let a810 = {
                let a798 = {
                    let a792 = {
                        let a790 = v2;
                        let a791 = 8u8;
                        fixed_u8::bit_and(a790, a791)
                    };
                    let a793 = 8u8;
                    if equal(a792, a793) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a799 = {
                    let a796 = {
                        let a794 = v2;
                        let a795 = 4u8;
                        fixed_u8::bit_and(a794, a795)
                    };
                    let a797 = 4u8;
                    if equal(a796, a797) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a798, a799)?
            };
            let a811 = {
                let a808 = {
                    let a802 = {
                        let a800 = v2;
                        let a801 = 2u8;
                        fixed_u8::bit_and(a800, a801)
                    };
                    let a803 = 2u8;
                    if equal(a802, a803) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a809 = {
                    let a806 = {
                        let a804 = v2;
                        let a805 = 1u8;
                        fixed_u8::bit_and(a804, a805)
                    };
                    let a807 = 1u8;
                    if equal(a806, a807) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a808, a809)?
            };
            nat_add(a810, a811)?
        };
        nat_add(a812, a813)?
    };
    f36(v0, {
        let a814 = f75(v3, 16u64);
        let a815 = f75(v3, 1u64);
        append_bytes(a814, a815)
    })
}

pub fn f73(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m816 = f71(v0, v1);
    match m816 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            let a817 = v2;
            let a818 = 48u8;
            if equal(a817, a818) {
                Ok(Some({
                    let a819 = v1;
                    let a820 = 1u64;
                    nat_add(a819, a820)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f74(v0: Bytes, v1: u64) -> R<u64> {
    let m821 = f71(v0, v1);
    match m821 {
        None => {
            Ok(0u64)
        }
        Some(v2) => {
            let a868 = {
                let v3: u8 = v2;
                let a866 = {
                    let a842 = {
                        let a830 = {
                            let a824 = {
                                let a822 = v3;
                                let a823 = 128u8;
                                fixed_u8::bit_and(a822, a823)
                            };
                            let a825 = 128u8;
                            if equal(a824, a825) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a831 = {
                            let a828 = {
                                let a826 = v3;
                                let a827 = 64u8;
                                fixed_u8::bit_and(a826, a827)
                            };
                            let a829 = 64u8;
                            if equal(a828, a829) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a830, a831)?
                    };
                    let a843 = {
                        let a840 = {
                            let a834 = {
                                let a832 = v3;
                                let a833 = 32u8;
                                fixed_u8::bit_and(a832, a833)
                            };
                            let a835 = 32u8;
                            if equal(a834, a835) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a841 = {
                            let a838 = {
                                let a836 = v3;
                                let a837 = 16u8;
                                fixed_u8::bit_and(a836, a837)
                            };
                            let a839 = 16u8;
                            if equal(a838, a839) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a840, a841)?
                    };
                    nat_add(a842, a843)?
                };
                let a867 = {
                    let a864 = {
                        let a852 = {
                            let a846 = {
                                let a844 = v3;
                                let a845 = 8u8;
                                fixed_u8::bit_and(a844, a845)
                            };
                            let a847 = 8u8;
                            if equal(a846, a847) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a853 = {
                            let a850 = {
                                let a848 = v3;
                                let a849 = 4u8;
                                fixed_u8::bit_and(a848, a849)
                            };
                            let a851 = 4u8;
                            if equal(a850, a851) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a852, a853)?
                    };
                    let a865 = {
                        let a862 = {
                            let a856 = {
                                let a854 = v3;
                                let a855 = 2u8;
                                fixed_u8::bit_and(a854, a855)
                            };
                            let a857 = 2u8;
                            if equal(a856, a857) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a863 = {
                            let a860 = {
                                let a858 = v3;
                                let a859 = 1u8;
                                fixed_u8::bit_and(a858, a859)
                            };
                            let a861 = 1u8;
                            if equal(a860, a861) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a862, a863)?
                    };
                    nat_add(a864, a865)?
                };
                nat_add(a866, a867)?
            };
            let a869 = 64u64;
            let a870 = 0u64;
            Ok(nat_rem(a868, a869, a870))
        }
    }
}

pub fn f75(v0: u64, v1: u64) -> Bytes {
    f67(Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66]), {
        let a874 = {
            let a871 = v0;
            let a872 = v1;
            let a873 = 0u64;
            nat_quot(a871, a872, a873)
        };
        let a875 = 16u64;
        let a876 = 0u64;
        nat_rem(a874, a875, a876)
    }, 1u64)
}

pub fn f76(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f77(v0, v1)
}

pub fn f77(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m877 = f71(v0, v1);
    match m877 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            let a878 = v2;
            let a879 = 48u8;
            if equal(a878, a879) {
                Ok(Some({
                    let a880 = v1;
                    let a881 = 1u64;
                    nat_add(a880, a881)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn run(p0: u64, p1: Adt0) -> R<Result<Bytes, Bytes>> {
    f0(p0, p1)
}
