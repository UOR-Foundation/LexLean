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
pub enum Fn0 {
    F14(Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: Option<Bytes>, p1: (Bytes, Bytes)) -> Option<Bytes> {
        match self {
            Fn0::F14(k0) => f14(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F20(u64, Bytes),
}

impl Fn1 {
    pub fn apply(&self, p0: (u64, Bytes)) -> Option<(u64, Bytes)> {
        match self {
            Fn1::F20(k0, k1) => f20(*k0, k1.clone(), p0),
        }
    }
}

pub fn f0(v0: Adt0, v1: bool) -> R<Bytes> {
    let (h1, _) = f1(v0, v1, List::<(Bytes, Bytes)>::nil(), 0u64)?;
    Ok(h1)
}

pub fn f1(v0: Adt0, v1: bool, v2: List<(Bytes, Bytes)>, v3: u64) -> R<(Bytes, u64)> {
    let m2 = v0;
    match m2 {
        Adt0::C0 => {
            Ok((f2(Str::lit("(sort prop)")), v3))
        }
        Adt0::C1(v4) => {
            Ok((f3(f3(f2(Str::lit("(sort (type ")), f4(v4)), f2(Str::lit("))"))), v3))
        }
        Adt0::C2(v5, v6, v7) => {
            Ok((f5(v5, v6, v7), v3))
        }
        Adt0::C3(v8) => {
            Ok((f3(f3(f2(Str::lit("(local ")), if v1 {
                f6(v2, v8)
            } else {
                v8
            }), f2(Str::lit(")"))), v3))
        }
        Adt0::C4(r30, r31) => {
            let v9 = (*r30).clone();
            let v10 = (*r31).clone();
            let v11: (Bytes, u64) = f1(v9, v1, v2.clone(), v3)?;
            let v12: (Bytes, u64) = f7(v10, v1, v2, {
                let (_, h3) = v11;
                h3
            })?;
            Ok((f3(f3(f3(f2(Str::lit("(app ")), {
                let (h4, _) = v11;
                h4
            }), {
                let (h5, _) = v12;
                h5
            }), f2(Str::lit(")"))), {
                let (_, h6) = v12;
                h6
            }))
        }
        Adt0::C5(r32, r33) => {
            let v13 = (*r32).clone();
            let v14 = (*r33).clone();
            let v15: ((Bytes, List<(Bytes, Bytes)>), u64) = f8(v13, v1, v2, v3)?;
            let v16: (Bytes, u64) = f1(v14, v1, {
                let (_, h7) = {
                    let (h8, _) = v15.clone();
                    h8
                };
                h7
            }, {
                let (_, h9) = v15;
                h9
            })?;
            Ok((f3(f3(f3(f3(f2(Str::lit("(pi (")), {
                let (h10, _) = {
                    let (h11, _) = v15;
                    h11
                };
                h10
            }), f2(Str::lit(") "))), {
                let (h12, _) = v16;
                h12
            }), f2(Str::lit(")"))), {
                let (_, h13) = v16;
                h13
            }))
        }
        Adt0::C6(r34, r35) => {
            let v17 = (*r34).clone();
            let v18 = (*r35).clone();
            let v19: ((Bytes, List<(Bytes, Bytes)>), u64) = f8(v17, v1, v2, v3)?;
            let v20: (Bytes, u64) = f1(v18, v1, {
                let (_, h14) = {
                    let (h15, _) = v19.clone();
                    h15
                };
                h14
            }, {
                let (_, h16) = v19;
                h16
            })?;
            Ok((f3(f3(f3(f3(f2(Str::lit("(lam (")), {
                let (h17, _) = {
                    let (h18, _) = v19;
                    h18
                };
                h17
            }), f2(Str::lit(") "))), {
                let (h19, _) = v20;
                h19
            }), f2(Str::lit(")"))), {
                let (_, h20) = v20;
                h20
            }))
        }
        Adt0::C7(v21, r36, r37, r38) => {
            let v22 = (*r36).clone();
            let v23 = (*r37).clone();
            let v24 = (*r38).clone();
            let v25: (Bytes, u64) = f9(v21.clone(), v1, v3)?;
            let v26: List<(Bytes, Bytes)> = List::cons((v21, {
                let (h21, _) = v25.clone();
                h21
            }), v2);
            let v27: (Bytes, u64) = f1(v22, v1, v26.clone(), {
                let (_, h22) = v25;
                h22
            })?;
            let v28: (Bytes, u64) = f1(v23, v1, v26.clone(), {
                let (_, h23) = v27;
                h23
            })?;
            let v29: (Bytes, u64) = f1(v24, v1, v26, {
                let (_, h24) = v28;
                h24
            })?;
            Ok((f3(f3(f3(f3(f3(f3(f3(f2(Str::lit("(let ")), {
                let (h25, _) = v25;
                h25
            }), f2(Str::lit(" "))), {
                let (h26, _) = v27;
                h26
            }), f2(Str::lit(" "))), {
                let (h27, _) = v28;
                h27
            }), f2(Str::lit(" "))), f3({
                let (h28, _) = v29;
                h28
            }, f2(Str::lit(")")))), {
                let (_, h29) = v29;
                h29
            }))
        }
        Adt0::C8(v30) => {
            Ok((f3(f3(f2(Str::lit("(nat ")), v30), f2(Str::lit(")"))), v3))
        }
        Adt0::C9 => {
            Ok((Bytes::lit(&[]), v3))
        }
        Adt0::C10(_, _) => {
            Ok((Bytes::lit(&[]), v3))
        }
        Adt0::C11(_, _, _, _) => {
            Ok((Bytes::lit(&[]), v3))
        }
    }
}

pub fn f2(v0: Str) -> Bytes {
    let a39 = v0;
    utf8_encode(a39)
}

pub fn f3(v0: Bytes, v1: Bytes) -> Bytes {
    let a40 = v0;
    let a41 = v1;
    append_bytes(a40, a41)
}

pub fn f4(v0: Adt1) -> Bytes {
    let m42 = v0;
    match m42 {
        Adt1::C0(v1) => {
            f10(v1)
        }
        Adt1::C1(v2) => {
            v2
        }
        Adt1::C2(r43) => {
            let v3 = (*r43).clone();
            f3(f3(f2(Str::lit("(succ ")), f4(v3)), f2(Str::lit(")")))
        }
        Adt1::C3(r44) => {
            let v4 = (*r44).clone();
            f3(f3(f2(Str::lit("(max")), f11(v4)), f2(Str::lit(")")))
        }
        Adt1::C4(r45, r46) => {
            let v5 = (*r45).clone();
            let v6 = (*r46).clone();
            f3(f3(f3(f3(f2(Str::lit("(imax ")), f4(v5)), f2(Str::lit(" "))), f4(v6)), f2(Str::lit(")")))
        }
        Adt1::C5 => {
            Bytes::lit(&[])
        }
        Adt1::C6(_, _) => {
            Bytes::lit(&[])
        }
    }
}

pub fn f5(v0: Bytes, v1: Bytes, v2: Adt1) -> Bytes {
    let v3: Bytes = f3(f3(f3(f2(Str::lit("(const ")), v0), f2(Str::lit("::"))), v1);
    let m47 = v2.clone();
    match m47 {
        Adt1::C0(_) => {
            f3(v3, f2(Str::lit(")")))
        }
        Adt1::C1(_) => {
            f3(v3, f2(Str::lit(")")))
        }
        Adt1::C2(_) => {
            f3(v3, f2(Str::lit(")")))
        }
        Adt1::C3(_) => {
            f3(v3, f2(Str::lit(")")))
        }
        Adt1::C4(_, _) => {
            f3(v3, f2(Str::lit(")")))
        }
        Adt1::C5 => {
            f3(v3, f2(Str::lit(")")))
        }
        Adt1::C6(_, _) => {
            let v6: Bytes = f11(v2);
            f3(f3(f3(v3, f2(Str::lit(" ("))), f12(v6.clone(), 1u64, {
                let a49 = {
                    let a48 = v6;
                    length_bytes(a48)
                };
                let a50 = 1u64;
                nat_sub(a49, a50)
            })), f2(Str::lit("))")))
        }
    }
}

pub fn f6(v0: List<(Bytes, Bytes)>, v1: Bytes) -> Bytes {
    let m51 = f13(Fn0::F14(v1.clone()), None::<Bytes>, v0);
    match m51 {
        None => {
            v1
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f7(v0: Adt0, v1: bool, v2: List<(Bytes, Bytes)>, v3: u64) -> R<(Bytes, u64)> {
    let m52 = v0;
    match m52 {
        Adt0::C0 => {
            Ok((Bytes::lit(&[]), v3))
        }
        Adt0::C1(_) => {
            Ok((Bytes::lit(&[]), v3))
        }
        Adt0::C2(_, _, _) => {
            Ok((Bytes::lit(&[]), v3))
        }
        Adt0::C3(_) => {
            Ok((Bytes::lit(&[]), v3))
        }
        Adt0::C4(_, _) => {
            Ok((Bytes::lit(&[]), v3))
        }
        Adt0::C5(_, _) => {
            Ok((Bytes::lit(&[]), v3))
        }
        Adt0::C6(_, _) => {
            Ok((Bytes::lit(&[]), v3))
        }
        Adt0::C7(_, _, _, _) => {
            Ok((Bytes::lit(&[]), v3))
        }
        Adt0::C8(_) => {
            Ok((Bytes::lit(&[]), v3))
        }
        Adt0::C9 => {
            Ok((Bytes::lit(&[]), v3))
        }
        Adt0::C10(r57, r58) => {
            let v4 = (*r57).clone();
            let v5 = (*r58).clone();
            let v6: (Bytes, u64) = f1(v4, v1, v2.clone(), v3)?;
            let v7: (Bytes, u64) = f7(v5, v1, v2, {
                let (_, h53) = v6;
                h53
            })?;
            Ok((f3(f3(f2(Str::lit(" ")), {
                let (h54, _) = v6;
                h54
            }), {
                let (h55, _) = v7;
                h55
            }), {
                let (_, h56) = v7;
                h56
            }))
        }
        Adt0::C11(_, _, _, _) => {
            Ok((Bytes::lit(&[]), v3))
        }
    }
}

pub fn f8(v0: Adt0, v1: bool, v2: List<(Bytes, Bytes)>, v3: u64) -> R<((Bytes, List<(Bytes, Bytes)>), u64)> {
    let m59 = v0;
    match m59 {
        Adt0::C0 => {
            Ok(((Bytes::lit(&[]), v2), v3))
        }
        Adt0::C1(_) => {
            Ok(((Bytes::lit(&[]), v2), v3))
        }
        Adt0::C2(_, _, _) => {
            Ok(((Bytes::lit(&[]), v2), v3))
        }
        Adt0::C3(_) => {
            Ok(((Bytes::lit(&[]), v2), v3))
        }
        Adt0::C4(_, _) => {
            Ok(((Bytes::lit(&[]), v2), v3))
        }
        Adt0::C5(_, _) => {
            Ok(((Bytes::lit(&[]), v2), v3))
        }
        Adt0::C6(_, _) => {
            Ok(((Bytes::lit(&[]), v2), v3))
        }
        Adt0::C7(_, _, _, _) => {
            Ok(((Bytes::lit(&[]), v2), v3))
        }
        Adt0::C8(_) => {
            Ok(((Bytes::lit(&[]), v2), v3))
        }
        Adt0::C9 => {
            Ok(((Bytes::lit(&[]), v2), v3))
        }
        Adt0::C10(_, _) => {
            Ok(((Bytes::lit(&[]), v2), v3))
        }
        Adt0::C11(v4, v5, r82, r83) => {
            let v6 = (*r82).clone();
            let v7 = (*r83).clone();
            let v8: (Bytes, u64) = f9(v5.clone(), v1, v3)?;
            let v9: (Bytes, u64) = f1(v6, v1, v2.clone(), {
                let (_, h60) = v8.clone();
                h60
            })?;
            let v10: Bytes = f3(f3(f3(f3(f3(f3(f2(Str::lit("(")), f15(v4)), f2(Str::lit(" "))), {
                let (h61, _) = v8.clone();
                h61
            }), f2(Str::lit(" "))), {
                let (h62, _) = v9;
                h62
            }), f2(Str::lit(")")));
            let v11: List<(Bytes, Bytes)> = List::cons((v5, {
                let (h63, _) = v8;
                h63
            }), v2);
            let m64 = v7.clone();
            match m64 {
                Adt0::C0 => {
                    Ok(((v10, v11), {
                        let (_, h72) = v9;
                        h72
                    }))
                }
                Adt0::C1(_) => {
                    Ok(((v10, v11), {
                        let (_, h73) = v9;
                        h73
                    }))
                }
                Adt0::C2(_, _, _) => {
                    Ok(((v10, v11), {
                        let (_, h74) = v9;
                        h74
                    }))
                }
                Adt0::C3(_) => {
                    Ok(((v10, v11), {
                        let (_, h75) = v9;
                        h75
                    }))
                }
                Adt0::C4(_, _) => {
                    Ok(((v10, v11), {
                        let (_, h76) = v9;
                        h76
                    }))
                }
                Adt0::C5(_, _) => {
                    Ok(((v10, v11), {
                        let (_, h77) = v9;
                        h77
                    }))
                }
                Adt0::C6(_, _) => {
                    Ok(((v10, v11), {
                        let (_, h78) = v9;
                        h78
                    }))
                }
                Adt0::C7(_, _, _, _) => {
                    Ok(((v10, v11), {
                        let (_, h79) = v9;
                        h79
                    }))
                }
                Adt0::C8(_) => {
                    Ok(((v10, v11), {
                        let (_, h80) = v9;
                        h80
                    }))
                }
                Adt0::C9 => {
                    Ok(((v10, v11), {
                        let (_, h65) = v9;
                        h65
                    }))
                }
                Adt0::C10(_, _) => {
                    Ok(((v10, v11), {
                        let (_, h81) = v9;
                        h81
                    }))
                }
                Adt0::C11(_, _, _, _) => {
                    let v16: ((Bytes, List<(Bytes, Bytes)>), u64) = f8(v7, v1, v11, {
                        let (_, h66) = v9;
                        h66
                    })?;
                    Ok(((f3(f3(v10, f2(Str::lit(" "))), {
                        let (h67, _) = {
                            let (h68, _) = v16.clone();
                            h68
                        };
                        h67
                    }), {
                        let (_, h69) = {
                            let (h70, _) = v16;
                            h70
                        };
                        h69
                    }), {
                        let (_, h71) = v16;
                        h71
                    }))
                }
            }
        }
    }
}

pub fn f9(v0: Bytes, v1: bool, v2: u64) -> R<(Bytes, u64)> {
    if v1 {
        Ok((f3(f2(Str::lit("x")), f10(v2)), {
            let a84 = v2;
            let a85 = 1u64;
            nat_add(a84, a85)?
        }))
    } else {
        Ok((v0, v2))
    }
}

pub fn f10(v0: u64) -> Bytes {
    f16(f17(), 10u64, v0)
}

pub fn f11(v0: Adt1) -> Bytes {
    let m86 = v0;
    match m86 {
        Adt1::C0(_) => {
            Bytes::lit(&[])
        }
        Adt1::C1(_) => {
            Bytes::lit(&[])
        }
        Adt1::C2(_) => {
            Bytes::lit(&[])
        }
        Adt1::C3(_) => {
            Bytes::lit(&[])
        }
        Adt1::C4(_, _) => {
            Bytes::lit(&[])
        }
        Adt1::C5 => {
            Bytes::lit(&[])
        }
        Adt1::C6(r87, r88) => {
            let v7 = (*r87).clone();
            let v8 = (*r88).clone();
            f3(f3(f2(Str::lit(" ")), f4(v7)), f11(v8))
        }
    }
}

pub fn f12(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m92 = {
        let a89 = v0;
        let a90 = v1;
        let a91 = v2;
        slice_bytes(a89, a90, a91)
    };
    match m92 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f13(v0: Fn0, p96: Option<Bytes>, p97: List<(Bytes, Bytes)>) -> Option<Bytes> {
    let m95 = (p96, p97);
    {
        let mut m95 = m95;
        loop {
            let step = {
                let (v1, v2) = m95;
                let m93 = v2;
                match m93.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<(Bytes, Bytes)>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<(Bytes, Bytes)>), Option<Bytes>>(({
                            let c94: Fn0 = v0.clone();
                            c94.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m95 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f14(v0: Bytes, v1: Option<Bytes>, v2: (Bytes, Bytes)) -> Option<Bytes> {
    let m98 = v1.clone();
    match m98 {
        None => {
            let a100 = {
                let (h99, _) = v2;
                h99
            };
            let a101 = v0;
            if equal(a100, a101) {
                Some({
                    let (_, h102) = v2;
                    h102
                })
            } else {
                None::<Bytes>
            }
        }
        Some(_) => {
            v1
        }
    }
}

pub fn f15(v0: u64) -> Bytes {
    let a103 = v0;
    let a104 = 1u64;
    if nat_eq(a103, a104) {
        f2(Str::lit("implicit"))
    } else {
        let a105 = v0;
        let a106 = 2u64;
        if nat_eq(a105, a106) {
            f2(Str::lit("instance"))
        } else {
            f2(Str::lit("explicit"))
        }
    }
}

pub fn f16(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let a107 = v2;
    let a108 = 0u64;
    if nat_eq(a107, a108) {
        f18(v0, 0u64)
    } else {
        let (_, h109) = {
            let (h110, _) = f19(Fn1::F20(v1, v0), v2, (v2, Bytes::lit(&[])));
            h110
        };
        h109
    }
}

pub fn f17() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f18(v0: Bytes, v1: u64) -> Bytes {
    let m114 = {
        let a111 = v0;
        let a112 = v1;
        let a113 = 1u64;
        slice_bytes(a111, a112, a113)
    };
    match m114 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f19(v0: Fn1, p119: u64, p120: (u64, Bytes)) -> ((u64, Bytes), bool) {
    let m118 = (p119, p120);
    {
        let mut m118 = m118;
        loop {
            let step = {
                let (v1, v2) = m118;
                let m115 = v1;
                if m115 == 0 {
                    Err::<(u64, (u64, Bytes)), ((u64, Bytes), bool)>((v2, false))
                } else {
                    let v3 = m115 - 1;
                    let m117 = {
                        let c116: Fn1 = v0.clone();
                        c116.apply(v2.clone())
                    };
                    match m117 {
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
                Ok(next) => { m118 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f20(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f21(v1, v0, v2)
}

pub fn f21(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h121, _) = v2;
        h121
    };
    let a122 = v3;
    let a123 = 0u64;
    if nat_eq(a122, a123) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a124 = v3;
            let a125 = v1;
            let a126 = 0u64;
            nat_quot(a124, a125, a126)
        }, {
            let a131 = f18(v0, {
                let a127 = v3;
                let a128 = v1;
                let a129 = 0u64;
                nat_rem(a127, a128, a129)
            });
            let a132 = {
                let (_, h130) = v2;
                h130
            };
            append_bytes(a131, a132)
        }))
    }
}

pub fn run(p0: Adt0, p1: bool) -> R<Bytes> {
    f0(p0, p1)
}
