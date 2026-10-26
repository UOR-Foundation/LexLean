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

pub fn parse_int(text: Str) -> R<Option<i64>> { match canonical_decimal(&text.0) { None => Ok(None), Some(Some(n)) => match i64::try_from(n) { Ok(value) => Ok(Some(value)), Err(_) => Err(Overflow), }, Some(None) => Err(Overflow), } }

pub fn format_u8(a: u8) -> Str { let text = a.to_string(); tick(text.len() as u64); Str::lit(&text) }

pub fn parse_u8(text: Str) -> Option<u8> { match canonical_decimal(&text.0) { Some(Some(n)) => u8::try_from(n).ok(), _ => None, } }

pub fn format_u16(a: u16) -> Str { let text = a.to_string(); tick(text.len() as u64); Str::lit(&text) }

pub fn parse_u16(text: Str) -> Option<u16> { match canonical_decimal(&text.0) { Some(Some(n)) => u16::try_from(n).ok(), _ => None, } }

pub fn format_u32(a: u32) -> Str { let text = a.to_string(); tick(text.len() as u64); Str::lit(&text) }

pub fn parse_u32(text: Str) -> Option<u32> { match canonical_decimal(&text.0) { Some(Some(n)) => u32::try_from(n).ok(), _ => None, } }

pub fn format_u64(a: u64) -> Str { let text = a.to_string(); tick(text.len() as u64); Str::lit(&text) }

pub fn parse_u64(text: Str) -> Option<u64> { match canonical_decimal(&text.0) { Some(Some(n)) => u64::try_from(n).ok(), _ => None, } }

pub fn format_i8(a: i8) -> Str { let text = a.to_string(); tick(text.len() as u64); Str::lit(&text) }

pub fn parse_i8(text: Str) -> Option<i8> { match canonical_decimal(&text.0) { Some(Some(n)) => i8::try_from(n).ok(), _ => None, } }

pub fn format_i16(a: i16) -> Str { let text = a.to_string(); tick(text.len() as u64); Str::lit(&text) }

pub fn parse_i16(text: Str) -> Option<i16> { match canonical_decimal(&text.0) { Some(Some(n)) => i16::try_from(n).ok(), _ => None, } }

pub fn format_i32(a: i32) -> Str { let text = a.to_string(); tick(text.len() as u64); Str::lit(&text) }

pub fn parse_i32(text: Str) -> Option<i32> { match canonical_decimal(&text.0) { Some(Some(n)) => i32::try_from(n).ok(), _ => None, } }

pub fn format_i64(a: i64) -> Str { let text = a.to_string(); tick(text.len() as u64); Str::lit(&text) }

pub fn parse_i64(text: Str) -> Option<i64> { match canonical_decimal(&text.0) { Some(Some(n)) => i64::try_from(n).ok(), _ => None, } }

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

#[derive(Clone)]
pub enum Adt0 {
    C0,
    C1,
    C2(u64),
}

#[derive(Clone)]
pub enum Adt1 {
    C0((u64, u64), List<Adt0>),
}

#[derive(Clone)]
pub enum Adt2 {
    C0(List<Adt1>, List<(u64, u64)>, Option<((u64, u64), Adt1)>, bool, Adt3),
}

