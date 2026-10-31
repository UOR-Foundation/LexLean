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
    C0(Adt0, Adt3),
    C1(u64),
    C2(u64, Adt0, std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C3(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C4(Adt0, std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C5(Adt5, Adt0, std::rc::Rc<Adt2>),
    C6(u64, std::rc::Rc<Adt2>),
    C7(u64, std::rc::Rc<Adt2>),
    C8(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C9(Adt6, std::rc::Rc<Adt2>),
    C10(std::rc::Rc<Adt2>),
    C11(std::rc::Rc<Adt2>),
    C12(std::rc::Rc<Adt2>, u64),
    C13,
    C14(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C15(Adt5, List<u64>, std::rc::Rc<Adt2>),
}

#[derive(Clone)]
pub enum Adt3 {
    C0,
    C1(bool),
    C2(Bytes),
    C3(Bytes),
    C4(Bytes),
    C5(Bytes),
    C6(Bytes),
    C7(Bytes),
    C8(Bytes),
    C9(Bytes),
    C10(Bytes),
    C11(Bytes),
    C12(Bytes),
    C13(Bytes),
    C14(Adt4),
    C15,
    C16(std::rc::Rc<Adt3>),
    C17(std::rc::Rc<Adt3>),
    C18(std::rc::Rc<Adt3>),
    C19(std::rc::Rc<Adt3>),
    C20(std::rc::Rc<Adt3>, std::rc::Rc<Adt3>),
    C21(u64, std::rc::Rc<Adt3>),
    C22(u64, std::rc::Rc<Adt3>),
    C23,
    C24(std::rc::Rc<Adt3>, std::rc::Rc<Adt3>),
}

#[derive(Clone)]
pub enum Adt4 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt5 {
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
    C15(u64),
}

#[derive(Clone)]
pub enum Adt6 {
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
    C40(Adt1),
    C41(Adt0),
}

#[derive(Clone)]
pub enum Adt7 {
    C0(Bytes, Adt0, List<List<u64>>, Adt0, Adt0, Adt2),
}

#[derive(Clone)]
pub enum Adt8 {
    C0(Bytes),
    C1(Adt0),
    C2(List<Adt5>),
    C3,
}

#[derive(Clone)]
pub enum Adt9 {
    C0(bool, Bytes),
}

#[derive(Clone)]
pub enum Fn0 {
    F65,
}

impl Fn0 {
    pub fn apply(&self, p0: u64, p1: u64) -> R<u64> {
        match self {
            Fn0::F65 => f65(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F80(Bytes),
    F93(Bytes),
    F101(Bytes),
}

impl Fn1 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn1::F80(k0) => f80(k0.clone(), p0),
            Fn1::F93(k0) => f93(k0.clone(), p0),
            Fn1::F101(k0) => f101(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F87(Bytes),
}

impl Fn2 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn2::F87(k0) => f87(k0.clone(), p0),
        }
    }
}

pub fn f0(v0: Bytes, v1: Adt0, v2: List<List<u64>>, v3: Adt0, v4: Adt0, v5: Adt2) -> R<Adt0> {
    f1(Adt7::C0(v0, v1, v2, v3, v4, v5.clone()), v5, List::<(u64, Adt0)>::nil(), Adt0::C14, 1u64)
}

pub fn f1(v0: Adt7, v1: Adt2, v2: List<(u64, Adt0)>, v3: Adt0, v4: u64) -> R<Adt0> {
    let m1 = v1;
    match m1 {
        Adt2::C0(_, _) => {
            Ok(Adt0::C14)
        }
        Adt2::C1(_) => {
            Ok(Adt0::C14)
        }
        Adt2::C2(v8, v9, r5, r6) => {
            let v10 = (*r5).clone();
            let v11 = (*r6).clone();
            Ok(f2(Adt0::C14, f1(v0.clone(), v10, v2.clone(), Adt0::C14, 0u64)?, Adt0::C15(std::rc::Rc::new(f1(v0, v11, f3(v8, v9, v2), Adt0::C14, 0u64)?), std::rc::Rc::new(Adt0::C14))))
        }
        Adt2::C3(r7, r8, r9) => {
            let v12 = (*r7).clone();
            let v13 = (*r8).clone();
            let v14 = (*r9).clone();
            Ok(f2(Adt0::C14, f1(v0.clone(), v12, v2.clone(), Adt0::C14, 0u64)?, Adt0::C15(std::rc::Rc::new(f1(v0.clone(), v13, v2.clone(), Adt0::C14, 0u64)?), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(f1(v0, v14, v2, Adt0::C14, 0u64)?), std::rc::Rc::new(Adt0::C14))))))
        }
        Adt2::C4(_, r10, r11) => {
            let v16 = (*r10).clone();
            let v17 = (*r11).clone();
            let v18: Adt0 = f4(v0.clone(), v16.clone(), v2.clone())?;
            Ok(f2(Adt0::C14, f1(v0.clone(), v16, v2.clone(), Adt0::C14, 0u64)?, Adt0::C15(std::rc::Rc::new(f1(v0, v17, v2, v18, 0u64)?), std::rc::Rc::new(Adt0::C14))))
        }
        Adt2::C5(_, _, r12) => {
            let v21 = (*r12).clone();
            Ok(f5(f1(v0, v21, v2, Adt0::C14, 0u64)?))
        }
        Adt2::C6(_, r13) => {
            let v23 = (*r13).clone();
            Ok(f5(f1(v0, v23, v2, Adt0::C14, 0u64)?))
        }
        Adt2::C7(_, r14) => {
            let v25 = (*r14).clone();
            Ok(f5(f1(v0, v25, v2, Adt0::C14, 0u64)?))
        }
        Adt2::C8(r15, r16) => {
            let v26 = (*r15).clone();
            let v27 = (*r16).clone();
            Ok(f2(Adt0::C14, f1(v0.clone(), v26, v2.clone(), Adt0::C14, 0u64)?, Adt0::C15(std::rc::Rc::new(f1(v0, v27, v2, Adt0::C14, 0u64)?), std::rc::Rc::new(Adt0::C14))))
        }
        Adt2::C9(_, r17) => {
            let v29 = (*r17).clone();
            Ok(f5(f1(v0, v29, v2, Adt0::C0, 0u64)?))
        }
        Adt2::C10(r18) => {
            let v30 = (*r18).clone();
            Ok(f5(f1(v0, v30, v2, Adt0::C14, 0u64)?))
        }
        Adt2::C11(r19) => {
            let v31 = (*r19).clone();
            Ok(f5(f1(v0, v31, v2, Adt0::C14, 0u64)?))
        }
        Adt2::C12(r20, _) => {
            let v32 = (*r20).clone();
            Ok(f5(f6(f4(v0.clone(), v32.clone(), v2.clone())?, f1(v0, v32, v2, Adt0::C14, 0u64)?)))
        }
        Adt2::C13 => {
            Ok(Adt0::C14)
        }
        Adt2::C14(r21, r22) => {
            let v34 = (*r21).clone();
            let v35 = (*r22).clone();
            let m2 = v4;
            if m2 == 0 {
                let v36: Adt0 = f1(v0.clone(), v34.clone(), v2.clone(), v3.clone(), 0u64)?;
                Ok(f2(Adt0::C14, if f7(v3.clone()) {
                    v36
                } else {
                    f6(f4(v0.clone(), v34, v2.clone())?, v36)
                }, Adt0::C15(std::rc::Rc::new(f1(v0, v35, v2, v3, 0u64)?), std::rc::Rc::new(Adt0::C14))))
            } else {
                let v37 = m2 - 1;
                Ok(f2(Adt0::C14, f1(v0.clone(), v34, f8(f9(v0.clone(), v37), f10(v0.clone(), v37), List::<(u64, Adt0)>::nil()), Adt0::C14, 0u64)?, Adt0::C15(std::rc::Rc::new(f1(v0, v35, v2, v3, {
                    let a3 = v4;
                    let a4 = 1u64;
                    nat_add(a3, a4)?
                })?), std::rc::Rc::new(Adt0::C14))))
            }
        }
        Adt2::C15(v38, v39, r23) => {
            let v40 = (*r23).clone();
            Ok(f2(Adt0::C14, f1(v0.clone(), v40, f8(v39, f11(v0, v38, v3)?, v2), Adt0::C14, 0u64)?, Adt0::C14))
        }
    }
}

pub fn f2(v0: Adt0, v1: Adt0, v2: Adt0) -> Adt0 {
    Adt0::C11(std::rc::Rc::new(v0), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(v1), std::rc::Rc::new(v2))))
}

pub fn f3(v0: u64, v1: Adt0, v2: List<(u64, Adt0)>) -> List<(u64, Adt0)> {
    List::cons((v0, v1), v2)
}

pub fn f4(v0: Adt7, v1: Adt2, v2: List<(u64, Adt0)>) -> R<Adt0> {
    let m24 = f12(v0, v1, v2, 0u64, Adt0::C14, Adt0::C14, f13(Str::lit("")), 0u64, List::<Adt5>::nil())?;
    match m24 {
        Adt8::C0(_) => {
            Ok(Adt0::C14)
        }
        Adt8::C1(v3) => {
            Ok(v3)
        }
        Adt8::C2(_) => {
            Ok(Adt0::C14)
        }
        Adt8::C3 => {
            Ok(Adt0::C14)
        }
    }
}

pub fn f5(v0: Adt0) -> Adt0 {
    Adt0::C11(std::rc::Rc::new(Adt0::C14), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(v0), std::rc::Rc::new(Adt0::C14))))
}

pub fn f6(v0: Adt0, v1: Adt0) -> Adt0 {
    Adt0::C11(std::rc::Rc::new(v0), std::rc::Rc::new(if f7(v1.clone()) {
        Adt0::C14
    } else {
        f14(v1)
    }))
}

pub fn f7(v0: Adt0) -> bool {
    let a25 = f15(v0);
    let a26 = 14u64;
    nat_eq(a25, a26)
}

pub fn f8(v0: List<u64>, v1: Adt0, v2: List<(u64, Adt0)>) -> List<(u64, Adt0)> {
    let m27 = v0;
    match m27.uncons() {
        None => {
            v2
        }
        Some((v3, v4)) => {
            let v5: List<(u64, Adt0)> = f8(v4, f14(v1.clone()), f3(v3, f16(v1.clone()), v2.clone()));
            if f7(v1) {
                v2
            } else {
                v5
            }
        }
    }
}

