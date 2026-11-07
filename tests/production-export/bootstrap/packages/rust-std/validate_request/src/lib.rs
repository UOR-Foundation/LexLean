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
    C0(u64, Adt1, Adt8, Adt9, Adt10, Adt11, Adt12),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(u64, Adt2, Adt3, List<Adt4>, Adt7, List<Adt5>),
}

#[derive(Clone)]
pub enum Adt2 {
    C0(u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt3 {
    C0,
    C1,
}

#[derive(Clone)]
pub enum Adt4 {
    C0(Adt5, Adt6),
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
}

#[derive(Clone)]
pub enum Adt6 {
    C0,
    C1(u64),
    C2,
    C3,
}

#[derive(Clone)]
pub enum Adt7 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt8 {
    C0(u64, List<Adt5>, u64),
    C1,
    C2,
    C3,
    C4,
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
    C0,
    C1,
}

#[derive(Clone)]
pub enum Adt11 {
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
}

#[derive(Clone)]
pub enum Adt12 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt13 {
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
    C10(Adt5),
    C11(Adt5),
    C12(Adt5),
    C13(Adt5),
    C14(Adt5),
    C15(Adt5),
    C16,
    C17,
    C18,
    C19,
    C20,
}

#[derive(Clone)]
pub enum Adt14 {
    C0(List<Bytes>, Bytes, u64, u64, bool),
}

#[derive(Clone)]
pub enum Fn0 {
    F15,
}

impl Fn0 {
    pub fn apply(&self, p0: List<Bytes>) -> Option<List<Bytes>> {
        match self {
            Fn0::F15 => f15(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F17,
}

impl Fn1 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn1::F17 => f17(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F25,
}

impl Fn2 {
    pub fn apply(&self, p0: (Option<Bytes>, List<Bytes>), p1: Bytes) -> (Option<Bytes>, List<Bytes>) {
        match self {
            Fn2::F25 => f25(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F27(Adt1),
}

impl Fn3 {
    pub fn apply(&self, p0: bool, p1: Adt4) -> bool {
        match self {
            Fn3::F27(k0) => f27(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F32(List<Adt4>),
    F42(Adt1),
}

impl Fn4 {
    pub fn apply(&self, p0: Option<Adt13>, p1: Adt4) -> R<Option<Adt13>> {
        match self {
            Fn4::F32(k0) => f32(k0.clone(), p0, p1),
            Fn4::F42(k0) => Ok(f42(k0.clone(), p0, p1)),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F40(Adt5),
}

impl Fn5 {
    pub fn apply(&self, p0: u64, p1: Adt4) -> R<u64> {
        match self {
            Fn5::F40(k0) => f40(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F47(Adt5),
}

impl Fn6 {
    pub fn apply(&self, p0: Option<Adt6>, p1: Adt4) -> Option<Adt6> {
        match self {
            Fn6::F47(k0) => f47(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F50(Adt1),
    F57(Adt0),
}

impl Fn7 {
    pub fn apply(&self, p0: Option<Adt13>, p1: Adt5) -> Option<Adt13> {
        match self {
            Fn7::F50(k0) => f50(k0.clone(), p0, p1),
            Fn7::F57(k0) => f57(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F61(Adt5),
}

impl Fn8 {
    pub fn apply(&self, p0: bool, p1: Adt5) -> bool {
        match self {
            Fn8::F61(k0) => f61(k0.clone(), p0, p1),
        }
    }
}

pub fn f0(v0: Adt0) -> R<Result<Option<Bytes>, Bytes>> {
    let m1 = f1(v0)?;
    match m1 {
        None => {
            Ok(Ok::<Option<Bytes>, Bytes>(None::<Bytes>))
        }
        Some(v1) => {
            let m2 = f2(v1)?;
            match m2 {
                Ok(v2) => {
                    Ok(Ok::<Option<Bytes>, Bytes>(Some(v2)))
                }
                Err(v3) => {
                    Ok(Err::<Option<Bytes>, Bytes>(v3))
                }
            }
        }
    }
}

pub fn f1(v0: Adt0) -> R<Option<Adt13>> {
    let a4 = match v0.clone() {
        Adt0::C0(h3, _, _, _, _, _, _) => {
            h3
        }
    };
    let a5 = 0u64;
    if nat_eq(a4, a5) {
        Ok(Some(Adt13::C0))
    } else {
        f3(v0)
    }
}

pub fn f2(v0: Adt13) -> R<Result<Bytes, Bytes>> {
    let m6 = v0;
    match m6 {
        Adt13::C0 => {
            Ok(f4(f5(f5(f6(256u64), {
                let a7 = Str::lit("{\"kind\":\"empty_domain\"");
                utf8_encode(a7)
            })?, {
                let a8 = Str::lit("}");
                utf8_encode(a8)
            })?))
        }
        Adt13::C1 => {
            Ok(f4(f5(f5(f6(256u64), {
                let a9 = Str::lit("{\"kind\":\"internal_plan_universe\"");
                utf8_encode(a9)
            })?, {
                let a10 = Str::lit("}");
                utf8_encode(a10)
            })?))
        }
        Adt13::C2 => {
            Ok(f4(f5(f5(f6(256u64), {
                let a11 = Str::lit("{\"kind\":\"optimizer_defined_universe\"");
                utf8_encode(a11)
            })?, {
                let a12 = Str::lit("}");
                utf8_encode(a12)
            })?))
        }
        Adt13::C3 => {
            Ok(f4(f5(f5(f6(256u64), {
                let a13 = Str::lit("{\"kind\":\"discovered_universe\"");
                utf8_encode(a13)
            })?, {
                let a14 = Str::lit("}");
                utf8_encode(a14)
            })?))
        }
        Adt13::C4 => {
            Ok(f4(f5(f5(f6(256u64), {
                let a15 = Str::lit("{\"kind\":\"cached_universe\"");
                utf8_encode(a15)
            })?, {
                let a16 = Str::lit("}");
                utf8_encode(a16)
            })?))
        }
        Adt13::C5 => {
            Ok(f4(f5(f5(f6(256u64), {
                let a17 = Str::lit("{\"kind\":\"missing_completeness\"");
                utf8_encode(a17)
            })?, {
                let a18 = Str::lit("}");
                utf8_encode(a18)
            })?))
        }
        Adt13::C6 => {
            Ok(f4(f5(f5(f6(256u64), {
                let a19 = Str::lit("{\"kind\":\"self_referential_completeness\"");
                utf8_encode(a19)
            })?, {
                let a20 = Str::lit("}");
                utf8_encode(a20)
            })?))
        }
        Adt13::C7 => {
            Ok(f4(f5(f5(f6(256u64), {
                let a21 = Str::lit("{\"kind\":\"optimizer_completeness\"");
                utf8_encode(a21)
            })?, {
                let a22 = Str::lit("}");
                utf8_encode(a22)
            })?))
        }
        Adt13::C8 => {
            Ok(f4(f5(f5(f6(256u64), {
                let a23 = Str::lit("{\"kind\":\"beyond_capacity\"");
                utf8_encode(a23)
            })?, {
                let a24 = Str::lit("}");
                utf8_encode(a24)
            })?))
        }
        Adt13::C9 => {
            Ok(f4(f5(f5(f6(256u64), {
                let a25 = Str::lit("{\"kind\":\"unit_cost_operands\"");
                utf8_encode(a25)
            })?, {
                let a26 = Str::lit("}");
                utf8_encode(a26)
            })?))
        }
        Adt13::C10(v1) => {
            Ok(f4(f5(f5(f5(f5(f6(256u64), {
                let a27 = Str::lit("{\"kind\":\"duplicate_action\"");
                utf8_encode(a27)
            })?, {
                let a28 = Str::lit(",\"action\":\"");
                utf8_encode(a28)
            })?, f7(v1))?, {
                let a29 = Str::lit("\"}");
                utf8_encode(a29)
            })?))
        }
        Adt13::C11(v2) => {
            Ok(f4(f5(f5(f5(f5(f6(256u64), {
                let a30 = Str::lit("{\"kind\":\"unaccounted_action\"");
                utf8_encode(a30)
            })?, {
                let a31 = Str::lit(",\"action\":\"");
                utf8_encode(a31)
            })?, f7(v2))?, {
                let a32 = Str::lit("\"}");
                utf8_encode(a32)
            })?))
        }
        Adt13::C12(v3) => {
            Ok(f4(f5(f5(f5(f5(f6(256u64), {
                let a33 = Str::lit("{\"kind\":\"hidden_cost\"");
                utf8_encode(a33)
            })?, {
                let a34 = Str::lit(",\"action\":\"");
                utf8_encode(a34)
            })?, f7(v3))?, {
                let a35 = Str::lit("\"}");
                utf8_encode(a35)
            })?))
        }
        Adt13::C13(v4) => {
            Ok(f4(f5(f5(f5(f5(f6(256u64), {
                let a36 = Str::lit("{\"kind\":\"unrealizable_action\"");
                utf8_encode(a36)
            })?, {
                let a37 = Str::lit(",\"action\":\"");
                utf8_encode(a37)
            })?, f7(v4))?, {
                let a38 = Str::lit("\"}");
                utf8_encode(a38)
            })?))
        }
        Adt13::C14(v5) => {
            Ok(f4(f5(f5(f5(f5(f6(256u64), {
                let a39 = Str::lit("{\"kind\":\"unbound_preparation\"");
                utf8_encode(a39)
            })?, {
                let a40 = Str::lit(",\"action\":\"");
                utf8_encode(a40)
            })?, f7(v5))?, {
                let a41 = Str::lit("\"}");
                utf8_encode(a41)
            })?))
        }
        Adt13::C15(v6) => {
            Ok(f4(f5(f5(f5(f5(f6(256u64), {
                let a42 = Str::lit("{\"kind\":\"stray_prepared_artifact\"");
                utf8_encode(a42)
            })?, {
                let a43 = Str::lit(",\"action\":\"");
                utf8_encode(a43)
            })?, f7(v6))?, {
                let a44 = Str::lit("\"}");
                utf8_encode(a44)
            })?))
        }
        Adt13::C16 => {
            Ok(f4(f5(f5(f6(256u64), {
                let a45 = Str::lit("{\"kind\":\"claim_alias\"");
                utf8_encode(a45)
            })?, {
                let a46 = Str::lit("}");
                utf8_encode(a46)
            })?))
        }
        Adt13::C17 => {
            Ok(f4(f5(f5(f6(256u64), {
                let a47 = Str::lit("{\"kind\":\"scalar_claim_over_partial_order\"");
                utf8_encode(a47)
            })?, {
                let a48 = Str::lit("}");
                utf8_encode(a48)
            })?))
        }
        Adt13::C18 => {
            Ok(f4(f5(f5(f6(256u64), {
                let a49 = Str::lit("{\"kind\":\"vector_claim_over_total_order\"");
                utf8_encode(a49)
            })?, {
                let a50 = Str::lit("}");
                utf8_encode(a50)
            })?))
        }
        Adt13::C19 => {
            Ok(f4(f5(f5(f6(256u64), {
                let a51 = Str::lit("{\"kind\":\"uncovered_scope\"");
                utf8_encode(a51)
            })?, {
                let a52 = Str::lit("}");
                utf8_encode(a52)
            })?))
        }
        Adt13::C20 => {
            Ok(f4(f5(f5(f6(256u64), {
                let a53 = Str::lit("{\"kind\":\"unsupported_claim\"");
                utf8_encode(a53)
            })?, {
                let a54 = Str::lit("}");
                utf8_encode(a54)
            })?))
        }
    }
}

pub fn f3(v0: Adt0) -> R<Option<Adt13>> {
    let m56 = match v0.clone() {
        Adt0::C0(_, _, h55, _, _, _, _) => {
            h55
        }
    };
    match m56 {
        Adt8::C0(_, _, _) => {
            f8(v0)
        }
        Adt8::C1 => {
            Ok(Some(Adt13::C1))
        }
        Adt8::C2 => {
            Ok(Some(Adt13::C2))
        }
        Adt8::C3 => {
            Ok(Some(Adt13::C3))
        }
        Adt8::C4 => {
            Ok(Some(Adt13::C4))
        }
    }
}

pub fn f4(v0: Adt14) -> Result<Bytes, Bytes> {
    if match v0.clone() {
        Adt14::C0(_, _, _, _, h57) => {
            h57
        }
    } {
        Err::<Bytes, Bytes>({
            let a58 = Str::lit("output exceeds the capacity of the buffer");
            utf8_encode(a58)
        })
    } else {
        Ok::<Bytes, Bytes>(f9(f10(List::cons(match v0.clone() {
            Adt14::C0(_, h59, _, _, _) => {
                h59
            }
        }, match v0 {
            Adt14::C0(h60, _, _, _, _) => {
                h60
            }
        }))))
    }
}

pub fn f5(v0: Adt14, v1: Bytes) -> R<Adt14> {
    if match v0.clone() {
        Adt14::C0(_, _, _, _, h61) => {
            h61
        }
    } {
        Ok(v0)
    } else {
        let v2: u64 = {
            let a64 = match v0.clone() {
                Adt14::C0(_, _, h62, _, _) => {
                    h62
                }
            };
            let a65 = {
                let a63 = v1.clone();
                length_bytes(a63)
            };
            nat_add(a64, a65)?
        };
        let a67 = v2;
        let a68 = match v0.clone() {
            Adt14::C0(_, _, _, h66, _) => {
                h66
            }
        };
        if nat_le(a67, a68) {
            Ok(f11(v0.clone(), {
                let a70 = match v0 {
                    Adt14::C0(_, h69, _, _, _) => {
                        h69
                    }
                };
                let a71 = v1;
                append_bytes(a70, a71)
            }, v2))
        } else {
            Ok(f12(v0))
        }
    }
}

pub fn f6(v0: u64) -> Adt14 {
    Adt14::C0(List::<Bytes>::nil(), Bytes::lit(&[]), 0u64, v0, false)
}

pub fn f7(v0: Adt5) -> Bytes {
    let m72 = v0;
    match m72 {
        Adt5::C0 => {
            let a73 = Str::lit("observation");
            utf8_encode(a73)
        }
        Adt5::C1 => {
            let a74 = Str::lit("preprocessing");
            utf8_encode(a74)
        }
        Adt5::C2 => {
            let a75 = Str::lit("advice");
            utf8_encode(a75)
        }
        Adt5::C3 => {
            let a76 = Str::lit("retained_state");
            utf8_encode(a76)
        }
        Adt5::C4 => {
            let a77 = Str::lit("dispatch");
            utf8_encode(a77)
        }
        Adt5::C5 => {
            let a78 = Str::lit("fallback");
            utf8_encode(a78)
        }
        Adt5::C6 => {
            let a79 = Str::lit("communication");
            utf8_encode(a79)
        }
        Adt5::C7 => {
            let a80 = Str::lit("randomness");
            utf8_encode(a80)
        }
        Adt5::C8 => {
            let a81 = Str::lit("scheduling");
            utf8_encode(a81)
        }
        Adt5::C9 => {
            let a82 = Str::lit("execution");
            utf8_encode(a82)
        }
    }
}

pub fn f8(v0: Adt0) -> R<Option<Adt13>> {
    let m84 = match v0.clone() {
        Adt0::C0(_, _, _, h83, _, _, _) => {
            h83
        }
    };
    match m84 {
        Adt9::C0 => {
            f13(v0)
        }
        Adt9::C1 => {
            Ok(Some(Adt13::C5))
        }
        Adt9::C2 => {
            Ok(Some(Adt13::C6))
        }
        Adt9::C3 => {
            Ok(Some(Adt13::C7))
        }
    }
}

pub fn f9(v0: List<Bytes>) -> Bytes {
    let m87 = {
        let (h85, _) = f14(Fn0::F15, {
            let a86 = v0.clone();
            length_list(a86)
        }, v0);
        h85
    };
    match m87.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f10(v0: List<Bytes>) -> List<Bytes> {
    f16(Fn1::F17, List::<Bytes>::nil(), v0)
}

pub fn f11(v0: Adt14, v1: Bytes, v2: u64) -> Adt14 {
    let a89 = 256u64;
    let a90 = {
        let a88 = v1.clone();
        length_bytes(a88)
    };
    if nat_le(a89, a90) {
        Adt14::C0(List::cons(v1, match v0.clone() {
            Adt14::C0(h91, _, _, _, _) => {
                h91
            }
        }), Bytes::lit(&[]), v2, match v0 {
            Adt14::C0(_, _, _, h92, _) => {
                h92
            }
        }, false)
    } else {
        Adt14::C0(match v0.clone() {
            Adt14::C0(h93, _, _, _, _) => {
                h93
            }
        }, v1, v2, match v0 {
            Adt14::C0(_, _, _, h94, _) => {
                h94
            }
        }, false)
    }
}

pub fn f12(v0: Adt14) -> Adt14 {
    Adt14::C0(match v0.clone() {
        Adt14::C0(h95, _, _, _, _) => {
            h95
        }
    }, match v0.clone() {
        Adt14::C0(_, h96, _, _, _) => {
            h96
        }
    }, match v0.clone() {
        Adt14::C0(_, _, h97, _, _) => {
            h97
        }
    }, match v0 {
        Adt14::C0(_, _, _, h98, _) => {
            h98
        }
    }, true)
}

pub fn f13(v0: Adt0) -> R<Option<Adt13>> {
    let m99 = f18(v0.clone())?;
    match m99 {
        None => {
            f19(v0)
        }
        Some(v1) => {
            Ok(Some(v1))
        }
    }
}

pub fn f14(v0: Fn0, p104: u64, p105: List<Bytes>) -> (List<Bytes>, bool) {
    let m103 = (p104, p105);
    {
        let mut m103 = m103;
        loop {
            let step = {
                let (v1, v2) = m103;
                let m100 = v1;
                if m100 == 0 {
                    Err::<(u64, List<Bytes>), (List<Bytes>, bool)>((v2, false))
                } else {
                    let v3 = m100 - 1;
                    let m102 = {
                        let c101: Fn0 = v0.clone();
                        c101.apply(v2.clone())
                    };
                    match m102 {
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
                Ok(next) => { m103 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f15(v0: List<Bytes>) -> Option<List<Bytes>> {
    f20(v0)
}

pub fn f16(v0: Fn1, p109: List<Bytes>, p110: List<Bytes>) -> List<Bytes> {
    let m108 = (p109, p110);
    {
        let mut m108 = m108;
        loop {
            let step = {
                let (v1, v2) = m108;
                let m106 = v2;
                match m106.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c107: Fn1 = v0.clone();
                            c107.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m108 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f17(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f18(v0: Adt0) -> R<Option<Adt13>> {
    let v1: Adt1 = match v0.clone() {
        Adt0::C0(_, h111, _, _, _, _, _) => {
            h111
        }
    };
    let v2: Adt2 = match v1.clone() {
        Adt1::C0(_, h112, _, _, _, _) => {
            h112
        }
    };
    let v6: bool = {
        let m114 = match v0.clone() {
            Adt0::C0(_, _, h113, _, _, _, _) => {
                h113
            }
        };
        match m114 {
            Adt8::C0(v3, _, v5) => {
                let a116 = match v2.clone() {
                    Adt2::C0(_, _, h115, _) => {
                        h115
                    }
                };
                let a117 = f21(v3, v5)?;
                nat_lt(a116, a117)
            }
            Adt8::C1 => {
                false
            }
            Adt8::C2 => {
                false
            }
            Adt8::C3 => {
                false
            }
            Adt8::C4 => {
                false
            }
        }
    };
    let a120 = match v2.clone() {
        Adt2::C0(h118, _, _, _) => {
            h118
        }
    };
    let a121 = match v1.clone() {
        Adt1::C0(h119, _, _, _, _, _) => {
            h119
        }
    };
    if if nat_lt(a120, a121) {
        true
    } else {
        let a124 = match v2 {
            Adt2::C0(_, h122, _, _) => {
                h122
            }
        };
        let a125 = match v0 {
            Adt0::C0(h123, _, _, _, _, _, _) => {
                h123
            }
        };
        if nat_lt(a124, a125) {
            true
        } else {
            if v6 {
                true
            } else {
                f22(v1)
            }
        }
    } {
        Ok(Some(Adt13::C8))
    } else {
        Ok(None::<Adt13>)
    }
}

pub fn f19(v0: Adt0) -> R<Option<Adt13>> {
    let m129 = {
        let m128 = match v0.clone() {
            Adt0::C0(_, h127, _, _, _, _, _) => {
                h127
            }
        };
        match m128 {
            Adt1::C0(_, _, h126, _, _, _) => {
                h126
            }
        }
    };
    match m129 {
        Adt3::C0 => {
            f23(v0)
        }
        Adt3::C1 => {
            Ok(Some(Adt13::C9))
        }
    }
}

pub fn f20(v0: List<Bytes>) -> Option<List<Bytes>> {
    let m130 = v0.clone();
    match m130.uncons() {
        None => {
            None::<List<Bytes>>
        }
        Some((_, v2)) => {
            let m131 = v2;
            match m131.uncons() {
                None => {
                    None::<List<Bytes>>
                }
                Some((_, _)) => {
                    let v5: (Option<Bytes>, List<Bytes>) = f24(Fn2::F25, (None::<Bytes>, List::<Bytes>::nil()), v0);
                    let m133 = {
                        let (h132, _) = v5;
                        h132
                    };
                    match m133 {
                        None => {
                            Some(f10({
                                let (_, h135) = v5;
                                h135
                            }))
                        }
                        Some(v6) => {
                            Some(f10(List::cons(v6, {
                                let (_, h134) = v5;
                                h134
                            })))
                        }
                    }
                }
            }
        }
    }
}

pub fn f21(v0: u64, v1: u64) -> R<u64> {
    let a140 = v0;
    let a141 = {
        let a138 = v1;
        let a139 = {
            let a136 = v0;
            let a137 = v0;
            nat_mul(a136, a137)?
        };
        nat_mul(a138, a139)?
    };
    nat_add(a140, a141)
}

pub fn f22(v0: Adt1) -> bool {
    f26(Fn3::F27(v0.clone()), false, match v0 {
        Adt1::C0(_, _, _, h142, _, _) => {
            h142
        }
    })
}

pub fn f23(v0: Adt0) -> R<Option<Adt13>> {
    let m146 = f28({
        let m145 = match v0.clone() {
            Adt0::C0(_, h144, _, _, _, _, _) => {
                h144
            }
        };
        match m145 {
            Adt1::C0(_, _, _, h143, _, _) => {
                h143
            }
        }
    })?;
    match m146 {
        None => {
            f29(v0)
        }
        Some(v1) => {
            Ok(Some(v1))
        }
    }
}

pub fn f24(v0: Fn2, p150: (Option<Bytes>, List<Bytes>), p151: List<Bytes>) -> (Option<Bytes>, List<Bytes>) {
    let m149 = (p150, p151);
    {
        let mut m149 = m149;
        loop {
            let step = {
                let (v1, v2) = m149;
                let m147 = v2;
                match m147.uncons() {
                    None => {
                        Err::<((Option<Bytes>, List<Bytes>), List<Bytes>), (Option<Bytes>, List<Bytes>)>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<((Option<Bytes>, List<Bytes>), List<Bytes>), (Option<Bytes>, List<Bytes>)>(({
                            let c148: Fn2 = v0.clone();
                            c148.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m149 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f25(v0: (Option<Bytes>, List<Bytes>), v1: Bytes) -> (Option<Bytes>, List<Bytes>) {
    f30(v0, v1)
}

pub fn f26(v0: Fn3, p155: bool, p156: List<Adt4>) -> bool {
    let m154 = (p155, p156);
    {
        let mut m154 = m154;
        loop {
            let step = {
                let (v1, v2) = m154;
                let m152 = v2;
                match m152.uncons() {
                    None => {
                        Err::<(bool, List<Adt4>), bool>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<Adt4>), bool>(({
                            let c153: Fn3 = v0.clone();
                            c153.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m154 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f27(v0: Adt1, v1: bool, v2: Adt4) -> bool {
    if v1 {
        true
    } else {
        let m158 = match v2 {
            Adt4::C0(_, h157) => {
                h157
            }
        };
        match m158 {
            Adt6::C0 => {
                false
            }
            Adt6::C1(v3) => {
                let a162 = {
                    let m161 = match v0 {
                        Adt1::C0(_, h160, _, _, _, _) => {
                            h160
                        }
                    };
                    match m161 {
                        Adt2::C0(_, _, _, h159) => {
                            h159
                        }
                    }
                };
                let a163 = v3;
                nat_lt(a162, a163)
            }
            Adt6::C2 => {
                false
            }
            Adt6::C3 => {
                false
            }
        }
    }
}

pub fn f28(v0: List<Adt4>) -> R<Option<Adt13>> {
    f31(Fn4::F32(v0.clone()), None::<Adt13>, v0)
}

pub fn f29(v0: Adt0) -> R<Option<Adt13>> {
    let m165 = f33(match v0.clone() {
        Adt0::C0(_, h164, _, _, _, _, _) => {
            h164
        }
    });
    match m165 {
        None => {
            f34(v0)
        }
        Some(v1) => {
            Ok(Some(v1))
        }
    }
}

pub fn f30(v0: (Option<Bytes>, List<Bytes>), v1: Bytes) -> (Option<Bytes>, List<Bytes>) {
    let m167 = {
        let (h166, _) = v0;
        h166
    };
    match m167 {
        None => {
            (Some(v1), {
                let (_, h168) = v0;
                h168
            })
        }
        Some(v2) => {
            (None::<Bytes>, List::cons({
                let a169 = v2;
                let a170 = v1;
                append_bytes(a169, a170)
            }, {
                let (_, h171) = v0;
                h171
            }))
        }
    }
}

pub fn f31(v0: Fn4, p175: Option<Adt13>, p176: List<Adt4>) -> R<Option<Adt13>> {
    let m174 = (p175, p176);
    {
        let mut m174 = m174;
        loop {
            let step = {
                let (v1, v2) = m174;
                let m172 = v2;
                match m172.uncons() {
                    None => {
                        Err::<(Option<Adt13>, List<Adt4>), R<Option<Adt13>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Adt13>, List<Adt4>), R<Option<Adt13>>>(({
                            let c173: Fn4 = v0.clone();
                            c173.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m174 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f32(v0: List<Adt4>, v1: Option<Adt13>, v2: Adt4) -> R<Option<Adt13>> {
    let m177 = v1;
    match m177 {
        None => {
            let a179 = 1u64;
            let a180 = f35(v0, match v2.clone() {
                Adt4::C0(h178, _) => {
                    h178
                }
            })?;
            if nat_lt(a179, a180) {
                Ok(Some(Adt13::C10(match v2 {
                    Adt4::C0(h181, _) => {
                        h181
                    }
                })))
            } else {
                Ok(None::<Adt13>)
            }
        }
        Some(v3) => {
            Ok(Some(v3))
        }
    }
}

pub fn f33(v0: Adt1) -> Option<Adt13> {
    let m182 = f36(v0.clone(), Adt5::C0);
    match m182 {
        None => {
            let m183 = f36(v0.clone(), Adt5::C4);
            match m183 {
                None => {
                    let m184 = f36(v0.clone(), Adt5::C5);
                    match m184 {
                        None => {
                            f36(v0, Adt5::C9)
                        }
                        Some(v3) => {
                            Some(v3)
                        }
                    }
                }
                Some(v2) => {
                    Some(v2)
                }
            }
        }
        Some(v1) => {
            Some(v1)
        }
    }
}

pub fn f34(v0: Adt0) -> R<Option<Adt13>> {
    let m186 = f37(match v0.clone() {
        Adt0::C0(_, h185, _, _, _, _, _) => {
            h185
        }
    })?;
    match m186 {
        None => {
            Ok(f38(v0))
        }
        Some(v1) => {
            Ok(Some(v1))
        }
    }
}

pub fn f35(v0: List<Adt4>, v1: Adt5) -> R<u64> {
    f39(Fn5::F40(v1), 0u64, v0)
}

pub fn f36(v0: Adt1, v1: Adt5) -> Option<Adt13> {
    let m188 = f41(match v0 {
        Adt1::C0(_, _, _, h187, _, _) => {
            h187
        }
    }, v1.clone());
    match m188 {
        None => {
            Some(Adt13::C11(v1))
        }
        Some(v2) => {
            let m189 = v2;
            match m189 {
                Adt6::C0 => {
                    None::<Adt13>
                }
                Adt6::C1(_) => {
                    Some(Adt13::C12(v1))
                }
                Adt6::C2 => {
                    Some(Adt13::C12(v1))
                }
                Adt6::C3 => {
                    Some(Adt13::C12(v1))
                }
            }
        }
    }
}

pub fn f37(v0: Adt1) -> R<Option<Adt13>> {
    f31(Fn4::F42(v0.clone()), None::<Adt13>, match v0 {
        Adt1::C0(_, _, _, h190, _, _) => {
            h190
        }
    })
}

pub fn f38(v0: Adt0) -> Option<Adt13> {
    let m192 = f43(match v0.clone() {
        Adt0::C0(_, h191, _, _, _, _, _) => {
            h191
        }
    });
    match m192 {
        None => {
            f44(v0)
        }
        Some(v1) => {
            Some(v1)
        }
    }
}

pub fn f39(v0: Fn5, p196: u64, p197: List<Adt4>) -> R<u64> {
    let m195 = (p196, p197);
    {
        let mut m195 = m195;
        loop {
            let step = {
                let (v1, v2) = m195;
                let m193 = v2;
                match m193.uncons() {
                    None => {
                        Err::<(u64, List<Adt4>), R<u64>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(u64, List<Adt4>), R<u64>>(({
                            let c194: Fn5 = v0.clone();
                            c194.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m195 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f40(v0: Adt5, v1: u64, v2: Adt4) -> R<u64> {
    if f45(match v2 {
        Adt4::C0(h198, _) => {
            h198
        }
    }, v0) {
        let a199 = v1;
        let a200 = 1u64;
        nat_add(a199, a200)
    } else {
        Ok(v1)
    }
}

pub fn f41(v0: List<Adt4>, v1: Adt5) -> Option<Adt6> {
    f46(Fn6::F47(v1), None::<Adt6>, v0)
}

pub fn f42(v0: Adt1, v1: Option<Adt13>, v2: Adt4) -> Option<Adt13> {
    let m201 = v1;
    match m201 {
        None => {
            f48(v0, v2)
        }
        Some(v3) => {
            Some(v3)
        }
    }
}

pub fn f43(v0: Adt1) -> Option<Adt13> {
    f49(Fn7::F50(v0.clone()), None::<Adt13>, match v0 {
        Adt1::C0(_, _, _, _, _, h202) => {
            h202
        }
    })
}

pub fn f44(v0: Adt0) -> Option<Adt13> {
    let m203 = f51(v0.clone());
    match m203 {
        None => {
            f52(v0)
        }
        Some(v1) => {
            Some(v1)
        }
    }
}

pub fn f45(v0: Adt5, v1: Adt5) -> bool {
    let a204 = f53(v0);
    let a205 = f53(v1);
    nat_eq(a204, a205)
}

pub fn f46(v0: Fn6, p209: Option<Adt6>, p210: List<Adt4>) -> Option<Adt6> {
    let m208 = (p209, p210);
    {
        let mut m208 = m208;
        loop {
            let step = {
                let (v1, v2) = m208;
                let m206 = v2;
                match m206.uncons() {
                    None => {
                        Err::<(Option<Adt6>, List<Adt4>), Option<Adt6>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Adt6>, List<Adt4>), Option<Adt6>>(({
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

pub fn f47(v0: Adt5, v1: Option<Adt6>, v2: Adt4) -> Option<Adt6> {
    let m211 = v1;
    match m211 {
        None => {
            if f45(match v2.clone() {
                Adt4::C0(h212, _) => {
                    h212
                }
            }, v0) {
                Some(match v2 {
                    Adt4::C0(_, h213) => {
                        h213
                    }
                })
            } else {
                None::<Adt6>
            }
        }
        Some(v3) => {
            Some(v3)
        }
    }
}

pub fn f48(v0: Adt1, v1: Adt4) -> Option<Adt13> {
    let v2: Adt5 = match v1.clone() {
        Adt4::C0(h214, _) => {
            h214
        }
    };
    if f54(v2.clone()) {
        None::<Adt13>
    } else {
        if f55(v2.clone()) {
            let m216 = match v1 {
                Adt4::C0(_, h215) => {
                    h215
                }
            };
            match m216 {
                Adt6::C0 => {
                    Some(Adt13::C12(v2))
                }
                Adt6::C1(v3) => {
                    let a217 = 0u64;
                    let a218 = v3;
                    if nat_lt(a217, a218) {
                        None::<Adt13>
                    } else {
                        Some(Adt13::C12(v2))
                    }
                }
                Adt6::C2 => {
                    let m220 = match v0.clone() {
                        Adt1::C0(_, _, _, _, h219, _) => {
                            h219
                        }
                    };
                    match m220 {
                        Adt7::C0 => {
                            Some(Adt13::C14(v2))
                        }
                        Adt7::C1 => {
                            if f56(match v0 {
                                Adt1::C0(_, _, _, _, _, h221) => {
                                    h221
                                }
                            }, v2.clone()) {
                                None::<Adt13>
                            } else {
                                Some(Adt13::C14(v2))
                            }
                        }
                        Adt7::C2 => {
                            if f56(match v0 {
                                Adt1::C0(_, _, _, _, _, h222) => {
                                    h222
                                }
                            }, v2.clone()) {
                                None::<Adt13>
                            } else {
                                Some(Adt13::C14(v2))
                            }
                        }
                    }
                }
                Adt6::C3 => {
                    Some(Adt13::C12(v2))
                }
            }
        } else {
            Some(Adt13::C13(v2))
        }
    }
}

pub fn f49(v0: Fn7, p226: Option<Adt13>, p227: List<Adt5>) -> Option<Adt13> {
    let m225 = (p226, p227);
    {
        let mut m225 = m225;
        loop {
            let step = {
                let (v1, v2) = m225;
                let m223 = v2;
                match m223.uncons() {
                    None => {
                        Err::<(Option<Adt13>, List<Adt5>), Option<Adt13>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Adt13>, List<Adt5>), Option<Adt13>>(({
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

pub fn f50(v0: Adt1, v1: Option<Adt13>, v2: Adt5) -> Option<Adt13> {
    let m228 = v1;
    match m228 {
        None => {
            let m230 = f41(match v0 {
                Adt1::C0(_, _, _, h229, _, _) => {
                    h229
                }
            }, v2.clone());
            match m230 {
                None => {
                    Some(Adt13::C15(v2))
                }
                Some(v4) => {
                    let m231 = v4;
                    match m231 {
                        Adt6::C0 => {
                            Some(Adt13::C15(v2))
                        }
                        Adt6::C1(_) => {
                            Some(Adt13::C15(v2))
                        }
                        Adt6::C2 => {
                            None::<Adt13>
                        }
                        Adt6::C3 => {
                            Some(Adt13::C15(v2))
                        }
                    }
                }
            }
        }
        Some(v3) => {
            Some(v3)
        }
    }
}

pub fn f51(v0: Adt0) -> Option<Adt13> {
    let m233 = match v0.clone() {
        Adt0::C0(_, _, h232, _, _, _, _) => {
            h232
        }
    };
    match m233 {
        Adt8::C0(_, v2, _) => {
            f49(Fn7::F57(v0), None::<Adt13>, v2)
        }
        Adt8::C1 => {
            None::<Adt13>
        }
        Adt8::C2 => {
            None::<Adt13>
        }
        Adt8::C3 => {
            None::<Adt13>
        }
        Adt8::C4 => {
            None::<Adt13>
        }
    }
}

pub fn f52(v0: Adt0) -> Option<Adt13> {
    let m234 = f58(v0.clone());
    match m234 {
        None => {
            f59(v0)
        }
        Some(v1) => {
            Some(v1)
        }
    }
}

pub fn f53(v0: Adt5) -> u64 {
    let m235 = v0;
    match m235 {
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
    }
}

pub fn f54(v0: Adt5) -> bool {
    let m236 = v0;
    match m236 {
        Adt5::C0 => {
            true
        }
        Adt5::C1 => {
            false
        }
        Adt5::C2 => {
            false
        }
        Adt5::C3 => {
            false
        }
        Adt5::C4 => {
            true
        }
        Adt5::C5 => {
            true
        }
        Adt5::C6 => {
            false
        }
        Adt5::C7 => {
            false
        }
        Adt5::C8 => {
            false
        }
        Adt5::C9 => {
            true
        }
    }
}

pub fn f55(v0: Adt5) -> bool {
    let m237 = v0;
    match m237 {
        Adt5::C0 => {
            false
        }
        Adt5::C1 => {
            true
        }
        Adt5::C2 => {
            true
        }
        Adt5::C3 => {
            true
        }
        Adt5::C4 => {
            false
        }
        Adt5::C5 => {
            false
        }
        Adt5::C6 => {
            false
        }
        Adt5::C7 => {
            false
        }
        Adt5::C8 => {
            false
        }
        Adt5::C9 => {
            false
        }
    }
}

pub fn f56(v0: List<Adt5>, v1: Adt5) -> bool {
    f60(Fn8::F61(v1), false, v0)
}

pub fn f57(v0: Adt0, v1: Option<Adt13>, v2: Adt5) -> Option<Adt13> {
    let m238 = v1;
    match m238 {
        None => {
            let m242 = f41({
                let m241 = match v0 {
                    Adt0::C0(_, h240, _, _, _, _, _) => {
                        h240
                    }
                };
                match m241 {
                    Adt1::C0(_, _, _, h239, _, _) => {
                        h239
                    }
                }
            }, v2.clone());
            match m242 {
                None => {
                    Some(Adt13::C11(v2))
                }
                Some(_) => {
                    None::<Adt13>
                }
            }
        }
        Some(v3) => {
            Some(v3)
        }
    }
}

pub fn f58(v0: Adt0) -> Option<Adt13> {
    let m244 = match v0.clone() {
        Adt0::C0(_, _, _, _, _, h243, _) => {
            h243
        }
    };
    match m244 {
        Adt11::C0 => {
            Some(Adt13::C20)
        }
        Adt11::C1 => {
            Some(Adt13::C20)
        }
        Adt11::C2 => {
            Some(Adt13::C20)
        }
        Adt11::C3 => {
            Some(Adt13::C20)
        }
        Adt11::C4 => {
            Some(Adt13::C20)
        }
        Adt11::C5 => {
            Some(Adt13::C20)
        }
        Adt11::C6 => {
            Some(Adt13::C20)
        }
        Adt11::C7 => {
            let m246 = match v0 {
                Adt0::C0(_, _, _, _, h245, _, _) => {
                    h245
                }
            };
            match m246 {
                Adt10::C0 => {
                    None::<Adt13>
                }
                Adt10::C1 => {
                    Some(Adt13::C17)
                }
            }
        }
        Adt11::C8 => {
            let m248 = match v0 {
                Adt0::C0(_, _, _, _, h247, _, _) => {
                    h247
                }
            };
            match m248 {
                Adt10::C0 => {
                    None::<Adt13>
                }
                Adt10::C1 => {
                    Some(Adt13::C17)
                }
            }
        }
        Adt11::C9 => {
            let m250 = match v0 {
                Adt0::C0(_, _, _, _, h249, _, _) => {
                    h249
                }
            };
            match m250 {
                Adt10::C0 => {
                    Some(Adt13::C18)
                }
                Adt10::C1 => {
                    None::<Adt13>
                }
            }
        }
        Adt11::C10 => {
            let m252 = match v0 {
                Adt0::C0(_, _, _, _, h251, _, _) => {
                    h251
                }
            };
            match m252 {
                Adt10::C0 => {
                    Some(Adt13::C18)
                }
                Adt10::C1 => {
                    None::<Adt13>
                }
            }
        }
        Adt11::C11 => {
            Some(Adt13::C20)
        }
        Adt11::C12 => {
            Some(Adt13::C20)
        }
        Adt11::C13 => {
            Some(Adt13::C20)
        }
        Adt11::C14 => {
            Some(Adt13::C20)
        }
        Adt11::C15 => {
            Some(Adt13::C20)
        }
        Adt11::C16 => {
            Some(Adt13::C20)
        }
        Adt11::C17 => {
            Some(Adt13::C20)
        }
        Adt11::C18 => {
            Some(Adt13::C20)
        }
        Adt11::C19 => {
            Some(Adt13::C20)
        }
        Adt11::C20 => {
            Some(Adt13::C20)
        }
        Adt11::C21 => {
            Some(Adt13::C20)
        }
        Adt11::C22 => {
            Some(Adt13::C20)
        }
        Adt11::C23 => {
            Some(Adt13::C20)
        }
        Adt11::C24 => {
            Some(Adt13::C20)
        }
        Adt11::C25 => {
            Some(Adt13::C16)
        }
        Adt11::C26 => {
            Some(Adt13::C20)
        }
        Adt11::C27 => {
            Some(Adt13::C20)
        }
        Adt11::C28 => {
            Some(Adt13::C20)
        }
        Adt11::C29 => {
            Some(Adt13::C20)
        }
        Adt11::C30 => {
            Some(Adt13::C20)
        }
    }
}

pub fn f59(v0: Adt0) -> Option<Adt13> {
    let m254 = match v0 {
        Adt0::C0(_, _, _, _, _, _, h253) => {
            h253
        }
    };
    match m254 {
        Adt12::C0 => {
            None::<Adt13>
        }
        Adt12::C1 => {
            Some(Adt13::C19)
        }
        Adt12::C2 => {
            Some(Adt13::C19)
        }
    }
}

pub fn f60(v0: Fn8, p258: bool, p259: List<Adt5>) -> bool {
    let m257 = (p258, p259);
    {
        let mut m257 = m257;
        loop {
            let step = {
                let (v1, v2) = m257;
                let m255 = v2;
                match m255.uncons() {
                    None => {
                        Err::<(bool, List<Adt5>), bool>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<Adt5>), bool>(({
                            let c256: Fn8 = v0.clone();
                            c256.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m257 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f61(v0: Adt5, v1: bool, v2: Adt5) -> bool {
    if v1 {
        true
    } else {
        f45(v2, v0)
    }
}

pub fn run(p0: Adt0) -> R<Result<Option<Bytes>, Bytes>> {
    f0(p0)
}
