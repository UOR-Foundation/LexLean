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
pub enum Fn0 {
    F6,
}

impl Fn0 {
    pub fn apply(&self, p0: (Option<(u64, i64)>, List<u64>), p1: i64) -> (Option<(u64, i64)>, List<u64>) {
        match self {
            Fn0::F6 => f6(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F14(List<i64>),
    F20(List<i64>),
}

impl Fn1 {
    pub fn apply(&self, p0: (List<i64>, List<i64>), p1: List<i64>) -> R<(List<i64>, List<i64>)> {
        match self {
            Fn1::F14(k0) => f14(k0.clone(), p0, p1),
            Fn1::F20(k0) => f20(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F18,
    F19,
}

impl Fn2 {
    pub fn apply(&self, p0: List<i64>, p1: i64) -> R<List<i64>> {
        match self {
            Fn2::F18 => f18(p0, p1),
            Fn2::F19 => Ok(f19(p0, p1)),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F25,
    F26,
}

impl Fn3 {
    pub fn apply(&self, p0: (i64, List<i64>), p1: i64) -> R<(i64, List<i64>)> {
        match self {
            Fn3::F25 => f25(p0, p1),
            Fn3::F26 => f26(p0, p1),
        }
    }
}

pub fn f0(v0: Adt0) -> R<Result<u64, (bool, bool)>> {
    let v1: Adt0 = v0;
    let v2: u64 = f1(v1.clone())?;
    if f2(v1, v2) {
        Ok(Ok::<u64, (bool, bool)>(v2))
    } else {
        Ok(Err::<u64, (bool, bool)>((true, false)))
    }
}

pub fn f1(v0: Adt0) -> R<u64> {
    f3(v0)
}

pub fn f2(v0: Adt0, v1: u64) -> bool {
    let a1 = v1;
    let a2 = f4(v0);
    nat_eq(a1, a2)
}

pub fn f3(v0: Adt0) -> R<u64> {
    let m4 = {
        let (h3, _) = f5(Fn0::F6, (None::<(u64, i64)>, List::cons(0u64, List::cons(1u64, List::cons(2u64, List::cons(3u64, List::cons(4u64, List::cons(5u64, List::cons(6u64, List::cons(7u64, List::cons(8u64, List::cons(9u64, List::<u64>::nil()))))))))))), f7(v0)?);
        h3
    };
    match m4 {
        None => {
            Ok(0u64)
        }
        Some(v1) => {
            let (h5, _) = v1;
            Ok(h5)
        }
    }
}

pub fn f4(v0: Adt0) -> u64 {
    let m6 = v0;
    match m6 {
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
        Adt0::C4 => {
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
        Adt0::C8 => {
            8u64
        }
        Adt0::C9 => {
            9u64
        }
    }
}

pub fn f5(v0: Fn0, p10: (Option<(u64, i64)>, List<u64>), p11: List<i64>) -> (Option<(u64, i64)>, List<u64>) {
    let m9 = (p10, p11);
    {
        let mut m9 = m9;
        loop {
            let step = {
                let (v1, v2) = m9;
                let m7 = v2;
                match m7.uncons() {
                    None => {
                        Err::<((Option<(u64, i64)>, List<u64>), List<i64>), (Option<(u64, i64)>, List<u64>)>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<((Option<(u64, i64)>, List<u64>), List<i64>), (Option<(u64, i64)>, List<u64>)>(({
                            let c8: Fn0 = v0.clone();
                            c8.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m9 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f6(v0: (Option<(u64, i64)>, List<u64>), v1: i64) -> (Option<(u64, i64)>, List<u64>) {
    let m13 = {
        let (_, h12) = v0.clone();
        h12
    };
    match m13.uncons() {
        None => {
            v0
        }
        Some((v2, v3)) => {
            ({
                let m15 = {
                    let (h14, _) = v0;
                    h14
                };
                match m15 {
                    None => {
                        Some((v2, v1))
                    }
                    Some(v4) => {
                        let m19 = {
                            let a17 = {
                                let (_, h16) = v4;
                                h16
                            };
                            let a18 = v1;
                            compare(a17, a18)
                        };
                        if match m19 {
                            Ordering::Less => {
                                true
                            }
                            Ordering::Equal => {
                                false
                            }
                            Ordering::Greater => {
                                false
                            }
                        } {
                            Some((v2, v1))
                        } else {
                            Some(v4)
                        }
                    }
                }
            }, v3)
        }
    }
}

pub fn f7(v0: Adt0) -> R<List<i64>> {
    f8(f9(f10(f11(f12(v0))?)?)?)
}

pub fn f8(v0: List<i64>) -> R<List<i64>> {
    let (h20, _) = f13(Fn1::F14(v0), (List::<i64>::nil(), f15()), f16())?;
    Ok(h20)
}

pub fn f9(v0: List<i64>) -> R<List<i64>> {
    f17(Fn2::F18, List::<i64>::nil(), v0)
}

pub fn f10(v0: List<i64>) -> R<List<i64>> {
    f17(Fn2::F19, List::<i64>::nil(), v0)
}

pub fn f11(v0: List<i64>) -> R<List<i64>> {
    let (h21, _) = f13(Fn1::F20(v0), (List::<i64>::nil(), f21()), f22())?;
    Ok(h21)
}

pub fn f12(v0: Adt0) -> List<i64> {
    f23(v0)
}

pub fn f13(v0: Fn1, p25: (List<i64>, List<i64>), p26: List<List<i64>>) -> R<(List<i64>, List<i64>)> {
    let m24 = (p25, p26);
    {
        let mut m24 = m24;
        loop {
            let step = {
                let (v1, v2) = m24;
                let m22 = v2;
                match m22.uncons() {
                    None => {
                        Err::<((List<i64>, List<i64>), List<List<i64>>), R<(List<i64>, List<i64>)>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<((List<i64>, List<i64>), List<List<i64>>), R<(List<i64>, List<i64>)>>(({
                            let c23: Fn1 = v0.clone();
                            c23.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m24 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f14(v0: List<i64>, v1: (List<i64>, List<i64>), v2: List<i64>) -> R<(List<i64>, List<i64>)> {
    let m28 = {
        let (_, h27) = v1.clone();
        h27
    };
    match m28.uncons() {
        None => {
            Ok(v1)
        }
        Some((v3, v4)) => {
            Ok(({
                let a34 = {
                    let (h29, _) = v1;
                    h29
                };
                let a35 = List::cons({
                    let a32 = v3;
                    let a33 = {
                        let a31 = {
                            let (h30, _) = f24(Fn3::F25, (0i64, v0), v2)?;
                            h30
                        };
                        int_neg(a31)?
                    };
                    int_sub(a32, a33)?
                }, List::<i64>::nil());
                append_list(a34, a35)
            }, v4))
        }
    }
}

pub fn f15() -> List<i64> {
    List::cons(-6i64, List::cons(-2i64, List::cons(-5i64, List::cons(-5i64, List::cons(-4i64, List::cons(-5i64, List::cons(-6i64, List::cons(-3i64, List::cons(-7i64, List::cons(-6i64, List::<i64>::nil()))))))))))
}

pub fn f16() -> List<List<i64>> {
    List::cons(List::cons(2i64, List::cons(2i64, List::cons(2i64, List::cons(2i64, List::cons(2i64, List::cons(2i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::<i64>::nil())))))))))))), List::cons(List::cons(0i64, List::cons(2i64, List::cons(2i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::<i64>::nil())))))))))))), List::cons(List::cons(2i64, List::cons(2i64, List::cons(0i64, List::cons(2i64, List::cons(2i64, List::cons(0i64, List::cons(2i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::<i64>::nil())))))))))))), List::cons(List::cons(2i64, List::cons(2i64, List::cons(2i64, List::cons(2i64, List::cons(0i64, List::cons(0i64, List::cons(2i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::<i64>::nil())))))))))))), List::cons(List::cons(0i64, List::cons(2i64, List::cons(2i64, List::cons(0i64, List::cons(0i64, List::cons(2i64, List::cons(2i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::<i64>::nil())))))))))))), List::cons(List::cons(2i64, List::cons(0i64, List::cons(2i64, List::cons(2i64, List::cons(0i64, List::cons(2i64, List::cons(2i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::<i64>::nil())))))))))))), List::cons(List::cons(2i64, List::cons(0i64, List::cons(2i64, List::cons(2i64, List::cons(2i64, List::cons(2i64, List::cons(2i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::<i64>::nil())))))))))))), List::cons(List::cons(2i64, List::cons(2i64, List::cons(2i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::<i64>::nil())))))))))))), List::cons(List::cons(2i64, List::cons(2i64, List::cons(2i64, List::cons(2i64, List::cons(2i64, List::cons(2i64, List::cons(2i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::<i64>::nil())))))))))))), List::cons(List::cons(2i64, List::cons(2i64, List::cons(2i64, List::cons(2i64, List::cons(0i64, List::cons(2i64, List::cons(2i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::<i64>::nil())))))))))))), List::<List<i64>>::nil()))))))))))
}

