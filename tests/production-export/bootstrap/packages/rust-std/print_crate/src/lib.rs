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
    pub fn update(mut a: Bytes, i: u64, byte: u8) -> Option<Bytes> { let at = match usize::try_from(i) { Ok(at) => at, Err(_) => return None, }; if at >= a.0.len() { return None; }; tick(1); match Rc::get_mut(&mut a.0) { Some(cells) => { let slot: &mut [u8] = cells; slot[at] = byte; Some(a) }, None => { tick(a.0.len() as u64); let mut out = Vec::with_capacity(a.0.len()); out.extend_from_slice(&a.0); out[at] = byte; Some(Bytes(Rc::from(out))) }, } }
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

pub fn update_bytes(p0: Bytes, p1: u64, p2: u8) -> Option<Bytes> {
    Bytes::update(p0, p1, p2)
}

#[derive(Clone)]
pub enum Adt0 {
    C0(bool, Adt1),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(Adt2, Adt8, Adt9),
    C1(u64, Adt2, Adt2, bool, Adt10, Adt9),
    C2(Adt12, Adt13, Adt2, Adt18, Adt9),
    C3,
    C4(std::rc::Rc<Adt1>, std::rc::Rc<Adt1>),
}

#[derive(Clone)]
pub enum Adt2 {
    C0(Adt3),
    C1(Adt4),
    C2(Adt5, std::rc::Rc<Adt2>),
    C3(Adt6, std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C4(Adt7, u64),
    C5,
    C6(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
}

#[derive(Clone)]
pub enum Adt3 {
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
pub enum Adt4 {
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
pub enum Adt5 {
    C0,
    C1,
    C2,
    C3,
    C4,
}

#[derive(Clone)]
pub enum Adt6 {
    C0,
    C1,
}

#[derive(Clone)]
pub enum Adt7 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt8 {
    C0,
    C1(u64, Adt2, std::rc::Rc<Adt8>),
}

#[derive(Clone)]
pub enum Adt9 {
    C0(Bytes, Option<Adt4>),
}

#[derive(Clone)]
pub enum Adt10 {
    C0,
    C1(u64, List<Adt11>, bool, std::rc::Rc<Adt10>),
}

#[derive(Clone)]
pub enum Adt11 {
    C0,
    C1,
    C2,
    C3,
}

#[derive(Clone)]
pub enum Adt12 {
    C0(u64),
    C1(u64),
    C2(u64),
    C3(u64),
    C4(u64),
    C5(u64),
    C6(u64),
    C7(u64),
    C8(u64),
    C9(Bytes),
}

#[derive(Clone)]
pub enum Adt13 {
    C0,
    C1(Adt14, Adt2, std::rc::Rc<Adt13>),
}

#[derive(Clone)]
pub enum Adt14 {
    C0(Adt15),
    C1(Adt12),
    C2(std::rc::Rc<Adt14>),
    C3(Adt16, std::rc::Rc<Adt14>),
    C4(u64, u64, std::rc::Rc<Adt14>),
    C5,
    C6(std::rc::Rc<Adt14>, std::rc::Rc<Adt14>),
    C7(Adt17),
}

#[derive(Clone)]
pub enum Adt15 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt16 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt17 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt18 {
    C0(Adt19, Adt9),
    C1(Adt20, Adt12, Adt9),
    C2(Adt21, std::rc::Rc<Adt18>, Adt9),
    C3(Adt22, std::rc::Rc<Adt18>, bool, Adt9),
    C4(Adt12, std::rc::Rc<Adt18>, bool, Adt9),
    C5(Adt24, std::rc::Rc<Adt18>, Adt9),
    C6(std::rc::Rc<Adt18>, std::rc::Rc<Adt18>, Adt9),
    C7(std::rc::Rc<Adt18>, std::rc::Rc<Adt18>, std::rc::Rc<Adt18>, Adt9),
    C8(std::rc::Rc<Adt18>, std::rc::Rc<Adt18>, Adt9),
    C9(std::rc::Rc<Adt18>),
    C10(Adt25, Adt12, Adt9),
    C11(std::rc::Rc<Adt18>, std::rc::Rc<Adt18>),
    C12,
    C13(std::rc::Rc<Adt18>, std::rc::Rc<Adt18>),
    C14(Adt14, bool, Adt2, std::rc::Rc<Adt18>, Adt9, std::rc::Rc<Adt18>),
    C15(Adt14, std::rc::Rc<Adt18>, std::rc::Rc<Adt18>),
    C16(Adt12, std::rc::Rc<Adt18>),
}

#[derive(Clone)]
pub enum Adt19 {
    C0,
    C1(bool),
    C2(u64),
    C3(bool, u64),
    C4(Adt4, bool, u64),
    C5(Bytes),
    C6(Bytes),
    C7(Adt17),
    C8,
}

#[derive(Clone)]
pub enum Adt20 {
    C0,
    C1,
    C2,
    C3,
    C4,
}

#[derive(Clone)]
pub enum Adt21 {
    C0,
    C1,
    C2,
    C3,
}

#[derive(Clone)]
pub enum Adt22 {
    C0(u64),
    C1(Adt23),
    C2(u64, Option<Adt4>),
}

#[derive(Clone)]
pub enum Adt23 {
    C0(u64, Option<Adt4>),
}

#[derive(Clone)]
pub enum Adt24 {
    C0(Adt2),
    C1,
    C2(Adt2, Adt2),
    C3(Adt2, Adt2),
    C4(u64, u64),
    C5(u64, u64),
    C6,
    C7(Adt2),
}

#[derive(Clone)]
pub enum Adt25 {
    C0,
    C1,
    C2,
    C3,
}

#[derive(Clone)]
pub enum Adt26 {
    C0(List<Bytes>, Bytes, u64, u64, bool),
}

#[derive(Clone)]
pub enum Fn0 {
    F12,
}

impl Fn0 {
    pub fn apply(&self, p0: List<Bytes>) -> Option<List<Bytes>> {
        match self {
            Fn0::F12 => f12(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F14,
}

impl Fn1 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn1::F14 => f14(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F38,
    F39,
}

impl Fn2 {
    pub fn apply(&self, p0: (u64, Adt26), p1: Adt11) -> R<(u64, Adt26)> {
        match self {
            Fn2::F38 => f38(p0, p1),
            Fn2::F39 => f39(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F41(u64, u64),
    F81(u64),
    F90(Bytes),
    F91(Bytes),
}

impl Fn3 {
    pub fn apply(&self, p0: (u64, Adt26)) -> R<Option<(u64, Adt26)>> {
        match self {
            Fn3::F41(k0, k1) => f41(*k0, *k1, p0),
            Fn3::F81(k0) => f81(*k0, p0),
            Fn3::F90(k0) => f90(k0.clone(), p0),
            Fn3::F91(k0) => f91(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F63(u64, Adt13, Adt2),
}

impl Fn4 {
    pub fn apply(&self, p0: (u64, (Adt26, bool))) -> R<Option<(u64, (Adt26, bool))>> {
        match self {
            Fn4::F63(k0, k1, k2) => f63(*k0, k1.clone(), k2.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F65,
}

impl Fn5 {
    pub fn apply(&self, p0: (Option<Bytes>, List<Bytes>), p1: Bytes) -> (Option<Bytes>, List<Bytes>) {
        match self {
            Fn5::F65 => f65(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F88(Bytes),
    F104(Bytes),
}

impl Fn6 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn6::F88(k0) => f88(k0.clone(), p0),
            Fn6::F104(k0) => f104(k0.clone(), p0),
        }
    }
}

pub fn f0(v0: u64, v1: Bytes, v2: Adt0) -> R<Result<Bytes, Bytes>> {
    Ok(f1(f2(f3(f4(v0), v1)?, match v2 {
        Adt0::C0(_, h1) => {
            h1
        }
    })?))
}

pub fn f1(v0: Adt26) -> Result<Bytes, Bytes> {
    if match v0.clone() {
        Adt26::C0(_, _, _, _, h2) => {
            h2
        }
    } {
        Err::<Bytes, Bytes>({
            let a3 = Str::lit("output exceeds the capacity of the buffer");
            utf8_encode(a3)
        })
    } else {
        Ok::<Bytes, Bytes>(f5(f6(List::cons(match v0.clone() {
            Adt26::C0(_, h4, _, _, _) => {
                h4
            }
        }, match v0 {
            Adt26::C0(h5, _, _, _, _) => {
                h5
            }
        }))))
    }
}

pub fn f2(p10: Adt26, p11: Adt1) -> R<Adt26> {
    let m9 = (p10, p11);
    {
        let mut m9 = m9;
        loop {
            let step = {
                let (v0, v1) = m9;
                let m6 = v1;
                match m6 {
                    Adt1::C0(_, _, _) => {
                        Err::<(Adt26, Adt1), R<Adt26>>(Ok(v0))
                    }
                    Adt1::C1(_, _, _, _, _, _) => {
                        Err::<(Adt26, Adt1), R<Adt26>>(Ok(v0))
                    }
                    Adt1::C2(_, _, _, _, _) => {
                        Err::<(Adt26, Adt1), R<Adt26>>(Ok(v0))
                    }
                    Adt1::C3 => {
                        Err::<(Adt26, Adt1), R<Adt26>>(Ok(v0))
                    }
                    Adt1::C4(r7, r8) => {
                        let v2 = (*r7).clone();
                        let v3 = (*r8).clone();
                        Ok::<(Adt26, Adt1), R<Adt26>>((f7(f8(v0, Str::lit("\u{a}"))?, v2)?, v3))
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

pub fn f3(v0: Adt26, v1: Bytes) -> R<Adt26> {
    if match v0.clone() {
        Adt26::C0(_, _, _, _, h12) => {
            h12
        }
    } {
        Ok(v0)
    } else {
        let v2: u64 = {
            let a15 = match v0.clone() {
                Adt26::C0(_, _, h13, _, _) => {
                    h13
                }
            };
            let a16 = {
                let a14 = v1.clone();
                length_bytes(a14)
            };
            nat_add(a15, a16)?
        };
        let a18 = v2;
        let a19 = match v0.clone() {
            Adt26::C0(_, _, _, h17, _) => {
                h17
            }
        };
        if nat_le(a18, a19) {
            Ok(f9(v0.clone(), {
                let a21 = match v0 {
                    Adt26::C0(_, h20, _, _, _) => {
                        h20
                    }
                };
                let a22 = v1;
                append_bytes(a21, a22)
            }, v2))
        } else {
            Ok(f10(v0))
        }
    }
}

pub fn f4(v0: u64) -> Adt26 {
    Adt26::C0(List::<Bytes>::nil(), Bytes::lit(&[]), 0u64, v0, false)
}

pub fn f5(v0: List<Bytes>) -> Bytes {
    let m25 = {
        let (h23, _) = f11(Fn0::F12, {
            let a24 = v0.clone();
            length_list(a24)
        }, v0);
        h23
    };
    match m25.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f6(v0: List<Bytes>) -> List<Bytes> {
    f13(Fn1::F14, List::<Bytes>::nil(), v0)
}

pub fn f7(v0: Adt26, v1: Adt1) -> R<Adt26> {
    let m26 = v1;
    match m26 {
        Adt1::C0(v2, v3, _) => {
            f8(f15(f8(f16(f8(v0, Str::lit("#[derive(Clone)]\u{a}pub enum "))?, v2.clone())?, Str::lit(" {\u{a}"))?, v3, f17(v2))?, Str::lit("}\u{a}"))
        }
        Adt1::C1(v5, v6, v7, v8, v9, _) => {
            let v11: bool = f18(v9.clone());
            let v12: Adt26 = f19(f8(v0, Str::lit("impl Fn"))?, v5)?;
            let v13: Adt26 = f8(f20(f8(f8(v12, Str::lit(" {\u{a}"))?, Str::lit("    pub fn apply(&self, "))?, v6.clone(), 0u64, v11)?, Str::lit(")"))?;
            let v14: Adt26 = if v8 {
                f8(f16(f8(v13, Str::lit(" -> R<"))?, v7.clone())?, Str::lit(">"))?
            } else {
                f21(v13, v7.clone())?
            };
            let v15: Adt26 = f8(v14, Str::lit(" {\u{a}"))?;
            let v16: Adt26 = if v11 {
                f8(v15, Str::lit("        match *self {}\u{a}"))?
            } else {
                f8(f22(f8(v15, Str::lit("        match self {\u{a}"))?, v9, v5, f23(v6)?, f24(v7), v8)?, Str::lit("        }\u{a}"))?
            };
            f8(f8(v16, Str::lit("    }\u{a}"))?, Str::lit("}\u{a}"))
        }
        Adt1::C2(v17, v18, v19, v20, _) => {
            f8(f25(f8(f21(f8(f26(f8(f27(f28(f8(v0, Str::lit("pub fn "))?, v17)?, v18.clone(), v19.clone())?, Str::lit("("))?, v18, true)?, Str::lit(")"))?, v19)?, Str::lit(" "))?, v20, 0u64, 100u64, 0u64)?, Str::lit("\u{a}"))
        }
        Adt1::C3 => {
            Ok(v0)
        }
        Adt1::C4(_, _) => {
            Ok(v0)
        }
    }
}

pub fn f8(v0: Adt26, v1: Str) -> R<Adt26> {
    f3(v0, {
        let a27 = v1;
        utf8_encode(a27)
    })
}

pub fn f9(v0: Adt26, v1: Bytes, v2: u64) -> Adt26 {
    let a29 = 256u64;
    let a30 = {
        let a28 = v1.clone();
        length_bytes(a28)
    };
    if nat_le(a29, a30) {
        Adt26::C0(List::cons(v1, match v0.clone() {
            Adt26::C0(h31, _, _, _, _) => {
                h31
            }
        }), Bytes::lit(&[]), v2, match v0 {
            Adt26::C0(_, _, _, h32, _) => {
                h32
            }
        }, false)
    } else {
        Adt26::C0(match v0.clone() {
            Adt26::C0(h33, _, _, _, _) => {
                h33
            }
        }, v1, v2, match v0 {
            Adt26::C0(_, _, _, h34, _) => {
                h34
            }
        }, false)
    }
}

pub fn f10(v0: Adt26) -> Adt26 {
    Adt26::C0(match v0.clone() {
        Adt26::C0(h35, _, _, _, _) => {
            h35
        }
    }, match v0.clone() {
        Adt26::C0(_, h36, _, _, _) => {
            h36
        }
    }, match v0.clone() {
        Adt26::C0(_, _, h37, _, _) => {
            h37
        }
    }, match v0 {
        Adt26::C0(_, _, _, h38, _) => {
            h38
        }
    }, true)
}

pub fn f11(v0: Fn0, p43: u64, p44: List<Bytes>) -> (List<Bytes>, bool) {
    let m42 = (p43, p44);
    {
        let mut m42 = m42;
        loop {
            let step = {
                let (v1, v2) = m42;
                let m39 = v1;
                if m39 == 0 {
                    Err::<(u64, List<Bytes>), (List<Bytes>, bool)>((v2, false))
                } else {
                    let v3 = m39 - 1;
                    let m41 = {
                        let c40: Fn0 = v0.clone();
                        c40.apply(v2.clone())
                    };
                    match m41 {
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
                Ok(next) => { m42 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f12(v0: List<Bytes>) -> Option<List<Bytes>> {
    f29(v0)
}

pub fn f13(v0: Fn1, p48: List<Bytes>, p49: List<Bytes>) -> List<Bytes> {
    let m47 = (p48, p49);
    {
        let mut m47 = m47;
        loop {
            let step = {
                let (v1, v2) = m47;
                let m45 = v2;
                match m45.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c46: Fn1 = v0.clone();
                            c46.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m47 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f14(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f15(p53: Adt26, p54: Adt8, v2: bool) -> R<Adt26> {
    let m52 = (p53, p54);
    {
        let mut m52 = m52;
        loop {
            let step = {
                let (v0, v1) = m52;
                let m50 = v1;
                match m50 {
                    Adt8::C0 => {
                        Err::<(Adt26, Adt8), R<Adt26>>(Ok(v0))
                    }
                    Adt8::C1(v3, v4, r51) => {
                        let v5 = (*r51).clone();
                        let v6: Adt26 = f19(f8(f8(v0, Str::lit("    "))?, if v2 {
                            Str::lit("F")
                        } else {
                            Str::lit("C")
                        })?, v3)?;
                        let v7: Adt26 = if f30(v4.clone()) {
                            f8(f16(f8(v6, Str::lit("("))?, v4)?, Str::lit(")"))?
                        } else {
                            v6
                        };
                        Ok::<(Adt26, Adt8), R<Adt26>>((f8(v7, Str::lit(",\u{a}"))?, v5))
                    }
                }
            };
            match step {
                Ok(next) => { m52 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f16(v0: Adt26, v1: Adt2) -> R<Adt26> {
    let m55 = v1;
    match m55 {
        Adt2::C0(v2) => {
            f31(v0, v2)
        }
        Adt2::C1(v3) => {
            f32(v0, v3)
        }
        Adt2::C2(v4, r59) => {
            let v5 = (*r59).clone();
            f8(f16(f8(v0, f33(v4.clone()))?, v5)?, f34(v4))
        }
        Adt2::C3(v6, r60, r61) => {
            let v7 = (*r60).clone();
            let v8 = (*r61).clone();
            let m56 = v6;
            match m56 {
                Adt6::C0 => {
                    f8(f16(f8(f16(f8(v0, Str::lit("Result<"))?, v7)?, Str::lit(", "))?, v8)?, Str::lit(">"))
                }
                Adt6::C1 => {
                    f8(f16(f8(f16(f8(v0, Str::lit("("))?, v7)?, Str::lit(", "))?, v8)?, Str::lit(")"))
                }
            }
        }
        Adt2::C4(v9, v10) => {
            let m57 = v9;
            match m57 {
                Adt7::C0 => {
                    f19(f8(v0, Str::lit("Adt"))?, v10)
                }
                Adt7::C1 => {
                    f19(f8(v0, Str::lit("Fn"))?, v10)
                }
                Adt7::C2 => {
                    f35(v0, v10)
                }
            }
        }
        Adt2::C5 => {
            Ok(v0)
        }
        Adt2::C6(r62, r63) => {
            let v11 = (*r62).clone();
            let v12 = (*r63).clone();
            let m58 = v12.clone();
            match m58 {
                Adt2::C0(_) => {
                    f16(v0, v11)
                }
                Adt2::C1(_) => {
                    f16(v0, v11)
                }
                Adt2::C2(_, _) => {
                    f16(v0, v11)
                }
                Adt2::C3(_, _, _) => {
                    f16(v0, v11)
                }
                Adt2::C4(_, _) => {
                    f16(v0, v11)
                }
                Adt2::C5 => {
                    f16(v0, v11)
                }
                Adt2::C6(_, _) => {
                    f16(f8(f16(v0, v11)?, Str::lit(", "))?, v12)
                }
            }
        }
    }
}

pub fn f17(v0: Adt2) -> bool {
    let m64 = v0;
    match m64 {
        Adt2::C0(_) => {
            false
        }
        Adt2::C1(_) => {
            false
        }
        Adt2::C2(_, _) => {
            false
        }
        Adt2::C3(_, _, _) => {
            false
        }
        Adt2::C4(v1, _) => {
            let m65 = v1;
            match m65 {
                Adt7::C0 => {
                    false
                }
                Adt7::C1 => {
                    true
                }
                Adt7::C2 => {
                    false
                }
            }
        }
        Adt2::C5 => {
            false
        }
        Adt2::C6(_, _) => {
            false
        }
    }
}

pub fn f18(v0: Adt10) -> bool {
    let m66 = v0;
    match m66 {
        Adt10::C0 => {
            true
        }
        Adt10::C1(_, _, _, _) => {
            false
        }
    }
}

pub fn f19(v0: Adt26, v1: u64) -> R<Adt26> {
    f36(v0, v1)
}

pub fn f20(p75: Adt26, p76: Adt2, p77: u64, v3: bool) -> R<Adt26> {
    let m74 = (p75, (p76, p77));
    {
        let mut m74 = m74;
        loop {
            let step = {
                let (v0, (v1, v2)) = m74;
                let m67 = v1;
                match m67 {
                    Adt2::C0(_) => {
                        Err::<(Adt26, (Adt2, u64)), R<Adt26>>(Ok(v0))
                    }
                    Adt2::C1(_) => {
                        Err::<(Adt26, (Adt2, u64)), R<Adt26>>(Ok(v0))
                    }
                    Adt2::C2(_, _) => {
                        Err::<(Adt26, (Adt2, u64)), R<Adt26>>(Ok(v0))
                    }
                    Adt2::C3(_, _, _) => {
                        Err::<(Adt26, (Adt2, u64)), R<Adt26>>(Ok(v0))
                    }
                    Adt2::C4(_, _) => {
                        Err::<(Adt26, (Adt2, u64)), R<Adt26>>(Ok(v0))
                    }
                    Adt2::C5 => {
                        Err::<(Adt26, (Adt2, u64)), R<Adt26>>(Ok(v0))
                    }
                    Adt2::C6(r72, r73) => {
                        let v4 = (*r72).clone();
                        let v5 = (*r73).clone();
                        let v6: Adt26 = {
                            let a68 = v2;
                            let a69 = 0u64;
                            if nat_eq(a68, a69) {
                                v0
                            } else {
                                f8(v0, Str::lit(", "))?
                            }
                        };
                        let v7: Adt26 = f19(f8(if v3 {
                            f8(v6, Str::lit("_"))?
                        } else {
                            v6
                        }, Str::lit("p"))?, v2)?;
                        Ok::<(Adt26, (Adt2, u64)), R<Adt26>>((f16(f8(v7, Str::lit(": "))?, v4)?, (v5, {
                            let a70 = v2;
                            let a71 = 1u64;
                            nat_add(a70, a71)?
                        })))
                    }
                }
            };
            match step {
                Ok(next) => { m74 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f21(v0: Adt26, v1: Adt2) -> R<Adt26> {
    if f24(v1.clone()) {
        Ok(v0)
    } else {
        f16(f8(v0, Str::lit(" -> "))?, v1)
    }
}

pub fn f22(p89: Adt26, p90: Adt10, v2: u64, v3: u64, v4: bool, v5: bool) -> R<Adt26> {
    let m88 = (p89, p90);
    {
        let mut m88 = m88;
        loop {
            let step = {
                let (v0, v1) = m88;
                let m78 = v1;
                match m78 {
                    Adt10::C0 => {
                        Err::<(Adt26, Adt10), R<Adt26>>(Ok(v0))
                    }
                    Adt10::C1(v6, v7, v8, r87) => {
                        let v9 = (*r87).clone();
                        let v10: u64 = {
                            let a79 = v7.clone();
                            length_list(a79)
                        };
                        let v11: Adt26 = f19(f8(f19(f8(v0, Str::lit("            Fn"))?, v2)?, Str::lit("::F"))?, v6)?;
                        let v12: Adt26 = {
                            let a80 = v10;
                            let a81 = 0u64;
                            if nat_eq(a80, a81) {
                                v11
                            } else {
                                f8({
                                    let (_, h82) = f37(Fn2::F38, (0u64, f8(v11, Str::lit("("))?), v7.clone())?;
                                    h82
                                }, Str::lit(")"))?
                            }
                        };
                        let v13: bool = if v5 {
                            let a83 = v8;
                            bool_not(a83)
                        } else {
                            false
                        };
                        let v14: Adt26 = f8(v12, Str::lit(" => "))?;
                        let v15: Adt26 = if v13 {
                            if v4 {
                                f8(v14, Str::lit("{ let () = "))?
                            } else {
                                f8(v14, Str::lit("Ok("))?
                            }
                        } else {
                            v14
                        };
                        let v16: Adt26 = f8(f19(f8(v15, Str::lit("f"))?, v6)?, Str::lit("("))?;
                        let v17: Adt26 = {
                            let (_, h84) = f37(Fn2::F39, (0u64, v16), v7)?;
                            h84
                        };
                        let v18: Adt26 = {
                            let (_, h85) = {
                                let (h86, _) = f40(Fn3::F41(v10, v3), v3, (0u64, v17))?;
                                h86
                            };
                            h85
                        };
                        let v19: Adt26 = f8(v18, Str::lit(")"))?;
                        let v20: Adt26 = if v13 {
                            if v4 {
                                f8(v19, Str::lit("; Ok(()) }"))?
                            } else {
                                f8(v19, Str::lit(")"))?
                            }
                        } else {
                            v19
                        };
                        Ok::<(Adt26, Adt10), R<Adt26>>((f8(v20, Str::lit(",\u{a}"))?, v9))
                    }
                }
            };
            match step {
                Ok(next) => { m88 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f23(v0: Adt2) -> R<u64> {
    let m91 = v0;
    match m91 {
        Adt2::C0(_) => {
            Ok(0u64)
        }
        Adt2::C1(_) => {
            Ok(0u64)
        }
        Adt2::C2(_, _) => {
            Ok(0u64)
        }
        Adt2::C3(_, _, _) => {
            Ok(0u64)
        }
        Adt2::C4(_, _) => {
            Ok(0u64)
        }
        Adt2::C5 => {
            Ok(0u64)
        }
        Adt2::C6(_, r94) => {
            let v11 = (*r94).clone();
            let v12: u64 = f23(v11)?;
            let a92 = 1u64;
            let a93 = v12;
            nat_add(a92, a93)
        }
    }
}

pub fn f24(v0: Adt2) -> bool {
    let m95 = v0;
    match m95 {
        Adt2::C0(v1) => {
            let m96 = v1;
            match m96 {
                Adt3::C0 => {
                    true
                }
                Adt3::C1 => {
                    false
                }
                Adt3::C2 => {
                    false
                }
                Adt3::C3 => {
                    false
                }
                Adt3::C4 => {
                    false
                }
                Adt3::C5 => {
                    false
                }
                Adt3::C6 => {
                    false
                }
                Adt3::C7 => {
                    false
                }
                Adt3::C8 => {
                    false
                }
            }
        }
        Adt2::C1(_) => {
            false
        }
        Adt2::C2(_, _) => {
            false
        }
        Adt2::C3(_, _, _) => {
            false
        }
        Adt2::C4(_, _) => {
            false
        }
        Adt2::C5 => {
            false
        }
        Adt2::C6(_, _) => {
            false
        }
    }
}

pub fn f25(v0: Adt26, v1: Adt18, v2: u64, v3: u64, v4: u64) -> R<Adt26> {
    let m97 = v1;
    match m97 {
        Adt18::C0(v5, _) => {
            f42(v0, v5)
        }
        Adt18::C1(v7, v8, _) => {
            let m98 = v7;
            match m98 {
                Adt20::C0 => {
                    f28(v0, v8)
                }
                Adt20::C1 => {
                    f8(f28(v0, v8)?, Str::lit(".clone()"))
                }
                Adt20::C2 => {
                    f28(v0, v8)
                }
                Adt20::C3 => {
                    f28(f8(v0, Str::lit("*"))?, v8)
                }
                Adt20::C4 => {
                    f8(f28(f8(v0, Str::lit("(*"))?, v8)?, Str::lit(").clone()"))
                }
            }
        }
        Adt18::C2(v10, r155, _) => {
            let v11 = (*r155).clone();
            let m99 = v10;
            match m99 {
                Adt21::C0 => {
                    let a100 = f43(v11.clone());
                    let a101 = 0u64;
                    if if nat_eq(a100, a101) {
                        true
                    } else {
                        let a102 = f43(v11.clone());
                        let a103 = 1u64;
                        if nat_eq(a102, a103) {
                            true
                        } else {
                            let a104 = f43(v11.clone());
                            let a105 = 3u64;
                            nat_eq(a104, a105)
                        }
                    } {
                        f8(f25(f8(v0, Str::lit("!("))?, v11, v2, 100u64, 0u64)?, Str::lit(")"))
                    } else {
                        f25(f8(v0, Str::lit("!"))?, v11, v2, 100u64, 0u64)
                    }
                }
                Adt21::C1 => {
                    f8(f25(f8(v0, Str::lit("std::rc::Rc::new("))?, v11, v2, 100u64, 0u64)?, Str::lit(")"))
                }
                Adt21::C2 => {
                    f8(f25(f8(v0, Str::lit("i128::from("))?, v11, v2, 100u64, 0u64)?, Str::lit(")"))
                }
                Adt21::C3 => {
                    f8(f25(f8(v0, Str::lit("Ok("))?, v11, v2, 100u64, 0u64)?, Str::lit(")"))
                }
            }
        }
        Adt18::C3(v13, r156, v15, _) => {
            let v14 = (*r156).clone();
            let v17: u64 = f44(v13.clone());
            let v22: Adt26 = {
                let m106 = v13.clone();
                match m106 {
                    Adt22::C0(v18) => {
                        f8(f19(f8(v0, Str::lit("f"))?, v18)?, Str::lit("("))?
                    }
                    Adt22::C1(v19) => {
                        f8(f45(v0, v19)?, Str::lit("("))?
                    }
                    Adt22::C2(v20, v21) => {
                        f46(v0, v20, v21)?
                    }
                }
            };
            let v23: Adt26 = f25(v22, v14, v2, v17, 0u64)?;
            let v28: Adt26 = {
                let m107 = v13;
                match m107 {
                    Adt22::C0(_) => {
                        f8(v23, Str::lit(")"))?
                    }
                    Adt22::C1(_) => {
                        f8(v23, Str::lit(")"))?
                    }
                    Adt22::C2(v26, _) => {
                        f47(v23, v26)?
                    }
                }
            };
            if v15 {
                f8(v28, Str::lit("?"))
            } else {
                Ok(v28)
            }
        }
        Adt18::C4(v29, r157, v31, _) => {
            let v30 = (*r157).clone();
            let v33: Adt26 = f8(f25(f8(f28(v0, v29)?, Str::lit(".apply("))?, v30, v2, 100u64, 0u64)?, Str::lit(")"))?;
            if v31 {
                f8(v33, Str::lit("?"))
            } else {
                Ok(v33)
            }
        }
        Adt18::C5(v34, r158, _) => {
            let v35 = (*r158).clone();
            let v37: Adt26 = f48(v0, v34.clone())?;
            let a108 = f49(v34.clone());
            let a109 = 1u64;
            if nat_eq(a108, a109) {
                f8(f25(v37, v35, v2, 100u64, 0u64)?, Str::lit(")"))
            } else {
                let a110 = f49(v34);
                let a111 = 2u64;
                if nat_eq(a110, a111) {
                    if f50(v35.clone()) {
                        f8(f25(f8(v37, Str::lit("("))?, v35, v2, 100u64, 0u64)?, Str::lit(")"))
                    } else {
                        Ok(v37)
                    }
                } else {
                    Ok(v37)
                }
            }
        }
        Adt18::C6(r159, r160, _) => {
            let v38 = (*r159).clone();
            let v39 = (*r160).clone();
            f8(f25(f8(f25(f8(v0, Str::lit("("))?, v38, v2, 100u64, 0u64)?, Str::lit(", "))?, v39, v2, 100u64, 0u64)?, Str::lit(")"))
        }
        Adt18::C7(r161, r162, r163, _) => {
            let v41 = (*r161).clone();
            let v42 = (*r162).clone();
            let v43 = (*r163).clone();
            let v45: Adt26 = f25(f8(f25(f8(v0, Str::lit("if "))?, v41, v2, 100u64, 0u64)?, Str::lit(" "))?, v42, v2, 100u64, 0u64)?;
            if f51(v43.clone()) {
                Ok(v45)
            } else {
                f25(f8(v45, Str::lit(" else "))?, v43, v2, 100u64, 0u64)
            }
        }
        Adt18::C8(r164, r165, _) => {
            let v46 = (*r164).clone();
            let v47 = (*r165).clone();
            let a112 = f52(v47.clone());
            let a113 = 15u64;
            if nat_eq(a112, a113) {
                if f53(v46.clone()) {
                    let v49: Adt26 = f25(f8(f25(f8(v0, Str::lit("{ let value = "))?, v46, v2, 100u64, 0u64)?, Str::lit("; match value {\u{a}"))?, v47, v2, 100u64, 0u64)?;
                    f8(f54(v49, v2)?, Str::lit("} }"))
                } else {
                    let v50: Adt26 = f25(f8(f25(f8(v0, Str::lit("match "))?, v46, v2, 100u64, 0u64)?, Str::lit(" {\u{a}"))?, v47, v2, 100u64, 0u64)?;
                    f8(f54(v50, v2)?, Str::lit("}"))
                }
            } else {
                f8(f25(f8(v0, Str::lit("match "))?, v46, v2, 100u64, 0u64)?, Str::lit(" {}"))
            }
        }
        Adt18::C9(r166) => {
            let v51 = (*r166).clone();
            f25(v0, v51, v2, v3, v4)
        }
        Adt18::C10(v52, v53, _) => {
            let m114 = v52;
            match m114 {
                Adt25::C0 => {
                    f8(f28(v0, v53)?, Str::lit(" == 0"))
                }
                Adt25::C1 => {
                    f8(f28(v0, v53)?, Str::lit(" != 0"))
                }
                Adt25::C2 => {
                    f8(f28(v0, v53)?, Str::lit(".uncons()"))
                }
                Adt25::C3 => {
                    f8(f28(v0, v53)?, Str::lit(" - 1"))
                }
            }
        }
        Adt18::C11(r167, r168) => {
            let v55 = (*r167).clone();
            let v56 = (*r168).clone();
            let a115 = f52(v55.clone());
            let a116 = 12u64;
            if if nat_eq(a115, a116) {
                f55(v56.clone())
            } else {
                false
            } {
                f8(v0, Str::lit("{}"))
            } else {
                let v57: Adt26 = f25(f8(v0, Str::lit("{\u{a}"))?, v55, v2, 100u64, 0u64)?;
                let v58: Adt26 = if f55(v56.clone()) {
                    v57
                } else {
                    f8(f25(f54(v57, {
                        let a117 = v2;
                        let a118 = 1u64;
                        nat_add(a117, a118)?
                    })?, v56, {
                        let a119 = v2;
                        let a120 = 1u64;
                        nat_add(a119, a120)?
                    }, 100u64, 0u64)?, Str::lit("\u{a}"))?
                };
                f8(f54(v58, v2)?, Str::lit("}"))
            }
        }
        Adt18::C12 => {
            Ok(v0)
        }
        Adt18::C13(r169, r170) => {
            let v59 = (*r169).clone();
            let v60 = (*r170).clone();
            let a121 = v4;
            let a122 = f56(v3);
            if nat_lt(a121, a122) {
                let v61: Adt26 = f25(v0, v59, v2, 100u64, 0u64)?;
                if f50(v60.clone()) {
                    let a125 = {
                        let a123 = v4;
                        let a124 = 1u64;
                        nat_add(a123, a124)?
                    };
                    let a126 = f56(v3);
                    if nat_lt(a125, a126) {
                        f25(f57(v61, v3, v4)?, v60, v2, v3, {
                            let a127 = v4;
                            let a128 = 1u64;
                            nat_add(a127, a128)?
                        })
                    } else {
                        Ok(v61)
                    }
                } else {
                    Ok(v61)
                }
            } else {
                Ok(v0)
            }
        }
        Adt18::C14(v62, v63, v64, r171, _, r172) => {
            let v65 = (*r171).clone();
            let v67 = (*r172).clone();
            let v68: Adt26 = f58(f8(f54(v0, {
                let a129 = v2;
                let a130 = 1u64;
                nat_add(a129, a130)?
            })?, Str::lit("let "))?, v62)?;
            let v69: Adt26 = if v63 {
                f16(f8(v68, Str::lit(": "))?, v64)?
            } else {
                v68
            };
            let v70: Adt26 = f8(f25(f8(v69, Str::lit(" = "))?, v65, {
                let a131 = v2;
                let a132 = 1u64;
                nat_add(a131, a132)?
            }, 100u64, 0u64)?, Str::lit(";\u{a}"))?;
            f25(v70, v67, v2, 100u64, 0u64)
        }
        Adt18::C15(v71, r173, r174) => {
            let v72 = (*r173).clone();
            let v73 = (*r174).clone();
            let v74: Adt26 = f8(f25(f8(f58(f54(v0, {
                let a133 = v2;
                let a134 = 1u64;
                nat_add(a133, a134)?
            })?, v71)?, Str::lit(" => "))?, v72, {
                let a135 = v2;
                let a136 = 1u64;
                nat_add(a135, a136)?
            }, 100u64, 0u64)?, Str::lit("\u{a}"))?;
            f25(v74, v73, v2, 100u64, 0u64)
        }
        Adt18::C16(v75, r175) => {
            let v76 = (*r175).clone();
            let v77: Adt26 = f28(f8(f54(f8(v0, Str::lit("{\u{a}"))?, {
                let a137 = v2;
                let a138 = 1u64;
                nat_add(a137, a138)?
            })?, Str::lit("let mut "))?, v75.clone())?;
            let v78: Adt26 = f8(f28(f8(v77, Str::lit(" = "))?, v75.clone())?, Str::lit(";\u{a}"))?;
            let v79: Adt26 = f8(f54(v78, {
                let a139 = v2;
                let a140 = 1u64;
                nat_add(a139, a140)?
            })?, Str::lit("loop {\u{a}"))?;
            let v80: Adt26 = f8(f25(f8(f54(v79, {
                let a141 = v2;
                let a142 = 2u64;
                nat_add(a141, a142)?
            })?, Str::lit("let step = "))?, v76, {
                let a143 = v2;
                let a144 = 2u64;
                nat_add(a143, a144)?
            }, 100u64, 0u64)?, Str::lit(";\u{a}"))?;
            let v81: Adt26 = f8(f54(v80, {
                let a145 = v2;
                let a146 = 2u64;
                nat_add(a145, a146)?
            })?, Str::lit("match step {\u{a}"))?;
            let v82: Adt26 = f8(f28(f8(f54(v81, {
                let a147 = v2;
                let a148 = 3u64;
                nat_add(a147, a148)?
            })?, Str::lit("Ok(next) => { "))?, v75)?, Str::lit(" = next; }\u{a}"))?;
            let v83: Adt26 = f8(f54(v82, {
                let a149 = v2;
                let a150 = 3u64;
                nat_add(a149, a150)?
            })?, Str::lit("Err(done) => break done,\u{a}"))?;
            f8(f54(f8(f54(f8(f54(v83, {
                let a151 = v2;
                let a152 = 2u64;
                nat_add(a151, a152)?
            })?, Str::lit("}\u{a}"))?, {
                let a153 = v2;
                let a154 = 1u64;
                nat_add(a153, a154)?
            })?, Str::lit("}\u{a}"))?, v2)?, Str::lit("}"))
        }
    }
}

pub fn f26(p179: Adt26, p180: Adt13, p181: bool) -> R<Adt26> {
    let m178 = (p179, (p180, p181));
    {
        let mut m178 = m178;
        loop {
            let step = {
                let (v0, (v1, v2)) = m178;
                let m176 = v1;
                match m176 {
                    Adt13::C0 => {
                        Err::<(Adt26, (Adt13, bool)), R<Adt26>>(Ok(v0))
                    }
                    Adt13::C1(v3, v4, r177) => {
                        let v5 = (*r177).clone();
                        Ok::<(Adt26, (Adt13, bool)), R<Adt26>>((f16(f8(f58(if v2 {
                            v0
                        } else {
                            f8(v0, Str::lit(", "))?
                        }, v3)?, Str::lit(": "))?, v4)?, (v5, false)))
                    }
                }
            };
            match step {
                Ok(next) => { m178 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f27(v0: Adt26, v1: Adt13, v2: Adt2) -> R<Adt26> {
    let v3: u64 = f59(f60(v1.clone())?, f61(v2.clone())?);
    let v4: ((u64, (Adt26, bool)), bool) = f62(Fn4::F63(v3, v1, v2), v3, (0u64, (v0, false)))?;
    let v5: (Adt26, bool) = {
        let (_, h182) = {
            let (h183, _) = v4;
            h183
        };
        h182
    };
    let (_, h184) = v5;
    if h184 {
        f8({
            let (h185, _) = v5;
            h185
        }, Str::lit(">"))
    } else {
        let (h186, _) = v5;
        Ok(h186)
    }
}

pub fn f28(v0: Adt26, v1: Adt12) -> R<Adt26> {
    let m187 = v1;
    match m187 {
        Adt12::C0(v2) => {
            f19(f8(v0, Str::lit("v"))?, v2)
        }
        Adt12::C1(v3) => {
            f19(f8(v0, Str::lit("m"))?, v3)
        }
        Adt12::C2(v4) => {
            f19(f8(v0, Str::lit("a"))?, v4)
        }
        Adt12::C3(v5) => {
            f19(f8(v0, Str::lit("c"))?, v5)
        }
        Adt12::C4(v6) => {
            f19(f8(v0, Str::lit("r"))?, v6)
        }
        Adt12::C5(v7) => {
            f19(f8(v0, Str::lit("h"))?, v7)
        }
        Adt12::C6(v8) => {
            f19(f8(v0, Str::lit("k"))?, v8)
        }
        Adt12::C7(v9) => {
            f19(f8(v0, Str::lit("p"))?, v9)
        }
        Adt12::C8(v10) => {
            f19(f8(v0, Str::lit("f"))?, v10)
        }
        Adt12::C9(v11) => {
            f3(v0, v11)
        }
    }
}

pub fn f29(v0: List<Bytes>) -> Option<List<Bytes>> {
    let m188 = v0.clone();
    match m188.uncons() {
        None => {
            None::<List<Bytes>>
        }
        Some((_, v2)) => {
            let m189 = v2;
            match m189.uncons() {
                None => {
                    None::<List<Bytes>>
                }
                Some((_, _)) => {
                    let v5: (Option<Bytes>, List<Bytes>) = f64(Fn5::F65, (None::<Bytes>, List::<Bytes>::nil()), v0);
                    let m191 = {
                        let (h190, _) = v5;
                        h190
                    };
                    match m191 {
                        None => {
                            Some(f6({
                                let (_, h193) = v5;
                                h193
                            }))
                        }
                        Some(v6) => {
                            Some(f6(List::cons(v6, {
                                let (_, h192) = v5;
                                h192
                            })))
                        }
                    }
                }
            }
        }
    }
}

pub fn f30(v0: Adt2) -> bool {
    let m194 = v0;
    match m194 {
        Adt2::C0(_) => {
            false
        }
        Adt2::C1(_) => {
            false
        }
        Adt2::C2(_, _) => {
            false
        }
        Adt2::C3(_, _, _) => {
            false
        }
        Adt2::C4(_, _) => {
            false
        }
        Adt2::C5 => {
            false
        }
        Adt2::C6(_, _) => {
            true
        }
    }
}

pub fn f31(v0: Adt26, v1: Adt3) -> R<Adt26> {
    f8(v0, {
        let m195 = v1;
        match m195 {
            Adt3::C0 => {
                Str::lit("()")
            }
            Adt3::C1 => {
                Str::lit("bool")
            }
            Adt3::C2 => {
                Str::lit("u64")
            }
            Adt3::C3 => {
                Str::lit("i64")
            }
            Adt3::C4 => {
                Str::lit("Ordering")
            }
            Adt3::C5 => {
                Str::lit("Str")
            }
            Adt3::C6 => {
                Str::lit("Bytes")
            }
            Adt3::C7 => {
                Str::lit("Overflow")
            }
            Adt3::C8 => {
                Str::lit("i128")
            }
        }
    })
}

pub fn f32(v0: Adt26, v1: Adt4) -> R<Adt26> {
    f8(v0, f66(v1))
}

pub fn f33(v0: Adt5) -> Str {
    let m196 = v0;
    match m196 {
        Adt5::C0 => {
            Str::lit("Option<")
        }
        Adt5::C1 => {
            Str::lit("List<")
        }
        Adt5::C2 => {
            Str::lit("std::rc::Rc<")
        }
        Adt5::C3 => {
            Str::lit("R<")
        }
        Adt5::C4 => {
            Str::lit("&")
        }
    }
}

pub fn f34(v0: Adt5) -> Str {
    let m197 = v0;
    match m197 {
        Adt5::C0 => {
            Str::lit(">")
        }
        Adt5::C1 => {
            Str::lit(">")
        }
        Adt5::C2 => {
            Str::lit(">")
        }
        Adt5::C3 => {
            Str::lit(">")
        }
        Adt5::C4 => {
            Str::lit("")
        }
    }
}

pub fn f35(v0: Adt26, v1: u64) -> R<Adt26> {
    let a198 = v1;
    let a199 = 0u64;
    if nat_eq(a198, a199) {
        f8(v0, Str::lit("T"))
    } else {
        let a200 = v1;
        let a201 = 1u64;
        if nat_eq(a200, a201) {
            f8(v0, Str::lit("U"))
        } else {
            let a202 = v1;
            let a203 = 2u64;
            if nat_eq(a202, a203) {
                f8(v0, Str::lit("V"))
            } else {
                f19(f8(v0, Str::lit("T"))?, v1)
            }
        }
    }
}

pub fn f36(v0: Adt26, v1: u64) -> R<Adt26> {
    f3(v0, f67(v1)?)
}

pub fn f37(v0: Fn2, p207: (u64, Adt26), p208: List<Adt11>) -> R<(u64, Adt26)> {
    let m206 = (p207, p208);
    {
        let mut m206 = m206;
        loop {
            let step = {
                let (v1, v2) = m206;
                let m204 = v2;
                match m204.uncons() {
                    None => {
                        Err::<((u64, Adt26), List<Adt11>), R<(u64, Adt26)>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<((u64, Adt26), List<Adt11>), R<(u64, Adt26)>>(({
                            let c205: Fn2 = v0.clone();
                            c205.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m206 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f38(v0: (u64, Adt26), v1: Adt11) -> R<(u64, Adt26)> {
    f68(v0, v1)
}

pub fn f39(v0: (u64, Adt26), v1: Adt11) -> R<(u64, Adt26)> {
    f69(v0, v1)
}

pub fn f40(v0: Fn3, p213: u64, p214: (u64, Adt26)) -> R<((u64, Adt26), bool)> {
    let m212 = (p213, p214);
    {
        let mut m212 = m212;
        loop {
            let step = {
                let (v1, v2) = m212;
                let m209 = v1;
                if m209 == 0 {
                    Err::<(u64, (u64, Adt26)), R<((u64, Adt26), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m209 - 1;
                    let m211 = {
                        let c210: Fn3 = v0.clone();
                        c210.apply(v2.clone())?
                    };
                    match m211 {
                        None => {
                            Err::<(u64, (u64, Adt26)), R<((u64, Adt26), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (u64, Adt26)), R<((u64, Adt26), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m212 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f41(v0: u64, v1: u64, v2: (u64, Adt26)) -> R<Option<(u64, Adt26)>> {
    f70(v1, v0, v2)
}

pub fn f42(v0: Adt26, v1: Adt19) -> R<Adt26> {
    let m215 = v1;
    match m215 {
        Adt19::C0 => {
            f8(v0, Str::lit("()"))
        }
        Adt19::C1(v2) => {
            if v2 {
                f8(v0, Str::lit("true"))
            } else {
                f8(v0, Str::lit("false"))
            }
        }
        Adt19::C2(v3) => {
            f8(f19(v0, v3)?, Str::lit("u64"))
        }
        Adt19::C3(v4, v5) => {
            if if v4 {
                let a216 = v5;
                let a217 = 9223372036854775808u64;
                nat_eq(a216, a217)
            } else {
                false
            } {
                f8(v0, Str::lit("i64::MIN"))
            } else {
                f8(f19(if v4 {
                    f8(v0, Str::lit("-"))?
                } else {
                    v0
                }, v5)?, Str::lit("i64"))
            }
        }
        Adt19::C4(v6, v7, v8) => {
            if if v7 {
                let a218 = v8;
                let a219 = f71(v6.clone());
                nat_eq(a218, a219)
            } else {
                false
            } {
                f8(f32(v0, v6)?, Str::lit("::MIN"))
            } else {
                f32(f19(if v7 {
                    f8(v0, Str::lit("-"))?
                } else {
                    v0
                }, v8)?, v6)
            }
        }
        Adt19::C5(v9) => {
            f8(f72(f8(v0, Str::lit("Str::lit("))?, v9)?, Str::lit(")"))
        }
        Adt19::C6(v10) => {
            f73(v0, v10)
        }
        Adt19::C7(v11) => {
            f74(v0, v11)
        }
        Adt19::C8 => {
            f8(v0, Str::lit("Overflow"))
        }
    }
}

pub fn f43(v0: Adt18) -> u64 {
    let m220 = v0;
    match m220 {
        Adt18::C0(_, _) => {
            1000u64
        }
        Adt18::C1(_, _, _) => {
            1000u64
        }
        Adt18::C2(_, _, _) => {
            1000u64
        }
        Adt18::C3(_, _, _, _) => {
            1000u64
        }
        Adt18::C4(_, _, _, _) => {
            1000u64
        }
        Adt18::C5(_, _, _) => {
            1000u64
        }
        Adt18::C6(_, _, _) => {
            1000u64
        }
        Adt18::C7(_, _, _, _) => {
            1000u64
        }
        Adt18::C8(_, _, _) => {
            1000u64
        }
        Adt18::C9(_) => {
            1000u64
        }
        Adt18::C10(v31, _, _) => {
            let m221 = v31;
            match m221 {
                Adt25::C0 => {
                    0u64
                }
                Adt25::C1 => {
                    1u64
                }
                Adt25::C2 => {
                    2u64
                }
                Adt25::C3 => {
                    3u64
                }
            }
        }
        Adt18::C11(_, _) => {
            1000u64
        }
        Adt18::C12 => {
            1000u64
        }
        Adt18::C13(_, _) => {
            1000u64
        }
        Adt18::C14(_, _, _, _, _, _) => {
            1000u64
        }
        Adt18::C15(_, _, _) => {
            1000u64
        }
        Adt18::C16(_, _) => {
            1000u64
        }
    }
}

pub fn f44(v0: Adt22) -> u64 {
    let m222 = v0;
    match m222 {
        Adt22::C0(_) => {
            1000u64
        }
        Adt22::C1(_) => {
            1000u64
        }
        Adt22::C2(v3, _) => {
            v3
        }
    }
}

pub fn f45(v0: Adt26, v1: Adt23) -> R<Adt26> {
    let v2: u64 = match v1.clone() {
        Adt23::C0(h223, _) => {
            h223
        }
    };
    let m225 = match v1 {
        Adt23::C0(_, h224) => {
            h224
        }
    };
    match m225 {
        None => {
            f8(v0, {
                let a232 = v2;
                let a233 = 20u64;
                if nat_lt(a232, a233) {
                    f76(v2)
                } else {
                    f77(v2)
                }
            })
        }
        Some(v3) => {
            let a226 = v2;
            let a227 = 24u64;
            if nat_eq(a226, a227) {
                f32(f8(v0, Str::lit("checked_neg_"))?, v3)
            } else {
                let a228 = v2;
                let a229 = 47u64;
                if nat_eq(a228, a229) {
                    f32(f8(v0, Str::lit("format_"))?, v3)
                } else {
                    let a230 = v2;
                    let a231 = 49u64;
                    if nat_eq(a230, a231) {
                        f32(f8(v0, Str::lit("parse_"))?, v3)
                    } else {
                        f8(f8(f32(f8(v0, Str::lit("fixed_"))?, v3)?, Str::lit("::"))?, f75(v2))
                    }
                }
            }
        }
    }
}

pub fn f46(v0: Adt26, v1: u64, v2: Option<Adt4>) -> R<Adt26> {
    let a234 = v1;
    let a235 = 8u64;
    if nat_lt(a234, a235) {
        let m236 = v2;
        match m236 {
            None => {
                Ok(v0)
            }
            Some(v3) => {
                f8(f8(f8(f32(v0, v3)?, Str::lit("::"))?, {
                    let a237 = v1;
                    let a238 = 7u64;
                    if nat_eq(a237, a238) {
                        Str::lit("try_from")
                    } else {
                        f78(v1)
                    }
                })?, Str::lit("("))
            }
        }
    } else {
        f8(v0, f79(v1))
    }
}

pub fn f47(v0: Adt26, v1: u64) -> R<Adt26> {
    let a239 = v1;
    let a240 = 8u64;
    if nat_lt(a239, a240) {
        f8(v0, {
            let a241 = v1;
            let a242 = 7u64;
            if nat_eq(a241, a242) {
                Str::lit(").ok()")
            } else {
                Str::lit(")")
            }
        })
    } else {
        f8(v0, f80(v1))
    }
}

pub fn f48(v0: Adt26, v1: Adt24) -> R<Adt26> {
    let m243 = v1;
    match m243 {
        Adt24::C0(v2) => {
            f8(f16(f8(v0, Str::lit("None::<"))?, v2)?, Str::lit(">"))
        }
        Adt24::C1 => {
            f8(v0, Str::lit("Some("))
        }
        Adt24::C2(v3, v4) => {
            f8(f16(f8(f16(f8(v0, Str::lit("Ok::<"))?, v3)?, Str::lit(", "))?, v4)?, Str::lit(">("))
        }
        Adt24::C3(v5, v6) => {
            f8(f16(f8(f16(f8(v0, Str::lit("Err::<"))?, v5)?, Str::lit(", "))?, v6)?, Str::lit(">("))
        }
        Adt24::C4(v7, v8) => {
            f19(f8(f19(f8(v0, Str::lit("Adt"))?, v7)?, Str::lit("::C"))?, v8)
        }
        Adt24::C5(v9, v10) => {
            f19(f8(f19(f8(v0, Str::lit("Fn"))?, v9)?, Str::lit("::F"))?, v10)
        }
        Adt24::C6 => {
            f8(v0, Str::lit("List::cons("))
        }
        Adt24::C7(v11) => {
            f8(f16(f8(v0, Str::lit("List::<"))?, v11)?, Str::lit(">::nil()"))
        }
    }
}

pub fn f49(v0: Adt24) -> u64 {
    let m244 = v0;
    match m244 {
        Adt24::C0(_) => {
            0u64
        }
        Adt24::C1 => {
            1u64
        }
        Adt24::C2(_, _) => {
            1u64
        }
        Adt24::C3(_, _) => {
            1u64
        }
        Adt24::C4(_, _) => {
            2u64
        }
        Adt24::C5(_, _) => {
            2u64
        }
        Adt24::C6 => {
            1u64
        }
        Adt24::C7(_) => {
            0u64
        }
    }
}

pub fn f50(v0: Adt18) -> bool {
    let a245 = f52(v0);
    let a246 = 13u64;
    nat_eq(a245, a246)
}

pub fn f51(v0: Adt18) -> bool {
    let m247 = v0;
    match m247 {
        Adt18::C0(_, _) => {
            false
        }
        Adt18::C1(_, _, _) => {
            false
        }
        Adt18::C2(_, _, _) => {
            false
        }
        Adt18::C3(_, _, _, _) => {
            false
        }
        Adt18::C4(_, _, _, _) => {
            false
        }
        Adt18::C5(_, _, _) => {
            false
        }
        Adt18::C6(_, _, _) => {
            false
        }
        Adt18::C7(_, _, _, _) => {
            false
        }
        Adt18::C8(_, _, _) => {
            false
        }
        Adt18::C9(_) => {
            false
        }
        Adt18::C10(_, _, _) => {
            false
        }
        Adt18::C11(r250, r251) => {
            let v34 = (*r250).clone();
            let v35 = (*r251).clone();
            let a248 = f52(v34);
            let a249 = 12u64;
            if nat_eq(a248, a249) {
                f55(v35)
            } else {
                false
            }
        }
        Adt18::C12 => {
            false
        }
        Adt18::C13(_, _) => {
            false
        }
        Adt18::C14(_, _, _, _, _, _) => {
            false
        }
        Adt18::C15(_, _, _) => {
            false
        }
        Adt18::C16(_, _) => {
            false
        }
    }
}

pub fn f52(v0: Adt18) -> u64 {
    let m252 = v0;
    match m252 {
        Adt18::C0(_, _) => {
            0u64
        }
        Adt18::C1(_, _, _) => {
            1u64
        }
        Adt18::C2(_, _, _) => {
            2u64
        }
        Adt18::C3(_, _, _, _) => {
            3u64
        }
        Adt18::C4(_, _, _, _) => {
            4u64
        }
        Adt18::C5(_, _, _) => {
            5u64
        }
        Adt18::C6(_, _, _) => {
            6u64
        }
        Adt18::C7(_, _, _, _) => {
            7u64
        }
        Adt18::C8(_, _, _) => {
            8u64
        }
        Adt18::C9(_) => {
            9u64
        }
        Adt18::C10(_, _, _) => {
            10u64
        }
        Adt18::C11(_, _) => {
            11u64
        }
        Adt18::C12 => {
            12u64
        }
        Adt18::C13(_, _) => {
            13u64
        }
        Adt18::C14(_, _, _, _, _, _) => {
            14u64
        }
        Adt18::C15(_, _, _) => {
            15u64
        }
        Adt18::C16(_, _) => {
            16u64
        }
    }
}

pub fn f53(p263: Adt18) -> bool {
    let m262 = p263;
    {
        let mut m262 = m262;
        loop {
            let step = {
                let v0 = m262;
                let m253 = v0;
                match m253 {
                    Adt18::C0(_, _) => {
                        Err::<Adt18, bool>(false)
                    }
                    Adt18::C1(_, _, _) => {
                        Err::<Adt18, bool>(false)
                    }
                    Adt18::C2(_, _, _) => {
                        Err::<Adt18, bool>(false)
                    }
                    Adt18::C3(v9, _, _, _) => {
                        let a254 = f44(v9.clone());
                        let a255 = 16u64;
                        if nat_eq(a254, a255) {
                            Err::<Adt18, bool>(true)
                        } else {
                            let a256 = f44(v9);
                            let a257 = 25u64;
                            Err::<Adt18, bool>(nat_eq(a256, a257))
                        }
                    }
                    Adt18::C4(_, _, _, _) => {
                        Err::<Adt18, bool>(false)
                    }
                    Adt18::C5(_, _, _) => {
                        Err::<Adt18, bool>(false)
                    }
                    Adt18::C6(_, _, _) => {
                        Err::<Adt18, bool>(false)
                    }
                    Adt18::C7(_, _, _, _) => {
                        Err::<Adt18, bool>(false)
                    }
                    Adt18::C8(r260, r261, _) => {
                        let v27 = (*r260).clone();
                        let v28 = (*r261).clone();
                        let a258 = f52(v28);
                        let a259 = 15u64;
                        if nat_eq(a258, a259) {
                            Ok::<Adt18, bool>(v27)
                        } else {
                            Err::<Adt18, bool>(false)
                        }
                    }
                    Adt18::C9(_) => {
                        Err::<Adt18, bool>(true)
                    }
                    Adt18::C10(_, _, _) => {
                        Err::<Adt18, bool>(false)
                    }
                    Adt18::C11(_, _) => {
                        Err::<Adt18, bool>(false)
                    }
                    Adt18::C12 => {
                        Err::<Adt18, bool>(false)
                    }
                    Adt18::C13(_, _) => {
                        Err::<Adt18, bool>(false)
                    }
                    Adt18::C14(_, _, _, _, _, _) => {
                        Err::<Adt18, bool>(false)
                    }
                    Adt18::C15(_, _, _) => {
                        Err::<Adt18, bool>(false)
                    }
                    Adt18::C16(_, _) => {
                        Err::<Adt18, bool>(true)
                    }
                }
            };
            match step {
                Ok(next) => { m262 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f54(v0: Adt26, v1: u64) -> R<Adt26> {
    let (_, h264) = {
        let (h265, _) = f40(Fn3::F81(v1), v1, (0u64, v0))?;
        h265
    };
    Ok(h264)
}

pub fn f55(v0: Adt18) -> bool {
    let m266 = v0;
    match m266 {
        Adt18::C0(v1, _) => {
            let m267 = v1;
            match m267 {
                Adt19::C0 => {
                    true
                }
                Adt19::C1(_) => {
                    false
                }
                Adt19::C2(_) => {
                    false
                }
                Adt19::C3(_, _) => {
                    false
                }
                Adt19::C4(_, _, _) => {
                    false
                }
                Adt19::C5(_) => {
                    false
                }
                Adt19::C6(_) => {
                    false
                }
                Adt19::C7(_) => {
                    false
                }
                Adt19::C8 => {
                    false
                }
            }
        }
        Adt18::C1(_, _, _) => {
            false
        }
        Adt18::C2(_, _, _) => {
            false
        }
        Adt18::C3(_, _, _, _) => {
            false
        }
        Adt18::C4(_, _, _, _) => {
            false
        }
        Adt18::C5(_, _, _) => {
            false
        }
        Adt18::C6(_, _, _) => {
            false
        }
        Adt18::C7(_, _, _, _) => {
            false
        }
        Adt18::C8(_, _, _) => {
            false
        }
        Adt18::C9(_) => {
            false
        }
        Adt18::C10(_, _, _) => {
            false
        }
        Adt18::C11(_, _) => {
            false
        }
        Adt18::C12 => {
            false
        }
        Adt18::C13(_, _) => {
            false
        }
        Adt18::C14(_, _, _, _, _, _) => {
            false
        }
        Adt18::C15(_, _, _) => {
            false
        }
        Adt18::C16(_, _) => {
            false
        }
    }
}

pub fn f56(v0: u64) -> u64 {
    let a268 = v0;
    let a269 = 8u64;
    if if nat_lt(a268, a269) {
        true
    } else {
        let a270 = 41u64;
        let a271 = v0;
        nat_le(a270, a271)
    } {
        100000u64
    } else {
        let a272 = v0;
        let a273 = 11u64;
        if nat_eq(a272, a273) {
            1u64
        } else {
            let a274 = v0;
            let a275 = 24u64;
            if if nat_eq(a274, a275) {
                true
            } else {
                let a276 = v0;
                let a277 = 26u64;
                if nat_eq(a276, a277) {
                    true
                } else {
                    let a278 = v0;
                    let a279 = 31u64;
                    if nat_eq(a278, a279) {
                        true
                    } else {
                        let a280 = v0;
                        let a281 = 32u64;
                        if nat_eq(a280, a281) {
                            true
                        } else {
                            let a282 = v0;
                            let a283 = 35u64;
                            if nat_eq(a282, a283) {
                                true
                            } else {
                                let a284 = v0;
                                let a285 = 36u64;
                                nat_eq(a284, a285)
                            }
                        }
                    }
                }
            } {
                1u64
            } else {
                let a286 = v0;
                let a287 = 15u64;
                if nat_eq(a286, a287) {
                    3u64
                } else {
                    let a288 = v0;
                    let a289 = 16u64;
                    if nat_eq(a288, a289) {
                        3u64
                    } else {
                        let a290 = v0;
                        let a291 = 20u64;
                        if nat_eq(a290, a291) {
                            3u64
                        } else {
                            let a292 = v0;
                            let a293 = 25u64;
                            if if nat_eq(a292, a293) {
                                true
                            } else {
                                let a294 = v0;
                                let a295 = 29u64;
                                if nat_eq(a294, a295) {
                                    true
                                } else {
                                    let a296 = v0;
                                    let a297 = 34u64;
                                    if nat_eq(a296, a297) {
                                        true
                                    } else {
                                        let a298 = v0;
                                        let a299 = 38u64;
                                        if nat_eq(a298, a299) {
                                            true
                                        } else {
                                            let a300 = v0;
                                            let a301 = 40u64;
                                            nat_eq(a300, a301)
                                        }
                                    }
                                }
                            } {
                                3u64
                            } else {
                                2u64
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f57(v0: Adt26, v1: u64, v2: u64) -> R<Adt26> {
    let a302 = v1;
    let a303 = 8u64;
    if nat_lt(a302, a303) {
        f8(v0, Str::lit(", "))
    } else {
        let a304 = v1;
        let a305 = 100u64;
        if nat_eq(a304, a305) {
            f8(v0, Str::lit(", "))
        } else {
            let a306 = v2;
            let a307 = 0u64;
            if nat_eq(a306, a307) {
                f8(v0, f82(v1))
            } else {
                f8(v0, f83(v1))
            }
        }
    }
}

pub fn f58(v0: Adt26, v1: Adt14) -> R<Adt26> {
    let m308 = v1;
    match m308 {
        Adt14::C0(v2) => {
            f8(v0, {
                let m309 = v2;
                match m309 {
                    Adt15::C0 => {
                        Str::lit("_")
                    }
                    Adt15::C1 => {
                        Str::lit("()")
                    }
                    Adt15::C2 => {
                        Str::lit("None")
                    }
                }
            })
        }
        Adt14::C1(v3) => {
            f28(v0, v3)
        }
        Adt14::C2(r313) => {
            let v4 = (*r313).clone();
            f8(f58(f8(v0, Str::lit("("))?, v4)?, Str::lit(")"))
        }
        Adt14::C3(v5, r314) => {
            let v6 = (*r314).clone();
            f8(f58(f8(v0, {
                let m310 = v5;
                match m310 {
                    Adt16::C0 => {
                        Str::lit("Some(")
                    }
                    Adt16::C1 => {
                        Str::lit("Ok(")
                    }
                    Adt16::C2 => {
                        Str::lit("Err(")
                    }
                }
            })?, v6)?, Str::lit(")"))
        }
        Adt14::C4(v7, v8, r315) => {
            let v9 = (*r315).clone();
            let v10: Adt26 = f19(f8(f19(f8(v0, Str::lit("Adt"))?, v7)?, Str::lit("::C"))?, v8)?;
            let m311 = v9.clone();
            match m311 {
                Adt14::C0(_) => {
                    Ok(v10)
                }
                Adt14::C1(_) => {
                    Ok(v10)
                }
                Adt14::C2(_) => {
                    Ok(v10)
                }
                Adt14::C3(_, _) => {
                    Ok(v10)
                }
                Adt14::C4(_, _, _) => {
                    Ok(v10)
                }
                Adt14::C5 => {
                    Ok(v10)
                }
                Adt14::C6(_, _) => {
                    f8(f58(f8(v10, Str::lit("("))?, v9)?, Str::lit(")"))
                }
                Adt14::C7(_) => {
                    Ok(v10)
                }
            }
        }
        Adt14::C5 => {
            Ok(v0)
        }
        Adt14::C6(r316, r317) => {
            let v22 = (*r316).clone();
            let v23 = (*r317).clone();
            let m312 = v23.clone();
            match m312 {
                Adt14::C0(_) => {
                    f58(v0, v22)
                }
                Adt14::C1(_) => {
                    f58(v0, v22)
                }
                Adt14::C2(_) => {
                    f58(v0, v22)
                }
                Adt14::C3(_, _) => {
                    f58(v0, v22)
                }
                Adt14::C4(_, _, _) => {
                    f58(v0, v22)
                }
                Adt14::C5 => {
                    f58(v0, v22)
                }
                Adt14::C6(_, _) => {
                    f58(f8(f58(v0, v22)?, Str::lit(", "))?, v23)
                }
                Adt14::C7(_) => {
                    f58(v0, v22)
                }
            }
        }
        Adt14::C7(v35) => {
            f74(v0, v35)
        }
    }
}

pub fn f59(v0: u64, v1: u64) -> u64 {
    let a318 = v0;
    let a319 = v1;
    if nat_lt(a318, a319) {
        v1
    } else {
        v0
    }
}

pub fn f60(v0: Adt13) -> R<u64> {
    let m320 = v0;
    match m320 {
        Adt13::C0 => {
            Ok(0u64)
        }
        Adt13::C1(_, v2, r321) => {
            let v3 = (*r321).clone();
            Ok(f59(f61(v2)?, f60(v3)?))
        }
    }
}

pub fn f61(v0: Adt2) -> R<u64> {
    let m322 = v0;
    match m322 {
        Adt2::C0(_) => {
            Ok(0u64)
        }
        Adt2::C1(_) => {
            Ok(0u64)
        }
        Adt2::C2(_, r326) => {
            let v6 = (*r326).clone();
            f61(v6)
        }
        Adt2::C3(_, r327, r328) => {
            let v8 = (*r327).clone();
            let v9 = (*r328).clone();
            Ok(f59(f61(v8)?, f61(v9)?))
        }
        Adt2::C4(v1, v2) => {
            let m323 = v1;
            match m323 {
                Adt7::C0 => {
                    Ok(0u64)
                }
                Adt7::C1 => {
                    Ok(0u64)
                }
                Adt7::C2 => {
                    let a324 = v2;
                    let a325 = 1u64;
                    nat_add(a324, a325)
                }
            }
        }
        Adt2::C5 => {
            Ok(0u64)
        }
        Adt2::C6(r329, r330) => {
            let v10 = (*r329).clone();
            let v11 = (*r330).clone();
            Ok(f59(f61(v10)?, f61(v11)?))
        }
    }
}

pub fn f62(v0: Fn4, p335: u64, p336: (u64, (Adt26, bool))) -> R<((u64, (Adt26, bool)), bool)> {
    let m334 = (p335, p336);
    {
        let mut m334 = m334;
        loop {
            let step = {
                let (v1, v2) = m334;
                let m331 = v1;
                if m331 == 0 {
                    Err::<(u64, (u64, (Adt26, bool))), R<((u64, (Adt26, bool)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m331 - 1;
                    let m333 = {
                        let c332: Fn4 = v0.clone();
                        c332.apply(v2.clone())?
                    };
                    match m333 {
                        None => {
                            Err::<(u64, (u64, (Adt26, bool))), R<((u64, (Adt26, bool)), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (u64, (Adt26, bool))), R<((u64, (Adt26, bool)), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m334 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f63(v0: u64, v1: Adt13, v2: Adt2, v3: (u64, (Adt26, bool))) -> R<Option<(u64, (Adt26, bool))>> {
    f84(v1, v2, v0, v3)
}

pub fn f64(v0: Fn5, p340: (Option<Bytes>, List<Bytes>), p341: List<Bytes>) -> (Option<Bytes>, List<Bytes>) {
    let m339 = (p340, p341);
    {
        let mut m339 = m339;
        loop {
            let step = {
                let (v1, v2) = m339;
                let m337 = v2;
                match m337.uncons() {
                    None => {
                        Err::<((Option<Bytes>, List<Bytes>), List<Bytes>), (Option<Bytes>, List<Bytes>)>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<((Option<Bytes>, List<Bytes>), List<Bytes>), (Option<Bytes>, List<Bytes>)>(({
                            let c338: Fn5 = v0.clone();
                            c338.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m339 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f65(v0: (Option<Bytes>, List<Bytes>), v1: Bytes) -> (Option<Bytes>, List<Bytes>) {
    f85(v0, v1)
}

pub fn f66(v0: Adt4) -> Str {
    let m342 = v0;
    match m342 {
        Adt4::C0 => {
            Str::lit("u8")
        }
        Adt4::C1 => {
            Str::lit("u16")
        }
        Adt4::C2 => {
            Str::lit("u32")
        }
        Adt4::C3 => {
            Str::lit("u64")
        }
        Adt4::C4 => {
            Str::lit("i8")
        }
        Adt4::C5 => {
            Str::lit("i16")
        }
        Adt4::C6 => {
            Str::lit("i32")
        }
        Adt4::C7 => {
            Str::lit("i64")
        }
    }
}

pub fn f67(v0: u64) -> R<Bytes> {
    let v1: Bytes = {
        let a379 = f86(v0, 10000000000000000000u64);
        let a380 = {
            let a377 = f86(v0, 1000000000000000000u64);
            let a378 = {
                let a375 = f86(v0, 100000000000000000u64);
                let a376 = {
                    let a373 = f86(v0, 10000000000000000u64);
                    let a374 = {
                        let a371 = f86(v0, 1000000000000000u64);
                        let a372 = {
                            let a369 = f86(v0, 100000000000000u64);
                            let a370 = {
                                let a367 = f86(v0, 10000000000000u64);
                                let a368 = {
                                    let a365 = f86(v0, 1000000000000u64);
                                    let a366 = {
                                        let a363 = f86(v0, 100000000000u64);
                                        let a364 = {
                                            let a361 = f86(v0, 10000000000u64);
                                            let a362 = {
                                                let a359 = f86(v0, 1000000000u64);
                                                let a360 = {
                                                    let a357 = f86(v0, 100000000u64);
                                                    let a358 = {
                                                        let a355 = f86(v0, 10000000u64);
                                                        let a356 = {
                                                            let a353 = f86(v0, 1000000u64);
                                                            let a354 = {
                                                                let a351 = f86(v0, 100000u64);
                                                                let a352 = {
                                                                    let a349 = f86(v0, 10000u64);
                                                                    let a350 = {
                                                                        let a347 = f86(v0, 1000u64);
                                                                        let a348 = {
                                                                            let a345 = f86(v0, 100u64);
                                                                            let a346 = {
                                                                                let a343 = f86(v0, 10u64);
                                                                                let a344 = f86(v0, 1u64);
                                                                                append_bytes(a343, a344)
                                                                            };
                                                                            append_bytes(a345, a346)
                                                                        };
                                                                        append_bytes(a347, a348)
                                                                    };
                                                                    append_bytes(a349, a350)
                                                                };
                                                                append_bytes(a351, a352)
                                                            };
                                                            append_bytes(a353, a354)
                                                        };
                                                        append_bytes(a355, a356)
                                                    };
                                                    append_bytes(a357, a358)
                                                };
                                                append_bytes(a359, a360)
                                            };
                                            append_bytes(a361, a362)
                                        };
                                        append_bytes(a363, a364)
                                    };
                                    append_bytes(a365, a366)
                                };
                                append_bytes(a367, a368)
                            };
                            append_bytes(a369, a370)
                        };
                        append_bytes(a371, a372)
                    };
                    append_bytes(a373, a374)
                };
                append_bytes(a375, a376)
            };
            append_bytes(a377, a378)
        };
        append_bytes(a379, a380)
    };
    let v2: u64 = {
        let (h381, _) = f87(Fn6::F88(v1.clone()), {
            let a382 = v1.clone();
            length_bytes(a382)
        }, 0u64)?;
        h381
    };
    let a383 = v0;
    let a384 = 0u64;
    if nat_eq(a383, a384) {
        let a385 = Str::lit("0");
        Ok(utf8_encode(a385))
    } else {
        Ok(f89(v1.clone(), v2, {
            let a387 = {
                let a386 = v1;
                length_bytes(a386)
            };
            let a388 = v2;
            nat_sub(a387, a388)
        }))
    }
}

pub fn f68(v0: (u64, Adt26), v1: Adt11) -> R<(u64, Adt26)> {
    let v2: Adt26 = {
        let a390 = {
            let (h389, _) = v0.clone();
            h389
        };
        let a391 = 0u64;
        if nat_eq(a390, a391) {
            let (_, h392) = v0.clone();
            h392
        } else {
            f8({
                let (_, h393) = v0.clone();
                h393
            }, Str::lit(", "))?
        }
    };
    Ok(({
        let a395 = {
            let (h394, _) = v0.clone();
            h394
        };
        let a396 = 1u64;
        nat_add(a395, a396)?
    }, {
        let m397 = v1;
        match m397 {
            Adt11::C0 => {
                f19(f8(v2, Str::lit("k"))?, {
                    let (h398, _) = v0;
                    h398
                })?
            }
            Adt11::C1 => {
                f19(f8(v2, Str::lit("k"))?, {
                    let (h399, _) = v0;
                    h399
                })?
            }
            Adt11::C2 => {
                f19(f8(v2, Str::lit("k"))?, {
                    let (h400, _) = v0;
                    h400
                })?
            }
            Adt11::C3 => {
                f8(v2, Str::lit("_"))?
            }
        }
    }))
}

pub fn f69(v0: (u64, Adt26), v1: Adt11) -> R<(u64, Adt26)> {
    let v2: Adt26 = {
        let a402 = {
            let (h401, _) = v0.clone();
            h401
        };
        let a403 = 0u64;
        if nat_eq(a402, a403) {
            let (_, h404) = v0.clone();
            h404
        } else {
            f8({
                let (_, h405) = v0.clone();
                h405
            }, Str::lit(", "))?
        }
    };
    Ok(({
        let a407 = {
            let (h406, _) = v0.clone();
            h406
        };
        let a408 = 1u64;
        nat_add(a407, a408)?
    }, {
        let m409 = v1;
        match m409 {
            Adt11::C0 => {
                f19(f8(v2, Str::lit("*k"))?, {
                    let (h410, _) = v0;
                    h410
                })?
            }
            Adt11::C1 => {
                f8(f19(f8(v2, Str::lit("k"))?, {
                    let (h411, _) = v0;
                    h411
                })?, Str::lit(".clone()"))?
            }
            Adt11::C2 => {
                f8(f19(f8(v2, Str::lit("(**k"))?, {
                    let (h412, _) = v0;
                    h412
                })?, Str::lit(").clone()"))?
            }
            Adt11::C3 => {
                f8(v2, Str::lit("()"))?
            }
        }
    }))
}

pub fn f70(v0: u64, v1: u64, v2: (u64, Adt26)) -> R<Option<(u64, Adt26)>> {
    let a414 = {
        let (h413, _) = v2.clone();
        h413
    };
    let a415 = v0;
    if nat_lt(a414, a415) {
        Ok(Some(({
            let a417 = {
                let (h416, _) = v2.clone();
                h416
            };
            let a418 = 1u64;
            nat_add(a417, a418)?
        }, f19(f8({
            let a420 = {
                let (h419, _) = v2.clone();
                h419
            };
            let a421 = 0u64;
            if if nat_eq(a420, a421) {
                let a422 = v1;
                let a423 = 0u64;
                nat_eq(a422, a423)
            } else {
                false
            } {
                let (_, h424) = v2;
                h424
            } else {
                f8({
                    let (_, h425) = v2;
                    h425
                }, Str::lit(", "))?
            }
        }, Str::lit("p"))?, {
            let (h426, _) = v2;
            h426
        })?)))
    } else {
        Ok(None::<(u64, Adt26)>)
    }
}

pub fn f71(v0: Adt4) -> u64 {
    let m427 = v0;
    match m427 {
        Adt4::C0 => {
            0u64
        }
        Adt4::C1 => {
            0u64
        }
        Adt4::C2 => {
            0u64
        }
        Adt4::C3 => {
            0u64
        }
        Adt4::C4 => {
            128u64
        }
        Adt4::C5 => {
            32768u64
        }
        Adt4::C6 => {
            2147483648u64
        }
        Adt4::C7 => {
            9223372036854775808u64
        }
    }
}

pub fn f72(v0: Adt26, v1: Bytes) -> R<Adt26> {
    f8({
        let (_, h428) = {
            let (h429, _) = f40(Fn3::F90(v1.clone()), {
                let a430 = v1;
                length_bytes(a430)
            }, (0u64, f8(v0, Str::lit("\""))?))?;
            h429
        };
        h428
    }, Str::lit("\""))
}

pub fn f73(v0: Adt26, v1: Bytes) -> R<Adt26> {
    f8({
        let (_, h431) = {
            let (h432, _) = f40(Fn3::F91(v1.clone()), {
                let a433 = v1;
                length_bytes(a433)
            }, (0u64, f8(v0, Str::lit("Bytes::lit(&["))?))?;
            h432
        };
        h431
    }, Str::lit("])"))
}

pub fn f74(v0: Adt26, v1: Adt17) -> R<Adt26> {
    f8(v0, {
        let m434 = v1;
        match m434 {
            Adt17::C0 => {
                Str::lit("Ordering::Less")
            }
            Adt17::C1 => {
                Str::lit("Ordering::Equal")
            }
            Adt17::C2 => {
                Str::lit("Ordering::Greater")
            }
        }
    })
}

pub fn f75(v0: u64) -> Str {
    let a435 = v0;
    let a436 = 20u64;
    if nat_eq(a435, a436) {
        Str::lit("checked_add")
    } else {
        let a437 = v0;
        let a438 = 21u64;
        if nat_eq(a437, a438) {
            Str::lit("checked_sub")
        } else {
            let a439 = v0;
            let a440 = 22u64;
            if nat_eq(a439, a440) {
                Str::lit("checked_mul")
            } else {
                let a441 = v0;
                let a442 = 23u64;
                if nat_eq(a441, a442) {
                    Str::lit("checked_quot")
                } else {
                    let a443 = v0;
                    let a444 = 25u64;
                    if nat_eq(a443, a444) {
                        Str::lit("bit_and")
                    } else {
                        let a445 = v0;
                        let a446 = 26u64;
                        if nat_eq(a445, a446) {
                            Str::lit("bit_or")
                        } else {
                            let a447 = v0;
                            let a448 = 27u64;
                            if nat_eq(a447, a448) {
                                Str::lit("bit_xor")
                            } else {
                                let a449 = v0;
                                let a450 = 28u64;
                                if nat_eq(a449, a450) {
                                    Str::lit("bit_not")
                                } else {
                                    let a451 = v0;
                                    let a452 = 29u64;
                                    if nat_eq(a451, a452) {
                                        Str::lit("shift_left")
                                    } else {
                                        let a453 = v0;
                                        let a454 = 30u64;
                                        if nat_eq(a453, a454) {
                                            Str::lit("shift_right")
                                        } else {
                                            let a455 = v0;
                                            let a456 = 31u64;
                                            if nat_eq(a455, a456) {
                                                Str::lit("convert")
                                            } else {
                                                Str::lit("")
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

pub fn f76(v0: u64) -> Str {
    let a457 = v0;
    let a458 = 0u64;
    if nat_eq(a457, a458) {
        Str::lit("nat_add")
    } else {
        let a459 = v0;
        let a460 = 1u64;
        if nat_eq(a459, a460) {
            Str::lit("nat_sub")
        } else {
            let a461 = v0;
            let a462 = 2u64;
            if nat_eq(a461, a462) {
                Str::lit("nat_mul")
            } else {
                let a463 = v0;
                let a464 = 3u64;
                if nat_eq(a463, a464) {
                    Str::lit("nat_quot")
                } else {
                    let a465 = v0;
                    let a466 = 4u64;
                    if nat_eq(a465, a466) {
                        Str::lit("nat_rem")
                    } else {
                        let a467 = v0;
                        let a468 = 5u64;
                        if nat_eq(a467, a468) {
                            Str::lit("nat_eq")
                        } else {
                            let a469 = v0;
                            let a470 = 6u64;
                            if nat_eq(a469, a470) {
                                Str::lit("nat_le")
                            } else {
                                let a471 = v0;
                                let a472 = 7u64;
                                if nat_eq(a471, a472) {
                                    Str::lit("nat_lt")
                                } else {
                                    let a473 = v0;
                                    let a474 = 8u64;
                                    if nat_eq(a473, a474) {
                                        Str::lit("nat_succ")
                                    } else {
                                        let a475 = v0;
                                        let a476 = 9u64;
                                        if nat_eq(a475, a476) {
                                            Str::lit("int_add")
                                        } else {
                                            let a477 = v0;
                                            let a478 = 10u64;
                                            if nat_eq(a477, a478) {
                                                Str::lit("int_sub")
                                            } else {
                                                let a479 = v0;
                                                let a480 = 11u64;
                                                if nat_eq(a479, a480) {
                                                    Str::lit("int_mul")
                                                } else {
                                                    let a481 = v0;
                                                    let a482 = 12u64;
                                                    if nat_eq(a481, a482) {
                                                        Str::lit("int_neg")
                                                    } else {
                                                        let a483 = v0;
                                                        let a484 = 13u64;
                                                        if nat_eq(a483, a484) {
                                                            Str::lit("int_quot")
                                                        } else {
                                                            let a485 = v0;
                                                            let a486 = 14u64;
                                                            if nat_eq(a485, a486) {
                                                                Str::lit("int_rem")
                                                            } else {
                                                                let a487 = v0;
                                                                let a488 = 15u64;
                                                                if nat_eq(a487, a488) {
                                                                    Str::lit("bool_not")
                                                                } else {
                                                                    let a489 = v0;
                                                                    let a490 = 16u64;
                                                                    if nat_eq(a489, a490) {
                                                                        Str::lit("bool_and")
                                                                    } else {
                                                                        let a491 = v0;
                                                                        let a492 = 17u64;
                                                                        if nat_eq(a491, a492) {
                                                                            Str::lit("bool_or")
                                                                        } else {
                                                                            let a493 = v0;
                                                                            let a494 = 18u64;
                                                                            if nat_eq(a493, a494) {
                                                                                Str::lit("equal")
                                                                            } else {
                                                                                let a495 = v0;
                                                                                let a496 = 19u64;
                                                                                if nat_eq(a495, a496) {
                                                                                    Str::lit("compare")
                                                                                } else {
                                                                                    Str::lit("")
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

pub fn f77(v0: u64) -> Str {
    let a497 = v0;
    let a498 = 32u64;
    if nat_eq(a497, a498) {
        Str::lit("append_list")
    } else {
        let a499 = v0;
        let a500 = 33u64;
        if nat_eq(a499, a500) {
            Str::lit("append_bytes")
        } else {
            let a501 = v0;
            let a502 = 34u64;
            if nat_eq(a501, a502) {
                Str::lit("length_list")
            } else {
                let a503 = v0;
                let a504 = 35u64;
                if nat_eq(a503, a504) {
                    Str::lit("length_bytes")
                } else {
                    let a505 = v0;
                    let a506 = 36u64;
                    if nat_eq(a505, a506) {
                        Str::lit("length_string")
                    } else {
                        let a507 = v0;
                        let a508 = 37u64;
                        if nat_eq(a507, a508) {
                            Str::lit("index_list")
                        } else {
                            let a509 = v0;
                            let a510 = 38u64;
                            if nat_eq(a509, a510) {
                                Str::lit("index_bytes")
                            } else {
                                let a511 = v0;
                                let a512 = 39u64;
                                if nat_eq(a511, a512) {
                                    Str::lit("slice_list")
                                } else {
                                    let a513 = v0;
                                    let a514 = 40u64;
                                    if nat_eq(a513, a514) {
                                        Str::lit("slice_bytes")
                                    } else {
                                        let a515 = v0;
                                        let a516 = 41u64;
                                        if nat_eq(a515, a516) {
                                            Str::lit("utf8_encode")
                                        } else {
                                            let a517 = v0;
                                            let a518 = 42u64;
                                            if nat_eq(a517, a518) {
                                                Str::lit("utf8_decode")
                                            } else {
                                                let a519 = v0;
                                                let a520 = 43u64;
                                                if nat_eq(a519, a520) {
                                                    Str::lit("compare_bytes")
                                                } else {
                                                    let a521 = v0;
                                                    let a522 = 44u64;
                                                    if nat_eq(a521, a522) {
                                                        Str::lit("split_exact")
                                                    } else {
                                                        let a523 = v0;
                                                        let a524 = 45u64;
                                                        if nat_eq(a523, a524) {
                                                            Str::lit("join")
                                                        } else {
                                                            let a525 = v0;
                                                            let a526 = 46u64;
                                                            if nat_eq(a525, a526) {
                                                                Str::lit("format_i64")
                                                            } else {
                                                                let a527 = v0;
                                                                let a528 = 48u64;
                                                                if nat_eq(a527, a528) {
                                                                    Str::lit("parse_int")
                                                                } else {
                                                                    let a529 = v0;
                                                                    let a530 = 50u64;
                                                                    if nat_eq(a529, a530) {
                                                                        Str::lit("update_bytes")
                                                                    } else {
                                                                        Str::lit("")
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

pub fn f78(v0: u64) -> Str {
    let a531 = v0;
    let a532 = 0u64;
    if nat_eq(a531, a532) {
        Str::lit("checked_add")
    } else {
        let a533 = v0;
        let a534 = 1u64;
        if nat_eq(a533, a534) {
            Str::lit("checked_sub")
        } else {
            let a535 = v0;
            let a536 = 2u64;
            if nat_eq(a535, a536) {
                Str::lit("checked_mul")
            } else {
                let a537 = v0;
                let a538 = 3u64;
                if nat_eq(a537, a538) {
                    Str::lit("checked_div")
                } else {
                    let a539 = v0;
                    let a540 = 4u64;
                    if nat_eq(a539, a540) {
                        Str::lit("checked_neg")
                    } else {
                        let a541 = v0;
                        let a542 = 5u64;
                        if nat_eq(a541, a542) {
                            Str::lit("checked_shl")
                        } else {
                            let a543 = v0;
                            let a544 = 6u64;
                            if nat_eq(a543, a544) {
                                Str::lit("checked_shr")
                            } else {
                                Str::lit("")
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f79(v0: u64) -> Str {
    let a545 = v0;
    let a546 = 40u64;
    if nat_eq(a545, a546) {
        Str::lit("Bytes::update(")
    } else {
        let a547 = v0;
        let a548 = 30u64;
        if nat_eq(a547, a548) {
            Str::lit("Bytes::append(")
        } else {
            let a549 = v0;
            let a550 = 31u64;
            if nat_eq(a549, a550) {
                Str::lit("Bytes::length(&")
            } else {
                let a551 = v0;
                let a552 = 32u64;
                if nat_eq(a551, a552) {
                    Str::lit("Str::length(&")
                } else {
                    let a553 = v0;
                    let a554 = 33u64;
                    if nat_eq(a553, a554) {
                        Str::lit("Bytes::index(&")
                    } else {
                        let a555 = v0;
                        let a556 = 34u64;
                        if nat_eq(a555, a556) {
                            Str::lit("Bytes::slice(&")
                        } else {
                            let a557 = v0;
                            let a558 = 35u64;
                            if nat_eq(a557, a558) {
                                Str::lit("Str::utf8_encode(&")
                            } else {
                                let a559 = v0;
                                let a560 = 36u64;
                                if nat_eq(a559, a560) {
                                    Str::lit("Bytes::utf8_decode(&")
                                } else {
                                    let a561 = v0;
                                    let a562 = 37u64;
                                    if nat_eq(a561, a562) {
                                        Str::lit("Bytes::compare(&")
                                    } else {
                                        let a563 = v0;
                                        let a564 = 38u64;
                                        if nat_eq(a563, a564) {
                                            Str::lit("Str::split_exact(&")
                                        } else {
                                            let a565 = v0;
                                            let a566 = 39u64;
                                            if nat_eq(a565, a566) {
                                                Str::lit("Str::join(")
                                            } else {
                                                let a567 = v0;
                                                let a568 = 8u64;
                                                if nat_eq(a567, a568) {
                                                    Str::lit("core::ops::BitAnd::bitand(")
                                                } else {
                                                    let a569 = v0;
                                                    let a570 = 9u64;
                                                    if nat_eq(a569, a570) {
                                                        Str::lit("core::ops::BitOr::bitor(")
                                                    } else {
                                                        let a571 = v0;
                                                        let a572 = 10u64;
                                                        if nat_eq(a571, a572) {
                                                            Str::lit("core::ops::BitXor::bitxor(")
                                                        } else {
                                                            let a573 = v0;
                                                            let a574 = 11u64;
                                                            if nat_eq(a573, a574) {
                                                                Str::lit("core::ops::Not::not(")
                                                            } else {
                                                                let a575 = v0;
                                                                let a576 = 12u64;
                                                                if nat_eq(a575, a576) {
                                                                    Str::lit("core::cmp::PartialEq::eq(&")
                                                                } else {
                                                                    let a577 = v0;
                                                                    let a578 = 13u64;
                                                                    if nat_eq(a577, a578) {
                                                                        Str::lit("core::cmp::PartialOrd::le(&")
                                                                    } else {
                                                                        let a579 = v0;
                                                                        let a580 = 14u64;
                                                                        if nat_eq(a579, a580) {
                                                                            Str::lit("core::cmp::PartialOrd::lt(&")
                                                                        } else {
                                                                            let a581 = v0;
                                                                            let a582 = 15u64;
                                                                            if nat_eq(a581, a582) {
                                                                                Str::lit("u64::checked_rem(")
                                                                            } else {
                                                                                let a583 = v0;
                                                                                let a584 = 16u64;
                                                                                if nat_eq(a583, a584) {
                                                                                    Str::lit("{ let (a, b, z): (i64, i64, i64) = (")
                                                                                } else {
                                                                                    let a585 = v0;
                                                                                    let a586 = 17u64;
                                                                                    if nat_eq(a585, a586) {
                                                                                        Str::lit("u64::checked_add(")
                                                                                    } else {
                                                                                        let a587 = v0;
                                                                                        let a588 = 18u64;
                                                                                        if nat_eq(a587, a588) {
                                                                                            Str::lit("u64::saturating_sub(")
                                                                                        } else {
                                                                                            let a589 = v0;
                                                                                            let a590 = 19u64;
                                                                                            if nat_eq(a589, a590) {
                                                                                                Str::lit("u64::checked_mul(")
                                                                                            } else {
                                                                                                let a591 = v0;
                                                                                                let a592 = 20u64;
                                                                                                if nat_eq(a591, a592) {
                                                                                                    Str::lit("u64::checked_div(")
                                                                                                } else {
                                                                                                    let a593 = v0;
                                                                                                    let a594 = 21u64;
                                                                                                    if nat_eq(a593, a594) {
                                                                                                        Str::lit("i64::checked_add(")
                                                                                                    } else {
                                                                                                        let a595 = v0;
                                                                                                        let a596 = 22u64;
                                                                                                        if nat_eq(a595, a596) {
                                                                                                            Str::lit("i64::checked_sub(")
                                                                                                        } else {
                                                                                                            let a597 = v0;
                                                                                                            let a598 = 23u64;
                                                                                                            if nat_eq(a597, a598) {
                                                                                                                Str::lit("i64::checked_mul(")
                                                                                                            } else {
                                                                                                                let a599 = v0;
                                                                                                                let a600 = 24u64;
                                                                                                                if nat_eq(a599, a600) {
                                                                                                                    Str::lit("i64::checked_neg(")
                                                                                                                } else {
                                                                                                                    let a601 = v0;
                                                                                                                    let a602 = 25u64;
                                                                                                                    if nat_eq(a601, a602) {
                                                                                                                        Str::lit("{ let (a, b, z): (i64, i64, i64) = (")
                                                                                                                    } else {
                                                                                                                        let a603 = v0;
                                                                                                                        let a604 = 26u64;
                                                                                                                        if nat_eq(a603, a604) {
                                                                                                                            Str::lit("List::length(&")
                                                                                                                        } else {
                                                                                                                            let a605 = v0;
                                                                                                                            let a606 = 27u64;
                                                                                                                            if nat_eq(a605, a606) {
                                                                                                                                Str::lit("List::append(")
                                                                                                                            } else {
                                                                                                                                let a607 = v0;
                                                                                                                                let a608 = 28u64;
                                                                                                                                if nat_eq(a607, a608) {
                                                                                                                                    Str::lit("List::index(&")
                                                                                                                                } else {
                                                                                                                                    let a609 = v0;
                                                                                                                                    let a610 = 29u64;
                                                                                                                                    if nat_eq(a609, a610) {
                                                                                                                                        Str::lit("List::slice(&")
                                                                                                                                    } else {
                                                                                                                                        Str::lit("")
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
                    }
                }
            }
        }
    }
}

pub fn f80(v0: u64) -> Str {
    let a611 = v0;
    let a612 = 16u64;
    if nat_eq(a611, a612) {
        Str::lit("); if b == 0 { z } else { a.wrapping_rem(b) } }")
    } else {
        let a613 = v0;
        let a614 = 25u64;
        if nat_eq(a613, a614) {
            Str::lit("); if b == 0 { Some(z) } else { a.checked_div(b) } }")
        } else {
            let a615 = v0;
            let a616 = 15u64;
            if nat_eq(a615, a616) {
                Str::lit(")")
            } else {
                let a617 = v0;
                let a618 = 20u64;
                if nat_eq(a617, a618) {
                    Str::lit(")")
                } else {
                    let a619 = v0;
                    let a620 = 8u64;
                    if nat_eq(a619, a620) {
                        Str::lit(")")
                    } else {
                        let a621 = v0;
                        let a622 = 9u64;
                        if nat_eq(a621, a622) {
                            Str::lit(")")
                        } else {
                            let a623 = v0;
                            let a624 = 10u64;
                            if nat_eq(a623, a624) {
                                Str::lit(")")
                            } else {
                                let a625 = v0;
                                let a626 = 11u64;
                                if nat_eq(a625, a626) {
                                    Str::lit(")")
                                } else {
                                    let a627 = v0;
                                    let a628 = 12u64;
                                    if nat_eq(a627, a628) {
                                        Str::lit(")")
                                    } else {
                                        let a629 = v0;
                                        let a630 = 13u64;
                                        if nat_eq(a629, a630) {
                                            Str::lit(")")
                                        } else {
                                            let a631 = v0;
                                            let a632 = 14u64;
                                            if nat_eq(a631, a632) {
                                                Str::lit(")")
                                            } else {
                                                let a633 = v0;
                                                let a634 = 17u64;
                                                if nat_eq(a633, a634) {
                                                    Str::lit(")")
                                                } else {
                                                    let a635 = v0;
                                                    let a636 = 18u64;
                                                    if nat_eq(a635, a636) {
                                                        Str::lit(")")
                                                    } else {
                                                        let a637 = v0;
                                                        let a638 = 19u64;
                                                        if nat_eq(a637, a638) {
                                                            Str::lit(")")
                                                        } else {
                                                            let a639 = v0;
                                                            let a640 = 21u64;
                                                            if nat_eq(a639, a640) {
                                                                Str::lit(")")
                                                            } else {
                                                                let a641 = v0;
                                                                let a642 = 22u64;
                                                                if nat_eq(a641, a642) {
                                                                    Str::lit(")")
                                                                } else {
                                                                    let a643 = v0;
                                                                    let a644 = 23u64;
                                                                    if nat_eq(a643, a644) {
                                                                        Str::lit(")")
                                                                    } else {
                                                                        let a645 = v0;
                                                                        let a646 = 24u64;
                                                                        let _ = nat_eq(a645, a646);
                                                                        Str::lit(")")
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

pub fn f81(v0: u64, v1: (u64, Adt26)) -> R<Option<(u64, Adt26)>> {
    f92(v0, v1)
}

pub fn f82(v0: u64) -> Str {
    let a647 = v0;
    let a648 = 37u64;
    if nat_eq(a647, a648) {
        Str::lit(", &")
    } else {
        let a649 = v0;
        let a650 = 38u64;
        if nat_eq(a649, a650) {
            Str::lit(", &")
        } else {
            let a651 = v0;
            let a652 = 39u64;
            if nat_eq(a651, a652) {
                Str::lit(", &")
            } else {
                let a653 = v0;
                let a654 = 12u64;
                if nat_eq(a653, a654) {
                    Str::lit(", &")
                } else {
                    let a655 = v0;
                    let a656 = 13u64;
                    if nat_eq(a655, a656) {
                        Str::lit(", &")
                    } else {
                        let a657 = v0;
                        let a658 = 14u64;
                        if nat_eq(a657, a658) {
                            Str::lit(", &")
                        } else {
                            let a659 = v0;
                            let a660 = 8u64;
                            if nat_eq(a659, a660) {
                                Str::lit(", ")
                            } else {
                                let a661 = v0;
                                let a662 = 9u64;
                                if nat_eq(a661, a662) {
                                    Str::lit(", ")
                                } else {
                                    let a663 = v0;
                                    let a664 = 10u64;
                                    if nat_eq(a663, a664) {
                                        Str::lit(", ")
                                    } else {
                                        let a665 = v0;
                                        let a666 = 11u64;
                                        if nat_eq(a665, a666) {
                                            Str::lit(", ")
                                        } else {
                                            let a667 = v0;
                                            let a668 = 15u64;
                                            if nat_eq(a667, a668) {
                                                Str::lit(", ")
                                            } else {
                                                let a669 = v0;
                                                let a670 = 16u64;
                                                if nat_eq(a669, a670) {
                                                    Str::lit(", ")
                                                } else {
                                                    let a671 = v0;
                                                    let a672 = 17u64;
                                                    if nat_eq(a671, a672) {
                                                        Str::lit(", ")
                                                    } else {
                                                        let a673 = v0;
                                                        let a674 = 18u64;
                                                        if nat_eq(a673, a674) {
                                                            Str::lit(", ")
                                                        } else {
                                                            let a675 = v0;
                                                            let a676 = 19u64;
                                                            if nat_eq(a675, a676) {
                                                                Str::lit(", ")
                                                            } else {
                                                                let a677 = v0;
                                                                let a678 = 20u64;
                                                                if nat_eq(a677, a678) {
                                                                    Str::lit(", ")
                                                                } else {
                                                                    let a679 = v0;
                                                                    let a680 = 21u64;
                                                                    if nat_eq(a679, a680) {
                                                                        Str::lit(", ")
                                                                    } else {
                                                                        let a681 = v0;
                                                                        let a682 = 22u64;
                                                                        if nat_eq(a681, a682) {
                                                                            Str::lit(", ")
                                                                        } else {
                                                                            let a683 = v0;
                                                                            let a684 = 23u64;
                                                                            if nat_eq(a683, a684) {
                                                                                Str::lit(", ")
                                                                            } else {
                                                                                let a685 = v0;
                                                                                let a686 = 24u64;
                                                                                if nat_eq(a685, a686) {
                                                                                    Str::lit(", ")
                                                                                } else {
                                                                                    let a687 = v0;
                                                                                    let a688 = 25u64;
                                                                                    let _ = nat_eq(a687, a688);
                                                                                    Str::lit(", ")
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

pub fn f83(v0: u64) -> Str {
    let a689 = v0;
    let a690 = 15u64;
    if nat_eq(a689, a690) {
        Str::lit(").unwrap_or(")
    } else {
        let a691 = v0;
        let a692 = 16u64;
        if nat_eq(a691, a692) {
            Str::lit(", ")
        } else {
            let a693 = v0;
            let a694 = 20u64;
            if nat_eq(a693, a694) {
                Str::lit(").unwrap_or(")
            } else {
                let a695 = v0;
                let a696 = 25u64;
                let _ = nat_eq(a695, a696);
                Str::lit(", ")
            }
        }
    }
}

pub fn f84(v0: Adt13, v1: Adt2, v2: u64, v3: (u64, (Adt26, bool))) -> R<Option<(u64, (Adt26, bool))>> {
    let a698 = {
        let (h697, _) = v3.clone();
        h697
    };
    let a699 = v2;
    if nat_lt(a698, a699) {
        Ok(Some(({
            let a701 = {
                let (h700, _) = v3.clone();
                h700
            };
            let a702 = 1u64;
            nat_add(a701, a702)?
        }, if if f93(v0, {
            let (h703, _) = v3.clone();
            h703
        }) {
            true
        } else {
            f94(v1, {
                let (h704, _) = v3.clone();
                h704
            })
        } {
            (f8(f35(f8({
                let (h705, _) = {
                    let (_, h706) = v3.clone();
                    h706
                };
                h705
            }, {
                let (_, h707) = {
                    let (_, h708) = v3;
                    h708
                };
                if h707 {
                    Str::lit(", ")
                } else {
                    Str::lit("<")
                }
            })?, {
                let (h709, _) = v3;
                h709
            })?, Str::lit(": Clone"))?, true)
        } else {
            let (_, h710) = v3;
            h710
        })))
    } else {
        Ok(None::<(u64, (Adt26, bool))>)
    }
}

pub fn f85(v0: (Option<Bytes>, List<Bytes>), v1: Bytes) -> (Option<Bytes>, List<Bytes>) {
    let m712 = {
        let (h711, _) = v0;
        h711
    };
    match m712 {
        None => {
            (Some(v1), {
                let (_, h713) = v0;
                h713
            })
        }
        Some(v2) => {
            (None::<Bytes>, List::cons({
                let a714 = v2;
                let a715 = v1;
                append_bytes(a714, a715)
            }, {
                let (_, h716) = v0;
                h716
            }))
        }
    }
}

pub fn f86(v0: u64, v1: u64) -> Bytes {
    f89(Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39]), {
        let a720 = {
            let a717 = v0;
            let a718 = v1;
            let a719 = 0u64;
            nat_quot(a717, a718, a719)
        };
        let a721 = 10u64;
        let a722 = 0u64;
        nat_rem(a720, a721, a722)
    }, 1u64)
}

pub fn f87(v0: Fn6, p727: u64, p728: u64) -> R<(u64, bool)> {
    let m726 = (p727, p728);
    {
        let mut m726 = m726;
        loop {
            let step = {
                let (v1, v2) = m726;
                let m723 = v1;
                if m723 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m723 - 1;
                    let m725 = {
                        let c724: Fn6 = v0.clone();
                        c724.apply(v2)?
                    };
                    match m725 {
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
                Ok(next) => { m726 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f88(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f95(v0, v1)
}

pub fn f89(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m732 = {
        let a729 = v0;
        let a730 = v1;
        let a731 = v2;
        slice_bytes(a729, a730, a731)
    };
    match m732 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f90(v0: Bytes, v1: (u64, Adt26)) -> R<Option<(u64, Adt26)>> {
    f96(v0, v1)
}

pub fn f91(v0: Bytes, v1: (u64, Adt26)) -> R<Option<(u64, Adt26)>> {
    f97(v0, v1)
}

pub fn f92(v0: u64, v1: (u64, Adt26)) -> R<Option<(u64, Adt26)>> {
    let a734 = {
        let (h733, _) = v1.clone();
        h733
    };
    let a735 = v0;
    if nat_lt(a734, a735) {
        Ok(Some(({
            let a737 = {
                let (h736, _) = v1;
                h736
            };
            let a738 = 1u64;
            nat_add(a737, a738)?
        }, f8({
            let (_, h739) = v1;
            h739
        }, Str::lit("    "))?)))
    } else {
        Ok(None::<(u64, Adt26)>)
    }
}

pub fn f93(p743: Adt13, v1: u64) -> bool {
    let m742 = p743;
    {
        let mut m742 = m742;
        loop {
            let step = {
                let v0 = m742;
                let m740 = v0;
                match m740 {
                    Adt13::C0 => {
                        Err::<Adt13, bool>(false)
                    }
                    Adt13::C1(_, v3, r741) => {
                        let v4 = (*r741).clone();
                        if f94(v3, v1) {
                            Err::<Adt13, bool>(true)
                        } else {
                            Ok::<Adt13, bool>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m742 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f94(v0: Adt2, v1: u64) -> bool {
    let m744 = v0;
    match m744 {
        Adt2::C0(_) => {
            false
        }
        Adt2::C1(_) => {
            false
        }
        Adt2::C2(_, r748) => {
            let v7 = (*r748).clone();
            f94(v7, v1)
        }
        Adt2::C3(_, r749, r750) => {
            let v9 = (*r749).clone();
            let v10 = (*r750).clone();
            if f94(v9, v1) {
                true
            } else {
                f94(v10, v1)
            }
        }
        Adt2::C4(v2, v3) => {
            let m745 = v2;
            match m745 {
                Adt7::C0 => {
                    false
                }
                Adt7::C1 => {
                    false
                }
                Adt7::C2 => {
                    let a746 = v3;
                    let a747 = v1;
                    nat_eq(a746, a747)
                }
            }
        }
        Adt2::C5 => {
            false
        }
        Adt2::C6(r751, r752) => {
            let v11 = (*r751).clone();
            let v12 = (*r752).clone();
            if f94(v11, v1) {
                true
            } else {
                f94(v12, v1)
            }
        }
    }
}

pub fn f95(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m753 = f98(v0, v1);
    match m753 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            let a754 = v2;
            let a755 = 48u8;
            if equal(a754, a755) {
                Ok(Some({
                    let a756 = v1;
                    let a757 = 1u64;
                    nat_add(a756, a757)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f96(v0: Bytes, v1: (u64, Adt26)) -> R<Option<(u64, Adt26)>> {
    let a760 = {
        let (h758, _) = v1.clone();
        h758
    };
    let a761 = {
        let a759 = v0.clone();
        length_bytes(a759)
    };
    if nat_lt(a760, a761) {
        let v2: (u64, u64) = f99(v0.clone(), {
            let (h762, _) = v1.clone();
            h762
        })?;
        let v3: u64 = {
            let (h763, _) = v2;
            h763
        };
        Ok(Some(({
            let a766 = {
                let (h764, _) = v1.clone();
                h764
            };
            let a767 = {
                let (_, h765) = v2;
                h765
            };
            nat_add(a766, a767)?
        }, {
            let a768 = v3;
            let a769 = 34u64;
            if nat_eq(a768, a769) {
                f8({
                    let (_, h770) = v1;
                    h770
                }, Str::lit("\\\""))?
            } else {
                let a771 = v3;
                let a772 = 92u64;
                if nat_eq(a771, a772) {
                    f8({
                        let (_, h773) = v1;
                        h773
                    }, Str::lit("\\\\"))?
                } else {
                    let a774 = 32u64;
                    let a775 = v3;
                    if if nat_le(a774, a775) {
                        let a776 = v3;
                        let a777 = 126u64;
                        nat_le(a776, a777)
                    } else {
                        false
                    } {
                        f3({
                            let (_, h778) = v1;
                            h778
                        }, f89(v0, {
                            let (h779, _) = v1;
                            h779
                        }, 1u64))?
                    } else {
                        f8(f100(f8({
                            let (_, h780) = v1;
                            h780
                        }, Str::lit("\\u{"))?, v3)?, Str::lit("}"))?
                    }
                }
            }
        })))
    } else {
        Ok(None::<(u64, Adt26)>)
    }
}

pub fn f97(v0: Bytes, v1: (u64, Adt26)) -> R<Option<(u64, Adt26)>> {
    let m782 = f98(v0, {
        let (h781, _) = v1.clone();
        h781
    });
    match m782 {
        None => {
            Ok(None::<(u64, Adt26)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a784 = {
                    let (h783, _) = v1.clone();
                    h783
                };
                let a785 = 1u64;
                nat_add(a784, a785)?
            }, f101(f8({
                let a787 = {
                    let (h786, _) = v1;
                    h786
                };
                let a788 = 0u64;
                if nat_eq(a787, a788) {
                    let (_, h789) = v1;
                    h789
                } else {
                    f8({
                        let (_, h790) = v1;
                        h790
                    }, Str::lit(", "))?
                }
            }, Str::lit("0x"))?, v2)?)))
        }
    }
}

pub fn f98(v0: Bytes, v1: u64) -> Option<u8> {
    let a791 = v0;
    let a792 = v1;
    index_bytes(a791, a792)
}

pub fn f99(v0: Bytes, v1: u64) -> R<(u64, u64)> {
    let m793 = f98(v0.clone(), v1);
    match m793 {
        None => {
            Ok((0u64, 1u64))
        }
        Some(v2) => {
            let v4: u64 = {
                let v3: u8 = v2;
                let a838 = {
                    let a814 = {
                        let a802 = {
                            let a796 = {
                                let a794 = v3;
                                let a795 = 128u8;
                                fixed_u8::bit_and(a794, a795)
                            };
                            let a797 = 128u8;
                            if equal(a796, a797) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a803 = {
                            let a800 = {
                                let a798 = v3;
                                let a799 = 64u8;
                                fixed_u8::bit_and(a798, a799)
                            };
                            let a801 = 64u8;
                            if equal(a800, a801) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a802, a803)?
                    };
                    let a815 = {
                        let a812 = {
                            let a806 = {
                                let a804 = v3;
                                let a805 = 32u8;
                                fixed_u8::bit_and(a804, a805)
                            };
                            let a807 = 32u8;
                            if equal(a806, a807) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a813 = {
                            let a810 = {
                                let a808 = v3;
                                let a809 = 16u8;
                                fixed_u8::bit_and(a808, a809)
                            };
                            let a811 = 16u8;
                            if equal(a810, a811) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a812, a813)?
                    };
                    nat_add(a814, a815)?
                };
                let a839 = {
                    let a836 = {
                        let a824 = {
                            let a818 = {
                                let a816 = v3;
                                let a817 = 8u8;
                                fixed_u8::bit_and(a816, a817)
                            };
                            let a819 = 8u8;
                            if equal(a818, a819) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a825 = {
                            let a822 = {
                                let a820 = v3;
                                let a821 = 4u8;
                                fixed_u8::bit_and(a820, a821)
                            };
                            let a823 = 4u8;
                            if equal(a822, a823) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a824, a825)?
                    };
                    let a837 = {
                        let a834 = {
                            let a828 = {
                                let a826 = v3;
                                let a827 = 2u8;
                                fixed_u8::bit_and(a826, a827)
                            };
                            let a829 = 2u8;
                            if equal(a828, a829) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a835 = {
                            let a832 = {
                                let a830 = v3;
                                let a831 = 1u8;
                                fixed_u8::bit_and(a830, a831)
                            };
                            let a833 = 1u8;
                            if equal(a832, a833) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a834, a835)?
                    };
                    nat_add(a836, a837)?
                };
                nat_add(a838, a839)?
            };
            let a840 = v4;
            let a841 = 128u64;
            if nat_lt(a840, a841) {
                Ok((v4, 1u64))
            } else {
                let a842 = v4;
                let a843 = 224u64;
                if nat_lt(a842, a843) {
                    Ok(({
                        let a851 = {
                            let a847 = {
                                let a844 = v4;
                                let a845 = 32u64;
                                let a846 = 0u64;
                                nat_rem(a844, a845, a846)
                            };
                            let a848 = 64u64;
                            nat_mul(a847, a848)?
                        };
                        let a852 = f102(v0, {
                            let a849 = v1;
                            let a850 = 1u64;
                            nat_add(a849, a850)?
                        })?;
                        nat_add(a851, a852)?
                    }, 2u64))
                } else {
                    let a853 = v4;
                    let a854 = 240u64;
                    if nat_lt(a853, a854) {
                        Ok(({
                            let a868 = {
                                let a858 = {
                                    let a855 = v4;
                                    let a856 = 16u64;
                                    let a857 = 0u64;
                                    nat_rem(a855, a856, a857)
                                };
                                let a859 = 4096u64;
                                nat_mul(a858, a859)?
                            };
                            let a869 = {
                                let a866 = {
                                    let a862 = f102(v0.clone(), {
                                        let a860 = v1;
                                        let a861 = 1u64;
                                        nat_add(a860, a861)?
                                    })?;
                                    let a863 = 64u64;
                                    nat_mul(a862, a863)?
                                };
                                let a867 = f102(v0, {
                                    let a864 = v1;
                                    let a865 = 2u64;
                                    nat_add(a864, a865)?
                                })?;
                                nat_add(a866, a867)?
                            };
                            nat_add(a868, a869)?
                        }, 3u64))
                    } else {
                        Ok(({
                            let a889 = {
                                let a873 = {
                                    let a870 = v4;
                                    let a871 = 8u64;
                                    let a872 = 0u64;
                                    nat_rem(a870, a871, a872)
                                };
                                let a874 = 262144u64;
                                nat_mul(a873, a874)?
                            };
                            let a890 = {
                                let a887 = {
                                    let a877 = f102(v0.clone(), {
                                        let a875 = v1;
                                        let a876 = 1u64;
                                        nat_add(a875, a876)?
                                    })?;
                                    let a878 = 4096u64;
                                    nat_mul(a877, a878)?
                                };
                                let a888 = {
                                    let a885 = {
                                        let a881 = f102(v0.clone(), {
                                            let a879 = v1;
                                            let a880 = 2u64;
                                            nat_add(a879, a880)?
                                        })?;
                                        let a882 = 64u64;
                                        nat_mul(a881, a882)?
                                    };
                                    let a886 = f102(v0, {
                                        let a883 = v1;
                                        let a884 = 3u64;
                                        nat_add(a883, a884)?
                                    })?;
                                    nat_add(a885, a886)?
                                };
                                nat_add(a887, a888)?
                            };
                            nat_add(a889, a890)?
                        }, 4u64))
                    }
                }
            }
        }
    }
}

pub fn f100(v0: Adt26, v1: u64) -> R<Adt26> {
    let v2: Bytes = {
        let a903 = f103(v1, 268435456u64);
        let a904 = {
            let a901 = f103(v1, 16777216u64);
            let a902 = {
                let a899 = f103(v1, 1048576u64);
                let a900 = {
                    let a897 = f103(v1, 65536u64);
                    let a898 = {
                        let a895 = f103(v1, 4096u64);
                        let a896 = {
                            let a893 = f103(v1, 256u64);
                            let a894 = {
                                let a891 = f103(v1, 16u64);
                                let a892 = f103(v1, 1u64);
                                append_bytes(a891, a892)
                            };
                            append_bytes(a893, a894)
                        };
                        append_bytes(a895, a896)
                    };
                    append_bytes(a897, a898)
                };
                append_bytes(a899, a900)
            };
            append_bytes(a901, a902)
        };
        append_bytes(a903, a904)
    };
    let v3: u64 = {
        let (h905, _) = f87(Fn6::F104(v2.clone()), 7u64, 0u64)?;
        h905
    };
    f3(v0, f89(v2, v3, {
        let a906 = 8u64;
        let a907 = v3;
        nat_sub(a906, a907)
    }))
}

pub fn f101(v0: Adt26, v1: u8) -> R<Adt26> {
    let v3: u64 = {
        let v2: u8 = v1;
        let a952 = {
            let a928 = {
                let a916 = {
                    let a910 = {
                        let a908 = v2;
                        let a909 = 128u8;
                        fixed_u8::bit_and(a908, a909)
                    };
                    let a911 = 128u8;
                    if equal(a910, a911) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a917 = {
                    let a914 = {
                        let a912 = v2;
                        let a913 = 64u8;
                        fixed_u8::bit_and(a912, a913)
                    };
                    let a915 = 64u8;
                    if equal(a914, a915) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a916, a917)?
            };
            let a929 = {
                let a926 = {
                    let a920 = {
                        let a918 = v2;
                        let a919 = 32u8;
                        fixed_u8::bit_and(a918, a919)
                    };
                    let a921 = 32u8;
                    if equal(a920, a921) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a927 = {
                    let a924 = {
                        let a922 = v2;
                        let a923 = 16u8;
                        fixed_u8::bit_and(a922, a923)
                    };
                    let a925 = 16u8;
                    if equal(a924, a925) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a926, a927)?
            };
            nat_add(a928, a929)?
        };
        let a953 = {
            let a950 = {
                let a938 = {
                    let a932 = {
                        let a930 = v2;
                        let a931 = 8u8;
                        fixed_u8::bit_and(a930, a931)
                    };
                    let a933 = 8u8;
                    if equal(a932, a933) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a939 = {
                    let a936 = {
                        let a934 = v2;
                        let a935 = 4u8;
                        fixed_u8::bit_and(a934, a935)
                    };
                    let a937 = 4u8;
                    if equal(a936, a937) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a938, a939)?
            };
            let a951 = {
                let a948 = {
                    let a942 = {
                        let a940 = v2;
                        let a941 = 2u8;
                        fixed_u8::bit_and(a940, a941)
                    };
                    let a943 = 2u8;
                    if equal(a942, a943) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a949 = {
                    let a946 = {
                        let a944 = v2;
                        let a945 = 1u8;
                        fixed_u8::bit_and(a944, a945)
                    };
                    let a947 = 1u8;
                    if equal(a946, a947) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a948, a949)?
            };
            nat_add(a950, a951)?
        };
        nat_add(a952, a953)?
    };
    f3(v0, {
        let a954 = f103(v3, 16u64);
        let a955 = f103(v3, 1u64);
        append_bytes(a954, a955)
    })
}

pub fn f102(v0: Bytes, v1: u64) -> R<u64> {
    let m956 = f98(v0, v1);
    match m956 {
        None => {
            Ok(0u64)
        }
        Some(v2) => {
            let a1003 = {
                let v3: u8 = v2;
                let a1001 = {
                    let a977 = {
                        let a965 = {
                            let a959 = {
                                let a957 = v3;
                                let a958 = 128u8;
                                fixed_u8::bit_and(a957, a958)
                            };
                            let a960 = 128u8;
                            if equal(a959, a960) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a966 = {
                            let a963 = {
                                let a961 = v3;
                                let a962 = 64u8;
                                fixed_u8::bit_and(a961, a962)
                            };
                            let a964 = 64u8;
                            if equal(a963, a964) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a965, a966)?
                    };
                    let a978 = {
                        let a975 = {
                            let a969 = {
                                let a967 = v3;
                                let a968 = 32u8;
                                fixed_u8::bit_and(a967, a968)
                            };
                            let a970 = 32u8;
                            if equal(a969, a970) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a976 = {
                            let a973 = {
                                let a971 = v3;
                                let a972 = 16u8;
                                fixed_u8::bit_and(a971, a972)
                            };
                            let a974 = 16u8;
                            if equal(a973, a974) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a975, a976)?
                    };
                    nat_add(a977, a978)?
                };
                let a1002 = {
                    let a999 = {
                        let a987 = {
                            let a981 = {
                                let a979 = v3;
                                let a980 = 8u8;
                                fixed_u8::bit_and(a979, a980)
                            };
                            let a982 = 8u8;
                            if equal(a981, a982) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a988 = {
                            let a985 = {
                                let a983 = v3;
                                let a984 = 4u8;
                                fixed_u8::bit_and(a983, a984)
                            };
                            let a986 = 4u8;
                            if equal(a985, a986) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a987, a988)?
                    };
                    let a1000 = {
                        let a997 = {
                            let a991 = {
                                let a989 = v3;
                                let a990 = 2u8;
                                fixed_u8::bit_and(a989, a990)
                            };
                            let a992 = 2u8;
                            if equal(a991, a992) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a998 = {
                            let a995 = {
                                let a993 = v3;
                                let a994 = 1u8;
                                fixed_u8::bit_and(a993, a994)
                            };
                            let a996 = 1u8;
                            if equal(a995, a996) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a997, a998)?
                    };
                    nat_add(a999, a1000)?
                };
                nat_add(a1001, a1002)?
            };
            let a1004 = 64u64;
            let a1005 = 0u64;
            Ok(nat_rem(a1003, a1004, a1005))
        }
    }
}

pub fn f103(v0: u64, v1: u64) -> Bytes {
    f89(Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66]), {
        let a1009 = {
            let a1006 = v0;
            let a1007 = v1;
            let a1008 = 0u64;
            nat_quot(a1006, a1007, a1008)
        };
        let a1010 = 16u64;
        let a1011 = 0u64;
        nat_rem(a1009, a1010, a1011)
    }, 1u64)
}

pub fn f104(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f105(v0, v1)
}

pub fn f105(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m1012 = f98(v0, v1);
    match m1012 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            let a1013 = v2;
            let a1014 = 48u8;
            if equal(a1013, a1014) {
                Ok(Some({
                    let a1015 = v1;
                    let a1016 = 1u64;
                    nat_add(a1015, a1016)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn run(p0: u64, p1: Bytes, p2: Adt0) -> R<Result<Bytes, Bytes>> {
    f0(p0, p1, p2)
}
