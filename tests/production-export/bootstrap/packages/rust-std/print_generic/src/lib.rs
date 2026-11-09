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
    C0,
    C1(Bytes),
    C2(std::rc::Rc<Adt1>, std::rc::Rc<Adt1>),
    C3(u64, std::rc::Rc<Adt1>),
    C4,
    C5,
    C6(std::rc::Rc<Adt1>),
}

#[derive(Clone)]
pub enum Adt2 {
    C0(List<Bytes>, u64, List<Adt3>),
}

#[derive(Clone)]
pub enum Adt3 {
    C0(u64, bool, Adt1),
}

#[derive(Clone)]
pub enum Adt4 {
    C0(u64, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt5 {
    C0(u64, u64, bool, List<Adt3>, List<Adt3>, Option<bool>),
}

#[derive(Clone)]
pub enum Fn0 {
    F14(u64, u64),
}

impl Fn0 {
    pub fn apply(&self, p0: Adt2) -> R<Option<Adt2>> {
        match self {
            Fn0::F14(k0, k1) => f14(*k0, *k1, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F21,
}

impl Fn1 {
    pub fn apply(&self, p0: (bool, Adt1), p1: Adt1) -> (bool, Adt1) {
        match self {
            Fn1::F21 => f21(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F23,
}

impl Fn2 {
    pub fn apply(&self, p0: List<Adt1>, p1: Adt1) -> List<Adt1> {
        match self {
            Fn2::F23 => f23(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F25(Option<Bytes>),
    F107,
}

impl Fn3 {
    pub fn apply(&self, p0: List<(Bytes, Adt1)>, p1: (Bytes, Adt1)) -> R<List<(Bytes, Adt1)>> {
        match self {
            Fn3::F25(k0) => f25(k0.clone(), p0, p1),
            Fn3::F107 => Ok(f107(p0, p1)),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F27,
}

impl Fn4 {
    pub fn apply(&self, p0: List<Adt1>, p1: (Bytes, Adt1)) -> List<Adt1> {
        match self {
            Fn4::F27 => f27(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F39(u64, Bytes),
}

impl Fn5 {
    pub fn apply(&self, p0: Adt4) -> R<Option<Adt4>> {
        match self {
            Fn5::F39(k0, k1) => f39(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F48,
}

impl Fn6 {
    pub fn apply(&self, p0: List<List<(Bytes, Adt1)>>, p1: (Bytes, Adt1)) -> List<List<(Bytes, Adt1)>> {
        match self {
            Fn6::F48 => f48(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F50,
}

impl Fn7 {
    pub fn apply(&self, p0: List<List<(Bytes, Adt1)>>) -> R<Option<List<List<(Bytes, Adt1)>>>> {
        match self {
            Fn7::F50 => f50(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F53,
}

impl Fn8 {
    pub fn apply(&self, p0: List<Bytes>) -> Option<List<Bytes>> {
        match self {
            Fn8::F53 => f53(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F55,
}

impl Fn9 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn9::F55 => f55(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F58(u64, Bytes),
}

impl Fn10 {
    pub fn apply(&self, p0: (u64, Bytes)) -> Option<(u64, Bytes)> {
        match self {
            Fn10::F58(k0, k1) => f58(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F61(Bytes),
    F88(u64, Bytes),
}

impl Fn11 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn11::F61(k0) => f61(k0.clone(), p0),
            Fn11::F88(k0, k1) => f88(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F66,
}

impl Fn12 {
    pub fn apply(&self, p0: List<List<(Bytes, Adt1)>>, p1: List<(Bytes, Adt1)>) -> List<List<(Bytes, Adt1)>> {
        match self {
            Fn12::F66 => f66(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F80(Bytes),
}

impl Fn13 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn13::F80(k0) => f80(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F83,
}

impl Fn14 {
    pub fn apply(&self, p0: Adt5) -> R<Option<Adt5>> {
        match self {
            Fn14::F83 => f83(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F85,
}

impl Fn15 {
    pub fn apply(&self, p0: (Option<Bytes>, List<Bytes>), p1: Bytes) -> (Option<Bytes>, List<Bytes>) {
        match self {
            Fn15::F85 => f85(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F90,
}

impl Fn16 {
    pub fn apply(&self, p0: (List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>)) -> R<Option<(List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>)>> {
        match self {
            Fn16::F90 => f90(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F93(u64),
}

impl Fn17 {
    pub fn apply(&self, p0: (u64, List<Bytes>)) -> R<Option<(u64, List<Bytes>)>> {
        match self {
            Fn17::F93(k0) => f93(*k0, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F106,
}

impl Fn18 {
    pub fn apply(&self, p0: (List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>))) -> Option<(List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>))> {
        match self {
            Fn18::F106 => f106(p0),
        }
    }
}

pub fn f0(v0: Adt0) -> R<Bytes> {
    let v1: Adt1 = f1(v0)?;
    f2(v1.clone(), 100u64, f3(v1)?)
}

pub fn f1(v0: Adt0) -> R<Adt1> {
    let m1 = v0;
    match m1 {
        Adt0::C0(v1) => {
            Ok(Adt1::C1(if v1 {
                Bytes::lit(&[0x74, 0x72, 0x75, 0x65])
            } else {
                Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65])
            }))
        }
        Adt0::C1(v2, v3) => {
            Ok(Adt1::C1(f4(v2, v3)))
        }
        Adt0::C2(v4) => {
            Ok(Adt1::C1(f5(v4)?))
        }
        Adt0::C3(r2) => {
            let v5 = (*r2).clone();
            Ok(f6(f7(Str::lit("[")), f7(Str::lit("]")), f8(f9(v5)?)))
        }
        Adt0::C4(r3) => {
            let v6 = (*r3).clone();
            f10(f11(v6.clone())?, f12(v6)?)
        }
        Adt0::C5 => {
            Ok(Adt1::C0)
        }
        Adt0::C6(_, _) => {
            Ok(Adt1::C0)
        }
        Adt0::C7(_, _, _) => {
            Ok(Adt1::C0)
        }
    }
}

pub fn f2(v0: Adt1, v1: u64, v2: u64) -> R<Bytes> {
    let v3: Adt2 = {
        let (h4, _) = f13(Fn0::F14(v2, v1), {
            let a5 = v2;
            let a6 = 1u64;
            nat_add(a5, a6)?
        }, Adt2::C0(List::<Bytes>::nil(), 0u64, List::cons(Adt3::C0(0u64, false, v0), List::<Adt3>::nil())))?;
        h4
    };
    Ok(f15(match v3 {
        Adt2::C0(h7, _, _) => {
            h7
        }
    }))
}

pub fn f3(v0: Adt1) -> R<u64> {
    let m8 = v0;
    match m8 {
        Adt1::C0 => {
            Ok(1u64)
        }
        Adt1::C1(_) => {
            Ok(1u64)
        }
        Adt1::C2(r17, r18) => {
            let v2 = (*r17).clone();
            let v3 = (*r18).clone();
            let v4: u64 = f3(v2)?;
            let v5: u64 = f3(v3)?;
            let a11 = {
                let a9 = 1u64;
                let a10 = v4;
                nat_add(a9, a10)?
            };
            let a12 = v5;
            nat_add(a11, a12)
        }
        Adt1::C3(_, r19) => {
            let v7 = (*r19).clone();
            let v8: u64 = f3(v7)?;
            let a13 = 1u64;
            let a14 = v8;
            nat_add(a13, a14)
        }
        Adt1::C4 => {
            Ok(1u64)
        }
        Adt1::C5 => {
            Ok(1u64)
        }
        Adt1::C6(r20) => {
            let v9 = (*r20).clone();
            let v10: u64 = f3(v9)?;
            let a15 = 1u64;
            let a16 = v10;
            nat_add(a15, a16)
        }
    }
}

pub fn f4(v0: bool, v1: u64) -> Bytes {
    if if v0 {
        let a21 = 0u64;
        let a22 = v1;
        nat_lt(a21, a22)
    } else {
        false
    } {
        let a23 = Bytes::lit(&[0x2d]);
        let a24 = f16(v1);
        append_bytes(a23, a24)
    } else {
        f16(v1)
    }
}

pub fn f5(v0: Bytes) -> R<Bytes> {
    if f17(v0.clone())? {
        Ok(v0)
    } else {
        f18(v0)
    }
}

pub fn f6(v0: Bytes, v1: Bytes, v2: List<Adt1>) -> Adt1 {
    let m25 = v2.clone();
    match m25.uncons() {
        None => {
            Adt1::C1(f19(v0, v1))
        }
        Some((_, _)) => {
            let v5: (bool, Adt1) = f20(Fn1::F21, (true, Adt1::C0), v2);
            Adt1::C6(std::rc::Rc::new(Adt1::C2(std::rc::Rc::new(Adt1::C1(v0)), std::rc::Rc::new(Adt1::C2(std::rc::Rc::new(Adt1::C3(2u64, std::rc::Rc::new(Adt1::C2(std::rc::Rc::new(Adt1::C5), std::rc::Rc::new(Adt1::C2(std::rc::Rc::new({
                let (_, h26) = v5;
                h26
            }), std::rc::Rc::new(Adt1::C0))))))), std::rc::Rc::new(Adt1::C2(std::rc::Rc::new(Adt1::C5), std::rc::Rc::new(Adt1::C2(std::rc::Rc::new(Adt1::C1(v1)), std::rc::Rc::new(Adt1::C0))))))))))
        }
    }
}

pub fn f7(v0: Str) -> Bytes {
    let a27 = v0;
    utf8_encode(a27)
}

pub fn f8(v0: List<Adt1>) -> List<Adt1> {
    f22(Fn2::F23, List::<Adt1>::nil(), v0)
}

pub fn f9(v0: Adt0) -> R<List<Adt1>> {
    let m28 = v0;
    match m28 {
        Adt0::C0(_) => {
            Ok(List::<Adt1>::nil())
        }
        Adt0::C1(_, _) => {
            Ok(List::<Adt1>::nil())
        }
        Adt0::C2(_) => {
            Ok(List::<Adt1>::nil())
        }
        Adt0::C3(_) => {
            Ok(List::<Adt1>::nil())
        }
        Adt0::C4(_) => {
            Ok(List::<Adt1>::nil())
        }
        Adt0::C5 => {
            Ok(List::<Adt1>::nil())
        }
        Adt0::C6(r29, r30) => {
            let v7 = (*r29).clone();
            let v8 = (*r30).clone();
            Ok(List::cons(f1(v7)?, f9(v8)?))
        }
        Adt0::C7(_, _, _) => {
            Ok(List::<Adt1>::nil())
        }
    }
}

pub fn f10(v0: Option<Bytes>, v1: List<(Bytes, Adt1)>) -> R<Adt1> {
    let v2: List<(Bytes, Adt1)> = f24(Fn3::F25(v0.clone()), List::<(Bytes, Adt1)>::nil(), v1)?;
    Ok(f6({
        let m31 = v0;
        match m31 {
            None => {
                f7(Str::lit("{"))
            }
            Some(v3) => {
                f19(v3, f7(Str::lit("{")))
            }
        }
    }, f7(Str::lit("}")), f26(Fn4::F27, List::<Adt1>::nil(), f28(v2)?)))
}

pub fn f11(p39: Adt0) -> R<Option<Bytes>> {
    let m38 = p39;
    {
        let mut m38 = m38;
        loop {
            let step = {
                let v0 = m38;
                let m32 = v0;
                match m32 {
                    Adt0::C0(_) => {
                        Err::<Adt0, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt0::C1(_, _) => {
                        Err::<Adt0, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt0::C2(_) => {
                        Err::<Adt0, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt0::C3(_) => {
                        Err::<Adt0, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt0::C4(_) => {
                        Err::<Adt0, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt0::C5 => {
                        Err::<Adt0, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt0::C6(_, _) => {
                        Err::<Adt0, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt0::C7(v9, r36, r37) => {
                        let v10 = (*r36).clone();
                        let v11 = (*r37).clone();
                        let a33 = v9;
                        let a34 = Bytes::lit(&[0x6b, 0x69, 0x6e, 0x64]);
                        if equal(a33, a34) {
                            let m35 = v10;
                            match m35 {
                                Adt0::C0(_) => {
                                    Err::<Adt0, R<Option<Bytes>>>(Ok(None::<Bytes>))
                                }
                                Adt0::C1(_, _) => {
                                    Err::<Adt0, R<Option<Bytes>>>(Ok(None::<Bytes>))
                                }
                                Adt0::C2(v12) => {
                                    if f17(v12.clone())? {
                                        Err::<Adt0, R<Option<Bytes>>>(Ok(Some(v12)))
                                    } else {
                                        Err::<Adt0, R<Option<Bytes>>>(Ok(None::<Bytes>))
                                    }
                                }
                                Adt0::C3(_) => {
                                    Err::<Adt0, R<Option<Bytes>>>(Ok(None::<Bytes>))
                                }
                                Adt0::C4(_) => {
                                    Err::<Adt0, R<Option<Bytes>>>(Ok(None::<Bytes>))
                                }
                                Adt0::C5 => {
                                    Err::<Adt0, R<Option<Bytes>>>(Ok(None::<Bytes>))
                                }
                                Adt0::C6(_, _) => {
                                    Err::<Adt0, R<Option<Bytes>>>(Ok(None::<Bytes>))
                                }
                                Adt0::C7(_, _, _) => {
                                    Err::<Adt0, R<Option<Bytes>>>(Ok(None::<Bytes>))
                                }
                            }
                        } else {
                            Ok::<Adt0, R<Option<Bytes>>>(v11)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m38 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f12(v0: Adt0) -> R<List<(Bytes, Adt1)>> {
    let m40 = v0;
    match m40 {
        Adt0::C0(_) => {
            Ok(List::<(Bytes, Adt1)>::nil())
        }
        Adt0::C1(_, _) => {
            Ok(List::<(Bytes, Adt1)>::nil())
        }
        Adt0::C2(_) => {
            Ok(List::<(Bytes, Adt1)>::nil())
        }
        Adt0::C3(_) => {
            Ok(List::<(Bytes, Adt1)>::nil())
        }
        Adt0::C4(_) => {
            Ok(List::<(Bytes, Adt1)>::nil())
        }
        Adt0::C5 => {
            Ok(List::<(Bytes, Adt1)>::nil())
        }
        Adt0::C6(_, _) => {
            Ok(List::<(Bytes, Adt1)>::nil())
        }
        Adt0::C7(v9, r41, r42) => {
            let v10 = (*r41).clone();
            let v11 = (*r42).clone();
            Ok(List::cons((v9, f1(v10)?), f12(v11)?))
        }
    }
}

pub fn f13(v0: Fn0, p47: u64, p48: Adt2) -> R<(Adt2, bool)> {
    let m46 = (p47, p48);
    {
        let mut m46 = m46;
        loop {
            let step = {
                let (v1, v2) = m46;
                let m43 = v1;
                if m43 == 0 {
                    Err::<(u64, Adt2), R<(Adt2, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m43 - 1;
                    let m45 = {
                        let c44: Fn0 = v0.clone();
                        c44.apply(v2.clone())?
                    };
                    match m45 {
                        None => {
                            Err::<(u64, Adt2), R<(Adt2, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt2), R<(Adt2, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m46 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f14(v0: u64, v1: u64, v2: Adt2) -> R<Option<Adt2>> {
    f29(v1, v0, v2)
}

pub fn f15(v0: List<Bytes>) -> Bytes {
    f30(f31(v0))
}

pub fn f16(v0: u64) -> Bytes {
    f32(f33(), 10u64, v0)
}

pub fn f17(v0: Bytes) -> R<bool> {
    let a50 = {
        let a49 = v0.clone();
        length_bytes(a49)
    };
    let a51 = 0u64;
    if nat_eq(a50, a51) {
        Ok(false)
    } else {
        if f34(f35(v0.clone(), 0u64)?) {
            let a53 = f36(v0.clone(), 1u64)?;
            let a54 = {
                let a52 = v0.clone();
                length_bytes(a52)
            };
            if nat_eq(a53, a54) {
                let a55 = f37(v0);
                Ok(bool_not(a55))
            } else {
                Ok(false)
            }
        } else {
            Ok(false)
        }
    }
}

pub fn f18(v0: Bytes) -> R<Bytes> {
    let v1: u64 = {
        let a56 = v0.clone();
        length_bytes(a56)
    };
    let v2: Adt4 = {
        let (h57, _) = f38(Fn5::F39(v1, v0), {
            let a58 = v1;
            let a59 = 1u64;
            nat_add(a58, a59)?
        }, Adt4::C0(0u64, List::<Bytes>::nil()))?;
        h57
    };
    Ok(f40(f40(f41(Str::lit("\"")), f42(match v2 {
        Adt4::C0(_, h60) => {
            h60
        }
    })), f41(Str::lit("\""))))
}

pub fn f19(v0: Bytes, v1: Bytes) -> Bytes {
    let a61 = v0;
    let a62 = v1;
    append_bytes(a61, a62)
}

pub fn f20(v0: Fn1, p66: (bool, Adt1), p67: List<Adt1>) -> (bool, Adt1) {
    let m65 = (p66, p67);
    {
        let mut m65 = m65;
        loop {
            let step = {
                let (v1, v2) = m65;
                let m63 = v2;
                match m63.uncons() {
                    None => {
                        Err::<((bool, Adt1), List<Adt1>), (bool, Adt1)>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<((bool, Adt1), List<Adt1>), (bool, Adt1)>(({
                            let c64: Fn1 = v0.clone();
                            c64.apply(v1, v3)
                        }, v4))
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

pub fn f21(v0: (bool, Adt1), v1: Adt1) -> (bool, Adt1) {
    f43(v0, v1)
}

pub fn f22(v0: Fn2, p71: List<Adt1>, p72: List<Adt1>) -> List<Adt1> {
    let m70 = (p71, p72);
    {
        let mut m70 = m70;
        loop {
            let step = {
                let (v1, v2) = m70;
                let m68 = v2;
                match m68.uncons() {
                    None => {
                        Err::<(List<Adt1>, List<Adt1>), List<Adt1>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt1>, List<Adt1>), List<Adt1>>(({
                            let c69: Fn2 = v0.clone();
                            c69.apply(v1, v3)
                        }, v4))
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

pub fn f23(v0: List<Adt1>, v1: Adt1) -> List<Adt1> {
    List::cons(v1, v0)
}

pub fn f24(v0: Fn3, p76: List<(Bytes, Adt1)>, p77: List<(Bytes, Adt1)>) -> R<List<(Bytes, Adt1)>> {
    let m75 = (p76, p77);
    {
        let mut m75 = m75;
        loop {
            let step = {
                let (v1, v2) = m75;
                let m73 = v2;
                match m73.uncons() {
                    None => {
                        Err::<(List<(Bytes, Adt1)>, List<(Bytes, Adt1)>), R<List<(Bytes, Adt1)>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Adt1)>, List<(Bytes, Adt1)>), R<List<(Bytes, Adt1)>>>(({
                            let c74: Fn3 = v0.clone();
                            c74.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m75 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f25(v0: Option<Bytes>, v1: List<(Bytes, Adt1)>, v2: (Bytes, Adt1)) -> R<List<(Bytes, Adt1)>> {
    let m78 = v0;
    match m78 {
        None => {
            Ok(List::cons((f44({
                let (h85, _) = v2.clone();
                h85
            }), f45({
                let (h86, _) = v2;
                h86
            }, {
                let (_, h87) = v2;
                h87
            })?), v1))
        }
        Some(_) => {
            let a80 = {
                let (h79, _) = v2.clone();
                h79
            };
            let a81 = Bytes::lit(&[0x6b, 0x69, 0x6e, 0x64]);
            if equal(a80, a81) {
                Ok(v1)
            } else {
                Ok(List::cons((f44({
                    let (h82, _) = v2.clone();
                    h82
                }), f45({
                    let (h83, _) = v2;
                    h83
                }, {
                    let (_, h84) = v2;
                    h84
                })?), v1))
            }
        }
    }
}

pub fn f26(v0: Fn4, p91: List<Adt1>, p92: List<(Bytes, Adt1)>) -> List<Adt1> {
    let m90 = (p91, p92);
    {
        let mut m90 = m90;
        loop {
            let step = {
                let (v1, v2) = m90;
                let m88 = v2;
                match m88.uncons() {
                    None => {
                        Err::<(List<Adt1>, List<(Bytes, Adt1)>), List<Adt1>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt1>, List<(Bytes, Adt1)>), List<Adt1>>(({
                            let c89: Fn4 = v0.clone();
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

pub fn f27(v0: List<Adt1>, v1: (Bytes, Adt1)) -> List<Adt1> {
    List::cons({
        let (_, h93) = v1;
        h93
    }, v0)
}

pub fn f28(v0: List<(Bytes, Adt1)>) -> R<List<(Bytes, Adt1)>> {
    let v1: List<List<(Bytes, Adt1)>> = f46(f47(Fn6::F48, List::<List<(Bytes, Adt1)>>::nil(), v0.clone()));
    let m98 = {
        let (h94, _) = f49(Fn7::F50, {
            let a96 = {
                let a95 = v0;
                length_list(a95)
            };
            let a97 = 1u64;
            nat_add(a96, a97)?
        }, v1)?;
        h94
    };
    match m98.uncons() {
        None => {
            Ok(List::<(Bytes, Adt1)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f29(v0: u64, v1: u64, v2: Adt2) -> R<Option<Adt2>> {
    let m100 = match v2.clone() {
        Adt2::C0(_, _, h99) => {
            h99
        }
    };
    match m100.uncons() {
        None => {
            Ok(None::<Adt2>)
        }
        Some((v3, v4)) => {
            Ok(Some(f51(v0, v1, v2, v3, v4)?))
        }
    }
}

pub fn f30(v0: List<Bytes>) -> Bytes {
    let m103 = {
        let (h101, _) = f52(Fn8::F53, {
            let a102 = v0.clone();
            length_list(a102)
        }, v0);
        h101
    };
    match m103.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f31(v0: List<Bytes>) -> List<Bytes> {
    f54(Fn9::F55, List::<Bytes>::nil(), v0)
}

pub fn f32(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let a104 = v2;
    let a105 = 0u64;
    if nat_eq(a104, a105) {
        f56(v0, 0u64)
    } else {
        let (_, h106) = {
            let (h107, _) = f57(Fn10::F58(v1, v0), v2, (v2, Bytes::lit(&[])));
            h107
        };
        h106
    }
}

pub fn f33() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f34(v0: u64) -> bool {
    let a108 = 64u64;
    let a109 = v0;
    if if nat_lt(a108, a109) {
        let a110 = v0;
        let a111 = 91u64;
        nat_lt(a110, a111)
    } else {
        false
    } {
        true
    } else {
        let a112 = 96u64;
        let a113 = v0;
        if if nat_lt(a112, a113) {
            let a114 = v0;
            let a115 = 123u64;
            nat_lt(a114, a115)
        } else {
            false
        } {
            true
        } else {
            let a116 = v0;
            let a117 = 95u64;
            nat_eq(a116, a117)
        }
    }
}

pub fn f35(v0: Bytes, v1: u64) -> R<u64> {
    let m118 = f59(v0, v1);
    match m118 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a163 = {
                let a139 = {
                    let a127 = {
                        let a121 = {
                            let a119 = v3;
                            let a120 = 128u8;
                            fixed_u8::bit_and(a119, a120)
                        };
                        let a122 = 128u8;
                        if equal(a121, a122) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a128 = {
                        let a125 = {
                            let a123 = v3;
                            let a124 = 64u8;
                            fixed_u8::bit_and(a123, a124)
                        };
                        let a126 = 64u8;
                        if equal(a125, a126) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a127, a128)?
                };
                let a140 = {
                    let a137 = {
                        let a131 = {
                            let a129 = v3;
                            let a130 = 32u8;
                            fixed_u8::bit_and(a129, a130)
                        };
                        let a132 = 32u8;
                        if equal(a131, a132) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a138 = {
                        let a135 = {
                            let a133 = v3;
                            let a134 = 16u8;
                            fixed_u8::bit_and(a133, a134)
                        };
                        let a136 = 16u8;
                        if equal(a135, a136) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a137, a138)?
                };
                nat_add(a139, a140)?
            };
            let a164 = {
                let a161 = {
                    let a149 = {
                        let a143 = {
                            let a141 = v3;
                            let a142 = 8u8;
                            fixed_u8::bit_and(a141, a142)
                        };
                        let a144 = 8u8;
                        if equal(a143, a144) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a150 = {
                        let a147 = {
                            let a145 = v3;
                            let a146 = 4u8;
                            fixed_u8::bit_and(a145, a146)
                        };
                        let a148 = 4u8;
                        if equal(a147, a148) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a149, a150)?
                };
                let a162 = {
                    let a159 = {
                        let a153 = {
                            let a151 = v3;
                            let a152 = 2u8;
                            fixed_u8::bit_and(a151, a152)
                        };
                        let a154 = 2u8;
                        if equal(a153, a154) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a160 = {
                        let a157 = {
                            let a155 = v3;
                            let a156 = 1u8;
                            fixed_u8::bit_and(a155, a156)
                        };
                        let a158 = 1u8;
                        if equal(a157, a158) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a159, a160)?
                };
                nat_add(a161, a162)?
            };
            nat_add(a163, a164)
        }
    }
}

pub fn f36(v0: Bytes, v1: u64) -> R<u64> {
    let (h165, _) = f60(Fn11::F61(v0.clone()), {
        let a167 = {
            let a166 = v0;
            length_bytes(a166)
        };
        let a168 = 1u64;
        nat_add(a167, a168)?
    }, v1)?;
    Ok(h165)
}

pub fn f37(v0: Bytes) -> bool {
    let a169 = v0.clone();
    let a170 = Bytes::lit(&[0x74, 0x72, 0x75, 0x65]);
    if equal(a169, a170) {
        true
    } else {
        let a171 = v0;
        let a172 = Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]);
        equal(a171, a172)
    }
}

pub fn f38(v0: Fn5, p177: u64, p178: Adt4) -> R<(Adt4, bool)> {
    let m176 = (p177, p178);
    {
        let mut m176 = m176;
        loop {
            let step = {
                let (v1, v2) = m176;
                let m173 = v1;
                if m173 == 0 {
                    Err::<(u64, Adt4), R<(Adt4, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m173 - 1;
                    let m175 = {
                        let c174: Fn5 = v0.clone();
                        c174.apply(v2.clone())?
                    };
                    match m175 {
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
                Ok(next) => { m176 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f39(v0: u64, v1: Bytes, v2: Adt4) -> R<Option<Adt4>> {
    f62(v1, v0, v2)
}

pub fn f40(v0: Bytes, v1: Bytes) -> Bytes {
    let a179 = v0;
    let a180 = v1;
    append_bytes(a179, a180)
}

pub fn f41(v0: Str) -> Bytes {
    let a181 = v0;
    utf8_encode(a181)
}

pub fn f42(v0: List<Bytes>) -> Bytes {
    f30(f31(v0))
}

pub fn f43(v0: (bool, Adt1), v1: Adt1) -> (bool, Adt1) {
    let (h182, _) = v0;
    if h182 {
        (false, Adt1::C2(std::rc::Rc::new(v1), std::rc::Rc::new(Adt1::C0)))
    } else {
        (false, Adt1::C2(std::rc::Rc::new(v1), std::rc::Rc::new(Adt1::C2(std::rc::Rc::new(Adt1::C1(f7(Str::lit(",")))), std::rc::Rc::new(Adt1::C2(std::rc::Rc::new(Adt1::C4), std::rc::Rc::new({
            let (_, h183) = v0;
            h183
        })))))))
    }
}

pub fn f44(v0: Bytes) -> Bytes {
    f19(f63(f64(v0.clone())), v0)
}

pub fn f45(v0: Bytes, v1: Adt1) -> R<Adt1> {
    Ok(Adt1::C2(std::rc::Rc::new(Adt1::C1(f19(f5(v0)?, f7(Str::lit(": "))))), std::rc::Rc::new(Adt1::C2(std::rc::Rc::new(v1), std::rc::Rc::new(Adt1::C0)))))
}

pub fn f46(v0: List<List<(Bytes, Adt1)>>) -> List<List<(Bytes, Adt1)>> {
    f65(Fn12::F66, List::<List<(Bytes, Adt1)>>::nil(), v0)
}

pub fn f47(v0: Fn6, p187: List<List<(Bytes, Adt1)>>, p188: List<(Bytes, Adt1)>) -> List<List<(Bytes, Adt1)>> {
    let m186 = (p187, p188);
    {
        let mut m186 = m186;
        loop {
            let step = {
                let (v1, v2) = m186;
                let m184 = v2;
                match m184.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Adt1)>>, List<(Bytes, Adt1)>), List<List<(Bytes, Adt1)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Adt1)>>, List<(Bytes, Adt1)>), List<List<(Bytes, Adt1)>>>(({
                            let c185: Fn6 = v0.clone();
                            c185.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m186 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f48(v0: List<List<(Bytes, Adt1)>>, v1: (Bytes, Adt1)) -> List<List<(Bytes, Adt1)>> {
    List::cons(List::cons(v1, List::<(Bytes, Adt1)>::nil()), v0)
}

pub fn f49(v0: Fn7, p193: u64, p194: List<List<(Bytes, Adt1)>>) -> R<(List<List<(Bytes, Adt1)>>, bool)> {
    let m192 = (p193, p194);
    {
        let mut m192 = m192;
        loop {
            let step = {
                let (v1, v2) = m192;
                let m189 = v1;
                if m189 == 0 {
                    Err::<(u64, List<List<(Bytes, Adt1)>>), R<(List<List<(Bytes, Adt1)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m189 - 1;
                    let m191 = {
                        let c190: Fn7 = v0.clone();
                        c190.apply(v2.clone())?
                    };
                    match m191 {
                        None => {
                            Err::<(u64, List<List<(Bytes, Adt1)>>), R<(List<List<(Bytes, Adt1)>>, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, List<List<(Bytes, Adt1)>>), R<(List<List<(Bytes, Adt1)>>, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m192 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f50(v0: List<List<(Bytes, Adt1)>>) -> R<Option<List<List<(Bytes, Adt1)>>>> {
    f67(v0)
}

pub fn f51(v0: u64, v1: u64, v2: Adt2, v3: Adt3, v4: List<Adt3>) -> R<Adt2> {
    let v5: u64 = match v3.clone() {
        Adt3::C0(h195, _, _) => {
            h195
        }
    };
    let v6: bool = match v3.clone() {
        Adt3::C0(_, h196, _) => {
            h196
        }
    };
    let m198 = match v3 {
        Adt3::C0(_, _, h197) => {
            h197
        }
    };
    match m198 {
        Adt1::C0 => {
            Ok(Adt2::C0(match v2.clone() {
                Adt2::C0(h199, _, _) => {
                    h199
                }
            }, match v2 {
                Adt2::C0(_, h200, _) => {
                    h200
                }
            }, v4))
        }
        Adt1::C1(v7) => {
            Ok(Adt2::C0(List::cons(v7.clone(), match v2.clone() {
                Adt2::C0(h201, _, _) => {
                    h201
                }
            }), {
                let a203 = match v2 {
                    Adt2::C0(_, h202, _) => {
                        h202
                    }
                };
                let a204 = f68(v7)?;
                nat_add(a203, a204)?
            }, v4))
        }
        Adt1::C2(r221, r222) => {
            let v8 = (*r221).clone();
            let v9 = (*r222).clone();
            Ok(Adt2::C0(match v2.clone() {
                Adt2::C0(h205, _, _) => {
                    h205
                }
            }, match v2 {
                Adt2::C0(_, h206, _) => {
                    h206
                }
            }, List::cons(Adt3::C0(v5, v6, v8), List::cons(Adt3::C0(v5, v6, v9), v4))))
        }
        Adt1::C3(v10, r223) => {
            let v11 = (*r223).clone();
            Ok(Adt2::C0(match v2.clone() {
                Adt2::C0(h207, _, _) => {
                    h207
                }
            }, match v2 {
                Adt2::C0(_, h208, _) => {
                    h208
                }
            }, List::cons(Adt3::C0({
                let a209 = v5;
                let a210 = v10;
                nat_add(a209, a210)?
            }, v6, v11), v4)))
        }
        Adt1::C4 => {
            if v6 {
                Ok(Adt2::C0(List::cons({
                    let a211 = Str::lit(" ");
                    utf8_encode(a211)
                }, match v2.clone() {
                    Adt2::C0(h212, _, _) => {
                        h212
                    }
                }), {
                    let a214 = match v2 {
                        Adt2::C0(_, h213, _) => {
                            h213
                        }
                    };
                    let a215 = 1u64;
                    nat_add(a214, a215)?
                }, v4))
            } else {
                f69(v2, v5, v4)
            }
        }
        Adt1::C5 => {
            if v6 {
                Ok(Adt2::C0(match v2.clone() {
                    Adt2::C0(h216, _, _) => {
                        h216
                    }
                }, match v2 {
                    Adt2::C0(_, h217, _) => {
                        h217
                    }
                }, v4))
            } else {
                f69(v2, v5, v4)
            }
        }
        Adt1::C6(r224) => {
            let v12 = (*r224).clone();
            let v13: bool = f70(v1, v0, match v2.clone() {
                Adt2::C0(_, h218, _) => {
                    h218
                }
            }, v4.clone(), Adt3::C0(v5, true, v12.clone()))?;
            Ok(Adt2::C0(match v2.clone() {
                Adt2::C0(h219, _, _) => {
                    h219
                }
            }, match v2 {
                Adt2::C0(_, h220, _) => {
                    h220
                }
            }, List::cons(Adt3::C0(v5, v13, v12), v4)))
        }
    }
}

pub fn f52(v0: Fn8, p229: u64, p230: List<Bytes>) -> (List<Bytes>, bool) {
    let m228 = (p229, p230);
    {
        let mut m228 = m228;
        loop {
            let step = {
                let (v1, v2) = m228;
                let m225 = v1;
                if m225 == 0 {
                    Err::<(u64, List<Bytes>), (List<Bytes>, bool)>((v2, false))
                } else {
                    let v3 = m225 - 1;
                    let m227 = {
                        let c226: Fn8 = v0.clone();
                        c226.apply(v2.clone())
                    };
                    match m227 {
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
                Ok(next) => { m228 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f53(v0: List<Bytes>) -> Option<List<Bytes>> {
    f71(v0)
}

pub fn f54(v0: Fn9, p234: List<Bytes>, p235: List<Bytes>) -> List<Bytes> {
    let m233 = (p234, p235);
    {
        let mut m233 = m233;
        loop {
            let step = {
                let (v1, v2) = m233;
                let m231 = v2;
                match m231.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c232: Fn9 = v0.clone();
                            c232.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m233 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f55(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f56(v0: Bytes, v1: u64) -> Bytes {
    let m239 = {
        let a236 = v0;
        let a237 = v1;
        let a238 = 1u64;
        slice_bytes(a236, a237, a238)
    };
    match m239 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f57(v0: Fn10, p244: u64, p245: (u64, Bytes)) -> ((u64, Bytes), bool) {
    let m243 = (p244, p245);
    {
        let mut m243 = m243;
        loop {
            let step = {
                let (v1, v2) = m243;
                let m240 = v1;
                if m240 == 0 {
                    Err::<(u64, (u64, Bytes)), ((u64, Bytes), bool)>((v2, false))
                } else {
                    let v3 = m240 - 1;
                    let m242 = {
                        let c241: Fn10 = v0.clone();
                        c241.apply(v2.clone())
                    };
                    match m242 {
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
                Ok(next) => { m243 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f58(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f72(v1, v0, v2)
}

pub fn f59(v0: Bytes, v1: u64) -> Option<u8> {
    let a246 = v0;
    let a247 = v1;
    index_bytes(a246, a247)
}

pub fn f60(v0: Fn11, p252: u64, p253: u64) -> R<(u64, bool)> {
    let m251 = (p252, p253);
    {
        let mut m251 = m251;
        loop {
            let step = {
                let (v1, v2) = m251;
                let m248 = v1;
                if m248 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m248 - 1;
                    let m250 = {
                        let c249: Fn11 = v0.clone();
                        c249.apply(v2)?
                    };
                    match m250 {
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
                Ok(next) => { m251 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f61(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f73(v0, v1)
}

pub fn f62(v0: Bytes, v1: u64, v2: Adt4) -> R<Option<Adt4>> {
    let v3: u64 = match v2.clone() {
        Adt4::C0(h254, _) => {
            h254
        }
    };
    let a255 = v3;
    let a256 = v1;
    if nat_lt(a255, a256) {
        let v4: u64 = f74(v0.clone(), v3)?;
        let a257 = v4;
        let a258 = 32u64;
        if if nat_lt(a257, a258) {
            true
        } else {
            let a259 = v4;
            let a260 = 34u64;
            if nat_eq(a259, a260) {
                true
            } else {
                let a261 = v4;
                let a262 = 92u64;
                nat_eq(a261, a262)
            }
        } {
            Ok(Some(Adt4::C0({
                let a263 = v3;
                let a264 = 1u64;
                nat_add(a263, a264)?
            }, List::cons(f75(v4), match v2 {
                Adt4::C0(_, h265) => {
                    h265
                }
            }))))
        } else {
            let v5: u64 = f76(v0.clone(), v1, v3)?;
            Ok(Some(Adt4::C0(v5, List::cons(f77(v0, v3, {
                let a266 = v5;
                let a267 = v3;
                nat_sub(a266, a267)
            }), match v2 {
                Adt4::C0(_, h268) => {
                    h268
                }
            }))))
        }
    } else {
        Ok(None::<Adt4>)
    }
}

pub fn f63(v0: u64) -> Bytes {
    f77(Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff]), v0, 1u64)
}

pub fn f64(v0: Bytes) -> u64 {
    let a269 = v0.clone();
    let a270 = Bytes::lit(&[0x6e, 0x61, 0x6d, 0x65]);
    if equal(a269, a270) {
        0u64
    } else {
        let a271 = v0.clone();
        let a272 = Bytes::lit(&[0x74, 0x79, 0x70, 0x65, 0x5f, 0x70, 0x61, 0x72, 0x61, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x73]);
        if equal(a271, a272) {
            1u64
        } else {
            let a273 = v0.clone();
            let a274 = Bytes::lit(&[0x70, 0x61, 0x72, 0x61, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x73]);
            if equal(a273, a274) {
                2u64
            } else {
                let a275 = v0.clone();
                let a276 = Bytes::lit(&[0x74, 0x79, 0x70, 0x65]);
                if equal(a275, a276) {
                    3u64
                } else {
                    let a277 = v0.clone();
                    let a278 = Bytes::lit(&[0x72, 0x65, 0x73, 0x75, 0x6c, 0x74]);
                    if equal(a277, a278) {
                        4u64
                    } else {
                        let a279 = v0.clone();
                        let a280 = Bytes::lit(&[0x73, 0x74, 0x61, 0x74, 0x65, 0x6d, 0x65, 0x6e, 0x74]);
                        if equal(a279, a280) {
                            5u64
                        } else {
                            let a281 = v0.clone();
                            let a282 = Bytes::lit(&[0x66, 0x69, 0x65, 0x6c, 0x64, 0x73]);
                            if equal(a281, a282) {
                                6u64
                            } else {
                                let a283 = v0.clone();
                                let a284 = Bytes::lit(&[0x63, 0x6f, 0x6e, 0x73, 0x74, 0x72, 0x75, 0x63, 0x74, 0x6f, 0x72, 0x73]);
                                if equal(a283, a284) {
                                    7u64
                                } else {
                                    let a285 = v0.clone();
                                    let a286 = Bytes::lit(&[0x61, 0x72, 0x67, 0x75, 0x6d, 0x65, 0x6e, 0x74, 0x73]);
                                    if equal(a285, a286) {
                                        8u64
                                    } else {
                                        let a287 = v0.clone();
                                        let a288 = Bytes::lit(&[0x62, 0x6f, 0x64, 0x79]);
                                        if equal(a287, a288) {
                                            9u64
                                        } else {
                                            let a289 = v0.clone();
                                            let a290 = Bytes::lit(&[0x70, 0x72, 0x6f, 0x6f, 0x66]);
                                            if equal(a289, a290) {
                                                10u64
                                            } else {
                                                let a291 = v0;
                                                let a292 = Bytes::lit(&[0x61, 0x78, 0x69, 0x6f, 0x6d, 0x73]);
                                                if equal(a291, a292) {
                                                    11u64
                                                } else {
                                                    12u64
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

pub fn f65(v0: Fn12, p296: List<List<(Bytes, Adt1)>>, p297: List<List<(Bytes, Adt1)>>) -> List<List<(Bytes, Adt1)>> {
    let m295 = (p296, p297);
    {
        let mut m295 = m295;
        loop {
            let step = {
                let (v1, v2) = m295;
                let m293 = v2;
                match m293.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>), List<List<(Bytes, Adt1)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>), List<List<(Bytes, Adt1)>>>(({
                            let c294: Fn12 = v0.clone();
                            c294.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m295 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f66(v0: List<List<(Bytes, Adt1)>>, v1: List<(Bytes, Adt1)>) -> List<List<(Bytes, Adt1)>> {
    List::cons(v1, v0)
}

pub fn f67(v0: List<List<(Bytes, Adt1)>>) -> R<Option<List<List<(Bytes, Adt1)>>>> {
    let a299 = {
        let a298 = v0.clone();
        length_list(a298)
    };
    let a300 = 1u64;
    if nat_le(a299, a300) {
        Ok(None::<List<List<(Bytes, Adt1)>>>)
    } else {
        Ok(Some(f78(v0)?))
    }
}

pub fn f68(v0: Bytes) -> R<u64> {
    let (_, h301) = {
        let (h302, _) = f79(Fn13::F80(v0.clone()), {
            let a303 = v0;
            length_bytes(a303)
        }, (0u64, 0u64))?;
        h302
    };
    Ok(h301)
}

pub fn f69(v0: Adt2, v1: u64, v2: List<Adt3>) -> R<Adt2> {
    Ok(Adt2::C0(f81(match v0 {
        Adt2::C0(h304, _, _) => {
            h304
        }
    }, v1)?, v1, v2))
}

pub fn f70(v0: u64, v1: u64, v2: u64, v3: List<Adt3>, v4: Adt3) -> R<bool> {
    let v5: Adt5 = {
        let (h305, _) = f82(Fn14::F83, {
            let a306 = v0;
            let a307 = 1u64;
            nat_add(a306, a307)?
        }, Adt5::C0(0u64, {
            let a308 = v1;
            let a309 = v2;
            nat_sub(a308, a309)
        }, {
            let a310 = v1;
            let a311 = v2;
            nat_lt(a310, a311)
        }, List::cons(v4, List::<Adt3>::nil()), v3, None::<bool>))?;
        h305
    };
    let m313 = match v5 {
        Adt5::C0(_, _, _, _, _, h312) => {
            h312
        }
    };
    match m313 {
        None => {
            Ok(false)
        }
        Some(v6) => {
            Ok(v6)
        }
    }
}

pub fn f71(v0: List<Bytes>) -> Option<List<Bytes>> {
    let m314 = v0.clone();
    match m314.uncons() {
        None => {
            None::<List<Bytes>>
        }
        Some((_, v2)) => {
            let m315 = v2;
            match m315.uncons() {
                None => {
                    None::<List<Bytes>>
                }
                Some((_, _)) => {
                    let v5: (Option<Bytes>, List<Bytes>) = f84(Fn15::F85, (None::<Bytes>, List::<Bytes>::nil()), v0);
                    let m317 = {
                        let (h316, _) = v5;
                        h316
                    };
                    match m317 {
                        None => {
                            Some(f31({
                                let (_, h319) = v5;
                                h319
                            }))
                        }
                        Some(v6) => {
                            Some(f31(List::cons(v6, {
                                let (_, h318) = v5;
                                h318
                            })))
                        }
                    }
                }
            }
        }
    }
}

pub fn f72(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h320, _) = v2;
        h320
    };
    let a321 = v3;
    let a322 = 0u64;
    if nat_eq(a321, a322) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a323 = v3;
            let a324 = v1;
            let a325 = 0u64;
            nat_quot(a323, a324, a325)
        }, {
            let a330 = f56(v0, {
                let a326 = v3;
                let a327 = v1;
                let a328 = 0u64;
                nat_rem(a326, a327, a328)
            });
            let a331 = {
                let (_, h329) = v2;
                h329
            };
            append_bytes(a330, a331)
        }))
    }
}

pub fn f73(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f86(f35(v0, v1)?) {
        Ok(Some({
            let a332 = v1;
            let a333 = 1u64;
            nat_add(a332, a333)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f74(v0: Bytes, v1: u64) -> R<u64> {
    let m334 = f59(v0, v1);
    match m334 {
        None => {
            Ok(0u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a379 = {
                let a355 = {
                    let a343 = {
                        let a337 = {
                            let a335 = v3;
                            let a336 = 128u8;
                            fixed_u8::bit_and(a335, a336)
                        };
                        let a338 = 128u8;
                        if equal(a337, a338) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a344 = {
                        let a341 = {
                            let a339 = v3;
                            let a340 = 64u8;
                            fixed_u8::bit_and(a339, a340)
                        };
                        let a342 = 64u8;
                        if equal(a341, a342) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a343, a344)?
                };
                let a356 = {
                    let a353 = {
                        let a347 = {
                            let a345 = v3;
                            let a346 = 32u8;
                            fixed_u8::bit_and(a345, a346)
                        };
                        let a348 = 32u8;
                        if equal(a347, a348) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a354 = {
                        let a351 = {
                            let a349 = v3;
                            let a350 = 16u8;
                            fixed_u8::bit_and(a349, a350)
                        };
                        let a352 = 16u8;
                        if equal(a351, a352) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a353, a354)?
                };
                nat_add(a355, a356)?
            };
            let a380 = {
                let a377 = {
                    let a365 = {
                        let a359 = {
                            let a357 = v3;
                            let a358 = 8u8;
                            fixed_u8::bit_and(a357, a358)
                        };
                        let a360 = 8u8;
                        if equal(a359, a360) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a366 = {
                        let a363 = {
                            let a361 = v3;
                            let a362 = 4u8;
                            fixed_u8::bit_and(a361, a362)
                        };
                        let a364 = 4u8;
                        if equal(a363, a364) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a365, a366)?
                };
                let a378 = {
                    let a375 = {
                        let a369 = {
                            let a367 = v3;
                            let a368 = 2u8;
                            fixed_u8::bit_and(a367, a368)
                        };
                        let a370 = 2u8;
                        if equal(a369, a370) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a376 = {
                        let a373 = {
                            let a371 = v3;
                            let a372 = 1u8;
                            fixed_u8::bit_and(a371, a372)
                        };
                        let a374 = 1u8;
                        if equal(a373, a374) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a375, a376)?
                };
                nat_add(a377, a378)?
            };
            nat_add(a379, a380)
        }
    }
}

pub fn f75(v0: u64) -> Bytes {
    let a381 = v0;
    let a382 = 34u64;
    if nat_eq(a381, a382) {
        f41(Str::lit("\\\""))
    } else {
        let a383 = v0;
        let a384 = 92u64;
        if nat_eq(a383, a384) {
            f41(Str::lit("\\\\"))
        } else {
            let a385 = v0;
            let a386 = 8u64;
            if nat_eq(a385, a386) {
                f41(Str::lit("\\b"))
            } else {
                let a387 = v0;
                let a388 = 12u64;
                if nat_eq(a387, a388) {
                    f41(Str::lit("\\f"))
                } else {
                    let a389 = v0;
                    let a390 = 10u64;
                    if nat_eq(a389, a390) {
                        f41(Str::lit("\\n"))
                    } else {
                        let a391 = v0;
                        let a392 = 13u64;
                        if nat_eq(a391, a392) {
                            f41(Str::lit("\\r"))
                        } else {
                            let a393 = v0;
                            let a394 = 9u64;
                            if nat_eq(a393, a394) {
                                f41(Str::lit("\\t"))
                            } else {
                                f40(f41(Str::lit("\\u00")), f87(v0))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f76(v0: Bytes, v1: u64, v2: u64) -> R<u64> {
    let (h395, _) = f60(Fn11::F88(v1, v0), {
        let a396 = v1;
        let a397 = v2;
        nat_sub(a396, a397)
    }, v2)?;
    Ok(h395)
}

pub fn f77(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m401 = {
        let a398 = v0;
        let a399 = v1;
        let a400 = v2;
        slice_bytes(a398, a399, a400)
    };
    match m401 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f78(v0: List<List<(Bytes, Adt1)>>) -> R<List<List<(Bytes, Adt1)>>> {
    Ok(f46({
        let (_, h402) = {
            let (h403, _) = f89(Fn16::F90, {
                let a405 = {
                    let a404 = v0.clone();
                    length_list(a404)
                };
                let a406 = 1u64;
                nat_add(a405, a406)?
            }, (v0, List::<List<(Bytes, Adt1)>>::nil()))?;
            h403
        };
        h402
    }))
}

pub fn f79(v0: Fn13, p411: u64, p412: (u64, u64)) -> R<((u64, u64), bool)> {
    let m410 = (p411, p412);
    {
        let mut m410 = m410;
        loop {
            let step = {
                let (v1, v2) = m410;
                let m407 = v1;
                if m407 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m407 - 1;
                    let m409 = {
                        let c408: Fn13 = v0.clone();
                        c408.apply(v2)?
                    };
                    match m409 {
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
                Ok(next) => { m410 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f80(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f91(v0, v1)
}

pub fn f81(v0: List<Bytes>, v1: u64) -> R<List<Bytes>> {
    let v2: u64 = {
        let a413 = v1;
        let a414 = 256u64;
        let a415 = 0u64;
        nat_quot(a413, a414, a415)
    };
    let v3: List<Bytes> = {
        let (_, h416) = {
            let (h417, _) = f92(Fn17::F93(v2), {
                let a418 = v2;
                let a419 = 1u64;
                nat_add(a418, a419)?
            }, (0u64, List::cons({
                let a420 = Str::lit("\u{a}");
                utf8_encode(a420)
            }, v0)))?;
            h417
        };
        h416
    };
    Ok(List::cons(f77(Bytes::lit(&[0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20]), 0u64, {
        let a421 = v1;
        let a422 = 256u64;
        let a423 = 0u64;
        nat_rem(a421, a422, a423)
    }), v3))
}

pub fn f82(v0: Fn14, p428: u64, p429: Adt5) -> R<(Adt5, bool)> {
    let m427 = (p428, p429);
    {
        let mut m427 = m427;
        loop {
            let step = {
                let (v1, v2) = m427;
                let m424 = v1;
                if m424 == 0 {
                    Err::<(u64, Adt5), R<(Adt5, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m424 - 1;
                    let m426 = {
                        let c425: Fn14 = v0.clone();
                        c425.apply(v2.clone())?
                    };
                    match m426 {
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
                Ok(next) => { m427 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f83(v0: Adt5) -> R<Option<Adt5>> {
    f94(v0)
}

pub fn f84(v0: Fn15, p433: (Option<Bytes>, List<Bytes>), p434: List<Bytes>) -> (Option<Bytes>, List<Bytes>) {
    let m432 = (p433, p434);
    {
        let mut m432 = m432;
        loop {
            let step = {
                let (v1, v2) = m432;
                let m430 = v2;
                match m430.uncons() {
                    None => {
                        Err::<((Option<Bytes>, List<Bytes>), List<Bytes>), (Option<Bytes>, List<Bytes>)>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<((Option<Bytes>, List<Bytes>), List<Bytes>), (Option<Bytes>, List<Bytes>)>(({
                            let c431: Fn15 = v0.clone();
                            c431.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m432 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f85(v0: (Option<Bytes>, List<Bytes>), v1: Bytes) -> (Option<Bytes>, List<Bytes>) {
    f95(v0, v1)
}

pub fn f86(v0: u64) -> bool {
    if f34(v0) {
        true
    } else {
        let a435 = 47u64;
        let a436 = v0;
        if if nat_lt(a435, a436) {
            let a437 = v0;
            let a438 = 58u64;
            nat_lt(a437, a438)
        } else {
            false
        } {
            true
        } else {
            let a439 = v0;
            let a440 = 46u64;
            if nat_eq(a439, a440) {
                true
            } else {
                let a441 = v0;
                let a442 = 39u64;
                if nat_eq(a441, a442) {
                    true
                } else {
                    let a443 = v0;
                    let a444 = 47u64;
                    if nat_eq(a443, a444) {
                        true
                    } else {
                        let a445 = v0;
                        let a446 = 45u64;
                        nat_eq(a445, a446)
                    }
                }
            }
        }
    }
}

pub fn f87(v0: u64) -> Bytes {
    f40(f96(v0, 16u64), f97(v0))
}

pub fn f88(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f98(v1, v0, v2)
}

pub fn f89(v0: Fn16, p451: u64, p452: (List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>)) -> R<((List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>), bool)> {
    let m450 = (p451, p452);
    {
        let mut m450 = m450;
        loop {
            let step = {
                let (v1, v2) = m450;
                let m447 = v1;
                if m447 == 0 {
                    Err::<(u64, (List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>)), R<((List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m447 - 1;
                    let m449 = {
                        let c448: Fn16 = v0.clone();
                        c448.apply(v2.clone())?
                    };
                    match m449 {
                        None => {
                            Err::<(u64, (List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>)), R<((List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>)), R<((List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m450 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f90(v0: (List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>)) -> R<Option<(List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>)>> {
    f99(v0)
}

pub fn f91(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m454 = f59(v0, {
        let (h453, _) = v1;
        h453
    });
    match m454 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            let v4: u64 = {
                let v3: u8 = v2;
                let a499 = {
                    let a475 = {
                        let a463 = {
                            let a457 = {
                                let a455 = v3;
                                let a456 = 128u8;
                                fixed_u8::bit_and(a455, a456)
                            };
                            let a458 = 128u8;
                            if equal(a457, a458) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a464 = {
                            let a461 = {
                                let a459 = v3;
                                let a460 = 64u8;
                                fixed_u8::bit_and(a459, a460)
                            };
                            let a462 = 64u8;
                            if equal(a461, a462) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a463, a464)?
                    };
                    let a476 = {
                        let a473 = {
                            let a467 = {
                                let a465 = v3;
                                let a466 = 32u8;
                                fixed_u8::bit_and(a465, a466)
                            };
                            let a468 = 32u8;
                            if equal(a467, a468) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a474 = {
                            let a471 = {
                                let a469 = v3;
                                let a470 = 16u8;
                                fixed_u8::bit_and(a469, a470)
                            };
                            let a472 = 16u8;
                            if equal(a471, a472) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a473, a474)?
                    };
                    nat_add(a475, a476)?
                };
                let a500 = {
                    let a497 = {
                        let a485 = {
                            let a479 = {
                                let a477 = v3;
                                let a478 = 8u8;
                                fixed_u8::bit_and(a477, a478)
                            };
                            let a480 = 8u8;
                            if equal(a479, a480) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a486 = {
                            let a483 = {
                                let a481 = v3;
                                let a482 = 4u8;
                                fixed_u8::bit_and(a481, a482)
                            };
                            let a484 = 4u8;
                            if equal(a483, a484) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a485, a486)?
                    };
                    let a498 = {
                        let a495 = {
                            let a489 = {
                                let a487 = v3;
                                let a488 = 2u8;
                                fixed_u8::bit_and(a487, a488)
                            };
                            let a490 = 2u8;
                            if equal(a489, a490) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a496 = {
                            let a493 = {
                                let a491 = v3;
                                let a492 = 1u8;
                                fixed_u8::bit_and(a491, a492)
                            };
                            let a494 = 1u8;
                            if equal(a493, a494) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a495, a496)?
                    };
                    nat_add(a497, a498)?
                };
                nat_add(a499, a500)?
            };
            let a501 = v4;
            let a502 = 128u64;
            if nat_lt(a501, a502) {
                Ok(Some(({
                    let a504 = {
                        let (h503, _) = v1;
                        h503
                    };
                    let a505 = 1u64;
                    nat_add(a504, a505)?
                }, {
                    let a507 = {
                        let (_, h506) = v1;
                        h506
                    };
                    let a508 = 1u64;
                    nat_add(a507, a508)?
                })))
            } else {
                let a509 = v4;
                let a510 = 192u64;
                if nat_lt(a509, a510) {
                    Ok(Some(({
                        let a512 = {
                            let (h511, _) = v1;
                            h511
                        };
                        let a513 = 1u64;
                        nat_add(a512, a513)?
                    }, {
                        let (_, h514) = v1;
                        h514
                    })))
                } else {
                    Ok(Some(({
                        let a516 = {
                            let (h515, _) = v1;
                            h515
                        };
                        let a517 = 1u64;
                        nat_add(a516, a517)?
                    }, {
                        let a519 = {
                            let (_, h518) = v1;
                            h518
                        };
                        let a520 = 1u64;
                        nat_add(a519, a520)?
                    })))
                }
            }
        }
    }
}

pub fn f92(v0: Fn17, p525: u64, p526: (u64, List<Bytes>)) -> R<((u64, List<Bytes>), bool)> {
    let m524 = (p525, p526);
    {
        let mut m524 = m524;
        loop {
            let step = {
                let (v1, v2) = m524;
                let m521 = v1;
                if m521 == 0 {
                    Err::<(u64, (u64, List<Bytes>)), R<((u64, List<Bytes>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m521 - 1;
                    let m523 = {
                        let c522: Fn17 = v0.clone();
                        c522.apply(v2.clone())?
                    };
                    match m523 {
                        None => {
                            Err::<(u64, (u64, List<Bytes>)), R<((u64, List<Bytes>), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (u64, List<Bytes>)), R<((u64, List<Bytes>), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m524 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f93(v0: u64, v1: (u64, List<Bytes>)) -> R<Option<(u64, List<Bytes>)>> {
    f100(v0, v1)
}

pub fn f94(v0: Adt5) -> R<Option<Adt5>> {
    let m528 = match v0.clone() {
        Adt5::C0(_, _, _, _, _, h527) => {
            h527
        }
    };
    match m528 {
        None => {
            if if match v0.clone() {
                Adt5::C0(_, _, h529, _, _, _) => {
                    h529
                }
            } {
                true
            } else {
                let a532 = match v0.clone() {
                    Adt5::C0(_, h530, _, _, _, _) => {
                        h530
                    }
                };
                let a533 = match v0.clone() {
                    Adt5::C0(h531, _, _, _, _, _) => {
                        h531
                    }
                };
                nat_lt(a532, a533)
            } {
                Ok(Some(f101(v0.clone(), match v0.clone() {
                    Adt5::C0(h534, _, _, _, _, _) => {
                        h534
                    }
                }, match v0.clone() {
                    Adt5::C0(_, _, _, h535, _, _) => {
                        h535
                    }
                }, match v0 {
                    Adt5::C0(_, _, _, _, h536, _) => {
                        h536
                    }
                }, Some(false))))
            } else {
                let m538 = match v0.clone() {
                    Adt5::C0(_, _, _, h537, _, _) => {
                        h537
                    }
                };
                match m538.uncons() {
                    None => {
                        let m541 = match v0.clone() {
                            Adt5::C0(_, _, _, _, h540, _) => {
                                h540
                            }
                        };
                        match m541.uncons() {
                            None => {
                                Ok(Some(f101(v0.clone(), match v0 {
                                    Adt5::C0(h542, _, _, _, _, _) => {
                                        h542
                                    }
                                }, List::<Adt3>::nil(), List::<Adt3>::nil(), Some(true))))
                            }
                            Some((v4, v5)) => {
                                Ok(Some(f102(v0, v4, List::<Adt3>::nil(), v5)?))
                            }
                        }
                    }
                    Some((v2, v3)) => {
                        Ok(Some(f102(v0.clone(), v2, v3, match v0 {
                            Adt5::C0(_, _, _, _, h539, _) => {
                                h539
                            }
                        })?))
                    }
                }
            }
        }
        Some(_) => {
            Ok(None::<Adt5>)
        }
    }
}

pub fn f95(v0: (Option<Bytes>, List<Bytes>), v1: Bytes) -> (Option<Bytes>, List<Bytes>) {
    let m544 = {
        let (h543, _) = v0;
        h543
    };
    match m544 {
        None => {
            (Some(v1), {
                let (_, h545) = v0;
                h545
            })
        }
        Some(v2) => {
            (None::<Bytes>, List::cons({
                let a546 = v2;
                let a547 = v1;
                append_bytes(a546, a547)
            }, {
                let (_, h548) = v0;
                h548
            }))
        }
    }
}

pub fn f96(v0: u64, v1: u64) -> Bytes {
    f97({
        let a549 = v0;
        let a550 = v1;
        let a551 = 0u64;
        nat_quot(a549, a550, a551)
    })
}

pub fn f97(v0: u64) -> Bytes {
    f77(Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66]), {
        let a552 = v0;
        let a553 = 16u64;
        let a554 = 0u64;
        nat_rem(a552, a553, a554)
    }, 1u64)
}

pub fn f98(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let a555 = v2;
    let a556 = v1;
    if nat_lt(a555, a556) {
        let v3: u64 = f74(v0, v2)?;
        let a557 = v3;
        let a558 = 32u64;
        if nat_lt(a557, a558) {
            Ok(None::<u64>)
        } else {
            let a559 = v3;
            let a560 = 34u64;
            if nat_eq(a559, a560) {
                Ok(None::<u64>)
            } else {
                let a561 = v3;
                let a562 = 92u64;
                if nat_eq(a561, a562) {
                    Ok(None::<u64>)
                } else {
                    Ok(Some({
                        let a563 = v2;
                        let a564 = 1u64;
                        nat_add(a563, a564)?
                    }))
                }
            }
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f99(v0: (List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>)) -> R<Option<(List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>)>> {
    let v1: List<List<(Bytes, Adt1)>> = {
        let (h565, _) = v0;
        h565
    };
    let v2: List<List<(Bytes, Adt1)>> = {
        let (_, h566) = v0;
        h566
    };
    let m567 = v1;
    match m567.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Adt1)>>, List<List<(Bytes, Adt1)>>)>)
        }
        Some((v3, v4)) => {
            let m568 = v4;
            match m568.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Adt1)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f103(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f100(v0: u64, v1: (u64, List<Bytes>)) -> R<Option<(u64, List<Bytes>)>> {
    let a570 = {
        let (h569, _) = v1.clone();
        h569
    };
    let a571 = v0;
    if nat_lt(a570, a571) {
        Ok(Some(({
            let a573 = {
                let (h572, _) = v1;
                h572
            };
            let a574 = 1u64;
            nat_add(a573, a574)?
        }, List::cons(Bytes::lit(&[0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20]), {
            let (_, h575) = v1;
            h575
        }))))
    } else {
        Ok(None::<(u64, List<Bytes>)>)
    }
}

pub fn f101(v0: Adt5, v1: u64, v2: List<Adt3>, v3: List<Adt3>, v4: Option<bool>) -> Adt5 {
    Adt5::C0(v1, match v0.clone() {
        Adt5::C0(_, h576, _, _, _, _) => {
            h576
        }
    }, match v0 {
        Adt5::C0(_, _, h577, _, _, _) => {
            h577
        }
    }, v2, v3, v4)
}

pub fn f102(v0: Adt5, v1: Adt3, v2: List<Adt3>, v3: List<Adt3>) -> R<Adt5> {
    let v4: u64 = match v1.clone() {
        Adt3::C0(h578, _, _) => {
            h578
        }
    };
    let v5: bool = match v1.clone() {
        Adt3::C0(_, h579, _) => {
            h579
        }
    };
    let v6: u64 = match v0.clone() {
        Adt5::C0(h580, _, _, _, _, _) => {
            h580
        }
    };
    let m582 = match v1 {
        Adt3::C0(_, _, h581) => {
            h581
        }
    };
    match m582 {
        Adt1::C0 => {
            Ok(f101(v0, v6, v2, v3, None::<bool>))
        }
        Adt1::C1(v7) => {
            Ok(f101(v0, {
                let a583 = v6;
                let a584 = f68(v7)?;
                nat_add(a583, a584)?
            }, v2, v3, None::<bool>))
        }
        Adt1::C2(r589, r590) => {
            let v8 = (*r589).clone();
            let v9 = (*r590).clone();
            Ok(f101(v0, v6, List::cons(Adt3::C0(v4, v5, v8), List::cons(Adt3::C0(v4, v5, v9), v2)), v3, None::<bool>))
        }
        Adt1::C3(v10, r591) => {
            let v11 = (*r591).clone();
            Ok(f101(v0, v6, List::cons(Adt3::C0({
                let a585 = v4;
                let a586 = v10;
                nat_add(a585, a586)?
            }, v5, v11), v2), v3, None::<bool>))
        }
        Adt1::C4 => {
            if v5 {
                Ok(f101(v0, {
                    let a587 = v6;
                    let a588 = 1u64;
                    nat_add(a587, a588)?
                }, v2, v3, None::<bool>))
            } else {
                Ok(f101(v0, v6, v2, v3, Some(true)))
            }
        }
        Adt1::C5 => {
            if v5 {
                Ok(f101(v0, v6, v2, v3, None::<bool>))
            } else {
                Ok(f101(v0, v6, v2, v3, Some(true)))
            }
        }
        Adt1::C6(r592) => {
            let v12 = (*r592).clone();
            Ok(f101(v0, v6, List::cons(Adt3::C0(v4, true, v12), v2), v3, None::<bool>))
        }
    }
}

pub fn f103(v0: List<(Bytes, Adt1)>, v1: List<(Bytes, Adt1)>) -> R<List<(Bytes, Adt1)>> {
    let v2: u64 = {
        let a597 = {
            let a595 = {
                let a593 = v0.clone();
                length_list(a593)
            };
            let a596 = {
                let a594 = v1.clone();
                length_list(a594)
            };
            nat_add(a595, a596)?
        };
        let a598 = 1u64;
        nat_add(a597, a598)?
    };
    f104({
        let (_, h599) = {
            let (_, h600) = {
                let (h601, _) = f105(Fn18::F106, v2, (v0, (v1, List::<(Bytes, Adt1)>::nil())));
                h601
            };
            h600
        };
        h599
    })
}

pub fn f104(v0: List<(Bytes, Adt1)>) -> R<List<(Bytes, Adt1)>> {
    f24(Fn3::F107, List::<(Bytes, Adt1)>::nil(), v0)
}

pub fn f105(v0: Fn18, p606: u64, p607: (List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>))) -> ((List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>)), bool) {
    let m605 = (p606, p607);
    {
        let mut m605 = m605;
        loop {
            let step = {
                let (v1, v2) = m605;
                let m602 = v1;
                if m602 == 0 {
                    Err::<(u64, (List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>))), ((List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>)), bool)>((v2, false))
                } else {
                    let v3 = m602 - 1;
                    let m604 = {
                        let c603: Fn18 = v0.clone();
                        c603.apply(v2.clone())
                    };
                    match m604 {
                        None => {
                            Err::<(u64, (List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>))), ((List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>)), bool)>((v2, true))
                        }
                        Some(v4) => {
                            Ok::<(u64, (List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>))), ((List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>)), bool)>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m605 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f106(v0: (List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>))) -> Option<(List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>))> {
    f108(v0)
}

pub fn f107(v0: List<(Bytes, Adt1)>, v1: (Bytes, Adt1)) -> List<(Bytes, Adt1)> {
    List::cons(v1, v0)
}

pub fn f108(v0: (List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>))) -> Option<(List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>))> {
    let v1: List<(Bytes, Adt1)> = {
        let (h608, _) = v0.clone();
        h608
    };
    let v2: List<(Bytes, Adt1)> = {
        let (h609, _) = {
            let (_, h610) = v0.clone();
            h610
        };
        h609
    };
    let v3: List<(Bytes, Adt1)> = {
        let (_, h611) = {
            let (_, h612) = v0;
            h612
        };
        h611
    };
    let m613 = v1.clone();
    match m613.uncons() {
        None => {
            let m614 = v2;
            match m614.uncons() {
                None => {
                    None::<(List<(Bytes, Adt1)>, (List<(Bytes, Adt1)>, List<(Bytes, Adt1)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Adt1)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m615 = v2.clone();
            match m615.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Adt1)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f109({
                        let (h616, _) = v8.clone();
                        h616
                    }, {
                        let (h617, _) = v6.clone();
                        h617
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

pub fn f109(v0: Bytes, v1: Bytes) -> bool {
    let a622 = {
        let a618 = v0;
        let a619 = v1;
        compare_bytes(a618, a619)
    };
    let a623 = {
        let a620 = Bytes::lit(&[]);
        let a621 = Bytes::lit(&[0x01]);
        compare_bytes(a620, a621)
    };
    equal(a622, a623)
}

pub fn run(p0: Adt0) -> R<Bytes> {
    f0(p0)
}
