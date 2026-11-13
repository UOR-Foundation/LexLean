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
    C0(Bytes, List<Bytes>, List<(Bytes, Adt1)>, List<(Bytes, Adt1)>),
    C1(Bytes, List<Bytes>, List<(Bytes, Adt1)>, List<(Bytes, Adt1)>),
    C2(Bytes, Option<Bytes>, Bytes, Adt1, u64, Adt2),
    C3(Bytes, List<Bytes>, List<(Bytes, Adt1)>, List<(Bytes, Adt1)>, Option<Bytes>),
    C4(Bytes, List<Bytes>, List<(Bytes, Adt1)>, Adt1, Option<Bytes>, Adt2, List<Bytes>, bool, Option<Bytes>, Option<(Adt2, List<(Option<Bytes>, Bytes)>)>, Option<(List<Bytes>, List<Bytes>)>),
    C5(Bytes, List<Bytes>, List<(Bytes, Adt1)>, Adt2, Adt5, List<Bytes>),
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
    C20(std::rc::Rc<Adt1>),
    C21(std::rc::Rc<Adt1>, std::rc::Rc<Adt1>),
    C22(std::rc::Rc<Adt1>),
    C23(Option<Bytes>, Bytes, std::rc::Rc<Adt1>),
    C24(std::rc::Rc<Adt1>, std::rc::Rc<Adt1>),
    C25(std::rc::Rc<Adt1>, std::rc::Rc<Adt1>),
    C26(std::rc::Rc<Adt1>, std::rc::Rc<Adt1>),
    C27(std::rc::Rc<Adt1>),
    C28,
    C29(std::rc::Rc<Adt1>, std::rc::Rc<Adt1>),
}

