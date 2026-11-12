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
    C0,
    C1(Bytes),
    C2(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C3(u64, std::rc::Rc<Adt2>),
    C4,
    C5,
    C6(std::rc::Rc<Adt2>),
}

#[derive(Clone)]
pub enum Adt3 {
    C0(List<Bytes>, u64, List<Adt4>),
}

#[derive(Clone)]
pub enum Adt4 {
    C0(u64, bool, Adt2),
}

#[derive(Clone)]
pub enum Adt5 {
    C0(u64, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt6 {
    C0(u64, u64, bool, List<Adt4>, List<Adt4>, Option<bool>),
}

#[derive(Clone)]
pub enum Fn0 {
    F19(u64, u64),
}

impl Fn0 {
    pub fn apply(&self, p0: Adt3) -> R<Option<Adt3>> {
        match self {
            Fn0::F19(k0, k1) => f19(*k0, *k1, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F26,
}

impl Fn1 {
    pub fn apply(&self, p0: (bool, Adt2), p1: Adt2) -> (bool, Adt2) {
        match self {
            Fn1::F26 => f26(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F28,
}

impl Fn2 {
    pub fn apply(&self, p0: List<Adt2>, p1: Adt2) -> List<Adt2> {
        match self {
            Fn2::F28 => f28(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F30(Option<Bytes>),
    F112,
}

impl Fn3 {
    pub fn apply(&self, p0: List<(Bytes, Adt2)>, p1: (Bytes, Adt2)) -> R<List<(Bytes, Adt2)>> {
        match self {
            Fn3::F30(k0) => f30(k0.clone(), p0, p1),
            Fn3::F112 => Ok(f112(p0, p1)),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F32,
}

impl Fn4 {
    pub fn apply(&self, p0: List<Adt2>, p1: (Bytes, Adt2)) -> List<Adt2> {
        match self {
            Fn4::F32 => f32(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F44(u64, Bytes),
}

impl Fn5 {
    pub fn apply(&self, p0: Adt5) -> R<Option<Adt5>> {
        match self {
            Fn5::F44(k0, k1) => f44(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F53,
}

impl Fn6 {
    pub fn apply(&self, p0: List<List<(Bytes, Adt2)>>, p1: (Bytes, Adt2)) -> List<List<(Bytes, Adt2)>> {
        match self {
            Fn6::F53 => f53(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F55,
}

impl Fn7 {
    pub fn apply(&self, p0: List<List<(Bytes, Adt2)>>) -> R<Option<List<List<(Bytes, Adt2)>>>> {
        match self {
            Fn7::F55 => f55(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F58,
}

impl Fn8 {
    pub fn apply(&self, p0: List<Bytes>) -> Option<List<Bytes>> {
        match self {
            Fn8::F58 => f58(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F60,
}

impl Fn9 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn9::F60 => f60(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F63(u64, Bytes),
}

impl Fn10 {
    pub fn apply(&self, p0: (u64, Bytes)) -> Option<(u64, Bytes)> {
        match self {
            Fn10::F63(k0, k1) => f63(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F66(Bytes),
    F93(u64, Bytes),
}

impl Fn11 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn11::F66(k0) => f66(k0.clone(), p0),
            Fn11::F93(k0, k1) => f93(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F71,
}

impl Fn12 {
    pub fn apply(&self, p0: List<List<(Bytes, Adt2)>>, p1: List<(Bytes, Adt2)>) -> List<List<(Bytes, Adt2)>> {
        match self {
            Fn12::F71 => f71(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F85(Bytes),
}

impl Fn13 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn13::F85(k0) => f85(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F88,
}

impl Fn14 {
    pub fn apply(&self, p0: Adt6) -> R<Option<Adt6>> {
        match self {
            Fn14::F88 => f88(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F90,
}

impl Fn15 {
    pub fn apply(&self, p0: (Option<Bytes>, List<Bytes>), p1: Bytes) -> (Option<Bytes>, List<Bytes>) {
        match self {
            Fn15::F90 => f90(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F95,
}

impl Fn16 {
    pub fn apply(&self, p0: (List<List<(Bytes, Adt2)>>, List<List<(Bytes, Adt2)>>)) -> R<Option<(List<List<(Bytes, Adt2)>>, List<List<(Bytes, Adt2)>>)>> {
        match self {
            Fn16::F95 => f95(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F98(u64),
}

impl Fn17 {
    pub fn apply(&self, p0: (u64, List<Bytes>)) -> R<Option<(u64, List<Bytes>)>> {
        match self {
            Fn17::F98(k0) => f98(*k0, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F111,
}

impl Fn18 {
    pub fn apply(&self, p0: (List<(Bytes, Adt2)>, (List<(Bytes, Adt2)>, List<(Bytes, Adt2)>))) -> Option<(List<(Bytes, Adt2)>, (List<(Bytes, Adt2)>, List<(Bytes, Adt2)>))> {
        match self {
            Fn18::F111 => f111(p0),
        }
    }
}

pub fn f0(v0: Adt0) -> R<Result<Bytes, Adt1>> {
    let m1 = v0;
    match m1 {
        Adt0::C0(_) => {
            Ok(Err::<Bytes, Adt1>(f4(0u64, f5(Str::lit("a module is an object")))))
        }
        Adt0::C1(_, _) => {
            Ok(Err::<Bytes, Adt1>(f4(0u64, f5(Str::lit("a module is an object")))))
        }
        Adt0::C2(_) => {
            Ok(Err::<Bytes, Adt1>(f4(0u64, f5(Str::lit("a module is an object")))))
        }
        Adt0::C3(_) => {
            Ok(Err::<Bytes, Adt1>(f4(0u64, f5(Str::lit("a module is an object")))))
        }
        Adt0::C4(r4) => {
            let v1 = (*r4).clone();
            if f1(v1.clone()) {
                Ok(Ok::<Bytes, Adt1>({
                    let a2 = f2(Adt0::C4(std::rc::Rc::new(f3(v1))))?;
                    let a3 = Bytes::lit(&[0x0a]);
                    append_bytes(a2, a3)
                }))
            } else {
                Ok(Err::<Bytes, Adt1>(f4(0u64, f5(Str::lit("a module has a `spec`")))))
            }
        }
        Adt0::C5 => {
            Ok(Err::<Bytes, Adt1>(f4(0u64, f5(Str::lit("a module is an object")))))
        }
        Adt0::C6(_, _) => {
            Ok(Err::<Bytes, Adt1>(f4(0u64, f5(Str::lit("a module is an object")))))
        }
        Adt0::C7(_, _, _) => {
            Ok(Err::<Bytes, Adt1>(f4(0u64, f5(Str::lit("a module is an object")))))
        }
    }
}

pub fn f1(p12: Adt0) -> bool {
    let m11 = p12;
    {
        let mut m11 = m11;
        loop {
            let step = {
                let v0 = m11;
                let m5 = v0;
                match m5 {
                    Adt0::C0(_) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C1(_, _) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C2(_) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C3(_) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C4(_) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C5 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C6(_, _) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C7(v1, r9, r10) => {
                        let v2 = (*r9).clone();
                        let v3 = (*r10).clone();
                        let a6 = v1;
                        let a7 = f5(Str::lit("spec"));
                        if equal(a6, a7) {
                            let m8 = v2;
                            match m8 {
                                Adt0::C0(_) => {
                                    Err::<Adt0, bool>(false)
                                }
                                Adt0::C1(_, _) => {
                                    Err::<Adt0, bool>(false)
                                }
                                Adt0::C2(_) => {
                                    Err::<Adt0, bool>(true)
                                }
                                Adt0::C3(_) => {
                                    Err::<Adt0, bool>(false)
                                }
                                Adt0::C4(_) => {
                                    Err::<Adt0, bool>(false)
                                }
                                Adt0::C5 => {
                                    Err::<Adt0, bool>(false)
                                }
                                Adt0::C6(_, _) => {
                                    Err::<Adt0, bool>(false)
                                }
                                Adt0::C7(_, _, _) => {
                                    Err::<Adt0, bool>(false)
                                }
                            }
                        } else {
                            Ok::<Adt0, bool>(v3)
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

pub fn f2(v0: Adt0) -> R<Bytes> {
    let v1: Adt2 = f6(v0)?;
    f7(v1.clone(), 100u64, f8(v1)?)
}

pub fn f3(v0: Adt0) -> Adt0 {
    let m13 = v0;
    match m13 {
        Adt0::C0(v4) => {
            Adt0::C0(v4)
        }
        Adt0::C1(v5, v6) => {
            Adt0::C1(v5, v6)
        }
        Adt0::C2(v7) => {
            Adt0::C2(v7)
        }
        Adt0::C3(r16) => {
            let v8 = (*r16).clone();
            Adt0::C3(std::rc::Rc::new(v8))
        }
        Adt0::C4(r17) => {
            let v9 = (*r17).clone();
            Adt0::C4(std::rc::Rc::new(v9))
        }
        Adt0::C5 => {
            Adt0::C5
        }
        Adt0::C6(r18, r19) => {
            let v10 = (*r18).clone();
            let v11 = (*r19).clone();
            Adt0::C6(std::rc::Rc::new(v10), std::rc::Rc::new(v11))
        }
        Adt0::C7(v1, r20, r21) => {
            let v2 = (*r20).clone();
            let v3 = (*r21).clone();
            let a14 = v1.clone();
            let a15 = f5(Str::lit("spec"));
            if equal(a14, a15) {
                f3(v3)
            } else {
                Adt0::C7(v1, std::rc::Rc::new(v2), std::rc::Rc::new(f3(v3)))
            }
        }
    }
}

pub fn f4(v0: u64, v1: Bytes) -> Adt1 {
    Adt1::C0(v0, v1)
}

pub fn f5(v0: Str) -> Bytes {
    let a22 = v0;
    utf8_encode(a22)
}

pub fn f6(v0: Adt0) -> R<Adt2> {
    let m23 = v0;
    match m23 {
        Adt0::C0(v1) => {
            Ok(Adt2::C1(if v1 {
                Bytes::lit(&[0x74, 0x72, 0x75, 0x65])
            } else {
                Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65])
            }))
        }
        Adt0::C1(v2, v3) => {
            Ok(Adt2::C1(f9(v2, v3)))
        }
        Adt0::C2(v4) => {
            Ok(Adt2::C1(f10(v4)?))
        }
        Adt0::C3(r24) => {
            let v5 = (*r24).clone();
            Ok(f11(f12(Str::lit("[")), f12(Str::lit("]")), f13(f14(v5)?)))
        }
        Adt0::C4(r25) => {
            let v6 = (*r25).clone();
            f15(f16(v6.clone())?, f17(v6)?)
        }
        Adt0::C5 => {
            Ok(Adt2::C0)
        }
        Adt0::C6(_, _) => {
            Ok(Adt2::C0)
        }
        Adt0::C7(_, _, _) => {
            Ok(Adt2::C0)
        }
    }
}

pub fn f7(v0: Adt2, v1: u64, v2: u64) -> R<Bytes> {
    let v3: Adt3 = {
        let (h26, _) = f18(Fn0::F19(v2, v1), {
            let a27 = v2;
            let a28 = 1u64;
            nat_add(a27, a28)?
        }, Adt3::C0(List::<Bytes>::nil(), 0u64, List::cons(Adt4::C0(0u64, false, v0), List::<Adt4>::nil())))?;
        h26
    };
    Ok(f20(match v3 {
        Adt3::C0(h29, _, _) => {
            h29
        }
    }))
}

pub fn f8(v0: Adt2) -> R<u64> {
    let m30 = v0;
    match m30 {
        Adt2::C0 => {
            Ok(1u64)
        }
        Adt2::C1(_) => {
            Ok(1u64)
        }
        Adt2::C2(r39, r40) => {
            let v2 = (*r39).clone();
            let v3 = (*r40).clone();
            let v4: u64 = f8(v2)?;
            let v5: u64 = f8(v3)?;
            let a33 = {
                let a31 = 1u64;
                let a32 = v4;
                nat_add(a31, a32)?
            };
            let a34 = v5;
            nat_add(a33, a34)
        }
        Adt2::C3(_, r41) => {
            let v7 = (*r41).clone();
            let v8: u64 = f8(v7)?;
            let a35 = 1u64;
            let a36 = v8;
            nat_add(a35, a36)
        }
        Adt2::C4 => {
            Ok(1u64)
        }
        Adt2::C5 => {
            Ok(1u64)
        }
        Adt2::C6(r42) => {
            let v9 = (*r42).clone();
            let v10: u64 = f8(v9)?;
            let a37 = 1u64;
            let a38 = v10;
            nat_add(a37, a38)
        }
    }
}

pub fn f9(v0: bool, v1: u64) -> Bytes {
    if if v0 {
        let a43 = 0u64;
        let a44 = v1;
        nat_lt(a43, a44)
    } else {
        false
    } {
        let a45 = Bytes::lit(&[0x2d]);
        let a46 = f21(v1);
        append_bytes(a45, a46)
    } else {
        f21(v1)
    }
}

pub fn f10(v0: Bytes) -> R<Bytes> {
    if f22(v0.clone())? {
        Ok(v0)
    } else {
        f23(v0)
    }
}

pub fn f11(v0: Bytes, v1: Bytes, v2: List<Adt2>) -> Adt2 {
    let m47 = v2.clone();
    match m47.uncons() {
        None => {
            Adt2::C1(f24(v0, v1))
        }
        Some((_, _)) => {
            let v5: (bool, Adt2) = f25(Fn1::F26, (true, Adt2::C0), v2);
            Adt2::C6(std::rc::Rc::new(Adt2::C2(std::rc::Rc::new(Adt2::C1(v0)), std::rc::Rc::new(Adt2::C2(std::rc::Rc::new(Adt2::C3(2u64, std::rc::Rc::new(Adt2::C2(std::rc::Rc::new(Adt2::C5), std::rc::Rc::new(Adt2::C2(std::rc::Rc::new({
                let (_, h48) = v5;
                h48
            }), std::rc::Rc::new(Adt2::C0))))))), std::rc::Rc::new(Adt2::C2(std::rc::Rc::new(Adt2::C5), std::rc::Rc::new(Adt2::C2(std::rc::Rc::new(Adt2::C1(v1)), std::rc::Rc::new(Adt2::C0))))))))))
        }
    }
}

pub fn f12(v0: Str) -> Bytes {
    let a49 = v0;
    utf8_encode(a49)
}

pub fn f13(v0: List<Adt2>) -> List<Adt2> {
    f27(Fn2::F28, List::<Adt2>::nil(), v0)
}

pub fn f14(v0: Adt0) -> R<List<Adt2>> {
    let m50 = v0;
    match m50 {
        Adt0::C0(_) => {
            Ok(List::<Adt2>::nil())
        }
        Adt0::C1(_, _) => {
            Ok(List::<Adt2>::nil())
        }
        Adt0::C2(_) => {
            Ok(List::<Adt2>::nil())
        }
        Adt0::C3(_) => {
            Ok(List::<Adt2>::nil())
        }
        Adt0::C4(_) => {
            Ok(List::<Adt2>::nil())
        }
        Adt0::C5 => {
            Ok(List::<Adt2>::nil())
        }
        Adt0::C6(r51, r52) => {
            let v7 = (*r51).clone();
            let v8 = (*r52).clone();
            Ok(List::cons(f6(v7)?, f14(v8)?))
        }
        Adt0::C7(_, _, _) => {
            Ok(List::<Adt2>::nil())
        }
    }
}

pub fn f15(v0: Option<Bytes>, v1: List<(Bytes, Adt2)>) -> R<Adt2> {
    let v2: List<(Bytes, Adt2)> = f29(Fn3::F30(v0.clone()), List::<(Bytes, Adt2)>::nil(), v1)?;
    Ok(f11({
        let m53 = v0;
        match m53 {
            None => {
                f12(Str::lit("{"))
            }
            Some(v3) => {
                f24(v3, f12(Str::lit("{")))
            }
        }
    }, f12(Str::lit("}")), f31(Fn4::F32, List::<Adt2>::nil(), f33(v2)?)))
}

pub fn f16(p61: Adt0) -> R<Option<Bytes>> {
    let m60 = p61;
    {
        let mut m60 = m60;
        loop {
            let step = {
                let v0 = m60;
                let m54 = v0;
                match m54 {
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
                    Adt0::C7(v9, r58, r59) => {
                        let v10 = (*r58).clone();
                        let v11 = (*r59).clone();
                        let a55 = v9;
                        let a56 = Bytes::lit(&[0x6b, 0x69, 0x6e, 0x64]);
                        if equal(a55, a56) {
                            let m57 = v10;
                            match m57 {
                                Adt0::C0(_) => {
                                    Err::<Adt0, R<Option<Bytes>>>(Ok(None::<Bytes>))
                                }
                                Adt0::C1(_, _) => {
                                    Err::<Adt0, R<Option<Bytes>>>(Ok(None::<Bytes>))
                                }
                                Adt0::C2(v12) => {
                                    if f22(v12.clone())? {
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
                Ok(next) => { m60 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f17(v0: Adt0) -> R<List<(Bytes, Adt2)>> {
    let m62 = v0;
    match m62 {
        Adt0::C0(_) => {
            Ok(List::<(Bytes, Adt2)>::nil())
        }
        Adt0::C1(_, _) => {
            Ok(List::<(Bytes, Adt2)>::nil())
        }
        Adt0::C2(_) => {
            Ok(List::<(Bytes, Adt2)>::nil())
        }
        Adt0::C3(_) => {
            Ok(List::<(Bytes, Adt2)>::nil())
        }
        Adt0::C4(_) => {
            Ok(List::<(Bytes, Adt2)>::nil())
        }
        Adt0::C5 => {
            Ok(List::<(Bytes, Adt2)>::nil())
        }
        Adt0::C6(_, _) => {
            Ok(List::<(Bytes, Adt2)>::nil())
        }
        Adt0::C7(v9, r63, r64) => {
            let v10 = (*r63).clone();
            let v11 = (*r64).clone();
            Ok(List::cons((v9, f6(v10)?), f17(v11)?))
        }
    }
}

pub fn f18(v0: Fn0, p69: u64, p70: Adt3) -> R<(Adt3, bool)> {
    let m68 = (p69, p70);
    {
        let mut m68 = m68;
        loop {
            let step = {
                let (v1, v2) = m68;
                let m65 = v1;
                if m65 == 0 {
                    Err::<(u64, Adt3), R<(Adt3, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m65 - 1;
                    let m67 = {
                        let c66: Fn0 = v0.clone();
                        c66.apply(v2.clone())?
                    };
                    match m67 {
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
                Ok(next) => { m68 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f19(v0: u64, v1: u64, v2: Adt3) -> R<Option<Adt3>> {
    f34(v1, v0, v2)
}

pub fn f20(v0: List<Bytes>) -> Bytes {
    f35(f36(v0))
}

pub fn f21(v0: u64) -> Bytes {
    f37(f38(), 10u64, v0)
}

pub fn f22(v0: Bytes) -> R<bool> {
    let a72 = {
        let a71 = v0.clone();
        length_bytes(a71)
    };
    let a73 = 0u64;
    if nat_eq(a72, a73) {
        Ok(false)
    } else {
        if f39(f40(v0.clone(), 0u64)?) {
            let a75 = f41(v0.clone(), 1u64)?;
            let a76 = {
                let a74 = v0.clone();
                length_bytes(a74)
            };
            if nat_eq(a75, a76) {
                let a77 = f42(v0);
                Ok(bool_not(a77))
            } else {
                Ok(false)
            }
        } else {
            Ok(false)
        }
    }
}

pub fn f23(v0: Bytes) -> R<Bytes> {
    let v1: u64 = {
        let a78 = v0.clone();
        length_bytes(a78)
    };
    let v2: Adt5 = {
        let (h79, _) = f43(Fn5::F44(v1, v0), {
            let a80 = v1;
            let a81 = 1u64;
            nat_add(a80, a81)?
        }, Adt5::C0(0u64, List::<Bytes>::nil()))?;
        h79
    };
    Ok(f45(f45(f46(Str::lit("\"")), f47(match v2 {
        Adt5::C0(_, h82) => {
            h82
        }
    })), f46(Str::lit("\""))))
}

pub fn f24(v0: Bytes, v1: Bytes) -> Bytes {
    let a83 = v0;
    let a84 = v1;
    append_bytes(a83, a84)
}

pub fn f25(v0: Fn1, p88: (bool, Adt2), p89: List<Adt2>) -> (bool, Adt2) {
    let m87 = (p88, p89);
    {
        let mut m87 = m87;
        loop {
            let step = {
                let (v1, v2) = m87;
                let m85 = v2;
                match m85.uncons() {
                    None => {
                        Err::<((bool, Adt2), List<Adt2>), (bool, Adt2)>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<((bool, Adt2), List<Adt2>), (bool, Adt2)>(({
                            let c86: Fn1 = v0.clone();
                            c86.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m87 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f26(v0: (bool, Adt2), v1: Adt2) -> (bool, Adt2) {
    f48(v0, v1)
}

pub fn f27(v0: Fn2, p93: List<Adt2>, p94: List<Adt2>) -> List<Adt2> {
    let m92 = (p93, p94);
    {
        let mut m92 = m92;
        loop {
            let step = {
                let (v1, v2) = m92;
                let m90 = v2;
                match m90.uncons() {
                    None => {
                        Err::<(List<Adt2>, List<Adt2>), List<Adt2>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt2>, List<Adt2>), List<Adt2>>(({
                            let c91: Fn2 = v0.clone();
                            c91.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m92 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f28(v0: List<Adt2>, v1: Adt2) -> List<Adt2> {
    List::cons(v1, v0)
}

pub fn f29(v0: Fn3, p98: List<(Bytes, Adt2)>, p99: List<(Bytes, Adt2)>) -> R<List<(Bytes, Adt2)>> {
    let m97 = (p98, p99);
    {
        let mut m97 = m97;
        loop {
            let step = {
                let (v1, v2) = m97;
                let m95 = v2;
                match m95.uncons() {
                    None => {
                        Err::<(List<(Bytes, Adt2)>, List<(Bytes, Adt2)>), R<List<(Bytes, Adt2)>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Adt2)>, List<(Bytes, Adt2)>), R<List<(Bytes, Adt2)>>>(({
                            let c96: Fn3 = v0.clone();
                            c96.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m97 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f30(v0: Option<Bytes>, v1: List<(Bytes, Adt2)>, v2: (Bytes, Adt2)) -> R<List<(Bytes, Adt2)>> {
    let m100 = v0;
    match m100 {
        None => {
            Ok(List::cons((f49({
                let (h107, _) = v2.clone();
                h107
            }), f50({
                let (h108, _) = v2;
                h108
            }, {
                let (_, h109) = v2;
                h109
            })?), v1))
        }
        Some(_) => {
            let a102 = {
                let (h101, _) = v2.clone();
                h101
            };
            let a103 = Bytes::lit(&[0x6b, 0x69, 0x6e, 0x64]);
            if equal(a102, a103) {
                Ok(v1)
            } else {
                Ok(List::cons((f49({
                    let (h104, _) = v2.clone();
                    h104
                }), f50({
                    let (h105, _) = v2;
                    h105
                }, {
                    let (_, h106) = v2;
                    h106
                })?), v1))
            }
        }
    }
}

pub fn f31(v0: Fn4, p113: List<Adt2>, p114: List<(Bytes, Adt2)>) -> List<Adt2> {
    let m112 = (p113, p114);
    {
        let mut m112 = m112;
        loop {
            let step = {
                let (v1, v2) = m112;
                let m110 = v2;
                match m110.uncons() {
                    None => {
                        Err::<(List<Adt2>, List<(Bytes, Adt2)>), List<Adt2>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt2>, List<(Bytes, Adt2)>), List<Adt2>>(({
                            let c111: Fn4 = v0.clone();
                            c111.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m112 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f32(v0: List<Adt2>, v1: (Bytes, Adt2)) -> List<Adt2> {
    List::cons({
        let (_, h115) = v1;
        h115
    }, v0)
}

pub fn f33(v0: List<(Bytes, Adt2)>) -> R<List<(Bytes, Adt2)>> {
    let v1: List<List<(Bytes, Adt2)>> = f51(f52(Fn6::F53, List::<List<(Bytes, Adt2)>>::nil(), v0.clone()));
    let m120 = {
        let (h116, _) = f54(Fn7::F55, {
            let a118 = {
                let a117 = v0;
                length_list(a117)
            };
            let a119 = 1u64;
            nat_add(a118, a119)?
        }, v1)?;
        h116
    };
    match m120.uncons() {
        None => {
            Ok(List::<(Bytes, Adt2)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f34(v0: u64, v1: u64, v2: Adt3) -> R<Option<Adt3>> {
    let m122 = match v2.clone() {
        Adt3::C0(_, _, h121) => {
            h121
        }
    };
    match m122.uncons() {
        None => {
            Ok(None::<Adt3>)
        }
        Some((v3, v4)) => {
            Ok(Some(f56(v0, v1, v2, v3, v4)?))
        }
    }
}

pub fn f35(v0: List<Bytes>) -> Bytes {
    let m125 = {
        let (h123, _) = f57(Fn8::F58, {
            let a124 = v0.clone();
            length_list(a124)
        }, v0);
        h123
    };
    match m125.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f36(v0: List<Bytes>) -> List<Bytes> {
    f59(Fn9::F60, List::<Bytes>::nil(), v0)
}

pub fn f37(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let a126 = v2;
    let a127 = 0u64;
    if nat_eq(a126, a127) {
        f61(v0, 0u64)
    } else {
        let (_, h128) = {
            let (h129, _) = f62(Fn10::F63(v1, v0), v2, (v2, Bytes::lit(&[])));
            h129
        };
        h128
    }
}

pub fn f38() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f39(v0: u64) -> bool {
    let a130 = 64u64;
    let a131 = v0;
    if if nat_lt(a130, a131) {
        let a132 = v0;
        let a133 = 91u64;
        nat_lt(a132, a133)
    } else {
        false
    } {
        true
    } else {
        let a134 = 96u64;
        let a135 = v0;
        if if nat_lt(a134, a135) {
            let a136 = v0;
            let a137 = 123u64;
            nat_lt(a136, a137)
        } else {
            false
        } {
            true
        } else {
            let a138 = v0;
            let a139 = 95u64;
            nat_eq(a138, a139)
        }
    }
}

pub fn f40(v0: Bytes, v1: u64) -> R<u64> {
    let m140 = f64(v0, v1);
    match m140 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a185 = {
                let a161 = {
                    let a149 = {
                        let a143 = {
                            let a141 = v3;
                            let a142 = 128u8;
                            fixed_u8::bit_and(a141, a142)
                        };
                        let a144 = 128u8;
                        if equal(a143, a144) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a150 = {
                        let a147 = {
                            let a145 = v3;
                            let a146 = 64u8;
                            fixed_u8::bit_and(a145, a146)
                        };
                        let a148 = 64u8;
                        if equal(a147, a148) {
                            64u64
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
                            let a152 = 32u8;
                            fixed_u8::bit_and(a151, a152)
                        };
                        let a154 = 32u8;
                        if equal(a153, a154) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a160 = {
                        let a157 = {
                            let a155 = v3;
                            let a156 = 16u8;
                            fixed_u8::bit_and(a155, a156)
                        };
                        let a158 = 16u8;
                        if equal(a157, a158) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a159, a160)?
                };
                nat_add(a161, a162)?
            };
            let a186 = {
                let a183 = {
                    let a171 = {
                        let a165 = {
                            let a163 = v3;
                            let a164 = 8u8;
                            fixed_u8::bit_and(a163, a164)
                        };
                        let a166 = 8u8;
                        if equal(a165, a166) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a172 = {
                        let a169 = {
                            let a167 = v3;
                            let a168 = 4u8;
                            fixed_u8::bit_and(a167, a168)
                        };
                        let a170 = 4u8;
                        if equal(a169, a170) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a171, a172)?
                };
                let a184 = {
                    let a181 = {
                        let a175 = {
                            let a173 = v3;
                            let a174 = 2u8;
                            fixed_u8::bit_and(a173, a174)
                        };
                        let a176 = 2u8;
                        if equal(a175, a176) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a182 = {
                        let a179 = {
                            let a177 = v3;
                            let a178 = 1u8;
                            fixed_u8::bit_and(a177, a178)
                        };
                        let a180 = 1u8;
                        if equal(a179, a180) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a181, a182)?
                };
                nat_add(a183, a184)?
            };
            nat_add(a185, a186)
        }
    }
}

pub fn f41(v0: Bytes, v1: u64) -> R<u64> {
    let (h187, _) = f65(Fn11::F66(v0.clone()), {
        let a189 = {
            let a188 = v0;
            length_bytes(a188)
        };
        let a190 = 1u64;
        nat_add(a189, a190)?
    }, v1)?;
    Ok(h187)
}

pub fn f42(v0: Bytes) -> bool {
    let a191 = v0.clone();
    let a192 = Bytes::lit(&[0x74, 0x72, 0x75, 0x65]);
    if equal(a191, a192) {
        true
    } else {
        let a193 = v0;
        let a194 = Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]);
        equal(a193, a194)
    }
}

pub fn f43(v0: Fn5, p199: u64, p200: Adt5) -> R<(Adt5, bool)> {
    let m198 = (p199, p200);
    {
        let mut m198 = m198;
        loop {
            let step = {
                let (v1, v2) = m198;
                let m195 = v1;
                if m195 == 0 {
                    Err::<(u64, Adt5), R<(Adt5, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m195 - 1;
                    let m197 = {
                        let c196: Fn5 = v0.clone();
                        c196.apply(v2.clone())?
                    };
                    match m197 {
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
                Ok(next) => { m198 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f44(v0: u64, v1: Bytes, v2: Adt5) -> R<Option<Adt5>> {
    f67(v1, v0, v2)
}

pub fn f45(v0: Bytes, v1: Bytes) -> Bytes {
    let a201 = v0;
    let a202 = v1;
    append_bytes(a201, a202)
}

pub fn f46(v0: Str) -> Bytes {
    let a203 = v0;
    utf8_encode(a203)
}

pub fn f47(v0: List<Bytes>) -> Bytes {
    f35(f36(v0))
}

pub fn f48(v0: (bool, Adt2), v1: Adt2) -> (bool, Adt2) {
    let (h204, _) = v0;
    if h204 {
        (false, Adt2::C2(std::rc::Rc::new(v1), std::rc::Rc::new(Adt2::C0)))
    } else {
        (false, Adt2::C2(std::rc::Rc::new(v1), std::rc::Rc::new(Adt2::C2(std::rc::Rc::new(Adt2::C1(f12(Str::lit(",")))), std::rc::Rc::new(Adt2::C2(std::rc::Rc::new(Adt2::C4), std::rc::Rc::new({
            let (_, h205) = v0;
            h205
        })))))))
    }
}

pub fn f49(v0: Bytes) -> Bytes {
    f24(f68(f69(v0.clone())), v0)
}

pub fn f50(v0: Bytes, v1: Adt2) -> R<Adt2> {
    Ok(Adt2::C2(std::rc::Rc::new(Adt2::C1(f24(f10(v0)?, f12(Str::lit(": "))))), std::rc::Rc::new(Adt2::C2(std::rc::Rc::new(v1), std::rc::Rc::new(Adt2::C0)))))
}

pub fn f51(v0: List<List<(Bytes, Adt2)>>) -> List<List<(Bytes, Adt2)>> {
    f70(Fn12::F71, List::<List<(Bytes, Adt2)>>::nil(), v0)
}

pub fn f52(v0: Fn6, p209: List<List<(Bytes, Adt2)>>, p210: List<(Bytes, Adt2)>) -> List<List<(Bytes, Adt2)>> {
    let m208 = (p209, p210);
    {
        let mut m208 = m208;
        loop {
            let step = {
                let (v1, v2) = m208;
                let m206 = v2;
                match m206.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Adt2)>>, List<(Bytes, Adt2)>), List<List<(Bytes, Adt2)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Adt2)>>, List<(Bytes, Adt2)>), List<List<(Bytes, Adt2)>>>(({
                            let c207: Fn6 = v0.clone();
                            c207.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m208 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f53(v0: List<List<(Bytes, Adt2)>>, v1: (Bytes, Adt2)) -> List<List<(Bytes, Adt2)>> {
    List::cons(List::cons(v1, List::<(Bytes, Adt2)>::nil()), v0)
}

pub fn f54(v0: Fn7, p215: u64, p216: List<List<(Bytes, Adt2)>>) -> R<(List<List<(Bytes, Adt2)>>, bool)> {
    let m214 = (p215, p216);
    {
        let mut m214 = m214;
        loop {
            let step = {
                let (v1, v2) = m214;
                let m211 = v1;
                if m211 == 0 {
                    Err::<(u64, List<List<(Bytes, Adt2)>>), R<(List<List<(Bytes, Adt2)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m211 - 1;
                    let m213 = {
                        let c212: Fn7 = v0.clone();
                        c212.apply(v2.clone())?
                    };
                    match m213 {
                        None => {
                            Err::<(u64, List<List<(Bytes, Adt2)>>), R<(List<List<(Bytes, Adt2)>>, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, List<List<(Bytes, Adt2)>>), R<(List<List<(Bytes, Adt2)>>, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m214 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f55(v0: List<List<(Bytes, Adt2)>>) -> R<Option<List<List<(Bytes, Adt2)>>>> {
    f72(v0)
}

pub fn f56(v0: u64, v1: u64, v2: Adt3, v3: Adt4, v4: List<Adt4>) -> R<Adt3> {
    let v5: u64 = match v3.clone() {
        Adt4::C0(h217, _, _) => {
            h217
        }
    };
    let v6: bool = match v3.clone() {
        Adt4::C0(_, h218, _) => {
            h218
        }
    };
    let m220 = match v3 {
        Adt4::C0(_, _, h219) => {
            h219
        }
    };
    match m220 {
        Adt2::C0 => {
            Ok(Adt3::C0(match v2.clone() {
                Adt3::C0(h221, _, _) => {
                    h221
                }
            }, match v2 {
                Adt3::C0(_, h222, _) => {
                    h222
                }
            }, v4))
        }
        Adt2::C1(v7) => {
            Ok(Adt3::C0(List::cons(v7.clone(), match v2.clone() {
                Adt3::C0(h223, _, _) => {
                    h223
                }
            }), {
                let a225 = match v2 {
                    Adt3::C0(_, h224, _) => {
                        h224
                    }
                };
                let a226 = f73(v7)?;
                nat_add(a225, a226)?
            }, v4))
        }
        Adt2::C2(r243, r244) => {
            let v8 = (*r243).clone();
            let v9 = (*r244).clone();
            Ok(Adt3::C0(match v2.clone() {
                Adt3::C0(h227, _, _) => {
                    h227
                }
            }, match v2 {
                Adt3::C0(_, h228, _) => {
                    h228
                }
            }, List::cons(Adt4::C0(v5, v6, v8), List::cons(Adt4::C0(v5, v6, v9), v4))))
        }
        Adt2::C3(v10, r245) => {
            let v11 = (*r245).clone();
            Ok(Adt3::C0(match v2.clone() {
                Adt3::C0(h229, _, _) => {
                    h229
                }
            }, match v2 {
                Adt3::C0(_, h230, _) => {
                    h230
                }
            }, List::cons(Adt4::C0({
                let a231 = v5;
                let a232 = v10;
                nat_add(a231, a232)?
            }, v6, v11), v4)))
        }
        Adt2::C4 => {
            if v6 {
                Ok(Adt3::C0(List::cons({
                    let a233 = Str::lit(" ");
                    utf8_encode(a233)
                }, match v2.clone() {
                    Adt3::C0(h234, _, _) => {
                        h234
                    }
                }), {
                    let a236 = match v2 {
                        Adt3::C0(_, h235, _) => {
                            h235
                        }
                    };
                    let a237 = 1u64;
                    nat_add(a236, a237)?
                }, v4))
            } else {
                f74(v2, v5, v4)
            }
        }
        Adt2::C5 => {
            if v6 {
                Ok(Adt3::C0(match v2.clone() {
                    Adt3::C0(h238, _, _) => {
                        h238
                    }
                }, match v2 {
                    Adt3::C0(_, h239, _) => {
                        h239
                    }
                }, v4))
            } else {
                f74(v2, v5, v4)
            }
        }
        Adt2::C6(r246) => {
            let v12 = (*r246).clone();
            let v13: bool = f75(v1, v0, match v2.clone() {
                Adt3::C0(_, h240, _) => {
                    h240
                }
            }, v4.clone(), Adt4::C0(v5, true, v12.clone()))?;
            Ok(Adt3::C0(match v2.clone() {
                Adt3::C0(h241, _, _) => {
                    h241
                }
            }, match v2 {
                Adt3::C0(_, h242, _) => {
                    h242
                }
            }, List::cons(Adt4::C0(v5, v13, v12), v4)))
        }
    }
}

pub fn f57(v0: Fn8, p251: u64, p252: List<Bytes>) -> (List<Bytes>, bool) {
    let m250 = (p251, p252);
    {
        let mut m250 = m250;
        loop {
            let step = {
                let (v1, v2) = m250;
                let m247 = v1;
                if m247 == 0 {
                    Err::<(u64, List<Bytes>), (List<Bytes>, bool)>((v2, false))
                } else {
                    let v3 = m247 - 1;
                    let m249 = {
                        let c248: Fn8 = v0.clone();
                        c248.apply(v2.clone())
                    };
                    match m249 {
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
                Ok(next) => { m250 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f58(v0: List<Bytes>) -> Option<List<Bytes>> {
    f76(v0)
}

pub fn f59(v0: Fn9, p256: List<Bytes>, p257: List<Bytes>) -> List<Bytes> {
    let m255 = (p256, p257);
    {
        let mut m255 = m255;
        loop {
            let step = {
                let (v1, v2) = m255;
                let m253 = v2;
                match m253.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c254: Fn9 = v0.clone();
                            c254.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m255 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f60(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f61(v0: Bytes, v1: u64) -> Bytes {
    let m261 = {
        let a258 = v0;
        let a259 = v1;
        let a260 = 1u64;
        slice_bytes(a258, a259, a260)
    };
    match m261 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f62(v0: Fn10, p266: u64, p267: (u64, Bytes)) -> ((u64, Bytes), bool) {
    let m265 = (p266, p267);
    {
        let mut m265 = m265;
        loop {
            let step = {
                let (v1, v2) = m265;
                let m262 = v1;
                if m262 == 0 {
                    Err::<(u64, (u64, Bytes)), ((u64, Bytes), bool)>((v2, false))
                } else {
                    let v3 = m262 - 1;
                    let m264 = {
                        let c263: Fn10 = v0.clone();
                        c263.apply(v2.clone())
                    };
                    match m264 {
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
                Ok(next) => { m265 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f63(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f77(v1, v0, v2)
}

pub fn f64(v0: Bytes, v1: u64) -> Option<u8> {
    let a268 = v0;
    let a269 = v1;
    index_bytes(a268, a269)
}

pub fn f65(v0: Fn11, p274: u64, p275: u64) -> R<(u64, bool)> {
    let m273 = (p274, p275);
    {
        let mut m273 = m273;
        loop {
            let step = {
                let (v1, v2) = m273;
                let m270 = v1;
                if m270 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m270 - 1;
                    let m272 = {
                        let c271: Fn11 = v0.clone();
                        c271.apply(v2)?
                    };
                    match m272 {
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
                Ok(next) => { m273 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f66(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f78(v0, v1)
}

pub fn f67(v0: Bytes, v1: u64, v2: Adt5) -> R<Option<Adt5>> {
    let v3: u64 = match v2.clone() {
        Adt5::C0(h276, _) => {
            h276
        }
    };
    let a277 = v3;
    let a278 = v1;
    if nat_lt(a277, a278) {
        let v4: u64 = f79(v0.clone(), v3)?;
        let a279 = v4;
        let a280 = 32u64;
        if if nat_lt(a279, a280) {
            true
        } else {
            let a281 = v4;
            let a282 = 34u64;
            if nat_eq(a281, a282) {
                true
            } else {
                let a283 = v4;
                let a284 = 92u64;
                nat_eq(a283, a284)
            }
        } {
            Ok(Some(Adt5::C0({
                let a285 = v3;
                let a286 = 1u64;
                nat_add(a285, a286)?
            }, List::cons(f80(v4), match v2 {
                Adt5::C0(_, h287) => {
                    h287
                }
            }))))
        } else {
            let v5: u64 = f81(v0.clone(), v1, v3)?;
            Ok(Some(Adt5::C0(v5, List::cons(f82(v0, v3, {
                let a288 = v5;
                let a289 = v3;
                nat_sub(a288, a289)
            }), match v2 {
                Adt5::C0(_, h290) => {
                    h290
                }
            }))))
        }
    } else {
        Ok(None::<Adt5>)
    }
}

pub fn f68(v0: u64) -> Bytes {
    f82(Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff]), v0, 1u64)
}

pub fn f69(v0: Bytes) -> u64 {
    let a291 = v0.clone();
    let a292 = Bytes::lit(&[0x6e, 0x61, 0x6d, 0x65]);
    if equal(a291, a292) {
        0u64
    } else {
        let a293 = v0.clone();
        let a294 = Bytes::lit(&[0x74, 0x79, 0x70, 0x65, 0x5f, 0x70, 0x61, 0x72, 0x61, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x73]);
        if equal(a293, a294) {
            1u64
        } else {
            let a295 = v0.clone();
            let a296 = Bytes::lit(&[0x70, 0x61, 0x72, 0x61, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x73]);
            if equal(a295, a296) {
                2u64
            } else {
                let a297 = v0.clone();
                let a298 = Bytes::lit(&[0x74, 0x79, 0x70, 0x65]);
                if equal(a297, a298) {
                    3u64
                } else {
                    let a299 = v0.clone();
                    let a300 = Bytes::lit(&[0x72, 0x65, 0x73, 0x75, 0x6c, 0x74]);
                    if equal(a299, a300) {
                        4u64
                    } else {
                        let a301 = v0.clone();
                        let a302 = Bytes::lit(&[0x73, 0x74, 0x61, 0x74, 0x65, 0x6d, 0x65, 0x6e, 0x74]);
                        if equal(a301, a302) {
                            5u64
                        } else {
                            let a303 = v0.clone();
                            let a304 = Bytes::lit(&[0x66, 0x69, 0x65, 0x6c, 0x64, 0x73]);
                            if equal(a303, a304) {
                                6u64
                            } else {
                                let a305 = v0.clone();
                                let a306 = Bytes::lit(&[0x63, 0x6f, 0x6e, 0x73, 0x74, 0x72, 0x75, 0x63, 0x74, 0x6f, 0x72, 0x73]);
                                if equal(a305, a306) {
                                    7u64
                                } else {
                                    let a307 = v0.clone();
                                    let a308 = Bytes::lit(&[0x61, 0x72, 0x67, 0x75, 0x6d, 0x65, 0x6e, 0x74, 0x73]);
                                    if equal(a307, a308) {
                                        8u64
                                    } else {
                                        let a309 = v0.clone();
                                        let a310 = Bytes::lit(&[0x62, 0x6f, 0x64, 0x79]);
                                        if equal(a309, a310) {
                                            9u64
                                        } else {
                                            let a311 = v0.clone();
                                            let a312 = Bytes::lit(&[0x70, 0x72, 0x6f, 0x6f, 0x66]);
                                            if equal(a311, a312) {
                                                10u64
                                            } else {
                                                let a313 = v0;
                                                let a314 = Bytes::lit(&[0x61, 0x78, 0x69, 0x6f, 0x6d, 0x73]);
                                                if equal(a313, a314) {
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

pub fn f70(v0: Fn12, p318: List<List<(Bytes, Adt2)>>, p319: List<List<(Bytes, Adt2)>>) -> List<List<(Bytes, Adt2)>> {
    let m317 = (p318, p319);
    {
        let mut m317 = m317;
        loop {
            let step = {
                let (v1, v2) = m317;
                let m315 = v2;
                match m315.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Adt2)>>, List<List<(Bytes, Adt2)>>), List<List<(Bytes, Adt2)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Adt2)>>, List<List<(Bytes, Adt2)>>), List<List<(Bytes, Adt2)>>>(({
                            let c316: Fn12 = v0.clone();
                            c316.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m317 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f71(v0: List<List<(Bytes, Adt2)>>, v1: List<(Bytes, Adt2)>) -> List<List<(Bytes, Adt2)>> {
    List::cons(v1, v0)
}

pub fn f72(v0: List<List<(Bytes, Adt2)>>) -> R<Option<List<List<(Bytes, Adt2)>>>> {
    let a321 = {
        let a320 = v0.clone();
        length_list(a320)
    };
    let a322 = 1u64;
    if nat_le(a321, a322) {
        Ok(None::<List<List<(Bytes, Adt2)>>>)
    } else {
        Ok(Some(f83(v0)?))
    }
}

pub fn f73(v0: Bytes) -> R<u64> {
    let (_, h323) = {
        let (h324, _) = f84(Fn13::F85(v0.clone()), {
            let a325 = v0;
            length_bytes(a325)
        }, (0u64, 0u64))?;
        h324
    };
    Ok(h323)
}

pub fn f74(v0: Adt3, v1: u64, v2: List<Adt4>) -> R<Adt3> {
    Ok(Adt3::C0(f86(match v0 {
        Adt3::C0(h326, _, _) => {
            h326
        }
    }, v1)?, v1, v2))
}

pub fn f75(v0: u64, v1: u64, v2: u64, v3: List<Adt4>, v4: Adt4) -> R<bool> {
    let v5: Adt6 = {
        let (h327, _) = f87(Fn14::F88, {
            let a328 = v0;
            let a329 = 1u64;
            nat_add(a328, a329)?
        }, Adt6::C0(0u64, {
            let a330 = v1;
            let a331 = v2;
            nat_sub(a330, a331)
        }, {
            let a332 = v1;
            let a333 = v2;
            nat_lt(a332, a333)
        }, List::cons(v4, List::<Adt4>::nil()), v3, None::<bool>))?;
        h327
    };
    let m335 = match v5 {
        Adt6::C0(_, _, _, _, _, h334) => {
            h334
        }
    };
    match m335 {
        None => {
            Ok(false)
        }
        Some(v6) => {
            Ok(v6)
        }
    }
}

pub fn f76(v0: List<Bytes>) -> Option<List<Bytes>> {
    let m336 = v0.clone();
    match m336.uncons() {
        None => {
            None::<List<Bytes>>
        }
        Some((_, v2)) => {
            let m337 = v2;
            match m337.uncons() {
                None => {
                    None::<List<Bytes>>
                }
                Some((_, _)) => {
                    let v5: (Option<Bytes>, List<Bytes>) = f89(Fn15::F90, (None::<Bytes>, List::<Bytes>::nil()), v0);
                    let m339 = {
                        let (h338, _) = v5;
                        h338
                    };
                    match m339 {
                        None => {
                            Some(f36({
                                let (_, h341) = v5;
                                h341
                            }))
                        }
                        Some(v6) => {
                            Some(f36(List::cons(v6, {
                                let (_, h340) = v5;
                                h340
                            })))
                        }
                    }
                }
            }
        }
    }
}

pub fn f77(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h342, _) = v2;
        h342
    };
    let a343 = v3;
    let a344 = 0u64;
    if nat_eq(a343, a344) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a345 = v3;
            let a346 = v1;
            let a347 = 0u64;
            nat_quot(a345, a346, a347)
        }, {
            let a352 = f61(v0, {
                let a348 = v3;
                let a349 = v1;
                let a350 = 0u64;
                nat_rem(a348, a349, a350)
            });
            let a353 = {
                let (_, h351) = v2;
                h351
            };
            append_bytes(a352, a353)
        }))
    }
}

pub fn f78(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f91(f40(v0, v1)?) {
        Ok(Some({
            let a354 = v1;
            let a355 = 1u64;
            nat_add(a354, a355)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f79(v0: Bytes, v1: u64) -> R<u64> {
    let m356 = f64(v0, v1);
    match m356 {
        None => {
            Ok(0u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a401 = {
                let a377 = {
                    let a365 = {
                        let a359 = {
                            let a357 = v3;
                            let a358 = 128u8;
                            fixed_u8::bit_and(a357, a358)
                        };
                        let a360 = 128u8;
                        if equal(a359, a360) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a366 = {
                        let a363 = {
                            let a361 = v3;
                            let a362 = 64u8;
                            fixed_u8::bit_and(a361, a362)
                        };
                        let a364 = 64u8;
                        if equal(a363, a364) {
                            64u64
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
                            let a368 = 32u8;
                            fixed_u8::bit_and(a367, a368)
                        };
                        let a370 = 32u8;
                        if equal(a369, a370) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a376 = {
                        let a373 = {
                            let a371 = v3;
                            let a372 = 16u8;
                            fixed_u8::bit_and(a371, a372)
                        };
                        let a374 = 16u8;
                        if equal(a373, a374) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a375, a376)?
                };
                nat_add(a377, a378)?
            };
            let a402 = {
                let a399 = {
                    let a387 = {
                        let a381 = {
                            let a379 = v3;
                            let a380 = 8u8;
                            fixed_u8::bit_and(a379, a380)
                        };
                        let a382 = 8u8;
                        if equal(a381, a382) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a388 = {
                        let a385 = {
                            let a383 = v3;
                            let a384 = 4u8;
                            fixed_u8::bit_and(a383, a384)
                        };
                        let a386 = 4u8;
                        if equal(a385, a386) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a387, a388)?
                };
                let a400 = {
                    let a397 = {
                        let a391 = {
                            let a389 = v3;
                            let a390 = 2u8;
                            fixed_u8::bit_and(a389, a390)
                        };
                        let a392 = 2u8;
                        if equal(a391, a392) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a398 = {
                        let a395 = {
                            let a393 = v3;
                            let a394 = 1u8;
                            fixed_u8::bit_and(a393, a394)
                        };
                        let a396 = 1u8;
                        if equal(a395, a396) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a397, a398)?
                };
                nat_add(a399, a400)?
            };
            nat_add(a401, a402)
        }
    }
}

pub fn f80(v0: u64) -> Bytes {
    let a403 = v0;
    let a404 = 34u64;
    if nat_eq(a403, a404) {
        f46(Str::lit("\\\""))
    } else {
        let a405 = v0;
        let a406 = 92u64;
        if nat_eq(a405, a406) {
            f46(Str::lit("\\\\"))
        } else {
            let a407 = v0;
            let a408 = 8u64;
            if nat_eq(a407, a408) {
                f46(Str::lit("\\b"))
            } else {
                let a409 = v0;
                let a410 = 12u64;
                if nat_eq(a409, a410) {
                    f46(Str::lit("\\f"))
                } else {
                    let a411 = v0;
                    let a412 = 10u64;
                    if nat_eq(a411, a412) {
                        f46(Str::lit("\\n"))
                    } else {
                        let a413 = v0;
                        let a414 = 13u64;
                        if nat_eq(a413, a414) {
                            f46(Str::lit("\\r"))
                        } else {
                            let a415 = v0;
                            let a416 = 9u64;
                            if nat_eq(a415, a416) {
                                f46(Str::lit("\\t"))
                            } else {
                                f45(f46(Str::lit("\\u00")), f92(v0))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f81(v0: Bytes, v1: u64, v2: u64) -> R<u64> {
    let (h417, _) = f65(Fn11::F93(v1, v0), {
        let a418 = v1;
        let a419 = v2;
        nat_sub(a418, a419)
    }, v2)?;
    Ok(h417)
}

pub fn f82(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m423 = {
        let a420 = v0;
        let a421 = v1;
        let a422 = v2;
        slice_bytes(a420, a421, a422)
    };
    match m423 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f83(v0: List<List<(Bytes, Adt2)>>) -> R<List<List<(Bytes, Adt2)>>> {
    Ok(f51({
        let (_, h424) = {
            let (h425, _) = f94(Fn16::F95, {
                let a427 = {
                    let a426 = v0.clone();
                    length_list(a426)
                };
                let a428 = 1u64;
                nat_add(a427, a428)?
            }, (v0, List::<List<(Bytes, Adt2)>>::nil()))?;
            h425
        };
        h424
    }))
}

pub fn f84(v0: Fn13, p433: u64, p434: (u64, u64)) -> R<((u64, u64), bool)> {
    let m432 = (p433, p434);
    {
        let mut m432 = m432;
        loop {
            let step = {
                let (v1, v2) = m432;
                let m429 = v1;
                if m429 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m429 - 1;
                    let m431 = {
                        let c430: Fn13 = v0.clone();
                        c430.apply(v2)?
                    };
                    match m431 {
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
                Ok(next) => { m432 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f85(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f96(v0, v1)
}

pub fn f86(v0: List<Bytes>, v1: u64) -> R<List<Bytes>> {
    let v2: u64 = {
        let a435 = v1;
        let a436 = 256u64;
        let a437 = 0u64;
        nat_quot(a435, a436, a437)
    };
    let v3: List<Bytes> = {
        let (_, h438) = {
            let (h439, _) = f97(Fn17::F98(v2), {
                let a440 = v2;
                let a441 = 1u64;
                nat_add(a440, a441)?
            }, (0u64, List::cons({
                let a442 = Str::lit("\u{a}");
                utf8_encode(a442)
            }, v0)))?;
            h439
        };
        h438
    };
    Ok(List::cons(f82(Bytes::lit(&[0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20]), 0u64, {
        let a443 = v1;
        let a444 = 256u64;
        let a445 = 0u64;
        nat_rem(a443, a444, a445)
    }), v3))
}

pub fn f87(v0: Fn14, p450: u64, p451: Adt6) -> R<(Adt6, bool)> {
    let m449 = (p450, p451);
    {
        let mut m449 = m449;
        loop {
            let step = {
                let (v1, v2) = m449;
                let m446 = v1;
                if m446 == 0 {
                    Err::<(u64, Adt6), R<(Adt6, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m446 - 1;
                    let m448 = {
                        let c447: Fn14 = v0.clone();
                        c447.apply(v2.clone())?
                    };
                    match m448 {
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
                Ok(next) => { m449 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f88(v0: Adt6) -> R<Option<Adt6>> {
    f99(v0)
}

pub fn f89(v0: Fn15, p455: (Option<Bytes>, List<Bytes>), p456: List<Bytes>) -> (Option<Bytes>, List<Bytes>) {
    let m454 = (p455, p456);
    {
        let mut m454 = m454;
        loop {
            let step = {
                let (v1, v2) = m454;
                let m452 = v2;
                match m452.uncons() {
                    None => {
                        Err::<((Option<Bytes>, List<Bytes>), List<Bytes>), (Option<Bytes>, List<Bytes>)>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<((Option<Bytes>, List<Bytes>), List<Bytes>), (Option<Bytes>, List<Bytes>)>(({
                            let c453: Fn15 = v0.clone();
                            c453.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m454 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f90(v0: (Option<Bytes>, List<Bytes>), v1: Bytes) -> (Option<Bytes>, List<Bytes>) {
    f100(v0, v1)
}

pub fn f91(v0: u64) -> bool {
    if f39(v0) {
        true
    } else {
        let a457 = 47u64;
        let a458 = v0;
        if if nat_lt(a457, a458) {
            let a459 = v0;
            let a460 = 58u64;
            nat_lt(a459, a460)
        } else {
            false
        } {
            true
        } else {
            let a461 = v0;
            let a462 = 46u64;
            if nat_eq(a461, a462) {
                true
            } else {
                let a463 = v0;
                let a464 = 39u64;
                if nat_eq(a463, a464) {
                    true
                } else {
                    let a465 = v0;
                    let a466 = 47u64;
                    if nat_eq(a465, a466) {
                        true
                    } else {
                        let a467 = v0;
                        let a468 = 45u64;
                        nat_eq(a467, a468)
                    }
                }
            }
        }
    }
}

pub fn f92(v0: u64) -> Bytes {
    f45(f101(v0, 16u64), f102(v0))
}

pub fn f93(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f103(v1, v0, v2)
}

pub fn f94(v0: Fn16, p473: u64, p474: (List<List<(Bytes, Adt2)>>, List<List<(Bytes, Adt2)>>)) -> R<((List<List<(Bytes, Adt2)>>, List<List<(Bytes, Adt2)>>), bool)> {
    let m472 = (p473, p474);
    {
        let mut m472 = m472;
        loop {
            let step = {
                let (v1, v2) = m472;
                let m469 = v1;
                if m469 == 0 {
                    Err::<(u64, (List<List<(Bytes, Adt2)>>, List<List<(Bytes, Adt2)>>)), R<((List<List<(Bytes, Adt2)>>, List<List<(Bytes, Adt2)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m469 - 1;
                    let m471 = {
                        let c470: Fn16 = v0.clone();
                        c470.apply(v2.clone())?
                    };
                    match m471 {
                        None => {
                            Err::<(u64, (List<List<(Bytes, Adt2)>>, List<List<(Bytes, Adt2)>>)), R<((List<List<(Bytes, Adt2)>>, List<List<(Bytes, Adt2)>>), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (List<List<(Bytes, Adt2)>>, List<List<(Bytes, Adt2)>>)), R<((List<List<(Bytes, Adt2)>>, List<List<(Bytes, Adt2)>>), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m472 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f95(v0: (List<List<(Bytes, Adt2)>>, List<List<(Bytes, Adt2)>>)) -> R<Option<(List<List<(Bytes, Adt2)>>, List<List<(Bytes, Adt2)>>)>> {
    f104(v0)
}

pub fn f96(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m476 = f64(v0, {
        let (h475, _) = v1;
        h475
    });
    match m476 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            let v4: u64 = {
                let v3: u8 = v2;
                let a521 = {
                    let a497 = {
                        let a485 = {
                            let a479 = {
                                let a477 = v3;
                                let a478 = 128u8;
                                fixed_u8::bit_and(a477, a478)
                            };
                            let a480 = 128u8;
                            if equal(a479, a480) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a486 = {
                            let a483 = {
                                let a481 = v3;
                                let a482 = 64u8;
                                fixed_u8::bit_and(a481, a482)
                            };
                            let a484 = 64u8;
                            if equal(a483, a484) {
                                64u64
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
                                let a488 = 32u8;
                                fixed_u8::bit_and(a487, a488)
                            };
                            let a490 = 32u8;
                            if equal(a489, a490) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a496 = {
                            let a493 = {
                                let a491 = v3;
                                let a492 = 16u8;
                                fixed_u8::bit_and(a491, a492)
                            };
                            let a494 = 16u8;
                            if equal(a493, a494) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a495, a496)?
                    };
                    nat_add(a497, a498)?
                };
                let a522 = {
                    let a519 = {
                        let a507 = {
                            let a501 = {
                                let a499 = v3;
                                let a500 = 8u8;
                                fixed_u8::bit_and(a499, a500)
                            };
                            let a502 = 8u8;
                            if equal(a501, a502) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a508 = {
                            let a505 = {
                                let a503 = v3;
                                let a504 = 4u8;
                                fixed_u8::bit_and(a503, a504)
                            };
                            let a506 = 4u8;
                            if equal(a505, a506) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a507, a508)?
                    };
                    let a520 = {
                        let a517 = {
                            let a511 = {
                                let a509 = v3;
                                let a510 = 2u8;
                                fixed_u8::bit_and(a509, a510)
                            };
                            let a512 = 2u8;
                            if equal(a511, a512) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a518 = {
                            let a515 = {
                                let a513 = v3;
                                let a514 = 1u8;
                                fixed_u8::bit_and(a513, a514)
                            };
                            let a516 = 1u8;
                            if equal(a515, a516) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a517, a518)?
                    };
                    nat_add(a519, a520)?
                };
                nat_add(a521, a522)?
            };
            let a523 = v4;
            let a524 = 128u64;
            if nat_lt(a523, a524) {
                Ok(Some(({
                    let a526 = {
                        let (h525, _) = v1;
                        h525
                    };
                    let a527 = 1u64;
                    nat_add(a526, a527)?
                }, {
                    let a529 = {
                        let (_, h528) = v1;
                        h528
                    };
                    let a530 = 1u64;
                    nat_add(a529, a530)?
                })))
            } else {
                let a531 = v4;
                let a532 = 192u64;
                if nat_lt(a531, a532) {
                    Ok(Some(({
                        let a534 = {
                            let (h533, _) = v1;
                            h533
                        };
                        let a535 = 1u64;
                        nat_add(a534, a535)?
                    }, {
                        let (_, h536) = v1;
                        h536
                    })))
                } else {
                    Ok(Some(({
                        let a538 = {
                            let (h537, _) = v1;
                            h537
                        };
                        let a539 = 1u64;
                        nat_add(a538, a539)?
                    }, {
                        let a541 = {
                            let (_, h540) = v1;
                            h540
                        };
                        let a542 = 1u64;
                        nat_add(a541, a542)?
                    })))
                }
            }
        }
    }
}

pub fn f97(v0: Fn17, p547: u64, p548: (u64, List<Bytes>)) -> R<((u64, List<Bytes>), bool)> {
    let m546 = (p547, p548);
    {
        let mut m546 = m546;
        loop {
            let step = {
                let (v1, v2) = m546;
                let m543 = v1;
                if m543 == 0 {
                    Err::<(u64, (u64, List<Bytes>)), R<((u64, List<Bytes>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m543 - 1;
                    let m545 = {
                        let c544: Fn17 = v0.clone();
                        c544.apply(v2.clone())?
                    };
                    match m545 {
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
                Ok(next) => { m546 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f98(v0: u64, v1: (u64, List<Bytes>)) -> R<Option<(u64, List<Bytes>)>> {
    f105(v0, v1)
}

pub fn f99(v0: Adt6) -> R<Option<Adt6>> {
    let m550 = match v0.clone() {
        Adt6::C0(_, _, _, _, _, h549) => {
            h549
        }
    };
    match m550 {
        None => {
            if if match v0.clone() {
                Adt6::C0(_, _, h551, _, _, _) => {
                    h551
                }
            } {
                true
            } else {
                let a554 = match v0.clone() {
                    Adt6::C0(_, h552, _, _, _, _) => {
                        h552
                    }
                };
                let a555 = match v0.clone() {
                    Adt6::C0(h553, _, _, _, _, _) => {
                        h553
                    }
                };
                nat_lt(a554, a555)
            } {
                Ok(Some(f106(v0.clone(), match v0.clone() {
                    Adt6::C0(h556, _, _, _, _, _) => {
                        h556
                    }
                }, match v0.clone() {
                    Adt6::C0(_, _, _, h557, _, _) => {
                        h557
                    }
                }, match v0 {
                    Adt6::C0(_, _, _, _, h558, _) => {
                        h558
                    }
                }, Some(false))))
            } else {
                let m560 = match v0.clone() {
                    Adt6::C0(_, _, _, h559, _, _) => {
                        h559
                    }
                };
                match m560.uncons() {
                    None => {
                        let m563 = match v0.clone() {
                            Adt6::C0(_, _, _, _, h562, _) => {
                                h562
                            }
                        };
                        match m563.uncons() {
                            None => {
                                Ok(Some(f106(v0.clone(), match v0 {
                                    Adt6::C0(h564, _, _, _, _, _) => {
                                        h564
                                    }
                                }, List::<Adt4>::nil(), List::<Adt4>::nil(), Some(true))))
                            }
                            Some((v4, v5)) => {
                                Ok(Some(f107(v0, v4, List::<Adt4>::nil(), v5)?))
                            }
                        }
                    }
                    Some((v2, v3)) => {
                        Ok(Some(f107(v0.clone(), v2, v3, match v0 {
                            Adt6::C0(_, _, _, _, h561, _) => {
                                h561
                            }
                        })?))
                    }
                }
            }
        }
        Some(_) => {
            Ok(None::<Adt6>)
        }
    }
}

pub fn f100(v0: (Option<Bytes>, List<Bytes>), v1: Bytes) -> (Option<Bytes>, List<Bytes>) {
    let m566 = {
        let (h565, _) = v0;
        h565
    };
    match m566 {
        None => {
            (Some(v1), {
                let (_, h567) = v0;
                h567
            })
        }
        Some(v2) => {
            (None::<Bytes>, List::cons({
                let a568 = v2;
                let a569 = v1;
                append_bytes(a568, a569)
            }, {
                let (_, h570) = v0;
                h570
            }))
        }
    }
}

pub fn f101(v0: u64, v1: u64) -> Bytes {
    f102({
        let a571 = v0;
        let a572 = v1;
        let a573 = 0u64;
        nat_quot(a571, a572, a573)
    })
}

pub fn f102(v0: u64) -> Bytes {
    f82(Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66]), {
        let a574 = v0;
        let a575 = 16u64;
        let a576 = 0u64;
        nat_rem(a574, a575, a576)
    }, 1u64)
}

pub fn f103(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let a577 = v2;
    let a578 = v1;
    if nat_lt(a577, a578) {
        let v3: u64 = f79(v0, v2)?;
        let a579 = v3;
        let a580 = 32u64;
        if nat_lt(a579, a580) {
            Ok(None::<u64>)
        } else {
            let a581 = v3;
            let a582 = 34u64;
            if nat_eq(a581, a582) {
                Ok(None::<u64>)
            } else {
                let a583 = v3;
                let a584 = 92u64;
                if nat_eq(a583, a584) {
                    Ok(None::<u64>)
                } else {
                    Ok(Some({
                        let a585 = v2;
                        let a586 = 1u64;
                        nat_add(a585, a586)?
                    }))
                }
            }
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f104(v0: (List<List<(Bytes, Adt2)>>, List<List<(Bytes, Adt2)>>)) -> R<Option<(List<List<(Bytes, Adt2)>>, List<List<(Bytes, Adt2)>>)>> {
    let v1: List<List<(Bytes, Adt2)>> = {
        let (h587, _) = v0;
        h587
    };
    let v2: List<List<(Bytes, Adt2)>> = {
        let (_, h588) = v0;
        h588
    };
    let m589 = v1;
    match m589.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Adt2)>>, List<List<(Bytes, Adt2)>>)>)
        }
        Some((v3, v4)) => {
            let m590 = v4;
            match m590.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Adt2)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f108(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f105(v0: u64, v1: (u64, List<Bytes>)) -> R<Option<(u64, List<Bytes>)>> {
    let a592 = {
        let (h591, _) = v1.clone();
        h591
    };
    let a593 = v0;
    if nat_lt(a592, a593) {
        Ok(Some(({
            let a595 = {
                let (h594, _) = v1;
                h594
            };
            let a596 = 1u64;
            nat_add(a595, a596)?
        }, List::cons(Bytes::lit(&[0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20]), {
            let (_, h597) = v1;
            h597
        }))))
    } else {
        Ok(None::<(u64, List<Bytes>)>)
    }
}

pub fn f106(v0: Adt6, v1: u64, v2: List<Adt4>, v3: List<Adt4>, v4: Option<bool>) -> Adt6 {
    Adt6::C0(v1, match v0.clone() {
        Adt6::C0(_, h598, _, _, _, _) => {
            h598
        }
    }, match v0 {
        Adt6::C0(_, _, h599, _, _, _) => {
            h599
        }
    }, v2, v3, v4)
}

pub fn f107(v0: Adt6, v1: Adt4, v2: List<Adt4>, v3: List<Adt4>) -> R<Adt6> {
    let v4: u64 = match v1.clone() {
        Adt4::C0(h600, _, _) => {
            h600
        }
    };
    let v5: bool = match v1.clone() {
        Adt4::C0(_, h601, _) => {
            h601
        }
    };
    let v6: u64 = match v0.clone() {
        Adt6::C0(h602, _, _, _, _, _) => {
            h602
        }
    };
    let m604 = match v1 {
        Adt4::C0(_, _, h603) => {
            h603
        }
    };
    match m604 {
        Adt2::C0 => {
            Ok(f106(v0, v6, v2, v3, None::<bool>))
        }
        Adt2::C1(v7) => {
            Ok(f106(v0, {
                let a605 = v6;
                let a606 = f73(v7)?;
                nat_add(a605, a606)?
            }, v2, v3, None::<bool>))
        }
        Adt2::C2(r611, r612) => {
            let v8 = (*r611).clone();
            let v9 = (*r612).clone();
            Ok(f106(v0, v6, List::cons(Adt4::C0(v4, v5, v8), List::cons(Adt4::C0(v4, v5, v9), v2)), v3, None::<bool>))
        }
        Adt2::C3(v10, r613) => {
            let v11 = (*r613).clone();
            Ok(f106(v0, v6, List::cons(Adt4::C0({
                let a607 = v4;
                let a608 = v10;
                nat_add(a607, a608)?
            }, v5, v11), v2), v3, None::<bool>))
        }
        Adt2::C4 => {
            if v5 {
                Ok(f106(v0, {
                    let a609 = v6;
                    let a610 = 1u64;
                    nat_add(a609, a610)?
                }, v2, v3, None::<bool>))
            } else {
                Ok(f106(v0, v6, v2, v3, Some(true)))
            }
        }
        Adt2::C5 => {
            if v5 {
                Ok(f106(v0, v6, v2, v3, None::<bool>))
            } else {
                Ok(f106(v0, v6, v2, v3, Some(true)))
            }
        }
        Adt2::C6(r614) => {
            let v12 = (*r614).clone();
            Ok(f106(v0, v6, List::cons(Adt4::C0(v4, true, v12), v2), v3, None::<bool>))
        }
    }
}

pub fn f108(v0: List<(Bytes, Adt2)>, v1: List<(Bytes, Adt2)>) -> R<List<(Bytes, Adt2)>> {
    let v2: u64 = {
        let a619 = {
            let a617 = {
                let a615 = v0.clone();
                length_list(a615)
            };
            let a618 = {
                let a616 = v1.clone();
                length_list(a616)
            };
            nat_add(a617, a618)?
        };
        let a620 = 1u64;
        nat_add(a619, a620)?
    };
    f109({
        let (_, h621) = {
            let (_, h622) = {
                let (h623, _) = f110(Fn18::F111, v2, (v0, (v1, List::<(Bytes, Adt2)>::nil())));
                h623
            };
            h622
        };
        h621
    })
}

pub fn f109(v0: List<(Bytes, Adt2)>) -> R<List<(Bytes, Adt2)>> {
    f29(Fn3::F112, List::<(Bytes, Adt2)>::nil(), v0)
}

pub fn f110(v0: Fn18, p628: u64, p629: (List<(Bytes, Adt2)>, (List<(Bytes, Adt2)>, List<(Bytes, Adt2)>))) -> ((List<(Bytes, Adt2)>, (List<(Bytes, Adt2)>, List<(Bytes, Adt2)>)), bool) {
    let m627 = (p628, p629);
    {
        let mut m627 = m627;
        loop {
            let step = {
                let (v1, v2) = m627;
                let m624 = v1;
                if m624 == 0 {
                    Err::<(u64, (List<(Bytes, Adt2)>, (List<(Bytes, Adt2)>, List<(Bytes, Adt2)>))), ((List<(Bytes, Adt2)>, (List<(Bytes, Adt2)>, List<(Bytes, Adt2)>)), bool)>((v2, false))
                } else {
                    let v3 = m624 - 1;
                    let m626 = {
                        let c625: Fn18 = v0.clone();
                        c625.apply(v2.clone())
                    };
                    match m626 {
                        None => {
                            Err::<(u64, (List<(Bytes, Adt2)>, (List<(Bytes, Adt2)>, List<(Bytes, Adt2)>))), ((List<(Bytes, Adt2)>, (List<(Bytes, Adt2)>, List<(Bytes, Adt2)>)), bool)>((v2, true))
                        }
                        Some(v4) => {
                            Ok::<(u64, (List<(Bytes, Adt2)>, (List<(Bytes, Adt2)>, List<(Bytes, Adt2)>))), ((List<(Bytes, Adt2)>, (List<(Bytes, Adt2)>, List<(Bytes, Adt2)>)), bool)>((v3, v4))
                        }
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

pub fn f111(v0: (List<(Bytes, Adt2)>, (List<(Bytes, Adt2)>, List<(Bytes, Adt2)>))) -> Option<(List<(Bytes, Adt2)>, (List<(Bytes, Adt2)>, List<(Bytes, Adt2)>))> {
    f113(v0)
}

pub fn f112(v0: List<(Bytes, Adt2)>, v1: (Bytes, Adt2)) -> List<(Bytes, Adt2)> {
    List::cons(v1, v0)
}

pub fn f113(v0: (List<(Bytes, Adt2)>, (List<(Bytes, Adt2)>, List<(Bytes, Adt2)>))) -> Option<(List<(Bytes, Adt2)>, (List<(Bytes, Adt2)>, List<(Bytes, Adt2)>))> {
    let v1: List<(Bytes, Adt2)> = {
        let (h630, _) = v0.clone();
        h630
    };
    let v2: List<(Bytes, Adt2)> = {
        let (h631, _) = {
            let (_, h632) = v0.clone();
            h632
        };
        h631
    };
    let v3: List<(Bytes, Adt2)> = {
        let (_, h633) = {
            let (_, h634) = v0;
            h634
        };
        h633
    };
    let m635 = v1.clone();
    match m635.uncons() {
        None => {
            let m636 = v2;
            match m636.uncons() {
                None => {
                    None::<(List<(Bytes, Adt2)>, (List<(Bytes, Adt2)>, List<(Bytes, Adt2)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Adt2)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m637 = v2.clone();
            match m637.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Adt2)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f114({
                        let (h638, _) = v8.clone();
                        h638
                    }, {
                        let (h639, _) = v6.clone();
                        h639
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

pub fn f114(v0: Bytes, v1: Bytes) -> bool {
    let a644 = {
        let a640 = v0;
        let a641 = v1;
        compare_bytes(a640, a641)
    };
    let a645 = {
        let a642 = Bytes::lit(&[]);
        let a643 = Bytes::lit(&[0x01]);
        compare_bytes(a642, a643)
    };
    equal(a644, a645)
}

pub fn run(p0: Adt0) -> R<Result<Bytes, Adt1>> {
    f0(p0)
}
