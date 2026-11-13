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
    C0(Adt3, Adt6),
    C1(Adt7, Adt8, Adt6),
    C2(Adt9, std::rc::Rc<Adt2>, Adt6),
    C3(Adt10, std::rc::Rc<Adt2>, bool, Adt6),
    C4(Adt8, std::rc::Rc<Adt2>, bool, Adt6),
    C5(Adt12, std::rc::Rc<Adt2>, Adt6),
    C6(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>, Adt6),
    C7(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>, std::rc::Rc<Adt2>, Adt6),
    C8(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>, Adt6),
    C9(std::rc::Rc<Adt2>),
    C10(Adt18, Adt8, Adt6),
    C11(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C12,
    C13(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C14(Adt19, bool, Adt13, std::rc::Rc<Adt2>, Adt6, std::rc::Rc<Adt2>),
    C15(Adt19, std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C16(Adt8, std::rc::Rc<Adt2>),
}

#[derive(Clone)]
pub enum Adt3 {
    C0,
    C1(bool),
    C2(u64),
    C3(bool, u64),
    C4(Adt4, bool, u64),
    C5(Bytes),
    C6(Bytes),
    C7(Adt5),
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
}

#[derive(Clone)]
pub enum Adt5 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt6 {
    C0(Bytes, Option<Adt4>),
}

#[derive(Clone)]
pub enum Adt7 {
    C0,
    C1,
    C2,
    C3,
    C4,
}

#[derive(Clone)]
pub enum Adt8 {
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
pub enum Adt9 {
    C0,
    C1,
    C2,
    C3,
}

#[derive(Clone)]
pub enum Adt10 {
    C0(u64),
    C1(Adt11),
    C2(u64, Option<Adt4>),
}

#[derive(Clone)]
pub enum Adt11 {
    C0(u64, Option<Adt4>),
}

#[derive(Clone)]
pub enum Adt12 {
    C0(Adt13),
    C1,
    C2(Adt13, Adt13),
    C3(Adt13, Adt13),
    C4(u64, u64),
    C5(u64, u64),
    C6,
    C7(Adt13),
}

#[derive(Clone)]
pub enum Adt13 {
    C0(Adt14),
    C1(Adt4),
    C2(Adt15, std::rc::Rc<Adt13>),
    C3(Adt16, std::rc::Rc<Adt13>, std::rc::Rc<Adt13>),
    C4(Adt17, u64),
    C5,
    C6(std::rc::Rc<Adt13>, std::rc::Rc<Adt13>),
}

#[derive(Clone)]
pub enum Adt14 {
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
pub enum Adt15 {
    C0,
    C1,
    C2,
    C3,
    C4,
}

#[derive(Clone)]
pub enum Adt16 {
    C0,
    C1,
}

#[derive(Clone)]
pub enum Adt17 {
    C0,
    C1,
    C2,
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
    C0(Adt20),
    C1(Adt8),
    C2(std::rc::Rc<Adt19>),
    C3(Adt21, std::rc::Rc<Adt19>),
    C4(u64, u64, std::rc::Rc<Adt19>),
    C5,
    C6(std::rc::Rc<Adt19>, std::rc::Rc<Adt19>),
    C7(Adt5),
}

#[derive(Clone)]
pub enum Adt20 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt21 {
    C0,
    C1,
    C2,
}

pub fn f0(v0: Adt0, v1: List<bool>, v2: Adt2) -> R<Adt2> {
    f1(v0, v1, v2, 0u64)
}

pub fn f1(v0: Adt0, v1: List<bool>, v2: Adt2, v3: u64) -> R<Adt2> {
    let m1 = v2;
    match m1 {
        Adt2::C0(_, _) => {
            Ok(Adt2::C12)
        }
        Adt2::C1(_, _, _) => {
            Ok(Adt2::C12)
        }
        Adt2::C2(_, _, _) => {
            Ok(Adt2::C12)
        }
        Adt2::C3(_, _, _, _) => {
            Ok(Adt2::C12)
        }
        Adt2::C4(_, _, _, _) => {
            Ok(Adt2::C12)
        }
        Adt2::C5(_, _, _) => {
            Ok(Adt2::C12)
        }
        Adt2::C6(_, _, _) => {
            Ok(Adt2::C12)
        }
        Adt2::C7(_, _, _, _) => {
            Ok(Adt2::C12)
        }
        Adt2::C8(_, _, _) => {
            Ok(Adt2::C12)
        }
        Adt2::C9(_) => {
            Ok(Adt2::C12)
        }
        Adt2::C10(_, _, _) => {
            Ok(Adt2::C12)
        }
        Adt2::C11(_, _) => {
            Ok(Adt2::C12)
        }
        Adt2::C12 => {
            Ok(Adt2::C12)
        }
        Adt2::C13(r5, r6) => {
            let v39 = (*r5).clone();
            let v40 = (*r6).clone();
            Ok(Adt2::C13(std::rc::Rc::new(if if f2(v1.clone(), v3) {
                let a2 = f3(v39.clone());
                bool_not(a2)
            } else {
                false
            } {
                f4(v39.clone(), v39, f5(v0.clone(), v3), 1u64)
            } else {
                v39
            }), std::rc::Rc::new(f1(v0, v1, v40, {
                let a3 = v3;
                let a4 = 1u64;
                nat_add(a3, a4)?
            })?)))
        }
        Adt2::C14(_, _, _, _, _, _) => {
            Ok(Adt2::C12)
        }
        Adt2::C15(_, _, _) => {
            Ok(Adt2::C12)
        }
        Adt2::C16(_, _) => {
            Ok(Adt2::C12)
        }
    }
}

pub fn f2(v0: List<bool>, v1: u64) -> bool {
    let m7 = f6(v0, v1);
    match m7 {
        None => {
            false
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f3(v0: Adt2) -> bool {
    let m8 = v0;
    match m8 {
        Adt2::C0(_, _) => {
            false
        }
        Adt2::C1(_, _, _) => {
            false
        }
        Adt2::C2(_, _, _) => {
            false
        }
        Adt2::C3(_, _, _, _) => {
            false
        }
        Adt2::C4(_, _, _, _) => {
            false
        }
        Adt2::C5(_, _, _) => {
            false
        }
        Adt2::C6(_, _, _) => {
            false
        }
        Adt2::C7(_, _, _, _) => {
            false
        }
        Adt2::C8(_, _, _) => {
            false
        }
        Adt2::C9(_) => {
            false
        }
        Adt2::C10(_, _, _) => {
            false
        }
        Adt2::C11(_, _) => {
            false
        }
        Adt2::C12 => {
            true
        }
        Adt2::C13(_, _) => {
            false
        }
        Adt2::C14(_, _, _, _, _, _) => {
            false
        }
        Adt2::C15(_, _, _) => {
            false
        }
        Adt2::C16(_, _) => {
            false
        }
    }
}

pub fn f4(v0: Adt2, v1: Adt2, v2: bool, v3: u64) -> Adt2 {
    let m9 = v0;
    match m9 {
        Adt2::C0(_, _) => {
            let a10 = v3;
            let a11 = 0u64;
            if if nat_eq(a10, a11) {
                f7(v1.clone())
            } else {
                false
            } {
                f8(v1)
            } else {
                f9(v1, v2, v3)
            }
        }
        Adt2::C1(_, _, _) => {
            f9(v1, v2, v3)
        }
        Adt2::C2(_, _, _) => {
            f9(v1, v2, v3)
        }
        Adt2::C3(v12, r24, v14, v15) => {
            let v13 = (*r24).clone();
            let a12 = v3;
            let a13 = 0u64;
            if if nat_eq(a12, a13) {
                v14
            } else {
                false
            } {
                Adt2::C3(v12, std::rc::Rc::new(v13), false, v15)
            } else {
                f9(v1, v2, v3)
            }
        }
        Adt2::C4(v16, r25, v18, v19) => {
            let v17 = (*r25).clone();
            let a14 = v3;
            let a15 = 0u64;
            if if nat_eq(a14, a15) {
                v18
            } else {
                false
            } {
                Adt2::C4(v16, std::rc::Rc::new(v17), false, v19)
            } else {
                f9(v1, v2, v3)
            }
        }
        Adt2::C5(_, _, _) => {
            f9(v1, v2, v3)
        }
        Adt2::C6(_, _, _) => {
            f9(v1, v2, v3)
        }
        Adt2::C7(r26, r27, r28, v29) => {
            let v26 = (*r26).clone();
            let v27 = (*r27).clone();
            let v28 = (*r28).clone();
            let a16 = v3;
            let a17 = 0u64;
            if nat_eq(a16, a17) {
                Adt2::C7(std::rc::Rc::new(v26), std::rc::Rc::new(f4(v27.clone(), v27, v2, 1u64)), std::rc::Rc::new(f4(v28.clone(), v28, v2, 1u64)), v29)
            } else {
                v1
            }
        }
        Adt2::C8(r29, r30, v32) => {
            let v30 = (*r29).clone();
            let v31 = (*r30).clone();
            let a18 = v3;
            let a19 = 0u64;
            if nat_eq(a18, a19) {
                Adt2::C8(std::rc::Rc::new(v30), std::rc::Rc::new(f4(v31.clone(), v31, v2, 2u64)), v32)
            } else {
                v1
            }
        }
        Adt2::C9(_) => {
            f9(v1, v2, v3)
        }
        Adt2::C10(_, _, _) => {
            f9(v1, v2, v3)
        }
        Adt2::C11(r31, r32) => {
            let v37 = (*r31).clone();
            let v38 = (*r32).clone();
            let a20 = v3;
            let a21 = 1u64;
            if nat_eq(a20, a21) {
                f10(Adt2::C11(std::rc::Rc::new(v37), std::rc::Rc::new(v38.clone())), f4(v38.clone(), v38, v2, 0u64))
            } else {
                f9(v1, v2, v3)
            }
        }
        Adt2::C12 => {
            f9(v1, v2, v3)
        }
        Adt2::C13(_, _) => {
            f9(v1, v2, v3)
        }
        Adt2::C14(_, _, _, _, _, _) => {
            f9(v1, v2, v3)
        }
        Adt2::C15(v47, r33, r34) => {
            let v48 = (*r33).clone();
            let v49 = (*r34).clone();
            let a22 = v3;
            let a23 = 2u64;
            if nat_eq(a22, a23) {
                Adt2::C15(v47, std::rc::Rc::new(f4(v48.clone(), v48, v2, 1u64)), std::rc::Rc::new(f4(v49.clone(), v49, v2, 2u64)))
            } else {
                f9(v1, v2, v3)
            }
        }
        Adt2::C16(_, _) => {
            f9(v1, v2, v3)
        }
    }
}

pub fn f5(v0: Adt0, v1: u64) -> bool {
    let m35 = f11(v0, v1);
    match m35 {
        None => {
            false
        }
        Some(v2) => {
            f12(v2, Adt0::C0)
        }
    }
}

pub fn f6(p39: List<bool>, p40: u64) -> Option<bool> {
    let m38 = (p39, p40);
    {
        let mut m38 = m38;
        loop {
            let step = {
                let (v0, v1) = m38;
                let m36 = v0;
                match m36.uncons() {
                    None => {
                        Err::<(List<bool>, u64), Option<bool>>(None::<bool>)
                    }
                    Some((v2, v3)) => {
                        let m37 = v1;
                        if m37 == 0 {
                            Err::<(List<bool>, u64), Option<bool>>(Some(v2))
                        } else {
                            let v4 = m37 - 1;
                            Ok::<(List<bool>, u64), Option<bool>>((v3, v4))
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

pub fn f7(v0: Adt2) -> bool {
    let m41 = v0;
    match m41 {
        Adt2::C0(v1, _) => {
            let m42 = v1;
            match m42 {
                Adt3::C0 => {
                    true
                }
                Adt3::C1(_) => {
                    false
                }
                Adt3::C2(_) => {
                    false
                }
                Adt3::C3(_, _) => {
                    false
                }
                Adt3::C4(_, _, _) => {
                    false
                }
                Adt3::C5(_) => {
                    false
                }
                Adt3::C6(_) => {
                    false
                }
                Adt3::C7(_) => {
                    false
                }
                Adt3::C8 => {
                    false
                }
            }
        }
        Adt2::C1(_, _, _) => {
            false
        }
        Adt2::C2(_, _, _) => {
            false
        }
        Adt2::C3(_, _, _, _) => {
            false
        }
        Adt2::C4(_, _, _, _) => {
            false
        }
        Adt2::C5(_, _, _) => {
            false
        }
        Adt2::C6(_, _, _) => {
            false
        }
        Adt2::C7(_, _, _, _) => {
            false
        }
        Adt2::C8(_, _, _) => {
            false
        }
        Adt2::C9(_) => {
            false
        }
        Adt2::C10(_, _, _) => {
            false
        }
        Adt2::C11(_, _) => {
            false
        }
        Adt2::C12 => {
            false
        }
        Adt2::C13(_, _) => {
            false
        }
        Adt2::C14(_, _, _, _, _, _) => {
            false
        }
        Adt2::C15(_, _, _) => {
            false
        }
        Adt2::C16(_, _) => {
            false
        }
    }
}

pub fn f8(v0: Adt2) -> Adt2 {
    Adt2::C2(Adt9::C3, std::rc::Rc::new(v0), f13(0u64))
}

pub fn f9(v0: Adt2, v1: bool, v2: u64) -> Adt2 {
    let a43 = v2;
    let a44 = 0u64;
    if nat_eq(a43, a44) {
        if v1 {
            f14(v0)
        } else {
            f8(v0)
        }
    } else {
        v0
    }
}

pub fn f10(v0: Adt2, v1: Adt2) -> Adt2 {
    f15(v0, v1.clone(), v1)
}

pub fn f11(p50: Adt0, p51: u64) -> Option<Adt0> {
    let m49 = (p50, p51);
    {
        let mut m49 = m49;
        loop {
            let step = {
                let (v0, v1) = m49;
                let m45 = v0;
                match m45 {
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
                    Adt0::C15(r47, r48) => {
                        let v12 = (*r47).clone();
                        let v13 = (*r48).clone();
                        let m46 = v1;
                        if m46 == 0 {
                            Err::<(Adt0, u64), Option<Adt0>>(Some(v12))
                        } else {
                            let v14 = m46 - 1;
                            Ok::<(Adt0, u64), Option<Adt0>>((v13, v14))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m49 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f12(v0: Adt0, v1: Adt0) -> bool {
    let m52 = v0;
    match m52 {
        Adt0::C0 => {
            let a53 = f16(v1);
            let a54 = 0u64;
            nat_eq(a53, a54)
        }
        Adt0::C1 => {
            let a55 = f16(v1);
            let a56 = 1u64;
            nat_eq(a55, a56)
        }
        Adt0::C2 => {
            let a57 = f16(v1);
            let a58 = 2u64;
            nat_eq(a57, a58)
        }
        Adt0::C3 => {
            let a59 = f16(v1);
            let a60 = 3u64;
            nat_eq(a59, a60)
        }
        Adt0::C4(v2) => {
            let a69 = f16(v1.clone());
            let a70 = 4u64;
            if nat_eq(a69, a70) {
                let a71 = f17(v2);
                let a72 = f17(f18(v1));
                nat_eq(a71, a72)
            } else {
                false
            }
        }
        Adt0::C5 => {
            let a61 = f16(v1);
            let a62 = 5u64;
            nat_eq(a61, a62)
        }
        Adt0::C6 => {
            let a63 = f16(v1);
            let a64 = 6u64;
            nat_eq(a63, a64)
        }
        Adt0::C7 => {
            let a65 = f16(v1);
            let a66 = 7u64;
            nat_eq(a65, a66)
        }
        Adt0::C8(r89) => {
            let v3 = (*r89).clone();
            let v4: bool = f12(v3, f19(v1.clone()));
            let a73 = f16(v1);
            let a74 = 8u64;
            if nat_eq(a73, a74) {
                v4
            } else {
                false
            }
        }
        Adt0::C9(r90, r91) => {
            let v5 = (*r90).clone();
            let v6 = (*r91).clone();
            let v7: bool = f12(v5, f19(v1.clone()));
            let v8: bool = f12(v6, f20(v1.clone()));
            let a75 = f16(v1);
            let a76 = 9u64;
            if nat_eq(a75, a76) {
                if v7 {
                    v8
                } else {
                    false
                }
            } else {
                false
            }
        }
        Adt0::C10(r92) => {
            let v9 = (*r92).clone();
            let v10: bool = f12(v9, f19(v1.clone()));
            let a77 = f16(v1);
            let a78 = 10u64;
            if nat_eq(a77, a78) {
                v10
            } else {
                false
            }
        }
        Adt0::C11(r93, r94) => {
            let v11 = (*r93).clone();
            let v12 = (*r94).clone();
            let v13: bool = f12(v11, f19(v1.clone()));
            let v14: bool = f12(v12, f20(v1.clone()));
            let a79 = f16(v1);
            let a80 = 11u64;
            if nat_eq(a79, a80) {
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
            let a81 = f16(v1.clone());
            let a82 = 12u64;
            if nat_eq(a81, a82) {
                let a83 = v15;
                let a84 = f21(v1);
                nat_eq(a83, a84)
            } else {
                false
            }
        }
        Adt0::C13(r95, r96) => {
            let v16 = (*r95).clone();
            let v17 = (*r96).clone();
            let v18: bool = f12(v16, f19(v1.clone()));
            let v19: bool = f12(v17, f20(v1.clone()));
            let a85 = f16(v1);
            let a86 = 13u64;
            if nat_eq(a85, a86) {
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
            let a67 = f16(v1);
            let a68 = 14u64;
            nat_eq(a67, a68)
        }
        Adt0::C15(r97, r98) => {
            let v20 = (*r97).clone();
            let v21 = (*r98).clone();
            let v22: bool = f12(v20, f19(v1.clone()));
            let v23: bool = f12(v21, f20(v1.clone()));
            let a87 = f16(v1);
            let a88 = 15u64;
            if nat_eq(a87, a88) {
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

pub fn f13(_: u64) -> Adt6 {
    Adt6::C0({
        let a99 = Str::lit("overflow");
        utf8_encode(a99)
    }, None::<Adt4>)
}

pub fn f14(v0: Adt2) -> Adt2 {
    Adt2::C9(std::rc::Rc::new(Adt2::C11(std::rc::Rc::new(Adt2::C14(Adt19::C0(Adt20::C1), false, f22(0u64), std::rc::Rc::new(v0), f13(0u64), std::rc::Rc::new(Adt2::C12))), std::rc::Rc::new(f8(Adt2::C0(Adt3::C0, f13(0u64)))))))
}

pub fn f15(v0: Adt2, v1: Adt2, v2: Adt2) -> Adt2 {
    let m100 = v1;
    match m100 {
        Adt2::C0(_, _) => {
            Adt2::C11(std::rc::Rc::new(f23(v0)), std::rc::Rc::new(v2))
        }
        Adt2::C1(_, _, _) => {
            Adt2::C11(std::rc::Rc::new(f23(v0)), std::rc::Rc::new(v2))
        }
        Adt2::C2(_, _, _) => {
            Adt2::C11(std::rc::Rc::new(f23(v0)), std::rc::Rc::new(v2))
        }
        Adt2::C3(_, _, _, _) => {
            Adt2::C11(std::rc::Rc::new(f23(v0)), std::rc::Rc::new(v2))
        }
        Adt2::C4(_, _, _, _) => {
            Adt2::C11(std::rc::Rc::new(f23(v0)), std::rc::Rc::new(v2))
        }
        Adt2::C5(_, _, _) => {
            Adt2::C11(std::rc::Rc::new(f23(v0)), std::rc::Rc::new(v2))
        }
        Adt2::C6(_, _, _) => {
            Adt2::C11(std::rc::Rc::new(f23(v0)), std::rc::Rc::new(v2))
        }
        Adt2::C7(_, _, _, _) => {
            Adt2::C11(std::rc::Rc::new(f23(v0)), std::rc::Rc::new(v2))
        }
        Adt2::C8(_, _, _) => {
            Adt2::C11(std::rc::Rc::new(f23(v0)), std::rc::Rc::new(v2))
        }
        Adt2::C9(r101) => {
            let v32 = (*r101).clone();
            Adt2::C11(std::rc::Rc::new(f24(f23(v0), f23(v32.clone()))), std::rc::Rc::new(f25(v32)))
        }
        Adt2::C10(_, _, _) => {
            Adt2::C11(std::rc::Rc::new(f23(v0)), std::rc::Rc::new(v2))
        }
        Adt2::C11(_, _) => {
            Adt2::C11(std::rc::Rc::new(f23(v0)), std::rc::Rc::new(v2))
        }
        Adt2::C12 => {
            Adt2::C11(std::rc::Rc::new(f23(v0)), std::rc::Rc::new(v2))
        }
        Adt2::C13(_, _) => {
            Adt2::C11(std::rc::Rc::new(f23(v0)), std::rc::Rc::new(v2))
        }
        Adt2::C14(_, _, _, _, _, _) => {
            Adt2::C11(std::rc::Rc::new(f23(v0)), std::rc::Rc::new(v2))
        }
        Adt2::C15(_, _, _) => {
            Adt2::C11(std::rc::Rc::new(f23(v0)), std::rc::Rc::new(v2))
        }
        Adt2::C16(_, _) => {
            Adt2::C11(std::rc::Rc::new(f23(v0)), std::rc::Rc::new(v2))
        }
    }
}

pub fn f16(v0: Adt0) -> u64 {
    let m102 = v0;
    match m102 {
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

pub fn f17(v0: Adt1) -> u64 {
    let m103 = v0;
    match m103 {
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

pub fn f18(v0: Adt0) -> Adt1 {
    let m104 = v0;
    match m104 {
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

pub fn f19(v0: Adt0) -> Adt0 {
    let m105 = f26(v0);
    match m105 {
        None => {
            Adt0::C0
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f20(v0: Adt0) -> Adt0 {
    let m106 = f27(v0);
    match m106 {
        None => {
            Adt0::C0
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f21(v0: Adt0) -> u64 {
    let m107 = v0;
    match m107 {
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

pub fn f22(_: u64) -> Adt13 {
    Adt13::C0(Adt14::C0)
}

pub fn f23(v0: Adt2) -> Adt2 {
    let m108 = v0;
    match m108 {
        Adt2::C0(_, _) => {
            Adt2::C12
        }
        Adt2::C1(_, _, _) => {
            Adt2::C12
        }
        Adt2::C2(_, _, _) => {
            Adt2::C12
        }
        Adt2::C3(_, _, _, _) => {
            Adt2::C12
        }
        Adt2::C4(_, _, _, _) => {
            Adt2::C12
        }
        Adt2::C5(_, _, _) => {
            Adt2::C12
        }
        Adt2::C6(_, _, _) => {
            Adt2::C12
        }
        Adt2::C7(_, _, _, _) => {
            Adt2::C12
        }
        Adt2::C8(_, _, _) => {
            Adt2::C12
        }
        Adt2::C9(_) => {
            Adt2::C12
        }
        Adt2::C10(_, _, _) => {
            Adt2::C12
        }
        Adt2::C11(r109, _) => {
            (*r109).clone()
        }
        Adt2::C12 => {
            Adt2::C12
        }
        Adt2::C13(_, _) => {
            Adt2::C12
        }
        Adt2::C14(_, _, _, _, _, _) => {
            Adt2::C12
        }
        Adt2::C15(_, _, _) => {
            Adt2::C12
        }
        Adt2::C16(_, _) => {
            Adt2::C12
        }
    }
}

pub fn f24(v0: Adt2, v1: Adt2) -> Adt2 {
    let m110 = v0;
    match m110 {
        Adt2::C0(_, _) => {
            v1
        }
        Adt2::C1(_, _, _) => {
            v1
        }
        Adt2::C2(_, _, _) => {
            v1
        }
        Adt2::C3(_, _, _, _) => {
            v1
        }
        Adt2::C4(_, _, _, _) => {
            v1
        }
        Adt2::C5(_, _, _) => {
            v1
        }
        Adt2::C6(_, _, _) => {
            v1
        }
        Adt2::C7(_, _, _, _) => {
            v1
        }
        Adt2::C8(_, _, _) => {
            v1
        }
        Adt2::C9(_) => {
            v1
        }
        Adt2::C10(_, _, _) => {
            v1
        }
        Adt2::C11(_, _) => {
            v1
        }
        Adt2::C12 => {
            v1
        }
        Adt2::C13(_, _) => {
            v1
        }
        Adt2::C14(v39, v40, v41, r111, v43, r112) => {
            let v42 = (*r111).clone();
            let v44 = (*r112).clone();
            Adt2::C14(v39, v40, v41, std::rc::Rc::new(v42), v43, std::rc::Rc::new(f24(v44, v1)))
        }
        Adt2::C15(_, _, _) => {
            v1
        }
        Adt2::C16(_, _) => {
            v1
        }
    }
}

pub fn f25(v0: Adt2) -> Adt2 {
    let m113 = v0;
    match m113 {
        Adt2::C0(_, _) => {
            Adt2::C12
        }
        Adt2::C1(_, _, _) => {
            Adt2::C12
        }
        Adt2::C2(_, _, _) => {
            Adt2::C12
        }
        Adt2::C3(_, _, _, _) => {
            Adt2::C12
        }
        Adt2::C4(_, _, _, _) => {
            Adt2::C12
        }
        Adt2::C5(_, _, _) => {
            Adt2::C12
        }
        Adt2::C6(_, _, _) => {
            Adt2::C12
        }
        Adt2::C7(_, _, _, _) => {
            Adt2::C12
        }
        Adt2::C8(_, _, _) => {
            Adt2::C12
        }
        Adt2::C9(_) => {
            Adt2::C12
        }
        Adt2::C10(_, _, _) => {
            Adt2::C12
        }
        Adt2::C11(_, r114) => {
            (*r114).clone()
        }
        Adt2::C12 => {
            Adt2::C12
        }
        Adt2::C13(_, _) => {
            Adt2::C12
        }
        Adt2::C14(_, _, _, _, _, _) => {
            Adt2::C12
        }
        Adt2::C15(_, _, _) => {
            Adt2::C12
        }
        Adt2::C16(_, _) => {
            Adt2::C12
        }
    }
}

pub fn f26(v0: Adt0) -> Option<Adt0> {
    let m115 = v0;
    match m115 {
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
        Adt0::C8(r116) => {
            let v2 = (*r116).clone();
            Some(v2)
        }
        Adt0::C9(r117, _) => {
            let v3 = (*r117).clone();
            Some(v3)
        }
        Adt0::C10(r118) => {
            let v5 = (*r118).clone();
            Some(v5)
        }
        Adt0::C11(r119, _) => {
            let v6 = (*r119).clone();
            Some(v6)
        }
        Adt0::C12(_) => {
            None::<Adt0>
        }
        Adt0::C13(r120, _) => {
            let v9 = (*r120).clone();
            Some(v9)
        }
        Adt0::C14 => {
            None::<Adt0>
        }
        Adt0::C15(r121, _) => {
            let v11 = (*r121).clone();
            Some(v11)
        }
    }
}

pub fn f27(v0: Adt0) -> Option<Adt0> {
    let m122 = v0;
    match m122 {
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
        Adt0::C9(_, r123) => {
            let v4 = (*r123).clone();
            Some(v4)
        }
        Adt0::C10(_) => {
            None::<Adt0>
        }
        Adt0::C11(_, r124) => {
            let v7 = (*r124).clone();
            Some(v7)
        }
        Adt0::C12(_) => {
            None::<Adt0>
        }
        Adt0::C13(_, r125) => {
            let v10 = (*r125).clone();
            Some(v10)
        }
        Adt0::C14 => {
            None::<Adt0>
        }
        Adt0::C15(_, r126) => {
            let v12 = (*r126).clone();
            Some(v12)
        }
    }
}

pub fn run(p0: Adt0, p1: List<bool>, p2: Adt2) -> R<Adt2> {
    f0(p0, p1, p2)
}