pub fn f17(v0: Fn2, p39: List<i64>, p40: List<i64>) -> R<List<i64>> {
    let m38 = (p39, p40);
    {
        let mut m38 = m38;
        loop {
            let step = {
                let (v1, v2) = m38;
                let m36 = v2;
                match m36.uncons() {
                    None => {
                        Err::<(List<i64>, List<i64>), R<List<i64>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<i64>, List<i64>), R<List<i64>>>(({
                            let c37: Fn2 = v0.clone();
                            c37.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m38 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f18(v0: List<i64>, v1: i64) -> R<List<i64>> {
    let a50 = v0;
    let a51 = List::cons({
        let v2: i64 = {
            let a41 = v1;
            let a42 = 4i64;
            let a43 = 0i64;
            int_quot(a41, a42, a43)?
        };
        let m46 = {
            let a44 = v2;
            let a45 = 0i64;
            compare(a44, a45)
        };
        if match m46 {
            Ordering::Less => {
                true
            }
            Ordering::Equal => {
                false
            }
            Ordering::Greater => {
                false
            }
        } {
            0i64
        } else {
            let m49 = {
                let a47 = 127i64;
                let a48 = v2;
                compare(a47, a48)
            };
            if match m49 {
                Ordering::Less => {
                    true
                }
                Ordering::Equal => {
                    false
                }
                Ordering::Greater => {
                    false
                }
            } {
                127i64
            } else {
                v2
            }
        }
    }, List::<i64>::nil());
    Ok(append_list(a50, a51))
}

pub fn f19(v0: List<i64>, v1: i64) -> List<i64> {
    let a55 = v0;
    let a56 = List::cons({
        let m54 = {
            let a52 = v1;
            let a53 = 0i64;
            compare(a52, a53)
        };
        if match m54 {
            Ordering::Less => {
                true
            }
            Ordering::Equal => {
                false
            }
            Ordering::Greater => {
                false
            }
        } {
            0i64
        } else {
            v1
        }
    }, List::<i64>::nil());
    append_list(a55, a56)
}

pub fn f20(v0: List<i64>, v1: (List<i64>, List<i64>), v2: List<i64>) -> R<(List<i64>, List<i64>)> {
    let m58 = {
        let (_, h57) = v1.clone();
        h57
    };
    match m58.uncons() {
        None => {
            Ok(v1)
        }
        Some((v3, v4)) => {
            Ok(({
                let a64 = {
                    let (h59, _) = v1;
                    h59
                };
                let a65 = List::cons({
                    let a62 = v3;
                    let a63 = {
                        let a61 = {
                            let (h60, _) = f24(Fn3::F26, (0i64, v0), v2)?;
                            h60
                        };
                        int_neg(a61)?
                    };
                    int_sub(a62, a63)?
                }, List::<i64>::nil());
                append_list(a64, a65)
            }, v4))
        }
    }
}

pub fn f21() -> List<i64> {
    List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(1i64, List::cons(-2i64, List::cons(3i64, List::cons(-5i64, List::cons(2i64, List::<i64>::nil()))))))))))))
}

pub fn f22() -> List<List<i64>> {
    List::cons(List::cons(4i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::<i64>::nil()))))))), List::cons(List::cons(0i64, List::cons(4i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::<i64>::nil()))))))), List::cons(List::cons(0i64, List::cons(0i64, List::cons(4i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::<i64>::nil()))))))), List::cons(List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(4i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::<i64>::nil()))))))), List::cons(List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(4i64, List::cons(0i64, List::cons(0i64, List::<i64>::nil()))))))), List::cons(List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(4i64, List::cons(0i64, List::<i64>::nil()))))))), List::cons(List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(4i64, List::<i64>::nil()))))))), List::cons(List::cons(3i64, List::cons(-2i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(-1i64, List::<i64>::nil()))))))), List::cons(List::cons(0i64, List::cons(0i64, List::cons(2i64, List::cons(-3i64, List::cons(1i64, List::cons(0i64, List::cons(0i64, List::<i64>::nil()))))))), List::cons(List::cons(-1i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(2i64, List::cons(2i64, List::cons(-2i64, List::<i64>::nil()))))))), List::cons(List::cons(1i64, List::cons(1i64, List::cons(1i64, List::cons(1i64, List::cons(1i64, List::cons(1i64, List::cons(1i64, List::<i64>::nil()))))))), List::cons(List::cons(0i64, List::cons(-4i64, List::cons(0i64, List::cons(2i64, List::cons(0i64, List::cons(3i64, List::cons(0i64, List::<i64>::nil()))))))), List::<List<i64>>::nil()))))))))))))
}