pub fn f9(v0: Adt7, v1: u64) -> List<u64> {
    let m29 = f17(match v0 {
        Adt7::C0(_, _, h28, _, _, _) => {
            h28
        }
    }, v1);
    match m29 {
        None => {
            List::<u64>::nil()
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f10(v0: Adt7, v1: u64) -> Adt0 {
    let m30 = f18(v0, v1);
    match m30 {
        None => {
            Adt0::C14
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f11(v0: Adt7, v1: Adt5, v2: Adt0) -> R<Adt0> {
    let m31 = f19(v0, v1, v2)?;
    match m31 {
        Ok(v3) => {
            Ok(v3)
        }
        Err(_) => {
            Ok(Adt0::C14)
        }
    }
}

pub fn f12(v0: Adt7, v1: Adt2, v2: List<(u64, Adt0)>, v3: u64, v4: Adt0, v5: Adt0, v6: Bytes, v7: u64, v8: List<Adt5>) -> R<Adt8> {
    let m32 = v1;
    match m32 {
        Adt2::C0(v9, v10) => {
            let a33 = v3;
            let a34 = 0u64;
            if nat_eq(a33, a34) {
                let m35 = f20(v0.clone(), v9.clone())?;
                match m35 {
                    Ok(_) => {
                        let m36 = f21(v0, v10, v9.clone(), 0u64)?;
                        match m36 {
                            Ok(_) => {
                                Ok(Adt8::C1(v9))
                            }
                            Err(v13) => {
                                Ok(Adt8::C0(v13))
                            }
                        }
                    }
                    Err(v14) => {
                        Ok(Adt8::C0(v14))
                    }
                }
            } else {
                Ok(Adt8::C0(f13(Str::lit("unreachable"))))
            }
        }
        Adt2::C1(v15) => {
            let a37 = v3;
            let a38 = 0u64;
            if nat_eq(a37, a38) {
                let m39 = f22(v2, v15);
                match m39 {
                    None => {
                        Ok(Adt8::C0(f23(f13(Str::lit("local ")), f23(f24(v15)?, f13(Str::lit(" is unbound"))))))
                    }
                    Some(v16) => {
                        Ok(Adt8::C1(v16))
                    }
                }
            } else {
                Ok(Adt8::C0(f13(Str::lit("unreachable"))))
            }
        }
        Adt2::C2(v17, v18, r144, r145) => {
            let v19 = (*r144).clone();
            let v20 = (*r145).clone();
            let a40 = v3;
            let a41 = 0u64;
            if nat_eq(a40, a41) {
                let m42 = f20(v0.clone(), v18.clone())?;
                match m42 {
                    Ok(_) => {
                        let m43 = f12(v0.clone(), v19, v2.clone(), 0u64, Adt0::C14, Adt0::C14, f13(Str::lit("")), 0u64, List::<Adt5>::nil())?;
                        match m43 {
                            Adt8::C0(v25) => {
                                Ok(Adt8::C0(v25))
                            }
                            Adt8::C1(v22) => {
                                let m44 = f25(v22, v18.clone(), f23(f13(Str::lit("the value bound to ")), f24(v17)?))?;
                                match m44 {
                                    Ok(_) => {
                                        f12(v0, v20, f3(v17, v18, v2), 0u64, Adt0::C14, Adt0::C14, f13(Str::lit("")), 0u64, List::<Adt5>::nil())
                                    }
                                    Err(v24) => {
                                        Ok(Adt8::C0(v24))
                                    }
                                }
                            }
                            Adt8::C2(_) => {
                                Ok(Adt8::C0(f13(Str::lit("unreachable"))))
                            }
                            Adt8::C3 => {
                                Ok(Adt8::C0(f13(Str::lit("unreachable"))))
                            }
                        }
                    }
                    Err(v27) => {
                        Ok(Adt8::C0(v27))
                    }
                }
            } else {
                Ok(Adt8::C0(f13(Str::lit("unreachable"))))
            }
        }
        Adt2::C3(r146, r147, r148) => {
            let v28 = (*r146).clone();
            let v29 = (*r147).clone();
            let v30 = (*r148).clone();
            let a45 = v3;
            let a46 = 0u64;
            if nat_eq(a45, a46) {
                let m47 = f12(v0.clone(), v28, v2.clone(), 0u64, Adt0::C14, Adt0::C14, f13(Str::lit("")), 0u64, List::<Adt5>::nil())?;
                match m47 {
                    Adt8::C0(v42) => {
                        Ok(Adt8::C0(v42))
                    }
                    Adt8::C1(v31) => {
                        let m48 = f25(v31, Adt0::C1, f13(Str::lit("a condition")))?;
                        match m48 {
                            Ok(_) => {
                                let m49 = f12(v0.clone(), v29, v2.clone(), 0u64, Adt0::C14, Adt0::C14, f13(Str::lit("")), 0u64, List::<Adt5>::nil())?;
                                match m49 {
                                    Adt8::C0(v39) => {
                                        Ok(Adt8::C0(v39))
                                    }
                                    Adt8::C1(v33) => {
                                        let m50 = f12(v0, v30, v2, 0u64, Adt0::C14, Adt0::C14, f13(Str::lit("")), 0u64, List::<Adt5>::nil())?;
                                        match m50 {
                                            Adt8::C0(v37) => {
                                                Ok(Adt8::C0(v37))
                                            }
                                            Adt8::C1(v34) => {
                                                let m51 = f25(v34, v33.clone(), f13(Str::lit("the else branch")))?;
                                                match m51 {
                                                    Ok(_) => {
                                                        Ok(Adt8::C1(v33))
                                                    }
                                                    Err(v36) => {
                                                        Ok(Adt8::C0(v36))
                                                    }
                                                }
                                            }
                                            Adt8::C2(_) => {
                                                Ok(Adt8::C0(f13(Str::lit("unreachable"))))
                                            }
                                            Adt8::C3 => {
                                                Ok(Adt8::C0(f13(Str::lit("unreachable"))))
                                            }
                                        }
                                    }
                                    Adt8::C2(_) => {
                                        Ok(Adt8::C0(f13(Str::lit("unreachable"))))
                                    }
                                    Adt8::C3 => {
                                        Ok(Adt8::C0(f13(Str::lit("unreachable"))))
                                    }
                                }
                            }
                            Err(v41) => {
                                Ok(Adt8::C0(v41))
                            }
                        }
                    }
                    Adt8::C2(_) => {
                        Ok(Adt8::C0(f13(Str::lit("unreachable"))))
                    }
                    Adt8::C3 => {
                        Ok(Adt8::C0(f13(Str::lit("unreachable"))))
                    }
                }
            } else {
                Ok(Adt8::C0(f13(Str::lit("unreachable"))))
            }
        }
        Adt2::C4(v44, r149, r150) => {
            let v45 = (*r149).clone();
            let v46 = (*r150).clone();
            let a52 = v3;
            let a53 = 0u64;
            if nat_eq(a52, a53) {
                let m54 = f20(v0.clone(), v44.clone())?;
                match m54 {
                    Ok(_) => {
                        let m55 = f12(v0.clone(), v45, v2.clone(), 0u64, Adt0::C14, Adt0::C14, f13(Str::lit("")), 0u64, List::<Adt5>::nil())?;
                        match m55 {
                            Adt8::C0(v55) => {
                                Ok(Adt8::C0(v55))
                            }
                            Adt8::C1(v48) => {
                                let m56 = f26(v0.clone(), v48.clone())?;
                                match m56 {
                                    Ok(v49) => {
                                        let m57 = f12(v0, v46, v2, 4u64, v44.clone(), v48, f13(Str::lit("")), 0u64, List::<Adt5>::nil())?;
                                        match m57 {
                                            Adt8::C0(v52) => {
                                                Ok(Adt8::C0(v52))
                                            }
                                            Adt8::C1(_) => {
                                                Ok(Adt8::C0(f13(Str::lit("unreachable"))))
                                            }
                                            Adt8::C2(v50) => {
                                                let m58 = f27(v49, v50);
                                                match m58 {
                                                    None => {
                                                        Ok(Adt8::C1(v44))
                                                    }
                                                    Some(v51) => {
                                                        Ok(Adt8::C0(f23(f13(Str::lit("the match is not exhaustive: ")), f23(f28(v51)?, f13(Str::lit(" is not covered"))))))
                                                    }
                                                }
                                            }
                                            Adt8::C3 => {
                                                Ok(Adt8::C0(f13(Str::lit("unreachable"))))
                                            }
                                        }
                                    }
                                    Err(v54) => {
                                        Ok(Adt8::C0(v54))
                                    }
                                }
                            }
                            Adt8::C2(_) => {
                                Ok(Adt8::C0(f13(Str::lit("unreachable"))))
                            }
                            Adt8::C3 => {
                                Ok(Adt8::C0(f13(Str::lit("unreachable"))))
                            }
                        }
                    }
                    Err(v57) => {
                        Ok(Adt8::C0(v57))
                    }
                }
            } else {
                Ok(Adt8::C0(f13(Str::lit("unreachable"))))
            }
        }
        Adt2::C5(v58, v59, r151) => {
            let v60 = (*r151).clone();
            let a59 = v3;
            let a60 = 0u64;
            if nat_eq(a59, a60) {
                let m61 = f20(v0.clone(), v59.clone())?;
                match m61 {
                    Ok(_) => {
                        let m62 = f19(v0.clone(), v58.clone(), v59.clone())?;
                        match m62 {
                            Ok(v62) => {
                                let v63: Bytes = f23(f13(Str::lit("constructor ")), f28(v58)?);
                                let a63 = f29(v60.clone())?;
                                let a64 = f30(v62.clone())?;
                                if nat_eq(a63, a64) {
                                    let m65 = f12(v0, v60, v2, 2u64, v62, Adt0::C14, v63, 0u64, List::<Adt5>::nil())?;
                                    match m65 {
                                        Adt8::C0(v64) => {
                                            Ok(Adt8::C0(v64))
                                        }
                                        Adt8::C1(_) => {
                                            Ok(Adt8::C0(f13(Str::lit("unreachable"))))
                                        }
                                        Adt8::C2(_) => {
                                            Ok(Adt8::C0(f13(Str::lit("unreachable"))))
                                        }
                                        Adt8::C3 => {
                                            Ok(Adt8::C1(v59))
                                        }
                                    }
                                } else {
                                    Ok(Adt8::C0(f23(v63, f23(f13(Str::lit(" takes ")), f23(f24(f30(v62)?)?, f23(f13(Str::lit(" operand(s), received ")), f24(f29(v60)?)?))))))
                                }
                            }
                            Err(v67) => {
                                Ok(Adt8::C0(v67))
                            }
                        }
                    }
                    Err(v68) => {
                        Ok(Adt8::C0(v68))
                    }
                }
            } else {
                Ok(Adt8::C0(f13(Str::lit("unreachable"))))
            }
        }
        Adt2::C6(v69, r152) => {
            let v70 = (*r152).clone();
            let a66 = v3;
            let a67 = 0u64;
            if nat_eq(a66, a67) {
                let m68 = f18(v0.clone(), v69);
                match m68 {
                    None => {
                        Ok(Adt8::C0(f31(v69)?))
                    }
                    Some(v71) => {
                        let v72: Bytes = f23(f13(Str::lit("function ")), f24(v69)?);
                        let a69 = f29(v70.clone())?;
                        let a70 = f30(v71.clone())?;
                        if nat_eq(a69, a70) {
                            let m71 = f12(v0.clone(), v70, v2, 2u64, v71, Adt0::C14, v72, 0u64, List::<Adt5>::nil())?;
                            match m71 {
                                Adt8::C0(v73) => {
                                    Ok(Adt8::C0(v73))
                                }
                                Adt8::C1(_) => {
                                    Ok(Adt8::C0(f13(Str::lit("unreachable"))))
                                }
                                Adt8::C2(_) => {
                                    Ok(Adt8::C0(f13(Str::lit("unreachable"))))
                                }
                                Adt8::C3 => {
                                    Ok(Adt8::C1(f32(v0, v69)))
                                }
                            }
                        } else {
                            Ok(Adt8::C0(f23(v72, f23(f13(Str::lit(" takes ")), f23(f24(f30(v71)?)?, f23(f13(Str::lit(" operand(s), received ")), f24(f29(v70)?)?))))))
                        }
                    }
                }
            } else {
                Ok(Adt8::C0(f13(Str::lit("unreachable"))))
            }
        }
        Adt2::C7(v76, r153) => {
            let v77 = (*r153).clone();
            let a72 = v3;
            let a73 = 0u64;
            if nat_eq(a72, a73) {
                let m74 = f18(v0.clone(), v76);
                match m74 {
                    None => {
                        Ok(Adt8::C0(f31(v76)?))
                    }
                    Some(v78) => {
                        let v79: u64 = f30(v78.clone())?;
                        let v80: u64 = f29(v77.clone())?;
                        let a75 = v79;
                        let a76 = v80;
                        if nat_le(a75, a76) {
                            Ok(Adt8::C0(f23(f13(Str::lit("closure of function ")), f23(f24(v76)?, f23(f13(Str::lit(" captures ")), f23(f24(v80)?, f23(f13(Str::lit(" of its ")), f23(f24(v79)?, f13(Str::lit(" parameters; at least one must remain"))))))))))
                        } else {
                            let m77 = f12(v0.clone(), v77, v2, 3u64, v78, Adt0::C14, f23(f13(Str::lit("closure of function ")), f24(v76)?), 0u64, List::<Adt5>::nil())?;
                            match m77 {
                                Adt8::C0(v82) => {
                                    Ok(Adt8::C0(v82))
                                }
                                Adt8::C1(v81) => {
                                    Ok(Adt8::C1(f33(v81, f32(v0, v76))))
                                }
                                Adt8::C2(_) => {
                                    Ok(Adt8::C0(f13(Str::lit("unreachable"))))
                                }
                                Adt8::C3 => {
                                    Ok(Adt8::C0(f13(Str::lit("unreachable"))))
                                }
                            }
                        }
                    }
                }
            } else {
                Ok(Adt8::C0(f13(Str::lit("unreachable"))))
            }
        }
        Adt2::C8(r154, r155) => {
            let v84 = (*r154).clone();
            let v85 = (*r155).clone();
            let a78 = v3;
            let a79 = 0u64;
            if nat_eq(a78, a79) {
                let m80 = f12(v0.clone(), v84, v2.clone(), 0u64, Adt0::C14, Adt0::C14, f13(Str::lit("")), 0u64, List::<Adt5>::nil())?;
                match m80 {
                    Adt8::C0(v91) => {
                        Ok(Adt8::C0(v91))
                    }
                    Adt8::C1(v86) => {
                        let a81 = f15(v86.clone());
                        let a82 = 13u64;
                        if nat_eq(a81, a82) {
                            let v87: Bytes = f13(Str::lit("an application"));
                            let a83 = f29(v85.clone())?;
                            let a84 = f30(f16(v86.clone()))?;
                            if nat_eq(a83, a84) {
                                let m85 = f12(v0, v85, v2, 2u64, f16(v86.clone()), Adt0::C14, v87, 0u64, List::<Adt5>::nil())?;
                                match m85 {
                                    Adt8::C0(v88) => {
                                        Ok(Adt8::C0(v88))
                                    }
                                    Adt8::C1(_) => {
                                        Ok(Adt8::C0(f13(Str::lit("unreachable"))))
                                    }
                                    Adt8::C2(_) => {
                                        Ok(Adt8::C0(f13(Str::lit("unreachable"))))
                                    }
                                    Adt8::C3 => {
                                        Ok(Adt8::C1(f14(v86)))
                                    }
                                }
                            } else {
                                Ok(Adt8::C0(f23(v87, f23(f13(Str::lit(" takes ")), f23(f24(f30(f16(v86))?)?, f23(f13(Str::lit(" operand(s), received ")), f24(f29(v85)?)?))))))
                            }
                        } else {
                            Ok(Adt8::C0(f13(Str::lit("only a function value can be applied"))))
                        }
                    }
                    Adt8::C2(_) => {
                        Ok(Adt8::C0(f13(Str::lit("unreachable"))))
                    }
                    Adt8::C3 => {
                        Ok(Adt8::C0(f13(Str::lit("unreachable"))))
                    }
                }
            } else {
                Ok(Adt8::C0(f13(Str::lit("unreachable"))))
            }
        }
        Adt2::C9(v93, r156) => {
            let v94 = (*r156).clone();
            let a86 = v3;
            let a87 = 0u64;
            if nat_eq(a86, a87) {
                let m88 = f12(v0, v94, v2, 1u64, Adt0::C14, Adt0::C14, f13(Str::lit("")), 0u64, List::<Adt5>::nil())?;
                match m88 {
                    Adt8::C0(v98) => {
                        Ok(Adt8::C0(v98))
                    }
                    Adt8::C1(v95) => {
                        let m89 = f34(v93, v95)?;
                        match m89 {
                            Ok(v96) => {
                                Ok(Adt8::C1(v96))
                            }
                            Err(v97) => {
                                Ok(Adt8::C0(v97))
                            }
                        }
                    }
                    Adt8::C2(_) => {
                        Ok(Adt8::C0(f13(Str::lit("unreachable"))))
                    }
                    Adt8::C3 => {
                        Ok(Adt8::C0(f13(Str::lit("unreachable"))))
                    }
                }
            } else {
                Ok(Adt8::C0(f13(Str::lit("unreachable"))))
            }
        }
        Adt2::C10(r157) => {
            let v100 = (*r157).clone();
            let a90 = v3;
            let a91 = 0u64;
            if nat_eq(a90, a91) {
                let m92 = f12(v0, v100, v2, 0u64, Adt0::C14, Adt0::C14, f13(Str::lit("")), 0u64, List::<Adt5>::nil())?;
                match m92 {
                    Adt8::C0(v102) => {
                        Ok(Adt8::C0(v102))
                    }
                    Adt8::C1(v101) => {
                        let a93 = f15(v101.clone());
                        let a94 = 11u64;
                        if nat_eq(a93, a94) {
                            Ok(Adt8::C1(f16(v101)))
                        } else {
                            Ok(Adt8::C0(f23(f13(Str::lit("`first` of ")), f35(v101)?)))
                        }
                    }
                    Adt8::C2(_) => {
                        Ok(Adt8::C0(f13(Str::lit("unreachable"))))
                    }
                    Adt8::C3 => {
                        Ok(Adt8::C0(f13(Str::lit("unreachable"))))
                    }
                }
            } else {
                Ok(Adt8::C0(f13(Str::lit("unreachable"))))
            }
        }
        Adt2::C11(r158) => {
            let v104 = (*r158).clone();
            let a95 = v3;
            let a96 = 0u64;
            if nat_eq(a95, a96) {
                let m97 = f12(v0, v104, v2, 0u64, Adt0::C14, Adt0::C14, f13(Str::lit("")), 0u64, List::<Adt5>::nil())?;
                match m97 {
                    Adt8::C0(v106) => {
                        Ok(Adt8::C0(v106))
                    }
                    Adt8::C1(v105) => {
                        let a98 = f15(v105.clone());
                        let a99 = 11u64;
                        if nat_eq(a98, a99) {
                            Ok(Adt8::C1(f14(v105)))
                        } else {
                            Ok(Adt8::C0(f23(f13(Str::lit("`second` of ")), f35(v105)?)))
                        }
                    }
                    Adt8::C2(_) => {
                        Ok(Adt8::C0(f13(Str::lit("unreachable"))))
                    }
                    Adt8::C3 => {
                        Ok(Adt8::C0(f13(Str::lit("unreachable"))))
                    }
                }
            } else {
                Ok(Adt8::C0(f13(Str::lit("unreachable"))))
            }
        }
        Adt2::C12(r159, v109) => {
            let v108 = (*r159).clone();
            let a100 = v3;
            let a101 = 0u64;
            if nat_eq(a100, a101) {
                let m102 = f12(v0.clone(), v108, v2, 0u64, Adt0::C14, Adt0::C14, f13(Str::lit("")), 0u64, List::<Adt5>::nil())?;
                match m102 {
                    Adt8::C0(v115) => {
                        Ok(Adt8::C0(v115))
                    }
                    Adt8::C1(v110) => {
                        let a103 = f15(v110.clone());
                        let a104 = 12u64;
                        if nat_eq(a103, a104) {
                            let v111: u64 = f36(v110);
                            let m105 = f37(v0, v111);
                            match m105 {
                                None => {
                                    Ok(Adt8::C0(f38(v111)?))
                                }
                                Some(v112) => {
                                    let a106 = f30(v112.clone())?;
                                    let a107 = 1u64;
                                    if nat_eq(a106, a107) {
                                        let m108 = f39(v112, 0u64);
                                        match m108 {
                                            None => {
                                                Ok(Adt8::C0(f13(Str::lit("unreachable"))))
                                            }
                                            Some(v113) => {
                                                let m109 = f39(v113, v109);
                                                match m109 {
                                                    None => {
                                                        Ok(Adt8::C0(f23(f13(Str::lit("ADT ")), f23(f24(v111)?, f23(f13(Str::lit(" has no field ")), f24(v109)?)))))
                                                    }
                                                    Some(v114) => {
                                                        Ok(Adt8::C1(v114))
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        Ok(Adt8::C0(f23(f13(Str::lit("`field` requires an ADT with exactly one constructor; ADT ")), f23(f24(v111)?, f23(f13(Str::lit(" has ")), f24(f30(v112)?)?)))))
                                    }
                                }
                            }
                        } else {
                            Ok(Adt8::C0(f13(Str::lit("`field` requires an ADT value"))))
                        }
                    }
                    Adt8::C2(_) => {
                        Ok(Adt8::C0(f13(Str::lit("unreachable"))))
                    }
                    Adt8::C3 => {
                        Ok(Adt8::C0(f13(Str::lit("unreachable"))))
                    }
                }
            } else {
                Ok(Adt8::C0(f13(Str::lit("unreachable"))))
            }
        }
        Adt2::C13 => {
            let a110 = v3;
            let a111 = 1u64;
            if nat_eq(a110, a111) {
                Ok(Adt8::C1(Adt0::C14))
            } else {
                let a112 = v3;
                let a113 = 2u64;
                if nat_eq(a112, a113) {
                    Ok(Adt8::C3)
                } else {
                    let a114 = v3;
                    let a115 = 3u64;
                    if nat_eq(a114, a115) {
                        Ok(Adt8::C1(v4))
                    } else {
                        let a116 = v3;
                        let a117 = 4u64;
                        if nat_eq(a116, a117) {
                            Ok(Adt8::C2(v8))
                        } else {
                            Ok(Adt8::C0(f13(Str::lit("unreachable"))))
                        }
                    }
                }
            }
        }
        Adt2::C14(r160, r161) => {
            let v117 = (*r160).clone();
            let v118 = (*r161).clone();
            let a118 = v3;
            let a119 = 1u64;
            if nat_eq(a118, a119) {
                let m120 = f12(v0.clone(), v117, v2.clone(), 0u64, Adt0::C14, Adt0::C14, f13(Str::lit("")), 0u64, List::<Adt5>::nil())?;
                match m120 {
                    Adt8::C0(v123) => {
                        Ok(Adt8::C0(v123))
                    }
                    Adt8::C1(v119) => {
                        let m121 = f12(v0, v118, v2, 1u64, Adt0::C14, Adt0::C14, f13(Str::lit("")), 0u64, List::<Adt5>::nil())?;
                        match m121 {
                            Adt8::C0(v121) => {
                                Ok(Adt8::C0(v121))
                            }
                            Adt8::C1(v120) => {
                                Ok(Adt8::C1(f40(v119, v120)))
                            }
                            Adt8::C2(_) => {
                                Ok(Adt8::C0(f13(Str::lit("unreachable"))))
                            }
                            Adt8::C3 => {
                                Ok(Adt8::C0(f13(Str::lit("unreachable"))))
                            }
                        }
                    }
                    Adt8::C2(_) => {
                        Ok(Adt8::C0(f13(Str::lit("unreachable"))))
                    }
                    Adt8::C3 => {
                        Ok(Adt8::C0(f13(Str::lit("unreachable"))))
                    }
                }
            } else {
                let a122 = v3;
                let a123 = 2u64;
                if nat_eq(a122, a123) {
                    if f7(v4.clone()) {
                        Ok(Adt8::C3)
                    } else {
                        let m124 = f12(v0.clone(), v117, v2.clone(), 0u64, Adt0::C14, Adt0::C14, f13(Str::lit("")), 0u64, List::<Adt5>::nil())?;
                        match m124 {
                            Adt8::C0(v128) => {
                                Ok(Adt8::C0(v128))
                            }
                            Adt8::C1(v125) => {
                                let m125 = f25(v125, f16(v4.clone()), f23(v6.clone(), f23(f13(Str::lit(" operand ")), f24(v7)?)))?;
                                match m125 {
                                    Ok(_) => {
                                        f12(v0, v118, v2, 2u64, f14(v4), Adt0::C14, v6, {
                                            let a126 = v7;
                                            let a127 = 1u64;
                                            nat_add(a126, a127)?
                                        }, List::<Adt5>::nil())
                                    }
                                    Err(v127) => {
                                        Ok(Adt8::C0(v127))
                                    }
                                }
                            }
                            Adt8::C2(_) => {
                                Ok(Adt8::C0(f13(Str::lit("unreachable"))))
                            }
                            Adt8::C3 => {
                                Ok(Adt8::C0(f13(Str::lit("unreachable"))))
                            }
                        }
                    }
                } else {
                    let a128 = v3;
                    let a129 = 3u64;
                    if nat_eq(a128, a129) {
                        if f7(v4.clone()) {
                            Ok(Adt8::C1(Adt0::C14))
                        } else {
                            let m130 = f12(v0.clone(), v117, v2.clone(), 0u64, Adt0::C14, Adt0::C14, f13(Str::lit("")), 0u64, List::<Adt5>::nil())?;
                            match m130 {
                                Adt8::C0(v133) => {
                                    Ok(Adt8::C0(v133))
                                }
                                Adt8::C1(v130) => {
                                    let m131 = f25(v130, f16(v4.clone()), f23(v6.clone(), f23(f13(Str::lit(" operand ")), f24(v7)?)))?;
                                    match m131 {
                                        Ok(_) => {
                                            f12(v0, v118, v2, 3u64, f14(v4), Adt0::C14, v6, {
                                                let a132 = v7;
                                                let a133 = 1u64;
                                                nat_add(a132, a133)?
                                            }, List::<Adt5>::nil())
                                        }
                                        Err(v132) => {
                                            Ok(Adt8::C0(v132))
                                        }
                                    }
                                }
                                Adt8::C2(_) => {
                                    Ok(Adt8::C0(f13(Str::lit("unreachable"))))
                                }
                                Adt8::C3 => {
                                    Ok(Adt8::C0(f13(Str::lit("unreachable"))))
                                }
                            }
                        }
                    } else {
                        let a134 = v3;
                        let a135 = 4u64;
                        if nat_eq(a134, a135) {
                            let m136 = f12(v0.clone(), v117, v2.clone(), 5u64, v4.clone(), v5.clone(), f13(Str::lit("")), 0u64, v8)?;
                            match m136 {
                                Adt8::C0(v136) => {
                                    Ok(Adt8::C0(v136))
                                }
                                Adt8::C1(_) => {
                                    Ok(Adt8::C0(f13(Str::lit("unreachable"))))
                                }
                                Adt8::C2(v135) => {
                                    f12(v0, v118, v2, 4u64, v4, v5, f13(Str::lit("")), 0u64, v135)
                                }
                                Adt8::C3 => {
                                    Ok(Adt8::C0(f13(Str::lit("unreachable"))))
                                }
                            }
                        } else {
                            Ok(Adt8::C0(f13(Str::lit("unreachable"))))
                        }
                    }
                }
            }
        }
        Adt2::C15(v138, v139, r162) => {
            let v140 = (*r162).clone();
            let a137 = v3;
            let a138 = 5u64;
            if nat_eq(a137, a138) {
                let m139 = f19(v0.clone(), v138.clone(), v5)?;
                match m139 {
                    Ok(v141) => {
                        let v142: u64 = f41(v139.clone())?;
                        let v143: u64 = f30(v141.clone())?;
                        let a140 = v142;
                        let a141 = v143;
                        if nat_eq(a140, a141) {
                            if f42(v139.clone()) {
                                Ok(Adt8::C0(f23(f13(Str::lit("shape ")), f23(f28(v138)?, f13(Str::lit(" binds the same name twice"))))))
                            } else {
                                if f43(v8.clone(), v138.clone()) {
                                    Ok(Adt8::C0(f23(f13(Str::lit("the arm for ")), f23(f28(v138)?, f13(Str::lit(" is unreachable"))))))
                                } else {
                                    let m142 = f12(v0, v140, f8(v139, v141, v2), 0u64, Adt0::C14, Adt0::C14, f13(Str::lit("")), 0u64, List::<Adt5>::nil())?;
                                    match m142 {
                                        Adt8::C0(v147) => {
                                            Ok(Adt8::C0(v147))
                                        }
                                        Adt8::C1(v144) => {
                                            let m143 = f25(v144, v4, f13(Str::lit("an arm")))?;
                                            match m143 {
                                                Ok(_) => {
                                                    Ok(Adt8::C2(f44(v138, v8)))
                                                }
                                                Err(v146) => {
                                                    Ok(Adt8::C0(v146))
                                                }
                                            }
                                        }
                                        Adt8::C2(_) => {
                                            Ok(Adt8::C0(f13(Str::lit("unreachable"))))
                                        }
                                        Adt8::C3 => {
                                            Ok(Adt8::C0(f13(Str::lit("unreachable"))))
                                        }
                                    }
                                }
                            }
                        } else {
                            Ok(Adt8::C0(f23(f13(Str::lit("shape ")), f23(f28(v138)?, f23(f13(Str::lit(" binds ")), f23(f24(v143)?, f23(f13(Str::lit(" value(s), the arm names ")), f24(v142)?)))))))
                        }
                    }
                    Err(v149) => {
                        Ok(Adt8::C0(v149))
                    }
                }
            } else {
                Ok(Adt8::C0(f13(Str::lit("unreachable"))))
            }
        }
    }
}

pub fn f13(v0: Str) -> Bytes {
    let a163 = v0;
    utf8_encode(a163)
}

pub fn f14(v0: Adt0) -> Adt0 {
    let m164 = f45(v0);
    match m164 {
        None => {
            Adt0::C0
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f15(v0: Adt0) -> u64 {
    let m165 = v0;
    match m165 {
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

pub fn f16(v0: Adt0) -> Adt0 {
    let m166 = f46(v0);
    match m166 {
        None => {
            Adt0::C0
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f17(p170: List<List<u64>>, p171: u64) -> Option<List<u64>> {
    let m169 = (p170, p171);
    {
        let mut m169 = m169;
        loop {
            let step = {
                let (v0, v1) = m169;
                let m167 = v0;
                match m167.uncons() {
                    None => {
                        Err::<(List<List<u64>>, u64), Option<List<u64>>>(None::<List<u64>>)
                    }
                    Some((v2, v3)) => {
                        let m168 = v1;
                        if m168 == 0 {
                            Err::<(List<List<u64>>, u64), Option<List<u64>>>(Some(v2))
                        } else {
                            let v4 = m168 - 1;
                            Ok::<(List<List<u64>>, u64), Option<List<u64>>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m169 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f18(v0: Adt7, v1: u64) -> Option<Adt0> {
    f39(match v0 {
        Adt7::C0(_, _, _, h172, _, _) => {
            h172
        }
    }, v1)
}

pub fn f19(v0: Adt7, v1: Adt5, v2: Adt0) -> R<Result<Adt0, Bytes>> {
    let m173 = v1.clone();
    match m173 {
        Adt5::C0 => {
            let v3: Result<Adt0, Bytes> = Ok::<Adt0, Bytes>(Adt0::C14);
            let v4: Result<Adt0, Bytes> = Err::<Adt0, Bytes>(f23(f13(Str::lit("shape ")), f23(f28(v1)?, f23(f13(Str::lit(" does not belong to type ")), f35(v2.clone())?))));
            let a174 = f15(v2);
            let a175 = 8u64;
            if nat_eq(a174, a175) {
                Ok(v3)
            } else {
                Ok(v4)
            }
        }
        Adt5::C1 => {
            let v5: Result<Adt0, Bytes> = Ok::<Adt0, Bytes>(Adt0::C15(std::rc::Rc::new(f16(v2.clone())), std::rc::Rc::new(Adt0::C14)));
            let v6: Result<Adt0, Bytes> = Err::<Adt0, Bytes>(f23(f13(Str::lit("shape ")), f23(f28(v1)?, f23(f13(Str::lit(" does not belong to type ")), f35(v2.clone())?))));
            let a176 = f15(v2);
            let a177 = 8u64;
            if nat_eq(a176, a177) {
                Ok(v5)
            } else {
                Ok(v6)
            }
        }
        Adt5::C2 => {
            let v7: Result<Adt0, Bytes> = Ok::<Adt0, Bytes>(Adt0::C15(std::rc::Rc::new(f16(v2.clone())), std::rc::Rc::new(Adt0::C14)));
            let v8: Result<Adt0, Bytes> = Err::<Adt0, Bytes>(f23(f13(Str::lit("shape ")), f23(f28(v1)?, f23(f13(Str::lit(" does not belong to type ")), f35(v2.clone())?))));
            let a178 = f15(v2);
            let a179 = 9u64;
            if nat_eq(a178, a179) {
                Ok(v7)
            } else {
                Ok(v8)
            }
        }
        Adt5::C3 => {
            let v9: Result<Adt0, Bytes> = Ok::<Adt0, Bytes>(Adt0::C15(std::rc::Rc::new(f14(v2.clone())), std::rc::Rc::new(Adt0::C14)));
            let v10: Result<Adt0, Bytes> = Err::<Adt0, Bytes>(f23(f13(Str::lit("shape ")), f23(f28(v1)?, f23(f13(Str::lit(" does not belong to type ")), f35(v2.clone())?))));
            let a180 = f15(v2);
            let a181 = 9u64;
            if nat_eq(a180, a181) {
                Ok(v9)
            } else {
                Ok(v10)
            }
        }
        Adt5::C4 => {
            let v11: Result<Adt0, Bytes> = Ok::<Adt0, Bytes>(Adt0::C14);
            let v12: Result<Adt0, Bytes> = Err::<Adt0, Bytes>(f23(f13(Str::lit("shape ")), f23(f28(v1)?, f23(f13(Str::lit(" does not belong to type ")), f35(v2.clone())?))));
            let a182 = f15(v2);
            let a183 = 10u64;
            if nat_eq(a182, a183) {
                Ok(v11)
            } else {
                Ok(v12)
            }
        }
        Adt5::C5 => {
            let v13: Result<Adt0, Bytes> = Ok::<Adt0, Bytes>(Adt0::C15(std::rc::Rc::new(f16(v2.clone())), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(v2.clone()), std::rc::Rc::new(Adt0::C14)))));
            let v14: Result<Adt0, Bytes> = Err::<Adt0, Bytes>(f23(f13(Str::lit("shape ")), f23(f28(v1)?, f23(f13(Str::lit(" does not belong to type ")), f35(v2.clone())?))));
            let a184 = f15(v2);
            let a185 = 10u64;
            if nat_eq(a184, a185) {
                Ok(v13)
            } else {
                Ok(v14)
            }
        }
        Adt5::C6 => {
            let v15: Result<Adt0, Bytes> = Ok::<Adt0, Bytes>(Adt0::C14);
            let v16: Result<Adt0, Bytes> = Err::<Adt0, Bytes>(f23(f13(Str::lit("shape ")), f23(f28(v1)?, f23(f13(Str::lit(" does not belong to type ")), f35(v2.clone())?))));
            let a186 = f15(v2);
            let a187 = 2u64;
            if nat_eq(a186, a187) {
                Ok(v15)
            } else {
                Ok(v16)
            }
        }
        Adt5::C7 => {
            let v17: Result<Adt0, Bytes> = Ok::<Adt0, Bytes>(Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C14)));
            let v18: Result<Adt0, Bytes> = Err::<Adt0, Bytes>(f23(f13(Str::lit("shape ")), f23(f28(v1)?, f23(f13(Str::lit(" does not belong to type ")), f35(v2.clone())?))));
            let a188 = f15(v2);
            let a189 = 2u64;
            if nat_eq(a188, a189) {
                Ok(v17)
            } else {
                Ok(v18)
            }
        }
        Adt5::C8 => {
            let v19: Result<Adt0, Bytes> = Ok::<Adt0, Bytes>(Adt0::C15(std::rc::Rc::new(f16(v2.clone())), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(f14(v2.clone())), std::rc::Rc::new(Adt0::C14)))));
            let v20: Result<Adt0, Bytes> = Err::<Adt0, Bytes>(f23(f13(Str::lit("shape ")), f23(f28(v1)?, f23(f13(Str::lit(" does not belong to type ")), f35(v2.clone())?))));
            let a190 = f15(v2);
            let a191 = 11u64;
            if nat_eq(a190, a191) {
                Ok(v19)
            } else {
                Ok(v20)
            }
        }
        Adt5::C9 => {
            let v21: Result<Adt0, Bytes> = Ok::<Adt0, Bytes>(Adt0::C14);
            let v22: Result<Adt0, Bytes> = Err::<Adt0, Bytes>(f23(f13(Str::lit("shape ")), f23(f28(v1)?, f23(f13(Str::lit(" does not belong to type ")), f35(v2.clone())?))));
            let a192 = f15(v2);
            let a193 = 1u64;
            if nat_eq(a192, a193) {
                Ok(v21)
            } else {
                Ok(v22)
            }
        }
        Adt5::C10 => {
            let v23: Result<Adt0, Bytes> = Ok::<Adt0, Bytes>(Adt0::C14);
            let v24: Result<Adt0, Bytes> = Err::<Adt0, Bytes>(f23(f13(Str::lit("shape ")), f23(f28(v1)?, f23(f13(Str::lit(" does not belong to type ")), f35(v2.clone())?))));
            let a194 = f15(v2);
            let a195 = 1u64;
            if nat_eq(a194, a195) {
                Ok(v23)
            } else {
                Ok(v24)
            }
        }
        Adt5::C11 => {
            let v25: Result<Adt0, Bytes> = Ok::<Adt0, Bytes>(Adt0::C14);
            let v26: Result<Adt0, Bytes> = Err::<Adt0, Bytes>(f23(f13(Str::lit("shape ")), f23(f28(v1)?, f23(f13(Str::lit(" does not belong to type ")), f35(v2.clone())?))));
            let a196 = f15(v2);
            let a197 = 0u64;
            if nat_eq(a196, a197) {
                Ok(v25)
            } else {
                Ok(v26)
            }
        }
        Adt5::C12 => {
            let v27: Result<Adt0, Bytes> = Ok::<Adt0, Bytes>(Adt0::C14);
            let v28: Result<Adt0, Bytes> = Err::<Adt0, Bytes>(f23(f13(Str::lit("shape ")), f23(f28(v1)?, f23(f13(Str::lit(" does not belong to type ")), f35(v2.clone())?))));
            let a198 = f15(v2);
            let a199 = 7u64;
            if nat_eq(a198, a199) {
                Ok(v27)
            } else {
                Ok(v28)
            }
        }
        Adt5::C13 => {
            let v29: Result<Adt0, Bytes> = Ok::<Adt0, Bytes>(Adt0::C14);
            let v30: Result<Adt0, Bytes> = Err::<Adt0, Bytes>(f23(f13(Str::lit("shape ")), f23(f28(v1)?, f23(f13(Str::lit(" does not belong to type ")), f35(v2.clone())?))));
            let a200 = f15(v2);
            let a201 = 7u64;
            if nat_eq(a200, a201) {
                Ok(v29)
            } else {
                Ok(v30)
            }
        }
        Adt5::C14 => {
            let v31: Result<Adt0, Bytes> = Ok::<Adt0, Bytes>(Adt0::C14);
            let v32: Result<Adt0, Bytes> = Err::<Adt0, Bytes>(f23(f13(Str::lit("shape ")), f23(f28(v1)?, f23(f13(Str::lit(" does not belong to type ")), f35(v2.clone())?))));
            let a202 = f15(v2);
            let a203 = 7u64;
            if nat_eq(a202, a203) {
                Ok(v31)
            } else {
                Ok(v32)
            }
        }
        Adt5::C15(v33) => {
            let v34: Result<Adt0, Bytes> = f47(v0, f36(v2.clone()), v33)?;
            let v35: Result<Adt0, Bytes> = v34;
            let v36: Result<Adt0, Bytes> = Err::<Adt0, Bytes>(f23(f13(Str::lit("shape ")), f23(f28(v1)?, f23(f13(Str::lit(" does not belong to type ")), f35(v2.clone())?))));
            let a204 = f15(v2);
            let a205 = 12u64;
            if nat_eq(a204, a205) {
                Ok(v35)
            } else {
                Ok(v36)
            }
        }
    }
}

pub fn f20(v0: Adt7, v1: Adt0) -> R<Result<(), Bytes>> {
    let m206 = v1;
    match m206 {
        Adt0::C0 => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt0::C1 => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt0::C2 => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt0::C3 => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt0::C4(_) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt0::C5 => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt0::C6 => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt0::C7 => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt0::C8(r212) => {
            let v3 = (*r212).clone();
            f20(v0, v3)
        }
        Adt0::C9(r213, r214) => {
            let v4 = (*r213).clone();
            let v5 = (*r214).clone();
            let m207 = f20(v0.clone(), v4)?;
            match m207 {
                Ok(_) => {
                    f20(v0, v5)
                }
                Err(v7) => {
                    Ok(Err::<(), Bytes>(v7))
                }
            }
        }
        Adt0::C10(r215) => {
            let v8 = (*r215).clone();
            f20(v0, v8)
        }
        Adt0::C11(r216, r217) => {
            let v9 = (*r216).clone();
            let v10 = (*r217).clone();
            let m208 = f20(v0.clone(), v9)?;
            match m208 {
                Ok(_) => {
                    f20(v0, v10)
                }
                Err(v12) => {
                    Ok(Err::<(), Bytes>(v12))
                }
            }
        }
        Adt0::C12(v13) => {
            let m209 = f37(v0, v13);
            match m209 {
                None => {
                    Ok(Err::<(), Bytes>(f38(v13)?))
                }
                Some(_) => {
                    Ok(Ok::<(), Bytes>(()))
                }
            }
        }
        Adt0::C13(r218, r219) => {
            let v15 = (*r218).clone();
            let v16 = (*r219).clone();
            if f7(v15.clone()) {
                Ok(Err::<(), Bytes>(f13(Str::lit("a function type takes at least one parameter"))))
            } else {
                let m210 = f20(v0.clone(), v15)?;
                match m210 {
                    Ok(_) => {
                        f20(v0, v16)
                    }
                    Err(v18) => {
                        Ok(Err::<(), Bytes>(v18))
                    }
                }
            }
        }
        Adt0::C14 => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt0::C15(r220, r221) => {
            let v19 = (*r220).clone();
            let v20 = (*r221).clone();
            let m211 = f20(v0.clone(), v19)?;
            match m211 {
                Ok(_) => {
                    f20(v0, v20)
                }
                Err(v22) => {
                    Ok(Err::<(), Bytes>(v22))
                }
            }
        }
    }
}

pub fn f21(v0: Adt7, v1: Adt3, v2: Adt0, v3: u64) -> R<Result<(), Bytes>> {
    let m222 = v1;
    match m222 {
        Adt3::C0 => {
            let v4: Result<(), Bytes> = Ok::<(), Bytes>(());
            let v5: Result<(), Bytes> = Err::<(), Bytes>(f23(f13(Str::lit("the literal ")), f23(f48(Adt3::C0)?, f23(f13(Str::lit(" does not have type ")), f35(v2.clone())?))));
            let a223 = f15(v2);
            let a224 = 0u64;
            if nat_eq(a223, a224) {
                Ok(v4)
            } else {
                Ok(v5)
            }
        }
        Adt3::C1(v6) => {
            let v7: Result<(), Bytes> = Ok::<(), Bytes>(());
            let v8: Result<(), Bytes> = Err::<(), Bytes>(f23(f13(Str::lit("the literal ")), f23(f48(Adt3::C1(v6))?, f23(f13(Str::lit(" does not have type ")), f35(v2.clone())?))));
            let a225 = f15(v2);
            let a226 = 1u64;
            if nat_eq(a225, a226) {
                Ok(v7)
            } else {
                Ok(v8)
            }
        }
        Adt3::C2(v9) => {
            let v10: Result<(), Bytes> = f49(v9.clone())?;
            let v11: Result<(), Bytes> = Err::<(), Bytes>(f23(f13(Str::lit("the literal ")), f23(f48(Adt3::C2(v9))?, f23(f13(Str::lit(" does not have type ")), f35(v2.clone())?))));
            let a227 = f15(v2);
            let a228 = 2u64;
            if nat_eq(a227, a228) {
                Ok(v10)
            } else {
                Ok(v11)
            }
        }
        Adt3::C3(v12) => {
            let v13: Result<(), Bytes> = f50(v12.clone(), Adt1::C7)?;
            let v14: Result<(), Bytes> = Err::<(), Bytes>(f23(f13(Str::lit("the literal ")), f23(f48(Adt3::C3(v12))?, f23(f13(Str::lit(" does not have type ")), f35(v2.clone())?))));
            let a229 = f15(v2);
            let a230 = 3u64;
            if nat_eq(a229, a230) {
                Ok(v13)
            } else {
                Ok(v14)
            }
        }
        Adt3::C4(v15) => {
            let v16: Result<(), Bytes> = f50(v15.clone(), Adt1::C0)?;
            let v17: Result<(), Bytes> = Err::<(), Bytes>(f23(f13(Str::lit("the literal ")), f23(f48(Adt3::C4(v15))?, f23(f13(Str::lit(" does not have type ")), f35(v2.clone())?))));
            let a231 = f15(v2.clone());
            let a232 = 4u64;
            if if nat_eq(a231, a232) {
                let a233 = f51(f52(v2));
                let a234 = 0u64;
                nat_eq(a233, a234)
            } else {
                false
            } {
                Ok(v16)
            } else {
                Ok(v17)
            }
        }
        Adt3::C5(v18) => {
            let v19: Result<(), Bytes> = f50(v18.clone(), Adt1::C1)?;
            let v20: Result<(), Bytes> = Err::<(), Bytes>(f23(f13(Str::lit("the literal ")), f23(f48(Adt3::C5(v18))?, f23(f13(Str::lit(" does not have type ")), f35(v2.clone())?))));
            let a235 = f15(v2.clone());
            let a236 = 4u64;
            if if nat_eq(a235, a236) {
                let a237 = f51(f52(v2));
                let a238 = 1u64;
                nat_eq(a237, a238)
            } else {
                false
            } {
                Ok(v19)
            } else {
                Ok(v20)
            }
        }
        Adt3::C6(v21) => {
            let v22: Result<(), Bytes> = f50(v21.clone(), Adt1::C2)?;
            let v23: Result<(), Bytes> = Err::<(), Bytes>(f23(f13(Str::lit("the literal ")), f23(f48(Adt3::C6(v21))?, f23(f13(Str::lit(" does not have type ")), f35(v2.clone())?))));
            let a239 = f15(v2.clone());
            let a240 = 4u64;
            if if nat_eq(a239, a240) {
                let a241 = f51(f52(v2));
                let a242 = 2u64;
                nat_eq(a241, a242)
            } else {
                false
            } {
                Ok(v22)
            } else {
                Ok(v23)
            }
        }
        Adt3::C7(v24) => {
            let v25: Result<(), Bytes> = f50(v24.clone(), Adt1::C3)?;
            let v26: Result<(), Bytes> = Err::<(), Bytes>(f23(f13(Str::lit("the literal ")), f23(f48(Adt3::C7(v24))?, f23(f13(Str::lit(" does not have type ")), f35(v2.clone())?))));
            let a243 = f15(v2.clone());
            let a244 = 4u64;
            if if nat_eq(a243, a244) {
                let a245 = f51(f52(v2));
                let a246 = 3u64;
                nat_eq(a245, a246)
            } else {
                false
            } {
                Ok(v25)
            } else {
                Ok(v26)
            }
        }
        Adt3::C8(v27) => {
            let v28: Result<(), Bytes> = f50(v27.clone(), Adt1::C4)?;
            let v29: Result<(), Bytes> = Err::<(), Bytes>(f23(f13(Str::lit("the literal ")), f23(f48(Adt3::C8(v27))?, f23(f13(Str::lit(" does not have type ")), f35(v2.clone())?))));
            let a247 = f15(v2.clone());
            let a248 = 4u64;
            if if nat_eq(a247, a248) {
                let a249 = f51(f52(v2));
                let a250 = 4u64;
                nat_eq(a249, a250)
            } else {
                false
            } {
                Ok(v28)
            } else {
                Ok(v29)
            }
        }
        Adt3::C9(v30) => {
            let v31: Result<(), Bytes> = f50(v30.clone(), Adt1::C5)?;
            let v32: Result<(), Bytes> = Err::<(), Bytes>(f23(f13(Str::lit("the literal ")), f23(f48(Adt3::C9(v30))?, f23(f13(Str::lit(" does not have type ")), f35(v2.clone())?))));
            let a251 = f15(v2.clone());
            let a252 = 4u64;
            if if nat_eq(a251, a252) {
                let a253 = f51(f52(v2));
                let a254 = 5u64;
                nat_eq(a253, a254)
            } else {
                false
            } {
                Ok(v31)
            } else {
                Ok(v32)
            }
        }
        Adt3::C10(v33) => {
            let v34: Result<(), Bytes> = f50(v33.clone(), Adt1::C6)?;
            let v35: Result<(), Bytes> = Err::<(), Bytes>(f23(f13(Str::lit("the literal ")), f23(f48(Adt3::C10(v33))?, f23(f13(Str::lit(" does not have type ")), f35(v2.clone())?))));
            let a255 = f15(v2.clone());
            let a256 = 4u64;
            if if nat_eq(a255, a256) {
                let a257 = f51(f52(v2));
                let a258 = 6u64;
                nat_eq(a257, a258)
            } else {
                false
            } {
                Ok(v34)
            } else {
                Ok(v35)
            }
        }
        Adt3::C11(v36) => {
            let v37: Result<(), Bytes> = f50(v36.clone(), Adt1::C7)?;
            let v38: Result<(), Bytes> = Err::<(), Bytes>(f23(f13(Str::lit("the literal ")), f23(f48(Adt3::C11(v36))?, f23(f13(Str::lit(" does not have type ")), f35(v2.clone())?))));
            let a259 = f15(v2.clone());
            let a260 = 4u64;
            if if nat_eq(a259, a260) {
                let a261 = f51(f52(v2));
                let a262 = 7u64;
                nat_eq(a261, a262)
            } else {
                false
            } {
                Ok(v37)
            } else {
                Ok(v38)
            }
        }
        Adt3::C12(v39) => {
            let v40: Result<(), Bytes> = Ok::<(), Bytes>(());
            let v41: Result<(), Bytes> = Err::<(), Bytes>(f23(f13(Str::lit("the literal ")), f23(f48(Adt3::C12(v39))?, f23(f13(Str::lit(" does not have type ")), f35(v2.clone())?))));
            let a263 = f15(v2);
            let a264 = 5u64;
            if nat_eq(a263, a264) {
                Ok(v40)
            } else {
                Ok(v41)
            }
        }
        Adt3::C13(v42) => {
            let v43: Result<(), Bytes> = f53(v42.clone())?;
            let v44: Result<(), Bytes> = Err::<(), Bytes>(f23(f13(Str::lit("the literal ")), f23(f48(Adt3::C13(v42))?, f23(f13(Str::lit(" does not have type ")), f35(v2.clone())?))));
            let a265 = f15(v2);
            let a266 = 6u64;
            if nat_eq(a265, a266) {
                Ok(v43)
            } else {
                Ok(v44)
            }
        }
        Adt3::C14(v45) => {
            let v46: Result<(), Bytes> = Ok::<(), Bytes>(());
            let v47: Result<(), Bytes> = Err::<(), Bytes>(f23(f13(Str::lit("the literal ")), f23(f48(Adt3::C14(v45))?, f23(f13(Str::lit(" does not have type ")), f35(v2.clone())?))));
            let a267 = f15(v2);
            let a268 = 7u64;
            if nat_eq(a267, a268) {
                Ok(v46)
            } else {
                Ok(v47)
            }
        }
        Adt3::C15 => {
            let v48: Result<(), Bytes> = Ok::<(), Bytes>(());
            let v49: Result<(), Bytes> = Err::<(), Bytes>(f23(f13(Str::lit("the literal ")), f23(f48(Adt3::C15)?, f23(f13(Str::lit(" does not have type ")), f35(v2.clone())?))));
            let a269 = f15(v2);
            let a270 = 8u64;
            if nat_eq(a269, a270) {
                Ok(v48)
            } else {
                Ok(v49)
            }
        }
        Adt3::C16(r296) => {
            let v50 = (*r296).clone();
            let v51: Result<(), Bytes> = f21(v0, v50.clone(), f16(v2.clone()), 0u64)?;
            let v52: Result<(), Bytes> = Err::<(), Bytes>(f23(f13(Str::lit("the literal ")), f23(f48(Adt3::C16(std::rc::Rc::new(v50)))?, f23(f13(Str::lit(" does not have type ")), f35(v2.clone())?))));
            let a271 = f15(v2);
            let a272 = 8u64;
            if nat_eq(a271, a272) {
                Ok(v51)
            } else {
                Ok(v52)
            }
        }
        Adt3::C17(r297) => {
            let v53 = (*r297).clone();
            let v54: Result<(), Bytes> = f21(v0, v53.clone(), f16(v2.clone()), 0u64)?;
            let v55: Result<(), Bytes> = Err::<(), Bytes>(f23(f13(Str::lit("the literal ")), f23(f48(Adt3::C17(std::rc::Rc::new(v53)))?, f23(f13(Str::lit(" does not have type ")), f35(v2.clone())?))));
            let a273 = f15(v2);
            let a274 = 9u64;
            if nat_eq(a273, a274) {
                Ok(v54)
            } else {
                Ok(v55)
            }
        }
        Adt3::C18(r298) => {
            let v56 = (*r298).clone();
            let v57: Result<(), Bytes> = f21(v0, v56.clone(), f14(v2.clone()), 0u64)?;
            let v58: Result<(), Bytes> = Err::<(), Bytes>(f23(f13(Str::lit("the literal ")), f23(f48(Adt3::C18(std::rc::Rc::new(v56)))?, f23(f13(Str::lit(" does not have type ")), f35(v2.clone())?))));
            let a275 = f15(v2);
            let a276 = 9u64;
            if nat_eq(a275, a276) {
                Ok(v57)
            } else {
                Ok(v58)
            }
        }
        Adt3::C19(r299) => {
            let v59 = (*r299).clone();
            let v60: Result<(), Bytes> = f21(v0, v59.clone(), f16(v2.clone()), 1u64)?;
            let v61: Result<(), Bytes> = Err::<(), Bytes>(f23(f13(Str::lit("the literal ")), f23(f48(Adt3::C19(std::rc::Rc::new(v59)))?, f23(f13(Str::lit(" does not have type ")), f35(v2.clone())?))));
            let a277 = f15(v2);
            let a278 = 10u64;
            if nat_eq(a277, a278) {
                Ok(v60)
            } else {
                Ok(v61)
            }
        }
        Adt3::C20(r300, r301) => {
            let v62 = (*r300).clone();
            let v63 = (*r301).clone();
            let v64: Result<(), Bytes> = f21(v0.clone(), v62.clone(), f16(v2.clone()), 0u64)?;
            let v65: Result<(), Bytes> = f21(v0, v63.clone(), f14(v2.clone()), 0u64)?;
            let v68: Result<(), Bytes> = {
                let m279 = v64;
                match m279 {
                    Ok(_) => {
                        v65
                    }
                    Err(v67) => {
                        Err::<(), Bytes>(v67)
                    }
                }
            };
            let v69: Result<(), Bytes> = Err::<(), Bytes>(f23(f13(Str::lit("the literal ")), f23(f48(Adt3::C20(std::rc::Rc::new(v62), std::rc::Rc::new(v63)))?, f23(f13(Str::lit(" does not have type ")), f35(v2.clone())?))));
            let a280 = f15(v2);
            let a281 = 11u64;
            if nat_eq(a280, a281) {
                Ok(v68)
            } else {
                Ok(v69)
            }
        }
        Adt3::C21(v70, r302) => {
            let v71 = (*r302).clone();
            let v77: Result<(), Bytes> = {
                let m282 = f47(v0.clone(), f36(v2.clone()), v70)?;
                match m282 {
                    Ok(v73) => {
                        let v74: Result<(), Bytes> = f21(v0, v71.clone(), v73.clone(), 2u64)?;
                        let v75: Result<(), Bytes> = v74;
                        let v76: Result<(), Bytes> = Err::<(), Bytes>(f23(f13(Str::lit("constructor ")), f23(f24(v70)?, f23(f13(Str::lit(" of ADT ")), f23(f24(f36(v2.clone()))?, f23(f13(Str::lit(" takes ")), f23(f24(f30(v73.clone())?)?, f23(f13(Str::lit(" fields, the literal has ")), f24(f54(v71.clone())?)?))))))));
                        let a283 = f54(v71.clone())?;
                        let a284 = f30(v73)?;
                        if nat_eq(a283, a284) {
                            v75
                        } else {
                            v76
                        }
                    }
                    Err(v72) => {
                        Err::<(), Bytes>(v72)
                    }
                }
            };
            let v78: Result<(), Bytes> = Err::<(), Bytes>(f23(f13(Str::lit("the literal ")), f23(f48(Adt3::C21(v70, std::rc::Rc::new(v71)))?, f23(f13(Str::lit(" does not have type ")), f35(v2.clone())?))));
            let a285 = f15(v2);
            let a286 = 12u64;
            if nat_eq(a285, a286) {
                Ok(v77)
            } else {
                Ok(v78)
            }
        }
        Adt3::C22(_, _) => {
            Ok(Err::<(), Bytes>(f13(Str::lit("a closure has no literal form; build it with `closure`"))))
        }
        Adt3::C23 => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt3::C24(r303, r304) => {
            let v81 = (*r303).clone();
            let v82 = (*r304).clone();
            let v83: Adt0 = {
                let a287 = v3;
                let a288 = 2u64;
                if nat_eq(a287, a288) {
                    f16(v2.clone())
                } else {
                    v2.clone()
                }
            };
            let v84: Adt0 = {
                let a289 = v3;
                let a290 = 2u64;
                if nat_eq(a289, a290) {
                    f14(v2.clone())
                } else {
                    v2.clone()
                }
            };
            let v85: Result<(), Bytes> = f21(v0.clone(), v81, v83, 0u64)?;
            let v86: Result<(), Bytes> = f21(v0, v82, v84, v3)?;
            let a291 = v3;
            let a292 = 0u64;
            if nat_eq(a291, a292) {
                Ok(Ok::<(), Bytes>(()))
            } else {
                let a293 = v3;
                let a294 = 2u64;
                if if nat_eq(a293, a294) {
                    f7(v2)
                } else {
                    false
                } {
                    Ok(Ok::<(), Bytes>(()))
                } else {
                    let m295 = v85;
                    match m295 {
                        Ok(_) => {
                            Ok(v86)
                        }
                        Err(v88) => {
                            Ok(Err::<(), Bytes>(v88))
                        }
                    }
                }
            }
        }
    }
}

pub fn f22(v0: List<(u64, Adt0)>, v1: u64) -> Option<Adt0> {
    let m305 = v0;
    match m305.uncons() {
        None => {
            None::<Adt0>
        }
        Some((v2, v3)) => {
            let v4: Option<Adt0> = f22(v3, v1);
            let a307 = {
                let (h306, _) = v2;
                h306
            };
            let a308 = v1;
            if nat_eq(a307, a308) {
                Some({
                    let (_, h309) = v2;
                    h309
                })
            } else {
                v4
            }
        }
    }
}

pub fn f23(v0: Bytes, v1: Bytes) -> Bytes {
    let a310 = v0;
    let a311 = v1;
    append_bytes(a310, a311)
}

pub fn f24(v0: u64) -> R<Bytes> {
    let v1: Bytes = f13(Str::lit("0"));
    let v2: Bytes = f55({
        let a348 = f56(v0, 10000000000000000000u64);
        let a349 = {
            let a346 = f56(v0, 1000000000000000000u64);
            let a347 = {
                let a344 = f56(v0, 100000000000000000u64);
                let a345 = {
                    let a342 = f56(v0, 10000000000000000u64);
                    let a343 = {
                        let a340 = f56(v0, 1000000000000000u64);
                        let a341 = {
                            let a338 = f56(v0, 100000000000000u64);
                            let a339 = {
                                let a336 = f56(v0, 10000000000000u64);
                                let a337 = {
                                    let a334 = f56(v0, 1000000000000u64);
                                    let a335 = {
                                        let a332 = f56(v0, 100000000000u64);
                                        let a333 = {
                                            let a330 = f56(v0, 10000000000u64);
                                            let a331 = {
                                                let a328 = f56(v0, 1000000000u64);
                                                let a329 = {
                                                    let a326 = f56(v0, 100000000u64);
                                                    let a327 = {
                                                        let a324 = f56(v0, 10000000u64);
                                                        let a325 = {
                                                            let a322 = f56(v0, 1000000u64);
                                                            let a323 = {
                                                                let a320 = f56(v0, 100000u64);
                                                                let a321 = {
                                                                    let a318 = f56(v0, 10000u64);
                                                                    let a319 = {
                                                                        let a316 = f56(v0, 1000u64);
                                                                        let a317 = {
                                                                            let a314 = f56(v0, 100u64);
                                                                            let a315 = {
                                                                                let a312 = f56(v0, 10u64);
                                                                                let a313 = f56(v0, 1u64);
                                                                                append_bytes(a312, a313)
                                                                            };
                                                                            append_bytes(a314, a315)
                                                                        };
                                                                        append_bytes(a316, a317)
                                                                    };
                                                                    append_bytes(a318, a319)
                                                                };
                                                                append_bytes(a320, a321)
                                                            };
                                                            append_bytes(a322, a323)
                                                        };
                                                        append_bytes(a324, a325)
                                                    };
                                                    append_bytes(a326, a327)
                                                };
                                                append_bytes(a328, a329)
                                            };
                                            append_bytes(a330, a331)
                                        };
                                        append_bytes(a332, a333)
                                    };
                                    append_bytes(a334, a335)
                                };
                                append_bytes(a336, a337)
                            };
                            append_bytes(a338, a339)
                        };
                        append_bytes(a340, a341)
                    };
                    append_bytes(a342, a343)
                };
                append_bytes(a344, a345)
            };
            append_bytes(a346, a347)
        };
        append_bytes(a348, a349)
    })?;
    let a350 = v0;
    let a351 = 0u64;
    if nat_eq(a350, a351) {
        Ok(v1)
    } else {
        Ok(v2)
    }
}

pub fn f25(v0: Adt0, v1: Adt0, v2: Bytes) -> R<Result<(), Bytes>> {
    let v3: Result<(), Bytes> = Ok::<(), Bytes>(());
    let v4: Result<(), Bytes> = Err::<(), Bytes>(f23(v2, f23(f13(Str::lit(" has type ")), f23(f35(v0.clone())?, f23(f13(Str::lit(", expected ")), f35(v1.clone())?)))));
    if f57(v0, v1) {
        Ok(v3)
    } else {
        Ok(v4)
    }
}

pub fn f26(v0: Adt7, v1: Adt0) -> R<Result<List<Adt5>, Bytes>> {
    let m352 = v1.clone();
    match m352 {
        Adt0::C0 => {
            Ok(Ok::<List<Adt5>, Bytes>(List::cons(Adt5::C11, List::<Adt5>::nil())))
        }
        Adt0::C1 => {
            Ok(Ok::<List<Adt5>, Bytes>(List::cons(Adt5::C9, List::cons(Adt5::C10, List::<Adt5>::nil()))))
        }
        Adt0::C2 => {
            Ok(Ok::<List<Adt5>, Bytes>(List::cons(Adt5::C6, List::cons(Adt5::C7, List::<Adt5>::nil()))))
        }
        Adt0::C3 => {
            Ok(Err::<List<Adt5>, Bytes>(f23(f13(Str::lit("a value of type ")), f23(f35(v1)?, f13(Str::lit(" cannot be matched"))))))
        }
        Adt0::C4(_) => {
            Ok(Err::<List<Adt5>, Bytes>(f23(f13(Str::lit("a value of type ")), f23(f35(v1)?, f13(Str::lit(" cannot be matched"))))))
        }
        Adt0::C5 => {
            Ok(Err::<List<Adt5>, Bytes>(f23(f13(Str::lit("a value of type ")), f23(f35(v1)?, f13(Str::lit(" cannot be matched"))))))
        }
        Adt0::C6 => {
            Ok(Err::<List<Adt5>, Bytes>(f23(f13(Str::lit("a value of type ")), f23(f35(v1)?, f13(Str::lit(" cannot be matched"))))))
        }
        Adt0::C7 => {
            Ok(Ok::<List<Adt5>, Bytes>(List::cons(Adt5::C12, List::cons(Adt5::C13, List::cons(Adt5::C14, List::<Adt5>::nil())))))
        }
        Adt0::C8(_) => {
            Ok(Ok::<List<Adt5>, Bytes>(List::cons(Adt5::C0, List::cons(Adt5::C1, List::<Adt5>::nil()))))
        }
        Adt0::C9(_, _) => {
            Ok(Ok::<List<Adt5>, Bytes>(List::cons(Adt5::C2, List::cons(Adt5::C3, List::<Adt5>::nil()))))
        }
        Adt0::C10(_) => {
            Ok(Ok::<List<Adt5>, Bytes>(List::cons(Adt5::C4, List::cons(Adt5::C5, List::<Adt5>::nil()))))
        }
        Adt0::C11(_, _) => {
            Ok(Ok::<List<Adt5>, Bytes>(List::cons(Adt5::C8, List::<Adt5>::nil())))
        }
        Adt0::C12(v9) => {
            let m353 = f37(v0, v9);
            match m353 {
                None => {
                    Ok(Err::<List<Adt5>, Bytes>(f38(v9)?))
                }
                Some(v10) => {
                    Ok(Ok::<List<Adt5>, Bytes>(f58(v10, 0u64)?))
                }
            }
        }
        Adt0::C13(_, _) => {
            Ok(Err::<List<Adt5>, Bytes>(f23(f13(Str::lit("a value of type ")), f23(f35(v1)?, f13(Str::lit(" cannot be matched"))))))
        }
        Adt0::C14 => {
            Ok(Err::<List<Adt5>, Bytes>(f23(f13(Str::lit("a value of type ")), f23(f35(v1)?, f13(Str::lit(" cannot be matched"))))))
        }
        Adt0::C15(_, _) => {
            Ok(Err::<List<Adt5>, Bytes>(f23(f13(Str::lit("a value of type ")), f23(f35(v1)?, f13(Str::lit(" cannot be matched"))))))
        }
    }
}

pub fn f27(v0: List<Adt5>, v1: List<Adt5>) -> Option<Adt5> {
    let m354 = v0;
    match m354.uncons() {
        None => {
            None::<Adt5>
        }
        Some((v2, v3)) => {
            let v4: Option<Adt5> = f27(v3, v1.clone());
            let v5: Option<Adt5> = v4;
            let v6: Option<Adt5> = Some(v2.clone());
            if f43(v1, v2) {
                v5
            } else {
                v6
            }
        }
    }
}

pub fn f28(v0: Adt5) -> R<Bytes> {
    let m355 = v0;
    match m355 {
        Adt5::C0 => {
            Ok(f13(Str::lit("None")))
        }
        Adt5::C1 => {
            Ok(f13(Str::lit("Some")))
        }
        Adt5::C2 => {
            Ok(f13(Str::lit("Ok")))
        }
        Adt5::C3 => {
            Ok(f13(Str::lit("Error")))
        }
        Adt5::C4 => {
            Ok(f13(Str::lit("Nil")))
        }
        Adt5::C5 => {
            Ok(f13(Str::lit("Cons")))
        }
        Adt5::C6 => {
            Ok(f13(Str::lit("Zero")))
        }
        Adt5::C7 => {
            Ok(f13(Str::lit("Succ")))
        }
        Adt5::C8 => {
            Ok(f13(Str::lit("Pair")))
        }
        Adt5::C9 => {
            Ok(f13(Str::lit("True")))
        }
        Adt5::C10 => {
            Ok(f13(Str::lit("False")))
        }
        Adt5::C11 => {
            Ok(f13(Str::lit("Unit")))
        }
        Adt5::C12 => {
            Ok(f13(Str::lit("Lt")))
        }
        Adt5::C13 => {
            Ok(f13(Str::lit("Eq")))
        }
        Adt5::C14 => {
            Ok(f13(Str::lit("Gt")))
        }
        Adt5::C15(v1) => {
            Ok(f23(f13(Str::lit("Adt { constructor: ")), f23(f24(v1)?, f13(Str::lit(" }")))))
        }
    }
}

pub fn f29(v0: Adt2) -> R<u64> {
    let m356 = v0;
    match m356 {
        Adt2::C0(_, _) => {
            Ok(0u64)
        }
        Adt2::C1(_) => {
            Ok(0u64)
        }
        Adt2::C2(_, _, _, _) => {
            Ok(0u64)
        }
        Adt2::C3(_, _, _) => {
            Ok(0u64)
        }
        Adt2::C4(_, _, _) => {
            Ok(0u64)
        }
        Adt2::C5(_, _, _) => {
            Ok(0u64)
        }
        Adt2::C6(_, _) => {
            Ok(0u64)
        }
        Adt2::C7(_, _) => {
            Ok(0u64)
        }
        Adt2::C8(_, _) => {
            Ok(0u64)
        }
        Adt2::C9(_, _) => {
            Ok(0u64)
        }
        Adt2::C10(_) => {
            Ok(0u64)
        }
        Adt2::C11(_) => {
            Ok(0u64)
        }
        Adt2::C12(_, _) => {
            Ok(0u64)
        }
        Adt2::C13 => {
            Ok(0u64)
        }
        Adt2::C14(_, r359) => {
            let v33 = (*r359).clone();
            let v34: u64 = f29(v33)?;
            let a357 = 1u64;
            let a358 = v34;
            nat_add(a357, a358)
        }
        Adt2::C15(_, _, _) => {
            Ok(0u64)
        }
    }
}

pub fn f30(v0: Adt0) -> R<u64> {
    let m360 = v0;
    match m360 {
        Adt0::C0 => {
            Ok(0u64)
        }
        Adt0::C1 => {
            Ok(0u64)
        }
        Adt0::C2 => {
            Ok(0u64)
        }
        Adt0::C3 => {
            Ok(0u64)
        }
        Adt0::C4(_) => {
            Ok(0u64)
        }
        Adt0::C5 => {
            Ok(0u64)
        }
        Adt0::C6 => {
            Ok(0u64)
        }
        Adt0::C7 => {
            Ok(0u64)
        }
        Adt0::C8(_) => {
            Ok(0u64)
        }
        Adt0::C9(_, _) => {
            Ok(0u64)
        }
        Adt0::C10(_) => {
            Ok(0u64)
        }
        Adt0::C11(_, _) => {
            Ok(0u64)
        }
        Adt0::C12(_) => {
            Ok(0u64)
        }
        Adt0::C13(_, _) => {
            Ok(0u64)
        }
        Adt0::C14 => {
            Ok(0u64)
        }
        Adt0::C15(_, r363) => {
            let v12 = (*r363).clone();
            let v13: u64 = f30(v12)?;
            let a361 = 1u64;
            let a362 = v13;
            nat_add(a361, a362)
        }
    }
}

pub fn f31(v0: u64) -> R<Bytes> {
    Ok(f23(f13(Str::lit("function ")), f23(f24(v0)?, f13(Str::lit(" is not declared")))))
}

pub fn f32(v0: Adt7, v1: u64) -> Adt0 {
    let m365 = f39(match v0 {
        Adt7::C0(_, _, _, _, h364, _) => {
            h364
        }
    }, v1);
    match m365 {
        None => {
            Adt0::C0
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f33(v0: Adt0, v1: Adt0) -> Adt0 {
    Adt0::C13(std::rc::Rc::new(v0), std::rc::Rc::new(v1))
}

pub fn f34(v0: Adt6, v1: Adt0) -> R<Result<Adt0, Bytes>> {
    let v2: Adt0 = f16(v1.clone());
    let v3: Adt0 = if f7(v1.clone()) {
        Adt0::C0
    } else {
        v2
    };
    let m366 = v0.clone();
    match m366 {
        Adt6::C0 => {
            f59(v0, Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C14)))), v1, Adt0::C2)
        }
        Adt6::C1 => {
            f59(v0, Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C14)))), v1, Adt0::C2)
        }
        Adt6::C2 => {
            f59(v0, Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C14)))), v1, Adt0::C2)
        }
        Adt6::C3 => {
            f59(v0, Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C14)))))), v1, Adt0::C2)
        }
        Adt6::C4 => {
            f59(v0, Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C14)))))), v1, Adt0::C2)
        }
        Adt6::C5 => {
            f59(v0, Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C14)))), v1, Adt0::C1)
        }
        Adt6::C6 => {
            f59(v0, Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C14)))), v1, Adt0::C1)
        }
        Adt6::C7 => {
            f59(v0, Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C14)))), v1, Adt0::C1)
        }
        Adt6::C8 => {
            f59(v0, Adt0::C15(std::rc::Rc::new(Adt0::C3), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C3), std::rc::Rc::new(Adt0::C14)))), v1, Adt0::C3)
        }
        Adt6::C9 => {
            f59(v0, Adt0::C15(std::rc::Rc::new(Adt0::C3), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C3), std::rc::Rc::new(Adt0::C14)))), v1, Adt0::C3)
        }
        Adt6::C10 => {
            f59(v0, Adt0::C15(std::rc::Rc::new(Adt0::C3), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C3), std::rc::Rc::new(Adt0::C14)))), v1, Adt0::C3)
        }
        Adt6::C11 => {
            f59(v0, Adt0::C15(std::rc::Rc::new(Adt0::C3), std::rc::Rc::new(Adt0::C14)), v1, Adt0::C3)
        }
        Adt6::C12 => {
            f59(v0, Adt0::C15(std::rc::Rc::new(Adt0::C3), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C3), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C3), std::rc::Rc::new(Adt0::C14)))))), v1, Adt0::C3)
        }
        Adt6::C13 => {
            f59(v0, Adt0::C15(std::rc::Rc::new(Adt0::C3), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C3), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C3), std::rc::Rc::new(Adt0::C14)))))), v1, Adt0::C3)
        }
        Adt6::C14 => {
            let v4: Result<Adt0, Bytes> = f59(v0.clone(), Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C14)))), v1.clone(), Adt0::C8(std::rc::Rc::new(v3.clone())))?;
            let v5: Result<Adt0, Bytes> = f60(v0, v1)?;
            let a367 = f15(v3);
            let a368 = 4u64;
            if nat_eq(a367, a368) {
                Ok(v4)
            } else {
                Ok(v5)
            }
        }
        Adt6::C15 => {
            let v6: Result<Adt0, Bytes> = f59(v0.clone(), Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C14)))), v1.clone(), Adt0::C8(std::rc::Rc::new(v3.clone())))?;
            let v7: Result<Adt0, Bytes> = f60(v0, v1)?;
            let a369 = f15(v3);
            let a370 = 4u64;
            if nat_eq(a369, a370) {
                Ok(v6)
            } else {
                Ok(v7)
            }
        }
        Adt6::C16 => {
            let v8: Result<Adt0, Bytes> = f59(v0.clone(), Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C14)))), v1.clone(), Adt0::C8(std::rc::Rc::new(v3.clone())))?;
            let v9: Result<Adt0, Bytes> = f60(v0, v1)?;
            let a371 = f15(v3);
            let a372 = 4u64;
            if nat_eq(a371, a372) {
                Ok(v8)
            } else {
                Ok(v9)
            }
        }
        Adt6::C17 => {
            let v12: Result<Adt0, Bytes> = f59(v0.clone(), Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C14)), v1.clone(), Adt0::C8(std::rc::Rc::new(v3.clone())))?;
            let v13: Result<Adt0, Bytes> = f60(v0, v1)?;
            let a375 = f15(v3.clone());
            let a376 = 4u64;
            if if nat_eq(a375, a376) {
                f61(f52(v3))
            } else {
                false
            } {
                Ok(v12)
            } else {
                Ok(v13)
            }
        }
        Adt6::C18 => {
            let v10: Result<Adt0, Bytes> = f59(v0.clone(), Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C14)))), v1.clone(), Adt0::C8(std::rc::Rc::new(v3.clone())))?;
            let v11: Result<Adt0, Bytes> = f60(v0, v1)?;
            let a373 = f15(v3);
            let a374 = 4u64;
            if nat_eq(a373, a374) {
                Ok(v10)
            } else {
                Ok(v11)
            }
        }
        Adt6::C19 => {
            let v14: Result<Adt0, Bytes> = f59(v0.clone(), Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C14)))), v1.clone(), v3.clone())?;
            let v15: Result<Adt0, Bytes> = f60(v0, v1)?;
            let a377 = f15(v3);
            let a378 = 4u64;
            if nat_eq(a377, a378) {
                Ok(v14)
            } else {
                Ok(v15)
            }
        }
        Adt6::C20 => {
            let v16: Result<Adt0, Bytes> = f59(v0.clone(), Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C14)))), v1.clone(), v3.clone())?;
            let v17: Result<Adt0, Bytes> = f60(v0, v1)?;
            let a379 = f15(v3);
            let a380 = 4u64;
            if nat_eq(a379, a380) {
                Ok(v16)
            } else {
                Ok(v17)
            }
        }
        Adt6::C21 => {
            let v18: Result<Adt0, Bytes> = f59(v0.clone(), Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C14)))), v1.clone(), v3.clone())?;
            let v19: Result<Adt0, Bytes> = f60(v0, v1)?;
            let a381 = f15(v3);
            let a382 = 4u64;
            if nat_eq(a381, a382) {
                Ok(v18)
            } else {
                Ok(v19)
            }
        }
        Adt6::C22 => {
            let v20: Result<Adt0, Bytes> = f59(v0.clone(), Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C14)), v1.clone(), v3.clone())?;
            let v21: Result<Adt0, Bytes> = f60(v0, v1)?;
            let a383 = f15(v3);
            let a384 = 4u64;
            if nat_eq(a383, a384) {
                Ok(v20)
            } else {
                Ok(v21)
            }
        }
        Adt6::C23 => {
            let v22: Result<Adt0, Bytes> = f59(v0.clone(), Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C4(Adt1::C2)), std::rc::Rc::new(Adt0::C14)))), v1.clone(), Adt0::C8(std::rc::Rc::new(v3.clone())))?;
            let v23: Result<Adt0, Bytes> = f60(v0, v1)?;
            let a385 = f15(v3);
            let a386 = 4u64;
            if nat_eq(a385, a386) {
                Ok(v22)
            } else {
                Ok(v23)
            }
        }
        Adt6::C24 => {
            let v24: Result<Adt0, Bytes> = f59(v0.clone(), Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C4(Adt1::C2)), std::rc::Rc::new(Adt0::C14)))), v1.clone(), Adt0::C8(std::rc::Rc::new(v3.clone())))?;
            let v25: Result<Adt0, Bytes> = f60(v0, v1)?;
            let a387 = f15(v3);
            let a388 = 4u64;
            if nat_eq(a387, a388) {
                Ok(v24)
            } else {
                Ok(v25)
            }
        }
        Adt6::C25 => {
            let v26: Result<Adt0, Bytes> = f59(v0.clone(), Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C14)))), v1.clone(), Adt0::C1)?;
            let v27: Result<Adt0, Bytes> = f60(v0, v1)?;
            let a389 = f15(v3.clone());
            let a390 = 2u64;
            if if nat_eq(a389, a390) {
                true
            } else {
                let a391 = f15(v3.clone());
                let a392 = 1u64;
                if nat_eq(a391, a392) {
                    true
                } else {
                    let a393 = f15(v3.clone());
                    let a394 = 4u64;
                    if nat_eq(a393, a394) {
                        true
                    } else {
                        let a395 = f15(v3.clone());
                        let a396 = 5u64;
                        if nat_eq(a395, a396) {
                            true
                        } else {
                            let a397 = f15(v3.clone());
                            let a398 = 6u64;
                            if nat_eq(a397, a398) {
                                true
                            } else {
                                let a399 = f15(v3);
                                let a400 = 7u64;
                                nat_eq(a399, a400)
                            }
                        }
                    }
                }
            } {
                Ok(v26)
            } else {
                Ok(v27)
            }
        }
        Adt6::C26 => {
            f59(v0, Adt0::C15(std::rc::Rc::new(Adt0::C1), std::rc::Rc::new(Adt0::C14)), v1, Adt0::C1)
        }
        Adt6::C27 => {
            f59(v0, Adt0::C15(std::rc::Rc::new(Adt0::C1), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C1), std::rc::Rc::new(Adt0::C14)))), v1, Adt0::C1)
        }
        Adt6::C28 => {
            f59(v0, Adt0::C15(std::rc::Rc::new(Adt0::C1), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C1), std::rc::Rc::new(Adt0::C14)))), v1, Adt0::C1)
        }
        Adt6::C29 => {
            let v28: Result<Adt0, Bytes> = f59(v0.clone(), Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C14)))), v1.clone(), v3.clone())?;
            let v29: Result<Adt0, Bytes> = f60(v0, v1)?;
            let a401 = f15(v3.clone());
            let a402 = 10u64;
            if if nat_eq(a401, a402) {
                true
            } else {
                let a403 = f15(v3);
                let a404 = 6u64;
                nat_eq(a403, a404)
            } {
                Ok(v28)
            } else {
                Ok(v29)
            }
        }
        Adt6::C30 => {
            let v30: Result<Adt0, Bytes> = f59(v0.clone(), Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C14)), v1.clone(), Adt0::C2)?;
            let v31: Result<Adt0, Bytes> = f60(v0, v1)?;
            let a405 = f15(v3.clone());
            let a406 = 10u64;
            if if nat_eq(a405, a406) {
                true
            } else {
                let a407 = f15(v3.clone());
                let a408 = 6u64;
                if nat_eq(a407, a408) {
                    true
                } else {
                    let a409 = f15(v3);
                    let a410 = 5u64;
                    nat_eq(a409, a410)
                }
            } {
                Ok(v30)
            } else {
                Ok(v31)
            }
        }
        Adt6::C31 => {
            let v32: Result<Adt0, Bytes> = f59(v0.clone(), Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C14)))), v1.clone(), Adt0::C8(std::rc::Rc::new(f16(v3.clone()))))?;
            let v33: Result<Adt0, Bytes> = f59(v0, Adt0::C15(std::rc::Rc::new(Adt0::C6), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C14)))), v1, Adt0::C8(std::rc::Rc::new(Adt0::C4(Adt1::C0))))?;
            let a411 = f15(v3.clone());
            let a412 = 10u64;
            if nat_eq(a411, a412) {
                Ok(v32)
            } else {
                let a413 = f15(v3.clone());
                let a414 = 6u64;
                if nat_eq(a413, a414) {
                    Ok(v33)
                } else {
                    Ok(Err::<Adt0, Bytes>(f23(f13(Str::lit("primitive Index of ")), f35(v3)?)))
                }
            }
        }
        Adt6::C32 => {
            let v34: Result<Adt0, Bytes> = f59(v0.clone(), Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C14)))))), v1.clone(), Adt0::C8(std::rc::Rc::new(v3.clone())))?;
            let v35: Result<Adt0, Bytes> = f60(v0, v1)?;
            let a415 = f15(v3.clone());
            let a416 = 10u64;
            if if nat_eq(a415, a416) {
                true
            } else {
                let a417 = f15(v3);
                let a418 = 6u64;
                nat_eq(a417, a418)
            } {
                Ok(v34)
            } else {
                Ok(v35)
            }
        }
        Adt6::C33 => {
            f59(v0, Adt0::C15(std::rc::Rc::new(Adt0::C5), std::rc::Rc::new(Adt0::C14)), v1, Adt0::C6)
        }
        Adt6::C34 => {
            f59(v0, Adt0::C15(std::rc::Rc::new(Adt0::C6), std::rc::Rc::new(Adt0::C14)), v1, Adt0::C8(std::rc::Rc::new(Adt0::C5)))
        }
        Adt6::C35 => {
            f59(v0, Adt0::C15(std::rc::Rc::new(Adt0::C6), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C6), std::rc::Rc::new(Adt0::C14)))), v1, Adt0::C7)
        }
        Adt6::C36 => {
            f59(v0, Adt0::C15(std::rc::Rc::new(Adt0::C5), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C5), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C4(Adt1::C2)), std::rc::Rc::new(Adt0::C14)))))), v1, Adt0::C8(std::rc::Rc::new(Adt0::C10(std::rc::Rc::new(Adt0::C5)))))
        }
        Adt6::C37 => {
            f59(v0, Adt0::C15(std::rc::Rc::new(Adt0::C10(std::rc::Rc::new(Adt0::C5))), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C5), std::rc::Rc::new(Adt0::C14)))), v1, Adt0::C5)
        }
        Adt6::C38 => {
            let v36: Result<Adt0, Bytes> = f59(v0.clone(), Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C14)), v1.clone(), Adt0::C5)?;
            let v37: Result<Adt0, Bytes> = f60(v0, v1)?;
            let a419 = f15(v3.clone());
            let a420 = 3u64;
            if if nat_eq(a419, a420) {
                true
            } else {
                let a421 = f15(v3);
                let a422 = 4u64;
                nat_eq(a421, a422)
            } {
                Ok(v36)
            } else {
                Ok(v37)
            }
        }
        Adt6::C39 => {
            let v38: Result<Adt0, Bytes> = f59(v0.clone(), Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C14)))), v1.clone(), Adt0::C7)?;
            let v39: Result<Adt0, Bytes> = f60(v0, v1)?;
            if f62(v3) {
                Ok(v38)
            } else {
                Ok(v39)
            }
        }
        Adt6::C40(v40) => {
            let v41: Result<Adt0, Bytes> = f59(v0.clone(), Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C14)), v1.clone(), Adt0::C8(std::rc::Rc::new(Adt0::C4(v40))))?;
            let v42: Result<Adt0, Bytes> = f60(v0, v1)?;
            let a423 = f15(v3.clone());
            let a424 = 3u64;
            if if nat_eq(a423, a424) {
                true
            } else {
                let a425 = f15(v3);
                let a426 = 4u64;
                nat_eq(a425, a426)
            } {
                Ok(v41)
            } else {
                Ok(v42)
            }
        }
        Adt6::C41(v43) => {
            let v44: u64 = f15(v43.clone());
            let v45: Result<Adt0, Bytes> = f59(v0.clone(), Adt0::C15(std::rc::Rc::new(Adt0::C5), std::rc::Rc::new(Adt0::C14)), v1.clone(), Adt0::C8(std::rc::Rc::new(v43)))?;
            let v46: Result<Adt0, Bytes> = f60(v0, v1)?;
            let a427 = v44;
            let a428 = 3u64;
            if if nat_eq(a427, a428) {
                true
            } else {
                let a429 = v44;
                let a430 = 4u64;
                nat_eq(a429, a430)
            } {
                Ok(v45)
            } else {
                Ok(v46)
            }
        }
    }
}

