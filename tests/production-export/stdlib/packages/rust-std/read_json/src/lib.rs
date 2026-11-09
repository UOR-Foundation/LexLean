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
    C0(bool),
    C1(bool, u64),
    C2(Bytes),
    C3(std::rc::Rc<Adt0>),
    C4(std::rc::Rc<Adt0>),
    C5,
    C6(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
    C7(Bytes, std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(u64, u64, List<Adt2>, Adt0),
}

#[derive(Clone)]
pub enum Adt2 {
    C0(Adt0),
    C1(Bytes, Adt0),
}

#[derive(Clone)]
pub enum Fn0 {
    F3(Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: Adt1) -> R<Option<Adt1>> {
        match self {
            Fn0::F3(k0) => f3(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F18(Bytes),
}

impl Fn1 {
    pub fn apply(&self, p0: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
        match self {
            Fn1::F18(k0) => f18(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F26,
}

impl Fn2 {
    pub fn apply(&self, p0: (Adt0, Adt0)) -> Option<(Adt0, Adt0)> {
        match self {
            Fn2::F26 => f26(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F28(Bytes),
    F39(Bytes),
}

impl Fn3 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn3::F28(k0) => f28(k0.clone(), p0),
            Fn3::F39(k0) => f39(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F31(Bytes),
}

impl Fn4 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn4::F31(k0) => f31(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F45(Bytes),
}

impl Fn5 {
    pub fn apply(&self, p0: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
        match self {
            Fn5::F45(k0) => f45(k0.clone(), p0),
        }
    }
}

pub fn f0(v0: Bytes) -> R<Option<Adt0>> {
    f1(v0)
}

pub fn f1(v0: Bytes) -> R<Option<Adt0>> {
    let m5 = {
        let (h1, _) = f2(Fn0::F3(v0.clone()), {
            let a3 = {
                let a2 = v0;
                length_bytes(a2)
            };
            let a4 = 3u64;
            nat_add(a3, a4)?
        }, Adt1::C0(0u64, 0u64, List::<Adt2>::nil(), Adt0::C5))?;
        h1
    };
    match m5 {
        Adt1::C0(_, v2, _, v4) => {
            let a6 = v2;
            let a7 = 8u64;
            if nat_eq(a6, a7) {
                Ok(Some(v4))
            } else {
                Ok(None::<Adt0>)
            }
        }
    }
}

pub fn f2(v0: Fn0, p12: u64, p13: Adt1) -> R<(Adt1, bool)> {
    let m11 = (p12, p13);
    {
        let mut m11 = m11;
        loop {
            let step = {
                let (v1, v2) = m11;
                let m8 = v1;
                if m8 == 0 {
                    Err::<(u64, Adt1), R<(Adt1, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m8 - 1;
                    let m10 = {
                        let c9: Fn0 = v0.clone();
                        c9.apply(v2.clone())?
                    };
                    match m10 {
                        None => {
                            Err::<(u64, Adt1), R<(Adt1, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt1), R<(Adt1, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m11 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f3(v0: Bytes, v1: Adt1) -> R<Option<Adt1>> {
    f4(v0, v1)
}

pub fn f4(v0: Bytes, v1: Adt1) -> R<Option<Adt1>> {
    let m14 = v1;
    match m14 {
        Adt1::C0(v2, v3, v4, v5) => {
            let a15 = 8u64;
            let a16 = v3;
            if nat_le(a15, a16) {
                Ok(None::<Adt1>)
            } else {
                Ok(Some(f5(v0, v2, v3, v4, v5)?))
            }
        }
    }
}

pub fn f5(v0: Bytes, v1: u64, v2: u64, v3: List<Adt2>, v4: Adt0) -> R<Adt1> {
    let v5: u64 = f6(v0.clone(), v1)?;
    let a17 = v2;
    let a18 = 7u64;
    if nat_eq(a17, a18) {
        Ok(Adt1::C0(v1, {
            let a19 = v5;
            let a20 = 256u64;
            if nat_eq(a19, a20) {
                8u64
            } else {
                9u64
            }
        }, v3, v4))
    } else {
        let a21 = v5;
        let a22 = 256u64;
        if nat_eq(a21, a22) {
            Ok(f7(v1))
        } else {
            let a23 = v2;
            let a24 = 0u64;
            if if nat_eq(a23, a24) {
                true
            } else {
                let a25 = v2;
                let a26 = 1u64;
                nat_eq(a25, a26)
            } {
                f8(v0, v1, v2, v3, v5)
            } else {
                let a27 = v2;
                let a28 = 2u64;
                if if nat_eq(a27, a28) {
                    true
                } else {
                    let a29 = v2;
                    let a30 = 3u64;
                    nat_eq(a29, a30)
                } {
                    f9(v0, v1, v2, v3, v5)
                } else {
                    let a31 = v2;
                    let a32 = 4u64;
                    if nat_eq(a31, a32) {
                        let a33 = v5;
                        let a34 = 58u64;
                        if nat_eq(a33, a34) {
                            Ok(Adt1::C0({
                                let a35 = v1;
                                let a36 = 1u64;
                                nat_add(a35, a36)?
                            }, 0u64, v3, v4))
                        } else {
                            Ok(f7(v1))
                        }
                    } else {
                        f10(v0, v1, v3, v5)
                    }
                }
            }
        }
    }
}

pub fn f6(v0: Bytes, v1: u64) -> R<u64> {
    let m39 = {
        let a37 = v0;
        let a38 = v1;
        index_bytes(a37, a38)
    };
    match m39 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a84 = {
                let a60 = {
                    let a48 = {
                        let a42 = {
                            let a40 = v3;
                            let a41 = 128u8;
                            fixed_u8::bit_and(a40, a41)
                        };
                        let a43 = 128u8;
                        if equal(a42, a43) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a49 = {
                        let a46 = {
                            let a44 = v3;
                            let a45 = 64u8;
                            fixed_u8::bit_and(a44, a45)
                        };
                        let a47 = 64u8;
                        if equal(a46, a47) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a48, a49)?
                };
                let a61 = {
                    let a58 = {
                        let a52 = {
                            let a50 = v3;
                            let a51 = 32u8;
                            fixed_u8::bit_and(a50, a51)
                        };
                        let a53 = 32u8;
                        if equal(a52, a53) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a59 = {
                        let a56 = {
                            let a54 = v3;
                            let a55 = 16u8;
                            fixed_u8::bit_and(a54, a55)
                        };
                        let a57 = 16u8;
                        if equal(a56, a57) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a58, a59)?
                };
                nat_add(a60, a61)?
            };
            let a85 = {
                let a82 = {
                    let a70 = {
                        let a64 = {
                            let a62 = v3;
                            let a63 = 8u8;
                            fixed_u8::bit_and(a62, a63)
                        };
                        let a65 = 8u8;
                        if equal(a64, a65) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a71 = {
                        let a68 = {
                            let a66 = v3;
                            let a67 = 4u8;
                            fixed_u8::bit_and(a66, a67)
                        };
                        let a69 = 4u8;
                        if equal(a68, a69) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a70, a71)?
                };
                let a83 = {
                    let a80 = {
                        let a74 = {
                            let a72 = v3;
                            let a73 = 2u8;
                            fixed_u8::bit_and(a72, a73)
                        };
                        let a75 = 2u8;
                        if equal(a74, a75) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a81 = {
                        let a78 = {
                            let a76 = v3;
                            let a77 = 1u8;
                            fixed_u8::bit_and(a76, a77)
                        };
                        let a79 = 1u8;
                        if equal(a78, a79) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a80, a81)?
                };
                nat_add(a82, a83)?
            };
            nat_add(a84, a85)
        }
    }
}

pub fn f7(v0: u64) -> Adt1 {
    Adt1::C0(v0, 9u64, List::<Adt2>::nil(), Adt0::C5)
}

pub fn f8(v0: Bytes, v1: u64, v2: u64, v3: List<Adt2>, v4: u64) -> R<Adt1> {
    let a86 = v4;
    let a87 = 34u64;
    if nat_eq(a86, a87) {
        let m90 = f11(v0, {
            let a88 = v1;
            let a89 = 1u64;
            nat_add(a88, a89)?
        })?;
        match m90 {
            None => {
                Ok(f7(v1))
            }
            Some(v5) => {
                Ok(f12({
                    let (h91, _) = v5;
                    h91
                }, v3, Adt0::C2({
                    let (_, h92) = v5;
                    h92
                })))
            }
        }
    } else {
        let a93 = v4;
        let a94 = 116u64;
        if nat_eq(a93, a94) {
            let a95 = f13(v0, v1, 4u64);
            let a96 = Bytes::lit(&[0x74, 0x72, 0x75, 0x65]);
            if equal(a95, a96) {
                Ok(f12({
                    let a97 = v1;
                    let a98 = 4u64;
                    nat_add(a97, a98)?
                }, v3, Adt0::C0(true)))
            } else {
                Ok(f7(v1))
            }
        } else {
            let a99 = v4;
            let a100 = 102u64;
            if nat_eq(a99, a100) {
                let a101 = f13(v0, v1, 5u64);
                let a102 = Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]);
                if equal(a101, a102) {
                    Ok(f12({
                        let a103 = v1;
                        let a104 = 5u64;
                        nat_add(a103, a104)?
                    }, v3, Adt0::C0(false)))
                } else {
                    Ok(f7(v1))
                }
            } else {
                let a105 = v4;
                let a106 = 91u64;
                if nat_eq(a105, a106) {
                    Ok(Adt1::C0({
                        let a107 = v1;
                        let a108 = 1u64;
                        nat_add(a107, a108)?
                    }, 1u64, List::cons(Adt2::C0(Adt0::C5), v3), Adt0::C5))
                } else {
                    let a109 = v4;
                    let a110 = 123u64;
                    if nat_eq(a109, a110) {
                        Ok(Adt1::C0({
                            let a111 = v1;
                            let a112 = 1u64;
                            nat_add(a111, a112)?
                        }, 3u64, List::cons(Adt2::C1(Bytes::lit(&[]), Adt0::C5), v3), Adt0::C5))
                    } else {
                        let a113 = v4;
                        let a114 = 93u64;
                        if if nat_eq(a113, a114) {
                            let a115 = v2;
                            let a116 = 1u64;
                            nat_eq(a115, a116)
                        } else {
                            false
                        } {
                            f14(v0, v1, v3)
                        } else {
                            let a117 = v4;
                            let a118 = 45u64;
                            if if nat_eq(a117, a118) {
                                true
                            } else {
                                let a119 = 47u64;
                                let a120 = v4;
                                if nat_lt(a119, a120) {
                                    let a121 = v4;
                                    let a122 = 58u64;
                                    nat_lt(a121, a122)
                                } else {
                                    false
                                }
                            } {
                                let m123 = f15(v0, v1)?;
                                match m123 {
                                    None => {
                                        Ok(f7(v1))
                                    }
                                    Some(v6) => {
                                        Ok(f12({
                                            let (h124, _) = v6;
                                            h124
                                        }, v3, {
                                            let (_, h125) = v6;
                                            h125
                                        }))
                                    }
                                }
                            } else {
                                Ok(f7(v1))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f9(v0: Bytes, v1: u64, v2: u64, v3: List<Adt2>, v4: u64) -> R<Adt1> {
    let a126 = v4;
    let a127 = 34u64;
    if nat_eq(a126, a127) {
        let m130 = f11(v0, {
            let a128 = v1;
            let a129 = 1u64;
            nat_add(a128, a129)?
        })?;
        match m130 {
            None => {
                Ok(f7(v1))
            }
            Some(v5) => {
                let m131 = v3;
                match m131.uncons() {
                    None => {
                        Ok(f7(v1))
                    }
                    Some((v6, v7)) => {
                        let m132 = v6;
                        match m132 {
                            Adt2::C0(_) => {
                                Ok(f7(v1))
                            }
                            Adt2::C1(_, v10) => {
                                if f16(v10.clone(), {
                                    let (_, h133) = v5.clone();
                                    h133
                                }) {
                                    Ok(Adt1::C0({
                                        let (h134, _) = v5;
                                        h134
                                    }, 4u64, List::cons(Adt2::C1({
                                        let (_, h135) = v5;
                                        h135
                                    }, v10), v7), Adt0::C5))
                                } else {
                                    Ok(f7(v1))
                                }
                            }
                        }
                    }
                }
            }
        }
    } else {
        let a136 = v4;
        let a137 = 125u64;
        if if nat_eq(a136, a137) {
            let a138 = v2;
            let a139 = 3u64;
            nat_eq(a138, a139)
        } else {
            false
        } {
            f14(v0, v1, v3)
        } else {
            Ok(f7(v1))
        }
    }
}

pub fn f10(v0: Bytes, v1: u64, v2: List<Adt2>, v3: u64) -> R<Adt1> {
    let m140 = v2.clone();
    match m140.uncons() {
        None => {
            Ok(f7(v1))
        }
        Some((v4, _)) => {
            let m141 = v4;
            match m141 {
                Adt2::C0(_) => {
                    let a142 = v3;
                    let a143 = 44u64;
                    if nat_eq(a142, a143) {
                        Ok(Adt1::C0({
                            let a144 = v1;
                            let a145 = 1u64;
                            nat_add(a144, a145)?
                        }, 0u64, v2, Adt0::C5))
                    } else {
                        let a146 = v3;
                        let a147 = 93u64;
                        if nat_eq(a146, a147) {
                            f14(v0, v1, v2)
                        } else {
                            Ok(f7(v1))
                        }
                    }
                }
                Adt2::C1(_, _) => {
                    let a148 = v3;
                    let a149 = 44u64;
                    if nat_eq(a148, a149) {
                        Ok(Adt1::C0({
                            let a150 = v1;
                            let a151 = 1u64;
                            nat_add(a150, a151)?
                        }, 2u64, v2, Adt0::C5))
                    } else {
                        let a152 = v3;
                        let a153 = 125u64;
                        if nat_eq(a152, a153) {
                            f14(v0, v1, v2)
                        } else {
                            Ok(f7(v1))
                        }
                    }
                }
            }
        }
    }
}

pub fn f11(v0: Bytes, v1: u64) -> R<Option<(u64, Bytes)>> {
    let v2: ((u64, (Bytes, u64)), bool) = f17(Fn1::F18(v0.clone()), {
        let a155 = {
            let a154 = v0;
            length_bytes(a154)
        };
        let a156 = 1u64;
        nat_add(a155, a156)?
    }, (v1, (Bytes::lit(&[]), 0u64)))?;
    let a160 = {
        let (_, h157) = {
            let (_, h158) = {
                let (h159, _) = v2.clone();
                h159
            };
            h158
        };
        h157
    };
    let a161 = 1u64;
    if nat_eq(a160, a161) {
        Ok(Some(({
            let (h162, _) = {
                let (h163, _) = v2.clone();
                h163
            };
            h162
        }, {
            let (h164, _) = {
                let (_, h165) = {
                    let (h166, _) = v2;
                    h166
                };
                h165
            };
            h164
        })))
    } else {
        Ok(None::<(u64, Bytes)>)
    }
}

pub fn f12(v0: u64, v1: List<Adt2>, v2: Adt0) -> Adt1 {
    let m167 = v1;
    match m167.uncons() {
        None => {
            Adt1::C0(v0, 7u64, List::<Adt2>::nil(), v2)
        }
        Some((v3, v4)) => {
            let m168 = v3;
            match m168 {
                Adt2::C0(v5) => {
                    Adt1::C0(v0, 5u64, List::cons(Adt2::C0(Adt0::C6(std::rc::Rc::new(v2), std::rc::Rc::new(v5))), v4), Adt0::C5)
                }
                Adt2::C1(v6, v7) => {
                    Adt1::C0(v0, 5u64, List::cons(Adt2::C1(Bytes::lit(&[]), Adt0::C7(v6, std::rc::Rc::new(v2), std::rc::Rc::new(v7))), v4), Adt0::C5)
                }
            }
        }
    }
}

pub fn f13(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m172 = {
        let a169 = v0;
        let a170 = v1;
        let a171 = v2;
        slice_bytes(a169, a170, a171)
    };
    match m172 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f14(v0: Bytes, v1: u64, v2: List<Adt2>) -> R<Adt1> {
    let m173 = v2;
    match m173.uncons() {
        None => {
            Ok(f7(v1))
        }
        Some((v3, v4)) => {
            let m174 = v3;
            match m174 {
                Adt2::C0(v5) => {
                    Ok(f12({
                        let a175 = v1;
                        let a176 = 1u64;
                        nat_add(a175, a176)?
                    }, v4, Adt0::C3(std::rc::Rc::new(f19(v5, {
                        let a178 = {
                            let a177 = v0;
                            length_bytes(a177)
                        };
                        let a179 = 1u64;
                        nat_add(a178, a179)?
                    })))))
                }
                Adt2::C1(_, v7) => {
                    Ok(f12({
                        let a180 = v1;
                        let a181 = 1u64;
                        nat_add(a180, a181)?
                    }, v4, Adt0::C4(std::rc::Rc::new(f19(v7, {
                        let a183 = {
                            let a182 = v0;
                            length_bytes(a182)
                        };
                        let a184 = 1u64;
                        nat_add(a183, a184)?
                    })))))
                }
            }
        }
    }
}

pub fn f15(v0: Bytes, v1: u64) -> R<Option<(u64, Adt0)>> {
    let v2: bool = {
        let a185 = f6(v0.clone(), v1)?;
        let a186 = 45u64;
        nat_eq(a185, a186)
    };
    let v3: u64 = if v2 {
        let a187 = v1;
        let a188 = 1u64;
        nat_add(a187, a188)?
    } else {
        v1
    };
    let v4: u64 = f20(v0.clone(), v3)?;
    let a191 = 19u64;
    let a192 = {
        let a189 = v4;
        let a190 = v3;
        nat_sub(a189, a190)
    };
    if nat_lt(a191, a192) {
        Ok(None::<(u64, Adt0)>)
    } else {
        let m195 = f21(f13(v0, v3, {
            let a193 = v4;
            let a194 = v3;
            nat_sub(a193, a194)
        }))?;
        match m195 {
            None => {
                Ok(None::<(u64, Adt0)>)
            }
            Some(v5) => {
                if f22(v2, v5) {
                    Ok(Some((v4, Adt0::C1(v2, v5))))
                } else {
                    Ok(None::<(u64, Adt0)>)
                }
            }
        }
    }
}

pub fn f16(v0: Adt0, v1: Bytes) -> bool {
    let m196 = v0;
    match m196 {
        Adt0::C0(_) => {
            true
        }
        Adt0::C1(_, _) => {
            true
        }
        Adt0::C2(_) => {
            true
        }
        Adt0::C3(_) => {
            true
        }
        Adt0::C4(_) => {
            true
        }
        Adt0::C5 => {
            true
        }
        Adt0::C6(_, _) => {
            true
        }
        Adt0::C7(v10, _, _) => {
            f23(v10, v1)
        }
    }
}

pub fn f17(v0: Fn1, p201: u64, p202: (u64, (Bytes, u64))) -> R<((u64, (Bytes, u64)), bool)> {
    let m200 = (p201, p202);
    {
        let mut m200 = m200;
        loop {
            let step = {
                let (v1, v2) = m200;
                let m197 = v1;
                if m197 == 0 {
                    Err::<(u64, (u64, (Bytes, u64))), R<((u64, (Bytes, u64)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m197 - 1;
                    let m199 = {
                        let c198: Fn1 = v0.clone();
                        c198.apply(v2.clone())?
                    };
                    match m199 {
                        None => {
                            Err::<(u64, (u64, (Bytes, u64))), R<((u64, (Bytes, u64)), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (u64, (Bytes, u64))), R<((u64, (Bytes, u64)), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m200 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f18(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
    f24(v0, v1)
}

pub fn f19(v0: Adt0, v1: u64) -> Adt0 {
    let (_, h203) = {
        let (h204, _) = f25(Fn2::F26, v1, (v0, Adt0::C5));
        h204
    };
    h203
}

pub fn f20(v0: Bytes, v1: u64) -> R<u64> {
    let (h205, _) = f27(Fn3::F28(v0.clone()), {
        let a207 = {
            let a206 = v0;
            length_bytes(a206)
        };
        let a208 = 1u64;
        nat_add(a207, a208)?
    }, v1)?;
    Ok(h205)
}

pub fn f21(v0: Bytes) -> R<Option<u64>> {
    if f29(v0.clone())? {
        Ok(Some({
            let (_, h209) = {
                let (h210, _) = f30(Fn4::F31(v0.clone()), {
                    let a211 = v0;
                    length_bytes(a211)
                }, (0u64, 0u64))?;
                h210
            };
            h209
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f22(v0: bool, v1: u64) -> bool {
    if v0 {
        let a212 = 0u64;
        let a213 = v1;
        if nat_lt(a212, a213) {
            let a214 = v1;
            let a215 = 9223372036854775808u64;
            nat_le(a214, a215)
        } else {
            false
        }
    } else {
        let a216 = v1;
        let a217 = 9223372036854775807u64;
        nat_le(a216, a217)
    }
}

pub fn f23(v0: Bytes, v1: Bytes) -> bool {
    let a222 = {
        let a218 = v0;
        let a219 = v1;
        compare_bytes(a218, a219)
    };
    let a223 = {
        let a220 = Bytes::lit(&[]);
        let a221 = Bytes::lit(&[0x01]);
        compare_bytes(a220, a221)
    };
    equal(a222, a223)
}

pub fn f24(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
    let v2: u64 = {
        let (h224, _) = v1.clone();
        h224
    };
    let v3: Bytes = {
        let (h225, _) = {
            let (_, h226) = v1.clone();
            h226
        };
        h225
    };
    let a229 = {
        let (_, h227) = {
            let (_, h228) = v1;
            h228
        };
        h227
    };
    let a230 = 0u64;
    if nat_eq(a229, a230) {
        let v4: u64 = f6(v0.clone(), v2)?;
        let a231 = v4;
        let a232 = 34u64;
        if nat_eq(a231, a232) {
            Ok(Some(({
                let a233 = v2;
                let a234 = 1u64;
                nat_add(a233, a234)?
            }, (v3, 1u64))))
        } else {
            let a235 = v4;
            let a236 = 92u64;
            if nat_eq(a235, a236) {
                f32(v0, v2, v3)
            } else {
                let a237 = v4;
                let a238 = 32u64;
                if if nat_lt(a237, a238) {
                    true
                } else {
                    let a239 = v4;
                    let a240 = 256u64;
                    nat_eq(a239, a240)
                } {
                    Ok(Some((v2, (v3, 2u64))))
                } else {
                    Ok(Some(({
                        let a241 = v2;
                        let a242 = 1u64;
                        nat_add(a241, a242)?
                    }, ({
                        let a243 = v3;
                        let a244 = f13(v0, v2, 1u64);
                        append_bytes(a243, a244)
                    }, 0u64))))
                }
            }
        }
    } else {
        Ok(None::<(u64, (Bytes, u64))>)
    }
}

pub fn f25(v0: Fn2, p249: u64, p250: (Adt0, Adt0)) -> ((Adt0, Adt0), bool) {
    let m248 = (p249, p250);
    {
        let mut m248 = m248;
        loop {
            let step = {
                let (v1, v2) = m248;
                let m245 = v1;
                if m245 == 0 {
                    Err::<(u64, (Adt0, Adt0)), ((Adt0, Adt0), bool)>((v2, false))
                } else {
                    let v3 = m245 - 1;
                    let m247 = {
                        let c246: Fn2 = v0.clone();
                        c246.apply(v2.clone())
                    };
                    match m247 {
                        None => {
                            Err::<(u64, (Adt0, Adt0)), ((Adt0, Adt0), bool)>((v2, true))
                        }
                        Some(v4) => {
                            Ok::<(u64, (Adt0, Adt0)), ((Adt0, Adt0), bool)>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m248 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f26(v0: (Adt0, Adt0)) -> Option<(Adt0, Adt0)> {
    f33(v0)
}

pub fn f27(v0: Fn3, p255: u64, p256: u64) -> R<(u64, bool)> {
    let m254 = (p255, p256);
    {
        let mut m254 = m254;
        loop {
            let step = {
                let (v1, v2) = m254;
                let m251 = v1;
                if m251 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m251 - 1;
                    let m253 = {
                        let c252: Fn3 = v0.clone();
                        c252.apply(v2)?
                    };
                    match m253 {
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
                Ok(next) => { m254 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f28(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f34(v0, v1)
}

pub fn f29(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a257 = v0.clone();
        length_bytes(a257)
    };
    let a258 = 0u64;
    let a259 = v1;
    if nat_lt(a258, a259) {
        let a260 = f35(v0.clone())?;
        let a261 = v1;
        if nat_eq(a260, a261) {
            let a262 = 1u64;
            let a263 = v1;
            if nat_lt(a262, a263) {
                let m266 = {
                    let a264 = v0;
                    let a265 = 0u64;
                    index_bytes(a264, a265)
                };
                match m266 {
                    None => {
                        Ok(false)
                    }
                    Some(v2) => {
                        let a267 = v2;
                        let a268 = 48u8;
                        Ok(!equal(a267, a268))
                    }
                }
            } else {
                Ok(true)
            }
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f30(v0: Fn4, p273: u64, p274: (u64, u64)) -> R<((u64, u64), bool)> {
    let m272 = (p273, p274);
    {
        let mut m272 = m272;
        loop {
            let step = {
                let (v1, v2) = m272;
                let m269 = v1;
                if m269 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m269 - 1;
                    let m271 = {
                        let c270: Fn4 = v0.clone();
                        c270.apply(v2)?
                    };
                    match m271 {
                        None => {
                            Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (u64, u64)), R<((u64, u64), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m272 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f31(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f36(v0, v1)
}

pub fn f32(v0: Bytes, v1: u64, v2: Bytes) -> R<Option<(u64, (Bytes, u64))>> {
    let v3: u64 = f6(v0.clone(), {
        let a275 = v1;
        let a276 = 1u64;
        nat_add(a275, a276)?
    })?;
    let v4: Bytes = {
        let a277 = v3;
        let a278 = 117u64;
        if nat_eq(a277, a278) {
            f37(v0, v1)?
        } else {
            f38(v3)
        }
    };
    let a279 = v4.clone();
    let a280 = Bytes::lit(&[]);
    if equal(a279, a280) {
        Ok(Some((v1, (v2, 2u64))))
    } else {
        Ok(Some(({
            let a281 = v3;
            let a282 = 117u64;
            if nat_eq(a281, a282) {
                let a283 = v1;
                let a284 = 6u64;
                nat_add(a283, a284)?
            } else {
                let a285 = v1;
                let a286 = 2u64;
                nat_add(a285, a286)?
            }
        }, ({
            let a287 = v2;
            let a288 = v4;
            append_bytes(a287, a288)
        }, 0u64))))
    }
}

pub fn f33(v0: (Adt0, Adt0)) -> Option<(Adt0, Adt0)> {
    let m290 = {
        let (h289, _) = v0;
        h289
    };
    match m290 {
        Adt0::C0(_) => {
            None::<(Adt0, Adt0)>
        }
        Adt0::C1(_, _) => {
            None::<(Adt0, Adt0)>
        }
        Adt0::C2(_) => {
            None::<(Adt0, Adt0)>
        }
        Adt0::C3(_) => {
            None::<(Adt0, Adt0)>
        }
        Adt0::C4(_) => {
            None::<(Adt0, Adt0)>
        }
        Adt0::C5 => {
            None::<(Adt0, Adt0)>
        }
        Adt0::C6(r293, r294) => {
            let v7 = (*r293).clone();
            let v8 = (*r294).clone();
            Some((v8, Adt0::C6(std::rc::Rc::new(v7), std::rc::Rc::new({
                let (_, h291) = v0;
                h291
            }))))
        }
        Adt0::C7(v9, r295, r296) => {
            let v10 = (*r295).clone();
            let v11 = (*r296).clone();
            Some((v11, Adt0::C7(v9, std::rc::Rc::new(v10), std::rc::Rc::new({
                let (_, h292) = v0;
                h292
            }))))
        }
    }
}

pub fn f34(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f6(v0, v1)?;
    let a297 = 47u64;
    let a298 = v2;
    if if nat_lt(a297, a298) {
        let a299 = v2;
        let a300 = 58u64;
        nat_lt(a299, a300)
    } else {
        false
    } {
        Ok(Some({
            let a301 = v1;
            let a302 = 1u64;
            nat_add(a301, a302)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f35(v0: Bytes) -> R<u64> {
    let (h303, _) = f27(Fn3::F39(v0.clone()), {
        let a304 = v0;
        length_bytes(a304)
    }, 0u64)?;
    Ok(h303)
}

pub fn f36(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m308 = {
        let a306 = v0;
        let a307 = {
            let (h305, _) = v1;
            h305
        };
        index_bytes(a306, a307)
    };
    match m308 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a310 = {
                    let (h309, _) = v1;
                    h309
                };
                let a311 = 1u64;
                nat_add(a310, a311)?
            }, {
                let a315 = {
                    let a313 = {
                        let (_, h312) = v1;
                        h312
                    };
                    let a314 = 10u64;
                    nat_mul(a313, a314)?
                };
                let a316 = f40(v2)?;
                nat_add(a315, a316)?
            })))
        }
    }
}

pub fn f37(v0: Bytes, v1: u64) -> R<Bytes> {
    let a319 = f13(v0.clone(), {
        let a317 = v1;
        let a318 = 2u64;
        nat_add(a317, a318)?
    }, 2u64);
    let a320 = Bytes::lit(&[0x30, 0x30]);
    if equal(a319, a320) {
        let m323 = f41(f13(v0, {
            let a321 = v1;
            let a322 = 4u64;
            nat_add(a321, a322)?
        }, 2u64))?;
        match m323 {
            None => {
                Ok(Bytes::lit(&[]))
            }
            Some(v2) => {
                let v3: u64 = f6(v2.clone(), 0u64)?;
                let a324 = v3;
                let a325 = 32u64;
                if nat_lt(a324, a325) {
                    if f42(v3) {
                        Ok(Bytes::lit(&[]))
                    } else {
                        Ok(v2)
                    }
                } else {
                    Ok(Bytes::lit(&[]))
                }
            }
        }
    } else {
        Ok(Bytes::lit(&[]))
    }
}

pub fn f38(v0: u64) -> Bytes {
    let a326 = v0;
    let a327 = 34u64;
    if nat_eq(a326, a327) {
        Bytes::lit(&[0x22])
    } else {
        let a328 = v0;
        let a329 = 92u64;
        if nat_eq(a328, a329) {
            Bytes::lit(&[0x5c])
        } else {
            let a330 = v0;
            let a331 = 98u64;
            if nat_eq(a330, a331) {
                Bytes::lit(&[0x08])
            } else {
                let a332 = v0;
                let a333 = 102u64;
                if nat_eq(a332, a333) {
                    Bytes::lit(&[0x0c])
                } else {
                    let a334 = v0;
                    let a335 = 110u64;
                    if nat_eq(a334, a335) {
                        Bytes::lit(&[0x0a])
                    } else {
                        let a336 = v0;
                        let a337 = 114u64;
                        if nat_eq(a336, a337) {
                            Bytes::lit(&[0x0d])
                        } else {
                            let a338 = v0;
                            let a339 = 116u64;
                            if nat_eq(a338, a339) {
                                Bytes::lit(&[0x09])
                            } else {
                                Bytes::lit(&[])
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f39(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m342 = {
        let a340 = v0;
        let a341 = v1;
        index_bytes(a340, a341)
    };
    match m342 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f43(v2)? {
                Ok(Some({
                    let a343 = v1;
                    let a344 = 1u64;
                    nat_add(a343, a344)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f40(v0: u8) -> R<u64> {
    let a391 = {
        let v1: u8 = v0;
        let a389 = {
            let a365 = {
                let a353 = {
                    let a347 = {
                        let a345 = v1;
                        let a346 = 128u8;
                        fixed_u8::bit_and(a345, a346)
                    };
                    let a348 = 128u8;
                    if equal(a347, a348) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a354 = {
                    let a351 = {
                        let a349 = v1;
                        let a350 = 64u8;
                        fixed_u8::bit_and(a349, a350)
                    };
                    let a352 = 64u8;
                    if equal(a351, a352) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a353, a354)?
            };
            let a366 = {
                let a363 = {
                    let a357 = {
                        let a355 = v1;
                        let a356 = 32u8;
                        fixed_u8::bit_and(a355, a356)
                    };
                    let a358 = 32u8;
                    if equal(a357, a358) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a364 = {
                    let a361 = {
                        let a359 = v1;
                        let a360 = 16u8;
                        fixed_u8::bit_and(a359, a360)
                    };
                    let a362 = 16u8;
                    if equal(a361, a362) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a363, a364)?
            };
            nat_add(a365, a366)?
        };
        let a390 = {
            let a387 = {
                let a375 = {
                    let a369 = {
                        let a367 = v1;
                        let a368 = 8u8;
                        fixed_u8::bit_and(a367, a368)
                    };
                    let a370 = 8u8;
                    if equal(a369, a370) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a376 = {
                    let a373 = {
                        let a371 = v1;
                        let a372 = 4u8;
                        fixed_u8::bit_and(a371, a372)
                    };
                    let a374 = 4u8;
                    if equal(a373, a374) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a375, a376)?
            };
            let a388 = {
                let a385 = {
                    let a379 = {
                        let a377 = v1;
                        let a378 = 2u8;
                        fixed_u8::bit_and(a377, a378)
                    };
                    let a380 = 2u8;
                    if equal(a379, a380) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a386 = {
                    let a383 = {
                        let a381 = v1;
                        let a382 = 1u8;
                        fixed_u8::bit_and(a381, a382)
                    };
                    let a384 = 1u8;
                    if equal(a383, a384) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a385, a386)?
            };
            nat_add(a387, a388)?
        };
        nat_add(a389, a390)?
    };
    let a392 = 48u64;
    Ok(nat_sub(a391, a392))
}

pub fn f41(v0: Bytes) -> R<Option<Bytes>> {
    let v1: u64 = {
        let a393 = v0.clone();
        length_bytes(a393)
    };
    let a397 = {
        let a394 = v1;
        let a395 = 2u64;
        let a396 = 0u64;
        nat_rem(a394, a395, a396)
    };
    let a398 = 0u64;
    if nat_eq(a397, a398) {
        let (_, h399) = {
            let (h400, _) = f44(Fn5::F45(v0), v1, (0u64, Some(Bytes::lit(&[]))))?;
            h400
        };
        Ok(h399)
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f42(v0: u64) -> bool {
    let a401 = v0;
    let a402 = 8u64;
    if nat_eq(a401, a402) {
        true
    } else {
        let a403 = v0;
        let a404 = 9u64;
        if nat_eq(a403, a404) {
            true
        } else {
            let a405 = v0;
            let a406 = 10u64;
            if nat_eq(a405, a406) {
                true
            } else {
                let a407 = v0;
                let a408 = 12u64;
                if nat_eq(a407, a408) {
                    true
                } else {
                    let a409 = v0;
                    let a410 = 13u64;
                    nat_eq(a409, a410)
                }
            }
        }
    }
}

pub fn f43(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a455 = {
            let a431 = {
                let a419 = {
                    let a413 = {
                        let a411 = v1;
                        let a412 = 128u8;
                        fixed_u8::bit_and(a411, a412)
                    };
                    let a414 = 128u8;
                    if equal(a413, a414) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a420 = {
                    let a417 = {
                        let a415 = v1;
                        let a416 = 64u8;
                        fixed_u8::bit_and(a415, a416)
                    };
                    let a418 = 64u8;
                    if equal(a417, a418) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a419, a420)?
            };
            let a432 = {
                let a429 = {
                    let a423 = {
                        let a421 = v1;
                        let a422 = 32u8;
                        fixed_u8::bit_and(a421, a422)
                    };
                    let a424 = 32u8;
                    if equal(a423, a424) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a430 = {
                    let a427 = {
                        let a425 = v1;
                        let a426 = 16u8;
                        fixed_u8::bit_and(a425, a426)
                    };
                    let a428 = 16u8;
                    if equal(a427, a428) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a429, a430)?
            };
            nat_add(a431, a432)?
        };
        let a456 = {
            let a453 = {
                let a441 = {
                    let a435 = {
                        let a433 = v1;
                        let a434 = 8u8;
                        fixed_u8::bit_and(a433, a434)
                    };
                    let a436 = 8u8;
                    if equal(a435, a436) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a442 = {
                    let a439 = {
                        let a437 = v1;
                        let a438 = 4u8;
                        fixed_u8::bit_and(a437, a438)
                    };
                    let a440 = 4u8;
                    if equal(a439, a440) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a441, a442)?
            };
            let a454 = {
                let a451 = {
                    let a445 = {
                        let a443 = v1;
                        let a444 = 2u8;
                        fixed_u8::bit_and(a443, a444)
                    };
                    let a446 = 2u8;
                    if equal(a445, a446) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a452 = {
                    let a449 = {
                        let a447 = v1;
                        let a448 = 1u8;
                        fixed_u8::bit_and(a447, a448)
                    };
                    let a450 = 1u8;
                    if equal(a449, a450) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a451, a452)?
            };
            nat_add(a453, a454)?
        };
        nat_add(a455, a456)?
    };
    let a457 = 48u64;
    let a458 = v2;
    if nat_le(a457, a458) {
        let a459 = v2;
        let a460 = 57u64;
        Ok(nat_le(a459, a460))
    } else {
        Ok(false)
    }
}

pub fn f44(v0: Fn5, p465: u64, p466: (u64, Option<Bytes>)) -> R<((u64, Option<Bytes>), bool)> {
    let m464 = (p465, p466);
    {
        let mut m464 = m464;
        loop {
            let step = {
                let (v1, v2) = m464;
                let m461 = v1;
                if m461 == 0 {
                    Err::<(u64, (u64, Option<Bytes>)), R<((u64, Option<Bytes>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m461 - 1;
                    let m463 = {
                        let c462: Fn5 = v0.clone();
                        c462.apply(v2.clone())?
                    };
                    match m463 {
                        None => {
                            Err::<(u64, (u64, Option<Bytes>)), R<((u64, Option<Bytes>), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (u64, Option<Bytes>)), R<((u64, Option<Bytes>), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m464 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f45(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    f46(v0, v1)
}

pub fn f46(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    let v2: u64 = {
        let (h467, _) = v1;
        h467
    };
    let m469 = {
        let (_, h468) = v1;
        h468
    };
    match m469 {
        None => {
            Ok(None::<(u64, Option<Bytes>)>)
        }
        Some(v3) => {
            let m472 = {
                let a470 = v0.clone();
                let a471 = v2;
                index_bytes(a470, a471)
            };
            match m472 {
                None => {
                    Ok(None::<(u64, Option<Bytes>)>)
                }
                Some(v4) => {
                    let m477 = {
                        let a475 = v0;
                        let a476 = {
                            let a473 = v2;
                            let a474 = 1u64;
                            nat_add(a473, a474)?
                        };
                        index_bytes(a475, a476)
                    };
                    match m477 {
                        None => {
                            Ok(Some((v2, None::<Bytes>)))
                        }
                        Some(v5) => {
                            if if f47(v4)? {
                                f47(v5)?
                            } else {
                                false
                            } {
                                Ok(Some(({
                                    let a478 = v2;
                                    let a479 = 2u64;
                                    nat_add(a478, a479)?
                                }, Some({
                                    let a484 = v3;
                                    let a485 = f48(f49(), {
                                        let a482 = {
                                            let a480 = f50(v4)?;
                                            let a481 = 16u64;
                                            nat_mul(a480, a481)?
                                        };
                                        let a483 = f50(v5)?;
                                        nat_add(a482, a483)?
                                    });
                                    append_bytes(a484, a485)
                                }))))
                            } else {
                                Ok(Some((v2, None::<Bytes>)))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f47(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a530 = {
            let a506 = {
                let a494 = {
                    let a488 = {
                        let a486 = v1;
                        let a487 = 128u8;
                        fixed_u8::bit_and(a486, a487)
                    };
                    let a489 = 128u8;
                    if equal(a488, a489) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a495 = {
                    let a492 = {
                        let a490 = v1;
                        let a491 = 64u8;
                        fixed_u8::bit_and(a490, a491)
                    };
                    let a493 = 64u8;
                    if equal(a492, a493) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a494, a495)?
            };
            let a507 = {
                let a504 = {
                    let a498 = {
                        let a496 = v1;
                        let a497 = 32u8;
                        fixed_u8::bit_and(a496, a497)
                    };
                    let a499 = 32u8;
                    if equal(a498, a499) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a505 = {
                    let a502 = {
                        let a500 = v1;
                        let a501 = 16u8;
                        fixed_u8::bit_and(a500, a501)
                    };
                    let a503 = 16u8;
                    if equal(a502, a503) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a504, a505)?
            };
            nat_add(a506, a507)?
        };
        let a531 = {
            let a528 = {
                let a516 = {
                    let a510 = {
                        let a508 = v1;
                        let a509 = 8u8;
                        fixed_u8::bit_and(a508, a509)
                    };
                    let a511 = 8u8;
                    if equal(a510, a511) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a517 = {
                    let a514 = {
                        let a512 = v1;
                        let a513 = 4u8;
                        fixed_u8::bit_and(a512, a513)
                    };
                    let a515 = 4u8;
                    if equal(a514, a515) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a516, a517)?
            };
            let a529 = {
                let a526 = {
                    let a520 = {
                        let a518 = v1;
                        let a519 = 2u8;
                        fixed_u8::bit_and(a518, a519)
                    };
                    let a521 = 2u8;
                    if equal(a520, a521) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a527 = {
                    let a524 = {
                        let a522 = v1;
                        let a523 = 1u8;
                        fixed_u8::bit_and(a522, a523)
                    };
                    let a525 = 1u8;
                    if equal(a524, a525) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a526, a527)?
            };
            nat_add(a528, a529)?
        };
        nat_add(a530, a531)?
    };
    let a532 = 48u64;
    let a533 = v2;
    if if nat_le(a532, a533) {
        let a534 = v2;
        let a535 = 57u64;
        nat_le(a534, a535)
    } else {
        false
    } {
        Ok(true)
    } else {
        let a536 = 97u64;
        let a537 = v2;
        if nat_le(a536, a537) {
            let a538 = v2;
            let a539 = 102u64;
            Ok(nat_le(a538, a539))
        } else {
            Ok(false)
        }
    }
}

pub fn f48(v0: Bytes, v1: u64) -> Bytes {
    let m543 = {
        let a540 = v0;
        let a541 = v1;
        let a542 = 1u64;
        slice_bytes(a540, a541, a542)
    };
    match m543 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f49() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f50(v0: u8) -> R<u64> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a588 = {
            let a564 = {
                let a552 = {
                    let a546 = {
                        let a544 = v1;
                        let a545 = 128u8;
                        fixed_u8::bit_and(a544, a545)
                    };
                    let a547 = 128u8;
                    if equal(a546, a547) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a553 = {
                    let a550 = {
                        let a548 = v1;
                        let a549 = 64u8;
                        fixed_u8::bit_and(a548, a549)
                    };
                    let a551 = 64u8;
                    if equal(a550, a551) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a552, a553)?
            };
            let a565 = {
                let a562 = {
                    let a556 = {
                        let a554 = v1;
                        let a555 = 32u8;
                        fixed_u8::bit_and(a554, a555)
                    };
                    let a557 = 32u8;
                    if equal(a556, a557) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a563 = {
                    let a560 = {
                        let a558 = v1;
                        let a559 = 16u8;
                        fixed_u8::bit_and(a558, a559)
                    };
                    let a561 = 16u8;
                    if equal(a560, a561) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a562, a563)?
            };
            nat_add(a564, a565)?
        };
        let a589 = {
            let a586 = {
                let a574 = {
                    let a568 = {
                        let a566 = v1;
                        let a567 = 8u8;
                        fixed_u8::bit_and(a566, a567)
                    };
                    let a569 = 8u8;
                    if equal(a568, a569) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a575 = {
                    let a572 = {
                        let a570 = v1;
                        let a571 = 4u8;
                        fixed_u8::bit_and(a570, a571)
                    };
                    let a573 = 4u8;
                    if equal(a572, a573) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a574, a575)?
            };
            let a587 = {
                let a584 = {
                    let a578 = {
                        let a576 = v1;
                        let a577 = 2u8;
                        fixed_u8::bit_and(a576, a577)
                    };
                    let a579 = 2u8;
                    if equal(a578, a579) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a585 = {
                    let a582 = {
                        let a580 = v1;
                        let a581 = 1u8;
                        fixed_u8::bit_and(a580, a581)
                    };
                    let a583 = 1u8;
                    if equal(a582, a583) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a584, a585)?
            };
            nat_add(a586, a587)?
        };
        nat_add(a588, a589)?
    };
    let a590 = v2;
    let a591 = 57u64;
    if nat_le(a590, a591) {
        let a592 = v2;
        let a593 = 48u64;
        Ok(nat_sub(a592, a593))
    } else {
        let a594 = v2;
        let a595 = 87u64;
        Ok(nat_sub(a594, a595))
    }
}

pub fn run(p0: Bytes) -> R<Option<Adt0>> {
    f0(p0)
}
