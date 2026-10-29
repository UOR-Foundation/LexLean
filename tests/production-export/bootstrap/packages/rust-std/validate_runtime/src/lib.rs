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
    F88,
    F115(Bytes),
    F132(List<Bytes>, List<Bytes>),
    F217(Bytes, Adt14),
}

impl Fn8 {
    pub fn apply(&self, p0: bool, p1: Bytes) -> R<bool> {
        match self {
            Fn8::F88 => Ok(f88(p0, p1)),
            Fn8::F115(k0) => Ok(f115(k0.clone(), p0, p1)),
            Fn8::F132(k0, k1) => f132(k0.clone(), k1.clone(), p0, p1),
            Fn8::F217(k0, k1) => Ok(f217(k0.clone(), k1.clone(), p0, p1)),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F90,
}

impl Fn9 {
    pub fn apply(&self, p0: Bytes, p1: Bytes) -> Bytes {
        match self {
            Fn9::F90 => f90(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F93(Str),
    F94(Str),
    F97(Str),
    F134(Adt7, Adt14),
    F147(Bytes),
    F186(List<Bytes>, List<Bytes>),
}

impl Fn10 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Bytes) -> R<Option<Bytes>> {
        match self {
            Fn10::F93(k0) => f93(k0.clone(), p0, p1),
            Fn10::F94(k0) => f94(k0.clone(), p0, p1),
            Fn10::F97(k0) => Ok(f97(k0.clone(), p0, p1)),
            Fn10::F134(k0, k1) => Ok(f134(k0.clone(), k1.clone(), p0, p1)),
            Fn10::F147(k0) => Ok(f147(k0.clone(), p0, p1)),
            Fn10::F186(k0, k1) => f186(k0.clone(), k1.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F99,
}

impl Fn11 {
    pub fn apply(&self, p0: Adt14, p1: Adt5) -> Adt14 {
        match self {
            Fn11::F99 => f99(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F103,
    F150(Bytes),
}

impl Fn12 {
    pub fn apply(&self, p0: Adt14, p1: Adt8) -> Adt14 {
        match self {
            Fn12::F103 => f103(p0, p1),
            Fn12::F150(k0) => f150(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F106(Bytes),
    F107,
}

impl Fn13 {
    pub fn apply(&self, p0: Adt18, p1: Adt9) -> R<Adt18> {
        match self {
            Fn13::F106(k0) => f106(k0.clone(), p0, p1),
            Fn13::F107 => f107(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F114(Bytes),
}

impl Fn14 {
    pub fn apply(&self, p0: Option<Adt17>, p1: Adt17) -> Option<Adt17> {
        match self {
            Fn14::F114(k0) => f114(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F144(Bytes),
}

impl Fn15 {
    pub fn apply(&self, p0: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
        match self {
            Fn15::F144(k0) => f144(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F149(Bytes),
    F207(Bytes),
}

impl Fn16 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn16::F149(k0) => f149(k0.clone(), p0),
            Fn16::F207(k0) => f207(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F156(Bytes),
}

impl Fn17 {
    pub fn apply(&self, p0: Option<Adt11>, p1: Adt11) -> Option<Adt11> {
        match self {
            Fn17::F156(k0) => f156(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F158(Bytes),
}

impl Fn18 {
    pub fn apply(&self, p0: Option<Adt12>, p1: Adt12) -> Option<Adt12> {
        match self {
            Fn18::F158(k0) => f158(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F160(Bytes),
    F212(Bytes),
}

impl Fn19 {
    pub fn apply(&self, p0: Option<Adt13>, p1: Adt13) -> Option<Adt13> {
        match self {
            Fn19::F160(k0) => f160(k0.clone(), p0, p1),
            Fn19::F212(k0) => f212(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn20 {
    F173(Bytes),
}

impl Fn20 {
    pub fn apply(&self, p0: List<u64>, p1: Adt13) -> List<u64> {
        match self {
            Fn20::F173(k0) => f173(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn21 {
    F175(u64),
}

impl Fn21 {
    pub fn apply(&self, p0: bool, p1: u64) -> bool {
        match self {
            Fn21::F175(k0) => f175(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn22 {
    F181,
}

impl Fn22 {
    pub fn apply(&self, p0: Bytes, p1: u64) -> R<Bytes> {
        match self {
            Fn22::F181 => f181(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn23 {
    F189(bool, u64, Adt14),
}

impl Fn23 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Adt6) -> R<Option<Bytes>> {
        match self {
            Fn23::F189(k0, k1, k2) => f189(*k0, *k1, k2.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn24 {
    F191(Bytes),
}

impl Fn24 {
    pub fn apply(&self, p0: Option<Adt10>, p1: Adt10) -> Option<Adt10> {
        match self {
            Fn24::F191(k0) => f191(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn25 {
    F193(Bytes),
}

impl Fn25 {
    pub fn apply(&self, p0: u64, p1: Adt8) -> R<u64> {
        match self {
            Fn25::F193(k0) => f193(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn26 {
    F195(Bytes),
}

impl Fn26 {
    pub fn apply(&self, p0: Option<Adt8>, p1: Adt8) -> Option<Adt8> {
        match self {
            Fn26::F195(k0) => f195(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn27 {
    F197,
}

impl Fn27 {
    pub fn apply(&self, p0: Bytes, p1: Adt6) -> R<Bytes> {
        match self {
            Fn27::F197 => f197(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn28 {
    F200(Bytes),
    F224(Bytes),
}

impl Fn28 {
    pub fn apply(&self, p0: bool, p1: Adt5) -> bool {
        match self {
            Fn28::F200(k0) => f200(k0.clone(), p0, p1),
            Fn28::F224(k0) => f224(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn29 {
    F202(Bytes),
}

impl Fn29 {
    pub fn apply(&self, p0: Result<List<Adt17>, Bytes>, p1: Adt6) -> Result<List<Adt17>, Bytes> {
        match self {
            Fn29::F202(k0) => f202(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn30 {
    F205,
}

impl Fn30 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn30::F205 => f205(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn31 {
    F209(Bytes),
}

impl Fn31 {
    pub fn apply(&self, p0: Option<Adt9>, p1: Adt9) -> Option<Adt9> {
        match self {
            Fn31::F209(k0) => f209(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn32 {
    F220(Bytes),
}

impl Fn32 {
    pub fn apply(&self, p0: u64, p1: Bytes) -> R<u64> {
        match self {
            Fn32::F220(k0) => f220(k0.clone(), p0, p1),
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
            f28(v39)
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

pub fn f28(v0: Bytes) -> R<Option<Bytes>> {
    let a332 = {
        let a331 = v0.clone();
        length_bytes(a331)
    };
    let a333 = 0u64;
    if nat_eq(a332, a333) {
        Ok(Some(f17(Str::lit("runtime support: defined-path: empty"))))
    } else {
        f37(Str::lit("a defined path segment"), f85(v0)?)
    }
}

pub fn f29(v0: Bytes, v1: Adt14, v2: Option<Bytes>, v3: Adt1) -> R<Option<Bytes>> {
    let m334 = v2;
    match m334 {
        None => {
            f86(v1, v0, v3)
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
                    if f87(Fn8::F88, false, v1.clone())? {
                        Ok(Some(f16(f17(Str::lit("runtime support: attribute-args: ")), f16(f18(v0), f17(Str::lit(" takes a lint with a name"))))))
                    } else {
                        f37(Str::lit("a lint name"), v1)
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
    f89(Fn9::F90, f17(Str::lit("")), v0)
}

pub fn f34(v0: List<Bytes>, v1: Str) -> R<Bytes> {
    f91(v0, f17(v1))
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
    f92(Fn10::F93(v0), None::<Bytes>, v1)
}

pub fn f37(v0: Str, v1: List<Bytes>) -> R<Option<Bytes>> {
    f92(Fn10::F94(v0), None::<Bytes>, v1)
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
    if if f95(v1.clone())? {
        let a343 = f53(f96(), v1.clone())?;
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
    f92(Fn10::F97(v0), None::<Bytes>, v1)
}

pub fn f42(v0: Adt14, v1: List<Adt5>) -> Adt14 {
    f98(Fn11::F99, v0, v1)
}

pub fn f43(v0: Adt14, v1: Adt2, v2: List<Adt8>) -> Adt14 {
    let m357 = f100(v1);
    match m357 {
        None => {
            v0
        }
        Some(v3) => {
            f101(v0, v3, v2)
        }
    }
}

pub fn f44(v0: Adt14, v1: List<Adt8>) -> Adt14 {
    f102(Fn12::F103, v0, v1)
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
    f104(v0)
}

pub fn f48(v0: Bytes, v1: List<Adt9>) -> R<Option<Bytes>> {
    let m367 = f105(Fn13::F106(v0), Adt18::C0(List::<Bytes>::nil(), None::<Bytes>), v1)?;
    match m367 {
        Adt18::C0(_, h366) => {
            Ok(h366)
        }
    }
}

pub fn f49(v0: List<Adt9>) -> R<Option<Bytes>> {
    let m369 = f105(Fn13::F107, Adt18::C0(List::<Bytes>::nil(), None::<Bytes>), v0)?;
    match m369 {
        Adt18::C0(_, h368) => {
            Ok(h368)
        }
    }
}

pub fn f50(v0: Adt14, v1: Bytes) -> Option<u64> {
    let m370 = f108(v0.clone(), v1.clone());
    match m370 {
        None => {
            let m372 = f109(v0.clone(), v1.clone());
            match m372 {
                None => {
                    let m374 = f110(v0, v1.clone());
                    match m374 {
                        None => {
                            f111(v1)
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
    let m376 = f108(v0.clone(), v1.clone());
    match m376 {
        None => {
            let m377 = f109(v0.clone(), v1.clone());
            match m377 {
                None => {
                    let m378 = f110(v0, v1.clone());
                    match m378 {
                        None => {
                            f112(v1)
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
    f113(Fn14::F114(v1), None::<Adt17>, v0)
}

pub fn f53(v0: List<Bytes>, v1: Bytes) -> R<bool> {
    f87(Fn8::F115(v1), false, v0)
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
            f116(v0, v1)
        }
        Some(v2) => {
            Ok(Some(v2))
        }
    }
}

pub fn f56(v0: Adt14, v1: List<Bytes>, v2: u64) -> R<Option<Bytes>> {
    if f117(v0.clone(), v1.clone()) {
        Ok(Some(f16(f17(Str::lit("runtime support: hidden-allocation: ")), f16(f17(Str::lit("rust-core renders ")), f16(f18(f34(v1, Str::lit("::"))?), f17(Str::lit(", which requires heap allocation")))))))
    } else {
        let m382 = f118(v0, v1.clone())?;
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
    let m395 = f119(v3.clone());
    match m395 {
        None => {
            let v6: List<u64> = f121(v0.clone(), v3.clone());
            let m399 = v6.clone();
            match m399.uncons() {
                None => {
                    Ok(Some(f16(f17(Str::lit("runtime support: method-name: ")), f16(f18(v3), f17(Str::lit(" is a method of no declared trait or impl and not in the closed table"))))))
                }
                Some((_, _)) => {
                    if f122(v6.clone(), v4) {
                        f123(v0, v1, v2, v3)
                    } else {
                        Ok(Some(f16(f17(Str::lit("runtime support: method-arity: ")), f16(f18(v3), f16(f17(Str::lit(" takes ")), f16(f124(v6)?, f16(f17(Str::lit(" arguments, given ")), f47(v4)?)))))))
                    }
                }
            }
        }
        Some(v5) => {
            if if f120(v3.clone()) {
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
    if f125(v0)? {
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
                                    if f126(match v3 {
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
            if f127(v6.clone()) {
                f66(v0, v7)
            } else {
                let m433 = f55(v0.clone(), v6.clone())?;
                match m433 {
                    None => {
                        let m434 = f128(v0.clone(), v6, v7.clone())?;
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
                            let a439 = f129(v12.clone())?;
                            if nat_eq(a438, a439) {
                                let m440 = f130(v0.clone(), v10, v11)?;
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
                    let m461 = f108(v0, v2);
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
                            let m464 = f109(v0, v2);
                            match m464 {
                                None => {
                                    None::<List<Bytes>>
                                }
                                Some(v7) => {
                                    f131(match v7 {
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
            f87(Fn8::F132(v1.clone(), v0), true, v1)
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
                let m530 = f133(v0.clone(), match v1.clone() {
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
                    f92(Fn10::F134(v2.clone(), v0), None::<Bytes>, match v2 {
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
    let m541 = f135(v0, v1.clone());
    match m541 {
        None => {
            let m543 = f136(v1.clone());
            match m543 {
                None => {
                    None::<List<Adt5>>
                }
                Some(v3) => {
                    Some(List::cons(Adt5::C0(v3, true, f137(v1.clone()), List::<Adt6>::nil(), if f137(v1) {
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
            let v5: u64 = f138(v0.clone(), match v3.clone() {
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
                    let m552 = f139(v0, match v3.clone() {
                        Adt5::C0(h551, _, _, _, _) => {
                            h551
                        }
                    });
                    match m552 {
                        None => {
                            Ok(None::<Bytes>)
                        }
                        Some(v6) => {
                            let a554 = f140(match v6 {
                                Adt8::C0(h553, _, _, _) => {
                                    h553
                                }
                            })?;
                            let a555 = f140(v3.clone())?;
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
            let a562 = f141(v2, {
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
                            let m586 = f142(Ok::<List<Adt17>, Bytes>(v10), match v7.clone() {
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

pub fn f85(v0: Bytes) -> R<List<Bytes>> {
    let v1: u64 = {
        let a588 = v0.clone();
        length_bytes(a588)
    };
    let v2: (u64, (u64, List<Bytes>)) = {
        let (h589, _) = f143(Fn15::F144(v0.clone()), v1, (0u64, (0u64, List::<Bytes>::nil())))?;
        h589
    };
    let v3: u64 = {
        let (h590, _) = {
            let (_, h591) = v2.clone();
            h591
        };
        h590
    };
    Ok(f145(List::cons(f146(v0, v3, {
        let a592 = v1;
        let a593 = v3;
        nat_sub(a592, a593)
    }), {
        let (_, h594) = {
            let (_, h595) = v2;
            h595
        };
        h594
    })))
}

pub fn f86(v0: Adt14, v1: Bytes, v2: Adt1) -> R<Option<Bytes>> {
    let m596 = v2;
    match m596 {
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
            f28(v3)
        }
        Adt1::C11(_, _) => {
            Ok(Some(f16(f17(Str::lit("runtime support: module-member: ")), f16(f18(v1), f17(Str::lit(" holds an item that is neither a defined function nor a function"))))))
        }
        Adt1::C12(_, _) => {
            Ok(Some(f16(f17(Str::lit("runtime support: module-member: ")), f16(f18(v1), f17(Str::lit(" holds an item that is neither a defined function nor a function"))))))
        }
    }
}

pub fn f87(v0: Fn8, p600: bool, p601: List<Bytes>) -> R<bool> {
    let m599 = (p600, p601);
    {
        let mut m599 = m599;
        loop {
            let step = {
                let (v1, v2) = m599;
                let m597 = v2;
                match m597.uncons() {
                    None => {
                        Err::<(bool, List<Bytes>), R<bool>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<Bytes>), R<bool>>(({
                            let c598: Fn8 = v0.clone();
                            c598.apply(v1, v3)?
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

pub fn f88(v0: bool, v1: Bytes) -> bool {
    if v0 {
        true
    } else {
        let a603 = {
            let a602 = v1;
            length_bytes(a602)
        };
        let a604 = 0u64;
        nat_eq(a603, a604)
    }
}

pub fn f89(v0: Fn9, p608: Bytes, p609: List<Bytes>) -> Bytes {
    let m607 = (p608, p609);
    {
        let mut m607 = m607;
        loop {
            let step = {
                let (v1, v2) = m607;
                let m605 = v2;
                match m605.uncons() {
                    None => {
                        Err::<(Bytes, List<Bytes>), Bytes>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Bytes, List<Bytes>), Bytes>(({
                            let c606: Fn9 = v0.clone();
                            c606.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m607 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f90(_: Bytes, v1: Bytes) -> Bytes {
    v1
}

pub fn f91(v0: List<Bytes>, v1: Bytes) -> R<Bytes> {
    let m610 = f92(Fn10::F147(v1), None::<Bytes>, v0)?;
    match m610 {
        None => {
            Ok(Bytes::lit(&[]))
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f92(v0: Fn10, p614: Option<Bytes>, p615: List<Bytes>) -> R<Option<Bytes>> {
    let m613 = (p614, p615);
    {
        let mut m613 = m613;
        loop {
            let step = {
                let (v1, v2) = m613;
                let m611 = v2;
                match m611.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Bytes>), R<Option<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Bytes>), R<Option<Bytes>>>(({
                            let c612: Fn10 = v0.clone();
                            c612.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m613 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f93(v0: Str, v1: Option<Bytes>, v2: Bytes) -> R<Option<Bytes>> {
    let m616 = v1;
    match m616 {
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

pub fn f94(v0: Str, v1: Option<Bytes>, v2: Bytes) -> R<Option<Bytes>> {
    let m617 = v1;
    match m617 {
        None => {
            f39(v0, v2)
        }
        Some(v3) => {
            Ok(Some(v3))
        }
    }
}

pub fn f95(v0: Bytes) -> R<bool> {
    let a619 = 0u64;
    let a620 = {
        let a618 = v0.clone();
        length_bytes(a618)
    };
    if nat_lt(a619, a620) {
        let a624 = {
            let (h621, _) = f148(Fn16::F149(v0.clone()), {
                let a622 = v0.clone();
                length_bytes(a622)
            }, 0u64)?;
            h621
        };
        let a625 = {
            let a623 = v0;
            length_bytes(a623)
        };
        Ok(nat_eq(a624, a625))
    } else {
        Ok(false)
    }
}

pub fn f96() -> List<Bytes> {
    List::cons(f17(Str::lit("abstract")), List::cons(f17(Str::lit("as")), List::cons(f17(Str::lit("async")), List::cons(f17(Str::lit("await")), List::cons(f17(Str::lit("become")), List::cons(f17(Str::lit("box")), List::cons(f17(Str::lit("break")), List::cons(f17(Str::lit("const")), List::cons(f17(Str::lit("continue")), List::cons(f17(Str::lit("crate")), List::cons(f17(Str::lit("do")), List::cons(f17(Str::lit("dyn")), List::cons(f17(Str::lit("else")), List::cons(f17(Str::lit("enum")), List::cons(f17(Str::lit("extern")), List::cons(f17(Str::lit("false")), List::cons(f17(Str::lit("final")), List::cons(f17(Str::lit("fn")), List::cons(f17(Str::lit("for")), List::cons(f17(Str::lit("if")), List::cons(f17(Str::lit("impl")), List::cons(f17(Str::lit("in")), List::cons(f17(Str::lit("let")), List::cons(f17(Str::lit("loop")), List::cons(f17(Str::lit("macro")), List::cons(f17(Str::lit("match")), List::cons(f17(Str::lit("mod")), List::cons(f17(Str::lit("move")), List::cons(f17(Str::lit("mut")), List::cons(f17(Str::lit("override")), List::cons(f17(Str::lit("priv")), List::cons(f17(Str::lit("pub")), List::cons(f17(Str::lit("ref")), List::cons(f17(Str::lit("return")), List::cons(f17(Str::lit("self")), List::cons(f17(Str::lit("Self")), List::cons(f17(Str::lit("static")), List::cons(f17(Str::lit("struct")), List::cons(f17(Str::lit("super")), List::cons(f17(Str::lit("trait")), List::cons(f17(Str::lit("true")), List::cons(f17(Str::lit("try")), List::cons(f17(Str::lit("type")), List::cons(f17(Str::lit("typeof")), List::cons(f17(Str::lit("unsafe")), List::cons(f17(Str::lit("unsized")), List::cons(f17(Str::lit("use")), List::cons(f17(Str::lit("virtual")), List::cons(f17(Str::lit("where")), List::cons(f17(Str::lit("while")), List::cons(f17(Str::lit("yield")), List::cons(f17(Str::lit("_")), List::<Bytes>::nil()))))))))))))))))))))))))))))))))))))))))))))))))))))
}

pub fn f97(v0: Str, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m626 = v1;
    match m626 {
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

pub fn f98(v0: Fn11, p630: Adt14, p631: List<Adt5>) -> Adt14 {
    let m629 = (p630, p631);
    {
        let mut m629 = m629;
        loop {
            let step = {
                let (v1, v2) = m629;
                let m627 = v2;
                match m627.uncons() {
                    None => {
                        Err::<(Adt14, List<Adt5>), Adt14>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt14, List<Adt5>), Adt14>(({
                            let c628: Fn11 = v0.clone();
                            c628.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m629 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f99(v0: Adt14, v1: Adt5) -> Adt14 {
    Adt14::C0(match v0.clone() {
        Adt14::C0(h632, _, _, _, _, _, _, _, _, _) => {
            h632
        }
    }, match v0.clone() {
        Adt14::C0(_, h633, _, _, _, _, _, _, _, _) => {
            h633
        }
    }, match v0.clone() {
        Adt14::C0(_, _, h634, _, _, _, _, _, _, _) => {
            h634
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, h635, _, _, _, _, _, _) => {
            h635
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, h636, _, _, _, _, _) => {
            h636
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, h637, _, _, _, _) => {
            h637
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, h638, _, _, _) => {
            h638
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, h639, _, _) => {
            h639
        }
    }, {
        let a644 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, h640, _) => {
                h640
            }
        };
        let a645 = List::cons(Adt13::C0(match v1.clone() {
            Adt5::C0(h641, _, _, _, _) => {
                h641
            }
        }, {
            let a643 = match v1 {
                Adt5::C0(_, _, _, h642, _) => {
                    h642
                }
            };
            length_list(a643)
        }), List::<Adt13>::nil());
        append_list(a644, a645)
    }, match v0 {
        Adt14::C0(_, _, _, _, _, _, _, _, _, h646) => {
            h646
        }
    })
}

pub fn f100(v0: Adt2) -> Option<Bytes> {
    let m647 = v0;
    match m647 {
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

pub fn f101(v0: Adt14, v1: Bytes, v2: List<Adt8>) -> Adt14 {
    f102(Fn12::F150(v1), v0, v2)
}

pub fn f102(v0: Fn12, p651: Adt14, p652: List<Adt8>) -> Adt14 {
    let m650 = (p651, p652);
    {
        let mut m650 = m650;
        loop {
            let step = {
                let (v1, v2) = m650;
                let m648 = v2;
                match m648.uncons() {
                    None => {
                        Err::<(Adt14, List<Adt8>), Adt14>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt14, List<Adt8>), Adt14>(({
                            let c649: Fn12 = v0.clone();
                            c649.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m650 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f103(v0: Adt14, v1: Adt8) -> Adt14 {
    Adt14::C0(match v0.clone() {
        Adt14::C0(h653, _, _, _, _, _, _, _, _, _) => {
            h653
        }
    }, match v0.clone() {
        Adt14::C0(_, h654, _, _, _, _, _, _, _, _) => {
            h654
        }
    }, match v0.clone() {
        Adt14::C0(_, _, h655, _, _, _, _, _, _, _) => {
            h655
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, h656, _, _, _, _, _, _) => {
            h656
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, h657, _, _, _, _, _) => {
            h657
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, h658, _, _, _, _) => {
            h658
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, h659, _, _, _) => {
            h659
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, h660, _, _) => {
            h660
        }
    }, {
        let a669 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, h661, _) => {
                h661
            }
        };
        let a670 = List::cons(Adt13::C0({
            let m664 = match v1.clone() {
                Adt8::C0(h663, _, _, _) => {
                    h663
                }
            };
            match m664 {
                Adt5::C0(h662, _, _, _, _) => {
                    h662
                }
            }
        }, {
            let a668 = {
                let m667 = match v1 {
                    Adt8::C0(h666, _, _, _) => {
                        h666
                    }
                };
                match m667 {
                    Adt5::C0(_, _, _, h665, _) => {
                        h665
                    }
                }
            };
            length_list(a668)
        }), List::<Adt13>::nil());
        append_list(a669, a670)
    }, match v0 {
        Adt14::C0(_, _, _, _, _, _, _, _, _, h671) => {
            h671
        }
    })
}

pub fn f104(v0: u64) -> R<Bytes> {
    let v1: Bytes = f151(Str::lit("0"));
    let v2: Bytes = f152({
        let a708 = f153(v0, 10000000000000000000u64);
        let a709 = {
            let a706 = f153(v0, 1000000000000000000u64);
            let a707 = {
                let a704 = f153(v0, 100000000000000000u64);
                let a705 = {
                    let a702 = f153(v0, 10000000000000000u64);
                    let a703 = {
                        let a700 = f153(v0, 1000000000000000u64);
                        let a701 = {
                            let a698 = f153(v0, 100000000000000u64);
                            let a699 = {
                                let a696 = f153(v0, 10000000000000u64);
                                let a697 = {
                                    let a694 = f153(v0, 1000000000000u64);
                                    let a695 = {
                                        let a692 = f153(v0, 100000000000u64);
                                        let a693 = {
                                            let a690 = f153(v0, 10000000000u64);
                                            let a691 = {
                                                let a688 = f153(v0, 1000000000u64);
                                                let a689 = {
                                                    let a686 = f153(v0, 100000000u64);
                                                    let a687 = {
                                                        let a684 = f153(v0, 10000000u64);
                                                        let a685 = {
                                                            let a682 = f153(v0, 1000000u64);
                                                            let a683 = {
                                                                let a680 = f153(v0, 100000u64);
                                                                let a681 = {
                                                                    let a678 = f153(v0, 10000u64);
                                                                    let a679 = {
                                                                        let a676 = f153(v0, 1000u64);
                                                                        let a677 = {
                                                                            let a674 = f153(v0, 100u64);
                                                                            let a675 = {
                                                                                let a672 = f153(v0, 10u64);
                                                                                let a673 = f153(v0, 1u64);
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
                    };
                    append_bytes(a702, a703)
                };
                append_bytes(a704, a705)
            };
            append_bytes(a706, a707)
        };
        append_bytes(a708, a709)
    })?;
    let a710 = v0;
    let a711 = 0u64;
    if nat_eq(a710, a711) {
        Ok(v1)
    } else {
        Ok(v2)
    }
}

pub fn f105(v0: Fn13, p715: Adt18, p716: List<Adt9>) -> R<Adt18> {
    let m714 = (p715, p716);
    {
        let mut m714 = m714;
        loop {
            let step = {
                let (v1, v2) = m714;
                let m712 = v2;
                match m712.uncons() {
                    None => {
                        Err::<(Adt18, List<Adt9>), R<Adt18>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt18, List<Adt9>), R<Adt18>>(({
                            let c713: Fn13 = v0.clone();
                            c713.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m714 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f106(v0: Bytes, v1: Adt18, v2: Adt9) -> R<Adt18> {
    let m718 = match v1.clone() {
        Adt18::C0(_, h717) => {
            h717
        }
    };
    match m718 {
        None => {
            let m719 = f39(Str::lit("a variant name"), f154(v2.clone()))?;
            match m719 {
                None => {
                    let m722 = {
                        let m721 = v2.clone();
                        match m721 {
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
                    match m722 {
                        None => {
                            if f53(match v1.clone() {
                                Adt18::C0(h724, _) => {
                                    h724
                                }
                            }, f154(v2.clone()))? {
                                Ok(Adt18::C0(match v1 {
                                    Adt18::C0(h725, _) => {
                                        h725
                                    }
                                }, Some(f16(f17(Str::lit("runtime support: variant-name: ")), f16(f18(v0), f16(f17(Str::lit(" declares ")), f16(f18(f154(v2)), f17(Str::lit(" twice")))))))))
                            } else {
                                Ok(Adt18::C0(List::cons(f154(v2), match v1 {
                                    Adt18::C0(h726, _) => {
                                        h726
                                    }
                                }), None::<Bytes>))
                            }
                        }
                        Some(v11) => {
                            Ok(Adt18::C0(match v1 {
                                Adt18::C0(h723, _) => {
                                    h723
                                }
                            }, Some(v11)))
                        }
                    }
                }
                Some(v4) => {
                    Ok(Adt18::C0(match v1 {
                        Adt18::C0(h720, _) => {
                            h720
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

pub fn f107(v0: Adt18, v1: Adt9) -> R<Adt18> {
    let m728 = match v0.clone() {
        Adt18::C0(_, h727) => {
            h727
        }
    };
    match m728 {
        None => {
            if f53(match v0.clone() {
                Adt18::C0(h729, _) => {
                    h729
                }
            }, f154(v1.clone()))? {
                Ok(Adt18::C0(match v0 {
                    Adt18::C0(h730, _) => {
                        h730
                    }
                }, Some(f154(v1))))
            } else {
                Ok(Adt18::C0(List::cons(f154(v1), match v0 {
                    Adt18::C0(h731, _) => {
                        h731
                    }
                }), None::<Bytes>))
            }
        }
        Some(_) => {
            Ok(v0)
        }
    }
}

pub fn f108(v0: Adt14, v1: Bytes) -> Option<Adt11> {
    f155(Fn17::F156(v1), None::<Adt11>, match v0 {
        Adt14::C0(_, _, _, h732, _, _, _, _, _, _) => {
            h732
        }
    })
}

pub fn f109(v0: Adt14, v1: Bytes) -> Option<Adt12> {
    f157(Fn18::F158(v1), None::<Adt12>, match v0 {
        Adt14::C0(_, _, _, _, h733, _, _, _, _, _) => {
            h733
        }
    })
}

pub fn f110(v0: Adt14, v1: Bytes) -> Option<Adt13> {
    f159(Fn19::F160(v1), None::<Adt13>, match v0 {
        Adt14::C0(_, _, _, _, _, h734, _, _, _, _) => {
            h734
        }
    })
}

pub fn f111(v0: Bytes) -> Option<u64> {
    let a735 = v0.clone();
    let a736 = f17(Str::lit("Result"));
    if equal(a735, a736) {
        Some(2u64)
    } else {
        let a737 = v0.clone();
        let a738 = f17(Str::lit("Option"));
        if equal(a737, a738) {
            Some(1u64)
        } else {
            let a739 = v0.clone();
            let a740 = f17(Str::lit("AtomicU64"));
            if equal(a739, a740) {
                Some(0u64)
            } else {
                let a741 = v0.clone();
                let a742 = f17(Str::lit("Rc"));
                if equal(a741, a742) {
                    Some(1u64)
                } else {
                    let a743 = v0.clone();
                    let a744 = f17(Str::lit("Vec"));
                    if equal(a743, a744) {
                        Some(1u64)
                    } else {
                        let a745 = v0;
                        let a746 = f17(Str::lit("String"));
                        if equal(a745, a746) {
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

pub fn f112(v0: Bytes) -> bool {
    let a747 = v0.clone();
    let a748 = f17(Str::lit("Rc"));
    if equal(a747, a748) {
        true
    } else {
        let a749 = v0.clone();
        let a750 = f17(Str::lit("Vec"));
        if equal(a749, a750) {
            true
        } else {
            let a751 = v0;
            let a752 = f17(Str::lit("String"));
            equal(a751, a752)
        }
    }
}

pub fn f113(v0: Fn14, p756: Option<Adt17>, p757: List<Adt17>) -> Option<Adt17> {
    let m755 = (p756, p757);
    {
        let mut m755 = m755;
        loop {
            let step = {
                let (v1, v2) = m755;
                let m753 = v2;
                match m753.uncons() {
                    None => {
                        Err::<(Option<Adt17>, List<Adt17>), Option<Adt17>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Adt17>, List<Adt17>), Option<Adt17>>(({
                            let c754: Fn14 = v0.clone();
                            c754.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m755 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f114(v0: Bytes, v1: Option<Adt17>, v2: Adt17) -> Option<Adt17> {
    let m758 = v1;
    match m758 {
        None => {
            if f32(match v2.clone() {
                Adt17::C0(h759, _, _) => {
                    h759
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

pub fn f115(v0: Bytes, v1: bool, v2: Bytes) -> bool {
    if v1 {
        true
    } else {
        f32(v2, v0)
    }
}

pub fn f116(v0: Adt14, v1: List<Bytes>) -> R<Option<Bytes>> {
    let v2: Bytes = f161(v1.clone());
    let m760 = f109(v0.clone(), v2.clone());
    match m760 {
        None => {
            if if f53(match v0.clone() {
                Adt14::C0(_, _, _, _, _, _, _, _, _, h764) => {
                    h764
                }
            }, v2.clone())? {
                true
            } else {
                let m765 = f108(v0.clone(), v2.clone());
                if match m765 {
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
                        Adt14::C0(_, _, _, _, _, _, h766, _, _, _) => {
                            h766
                        }
                    }, v2.clone())? {
                        true
                    } else {
                        if f164(v2.clone()) {
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
            let m761 = f162(v1.clone(), 1u64);
            match m761 {
                None => {
                    Ok(Some(f16(f17(Str::lit("runtime support: variant-name: ")), f16(f18(f34(v1, Str::lit("::"))?), f16(f17(Str::lit(" is not a variant of ")), f18(v2))))))
                }
                Some(v4) => {
                    let m763 = f163(match v3 {
                        Adt12::C0(_, h762, _) => {
                            h762
                        }
                    }, v4);
                    match m763 {
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

pub fn f117(v0: Adt14, v1: List<Bytes>) -> bool {
    if match v0 {
        Adt14::C0(h767, _, _, _, _, _, _, _, _, _) => {
            h767
        }
    } {
        let m768 = v1;
        match m768.uncons() {
            None => {
                false
            }
            Some((v2, v3)) => {
                let m769 = v3;
                match m769.uncons() {
                    None => {
                        false
                    }
                    Some((v4, v5)) => {
                        let m770 = v5;
                        match m770.uncons() {
                            None => {
                                f165(v2, v4)
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

pub fn f118(v0: Adt14, v1: List<Bytes>) -> R<Option<u64>> {
    let m771 = v1;
    match m771.uncons() {
        None => {
            Ok(None::<u64>)
        }
        Some((v2, v3)) => {
            let m772 = v3;
            match m772.uncons() {
                None => {
                    let m773 = f166(v0.clone(), v2.clone());
                    match m773 {
                        None => {
                            let m775 = f167(v2.clone());
                            match m775 {
                                None => {
                                    Ok(f168(v0, v2))
                                }
                                Some(v5) => {
                                    Ok(Some(v5))
                                }
                            }
                        }
                        Some(v4) => {
                            Ok(Some(match v4 {
                                Adt13::C0(_, h774) => {
                                    h774
                                }
                            }))
                        }
                    }
                }
                Some((v6, v7)) => {
                    let m776 = v7;
                    match m776.uncons() {
                        None => {
                            let m777 = f169(v2.clone(), v6.clone());
                            match m777 {
                                None => {
                                    let m778 = f170(v0.clone(), v2.clone(), v6.clone())?;
                                    match m778 {
                                        None => {
                                            Ok(f171(v0, v2, v6))
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

pub fn f119(v0: Bytes) -> Option<u64> {
    let a779 = v0.clone();
    let a780 = f17(Str::lit("fetch_add"));
    if equal(a779, a780) {
        Some(2u64)
    } else {
        let a781 = v0.clone();
        let a782 = f17(Str::lit("load"));
        if equal(a781, a782) {
            Some(1u64)
        } else {
            let a783 = v0.clone();
            let a784 = f17(Str::lit("cmp"));
            if equal(a783, a784) {
                Some(1u64)
            } else {
                let a785 = v0.clone();
                let a786 = f17(Str::lit("len"));
                if equal(a785, a786) {
                    Some(0u64)
                } else {
                    let a787 = v0.clone();
                    let a788 = f17(Str::lit("is_empty"));
                    if equal(a787, a788) {
                        Some(0u64)
                    } else {
                        let a789 = v0.clone();
                        let a790 = f17(Str::lit("chars"));
                        if equal(a789, a790) {
                            Some(0u64)
                        } else {
                            let a791 = v0.clone();
                            let a792 = f17(Str::lit("next"));
                            if equal(a791, a792) {
                                Some(0u64)
                            } else {
                                let a793 = v0.clone();
                                let a794 = f17(Str::lit("min"));
                                if equal(a793, a794) {
                                    Some(1u64)
                                } else {
                                    let a795 = v0.clone();
                                    let a796 = f17(Str::lit("max"));
                                    if equal(a795, a796) {
                                        Some(1u64)
                                    } else {
                                        let a797 = v0.clone();
                                        let a798 = f17(Str::lit("push"));
                                        if equal(a797, a798) {
                                            Some(1u64)
                                        } else {
                                            let a799 = v0.clone();
                                            let a800 = f17(Str::lit("pop"));
                                            if equal(a799, a800) {
                                                Some(0u64)
                                            } else {
                                                let a801 = v0.clone();
                                                let a802 = f17(Str::lit("truncate"));
                                                if equal(a801, a802) {
                                                    Some(1u64)
                                                } else {
                                                    let a803 = v0.clone();
                                                    let a804 = f17(Str::lit("push_str"));
                                                    if equal(a803, a804) {
                                                        Some(1u64)
                                                    } else {
                                                        let a805 = v0.clone();
                                                        let a806 = f17(Str::lit("extend_from_slice"));
                                                        if equal(a805, a806) {
                                                            Some(1u64)
                                                        } else {
                                                            let a807 = v0.clone();
                                                            let a808 = f17(Str::lit("iter"));
                                                            if equal(a807, a808) {
                                                                Some(0u64)
                                                            } else {
                                                                let a809 = v0.clone();
                                                                let a810 = f17(Str::lit("rev"));
                                                                if equal(a809, a810) {
                                                                    Some(0u64)
                                                                } else {
                                                                    let a811 = v0.clone();
                                                                    let a812 = f17(Str::lit("get"));
                                                                    if equal(a811, a812) {
                                                                        Some(1u64)
                                                                    } else {
                                                                        let a813 = v0.clone();
                                                                        let a814 = f17(Str::lit("copied"));
                                                                        if equal(a813, a814) {
                                                                            Some(0u64)
                                                                        } else {
                                                                            let a815 = v0.clone();
                                                                            let a816 = f17(Str::lit("ok"));
                                                                            if equal(a815, a816) {
                                                                                Some(0u64)
                                                                            } else {
                                                                                let a817 = v0.clone();
                                                                                let a818 = f17(Str::lit("clone"));
                                                                                if equal(a817, a818) {
                                                                                    Some(0u64)
                                                                                } else {
                                                                                    let a819 = v0.clone();
                                                                                    let a820 = f17(Str::lit("to_string"));
                                                                                    if equal(a819, a820) {
                                                                                        Some(0u64)
                                                                                    } else {
                                                                                        let a821 = v0.clone();
                                                                                        let a822 = f17(Str::lit("as_bytes"));
                                                                                        if equal(a821, a822) {
                                                                                            Some(0u64)
                                                                                        } else {
                                                                                            let a823 = v0.clone();
                                                                                            let a824 = f17(Str::lit("strip_prefix"));
                                                                                            if equal(a823, a824) {
                                                                                                Some(1u64)
                                                                                            } else {
                                                                                                let a825 = v0.clone();
                                                                                                let a826 = f17(Str::lit("starts_with"));
                                                                                                if equal(a825, a826) {
                                                                                                    Some(1u64)
                                                                                                } else {
                                                                                                    let a827 = v0.clone();
                                                                                                    let a828 = f17(Str::lit("split"));
                                                                                                    if equal(a827, a828) {
                                                                                                        Some(1u64)
                                                                                                    } else {
                                                                                                        let a829 = v0.clone();
                                                                                                        let a830 = f17(Str::lit("collect"));
                                                                                                        if equal(a829, a830) {
                                                                                                            Some(0u64)
                                                                                                        } else {
                                                                                                            let a831 = v0;
                                                                                                            let a832 = f17(Str::lit("is_ascii_digit"));
                                                                                                            if equal(a831, a832) {
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

pub fn f120(v0: Bytes) -> bool {
    let a833 = v0.clone();
    let a834 = f17(Str::lit("push"));
    if equal(a833, a834) {
        true
    } else {
        let a835 = v0.clone();
        let a836 = f17(Str::lit("pop"));
        if equal(a835, a836) {
            true
        } else {
            let a837 = v0.clone();
            let a838 = f17(Str::lit("truncate"));
            if equal(a837, a838) {
                true
            } else {
                let a839 = v0.clone();
                let a840 = f17(Str::lit("push_str"));
                if equal(a839, a840) {
                    true
                } else {
                    let a841 = v0.clone();
                    let a842 = f17(Str::lit("extend_from_slice"));
                    if equal(a841, a842) {
                        true
                    } else {
                        let a843 = v0.clone();
                        let a844 = f17(Str::lit("to_string"));
                        if equal(a843, a844) {
                            true
                        } else {
                            let a845 = v0;
                            let a846 = f17(Str::lit("collect"));
                            equal(a845, a846)
                        }
                    }
                }
            }
        }
    }
}

pub fn f121(v0: Adt14, v1: Bytes) -> List<u64> {
    f172(Fn20::F173(v1), List::<u64>::nil(), match v0 {
        Adt14::C0(_, _, _, _, _, _, _, _, h847, _) => {
            h847
        }
    })
}

pub fn f122(v0: List<u64>, v1: u64) -> bool {
    f174(Fn21::F175(v1), false, v0)
}

pub fn f123(v0: Adt14, v1: Adt16, v2: Adt3, v3: Bytes) -> R<Option<Bytes>> {
    let m848 = f176(f177(v1.clone(), v2));
    match m848 {
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
            if f178(v0, f179(match v1 {
                Adt16::C0(h849, _, _, _) => {
                    h849
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

pub fn f124(v0: List<u64>) -> R<Bytes> {
    f180(Fn22::F181, f17(Str::lit("")), v0)
}

pub fn f125(v0: Adt3) -> R<bool> {
    let m850 = v0;
    match m850 {
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
        Adt3::C4(r881, _, _) => {
            let v6 = (*r881).clone();
            let a851 = 13u64;
            let a852 = f182(v6.clone());
            if nat_le(a851, a852) {
                f125(v6)
            } else {
                Ok(false)
            }
        }
        Adt3::C5(r882, _) => {
            let v9 = (*r882).clone();
            let a853 = 13u64;
            let a854 = f182(v9.clone());
            if nat_le(a853, a854) {
                f125(v9)
            } else {
                Ok(false)
            }
        }
        Adt3::C6(r883, _) => {
            let v11 = (*r883).clone();
            let a855 = 13u64;
            let a856 = f182(v11.clone());
            if nat_le(a855, a856) {
                f125(v11)
            } else {
                Ok(false)
            }
        }
        Adt3::C7(r884) => {
            let v13 = (*r884).clone();
            let a857 = 12u64;
            let a858 = f182(v13.clone());
            if nat_le(a857, a858) {
                f125(v13)
            } else {
                Ok(false)
            }
        }
        Adt3::C8(r885, r886) => {
            let v14 = (*r885).clone();
            let v15 = (*r886).clone();
            let a859 = 6u64;
            let a860 = f182(v14.clone());
            if if nat_le(a859, a860) {
                f125(v14)?
            } else {
                false
            } {
                Ok(true)
            } else {
                let a861 = 6u64;
                let a862 = f182(v15.clone());
                if nat_le(a861, a862) {
                    f125(v15)
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
        Adt3::C11(r887, r888) => {
            let v21 = (*r887).clone();
            let v22 = (*r888).clone();
            if f125(v21)? {
                Ok(true)
            } else {
                f125(v22)
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
        Adt3::C18(r889) => {
            let v35 = (*r889).clone();
            f125(v35)
        }
        Adt3::C19(_) => {
            Ok(false)
        }
        Adt3::C20(_, _, _) => {
            Ok(true)
        }
        Adt3::C21(v40, r890, r891) => {
            let v41 = (*r890).clone();
            let v42 = (*r891).clone();
            if if f183(v40, v41.clone()) {
                false
            } else {
                f125(v41)?
            } {
                Ok(true)
            } else {
                let a865 = {
                    let a863 = f184(v40);
                    let a864 = 1u64;
                    nat_add(a863, a864)?
                };
                let a866 = f182(v42.clone());
                if nat_le(a865, a866) {
                    f125(v42)
                } else {
                    Ok(false)
                }
            }
        }
        Adt3::C22(r892) => {
            let v43 = (*r892).clone();
            let a867 = 12u64;
            let a868 = f182(v43.clone());
            if nat_le(a867, a868) {
                f125(v43)
            } else {
                Ok(false)
            }
        }
        Adt3::C23(r893, _) => {
            let v44 = (*r893).clone();
            let a869 = 11u64;
            let a870 = f182(v44.clone());
            if nat_le(a869, a870) {
                f125(v44)
            } else {
                Ok(false)
            }
        }
        Adt3::C24(r894, _) => {
            let v46 = (*r894).clone();
            let a871 = 13u64;
            let a872 = f182(v46.clone());
            if nat_le(a871, a872) {
                f125(v46)
            } else {
                Ok(false)
            }
        }
        Adt3::C25(r895, _, _) => {
            let v48 = (*r895).clone();
            let a873 = 13u64;
            let a874 = f182(v48.clone());
            if nat_le(a873, a874) {
                f125(v48)
            } else {
                Ok(false)
            }
        }
        Adt3::C26(r896) => {
            let v51 = (*r896).clone();
            let a875 = 12u64;
            let a876 = f182(v51.clone());
            if nat_le(a875, a876) {
                f125(v51)
            } else {
                Ok(false)
            }
        }
        Adt3::C27(r897) => {
            let v52 = (*r897).clone();
            let a877 = 12u64;
            let a878 = f182(v52.clone());
            if nat_le(a877, a878) {
                f125(v52)
            } else {
                Ok(false)
            }
        }
        Adt3::C28(r898, _, _, _) => {
            let v53 = (*r898).clone();
            let a879 = 13u64;
            let a880 = f182(v53.clone());
            if nat_le(a879, a880) {
                f125(v53)
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

pub fn f126(v0: Adt2) -> bool {
    let m899 = v0;
    match m899 {
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

pub fn f127(v0: List<Bytes>) -> bool {
    let m900 = v0;
    match m900.uncons() {
        None => {
            false
        }
        Some((v1, v2)) => {
            let m901 = v2;
            match m901.uncons() {
                None => {
                    let m902 = f167(v1);
                    match m902 {
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

pub fn f128(v0: Adt14, v1: List<Bytes>, v2: Adt4) -> R<Option<Bytes>> {
    let m903 = f162(v1.clone(), 1u64);
    match m903 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v3) => {
            let m904 = f109(v0, f161(v1.clone()));
            match m904 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v4) => {
                    let m906 = f163(match v4 {
                        Adt12::C0(_, h905, _) => {
                            h905
                        }
                    }, v3);
                    match m906 {
                        None => {
                            Ok(Some(f16(f17(Str::lit("runtime support: pattern-fields: ")), f16(f18(f34(v1, Str::lit("::"))?), f16(f17(Str::lit(" is matched with ")), f16(f47(f129(v2)?)?, f17(Str::lit(" patterns"))))))))
                        }
                        Some(v5) => {
                            let a907 = f185(v5)?;
                            let a908 = f129(v2.clone())?;
                            if nat_eq(a907, a908) {
                                Ok(None::<Bytes>)
                            } else {
                                Ok(Some(f16(f17(Str::lit("runtime support: pattern-fields: ")), f16(f18(f34(v1, Str::lit("::"))?), f16(f17(Str::lit(" is matched with ")), f16(f47(f129(v2)?)?, f17(Str::lit(" patterns"))))))))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f129(v0: Adt4) -> R<u64> {
    let m909 = v0;
    match m909 {
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
        Adt4::C9(_, r912) => {
            let v12 = (*r912).clone();
            let a910 = f129(v12)?;
            let a911 = 1u64;
            nat_add(a910, a911)
        }
    }
}

pub fn f130(v0: Adt14, v1: List<Bytes>, v2: List<Bytes>) -> R<Option<Bytes>> {
    let m913 = f69(v0, v1.clone());
    match m913 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v3) => {
            f92(Fn10::F186(v3, v1), None::<Bytes>, v2)
        }
    }
}

pub fn f131(v0: List<Adt9>, v1: Bytes) -> Option<List<Bytes>> {
    let m914 = f163(v0, v1);
    match m914 {
        None => {
            None::<List<Bytes>>
        }
        Some(v2) => {
            let m915 = v2;
            match m915 {
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

pub fn f132(v0: List<Bytes>, v1: List<Bytes>, v2: bool, v3: Bytes) -> R<bool> {
    if v2 {
        let a916 = f187(v1, v3.clone())?;
        let a917 = f187(v0, v3)?;
        Ok(nat_eq(a916, a917))
    } else {
        Ok(false)
    }
}

pub fn f133(v0: Adt14, v1: List<Adt6>, v2: u64, v3: bool) -> R<Option<Bytes>> {
    f188(Fn23::F189(v3, v2, v0), None::<Bytes>, v1)
}

pub fn f134(v0: Adt7, v1: Adt14, v2: Option<Bytes>, v3: Bytes) -> Option<Bytes> {
    let m918 = v2;
    match m918 {
        None => {
            let m919 = f135(v1, v3.clone());
            if match m919 {
                None => {
                    let m920 = f136(v3.clone());
                    match m920 {
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
                    Adt7::C0(h921, _) => {
                        h921
                    }
                }, f16(f17(Str::lit(": ")), v3))), f17(Str::lit(" names no declared trait")))))
            }
        }
        Some(v4) => {
            Some(v4)
        }
    }
}

pub fn f135(v0: Adt14, v1: Bytes) -> Option<Adt10> {
    f190(Fn24::F191(v1), None::<Adt10>, match v0 {
        Adt14::C0(_, _, h922, _, _, _, _, _, _, _) => {
            h922
        }
    })
}

pub fn f136(v0: Bytes) -> Option<Bytes> {
    let a923 = v0.clone();
    let a924 = f17(Str::lit("Clone"));
    if equal(a923, a924) {
        Some(f17(Str::lit("clone")))
    } else {
        let a925 = v0;
        let a926 = f17(Str::lit("Drop"));
        if equal(a925, a926) {
            Some(f17(Str::lit("drop")))
        } else {
            None::<Bytes>
        }
    }
}

pub fn f137(v0: Bytes) -> bool {
    let a927 = v0.clone();
    let a928 = f17(Str::lit("Clone"));
    if equal(a927, a928) {
        false
    } else {
        let a929 = v0;
        let a930 = f17(Str::lit("Drop"));
        equal(a929, a930)
    }
}

pub fn f138(v0: List<Adt8>, v1: Bytes) -> R<u64> {
    f192(Fn25::F193(v1), 0u64, v0)
}

pub fn f139(v0: List<Adt8>, v1: Bytes) -> Option<Adt8> {
    f194(Fn26::F195(v1), None::<Adt8>, v0)
}

pub fn f140(v0: Adt5) -> R<Bytes> {
    Ok(f16(match v0.clone() {
        Adt5::C0(h931, _, _, _, _) => {
            h931
        }
    }, f16(f17(Str::lit("|")), f16(if match v0.clone() {
        Adt5::C0(_, h932, _, _, _) => {
            h932
        }
    } {
        f17(Str::lit("r"))
    } else {
        f17(Str::lit("-"))
    }, f16(if match v0.clone() {
        Adt5::C0(_, _, h933, _, _) => {
            h933
        }
    } {
        f17(Str::lit("x"))
    } else {
        f17(Str::lit("-"))
    }, f16(f17(Str::lit("|")), f16(f196(Fn27::F197, f17(Str::lit("")), match v0.clone() {
        Adt5::C0(_, _, _, h934, _) => {
            h934
        }
    })?, f16(f17(Str::lit("|")), f198(match v0 {
        Adt5::C0(_, _, _, _, h935) => {
            h935
        }
    })?))))))))
}

pub fn f141(v0: List<Adt5>, v1: Bytes) -> bool {
    f199(Fn28::F200(v1), false, v0)
}

pub fn f142(v0: Result<List<Adt17>, Bytes>, v1: List<Adt6>, v2: Bytes) -> Result<List<Adt17>, Bytes> {
    f201(Fn29::F202(v2), v0, v1)
}

pub fn f143(v0: Fn15, p940: u64, p941: (u64, (u64, List<Bytes>))) -> R<((u64, (u64, List<Bytes>)), bool)> {
    let m939 = (p940, p941);
    {
        let mut m939 = m939;
        loop {
            let step = {
                let (v1, v2) = m939;
                let m936 = v1;
                if m936 == 0 {
                    Err::<(u64, (u64, (u64, List<Bytes>))), R<((u64, (u64, List<Bytes>)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m936 - 1;
                    let m938 = {
                        let c937: Fn15 = v0.clone();
                        c937.apply(v2.clone())?
                    };
                    match m938 {
                        None => {
                            Err::<(u64, (u64, (u64, List<Bytes>))), R<((u64, (u64, List<Bytes>)), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (u64, (u64, List<Bytes>))), R<((u64, (u64, List<Bytes>)), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m939 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f144(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f203(v0, v1)
}

pub fn f145(v0: List<Bytes>) -> List<Bytes> {
    f204(Fn30::F205, List::<Bytes>::nil(), v0)
}

pub fn f146(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m945 = {
        let a942 = v0;
        let a943 = v1;
        let a944 = v2;
        slice_bytes(a942, a943, a944)
    };
    match m945 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f147(v0: Bytes, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m946 = v1;
    match m946 {
        None => {
            Some(v2)
        }
        Some(v3) => {
            Some({
                let a949 = {
                    let a947 = v3;
                    let a948 = v0;
                    append_bytes(a947, a948)
                };
                let a950 = v2;
                append_bytes(a949, a950)
            })
        }
    }
}

pub fn f148(v0: Fn16, p955: u64, p956: u64) -> R<(u64, bool)> {
    let m954 = (p955, p956);
    {
        let mut m954 = m954;
        loop {
            let step = {
                let (v1, v2) = m954;
                let m951 = v1;
                if m951 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m951 - 1;
                    let m953 = {
                        let c952: Fn16 = v0.clone();
                        c952.apply(v2)?
                    };
                    match m953 {
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
                Ok(next) => { m954 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f149(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f206(v0, v1) {
        Ok(Some({
            let a957 = v1;
            let a958 = 1u64;
            nat_add(a957, a958)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f150(v0: Bytes, v1: Adt14, v2: Adt8) -> Adt14 {
    let m961 = match v2.clone() {
        Adt8::C0(h960, _, _, _) => {
            h960
        }
    };
    if match m961 {
        Adt5::C0(_, h959, _, _, _) => {
            h959
        }
    } {
        v1
    } else {
        Adt14::C0(match v1.clone() {
            Adt14::C0(h962, _, _, _, _, _, _, _, _, _) => {
                h962
            }
        }, match v1.clone() {
            Adt14::C0(_, h963, _, _, _, _, _, _, _, _) => {
                h963
            }
        }, match v1.clone() {
            Adt14::C0(_, _, h964, _, _, _, _, _, _, _) => {
                h964
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, h965, _, _, _, _, _, _) => {
                h965
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, _, h966, _, _, _, _, _) => {
                h966
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, _, _, h967, _, _, _, _) => {
                h967
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, h968, _, _, _) => {
                h968
            }
        }, {
            let a977 = match v1.clone() {
                Adt14::C0(_, _, _, _, _, _, _, h969, _, _) => {
                    h969
                }
            };
            let a978 = List::cons(Adt13::C0(f16(v0, f16(f17(Str::lit("::")), {
                let m972 = match v2.clone() {
                    Adt8::C0(h971, _, _, _) => {
                        h971
                    }
                };
                match m972 {
                    Adt5::C0(h970, _, _, _, _) => {
                        h970
                    }
                }
            })), {
                let a976 = {
                    let m975 = match v2 {
                        Adt8::C0(h974, _, _, _) => {
                            h974
                        }
                    };
                    match m975 {
                        Adt5::C0(_, _, _, h973, _) => {
                            h973
                        }
                    }
                };
                length_list(a976)
            }), List::<Adt13>::nil());
            append_list(a977, a978)
        }, match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, h979, _) => {
                h979
            }
        }, match v1 {
            Adt14::C0(_, _, _, _, _, _, _, _, _, h980) => {
                h980
            }
        })
    }
}

pub fn f151(v0: Str) -> Bytes {
    let a981 = v0;
    utf8_encode(a981)
}

pub fn f152(v0: Bytes) -> R<Bytes> {
    let v1: u64 = {
        let (h982, _) = f148(Fn16::F207(v0.clone()), {
            let a983 = v0.clone();
            length_bytes(a983)
        }, 0u64)?;
        h982
    };
    Ok(f146(v0.clone(), v1, {
        let a985 = {
            let a984 = v0;
            length_bytes(a984)
        };
        let a986 = v1;
        nat_sub(a985, a986)
    }))
}

pub fn f153(v0: u64, v1: u64) -> Bytes {
    f146(f151(Str::lit("0123456789")), {
        let a990 = {
            let a987 = v0;
            let a988 = v1;
            let a989 = 0u64;
            nat_quot(a987, a988, a989)
        };
        let a991 = 10u64;
        let a992 = 0u64;
        nat_rem(a990, a991, a992)
    }, 1u64)
}

pub fn f154(v0: Adt9) -> Bytes {
    let m993 = v0;
    match m993 {
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

pub fn f155(v0: Fn17, p997: Option<Adt11>, p998: List<Adt11>) -> Option<Adt11> {
    let m996 = (p997, p998);
    {
        let mut m996 = m996;
        loop {
            let step = {
                let (v1, v2) = m996;
                let m994 = v2;
                match m994.uncons() {
                    None => {
                        Err::<(Option<Adt11>, List<Adt11>), Option<Adt11>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Adt11>, List<Adt11>), Option<Adt11>>(({
                            let c995: Fn17 = v0.clone();
                            c995.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m996 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f156(v0: Bytes, v1: Option<Adt11>, v2: Adt11) -> Option<Adt11> {
    let m999 = v1;
    match m999 {
        None => {
            if f32(match v2.clone() {
                Adt11::C0(h1000, _, _) => {
                    h1000
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

pub fn f157(v0: Fn18, p1004: Option<Adt12>, p1005: List<Adt12>) -> Option<Adt12> {
    let m1003 = (p1004, p1005);
    {
        let mut m1003 = m1003;
        loop {
            let step = {
                let (v1, v2) = m1003;
                let m1001 = v2;
                match m1001.uncons() {
                    None => {
                        Err::<(Option<Adt12>, List<Adt12>), Option<Adt12>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Adt12>, List<Adt12>), Option<Adt12>>(({
                            let c1002: Fn18 = v0.clone();
                            c1002.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1003 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f158(v0: Bytes, v1: Option<Adt12>, v2: Adt12) -> Option<Adt12> {
    let m1006 = v1;
    match m1006 {
        None => {
            if f32(match v2.clone() {
                Adt12::C0(h1007, _, _) => {
                    h1007
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

pub fn f159(v0: Fn19, p1011: Option<Adt13>, p1012: List<Adt13>) -> Option<Adt13> {
    let m1010 = (p1011, p1012);
    {
        let mut m1010 = m1010;
        loop {
            let step = {
                let (v1, v2) = m1010;
                let m1008 = v2;
                match m1008.uncons() {
                    None => {
                        Err::<(Option<Adt13>, List<Adt13>), Option<Adt13>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Adt13>, List<Adt13>), Option<Adt13>>(({
                            let c1009: Fn19 = v0.clone();
                            c1009.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1010 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f160(v0: Bytes, v1: Option<Adt13>, v2: Adt13) -> Option<Adt13> {
    let m1013 = v1;
    match m1013 {
        None => {
            if f32(match v2.clone() {
                Adt13::C0(h1014, _) => {
                    h1014
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

pub fn f161(v0: List<Bytes>) -> Bytes {
    let m1015 = v0;
    match m1015.uncons() {
        None => {
            f17(Str::lit(""))
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f162(p1019: List<Bytes>, p1020: u64) -> Option<Bytes> {
    let m1018 = (p1019, p1020);
    {
        let mut m1018 = m1018;
        loop {
            let step = {
                let (v0, v1) = m1018;
                let m1016 = v0;
                match m1016.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        let m1017 = v1;
                        if m1017 == 0 {
                            Err::<(List<Bytes>, u64), Option<Bytes>>(Some(v2))
                        } else {
                            let v4 = m1017 - 1;
                            Ok::<(List<Bytes>, u64), Option<Bytes>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1018 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f163(v0: List<Adt9>, v1: Bytes) -> Option<Adt9> {
    f208(Fn31::F209(v1), None::<Adt9>, v0)
}

pub fn f164(v0: Bytes) -> bool {
    let a1021 = v0.clone();
    let a1022 = f17(Str::lit("u8"));
    if equal(a1021, a1022) {
        true
    } else {
        let a1023 = v0.clone();
        let a1024 = f17(Str::lit("u16"));
        if equal(a1023, a1024) {
            true
        } else {
            let a1025 = v0.clone();
            let a1026 = f17(Str::lit("u32"));
            if equal(a1025, a1026) {
                true
            } else {
                let a1027 = v0.clone();
                let a1028 = f17(Str::lit("u64"));
                if equal(a1027, a1028) {
                    true
                } else {
                    let a1029 = v0.clone();
                    let a1030 = f17(Str::lit("u128"));
                    if equal(a1029, a1030) {
                        true
                    } else {
                        let a1031 = v0.clone();
                        let a1032 = f17(Str::lit("i8"));
                        if equal(a1031, a1032) {
                            true
                        } else {
                            let a1033 = v0.clone();
                            let a1034 = f17(Str::lit("i16"));
                            if equal(a1033, a1034) {
                                true
                            } else {
                                let a1035 = v0.clone();
                                let a1036 = f17(Str::lit("i32"));
                                if equal(a1035, a1036) {
                                    true
                                } else {
                                    let a1037 = v0.clone();
                                    let a1038 = f17(Str::lit("i64"));
                                    if equal(a1037, a1038) {
                                        true
                                    } else {
                                        let a1039 = v0;
                                        let a1040 = f17(Str::lit("i128"));
                                        equal(a1039, a1040)
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

pub fn f165(v0: Bytes, v1: Bytes) -> bool {
    let m1041 = f210(v0.clone(), v1.clone());
    match m1041 {
        None => {
            if f164(v0) {
                f211(f17(Str::lit("usize")), v1)
            } else {
                false
            }
        }
        Some(_) => {
            f211(v0, v1)
        }
    }
}

pub fn f166(v0: Adt14, v1: Bytes) -> Option<Adt13> {
    f159(Fn19::F212(v1), None::<Adt13>, match v0 {
        Adt14::C0(_, _, _, _, _, _, _, h1042, _, _) => {
            h1042
        }
    })
}

pub fn f167(v0: Bytes) -> Option<u64> {
    let a1043 = v0.clone();
    let a1044 = f17(Str::lit("Some"));
    if equal(a1043, a1044) {
        Some(1u64)
    } else {
        let a1045 = v0.clone();
        let a1046 = f17(Str::lit("Ok"));
        if equal(a1045, a1046) {
            Some(1u64)
        } else {
            let a1047 = v0;
            let a1048 = f17(Str::lit("Err"));
            if equal(a1047, a1048) {
                Some(1u64)
            } else {
                None::<u64>
            }
        }
    }
}

pub fn f168(v0: Adt14, v1: Bytes) -> Option<u64> {
    let m1049 = f108(v0, v1);
    match m1049 {
        None => {
            None::<u64>
        }
        Some(v2) => {
            Some({
                let a1051 = match v2 {
                    Adt11::C0(_, h1050, _) => {
                        h1050
                    }
                };
                length_list(a1051)
            })
        }
    }
}

pub fn f169(v0: Bytes, v1: Bytes) -> Option<u64> {
    let m1052 = f210(v0.clone(), v1.clone());
    match m1052 {
        None => {
            if f164(v0) {
                f210(f17(Str::lit("usize")), v1)
            } else {
                None::<u64>
            }
        }
        Some(v2) => {
            Some(v2)
        }
    }
}

pub fn f170(v0: Adt14, v1: Bytes, v2: Bytes) -> R<Option<u64>> {
    let m1053 = f109(v0, v1);
    match m1053 {
        None => {
            Ok(None::<u64>)
        }
        Some(v3) => {
            f213(match v3 {
                Adt12::C0(_, h1054, _) => {
                    h1054
                }
            }, v2)
        }
    }
}

pub fn f171(v0: Adt14, v1: Bytes, v2: Bytes) -> Option<u64> {
    let m1055 = f166(v0, f16(v1, f16(f17(Str::lit("::")), v2)));
    match m1055 {
        None => {
            None::<u64>
        }
        Some(v3) => {
            Some(match v3 {
                Adt13::C0(_, h1056) => {
                    h1056
                }
            })
        }
    }
}

pub fn f172(v0: Fn20, p1060: List<u64>, p1061: List<Adt13>) -> List<u64> {
    let m1059 = (p1060, p1061);
    {
        let mut m1059 = m1059;
        loop {
            let step = {
                let (v1, v2) = m1059;
                let m1057 = v2;
                match m1057.uncons() {
                    None => {
                        Err::<(List<u64>, List<Adt13>), List<u64>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<u64>, List<Adt13>), List<u64>>(({
                            let c1058: Fn20 = v0.clone();
                            c1058.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1059 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f173(v0: Bytes, v1: List<u64>, v2: Adt13) -> List<u64> {
    if f32(match v2.clone() {
        Adt13::C0(h1062, _) => {
            h1062
        }
    }, v0) {
        let a1064 = v1;
        let a1065 = List::cons(match v2 {
            Adt13::C0(_, h1063) => {
                h1063
            }
        }, List::<u64>::nil());
        append_list(a1064, a1065)
    } else {
        v1
    }
}

pub fn f174(v0: Fn21, p1069: bool, p1070: List<u64>) -> bool {
    let m1068 = (p1069, p1070);
    {
        let mut m1068 = m1068;
        loop {
            let step = {
                let (v1, v2) = m1068;
                let m1066 = v2;
                match m1066.uncons() {
                    None => {
                        Err::<(bool, List<u64>), bool>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<u64>), bool>(({
                            let c1067: Fn21 = v0.clone();
                            c1067.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1068 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f175(v0: u64, v1: bool, v2: u64) -> bool {
    if v1 {
        true
    } else {
        let a1071 = v2;
        let a1072 = v0;
        nat_eq(a1071, a1072)
    }
}

pub fn f176(p1083: Adt2) -> Adt2 {
    let m1082 = p1083;
    {
        let mut m1082 = m1082;
        loop {
            let step = {
                let v0 = m1082;
                let m1073 = v0;
                match m1073 {
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
                    Adt2::C7(r1074) => {
                        let v2 = (*r1074).clone();
                        Ok::<Adt2, Adt2>(v2)
                    }
                    Adt2::C8(r1075) => {
                        let v3 = (*r1075).clone();
                        Ok::<Adt2, Adt2>(v3)
                    }
                    Adt2::C9(r1076, r1077) => {
                        let v4 = (*r1076).clone();
                        let v5 = (*r1077).clone();
                        Err::<Adt2, Adt2>(Adt2::C9(std::rc::Rc::new(v4), std::rc::Rc::new(v5)))
                    }
                    Adt2::C10(v6, r1078) => {
                        let v7 = (*r1078).clone();
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
                    Adt2::C15(r1079) => {
                        let v8 = (*r1079).clone();
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
                    Adt2::C19(r1080, r1081) => {
                        let v9 = (*r1080).clone();
                        let v10 = (*r1081).clone();
                        Err::<Adt2, Adt2>(Adt2::C19(std::rc::Rc::new(v9), std::rc::Rc::new(v10)))
                    }
                }
            };
            match step {
                Ok(next) => { m1082 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f177(v0: Adt16, v1: Adt3) -> Adt2 {
    let m1084 = v1;
    match m1084 {
        Adt3::C0(v2) => {
            let m1086 = f52(match v0 {
                Adt16::C0(_, h1085, _, _) => {
                    h1085
                }
            }, v2);
            match m1086 {
                None => {
                    Adt2::C16
                }
                Some(v3) => {
                    match v3 {
                        Adt17::C0(_, _, h1087) => {
                            h1087
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
        Adt3::C5(r1088, v12) => {
            let v11 = (*r1088).clone();
            f214(f215(v0.clone(), f176(f177(v0, v11))), v12)
        }
        Adt3::C6(_, _) => {
            Adt2::C16
        }
        Adt3::C7(r1089) => {
            let v15 = (*r1089).clone();
            let v16: Adt2 = f177(v0, v15);
            if f216(v16.clone()) {
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

pub fn f178(v0: Adt14, v1: List<Bytes>, v2: Bytes) -> R<bool> {
    f87(Fn8::F217(v2, v0), false, v1)
}

pub fn f179(p1094: List<Adt7>, p1095: u64) -> List<Bytes> {
    let m1093 = (p1094, p1095);
    {
        let mut m1093 = m1093;
        loop {
            let step = {
                let (v0, v1) = m1093;
                let m1090 = v0;
                match m1090.uncons() {
                    None => {
                        Err::<(List<Adt7>, u64), List<Bytes>>(List::<Bytes>::nil())
                    }
                    Some((v2, v3)) => {
                        let m1091 = v1;
                        if m1091 == 0 {
                            match v2 {
                                Adt7::C0(_, h1092) => {
                                    Err::<(List<Adt7>, u64), List<Bytes>>(h1092)
                                }
                            }
                        } else {
                            let v4 = m1091 - 1;
                            Ok::<(List<Adt7>, u64), List<Bytes>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1093 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f180(v0: Fn22, p1099: Bytes, p1100: List<u64>) -> R<Bytes> {
    let m1098 = (p1099, p1100);
    {
        let mut m1098 = m1098;
        loop {
            let step = {
                let (v1, v2) = m1098;
                let m1096 = v2;
                match m1096.uncons() {
                    None => {
                        Err::<(Bytes, List<u64>), R<Bytes>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Bytes, List<u64>), R<Bytes>>(({
                            let c1097: Fn22 = v0.clone();
                            c1097.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1098 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f181(v0: Bytes, v1: u64) -> R<Bytes> {
    let a1101 = v0.clone();
    let a1102 = f17(Str::lit(""));
    if equal(a1101, a1102) {
        f47(v1)
    } else {
        Ok(f16(v0, f16(f17(Str::lit("/")), f47(v1)?)))
    }
}

pub fn f182(v0: Adt3) -> u64 {
    let m1103 = v0;
    match m1103 {
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
            f184(v40)
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

pub fn f183(v0: u64, v1: Adt3) -> bool {
    let a1104 = v0;
    let a1105 = 8u64;
    if if nat_eq(a1104, a1105) {
        f218(v1.clone())
    } else {
        false
    } {
        true
    } else {
        let a1108 = f182(v1);
        let a1109 = {
            let a1106 = f184(v0);
            let a1107 = 5u64;
            if nat_eq(a1106, a1107) {
                6u64
            } else {
                f184(v0)
            }
        };
        nat_lt(a1108, a1109)
    }
}

pub fn f184(v0: u64) -> u64 {
    let a1110 = v0;
    let a1111 = 6u64;
    if nat_eq(a1110, a1111) {
        3u64
    } else {
        let a1112 = v0;
        let a1113 = 5u64;
        if nat_eq(a1112, a1113) {
            4u64
        } else {
            let a1114 = v0;
            let a1115 = 7u64;
            if nat_eq(a1114, a1115) {
                5u64
            } else {
                let a1116 = v0;
                let a1117 = 8u64;
                if nat_eq(a1116, a1117) {
                    5u64
                } else {
                    let a1118 = v0;
                    let a1119 = 9u64;
                    if nat_eq(a1118, a1119) {
                        5u64
                    } else {
                        let a1120 = v0;
                        let a1121 = 10u64;
                        if nat_eq(a1120, a1121) {
                            5u64
                        } else {
                            let a1122 = v0;
                            let a1123 = 11u64;
                            if nat_eq(a1122, a1123) {
                                5u64
                            } else {
                                let a1124 = v0;
                                let a1125 = 0u64;
                                if nat_eq(a1124, a1125) {
                                    9u64
                                } else {
                                    let a1126 = v0;
                                    let a1127 = 1u64;
                                    if nat_eq(a1126, a1127) {
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

pub fn f185(v0: Adt9) -> R<u64> {
    let m1128 = v0;
    match m1128 {
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

pub fn f186(v0: List<Bytes>, v1: List<Bytes>, v2: Option<Bytes>, v3: Bytes) -> R<Option<Bytes>> {
    let m1129 = v2;
    match m1129 {
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

pub fn f187(v0: List<Bytes>, v1: Bytes) -> R<u64> {
    f219(Fn32::F220(v1), 0u64, v0)
}

pub fn f188(v0: Fn23, p1133: Option<Bytes>, p1134: List<Adt6>) -> R<Option<Bytes>> {
    let m1132 = (p1133, p1134);
    {
        let mut m1132 = m1132;
        loop {
            let step = {
                let (v1, v2) = m1132;
                let m1130 = v2;
                match m1130.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Adt6>), R<Option<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Adt6>), R<Option<Bytes>>>(({
                            let c1131: Fn23 = v0.clone();
                            c1131.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1132 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f189(v0: bool, v1: u64, v2: Adt14, v3: Option<Bytes>, v4: Adt6) -> R<Option<Bytes>> {
    let m1135 = v3;
    match m1135 {
        None => {
            let m1137 = f39(Str::lit("a parameter name"), match v4.clone() {
                Adt6::C0(h1136, _, _) => {
                    h1136
                }
            })?;
            match m1137 {
                None => {
                    f12(v2, match v4 {
                        Adt6::C0(_, _, h1138) => {
                            h1138
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

pub fn f190(v0: Fn24, p1142: Option<Adt10>, p1143: List<Adt10>) -> Option<Adt10> {
    let m1141 = (p1142, p1143);
    {
        let mut m1141 = m1141;
        loop {
            let step = {
                let (v1, v2) = m1141;
                let m1139 = v2;
                match m1139.uncons() {
                    None => {
                        Err::<(Option<Adt10>, List<Adt10>), Option<Adt10>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Adt10>, List<Adt10>), Option<Adt10>>(({
                            let c1140: Fn24 = v0.clone();
                            c1140.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1141 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f191(v0: Bytes, v1: Option<Adt10>, v2: Adt10) -> Option<Adt10> {
    let m1144 = v1;
    match m1144 {
        None => {
            if f32(match v2.clone() {
                Adt10::C0(h1145, _) => {
                    h1145
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

pub fn f192(v0: Fn25, p1149: u64, p1150: List<Adt8>) -> R<u64> {
    let m1148 = (p1149, p1150);
    {
        let mut m1148 = m1148;
        loop {
            let step = {
                let (v1, v2) = m1148;
                let m1146 = v2;
                match m1146.uncons() {
                    None => {
                        Err::<(u64, List<Adt8>), R<u64>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(u64, List<Adt8>), R<u64>>(({
                            let c1147: Fn25 = v0.clone();
                            c1147.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1148 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f193(v0: Bytes, v1: u64, v2: Adt8) -> R<u64> {
    if f32({
        let m1153 = match v2 {
            Adt8::C0(h1152, _, _, _) => {
                h1152
            }
        };
        match m1153 {
            Adt5::C0(h1151, _, _, _, _) => {
                h1151
            }
        }
    }, v0) {
        let a1154 = v1;
        let a1155 = 1u64;
        nat_add(a1154, a1155)
    } else {
        Ok(v1)
    }
}

pub fn f194(v0: Fn26, p1159: Option<Adt8>, p1160: List<Adt8>) -> Option<Adt8> {
    let m1158 = (p1159, p1160);
    {
        let mut m1158 = m1158;
        loop {
            let step = {
                let (v1, v2) = m1158;
                let m1156 = v2;
                match m1156.uncons() {
                    None => {
                        Err::<(Option<Adt8>, List<Adt8>), Option<Adt8>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Adt8>, List<Adt8>), Option<Adt8>>(({
                            let c1157: Fn26 = v0.clone();
                            c1157.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1158 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f195(v0: Bytes, v1: Option<Adt8>, v2: Adt8) -> Option<Adt8> {
    let m1161 = v1;
    match m1161 {
        None => {
            if f32({
                let m1164 = match v2.clone() {
                    Adt8::C0(h1163, _, _, _) => {
                        h1163
                    }
                };
                match m1164 {
                    Adt5::C0(h1162, _, _, _, _) => {
                        h1162
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

pub fn f196(v0: Fn27, p1168: Bytes, p1169: List<Adt6>) -> R<Bytes> {
    let m1167 = (p1168, p1169);
    {
        let mut m1167 = m1167;
        loop {
            let step = {
                let (v1, v2) = m1167;
                let m1165 = v2;
                match m1165.uncons() {
                    None => {
                        Err::<(Bytes, List<Adt6>), R<Bytes>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Bytes, List<Adt6>), R<Bytes>>(({
                            let c1166: Fn27 = v0.clone();
                            c1166.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1167 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f197(v0: Bytes, v1: Adt6) -> R<Bytes> {
    Ok(f16(v0, f16(match v1.clone() {
        Adt6::C0(h1170, _, _) => {
            h1170
        }
    }, f16(if match v1.clone() {
        Adt6::C0(_, h1171, _) => {
            h1171
        }
    } {
        f17(Str::lit(":m:"))
    } else {
        f17(Str::lit(":-:"))
    }, f16(f198(match v1 {
        Adt6::C0(_, _, h1172) => {
            h1172
        }
    })?, f17(Str::lit(",")))))))
}

pub fn f198(v0: Adt2) -> R<Bytes> {
    let m1173 = v0;
    match m1173 {
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
        Adt2::C7(r1174) => {
            let v2 = (*r1174).clone();
            Ok(f16(f17(Str::lit("R(")), f16(f198(v2)?, f17(Str::lit(")")))))
        }
        Adt2::C8(r1175) => {
            let v3 = (*r1175).clone();
            Ok(f16(f17(Str::lit("M(")), f16(f198(v3)?, f17(Str::lit(")")))))
        }
        Adt2::C9(r1176, r1177) => {
            let v4 = (*r1176).clone();
            let v5 = (*r1177).clone();
            Ok(f16(f17(Str::lit("T(")), f16(f198(v4)?, f16(f17(Str::lit(",")), f16(f198(v5)?, f17(Str::lit(")")))))))
        }
        Adt2::C10(v6, r1178) => {
            let v7 = (*r1178).clone();
            Ok(f16(f17(Str::lit("D")), f16(f18(v6), f16(f17(Str::lit("(")), f16(f198(v7)?, f17(Str::lit(")")))))))
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
        Adt2::C15(r1179) => {
            let v8 = (*r1179).clone();
            Ok(f16(f17(Str::lit("L(")), f16(f198(v8)?, f17(Str::lit(")")))))
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
        Adt2::C19(r1180, r1181) => {
            let v9 = (*r1180).clone();
            let v10 = (*r1181).clone();
            Ok(f16(f198(v9)?, f16(f17(Str::lit(";")), f198(v10)?)))
        }
    }
}

pub fn f199(v0: Fn28, p1185: bool, p1186: List<Adt5>) -> bool {
    let m1184 = (p1185, p1186);
    {
        let mut m1184 = m1184;
        loop {
            let step = {
                let (v1, v2) = m1184;
                let m1182 = v2;
                match m1182.uncons() {
                    None => {
                        Err::<(bool, List<Adt5>), bool>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<Adt5>), bool>(({
                            let c1183: Fn28 = v0.clone();
                            c1183.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1184 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f200(v0: Bytes, v1: bool, v2: Adt5) -> bool {
    if v1 {
        true
    } else {
        f32(match v2 {
            Adt5::C0(h1187, _, _, _, _) => {
                h1187
            }
        }, v0)
    }
}

pub fn f201(v0: Fn29, p1191: Result<List<Adt17>, Bytes>, p1192: List<Adt6>) -> Result<List<Adt17>, Bytes> {
    let m1190 = (p1191, p1192);
    {
        let mut m1190 = m1190;
        loop {
            let step = {
                let (v1, v2) = m1190;
                let m1188 = v2;
                match m1188.uncons() {
                    None => {
                        Err::<(Result<List<Adt17>, Bytes>, List<Adt6>), Result<List<Adt17>, Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Result<List<Adt17>, Bytes>, List<Adt6>), Result<List<Adt17>, Bytes>>(({
                            let c1189: Fn29 = v0.clone();
                            c1189.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1190 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f202(v0: Bytes, v1: Result<List<Adt17>, Bytes>, v2: Adt6) -> Result<List<Adt17>, Bytes> {
    let m1193 = v1;
    match m1193 {
        Ok(v4) => {
            let m1195 = f52(v4.clone(), match v2.clone() {
                Adt6::C0(h1194, _, _) => {
                    h1194
                }
            });
            match m1195 {
                None => {
                    Ok::<List<Adt17>, Bytes>(List::cons(Adt17::C0(match v2.clone() {
                        Adt6::C0(h1197, _, _) => {
                            h1197
                        }
                    }, match v2.clone() {
                        Adt6::C0(_, h1198, _) => {
                            h1198
                        }
                    }, match v2 {
                        Adt6::C0(_, _, h1199) => {
                            h1199
                        }
                    }), v4))
                }
                Some(_) => {
                    Err::<List<Adt17>, Bytes>(f16(f17(Str::lit("runtime support: param-duplicate: ")), f16(f18(match v2 {
                        Adt6::C0(h1196, _, _) => {
                            h1196
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

pub fn f203(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v2: u64 = {
        let (h1200, _) = v1.clone();
        h1200
    };
    let v3: u64 = {
        let (h1201, _) = {
            let (_, h1202) = v1.clone();
            h1202
        };
        h1201
    };
    let v4: List<Bytes> = {
        let (_, h1203) = {
            let (_, h1204) = v1;
            h1204
        };
        h1203
    };
    let m1205 = f221(v0.clone(), v2);
    match m1205 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v5) => {
            let a1206 = v5;
            let a1207 = 58u8;
            if equal(a1206, a1207) {
                let m1210 = f221(v0.clone(), {
                    let a1208 = v2;
                    let a1209 = 1u64;
                    nat_add(a1208, a1209)?
                });
                match m1210 {
                    None => {
                        Ok(Some(({
                            let a1221 = v2;
                            let a1222 = 1u64;
                            nat_add(a1221, a1222)?
                        }, (v3, v4))))
                    }
                    Some(v6) => {
                        let a1211 = v6;
                        let a1212 = 58u8;
                        if equal(a1211, a1212) {
                            Ok(Some(({
                                let a1213 = v2;
                                let a1214 = 2u64;
                                nat_add(a1213, a1214)?
                            }, ({
                                let a1215 = v2;
                                let a1216 = 2u64;
                                nat_add(a1215, a1216)?
                            }, List::cons(f146(v0, v3, {
                                let a1217 = v2;
                                let a1218 = v3;
                                nat_sub(a1217, a1218)
                            }), v4)))))
                        } else {
                            Ok(Some(({
                                let a1219 = v2;
                                let a1220 = 1u64;
                                nat_add(a1219, a1220)?
                            }, (v3, v4))))
                        }
                    }
                }
            } else {
                Ok(Some(({
                    let a1223 = v2;
                    let a1224 = 1u64;
                    nat_add(a1223, a1224)?
                }, (v3, v4))))
            }
        }
    }
}

pub fn f204(v0: Fn30, p1228: List<Bytes>, p1229: List<Bytes>) -> List<Bytes> {
    let m1227 = (p1228, p1229);
    {
        let mut m1227 = m1227;
        loop {
            let step = {
                let (v1, v2) = m1227;
                let m1225 = v2;
                match m1225.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c1226: Fn30 = v0.clone();
                            c1226.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1227 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f205(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f206(v0: Bytes, v1: u64) -> bool {
    let m1230 = f221(v0, v1);
    match m1230 {
        None => {
            false
        }
        Some(v2) => {
            if f222(v2, 97u8, 122u8) {
                true
            } else {
                if f222(v2, 65u8, 90u8) {
                    true
                } else {
                    let a1231 = v2;
                    let a1232 = 95u8;
                    if equal(a1231, a1232) {
                        true
                    } else {
                        let a1233 = 0u64;
                        let a1234 = v1;
                        if nat_lt(a1233, a1234) {
                            f222(v2, 48u8, 57u8)
                        } else {
                            false
                        }
                    }
                }
            }
        }
    }
}

pub fn f207(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f223(v0, v1)
}

pub fn f208(v0: Fn31, p1238: Option<Adt9>, p1239: List<Adt9>) -> Option<Adt9> {
    let m1237 = (p1238, p1239);
    {
        let mut m1237 = m1237;
        loop {
            let step = {
                let (v1, v2) = m1237;
                let m1235 = v2;
                match m1235.uncons() {
                    None => {
                        Err::<(Option<Adt9>, List<Adt9>), Option<Adt9>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Adt9>, List<Adt9>), Option<Adt9>>(({
                            let c1236: Fn31 = v0.clone();
                            c1236.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1237 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f209(v0: Bytes, v1: Option<Adt9>, v2: Adt9) -> Option<Adt9> {
    let m1240 = v1;
    match m1240 {
        None => {
            if f32(f154(v2.clone()), v0) {
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

pub fn f210(v0: Bytes, v1: Bytes) -> Option<u64> {
    let a1241 = v0.clone();
    let a1242 = f17(Str::lit("AtomicU64"));
    if if equal(a1241, a1242) {
        let a1243 = v1.clone();
        let a1244 = f17(Str::lit("new"));
        equal(a1243, a1244)
    } else {
        false
    } {
        Some(1u64)
    } else {
        let a1245 = v0.clone();
        let a1246 = f17(Str::lit("Rc"));
        if if equal(a1245, a1246) {
            let a1247 = v1.clone();
            let a1248 = f17(Str::lit("new"));
            equal(a1247, a1248)
        } else {
            false
        } {
            Some(1u64)
        } else {
            let a1249 = v0.clone();
            let a1250 = f17(Str::lit("Rc"));
            if if equal(a1249, a1250) {
                let a1251 = v1.clone();
                let a1252 = f17(Str::lit("from"));
                equal(a1251, a1252)
            } else {
                false
            } {
                Some(1u64)
            } else {
                let a1253 = v0.clone();
                let a1254 = f17(Str::lit("Rc"));
                if if equal(a1253, a1254) {
                    let a1255 = v1.clone();
                    let a1256 = f17(Str::lit("try_unwrap"));
                    equal(a1255, a1256)
                } else {
                    false
                } {
                    Some(1u64)
                } else {
                    let a1257 = v0.clone();
                    let a1258 = f17(Str::lit("Rc"));
                    if if equal(a1257, a1258) {
                        let a1259 = v1.clone();
                        let a1260 = f17(Str::lit("get_mut"));
                        equal(a1259, a1260)
                    } else {
                        false
                    } {
                        Some(1u64)
                    } else {
                        let a1261 = v0.clone();
                        let a1262 = f17(Str::lit("Vec"));
                        if if equal(a1261, a1262) {
                            let a1263 = v1.clone();
                            let a1264 = f17(Str::lit("new"));
                            equal(a1263, a1264)
                        } else {
                            false
                        } {
                            Some(0u64)
                        } else {
                            let a1265 = v0.clone();
                            let a1266 = f17(Str::lit("Vec"));
                            if if equal(a1265, a1266) {
                                let a1267 = v1.clone();
                                let a1268 = f17(Str::lit("with_capacity"));
                                equal(a1267, a1268)
                            } else {
                                false
                            } {
                                Some(1u64)
                            } else {
                                let a1269 = v0.clone();
                                let a1270 = f17(Str::lit("String"));
                                if if equal(a1269, a1270) {
                                    let a1271 = v1.clone();
                                    let a1272 = f17(Str::lit("new"));
                                    equal(a1271, a1272)
                                } else {
                                    false
                                } {
                                    Some(0u64)
                                } else {
                                    let a1273 = v0.clone();
                                    let a1274 = f17(Str::lit("mem"));
                                    if if equal(a1273, a1274) {
                                        let a1275 = v1.clone();
                                        let a1276 = f17(Str::lit("replace"));
                                        equal(a1275, a1276)
                                    } else {
                                        false
                                    } {
                                        Some(2u64)
                                    } else {
                                        let a1277 = v0.clone();
                                        let a1278 = f17(Str::lit("str"));
                                        if if equal(a1277, a1278) {
                                            let a1279 = v1.clone();
                                            let a1280 = f17(Str::lit("from_utf8"));
                                            equal(a1279, a1280)
                                        } else {
                                            false
                                        } {
                                            Some(1u64)
                                        } else {
                                            let a1281 = v0.clone();
                                            let a1282 = f17(Str::lit("usize"));
                                            if if equal(a1281, a1282) {
                                                let a1283 = v1.clone();
                                                let a1284 = f17(Str::lit("from"));
                                                equal(a1283, a1284)
                                            } else {
                                                false
                                            } {
                                                Some(1u64)
                                            } else {
                                                let a1285 = v0;
                                                let a1286 = f17(Str::lit("usize"));
                                                if if equal(a1285, a1286) {
                                                    let a1287 = v1;
                                                    let a1288 = f17(Str::lit("try_from"));
                                                    equal(a1287, a1288)
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

pub fn f211(v0: Bytes, v1: Bytes) -> bool {
    let a1289 = v0.clone();
    let a1290 = f17(Str::lit("AtomicU64"));
    if if equal(a1289, a1290) {
        let a1291 = v1.clone();
        let a1292 = f17(Str::lit("new"));
        equal(a1291, a1292)
    } else {
        false
    } {
        false
    } else {
        let a1293 = v0.clone();
        let a1294 = f17(Str::lit("Rc"));
        if if equal(a1293, a1294) {
            let a1295 = v1.clone();
            let a1296 = f17(Str::lit("new"));
            equal(a1295, a1296)
        } else {
            false
        } {
            true
        } else {
            let a1297 = v0.clone();
            let a1298 = f17(Str::lit("Rc"));
            if if equal(a1297, a1298) {
                let a1299 = v1.clone();
                let a1300 = f17(Str::lit("from"));
                equal(a1299, a1300)
            } else {
                false
            } {
                true
            } else {
                let a1301 = v0.clone();
                let a1302 = f17(Str::lit("Rc"));
                if if equal(a1301, a1302) {
                    let a1303 = v1.clone();
                    let a1304 = f17(Str::lit("try_unwrap"));
                    equal(a1303, a1304)
                } else {
                    false
                } {
                    true
                } else {
                    let a1305 = v0.clone();
                    let a1306 = f17(Str::lit("Rc"));
                    if if equal(a1305, a1306) {
                        let a1307 = v1.clone();
                        let a1308 = f17(Str::lit("get_mut"));
                        equal(a1307, a1308)
                    } else {
                        false
                    } {
                        true
                    } else {
                        let a1309 = v0.clone();
                        let a1310 = f17(Str::lit("Vec"));
                        if if equal(a1309, a1310) {
                            let a1311 = v1.clone();
                            let a1312 = f17(Str::lit("new"));
                            equal(a1311, a1312)
                        } else {
                            false
                        } {
                            true
                        } else {
                            let a1313 = v0.clone();
                            let a1314 = f17(Str::lit("Vec"));
                            if if equal(a1313, a1314) {
                                let a1315 = v1.clone();
                                let a1316 = f17(Str::lit("with_capacity"));
                                equal(a1315, a1316)
                            } else {
                                false
                            } {
                                true
                            } else {
                                let a1317 = v0.clone();
                                let a1318 = f17(Str::lit("String"));
                                if if equal(a1317, a1318) {
                                    let a1319 = v1.clone();
                                    let a1320 = f17(Str::lit("new"));
                                    equal(a1319, a1320)
                                } else {
                                    false
                                } {
                                    true
                                } else {
                                    let a1321 = v0.clone();
                                    let a1322 = f17(Str::lit("mem"));
                                    if if equal(a1321, a1322) {
                                        let a1323 = v1.clone();
                                        let a1324 = f17(Str::lit("replace"));
                                        equal(a1323, a1324)
                                    } else {
                                        false
                                    } {
                                        false
                                    } else {
                                        let a1325 = v0.clone();
                                        let a1326 = f17(Str::lit("str"));
                                        if if equal(a1325, a1326) {
                                            let a1327 = v1.clone();
                                            let a1328 = f17(Str::lit("from_utf8"));
                                            equal(a1327, a1328)
                                        } else {
                                            false
                                        } {
                                            false
                                        } else {
                                            let a1329 = v0.clone();
                                            let a1330 = f17(Str::lit("usize"));
                                            if if equal(a1329, a1330) {
                                                let a1331 = v1.clone();
                                                let a1332 = f17(Str::lit("from"));
                                                equal(a1331, a1332)
                                            } else {
                                                false
                                            } {
                                                false
                                            } else {
                                                let a1333 = v0;
                                                let a1334 = f17(Str::lit("usize"));
                                                let _ = if equal(a1333, a1334) {
                                                    let a1335 = v1;
                                                    let a1336 = f17(Str::lit("try_from"));
                                                    equal(a1335, a1336)
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

pub fn f212(v0: Bytes, v1: Option<Adt13>, v2: Adt13) -> Option<Adt13> {
    let m1337 = v1;
    match m1337 {
        None => {
            if f32(match v2.clone() {
                Adt13::C0(h1338, _) => {
                    h1338
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

pub fn f213(v0: List<Adt9>, v1: Bytes) -> R<Option<u64>> {
    let m1339 = f163(v0, v1);
    match m1339 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            let m1340 = v2;
            match m1340 {
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

pub fn f214(v0: Adt2, v1: u64) -> Adt2 {
    let m1341 = v0;
    match m1341 {
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
        Adt2::C9(r1346, r1347) => {
            let v5 = (*r1346).clone();
            let v6 = (*r1347).clone();
            let a1342 = v1;
            let a1343 = 0u64;
            if nat_eq(a1342, a1343) {
                v5
            } else {
                let a1344 = v1;
                let a1345 = 1u64;
                if nat_eq(a1344, a1345) {
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

pub fn f215(v0: Adt16, v1: Adt2) -> Adt2 {
    let m1348 = v1.clone();
    match m1348 {
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
                Adt16::C0(_, _, h1349, _) => {
                    h1349
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

pub fn f216(v0: Adt2) -> bool {
    let m1350 = v0;
    match m1350 {
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

pub fn f217(v0: Bytes, v1: Adt14, v2: bool, v3: Bytes) -> bool {
    if v2 {
        true
    } else {
        let m1351 = f135(v1, v3.clone());
        match m1351 {
            None => {
                let m1353 = f136(v3);
                match m1353 {
                    None => {
                        false
                    }
                    Some(v5) => {
                        f32(v5, v0)
                    }
                }
            }
            Some(v4) => {
                f199(Fn28::F224(v0), false, match v4 {
                    Adt10::C0(_, h1352) => {
                        h1352
                    }
                })
            }
        }
    }
}

pub fn f218(p1359: Adt3) -> bool {
    let m1358 = p1359;
    {
        let mut m1358 = m1358;
        loop {
            let step = {
                let v0 = m1358;
                let m1354 = v0;
                match m1354 {
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
                    Adt3::C21(v40, _, r1357) => {
                        let v42 = (*r1357).clone();
                        let a1355 = f184(v40);
                        let a1356 = f182(v42.clone());
                        if nat_lt(a1355, a1356) {
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
                Ok(next) => { m1358 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f219(v0: Fn32, p1363: u64, p1364: List<Bytes>) -> R<u64> {
    let m1362 = (p1363, p1364);
    {
        let mut m1362 = m1362;
        loop {
            let step = {
                let (v1, v2) = m1362;
                let m1360 = v2;
                match m1360.uncons() {
                    None => {
                        Err::<(u64, List<Bytes>), R<u64>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(u64, List<Bytes>), R<u64>>(({
                            let c1361: Fn32 = v0.clone();
                            c1361.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1362 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f220(v0: Bytes, v1: u64, v2: Bytes) -> R<u64> {
    if f32(v2, v0) {
        let a1365 = v1;
        let a1366 = 1u64;
        nat_add(a1365, a1366)
    } else {
        Ok(v1)
    }
}

pub fn f221(v0: Bytes, v1: u64) -> Option<u8> {
    let a1367 = v0;
    let a1368 = v1;
    index_bytes(a1367, a1368)
}

pub fn f222(v0: u8, v1: u8, v2: u8) -> bool {
    let m1371 = {
        let a1369 = v0;
        let a1370 = v1;
        fixed_u8::checked_sub(a1369, a1370)
    };
    match m1371 {
        None => {
            false
        }
        Some(_) => {
            let m1374 = {
                let a1372 = v2;
                let a1373 = v0;
                fixed_u8::checked_sub(a1372, a1373)
            };
            match m1374 {
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

pub fn f223(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f225(v0, v1, 48u8) {
        Ok(Some({
            let a1375 = v1;
            let a1376 = 1u64;
            nat_add(a1375, a1376)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f224(v0: Bytes, v1: bool, v2: Adt5) -> bool {
    if v1 {
        true
    } else {
        f32(match v2 {
            Adt5::C0(h1377, _, _, _, _) => {
                h1377
            }
        }, v0)
    }
}

pub fn f225(v0: Bytes, v1: u64, v2: u8) -> bool {
    let m1378 = f221(v0, v1);
    match m1378 {
        None => {
            false
        }
        Some(v3) => {
            let a1379 = v3;
            let a1380 = v2;
            equal(a1379, a1380)
        }
    }
}

pub fn run(p0: Adt0) -> R<Result<(), Bytes>> {
    f0(p0)
}