pub fn f35(v0: Adt0) -> R<Bytes> {
    let m431 = v0;
    match m431 {
        Adt0::C0 => {
            Ok(f13(Str::lit("Unit")))
        }
        Adt0::C1 => {
            Ok(f13(Str::lit("Bool")))
        }
        Adt0::C2 => {
            Ok(f13(Str::lit("Nat")))
        }
        Adt0::C3 => {
            Ok(f13(Str::lit("Int")))
        }
        Adt0::C4(v1) => {
            Ok(f23(f13(Str::lit("Fixed { width: ")), f23(f63(v1), f13(Str::lit(" }")))))
        }
        Adt0::C5 => {
            Ok(f13(Str::lit("String")))
        }
        Adt0::C6 => {
            Ok(f13(Str::lit("Bytes")))
        }
        Adt0::C7 => {
            Ok(f13(Str::lit("Ordering")))
        }
        Adt0::C8(r432) => {
            let v2 = (*r432).clone();
            Ok(f23(f13(Str::lit("Option { value: ")), f23(f35(v2)?, f13(Str::lit(" }")))))
        }
        Adt0::C9(r433, r434) => {
            let v3 = (*r433).clone();
            let v4 = (*r434).clone();
            Ok(f23(f13(Str::lit("Result { ok: ")), f23(f35(v3)?, f23(f13(Str::lit(", error: ")), f23(f35(v4)?, f13(Str::lit(" }")))))))
        }
        Adt0::C10(r435) => {
            let v5 = (*r435).clone();
            Ok(f23(f13(Str::lit("List { element: ")), f23(f35(v5)?, f13(Str::lit(" }")))))
        }
        Adt0::C11(r436, r437) => {
            let v6 = (*r436).clone();
            let v7 = (*r437).clone();
            Ok(f23(f13(Str::lit("Pair { left: ")), f23(f35(v6)?, f23(f13(Str::lit(", right: ")), f23(f35(v7)?, f13(Str::lit(" }")))))))
        }
        Adt0::C12(v8) => {
            Ok(f23(f13(Str::lit("Adt { index: ")), f23(f24(v8)?, f13(Str::lit(" }")))))
        }
        Adt0::C13(r438, r439) => {
            let v9 = (*r438).clone();
            let v10 = (*r439).clone();
            Ok(f23(f13(Str::lit("Fn { parameters: [")), f23(f35(v9)?, f23(f13(Str::lit("], result: ")), f23(f35(v10)?, f13(Str::lit(" }")))))))
        }
        Adt0::C14 => {
            Ok(f13(Str::lit("")))
        }
        Adt0::C15(r440, r441) => {
            let v11 = (*r440).clone();
            let v12 = (*r441).clone();
            let v13: Bytes = f35(v11)?;
            let v14: Bytes = f35(v12.clone())?;
            if f7(v12) {
                Ok(v13)
            } else {
                Ok(f23(v13, f23(f13(Str::lit(", ")), v14)))
            }
        }
    }
}

