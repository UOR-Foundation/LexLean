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
    C0(Adt1),
    C1(Adt8),
    C2(Adt27),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(Bytes, List<Bytes>, List<(Bytes, Adt2)>, List<(Bytes, Adt2)>),
    C1(Bytes, List<Bytes>, List<(Bytes, Adt2)>, List<(Bytes, Adt2)>),
    C2(Bytes, Option<Bytes>, Bytes, Adt2, u64, Adt3),
    C3(Bytes, List<Bytes>, List<(Bytes, Adt2)>, List<(Bytes, Adt2)>, Option<Bytes>),
    C4(Bytes, List<Bytes>, List<(Bytes, Adt2)>, Adt2, Option<Bytes>, Adt3, List<Bytes>, bool, Option<Bytes>, Option<(Adt3, List<(Option<Bytes>, Bytes)>)>, Option<(List<Bytes>, List<Bytes>)>),
    C5(Bytes, List<Bytes>, List<(Bytes, Adt2)>, Adt3, Adt6, List<Bytes>),
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
    C0(Bytes),
    C1(Bytes),
    C2(Adt4, Bytes),
    C3(Bytes),
    C4(Bytes),
    C5(Adt5, std::rc::Rc<Adt3>, Adt2),
    C6(bool),
    C7,
    C8(Adt2),
    C9(std::rc::Rc<Adt3>, std::rc::Rc<Adt3>),
    C10(Option<Bytes>, Bytes, Adt2, std::rc::Rc<Adt3>),
    C11(Option<Bytes>, Bytes, Adt2, std::rc::Rc<Adt3>),
    C12(Option<Bytes>, Bytes, Adt2, Option<Bytes>, Bytes),
    C13(std::rc::Rc<Adt3>, Bytes),
    C14(Option<Bytes>, Bytes, Adt2, std::rc::Rc<Adt3>),
    C15(std::rc::Rc<Adt3>, std::rc::Rc<Adt3>, std::rc::Rc<Adt3>),
    C16(std::rc::Rc<Adt3>, std::rc::Rc<Adt3>),
    C17(std::rc::Rc<Adt3>, std::rc::Rc<Adt3>),
    C18(std::rc::Rc<Adt3>, std::rc::Rc<Adt3>),
    C19(std::rc::Rc<Adt3>, std::rc::Rc<Adt3>),
    C20(std::rc::Rc<Adt3>, std::rc::Rc<Adt3>),
    C21(std::rc::Rc<Adt3>, std::rc::Rc<Adt3>),
    C22(std::rc::Rc<Adt3>, std::rc::Rc<Adt3>),
    C23(std::rc::Rc<Adt3>, std::rc::Rc<Adt3>),
    C24(std::rc::Rc<Adt3>, std::rc::Rc<Adt3>),
    C25(std::rc::Rc<Adt3>, std::rc::Rc<Adt3>),
    C26(std::rc::Rc<Adt3>, std::rc::Rc<Adt3>),
    C27(std::rc::Rc<Adt3>),
    C28(std::rc::Rc<Adt3>, std::rc::Rc<Adt3>),
    C29(std::rc::Rc<Adt3>, std::rc::Rc<Adt3>),
    C30(Bytes, Adt2, std::rc::Rc<Adt3>),
    C31(Bytes, Adt2, std::rc::Rc<Adt3>, std::rc::Rc<Adt3>),
    C32(std::rc::Rc<Adt3>, std::rc::Rc<Adt3>),
    C33(std::rc::Rc<Adt3>),
    C34(std::rc::Rc<Adt3>),
    C35(List<(Bytes, Adt2)>, List<Bytes>, std::rc::Rc<Adt3>),
    C36(std::rc::Rc<Adt3>, std::rc::Rc<Adt3>),
    C37(Option<Bytes>, Bytes, Adt2),
    C38(Adt2, Adt2, std::rc::Rc<Adt3>),
    C39(Adt2, std::rc::Rc<Adt3>),
    C40(Adt2, std::rc::Rc<Adt3>, std::rc::Rc<Adt3>),
    C41(Option<Bytes>, Bytes, Adt2, std::rc::Rc<Adt3>, List<Bytes>),
    C42,
    C43(std::rc::Rc<Adt3>, std::rc::Rc<Adt3>),
    C44(Bytes, std::rc::Rc<Adt3>, std::rc::Rc<Adt3>),
    C45(Option<Bytes>, Bytes, List<Bytes>, std::rc::Rc<Adt3>, std::rc::Rc<Adt3>),
    C46(std::rc::Rc<Adt3>, std::rc::Rc<Adt3>, std::rc::Rc<Adt3>),
    C47(std::rc::Rc<Adt3>, std::rc::Rc<Adt3>, std::rc::Rc<Adt3>),
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
pub enum Adt6 {
    C0,
    C1,
    C2(List<(Option<Bytes>, Bytes)>),
    C3(List<(Option<Bytes>, Bytes)>),
    C4(std::rc::Rc<Adt6>),
    C5(Bytes, std::rc::Rc<Adt6>),
    C6(Adt3, List<(Option<Bytes>, Bytes)>),
    C7(Bytes, List<Bytes>, std::rc::Rc<Adt6>),
    C8(Bytes, std::rc::Rc<Adt6>, std::rc::Rc<Adt6>),
    C9,
    C10(Adt7),
    C11(Bytes, Adt2, Adt3, List<(Option<Bytes>, Bytes)>),
    C12(Option<Bytes>, Bytes, Adt2, Adt3),
    C13,
    C14(std::rc::Rc<Adt6>, std::rc::Rc<Adt6>),
    C15(Bytes, List<Bytes>, std::rc::Rc<Adt6>, std::rc::Rc<Adt6>),
    C16(Adt2, Bytes, Adt3, std::rc::Rc<Adt6>, std::rc::Rc<Adt6>),
}

#[derive(Clone)]
pub enum Adt7 {
    C0(Bytes, Bytes, Option<Bytes>, Bytes, Option<Bytes>, Bytes, Bytes),
    C1(Bytes, Option<Bytes>, Bytes, Option<Bytes>, Bytes, List<(Bytes, Bytes)>),
}

