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
pub enum Fn0 {
    F10(Bytes),
    F26(u64, Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn0::F10(k0) => f10(k0.clone(), p0),
            Fn0::F26(k0, k1) => Ok(f26(*k0, k1.clone(), p0)),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F13,
    F48,
}

impl Fn1 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn1::F13 => f13(p0, p1),
            Fn1::F48 => f48(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F16,
}

impl Fn2 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, Bytes)) -> R<List<Bytes>> {
        match self {
            Fn2::F16 => f16(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F21(Bytes),
}

impl Fn3 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Bytes) -> Option<Bytes> {
        match self {
            Fn3::F21(k0) => f21(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F30,
}

impl Fn4 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn4::F30 => f30(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F32,
}

impl Fn5 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
        match self {
            Fn5::F32 => f32(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F37,
}

impl Fn6 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn6::F37 => f37(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F42,
}

impl Fn7 {
    pub fn apply(&self, p0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
        match self {
            Fn7::F42 => f42(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F47,
}

impl Fn8 {
    pub fn apply(&self, p0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
        match self {
            Fn8::F47 => f47(p0),
        }
    }
}

pub fn f0(v0: Adt0) -> R<Bytes> {
    let a1 = f1(v0)?;
    let a2 = Bytes::lit(&[0x0a]);
    Ok(append_bytes(a1, a2))
}

pub fn f1(v0: Adt0) -> R<Bytes> {
    let m3 = v0;
    match m3 {
        Adt0::C0(v1) => {
            if v1 {
                Ok(Bytes::lit(&[0x74, 0x72, 0x75, 0x65]))
            } else {
                Ok(Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]))
            }
        }
        Adt0::C1(v2, v3) => {
            f2(v2, v3)
        }
        Adt0::C2(v4) => {
            f3(v4)
        }
        Adt0::C3(r4) => {
            let v5 = (*r4).clone();
            Ok(f4(f5(v5)?))
        }
        Adt0::C4(r5) => {
            let v6 = (*r5).clone();
            f6(f7(v6)?)
        }
        Adt0::C5 => {
            Ok(Bytes::lit(&[]))
        }
        Adt0::C6(_, _) => {
            Ok(Bytes::lit(&[]))
        }
        Adt0::C7(_, _, _) => {
            Ok(Bytes::lit(&[]))
        }
    }
}

pub fn f2(v0: bool, v1: u64) -> R<Bytes> {
    if if v0 {
        let a6 = 0u64;
        let a7 = v1;
        nat_lt(a6, a7)
    } else {
        false
    } {
        let a8 = Bytes::lit(&[0x2d]);
        let a9 = f8(v1)?;
        Ok(append_bytes(a8, a9))
    } else {
        f8(v1)
    }
}

pub fn f3(v0: Bytes) -> R<Bytes> {
    let a15 = {
        let a13 = Bytes::lit(&[0x22]);
        let a14 = {
            let (_, h10) = {
                let (h11, _) = f9(Fn0::F10(v0.clone()), {
                    let a12 = v0;
                    length_bytes(a12)
                }, (0u64, Bytes::lit(&[])))?;
                h11
            };
            h10
        };
        append_bytes(a13, a14)
    };
    let a16 = Bytes::lit(&[0x22]);
    Ok(append_bytes(a15, a16))
}

pub fn f4(v0: List<Bytes>) -> Bytes {
    let a19 = {
        let a17 = Bytes::lit(&[0x5b]);
        let a18 = f11(v0, Bytes::lit(&[0x2c]));
        append_bytes(a17, a18)
    };
    let a20 = Bytes::lit(&[0x5d]);
    append_bytes(a19, a20)
}

pub fn f5(v0: Adt0) -> R<List<Bytes>> {
    let m21 = v0;
    match m21 {
        Adt0::C0(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt0::C1(_, _) => {
            Ok(List::<Bytes>::nil())
        }
        Adt0::C2(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt0::C3(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt0::C4(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt0::C5 => {
            Ok(List::<Bytes>::nil())
        }
        Adt0::C6(r22, r23) => {
            let v7 = (*r22).clone();
            let v8 = (*r23).clone();
            Ok(List::cons(f1(v7)?, f5(v8)?))
        }
        Adt0::C7(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f6(v0: List<(Bytes, Bytes)>) -> R<Bytes> {
    let v1: List<(Bytes, Bytes)> = f12(Fn1::F13, List::<(Bytes, Bytes)>::nil(), f14(v0)?);
    let v2: List<Bytes> = f15(Fn2::F16, List::<Bytes>::nil(), v1)?;
    let a26 = {
        let a24 = Bytes::lit(&[0x7b]);
        let a25 = f11(v2, Bytes::lit(&[0x2c]));
        append_bytes(a24, a25)
    };
    let a27 = Bytes::lit(&[0x7d]);
    Ok(append_bytes(a26, a27))
}

pub fn f7(v0: Adt0) -> R<List<(Bytes, Bytes)>> {
    let m28 = v0;
    match m28 {
        Adt0::C0(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt0::C1(_, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt0::C2(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt0::C3(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt0::C4(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt0::C5 => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt0::C6(_, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt0::C7(v9, r29, r30) => {
            let v10 = (*r29).clone();
            let v11 = (*r30).clone();
            Ok(List::cons((v9, f1(v10)?), f7(v11)?))
        }
    }
}

pub fn f8(v0: u64) -> R<Bytes> {
    f17(f18(), 10u64, v0)
}

pub fn f9(v0: Fn0, p35: u64, p36: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m34 = (p35, p36);
    {
        let mut m34 = m34;
        loop {
            let step = {
                let (v1, v2) = m34;
                let m31 = v1;
                if m31 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m31 - 1;
                    let m33 = {
                        let c32: Fn0 = v0.clone();
                        c32.apply(v2.clone())?
                    };
                    match m33 {
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
                Ok(next) => { m34 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f10(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f19(v0, v1)
}

pub fn f11(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m37 = f20(Fn3::F21(v1), None::<Bytes>, v0);
    match m37 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f12(v0: Fn1, p41: List<(Bytes, Bytes)>, p42: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m40 = (p41, p42);
    {
        let mut m40 = m40;
        loop {
            let step = {
                let (v1, v2) = m40;
                let m38 = v2;
                match m38.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c39: Fn1 = v0.clone();
                            c39.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m40 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f13(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    f22(v0, v1)
}

pub fn f14(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    f23(v0)
}

pub fn f15(v0: Fn2, p46: List<Bytes>, p47: List<(Bytes, Bytes)>) -> R<List<Bytes>> {
    let m45 = (p46, p47);
    {
        let mut m45 = m45;
        loop {
            let step = {
                let (v1, v2) = m45;
                let m43 = v2;
                match m43.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(({
                            let c44: Fn2 = v0.clone();
                            c44.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m45 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f16(v0: List<Bytes>, v1: (Bytes, Bytes)) -> R<List<Bytes>> {
    Ok(List::cons(f24(v1)?, v0))
}

pub fn f17(v0: Bytes, v1: u64, v2: u64) -> R<Bytes> {
    let a48 = v2;
    let a49 = 0u64;
    if nat_eq(a48, a49) {
        Ok(f25(v0, 0u64))
    } else {
        let (_, h50) = {
            let (h51, _) = f9(Fn0::F26(v1, v0), v2, (v2, Bytes::lit(&[])))?;
            h51
        };
        Ok(h50)
    }
}

pub fn f18() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f19(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let m55 = {
        let a53 = v0.clone();
        let a54 = {
            let (h52, _) = v1.clone();
            h52
        };
        index_bytes(a53, a54)
    };
    match m55 {
        None => {
            Ok(None::<(u64, Bytes)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a57 = {
                    let (h56, _) = v1.clone();
                    h56
                };
                let a58 = 1u64;
                nat_add(a57, a58)?
            }, {
                let a61 = {
                    let (_, h59) = v1;
                    h59
                };
                let a62 = f27(v0, {
                    let (h60, _) = v1;
                    h60
                }, v2)?;
                append_bytes(a61, a62)
            })))
        }
    }
}

pub fn f20(v0: Fn3, p66: Option<Bytes>, p67: List<Bytes>) -> Option<Bytes> {
    let m65 = (p66, p67);
    {
        let mut m65 = m65;
        loop {
            let step = {
                let (v1, v2) = m65;
                let m63 = v2;
                match m63.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(({
                            let c64: Fn3 = v0.clone();
                            c64.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m65 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f21(v0: Bytes, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m68 = v1;
    match m68 {
        None => {
            Some(v2)
        }
        Some(v3) => {
            Some({
                let a71 = {
                    let a69 = v3;
                    let a70 = v0;
                    append_bytes(a69, a70)
                };
                let a72 = v2;
                append_bytes(a71, a72)
            })
        }
    }
}

pub fn f22(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    let m73 = v0.clone();
    match m73.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, v3)) => {
            let a76 = {
                let (h74, _) = v2;
                h74
            };
            let a77 = {
                let (h75, _) = v1.clone();
                h75
            };
            if equal(a76, a77) {
                List::cons(v1, v3)
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f23(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v1: List<List<(Bytes, Bytes)>> = f28(f29(Fn4::F30, List::<List<(Bytes, Bytes)>>::nil(), v0.clone()));
    let m82 = {
        let (h78, _) = f31(Fn5::F32, {
            let a80 = {
                let a79 = v0;
                length_list(a79)
            };
            let a81 = 1u64;
            nat_add(a80, a81)?
        }, v1)?;
        h78
    };
    match m82.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f24(v0: (Bytes, Bytes)) -> R<Bytes> {
    let a87 = {
        let a84 = f3({
            let (h83, _) = v0;
            h83
        })?;
        let a85 = Bytes::lit(&[0x3a]);
        append_bytes(a84, a85)
    };
    let a88 = {
        let (_, h86) = v0;
        h86
    };
    Ok(append_bytes(a87, a88))
}

pub fn f25(v0: Bytes, v1: u64) -> Bytes {
    let m92 = {
        let a89 = v0;
        let a90 = v1;
        let a91 = 1u64;
        slice_bytes(a89, a90, a91)
    };
    match m92 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f26(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f33(v1, v0, v2)
}

pub fn f27(v0: Bytes, v1: u64, v2: u8) -> R<Bytes> {
    let v4: u64 = {
        let v3: u8 = v2;
        let a137 = {
            let a113 = {
                let a101 = {
                    let a95 = {
                        let a93 = v3;
                        let a94 = 128u8;
                        fixed_u8::bit_and(a93, a94)
                    };
                    let a96 = 128u8;
                    if equal(a95, a96) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a102 = {
                    let a99 = {
                        let a97 = v3;
                        let a98 = 64u8;
                        fixed_u8::bit_and(a97, a98)
                    };
                    let a100 = 64u8;
                    if equal(a99, a100) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a101, a102)?
            };
            let a114 = {
                let a111 = {
                    let a105 = {
                        let a103 = v3;
                        let a104 = 32u8;
                        fixed_u8::bit_and(a103, a104)
                    };
                    let a106 = 32u8;
                    if equal(a105, a106) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a112 = {
                    let a109 = {
                        let a107 = v3;
                        let a108 = 16u8;
                        fixed_u8::bit_and(a107, a108)
                    };
                    let a110 = 16u8;
                    if equal(a109, a110) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a111, a112)?
            };
            nat_add(a113, a114)?
        };
        let a138 = {
            let a135 = {
                let a123 = {
                    let a117 = {
                        let a115 = v3;
                        let a116 = 8u8;
                        fixed_u8::bit_and(a115, a116)
                    };
                    let a118 = 8u8;
                    if equal(a117, a118) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a124 = {
                    let a121 = {
                        let a119 = v3;
                        let a120 = 4u8;
                        fixed_u8::bit_and(a119, a120)
                    };
                    let a122 = 4u8;
                    if equal(a121, a122) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a123, a124)?
            };
            let a136 = {
                let a133 = {
                    let a127 = {
                        let a125 = v3;
                        let a126 = 2u8;
                        fixed_u8::bit_and(a125, a126)
                    };
                    let a128 = 2u8;
                    if equal(a127, a128) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a134 = {
                    let a131 = {
                        let a129 = v3;
                        let a130 = 1u8;
                        fixed_u8::bit_and(a129, a130)
                    };
                    let a132 = 1u8;
                    if equal(a131, a132) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a133, a134)?
            };
            nat_add(a135, a136)?
        };
        nat_add(a137, a138)?
    };
    let a139 = v4;
    let a140 = 34u64;
    if nat_eq(a139, a140) {
        Ok(Bytes::lit(&[0x5c, 0x22]))
    } else {
        let a141 = v4;
        let a142 = 92u64;
        if nat_eq(a141, a142) {
            Ok(Bytes::lit(&[0x5c, 0x5c]))
        } else {
            let a143 = v4;
            let a144 = 8u64;
            if nat_eq(a143, a144) {
                Ok(Bytes::lit(&[0x5c, 0x62]))
            } else {
                let a145 = v4;
                let a146 = 12u64;
                if nat_eq(a145, a146) {
                    Ok(Bytes::lit(&[0x5c, 0x66]))
                } else {
                    let a147 = v4;
                    let a148 = 10u64;
                    if nat_eq(a147, a148) {
                        Ok(Bytes::lit(&[0x5c, 0x6e]))
                    } else {
                        let a149 = v4;
                        let a150 = 13u64;
                        if nat_eq(a149, a150) {
                            Ok(Bytes::lit(&[0x5c, 0x72]))
                        } else {
                            let a151 = v4;
                            let a152 = 9u64;
                            if nat_eq(a151, a152) {
                                Ok(Bytes::lit(&[0x5c, 0x74]))
                            } else {
                                let a153 = v4;
                                let a154 = 32u64;
                                if nat_lt(a153, a154) {
                                    let a155 = Bytes::lit(&[0x5c, 0x75, 0x30, 0x30]);
                                    let a156 = f34(v2)?;
                                    Ok(append_bytes(a155, a156))
                                } else {
                                    Ok(f35(v0, v1, 1u64))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f28(v0: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    f36(Fn6::F37, List::<List<(Bytes, Bytes)>>::nil(), v0)
}

pub fn f29(v0: Fn4, p160: List<List<(Bytes, Bytes)>>, p161: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    let m159 = (p160, p161);
    {
        let mut m159 = m159;
        loop {
            let step = {
                let (v1, v2) = m159;
                let m157 = v2;
                match m157.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(({
                            let c158: Fn4 = v0.clone();
                            c158.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m159 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f30(v0: List<List<(Bytes, Bytes)>>, v1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
    List::cons(List::cons(v1, List::<(Bytes, Bytes)>::nil()), v0)
}

pub fn f31(v0: Fn5, p166: u64, p167: List<List<(Bytes, Bytes)>>) -> R<(List<List<(Bytes, Bytes)>>, bool)> {
    let m165 = (p166, p167);
    {
        let mut m165 = m165;
        loop {
            let step = {
                let (v1, v2) = m165;
                let m162 = v1;
                if m162 == 0 {
                    Err::<(u64, List<List<(Bytes, Bytes)>>), R<(List<List<(Bytes, Bytes)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m162 - 1;
                    let m164 = {
                        let c163: Fn5 = v0.clone();
                        c163.apply(v2.clone())?
                    };
                    match m164 {
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
                Ok(next) => { m165 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f32(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    f38(v0)
}

pub fn f33(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h168, _) = v2;
        h168
    };
    let a169 = v3;
    let a170 = 0u64;
    if nat_eq(a169, a170) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a171 = v3;
            let a172 = v1;
            let a173 = 0u64;
            nat_quot(a171, a172, a173)
        }, {
            let a178 = f25(v0, {
                let a174 = v3;
                let a175 = v1;
                let a176 = 0u64;
                nat_rem(a174, a175, a176)
            });
            let a179 = {
                let (_, h177) = v2;
                h177
            };
            append_bytes(a178, a179)
        }))
    }
}

pub fn f34(v0: u8) -> R<Bytes> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a224 = {
            let a200 = {
                let a188 = {
                    let a182 = {
                        let a180 = v1;
                        let a181 = 128u8;
                        fixed_u8::bit_and(a180, a181)
                    };
                    let a183 = 128u8;
                    if equal(a182, a183) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a189 = {
                    let a186 = {
                        let a184 = v1;
                        let a185 = 64u8;
                        fixed_u8::bit_and(a184, a185)
                    };
                    let a187 = 64u8;
                    if equal(a186, a187) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a188, a189)?
            };
            let a201 = {
                let a198 = {
                    let a192 = {
                        let a190 = v1;
                        let a191 = 32u8;
                        fixed_u8::bit_and(a190, a191)
                    };
                    let a193 = 32u8;
                    if equal(a192, a193) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a199 = {
                    let a196 = {
                        let a194 = v1;
                        let a195 = 16u8;
                        fixed_u8::bit_and(a194, a195)
                    };
                    let a197 = 16u8;
                    if equal(a196, a197) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a198, a199)?
            };
            nat_add(a200, a201)?
        };
        let a225 = {
            let a222 = {
                let a210 = {
                    let a204 = {
                        let a202 = v1;
                        let a203 = 8u8;
                        fixed_u8::bit_and(a202, a203)
                    };
                    let a205 = 8u8;
                    if equal(a204, a205) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a211 = {
                    let a208 = {
                        let a206 = v1;
                        let a207 = 4u8;
                        fixed_u8::bit_and(a206, a207)
                    };
                    let a209 = 4u8;
                    if equal(a208, a209) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a210, a211)?
            };
            let a223 = {
                let a220 = {
                    let a214 = {
                        let a212 = v1;
                        let a213 = 2u8;
                        fixed_u8::bit_and(a212, a213)
                    };
                    let a215 = 2u8;
                    if equal(a214, a215) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a221 = {
                    let a218 = {
                        let a216 = v1;
                        let a217 = 1u8;
                        fixed_u8::bit_and(a216, a217)
                    };
                    let a219 = 1u8;
                    if equal(a218, a219) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a220, a221)?
            };
            nat_add(a222, a223)?
        };
        nat_add(a224, a225)?
    };
    let a232 = f25(f39(), {
        let a226 = v2;
        let a227 = 16u64;
        let a228 = 0u64;
        nat_quot(a226, a227, a228)
    });
    let a233 = f25(f39(), {
        let a229 = v2;
        let a230 = 16u64;
        let a231 = 0u64;
        nat_rem(a229, a230, a231)
    });
    Ok(append_bytes(a232, a233))
}

pub fn f35(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m237 = {
        let a234 = v0;
        let a235 = v1;
        let a236 = v2;
        slice_bytes(a234, a235, a236)
    };
    match m237 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f36(v0: Fn6, p241: List<List<(Bytes, Bytes)>>, p242: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    let m240 = (p241, p242);
    {
        let mut m240 = m240;
        loop {
            let step = {
                let (v1, v2) = m240;
                let m238 = v2;
                match m238.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(({
                            let c239: Fn6 = v0.clone();
                            c239.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m240 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f37(v0: List<List<(Bytes, Bytes)>>, v1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    List::cons(v1, v0)
}

pub fn f38(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    let a244 = {
        let a243 = v0.clone();
        length_list(a243)
    };
    let a245 = 1u64;
    if nat_le(a244, a245) {
        Ok(None::<List<List<(Bytes, Bytes)>>>)
    } else {
        Ok(Some(f40(v0)?))
    }
}

pub fn f39() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66])
}

pub fn f40(v0: List<List<(Bytes, Bytes)>>) -> R<List<List<(Bytes, Bytes)>>> {
    Ok(f28({
        let (_, h246) = {
            let (h247, _) = f41(Fn7::F42, {
                let a249 = {
                    let a248 = v0.clone();
                    length_list(a248)
                };
                let a250 = 1u64;
                nat_add(a249, a250)?
            }, (v0, List::<List<(Bytes, Bytes)>>::nil()))?;
            h247
        };
        h246
    }))
}

pub fn f41(v0: Fn7, p255: u64, p256: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)> {
    let m254 = (p255, p256);
    {
        let mut m254 = m254;
        loop {
            let step = {
                let (v1, v2) = m254;
                let m251 = v1;
                if m251 == 0 {
                    Err::<(u64, (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)), R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m251 - 1;
                    let m253 = {
                        let c252: Fn7 = v0.clone();
                        c252.apply(v2.clone())?
                    };
                    match m253 {
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
                Ok(next) => { m254 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f42(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    f43(v0)
}

pub fn f43(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    let v1: List<List<(Bytes, Bytes)>> = {
        let (h257, _) = v0;
        h257
    };
    let v2: List<List<(Bytes, Bytes)>> = {
        let (_, h258) = v0;
        h258
    };
    let m259 = v1;
    match m259.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>)
        }
        Some((v3, v4)) => {
            let m260 = v4;
            match m260.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Bytes)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f44(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f44(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v2: u64 = {
        let a265 = {
            let a263 = {
                let a261 = v0.clone();
                length_list(a261)
            };
            let a264 = {
                let a262 = v1.clone();
                length_list(a262)
            };
            nat_add(a263, a264)?
        };
        let a266 = 1u64;
        nat_add(a265, a266)?
    };
    Ok(f45({
        let (_, h267) = {
            let (_, h268) = {
                let (h269, _) = f46(Fn8::F47, v2, (v0, (v1, List::<(Bytes, Bytes)>::nil())));
                h269
            };
            h268
        };
        h267
    }))
}

pub fn f45(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f12(Fn1::F48, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f46(v0: Fn8, p274: u64, p275: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool) {
    let m273 = (p274, p275);
    {
        let mut m273 = m273;
        loop {
            let step = {
                let (v1, v2) = m273;
                let m270 = v1;
                if m270 == 0 {
                    Err::<(u64, (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))), ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool)>((v2, false))
                } else {
                    let v3 = m270 - 1;
                    let m272 = {
                        let c271: Fn8 = v0.clone();
                        c271.apply(v2.clone())
                    };
                    match m272 {
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
                Ok(next) => { m273 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f47(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    f49(v0)
}

pub fn f48(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn f49(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    let v1: List<(Bytes, Bytes)> = {
        let (h276, _) = v0.clone();
        h276
    };
    let v2: List<(Bytes, Bytes)> = {
        let (h277, _) = {
            let (_, h278) = v0.clone();
            h278
        };
        h277
    };
    let v3: List<(Bytes, Bytes)> = {
        let (_, h279) = {
            let (_, h280) = v0;
            h280
        };
        h279
    };
    let m281 = v1.clone();
    match m281.uncons() {
        None => {
            let m282 = v2;
            match m282.uncons() {
                None => {
                    None::<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Bytes)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m283 = v2.clone();
            match m283.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Bytes)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f50({
                        let (h284, _) = v8.clone();
                        h284
                    }, {
                        let (h285, _) = v6.clone();
                        h285
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

pub fn f50(v0: Bytes, v1: Bytes) -> bool {
    let a290 = {
        let a286 = v0;
        let a287 = v1;
        compare_bytes(a286, a287)
    };
    let a291 = {
        let a288 = Bytes::lit(&[]);
        let a289 = Bytes::lit(&[0x01]);
        compare_bytes(a288, a289)
    };
    equal(a290, a291)
}

pub fn run(p0: Adt0) -> R<Bytes> {
    f0(p0)
}