pub fn f23(v0: Adt0) -> List<i64> {
    let m66 = v0;
    match m66 {
        Adt0::C0 => {
            List::cons(1i64, List::cons(1i64, List::cons(1i64, List::cons(1i64, List::cons(1i64, List::cons(1i64, List::cons(0i64, List::<i64>::nil())))))))
        }
        Adt0::C1 => {
            List::cons(0i64, List::cons(1i64, List::cons(1i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::<i64>::nil())))))))
        }
        Adt0::C2 => {
            List::cons(1i64, List::cons(1i64, List::cons(0i64, List::cons(1i64, List::cons(1i64, List::cons(0i64, List::cons(1i64, List::<i64>::nil())))))))
        }
        Adt0::C3 => {
            List::cons(1i64, List::cons(1i64, List::cons(1i64, List::cons(1i64, List::cons(0i64, List::cons(0i64, List::cons(1i64, List::<i64>::nil())))))))
        }
        Adt0::C4 => {
            List::cons(0i64, List::cons(1i64, List::cons(1i64, List::cons(0i64, List::cons(0i64, List::cons(1i64, List::cons(1i64, List::<i64>::nil())))))))
        }
        Adt0::C5 => {
            List::cons(1i64, List::cons(0i64, List::cons(1i64, List::cons(1i64, List::cons(0i64, List::cons(1i64, List::cons(1i64, List::<i64>::nil())))))))
        }
        Adt0::C6 => {
            List::cons(1i64, List::cons(0i64, List::cons(1i64, List::cons(1i64, List::cons(1i64, List::cons(1i64, List::cons(1i64, List::<i64>::nil())))))))
        }
        Adt0::C7 => {
            List::cons(1i64, List::cons(1i64, List::cons(1i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::<i64>::nil())))))))
        }
        Adt0::C8 => {
            List::cons(1i64, List::cons(1i64, List::cons(1i64, List::cons(1i64, List::cons(1i64, List::cons(1i64, List::cons(1i64, List::<i64>::nil())))))))
        }
        Adt0::C9 => {
            List::cons(1i64, List::cons(1i64, List::cons(1i64, List::cons(1i64, List::cons(0i64, List::cons(1i64, List::cons(1i64, List::<i64>::nil())))))))
        }
    }
}