#[derive(Clone)]
pub enum Adt8 {
    C0(Bytes, Adt9, Bytes, u64, Adt10, Adt2, List<Bytes>),
    C1(Bytes, List<Bytes>, Adt12, Adt12, Option<Adt13>, Option<(Option<Bytes>, Bytes)>, Option<(Option<Bytes>, Bytes)>, Option<(Option<Bytes>, Bytes)>, List<Adt14>, List<Bytes>),
    C2(Bytes, List<Bytes>, Adt12, Adt2, Option<Adt16>, Adt17, bool, List<Bytes>),
    C3(Bytes, List<Bytes>, Adt24, Adt24, List<Adt26>, List<Bytes>),
    C4(Bytes, List<Bytes>, Adt24, Adt24, List<Adt24>, List<(Option<Bytes>, Bytes)>, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt9 {
    C0,
    C1,
    C2,
    C3,
    C4,
}

#[derive(Clone)]
pub enum Adt10 {
    C0,
    C1(Adt11, List<u64>),
    C2,
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
}

#[derive(Clone)]
pub enum Adt12 {
    C0(Bytes, Adt2),
}

#[derive(Clone)]
pub enum Adt13 {
    C0(Bytes, Bytes, Adt2),
}

#[derive(Clone)]
pub enum Adt14 {
    C0(Adt15, (Option<Bytes>, Bytes), (Option<Bytes>, Bytes), Option<(Option<Bytes>, Bytes)>),
}

#[derive(Clone)]
pub enum Adt15 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt16 {
    C0(Bytes, Adt2, Adt3),
}

#[derive(Clone)]
pub enum Adt17 {
    C0(Adt3),
    C1(List<Adt18>, Adt3),
    C2(Adt19, Adt3, u64, (Option<Bytes>, Bytes), (Option<Bytes>, Bytes), (Option<Bytes>, Bytes), List<Adt3>),
    C3(Adt20, Adt3, u64, (Option<Bytes>, Bytes), List<Adt21>, Adt22),
    C4(Adt23),
}

#[derive(Clone)]
pub enum Adt18 {
    C0(Bytes, Adt3, Adt3),
}

#[derive(Clone)]
pub enum Adt19 {
    C0,
}

#[derive(Clone)]
pub enum Adt20 {
    C0,
}

#[derive(Clone)]
pub enum Adt21 {
    C0(u64, u64, (Option<Bytes>, Bytes), (Option<Bytes>, Bytes)),
    C1,
    C2(u64, Bytes, Bytes),
}

#[derive(Clone)]
pub enum Adt22 {
    C0(List<Adt3>),
    C1(Bytes, Adt3),
}

#[derive(Clone)]
pub enum Adt23 {
    C0(List<Adt24>, List<Adt25>),
    C1(Adt24, Adt24),
    C2(Adt24, Adt24),
    C3(Adt3, Adt24, Adt24),
    C4(Adt24, Adt25),
}

#[derive(Clone)]
pub enum Adt24 {
    C0((Option<Bytes>, Bytes), List<Adt2>),
}

#[derive(Clone)]
pub enum Adt25 {
    C0,
    C1((Option<Bytes>, Bytes)),
    C2,
}

#[derive(Clone)]
pub enum Adt26 {
    C0((Option<Bytes>, Bytes), (Option<Bytes>, Bytes), (Option<Bytes>, Bytes), u64, u64, (Option<Bytes>, Bytes)),
    C1(Adt24, (Option<Bytes>, Bytes)),
    C2((Option<Bytes>, Bytes)),
    C3((Option<Bytes>, Bytes)),
    C4((Option<Bytes>, Bytes)),
}

#[derive(Clone)]
pub enum Adt27 {
    C0(Bytes, List<Bytes>, Adt13, (Option<Bytes>, Bytes), Option<Adt28>, Option<(Option<Bytes>, Bytes)>, List<Bytes>),
    C1(Bytes, List<Bytes>, Adt24, Option<Adt29>, Adt3, Adt3, (Option<Bytes>, Bytes), Option<(Option<Bytes>, Bytes)>, bool, List<Bytes>),
    C2(Bytes, List<Bytes>, Adt12, Adt12, (Option<Bytes>, Bytes), (Option<Bytes>, Bytes), (Option<Bytes>, Bytes), Option<(Option<Bytes>, Bytes)>, List<Bytes>),
    C3(Bytes, List<Bytes>, Option<Adt24>, Adt12, Option<Adt3>, List<Adt24>, Adt30, Option<Adt32>, Adt24, List<Adt33>, bool, List<Bytes>),
    C4(Bytes, List<Bytes>, Adt34, Adt24, List<Bytes>),
    C5(Bytes, u64, Adt35, List<Bytes>),
    C6(Bytes, (Option<Bytes>, Bytes), (Option<Bytes>, Bytes), Adt24, u64, (Option<Bytes>, Bytes), Adt36, List<Bytes>),
    C7(Bytes, (Option<Bytes>, Bytes), Adt24, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt28 {
    C0((Option<Bytes>, Bytes), (Option<Bytes>, Bytes)),
}

#[derive(Clone)]
pub enum Adt29 {
    C0(Bytes, Adt2, Adt3),
}

#[derive(Clone)]
pub enum Adt30 {
    C0(Option<Adt3>),
    C1(Adt31, Option<Adt3>, Option<Adt3>, bool),
    C2(Adt3, Option<Adt3>),
}

#[derive(Clone)]
pub enum Adt31 {
    C0,
    C1,
}

#[derive(Clone)]
pub enum Adt32 {
    C0(Bytes, Adt2, Adt3),
}

#[derive(Clone)]
pub enum Adt33 {
    C0((Option<Bytes>, Bytes)),
    C1((Option<Bytes>, Bytes)),
    C2((Option<Bytes>, Bytes), (Option<Bytes>, Bytes), (Option<Bytes>, Bytes)),
    C3((Option<Bytes>, Bytes)),
}

#[derive(Clone)]
pub enum Adt34 {
    C0,
    C1,
}

#[derive(Clone)]
pub enum Adt35 {
    C0,
    C1,
}

#[derive(Clone)]
pub enum Adt36 {
    C0,
    C1(Adt37, Adt38),
}

#[derive(Clone)]
pub enum Adt37 {
    C0,
}

#[derive(Clone)]
pub enum Adt38 {
    C0,
}

#[derive(Clone)]
pub enum Adt39 {
    C0(bool),
    C1(bool, u64),
    C2(Bytes),
    C3(std::rc::Rc<Adt39>),
    C4(std::rc::Rc<Adt39>),
    C5,
    C6(std::rc::Rc<Adt39>, std::rc::Rc<Adt39>),
    C7(Bytes, std::rc::Rc<Adt39>, std::rc::Rc<Adt39>),
}

#[derive(Clone)]
pub enum Adt40 {
    C0(Bytes, bool, bool, Option<u64>),
}

#[derive(Clone)]
pub enum Fn0 {
    F6,
}

impl Fn0 {
    pub fn apply(&self, p0: List<Adt39>, p1: Adt0) -> List<Adt39> {
        match self {
            Fn0::F6 => f6(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F10,
    F139,
}

impl Fn1 {
    pub fn apply(&self, p0: Adt39, p1: (Bytes, Adt39)) -> Adt39 {
        match self {
            Fn1::F10 => f10(p0, p1),
            Fn1::F139 => f139(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F12,
}

impl Fn2 {
    pub fn apply(&self, p0: Adt39, p1: Adt39) -> Adt39 {
        match self {
            Fn2::F12 => f12(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F16,
}

impl Fn3 {
    pub fn apply(&self, p0: List<Adt0>, p1: Adt0) -> List<Adt0> {
        match self {
            Fn3::F16 => f16(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F24,
}

impl Fn4 {
    pub fn apply(&self, p0: List<Adt39>, p1: Adt39) -> List<Adt39> {
        match self {
            Fn4::F24 => f24(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F30(Bytes),
    F173(u64, Bytes),
}

impl Fn5 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn5::F30(k0) => f30(k0.clone(), p0),
            Fn5::F173(k0, k1) => Ok(f173(*k0, k1.clone(), p0)),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F33,
    F213,
}

impl Fn6 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn6::F33 => f33(p0, p1),
            Fn6::F213 => f213(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F36,
}

impl Fn7 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, Bytes)) -> R<List<Bytes>> {
        match self {
            Fn7::F36 => f36(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F57,
    F60,
    F66,
    F67,
    F70,
    F71,
    F76,
    F77,
    F84,
    F85,
    F87,
    F88,
    F90,
    F91,
    F92,
    F93,
    F100,
    F101,
    F103,
    F105,
    F107,
    F108,
    F118,
}

impl Fn8 {
    pub fn apply(&self, p0: List<Adt39>, p1: Bytes) -> List<Adt39> {
        match self {
            Fn8::F57 => f57(p0, p1),
            Fn8::F60 => f60(p0, p1),
            Fn8::F66 => f66(p0, p1),
            Fn8::F67 => f67(p0, p1),
            Fn8::F70 => f70(p0, p1),
            Fn8::F71 => f71(p0, p1),
            Fn8::F76 => f76(p0, p1),
            Fn8::F77 => f77(p0, p1),
            Fn8::F84 => f84(p0, p1),
            Fn8::F85 => f85(p0, p1),
            Fn8::F87 => f87(p0, p1),
            Fn8::F88 => f88(p0, p1),
            Fn8::F90 => f90(p0, p1),
            Fn8::F91 => f91(p0, p1),
            Fn8::F92 => f92(p0, p1),
            Fn8::F93 => f93(p0, p1),
            Fn8::F100 => f100(p0, p1),
            Fn8::F101 => f101(p0, p1),
            Fn8::F103 => f103(p0, p1),
            Fn8::F105 => f105(p0, p1),
            Fn8::F107 => f107(p0, p1),
            Fn8::F108 => f108(p0, p1),
            Fn8::F118 => f118(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F64,
}

impl Fn9 {
    pub fn apply(&self, p0: List<Adt39>, p1: Adt14) -> List<Adt39> {
        match self {
            Fn9::F64 => f64(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F74,
}

impl Fn10 {
    pub fn apply(&self, p0: List<Adt39>, p1: Adt26) -> List<Adt39> {
        match self {
            Fn10::F74 => f74(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F79,
    F94,
    F200,
}

impl Fn11 {
    pub fn apply(&self, p0: List<Adt39>, p1: Adt24) -> List<Adt39> {
        match self {
            Fn11::F79 => f79(p0, p1),
            Fn11::F94 => f94(p0, p1),
            Fn11::F200 => f200(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F82,
    F184,
}

impl Fn12 {
    pub fn apply(&self, p0: List<Adt39>, p1: (Option<Bytes>, Bytes)) -> List<Adt39> {
        match self {
            Fn12::F82 => f82(p0, p1),
            Fn12::F184 => f184(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F98,
}

impl Fn13 {
    pub fn apply(&self, p0: List<Adt39>, p1: Adt33) -> List<Adt39> {
        match self {
            Fn13::F98 => f98(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F113(Bytes),
}

impl Fn14 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Bytes) -> Option<Bytes> {
        match self {
            Fn14::F113(k0) => f113(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F120,
    F130,
}

impl Fn15 {
    pub fn apply(&self, p0: List<Adt39>, p1: (Bytes, Adt2)) -> List<Adt39> {
        match self {
            Fn15::F120 => f120(p0, p1),
            Fn15::F130 => f130(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F135,
}

impl Fn16 {
    pub fn apply(&self, p0: List<Adt39>, p1: u64) -> List<Adt39> {
        match self {
            Fn16::F135 => f135(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F138,
}

impl Fn17 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn17::F138 => f138(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F142,
}

impl Fn18 {
    pub fn apply(&self, p0: List<Adt14>, p1: Adt14) -> List<Adt14> {
        match self {
            Fn18::F142 => f142(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F144,
}

impl Fn19 {
    pub fn apply(&self, p0: List<Adt39>, p1: Adt18) -> List<Adt39> {
        match self {
            Fn19::F144 => f144(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn20 {
    F148,
    F199,
}

impl Fn20 {
    pub fn apply(&self, p0: List<Adt39>, p1: Adt3) -> List<Adt39> {
        match self {
            Fn20::F148 => f148(p0, p1),
            Fn20::F199 => f199(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn21 {
    F152,
}

impl Fn21 {
    pub fn apply(&self, p0: List<Adt39>, p1: Adt21) -> List<Adt39> {
        match self {
            Fn21::F152 => f152(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn22 {
    F157,
}

impl Fn22 {
    pub fn apply(&self, p0: List<Adt39>, p1: Adt2) -> List<Adt39> {
        match self {
            Fn22::F157 => f157(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn23 {
    F161,
}

impl Fn23 {
    pub fn apply(&self, p0: List<Adt26>, p1: Adt26) -> List<Adt26> {
        match self {
            Fn23::F161 => f161(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn24 {
    F163,
}

impl Fn24 {
    pub fn apply(&self, p0: List<Adt24>, p1: Adt24) -> List<Adt24> {
        match self {
            Fn24::F163 => f163(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn25 {
    F165,
}

impl Fn25 {
    pub fn apply(&self, p0: List<(Option<Bytes>, Bytes)>, p1: (Option<Bytes>, Bytes)) -> List<(Option<Bytes>, Bytes)> {
        match self {
            Fn25::F165 => f165(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn26 {
    F169,
}

impl Fn26 {
    pub fn apply(&self, p0: List<Adt33>, p1: Adt33) -> List<Adt33> {
        match self {
            Fn26::F169 => f169(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn27 {
    F177,
}

impl Fn27 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn27::F177 => f177(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn28 {
    F179,
}

impl Fn28 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
        match self {
            Fn28::F179 => f179(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn29 {
    F181,
}

impl Fn29 {
    pub fn apply(&self, p0: List<(Bytes, Adt2)>, p1: (Bytes, Adt2)) -> List<(Bytes, Adt2)> {
        match self {
            Fn29::F181 => f181(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn30 {
    F186,
}

impl Fn30 {
    pub fn apply(&self, p0: List<Adt39>, p1: (Bytes, Bytes)) -> List<Adt39> {
        match self {
            Fn30::F186 => f186(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn31 {
    F189,
}

impl Fn31 {
    pub fn apply(&self, p0: List<u64>, p1: u64) -> List<u64> {
        match self {
            Fn31::F189 => f189(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn32 {
    F193,
}

impl Fn32 {
    pub fn apply(&self, p0: List<Adt18>, p1: Adt18) -> List<Adt18> {
        match self {
            Fn32::F193 => f193(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn33 {
    F195,
}

impl Fn33 {
    pub fn apply(&self, p0: List<Adt3>, p1: Adt3) -> List<Adt3> {
        match self {
            Fn33::F195 => f195(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn34 {
    F198,
}

impl Fn34 {
    pub fn apply(&self, p0: List<Adt21>, p1: Adt21) -> List<Adt21> {
        match self {
            Fn34::F198 => f198(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn35 {
    F202,
}

impl Fn35 {
    pub fn apply(&self, p0: List<Adt39>, p1: Adt25) -> List<Adt39> {
        match self {
            Fn35::F202 => f202(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn36 {
    F206,
}

impl Fn36 {
    pub fn apply(&self, p0: List<Adt2>, p1: Adt2) -> List<Adt2> {
        match self {
            Fn36::F206 => f206(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn37 {
    F211,
}

impl Fn37 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn37::F211 => f211(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn38 {
    F215,
}

impl Fn38 {
    pub fn apply(&self, p0: List<Adt25>, p1: Adt25) -> List<Adt25> {
        match self {
            Fn38::F215 => f215(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn39 {
    F219,
}

impl Fn39 {
    pub fn apply(&self, p0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
        match self {
            Fn39::F219 => f219(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn40 {
    F223,
}

impl Fn40 {
    pub fn apply(&self, p0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
        match self {
            Fn40::F223 => f223(p0),
        }
    }
}

pub fn f0(v0: Bytes, v1: List<Adt0>) -> R<Bytes> {
    f1(f2(List::cons((f3(Str::lit("spec")), Adt39::C2(v0)), List::cons((f3(Str::lit("declarations")), Adt39::C3(std::rc::Rc::new(f4(f5(Fn0::F6, List::<Adt39>::nil(), f7(v1)))))), List::<(Bytes, Adt39)>::nil()))))
}

pub fn f1(v0: Adt39) -> R<Bytes> {
    f8(v0)
}

pub fn f2(v0: List<(Bytes, Adt39)>) -> Adt39 {
    Adt39::C4(std::rc::Rc::new(f9(Fn1::F10, Adt39::C5, v0)))
}

pub fn f3(v0: Str) -> Bytes {
    let a1 = v0;
    utf8_encode(a1)
}

pub fn f4(v0: List<Adt39>) -> Adt39 {
    f11(Fn2::F12, Adt39::C5, f13(v0))
}

pub fn f5(v0: Fn0, p5: List<Adt39>, p6: List<Adt0>) -> List<Adt39> {
    let m4 = (p5, p6);
    {
        let mut m4 = m4;
        loop {
            let step = {
                let (v1, v2) = m4;
                let m2 = v2;
                match m2.uncons() {
                    None => {
                        Err::<(List<Adt39>, List<Adt0>), List<Adt39>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt39>, List<Adt0>), List<Adt39>>(({
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

pub fn f6(v0: List<Adt39>, v1: Adt0) -> List<Adt39> {
    List::cons(f14(v1), v0)
}

pub fn f7(v0: List<Adt0>) -> List<Adt0> {
    f15(Fn3::F16, List::<Adt0>::nil(), v0)
}

pub fn f8(v0: Adt39) -> R<Bytes> {
    let m7 = v0;
    match m7 {
        Adt39::C0(v1) => {
            if v1 {
                Ok(Bytes::lit(&[0x74, 0x72, 0x75, 0x65]))
            } else {
                Ok(Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]))
            }
        }
        Adt39::C1(v2, v3) => {
            f17(v2, v3)
        }
        Adt39::C2(v4) => {
            f18(v4)
        }
        Adt39::C3(r8) => {
            let v5 = (*r8).clone();
            Ok(f19(f20(v5)?))
        }
        Adt39::C4(r9) => {
            let v6 = (*r9).clone();
            f21(f22(v6)?)
        }
        Adt39::C5 => {
            Ok(Bytes::lit(&[]))
        }
        Adt39::C6(_, _) => {
            Ok(Bytes::lit(&[]))
        }
        Adt39::C7(_, _, _) => {
            Ok(Bytes::lit(&[]))
        }
    }
}

pub fn f9(v0: Fn1, p13: Adt39, p14: List<(Bytes, Adt39)>) -> Adt39 {
    let m12 = (p13, p14);
    {
        let mut m12 = m12;
        loop {
            let step = {
                let (v1, v2) = m12;
                let m10 = v2;
                match m10.uncons() {
                    None => {
                        Err::<(Adt39, List<(Bytes, Adt39)>), Adt39>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt39, List<(Bytes, Adt39)>), Adt39>(({
                            let c11: Fn1 = v0.clone();
                            c11.apply(v1, v3)
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

pub fn f10(v0: Adt39, v1: (Bytes, Adt39)) -> Adt39 {
    Adt39::C7({
        let (h15, _) = v1;
        h15
    }, std::rc::Rc::new({
        let (_, h16) = v1;
        h16
    }), std::rc::Rc::new(v0))
}

pub fn f11(v0: Fn2, p20: Adt39, p21: List<Adt39>) -> Adt39 {
    let m19 = (p20, p21);
    {
        let mut m19 = m19;
        loop {
            let step = {
                let (v1, v2) = m19;
                let m17 = v2;
                match m17.uncons() {
                    None => {
                        Err::<(Adt39, List<Adt39>), Adt39>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt39, List<Adt39>), Adt39>(({
                            let c18: Fn2 = v0.clone();
                            c18.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m19 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f12(v0: Adt39, v1: Adt39) -> Adt39 {
    Adt39::C6(std::rc::Rc::new(v1), std::rc::Rc::new(v0))
}

pub fn f13(v0: List<Adt39>) -> List<Adt39> {
    f23(Fn4::F24, List::<Adt39>::nil(), v0)
}

pub fn f14(v0: Adt0) -> Adt39 {
    let m22 = v0;
    match m22 {
        Adt0::C0(v1) => {
            f25(v1)
        }
        Adt0::C1(v2) => {
            f26(v2)
        }
        Adt0::C2(v3) => {
            f27(v3)
        }
    }
}

pub fn f15(v0: Fn3, p26: List<Adt0>, p27: List<Adt0>) -> List<Adt0> {
    let m25 = (p26, p27);
    {
        let mut m25 = m25;
        loop {
            let step = {
                let (v1, v2) = m25;
                let m23 = v2;
                match m23.uncons() {
                    None => {
                        Err::<(List<Adt0>, List<Adt0>), List<Adt0>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt0>, List<Adt0>), List<Adt0>>(({
                            let c24: Fn3 = v0.clone();
                            c24.apply(v1, v3)
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

pub fn f16(v0: List<Adt0>, v1: Adt0) -> List<Adt0> {
    List::cons(v1, v0)
}

pub fn f17(v0: bool, v1: u64) -> R<Bytes> {
    if if v0 {
        let a28 = 0u64;
        let a29 = v1;
        nat_lt(a28, a29)
    } else {
        false
    } {
        let a30 = Bytes::lit(&[0x2d]);
        let a31 = f28(v1)?;
        Ok(append_bytes(a30, a31))
    } else {
        f28(v1)
    }
}

pub fn f18(v0: Bytes) -> R<Bytes> {
    let a37 = {
        let a35 = Bytes::lit(&[0x22]);
        let a36 = {
            let (_, h32) = {
                let (h33, _) = f29(Fn5::F30(v0.clone()), {
                    let a34 = v0;
                    length_bytes(a34)
                }, (0u64, Bytes::lit(&[])))?;
                h33
            };
            h32
        };
        append_bytes(a35, a36)
    };
    let a38 = Bytes::lit(&[0x22]);
    Ok(append_bytes(a37, a38))
}

pub fn f19(v0: List<Bytes>) -> Bytes {
    let a41 = {
        let a39 = Bytes::lit(&[0x5b]);
        let a40 = f31(v0, Bytes::lit(&[0x2c]));
        append_bytes(a39, a40)
    };
    let a42 = Bytes::lit(&[0x5d]);
    append_bytes(a41, a42)
}

pub fn f20(v0: Adt39) -> R<List<Bytes>> {
    let m43 = v0;
    match m43 {
        Adt39::C0(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt39::C1(_, _) => {
            Ok(List::<Bytes>::nil())
        }
        Adt39::C2(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt39::C3(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt39::C4(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt39::C5 => {
            Ok(List::<Bytes>::nil())
        }
        Adt39::C6(r44, r45) => {
            let v7 = (*r44).clone();
            let v8 = (*r45).clone();
            Ok(List::cons(f8(v7)?, f20(v8)?))
        }
        Adt39::C7(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f21(v0: List<(Bytes, Bytes)>) -> R<Bytes> {
    let v1: List<(Bytes, Bytes)> = f32(Fn6::F33, List::<(Bytes, Bytes)>::nil(), f34(v0)?);
    let v2: List<Bytes> = f35(Fn7::F36, List::<Bytes>::nil(), v1)?;
    let a48 = {
        let a46 = Bytes::lit(&[0x7b]);
        let a47 = f31(v2, Bytes::lit(&[0x2c]));
        append_bytes(a46, a47)
    };
    let a49 = Bytes::lit(&[0x7d]);
    Ok(append_bytes(a48, a49))
}

pub fn f22(v0: Adt39) -> R<List<(Bytes, Bytes)>> {
    let m50 = v0;
    match m50 {
        Adt39::C0(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt39::C1(_, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt39::C2(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt39::C3(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt39::C4(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt39::C5 => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt39::C6(_, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt39::C7(v9, r51, r52) => {
            let v10 = (*r51).clone();
            let v11 = (*r52).clone();
            Ok(List::cons((v9, f8(v10)?), f22(v11)?))
        }
    }
}

pub fn f23(v0: Fn4, p56: List<Adt39>, p57: List<Adt39>) -> List<Adt39> {
    let m55 = (p56, p57);
    {
        let mut m55 = m55;
        loop {
            let step = {
                let (v1, v2) = m55;
                let m53 = v2;
                match m53.uncons() {
                    None => {
                        Err::<(List<Adt39>, List<Adt39>), List<Adt39>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt39>, List<Adt39>), List<Adt39>>(({
                            let c54: Fn4 = v0.clone();
                            c54.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m55 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f24(v0: List<Adt39>, v1: Adt39) -> List<Adt39> {
    List::cons(v1, v0)
}

pub fn f25(v0: Adt1) -> Adt39 {
    let m58 = v0;
    match m58 {
        Adt1::C0(v1, v2, v3, v4) => {
            f37(Str::lit("structure"), f38(Str::lit("name"), f39(v1), f38(Str::lit("type_parameters"), f40(v2), f38(Str::lit("parameters"), f41(v3), f38(Str::lit("fields"), f42(v4), f43())))))
        }
        Adt1::C1(v5, v6, v7, v8) => {
            f37(Str::lit("class"), f38(Str::lit("name"), f39(v5), f38(Str::lit("type_parameters"), f40(v6), f38(Str::lit("parameters"), f41(v7), f38(Str::lit("fields"), f42(v8), f43())))))
        }
        Adt1::C2(v9, v10, v11, v12, v13, v14) => {
            f37(Str::lit("instance"), f38(Str::lit("name"), f39(v9), f38(Str::lit("class"), f44(v10, v11), f38(Str::lit("arguments"), Adt39::C3(std::rc::Rc::new(f45(v12))), f38(Str::lit("priority"), Adt39::C1(false, v13), f38(Str::lit("fields"), Adt39::C3(std::rc::Rc::new(f46(v14))), f43()))))))
        }
        Adt1::C3(v15, v16, v17, v18, v19) => {
            f37(Str::lit("inductive"), f38(Str::lit("name"), f39(v15), f38(Str::lit("type_parameters"), f40(v16), f38(Str::lit("parameters"), f41(v17), f38(Str::lit("constructors"), f47(v18), f48(Str::lit("mutual"), v19, f43()))))))
        }
        Adt1::C4(v20, v21, v22, v23, v24, v25, v26, v27, v28, v29, v30) => {
            f37(Str::lit("definition"), f38(Str::lit("name"), f39(v20), f49(Str::lit("type_parameters"), v21, f38(Str::lit("parameters"), f41(v22), f38(Str::lit("result"), f45(v23), f48(Str::lit("recursive_argument"), v24, f38(Str::lit("body"), f46(v25), f49(Str::lit("axioms"), v26, if v27 {
                f38(Str::lit("executable"), Adt39::C0(true), f48(Str::lit("mutual"), v28, f50(v29, f51(v30, f43()))))
            } else {
                f48(Str::lit("mutual"), v28, f50(v29, f51(v30, f43())))
            }))))))))
        }
        Adt1::C5(v31, v32, v33, v34, v35, v36) => {
            f37(Str::lit("theorem"), f38(Str::lit("name"), f39(v31), f49(Str::lit("type_parameters"), v32, f38(Str::lit("parameters"), f41(v33), f38(Str::lit("statement"), f46(v34), f38(Str::lit("proof"), f52(v35), f49(Str::lit("axioms"), v36, f43())))))))
        }
    }
}

pub fn f26(v0: Adt8) -> Adt39 {
    let m59 = v0;
    match m59 {
        Adt8::C0(v1, v2, v3, v4, v5, v6, v7) => {
            let v8: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("kind")), Adt39::C2(f53(Str::lit("artifact")))), List::<(Bytes, Adt39)>::nil());
            let v9: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("name")), Adt39::C2(v1)), v8);
            let v10: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("role")), f54(v2)), v9);
            let v11: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("sha256")), Adt39::C2(v3)), v10);
            let v12: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("length")), Adt39::C1(false, v4)), v11);
            let v13: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("schema")), f55(v5)), v12);
            let v14: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("type")), f45(v6)), v13);
            let v17: List<(Bytes, Adt39)> = {
                let m60 = v7.clone();
                match m60.uncons() {
                    None => {
                        v14
                    }
                    Some((_, _)) => {
                        List::cons((f53(Str::lit("axioms")), Adt39::C3(std::rc::Rc::new(f4(f56(Fn8::F57, List::<Adt39>::nil(), f58(v7)))))), v14)
                    }
                }
            };
            f59(v17)
        }
        Adt8::C1(v18, v19, v20, v21, v22, v23, v24, v25, v26, v27) => {
            let v28: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("kind")), Adt39::C2(f53(Str::lit("contract")))), List::<(Bytes, Adt39)>::nil());
            let v29: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("name")), Adt39::C2(v18)), v28);
            let v32: List<(Bytes, Adt39)> = {
                let m61 = v19.clone();
                match m61.uncons() {
                    None => {
                        v29
                    }
                    Some((_, _)) => {
                        List::cons((f53(Str::lit("type_parameters")), Adt39::C3(std::rc::Rc::new(f4(f56(Fn8::F60, List::<Adt39>::nil(), f58(v19)))))), v29)
                    }
                }
            };
            let v33: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("input")), f61(v20)), v32);
            let v34: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("output")), f61(v21)), v33);
            let v36: List<(Bytes, Adt39)> = {
                let m62 = v22;
                match m62 {
                    None => {
                        v34
                    }
                    Some(v35) => {
                        List::cons((f53(Str::lit("state")), f62(v35)), v34)
                    }
                }
            };
            let v38: List<(Bytes, Adt39)> = {
                let m63 = v23;
                match m63 {
                    None => {
                        v36
                    }
                    Some(v37) => {
                        List::cons((f53(Str::lit("precondition")), f44({
                            let (h64, _) = v37;
                            h64
                        }, {
                            let (_, h65) = v37;
                            h65
                        })), v36)
                    }
                }
            };
            let v40: List<(Bytes, Adt39)> = {
                let m66 = v24;
                match m66 {
                    None => {
                        v38
                    }
                    Some(v39) => {
                        List::cons((f53(Str::lit("postcondition")), f44({
                            let (h67, _) = v39;
                            h67
                        }, {
                            let (_, h68) = v39;
                            h68
                        })), v38)
                    }
                }
            };
            let v42: List<(Bytes, Adt39)> = {
                let m69 = v25;
                match m69 {
                    None => {
                        v40
                    }
                    Some(v41) => {
                        List::cons((f53(Str::lit("invariant")), f44({
                            let (h70, _) = v41;
                            h70
                        }, {
                            let (_, h71) = v41;
                            h71
                        })), v40)
                    }
                }
            };
            let v45: List<(Bytes, Adt39)> = {
                let m72 = v26.clone();
                match m72.uncons() {
                    None => {
                        v42
                    }
                    Some((_, _)) => {
                        List::cons((f53(Str::lit("validators")), Adt39::C3(std::rc::Rc::new(f4(f63(Fn9::F64, List::<Adt39>::nil(), f65(v26)))))), v42)
                    }
                }
            };
            let v48: List<(Bytes, Adt39)> = {
                let m73 = v27.clone();
                match m73.uncons() {
                    None => {
                        v45
                    }
                    Some((_, _)) => {
                        List::cons((f53(Str::lit("axioms")), Adt39::C3(std::rc::Rc::new(f4(f56(Fn8::F66, List::<Adt39>::nil(), f58(v27)))))), v45)
                    }
                }
            };
            f59(v48)
        }
        Adt8::C2(v49, v50, v51, v52, v53, v54, v55, v56) => {
            let v57: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("kind")), Adt39::C2(f53(Str::lit("realization")))), List::<(Bytes, Adt39)>::nil());
            let v58: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("name")), Adt39::C2(v49)), v57);
            let v61: List<(Bytes, Adt39)> = {
                let m74 = v50.clone();
                match m74.uncons() {
                    None => {
                        v58
                    }
                    Some((_, _)) => {
                        List::cons((f53(Str::lit("type_parameters")), Adt39::C3(std::rc::Rc::new(f4(f56(Fn8::F67, List::<Adt39>::nil(), f58(v50)))))), v58)
                    }
                }
            };
            let v62: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("input")), f61(v51)), v61);
            let v63: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("output")), f45(v52)), v62);
            let v65: List<(Bytes, Adt39)> = {
                let m75 = v53;
                match m75 {
                    None => {
                        v63
                    }
                    Some(v64) => {
                        List::cons((f53(Str::lit("state")), f68(v64)), v63)
                    }
                }
            };
            let v66: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("descriptor")), f69(v54)), v65);
            let v67: List<(Bytes, Adt39)> = if v55 {
                List::cons((f53(Str::lit("executable")), Adt39::C0(true)), v66)
            } else {
                v66
            };
            let v70: List<(Bytes, Adt39)> = {
                let m76 = v56.clone();
                match m76.uncons() {
                    None => {
                        v67
                    }
                    Some((_, _)) => {
                        List::cons((f53(Str::lit("axioms")), Adt39::C3(std::rc::Rc::new(f4(f56(Fn8::F70, List::<Adt39>::nil(), f58(v56)))))), v67)
                    }
                }
            };
            f59(v70)
        }
        Adt8::C3(v71, v72, v73, v74, v75, v76) => {
            let v77: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("kind")), Adt39::C2(f53(Str::lit("evidence")))), List::<(Bytes, Adt39)>::nil());
            let v78: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("name")), Adt39::C2(v71)), v77);
            let v81: List<(Bytes, Adt39)> = {
                let m77 = v72.clone();
                match m77.uncons() {
                    None => {
                        v78
                    }
                    Some((_, _)) => {
                        List::cons((f53(Str::lit("type_parameters")), Adt39::C3(std::rc::Rc::new(f4(f56(Fn8::F71, List::<Adt39>::nil(), f58(v72)))))), v78)
                    }
                }
            };
            let v82: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("contract")), f72(v73)), v81);
            let v83: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("realization")), f72(v74)), v82);
            let v84: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("claims")), Adt39::C3(std::rc::Rc::new(f4(f73(Fn10::F74, List::<Adt39>::nil(), f75(v75)))))), v83);
            let v87: List<(Bytes, Adt39)> = {
                let m78 = v76.clone();
                match m78.uncons() {
                    None => {
                        v84
                    }
                    Some((_, _)) => {
                        List::cons((f53(Str::lit("axioms")), Adt39::C3(std::rc::Rc::new(f4(f56(Fn8::F76, List::<Adt39>::nil(), f58(v76)))))), v84)
                    }
                }
            };
            f59(v87)
        }
        Adt8::C4(v88, v89, v90, v91, v92, v93, v94) => {
            let v95: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("kind")), Adt39::C2(f53(Str::lit("model")))), List::<(Bytes, Adt39)>::nil());
            let v96: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("name")), Adt39::C2(v88)), v95);
            let v99: List<(Bytes, Adt39)> = {
                let m79 = v89.clone();
                match m79.uncons() {
                    None => {
                        v96
                    }
                    Some((_, _)) => {
                        List::cons((f53(Str::lit("type_parameters")), Adt39::C3(std::rc::Rc::new(f4(f56(Fn8::F77, List::<Adt39>::nil(), f58(v89)))))), v96)
                    }
                }
            };
            let v100: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("contract")), f72(v90)), v99);
            let v101: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("realization")), f72(v91)), v100);
            let v104: List<(Bytes, Adt39)> = {
                let m80 = v92.clone();
                match m80.uncons() {
                    None => {
                        v101
                    }
                    Some((_, _)) => {
                        List::cons((f53(Str::lit("evidence")), Adt39::C3(std::rc::Rc::new(f4(f78(Fn11::F79, List::<Adt39>::nil(), f80(v92)))))), v101)
                    }
                }
            };
            let v107: List<(Bytes, Adt39)> = {
                let m81 = v93.clone();
                match m81.uncons() {
                    None => {
                        v104
                    }
                    Some((_, _)) => {
                        List::cons((f53(Str::lit("entry")), Adt39::C3(std::rc::Rc::new(f4(f81(Fn12::F82, List::<Adt39>::nil(), f83(v93)))))), v104)
                    }
                }
            };
            let v110: List<(Bytes, Adt39)> = {
                let m82 = v94.clone();
                match m82.uncons() {
                    None => {
                        v107
                    }
                    Some((_, _)) => {
                        List::cons((f53(Str::lit("axioms")), Adt39::C3(std::rc::Rc::new(f4(f56(Fn8::F84, List::<Adt39>::nil(), f58(v94)))))), v107)
                    }
                }
            };
            f59(v110)
        }
    }
}

pub fn f27(v0: Adt27) -> Adt39 {
    let m83 = v0;
    match m83 {
        Adt27::C0(v1, v2, v3, v4, v5, v6, v7) => {
            let v8: List<(Bytes, Adt39)> = List::cons((f3(Str::lit("kind")), Adt39::C2(f3(Str::lit("logic")))), List::<(Bytes, Adt39)>::nil());
            let v9: List<(Bytes, Adt39)> = List::cons((f3(Str::lit("name")), Adt39::C2(v1)), v8);
            let v12: List<(Bytes, Adt39)> = {
                let m84 = v2.clone();
                match m84.uncons() {
                    None => {
                        v9
                    }
                    Some((_, _)) => {
                        List::cons((f3(Str::lit("type_parameters")), Adt39::C3(std::rc::Rc::new(f4(f56(Fn8::F85, List::<Adt39>::nil(), f58(v2)))))), v9)
                    }
                }
            };
            let v13: List<(Bytes, Adt39)> = List::cons((f3(Str::lit("state")), f62(v3)), v12);
            let v14: List<(Bytes, Adt39)> = List::cons((f3(Str::lit("relation")), f44({
                let (h85, _) = v4;
                h85
            }, {
                let (_, h86) = v4;
                h86
            })), v13);
            let v16: List<(Bytes, Adt39)> = {
                let m87 = v5;
                match m87 {
                    None => {
                        v14
                    }
                    Some(v15) => {
                        List::cons((f3(Str::lit("invariant")), f86(v15)), v14)
                    }
                }
            };
            let v18: List<(Bytes, Adt39)> = {
                let m88 = v6;
                match m88 {
                    None => {
                        v16
                    }
                    Some(v17) => {
                        List::cons((f3(Str::lit("ranking")), f44({
                            let (h89, _) = v17;
                            h89
                        }, {
                            let (_, h90) = v17;
                            h90
                        })), v16)
                    }
                }
            };
            let v21: List<(Bytes, Adt39)> = {
                let m91 = v7.clone();
                match m91.uncons() {
                    None => {
                        v18
                    }
                    Some((_, _)) => {
                        List::cons((f3(Str::lit("axioms")), Adt39::C3(std::rc::Rc::new(f4(f56(Fn8::F87, List::<Adt39>::nil(), f58(v7)))))), v18)
                    }
                }
            };
            f2(v21)
        }
        Adt27::C1(v22, v23, v24, v25, v26, v27, v28, v29, v30, v31) => {
            let v32: List<(Bytes, Adt39)> = List::cons((f3(Str::lit("kind")), Adt39::C2(f3(Str::lit("inference_rule")))), List::<(Bytes, Adt39)>::nil());
            let v33: List<(Bytes, Adt39)> = List::cons((f3(Str::lit("name")), Adt39::C2(v22)), v32);
            let v36: List<(Bytes, Adt39)> = {
                let m92 = v23.clone();
                match m92.uncons() {
                    None => {
                        v33
                    }
                    Some((_, _)) => {
                        List::cons((f3(Str::lit("type_parameters")), Adt39::C3(std::rc::Rc::new(f4(f56(Fn8::F88, List::<Adt39>::nil(), f58(v23)))))), v33)
                    }
                }
            };
            let v37: List<(Bytes, Adt39)> = List::cons((f3(Str::lit("logic")), f72(v24)), v36);
            let v39: List<(Bytes, Adt39)> = {
                let m93 = v25;
                match m93 {
                    None => {
                        v37
                    }
                    Some(v38) => {
                        List::cons((f3(Str::lit("binding")), f89(v38)), v37)
                    }
                }
            };
            let v40: List<(Bytes, Adt39)> = List::cons((f3(Str::lit("guard")), f46(v26)), v39);
            let v41: List<(Bytes, Adt39)> = List::cons((f3(Str::lit("conclusion")), f46(v27)), v40);
            let v42: List<(Bytes, Adt39)> = List::cons((f3(Str::lit("soundness")), f44({
                let (h94, _) = v28;
                h94
            }, {
                let (_, h95) = v28;
                h95
            })), v41);
            let v44: List<(Bytes, Adt39)> = {
                let m96 = v29;
                match m96 {
                    None => {
                        v42
                    }
                    Some(v43) => {
                        List::cons((f3(Str::lit("progress")), f44({
                            let (h97, _) = v43;
                            h97
                        }, {
                            let (_, h98) = v43;
                            h98
                        })), v42)
                    }
                }
            };
            let v45: List<(Bytes, Adt39)> = if v30 {
                List::cons((f3(Str::lit("executable")), Adt39::C0(true)), v44)
            } else {
                v44
            };
            let v48: List<(Bytes, Adt39)> = {
                let m99 = v31.clone();
                match m99.uncons() {
                    None => {
                        v45
                    }
                    Some((_, _)) => {
                        List::cons((f3(Str::lit("axioms")), Adt39::C3(std::rc::Rc::new(f4(f56(Fn8::F90, List::<Adt39>::nil(), f58(v31)))))), v45)
                    }
                }
            };
            f2(v48)
        }
        Adt27::C2(v49, v50, v51, v52, v53, v54, v55, v56, v57) => {
            let v58: List<(Bytes, Adt39)> = List::cons((f3(Str::lit("kind")), Adt39::C2(f3(Str::lit("verifier")))), List::<(Bytes, Adt39)>::nil());
            let v59: List<(Bytes, Adt39)> = List::cons((f3(Str::lit("name")), Adt39::C2(v49)), v58);
            let v62: List<(Bytes, Adt39)> = {
                let m100 = v50.clone();
                match m100.uncons() {
                    None => {
                        v59
                    }
                    Some((_, _)) => {
                        List::cons((f3(Str::lit("type_parameters")), Adt39::C3(std::rc::Rc::new(f4(f56(Fn8::F91, List::<Adt39>::nil(), f58(v50)))))), v59)
                    }
                }
            };
            let v63: List<(Bytes, Adt39)> = List::cons((f3(Str::lit("subject")), f61(v51)), v62);
            let v64: List<(Bytes, Adt39)> = List::cons((f3(Str::lit("candidate")), f61(v52)), v63);
            let v65: List<(Bytes, Adt39)> = List::cons((f3(Str::lit("specification")), f44({
                let (h101, _) = v53;
                h101
            }, {
                let (_, h102) = v53;
                h102
            })), v64);
            let v66: List<(Bytes, Adt39)> = List::cons((f3(Str::lit("check")), f44({
                let (h103, _) = v54;
                h103
            }, {
                let (_, h104) = v54;
                h104
            })), v65);
            let v67: List<(Bytes, Adt39)> = List::cons((f3(Str::lit("sound")), f44({
                let (h105, _) = v55;
                h105
            }, {
                let (_, h106) = v55;
                h106
            })), v66);
            let v69: List<(Bytes, Adt39)> = {
                let m107 = v56;
                match m107 {
                    None => {
                        v67
                    }
                    Some(v68) => {
                        List::cons((f3(Str::lit("complete")), f44({
                            let (h108, _) = v68;
                            h108
                        }, {
                            let (_, h109) = v68;
                            h109
                        })), v67)
                    }
                }
            };
            let v72: List<(Bytes, Adt39)> = {
                let m110 = v57.clone();
                match m110.uncons() {
                    None => {
                        v69
                    }
                    Some((_, _)) => {
                        List::cons((f3(Str::lit("axioms")), Adt39::C3(std::rc::Rc::new(f4(f56(Fn8::F92, List::<Adt39>::nil(), f58(v57)))))), v69)
                    }
                }
            };
            f2(v72)
        }
        Adt27::C3(v73, v74, v75, v76, v77, v78, v79, v80, v81, v82, v83, v84) => {
            let v85: List<(Bytes, Adt39)> = List::cons((f3(Str::lit("kind")), Adt39::C2(f3(Str::lit("reasoner")))), List::<(Bytes, Adt39)>::nil());
            let v86: List<(Bytes, Adt39)> = List::cons((f3(Str::lit("name")), Adt39::C2(v73)), v85);
            let v89: List<(Bytes, Adt39)> = {
                let m111 = v74.clone();
                match m111.uncons() {
                    None => {
                        v86
                    }
                    Some((_, _)) => {
                        List::cons((f3(Str::lit("type_parameters")), Adt39::C3(std::rc::Rc::new(f4(f56(Fn8::F93, List::<Adt39>::nil(), f58(v74)))))), v86)
                    }
                }
            };
            let v91: List<(Bytes, Adt39)> = {
                let m112 = v75;
                match m112 {
                    None => {
                        v89
                    }
                    Some(v90) => {
                        List::cons((f3(Str::lit("logic")), f72(v90)), v89)
                    }
                }
            };
            let v92: List<(Bytes, Adt39)> = List::cons((f3(Str::lit("observation")), f61(v76)), v91);
            let v94: List<(Bytes, Adt39)> = {
                let m113 = v77;
                match m113 {
                    None => {
                        v92
                    }
                    Some(v93) => {
                        List::cons((f3(Str::lit("observe")), f46(v93)), v92)
                    }
                }
            };
            let v95: List<(Bytes, Adt39)> = List::cons((f3(Str::lit("rules")), Adt39::C3(std::rc::Rc::new(f4(f78(Fn11::F94, List::<Adt39>::nil(), f80(v78)))))), v94);
            let v96: List<(Bytes, Adt39)> = List::cons((f3(Str::lit("strategy")), f95(v79)), v95);
            let v98: List<(Bytes, Adt39)> = {
                let m114 = v80;
                match m114 {
                    None => {
                        v96
                    }
                    Some(v97) => {
                        List::cons((f3(Str::lit("answer")), f96(v97)), v96)
                    }
                }
            };
            let v99: List<(Bytes, Adt39)> = List::cons((f3(Str::lit("verifier")), f72(v81)), v98);
            let v102: List<(Bytes, Adt39)> = {
                let m115 = v82.clone();
                match m115.uncons() {
                    None => {
                        v99
                    }
                    Some((_, _)) => {
                        List::cons((f3(Str::lit("claims")), Adt39::C3(std::rc::Rc::new(f4(f97(Fn13::F98, List::<Adt39>::nil(), f99(v82)))))), v99)
                    }
                }
            };
            let v103: List<(Bytes, Adt39)> = if v83 {
                List::cons((f3(Str::lit("executable")), Adt39::C0(true)), v102)
            } else {
                v102
            };
            let v106: List<(Bytes, Adt39)> = {
                let m116 = v84.clone();
                match m116.uncons() {
                    None => {
                        v103
                    }
                    Some((_, _)) => {
                        List::cons((f3(Str::lit("axioms")), Adt39::C3(std::rc::Rc::new(f4(f56(Fn8::F100, List::<Adt39>::nil(), f58(v84)))))), v103)
                    }
                }
            };
            f2(v106)
        }
        Adt27::C4(v107, v108, v109, v110, v111) => {
            let v112: List<(Bytes, Adt39)> = List::cons((f3(Str::lit("kind")), Adt39::C2(f3(Str::lit("tokenizer")))), List::<(Bytes, Adt39)>::nil());
            let v113: List<(Bytes, Adt39)> = List::cons((f3(Str::lit("name")), Adt39::C2(v107)), v112);
            let v116: List<(Bytes, Adt39)> = {
                let m117 = v108.clone();
                match m117.uncons() {
                    None => {
                        v113
                    }
                    Some((_, _)) => {
                        List::cons((f3(Str::lit("type_parameters")), Adt39::C3(std::rc::Rc::new(f4(f56(Fn8::F101, List::<Adt39>::nil(), f58(v108)))))), v113)
                    }
                }
            };
            let v117: List<(Bytes, Adt39)> = List::cons((f3(Str::lit("scheme")), f102(v109)), v116);
            let v118: List<(Bytes, Adt39)> = List::cons((f3(Str::lit("vocabulary")), f72(v110)), v117);
            let v121: List<(Bytes, Adt39)> = {
                let m118 = v111.clone();
                match m118.uncons() {
                    None => {
                        v118
                    }
                    Some((_, _)) => {
                        List::cons((f3(Str::lit("axioms")), Adt39::C3(std::rc::Rc::new(f4(f56(Fn8::F103, List::<Adt39>::nil(), f58(v111)))))), v118)
                    }
                }
            };
            f2(v121)
        }
        Adt27::C5(v122, v123, v124, v125) => {
            let v126: List<(Bytes, Adt39)> = List::cons((f3(Str::lit("kind")), Adt39::C2(f3(Str::lit("context_window")))), List::<(Bytes, Adt39)>::nil());
            let v127: List<(Bytes, Adt39)> = List::cons((f3(Str::lit("name")), Adt39::C2(v122)), v126);
            let v128: List<(Bytes, Adt39)> = List::cons((f3(Str::lit("capacity")), Adt39::C1(false, v123)), v127);
            let v129: List<(Bytes, Adt39)> = List::cons((f3(Str::lit("overflow")), f104(v124)), v128);
            let v132: List<(Bytes, Adt39)> = {
                let m119 = v125.clone();
                match m119.uncons() {
                    None => {
                        v129
                    }
                    Some((_, _)) => {
                        List::cons((f3(Str::lit("axioms")), Adt39::C3(std::rc::Rc::new(f4(f56(Fn8::F105, List::<Adt39>::nil(), f58(v125)))))), v129)
                    }
                }
            };
            f2(v132)
        }
        Adt27::C6(v133, v134, v135, v136, v137, v138, v139, v140) => {
            let v141: List<(Bytes, Adt39)> = List::cons((f3(Str::lit("kind")), Adt39::C2(f3(Str::lit("text_generator")))), List::<(Bytes, Adt39)>::nil());
            let v142: List<(Bytes, Adt39)> = List::cons((f3(Str::lit("name")), Adt39::C2(v133)), v141);
            let v143: List<(Bytes, Adt39)> = List::cons((f3(Str::lit("tokenizer")), f44({
                let (h120, _) = v134;
                h120
            }, {
                let (_, h121) = v134;
                h121
            })), v142);
            let v144: List<(Bytes, Adt39)> = List::cons((f3(Str::lit("window")), f44({
                let (h122, _) = v135;
                h122
            }, {
                let (_, h123) = v135;
                h123
            })), v143);
            let v145: List<(Bytes, Adt39)> = List::cons((f3(Str::lit("scorer")), f72(v136)), v144);
            let v146: List<(Bytes, Adt39)> = List::cons((f3(Str::lit("max_tokens")), Adt39::C1(false, v137)), v145);
            let v147: List<(Bytes, Adt39)> = List::cons((f3(Str::lit("stop")), f44({
                let (h124, _) = v138;
                h124
            }, {
                let (_, h125) = v138;
                h125
            })), v146);
            let v148: List<(Bytes, Adt39)> = List::cons((f3(Str::lit("policy")), f106(v139)), v147);
            let v151: List<(Bytes, Adt39)> = {
                let m126 = v140.clone();
                match m126.uncons() {
                    None => {
                        v148
                    }
                    Some((_, _)) => {
                        List::cons((f3(Str::lit("axioms")), Adt39::C3(std::rc::Rc::new(f4(f56(Fn8::F107, List::<Adt39>::nil(), f58(v140)))))), v148)
                    }
                }
            };
            f2(v151)
        }
        Adt27::C7(v152, v153, v154, v155) => {
            let v156: List<(Bytes, Adt39)> = List::cons((f3(Str::lit("kind")), Adt39::C2(f3(Str::lit("text_classifier")))), List::<(Bytes, Adt39)>::nil());
            let v157: List<(Bytes, Adt39)> = List::cons((f3(Str::lit("name")), Adt39::C2(v152)), v156);
            let v158: List<(Bytes, Adt39)> = List::cons((f3(Str::lit("tokenizer")), f44({
                let (h127, _) = v153;
                h127
            }, {
                let (_, h128) = v153;
                h128
            })), v157);
            let v159: List<(Bytes, Adt39)> = List::cons((f3(Str::lit("model")), f72(v154)), v158);
            let v162: List<(Bytes, Adt39)> = {
                let m129 = v155.clone();
                match m129.uncons() {
                    None => {
                        v159
                    }
                    Some((_, _)) => {
                        List::cons((f3(Str::lit("axioms")), Adt39::C3(std::rc::Rc::new(f4(f56(Fn8::F108, List::<Adt39>::nil(), f58(v155)))))), v159)
                    }
                }
            };
            f2(v162)
        }
    }
}

pub fn f28(v0: u64) -> R<Bytes> {
    f109(f110(), 10u64, v0)
}

pub fn f29(v0: Fn5, p134: u64, p135: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m133 = (p134, p135);
    {
        let mut m133 = m133;
        loop {
            let step = {
                let (v1, v2) = m133;
                let m130 = v1;
                if m130 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m130 - 1;
                    let m132 = {
                        let c131: Fn5 = v0.clone();
                        c131.apply(v2.clone())?
                    };
                    match m132 {
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
                Ok(next) => { m133 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f30(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f111(v0, v1)
}

pub fn f31(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m136 = f112(Fn14::F113(v1), None::<Bytes>, v0);
    match m136 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f32(v0: Fn6, p140: List<(Bytes, Bytes)>, p141: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m139 = (p140, p141);
    {
        let mut m139 = m139;
        loop {
            let step = {
                let (v1, v2) = m139;
                let m137 = v2;
                match m137.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c138: Fn6 = v0.clone();
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

pub fn f33(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    f114(v0, v1)
}

pub fn f34(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    f115(v0)
}

pub fn f35(v0: Fn7, p145: List<Bytes>, p146: List<(Bytes, Bytes)>) -> R<List<Bytes>> {
    let m144 = (p145, p146);
    {
        let mut m144 = m144;
        loop {
            let step = {
                let (v1, v2) = m144;
                let m142 = v2;
                match m142.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(({
                            let c143: Fn7 = v0.clone();
                            c143.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m144 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f36(v0: List<Bytes>, v1: (Bytes, Bytes)) -> R<List<Bytes>> {
    Ok(List::cons(f116(v1)?, v0))
}

pub fn f37(v0: Str, v1: Adt39) -> Adt39 {
    Adt39::C4(std::rc::Rc::new(f38(Str::lit("kind"), f39(f117(v0)), v1)))
}

pub fn f38(v0: Str, v1: Adt39, v2: Adt39) -> Adt39 {
    Adt39::C7(f117(v0), std::rc::Rc::new(v1), std::rc::Rc::new(v2))
}

pub fn f39(v0: Bytes) -> Adt39 {
    Adt39::C2(v0)
}

pub fn f40(v0: List<Bytes>) -> Adt39 {
    Adt39::C3(std::rc::Rc::new(f4(f56(Fn8::F118, List::<Adt39>::nil(), f58(v0)))))
}

pub fn f41(v0: List<(Bytes, Adt2)>) -> Adt39 {
    Adt39::C3(std::rc::Rc::new(f4(f119(Fn15::F120, List::<Adt39>::nil(), f121(v0)))))
}

pub fn f42(v0: List<(Bytes, Adt2)>) -> Adt39 {
    f41(v0)
}

pub fn f43() -> Adt39 {
    Adt39::C5
}

pub fn f44(v0: Option<Bytes>, v1: Bytes) -> Adt39 {
    Adt39::C4(std::rc::Rc::new({
        let m147 = v0;
        match m147 {
            None => {
                f122(Str::lit("name"), Adt39::C2(v1), Adt39::C5)
            }
            Some(v2) => {
                f122(Str::lit("module"), Adt39::C2(v2), f122(Str::lit("name"), Adt39::C2(v1), Adt39::C5))
            }
        }
    }))
}

pub fn f45(v0: Adt2) -> Adt39 {
    let m148 = v0;
    match m148 {
        Adt2::C0 => {
            f123(Str::lit("type"), Adt39::C5)
        }
        Adt2::C1 => {
            f123(Str::lit("nat"), Adt39::C5)
        }
        Adt2::C2 => {
            f123(Str::lit("bool"), Adt39::C5)
        }
        Adt2::C3 => {
            f123(Str::lit("prop"), Adt39::C5)
        }
        Adt2::C4 => {
            f123(Str::lit("unit"), Adt39::C5)
        }
        Adt2::C5 => {
            f123(Str::lit("int"), Adt39::C5)
        }
        Adt2::C6 => {
            f123(Str::lit("int8"), Adt39::C5)
        }
        Adt2::C7 => {
            f123(Str::lit("int16"), Adt39::C5)
        }
        Adt2::C8 => {
            f123(Str::lit("int32"), Adt39::C5)
        }
        Adt2::C9 => {
            f123(Str::lit("int64"), Adt39::C5)
        }
        Adt2::C10 => {
            f123(Str::lit("uint8"), Adt39::C5)
        }
        Adt2::C11 => {
            f123(Str::lit("uint16"), Adt39::C5)
        }
        Adt2::C12 => {
            f123(Str::lit("uint32"), Adt39::C5)
        }
        Adt2::C13 => {
            f123(Str::lit("uint64"), Adt39::C5)
        }
        Adt2::C14 => {
            f123(Str::lit("string"), Adt39::C5)
        }
        Adt2::C15 => {
            f123(Str::lit("bytes"), Adt39::C5)
        }
        Adt2::C16 => {
            f123(Str::lit("ordering"), Adt39::C5)
        }
        Adt2::C17 => {
            f123(Str::lit("contract_violation"), Adt39::C5)
        }
        Adt2::C18 => {
            f123(Str::lit("reasoning_failure"), Adt39::C5)
        }
        Adt2::C19(v1) => {
            f123(Str::lit("parameter"), f124(Str::lit("name"), Adt39::C2(v1), Adt39::C5))
        }
        Adt2::C20(r150) => {
            let v2 = (*r150).clone();
            f123(Str::lit("option"), f124(Str::lit("value"), f45(v2), Adt39::C5))
        }
        Adt2::C21(r151, r152) => {
            let v3 = (*r151).clone();
            let v4 = (*r152).clone();
            f123(Str::lit("result"), f124(Str::lit("ok"), f45(v3), f124(Str::lit("error"), f45(v4), Adt39::C5)))
        }
        Adt2::C22(r153) => {
            let v5 = (*r153).clone();
            f123(Str::lit("list"), f124(Str::lit("element"), f45(v5), Adt39::C5))
        }
        Adt2::C23(v6, v7, r154) => {
            let v8 = (*r154).clone();
            f123(Str::lit("named"), f124(Str::lit("member"), Adt39::C4(std::rc::Rc::new({
                let m149 = v6;
                match m149 {
                    None => {
                        f124(Str::lit("name"), Adt39::C2(v7), Adt39::C5)
                    }
                    Some(v9) => {
                        f124(Str::lit("module"), Adt39::C2(v9), f124(Str::lit("name"), Adt39::C2(v7), Adt39::C5))
                    }
                }
            })), f124(Str::lit("arguments"), Adt39::C3(std::rc::Rc::new(f45(v8))), Adt39::C5)))
        }
        Adt2::C24(r155, r156) => {
            let v10 = (*r155).clone();
            let v11 = (*r156).clone();
            f123(Str::lit("product"), f124(Str::lit("left"), f45(v10), f124(Str::lit("right"), f45(v11), Adt39::C5)))
        }
        Adt2::C25(r157, r158) => {
            let v12 = (*r157).clone();
            let v13 = (*r158).clone();
            f123(Str::lit("function"), f124(Str::lit("parameters"), Adt39::C3(std::rc::Rc::new(f45(v12))), f124(Str::lit("result"), f45(v13), Adt39::C5)))
        }
        Adt2::C26(r159, r160) => {
            let v14 = (*r159).clone();
            let v15 = (*r160).clone();
            f123(Str::lit("map"), f124(Str::lit("key"), f45(v14), f124(Str::lit("value"), f45(v15), Adt39::C5)))
        }
        Adt2::C27(r161) => {
            let v16 = (*r161).clone();
            f123(Str::lit("set"), f124(Str::lit("element"), f45(v16), Adt39::C5))
        }
        Adt2::C28 => {
            Adt39::C5
        }
        Adt2::C29(r162, r163) => {
            let v17 = (*r162).clone();
            let v18 = (*r163).clone();
            Adt39::C6(std::rc::Rc::new(f45(v17)), std::rc::Rc::new(f45(v18)))
        }
    }
}

pub fn f46(v0: Adt3) -> Adt39 {
    let m164 = v0;
    match m164 {
        Adt3::C0(v1) => {
            f125(Str::lit("var"), f122(Str::lit("name"), Adt39::C2(v1), Adt39::C5))
        }
        Adt3::C1(v2) => {
            f125(Str::lit("nat"), f122(Str::lit("value"), Adt39::C2(v2), Adt39::C5))
        }
        Adt3::C2(v3, v4) => {
            f125(Str::lit("integer"), f122(Str::lit("representation"), Adt39::C2(f126(v3)), f122(Str::lit("value"), Adt39::C2(v4), Adt39::C5)))
        }
        Adt3::C3(v5) => {
            f125(Str::lit("string"), f122(Str::lit("value"), Adt39::C2(v5), Adt39::C5))
        }
        Adt3::C4(v6) => {
            f125(Str::lit("bytes"), f122(Str::lit("hex"), Adt39::C2(v6), Adt39::C5))
        }
        Adt3::C5(v7, r167, v9) => {
            let v8 = (*r167).clone();
            f125(Str::lit("primitive"), f122(Str::lit("operation"), Adt39::C2({
                let m166 = f127(v7);
                match m166 {
                    Adt40::C0(h165, _, _, _) => {
                        h165
                    }
                }
            }), f122(Str::lit("arguments"), Adt39::C3(std::rc::Rc::new(f46(v8))), f122(Str::lit("result"), f45(v9), Adt39::C5))))
        }
        Adt3::C6(v10) => {
            f125(Str::lit("bool"), f122(Str::lit("value"), Adt39::C0(v10), Adt39::C5))
        }
        Adt3::C7 => {
            f125(Str::lit("unit"), Adt39::C5)
        }
        Adt3::C8(v11) => {
            f125(Str::lit("nil"), f122(Str::lit("element"), f45(v11), Adt39::C5))
        }
        Adt3::C9(r168, r169) => {
            let v12 = (*r168).clone();
            let v13 = (*r169).clone();
            f125(Str::lit("cons"), f122(Str::lit("head"), f46(v12), f122(Str::lit("tail"), f46(v13), Adt39::C5)))
        }
        Adt3::C10(v14, v15, v16, r170) => {
            let v17 = (*r170).clone();
            f125(Str::lit("record"), f122(Str::lit("type"), f44(v14, v15), f128(Str::lit("type_arguments"), v16, f122(Str::lit("fields"), Adt39::C3(std::rc::Rc::new(f46(v17))), Adt39::C5))))
        }
        Adt3::C11(v18, v19, v20, r171) => {
            let v21 = (*r171).clone();
            f125(Str::lit("constructor"), f122(Str::lit("constructor"), f44(v18, v19), f128(Str::lit("type_arguments"), v20, f122(Str::lit("arguments"), Adt39::C3(std::rc::Rc::new(f46(v21))), Adt39::C5))))
        }
        Adt3::C12(v22, v23, v24, v25, v26) => {
            f125(Str::lit("instance_value"), f122(Str::lit("class"), f44(v22, v23), f122(Str::lit("arguments"), Adt39::C3(std::rc::Rc::new(f45(v24))), f122(Str::lit("resolved"), f44(v25, v26), Adt39::C5))))
        }
        Adt3::C13(r172, v28) => {
            let v27 = (*r172).clone();
            f125(Str::lit("project"), f122(Str::lit("value"), f46(v27), f122(Str::lit("field"), Adt39::C2(v28), Adt39::C5)))
        }
        Adt3::C14(v29, v30, v31, r173) => {
            let v32 = (*r173).clone();
            f125(Str::lit("call"), f122(Str::lit("function"), f44(v29, v30), f128(Str::lit("type_arguments"), v31, f122(Str::lit("arguments"), Adt39::C3(std::rc::Rc::new(f46(v32))), Adt39::C5))))
        }
        Adt3::C15(r174, r175, r176) => {
            let v33 = (*r174).clone();
            let v34 = (*r175).clone();
            let v35 = (*r176).clone();
            f125(Str::lit("if"), f122(Str::lit("condition"), f46(v33), f122(Str::lit("then_value"), f46(v34), f122(Str::lit("else_value"), f46(v35), Adt39::C5))))
        }
        Adt3::C16(r177, r178) => {
            let v36 = (*r177).clone();
            let v37 = (*r178).clone();
            f125(Str::lit("match"), f122(Str::lit("scrutinee"), f46(v36), f122(Str::lit("branches"), Adt39::C3(std::rc::Rc::new(f46(v37))), Adt39::C5)))
        }
        Adt3::C17(r179, r180) => {
            let v38 = (*r179).clone();
            let v39 = (*r180).clone();
            f125(Str::lit("eq"), f122(Str::lit("left"), f46(v38), f122(Str::lit("right"), f46(v39), Adt39::C5)))
        }
        Adt3::C18(r181, r182) => {
            let v40 = (*r181).clone();
            let v41 = (*r182).clone();
            f125(Str::lit("le"), f122(Str::lit("left"), f46(v40), f122(Str::lit("right"), f46(v41), Adt39::C5)))
        }
        Adt3::C19(r183, r184) => {
            let v42 = (*r183).clone();
            let v43 = (*r184).clone();
            f125(Str::lit("lt"), f122(Str::lit("left"), f46(v42), f122(Str::lit("right"), f46(v43), Adt39::C5)))
        }
        Adt3::C20(r185, r186) => {
            let v44 = (*r185).clone();
            let v45 = (*r186).clone();
            f125(Str::lit("add"), f122(Str::lit("left"), f46(v44), f122(Str::lit("right"), f46(v45), Adt39::C5)))
        }
        Adt3::C21(r187, r188) => {
            let v46 = (*r187).clone();
            let v47 = (*r188).clone();
            f125(Str::lit("beq"), f122(Str::lit("left"), f46(v46), f122(Str::lit("right"), f46(v47), Adt39::C5)))
        }
        Adt3::C22(r189, r190) => {
            let v48 = (*r189).clone();
            let v49 = (*r190).clone();
            f125(Str::lit("ble"), f122(Str::lit("left"), f46(v48), f122(Str::lit("right"), f46(v49), Adt39::C5)))
        }
        Adt3::C23(r191, r192) => {
            let v50 = (*r191).clone();
            let v51 = (*r192).clone();
            f125(Str::lit("blt"), f122(Str::lit("left"), f46(v50), f122(Str::lit("right"), f46(v51), Adt39::C5)))
        }
        Adt3::C24(r193, r194) => {
            let v52 = (*r193).clone();
            let v53 = (*r194).clone();
            f125(Str::lit("and"), f122(Str::lit("left"), f46(v52), f122(Str::lit("right"), f46(v53), Adt39::C5)))
        }
        Adt3::C25(r195, r196) => {
            let v54 = (*r195).clone();
            let v55 = (*r196).clone();
            f125(Str::lit("prop_and"), f122(Str::lit("left"), f46(v54), f122(Str::lit("right"), f46(v55), Adt39::C5)))
        }
        Adt3::C26(r197, r198) => {
            let v56 = (*r197).clone();
            let v57 = (*r198).clone();
            f125(Str::lit("or"), f122(Str::lit("left"), f46(v56), f122(Str::lit("right"), f46(v57), Adt39::C5)))
        }
        Adt3::C27(r199) => {
            let v58 = (*r199).clone();
            f125(Str::lit("not"), f122(Str::lit("value"), f46(v58), Adt39::C5))
        }
        Adt3::C28(r200, r201) => {
            let v59 = (*r200).clone();
            let v60 = (*r201).clone();
            f125(Str::lit("implies"), f122(Str::lit("premise"), f46(v59), f122(Str::lit("conclusion"), f46(v60), Adt39::C5)))
        }
        Adt3::C29(r202, r203) => {
            let v61 = (*r202).clone();
            let v62 = (*r203).clone();
            f125(Str::lit("iff"), f122(Str::lit("left"), f46(v61), f122(Str::lit("right"), f46(v62), Adt39::C5)))
        }
        Adt3::C30(v63, v64, r204) => {
            let v65 = (*r204).clone();
            f125(Str::lit("forall"), f122(Str::lit("binder"), f129(v63, v64), f122(Str::lit("body"), f46(v65), Adt39::C5)))
        }
        Adt3::C31(v66, v67, r205, r206) => {
            let v68 = (*r205).clone();
            let v69 = (*r206).clone();
            f125(Str::lit("let"), f122(Str::lit("binder"), f129(v66, v67), f122(Str::lit("value"), f46(v68), f122(Str::lit("body"), f46(v69), Adt39::C5))))
        }
        Adt3::C32(r207, r208) => {
            let v70 = (*r207).clone();
            let v71 = (*r208).clone();
            f125(Str::lit("pair"), f122(Str::lit("left"), f46(v70), f122(Str::lit("right"), f46(v71), Adt39::C5)))
        }
        Adt3::C33(r209) => {
            let v72 = (*r209).clone();
            f125(Str::lit("first"), f122(Str::lit("value"), f46(v72), Adt39::C5))
        }
        Adt3::C34(r210) => {
            let v73 = (*r210).clone();
            f125(Str::lit("second"), f122(Str::lit("value"), f46(v73), Adt39::C5))
        }
        Adt3::C35(v74, v75, r211) => {
            let v76 = (*r211).clone();
            f125(Str::lit("lambda"), f122(Str::lit("parameters"), f41(v74), f122(Str::lit("captures"), f40(v75), f122(Str::lit("body"), f46(v76), Adt39::C5))))
        }
        Adt3::C36(r212, r213) => {
            let v77 = (*r212).clone();
            let v78 = (*r213).clone();
            f125(Str::lit("apply"), f122(Str::lit("function"), f46(v77), f122(Str::lit("arguments"), Adt39::C3(std::rc::Rc::new(f46(v78))), Adt39::C5)))
        }
        Adt3::C37(v79, v80, v81) => {
            f125(Str::lit("function_ref"), f122(Str::lit("function"), f44(v79, v80), f128(Str::lit("type_arguments"), v81, Adt39::C5)))
        }
        Adt3::C38(v82, v83, r214) => {
            let v84 = (*r214).clone();
            f125(Str::lit("map_literal"), f122(Str::lit("key"), f45(v82), f122(Str::lit("value"), f45(v83), f122(Str::lit("entries"), Adt39::C3(std::rc::Rc::new(f46(v84))), Adt39::C5))))
        }
        Adt3::C39(v85, r215) => {
            let v86 = (*r215).clone();
            f125(Str::lit("set_literal"), f122(Str::lit("element"), f45(v85), f122(Str::lit("elements"), Adt39::C3(std::rc::Rc::new(f46(v86))), Adt39::C5)))
        }
        Adt3::C40(v87, r216, r217) => {
            let v88 = (*r216).clone();
            let v89 = (*r217).clone();
            f125(Str::lit("graph_literal"), f122(Str::lit("node"), f45(v87), f122(Str::lit("nodes"), Adt39::C3(std::rc::Rc::new(f46(v88))), f122(Str::lit("edges"), Adt39::C3(std::rc::Rc::new(f46(v89))), Adt39::C5))))
        }
        Adt3::C41(v90, v91, v92, r218, v94) => {
            let v93 = (*r218).clone();
            f125(Str::lit("checked_apply"), f122(Str::lit("model"), f44(v90, v91), f128(Str::lit("type_arguments"), v92, f122(Str::lit("arguments"), Adt39::C3(std::rc::Rc::new(f46(v93))), f122(Str::lit("checks"), f40(v94), Adt39::C5)))))
        }
        Adt3::C42 => {
            Adt39::C5
        }
        Adt3::C43(r219, r220) => {
            let v95 = (*r219).clone();
            let v96 = (*r220).clone();
            Adt39::C6(std::rc::Rc::new(f46(v95)), std::rc::Rc::new(f46(v96)))
        }
        Adt3::C44(v97, r221, r222) => {
            let v98 = (*r221).clone();
            let v99 = (*r222).clone();
            Adt39::C6(std::rc::Rc::new(Adt39::C4(std::rc::Rc::new(f122(Str::lit("field"), Adt39::C2(v97), f122(Str::lit("value"), f46(v98), Adt39::C5))))), std::rc::Rc::new(f46(v99)))
        }
        Adt3::C45(v100, v101, v102, r223, r224) => {
            let v103 = (*r223).clone();
            let v104 = (*r224).clone();
            Adt39::C6(std::rc::Rc::new(Adt39::C4(std::rc::Rc::new(f122(Str::lit("constructor"), f44(v100, v101), f122(Str::lit("binders"), f40(v102), f122(Str::lit("body"), f46(v103), Adt39::C5)))))), std::rc::Rc::new(f46(v104)))
        }
        Adt3::C46(r225, r226, r227) => {
            let v105 = (*r225).clone();
            let v106 = (*r226).clone();
            let v107 = (*r227).clone();
            Adt39::C6(std::rc::Rc::new(Adt39::C4(std::rc::Rc::new(f122(Str::lit("key"), f46(v105), f122(Str::lit("value"), f46(v106), Adt39::C5))))), std::rc::Rc::new(f46(v107)))
        }
        Adt3::C47(r228, r229, r230) => {
            let v108 = (*r228).clone();
            let v109 = (*r229).clone();
            let v110 = (*r230).clone();
            Adt39::C6(std::rc::Rc::new(Adt39::C4(std::rc::Rc::new(f122(Str::lit("source"), f46(v108), f122(Str::lit("target"), f46(v109), Adt39::C5))))), std::rc::Rc::new(f46(v110)))
        }
    }
}

pub fn f47(v0: List<(Bytes, Adt2)>) -> Adt39 {
    Adt39::C3(std::rc::Rc::new(f4(f119(Fn15::F130, List::<Adt39>::nil(), f121(v0)))))
}

pub fn f48(v0: Str, v1: Option<Bytes>, v2: Adt39) -> Adt39 {
    let m231 = v1;
    match m231 {
        None => {
            v2
        }
        Some(v3) => {
            f38(v0, f39(v3), v2)
        }
    }
}

pub fn f49(v0: Str, v1: List<Bytes>, v2: Adt39) -> Adt39 {
    let m232 = v1.clone();
    match m232.uncons() {
        None => {
            v2
        }
        Some((_, _)) => {
            f38(v0, f40(v1), v2)
        }
    }
}

pub fn f50(v0: Option<(Adt3, List<(Option<Bytes>, Bytes)>)>, v1: Adt39) -> Adt39 {
    let m233 = v0;
    match m233 {
        None => {
            v1
        }
        Some(v2) => {
            f38(Str::lit("termination"), Adt39::C4(std::rc::Rc::new(f38(Str::lit("measure"), f46({
                let (h234, _) = v2;
                h234
            }), f38(Str::lit("evidence"), f131({
                let (_, h235) = v2;
                h235
            }), f43())))), v1)
        }
    }
}

pub fn f51(v0: Option<(List<Bytes>, List<Bytes>)>, v1: Adt39) -> Adt39 {
    let m236 = v0;
    match m236 {
        None => {
            v1
        }
        Some(v2) => {
            f38(Str::lit("production"), Adt39::C4(std::rc::Rc::new(f38(Str::lit("targets"), f40({
                let (h237, _) = v2;
                h237
            }), f38(Str::lit("effects"), f40({
                let (_, h238) = v2;
                h238
            }), f43())))), v1)
        }
    }
}

pub fn f52(v0: Adt6) -> Adt39 {
    let m239 = v0;
    match m239 {
        Adt6::C0 => {
            f125(Str::lit("reflexivity"), Adt39::C5)
        }
        Adt6::C1 => {
            f125(Str::lit("decide"), Adt39::C5)
        }
        Adt6::C2(v1) => {
            f125(Str::lit("linear_arithmetic"), {
                let m240 = v1.clone();
                match m240.uncons() {
                    None => {
                        Adt39::C5
                    }
                    Some((_, _)) => {
                        f122(Str::lit("definitions"), f131(v1), Adt39::C5)
                    }
                }
            })
        }
        Adt6::C3(v4) => {
            f125(Str::lit("simplify"), f122(Str::lit("definitions"), f131(v4), Adt39::C5))
        }
        Adt6::C4(r243) => {
            let v5 = (*r243).clone();
            f125(Str::lit("constructor"), f122(Str::lit("branches"), Adt39::C3(std::rc::Rc::new(f52(v5))), Adt39::C5))
        }
        Adt6::C5(v6, r244) => {
            let v7 = (*r244).clone();
            f125(Str::lit("cases"), f122(Str::lit("scrutinee"), Adt39::C2(v6), f122(Str::lit("branches"), Adt39::C3(std::rc::Rc::new(f52(v7))), Adt39::C5)))
        }
        Adt6::C6(v8, v9) => {
            f125(Str::lit("term_cases"), f122(Str::lit("scrutinee"), f46(v8), f122(Str::lit("definitions"), f131(v9), Adt39::C5)))
        }
        Adt6::C7(v10, v11, r245) => {
            let v12 = (*r245).clone();
            f125(Str::lit("induction"), f122(Str::lit("scrutinee"), Adt39::C2(v10), {
                let m241 = v11.clone();
                match m241.uncons() {
                    None => {
                        f122(Str::lit("branches"), Adt39::C3(std::rc::Rc::new(f52(v12))), Adt39::C5)
                    }
                    Some((_, _)) => {
                        f122(Str::lit("generalizing"), f40(v11), f122(Str::lit("branches"), Adt39::C3(std::rc::Rc::new(f52(v12))), Adt39::C5))
                    }
                }
            }))
        }
        Adt6::C8(v15, r246, r247) => {
            let v16 = (*r246).clone();
            let v17 = (*r247).clone();
            f125(Str::lit("nested_induction"), f122(Str::lit("scrutinee"), Adt39::C2(v15), f122(Str::lit("branches"), Adt39::C3(std::rc::Rc::new(f52(v16))), f122(Str::lit("members"), Adt39::C3(std::rc::Rc::new(f52(v17))), Adt39::C5))))
        }
        Adt6::C9 => {
            f125(Str::lit("congruence"), Adt39::C5)
        }
        Adt6::C10(v18) => {
            f125(Str::lit("boolean_reflection"), f122(Str::lit("reflection"), f132(v18), Adt39::C5))
        }
        Adt6::C11(v19, v20, v21, v22) => {
            f125(Str::lit("law"), f122(Str::lit("name"), Adt39::C2(v19), f128(Str::lit("type_arguments"), v20, f122(Str::lit("arguments"), Adt39::C3(std::rc::Rc::new(f46(v21))), {
                let m242 = v22.clone();
                match m242.uncons() {
                    None => {
                        Adt39::C5
                    }
                    Some((_, _)) => {
                        f122(Str::lit("premises"), f131(v22), Adt39::C5)
                    }
                }
            }))))
        }
        Adt6::C12(v25, v26, v27, v28) => {
            f125(Str::lit("apply"), f122(Str::lit("theorem"), f44(v25, v26), f128(Str::lit("type_arguments"), v27, f122(Str::lit("arguments"), Adt39::C3(std::rc::Rc::new(f46(v28))), Adt39::C5))))
        }
        Adt6::C13 => {
            Adt39::C5
        }
        Adt6::C14(r248, r249) => {
            let v29 = (*r248).clone();
            let v30 = (*r249).clone();
            Adt39::C6(std::rc::Rc::new(f52(v29)), std::rc::Rc::new(f52(v30)))
        }
        Adt6::C15(v31, v32, r250, r251) => {
            let v33 = (*r250).clone();
            let v34 = (*r251).clone();
            Adt39::C6(std::rc::Rc::new(Adt39::C4(std::rc::Rc::new(f122(Str::lit("constructor"), Adt39::C2(v31), f122(Str::lit("binders"), f40(v32), f122(Str::lit("proof"), f52(v33), Adt39::C5)))))), std::rc::Rc::new(f52(v34)))
        }
        Adt6::C16(v35, v36, v37, r252, r253) => {
            let v38 = (*r252).clone();
            let v39 = (*r253).clone();
            Adt39::C6(std::rc::Rc::new(Adt39::C4(std::rc::Rc::new(f122(Str::lit("type"), f45(v35), f122(Str::lit("binder"), Adt39::C2(v36), f122(Str::lit("statement"), f46(v37), f122(Str::lit("branches"), Adt39::C3(std::rc::Rc::new(f52(v38))), Adt39::C5))))))), std::rc::Rc::new(f52(v39)))
        }
    }
}

pub fn f53(v0: Str) -> Bytes {
    let a254 = v0;
    utf8_encode(a254)
}

pub fn f54(v0: Adt9) -> Adt39 {
    let m255 = v0;
    match m255 {
        Adt9::C0 => {
            Adt39::C2(f53(Str::lit("parameters")))
        }
        Adt9::C1 => {
            Adt39::C2(f53(Str::lit("vocabulary")))
        }
        Adt9::C2 => {
            Adt39::C2(f53(Str::lit("dataset")))
        }
        Adt9::C3 => {
            Adt39::C2(f53(Str::lit("table")))
        }
        Adt9::C4 => {
            Adt39::C2(f53(Str::lit("binary")))
        }
    }
}

pub fn f55(v0: Adt10) -> Adt39 {
    let m256 = v0;
    match m256 {
        Adt10::C0 => {
            let v1: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("kind")), Adt39::C2(f53(Str::lit("bytes")))), List::<(Bytes, Adt39)>::nil());
            f59(v1)
        }
        Adt10::C1(v2, v3) => {
            let v4: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("kind")), Adt39::C2(f53(Str::lit("int_tensor")))), List::<(Bytes, Adt39)>::nil());
            let v5: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("element")), f133(v2)), v4);
            let v6: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("shape")), Adt39::C3(std::rc::Rc::new(f4(f134(Fn16::F135, List::<Adt39>::nil(), f136(v3)))))), v5);
            f59(v6)
        }
        Adt10::C2 => {
            let v7: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("kind")), Adt39::C2(f53(Str::lit("utf8_lines")))), List::<(Bytes, Adt39)>::nil());
            f59(v7)
        }
    }
}

pub fn f56(v0: Fn8, p260: List<Adt39>, p261: List<Bytes>) -> List<Adt39> {
    let m259 = (p260, p261);
    {
        let mut m259 = m259;
        loop {
            let step = {
                let (v1, v2) = m259;
                let m257 = v2;
                match m257.uncons() {
                    None => {
                        Err::<(List<Adt39>, List<Bytes>), List<Adt39>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt39>, List<Bytes>), List<Adt39>>(({
                            let c258: Fn8 = v0.clone();
                            c258.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m259 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f57(v0: List<Adt39>, v1: Bytes) -> List<Adt39> {
    List::cons(Adt39::C2(v1), v0)
}

pub fn f58(v0: List<Bytes>) -> List<Bytes> {
    f137(Fn17::F138, List::<Bytes>::nil(), v0)
}

pub fn f59(v0: List<(Bytes, Adt39)>) -> Adt39 {
    Adt39::C4(std::rc::Rc::new(f9(Fn1::F139, Adt39::C5, v0)))
}

pub fn f60(v0: List<Adt39>, v1: Bytes) -> List<Adt39> {
    List::cons(Adt39::C2(v1), v0)
}

pub fn f61(v0: Adt12) -> Adt39 {
    let m262 = v0;
    match m262 {
        Adt12::C0(v1, v2) => {
            let v3: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("name")), Adt39::C2(v1)), List::<(Bytes, Adt39)>::nil());
            let v4: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("type")), f45(v2)), v3);
            f59(v4)
        }
    }
}

pub fn f62(v0: Adt13) -> Adt39 {
    let m263 = v0;
    match m263 {
        Adt13::C0(v1, v2, v3) => {
            let v4: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("name")), Adt39::C2(v1)), List::<(Bytes, Adt39)>::nil());
            let v5: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("next")), Adt39::C2(v2)), v4);
            let v6: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("type")), f45(v3)), v5);
            f59(v6)
        }
    }
}

pub fn f63(v0: Fn9, p267: List<Adt39>, p268: List<Adt14>) -> List<Adt39> {
    let m266 = (p267, p268);
    {
        let mut m266 = m266;
        loop {
            let step = {
                let (v1, v2) = m266;
                let m264 = v2;
                match m264.uncons() {
                    None => {
                        Err::<(List<Adt39>, List<Adt14>), List<Adt39>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt39>, List<Adt14>), List<Adt39>>(({
                            let c265: Fn9 = v0.clone();
                            c265.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m266 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f64(v0: List<Adt39>, v1: Adt14) -> List<Adt39> {
    List::cons(f140(v1), v0)
}

pub fn f65(v0: List<Adt14>) -> List<Adt14> {
    f141(Fn18::F142, List::<Adt14>::nil(), v0)
}

pub fn f66(v0: List<Adt39>, v1: Bytes) -> List<Adt39> {
    List::cons(Adt39::C2(v1), v0)
}

pub fn f67(v0: List<Adt39>, v1: Bytes) -> List<Adt39> {
    List::cons(Adt39::C2(v1), v0)
}

pub fn f68(v0: Adt16) -> Adt39 {
    let m269 = v0;
    match m269 {
        Adt16::C0(v1, v2, v3) => {
            let v4: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("name")), Adt39::C2(v1)), List::<(Bytes, Adt39)>::nil());
            let v5: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("type")), f45(v2)), v4);
            let v6: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("initial")), f46(v3)), v5);
            f59(v6)
        }
    }
}

pub fn f69(v0: Adt17) -> Adt39 {
    let m270 = v0;
    match m270 {
        Adt17::C0(v1) => {
            let v2: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("kind")), Adt39::C2(f53(Str::lit("deterministic")))), List::<(Bytes, Adt39)>::nil());
            let v3: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("body")), f46(v1)), v2);
            f59(v3)
        }
        Adt17::C1(v4, v5) => {
            let v6: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("kind")), Adt39::C2(f53(Str::lit("rule")))), List::<(Bytes, Adt39)>::nil());
            let v7: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("rules")), Adt39::C3(std::rc::Rc::new(f4(f143(Fn19::F144, List::<Adt39>::nil(), f145(v4)))))), v6);
            let v8: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("default")), f46(v5)), v7);
            f59(v8)
        }
        Adt17::C2(v9, v10, v11, v12, v13, v14, v15) => {
            let v16: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("kind")), Adt39::C2(f53(Str::lit("statistical")))), List::<(Bytes, Adt39)>::nil());
            let v17: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("scheme")), f146(v9)), v16);
            let v18: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("features")), f46(v10)), v17);
            let v19: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("width")), Adt39::C1(false, v11)), v18);
            let v20: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("width_evidence")), f44({
                let (h271, _) = v12;
                h271
            }, {
                let (_, h272) = v12;
                h272
            })), v19);
            let v21: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("weights")), f44({
                let (h273, _) = v13;
                h273
            }, {
                let (_, h274) = v13;
                h274
            })), v20);
            let v22: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("bias")), f44({
                let (h275, _) = v14;
                h275
            }, {
                let (_, h276) = v14;
                h276
            })), v21);
            let v23: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("labels")), Adt39::C3(std::rc::Rc::new(f4(f147(Fn20::F148, List::<Adt39>::nil(), f149(v15)))))), v22);
            f59(v23)
        }
        Adt17::C3(v24, v25, v26, v27, v28, v29) => {
            let v30: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("kind")), Adt39::C2(f53(Str::lit("neural")))), List::<(Bytes, Adt39)>::nil());
            let v31: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("architecture")), f150(v24)), v30);
            let v32: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("encoder")), f46(v25)), v31);
            let v33: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("width")), Adt39::C1(false, v26)), v32);
            let v34: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("width_evidence")), f44({
                let (h277, _) = v27;
                h277
            }, {
                let (_, h278) = v27;
                h278
            })), v33);
            let v35: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("layers")), Adt39::C3(std::rc::Rc::new(f4(f151(Fn21::F152, List::<Adt39>::nil(), f153(v28)))))), v34);
            let v36: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("decoder")), f154(v29)), v35);
            f59(v36)
        }
        Adt17::C4(v37) => {
            let v38: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("kind")), Adt39::C2(f53(Str::lit("composite")))), List::<(Bytes, Adt39)>::nil());
            let v39: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("form")), f155(v37)), v38);
            f59(v39)
        }
    }
}

pub fn f70(v0: List<Adt39>, v1: Bytes) -> List<Adt39> {
    List::cons(Adt39::C2(v1), v0)
}

pub fn f71(v0: List<Adt39>, v1: Bytes) -> List<Adt39> {
    List::cons(Adt39::C2(v1), v0)
}

pub fn f72(v0: Adt24) -> Adt39 {
    let m279 = v0;
    match m279 {
        Adt24::C0(v1, v2) => {
            let v3: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("member")), f44({
                let (h280, _) = v1;
                h280
            }, {
                let (_, h281) = v1;
                h281
            })), List::<(Bytes, Adt39)>::nil());
            let v6: List<(Bytes, Adt39)> = {
                let m282 = v2.clone();
                match m282.uncons() {
                    None => {
                        v3
                    }
                    Some((_, _)) => {
                        List::cons((f53(Str::lit("type_arguments")), Adt39::C3(std::rc::Rc::new(f4(f156(Fn22::F157, List::<Adt39>::nil(), f158(v2)))))), v3)
                    }
                }
            };
            f59(v6)
        }
    }
}

pub fn f73(v0: Fn10, p286: List<Adt39>, p287: List<Adt26>) -> List<Adt39> {
    let m285 = (p286, p287);
    {
        let mut m285 = m285;
        loop {
            let step = {
                let (v1, v2) = m285;
                let m283 = v2;
                match m283.uncons() {
                    None => {
                        Err::<(List<Adt39>, List<Adt26>), List<Adt39>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt39>, List<Adt26>), List<Adt39>>(({
                            let c284: Fn10 = v0.clone();
                            c284.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m285 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f74(v0: List<Adt39>, v1: Adt26) -> List<Adt39> {
    List::cons(f159(v1), v0)
}

pub fn f75(v0: List<Adt26>) -> List<Adt26> {
    f160(Fn23::F161, List::<Adt26>::nil(), v0)
}

pub fn f76(v0: List<Adt39>, v1: Bytes) -> List<Adt39> {
    List::cons(Adt39::C2(v1), v0)
}

pub fn f77(v0: List<Adt39>, v1: Bytes) -> List<Adt39> {
    List::cons(Adt39::C2(v1), v0)
}

pub fn f78(v0: Fn11, p291: List<Adt39>, p292: List<Adt24>) -> List<Adt39> {
    let m290 = (p291, p292);
    {
        let mut m290 = m290;
        loop {
            let step = {
                let (v1, v2) = m290;
                let m288 = v2;
                match m288.uncons() {
                    None => {
                        Err::<(List<Adt39>, List<Adt24>), List<Adt39>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt39>, List<Adt24>), List<Adt39>>(({
                            let c289: Fn11 = v0.clone();
                            c289.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m290 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f79(v0: List<Adt39>, v1: Adt24) -> List<Adt39> {
    List::cons(f72(v1), v0)
}

pub fn f80(v0: List<Adt24>) -> List<Adt24> {
    f162(Fn24::F163, List::<Adt24>::nil(), v0)
}

pub fn f81(v0: Fn12, p296: List<Adt39>, p297: List<(Option<Bytes>, Bytes)>) -> List<Adt39> {
    let m295 = (p296, p297);
    {
        let mut m295 = m295;
        loop {
            let step = {
                let (v1, v2) = m295;
                let m293 = v2;
                match m293.uncons() {
                    None => {
                        Err::<(List<Adt39>, List<(Option<Bytes>, Bytes)>), List<Adt39>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt39>, List<(Option<Bytes>, Bytes)>), List<Adt39>>(({
                            let c294: Fn12 = v0.clone();
                            c294.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m295 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f82(v0: List<Adt39>, v1: (Option<Bytes>, Bytes)) -> List<Adt39> {
    List::cons(f44({
        let (h298, _) = v1;
        h298
    }, {
        let (_, h299) = v1;
        h299
    }), v0)
}

pub fn f83(v0: List<(Option<Bytes>, Bytes)>) -> List<(Option<Bytes>, Bytes)> {
    f164(Fn25::F165, List::<(Option<Bytes>, Bytes)>::nil(), v0)
}

pub fn f84(v0: List<Adt39>, v1: Bytes) -> List<Adt39> {
    List::cons(Adt39::C2(v1), v0)
}

pub fn f85(v0: List<Adt39>, v1: Bytes) -> List<Adt39> {
    List::cons(Adt39::C2(v1), v0)
}

pub fn f86(v0: Adt28) -> Adt39 {
    let m300 = v0;
    match m300 {
        Adt28::C0(v1, v2) => {
            let v3: List<(Bytes, Adt39)> = List::cons((f3(Str::lit("predicate")), f44({
                let (h301, _) = v1;
                h301
            }, {
                let (_, h302) = v1;
                h302
            })), List::<(Bytes, Adt39)>::nil());
            let v4: List<(Bytes, Adt39)> = List::cons((f3(Str::lit("preserves")), f44({
                let (h303, _) = v2;
                h303
            }, {
                let (_, h304) = v2;
                h304
            })), v3);
            f2(v4)
        }
    }
}

pub fn f87(v0: List<Adt39>, v1: Bytes) -> List<Adt39> {
    List::cons(Adt39::C2(v1), v0)
}

pub fn f88(v0: List<Adt39>, v1: Bytes) -> List<Adt39> {
    List::cons(Adt39::C2(v1), v0)
}

pub fn f89(v0: Adt29) -> Adt39 {
    let m305 = v0;
    match m305 {
        Adt29::C0(v1, v2, v3) => {
            let v4: List<(Bytes, Adt39)> = List::cons((f3(Str::lit("name")), Adt39::C2(v1)), List::<(Bytes, Adt39)>::nil());
            let v5: List<(Bytes, Adt39)> = List::cons((f3(Str::lit("type")), f45(v2)), v4);
            let v6: List<(Bytes, Adt39)> = List::cons((f3(Str::lit("candidates")), f46(v3)), v5);
            f2(v6)
        }
    }
}

pub fn f90(v0: List<Adt39>, v1: Bytes) -> List<Adt39> {
    List::cons(Adt39::C2(v1), v0)
}

pub fn f91(v0: List<Adt39>, v1: Bytes) -> List<Adt39> {
    List::cons(Adt39::C2(v1), v0)
}

pub fn f92(v0: List<Adt39>, v1: Bytes) -> List<Adt39> {
    List::cons(Adt39::C2(v1), v0)
}

pub fn f93(v0: List<Adt39>, v1: Bytes) -> List<Adt39> {
    List::cons(Adt39::C2(v1), v0)
}

pub fn f94(v0: List<Adt39>, v1: Adt24) -> List<Adt39> {
    List::cons(f72(v1), v0)
}

pub fn f95(v0: Adt30) -> Adt39 {
    let m306 = v0;
    match m306 {
        Adt30::C0(v1) => {
            let v2: List<(Bytes, Adt39)> = List::cons((f3(Str::lit("kind")), Adt39::C2(f3(Str::lit("forward")))), List::<(Bytes, Adt39)>::nil());
            let v4: List<(Bytes, Adt39)> = {
                let m307 = v1;
                match m307 {
                    None => {
                        v2
                    }
                    Some(v3) => {
                        List::cons((f3(Str::lit("fuel")), f46(v3)), v2)
                    }
                }
            };
            f2(v4)
        }
        Adt30::C1(v5, v6, v7, v8) => {
            let v9: List<(Bytes, Adt39)> = List::cons((f3(Str::lit("kind")), Adt39::C2(f3(Str::lit("search")))), List::<(Bytes, Adt39)>::nil());
            let v10: List<(Bytes, Adt39)> = List::cons((f3(Str::lit("order")), f166(v5)), v9);
            let v12: List<(Bytes, Adt39)> = {
                let m308 = v6;
                match m308 {
                    None => {
                        v10
                    }
                    Some(v11) => {
                        List::cons((f3(Str::lit("fuel")), f46(v11)), v10)
                    }
                }
            };
            let v14: List<(Bytes, Adt39)> = {
                let m309 = v7;
                match m309 {
                    None => {
                        v12
                    }
                    Some(v13) => {
                        List::cons((f3(Str::lit("frontier")), f46(v13)), v12)
                    }
                }
            };
            let v15: List<(Bytes, Adt39)> = if v8 {
                List::cons((f3(Str::lit("deduplicate")), Adt39::C0(true)), v14)
            } else {
                v14
            };
            f2(v15)
        }
        Adt30::C2(v16, v17) => {
            let v18: List<(Bytes, Adt39)> = List::cons((f3(Str::lit("kind")), Adt39::C2(f3(Str::lit("generate_and_verify")))), List::<(Bytes, Adt39)>::nil());
            let v19: List<(Bytes, Adt39)> = List::cons((f3(Str::lit("generator")), f46(v16)), v18);
            let v21: List<(Bytes, Adt39)> = {
                let m310 = v17;
                match m310 {
                    None => {
                        v19
                    }
                    Some(v20) => {
                        List::cons((f3(Str::lit("budget")), f46(v20)), v19)
                    }
                }
            };
            f2(v21)
        }
    }
}

pub fn f96(v0: Adt32) -> Adt39 {
    let m311 = v0;
    match m311 {
        Adt32::C0(v1, v2, v3) => {
            let v4: List<(Bytes, Adt39)> = List::cons((f3(Str::lit("name")), Adt39::C2(v1)), List::<(Bytes, Adt39)>::nil());
            let v5: List<(Bytes, Adt39)> = List::cons((f3(Str::lit("type")), f45(v2)), v4);
            let v6: List<(Bytes, Adt39)> = List::cons((f3(Str::lit("value")), f46(v3)), v5);
            f2(v6)
        }
    }
}

pub fn f97(v0: Fn13, p315: List<Adt39>, p316: List<Adt33>) -> List<Adt39> {
    let m314 = (p315, p316);
    {
        let mut m314 = m314;
        loop {
            let step = {
                let (v1, v2) = m314;
                let m312 = v2;
                match m312.uncons() {
                    None => {
                        Err::<(List<Adt39>, List<Adt33>), List<Adt39>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt39>, List<Adt33>), List<Adt39>>(({
                            let c313: Fn13 = v0.clone();
                            c313.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m314 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f98(v0: List<Adt39>, v1: Adt33) -> List<Adt39> {
    List::cons(f167(v1), v0)
}

pub fn f99(v0: List<Adt33>) -> List<Adt33> {
    f168(Fn26::F169, List::<Adt33>::nil(), v0)
}

pub fn f100(v0: List<Adt39>, v1: Bytes) -> List<Adt39> {
    List::cons(Adt39::C2(v1), v0)
}

pub fn f101(v0: List<Adt39>, v1: Bytes) -> List<Adt39> {
    List::cons(Adt39::C2(v1), v0)
}

pub fn f102(v0: Adt34) -> Adt39 {
    let m317 = v0;
    match m317 {
        Adt34::C0 => {
            Adt39::C2(f3(Str::lit("byte_level")))
        }
        Adt34::C1 => {
            Adt39::C2(f3(Str::lit("longest_match")))
        }
    }
}

pub fn f103(v0: List<Adt39>, v1: Bytes) -> List<Adt39> {
    List::cons(Adt39::C2(v1), v0)
}

pub fn f104(v0: Adt35) -> Adt39 {
    let m318 = v0;
    match m318 {
        Adt35::C0 => {
            Adt39::C2(f3(Str::lit("refuse")))
        }
        Adt35::C1 => {
            Adt39::C2(f3(Str::lit("keep_last")))
        }
    }
}

pub fn f105(v0: List<Adt39>, v1: Bytes) -> List<Adt39> {
    List::cons(Adt39::C2(v1), v0)
}

pub fn f106(v0: Adt36) -> Adt39 {
    let m319 = v0;
    match m319 {
        Adt36::C0 => {
            let v1: List<(Bytes, Adt39)> = List::cons((f3(Str::lit("kind")), Adt39::C2(f3(Str::lit("greedy")))), List::<(Bytes, Adt39)>::nil());
            f2(v1)
        }
        Adt36::C1(v2, v3) => {
            let v4: List<(Bytes, Adt39)> = List::cons((f3(Str::lit("kind")), Adt39::C2(f3(Str::lit("sample")))), List::<(Bytes, Adt39)>::nil());
            let v5: List<(Bytes, Adt39)> = List::cons((f3(Str::lit("generator")), f170(v2)), v4);
            let v6: List<(Bytes, Adt39)> = List::cons((f3(Str::lit("distribution")), f171(v3)), v5);
            f2(v6)
        }
    }
}

pub fn f107(v0: List<Adt39>, v1: Bytes) -> List<Adt39> {
    List::cons(Adt39::C2(v1), v0)
}

pub fn f108(v0: List<Adt39>, v1: Bytes) -> List<Adt39> {
    List::cons(Adt39::C2(v1), v0)
}

pub fn f109(v0: Bytes, v1: u64, v2: u64) -> R<Bytes> {
    let a320 = v2;
    let a321 = 0u64;
    if nat_eq(a320, a321) {
        Ok(f172(v0, 0u64))
    } else {
        let (_, h322) = {
            let (h323, _) = f29(Fn5::F173(v1, v0), v2, (v2, Bytes::lit(&[])))?;
            h323
        };
        Ok(h322)
    }
}

pub fn f110() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f111(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let m327 = {
        let a325 = v0.clone();
        let a326 = {
            let (h324, _) = v1.clone();
            h324
        };
        index_bytes(a325, a326)
    };
    match m327 {
        None => {
            Ok(None::<(u64, Bytes)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a329 = {
                    let (h328, _) = v1.clone();
                    h328
                };
                let a330 = 1u64;
                nat_add(a329, a330)?
            }, {
                let a333 = {
                    let (_, h331) = v1;
                    h331
                };
                let a334 = f174(v0, {
                    let (h332, _) = v1;
                    h332
                }, v2)?;
                append_bytes(a333, a334)
            })))
        }
    }
}

pub fn f112(v0: Fn14, p338: Option<Bytes>, p339: List<Bytes>) -> Option<Bytes> {
    let m337 = (p338, p339);
    {
        let mut m337 = m337;
        loop {
            let step = {
                let (v1, v2) = m337;
                let m335 = v2;
                match m335.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(({
                            let c336: Fn14 = v0.clone();
                            c336.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m337 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f113(v0: Bytes, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m340 = v1;
    match m340 {
        None => {
            Some(v2)
        }
        Some(v3) => {
            Some({
                let a343 = {
                    let a341 = v3;
                    let a342 = v0;
                    append_bytes(a341, a342)
                };
                let a344 = v2;
                append_bytes(a343, a344)
            })
        }
    }
}

pub fn f114(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    let m345 = v0.clone();
    match m345.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, v3)) => {
            let a348 = {
                let (h346, _) = v2;
                h346
            };
            let a349 = {
                let (h347, _) = v1.clone();
                h347
            };
            if equal(a348, a349) {
                List::cons(v1, v3)
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f115(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v1: List<List<(Bytes, Bytes)>> = f175(f176(Fn27::F177, List::<List<(Bytes, Bytes)>>::nil(), v0.clone()));
    let m354 = {
        let (h350, _) = f178(Fn28::F179, {
            let a352 = {
                let a351 = v0;
                length_list(a351)
            };
            let a353 = 1u64;
            nat_add(a352, a353)?
        }, v1)?;
        h350
    };
    match m354.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f116(v0: (Bytes, Bytes)) -> R<Bytes> {
    let a359 = {
        let a356 = f18({
            let (h355, _) = v0;
            h355
        })?;
        let a357 = Bytes::lit(&[0x3a]);
        append_bytes(a356, a357)
    };
    let a360 = {
        let (_, h358) = v0;
        h358
    };
    Ok(append_bytes(a359, a360))
}

pub fn f117(v0: Str) -> Bytes {
    let a361 = v0;
    utf8_encode(a361)
}

pub fn f118(v0: List<Adt39>, v1: Bytes) -> List<Adt39> {
    List::cons(Adt39::C2(v1), v0)
}

pub fn f119(v0: Fn15, p365: List<Adt39>, p366: List<(Bytes, Adt2)>) -> List<Adt39> {
    let m364 = (p365, p366);
    {
        let mut m364 = m364;
        loop {
            let step = {
                let (v1, v2) = m364;
                let m362 = v2;
                match m362.uncons() {
                    None => {
                        Err::<(List<Adt39>, List<(Bytes, Adt2)>), List<Adt39>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt39>, List<(Bytes, Adt2)>), List<Adt39>>(({
                            let c363: Fn15 = v0.clone();
                            c363.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m364 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f120(v0: List<Adt39>, v1: (Bytes, Adt2)) -> List<Adt39> {
    List::cons(f129({
        let (h367, _) = v1;
        h367
    }, {
        let (_, h368) = v1;
        h368
    }), v0)
}

pub fn f121(v0: List<(Bytes, Adt2)>) -> List<(Bytes, Adt2)> {
    f180(Fn29::F181, List::<(Bytes, Adt2)>::nil(), v0)
}

pub fn f122(v0: Str, v1: Adt39, v2: Adt39) -> Adt39 {
    Adt39::C7(f182(v0), std::rc::Rc::new(v1), std::rc::Rc::new(v2))
}

pub fn f123(v0: Str, v1: Adt39) -> Adt39 {
    Adt39::C4(std::rc::Rc::new(f124(Str::lit("kind"), Adt39::C2(f183(v0)), v1)))
}

pub fn f124(v0: Str, v1: Adt39, v2: Adt39) -> Adt39 {
    Adt39::C7(f183(v0), std::rc::Rc::new(v1), std::rc::Rc::new(v2))
}

pub fn f125(v0: Str, v1: Adt39) -> Adt39 {
    Adt39::C4(std::rc::Rc::new(f122(Str::lit("kind"), Adt39::C2(f182(v0)), v1)))
}

pub fn f126(v0: Adt4) -> Bytes {
    let m369 = v0;
    match m369 {
        Adt4::C0 => {
            f183(Str::lit("int"))
        }
        Adt4::C1 => {
            f183(Str::lit("int8"))
        }
        Adt4::C2 => {
            f183(Str::lit("int16"))
        }
        Adt4::C3 => {
            f183(Str::lit("int32"))
        }
        Adt4::C4 => {
            f183(Str::lit("int64"))
        }
        Adt4::C5 => {
            f183(Str::lit("uint8"))
        }
        Adt4::C6 => {
            f183(Str::lit("uint16"))
        }
        Adt4::C7 => {
            f183(Str::lit("uint32"))
        }
        Adt4::C8 => {
            f183(Str::lit("uint64"))
        }
    }
}

pub fn f127(v0: Adt5) -> Adt40 {
    let m370 = v0;
    match m370 {
        Adt5::C0 => {
            Adt40::C0(f183(Str::lit("subtract")), false, false, None::<u64>)
        }
        Adt5::C1 => {
            Adt40::C0(f183(Str::lit("multiply")), false, false, None::<u64>)
        }
        Adt5::C2 => {
            Adt40::C0(f183(Str::lit("quotient")), false, false, None::<u64>)
        }
        Adt5::C3 => {
            Adt40::C0(f183(Str::lit("remainder")), false, false, None::<u64>)
        }
        Adt5::C4 => {
            Adt40::C0(f183(Str::lit("negate")), false, false, None::<u64>)
        }
        Adt5::C5 => {
            Adt40::C0(f183(Str::lit("checked_convert")), false, false, None::<u64>)
        }
        Adt5::C6 => {
            Adt40::C0(f183(Str::lit("checked_add")), false, false, None::<u64>)
        }
        Adt5::C7 => {
            Adt40::C0(f183(Str::lit("checked_subtract")), false, false, None::<u64>)
        }
        Adt5::C8 => {
            Adt40::C0(f183(Str::lit("checked_multiply")), false, false, None::<u64>)
        }
        Adt5::C9 => {
            Adt40::C0(f183(Str::lit("checked_negate")), false, false, None::<u64>)
        }
        Adt5::C10 => {
            Adt40::C0(f183(Str::lit("checked_quotient")), false, false, None::<u64>)
        }
        Adt5::C11 => {
            Adt40::C0(f183(Str::lit("bit_and")), false, false, None::<u64>)
        }
        Adt5::C12 => {
            Adt40::C0(f183(Str::lit("bit_or")), false, false, None::<u64>)
        }
        Adt5::C13 => {
            Adt40::C0(f183(Str::lit("bit_xor")), false, false, None::<u64>)
        }
        Adt5::C14 => {
            Adt40::C0(f183(Str::lit("bit_not")), false, false, None::<u64>)
        }
        Adt5::C15 => {
            Adt40::C0(f183(Str::lit("shift_left")), false, false, None::<u64>)
        }
        Adt5::C16 => {
            Adt40::C0(f183(Str::lit("shift_right")), false, false, None::<u64>)
        }
        Adt5::C17 => {
            Adt40::C0(f183(Str::lit("append")), false, false, None::<u64>)
        }
        Adt5::C18 => {
            Adt40::C0(f183(Str::lit("length")), false, false, None::<u64>)
        }
        Adt5::C19 => {
            Adt40::C0(f183(Str::lit("index")), false, false, None::<u64>)
        }
        Adt5::C20 => {
            Adt40::C0(f183(Str::lit("slice")), false, false, None::<u64>)
        }
        Adt5::C21 => {
            Adt40::C0(f183(Str::lit("utf8_encode")), false, false, None::<u64>)
        }
        Adt5::C22 => {
            Adt40::C0(f183(Str::lit("utf8_decode")), false, false, None::<u64>)
        }
        Adt5::C23 => {
            Adt40::C0(f183(Str::lit("compare_bytes")), false, false, None::<u64>)
        }
        Adt5::C24 => {
            Adt40::C0(f183(Str::lit("equal")), false, false, None::<u64>)
        }
        Adt5::C25 => {
            Adt40::C0(f183(Str::lit("split_exact")), false, false, None::<u64>)
        }
        Adt5::C26 => {
            Adt40::C0(f183(Str::lit("join")), false, false, None::<u64>)
        }
        Adt5::C27 => {
            Adt40::C0(f183(Str::lit("parse_decimal")), false, false, None::<u64>)
        }
        Adt5::C28 => {
            Adt40::C0(f183(Str::lit("format_decimal")), false, false, None::<u64>)
        }
        Adt5::C29 => {
            Adt40::C0(f183(Str::lit("map_insert")), true, false, None::<u64>)
        }
        Adt5::C30 => {
            Adt40::C0(f183(Str::lit("map_remove")), true, false, None::<u64>)
        }
        Adt5::C31 => {
            Adt40::C0(f183(Str::lit("map_lookup")), true, false, None::<u64>)
        }
        Adt5::C32 => {
            Adt40::C0(f183(Str::lit("map_contains")), true, false, None::<u64>)
        }
        Adt5::C33 => {
            Adt40::C0(f183(Str::lit("map_size")), true, false, None::<u64>)
        }
        Adt5::C34 => {
            Adt40::C0(f183(Str::lit("map_keys")), true, false, None::<u64>)
        }
        Adt5::C35 => {
            Adt40::C0(f183(Str::lit("map_values")), true, false, None::<u64>)
        }
        Adt5::C36 => {
            Adt40::C0(f183(Str::lit("map_entries")), true, false, None::<u64>)
        }
        Adt5::C37 => {
            Adt40::C0(f183(Str::lit("map_fold")), true, false, Some(0u64))
        }
        Adt5::C38 => {
            Adt40::C0(f183(Str::lit("set_insert")), true, false, None::<u64>)
        }
        Adt5::C39 => {
            Adt40::C0(f183(Str::lit("set_remove")), true, false, None::<u64>)
        }
        Adt5::C40 => {
            Adt40::C0(f183(Str::lit("set_contains")), true, false, None::<u64>)
        }
        Adt5::C41 => {
            Adt40::C0(f183(Str::lit("set_size")), true, false, None::<u64>)
        }
        Adt5::C42 => {
            Adt40::C0(f183(Str::lit("set_elements")), true, false, None::<u64>)
        }
        Adt5::C43 => {
            Adt40::C0(f183(Str::lit("set_union")), true, false, None::<u64>)
        }
        Adt5::C44 => {
            Adt40::C0(f183(Str::lit("set_intersection")), true, false, None::<u64>)
        }
        Adt5::C45 => {
            Adt40::C0(f183(Str::lit("set_difference")), true, false, None::<u64>)
        }
        Adt5::C46 => {
            Adt40::C0(f183(Str::lit("set_fold")), true, false, Some(0u64))
        }
        Adt5::C47 => {
            Adt40::C0(f183(Str::lit("list_fold")), true, false, Some(0u64))
        }
        Adt5::C48 => {
            Adt40::C0(f183(Str::lit("iterate")), true, false, Some(0u64))
        }
        Adt5::C49 => {
            Adt40::C0(f183(Str::lit("iterate_until")), true, false, Some(0u64))
        }
        Adt5::C50 => {
            Adt40::C0(f183(Str::lit("graph_successors")), true, false, None::<u64>)
        }
        Adt5::C51 => {
            Adt40::C0(f183(Str::lit("graph_reachable")), true, false, None::<u64>)
        }
        Adt5::C52 => {
            Adt40::C0(f183(Str::lit("graph_topological")), true, false, None::<u64>)
        }
        Adt5::C53 => {
            Adt40::C0(f183(Str::lit("less_than")), true, false, None::<u64>)
        }
        Adt5::C54 => {
            Adt40::C0(f183(Str::lit("to_int")), true, true, None::<u64>)
        }
        Adt5::C55 => {
            Adt40::C0(f183(Str::lit("to_nat")), true, true, None::<u64>)
        }
    }
}

pub fn f128(v0: Str, v1: Adt2, v2: Adt39) -> Adt39 {
    let m371 = v1.clone();
    match m371 {
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
            f122(v0, Adt39::C3(std::rc::Rc::new(f45(v1))), v2)
        }
    }
}

pub fn f129(v0: Bytes, v1: Adt2) -> Adt39 {
    Adt39::C4(std::rc::Rc::new(f122(Str::lit("name"), Adt39::C2(v0), f122(Str::lit("type"), f45(v1), Adt39::C5))))
}

pub fn f130(v0: List<Adt39>, v1: (Bytes, Adt2)) -> List<Adt39> {
    List::cons(Adt39::C4(std::rc::Rc::new(f38(Str::lit("name"), f39({
        let (h372, _) = v1;
        h372
    }), f38(Str::lit("fields"), Adt39::C3(std::rc::Rc::new(f45({
        let (_, h373) = v1;
        h373
    }))), f43())))), v0)
}

pub fn f131(v0: List<(Option<Bytes>, Bytes)>) -> Adt39 {
    Adt39::C3(std::rc::Rc::new(f4(f81(Fn12::F184, List::<Adt39>::nil(), f83(v0)))))
}

pub fn f132(v0: Adt7) -> Adt39 {
    let m374 = v0;
    match m374 {
        Adt7::C0(v1, v2, v3, v4, v5, v6, v7) => {
            f125(Str::lit("list"), f122(Str::lit("parameter"), Adt39::C2(v1), f122(Str::lit("values"), Adt39::C2(v2), f122(Str::lit("boolean_definition"), f44(v3, v4), f122(Str::lit("proposition_definition"), f44(v5, v6), f122(Str::lit("comparison"), Adt39::C2(v7), Adt39::C5))))))
        }
        Adt7::C1(v8, v9, v10, v11, v12, v13) => {
            f125(Str::lit("record"), f122(Str::lit("record"), Adt39::C2(v8), f122(Str::lit("boolean_definition"), f44(v9, v10), f122(Str::lit("proposition_definition"), f44(v11, v12), f122(Str::lit("fields"), Adt39::C3(std::rc::Rc::new(f4(f185(Fn30::F186, List::<Adt39>::nil(), f187(v13))))), Adt39::C5)))))
        }
    }
}

pub fn f133(v0: Adt11) -> Adt39 {
    let m375 = v0;
    match m375 {
        Adt11::C0 => {
            Adt39::C2(f53(Str::lit("int8")))
        }
        Adt11::C1 => {
            Adt39::C2(f53(Str::lit("int16")))
        }
        Adt11::C2 => {
            Adt39::C2(f53(Str::lit("int32")))
        }
        Adt11::C3 => {
            Adt39::C2(f53(Str::lit("int64")))
        }
        Adt11::C4 => {
            Adt39::C2(f53(Str::lit("uint8")))
        }
        Adt11::C5 => {
            Adt39::C2(f53(Str::lit("uint16")))
        }
        Adt11::C6 => {
            Adt39::C2(f53(Str::lit("uint32")))
        }
        Adt11::C7 => {
            Adt39::C2(f53(Str::lit("uint64")))
        }
    }
}

pub fn f134(v0: Fn16, p379: List<Adt39>, p380: List<u64>) -> List<Adt39> {
    let m378 = (p379, p380);
    {
        let mut m378 = m378;
        loop {
            let step = {
                let (v1, v2) = m378;
                let m376 = v2;
                match m376.uncons() {
                    None => {
                        Err::<(List<Adt39>, List<u64>), List<Adt39>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt39>, List<u64>), List<Adt39>>(({
                            let c377: Fn16 = v0.clone();
                            c377.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m378 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f135(v0: List<Adt39>, v1: u64) -> List<Adt39> {
    List::cons(Adt39::C1(false, v1), v0)
}

pub fn f136(v0: List<u64>) -> List<u64> {
    f188(Fn31::F189, List::<u64>::nil(), v0)
}

pub fn f137(v0: Fn17, p384: List<Bytes>, p385: List<Bytes>) -> List<Bytes> {
    let m383 = (p384, p385);
    {
        let mut m383 = m383;
        loop {
            let step = {
                let (v1, v2) = m383;
                let m381 = v2;
                match m381.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c382: Fn17 = v0.clone();
                            c382.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m383 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f138(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f139(v0: Adt39, v1: (Bytes, Adt39)) -> Adt39 {
    Adt39::C7({
        let (h386, _) = v1;
        h386
    }, std::rc::Rc::new({
        let (_, h387) = v1;
        h387
    }), std::rc::Rc::new(v0))
}

pub fn f140(v0: Adt14) -> Adt39 {
    let m388 = v0;
    match m388 {
        Adt14::C0(v1, v2, v3, v4) => {
            let v5: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("predicate")), f190(v1)), List::<(Bytes, Adt39)>::nil());
            let v6: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("validator")), f44({
                let (h389, _) = v2;
                h389
            }, {
                let (_, h390) = v2;
                h390
            })), v5);
            let v7: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("sound")), f44({
                let (h391, _) = v3;
                h391
            }, {
                let (_, h392) = v3;
                h392
            })), v6);
            let v9: List<(Bytes, Adt39)> = {
                let m393 = v4;
                match m393 {
                    None => {
                        v7
                    }
                    Some(v8) => {
                        List::cons((f53(Str::lit("complete")), f44({
                            let (h394, _) = v8;
                            h394
                        }, {
                            let (_, h395) = v8;
                            h395
                        })), v7)
                    }
                }
            };
            f59(v9)
        }
    }
}

pub fn f141(v0: Fn18, p399: List<Adt14>, p400: List<Adt14>) -> List<Adt14> {
    let m398 = (p399, p400);
    {
        let mut m398 = m398;
        loop {
            let step = {
                let (v1, v2) = m398;
                let m396 = v2;
                match m396.uncons() {
                    None => {
                        Err::<(List<Adt14>, List<Adt14>), List<Adt14>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt14>, List<Adt14>), List<Adt14>>(({
                            let c397: Fn18 = v0.clone();
                            c397.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m398 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f142(v0: List<Adt14>, v1: Adt14) -> List<Adt14> {
    List::cons(v1, v0)
}

pub fn f143(v0: Fn19, p404: List<Adt39>, p405: List<Adt18>) -> List<Adt39> {
    let m403 = (p404, p405);
    {
        let mut m403 = m403;
        loop {
            let step = {
                let (v1, v2) = m403;
                let m401 = v2;
                match m401.uncons() {
                    None => {
                        Err::<(List<Adt39>, List<Adt18>), List<Adt39>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt39>, List<Adt18>), List<Adt39>>(({
                            let c402: Fn19 = v0.clone();
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

pub fn f144(v0: List<Adt39>, v1: Adt18) -> List<Adt39> {
    List::cons(f191(v1), v0)
}

pub fn f145(v0: List<Adt18>) -> List<Adt18> {
    f192(Fn32::F193, List::<Adt18>::nil(), v0)
}

pub fn f146(v0: Adt19) -> Adt39 {
    let m406 = v0;
    match m406 {
        Adt19::C0 => {
            Adt39::C2(f53(Str::lit("linear_scoring")))
        }
    }
}

pub fn f147(v0: Fn20, p410: List<Adt39>, p411: List<Adt3>) -> List<Adt39> {
    let m409 = (p410, p411);
    {
        let mut m409 = m409;
        loop {
            let step = {
                let (v1, v2) = m409;
                let m407 = v2;
                match m407.uncons() {
                    None => {
                        Err::<(List<Adt39>, List<Adt3>), List<Adt39>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt39>, List<Adt3>), List<Adt39>>(({
                            let c408: Fn20 = v0.clone();
                            c408.apply(v1, v3)
                        }, v4))
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

pub fn f148(v0: List<Adt39>, v1: Adt3) -> List<Adt39> {
    List::cons(f46(v1), v0)
}

pub fn f149(v0: List<Adt3>) -> List<Adt3> {
    f194(Fn33::F195, List::<Adt3>::nil(), v0)
}

pub fn f150(v0: Adt20) -> Adt39 {
    let m412 = v0;
    match m412 {
        Adt20::C0 => {
            Adt39::C2(f53(Str::lit("integer_feedforward")))
        }
    }
}

pub fn f151(v0: Fn21, p416: List<Adt39>, p417: List<Adt21>) -> List<Adt39> {
    let m415 = (p416, p417);
    {
        let mut m415 = m415;
        loop {
            let step = {
                let (v1, v2) = m415;
                let m413 = v2;
                match m413.uncons() {
                    None => {
                        Err::<(List<Adt39>, List<Adt21>), List<Adt39>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt39>, List<Adt21>), List<Adt39>>(({
                            let c414: Fn21 = v0.clone();
                            c414.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m415 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f152(v0: List<Adt39>, v1: Adt21) -> List<Adt39> {
    List::cons(f196(v1), v0)
}

pub fn f153(v0: List<Adt21>) -> List<Adt21> {
    f197(Fn34::F198, List::<Adt21>::nil(), v0)
}

pub fn f154(v0: Adt22) -> Adt39 {
    let m418 = v0;
    match m418 {
        Adt22::C0(v1) => {
            let v2: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("kind")), Adt39::C2(f53(Str::lit("argmax")))), List::<(Bytes, Adt39)>::nil());
            let v3: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("labels")), Adt39::C3(std::rc::Rc::new(f4(f147(Fn20::F199, List::<Adt39>::nil(), f149(v1)))))), v2);
            f59(v3)
        }
        Adt22::C1(v4, v5) => {
            let v6: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("kind")), Adt39::C2(f53(Str::lit("function")))), List::<(Bytes, Adt39)>::nil());
            let v7: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("binder")), Adt39::C2(v4)), v6);
            let v8: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("body")), f46(v5)), v7);
            f59(v8)
        }
    }
}

pub fn f155(v0: Adt23) -> Adt39 {
    let m419 = v0;
    match m419 {
        Adt23::C0(v1, v2) => {
            let v3: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("kind")), Adt39::C2(f53(Str::lit("sequence")))), List::<(Bytes, Adt39)>::nil());
            let v4: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("stages")), Adt39::C3(std::rc::Rc::new(f4(f78(Fn11::F200, List::<Adt39>::nil(), f80(v1)))))), v3);
            let v5: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("junctions")), Adt39::C3(std::rc::Rc::new(f4(f201(Fn35::F202, List::<Adt39>::nil(), f203(v2)))))), v4);
            f59(v5)
        }
        Adt23::C1(v6, v7) => {
            let v8: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("kind")), Adt39::C2(f53(Str::lit("fanout")))), List::<(Bytes, Adt39)>::nil());
            let v9: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("left")), f72(v6)), v8);
            let v10: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("right")), f72(v7)), v9);
            f59(v10)
        }
        Adt23::C2(v11, v12) => {
            let v13: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("kind")), Adt39::C2(f53(Str::lit("product")))), List::<(Bytes, Adt39)>::nil());
            let v14: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("left")), f72(v11)), v13);
            let v15: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("right")), f72(v12)), v14);
            f59(v15)
        }
        Adt23::C3(v16, v17, v18) => {
            let v19: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("kind")), Adt39::C2(f53(Str::lit("branch")))), List::<(Bytes, Adt39)>::nil());
            let v20: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("guard")), f46(v16)), v19);
            let v21: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("then")), f72(v17)), v20);
            let v22: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("else")), f72(v18)), v21);
            f59(v22)
        }
        Adt23::C4(v23, v24) => {
            let v25: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("kind")), Adt39::C2(f53(Str::lit("scan")))), List::<(Bytes, Adt39)>::nil());
            let v26: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("stage")), f72(v23)), v25);
            let v27: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("junction")), f204(v24)), v26);
            f59(v27)
        }
    }
}

pub fn f156(v0: Fn22, p423: List<Adt39>, p424: List<Adt2>) -> List<Adt39> {
    let m422 = (p423, p424);
    {
        let mut m422 = m422;
        loop {
            let step = {
                let (v1, v2) = m422;
                let m420 = v2;
                match m420.uncons() {
                    None => {
                        Err::<(List<Adt39>, List<Adt2>), List<Adt39>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt39>, List<Adt2>), List<Adt39>>(({
                            let c421: Fn22 = v0.clone();
                            c421.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m422 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f157(v0: List<Adt39>, v1: Adt2) -> List<Adt39> {
    List::cons(f45(v1), v0)
}

pub fn f158(v0: List<Adt2>) -> List<Adt2> {
    f205(Fn36::F206, List::<Adt2>::nil(), v0)
}

pub fn f159(v0: Adt26) -> Adt39 {
    let m425 = v0;
    match m425 {
        Adt26::C0(v1, v2, v3, v4, v5, v6) => {
            let v7: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("kind")), Adt39::C2(f53(Str::lit("dataset_agreement")))), List::<(Bytes, Adt39)>::nil());
            let v8: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("dataset")), f44({
                let (h426, _) = v1;
                h426
            }, {
                let (_, h427) = v1;
                h427
            })), v7);
            let v9: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("comparison")), f44({
                let (h428, _) = v2;
                h428
            }, {
                let (_, h429) = v2;
                h429
            })), v8);
            let v10: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("comparison_sound")), f44({
                let (h430, _) = v3;
                h430
            }, {
                let (_, h431) = v3;
                h431
            })), v9);
            let v11: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("examples")), Adt39::C1(false, v4)), v10);
            let v12: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("agreements")), Adt39::C1(false, v5)), v11);
            let v13: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("theorem")), f44({
                let (h432, _) = v6;
                h432
            }, {
                let (_, h433) = v6;
                h433
            })), v12);
            f59(v13)
        }
        Adt26::C1(v14, v15) => {
            let v16: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("kind")), Adt39::C2(f53(Str::lit("equivalent_to")))), List::<(Bytes, Adt39)>::nil());
            let v17: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("reference")), f72(v14)), v16);
            let v18: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("theorem")), f44({
                let (h434, _) = v15;
                h434
            }, {
                let (_, h435) = v15;
                h435
            })), v17);
            f59(v18)
        }
        Adt26::C2(v19) => {
            let v20: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("kind")), Adt39::C2(f53(Str::lit("initial_invariant")))), List::<(Bytes, Adt39)>::nil());
            let v21: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("theorem")), f44({
                let (h436, _) = v19;
                h436
            }, {
                let (_, h437) = v19;
                h437
            })), v20);
            f59(v21)
        }
        Adt26::C3(v22) => {
            let v23: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("kind")), Adt39::C2(f53(Str::lit("preserves_invariant")))), List::<(Bytes, Adt39)>::nil());
            let v24: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("theorem")), f44({
                let (h438, _) = v22;
                h438
            }, {
                let (_, h439) = v22;
                h439
            })), v23);
            f59(v24)
        }
        Adt26::C4(v25) => {
            let v26: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("kind")), Adt39::C2(f53(Str::lit("satisfies_contract")))), List::<(Bytes, Adt39)>::nil());
            let v27: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("theorem")), f44({
                let (h440, _) = v25;
                h440
            }, {
                let (_, h441) = v25;
                h441
            })), v26);
            f59(v27)
        }
    }
}

