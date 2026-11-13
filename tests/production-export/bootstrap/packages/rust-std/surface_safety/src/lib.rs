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
    C0(Adt1, u64, u64, u64, u64, u64, u64, Bytes),
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
}

#[derive(Clone)]
pub enum Fn0 {
    F2(u64),
}

impl Fn0 {
    pub fn apply(&self, p0: (u64, Option<Bytes>), p1: Adt0) -> R<(u64, Option<Bytes>)> {
        match self {
            Fn0::F2(k0) => f2(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F18(u64, Bytes),
}

impl Fn1 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn1::F18(k0, k1) => f18(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F20(Bytes),
}

impl Fn2 {
    pub fn apply(&self, p0: (u64, Option<u64>), p1: Bytes) -> R<(u64, Option<u64>)> {
        match self {
            Fn2::F20(k0) => f20(k0.clone(), p0, p1),
        }
    }
}

pub fn f0(v0: List<Adt0>, v1: u64) -> R<Option<Bytes>> {
    let (_, h1) = f1(Fn0::F2(v1), (0u64, None::<Bytes>), v0)?;
    Ok(h1)
}

pub fn f1(v0: Fn0, p5: (u64, Option<Bytes>), p6: List<Adt0>) -> R<(u64, Option<Bytes>)> {
    let m4 = (p5, p6);
    {
        let mut m4 = m4;
        loop {
            let step = {
                let (v1, v2) = m4;
                let m2 = v2;
                match m2.uncons() {
                    None => {
                        Err::<((u64, Option<Bytes>), List<Adt0>), R<(u64, Option<Bytes>)>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<((u64, Option<Bytes>), List<Adt0>), R<(u64, Option<Bytes>)>>(({
                            let c3: Fn0 = v0.clone();
                            c3.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m4 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f2(v0: u64, v1: (u64, Option<Bytes>), v2: Adt0) -> R<(u64, Option<Bytes>)> {
    let m8 = {
        let (_, h7) = v1.clone();
        h7
    };
    match m8 {
        None => {
            if f3(v2.clone(), {
                let (h9, _) = v1.clone();
                h9
            }, v0)? {
                Ok(({
                    let a11 = {
                        let (h10, _) = v1;
                        h10
                    };
                    let a12 = 1u64;
                    nat_add(a11, a12)?
                }, None::<Bytes>))
            } else {
                Ok(({
                    let a14 = {
                        let (h13, _) = v1;
                        h13
                    };
                    let a15 = 1u64;
                    nat_add(a14, a15)?
                }, Some(f4(v2, v0))))
            }
        }
        Some(_) => {
            Ok(v1)
        }
    }
}

pub fn f3(v0: Adt0, v1: u64, v2: u64) -> R<bool> {
    let a16 = v2;
    let a17 = 0u64;
    if nat_eq(a16, a17) {
        f5(v0)
    } else {
        let a18 = v2;
        let a19 = 1u64;
        if nat_eq(a18, a19) {
            f6(v0, v1)
        } else {
            if f5(v0.clone())? {
                f6(v0, v1)
            } else {
                Ok(false)
            }
        }
    }
}

pub fn f4(v0: Adt0, v1: u64) -> Bytes {
    f7(f7(f7(f7(f7(f7(f8(Str::lit("`")), match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, h20) => {
            h20
        }
    }), f8(Str::lit("` ("))), f9(v0)), f8(Str::lit(") is not renderer-safe in the "))), f10(v1)), f8(Str::lit(" channel")))
}

pub fn f5(v0: Adt0) -> R<bool> {
    let m22 = match v0.clone() {
        Adt0::C0(h21, _, _, _, _, _, _, _) => {
            h21
        }
    };
    match m22 {
        Adt1::C0 => {
            Ok(false)
        }
        Adt1::C1 => {
            Ok(true)
        }
        Adt1::C2 => {
            Ok(false)
        }
        Adt1::C3 => {
            Ok(false)
        }
        Adt1::C4 => {
            f11(f12(), match v0 {
                Adt0::C0(_, _, _, _, _, _, _, h23) => {
                    h23
                }
            })
        }
        Adt1::C5 => {
            Ok(false)
        }
        Adt1::C6 => {
            Ok(true)
        }
    }
}

pub fn f6(v0: Adt0, v1: u64) -> R<bool> {
    let m25 = match v0.clone() {
        Adt0::C0(h24, _, _, _, _, _, _, _) => {
            h24
        }
    };
    match m25 {
        Adt1::C0 => {
            Ok(false)
        }
        Adt1::C1 => {
            Ok(true)
        }
        Adt1::C2 => {
            let a26 = 0u64;
            let a27 = v1;
            Ok(nat_lt(a26, a27))
        }
        Adt1::C3 => {
            f11(f13(), match v0 {
                Adt0::C0(_, _, _, _, _, _, _, h28) => {
                    h28
                }
            })
        }
        Adt1::C4 => {
            Ok(!f11(f14(), match v0 {
                Adt0::C0(_, _, _, _, _, _, _, h29) => {
                    h29
                }
            })?)
        }
        Adt1::C5 => {
            f15(match v0 {
                Adt0::C0(_, _, _, _, _, _, _, h30) => {
                    h30
                }
            })
        }
        Adt1::C6 => {
            Ok(true)
        }
    }
}

pub fn f7(v0: Bytes, v1: Bytes) -> Bytes {
    let a31 = v0;
    let a32 = v1;
    append_bytes(a31, a32)
}

pub fn f8(v0: Str) -> Bytes {
    let a33 = v0;
    utf8_encode(a33)
}

pub fn f9(v0: Adt0) -> Bytes {
    let m35 = match v0 {
        Adt0::C0(h34, _, _, _, _, _, _, _) => {
            h34
        }
    };
    match m35 {
        Adt1::C0 => {
            f8(Str::lit("control"))
        }
        Adt1::C1 => {
            f8(Str::lit("word"))
        }
        Adt1::C2 => {
            f8(Str::lit("numeral"))
        }
        Adt1::C3 => {
            f8(Str::lit("delimiter"))
        }
        Adt1::C4 => {
            f8(Str::lit("ascii-symbol"))
        }
        Adt1::C5 => {
            f8(Str::lit("unicode-symbol"))
        }
        Adt1::C6 => {
            f8(Str::lit("whitespace"))
        }
    }
}

pub fn f10(v0: u64) -> Bytes {
    let a36 = v0;
    let a37 = 0u64;
    if nat_eq(a36, a37) {
        f8(Str::lit("text"))
    } else {
        let a38 = v0;
        let a39 = 1u64;
        if nat_eq(a38, a39) {
            f8(Str::lit("math"))
        } else {
            f8(Str::lit("text and math"))
        }
    }
}

pub fn f11(v0: List<Bytes>, v1: Bytes) -> R<bool> {
    let m40 = f16(v0, v1)?;
    match m40 {
        None => {
            Ok(false)
        }
        Some(_) => {
            Ok(true)
        }
    }
}

pub fn f12() -> List<Bytes> {
    List::cons(f8(Str::lit(":")), List::cons(f8(Str::lit("-")), List::cons(f8(Str::lit(",")), List::cons(f8(Str::lit(".")), List::cons(f8(Str::lit(";")), List::cons(f8(Str::lit("'")), List::<Bytes>::nil()))))))
}

pub fn f13() -> List<Bytes> {
    List::cons(f8(Str::lit("(")), List::cons(f8(Str::lit(")")), List::cons(f8(Str::lit("[")), List::cons(f8(Str::lit("]")), List::<Bytes>::nil()))))
}

pub fn f14() -> List<Bytes> {
    List::cons(Bytes::lit(&[0x5c]), List::cons(Bytes::lit(&[0x25]), List::cons(Bytes::lit(&[0x24]), List::cons(Bytes::lit(&[0x23]), List::cons(Bytes::lit(&[0x26]), List::cons(Bytes::lit(&[0x7e]), List::cons(Bytes::lit(&[0x5e]), List::cons(Bytes::lit(&[0x5f]), List::cons(Bytes::lit(&[0x7b]), List::cons(Bytes::lit(&[0x7d]), List::cons(Bytes::lit(&[0x22]), List::cons(Bytes::lit(&[0x60]), List::<Bytes>::nil()))))))))))))
}

pub fn f15(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a41 = v0.clone();
        length_bytes(a41)
    };
    let a45 = v1;
    let a46 = {
        let (h42, _) = f17(Fn1::F18(v1, v0), {
            let a43 = v1;
            let a44 = 1u64;
            nat_add(a43, a44)?
        }, 0u64)?;
        h42
    };
    Ok(nat_le(a45, a46))
}

pub fn f16(v0: List<Bytes>, v1: Bytes) -> R<Option<u64>> {
    let (_, h47) = f19(Fn2::F20(v1), (0u64, None::<u64>), v0)?;
    Ok(h47)
}

pub fn f17(v0: Fn1, p52: u64, p53: u64) -> R<(u64, bool)> {
    let m51 = (p52, p53);
    {
        let mut m51 = m51;
        loop {
            let step = {
                let (v1, v2) = m51;
                let m48 = v1;
                if m48 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m48 - 1;
                    let m50 = {
                        let c49: Fn1 = v0.clone();
                        c49.apply(v2)?
                    };
                    match m50 {
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
                Ok(next) => { m51 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f18(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f21(v1, v0, v2)
}

pub fn f19(v0: Fn2, p57: (u64, Option<u64>), p58: List<Bytes>) -> R<(u64, Option<u64>)> {
    let m56 = (p57, p58);
    {
        let mut m56 = m56;
        loop {
            let step = {
                let (v1, v2) = m56;
                let m54 = v2;
                match m54.uncons() {
                    None => {
                        Err::<((u64, Option<u64>), List<Bytes>), R<(u64, Option<u64>)>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<((u64, Option<u64>), List<Bytes>), R<(u64, Option<u64>)>>(({
                            let c55: Fn2 = v0.clone();
                            c55.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m56 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f20(v0: Bytes, v1: (u64, Option<u64>), v2: Bytes) -> R<(u64, Option<u64>)> {
    let m60 = {
        let (_, h59) = v1;
        h59
    };
    match m60 {
        None => {
            let a61 = v2;
            let a62 = v0;
            if equal(a61, a62) {
                Ok(({
                    let (h63, _) = v1;
                    h63
                }, Some({
                    let (h64, _) = v1;
                    h64
                })))
            } else {
                Ok(({
                    let a66 = {
                        let (h65, _) = v1;
                        h65
                    };
                    let a67 = 1u64;
                    nat_add(a66, a67)?
                }, None::<u64>))
            }
        }
        Some(_) => {
            Ok(v1)
        }
    }
}

pub fn f21(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let a68 = v2;
    let a69 = v1;
    if nat_lt(a68, a69) {
        if f22(f23(v0.clone(), v2)?) {
            Ok(Some({
                let a70 = v2;
                let a71 = f24(f25(v0, v2)?);
                nat_add(a70, a71)?
            }))
        } else {
            Ok(None::<u64>)
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f22(v0: u64) -> bool {
    let a72 = v0;
    let a73 = 172u64;
    if if if if if nat_eq(a72, a73) {
        true
    } else {
        let a74 = v0;
        let a75 = 177u64;
        nat_eq(a74, a75)
    } {
        true
    } else {
        let a76 = v0;
        let a77 = 183u64;
        if nat_eq(a76, a77) {
            true
        } else {
            let a78 = v0;
            let a79 = 215u64;
            nat_eq(a78, a79)
        }
    } {
        true
    } else {
        let a80 = v0;
        let a81 = 247u64;
        if if nat_eq(a80, a81) {
            true
        } else {
            if f26(v0, 913u64, 937u64) {
                let a82 = v0;
                let a83 = 930u64;
                !nat_eq(a82, a83)
            } else {
                false
            }
        } {
            true
        } else {
            if f26(v0, 945u64, 969u64) {
                true
            } else {
                let a84 = v0;
                let a85 = 8230u64;
                nat_eq(a84, a85)
            }
        }
    } {
        true
    } else {
        if if if f26(v0, 8242u64, 8244u64) {
            true
        } else {
            f26(v0, 8448u64, 8527u64)
        } {
            true
        } else {
            if f26(v0, 8592u64, 8703u64) {
                true
            } else {
                f26(v0, 8704u64, 8959u64)
            }
        } {
            true
        } else {
            if if f26(v0, 8968u64, 8971u64) {
                true
            } else {
                f26(v0, 10176u64, 10223u64)
            } {
                true
            } else {
                if f26(v0, 10624u64, 10751u64) {
                    true
                } else {
                    f26(v0, 10752u64, 11007u64)
                }
            }
        }
    } {
        true
    } else {
        f26(v0, 119808u64, 120831u64)
    }
}

pub fn f23(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f25(v0.clone(), v1)?;
    let v3: u64 = f24(v2);
    let a86 = v3;
    let a87 = 1u64;
    if nat_eq(a86, a87) {
        Ok(v2)
    } else {
        let a88 = v3;
        let a89 = 2u64;
        if nat_eq(a88, a89) {
            let a97 = {
                let a93 = {
                    let a90 = v2;
                    let a91 = 32u64;
                    let a92 = 0u64;
                    nat_rem(a90, a91, a92)
                };
                let a94 = 64u64;
                nat_mul(a93, a94)?
            };
            let a98 = f27(v0, {
                let a95 = v1;
                let a96 = 1u64;
                nat_add(a95, a96)?
            })?;
            nat_add(a97, a98)
        } else {
            let a99 = v3;
            let a100 = 3u64;
            if nat_eq(a99, a100) {
                let a114 = {
                    let a110 = {
                        let a104 = {
                            let a101 = v2;
                            let a102 = 16u64;
                            let a103 = 0u64;
                            nat_rem(a101, a102, a103)
                        };
                        let a105 = 4096u64;
                        nat_mul(a104, a105)?
                    };
                    let a111 = {
                        let a108 = f27(v0.clone(), {
                            let a106 = v1;
                            let a107 = 1u64;
                            nat_add(a106, a107)?
                        })?;
                        let a109 = 64u64;
                        nat_mul(a108, a109)?
                    };
                    nat_add(a110, a111)?
                };
                let a115 = f27(v0, {
                    let a112 = v1;
                    let a113 = 2u64;
                    nat_add(a112, a113)?
                })?;
                nat_add(a114, a115)
            } else {
                let a135 = {
                    let a131 = {
                        let a125 = {
                            let a119 = {
                                let a116 = v2;
                                let a117 = 8u64;
                                let a118 = 0u64;
                                nat_rem(a116, a117, a118)
                            };
                            let a120 = 262144u64;
                            nat_mul(a119, a120)?
                        };
                        let a126 = {
                            let a123 = f27(v0.clone(), {
                                let a121 = v1;
                                let a122 = 1u64;
                                nat_add(a121, a122)?
                            })?;
                            let a124 = 4096u64;
                            nat_mul(a123, a124)?
                        };
                        nat_add(a125, a126)?
                    };
                    let a132 = {
                        let a129 = f27(v0.clone(), {
                            let a127 = v1;
                            let a128 = 2u64;
                            nat_add(a127, a128)?
                        })?;
                        let a130 = 64u64;
                        nat_mul(a129, a130)?
                    };
                    nat_add(a131, a132)?
                };
                let a136 = f27(v0, {
                    let a133 = v1;
                    let a134 = 3u64;
                    nat_add(a133, a134)?
                })?;
                nat_add(a135, a136)
            }
        }
    }
}

pub fn f24(v0: u64) -> u64 {
    let a137 = v0;
    let a138 = 128u64;
    if nat_lt(a137, a138) {
        1u64
    } else {
        let a139 = v0;
        let a140 = 224u64;
        if nat_lt(a139, a140) {
            2u64
        } else {
            let a141 = v0;
            let a142 = 240u64;
            if nat_lt(a141, a142) {
                3u64
            } else {
                4u64
            }
        }
    }
}

pub fn f25(v0: Bytes, v1: u64) -> R<u64> {
    let m143 = f28(v0, v1);
    match m143 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a188 = {
                let a164 = {
                    let a152 = {
                        let a146 = {
                            let a144 = v3;
                            let a145 = 128u8;
                            fixed_u8::bit_and(a144, a145)
                        };
                        let a147 = 128u8;
                        if equal(a146, a147) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a153 = {
                        let a150 = {
                            let a148 = v3;
                            let a149 = 64u8;
                            fixed_u8::bit_and(a148, a149)
                        };
                        let a151 = 64u8;
                        if equal(a150, a151) {
                            64u64
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
                            let a155 = 32u8;
                            fixed_u8::bit_and(a154, a155)
                        };
                        let a157 = 32u8;
                        if equal(a156, a157) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a163 = {
                        let a160 = {
                            let a158 = v3;
                            let a159 = 16u8;
                            fixed_u8::bit_and(a158, a159)
                        };
                        let a161 = 16u8;
                        if equal(a160, a161) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a162, a163)?
                };
                nat_add(a164, a165)?
            };
            let a189 = {
                let a186 = {
                    let a174 = {
                        let a168 = {
                            let a166 = v3;
                            let a167 = 8u8;
                            fixed_u8::bit_and(a166, a167)
                        };
                        let a169 = 8u8;
                        if equal(a168, a169) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a175 = {
                        let a172 = {
                            let a170 = v3;
                            let a171 = 4u8;
                            fixed_u8::bit_and(a170, a171)
                        };
                        let a173 = 4u8;
                        if equal(a172, a173) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a174, a175)?
                };
                let a187 = {
                    let a184 = {
                        let a178 = {
                            let a176 = v3;
                            let a177 = 2u8;
                            fixed_u8::bit_and(a176, a177)
                        };
                        let a179 = 2u8;
                        if equal(a178, a179) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a185 = {
                        let a182 = {
                            let a180 = v3;
                            let a181 = 1u8;
                            fixed_u8::bit_and(a180, a181)
                        };
                        let a183 = 1u8;
                        if equal(a182, a183) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a184, a185)?
                };
                nat_add(a186, a187)?
            };
            nat_add(a188, a189)
        }
    }
}

pub fn f26(v0: u64, v1: u64, v2: u64) -> bool {
    let a190 = v0;
    let a191 = v1;
    if nat_lt(a190, a191) {
        false
    } else {
        let a192 = v0;
        let a193 = v2;
        nat_le(a192, a193)
    }
}

pub fn f27(v0: Bytes, v1: u64) -> R<u64> {
    let a194 = f25(v0, v1)?;
    let a195 = 64u64;
    let a196 = 0u64;
    Ok(nat_rem(a194, a195, a196))
}

pub fn f28(v0: Bytes, v1: u64) -> Option<u8> {
    let a197 = v0;
    let a198 = v1;
    index_bytes(a197, a198)
}

pub fn run(p0: List<Adt0>, p1: u64) -> R<Option<Bytes>> {
    f0(p0, p1)
}
