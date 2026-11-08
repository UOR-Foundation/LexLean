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
    F5(Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: Adt1) -> R<Option<Adt1>> {
        match self {
            Fn0::F5(k0) => f5(k0.clone(), p0),
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
    let v1: u64 = {
        let a1 = v0.clone();
        length_bytes(a1)
    };
    let a4 = f1(v0.clone(), {
        let a2 = v1;
        let a3 = 1u64;
        nat_sub(a2, a3)
    })?;
    let a5 = 10u64;
    if nat_eq(a4, a5) {
        f2(f3(v0, 0u64, {
            let a6 = v1;
            let a7 = 1u64;
            nat_sub(a6, a7)
        }))
    } else {
        Ok(None::<Adt0>)
    }
}

pub fn f1(v0: Bytes, v1: u64) -> R<u64> {
    let m10 = {
        let a8 = v0;
        let a9 = v1;
        index_bytes(a8, a9)
    };
    match m10 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a55 = {
                let a31 = {
                    let a19 = {
                        let a13 = {
                            let a11 = v3;
                            let a12 = 128u8;
                            fixed_u8::bit_and(a11, a12)
                        };
                        let a14 = 128u8;
                        if equal(a13, a14) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a20 = {
                        let a17 = {
                            let a15 = v3;
                            let a16 = 64u8;
                            fixed_u8::bit_and(a15, a16)
                        };
                        let a18 = 64u8;
                        if equal(a17, a18) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a19, a20)?
                };
                let a32 = {
                    let a29 = {
                        let a23 = {
                            let a21 = v3;
                            let a22 = 32u8;
                            fixed_u8::bit_and(a21, a22)
                        };
                        let a24 = 32u8;
                        if equal(a23, a24) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a30 = {
                        let a27 = {
                            let a25 = v3;
                            let a26 = 16u8;
                            fixed_u8::bit_and(a25, a26)
                        };
                        let a28 = 16u8;
                        if equal(a27, a28) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a29, a30)?
                };
                nat_add(a31, a32)?
            };
            let a56 = {
                let a53 = {
                    let a41 = {
                        let a35 = {
                            let a33 = v3;
                            let a34 = 8u8;
                            fixed_u8::bit_and(a33, a34)
                        };
                        let a36 = 8u8;
                        if equal(a35, a36) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a42 = {
                        let a39 = {
                            let a37 = v3;
                            let a38 = 4u8;
                            fixed_u8::bit_and(a37, a38)
                        };
                        let a40 = 4u8;
                        if equal(a39, a40) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a41, a42)?
                };
                let a54 = {
                    let a51 = {
                        let a45 = {
                            let a43 = v3;
                            let a44 = 2u8;
                            fixed_u8::bit_and(a43, a44)
                        };
                        let a46 = 2u8;
                        if equal(a45, a46) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a52 = {
                        let a49 = {
                            let a47 = v3;
                            let a48 = 1u8;
                            fixed_u8::bit_and(a47, a48)
                        };
                        let a50 = 1u8;
                        if equal(a49, a50) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a51, a52)?
                };
                nat_add(a53, a54)?
            };
            nat_add(a55, a56)
        }
    }
}

pub fn f2(v0: Bytes) -> R<Option<Adt0>> {
    let m61 = {
        let (h57, _) = f4(Fn0::F5(v0.clone()), {
            let a59 = {
                let a58 = v0;
                length_bytes(a58)
            };
            let a60 = 3u64;
            nat_add(a59, a60)?
        }, Adt1::C0(0u64, 0u64, List::<Adt2>::nil(), Adt0::C5))?;
        h57
    };
    match m61 {
        Adt1::C0(_, v2, _, v4) => {
            let a62 = v2;
            let a63 = 8u64;
            if nat_eq(a62, a63) {
                Ok(Some(v4))
            } else {
                Ok(None::<Adt0>)
            }
        }
    }
}

pub fn f3(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m67 = {
        let a64 = v0;
        let a65 = v1;
        let a66 = v2;
        slice_bytes(a64, a65, a66)
    };
    match m67 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f4(v0: Fn0, p72: u64, p73: Adt1) -> R<(Adt1, bool)> {
    let m71 = (p72, p73);
    {
        let mut m71 = m71;
        loop {
            let step = {
                let (v1, v2) = m71;
                let m68 = v1;
                if m68 == 0 {
                    Err::<(u64, Adt1), R<(Adt1, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m68 - 1;
                    let m70 = {
                        let c69: Fn0 = v0.clone();
                        c69.apply(v2.clone())?
                    };
                    match m70 {
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
                Ok(next) => { m71 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f5(v0: Bytes, v1: Adt1) -> R<Option<Adt1>> {
    f6(v0, v1)
}

pub fn f6(v0: Bytes, v1: Adt1) -> R<Option<Adt1>> {
    let m74 = v1;
    match m74 {
        Adt1::C0(v2, v3, v4, v5) => {
            let a75 = 8u64;
            let a76 = v3;
            if nat_le(a75, a76) {
                Ok(None::<Adt1>)
            } else {
                Ok(Some(f7(v0, v2, v3, v4, v5)?))
            }
        }
    }
}

pub fn f7(v0: Bytes, v1: u64, v2: u64, v3: List<Adt2>, v4: Adt0) -> R<Adt1> {
    let v5: u64 = f1(v0.clone(), v1)?;
    let a77 = v2;
    let a78 = 7u64;
    if nat_eq(a77, a78) {
        Ok(Adt1::C0(v1, {
            let a79 = v5;
            let a80 = 256u64;
            if nat_eq(a79, a80) {
                8u64
            } else {
                9u64
            }
        }, v3, v4))
    } else {
        let a81 = v5;
        let a82 = 256u64;
        if nat_eq(a81, a82) {
            Ok(f8(v1))
        } else {
            let a83 = v2;
            let a84 = 0u64;
            if if nat_eq(a83, a84) {
                true
            } else {
                let a85 = v2;
                let a86 = 1u64;
                nat_eq(a85, a86)
            } {
                f9(v0, v1, v2, v3, v5)
            } else {
                let a87 = v2;
                let a88 = 2u64;
                if if nat_eq(a87, a88) {
                    true
                } else {
                    let a89 = v2;
                    let a90 = 3u64;
                    nat_eq(a89, a90)
                } {
                    f10(v0, v1, v2, v3, v5)
                } else {
                    let a91 = v2;
                    let a92 = 4u64;
                    if nat_eq(a91, a92) {
                        let a93 = v5;
                        let a94 = 58u64;
                        if nat_eq(a93, a94) {
                            Ok(Adt1::C0({
                                let a95 = v1;
                                let a96 = 1u64;
                                nat_add(a95, a96)?
                            }, 0u64, v3, v4))
                        } else {
                            Ok(f8(v1))
                        }
                    } else {
                        f11(v0, v1, v3, v5)
                    }
                }
            }
        }
    }
}

pub fn f8(v0: u64) -> Adt1 {
    Adt1::C0(v0, 9u64, List::<Adt2>::nil(), Adt0::C5)
}

pub fn f9(v0: Bytes, v1: u64, v2: u64, v3: List<Adt2>, v4: u64) -> R<Adt1> {
    let a97 = v4;
    let a98 = 34u64;
    if nat_eq(a97, a98) {
        let m101 = f12(v0, {
            let a99 = v1;
            let a100 = 1u64;
            nat_add(a99, a100)?
        })?;
        match m101 {
            None => {
                Ok(f8(v1))
            }
            Some(v5) => {
                Ok(f13({
                    let (h102, _) = v5;
                    h102
                }, v3, Adt0::C2({
                    let (_, h103) = v5;
                    h103
                })))
            }
        }
    } else {
        let a104 = v4;
        let a105 = 116u64;
        if nat_eq(a104, a105) {
            let a106 = f3(v0, v1, 4u64);
            let a107 = Bytes::lit(&[0x74, 0x72, 0x75, 0x65]);
            if equal(a106, a107) {
                Ok(f13({
                    let a108 = v1;
                    let a109 = 4u64;
                    nat_add(a108, a109)?
                }, v3, Adt0::C0(true)))
            } else {
                Ok(f8(v1))
            }
        } else {
            let a110 = v4;
            let a111 = 102u64;
            if nat_eq(a110, a111) {
                let a112 = f3(v0, v1, 5u64);
                let a113 = Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]);
                if equal(a112, a113) {
                    Ok(f13({
                        let a114 = v1;
                        let a115 = 5u64;
                        nat_add(a114, a115)?
                    }, v3, Adt0::C0(false)))
                } else {
                    Ok(f8(v1))
                }
            } else {
                let a116 = v4;
                let a117 = 91u64;
                if nat_eq(a116, a117) {
                    Ok(Adt1::C0({
                        let a118 = v1;
                        let a119 = 1u64;
                        nat_add(a118, a119)?
                    }, 1u64, List::cons(Adt2::C0(Adt0::C5), v3), Adt0::C5))
                } else {
                    let a120 = v4;
                    let a121 = 123u64;
                    if nat_eq(a120, a121) {
                        Ok(Adt1::C0({
                            let a122 = v1;
                            let a123 = 1u64;
                            nat_add(a122, a123)?
                        }, 3u64, List::cons(Adt2::C1(Bytes::lit(&[]), Adt0::C5), v3), Adt0::C5))
                    } else {
                        let a124 = v4;
                        let a125 = 93u64;
                        if if nat_eq(a124, a125) {
                            let a126 = v2;
                            let a127 = 1u64;
                            nat_eq(a126, a127)
                        } else {
                            false
                        } {
                            f14(v0, v1, v3)
                        } else {
                            let a128 = v4;
                            let a129 = 45u64;
                            if if nat_eq(a128, a129) {
                                true
                            } else {
                                let a130 = 47u64;
                                let a131 = v4;
                                if nat_lt(a130, a131) {
                                    let a132 = v4;
                                    let a133 = 58u64;
                                    nat_lt(a132, a133)
                                } else {
                                    false
                                }
                            } {
                                let m134 = f15(v0, v1)?;
                                match m134 {
                                    None => {
                                        Ok(f8(v1))
                                    }
                                    Some(v6) => {
                                        Ok(f13({
                                            let (h135, _) = v6;
                                            h135
                                        }, v3, {
                                            let (_, h136) = v6;
                                            h136
                                        }))
                                    }
                                }
                            } else {
                                Ok(f8(v1))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f10(v0: Bytes, v1: u64, v2: u64, v3: List<Adt2>, v4: u64) -> R<Adt1> {
    let a137 = v4;
    let a138 = 34u64;
    if nat_eq(a137, a138) {
        let m141 = f12(v0, {
            let a139 = v1;
            let a140 = 1u64;
            nat_add(a139, a140)?
        })?;
        match m141 {
            None => {
                Ok(f8(v1))
            }
            Some(v5) => {
                let m142 = v3;
                match m142.uncons() {
                    None => {
                        Ok(f8(v1))
                    }
                    Some((v6, v7)) => {
                        let m143 = v6;
                        match m143 {
                            Adt2::C0(_) => {
                                Ok(f8(v1))
                            }
                            Adt2::C1(_, v10) => {
                                if f16(v10.clone(), {
                                    let (_, h144) = v5.clone();
                                    h144
                                }) {
                                    Ok(Adt1::C0({
                                        let (h145, _) = v5;
                                        h145
                                    }, 4u64, List::cons(Adt2::C1({
                                        let (_, h146) = v5;
                                        h146
                                    }, v10), v7), Adt0::C5))
                                } else {
                                    Ok(f8(v1))
                                }
                            }
                        }
                    }
                }
            }
        }
    } else {
        let a147 = v4;
        let a148 = 125u64;
        if if nat_eq(a147, a148) {
            let a149 = v2;
            let a150 = 3u64;
            nat_eq(a149, a150)
        } else {
            false
        } {
            f14(v0, v1, v3)
        } else {
            Ok(f8(v1))
        }
    }
}

pub fn f11(v0: Bytes, v1: u64, v2: List<Adt2>, v3: u64) -> R<Adt1> {
    let m151 = v2.clone();
    match m151.uncons() {
        None => {
            Ok(f8(v1))
        }
        Some((v4, _)) => {
            let m152 = v4;
            match m152 {
                Adt2::C0(_) => {
                    let a153 = v3;
                    let a154 = 44u64;
                    if nat_eq(a153, a154) {
                        Ok(Adt1::C0({
                            let a155 = v1;
                            let a156 = 1u64;
                            nat_add(a155, a156)?
                        }, 0u64, v2, Adt0::C5))
                    } else {
                        let a157 = v3;
                        let a158 = 93u64;
                        if nat_eq(a157, a158) {
                            f14(v0, v1, v2)
                        } else {
                            Ok(f8(v1))
                        }
                    }
                }
                Adt2::C1(_, _) => {
                    let a159 = v3;
                    let a160 = 44u64;
                    if nat_eq(a159, a160) {
                        Ok(Adt1::C0({
                            let a161 = v1;
                            let a162 = 1u64;
                            nat_add(a161, a162)?
                        }, 2u64, v2, Adt0::C5))
                    } else {
                        let a163 = v3;
                        let a164 = 125u64;
                        if nat_eq(a163, a164) {
                            f14(v0, v1, v2)
                        } else {
                            Ok(f8(v1))
                        }
                    }
                }
            }
        }
    }
}

pub fn f12(v0: Bytes, v1: u64) -> R<Option<(u64, Bytes)>> {
    let v2: ((u64, (Bytes, u64)), bool) = f17(Fn1::F18(v0.clone()), {
        let a166 = {
            let a165 = v0;
            length_bytes(a165)
        };
        let a167 = 1u64;
        nat_add(a166, a167)?
    }, (v1, (Bytes::lit(&[]), 0u64)))?;
    let a171 = {
        let (_, h168) = {
            let (_, h169) = {
                let (h170, _) = v2.clone();
                h170
            };
            h169
        };
        h168
    };
    let a172 = 1u64;
    if nat_eq(a171, a172) {
        Ok(Some(({
            let (h173, _) = {
                let (h174, _) = v2.clone();
                h174
            };
            h173
        }, {
            let (h175, _) = {
                let (_, h176) = {
                    let (h177, _) = v2;
                    h177
                };
                h176
            };
            h175
        })))
    } else {
        Ok(None::<(u64, Bytes)>)
    }
}

pub fn f13(v0: u64, v1: List<Adt2>, v2: Adt0) -> Adt1 {
    let m178 = v1;
    match m178.uncons() {
        None => {
            Adt1::C0(v0, 7u64, List::<Adt2>::nil(), v2)
        }
        Some((v3, v4)) => {
            let m179 = v3;
            match m179 {
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

pub fn f14(v0: Bytes, v1: u64, v2: List<Adt2>) -> R<Adt1> {
    let m180 = v2;
    match m180.uncons() {
        None => {
            Ok(f8(v1))
        }
        Some((v3, v4)) => {
            let m181 = v3;
            match m181 {
                Adt2::C0(v5) => {
                    Ok(f13({
                        let a182 = v1;
                        let a183 = 1u64;
                        nat_add(a182, a183)?
                    }, v4, Adt0::C3(std::rc::Rc::new(f19(v5, {
                        let a185 = {
                            let a184 = v0;
                            length_bytes(a184)
                        };
                        let a186 = 1u64;
                        nat_add(a185, a186)?
                    })))))
                }
                Adt2::C1(_, v7) => {
                    Ok(f13({
                        let a187 = v1;
                        let a188 = 1u64;
                        nat_add(a187, a188)?
                    }, v4, Adt0::C4(std::rc::Rc::new(f19(v7, {
                        let a190 = {
                            let a189 = v0;
                            length_bytes(a189)
                        };
                        let a191 = 1u64;
                        nat_add(a190, a191)?
                    })))))
                }
            }
        }
    }
}

pub fn f15(v0: Bytes, v1: u64) -> R<Option<(u64, Adt0)>> {
    let v2: bool = {
        let a192 = f1(v0.clone(), v1)?;
        let a193 = 45u64;
        nat_eq(a192, a193)
    };
    let v3: u64 = if v2 {
        let a194 = v1;
        let a195 = 1u64;
        nat_add(a194, a195)?
    } else {
        v1
    };
    let v4: u64 = f20(v0.clone(), v3)?;
    let a198 = 19u64;
    let a199 = {
        let a196 = v4;
        let a197 = v3;
        nat_sub(a196, a197)
    };
    if nat_lt(a198, a199) {
        Ok(None::<(u64, Adt0)>)
    } else {
        let m202 = f21(f3(v0, v3, {
            let a200 = v4;
            let a201 = v3;
            nat_sub(a200, a201)
        }))?;
        match m202 {
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
    let m203 = v0;
    match m203 {
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

pub fn f17(v0: Fn1, p208: u64, p209: (u64, (Bytes, u64))) -> R<((u64, (Bytes, u64)), bool)> {
    let m207 = (p208, p209);
    {
        let mut m207 = m207;
        loop {
            let step = {
                let (v1, v2) = m207;
                let m204 = v1;
                if m204 == 0 {
                    Err::<(u64, (u64, (Bytes, u64))), R<((u64, (Bytes, u64)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m204 - 1;
                    let m206 = {
                        let c205: Fn1 = v0.clone();
                        c205.apply(v2.clone())?
                    };
                    match m206 {
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
                Ok(next) => { m207 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f18(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
    f24(v0, v1)
}

pub fn f19(v0: Adt0, v1: u64) -> Adt0 {
    let (_, h210) = {
        let (h211, _) = f25(Fn2::F26, v1, (v0, Adt0::C5));
        h211
    };
    h210
}

pub fn f20(v0: Bytes, v1: u64) -> R<u64> {
    let (h212, _) = f27(Fn3::F28(v0.clone()), {
        let a214 = {
            let a213 = v0;
            length_bytes(a213)
        };
        let a215 = 1u64;
        nat_add(a214, a215)?
    }, v1)?;
    Ok(h212)
}

pub fn f21(v0: Bytes) -> R<Option<u64>> {
    if f29(v0.clone())? {
        Ok(Some({
            let (_, h216) = {
                let (h217, _) = f30(Fn4::F31(v0.clone()), {
                    let a218 = v0;
                    length_bytes(a218)
                }, (0u64, 0u64))?;
                h217
            };
            h216
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f22(v0: bool, v1: u64) -> bool {
    if v0 {
        let a219 = 0u64;
        let a220 = v1;
        if nat_lt(a219, a220) {
            let a221 = v1;
            let a222 = 9223372036854775808u64;
            nat_le(a221, a222)
        } else {
            false
        }
    } else {
        let a223 = v1;
        let a224 = 9223372036854775807u64;
        nat_le(a223, a224)
    }
}

pub fn f23(v0: Bytes, v1: Bytes) -> bool {
    let a229 = {
        let a225 = v0;
        let a226 = v1;
        compare_bytes(a225, a226)
    };
    let a230 = {
        let a227 = Bytes::lit(&[]);
        let a228 = Bytes::lit(&[0x01]);
        compare_bytes(a227, a228)
    };
    equal(a229, a230)
}

pub fn f24(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
    let v2: u64 = {
        let (h231, _) = v1.clone();
        h231
    };
    let v3: Bytes = {
        let (h232, _) = {
            let (_, h233) = v1.clone();
            h233
        };
        h232
    };
    let a236 = {
        let (_, h234) = {
            let (_, h235) = v1;
            h235
        };
        h234
    };
    let a237 = 0u64;
    if nat_eq(a236, a237) {
        let v4: u64 = f1(v0.clone(), v2)?;
        let a238 = v4;
        let a239 = 34u64;
        if nat_eq(a238, a239) {
            Ok(Some(({
                let a240 = v2;
                let a241 = 1u64;
                nat_add(a240, a241)?
            }, (v3, 1u64))))
        } else {
            let a242 = v4;
            let a243 = 92u64;
            if nat_eq(a242, a243) {
                f32(v0, v2, v3)
            } else {
                let a244 = v4;
                let a245 = 32u64;
                if if nat_lt(a244, a245) {
                    true
                } else {
                    let a246 = v4;
                    let a247 = 256u64;
                    nat_eq(a246, a247)
                } {
                    Ok(Some((v2, (v3, 2u64))))
                } else {
                    Ok(Some(({
                        let a248 = v2;
                        let a249 = 1u64;
                        nat_add(a248, a249)?
                    }, ({
                        let a250 = v3;
                        let a251 = f3(v0, v2, 1u64);
                        append_bytes(a250, a251)
                    }, 0u64))))
                }
            }
        }
    } else {
        Ok(None::<(u64, (Bytes, u64))>)
    }
}

pub fn f25(v0: Fn2, p256: u64, p257: (Adt0, Adt0)) -> ((Adt0, Adt0), bool) {
    let m255 = (p256, p257);
    {
        let mut m255 = m255;
        loop {
            let step = {
                let (v1, v2) = m255;
                let m252 = v1;
                if m252 == 0 {
                    Err::<(u64, (Adt0, Adt0)), ((Adt0, Adt0), bool)>((v2, false))
                } else {
                    let v3 = m252 - 1;
                    let m254 = {
                        let c253: Fn2 = v0.clone();
                        c253.apply(v2.clone())
                    };
                    match m254 {
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
                Ok(next) => { m255 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f26(v0: (Adt0, Adt0)) -> Option<(Adt0, Adt0)> {
    f33(v0)
}

pub fn f27(v0: Fn3, p262: u64, p263: u64) -> R<(u64, bool)> {
    let m261 = (p262, p263);
    {
        let mut m261 = m261;
        loop {
            let step = {
                let (v1, v2) = m261;
                let m258 = v1;
                if m258 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m258 - 1;
                    let m260 = {
                        let c259: Fn3 = v0.clone();
                        c259.apply(v2)?
                    };
                    match m260 {
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
                Ok(next) => { m261 = next; }
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
        let a264 = v0.clone();
        length_bytes(a264)
    };
    let a265 = 0u64;
    let a266 = v1;
    if nat_lt(a265, a266) {
        let a267 = f35(v0.clone())?;
        let a268 = v1;
        if nat_eq(a267, a268) {
            let a269 = 1u64;
            let a270 = v1;
            if nat_lt(a269, a270) {
                let m273 = {
                    let a271 = v0;
                    let a272 = 0u64;
                    index_bytes(a271, a272)
                };
                match m273 {
                    None => {
                        Ok(false)
                    }
                    Some(v2) => {
                        let a274 = v2;
                        let a275 = 48u8;
                        Ok(!equal(a274, a275))
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

pub fn f30(v0: Fn4, p280: u64, p281: (u64, u64)) -> R<((u64, u64), bool)> {
    let m279 = (p280, p281);
    {
        let mut m279 = m279;
        loop {
            let step = {
                let (v1, v2) = m279;
                let m276 = v1;
                if m276 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m276 - 1;
                    let m278 = {
                        let c277: Fn4 = v0.clone();
                        c277.apply(v2)?
                    };
                    match m278 {
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
                Ok(next) => { m279 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f31(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f36(v0, v1)
}

pub fn f32(v0: Bytes, v1: u64, v2: Bytes) -> R<Option<(u64, (Bytes, u64))>> {
    let v3: u64 = f1(v0.clone(), {
        let a282 = v1;
        let a283 = 1u64;
        nat_add(a282, a283)?
    })?;
    let v4: Bytes = {
        let a284 = v3;
        let a285 = 117u64;
        if nat_eq(a284, a285) {
            f37(v0, v1)?
        } else {
            f38(v3)
        }
    };
    let a286 = v4.clone();
    let a287 = Bytes::lit(&[]);
    if equal(a286, a287) {
        Ok(Some((v1, (v2, 2u64))))
    } else {
        Ok(Some(({
            let a288 = v3;
            let a289 = 117u64;
            if nat_eq(a288, a289) {
                let a290 = v1;
                let a291 = 6u64;
                nat_add(a290, a291)?
            } else {
                let a292 = v1;
                let a293 = 2u64;
                nat_add(a292, a293)?
            }
        }, ({
            let a294 = v2;
            let a295 = v4;
            append_bytes(a294, a295)
        }, 0u64))))
    }
}

pub fn f33(v0: (Adt0, Adt0)) -> Option<(Adt0, Adt0)> {
    let m297 = {
        let (h296, _) = v0;
        h296
    };
    match m297 {
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
        Adt0::C6(r300, r301) => {
            let v7 = (*r300).clone();
            let v8 = (*r301).clone();
            Some((v8, Adt0::C6(std::rc::Rc::new(v7), std::rc::Rc::new({
                let (_, h298) = v0;
                h298
            }))))
        }
        Adt0::C7(v9, r302, r303) => {
            let v10 = (*r302).clone();
            let v11 = (*r303).clone();
            Some((v11, Adt0::C7(v9, std::rc::Rc::new(v10), std::rc::Rc::new({
                let (_, h299) = v0;
                h299
            }))))
        }
    }
}

pub fn f34(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f1(v0, v1)?;
    let a304 = 47u64;
    let a305 = v2;
    if if nat_lt(a304, a305) {
        let a306 = v2;
        let a307 = 58u64;
        nat_lt(a306, a307)
    } else {
        false
    } {
        Ok(Some({
            let a308 = v1;
            let a309 = 1u64;
            nat_add(a308, a309)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f35(v0: Bytes) -> R<u64> {
    let (h310, _) = f27(Fn3::F39(v0.clone()), {
        let a311 = v0;
        length_bytes(a311)
    }, 0u64)?;
    Ok(h310)
}

pub fn f36(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m315 = {
        let a313 = v0;
        let a314 = {
            let (h312, _) = v1;
            h312
        };
        index_bytes(a313, a314)
    };
    match m315 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a317 = {
                    let (h316, _) = v1;
                    h316
                };
                let a318 = 1u64;
                nat_add(a317, a318)?
            }, {
                let a322 = {
                    let a320 = {
                        let (_, h319) = v1;
                        h319
                    };
                    let a321 = 10u64;
                    nat_mul(a320, a321)?
                };
                let a323 = f40(v2)?;
                nat_add(a322, a323)?
            })))
        }
    }
}

pub fn f37(v0: Bytes, v1: u64) -> R<Bytes> {
    let a326 = f3(v0.clone(), {
        let a324 = v1;
        let a325 = 2u64;
        nat_add(a324, a325)?
    }, 2u64);
    let a327 = Bytes::lit(&[0x30, 0x30]);
    if equal(a326, a327) {
        let m330 = f41(f3(v0, {
            let a328 = v1;
            let a329 = 4u64;
            nat_add(a328, a329)?
        }, 2u64))?;
        match m330 {
            None => {
                Ok(Bytes::lit(&[]))
            }
            Some(v2) => {
                let v3: u64 = f1(v2.clone(), 0u64)?;
                let a331 = v3;
                let a332 = 32u64;
                if nat_lt(a331, a332) {
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
    let a333 = v0;
    let a334 = 34u64;
    if nat_eq(a333, a334) {
        Bytes::lit(&[0x22])
    } else {
        let a335 = v0;
        let a336 = 92u64;
        if nat_eq(a335, a336) {
            Bytes::lit(&[0x5c])
        } else {
            let a337 = v0;
            let a338 = 98u64;
            if nat_eq(a337, a338) {
                Bytes::lit(&[0x08])
            } else {
                let a339 = v0;
                let a340 = 102u64;
                if nat_eq(a339, a340) {
                    Bytes::lit(&[0x0c])
                } else {
                    let a341 = v0;
                    let a342 = 110u64;
                    if nat_eq(a341, a342) {
                        Bytes::lit(&[0x0a])
                    } else {
                        let a343 = v0;
                        let a344 = 114u64;
                        if nat_eq(a343, a344) {
                            Bytes::lit(&[0x0d])
                        } else {
                            let a345 = v0;
                            let a346 = 116u64;
                            if nat_eq(a345, a346) {
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
    let m349 = {
        let a347 = v0;
        let a348 = v1;
        index_bytes(a347, a348)
    };
    match m349 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f43(v2)? {
                Ok(Some({
                    let a350 = v1;
                    let a351 = 1u64;
                    nat_add(a350, a351)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f40(v0: u8) -> R<u64> {
    let a398 = {
        let v1: u8 = v0;
        let a396 = {
            let a372 = {
                let a360 = {
                    let a354 = {
                        let a352 = v1;
                        let a353 = 128u8;
                        fixed_u8::bit_and(a352, a353)
                    };
                    let a355 = 128u8;
                    if equal(a354, a355) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a361 = {
                    let a358 = {
                        let a356 = v1;
                        let a357 = 64u8;
                        fixed_u8::bit_and(a356, a357)
                    };
                    let a359 = 64u8;
                    if equal(a358, a359) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a360, a361)?
            };
            let a373 = {
                let a370 = {
                    let a364 = {
                        let a362 = v1;
                        let a363 = 32u8;
                        fixed_u8::bit_and(a362, a363)
                    };
                    let a365 = 32u8;
                    if equal(a364, a365) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a371 = {
                    let a368 = {
                        let a366 = v1;
                        let a367 = 16u8;
                        fixed_u8::bit_and(a366, a367)
                    };
                    let a369 = 16u8;
                    if equal(a368, a369) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a370, a371)?
            };
            nat_add(a372, a373)?
        };
        let a397 = {
            let a394 = {
                let a382 = {
                    let a376 = {
                        let a374 = v1;
                        let a375 = 8u8;
                        fixed_u8::bit_and(a374, a375)
                    };
                    let a377 = 8u8;
                    if equal(a376, a377) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a383 = {
                    let a380 = {
                        let a378 = v1;
                        let a379 = 4u8;
                        fixed_u8::bit_and(a378, a379)
                    };
                    let a381 = 4u8;
                    if equal(a380, a381) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a382, a383)?
            };
            let a395 = {
                let a392 = {
                    let a386 = {
                        let a384 = v1;
                        let a385 = 2u8;
                        fixed_u8::bit_and(a384, a385)
                    };
                    let a387 = 2u8;
                    if equal(a386, a387) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a393 = {
                    let a390 = {
                        let a388 = v1;
                        let a389 = 1u8;
                        fixed_u8::bit_and(a388, a389)
                    };
                    let a391 = 1u8;
                    if equal(a390, a391) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a392, a393)?
            };
            nat_add(a394, a395)?
        };
        nat_add(a396, a397)?
    };
    let a399 = 48u64;
    Ok(nat_sub(a398, a399))
}

pub fn f41(v0: Bytes) -> R<Option<Bytes>> {
    let v1: u64 = {
        let a400 = v0.clone();
        length_bytes(a400)
    };
    let a404 = {
        let a401 = v1;
        let a402 = 2u64;
        let a403 = 0u64;
        nat_rem(a401, a402, a403)
    };
    let a405 = 0u64;
    if nat_eq(a404, a405) {
        let (_, h406) = {
            let (h407, _) = f44(Fn5::F45(v0), v1, (0u64, Some(Bytes::lit(&[]))))?;
            h407
        };
        Ok(h406)
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f42(v0: u64) -> bool {
    let a408 = v0;
    let a409 = 8u64;
    if nat_eq(a408, a409) {
        true
    } else {
        let a410 = v0;
        let a411 = 9u64;
        if nat_eq(a410, a411) {
            true
        } else {
            let a412 = v0;
            let a413 = 10u64;
            if nat_eq(a412, a413) {
                true
            } else {
                let a414 = v0;
                let a415 = 12u64;
                if nat_eq(a414, a415) {
                    true
                } else {
                    let a416 = v0;
                    let a417 = 13u64;
                    nat_eq(a416, a417)
                }
            }
        }
    }
}

pub fn f43(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a462 = {
            let a438 = {
                let a426 = {
                    let a420 = {
                        let a418 = v1;
                        let a419 = 128u8;
                        fixed_u8::bit_and(a418, a419)
                    };
                    let a421 = 128u8;
                    if equal(a420, a421) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a427 = {
                    let a424 = {
                        let a422 = v1;
                        let a423 = 64u8;
                        fixed_u8::bit_and(a422, a423)
                    };
                    let a425 = 64u8;
                    if equal(a424, a425) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a426, a427)?
            };
            let a439 = {
                let a436 = {
                    let a430 = {
                        let a428 = v1;
                        let a429 = 32u8;
                        fixed_u8::bit_and(a428, a429)
                    };
                    let a431 = 32u8;
                    if equal(a430, a431) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a437 = {
                    let a434 = {
                        let a432 = v1;
                        let a433 = 16u8;
                        fixed_u8::bit_and(a432, a433)
                    };
                    let a435 = 16u8;
                    if equal(a434, a435) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a436, a437)?
            };
            nat_add(a438, a439)?
        };
        let a463 = {
            let a460 = {
                let a448 = {
                    let a442 = {
                        let a440 = v1;
                        let a441 = 8u8;
                        fixed_u8::bit_and(a440, a441)
                    };
                    let a443 = 8u8;
                    if equal(a442, a443) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a449 = {
                    let a446 = {
                        let a444 = v1;
                        let a445 = 4u8;
                        fixed_u8::bit_and(a444, a445)
                    };
                    let a447 = 4u8;
                    if equal(a446, a447) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a448, a449)?
            };
            let a461 = {
                let a458 = {
                    let a452 = {
                        let a450 = v1;
                        let a451 = 2u8;
                        fixed_u8::bit_and(a450, a451)
                    };
                    let a453 = 2u8;
                    if equal(a452, a453) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a459 = {
                    let a456 = {
                        let a454 = v1;
                        let a455 = 1u8;
                        fixed_u8::bit_and(a454, a455)
                    };
                    let a457 = 1u8;
                    if equal(a456, a457) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a458, a459)?
            };
            nat_add(a460, a461)?
        };
        nat_add(a462, a463)?
    };
    let a464 = 48u64;
    let a465 = v2;
    if nat_le(a464, a465) {
        let a466 = v2;
        let a467 = 57u64;
        Ok(nat_le(a466, a467))
    } else {
        Ok(false)
    }
}

pub fn f44(v0: Fn5, p472: u64, p473: (u64, Option<Bytes>)) -> R<((u64, Option<Bytes>), bool)> {
    let m471 = (p472, p473);
    {
        let mut m471 = m471;
        loop {
            let step = {
                let (v1, v2) = m471;
                let m468 = v1;
                if m468 == 0 {
                    Err::<(u64, (u64, Option<Bytes>)), R<((u64, Option<Bytes>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m468 - 1;
                    let m470 = {
                        let c469: Fn5 = v0.clone();
                        c469.apply(v2.clone())?
                    };
                    match m470 {
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
                Ok(next) => { m471 = next; }
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
        let (h474, _) = v1;
        h474
    };
    let m476 = {
        let (_, h475) = v1;
        h475
    };
    match m476 {
        None => {
            Ok(None::<(u64, Option<Bytes>)>)
        }
        Some(v3) => {
            let m479 = {
                let a477 = v0.clone();
                let a478 = v2;
                index_bytes(a477, a478)
            };
            match m479 {
                None => {
                    Ok(None::<(u64, Option<Bytes>)>)
                }
                Some(v4) => {
                    let m484 = {
                        let a482 = v0;
                        let a483 = {
                            let a480 = v2;
                            let a481 = 1u64;
                            nat_add(a480, a481)?
                        };
                        index_bytes(a482, a483)
                    };
                    match m484 {
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
                                    let a485 = v2;
                                    let a486 = 2u64;
                                    nat_add(a485, a486)?
                                }, Some({
                                    let a491 = v3;
                                    let a492 = f48(f49(), {
                                        let a489 = {
                                            let a487 = f50(v4)?;
                                            let a488 = 16u64;
                                            nat_mul(a487, a488)?
                                        };
                                        let a490 = f50(v5)?;
                                        nat_add(a489, a490)?
                                    });
                                    append_bytes(a491, a492)
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
        let a537 = {
            let a513 = {
                let a501 = {
                    let a495 = {
                        let a493 = v1;
                        let a494 = 128u8;
                        fixed_u8::bit_and(a493, a494)
                    };
                    let a496 = 128u8;
                    if equal(a495, a496) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a502 = {
                    let a499 = {
                        let a497 = v1;
                        let a498 = 64u8;
                        fixed_u8::bit_and(a497, a498)
                    };
                    let a500 = 64u8;
                    if equal(a499, a500) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a501, a502)?
            };
            let a514 = {
                let a511 = {
                    let a505 = {
                        let a503 = v1;
                        let a504 = 32u8;
                        fixed_u8::bit_and(a503, a504)
                    };
                    let a506 = 32u8;
                    if equal(a505, a506) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a512 = {
                    let a509 = {
                        let a507 = v1;
                        let a508 = 16u8;
                        fixed_u8::bit_and(a507, a508)
                    };
                    let a510 = 16u8;
                    if equal(a509, a510) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a511, a512)?
            };
            nat_add(a513, a514)?
        };
        let a538 = {
            let a535 = {
                let a523 = {
                    let a517 = {
                        let a515 = v1;
                        let a516 = 8u8;
                        fixed_u8::bit_and(a515, a516)
                    };
                    let a518 = 8u8;
                    if equal(a517, a518) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a524 = {
                    let a521 = {
                        let a519 = v1;
                        let a520 = 4u8;
                        fixed_u8::bit_and(a519, a520)
                    };
                    let a522 = 4u8;
                    if equal(a521, a522) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a523, a524)?
            };
            let a536 = {
                let a533 = {
                    let a527 = {
                        let a525 = v1;
                        let a526 = 2u8;
                        fixed_u8::bit_and(a525, a526)
                    };
                    let a528 = 2u8;
                    if equal(a527, a528) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a534 = {
                    let a531 = {
                        let a529 = v1;
                        let a530 = 1u8;
                        fixed_u8::bit_and(a529, a530)
                    };
                    let a532 = 1u8;
                    if equal(a531, a532) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a533, a534)?
            };
            nat_add(a535, a536)?
        };
        nat_add(a537, a538)?
    };
    let a539 = 48u64;
    let a540 = v2;
    if if nat_le(a539, a540) {
        let a541 = v2;
        let a542 = 57u64;
        nat_le(a541, a542)
    } else {
        false
    } {
        Ok(true)
    } else {
        let a543 = 97u64;
        let a544 = v2;
        if nat_le(a543, a544) {
            let a545 = v2;
            let a546 = 102u64;
            Ok(nat_le(a545, a546))
        } else {
            Ok(false)
        }
    }
}

pub fn f48(v0: Bytes, v1: u64) -> Bytes {
    let m550 = {
        let a547 = v0;
        let a548 = v1;
        let a549 = 1u64;
        slice_bytes(a547, a548, a549)
    };
    match m550 {
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
        let a595 = {
            let a571 = {
                let a559 = {
                    let a553 = {
                        let a551 = v1;
                        let a552 = 128u8;
                        fixed_u8::bit_and(a551, a552)
                    };
                    let a554 = 128u8;
                    if equal(a553, a554) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a560 = {
                    let a557 = {
                        let a555 = v1;
                        let a556 = 64u8;
                        fixed_u8::bit_and(a555, a556)
                    };
                    let a558 = 64u8;
                    if equal(a557, a558) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a559, a560)?
            };
            let a572 = {
                let a569 = {
                    let a563 = {
                        let a561 = v1;
                        let a562 = 32u8;
                        fixed_u8::bit_and(a561, a562)
                    };
                    let a564 = 32u8;
                    if equal(a563, a564) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a570 = {
                    let a567 = {
                        let a565 = v1;
                        let a566 = 16u8;
                        fixed_u8::bit_and(a565, a566)
                    };
                    let a568 = 16u8;
                    if equal(a567, a568) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a569, a570)?
            };
            nat_add(a571, a572)?
        };
        let a596 = {
            let a593 = {
                let a581 = {
                    let a575 = {
                        let a573 = v1;
                        let a574 = 8u8;
                        fixed_u8::bit_and(a573, a574)
                    };
                    let a576 = 8u8;
                    if equal(a575, a576) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a582 = {
                    let a579 = {
                        let a577 = v1;
                        let a578 = 4u8;
                        fixed_u8::bit_and(a577, a578)
                    };
                    let a580 = 4u8;
                    if equal(a579, a580) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a581, a582)?
            };
            let a594 = {
                let a591 = {
                    let a585 = {
                        let a583 = v1;
                        let a584 = 2u8;
                        fixed_u8::bit_and(a583, a584)
                    };
                    let a586 = 2u8;
                    if equal(a585, a586) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a592 = {
                    let a589 = {
                        let a587 = v1;
                        let a588 = 1u8;
                        fixed_u8::bit_and(a587, a588)
                    };
                    let a590 = 1u8;
                    if equal(a589, a590) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a591, a592)?
            };
            nat_add(a593, a594)?
        };
        nat_add(a595, a596)?
    };
    let a597 = v2;
    let a598 = 57u64;
    if nat_le(a597, a598) {
        let a599 = v2;
        let a600 = 48u64;
        Ok(nat_sub(a599, a600))
    } else {
        let a601 = v2;
        let a602 = 87u64;
        Ok(nat_sub(a601, a602))
    }
}

pub fn run(p0: Bytes) -> R<Option<Adt0>> {
    f0(p0)
}