pub fn f160(v0: Fn23, p445: List<Adt26>, p446: List<Adt26>) -> List<Adt26> {
    let m444 = (p445, p446);
    {
        let mut m444 = m444;
        loop {
            let step = {
                let (v1, v2) = m444;
                let m442 = v2;
                match m442.uncons() {
                    None => {
                        Err::<(List<Adt26>, List<Adt26>), List<Adt26>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt26>, List<Adt26>), List<Adt26>>(({
                            let c443: Fn23 = v0.clone();
                            c443.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m444 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f161(v0: List<Adt26>, v1: Adt26) -> List<Adt26> {
    List::cons(v1, v0)
}

pub fn f162(v0: Fn24, p450: List<Adt24>, p451: List<Adt24>) -> List<Adt24> {
    let m449 = (p450, p451);
    {
        let mut m449 = m449;
        loop {
            let step = {
                let (v1, v2) = m449;
                let m447 = v2;
                match m447.uncons() {
                    None => {
                        Err::<(List<Adt24>, List<Adt24>), List<Adt24>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt24>, List<Adt24>), List<Adt24>>(({
                            let c448: Fn24 = v0.clone();
                            c448.apply(v1, v3)
                        }, v4))
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

pub fn f163(v0: List<Adt24>, v1: Adt24) -> List<Adt24> {
    List::cons(v1, v0)
}

pub fn f164(v0: Fn25, p455: List<(Option<Bytes>, Bytes)>, p456: List<(Option<Bytes>, Bytes)>) -> List<(Option<Bytes>, Bytes)> {
    let m454 = (p455, p456);
    {
        let mut m454 = m454;
        loop {
            let step = {
                let (v1, v2) = m454;
                let m452 = v2;
                match m452.uncons() {
                    None => {
                        Err::<(List<(Option<Bytes>, Bytes)>, List<(Option<Bytes>, Bytes)>), List<(Option<Bytes>, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Option<Bytes>, Bytes)>, List<(Option<Bytes>, Bytes)>), List<(Option<Bytes>, Bytes)>>(({
                            let c453: Fn25 = v0.clone();
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

pub fn f165(v0: List<(Option<Bytes>, Bytes)>, v1: (Option<Bytes>, Bytes)) -> List<(Option<Bytes>, Bytes)> {
    List::cons(v1, v0)
}

pub fn f166(v0: Adt31) -> Adt39 {
    let m457 = v0;
    match m457 {
        Adt31::C0 => {
            Adt39::C2(f3(Str::lit("breadth_first")))
        }
        Adt31::C1 => {
            Adt39::C2(f3(Str::lit("depth_first")))
        }
    }
}

pub fn f167(v0: Adt33) -> Adt39 {
    let m458 = v0;
    match m458 {
        Adt33::C0(v1) => {
            let v2: List<(Bytes, Adt39)> = List::cons((f3(Str::lit("kind")), Adt39::C2(f3(Str::lit("initial_invariant")))), List::<(Bytes, Adt39)>::nil());
            let v3: List<(Bytes, Adt39)> = List::cons((f3(Str::lit("theorem")), f44({
                let (h459, _) = v1;
                h459
            }, {
                let (_, h460) = v1;
                h460
            })), v2);
            f2(v3)
        }
        Adt33::C1(v4) => {
            let v5: List<(Bytes, Adt39)> = List::cons((f3(Str::lit("kind")), Adt39::C2(f3(Str::lit("terminates")))), List::<(Bytes, Adt39)>::nil());
            let v6: List<(Bytes, Adt39)> = List::cons((f3(Str::lit("theorem")), f44({
                let (h461, _) = v4;
                h461
            }, {
                let (_, h462) = v4;
                h462
            })), v5);
            f2(v6)
        }
        Adt33::C2(v7, v8, v9) => {
            let v10: List<(Bytes, Adt39)> = List::cons((f3(Str::lit("kind")), Adt39::C2(f3(Str::lit("observation_invariant")))), List::<(Bytes, Adt39)>::nil());
            let v11: List<(Bytes, Adt39)> = List::cons((f3(Str::lit("predicate")), f44({
                let (h463, _) = v7;
                h463
            }, {
                let (_, h464) = v7;
                h464
            })), v10);
            let v12: List<(Bytes, Adt39)> = List::cons((f3(Str::lit("initial")), f44({
                let (h465, _) = v8;
                h465
            }, {
                let (_, h466) = v8;
                h466
            })), v11);
            let v13: List<(Bytes, Adt39)> = List::cons((f3(Str::lit("preserved")), f44({
                let (h467, _) = v9;
                h467
            }, {
                let (_, h468) = v9;
                h468
            })), v12);
            f2(v13)
        }
        Adt33::C3(v14) => {
            let v15: List<(Bytes, Adt39)> = List::cons((f3(Str::lit("kind")), Adt39::C2(f3(Str::lit("answer_correct")))), List::<(Bytes, Adt39)>::nil());
            let v16: List<(Bytes, Adt39)> = List::cons((f3(Str::lit("theorem")), f44({
                let (h469, _) = v14;
                h469
            }, {
                let (_, h470) = v14;
                h470
            })), v15);
            f2(v16)
        }
    }
}

pub fn f168(v0: Fn26, p474: List<Adt33>, p475: List<Adt33>) -> List<Adt33> {
    let m473 = (p474, p475);
    {
        let mut m473 = m473;
        loop {
            let step = {
                let (v1, v2) = m473;
                let m471 = v2;
                match m471.uncons() {
                    None => {
                        Err::<(List<Adt33>, List<Adt33>), List<Adt33>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt33>, List<Adt33>), List<Adt33>>(({
                            let c472: Fn26 = v0.clone();
                            c472.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m473 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f169(v0: List<Adt33>, v1: Adt33) -> List<Adt33> {
    List::cons(v1, v0)
}

pub fn f170(v0: Adt37) -> Adt39 {
    let m476 = v0;
    match m476 {
        Adt37::C0 => {
            Adt39::C2(f3(Str::lit("xorshift32")))
        }
    }
}

pub fn f171(v0: Adt38) -> Adt39 {
    let m477 = v0;
    match m477 {
        Adt38::C0 => {
            Adt39::C2(f3(Str::lit("proportional")))
        }
    }
}

pub fn f172(v0: Bytes, v1: u64) -> Bytes {
    let m481 = {
        let a478 = v0;
        let a479 = v1;
        let a480 = 1u64;
        slice_bytes(a478, a479, a480)
    };
    match m481 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f173(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f207(v1, v0, v2)
}

pub fn f174(v0: Bytes, v1: u64, v2: u8) -> R<Bytes> {
    let v4: u64 = {
        let v3: u8 = v2;
        let a526 = {
            let a502 = {
                let a490 = {
                    let a484 = {
                        let a482 = v3;
                        let a483 = 128u8;
                        fixed_u8::bit_and(a482, a483)
                    };
                    let a485 = 128u8;
                    if equal(a484, a485) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a491 = {
                    let a488 = {
                        let a486 = v3;
                        let a487 = 64u8;
                        fixed_u8::bit_and(a486, a487)
                    };
                    let a489 = 64u8;
                    if equal(a488, a489) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a490, a491)?
            };
            let a503 = {
                let a500 = {
                    let a494 = {
                        let a492 = v3;
                        let a493 = 32u8;
                        fixed_u8::bit_and(a492, a493)
                    };
                    let a495 = 32u8;
                    if equal(a494, a495) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a501 = {
                    let a498 = {
                        let a496 = v3;
                        let a497 = 16u8;
                        fixed_u8::bit_and(a496, a497)
                    };
                    let a499 = 16u8;
                    if equal(a498, a499) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a500, a501)?
            };
            nat_add(a502, a503)?
        };
        let a527 = {
            let a524 = {
                let a512 = {
                    let a506 = {
                        let a504 = v3;
                        let a505 = 8u8;
                        fixed_u8::bit_and(a504, a505)
                    };
                    let a507 = 8u8;
                    if equal(a506, a507) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a513 = {
                    let a510 = {
                        let a508 = v3;
                        let a509 = 4u8;
                        fixed_u8::bit_and(a508, a509)
                    };
                    let a511 = 4u8;
                    if equal(a510, a511) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a512, a513)?
            };
            let a525 = {
                let a522 = {
                    let a516 = {
                        let a514 = v3;
                        let a515 = 2u8;
                        fixed_u8::bit_and(a514, a515)
                    };
                    let a517 = 2u8;
                    if equal(a516, a517) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a523 = {
                    let a520 = {
                        let a518 = v3;
                        let a519 = 1u8;
                        fixed_u8::bit_and(a518, a519)
                    };
                    let a521 = 1u8;
                    if equal(a520, a521) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a522, a523)?
            };
            nat_add(a524, a525)?
        };
        nat_add(a526, a527)?
    };
    let a528 = v4;
    let a529 = 34u64;
    if nat_eq(a528, a529) {
        Ok(Bytes::lit(&[0x5c, 0x22]))
    } else {
        let a530 = v4;
        let a531 = 92u64;
        if nat_eq(a530, a531) {
            Ok(Bytes::lit(&[0x5c, 0x5c]))
        } else {
            let a532 = v4;
            let a533 = 8u64;
            if nat_eq(a532, a533) {
                Ok(Bytes::lit(&[0x5c, 0x62]))
            } else {
                let a534 = v4;
                let a535 = 12u64;
                if nat_eq(a534, a535) {
                    Ok(Bytes::lit(&[0x5c, 0x66]))
                } else {
                    let a536 = v4;
                    let a537 = 10u64;
                    if nat_eq(a536, a537) {
                        Ok(Bytes::lit(&[0x5c, 0x6e]))
                    } else {
                        let a538 = v4;
                        let a539 = 13u64;
                        if nat_eq(a538, a539) {
                            Ok(Bytes::lit(&[0x5c, 0x72]))
                        } else {
                            let a540 = v4;
                            let a541 = 9u64;
                            if nat_eq(a540, a541) {
                                Ok(Bytes::lit(&[0x5c, 0x74]))
                            } else {
                                let a542 = v4;
                                let a543 = 32u64;
                                if nat_lt(a542, a543) {
                                    let a544 = Bytes::lit(&[0x5c, 0x75, 0x30, 0x30]);
                                    let a545 = f208(v2)?;
                                    Ok(append_bytes(a544, a545))
                                } else {
                                    Ok(f209(v0, v1, 1u64))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f175(v0: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    f210(Fn37::F211, List::<List<(Bytes, Bytes)>>::nil(), v0)
}

pub fn f176(v0: Fn27, p549: List<List<(Bytes, Bytes)>>, p550: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    let m548 = (p549, p550);
    {
        let mut m548 = m548;
        loop {
            let step = {
                let (v1, v2) = m548;
                let m546 = v2;
                match m546.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(({
                            let c547: Fn27 = v0.clone();
                            c547.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m548 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f177(v0: List<List<(Bytes, Bytes)>>, v1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
    List::cons(List::cons(v1, List::<(Bytes, Bytes)>::nil()), v0)
}

pub fn f178(v0: Fn28, p555: u64, p556: List<List<(Bytes, Bytes)>>) -> R<(List<List<(Bytes, Bytes)>>, bool)> {
    let m554 = (p555, p556);
    {
        let mut m554 = m554;
        loop {
            let step = {
                let (v1, v2) = m554;
                let m551 = v1;
                if m551 == 0 {
                    Err::<(u64, List<List<(Bytes, Bytes)>>), R<(List<List<(Bytes, Bytes)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m551 - 1;
                    let m553 = {
                        let c552: Fn28 = v0.clone();
                        c552.apply(v2.clone())?
                    };
                    match m553 {
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
                Ok(next) => { m554 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f179(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    f212(v0)
}

pub fn f180(v0: Fn29, p560: List<(Bytes, Adt2)>, p561: List<(Bytes, Adt2)>) -> List<(Bytes, Adt2)> {
    let m559 = (p560, p561);
    {
        let mut m559 = m559;
        loop {
            let step = {
                let (v1, v2) = m559;
                let m557 = v2;
                match m557.uncons() {
                    None => {
                        Err::<(List<(Bytes, Adt2)>, List<(Bytes, Adt2)>), List<(Bytes, Adt2)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Adt2)>, List<(Bytes, Adt2)>), List<(Bytes, Adt2)>>(({
                            let c558: Fn29 = v0.clone();
                            c558.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m559 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f181(v0: List<(Bytes, Adt2)>, v1: (Bytes, Adt2)) -> List<(Bytes, Adt2)> {
    List::cons(v1, v0)
}

pub fn f182(v0: Str) -> Bytes {
    let a562 = v0;
    utf8_encode(a562)
}

pub fn f183(v0: Str) -> Bytes {
    let a563 = v0;
    utf8_encode(a563)
}

pub fn f184(v0: List<Adt39>, v1: (Option<Bytes>, Bytes)) -> List<Adt39> {
    List::cons(f44({
        let (h564, _) = v1;
        h564
    }, {
        let (_, h565) = v1;
        h565
    }), v0)
}

pub fn f185(v0: Fn30, p569: List<Adt39>, p570: List<(Bytes, Bytes)>) -> List<Adt39> {
    let m568 = (p569, p570);
    {
        let mut m568 = m568;
        loop {
            let step = {
                let (v1, v2) = m568;
                let m566 = v2;
                match m566.uncons() {
                    None => {
                        Err::<(List<Adt39>, List<(Bytes, Bytes)>), List<Adt39>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt39>, List<(Bytes, Bytes)>), List<Adt39>>(({
                            let c567: Fn30 = v0.clone();
                            c567.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m568 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f186(v0: List<Adt39>, v1: (Bytes, Bytes)) -> List<Adt39> {
    List::cons(Adt39::C4(std::rc::Rc::new(f122(Str::lit("field"), Adt39::C2({
        let (h571, _) = v1;
        h571
    }), f122(Str::lit("expected"), Adt39::C2({
        let (_, h572) = v1;
        h572
    }), Adt39::C5)))), v0)
}

pub fn f187(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f32(Fn6::F213, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f188(v0: Fn31, p576: List<u64>, p577: List<u64>) -> List<u64> {
    let m575 = (p576, p577);
    {
        let mut m575 = m575;
        loop {
            let step = {
                let (v1, v2) = m575;
                let m573 = v2;
                match m573.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>(({
                            let c574: Fn31 = v0.clone();
                            c574.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m575 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f189(v0: List<u64>, v1: u64) -> List<u64> {
    List::cons(v1, v0)
}

pub fn f190(v0: Adt15) -> Adt39 {
    let m578 = v0;
    match m578 {
        Adt15::C0 => {
            Adt39::C2(f53(Str::lit("invariant")))
        }
        Adt15::C1 => {
            Adt39::C2(f53(Str::lit("postcondition")))
        }
        Adt15::C2 => {
            Adt39::C2(f53(Str::lit("precondition")))
        }
    }
}

pub fn f191(v0: Adt18) -> Adt39 {
    let m579 = v0;
    match m579 {
        Adt18::C0(v1, v2, v3) => {
            let v4: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("name")), Adt39::C2(v1)), List::<(Bytes, Adt39)>::nil());
            let v5: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("guard")), f46(v2)), v4);
            let v6: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("action")), f46(v3)), v5);
            f59(v6)
        }
    }
}

pub fn f192(v0: Fn32, p583: List<Adt18>, p584: List<Adt18>) -> List<Adt18> {
    let m582 = (p583, p584);
    {
        let mut m582 = m582;
        loop {
            let step = {
                let (v1, v2) = m582;
                let m580 = v2;
                match m580.uncons() {
                    None => {
                        Err::<(List<Adt18>, List<Adt18>), List<Adt18>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt18>, List<Adt18>), List<Adt18>>(({
                            let c581: Fn32 = v0.clone();
                            c581.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m582 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f193(v0: List<Adt18>, v1: Adt18) -> List<Adt18> {
    List::cons(v1, v0)
}

pub fn f194(v0: Fn33, p588: List<Adt3>, p589: List<Adt3>) -> List<Adt3> {
    let m587 = (p588, p589);
    {
        let mut m587 = m587;
        loop {
            let step = {
                let (v1, v2) = m587;
                let m585 = v2;
                match m585.uncons() {
                    None => {
                        Err::<(List<Adt3>, List<Adt3>), List<Adt3>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt3>, List<Adt3>), List<Adt3>>(({
                            let c586: Fn33 = v0.clone();
                            c586.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m587 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f195(v0: List<Adt3>, v1: Adt3) -> List<Adt3> {
    List::cons(v1, v0)
}

pub fn f196(v0: Adt21) -> Adt39 {
    let m590 = v0;
    match m590 {
        Adt21::C0(v1, v2, v3, v4) => {
            let v5: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("kind")), Adt39::C2(f53(Str::lit("dense")))), List::<(Bytes, Adt39)>::nil());
            let v6: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("inputs")), Adt39::C1(false, v1)), v5);
            let v7: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("outputs")), Adt39::C1(false, v2)), v6);
            let v8: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("weights")), f44({
                let (h591, _) = v3;
                h591
            }, {
                let (_, h592) = v3;
                h592
            })), v7);
            let v9: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("bias")), f44({
                let (h593, _) = v4;
                h593
            }, {
                let (_, h594) = v4;
                h594
            })), v8);
            f59(v9)
        }
        Adt21::C1 => {
            let v10: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("kind")), Adt39::C2(f53(Str::lit("relu")))), List::<(Bytes, Adt39)>::nil());
            f59(v10)
        }
        Adt21::C2(v11, v12, v13) => {
            let v14: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("kind")), Adt39::C2(f53(Str::lit("requantize")))), List::<(Bytes, Adt39)>::nil());
            let v15: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("shift")), Adt39::C1(false, v11)), v14);
            let v16: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("minimum")), Adt39::C2(v12)), v15);
            let v17: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("maximum")), Adt39::C2(v13)), v16);
            f59(v17)
        }
    }
}

pub fn f197(v0: Fn34, p598: List<Adt21>, p599: List<Adt21>) -> List<Adt21> {
    let m597 = (p598, p599);
    {
        let mut m597 = m597;
        loop {
            let step = {
                let (v1, v2) = m597;
                let m595 = v2;
                match m595.uncons() {
                    None => {
                        Err::<(List<Adt21>, List<Adt21>), List<Adt21>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt21>, List<Adt21>), List<Adt21>>(({
                            let c596: Fn34 = v0.clone();
                            c596.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m597 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f198(v0: List<Adt21>, v1: Adt21) -> List<Adt21> {
    List::cons(v1, v0)
}

pub fn f199(v0: List<Adt39>, v1: Adt3) -> List<Adt39> {
    List::cons(f46(v1), v0)
}

pub fn f200(v0: List<Adt39>, v1: Adt24) -> List<Adt39> {
    List::cons(f72(v1), v0)
}

pub fn f201(v0: Fn35, p603: List<Adt39>, p604: List<Adt25>) -> List<Adt39> {
    let m602 = (p603, p604);
    {
        let mut m602 = m602;
        loop {
            let step = {
                let (v1, v2) = m602;
                let m600 = v2;
                match m600.uncons() {
                    None => {
                        Err::<(List<Adt39>, List<Adt25>), List<Adt39>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt39>, List<Adt25>), List<Adt39>>(({
                            let c601: Fn35 = v0.clone();
                            c601.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m602 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f202(v0: List<Adt39>, v1: Adt25) -> List<Adt39> {
    List::cons(f204(v1), v0)
}

pub fn f203(v0: List<Adt25>) -> List<Adt25> {
    f214(Fn38::F215, List::<Adt25>::nil(), v0)
}

pub fn f204(v0: Adt25) -> Adt39 {
    let m605 = v0;
    match m605 {
        Adt25::C0 => {
            let v1: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("kind")), Adt39::C2(f53(Str::lit("unconditional")))), List::<(Bytes, Adt39)>::nil());
            f59(v1)
        }
        Adt25::C1(v2) => {
            let v3: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("kind")), Adt39::C2(f53(Str::lit("proved")))), List::<(Bytes, Adt39)>::nil());
            let v4: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("evidence")), f44({
                let (h606, _) = v2;
                h606
            }, {
                let (_, h607) = v2;
                h607
            })), v3);
            f59(v4)
        }
        Adt25::C2 => {
            let v5: List<(Bytes, Adt39)> = List::cons((f53(Str::lit("kind")), Adt39::C2(f53(Str::lit("checked")))), List::<(Bytes, Adt39)>::nil());
            f59(v5)
        }
    }
}

pub fn f205(v0: Fn36, p611: List<Adt2>, p612: List<Adt2>) -> List<Adt2> {
    let m610 = (p611, p612);
    {
        let mut m610 = m610;
        loop {
            let step = {
                let (v1, v2) = m610;
                let m608 = v2;
                match m608.uncons() {
                    None => {
                        Err::<(List<Adt2>, List<Adt2>), List<Adt2>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt2>, List<Adt2>), List<Adt2>>(({
                            let c609: Fn36 = v0.clone();
                            c609.apply(v1, v3)
                        }, v4))
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

pub fn f206(v0: List<Adt2>, v1: Adt2) -> List<Adt2> {
    List::cons(v1, v0)
}

pub fn f207(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h613, _) = v2;
        h613
    };
    let a614 = v3;
    let a615 = 0u64;
    if nat_eq(a614, a615) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a616 = v3;
            let a617 = v1;
            let a618 = 0u64;
            nat_quot(a616, a617, a618)
        }, {
            let a623 = f172(v0, {
                let a619 = v3;
                let a620 = v1;
                let a621 = 0u64;
                nat_rem(a619, a620, a621)
            });
            let a624 = {
                let (_, h622) = v2;
                h622
            };
            append_bytes(a623, a624)
        }))
    }
}

pub fn f208(v0: u8) -> R<Bytes> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a669 = {
            let a645 = {
                let a633 = {
                    let a627 = {
                        let a625 = v1;
                        let a626 = 128u8;
                        fixed_u8::bit_and(a625, a626)
                    };
                    let a628 = 128u8;
                    if equal(a627, a628) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a634 = {
                    let a631 = {
                        let a629 = v1;
                        let a630 = 64u8;
                        fixed_u8::bit_and(a629, a630)
                    };
                    let a632 = 64u8;
                    if equal(a631, a632) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a633, a634)?
            };
            let a646 = {
                let a643 = {
                    let a637 = {
                        let a635 = v1;
                        let a636 = 32u8;
                        fixed_u8::bit_and(a635, a636)
                    };
                    let a638 = 32u8;
                    if equal(a637, a638) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a644 = {
                    let a641 = {
                        let a639 = v1;
                        let a640 = 16u8;
                        fixed_u8::bit_and(a639, a640)
                    };
                    let a642 = 16u8;
                    if equal(a641, a642) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a643, a644)?
            };
            nat_add(a645, a646)?
        };
        let a670 = {
            let a667 = {
                let a655 = {
                    let a649 = {
                        let a647 = v1;
                        let a648 = 8u8;
                        fixed_u8::bit_and(a647, a648)
                    };
                    let a650 = 8u8;
                    if equal(a649, a650) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a656 = {
                    let a653 = {
                        let a651 = v1;
                        let a652 = 4u8;
                        fixed_u8::bit_and(a651, a652)
                    };
                    let a654 = 4u8;
                    if equal(a653, a654) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a655, a656)?
            };
            let a668 = {
                let a665 = {
                    let a659 = {
                        let a657 = v1;
                        let a658 = 2u8;
                        fixed_u8::bit_and(a657, a658)
                    };
                    let a660 = 2u8;
                    if equal(a659, a660) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a666 = {
                    let a663 = {
                        let a661 = v1;
                        let a662 = 1u8;
                        fixed_u8::bit_and(a661, a662)
                    };
                    let a664 = 1u8;
                    if equal(a663, a664) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a665, a666)?
            };
            nat_add(a667, a668)?
        };
        nat_add(a669, a670)?
    };
    let a677 = f172(f216(), {
        let a671 = v2;
        let a672 = 16u64;
        let a673 = 0u64;
        nat_quot(a671, a672, a673)
    });
    let a678 = f172(f216(), {
        let a674 = v2;
        let a675 = 16u64;
        let a676 = 0u64;
        nat_rem(a674, a675, a676)
    });
    Ok(append_bytes(a677, a678))
}

pub fn f209(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m682 = {
        let a679 = v0;
        let a680 = v1;
        let a681 = v2;
        slice_bytes(a679, a680, a681)
    };
    match m682 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f210(v0: Fn37, p686: List<List<(Bytes, Bytes)>>, p687: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    let m685 = (p686, p687);
    {
        let mut m685 = m685;
        loop {
            let step = {
                let (v1, v2) = m685;
                let m683 = v2;
                match m683.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(({
                            let c684: Fn37 = v0.clone();
                            c684.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m685 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f211(v0: List<List<(Bytes, Bytes)>>, v1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    List::cons(v1, v0)
}

pub fn f212(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    let a689 = {
        let a688 = v0.clone();
        length_list(a688)
    };
    let a690 = 1u64;
    if nat_le(a689, a690) {
        Ok(None::<List<List<(Bytes, Bytes)>>>)
    } else {
        Ok(Some(f217(v0)?))
    }
}

pub fn f213(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn f214(v0: Fn38, p694: List<Adt25>, p695: List<Adt25>) -> List<Adt25> {
    let m693 = (p694, p695);
    {
        let mut m693 = m693;
        loop {
            let step = {
                let (v1, v2) = m693;
                let m691 = v2;
                match m691.uncons() {
                    None => {
                        Err::<(List<Adt25>, List<Adt25>), List<Adt25>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt25>, List<Adt25>), List<Adt25>>(({
                            let c692: Fn38 = v0.clone();
                            c692.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m693 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f215(v0: List<Adt25>, v1: Adt25) -> List<Adt25> {
    List::cons(v1, v0)
}

pub fn f216() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66])
}

pub fn f217(v0: List<List<(Bytes, Bytes)>>) -> R<List<List<(Bytes, Bytes)>>> {
    Ok(f175({
        let (_, h696) = {
            let (h697, _) = f218(Fn39::F219, {
                let a699 = {
                    let a698 = v0.clone();
                    length_list(a698)
                };
                let a700 = 1u64;
                nat_add(a699, a700)?
            }, (v0, List::<List<(Bytes, Bytes)>>::nil()))?;
            h697
        };
        h696
    }))
}

pub fn f218(v0: Fn39, p705: u64, p706: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)> {
    let m704 = (p705, p706);
    {
        let mut m704 = m704;
        loop {
            let step = {
                let (v1, v2) = m704;
                let m701 = v1;
                if m701 == 0 {
                    Err::<(u64, (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)), R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m701 - 1;
                    let m703 = {
                        let c702: Fn39 = v0.clone();
                        c702.apply(v2.clone())?
                    };
                    match m703 {
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
                Ok(next) => { m704 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f219(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    f220(v0)
}

pub fn f220(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    let v1: List<List<(Bytes, Bytes)>> = {
        let (h707, _) = v0;
        h707
    };
    let v2: List<List<(Bytes, Bytes)>> = {
        let (_, h708) = v0;
        h708
    };
    let m709 = v1;
    match m709.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>)
        }
        Some((v3, v4)) => {
            let m710 = v4;
            match m710.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Bytes)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f221(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f221(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v2: u64 = {
        let a715 = {
            let a713 = {
                let a711 = v0.clone();
                length_list(a711)
            };
            let a714 = {
                let a712 = v1.clone();
                length_list(a712)
            };
            nat_add(a713, a714)?
        };
        let a716 = 1u64;
        nat_add(a715, a716)?
    };
    Ok(f187({
        let (_, h717) = {
            let (_, h718) = {
                let (h719, _) = f222(Fn40::F223, v2, (v0, (v1, List::<(Bytes, Bytes)>::nil())));
                h719
            };
            h718
        };
        h717
    }))
}

pub fn f222(v0: Fn40, p724: u64, p725: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool) {
    let m723 = (p724, p725);
    {
        let mut m723 = m723;
        loop {
            let step = {
                let (v1, v2) = m723;
                let m720 = v1;
                if m720 == 0 {
                    Err::<(u64, (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))), ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool)>((v2, false))
                } else {
                    let v3 = m720 - 1;
                    let m722 = {
                        let c721: Fn40 = v0.clone();
                        c721.apply(v2.clone())
                    };
                    match m722 {
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
                Ok(next) => { m723 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f223(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    f224(v0)
}

pub fn f224(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    let v1: List<(Bytes, Bytes)> = {
        let (h726, _) = v0.clone();
        h726
    };
    let v2: List<(Bytes, Bytes)> = {
        let (h727, _) = {
            let (_, h728) = v0.clone();
            h728
        };
        h727
    };
    let v3: List<(Bytes, Bytes)> = {
        let (_, h729) = {
            let (_, h730) = v0;
            h730
        };
        h729
    };
    let m731 = v1.clone();
    match m731.uncons() {
        None => {
            let m732 = v2;
            match m732.uncons() {
                None => {
                    None::<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Bytes)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m733 = v2.clone();
            match m733.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Bytes)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f225({
                        let (h734, _) = v8.clone();
                        h734
                    }, {
                        let (h735, _) = v6.clone();
                        h735
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

pub fn f225(v0: Bytes, v1: Bytes) -> bool {
    let a740 = {
        let a736 = v0;
        let a737 = v1;
        compare_bytes(a736, a737)
    };
    let a741 = {
        let a738 = Bytes::lit(&[]);
        let a739 = Bytes::lit(&[0x01]);
        compare_bytes(a738, a739)
    };
    equal(a740, a741)
}

pub fn run(p0: Bytes, p1: List<Adt0>) -> R<Bytes> {
    f0(p0, p1)
}