pub fn f36(v0: Adt0) -> u64 {
    let m442 = v0;
    match m442 {
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

pub fn f37(v0: Adt7, v1: u64) -> Option<Adt0> {
    f39(match v0 {
        Adt7::C0(_, h443, _, _, _, _) => {
            h443
        }
    }, v1)
}

pub fn f38(v0: u64) -> R<Bytes> {
    Ok(f23(f13(Str::lit("ADT ")), f23(f24(v0)?, f13(Str::lit(" is not declared")))))
}

pub fn f39(p449: Adt0, p450: u64) -> Option<Adt0> {
    let m448 = (p449, p450);
    {
        let mut m448 = m448;
        loop {
            let step = {
                let (v0, v1) = m448;
                let m444 = v0;
                match m444 {
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
                    Adt0::C15(r446, r447) => {
                        let v12 = (*r446).clone();
                        let v13 = (*r447).clone();
                        let m445 = v1;
                        if m445 == 0 {
                            Err::<(Adt0, u64), Option<Adt0>>(Some(v12))
                        } else {
                            let v14 = m445 - 1;
                            Ok::<(Adt0, u64), Option<Adt0>>((v13, v14))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m448 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f40(v0: Adt0, v1: Adt0) -> Adt0 {
    Adt0::C15(std::rc::Rc::new(v0), std::rc::Rc::new(v1))
}

pub fn f41(v0: List<u64>) -> R<u64> {
    f64(Fn0::F65, 0u64, v0)
}

pub fn f42(v0: List<u64>) -> bool {
    let m451 = v0;
    match m451.uncons() {
        None => {
            false
        }
        Some((v1, v2)) => {
            let v3: bool = f42(v2.clone());
            let v4: bool = f66(v2, v1);
            if v4 {
                true
            } else {
                v3
            }
        }
    }
}

pub fn f43(v0: List<Adt5>, v1: Adt5) -> bool {
    let m452 = v0;
    match m452.uncons() {
        None => {
            false
        }
        Some((v2, v3)) => {
            let v4: bool = f67(v2, v1.clone());
            let v5: bool = f43(v3, v1);
            if v4 {
                true
            } else {
                v5
            }
        }
    }
}

pub fn f44(v0: Adt5, v1: List<Adt5>) -> List<Adt5> {
    List::cons(v0, v1)
}

pub fn f45(v0: Adt0) -> Option<Adt0> {
    let m453 = v0;
    match m453 {
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
        Adt0::C9(_, r454) => {
            let v4 = (*r454).clone();
            Some(v4)
        }
        Adt0::C10(_) => {
            None::<Adt0>
        }
        Adt0::C11(_, r455) => {
            let v7 = (*r455).clone();
            Some(v7)
        }
        Adt0::C12(_) => {
            None::<Adt0>
        }
        Adt0::C13(_, r456) => {
            let v10 = (*r456).clone();
            Some(v10)
        }
        Adt0::C14 => {
            None::<Adt0>
        }
        Adt0::C15(_, r457) => {
            let v12 = (*r457).clone();
            Some(v12)
        }
    }
}

pub fn f46(v0: Adt0) -> Option<Adt0> {
    let m458 = v0;
    match m458 {
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
        Adt0::C8(r459) => {
            let v2 = (*r459).clone();
            Some(v2)
        }
        Adt0::C9(r460, _) => {
            let v3 = (*r460).clone();
            Some(v3)
        }
        Adt0::C10(r461) => {
            let v5 = (*r461).clone();
            Some(v5)
        }
        Adt0::C11(r462, _) => {
            let v6 = (*r462).clone();
            Some(v6)
        }
        Adt0::C12(_) => {
            None::<Adt0>
        }
        Adt0::C13(r463, _) => {
            let v9 = (*r463).clone();
            Some(v9)
        }
        Adt0::C14 => {
            None::<Adt0>
        }
        Adt0::C15(r464, _) => {
            let v11 = (*r464).clone();
            Some(v11)
        }
    }
}

pub fn f47(v0: Adt7, v1: u64, v2: u64) -> R<Result<Adt0, Bytes>> {
    let m465 = f37(v0, v1);
    match m465 {
        None => {
            Ok(Err::<Adt0, Bytes>(f38(v1)?))
        }
        Some(v3) => {
            let m466 = f39(v3, v2);
            match m466 {
                None => {
                    Ok(Err::<Adt0, Bytes>(f23(f13(Str::lit("ADT ")), f23(f24(v1)?, f23(f13(Str::lit(" has no constructor ")), f24(v2)?)))))
                }
                Some(v4) => {
                    Ok(Ok::<Adt0, Bytes>(v4))
                }
            }
        }
    }
}

pub fn f48(v0: Adt3) -> R<Bytes> {
    let m467 = v0;
    match m467 {
        Adt3::C0 => {
            Ok(f13(Str::lit("Unit")))
        }
        Adt3::C1(v1) => {
            Ok(f23(f13(Str::lit("Bool { value: ")), f23(f68(v1), f13(Str::lit(" }")))))
        }
        Adt3::C2(v2) => {
            Ok(f23(f13(Str::lit("Nat { value: ")), f23(f69(v2)?, f13(Str::lit(" }")))))
        }
        Adt3::C3(v3) => {
            Ok(f23(f13(Str::lit("Int { value: ")), f23(f69(v3)?, f13(Str::lit(" }")))))
        }
        Adt3::C4(v4) => {
            Ok(f23(f13(Str::lit("U8 { value: ")), f23(f69(v4)?, f13(Str::lit(" }")))))
        }
        Adt3::C5(v5) => {
            Ok(f23(f13(Str::lit("U16 { value: ")), f23(f69(v5)?, f13(Str::lit(" }")))))
        }
        Adt3::C6(v6) => {
            Ok(f23(f13(Str::lit("U32 { value: ")), f23(f69(v6)?, f13(Str::lit(" }")))))
        }
        Adt3::C7(v7) => {
            Ok(f23(f13(Str::lit("U64 { value: ")), f23(f69(v7)?, f13(Str::lit(" }")))))
        }
        Adt3::C8(v8) => {
            Ok(f23(f13(Str::lit("I8 { value: ")), f23(f69(v8)?, f13(Str::lit(" }")))))
        }
        Adt3::C9(v9) => {
            Ok(f23(f13(Str::lit("I16 { value: ")), f23(f69(v9)?, f13(Str::lit(" }")))))
        }
        Adt3::C10(v10) => {
            Ok(f23(f13(Str::lit("I32 { value: ")), f23(f69(v10)?, f13(Str::lit(" }")))))
        }
        Adt3::C11(v11) => {
            Ok(f23(f13(Str::lit("I64 { value: ")), f23(f69(v11)?, f13(Str::lit(" }")))))
        }
        Adt3::C12(v12) => {
            Ok(f23(f13(Str::lit("String { value: ")), f23(f69(v12)?, f13(Str::lit(" }")))))
        }
        Adt3::C13(v13) => {
            Ok(f23(f13(Str::lit("Bytes { hex: ")), f23(f69(v13)?, f13(Str::lit(" }")))))
        }
        Adt3::C14(v14) => {
            Ok(f23(f13(Str::lit("Ordering { value: ")), f23(f70(v14), f13(Str::lit(" }")))))
        }
        Adt3::C15 => {
            Ok(f13(Str::lit("None")))
        }
        Adt3::C16(r468) => {
            let v15 = (*r468).clone();
            Ok(f23(f13(Str::lit("Some { value: ")), f23(f48(v15)?, f13(Str::lit(" }")))))
        }
        Adt3::C17(r469) => {
            let v16 = (*r469).clone();
            Ok(f23(f13(Str::lit("Ok { value: ")), f23(f48(v16)?, f13(Str::lit(" }")))))
        }
        Adt3::C18(r470) => {
            let v17 = (*r470).clone();
            Ok(f23(f13(Str::lit("Error { value: ")), f23(f48(v17)?, f13(Str::lit(" }")))))
        }
        Adt3::C19(r471) => {
            let v18 = (*r471).clone();
            Ok(f23(f13(Str::lit("List { items: [")), f23(f48(v18)?, f13(Str::lit("] }")))))
        }
        Adt3::C20(r472, r473) => {
            let v19 = (*r472).clone();
            let v20 = (*r473).clone();
            Ok(f23(f13(Str::lit("Pair { left: ")), f23(f48(v19)?, f23(f13(Str::lit(", right: ")), f23(f48(v20)?, f13(Str::lit(" }")))))))
        }
        Adt3::C21(v21, r474) => {
            let v22 = (*r474).clone();
            Ok(f23(f13(Str::lit("Adt { constructor: ")), f23(f24(v21)?, f23(f13(Str::lit(", fields: [")), f23(f48(v22)?, f13(Str::lit("] }")))))))
        }
        Adt3::C22(v23, r475) => {
            let v24 = (*r475).clone();
            Ok(f23(f13(Str::lit("Closure { function: ")), f23(f24(v23)?, f23(f13(Str::lit(", captures: [")), f23(f48(v24)?, f13(Str::lit("] }")))))))
        }
        Adt3::C23 => {
            Ok(f13(Str::lit("")))
        }
        Adt3::C24(r476, r477) => {
            let v25 = (*r476).clone();
            let v26 = (*r477).clone();
            let v27: Bytes = f48(v25)?;
            let v28: Bytes = f48(v26.clone())?;
            if f71(v26) {
                Ok(v27)
            } else {
                Ok(f23(v27, f23(f13(Str::lit(", ")), v28)))
            }
        }
    }
}

pub fn f49(v0: Bytes) -> R<Result<(), Bytes>> {
    let m478 = f72(v0.clone())?;
    match m478 {
        Ok(v1) => {
            let v2: bool = f73(match v1.clone() {
                Adt9::C0(_, h479) => {
                    h479
                }
            }, f13(Str::lit("18446744073709551615")));
            if match v1 {
                Adt9::C0(h480, _) => {
                    h480
                }
            } {
                Ok(Err::<(), Bytes>(f23(f13(Str::lit("nat literal ")), f23(v0, f13(Str::lit(" is outside the 64-bit realization"))))))
            } else {
                if v2 {
                    Ok(Ok::<(), Bytes>(()))
                } else {
                    Ok(Err::<(), Bytes>(f23(f13(Str::lit("nat literal ")), f23(v0, f13(Str::lit(" is outside the 64-bit realization"))))))
                }
            }
        }
        Err(v3) => {
            Ok(Err::<(), Bytes>(v3))
        }
    }
}

pub fn f50(v0: Bytes, v1: Adt1) -> R<Result<(), Bytes>> {
    let m481 = f72(v0.clone())?;
    match m481 {
        Ok(v2) => {
            let v3: Adt9 = f74(v1.clone());
            let v4: Adt9 = f75(v1.clone());
            let v5: bool = f76(v3, v2.clone());
            let v6: bool = f76(v2, v4);
            if v5 {
                if v6 {
                    Ok(Ok::<(), Bytes>(()))
                } else {
                    Ok(Err::<(), Bytes>(f23(v0, f23(f13(Str::lit(" is outside ")), f77(v1)))))
                }
            } else {
                Ok(Err::<(), Bytes>(f23(v0, f23(f13(Str::lit(" is outside ")), f77(v1)))))
            }
        }
        Err(v7) => {
            Ok(Err::<(), Bytes>(v7))
        }
    }
}

pub fn f51(v0: Adt1) -> u64 {
    let m482 = v0;
    match m482 {
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

pub fn f52(v0: Adt0) -> Adt1 {
    let m483 = v0;
    match m483 {
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

pub fn f53(v0: Bytes) -> R<Result<(), Bytes>> {
    let a488 = {
        let a485 = {
            let a484 = v0.clone();
            length_bytes(a484)
        };
        let a486 = 2u64;
        let a487 = 0u64;
        nat_rem(a485, a486, a487)
    };
    let a489 = 0u64;
    if nat_eq(a488, a489) {
        if f78(v0.clone())? {
            Ok(Ok::<(), Bytes>(()))
        } else {
            Ok(Err::<(), Bytes>(f23(f13(Str::lit("byte literal `")), f23(v0, f13(Str::lit("` is not even-length lowercase hexadecimal"))))))
        }
    } else {
        Ok(Err::<(), Bytes>(f23(f13(Str::lit("byte literal `")), f23(v0, f13(Str::lit("` is not even-length lowercase hexadecimal"))))))
    }
}

pub fn f54(v0: Adt3) -> R<u64> {
    let m490 = v0;
    match m490 {
        Adt3::C0 => {
            Ok(0u64)
        }
        Adt3::C1(_) => {
            Ok(0u64)
        }
        Adt3::C2(_) => {
            Ok(0u64)
        }
        Adt3::C3(_) => {
            Ok(0u64)
        }
        Adt3::C4(_) => {
            Ok(0u64)
        }
        Adt3::C5(_) => {
            Ok(0u64)
        }
        Adt3::C6(_) => {
            Ok(0u64)
        }
        Adt3::C7(_) => {
            Ok(0u64)
        }
        Adt3::C8(_) => {
            Ok(0u64)
        }
        Adt3::C9(_) => {
            Ok(0u64)
        }
        Adt3::C10(_) => {
            Ok(0u64)
        }
        Adt3::C11(_) => {
            Ok(0u64)
        }
        Adt3::C12(_) => {
            Ok(0u64)
        }
        Adt3::C13(_) => {
            Ok(0u64)
        }
        Adt3::C14(_) => {
            Ok(0u64)
        }
        Adt3::C15 => {
            Ok(0u64)
        }
        Adt3::C16(_) => {
            Ok(0u64)
        }
        Adt3::C17(_) => {
            Ok(0u64)
        }
        Adt3::C18(_) => {
            Ok(0u64)
        }
        Adt3::C19(_) => {
            Ok(0u64)
        }
        Adt3::C20(_, _) => {
            Ok(0u64)
        }
        Adt3::C21(_, _) => {
            Ok(0u64)
        }
        Adt3::C22(_, _) => {
            Ok(0u64)
        }
        Adt3::C23 => {
            Ok(0u64)
        }
        Adt3::C24(_, r493) => {
            let v26 = (*r493).clone();
            let v27: u64 = f54(v26)?;
            let a491 = 1u64;
            let a492 = v27;
            nat_add(a491, a492)
        }
    }
}

pub fn f55(v0: Bytes) -> R<Bytes> {
    let v1: u64 = {
        let (h494, _) = f79(Fn1::F80(v0.clone()), {
            let a495 = v0.clone();
            length_bytes(a495)
        }, 0u64)?;
        h494
    };
    Ok(f81(v0.clone(), v1, {
        let a497 = {
            let a496 = v0;
            length_bytes(a496)
        };
        let a498 = v1;
        nat_sub(a497, a498)
    }))
}

pub fn f56(v0: u64, v1: u64) -> Bytes {
    f81(f13(Str::lit("0123456789")), {
        let a502 = {
            let a499 = v0;
            let a500 = v1;
            let a501 = 0u64;
            nat_quot(a499, a500, a501)
        };
        let a503 = 10u64;
        let a504 = 0u64;
        nat_rem(a502, a503, a504)
    }, 1u64)
}

pub fn f57(v0: Adt0, v1: Adt0) -> bool {
    let m505 = v0;
    match m505 {
        Adt0::C0 => {
            let a506 = f15(v1);
            let a507 = 0u64;
            nat_eq(a506, a507)
        }
        Adt0::C1 => {
            let a508 = f15(v1);
            let a509 = 1u64;
            nat_eq(a508, a509)
        }
        Adt0::C2 => {
            let a510 = f15(v1);
            let a511 = 2u64;
            nat_eq(a510, a511)
        }
        Adt0::C3 => {
            let a512 = f15(v1);
            let a513 = 3u64;
            nat_eq(a512, a513)
        }
        Adt0::C4(v2) => {
            let a522 = f15(v1.clone());
            let a523 = 4u64;
            if nat_eq(a522, a523) {
                let a524 = f51(v2);
                let a525 = f51(f52(v1));
                nat_eq(a524, a525)
            } else {
                false
            }
        }
        Adt0::C5 => {
            let a514 = f15(v1);
            let a515 = 5u64;
            nat_eq(a514, a515)
        }
        Adt0::C6 => {
            let a516 = f15(v1);
            let a517 = 6u64;
            nat_eq(a516, a517)
        }
        Adt0::C7 => {
            let a518 = f15(v1);
            let a519 = 7u64;
            nat_eq(a518, a519)
        }
        Adt0::C8(r542) => {
            let v3 = (*r542).clone();
            let v4: bool = f57(v3, f16(v1.clone()));
            let a526 = f15(v1);
            let a527 = 8u64;
            if nat_eq(a526, a527) {
                v4
            } else {
                false
            }
        }
        Adt0::C9(r543, r544) => {
            let v5 = (*r543).clone();
            let v6 = (*r544).clone();
            let v7: bool = f57(v5, f16(v1.clone()));
            let v8: bool = f57(v6, f14(v1.clone()));
            let a528 = f15(v1);
            let a529 = 9u64;
            if nat_eq(a528, a529) {
                if v7 {
                    v8
                } else {
                    false
                }
            } else {
                false
            }
        }
        Adt0::C10(r545) => {
            let v9 = (*r545).clone();
            let v10: bool = f57(v9, f16(v1.clone()));
            let a530 = f15(v1);
            let a531 = 10u64;
            if nat_eq(a530, a531) {
                v10
            } else {
                false
            }
        }
        Adt0::C11(r546, r547) => {
            let v11 = (*r546).clone();
            let v12 = (*r547).clone();
            let v13: bool = f57(v11, f16(v1.clone()));
            let v14: bool = f57(v12, f14(v1.clone()));
            let a532 = f15(v1);
            let a533 = 11u64;
            if nat_eq(a532, a533) {
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
            let a534 = f15(v1.clone());
            let a535 = 12u64;
            if nat_eq(a534, a535) {
                let a536 = v15;
                let a537 = f36(v1);
                nat_eq(a536, a537)
            } else {
                false
            }
        }
        Adt0::C13(r548, r549) => {
            let v16 = (*r548).clone();
            let v17 = (*r549).clone();
            let v18: bool = f57(v16, f16(v1.clone()));
            let v19: bool = f57(v17, f14(v1.clone()));
            let a538 = f15(v1);
            let a539 = 13u64;
            if nat_eq(a538, a539) {
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
            let a520 = f15(v1);
            let a521 = 14u64;
            nat_eq(a520, a521)
        }
        Adt0::C15(r550, r551) => {
            let v20 = (*r550).clone();
            let v21 = (*r551).clone();
            let v22: bool = f57(v20, f16(v1.clone()));
            let v23: bool = f57(v21, f14(v1.clone()));
            let a540 = f15(v1);
            let a541 = 15u64;
            if nat_eq(a540, a541) {
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

pub fn f58(v0: Adt0, v1: u64) -> R<List<Adt5>> {
    let m552 = v0;
    match m552 {
        Adt0::C0 => {
            Ok(List::<Adt5>::nil())
        }
        Adt0::C1 => {
            Ok(List::<Adt5>::nil())
        }
        Adt0::C2 => {
            Ok(List::<Adt5>::nil())
        }
        Adt0::C3 => {
            Ok(List::<Adt5>::nil())
        }
        Adt0::C4(_) => {
            Ok(List::<Adt5>::nil())
        }
        Adt0::C5 => {
            Ok(List::<Adt5>::nil())
        }
        Adt0::C6 => {
            Ok(List::<Adt5>::nil())
        }
        Adt0::C7 => {
            Ok(List::<Adt5>::nil())
        }
        Adt0::C8(_) => {
            Ok(List::<Adt5>::nil())
        }
        Adt0::C9(_, _) => {
            Ok(List::<Adt5>::nil())
        }
        Adt0::C10(_) => {
            Ok(List::<Adt5>::nil())
        }
        Adt0::C11(_, _) => {
            Ok(List::<Adt5>::nil())
        }
        Adt0::C12(_) => {
            Ok(List::<Adt5>::nil())
        }
        Adt0::C13(_, _) => {
            Ok(List::<Adt5>::nil())
        }
        Adt0::C14 => {
            Ok(List::<Adt5>::nil())
        }
        Adt0::C15(_, r555) => {
            let v13 = (*r555).clone();
            Ok(f44(Adt5::C15(v1), f58(v13, {
                let a553 = v1;
                let a554 = 1u64;
                nat_add(a553, a554)?
            })?))
        }
    }
}

pub fn f59(v0: Adt6, v1: Adt0, v2: Adt0, v3: Adt0) -> R<Result<Adt0, Bytes>> {
    let v4: Result<Adt0, Bytes> = Ok::<Adt0, Bytes>(v3);
    let v5: Result<Adt0, Bytes> = Err::<Adt0, Bytes>(f23(f13(Str::lit("primitive ")), f23(f82(v0)?, f23(f13(Str::lit(" takes ")), f23(f83(v1.clone())?, f23(f13(Str::lit(", received ")), f83(v2.clone())?))))));
    if f57(v2, v1) {
        Ok(v4)
    } else {
        Ok(v5)
    }
}

pub fn f60(v0: Adt6, v1: Adt0) -> R<Result<Adt0, Bytes>> {
    Ok(Err::<Adt0, Bytes>(f23(f13(Str::lit("primitive ")), f23(f82(v0)?, f23(f13(Str::lit(" does not apply to ")), f83(v1)?)))))
}

pub fn f61(v0: Adt1) -> bool {
    let m556 = v0;
    match m556 {
        Adt1::C0 => {
            false
        }
        Adt1::C1 => {
            false
        }
        Adt1::C2 => {
            false
        }
        Adt1::C3 => {
            false
        }
        Adt1::C4 => {
            true
        }
        Adt1::C5 => {
            true
        }
        Adt1::C6 => {
            true
        }
        Adt1::C7 => {
            true
        }
    }
}

pub fn f62(v0: Adt0) -> bool {
    let m557 = v0;
    match m557 {
        Adt0::C0 => {
            false
        }
        Adt0::C1 => {
            true
        }
        Adt0::C2 => {
            true
        }
        Adt0::C3 => {
            true
        }
        Adt0::C4(_) => {
            true
        }
        Adt0::C5 => {
            true
        }
        Adt0::C6 => {
            false
        }
        Adt0::C7 => {
            false
        }
        Adt0::C8(_) => {
            false
        }
        Adt0::C9(_, _) => {
            false
        }
        Adt0::C10(_) => {
            false
        }
        Adt0::C11(r558, r559) => {
            let v2 = (*r558).clone();
            let v3 = (*r559).clone();
            let v4: bool = f62(v2);
            let v5: bool = f62(v3);
            if v4 {
                v5
            } else {
                false
            }
        }
        Adt0::C12(_) => {
            false
        }
        Adt0::C13(_, _) => {
            false
        }
        Adt0::C14 => {
            false
        }
        Adt0::C15(_, _) => {
            false
        }
    }
}

pub fn f63(v0: Adt1) -> Bytes {
    let m560 = v0;
    match m560 {
        Adt1::C0 => {
            f13(Str::lit("U8"))
        }
        Adt1::C1 => {
            f13(Str::lit("U16"))
        }
        Adt1::C2 => {
            f13(Str::lit("U32"))
        }
        Adt1::C3 => {
            f13(Str::lit("U64"))
        }
        Adt1::C4 => {
            f13(Str::lit("I8"))
        }
        Adt1::C5 => {
            f13(Str::lit("I16"))
        }
        Adt1::C6 => {
            f13(Str::lit("I32"))
        }
        Adt1::C7 => {
            f13(Str::lit("I64"))
        }
    }
}

pub fn f64(v0: Fn0, p564: u64, p565: List<u64>) -> R<u64> {
    let m563 = (p564, p565);
    {
        let mut m563 = m563;
        loop {
            let step = {
                let (v1, v2) = m563;
                let m561 = v2;
                match m561.uncons() {
                    None => {
                        Err::<(u64, List<u64>), R<u64>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(u64, List<u64>), R<u64>>(({
                            let c562: Fn0 = v0.clone();
                            c562.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m563 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f65(v0: u64, _: u64) -> R<u64> {
    let a566 = v0;
    let a567 = 1u64;
    nat_add(a566, a567)
}

pub fn f66(v0: List<u64>, v1: u64) -> bool {
    let m568 = v0;
    match m568.uncons() {
        None => {
            false
        }
        Some((v2, v3)) => {
            let v4: bool = f66(v3, v1);
            let a569 = v2;
            let a570 = v1;
            if nat_eq(a569, a570) {
                true
            } else {
                v4
            }
        }
    }
}

pub fn f67(v0: Adt5, v1: Adt5) -> bool {
    let a571 = f84(v0.clone());
    let a572 = f84(v1.clone());
    if nat_eq(a571, a572) {
        let a573 = f85(v0);
        let a574 = f85(v1);
        nat_eq(a573, a574)
    } else {
        false
    }
}

pub fn f68(v0: bool) -> Bytes {
    if v0 {
        f13(Str::lit("true"))
    } else {
        f13(Str::lit("false"))
    }
}

pub fn f69(v0: Bytes) -> R<Bytes> {
    Ok(f23(f13(Str::lit("\"")), f23({
        let (_, h575) = {
            let (h576, _) = f86(Fn2::F87(v0.clone()), {
                let a577 = v0;
                length_bytes(a577)
            }, (0u64, Bytes::lit(&[])))?;
            h576
        };
        h575
    }, f13(Str::lit("\"")))))
}

pub fn f70(v0: Adt4) -> Bytes {
    let m578 = v0;
    match m578 {
        Adt4::C0 => {
            f13(Str::lit("Lt"))
        }
        Adt4::C1 => {
            f13(Str::lit("Eq"))
        }
        Adt4::C2 => {
            f13(Str::lit("Gt"))
        }
    }
}

pub fn f71(v0: Adt3) -> bool {
    let m579 = v0;
    match m579 {
        Adt3::C0 => {
            false
        }
        Adt3::C1(_) => {
            false
        }
        Adt3::C2(_) => {
            false
        }
        Adt3::C3(_) => {
            false
        }
        Adt3::C4(_) => {
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
        Adt3::C8(_) => {
            false
        }
        Adt3::C9(_) => {
            false
        }
        Adt3::C10(_) => {
            false
        }
        Adt3::C11(_) => {
            false
        }
        Adt3::C12(_) => {
            false
        }
        Adt3::C13(_) => {
            false
        }
        Adt3::C14(_) => {
            false
        }
        Adt3::C15 => {
            false
        }
        Adt3::C16(_) => {
            false
        }
        Adt3::C17(_) => {
            false
        }
        Adt3::C18(_) => {
            false
        }
        Adt3::C19(_) => {
            false
        }
        Adt3::C20(_, _) => {
            false
        }
        Adt3::C21(_, _) => {
            false
        }
        Adt3::C22(_, _) => {
            false
        }
        Adt3::C23 => {
            true
        }
        Adt3::C24(_, _) => {
            false
        }
    }
}

pub fn f72(v0: Bytes) -> R<Result<Adt9, Bytes>> {
    let v1: bool = f88(v0.clone(), 0u64, 45u8);
    let v3: Bytes = {
        let v2: Bytes = f81(v0.clone(), 1u64, {
            let a581 = {
                let a580 = v0.clone();
                length_bytes(a580)
            };
            let a582 = 1u64;
            nat_sub(a581, a582)
        });
        if v1 {
            v2
        } else {
            v0.clone()
        }
    };
    let v4: bool = f89(v3.clone(), v0.clone())?;
    let v5: bool = f73(v3.clone(), f13(Str::lit("170141183460469231731687303715884105727")));
    let v6: bool = f73(v3.clone(), f13(Str::lit("170141183460469231731687303715884105728")));
    if v4 {
        if if v1 {
            v6
        } else {
            v5
        } {
            Ok(Ok::<Adt9, Bytes>(Adt9::C0(v1, v3)))
        } else {
            Ok(Err::<Adt9, Bytes>(f23(f90(v0), f13(Str::lit(" is outside every realized integer")))))
        }
    } else {
        Ok(Err::<Adt9, Bytes>(f23(f90(v0), f13(Str::lit(" is not a canonical decimal")))))
    }
}

pub fn f73(v0: Bytes, v1: Bytes) -> bool {
    let v2: bool = {
        let a585 = {
            let a583 = v0.clone();
            length_bytes(a583)
        };
        let a586 = {
            let a584 = v1.clone();
            length_bytes(a584)
        };
        nat_lt(a585, a586)
    };
    let v3: bool = {
        let a589 = {
            let a587 = v0.clone();
            length_bytes(a587)
        };
        let a590 = {
            let a588 = v1.clone();
            length_bytes(a588)
        };
        nat_eq(a589, a590)
    };
    let v4: bool = f91(v1, v0);
    if v2 {
        true
    } else {
        if v3 {
            let a591 = v4;
            bool_not(a591)
        } else {
            false
        }
    }
}

pub fn f74(v0: Adt1) -> Adt9 {
    let m592 = v0;
    match m592 {
        Adt1::C0 => {
            f92(false, Str::lit("0"))
        }
        Adt1::C1 => {
            f92(false, Str::lit("0"))
        }
        Adt1::C2 => {
            f92(false, Str::lit("0"))
        }
        Adt1::C3 => {
            f92(false, Str::lit("0"))
        }
        Adt1::C4 => {
            f92(true, Str::lit("128"))
        }
        Adt1::C5 => {
            f92(true, Str::lit("32768"))
        }
        Adt1::C6 => {
            f92(true, Str::lit("2147483648"))
        }
        Adt1::C7 => {
            f92(true, Str::lit("9223372036854775808"))
        }
    }
}

pub fn f75(v0: Adt1) -> Adt9 {
    let m593 = v0;
    match m593 {
        Adt1::C0 => {
            f92(false, Str::lit("255"))
        }
        Adt1::C1 => {
            f92(false, Str::lit("65535"))
        }
        Adt1::C2 => {
            f92(false, Str::lit("4294967295"))
        }
        Adt1::C3 => {
            f92(false, Str::lit("18446744073709551615"))
        }
        Adt1::C4 => {
            f92(false, Str::lit("127"))
        }
        Adt1::C5 => {
            f92(false, Str::lit("32767"))
        }
        Adt1::C6 => {
            f92(false, Str::lit("2147483647"))
        }
        Adt1::C7 => {
            f92(false, Str::lit("9223372036854775807"))
        }
    }
}

pub fn f76(v0: Adt9, v1: Adt9) -> bool {
    let v2: bool = f73(match v1.clone() {
        Adt9::C0(_, h594) => {
            h594
        }
    }, match v0.clone() {
        Adt9::C0(_, h595) => {
            h595
        }
    });
    let v3: bool = f73(match v0.clone() {
        Adt9::C0(_, h596) => {
            h596
        }
    }, match v1.clone() {
        Adt9::C0(_, h597) => {
            h597
        }
    });
    if match v0 {
        Adt9::C0(h598, _) => {
            h598
        }
    } {
        if match v1 {
            Adt9::C0(h599, _) => {
                h599
            }
        } {
            v2
        } else {
            true
        }
    } else {
        if match v1 {
            Adt9::C0(h600, _) => {
                h600
            }
        } {
            false
        } else {
            v3
        }
    }
}

pub fn f77(v0: Adt1) -> Bytes {
    let m601 = v0;
    match m601 {
        Adt1::C0 => {
            f13(Str::lit("u8"))
        }
        Adt1::C1 => {
            f13(Str::lit("u16"))
        }
        Adt1::C2 => {
            f13(Str::lit("u32"))
        }
        Adt1::C3 => {
            f13(Str::lit("u64"))
        }
        Adt1::C4 => {
            f13(Str::lit("i8"))
        }
        Adt1::C5 => {
            f13(Str::lit("i16"))
        }
        Adt1::C6 => {
            f13(Str::lit("i32"))
        }
        Adt1::C7 => {
            f13(Str::lit("i64"))
        }
    }
}

pub fn f78(v0: Bytes) -> R<bool> {
    let a605 = {
        let (h602, _) = f79(Fn1::F93(v0.clone()), {
            let a603 = v0.clone();
            length_bytes(a603)
        }, 0u64)?;
        h602
    };
    let a606 = {
        let a604 = v0;
        length_bytes(a604)
    };
    Ok(nat_eq(a605, a606))
}

pub fn f79(v0: Fn1, p611: u64, p612: u64) -> R<(u64, bool)> {
    let m610 = (p611, p612);
    {
        let mut m610 = m610;
        loop {
            let step = {
                let (v1, v2) = m610;
                let m607 = v1;
                if m607 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m607 - 1;
                    let m609 = {
                        let c608: Fn1 = v0.clone();
                        c608.apply(v2)?
                    };
                    match m609 {
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
                Ok(next) => { m610 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f80(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f94(v0, v1)
}

pub fn f81(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m616 = {
        let a613 = v0;
        let a614 = v1;
        let a615 = v2;
        slice_bytes(a613, a614, a615)
    };
    match m616 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f82(v0: Adt6) -> R<Bytes> {
    let m617 = v0;
    match m617 {
        Adt6::C0 => {
            Ok(f13(Str::lit("NatAdd")))
        }
        Adt6::C1 => {
            Ok(f13(Str::lit("NatSub")))
        }
        Adt6::C2 => {
            Ok(f13(Str::lit("NatMul")))
        }
        Adt6::C3 => {
            Ok(f13(Str::lit("NatQuot")))
        }
        Adt6::C4 => {
            Ok(f13(Str::lit("NatRem")))
        }
        Adt6::C5 => {
            Ok(f13(Str::lit("NatEq")))
        }
        Adt6::C6 => {
            Ok(f13(Str::lit("NatLe")))
        }
        Adt6::C7 => {
            Ok(f13(Str::lit("NatLt")))
        }
        Adt6::C8 => {
            Ok(f13(Str::lit("IntAdd")))
        }
        Adt6::C9 => {
            Ok(f13(Str::lit("IntSub")))
        }
        Adt6::C10 => {
            Ok(f13(Str::lit("IntMul")))
        }
        Adt6::C11 => {
            Ok(f13(Str::lit("IntNeg")))
        }
        Adt6::C12 => {
            Ok(f13(Str::lit("IntQuot")))
        }
        Adt6::C13 => {
            Ok(f13(Str::lit("IntRem")))
        }
        Adt6::C14 => {
            Ok(f13(Str::lit("CheckedAdd")))
        }
        Adt6::C15 => {
            Ok(f13(Str::lit("CheckedSub")))
        }
        Adt6::C16 => {
            Ok(f13(Str::lit("CheckedMul")))
        }
        Adt6::C17 => {
            Ok(f13(Str::lit("CheckedNeg")))
        }
        Adt6::C18 => {
            Ok(f13(Str::lit("CheckedQuot")))
        }
        Adt6::C19 => {
            Ok(f13(Str::lit("BitAnd")))
        }
        Adt6::C20 => {
            Ok(f13(Str::lit("BitOr")))
        }
        Adt6::C21 => {
            Ok(f13(Str::lit("BitXor")))
        }
        Adt6::C22 => {
            Ok(f13(Str::lit("BitNot")))
        }
        Adt6::C23 => {
            Ok(f13(Str::lit("ShiftLeft")))
        }
        Adt6::C24 => {
            Ok(f13(Str::lit("ShiftRight")))
        }
        Adt6::C25 => {
            Ok(f13(Str::lit("Equal")))
        }
        Adt6::C26 => {
            Ok(f13(Str::lit("BoolNot")))
        }
        Adt6::C27 => {
            Ok(f13(Str::lit("BoolAnd")))
        }
        Adt6::C28 => {
            Ok(f13(Str::lit("BoolOr")))
        }
        Adt6::C29 => {
            Ok(f13(Str::lit("Append")))
        }
        Adt6::C30 => {
            Ok(f13(Str::lit("Length")))
        }
        Adt6::C31 => {
            Ok(f13(Str::lit("Index")))
        }
        Adt6::C32 => {
            Ok(f13(Str::lit("Slice")))
        }
        Adt6::C33 => {
            Ok(f13(Str::lit("Utf8Encode")))
        }
        Adt6::C34 => {
            Ok(f13(Str::lit("Utf8Decode")))
        }
        Adt6::C35 => {
            Ok(f13(Str::lit("CompareBytes")))
        }
        Adt6::C36 => {
            Ok(f13(Str::lit("SplitExact")))
        }
        Adt6::C37 => {
            Ok(f13(Str::lit("Join")))
        }
        Adt6::C38 => {
            Ok(f13(Str::lit("FormatDecimal")))
        }
        Adt6::C39 => {
            Ok(f13(Str::lit("Compare")))
        }
        Adt6::C40(v1) => {
            Ok(f23(f13(Str::lit("Convert { target: ")), f23(f63(v1), f13(Str::lit(" }")))))
        }
        Adt6::C41(v2) => {
            Ok(f23(f13(Str::lit("ParseDecimal { target: ")), f23(f35(v2)?, f13(Str::lit(" }")))))
        }
    }
}

pub fn f83(v0: Adt0) -> R<Bytes> {
    Ok(f23(f13(Str::lit("[")), f23(f35(v0)?, f13(Str::lit("]")))))
}

pub fn f84(v0: Adt5) -> u64 {
    let m618 = v0;
    match m618 {
        Adt5::C0 => {
            0u64
        }
        Adt5::C1 => {
            1u64
        }
        Adt5::C2 => {
            2u64
        }
        Adt5::C3 => {
            3u64
        }
        Adt5::C4 => {
            4u64
        }
        Adt5::C5 => {
            5u64
        }
        Adt5::C6 => {
            6u64
        }
        Adt5::C7 => {
            7u64
        }
        Adt5::C8 => {
            8u64
        }
        Adt5::C9 => {
            9u64
        }
        Adt5::C10 => {
            10u64
        }
        Adt5::C11 => {
            11u64
        }
        Adt5::C12 => {
            12u64
        }
        Adt5::C13 => {
            13u64
        }
        Adt5::C14 => {
            14u64
        }
        Adt5::C15(_) => {
            15u64
        }
    }
}

pub fn f85(v0: Adt5) -> u64 {
    let m619 = v0;
    match m619 {
        Adt5::C0 => {
            0u64
        }
        Adt5::C1 => {
            0u64
        }
        Adt5::C2 => {
            0u64
        }
        Adt5::C3 => {
            0u64
        }
        Adt5::C4 => {
            0u64
        }
        Adt5::C5 => {
            0u64
        }
        Adt5::C6 => {
            0u64
        }
        Adt5::C7 => {
            0u64
        }
        Adt5::C8 => {
            0u64
        }
        Adt5::C9 => {
            0u64
        }
        Adt5::C10 => {
            0u64
        }
        Adt5::C11 => {
            0u64
        }
        Adt5::C12 => {
            0u64
        }
        Adt5::C13 => {
            0u64
        }
        Adt5::C14 => {
            0u64
        }
        Adt5::C15(v1) => {
            v1
        }
    }
}

pub fn f86(v0: Fn2, p624: u64, p625: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m623 = (p624, p625);
    {
        let mut m623 = m623;
        loop {
            let step = {
                let (v1, v2) = m623;
                let m620 = v1;
                if m620 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m620 - 1;
                    let m622 = {
                        let c621: Fn2 = v0.clone();
                        c621.apply(v2.clone())?
                    };
                    match m622 {
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
                Ok(next) => { m623 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f87(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f95(v0, v1)
}

pub fn f88(v0: Bytes, v1: u64, v2: u8) -> bool {
    let m626 = f96(v0, v1);
    match m626 {
        None => {
            false
        }
        Some(v3) => {
            let a627 = v3;
            let a628 = v2;
            equal(a627, a628)
        }
    }
}

pub fn f89(v0: Bytes, v1: Bytes) -> R<bool> {
    let a630 = 0u64;
    let a631 = {
        let a629 = v0.clone();
        length_bytes(a629)
    };
    if nat_lt(a630, a631) {
        if f97(v0.clone())? {
            let a632 = v0.clone();
            let a633 = f13(Str::lit("0"));
            if equal(a632, a633) {
                let a636 = {
                    let a634 = v1;
                    let a635 = f13(Str::lit("-0"));
                    equal(a634, a635)
                };
                Ok(bool_not(a636))
            } else {
                let a637 = f88(v0, 0u64, 48u8);
                Ok(bool_not(a637))
            }
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f90(v0: Bytes) -> Bytes {
    f23(f13(Str::lit("`")), f23(v0, f13(Str::lit("`"))))
}

pub fn f91(v0: Bytes, v1: Bytes) -> bool {
    let a642 = {
        let a638 = v0;
        let a639 = v1;
        compare_bytes(a638, a639)
    };
    let a643 = {
        let a640 = Bytes::lit(&[]);
        let a641 = Bytes::lit(&[0x01]);
        compare_bytes(a640, a641)
    };
    equal(a642, a643)
}

pub fn f92(v0: bool, v1: Str) -> Adt9 {
    Adt9::C0(v0, f13(v1))
}

pub fn f93(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f98(v0, v1)
}

pub fn f94(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f88(v0, v1, 48u8) {
        Ok(Some({
            let a644 = v1;
            let a645 = 1u64;
            nat_add(a644, a645)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f95(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let v2: u64 = {
        let (h646, _) = v1;
        h646
    };
    let v3: u64 = f99(v0.clone(), v2)?;
    let a647 = v3;
    let a648 = 256u64;
    if nat_eq(a647, a648) {
        Ok(None::<(u64, Bytes)>)
    } else {
        let v4: (u64, Bytes) = f100(v0, v2, v3)?;
        Ok(Some(({
            let a650 = v2;
            let a651 = {
                let (h649, _) = v4;
                h649
            };
            nat_add(a650, a651)?
        }, f23({
            let (_, h652) = v1;
            h652
        }, {
            let (_, h653) = v4;
            h653
        }))))
    }
}

pub fn f96(v0: Bytes, v1: u64) -> Option<u8> {
    let a654 = v0;
    let a655 = v1;
    index_bytes(a654, a655)
}

pub fn f97(v0: Bytes) -> R<bool> {
    let a659 = {
        let (h656, _) = f79(Fn1::F101(v0.clone()), {
            let a657 = v0.clone();
            length_bytes(a657)
        }, 0u64)?;
        h656
    };
    let a660 = {
        let a658 = v0;
        length_bytes(a658)
    };
    Ok(nat_eq(a659, a660))
}

pub fn f98(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f102(v0, v1)? {
        Ok(Some({
            let a661 = v1;
            let a662 = 1u64;
            nat_add(a661, a662)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f99(v0: Bytes, v1: u64) -> R<u64> {
    let m663 = f96(v0, v1);
    match m663 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a708 = {
                let a684 = {
                    let a672 = {
                        let a666 = {
                            let a664 = v3;
                            let a665 = 128u8;
                            fixed_u8::bit_and(a664, a665)
                        };
                        let a667 = 128u8;
                        if equal(a666, a667) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a673 = {
                        let a670 = {
                            let a668 = v3;
                            let a669 = 64u8;
                            fixed_u8::bit_and(a668, a669)
                        };
                        let a671 = 64u8;
                        if equal(a670, a671) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a672, a673)?
                };
                let a685 = {
                    let a682 = {
                        let a676 = {
                            let a674 = v3;
                            let a675 = 32u8;
                            fixed_u8::bit_and(a674, a675)
                        };
                        let a677 = 32u8;
                        if equal(a676, a677) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a683 = {
                        let a680 = {
                            let a678 = v3;
                            let a679 = 16u8;
                            fixed_u8::bit_and(a678, a679)
                        };
                        let a681 = 16u8;
                        if equal(a680, a681) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a682, a683)?
                };
                nat_add(a684, a685)?
            };
            let a709 = {
                let a706 = {
                    let a694 = {
                        let a688 = {
                            let a686 = v3;
                            let a687 = 8u8;
                            fixed_u8::bit_and(a686, a687)
                        };
                        let a689 = 8u8;
                        if equal(a688, a689) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a695 = {
                        let a692 = {
                            let a690 = v3;
                            let a691 = 4u8;
                            fixed_u8::bit_and(a690, a691)
                        };
                        let a693 = 4u8;
                        if equal(a692, a693) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a694, a695)?
                };
                let a707 = {
                    let a704 = {
                        let a698 = {
                            let a696 = v3;
                            let a697 = 2u8;
                            fixed_u8::bit_and(a696, a697)
                        };
                        let a699 = 2u8;
                        if equal(a698, a699) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a705 = {
                        let a702 = {
                            let a700 = v3;
                            let a701 = 1u8;
                            fixed_u8::bit_and(a700, a701)
                        };
                        let a703 = 1u8;
                        if equal(a702, a703) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a704, a705)?
                };
                nat_add(a706, a707)?
            };
            nat_add(a708, a709)
        }
    }
}

pub fn f100(v0: Bytes, v1: u64, v2: u64) -> R<(u64, Bytes)> {
    let v3: u64 = f99(v0.clone(), {
        let a710 = v1;
        let a711 = 1u64;
        nat_add(a710, a711)?
    })?;
    let a712 = v2;
    let a713 = 34u64;
    if nat_eq(a712, a713) {
        Ok((1u64, f13(Str::lit("\\\""))))
    } else {
        let a714 = v2;
        let a715 = 92u64;
        if nat_eq(a714, a715) {
            Ok((1u64, f13(Str::lit("\\\\"))))
        } else {
            let a716 = v2;
            let a717 = 10u64;
            if nat_eq(a716, a717) {
                Ok((1u64, f13(Str::lit("\\n"))))
            } else {
                let a718 = v2;
                let a719 = 13u64;
                if nat_eq(a718, a719) {
                    Ok((1u64, f13(Str::lit("\\r"))))
                } else {
                    let a720 = v2;
                    let a721 = 9u64;
                    if nat_eq(a720, a721) {
                        Ok((1u64, f13(Str::lit("\\t"))))
                    } else {
                        let a722 = v2;
                        let a723 = 0u64;
                        if nat_eq(a722, a723) {
                            Ok((1u64, f13(Str::lit("\\0"))))
                        } else {
                            let a724 = v2;
                            let a725 = 32u64;
                            if nat_lt(a724, a725) {
                                Ok((1u64, f103(v2)))
                            } else {
                                let a726 = v2;
                                let a727 = 127u64;
                                if nat_eq(a726, a727) {
                                    Ok((1u64, f103(v2)))
                                } else {
                                    let a728 = v2;
                                    let a729 = 194u64;
                                    if nat_eq(a728, a729) {
                                        let a730 = 128u64;
                                        let a731 = v3;
                                        if nat_le(a730, a731) {
                                            let a732 = v3;
                                            let a733 = 159u64;
                                            if nat_le(a732, a733) {
                                                Ok((2u64, f103(v3)))
                                            } else {
                                                Ok((1u64, f81(v0, v1, 1u64)))
                                            }
                                        } else {
                                            Ok((1u64, f81(v0, v1, 1u64)))
                                        }
                                    } else {
                                        Ok((1u64, f81(v0, v1, 1u64)))
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

pub fn f101(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f104(v0, v1)
}

pub fn f102(v0: Bytes, v1: u64) -> R<bool> {
    let v2: u64 = f99(v0, v1)?;
    let a734 = 47u64;
    let a735 = v2;
    if nat_lt(a734, a735) {
        let a736 = v2;
        let a737 = 58u64;
        if nat_lt(a736, a737) {
            Ok(true)
        } else {
            let a738 = 96u64;
            let a739 = v2;
            if nat_lt(a738, a739) {
                let a740 = v2;
                let a741 = 103u64;
                Ok(nat_lt(a740, a741))
            } else {
                Ok(false)
            }
        }
    } else {
        Ok(false)
    }
}

pub fn f103(v0: u64) -> Bytes {
    f23(f13(Str::lit("\\u{")), f23(f105(v0), f13(Str::lit("}"))))
}

pub fn f104(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f106(v0, v1)? {
        Ok(Some({
            let a742 = v1;
            let a743 = 1u64;
            nat_add(a742, a743)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f105(v0: u64) -> Bytes {
    let v1: Bytes = f107(v0);
    let v2: Bytes = f23(f107({
        let a744 = v0;
        let a745 = 16u64;
        let a746 = 0u64;
        nat_quot(a744, a745, a746)
    }), f107({
        let a747 = v0;
        let a748 = 16u64;
        let a749 = 0u64;
        nat_rem(a747, a748, a749)
    }));
    let a750 = v0;
    let a751 = 16u64;
    if nat_lt(a750, a751) {
        v1
    } else {
        v2
    }
}

pub fn f106(v0: Bytes, v1: u64) -> R<bool> {
    let v2: u64 = f99(v0, v1)?;
    let a752 = 47u64;
    let a753 = v2;
    if nat_lt(a752, a753) {
        let a754 = v2;
        let a755 = 58u64;
        Ok(nat_lt(a754, a755))
    } else {
        Ok(false)
    }
}

pub fn f107(v0: u64) -> Bytes {
    f81(f13(Str::lit("0123456789abcdef")), v0, 1u64)
}

pub fn run(p0: Bytes, p1: Adt0, p2: List<List<u64>>, p3: Adt0, p4: Adt0, p5: Adt2) -> R<Adt0> {
    f0(p0, p1, p2, p3, p4, p5)
}