#[derive(Clone)]
pub enum Adt2 {
    C0(Bytes),
    C1(Bytes),
    C2(Adt3, Bytes),
    C3(Bytes),
    C4(Bytes),
    C5(Adt4, std::rc::Rc<Adt2>, Adt1),
    C6(bool),
    C7,
    C8(Adt1),
    C9(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C10(Option<Bytes>, Bytes, Adt1, std::rc::Rc<Adt2>),
    C11(Option<Bytes>, Bytes, Adt1, std::rc::Rc<Adt2>),
    C12(Option<Bytes>, Bytes, Adt1, Option<Bytes>, Bytes),
    C13(std::rc::Rc<Adt2>, Bytes),
    C14(Option<Bytes>, Bytes, Adt1, std::rc::Rc<Adt2>),
    C15(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C16(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C17(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C18(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C19(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C20(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C21(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C22(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C23(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C24(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C25(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C26(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C27(std::rc::Rc<Adt2>),
    C28(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C29(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C30(Bytes, Adt1, std::rc::Rc<Adt2>),
    C31(Bytes, Adt1, std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C32(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C33(std::rc::Rc<Adt2>),
    C34(std::rc::Rc<Adt2>),
    C35(List<(Bytes, Adt1)>, List<Bytes>, std::rc::Rc<Adt2>),
    C36(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C37(Option<Bytes>, Bytes, Adt1),
    C38(Adt1, Adt1, std::rc::Rc<Adt2>),
    C39(Adt1, std::rc::Rc<Adt2>),
    C40(Adt1, std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C41(Option<Bytes>, Bytes, Adt1, std::rc::Rc<Adt2>, List<Bytes>),
    C42,
    C43(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C44(Bytes, std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C45(Option<Bytes>, Bytes, List<Bytes>, std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C46(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C47(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
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
}

#[derive(Clone)]
pub enum Adt4 {
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
pub enum Adt5 {
    C0,
    C1,
    C2(List<(Option<Bytes>, Bytes)>),
    C3(List<(Option<Bytes>, Bytes)>),
    C4(std::rc::Rc<Adt5>),
    C5(Bytes, std::rc::Rc<Adt5>),
    C6(Adt2, List<(Option<Bytes>, Bytes)>),
    C7(Bytes, List<Bytes>, std::rc::Rc<Adt5>),
    C8(Bytes, std::rc::Rc<Adt5>, std::rc::Rc<Adt5>),
    C9,
    C10(Adt6),
    C11(Bytes, Adt1, Adt2, List<(Option<Bytes>, Bytes)>),
    C12(Option<Bytes>, Bytes, Adt1, Adt2),
    C13,
    C14(std::rc::Rc<Adt5>, std::rc::Rc<Adt5>),
    C15(Bytes, List<Bytes>, std::rc::Rc<Adt5>, std::rc::Rc<Adt5>),
    C16(Adt1, Bytes, Adt2, std::rc::Rc<Adt5>, std::rc::Rc<Adt5>),
}

#[derive(Clone)]
pub enum Adt6 {
    C0(Bytes, Bytes, Option<Bytes>, Bytes, Option<Bytes>, Bytes, Bytes),
    C1(Bytes, Option<Bytes>, Bytes, Option<Bytes>, Bytes, List<(Bytes, Bytes)>),
}

#[derive(Clone)]
pub enum Adt7 {
    C0(bool),
    C1(bool, u64),
    C2(Bytes),
    C3(std::rc::Rc<Adt7>),
    C4(std::rc::Rc<Adt7>),
    C5,
    C6(std::rc::Rc<Adt7>, std::rc::Rc<Adt7>),
    C7(Bytes, std::rc::Rc<Adt7>, std::rc::Rc<Adt7>),
}

#[derive(Clone)]
pub enum Adt8 {
    C0(Bytes, bool, bool, Option<u64>),
}

#[derive(Clone)]
pub enum Fn0 {
    F29,
}

impl Fn0 {
    pub fn apply(&self, p0: List<Adt7>, p1: Bytes) -> List<Adt7> {
        match self {
            Fn0::F29 => f29(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F32,
    F42,
}

impl Fn1 {
    pub fn apply(&self, p0: List<Adt7>, p1: (Bytes, Adt1)) -> List<Adt7> {
        match self {
            Fn1::F32 => f32(p0, p1),
            Fn1::F42 => f42(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F47(Bytes),
    F83(u64, Bytes),
}

impl Fn2 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn2::F47(k0) => f47(k0.clone(), p0),
            Fn2::F83(k0, k1) => Ok(f83(*k0, k1.clone(), p0)),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F50,
    F81,
}

impl Fn3 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn3::F50 => f50(p0, p1),
            Fn3::F81 => f81(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F53,
}

impl Fn4 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, Bytes)) -> R<List<Bytes>> {
        match self {
            Fn4::F53 => f53(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F55,
}

impl Fn5 {
    pub fn apply(&self, p0: Adt7, p1: Adt7) -> Adt7 {
        match self {
            Fn5::F55 => f55(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F58,
}

impl Fn6 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn6::F58 => f58(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F60,
}

impl Fn7 {
    pub fn apply(&self, p0: List<(Bytes, Adt1)>, p1: (Bytes, Adt1)) -> List<(Bytes, Adt1)> {
        match self {
            Fn7::F60 => f60(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F64,
}

impl Fn8 {
    pub fn apply(&self, p0: List<Adt7>, p1: (Option<Bytes>, Bytes)) -> List<Adt7> {
        match self {
            Fn8::F64 => f64(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F67,
}

impl Fn9 {
    pub fn apply(&self, p0: List<Adt7>, p1: (Bytes, Bytes)) -> List<Adt7> {
        match self {
            Fn9::F67 => f67(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F73(Bytes),
}

impl Fn10 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Bytes) -> Option<Bytes> {
        match self {
            Fn10::F73(k0) => f73(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F78,
}

impl Fn11 {
    pub fn apply(&self, p0: List<Adt7>, p1: Adt7) -> List<Adt7> {
        match self {
            Fn11::F78 => f78(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F80,
}

impl Fn12 {
    pub fn apply(&self, p0: List<(Option<Bytes>, Bytes)>, p1: (Option<Bytes>, Bytes)) -> List<(Option<Bytes>, Bytes)> {
        match self {
            Fn12::F80 => f80(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F87,
}

impl Fn13 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn13::F87 => f87(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F89,
}

impl Fn14 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
        match self {
            Fn14::F89 => f89(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F94,
}

impl Fn15 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn15::F94 => f94(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F99,
}

impl Fn16 {
    pub fn apply(&self, p0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
        match self {
            Fn16::F99 => f99(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F103,
}

impl Fn17 {
    pub fn apply(&self, p0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
        match self {
            Fn17::F103 => f103(p0),
        }
    }
}

pub fn f0(v0: Adt0) -> R<Bytes> {
    f1(f2(v0))
}

pub fn f1(v0: Adt7) -> R<Bytes> {
    f3(v0)
}

pub fn f2(v0: Adt0) -> Adt7 {
    let m1 = v0;
    match m1 {
        Adt0::C0(v1, v2, v3, v4) => {
            f4(Str::lit("structure"), f5(Str::lit("name"), f6(v1), f5(Str::lit("type_parameters"), f7(v2), f5(Str::lit("parameters"), f8(v3), f5(Str::lit("fields"), f9(v4), f10())))))
        }
        Adt0::C1(v5, v6, v7, v8) => {
            f4(Str::lit("class"), f5(Str::lit("name"), f6(v5), f5(Str::lit("type_parameters"), f7(v6), f5(Str::lit("parameters"), f8(v7), f5(Str::lit("fields"), f9(v8), f10())))))
        }
        Adt0::C2(v9, v10, v11, v12, v13, v14) => {
            f4(Str::lit("instance"), f5(Str::lit("name"), f6(v9), f5(Str::lit("class"), f11(v10, v11), f5(Str::lit("arguments"), Adt7::C3(std::rc::Rc::new(f12(v12))), f5(Str::lit("priority"), Adt7::C1(false, v13), f5(Str::lit("fields"), Adt7::C3(std::rc::Rc::new(f13(v14))), f10()))))))
        }
        Adt0::C3(v15, v16, v17, v18, v19) => {
            f4(Str::lit("inductive"), f5(Str::lit("name"), f6(v15), f5(Str::lit("type_parameters"), f7(v16), f5(Str::lit("parameters"), f8(v17), f5(Str::lit("constructors"), f14(v18), f15(Str::lit("mutual"), v19, f10()))))))
        }
        Adt0::C4(v20, v21, v22, v23, v24, v25, v26, v27, v28, v29, v30) => {
            f4(Str::lit("definition"), f5(Str::lit("name"), f6(v20), f16(Str::lit("type_parameters"), v21, f5(Str::lit("parameters"), f8(v22), f5(Str::lit("result"), f12(v23), f15(Str::lit("recursive_argument"), v24, f5(Str::lit("body"), f13(v25), f16(Str::lit("axioms"), v26, if v27 {
                f5(Str::lit("executable"), Adt7::C0(true), f15(Str::lit("mutual"), v28, f17(v29, f18(v30, f10()))))
            } else {
                f15(Str::lit("mutual"), v28, f17(v29, f18(v30, f10())))
            }))))))))
        }
        Adt0::C5(v31, v32, v33, v34, v35, v36) => {
            f4(Str::lit("theorem"), f5(Str::lit("name"), f6(v31), f16(Str::lit("type_parameters"), v32, f5(Str::lit("parameters"), f8(v33), f5(Str::lit("statement"), f13(v34), f5(Str::lit("proof"), f19(v35), f16(Str::lit("axioms"), v36, f10())))))))
        }
    }
}

pub fn f3(v0: Adt7) -> R<Bytes> {
    let m2 = v0;
    match m2 {
        Adt7::C0(v1) => {
            if v1 {
                Ok(Bytes::lit(&[0x74, 0x72, 0x75, 0x65]))
            } else {
                Ok(Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]))
            }
        }
        Adt7::C1(v2, v3) => {
            f20(v2, v3)
        }
        Adt7::C2(v4) => {
            f21(v4)
        }
        Adt7::C3(r3) => {
            let v5 = (*r3).clone();
            Ok(f22(f23(v5)?))
        }
        Adt7::C4(r4) => {
            let v6 = (*r4).clone();
            f24(f25(v6)?)
        }
        Adt7::C5 => {
            Ok(Bytes::lit(&[]))
        }
        Adt7::C6(_, _) => {
            Ok(Bytes::lit(&[]))
        }
        Adt7::C7(_, _, _) => {
            Ok(Bytes::lit(&[]))
        }
    }
}

pub fn f4(v0: Str, v1: Adt7) -> Adt7 {
    Adt7::C4(std::rc::Rc::new(f5(Str::lit("kind"), f6(f26(v0)), v1)))
}

pub fn f5(v0: Str, v1: Adt7, v2: Adt7) -> Adt7 {
    Adt7::C7(f26(v0), std::rc::Rc::new(v1), std::rc::Rc::new(v2))
}

pub fn f6(v0: Bytes) -> Adt7 {
    Adt7::C2(v0)
}

pub fn f7(v0: List<Bytes>) -> Adt7 {
    Adt7::C3(std::rc::Rc::new(f27(f28(Fn0::F29, List::<Adt7>::nil(), f30(v0)))))
}

pub fn f8(v0: List<(Bytes, Adt1)>) -> Adt7 {
    Adt7::C3(std::rc::Rc::new(f27(f31(Fn1::F32, List::<Adt7>::nil(), f33(v0)))))
}

pub fn f9(v0: List<(Bytes, Adt1)>) -> Adt7 {
    f8(v0)
}

pub fn f10() -> Adt7 {
    Adt7::C5
}

pub fn f11(v0: Option<Bytes>, v1: Bytes) -> Adt7 {
    Adt7::C4(std::rc::Rc::new({
        let m5 = v0;
        match m5 {
            None => {
                f34(Str::lit("name"), Adt7::C2(v1), Adt7::C5)
            }
            Some(v2) => {
                f34(Str::lit("module"), Adt7::C2(v2), f34(Str::lit("name"), Adt7::C2(v1), Adt7::C5))
            }
        }
    }))
}

pub fn f12(v0: Adt1) -> Adt7 {
    let m6 = v0;
    match m6 {
        Adt1::C0 => {
            f35(Str::lit("type"), Adt7::C5)
        }
        Adt1::C1 => {
            f35(Str::lit("nat"), Adt7::C5)
        }
        Adt1::C2 => {
            f35(Str::lit("bool"), Adt7::C5)
        }
        Adt1::C3 => {
            f35(Str::lit("prop"), Adt7::C5)
        }
        Adt1::C4 => {
            f35(Str::lit("unit"), Adt7::C5)
        }
        Adt1::C5 => {
            f35(Str::lit("int"), Adt7::C5)
        }
        Adt1::C6 => {
            f35(Str::lit("int8"), Adt7::C5)
        }
        Adt1::C7 => {
            f35(Str::lit("int16"), Adt7::C5)
        }
        Adt1::C8 => {
            f35(Str::lit("int32"), Adt7::C5)
        }
        Adt1::C9 => {
            f35(Str::lit("int64"), Adt7::C5)
        }
        Adt1::C10 => {
            f35(Str::lit("uint8"), Adt7::C5)
        }
        Adt1::C11 => {
            f35(Str::lit("uint16"), Adt7::C5)
        }
        Adt1::C12 => {
            f35(Str::lit("uint32"), Adt7::C5)
        }
        Adt1::C13 => {
            f35(Str::lit("uint64"), Adt7::C5)
        }
        Adt1::C14 => {
            f35(Str::lit("string"), Adt7::C5)
        }
        Adt1::C15 => {
            f35(Str::lit("bytes"), Adt7::C5)
        }
        Adt1::C16 => {
            f35(Str::lit("ordering"), Adt7::C5)
        }
        Adt1::C17 => {
            f35(Str::lit("contract_violation"), Adt7::C5)
        }
        Adt1::C18 => {
            f35(Str::lit("reasoning_failure"), Adt7::C5)
        }
        Adt1::C19(v1) => {
            f35(Str::lit("parameter"), f36(Str::lit("name"), Adt7::C2(v1), Adt7::C5))
        }
        Adt1::C20(r8) => {
            let v2 = (*r8).clone();
            f35(Str::lit("option"), f36(Str::lit("value"), f12(v2), Adt7::C5))
        }
        Adt1::C21(r9, r10) => {
            let v3 = (*r9).clone();
            let v4 = (*r10).clone();
            f35(Str::lit("result"), f36(Str::lit("ok"), f12(v3), f36(Str::lit("error"), f12(v4), Adt7::C5)))
        }
        Adt1::C22(r11) => {
            let v5 = (*r11).clone();
            f35(Str::lit("list"), f36(Str::lit("element"), f12(v5), Adt7::C5))
        }
        Adt1::C23(v6, v7, r12) => {
            let v8 = (*r12).clone();
            f35(Str::lit("named"), f36(Str::lit("member"), Adt7::C4(std::rc::Rc::new({
                let m7 = v6;
                match m7 {
                    None => {
                        f36(Str::lit("name"), Adt7::C2(v7), Adt7::C5)
                    }
                    Some(v9) => {
                        f36(Str::lit("module"), Adt7::C2(v9), f36(Str::lit("name"), Adt7::C2(v7), Adt7::C5))
                    }
                }
            })), f36(Str::lit("arguments"), Adt7::C3(std::rc::Rc::new(f12(v8))), Adt7::C5)))
        }
        Adt1::C24(r13, r14) => {
            let v10 = (*r13).clone();
            let v11 = (*r14).clone();
            f35(Str::lit("product"), f36(Str::lit("left"), f12(v10), f36(Str::lit("right"), f12(v11), Adt7::C5)))
        }
        Adt1::C25(r15, r16) => {
            let v12 = (*r15).clone();
            let v13 = (*r16).clone();
            f35(Str::lit("function"), f36(Str::lit("parameters"), Adt7::C3(std::rc::Rc::new(f12(v12))), f36(Str::lit("result"), f12(v13), Adt7::C5)))
        }
        Adt1::C26(r17, r18) => {
            let v14 = (*r17).clone();
            let v15 = (*r18).clone();
            f35(Str::lit("map"), f36(Str::lit("key"), f12(v14), f36(Str::lit("value"), f12(v15), Adt7::C5)))
        }
        Adt1::C27(r19) => {
            let v16 = (*r19).clone();
            f35(Str::lit("set"), f36(Str::lit("element"), f12(v16), Adt7::C5))
        }
        Adt1::C28 => {
            Adt7::C5
        }
        Adt1::C29(r20, r21) => {
            let v17 = (*r20).clone();
            let v18 = (*r21).clone();
            Adt7::C6(std::rc::Rc::new(f12(v17)), std::rc::Rc::new(f12(v18)))
        }
    }
}

pub fn f13(v0: Adt2) -> Adt7 {
    let m22 = v0;
    match m22 {
        Adt2::C0(v1) => {
            f37(Str::lit("var"), f34(Str::lit("name"), Adt7::C2(v1), Adt7::C5))
        }
        Adt2::C1(v2) => {
            f37(Str::lit("nat"), f34(Str::lit("value"), Adt7::C2(v2), Adt7::C5))
        }
        Adt2::C2(v3, v4) => {
            f37(Str::lit("integer"), f34(Str::lit("representation"), Adt7::C2(f38(v3)), f34(Str::lit("value"), Adt7::C2(v4), Adt7::C5)))
        }
        Adt2::C3(v5) => {
            f37(Str::lit("string"), f34(Str::lit("value"), Adt7::C2(v5), Adt7::C5))
        }
        Adt2::C4(v6) => {
            f37(Str::lit("bytes"), f34(Str::lit("hex"), Adt7::C2(v6), Adt7::C5))
        }
        Adt2::C5(v7, r25, v9) => {
            let v8 = (*r25).clone();
            f37(Str::lit("primitive"), f34(Str::lit("operation"), Adt7::C2({
                let m24 = f39(v7);
                match m24 {
                    Adt8::C0(h23, _, _, _) => {
                        h23
                    }
                }
            }), f34(Str::lit("arguments"), Adt7::C3(std::rc::Rc::new(f13(v8))), f34(Str::lit("result"), f12(v9), Adt7::C5))))
        }
        Adt2::C6(v10) => {
            f37(Str::lit("bool"), f34(Str::lit("value"), Adt7::C0(v10), Adt7::C5))
        }
        Adt2::C7 => {
            f37(Str::lit("unit"), Adt7::C5)
        }
        Adt2::C8(v11) => {
            f37(Str::lit("nil"), f34(Str::lit("element"), f12(v11), Adt7::C5))
        }
        Adt2::C9(r26, r27) => {
            let v12 = (*r26).clone();
            let v13 = (*r27).clone();
            f37(Str::lit("cons"), f34(Str::lit("head"), f13(v12), f34(Str::lit("tail"), f13(v13), Adt7::C5)))
        }
        Adt2::C10(v14, v15, v16, r28) => {
            let v17 = (*r28).clone();
            f37(Str::lit("record"), f34(Str::lit("type"), f11(v14, v15), f40(Str::lit("type_arguments"), v16, f34(Str::lit("fields"), Adt7::C3(std::rc::Rc::new(f13(v17))), Adt7::C5))))
        }
        Adt2::C11(v18, v19, v20, r29) => {
            let v21 = (*r29).clone();
            f37(Str::lit("constructor"), f34(Str::lit("constructor"), f11(v18, v19), f40(Str::lit("type_arguments"), v20, f34(Str::lit("arguments"), Adt7::C3(std::rc::Rc::new(f13(v21))), Adt7::C5))))
        }
        Adt2::C12(v22, v23, v24, v25, v26) => {
            f37(Str::lit("instance_value"), f34(Str::lit("class"), f11(v22, v23), f34(Str::lit("arguments"), Adt7::C3(std::rc::Rc::new(f12(v24))), f34(Str::lit("resolved"), f11(v25, v26), Adt7::C5))))
        }
        Adt2::C13(r30, v28) => {
            let v27 = (*r30).clone();
            f37(Str::lit("project"), f34(Str::lit("value"), f13(v27), f34(Str::lit("field"), Adt7::C2(v28), Adt7::C5)))
        }
        Adt2::C14(v29, v30, v31, r31) => {
            let v32 = (*r31).clone();
            f37(Str::lit("call"), f34(Str::lit("function"), f11(v29, v30), f40(Str::lit("type_arguments"), v31, f34(Str::lit("arguments"), Adt7::C3(std::rc::Rc::new(f13(v32))), Adt7::C5))))
        }
        Adt2::C15(r32, r33, r34) => {
            let v33 = (*r32).clone();
            let v34 = (*r33).clone();
            let v35 = (*r34).clone();
            f37(Str::lit("if"), f34(Str::lit("condition"), f13(v33), f34(Str::lit("then_value"), f13(v34), f34(Str::lit("else_value"), f13(v35), Adt7::C5))))
        }
        Adt2::C16(r35, r36) => {
            let v36 = (*r35).clone();
            let v37 = (*r36).clone();
            f37(Str::lit("match"), f34(Str::lit("scrutinee"), f13(v36), f34(Str::lit("branches"), Adt7::C3(std::rc::Rc::new(f13(v37))), Adt7::C5)))
        }
        Adt2::C17(r37, r38) => {
            let v38 = (*r37).clone();
            let v39 = (*r38).clone();
            f37(Str::lit("eq"), f34(Str::lit("left"), f13(v38), f34(Str::lit("right"), f13(v39), Adt7::C5)))
        }
        Adt2::C18(r39, r40) => {
            let v40 = (*r39).clone();
            let v41 = (*r40).clone();
            f37(Str::lit("le"), f34(Str::lit("left"), f13(v40), f34(Str::lit("right"), f13(v41), Adt7::C5)))
        }
        Adt2::C19(r41, r42) => {
            let v42 = (*r41).clone();
            let v43 = (*r42).clone();
            f37(Str::lit("lt"), f34(Str::lit("left"), f13(v42), f34(Str::lit("right"), f13(v43), Adt7::C5)))
        }
        Adt2::C20(r43, r44) => {
            let v44 = (*r43).clone();
            let v45 = (*r44).clone();
            f37(Str::lit("add"), f34(Str::lit("left"), f13(v44), f34(Str::lit("right"), f13(v45), Adt7::C5)))
        }
        Adt2::C21(r45, r46) => {
            let v46 = (*r45).clone();
            let v47 = (*r46).clone();
            f37(Str::lit("beq"), f34(Str::lit("left"), f13(v46), f34(Str::lit("right"), f13(v47), Adt7::C5)))
        }
        Adt2::C22(r47, r48) => {
            let v48 = (*r47).clone();
            let v49 = (*r48).clone();
            f37(Str::lit("ble"), f34(Str::lit("left"), f13(v48), f34(Str::lit("right"), f13(v49), Adt7::C5)))
        }
        Adt2::C23(r49, r50) => {
            let v50 = (*r49).clone();
            let v51 = (*r50).clone();
            f37(Str::lit("blt"), f34(Str::lit("left"), f13(v50), f34(Str::lit("right"), f13(v51), Adt7::C5)))
        }
        Adt2::C24(r51, r52) => {
            let v52 = (*r51).clone();
            let v53 = (*r52).clone();
            f37(Str::lit("and"), f34(Str::lit("left"), f13(v52), f34(Str::lit("right"), f13(v53), Adt7::C5)))
        }
        Adt2::C25(r53, r54) => {
            let v54 = (*r53).clone();
            let v55 = (*r54).clone();
            f37(Str::lit("prop_and"), f34(Str::lit("left"), f13(v54), f34(Str::lit("right"), f13(v55), Adt7::C5)))
        }
        Adt2::C26(r55, r56) => {
            let v56 = (*r55).clone();
            let v57 = (*r56).clone();
            f37(Str::lit("or"), f34(Str::lit("left"), f13(v56), f34(Str::lit("right"), f13(v57), Adt7::C5)))
        }
        Adt2::C27(r57) => {
            let v58 = (*r57).clone();
            f37(Str::lit("not"), f34(Str::lit("value"), f13(v58), Adt7::C5))
        }
        Adt2::C28(r58, r59) => {
            let v59 = (*r58).clone();
            let v60 = (*r59).clone();
            f37(Str::lit("implies"), f34(Str::lit("premise"), f13(v59), f34(Str::lit("conclusion"), f13(v60), Adt7::C5)))
        }
        Adt2::C29(r60, r61) => {
            let v61 = (*r60).clone();
            let v62 = (*r61).clone();
            f37(Str::lit("iff"), f34(Str::lit("left"), f13(v61), f34(Str::lit("right"), f13(v62), Adt7::C5)))
        }
        Adt2::C30(v63, v64, r62) => {
            let v65 = (*r62).clone();
            f37(Str::lit("forall"), f34(Str::lit("binder"), f41(v63, v64), f34(Str::lit("body"), f13(v65), Adt7::C5)))
        }
        Adt2::C31(v66, v67, r63, r64) => {
            let v68 = (*r63).clone();
            let v69 = (*r64).clone();
            f37(Str::lit("let"), f34(Str::lit("binder"), f41(v66, v67), f34(Str::lit("value"), f13(v68), f34(Str::lit("body"), f13(v69), Adt7::C5))))
        }
        Adt2::C32(r65, r66) => {
            let v70 = (*r65).clone();
            let v71 = (*r66).clone();
            f37(Str::lit("pair"), f34(Str::lit("left"), f13(v70), f34(Str::lit("right"), f13(v71), Adt7::C5)))
        }
        Adt2::C33(r67) => {
            let v72 = (*r67).clone();
            f37(Str::lit("first"), f34(Str::lit("value"), f13(v72), Adt7::C5))
        }
        Adt2::C34(r68) => {
            let v73 = (*r68).clone();
            f37(Str::lit("second"), f34(Str::lit("value"), f13(v73), Adt7::C5))
        }
        Adt2::C35(v74, v75, r69) => {
            let v76 = (*r69).clone();
            f37(Str::lit("lambda"), f34(Str::lit("parameters"), f8(v74), f34(Str::lit("captures"), f7(v75), f34(Str::lit("body"), f13(v76), Adt7::C5))))
        }
        Adt2::C36(r70, r71) => {
            let v77 = (*r70).clone();
            let v78 = (*r71).clone();
            f37(Str::lit("apply"), f34(Str::lit("function"), f13(v77), f34(Str::lit("arguments"), Adt7::C3(std::rc::Rc::new(f13(v78))), Adt7::C5)))
        }
        Adt2::C37(v79, v80, v81) => {
            f37(Str::lit("function_ref"), f34(Str::lit("function"), f11(v79, v80), f40(Str::lit("type_arguments"), v81, Adt7::C5)))
        }
        Adt2::C38(v82, v83, r72) => {
            let v84 = (*r72).clone();
            f37(Str::lit("map_literal"), f34(Str::lit("key"), f12(v82), f34(Str::lit("value"), f12(v83), f34(Str::lit("entries"), Adt7::C3(std::rc::Rc::new(f13(v84))), Adt7::C5))))
        }
        Adt2::C39(v85, r73) => {
            let v86 = (*r73).clone();
            f37(Str::lit("set_literal"), f34(Str::lit("element"), f12(v85), f34(Str::lit("elements"), Adt7::C3(std::rc::Rc::new(f13(v86))), Adt7::C5)))
        }
        Adt2::C40(v87, r74, r75) => {
            let v88 = (*r74).clone();
            let v89 = (*r75).clone();
            f37(Str::lit("graph_literal"), f34(Str::lit("node"), f12(v87), f34(Str::lit("nodes"), Adt7::C3(std::rc::Rc::new(f13(v88))), f34(Str::lit("edges"), Adt7::C3(std::rc::Rc::new(f13(v89))), Adt7::C5))))
        }
        Adt2::C41(v90, v91, v92, r76, v94) => {
            let v93 = (*r76).clone();
            f37(Str::lit("checked_apply"), f34(Str::lit("model"), f11(v90, v91), f40(Str::lit("type_arguments"), v92, f34(Str::lit("arguments"), Adt7::C3(std::rc::Rc::new(f13(v93))), f34(Str::lit("checks"), f7(v94), Adt7::C5)))))
        }
        Adt2::C42 => {
            Adt7::C5
        }
        Adt2::C43(r77, r78) => {
            let v95 = (*r77).clone();
            let v96 = (*r78).clone();
            Adt7::C6(std::rc::Rc::new(f13(v95)), std::rc::Rc::new(f13(v96)))
        }
        Adt2::C44(v97, r79, r80) => {
            let v98 = (*r79).clone();
            let v99 = (*r80).clone();
            Adt7::C6(std::rc::Rc::new(Adt7::C4(std::rc::Rc::new(f34(Str::lit("field"), Adt7::C2(v97), f34(Str::lit("value"), f13(v98), Adt7::C5))))), std::rc::Rc::new(f13(v99)))
        }
        Adt2::C45(v100, v101, v102, r81, r82) => {
            let v103 = (*r81).clone();
            let v104 = (*r82).clone();
            Adt7::C6(std::rc::Rc::new(Adt7::C4(std::rc::Rc::new(f34(Str::lit("constructor"), f11(v100, v101), f34(Str::lit("binders"), f7(v102), f34(Str::lit("body"), f13(v103), Adt7::C5)))))), std::rc::Rc::new(f13(v104)))
        }
        Adt2::C46(r83, r84, r85) => {
            let v105 = (*r83).clone();
            let v106 = (*r84).clone();
            let v107 = (*r85).clone();
            Adt7::C6(std::rc::Rc::new(Adt7::C4(std::rc::Rc::new(f34(Str::lit("key"), f13(v105), f34(Str::lit("value"), f13(v106), Adt7::C5))))), std::rc::Rc::new(f13(v107)))
        }
        Adt2::C47(r86, r87, r88) => {
            let v108 = (*r86).clone();
            let v109 = (*r87).clone();
            let v110 = (*r88).clone();
            Adt7::C6(std::rc::Rc::new(Adt7::C4(std::rc::Rc::new(f34(Str::lit("source"), f13(v108), f34(Str::lit("target"), f13(v109), Adt7::C5))))), std::rc::Rc::new(f13(v110)))
        }
    }
}

pub fn f14(v0: List<(Bytes, Adt1)>) -> Adt7 {
    Adt7::C3(std::rc::Rc::new(f27(f31(Fn1::F42, List::<Adt7>::nil(), f33(v0)))))
}

pub fn f15(v0: Str, v1: Option<Bytes>, v2: Adt7) -> Adt7 {
    let m89 = v1;
    match m89 {
        None => {
            v2
        }
        Some(v3) => {
            f5(v0, f6(v3), v2)
        }
    }
}

pub fn f16(v0: Str, v1: List<Bytes>, v2: Adt7) -> Adt7 {
    let m90 = v1.clone();
    match m90.uncons() {
        None => {
            v2
        }
        Some((_, _)) => {
            f5(v0, f7(v1), v2)
        }
    }
}

pub fn f17(v0: Option<(Adt2, List<(Option<Bytes>, Bytes)>)>, v1: Adt7) -> Adt7 {
    let m91 = v0;
    match m91 {
        None => {
            v1
        }
        Some(v2) => {
            f5(Str::lit("termination"), Adt7::C4(std::rc::Rc::new(f5(Str::lit("measure"), f13({
                let (h92, _) = v2;
                h92
            }), f5(Str::lit("evidence"), f43({
                let (_, h93) = v2;
                h93
            }), f10())))), v1)
        }
    }
}

pub fn f18(v0: Option<(List<Bytes>, List<Bytes>)>, v1: Adt7) -> Adt7 {
    let m94 = v0;
    match m94 {
        None => {
            v1
        }
        Some(v2) => {
            f5(Str::lit("production"), Adt7::C4(std::rc::Rc::new(f5(Str::lit("targets"), f7({
                let (h95, _) = v2;
                h95
            }), f5(Str::lit("effects"), f7({
                let (_, h96) = v2;
                h96
            }), f10())))), v1)
        }
    }
}

pub fn f19(v0: Adt5) -> Adt7 {
    let m97 = v0;
    match m97 {
        Adt5::C0 => {
            f37(Str::lit("reflexivity"), Adt7::C5)
        }
        Adt5::C1 => {
            f37(Str::lit("decide"), Adt7::C5)
        }
        Adt5::C2(v1) => {
            f37(Str::lit("linear_arithmetic"), {
                let m98 = v1.clone();
                match m98.uncons() {
                    None => {
                        Adt7::C5
                    }
                    Some((_, _)) => {
                        f34(Str::lit("definitions"), f43(v1), Adt7::C5)
                    }
                }
            })
        }
        Adt5::C3(v4) => {
            f37(Str::lit("simplify"), f34(Str::lit("definitions"), f43(v4), Adt7::C5))
        }
        Adt5::C4(r101) => {
            let v5 = (*r101).clone();
            f37(Str::lit("constructor"), f34(Str::lit("branches"), Adt7::C3(std::rc::Rc::new(f19(v5))), Adt7::C5))
        }
        Adt5::C5(v6, r102) => {
            let v7 = (*r102).clone();
            f37(Str::lit("cases"), f34(Str::lit("scrutinee"), Adt7::C2(v6), f34(Str::lit("branches"), Adt7::C3(std::rc::Rc::new(f19(v7))), Adt7::C5)))
        }
        Adt5::C6(v8, v9) => {
            f37(Str::lit("term_cases"), f34(Str::lit("scrutinee"), f13(v8), f34(Str::lit("definitions"), f43(v9), Adt7::C5)))
        }
        Adt5::C7(v10, v11, r103) => {
            let v12 = (*r103).clone();
            f37(Str::lit("induction"), f34(Str::lit("scrutinee"), Adt7::C2(v10), {
                let m99 = v11.clone();
                match m99.uncons() {
                    None => {
                        f34(Str::lit("branches"), Adt7::C3(std::rc::Rc::new(f19(v12))), Adt7::C5)
                    }
                    Some((_, _)) => {
                        f34(Str::lit("generalizing"), f7(v11), f34(Str::lit("branches"), Adt7::C3(std::rc::Rc::new(f19(v12))), Adt7::C5))
                    }
                }
            }))
        }
        Adt5::C8(v15, r104, r105) => {
            let v16 = (*r104).clone();
            let v17 = (*r105).clone();
            f37(Str::lit("nested_induction"), f34(Str::lit("scrutinee"), Adt7::C2(v15), f34(Str::lit("branches"), Adt7::C3(std::rc::Rc::new(f19(v16))), f34(Str::lit("members"), Adt7::C3(std::rc::Rc::new(f19(v17))), Adt7::C5))))
        }
        Adt5::C9 => {
            f37(Str::lit("congruence"), Adt7::C5)
        }
        Adt5::C10(v18) => {
            f37(Str::lit("boolean_reflection"), f34(Str::lit("reflection"), f44(v18), Adt7::C5))
        }
        Adt5::C11(v19, v20, v21, v22) => {
            f37(Str::lit("law"), f34(Str::lit("name"), Adt7::C2(v19), f40(Str::lit("type_arguments"), v20, f34(Str::lit("arguments"), Adt7::C3(std::rc::Rc::new(f13(v21))), {
                let m100 = v22.clone();
                match m100.uncons() {
                    None => {
                        Adt7::C5
                    }
                    Some((_, _)) => {
                        f34(Str::lit("premises"), f43(v22), Adt7::C5)
                    }
                }
            }))))
        }
        Adt5::C12(v25, v26, v27, v28) => {
            f37(Str::lit("apply"), f34(Str::lit("theorem"), f11(v25, v26), f40(Str::lit("type_arguments"), v27, f34(Str::lit("arguments"), Adt7::C3(std::rc::Rc::new(f13(v28))), Adt7::C5))))
        }
        Adt5::C13 => {
            Adt7::C5
        }
        Adt5::C14(r106, r107) => {
            let v29 = (*r106).clone();
            let v30 = (*r107).clone();
            Adt7::C6(std::rc::Rc::new(f19(v29)), std::rc::Rc::new(f19(v30)))
        }
        Adt5::C15(v31, v32, r108, r109) => {
            let v33 = (*r108).clone();
            let v34 = (*r109).clone();
            Adt7::C6(std::rc::Rc::new(Adt7::C4(std::rc::Rc::new(f34(Str::lit("constructor"), Adt7::C2(v31), f34(Str::lit("binders"), f7(v32), f34(Str::lit("proof"), f19(v33), Adt7::C5)))))), std::rc::Rc::new(f19(v34)))
        }
        Adt5::C16(v35, v36, v37, r110, r111) => {
            let v38 = (*r110).clone();
            let v39 = (*r111).clone();
            Adt7::C6(std::rc::Rc::new(Adt7::C4(std::rc::Rc::new(f34(Str::lit("type"), f12(v35), f34(Str::lit("binder"), Adt7::C2(v36), f34(Str::lit("statement"), f13(v37), f34(Str::lit("branches"), Adt7::C3(std::rc::Rc::new(f19(v38))), Adt7::C5))))))), std::rc::Rc::new(f19(v39)))
        }
    }
}

pub fn f20(v0: bool, v1: u64) -> R<Bytes> {
    if if v0 {
        let a112 = 0u64;
        let a113 = v1;
        nat_lt(a112, a113)
    } else {
        false
    } {
        let a114 = Bytes::lit(&[0x2d]);
        let a115 = f45(v1)?;
        Ok(append_bytes(a114, a115))
    } else {
        f45(v1)
    }
}

pub fn f21(v0: Bytes) -> R<Bytes> {
    let a121 = {
        let a119 = Bytes::lit(&[0x22]);
        let a120 = {
            let (_, h116) = {
                let (h117, _) = f46(Fn2::F47(v0.clone()), {
                    let a118 = v0;
                    length_bytes(a118)
                }, (0u64, Bytes::lit(&[])))?;
                h117
            };
            h116
        };
        append_bytes(a119, a120)
    };
    let a122 = Bytes::lit(&[0x22]);
    Ok(append_bytes(a121, a122))
}

pub fn f22(v0: List<Bytes>) -> Bytes {
    let a125 = {
        let a123 = Bytes::lit(&[0x5b]);
        let a124 = f48(v0, Bytes::lit(&[0x2c]));
        append_bytes(a123, a124)
    };
    let a126 = Bytes::lit(&[0x5d]);
    append_bytes(a125, a126)
}

pub fn f23(v0: Adt7) -> R<List<Bytes>> {
    let m127 = v0;
    match m127 {
        Adt7::C0(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt7::C1(_, _) => {
            Ok(List::<Bytes>::nil())
        }
        Adt7::C2(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt7::C3(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt7::C4(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt7::C5 => {
            Ok(List::<Bytes>::nil())
        }
        Adt7::C6(r128, r129) => {
            let v7 = (*r128).clone();
            let v8 = (*r129).clone();
            Ok(List::cons(f3(v7)?, f23(v8)?))
        }
        Adt7::C7(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f24(v0: List<(Bytes, Bytes)>) -> R<Bytes> {
    let v1: List<(Bytes, Bytes)> = f49(Fn3::F50, List::<(Bytes, Bytes)>::nil(), f51(v0)?);
    let v2: List<Bytes> = f52(Fn4::F53, List::<Bytes>::nil(), v1)?;
    let a132 = {
        let a130 = Bytes::lit(&[0x7b]);
        let a131 = f48(v2, Bytes::lit(&[0x2c]));
        append_bytes(a130, a131)
    };
    let a133 = Bytes::lit(&[0x7d]);
    Ok(append_bytes(a132, a133))
}

pub fn f25(v0: Adt7) -> R<List<(Bytes, Bytes)>> {
    let m134 = v0;
    match m134 {
        Adt7::C0(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt7::C1(_, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt7::C2(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt7::C3(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt7::C4(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt7::C5 => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt7::C6(_, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt7::C7(v9, r135, r136) => {
            let v10 = (*r135).clone();
            let v11 = (*r136).clone();
            Ok(List::cons((v9, f3(v10)?), f25(v11)?))
        }
    }
}

pub fn f26(v0: Str) -> Bytes {
    let a137 = v0;
    utf8_encode(a137)
}

pub fn f27(v0: List<Adt7>) -> Adt7 {
    f54(Fn5::F55, Adt7::C5, f56(v0))
}

pub fn f28(v0: Fn0, p141: List<Adt7>, p142: List<Bytes>) -> List<Adt7> {
    let m140 = (p141, p142);
    {
        let mut m140 = m140;
        loop {
            let step = {
                let (v1, v2) = m140;
                let m138 = v2;
                match m138.uncons() {
                    None => {
                        Err::<(List<Adt7>, List<Bytes>), List<Adt7>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt7>, List<Bytes>), List<Adt7>>(({
                            let c139: Fn0 = v0.clone();
                            c139.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m140 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f29(v0: List<Adt7>, v1: Bytes) -> List<Adt7> {
    List::cons(Adt7::C2(v1), v0)
}

pub fn f30(v0: List<Bytes>) -> List<Bytes> {
    f57(Fn6::F58, List::<Bytes>::nil(), v0)
}

pub fn f31(v0: Fn1, p146: List<Adt7>, p147: List<(Bytes, Adt1)>) -> List<Adt7> {
    let m145 = (p146, p147);
    {
        let mut m145 = m145;
        loop {
            let step = {
                let (v1, v2) = m145;
                let m143 = v2;
                match m143.uncons() {
                    None => {
                        Err::<(List<Adt7>, List<(Bytes, Adt1)>), List<Adt7>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt7>, List<(Bytes, Adt1)>), List<Adt7>>(({
                            let c144: Fn1 = v0.clone();
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

pub fn f32(v0: List<Adt7>, v1: (Bytes, Adt1)) -> List<Adt7> {
    List::cons(f41({
        let (h148, _) = v1;
        h148
    }, {
        let (_, h149) = v1;
        h149
    }), v0)
}

pub fn f33(v0: List<(Bytes, Adt1)>) -> List<(Bytes, Adt1)> {
    f59(Fn7::F60, List::<(Bytes, Adt1)>::nil(), v0)
}

pub fn f34(v0: Str, v1: Adt7, v2: Adt7) -> Adt7 {
    Adt7::C7(f61(v0), std::rc::Rc::new(v1), std::rc::Rc::new(v2))
}

pub fn f35(v0: Str, v1: Adt7) -> Adt7 {
    Adt7::C4(std::rc::Rc::new(f36(Str::lit("kind"), Adt7::C2(f62(v0)), v1)))
}

pub fn f36(v0: Str, v1: Adt7, v2: Adt7) -> Adt7 {
    Adt7::C7(f62(v0), std::rc::Rc::new(v1), std::rc::Rc::new(v2))
}

pub fn f37(v0: Str, v1: Adt7) -> Adt7 {
    Adt7::C4(std::rc::Rc::new(f34(Str::lit("kind"), Adt7::C2(f61(v0)), v1)))
}

pub fn f38(v0: Adt3) -> Bytes {
    let m150 = v0;
    match m150 {
        Adt3::C0 => {
            f62(Str::lit("int"))
        }
        Adt3::C1 => {
            f62(Str::lit("int8"))
        }
        Adt3::C2 => {
            f62(Str::lit("int16"))
        }
        Adt3::C3 => {
            f62(Str::lit("int32"))
        }
        Adt3::C4 => {
            f62(Str::lit("int64"))
        }
        Adt3::C5 => {
            f62(Str::lit("uint8"))
        }
        Adt3::C6 => {
            f62(Str::lit("uint16"))
        }
        Adt3::C7 => {
            f62(Str::lit("uint32"))
        }
        Adt3::C8 => {
            f62(Str::lit("uint64"))
        }
    }
}

pub fn f39(v0: Adt4) -> Adt8 {
    let m151 = v0;
    match m151 {
        Adt4::C0 => {
            Adt8::C0(f62(Str::lit("subtract")), false, false, None::<u64>)
        }
        Adt4::C1 => {
            Adt8::C0(f62(Str::lit("multiply")), false, false, None::<u64>)
        }
        Adt4::C2 => {
            Adt8::C0(f62(Str::lit("quotient")), false, false, None::<u64>)
        }
        Adt4::C3 => {
            Adt8::C0(f62(Str::lit("remainder")), false, false, None::<u64>)
        }
        Adt4::C4 => {
            Adt8::C0(f62(Str::lit("negate")), false, false, None::<u64>)
        }
        Adt4::C5 => {
            Adt8::C0(f62(Str::lit("checked_convert")), false, false, None::<u64>)
        }
        Adt4::C6 => {
            Adt8::C0(f62(Str::lit("checked_add")), false, false, None::<u64>)
        }
        Adt4::C7 => {
            Adt8::C0(f62(Str::lit("checked_subtract")), false, false, None::<u64>)
        }
        Adt4::C8 => {
            Adt8::C0(f62(Str::lit("checked_multiply")), false, false, None::<u64>)
        }
        Adt4::C9 => {
            Adt8::C0(f62(Str::lit("checked_negate")), false, false, None::<u64>)
        }
        Adt4::C10 => {
            Adt8::C0(f62(Str::lit("checked_quotient")), false, false, None::<u64>)
        }
        Adt4::C11 => {
            Adt8::C0(f62(Str::lit("bit_and")), false, false, None::<u64>)
        }
        Adt4::C12 => {
            Adt8::C0(f62(Str::lit("bit_or")), false, false, None::<u64>)
        }
        Adt4::C13 => {
            Adt8::C0(f62(Str::lit("bit_xor")), false, false, None::<u64>)
        }
        Adt4::C14 => {
            Adt8::C0(f62(Str::lit("bit_not")), false, false, None::<u64>)
        }
        Adt4::C15 => {
            Adt8::C0(f62(Str::lit("shift_left")), false, false, None::<u64>)
        }
        Adt4::C16 => {
            Adt8::C0(f62(Str::lit("shift_right")), false, false, None::<u64>)
        }
        Adt4::C17 => {
            Adt8::C0(f62(Str::lit("append")), false, false, None::<u64>)
        }
        Adt4::C18 => {
            Adt8::C0(f62(Str::lit("length")), false, false, None::<u64>)
        }
        Adt4::C19 => {
            Adt8::C0(f62(Str::lit("index")), false, false, None::<u64>)
        }
        Adt4::C20 => {
            Adt8::C0(f62(Str::lit("slice")), false, false, None::<u64>)
        }
        Adt4::C21 => {
            Adt8::C0(f62(Str::lit("utf8_encode")), false, false, None::<u64>)
        }
        Adt4::C22 => {
            Adt8::C0(f62(Str::lit("utf8_decode")), false, false, None::<u64>)
        }
        Adt4::C23 => {
            Adt8::C0(f62(Str::lit("compare_bytes")), false, false, None::<u64>)
        }
        Adt4::C24 => {
            Adt8::C0(f62(Str::lit("equal")), false, false, None::<u64>)
        }
        Adt4::C25 => {
            Adt8::C0(f62(Str::lit("split_exact")), false, false, None::<u64>)
        }
        Adt4::C26 => {
            Adt8::C0(f62(Str::lit("join")), false, false, None::<u64>)
        }
        Adt4::C27 => {
            Adt8::C0(f62(Str::lit("parse_decimal")), false, false, None::<u64>)
        }
        Adt4::C28 => {
            Adt8::C0(f62(Str::lit("format_decimal")), false, false, None::<u64>)
        }
        Adt4::C29 => {
            Adt8::C0(f62(Str::lit("map_insert")), true, false, None::<u64>)
        }
        Adt4::C30 => {
            Adt8::C0(f62(Str::lit("map_remove")), true, false, None::<u64>)
        }
        Adt4::C31 => {
            Adt8::C0(f62(Str::lit("map_lookup")), true, false, None::<u64>)
        }
        Adt4::C32 => {
            Adt8::C0(f62(Str::lit("map_contains")), true, false, None::<u64>)
        }
        Adt4::C33 => {
            Adt8::C0(f62(Str::lit("map_size")), true, false, None::<u64>)
        }
        Adt4::C34 => {
            Adt8::C0(f62(Str::lit("map_keys")), true, false, None::<u64>)
        }
        Adt4::C35 => {
            Adt8::C0(f62(Str::lit("map_values")), true, false, None::<u64>)
        }
        Adt4::C36 => {
            Adt8::C0(f62(Str::lit("map_entries")), true, false, None::<u64>)
        }
        Adt4::C37 => {
            Adt8::C0(f62(Str::lit("map_fold")), true, false, Some(0u64))
        }
        Adt4::C38 => {
            Adt8::C0(f62(Str::lit("set_insert")), true, false, None::<u64>)
        }
        Adt4::C39 => {
            Adt8::C0(f62(Str::lit("set_remove")), true, false, None::<u64>)
        }
        Adt4::C40 => {
            Adt8::C0(f62(Str::lit("set_contains")), true, false, None::<u64>)
        }
        Adt4::C41 => {
            Adt8::C0(f62(Str::lit("set_size")), true, false, None::<u64>)
        }
        Adt4::C42 => {
            Adt8::C0(f62(Str::lit("set_elements")), true, false, None::<u64>)
        }
        Adt4::C43 => {
            Adt8::C0(f62(Str::lit("set_union")), true, false, None::<u64>)
        }
        Adt4::C44 => {
            Adt8::C0(f62(Str::lit("set_intersection")), true, false, None::<u64>)
        }
        Adt4::C45 => {
            Adt8::C0(f62(Str::lit("set_difference")), true, false, None::<u64>)
        }
        Adt4::C46 => {
            Adt8::C0(f62(Str::lit("set_fold")), true, false, Some(0u64))
        }
        Adt4::C47 => {
            Adt8::C0(f62(Str::lit("list_fold")), true, false, Some(0u64))
        }
        Adt4::C48 => {
            Adt8::C0(f62(Str::lit("iterate")), true, false, Some(0u64))
        }
        Adt4::C49 => {
            Adt8::C0(f62(Str::lit("iterate_until")), true, false, Some(0u64))
        }
        Adt4::C50 => {
            Adt8::C0(f62(Str::lit("graph_successors")), true, false, None::<u64>)
        }
        Adt4::C51 => {
            Adt8::C0(f62(Str::lit("graph_reachable")), true, false, None::<u64>)
        }
        Adt4::C52 => {
            Adt8::C0(f62(Str::lit("graph_topological")), true, false, None::<u64>)
        }
        Adt4::C53 => {
            Adt8::C0(f62(Str::lit("less_than")), true, false, None::<u64>)
        }
        Adt4::C54 => {
            Adt8::C0(f62(Str::lit("to_int")), true, true, None::<u64>)
        }
        Adt4::C55 => {
            Adt8::C0(f62(Str::lit("to_nat")), true, true, None::<u64>)
        }
    }
}

pub fn f40(v0: Str, v1: Adt1, v2: Adt7) -> Adt7 {
    let m152 = v1.clone();
    match m152 {
        Adt1::C0 => {
            v2
        }
        Adt1::C1 => {
            v2
        }
        Adt1::C2 => {
            v2
        }
        Adt1::C3 => {
            v2
        }
        Adt1::C4 => {
            v2
        }
        Adt1::C5 => {
            v2
        }
        Adt1::C6 => {
            v2
        }
        Adt1::C7 => {
            v2
        }
        Adt1::C8 => {
            v2
        }
        Adt1::C9 => {
            v2
        }
        Adt1::C10 => {
            v2
        }
        Adt1::C11 => {
            v2
        }
        Adt1::C12 => {
            v2
        }
        Adt1::C13 => {
            v2
        }
        Adt1::C14 => {
            v2
        }
        Adt1::C15 => {
            v2
        }
        Adt1::C16 => {
            v2
        }
        Adt1::C17 => {
            v2
        }
        Adt1::C18 => {
            v2
        }
        Adt1::C19(_) => {
            v2
        }
        Adt1::C20(_) => {
            v2
        }
        Adt1::C21(_, _) => {
            v2
        }
        Adt1::C22(_) => {
            v2
        }
        Adt1::C23(_, _, _) => {
            v2
        }
        Adt1::C24(_, _) => {
            v2
        }
        Adt1::C25(_, _) => {
            v2
        }
        Adt1::C26(_, _) => {
            v2
        }
        Adt1::C27(_) => {
            v2
        }
        Adt1::C28 => {
            v2
        }
        Adt1::C29(_, _) => {
            f34(v0, Adt7::C3(std::rc::Rc::new(f12(v1))), v2)
        }
    }
}

pub fn f41(v0: Bytes, v1: Adt1) -> Adt7 {
    Adt7::C4(std::rc::Rc::new(f34(Str::lit("name"), Adt7::C2(v0), f34(Str::lit("type"), f12(v1), Adt7::C5))))
}

pub fn f42(v0: List<Adt7>, v1: (Bytes, Adt1)) -> List<Adt7> {
    List::cons(Adt7::C4(std::rc::Rc::new(f5(Str::lit("name"), f6({
        let (h153, _) = v1;
        h153
    }), f5(Str::lit("fields"), Adt7::C3(std::rc::Rc::new(f12({
        let (_, h154) = v1;
        h154
    }))), f10())))), v0)
}

pub fn f43(v0: List<(Option<Bytes>, Bytes)>) -> Adt7 {
    Adt7::C3(std::rc::Rc::new(f27(f63(Fn8::F64, List::<Adt7>::nil(), f65(v0)))))
}

pub fn f44(v0: Adt6) -> Adt7 {
    let m155 = v0;
    match m155 {
        Adt6::C0(v1, v2, v3, v4, v5, v6, v7) => {
            f37(Str::lit("list"), f34(Str::lit("parameter"), Adt7::C2(v1), f34(Str::lit("values"), Adt7::C2(v2), f34(Str::lit("boolean_definition"), f11(v3, v4), f34(Str::lit("proposition_definition"), f11(v5, v6), f34(Str::lit("comparison"), Adt7::C2(v7), Adt7::C5))))))
        }
        Adt6::C1(v8, v9, v10, v11, v12, v13) => {
            f37(Str::lit("record"), f34(Str::lit("record"), Adt7::C2(v8), f34(Str::lit("boolean_definition"), f11(v9, v10), f34(Str::lit("proposition_definition"), f11(v11, v12), f34(Str::lit("fields"), Adt7::C3(std::rc::Rc::new(f27(f66(Fn9::F67, List::<Adt7>::nil(), f68(v13))))), Adt7::C5)))))
        }
    }
}

pub fn f45(v0: u64) -> R<Bytes> {
    f69(f70(), 10u64, v0)
}

pub fn f46(v0: Fn2, p160: u64, p161: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m159 = (p160, p161);
    {
        let mut m159 = m159;
        loop {
            let step = {
                let (v1, v2) = m159;
                let m156 = v1;
                if m156 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m156 - 1;
                    let m158 = {
                        let c157: Fn2 = v0.clone();
                        c157.apply(v2.clone())?
                    };
                    match m158 {
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
                Ok(next) => { m159 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f47(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f71(v0, v1)
}

pub fn f48(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m162 = f72(Fn10::F73(v1), None::<Bytes>, v0);
    match m162 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f49(v0: Fn3, p166: List<(Bytes, Bytes)>, p167: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m165 = (p166, p167);
    {
        let mut m165 = m165;
        loop {
            let step = {
                let (v1, v2) = m165;
                let m163 = v2;
                match m163.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c164: Fn3 = v0.clone();
                            c164.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m165 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f50(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    f74(v0, v1)
}

pub fn f51(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    f75(v0)
}

pub fn f52(v0: Fn4, p171: List<Bytes>, p172: List<(Bytes, Bytes)>) -> R<List<Bytes>> {
    let m170 = (p171, p172);
    {
        let mut m170 = m170;
        loop {
            let step = {
                let (v1, v2) = m170;
                let m168 = v2;
                match m168.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(({
                            let c169: Fn4 = v0.clone();
                            c169.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m170 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f53(v0: List<Bytes>, v1: (Bytes, Bytes)) -> R<List<Bytes>> {
    Ok(List::cons(f76(v1)?, v0))
}

pub fn f54(v0: Fn5, p176: Adt7, p177: List<Adt7>) -> Adt7 {
    let m175 = (p176, p177);
    {
        let mut m175 = m175;
        loop {
            let step = {
                let (v1, v2) = m175;
                let m173 = v2;
                match m173.uncons() {
                    None => {
                        Err::<(Adt7, List<Adt7>), Adt7>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt7, List<Adt7>), Adt7>(({
                            let c174: Fn5 = v0.clone();
                            c174.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m175 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f55(v0: Adt7, v1: Adt7) -> Adt7 {
    Adt7::C6(std::rc::Rc::new(v1), std::rc::Rc::new(v0))
}

pub fn f56(v0: List<Adt7>) -> List<Adt7> {
    f77(Fn11::F78, List::<Adt7>::nil(), v0)
}

pub fn f57(v0: Fn6, p181: List<Bytes>, p182: List<Bytes>) -> List<Bytes> {
    let m180 = (p181, p182);
    {
        let mut m180 = m180;
        loop {
            let step = {
                let (v1, v2) = m180;
                let m178 = v2;
                match m178.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c179: Fn6 = v0.clone();
                            c179.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m180 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f58(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f59(v0: Fn7, p186: List<(Bytes, Adt1)>, p187: List<(Bytes, Adt1)>) -> List<(Bytes, Adt1)> {
    let m185 = (p186, p187);
    {
        let mut m185 = m185;
        loop {
            let step = {
                let (v1, v2) = m185;
                let m183 = v2;
                match m183.uncons() {
                    None => {
                        Err::<(List<(Bytes, Adt1)>, List<(Bytes, Adt1)>), List<(Bytes, Adt1)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Adt1)>, List<(Bytes, Adt1)>), List<(Bytes, Adt1)>>(({
                            let c184: Fn7 = v0.clone();
                            c184.apply(v1, v3)
                        }, v4))
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

pub fn f60(v0: List<(Bytes, Adt1)>, v1: (Bytes, Adt1)) -> List<(Bytes, Adt1)> {
    List::cons(v1, v0)
}

pub fn f61(v0: Str) -> Bytes {
    let a188 = v0;
    utf8_encode(a188)
}

pub fn f62(v0: Str) -> Bytes {
    let a189 = v0;
    utf8_encode(a189)
}

pub fn f63(v0: Fn8, p193: List<Adt7>, p194: List<(Option<Bytes>, Bytes)>) -> List<Adt7> {
    let m192 = (p193, p194);
    {
        let mut m192 = m192;
        loop {
            let step = {
                let (v1, v2) = m192;
                let m190 = v2;
                match m190.uncons() {
                    None => {
                        Err::<(List<Adt7>, List<(Option<Bytes>, Bytes)>), List<Adt7>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt7>, List<(Option<Bytes>, Bytes)>), List<Adt7>>(({
                            let c191: Fn8 = v0.clone();
                            c191.apply(v1, v3)
                        }, v4))
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

pub fn f64(v0: List<Adt7>, v1: (Option<Bytes>, Bytes)) -> List<Adt7> {
    List::cons(f11({
        let (h195, _) = v1;
        h195
    }, {
        let (_, h196) = v1;
        h196
    }), v0)
}

pub fn f65(v0: List<(Option<Bytes>, Bytes)>) -> List<(Option<Bytes>, Bytes)> {
    f79(Fn12::F80, List::<(Option<Bytes>, Bytes)>::nil(), v0)
}

pub fn f66(v0: Fn9, p200: List<Adt7>, p201: List<(Bytes, Bytes)>) -> List<Adt7> {
    let m199 = (p200, p201);
    {
        let mut m199 = m199;
        loop {
            let step = {
                let (v1, v2) = m199;
                let m197 = v2;
                match m197.uncons() {
                    None => {
                        Err::<(List<Adt7>, List<(Bytes, Bytes)>), List<Adt7>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt7>, List<(Bytes, Bytes)>), List<Adt7>>(({
                            let c198: Fn9 = v0.clone();
                            c198.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m199 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f67(v0: List<Adt7>, v1: (Bytes, Bytes)) -> List<Adt7> {
    List::cons(Adt7::C4(std::rc::Rc::new(f34(Str::lit("field"), Adt7::C2({
        let (h202, _) = v1;
        h202
    }), f34(Str::lit("expected"), Adt7::C2({
        let (_, h203) = v1;
        h203
    }), Adt7::C5)))), v0)
}

pub fn f68(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f49(Fn3::F81, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f69(v0: Bytes, v1: u64, v2: u64) -> R<Bytes> {
    let a204 = v2;
    let a205 = 0u64;
    if nat_eq(a204, a205) {
        Ok(f82(v0, 0u64))
    } else {
        let (_, h206) = {
            let (h207, _) = f46(Fn2::F83(v1, v0), v2, (v2, Bytes::lit(&[])))?;
            h207
        };
        Ok(h206)
    }
}

pub fn f70() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f71(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let m211 = {
        let a209 = v0.clone();
        let a210 = {
            let (h208, _) = v1.clone();
            h208
        };
        index_bytes(a209, a210)
    };
    match m211 {
        None => {
            Ok(None::<(u64, Bytes)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a213 = {
                    let (h212, _) = v1.clone();
                    h212
                };
                let a214 = 1u64;
                nat_add(a213, a214)?
            }, {
                let a217 = {
                    let (_, h215) = v1;
                    h215
                };
                let a218 = f84(v0, {
                    let (h216, _) = v1;
                    h216
                }, v2)?;
                append_bytes(a217, a218)
            })))
        }
    }
}

pub fn f72(v0: Fn10, p222: Option<Bytes>, p223: List<Bytes>) -> Option<Bytes> {
    let m221 = (p222, p223);
    {
        let mut m221 = m221;
        loop {
            let step = {
                let (v1, v2) = m221;
                let m219 = v2;
                match m219.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(({
                            let c220: Fn10 = v0.clone();
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

pub fn f73(v0: Bytes, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m224 = v1;
    match m224 {
        None => {
            Some(v2)
        }
        Some(v3) => {
            Some({
                let a227 = {
                    let a225 = v3;
                    let a226 = v0;
                    append_bytes(a225, a226)
                };
                let a228 = v2;
                append_bytes(a227, a228)
            })
        }
    }
}

pub fn f74(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    let m229 = v0.clone();
    match m229.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, v3)) => {
            let a232 = {
                let (h230, _) = v2;
                h230
            };
            let a233 = {
                let (h231, _) = v1.clone();
                h231
            };
            if equal(a232, a233) {
                List::cons(v1, v3)
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f75(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v1: List<List<(Bytes, Bytes)>> = f85(f86(Fn13::F87, List::<List<(Bytes, Bytes)>>::nil(), v0.clone()));
    let m238 = {
        let (h234, _) = f88(Fn14::F89, {
            let a236 = {
                let a235 = v0;
                length_list(a235)
            };
            let a237 = 1u64;
            nat_add(a236, a237)?
        }, v1)?;
        h234
    };
    match m238.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f76(v0: (Bytes, Bytes)) -> R<Bytes> {
    let a243 = {
        let a240 = f21({
            let (h239, _) = v0;
            h239
        })?;
        let a241 = Bytes::lit(&[0x3a]);
        append_bytes(a240, a241)
    };
    let a244 = {
        let (_, h242) = v0;
        h242
    };
    Ok(append_bytes(a243, a244))
}

pub fn f77(v0: Fn11, p248: List<Adt7>, p249: List<Adt7>) -> List<Adt7> {
    let m247 = (p248, p249);
    {
        let mut m247 = m247;
        loop {
            let step = {
                let (v1, v2) = m247;
                let m245 = v2;
                match m245.uncons() {
                    None => {
                        Err::<(List<Adt7>, List<Adt7>), List<Adt7>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt7>, List<Adt7>), List<Adt7>>(({
                            let c246: Fn11 = v0.clone();
                            c246.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m247 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f78(v0: List<Adt7>, v1: Adt7) -> List<Adt7> {
    List::cons(v1, v0)
}

pub fn f79(v0: Fn12, p253: List<(Option<Bytes>, Bytes)>, p254: List<(Option<Bytes>, Bytes)>) -> List<(Option<Bytes>, Bytes)> {
    let m252 = (p253, p254);
    {
        let mut m252 = m252;
        loop {
            let step = {
                let (v1, v2) = m252;
                let m250 = v2;
                match m250.uncons() {
                    None => {
                        Err::<(List<(Option<Bytes>, Bytes)>, List<(Option<Bytes>, Bytes)>), List<(Option<Bytes>, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Option<Bytes>, Bytes)>, List<(Option<Bytes>, Bytes)>), List<(Option<Bytes>, Bytes)>>(({
                            let c251: Fn12 = v0.clone();
                            c251.apply(v1, v3)
                        }, v4))
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

pub fn f80(v0: List<(Option<Bytes>, Bytes)>, v1: (Option<Bytes>, Bytes)) -> List<(Option<Bytes>, Bytes)> {
    List::cons(v1, v0)
}

pub fn f81(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn f82(v0: Bytes, v1: u64) -> Bytes {
    let m258 = {
        let a255 = v0;
        let a256 = v1;
        let a257 = 1u64;
        slice_bytes(a255, a256, a257)
    };
    match m258 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f83(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f90(v1, v0, v2)
}

pub fn f84(v0: Bytes, v1: u64, v2: u8) -> R<Bytes> {
    let v4: u64 = {
        let v3: u8 = v2;
        let a303 = {
            let a279 = {
                let a267 = {
                    let a261 = {
                        let a259 = v3;
                        let a260 = 128u8;
                        fixed_u8::bit_and(a259, a260)
                    };
                    let a262 = 128u8;
                    if equal(a261, a262) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a268 = {
                    let a265 = {
                        let a263 = v3;
                        let a264 = 64u8;
                        fixed_u8::bit_and(a263, a264)
                    };
                    let a266 = 64u8;
                    if equal(a265, a266) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a267, a268)?
            };
            let a280 = {
                let a277 = {
                    let a271 = {
                        let a269 = v3;
                        let a270 = 32u8;
                        fixed_u8::bit_and(a269, a270)
                    };
                    let a272 = 32u8;
                    if equal(a271, a272) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a278 = {
                    let a275 = {
                        let a273 = v3;
                        let a274 = 16u8;
                        fixed_u8::bit_and(a273, a274)
                    };
                    let a276 = 16u8;
                    if equal(a275, a276) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a277, a278)?
            };
            nat_add(a279, a280)?
        };
        let a304 = {
            let a301 = {
                let a289 = {
                    let a283 = {
                        let a281 = v3;
                        let a282 = 8u8;
                        fixed_u8::bit_and(a281, a282)
                    };
                    let a284 = 8u8;
                    if equal(a283, a284) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a290 = {
                    let a287 = {
                        let a285 = v3;
                        let a286 = 4u8;
                        fixed_u8::bit_and(a285, a286)
                    };
                    let a288 = 4u8;
                    if equal(a287, a288) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a289, a290)?
            };
            let a302 = {
                let a299 = {
                    let a293 = {
                        let a291 = v3;
                        let a292 = 2u8;
                        fixed_u8::bit_and(a291, a292)
                    };
                    let a294 = 2u8;
                    if equal(a293, a294) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a300 = {
                    let a297 = {
                        let a295 = v3;
                        let a296 = 1u8;
                        fixed_u8::bit_and(a295, a296)
                    };
                    let a298 = 1u8;
                    if equal(a297, a298) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a299, a300)?
            };
            nat_add(a301, a302)?
        };
        nat_add(a303, a304)?
    };
    let a305 = v4;
    let a306 = 34u64;
    if nat_eq(a305, a306) {
        Ok(Bytes::lit(&[0x5c, 0x22]))
    } else {
        let a307 = v4;
        let a308 = 92u64;
        if nat_eq(a307, a308) {
            Ok(Bytes::lit(&[0x5c, 0x5c]))
        } else {
            let a309 = v4;
            let a310 = 8u64;
            if nat_eq(a309, a310) {
                Ok(Bytes::lit(&[0x5c, 0x62]))
            } else {
                let a311 = v4;
                let a312 = 12u64;
                if nat_eq(a311, a312) {
                    Ok(Bytes::lit(&[0x5c, 0x66]))
                } else {
                    let a313 = v4;
                    let a314 = 10u64;
                    if nat_eq(a313, a314) {
                        Ok(Bytes::lit(&[0x5c, 0x6e]))
                    } else {
                        let a315 = v4;
                        let a316 = 13u64;
                        if nat_eq(a315, a316) {
                            Ok(Bytes::lit(&[0x5c, 0x72]))
                        } else {
                            let a317 = v4;
                            let a318 = 9u64;
                            if nat_eq(a317, a318) {
                                Ok(Bytes::lit(&[0x5c, 0x74]))
                            } else {
                                let a319 = v4;
                                let a320 = 32u64;
                                if nat_lt(a319, a320) {
                                    let a321 = Bytes::lit(&[0x5c, 0x75, 0x30, 0x30]);
                                    let a322 = f91(v2)?;
                                    Ok(append_bytes(a321, a322))
                                } else {
                                    Ok(f92(v0, v1, 1u64))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f85(v0: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    f93(Fn15::F94, List::<List<(Bytes, Bytes)>>::nil(), v0)
}

pub fn f86(v0: Fn13, p326: List<List<(Bytes, Bytes)>>, p327: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    let m325 = (p326, p327);
    {
        let mut m325 = m325;
        loop {
            let step = {
                let (v1, v2) = m325;
                let m323 = v2;
                match m323.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(({
                            let c324: Fn13 = v0.clone();
                            c324.apply(v1, v3)
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

pub fn f87(v0: List<List<(Bytes, Bytes)>>, v1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
    List::cons(List::cons(v1, List::<(Bytes, Bytes)>::nil()), v0)
}

pub fn f88(v0: Fn14, p332: u64, p333: List<List<(Bytes, Bytes)>>) -> R<(List<List<(Bytes, Bytes)>>, bool)> {
    let m331 = (p332, p333);
    {
        let mut m331 = m331;
        loop {
            let step = {
                let (v1, v2) = m331;
                let m328 = v1;
                if m328 == 0 {
                    Err::<(u64, List<List<(Bytes, Bytes)>>), R<(List<List<(Bytes, Bytes)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m328 - 1;
                    let m330 = {
                        let c329: Fn14 = v0.clone();
                        c329.apply(v2.clone())?
                    };
                    match m330 {
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
                Ok(next) => { m331 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f89(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    f95(v0)
}

pub fn f90(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h334, _) = v2;
        h334
    };
    let a335 = v3;
    let a336 = 0u64;
    if nat_eq(a335, a336) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a337 = v3;
            let a338 = v1;
            let a339 = 0u64;
            nat_quot(a337, a338, a339)
        }, {
            let a344 = f82(v0, {
                let a340 = v3;
                let a341 = v1;
                let a342 = 0u64;
                nat_rem(a340, a341, a342)
            });
            let a345 = {
                let (_, h343) = v2;
                h343
            };
            append_bytes(a344, a345)
        }))
    }
}

pub fn f91(v0: u8) -> R<Bytes> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a390 = {
            let a366 = {
                let a354 = {
                    let a348 = {
                        let a346 = v1;
                        let a347 = 128u8;
                        fixed_u8::bit_and(a346, a347)
                    };
                    let a349 = 128u8;
                    if equal(a348, a349) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a355 = {
                    let a352 = {
                        let a350 = v1;
                        let a351 = 64u8;
                        fixed_u8::bit_and(a350, a351)
                    };
                    let a353 = 64u8;
                    if equal(a352, a353) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a354, a355)?
            };
            let a367 = {
                let a364 = {
                    let a358 = {
                        let a356 = v1;
                        let a357 = 32u8;
                        fixed_u8::bit_and(a356, a357)
                    };
                    let a359 = 32u8;
                    if equal(a358, a359) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a365 = {
                    let a362 = {
                        let a360 = v1;
                        let a361 = 16u8;
                        fixed_u8::bit_and(a360, a361)
                    };
                    let a363 = 16u8;
                    if equal(a362, a363) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a364, a365)?
            };
            nat_add(a366, a367)?
        };
        let a391 = {
            let a388 = {
                let a376 = {
                    let a370 = {
                        let a368 = v1;
                        let a369 = 8u8;
                        fixed_u8::bit_and(a368, a369)
                    };
                    let a371 = 8u8;
                    if equal(a370, a371) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a377 = {
                    let a374 = {
                        let a372 = v1;
                        let a373 = 4u8;
                        fixed_u8::bit_and(a372, a373)
                    };
                    let a375 = 4u8;
                    if equal(a374, a375) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a376, a377)?
            };
            let a389 = {
                let a386 = {
                    let a380 = {
                        let a378 = v1;
                        let a379 = 2u8;
                        fixed_u8::bit_and(a378, a379)
                    };
                    let a381 = 2u8;
                    if equal(a380, a381) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a387 = {
                    let a384 = {
                        let a382 = v1;
                        let a383 = 1u8;
                        fixed_u8::bit_and(a382, a383)
                    };
                    let a385 = 1u8;
                    if equal(a384, a385) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a386, a387)?
            };
            nat_add(a388, a389)?
        };
        nat_add(a390, a391)?
    };
    let a398 = f82(f96(), {
        let a392 = v2;
        let a393 = 16u64;
        let a394 = 0u64;
        nat_quot(a392, a393, a394)
    });
    let a399 = f82(f96(), {
        let a395 = v2;
        let a396 = 16u64;
        let a397 = 0u64;
        nat_rem(a395, a396, a397)
    });
    Ok(append_bytes(a398, a399))
}

pub fn f92(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m403 = {
        let a400 = v0;
        let a401 = v1;
        let a402 = v2;
        slice_bytes(a400, a401, a402)
    };
    match m403 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f93(v0: Fn15, p407: List<List<(Bytes, Bytes)>>, p408: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    let m406 = (p407, p408);
    {
        let mut m406 = m406;
        loop {
            let step = {
                let (v1, v2) = m406;
                let m404 = v2;
                match m404.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(({
                            let c405: Fn15 = v0.clone();
                            c405.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m406 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f94(v0: List<List<(Bytes, Bytes)>>, v1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    List::cons(v1, v0)
}

pub fn f95(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    let a410 = {
        let a409 = v0.clone();
        length_list(a409)
    };
    let a411 = 1u64;
    if nat_le(a410, a411) {
        Ok(None::<List<List<(Bytes, Bytes)>>>)
    } else {
        Ok(Some(f97(v0)?))
    }
}

pub fn f96() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66])
}

pub fn f97(v0: List<List<(Bytes, Bytes)>>) -> R<List<List<(Bytes, Bytes)>>> {
    Ok(f85({
        let (_, h412) = {
            let (h413, _) = f98(Fn16::F99, {
                let a415 = {
                    let a414 = v0.clone();
                    length_list(a414)
                };
                let a416 = 1u64;
                nat_add(a415, a416)?
            }, (v0, List::<List<(Bytes, Bytes)>>::nil()))?;
            h413
        };
        h412
    }))
}

pub fn f98(v0: Fn16, p421: u64, p422: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)> {
    let m420 = (p421, p422);
    {
        let mut m420 = m420;
        loop {
            let step = {
                let (v1, v2) = m420;
                let m417 = v1;
                if m417 == 0 {
                    Err::<(u64, (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)), R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m417 - 1;
                    let m419 = {
                        let c418: Fn16 = v0.clone();
                        c418.apply(v2.clone())?
                    };
                    match m419 {
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
                Ok(next) => { m420 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f99(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    f100(v0)
}

pub fn f100(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    let v1: List<List<(Bytes, Bytes)>> = {
        let (h423, _) = v0;
        h423
    };
    let v2: List<List<(Bytes, Bytes)>> = {
        let (_, h424) = v0;
        h424
    };
    let m425 = v1;
    match m425.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>)
        }
        Some((v3, v4)) => {
            let m426 = v4;
            match m426.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Bytes)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f101(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f101(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v2: u64 = {
        let a431 = {
            let a429 = {
                let a427 = v0.clone();
                length_list(a427)
            };
            let a430 = {
                let a428 = v1.clone();
                length_list(a428)
            };
            nat_add(a429, a430)?
        };
        let a432 = 1u64;
        nat_add(a431, a432)?
    };
    Ok(f68({
        let (_, h433) = {
            let (_, h434) = {
                let (h435, _) = f102(Fn17::F103, v2, (v0, (v1, List::<(Bytes, Bytes)>::nil())));
                h435
            };
            h434
        };
        h433
    }))
}

pub fn f102(v0: Fn17, p440: u64, p441: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool) {
    let m439 = (p440, p441);
    {
        let mut m439 = m439;
        loop {
            let step = {
                let (v1, v2) = m439;
                let m436 = v1;
                if m436 == 0 {
                    Err::<(u64, (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))), ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool)>((v2, false))
                } else {
                    let v3 = m436 - 1;
                    let m438 = {
                        let c437: Fn17 = v0.clone();
                        c437.apply(v2.clone())
                    };
                    match m438 {
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
                Ok(next) => { m439 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f103(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    f104(v0)
}

pub fn f104(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    let v1: List<(Bytes, Bytes)> = {
        let (h442, _) = v0.clone();
        h442
    };
    let v2: List<(Bytes, Bytes)> = {
        let (h443, _) = {
            let (_, h444) = v0.clone();
            h444
        };
        h443
    };
    let v3: List<(Bytes, Bytes)> = {
        let (_, h445) = {
            let (_, h446) = v0;
            h446
        };
        h445
    };
    let m447 = v1.clone();
    match m447.uncons() {
        None => {
            let m448 = v2;
            match m448.uncons() {
                None => {
                    None::<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Bytes)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m449 = v2.clone();
            match m449.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Bytes)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f105({
                        let (h450, _) = v8.clone();
                        h450
                    }, {
                        let (h451, _) = v6.clone();
                        h451
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

pub fn f105(v0: Bytes, v1: Bytes) -> bool {
    let a456 = {
        let a452 = v0;
        let a453 = v1;
        compare_bytes(a452, a453)
    };
    let a457 = {
        let a454 = Bytes::lit(&[]);
        let a455 = Bytes::lit(&[0x01]);
        compare_bytes(a454, a455)
    };
    equal(a456, a457)
}

pub fn run(p0: Adt0) -> R<Bytes> {
    f0(p0)
}
