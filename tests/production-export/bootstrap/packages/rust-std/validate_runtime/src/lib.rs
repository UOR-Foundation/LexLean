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
    C0(bool, List<Adt1>),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(List<Bytes>, List<Bytes>),
    C1(Bytes, List<Bytes>),
    C2(Bytes, List<Bytes>, Adt2),
    C3(Bytes, Adt2, Adt3),
    C4(Bytes, List<Adt5>),
    C5(List<Adt7>, List<Bytes>, Adt2, List<Adt8>),
    C6(Adt8),
    C7(Bytes, List<Bytes>, List<Bytes>, Adt2),
    C8(Bytes, List<Bytes>, List<Bytes>, List<Bytes>, Adt2),
    C9(Bytes, List<Bytes>, List<Bytes>, List<Adt9>),
    C10(Bytes),
    C11(Bytes, List<Adt1>),
    C12(Bytes, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt2 {
    C0,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6(u64),
    C7(std::rc::Rc<Adt2>),
    C8(std::rc::Rc<Adt2>),
    C9(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C10(Bytes, std::rc::Rc<Adt2>),
    C11,
    C12,
    C13,
    C14,
    C15(std::rc::Rc<Adt2>),
    C16,
    C17,
    C18,
    C19(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
}

#[derive(Clone)]
pub enum Adt3 {
    C0(Bytes),
    C1(u64),
    C2(List<Bytes>),
    C3(List<Bytes>, std::rc::Rc<Adt3>),
    C4(std::rc::Rc<Adt3>, Bytes, std::rc::Rc<Adt3>),
    C5(std::rc::Rc<Adt3>, u64),
    C6(std::rc::Rc<Adt3>, Bytes),
    C7(std::rc::Rc<Adt3>),
    C8(std::rc::Rc<Adt3>, std::rc::Rc<Adt3>),
    C9(std::rc::Rc<Adt3>, std::rc::Rc<Adt3>),
    C10(Adt4, Adt2, std::rc::Rc<Adt3>),
    C11(std::rc::Rc<Adt3>, std::rc::Rc<Adt3>),
    C12(std::rc::Rc<Adt3>, std::rc::Rc<Adt3>, std::rc::Rc<Adt3>),
    C13(std::rc::Rc<Adt3>, std::rc::Rc<Adt3>),
    C14(Adt4, std::rc::Rc<Adt3>, std::rc::Rc<Adt3>),
    C15(std::rc::Rc<Adt3>),
    C16(Adt4, std::rc::Rc<Adt3>, std::rc::Rc<Adt3>),
    C17,
    C18(std::rc::Rc<Adt3>),
    C19(std::rc::Rc<Adt3>),
    C20(List<Bytes>, List<Bytes>, std::rc::Rc<Adt3>),
    C21(u64, std::rc::Rc<Adt3>, std::rc::Rc<Adt3>),
    C22(std::rc::Rc<Adt3>),
    C23(std::rc::Rc<Adt3>, Adt2),
    C24(std::rc::Rc<Adt3>, std::rc::Rc<Adt3>),
    C25(std::rc::Rc<Adt3>, std::rc::Rc<Adt3>, std::rc::Rc<Adt3>),
    C26(std::rc::Rc<Adt3>),
    C27(std::rc::Rc<Adt3>),
    C28(std::rc::Rc<Adt3>, Bytes, Adt2, std::rc::Rc<Adt3>),
    C29(std::rc::Rc<Adt3>, std::rc::Rc<Adt3>),
    C30,
    C31,
    C32(std::rc::Rc<Adt3>, std::rc::Rc<Adt3>),
    C33(Adt4, Adt2, std::rc::Rc<Adt3>, std::rc::Rc<Adt3>),
    C34(Adt4, std::rc::Rc<Adt3>, std::rc::Rc<Adt3>, std::rc::Rc<Adt3>),
}

#[derive(Clone)]
pub enum Adt4 {
    C0,
    C1(Bytes),
    C2(Bytes),
    C3(List<Bytes>),
    C4(std::rc::Rc<Adt4>),
    C5(List<Bytes>, std::rc::Rc<Adt4>),
    C6(List<Bytes>, List<Bytes>, std::rc::Rc<Adt4>),
    C7(u64),
    C8,
    C9(std::rc::Rc<Adt4>, std::rc::Rc<Adt4>),
}

#[derive(Clone)]
pub enum Adt5 {
    C0(Bytes, bool, bool, List<Adt6>, Adt2),
}

#[derive(Clone)]
pub enum Adt6 {
    C0(Bytes, bool, Adt2),
}

#[derive(Clone)]
pub enum Adt7 {
    C0(Bytes, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt8 {
    C0(Adt5, List<Adt7>, bool, Adt3),
}

#[derive(Clone)]
pub enum Adt9 {
    C0(Bytes),
    C1(Bytes, Adt2),
    C2(Bytes, List<Bytes>, Adt2),
}

#[derive(Clone)]
pub enum Adt10 {
    C0(Bytes, List<Adt5>),
}

#[derive(Clone)]
pub enum Adt11 {
    C0(Bytes, List<Bytes>, u64),
}

#[derive(Clone)]
pub enum Adt12 {
    C0(Bytes, List<Adt9>, u64),
}

#[derive(Clone)]
pub enum Adt13 {
    C0(Bytes, u64),
}

#[derive(Clone)]
pub enum Adt14 {
    C0(bool, List<Bytes>, List<Adt10>, List<Adt11>, List<Adt12>, List<Adt13>, List<Bytes>, List<Adt13>, List<Adt13>, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt15 {
    C0(bool, Option<Bytes>),
}

#[derive(Clone)]
pub enum Adt16 {
    C0(List<Adt7>, List<Adt17>, Adt2, bool),
}

#[derive(Clone)]
pub enum Adt17 {
    C0(Bytes, bool, Adt2),
}

#[derive(Clone)]
pub enum Adt18 {
    C0(List<Bytes>, Option<Bytes>),
}

#[derive(Clone)]
pub enum Fn0 {
    F4(Adt14),
    F29(Bytes, Adt14),
}

impl Fn0 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Adt1) -> R<Option<Bytes>> {
        match self {
            Fn0::F4(k0) => f4(k0.clone(), p0, p1),
            Fn0::F29(k0, k1) => f29(k0.clone(), k1.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F6,
}

impl Fn1 {
    pub fn apply(&self, p0: Adt15, p1: Adt1) -> R<Adt15> {
        match self {
            Fn1::F6 => f6(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F8,
}

impl Fn2 {
    pub fn apply(&self, p0: Result<Adt14, Bytes>, p1: Adt1) -> R<Result<Adt14, Bytes>> {
        match self {
            Fn2::F8 => f8(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F20(Adt14),
    F82(List<Adt8>, Bytes),
}

impl Fn3 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Adt5) -> R<Option<Bytes>> {
        match self {
            Fn3::F20(k0) => f20(k0.clone(), p0, p1),
            Fn3::F82(k0, k1) => f82(k0.clone(), k1.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F24(List<Adt7>, Adt14, Adt2),
    F83(List<Adt7>, Adt14, List<Adt5>, Bytes, Adt2),
}

impl Fn4 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Adt8) -> R<Option<Bytes>> {
        match self {
            Fn4::F24(k0, k1, k2) => f24(k0.clone(), k1.clone(), k2.clone(), p0, p1),
            Fn4::F83(k0, k1, k2, k3, k4) => f83(k0.clone(), k1.clone(), k2.clone(), k3.clone(), k4.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F27(List<Bytes>, Adt14),
}

impl Fn5 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Adt9) -> R<Option<Bytes>> {
        match self {
            Fn5::F27(k0, k1) => f27(k0.clone(), k1.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F77,
}

impl Fn6 {
    pub fn apply(&self, p0: Adt18, p1: Adt5) -> R<Adt18> {
        match self {
            Fn6::F77 => f77(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F80(Adt14),
}

impl Fn7 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Adt7) -> R<Option<Bytes>> {
        match self {
            Fn7::F80(k0) => f80(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F87,
    F114(Bytes),
    F131(List<Bytes>, List<Bytes>),
    F210(Bytes, Adt14),
}

impl Fn8 {
    pub fn apply(&self, p0: bool, p1: Bytes) -> R<bool> {
        match self {
            Fn8::F87 => Ok(f87(p0, p1)),
            Fn8::F114(k0) => Ok(f114(k0.clone(), p0, p1)),
            Fn8::F131(k0, k1) => f131(k0.clone(), k1.clone(), p0, p1),
            Fn8::F210(k0, k1) => Ok(f210(k0.clone(), k1.clone(), p0, p1)),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F89,
}

impl Fn9 {
    pub fn apply(&self, p0: Bytes, p1: Bytes) -> Bytes {
        match self {
            Fn9::F89 => f89(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F92(Str),
    F93(Str),
    F96(Str),
    F133(Adt7, Adt14),
    F142(Bytes),
    F181(List<Bytes>, List<Bytes>),
}

impl Fn10 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Bytes) -> R<Option<Bytes>> {
        match self {
            Fn10::F92(k0) => f92(k0.clone(), p0, p1),
            Fn10::F93(k0) => f93(k0.clone(), p0, p1),
            Fn10::F96(k0) => Ok(f96(k0.clone(), p0, p1)),
            Fn10::F133(k0, k1) => Ok(f133(k0.clone(), k1.clone(), p0, p1)),
            Fn10::F142(k0) => Ok(f142(k0.clone(), p0, p1)),
            Fn10::F181(k0, k1) => f181(k0.clone(), k1.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F98,
}

impl Fn11 {
    pub fn apply(&self, p0: Adt14, p1: Adt5) -> Adt14 {
        match self {
            Fn11::F98 => f98(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F102,
    F145(Bytes),
}

impl Fn12 {
    pub fn apply(&self, p0: Adt14, p1: Adt8) -> Adt14 {
        match self {
            Fn12::F102 => f102(p0, p1),
            Fn12::F145(k0) => f145(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F105(Bytes),
    F106,
}

impl Fn13 {
    pub fn apply(&self, p0: Adt18, p1: Adt9) -> R<Adt18> {
        match self {
            Fn13::F105(k0) => f105(k0.clone(), p0, p1),
            Fn13::F106 => f106(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F113(Bytes),
}

impl Fn14 {
    pub fn apply(&self, p0: Option<Adt17>, p1: Adt17) -> Option<Adt17> {
        match self {
            Fn14::F113(k0) => f113(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F144(Bytes),
    F199(Bytes),
}

impl Fn15 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn15::F144(k0) => f144(k0.clone(), p0),
            Fn15::F199(k0) => f199(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F151(Bytes),
}

impl Fn16 {
    pub fn apply(&self, p0: Option<Adt11>, p1: Adt11) -> Option<Adt11> {
        match self {
            Fn16::F151(k0) => f151(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F153(Bytes),
}

impl Fn17 {
    pub fn apply(&self, p0: Option<Adt12>, p1: Adt12) -> Option<Adt12> {
        match self {
            Fn17::F153(k0) => f153(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F155(Bytes),
    F205(Bytes),
}

impl Fn18 {
    pub fn apply(&self, p0: Option<Adt13>, p1: Adt13) -> Option<Adt13> {
        match self {
            Fn18::F155(k0) => f155(k0.clone(), p0, p1),
            Fn18::F205(k0) => f205(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F168(Bytes),
}

impl Fn19 {
    pub fn apply(&self, p0: List<u64>, p1: Adt13) -> List<u64> {
        match self {
            Fn19::F168(k0) => f168(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn20 {
    F170(u64),
}

impl Fn20 {
    pub fn apply(&self, p0: bool, p1: u64) -> bool {
        match self {
            Fn20::F170(k0) => f170(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn21 {
    F176,
}

impl Fn21 {
    pub fn apply(&self, p0: Bytes, p1: u64) -> R<Bytes> {
        match self {
            Fn21::F176 => f176(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn22 {
    F184(bool, u64, Adt14),
}

impl Fn22 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Adt6) -> R<Option<Bytes>> {
        match self {
            Fn22::F184(k0, k1, k2) => f184(*k0, *k1, k2.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn23 {
    F186(Bytes),
}

impl Fn23 {
    pub fn apply(&self, p0: Option<Adt10>, p1: Adt10) -> Option<Adt10> {
        match self {
            Fn23::F186(k0) => f186(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn24 {
    F188(Bytes),
}

impl Fn24 {
    pub fn apply(&self, p0: u64, p1: Adt8) -> R<u64> {
        match self {
            Fn24::F188(k0) => f188(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn25 {
    F190(Bytes),
}

impl Fn25 {
    pub fn apply(&self, p0: Option<Adt8>, p1: Adt8) -> Option<Adt8> {
        match self {
            Fn25::F190(k0) => f190(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn26 {
    F192,
}

impl Fn26 {
    pub fn apply(&self, p0: Bytes, p1: Adt6) -> R<Bytes> {
        match self {
            Fn26::F192 => f192(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn27 {
    F195(Bytes),
    F217(Bytes),
}

impl Fn27 {
    pub fn apply(&self, p0: bool, p1: Adt5) -> bool {
        match self {
            Fn27::F195(k0) => f195(k0.clone(), p0, p1),
            Fn27::F217(k0) => f217(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn28 {
    F197(Bytes),
}

impl Fn28 {
    pub fn apply(&self, p0: Result<List<Adt17>, Bytes>, p1: Adt6) -> Result<List<Adt17>, Bytes> {
        match self {
            Fn28::F197(k0) => f197(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn29 {
    F202(Bytes),
}

impl Fn29 {
    pub fn apply(&self, p0: Option<Adt9>, p1: Adt9) -> Option<Adt9> {
        match self {
            Fn29::F202(k0) => f202(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn30 {
    F213(Bytes),
}

impl Fn30 {
    pub fn apply(&self, p0: u64, p1: Bytes) -> R<u64> {
        match self {
            Fn30::F213(k0) => f213(k0.clone(), p0, p1),
        }
    }
}

pub fn f0(v0: Adt0) -> R<Result<(), Bytes>> {
    let m2 = f1(match v0.clone() {
        Adt0::C0(_, h1) => {
            h1
        }
    })?;
    match m2 {
        None => {
            let m5 = f2(Adt14::C0(match v0.clone() {
                Adt0::C0(h3, _) => {
                    h3
                }
            }, List::<Bytes>::nil(), List::<Adt10>::nil(), List::<Adt11>::nil(), List::<Adt12>::nil(), List::<Adt13>::nil(), List::<Bytes>::nil(), List::<Adt13>::nil(), List::<Adt13>::nil(), List::<Bytes>::nil()), match v0.clone() {
                Adt0::C0(_, h4) => {
                    h4
                }
            })?;
            match m5 {
                Ok(v3) => {
                    let m7 = f3(Fn0::F4(v3), None::<Bytes>, match v0 {
                        Adt0::C0(_, h6) => {
                            h6
                        }
                    })?;
                    match m7 {
                        None => {
                            Ok(Ok::<(), Bytes>(()))
                        }
                        Some(v4) => {
                            Ok(Err::<(), Bytes>(v4))
                        }
                    }
                }
                Err(v2) => {
                    Ok(Err::<(), Bytes>(v2))
                }
            }
        }
        Some(v1) => {
            Ok(Err::<(), Bytes>(v1))
        }
    }
}

pub fn f1(v0: List<Adt1>) -> R<Option<Bytes>> {
    let m9 = f5(Fn1::F6, Adt15::C0(false, None::<Bytes>), v0)?;
    match m9 {
        Adt15::C0(_, h8) => {
            Ok(h8)
        }
    }
}

pub fn f2(v0: Adt14, v1: List<Adt1>) -> R<Result<Adt14, Bytes>> {
    f7(Fn2::F8, Ok::<Adt14, Bytes>(v0), v1)
}

pub fn f3(v0: Fn0, p13: Option<Bytes>, p14: List<Adt1>) -> R<Option<Bytes>> {
    let m12 = (p13, p14);
    {
        let mut m12 = m12;
        loop {
            let step = {
                let (v1, v2) = m12;
                let m10 = v2;
                match m10.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Adt1>), R<Option<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Adt1>), R<Option<Bytes>>>(({
                            let c11: Fn0 = v0.clone();
                            c11.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m12 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f4(v0: Adt14, v1: Option<Bytes>, v2: Adt1) -> R<Option<Bytes>> {
    let m15 = v1;
    match m15 {
        None => {
            f9(v0, v2)
        }
        Some(v3) => {
            Ok(Some(v3))
        }
    }
}

pub fn f5(v0: Fn1, p19: Adt15, p20: List<Adt1>) -> R<Adt15> {
    let m18 = (p19, p20);
    {
        let mut m18 = m18;
        loop {
            let step = {
                let (v1, v2) = m18;
                let m16 = v2;
                match m16.uncons() {
                    None => {
                        Err::<(Adt15, List<Adt1>), R<Adt15>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt15, List<Adt1>), R<Adt15>>(({
                            let c17: Fn1 = v0.clone();
                            c17.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m18 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f6(v0: Adt15, v1: Adt1) -> R<Adt15> {
    let m22 = match v0.clone() {
        Adt15::C0(_, h21) => {
            h21
        }
    };
    match m22 {
        None => {
            f10(v0, v1)
        }
        Some(_) => {
            Ok(v0)
        }
    }
}

pub fn f7(v0: Fn2, p26: Result<Adt14, Bytes>, p27: List<Adt1>) -> R<Result<Adt14, Bytes>> {
    let m25 = (p26, p27);
    {
        let mut m25 = m25;
        loop {
            let step = {
                let (v1, v2) = m25;
                let m23 = v2;
                match m23.uncons() {
                    None => {
                        Err::<(Result<Adt14, Bytes>, List<Adt1>), R<Result<Adt14, Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Result<Adt14, Bytes>, List<Adt1>), R<Result<Adt14, Bytes>>>(({
                            let c24: Fn2 = v0.clone();
                            c24.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m25 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f8(v0: Result<Adt14, Bytes>, v1: Adt1) -> R<Result<Adt14, Bytes>> {
    let m28 = v0;
    match m28 {
        Ok(v3) => {
            f11(v3, v1)
        }
        Err(v2) => {
            Ok(Err::<Adt14, Bytes>(v2))
        }
    }
}

pub fn f9(v0: Adt14, v1: Adt1) -> R<Option<Bytes>> {
    let m29 = v1;
    match m29 {
        Adt1::C0(_, _) => {
            Ok(None::<Bytes>)
        }
        Adt1::C1(_, _) => {
            Ok(None::<Bytes>)
        }
        Adt1::C2(_, v7, v8) => {
            f12(v0, v8, {
                let a30 = v7;
                length_list(a30)
            }, false)
        }
        Adt1::C3(_, v10, v11) => {
            let m31 = f12(v0.clone(), v10, 0u64, false)?;
            match m31 {
                None => {
                    f13(v0, f14(()), v11)
                }
                Some(v12) => {
                    Ok(Some(v12))
                }
            }
        }
        Adt1::C4(v13, v14) => {
            let m33 = {
                let m32 = f15(v14.clone())?;
                match m32 {
                    None => {
                        None::<Bytes>
                    }
                    Some(v15) => {
                        Some(f16(f17(Str::lit("runtime support: trait-method: ")), f16(f18(v13), f16(f17(Str::lit(" declares ")), f16(f18(v15), f17(Str::lit(" twice")))))))
                    }
                }
            };
            match m33 {
                None => {
                    f19(Fn3::F20(v0), None::<Bytes>, v14)
                }
                Some(v16) => {
                    Ok(Some(v16))
                }
            }
        }
        Adt1::C5(v17, v18, v19, v20) => {
            let m34 = f21(v0.clone(), v17.clone())?;
            match m34 {
                None => {
                    let m36 = f12(v0.clone(), v19.clone(), {
                        let a35 = v17.clone();
                        length_list(a35)
                    }, false)?;
                    match m36 {
                        None => {
                            let m37 = v18;
                            match m37.uncons() {
                                None => {
                                    f23(Fn4::F24(v17, v0, v19), None::<Bytes>, v20)
                                }
                                Some((v23, _)) => {
                                    f22(v0, v23, v17, v19, v20)
                                }
                            }
                        }
                        Some(v22) => {
                            Ok(Some(v22))
                        }
                    }
                }
                Some(v21) => {
                    Ok(Some(v21))
                }
            }
        }
        Adt1::C6(v25) => {
            f25(v0, v25)
        }
        Adt1::C7(_, v27, _, v29) => {
            f12(v0, v29, {
                let a38 = v27;
                length_list(a38)
            }, false)
        }
        Adt1::C8(_, v31, _, _, v34) => {
            f12(v0, v34, {
                let a39 = v31;
                length_list(a39)
            }, false)
        }
        Adt1::C9(_, v36, _, v38) => {
            f26(Fn5::F27(v36, v0), None::<Bytes>, v38)
        }
        Adt1::C10(v39) => {
            Ok(f28(v39))
        }
        Adt1::C11(v40, v41) => {
            f3(Fn0::F29(v40, v0), None::<Bytes>, v41)
        }
        Adt1::C12(_, _) => {
            Ok(None::<Bytes>)
        }
    }
}

pub fn f10(v0: Adt15, v1: Adt1) -> R<Adt15> {
    let m40 = v1;
    match m40 {
        Adt1::C0(_, _) => {
            Ok(Adt15::C0(true, None::<Bytes>))
        }
        Adt1::C1(_, _) => {
            Ok(Adt15::C0(true, None::<Bytes>))
        }
        Adt1::C2(_, _, _) => {
            Ok(Adt15::C0(true, None::<Bytes>))
        }
        Adt1::C3(_, _, _) => {
            Ok(Adt15::C0(true, None::<Bytes>))
        }
        Adt1::C4(_, _) => {
            Ok(Adt15::C0(true, None::<Bytes>))
        }
        Adt1::C5(_, _, _, _) => {
            Ok(Adt15::C0(true, None::<Bytes>))
        }
        Adt1::C6(_) => {
            Ok(Adt15::C0(true, None::<Bytes>))
        }
        Adt1::C7(_, _, _, _) => {
            Ok(Adt15::C0(true, None::<Bytes>))
        }
        Adt1::C8(_, _, _, _, _) => {
            Ok(Adt15::C0(true, None::<Bytes>))
        }
        Adt1::C9(_, _, _, _) => {
            Ok(Adt15::C0(true, None::<Bytes>))
        }
        Adt1::C10(_) => {
            Ok(Adt15::C0(true, None::<Bytes>))
        }
        Adt1::C11(_, _) => {
            Ok(Adt15::C0(true, None::<Bytes>))
        }
        Adt1::C12(v35, v36) => {
            if match v0 {
                Adt15::C0(h41, _) => {
                    h41
                }
            } {
                Ok(Adt15::C0(true, Some(f16(f17(Str::lit("runtime support: attribute-position: `#![")), f16(v35, f17(Str::lit("]` follows an item")))))))
            } else {
                Ok(Adt15::C0(false, f30(v35, v36)?))
            }
        }
    }
}

pub fn f11(v0: Adt14, v1: Adt1) -> R<Result<Adt14, Bytes>> {
    let m42 = v1;
    match m42 {
        Adt1::C0(v2, v3) => {
            Ok(f31({
                let m43 = v2.clone();
                match m43.uncons() {
                    None => {
                        Err::<Adt14, Bytes>(f17(Str::lit("runtime support: use-path: empty")))
                    }
                    Some((v4, _)) => {
                        if if f32(v4.clone(), f17(Str::lit("core"))) {
                            true
                        } else {
                            if match v0.clone() {
                                Adt14::C0(h44, _, _, _, _, _, _, _, _, _) => {
                                    h44
                                }
                            } {
                                false
                            } else {
                                f32(v4.clone(), f17(Str::lit("std")))
                            }
                        } {
                            Ok::<Adt14, Bytes>(Adt14::C0(match v0.clone() {
                                Adt14::C0(h45, _, _, _, _, _, _, _, _, _) => {
                                    h45
                                }
                            }, match v0.clone() {
                                Adt14::C0(_, h46, _, _, _, _, _, _, _, _) => {
                                    h46
                                }
                            }, match v0.clone() {
                                Adt14::C0(_, _, h47, _, _, _, _, _, _, _) => {
                                    h47
                                }
                            }, match v0.clone() {
                                Adt14::C0(_, _, _, h48, _, _, _, _, _, _) => {
                                    h48
                                }
                            }, match v0.clone() {
                                Adt14::C0(_, _, _, _, h49, _, _, _, _, _) => {
                                    h49
                                }
                            }, match v0.clone() {
                                Adt14::C0(_, _, _, _, _, h50, _, _, _, _) => {
                                    h50
                                }
                            }, match v0.clone() {
                                Adt14::C0(_, _, _, _, _, _, h51, _, _, _) => {
                                    h51
                                }
                            }, match v0.clone() {
                                Adt14::C0(_, _, _, _, _, _, _, h52, _, _) => {
                                    h52
                                }
                            }, match v0.clone() {
                                Adt14::C0(_, _, _, _, _, _, _, _, h53, _) => {
                                    h53
                                }
                            }, {
                                let a56 = match v0 {
                                    Adt14::C0(_, _, _, _, _, _, _, _, _, h54) => {
                                        h54
                                    }
                                };
                                let a57 = List::cons({
                                    let m55 = v3.clone();
                                    match m55.uncons() {
                                        None => {
                                            f33(v2.clone())
                                        }
                                        Some((v6, _)) => {
                                            v6
                                        }
                                    }
                                }, List::<Bytes>::nil());
                                append_list(a56, a57)
                            }))
                        } else {
                            Err::<Adt14, Bytes>(f16(f17(Str::lit("runtime support: profile-root: ")), f16(f18(f16(f17(Str::lit("use ")), f34(v2.clone(), Str::lit("::"))?)), f16(f17(Str::lit(" starts at ")), f16(f18(v4), f17(Str::lit(", which the profile does not carry")))))))
                        }
                    }
                }
            }, f35(f36(Str::lit("a `use` segment"), v2)?, f37(Str::lit("a `use` alias"), v3)?)))
        }
        Adt1::C1(v8, v9) => {
            Ok(f38(f39(Str::lit("a unit struct name"), v8.clone())?, f31({
                let m58 = f40(v0, v8.clone())?;
                match m58 {
                    Ok(v11) => {
                        Ok::<Adt14, Bytes>(Adt14::C0(match v11.clone() {
                            Adt14::C0(h59, _, _, _, _, _, _, _, _, _) => {
                                h59
                            }
                        }, match v11.clone() {
                            Adt14::C0(_, h60, _, _, _, _, _, _, _, _) => {
                                h60
                            }
                        }, match v11.clone() {
                            Adt14::C0(_, _, h61, _, _, _, _, _, _, _) => {
                                h61
                            }
                        }, {
                            let a63 = match v11.clone() {
                                Adt14::C0(_, _, _, h62, _, _, _, _, _, _) => {
                                    h62
                                }
                            };
                            let a64 = List::cons(Adt11::C0(v8, List::<Bytes>::nil(), 0u64), List::<Adt11>::nil());
                            append_list(a63, a64)
                        }, match v11.clone() {
                            Adt14::C0(_, _, _, _, h65, _, _, _, _, _) => {
                                h65
                            }
                        }, match v11.clone() {
                            Adt14::C0(_, _, _, _, _, h66, _, _, _, _) => {
                                h66
                            }
                        }, match v11.clone() {
                            Adt14::C0(_, _, _, _, _, _, h67, _, _, _) => {
                                h67
                            }
                        }, match v11.clone() {
                            Adt14::C0(_, _, _, _, _, _, _, h68, _, _) => {
                                h68
                            }
                        }, match v11.clone() {
                            Adt14::C0(_, _, _, _, _, _, _, _, h69, _) => {
                                h69
                            }
                        }, match v11 {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, h70) => {
                                h70
                            }
                        }))
                    }
                    Err(v10) => {
                        Err::<Adt14, Bytes>(v10)
                    }
                }
            }, f41(Str::lit("a unit struct"), v9)?)))
        }
        Adt1::C2(v12, v13, _) => {
            Ok(f38(f39(Str::lit("an alias name"), v12.clone())?, f31({
                let m71 = f40(v0, v12.clone())?;
                match m71 {
                    Ok(v16) => {
                        Ok::<Adt14, Bytes>(Adt14::C0(match v16.clone() {
                            Adt14::C0(h72, _, _, _, _, _, _, _, _, _) => {
                                h72
                            }
                        }, match v16.clone() {
                            Adt14::C0(_, h73, _, _, _, _, _, _, _, _) => {
                                h73
                            }
                        }, match v16.clone() {
                            Adt14::C0(_, _, h74, _, _, _, _, _, _, _) => {
                                h74
                            }
                        }, match v16.clone() {
                            Adt14::C0(_, _, _, h75, _, _, _, _, _, _) => {
                                h75
                            }
                        }, match v16.clone() {
                            Adt14::C0(_, _, _, _, h76, _, _, _, _, _) => {
                                h76
                            }
                        }, {
                            let a79 = match v16.clone() {
                                Adt14::C0(_, _, _, _, _, h77, _, _, _, _) => {
                                    h77
                                }
                            };
                            let a80 = List::cons(Adt13::C0(v12, {
                                let a78 = v13.clone();
                                length_list(a78)
                            }), List::<Adt13>::nil());
                            append_list(a79, a80)
                        }, match v16.clone() {
                            Adt14::C0(_, _, _, _, _, _, h81, _, _, _) => {
                                h81
                            }
                        }, match v16.clone() {
                            Adt14::C0(_, _, _, _, _, _, _, h82, _, _) => {
                                h82
                            }
                        }, match v16.clone() {
                            Adt14::C0(_, _, _, _, _, _, _, _, h83, _) => {
                                h83
                            }
                        }, match v16 {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, h84) => {
                                h84
                            }
                        }))
                    }
                    Err(v15) => {
                        Err::<Adt14, Bytes>(v15)
                    }
                }
            }, f37(Str::lit("an alias parameter"), v13)?)))
        }
        Adt1::C3(v17, _, _) => {
            Ok(f38(f39(Str::lit("a static name"), v17.clone())?, {
                let m85 = f40(v0, v17.clone())?;
                match m85 {
                    Ok(v21) => {
                        Ok::<Adt14, Bytes>(Adt14::C0(match v21.clone() {
                            Adt14::C0(h86, _, _, _, _, _, _, _, _, _) => {
                                h86
                            }
                        }, match v21.clone() {
                            Adt14::C0(_, h87, _, _, _, _, _, _, _, _) => {
                                h87
                            }
                        }, match v21.clone() {
                            Adt14::C0(_, _, h88, _, _, _, _, _, _, _) => {
                                h88
                            }
                        }, match v21.clone() {
                            Adt14::C0(_, _, _, h89, _, _, _, _, _, _) => {
                                h89
                            }
                        }, match v21.clone() {
                            Adt14::C0(_, _, _, _, h90, _, _, _, _, _) => {
                                h90
                            }
                        }, match v21.clone() {
                            Adt14::C0(_, _, _, _, _, h91, _, _, _, _) => {
                                h91
                            }
                        }, {
                            let a93 = match v21.clone() {
                                Adt14::C0(_, _, _, _, _, _, h92, _, _, _) => {
                                    h92
                                }
                            };
                            let a94 = List::cons(v17, List::<Bytes>::nil());
                            append_list(a93, a94)
                        }, match v21.clone() {
                            Adt14::C0(_, _, _, _, _, _, _, h95, _, _) => {
                                h95
                            }
                        }, match v21.clone() {
                            Adt14::C0(_, _, _, _, _, _, _, _, h96, _) => {
                                h96
                            }
                        }, match v21 {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, h97) => {
                                h97
                            }
                        }))
                    }
                    Err(v20) => {
                        Err::<Adt14, Bytes>(v20)
                    }
                }
            }))
        }
        Adt1::C4(v22, v23) => {
            Ok(f38(f39(Str::lit("a trait name"), v22.clone())?, {
                let m98 = f40(v0, v22.clone())?;
                match m98 {
                    Ok(v25) => {
                        Ok::<Adt14, Bytes>(f42(Adt14::C0(match v25.clone() {
                            Adt14::C0(h99, _, _, _, _, _, _, _, _, _) => {
                                h99
                            }
                        }, match v25.clone() {
                            Adt14::C0(_, h100, _, _, _, _, _, _, _, _) => {
                                h100
                            }
                        }, {
                            let a102 = match v25.clone() {
                                Adt14::C0(_, _, h101, _, _, _, _, _, _, _) => {
                                    h101
                                }
                            };
                            let a103 = List::cons(Adt10::C0(v22, v23.clone()), List::<Adt10>::nil());
                            append_list(a102, a103)
                        }, match v25.clone() {
                            Adt14::C0(_, _, _, h104, _, _, _, _, _, _) => {
                                h104
                            }
                        }, match v25.clone() {
                            Adt14::C0(_, _, _, _, h105, _, _, _, _, _) => {
                                h105
                            }
                        }, match v25.clone() {
                            Adt14::C0(_, _, _, _, _, h106, _, _, _, _) => {
                                h106
                            }
                        }, match v25.clone() {
                            Adt14::C0(_, _, _, _, _, _, h107, _, _, _) => {
                                h107
                            }
                        }, match v25.clone() {
                            Adt14::C0(_, _, _, _, _, _, _, h108, _, _) => {
                                h108
                            }
                        }, match v25.clone() {
                            Adt14::C0(_, _, _, _, _, _, _, _, h109, _) => {
                                h109
                            }
                        }, match v25 {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, h110) => {
                                h110
                            }
                        }), v23))
                    }
                    Err(v24) => {
                        Err::<Adt14, Bytes>(v24)
                    }
                }
            }))
        }
        Adt1::C5(_, _, v28, v29) => {
            Ok(Ok::<Adt14, Bytes>(f43(f44(v0, v29.clone()), v28, v29)))
        }
        Adt1::C6(v30) => {
            Ok(f38(f39(Str::lit("a function name"), {
                let m113 = match v30.clone() {
                    Adt8::C0(h112, _, _, _) => {
                        h112
                    }
                };
                match m113 {
                    Adt5::C0(h111, _, _, _, _) => {
                        h111
                    }
                }
            })?, {
                let m117 = f40(v0, {
                    let m116 = match v30.clone() {
                        Adt8::C0(h115, _, _, _) => {
                            h115
                        }
                    };
                    match m116 {
                        Adt5::C0(h114, _, _, _, _) => {
                            h114
                        }
                    }
                })?;
                match m117 {
                    Ok(v32) => {
                        Ok::<Adt14, Bytes>(Adt14::C0(match v32.clone() {
                            Adt14::C0(h118, _, _, _, _, _, _, _, _, _) => {
                                h118
                            }
                        }, match v32.clone() {
                            Adt14::C0(_, h119, _, _, _, _, _, _, _, _) => {
                                h119
                            }
                        }, match v32.clone() {
                            Adt14::C0(_, _, h120, _, _, _, _, _, _, _) => {
                                h120
                            }
                        }, match v32.clone() {
                            Adt14::C0(_, _, _, h121, _, _, _, _, _, _) => {
                                h121
                            }
                        }, match v32.clone() {
                            Adt14::C0(_, _, _, _, h122, _, _, _, _, _) => {
                                h122
                            }
                        }, match v32.clone() {
                            Adt14::C0(_, _, _, _, _, h123, _, _, _, _) => {
                                h123
                            }
                        }, match v32.clone() {
                            Adt14::C0(_, _, _, _, _, _, h124, _, _, _) => {
                                h124
                            }
                        }, {
                            let a133 = match v32.clone() {
                                Adt14::C0(_, _, _, _, _, _, _, h125, _, _) => {
                                    h125
                                }
                            };
                            let a134 = List::cons(Adt13::C0({
                                let m128 = match v30.clone() {
                                    Adt8::C0(h127, _, _, _) => {
                                        h127
                                    }
                                };
                                match m128 {
                                    Adt5::C0(h126, _, _, _, _) => {
                                        h126
                                    }
                                }
                            }, {
                                let a132 = {
                                    let m131 = match v30 {
                                        Adt8::C0(h130, _, _, _) => {
                                            h130
                                        }
                                    };
                                    match m131 {
                                        Adt5::C0(_, _, _, h129, _) => {
                                            h129
                                        }
                                    }
                                };
                                length_list(a132)
                            }), List::<Adt13>::nil());
                            append_list(a133, a134)
                        }, match v32.clone() {
                            Adt14::C0(_, _, _, _, _, _, _, _, h135, _) => {
                                h135
                            }
                        }, match v32 {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, h136) => {
                                h136
                            }
                        }))
                    }
                    Err(v31) => {
                        Err::<Adt14, Bytes>(v31)
                    }
                }
            }))
        }
        Adt1::C7(v33, v34, v35, v36) => {
            Ok(f38(f39(Str::lit("a tuple struct name"), v33.clone())?, f31({
                let m137 = f40(v0, v33.clone())?;
                match m137 {
                    Ok(v38) => {
                        Ok::<Adt14, Bytes>(Adt14::C0(match v38.clone() {
                            Adt14::C0(h138, _, _, _, _, _, _, _, _, _) => {
                                h138
                            }
                        }, match v38.clone() {
                            Adt14::C0(_, h139, _, _, _, _, _, _, _, _) => {
                                h139
                            }
                        }, match v38.clone() {
                            Adt14::C0(_, _, h140, _, _, _, _, _, _, _) => {
                                h140
                            }
                        }, {
                            let a143 = match v38.clone() {
                                Adt14::C0(_, _, _, h141, _, _, _, _, _, _) => {
                                    h141
                                }
                            };
                            let a144 = List::cons(Adt11::C0(v33, f45(v36, 0u64)?, {
                                let a142 = v34.clone();
                                length_list(a142)
                            }), List::<Adt11>::nil());
                            append_list(a143, a144)
                        }, match v38.clone() {
                            Adt14::C0(_, _, _, _, h145, _, _, _, _, _) => {
                                h145
                            }
                        }, match v38.clone() {
                            Adt14::C0(_, _, _, _, _, h146, _, _, _, _) => {
                                h146
                            }
                        }, match v38.clone() {
                            Adt14::C0(_, _, _, _, _, _, h147, _, _, _) => {
                                h147
                            }
                        }, match v38.clone() {
                            Adt14::C0(_, _, _, _, _, _, _, h148, _, _) => {
                                h148
                            }
                        }, match v38.clone() {
                            Adt14::C0(_, _, _, _, _, _, _, _, h149, _) => {
                                h149
                            }
                        }, match v38 {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, h150) => {
                                h150
                            }
                        }))
                    }
                    Err(v37) => {
                        Err::<Adt14, Bytes>(v37)
                    }
                }
            }, f35(f37(Str::lit("a tuple struct generic"), v34)?, f41(Str::lit("a tuple struct"), v35)?))))
        }
        Adt1::C8(v39, v40, v41, v42, v43) => {
            Ok(f38(f39(Str::lit("a record struct name"), v39.clone())?, {
                let m151 = f40(v0, v39.clone())?;
                match m151 {
                    Ok(v45) => {
                        let m152 = f35(f37(Str::lit("a record struct generic"), v40.clone())?, f35(f41(Str::lit("a record struct"), v41)?, f37(Str::lit("a record field name"), v42.clone())?));
                        match m152 {
                            None => {
                                let a154 = {
                                    let a153 = v42.clone();
                                    length_list(a153)
                                };
                                let a155 = f46(v43.clone())?;
                                if nat_eq(a154, a155) {
                                    Ok::<Adt14, Bytes>(Adt14::C0(match v45.clone() {
                                        Adt14::C0(h156, _, _, _, _, _, _, _, _, _) => {
                                            h156
                                        }
                                    }, match v45.clone() {
                                        Adt14::C0(_, h157, _, _, _, _, _, _, _, _) => {
                                            h157
                                        }
                                    }, match v45.clone() {
                                        Adt14::C0(_, _, h158, _, _, _, _, _, _, _) => {
                                            h158
                                        }
                                    }, {
                                        let a161 = match v45.clone() {
                                            Adt14::C0(_, _, _, h159, _, _, _, _, _, _) => {
                                                h159
                                            }
                                        };
                                        let a162 = List::cons(Adt11::C0(v39, v42, {
                                            let a160 = v40;
                                            length_list(a160)
                                        }), List::<Adt11>::nil());
                                        append_list(a161, a162)
                                    }, match v45.clone() {
                                        Adt14::C0(_, _, _, _, h163, _, _, _, _, _) => {
                                            h163
                                        }
                                    }, match v45.clone() {
                                        Adt14::C0(_, _, _, _, _, h164, _, _, _, _) => {
                                            h164
                                        }
                                    }, match v45.clone() {
                                        Adt14::C0(_, _, _, _, _, _, h165, _, _, _) => {
                                            h165
                                        }
                                    }, match v45.clone() {
                                        Adt14::C0(_, _, _, _, _, _, _, h166, _, _) => {
                                            h166
                                        }
                                    }, match v45.clone() {
                                        Adt14::C0(_, _, _, _, _, _, _, _, h167, _) => {
                                            h167
                                        }
                                    }, match v45 {
                                        Adt14::C0(_, _, _, _, _, _, _, _, _, h168) => {
                                            h168
                                        }
                                    }))
                                } else {
                                    Err::<Adt14, Bytes>(f16(f17(Str::lit("runtime support: struct-fields: ")), f16(f18(v39), f16(f17(Str::lit(" has ")), f16(f47({
                                        let a169 = v42;
                                        length_list(a169)
                                    })?, f16(f17(Str::lit(" field names and ")), f16(f47(f46(v43)?)?, f17(Str::lit(" types")))))))))
                                }
                            }
                            Some(v46) => {
                                Err::<Adt14, Bytes>(v46)
                            }
                        }
                    }
                    Err(v44) => {
                        Err::<Adt14, Bytes>(v44)
                    }
                }
            }))
        }
        Adt1::C9(v52, v53, v54, v55) => {
            Ok(f38(f39(Str::lit("an enum name"), v52.clone())?, {
                let m170 = f40(v0, v52.clone())?;
                match m170 {
                    Ok(v57) => {
                        let m171 = f35(f37(Str::lit("an enum generic"), v53.clone())?, f35(f41(Str::lit("an enum"), v54)?, f48(v52.clone(), v55.clone())?));
                        match m171 {
                            None => {
                                let m172 = f49(v55.clone())?;
                                match m172 {
                                    None => {
                                        Ok::<Adt14, Bytes>(Adt14::C0(match v57.clone() {
                                            Adt14::C0(h173, _, _, _, _, _, _, _, _, _) => {
                                                h173
                                            }
                                        }, match v57.clone() {
                                            Adt14::C0(_, h174, _, _, _, _, _, _, _, _) => {
                                                h174
                                            }
                                        }, match v57.clone() {
                                            Adt14::C0(_, _, h175, _, _, _, _, _, _, _) => {
                                                h175
                                            }
                                        }, match v57.clone() {
                                            Adt14::C0(_, _, _, h176, _, _, _, _, _, _) => {
                                                h176
                                            }
                                        }, {
                                            let a179 = match v57.clone() {
                                                Adt14::C0(_, _, _, _, h177, _, _, _, _, _) => {
                                                    h177
                                                }
                                            };
                                            let a180 = List::cons(Adt12::C0(v52, v55, {
                                                let a178 = v53;
                                                length_list(a178)
                                            }), List::<Adt12>::nil());
                                            append_list(a179, a180)
                                        }, match v57.clone() {
                                            Adt14::C0(_, _, _, _, _, h181, _, _, _, _) => {
                                                h181
                                            }
                                        }, match v57.clone() {
                                            Adt14::C0(_, _, _, _, _, _, h182, _, _, _) => {
                                                h182
                                            }
                                        }, match v57.clone() {
                                            Adt14::C0(_, _, _, _, _, _, _, h183, _, _) => {
                                                h183
                                            }
                                        }, match v57.clone() {
                                            Adt14::C0(_, _, _, _, _, _, _, _, h184, _) => {
                                                h184
                                            }
                                        }, match v57 {
                                            Adt14::C0(_, _, _, _, _, _, _, _, _, h185) => {
                                                h185
                                            }
                                        }))
                                    }
                                    Some(v59) => {
                                        Err::<Adt14, Bytes>(f16(f17(Str::lit("runtime support: variant-name: ")), f16(f18(v52), f16(f17(Str::lit(" declares ")), f16(f18(v59), f17(Str::lit(" twice")))))))
                                    }
                                }
                            }
                            Some(v58) => {
                                Err::<Adt14, Bytes>(v58)
                            }
                        }
                    }
                    Err(v56) => {
                        Err::<Adt14, Bytes>(v56)
                    }
                }
            }))
        }
        Adt1::C10(_) => {
            Ok(Ok::<Adt14, Bytes>(v0))
        }
        Adt1::C11(v48, _) => {
            Ok(f38(f39(Str::lit("a module name"), v48.clone())?, f40(v0, v48)?))
        }
        Adt1::C12(_, _) => {
            Ok(Ok::<Adt14, Bytes>(v0))
        }
    }
}

pub fn f12(v0: Adt14, v1: Adt2, v2: u64, v3: bool) -> R<Option<Bytes>> {
    let m186 = v1;
    match m186 {
        Adt2::C0 => {
            Ok(None::<Bytes>)
        }
        Adt2::C1 => {
            Ok(None::<Bytes>)
        }
        Adt2::C2 => {
            Ok(None::<Bytes>)
        }
        Adt2::C3 => {
            Ok(None::<Bytes>)
        }
        Adt2::C4 => {
            Ok(None::<Bytes>)
        }
        Adt2::C5 => {
            if v3 {
                Ok(None::<Bytes>)
            } else {
                Ok(Some(f16(f17(Str::lit("runtime support: self-type: ")), f17(Str::lit("`Self` outside a trait or an impl")))))
            }
        }
        Adt2::C6(v4) => {
            let a187 = v4;
            let a188 = v2;
            if nat_lt(a187, a188) {
                Ok(None::<Bytes>)
            } else {
                Ok(Some(f16(f17(Str::lit("runtime support: type-parameter: ")), f16(f17(Str::lit("parameter ")), f16(f47(v4)?, f16(f17(Str::lit(" of ")), f16(f47(v2)?, f17(Str::lit(" in scope")))))))))
            }
        }
        Adt2::C7(r195) => {
            let v5 = (*r195).clone();
            f12(v0, v5, v2, v3)
        }
        Adt2::C8(r196) => {
            let v6 = (*r196).clone();
            f12(v0, v6, v2, v3)
        }
        Adt2::C9(r197, r198) => {
            let v7 = (*r197).clone();
            let v8 = (*r198).clone();
            let m189 = f12(v0.clone(), v7, v2, v3)?;
            match m189 {
                None => {
                    f12(v0, v8, v2, v3)
                }
                Some(v9) => {
                    Ok(Some(v9))
                }
            }
        }
        Adt2::C10(v10, r199) => {
            let v11 = (*r199).clone();
            let m190 = f50(v0.clone(), v10.clone());
            match m190 {
                None => {
                    Ok(Some(f16(f17(Str::lit("runtime support: type-name: ")), f16(f18(v10), f17(Str::lit(" is neither declared nor in the closed table"))))))
                }
                Some(v12) => {
                    if if match v0.clone() {
                        Adt14::C0(h191, _, _, _, _, _, _, _, _, _) => {
                            h191
                        }
                    } {
                        f51(v0.clone(), v10.clone())
                    } else {
                        false
                    } {
                        Ok(Some(f16(f17(Str::lit("runtime support: hidden-allocation: ")), f16(f17(Str::lit("rust-core renders ")), f16(f18(v10), f17(Str::lit(", which requires heap allocation")))))))
                    } else {
                        let a192 = v12;
                        let a193 = f46(v11.clone())?;
                        if nat_eq(a192, a193) {
                            f12(v0, v11, v2, v3)
                        } else {
                            Ok(Some(f16(f17(Str::lit("runtime support: type-arity: ")), f16(f18(v10), f16(f17(Str::lit(" takes ")), f16(f47(v12)?, f16(f17(Str::lit(" arguments, given ")), f47(f46(v11)?)?)))))))
                        }
                    }
                }
            }
        }
        Adt2::C11 => {
            Ok(None::<Bytes>)
        }
        Adt2::C12 => {
            Ok(None::<Bytes>)
        }
        Adt2::C13 => {
            Ok(None::<Bytes>)
        }
        Adt2::C14 => {
            Ok(None::<Bytes>)
        }
        Adt2::C15(r200) => {
            let v13 = (*r200).clone();
            f12(v0, v13, v2, v3)
        }
        Adt2::C16 => {
            Ok(None::<Bytes>)
        }
        Adt2::C17 => {
            Ok(None::<Bytes>)
        }
        Adt2::C18 => {
            Ok(None::<Bytes>)
        }
        Adt2::C19(r201, r202) => {
            let v14 = (*r201).clone();
            let v15 = (*r202).clone();
            let m194 = f12(v0.clone(), v14, v2, v3)?;
            match m194 {
                None => {
                    f12(v0, v15, v2, v3)
                }
                Some(v16) => {
                    Ok(Some(v16))
                }
            }
        }
    }
}

pub fn f13(v0: Adt14, v1: Adt16, v2: Adt3) -> R<Option<Bytes>> {
    let m203 = v2;
    match m203 {
        Adt3::C0(v3) => {
            let m205 = f52(match v1 {
                Adt16::C0(_, h204, _, _) => {
                    h204
                }
            }, v3.clone());
            match m205 {
                None => {
                    if f53(match v0 {
                        Adt14::C0(_, _, _, _, _, _, h206, _, _, _) => {
                            h206
                        }
                    }, v3.clone())? {
                        Ok(None::<Bytes>)
                    } else {
                        Ok(Some(f16(f17(Str::lit("runtime support: variable-scope: ")), f16(f18(v3), f17(Str::lit(" is not in scope"))))))
                    }
                }
                Some(_) => {
                    Ok(None::<Bytes>)
                }
            }
        }
        Adt3::C1(v5) => {
            f54(v5)
        }
        Adt3::C2(v6) => {
            f55(v0, v6)
        }
        Adt3::C3(v7, r251) => {
            let v8 = (*r251).clone();
            let m207 = f56(v0.clone(), v7, f57(v8.clone())?)?;
            match m207 {
                None => {
                    f58(v0, v1, v8)
                }
                Some(v9) => {
                    Ok(Some(v9))
                }
            }
        }
        Adt3::C4(r252, v11, r253) => {
            let v10 = (*r252).clone();
            let v12 = (*r253).clone();
            let m208 = f13(v0.clone(), v1.clone(), v10.clone())?;
            match m208 {
                None => {
                    let m209 = f59(v0.clone(), v1.clone(), v10, v11, f57(v12.clone())?)?;
                    match m209 {
                        None => {
                            f58(v0, v1, v12)
                        }
                        Some(v14) => {
                            Ok(Some(v14))
                        }
                    }
                }
                Some(v13) => {
                    Ok(Some(v13))
                }
            }
        }
        Adt3::C5(r254, _) => {
            let v15 = (*r254).clone();
            f13(v0, v1, v15)
        }
        Adt3::C6(r255, v18) => {
            let v17 = (*r255).clone();
            let m210 = f39(Str::lit("a field access"), v18)?;
            match m210 {
                None => {
                    f13(v0, v1, v17)
                }
                Some(v19) => {
                    Ok(Some(v19))
                }
            }
        }
        Adt3::C7(r256) => {
            let v20 = (*r256).clone();
            f13(v0, v1, v20)
        }
        Adt3::C8(r257, r258) => {
            let v21 = (*r257).clone();
            let v22 = (*r258).clone();
            let m211 = f13(v0.clone(), v1.clone(), v21)?;
            match m211 {
                None => {
                    f13(v0, v1, v22)
                }
                Some(v23) => {
                    Ok(Some(v23))
                }
            }
        }
        Adt3::C9(r259, r260) => {
            let v24 = (*r259).clone();
            let v25 = (*r260).clone();
            let m212 = f35(f60(v24.clone())?, f13(v0.clone(), v1.clone(), v24)?);
            match m212 {
                None => {
                    if f61(v25.clone()) {
                        Ok(Some(f16(f17(Str::lit("runtime support: match-arms: ")), f17(Str::lit("a match with no arm")))))
                    } else {
                        f62(v0, v1, v25)
                    }
                }
                Some(v26) => {
                    Ok(Some(v26))
                }
            }
        }
        Adt3::C10(_, _, _) => {
            Ok(Some(f16(f17(Str::lit("runtime support: let-position: ")), f17(Str::lit("a `let` that is not a statement of a block")))))
        }
        Adt3::C11(r261, r262) => {
            let v30 = (*r261).clone();
            let v31 = (*r262).clone();
            let m213 = f63(v1.clone(), v30.clone());
            match m213 {
                None => {
                    let m214 = f13(v0.clone(), v1.clone(), v30)?;
                    match m214 {
                        None => {
                            f13(v0, v1, v31)
                        }
                        Some(v33) => {
                            Ok(Some(v33))
                        }
                    }
                }
                Some(v32) => {
                    Ok(Some(v32))
                }
            }
        }
        Adt3::C12(r263, r264, r265) => {
            let v34 = (*r263).clone();
            let v35 = (*r264).clone();
            let v36 = (*r265).clone();
            let m215 = f35(f60(v34.clone())?, f13(v0.clone(), v1.clone(), v34)?);
            match m215 {
                None => {
                    let m216 = f64(v0.clone(), v1.clone(), v35)?;
                    match m216 {
                        None => {
                            f64(v0, v1, v36)
                        }
                        Some(v38) => {
                            Ok(Some(v38))
                        }
                    }
                }
                Some(v37) => {
                    Ok(Some(v37))
                }
            }
        }
        Adt3::C13(r266, r267) => {
            let v39 = (*r266).clone();
            let v40 = (*r267).clone();
            let m217 = f35(f60(v39.clone())?, f13(v0.clone(), v1.clone(), v39)?);
            match m217 {
                None => {
                    f64(v0, f65(v1, List::<Adt17>::nil(), true), v40)
                }
                Some(v41) => {
                    Ok(Some(v41))
                }
            }
        }
        Adt3::C14(v42, r268, r269) => {
            let v43 = (*r268).clone();
            let v44 = (*r269).clone();
            let m218 = f35(f60(v43.clone())?, f13(v0.clone(), v1.clone(), v43)?);
            match m218 {
                None => {
                    let m219 = f66(v0.clone(), v42.clone())?;
                    match m219 {
                        None => {
                            let m220 = f67(List::<Adt17>::nil(), v42)?;
                            match m220 {
                                Ok(v48) => {
                                    f64(v0, f65(v1, v48, true), v44)
                                }
                                Err(v47) => {
                                    Ok(Some(v47))
                                }
                            }
                        }
                        Some(v46) => {
                            Ok(Some(v46))
                        }
                    }
                }
                Some(v45) => {
                    Ok(Some(v45))
                }
            }
        }
        Adt3::C15(r270) => {
            let v49 = (*r270).clone();
            f64(v0, f65(v1, List::<Adt17>::nil(), true), v49)
        }
        Adt3::C16(v50, r271, r272) => {
            let v51 = (*r271).clone();
            let v52 = (*r272).clone();
            let m221 = f35(f60(v51.clone())?, f13(v0.clone(), v1.clone(), v51)?);
            match m221 {
                None => {
                    let m222 = f66(v0.clone(), v50.clone())?;
                    match m222 {
                        None => {
                            let m223 = f67(List::<Adt17>::nil(), v50)?;
                            match m223 {
                                Ok(v56) => {
                                    f64(v0, f65(v1, v56, true), v52)
                                }
                                Err(v55) => {
                                    Ok(Some(v55))
                                }
                            }
                        }
                        Some(v54) => {
                            Ok(Some(v54))
                        }
                    }
                }
                Some(v53) => {
                    Ok(Some(v53))
                }
            }
        }
        Adt3::C17 => {
            if match v1 {
                Adt16::C0(_, _, _, h224) => {
                    h224
                }
            } {
                Ok(None::<Bytes>)
            } else {
                Ok(Some(f16(f17(Str::lit("runtime support: break-position: ")), f17(Str::lit("a `break` outside a loop")))))
            }
        }
        Adt3::C18(r273) => {
            let v57 = (*r273).clone();
            if f68(v57.clone()) {
                Ok(None::<Bytes>)
            } else {
                f13(v0, v1, v57)
            }
        }
        Adt3::C19(r274) => {
            let v58 = (*r274).clone();
            let a225 = f57(v58.clone())?;
            let a226 = 2u64;
            if nat_lt(a225, a226) {
                Ok(Some(f16(f17(Str::lit("runtime support: tuple-arity: ")), f16(f17(Str::lit("a tuple of ")), f47(f57(v58)?)?))))
            } else {
                f58(v0, v1, v58)
            }
        }
        Adt3::C20(v59, v60, r275) => {
            let v61 = (*r275).clone();
            let m227 = f55(v0.clone(), v59.clone())?;
            match m227 {
                None => {
                    let m228 = f69(v0.clone(), v59.clone());
                    match m228 {
                        None => {
                            Ok(Some(f16(f17(Str::lit("runtime support: struct-fields: ")), f16(f18(f34(v59, Str::lit("::"))?), f17(Str::lit(" is no record struct or record variant"))))))
                        }
                        Some(v63) => {
                            if f70(v60.clone(), v63.clone(), f57(v61.clone())?)? {
                                f58(v0, v1, v61)
                            } else {
                                Ok(Some(f16(f17(Str::lit("runtime support: struct-fields: ")), f16(f18(f34(v59, Str::lit("::"))?), f16(f17(Str::lit(" is given ")), f16(f34(v60, Str::lit(", "))?, f16(f17(Str::lit(", not ")), f34(v63, Str::lit(", "))?)))))))
                            }
                        }
                    }
                }
                Some(v62) => {
                    Ok(Some(v62))
                }
            }
        }
        Adt3::C21(_, r276, r277) => {
            let v65 = (*r276).clone();
            let v66 = (*r277).clone();
            let m229 = f13(v0.clone(), v1.clone(), v65)?;
            match m229 {
                None => {
                    f13(v0, v1, v66)
                }
                Some(v67) => {
                    Ok(Some(v67))
                }
            }
        }
        Adt3::C22(r278) => {
            let v68 = (*r278).clone();
            f13(v0, v1, v68)
        }
        Adt3::C23(r279, v70) => {
            let v69 = (*r279).clone();
            let m234 = f35(f12(v0.clone(), v70.clone(), {
                let a231 = match v1.clone() {
                    Adt16::C0(h230, _, _, _) => {
                        h230
                    }
                };
                length_list(a231)
            }, {
                let a233 = f71(match v1.clone() {
                    Adt16::C0(_, _, h232, _) => {
                        h232
                    }
                });
                bool_not(a233)
            })?, f72(Str::lit("a cast type"), v70));
            match m234 {
                None => {
                    f13(v0, v1, v69)
                }
                Some(v71) => {
                    Ok(Some(v71))
                }
            }
        }
        Adt3::C24(r280, r281) => {
            let v72 = (*r280).clone();
            let v73 = (*r281).clone();
            let m235 = f13(v0.clone(), v1.clone(), v72)?;
            match m235 {
                None => {
                    f13(v0, v1, v73)
                }
                Some(v74) => {
                    Ok(Some(v74))
                }
            }
        }
        Adt3::C25(r282, r283, r284) => {
            let v75 = (*r282).clone();
            let v76 = (*r283).clone();
            let v77 = (*r284).clone();
            let m236 = f13(v0.clone(), v1.clone(), v75)?;
            match m236 {
                None => {
                    let m237 = if f68(v76.clone()) {
                        None::<Bytes>
                    } else {
                        f13(v0.clone(), v1.clone(), v76)?
                    };
                    match m237 {
                        None => {
                            if f68(v77.clone()) {
                                Ok(None::<Bytes>)
                            } else {
                                f13(v0, v1, v77)
                            }
                        }
                        Some(v79) => {
                            Ok(Some(v79))
                        }
                    }
                }
                Some(v78) => {
                    Ok(Some(v78))
                }
            }
        }
        Adt3::C26(r285) => {
            let v80 = (*r285).clone();
            f13(v0, v1, v80)
        }
        Adt3::C27(r286) => {
            let v81 = (*r286).clone();
            f13(v0, v1, v81)
        }
        Adt3::C28(r287, v83, v84, r288) => {
            let v82 = (*r287).clone();
            let v85 = (*r288).clone();
            let m238 = f13(v0.clone(), v1.clone(), v82)?;
            match m238 {
                None => {
                    let m239 = f73(v83.clone());
                    match m239 {
                        None => {
                            Ok(Some(f16(f17(Str::lit("runtime support: method-name: ")), f16(f18(v83), f17(Str::lit(" takes no type arguments in the closed table"))))))
                        }
                        Some(v87) => {
                            let a240 = v87;
                            let a241 = f46(v84.clone())?;
                            if if nat_eq(a240, a241) {
                                let a242 = f74(v83.clone());
                                let a243 = f57(v85.clone())?;
                                nat_eq(a242, a243)
                            } else {
                                false
                            } {
                                if if match v0.clone() {
                                    Adt14::C0(h244, _, _, _, _, _, _, _, _, _) => {
                                        h244
                                    }
                                } {
                                    f32(v83.clone(), f17(Str::lit("collect")))
                                } else {
                                    false
                                } {
                                    Ok(Some(f16(f17(Str::lit("runtime support: hidden-allocation: ")), f16(f17(Str::lit("rust-core renders ")), f16(f18(v83), f17(Str::lit(", which requires heap allocation")))))))
                                } else {
                                    let m249 = f35(f12(v0.clone(), v84.clone(), {
                                        let a246 = match v1.clone() {
                                            Adt16::C0(h245, _, _, _) => {
                                                h245
                                            }
                                        };
                                        length_list(a246)
                                    }, {
                                        let a248 = f71(match v1.clone() {
                                            Adt16::C0(_, _, h247, _) => {
                                                h247
                                            }
                                        });
                                        bool_not(a248)
                                    })?, f72(Str::lit("a type argument"), v84));
                                    match m249 {
                                        None => {
                                            f58(v0, v1, v85)
                                        }
                                        Some(v88) => {
                                            Ok(Some(v88))
                                        }
                                    }
                                }
                            } else {
                                Ok(Some(f16(f17(Str::lit("runtime support: method-arity: ")), f16(f18(v83.clone()), f16(f17(Str::lit(" takes ")), f16(f47(v87)?, f16(f17(Str::lit(" type arguments and ")), f16(f47(f74(v83))?, f17(Str::lit(" arguments"))))))))))
                            }
                        }
                    }
                }
                Some(v86) => {
                    Ok(Some(v86))
                }
            }
        }
        Adt3::C29(r289, r290) => {
            let v89 = (*r289).clone();
            let v90 = (*r290).clone();
            let m250 = f75(v0.clone(), v1, v89)?;
            match m250 {
                Ok(v92) => {
                    f13(v0, v92, v90)
                }
                Err(v91) => {
                    Ok(Some(v91))
                }
            }
        }
        Adt3::C30 => {
            Ok(None::<Bytes>)
        }
        Adt3::C31 => {
            Ok(None::<Bytes>)
        }
        Adt3::C32(_, _) => {
            Ok(None::<Bytes>)
        }
        Adt3::C33(_, _, _, _) => {
            Ok(None::<Bytes>)
        }
        Adt3::C34(_, _, _, _) => {
            Ok(None::<Bytes>)
        }
    }
}

pub fn f14(_: ()) -> Adt16 {
    Adt16::C0(List::<Adt7>::nil(), List::<Adt17>::nil(), Adt2::C17, false)
}

pub fn f15(v0: List<Adt5>) -> R<Option<Bytes>> {
    let m292 = f76(Fn6::F77, Adt18::C0(List::<Bytes>::nil(), None::<Bytes>), v0)?;
    match m292 {
        Adt18::C0(_, h291) => {
            Ok(h291)
        }
    }
}

pub fn f16(v0: Bytes, v1: Bytes) -> Bytes {
    let a293 = v0;
    let a294 = v1;
    append_bytes(a293, a294)
}

pub fn f17(v0: Str) -> Bytes {
    let a295 = v0;
    utf8_encode(a295)
}

pub fn f18(v0: Bytes) -> Bytes {
    f16(f17(Str::lit("`")), f16(v0, f17(Str::lit("`"))))
}

pub fn f19(v0: Fn3, p299: Option<Bytes>, p300: List<Adt5>) -> R<Option<Bytes>> {
    let m298 = (p299, p300);
    {
        let mut m298 = m298;
        loop {
            let step = {
                let (v1, v2) = m298;
                let m296 = v2;
                match m296.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Adt5>), R<Option<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Adt5>), R<Option<Bytes>>>(({
                            let c297: Fn3 = v0.clone();
                            c297.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m298 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f20(v0: Adt14, v1: Option<Bytes>, v2: Adt5) -> R<Option<Bytes>> {
    let m301 = v1;
    match m301 {
        None => {
            f78(v0, v2, 0u64, true)
        }
        Some(v3) => {
            Ok(Some(v3))
        }
    }
}

pub fn f21(v0: Adt14, v1: List<Adt7>) -> R<Option<Bytes>> {
    f79(Fn7::F80(v0), None::<Bytes>, v1)
}

pub fn f22(v0: Adt14, v1: Bytes, v2: List<Adt7>, v3: Adt2, v4: List<Adt8>) -> R<Option<Bytes>> {
    let m302 = f21(v0.clone(), v2.clone())?;
    match m302 {
        None => {
            let m303 = f81(v0.clone(), v1.clone());
            match m303 {
                None => {
                    Ok(Some(f16(f17(Str::lit("runtime support: impl-trait: ")), f16(f17(Str::lit("`impl ")), f16(v1, f17(Str::lit("` names no declared trait")))))))
                }
                Some(v6) => {
                    let m304 = f19(Fn3::F82(v4.clone(), v1.clone()), None::<Bytes>, v6.clone())?;
                    match m304 {
                        None => {
                            f23(Fn4::F83(v2, v0, v6, v1, v3), None::<Bytes>, v4)
                        }
                        Some(v7) => {
                            Ok(Some(v7))
                        }
                    }
                }
            }
        }
        Some(v5) => {
            Ok(Some(v5))
        }
    }
}

pub fn f23(v0: Fn4, p308: Option<Bytes>, p309: List<Adt8>) -> R<Option<Bytes>> {
    let m307 = (p308, p309);
    {
        let mut m307 = m307;
        loop {
            let step = {
                let (v1, v2) = m307;
                let m305 = v2;
                match m305.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Adt8>), R<Option<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Adt8>), R<Option<Bytes>>>(({
                            let c306: Fn4 = v0.clone();
                            c306.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m307 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f24(v0: List<Adt7>, v1: Adt14, v2: Adt2, v3: Option<Bytes>, v4: Adt8) -> R<Option<Bytes>> {
    let m310 = v3;
    match m310 {
        None => {
            f84(v1, v0, v2, v4)
        }
        Some(v5) => {
            Ok(Some(v5))
        }
    }
}

pub fn f25(v0: Adt14, v1: Adt8) -> R<Option<Bytes>> {
    let a312 = match v1.clone() {
        Adt8::C0(_, _, h311, _) => {
            h311
        }
    };
    if bool_not(a312) {
        Ok(Some(f16(f17(Str::lit("runtime support: function-visibility: ")), f16(f18({
            let m315 = match v1 {
                Adt8::C0(h314, _, _, _) => {
                    h314
                }
            };
            match m315 {
                Adt5::C0(h313, _, _, _, _) => {
                    h313
                }
            }
        }), f17(Str::lit(" is a free function that is not public"))))))
    } else {
        let m318 = match v1.clone() {
            Adt8::C0(h317, _, _, _) => {
                h317
            }
        };
        if match m318 {
            Adt5::C0(_, h316, _, _, _) => {
                h316
            }
        } {
            Ok(Some(f16(f17(Str::lit("runtime support: function-receiver: ")), f16(f17(Str::lit("free function ")), f16(f18({
                let m321 = match v1 {
                    Adt8::C0(h320, _, _, _) => {
                        h320
                    }
                };
                match m321 {
                    Adt5::C0(h319, _, _, _, _) => {
                        h319
                    }
                }
            }), f17(Str::lit(" takes `&self`")))))))
        } else {
            f84(v0, List::<Adt7>::nil(), Adt2::C17, v1)
        }
    }
}

pub fn f26(v0: Fn5, p325: Option<Bytes>, p326: List<Adt9>) -> R<Option<Bytes>> {
    let m324 = (p325, p326);
    {
        let mut m324 = m324;
        loop {
            let step = {
                let (v1, v2) = m324;
                let m322 = v2;
                match m322.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Adt9>), R<Option<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Adt9>), R<Option<Bytes>>>(({
                            let c323: Fn5 = v0.clone();
                            c323.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m324 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f27(v0: List<Bytes>, v1: Adt14, v2: Option<Bytes>, v3: Adt9) -> R<Option<Bytes>> {
    let m327 = v2;
    match m327 {
        None => {
            let m328 = v3;
            match m328 {
                Adt9::C0(_) => {
                    Ok(None::<Bytes>)
                }
                Adt9::C1(_, v7) => {
                    f12(v1, v7, {
                        let a329 = v0;
                        length_list(a329)
                    }, false)
                }
                Adt9::C2(_, _, v10) => {
                    f12(v1, v10, {
                        let a330 = v0;
                        length_list(a330)
                    }, false)
                }
            }
        }
        Some(v4) => {
            Ok(Some(v4))
        }
    }
}

pub fn f28(v0: Bytes) -> Option<Bytes> {
    let a332 = {
        let a331 = v0;
        length_bytes(a331)
    };
    let a333 = 0u64;
    if nat_eq(a332, a333) {
        Some(f17(Str::lit("runtime support: defined-path: empty")))
    } else {
        None::<Bytes>
    }
}

pub fn f29(v0: Bytes, v1: Adt14, v2: Option<Bytes>, v3: Adt1) -> R<Option<Bytes>> {
    let m334 = v2;
    match m334 {
        None => {
            f85(v1, v0, v3)
        }
        Some(v4) => {
            Ok(Some(v4))
        }
    }
}

pub fn f30(v0: Bytes, v1: List<Bytes>) -> R<Option<Bytes>> {
    if f32(v0.clone(), f17(Str::lit("no_std"))) {
        let m335 = v1;
        match m335.uncons() {
            None => {
                Ok(None::<Bytes>)
            }
            Some((_, _)) => {
                Ok(Some(f16(f17(Str::lit("runtime support: attribute-args: ")), f16(f18(v0), f17(Str::lit(" takes no arguments"))))))
            }
        }
    } else {
        if f32(v0.clone(), f17(Str::lit("forbid"))) {
            let m336 = v1.clone();
            match m336.uncons() {
                None => {
                    Ok(Some(f16(f17(Str::lit("runtime support: attribute-args: ")), f16(f18(v0), f17(Str::lit(" takes a lint"))))))
                }
                Some((_, _)) => {
                    if f86(Fn8::F87, false, v1)? {
                        Ok(Some(f16(f17(Str::lit("runtime support: attribute-args: ")), f16(f18(v0), f17(Str::lit(" takes a lint with a name"))))))
                    } else {
                        Ok(None::<Bytes>)
                    }
                }
            }
        } else {
            Ok(Some(f16(f17(Str::lit("runtime support: attribute-name: ")), f16(f18(v0), f17(Str::lit(" is not an inner attribute of the closed set"))))))
        }
    }
}

pub fn f31(v0: Result<Adt14, Bytes>, v1: Option<Bytes>) -> Result<Adt14, Bytes> {
    let m337 = v0.clone();
    match m337 {
        Ok(_) => {
            let m338 = v1;
            match m338 {
                None => {
                    v0
                }
                Some(v4) => {
                    Err::<Adt14, Bytes>(v4)
                }
            }
        }
        Err(v2) => {
            Err::<Adt14, Bytes>(v2)
        }
    }
}

pub fn f32(v0: Bytes, v1: Bytes) -> bool {
    let a339 = v0;
    let a340 = v1;
    equal(a339, a340)
}

pub fn f33(v0: List<Bytes>) -> Bytes {
    f88(Fn9::F89, f17(Str::lit("")), v0)
}

pub fn f34(v0: List<Bytes>, v1: Str) -> R<Bytes> {
    f90(v0, f17(v1))
}

pub fn f35(v0: Option<Bytes>, v1: Option<Bytes>) -> Option<Bytes> {
    let m341 = v0;
    match m341 {
        None => {
            v1
        }
        Some(v2) => {
            Some(v2)
        }
    }
}

pub fn f36(v0: Str, v1: List<Bytes>) -> R<Option<Bytes>> {
    f91(Fn10::F92(v0), None::<Bytes>, v1)
}

pub fn f37(v0: Str, v1: List<Bytes>) -> R<Option<Bytes>> {
    f91(Fn10::F93(v0), None::<Bytes>, v1)
}

pub fn f38(v0: Option<Bytes>, v1: Result<Adt14, Bytes>) -> Result<Adt14, Bytes> {
    let m342 = v0;
    match m342 {
        None => {
            v1
        }
        Some(v2) => {
            Err::<Adt14, Bytes>(v2)
        }
    }
}

pub fn f39(v0: Str, v1: Bytes) -> R<Option<Bytes>> {
    if if f94(v1.clone())? {
        let a343 = f53(f95(), v1.clone())?;
        bool_not(a343)
    } else {
        false
    } {
        Ok(None::<Bytes>)
    } else {
        Ok(Some(f16(f17(Str::lit("runtime support: identifier: ")), f16(f17(v0), f16(f17(Str::lit(" ")), f16(f18(v1), f17(Str::lit(" is not an identifier"))))))))
    }
}

pub fn f40(v0: Adt14, v1: Bytes) -> R<Result<Adt14, Bytes>> {
    if f53(match v0.clone() {
        Adt14::C0(_, h344, _, _, _, _, _, _, _, _) => {
            h344
        }
    }, v1.clone())? {
        Ok(Err::<Adt14, Bytes>(f16(f17(Str::lit("runtime support: duplicate-name: ")), f16(f18(v1), f17(Str::lit(" is declared twice"))))))
    } else {
        Ok(Ok::<Adt14, Bytes>(Adt14::C0(match v0.clone() {
            Adt14::C0(h345, _, _, _, _, _, _, _, _, _) => {
                h345
            }
        }, {
            let a347 = match v0.clone() {
                Adt14::C0(_, h346, _, _, _, _, _, _, _, _) => {
                    h346
                }
            };
            let a348 = List::cons(v1, List::<Bytes>::nil());
            append_list(a347, a348)
        }, match v0.clone() {
            Adt14::C0(_, _, h349, _, _, _, _, _, _, _) => {
                h349
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, h350, _, _, _, _, _, _) => {
                h350
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, h351, _, _, _, _, _) => {
                h351
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, h352, _, _, _, _) => {
                h352
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, h353, _, _, _) => {
                h353
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h354, _, _) => {
                h354
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, h355, _) => {
                h355
            }
        }, match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, _, h356) => {
                h356
            }
        })))
    }
}

pub fn f41(v0: Str, v1: List<Bytes>) -> R<Option<Bytes>> {
    f91(Fn10::F96(v0), None::<Bytes>, v1)
}

pub fn f42(v0: Adt14, v1: List<Adt5>) -> Adt14 {
    f97(Fn11::F98, v0, v1)
}

pub fn f43(v0: Adt14, v1: Adt2, v2: List<Adt8>) -> Adt14 {
    let m357 = f99(v1);
    match m357 {
        None => {
            v0
        }
        Some(v3) => {
            f100(v0, v3, v2)
        }
    }
}

pub fn f44(v0: Adt14, v1: List<Adt8>) -> Adt14 {
    f101(Fn12::F102, v0, v1)
}

pub fn f45(v0: Adt2, v1: u64) -> R<List<Bytes>> {
    let m358 = v0;
    match m358 {
        Adt2::C0 => {
            Ok(List::<Bytes>::nil())
        }
        Adt2::C1 => {
            Ok(List::<Bytes>::nil())
        }
        Adt2::C2 => {
            Ok(List::<Bytes>::nil())
        }
        Adt2::C3 => {
            Ok(List::<Bytes>::nil())
        }
        Adt2::C4 => {
            Ok(List::<Bytes>::nil())
        }
        Adt2::C5 => {
            Ok(List::<Bytes>::nil())
        }
        Adt2::C6(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt2::C7(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt2::C8(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt2::C9(_, _) => {
            Ok(List::<Bytes>::nil())
        }
        Adt2::C10(_, _) => {
            Ok(List::<Bytes>::nil())
        }
        Adt2::C11 => {
            Ok(List::<Bytes>::nil())
        }
        Adt2::C12 => {
            Ok(List::<Bytes>::nil())
        }
        Adt2::C13 => {
            Ok(List::<Bytes>::nil())
        }
        Adt2::C14 => {
            Ok(List::<Bytes>::nil())
        }
        Adt2::C15(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt2::C16 => {
            Ok(List::<Bytes>::nil())
        }
        Adt2::C17 => {
            Ok(List::<Bytes>::nil())
        }
        Adt2::C18 => {
            Ok(List::<Bytes>::nil())
        }
        Adt2::C19(_, r361) => {
            let v11 = (*r361).clone();
            Ok(List::cons(f47(v1)?, f45(v11, {
                let a359 = v1;
                let a360 = 1u64;
                nat_add(a359, a360)?
            })?))
        }
    }
}

pub fn f46(v0: Adt2) -> R<u64> {
    let m362 = v0;
    match m362 {
        Adt2::C0 => {
            Ok(0u64)
        }
        Adt2::C1 => {
            Ok(0u64)
        }
        Adt2::C2 => {
            Ok(0u64)
        }
        Adt2::C3 => {
            Ok(0u64)
        }
        Adt2::C4 => {
            Ok(0u64)
        }
        Adt2::C5 => {
            Ok(0u64)
        }
        Adt2::C6(_) => {
            Ok(0u64)
        }
        Adt2::C7(_) => {
            Ok(0u64)
        }
        Adt2::C8(_) => {
            Ok(0u64)
        }
        Adt2::C9(_, _) => {
            Ok(0u64)
        }
        Adt2::C10(_, _) => {
            Ok(0u64)
        }
        Adt2::C11 => {
            Ok(0u64)
        }
        Adt2::C12 => {
            Ok(0u64)
        }
        Adt2::C13 => {
            Ok(0u64)
        }
        Adt2::C14 => {
            Ok(0u64)
        }
        Adt2::C15(_) => {
            Ok(0u64)
        }
        Adt2::C16 => {
            Ok(0u64)
        }
        Adt2::C17 => {
            Ok(0u64)
        }
        Adt2::C18 => {
            Ok(0u64)
        }
        Adt2::C19(_, r365) => {
            let v10 = (*r365).clone();
            let a363 = f46(v10)?;
            let a364 = 1u64;
            nat_add(a363, a364)
        }
    }
}

pub fn f47(v0: u64) -> R<Bytes> {
    f103(v0)
}

pub fn f48(v0: Bytes, v1: List<Adt9>) -> R<Option<Bytes>> {
    let m367 = f104(Fn13::F105(v0), Adt18::C0(List::<Bytes>::nil(), None::<Bytes>), v1)?;
    match m367 {
        Adt18::C0(_, h366) => {
            Ok(h366)
        }
    }
}

pub fn f49(v0: List<Adt9>) -> R<Option<Bytes>> {
    let m369 = f104(Fn13::F106, Adt18::C0(List::<Bytes>::nil(), None::<Bytes>), v0)?;
    match m369 {
        Adt18::C0(_, h368) => {
            Ok(h368)
        }
    }
}

pub fn f50(v0: Adt14, v1: Bytes) -> Option<u64> {
    let m370 = f107(v0.clone(), v1.clone());
    match m370 {
        None => {
            let m372 = f108(v0.clone(), v1.clone());
            match m372 {
                None => {
                    let m374 = f109(v0, v1.clone());
                    match m374 {
                        None => {
                            f110(v1)
                        }
                        Some(v4) => {
                            Some(match v4 {
                                Adt13::C0(_, h375) => {
                                    h375
                                }
                            })
                        }
                    }
                }
                Some(v3) => {
                    Some(match v3 {
                        Adt12::C0(_, _, h373) => {
                            h373
                        }
                    })
                }
            }
        }
        Some(v2) => {
            Some(match v2 {
                Adt11::C0(_, _, h371) => {
                    h371
                }
            })
        }
    }
}

pub fn f51(v0: Adt14, v1: Bytes) -> bool {
    let m376 = f107(v0.clone(), v1.clone());
    match m376 {
        None => {
            let m377 = f108(v0.clone(), v1.clone());
            match m377 {
                None => {
                    let m378 = f109(v0, v1.clone());
                    match m378 {
                        None => {
                            f111(v1)
                        }
                        Some(_) => {
                            false
                        }
                    }
                }
                Some(_) => {
                    false
                }
            }
        }
        Some(_) => {
            false
        }
    }
}

pub fn f52(v0: List<Adt17>, v1: Bytes) -> Option<Adt17> {
    f112(Fn14::F113(v1), None::<Adt17>, v0)
}

pub fn f53(v0: List<Bytes>, v1: Bytes) -> R<bool> {
    f86(Fn8::F114(v1), false, v0)
}

pub fn f54(v0: u64) -> R<Option<Bytes>> {
    let a379 = v0;
    let a380 = 1u64;
    if nat_eq(a379, a380) {
        Ok(None::<Bytes>)
    } else {
        Ok(Some(f16(f17(Str::lit("runtime support: char-literal: ")), f16(f17(Str::lit("a character literal of ")), f16(f47(v0)?, f17(Str::lit(" scalar values")))))))
    }
}

pub fn f55(v0: Adt14, v1: List<Bytes>) -> R<Option<Bytes>> {
    let m381 = f36(Str::lit("a path segment"), v1.clone())?;
    match m381 {
        None => {
            f115(v0, v1)
        }
        Some(v2) => {
            Ok(Some(v2))
        }
    }
}

pub fn f56(v0: Adt14, v1: List<Bytes>, v2: u64) -> R<Option<Bytes>> {
    if f116(v0.clone(), v1.clone()) {
        Ok(Some(f16(f17(Str::lit("runtime support: hidden-allocation: ")), f16(f17(Str::lit("rust-core renders ")), f16(f18(f34(v1, Str::lit("::"))?), f17(Str::lit(", which requires heap allocation")))))))
    } else {
        let m382 = f117(v0, v1.clone())?;
        match m382 {
            None => {
                Ok(Some(f16(f17(Str::lit("runtime support: call-name: ")), f16(f18(f34(v1, Str::lit("::"))?), f17(Str::lit(" is neither declared nor in the closed table"))))))
            }
            Some(v3) => {
                let a383 = v3;
                let a384 = v2;
                if nat_eq(a383, a384) {
                    Ok(None::<Bytes>)
                } else {
                    Ok(Some(f16(f17(Str::lit("runtime support: call-arity: ")), f16(f18(f34(v1, Str::lit("::"))?), f16(f17(Str::lit(" takes ")), f16(f47(v3)?, f16(f17(Str::lit(" arguments, given ")), f47(v2)?)))))))
                }
            }
        }
    }
}

pub fn f57(v0: Adt3) -> R<u64> {
    let m385 = v0;
    match m385 {
        Adt3::C0(_) => {
            Ok(0u64)
        }
        Adt3::C1(_) => {
            Ok(0u64)
        }
        Adt3::C2(_) => {
            Ok(0u64)
        }
        Adt3::C3(_, _) => {
            Ok(0u64)
        }
        Adt3::C4(_, _, _) => {
            Ok(0u64)
        }
        Adt3::C5(_, _) => {
            Ok(0u64)
        }
        Adt3::C6(_, _) => {
            Ok(0u64)
        }
        Adt3::C7(_) => {
            Ok(0u64)
        }
        Adt3::C8(_, _) => {
            Ok(0u64)
        }
        Adt3::C9(_, _) => {
            Ok(0u64)
        }
        Adt3::C10(_, _, _) => {
            Ok(0u64)
        }
        Adt3::C11(_, _) => {
            Ok(0u64)
        }
        Adt3::C12(_, _, _) => {
            Ok(0u64)
        }
        Adt3::C13(_, _) => {
            Ok(0u64)
        }
        Adt3::C14(_, _, _) => {
            Ok(0u64)
        }
        Adt3::C15(_) => {
            Ok(0u64)
        }
        Adt3::C16(_, _, _) => {
            Ok(0u64)
        }
        Adt3::C17 => {
            Ok(0u64)
        }
        Adt3::C18(_) => {
            Ok(0u64)
        }
        Adt3::C19(_) => {
            Ok(0u64)
        }
        Adt3::C20(_, _, _) => {
            Ok(0u64)
        }
        Adt3::C21(_, _, _) => {
            Ok(0u64)
        }
        Adt3::C22(_) => {
            Ok(0u64)
        }
        Adt3::C23(_, _) => {
            Ok(0u64)
        }
        Adt3::C24(_, _) => {
            Ok(0u64)
        }
        Adt3::C25(_, _, _) => {
            Ok(0u64)
        }
        Adt3::C26(_) => {
            Ok(0u64)
        }
        Adt3::C27(_) => {
            Ok(0u64)
        }
        Adt3::C28(_, _, _, _) => {
            Ok(0u64)
        }
        Adt3::C29(_, _) => {
            Ok(0u64)
        }
        Adt3::C30 => {
            Ok(0u64)
        }
        Adt3::C31 => {
            Ok(0u64)
        }
        Adt3::C32(_, r388) => {
            let v60 = (*r388).clone();
            let a386 = f57(v60)?;
            let a387 = 1u64;
            nat_add(a386, a387)
        }
        Adt3::C33(_, _, _, _) => {
            Ok(0u64)
        }
        Adt3::C34(_, _, _, _) => {
            Ok(0u64)
        }
    }
}

pub fn f58(v0: Adt14, v1: Adt16, p394: Adt3) -> R<Option<Bytes>> {
    let m393 = p394;
    {
        let mut m393 = m393;
        loop {
            let step = {
                let v2 = m393;
                let m389 = v2;
                match m389 {
                    Adt3::C0(_) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C1(_) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C2(_) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C3(_, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C4(_, _, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C5(_, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C6(_, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C7(_) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C8(_, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C9(_, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C10(_, _, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C11(_, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C12(_, _, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C13(_, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C14(_, _, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C15(_) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C16(_, _, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C17 => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C18(_) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C19(_) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C20(_, _, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C21(_, _, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C22(_) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C23(_, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C24(_, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C25(_, _, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C26(_) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C27(_) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C28(_, _, _, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C29(_, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C30 => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C31 => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C32(r391, r392) => {
                        let v61 = (*r391).clone();
                        let v62 = (*r392).clone();
                        let m390 = f13(v0.clone(), v1.clone(), v61)?;
                        match m390 {
                            None => {
                                Ok::<Adt3, R<Option<Bytes>>>(v62)
                            }
                            Some(v63) => {
                                Err::<Adt3, R<Option<Bytes>>>(Ok(Some(v63)))
                            }
                        }
                    }
                    Adt3::C33(_, _, _, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C34(_, _, _, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                }
            };
            match step {
                Ok(next) => { m393 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f59(v0: Adt14, v1: Adt16, v2: Adt3, v3: Bytes, v4: u64) -> R<Option<Bytes>> {
    let m395 = f118(v3.clone());
    match m395 {
        None => {
            let v6: List<u64> = f120(v0.clone(), v3.clone());
            let m399 = v6.clone();
            match m399.uncons() {
                None => {
                    Ok(Some(f16(f17(Str::lit("runtime support: method-name: ")), f16(f18(v3), f17(Str::lit(" is a method of no declared trait or impl and not in the closed table"))))))
                }
                Some((_, _)) => {
                    if f121(v6.clone(), v4) {
                        f122(v0, v1, v2, v3)
                    } else {
                        Ok(Some(f16(f17(Str::lit("runtime support: method-arity: ")), f16(f18(v3), f16(f17(Str::lit(" takes ")), f16(f123(v6)?, f16(f17(Str::lit(" arguments, given ")), f47(v4)?)))))))
                    }
                }
            }
        }
        Some(v5) => {
            if if f119(v3.clone()) {
                match v0 {
                    Adt14::C0(h396, _, _, _, _, _, _, _, _, _) => {
                        h396
                    }
                }
            } else {
                false
            } {
                Ok(Some(f16(f17(Str::lit("runtime support: hidden-allocation: ")), f16(f17(Str::lit("rust-core renders ")), f16(f18(v3), f17(Str::lit(", which requires heap allocation")))))))
            } else {
                let a397 = v5;
                let a398 = v4;
                if nat_eq(a397, a398) {
                    Ok(None::<Bytes>)
                } else {
                    Ok(Some(f16(f17(Str::lit("runtime support: method-arity: ")), f16(f18(v3), f16(f17(Str::lit(" takes ")), f16(f47(v5)?, f16(f17(Str::lit(" arguments, given ")), f47(v4)?)))))))
                }
            }
        }
    }
}

pub fn f60(v0: Adt3) -> R<Option<Bytes>> {
    if f124(v0)? {
        Ok(Some(f16(f17(Str::lit("runtime support: struct-head: ")), f17(Str::lit("a struct literal outside parentheses in the head of a condition, a loop, or a match")))))
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f61(v0: Adt3) -> bool {
    let m400 = v0;
    match m400 {
        Adt3::C0(_) => {
            false
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
        Adt3::C5(_, _) => {
            false
        }
        Adt3::C6(_, _) => {
            false
        }
        Adt3::C7(_) => {
            false
        }
        Adt3::C8(_, _) => {
            false
        }
        Adt3::C9(_, _) => {
            false
        }
        Adt3::C10(_, _, _) => {
            false
        }
        Adt3::C11(_, _) => {
            false
        }
        Adt3::C12(_, _, _) => {
            false
        }
        Adt3::C13(_, _) => {
            false
        }
        Adt3::C14(_, _, _) => {
            false
        }
        Adt3::C15(_) => {
            false
        }
        Adt3::C16(_, _, _) => {
            false
        }
        Adt3::C17 => {
            false
        }
        Adt3::C18(_) => {
            false
        }
        Adt3::C19(_) => {
            false
        }
        Adt3::C20(_, _, _) => {
            false
        }
        Adt3::C21(_, _, _) => {
            false
        }
        Adt3::C22(_) => {
            false
        }
        Adt3::C23(_, _) => {
            false
        }
        Adt3::C24(_, _) => {
            false
        }
        Adt3::C25(_, _, _) => {
            false
        }
        Adt3::C26(_) => {
            false
        }
        Adt3::C27(_) => {
            false
        }
        Adt3::C28(_, _, _, _) => {
            false
        }
        Adt3::C29(_, _) => {
            false
        }
        Adt3::C30 => {
            false
        }
        Adt3::C31 => {
            true
        }
        Adt3::C32(_, _) => {
            false
        }
        Adt3::C33(_, _, _, _) => {
            false
        }
        Adt3::C34(_, _, _, _) => {
            false
        }
    }
}

pub fn f62(v0: Adt14, v1: Adt16, p410: Adt3) -> R<Option<Bytes>> {
    let m409 = p410;
    {
        let mut m409 = m409;
        loop {
            let step = {
                let v2 = m409;
                let m401 = v2;
                match m401 {
                    Adt3::C0(_) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C1(_) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C2(_) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C3(_, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C4(_, _, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C5(_, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C6(_, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C7(_) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C8(_, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C9(_, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C10(_, _, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C11(_, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C12(_, _, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C13(_, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C14(_, _, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C15(_) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C16(_, _, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C17 => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C18(_) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C19(_) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C20(_, _, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C21(_, _, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C22(_) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C23(_, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C24(_, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C25(_, _, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C26(_) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C27(_) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C28(_, _, _, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C29(_, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C30 => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C31 => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C32(_, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C33(_, _, _, _) => {
                        Err::<Adt3, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Adt3::C34(v67, r406, r407, r408) => {
                        let v68 = (*r406).clone();
                        let v69 = (*r407).clone();
                        let v70 = (*r408).clone();
                        let m402 = f66(v0.clone(), v67.clone())?;
                        match m402 {
                            None => {
                                let m403 = f67(List::<Adt17>::nil(), v67)?;
                                match m403 {
                                    Ok(v73) => {
                                        let v74: Adt16 = f65(v1.clone(), v73, false);
                                        let m404 = if f68(v68.clone()) {
                                            None::<Bytes>
                                        } else {
                                            f13(v0.clone(), v74.clone(), v68)?
                                        };
                                        match m404 {
                                            None => {
                                                let m405 = f13(v0.clone(), v74, v69)?;
                                                match m405 {
                                                    None => {
                                                        Ok::<Adt3, R<Option<Bytes>>>(v70)
                                                    }
                                                    Some(v76) => {
                                                        Err::<Adt3, R<Option<Bytes>>>(Ok(Some(v76)))
                                                    }
                                                }
                                            }
                                            Some(v75) => {
                                                Err::<Adt3, R<Option<Bytes>>>(Ok(Some(v75)))
                                            }
                                        }
                                    }
                                    Err(v72) => {
                                        Err::<Adt3, R<Option<Bytes>>>(Ok(Some(v72)))
                                    }
                                }
                            }
                            Some(v71) => {
                                Err::<Adt3, R<Option<Bytes>>>(Ok(Some(v71)))
                            }
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

pub fn f63(v0: Adt16, p421: Adt3) -> Option<Bytes> {
    let m420 = p421;
    {
        let mut m420 = m420;
        loop {
            let step = {
                let v1 = m420;
                let m411 = v1;
                match m411 {
                    Adt3::C0(v2) => {
                        let m413 = f52(match v0.clone() {
                            Adt16::C0(_, h412, _, _) => {
                                h412
                            }
                        }, v2.clone());
                        match m413 {
                            None => {
                                Err::<Adt3, Option<Bytes>>(Some(f16(f17(Str::lit("runtime support: variable-scope: ")), f16(f18(v2), f17(Str::lit(" is not in scope"))))))
                            }
                            Some(v3) => {
                                if match v3.clone() {
                                    Adt17::C0(_, h414, _) => {
                                        h414
                                    }
                                } {
                                    Err::<Adt3, Option<Bytes>>(None::<Bytes>)
                                } else {
                                    if f125(match v3 {
                                        Adt17::C0(_, _, h415) => {
                                            h415
                                        }
                                    }) {
                                        Err::<Adt3, Option<Bytes>>(None::<Bytes>)
                                    } else {
                                        Err::<Adt3, Option<Bytes>>(Some(f16(f17(Str::lit("runtime support: assign-immutable: ")), f16(f18(v2), f17(Str::lit(" is assigned and is not `mut` or `&mut`"))))))
                                    }
                                }
                            }
                        }
                    }
                    Adt3::C1(_) => {
                        Err::<Adt3, Option<Bytes>>(Some(f16(f17(Str::lit("runtime support: assign-place: ")), f17(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C2(_) => {
                        Err::<Adt3, Option<Bytes>>(Some(f16(f17(Str::lit("runtime support: assign-place: ")), f17(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C3(_, _) => {
                        Err::<Adt3, Option<Bytes>>(Some(f16(f17(Str::lit("runtime support: assign-place: ")), f17(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C4(_, _, _) => {
                        Err::<Adt3, Option<Bytes>>(Some(f16(f17(Str::lit("runtime support: assign-place: ")), f17(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C5(r416, _) => {
                        let v11 = (*r416).clone();
                        Ok::<Adt3, Option<Bytes>>(v11)
                    }
                    Adt3::C6(r417, _) => {
                        let v13 = (*r417).clone();
                        Ok::<Adt3, Option<Bytes>>(v13)
                    }
                    Adt3::C7(_) => {
                        Err::<Adt3, Option<Bytes>>(Some(f16(f17(Str::lit("runtime support: assign-place: ")), f17(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C8(_, _) => {
                        Err::<Adt3, Option<Bytes>>(Some(f16(f17(Str::lit("runtime support: assign-place: ")), f17(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C9(_, _) => {
                        Err::<Adt3, Option<Bytes>>(Some(f16(f17(Str::lit("runtime support: assign-place: ")), f17(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C10(_, _, _) => {
                        Err::<Adt3, Option<Bytes>>(Some(f16(f17(Str::lit("runtime support: assign-place: ")), f17(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C11(_, _) => {
                        Err::<Adt3, Option<Bytes>>(Some(f16(f17(Str::lit("runtime support: assign-place: ")), f17(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C12(_, _, _) => {
                        Err::<Adt3, Option<Bytes>>(Some(f16(f17(Str::lit("runtime support: assign-place: ")), f17(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C13(_, _) => {
                        Err::<Adt3, Option<Bytes>>(Some(f16(f17(Str::lit("runtime support: assign-place: ")), f17(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C14(_, _, _) => {
                        Err::<Adt3, Option<Bytes>>(Some(f16(f17(Str::lit("runtime support: assign-place: ")), f17(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C15(_) => {
                        Err::<Adt3, Option<Bytes>>(Some(f16(f17(Str::lit("runtime support: assign-place: ")), f17(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C16(_, _, _) => {
                        Err::<Adt3, Option<Bytes>>(Some(f16(f17(Str::lit("runtime support: assign-place: ")), f17(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C17 => {
                        Err::<Adt3, Option<Bytes>>(Some(f16(f17(Str::lit("runtime support: assign-place: ")), f17(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C18(_) => {
                        Err::<Adt3, Option<Bytes>>(Some(f16(f17(Str::lit("runtime support: assign-place: ")), f17(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C19(_) => {
                        Err::<Adt3, Option<Bytes>>(Some(f16(f17(Str::lit("runtime support: assign-place: ")), f17(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C20(_, _, _) => {
                        Err::<Adt3, Option<Bytes>>(Some(f16(f17(Str::lit("runtime support: assign-place: ")), f17(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C21(_, _, _) => {
                        Err::<Adt3, Option<Bytes>>(Some(f16(f17(Str::lit("runtime support: assign-place: ")), f17(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C22(_) => {
                        Err::<Adt3, Option<Bytes>>(Some(f16(f17(Str::lit("runtime support: assign-place: ")), f17(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C23(_, _) => {
                        Err::<Adt3, Option<Bytes>>(Some(f16(f17(Str::lit("runtime support: assign-place: ")), f17(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C24(r418, _) => {
                        let v48 = (*r418).clone();
                        Ok::<Adt3, Option<Bytes>>(v48)
                    }
                    Adt3::C25(_, _, _) => {
                        Err::<Adt3, Option<Bytes>>(Some(f16(f17(Str::lit("runtime support: assign-place: ")), f17(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C26(r419) => {
                        let v53 = (*r419).clone();
                        Ok::<Adt3, Option<Bytes>>(v53)
                    }
                    Adt3::C27(_) => {
                        Err::<Adt3, Option<Bytes>>(Some(f16(f17(Str::lit("runtime support: assign-place: ")), f17(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C28(_, _, _, _) => {
                        Err::<Adt3, Option<Bytes>>(Some(f16(f17(Str::lit("runtime support: assign-place: ")), f17(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C29(_, _) => {
                        Err::<Adt3, Option<Bytes>>(Some(f16(f17(Str::lit("runtime support: assign-place: ")), f17(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C30 => {
                        Err::<Adt3, Option<Bytes>>(Some(f16(f17(Str::lit("runtime support: assign-place: ")), f17(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C31 => {
                        Err::<Adt3, Option<Bytes>>(Some(f16(f17(Str::lit("runtime support: assign-place: ")), f17(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C32(_, _) => {
                        Err::<Adt3, Option<Bytes>>(Some(f16(f17(Str::lit("runtime support: assign-place: ")), f17(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C33(_, _, _, _) => {
                        Err::<Adt3, Option<Bytes>>(Some(f16(f17(Str::lit("runtime support: assign-place: ")), f17(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C34(_, _, _, _) => {
                        Err::<Adt3, Option<Bytes>>(Some(f16(f17(Str::lit("runtime support: assign-place: ")), f17(Str::lit("an assignment to a value that is no place")))))
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

pub fn f64(v0: Adt14, v1: Adt16, v2: Adt3) -> R<Option<Bytes>> {
    let m422 = v2;
    match m422 {
        Adt3::C0(_) => {
            Ok(None::<Bytes>)
        }
        Adt3::C1(_) => {
            Ok(None::<Bytes>)
        }
        Adt3::C2(_) => {
            Ok(None::<Bytes>)
        }
        Adt3::C3(_, _) => {
            Ok(None::<Bytes>)
        }
        Adt3::C4(_, _, _) => {
            Ok(None::<Bytes>)
        }
        Adt3::C5(_, _) => {
            Ok(None::<Bytes>)
        }
        Adt3::C6(_, _) => {
            Ok(None::<Bytes>)
        }
        Adt3::C7(_) => {
            Ok(None::<Bytes>)
        }
        Adt3::C8(_, _) => {
            Ok(None::<Bytes>)
        }
        Adt3::C9(_, _) => {
            Ok(None::<Bytes>)
        }
        Adt3::C10(_, _, _) => {
            Ok(None::<Bytes>)
        }
        Adt3::C11(_, _) => {
            Ok(None::<Bytes>)
        }
        Adt3::C12(_, _, _) => {
            Ok(None::<Bytes>)
        }
        Adt3::C13(_, _) => {
            Ok(None::<Bytes>)
        }
        Adt3::C14(_, _, _) => {
            Ok(None::<Bytes>)
        }
        Adt3::C15(_) => {
            Ok(None::<Bytes>)
        }
        Adt3::C16(_, _, _) => {
            Ok(None::<Bytes>)
        }
        Adt3::C17 => {
            Ok(None::<Bytes>)
        }
        Adt3::C18(_) => {
            Ok(None::<Bytes>)
        }
        Adt3::C19(_) => {
            Ok(None::<Bytes>)
        }
        Adt3::C20(_, _, _) => {
            Ok(None::<Bytes>)
        }
        Adt3::C21(_, _, _) => {
            Ok(None::<Bytes>)
        }
        Adt3::C22(_) => {
            Ok(None::<Bytes>)
        }
        Adt3::C23(_, _) => {
            Ok(None::<Bytes>)
        }
        Adt3::C24(_, _) => {
            Ok(None::<Bytes>)
        }
        Adt3::C25(_, _, _) => {
            Ok(None::<Bytes>)
        }
        Adt3::C26(_) => {
            Ok(None::<Bytes>)
        }
        Adt3::C27(_) => {
            Ok(None::<Bytes>)
        }
        Adt3::C28(_, _, _, _) => {
            Ok(None::<Bytes>)
        }
        Adt3::C29(r424, r425) => {
            let v59 = (*r424).clone();
            let v60 = (*r425).clone();
            let m423 = f75(v0.clone(), v1, v59)?;
            match m423 {
                Ok(v62) => {
                    f13(v0, v62, v60)
                }
                Err(v61) => {
                    Ok(Some(v61))
                }
            }
        }
        Adt3::C30 => {
            Ok(None::<Bytes>)
        }
        Adt3::C31 => {
            Ok(None::<Bytes>)
        }
        Adt3::C32(_, _) => {
            Ok(None::<Bytes>)
        }
        Adt3::C33(_, _, _, _) => {
            Ok(None::<Bytes>)
        }
        Adt3::C34(_, _, _, _) => {
            Ok(None::<Bytes>)
        }
    }
}

pub fn f65(v0: Adt16, v1: List<Adt17>, v2: bool) -> Adt16 {
    Adt16::C0(match v0.clone() {
        Adt16::C0(h426, _, _, _) => {
            h426
        }
    }, {
        let a428 = v1;
        let a429 = match v0.clone() {
            Adt16::C0(_, h427, _, _) => {
                h427
            }
        };
        append_list(a428, a429)
    }, match v0.clone() {
        Adt16::C0(_, _, h430, _) => {
            h430
        }
    }, if match v0 {
        Adt16::C0(_, _, _, h431) => {
            h431
        }
    } {
        true
    } else {
        v2
    })
}

pub fn f66(v0: Adt14, v1: Adt4) -> R<Option<Bytes>> {
    let m432 = v1;
    match m432 {
        Adt4::C0 => {
            Ok(None::<Bytes>)
        }
        Adt4::C1(_) => {
            Ok(None::<Bytes>)
        }
        Adt4::C2(_) => {
            Ok(None::<Bytes>)
        }
        Adt4::C3(v4) => {
            f55(v0, v4)
        }
        Adt4::C4(r442) => {
            let v5 = (*r442).clone();
            f66(v0, v5)
        }
        Adt4::C5(v6, r443) => {
            let v7 = (*r443).clone();
            if f126(v6.clone()) {
                f66(v0, v7)
            } else {
                let m433 = f55(v0.clone(), v6.clone())?;
                match m433 {
                    None => {
                        let m434 = f127(v0.clone(), v6, v7.clone())?;
                        match m434 {
                            None => {
                                f66(v0, v7)
                            }
                            Some(v9) => {
                                Ok(Some(v9))
                            }
                        }
                    }
                    Some(v8) => {
                        Ok(Some(v8))
                    }
                }
            }
        }
        Adt4::C6(v10, v11, r444) => {
            let v12 = (*r444).clone();
            let m435 = f55(v0.clone(), v10.clone())?;
            match m435 {
                None => {
                    let m436 = f37(Str::lit("a pattern field name"), v11.clone())?;
                    match m436 {
                        None => {
                            let a438 = {
                                let a437 = v11.clone();
                                length_list(a437)
                            };
                            let a439 = f128(v12.clone())?;
                            if nat_eq(a438, a439) {
                                let m440 = f129(v0.clone(), v10, v11)?;
                                match m440 {
                                    None => {
                                        f66(v0, v12)
                                    }
                                    Some(v15) => {
                                        Ok(Some(v15))
                                    }
                                }
                            } else {
                                Ok(Some(f16(f17(Str::lit("runtime support: pattern-fields: ")), f17(Str::lit("a struct pattern with unequal names and patterns")))))
                            }
                        }
                        Some(v14) => {
                            Ok(Some(v14))
                        }
                    }
                }
                Some(v13) => {
                    Ok(Some(v13))
                }
            }
        }
        Adt4::C7(v16) => {
            f54(v16)
        }
        Adt4::C8 => {
            Ok(None::<Bytes>)
        }
        Adt4::C9(r445, r446) => {
            let v17 = (*r445).clone();
            let v18 = (*r446).clone();
            let m441 = f66(v0.clone(), v17)?;
            match m441 {
                None => {
                    f66(v0, v18)
                }
                Some(v19) => {
                    Ok(Some(v19))
                }
            }
        }
    }
}

pub fn f67(v0: List<Adt17>, v1: Adt4) -> R<Result<List<Adt17>, Bytes>> {
    let m447 = v1;
    match m447 {
        Adt4::C0 => {
            Ok(Ok::<List<Adt17>, Bytes>(v0))
        }
        Adt4::C1(v2) => {
            let m448 = f39(Str::lit("a pattern binding"), v2.clone())?;
            match m448 {
                None => {
                    let m449 = f52(v0.clone(), v2.clone());
                    match m449 {
                        None => {
                            Ok(Ok::<List<Adt17>, Bytes>(List::cons(Adt17::C0(v2, false, Adt2::C16), v0)))
                        }
                        Some(_) => {
                            Ok(Err::<List<Adt17>, Bytes>(f16(f17(Str::lit("runtime support: pattern-duplicate: ")), f16(f18(v2), f17(Str::lit(" is bound twice in one pattern"))))))
                        }
                    }
                }
                Some(v3) => {
                    Ok(Err::<List<Adt17>, Bytes>(v3))
                }
            }
        }
        Adt4::C2(v5) => {
            let m450 = f39(Str::lit("a pattern binding"), v5.clone())?;
            match m450 {
                None => {
                    let m451 = f52(v0.clone(), v5.clone());
                    match m451 {
                        None => {
                            Ok(Ok::<List<Adt17>, Bytes>(List::cons(Adt17::C0(v5, true, Adt2::C16), v0)))
                        }
                        Some(_) => {
                            Ok(Err::<List<Adt17>, Bytes>(f16(f17(Str::lit("runtime support: pattern-duplicate: ")), f16(f18(v5), f17(Str::lit(" is bound twice in one pattern"))))))
                        }
                    }
                }
                Some(v6) => {
                    Ok(Err::<List<Adt17>, Bytes>(v6))
                }
            }
        }
        Adt4::C3(_) => {
            Ok(Ok::<List<Adt17>, Bytes>(v0))
        }
        Adt4::C4(r453) => {
            let v9 = (*r453).clone();
            f67(v0, v9)
        }
        Adt4::C5(_, r454) => {
            let v11 = (*r454).clone();
            f67(v0, v11)
        }
        Adt4::C6(_, _, r455) => {
            let v14 = (*r455).clone();
            f67(v0, v14)
        }
        Adt4::C7(_) => {
            Ok(Ok::<List<Adt17>, Bytes>(v0))
        }
        Adt4::C8 => {
            Ok(Ok::<List<Adt17>, Bytes>(v0))
        }
        Adt4::C9(r456, r457) => {
            let v16 = (*r456).clone();
            let v17 = (*r457).clone();
            let m452 = f67(v0, v16)?;
            match m452 {
                Ok(v18) => {
                    f67(v18, v17)
                }
                Err(v19) => {
                    Ok(Err::<List<Adt17>, Bytes>(v19))
                }
            }
        }
    }
}

pub fn f68(v0: Adt3) -> bool {
    let m458 = v0;
    match m458 {
        Adt3::C0(_) => {
            false
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
        Adt3::C5(_, _) => {
            false
        }
        Adt3::C6(_, _) => {
            false
        }
        Adt3::C7(_) => {
            false
        }
        Adt3::C8(_, _) => {
            false
        }
        Adt3::C9(_, _) => {
            false
        }
        Adt3::C10(_, _, _) => {
            false
        }
        Adt3::C11(_, _) => {
            false
        }
        Adt3::C12(_, _, _) => {
            false
        }
        Adt3::C13(_, _) => {
            false
        }
        Adt3::C14(_, _, _) => {
            false
        }
        Adt3::C15(_) => {
            false
        }
        Adt3::C16(_, _, _) => {
            false
        }
        Adt3::C17 => {
            false
        }
        Adt3::C18(_) => {
            false
        }
        Adt3::C19(_) => {
            false
        }
        Adt3::C20(_, _, _) => {
            false
        }
        Adt3::C21(_, _, _) => {
            false
        }
        Adt3::C22(_) => {
            false
        }
        Adt3::C23(_, _) => {
            false
        }
        Adt3::C24(_, _) => {
            false
        }
        Adt3::C25(_, _, _) => {
            false
        }
        Adt3::C26(_) => {
            false
        }
        Adt3::C27(_) => {
            false
        }
        Adt3::C28(_, _, _, _) => {
            false
        }
        Adt3::C29(_, _) => {
            false
        }
        Adt3::C30 => {
            true
        }
        Adt3::C31 => {
            false
        }
        Adt3::C32(_, _) => {
            false
        }
        Adt3::C33(_, _, _, _) => {
            false
        }
        Adt3::C34(_, _, _, _) => {
            false
        }
    }
}

pub fn f69(v0: Adt14, v1: List<Bytes>) -> Option<List<Bytes>> {
    let m459 = v1;
    match m459.uncons() {
        None => {
            None::<List<Bytes>>
        }
        Some((v2, v3)) => {
            let m460 = v3;
            match m460.uncons() {
                None => {
                    let m461 = f107(v0, v2);
                    match m461 {
                        None => {
                            None::<List<Bytes>>
                        }
                        Some(v4) => {
                            Some(match v4 {
                                Adt11::C0(_, h462, _) => {
                                    h462
                                }
                            })
                        }
                    }
                }
                Some((v5, v6)) => {
                    let m463 = v6;
                    match m463.uncons() {
                        None => {
                            let m464 = f108(v0, v2);
                            match m464 {
                                None => {
                                    None::<List<Bytes>>
                                }
                                Some(v7) => {
                                    f130(match v7 {
                                        Adt12::C0(_, h465, _) => {
                                            h465
                                        }
                                    }, v5)
                                }
                            }
                        }
                        Some((_, _)) => {
                            None::<List<Bytes>>
                        }
                    }
                }
            }
        }
    }
}

pub fn f70(v0: List<Bytes>, v1: List<Bytes>, v2: u64) -> R<bool> {
    let a468 = {
        let a466 = v0.clone();
        length_list(a466)
    };
    let a469 = {
        let a467 = v1.clone();
        length_list(a467)
    };
    if nat_eq(a468, a469) {
        let a471 = {
            let a470 = v0.clone();
            length_list(a470)
        };
        let a472 = v2;
        if nat_eq(a471, a472) {
            f86(Fn8::F131(v1.clone(), v0), true, v1)
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f71(v0: Adt2) -> bool {
    let m473 = v0;
    match m473 {
        Adt2::C0 => {
            false
        }
        Adt2::C1 => {
            false
        }
        Adt2::C2 => {
            false
        }
        Adt2::C3 => {
            false
        }
        Adt2::C4 => {
            false
        }
        Adt2::C5 => {
            false
        }
        Adt2::C6(_) => {
            false
        }
        Adt2::C7(_) => {
            false
        }
        Adt2::C8(_) => {
            false
        }
        Adt2::C9(_, _) => {
            false
        }
        Adt2::C10(_, _) => {
            false
        }
        Adt2::C11 => {
            false
        }
        Adt2::C12 => {
            false
        }
        Adt2::C13 => {
            false
        }
        Adt2::C14 => {
            false
        }
        Adt2::C15(_) => {
            false
        }
        Adt2::C16 => {
            false
        }
        Adt2::C17 => {
            true
        }
        Adt2::C18 => {
            false
        }
        Adt2::C19(_, _) => {
            false
        }
    }
}

pub fn f72(v0: Str, v1: Adt2) -> Option<Bytes> {
    let m474 = v1;
    match m474 {
        Adt2::C0 => {
            None::<Bytes>
        }
        Adt2::C1 => {
            None::<Bytes>
        }
        Adt2::C2 => {
            None::<Bytes>
        }
        Adt2::C3 => {
            None::<Bytes>
        }
        Adt2::C4 => {
            None::<Bytes>
        }
        Adt2::C5 => {
            None::<Bytes>
        }
        Adt2::C6(_) => {
            Some(f16(f17(Str::lit("runtime support: type-scope: ")), f16(f17(v0), f17(Str::lit(" names a generic parameter, which the printer writes without the generics in scope")))))
        }
        Adt2::C7(r477) => {
            let v3 = (*r477).clone();
            f72(v0, v3)
        }
        Adt2::C8(r478) => {
            let v4 = (*r478).clone();
            f72(v0, v4)
        }
        Adt2::C9(r479, r480) => {
            let v5 = (*r479).clone();
            let v6 = (*r480).clone();
            let m475 = f72(v0.clone(), v5);
            match m475 {
                None => {
                    f72(v0, v6)
                }
                Some(v7) => {
                    Some(v7)
                }
            }
        }
        Adt2::C10(_, r481) => {
            let v9 = (*r481).clone();
            f72(v0, v9)
        }
        Adt2::C11 => {
            None::<Bytes>
        }
        Adt2::C12 => {
            None::<Bytes>
        }
        Adt2::C13 => {
            None::<Bytes>
        }
        Adt2::C14 => {
            None::<Bytes>
        }
        Adt2::C15(r482) => {
            let v10 = (*r482).clone();
            f72(v0, v10)
        }
        Adt2::C16 => {
            None::<Bytes>
        }
        Adt2::C17 => {
            None::<Bytes>
        }
        Adt2::C18 => {
            None::<Bytes>
        }
        Adt2::C19(r483, r484) => {
            let v11 = (*r483).clone();
            let v12 = (*r484).clone();
            let m476 = f72(v0.clone(), v11);
            match m476 {
                None => {
                    f72(v0, v12)
                }
                Some(v13) => {
                    Some(v13)
                }
            }
        }
    }
}

pub fn f73(v0: Bytes) -> Option<u64> {
    let a485 = v0.clone();
    let a486 = f17(Str::lit("parse"));
    if equal(a485, a486) {
        Some(1u64)
    } else {
        let a487 = v0;
        let a488 = f17(Str::lit("collect"));
        if equal(a487, a488) {
            Some(1u64)
        } else {
            None::<u64>
        }
    }
}

pub fn f74(v0: Bytes) -> u64 {
    let a489 = v0.clone();
    let a490 = f17(Str::lit("parse"));
    if equal(a489, a490) {
        0u64
    } else {
        let a491 = v0;
        let a492 = f17(Str::lit("collect"));
        let _ = equal(a491, a492);
        0u64
    }
}

pub fn f75(v0: Adt14, p508: Adt16, p509: Adt3) -> R<Result<Adt16, Bytes>> {
    let m507 = (p508, p509);
    {
        let mut m507 = m507;
        loop {
            let step = {
                let (v1, v2) = m507;
                let m493 = v2;
                match m493 {
                    Adt3::C0(_) => {
                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Ok::<Adt16, Bytes>(v1)))
                    }
                    Adt3::C1(_) => {
                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Ok::<Adt16, Bytes>(v1)))
                    }
                    Adt3::C2(_) => {
                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Ok::<Adt16, Bytes>(v1)))
                    }
                    Adt3::C3(_, _) => {
                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Ok::<Adt16, Bytes>(v1)))
                    }
                    Adt3::C4(_, _, _) => {
                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Ok::<Adt16, Bytes>(v1)))
                    }
                    Adt3::C5(_, _) => {
                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Ok::<Adt16, Bytes>(v1)))
                    }
                    Adt3::C6(_, _) => {
                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Ok::<Adt16, Bytes>(v1)))
                    }
                    Adt3::C7(_) => {
                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Ok::<Adt16, Bytes>(v1)))
                    }
                    Adt3::C8(_, _) => {
                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Ok::<Adt16, Bytes>(v1)))
                    }
                    Adt3::C9(_, _) => {
                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Ok::<Adt16, Bytes>(v1)))
                    }
                    Adt3::C10(_, _, _) => {
                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Ok::<Adt16, Bytes>(v1)))
                    }
                    Adt3::C11(_, _) => {
                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Ok::<Adt16, Bytes>(v1)))
                    }
                    Adt3::C12(_, _, _) => {
                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Ok::<Adt16, Bytes>(v1)))
                    }
                    Adt3::C13(_, _) => {
                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Ok::<Adt16, Bytes>(v1)))
                    }
                    Adt3::C14(_, _, _) => {
                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Ok::<Adt16, Bytes>(v1)))
                    }
                    Adt3::C15(_) => {
                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Ok::<Adt16, Bytes>(v1)))
                    }
                    Adt3::C16(_, _, _) => {
                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Ok::<Adt16, Bytes>(v1)))
                    }
                    Adt3::C17 => {
                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Ok::<Adt16, Bytes>(v1)))
                    }
                    Adt3::C18(_) => {
                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Ok::<Adt16, Bytes>(v1)))
                    }
                    Adt3::C19(_) => {
                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Ok::<Adt16, Bytes>(v1)))
                    }
                    Adt3::C20(_, _, _) => {
                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Ok::<Adt16, Bytes>(v1)))
                    }
                    Adt3::C21(_, _, _) => {
                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Ok::<Adt16, Bytes>(v1)))
                    }
                    Adt3::C22(_) => {
                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Ok::<Adt16, Bytes>(v1)))
                    }
                    Adt3::C23(_, _) => {
                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Ok::<Adt16, Bytes>(v1)))
                    }
                    Adt3::C24(_, _) => {
                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Ok::<Adt16, Bytes>(v1)))
                    }
                    Adt3::C25(_, _, _) => {
                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Ok::<Adt16, Bytes>(v1)))
                    }
                    Adt3::C26(_) => {
                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Ok::<Adt16, Bytes>(v1)))
                    }
                    Adt3::C27(_) => {
                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Ok::<Adt16, Bytes>(v1)))
                    }
                    Adt3::C28(_, _, _, _) => {
                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Ok::<Adt16, Bytes>(v1)))
                    }
                    Adt3::C29(_, _) => {
                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Ok::<Adt16, Bytes>(v1)))
                    }
                    Adt3::C30 => {
                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Ok::<Adt16, Bytes>(v1)))
                    }
                    Adt3::C31 => {
                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Ok::<Adt16, Bytes>(v1)))
                    }
                    Adt3::C32(r503, r504) => {
                        let v61 = (*r503).clone();
                        let v62 = (*r504).clone();
                        let m494 = f13(v0.clone(), v1.clone(), v61)?;
                        match m494 {
                            None => {
                                Ok::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>((v1, v62))
                            }
                            Some(v63) => {
                                Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Err::<Adt16, Bytes>(v63)))
                            }
                        }
                    }
                    Adt3::C33(v64, v65, r505, r506) => {
                        let v66 = (*r505).clone();
                        let v67 = (*r506).clone();
                        let m499 = if f71(v65.clone()) {
                            None::<Bytes>
                        } else {
                            f35(f12(v0.clone(), v65.clone(), {
                                let a496 = match v1.clone() {
                                    Adt16::C0(h495, _, _, _) => {
                                        h495
                                    }
                                };
                                length_list(a496)
                            }, {
                                let a498 = f71(match v1.clone() {
                                    Adt16::C0(_, _, h497, _) => {
                                        h497
                                    }
                                });
                                bool_not(a498)
                            })?, f72(Str::lit("a `let` type"), v65))
                        };
                        match m499 {
                            None => {
                                let m500 = f13(v0.clone(), v1.clone(), v66)?;
                                match m500 {
                                    None => {
                                        let m501 = f67(List::<Adt17>::nil(), v64.clone())?;
                                        match m501 {
                                            Ok(v71) => {
                                                let m502 = f66(v0.clone(), v64)?;
                                                match m502 {
                                                    None => {
                                                        Ok::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>((f65(v1, v71, false), v67))
                                                    }
                                                    Some(v72) => {
                                                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Err::<Adt16, Bytes>(v72)))
                                                    }
                                                }
                                            }
                                            Err(v70) => {
                                                Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Err::<Adt16, Bytes>(v70)))
                                            }
                                        }
                                    }
                                    Some(v69) => {
                                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Err::<Adt16, Bytes>(v69)))
                                    }
                                }
                            }
                            Some(v68) => {
                                Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Err::<Adt16, Bytes>(v68)))
                            }
                        }
                    }
                    Adt3::C34(_, _, _, _) => {
                        Err::<(Adt16, Adt3), R<Result<Adt16, Bytes>>>(Ok(Ok::<Adt16, Bytes>(v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m507 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f76(v0: Fn6, p513: Adt18, p514: List<Adt5>) -> R<Adt18> {
    let m512 = (p513, p514);
    {
        let mut m512 = m512;
        loop {
            let step = {
                let (v1, v2) = m512;
                let m510 = v2;
                match m510.uncons() {
                    None => {
                        Err::<(Adt18, List<Adt5>), R<Adt18>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt18, List<Adt5>), R<Adt18>>(({
                            let c511: Fn6 = v0.clone();
                            c511.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m512 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f77(v0: Adt18, v1: Adt5) -> R<Adt18> {
    let m516 = match v0.clone() {
        Adt18::C0(_, h515) => {
            h515
        }
    };
    match m516 {
        None => {
            if f53(match v0.clone() {
                Adt18::C0(h517, _) => {
                    h517
                }
            }, match v1.clone() {
                Adt5::C0(h518, _, _, _, _) => {
                    h518
                }
            })? {
                Ok(Adt18::C0(match v0 {
                    Adt18::C0(h519, _) => {
                        h519
                    }
                }, Some(match v1 {
                    Adt5::C0(h520, _, _, _, _) => {
                        h520
                    }
                })))
            } else {
                Ok(Adt18::C0(List::cons(match v1 {
                    Adt5::C0(h521, _, _, _, _) => {
                        h521
                    }
                }, match v0 {
                    Adt18::C0(h522, _) => {
                        h522
                    }
                }), None::<Bytes>))
            }
        }
        Some(_) => {
            Ok(v0)
        }
    }
}

pub fn f78(v0: Adt14, v1: Adt5, v2: u64, v3: bool) -> R<Option<Bytes>> {
    let m524 = f39(Str::lit("a method name"), match v1.clone() {
        Adt5::C0(h523, _, _, _, _) => {
            h523
        }
    })?;
    match m524 {
        None => {
            if if match v1.clone() {
                Adt5::C0(_, _, h525, _, _) => {
                    h525
                }
            } {
                let a527 = match v1.clone() {
                    Adt5::C0(_, h526, _, _, _) => {
                        h526
                    }
                };
                bool_not(a527)
            } else {
                false
            } {
                Ok(Some(f16(f17(Str::lit("runtime support: function-receiver: ")), f16(f18(match v1 {
                    Adt5::C0(h528, _, _, _, _) => {
                        h528
                    }
                }), f17(Str::lit(" is exclusive without a receiver"))))))
            } else {
                let m530 = f132(v0.clone(), match v1.clone() {
                    Adt5::C0(_, _, _, h529, _) => {
                        h529
                    }
                }, v2, v3)?;
                match m530 {
                    None => {
                        f12(v0, match v1 {
                            Adt5::C0(_, _, _, _, h531) => {
                                h531
                            }
                        }, v2, v3)
                    }
                    Some(v5) => {
                        Ok(Some(v5))
                    }
                }
            }
        }
        Some(v4) => {
            Ok(Some(v4))
        }
    }
}

pub fn f79(v0: Fn7, p535: Option<Bytes>, p536: List<Adt7>) -> R<Option<Bytes>> {
    let m534 = (p535, p536);
    {
        let mut m534 = m534;
        loop {
            let step = {
                let (v1, v2) = m534;
                let m532 = v2;
                match m532.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Adt7>), R<Option<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Adt7>), R<Option<Bytes>>>(({
                            let c533: Fn7 = v0.clone();
                            c533.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m534 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f80(v0: Adt14, v1: Option<Bytes>, v2: Adt7) -> R<Option<Bytes>> {
    let m537 = v1;
    match m537 {
        None => {
            let m539 = f39(Str::lit("a generic parameter"), match v2.clone() {
                Adt7::C0(h538, _) => {
                    h538
                }
            })?;
            match m539 {
                None => {
                    f91(Fn10::F133(v2.clone(), v0), None::<Bytes>, match v2 {
                        Adt7::C0(_, h540) => {
                            h540
                        }
                    })
                }
                Some(v4) => {
                    Ok(Some(v4))
                }
            }
        }
        Some(v3) => {
            Ok(Some(v3))
        }
    }
}

pub fn f81(v0: Adt14, v1: Bytes) -> Option<List<Adt5>> {
    let m541 = f134(v0, v1.clone());
    match m541 {
        None => {
            let m543 = f135(v1.clone());
            match m543 {
                None => {
                    None::<List<Adt5>>
                }
                Some(v3) => {
                    Some(List::cons(Adt5::C0(v3, true, f136(v1.clone()), List::<Adt6>::nil(), if f136(v1) {
                        Adt2::C0
                    } else {
                        Adt2::C5
                    }), List::<Adt5>::nil()))
                }
            }
        }
        Some(v2) => {
            Some(match v2 {
                Adt10::C0(_, h542) => {
                    h542
                }
            })
        }
    }
}

pub fn f82(v0: List<Adt8>, v1: Bytes, v2: Option<Bytes>, v3: Adt5) -> R<Option<Bytes>> {
    let m544 = v2;
    match m544 {
        None => {
            let v5: u64 = f137(v0.clone(), match v3.clone() {
                Adt5::C0(h545, _, _, _, _) => {
                    h545
                }
            })?;
            let a546 = v5;
            let a547 = 0u64;
            if nat_eq(a546, a547) {
                Ok(Some(f16(f17(Str::lit("runtime support: impl-complete: ")), f16(f17(Str::lit("`impl ")), f16(v1, f16(f17(Str::lit("` does not state ")), f18(match v3 {
                    Adt5::C0(h548, _, _, _, _) => {
                        h548
                    }
                })))))))
            } else {
                let a549 = v5;
                let a550 = 1u64;
                if nat_eq(a549, a550) {
                    let m552 = f138(v0, match v3.clone() {
                        Adt5::C0(h551, _, _, _, _) => {
                            h551
                        }
                    });
                    match m552 {
                        None => {
                            Ok(None::<Bytes>)
                        }
                        Some(v6) => {
                            let a554 = f139(match v6 {
                                Adt8::C0(h553, _, _, _) => {
                                    h553
                                }
                            })?;
                            let a555 = f139(v3.clone())?;
                            if equal(a554, a555) {
                                Ok(None::<Bytes>)
                            } else {
                                Ok(Some(f16(f17(Str::lit("runtime support: impl-signature: ")), f16(f17(Str::lit("`impl ")), f16(v1, f16(f17(Str::lit("` states ")), f16(f18(match v3 {
                                    Adt5::C0(h556, _, _, _, _) => {
                                        h556
                                    }
                                }), f17(Str::lit(" with a signature other than the trait's")))))))))
                            }
                        }
                    }
                } else {
                    Ok(Some(f16(f17(Str::lit("runtime support: impl-complete: ")), f16(f17(Str::lit("`impl ")), f16(v1, f16(f17(Str::lit("` states ")), f16(f18(match v3 {
                        Adt5::C0(h557, _, _, _, _) => {
                            h557
                        }
                    }), f17(Str::lit(" twice")))))))))
                }
            }
        }
        Some(v4) => {
            Ok(Some(v4))
        }
    }
}

pub fn f83(v0: List<Adt7>, v1: Adt14, v2: List<Adt5>, v3: Bytes, v4: Adt2, v5: Option<Bytes>, v6: Adt8) -> R<Option<Bytes>> {
    let m558 = v5;
    match m558 {
        None => {
            let a562 = f140(v2, {
                let m561 = match v6.clone() {
                    Adt8::C0(h560, _, _, _) => {
                        h560
                    }
                };
                match m561 {
                    Adt5::C0(h559, _, _, _, _) => {
                        h559
                    }
                }
            });
            if bool_not(a562) {
                Ok(Some(f16(f17(Str::lit("runtime support: impl-extra: ")), f16(f17(Str::lit("`impl ")), f16(v3, f16(f17(Str::lit("` states ")), f16(f18({
                    let m565 = match v6 {
                        Adt8::C0(h564, _, _, _) => {
                            h564
                        }
                    };
                    match m565 {
                        Adt5::C0(h563, _, _, _, _) => {
                            h563
                        }
                    }
                }), f17(Str::lit(", which the trait does not")))))))))
            } else {
                if match v6.clone() {
                    Adt8::C0(_, _, h566, _) => {
                        h566
                    }
                } {
                    Ok(Some(f16(f17(Str::lit("runtime support: impl-visibility: ")), f16(f18({
                        let m569 = match v6 {
                            Adt8::C0(h568, _, _, _) => {
                                h568
                            }
                        };
                        match m569 {
                            Adt5::C0(h567, _, _, _, _) => {
                                h567
                            }
                        }
                    }), f16(f17(Str::lit(" of `impl ")), f16(v3, f17(Str::lit("` is public; a trait method has the trait's visibility"))))))))
                } else {
                    f84(v1, v0, v4, v6)
                }
            }
        }
        Some(v7) => {
            Ok(Some(v7))
        }
    }
}

pub fn f84(v0: Adt14, v1: List<Adt7>, v2: Adt2, v3: Adt8) -> R<Option<Bytes>> {
    let m571 = f21(v0.clone(), match v3.clone() {
        Adt8::C0(_, h570, _, _) => {
            h570
        }
    })?;
    match m571 {
        None => {
            let v5: List<Adt7> = {
                let a573 = v1;
                let a574 = match v3.clone() {
                    Adt8::C0(_, h572, _, _) => {
                        h572
                    }
                };
                append_list(a573, a574)
            };
            let v6: u64 = {
                let a575 = v5.clone();
                length_list(a575)
            };
            let v7: Adt5 = match v3.clone() {
                Adt8::C0(h576, _, _, _) => {
                    h576
                }
            };
            let m578 = f78(v0.clone(), v7.clone(), v6, {
                let a577 = f71(v2.clone());
                bool_not(a577)
            })?;
            match m578 {
                None => {
                    let m583 = if match v7.clone() {
                        Adt5::C0(_, h579, _, _, _) => {
                            h579
                        }
                    } {
                        if f71(v2.clone()) {
                            Err::<List<Adt17>, Bytes>(f16(f17(Str::lit("runtime support: function-receiver: ")), f16(f18(match v7.clone() {
                                Adt5::C0(h580, _, _, _, _) => {
                                    h580
                                }
                            }), f17(Str::lit(" takes `&self` outside an impl")))))
                        } else {
                            Ok::<List<Adt17>, Bytes>(List::cons(Adt17::C0(f17(Str::lit("self")), match v7.clone() {
                                Adt5::C0(_, _, h581, _, _) => {
                                    h581
                                }
                            }, if match v7.clone() {
                                Adt5::C0(_, _, h582, _, _) => {
                                    h582
                                }
                            } {
                                Adt2::C8(std::rc::Rc::new(v2.clone()))
                            } else {
                                Adt2::C7(std::rc::Rc::new(v2.clone()))
                            }), List::<Adt17>::nil()))
                        }
                    } else {
                        Ok::<List<Adt17>, Bytes>(List::<Adt17>::nil())
                    };
                    match m583 {
                        Ok(v10) => {
                            let m586 = f141(Ok::<List<Adt17>, Bytes>(v10), match v7.clone() {
                                Adt5::C0(_, _, _, h584, _) => {
                                    h584
                                }
                            }, match v7 {
                                Adt5::C0(h585, _, _, _, _) => {
                                    h585
                                }
                            });
                            match m586 {
                                Ok(v12) => {
                                    f64(v0, Adt16::C0(v5, v12, v2, false), match v3 {
                                        Adt8::C0(_, _, _, h587) => {
                                            h587
                                        }
                                    })
                                }
                                Err(v11) => {
                                    Ok(Some(v11))
                                }
                            }
                        }
                        Err(v9) => {
                            Ok(Some(v9))
                        }
                    }
                }
                Some(v8) => {
                    Ok(Some(v8))
                }
            }
        }
        Some(v4) => {
            Ok(Some(v4))
        }
    }
}

pub fn f85(v0: Adt14, v1: Bytes, v2: Adt1) -> R<Option<Bytes>> {
    let m588 = v2;
    match m588 {
        Adt1::C0(_, _) => {
            Ok(Some(f16(f17(Str::lit("runtime support: module-member: ")), f16(f18(v1), f17(Str::lit(" holds an item that is neither a defined function nor a function"))))))
        }
        Adt1::C1(_, _) => {
            Ok(Some(f16(f17(Str::lit("runtime support: module-member: ")), f16(f18(v1), f17(Str::lit(" holds an item that is neither a defined function nor a function"))))))
        }
        Adt1::C2(_, _, _) => {
            Ok(Some(f16(f17(Str::lit("runtime support: module-member: ")), f16(f18(v1), f17(Str::lit(" holds an item that is neither a defined function nor a function"))))))
        }
        Adt1::C3(_, _, _) => {
            Ok(Some(f16(f17(Str::lit("runtime support: module-member: ")), f16(f18(v1), f17(Str::lit(" holds an item that is neither a defined function nor a function"))))))
        }
        Adt1::C4(_, _) => {
            Ok(Some(f16(f17(Str::lit("runtime support: module-member: ")), f16(f18(v1), f17(Str::lit(" holds an item that is neither a defined function nor a function"))))))
        }
        Adt1::C5(_, _, _, _) => {
            Ok(Some(f16(f17(Str::lit("runtime support: module-member: ")), f16(f18(v1), f17(Str::lit(" holds an item that is neither a defined function nor a function"))))))
        }
        Adt1::C6(v4) => {
            f25(v0, v4)
        }
        Adt1::C7(_, _, _, _) => {
            Ok(Some(f16(f17(Str::lit("runtime support: module-member: ")), f16(f18(v1), f17(Str::lit(" holds an item that is neither a defined function nor a function"))))))
        }
        Adt1::C8(_, _, _, _, _) => {
            Ok(Some(f16(f17(Str::lit("runtime support: module-member: ")), f16(f18(v1), f17(Str::lit(" holds an item that is neither a defined function nor a function"))))))
        }
        Adt1::C9(_, _, _, _) => {
            Ok(Some(f16(f17(Str::lit("runtime support: module-member: ")), f16(f18(v1), f17(Str::lit(" holds an item that is neither a defined function nor a function"))))))
        }
        Adt1::C10(v3) => {
            Ok(f28(v3))
        }
        Adt1::C11(_, _) => {
            Ok(Some(f16(f17(Str::lit("runtime support: module-member: ")), f16(f18(v1), f17(Str::lit(" holds an item that is neither a defined function nor a function"))))))
        }
        Adt1::C12(_, _) => {
            Ok(Some(f16(f17(Str::lit("runtime support: module-member: ")), f16(f18(v1), f17(Str::lit(" holds an item that is neither a defined function nor a function"))))))
        }
    }
}

pub fn f86(v0: Fn8, p592: bool, p593: List<Bytes>) -> R<bool> {
    let m591 = (p592, p593);
    {
        let mut m591 = m591;
        loop {
            let step = {
                let (v1, v2) = m591;
                let m589 = v2;
                match m589.uncons() {
                    None => {
                        Err::<(bool, List<Bytes>), R<bool>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<Bytes>), R<bool>>(({
                            let c590: Fn8 = v0.clone();
                            c590.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m591 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f87(v0: bool, v1: Bytes) -> bool {
    if v0 {
        true
    } else {
        let a595 = {
            let a594 = v1;
            length_bytes(a594)
        };
        let a596 = 0u64;
        nat_eq(a595, a596)
    }
}

pub fn f88(v0: Fn9, p600: Bytes, p601: List<Bytes>) -> Bytes {
    let m599 = (p600, p601);
    {
        let mut m599 = m599;
        loop {
            let step = {
                let (v1, v2) = m599;
                let m597 = v2;
                match m597.uncons() {
                    None => {
                        Err::<(Bytes, List<Bytes>), Bytes>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Bytes, List<Bytes>), Bytes>(({
                            let c598: Fn9 = v0.clone();
                            c598.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m599 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f89(_: Bytes, v1: Bytes) -> Bytes {
    v1
}

pub fn f90(v0: List<Bytes>, v1: Bytes) -> R<Bytes> {
    let m602 = f91(Fn10::F142(v1), None::<Bytes>, v0)?;
    match m602 {
        None => {
            Ok(Bytes::lit(&[]))
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f91(v0: Fn10, p606: Option<Bytes>, p607: List<Bytes>) -> R<Option<Bytes>> {
    let m605 = (p606, p607);
    {
        let mut m605 = m605;
        loop {
            let step = {
                let (v1, v2) = m605;
                let m603 = v2;
                match m603.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Bytes>), R<Option<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Bytes>), R<Option<Bytes>>>(({
                            let c604: Fn10 = v0.clone();
                            c604.apply(v1, v3)?
                        }, v4))
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

pub fn f92(v0: Str, v1: Option<Bytes>, v2: Bytes) -> R<Option<Bytes>> {
    let m608 = v1;
    match m608 {
        None => {
            if if f32(v2.clone(), f17(Str::lit("Self"))) {
                true
            } else {
                if f32(v2.clone(), f17(Str::lit("self"))) {
                    true
                } else {
                    if f32(v2.clone(), f17(Str::lit("crate"))) {
                        true
                    } else {
                        f32(v2.clone(), f17(Str::lit("super")))
                    }
                }
            } {
                Ok(None::<Bytes>)
            } else {
                f39(v0, v2)
            }
        }
        Some(v3) => {
            Ok(Some(v3))
        }
    }
}

pub fn f93(v0: Str, v1: Option<Bytes>, v2: Bytes) -> R<Option<Bytes>> {
    let m609 = v1;
    match m609 {
        None => {
            f39(v0, v2)
        }
        Some(v3) => {
            Ok(Some(v3))
        }
    }
}

pub fn f94(v0: Bytes) -> R<bool> {
    let a611 = 0u64;
    let a612 = {
        let a610 = v0.clone();
        length_bytes(a610)
    };
    if nat_lt(a611, a612) {
        let a616 = {
            let (h613, _) = f143(Fn15::F144(v0.clone()), {
                let a614 = v0.clone();
                length_bytes(a614)
            }, 0u64)?;
            h613
        };
        let a617 = {
            let a615 = v0;
            length_bytes(a615)
        };
        Ok(nat_eq(a616, a617))
    } else {
        Ok(false)
    }
}

pub fn f95() -> List<Bytes> {
    List::cons(f17(Str::lit("abstract")), List::cons(f17(Str::lit("as")), List::cons(f17(Str::lit("async")), List::cons(f17(Str::lit("await")), List::cons(f17(Str::lit("become")), List::cons(f17(Str::lit("box")), List::cons(f17(Str::lit("break")), List::cons(f17(Str::lit("const")), List::cons(f17(Str::lit("continue")), List::cons(f17(Str::lit("crate")), List::cons(f17(Str::lit("do")), List::cons(f17(Str::lit("dyn")), List::cons(f17(Str::lit("else")), List::cons(f17(Str::lit("enum")), List::cons(f17(Str::lit("extern")), List::cons(f17(Str::lit("false")), List::cons(f17(Str::lit("final")), List::cons(f17(Str::lit("fn")), List::cons(f17(Str::lit("for")), List::cons(f17(Str::lit("if")), List::cons(f17(Str::lit("impl")), List::cons(f17(Str::lit("in")), List::cons(f17(Str::lit("let")), List::cons(f17(Str::lit("loop")), List::cons(f17(Str::lit("macro")), List::cons(f17(Str::lit("match")), List::cons(f17(Str::lit("mod")), List::cons(f17(Str::lit("move")), List::cons(f17(Str::lit("mut")), List::cons(f17(Str::lit("override")), List::cons(f17(Str::lit("priv")), List::cons(f17(Str::lit("pub")), List::cons(f17(Str::lit("ref")), List::cons(f17(Str::lit("return")), List::cons(f17(Str::lit("self")), List::cons(f17(Str::lit("Self")), List::cons(f17(Str::lit("static")), List::cons(f17(Str::lit("struct")), List::cons(f17(Str::lit("super")), List::cons(f17(Str::lit("trait")), List::cons(f17(Str::lit("true")), List::cons(f17(Str::lit("try")), List::cons(f17(Str::lit("type")), List::cons(f17(Str::lit("typeof")), List::cons(f17(Str::lit("unsafe")), List::cons(f17(Str::lit("unsized")), List::cons(f17(Str::lit("use")), List::cons(f17(Str::lit("virtual")), List::cons(f17(Str::lit("where")), List::cons(f17(Str::lit("while")), List::cons(f17(Str::lit("yield")), List::cons(f17(Str::lit("_")), List::<Bytes>::nil()))))))))))))))))))))))))))))))))))))))))))))))))))))
}

pub fn f96(v0: Str, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m618 = v1;
    match m618 {
        None => {
            if if f32(v2.clone(), f17(Str::lit("Clone"))) {
                true
            } else {
                if f32(v2.clone(), f17(Str::lit("Copy"))) {
                    true
                } else {
                    if f32(v2.clone(), f17(Str::lit("Debug"))) {
                        true
                    } else {
                        if f32(v2.clone(), f17(Str::lit("Eq"))) {
                            true
                        } else {
                            f32(v2.clone(), f17(Str::lit("PartialEq")))
                        }
                    }
                }
            } {
                None::<Bytes>
            } else {
                Some(f16(f17(Str::lit("runtime support: derive: ")), f16(f17(v0), f16(f17(Str::lit(" derives ")), f16(f18(v2), f17(Str::lit(", which is not in the closed table")))))))
            }
        }
        Some(v3) => {
            Some(v3)
        }
    }
}

pub fn f97(v0: Fn11, p622: Adt14, p623: List<Adt5>) -> Adt14 {
    let m621 = (p622, p623);
    {
        let mut m621 = m621;
        loop {
            let step = {
                let (v1, v2) = m621;
                let m619 = v2;
                match m619.uncons() {
                    None => {
                        Err::<(Adt14, List<Adt5>), Adt14>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt14, List<Adt5>), Adt14>(({
                            let c620: Fn11 = v0.clone();
                            c620.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m621 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f98(v0: Adt14, v1: Adt5) -> Adt14 {
    Adt14::C0(match v0.clone() {
        Adt14::C0(h624, _, _, _, _, _, _, _, _, _) => {
            h624
        }
    }, match v0.clone() {
        Adt14::C0(_, h625, _, _, _, _, _, _, _, _) => {
            h625
        }
    }, match v0.clone() {
        Adt14::C0(_, _, h626, _, _, _, _, _, _, _) => {
            h626
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, h627, _, _, _, _, _, _) => {
            h627
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, h628, _, _, _, _, _) => {
            h628
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, h629, _, _, _, _) => {
            h629
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, h630, _, _, _) => {
            h630
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, h631, _, _) => {
            h631
        }
    }, {
        let a636 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, h632, _) => {
                h632
            }
        };
        let a637 = List::cons(Adt13::C0(match v1.clone() {
            Adt5::C0(h633, _, _, _, _) => {
                h633
            }
        }, {
            let a635 = match v1 {
                Adt5::C0(_, _, _, h634, _) => {
                    h634
                }
            };
            length_list(a635)
        }), List::<Adt13>::nil());
        append_list(a636, a637)
    }, match v0 {
        Adt14::C0(_, _, _, _, _, _, _, _, _, h638) => {
            h638
        }
    })
}

pub fn f99(v0: Adt2) -> Option<Bytes> {
    let m639 = v0;
    match m639 {
        Adt2::C0 => {
            None::<Bytes>
        }
        Adt2::C1 => {
            None::<Bytes>
        }
        Adt2::C2 => {
            None::<Bytes>
        }
        Adt2::C3 => {
            None::<Bytes>
        }
        Adt2::C4 => {
            None::<Bytes>
        }
        Adt2::C5 => {
            None::<Bytes>
        }
        Adt2::C6(_) => {
            None::<Bytes>
        }
        Adt2::C7(_) => {
            None::<Bytes>
        }
        Adt2::C8(_) => {
            None::<Bytes>
        }
        Adt2::C9(_, _) => {
            None::<Bytes>
        }
        Adt2::C10(v6, _) => {
            Some(v6)
        }
        Adt2::C11 => {
            None::<Bytes>
        }
        Adt2::C12 => {
            None::<Bytes>
        }
        Adt2::C13 => {
            None::<Bytes>
        }
        Adt2::C14 => {
            None::<Bytes>
        }
        Adt2::C15(_) => {
            None::<Bytes>
        }
        Adt2::C16 => {
            None::<Bytes>
        }
        Adt2::C17 => {
            None::<Bytes>
        }
        Adt2::C18 => {
            None::<Bytes>
        }
        Adt2::C19(_, _) => {
            None::<Bytes>
        }
    }
}

pub fn f100(v0: Adt14, v1: Bytes, v2: List<Adt8>) -> Adt14 {
    f101(Fn12::F145(v1), v0, v2)
}

pub fn f101(v0: Fn12, p643: Adt14, p644: List<Adt8>) -> Adt14 {
    let m642 = (p643, p644);
    {
        let mut m642 = m642;
        loop {
            let step = {
                let (v1, v2) = m642;
                let m640 = v2;
                match m640.uncons() {
                    None => {
                        Err::<(Adt14, List<Adt8>), Adt14>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt14, List<Adt8>), Adt14>(({
                            let c641: Fn12 = v0.clone();
                            c641.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m642 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f102(v0: Adt14, v1: Adt8) -> Adt14 {
    Adt14::C0(match v0.clone() {
        Adt14::C0(h645, _, _, _, _, _, _, _, _, _) => {
            h645
        }
    }, match v0.clone() {
        Adt14::C0(_, h646, _, _, _, _, _, _, _, _) => {
            h646
        }
    }, match v0.clone() {
        Adt14::C0(_, _, h647, _, _, _, _, _, _, _) => {
            h647
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, h648, _, _, _, _, _, _) => {
            h648
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, h649, _, _, _, _, _) => {
            h649
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, h650, _, _, _, _) => {
            h650
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, h651, _, _, _) => {
            h651
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, h652, _, _) => {
            h652
        }
    }, {
        let a661 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, h653, _) => {
                h653
            }
        };
        let a662 = List::cons(Adt13::C0({
            let m656 = match v1.clone() {
                Adt8::C0(h655, _, _, _) => {
                    h655
                }
            };
            match m656 {
                Adt5::C0(h654, _, _, _, _) => {
                    h654
                }
            }
        }, {
            let a660 = {
                let m659 = match v1 {
                    Adt8::C0(h658, _, _, _) => {
                        h658
                    }
                };
                match m659 {
                    Adt5::C0(_, _, _, h657, _) => {
                        h657
                    }
                }
            };
            length_list(a660)
        }), List::<Adt13>::nil());
        append_list(a661, a662)
    }, match v0 {
        Adt14::C0(_, _, _, _, _, _, _, _, _, h663) => {
            h663
        }
    })
}

pub fn f103(v0: u64) -> R<Bytes> {
    let v1: Bytes = f146(Str::lit("0"));
    let v2: Bytes = f147({
        let a700 = f148(v0, 10000000000000000000u64);
        let a701 = {
            let a698 = f148(v0, 1000000000000000000u64);
            let a699 = {
                let a696 = f148(v0, 100000000000000000u64);
                let a697 = {
                    let a694 = f148(v0, 10000000000000000u64);
                    let a695 = {
                        let a692 = f148(v0, 1000000000000000u64);
                        let a693 = {
                            let a690 = f148(v0, 100000000000000u64);
                            let a691 = {
                                let a688 = f148(v0, 10000000000000u64);
                                let a689 = {
                                    let a686 = f148(v0, 1000000000000u64);
                                    let a687 = {
                                        let a684 = f148(v0, 100000000000u64);
                                        let a685 = {
                                            let a682 = f148(v0, 10000000000u64);
                                            let a683 = {
                                                let a680 = f148(v0, 1000000000u64);
                                                let a681 = {
                                                    let a678 = f148(v0, 100000000u64);
                                                    let a679 = {
                                                        let a676 = f148(v0, 10000000u64);
                                                        let a677 = {
                                                            let a674 = f148(v0, 1000000u64);
                                                            let a675 = {
                                                                let a672 = f148(v0, 100000u64);
                                                                let a673 = {
                                                                    let a670 = f148(v0, 10000u64);
                                                                    let a671 = {
                                                                        let a668 = f148(v0, 1000u64);
                                                                        let a669 = {
                                                                            let a666 = f148(v0, 100u64);
                                                                            let a667 = {
                                                                                let a664 = f148(v0, 10u64);
                                                                                let a665 = f148(v0, 1u64);
                                                                                append_bytes(a664, a665)
                                                                            };
                                                                            append_bytes(a666, a667)
                                                                        };
                                                                        append_bytes(a668, a669)
                                                                    };
                                                                    append_bytes(a670, a671)
                                                                };
                                                                append_bytes(a672, a673)
                                                            };
                                                            append_bytes(a674, a675)
                                                        };
                                                        append_bytes(a676, a677)
                                                    };
                                                    append_bytes(a678, a679)
                                                };
                                                append_bytes(a680, a681)
                                            };
                                            append_bytes(a682, a683)
                                        };
                                        append_bytes(a684, a685)
                                    };
                                    append_bytes(a686, a687)
                                };
                                append_bytes(a688, a689)
                            };
                            append_bytes(a690, a691)
                        };
                        append_bytes(a692, a693)
                    };
                    append_bytes(a694, a695)
                };
                append_bytes(a696, a697)
            };
            append_bytes(a698, a699)
        };
        append_bytes(a700, a701)
    })?;
    let a702 = v0;
    let a703 = 0u64;
    if nat_eq(a702, a703) {
        Ok(v1)
    } else {
        Ok(v2)
    }
}

pub fn f104(v0: Fn13, p707: Adt18, p708: List<Adt9>) -> R<Adt18> {
    let m706 = (p707, p708);
    {
        let mut m706 = m706;
        loop {
            let step = {
                let (v1, v2) = m706;
                let m704 = v2;
                match m704.uncons() {
                    None => {
                        Err::<(Adt18, List<Adt9>), R<Adt18>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt18, List<Adt9>), R<Adt18>>(({
                            let c705: Fn13 = v0.clone();
                            c705.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m706 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f105(v0: Bytes, v1: Adt18, v2: Adt9) -> R<Adt18> {
    let m710 = match v1.clone() {
        Adt18::C0(_, h709) => {
            h709
        }
    };
    match m710 {
        None => {
            let m711 = f39(Str::lit("a variant name"), f149(v2.clone()))?;
            match m711 {
                None => {
                    let m714 = {
                        let m713 = v2.clone();
                        match m713 {
                            Adt9::C0(_) => {
                                None::<Bytes>
                            }
                            Adt9::C1(_, _) => {
                                None::<Bytes>
                            }
                            Adt9::C2(_, v9, _) => {
                                f37(Str::lit("a record variant field name"), v9)?
                            }
                        }
                    };
                    match m714 {
                        None => {
                            if f53(match v1.clone() {
                                Adt18::C0(h716, _) => {
                                    h716
                                }
                            }, f149(v2.clone()))? {
                                Ok(Adt18::C0(match v1 {
                                    Adt18::C0(h717, _) => {
                                        h717
                                    }
                                }, Some(f16(f17(Str::lit("runtime support: variant-name: ")), f16(f18(v0), f16(f17(Str::lit(" declares ")), f16(f18(f149(v2)), f17(Str::lit(" twice")))))))))
                            } else {
                                Ok(Adt18::C0(List::cons(f149(v2), match v1 {
                                    Adt18::C0(h718, _) => {
                                        h718
                                    }
                                }), None::<Bytes>))
                            }
                        }
                        Some(v11) => {
                            Ok(Adt18::C0(match v1 {
                                Adt18::C0(h715, _) => {
                                    h715
                                }
                            }, Some(v11)))
                        }
                    }
                }
                Some(v4) => {
                    Ok(Adt18::C0(match v1 {
                        Adt18::C0(h712, _) => {
                            h712
                        }
                    }, Some(v4)))
                }
            }
        }
        Some(_) => {
            Ok(v1)
        }
    }
}

pub fn f106(v0: Adt18, v1: Adt9) -> R<Adt18> {
    let m720 = match v0.clone() {
        Adt18::C0(_, h719) => {
            h719
        }
    };
    match m720 {
        None => {
            if f53(match v0.clone() {
                Adt18::C0(h721, _) => {
                    h721
                }
            }, f149(v1.clone()))? {
                Ok(Adt18::C0(match v0 {
                    Adt18::C0(h722, _) => {
                        h722
                    }
                }, Some(f149(v1))))
            } else {
                Ok(Adt18::C0(List::cons(f149(v1), match v0 {
                    Adt18::C0(h723, _) => {
                        h723
                    }
                }), None::<Bytes>))
            }
        }
        Some(_) => {
            Ok(v0)
        }
    }
}

pub fn f107(v0: Adt14, v1: Bytes) -> Option<Adt11> {
    f150(Fn16::F151(v1), None::<Adt11>, match v0 {
        Adt14::C0(_, _, _, h724, _, _, _, _, _, _) => {
            h724
        }
    })
}

pub fn f108(v0: Adt14, v1: Bytes) -> Option<Adt12> {
    f152(Fn17::F153(v1), None::<Adt12>, match v0 {
        Adt14::C0(_, _, _, _, h725, _, _, _, _, _) => {
            h725
        }
    })
}

pub fn f109(v0: Adt14, v1: Bytes) -> Option<Adt13> {
    f154(Fn18::F155(v1), None::<Adt13>, match v0 {
        Adt14::C0(_, _, _, _, _, h726, _, _, _, _) => {
            h726
        }
    })
}

pub fn f110(v0: Bytes) -> Option<u64> {
    let a727 = v0.clone();
    let a728 = f17(Str::lit("Result"));
    if equal(a727, a728) {
        Some(2u64)
    } else {
        let a729 = v0.clone();
        let a730 = f17(Str::lit("Option"));
        if equal(a729, a730) {
            Some(1u64)
        } else {
            let a731 = v0.clone();
            let a732 = f17(Str::lit("AtomicU64"));
            if equal(a731, a732) {
                Some(0u64)
            } else {
                let a733 = v0.clone();
                let a734 = f17(Str::lit("Rc"));
                if equal(a733, a734) {
                    Some(1u64)
                } else {
                    let a735 = v0.clone();
                    let a736 = f17(Str::lit("Vec"));
                    if equal(a735, a736) {
                        Some(1u64)
                    } else {
                        let a737 = v0;
                        let a738 = f17(Str::lit("String"));
                        if equal(a737, a738) {
                            Some(0u64)
                        } else {
                            None::<u64>
                        }
                    }
                }
            }
        }
    }
}

pub fn f111(v0: Bytes) -> bool {
    let a739 = v0.clone();
    let a740 = f17(Str::lit("Rc"));
    if equal(a739, a740) {
        true
    } else {
        let a741 = v0.clone();
        let a742 = f17(Str::lit("Vec"));
        if equal(a741, a742) {
            true
        } else {
            let a743 = v0;
            let a744 = f17(Str::lit("String"));
            equal(a743, a744)
        }
    }
}

pub fn f112(v0: Fn14, p748: Option<Adt17>, p749: List<Adt17>) -> Option<Adt17> {
    let m747 = (p748, p749);
    {
        let mut m747 = m747;
        loop {
            let step = {
                let (v1, v2) = m747;
                let m745 = v2;
                match m745.uncons() {
                    None => {
                        Err::<(Option<Adt17>, List<Adt17>), Option<Adt17>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Adt17>, List<Adt17>), Option<Adt17>>(({
                            let c746: Fn14 = v0.clone();
                            c746.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m747 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f113(v0: Bytes, v1: Option<Adt17>, v2: Adt17) -> Option<Adt17> {
    let m750 = v1;
    match m750 {
        None => {
            if f32(match v2.clone() {
                Adt17::C0(h751, _, _) => {
                    h751
                }
            }, v0) {
                Some(v2)
            } else {
                None::<Adt17>
            }
        }
        Some(v3) => {
            Some(v3)
        }
    }
}

pub fn f114(v0: Bytes, v1: bool, v2: Bytes) -> bool {
    if v1 {
        true
    } else {
        f32(v2, v0)
    }
}

pub fn f115(v0: Adt14, v1: List<Bytes>) -> R<Option<Bytes>> {
    let v2: Bytes = f156(v1.clone());
    let m752 = f108(v0.clone(), v2.clone());
    match m752 {
        None => {
            if if f53(match v0.clone() {
                Adt14::C0(_, _, _, _, _, _, _, _, _, h756) => {
                    h756
                }
            }, v2.clone())? {
                true
            } else {
                let m757 = f107(v0.clone(), v2.clone());
                if match m757 {
                    None => {
                        false
                    }
                    Some(_) => {
                        true
                    }
                } {
                    true
                } else {
                    if f53(match v0 {
                        Adt14::C0(_, _, _, _, _, _, h758, _, _, _) => {
                            h758
                        }
                    }, v2.clone())? {
                        true
                    } else {
                        if f159(v2.clone()) {
                            true
                        } else {
                            if f32(v2.clone(), f17(Str::lit("None"))) {
                                true
                            } else {
                                if f32(v2.clone(), f17(Str::lit("Self"))) {
                                    true
                                } else {
                                    f32(v2.clone(), f17(Str::lit("usize")))
                                }
                            }
                        }
                    }
                }
            } {
                Ok(None::<Bytes>)
            } else {
                Ok(Some(f16(f17(Str::lit("runtime support: path-name: ")), f16(f18(f34(v1, Str::lit("::"))?), f16(f17(Str::lit(" starts at ")), f16(f18(v2), f17(Str::lit(", which no `use` or item declares"))))))))
            }
        }
        Some(v3) => {
            let m753 = f157(v1.clone(), 1u64);
            match m753 {
                None => {
                    Ok(Some(f16(f17(Str::lit("runtime support: variant-name: ")), f16(f18(f34(v1, Str::lit("::"))?), f16(f17(Str::lit(" is not a variant of ")), f18(v2))))))
                }
                Some(v4) => {
                    let m755 = f158(match v3 {
                        Adt12::C0(_, h754, _) => {
                            h754
                        }
                    }, v4);
                    match m755 {
                        None => {
                            Ok(Some(f16(f17(Str::lit("runtime support: variant-name: ")), f16(f18(f34(v1, Str::lit("::"))?), f16(f17(Str::lit(" is not a variant of ")), f18(v2))))))
                        }
                        Some(_) => {
                            Ok(None::<Bytes>)
                        }
                    }
                }
            }
        }
    }
}

pub fn f116(v0: Adt14, v1: List<Bytes>) -> bool {
    if match v0 {
        Adt14::C0(h759, _, _, _, _, _, _, _, _, _) => {
            h759
        }
    } {
        let m760 = v1;
        match m760.uncons() {
            None => {
                false
            }
            Some((v2, v3)) => {
                let m761 = v3;
                match m761.uncons() {
                    None => {
                        false
                    }
                    Some((v4, v5)) => {
                        let m762 = v5;
                        match m762.uncons() {
                            None => {
                                f160(v2, v4)
                            }
                            Some((_, _)) => {
                                false
                            }
                        }
                    }
                }
            }
        }
    } else {
        false
    }
}

pub fn f117(v0: Adt14, v1: List<Bytes>) -> R<Option<u64>> {
    let m763 = v1;
    match m763.uncons() {
        None => {
            Ok(None::<u64>)
        }
        Some((v2, v3)) => {
            let m764 = v3;
            match m764.uncons() {
                None => {
                    let m765 = f161(v0.clone(), v2.clone());
                    match m765 {
                        None => {
                            let m767 = f162(v2.clone());
                            match m767 {
                                None => {
                                    Ok(f163(v0, v2))
                                }
                                Some(v5) => {
                                    Ok(Some(v5))
                                }
                            }
                        }
                        Some(v4) => {
                            Ok(Some(match v4 {
                                Adt13::C0(_, h766) => {
                                    h766
                                }
                            }))
                        }
                    }
                }
                Some((v6, v7)) => {
                    let m768 = v7;
                    match m768.uncons() {
                        None => {
                            let m769 = f164(v2.clone(), v6.clone());
                            match m769 {
                                None => {
                                    let m770 = f165(v0.clone(), v2.clone(), v6.clone())?;
                                    match m770 {
                                        None => {
                                            Ok(f166(v0, v2, v6))
                                        }
                                        Some(v9) => {
                                            Ok(Some(v9))
                                        }
                                    }
                                }
                                Some(v8) => {
                                    Ok(Some(v8))
                                }
                            }
                        }
                        Some((_, _)) => {
                            Ok(None::<u64>)
                        }
                    }
                }
            }
        }
    }
}

pub fn f118(v0: Bytes) -> Option<u64> {
    let a771 = v0.clone();
    let a772 = f17(Str::lit("fetch_add"));
    if equal(a771, a772) {
        Some(2u64)
    } else {
        let a773 = v0.clone();
        let a774 = f17(Str::lit("load"));
        if equal(a773, a774) {
            Some(1u64)
        } else {
            let a775 = v0.clone();
            let a776 = f17(Str::lit("cmp"));
            if equal(a775, a776) {
                Some(1u64)
            } else {
                let a777 = v0.clone();
                let a778 = f17(Str::lit("len"));
                if equal(a777, a778) {
                    Some(0u64)
                } else {
                    let a779 = v0.clone();
                    let a780 = f17(Str::lit("is_empty"));
                    if equal(a779, a780) {
                        Some(0u64)
                    } else {
                        let a781 = v0.clone();
                        let a782 = f17(Str::lit("chars"));
                        if equal(a781, a782) {
                            Some(0u64)
                        } else {
                            let a783 = v0.clone();
                            let a784 = f17(Str::lit("next"));
                            if equal(a783, a784) {
                                Some(0u64)
                            } else {
                                let a785 = v0.clone();
                                let a786 = f17(Str::lit("min"));
                                if equal(a785, a786) {
                                    Some(1u64)
                                } else {
                                    let a787 = v0.clone();
                                    let a788 = f17(Str::lit("max"));
                                    if equal(a787, a788) {
                                        Some(1u64)
                                    } else {
                                        let a789 = v0.clone();
                                        let a790 = f17(Str::lit("push"));
                                        if equal(a789, a790) {
                                            Some(1u64)
                                        } else {
                                            let a791 = v0.clone();
                                            let a792 = f17(Str::lit("pop"));
                                            if equal(a791, a792) {
                                                Some(0u64)
                                            } else {
                                                let a793 = v0.clone();
                                                let a794 = f17(Str::lit("truncate"));
                                                if equal(a793, a794) {
                                                    Some(1u64)
                                                } else {
                                                    let a795 = v0.clone();
                                                    let a796 = f17(Str::lit("push_str"));
                                                    if equal(a795, a796) {
                                                        Some(1u64)
                                                    } else {
                                                        let a797 = v0.clone();
                                                        let a798 = f17(Str::lit("extend_from_slice"));
                                                        if equal(a797, a798) {
                                                            Some(1u64)
                                                        } else {
                                                            let a799 = v0.clone();
                                                            let a800 = f17(Str::lit("iter"));
                                                            if equal(a799, a800) {
                                                                Some(0u64)
                                                            } else {
                                                                let a801 = v0.clone();
                                                                let a802 = f17(Str::lit("rev"));
                                                                if equal(a801, a802) {
                                                                    Some(0u64)
                                                                } else {
                                                                    let a803 = v0.clone();
                                                                    let a804 = f17(Str::lit("get"));
                                                                    if equal(a803, a804) {
                                                                        Some(1u64)
                                                                    } else {
                                                                        let a805 = v0.clone();
                                                                        let a806 = f17(Str::lit("copied"));
                                                                        if equal(a805, a806) {
                                                                            Some(0u64)
                                                                        } else {
                                                                            let a807 = v0.clone();
                                                                            let a808 = f17(Str::lit("ok"));
                                                                            if equal(a807, a808) {
                                                                                Some(0u64)
                                                                            } else {
                                                                                let a809 = v0.clone();
                                                                                let a810 = f17(Str::lit("clone"));
                                                                                if equal(a809, a810) {
                                                                                    Some(0u64)
                                                                                } else {
                                                                                    let a811 = v0.clone();
                                                                                    let a812 = f17(Str::lit("to_string"));
                                                                                    if equal(a811, a812) {
                                                                                        Some(0u64)
                                                                                    } else {
                                                                                        let a813 = v0.clone();
                                                                                        let a814 = f17(Str::lit("as_bytes"));
                                                                                        if equal(a813, a814) {
                                                                                            Some(0u64)
                                                                                        } else {
                                                                                            let a815 = v0.clone();
                                                                                            let a816 = f17(Str::lit("strip_prefix"));
                                                                                            if equal(a815, a816) {
                                                                                                Some(1u64)
                                                                                            } else {
                                                                                                let a817 = v0.clone();
                                                                                                let a818 = f17(Str::lit("starts_with"));
                                                                                                if equal(a817, a818) {
                                                                                                    Some(1u64)
                                                                                                } else {
                                                                                                    let a819 = v0.clone();
                                                                                                    let a820 = f17(Str::lit("split"));
                                                                                                    if equal(a819, a820) {
                                                                                                        Some(1u64)
                                                                                                    } else {
                                                                                                        let a821 = v0.clone();
                                                                                                        let a822 = f17(Str::lit("collect"));
                                                                                                        if equal(a821, a822) {
                                                                                                            Some(0u64)
                                                                                                        } else {
                                                                                                            let a823 = v0;
                                                                                                            let a824 = f17(Str::lit("is_ascii_digit"));
                                                                                                            if equal(a823, a824) {
                                                                                                                Some(0u64)
                                                                                                            } else {
                                                                                                                None::<u64>
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
    }
}

pub fn f119(v0: Bytes) -> bool {
    let a825 = v0.clone();
    let a826 = f17(Str::lit("push"));
    if equal(a825, a826) {
        true
    } else {
        let a827 = v0.clone();
        let a828 = f17(Str::lit("pop"));
        if equal(a827, a828) {
            true
        } else {
            let a829 = v0.clone();
            let a830 = f17(Str::lit("truncate"));
            if equal(a829, a830) {
                true
            } else {
                let a831 = v0.clone();
                let a832 = f17(Str::lit("push_str"));
                if equal(a831, a832) {
                    true
                } else {
                    let a833 = v0.clone();
                    let a834 = f17(Str::lit("extend_from_slice"));
                    if equal(a833, a834) {
                        true
                    } else {
                        let a835 = v0.clone();
                        let a836 = f17(Str::lit("to_string"));
                        if equal(a835, a836) {
                            true
                        } else {
                            let a837 = v0;
                            let a838 = f17(Str::lit("collect"));
                            equal(a837, a838)
                        }
                    }
                }
            }
        }
    }
}

pub fn f120(v0: Adt14, v1: Bytes) -> List<u64> {
    f167(Fn19::F168(v1), List::<u64>::nil(), match v0 {
        Adt14::C0(_, _, _, _, _, _, _, _, h839, _) => {
            h839
        }
    })
}

pub fn f121(v0: List<u64>, v1: u64) -> bool {
    f169(Fn20::F170(v1), false, v0)
}

pub fn f122(v0: Adt14, v1: Adt16, v2: Adt3, v3: Bytes) -> R<Option<Bytes>> {
    let m840 = f171(f172(v1.clone(), v2));
    match m840 {
        Adt2::C0 => {
            Ok(None::<Bytes>)
        }
        Adt2::C1 => {
            Ok(None::<Bytes>)
        }
        Adt2::C2 => {
            Ok(None::<Bytes>)
        }
        Adt2::C3 => {
            Ok(None::<Bytes>)
        }
        Adt2::C4 => {
            Ok(None::<Bytes>)
        }
        Adt2::C5 => {
            Ok(None::<Bytes>)
        }
        Adt2::C6(v4) => {
            if f173(v0, f174(match v1 {
                Adt16::C0(h841, _, _, _) => {
                    h841
                }
            }, v4), v3.clone())? {
                Ok(None::<Bytes>)
            } else {
                Ok(Some(f16(f17(Str::lit("runtime support: method-bound: ")), f16(f18(v3), f17(Str::lit(" is called on a parameter whose bounds do not provide it"))))))
            }
        }
        Adt2::C7(_) => {
            Ok(None::<Bytes>)
        }
        Adt2::C8(_) => {
            Ok(None::<Bytes>)
        }
        Adt2::C9(_, _) => {
            Ok(None::<Bytes>)
        }
        Adt2::C10(_, _) => {
            Ok(None::<Bytes>)
        }
        Adt2::C11 => {
            Ok(None::<Bytes>)
        }
        Adt2::C12 => {
            Ok(None::<Bytes>)
        }
        Adt2::C13 => {
            Ok(None::<Bytes>)
        }
        Adt2::C14 => {
            Ok(None::<Bytes>)
        }
        Adt2::C15(_) => {
            Ok(None::<Bytes>)
        }
        Adt2::C16 => {
            Ok(None::<Bytes>)
        }
        Adt2::C17 => {
            Ok(None::<Bytes>)
        }
        Adt2::C18 => {
            Ok(None::<Bytes>)
        }
        Adt2::C19(_, _) => {
            Ok(None::<Bytes>)
        }
    }
}

pub fn f123(v0: List<u64>) -> R<Bytes> {
    f175(Fn21::F176, f17(Str::lit("")), v0)
}

pub fn f124(v0: Adt3) -> R<bool> {
    let m842 = v0;
    match m842 {
        Adt3::C0(_) => {
            Ok(false)
        }
        Adt3::C1(_) => {
            Ok(false)
        }
        Adt3::C2(_) => {
            Ok(false)
        }
        Adt3::C3(_, _) => {
            Ok(false)
        }
        Adt3::C4(r873, _, _) => {
            let v6 = (*r873).clone();
            let a843 = 13u64;
            let a844 = f177(v6.clone());
            if nat_le(a843, a844) {
                f124(v6)
            } else {
                Ok(false)
            }
        }
        Adt3::C5(r874, _) => {
            let v9 = (*r874).clone();
            let a845 = 13u64;
            let a846 = f177(v9.clone());
            if nat_le(a845, a846) {
                f124(v9)
            } else {
                Ok(false)
            }
        }
        Adt3::C6(r875, _) => {
            let v11 = (*r875).clone();
            let a847 = 13u64;
            let a848 = f177(v11.clone());
            if nat_le(a847, a848) {
                f124(v11)
            } else {
                Ok(false)
            }
        }
        Adt3::C7(r876) => {
            let v13 = (*r876).clone();
            let a849 = 12u64;
            let a850 = f177(v13.clone());
            if nat_le(a849, a850) {
                f124(v13)
            } else {
                Ok(false)
            }
        }
        Adt3::C8(r877, r878) => {
            let v14 = (*r877).clone();
            let v15 = (*r878).clone();
            let a851 = 6u64;
            let a852 = f177(v14.clone());
            if if nat_le(a851, a852) {
                f124(v14)?
            } else {
                false
            } {
                Ok(true)
            } else {
                let a853 = 6u64;
                let a854 = f177(v15.clone());
                if nat_le(a853, a854) {
                    f124(v15)
                } else {
                    Ok(false)
                }
            }
        }
        Adt3::C9(_, _) => {
            Ok(false)
        }
        Adt3::C10(_, _, _) => {
            Ok(false)
        }
        Adt3::C11(r879, r880) => {
            let v21 = (*r879).clone();
            let v22 = (*r880).clone();
            if f124(v21)? {
                Ok(true)
            } else {
                f124(v22)
            }
        }
        Adt3::C12(_, _, _) => {
            Ok(false)
        }
        Adt3::C13(_, _) => {
            Ok(false)
        }
        Adt3::C14(_, _, _) => {
            Ok(false)
        }
        Adt3::C15(_) => {
            Ok(false)
        }
        Adt3::C16(_, _, _) => {
            Ok(false)
        }
        Adt3::C17 => {
            Ok(false)
        }
        Adt3::C18(r881) => {
            let v35 = (*r881).clone();
            f124(v35)
        }
        Adt3::C19(_) => {
            Ok(false)
        }
        Adt3::C20(_, _, _) => {
            Ok(true)
        }
        Adt3::C21(v40, r882, r883) => {
            let v41 = (*r882).clone();
            let v42 = (*r883).clone();
            if if f178(v40, v41.clone()) {
                false
            } else {
                f124(v41)?
            } {
                Ok(true)
            } else {
                let a857 = {
                    let a855 = f179(v40);
                    let a856 = 1u64;
                    nat_add(a855, a856)?
                };
                let a858 = f177(v42.clone());
                if nat_le(a857, a858) {
                    f124(v42)
                } else {
                    Ok(false)
                }
            }
        }
        Adt3::C22(r884) => {
            let v43 = (*r884).clone();
            let a859 = 12u64;
            let a860 = f177(v43.clone());
            if nat_le(a859, a860) {
                f124(v43)
            } else {
                Ok(false)
            }
        }
        Adt3::C23(r885, _) => {
            let v44 = (*r885).clone();
            let a861 = 11u64;
            let a862 = f177(v44.clone());
            if nat_le(a861, a862) {
                f124(v44)
            } else {
                Ok(false)
            }
        }
        Adt3::C24(r886, _) => {
            let v46 = (*r886).clone();
            let a863 = 13u64;
            let a864 = f177(v46.clone());
            if nat_le(a863, a864) {
                f124(v46)
            } else {
                Ok(false)
            }
        }
        Adt3::C25(r887, _, _) => {
            let v48 = (*r887).clone();
            let a865 = 13u64;
            let a866 = f177(v48.clone());
            if nat_le(a865, a866) {
                f124(v48)
            } else {
                Ok(false)
            }
        }
        Adt3::C26(r888) => {
            let v51 = (*r888).clone();
            let a867 = 12u64;
            let a868 = f177(v51.clone());
            if nat_le(a867, a868) {
                f124(v51)
            } else {
                Ok(false)
            }
        }
        Adt3::C27(r889) => {
            let v52 = (*r889).clone();
            let a869 = 12u64;
            let a870 = f177(v52.clone());
            if nat_le(a869, a870) {
                f124(v52)
            } else {
                Ok(false)
            }
        }
        Adt3::C28(r890, _, _, _) => {
            let v53 = (*r890).clone();
            let a871 = 13u64;
            let a872 = f177(v53.clone());
            if nat_le(a871, a872) {
                f124(v53)
            } else {
                Ok(false)
            }
        }
        Adt3::C29(_, _) => {
            Ok(false)
        }
        Adt3::C30 => {
            Ok(false)
        }
        Adt3::C31 => {
            Ok(false)
        }
        Adt3::C32(_, _) => {
            Ok(false)
        }
        Adt3::C33(_, _, _, _) => {
            Ok(false)
        }
        Adt3::C34(_, _, _, _) => {
            Ok(false)
        }
    }
}

pub fn f125(v0: Adt2) -> bool {
    let m891 = v0;
    match m891 {
        Adt2::C0 => {
            false
        }
        Adt2::C1 => {
            false
        }
        Adt2::C2 => {
            false
        }
        Adt2::C3 => {
            false
        }
        Adt2::C4 => {
            false
        }
        Adt2::C5 => {
            false
        }
        Adt2::C6(_) => {
            false
        }
        Adt2::C7(_) => {
            false
        }
        Adt2::C8(_) => {
            true
        }
        Adt2::C9(_, _) => {
            false
        }
        Adt2::C10(_, _) => {
            false
        }
        Adt2::C11 => {
            false
        }
        Adt2::C12 => {
            false
        }
        Adt2::C13 => {
            false
        }
        Adt2::C14 => {
            false
        }
        Adt2::C15(_) => {
            false
        }
        Adt2::C16 => {
            false
        }
        Adt2::C17 => {
            false
        }
        Adt2::C18 => {
            false
        }
        Adt2::C19(_, _) => {
            false
        }
    }
}

pub fn f126(v0: List<Bytes>) -> bool {
    let m892 = v0;
    match m892.uncons() {
        None => {
            false
        }
        Some((v1, v2)) => {
            let m893 = v2;
            match m893.uncons() {
                None => {
                    let m894 = f162(v1);
                    match m894 {
                        None => {
                            false
                        }
                        Some(_) => {
                            true
                        }
                    }
                }
                Some((_, _)) => {
                    false
                }
            }
        }
    }
}

pub fn f127(v0: Adt14, v1: List<Bytes>, v2: Adt4) -> R<Option<Bytes>> {
    let m895 = f157(v1.clone(), 1u64);
    match m895 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v3) => {
            let m896 = f108(v0, f156(v1.clone()));
            match m896 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v4) => {
                    let m898 = f158(match v4 {
                        Adt12::C0(_, h897, _) => {
                            h897
                        }
                    }, v3);
                    match m898 {
                        None => {
                            Ok(Some(f16(f17(Str::lit("runtime support: pattern-fields: ")), f16(f18(f34(v1, Str::lit("::"))?), f16(f17(Str::lit(" is matched with ")), f16(f47(f128(v2)?)?, f17(Str::lit(" patterns"))))))))
                        }
                        Some(v5) => {
                            let a899 = f180(v5)?;
                            let a900 = f128(v2.clone())?;
                            if nat_eq(a899, a900) {
                                Ok(None::<Bytes>)
                            } else {
                                Ok(Some(f16(f17(Str::lit("runtime support: pattern-fields: ")), f16(f18(f34(v1, Str::lit("::"))?), f16(f17(Str::lit(" is matched with ")), f16(f47(f128(v2)?)?, f17(Str::lit(" patterns"))))))))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f128(v0: Adt4) -> R<u64> {
    let m901 = v0;
    match m901 {
        Adt4::C0 => {
            Ok(0u64)
        }
        Adt4::C1(_) => {
            Ok(0u64)
        }
        Adt4::C2(_) => {
            Ok(0u64)
        }
        Adt4::C3(_) => {
            Ok(0u64)
        }
        Adt4::C4(_) => {
            Ok(0u64)
        }
        Adt4::C5(_, _) => {
            Ok(0u64)
        }
        Adt4::C6(_, _, _) => {
            Ok(0u64)
        }
        Adt4::C7(_) => {
            Ok(0u64)
        }
        Adt4::C8 => {
            Ok(0u64)
        }
        Adt4::C9(_, r904) => {
            let v12 = (*r904).clone();
            let a902 = f128(v12)?;
            let a903 = 1u64;
            nat_add(a902, a903)
        }
    }
}

pub fn f129(v0: Adt14, v1: List<Bytes>, v2: List<Bytes>) -> R<Option<Bytes>> {
    let m905 = f69(v0, v1.clone());
    match m905 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v3) => {
            f91(Fn10::F181(v3, v1), None::<Bytes>, v2)
        }
    }
}

pub fn f130(v0: List<Adt9>, v1: Bytes) -> Option<List<Bytes>> {
    let m906 = f158(v0, v1);
    match m906 {
        None => {
            None::<List<Bytes>>
        }
        Some(v2) => {
            let m907 = v2;
            match m907 {
                Adt9::C0(_) => {
                    None::<List<Bytes>>
                }
                Adt9::C1(_, _) => {
                    None::<List<Bytes>>
                }
                Adt9::C2(_, v7, _) => {
                    Some(v7)
                }
            }
        }
    }
}

pub fn f131(v0: List<Bytes>, v1: List<Bytes>, v2: bool, v3: Bytes) -> R<bool> {
    if v2 {
        let a908 = f182(v1, v3.clone())?;
        let a909 = f182(v0, v3)?;
        Ok(nat_eq(a908, a909))
    } else {
        Ok(false)
    }
}

pub fn f132(v0: Adt14, v1: List<Adt6>, v2: u64, v3: bool) -> R<Option<Bytes>> {
    f183(Fn22::F184(v3, v2, v0), None::<Bytes>, v1)
}

pub fn f133(v0: Adt7, v1: Adt14, v2: Option<Bytes>, v3: Bytes) -> Option<Bytes> {
    let m910 = v2;
    match m910 {
        None => {
            let m911 = f134(v1, v3.clone());
            if match m911 {
                None => {
                    let m912 = f135(v3.clone());
                    match m912 {
                        None => {
                            false
                        }
                        Some(_) => {
                            true
                        }
                    }
                }
                Some(_) => {
                    true
                }
            } {
                None::<Bytes>
            } else {
                Some(f16(f17(Str::lit("runtime support: bound-trait: ")), f16(f18(f16(match v0 {
                    Adt7::C0(h913, _) => {
                        h913
                    }
                }, f16(f17(Str::lit(": ")), v3))), f17(Str::lit(" names no declared trait")))))
            }
        }
        Some(v4) => {
            Some(v4)
        }
    }
}

pub fn f134(v0: Adt14, v1: Bytes) -> Option<Adt10> {
    f185(Fn23::F186(v1), None::<Adt10>, match v0 {
        Adt14::C0(_, _, h914, _, _, _, _, _, _, _) => {
            h914
        }
    })
}

pub fn f135(v0: Bytes) -> Option<Bytes> {
    let a915 = v0.clone();
    let a916 = f17(Str::lit("Clone"));
    if equal(a915, a916) {
        Some(f17(Str::lit("clone")))
    } else {
        let a917 = v0;
        let a918 = f17(Str::lit("Drop"));
        if equal(a917, a918) {
            Some(f17(Str::lit("drop")))
        } else {
            None::<Bytes>
        }
    }
}

pub fn f136(v0: Bytes) -> bool {
    let a919 = v0.clone();
    let a920 = f17(Str::lit("Clone"));
    if equal(a919, a920) {
        false
    } else {
        let a921 = v0;
        let a922 = f17(Str::lit("Drop"));
        equal(a921, a922)
    }
}

pub fn f137(v0: List<Adt8>, v1: Bytes) -> R<u64> {
    f187(Fn24::F188(v1), 0u64, v0)
}

pub fn f138(v0: List<Adt8>, v1: Bytes) -> Option<Adt8> {
    f189(Fn25::F190(v1), None::<Adt8>, v0)
}

pub fn f139(v0: Adt5) -> R<Bytes> {
    Ok(f16(match v0.clone() {
        Adt5::C0(h923, _, _, _, _) => {
            h923
        }
    }, f16(f17(Str::lit("|")), f16(if match v0.clone() {
        Adt5::C0(_, h924, _, _, _) => {
            h924
        }
    } {
        f17(Str::lit("r"))
    } else {
        f17(Str::lit("-"))
    }, f16(if match v0.clone() {
        Adt5::C0(_, _, h925, _, _) => {
            h925
        }
    } {
        f17(Str::lit("x"))
    } else {
        f17(Str::lit("-"))
    }, f16(f17(Str::lit("|")), f16(f191(Fn26::F192, f17(Str::lit("")), match v0.clone() {
        Adt5::C0(_, _, _, h926, _) => {
            h926
        }
    })?, f16(f17(Str::lit("|")), f193(match v0 {
        Adt5::C0(_, _, _, _, h927) => {
            h927
        }
    })?))))))))
}

pub fn f140(v0: List<Adt5>, v1: Bytes) -> bool {
    f194(Fn27::F195(v1), false, v0)
}

pub fn f141(v0: Result<List<Adt17>, Bytes>, v1: List<Adt6>, v2: Bytes) -> Result<List<Adt17>, Bytes> {
    f196(Fn28::F197(v2), v0, v1)
}

pub fn f142(v0: Bytes, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m928 = v1;
    match m928 {
        None => {
            Some(v2)
        }
        Some(v3) => {
            Some({
                let a931 = {
                    let a929 = v3;
                    let a930 = v0;
                    append_bytes(a929, a930)
                };
                let a932 = v2;
                append_bytes(a931, a932)
            })
        }
    }
}

pub fn f143(v0: Fn15, p937: u64, p938: u64) -> R<(u64, bool)> {
    let m936 = (p937, p938);
    {
        let mut m936 = m936;
        loop {
            let step = {
                let (v1, v2) = m936;
                let m933 = v1;
                if m933 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m933 - 1;
                    let m935 = {
                        let c934: Fn15 = v0.clone();
                        c934.apply(v2)?
                    };
                    match m935 {
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
                Ok(next) => { m936 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f144(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f198(v0, v1) {
        Ok(Some({
            let a939 = v1;
            let a940 = 1u64;
            nat_add(a939, a940)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f145(v0: Bytes, v1: Adt14, v2: Adt8) -> Adt14 {
    let m943 = match v2.clone() {
        Adt8::C0(h942, _, _, _) => {
            h942
        }
    };
    if match m943 {
        Adt5::C0(_, h941, _, _, _) => {
            h941
        }
    } {
        v1
    } else {
        Adt14::C0(match v1.clone() {
            Adt14::C0(h944, _, _, _, _, _, _, _, _, _) => {
                h944
            }
        }, match v1.clone() {
            Adt14::C0(_, h945, _, _, _, _, _, _, _, _) => {
                h945
            }
        }, match v1.clone() {
            Adt14::C0(_, _, h946, _, _, _, _, _, _, _) => {
                h946
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, h947, _, _, _, _, _, _) => {
                h947
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, _, h948, _, _, _, _, _) => {
                h948
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, _, _, h949, _, _, _, _) => {
                h949
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, h950, _, _, _) => {
                h950
            }
        }, {
            let a959 = match v1.clone() {
                Adt14::C0(_, _, _, _, _, _, _, h951, _, _) => {
                    h951
                }
            };
            let a960 = List::cons(Adt13::C0(f16(v0, f16(f17(Str::lit("::")), {
                let m954 = match v2.clone() {
                    Adt8::C0(h953, _, _, _) => {
                        h953
                    }
                };
                match m954 {
                    Adt5::C0(h952, _, _, _, _) => {
                        h952
                    }
                }
            })), {
                let a958 = {
                    let m957 = match v2 {
                        Adt8::C0(h956, _, _, _) => {
                            h956
                        }
                    };
                    match m957 {
                        Adt5::C0(_, _, _, h955, _) => {
                            h955
                        }
                    }
                };
                length_list(a958)
            }), List::<Adt13>::nil());
            append_list(a959, a960)
        }, match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, h961, _) => {
                h961
            }
        }, match v1 {
            Adt14::C0(_, _, _, _, _, _, _, _, _, h962) => {
                h962
            }
        })
    }
}

pub fn f146(v0: Str) -> Bytes {
    let a963 = v0;
    utf8_encode(a963)
}

pub fn f147(v0: Bytes) -> R<Bytes> {
    let v1: u64 = {
        let (h964, _) = f143(Fn15::F199(v0.clone()), {
            let a965 = v0.clone();
            length_bytes(a965)
        }, 0u64)?;
        h964
    };
    Ok(f200(v0.clone(), v1, {
        let a967 = {
            let a966 = v0;
            length_bytes(a966)
        };
        let a968 = v1;
        nat_sub(a967, a968)
    }))
}

pub fn f148(v0: u64, v1: u64) -> Bytes {
    f200(f146(Str::lit("0123456789")), {
        let a972 = {
            let a969 = v0;
            let a970 = v1;
            let a971 = 0u64;
            nat_quot(a969, a970, a971)
        };
        let a973 = 10u64;
        let a974 = 0u64;
        nat_rem(a972, a973, a974)
    }, 1u64)
}

pub fn f149(v0: Adt9) -> Bytes {
    let m975 = v0;
    match m975 {
        Adt9::C0(v1) => {
            v1
        }
        Adt9::C1(v2, _) => {
            v2
        }
        Adt9::C2(v4, _, _) => {
            v4
        }
    }
}

pub fn f150(v0: Fn16, p979: Option<Adt11>, p980: List<Adt11>) -> Option<Adt11> {
    let m978 = (p979, p980);
    {
        let mut m978 = m978;
        loop {
            let step = {
                let (v1, v2) = m978;
                let m976 = v2;
                match m976.uncons() {
                    None => {
                        Err::<(Option<Adt11>, List<Adt11>), Option<Adt11>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Adt11>, List<Adt11>), Option<Adt11>>(({
                            let c977: Fn16 = v0.clone();
                            c977.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m978 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f151(v0: Bytes, v1: Option<Adt11>, v2: Adt11) -> Option<Adt11> {
    let m981 = v1;
    match m981 {
        None => {
            if f32(match v2.clone() {
                Adt11::C0(h982, _, _) => {
                    h982
                }
            }, v0) {
                Some(v2)
            } else {
                None::<Adt11>
            }
        }
        Some(v3) => {
            Some(v3)
        }
    }
}

pub fn f152(v0: Fn17, p986: Option<Adt12>, p987: List<Adt12>) -> Option<Adt12> {
    let m985 = (p986, p987);
    {
        let mut m985 = m985;
        loop {
            let step = {
                let (v1, v2) = m985;
                let m983 = v2;
                match m983.uncons() {
                    None => {
                        Err::<(Option<Adt12>, List<Adt12>), Option<Adt12>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Adt12>, List<Adt12>), Option<Adt12>>(({
                            let c984: Fn17 = v0.clone();
                            c984.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m985 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f153(v0: Bytes, v1: Option<Adt12>, v2: Adt12) -> Option<Adt12> {
    let m988 = v1;
    match m988 {
        None => {
            if f32(match v2.clone() {
                Adt12::C0(h989, _, _) => {
                    h989
                }
            }, v0) {
                Some(v2)
            } else {
                None::<Adt12>
            }
        }
        Some(v3) => {
            Some(v3)
        }
    }
}

pub fn f154(v0: Fn18, p993: Option<Adt13>, p994: List<Adt13>) -> Option<Adt13> {
    let m992 = (p993, p994);
    {
        let mut m992 = m992;
        loop {
            let step = {
                let (v1, v2) = m992;
                let m990 = v2;
                match m990.uncons() {
                    None => {
                        Err::<(Option<Adt13>, List<Adt13>), Option<Adt13>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Adt13>, List<Adt13>), Option<Adt13>>(({
                            let c991: Fn18 = v0.clone();
                            c991.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m992 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f155(v0: Bytes, v1: Option<Adt13>, v2: Adt13) -> Option<Adt13> {
    let m995 = v1;
    match m995 {
        None => {
            if f32(match v2.clone() {
                Adt13::C0(h996, _) => {
                    h996
                }
            }, v0) {
                Some(v2)
            } else {
                None::<Adt13>
            }
        }
        Some(v3) => {
            Some(v3)
        }
    }
}

pub fn f156(v0: List<Bytes>) -> Bytes {
    let m997 = v0;
    match m997.uncons() {
        None => {
            f17(Str::lit(""))
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f157(p1001: List<Bytes>, p1002: u64) -> Option<Bytes> {
    let m1000 = (p1001, p1002);
    {
        let mut m1000 = m1000;
        loop {
            let step = {
                let (v0, v1) = m1000;
                let m998 = v0;
                match m998.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        let m999 = v1;
                        if m999 == 0 {
                            Err::<(List<Bytes>, u64), Option<Bytes>>(Some(v2))
                        } else {
                            let v4 = m999 - 1;
                            Ok::<(List<Bytes>, u64), Option<Bytes>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1000 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f158(v0: List<Adt9>, v1: Bytes) -> Option<Adt9> {
    f201(Fn29::F202(v1), None::<Adt9>, v0)
}

pub fn f159(v0: Bytes) -> bool {
    let a1003 = v0.clone();
    let a1004 = f17(Str::lit("u8"));
    if equal(a1003, a1004) {
        true
    } else {
        let a1005 = v0.clone();
        let a1006 = f17(Str::lit("u16"));
        if equal(a1005, a1006) {
            true
        } else {
            let a1007 = v0.clone();
            let a1008 = f17(Str::lit("u32"));
            if equal(a1007, a1008) {
                true
            } else {
                let a1009 = v0.clone();
                let a1010 = f17(Str::lit("u64"));
                if equal(a1009, a1010) {
                    true
                } else {
                    let a1011 = v0.clone();
                    let a1012 = f17(Str::lit("u128"));
                    if equal(a1011, a1012) {
                        true
                    } else {
                        let a1013 = v0.clone();
                        let a1014 = f17(Str::lit("i8"));
                        if equal(a1013, a1014) {
                            true
                        } else {
                            let a1015 = v0.clone();
                            let a1016 = f17(Str::lit("i16"));
                            if equal(a1015, a1016) {
                                true
                            } else {
                                let a1017 = v0.clone();
                                let a1018 = f17(Str::lit("i32"));
                                if equal(a1017, a1018) {
                                    true
                                } else {
                                    let a1019 = v0.clone();
                                    let a1020 = f17(Str::lit("i64"));
                                    if equal(a1019, a1020) {
                                        true
                                    } else {
                                        let a1021 = v0;
                                        let a1022 = f17(Str::lit("i128"));
                                        equal(a1021, a1022)
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

pub fn f160(v0: Bytes, v1: Bytes) -> bool {
    let m1023 = f203(v0.clone(), v1.clone());
    match m1023 {
        None => {
            if f159(v0) {
                f204(f17(Str::lit("usize")), v1)
            } else {
                false
            }
        }
        Some(_) => {
            f204(v0, v1)
        }
    }
}

pub fn f161(v0: Adt14, v1: Bytes) -> Option<Adt13> {
    f154(Fn18::F205(v1), None::<Adt13>, match v0 {
        Adt14::C0(_, _, _, _, _, _, _, h1024, _, _) => {
            h1024
        }
    })
}

pub fn f162(v0: Bytes) -> Option<u64> {
    let a1025 = v0.clone();
    let a1026 = f17(Str::lit("Some"));
    if equal(a1025, a1026) {
        Some(1u64)
    } else {
        let a1027 = v0.clone();
        let a1028 = f17(Str::lit("Ok"));
        if equal(a1027, a1028) {
            Some(1u64)
        } else {
            let a1029 = v0;
            let a1030 = f17(Str::lit("Err"));
            if equal(a1029, a1030) {
                Some(1u64)
            } else {
                None::<u64>
            }
        }
    }
}

pub fn f163(v0: Adt14, v1: Bytes) -> Option<u64> {
    let m1031 = f107(v0, v1);
    match m1031 {
        None => {
            None::<u64>
        }
        Some(v2) => {
            Some({
                let a1033 = match v2 {
                    Adt11::C0(_, h1032, _) => {
                        h1032
                    }
                };
                length_list(a1033)
            })
        }
    }
}

pub fn f164(v0: Bytes, v1: Bytes) -> Option<u64> {
    let m1034 = f203(v0.clone(), v1.clone());
    match m1034 {
        None => {
            if f159(v0) {
                f203(f17(Str::lit("usize")), v1)
            } else {
                None::<u64>
            }
        }
        Some(v2) => {
            Some(v2)
        }
    }
}

pub fn f165(v0: Adt14, v1: Bytes, v2: Bytes) -> R<Option<u64>> {
    let m1035 = f108(v0, v1);
    match m1035 {
        None => {
            Ok(None::<u64>)
        }
        Some(v3) => {
            f206(match v3 {
                Adt12::C0(_, h1036, _) => {
                    h1036
                }
            }, v2)
        }
    }
}

pub fn f166(v0: Adt14, v1: Bytes, v2: Bytes) -> Option<u64> {
    let m1037 = f161(v0, f16(v1, f16(f17(Str::lit("::")), v2)));
    match m1037 {
        None => {
            None::<u64>
        }
        Some(v3) => {
            Some(match v3 {
                Adt13::C0(_, h1038) => {
                    h1038
                }
            })
        }
    }
}

pub fn f167(v0: Fn19, p1042: List<u64>, p1043: List<Adt13>) -> List<u64> {
    let m1041 = (p1042, p1043);
    {
        let mut m1041 = m1041;
        loop {
            let step = {
                let (v1, v2) = m1041;
                let m1039 = v2;
                match m1039.uncons() {
                    None => {
                        Err::<(List<u64>, List<Adt13>), List<u64>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<u64>, List<Adt13>), List<u64>>(({
                            let c1040: Fn19 = v0.clone();
                            c1040.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1041 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f168(v0: Bytes, v1: List<u64>, v2: Adt13) -> List<u64> {
    if f32(match v2.clone() {
        Adt13::C0(h1044, _) => {
            h1044
        }
    }, v0) {
        let a1046 = v1;
        let a1047 = List::cons(match v2 {
            Adt13::C0(_, h1045) => {
                h1045
            }
        }, List::<u64>::nil());
        append_list(a1046, a1047)
    } else {
        v1
    }
}

pub fn f169(v0: Fn20, p1051: bool, p1052: List<u64>) -> bool {
    let m1050 = (p1051, p1052);
    {
        let mut m1050 = m1050;
        loop {
            let step = {
                let (v1, v2) = m1050;
                let m1048 = v2;
                match m1048.uncons() {
                    None => {
                        Err::<(bool, List<u64>), bool>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<u64>), bool>(({
                            let c1049: Fn20 = v0.clone();
                            c1049.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1050 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f170(v0: u64, v1: bool, v2: u64) -> bool {
    if v1 {
        true
    } else {
        let a1053 = v2;
        let a1054 = v0;
        nat_eq(a1053, a1054)
    }
}

pub fn f171(p1065: Adt2) -> Adt2 {
    let m1064 = p1065;
    {
        let mut m1064 = m1064;
        loop {
            let step = {
                let v0 = m1064;
                let m1055 = v0;
                match m1055 {
                    Adt2::C0 => {
                        Err::<Adt2, Adt2>(Adt2::C0)
                    }
                    Adt2::C1 => {
                        Err::<Adt2, Adt2>(Adt2::C1)
                    }
                    Adt2::C2 => {
                        Err::<Adt2, Adt2>(Adt2::C2)
                    }
                    Adt2::C3 => {
                        Err::<Adt2, Adt2>(Adt2::C3)
                    }
                    Adt2::C4 => {
                        Err::<Adt2, Adt2>(Adt2::C4)
                    }
                    Adt2::C5 => {
                        Err::<Adt2, Adt2>(Adt2::C5)
                    }
                    Adt2::C6(v1) => {
                        Err::<Adt2, Adt2>(Adt2::C6(v1))
                    }
                    Adt2::C7(r1056) => {
                        let v2 = (*r1056).clone();
                        Ok::<Adt2, Adt2>(v2)
                    }
                    Adt2::C8(r1057) => {
                        let v3 = (*r1057).clone();
                        Ok::<Adt2, Adt2>(v3)
                    }
                    Adt2::C9(r1058, r1059) => {
                        let v4 = (*r1058).clone();
                        let v5 = (*r1059).clone();
                        Err::<Adt2, Adt2>(Adt2::C9(std::rc::Rc::new(v4), std::rc::Rc::new(v5)))
                    }
                    Adt2::C10(v6, r1060) => {
                        let v7 = (*r1060).clone();
                        Err::<Adt2, Adt2>(Adt2::C10(v6, std::rc::Rc::new(v7)))
                    }
                    Adt2::C11 => {
                        Err::<Adt2, Adt2>(Adt2::C11)
                    }
                    Adt2::C12 => {
                        Err::<Adt2, Adt2>(Adt2::C12)
                    }
                    Adt2::C13 => {
                        Err::<Adt2, Adt2>(Adt2::C13)
                    }
                    Adt2::C14 => {
                        Err::<Adt2, Adt2>(Adt2::C14)
                    }
                    Adt2::C15(r1061) => {
                        let v8 = (*r1061).clone();
                        Err::<Adt2, Adt2>(Adt2::C15(std::rc::Rc::new(v8)))
                    }
                    Adt2::C16 => {
                        Err::<Adt2, Adt2>(Adt2::C16)
                    }
                    Adt2::C17 => {
                        Err::<Adt2, Adt2>(Adt2::C17)
                    }
                    Adt2::C18 => {
                        Err::<Adt2, Adt2>(Adt2::C18)
                    }
                    Adt2::C19(r1062, r1063) => {
                        let v9 = (*r1062).clone();
                        let v10 = (*r1063).clone();
                        Err::<Adt2, Adt2>(Adt2::C19(std::rc::Rc::new(v9), std::rc::Rc::new(v10)))
                    }
                }
            };
            match step {
                Ok(next) => { m1064 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f172(v0: Adt16, v1: Adt3) -> Adt2 {
    let m1066 = v1;
    match m1066 {
        Adt3::C0(v2) => {
            let m1068 = f52(match v0 {
                Adt16::C0(_, h1067, _, _) => {
                    h1067
                }
            }, v2);
            match m1068 {
                None => {
                    Adt2::C16
                }
                Some(v3) => {
                    match v3 {
                        Adt17::C0(_, _, h1069) => {
                            h1069
                        }
                    }
                }
            }
        }
        Adt3::C1(_) => {
            Adt2::C16
        }
        Adt3::C2(_) => {
            Adt2::C16
        }
        Adt3::C3(_, _) => {
            Adt2::C16
        }
        Adt3::C4(_, _, _) => {
            Adt2::C16
        }
        Adt3::C5(r1070, v12) => {
            let v11 = (*r1070).clone();
            f207(f208(v0.clone(), f171(f172(v0, v11))), v12)
        }
        Adt3::C6(_, _) => {
            Adt2::C16
        }
        Adt3::C7(r1071) => {
            let v15 = (*r1071).clone();
            let v16: Adt2 = f172(v0, v15);
            if f209(v16.clone()) {
                Adt2::C16
            } else {
                Adt2::C7(std::rc::Rc::new(v16))
            }
        }
        Adt3::C8(_, _) => {
            Adt2::C16
        }
        Adt3::C9(_, _) => {
            Adt2::C16
        }
        Adt3::C10(_, _, _) => {
            Adt2::C16
        }
        Adt3::C11(_, _) => {
            Adt2::C16
        }
        Adt3::C12(_, _, _) => {
            Adt2::C16
        }
        Adt3::C13(_, _) => {
            Adt2::C16
        }
        Adt3::C14(_, _, _) => {
            Adt2::C16
        }
        Adt3::C15(_) => {
            Adt2::C16
        }
        Adt3::C16(_, _, _) => {
            Adt2::C16
        }
        Adt3::C17 => {
            Adt2::C16
        }
        Adt3::C18(_) => {
            Adt2::C16
        }
        Adt3::C19(_) => {
            Adt2::C16
        }
        Adt3::C20(_, _, _) => {
            Adt2::C16
        }
        Adt3::C21(_, _, _) => {
            Adt2::C16
        }
        Adt3::C22(_) => {
            Adt2::C16
        }
        Adt3::C23(_, _) => {
            Adt2::C16
        }
        Adt3::C24(_, _) => {
            Adt2::C16
        }
        Adt3::C25(_, _, _) => {
            Adt2::C16
        }
        Adt3::C26(_) => {
            Adt2::C16
        }
        Adt3::C27(_) => {
            Adt2::C16
        }
        Adt3::C28(_, _, _, _) => {
            Adt2::C16
        }
        Adt3::C29(_, _) => {
            Adt2::C16
        }
        Adt3::C30 => {
            Adt2::C16
        }
        Adt3::C31 => {
            Adt2::C16
        }
        Adt3::C32(_, _) => {
            Adt2::C16
        }
        Adt3::C33(_, _, _, _) => {
            Adt2::C16
        }
        Adt3::C34(_, _, _, _) => {
            Adt2::C16
        }
    }
}

pub fn f173(v0: Adt14, v1: List<Bytes>, v2: Bytes) -> R<bool> {
    f86(Fn8::F210(v2, v0), false, v1)
}

pub fn f174(p1076: List<Adt7>, p1077: u64) -> List<Bytes> {
    let m1075 = (p1076, p1077);
    {
        let mut m1075 = m1075;
        loop {
            let step = {
                let (v0, v1) = m1075;
                let m1072 = v0;
                match m1072.uncons() {
                    None => {
                        Err::<(List<Adt7>, u64), List<Bytes>>(List::<Bytes>::nil())
                    }
                    Some((v2, v3)) => {
                        let m1073 = v1;
                        if m1073 == 0 {
                            match v2 {
                                Adt7::C0(_, h1074) => {
                                    Err::<(List<Adt7>, u64), List<Bytes>>(h1074)
                                }
                            }
                        } else {
                            let v4 = m1073 - 1;
                            Ok::<(List<Adt7>, u64), List<Bytes>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1075 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f175(v0: Fn21, p1081: Bytes, p1082: List<u64>) -> R<Bytes> {
    let m1080 = (p1081, p1082);
    {
        let mut m1080 = m1080;
        loop {
            let step = {
                let (v1, v2) = m1080;
                let m1078 = v2;
                match m1078.uncons() {
                    None => {
                        Err::<(Bytes, List<u64>), R<Bytes>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Bytes, List<u64>), R<Bytes>>(({
                            let c1079: Fn21 = v0.clone();
                            c1079.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1080 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f176(v0: Bytes, v1: u64) -> R<Bytes> {
    let a1083 = v0.clone();
    let a1084 = f17(Str::lit(""));
    if equal(a1083, a1084) {
        f47(v1)
    } else {
        Ok(f16(v0, f16(f17(Str::lit("/")), f47(v1)?)))
    }
}

pub fn f177(v0: Adt3) -> u64 {
    let m1085 = v0;
    match m1085 {
        Adt3::C0(_) => {
            14u64
        }
        Adt3::C1(_) => {
            14u64
        }
        Adt3::C2(_) => {
            14u64
        }
        Adt3::C3(_, _) => {
            13u64
        }
        Adt3::C4(_, _, _) => {
            13u64
        }
        Adt3::C5(_, _) => {
            13u64
        }
        Adt3::C6(_, _) => {
            13u64
        }
        Adt3::C7(_) => {
            12u64
        }
        Adt3::C8(_, _) => {
            5u64
        }
        Adt3::C9(_, _) => {
            14u64
        }
        Adt3::C10(_, _, _) => {
            1u64
        }
        Adt3::C11(_, _) => {
            1u64
        }
        Adt3::C12(_, _, _) => {
            14u64
        }
        Adt3::C13(_, _) => {
            14u64
        }
        Adt3::C14(_, _, _) => {
            14u64
        }
        Adt3::C15(_) => {
            14u64
        }
        Adt3::C16(_, _, _) => {
            14u64
        }
        Adt3::C17 => {
            1u64
        }
        Adt3::C18(_) => {
            1u64
        }
        Adt3::C19(_) => {
            14u64
        }
        Adt3::C20(_, _, _) => {
            14u64
        }
        Adt3::C21(v40, _, _) => {
            f179(v40)
        }
        Adt3::C22(_) => {
            12u64
        }
        Adt3::C23(_, _) => {
            11u64
        }
        Adt3::C24(_, _) => {
            13u64
        }
        Adt3::C25(_, _, _) => {
            14u64
        }
        Adt3::C26(_) => {
            12u64
        }
        Adt3::C27(_) => {
            12u64
        }
        Adt3::C28(_, _, _, _) => {
            13u64
        }
        Adt3::C29(_, _) => {
            14u64
        }
        Adt3::C30 => {
            14u64
        }
        Adt3::C31 => {
            14u64
        }
        Adt3::C32(_, _) => {
            14u64
        }
        Adt3::C33(_, _, _, _) => {
            14u64
        }
        Adt3::C34(_, _, _, _) => {
            14u64
        }
    }
}

pub fn f178(v0: u64, v1: Adt3) -> bool {
    let a1086 = v0;
    let a1087 = 8u64;
    if if nat_eq(a1086, a1087) {
        f211(v1.clone())
    } else {
        false
    } {
        true
    } else {
        let a1090 = f177(v1);
        let a1091 = {
            let a1088 = f179(v0);
            let a1089 = 5u64;
            if nat_eq(a1088, a1089) {
                6u64
            } else {
                f179(v0)
            }
        };
        nat_lt(a1090, a1091)
    }
}

pub fn f179(v0: u64) -> u64 {
    let a1092 = v0;
    let a1093 = 6u64;
    if nat_eq(a1092, a1093) {
        3u64
    } else {
        let a1094 = v0;
        let a1095 = 5u64;
        if nat_eq(a1094, a1095) {
            4u64
        } else {
            let a1096 = v0;
            let a1097 = 7u64;
            if nat_eq(a1096, a1097) {
                5u64
            } else {
                let a1098 = v0;
                let a1099 = 8u64;
                if nat_eq(a1098, a1099) {
                    5u64
                } else {
                    let a1100 = v0;
                    let a1101 = 9u64;
                    if nat_eq(a1100, a1101) {
                        5u64
                    } else {
                        let a1102 = v0;
                        let a1103 = 10u64;
                        if nat_eq(a1102, a1103) {
                            5u64
                        } else {
                            let a1104 = v0;
                            let a1105 = 11u64;
                            if nat_eq(a1104, a1105) {
                                5u64
                            } else {
                                let a1106 = v0;
                                let a1107 = 0u64;
                                if nat_eq(a1106, a1107) {
                                    9u64
                                } else {
                                    let a1108 = v0;
                                    let a1109 = 1u64;
                                    if nat_eq(a1108, a1109) {
                                        9u64
                                    } else {
                                        10u64
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

pub fn f180(v0: Adt9) -> R<u64> {
    let m1110 = v0;
    match m1110 {
        Adt9::C0(_) => {
            Ok(0u64)
        }
        Adt9::C1(_, v3) => {
            f46(v3)
        }
        Adt9::C2(_, _, v6) => {
            f46(v6)
        }
    }
}

pub fn f181(v0: List<Bytes>, v1: List<Bytes>, v2: Option<Bytes>, v3: Bytes) -> R<Option<Bytes>> {
    let m1111 = v2;
    match m1111 {
        None => {
            if f53(v0, v3.clone())? {
                Ok(None::<Bytes>)
            } else {
                Ok(Some(f16(f17(Str::lit("runtime support: pattern-fields: ")), f16(f18(f34(v1, Str::lit("::"))?), f16(f17(Str::lit(" has no field ")), f18(v3))))))
            }
        }
        Some(v4) => {
            Ok(Some(v4))
        }
    }
}

pub fn f182(v0: List<Bytes>, v1: Bytes) -> R<u64> {
    f212(Fn30::F213(v1), 0u64, v0)
}

pub fn f183(v0: Fn22, p1115: Option<Bytes>, p1116: List<Adt6>) -> R<Option<Bytes>> {
    let m1114 = (p1115, p1116);
    {
        let mut m1114 = m1114;
        loop {
            let step = {
                let (v1, v2) = m1114;
                let m1112 = v2;
                match m1112.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Adt6>), R<Option<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Adt6>), R<Option<Bytes>>>(({
                            let c1113: Fn22 = v0.clone();
                            c1113.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1114 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f184(v0: bool, v1: u64, v2: Adt14, v3: Option<Bytes>, v4: Adt6) -> R<Option<Bytes>> {
    let m1117 = v3;
    match m1117 {
        None => {
            let m1119 = f39(Str::lit("a parameter name"), match v4.clone() {
                Adt6::C0(h1118, _, _) => {
                    h1118
                }
            })?;
            match m1119 {
                None => {
                    f12(v2, match v4 {
                        Adt6::C0(_, _, h1120) => {
                            h1120
                        }
                    }, v1, v0)
                }
                Some(v6) => {
                    Ok(Some(v6))
                }
            }
        }
        Some(v5) => {
            Ok(Some(v5))
        }
    }
}

pub fn f185(v0: Fn23, p1124: Option<Adt10>, p1125: List<Adt10>) -> Option<Adt10> {
    let m1123 = (p1124, p1125);
    {
        let mut m1123 = m1123;
        loop {
            let step = {
                let (v1, v2) = m1123;
                let m1121 = v2;
                match m1121.uncons() {
                    None => {
                        Err::<(Option<Adt10>, List<Adt10>), Option<Adt10>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Adt10>, List<Adt10>), Option<Adt10>>(({
                            let c1122: Fn23 = v0.clone();
                            c1122.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1123 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f186(v0: Bytes, v1: Option<Adt10>, v2: Adt10) -> Option<Adt10> {
    let m1126 = v1;
    match m1126 {
        None => {
            if f32(match v2.clone() {
                Adt10::C0(h1127, _) => {
                    h1127
                }
            }, v0) {
                Some(v2)
            } else {
                None::<Adt10>
            }
        }
        Some(v3) => {
            Some(v3)
        }
    }
}

pub fn f187(v0: Fn24, p1131: u64, p1132: List<Adt8>) -> R<u64> {
    let m1130 = (p1131, p1132);
    {
        let mut m1130 = m1130;
        loop {
            let step = {
                let (v1, v2) = m1130;
                let m1128 = v2;
                match m1128.uncons() {
                    None => {
                        Err::<(u64, List<Adt8>), R<u64>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(u64, List<Adt8>), R<u64>>(({
                            let c1129: Fn24 = v0.clone();
                            c1129.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1130 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f188(v0: Bytes, v1: u64, v2: Adt8) -> R<u64> {
    if f32({
        let m1135 = match v2 {
            Adt8::C0(h1134, _, _, _) => {
                h1134
            }
        };
        match m1135 {
            Adt5::C0(h1133, _, _, _, _) => {
                h1133
            }
        }
    }, v0) {
        let a1136 = v1;
        let a1137 = 1u64;
        nat_add(a1136, a1137)
    } else {
        Ok(v1)
    }
}

pub fn f189(v0: Fn25, p1141: Option<Adt8>, p1142: List<Adt8>) -> Option<Adt8> {
    let m1140 = (p1141, p1142);
    {
        let mut m1140 = m1140;
        loop {
            let step = {
                let (v1, v2) = m1140;
                let m1138 = v2;
                match m1138.uncons() {
                    None => {
                        Err::<(Option<Adt8>, List<Adt8>), Option<Adt8>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Adt8>, List<Adt8>), Option<Adt8>>(({
                            let c1139: Fn25 = v0.clone();
                            c1139.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1140 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f190(v0: Bytes, v1: Option<Adt8>, v2: Adt8) -> Option<Adt8> {
    let m1143 = v1;
    match m1143 {
        None => {
            if f32({
                let m1146 = match v2.clone() {
                    Adt8::C0(h1145, _, _, _) => {
                        h1145
                    }
                };
                match m1146 {
                    Adt5::C0(h1144, _, _, _, _) => {
                        h1144
                    }
                }
            }, v0) {
                Some(v2)
            } else {
                None::<Adt8>
            }
        }
        Some(v3) => {
            Some(v3)
        }
    }
}

pub fn f191(v0: Fn26, p1150: Bytes, p1151: List<Adt6>) -> R<Bytes> {
    let m1149 = (p1150, p1151);
    {
        let mut m1149 = m1149;
        loop {
            let step = {
                let (v1, v2) = m1149;
                let m1147 = v2;
                match m1147.uncons() {
                    None => {
                        Err::<(Bytes, List<Adt6>), R<Bytes>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Bytes, List<Adt6>), R<Bytes>>(({
                            let c1148: Fn26 = v0.clone();
                            c1148.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1149 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f192(v0: Bytes, v1: Adt6) -> R<Bytes> {
    Ok(f16(v0, f16(match v1.clone() {
        Adt6::C0(h1152, _, _) => {
            h1152
        }
    }, f16(if match v1.clone() {
        Adt6::C0(_, h1153, _) => {
            h1153
        }
    } {
        f17(Str::lit(":m:"))
    } else {
        f17(Str::lit(":-:"))
    }, f16(f193(match v1 {
        Adt6::C0(_, _, h1154) => {
            h1154
        }
    })?, f17(Str::lit(",")))))))
}

pub fn f193(v0: Adt2) -> R<Bytes> {
    let m1155 = v0;
    match m1155 {
        Adt2::C0 => {
            Ok(f17(Str::lit("U")))
        }
        Adt2::C1 => {
            Ok(f17(Str::lit("B")))
        }
        Adt2::C2 => {
            Ok(f17(Str::lit("N")))
        }
        Adt2::C3 => {
            Ok(f17(Str::lit("F")))
        }
        Adt2::C4 => {
            Ok(f17(Str::lit("O")))
        }
        Adt2::C5 => {
            Ok(f17(Str::lit("S")))
        }
        Adt2::C6(v1) => {
            Ok(f16(f17(Str::lit("P")), f16(f47(v1)?, f17(Str::lit(";")))))
        }
        Adt2::C7(r1156) => {
            let v2 = (*r1156).clone();
            Ok(f16(f17(Str::lit("R(")), f16(f193(v2)?, f17(Str::lit(")")))))
        }
        Adt2::C8(r1157) => {
            let v3 = (*r1157).clone();
            Ok(f16(f17(Str::lit("M(")), f16(f193(v3)?, f17(Str::lit(")")))))
        }
        Adt2::C9(r1158, r1159) => {
            let v4 = (*r1158).clone();
            let v5 = (*r1159).clone();
            Ok(f16(f17(Str::lit("T(")), f16(f193(v4)?, f16(f17(Str::lit(",")), f16(f193(v5)?, f17(Str::lit(")")))))))
        }
        Adt2::C10(v6, r1160) => {
            let v7 = (*r1160).clone();
            Ok(f16(f17(Str::lit("D")), f16(f18(v6), f16(f17(Str::lit("(")), f16(f193(v7)?, f17(Str::lit(")")))))))
        }
        Adt2::C11 => {
            Ok(f17(Str::lit("W")))
        }
        Adt2::C12 => {
            Ok(f17(Str::lit("Z")))
        }
        Adt2::C13 => {
            Ok(f17(Str::lit("C")))
        }
        Adt2::C14 => {
            Ok(f17(Str::lit("Q")))
        }
        Adt2::C15(r1161) => {
            let v8 = (*r1161).clone();
            Ok(f16(f17(Str::lit("L(")), f16(f193(v8)?, f17(Str::lit(")")))))
        }
        Adt2::C16 => {
            Ok(f17(Str::lit("?")))
        }
        Adt2::C17 => {
            Ok(f17(Str::lit("-")))
        }
        Adt2::C18 => {
            Ok(f17(Str::lit("")))
        }
        Adt2::C19(r1162, r1163) => {
            let v9 = (*r1162).clone();
            let v10 = (*r1163).clone();
            Ok(f16(f193(v9)?, f16(f17(Str::lit(";")), f193(v10)?)))
        }
    }
}

pub fn f194(v0: Fn27, p1167: bool, p1168: List<Adt5>) -> bool {
    let m1166 = (p1167, p1168);
    {
        let mut m1166 = m1166;
        loop {
            let step = {
                let (v1, v2) = m1166;
                let m1164 = v2;
                match m1164.uncons() {
                    None => {
                        Err::<(bool, List<Adt5>), bool>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<Adt5>), bool>(({
                            let c1165: Fn27 = v0.clone();
                            c1165.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1166 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f195(v0: Bytes, v1: bool, v2: Adt5) -> bool {
    if v1 {
        true
    } else {
        f32(match v2 {
            Adt5::C0(h1169, _, _, _, _) => {
                h1169
            }
        }, v0)
    }
}

pub fn f196(v0: Fn28, p1173: Result<List<Adt17>, Bytes>, p1174: List<Adt6>) -> Result<List<Adt17>, Bytes> {
    let m1172 = (p1173, p1174);
    {
        let mut m1172 = m1172;
        loop {
            let step = {
                let (v1, v2) = m1172;
                let m1170 = v2;
                match m1170.uncons() {
                    None => {
                        Err::<(Result<List<Adt17>, Bytes>, List<Adt6>), Result<List<Adt17>, Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Result<List<Adt17>, Bytes>, List<Adt6>), Result<List<Adt17>, Bytes>>(({
                            let c1171: Fn28 = v0.clone();
                            c1171.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1172 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f197(v0: Bytes, v1: Result<List<Adt17>, Bytes>, v2: Adt6) -> Result<List<Adt17>, Bytes> {
    let m1175 = v1;
    match m1175 {
        Ok(v4) => {
            let m1177 = f52(v4.clone(), match v2.clone() {
                Adt6::C0(h1176, _, _) => {
                    h1176
                }
            });
            match m1177 {
                None => {
                    Ok::<List<Adt17>, Bytes>(List::cons(Adt17::C0(match v2.clone() {
                        Adt6::C0(h1179, _, _) => {
                            h1179
                        }
                    }, match v2.clone() {
                        Adt6::C0(_, h1180, _) => {
                            h1180
                        }
                    }, match v2 {
                        Adt6::C0(_, _, h1181) => {
                            h1181
                        }
                    }), v4))
                }
                Some(_) => {
                    Err::<List<Adt17>, Bytes>(f16(f17(Str::lit("runtime support: param-duplicate: ")), f16(f18(match v2 {
                        Adt6::C0(h1178, _, _) => {
                            h1178
                        }
                    }), f16(f17(Str::lit(" names two parameters of ")), f18(v0)))))
                }
            }
        }
        Err(v3) => {
            Err::<List<Adt17>, Bytes>(v3)
        }
    }
}

pub fn f198(v0: Bytes, v1: u64) -> bool {
    let m1182 = f214(v0, v1);
    match m1182 {
        None => {
            false
        }
        Some(v2) => {
            if f215(v2, 97u8, 122u8) {
                true
            } else {
                if f215(v2, 65u8, 90u8) {
                    true
                } else {
                    let a1183 = v2;
                    let a1184 = 95u8;
                    if equal(a1183, a1184) {
                        true
                    } else {
                        let a1185 = 0u64;
                        let a1186 = v1;
                        if nat_lt(a1185, a1186) {
                            f215(v2, 48u8, 57u8)
                        } else {
                            false
                        }
                    }
                }
            }
        }
    }
}

pub fn f199(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f216(v0, v1)
}

pub fn f200(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m1190 = {
        let a1187 = v0;
        let a1188 = v1;
        let a1189 = v2;
        slice_bytes(a1187, a1188, a1189)
    };
    match m1190 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f201(v0: Fn29, p1194: Option<Adt9>, p1195: List<Adt9>) -> Option<Adt9> {
    let m1193 = (p1194, p1195);
    {
        let mut m1193 = m1193;
        loop {
            let step = {
                let (v1, v2) = m1193;
                let m1191 = v2;
                match m1191.uncons() {
                    None => {
                        Err::<(Option<Adt9>, List<Adt9>), Option<Adt9>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Adt9>, List<Adt9>), Option<Adt9>>(({
                            let c1192: Fn29 = v0.clone();
                            c1192.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1193 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f202(v0: Bytes, v1: Option<Adt9>, v2: Adt9) -> Option<Adt9> {
    let m1196 = v1;
    match m1196 {
        None => {
            if f32(f149(v2.clone()), v0) {
                Some(v2)
            } else {
                None::<Adt9>
            }
        }
        Some(v3) => {
            Some(v3)
        }
    }
}

pub fn f203(v0: Bytes, v1: Bytes) -> Option<u64> {
    let a1197 = v0.clone();
    let a1198 = f17(Str::lit("AtomicU64"));
    if if equal(a1197, a1198) {
        let a1199 = v1.clone();
        let a1200 = f17(Str::lit("new"));
        equal(a1199, a1200)
    } else {
        false
    } {
        Some(1u64)
    } else {
        let a1201 = v0.clone();
        let a1202 = f17(Str::lit("Rc"));
        if if equal(a1201, a1202) {
            let a1203 = v1.clone();
            let a1204 = f17(Str::lit("new"));
            equal(a1203, a1204)
        } else {
            false
        } {
            Some(1u64)
        } else {
            let a1205 = v0.clone();
            let a1206 = f17(Str::lit("Rc"));
            if if equal(a1205, a1206) {
                let a1207 = v1.clone();
                let a1208 = f17(Str::lit("from"));
                equal(a1207, a1208)
            } else {
                false
            } {
                Some(1u64)
            } else {
                let a1209 = v0.clone();
                let a1210 = f17(Str::lit("Rc"));
                if if equal(a1209, a1210) {
                    let a1211 = v1.clone();
                    let a1212 = f17(Str::lit("try_unwrap"));
                    equal(a1211, a1212)
                } else {
                    false
                } {
                    Some(1u64)
                } else {
                    let a1213 = v0.clone();
                    let a1214 = f17(Str::lit("Rc"));
                    if if equal(a1213, a1214) {
                        let a1215 = v1.clone();
                        let a1216 = f17(Str::lit("get_mut"));
                        equal(a1215, a1216)
                    } else {
                        false
                    } {
                        Some(1u64)
                    } else {
                        let a1217 = v0.clone();
                        let a1218 = f17(Str::lit("Vec"));
                        if if equal(a1217, a1218) {
                            let a1219 = v1.clone();
                            let a1220 = f17(Str::lit("new"));
                            equal(a1219, a1220)
                        } else {
                            false
                        } {
                            Some(0u64)
                        } else {
                            let a1221 = v0.clone();
                            let a1222 = f17(Str::lit("Vec"));
                            if if equal(a1221, a1222) {
                                let a1223 = v1.clone();
                                let a1224 = f17(Str::lit("with_capacity"));
                                equal(a1223, a1224)
                            } else {
                                false
                            } {
                                Some(1u64)
                            } else {
                                let a1225 = v0.clone();
                                let a1226 = f17(Str::lit("String"));
                                if if equal(a1225, a1226) {
                                    let a1227 = v1.clone();
                                    let a1228 = f17(Str::lit("new"));
                                    equal(a1227, a1228)
                                } else {
                                    false
                                } {
                                    Some(0u64)
                                } else {
                                    let a1229 = v0.clone();
                                    let a1230 = f17(Str::lit("mem"));
                                    if if equal(a1229, a1230) {
                                        let a1231 = v1.clone();
                                        let a1232 = f17(Str::lit("replace"));
                                        equal(a1231, a1232)
                                    } else {
                                        false
                                    } {
                                        Some(2u64)
                                    } else {
                                        let a1233 = v0.clone();
                                        let a1234 = f17(Str::lit("str"));
                                        if if equal(a1233, a1234) {
                                            let a1235 = v1.clone();
                                            let a1236 = f17(Str::lit("from_utf8"));
                                            equal(a1235, a1236)
                                        } else {
                                            false
                                        } {
                                            Some(1u64)
                                        } else {
                                            let a1237 = v0.clone();
                                            let a1238 = f17(Str::lit("usize"));
                                            if if equal(a1237, a1238) {
                                                let a1239 = v1.clone();
                                                let a1240 = f17(Str::lit("from"));
                                                equal(a1239, a1240)
                                            } else {
                                                false
                                            } {
                                                Some(1u64)
                                            } else {
                                                let a1241 = v0;
                                                let a1242 = f17(Str::lit("usize"));
                                                if if equal(a1241, a1242) {
                                                    let a1243 = v1;
                                                    let a1244 = f17(Str::lit("try_from"));
                                                    equal(a1243, a1244)
                                                } else {
                                                    false
                                                } {
                                                    Some(1u64)
                                                } else {
                                                    None::<u64>
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

pub fn f204(v0: Bytes, v1: Bytes) -> bool {
    let a1245 = v0.clone();
    let a1246 = f17(Str::lit("AtomicU64"));
    if if equal(a1245, a1246) {
        let a1247 = v1.clone();
        let a1248 = f17(Str::lit("new"));
        equal(a1247, a1248)
    } else {
        false
    } {
        false
    } else {
        let a1249 = v0.clone();
        let a1250 = f17(Str::lit("Rc"));
        if if equal(a1249, a1250) {
            let a1251 = v1.clone();
            let a1252 = f17(Str::lit("new"));
            equal(a1251, a1252)
        } else {
            false
        } {
            true
        } else {
            let a1253 = v0.clone();
            let a1254 = f17(Str::lit("Rc"));
            if if equal(a1253, a1254) {
                let a1255 = v1.clone();
                let a1256 = f17(Str::lit("from"));
                equal(a1255, a1256)
            } else {
                false
            } {
                true
            } else {
                let a1257 = v0.clone();
                let a1258 = f17(Str::lit("Rc"));
                if if equal(a1257, a1258) {
                    let a1259 = v1.clone();
                    let a1260 = f17(Str::lit("try_unwrap"));
                    equal(a1259, a1260)
                } else {
                    false
                } {
                    true
                } else {
                    let a1261 = v0.clone();
                    let a1262 = f17(Str::lit("Rc"));
                    if if equal(a1261, a1262) {
                        let a1263 = v1.clone();
                        let a1264 = f17(Str::lit("get_mut"));
                        equal(a1263, a1264)
                    } else {
                        false
                    } {
                        true
                    } else {
                        let a1265 = v0.clone();
                        let a1266 = f17(Str::lit("Vec"));
                        if if equal(a1265, a1266) {
                            let a1267 = v1.clone();
                            let a1268 = f17(Str::lit("new"));
                            equal(a1267, a1268)
                        } else {
                            false
                        } {
                            true
                        } else {
                            let a1269 = v0.clone();
                            let a1270 = f17(Str::lit("Vec"));
                            if if equal(a1269, a1270) {
                                let a1271 = v1.clone();
                                let a1272 = f17(Str::lit("with_capacity"));
                                equal(a1271, a1272)
                            } else {
                                false
                            } {
                                true
                            } else {
                                let a1273 = v0.clone();
                                let a1274 = f17(Str::lit("String"));
                                if if equal(a1273, a1274) {
                                    let a1275 = v1.clone();
                                    let a1276 = f17(Str::lit("new"));
                                    equal(a1275, a1276)
                                } else {
                                    false
                                } {
                                    true
                                } else {
                                    let a1277 = v0.clone();
                                    let a1278 = f17(Str::lit("mem"));
                                    if if equal(a1277, a1278) {
                                        let a1279 = v1.clone();
                                        let a1280 = f17(Str::lit("replace"));
                                        equal(a1279, a1280)
                                    } else {
                                        false
                                    } {
                                        false
                                    } else {
                                        let a1281 = v0.clone();
                                        let a1282 = f17(Str::lit("str"));
                                        if if equal(a1281, a1282) {
                                            let a1283 = v1.clone();
                                            let a1284 = f17(Str::lit("from_utf8"));
                                            equal(a1283, a1284)
                                        } else {
                                            false
                                        } {
                                            false
                                        } else {
                                            let a1285 = v0.clone();
                                            let a1286 = f17(Str::lit("usize"));
                                            if if equal(a1285, a1286) {
                                                let a1287 = v1.clone();
                                                let a1288 = f17(Str::lit("from"));
                                                equal(a1287, a1288)
                                            } else {
                                                false
                                            } {
                                                false
                                            } else {
                                                let a1289 = v0;
                                                let a1290 = f17(Str::lit("usize"));
                                                let _ = if equal(a1289, a1290) {
                                                    let a1291 = v1;
                                                    let a1292 = f17(Str::lit("try_from"));
                                                    equal(a1291, a1292)
                                                } else {
                                                    false
                                                };
                                                false
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

pub fn f205(v0: Bytes, v1: Option<Adt13>, v2: Adt13) -> Option<Adt13> {
    let m1293 = v1;
    match m1293 {
        None => {
            if f32(match v2.clone() {
                Adt13::C0(h1294, _) => {
                    h1294
                }
            }, v0) {
                Some(v2)
            } else {
                None::<Adt13>
            }
        }
        Some(v3) => {
            Some(v3)
        }
    }
}

pub fn f206(v0: List<Adt9>, v1: Bytes) -> R<Option<u64>> {
    let m1295 = f158(v0, v1);
    match m1295 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            let m1296 = v2;
            match m1296 {
                Adt9::C0(_) => {
                    Ok(None::<u64>)
                }
                Adt9::C1(_, v5) => {
                    Ok(Some(f46(v5)?))
                }
                Adt9::C2(_, _, _) => {
                    Ok(None::<u64>)
                }
            }
        }
    }
}

pub fn f207(v0: Adt2, v1: u64) -> Adt2 {
    let m1297 = v0;
    match m1297 {
        Adt2::C0 => {
            Adt2::C16
        }
        Adt2::C1 => {
            Adt2::C16
        }
        Adt2::C2 => {
            Adt2::C16
        }
        Adt2::C3 => {
            Adt2::C16
        }
        Adt2::C4 => {
            Adt2::C16
        }
        Adt2::C5 => {
            Adt2::C16
        }
        Adt2::C6(_) => {
            Adt2::C16
        }
        Adt2::C7(_) => {
            Adt2::C16
        }
        Adt2::C8(_) => {
            Adt2::C16
        }
        Adt2::C9(r1302, r1303) => {
            let v5 = (*r1302).clone();
            let v6 = (*r1303).clone();
            let a1298 = v1;
            let a1299 = 0u64;
            if nat_eq(a1298, a1299) {
                v5
            } else {
                let a1300 = v1;
                let a1301 = 1u64;
                if nat_eq(a1300, a1301) {
                    v6
                } else {
                    Adt2::C16
                }
            }
        }
        Adt2::C10(_, _) => {
            Adt2::C16
        }
        Adt2::C11 => {
            Adt2::C16
        }
        Adt2::C12 => {
            Adt2::C16
        }
        Adt2::C13 => {
            Adt2::C16
        }
        Adt2::C14 => {
            Adt2::C16
        }
        Adt2::C15(_) => {
            Adt2::C16
        }
        Adt2::C16 => {
            Adt2::C16
        }
        Adt2::C17 => {
            Adt2::C16
        }
        Adt2::C18 => {
            Adt2::C16
        }
        Adt2::C19(_, _) => {
            Adt2::C16
        }
    }
}

pub fn f208(v0: Adt16, v1: Adt2) -> Adt2 {
    let m1304 = v1.clone();
    match m1304 {
        Adt2::C0 => {
            v1
        }
        Adt2::C1 => {
            v1
        }
        Adt2::C2 => {
            v1
        }
        Adt2::C3 => {
            v1
        }
        Adt2::C4 => {
            v1
        }
        Adt2::C5 => {
            match v0 {
                Adt16::C0(_, _, h1305, _) => {
                    h1305
                }
            }
        }
        Adt2::C6(_) => {
            v1
        }
        Adt2::C7(_) => {
            v1
        }
        Adt2::C8(_) => {
            v1
        }
        Adt2::C9(_, _) => {
            v1
        }
        Adt2::C10(_, _) => {
            v1
        }
        Adt2::C11 => {
            v1
        }
        Adt2::C12 => {
            v1
        }
        Adt2::C13 => {
            v1
        }
        Adt2::C14 => {
            v1
        }
        Adt2::C15(_) => {
            v1
        }
        Adt2::C16 => {
            v1
        }
        Adt2::C17 => {
            v1
        }
        Adt2::C18 => {
            v1
        }
        Adt2::C19(_, _) => {
            v1
        }
    }
}

pub fn f209(v0: Adt2) -> bool {
    let m1306 = v0;
    match m1306 {
        Adt2::C0 => {
            false
        }
        Adt2::C1 => {
            false
        }
        Adt2::C2 => {
            false
        }
        Adt2::C3 => {
            false
        }
        Adt2::C4 => {
            false
        }
        Adt2::C5 => {
            false
        }
        Adt2::C6(_) => {
            false
        }
        Adt2::C7(_) => {
            false
        }
        Adt2::C8(_) => {
            false
        }
        Adt2::C9(_, _) => {
            false
        }
        Adt2::C10(_, _) => {
            false
        }
        Adt2::C11 => {
            false
        }
        Adt2::C12 => {
            false
        }
        Adt2::C13 => {
            false
        }
        Adt2::C14 => {
            false
        }
        Adt2::C15(_) => {
            false
        }
        Adt2::C16 => {
            true
        }
        Adt2::C17 => {
            false
        }
        Adt2::C18 => {
            false
        }
        Adt2::C19(_, _) => {
            false
        }
    }
}

pub fn f210(v0: Bytes, v1: Adt14, v2: bool, v3: Bytes) -> bool {
    if v2 {
        true
    } else {
        let m1307 = f134(v1, v3.clone());
        match m1307 {
            None => {
                let m1309 = f135(v3);
                match m1309 {
                    None => {
                        false
                    }
                    Some(v5) => {
                        f32(v5, v0)
                    }
                }
            }
            Some(v4) => {
                f194(Fn27::F217(v0), false, match v4 {
                    Adt10::C0(_, h1308) => {
                        h1308
                    }
                })
            }
        }
    }
}

pub fn f211(p1315: Adt3) -> bool {
    let m1314 = p1315;
    {
        let mut m1314 = m1314;
        loop {
            let step = {
                let v0 = m1314;
                let m1310 = v0;
                match m1310 {
                    Adt3::C0(_) => {
                        Err::<Adt3, bool>(false)
                    }
                    Adt3::C1(_) => {
                        Err::<Adt3, bool>(false)
                    }
                    Adt3::C2(_) => {
                        Err::<Adt3, bool>(false)
                    }
                    Adt3::C3(_, _) => {
                        Err::<Adt3, bool>(false)
                    }
                    Adt3::C4(_, _, _) => {
                        Err::<Adt3, bool>(false)
                    }
                    Adt3::C5(_, _) => {
                        Err::<Adt3, bool>(false)
                    }
                    Adt3::C6(_, _) => {
                        Err::<Adt3, bool>(false)
                    }
                    Adt3::C7(_) => {
                        Err::<Adt3, bool>(false)
                    }
                    Adt3::C8(_, _) => {
                        Err::<Adt3, bool>(false)
                    }
                    Adt3::C9(_, _) => {
                        Err::<Adt3, bool>(false)
                    }
                    Adt3::C10(_, _, _) => {
                        Err::<Adt3, bool>(false)
                    }
                    Adt3::C11(_, _) => {
                        Err::<Adt3, bool>(false)
                    }
                    Adt3::C12(_, _, _) => {
                        Err::<Adt3, bool>(false)
                    }
                    Adt3::C13(_, _) => {
                        Err::<Adt3, bool>(false)
                    }
                    Adt3::C14(_, _, _) => {
                        Err::<Adt3, bool>(false)
                    }
                    Adt3::C15(_) => {
                        Err::<Adt3, bool>(false)
                    }
                    Adt3::C16(_, _, _) => {
                        Err::<Adt3, bool>(false)
                    }
                    Adt3::C17 => {
                        Err::<Adt3, bool>(false)
                    }
                    Adt3::C18(_) => {
                        Err::<Adt3, bool>(false)
                    }
                    Adt3::C19(_) => {
                        Err::<Adt3, bool>(false)
                    }
                    Adt3::C20(_, _, _) => {
                        Err::<Adt3, bool>(false)
                    }
                    Adt3::C21(v40, _, r1313) => {
                        let v42 = (*r1313).clone();
                        let a1311 = f179(v40);
                        let a1312 = f177(v42.clone());
                        if nat_lt(a1311, a1312) {
                            Ok::<Adt3, bool>(v42)
                        } else {
                            Err::<Adt3, bool>(false)
                        }
                    }
                    Adt3::C22(_) => {
                        Err::<Adt3, bool>(false)
                    }
                    Adt3::C23(_, _) => {
                        Err::<Adt3, bool>(true)
                    }
                    Adt3::C24(_, _) => {
                        Err::<Adt3, bool>(false)
                    }
                    Adt3::C25(_, _, _) => {
                        Err::<Adt3, bool>(false)
                    }
                    Adt3::C26(_) => {
                        Err::<Adt3, bool>(false)
                    }
                    Adt3::C27(_) => {
                        Err::<Adt3, bool>(false)
                    }
                    Adt3::C28(_, _, _, _) => {
                        Err::<Adt3, bool>(false)
                    }
                    Adt3::C29(_, _) => {
                        Err::<Adt3, bool>(false)
                    }
                    Adt3::C30 => {
                        Err::<Adt3, bool>(false)
                    }
                    Adt3::C31 => {
                        Err::<Adt3, bool>(false)
                    }
                    Adt3::C32(_, _) => {
                        Err::<Adt3, bool>(false)
                    }
                    Adt3::C33(_, _, _, _) => {
                        Err::<Adt3, bool>(false)
                    }
                    Adt3::C34(_, _, _, _) => {
                        Err::<Adt3, bool>(false)
                    }
                }
            };
            match step {
                Ok(next) => { m1314 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f212(v0: Fn30, p1319: u64, p1320: List<Bytes>) -> R<u64> {
    let m1318 = (p1319, p1320);
    {
        let mut m1318 = m1318;
        loop {
            let step = {
                let (v1, v2) = m1318;
                let m1316 = v2;
                match m1316.uncons() {
                    None => {
                        Err::<(u64, List<Bytes>), R<u64>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(u64, List<Bytes>), R<u64>>(({
                            let c1317: Fn30 = v0.clone();
                            c1317.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1318 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f213(v0: Bytes, v1: u64, v2: Bytes) -> R<u64> {
    if f32(v2, v0) {
        let a1321 = v1;
        let a1322 = 1u64;
        nat_add(a1321, a1322)
    } else {
        Ok(v1)
    }
}

pub fn f214(v0: Bytes, v1: u64) -> Option<u8> {
    let a1323 = v0;
    let a1324 = v1;
    index_bytes(a1323, a1324)
}

pub fn f215(v0: u8, v1: u8, v2: u8) -> bool {
    let m1327 = {
        let a1325 = v0;
        let a1326 = v1;
        fixed_u8::checked_sub(a1325, a1326)
    };
    match m1327 {
        None => {
            false
        }
        Some(_) => {
            let m1330 = {
                let a1328 = v2;
                let a1329 = v0;
                fixed_u8::checked_sub(a1328, a1329)
            };
            match m1330 {
                None => {
                    false
                }
                Some(_) => {
                    true
                }
            }
        }
    }
}

pub fn f216(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f218(v0, v1, 48u8) {
        Ok(Some({
            let a1331 = v1;
            let a1332 = 1u64;
            nat_add(a1331, a1332)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f217(v0: Bytes, v1: bool, v2: Adt5) -> bool {
    if v1 {
        true
    } else {
        f32(match v2 {
            Adt5::C0(h1333, _, _, _, _) => {
                h1333
            }
        }, v0)
    }
}

pub fn f218(v0: Bytes, v1: u64, v2: u8) -> bool {
    let m1334 = f214(v0, v1);
    match m1334 {
        None => {
            false
        }
        Some(v3) => {
            let a1335 = v3;
            let a1336 = v2;
            equal(a1335, a1336)
        }
    }
}

pub fn run(p0: Adt0) -> R<Result<(), Bytes>> {
    f0(p0)
}
