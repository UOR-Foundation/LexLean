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
    C0(u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt2 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32, Adt1, u64),
}

#[derive(Clone)]
pub enum Fn0 {
    F4,
}

impl Fn0 {
    pub fn apply(&self, p0: Bytes, p1: (Bytes, Bytes)) -> Bytes {
        match self {
            Fn0::F4 => f4(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F11(Bytes, u64, u64),
}

impl Fn1 {
    pub fn apply(&self, p0: (u64, Adt0)) -> R<(u64, Adt0)> {
        match self {
            Fn1::F11(k0, k1, k2) => f11(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F19,
}

impl Fn2 {
    pub fn apply(&self, p0: (u64, Bytes)) -> (u64, Bytes) {
        match self {
            Fn2::F19 => f19(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F23,
}

impl Fn3 {
    pub fn apply(&self, p0: Adt2) -> R<Adt2> {
        match self {
            Fn3::F23 => f23(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F39,
}

impl Fn4 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn4::F39 => f39(p0),
        }
    }
}

pub fn f0(v0: List<(Bytes, Bytes)>) -> R<Bytes> {
    f1(f2(Str::lit("lexlean-tree-v1")), f3(Fn0::F4, Bytes::lit(&[]), v0))
}

pub fn f1(v0: Bytes, v1: Bytes) -> R<Bytes> {
    f5(f6(f7(f7(v0, Bytes::lit(&[0x00])), v1))?)
}

pub fn f2(v0: Str) -> Bytes {
    let a1 = v0;
    utf8_encode(a1)
}

pub fn f3(v0: Fn0, p5: Bytes, p6: List<(Bytes, Bytes)>) -> Bytes {
    let m4 = (p5, p6);
    {
        let mut m4 = m4;
        loop {
            let step = {
                let (v1, v2) = m4;
                let m2 = v2;
                match m2.uncons() {
                    None => {
                        Err::<(Bytes, List<(Bytes, Bytes)>), Bytes>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Bytes, List<(Bytes, Bytes)>), Bytes>(({
                            let c3: Fn0 = v0.clone();
                            c3.apply(v1, v3)
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

pub fn f4(v0: Bytes, v1: (Bytes, Bytes)) -> Bytes {
    f7(v0, f8({
        let (h7, _) = v1;
        h7
    }, {
        let (_, h8) = v1;
        h8
    }))
}

pub fn f5(v0: Adt0) -> R<Bytes> {
    Ok(f7(f7(f7(f7(f7(f7(f7(f9(match v0.clone() {
        Adt0::C0(h9, _, _, _, _, _, _, _) => {
            h9
        }
    })?, f9(match v0.clone() {
        Adt0::C0(_, h10, _, _, _, _, _, _) => {
            h10
        }
    })?), f9(match v0.clone() {
        Adt0::C0(_, _, h11, _, _, _, _, _) => {
            h11
        }
    })?), f9(match v0.clone() {
        Adt0::C0(_, _, _, h12, _, _, _, _) => {
            h12
        }
    })?), f9(match v0.clone() {
        Adt0::C0(_, _, _, _, h13, _, _, _) => {
            h13
        }
    })?), f9(match v0.clone() {
        Adt0::C0(_, _, _, _, _, h14, _, _) => {
            h14
        }
    })?), f9(match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, h15, _) => {
            h15
        }
    })?), f9(match v0 {
        Adt0::C0(_, _, _, _, _, _, _, h16) => {
            h16
        }
    })?))
}

pub fn f6(v0: Bytes) -> R<Adt0> {
    let v1: u64 = {
        let a17 = v0.clone();
        length_bytes(a17)
    };
    let v2: u64 = {
        let a23 = {
            let a20 = {
                let a18 = v1;
                let a19 = 72u64;
                nat_add(a18, a19)?
            };
            let a21 = 64u64;
            let a22 = 0u64;
            nat_quot(a20, a21, a22)
        };
        let a24 = 64u64;
        nat_mul(a23, a24)?
    };
    let (_, h25) = f10(Fn1::F11(v0, v1, v2), {
        let a26 = v2;
        let a27 = 64u64;
        let a28 = 0u64;
        nat_quot(a26, a27, a28)
    }, (0u64, f12()))?;
    Ok(h25)
}

pub fn f7(v0: Bytes, v1: Bytes) -> Bytes {
    let a29 = v0;
    let a30 = v1;
    append_bytes(a29, a30)
}

pub fn f8(v0: Bytes, v1: Bytes) -> Bytes {
    f7(f7(f7(f13({
        let a31 = v0.clone();
        length_bytes(a31)
    }, 4u64), v0), f13({
        let a32 = v1.clone();
        length_bytes(a32)
    }, 8u64)), v1)
}

pub fn f9(v0: u32) -> R<Bytes> {
    Ok(f7(f7(f7(f14({
        let v1: u32 = f15(v0, 24u32);
        let a221 = {
            let a125 = {
                let a77 = {
                    let a53 = {
                        let a41 = {
                            let a35 = {
                                let a33 = v1;
                                let a34 = 2147483648u32;
                                fixed_u32::bit_and(a33, a34)
                            };
                            let a36 = 2147483648u32;
                            if equal(a35, a36) {
                                2147483648u64
                            } else {
                                0u64
                            }
                        };
                        let a42 = {
                            let a39 = {
                                let a37 = v1;
                                let a38 = 1073741824u32;
                                fixed_u32::bit_and(a37, a38)
                            };
                            let a40 = 1073741824u32;
                            if equal(a39, a40) {
                                1073741824u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a41, a42)?
                    };
                    let a54 = {
                        let a51 = {
                            let a45 = {
                                let a43 = v1;
                                let a44 = 536870912u32;
                                fixed_u32::bit_and(a43, a44)
                            };
                            let a46 = 536870912u32;
                            if equal(a45, a46) {
                                536870912u64
                            } else {
                                0u64
                            }
                        };
                        let a52 = {
                            let a49 = {
                                let a47 = v1;
                                let a48 = 268435456u32;
                                fixed_u32::bit_and(a47, a48)
                            };
                            let a50 = 268435456u32;
                            if equal(a49, a50) {
                                268435456u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a51, a52)?
                    };
                    nat_add(a53, a54)?
                };
                let a78 = {
                    let a75 = {
                        let a63 = {
                            let a57 = {
                                let a55 = v1;
                                let a56 = 134217728u32;
                                fixed_u32::bit_and(a55, a56)
                            };
                            let a58 = 134217728u32;
                            if equal(a57, a58) {
                                134217728u64
                            } else {
                                0u64
                            }
                        };
                        let a64 = {
                            let a61 = {
                                let a59 = v1;
                                let a60 = 67108864u32;
                                fixed_u32::bit_and(a59, a60)
                            };
                            let a62 = 67108864u32;
                            if equal(a61, a62) {
                                67108864u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a63, a64)?
                    };
                    let a76 = {
                        let a73 = {
                            let a67 = {
                                let a65 = v1;
                                let a66 = 33554432u32;
                                fixed_u32::bit_and(a65, a66)
                            };
                            let a68 = 33554432u32;
                            if equal(a67, a68) {
                                33554432u64
                            } else {
                                0u64
                            }
                        };
                        let a74 = {
                            let a71 = {
                                let a69 = v1;
                                let a70 = 16777216u32;
                                fixed_u32::bit_and(a69, a70)
                            };
                            let a72 = 16777216u32;
                            if equal(a71, a72) {
                                16777216u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a73, a74)?
                    };
                    nat_add(a75, a76)?
                };
                nat_add(a77, a78)?
            };
            let a126 = {
                let a123 = {
                    let a99 = {
                        let a87 = {
                            let a81 = {
                                let a79 = v1;
                                let a80 = 8388608u32;
                                fixed_u32::bit_and(a79, a80)
                            };
                            let a82 = 8388608u32;
                            if equal(a81, a82) {
                                8388608u64
                            } else {
                                0u64
                            }
                        };
                        let a88 = {
                            let a85 = {
                                let a83 = v1;
                                let a84 = 4194304u32;
                                fixed_u32::bit_and(a83, a84)
                            };
                            let a86 = 4194304u32;
                            if equal(a85, a86) {
                                4194304u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a87, a88)?
                    };
                    let a100 = {
                        let a97 = {
                            let a91 = {
                                let a89 = v1;
                                let a90 = 2097152u32;
                                fixed_u32::bit_and(a89, a90)
                            };
                            let a92 = 2097152u32;
                            if equal(a91, a92) {
                                2097152u64
                            } else {
                                0u64
                            }
                        };
                        let a98 = {
                            let a95 = {
                                let a93 = v1;
                                let a94 = 1048576u32;
                                fixed_u32::bit_and(a93, a94)
                            };
                            let a96 = 1048576u32;
                            if equal(a95, a96) {
                                1048576u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a97, a98)?
                    };
                    nat_add(a99, a100)?
                };
                let a124 = {
                    let a121 = {
                        let a109 = {
                            let a103 = {
                                let a101 = v1;
                                let a102 = 524288u32;
                                fixed_u32::bit_and(a101, a102)
                            };
                            let a104 = 524288u32;
                            if equal(a103, a104) {
                                524288u64
                            } else {
                                0u64
                            }
                        };
                        let a110 = {
                            let a107 = {
                                let a105 = v1;
                                let a106 = 262144u32;
                                fixed_u32::bit_and(a105, a106)
                            };
                            let a108 = 262144u32;
                            if equal(a107, a108) {
                                262144u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a109, a110)?
                    };
                    let a122 = {
                        let a119 = {
                            let a113 = {
                                let a111 = v1;
                                let a112 = 131072u32;
                                fixed_u32::bit_and(a111, a112)
                            };
                            let a114 = 131072u32;
                            if equal(a113, a114) {
                                131072u64
                            } else {
                                0u64
                            }
                        };
                        let a120 = {
                            let a117 = {
                                let a115 = v1;
                                let a116 = 65536u32;
                                fixed_u32::bit_and(a115, a116)
                            };
                            let a118 = 65536u32;
                            if equal(a117, a118) {
                                65536u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a119, a120)?
                    };
                    nat_add(a121, a122)?
                };
                nat_add(a123, a124)?
            };
            nat_add(a125, a126)?
        };
        let a222 = {
            let a219 = {
                let a171 = {
                    let a147 = {
                        let a135 = {
                            let a129 = {
                                let a127 = v1;
                                let a128 = 32768u32;
                                fixed_u32::bit_and(a127, a128)
                            };
                            let a130 = 32768u32;
                            if equal(a129, a130) {
                                32768u64
                            } else {
                                0u64
                            }
                        };
                        let a136 = {
                            let a133 = {
                                let a131 = v1;
                                let a132 = 16384u32;
                                fixed_u32::bit_and(a131, a132)
                            };
                            let a134 = 16384u32;
                            if equal(a133, a134) {
                                16384u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a135, a136)?
                    };
                    let a148 = {
                        let a145 = {
                            let a139 = {
                                let a137 = v1;
                                let a138 = 8192u32;
                                fixed_u32::bit_and(a137, a138)
                            };
                            let a140 = 8192u32;
                            if equal(a139, a140) {
                                8192u64
                            } else {
                                0u64
                            }
                        };
                        let a146 = {
                            let a143 = {
                                let a141 = v1;
                                let a142 = 4096u32;
                                fixed_u32::bit_and(a141, a142)
                            };
                            let a144 = 4096u32;
                            if equal(a143, a144) {
                                4096u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a145, a146)?
                    };
                    nat_add(a147, a148)?
                };
                let a172 = {
                    let a169 = {
                        let a157 = {
                            let a151 = {
                                let a149 = v1;
                                let a150 = 2048u32;
                                fixed_u32::bit_and(a149, a150)
                            };
                            let a152 = 2048u32;
                            if equal(a151, a152) {
                                2048u64
                            } else {
                                0u64
                            }
                        };
                        let a158 = {
                            let a155 = {
                                let a153 = v1;
                                let a154 = 1024u32;
                                fixed_u32::bit_and(a153, a154)
                            };
                            let a156 = 1024u32;
                            if equal(a155, a156) {
                                1024u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a157, a158)?
                    };
                    let a170 = {
                        let a167 = {
                            let a161 = {
                                let a159 = v1;
                                let a160 = 512u32;
                                fixed_u32::bit_and(a159, a160)
                            };
                            let a162 = 512u32;
                            if equal(a161, a162) {
                                512u64
                            } else {
                                0u64
                            }
                        };
                        let a168 = {
                            let a165 = {
                                let a163 = v1;
                                let a164 = 256u32;
                                fixed_u32::bit_and(a163, a164)
                            };
                            let a166 = 256u32;
                            if equal(a165, a166) {
                                256u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a167, a168)?
                    };
                    nat_add(a169, a170)?
                };
                nat_add(a171, a172)?
            };
            let a220 = {
                let a217 = {
                    let a193 = {
                        let a181 = {
                            let a175 = {
                                let a173 = v1;
                                let a174 = 128u32;
                                fixed_u32::bit_and(a173, a174)
                            };
                            let a176 = 128u32;
                            if equal(a175, a176) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a182 = {
                            let a179 = {
                                let a177 = v1;
                                let a178 = 64u32;
                                fixed_u32::bit_and(a177, a178)
                            };
                            let a180 = 64u32;
                            if equal(a179, a180) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a181, a182)?
                    };
                    let a194 = {
                        let a191 = {
                            let a185 = {
                                let a183 = v1;
                                let a184 = 32u32;
                                fixed_u32::bit_and(a183, a184)
                            };
                            let a186 = 32u32;
                            if equal(a185, a186) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a192 = {
                            let a189 = {
                                let a187 = v1;
                                let a188 = 16u32;
                                fixed_u32::bit_and(a187, a188)
                            };
                            let a190 = 16u32;
                            if equal(a189, a190) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a191, a192)?
                    };
                    nat_add(a193, a194)?
                };
                let a218 = {
                    let a215 = {
                        let a203 = {
                            let a197 = {
                                let a195 = v1;
                                let a196 = 8u32;
                                fixed_u32::bit_and(a195, a196)
                            };
                            let a198 = 8u32;
                            if equal(a197, a198) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a204 = {
                            let a201 = {
                                let a199 = v1;
                                let a200 = 4u32;
                                fixed_u32::bit_and(a199, a200)
                            };
                            let a202 = 4u32;
                            if equal(a201, a202) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a203, a204)?
                    };
                    let a216 = {
                        let a213 = {
                            let a207 = {
                                let a205 = v1;
                                let a206 = 2u32;
                                fixed_u32::bit_and(a205, a206)
                            };
                            let a208 = 2u32;
                            if equal(a207, a208) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a214 = {
                            let a211 = {
                                let a209 = v1;
                                let a210 = 1u32;
                                fixed_u32::bit_and(a209, a210)
                            };
                            let a212 = 1u32;
                            if equal(a211, a212) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a213, a214)?
                    };
                    nat_add(a215, a216)?
                };
                nat_add(a217, a218)?
            };
            nat_add(a219, a220)?
        };
        nat_add(a221, a222)?
    }), f14({
        let v2: u32 = {
            let a223 = f15(v0, 16u32);
            let a224 = 255u32;
            fixed_u32::bit_and(a223, a224)
        };
        let a413 = {
            let a317 = {
                let a269 = {
                    let a245 = {
                        let a233 = {
                            let a227 = {
                                let a225 = v2;
                                let a226 = 2147483648u32;
                                fixed_u32::bit_and(a225, a226)
                            };
                            let a228 = 2147483648u32;
                            if equal(a227, a228) {
                                2147483648u64
                            } else {
                                0u64
                            }
                        };
                        let a234 = {
                            let a231 = {
                                let a229 = v2;
                                let a230 = 1073741824u32;
                                fixed_u32::bit_and(a229, a230)
                            };
                            let a232 = 1073741824u32;
                            if equal(a231, a232) {
                                1073741824u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a233, a234)?
                    };
                    let a246 = {
                        let a243 = {
                            let a237 = {
                                let a235 = v2;
                                let a236 = 536870912u32;
                                fixed_u32::bit_and(a235, a236)
                            };
                            let a238 = 536870912u32;
                            if equal(a237, a238) {
                                536870912u64
                            } else {
                                0u64
                            }
                        };
                        let a244 = {
                            let a241 = {
                                let a239 = v2;
                                let a240 = 268435456u32;
                                fixed_u32::bit_and(a239, a240)
                            };
                            let a242 = 268435456u32;
                            if equal(a241, a242) {
                                268435456u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a243, a244)?
                    };
                    nat_add(a245, a246)?
                };
                let a270 = {
                    let a267 = {
                        let a255 = {
                            let a249 = {
                                let a247 = v2;
                                let a248 = 134217728u32;
                                fixed_u32::bit_and(a247, a248)
                            };
                            let a250 = 134217728u32;
                            if equal(a249, a250) {
                                134217728u64
                            } else {
                                0u64
                            }
                        };
                        let a256 = {
                            let a253 = {
                                let a251 = v2;
                                let a252 = 67108864u32;
                                fixed_u32::bit_and(a251, a252)
                            };
                            let a254 = 67108864u32;
                            if equal(a253, a254) {
                                67108864u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a255, a256)?
                    };
                    let a268 = {
                        let a265 = {
                            let a259 = {
                                let a257 = v2;
                                let a258 = 33554432u32;
                                fixed_u32::bit_and(a257, a258)
                            };
                            let a260 = 33554432u32;
                            if equal(a259, a260) {
                                33554432u64
                            } else {
                                0u64
                            }
                        };
                        let a266 = {
                            let a263 = {
                                let a261 = v2;
                                let a262 = 16777216u32;
                                fixed_u32::bit_and(a261, a262)
                            };
                            let a264 = 16777216u32;
                            if equal(a263, a264) {
                                16777216u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a265, a266)?
                    };
                    nat_add(a267, a268)?
                };
                nat_add(a269, a270)?
            };
            let a318 = {
                let a315 = {
                    let a291 = {
                        let a279 = {
                            let a273 = {
                                let a271 = v2;
                                let a272 = 8388608u32;
                                fixed_u32::bit_and(a271, a272)
                            };
                            let a274 = 8388608u32;
                            if equal(a273, a274) {
                                8388608u64
                            } else {
                                0u64
                            }
                        };
                        let a280 = {
                            let a277 = {
                                let a275 = v2;
                                let a276 = 4194304u32;
                                fixed_u32::bit_and(a275, a276)
                            };
                            let a278 = 4194304u32;
                            if equal(a277, a278) {
                                4194304u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a279, a280)?
                    };
                    let a292 = {
                        let a289 = {
                            let a283 = {
                                let a281 = v2;
                                let a282 = 2097152u32;
                                fixed_u32::bit_and(a281, a282)
                            };
                            let a284 = 2097152u32;
                            if equal(a283, a284) {
                                2097152u64
                            } else {
                                0u64
                            }
                        };
                        let a290 = {
                            let a287 = {
                                let a285 = v2;
                                let a286 = 1048576u32;
                                fixed_u32::bit_and(a285, a286)
                            };
                            let a288 = 1048576u32;
                            if equal(a287, a288) {
                                1048576u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a289, a290)?
                    };
                    nat_add(a291, a292)?
                };
                let a316 = {
                    let a313 = {
                        let a301 = {
                            let a295 = {
                                let a293 = v2;
                                let a294 = 524288u32;
                                fixed_u32::bit_and(a293, a294)
                            };
                            let a296 = 524288u32;
                            if equal(a295, a296) {
                                524288u64
                            } else {
                                0u64
                            }
                        };
                        let a302 = {
                            let a299 = {
                                let a297 = v2;
                                let a298 = 262144u32;
                                fixed_u32::bit_and(a297, a298)
                            };
                            let a300 = 262144u32;
                            if equal(a299, a300) {
                                262144u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a301, a302)?
                    };
                    let a314 = {
                        let a311 = {
                            let a305 = {
                                let a303 = v2;
                                let a304 = 131072u32;
                                fixed_u32::bit_and(a303, a304)
                            };
                            let a306 = 131072u32;
                            if equal(a305, a306) {
                                131072u64
                            } else {
                                0u64
                            }
                        };
                        let a312 = {
                            let a309 = {
                                let a307 = v2;
                                let a308 = 65536u32;
                                fixed_u32::bit_and(a307, a308)
                            };
                            let a310 = 65536u32;
                            if equal(a309, a310) {
                                65536u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a311, a312)?
                    };
                    nat_add(a313, a314)?
                };
                nat_add(a315, a316)?
            };
            nat_add(a317, a318)?
        };
        let a414 = {
            let a411 = {
                let a363 = {
                    let a339 = {
                        let a327 = {
                            let a321 = {
                                let a319 = v2;
                                let a320 = 32768u32;
                                fixed_u32::bit_and(a319, a320)
                            };
                            let a322 = 32768u32;
                            if equal(a321, a322) {
                                32768u64
                            } else {
                                0u64
                            }
                        };
                        let a328 = {
                            let a325 = {
                                let a323 = v2;
                                let a324 = 16384u32;
                                fixed_u32::bit_and(a323, a324)
                            };
                            let a326 = 16384u32;
                            if equal(a325, a326) {
                                16384u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a327, a328)?
                    };
                    let a340 = {
                        let a337 = {
                            let a331 = {
                                let a329 = v2;
                                let a330 = 8192u32;
                                fixed_u32::bit_and(a329, a330)
                            };
                            let a332 = 8192u32;
                            if equal(a331, a332) {
                                8192u64
                            } else {
                                0u64
                            }
                        };
                        let a338 = {
                            let a335 = {
                                let a333 = v2;
                                let a334 = 4096u32;
                                fixed_u32::bit_and(a333, a334)
                            };
                            let a336 = 4096u32;
                            if equal(a335, a336) {
                                4096u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a337, a338)?
                    };
                    nat_add(a339, a340)?
                };
                let a364 = {
                    let a361 = {
                        let a349 = {
                            let a343 = {
                                let a341 = v2;
                                let a342 = 2048u32;
                                fixed_u32::bit_and(a341, a342)
                            };
                            let a344 = 2048u32;
                            if equal(a343, a344) {
                                2048u64
                            } else {
                                0u64
                            }
                        };
                        let a350 = {
                            let a347 = {
                                let a345 = v2;
                                let a346 = 1024u32;
                                fixed_u32::bit_and(a345, a346)
                            };
                            let a348 = 1024u32;
                            if equal(a347, a348) {
                                1024u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a349, a350)?
                    };
                    let a362 = {
                        let a359 = {
                            let a353 = {
                                let a351 = v2;
                                let a352 = 512u32;
                                fixed_u32::bit_and(a351, a352)
                            };
                            let a354 = 512u32;
                            if equal(a353, a354) {
                                512u64
                            } else {
                                0u64
                            }
                        };
                        let a360 = {
                            let a357 = {
                                let a355 = v2;
                                let a356 = 256u32;
                                fixed_u32::bit_and(a355, a356)
                            };
                            let a358 = 256u32;
                            if equal(a357, a358) {
                                256u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a359, a360)?
                    };
                    nat_add(a361, a362)?
                };
                nat_add(a363, a364)?
            };
            let a412 = {
                let a409 = {
                    let a385 = {
                        let a373 = {
                            let a367 = {
                                let a365 = v2;
                                let a366 = 128u32;
                                fixed_u32::bit_and(a365, a366)
                            };
                            let a368 = 128u32;
                            if equal(a367, a368) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a374 = {
                            let a371 = {
                                let a369 = v2;
                                let a370 = 64u32;
                                fixed_u32::bit_and(a369, a370)
                            };
                            let a372 = 64u32;
                            if equal(a371, a372) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a373, a374)?
                    };
                    let a386 = {
                        let a383 = {
                            let a377 = {
                                let a375 = v2;
                                let a376 = 32u32;
                                fixed_u32::bit_and(a375, a376)
                            };
                            let a378 = 32u32;
                            if equal(a377, a378) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a384 = {
                            let a381 = {
                                let a379 = v2;
                                let a380 = 16u32;
                                fixed_u32::bit_and(a379, a380)
                            };
                            let a382 = 16u32;
                            if equal(a381, a382) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a383, a384)?
                    };
                    nat_add(a385, a386)?
                };
                let a410 = {
                    let a407 = {
                        let a395 = {
                            let a389 = {
                                let a387 = v2;
                                let a388 = 8u32;
                                fixed_u32::bit_and(a387, a388)
                            };
                            let a390 = 8u32;
                            if equal(a389, a390) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a396 = {
                            let a393 = {
                                let a391 = v2;
                                let a392 = 4u32;
                                fixed_u32::bit_and(a391, a392)
                            };
                            let a394 = 4u32;
                            if equal(a393, a394) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a395, a396)?
                    };
                    let a408 = {
                        let a405 = {
                            let a399 = {
                                let a397 = v2;
                                let a398 = 2u32;
                                fixed_u32::bit_and(a397, a398)
                            };
                            let a400 = 2u32;
                            if equal(a399, a400) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a406 = {
                            let a403 = {
                                let a401 = v2;
                                let a402 = 1u32;
                                fixed_u32::bit_and(a401, a402)
                            };
                            let a404 = 1u32;
                            if equal(a403, a404) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a405, a406)?
                    };
                    nat_add(a407, a408)?
                };
                nat_add(a409, a410)?
            };
            nat_add(a411, a412)?
        };
        nat_add(a413, a414)?
    })), f14({
        let v3: u32 = {
            let a415 = f15(v0, 8u32);
            let a416 = 255u32;
            fixed_u32::bit_and(a415, a416)
        };
        let a605 = {
            let a509 = {
                let a461 = {
                    let a437 = {
                        let a425 = {
                            let a419 = {
                                let a417 = v3;
                                let a418 = 2147483648u32;
                                fixed_u32::bit_and(a417, a418)
                            };
                            let a420 = 2147483648u32;
                            if equal(a419, a420) {
                                2147483648u64
                            } else {
                                0u64
                            }
                        };
                        let a426 = {
                            let a423 = {
                                let a421 = v3;
                                let a422 = 1073741824u32;
                                fixed_u32::bit_and(a421, a422)
                            };
                            let a424 = 1073741824u32;
                            if equal(a423, a424) {
                                1073741824u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a425, a426)?
                    };
                    let a438 = {
                        let a435 = {
                            let a429 = {
                                let a427 = v3;
                                let a428 = 536870912u32;
                                fixed_u32::bit_and(a427, a428)
                            };
                            let a430 = 536870912u32;
                            if equal(a429, a430) {
                                536870912u64
                            } else {
                                0u64
                            }
                        };
                        let a436 = {
                            let a433 = {
                                let a431 = v3;
                                let a432 = 268435456u32;
                                fixed_u32::bit_and(a431, a432)
                            };
                            let a434 = 268435456u32;
                            if equal(a433, a434) {
                                268435456u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a435, a436)?
                    };
                    nat_add(a437, a438)?
                };
                let a462 = {
                    let a459 = {
                        let a447 = {
                            let a441 = {
                                let a439 = v3;
                                let a440 = 134217728u32;
                                fixed_u32::bit_and(a439, a440)
                            };
                            let a442 = 134217728u32;
                            if equal(a441, a442) {
                                134217728u64
                            } else {
                                0u64
                            }
                        };
                        let a448 = {
                            let a445 = {
                                let a443 = v3;
                                let a444 = 67108864u32;
                                fixed_u32::bit_and(a443, a444)
                            };
                            let a446 = 67108864u32;
                            if equal(a445, a446) {
                                67108864u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a447, a448)?
                    };
                    let a460 = {
                        let a457 = {
                            let a451 = {
                                let a449 = v3;
                                let a450 = 33554432u32;
                                fixed_u32::bit_and(a449, a450)
                            };
                            let a452 = 33554432u32;
                            if equal(a451, a452) {
                                33554432u64
                            } else {
                                0u64
                            }
                        };
                        let a458 = {
                            let a455 = {
                                let a453 = v3;
                                let a454 = 16777216u32;
                                fixed_u32::bit_and(a453, a454)
                            };
                            let a456 = 16777216u32;
                            if equal(a455, a456) {
                                16777216u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a457, a458)?
                    };
                    nat_add(a459, a460)?
                };
                nat_add(a461, a462)?
            };
            let a510 = {
                let a507 = {
                    let a483 = {
                        let a471 = {
                            let a465 = {
                                let a463 = v3;
                                let a464 = 8388608u32;
                                fixed_u32::bit_and(a463, a464)
                            };
                            let a466 = 8388608u32;
                            if equal(a465, a466) {
                                8388608u64
                            } else {
                                0u64
                            }
                        };
                        let a472 = {
                            let a469 = {
                                let a467 = v3;
                                let a468 = 4194304u32;
                                fixed_u32::bit_and(a467, a468)
                            };
                            let a470 = 4194304u32;
                            if equal(a469, a470) {
                                4194304u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a471, a472)?
                    };
                    let a484 = {
                        let a481 = {
                            let a475 = {
                                let a473 = v3;
                                let a474 = 2097152u32;
                                fixed_u32::bit_and(a473, a474)
                            };
                            let a476 = 2097152u32;
                            if equal(a475, a476) {
                                2097152u64
                            } else {
                                0u64
                            }
                        };
                        let a482 = {
                            let a479 = {
                                let a477 = v3;
                                let a478 = 1048576u32;
                                fixed_u32::bit_and(a477, a478)
                            };
                            let a480 = 1048576u32;
                            if equal(a479, a480) {
                                1048576u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a481, a482)?
                    };
                    nat_add(a483, a484)?
                };
                let a508 = {
                    let a505 = {
                        let a493 = {
                            let a487 = {
                                let a485 = v3;
                                let a486 = 524288u32;
                                fixed_u32::bit_and(a485, a486)
                            };
                            let a488 = 524288u32;
                            if equal(a487, a488) {
                                524288u64
                            } else {
                                0u64
                            }
                        };
                        let a494 = {
                            let a491 = {
                                let a489 = v3;
                                let a490 = 262144u32;
                                fixed_u32::bit_and(a489, a490)
                            };
                            let a492 = 262144u32;
                            if equal(a491, a492) {
                                262144u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a493, a494)?
                    };
                    let a506 = {
                        let a503 = {
                            let a497 = {
                                let a495 = v3;
                                let a496 = 131072u32;
                                fixed_u32::bit_and(a495, a496)
                            };
                            let a498 = 131072u32;
                            if equal(a497, a498) {
                                131072u64
                            } else {
                                0u64
                            }
                        };
                        let a504 = {
                            let a501 = {
                                let a499 = v3;
                                let a500 = 65536u32;
                                fixed_u32::bit_and(a499, a500)
                            };
                            let a502 = 65536u32;
                            if equal(a501, a502) {
                                65536u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a503, a504)?
                    };
                    nat_add(a505, a506)?
                };
                nat_add(a507, a508)?
            };
            nat_add(a509, a510)?
        };
        let a606 = {
            let a603 = {
                let a555 = {
                    let a531 = {
                        let a519 = {
                            let a513 = {
                                let a511 = v3;
                                let a512 = 32768u32;
                                fixed_u32::bit_and(a511, a512)
                            };
                            let a514 = 32768u32;
                            if equal(a513, a514) {
                                32768u64
                            } else {
                                0u64
                            }
                        };
                        let a520 = {
                            let a517 = {
                                let a515 = v3;
                                let a516 = 16384u32;
                                fixed_u32::bit_and(a515, a516)
                            };
                            let a518 = 16384u32;
                            if equal(a517, a518) {
                                16384u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a519, a520)?
                    };
                    let a532 = {
                        let a529 = {
                            let a523 = {
                                let a521 = v3;
                                let a522 = 8192u32;
                                fixed_u32::bit_and(a521, a522)
                            };
                            let a524 = 8192u32;
                            if equal(a523, a524) {
                                8192u64
                            } else {
                                0u64
                            }
                        };
                        let a530 = {
                            let a527 = {
                                let a525 = v3;
                                let a526 = 4096u32;
                                fixed_u32::bit_and(a525, a526)
                            };
                            let a528 = 4096u32;
                            if equal(a527, a528) {
                                4096u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a529, a530)?
                    };
                    nat_add(a531, a532)?
                };
                let a556 = {
                    let a553 = {
                        let a541 = {
                            let a535 = {
                                let a533 = v3;
                                let a534 = 2048u32;
                                fixed_u32::bit_and(a533, a534)
                            };
                            let a536 = 2048u32;
                            if equal(a535, a536) {
                                2048u64
                            } else {
                                0u64
                            }
                        };
                        let a542 = {
                            let a539 = {
                                let a537 = v3;
                                let a538 = 1024u32;
                                fixed_u32::bit_and(a537, a538)
                            };
                            let a540 = 1024u32;
                            if equal(a539, a540) {
                                1024u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a541, a542)?
                    };
                    let a554 = {
                        let a551 = {
                            let a545 = {
                                let a543 = v3;
                                let a544 = 512u32;
                                fixed_u32::bit_and(a543, a544)
                            };
                            let a546 = 512u32;
                            if equal(a545, a546) {
                                512u64
                            } else {
                                0u64
                            }
                        };
                        let a552 = {
                            let a549 = {
                                let a547 = v3;
                                let a548 = 256u32;
                                fixed_u32::bit_and(a547, a548)
                            };
                            let a550 = 256u32;
                            if equal(a549, a550) {
                                256u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a551, a552)?
                    };
                    nat_add(a553, a554)?
                };
                nat_add(a555, a556)?
            };
            let a604 = {
                let a601 = {
                    let a577 = {
                        let a565 = {
                            let a559 = {
                                let a557 = v3;
                                let a558 = 128u32;
                                fixed_u32::bit_and(a557, a558)
                            };
                            let a560 = 128u32;
                            if equal(a559, a560) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a566 = {
                            let a563 = {
                                let a561 = v3;
                                let a562 = 64u32;
                                fixed_u32::bit_and(a561, a562)
                            };
                            let a564 = 64u32;
                            if equal(a563, a564) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a565, a566)?
                    };
                    let a578 = {
                        let a575 = {
                            let a569 = {
                                let a567 = v3;
                                let a568 = 32u32;
                                fixed_u32::bit_and(a567, a568)
                            };
                            let a570 = 32u32;
                            if equal(a569, a570) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a576 = {
                            let a573 = {
                                let a571 = v3;
                                let a572 = 16u32;
                                fixed_u32::bit_and(a571, a572)
                            };
                            let a574 = 16u32;
                            if equal(a573, a574) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a575, a576)?
                    };
                    nat_add(a577, a578)?
                };
                let a602 = {
                    let a599 = {
                        let a587 = {
                            let a581 = {
                                let a579 = v3;
                                let a580 = 8u32;
                                fixed_u32::bit_and(a579, a580)
                            };
                            let a582 = 8u32;
                            if equal(a581, a582) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a588 = {
                            let a585 = {
                                let a583 = v3;
                                let a584 = 4u32;
                                fixed_u32::bit_and(a583, a584)
                            };
                            let a586 = 4u32;
                            if equal(a585, a586) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a587, a588)?
                    };
                    let a600 = {
                        let a597 = {
                            let a591 = {
                                let a589 = v3;
                                let a590 = 2u32;
                                fixed_u32::bit_and(a589, a590)
                            };
                            let a592 = 2u32;
                            if equal(a591, a592) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a598 = {
                            let a595 = {
                                let a593 = v3;
                                let a594 = 1u32;
                                fixed_u32::bit_and(a593, a594)
                            };
                            let a596 = 1u32;
                            if equal(a595, a596) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a597, a598)?
                    };
                    nat_add(a599, a600)?
                };
                nat_add(a601, a602)?
            };
            nat_add(a603, a604)?
        };
        nat_add(a605, a606)?
    })), f14({
        let v4: u32 = {
            let a607 = v0;
            let a608 = 255u32;
            fixed_u32::bit_and(a607, a608)
        };
        let a797 = {
            let a701 = {
                let a653 = {
                    let a629 = {
                        let a617 = {
                            let a611 = {
                                let a609 = v4;
                                let a610 = 2147483648u32;
                                fixed_u32::bit_and(a609, a610)
                            };
                            let a612 = 2147483648u32;
                            if equal(a611, a612) {
                                2147483648u64
                            } else {
                                0u64
                            }
                        };
                        let a618 = {
                            let a615 = {
                                let a613 = v4;
                                let a614 = 1073741824u32;
                                fixed_u32::bit_and(a613, a614)
                            };
                            let a616 = 1073741824u32;
                            if equal(a615, a616) {
                                1073741824u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a617, a618)?
                    };
                    let a630 = {
                        let a627 = {
                            let a621 = {
                                let a619 = v4;
                                let a620 = 536870912u32;
                                fixed_u32::bit_and(a619, a620)
                            };
                            let a622 = 536870912u32;
                            if equal(a621, a622) {
                                536870912u64
                            } else {
                                0u64
                            }
                        };
                        let a628 = {
                            let a625 = {
                                let a623 = v4;
                                let a624 = 268435456u32;
                                fixed_u32::bit_and(a623, a624)
                            };
                            let a626 = 268435456u32;
                            if equal(a625, a626) {
                                268435456u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a627, a628)?
                    };
                    nat_add(a629, a630)?
                };
                let a654 = {
                    let a651 = {
                        let a639 = {
                            let a633 = {
                                let a631 = v4;
                                let a632 = 134217728u32;
                                fixed_u32::bit_and(a631, a632)
                            };
                            let a634 = 134217728u32;
                            if equal(a633, a634) {
                                134217728u64
                            } else {
                                0u64
                            }
                        };
                        let a640 = {
                            let a637 = {
                                let a635 = v4;
                                let a636 = 67108864u32;
                                fixed_u32::bit_and(a635, a636)
                            };
                            let a638 = 67108864u32;
                            if equal(a637, a638) {
                                67108864u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a639, a640)?
                    };
                    let a652 = {
                        let a649 = {
                            let a643 = {
                                let a641 = v4;
                                let a642 = 33554432u32;
                                fixed_u32::bit_and(a641, a642)
                            };
                            let a644 = 33554432u32;
                            if equal(a643, a644) {
                                33554432u64
                            } else {
                                0u64
                            }
                        };
                        let a650 = {
                            let a647 = {
                                let a645 = v4;
                                let a646 = 16777216u32;
                                fixed_u32::bit_and(a645, a646)
                            };
                            let a648 = 16777216u32;
                            if equal(a647, a648) {
                                16777216u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a649, a650)?
                    };
                    nat_add(a651, a652)?
                };
                nat_add(a653, a654)?
            };
            let a702 = {
                let a699 = {
                    let a675 = {
                        let a663 = {
                            let a657 = {
                                let a655 = v4;
                                let a656 = 8388608u32;
                                fixed_u32::bit_and(a655, a656)
                            };
                            let a658 = 8388608u32;
                            if equal(a657, a658) {
                                8388608u64
                            } else {
                                0u64
                            }
                        };
                        let a664 = {
                            let a661 = {
                                let a659 = v4;
                                let a660 = 4194304u32;
                                fixed_u32::bit_and(a659, a660)
                            };
                            let a662 = 4194304u32;
                            if equal(a661, a662) {
                                4194304u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a663, a664)?
                    };
                    let a676 = {
                        let a673 = {
                            let a667 = {
                                let a665 = v4;
                                let a666 = 2097152u32;
                                fixed_u32::bit_and(a665, a666)
                            };
                            let a668 = 2097152u32;
                            if equal(a667, a668) {
                                2097152u64
                            } else {
                                0u64
                            }
                        };
                        let a674 = {
                            let a671 = {
                                let a669 = v4;
                                let a670 = 1048576u32;
                                fixed_u32::bit_and(a669, a670)
                            };
                            let a672 = 1048576u32;
                            if equal(a671, a672) {
                                1048576u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a673, a674)?
                    };
                    nat_add(a675, a676)?
                };
                let a700 = {
                    let a697 = {
                        let a685 = {
                            let a679 = {
                                let a677 = v4;
                                let a678 = 524288u32;
                                fixed_u32::bit_and(a677, a678)
                            };
                            let a680 = 524288u32;
                            if equal(a679, a680) {
                                524288u64
                            } else {
                                0u64
                            }
                        };
                        let a686 = {
                            let a683 = {
                                let a681 = v4;
                                let a682 = 262144u32;
                                fixed_u32::bit_and(a681, a682)
                            };
                            let a684 = 262144u32;
                            if equal(a683, a684) {
                                262144u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a685, a686)?
                    };
                    let a698 = {
                        let a695 = {
                            let a689 = {
                                let a687 = v4;
                                let a688 = 131072u32;
                                fixed_u32::bit_and(a687, a688)
                            };
                            let a690 = 131072u32;
                            if equal(a689, a690) {
                                131072u64
                            } else {
                                0u64
                            }
                        };
                        let a696 = {
                            let a693 = {
                                let a691 = v4;
                                let a692 = 65536u32;
                                fixed_u32::bit_and(a691, a692)
                            };
                            let a694 = 65536u32;
                            if equal(a693, a694) {
                                65536u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a695, a696)?
                    };
                    nat_add(a697, a698)?
                };
                nat_add(a699, a700)?
            };
            nat_add(a701, a702)?
        };
        let a798 = {
            let a795 = {
                let a747 = {
                    let a723 = {
                        let a711 = {
                            let a705 = {
                                let a703 = v4;
                                let a704 = 32768u32;
                                fixed_u32::bit_and(a703, a704)
                            };
                            let a706 = 32768u32;
                            if equal(a705, a706) {
                                32768u64
                            } else {
                                0u64
                            }
                        };
                        let a712 = {
                            let a709 = {
                                let a707 = v4;
                                let a708 = 16384u32;
                                fixed_u32::bit_and(a707, a708)
                            };
                            let a710 = 16384u32;
                            if equal(a709, a710) {
                                16384u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a711, a712)?
                    };
                    let a724 = {
                        let a721 = {
                            let a715 = {
                                let a713 = v4;
                                let a714 = 8192u32;
                                fixed_u32::bit_and(a713, a714)
                            };
                            let a716 = 8192u32;
                            if equal(a715, a716) {
                                8192u64
                            } else {
                                0u64
                            }
                        };
                        let a722 = {
                            let a719 = {
                                let a717 = v4;
                                let a718 = 4096u32;
                                fixed_u32::bit_and(a717, a718)
                            };
                            let a720 = 4096u32;
                            if equal(a719, a720) {
                                4096u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a721, a722)?
                    };
                    nat_add(a723, a724)?
                };
                let a748 = {
                    let a745 = {
                        let a733 = {
                            let a727 = {
                                let a725 = v4;
                                let a726 = 2048u32;
                                fixed_u32::bit_and(a725, a726)
                            };
                            let a728 = 2048u32;
                            if equal(a727, a728) {
                                2048u64
                            } else {
                                0u64
                            }
                        };
                        let a734 = {
                            let a731 = {
                                let a729 = v4;
                                let a730 = 1024u32;
                                fixed_u32::bit_and(a729, a730)
                            };
                            let a732 = 1024u32;
                            if equal(a731, a732) {
                                1024u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a733, a734)?
                    };
                    let a746 = {
                        let a743 = {
                            let a737 = {
                                let a735 = v4;
                                let a736 = 512u32;
                                fixed_u32::bit_and(a735, a736)
                            };
                            let a738 = 512u32;
                            if equal(a737, a738) {
                                512u64
                            } else {
                                0u64
                            }
                        };
                        let a744 = {
                            let a741 = {
                                let a739 = v4;
                                let a740 = 256u32;
                                fixed_u32::bit_and(a739, a740)
                            };
                            let a742 = 256u32;
                            if equal(a741, a742) {
                                256u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a743, a744)?
                    };
                    nat_add(a745, a746)?
                };
                nat_add(a747, a748)?
            };
            let a796 = {
                let a793 = {
                    let a769 = {
                        let a757 = {
                            let a751 = {
                                let a749 = v4;
                                let a750 = 128u32;
                                fixed_u32::bit_and(a749, a750)
                            };
                            let a752 = 128u32;
                            if equal(a751, a752) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a758 = {
                            let a755 = {
                                let a753 = v4;
                                let a754 = 64u32;
                                fixed_u32::bit_and(a753, a754)
                            };
                            let a756 = 64u32;
                            if equal(a755, a756) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a757, a758)?
                    };
                    let a770 = {
                        let a767 = {
                            let a761 = {
                                let a759 = v4;
                                let a760 = 32u32;
                                fixed_u32::bit_and(a759, a760)
                            };
                            let a762 = 32u32;
                            if equal(a761, a762) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a768 = {
                            let a765 = {
                                let a763 = v4;
                                let a764 = 16u32;
                                fixed_u32::bit_and(a763, a764)
                            };
                            let a766 = 16u32;
                            if equal(a765, a766) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a767, a768)?
                    };
                    nat_add(a769, a770)?
                };
                let a794 = {
                    let a791 = {
                        let a779 = {
                            let a773 = {
                                let a771 = v4;
                                let a772 = 8u32;
                                fixed_u32::bit_and(a771, a772)
                            };
                            let a774 = 8u32;
                            if equal(a773, a774) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a780 = {
                            let a777 = {
                                let a775 = v4;
                                let a776 = 4u32;
                                fixed_u32::bit_and(a775, a776)
                            };
                            let a778 = 4u32;
                            if equal(a777, a778) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a779, a780)?
                    };
                    let a792 = {
                        let a789 = {
                            let a783 = {
                                let a781 = v4;
                                let a782 = 2u32;
                                fixed_u32::bit_and(a781, a782)
                            };
                            let a784 = 2u32;
                            if equal(a783, a784) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a790 = {
                            let a787 = {
                                let a785 = v4;
                                let a786 = 1u32;
                                fixed_u32::bit_and(a785, a786)
                            };
                            let a788 = 1u32;
                            if equal(a787, a788) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a789, a790)?
                    };
                    nat_add(a791, a792)?
                };
                nat_add(a793, a794)?
            };
            nat_add(a795, a796)?
        };
        nat_add(a797, a798)?
    })))
}

pub fn f10(v0: Fn1, p802: u64, p803: (u64, Adt0)) -> R<(u64, Adt0)> {
    let m801 = (p802, p803);
    {
        let mut m801 = m801;
        loop {
            let step = {
                let (v1, v2) = m801;
                let m799 = v1;
                if m799 == 0 {
                    Err::<(u64, (u64, Adt0)), R<(u64, Adt0)>>(Ok(v2))
                } else {
                    let v3 = m799 - 1;
                    Ok::<(u64, (u64, Adt0)), R<(u64, Adt0)>>((v3, {
                        let c800: Fn1 = v0.clone();
                        c800.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m801 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f11(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt0)) -> R<(u64, Adt0)> {
    Ok(({
        let a805 = {
            let (h804, _) = v3.clone();
            h804
        };
        let a806 = 64u64;
        nat_add(a805, a806)?
    }, f16({
        let (_, h807) = v3;
        h807
    }, f17(v0, v1, v2, {
        let (h808, _) = v3;
        h808
    })?)?))
}

pub fn f12() -> Adt0 {
    Adt0::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f13(v0: u64, v1: u64) -> Bytes {
    let (_, h809) = f18(Fn2::F19, v1, (v0, Bytes::lit(&[])));
    h809
}

pub fn f14(v0: u64) -> Bytes {
    f20(f21(), v0)
}

pub fn f15(v0: u32, v1: u32) -> u32 {
    let m812 = {
        let a810 = v0;
        let a811 = v1;
        fixed_u32::shift_right(a810, a811)
    };
    match m812 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f16(v0: Adt0, v1: Adt1) -> R<Adt0> {
    let v2: Adt2 = f22(Fn3::F23, 64u64, Adt2::C0(match v0.clone() {
        Adt0::C0(h813, _, _, _, _, _, _, _) => {
            h813
        }
    }, match v0.clone() {
        Adt0::C0(_, h814, _, _, _, _, _, _) => {
            h814
        }
    }, match v0.clone() {
        Adt0::C0(_, _, h815, _, _, _, _, _) => {
            h815
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, h816, _, _, _, _) => {
            h816
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, h817, _, _, _) => {
            h817
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, h818, _, _) => {
            h818
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, h819, _) => {
            h819
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, h820) => {
            h820
        }
    }, v1, 0u64))?;
    Ok(Adt0::C0(f24(match v0.clone() {
        Adt0::C0(h821, _, _, _, _, _, _, _) => {
            h821
        }
    }, match v2.clone() {
        Adt2::C0(h822, _, _, _, _, _, _, _, _, _) => {
            h822
        }
    }), f24(match v0.clone() {
        Adt0::C0(_, h823, _, _, _, _, _, _) => {
            h823
        }
    }, match v2.clone() {
        Adt2::C0(_, h824, _, _, _, _, _, _, _, _) => {
            h824
        }
    }), f24(match v0.clone() {
        Adt0::C0(_, _, h825, _, _, _, _, _) => {
            h825
        }
    }, match v2.clone() {
        Adt2::C0(_, _, h826, _, _, _, _, _, _, _) => {
            h826
        }
    }), f24(match v0.clone() {
        Adt0::C0(_, _, _, h827, _, _, _, _) => {
            h827
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, h828, _, _, _, _, _, _) => {
            h828
        }
    }), f24(match v0.clone() {
        Adt0::C0(_, _, _, _, h829, _, _, _) => {
            h829
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, _, h830, _, _, _, _, _) => {
            h830
        }
    }), f24(match v0.clone() {
        Adt0::C0(_, _, _, _, _, h831, _, _) => {
            h831
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, _, _, h832, _, _, _, _) => {
            h832
        }
    }), f24(match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, h833, _) => {
            h833
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, _, _, _, h834, _, _, _) => {
            h834
        }
    }), f24(match v0 {
        Adt0::C0(_, _, _, _, _, _, _, h835) => {
            h835
        }
    }, match v2 {
        Adt2::C0(_, _, _, _, _, _, _, h836, _, _) => {
            h836
        }
    })))
}

pub fn f17(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt1> {
    Ok(Adt1::C0(f25(v0.clone(), v1, v2, v3)?, f25(v0.clone(), v1, v2, {
        let a837 = v3;
        let a838 = 4u64;
        nat_add(a837, a838)?
    })?, f25(v0.clone(), v1, v2, {
        let a839 = v3;
        let a840 = 8u64;
        nat_add(a839, a840)?
    })?, f25(v0.clone(), v1, v2, {
        let a841 = v3;
        let a842 = 12u64;
        nat_add(a841, a842)?
    })?, f25(v0.clone(), v1, v2, {
        let a843 = v3;
        let a844 = 16u64;
        nat_add(a843, a844)?
    })?, f25(v0.clone(), v1, v2, {
        let a845 = v3;
        let a846 = 20u64;
        nat_add(a845, a846)?
    })?, f25(v0.clone(), v1, v2, {
        let a847 = v3;
        let a848 = 24u64;
        nat_add(a847, a848)?
    })?, f25(v0.clone(), v1, v2, {
        let a849 = v3;
        let a850 = 28u64;
        nat_add(a849, a850)?
    })?, f25(v0.clone(), v1, v2, {
        let a851 = v3;
        let a852 = 32u64;
        nat_add(a851, a852)?
    })?, f25(v0.clone(), v1, v2, {
        let a853 = v3;
        let a854 = 36u64;
        nat_add(a853, a854)?
    })?, f25(v0.clone(), v1, v2, {
        let a855 = v3;
        let a856 = 40u64;
        nat_add(a855, a856)?
    })?, f25(v0.clone(), v1, v2, {
        let a857 = v3;
        let a858 = 44u64;
        nat_add(a857, a858)?
    })?, f25(v0.clone(), v1, v2, {
        let a859 = v3;
        let a860 = 48u64;
        nat_add(a859, a860)?
    })?, f25(v0.clone(), v1, v2, {
        let a861 = v3;
        let a862 = 52u64;
        nat_add(a861, a862)?
    })?, f25(v0.clone(), v1, v2, {
        let a863 = v3;
        let a864 = 56u64;
        nat_add(a863, a864)?
    })?, f25(v0, v1, v2, {
        let a865 = v3;
        let a866 = 60u64;
        nat_add(a865, a866)?
    })?))
}

pub fn f18(v0: Fn2, p870: u64, p871: (u64, Bytes)) -> (u64, Bytes) {
    let m869 = (p870, p871);
    {
        let mut m869 = m869;
        loop {
            let step = {
                let (v1, v2) = m869;
                let m867 = v1;
                if m867 == 0 {
                    Err::<(u64, (u64, Bytes)), (u64, Bytes)>(v2)
                } else {
                    let v3 = m867 - 1;
                    Ok::<(u64, (u64, Bytes)), (u64, Bytes)>((v3, {
                        let c868: Fn2 = v0.clone();
                        c868.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m869 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f19(v0: (u64, Bytes)) -> (u64, Bytes) {
    ({
        let a873 = {
            let (h872, _) = v0.clone();
            h872
        };
        let a874 = 256u64;
        let a875 = 0u64;
        nat_quot(a873, a874, a875)
    }, f7(f14({
        let a877 = {
            let (h876, _) = v0;
            h876
        };
        let a878 = 256u64;
        let a879 = 0u64;
        nat_rem(a877, a878, a879)
    }), {
        let (_, h880) = v0;
        h880
    }))
}

pub fn f20(v0: Bytes, v1: u64) -> Bytes {
    let m884 = {
        let a881 = v0;
        let a882 = v1;
        let a883 = 1u64;
        slice_bytes(a881, a882, a883)
    };
    match m884 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f21() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f22(v0: Fn3, p888: u64, p889: Adt2) -> R<Adt2> {
    let m887 = (p888, p889);
    {
        let mut m887 = m887;
        loop {
            let step = {
                let (v1, v2) = m887;
                let m885 = v1;
                if m885 == 0 {
                    Err::<(u64, Adt2), R<Adt2>>(Ok(v2))
                } else {
                    let v3 = m885 - 1;
                    Ok::<(u64, Adt2), R<Adt2>>((v3, {
                        let c886: Fn3 = v0.clone();
                        c886.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m887 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f23(v0: Adt2) -> R<Adt2> {
    f26(v0)
}

pub fn f24(v0: u32, v1: u32) -> u32 {
    let m892 = {
        let a890 = f27(v0);
        let a891 = f27(v1);
        fixed_u64::checked_add(a890, a891)
    };
    match m892 {
        None => {
            0u32
        }
        Some(v2) => {
            f28(v2)
        }
    }
}

pub fn f25(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u32> {
    let a895 = {
        let a893 = v3;
        let a894 = 8u64;
        nat_add(a893, a894)?
    };
    let a896 = v2;
    if nat_eq(a895, a896) {
        Ok(f29({
            let a899 = {
                let a897 = v1;
                let a898 = 8u64;
                nat_mul(a897, a898)?
            };
            let a900 = 4294967296u64;
            let a901 = 0u64;
            nat_quot(a899, a900, a901)
        }))
    } else {
        let a904 = {
            let a902 = v3;
            let a903 = 4u64;
            nat_add(a902, a903)?
        };
        let a905 = v2;
        if nat_eq(a904, a905) {
            Ok(f29({
                let a908 = {
                    let a906 = v1;
                    let a907 = 8u64;
                    nat_mul(a906, a907)?
                };
                let a909 = 4294967296u64;
                let a910 = 0u64;
                nat_rem(a908, a909, a910)
            }))
        } else {
            Ok(f30(f31(v0.clone(), v1, v3), f31(v0.clone(), v1, {
                let a911 = v3;
                let a912 = 1u64;
                nat_add(a911, a912)?
            }), f31(v0.clone(), v1, {
                let a913 = v3;
                let a914 = 2u64;
                nat_add(a913, a914)?
            }), f31(v0, v1, {
                let a915 = v3;
                let a916 = 3u64;
                nat_add(a915, a916)?
            })))
        }
    }
}

pub fn f26(v0: Adt2) -> R<Adt2> {
    let v1: u32 = f32(match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h917) => {
            h917
        }
    });
    let v2: u32 = f24(match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h918, _, _) => {
            h918
        }
    }, f24(f33(match v0.clone() {
        Adt2::C0(_, _, _, _, h919, _, _, _, _, _) => {
            h919
        }
    }), f24(f34(match v0.clone() {
        Adt2::C0(_, _, _, _, h920, _, _, _, _, _) => {
            h920
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h921, _, _, _, _) => {
            h921
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h922, _, _, _) => {
            h922
        }
    }), f24(v1, {
        let m925 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, h924, _) => {
                h924
            }
        };
        match m925 {
            Adt1::C0(h923, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h923
            }
        }
    }))));
    let v3: u32 = f24(f35(match v0.clone() {
        Adt2::C0(h926, _, _, _, _, _, _, _, _, _) => {
            h926
        }
    }), f36(match v0.clone() {
        Adt2::C0(h927, _, _, _, _, _, _, _, _, _) => {
            h927
        }
    }, match v0.clone() {
        Adt2::C0(_, h928, _, _, _, _, _, _, _, _) => {
            h928
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h929, _, _, _, _, _, _, _) => {
            h929
        }
    }));
    Ok(Adt2::C0(f24(v2, v3), match v0.clone() {
        Adt2::C0(h930, _, _, _, _, _, _, _, _, _) => {
            h930
        }
    }, match v0.clone() {
        Adt2::C0(_, h931, _, _, _, _, _, _, _, _) => {
            h931
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h932, _, _, _, _, _, _, _) => {
            h932
        }
    }, f24(match v0.clone() {
        Adt2::C0(_, _, _, h933, _, _, _, _, _, _) => {
            h933
        }
    }, v2), match v0.clone() {
        Adt2::C0(_, _, _, _, h934, _, _, _, _, _) => {
            h934
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h935, _, _, _, _) => {
            h935
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h936, _, _, _) => {
            h936
        }
    }, f37(match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, h937, _) => {
            h937
        }
    }), {
        let a939 = match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h938) => {
                h938
            }
        };
        let a940 = 1u64;
        nat_add(a939, a940)?
    }))
}

pub fn f27(v0: u32) -> u64 {
    let m942 = {
        let a941 = v0;
        fixed_u64::convert(i128::from(a941))
    };
    match m942 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f28(v0: u64) -> u32 {
    let m946 = {
        let a945 = {
            let a943 = v0;
            let a944 = 4294967295u64;
            fixed_u64::bit_and(a943, a944)
        };
        fixed_u32::convert(i128::from(a945))
    };
    match m946 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f29(v0: u64) -> u32 {
    let (_, h947) = {
        let (_, h948) = f38(Fn4::F39, 32u64, (v0, (1u32, 0u32)));
        h948
    };
    h947
}

pub fn f30(v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
    let a953 = f40(v0, 24u32);
    let a954 = {
        let a951 = f40(v1, 16u32);
        let a952 = {
            let a949 = f40(v2, 8u32);
            let a950 = v3;
            fixed_u32::bit_or(a949, a950)
        };
        fixed_u32::bit_or(a951, a952)
    };
    fixed_u32::bit_or(a953, a954)
}

pub fn f31(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a955 = v2;
    let a956 = v1;
    if nat_lt(a955, a956) {
        let m959 = {
            let a957 = v0;
            let a958 = v2;
            index_bytes(a957, a958)
        };
        match m959 {
            None => {
                0u32
            }
            Some(v3) => {
                let m961 = {
                    let a960 = v3;
                    fixed_u32::convert(i128::from(a960))
                };
                match m961 {
                    None => {
                        0u32
                    }
                    Some(v4) => {
                        v4
                    }
                }
            }
        }
    } else {
        let a962 = v2;
        let a963 = v1;
        if nat_eq(a962, a963) {
            128u32
        } else {
            0u32
        }
    }
}

pub fn f32(v0: u64) -> u32 {
    let a964 = v0;
    let a965 = 32u64;
    if nat_lt(a964, a965) {
        let a966 = v0;
        let a967 = 16u64;
        if nat_lt(a966, a967) {
            let a968 = v0;
            let a969 = 8u64;
            if nat_lt(a968, a969) {
                let a970 = v0;
                let a971 = 4u64;
                if nat_lt(a970, a971) {
                    let a972 = v0;
                    let a973 = 2u64;
                    if nat_lt(a972, a973) {
                        let a974 = v0;
                        let a975 = 1u64;
                        if nat_lt(a974, a975) {
                            1116352408u32
                        } else {
                            1899447441u32
                        }
                    } else {
                        let a976 = v0;
                        let a977 = 3u64;
                        if nat_lt(a976, a977) {
                            3049323471u32
                        } else {
                            3921009573u32
                        }
                    }
                } else {
                    let a978 = v0;
                    let a979 = 6u64;
                    if nat_lt(a978, a979) {
                        let a980 = v0;
                        let a981 = 5u64;
                        if nat_lt(a980, a981) {
                            961987163u32
                        } else {
                            1508970993u32
                        }
                    } else {
                        let a982 = v0;
                        let a983 = 7u64;
                        if nat_lt(a982, a983) {
                            2453635748u32
                        } else {
                            2870763221u32
                        }
                    }
                }
            } else {
                let a984 = v0;
                let a985 = 12u64;
                if nat_lt(a984, a985) {
                    let a986 = v0;
                    let a987 = 10u64;
                    if nat_lt(a986, a987) {
                        let a988 = v0;
                        let a989 = 9u64;
                        if nat_lt(a988, a989) {
                            3624381080u32
                        } else {
                            310598401u32
                        }
                    } else {
                        let a990 = v0;
                        let a991 = 11u64;
                        if nat_lt(a990, a991) {
                            607225278u32
                        } else {
                            1426881987u32
                        }
                    }
                } else {
                    let a992 = v0;
                    let a993 = 14u64;
                    if nat_lt(a992, a993) {
                        let a994 = v0;
                        let a995 = 13u64;
                        if nat_lt(a994, a995) {
                            1925078388u32
                        } else {
                            2162078206u32
                        }
                    } else {
                        let a996 = v0;
                        let a997 = 15u64;
                        if nat_lt(a996, a997) {
                            2614888103u32
                        } else {
                            3248222580u32
                        }
                    }
                }
            }
        } else {
            let a998 = v0;
            let a999 = 24u64;
            if nat_lt(a998, a999) {
                let a1000 = v0;
                let a1001 = 20u64;
                if nat_lt(a1000, a1001) {
                    let a1002 = v0;
                    let a1003 = 18u64;
                    if nat_lt(a1002, a1003) {
                        let a1004 = v0;
                        let a1005 = 17u64;
                        if nat_lt(a1004, a1005) {
                            3835390401u32
                        } else {
                            4022224774u32
                        }
                    } else {
                        let a1006 = v0;
                        let a1007 = 19u64;
                        if nat_lt(a1006, a1007) {
                            264347078u32
                        } else {
                            604807628u32
                        }
                    }
                } else {
                    let a1008 = v0;
                    let a1009 = 22u64;
                    if nat_lt(a1008, a1009) {
                        let a1010 = v0;
                        let a1011 = 21u64;
                        if nat_lt(a1010, a1011) {
                            770255983u32
                        } else {
                            1249150122u32
                        }
                    } else {
                        let a1012 = v0;
                        let a1013 = 23u64;
                        if nat_lt(a1012, a1013) {
                            1555081692u32
                        } else {
                            1996064986u32
                        }
                    }
                }
            } else {
                let a1014 = v0;
                let a1015 = 28u64;
                if nat_lt(a1014, a1015) {
                    let a1016 = v0;
                    let a1017 = 26u64;
                    if nat_lt(a1016, a1017) {
                        let a1018 = v0;
                        let a1019 = 25u64;
                        if nat_lt(a1018, a1019) {
                            2554220882u32
                        } else {
                            2821834349u32
                        }
                    } else {
                        let a1020 = v0;
                        let a1021 = 27u64;
                        if nat_lt(a1020, a1021) {
                            2952996808u32
                        } else {
                            3210313671u32
                        }
                    }
                } else {
                    let a1022 = v0;
                    let a1023 = 30u64;
                    if nat_lt(a1022, a1023) {
                        let a1024 = v0;
                        let a1025 = 29u64;
                        if nat_lt(a1024, a1025) {
                            3336571891u32
                        } else {
                            3584528711u32
                        }
                    } else {
                        let a1026 = v0;
                        let a1027 = 31u64;
                        if nat_lt(a1026, a1027) {
                            113926993u32
                        } else {
                            338241895u32
                        }
                    }
                }
            }
        }
    } else {
        let a1028 = v0;
        let a1029 = 48u64;
        if nat_lt(a1028, a1029) {
            let a1030 = v0;
            let a1031 = 40u64;
            if nat_lt(a1030, a1031) {
                let a1032 = v0;
                let a1033 = 36u64;
                if nat_lt(a1032, a1033) {
                    let a1034 = v0;
                    let a1035 = 34u64;
                    if nat_lt(a1034, a1035) {
                        let a1036 = v0;
                        let a1037 = 33u64;
                        if nat_lt(a1036, a1037) {
                            666307205u32
                        } else {
                            773529912u32
                        }
                    } else {
                        let a1038 = v0;
                        let a1039 = 35u64;
                        if nat_lt(a1038, a1039) {
                            1294757372u32
                        } else {
                            1396182291u32
                        }
                    }
                } else {
                    let a1040 = v0;
                    let a1041 = 38u64;
                    if nat_lt(a1040, a1041) {
                        let a1042 = v0;
                        let a1043 = 37u64;
                        if nat_lt(a1042, a1043) {
                            1695183700u32
                        } else {
                            1986661051u32
                        }
                    } else {
                        let a1044 = v0;
                        let a1045 = 39u64;
                        if nat_lt(a1044, a1045) {
                            2177026350u32
                        } else {
                            2456956037u32
                        }
                    }
                }
            } else {
                let a1046 = v0;
                let a1047 = 44u64;
                if nat_lt(a1046, a1047) {
                    let a1048 = v0;
                    let a1049 = 42u64;
                    if nat_lt(a1048, a1049) {
                        let a1050 = v0;
                        let a1051 = 41u64;
                        if nat_lt(a1050, a1051) {
                            2730485921u32
                        } else {
                            2820302411u32
                        }
                    } else {
                        let a1052 = v0;
                        let a1053 = 43u64;
                        if nat_lt(a1052, a1053) {
                            3259730800u32
                        } else {
                            3345764771u32
                        }
                    }
                } else {
                    let a1054 = v0;
                    let a1055 = 46u64;
                    if nat_lt(a1054, a1055) {
                        let a1056 = v0;
                        let a1057 = 45u64;
                        if nat_lt(a1056, a1057) {
                            3516065817u32
                        } else {
                            3600352804u32
                        }
                    } else {
                        let a1058 = v0;
                        let a1059 = 47u64;
                        if nat_lt(a1058, a1059) {
                            4094571909u32
                        } else {
                            275423344u32
                        }
                    }
                }
            }
        } else {
            let a1060 = v0;
            let a1061 = 56u64;
            if nat_lt(a1060, a1061) {
                let a1062 = v0;
                let a1063 = 52u64;
                if nat_lt(a1062, a1063) {
                    let a1064 = v0;
                    let a1065 = 50u64;
                    if nat_lt(a1064, a1065) {
                        let a1066 = v0;
                        let a1067 = 49u64;
                        if nat_lt(a1066, a1067) {
                            430227734u32
                        } else {
                            506948616u32
                        }
                    } else {
                        let a1068 = v0;
                        let a1069 = 51u64;
                        if nat_lt(a1068, a1069) {
                            659060556u32
                        } else {
                            883997877u32
                        }
                    }
                } else {
                    let a1070 = v0;
                    let a1071 = 54u64;
                    if nat_lt(a1070, a1071) {
                        let a1072 = v0;
                        let a1073 = 53u64;
                        if nat_lt(a1072, a1073) {
                            958139571u32
                        } else {
                            1322822218u32
                        }
                    } else {
                        let a1074 = v0;
                        let a1075 = 55u64;
                        if nat_lt(a1074, a1075) {
                            1537002063u32
                        } else {
                            1747873779u32
                        }
                    }
                }
            } else {
                let a1076 = v0;
                let a1077 = 60u64;
                if nat_lt(a1076, a1077) {
                    let a1078 = v0;
                    let a1079 = 58u64;
                    if nat_lt(a1078, a1079) {
                        let a1080 = v0;
                        let a1081 = 57u64;
                        if nat_lt(a1080, a1081) {
                            1955562222u32
                        } else {
                            2024104815u32
                        }
                    } else {
                        let a1082 = v0;
                        let a1083 = 59u64;
                        if nat_lt(a1082, a1083) {
                            2227730452u32
                        } else {
                            2361852424u32
                        }
                    }
                } else {
                    let a1084 = v0;
                    let a1085 = 62u64;
                    if nat_lt(a1084, a1085) {
                        let a1086 = v0;
                        let a1087 = 61u64;
                        if nat_lt(a1086, a1087) {
                            2428436474u32
                        } else {
                            2756734187u32
                        }
                    } else {
                        let a1088 = v0;
                        let a1089 = 63u64;
                        if nat_lt(a1088, a1089) {
                            3204031479u32
                        } else {
                            3329325298u32
                        }
                    }
                }
            }
        }
    }
}

pub fn f33(v0: u32) -> u32 {
    let a1092 = {
        let a1090 = f41(v0, 6u32, 26u32);
        let a1091 = f41(v0, 11u32, 21u32);
        fixed_u32::bit_xor(a1090, a1091)
    };
    let a1093 = f41(v0, 25u32, 7u32);
    fixed_u32::bit_xor(a1092, a1093)
}

pub fn f34(v0: u32, v1: u32, v2: u32) -> u32 {
    let a1099 = {
        let a1094 = v0;
        let a1095 = v1;
        fixed_u32::bit_and(a1094, a1095)
    };
    let a1100 = {
        let a1097 = {
            let a1096 = v0;
            fixed_u32::bit_not(a1096)
        };
        let a1098 = v2;
        fixed_u32::bit_and(a1097, a1098)
    };
    fixed_u32::bit_xor(a1099, a1100)
}

pub fn f35(v0: u32) -> u32 {
    let a1103 = {
        let a1101 = f41(v0, 2u32, 30u32);
        let a1102 = f41(v0, 13u32, 19u32);
        fixed_u32::bit_xor(a1101, a1102)
    };
    let a1104 = f41(v0, 22u32, 10u32);
    fixed_u32::bit_xor(a1103, a1104)
}

pub fn f36(v0: u32, v1: u32, v2: u32) -> u32 {
    let a1113 = {
        let a1109 = {
            let a1105 = v0;
            let a1106 = v1;
            fixed_u32::bit_and(a1105, a1106)
        };
        let a1110 = {
            let a1107 = v0;
            let a1108 = v2;
            fixed_u32::bit_and(a1107, a1108)
        };
        fixed_u32::bit_xor(a1109, a1110)
    };
    let a1114 = {
        let a1111 = v1;
        let a1112 = v2;
        fixed_u32::bit_and(a1111, a1112)
    };
    fixed_u32::bit_xor(a1113, a1114)
}

pub fn f37(v0: Adt1) -> Adt1 {
    Adt1::C0(match v0.clone() {
        Adt1::C0(_, h1115, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1115
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h1116, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1116
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h1117, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1117
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h1118, _, _, _, _, _, _, _, _, _, _, _) => {
            h1118
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, h1119, _, _, _, _, _, _, _, _, _, _) => {
            h1119
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h1120, _, _, _, _, _, _, _, _, _) => {
            h1120
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h1121, _, _, _, _, _, _, _, _) => {
            h1121
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, h1122, _, _, _, _, _, _, _) => {
            h1122
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, h1123, _, _, _, _, _, _) => {
            h1123
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, h1124, _, _, _, _, _) => {
            h1124
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, h1125, _, _, _, _) => {
            h1125
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, h1126, _, _, _) => {
            h1126
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1127, _, _) => {
            h1127
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1128, _) => {
            h1128
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1129) => {
            h1129
        }
    }, f24(f42(match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1130, _) => {
            h1130
        }
    }), f24(match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, h1131, _, _, _, _, _, _) => {
            h1131
        }
    }, f24(f43(match v0.clone() {
        Adt1::C0(_, h1132, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1132
        }
    }), match v0 {
        Adt1::C0(h1133, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1133
        }
    }))))
}

pub fn f38(v0: Fn4, p1137: u64, p1138: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m1136 = (p1137, p1138);
    {
        let mut m1136 = m1136;
        loop {
            let step = {
                let (v1, v2) = m1136;
                let m1134 = v1;
                if m1134 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m1134 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c1135: Fn4 = v0.clone();
                        c1135.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m1136 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f39(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h1139, _) = v0;
        h1139
    };
    let v2: u32 = {
        let (h1140, _) = {
            let (_, h1141) = v0;
            h1141
        };
        h1140
    };
    let v3: u32 = {
        let (_, h1142) = {
            let (_, h1143) = v0;
            h1143
        };
        h1142
    };
    ({
        let a1144 = v1;
        let a1145 = 2u64;
        let a1146 = 0u64;
        nat_quot(a1144, a1145, a1146)
    }, (f40(v2, 1u32), {
        let a1150 = {
            let a1147 = v1;
            let a1148 = 2u64;
            let a1149 = 0u64;
            nat_rem(a1147, a1148, a1149)
        };
        let a1151 = 1u64;
        if nat_eq(a1150, a1151) {
            let a1152 = v3;
            let a1153 = v2;
            fixed_u32::bit_or(a1152, a1153)
        } else {
            v3
        }
    }))
}

pub fn f40(v0: u32, v1: u32) -> u32 {
    let m1156 = {
        let a1154 = v0;
        let a1155 = v1;
        fixed_u32::shift_left(a1154, a1155)
    };
    match m1156 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f41(v0: u32, v1: u32, v2: u32) -> u32 {
    let a1157 = f15(v0, v1);
    let a1158 = f40(v0, v2);
    fixed_u32::bit_or(a1157, a1158)
}

pub fn f42(v0: u32) -> u32 {
    let a1161 = {
        let a1159 = f41(v0, 17u32, 15u32);
        let a1160 = f41(v0, 19u32, 13u32);
        fixed_u32::bit_xor(a1159, a1160)
    };
    let a1162 = f15(v0, 10u32);
    fixed_u32::bit_xor(a1161, a1162)
}

pub fn f43(v0: u32) -> u32 {
    let a1165 = {
        let a1163 = f41(v0, 7u32, 25u32);
        let a1164 = f41(v0, 18u32, 14u32);
        fixed_u32::bit_xor(a1163, a1164)
    };
    let a1166 = f15(v0, 3u32);
    fixed_u32::bit_xor(a1165, a1166)
}

pub fn run(p0: List<(Bytes, Bytes)>) -> R<Bytes> {
    f0(p0)
}
