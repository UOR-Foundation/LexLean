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
    C0(Bytes, List<Bytes>, Adt1, (Option<Bytes>, Bytes), Option<Adt3>, Option<(Option<Bytes>, Bytes)>, List<Bytes>),
    C1(Bytes, List<Bytes>, Adt4, Option<Adt5>, Adt6, Adt6, (Option<Bytes>, Bytes), Option<(Option<Bytes>, Bytes)>, bool, List<Bytes>),
    C2(Bytes, List<Bytes>, Adt9, Adt9, (Option<Bytes>, Bytes), (Option<Bytes>, Bytes), (Option<Bytes>, Bytes), Option<(Option<Bytes>, Bytes)>, List<Bytes>),
    C3(Bytes, List<Bytes>, Option<Adt4>, Adt9, Option<Adt6>, List<Adt4>, Adt10, Option<Adt12>, Adt4, List<Adt13>, bool, List<Bytes>),
    C4(Bytes, List<Bytes>, Adt14, Adt4, List<Bytes>),
    C5(Bytes, u64, Adt15, List<Bytes>),
    C6(Bytes, (Option<Bytes>, Bytes), (Option<Bytes>, Bytes), Adt4, u64, (Option<Bytes>, Bytes), Adt16, List<Bytes>),
    C7(Bytes, (Option<Bytes>, Bytes), Adt4, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(Bytes, Bytes, Adt2),
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
    C19(Bytes),
    C20(std::rc::Rc<Adt2>),
    C21(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C22(std::rc::Rc<Adt2>),
    C23(Option<Bytes>, Bytes, std::rc::Rc<Adt2>),
    C24(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C25(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C26(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C27(std::rc::Rc<Adt2>),
    C28,
    C29(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
}

#[derive(Clone)]
pub enum Adt3 {
    C0((Option<Bytes>, Bytes), (Option<Bytes>, Bytes)),
}

#[derive(Clone)]
pub enum Adt4 {
    C0((Option<Bytes>, Bytes), List<Adt2>),
}

#[derive(Clone)]
pub enum Adt5 {
    C0(Bytes, Adt2, Adt6),
}

#[derive(Clone)]
pub enum Adt6 {
    C0(Bytes),
    C1(Bytes),
    C2(Adt7, Bytes),
    C3(Bytes),
    C4(Bytes),
    C5(Adt8, std::rc::Rc<Adt6>, Adt2),
    C6(bool),
    C7,
    C8(Adt2),
    C9(std::rc::Rc<Adt6>, std::rc::Rc<Adt6>),
    C10(Option<Bytes>, Bytes, Adt2, std::rc::Rc<Adt6>),
    C11(Option<Bytes>, Bytes, Adt2, std::rc::Rc<Adt6>),
    C12(Option<Bytes>, Bytes, Adt2, Option<Bytes>, Bytes),
    C13(std::rc::Rc<Adt6>, Bytes),
    C14(Option<Bytes>, Bytes, Adt2, std::rc::Rc<Adt6>),
    C15(std::rc::Rc<Adt6>, std::rc::Rc<Adt6>, std::rc::Rc<Adt6>),
    C16(std::rc::Rc<Adt6>, std::rc::Rc<Adt6>),
    C17(std::rc::Rc<Adt6>, std::rc::Rc<Adt6>),
    C18(std::rc::Rc<Adt6>, std::rc::Rc<Adt6>),
    C19(std::rc::Rc<Adt6>, std::rc::Rc<Adt6>),
    C20(std::rc::Rc<Adt6>, std::rc::Rc<Adt6>),
    C21(std::rc::Rc<Adt6>, std::rc::Rc<Adt6>),
    C22(std::rc::Rc<Adt6>, std::rc::Rc<Adt6>),
    C23(std::rc::Rc<Adt6>, std::rc::Rc<Adt6>),
    C24(std::rc::Rc<Adt6>, std::rc::Rc<Adt6>),
    C25(std::rc::Rc<Adt6>, std::rc::Rc<Adt6>),
    C26(std::rc::Rc<Adt6>, std::rc::Rc<Adt6>),
    C27(std::rc::Rc<Adt6>),
    C28(std::rc::Rc<Adt6>, std::rc::Rc<Adt6>),
    C29(std::rc::Rc<Adt6>, std::rc::Rc<Adt6>),
    C30(Bytes, Adt2, std::rc::Rc<Adt6>),
    C31(Bytes, Adt2, std::rc::Rc<Adt6>, std::rc::Rc<Adt6>),
    C32(std::rc::Rc<Adt6>, std::rc::Rc<Adt6>),
    C33(std::rc::Rc<Adt6>),
    C34(std::rc::Rc<Adt6>),
    C35(List<(Bytes, Adt2)>, List<Bytes>, std::rc::Rc<Adt6>),
    C36(std::rc::Rc<Adt6>, std::rc::Rc<Adt6>),
    C37(Option<Bytes>, Bytes, Adt2),
    C38(Adt2, Adt2, std::rc::Rc<Adt6>),
    C39(Adt2, std::rc::Rc<Adt6>),
    C40(Adt2, std::rc::Rc<Adt6>, std::rc::Rc<Adt6>),
    C41(Option<Bytes>, Bytes, Adt2, std::rc::Rc<Adt6>, List<Bytes>),
    C42,
    C43(std::rc::Rc<Adt6>, std::rc::Rc<Adt6>),
    C44(Bytes, std::rc::Rc<Adt6>, std::rc::Rc<Adt6>),
    C45(Option<Bytes>, Bytes, List<Bytes>, std::rc::Rc<Adt6>, std::rc::Rc<Adt6>),
    C46(std::rc::Rc<Adt6>, std::rc::Rc<Adt6>, std::rc::Rc<Adt6>),
    C47(std::rc::Rc<Adt6>, std::rc::Rc<Adt6>, std::rc::Rc<Adt6>),
}

#[derive(Clone)]
pub enum Adt7 {
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
pub enum Adt8 {
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
    C40,
    C41,
    C42,
    C43,
    C44,
    C45,
    C46,
    C47,
    C48,
    C49,
    C50,
    C51,
    C52,
    C53,
    C54,
    C55,
}

#[derive(Clone)]
pub enum Adt9 {
    C0(Bytes, Adt2),
}

#[derive(Clone)]
pub enum Adt10 {
    C0(Option<Adt6>),
    C1(Adt11, Option<Adt6>, Option<Adt6>, bool),
    C2(Adt6, Option<Adt6>),
}

#[derive(Clone)]
pub enum Adt11 {
    C0,
    C1,
}

#[derive(Clone)]
pub enum Adt12 {
    C0(Bytes, Adt2, Adt6),
}

#[derive(Clone)]
pub enum Adt13 {
    C0((Option<Bytes>, Bytes)),
    C1((Option<Bytes>, Bytes)),
    C2((Option<Bytes>, Bytes), (Option<Bytes>, Bytes), (Option<Bytes>, Bytes)),
    C3((Option<Bytes>, Bytes)),
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
}

#[derive(Clone)]
pub enum Adt16 {
    C0,
    C1(Adt17, Adt18),
}

#[derive(Clone)]
pub enum Adt17 {
    C0,
}

#[derive(Clone)]
pub enum Adt18 {
    C0,
}

#[derive(Clone)]
pub enum Adt19 {
    C0(bool),
    C1(bool, u64),
    C2(Bytes),
    C3(std::rc::Rc<Adt19>),
    C4(std::rc::Rc<Adt19>),
    C5,
    C6(std::rc::Rc<Adt19>, std::rc::Rc<Adt19>),
    C7(Bytes, std::rc::Rc<Adt19>, std::rc::Rc<Adt19>),
}

#[derive(Clone)]
pub enum Adt20 {
    C0(Bytes, bool, bool, Option<u64>),
}

#[derive(Clone)]
pub enum Fn0 {
    F7,
    F12,
    F14,
    F18,
    F19,
    F21,
    F22,
    F31,
    F32,
    F34,
    F36,
    F38,
    F39,
    F96,
}

impl Fn0 {
    pub fn apply(&self, p0: List<Adt19>, p1: Bytes) -> List<Adt19> {
        match self {
            Fn0::F7 => f7(p0, p1),
            Fn0::F12 => f12(p0, p1),
            Fn0::F14 => f14(p0, p1),
            Fn0::F18 => f18(p0, p1),
            Fn0::F19 => f19(p0, p1),
            Fn0::F21 => f21(p0, p1),
            Fn0::F22 => f22(p0, p1),
            Fn0::F31 => f31(p0, p1),
            Fn0::F32 => f32(p0, p1),
            Fn0::F34 => f34(p0, p1),
            Fn0::F36 => f36(p0, p1),
            Fn0::F38 => f38(p0, p1),
            Fn0::F39 => f39(p0, p1),
            Fn0::F96 => f96(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F24,
}

impl Fn1 {
    pub fn apply(&self, p0: List<Adt19>, p1: Adt4) -> List<Adt19> {
        match self {
            Fn1::F24 => f24(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F29,
}

impl Fn2 {
    pub fn apply(&self, p0: List<Adt19>, p1: Adt13) -> List<Adt19> {
        match self {
            Fn2::F29 => f29(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F47,
}

impl Fn3 {
    pub fn apply(&self, p0: Adt19, p1: Adt19) -> Adt19 {
        match self {
            Fn3::F47 => f47(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F50,
}

impl Fn4 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn4::F50 => f50(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F56,
    F88,
}

impl Fn5 {
    pub fn apply(&self, p0: Adt19, p1: (Bytes, Adt19)) -> Adt19 {
        match self {
            Fn5::F56 => f56(p0, p1),
            Fn5::F88 => f88(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F58,
}

impl Fn6 {
    pub fn apply(&self, p0: List<Adt19>, p1: Adt2) -> List<Adt19> {
        match self {
            Fn6::F58 => f58(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F68,
}

impl Fn7 {
    pub fn apply(&self, p0: List<Adt4>, p1: Adt4) -> List<Adt4> {
        match self {
            Fn7::F68 => f68(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F72,
}

impl Fn8 {
    pub fn apply(&self, p0: List<Adt13>, p1: Adt13) -> List<Adt13> {
        match self {
            Fn8::F72 => f72(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F77(Bytes),
    F108(u64, Bytes),
}

impl Fn9 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn9::F77(k0) => f77(k0.clone(), p0),
            Fn9::F108(k0, k1) => Ok(f108(*k0, k1.clone(), p0)),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F80,
    F130,
}

impl Fn10 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn10::F80 => f80(p0, p1),
            Fn10::F130 => f130(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F83,
}

impl Fn11 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, Bytes)) -> R<List<Bytes>> {
        match self {
            Fn11::F83 => f83(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F85,
}

impl Fn12 {
    pub fn apply(&self, p0: List<Adt19>, p1: Adt19) -> List<Adt19> {
        match self {
            Fn12::F85 => f85(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F91,
}

impl Fn13 {
    pub fn apply(&self, p0: List<Adt2>, p1: Adt2) -> List<Adt2> {
        match self {
            Fn13::F91 => f91(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F94,
}

impl Fn14 {
    pub fn apply(&self, p0: List<Adt19>, p1: (Bytes, Adt2)) -> List<Adt19> {
        match self {
            Fn14::F94 => f94(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F101(Bytes),
}

impl Fn15 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Bytes) -> Option<Bytes> {
        match self {
            Fn15::F101(k0) => f101(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F106,
}

impl Fn16 {
    pub fn apply(&self, p0: List<(Bytes, Adt2)>, p1: (Bytes, Adt2)) -> List<(Bytes, Adt2)> {
        match self {
            Fn16::F106 => f106(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F112,
}

impl Fn17 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn17::F112 => f112(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F114,
}

impl Fn18 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
        match self {
            Fn18::F114 => f114(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F119,
}

impl Fn19 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn19::F119 => f119(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn20 {
    F124,
}

impl Fn20 {
    pub fn apply(&self, p0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
        match self {
            Fn20::F124 => f124(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn21 {
    F129,
}

impl Fn21 {
    pub fn apply(&self, p0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
        match self {
            Fn21::F129 => f129(p0),
        }
    }
}

pub fn f0(v0: Adt0) -> R<Bytes> {
    f1(f2(v0))
}

pub fn f1(v0: Adt19) -> R<Bytes> {
    f3(v0)
}

pub fn f2(v0: Adt0) -> Adt19 {
    let m1 = v0;
    match m1 {
        Adt0::C0(v1, v2, v3, v4, v5, v6, v7) => {
            let v8: List<(Bytes, Adt19)> = List::cons((f4(Str::lit("kind")), Adt19::C2(f4(Str::lit("logic")))), List::<(Bytes, Adt19)>::nil());
            let v9: List<(Bytes, Adt19)> = List::cons((f4(Str::lit("name")), Adt19::C2(v1)), v8);
            let v12: List<(Bytes, Adt19)> = {
                let m2 = v2.clone();
                match m2.uncons() {
                    None => {
                        v9
                    }
                    Some((_, _)) => {
                        List::cons((f4(Str::lit("type_parameters")), Adt19::C3(std::rc::Rc::new(f5(f6(Fn0::F7, List::<Adt19>::nil(), f8(v2)))))), v9)
                    }
                }
            };
            let v13: List<(Bytes, Adt19)> = List::cons((f4(Str::lit("state")), f9(v3)), v12);
            let v14: List<(Bytes, Adt19)> = List::cons((f4(Str::lit("relation")), f10({
                let (h3, _) = v4;
                h3
            }, {
                let (_, h4) = v4;
                h4
            })), v13);
            let v16: List<(Bytes, Adt19)> = {
                let m5 = v5;
                match m5 {
                    None => {
                        v14
                    }
                    Some(v15) => {
                        List::cons((f4(Str::lit("invariant")), f11(v15)), v14)
                    }
                }
            };
            let v18: List<(Bytes, Adt19)> = {
                let m6 = v6;
                match m6 {
                    None => {
                        v16
                    }
                    Some(v17) => {
                        List::cons((f4(Str::lit("ranking")), f10({
                            let (h7, _) = v17;
                            h7
                        }, {
                            let (_, h8) = v17;
                            h8
                        })), v16)
                    }
                }
            };
            let v21: List<(Bytes, Adt19)> = {
                let m9 = v7.clone();
                match m9.uncons() {
                    None => {
                        v18
                    }
                    Some((_, _)) => {
                        List::cons((f4(Str::lit("axioms")), Adt19::C3(std::rc::Rc::new(f5(f6(Fn0::F12, List::<Adt19>::nil(), f8(v7)))))), v18)
                    }
                }
            };
            f13(v21)
        }
        Adt0::C1(v22, v23, v24, v25, v26, v27, v28, v29, v30, v31) => {
            let v32: List<(Bytes, Adt19)> = List::cons((f4(Str::lit("kind")), Adt19::C2(f4(Str::lit("inference_rule")))), List::<(Bytes, Adt19)>::nil());
            let v33: List<(Bytes, Adt19)> = List::cons((f4(Str::lit("name")), Adt19::C2(v22)), v32);
            let v36: List<(Bytes, Adt19)> = {
                let m10 = v23.clone();
                match m10.uncons() {
                    None => {
                        v33
                    }
                    Some((_, _)) => {
                        List::cons((f4(Str::lit("type_parameters")), Adt19::C3(std::rc::Rc::new(f5(f6(Fn0::F14, List::<Adt19>::nil(), f8(v23)))))), v33)
                    }
                }
            };
            let v37: List<(Bytes, Adt19)> = List::cons((f4(Str::lit("logic")), f15(v24)), v36);
            let v39: List<(Bytes, Adt19)> = {
                let m11 = v25;
                match m11 {
                    None => {
                        v37
                    }
                    Some(v38) => {
                        List::cons((f4(Str::lit("binding")), f16(v38)), v37)
                    }
                }
            };
            let v40: List<(Bytes, Adt19)> = List::cons((f4(Str::lit("guard")), f17(v26)), v39);
            let v41: List<(Bytes, Adt19)> = List::cons((f4(Str::lit("conclusion")), f17(v27)), v40);
            let v42: List<(Bytes, Adt19)> = List::cons((f4(Str::lit("soundness")), f10({
                let (h12, _) = v28;
                h12
            }, {
                let (_, h13) = v28;
                h13
            })), v41);
            let v44: List<(Bytes, Adt19)> = {
                let m14 = v29;
                match m14 {
                    None => {
                        v42
                    }
                    Some(v43) => {
                        List::cons((f4(Str::lit("progress")), f10({
                            let (h15, _) = v43;
                            h15
                        }, {
                            let (_, h16) = v43;
                            h16
                        })), v42)
                    }
                }
            };
            let v45: List<(Bytes, Adt19)> = if v30 {
                List::cons((f4(Str::lit("executable")), Adt19::C0(true)), v44)
            } else {
                v44
            };
            let v48: List<(Bytes, Adt19)> = {
                let m17 = v31.clone();
                match m17.uncons() {
                    None => {
                        v45
                    }
                    Some((_, _)) => {
                        List::cons((f4(Str::lit("axioms")), Adt19::C3(std::rc::Rc::new(f5(f6(Fn0::F18, List::<Adt19>::nil(), f8(v31)))))), v45)
                    }
                }
            };
            f13(v48)
        }
        Adt0::C2(v49, v50, v51, v52, v53, v54, v55, v56, v57) => {
            let v58: List<(Bytes, Adt19)> = List::cons((f4(Str::lit("kind")), Adt19::C2(f4(Str::lit("verifier")))), List::<(Bytes, Adt19)>::nil());
            let v59: List<(Bytes, Adt19)> = List::cons((f4(Str::lit("name")), Adt19::C2(v49)), v58);
            let v62: List<(Bytes, Adt19)> = {
                let m18 = v50.clone();
                match m18.uncons() {
                    None => {
                        v59
                    }
                    Some((_, _)) => {
                        List::cons((f4(Str::lit("type_parameters")), Adt19::C3(std::rc::Rc::new(f5(f6(Fn0::F19, List::<Adt19>::nil(), f8(v50)))))), v59)
                    }
                }
            };
            let v63: List<(Bytes, Adt19)> = List::cons((f4(Str::lit("subject")), f20(v51)), v62);
            let v64: List<(Bytes, Adt19)> = List::cons((f4(Str::lit("candidate")), f20(v52)), v63);
            let v65: List<(Bytes, Adt19)> = List::cons((f4(Str::lit("specification")), f10({
                let (h19, _) = v53;
                h19
            }, {
                let (_, h20) = v53;
                h20
            })), v64);
            let v66: List<(Bytes, Adt19)> = List::cons((f4(Str::lit("check")), f10({
                let (h21, _) = v54;
                h21
            }, {
                let (_, h22) = v54;
                h22
            })), v65);
            let v67: List<(Bytes, Adt19)> = List::cons((f4(Str::lit("sound")), f10({
                let (h23, _) = v55;
                h23
            }, {
                let (_, h24) = v55;
                h24
            })), v66);
            let v69: List<(Bytes, Adt19)> = {
                let m25 = v56;
                match m25 {
                    None => {
                        v67
                    }
                    Some(v68) => {
                        List::cons((f4(Str::lit("complete")), f10({
                            let (h26, _) = v68;
                            h26
                        }, {
                            let (_, h27) = v68;
                            h27
                        })), v67)
                    }
                }
            };
            let v72: List<(Bytes, Adt19)> = {
                let m28 = v57.clone();
                match m28.uncons() {
                    None => {
                        v69
                    }
                    Some((_, _)) => {
                        List::cons((f4(Str::lit("axioms")), Adt19::C3(std::rc::Rc::new(f5(f6(Fn0::F21, List::<Adt19>::nil(), f8(v57)))))), v69)
                    }
                }
            };
            f13(v72)
        }
        Adt0::C3(v73, v74, v75, v76, v77, v78, v79, v80, v81, v82, v83, v84) => {
            let v85: List<(Bytes, Adt19)> = List::cons((f4(Str::lit("kind")), Adt19::C2(f4(Str::lit("reasoner")))), List::<(Bytes, Adt19)>::nil());
            let v86: List<(Bytes, Adt19)> = List::cons((f4(Str::lit("name")), Adt19::C2(v73)), v85);
            let v89: List<(Bytes, Adt19)> = {
                let m29 = v74.clone();
                match m29.uncons() {
                    None => {
                        v86
                    }
                    Some((_, _)) => {
                        List::cons((f4(Str::lit("type_parameters")), Adt19::C3(std::rc::Rc::new(f5(f6(Fn0::F22, List::<Adt19>::nil(), f8(v74)))))), v86)
                    }
                }
            };
            let v91: List<(Bytes, Adt19)> = {
                let m30 = v75;
                match m30 {
                    None => {
                        v89
                    }
                    Some(v90) => {
                        List::cons((f4(Str::lit("logic")), f15(v90)), v89)
                    }
                }
            };
            let v92: List<(Bytes, Adt19)> = List::cons((f4(Str::lit("observation")), f20(v76)), v91);
            let v94: List<(Bytes, Adt19)> = {
                let m31 = v77;
                match m31 {
                    None => {
                        v92
                    }
                    Some(v93) => {
                        List::cons((f4(Str::lit("observe")), f17(v93)), v92)
                    }
                }
            };
            let v95: List<(Bytes, Adt19)> = List::cons((f4(Str::lit("rules")), Adt19::C3(std::rc::Rc::new(f5(f23(Fn1::F24, List::<Adt19>::nil(), f25(v78)))))), v94);
            let v96: List<(Bytes, Adt19)> = List::cons((f4(Str::lit("strategy")), f26(v79)), v95);
            let v98: List<(Bytes, Adt19)> = {
                let m32 = v80;
                match m32 {
                    None => {
                        v96
                    }
                    Some(v97) => {
                        List::cons((f4(Str::lit("answer")), f27(v97)), v96)
                    }
                }
            };
            let v99: List<(Bytes, Adt19)> = List::cons((f4(Str::lit("verifier")), f15(v81)), v98);
            let v102: List<(Bytes, Adt19)> = {
                let m33 = v82.clone();
                match m33.uncons() {
                    None => {
                        v99
                    }
                    Some((_, _)) => {
                        List::cons((f4(Str::lit("claims")), Adt19::C3(std::rc::Rc::new(f5(f28(Fn2::F29, List::<Adt19>::nil(), f30(v82)))))), v99)
                    }
                }
            };
            let v103: List<(Bytes, Adt19)> = if v83 {
                List::cons((f4(Str::lit("executable")), Adt19::C0(true)), v102)
            } else {
                v102
            };
            let v106: List<(Bytes, Adt19)> = {
                let m34 = v84.clone();
                match m34.uncons() {
                    None => {
                        v103
                    }
                    Some((_, _)) => {
                        List::cons((f4(Str::lit("axioms")), Adt19::C3(std::rc::Rc::new(f5(f6(Fn0::F31, List::<Adt19>::nil(), f8(v84)))))), v103)
                    }
                }
            };
            f13(v106)
        }
        Adt0::C4(v107, v108, v109, v110, v111) => {
            let v112: List<(Bytes, Adt19)> = List::cons((f4(Str::lit("kind")), Adt19::C2(f4(Str::lit("tokenizer")))), List::<(Bytes, Adt19)>::nil());
            let v113: List<(Bytes, Adt19)> = List::cons((f4(Str::lit("name")), Adt19::C2(v107)), v112);
            let v116: List<(Bytes, Adt19)> = {
                let m35 = v108.clone();
                match m35.uncons() {
                    None => {
                        v113
                    }
                    Some((_, _)) => {
                        List::cons((f4(Str::lit("type_parameters")), Adt19::C3(std::rc::Rc::new(f5(f6(Fn0::F32, List::<Adt19>::nil(), f8(v108)))))), v113)
                    }
                }
            };
            let v117: List<(Bytes, Adt19)> = List::cons((f4(Str::lit("scheme")), f33(v109)), v116);
            let v118: List<(Bytes, Adt19)> = List::cons((f4(Str::lit("vocabulary")), f15(v110)), v117);
            let v121: List<(Bytes, Adt19)> = {
                let m36 = v111.clone();
                match m36.uncons() {
                    None => {
                        v118
                    }
                    Some((_, _)) => {
                        List::cons((f4(Str::lit("axioms")), Adt19::C3(std::rc::Rc::new(f5(f6(Fn0::F34, List::<Adt19>::nil(), f8(v111)))))), v118)
                    }
                }
            };
            f13(v121)
        }
        Adt0::C5(v122, v123, v124, v125) => {
            let v126: List<(Bytes, Adt19)> = List::cons((f4(Str::lit("kind")), Adt19::C2(f4(Str::lit("context_window")))), List::<(Bytes, Adt19)>::nil());
            let v127: List<(Bytes, Adt19)> = List::cons((f4(Str::lit("name")), Adt19::C2(v122)), v126);
            let v128: List<(Bytes, Adt19)> = List::cons((f4(Str::lit("capacity")), Adt19::C1(false, v123)), v127);
            let v129: List<(Bytes, Adt19)> = List::cons((f4(Str::lit("overflow")), f35(v124)), v128);
            let v132: List<(Bytes, Adt19)> = {
                let m37 = v125.clone();
                match m37.uncons() {
                    None => {
                        v129
                    }
                    Some((_, _)) => {
                        List::cons((f4(Str::lit("axioms")), Adt19::C3(std::rc::Rc::new(f5(f6(Fn0::F36, List::<Adt19>::nil(), f8(v125)))))), v129)
                    }
                }
            };
            f13(v132)
        }
        Adt0::C6(v133, v134, v135, v136, v137, v138, v139, v140) => {
            let v141: List<(Bytes, Adt19)> = List::cons((f4(Str::lit("kind")), Adt19::C2(f4(Str::lit("text_generator")))), List::<(Bytes, Adt19)>::nil());
            let v142: List<(Bytes, Adt19)> = List::cons((f4(Str::lit("name")), Adt19::C2(v133)), v141);
            let v143: List<(Bytes, Adt19)> = List::cons((f4(Str::lit("tokenizer")), f10({
                let (h38, _) = v134;
                h38
            }, {
                let (_, h39) = v134;
                h39
            })), v142);
            let v144: List<(Bytes, Adt19)> = List::cons((f4(Str::lit("window")), f10({
                let (h40, _) = v135;
                h40
            }, {
                let (_, h41) = v135;
                h41
            })), v143);
            let v145: List<(Bytes, Adt19)> = List::cons((f4(Str::lit("scorer")), f15(v136)), v144);
            let v146: List<(Bytes, Adt19)> = List::cons((f4(Str::lit("max_tokens")), Adt19::C1(false, v137)), v145);
            let v147: List<(Bytes, Adt19)> = List::cons((f4(Str::lit("stop")), f10({
                let (h42, _) = v138;
                h42
            }, {
                let (_, h43) = v138;
                h43
            })), v146);
            let v148: List<(Bytes, Adt19)> = List::cons((f4(Str::lit("policy")), f37(v139)), v147);
            let v151: List<(Bytes, Adt19)> = {
                let m44 = v140.clone();
                match m44.uncons() {
                    None => {
                        v148
                    }
                    Some((_, _)) => {
                        List::cons((f4(Str::lit("axioms")), Adt19::C3(std::rc::Rc::new(f5(f6(Fn0::F38, List::<Adt19>::nil(), f8(v140)))))), v148)
                    }
                }
            };
            f13(v151)
        }
        Adt0::C7(v152, v153, v154, v155) => {
            let v156: List<(Bytes, Adt19)> = List::cons((f4(Str::lit("kind")), Adt19::C2(f4(Str::lit("text_classifier")))), List::<(Bytes, Adt19)>::nil());
            let v157: List<(Bytes, Adt19)> = List::cons((f4(Str::lit("name")), Adt19::C2(v152)), v156);
            let v158: List<(Bytes, Adt19)> = List::cons((f4(Str::lit("tokenizer")), f10({
                let (h45, _) = v153;
                h45
            }, {
                let (_, h46) = v153;
                h46
            })), v157);
            let v159: List<(Bytes, Adt19)> = List::cons((f4(Str::lit("model")), f15(v154)), v158);
            let v162: List<(Bytes, Adt19)> = {
                let m47 = v155.clone();
                match m47.uncons() {
                    None => {
                        v159
                    }
                    Some((_, _)) => {
                        List::cons((f4(Str::lit("axioms")), Adt19::C3(std::rc::Rc::new(f5(f6(Fn0::F39, List::<Adt19>::nil(), f8(v155)))))), v159)
                    }
                }
            };
            f13(v162)
        }
    }
}

pub fn f3(v0: Adt19) -> R<Bytes> {
    let m48 = v0;
    match m48 {
        Adt19::C0(v1) => {
            if v1 {
                Ok(Bytes::lit(&[0x74, 0x72, 0x75, 0x65]))
            } else {
                Ok(Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]))
            }
        }
        Adt19::C1(v2, v3) => {
            f40(v2, v3)
        }
        Adt19::C2(v4) => {
            f41(v4)
        }
        Adt19::C3(r49) => {
            let v5 = (*r49).clone();
            Ok(f42(f43(v5)?))
        }
        Adt19::C4(r50) => {
            let v6 = (*r50).clone();
            f44(f45(v6)?)
        }
        Adt19::C5 => {
            Ok(Bytes::lit(&[]))
        }
        Adt19::C6(_, _) => {
            Ok(Bytes::lit(&[]))
        }
        Adt19::C7(_, _, _) => {
            Ok(Bytes::lit(&[]))
        }
    }
}

pub fn f4(v0: Str) -> Bytes {
    let a51 = v0;
    utf8_encode(a51)
}

pub fn f5(v0: List<Adt19>) -> Adt19 {
    f46(Fn3::F47, Adt19::C5, f48(v0))
}

pub fn f6(v0: Fn0, p55: List<Adt19>, p56: List<Bytes>) -> List<Adt19> {
    let m54 = (p55, p56);
    {
        let mut m54 = m54;
        loop {
            let step = {
                let (v1, v2) = m54;
                let m52 = v2;
                match m52.uncons() {
                    None => {
                        Err::<(List<Adt19>, List<Bytes>), List<Adt19>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt19>, List<Bytes>), List<Adt19>>(({
                            let c53: Fn0 = v0.clone();
                            c53.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m54 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f7(v0: List<Adt19>, v1: Bytes) -> List<Adt19> {
    List::cons(Adt19::C2(v1), v0)
}

pub fn f8(v0: List<Bytes>) -> List<Bytes> {
    f49(Fn4::F50, List::<Bytes>::nil(), v0)
}

pub fn f9(v0: Adt1) -> Adt19 {
    let m57 = v0;
    match m57 {
        Adt1::C0(v1, v2, v3) => {
            let v4: List<(Bytes, Adt19)> = List::cons((f51(Str::lit("name")), Adt19::C2(v1)), List::<(Bytes, Adt19)>::nil());
            let v5: List<(Bytes, Adt19)> = List::cons((f51(Str::lit("next")), Adt19::C2(v2)), v4);
            let v6: List<(Bytes, Adt19)> = List::cons((f51(Str::lit("type")), f52(v3)), v5);
            f53(v6)
        }
    }
}

pub fn f10(v0: Option<Bytes>, v1: Bytes) -> Adt19 {
    Adt19::C4(std::rc::Rc::new({
        let m58 = v0;
        match m58 {
            None => {
                f54(Str::lit("name"), Adt19::C2(v1), Adt19::C5)
            }
            Some(v2) => {
                f54(Str::lit("module"), Adt19::C2(v2), f54(Str::lit("name"), Adt19::C2(v1), Adt19::C5))
            }
        }
    }))
}

pub fn f11(v0: Adt3) -> Adt19 {
    let m59 = v0;
    match m59 {
        Adt3::C0(v1, v2) => {
            let v3: List<(Bytes, Adt19)> = List::cons((f4(Str::lit("predicate")), f10({
                let (h60, _) = v1;
                h60
            }, {
                let (_, h61) = v1;
                h61
            })), List::<(Bytes, Adt19)>::nil());
            let v4: List<(Bytes, Adt19)> = List::cons((f4(Str::lit("preserves")), f10({
                let (h62, _) = v2;
                h62
            }, {
                let (_, h63) = v2;
                h63
            })), v3);
            f13(v4)
        }
    }
}

pub fn f12(v0: List<Adt19>, v1: Bytes) -> List<Adt19> {
    List::cons(Adt19::C2(v1), v0)
}

pub fn f13(v0: List<(Bytes, Adt19)>) -> Adt19 {
    Adt19::C4(std::rc::Rc::new(f55(Fn5::F56, Adt19::C5, v0)))
}

pub fn f14(v0: List<Adt19>, v1: Bytes) -> List<Adt19> {
    List::cons(Adt19::C2(v1), v0)
}

pub fn f15(v0: Adt4) -> Adt19 {
    let m64 = v0;
    match m64 {
        Adt4::C0(v1, v2) => {
            let v3: List<(Bytes, Adt19)> = List::cons((f51(Str::lit("member")), f10({
                let (h65, _) = v1;
                h65
            }, {
                let (_, h66) = v1;
                h66
            })), List::<(Bytes, Adt19)>::nil());
            let v6: List<(Bytes, Adt19)> = {
                let m67 = v2.clone();
                match m67.uncons() {
                    None => {
                        v3
                    }
                    Some((_, _)) => {
                        List::cons((f51(Str::lit("type_arguments")), Adt19::C3(std::rc::Rc::new(f5(f57(Fn6::F58, List::<Adt19>::nil(), f59(v2)))))), v3)
                    }
                }
            };
            f53(v6)
        }
    }
}

pub fn f16(v0: Adt5) -> Adt19 {
    let m68 = v0;
    match m68 {
        Adt5::C0(v1, v2, v3) => {
            let v4: List<(Bytes, Adt19)> = List::cons((f4(Str::lit("name")), Adt19::C2(v1)), List::<(Bytes, Adt19)>::nil());
            let v5: List<(Bytes, Adt19)> = List::cons((f4(Str::lit("type")), f52(v2)), v4);
            let v6: List<(Bytes, Adt19)> = List::cons((f4(Str::lit("candidates")), f17(v3)), v5);
            f13(v6)
        }
    }
}

pub fn f17(v0: Adt6) -> Adt19 {
    let m69 = v0;
    match m69 {
        Adt6::C0(v1) => {
            f60(Str::lit("var"), f54(Str::lit("name"), Adt19::C2(v1), Adt19::C5))
        }
        Adt6::C1(v2) => {
            f60(Str::lit("nat"), f54(Str::lit("value"), Adt19::C2(v2), Adt19::C5))
        }
        Adt6::C2(v3, v4) => {
            f60(Str::lit("integer"), f54(Str::lit("representation"), Adt19::C2(f61(v3)), f54(Str::lit("value"), Adt19::C2(v4), Adt19::C5)))
        }
        Adt6::C3(v5) => {
            f60(Str::lit("string"), f54(Str::lit("value"), Adt19::C2(v5), Adt19::C5))
        }
        Adt6::C4(v6) => {
            f60(Str::lit("bytes"), f54(Str::lit("hex"), Adt19::C2(v6), Adt19::C5))
        }
        Adt6::C5(v7, r72, v9) => {
            let v8 = (*r72).clone();
            f60(Str::lit("primitive"), f54(Str::lit("operation"), Adt19::C2({
                let m71 = f62(v7);
                match m71 {
                    Adt20::C0(h70, _, _, _) => {
                        h70
                    }
                }
            }), f54(Str::lit("arguments"), Adt19::C3(std::rc::Rc::new(f17(v8))), f54(Str::lit("result"), f52(v9), Adt19::C5))))
        }
        Adt6::C6(v10) => {
            f60(Str::lit("bool"), f54(Str::lit("value"), Adt19::C0(v10), Adt19::C5))
        }
        Adt6::C7 => {
            f60(Str::lit("unit"), Adt19::C5)
        }
        Adt6::C8(v11) => {
            f60(Str::lit("nil"), f54(Str::lit("element"), f52(v11), Adt19::C5))
        }
        Adt6::C9(r73, r74) => {
            let v12 = (*r73).clone();
            let v13 = (*r74).clone();
            f60(Str::lit("cons"), f54(Str::lit("head"), f17(v12), f54(Str::lit("tail"), f17(v13), Adt19::C5)))
        }
        Adt6::C10(v14, v15, v16, r75) => {
            let v17 = (*r75).clone();
            f60(Str::lit("record"), f54(Str::lit("type"), f10(v14, v15), f63(Str::lit("type_arguments"), v16, f54(Str::lit("fields"), Adt19::C3(std::rc::Rc::new(f17(v17))), Adt19::C5))))
        }
        Adt6::C11(v18, v19, v20, r76) => {
            let v21 = (*r76).clone();
            f60(Str::lit("constructor"), f54(Str::lit("constructor"), f10(v18, v19), f63(Str::lit("type_arguments"), v20, f54(Str::lit("arguments"), Adt19::C3(std::rc::Rc::new(f17(v21))), Adt19::C5))))
        }
        Adt6::C12(v22, v23, v24, v25, v26) => {
            f60(Str::lit("instance_value"), f54(Str::lit("class"), f10(v22, v23), f54(Str::lit("arguments"), Adt19::C3(std::rc::Rc::new(f52(v24))), f54(Str::lit("resolved"), f10(v25, v26), Adt19::C5))))
        }
        Adt6::C13(r77, v28) => {
            let v27 = (*r77).clone();
            f60(Str::lit("project"), f54(Str::lit("value"), f17(v27), f54(Str::lit("field"), Adt19::C2(v28), Adt19::C5)))
        }
        Adt6::C14(v29, v30, v31, r78) => {
            let v32 = (*r78).clone();
            f60(Str::lit("call"), f54(Str::lit("function"), f10(v29, v30), f63(Str::lit("type_arguments"), v31, f54(Str::lit("arguments"), Adt19::C3(std::rc::Rc::new(f17(v32))), Adt19::C5))))
        }
        Adt6::C15(r79, r80, r81) => {
            let v33 = (*r79).clone();
            let v34 = (*r80).clone();
            let v35 = (*r81).clone();
            f60(Str::lit("if"), f54(Str::lit("condition"), f17(v33), f54(Str::lit("then_value"), f17(v34), f54(Str::lit("else_value"), f17(v35), Adt19::C5))))
        }
        Adt6::C16(r82, r83) => {
            let v36 = (*r82).clone();
            let v37 = (*r83).clone();
            f60(Str::lit("match"), f54(Str::lit("scrutinee"), f17(v36), f54(Str::lit("branches"), Adt19::C3(std::rc::Rc::new(f17(v37))), Adt19::C5)))
        }
        Adt6::C17(r84, r85) => {
            let v38 = (*r84).clone();
            let v39 = (*r85).clone();
            f60(Str::lit("eq"), f54(Str::lit("left"), f17(v38), f54(Str::lit("right"), f17(v39), Adt19::C5)))
        }
        Adt6::C18(r86, r87) => {
            let v40 = (*r86).clone();
            let v41 = (*r87).clone();
            f60(Str::lit("le"), f54(Str::lit("left"), f17(v40), f54(Str::lit("right"), f17(v41), Adt19::C5)))
        }
        Adt6::C19(r88, r89) => {
            let v42 = (*r88).clone();
            let v43 = (*r89).clone();
            f60(Str::lit("lt"), f54(Str::lit("left"), f17(v42), f54(Str::lit("right"), f17(v43), Adt19::C5)))
        }
        Adt6::C20(r90, r91) => {
            let v44 = (*r90).clone();
            let v45 = (*r91).clone();
            f60(Str::lit("add"), f54(Str::lit("left"), f17(v44), f54(Str::lit("right"), f17(v45), Adt19::C5)))
        }
        Adt6::C21(r92, r93) => {
            let v46 = (*r92).clone();
            let v47 = (*r93).clone();
            f60(Str::lit("beq"), f54(Str::lit("left"), f17(v46), f54(Str::lit("right"), f17(v47), Adt19::C5)))
        }
        Adt6::C22(r94, r95) => {
            let v48 = (*r94).clone();
            let v49 = (*r95).clone();
            f60(Str::lit("ble"), f54(Str::lit("left"), f17(v48), f54(Str::lit("right"), f17(v49), Adt19::C5)))
        }
        Adt6::C23(r96, r97) => {
            let v50 = (*r96).clone();
            let v51 = (*r97).clone();
            f60(Str::lit("blt"), f54(Str::lit("left"), f17(v50), f54(Str::lit("right"), f17(v51), Adt19::C5)))
        }
        Adt6::C24(r98, r99) => {
            let v52 = (*r98).clone();
            let v53 = (*r99).clone();
            f60(Str::lit("and"), f54(Str::lit("left"), f17(v52), f54(Str::lit("right"), f17(v53), Adt19::C5)))
        }
        Adt6::C25(r100, r101) => {
            let v54 = (*r100).clone();
            let v55 = (*r101).clone();
            f60(Str::lit("prop_and"), f54(Str::lit("left"), f17(v54), f54(Str::lit("right"), f17(v55), Adt19::C5)))
        }
        Adt6::C26(r102, r103) => {
            let v56 = (*r102).clone();
            let v57 = (*r103).clone();
            f60(Str::lit("or"), f54(Str::lit("left"), f17(v56), f54(Str::lit("right"), f17(v57), Adt19::C5)))
        }
        Adt6::C27(r104) => {
            let v58 = (*r104).clone();
            f60(Str::lit("not"), f54(Str::lit("value"), f17(v58), Adt19::C5))
        }
        Adt6::C28(r105, r106) => {
            let v59 = (*r105).clone();
            let v60 = (*r106).clone();
            f60(Str::lit("implies"), f54(Str::lit("premise"), f17(v59), f54(Str::lit("conclusion"), f17(v60), Adt19::C5)))
        }
        Adt6::C29(r107, r108) => {
            let v61 = (*r107).clone();
            let v62 = (*r108).clone();
            f60(Str::lit("iff"), f54(Str::lit("left"), f17(v61), f54(Str::lit("right"), f17(v62), Adt19::C5)))
        }
        Adt6::C30(v63, v64, r109) => {
            let v65 = (*r109).clone();
            f60(Str::lit("forall"), f54(Str::lit("binder"), f64(v63, v64), f54(Str::lit("body"), f17(v65), Adt19::C5)))
        }
        Adt6::C31(v66, v67, r110, r111) => {
            let v68 = (*r110).clone();
            let v69 = (*r111).clone();
            f60(Str::lit("let"), f54(Str::lit("binder"), f64(v66, v67), f54(Str::lit("value"), f17(v68), f54(Str::lit("body"), f17(v69), Adt19::C5))))
        }
        Adt6::C32(r112, r113) => {
            let v70 = (*r112).clone();
            let v71 = (*r113).clone();
            f60(Str::lit("pair"), f54(Str::lit("left"), f17(v70), f54(Str::lit("right"), f17(v71), Adt19::C5)))
        }
        Adt6::C33(r114) => {
            let v72 = (*r114).clone();
            f60(Str::lit("first"), f54(Str::lit("value"), f17(v72), Adt19::C5))
        }
        Adt6::C34(r115) => {
            let v73 = (*r115).clone();
            f60(Str::lit("second"), f54(Str::lit("value"), f17(v73), Adt19::C5))
        }
        Adt6::C35(v74, v75, r116) => {
            let v76 = (*r116).clone();
            f60(Str::lit("lambda"), f54(Str::lit("parameters"), f65(v74), f54(Str::lit("captures"), f66(v75), f54(Str::lit("body"), f17(v76), Adt19::C5))))
        }
        Adt6::C36(r117, r118) => {
            let v77 = (*r117).clone();
            let v78 = (*r118).clone();
            f60(Str::lit("apply"), f54(Str::lit("function"), f17(v77), f54(Str::lit("arguments"), Adt19::C3(std::rc::Rc::new(f17(v78))), Adt19::C5)))
        }
        Adt6::C37(v79, v80, v81) => {
            f60(Str::lit("function_ref"), f54(Str::lit("function"), f10(v79, v80), f63(Str::lit("type_arguments"), v81, Adt19::C5)))
        }
        Adt6::C38(v82, v83, r119) => {
            let v84 = (*r119).clone();
            f60(Str::lit("map_literal"), f54(Str::lit("key"), f52(v82), f54(Str::lit("value"), f52(v83), f54(Str::lit("entries"), Adt19::C3(std::rc::Rc::new(f17(v84))), Adt19::C5))))
        }
        Adt6::C39(v85, r120) => {
            let v86 = (*r120).clone();
            f60(Str::lit("set_literal"), f54(Str::lit("element"), f52(v85), f54(Str::lit("elements"), Adt19::C3(std::rc::Rc::new(f17(v86))), Adt19::C5)))
        }
        Adt6::C40(v87, r121, r122) => {
            let v88 = (*r121).clone();
            let v89 = (*r122).clone();
            f60(Str::lit("graph_literal"), f54(Str::lit("node"), f52(v87), f54(Str::lit("nodes"), Adt19::C3(std::rc::Rc::new(f17(v88))), f54(Str::lit("edges"), Adt19::C3(std::rc::Rc::new(f17(v89))), Adt19::C5))))
        }
        Adt6::C41(v90, v91, v92, r123, v94) => {
            let v93 = (*r123).clone();
            f60(Str::lit("checked_apply"), f54(Str::lit("model"), f10(v90, v91), f63(Str::lit("type_arguments"), v92, f54(Str::lit("arguments"), Adt19::C3(std::rc::Rc::new(f17(v93))), f54(Str::lit("checks"), f66(v94), Adt19::C5)))))
        }
        Adt6::C42 => {
            Adt19::C5
        }
        Adt6::C43(r124, r125) => {
            let v95 = (*r124).clone();
            let v96 = (*r125).clone();
            Adt19::C6(std::rc::Rc::new(f17(v95)), std::rc::Rc::new(f17(v96)))
        }
        Adt6::C44(v97, r126, r127) => {
            let v98 = (*r126).clone();
            let v99 = (*r127).clone();
            Adt19::C6(std::rc::Rc::new(Adt19::C4(std::rc::Rc::new(f54(Str::lit("field"), Adt19::C2(v97), f54(Str::lit("value"), f17(v98), Adt19::C5))))), std::rc::Rc::new(f17(v99)))
        }
        Adt6::C45(v100, v101, v102, r128, r129) => {
            let v103 = (*r128).clone();
            let v104 = (*r129).clone();
            Adt19::C6(std::rc::Rc::new(Adt19::C4(std::rc::Rc::new(f54(Str::lit("constructor"), f10(v100, v101), f54(Str::lit("binders"), f66(v102), f54(Str::lit("body"), f17(v103), Adt19::C5)))))), std::rc::Rc::new(f17(v104)))
        }
        Adt6::C46(r130, r131, r132) => {
            let v105 = (*r130).clone();
            let v106 = (*r131).clone();
            let v107 = (*r132).clone();
            Adt19::C6(std::rc::Rc::new(Adt19::C4(std::rc::Rc::new(f54(Str::lit("key"), f17(v105), f54(Str::lit("value"), f17(v106), Adt19::C5))))), std::rc::Rc::new(f17(v107)))
        }
        Adt6::C47(r133, r134, r135) => {
            let v108 = (*r133).clone();
            let v109 = (*r134).clone();
            let v110 = (*r135).clone();
            Adt19::C6(std::rc::Rc::new(Adt19::C4(std::rc::Rc::new(f54(Str::lit("source"), f17(v108), f54(Str::lit("target"), f17(v109), Adt19::C5))))), std::rc::Rc::new(f17(v110)))
        }
    }
}

pub fn f18(v0: List<Adt19>, v1: Bytes) -> List<Adt19> {
    List::cons(Adt19::C2(v1), v0)
}

pub fn f19(v0: List<Adt19>, v1: Bytes) -> List<Adt19> {
    List::cons(Adt19::C2(v1), v0)
}

pub fn f20(v0: Adt9) -> Adt19 {
    let m136 = v0;
    match m136 {
        Adt9::C0(v1, v2) => {
            let v3: List<(Bytes, Adt19)> = List::cons((f51(Str::lit("name")), Adt19::C2(v1)), List::<(Bytes, Adt19)>::nil());
            let v4: List<(Bytes, Adt19)> = List::cons((f51(Str::lit("type")), f52(v2)), v3);
            f53(v4)
        }
    }
}

pub fn f21(v0: List<Adt19>, v1: Bytes) -> List<Adt19> {
    List::cons(Adt19::C2(v1), v0)
}

pub fn f22(v0: List<Adt19>, v1: Bytes) -> List<Adt19> {
    List::cons(Adt19::C2(v1), v0)
}

pub fn f23(v0: Fn1, p140: List<Adt19>, p141: List<Adt4>) -> List<Adt19> {
    let m139 = (p140, p141);
    {
        let mut m139 = m139;
        loop {
            let step = {
                let (v1, v2) = m139;
                let m137 = v2;
                match m137.uncons() {
                    None => {
                        Err::<(List<Adt19>, List<Adt4>), List<Adt19>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt19>, List<Adt4>), List<Adt19>>(({
                            let c138: Fn1 = v0.clone();
                            c138.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m139 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f24(v0: List<Adt19>, v1: Adt4) -> List<Adt19> {
    List::cons(f15(v1), v0)
}

pub fn f25(v0: List<Adt4>) -> List<Adt4> {
    f67(Fn7::F68, List::<Adt4>::nil(), v0)
}

pub fn f26(v0: Adt10) -> Adt19 {
    let m142 = v0;
    match m142 {
        Adt10::C0(v1) => {
            let v2: List<(Bytes, Adt19)> = List::cons((f4(Str::lit("kind")), Adt19::C2(f4(Str::lit("forward")))), List::<(Bytes, Adt19)>::nil());
            let v4: List<(Bytes, Adt19)> = {
                let m143 = v1;
                match m143 {
                    None => {
                        v2
                    }
                    Some(v3) => {
                        List::cons((f4(Str::lit("fuel")), f17(v3)), v2)
                    }
                }
            };
            f13(v4)
        }
        Adt10::C1(v5, v6, v7, v8) => {
            let v9: List<(Bytes, Adt19)> = List::cons((f4(Str::lit("kind")), Adt19::C2(f4(Str::lit("search")))), List::<(Bytes, Adt19)>::nil());
            let v10: List<(Bytes, Adt19)> = List::cons((f4(Str::lit("order")), f69(v5)), v9);
            let v12: List<(Bytes, Adt19)> = {
                let m144 = v6;
                match m144 {
                    None => {
                        v10
                    }
                    Some(v11) => {
                        List::cons((f4(Str::lit("fuel")), f17(v11)), v10)
                    }
                }
            };
            let v14: List<(Bytes, Adt19)> = {
                let m145 = v7;
                match m145 {
                    None => {
                        v12
                    }
                    Some(v13) => {
                        List::cons((f4(Str::lit("frontier")), f17(v13)), v12)
                    }
                }
            };
            let v15: List<(Bytes, Adt19)> = if v8 {
                List::cons((f4(Str::lit("deduplicate")), Adt19::C0(true)), v14)
            } else {
                v14
            };
            f13(v15)
        }
        Adt10::C2(v16, v17) => {
            let v18: List<(Bytes, Adt19)> = List::cons((f4(Str::lit("kind")), Adt19::C2(f4(Str::lit("generate_and_verify")))), List::<(Bytes, Adt19)>::nil());
            let v19: List<(Bytes, Adt19)> = List::cons((f4(Str::lit("generator")), f17(v16)), v18);
            let v21: List<(Bytes, Adt19)> = {
                let m146 = v17;
                match m146 {
                    None => {
                        v19
                    }
                    Some(v20) => {
                        List::cons((f4(Str::lit("budget")), f17(v20)), v19)
                    }
                }
            };
            f13(v21)
        }
    }
}

pub fn f27(v0: Adt12) -> Adt19 {
    let m147 = v0;
    match m147 {
        Adt12::C0(v1, v2, v3) => {
            let v4: List<(Bytes, Adt19)> = List::cons((f4(Str::lit("name")), Adt19::C2(v1)), List::<(Bytes, Adt19)>::nil());
            let v5: List<(Bytes, Adt19)> = List::cons((f4(Str::lit("type")), f52(v2)), v4);
            let v6: List<(Bytes, Adt19)> = List::cons((f4(Str::lit("value")), f17(v3)), v5);
            f13(v6)
        }
    }
}

pub fn f28(v0: Fn2, p151: List<Adt19>, p152: List<Adt13>) -> List<Adt19> {
    let m150 = (p151, p152);
    {
        let mut m150 = m150;
        loop {
            let step = {
                let (v1, v2) = m150;
                let m148 = v2;
                match m148.uncons() {
                    None => {
                        Err::<(List<Adt19>, List<Adt13>), List<Adt19>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt19>, List<Adt13>), List<Adt19>>(({
                            let c149: Fn2 = v0.clone();
                            c149.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m150 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f29(v0: List<Adt19>, v1: Adt13) -> List<Adt19> {
    List::cons(f70(v1), v0)
}

pub fn f30(v0: List<Adt13>) -> List<Adt13> {
    f71(Fn8::F72, List::<Adt13>::nil(), v0)
}

pub fn f31(v0: List<Adt19>, v1: Bytes) -> List<Adt19> {
    List::cons(Adt19::C2(v1), v0)
}

pub fn f32(v0: List<Adt19>, v1: Bytes) -> List<Adt19> {
    List::cons(Adt19::C2(v1), v0)
}

pub fn f33(v0: Adt14) -> Adt19 {
    let m153 = v0;
    match m153 {
        Adt14::C0 => {
            Adt19::C2(f4(Str::lit("byte_level")))
        }
        Adt14::C1 => {
            Adt19::C2(f4(Str::lit("longest_match")))
        }
    }
}

pub fn f34(v0: List<Adt19>, v1: Bytes) -> List<Adt19> {
    List::cons(Adt19::C2(v1), v0)
}

pub fn f35(v0: Adt15) -> Adt19 {
    let m154 = v0;
    match m154 {
        Adt15::C0 => {
            Adt19::C2(f4(Str::lit("refuse")))
        }
        Adt15::C1 => {
            Adt19::C2(f4(Str::lit("keep_last")))
        }
    }
}

pub fn f36(v0: List<Adt19>, v1: Bytes) -> List<Adt19> {
    List::cons(Adt19::C2(v1), v0)
}

pub fn f37(v0: Adt16) -> Adt19 {
    let m155 = v0;
    match m155 {
        Adt16::C0 => {
            let v1: List<(Bytes, Adt19)> = List::cons((f4(Str::lit("kind")), Adt19::C2(f4(Str::lit("greedy")))), List::<(Bytes, Adt19)>::nil());
            f13(v1)
        }
        Adt16::C1(v2, v3) => {
            let v4: List<(Bytes, Adt19)> = List::cons((f4(Str::lit("kind")), Adt19::C2(f4(Str::lit("sample")))), List::<(Bytes, Adt19)>::nil());
            let v5: List<(Bytes, Adt19)> = List::cons((f4(Str::lit("generator")), f73(v2)), v4);
            let v6: List<(Bytes, Adt19)> = List::cons((f4(Str::lit("distribution")), f74(v3)), v5);
            f13(v6)
        }
    }
}

pub fn f38(v0: List<Adt19>, v1: Bytes) -> List<Adt19> {
    List::cons(Adt19::C2(v1), v0)
}

pub fn f39(v0: List<Adt19>, v1: Bytes) -> List<Adt19> {
    List::cons(Adt19::C2(v1), v0)
}

pub fn f40(v0: bool, v1: u64) -> R<Bytes> {
    if if v0 {
        let a156 = 0u64;
        let a157 = v1;
        nat_lt(a156, a157)
    } else {
        false
    } {
        let a158 = Bytes::lit(&[0x2d]);
        let a159 = f75(v1)?;
        Ok(append_bytes(a158, a159))
    } else {
        f75(v1)
    }
}

pub fn f41(v0: Bytes) -> R<Bytes> {
    let a165 = {
        let a163 = Bytes::lit(&[0x22]);
        let a164 = {
            let (_, h160) = {
                let (h161, _) = f76(Fn9::F77(v0.clone()), {
                    let a162 = v0;
                    length_bytes(a162)
                }, (0u64, Bytes::lit(&[])))?;
                h161
            };
            h160
        };
        append_bytes(a163, a164)
    };
    let a166 = Bytes::lit(&[0x22]);
    Ok(append_bytes(a165, a166))
}

pub fn f42(v0: List<Bytes>) -> Bytes {
    let a169 = {
        let a167 = Bytes::lit(&[0x5b]);
        let a168 = f78(v0, Bytes::lit(&[0x2c]));
        append_bytes(a167, a168)
    };
    let a170 = Bytes::lit(&[0x5d]);
    append_bytes(a169, a170)
}

pub fn f43(v0: Adt19) -> R<List<Bytes>> {
    let m171 = v0;
    match m171 {
        Adt19::C0(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt19::C1(_, _) => {
            Ok(List::<Bytes>::nil())
        }
        Adt19::C2(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt19::C3(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt19::C4(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt19::C5 => {
            Ok(List::<Bytes>::nil())
        }
        Adt19::C6(r172, r173) => {
            let v7 = (*r172).clone();
            let v8 = (*r173).clone();
            Ok(List::cons(f3(v7)?, f43(v8)?))
        }
        Adt19::C7(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f44(v0: List<(Bytes, Bytes)>) -> R<Bytes> {
    let v1: List<(Bytes, Bytes)> = f79(Fn10::F80, List::<(Bytes, Bytes)>::nil(), f81(v0)?);
    let v2: List<Bytes> = f82(Fn11::F83, List::<Bytes>::nil(), v1)?;
    let a176 = {
        let a174 = Bytes::lit(&[0x7b]);
        let a175 = f78(v2, Bytes::lit(&[0x2c]));
        append_bytes(a174, a175)
    };
    let a177 = Bytes::lit(&[0x7d]);
    Ok(append_bytes(a176, a177))
}

pub fn f45(v0: Adt19) -> R<List<(Bytes, Bytes)>> {
    let m178 = v0;
    match m178 {
        Adt19::C0(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt19::C1(_, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt19::C2(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt19::C3(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt19::C4(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt19::C5 => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt19::C6(_, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt19::C7(v9, r179, r180) => {
            let v10 = (*r179).clone();
            let v11 = (*r180).clone();
            Ok(List::cons((v9, f3(v10)?), f45(v11)?))
        }
    }
}

pub fn f46(v0: Fn3, p184: Adt19, p185: List<Adt19>) -> Adt19 {
    let m183 = (p184, p185);
    {
        let mut m183 = m183;
        loop {
            let step = {
                let (v1, v2) = m183;
                let m181 = v2;
                match m181.uncons() {
                    None => {
                        Err::<(Adt19, List<Adt19>), Adt19>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt19, List<Adt19>), Adt19>(({
                            let c182: Fn3 = v0.clone();
                            c182.apply(v1, v3)
                        }, v4))
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

pub fn f47(v0: Adt19, v1: Adt19) -> Adt19 {
    Adt19::C6(std::rc::Rc::new(v1), std::rc::Rc::new(v0))
}

pub fn f48(v0: List<Adt19>) -> List<Adt19> {
    f84(Fn12::F85, List::<Adt19>::nil(), v0)
}

pub fn f49(v0: Fn4, p189: List<Bytes>, p190: List<Bytes>) -> List<Bytes> {
    let m188 = (p189, p190);
    {
        let mut m188 = m188;
        loop {
            let step = {
                let (v1, v2) = m188;
                let m186 = v2;
                match m186.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c187: Fn4 = v0.clone();
                            c187.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m188 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f50(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f51(v0: Str) -> Bytes {
    let a191 = v0;
    utf8_encode(a191)
}

pub fn f52(v0: Adt2) -> Adt19 {
    let m192 = v0;
    match m192 {
        Adt2::C0 => {
            f86(Str::lit("type"), Adt19::C5)
        }
        Adt2::C1 => {
            f86(Str::lit("nat"), Adt19::C5)
        }
        Adt2::C2 => {
            f86(Str::lit("bool"), Adt19::C5)
        }
        Adt2::C3 => {
            f86(Str::lit("prop"), Adt19::C5)
        }
        Adt2::C4 => {
            f86(Str::lit("unit"), Adt19::C5)
        }
        Adt2::C5 => {
            f86(Str::lit("int"), Adt19::C5)
        }
        Adt2::C6 => {
            f86(Str::lit("int8"), Adt19::C5)
        }
        Adt2::C7 => {
            f86(Str::lit("int16"), Adt19::C5)
        }
        Adt2::C8 => {
            f86(Str::lit("int32"), Adt19::C5)
        }
        Adt2::C9 => {
            f86(Str::lit("int64"), Adt19::C5)
        }
        Adt2::C10 => {
            f86(Str::lit("uint8"), Adt19::C5)
        }
        Adt2::C11 => {
            f86(Str::lit("uint16"), Adt19::C5)
        }
        Adt2::C12 => {
            f86(Str::lit("uint32"), Adt19::C5)
        }
        Adt2::C13 => {
            f86(Str::lit("uint64"), Adt19::C5)
        }
        Adt2::C14 => {
            f86(Str::lit("string"), Adt19::C5)
        }
        Adt2::C15 => {
            f86(Str::lit("bytes"), Adt19::C5)
        }
        Adt2::C16 => {
            f86(Str::lit("ordering"), Adt19::C5)
        }
        Adt2::C17 => {
            f86(Str::lit("contract_violation"), Adt19::C5)
        }
        Adt2::C18 => {
            f86(Str::lit("reasoning_failure"), Adt19::C5)
        }
        Adt2::C19(v1) => {
            f86(Str::lit("parameter"), f87(Str::lit("name"), Adt19::C2(v1), Adt19::C5))
        }
        Adt2::C20(r194) => {
            let v2 = (*r194).clone();
            f86(Str::lit("option"), f87(Str::lit("value"), f52(v2), Adt19::C5))
        }
        Adt2::C21(r195, r196) => {
            let v3 = (*r195).clone();
            let v4 = (*r196).clone();
            f86(Str::lit("result"), f87(Str::lit("ok"), f52(v3), f87(Str::lit("error"), f52(v4), Adt19::C5)))
        }
        Adt2::C22(r197) => {
            let v5 = (*r197).clone();
            f86(Str::lit("list"), f87(Str::lit("element"), f52(v5), Adt19::C5))
        }
        Adt2::C23(v6, v7, r198) => {
            let v8 = (*r198).clone();
            f86(Str::lit("named"), f87(Str::lit("member"), Adt19::C4(std::rc::Rc::new({
                let m193 = v6;
                match m193 {
                    None => {
                        f87(Str::lit("name"), Adt19::C2(v7), Adt19::C5)
                    }
                    Some(v9) => {
                        f87(Str::lit("module"), Adt19::C2(v9), f87(Str::lit("name"), Adt19::C2(v7), Adt19::C5))
                    }
                }
            })), f87(Str::lit("arguments"), Adt19::C3(std::rc::Rc::new(f52(v8))), Adt19::C5)))
        }
        Adt2::C24(r199, r200) => {
            let v10 = (*r199).clone();
            let v11 = (*r200).clone();
            f86(Str::lit("product"), f87(Str::lit("left"), f52(v10), f87(Str::lit("right"), f52(v11), Adt19::C5)))
        }
        Adt2::C25(r201, r202) => {
            let v12 = (*r201).clone();
            let v13 = (*r202).clone();
            f86(Str::lit("function"), f87(Str::lit("parameters"), Adt19::C3(std::rc::Rc::new(f52(v12))), f87(Str::lit("result"), f52(v13), Adt19::C5)))
        }
        Adt2::C26(r203, r204) => {
            let v14 = (*r203).clone();
            let v15 = (*r204).clone();
            f86(Str::lit("map"), f87(Str::lit("key"), f52(v14), f87(Str::lit("value"), f52(v15), Adt19::C5)))
        }
        Adt2::C27(r205) => {
            let v16 = (*r205).clone();
            f86(Str::lit("set"), f87(Str::lit("element"), f52(v16), Adt19::C5))
        }
        Adt2::C28 => {
            Adt19::C5
        }
        Adt2::C29(r206, r207) => {
            let v17 = (*r206).clone();
            let v18 = (*r207).clone();
            Adt19::C6(std::rc::Rc::new(f52(v17)), std::rc::Rc::new(f52(v18)))
        }
    }
}

pub fn f53(v0: List<(Bytes, Adt19)>) -> Adt19 {
    Adt19::C4(std::rc::Rc::new(f55(Fn5::F88, Adt19::C5, v0)))
}

pub fn f54(v0: Str, v1: Adt19, v2: Adt19) -> Adt19 {
    Adt19::C7(f89(v0), std::rc::Rc::new(v1), std::rc::Rc::new(v2))
}

pub fn f55(v0: Fn5, p211: Adt19, p212: List<(Bytes, Adt19)>) -> Adt19 {
    let m210 = (p211, p212);
    {
        let mut m210 = m210;
        loop {
            let step = {
                let (v1, v2) = m210;
                let m208 = v2;
                match m208.uncons() {
                    None => {
                        Err::<(Adt19, List<(Bytes, Adt19)>), Adt19>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt19, List<(Bytes, Adt19)>), Adt19>(({
                            let c209: Fn5 = v0.clone();
                            c209.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m210 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f56(v0: Adt19, v1: (Bytes, Adt19)) -> Adt19 {
    Adt19::C7({
        let (h213, _) = v1;
        h213
    }, std::rc::Rc::new({
        let (_, h214) = v1;
        h214
    }), std::rc::Rc::new(v0))
}

pub fn f57(v0: Fn6, p218: List<Adt19>, p219: List<Adt2>) -> List<Adt19> {
    let m217 = (p218, p219);
    {
        let mut m217 = m217;
        loop {
            let step = {
                let (v1, v2) = m217;
                let m215 = v2;
                match m215.uncons() {
                    None => {
                        Err::<(List<Adt19>, List<Adt2>), List<Adt19>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt19>, List<Adt2>), List<Adt19>>(({
                            let c216: Fn6 = v0.clone();
                            c216.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m217 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f58(v0: List<Adt19>, v1: Adt2) -> List<Adt19> {
    List::cons(f52(v1), v0)
}

pub fn f59(v0: List<Adt2>) -> List<Adt2> {
    f90(Fn13::F91, List::<Adt2>::nil(), v0)
}

pub fn f60(v0: Str, v1: Adt19) -> Adt19 {
    Adt19::C4(std::rc::Rc::new(f54(Str::lit("kind"), Adt19::C2(f89(v0)), v1)))
}

pub fn f61(v0: Adt7) -> Bytes {
    let m220 = v0;
    match m220 {
        Adt7::C0 => {
            f92(Str::lit("int"))
        }
        Adt7::C1 => {
            f92(Str::lit("int8"))
        }
        Adt7::C2 => {
            f92(Str::lit("int16"))
        }
        Adt7::C3 => {
            f92(Str::lit("int32"))
        }
        Adt7::C4 => {
            f92(Str::lit("int64"))
        }
        Adt7::C5 => {
            f92(Str::lit("uint8"))
        }
        Adt7::C6 => {
            f92(Str::lit("uint16"))
        }
        Adt7::C7 => {
            f92(Str::lit("uint32"))
        }
        Adt7::C8 => {
            f92(Str::lit("uint64"))
        }
    }
}

pub fn f62(v0: Adt8) -> Adt20 {
    let m221 = v0;
    match m221 {
        Adt8::C0 => {
            Adt20::C0(f92(Str::lit("subtract")), false, false, None::<u64>)
        }
        Adt8::C1 => {
            Adt20::C0(f92(Str::lit("multiply")), false, false, None::<u64>)
        }
        Adt8::C2 => {
            Adt20::C0(f92(Str::lit("quotient")), false, false, None::<u64>)
        }
        Adt8::C3 => {
            Adt20::C0(f92(Str::lit("remainder")), false, false, None::<u64>)
        }
        Adt8::C4 => {
            Adt20::C0(f92(Str::lit("negate")), false, false, None::<u64>)
        }
        Adt8::C5 => {
            Adt20::C0(f92(Str::lit("checked_convert")), false, false, None::<u64>)
        }
        Adt8::C6 => {
            Adt20::C0(f92(Str::lit("checked_add")), false, false, None::<u64>)
        }
        Adt8::C7 => {
            Adt20::C0(f92(Str::lit("checked_subtract")), false, false, None::<u64>)
        }
        Adt8::C8 => {
            Adt20::C0(f92(Str::lit("checked_multiply")), false, false, None::<u64>)
        }
        Adt8::C9 => {
            Adt20::C0(f92(Str::lit("checked_negate")), false, false, None::<u64>)
        }
        Adt8::C10 => {
            Adt20::C0(f92(Str::lit("checked_quotient")), false, false, None::<u64>)
        }
        Adt8::C11 => {
            Adt20::C0(f92(Str::lit("bit_and")), false, false, None::<u64>)
        }
        Adt8::C12 => {
            Adt20::C0(f92(Str::lit("bit_or")), false, false, None::<u64>)
        }
        Adt8::C13 => {
            Adt20::C0(f92(Str::lit("bit_xor")), false, false, None::<u64>)
        }
        Adt8::C14 => {
            Adt20::C0(f92(Str::lit("bit_not")), false, false, None::<u64>)
        }
        Adt8::C15 => {
            Adt20::C0(f92(Str::lit("shift_left")), false, false, None::<u64>)
        }
        Adt8::C16 => {
            Adt20::C0(f92(Str::lit("shift_right")), false, false, None::<u64>)
        }
        Adt8::C17 => {
            Adt20::C0(f92(Str::lit("append")), false, false, None::<u64>)
        }
        Adt8::C18 => {
            Adt20::C0(f92(Str::lit("length")), false, false, None::<u64>)
        }
        Adt8::C19 => {
            Adt20::C0(f92(Str::lit("index")), false, false, None::<u64>)
        }
        Adt8::C20 => {
            Adt20::C0(f92(Str::lit("slice")), false, false, None::<u64>)
        }
        Adt8::C21 => {
            Adt20::C0(f92(Str::lit("utf8_encode")), false, false, None::<u64>)
        }
        Adt8::C22 => {
            Adt20::C0(f92(Str::lit("utf8_decode")), false, false, None::<u64>)
        }
        Adt8::C23 => {
            Adt20::C0(f92(Str::lit("compare_bytes")), false, false, None::<u64>)
        }
        Adt8::C24 => {
            Adt20::C0(f92(Str::lit("equal")), false, false, None::<u64>)
        }
        Adt8::C25 => {
            Adt20::C0(f92(Str::lit("split_exact")), false, false, None::<u64>)
        }
        Adt8::C26 => {
            Adt20::C0(f92(Str::lit("join")), false, false, None::<u64>)
        }
        Adt8::C27 => {
            Adt20::C0(f92(Str::lit("parse_decimal")), false, false, None::<u64>)
        }
        Adt8::C28 => {
            Adt20::C0(f92(Str::lit("format_decimal")), false, false, None::<u64>)
        }
        Adt8::C29 => {
            Adt20::C0(f92(Str::lit("map_insert")), true, false, None::<u64>)
        }
        Adt8::C30 => {
            Adt20::C0(f92(Str::lit("map_remove")), true, false, None::<u64>)
        }
        Adt8::C31 => {
            Adt20::C0(f92(Str::lit("map_lookup")), true, false, None::<u64>)
        }
        Adt8::C32 => {
            Adt20::C0(f92(Str::lit("map_contains")), true, false, None::<u64>)
        }
        Adt8::C33 => {
            Adt20::C0(f92(Str::lit("map_size")), true, false, None::<u64>)
        }
        Adt8::C34 => {
            Adt20::C0(f92(Str::lit("map_keys")), true, false, None::<u64>)
        }
        Adt8::C35 => {
            Adt20::C0(f92(Str::lit("map_values")), true, false, None::<u64>)
        }
        Adt8::C36 => {
            Adt20::C0(f92(Str::lit("map_entries")), true, false, None::<u64>)
        }
        Adt8::C37 => {
            Adt20::C0(f92(Str::lit("map_fold")), true, false, Some(0u64))
        }
        Adt8::C38 => {
            Adt20::C0(f92(Str::lit("set_insert")), true, false, None::<u64>)
        }
        Adt8::C39 => {
            Adt20::C0(f92(Str::lit("set_remove")), true, false, None::<u64>)
        }
        Adt8::C40 => {
            Adt20::C0(f92(Str::lit("set_contains")), true, false, None::<u64>)
        }
        Adt8::C41 => {
            Adt20::C0(f92(Str::lit("set_size")), true, false, None::<u64>)
        }
        Adt8::C42 => {
            Adt20::C0(f92(Str::lit("set_elements")), true, false, None::<u64>)
        }
        Adt8::C43 => {
            Adt20::C0(f92(Str::lit("set_union")), true, false, None::<u64>)
        }
        Adt8::C44 => {
            Adt20::C0(f92(Str::lit("set_intersection")), true, false, None::<u64>)
        }
        Adt8::C45 => {
            Adt20::C0(f92(Str::lit("set_difference")), true, false, None::<u64>)
        }
        Adt8::C46 => {
            Adt20::C0(f92(Str::lit("set_fold")), true, false, Some(0u64))
        }
        Adt8::C47 => {
            Adt20::C0(f92(Str::lit("list_fold")), true, false, Some(0u64))
        }
        Adt8::C48 => {
            Adt20::C0(f92(Str::lit("iterate")), true, false, Some(0u64))
        }
        Adt8::C49 => {
            Adt20::C0(f92(Str::lit("iterate_until")), true, false, Some(0u64))
        }
        Adt8::C50 => {
            Adt20::C0(f92(Str::lit("graph_successors")), true, false, None::<u64>)
        }
        Adt8::C51 => {
            Adt20::C0(f92(Str::lit("graph_reachable")), true, false, None::<u64>)
        }
        Adt8::C52 => {
            Adt20::C0(f92(Str::lit("graph_topological")), true, false, None::<u64>)
        }
        Adt8::C53 => {
            Adt20::C0(f92(Str::lit("less_than")), true, false, None::<u64>)
        }
        Adt8::C54 => {
            Adt20::C0(f92(Str::lit("to_int")), true, true, None::<u64>)
        }
        Adt8::C55 => {
            Adt20::C0(f92(Str::lit("to_nat")), true, true, None::<u64>)
        }
    }
}

pub fn f63(v0: Str, v1: Adt2, v2: Adt19) -> Adt19 {
    let m222 = v1.clone();
    match m222 {
        Adt2::C0 => {
            v2
        }
        Adt2::C1 => {
            v2
        }
        Adt2::C2 => {
            v2
        }
        Adt2::C3 => {
            v2
        }
        Adt2::C4 => {
            v2
        }
        Adt2::C5 => {
            v2
        }
        Adt2::C6 => {
            v2
        }
        Adt2::C7 => {
            v2
        }
        Adt2::C8 => {
            v2
        }
        Adt2::C9 => {
            v2
        }
        Adt2::C10 => {
            v2
        }
        Adt2::C11 => {
            v2
        }
        Adt2::C12 => {
            v2
        }
        Adt2::C13 => {
            v2
        }
        Adt2::C14 => {
            v2
        }
        Adt2::C15 => {
            v2
        }
        Adt2::C16 => {
            v2
        }
        Adt2::C17 => {
            v2
        }
        Adt2::C18 => {
            v2
        }
        Adt2::C19(_) => {
            v2
        }
        Adt2::C20(_) => {
            v2
        }
        Adt2::C21(_, _) => {
            v2
        }
        Adt2::C22(_) => {
            v2
        }
        Adt2::C23(_, _, _) => {
            v2
        }
        Adt2::C24(_, _) => {
            v2
        }
        Adt2::C25(_, _) => {
            v2
        }
        Adt2::C26(_, _) => {
            v2
        }
        Adt2::C27(_) => {
            v2
        }
        Adt2::C28 => {
            v2
        }
        Adt2::C29(_, _) => {
            f54(v0, Adt19::C3(std::rc::Rc::new(f52(v1))), v2)
        }
    }
}

pub fn f64(v0: Bytes, v1: Adt2) -> Adt19 {
    Adt19::C4(std::rc::Rc::new(f54(Str::lit("name"), Adt19::C2(v0), f54(Str::lit("type"), f52(v1), Adt19::C5))))
}

pub fn f65(v0: List<(Bytes, Adt2)>) -> Adt19 {
    Adt19::C3(std::rc::Rc::new(f5(f93(Fn14::F94, List::<Adt19>::nil(), f95(v0)))))
}

pub fn f66(v0: List<Bytes>) -> Adt19 {
    Adt19::C3(std::rc::Rc::new(f5(f6(Fn0::F96, List::<Adt19>::nil(), f8(v0)))))
}

pub fn f67(v0: Fn7, p226: List<Adt4>, p227: List<Adt4>) -> List<Adt4> {
    let m225 = (p226, p227);
    {
        let mut m225 = m225;
        loop {
            let step = {
                let (v1, v2) = m225;
                let m223 = v2;
                match m223.uncons() {
                    None => {
                        Err::<(List<Adt4>, List<Adt4>), List<Adt4>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt4>, List<Adt4>), List<Adt4>>(({
                            let c224: Fn7 = v0.clone();
                            c224.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m225 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f68(v0: List<Adt4>, v1: Adt4) -> List<Adt4> {
    List::cons(v1, v0)
}

pub fn f69(v0: Adt11) -> Adt19 {
    let m228 = v0;
    match m228 {
        Adt11::C0 => {
            Adt19::C2(f4(Str::lit("breadth_first")))
        }
        Adt11::C1 => {
            Adt19::C2(f4(Str::lit("depth_first")))
        }
    }
}

pub fn f70(v0: Adt13) -> Adt19 {
    let m229 = v0;
    match m229 {
        Adt13::C0(v1) => {
            let v2: List<(Bytes, Adt19)> = List::cons((f4(Str::lit("kind")), Adt19::C2(f4(Str::lit("initial_invariant")))), List::<(Bytes, Adt19)>::nil());
            let v3: List<(Bytes, Adt19)> = List::cons((f4(Str::lit("theorem")), f10({
                let (h230, _) = v1;
                h230
            }, {
                let (_, h231) = v1;
                h231
            })), v2);
            f13(v3)
        }
        Adt13::C1(v4) => {
            let v5: List<(Bytes, Adt19)> = List::cons((f4(Str::lit("kind")), Adt19::C2(f4(Str::lit("terminates")))), List::<(Bytes, Adt19)>::nil());
            let v6: List<(Bytes, Adt19)> = List::cons((f4(Str::lit("theorem")), f10({
                let (h232, _) = v4;
                h232
            }, {
                let (_, h233) = v4;
                h233
            })), v5);
            f13(v6)
        }
        Adt13::C2(v7, v8, v9) => {
            let v10: List<(Bytes, Adt19)> = List::cons((f4(Str::lit("kind")), Adt19::C2(f4(Str::lit("observation_invariant")))), List::<(Bytes, Adt19)>::nil());
            let v11: List<(Bytes, Adt19)> = List::cons((f4(Str::lit("predicate")), f10({
                let (h234, _) = v7;
                h234
            }, {
                let (_, h235) = v7;
                h235
            })), v10);
            let v12: List<(Bytes, Adt19)> = List::cons((f4(Str::lit("initial")), f10({
                let (h236, _) = v8;
                h236
            }, {
                let (_, h237) = v8;
                h237
            })), v11);
            let v13: List<(Bytes, Adt19)> = List::cons((f4(Str::lit("preserved")), f10({
                let (h238, _) = v9;
                h238
            }, {
                let (_, h239) = v9;
                h239
            })), v12);
            f13(v13)
        }
        Adt13::C3(v14) => {
            let v15: List<(Bytes, Adt19)> = List::cons((f4(Str::lit("kind")), Adt19::C2(f4(Str::lit("answer_correct")))), List::<(Bytes, Adt19)>::nil());
            let v16: List<(Bytes, Adt19)> = List::cons((f4(Str::lit("theorem")), f10({
                let (h240, _) = v14;
                h240
            }, {
                let (_, h241) = v14;
                h241
            })), v15);
            f13(v16)
        }
    }
}

pub fn f71(v0: Fn8, p245: List<Adt13>, p246: List<Adt13>) -> List<Adt13> {
    let m244 = (p245, p246);
    {
        let mut m244 = m244;
        loop {
            let step = {
                let (v1, v2) = m244;
                let m242 = v2;
                match m242.uncons() {
                    None => {
                        Err::<(List<Adt13>, List<Adt13>), List<Adt13>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt13>, List<Adt13>), List<Adt13>>(({
                            let c243: Fn8 = v0.clone();
                            c243.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m244 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f72(v0: List<Adt13>, v1: Adt13) -> List<Adt13> {
    List::cons(v1, v0)
}

pub fn f73(v0: Adt17) -> Adt19 {
    let m247 = v0;
    match m247 {
        Adt17::C0 => {
            Adt19::C2(f4(Str::lit("xorshift32")))
        }
    }
}

pub fn f74(v0: Adt18) -> Adt19 {
    let m248 = v0;
    match m248 {
        Adt18::C0 => {
            Adt19::C2(f4(Str::lit("proportional")))
        }
    }
}

pub fn f75(v0: u64) -> R<Bytes> {
    f97(f98(), 10u64, v0)
}

pub fn f76(v0: Fn9, p253: u64, p254: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m252 = (p253, p254);
    {
        let mut m252 = m252;
        loop {
            let step = {
                let (v1, v2) = m252;
                let m249 = v1;
                if m249 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m249 - 1;
                    let m251 = {
                        let c250: Fn9 = v0.clone();
                        c250.apply(v2.clone())?
                    };
                    match m251 {
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
                Ok(next) => { m252 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f77(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f99(v0, v1)
}

pub fn f78(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m255 = f100(Fn15::F101(v1), None::<Bytes>, v0);
    match m255 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f79(v0: Fn10, p259: List<(Bytes, Bytes)>, p260: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m258 = (p259, p260);
    {
        let mut m258 = m258;
        loop {
            let step = {
                let (v1, v2) = m258;
                let m256 = v2;
                match m256.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c257: Fn10 = v0.clone();
                            c257.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m258 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f80(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    f102(v0, v1)
}

pub fn f81(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    f103(v0)
}

pub fn f82(v0: Fn11, p264: List<Bytes>, p265: List<(Bytes, Bytes)>) -> R<List<Bytes>> {
    let m263 = (p264, p265);
    {
        let mut m263 = m263;
        loop {
            let step = {
                let (v1, v2) = m263;
                let m261 = v2;
                match m261.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(({
                            let c262: Fn11 = v0.clone();
                            c262.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m263 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f83(v0: List<Bytes>, v1: (Bytes, Bytes)) -> R<List<Bytes>> {
    Ok(List::cons(f104(v1)?, v0))
}

pub fn f84(v0: Fn12, p269: List<Adt19>, p270: List<Adt19>) -> List<Adt19> {
    let m268 = (p269, p270);
    {
        let mut m268 = m268;
        loop {
            let step = {
                let (v1, v2) = m268;
                let m266 = v2;
                match m266.uncons() {
                    None => {
                        Err::<(List<Adt19>, List<Adt19>), List<Adt19>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt19>, List<Adt19>), List<Adt19>>(({
                            let c267: Fn12 = v0.clone();
                            c267.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m268 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f85(v0: List<Adt19>, v1: Adt19) -> List<Adt19> {
    List::cons(v1, v0)
}

pub fn f86(v0: Str, v1: Adt19) -> Adt19 {
    Adt19::C4(std::rc::Rc::new(f87(Str::lit("kind"), Adt19::C2(f92(v0)), v1)))
}

pub fn f87(v0: Str, v1: Adt19, v2: Adt19) -> Adt19 {
    Adt19::C7(f92(v0), std::rc::Rc::new(v1), std::rc::Rc::new(v2))
}

pub fn f88(v0: Adt19, v1: (Bytes, Adt19)) -> Adt19 {
    Adt19::C7({
        let (h271, _) = v1;
        h271
    }, std::rc::Rc::new({
        let (_, h272) = v1;
        h272
    }), std::rc::Rc::new(v0))
}

pub fn f89(v0: Str) -> Bytes {
    let a273 = v0;
    utf8_encode(a273)
}

pub fn f90(v0: Fn13, p277: List<Adt2>, p278: List<Adt2>) -> List<Adt2> {
    let m276 = (p277, p278);
    {
        let mut m276 = m276;
        loop {
            let step = {
                let (v1, v2) = m276;
                let m274 = v2;
                match m274.uncons() {
                    None => {
                        Err::<(List<Adt2>, List<Adt2>), List<Adt2>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt2>, List<Adt2>), List<Adt2>>(({
                            let c275: Fn13 = v0.clone();
                            c275.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m276 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f91(v0: List<Adt2>, v1: Adt2) -> List<Adt2> {
    List::cons(v1, v0)
}

pub fn f92(v0: Str) -> Bytes {
    let a279 = v0;
    utf8_encode(a279)
}

pub fn f93(v0: Fn14, p283: List<Adt19>, p284: List<(Bytes, Adt2)>) -> List<Adt19> {
    let m282 = (p283, p284);
    {
        let mut m282 = m282;
        loop {
            let step = {
                let (v1, v2) = m282;
                let m280 = v2;
                match m280.uncons() {
                    None => {
                        Err::<(List<Adt19>, List<(Bytes, Adt2)>), List<Adt19>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt19>, List<(Bytes, Adt2)>), List<Adt19>>(({
                            let c281: Fn14 = v0.clone();
                            c281.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m282 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f94(v0: List<Adt19>, v1: (Bytes, Adt2)) -> List<Adt19> {
    List::cons(f64({
        let (h285, _) = v1;
        h285
    }, {
        let (_, h286) = v1;
        h286
    }), v0)
}

pub fn f95(v0: List<(Bytes, Adt2)>) -> List<(Bytes, Adt2)> {
    f105(Fn16::F106, List::<(Bytes, Adt2)>::nil(), v0)
}

pub fn f96(v0: List<Adt19>, v1: Bytes) -> List<Adt19> {
    List::cons(Adt19::C2(v1), v0)
}

pub fn f97(v0: Bytes, v1: u64, v2: u64) -> R<Bytes> {
    let a287 = v2;
    let a288 = 0u64;
    if nat_eq(a287, a288) {
        Ok(f107(v0, 0u64))
    } else {
        let (_, h289) = {
            let (h290, _) = f76(Fn9::F108(v1, v0), v2, (v2, Bytes::lit(&[])))?;
            h290
        };
        Ok(h289)
    }
}

pub fn f98() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f99(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let m294 = {
        let a292 = v0.clone();
        let a293 = {
            let (h291, _) = v1.clone();
            h291
        };
        index_bytes(a292, a293)
    };
    match m294 {
        None => {
            Ok(None::<(u64, Bytes)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a296 = {
                    let (h295, _) = v1.clone();
                    h295
                };
                let a297 = 1u64;
                nat_add(a296, a297)?
            }, {
                let a300 = {
                    let (_, h298) = v1;
                    h298
                };
                let a301 = f109(v0, {
                    let (h299, _) = v1;
                    h299
                }, v2)?;
                append_bytes(a300, a301)
            })))
        }
    }
}

pub fn f100(v0: Fn15, p305: Option<Bytes>, p306: List<Bytes>) -> Option<Bytes> {
    let m304 = (p305, p306);
    {
        let mut m304 = m304;
        loop {
            let step = {
                let (v1, v2) = m304;
                let m302 = v2;
                match m302.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(({
                            let c303: Fn15 = v0.clone();
                            c303.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m304 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f101(v0: Bytes, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m307 = v1;
    match m307 {
        None => {
            Some(v2)
        }
        Some(v3) => {
            Some({
                let a310 = {
                    let a308 = v3;
                    let a309 = v0;
                    append_bytes(a308, a309)
                };
                let a311 = v2;
                append_bytes(a310, a311)
            })
        }
    }
}

pub fn f102(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    let m312 = v0.clone();
    match m312.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, v3)) => {
            let a315 = {
                let (h313, _) = v2;
                h313
            };
            let a316 = {
                let (h314, _) = v1.clone();
                h314
            };
            if equal(a315, a316) {
                List::cons(v1, v3)
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f103(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v1: List<List<(Bytes, Bytes)>> = f110(f111(Fn17::F112, List::<List<(Bytes, Bytes)>>::nil(), v0.clone()));
    let m321 = {
        let (h317, _) = f113(Fn18::F114, {
            let a319 = {
                let a318 = v0;
                length_list(a318)
            };
            let a320 = 1u64;
            nat_add(a319, a320)?
        }, v1)?;
        h317
    };
    match m321.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f104(v0: (Bytes, Bytes)) -> R<Bytes> {
    let a326 = {
        let a323 = f41({
            let (h322, _) = v0;
            h322
        })?;
        let a324 = Bytes::lit(&[0x3a]);
        append_bytes(a323, a324)
    };
    let a327 = {
        let (_, h325) = v0;
        h325
    };
    Ok(append_bytes(a326, a327))
}

pub fn f105(v0: Fn16, p331: List<(Bytes, Adt2)>, p332: List<(Bytes, Adt2)>) -> List<(Bytes, Adt2)> {
    let m330 = (p331, p332);
    {
        let mut m330 = m330;
        loop {
            let step = {
                let (v1, v2) = m330;
                let m328 = v2;
                match m328.uncons() {
                    None => {
                        Err::<(List<(Bytes, Adt2)>, List<(Bytes, Adt2)>), List<(Bytes, Adt2)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Adt2)>, List<(Bytes, Adt2)>), List<(Bytes, Adt2)>>(({
                            let c329: Fn16 = v0.clone();
                            c329.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m330 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f106(v0: List<(Bytes, Adt2)>, v1: (Bytes, Adt2)) -> List<(Bytes, Adt2)> {
    List::cons(v1, v0)
}

pub fn f107(v0: Bytes, v1: u64) -> Bytes {
    let m336 = {
        let a333 = v0;
        let a334 = v1;
        let a335 = 1u64;
        slice_bytes(a333, a334, a335)
    };
    match m336 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f108(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f115(v1, v0, v2)
}

pub fn f109(v0: Bytes, v1: u64, v2: u8) -> R<Bytes> {
    let v4: u64 = {
        let v3: u8 = v2;
        let a381 = {
            let a357 = {
                let a345 = {
                    let a339 = {
                        let a337 = v3;
                        let a338 = 128u8;
                        fixed_u8::bit_and(a337, a338)
                    };
                    let a340 = 128u8;
                    if equal(a339, a340) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a346 = {
                    let a343 = {
                        let a341 = v3;
                        let a342 = 64u8;
                        fixed_u8::bit_and(a341, a342)
                    };
                    let a344 = 64u8;
                    if equal(a343, a344) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a345, a346)?
            };
            let a358 = {
                let a355 = {
                    let a349 = {
                        let a347 = v3;
                        let a348 = 32u8;
                        fixed_u8::bit_and(a347, a348)
                    };
                    let a350 = 32u8;
                    if equal(a349, a350) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a356 = {
                    let a353 = {
                        let a351 = v3;
                        let a352 = 16u8;
                        fixed_u8::bit_and(a351, a352)
                    };
                    let a354 = 16u8;
                    if equal(a353, a354) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a355, a356)?
            };
            nat_add(a357, a358)?
        };
        let a382 = {
            let a379 = {
                let a367 = {
                    let a361 = {
                        let a359 = v3;
                        let a360 = 8u8;
                        fixed_u8::bit_and(a359, a360)
                    };
                    let a362 = 8u8;
                    if equal(a361, a362) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a368 = {
                    let a365 = {
                        let a363 = v3;
                        let a364 = 4u8;
                        fixed_u8::bit_and(a363, a364)
                    };
                    let a366 = 4u8;
                    if equal(a365, a366) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a367, a368)?
            };
            let a380 = {
                let a377 = {
                    let a371 = {
                        let a369 = v3;
                        let a370 = 2u8;
                        fixed_u8::bit_and(a369, a370)
                    };
                    let a372 = 2u8;
                    if equal(a371, a372) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a378 = {
                    let a375 = {
                        let a373 = v3;
                        let a374 = 1u8;
                        fixed_u8::bit_and(a373, a374)
                    };
                    let a376 = 1u8;
                    if equal(a375, a376) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a377, a378)?
            };
            nat_add(a379, a380)?
        };
        nat_add(a381, a382)?
    };
    let a383 = v4;
    let a384 = 34u64;
    if nat_eq(a383, a384) {
        Ok(Bytes::lit(&[0x5c, 0x22]))
    } else {
        let a385 = v4;
        let a386 = 92u64;
        if nat_eq(a385, a386) {
            Ok(Bytes::lit(&[0x5c, 0x5c]))
        } else {
            let a387 = v4;
            let a388 = 8u64;
            if nat_eq(a387, a388) {
                Ok(Bytes::lit(&[0x5c, 0x62]))
            } else {
                let a389 = v4;
                let a390 = 12u64;
                if nat_eq(a389, a390) {
                    Ok(Bytes::lit(&[0x5c, 0x66]))
                } else {
                    let a391 = v4;
                    let a392 = 10u64;
                    if nat_eq(a391, a392) {
                        Ok(Bytes::lit(&[0x5c, 0x6e]))
                    } else {
                        let a393 = v4;
                        let a394 = 13u64;
                        if nat_eq(a393, a394) {
                            Ok(Bytes::lit(&[0x5c, 0x72]))
                        } else {
                            let a395 = v4;
                            let a396 = 9u64;
                            if nat_eq(a395, a396) {
                                Ok(Bytes::lit(&[0x5c, 0x74]))
                            } else {
                                let a397 = v4;
                                let a398 = 32u64;
                                if nat_lt(a397, a398) {
                                    let a399 = Bytes::lit(&[0x5c, 0x75, 0x30, 0x30]);
                                    let a400 = f116(v2)?;
                                    Ok(append_bytes(a399, a400))
                                } else {
                                    Ok(f117(v0, v1, 1u64))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f110(v0: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    f118(Fn19::F119, List::<List<(Bytes, Bytes)>>::nil(), v0)
}

pub fn f111(v0: Fn17, p404: List<List<(Bytes, Bytes)>>, p405: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    let m403 = (p404, p405);
    {
        let mut m403 = m403;
        loop {
            let step = {
                let (v1, v2) = m403;
                let m401 = v2;
                match m401.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(({
                            let c402: Fn17 = v0.clone();
                            c402.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m403 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f112(v0: List<List<(Bytes, Bytes)>>, v1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
    List::cons(List::cons(v1, List::<(Bytes, Bytes)>::nil()), v0)
}

pub fn f113(v0: Fn18, p410: u64, p411: List<List<(Bytes, Bytes)>>) -> R<(List<List<(Bytes, Bytes)>>, bool)> {
    let m409 = (p410, p411);
    {
        let mut m409 = m409;
        loop {
            let step = {
                let (v1, v2) = m409;
                let m406 = v1;
                if m406 == 0 {
                    Err::<(u64, List<List<(Bytes, Bytes)>>), R<(List<List<(Bytes, Bytes)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m406 - 1;
                    let m408 = {
                        let c407: Fn18 = v0.clone();
                        c407.apply(v2.clone())?
                    };
                    match m408 {
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
                Ok(next) => { m409 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f114(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    f120(v0)
}

pub fn f115(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h412, _) = v2;
        h412
    };
    let a413 = v3;
    let a414 = 0u64;
    if nat_eq(a413, a414) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a415 = v3;
            let a416 = v1;
            let a417 = 0u64;
            nat_quot(a415, a416, a417)
        }, {
            let a422 = f107(v0, {
                let a418 = v3;
                let a419 = v1;
                let a420 = 0u64;
                nat_rem(a418, a419, a420)
            });
            let a423 = {
                let (_, h421) = v2;
                h421
            };
            append_bytes(a422, a423)
        }))
    }
}

pub fn f116(v0: u8) -> R<Bytes> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a468 = {
            let a444 = {
                let a432 = {
                    let a426 = {
                        let a424 = v1;
                        let a425 = 128u8;
                        fixed_u8::bit_and(a424, a425)
                    };
                    let a427 = 128u8;
                    if equal(a426, a427) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a433 = {
                    let a430 = {
                        let a428 = v1;
                        let a429 = 64u8;
                        fixed_u8::bit_and(a428, a429)
                    };
                    let a431 = 64u8;
                    if equal(a430, a431) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a432, a433)?
            };
            let a445 = {
                let a442 = {
                    let a436 = {
                        let a434 = v1;
                        let a435 = 32u8;
                        fixed_u8::bit_and(a434, a435)
                    };
                    let a437 = 32u8;
                    if equal(a436, a437) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a443 = {
                    let a440 = {
                        let a438 = v1;
                        let a439 = 16u8;
                        fixed_u8::bit_and(a438, a439)
                    };
                    let a441 = 16u8;
                    if equal(a440, a441) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a442, a443)?
            };
            nat_add(a444, a445)?
        };
        let a469 = {
            let a466 = {
                let a454 = {
                    let a448 = {
                        let a446 = v1;
                        let a447 = 8u8;
                        fixed_u8::bit_and(a446, a447)
                    };
                    let a449 = 8u8;
                    if equal(a448, a449) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a455 = {
                    let a452 = {
                        let a450 = v1;
                        let a451 = 4u8;
                        fixed_u8::bit_and(a450, a451)
                    };
                    let a453 = 4u8;
                    if equal(a452, a453) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a454, a455)?
            };
            let a467 = {
                let a464 = {
                    let a458 = {
                        let a456 = v1;
                        let a457 = 2u8;
                        fixed_u8::bit_and(a456, a457)
                    };
                    let a459 = 2u8;
                    if equal(a458, a459) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a465 = {
                    let a462 = {
                        let a460 = v1;
                        let a461 = 1u8;
                        fixed_u8::bit_and(a460, a461)
                    };
                    let a463 = 1u8;
                    if equal(a462, a463) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a464, a465)?
            };
            nat_add(a466, a467)?
        };
        nat_add(a468, a469)?
    };
    let a476 = f107(f121(), {
        let a470 = v2;
        let a471 = 16u64;
        let a472 = 0u64;
        nat_quot(a470, a471, a472)
    });
    let a477 = f107(f121(), {
        let a473 = v2;
        let a474 = 16u64;
        let a475 = 0u64;
        nat_rem(a473, a474, a475)
    });
    Ok(append_bytes(a476, a477))
}

pub fn f117(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m481 = {
        let a478 = v0;
        let a479 = v1;
        let a480 = v2;
        slice_bytes(a478, a479, a480)
    };
    match m481 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f118(v0: Fn19, p485: List<List<(Bytes, Bytes)>>, p486: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    let m484 = (p485, p486);
    {
        let mut m484 = m484;
        loop {
            let step = {
                let (v1, v2) = m484;
                let m482 = v2;
                match m482.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(({
                            let c483: Fn19 = v0.clone();
                            c483.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m484 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f119(v0: List<List<(Bytes, Bytes)>>, v1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    List::cons(v1, v0)
}

pub fn f120(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    let a488 = {
        let a487 = v0.clone();
        length_list(a487)
    };
    let a489 = 1u64;
    if nat_le(a488, a489) {
        Ok(None::<List<List<(Bytes, Bytes)>>>)
    } else {
        Ok(Some(f122(v0)?))
    }
}

pub fn f121() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66])
}

pub fn f122(v0: List<List<(Bytes, Bytes)>>) -> R<List<List<(Bytes, Bytes)>>> {
    Ok(f110({
        let (_, h490) = {
            let (h491, _) = f123(Fn20::F124, {
                let a493 = {
                    let a492 = v0.clone();
                    length_list(a492)
                };
                let a494 = 1u64;
                nat_add(a493, a494)?
            }, (v0, List::<List<(Bytes, Bytes)>>::nil()))?;
            h491
        };
        h490
    }))
}

pub fn f123(v0: Fn20, p499: u64, p500: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)> {
    let m498 = (p499, p500);
    {
        let mut m498 = m498;
        loop {
            let step = {
                let (v1, v2) = m498;
                let m495 = v1;
                if m495 == 0 {
                    Err::<(u64, (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)), R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m495 - 1;
                    let m497 = {
                        let c496: Fn20 = v0.clone();
                        c496.apply(v2.clone())?
                    };
                    match m497 {
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
                Ok(next) => { m498 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f124(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    f125(v0)
}

pub fn f125(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    let v1: List<List<(Bytes, Bytes)>> = {
        let (h501, _) = v0;
        h501
    };
    let v2: List<List<(Bytes, Bytes)>> = {
        let (_, h502) = v0;
        h502
    };
    let m503 = v1;
    match m503.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>)
        }
        Some((v3, v4)) => {
            let m504 = v4;
            match m504.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Bytes)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f126(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f126(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v2: u64 = {
        let a509 = {
            let a507 = {
                let a505 = v0.clone();
                length_list(a505)
            };
            let a508 = {
                let a506 = v1.clone();
                length_list(a506)
            };
            nat_add(a507, a508)?
        };
        let a510 = 1u64;
        nat_add(a509, a510)?
    };
    Ok(f127({
        let (_, h511) = {
            let (_, h512) = {
                let (h513, _) = f128(Fn21::F129, v2, (v0, (v1, List::<(Bytes, Bytes)>::nil())));
                h513
            };
            h512
        };
        h511
    }))
}

pub fn f127(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f79(Fn10::F130, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f128(v0: Fn21, p518: u64, p519: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool) {
    let m517 = (p518, p519);
    {
        let mut m517 = m517;
        loop {
            let step = {
                let (v1, v2) = m517;
                let m514 = v1;
                if m514 == 0 {
                    Err::<(u64, (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))), ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool)>((v2, false))
                } else {
                    let v3 = m514 - 1;
                    let m516 = {
                        let c515: Fn21 = v0.clone();
                        c515.apply(v2.clone())
                    };
                    match m516 {
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
                Ok(next) => { m517 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f129(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    f131(v0)
}

pub fn f130(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn f131(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    let v1: List<(Bytes, Bytes)> = {
        let (h520, _) = v0.clone();
        h520
    };
    let v2: List<(Bytes, Bytes)> = {
        let (h521, _) = {
            let (_, h522) = v0.clone();
            h522
        };
        h521
    };
    let v3: List<(Bytes, Bytes)> = {
        let (_, h523) = {
            let (_, h524) = v0;
            h524
        };
        h523
    };
    let m525 = v1.clone();
    match m525.uncons() {
        None => {
            let m526 = v2;
            match m526.uncons() {
                None => {
                    None::<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Bytes)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m527 = v2.clone();
            match m527.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Bytes)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f132({
                        let (h528, _) = v8.clone();
                        h528
                    }, {
                        let (h529, _) = v6.clone();
                        h529
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

pub fn f132(v0: Bytes, v1: Bytes) -> bool {
    let a534 = {
        let a530 = v0;
        let a531 = v1;
        compare_bytes(a530, a531)
    };
    let a535 = {
        let a532 = Bytes::lit(&[]);
        let a533 = Bytes::lit(&[0x01]);
        compare_bytes(a532, a533)
    };
    equal(a534, a535)
}

pub fn run(p0: Adt0) -> R<Bytes> {
    f0(p0)
}