pub fn f24(v0: Fn3, p70: (i64, List<i64>), p71: List<i64>) -> R<(i64, List<i64>)> {
    let m69 = (p70, p71);
    {
        let mut m69 = m69;
        loop {
            let step = {
                let (v1, v2) = m69;
                let m67 = v2;
                match m67.uncons() {
                    None => {
                        Err::<((i64, List<i64>), List<i64>), R<(i64, List<i64>)>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<((i64, List<i64>), List<i64>), R<(i64, List<i64>)>>(({
                            let c68: Fn3 = v0.clone();
                            c68.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m69 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f25(v0: (i64, List<i64>), v1: i64) -> R<(i64, List<i64>)> {
    let m73 = {
        let (_, h72) = v0.clone();
        h72
    };
    match m73.uncons() {
        None => {
            Ok(v0)
        }
        Some((v2, v3)) => {
            Ok(({
                let a78 = {
                    let (h74, _) = v0;
                    h74
                };
                let a79 = {
                    let a77 = {
                        let a75 = v1;
                        let a76 = v2;
                        int_mul(a75, a76)?
                    };
                    int_neg(a77)?
                };
                int_sub(a78, a79)?
            }, v3))
        }
    }
}

pub fn f26(v0: (i64, List<i64>), v1: i64) -> R<(i64, List<i64>)> {
    let m81 = {
        let (_, h80) = v0.clone();
        h80
    };
    match m81.uncons() {
        None => {
            Ok(v0)
        }
        Some((v2, v3)) => {
            Ok(({
                let a86 = {
                    let (h82, _) = v0;
                    h82
                };
                let a87 = {
                    let a85 = {
                        let a83 = v1;
                        let a84 = v2;
                        int_mul(a83, a84)?
                    };
                    int_neg(a85)?
                };
                int_sub(a86, a87)?
            }, v3))
        }
    }
}
