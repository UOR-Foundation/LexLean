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
    C0(List<Adt7>, List<Adt16>, Adt2, bool),
}

#[derive(Clone)]
pub enum Adt16 {
    C0(Bytes, bool, Adt2),
}

#[derive(Clone)]
pub enum Adt17 {
    C0(List<Bytes>, Option<Bytes>),
}

#[derive(Clone)]
pub enum Fn0 {
    F3(Adt14),
}

impl Fn0 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Adt1) -> R<Option<Bytes>> {
        match self {
            Fn0::F3(k0) => f3(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F5,
}

impl Fn1 {
    pub fn apply(&self, p0: Result<Adt14, Bytes>, p1: Adt1) -> R<Result<Adt14, Bytes>> {
        match self {
            Fn1::F5 => f5(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F16(Adt14),
    F75(List<Adt8>, Bytes),
}

impl Fn2 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Adt5) -> R<Option<Bytes>> {
        match self {
            Fn2::F16(k0) => f16(k0.clone(), p0, p1),
            Fn2::F75(k0, k1) => f75(k0.clone(), k1.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F20(List<Adt7>, Adt14, Adt2),
    F76(List<Adt7>, Adt14, List<Adt5>, Bytes, Adt2),
}

impl Fn3 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Adt8) -> R<Option<Bytes>> {
        match self {
            Fn3::F20(k0, k1, k2) => f20(k0.clone(), k1.clone(), k2.clone(), p0, p1),
            Fn3::F76(k0, k1, k2, k3, k4) => f76(k0.clone(), k1.clone(), k2.clone(), k3.clone(), k4.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F23(List<Bytes>, Adt14),
}

impl Fn4 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Adt9) -> R<Option<Bytes>> {
        match self {
            Fn4::F23(k0, k1) => f23(k0.clone(), k1.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F70,
}

impl Fn5 {
    pub fn apply(&self, p0: Adt17, p1: Adt5) -> R<Adt17> {
        match self {
            Fn5::F70 => f70(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F73(Adt14),
}

impl Fn6 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Adt7) -> R<Option<Bytes>> {
        match self {
            Fn6::F73(k0) => f73(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F79,
}

impl Fn7 {
    pub fn apply(&self, p0: Bytes, p1: Bytes) -> Bytes {
        match self {
            Fn7::F79 => f79(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F82(Str),
    F83(Str),
    F86(Str),
    F124(Adt7, Adt14),
    F134(Bytes),
    F173(List<Bytes>, List<Bytes>),
}

impl Fn8 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Bytes) -> R<Option<Bytes>> {
        match self {
            Fn8::F82(k0) => f82(k0.clone(), p0, p1),
            Fn8::F83(k0) => f83(k0.clone(), p0, p1),
            Fn8::F86(k0) => Ok(f86(k0.clone(), p0, p1)),
            Fn8::F124(k0, k1) => Ok(f124(k0.clone(), k1.clone(), p0, p1)),
            Fn8::F134(k0) => Ok(f134(k0.clone(), p0, p1)),
            Fn8::F173(k0, k1) => f173(k0.clone(), k1.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F88,
}

impl Fn9 {
    pub fn apply(&self, p0: Adt14, p1: Adt5) -> Adt14 {
        match self {
            Fn9::F88 => f88(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F92,
    F137(Bytes),
}

impl Fn10 {
    pub fn apply(&self, p0: Adt14, p1: Adt8) -> Adt14 {
        match self {
            Fn10::F92 => f92(p0, p1),
            Fn10::F137(k0) => f137(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F95(Bytes),
    F96,
}

impl Fn11 {
    pub fn apply(&self, p0: Adt17, p1: Adt9) -> R<Adt17> {
        match self {
            Fn11::F95(k0) => f95(k0.clone(), p0, p1),
            Fn11::F96 => f96(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F103(Bytes),
}

impl Fn12 {
    pub fn apply(&self, p0: Option<Adt16>, p1: Adt16) -> Option<Adt16> {
        match self {
            Fn12::F103(k0) => f103(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F105(Bytes),
    F122(List<Bytes>, List<Bytes>),
    F200(Bytes, Adt14),
}

impl Fn13 {
    pub fn apply(&self, p0: bool, p1: Bytes) -> R<bool> {
        match self {
            Fn13::F105(k0) => Ok(f105(k0.clone(), p0, p1)),
            Fn13::F122(k0, k1) => f122(k0.clone(), k1.clone(), p0, p1),
            Fn13::F200(k0, k1) => Ok(f200(k0.clone(), k1.clone(), p0, p1)),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F133(Bytes),
}

impl Fn14 {
    pub fn apply(&self, p0: Result<List<Adt16>, Bytes>, p1: Adt6) -> Result<List<Adt16>, Bytes> {
        match self {
            Fn14::F133(k0) => f133(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F136(Bytes),
    F189(Bytes),
}

impl Fn15 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn15::F136(k0) => f136(k0.clone(), p0),
            Fn15::F189(k0) => f189(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F143(Bytes),
}

impl Fn16 {
    pub fn apply(&self, p0: Option<Adt11>, p1: Adt11) -> Option<Adt11> {
        match self {
            Fn16::F143(k0) => f143(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F145(Bytes),
}

impl Fn17 {
    pub fn apply(&self, p0: Option<Adt12>, p1: Adt12) -> Option<Adt12> {
        match self {
            Fn17::F145(k0) => f145(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F147(Bytes),
    F195(Bytes),
}

impl Fn18 {
    pub fn apply(&self, p0: Option<Adt13>, p1: Adt13) -> Option<Adt13> {
        match self {
            Fn18::F147(k0) => f147(k0.clone(), p0, p1),
            Fn18::F195(k0) => f195(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F160(Bytes),
}

impl Fn19 {
    pub fn apply(&self, p0: List<u64>, p1: Adt13) -> List<u64> {
        match self {
            Fn19::F160(k0) => f160(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn20 {
    F162(u64),
}

impl Fn20 {
    pub fn apply(&self, p0: bool, p1: u64) -> bool {
        match self {
            Fn20::F162(k0) => f162(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn21 {
    F168,
}

impl Fn21 {
    pub fn apply(&self, p0: Bytes, p1: u64) -> R<Bytes> {
        match self {
            Fn21::F168 => f168(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn22 {
    F176(bool, u64, Adt14),
}

impl Fn22 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Adt6) -> R<Option<Bytes>> {
        match self {
            Fn22::F176(k0, k1, k2) => f176(*k0, *k1, k2.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn23 {
    F178(Bytes),
}

impl Fn23 {
    pub fn apply(&self, p0: Option<Adt10>, p1: Adt10) -> Option<Adt10> {
        match self {
            Fn23::F178(k0) => f178(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn24 {
    F180(Bytes),
}

impl Fn24 {
    pub fn apply(&self, p0: u64, p1: Adt8) -> R<u64> {
        match self {
            Fn24::F180(k0) => f180(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn25 {
    F182(Bytes),
}

impl Fn25 {
    pub fn apply(&self, p0: Option<Adt8>, p1: Adt8) -> Option<Adt8> {
        match self {
            Fn25::F182(k0) => f182(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn26 {
    F184,
}

impl Fn26 {
    pub fn apply(&self, p0: Bytes, p1: Adt6) -> R<Bytes> {
        match self {
            Fn26::F184 => f184(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn27 {
    F187(Bytes),
    F207(Bytes),
}

impl Fn27 {
    pub fn apply(&self, p0: bool, p1: Adt5) -> bool {
        match self {
            Fn27::F187(k0) => f187(k0.clone(), p0, p1),
            Fn27::F207(k0) => f207(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn28 {
    F192(Bytes),
}

impl Fn28 {
    pub fn apply(&self, p0: Option<Adt9>, p1: Adt9) -> Option<Adt9> {
        match self {
            Fn28::F192(k0) => f192(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn29 {
    F203(Bytes),
}

impl Fn29 {
    pub fn apply(&self, p0: u64, p1: Bytes) -> R<u64> {
        match self {
            Fn29::F203(k0) => f203(k0.clone(), p0, p1),
        }
    }
}

pub fn f0(v0: Adt0) -> R<Result<(), Bytes>> {
    let m3 = f1(Adt14::C0(match v0.clone() {
        Adt0::C0(h1, _) => {
            h1
        }
    }, List::<Bytes>::nil(), List::<Adt10>::nil(), List::<Adt11>::nil(), List::<Adt12>::nil(), List::<Adt13>::nil(), List::<Bytes>::nil(), List::<Adt13>::nil(), List::<Adt13>::nil(), List::<Bytes>::nil()), match v0.clone() {
        Adt0::C0(_, h2) => {
            h2
        }
    })?;
    match m3 {
        Ok(v2) => {
            let m5 = f2(Fn0::F3(v2), None::<Bytes>, match v0 {
                Adt0::C0(_, h4) => {
                    h4
                }
            })?;
            match m5 {
                None => {
                    Ok(Ok::<(), Bytes>(()))
                }
                Some(v3) => {
                    Ok(Err::<(), Bytes>(v3))
                }
            }
        }
        Err(v1) => {
            Ok(Err::<(), Bytes>(v1))
        }
    }
}

pub fn f1(v0: Adt14, v1: List<Adt1>) -> R<Result<Adt14, Bytes>> {
    f4(Fn1::F5, Ok::<Adt14, Bytes>(v0), v1)
}

pub fn f2(v0: Fn0, p9: Option<Bytes>, p10: List<Adt1>) -> R<Option<Bytes>> {
    let m8 = (p9, p10);
    {
        let mut m8 = m8;
        loop {
            let step = {
                let (v1, v2) = m8;
                let m6 = v2;
                match m6.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Adt1>), R<Option<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Adt1>), R<Option<Bytes>>>(({
                            let c7: Fn0 = v0.clone();
                            c7.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m8 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f3(v0: Adt14, v1: Option<Bytes>, v2: Adt1) -> R<Option<Bytes>> {
    let m11 = v1;
    match m11 {
        None => {
            f6(v0, v2)
        }
        Some(v3) => {
            Ok(Some(v3))
        }
    }
}

pub fn f4(v0: Fn1, p15: Result<Adt14, Bytes>, p16: List<Adt1>) -> R<Result<Adt14, Bytes>> {
    let m14 = (p15, p16);
    {
        let mut m14 = m14;
        loop {
            let step = {
                let (v1, v2) = m14;
                let m12 = v2;
                match m12.uncons() {
                    None => {
                        Err::<(Result<Adt14, Bytes>, List<Adt1>), R<Result<Adt14, Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Result<Adt14, Bytes>, List<Adt1>), R<Result<Adt14, Bytes>>>(({
                            let c13: Fn1 = v0.clone();
                            c13.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m14 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f5(v0: Result<Adt14, Bytes>, v1: Adt1) -> R<Result<Adt14, Bytes>> {
    let m17 = v0;
    match m17 {
        Ok(v3) => {
            f7(v3, v1)
        }
        Err(v2) => {
            Ok(Err::<Adt14, Bytes>(v2))
        }
    }
}

pub fn f6(v0: Adt14, v1: Adt1) -> R<Option<Bytes>> {
    let m18 = v1;
    match m18 {
        Adt1::C0(_, _) => {
            Ok(None::<Bytes>)
        }
        Adt1::C1(_, _) => {
            Ok(None::<Bytes>)
        }
        Adt1::C2(_, v7, v8) => {
            f8(v0, v8, {
                let a19 = v7;
                length_list(a19)
            }, false)
        }
        Adt1::C3(_, v10, v11) => {
            let m20 = f8(v0.clone(), v10, 0u64, false)?;
            match m20 {
                None => {
                    f9(v0, f10(()), v11)
                }
                Some(v12) => {
                    Ok(Some(v12))
                }
            }
        }
        Adt1::C4(v13, v14) => {
            let m22 = {
                let m21 = f11(v14.clone())?;
                match m21 {
                    None => {
                        None::<Bytes>
                    }
                    Some(v15) => {
                        Some(f12(f13(Str::lit("runtime support: trait-method: ")), f12(f14(v13), f12(f13(Str::lit(" declares ")), f12(f14(v15), f13(Str::lit(" twice")))))))
                    }
                }
            };
            match m22 {
                None => {
                    f15(Fn2::F16(v0), None::<Bytes>, v14)
                }
                Some(v16) => {
                    Ok(Some(v16))
                }
            }
        }
        Adt1::C5(v17, v18, v19, v20) => {
            let m23 = f17(v0.clone(), v17.clone())?;
            match m23 {
                None => {
                    let m25 = f8(v0.clone(), v19.clone(), {
                        let a24 = v17.clone();
                        length_list(a24)
                    }, false)?;
                    match m25 {
                        None => {
                            let m26 = v18;
                            match m26.uncons() {
                                None => {
                                    f19(Fn3::F20(v17, v0, v19), None::<Bytes>, v20)
                                }
                                Some((v23, _)) => {
                                    f18(v0, v23, v17, v19, v20)
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
            let a28 = match v25.clone() {
                Adt8::C0(_, _, h27, _) => {
                    h27
                }
            };
            if bool_not(a28) {
                Ok(Some(f12(f13(Str::lit("runtime support: function-visibility: ")), f12(f14({
                    let m31 = match v25 {
                        Adt8::C0(h30, _, _, _) => {
                            h30
                        }
                    };
                    match m31 {
                        Adt5::C0(h29, _, _, _, _) => {
                            h29
                        }
                    }
                }), f13(Str::lit(" is a free function that is not public"))))))
            } else {
                let m34 = match v25.clone() {
                    Adt8::C0(h33, _, _, _) => {
                        h33
                    }
                };
                if match m34 {
                    Adt5::C0(_, h32, _, _, _) => {
                        h32
                    }
                } {
                    Ok(Some(f12(f13(Str::lit("runtime support: function-receiver: ")), f12(f13(Str::lit("free function ")), f12(f14({
                        let m37 = match v25 {
                            Adt8::C0(h36, _, _, _) => {
                                h36
                            }
                        };
                        match m37 {
                            Adt5::C0(h35, _, _, _, _) => {
                                h35
                            }
                        }
                    }), f13(Str::lit(" takes `&self`")))))))
                } else {
                    f21(v0, List::<Adt7>::nil(), Adt2::C17, v25)
                }
            }
        }
        Adt1::C7(_, v27, _, v29) => {
            f8(v0, v29, {
                let a38 = v27;
                length_list(a38)
            }, false)
        }
        Adt1::C8(_, v31, _, _, v34) => {
            f8(v0, v34, {
                let a39 = v31;
                length_list(a39)
            }, false)
        }
        Adt1::C9(_, v36, _, v38) => {
            f22(Fn4::F23(v36, v0), None::<Bytes>, v38)
        }
    }
}

pub fn f7(v0: Adt14, v1: Adt1) -> R<Result<Adt14, Bytes>> {
    let m40 = v1;
    match m40 {
        Adt1::C0(v2, v3) => {
            Ok(f24({
                let m41 = v2.clone();
                match m41.uncons() {
                    None => {
                        Err::<Adt14, Bytes>(f13(Str::lit("runtime support: use-path: empty")))
                    }
                    Some((v4, _)) => {
                        if if f25(v4.clone(), f13(Str::lit("core"))) {
                            true
                        } else {
                            if match v0.clone() {
                                Adt14::C0(h42, _, _, _, _, _, _, _, _, _) => {
                                    h42
                                }
                            } {
                                false
                            } else {
                                f25(v4.clone(), f13(Str::lit("std")))
                            }
                        } {
                            Ok::<Adt14, Bytes>(Adt14::C0(match v0.clone() {
                                Adt14::C0(h43, _, _, _, _, _, _, _, _, _) => {
                                    h43
                                }
                            }, match v0.clone() {
                                Adt14::C0(_, h44, _, _, _, _, _, _, _, _) => {
                                    h44
                                }
                            }, match v0.clone() {
                                Adt14::C0(_, _, h45, _, _, _, _, _, _, _) => {
                                    h45
                                }
                            }, match v0.clone() {
                                Adt14::C0(_, _, _, h46, _, _, _, _, _, _) => {
                                    h46
                                }
                            }, match v0.clone() {
                                Adt14::C0(_, _, _, _, h47, _, _, _, _, _) => {
                                    h47
                                }
                            }, match v0.clone() {
                                Adt14::C0(_, _, _, _, _, h48, _, _, _, _) => {
                                    h48
                                }
                            }, match v0.clone() {
                                Adt14::C0(_, _, _, _, _, _, h49, _, _, _) => {
                                    h49
                                }
                            }, match v0.clone() {
                                Adt14::C0(_, _, _, _, _, _, _, h50, _, _) => {
                                    h50
                                }
                            }, match v0.clone() {
                                Adt14::C0(_, _, _, _, _, _, _, _, h51, _) => {
                                    h51
                                }
                            }, {
                                let a54 = match v0 {
                                    Adt14::C0(_, _, _, _, _, _, _, _, _, h52) => {
                                        h52
                                    }
                                };
                                let a55 = List::cons({
                                    let m53 = v3.clone();
                                    match m53.uncons() {
                                        None => {
                                            f26(v2.clone())
                                        }
                                        Some((v6, _)) => {
                                            v6
                                        }
                                    }
                                }, List::<Bytes>::nil());
                                append_list(a54, a55)
                            }))
                        } else {
                            Err::<Adt14, Bytes>(f12(f13(Str::lit("runtime support: profile-root: ")), f12(f14(f12(f13(Str::lit("use ")), f27(v2.clone(), Str::lit("::"))?)), f12(f13(Str::lit(" starts at ")), f12(f14(v4), f13(Str::lit(", which the profile does not carry")))))))
                        }
                    }
                }
            }, f28(f29(Str::lit("a `use` segment"), v2)?, f30(Str::lit("a `use` alias"), v3)?)))
        }
        Adt1::C1(v8, v9) => {
            Ok(f31(f32(Str::lit("a unit struct name"), v8.clone())?, f24({
                let m56 = f33(v0, v8.clone())?;
                match m56 {
                    Ok(v11) => {
                        Ok::<Adt14, Bytes>(Adt14::C0(match v11.clone() {
                            Adt14::C0(h57, _, _, _, _, _, _, _, _, _) => {
                                h57
                            }
                        }, match v11.clone() {
                            Adt14::C0(_, h58, _, _, _, _, _, _, _, _) => {
                                h58
                            }
                        }, match v11.clone() {
                            Adt14::C0(_, _, h59, _, _, _, _, _, _, _) => {
                                h59
                            }
                        }, {
                            let a61 = match v11.clone() {
                                Adt14::C0(_, _, _, h60, _, _, _, _, _, _) => {
                                    h60
                                }
                            };
                            let a62 = List::cons(Adt11::C0(v8, List::<Bytes>::nil(), 0u64), List::<Adt11>::nil());
                            append_list(a61, a62)
                        }, match v11.clone() {
                            Adt14::C0(_, _, _, _, h63, _, _, _, _, _) => {
                                h63
                            }
                        }, match v11.clone() {
                            Adt14::C0(_, _, _, _, _, h64, _, _, _, _) => {
                                h64
                            }
                        }, match v11.clone() {
                            Adt14::C0(_, _, _, _, _, _, h65, _, _, _) => {
                                h65
                            }
                        }, match v11.clone() {
                            Adt14::C0(_, _, _, _, _, _, _, h66, _, _) => {
                                h66
                            }
                        }, match v11.clone() {
                            Adt14::C0(_, _, _, _, _, _, _, _, h67, _) => {
                                h67
                            }
                        }, match v11 {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, h68) => {
                                h68
                            }
                        }))
                    }
                    Err(v10) => {
                        Err::<Adt14, Bytes>(v10)
                    }
                }
            }, f34(Str::lit("a unit struct"), v9)?)))
        }
        Adt1::C2(v12, v13, _) => {
            Ok(f31(f32(Str::lit("an alias name"), v12.clone())?, f24({
                let m69 = f33(v0, v12.clone())?;
                match m69 {
                    Ok(v16) => {
                        Ok::<Adt14, Bytes>(Adt14::C0(match v16.clone() {
                            Adt14::C0(h70, _, _, _, _, _, _, _, _, _) => {
                                h70
                            }
                        }, match v16.clone() {
                            Adt14::C0(_, h71, _, _, _, _, _, _, _, _) => {
                                h71
                            }
                        }, match v16.clone() {
                            Adt14::C0(_, _, h72, _, _, _, _, _, _, _) => {
                                h72
                            }
                        }, match v16.clone() {
                            Adt14::C0(_, _, _, h73, _, _, _, _, _, _) => {
                                h73
                            }
                        }, match v16.clone() {
                            Adt14::C0(_, _, _, _, h74, _, _, _, _, _) => {
                                h74
                            }
                        }, {
                            let a77 = match v16.clone() {
                                Adt14::C0(_, _, _, _, _, h75, _, _, _, _) => {
                                    h75
                                }
                            };
                            let a78 = List::cons(Adt13::C0(v12, {
                                let a76 = v13.clone();
                                length_list(a76)
                            }), List::<Adt13>::nil());
                            append_list(a77, a78)
                        }, match v16.clone() {
                            Adt14::C0(_, _, _, _, _, _, h79, _, _, _) => {
                                h79
                            }
                        }, match v16.clone() {
                            Adt14::C0(_, _, _, _, _, _, _, h80, _, _) => {
                                h80
                            }
                        }, match v16.clone() {
                            Adt14::C0(_, _, _, _, _, _, _, _, h81, _) => {
                                h81
                            }
                        }, match v16 {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, h82) => {
                                h82
                            }
                        }))
                    }
                    Err(v15) => {
                        Err::<Adt14, Bytes>(v15)
                    }
                }
            }, f30(Str::lit("an alias parameter"), v13)?)))
        }
        Adt1::C3(v17, _, _) => {
            Ok(f31(f32(Str::lit("a static name"), v17.clone())?, {
                let m83 = f33(v0, v17.clone())?;
                match m83 {
                    Ok(v21) => {
                        Ok::<Adt14, Bytes>(Adt14::C0(match v21.clone() {
                            Adt14::C0(h84, _, _, _, _, _, _, _, _, _) => {
                                h84
                            }
                        }, match v21.clone() {
                            Adt14::C0(_, h85, _, _, _, _, _, _, _, _) => {
                                h85
                            }
                        }, match v21.clone() {
                            Adt14::C0(_, _, h86, _, _, _, _, _, _, _) => {
                                h86
                            }
                        }, match v21.clone() {
                            Adt14::C0(_, _, _, h87, _, _, _, _, _, _) => {
                                h87
                            }
                        }, match v21.clone() {
                            Adt14::C0(_, _, _, _, h88, _, _, _, _, _) => {
                                h88
                            }
                        }, match v21.clone() {
                            Adt14::C0(_, _, _, _, _, h89, _, _, _, _) => {
                                h89
                            }
                        }, {
                            let a91 = match v21.clone() {
                                Adt14::C0(_, _, _, _, _, _, h90, _, _, _) => {
                                    h90
                                }
                            };
                            let a92 = List::cons(v17, List::<Bytes>::nil());
                            append_list(a91, a92)
                        }, match v21.clone() {
                            Adt14::C0(_, _, _, _, _, _, _, h93, _, _) => {
                                h93
                            }
                        }, match v21.clone() {
                            Adt14::C0(_, _, _, _, _, _, _, _, h94, _) => {
                                h94
                            }
                        }, match v21 {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, h95) => {
                                h95
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
            Ok(f31(f32(Str::lit("a trait name"), v22.clone())?, {
                let m96 = f33(v0, v22.clone())?;
                match m96 {
                    Ok(v25) => {
                        Ok::<Adt14, Bytes>(f35(Adt14::C0(match v25.clone() {
                            Adt14::C0(h97, _, _, _, _, _, _, _, _, _) => {
                                h97
                            }
                        }, match v25.clone() {
                            Adt14::C0(_, h98, _, _, _, _, _, _, _, _) => {
                                h98
                            }
                        }, {
                            let a100 = match v25.clone() {
                                Adt14::C0(_, _, h99, _, _, _, _, _, _, _) => {
                                    h99
                                }
                            };
                            let a101 = List::cons(Adt10::C0(v22, v23.clone()), List::<Adt10>::nil());
                            append_list(a100, a101)
                        }, match v25.clone() {
                            Adt14::C0(_, _, _, h102, _, _, _, _, _, _) => {
                                h102
                            }
                        }, match v25.clone() {
                            Adt14::C0(_, _, _, _, h103, _, _, _, _, _) => {
                                h103
                            }
                        }, match v25.clone() {
                            Adt14::C0(_, _, _, _, _, h104, _, _, _, _) => {
                                h104
                            }
                        }, match v25.clone() {
                            Adt14::C0(_, _, _, _, _, _, h105, _, _, _) => {
                                h105
                            }
                        }, match v25.clone() {
                            Adt14::C0(_, _, _, _, _, _, _, h106, _, _) => {
                                h106
                            }
                        }, match v25.clone() {
                            Adt14::C0(_, _, _, _, _, _, _, _, h107, _) => {
                                h107
                            }
                        }, match v25 {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, h108) => {
                                h108
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
            Ok(Ok::<Adt14, Bytes>(f36(f37(v0, v29.clone()), v28, v29)))
        }
        Adt1::C6(v30) => {
            Ok(f31(f32(Str::lit("a function name"), {
                let m111 = match v30.clone() {
                    Adt8::C0(h110, _, _, _) => {
                        h110
                    }
                };
                match m111 {
                    Adt5::C0(h109, _, _, _, _) => {
                        h109
                    }
                }
            })?, {
                let m115 = f33(v0, {
                    let m114 = match v30.clone() {
                        Adt8::C0(h113, _, _, _) => {
                            h113
                        }
                    };
                    match m114 {
                        Adt5::C0(h112, _, _, _, _) => {
                            h112
                        }
                    }
                })?;
                match m115 {
                    Ok(v32) => {
                        Ok::<Adt14, Bytes>(Adt14::C0(match v32.clone() {
                            Adt14::C0(h116, _, _, _, _, _, _, _, _, _) => {
                                h116
                            }
                        }, match v32.clone() {
                            Adt14::C0(_, h117, _, _, _, _, _, _, _, _) => {
                                h117
                            }
                        }, match v32.clone() {
                            Adt14::C0(_, _, h118, _, _, _, _, _, _, _) => {
                                h118
                            }
                        }, match v32.clone() {
                            Adt14::C0(_, _, _, h119, _, _, _, _, _, _) => {
                                h119
                            }
                        }, match v32.clone() {
                            Adt14::C0(_, _, _, _, h120, _, _, _, _, _) => {
                                h120
                            }
                        }, match v32.clone() {
                            Adt14::C0(_, _, _, _, _, h121, _, _, _, _) => {
                                h121
                            }
                        }, match v32.clone() {
                            Adt14::C0(_, _, _, _, _, _, h122, _, _, _) => {
                                h122
                            }
                        }, {
                            let a131 = match v32.clone() {
                                Adt14::C0(_, _, _, _, _, _, _, h123, _, _) => {
                                    h123
                                }
                            };
                            let a132 = List::cons(Adt13::C0({
                                let m126 = match v30.clone() {
                                    Adt8::C0(h125, _, _, _) => {
                                        h125
                                    }
                                };
                                match m126 {
                                    Adt5::C0(h124, _, _, _, _) => {
                                        h124
                                    }
                                }
                            }, {
                                let a130 = {
                                    let m129 = match v30 {
                                        Adt8::C0(h128, _, _, _) => {
                                            h128
                                        }
                                    };
                                    match m129 {
                                        Adt5::C0(_, _, _, h127, _) => {
                                            h127
                                        }
                                    }
                                };
                                length_list(a130)
                            }), List::<Adt13>::nil());
                            append_list(a131, a132)
                        }, match v32.clone() {
                            Adt14::C0(_, _, _, _, _, _, _, _, h133, _) => {
                                h133
                            }
                        }, match v32 {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, h134) => {
                                h134
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
            Ok(f31(f32(Str::lit("a tuple struct name"), v33.clone())?, f24({
                let m135 = f33(v0, v33.clone())?;
                match m135 {
                    Ok(v38) => {
                        Ok::<Adt14, Bytes>(Adt14::C0(match v38.clone() {
                            Adt14::C0(h136, _, _, _, _, _, _, _, _, _) => {
                                h136
                            }
                        }, match v38.clone() {
                            Adt14::C0(_, h137, _, _, _, _, _, _, _, _) => {
                                h137
                            }
                        }, match v38.clone() {
                            Adt14::C0(_, _, h138, _, _, _, _, _, _, _) => {
                                h138
                            }
                        }, {
                            let a141 = match v38.clone() {
                                Adt14::C0(_, _, _, h139, _, _, _, _, _, _) => {
                                    h139
                                }
                            };
                            let a142 = List::cons(Adt11::C0(v33, f38(v36, 0u64)?, {
                                let a140 = v34.clone();
                                length_list(a140)
                            }), List::<Adt11>::nil());
                            append_list(a141, a142)
                        }, match v38.clone() {
                            Adt14::C0(_, _, _, _, h143, _, _, _, _, _) => {
                                h143
                            }
                        }, match v38.clone() {
                            Adt14::C0(_, _, _, _, _, h144, _, _, _, _) => {
                                h144
                            }
                        }, match v38.clone() {
                            Adt14::C0(_, _, _, _, _, _, h145, _, _, _) => {
                                h145
                            }
                        }, match v38.clone() {
                            Adt14::C0(_, _, _, _, _, _, _, h146, _, _) => {
                                h146
                            }
                        }, match v38.clone() {
                            Adt14::C0(_, _, _, _, _, _, _, _, h147, _) => {
                                h147
                            }
                        }, match v38 {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, h148) => {
                                h148
                            }
                        }))
                    }
                    Err(v37) => {
                        Err::<Adt14, Bytes>(v37)
                    }
                }
            }, f28(f30(Str::lit("a tuple struct generic"), v34)?, f34(Str::lit("a tuple struct"), v35)?))))
        }
        Adt1::C8(v39, v40, v41, v42, v43) => {
            Ok(f31(f32(Str::lit("a record struct name"), v39.clone())?, {
                let m149 = f33(v0, v39.clone())?;
                match m149 {
                    Ok(v45) => {
                        let m150 = f28(f30(Str::lit("a record struct generic"), v40.clone())?, f28(f34(Str::lit("a record struct"), v41)?, f30(Str::lit("a record field name"), v42.clone())?));
                        match m150 {
                            None => {
                                let a152 = {
                                    let a151 = v42.clone();
                                    length_list(a151)
                                };
                                let a153 = f39(v43.clone())?;
                                if nat_eq(a152, a153) {
                                    Ok::<Adt14, Bytes>(Adt14::C0(match v45.clone() {
                                        Adt14::C0(h154, _, _, _, _, _, _, _, _, _) => {
                                            h154
                                        }
                                    }, match v45.clone() {
                                        Adt14::C0(_, h155, _, _, _, _, _, _, _, _) => {
                                            h155
                                        }
                                    }, match v45.clone() {
                                        Adt14::C0(_, _, h156, _, _, _, _, _, _, _) => {
                                            h156
                                        }
                                    }, {
                                        let a159 = match v45.clone() {
                                            Adt14::C0(_, _, _, h157, _, _, _, _, _, _) => {
                                                h157
                                            }
                                        };
                                        let a160 = List::cons(Adt11::C0(v39, v42, {
                                            let a158 = v40;
                                            length_list(a158)
                                        }), List::<Adt11>::nil());
                                        append_list(a159, a160)
                                    }, match v45.clone() {
                                        Adt14::C0(_, _, _, _, h161, _, _, _, _, _) => {
                                            h161
                                        }
                                    }, match v45.clone() {
                                        Adt14::C0(_, _, _, _, _, h162, _, _, _, _) => {
                                            h162
                                        }
                                    }, match v45.clone() {
                                        Adt14::C0(_, _, _, _, _, _, h163, _, _, _) => {
                                            h163
                                        }
                                    }, match v45.clone() {
                                        Adt14::C0(_, _, _, _, _, _, _, h164, _, _) => {
                                            h164
                                        }
                                    }, match v45.clone() {
                                        Adt14::C0(_, _, _, _, _, _, _, _, h165, _) => {
                                            h165
                                        }
                                    }, match v45 {
                                        Adt14::C0(_, _, _, _, _, _, _, _, _, h166) => {
                                            h166
                                        }
                                    }))
                                } else {
                                    Err::<Adt14, Bytes>(f12(f13(Str::lit("runtime support: struct-fields: ")), f12(f14(v39), f12(f13(Str::lit(" has ")), f12(f40({
                                        let a167 = v42;
                                        length_list(a167)
                                    })?, f12(f13(Str::lit(" field names and ")), f12(f40(f39(v43)?)?, f13(Str::lit(" types")))))))))
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
        Adt1::C9(v47, v48, v49, v50) => {
            Ok(f31(f32(Str::lit("an enum name"), v47.clone())?, {
                let m168 = f33(v0, v47.clone())?;
                match m168 {
                    Ok(v52) => {
                        let m169 = f28(f30(Str::lit("an enum generic"), v48.clone())?, f28(f34(Str::lit("an enum"), v49)?, f41(v47.clone(), v50.clone())?));
                        match m169 {
                            None => {
                                let m170 = f42(v50.clone())?;
                                match m170 {
                                    None => {
                                        Ok::<Adt14, Bytes>(Adt14::C0(match v52.clone() {
                                            Adt14::C0(h171, _, _, _, _, _, _, _, _, _) => {
                                                h171
                                            }
                                        }, match v52.clone() {
                                            Adt14::C0(_, h172, _, _, _, _, _, _, _, _) => {
                                                h172
                                            }
                                        }, match v52.clone() {
                                            Adt14::C0(_, _, h173, _, _, _, _, _, _, _) => {
                                                h173
                                            }
                                        }, match v52.clone() {
                                            Adt14::C0(_, _, _, h174, _, _, _, _, _, _) => {
                                                h174
                                            }
                                        }, {
                                            let a177 = match v52.clone() {
                                                Adt14::C0(_, _, _, _, h175, _, _, _, _, _) => {
                                                    h175
                                                }
                                            };
                                            let a178 = List::cons(Adt12::C0(v47, v50, {
                                                let a176 = v48;
                                                length_list(a176)
                                            }), List::<Adt12>::nil());
                                            append_list(a177, a178)
                                        }, match v52.clone() {
                                            Adt14::C0(_, _, _, _, _, h179, _, _, _, _) => {
                                                h179
                                            }
                                        }, match v52.clone() {
                                            Adt14::C0(_, _, _, _, _, _, h180, _, _, _) => {
                                                h180
                                            }
                                        }, match v52.clone() {
                                            Adt14::C0(_, _, _, _, _, _, _, h181, _, _) => {
                                                h181
                                            }
                                        }, match v52.clone() {
                                            Adt14::C0(_, _, _, _, _, _, _, _, h182, _) => {
                                                h182
                                            }
                                        }, match v52 {
                                            Adt14::C0(_, _, _, _, _, _, _, _, _, h183) => {
                                                h183
                                            }
                                        }))
                                    }
                                    Some(v54) => {
                                        Err::<Adt14, Bytes>(f12(f13(Str::lit("runtime support: variant-name: ")), f12(f14(v47), f12(f13(Str::lit(" declares ")), f12(f14(v54), f13(Str::lit(" twice")))))))
                                    }
                                }
                            }
                            Some(v53) => {
                                Err::<Adt14, Bytes>(v53)
                            }
                        }
                    }
                    Err(v51) => {
                        Err::<Adt14, Bytes>(v51)
                    }
                }
            }))
        }
    }
}

pub fn f8(v0: Adt14, v1: Adt2, v2: u64, v3: bool) -> R<Option<Bytes>> {
    let m184 = v1;
    match m184 {
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
                Ok(Some(f12(f13(Str::lit("runtime support: self-type: ")), f13(Str::lit("`Self` outside a trait or an impl")))))
            }
        }
        Adt2::C6(v4) => {
            let a185 = v4;
            let a186 = v2;
            if nat_lt(a185, a186) {
                Ok(None::<Bytes>)
            } else {
                Ok(Some(f12(f13(Str::lit("runtime support: type-parameter: ")), f12(f13(Str::lit("parameter ")), f12(f40(v4)?, f12(f13(Str::lit(" of ")), f12(f40(v2)?, f13(Str::lit(" in scope")))))))))
            }
        }
        Adt2::C7(r193) => {
            let v5 = (*r193).clone();
            f8(v0, v5, v2, v3)
        }
        Adt2::C8(r194) => {
            let v6 = (*r194).clone();
            f8(v0, v6, v2, v3)
        }
        Adt2::C9(r195, r196) => {
            let v7 = (*r195).clone();
            let v8 = (*r196).clone();
            let m187 = f8(v0.clone(), v7, v2, v3)?;
            match m187 {
                None => {
                    f8(v0, v8, v2, v3)
                }
                Some(v9) => {
                    Ok(Some(v9))
                }
            }
        }
        Adt2::C10(v10, r197) => {
            let v11 = (*r197).clone();
            let m188 = f43(v0.clone(), v10.clone());
            match m188 {
                None => {
                    Ok(Some(f12(f13(Str::lit("runtime support: type-name: ")), f12(f14(v10), f13(Str::lit(" is neither declared nor in the closed table"))))))
                }
                Some(v12) => {
                    if if match v0.clone() {
                        Adt14::C0(h189, _, _, _, _, _, _, _, _, _) => {
                            h189
                        }
                    } {
                        f44(v0.clone(), v10.clone())
                    } else {
                        false
                    } {
                        Ok(Some(f12(f13(Str::lit("runtime support: hidden-allocation: ")), f12(f13(Str::lit("rust-core renders ")), f12(f14(v10), f13(Str::lit(", which requires heap allocation")))))))
                    } else {
                        let a190 = v12;
                        let a191 = f39(v11.clone())?;
                        if nat_eq(a190, a191) {
                            f8(v0, v11, v2, v3)
                        } else {
                            Ok(Some(f12(f13(Str::lit("runtime support: type-arity: ")), f12(f14(v10), f12(f13(Str::lit(" takes ")), f12(f40(v12)?, f12(f13(Str::lit(" arguments, given ")), f40(f39(v11)?)?)))))))
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
        Adt2::C15(r198) => {
            let v13 = (*r198).clone();
            f8(v0, v13, v2, v3)
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
        Adt2::C19(r199, r200) => {
            let v14 = (*r199).clone();
            let v15 = (*r200).clone();
            let m192 = f8(v0.clone(), v14, v2, v3)?;
            match m192 {
                None => {
                    f8(v0, v15, v2, v3)
                }
                Some(v16) => {
                    Ok(Some(v16))
                }
            }
        }
    }
}

pub fn f9(v0: Adt14, v1: Adt15, v2: Adt3) -> R<Option<Bytes>> {
    let m201 = v2;
    match m201 {
        Adt3::C0(v3) => {
            let m203 = f45(match v1 {
                Adt15::C0(_, h202, _, _) => {
                    h202
                }
            }, v3.clone());
            match m203 {
                None => {
                    if f46(match v0 {
                        Adt14::C0(_, _, _, _, _, _, h204, _, _, _) => {
                            h204
                        }
                    }, v3.clone())? {
                        Ok(None::<Bytes>)
                    } else {
                        Ok(Some(f12(f13(Str::lit("runtime support: variable-scope: ")), f12(f14(v3), f13(Str::lit(" is not in scope"))))))
                    }
                }
                Some(_) => {
                    Ok(None::<Bytes>)
                }
            }
        }
        Adt3::C1(v5) => {
            f47(v5)
        }
        Adt3::C2(v6) => {
            f48(v0, v6)
        }
        Adt3::C3(v7, r249) => {
            let v8 = (*r249).clone();
            let m205 = f49(v0.clone(), v7, f50(v8.clone())?)?;
            match m205 {
                None => {
                    f51(v0, v1, v8)
                }
                Some(v9) => {
                    Ok(Some(v9))
                }
            }
        }
        Adt3::C4(r250, v11, r251) => {
            let v10 = (*r250).clone();
            let v12 = (*r251).clone();
            let m206 = f9(v0.clone(), v1.clone(), v10.clone())?;
            match m206 {
                None => {
                    let m207 = f52(v0.clone(), v1.clone(), v10, v11, f50(v12.clone())?)?;
                    match m207 {
                        None => {
                            f51(v0, v1, v12)
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
        Adt3::C5(r252, _) => {
            let v15 = (*r252).clone();
            f9(v0, v1, v15)
        }
        Adt3::C6(r253, v18) => {
            let v17 = (*r253).clone();
            let m208 = f32(Str::lit("a field access"), v18)?;
            match m208 {
                None => {
                    f9(v0, v1, v17)
                }
                Some(v19) => {
                    Ok(Some(v19))
                }
            }
        }
        Adt3::C7(r254) => {
            let v20 = (*r254).clone();
            f9(v0, v1, v20)
        }
        Adt3::C8(r255, r256) => {
            let v21 = (*r255).clone();
            let v22 = (*r256).clone();
            let m209 = f9(v0.clone(), v1.clone(), v21)?;
            match m209 {
                None => {
                    f9(v0, v1, v22)
                }
                Some(v23) => {
                    Ok(Some(v23))
                }
            }
        }
        Adt3::C9(r257, r258) => {
            let v24 = (*r257).clone();
            let v25 = (*r258).clone();
            let m210 = f28(f53(v24.clone())?, f9(v0.clone(), v1.clone(), v24)?);
            match m210 {
                None => {
                    if f54(v25.clone()) {
                        Ok(Some(f12(f13(Str::lit("runtime support: match-arms: ")), f13(Str::lit("a match with no arm")))))
                    } else {
                        f55(v0, v1, v25)
                    }
                }
                Some(v26) => {
                    Ok(Some(v26))
                }
            }
        }
        Adt3::C10(_, _, _) => {
            Ok(Some(f12(f13(Str::lit("runtime support: let-position: ")), f13(Str::lit("a `let` that is not a statement of a block")))))
        }
        Adt3::C11(r259, r260) => {
            let v30 = (*r259).clone();
            let v31 = (*r260).clone();
            let m211 = f56(v1.clone(), v30.clone());
            match m211 {
                None => {
                    let m212 = f9(v0.clone(), v1.clone(), v30)?;
                    match m212 {
                        None => {
                            f9(v0, v1, v31)
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
        Adt3::C12(r261, r262, r263) => {
            let v34 = (*r261).clone();
            let v35 = (*r262).clone();
            let v36 = (*r263).clone();
            let m213 = f28(f53(v34.clone())?, f9(v0.clone(), v1.clone(), v34)?);
            match m213 {
                None => {
                    let m214 = f57(v0.clone(), v1.clone(), v35)?;
                    match m214 {
                        None => {
                            f57(v0, v1, v36)
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
        Adt3::C13(r264, r265) => {
            let v39 = (*r264).clone();
            let v40 = (*r265).clone();
            let m215 = f28(f53(v39.clone())?, f9(v0.clone(), v1.clone(), v39)?);
            match m215 {
                None => {
                    f57(v0, f58(v1, List::<Adt16>::nil(), true), v40)
                }
                Some(v41) => {
                    Ok(Some(v41))
                }
            }
        }
        Adt3::C14(v42, r266, r267) => {
            let v43 = (*r266).clone();
            let v44 = (*r267).clone();
            let m216 = f28(f53(v43.clone())?, f9(v0.clone(), v1.clone(), v43)?);
            match m216 {
                None => {
                    let m217 = f59(v0.clone(), v42.clone())?;
                    match m217 {
                        None => {
                            let m218 = f60(List::<Adt16>::nil(), v42)?;
                            match m218 {
                                Ok(v48) => {
                                    f57(v0, f58(v1, v48, true), v44)
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
        Adt3::C15(r268) => {
            let v49 = (*r268).clone();
            f57(v0, f58(v1, List::<Adt16>::nil(), true), v49)
        }
        Adt3::C16(v50, r269, r270) => {
            let v51 = (*r269).clone();
            let v52 = (*r270).clone();
            let m219 = f28(f53(v51.clone())?, f9(v0.clone(), v1.clone(), v51)?);
            match m219 {
                None => {
                    let m220 = f59(v0.clone(), v50.clone())?;
                    match m220 {
                        None => {
                            let m221 = f60(List::<Adt16>::nil(), v50)?;
                            match m221 {
                                Ok(v56) => {
                                    f57(v0, f58(v1, v56, true), v52)
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
                Adt15::C0(_, _, _, h222) => {
                    h222
                }
            } {
                Ok(None::<Bytes>)
            } else {
                Ok(Some(f12(f13(Str::lit("runtime support: break-position: ")), f13(Str::lit("a `break` outside a loop")))))
            }
        }
        Adt3::C18(r271) => {
            let v57 = (*r271).clone();
            if f61(v57.clone()) {
                Ok(None::<Bytes>)
            } else {
                f9(v0, v1, v57)
            }
        }
        Adt3::C19(r272) => {
            let v58 = (*r272).clone();
            let a223 = f50(v58.clone())?;
            let a224 = 2u64;
            if nat_lt(a223, a224) {
                Ok(Some(f12(f13(Str::lit("runtime support: tuple-arity: ")), f12(f13(Str::lit("a tuple of ")), f40(f50(v58)?)?))))
            } else {
                f51(v0, v1, v58)
            }
        }
        Adt3::C20(v59, v60, r273) => {
            let v61 = (*r273).clone();
            let m225 = f48(v0.clone(), v59.clone())?;
            match m225 {
                None => {
                    let m226 = f62(v0.clone(), v59.clone());
                    match m226 {
                        None => {
                            Ok(Some(f12(f13(Str::lit("runtime support: struct-fields: ")), f12(f14(f27(v59, Str::lit("::"))?), f13(Str::lit(" is no record struct or record variant"))))))
                        }
                        Some(v63) => {
                            if f63(v60.clone(), v63.clone(), f50(v61.clone())?)? {
                                f51(v0, v1, v61)
                            } else {
                                Ok(Some(f12(f13(Str::lit("runtime support: struct-fields: ")), f12(f14(f27(v59, Str::lit("::"))?), f12(f13(Str::lit(" is given ")), f12(f27(v60, Str::lit(", "))?, f12(f13(Str::lit(", not ")), f27(v63, Str::lit(", "))?)))))))
                            }
                        }
                    }
                }
                Some(v62) => {
                    Ok(Some(v62))
                }
            }
        }
        Adt3::C21(_, r274, r275) => {
            let v65 = (*r274).clone();
            let v66 = (*r275).clone();
            let m227 = f9(v0.clone(), v1.clone(), v65)?;
            match m227 {
                None => {
                    f9(v0, v1, v66)
                }
                Some(v67) => {
                    Ok(Some(v67))
                }
            }
        }
        Adt3::C22(r276) => {
            let v68 = (*r276).clone();
            f9(v0, v1, v68)
        }
        Adt3::C23(r277, v70) => {
            let v69 = (*r277).clone();
            let m232 = f28(f8(v0.clone(), v70.clone(), {
                let a229 = match v1.clone() {
                    Adt15::C0(h228, _, _, _) => {
                        h228
                    }
                };
                length_list(a229)
            }, {
                let a231 = f64(match v1.clone() {
                    Adt15::C0(_, _, h230, _) => {
                        h230
                    }
                });
                bool_not(a231)
            })?, f65(Str::lit("a cast type"), v70));
            match m232 {
                None => {
                    f9(v0, v1, v69)
                }
                Some(v71) => {
                    Ok(Some(v71))
                }
            }
        }
        Adt3::C24(r278, r279) => {
            let v72 = (*r278).clone();
            let v73 = (*r279).clone();
            let m233 = f9(v0.clone(), v1.clone(), v72)?;
            match m233 {
                None => {
                    f9(v0, v1, v73)
                }
                Some(v74) => {
                    Ok(Some(v74))
                }
            }
        }
        Adt3::C25(r280, r281, r282) => {
            let v75 = (*r280).clone();
            let v76 = (*r281).clone();
            let v77 = (*r282).clone();
            let m234 = f9(v0.clone(), v1.clone(), v75)?;
            match m234 {
                None => {
                    let m235 = if f61(v76.clone()) {
                        None::<Bytes>
                    } else {
                        f9(v0.clone(), v1.clone(), v76)?
                    };
                    match m235 {
                        None => {
                            if f61(v77.clone()) {
                                Ok(None::<Bytes>)
                            } else {
                                f9(v0, v1, v77)
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
        Adt3::C26(r283) => {
            let v80 = (*r283).clone();
            f9(v0, v1, v80)
        }
        Adt3::C27(r284) => {
            let v81 = (*r284).clone();
            f9(v0, v1, v81)
        }
        Adt3::C28(r285, v83, v84, r286) => {
            let v82 = (*r285).clone();
            let v85 = (*r286).clone();
            let m236 = f9(v0.clone(), v1.clone(), v82)?;
            match m236 {
                None => {
                    let m237 = f66(v83.clone());
                    match m237 {
                        None => {
                            Ok(Some(f12(f13(Str::lit("runtime support: method-name: ")), f12(f14(v83), f13(Str::lit(" takes no type arguments in the closed table"))))))
                        }
                        Some(v87) => {
                            let a238 = v87;
                            let a239 = f39(v84.clone())?;
                            if if nat_eq(a238, a239) {
                                let a240 = f67(v83.clone());
                                let a241 = f50(v85.clone())?;
                                nat_eq(a240, a241)
                            } else {
                                false
                            } {
                                if if match v0.clone() {
                                    Adt14::C0(h242, _, _, _, _, _, _, _, _, _) => {
                                        h242
                                    }
                                } {
                                    f25(v83.clone(), f13(Str::lit("collect")))
                                } else {
                                    false
                                } {
                                    Ok(Some(f12(f13(Str::lit("runtime support: hidden-allocation: ")), f12(f13(Str::lit("rust-core renders ")), f12(f14(v83), f13(Str::lit(", which requires heap allocation")))))))
                                } else {
                                    let m247 = f28(f8(v0.clone(), v84.clone(), {
                                        let a244 = match v1.clone() {
                                            Adt15::C0(h243, _, _, _) => {
                                                h243
                                            }
                                        };
                                        length_list(a244)
                                    }, {
                                        let a246 = f64(match v1.clone() {
                                            Adt15::C0(_, _, h245, _) => {
                                                h245
                                            }
                                        });
                                        bool_not(a246)
                                    })?, f65(Str::lit("a type argument"), v84));
                                    match m247 {
                                        None => {
                                            f51(v0, v1, v85)
                                        }
                                        Some(v88) => {
                                            Ok(Some(v88))
                                        }
                                    }
                                }
                            } else {
                                Ok(Some(f12(f13(Str::lit("runtime support: method-arity: ")), f12(f14(v83.clone()), f12(f13(Str::lit(" takes ")), f12(f40(v87)?, f12(f13(Str::lit(" type arguments and ")), f12(f40(f67(v83))?, f13(Str::lit(" arguments"))))))))))
                            }
                        }
                    }
                }
                Some(v86) => {
                    Ok(Some(v86))
                }
            }
        }
        Adt3::C29(r287, r288) => {
            let v89 = (*r287).clone();
            let v90 = (*r288).clone();
            let m248 = f68(v0.clone(), v1, v89)?;
            match m248 {
                Ok(v92) => {
                    f9(v0, v92, v90)
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

pub fn f10(_: ()) -> Adt15 {
    Adt15::C0(List::<Adt7>::nil(), List::<Adt16>::nil(), Adt2::C17, false)
}

pub fn f11(v0: List<Adt5>) -> R<Option<Bytes>> {
    let m290 = f69(Fn5::F70, Adt17::C0(List::<Bytes>::nil(), None::<Bytes>), v0)?;
    match m290 {
        Adt17::C0(_, h289) => {
            Ok(h289)
        }
    }
}

pub fn f12(v0: Bytes, v1: Bytes) -> Bytes {
    let a291 = v0;
    let a292 = v1;
    append_bytes(a291, a292)
}

pub fn f13(v0: Str) -> Bytes {
    let a293 = v0;
    utf8_encode(a293)
}

pub fn f14(v0: Bytes) -> Bytes {
    f12(f13(Str::lit("`")), f12(v0, f13(Str::lit("`"))))
}

pub fn f15(v0: Fn2, p297: Option<Bytes>, p298: List<Adt5>) -> R<Option<Bytes>> {
    let m296 = (p297, p298);
    {
        let mut m296 = m296;
        loop {
            let step = {
                let (v1, v2) = m296;
                let m294 = v2;
                match m294.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Adt5>), R<Option<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Adt5>), R<Option<Bytes>>>(({
                            let c295: Fn2 = v0.clone();
                            c295.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m296 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f16(v0: Adt14, v1: Option<Bytes>, v2: Adt5) -> R<Option<Bytes>> {
    let m299 = v1;
    match m299 {
        None => {
            f71(v0, v2, 0u64, true)
        }
        Some(v3) => {
            Ok(Some(v3))
        }
    }
}

pub fn f17(v0: Adt14, v1: List<Adt7>) -> R<Option<Bytes>> {
    f72(Fn6::F73(v0), None::<Bytes>, v1)
}

pub fn f18(v0: Adt14, v1: Bytes, v2: List<Adt7>, v3: Adt2, v4: List<Adt8>) -> R<Option<Bytes>> {
    let m300 = f17(v0.clone(), v2.clone())?;
    match m300 {
        None => {
            let m301 = f74(v0.clone(), v1.clone());
            match m301 {
                None => {
                    Ok(Some(f12(f13(Str::lit("runtime support: impl-trait: ")), f12(f13(Str::lit("`impl ")), f12(v1, f13(Str::lit("` names no declared trait")))))))
                }
                Some(v6) => {
                    let m302 = f15(Fn2::F75(v4.clone(), v1.clone()), None::<Bytes>, v6.clone())?;
                    match m302 {
                        None => {
                            f19(Fn3::F76(v2, v0, v6, v1, v3), None::<Bytes>, v4)
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

pub fn f19(v0: Fn3, p306: Option<Bytes>, p307: List<Adt8>) -> R<Option<Bytes>> {
    let m305 = (p306, p307);
    {
        let mut m305 = m305;
        loop {
            let step = {
                let (v1, v2) = m305;
                let m303 = v2;
                match m303.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Adt8>), R<Option<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Adt8>), R<Option<Bytes>>>(({
                            let c304: Fn3 = v0.clone();
                            c304.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m305 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f20(v0: List<Adt7>, v1: Adt14, v2: Adt2, v3: Option<Bytes>, v4: Adt8) -> R<Option<Bytes>> {
    let m308 = v3;
    match m308 {
        None => {
            f21(v1, v0, v2, v4)
        }
        Some(v5) => {
            Ok(Some(v5))
        }
    }
}

pub fn f21(v0: Adt14, v1: List<Adt7>, v2: Adt2, v3: Adt8) -> R<Option<Bytes>> {
    let m310 = f17(v0.clone(), match v3.clone() {
        Adt8::C0(_, h309, _, _) => {
            h309
        }
    })?;
    match m310 {
        None => {
            let v5: List<Adt7> = {
                let a312 = v1;
                let a313 = match v3.clone() {
                    Adt8::C0(_, h311, _, _) => {
                        h311
                    }
                };
                append_list(a312, a313)
            };
            let v6: u64 = {
                let a314 = v5.clone();
                length_list(a314)
            };
            let v7: Adt5 = match v3.clone() {
                Adt8::C0(h315, _, _, _) => {
                    h315
                }
            };
            let m317 = f71(v0.clone(), v7.clone(), v6, {
                let a316 = f64(v2.clone());
                bool_not(a316)
            })?;
            match m317 {
                None => {
                    let m322 = if match v7.clone() {
                        Adt5::C0(_, h318, _, _, _) => {
                            h318
                        }
                    } {
                        if f64(v2.clone()) {
                            Err::<List<Adt16>, Bytes>(f12(f13(Str::lit("runtime support: function-receiver: ")), f12(f14(match v7.clone() {
                                Adt5::C0(h319, _, _, _, _) => {
                                    h319
                                }
                            }), f13(Str::lit(" takes `&self` outside an impl")))))
                        } else {
                            Ok::<List<Adt16>, Bytes>(List::cons(Adt16::C0(f13(Str::lit("self")), match v7.clone() {
                                Adt5::C0(_, _, h320, _, _) => {
                                    h320
                                }
                            }, if match v7.clone() {
                                Adt5::C0(_, _, h321, _, _) => {
                                    h321
                                }
                            } {
                                Adt2::C8(std::rc::Rc::new(v2.clone()))
                            } else {
                                Adt2::C7(std::rc::Rc::new(v2.clone()))
                            }), List::<Adt16>::nil()))
                        }
                    } else {
                        Ok::<List<Adt16>, Bytes>(List::<Adt16>::nil())
                    };
                    match m322 {
                        Ok(v10) => {
                            let m325 = f77(Ok::<List<Adt16>, Bytes>(v10), match v7.clone() {
                                Adt5::C0(_, _, _, h323, _) => {
                                    h323
                                }
                            }, match v7 {
                                Adt5::C0(h324, _, _, _, _) => {
                                    h324
                                }
                            });
                            match m325 {
                                Ok(v12) => {
                                    f57(v0, Adt15::C0(v5, v12, v2, false), match v3 {
                                        Adt8::C0(_, _, _, h326) => {
                                            h326
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

pub fn f22(v0: Fn4, p330: Option<Bytes>, p331: List<Adt9>) -> R<Option<Bytes>> {
    let m329 = (p330, p331);
    {
        let mut m329 = m329;
        loop {
            let step = {
                let (v1, v2) = m329;
                let m327 = v2;
                match m327.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Adt9>), R<Option<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Adt9>), R<Option<Bytes>>>(({
                            let c328: Fn4 = v0.clone();
                            c328.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m329 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f23(v0: List<Bytes>, v1: Adt14, v2: Option<Bytes>, v3: Adt9) -> R<Option<Bytes>> {
    let m332 = v2;
    match m332 {
        None => {
            let m333 = v3;
            match m333 {
                Adt9::C0(_) => {
                    Ok(None::<Bytes>)
                }
                Adt9::C1(_, v7) => {
                    f8(v1, v7, {
                        let a334 = v0;
                        length_list(a334)
                    }, false)
                }
                Adt9::C2(_, _, v10) => {
                    f8(v1, v10, {
                        let a335 = v0;
                        length_list(a335)
                    }, false)
                }
            }
        }
        Some(v4) => {
            Ok(Some(v4))
        }
    }
}

pub fn f24(v0: Result<Adt14, Bytes>, v1: Option<Bytes>) -> Result<Adt14, Bytes> {
    let m336 = v0.clone();
    match m336 {
        Ok(_) => {
            let m337 = v1;
            match m337 {
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

pub fn f25(v0: Bytes, v1: Bytes) -> bool {
    let a338 = v0;
    let a339 = v1;
    equal(a338, a339)
}

pub fn f26(v0: List<Bytes>) -> Bytes {
    f78(Fn7::F79, f13(Str::lit("")), v0)
}

pub fn f27(v0: List<Bytes>, v1: Str) -> R<Bytes> {
    f80(v0, f13(v1))
}

pub fn f28(v0: Option<Bytes>, v1: Option<Bytes>) -> Option<Bytes> {
    let m340 = v0;
    match m340 {
        None => {
            v1
        }
        Some(v2) => {
            Some(v2)
        }
    }
}

pub fn f29(v0: Str, v1: List<Bytes>) -> R<Option<Bytes>> {
    f81(Fn8::F82(v0), None::<Bytes>, v1)
}

pub fn f30(v0: Str, v1: List<Bytes>) -> R<Option<Bytes>> {
    f81(Fn8::F83(v0), None::<Bytes>, v1)
}

pub fn f31(v0: Option<Bytes>, v1: Result<Adt14, Bytes>) -> Result<Adt14, Bytes> {
    let m341 = v0;
    match m341 {
        None => {
            v1
        }
        Some(v2) => {
            Err::<Adt14, Bytes>(v2)
        }
    }
}

pub fn f32(v0: Str, v1: Bytes) -> R<Option<Bytes>> {
    if if f84(v1.clone())? {
        let a342 = f46(f85(), v1.clone())?;
        bool_not(a342)
    } else {
        false
    } {
        Ok(None::<Bytes>)
    } else {
        Ok(Some(f12(f13(Str::lit("runtime support: identifier: ")), f12(f13(v0), f12(f13(Str::lit(" ")), f12(f14(v1), f13(Str::lit(" is not an identifier"))))))))
    }
}

pub fn f33(v0: Adt14, v1: Bytes) -> R<Result<Adt14, Bytes>> {
    if f46(match v0.clone() {
        Adt14::C0(_, h343, _, _, _, _, _, _, _, _) => {
            h343
        }
    }, v1.clone())? {
        Ok(Err::<Adt14, Bytes>(f12(f13(Str::lit("runtime support: duplicate-name: ")), f12(f14(v1), f13(Str::lit(" is declared twice"))))))
    } else {
        Ok(Ok::<Adt14, Bytes>(Adt14::C0(match v0.clone() {
            Adt14::C0(h344, _, _, _, _, _, _, _, _, _) => {
                h344
            }
        }, {
            let a346 = match v0.clone() {
                Adt14::C0(_, h345, _, _, _, _, _, _, _, _) => {
                    h345
                }
            };
            let a347 = List::cons(v1, List::<Bytes>::nil());
            append_list(a346, a347)
        }, match v0.clone() {
            Adt14::C0(_, _, h348, _, _, _, _, _, _, _) => {
                h348
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, h349, _, _, _, _, _, _) => {
                h349
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, h350, _, _, _, _, _) => {
                h350
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, h351, _, _, _, _) => {
                h351
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, h352, _, _, _) => {
                h352
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h353, _, _) => {
                h353
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, h354, _) => {
                h354
            }
        }, match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, _, h355) => {
                h355
            }
        })))
    }
}

pub fn f34(v0: Str, v1: List<Bytes>) -> R<Option<Bytes>> {
    f81(Fn8::F86(v0), None::<Bytes>, v1)
}

pub fn f35(v0: Adt14, v1: List<Adt5>) -> Adt14 {
    f87(Fn9::F88, v0, v1)
}

pub fn f36(v0: Adt14, v1: Adt2, v2: List<Adt8>) -> Adt14 {
    let m356 = f89(v1);
    match m356 {
        None => {
            v0
        }
        Some(v3) => {
            f90(v0, v3, v2)
        }
    }
}

pub fn f37(v0: Adt14, v1: List<Adt8>) -> Adt14 {
    f91(Fn10::F92, v0, v1)
}

pub fn f38(v0: Adt2, v1: u64) -> R<List<Bytes>> {
    let m357 = v0;
    match m357 {
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
        Adt2::C19(_, r360) => {
            let v11 = (*r360).clone();
            Ok(List::cons(f40(v1)?, f38(v11, {
                let a358 = v1;
                let a359 = 1u64;
                nat_add(a358, a359)?
            })?))
        }
    }
}

pub fn f39(v0: Adt2) -> R<u64> {
    let m361 = v0;
    match m361 {
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
        Adt2::C19(_, r364) => {
            let v10 = (*r364).clone();
            let a362 = f39(v10)?;
            let a363 = 1u64;
            nat_add(a362, a363)
        }
    }
}

pub fn f40(v0: u64) -> R<Bytes> {
    f93(v0)
}

pub fn f41(v0: Bytes, v1: List<Adt9>) -> R<Option<Bytes>> {
    let m366 = f94(Fn11::F95(v0), Adt17::C0(List::<Bytes>::nil(), None::<Bytes>), v1)?;
    match m366 {
        Adt17::C0(_, h365) => {
            Ok(h365)
        }
    }
}

pub fn f42(v0: List<Adt9>) -> R<Option<Bytes>> {
    let m368 = f94(Fn11::F96, Adt17::C0(List::<Bytes>::nil(), None::<Bytes>), v0)?;
    match m368 {
        Adt17::C0(_, h367) => {
            Ok(h367)
        }
    }
}

pub fn f43(v0: Adt14, v1: Bytes) -> Option<u64> {
    let m369 = f97(v0.clone(), v1.clone());
    match m369 {
        None => {
            let m371 = f98(v0.clone(), v1.clone());
            match m371 {
                None => {
                    let m373 = f99(v0, v1.clone());
                    match m373 {
                        None => {
                            f100(v1)
                        }
                        Some(v4) => {
                            Some(match v4 {
                                Adt13::C0(_, h374) => {
                                    h374
                                }
                            })
                        }
                    }
                }
                Some(v3) => {
                    Some(match v3 {
                        Adt12::C0(_, _, h372) => {
                            h372
                        }
                    })
                }
            }
        }
        Some(v2) => {
            Some(match v2 {
                Adt11::C0(_, _, h370) => {
                    h370
                }
            })
        }
    }
}

pub fn f44(v0: Adt14, v1: Bytes) -> bool {
    let m375 = f97(v0.clone(), v1.clone());
    match m375 {
        None => {
            let m376 = f98(v0.clone(), v1.clone());
            match m376 {
                None => {
                    let m377 = f99(v0, v1.clone());
                    match m377 {
                        None => {
                            f101(v1)
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

pub fn f45(v0: List<Adt16>, v1: Bytes) -> Option<Adt16> {
    f102(Fn12::F103(v1), None::<Adt16>, v0)
}

pub fn f46(v0: List<Bytes>, v1: Bytes) -> R<bool> {
    f104(Fn13::F105(v1), false, v0)
}

pub fn f47(v0: u64) -> R<Option<Bytes>> {
    let a378 = v0;
    let a379 = 1u64;
    if nat_eq(a378, a379) {
        Ok(None::<Bytes>)
    } else {
        Ok(Some(f12(f13(Str::lit("runtime support: char-literal: ")), f12(f13(Str::lit("a character literal of ")), f12(f40(v0)?, f13(Str::lit(" scalar values")))))))
    }
}

pub fn f48(v0: Adt14, v1: List<Bytes>) -> R<Option<Bytes>> {
    let m380 = f29(Str::lit("a path segment"), v1.clone())?;
    match m380 {
        None => {
            f106(v0, v1)
        }
        Some(v2) => {
            Ok(Some(v2))
        }
    }
}

pub fn f49(v0: Adt14, v1: List<Bytes>, v2: u64) -> R<Option<Bytes>> {
    if f107(v0.clone(), v1.clone()) {
        Ok(Some(f12(f13(Str::lit("runtime support: hidden-allocation: ")), f12(f13(Str::lit("rust-core renders ")), f12(f14(f27(v1, Str::lit("::"))?), f13(Str::lit(", which requires heap allocation")))))))
    } else {
        let m381 = f108(v0, v1.clone())?;
        match m381 {
            None => {
                Ok(Some(f12(f13(Str::lit("runtime support: call-name: ")), f12(f14(f27(v1, Str::lit("::"))?), f13(Str::lit(" is neither declared nor in the closed table"))))))
            }
            Some(v3) => {
                let a382 = v3;
                let a383 = v2;
                if nat_eq(a382, a383) {
                    Ok(None::<Bytes>)
                } else {
                    Ok(Some(f12(f13(Str::lit("runtime support: call-arity: ")), f12(f14(f27(v1, Str::lit("::"))?), f12(f13(Str::lit(" takes ")), f12(f40(v3)?, f12(f13(Str::lit(" arguments, given ")), f40(v2)?)))))))
                }
            }
        }
    }
}

pub fn f50(v0: Adt3) -> R<u64> {
    let m384 = v0;
    match m384 {
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
        Adt3::C32(_, r387) => {
            let v60 = (*r387).clone();
            let a385 = f50(v60)?;
            let a386 = 1u64;
            nat_add(a385, a386)
        }
        Adt3::C33(_, _, _, _) => {
            Ok(0u64)
        }
        Adt3::C34(_, _, _, _) => {
            Ok(0u64)
        }
    }
}

pub fn f51(v0: Adt14, v1: Adt15, p393: Adt3) -> R<Option<Bytes>> {
    let m392 = p393;
    {
        let mut m392 = m392;
        loop {
            let step = {
                let v2 = m392;
                let m388 = v2;
                match m388 {
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
                    Adt3::C32(r390, r391) => {
                        let v61 = (*r390).clone();
                        let v62 = (*r391).clone();
                        let m389 = f9(v0.clone(), v1.clone(), v61)?;
                        match m389 {
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
                Ok(next) => { m392 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f52(v0: Adt14, v1: Adt15, v2: Adt3, v3: Bytes, v4: u64) -> R<Option<Bytes>> {
    let m394 = f109(v3.clone());
    match m394 {
        None => {
            let v6: List<u64> = f111(v0.clone(), v3.clone());
            let m398 = v6.clone();
            match m398.uncons() {
                None => {
                    Ok(Some(f12(f13(Str::lit("runtime support: method-name: ")), f12(f14(v3), f13(Str::lit(" is a method of no declared trait or impl and not in the closed table"))))))
                }
                Some((_, _)) => {
                    if f112(v6.clone(), v4) {
                        f113(v0, v1, v2, v3)
                    } else {
                        Ok(Some(f12(f13(Str::lit("runtime support: method-arity: ")), f12(f14(v3), f12(f13(Str::lit(" takes ")), f12(f114(v6)?, f12(f13(Str::lit(" arguments, given ")), f40(v4)?)))))))
                    }
                }
            }
        }
        Some(v5) => {
            if if f110(v3.clone()) {
                match v0 {
                    Adt14::C0(h395, _, _, _, _, _, _, _, _, _) => {
                        h395
                    }
                }
            } else {
                false
            } {
                Ok(Some(f12(f13(Str::lit("runtime support: hidden-allocation: ")), f12(f13(Str::lit("rust-core renders ")), f12(f14(v3), f13(Str::lit(", which requires heap allocation")))))))
            } else {
                let a396 = v5;
                let a397 = v4;
                if nat_eq(a396, a397) {
                    Ok(None::<Bytes>)
                } else {
                    Ok(Some(f12(f13(Str::lit("runtime support: method-arity: ")), f12(f14(v3), f12(f13(Str::lit(" takes ")), f12(f40(v5)?, f12(f13(Str::lit(" arguments, given ")), f40(v4)?)))))))
                }
            }
        }
    }
}

pub fn f53(v0: Adt3) -> R<Option<Bytes>> {
    if f115(v0)? {
        Ok(Some(f12(f13(Str::lit("runtime support: struct-head: ")), f13(Str::lit("a struct literal outside parentheses in the head of a condition, a loop, or a match")))))
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f54(v0: Adt3) -> bool {
    let m399 = v0;
    match m399 {
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

pub fn f55(v0: Adt14, v1: Adt15, p409: Adt3) -> R<Option<Bytes>> {
    let m408 = p409;
    {
        let mut m408 = m408;
        loop {
            let step = {
                let v2 = m408;
                let m400 = v2;
                match m400 {
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
                    Adt3::C34(v67, r405, r406, r407) => {
                        let v68 = (*r405).clone();
                        let v69 = (*r406).clone();
                        let v70 = (*r407).clone();
                        let m401 = f59(v0.clone(), v67.clone())?;
                        match m401 {
                            None => {
                                let m402 = f60(List::<Adt16>::nil(), v67)?;
                                match m402 {
                                    Ok(v73) => {
                                        let v74: Adt15 = f58(v1.clone(), v73, false);
                                        let m403 = if f61(v68.clone()) {
                                            None::<Bytes>
                                        } else {
                                            f9(v0.clone(), v74.clone(), v68)?
                                        };
                                        match m403 {
                                            None => {
                                                let m404 = f9(v0.clone(), v74, v69)?;
                                                match m404 {
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
                Ok(next) => { m408 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f56(v0: Adt15, p420: Adt3) -> Option<Bytes> {
    let m419 = p420;
    {
        let mut m419 = m419;
        loop {
            let step = {
                let v1 = m419;
                let m410 = v1;
                match m410 {
                    Adt3::C0(v2) => {
                        let m412 = f45(match v0.clone() {
                            Adt15::C0(_, h411, _, _) => {
                                h411
                            }
                        }, v2.clone());
                        match m412 {
                            None => {
                                Err::<Adt3, Option<Bytes>>(Some(f12(f13(Str::lit("runtime support: variable-scope: ")), f12(f14(v2), f13(Str::lit(" is not in scope"))))))
                            }
                            Some(v3) => {
                                if match v3.clone() {
                                    Adt16::C0(_, h413, _) => {
                                        h413
                                    }
                                } {
                                    Err::<Adt3, Option<Bytes>>(None::<Bytes>)
                                } else {
                                    if f116(match v3 {
                                        Adt16::C0(_, _, h414) => {
                                            h414
                                        }
                                    }) {
                                        Err::<Adt3, Option<Bytes>>(None::<Bytes>)
                                    } else {
                                        Err::<Adt3, Option<Bytes>>(Some(f12(f13(Str::lit("runtime support: assign-immutable: ")), f12(f14(v2), f13(Str::lit(" is assigned and is not `mut` or `&mut`"))))))
                                    }
                                }
                            }
                        }
                    }
                    Adt3::C1(_) => {
                        Err::<Adt3, Option<Bytes>>(Some(f12(f13(Str::lit("runtime support: assign-place: ")), f13(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C2(_) => {
                        Err::<Adt3, Option<Bytes>>(Some(f12(f13(Str::lit("runtime support: assign-place: ")), f13(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C3(_, _) => {
                        Err::<Adt3, Option<Bytes>>(Some(f12(f13(Str::lit("runtime support: assign-place: ")), f13(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C4(_, _, _) => {
                        Err::<Adt3, Option<Bytes>>(Some(f12(f13(Str::lit("runtime support: assign-place: ")), f13(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C5(r415, _) => {
                        let v11 = (*r415).clone();
                        Ok::<Adt3, Option<Bytes>>(v11)
                    }
                    Adt3::C6(r416, _) => {
                        let v13 = (*r416).clone();
                        Ok::<Adt3, Option<Bytes>>(v13)
                    }
                    Adt3::C7(_) => {
                        Err::<Adt3, Option<Bytes>>(Some(f12(f13(Str::lit("runtime support: assign-place: ")), f13(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C8(_, _) => {
                        Err::<Adt3, Option<Bytes>>(Some(f12(f13(Str::lit("runtime support: assign-place: ")), f13(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C9(_, _) => {
                        Err::<Adt3, Option<Bytes>>(Some(f12(f13(Str::lit("runtime support: assign-place: ")), f13(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C10(_, _, _) => {
                        Err::<Adt3, Option<Bytes>>(Some(f12(f13(Str::lit("runtime support: assign-place: ")), f13(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C11(_, _) => {
                        Err::<Adt3, Option<Bytes>>(Some(f12(f13(Str::lit("runtime support: assign-place: ")), f13(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C12(_, _, _) => {
                        Err::<Adt3, Option<Bytes>>(Some(f12(f13(Str::lit("runtime support: assign-place: ")), f13(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C13(_, _) => {
                        Err::<Adt3, Option<Bytes>>(Some(f12(f13(Str::lit("runtime support: assign-place: ")), f13(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C14(_, _, _) => {
                        Err::<Adt3, Option<Bytes>>(Some(f12(f13(Str::lit("runtime support: assign-place: ")), f13(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C15(_) => {
                        Err::<Adt3, Option<Bytes>>(Some(f12(f13(Str::lit("runtime support: assign-place: ")), f13(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C16(_, _, _) => {
                        Err::<Adt3, Option<Bytes>>(Some(f12(f13(Str::lit("runtime support: assign-place: ")), f13(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C17 => {
                        Err::<Adt3, Option<Bytes>>(Some(f12(f13(Str::lit("runtime support: assign-place: ")), f13(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C18(_) => {
                        Err::<Adt3, Option<Bytes>>(Some(f12(f13(Str::lit("runtime support: assign-place: ")), f13(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C19(_) => {
                        Err::<Adt3, Option<Bytes>>(Some(f12(f13(Str::lit("runtime support: assign-place: ")), f13(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C20(_, _, _) => {
                        Err::<Adt3, Option<Bytes>>(Some(f12(f13(Str::lit("runtime support: assign-place: ")), f13(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C21(_, _, _) => {
                        Err::<Adt3, Option<Bytes>>(Some(f12(f13(Str::lit("runtime support: assign-place: ")), f13(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C22(_) => {
                        Err::<Adt3, Option<Bytes>>(Some(f12(f13(Str::lit("runtime support: assign-place: ")), f13(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C23(_, _) => {
                        Err::<Adt3, Option<Bytes>>(Some(f12(f13(Str::lit("runtime support: assign-place: ")), f13(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C24(r417, _) => {
                        let v48 = (*r417).clone();
                        Ok::<Adt3, Option<Bytes>>(v48)
                    }
                    Adt3::C25(_, _, _) => {
                        Err::<Adt3, Option<Bytes>>(Some(f12(f13(Str::lit("runtime support: assign-place: ")), f13(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C26(r418) => {
                        let v53 = (*r418).clone();
                        Ok::<Adt3, Option<Bytes>>(v53)
                    }
                    Adt3::C27(_) => {
                        Err::<Adt3, Option<Bytes>>(Some(f12(f13(Str::lit("runtime support: assign-place: ")), f13(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C28(_, _, _, _) => {
                        Err::<Adt3, Option<Bytes>>(Some(f12(f13(Str::lit("runtime support: assign-place: ")), f13(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C29(_, _) => {
                        Err::<Adt3, Option<Bytes>>(Some(f12(f13(Str::lit("runtime support: assign-place: ")), f13(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C30 => {
                        Err::<Adt3, Option<Bytes>>(Some(f12(f13(Str::lit("runtime support: assign-place: ")), f13(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C31 => {
                        Err::<Adt3, Option<Bytes>>(Some(f12(f13(Str::lit("runtime support: assign-place: ")), f13(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C32(_, _) => {
                        Err::<Adt3, Option<Bytes>>(Some(f12(f13(Str::lit("runtime support: assign-place: ")), f13(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C33(_, _, _, _) => {
                        Err::<Adt3, Option<Bytes>>(Some(f12(f13(Str::lit("runtime support: assign-place: ")), f13(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C34(_, _, _, _) => {
                        Err::<Adt3, Option<Bytes>>(Some(f12(f13(Str::lit("runtime support: assign-place: ")), f13(Str::lit("an assignment to a value that is no place")))))
                    }
                }
            };
            match step {
                Ok(next) => { m419 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f57(v0: Adt14, v1: Adt15, v2: Adt3) -> R<Option<Bytes>> {
    let m421 = v2;
    match m421 {
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
        Adt3::C29(r423, r424) => {
            let v59 = (*r423).clone();
            let v60 = (*r424).clone();
            let m422 = f68(v0.clone(), v1, v59)?;
            match m422 {
                Ok(v62) => {
                    f9(v0, v62, v60)
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

pub fn f58(v0: Adt15, v1: List<Adt16>, v2: bool) -> Adt15 {
    Adt15::C0(match v0.clone() {
        Adt15::C0(h425, _, _, _) => {
            h425
        }
    }, {
        let a427 = v1;
        let a428 = match v0.clone() {
            Adt15::C0(_, h426, _, _) => {
                h426
            }
        };
        append_list(a427, a428)
    }, match v0.clone() {
        Adt15::C0(_, _, h429, _) => {
            h429
        }
    }, if match v0 {
        Adt15::C0(_, _, _, h430) => {
            h430
        }
    } {
        true
    } else {
        v2
    })
}

pub fn f59(v0: Adt14, v1: Adt4) -> R<Option<Bytes>> {
    let m431 = v1;
    match m431 {
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
            f48(v0, v4)
        }
        Adt4::C4(r441) => {
            let v5 = (*r441).clone();
            f59(v0, v5)
        }
        Adt4::C5(v6, r442) => {
            let v7 = (*r442).clone();
            if f117(v6.clone()) {
                f59(v0, v7)
            } else {
                let m432 = f48(v0.clone(), v6.clone())?;
                match m432 {
                    None => {
                        let m433 = f118(v0.clone(), v6, v7.clone())?;
                        match m433 {
                            None => {
                                f59(v0, v7)
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
        Adt4::C6(v10, v11, r443) => {
            let v12 = (*r443).clone();
            let m434 = f48(v0.clone(), v10.clone())?;
            match m434 {
                None => {
                    let m435 = f30(Str::lit("a pattern field name"), v11.clone())?;
                    match m435 {
                        None => {
                            let a437 = {
                                let a436 = v11.clone();
                                length_list(a436)
                            };
                            let a438 = f119(v12.clone())?;
                            if nat_eq(a437, a438) {
                                let m439 = f120(v0.clone(), v10, v11)?;
                                match m439 {
                                    None => {
                                        f59(v0, v12)
                                    }
                                    Some(v15) => {
                                        Ok(Some(v15))
                                    }
                                }
                            } else {
                                Ok(Some(f12(f13(Str::lit("runtime support: pattern-fields: ")), f13(Str::lit("a struct pattern with unequal names and patterns")))))
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
            f47(v16)
        }
        Adt4::C8 => {
            Ok(None::<Bytes>)
        }
        Adt4::C9(r444, r445) => {
            let v17 = (*r444).clone();
            let v18 = (*r445).clone();
            let m440 = f59(v0.clone(), v17)?;
            match m440 {
                None => {
                    f59(v0, v18)
                }
                Some(v19) => {
                    Ok(Some(v19))
                }
            }
        }
    }
}

pub fn f60(v0: List<Adt16>, v1: Adt4) -> R<Result<List<Adt16>, Bytes>> {
    let m446 = v1;
    match m446 {
        Adt4::C0 => {
            Ok(Ok::<List<Adt16>, Bytes>(v0))
        }
        Adt4::C1(v2) => {
            let m447 = f32(Str::lit("a pattern binding"), v2.clone())?;
            match m447 {
                None => {
                    let m448 = f45(v0.clone(), v2.clone());
                    match m448 {
                        None => {
                            Ok(Ok::<List<Adt16>, Bytes>(List::cons(Adt16::C0(v2, false, Adt2::C16), v0)))
                        }
                        Some(_) => {
                            Ok(Err::<List<Adt16>, Bytes>(f12(f13(Str::lit("runtime support: pattern-duplicate: ")), f12(f14(v2), f13(Str::lit(" is bound twice in one pattern"))))))
                        }
                    }
                }
                Some(v3) => {
                    Ok(Err::<List<Adt16>, Bytes>(v3))
                }
            }
        }
        Adt4::C2(v5) => {
            let m449 = f32(Str::lit("a pattern binding"), v5.clone())?;
            match m449 {
                None => {
                    let m450 = f45(v0.clone(), v5.clone());
                    match m450 {
                        None => {
                            Ok(Ok::<List<Adt16>, Bytes>(List::cons(Adt16::C0(v5, true, Adt2::C16), v0)))
                        }
                        Some(_) => {
                            Ok(Err::<List<Adt16>, Bytes>(f12(f13(Str::lit("runtime support: pattern-duplicate: ")), f12(f14(v5), f13(Str::lit(" is bound twice in one pattern"))))))
                        }
                    }
                }
                Some(v6) => {
                    Ok(Err::<List<Adt16>, Bytes>(v6))
                }
            }
        }
        Adt4::C3(_) => {
            Ok(Ok::<List<Adt16>, Bytes>(v0))
        }
        Adt4::C4(r452) => {
            let v9 = (*r452).clone();
            f60(v0, v9)
        }
        Adt4::C5(_, r453) => {
            let v11 = (*r453).clone();
            f60(v0, v11)
        }
        Adt4::C6(_, _, r454) => {
            let v14 = (*r454).clone();
            f60(v0, v14)
        }
        Adt4::C7(_) => {
            Ok(Ok::<List<Adt16>, Bytes>(v0))
        }
        Adt4::C8 => {
            Ok(Ok::<List<Adt16>, Bytes>(v0))
        }
        Adt4::C9(r455, r456) => {
            let v16 = (*r455).clone();
            let v17 = (*r456).clone();
            let m451 = f60(v0, v16)?;
            match m451 {
                Ok(v18) => {
                    f60(v18, v17)
                }
                Err(v19) => {
                    Ok(Err::<List<Adt16>, Bytes>(v19))
                }
            }
        }
    }
}

pub fn f61(v0: Adt3) -> bool {
    let m457 = v0;
    match m457 {
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

pub fn f62(v0: Adt14, v1: List<Bytes>) -> Option<List<Bytes>> {
    let m458 = v1;
    match m458.uncons() {
        None => {
            None::<List<Bytes>>
        }
        Some((v2, v3)) => {
            let m459 = v3;
            match m459.uncons() {
                None => {
                    let m460 = f97(v0, v2);
                    match m460 {
                        None => {
                            None::<List<Bytes>>
                        }
                        Some(v4) => {
                            Some(match v4 {
                                Adt11::C0(_, h461, _) => {
                                    h461
                                }
                            })
                        }
                    }
                }
                Some((v5, v6)) => {
                    let m462 = v6;
                    match m462.uncons() {
                        None => {
                            let m463 = f98(v0, v2);
                            match m463 {
                                None => {
                                    None::<List<Bytes>>
                                }
                                Some(v7) => {
                                    f121(match v7 {
                                        Adt12::C0(_, h464, _) => {
                                            h464
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

pub fn f63(v0: List<Bytes>, v1: List<Bytes>, v2: u64) -> R<bool> {
    let a467 = {
        let a465 = v0.clone();
        length_list(a465)
    };
    let a468 = {
        let a466 = v1.clone();
        length_list(a466)
    };
    if nat_eq(a467, a468) {
        let a470 = {
            let a469 = v0.clone();
            length_list(a469)
        };
        let a471 = v2;
        if nat_eq(a470, a471) {
            f104(Fn13::F122(v1.clone(), v0), true, v1)
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f64(v0: Adt2) -> bool {
    let m472 = v0;
    match m472 {
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

pub fn f65(v0: Str, v1: Adt2) -> Option<Bytes> {
    let m473 = v1;
    match m473 {
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
            Some(f12(f13(Str::lit("runtime support: type-scope: ")), f12(f13(v0), f13(Str::lit(" names a generic parameter, which the printer writes without the generics in scope")))))
        }
        Adt2::C7(r476) => {
            let v3 = (*r476).clone();
            f65(v0, v3)
        }
        Adt2::C8(r477) => {
            let v4 = (*r477).clone();
            f65(v0, v4)
        }
        Adt2::C9(r478, r479) => {
            let v5 = (*r478).clone();
            let v6 = (*r479).clone();
            let m474 = f65(v0.clone(), v5);
            match m474 {
                None => {
                    f65(v0, v6)
                }
                Some(v7) => {
                    Some(v7)
                }
            }
        }
        Adt2::C10(_, r480) => {
            let v9 = (*r480).clone();
            f65(v0, v9)
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
        Adt2::C15(r481) => {
            let v10 = (*r481).clone();
            f65(v0, v10)
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
        Adt2::C19(r482, r483) => {
            let v11 = (*r482).clone();
            let v12 = (*r483).clone();
            let m475 = f65(v0.clone(), v11);
            match m475 {
                None => {
                    f65(v0, v12)
                }
                Some(v13) => {
                    Some(v13)
                }
            }
        }
    }
}

pub fn f66(v0: Bytes) -> Option<u64> {
    let a484 = v0.clone();
    let a485 = f13(Str::lit("parse"));
    if equal(a484, a485) {
        Some(1u64)
    } else {
        let a486 = v0;
        let a487 = f13(Str::lit("collect"));
        if equal(a486, a487) {
            Some(1u64)
        } else {
            None::<u64>
        }
    }
}

pub fn f67(v0: Bytes) -> u64 {
    let a488 = v0.clone();
    let a489 = f13(Str::lit("parse"));
    if equal(a488, a489) {
        0u64
    } else {
        let a490 = v0;
        let a491 = f13(Str::lit("collect"));
        let _ = equal(a490, a491);
        0u64
    }
}

pub fn f68(v0: Adt14, p507: Adt15, p508: Adt3) -> R<Result<Adt15, Bytes>> {
    let m506 = (p507, p508);
    {
        let mut m506 = m506;
        loop {
            let step = {
                let (v1, v2) = m506;
                let m492 = v2;
                match m492 {
                    Adt3::C0(_) => {
                        Err::<(Adt15, Adt3), R<Result<Adt15, Bytes>>>(Ok(Ok::<Adt15, Bytes>(v1)))
                    }
                    Adt3::C1(_) => {
                        Err::<(Adt15, Adt3), R<Result<Adt15, Bytes>>>(Ok(Ok::<Adt15, Bytes>(v1)))
                    }
                    Adt3::C2(_) => {
                        Err::<(Adt15, Adt3), R<Result<Adt15, Bytes>>>(Ok(Ok::<Adt15, Bytes>(v1)))
                    }
                    Adt3::C3(_, _) => {
                        Err::<(Adt15, Adt3), R<Result<Adt15, Bytes>>>(Ok(Ok::<Adt15, Bytes>(v1)))
                    }
                    Adt3::C4(_, _, _) => {
                        Err::<(Adt15, Adt3), R<Result<Adt15, Bytes>>>(Ok(Ok::<Adt15, Bytes>(v1)))
                    }
                    Adt3::C5(_, _) => {
                        Err::<(Adt15, Adt3), R<Result<Adt15, Bytes>>>(Ok(Ok::<Adt15, Bytes>(v1)))
                    }
                    Adt3::C6(_, _) => {
                        Err::<(Adt15, Adt3), R<Result<Adt15, Bytes>>>(Ok(Ok::<Adt15, Bytes>(v1)))
                    }
                    Adt3::C7(_) => {
                        Err::<(Adt15, Adt3), R<Result<Adt15, Bytes>>>(Ok(Ok::<Adt15, Bytes>(v1)))
                    }
                    Adt3::C8(_, _) => {
                        Err::<(Adt15, Adt3), R<Result<Adt15, Bytes>>>(Ok(Ok::<Adt15, Bytes>(v1)))
                    }
                    Adt3::C9(_, _) => {
                        Err::<(Adt15, Adt3), R<Result<Adt15, Bytes>>>(Ok(Ok::<Adt15, Bytes>(v1)))
                    }
                    Adt3::C10(_, _, _) => {
                        Err::<(Adt15, Adt3), R<Result<Adt15, Bytes>>>(Ok(Ok::<Adt15, Bytes>(v1)))
                    }
                    Adt3::C11(_, _) => {
                        Err::<(Adt15, Adt3), R<Result<Adt15, Bytes>>>(Ok(Ok::<Adt15, Bytes>(v1)))
                    }
                    Adt3::C12(_, _, _) => {
                        Err::<(Adt15, Adt3), R<Result<Adt15, Bytes>>>(Ok(Ok::<Adt15, Bytes>(v1)))
                    }
                    Adt3::C13(_, _) => {
                        Err::<(Adt15, Adt3), R<Result<Adt15, Bytes>>>(Ok(Ok::<Adt15, Bytes>(v1)))
                    }
                    Adt3::C14(_, _, _) => {
                        Err::<(Adt15, Adt3), R<Result<Adt15, Bytes>>>(Ok(Ok::<Adt15, Bytes>(v1)))
                    }
                    Adt3::C15(_) => {
                        Err::<(Adt15, Adt3), R<Result<Adt15, Bytes>>>(Ok(Ok::<Adt15, Bytes>(v1)))
                    }
                    Adt3::C16(_, _, _) => {
                        Err::<(Adt15, Adt3), R<Result<Adt15, Bytes>>>(Ok(Ok::<Adt15, Bytes>(v1)))
                    }
                    Adt3::C17 => {
                        Err::<(Adt15, Adt3), R<Result<Adt15, Bytes>>>(Ok(Ok::<Adt15, Bytes>(v1)))
                    }
                    Adt3::C18(_) => {
                        Err::<(Adt15, Adt3), R<Result<Adt15, Bytes>>>(Ok(Ok::<Adt15, Bytes>(v1)))
                    }
                    Adt3::C19(_) => {
                        Err::<(Adt15, Adt3), R<Result<Adt15, Bytes>>>(Ok(Ok::<Adt15, Bytes>(v1)))
                    }
                    Adt3::C20(_, _, _) => {
                        Err::<(Adt15, Adt3), R<Result<Adt15, Bytes>>>(Ok(Ok::<Adt15, Bytes>(v1)))
                    }
                    Adt3::C21(_, _, _) => {
                        Err::<(Adt15, Adt3), R<Result<Adt15, Bytes>>>(Ok(Ok::<Adt15, Bytes>(v1)))
                    }
                    Adt3::C22(_) => {
                        Err::<(Adt15, Adt3), R<Result<Adt15, Bytes>>>(Ok(Ok::<Adt15, Bytes>(v1)))
                    }
                    Adt3::C23(_, _) => {
                        Err::<(Adt15, Adt3), R<Result<Adt15, Bytes>>>(Ok(Ok::<Adt15, Bytes>(v1)))
                    }
                    Adt3::C24(_, _) => {
                        Err::<(Adt15, Adt3), R<Result<Adt15, Bytes>>>(Ok(Ok::<Adt15, Bytes>(v1)))
                    }
                    Adt3::C25(_, _, _) => {
                        Err::<(Adt15, Adt3), R<Result<Adt15, Bytes>>>(Ok(Ok::<Adt15, Bytes>(v1)))
                    }
                    Adt3::C26(_) => {
                        Err::<(Adt15, Adt3), R<Result<Adt15, Bytes>>>(Ok(Ok::<Adt15, Bytes>(v1)))
                    }
                    Adt3::C27(_) => {
                        Err::<(Adt15, Adt3), R<Result<Adt15, Bytes>>>(Ok(Ok::<Adt15, Bytes>(v1)))
                    }
                    Adt3::C28(_, _, _, _) => {
                        Err::<(Adt15, Adt3), R<Result<Adt15, Bytes>>>(Ok(Ok::<Adt15, Bytes>(v1)))
                    }
                    Adt3::C29(_, _) => {
                        Err::<(Adt15, Adt3), R<Result<Adt15, Bytes>>>(Ok(Ok::<Adt15, Bytes>(v1)))
                    }
                    Adt3::C30 => {
                        Err::<(Adt15, Adt3), R<Result<Adt15, Bytes>>>(Ok(Ok::<Adt15, Bytes>(v1)))
                    }
                    Adt3::C31 => {
                        Err::<(Adt15, Adt3), R<Result<Adt15, Bytes>>>(Ok(Ok::<Adt15, Bytes>(v1)))
                    }
                    Adt3::C32(r502, r503) => {
                        let v61 = (*r502).clone();
                        let v62 = (*r503).clone();
                        let m493 = f9(v0.clone(), v1.clone(), v61)?;
                        match m493 {
                            None => {
                                Ok::<(Adt15, Adt3), R<Result<Adt15, Bytes>>>((v1, v62))
                            }
                            Some(v63) => {
                                Err::<(Adt15, Adt3), R<Result<Adt15, Bytes>>>(Ok(Err::<Adt15, Bytes>(v63)))
                            }
                        }
                    }
                    Adt3::C33(v64, v65, r504, r505) => {
                        let v66 = (*r504).clone();
                        let v67 = (*r505).clone();
                        let m498 = if f64(v65.clone()) {
                            None::<Bytes>
                        } else {
                            f28(f8(v0.clone(), v65.clone(), {
                                let a495 = match v1.clone() {
                                    Adt15::C0(h494, _, _, _) => {
                                        h494
                                    }
                                };
                                length_list(a495)
                            }, {
                                let a497 = f64(match v1.clone() {
                                    Adt15::C0(_, _, h496, _) => {
                                        h496
                                    }
                                });
                                bool_not(a497)
                            })?, f65(Str::lit("a `let` type"), v65))
                        };
                        match m498 {
                            None => {
                                let m499 = f9(v0.clone(), v1.clone(), v66)?;
                                match m499 {
                                    None => {
                                        let m500 = f60(List::<Adt16>::nil(), v64.clone())?;
                                        match m500 {
                                            Ok(v71) => {
                                                let m501 = f59(v0.clone(), v64)?;
                                                match m501 {
                                                    None => {
                                                        Ok::<(Adt15, Adt3), R<Result<Adt15, Bytes>>>((f58(v1, v71, false), v67))
                                                    }
                                                    Some(v72) => {
                                                        Err::<(Adt15, Adt3), R<Result<Adt15, Bytes>>>(Ok(Err::<Adt15, Bytes>(v72)))
                                                    }
                                                }
                                            }
                                            Err(v70) => {
                                                Err::<(Adt15, Adt3), R<Result<Adt15, Bytes>>>(Ok(Err::<Adt15, Bytes>(v70)))
                                            }
                                        }
                                    }
                                    Some(v69) => {
                                        Err::<(Adt15, Adt3), R<Result<Adt15, Bytes>>>(Ok(Err::<Adt15, Bytes>(v69)))
                                    }
                                }
                            }
                            Some(v68) => {
                                Err::<(Adt15, Adt3), R<Result<Adt15, Bytes>>>(Ok(Err::<Adt15, Bytes>(v68)))
                            }
                        }
                    }
                    Adt3::C34(_, _, _, _) => {
                        Err::<(Adt15, Adt3), R<Result<Adt15, Bytes>>>(Ok(Ok::<Adt15, Bytes>(v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m506 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f69(v0: Fn5, p512: Adt17, p513: List<Adt5>) -> R<Adt17> {
    let m511 = (p512, p513);
    {
        let mut m511 = m511;
        loop {
            let step = {
                let (v1, v2) = m511;
                let m509 = v2;
                match m509.uncons() {
                    None => {
                        Err::<(Adt17, List<Adt5>), R<Adt17>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt17, List<Adt5>), R<Adt17>>(({
                            let c510: Fn5 = v0.clone();
                            c510.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m511 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f70(v0: Adt17, v1: Adt5) -> R<Adt17> {
    let m515 = match v0.clone() {
        Adt17::C0(_, h514) => {
            h514
        }
    };
    match m515 {
        None => {
            if f46(match v0.clone() {
                Adt17::C0(h516, _) => {
                    h516
                }
            }, match v1.clone() {
                Adt5::C0(h517, _, _, _, _) => {
                    h517
                }
            })? {
                Ok(Adt17::C0(match v0 {
                    Adt17::C0(h518, _) => {
                        h518
                    }
                }, Some(match v1 {
                    Adt5::C0(h519, _, _, _, _) => {
                        h519
                    }
                })))
            } else {
                Ok(Adt17::C0(List::cons(match v1 {
                    Adt5::C0(h520, _, _, _, _) => {
                        h520
                    }
                }, match v0 {
                    Adt17::C0(h521, _) => {
                        h521
                    }
                }), None::<Bytes>))
            }
        }
        Some(_) => {
            Ok(v0)
        }
    }
}

pub fn f71(v0: Adt14, v1: Adt5, v2: u64, v3: bool) -> R<Option<Bytes>> {
    let m523 = f32(Str::lit("a method name"), match v1.clone() {
        Adt5::C0(h522, _, _, _, _) => {
            h522
        }
    })?;
    match m523 {
        None => {
            if if match v1.clone() {
                Adt5::C0(_, _, h524, _, _) => {
                    h524
                }
            } {
                let a526 = match v1.clone() {
                    Adt5::C0(_, h525, _, _, _) => {
                        h525
                    }
                };
                bool_not(a526)
            } else {
                false
            } {
                Ok(Some(f12(f13(Str::lit("runtime support: function-receiver: ")), f12(f14(match v1 {
                    Adt5::C0(h527, _, _, _, _) => {
                        h527
                    }
                }), f13(Str::lit(" is exclusive without a receiver"))))))
            } else {
                let m529 = f123(v0.clone(), match v1.clone() {
                    Adt5::C0(_, _, _, h528, _) => {
                        h528
                    }
                }, v2, v3)?;
                match m529 {
                    None => {
                        f8(v0, match v1 {
                            Adt5::C0(_, _, _, _, h530) => {
                                h530
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

pub fn f72(v0: Fn6, p534: Option<Bytes>, p535: List<Adt7>) -> R<Option<Bytes>> {
    let m533 = (p534, p535);
    {
        let mut m533 = m533;
        loop {
            let step = {
                let (v1, v2) = m533;
                let m531 = v2;
                match m531.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Adt7>), R<Option<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Adt7>), R<Option<Bytes>>>(({
                            let c532: Fn6 = v0.clone();
                            c532.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m533 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f73(v0: Adt14, v1: Option<Bytes>, v2: Adt7) -> R<Option<Bytes>> {
    let m536 = v1;
    match m536 {
        None => {
            let m538 = f32(Str::lit("a generic parameter"), match v2.clone() {
                Adt7::C0(h537, _) => {
                    h537
                }
            })?;
            match m538 {
                None => {
                    f81(Fn8::F124(v2.clone(), v0), None::<Bytes>, match v2 {
                        Adt7::C0(_, h539) => {
                            h539
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

pub fn f74(v0: Adt14, v1: Bytes) -> Option<List<Adt5>> {
    let m540 = f125(v0, v1.clone());
    match m540 {
        None => {
            let m542 = f126(v1.clone());
            match m542 {
                None => {
                    None::<List<Adt5>>
                }
                Some(v3) => {
                    Some(List::cons(Adt5::C0(v3, true, f127(v1.clone()), List::<Adt6>::nil(), if f127(v1) {
                        Adt2::C0
                    } else {
                        Adt2::C5
                    }), List::<Adt5>::nil()))
                }
            }
        }
        Some(v2) => {
            Some(match v2 {
                Adt10::C0(_, h541) => {
                    h541
                }
            })
        }
    }
}

pub fn f75(v0: List<Adt8>, v1: Bytes, v2: Option<Bytes>, v3: Adt5) -> R<Option<Bytes>> {
    let m543 = v2;
    match m543 {
        None => {
            let v5: u64 = f128(v0.clone(), match v3.clone() {
                Adt5::C0(h544, _, _, _, _) => {
                    h544
                }
            })?;
            let a545 = v5;
            let a546 = 0u64;
            if nat_eq(a545, a546) {
                Ok(Some(f12(f13(Str::lit("runtime support: impl-complete: ")), f12(f13(Str::lit("`impl ")), f12(v1, f12(f13(Str::lit("` does not state ")), f14(match v3 {
                    Adt5::C0(h547, _, _, _, _) => {
                        h547
                    }
                })))))))
            } else {
                let a548 = v5;
                let a549 = 1u64;
                if nat_eq(a548, a549) {
                    let m551 = f129(v0, match v3.clone() {
                        Adt5::C0(h550, _, _, _, _) => {
                            h550
                        }
                    });
                    match m551 {
                        None => {
                            Ok(None::<Bytes>)
                        }
                        Some(v6) => {
                            let a553 = f130(match v6 {
                                Adt8::C0(h552, _, _, _) => {
                                    h552
                                }
                            })?;
                            let a554 = f130(v3.clone())?;
                            if equal(a553, a554) {
                                Ok(None::<Bytes>)
                            } else {
                                Ok(Some(f12(f13(Str::lit("runtime support: impl-signature: ")), f12(f13(Str::lit("`impl ")), f12(v1, f12(f13(Str::lit("` states ")), f12(f14(match v3 {
                                    Adt5::C0(h555, _, _, _, _) => {
                                        h555
                                    }
                                }), f13(Str::lit(" with a signature other than the trait's")))))))))
                            }
                        }
                    }
                } else {
                    Ok(Some(f12(f13(Str::lit("runtime support: impl-complete: ")), f12(f13(Str::lit("`impl ")), f12(v1, f12(f13(Str::lit("` states ")), f12(f14(match v3 {
                        Adt5::C0(h556, _, _, _, _) => {
                            h556
                        }
                    }), f13(Str::lit(" twice")))))))))
                }
            }
        }
        Some(v4) => {
            Ok(Some(v4))
        }
    }
}

pub fn f76(v0: List<Adt7>, v1: Adt14, v2: List<Adt5>, v3: Bytes, v4: Adt2, v5: Option<Bytes>, v6: Adt8) -> R<Option<Bytes>> {
    let m557 = v5;
    match m557 {
        None => {
            let a561 = f131(v2, {
                let m560 = match v6.clone() {
                    Adt8::C0(h559, _, _, _) => {
                        h559
                    }
                };
                match m560 {
                    Adt5::C0(h558, _, _, _, _) => {
                        h558
                    }
                }
            });
            if bool_not(a561) {
                Ok(Some(f12(f13(Str::lit("runtime support: impl-extra: ")), f12(f13(Str::lit("`impl ")), f12(v3, f12(f13(Str::lit("` states ")), f12(f14({
                    let m564 = match v6 {
                        Adt8::C0(h563, _, _, _) => {
                            h563
                        }
                    };
                    match m564 {
                        Adt5::C0(h562, _, _, _, _) => {
                            h562
                        }
                    }
                }), f13(Str::lit(", which the trait does not")))))))))
            } else {
                if match v6.clone() {
                    Adt8::C0(_, _, h565, _) => {
                        h565
                    }
                } {
                    Ok(Some(f12(f13(Str::lit("runtime support: impl-visibility: ")), f12(f14({
                        let m568 = match v6 {
                            Adt8::C0(h567, _, _, _) => {
                                h567
                            }
                        };
                        match m568 {
                            Adt5::C0(h566, _, _, _, _) => {
                                h566
                            }
                        }
                    }), f12(f13(Str::lit(" of `impl ")), f12(v3, f13(Str::lit("` is public; a trait method has the trait's visibility"))))))))
                } else {
                    f21(v1, v0, v4, v6)
                }
            }
        }
        Some(v7) => {
            Ok(Some(v7))
        }
    }
}

pub fn f77(v0: Result<List<Adt16>, Bytes>, v1: List<Adt6>, v2: Bytes) -> Result<List<Adt16>, Bytes> {
    f132(Fn14::F133(v2), v0, v1)
}

pub fn f78(v0: Fn7, p572: Bytes, p573: List<Bytes>) -> Bytes {
    let m571 = (p572, p573);
    {
        let mut m571 = m571;
        loop {
            let step = {
                let (v1, v2) = m571;
                let m569 = v2;
                match m569.uncons() {
                    None => {
                        Err::<(Bytes, List<Bytes>), Bytes>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Bytes, List<Bytes>), Bytes>(({
                            let c570: Fn7 = v0.clone();
                            c570.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m571 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f79(_: Bytes, v1: Bytes) -> Bytes {
    v1
}

pub fn f80(v0: List<Bytes>, v1: Bytes) -> R<Bytes> {
    let m574 = f81(Fn8::F134(v1), None::<Bytes>, v0)?;
    match m574 {
        None => {
            Ok(Bytes::lit(&[]))
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f81(v0: Fn8, p578: Option<Bytes>, p579: List<Bytes>) -> R<Option<Bytes>> {
    let m577 = (p578, p579);
    {
        let mut m577 = m577;
        loop {
            let step = {
                let (v1, v2) = m577;
                let m575 = v2;
                match m575.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Bytes>), R<Option<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Bytes>), R<Option<Bytes>>>(({
                            let c576: Fn8 = v0.clone();
                            c576.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m577 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f82(v0: Str, v1: Option<Bytes>, v2: Bytes) -> R<Option<Bytes>> {
    let m580 = v1;
    match m580 {
        None => {
            if if f25(v2.clone(), f13(Str::lit("Self"))) {
                true
            } else {
                if f25(v2.clone(), f13(Str::lit("self"))) {
                    true
                } else {
                    if f25(v2.clone(), f13(Str::lit("crate"))) {
                        true
                    } else {
                        f25(v2.clone(), f13(Str::lit("super")))
                    }
                }
            } {
                Ok(None::<Bytes>)
            } else {
                f32(v0, v2)
            }
        }
        Some(v3) => {
            Ok(Some(v3))
        }
    }
}

pub fn f83(v0: Str, v1: Option<Bytes>, v2: Bytes) -> R<Option<Bytes>> {
    let m581 = v1;
    match m581 {
        None => {
            f32(v0, v2)
        }
        Some(v3) => {
            Ok(Some(v3))
        }
    }
}

pub fn f84(v0: Bytes) -> R<bool> {
    let a583 = 0u64;
    let a584 = {
        let a582 = v0.clone();
        length_bytes(a582)
    };
    if nat_lt(a583, a584) {
        let a588 = {
            let (h585, _) = f135(Fn15::F136(v0.clone()), {
                let a586 = v0.clone();
                length_bytes(a586)
            }, 0u64)?;
            h585
        };
        let a589 = {
            let a587 = v0;
            length_bytes(a587)
        };
        Ok(nat_eq(a588, a589))
    } else {
        Ok(false)
    }
}

pub fn f85() -> List<Bytes> {
    List::cons(f13(Str::lit("abstract")), List::cons(f13(Str::lit("as")), List::cons(f13(Str::lit("async")), List::cons(f13(Str::lit("await")), List::cons(f13(Str::lit("become")), List::cons(f13(Str::lit("box")), List::cons(f13(Str::lit("break")), List::cons(f13(Str::lit("const")), List::cons(f13(Str::lit("continue")), List::cons(f13(Str::lit("crate")), List::cons(f13(Str::lit("do")), List::cons(f13(Str::lit("dyn")), List::cons(f13(Str::lit("else")), List::cons(f13(Str::lit("enum")), List::cons(f13(Str::lit("extern")), List::cons(f13(Str::lit("false")), List::cons(f13(Str::lit("final")), List::cons(f13(Str::lit("fn")), List::cons(f13(Str::lit("for")), List::cons(f13(Str::lit("if")), List::cons(f13(Str::lit("impl")), List::cons(f13(Str::lit("in")), List::cons(f13(Str::lit("let")), List::cons(f13(Str::lit("loop")), List::cons(f13(Str::lit("macro")), List::cons(f13(Str::lit("match")), List::cons(f13(Str::lit("mod")), List::cons(f13(Str::lit("move")), List::cons(f13(Str::lit("mut")), List::cons(f13(Str::lit("override")), List::cons(f13(Str::lit("priv")), List::cons(f13(Str::lit("pub")), List::cons(f13(Str::lit("ref")), List::cons(f13(Str::lit("return")), List::cons(f13(Str::lit("self")), List::cons(f13(Str::lit("Self")), List::cons(f13(Str::lit("static")), List::cons(f13(Str::lit("struct")), List::cons(f13(Str::lit("super")), List::cons(f13(Str::lit("trait")), List::cons(f13(Str::lit("true")), List::cons(f13(Str::lit("try")), List::cons(f13(Str::lit("type")), List::cons(f13(Str::lit("typeof")), List::cons(f13(Str::lit("unsafe")), List::cons(f13(Str::lit("unsized")), List::cons(f13(Str::lit("use")), List::cons(f13(Str::lit("virtual")), List::cons(f13(Str::lit("where")), List::cons(f13(Str::lit("while")), List::cons(f13(Str::lit("yield")), List::cons(f13(Str::lit("_")), List::<Bytes>::nil()))))))))))))))))))))))))))))))))))))))))))))))))))))
}

pub fn f86(v0: Str, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m590 = v1;
    match m590 {
        None => {
            if if f25(v2.clone(), f13(Str::lit("Clone"))) {
                true
            } else {
                if f25(v2.clone(), f13(Str::lit("Copy"))) {
                    true
                } else {
                    if f25(v2.clone(), f13(Str::lit("Debug"))) {
                        true
                    } else {
                        if f25(v2.clone(), f13(Str::lit("Eq"))) {
                            true
                        } else {
                            f25(v2.clone(), f13(Str::lit("PartialEq")))
                        }
                    }
                }
            } {
                None::<Bytes>
            } else {
                Some(f12(f13(Str::lit("runtime support: derive: ")), f12(f13(v0), f12(f13(Str::lit(" derives ")), f12(f14(v2), f13(Str::lit(", which is not in the closed table")))))))
            }
        }
        Some(v3) => {
            Some(v3)
        }
    }
}

pub fn f87(v0: Fn9, p594: Adt14, p595: List<Adt5>) -> Adt14 {
    let m593 = (p594, p595);
    {
        let mut m593 = m593;
        loop {
            let step = {
                let (v1, v2) = m593;
                let m591 = v2;
                match m591.uncons() {
                    None => {
                        Err::<(Adt14, List<Adt5>), Adt14>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt14, List<Adt5>), Adt14>(({
                            let c592: Fn9 = v0.clone();
                            c592.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m593 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f88(v0: Adt14, v1: Adt5) -> Adt14 {
    Adt14::C0(match v0.clone() {
        Adt14::C0(h596, _, _, _, _, _, _, _, _, _) => {
            h596
        }
    }, match v0.clone() {
        Adt14::C0(_, h597, _, _, _, _, _, _, _, _) => {
            h597
        }
    }, match v0.clone() {
        Adt14::C0(_, _, h598, _, _, _, _, _, _, _) => {
            h598
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, h599, _, _, _, _, _, _) => {
            h599
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, h600, _, _, _, _, _) => {
            h600
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, h601, _, _, _, _) => {
            h601
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, h602, _, _, _) => {
            h602
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, h603, _, _) => {
            h603
        }
    }, {
        let a608 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, h604, _) => {
                h604
            }
        };
        let a609 = List::cons(Adt13::C0(match v1.clone() {
            Adt5::C0(h605, _, _, _, _) => {
                h605
            }
        }, {
            let a607 = match v1 {
                Adt5::C0(_, _, _, h606, _) => {
                    h606
                }
            };
            length_list(a607)
        }), List::<Adt13>::nil());
        append_list(a608, a609)
    }, match v0 {
        Adt14::C0(_, _, _, _, _, _, _, _, _, h610) => {
            h610
        }
    })
}

pub fn f89(v0: Adt2) -> Option<Bytes> {
    let m611 = v0;
    match m611 {
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

pub fn f90(v0: Adt14, v1: Bytes, v2: List<Adt8>) -> Adt14 {
    f91(Fn10::F137(v1), v0, v2)
}

pub fn f91(v0: Fn10, p615: Adt14, p616: List<Adt8>) -> Adt14 {
    let m614 = (p615, p616);
    {
        let mut m614 = m614;
        loop {
            let step = {
                let (v1, v2) = m614;
                let m612 = v2;
                match m612.uncons() {
                    None => {
                        Err::<(Adt14, List<Adt8>), Adt14>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt14, List<Adt8>), Adt14>(({
                            let c613: Fn10 = v0.clone();
                            c613.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m614 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f92(v0: Adt14, v1: Adt8) -> Adt14 {
    Adt14::C0(match v0.clone() {
        Adt14::C0(h617, _, _, _, _, _, _, _, _, _) => {
            h617
        }
    }, match v0.clone() {
        Adt14::C0(_, h618, _, _, _, _, _, _, _, _) => {
            h618
        }
    }, match v0.clone() {
        Adt14::C0(_, _, h619, _, _, _, _, _, _, _) => {
            h619
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, h620, _, _, _, _, _, _) => {
            h620
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, h621, _, _, _, _, _) => {
            h621
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, h622, _, _, _, _) => {
            h622
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, h623, _, _, _) => {
            h623
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, h624, _, _) => {
            h624
        }
    }, {
        let a633 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, h625, _) => {
                h625
            }
        };
        let a634 = List::cons(Adt13::C0({
            let m628 = match v1.clone() {
                Adt8::C0(h627, _, _, _) => {
                    h627
                }
            };
            match m628 {
                Adt5::C0(h626, _, _, _, _) => {
                    h626
                }
            }
        }, {
            let a632 = {
                let m631 = match v1 {
                    Adt8::C0(h630, _, _, _) => {
                        h630
                    }
                };
                match m631 {
                    Adt5::C0(_, _, _, h629, _) => {
                        h629
                    }
                }
            };
            length_list(a632)
        }), List::<Adt13>::nil());
        append_list(a633, a634)
    }, match v0 {
        Adt14::C0(_, _, _, _, _, _, _, _, _, h635) => {
            h635
        }
    })
}

pub fn f93(v0: u64) -> R<Bytes> {
    let v1: Bytes = f138(Str::lit("0"));
    let v2: Bytes = f139({
        let a672 = f140(v0, 10000000000000000000u64);
        let a673 = {
            let a670 = f140(v0, 1000000000000000000u64);
            let a671 = {
                let a668 = f140(v0, 100000000000000000u64);
                let a669 = {
                    let a666 = f140(v0, 10000000000000000u64);
                    let a667 = {
                        let a664 = f140(v0, 1000000000000000u64);
                        let a665 = {
                            let a662 = f140(v0, 100000000000000u64);
                            let a663 = {
                                let a660 = f140(v0, 10000000000000u64);
                                let a661 = {
                                    let a658 = f140(v0, 1000000000000u64);
                                    let a659 = {
                                        let a656 = f140(v0, 100000000000u64);
                                        let a657 = {
                                            let a654 = f140(v0, 10000000000u64);
                                            let a655 = {
                                                let a652 = f140(v0, 1000000000u64);
                                                let a653 = {
                                                    let a650 = f140(v0, 100000000u64);
                                                    let a651 = {
                                                        let a648 = f140(v0, 10000000u64);
                                                        let a649 = {
                                                            let a646 = f140(v0, 1000000u64);
                                                            let a647 = {
                                                                let a644 = f140(v0, 100000u64);
                                                                let a645 = {
                                                                    let a642 = f140(v0, 10000u64);
                                                                    let a643 = {
                                                                        let a640 = f140(v0, 1000u64);
                                                                        let a641 = {
                                                                            let a638 = f140(v0, 100u64);
                                                                            let a639 = {
                                                                                let a636 = f140(v0, 10u64);
                                                                                let a637 = f140(v0, 1u64);
                                                                                append_bytes(a636, a637)
                                                                            };
                                                                            append_bytes(a638, a639)
                                                                        };
                                                                        append_bytes(a640, a641)
                                                                    };
                                                                    append_bytes(a642, a643)
                                                                };
                                                                append_bytes(a644, a645)
                                                            };
                                                            append_bytes(a646, a647)
                                                        };
                                                        append_bytes(a648, a649)
                                                    };
                                                    append_bytes(a650, a651)
                                                };
                                                append_bytes(a652, a653)
                                            };
                                            append_bytes(a654, a655)
                                        };
                                        append_bytes(a656, a657)
                                    };
                                    append_bytes(a658, a659)
                                };
                                append_bytes(a660, a661)
                            };
                            append_bytes(a662, a663)
                        };
                        append_bytes(a664, a665)
                    };
                    append_bytes(a666, a667)
                };
                append_bytes(a668, a669)
            };
            append_bytes(a670, a671)
        };
        append_bytes(a672, a673)
    })?;
    let a674 = v0;
    let a675 = 0u64;
    if nat_eq(a674, a675) {
        Ok(v1)
    } else {
        Ok(v2)
    }
}

pub fn f94(v0: Fn11, p679: Adt17, p680: List<Adt9>) -> R<Adt17> {
    let m678 = (p679, p680);
    {
        let mut m678 = m678;
        loop {
            let step = {
                let (v1, v2) = m678;
                let m676 = v2;
                match m676.uncons() {
                    None => {
                        Err::<(Adt17, List<Adt9>), R<Adt17>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt17, List<Adt9>), R<Adt17>>(({
                            let c677: Fn11 = v0.clone();
                            c677.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m678 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f95(v0: Bytes, v1: Adt17, v2: Adt9) -> R<Adt17> {
    let m682 = match v1.clone() {
        Adt17::C0(_, h681) => {
            h681
        }
    };
    match m682 {
        None => {
            let m683 = f32(Str::lit("a variant name"), f141(v2.clone()))?;
            match m683 {
                None => {
                    let m686 = {
                        let m685 = v2.clone();
                        match m685 {
                            Adt9::C0(_) => {
                                None::<Bytes>
                            }
                            Adt9::C1(_, _) => {
                                None::<Bytes>
                            }
                            Adt9::C2(_, v9, _) => {
                                f30(Str::lit("a record variant field name"), v9)?
                            }
                        }
                    };
                    match m686 {
                        None => {
                            if f46(match v1.clone() {
                                Adt17::C0(h688, _) => {
                                    h688
                                }
                            }, f141(v2.clone()))? {
                                Ok(Adt17::C0(match v1 {
                                    Adt17::C0(h689, _) => {
                                        h689
                                    }
                                }, Some(f12(f13(Str::lit("runtime support: variant-name: ")), f12(f14(v0), f12(f13(Str::lit(" declares ")), f12(f14(f141(v2)), f13(Str::lit(" twice")))))))))
                            } else {
                                Ok(Adt17::C0(List::cons(f141(v2), match v1 {
                                    Adt17::C0(h690, _) => {
                                        h690
                                    }
                                }), None::<Bytes>))
                            }
                        }
                        Some(v11) => {
                            Ok(Adt17::C0(match v1 {
                                Adt17::C0(h687, _) => {
                                    h687
                                }
                            }, Some(v11)))
                        }
                    }
                }
                Some(v4) => {
                    Ok(Adt17::C0(match v1 {
                        Adt17::C0(h684, _) => {
                            h684
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

pub fn f96(v0: Adt17, v1: Adt9) -> R<Adt17> {
    let m692 = match v0.clone() {
        Adt17::C0(_, h691) => {
            h691
        }
    };
    match m692 {
        None => {
            if f46(match v0.clone() {
                Adt17::C0(h693, _) => {
                    h693
                }
            }, f141(v1.clone()))? {
                Ok(Adt17::C0(match v0 {
                    Adt17::C0(h694, _) => {
                        h694
                    }
                }, Some(f141(v1))))
            } else {
                Ok(Adt17::C0(List::cons(f141(v1), match v0 {
                    Adt17::C0(h695, _) => {
                        h695
                    }
                }), None::<Bytes>))
            }
        }
        Some(_) => {
            Ok(v0)
        }
    }
}

pub fn f97(v0: Adt14, v1: Bytes) -> Option<Adt11> {
    f142(Fn16::F143(v1), None::<Adt11>, match v0 {
        Adt14::C0(_, _, _, h696, _, _, _, _, _, _) => {
            h696
        }
    })
}

pub fn f98(v0: Adt14, v1: Bytes) -> Option<Adt12> {
    f144(Fn17::F145(v1), None::<Adt12>, match v0 {
        Adt14::C0(_, _, _, _, h697, _, _, _, _, _) => {
            h697
        }
    })
}

pub fn f99(v0: Adt14, v1: Bytes) -> Option<Adt13> {
    f146(Fn18::F147(v1), None::<Adt13>, match v0 {
        Adt14::C0(_, _, _, _, _, h698, _, _, _, _) => {
            h698
        }
    })
}

pub fn f100(v0: Bytes) -> Option<u64> {
    let a699 = v0.clone();
    let a700 = f13(Str::lit("Result"));
    if equal(a699, a700) {
        Some(2u64)
    } else {
        let a701 = v0.clone();
        let a702 = f13(Str::lit("Option"));
        if equal(a701, a702) {
            Some(1u64)
        } else {
            let a703 = v0.clone();
            let a704 = f13(Str::lit("AtomicU64"));
            if equal(a703, a704) {
                Some(0u64)
            } else {
                let a705 = v0.clone();
                let a706 = f13(Str::lit("Rc"));
                if equal(a705, a706) {
                    Some(1u64)
                } else {
                    let a707 = v0.clone();
                    let a708 = f13(Str::lit("Vec"));
                    if equal(a707, a708) {
                        Some(1u64)
                    } else {
                        let a709 = v0;
                        let a710 = f13(Str::lit("String"));
                        if equal(a709, a710) {
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

pub fn f101(v0: Bytes) -> bool {
    let a711 = v0.clone();
    let a712 = f13(Str::lit("Rc"));
    if equal(a711, a712) {
        true
    } else {
        let a713 = v0.clone();
        let a714 = f13(Str::lit("Vec"));
        if equal(a713, a714) {
            true
        } else {
            let a715 = v0;
            let a716 = f13(Str::lit("String"));
            equal(a715, a716)
        }
    }
}

pub fn f102(v0: Fn12, p720: Option<Adt16>, p721: List<Adt16>) -> Option<Adt16> {
    let m719 = (p720, p721);
    {
        let mut m719 = m719;
        loop {
            let step = {
                let (v1, v2) = m719;
                let m717 = v2;
                match m717.uncons() {
                    None => {
                        Err::<(Option<Adt16>, List<Adt16>), Option<Adt16>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Adt16>, List<Adt16>), Option<Adt16>>(({
                            let c718: Fn12 = v0.clone();
                            c718.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m719 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f103(v0: Bytes, v1: Option<Adt16>, v2: Adt16) -> Option<Adt16> {
    let m722 = v1;
    match m722 {
        None => {
            if f25(match v2.clone() {
                Adt16::C0(h723, _, _) => {
                    h723
                }
            }, v0) {
                Some(v2)
            } else {
                None::<Adt16>
            }
        }
        Some(v3) => {
            Some(v3)
        }
    }
}

pub fn f104(v0: Fn13, p727: bool, p728: List<Bytes>) -> R<bool> {
    let m726 = (p727, p728);
    {
        let mut m726 = m726;
        loop {
            let step = {
                let (v1, v2) = m726;
                let m724 = v2;
                match m724.uncons() {
                    None => {
                        Err::<(bool, List<Bytes>), R<bool>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<Bytes>), R<bool>>(({
                            let c725: Fn13 = v0.clone();
                            c725.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m726 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f105(v0: Bytes, v1: bool, v2: Bytes) -> bool {
    if v1 {
        true
    } else {
        f25(v2, v0)
    }
}

pub fn f106(v0: Adt14, v1: List<Bytes>) -> R<Option<Bytes>> {
    let v2: Bytes = f148(v1.clone());
    let m729 = f98(v0.clone(), v2.clone());
    match m729 {
        None => {
            if if f46(match v0.clone() {
                Adt14::C0(_, _, _, _, _, _, _, _, _, h733) => {
                    h733
                }
            }, v2.clone())? {
                true
            } else {
                let m734 = f97(v0.clone(), v2.clone());
                if match m734 {
                    None => {
                        false
                    }
                    Some(_) => {
                        true
                    }
                } {
                    true
                } else {
                    if f46(match v0 {
                        Adt14::C0(_, _, _, _, _, _, h735, _, _, _) => {
                            h735
                        }
                    }, v2.clone())? {
                        true
                    } else {
                        if f151(v2.clone()) {
                            true
                        } else {
                            if f25(v2.clone(), f13(Str::lit("None"))) {
                                true
                            } else {
                                if f25(v2.clone(), f13(Str::lit("Self"))) {
                                    true
                                } else {
                                    f25(v2.clone(), f13(Str::lit("usize")))
                                }
                            }
                        }
                    }
                }
            } {
                Ok(None::<Bytes>)
            } else {
                Ok(Some(f12(f13(Str::lit("runtime support: path-name: ")), f12(f14(f27(v1, Str::lit("::"))?), f12(f13(Str::lit(" starts at ")), f12(f14(v2), f13(Str::lit(", which no `use` or item declares"))))))))
            }
        }
        Some(v3) => {
            let m730 = f149(v1.clone(), 1u64);
            match m730 {
                None => {
                    Ok(Some(f12(f13(Str::lit("runtime support: variant-name: ")), f12(f14(f27(v1, Str::lit("::"))?), f12(f13(Str::lit(" is not a variant of ")), f14(v2))))))
                }
                Some(v4) => {
                    let m732 = f150(match v3 {
                        Adt12::C0(_, h731, _) => {
                            h731
                        }
                    }, v4);
                    match m732 {
                        None => {
                            Ok(Some(f12(f13(Str::lit("runtime support: variant-name: ")), f12(f14(f27(v1, Str::lit("::"))?), f12(f13(Str::lit(" is not a variant of ")), f14(v2))))))
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

pub fn f107(v0: Adt14, v1: List<Bytes>) -> bool {
    if match v0 {
        Adt14::C0(h736, _, _, _, _, _, _, _, _, _) => {
            h736
        }
    } {
        let m737 = v1;
        match m737.uncons() {
            None => {
                false
            }
            Some((v2, v3)) => {
                let m738 = v3;
                match m738.uncons() {
                    None => {
                        false
                    }
                    Some((v4, v5)) => {
                        let m739 = v5;
                        match m739.uncons() {
                            None => {
                                f152(v2, v4)
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

pub fn f108(v0: Adt14, v1: List<Bytes>) -> R<Option<u64>> {
    let m740 = v1;
    match m740.uncons() {
        None => {
            Ok(None::<u64>)
        }
        Some((v2, v3)) => {
            let m741 = v3;
            match m741.uncons() {
                None => {
                    let m742 = f153(v0.clone(), v2.clone());
                    match m742 {
                        None => {
                            let m744 = f154(v2.clone());
                            match m744 {
                                None => {
                                    Ok(f155(v0, v2))
                                }
                                Some(v5) => {
                                    Ok(Some(v5))
                                }
                            }
                        }
                        Some(v4) => {
                            Ok(Some(match v4 {
                                Adt13::C0(_, h743) => {
                                    h743
                                }
                            }))
                        }
                    }
                }
                Some((v6, v7)) => {
                    let m745 = v7;
                    match m745.uncons() {
                        None => {
                            let m746 = f156(v2.clone(), v6.clone());
                            match m746 {
                                None => {
                                    let m747 = f157(v0.clone(), v2.clone(), v6.clone())?;
                                    match m747 {
                                        None => {
                                            Ok(f158(v0, v2, v6))
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

pub fn f109(v0: Bytes) -> Option<u64> {
    let a748 = v0.clone();
    let a749 = f13(Str::lit("fetch_add"));
    if equal(a748, a749) {
        Some(2u64)
    } else {
        let a750 = v0.clone();
        let a751 = f13(Str::lit("load"));
        if equal(a750, a751) {
            Some(1u64)
        } else {
            let a752 = v0.clone();
            let a753 = f13(Str::lit("cmp"));
            if equal(a752, a753) {
                Some(1u64)
            } else {
                let a754 = v0.clone();
                let a755 = f13(Str::lit("len"));
                if equal(a754, a755) {
                    Some(0u64)
                } else {
                    let a756 = v0.clone();
                    let a757 = f13(Str::lit("is_empty"));
                    if equal(a756, a757) {
                        Some(0u64)
                    } else {
                        let a758 = v0.clone();
                        let a759 = f13(Str::lit("chars"));
                        if equal(a758, a759) {
                            Some(0u64)
                        } else {
                            let a760 = v0.clone();
                            let a761 = f13(Str::lit("next"));
                            if equal(a760, a761) {
                                Some(0u64)
                            } else {
                                let a762 = v0.clone();
                                let a763 = f13(Str::lit("min"));
                                if equal(a762, a763) {
                                    Some(1u64)
                                } else {
                                    let a764 = v0.clone();
                                    let a765 = f13(Str::lit("max"));
                                    if equal(a764, a765) {
                                        Some(1u64)
                                    } else {
                                        let a766 = v0.clone();
                                        let a767 = f13(Str::lit("push"));
                                        if equal(a766, a767) {
                                            Some(1u64)
                                        } else {
                                            let a768 = v0.clone();
                                            let a769 = f13(Str::lit("pop"));
                                            if equal(a768, a769) {
                                                Some(0u64)
                                            } else {
                                                let a770 = v0.clone();
                                                let a771 = f13(Str::lit("truncate"));
                                                if equal(a770, a771) {
                                                    Some(1u64)
                                                } else {
                                                    let a772 = v0.clone();
                                                    let a773 = f13(Str::lit("push_str"));
                                                    if equal(a772, a773) {
                                                        Some(1u64)
                                                    } else {
                                                        let a774 = v0.clone();
                                                        let a775 = f13(Str::lit("extend_from_slice"));
                                                        if equal(a774, a775) {
                                                            Some(1u64)
                                                        } else {
                                                            let a776 = v0.clone();
                                                            let a777 = f13(Str::lit("iter"));
                                                            if equal(a776, a777) {
                                                                Some(0u64)
                                                            } else {
                                                                let a778 = v0.clone();
                                                                let a779 = f13(Str::lit("rev"));
                                                                if equal(a778, a779) {
                                                                    Some(0u64)
                                                                } else {
                                                                    let a780 = v0.clone();
                                                                    let a781 = f13(Str::lit("get"));
                                                                    if equal(a780, a781) {
                                                                        Some(1u64)
                                                                    } else {
                                                                        let a782 = v0.clone();
                                                                        let a783 = f13(Str::lit("copied"));
                                                                        if equal(a782, a783) {
                                                                            Some(0u64)
                                                                        } else {
                                                                            let a784 = v0.clone();
                                                                            let a785 = f13(Str::lit("ok"));
                                                                            if equal(a784, a785) {
                                                                                Some(0u64)
                                                                            } else {
                                                                                let a786 = v0.clone();
                                                                                let a787 = f13(Str::lit("clone"));
                                                                                if equal(a786, a787) {
                                                                                    Some(0u64)
                                                                                } else {
                                                                                    let a788 = v0.clone();
                                                                                    let a789 = f13(Str::lit("to_string"));
                                                                                    if equal(a788, a789) {
                                                                                        Some(0u64)
                                                                                    } else {
                                                                                        let a790 = v0.clone();
                                                                                        let a791 = f13(Str::lit("as_bytes"));
                                                                                        if equal(a790, a791) {
                                                                                            Some(0u64)
                                                                                        } else {
                                                                                            let a792 = v0.clone();
                                                                                            let a793 = f13(Str::lit("strip_prefix"));
                                                                                            if equal(a792, a793) {
                                                                                                Some(1u64)
                                                                                            } else {
                                                                                                let a794 = v0.clone();
                                                                                                let a795 = f13(Str::lit("starts_with"));
                                                                                                if equal(a794, a795) {
                                                                                                    Some(1u64)
                                                                                                } else {
                                                                                                    let a796 = v0.clone();
                                                                                                    let a797 = f13(Str::lit("split"));
                                                                                                    if equal(a796, a797) {
                                                                                                        Some(1u64)
                                                                                                    } else {
                                                                                                        let a798 = v0.clone();
                                                                                                        let a799 = f13(Str::lit("collect"));
                                                                                                        if equal(a798, a799) {
                                                                                                            Some(0u64)
                                                                                                        } else {
                                                                                                            let a800 = v0;
                                                                                                            let a801 = f13(Str::lit("is_ascii_digit"));
                                                                                                            if equal(a800, a801) {
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

pub fn f110(v0: Bytes) -> bool {
    let a802 = v0.clone();
    let a803 = f13(Str::lit("push"));
    if equal(a802, a803) {
        true
    } else {
        let a804 = v0.clone();
        let a805 = f13(Str::lit("pop"));
        if equal(a804, a805) {
            true
        } else {
            let a806 = v0.clone();
            let a807 = f13(Str::lit("truncate"));
            if equal(a806, a807) {
                true
            } else {
                let a808 = v0.clone();
                let a809 = f13(Str::lit("push_str"));
                if equal(a808, a809) {
                    true
                } else {
                    let a810 = v0.clone();
                    let a811 = f13(Str::lit("extend_from_slice"));
                    if equal(a810, a811) {
                        true
                    } else {
                        let a812 = v0.clone();
                        let a813 = f13(Str::lit("to_string"));
                        if equal(a812, a813) {
                            true
                        } else {
                            let a814 = v0;
                            let a815 = f13(Str::lit("collect"));
                            equal(a814, a815)
                        }
                    }
                }
            }
        }
    }
}

pub fn f111(v0: Adt14, v1: Bytes) -> List<u64> {
    f159(Fn19::F160(v1), List::<u64>::nil(), match v0 {
        Adt14::C0(_, _, _, _, _, _, _, _, h816, _) => {
            h816
        }
    })
}

pub fn f112(v0: List<u64>, v1: u64) -> bool {
    f161(Fn20::F162(v1), false, v0)
}

pub fn f113(v0: Adt14, v1: Adt15, v2: Adt3, v3: Bytes) -> R<Option<Bytes>> {
    let m817 = f163(f164(v1.clone(), v2));
    match m817 {
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
            if f165(v0, f166(match v1 {
                Adt15::C0(h818, _, _, _) => {
                    h818
                }
            }, v4), v3.clone())? {
                Ok(None::<Bytes>)
            } else {
                Ok(Some(f12(f13(Str::lit("runtime support: method-bound: ")), f12(f14(v3), f13(Str::lit(" is called on a parameter whose bounds do not provide it"))))))
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

pub fn f114(v0: List<u64>) -> R<Bytes> {
    f167(Fn21::F168, f13(Str::lit("")), v0)
}

pub fn f115(v0: Adt3) -> R<bool> {
    let m819 = v0;
    match m819 {
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
        Adt3::C4(r850, _, _) => {
            let v6 = (*r850).clone();
            let a820 = 13u64;
            let a821 = f169(v6.clone());
            if nat_le(a820, a821) {
                f115(v6)
            } else {
                Ok(false)
            }
        }
        Adt3::C5(r851, _) => {
            let v9 = (*r851).clone();
            let a822 = 13u64;
            let a823 = f169(v9.clone());
            if nat_le(a822, a823) {
                f115(v9)
            } else {
                Ok(false)
            }
        }
        Adt3::C6(r852, _) => {
            let v11 = (*r852).clone();
            let a824 = 13u64;
            let a825 = f169(v11.clone());
            if nat_le(a824, a825) {
                f115(v11)
            } else {
                Ok(false)
            }
        }
        Adt3::C7(r853) => {
            let v13 = (*r853).clone();
            let a826 = 12u64;
            let a827 = f169(v13.clone());
            if nat_le(a826, a827) {
                f115(v13)
            } else {
                Ok(false)
            }
        }
        Adt3::C8(r854, r855) => {
            let v14 = (*r854).clone();
            let v15 = (*r855).clone();
            let a828 = 6u64;
            let a829 = f169(v14.clone());
            if if nat_le(a828, a829) {
                f115(v14)?
            } else {
                false
            } {
                Ok(true)
            } else {
                let a830 = 6u64;
                let a831 = f169(v15.clone());
                if nat_le(a830, a831) {
                    f115(v15)
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
        Adt3::C11(r856, r857) => {
            let v21 = (*r856).clone();
            let v22 = (*r857).clone();
            if f115(v21)? {
                Ok(true)
            } else {
                f115(v22)
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
        Adt3::C18(r858) => {
            let v35 = (*r858).clone();
            f115(v35)
        }
        Adt3::C19(_) => {
            Ok(false)
        }
        Adt3::C20(_, _, _) => {
            Ok(true)
        }
        Adt3::C21(v40, r859, r860) => {
            let v41 = (*r859).clone();
            let v42 = (*r860).clone();
            if if f170(v40, v41.clone()) {
                false
            } else {
                f115(v41)?
            } {
                Ok(true)
            } else {
                let a834 = {
                    let a832 = f171(v40);
                    let a833 = 1u64;
                    nat_add(a832, a833)?
                };
                let a835 = f169(v42.clone());
                if nat_le(a834, a835) {
                    f115(v42)
                } else {
                    Ok(false)
                }
            }
        }
        Adt3::C22(r861) => {
            let v43 = (*r861).clone();
            let a836 = 12u64;
            let a837 = f169(v43.clone());
            if nat_le(a836, a837) {
                f115(v43)
            } else {
                Ok(false)
            }
        }
        Adt3::C23(r862, _) => {
            let v44 = (*r862).clone();
            let a838 = 11u64;
            let a839 = f169(v44.clone());
            if nat_le(a838, a839) {
                f115(v44)
            } else {
                Ok(false)
            }
        }
        Adt3::C24(r863, _) => {
            let v46 = (*r863).clone();
            let a840 = 13u64;
            let a841 = f169(v46.clone());
            if nat_le(a840, a841) {
                f115(v46)
            } else {
                Ok(false)
            }
        }
        Adt3::C25(r864, _, _) => {
            let v48 = (*r864).clone();
            let a842 = 13u64;
            let a843 = f169(v48.clone());
            if nat_le(a842, a843) {
                f115(v48)
            } else {
                Ok(false)
            }
        }
        Adt3::C26(r865) => {
            let v51 = (*r865).clone();
            let a844 = 12u64;
            let a845 = f169(v51.clone());
            if nat_le(a844, a845) {
                f115(v51)
            } else {
                Ok(false)
            }
        }
        Adt3::C27(r866) => {
            let v52 = (*r866).clone();
            let a846 = 12u64;
            let a847 = f169(v52.clone());
            if nat_le(a846, a847) {
                f115(v52)
            } else {
                Ok(false)
            }
        }
        Adt3::C28(r867, _, _, _) => {
            let v53 = (*r867).clone();
            let a848 = 13u64;
            let a849 = f169(v53.clone());
            if nat_le(a848, a849) {
                f115(v53)
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

pub fn f116(v0: Adt2) -> bool {
    let m868 = v0;
    match m868 {
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

pub fn f117(v0: List<Bytes>) -> bool {
    let m869 = v0;
    match m869.uncons() {
        None => {
            false
        }
        Some((v1, v2)) => {
            let m870 = v2;
            match m870.uncons() {
                None => {
                    let m871 = f154(v1);
                    match m871 {
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

pub fn f118(v0: Adt14, v1: List<Bytes>, v2: Adt4) -> R<Option<Bytes>> {
    let m872 = f149(v1.clone(), 1u64);
    match m872 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v3) => {
            let m873 = f98(v0, f148(v1.clone()));
            match m873 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v4) => {
                    let m875 = f150(match v4 {
                        Adt12::C0(_, h874, _) => {
                            h874
                        }
                    }, v3);
                    match m875 {
                        None => {
                            Ok(Some(f12(f13(Str::lit("runtime support: pattern-fields: ")), f12(f14(f27(v1, Str::lit("::"))?), f12(f13(Str::lit(" is matched with ")), f12(f40(f119(v2)?)?, f13(Str::lit(" patterns"))))))))
                        }
                        Some(v5) => {
                            let a876 = f172(v5)?;
                            let a877 = f119(v2.clone())?;
                            if nat_eq(a876, a877) {
                                Ok(None::<Bytes>)
                            } else {
                                Ok(Some(f12(f13(Str::lit("runtime support: pattern-fields: ")), f12(f14(f27(v1, Str::lit("::"))?), f12(f13(Str::lit(" is matched with ")), f12(f40(f119(v2)?)?, f13(Str::lit(" patterns"))))))))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f119(v0: Adt4) -> R<u64> {
    let m878 = v0;
    match m878 {
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
        Adt4::C9(_, r881) => {
            let v12 = (*r881).clone();
            let a879 = f119(v12)?;
            let a880 = 1u64;
            nat_add(a879, a880)
        }
    }
}

pub fn f120(v0: Adt14, v1: List<Bytes>, v2: List<Bytes>) -> R<Option<Bytes>> {
    let m882 = f62(v0, v1.clone());
    match m882 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v3) => {
            f81(Fn8::F173(v3, v1), None::<Bytes>, v2)
        }
    }
}

pub fn f121(v0: List<Adt9>, v1: Bytes) -> Option<List<Bytes>> {
    let m883 = f150(v0, v1);
    match m883 {
        None => {
            None::<List<Bytes>>
        }
        Some(v2) => {
            let m884 = v2;
            match m884 {
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

pub fn f122(v0: List<Bytes>, v1: List<Bytes>, v2: bool, v3: Bytes) -> R<bool> {
    if v2 {
        let a885 = f174(v1, v3.clone())?;
        let a886 = f174(v0, v3)?;
        Ok(nat_eq(a885, a886))
    } else {
        Ok(false)
    }
}

pub fn f123(v0: Adt14, v1: List<Adt6>, v2: u64, v3: bool) -> R<Option<Bytes>> {
    f175(Fn22::F176(v3, v2, v0), None::<Bytes>, v1)
}

pub fn f124(v0: Adt7, v1: Adt14, v2: Option<Bytes>, v3: Bytes) -> Option<Bytes> {
    let m887 = v2;
    match m887 {
        None => {
            let m888 = f125(v1, v3.clone());
            if match m888 {
                None => {
                    let m889 = f126(v3.clone());
                    match m889 {
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
                Some(f12(f13(Str::lit("runtime support: bound-trait: ")), f12(f14(f12(match v0 {
                    Adt7::C0(h890, _) => {
                        h890
                    }
                }, f12(f13(Str::lit(": ")), v3))), f13(Str::lit(" names no declared trait")))))
            }
        }
        Some(v4) => {
            Some(v4)
        }
    }
}

pub fn f125(v0: Adt14, v1: Bytes) -> Option<Adt10> {
    f177(Fn23::F178(v1), None::<Adt10>, match v0 {
        Adt14::C0(_, _, h891, _, _, _, _, _, _, _) => {
            h891
        }
    })
}

pub fn f126(v0: Bytes) -> Option<Bytes> {
    let a892 = v0.clone();
    let a893 = f13(Str::lit("Clone"));
    if equal(a892, a893) {
        Some(f13(Str::lit("clone")))
    } else {
        let a894 = v0;
        let a895 = f13(Str::lit("Drop"));
        if equal(a894, a895) {
            Some(f13(Str::lit("drop")))
        } else {
            None::<Bytes>
        }
    }
}

pub fn f127(v0: Bytes) -> bool {
    let a896 = v0.clone();
    let a897 = f13(Str::lit("Clone"));
    if equal(a896, a897) {
        false
    } else {
        let a898 = v0;
        let a899 = f13(Str::lit("Drop"));
        equal(a898, a899)
    }
}

pub fn f128(v0: List<Adt8>, v1: Bytes) -> R<u64> {
    f179(Fn24::F180(v1), 0u64, v0)
}

pub fn f129(v0: List<Adt8>, v1: Bytes) -> Option<Adt8> {
    f181(Fn25::F182(v1), None::<Adt8>, v0)
}

pub fn f130(v0: Adt5) -> R<Bytes> {
    Ok(f12(match v0.clone() {
        Adt5::C0(h900, _, _, _, _) => {
            h900
        }
    }, f12(f13(Str::lit("|")), f12(if match v0.clone() {
        Adt5::C0(_, h901, _, _, _) => {
            h901
        }
    } {
        f13(Str::lit("r"))
    } else {
        f13(Str::lit("-"))
    }, f12(if match v0.clone() {
        Adt5::C0(_, _, h902, _, _) => {
            h902
        }
    } {
        f13(Str::lit("x"))
    } else {
        f13(Str::lit("-"))
    }, f12(f13(Str::lit("|")), f12(f183(Fn26::F184, f13(Str::lit("")), match v0.clone() {
        Adt5::C0(_, _, _, h903, _) => {
            h903
        }
    })?, f12(f13(Str::lit("|")), f185(match v0 {
        Adt5::C0(_, _, _, _, h904) => {
            h904
        }
    })?))))))))
}

pub fn f131(v0: List<Adt5>, v1: Bytes) -> bool {
    f186(Fn27::F187(v1), false, v0)
}

pub fn f132(v0: Fn14, p908: Result<List<Adt16>, Bytes>, p909: List<Adt6>) -> Result<List<Adt16>, Bytes> {
    let m907 = (p908, p909);
    {
        let mut m907 = m907;
        loop {
            let step = {
                let (v1, v2) = m907;
                let m905 = v2;
                match m905.uncons() {
                    None => {
                        Err::<(Result<List<Adt16>, Bytes>, List<Adt6>), Result<List<Adt16>, Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Result<List<Adt16>, Bytes>, List<Adt6>), Result<List<Adt16>, Bytes>>(({
                            let c906: Fn14 = v0.clone();
                            c906.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m907 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f133(v0: Bytes, v1: Result<List<Adt16>, Bytes>, v2: Adt6) -> Result<List<Adt16>, Bytes> {
    let m910 = v1;
    match m910 {
        Ok(v4) => {
            let m912 = f45(v4.clone(), match v2.clone() {
                Adt6::C0(h911, _, _) => {
                    h911
                }
            });
            match m912 {
                None => {
                    Ok::<List<Adt16>, Bytes>(List::cons(Adt16::C0(match v2.clone() {
                        Adt6::C0(h914, _, _) => {
                            h914
                        }
                    }, match v2.clone() {
                        Adt6::C0(_, h915, _) => {
                            h915
                        }
                    }, match v2 {
                        Adt6::C0(_, _, h916) => {
                            h916
                        }
                    }), v4))
                }
                Some(_) => {
                    Err::<List<Adt16>, Bytes>(f12(f13(Str::lit("runtime support: param-duplicate: ")), f12(f14(match v2 {
                        Adt6::C0(h913, _, _) => {
                            h913
                        }
                    }), f12(f13(Str::lit(" names two parameters of ")), f14(v0)))))
                }
            }
        }
        Err(v3) => {
            Err::<List<Adt16>, Bytes>(v3)
        }
    }
}

pub fn f134(v0: Bytes, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m917 = v1;
    match m917 {
        None => {
            Some(v2)
        }
        Some(v3) => {
            Some({
                let a920 = {
                    let a918 = v3;
                    let a919 = v0;
                    append_bytes(a918, a919)
                };
                let a921 = v2;
                append_bytes(a920, a921)
            })
        }
    }
}

pub fn f135(v0: Fn15, p926: u64, p927: u64) -> R<(u64, bool)> {
    let m925 = (p926, p927);
    {
        let mut m925 = m925;
        loop {
            let step = {
                let (v1, v2) = m925;
                let m922 = v1;
                if m922 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m922 - 1;
                    let m924 = {
                        let c923: Fn15 = v0.clone();
                        c923.apply(v2)?
                    };
                    match m924 {
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
                Ok(next) => { m925 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f136(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f188(v0, v1) {
        Ok(Some({
            let a928 = v1;
            let a929 = 1u64;
            nat_add(a928, a929)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f137(v0: Bytes, v1: Adt14, v2: Adt8) -> Adt14 {
    let m932 = match v2.clone() {
        Adt8::C0(h931, _, _, _) => {
            h931
        }
    };
    if match m932 {
        Adt5::C0(_, h930, _, _, _) => {
            h930
        }
    } {
        v1
    } else {
        Adt14::C0(match v1.clone() {
            Adt14::C0(h933, _, _, _, _, _, _, _, _, _) => {
                h933
            }
        }, match v1.clone() {
            Adt14::C0(_, h934, _, _, _, _, _, _, _, _) => {
                h934
            }
        }, match v1.clone() {
            Adt14::C0(_, _, h935, _, _, _, _, _, _, _) => {
                h935
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, h936, _, _, _, _, _, _) => {
                h936
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, _, h937, _, _, _, _, _) => {
                h937
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, _, _, h938, _, _, _, _) => {
                h938
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, h939, _, _, _) => {
                h939
            }
        }, {
            let a948 = match v1.clone() {
                Adt14::C0(_, _, _, _, _, _, _, h940, _, _) => {
                    h940
                }
            };
            let a949 = List::cons(Adt13::C0(f12(v0, f12(f13(Str::lit("::")), {
                let m943 = match v2.clone() {
                    Adt8::C0(h942, _, _, _) => {
                        h942
                    }
                };
                match m943 {
                    Adt5::C0(h941, _, _, _, _) => {
                        h941
                    }
                }
            })), {
                let a947 = {
                    let m946 = match v2 {
                        Adt8::C0(h945, _, _, _) => {
                            h945
                        }
                    };
                    match m946 {
                        Adt5::C0(_, _, _, h944, _) => {
                            h944
                        }
                    }
                };
                length_list(a947)
            }), List::<Adt13>::nil());
            append_list(a948, a949)
        }, match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, h950, _) => {
                h950
            }
        }, match v1 {
            Adt14::C0(_, _, _, _, _, _, _, _, _, h951) => {
                h951
            }
        })
    }
}

pub fn f138(v0: Str) -> Bytes {
    let a952 = v0;
    utf8_encode(a952)
}

pub fn f139(v0: Bytes) -> R<Bytes> {
    let v1: u64 = {
        let (h953, _) = f135(Fn15::F189(v0.clone()), {
            let a954 = v0.clone();
            length_bytes(a954)
        }, 0u64)?;
        h953
    };
    Ok(f190(v0.clone(), v1, {
        let a956 = {
            let a955 = v0;
            length_bytes(a955)
        };
        let a957 = v1;
        nat_sub(a956, a957)
    }))
}

pub fn f140(v0: u64, v1: u64) -> Bytes {
    f190(f138(Str::lit("0123456789")), {
        let a961 = {
            let a958 = v0;
            let a959 = v1;
            let a960 = 0u64;
            nat_quot(a958, a959, a960)
        };
        let a962 = 10u64;
        let a963 = 0u64;
        nat_rem(a961, a962, a963)
    }, 1u64)
}

pub fn f141(v0: Adt9) -> Bytes {
    let m964 = v0;
    match m964 {
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

pub fn f142(v0: Fn16, p968: Option<Adt11>, p969: List<Adt11>) -> Option<Adt11> {
    let m967 = (p968, p969);
    {
        let mut m967 = m967;
        loop {
            let step = {
                let (v1, v2) = m967;
                let m965 = v2;
                match m965.uncons() {
                    None => {
                        Err::<(Option<Adt11>, List<Adt11>), Option<Adt11>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Adt11>, List<Adt11>), Option<Adt11>>(({
                            let c966: Fn16 = v0.clone();
                            c966.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m967 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f143(v0: Bytes, v1: Option<Adt11>, v2: Adt11) -> Option<Adt11> {
    let m970 = v1;
    match m970 {
        None => {
            if f25(match v2.clone() {
                Adt11::C0(h971, _, _) => {
                    h971
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

pub fn f144(v0: Fn17, p975: Option<Adt12>, p976: List<Adt12>) -> Option<Adt12> {
    let m974 = (p975, p976);
    {
        let mut m974 = m974;
        loop {
            let step = {
                let (v1, v2) = m974;
                let m972 = v2;
                match m972.uncons() {
                    None => {
                        Err::<(Option<Adt12>, List<Adt12>), Option<Adt12>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Adt12>, List<Adt12>), Option<Adt12>>(({
                            let c973: Fn17 = v0.clone();
                            c973.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m974 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f145(v0: Bytes, v1: Option<Adt12>, v2: Adt12) -> Option<Adt12> {
    let m977 = v1;
    match m977 {
        None => {
            if f25(match v2.clone() {
                Adt12::C0(h978, _, _) => {
                    h978
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

pub fn f146(v0: Fn18, p982: Option<Adt13>, p983: List<Adt13>) -> Option<Adt13> {
    let m981 = (p982, p983);
    {
        let mut m981 = m981;
        loop {
            let step = {
                let (v1, v2) = m981;
                let m979 = v2;
                match m979.uncons() {
                    None => {
                        Err::<(Option<Adt13>, List<Adt13>), Option<Adt13>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Adt13>, List<Adt13>), Option<Adt13>>(({
                            let c980: Fn18 = v0.clone();
                            c980.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m981 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f147(v0: Bytes, v1: Option<Adt13>, v2: Adt13) -> Option<Adt13> {
    let m984 = v1;
    match m984 {
        None => {
            if f25(match v2.clone() {
                Adt13::C0(h985, _) => {
                    h985
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

pub fn f148(v0: List<Bytes>) -> Bytes {
    let m986 = v0;
    match m986.uncons() {
        None => {
            f13(Str::lit(""))
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f149(p990: List<Bytes>, p991: u64) -> Option<Bytes> {
    let m989 = (p990, p991);
    {
        let mut m989 = m989;
        loop {
            let step = {
                let (v0, v1) = m989;
                let m987 = v0;
                match m987.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        let m988 = v1;
                        if m988 == 0 {
                            Err::<(List<Bytes>, u64), Option<Bytes>>(Some(v2))
                        } else {
                            let v4 = m988 - 1;
                            Ok::<(List<Bytes>, u64), Option<Bytes>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m989 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f150(v0: List<Adt9>, v1: Bytes) -> Option<Adt9> {
    f191(Fn28::F192(v1), None::<Adt9>, v0)
}

pub fn f151(v0: Bytes) -> bool {
    let a992 = v0.clone();
    let a993 = f13(Str::lit("u8"));
    if equal(a992, a993) {
        true
    } else {
        let a994 = v0.clone();
        let a995 = f13(Str::lit("u16"));
        if equal(a994, a995) {
            true
        } else {
            let a996 = v0.clone();
            let a997 = f13(Str::lit("u32"));
            if equal(a996, a997) {
                true
            } else {
                let a998 = v0.clone();
                let a999 = f13(Str::lit("u64"));
                if equal(a998, a999) {
                    true
                } else {
                    let a1000 = v0.clone();
                    let a1001 = f13(Str::lit("u128"));
                    if equal(a1000, a1001) {
                        true
                    } else {
                        let a1002 = v0.clone();
                        let a1003 = f13(Str::lit("i8"));
                        if equal(a1002, a1003) {
                            true
                        } else {
                            let a1004 = v0.clone();
                            let a1005 = f13(Str::lit("i16"));
                            if equal(a1004, a1005) {
                                true
                            } else {
                                let a1006 = v0.clone();
                                let a1007 = f13(Str::lit("i32"));
                                if equal(a1006, a1007) {
                                    true
                                } else {
                                    let a1008 = v0.clone();
                                    let a1009 = f13(Str::lit("i64"));
                                    if equal(a1008, a1009) {
                                        true
                                    } else {
                                        let a1010 = v0;
                                        let a1011 = f13(Str::lit("i128"));
                                        equal(a1010, a1011)
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

pub fn f152(v0: Bytes, v1: Bytes) -> bool {
    let m1012 = f193(v0.clone(), v1.clone());
    match m1012 {
        None => {
            if f151(v0) {
                f194(f13(Str::lit("usize")), v1)
            } else {
                false
            }
        }
        Some(_) => {
            f194(v0, v1)
        }
    }
}

pub fn f153(v0: Adt14, v1: Bytes) -> Option<Adt13> {
    f146(Fn18::F195(v1), None::<Adt13>, match v0 {
        Adt14::C0(_, _, _, _, _, _, _, h1013, _, _) => {
            h1013
        }
    })
}

pub fn f154(v0: Bytes) -> Option<u64> {
    let a1014 = v0.clone();
    let a1015 = f13(Str::lit("Some"));
    if equal(a1014, a1015) {
        Some(1u64)
    } else {
        let a1016 = v0.clone();
        let a1017 = f13(Str::lit("Ok"));
        if equal(a1016, a1017) {
            Some(1u64)
        } else {
            let a1018 = v0;
            let a1019 = f13(Str::lit("Err"));
            if equal(a1018, a1019) {
                Some(1u64)
            } else {
                None::<u64>
            }
        }
    }
}

pub fn f155(v0: Adt14, v1: Bytes) -> Option<u64> {
    let m1020 = f97(v0, v1);
    match m1020 {
        None => {
            None::<u64>
        }
        Some(v2) => {
            Some({
                let a1022 = match v2 {
                    Adt11::C0(_, h1021, _) => {
                        h1021
                    }
                };
                length_list(a1022)
            })
        }
    }
}

pub fn f156(v0: Bytes, v1: Bytes) -> Option<u64> {
    let m1023 = f193(v0.clone(), v1.clone());
    match m1023 {
        None => {
            if f151(v0) {
                f193(f13(Str::lit("usize")), v1)
            } else {
                None::<u64>
            }
        }
        Some(v2) => {
            Some(v2)
        }
    }
}

pub fn f157(v0: Adt14, v1: Bytes, v2: Bytes) -> R<Option<u64>> {
    let m1024 = f98(v0, v1);
    match m1024 {
        None => {
            Ok(None::<u64>)
        }
        Some(v3) => {
            f196(match v3 {
                Adt12::C0(_, h1025, _) => {
                    h1025
                }
            }, v2)
        }
    }
}

pub fn f158(v0: Adt14, v1: Bytes, v2: Bytes) -> Option<u64> {
    let m1026 = f153(v0, f12(v1, f12(f13(Str::lit("::")), v2)));
    match m1026 {
        None => {
            None::<u64>
        }
        Some(v3) => {
            Some(match v3 {
                Adt13::C0(_, h1027) => {
                    h1027
                }
            })
        }
    }
}

pub fn f159(v0: Fn19, p1031: List<u64>, p1032: List<Adt13>) -> List<u64> {
    let m1030 = (p1031, p1032);
    {
        let mut m1030 = m1030;
        loop {
            let step = {
                let (v1, v2) = m1030;
                let m1028 = v2;
                match m1028.uncons() {
                    None => {
                        Err::<(List<u64>, List<Adt13>), List<u64>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<u64>, List<Adt13>), List<u64>>(({
                            let c1029: Fn19 = v0.clone();
                            c1029.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1030 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f160(v0: Bytes, v1: List<u64>, v2: Adt13) -> List<u64> {
    if f25(match v2.clone() {
        Adt13::C0(h1033, _) => {
            h1033
        }
    }, v0) {
        let a1035 = v1;
        let a1036 = List::cons(match v2 {
            Adt13::C0(_, h1034) => {
                h1034
            }
        }, List::<u64>::nil());
        append_list(a1035, a1036)
    } else {
        v1
    }
}

pub fn f161(v0: Fn20, p1040: bool, p1041: List<u64>) -> bool {
    let m1039 = (p1040, p1041);
    {
        let mut m1039 = m1039;
        loop {
            let step = {
                let (v1, v2) = m1039;
                let m1037 = v2;
                match m1037.uncons() {
                    None => {
                        Err::<(bool, List<u64>), bool>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<u64>), bool>(({
                            let c1038: Fn20 = v0.clone();
                            c1038.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1039 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f162(v0: u64, v1: bool, v2: u64) -> bool {
    if v1 {
        true
    } else {
        let a1042 = v2;
        let a1043 = v0;
        nat_eq(a1042, a1043)
    }
}

pub fn f163(p1054: Adt2) -> Adt2 {
    let m1053 = p1054;
    {
        let mut m1053 = m1053;
        loop {
            let step = {
                let v0 = m1053;
                let m1044 = v0;
                match m1044 {
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
                    Adt2::C7(r1045) => {
                        let v2 = (*r1045).clone();
                        Ok::<Adt2, Adt2>(v2)
                    }
                    Adt2::C8(r1046) => {
                        let v3 = (*r1046).clone();
                        Ok::<Adt2, Adt2>(v3)
                    }
                    Adt2::C9(r1047, r1048) => {
                        let v4 = (*r1047).clone();
                        let v5 = (*r1048).clone();
                        Err::<Adt2, Adt2>(Adt2::C9(std::rc::Rc::new(v4), std::rc::Rc::new(v5)))
                    }
                    Adt2::C10(v6, r1049) => {
                        let v7 = (*r1049).clone();
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
                    Adt2::C15(r1050) => {
                        let v8 = (*r1050).clone();
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
                    Adt2::C19(r1051, r1052) => {
                        let v9 = (*r1051).clone();
                        let v10 = (*r1052).clone();
                        Err::<Adt2, Adt2>(Adt2::C19(std::rc::Rc::new(v9), std::rc::Rc::new(v10)))
                    }
                }
            };
            match step {
                Ok(next) => { m1053 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f164(v0: Adt15, v1: Adt3) -> Adt2 {
    let m1055 = v1;
    match m1055 {
        Adt3::C0(v2) => {
            let m1057 = f45(match v0 {
                Adt15::C0(_, h1056, _, _) => {
                    h1056
                }
            }, v2);
            match m1057 {
                None => {
                    Adt2::C16
                }
                Some(v3) => {
                    match v3 {
                        Adt16::C0(_, _, h1058) => {
                            h1058
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
        Adt3::C5(r1059, v12) => {
            let v11 = (*r1059).clone();
            f197(f198(v0.clone(), f163(f164(v0, v11))), v12)
        }
        Adt3::C6(_, _) => {
            Adt2::C16
        }
        Adt3::C7(r1060) => {
            let v15 = (*r1060).clone();
            let v16: Adt2 = f164(v0, v15);
            if f199(v16.clone()) {
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

pub fn f165(v0: Adt14, v1: List<Bytes>, v2: Bytes) -> R<bool> {
    f104(Fn13::F200(v2, v0), false, v1)
}

pub fn f166(p1065: List<Adt7>, p1066: u64) -> List<Bytes> {
    let m1064 = (p1065, p1066);
    {
        let mut m1064 = m1064;
        loop {
            let step = {
                let (v0, v1) = m1064;
                let m1061 = v0;
                match m1061.uncons() {
                    None => {
                        Err::<(List<Adt7>, u64), List<Bytes>>(List::<Bytes>::nil())
                    }
                    Some((v2, v3)) => {
                        let m1062 = v1;
                        if m1062 == 0 {
                            match v2 {
                                Adt7::C0(_, h1063) => {
                                    Err::<(List<Adt7>, u64), List<Bytes>>(h1063)
                                }
                            }
                        } else {
                            let v4 = m1062 - 1;
                            Ok::<(List<Adt7>, u64), List<Bytes>>((v3, v4))
                        }
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

pub fn f167(v0: Fn21, p1070: Bytes, p1071: List<u64>) -> R<Bytes> {
    let m1069 = (p1070, p1071);
    {
        let mut m1069 = m1069;
        loop {
            let step = {
                let (v1, v2) = m1069;
                let m1067 = v2;
                match m1067.uncons() {
                    None => {
                        Err::<(Bytes, List<u64>), R<Bytes>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Bytes, List<u64>), R<Bytes>>(({
                            let c1068: Fn21 = v0.clone();
                            c1068.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1069 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f168(v0: Bytes, v1: u64) -> R<Bytes> {
    let a1072 = v0.clone();
    let a1073 = f13(Str::lit(""));
    if equal(a1072, a1073) {
        f40(v1)
    } else {
        Ok(f12(v0, f12(f13(Str::lit("/")), f40(v1)?)))
    }
}

pub fn f169(v0: Adt3) -> u64 {
    let m1074 = v0;
    match m1074 {
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
            f171(v40)
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

pub fn f170(v0: u64, v1: Adt3) -> bool {
    let a1075 = v0;
    let a1076 = 8u64;
    if if nat_eq(a1075, a1076) {
        f201(v1.clone())
    } else {
        false
    } {
        true
    } else {
        let a1079 = f169(v1);
        let a1080 = {
            let a1077 = f171(v0);
            let a1078 = 5u64;
            if nat_eq(a1077, a1078) {
                6u64
            } else {
                f171(v0)
            }
        };
        nat_lt(a1079, a1080)
    }
}

pub fn f171(v0: u64) -> u64 {
    let a1081 = v0;
    let a1082 = 6u64;
    if nat_eq(a1081, a1082) {
        3u64
    } else {
        let a1083 = v0;
        let a1084 = 5u64;
        if nat_eq(a1083, a1084) {
            4u64
        } else {
            let a1085 = v0;
            let a1086 = 7u64;
            if nat_eq(a1085, a1086) {
                5u64
            } else {
                let a1087 = v0;
                let a1088 = 8u64;
                if nat_eq(a1087, a1088) {
                    5u64
                } else {
                    let a1089 = v0;
                    let a1090 = 9u64;
                    if nat_eq(a1089, a1090) {
                        5u64
                    } else {
                        let a1091 = v0;
                        let a1092 = 10u64;
                        if nat_eq(a1091, a1092) {
                            5u64
                        } else {
                            let a1093 = v0;
                            let a1094 = 11u64;
                            if nat_eq(a1093, a1094) {
                                5u64
                            } else {
                                let a1095 = v0;
                                let a1096 = 0u64;
                                if nat_eq(a1095, a1096) {
                                    9u64
                                } else {
                                    let a1097 = v0;
                                    let a1098 = 1u64;
                                    if nat_eq(a1097, a1098) {
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

pub fn f172(v0: Adt9) -> R<u64> {
    let m1099 = v0;
    match m1099 {
        Adt9::C0(_) => {
            Ok(0u64)
        }
        Adt9::C1(_, v3) => {
            f39(v3)
        }
        Adt9::C2(_, _, v6) => {
            f39(v6)
        }
    }
}

pub fn f173(v0: List<Bytes>, v1: List<Bytes>, v2: Option<Bytes>, v3: Bytes) -> R<Option<Bytes>> {
    let m1100 = v2;
    match m1100 {
        None => {
            if f46(v0, v3.clone())? {
                Ok(None::<Bytes>)
            } else {
                Ok(Some(f12(f13(Str::lit("runtime support: pattern-fields: ")), f12(f14(f27(v1, Str::lit("::"))?), f12(f13(Str::lit(" has no field ")), f14(v3))))))
            }
        }
        Some(v4) => {
            Ok(Some(v4))
        }
    }
}

pub fn f174(v0: List<Bytes>, v1: Bytes) -> R<u64> {
    f202(Fn29::F203(v1), 0u64, v0)
}

pub fn f175(v0: Fn22, p1104: Option<Bytes>, p1105: List<Adt6>) -> R<Option<Bytes>> {
    let m1103 = (p1104, p1105);
    {
        let mut m1103 = m1103;
        loop {
            let step = {
                let (v1, v2) = m1103;
                let m1101 = v2;
                match m1101.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Adt6>), R<Option<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Adt6>), R<Option<Bytes>>>(({
                            let c1102: Fn22 = v0.clone();
                            c1102.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1103 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f176(v0: bool, v1: u64, v2: Adt14, v3: Option<Bytes>, v4: Adt6) -> R<Option<Bytes>> {
    let m1106 = v3;
    match m1106 {
        None => {
            let m1108 = f32(Str::lit("a parameter name"), match v4.clone() {
                Adt6::C0(h1107, _, _) => {
                    h1107
                }
            })?;
            match m1108 {
                None => {
                    f8(v2, match v4 {
                        Adt6::C0(_, _, h1109) => {
                            h1109
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

pub fn f177(v0: Fn23, p1113: Option<Adt10>, p1114: List<Adt10>) -> Option<Adt10> {
    let m1112 = (p1113, p1114);
    {
        let mut m1112 = m1112;
        loop {
            let step = {
                let (v1, v2) = m1112;
                let m1110 = v2;
                match m1110.uncons() {
                    None => {
                        Err::<(Option<Adt10>, List<Adt10>), Option<Adt10>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Adt10>, List<Adt10>), Option<Adt10>>(({
                            let c1111: Fn23 = v0.clone();
                            c1111.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1112 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f178(v0: Bytes, v1: Option<Adt10>, v2: Adt10) -> Option<Adt10> {
    let m1115 = v1;
    match m1115 {
        None => {
            if f25(match v2.clone() {
                Adt10::C0(h1116, _) => {
                    h1116
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

pub fn f179(v0: Fn24, p1120: u64, p1121: List<Adt8>) -> R<u64> {
    let m1119 = (p1120, p1121);
    {
        let mut m1119 = m1119;
        loop {
            let step = {
                let (v1, v2) = m1119;
                let m1117 = v2;
                match m1117.uncons() {
                    None => {
                        Err::<(u64, List<Adt8>), R<u64>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(u64, List<Adt8>), R<u64>>(({
                            let c1118: Fn24 = v0.clone();
                            c1118.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1119 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f180(v0: Bytes, v1: u64, v2: Adt8) -> R<u64> {
    if f25({
        let m1124 = match v2 {
            Adt8::C0(h1123, _, _, _) => {
                h1123
            }
        };
        match m1124 {
            Adt5::C0(h1122, _, _, _, _) => {
                h1122
            }
        }
    }, v0) {
        let a1125 = v1;
        let a1126 = 1u64;
        nat_add(a1125, a1126)
    } else {
        Ok(v1)
    }
}

pub fn f181(v0: Fn25, p1130: Option<Adt8>, p1131: List<Adt8>) -> Option<Adt8> {
    let m1129 = (p1130, p1131);
    {
        let mut m1129 = m1129;
        loop {
            let step = {
                let (v1, v2) = m1129;
                let m1127 = v2;
                match m1127.uncons() {
                    None => {
                        Err::<(Option<Adt8>, List<Adt8>), Option<Adt8>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Adt8>, List<Adt8>), Option<Adt8>>(({
                            let c1128: Fn25 = v0.clone();
                            c1128.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1129 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f182(v0: Bytes, v1: Option<Adt8>, v2: Adt8) -> Option<Adt8> {
    let m1132 = v1;
    match m1132 {
        None => {
            if f25({
                let m1135 = match v2.clone() {
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

pub fn f183(v0: Fn26, p1139: Bytes, p1140: List<Adt6>) -> R<Bytes> {
    let m1138 = (p1139, p1140);
    {
        let mut m1138 = m1138;
        loop {
            let step = {
                let (v1, v2) = m1138;
                let m1136 = v2;
                match m1136.uncons() {
                    None => {
                        Err::<(Bytes, List<Adt6>), R<Bytes>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Bytes, List<Adt6>), R<Bytes>>(({
                            let c1137: Fn26 = v0.clone();
                            c1137.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1138 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f184(v0: Bytes, v1: Adt6) -> R<Bytes> {
    Ok(f12(v0, f12(match v1.clone() {
        Adt6::C0(h1141, _, _) => {
            h1141
        }
    }, f12(if match v1.clone() {
        Adt6::C0(_, h1142, _) => {
            h1142
        }
    } {
        f13(Str::lit(":m:"))
    } else {
        f13(Str::lit(":-:"))
    }, f12(f185(match v1 {
        Adt6::C0(_, _, h1143) => {
            h1143
        }
    })?, f13(Str::lit(",")))))))
}

pub fn f185(v0: Adt2) -> R<Bytes> {
    let m1144 = v0;
    match m1144 {
        Adt2::C0 => {
            Ok(f13(Str::lit("U")))
        }
        Adt2::C1 => {
            Ok(f13(Str::lit("B")))
        }
        Adt2::C2 => {
            Ok(f13(Str::lit("N")))
        }
        Adt2::C3 => {
            Ok(f13(Str::lit("F")))
        }
        Adt2::C4 => {
            Ok(f13(Str::lit("O")))
        }
        Adt2::C5 => {
            Ok(f13(Str::lit("S")))
        }
        Adt2::C6(v1) => {
            Ok(f12(f13(Str::lit("P")), f12(f40(v1)?, f13(Str::lit(";")))))
        }
        Adt2::C7(r1145) => {
            let v2 = (*r1145).clone();
            Ok(f12(f13(Str::lit("R(")), f12(f185(v2)?, f13(Str::lit(")")))))
        }
        Adt2::C8(r1146) => {
            let v3 = (*r1146).clone();
            Ok(f12(f13(Str::lit("M(")), f12(f185(v3)?, f13(Str::lit(")")))))
        }
        Adt2::C9(r1147, r1148) => {
            let v4 = (*r1147).clone();
            let v5 = (*r1148).clone();
            Ok(f12(f13(Str::lit("T(")), f12(f185(v4)?, f12(f13(Str::lit(",")), f12(f185(v5)?, f13(Str::lit(")")))))))
        }
        Adt2::C10(v6, r1149) => {
            let v7 = (*r1149).clone();
            Ok(f12(f13(Str::lit("D")), f12(f14(v6), f12(f13(Str::lit("(")), f12(f185(v7)?, f13(Str::lit(")")))))))
        }
        Adt2::C11 => {
            Ok(f13(Str::lit("W")))
        }
        Adt2::C12 => {
            Ok(f13(Str::lit("Z")))
        }
        Adt2::C13 => {
            Ok(f13(Str::lit("C")))
        }
        Adt2::C14 => {
            Ok(f13(Str::lit("Q")))
        }
        Adt2::C15(r1150) => {
            let v8 = (*r1150).clone();
            Ok(f12(f13(Str::lit("L(")), f12(f185(v8)?, f13(Str::lit(")")))))
        }
        Adt2::C16 => {
            Ok(f13(Str::lit("?")))
        }
        Adt2::C17 => {
            Ok(f13(Str::lit("-")))
        }
        Adt2::C18 => {
            Ok(f13(Str::lit("")))
        }
        Adt2::C19(r1151, r1152) => {
            let v9 = (*r1151).clone();
            let v10 = (*r1152).clone();
            Ok(f12(f185(v9)?, f12(f13(Str::lit(";")), f185(v10)?)))
        }
    }
}

pub fn f186(v0: Fn27, p1156: bool, p1157: List<Adt5>) -> bool {
    let m1155 = (p1156, p1157);
    {
        let mut m1155 = m1155;
        loop {
            let step = {
                let (v1, v2) = m1155;
                let m1153 = v2;
                match m1153.uncons() {
                    None => {
                        Err::<(bool, List<Adt5>), bool>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<Adt5>), bool>(({
                            let c1154: Fn27 = v0.clone();
                            c1154.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1155 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f187(v0: Bytes, v1: bool, v2: Adt5) -> bool {
    if v1 {
        true
    } else {
        f25(match v2 {
            Adt5::C0(h1158, _, _, _, _) => {
                h1158
            }
        }, v0)
    }
}

pub fn f188(v0: Bytes, v1: u64) -> bool {
    let m1159 = f204(v0, v1);
    match m1159 {
        None => {
            false
        }
        Some(v2) => {
            if f205(v2, 97u8, 122u8) {
                true
            } else {
                if f205(v2, 65u8, 90u8) {
                    true
                } else {
                    let a1160 = v2;
                    let a1161 = 95u8;
                    if equal(a1160, a1161) {
                        true
                    } else {
                        let a1162 = 0u64;
                        let a1163 = v1;
                        if nat_lt(a1162, a1163) {
                            f205(v2, 48u8, 57u8)
                        } else {
                            false
                        }
                    }
                }
            }
        }
    }
}

pub fn f189(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f206(v0, v1)
}

pub fn f190(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m1167 = {
        let a1164 = v0;
        let a1165 = v1;
        let a1166 = v2;
        slice_bytes(a1164, a1165, a1166)
    };
    match m1167 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f191(v0: Fn28, p1171: Option<Adt9>, p1172: List<Adt9>) -> Option<Adt9> {
    let m1170 = (p1171, p1172);
    {
        let mut m1170 = m1170;
        loop {
            let step = {
                let (v1, v2) = m1170;
                let m1168 = v2;
                match m1168.uncons() {
                    None => {
                        Err::<(Option<Adt9>, List<Adt9>), Option<Adt9>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Adt9>, List<Adt9>), Option<Adt9>>(({
                            let c1169: Fn28 = v0.clone();
                            c1169.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1170 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f192(v0: Bytes, v1: Option<Adt9>, v2: Adt9) -> Option<Adt9> {
    let m1173 = v1;
    match m1173 {
        None => {
            if f25(f141(v2.clone()), v0) {
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

pub fn f193(v0: Bytes, v1: Bytes) -> Option<u64> {
    let a1174 = v0.clone();
    let a1175 = f13(Str::lit("AtomicU64"));
    if if equal(a1174, a1175) {
        let a1176 = v1.clone();
        let a1177 = f13(Str::lit("new"));
        equal(a1176, a1177)
    } else {
        false
    } {
        Some(1u64)
    } else {
        let a1178 = v0.clone();
        let a1179 = f13(Str::lit("Rc"));
        if if equal(a1178, a1179) {
            let a1180 = v1.clone();
            let a1181 = f13(Str::lit("new"));
            equal(a1180, a1181)
        } else {
            false
        } {
            Some(1u64)
        } else {
            let a1182 = v0.clone();
            let a1183 = f13(Str::lit("Rc"));
            if if equal(a1182, a1183) {
                let a1184 = v1.clone();
                let a1185 = f13(Str::lit("from"));
                equal(a1184, a1185)
            } else {
                false
            } {
                Some(1u64)
            } else {
                let a1186 = v0.clone();
                let a1187 = f13(Str::lit("Rc"));
                if if equal(a1186, a1187) {
                    let a1188 = v1.clone();
                    let a1189 = f13(Str::lit("try_unwrap"));
                    equal(a1188, a1189)
                } else {
                    false
                } {
                    Some(1u64)
                } else {
                    let a1190 = v0.clone();
                    let a1191 = f13(Str::lit("Rc"));
                    if if equal(a1190, a1191) {
                        let a1192 = v1.clone();
                        let a1193 = f13(Str::lit("get_mut"));
                        equal(a1192, a1193)
                    } else {
                        false
                    } {
                        Some(1u64)
                    } else {
                        let a1194 = v0.clone();
                        let a1195 = f13(Str::lit("Vec"));
                        if if equal(a1194, a1195) {
                            let a1196 = v1.clone();
                            let a1197 = f13(Str::lit("new"));
                            equal(a1196, a1197)
                        } else {
                            false
                        } {
                            Some(0u64)
                        } else {
                            let a1198 = v0.clone();
                            let a1199 = f13(Str::lit("Vec"));
                            if if equal(a1198, a1199) {
                                let a1200 = v1.clone();
                                let a1201 = f13(Str::lit("with_capacity"));
                                equal(a1200, a1201)
                            } else {
                                false
                            } {
                                Some(1u64)
                            } else {
                                let a1202 = v0.clone();
                                let a1203 = f13(Str::lit("String"));
                                if if equal(a1202, a1203) {
                                    let a1204 = v1.clone();
                                    let a1205 = f13(Str::lit("new"));
                                    equal(a1204, a1205)
                                } else {
                                    false
                                } {
                                    Some(0u64)
                                } else {
                                    let a1206 = v0.clone();
                                    let a1207 = f13(Str::lit("mem"));
                                    if if equal(a1206, a1207) {
                                        let a1208 = v1.clone();
                                        let a1209 = f13(Str::lit("replace"));
                                        equal(a1208, a1209)
                                    } else {
                                        false
                                    } {
                                        Some(2u64)
                                    } else {
                                        let a1210 = v0.clone();
                                        let a1211 = f13(Str::lit("str"));
                                        if if equal(a1210, a1211) {
                                            let a1212 = v1.clone();
                                            let a1213 = f13(Str::lit("from_utf8"));
                                            equal(a1212, a1213)
                                        } else {
                                            false
                                        } {
                                            Some(1u64)
                                        } else {
                                            let a1214 = v0.clone();
                                            let a1215 = f13(Str::lit("usize"));
                                            if if equal(a1214, a1215) {
                                                let a1216 = v1.clone();
                                                let a1217 = f13(Str::lit("from"));
                                                equal(a1216, a1217)
                                            } else {
                                                false
                                            } {
                                                Some(1u64)
                                            } else {
                                                let a1218 = v0;
                                                let a1219 = f13(Str::lit("usize"));
                                                if if equal(a1218, a1219) {
                                                    let a1220 = v1;
                                                    let a1221 = f13(Str::lit("try_from"));
                                                    equal(a1220, a1221)
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

pub fn f194(v0: Bytes, v1: Bytes) -> bool {
    let a1222 = v0.clone();
    let a1223 = f13(Str::lit("AtomicU64"));
    if if equal(a1222, a1223) {
        let a1224 = v1.clone();
        let a1225 = f13(Str::lit("new"));
        equal(a1224, a1225)
    } else {
        false
    } {
        false
    } else {
        let a1226 = v0.clone();
        let a1227 = f13(Str::lit("Rc"));
        if if equal(a1226, a1227) {
            let a1228 = v1.clone();
            let a1229 = f13(Str::lit("new"));
            equal(a1228, a1229)
        } else {
            false
        } {
            true
        } else {
            let a1230 = v0.clone();
            let a1231 = f13(Str::lit("Rc"));
            if if equal(a1230, a1231) {
                let a1232 = v1.clone();
                let a1233 = f13(Str::lit("from"));
                equal(a1232, a1233)
            } else {
                false
            } {
                true
            } else {
                let a1234 = v0.clone();
                let a1235 = f13(Str::lit("Rc"));
                if if equal(a1234, a1235) {
                    let a1236 = v1.clone();
                    let a1237 = f13(Str::lit("try_unwrap"));
                    equal(a1236, a1237)
                } else {
                    false
                } {
                    true
                } else {
                    let a1238 = v0.clone();
                    let a1239 = f13(Str::lit("Rc"));
                    if if equal(a1238, a1239) {
                        let a1240 = v1.clone();
                        let a1241 = f13(Str::lit("get_mut"));
                        equal(a1240, a1241)
                    } else {
                        false
                    } {
                        true
                    } else {
                        let a1242 = v0.clone();
                        let a1243 = f13(Str::lit("Vec"));
                        if if equal(a1242, a1243) {
                            let a1244 = v1.clone();
                            let a1245 = f13(Str::lit("new"));
                            equal(a1244, a1245)
                        } else {
                            false
                        } {
                            true
                        } else {
                            let a1246 = v0.clone();
                            let a1247 = f13(Str::lit("Vec"));
                            if if equal(a1246, a1247) {
                                let a1248 = v1.clone();
                                let a1249 = f13(Str::lit("with_capacity"));
                                equal(a1248, a1249)
                            } else {
                                false
                            } {
                                true
                            } else {
                                let a1250 = v0.clone();
                                let a1251 = f13(Str::lit("String"));
                                if if equal(a1250, a1251) {
                                    let a1252 = v1.clone();
                                    let a1253 = f13(Str::lit("new"));
                                    equal(a1252, a1253)
                                } else {
                                    false
                                } {
                                    true
                                } else {
                                    let a1254 = v0.clone();
                                    let a1255 = f13(Str::lit("mem"));
                                    if if equal(a1254, a1255) {
                                        let a1256 = v1.clone();
                                        let a1257 = f13(Str::lit("replace"));
                                        equal(a1256, a1257)
                                    } else {
                                        false
                                    } {
                                        false
                                    } else {
                                        let a1258 = v0.clone();
                                        let a1259 = f13(Str::lit("str"));
                                        if if equal(a1258, a1259) {
                                            let a1260 = v1.clone();
                                            let a1261 = f13(Str::lit("from_utf8"));
                                            equal(a1260, a1261)
                                        } else {
                                            false
                                        } {
                                            false
                                        } else {
                                            let a1262 = v0.clone();
                                            let a1263 = f13(Str::lit("usize"));
                                            if if equal(a1262, a1263) {
                                                let a1264 = v1.clone();
                                                let a1265 = f13(Str::lit("from"));
                                                equal(a1264, a1265)
                                            } else {
                                                false
                                            } {
                                                false
                                            } else {
                                                let a1266 = v0;
                                                let a1267 = f13(Str::lit("usize"));
                                                let _ = if equal(a1266, a1267) {
                                                    let a1268 = v1;
                                                    let a1269 = f13(Str::lit("try_from"));
                                                    equal(a1268, a1269)
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

pub fn f195(v0: Bytes, v1: Option<Adt13>, v2: Adt13) -> Option<Adt13> {
    let m1270 = v1;
    match m1270 {
        None => {
            if f25(match v2.clone() {
                Adt13::C0(h1271, _) => {
                    h1271
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

pub fn f196(v0: List<Adt9>, v1: Bytes) -> R<Option<u64>> {
    let m1272 = f150(v0, v1);
    match m1272 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            let m1273 = v2;
            match m1273 {
                Adt9::C0(_) => {
                    Ok(None::<u64>)
                }
                Adt9::C1(_, v5) => {
                    Ok(Some(f39(v5)?))
                }
                Adt9::C2(_, _, _) => {
                    Ok(None::<u64>)
                }
            }
        }
    }
}

pub fn f197(v0: Adt2, v1: u64) -> Adt2 {
    let m1274 = v0;
    match m1274 {
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
        Adt2::C9(r1279, r1280) => {
            let v5 = (*r1279).clone();
            let v6 = (*r1280).clone();
            let a1275 = v1;
            let a1276 = 0u64;
            if nat_eq(a1275, a1276) {
                v5
            } else {
                let a1277 = v1;
                let a1278 = 1u64;
                if nat_eq(a1277, a1278) {
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

pub fn f198(v0: Adt15, v1: Adt2) -> Adt2 {
    let m1281 = v1.clone();
    match m1281 {
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
                Adt15::C0(_, _, h1282, _) => {
                    h1282
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

pub fn f199(v0: Adt2) -> bool {
    let m1283 = v0;
    match m1283 {
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

pub fn f200(v0: Bytes, v1: Adt14, v2: bool, v3: Bytes) -> bool {
    if v2 {
        true
    } else {
        let m1284 = f125(v1, v3.clone());
        match m1284 {
            None => {
                let m1286 = f126(v3);
                match m1286 {
                    None => {
                        false
                    }
                    Some(v5) => {
                        f25(v5, v0)
                    }
                }
            }
            Some(v4) => {
                f186(Fn27::F207(v0), false, match v4 {
                    Adt10::C0(_, h1285) => {
                        h1285
                    }
                })
            }
        }
    }
}

pub fn f201(p1292: Adt3) -> bool {
    let m1291 = p1292;
    {
        let mut m1291 = m1291;
        loop {
            let step = {
                let v0 = m1291;
                let m1287 = v0;
                match m1287 {
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
                    Adt3::C21(v40, _, r1290) => {
                        let v42 = (*r1290).clone();
                        let a1288 = f171(v40);
                        let a1289 = f169(v42.clone());
                        if nat_lt(a1288, a1289) {
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
                Ok(next) => { m1291 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f202(v0: Fn29, p1296: u64, p1297: List<Bytes>) -> R<u64> {
    let m1295 = (p1296, p1297);
    {
        let mut m1295 = m1295;
        loop {
            let step = {
                let (v1, v2) = m1295;
                let m1293 = v2;
                match m1293.uncons() {
                    None => {
                        Err::<(u64, List<Bytes>), R<u64>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(u64, List<Bytes>), R<u64>>(({
                            let c1294: Fn29 = v0.clone();
                            c1294.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1295 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f203(v0: Bytes, v1: u64, v2: Bytes) -> R<u64> {
    if f25(v2, v0) {
        let a1298 = v1;
        let a1299 = 1u64;
        nat_add(a1298, a1299)
    } else {
        Ok(v1)
    }
}

pub fn f204(v0: Bytes, v1: u64) -> Option<u8> {
    let a1300 = v0;
    let a1301 = v1;
    index_bytes(a1300, a1301)
}

pub fn f205(v0: u8, v1: u8, v2: u8) -> bool {
    let m1304 = {
        let a1302 = v0;
        let a1303 = v1;
        fixed_u8::checked_sub(a1302, a1303)
    };
    match m1304 {
        None => {
            false
        }
        Some(_) => {
            let m1307 = {
                let a1305 = v2;
                let a1306 = v0;
                fixed_u8::checked_sub(a1305, a1306)
            };
            match m1307 {
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

pub fn f206(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f208(v0, v1, 48u8) {
        Ok(Some({
            let a1308 = v1;
            let a1309 = 1u64;
            nat_add(a1308, a1309)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f207(v0: Bytes, v1: bool, v2: Adt5) -> bool {
    if v1 {
        true
    } else {
        f25(match v2 {
            Adt5::C0(h1310, _, _, _, _) => {
                h1310
            }
        }, v0)
    }
}

pub fn f208(v0: Bytes, v1: u64, v2: u8) -> bool {
    let m1311 = f204(v0, v1);
    match m1311 {
        None => {
            false
        }
        Some(v3) => {
            let a1312 = v3;
            let a1313 = v2;
            equal(a1312, a1313)
        }
    }
}

pub fn run(p0: Adt0) -> R<Result<(), Bytes>> {
    f0(p0)
}