#[derive(Clone)]
pub enum Adt3 {
    C0(u64, u64, u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Fn0 {
    F25(u64),
}

impl Fn0 {
    pub fn apply(&self, p0: Adt2) -> R<Option<Adt2>> {
        match self {
            Fn0::F25(k0) => f25(*k0, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F18(Adt1),
}

impl Fn1 {
    pub fn apply(&self, p0: List<Adt1>, p1: u64) -> R<List<Adt1>> {
        match self {
            Fn1::F18(k0) => f18(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F21,
}

impl Fn2 {
    pub fn apply(&self, p0: (List<Adt1>, List<(u64, u64)>), p1: Adt1) -> (List<Adt1>, List<(u64, u64)>) {
        match self {
            Fn2::F21 => f21(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F23(u64),
}

impl Fn3 {
    pub fn apply(&self, p0: List<Adt1>, p1: Adt1) -> List<Adt1> {
        match self {
            Fn3::F23(k0) => f23(*k0, p0, p1),
        }
    }
}

pub fn f0(v0: u64) -> R<(Result<((u64, u64), List<Adt0>), (bool, bool)>, (u64, (u64, (u64, (u64, (u64, u64))))))> {
    let v1: (Adt2, bool) = f32(Fn0::F25(v0), {
        let a1 = v0;
        let a2 = 8u64;
        nat_mul(a1, a2)?
    }, f24(v0)?)?;
    Ok(({
        let m6 = {
            let m5 = {
                let (h4, _) = v1.clone();
                h4
            };
            match m5 {
                Adt2::C0(_, _, h3, _, _) => {
                    h3
                }
            }
        };
        match m6 {
            None => {
                Err::<((u64, u64), List<Adt0>), (bool, bool)>({
                    let a16 = {
                        let (_, h11) = v1.clone();
                        h11
                    };
                    let a17 = {
                        let a15 = {
                            let m14 = {
                                let (h13, _) = v1.clone();
                                h13
                            };
                            match m14 {
                                Adt2::C0(_, _, _, h12, _) => {
                                    h12
                                }
                            }
                        };
                        bool_not(a15)
                    };
                    if bool_and(a16, a17) {
                        (false, true)
                    } else {
                        (false, false)
                    }
                })
            }
            Some(v2) => {
                Ok::<((u64, u64), List<Adt0>), (bool, bool)>(({
                    let (h7, _) = v2;
                    h7
                }, {
                    let m10 = {
                        let (_, h9) = v2;
                        h9
                    };
                    match m10 {
                        Adt1::C0(_, h8) => {
                            h8
                        }
                    }
                }))
            }
        }
    }, ({
        let m22 = {
            let m21 = {
                let (h20, _) = v1.clone();
                h20
            };
            match m21 {
                Adt2::C0(_, _, _, _, h19) => {
                    h19
                }
            }
        };
        match m22 {
            Adt3::C0(h18, _, _, _, _, _) => {
                h18
            }
        }
    }, ({
        let m27 = {
            let m26 = {
                let (h25, _) = v1.clone();
                h25
            };
            match m26 {
                Adt2::C0(_, _, _, _, h24) => {
                    h24
                }
            }
        };
        match m27 {
            Adt3::C0(_, h23, _, _, _, _) => {
                h23
            }
        }
    }, ({
        let m32 = {
            let m31 = {
                let (h30, _) = v1.clone();
                h30
            };
            match m31 {
                Adt2::C0(_, _, _, _, h29) => {
                    h29
                }
            }
        };
        match m32 {
            Adt3::C0(_, _, h28, _, _, _) => {
                h28
            }
        }
    }, ({
        let m37 = {
            let m36 = {
                let (h35, _) = v1.clone();
                h35
            };
            match m36 {
                Adt2::C0(_, _, _, _, h34) => {
                    h34
                }
            }
        };
        match m37 {
            Adt3::C0(_, _, _, h33, _, _) => {
                h33
            }
        }
    }, ({
        let m42 = {
            let m41 = {
                let (h40, _) = v1.clone();
                h40
            };
            match m41 {
                Adt2::C0(_, _, _, _, h39) => {
                    h39
                }
            }
        };
        match m42 {
            Adt3::C0(_, _, _, _, h38, _) => {
                h38
            }
        }
    }, {
        let m47 = {
            let m46 = {
                let (h45, _) = v1;
                h45
            };
            match m46 {
                Adt2::C0(_, _, _, _, h44) => {
                    h44
                }
            }
        };
        match m47 {
            Adt3::C0(_, _, _, _, _, h43) => {
                h43
            }
        }
    })))))))
}

pub fn f1(v0: (u64, u64)) -> bool {
    let a54 = {
        let a49 = {
            let (h48, _) = v0;
            h48
        };
        let a50 = 4u64;
        nat_lt(a49, a50)
    };
    let a55 = {
        let a52 = {
            let (_, h51) = v0;
            h51
        };
        let a53 = 3u64;
        nat_le(a52, a53)
    };
    bool_and(a54, a55)
}

pub fn f2(v0: (u64, u64)) -> (u64, u64) {
    (4u64, {
        let (_, h56) = v0;
        h56
    })
}

pub fn f3(v0: (u64, u64)) -> Option<(u64, u64)> {
    if f1(v0) {
        Some(f2(v0))
    } else {
        None::<(u64, u64)>
    }
}

pub fn f4(v0: (u64, u64)) -> bool {
    let a63 = {
        let a58 = 0u64;
        let a59 = {
            let (_, h57) = v0;
            h57
        };
        nat_lt(a58, a59)
    };
    let a64 = {
        let a61 = {
            let (h60, _) = v0;
            h60
        };
        let a62 = 4u64;
        nat_le(a61, a62)
    };
    bool_and(a63, a64)
}

pub fn f5(v0: (u64, u64)) -> (u64, u64) {
    ({
        let (h65, _) = v0;
        h65
    }, 0u64)
}

pub fn f6(v0: (u64, u64)) -> Option<(u64, u64)> {
    if f4(v0) {
        Some(f5(v0))
    } else {
        None::<(u64, u64)>
    }
}

pub fn f7(v0: (u64, u64), v1: u64) -> R<bool> {
    let a79 = {
        let a67 = v1;
        let a68 = {
            let (h66, _) = v0;
            h66
        };
        nat_le(a67, a68)
    };
    let a80 = {
        let a77 = {
            let a72 = {
                let a70 = {
                    let (_, h69) = v0;
                    h69
                };
                let a71 = v1;
                nat_add(a70, a71)?
            };
            let a73 = 3u64;
            nat_le(a72, a73)
        };
        let a78 = {
            let a75 = {
                let (h74, _) = v0;
                h74
            };
            let a76 = 4u64;
            nat_le(a75, a76)
        };
        bool_and(a77, a78)
    };
    Ok(bool_and(a79, a80))
}

pub fn f8(v0: (u64, u64), v1: u64) -> R<(u64, u64)> {
    Ok(({
        let a82 = {
            let (h81, _) = v0;
            h81
        };
        let a83 = v1;
        nat_sub(a82, a83)
    }, {
        let a85 = {
            let (_, h84) = v0;
            h84
        };
        let a86 = v1;
        nat_add(a85, a86)?
    }))
}

pub fn f9(_: (u64, u64)) -> List<u64> {
    List::cons(1u64, List::cons(2u64, List::cons(3u64, List::<u64>::nil())))
}

pub fn f10(v0: (u64, u64), v1: u64) -> R<Option<(u64, u64)>> {
    if f7(v0, v1)? {
        Ok(Some(f8(v0, v1)?))
    } else {
        Ok(None::<(u64, u64)>)
    }
}

pub fn f11(v0: (u64, u64), v1: Adt0) -> R<Option<(u64, u64)>> {
    let m87 = v1;
    match m87 {
        Adt0::C0 => {
            Ok(f3(v0))
        }
        Adt0::C1 => {
            Ok(f6(v0))
        }
        Adt0::C2(v2) => {
            f10(v0, v2)
        }
    }
}

pub fn f12(_: u64) -> (u64, u64) {
    (0u64, 0u64)
}

pub fn f13(v0: (u64, u64)) -> Option<(u64, u64)> {
    Some(v0)
}

pub fn f14(v0: u64, v1: (u64, u64)) -> bool {
    let a94 = {
        let a89 = {
            let (_, h88) = v1;
            h88
        };
        let a90 = v0;
        nat_eq(a89, a90)
    };
    let a95 = {
        let a92 = {
            let (h91, _) = v1;
            h91
        };
        let a93 = 4u64;
        nat_le(a92, a93)
    };
    bool_and(a94, a95)
}

pub fn f15(v0: u64, v1: (u64, u64)) -> Option<(u64, u64)> {
    let m96 = f13(v1);
    match m96 {
        None => {
            None::<(u64, u64)>
        }
        Some(v2) => {
            let v3: bool = f14(v0, v2);
            if v3 {
                Some(v2)
            } else {
                None::<(u64, u64)>
            }
        }
    }
}

pub fn f16(v0: Adt1) -> R<List<Adt1>> {
    let m98 = f11(match v0.clone() {
        Adt1::C0(h97, _) => {
            h97
        }
    }, Adt0::C0)?;
    match m98 {
        None => {
            Ok(List::<Adt1>::nil())
        }
        Some(v1) => {
            Ok(List::cons(Adt1::C0(v1, {
                let a100 = match v0 {
                    Adt1::C0(_, h99) => {
                        h99
                    }
                };
                let a101 = List::cons(Adt0::C0, List::<Adt0>::nil());
                append_list(a100, a101)
            }), List::<Adt1>::nil()))
        }
    }
}

pub fn f17(v0: Adt1) -> R<List<Adt1>> {
    let m103 = f11(match v0.clone() {
        Adt1::C0(h102, _) => {
            h102
        }
    }, Adt0::C1)?;
    match m103 {
        None => {
            Ok(List::<Adt1>::nil())
        }
        Some(v1) => {
            Ok(List::cons(Adt1::C0(v1, {
                let a105 = match v0 {
                    Adt1::C0(_, h104) => {
                        h104
                    }
                };
                let a106 = List::cons(Adt0::C1, List::<Adt0>::nil());
                append_list(a105, a106)
            }), List::<Adt1>::nil()))
        }
    }
}

pub fn f18(v0: Adt1, v1: List<Adt1>, v2: u64) -> R<List<Adt1>> {
    let m108 = f11(match v0.clone() {
        Adt1::C0(h107, _) => {
            h107
        }
    }, Adt0::C2(v2))?;
    match m108 {
        None => {
            Ok(v1)
        }
        Some(v3) => {
            let a112 = v1;
            let a113 = List::cons(Adt1::C0(v3, {
                let a110 = match v0 {
                    Adt1::C0(_, h109) => {
                        h109
                    }
                };
                let a111 = List::cons(Adt0::C2(v2), List::<Adt0>::nil());
                append_list(a110, a111)
            }), List::<Adt1>::nil());
            Ok(append_list(a112, a113))
        }
    }
}

pub fn f19(v0: Adt1) -> R<List<Adt1>> {
    f28(Fn1::F18(v0.clone()), List::<Adt1>::nil(), f9(match v0 {
        Adt1::C0(h114, _) => {
            h114
        }
    }))
}

pub fn f20(v0: Adt1) -> R<List<Adt1>> {
    let a117 = f16(v0.clone())?;
    let a118 = {
        let a115 = f17(v0.clone())?;
        let a116 = f19(v0)?;
        append_list(a115, a116)
    };
    Ok(append_list(a117, a118))
}

pub fn f21(v0: (List<Adt1>, List<(u64, u64)>), v1: Adt1) -> (List<Adt1>, List<(u64, u64)>) {
    if f30({
        let (_, h119) = v0.clone();
        h119
    }, match v1.clone() {
        Adt1::C0(h120, _) => {
            h120
        }
    }) {
        v0
    } else {
        ({
            let a122 = {
                let (h121, _) = v0;
                h121
            };
            let a123 = List::cons(v1.clone(), List::<Adt1>::nil());
            append_list(a122, a123)
        }, f31({
            let (_, h124) = v0;
            h124
        }, match v1 {
            Adt1::C0(h125, _) => {
                h125
            }
        }))
    }
}

pub fn f22(v0: List<(u64, u64)>, v1: List<Adt1>) -> (List<Adt1>, List<(u64, u64)>) {
    f29(Fn2::F21, (List::<Adt1>::nil(), v0), v1)
}

pub fn f23(v0: u64, v1: List<Adt1>, v2: Adt1) -> List<Adt1> {
    let a127 = {
        let a126 = v1.clone();
        length_list(a126)
    };
    let a128 = v0;
    if nat_lt(a127, a128) {
        let a129 = v1;
        let a130 = List::cons(v2, List::<Adt1>::nil());
        append_list(a129, a130)
    } else {
        v1
    }
}

pub fn f24(v0: u64) -> R<Adt2> {
    let v1: u64 = {
        let a131 = v0;
        let a132 = v0;
        nat_add(a131, a132)?
    };
    let v2: List<Adt1> = f27(Fn3::F23(v1), List::<Adt1>::nil(), List::cons(Adt1::C0(f12(v0), List::<Adt0>::nil()), List::<Adt1>::nil()));
    Ok(Adt2::C0(v2.clone(), f31(List::<(u64, u64)>::nil(), f12(v0)), None::<((u64, u64), Adt1)>, {
        let a133 = v1;
        let a134 = 1u64;
        nat_lt(a133, a134)
    }, Adt3::C0(0u64, 0u64, 0u64, 0u64, 0u64, {
        let a135 = v2;
        length_list(a135)
    })))
}

pub fn f25(v0: u64, v1: Adt2) -> R<Option<Adt2>> {
    let m137 = match v1.clone() {
        Adt2::C0(_, _, h136, _, _) => {
            h136
        }
    };
    match m137 {
        None => {
            let m139 = match v1.clone() {
                Adt2::C0(h138, _, _, _, _) => {
                    h138
                }
            };
            match m139.uncons() {
                None => {
                    Ok(None::<Adt2>)
                }
                Some((v3, v4)) => {
                    let m141 = f15(v0, match v3.clone() {
                        Adt1::C0(h140, _) => {
                            h140
                        }
                    });
                    match m141 {
                        None => {
                            let v6: List<Adt1> = f20(v3.clone())?;
                            let v7: (List<Adt1>, List<(u64, u64)>) = f22(match v1.clone() {
                                Adt2::C0(_, h166, _, _, _) => {
                                    h166
                                }
                            }, v6.clone());
                            let v8: List<Adt1> = {
                                let a168 = v4;
                                let a169 = {
                                    let (h167, _) = v7;
                                    h167
                                };
                                append_list(a168, a169)
                            };
                            let v9: List<Adt1> = f27(Fn3::F23({
                                let a170 = v0;
                                let a171 = v0;
                                nat_add(a170, a171)?
                            }), List::<Adt1>::nil(), v8.clone());
                            Ok(Some(Adt2::C0(v9.clone(), {
                                let (_, h172) = v7;
                                h172
                            }, None::<((u64, u64), Adt1)>, {
                                let a179 = match v1.clone() {
                                    Adt2::C0(_, _, _, h173, _) => {
                                        h173
                                    }
                                };
                                let a180 = {
                                    let a177 = {
                                        let a174 = v0;
                                        let a175 = v0;
                                        nat_add(a174, a175)?
                                    };
                                    let a178 = {
                                        let a176 = v8;
                                        length_list(a176)
                                    };
                                    nat_lt(a177, a178)
                                };
                                bool_or(a179, a180)
                            }, Adt3::C0({
                                let a184 = {
                                    let m183 = match v1.clone() {
                                        Adt2::C0(_, _, _, _, h182) => {
                                            h182
                                        }
                                    };
                                    match m183 {
                                        Adt3::C0(h181, _, _, _, _, _) => {
                                            h181
                                        }
                                    }
                                };
                                let a185 = 1u64;
                                nat_add(a184, a185)?
                            }, {
                                let a190 = {
                                    let m188 = match v1.clone() {
                                        Adt2::C0(_, _, _, _, h187) => {
                                            h187
                                        }
                                    };
                                    match m188 {
                                        Adt3::C0(_, h186, _, _, _, _) => {
                                            h186
                                        }
                                    }
                                };
                                let a191 = f26(match v3 {
                                    Adt1::C0(h189, _) => {
                                        h189
                                    }
                                })?;
                                nat_add(a190, a191)?
                            }, {
                                let a196 = {
                                    let m194 = match v1.clone() {
                                        Adt2::C0(_, _, _, _, h193) => {
                                            h193
                                        }
                                    };
                                    match m194 {
                                        Adt3::C0(_, _, h192, _, _, _) => {
                                            h192
                                        }
                                    }
                                };
                                let a197 = {
                                    let a195 = v6;
                                    length_list(a195)
                                };
                                nat_add(a196, a197)?
                            }, {
                                let a201 = {
                                    let m200 = match v1.clone() {
                                        Adt2::C0(_, _, _, _, h199) => {
                                            h199
                                        }
                                    };
                                    match m200 {
                                        Adt3::C0(_, _, _, h198, _, _) => {
                                            h198
                                        }
                                    }
                                };
                                let a202 = 1u64;
                                nat_add(a201, a202)?
                            }, {
                                let a206 = {
                                    let m205 = match v1.clone() {
                                        Adt2::C0(_, _, _, _, h204) => {
                                            h204
                                        }
                                    };
                                    match m205 {
                                        Adt3::C0(_, _, _, _, h203, _) => {
                                            h203
                                        }
                                    }
                                };
                                let a207 = 1u64;
                                nat_add(a206, a207)?
                            }, {
                                let a212 = {
                                    let m210 = match v1.clone() {
                                        Adt2::C0(_, _, _, _, h209) => {
                                            h209
                                        }
                                    };
                                    match m210 {
                                        Adt3::C0(_, _, _, _, _, h208) => {
                                            h208
                                        }
                                    }
                                };
                                let a213 = {
                                    let a211 = v9.clone();
                                    length_list(a211)
                                };
                                if nat_lt(a212, a213) {
                                    let a214 = v9;
                                    length_list(a214)
                                } else {
                                    let m217 = match v1 {
                                        Adt2::C0(_, _, _, _, h216) => {
                                            h216
                                        }
                                    };
                                    match m217 {
                                        Adt3::C0(_, _, _, _, _, h215) => {
                                            h215
                                        }
                                    }
                                }
                            }))))
                        }
                        Some(v5) => {
                            Ok(Some(Adt2::C0(v4, match v1.clone() {
                                Adt2::C0(_, h142, _, _, _) => {
                                    h142
                                }
                            }, Some((v5, v3)), match v1.clone() {
                                Adt2::C0(_, _, _, h143, _) => {
                                    h143
                                }
                            }, Adt3::C0({
                                let a147 = {
                                    let m146 = match v1.clone() {
                                        Adt2::C0(_, _, _, _, h145) => {
                                            h145
                                        }
                                    };
                                    match m146 {
                                        Adt3::C0(h144, _, _, _, _, _) => {
                                            h144
                                        }
                                    }
                                };
                                let a148 = 1u64;
                                nat_add(a147, a148)?
                            }, {
                                let m151 = match v1.clone() {
                                    Adt2::C0(_, _, _, _, h150) => {
                                        h150
                                    }
                                };
                                match m151 {
                                    Adt3::C0(_, h149, _, _, _, _) => {
                                        h149
                                    }
                                }
                            }, {
                                let m154 = match v1.clone() {
                                    Adt2::C0(_, _, _, _, h153) => {
                                        h153
                                    }
                                };
                                match m154 {
                                    Adt3::C0(_, _, h152, _, _, _) => {
                                        h152
                                    }
                                }
                            }, {
                                let m157 = match v1.clone() {
                                    Adt2::C0(_, _, _, _, h156) => {
                                        h156
                                    }
                                };
                                match m157 {
                                    Adt3::C0(_, _, _, h155, _, _) => {
                                        h155
                                    }
                                }
                            }, {
                                let a161 = {
                                    let m160 = match v1.clone() {
                                        Adt2::C0(_, _, _, _, h159) => {
                                            h159
                                        }
                                    };
                                    match m160 {
                                        Adt3::C0(_, _, _, _, h158, _) => {
                                            h158
                                        }
                                    }
                                };
                                let a162 = 1u64;
                                nat_add(a161, a162)?
                            }, {
                                let m165 = match v1 {
                                    Adt2::C0(_, _, _, _, h164) => {
                                        h164
                                    }
                                };
                                match m165 {
                                    Adt3::C0(_, _, _, _, _, h163) => {
                                        h163
                                    }
                                }
                            }))))
                        }
                    }
                }
            }
        }
        Some(_) => {
            Ok(None::<Adt2>)
        }
    }
}

pub fn f26(v0: (u64, u64)) -> R<u64> {
    let a221 = 1u64;
    let a222 = {
        let a219 = 1u64;
        let a220 = {
            let a218 = f9(v0);
            length_list(a218)
        };
        nat_add(a219, a220)?
    };
    nat_add(a221, a222)
}

pub fn f27(v0: Fn3, p226: List<Adt1>, p227: List<Adt1>) -> List<Adt1> {
    let m225 = (p226, p227);
    {
        let mut m225 = m225;
        loop {
            let step = {
                let (v1, v2) = m225;
                let m223 = v2;
                match m223.uncons() {
                    None => {
                        Err::<(List<Adt1>, List<Adt1>), List<Adt1>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt1>, List<Adt1>), List<Adt1>>(({
                            let c224: Fn3 = v0.clone();
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

pub fn f28(v0: Fn1, p231: List<Adt1>, p232: List<u64>) -> R<List<Adt1>> {
    let m230 = (p231, p232);
    {
        let mut m230 = m230;
        loop {
            let step = {
                let (v1, v2) = m230;
                let m228 = v2;
                match m228.uncons() {
                    None => {
                        Err::<(List<Adt1>, List<u64>), R<List<Adt1>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt1>, List<u64>), R<List<Adt1>>>(({
                            let c229: Fn1 = v0.clone();
                            c229.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m230 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f29(v0: Fn2, p236: (List<Adt1>, List<(u64, u64)>), p237: List<Adt1>) -> (List<Adt1>, List<(u64, u64)>) {
    let m235 = (p236, p237);
    {
        let mut m235 = m235;
        loop {
            let step = {
                let (v1, v2) = m235;
                let m233 = v2;
                match m233.uncons() {
                    None => {
                        Err::<((List<Adt1>, List<(u64, u64)>), List<Adt1>), (List<Adt1>, List<(u64, u64)>)>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<((List<Adt1>, List<(u64, u64)>), List<Adt1>), (List<Adt1>, List<(u64, u64)>)>(({
                            let c234: Fn2 = v0.clone();
                            c234.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m235 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f30(p243: List<(u64, u64)>, v1: (u64, u64)) -> bool {
    let m242 = p243;
    {
        let mut m242 = m242;
        loop {
            let step = {
                let v0 = m242;
                let m238 = v0;
                match m238.uncons() {
                    None => {
                        Err::<List<(u64, u64)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let m241 = {
                            let a239 = v1;
                            let a240 = v2;
                            compare(a239, a240)
                        };
                        match m241 {
                            Ordering::Less => {
                                Err::<List<(u64, u64)>, bool>(false)
                            }
                            Ordering::Equal => {
                                Err::<List<(u64, u64)>, bool>(true)
                            }
                            Ordering::Greater => {
                                Ok::<List<(u64, u64)>, bool>(v3)
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m242 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f31(v0: List<(u64, u64)>, v1: (u64, u64)) -> List<(u64, u64)> {
    let m244 = v0.clone();
    match m244.uncons() {
        None => {
            List::cons(v1, List::<(u64, u64)>::nil())
        }
        Some((v2, v3)) => {
            let m247 = {
                let a245 = v1;
                let a246 = v2;
                compare(a245, a246)
            };
            match m247 {
                Ordering::Less => {
                    List::cons(v1, v0)
                }
                Ordering::Equal => {
                    v0
                }
                Ordering::Greater => {
                    List::cons(v2, f31(v3, v1))
                }
            }
        }
    }
}

pub fn f32(v0: Fn0, p252: u64, p253: Adt2) -> R<(Adt2, bool)> {
    let m251 = (p252, p253);
    {
        let mut m251 = m251;
        loop {
            let step = {
                let (v1, v2) = m251;
                let m248 = v1;
                if m248 == 0 {
                    Err::<(u64, Adt2), R<(Adt2, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m248 - 1;
                    let m250 = {
                        let c249: Fn0 = v0.clone();
                        c249.apply(v2.clone())?
                    };
                    match m250 {
                        None => {
                            Err::<(u64, Adt2), R<(Adt2, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt2), R<(Adt2, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m251 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn run(p0: u64) -> R<(Result<((u64, u64), List<Adt0>), (bool, bool)>, (u64, (u64, (u64, (u64, (u64, u64))))))> {
    f0(p0)
}
