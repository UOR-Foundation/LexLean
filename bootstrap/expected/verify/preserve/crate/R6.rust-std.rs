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

#[derive(Clone)]
pub struct Str(std::rc::Rc<str>);
impl Str {
    pub fn lit(text: &str) -> Str { Str(std::rc::Rc::from(text)) }
    pub fn text(&self) -> &str { &self.0 }
    pub fn length(&self) -> u64 { chars(&self.0) }
    pub fn utf8_encode(&self) -> Bytes { tick(self.0.len() as u64); Bytes::lit(self.0.as_bytes()) }
    pub fn split_exact(&self, delimiter: &Str, maximum: u32) -> Option<List<Str>> {
        if delimiter.0.is_empty() { return None; }
        chars(&self.0);
        let fields: Vec<Str> = self.0.split(&*delimiter.0).map(Str::lit).collect();
        if fields.len() as u128 > u128::from(maximum) { return None; }
        Some(List::onto(fields, List::nil()))
    }
    pub fn join(texts: List<Str>, delimiter: &Str) -> Str {
        let mut out = String::new();
        let mut first = true;
        texts.walk(&mut |text| {
            if !first { chars(&delimiter.0); out.push_str(&delimiter.0); }
            first = false;
            chars(&text.0);
            out.push_str(&text.0);
            true
        });
        Str::lit(&out)
    }
}
#[derive(Clone)]
pub struct Bytes(std::rc::Rc<[u8]>);
impl Bytes {
    pub fn lit(octets: &[u8]) -> Bytes { Bytes(std::rc::Rc::from(octets)) }
    pub fn octets(&self) -> &[u8] { &self.0 }
    pub fn append(a: Bytes, b: Bytes) -> Bytes {
        tick((a.0.len() + b.0.len()) as u64);
        let mut out = Vec::with_capacity(a.0.len() + b.0.len());
        out.extend_from_slice(&a.0);
        out.extend_from_slice(&b.0);
        Bytes(std::rc::Rc::from(out))
    }
    pub fn length(&self) -> u64 { self.0.len() as u64 }
    pub fn index(&self, i: u64) -> Option<u8> { usize::try_from(i).ok().and_then(|i| self.0.get(i)).copied() }
    pub fn slice(&self, start: u64, count: u64) -> Option<Bytes> {
        let end = u128::from(start) + u128::from(count);
        if end > self.0.len() as u128 { return None; }
        tick(count);
        Some(Bytes::lit(&self.0[start as usize..end as usize]))
    }
    pub fn utf8_decode(&self) -> Option<Str> { tick(self.0.len() as u64); core::str::from_utf8(&self.0).ok().map(Str::lit) }
    pub fn compare(&self, other: &Bytes) -> Ordering { tick(1 + self.0.len().min(other.0.len()) as u64); self.0.cmp(&other.0) }
}

pub struct Node<T> { head: T, tail: List<T> }
/// A list is a run of cells laid out contiguously, from the newest at the end of a buffer
/// to the oldest at its start, or a cell in front of another list. A `cons` onto a run that
/// owns its buffer alone pushes in place; a `cons` onto any other list is one cell, so
/// building and taking apart a cell is constant work whatever the sharing, and no buffer is
/// written while another list can see it.
pub enum Repr<T> { Nil, Run { buffer: std::rc::Rc<Vec<T>>, len: usize }, Cell(std::rc::Rc<Node<T>>) }
pub struct List<T>(Repr<T>);
impl<T> Clone for List<T> {
    fn clone(&self) -> Self {
        List(match &self.0 {
            Repr::Nil => Repr::Nil,
            Repr::Run { buffer, len } => Repr::Run { buffer: buffer.clone(), len: *len },
            Repr::Cell(cell) => Repr::Cell(cell.clone()),
        })
    }
}
impl<T> List<T> {
    fn take(&mut self) -> Repr<T> { std::mem::replace(&mut self.0, Repr::Nil) }
}
impl<T> Drop for Node<T> {
    fn drop(&mut self) {
        let mut next = match self.tail.take() { Repr::Cell(cell) => Some(cell), _ => None };
        while let Some(cell) = next { // release
            match std::rc::Rc::try_unwrap(cell) {
                Ok(mut node) => next = match node.tail.take() { Repr::Cell(cell) => Some(cell), _ => None },
                Err(_) => break,
            }
        }
    }
}
impl<T: Clone> List<T> {
    pub fn nil() -> Self { List(Repr::Nil) }
    pub fn cons(head: T, mut tail: List<T>) -> Self {
        List(match tail.take() {
            Repr::Nil => {
                let mut buffer = Vec::with_capacity(16);
                buffer.push(head);
                Repr::Run { buffer: std::rc::Rc::new(buffer), len: 1 }
            }
            Repr::Run { mut buffer, len } => {
                if let Some(cells) = std::rc::Rc::get_mut(&mut buffer) {
                    cells.truncate(len);
                    cells.push(head);
                    Repr::Run { buffer, len: len + 1 }
                } else {
                    Repr::Cell(std::rc::Rc::new(Node { head, tail: List(Repr::Run { buffer, len }) }))
                }
            }
            rest => Repr::Cell(std::rc::Rc::new(Node { head, tail: List(rest) })),
        })
    }
    pub fn uncons(&self) -> Option<(T, List<T>)> {
        match &self.0 {
            Repr::Nil => None,
            Repr::Run { buffer, len } => {
                let head = buffer[len - 1].clone();
                Some((head, List(if *len == 1 { Repr::Nil } else { Repr::Run { buffer: buffer.clone(), len: len - 1 } })))
            }
            Repr::Cell(cell) => Some((cell.head.clone(), cell.tail.clone())),
        }
    }
    pub fn length(&self) -> u64 {
        let mut count = 0;
        self.walk(&mut |_| { count += 1; true });
        count
    }
    pub fn append(a: List<T>, b: List<T>) -> List<T> { List::onto(a.items(), b) }
    pub fn index(&self, i: u64) -> Option<T> {
        let mut position = 0;
        let mut found = None;
        self.walk(&mut |item| { if position == i { found = Some(item.clone()); return false; } position += 1; true });
        found
    }
    pub fn slice(&self, start: u64, count: u64) -> Option<List<T>> {
        let end = u128::from(start) + u128::from(count);
        let mut part = Vec::new();
        let mut position: u128 = 0;
        self.walk(&mut |item| {
            if position >= end { return false; }
            if position >= u128::from(start) { part.push(item.clone()); }
            position += 1;
            true
        });
        if position < end { None } else { Some(List::onto(part, List::nil())) }
    }
    /// Walk the items in order, ticking one per item, until `each` says to stop: the cells
    /// in front, then the run they end in.
    fn walk(&self, each: &mut dyn FnMut(&T) -> bool) {
        let mut cursor = self;
        while let Repr::Cell(cell) = &cursor.0 { tick(1); if !each(&cell.head) { return; } cursor = &cell.tail; }
        if let Repr::Run { buffer, len } = &cursor.0 {
            for item in buffer[..*len].iter().rev() { tick(1); if !each(item) { return; } }
        }
    }
    fn items(&self) -> Vec<T> {
        let mut out = Vec::new();
        self.walk(&mut |item| { out.push(item.clone()); true });
        out
    }
    fn onto(items: Vec<T>, tail: List<T>) -> List<T> {
        let mut out = tail;
        for item in items.into_iter().rev() { tick(1); out = List::cons(item, out); }
        out
    }
}

fn chars(text: &str) -> u64 {
    let mut count = 0;
    for _ in text.chars() { tick(1); count += 1; }
    count
}

impl Same for Str {
    fn same(&self, other: &Self) -> bool {
        let mut left = self.0.chars();
        let mut right = other.0.chars();
        loop {
            tick(1);
            match (left.next(), right.next()) { (None, None) => return true, (a, b) if a != b => return false, _ => {} }
        }
    }
}
impl Same for Bytes {
    fn same(&self, other: &Self) -> bool { tick(1 + self.0.len().min(other.0.len()) as u64); self.0 == other.0 }
}
impl Key for Str {
    fn key(&self, other: &Self) -> Ordering {
        let mut left = self.0.chars();
        let mut right = other.0.chars();
        loop {
            tick(1);
            match (left.next(), right.next()) {
                (None, None) => return Ordering::Equal,
                (None, Some(_)) => return Ordering::Less,
                (Some(_), None) => return Ordering::Greater,
                (Some(a), Some(b)) => match a.cmp(&b) { Ordering::Equal => {} decided => return decided },
            }
        }
    }
}

fn canonical_decimal(text: &str) -> Option<Option<i128>> {
    let digits = text.strip_prefix('-').unwrap_or(text);
    let mut canonical = !digits.is_empty() && (digits == "0" || !digits.starts_with('0')) && text != "-0";
    for c in digits.chars() { tick(1); canonical = canonical && c.is_ascii_digit(); }
    if !canonical { return None; }
    Some(text.parse::<i128>().ok())
}
pub fn parse_int(text: Str) -> R<Option<i64>> {
    match canonical_decimal(&text.0) {
        None => Ok(None),
        Some(Some(n)) => i64::try_from(n).map(Some).map_err(|_| Overflow),
        Some(None) => Err(Overflow),
    }
}
macro_rules! decimal {
    ($t:ident, $format:ident, $parse:ident) => {
        pub fn $format(a: $t) -> Str { let text = a.to_string(); tick(text.len() as u64); Str::lit(&text) }
        pub fn $parse(text: Str) -> Option<$t> {
            match canonical_decimal(&text.0) { Some(Some(n)) => $t::try_from(n).ok(), _ => None }
        }
    };
}
decimal!(u8, format_u8, parse_u8); decimal!(u16, format_u16, parse_u16);
decimal!(u32, format_u32, parse_u32); decimal!(u64, format_u64, parse_u64);
decimal!(i8, format_i8, parse_i8); decimal!(i16, format_i16, parse_i16);
decimal!(i32, format_i32, parse_i32); decimal!(i64, format_i64, parse_i64);

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
    F74(List<Adt8>, Bytes),
}

impl Fn2 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Adt5) -> R<Option<Bytes>> {
        match self {
            Fn2::F16(k0) => f16(k0.clone(), p0, p1),
            Fn2::F74(k0, k1) => f74(k0.clone(), k1.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F20(List<Adt7>, Adt14, Adt2),
    F75(List<Adt7>, Adt14, List<Adt5>, Bytes, Adt2),
}

impl Fn3 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Adt8) -> R<Option<Bytes>> {
        match self {
            Fn3::F20(k0, k1, k2) => f20(k0.clone(), k1.clone(), k2.clone(), p0, p1),
            Fn3::F75(k0, k1, k2, k3, k4) => f75(k0.clone(), k1.clone(), k2.clone(), k3.clone(), k4.clone(), p0, p1),
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
    F69,
}

impl Fn5 {
    pub fn apply(&self, p0: Adt17, p1: Adt5) -> R<Adt17> {
        match self {
            Fn5::F69 => f69(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F72(Adt14),
}

impl Fn6 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Adt7) -> R<Option<Bytes>> {
        match self {
            Fn6::F72(k0) => f72(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F78,
}

impl Fn7 {
    pub fn apply(&self, p0: Bytes, p1: Bytes) -> Bytes {
        match self {
            Fn7::F78 => f78(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F81(Str),
    F82(Str),
    F85(Str),
    F121(Adt7, Adt14),
    F131(Bytes),
    F168(List<Bytes>, List<Bytes>),
}

impl Fn8 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Bytes) -> R<Option<Bytes>> {
        match self {
            Fn8::F81(k0) => f81(k0.clone(), p0, p1),
            Fn8::F82(k0) => f82(k0.clone(), p0, p1),
            Fn8::F85(k0) => Ok(f85(k0.clone(), p0, p1)),
            Fn8::F121(k0, k1) => Ok(f121(k0.clone(), k1.clone(), p0, p1)),
            Fn8::F131(k0) => Ok(f131(k0.clone(), p0, p1)),
            Fn8::F168(k0, k1) => f168(k0.clone(), k1.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F87,
}

impl Fn9 {
    pub fn apply(&self, p0: Adt14, p1: Adt5) -> Adt14 {
        match self {
            Fn9::F87 => f87(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F89,
}

impl Fn10 {
    pub fn apply(&self, p0: Adt14, p1: Adt8) -> Adt14 {
        match self {
            Fn10::F89 => f89(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F92(Bytes),
    F93,
}

impl Fn11 {
    pub fn apply(&self, p0: Adt17, p1: Adt9) -> R<Adt17> {
        match self {
            Fn11::F92(k0) => f92(k0.clone(), p0, p1),
            Fn11::F93 => f93(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F100(Bytes),
}

impl Fn12 {
    pub fn apply(&self, p0: Option<Adt16>, p1: Adt16) -> Option<Adt16> {
        match self {
            Fn12::F100(k0) => f100(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F102(Bytes),
    F119(List<Bytes>, List<Bytes>),
    F195(Bytes, Adt14),
}

impl Fn13 {
    pub fn apply(&self, p0: bool, p1: Bytes) -> R<bool> {
        match self {
            Fn13::F102(k0) => Ok(f102(k0.clone(), p0, p1)),
            Fn13::F119(k0, k1) => f119(k0.clone(), k1.clone(), p0, p1),
            Fn13::F195(k0, k1) => Ok(f195(k0.clone(), k1.clone(), p0, p1)),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F130(Bytes),
}

impl Fn14 {
    pub fn apply(&self, p0: Result<List<Adt16>, Bytes>, p1: Adt6) -> Result<List<Adt16>, Bytes> {
        match self {
            Fn14::F130(k0) => f130(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F133(Bytes),
    F184(Bytes),
}

impl Fn15 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn15::F133(k0) => f133(k0.clone(), p0),
            Fn15::F184(k0) => f184(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F139(Bytes),
}

impl Fn16 {
    pub fn apply(&self, p0: Option<Adt11>, p1: Adt11) -> Option<Adt11> {
        match self {
            Fn16::F139(k0) => f139(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F141(Bytes),
}

impl Fn17 {
    pub fn apply(&self, p0: Option<Adt12>, p1: Adt12) -> Option<Adt12> {
        match self {
            Fn17::F141(k0) => f141(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F143(Bytes),
    F190(Bytes),
}

impl Fn18 {
    pub fn apply(&self, p0: Option<Adt13>, p1: Adt13) -> Option<Adt13> {
        match self {
            Fn18::F143(k0) => f143(k0.clone(), p0, p1),
            Fn18::F190(k0) => f190(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F155(Bytes),
}

impl Fn19 {
    pub fn apply(&self, p0: List<u64>, p1: Adt13) -> List<u64> {
        match self {
            Fn19::F155(k0) => f155(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn20 {
    F157(u64),
}

impl Fn20 {
    pub fn apply(&self, p0: bool, p1: u64) -> bool {
        match self {
            Fn20::F157(k0) => f157(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn21 {
    F163,
}

impl Fn21 {
    pub fn apply(&self, p0: Bytes, p1: u64) -> R<Bytes> {
        match self {
            Fn21::F163 => f163(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn22 {
    F171(bool, u64, Adt14),
}

impl Fn22 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Adt6) -> R<Option<Bytes>> {
        match self {
            Fn22::F171(k0, k1, k2) => f171(*k0, *k1, k2.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn23 {
    F173(Bytes),
}

impl Fn23 {
    pub fn apply(&self, p0: Option<Adt10>, p1: Adt10) -> Option<Adt10> {
        match self {
            Fn23::F173(k0) => f173(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn24 {
    F175(Bytes),
}

impl Fn24 {
    pub fn apply(&self, p0: u64, p1: Adt8) -> R<u64> {
        match self {
            Fn24::F175(k0) => f175(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn25 {
    F177(Bytes),
}

impl Fn25 {
    pub fn apply(&self, p0: Option<Adt8>, p1: Adt8) -> Option<Adt8> {
        match self {
            Fn25::F177(k0) => f177(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn26 {
    F179,
}

impl Fn26 {
    pub fn apply(&self, p0: Bytes, p1: Adt6) -> R<Bytes> {
        match self {
            Fn26::F179 => f179(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn27 {
    F182(Bytes),
    F202(Bytes),
}

impl Fn27 {
    pub fn apply(&self, p0: bool, p1: Adt5) -> bool {
        match self {
            Fn27::F182(k0) => f182(k0.clone(), p0, p1),
            Fn27::F202(k0) => f202(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn28 {
    F187(Bytes),
}

impl Fn28 {
    pub fn apply(&self, p0: Option<Adt9>, p1: Adt9) -> Option<Adt9> {
        match self {
            Fn28::F187(k0) => f187(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn29 {
    F198(Bytes),
}

impl Fn29 {
    pub fn apply(&self, p0: u64, p1: Bytes) -> R<u64> {
        match self {
            Fn29::F198(k0) => f198(k0.clone(), p0, p1),
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
        Adt1::C5(_, _, _, v29) => {
            Ok(Ok::<Adt14, Bytes>(f36(v0, v29)))
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
                            let a142 = List::cons(Adt11::C0(v33, f37(v36, 0u64)?, {
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
                                let a153 = f38(v43.clone())?;
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
                                    Err::<Adt14, Bytes>(f12(f13(Str::lit("runtime support: struct-fields: ")), f12(f14(v39), f12(f13(Str::lit(" has ")), f12(f39({
                                        let a167 = v42;
                                        length_list(a167)
                                    })?, f12(f13(Str::lit(" field names and ")), f12(f39(f38(v43)?)?, f13(Str::lit(" types")))))))))
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
                        let m169 = f28(f30(Str::lit("an enum generic"), v48.clone())?, f28(f34(Str::lit("an enum"), v49)?, f40(v47.clone(), v50.clone())?));
                        match m169 {
                            None => {
                                let m170 = f41(v50.clone())?;
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
                Ok(Some(f12(f13(Str::lit("runtime support: type-parameter: ")), f12(f13(Str::lit("parameter ")), f12(f39(v4)?, f12(f13(Str::lit(" of ")), f12(f39(v2)?, f13(Str::lit(" in scope")))))))))
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
            let m188 = f42(v0.clone(), v10.clone());
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
                        f43(v0.clone(), v10.clone())
                    } else {
                        false
                    } {
                        Ok(Some(f12(f13(Str::lit("runtime support: hidden-allocation: ")), f12(f13(Str::lit("rust-core renders ")), f12(f14(v10), f13(Str::lit(", which requires heap allocation")))))))
                    } else {
                        let a190 = v12;
                        let a191 = f38(v11.clone())?;
                        if nat_eq(a190, a191) {
                            f8(v0, v11, v2, v3)
                        } else {
                            Ok(Some(f12(f13(Str::lit("runtime support: type-arity: ")), f12(f14(v10), f12(f13(Str::lit(" takes ")), f12(f39(v12)?, f12(f13(Str::lit(" arguments, given ")), f39(f38(v11)?)?)))))))
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
            let m203 = f44(match v1 {
                Adt15::C0(_, h202, _, _) => {
                    h202
                }
            }, v3.clone());
            match m203 {
                None => {
                    if f45(match v0 {
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
            f46(v5)
        }
        Adt3::C2(v6) => {
            f47(v0, v6)
        }
        Adt3::C3(v7, r249) => {
            let v8 = (*r249).clone();
            let m205 = f48(v0.clone(), v7, f49(v8.clone())?)?;
            match m205 {
                None => {
                    f50(v0, v1, v8)
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
                    let m207 = f51(v0.clone(), v1.clone(), v10, v11, f49(v12.clone())?)?;
                    match m207 {
                        None => {
                            f50(v0, v1, v12)
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
            let m210 = f28(f52(v24.clone())?, f9(v0.clone(), v1.clone(), v24)?);
            match m210 {
                None => {
                    if f53(v25.clone()) {
                        Ok(Some(f12(f13(Str::lit("runtime support: match-arms: ")), f13(Str::lit("a match with no arm")))))
                    } else {
                        f54(v0, v1, v25)
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
            let m211 = f55(v1.clone(), v30.clone());
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
            let m213 = f28(f52(v34.clone())?, f9(v0.clone(), v1.clone(), v34)?);
            match m213 {
                None => {
                    let m214 = f56(v0.clone(), v1.clone(), v35)?;
                    match m214 {
                        None => {
                            f56(v0, v1, v36)
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
            let m215 = f28(f52(v39.clone())?, f9(v0.clone(), v1.clone(), v39)?);
            match m215 {
                None => {
                    f56(v0, f57(v1, List::<Adt16>::nil(), true), v40)
                }
                Some(v41) => {
                    Ok(Some(v41))
                }
            }
        }
        Adt3::C14(v42, r266, r267) => {
            let v43 = (*r266).clone();
            let v44 = (*r267).clone();
            let m216 = f28(f52(v43.clone())?, f9(v0.clone(), v1.clone(), v43)?);
            match m216 {
                None => {
                    let m217 = f58(v0.clone(), v42.clone())?;
                    match m217 {
                        None => {
                            let m218 = f59(List::<Adt16>::nil(), v42)?;
                            match m218 {
                                Ok(v48) => {
                                    f56(v0, f57(v1, v48, true), v44)
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
            f56(v0, f57(v1, List::<Adt16>::nil(), true), v49)
        }
        Adt3::C16(v50, r269, r270) => {
            let v51 = (*r269).clone();
            let v52 = (*r270).clone();
            let m219 = f28(f52(v51.clone())?, f9(v0.clone(), v1.clone(), v51)?);
            match m219 {
                None => {
                    let m220 = f58(v0.clone(), v50.clone())?;
                    match m220 {
                        None => {
                            let m221 = f59(List::<Adt16>::nil(), v50)?;
                            match m221 {
                                Ok(v56) => {
                                    f56(v0, f57(v1, v56, true), v52)
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
            if f60(v57.clone()) {
                Ok(None::<Bytes>)
            } else {
                f9(v0, v1, v57)
            }
        }
        Adt3::C19(r272) => {
            let v58 = (*r272).clone();
            let a223 = f49(v58.clone())?;
            let a224 = 2u64;
            if nat_lt(a223, a224) {
                Ok(Some(f12(f13(Str::lit("runtime support: tuple-arity: ")), f12(f13(Str::lit("a tuple of ")), f39(f49(v58)?)?))))
            } else {
                f50(v0, v1, v58)
            }
        }
        Adt3::C20(v59, v60, r273) => {
            let v61 = (*r273).clone();
            let m225 = f47(v0.clone(), v59.clone())?;
            match m225 {
                None => {
                    let m226 = f61(v0.clone(), v59.clone());
                    match m226 {
                        None => {
                            Ok(Some(f12(f13(Str::lit("runtime support: struct-fields: ")), f12(f14(f27(v59, Str::lit("::"))?), f13(Str::lit(" is no record struct or record variant"))))))
                        }
                        Some(v63) => {
                            if f62(v60.clone(), v63.clone(), f49(v61.clone())?)? {
                                f50(v0, v1, v61)
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
                let a231 = f63(match v1.clone() {
                    Adt15::C0(_, _, h230, _) => {
                        h230
                    }
                });
                bool_not(a231)
            })?, f64(Str::lit("a cast type"), v70));
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
                    let m235 = if f60(v76.clone()) {
                        None::<Bytes>
                    } else {
                        f9(v0.clone(), v1.clone(), v76)?
                    };
                    match m235 {
                        None => {
                            if f60(v77.clone()) {
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
                    let m237 = f65(v83.clone());
                    match m237 {
                        None => {
                            Ok(Some(f12(f13(Str::lit("runtime support: method-name: ")), f12(f14(v83), f13(Str::lit(" takes no type arguments in the closed table"))))))
                        }
                        Some(v87) => {
                            let a238 = v87;
                            let a239 = f38(v84.clone())?;
                            if if nat_eq(a238, a239) {
                                let a240 = f66(v83.clone());
                                let a241 = f49(v85.clone())?;
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
                                        let a246 = f63(match v1.clone() {
                                            Adt15::C0(_, _, h245, _) => {
                                                h245
                                            }
                                        });
                                        bool_not(a246)
                                    })?, f64(Str::lit("a type argument"), v84));
                                    match m247 {
                                        None => {
                                            f50(v0, v1, v85)
                                        }
                                        Some(v88) => {
                                            Ok(Some(v88))
                                        }
                                    }
                                }
                            } else {
                                Ok(Some(f12(f13(Str::lit("runtime support: method-arity: ")), f12(f14(v83.clone()), f12(f13(Str::lit(" takes ")), f12(f39(v87)?, f12(f13(Str::lit(" type arguments and ")), f12(f39(f66(v83))?, f13(Str::lit(" arguments"))))))))))
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
            let m248 = f67(v0.clone(), v1, v89)?;
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
    let m290 = f68(Fn5::F69, Adt17::C0(List::<Bytes>::nil(), None::<Bytes>), v0)?;
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
            f70(v0, v2, 0u64, true)
        }
        Some(v3) => {
            Ok(Some(v3))
        }
    }
}

pub fn f17(v0: Adt14, v1: List<Adt7>) -> R<Option<Bytes>> {
    f71(Fn6::F72(v0), None::<Bytes>, v1)
}

pub fn f18(v0: Adt14, v1: Bytes, v2: List<Adt7>, v3: Adt2, v4: List<Adt8>) -> R<Option<Bytes>> {
    let m300 = f17(v0.clone(), v2.clone())?;
    match m300 {
        None => {
            let m301 = f73(v0.clone(), v1.clone());
            match m301 {
                None => {
                    Ok(Some(f12(f13(Str::lit("runtime support: impl-trait: ")), f12(f13(Str::lit("`impl ")), f12(v1, f13(Str::lit("` names no declared trait")))))))
                }
                Some(v6) => {
                    let m302 = f15(Fn2::F74(v4.clone(), v1.clone()), None::<Bytes>, v6.clone())?;
                    match m302 {
                        None => {
                            f19(Fn3::F75(v2, v0, v6, v1, v3), None::<Bytes>, v4)
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
            let m317 = f70(v0.clone(), v7.clone(), v6, {
                let a316 = f63(v2.clone());
                bool_not(a316)
            })?;
            match m317 {
                None => {
                    let m322 = if match v7.clone() {
                        Adt5::C0(_, h318, _, _, _) => {
                            h318
                        }
                    } {
                        if f63(v2.clone()) {
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
                            let m325 = f76(Ok::<List<Adt16>, Bytes>(v10), match v7.clone() {
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
                                    f56(v0, Adt15::C0(v5, v12, v2, false), match v3 {
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
    f77(Fn7::F78, f13(Str::lit("")), v0)
}

pub fn f27(v0: List<Bytes>, v1: Str) -> R<Bytes> {
    f79(v0, f13(v1))
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
    f80(Fn8::F81(v0), None::<Bytes>, v1)
}

pub fn f30(v0: Str, v1: List<Bytes>) -> R<Option<Bytes>> {
    f80(Fn8::F82(v0), None::<Bytes>, v1)
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
    if if f83(v1.clone())? {
        let a342 = f45(f84(), v1.clone())?;
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
    if f45(match v0.clone() {
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
    f80(Fn8::F85(v0), None::<Bytes>, v1)
}

pub fn f35(v0: Adt14, v1: List<Adt5>) -> Adt14 {
    f86(Fn9::F87, v0, v1)
}

pub fn f36(v0: Adt14, v1: List<Adt8>) -> Adt14 {
    f88(Fn10::F89, v0, v1)
}

pub fn f37(v0: Adt2, v1: u64) -> R<List<Bytes>> {
    let m356 = v0;
    match m356 {
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
        Adt2::C19(_, r359) => {
            let v11 = (*r359).clone();
            Ok(List::cons(f39(v1)?, f37(v11, {
                let a357 = v1;
                let a358 = 1u64;
                nat_add(a357, a358)?
            })?))
        }
    }
}

pub fn f38(v0: Adt2) -> R<u64> {
    let m360 = v0;
    match m360 {
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
        Adt2::C19(_, r363) => {
            let v10 = (*r363).clone();
            let a361 = f38(v10)?;
            let a362 = 1u64;
            nat_add(a361, a362)
        }
    }
}

pub fn f39(v0: u64) -> R<Bytes> {
    f90(v0)
}

pub fn f40(v0: Bytes, v1: List<Adt9>) -> R<Option<Bytes>> {
    let m365 = f91(Fn11::F92(v0), Adt17::C0(List::<Bytes>::nil(), None::<Bytes>), v1)?;
    match m365 {
        Adt17::C0(_, h364) => {
            Ok(h364)
        }
    }
}

pub fn f41(v0: List<Adt9>) -> R<Option<Bytes>> {
    let m367 = f91(Fn11::F93, Adt17::C0(List::<Bytes>::nil(), None::<Bytes>), v0)?;
    match m367 {
        Adt17::C0(_, h366) => {
            Ok(h366)
        }
    }
}

pub fn f42(v0: Adt14, v1: Bytes) -> Option<u64> {
    let m368 = f94(v0.clone(), v1.clone());
    match m368 {
        None => {
            let m370 = f95(v0.clone(), v1.clone());
            match m370 {
                None => {
                    let m372 = f96(v0, v1.clone());
                    match m372 {
                        None => {
                            f97(v1)
                        }
                        Some(v4) => {
                            Some(match v4 {
                                Adt13::C0(_, h373) => {
                                    h373
                                }
                            })
                        }
                    }
                }
                Some(v3) => {
                    Some(match v3 {
                        Adt12::C0(_, _, h371) => {
                            h371
                        }
                    })
                }
            }
        }
        Some(v2) => {
            Some(match v2 {
                Adt11::C0(_, _, h369) => {
                    h369
                }
            })
        }
    }
}

pub fn f43(v0: Adt14, v1: Bytes) -> bool {
    let m374 = f94(v0.clone(), v1.clone());
    match m374 {
        None => {
            let m375 = f95(v0.clone(), v1.clone());
            match m375 {
                None => {
                    let m376 = f96(v0, v1.clone());
                    match m376 {
                        None => {
                            f98(v1)
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

pub fn f44(v0: List<Adt16>, v1: Bytes) -> Option<Adt16> {
    f99(Fn12::F100(v1), None::<Adt16>, v0)
}

pub fn f45(v0: List<Bytes>, v1: Bytes) -> R<bool> {
    f101(Fn13::F102(v1), false, v0)
}

pub fn f46(v0: u64) -> R<Option<Bytes>> {
    let a377 = v0;
    let a378 = 1u64;
    if nat_eq(a377, a378) {
        Ok(None::<Bytes>)
    } else {
        Ok(Some(f12(f13(Str::lit("runtime support: char-literal: ")), f12(f13(Str::lit("a character literal of ")), f12(f39(v0)?, f13(Str::lit(" scalar values")))))))
    }
}

pub fn f47(v0: Adt14, v1: List<Bytes>) -> R<Option<Bytes>> {
    let m379 = f29(Str::lit("a path segment"), v1.clone())?;
    match m379 {
        None => {
            f103(v0, v1)
        }
        Some(v2) => {
            Ok(Some(v2))
        }
    }
}

pub fn f48(v0: Adt14, v1: List<Bytes>, v2: u64) -> R<Option<Bytes>> {
    if f104(v0.clone(), v1.clone()) {
        Ok(Some(f12(f13(Str::lit("runtime support: hidden-allocation: ")), f12(f13(Str::lit("rust-core renders ")), f12(f14(f27(v1, Str::lit("::"))?), f13(Str::lit(", which requires heap allocation")))))))
    } else {
        let m380 = f105(v0, v1.clone())?;
        match m380 {
            None => {
                Ok(Some(f12(f13(Str::lit("runtime support: call-name: ")), f12(f14(f27(v1, Str::lit("::"))?), f13(Str::lit(" is neither declared nor in the closed table"))))))
            }
            Some(v3) => {
                let a381 = v3;
                let a382 = v2;
                if nat_eq(a381, a382) {
                    Ok(None::<Bytes>)
                } else {
                    Ok(Some(f12(f13(Str::lit("runtime support: call-arity: ")), f12(f14(f27(v1, Str::lit("::"))?), f12(f13(Str::lit(" takes ")), f12(f39(v3)?, f12(f13(Str::lit(" arguments, given ")), f39(v2)?)))))))
                }
            }
        }
    }
}

pub fn f49(v0: Adt3) -> R<u64> {
    let m383 = v0;
    match m383 {
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
        Adt3::C32(_, r386) => {
            let v60 = (*r386).clone();
            let a384 = f49(v60)?;
            let a385 = 1u64;
            nat_add(a384, a385)
        }
        Adt3::C33(_, _, _, _) => {
            Ok(0u64)
        }
        Adt3::C34(_, _, _, _) => {
            Ok(0u64)
        }
    }
}

pub fn f50(v0: Adt14, v1: Adt15, p392: Adt3) -> R<Option<Bytes>> {
    let m391 = p392;
    {
        let mut m391 = m391;
        loop {
            let step = {
                let v2 = m391;
                let m387 = v2;
                match m387 {
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
                    Adt3::C32(r389, r390) => {
                        let v61 = (*r389).clone();
                        let v62 = (*r390).clone();
                        let m388 = f9(v0.clone(), v1.clone(), v61)?;
                        match m388 {
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
                Ok(next) => { m391 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f51(v0: Adt14, v1: Adt15, v2: Adt3, v3: Bytes, v4: u64) -> R<Option<Bytes>> {
    let m393 = f106(v3.clone());
    match m393 {
        None => {
            let v6: List<u64> = f108(v0.clone(), v3.clone());
            let m397 = v6.clone();
            match m397.uncons() {
                None => {
                    Ok(Some(f12(f13(Str::lit("runtime support: method-name: ")), f12(f14(v3), f13(Str::lit(" is a method of no declared trait or impl and not in the closed table"))))))
                }
                Some((_, _)) => {
                    if f109(v6.clone(), v4) {
                        f110(v0, v1, v2, v3)
                    } else {
                        Ok(Some(f12(f13(Str::lit("runtime support: method-arity: ")), f12(f14(v3), f12(f13(Str::lit(" takes ")), f12(f111(v6)?, f12(f13(Str::lit(" arguments, given ")), f39(v4)?)))))))
                    }
                }
            }
        }
        Some(v5) => {
            if if f107(v3.clone()) {
                match v0 {
                    Adt14::C0(h394, _, _, _, _, _, _, _, _, _) => {
                        h394
                    }
                }
            } else {
                false
            } {
                Ok(Some(f12(f13(Str::lit("runtime support: hidden-allocation: ")), f12(f13(Str::lit("rust-core renders ")), f12(f14(v3), f13(Str::lit(", which requires heap allocation")))))))
            } else {
                let a395 = v5;
                let a396 = v4;
                if nat_eq(a395, a396) {
                    Ok(None::<Bytes>)
                } else {
                    Ok(Some(f12(f13(Str::lit("runtime support: method-arity: ")), f12(f14(v3), f12(f13(Str::lit(" takes ")), f12(f39(v5)?, f12(f13(Str::lit(" arguments, given ")), f39(v4)?)))))))
                }
            }
        }
    }
}

pub fn f52(v0: Adt3) -> R<Option<Bytes>> {
    if f112(v0)? {
        Ok(Some(f12(f13(Str::lit("runtime support: struct-head: ")), f13(Str::lit("a struct literal outside parentheses in the head of a condition, a loop, or a match")))))
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f53(v0: Adt3) -> bool {
    let m398 = v0;
    match m398 {
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

pub fn f54(v0: Adt14, v1: Adt15, p408: Adt3) -> R<Option<Bytes>> {
    let m407 = p408;
    {
        let mut m407 = m407;
        loop {
            let step = {
                let v2 = m407;
                let m399 = v2;
                match m399 {
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
                    Adt3::C34(v67, r404, r405, r406) => {
                        let v68 = (*r404).clone();
                        let v69 = (*r405).clone();
                        let v70 = (*r406).clone();
                        let m400 = f58(v0.clone(), v67.clone())?;
                        match m400 {
                            None => {
                                let m401 = f59(List::<Adt16>::nil(), v67)?;
                                match m401 {
                                    Ok(v73) => {
                                        let v74: Adt15 = f57(v1.clone(), v73, false);
                                        let m402 = if f60(v68.clone()) {
                                            None::<Bytes>
                                        } else {
                                            f9(v0.clone(), v74.clone(), v68)?
                                        };
                                        match m402 {
                                            None => {
                                                let m403 = f9(v0.clone(), v74, v69)?;
                                                match m403 {
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
                Ok(next) => { m407 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f55(v0: Adt15, p419: Adt3) -> Option<Bytes> {
    let m418 = p419;
    {
        let mut m418 = m418;
        loop {
            let step = {
                let v1 = m418;
                let m409 = v1;
                match m409 {
                    Adt3::C0(v2) => {
                        let m411 = f44(match v0.clone() {
                            Adt15::C0(_, h410, _, _) => {
                                h410
                            }
                        }, v2.clone());
                        match m411 {
                            None => {
                                Err::<Adt3, Option<Bytes>>(Some(f12(f13(Str::lit("runtime support: variable-scope: ")), f12(f14(v2), f13(Str::lit(" is not in scope"))))))
                            }
                            Some(v3) => {
                                if match v3.clone() {
                                    Adt16::C0(_, h412, _) => {
                                        h412
                                    }
                                } {
                                    Err::<Adt3, Option<Bytes>>(None::<Bytes>)
                                } else {
                                    if f113(match v3 {
                                        Adt16::C0(_, _, h413) => {
                                            h413
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
                    Adt3::C5(r414, _) => {
                        let v11 = (*r414).clone();
                        Ok::<Adt3, Option<Bytes>>(v11)
                    }
                    Adt3::C6(r415, _) => {
                        let v13 = (*r415).clone();
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
                    Adt3::C24(r416, _) => {
                        let v48 = (*r416).clone();
                        Ok::<Adt3, Option<Bytes>>(v48)
                    }
                    Adt3::C25(_, _, _) => {
                        Err::<Adt3, Option<Bytes>>(Some(f12(f13(Str::lit("runtime support: assign-place: ")), f13(Str::lit("an assignment to a value that is no place")))))
                    }
                    Adt3::C26(r417) => {
                        let v53 = (*r417).clone();
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
                Ok(next) => { m418 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f56(v0: Adt14, v1: Adt15, v2: Adt3) -> R<Option<Bytes>> {
    let m420 = v2;
    match m420 {
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
        Adt3::C29(r422, r423) => {
            let v59 = (*r422).clone();
            let v60 = (*r423).clone();
            let m421 = f67(v0.clone(), v1, v59)?;
            match m421 {
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

pub fn f57(v0: Adt15, v1: List<Adt16>, v2: bool) -> Adt15 {
    Adt15::C0(match v0.clone() {
        Adt15::C0(h424, _, _, _) => {
            h424
        }
    }, {
        let a426 = v1;
        let a427 = match v0.clone() {
            Adt15::C0(_, h425, _, _) => {
                h425
            }
        };
        append_list(a426, a427)
    }, match v0.clone() {
        Adt15::C0(_, _, h428, _) => {
            h428
        }
    }, if match v0 {
        Adt15::C0(_, _, _, h429) => {
            h429
        }
    } {
        true
    } else {
        v2
    })
}

pub fn f58(v0: Adt14, v1: Adt4) -> R<Option<Bytes>> {
    let m430 = v1;
    match m430 {
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
            f47(v0, v4)
        }
        Adt4::C4(r440) => {
            let v5 = (*r440).clone();
            f58(v0, v5)
        }
        Adt4::C5(v6, r441) => {
            let v7 = (*r441).clone();
            if f114(v6.clone()) {
                f58(v0, v7)
            } else {
                let m431 = f47(v0.clone(), v6.clone())?;
                match m431 {
                    None => {
                        let m432 = f115(v0.clone(), v6, v7.clone())?;
                        match m432 {
                            None => {
                                f58(v0, v7)
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
        Adt4::C6(v10, v11, r442) => {
            let v12 = (*r442).clone();
            let m433 = f47(v0.clone(), v10.clone())?;
            match m433 {
                None => {
                    let m434 = f30(Str::lit("a pattern field name"), v11.clone())?;
                    match m434 {
                        None => {
                            let a436 = {
                                let a435 = v11.clone();
                                length_list(a435)
                            };
                            let a437 = f116(v12.clone())?;
                            if nat_eq(a436, a437) {
                                let m438 = f117(v0.clone(), v10, v11)?;
                                match m438 {
                                    None => {
                                        f58(v0, v12)
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
            f46(v16)
        }
        Adt4::C8 => {
            Ok(None::<Bytes>)
        }
        Adt4::C9(r443, r444) => {
            let v17 = (*r443).clone();
            let v18 = (*r444).clone();
            let m439 = f58(v0.clone(), v17)?;
            match m439 {
                None => {
                    f58(v0, v18)
                }
                Some(v19) => {
                    Ok(Some(v19))
                }
            }
        }
    }
}

pub fn f59(v0: List<Adt16>, v1: Adt4) -> R<Result<List<Adt16>, Bytes>> {
    let m445 = v1;
    match m445 {
        Adt4::C0 => {
            Ok(Ok::<List<Adt16>, Bytes>(v0))
        }
        Adt4::C1(v2) => {
            let m446 = f32(Str::lit("a pattern binding"), v2.clone())?;
            match m446 {
                None => {
                    let m447 = f44(v0.clone(), v2.clone());
                    match m447 {
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
            let m448 = f32(Str::lit("a pattern binding"), v5.clone())?;
            match m448 {
                None => {
                    let m449 = f44(v0.clone(), v5.clone());
                    match m449 {
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
        Adt4::C4(r451) => {
            let v9 = (*r451).clone();
            f59(v0, v9)
        }
        Adt4::C5(_, r452) => {
            let v11 = (*r452).clone();
            f59(v0, v11)
        }
        Adt4::C6(_, _, r453) => {
            let v14 = (*r453).clone();
            f59(v0, v14)
        }
        Adt4::C7(_) => {
            Ok(Ok::<List<Adt16>, Bytes>(v0))
        }
        Adt4::C8 => {
            Ok(Ok::<List<Adt16>, Bytes>(v0))
        }
        Adt4::C9(r454, r455) => {
            let v16 = (*r454).clone();
            let v17 = (*r455).clone();
            let m450 = f59(v0, v16)?;
            match m450 {
                Ok(v18) => {
                    f59(v18, v17)
                }
                Err(v19) => {
                    Ok(Err::<List<Adt16>, Bytes>(v19))
                }
            }
        }
    }
}

pub fn f60(v0: Adt3) -> bool {
    let m456 = v0;
    match m456 {
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

pub fn f61(v0: Adt14, v1: List<Bytes>) -> Option<List<Bytes>> {
    let m457 = v1;
    match m457.uncons() {
        None => {
            None::<List<Bytes>>
        }
        Some((v2, v3)) => {
            let m458 = v3;
            match m458.uncons() {
                None => {
                    let m459 = f94(v0, v2);
                    match m459 {
                        None => {
                            None::<List<Bytes>>
                        }
                        Some(v4) => {
                            Some(match v4 {
                                Adt11::C0(_, h460, _) => {
                                    h460
                                }
                            })
                        }
                    }
                }
                Some((v5, v6)) => {
                    let m461 = v6;
                    match m461.uncons() {
                        None => {
                            let m462 = f95(v0, v2);
                            match m462 {
                                None => {
                                    None::<List<Bytes>>
                                }
                                Some(v7) => {
                                    f118(match v7 {
                                        Adt12::C0(_, h463, _) => {
                                            h463
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

pub fn f62(v0: List<Bytes>, v1: List<Bytes>, v2: u64) -> R<bool> {
    let a466 = {
        let a464 = v0.clone();
        length_list(a464)
    };
    let a467 = {
        let a465 = v1.clone();
        length_list(a465)
    };
    if nat_eq(a466, a467) {
        let a469 = {
            let a468 = v0.clone();
            length_list(a468)
        };
        let a470 = v2;
        if nat_eq(a469, a470) {
            f101(Fn13::F119(v1.clone(), v0), true, v1)
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f63(v0: Adt2) -> bool {
    let m471 = v0;
    match m471 {
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

pub fn f64(v0: Str, v1: Adt2) -> Option<Bytes> {
    let m472 = v1;
    match m472 {
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
        Adt2::C7(r475) => {
            let v3 = (*r475).clone();
            f64(v0, v3)
        }
        Adt2::C8(r476) => {
            let v4 = (*r476).clone();
            f64(v0, v4)
        }
        Adt2::C9(r477, r478) => {
            let v5 = (*r477).clone();
            let v6 = (*r478).clone();
            let m473 = f64(v0.clone(), v5);
            match m473 {
                None => {
                    f64(v0, v6)
                }
                Some(v7) => {
                    Some(v7)
                }
            }
        }
        Adt2::C10(_, r479) => {
            let v9 = (*r479).clone();
            f64(v0, v9)
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
        Adt2::C15(r480) => {
            let v10 = (*r480).clone();
            f64(v0, v10)
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
        Adt2::C19(r481, r482) => {
            let v11 = (*r481).clone();
            let v12 = (*r482).clone();
            let m474 = f64(v0.clone(), v11);
            match m474 {
                None => {
                    f64(v0, v12)
                }
                Some(v13) => {
                    Some(v13)
                }
            }
        }
    }
}

pub fn f65(v0: Bytes) -> Option<u64> {
    let a483 = v0.clone();
    let a484 = f13(Str::lit("parse"));
    if equal(a483, a484) {
        Some(1u64)
    } else {
        let a485 = v0;
        let a486 = f13(Str::lit("collect"));
        if equal(a485, a486) {
            Some(1u64)
        } else {
            None::<u64>
        }
    }
}

pub fn f66(v0: Bytes) -> u64 {
    let a487 = v0.clone();
    let a488 = f13(Str::lit("parse"));
    if equal(a487, a488) {
        0u64
    } else {
        let a489 = v0;
        let a490 = f13(Str::lit("collect"));
        let _ = equal(a489, a490);
        0u64
    }
}

pub fn f67(v0: Adt14, p506: Adt15, p507: Adt3) -> R<Result<Adt15, Bytes>> {
    let m505 = (p506, p507);
    {
        let mut m505 = m505;
        loop {
            let step = {
                let (v1, v2) = m505;
                let m491 = v2;
                match m491 {
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
                    Adt3::C32(r501, r502) => {
                        let v61 = (*r501).clone();
                        let v62 = (*r502).clone();
                        let m492 = f9(v0.clone(), v1.clone(), v61)?;
                        match m492 {
                            None => {
                                Ok::<(Adt15, Adt3), R<Result<Adt15, Bytes>>>((v1, v62))
                            }
                            Some(v63) => {
                                Err::<(Adt15, Adt3), R<Result<Adt15, Bytes>>>(Ok(Err::<Adt15, Bytes>(v63)))
                            }
                        }
                    }
                    Adt3::C33(v64, v65, r503, r504) => {
                        let v66 = (*r503).clone();
                        let v67 = (*r504).clone();
                        let m497 = if f63(v65.clone()) {
                            None::<Bytes>
                        } else {
                            f28(f8(v0.clone(), v65.clone(), {
                                let a494 = match v1.clone() {
                                    Adt15::C0(h493, _, _, _) => {
                                        h493
                                    }
                                };
                                length_list(a494)
                            }, {
                                let a496 = f63(match v1.clone() {
                                    Adt15::C0(_, _, h495, _) => {
                                        h495
                                    }
                                });
                                bool_not(a496)
                            })?, f64(Str::lit("a `let` type"), v65))
                        };
                        match m497 {
                            None => {
                                let m498 = f9(v0.clone(), v1.clone(), v66)?;
                                match m498 {
                                    None => {
                                        let m499 = f59(List::<Adt16>::nil(), v64.clone())?;
                                        match m499 {
                                            Ok(v71) => {
                                                let m500 = f58(v0.clone(), v64)?;
                                                match m500 {
                                                    None => {
                                                        Ok::<(Adt15, Adt3), R<Result<Adt15, Bytes>>>((f57(v1, v71, false), v67))
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
                Ok(next) => { m505 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f68(v0: Fn5, p511: Adt17, p512: List<Adt5>) -> R<Adt17> {
    let m510 = (p511, p512);
    {
        let mut m510 = m510;
        loop {
            let step = {
                let (v1, v2) = m510;
                let m508 = v2;
                match m508.uncons() {
                    None => {
                        Err::<(Adt17, List<Adt5>), R<Adt17>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt17, List<Adt5>), R<Adt17>>(({
                            let c509: Fn5 = v0.clone();
                            c509.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m510 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f69(v0: Adt17, v1: Adt5) -> R<Adt17> {
    let m514 = match v0.clone() {
        Adt17::C0(_, h513) => {
            h513
        }
    };
    match m514 {
        None => {
            if f45(match v0.clone() {
                Adt17::C0(h515, _) => {
                    h515
                }
            }, match v1.clone() {
                Adt5::C0(h516, _, _, _, _) => {
                    h516
                }
            })? {
                Ok(Adt17::C0(match v0 {
                    Adt17::C0(h517, _) => {
                        h517
                    }
                }, Some(match v1 {
                    Adt5::C0(h518, _, _, _, _) => {
                        h518
                    }
                })))
            } else {
                Ok(Adt17::C0(List::cons(match v1 {
                    Adt5::C0(h519, _, _, _, _) => {
                        h519
                    }
                }, match v0 {
                    Adt17::C0(h520, _) => {
                        h520
                    }
                }), None::<Bytes>))
            }
        }
        Some(_) => {
            Ok(v0)
        }
    }
}

pub fn f70(v0: Adt14, v1: Adt5, v2: u64, v3: bool) -> R<Option<Bytes>> {
    let m522 = f32(Str::lit("a method name"), match v1.clone() {
        Adt5::C0(h521, _, _, _, _) => {
            h521
        }
    })?;
    match m522 {
        None => {
            if if match v1.clone() {
                Adt5::C0(_, _, h523, _, _) => {
                    h523
                }
            } {
                let a525 = match v1.clone() {
                    Adt5::C0(_, h524, _, _, _) => {
                        h524
                    }
                };
                bool_not(a525)
            } else {
                false
            } {
                Ok(Some(f12(f13(Str::lit("runtime support: function-receiver: ")), f12(f14(match v1 {
                    Adt5::C0(h526, _, _, _, _) => {
                        h526
                    }
                }), f13(Str::lit(" is exclusive without a receiver"))))))
            } else {
                let m528 = f120(v0.clone(), match v1.clone() {
                    Adt5::C0(_, _, _, h527, _) => {
                        h527
                    }
                }, v2, v3)?;
                match m528 {
                    None => {
                        f8(v0, match v1 {
                            Adt5::C0(_, _, _, _, h529) => {
                                h529
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

pub fn f71(v0: Fn6, p533: Option<Bytes>, p534: List<Adt7>) -> R<Option<Bytes>> {
    let m532 = (p533, p534);
    {
        let mut m532 = m532;
        loop {
            let step = {
                let (v1, v2) = m532;
                let m530 = v2;
                match m530.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Adt7>), R<Option<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Adt7>), R<Option<Bytes>>>(({
                            let c531: Fn6 = v0.clone();
                            c531.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m532 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f72(v0: Adt14, v1: Option<Bytes>, v2: Adt7) -> R<Option<Bytes>> {
    let m535 = v1;
    match m535 {
        None => {
            let m537 = f32(Str::lit("a generic parameter"), match v2.clone() {
                Adt7::C0(h536, _) => {
                    h536
                }
            })?;
            match m537 {
                None => {
                    f80(Fn8::F121(v2.clone(), v0), None::<Bytes>, match v2 {
                        Adt7::C0(_, h538) => {
                            h538
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

pub fn f73(v0: Adt14, v1: Bytes) -> Option<List<Adt5>> {
    let m539 = f122(v0, v1.clone());
    match m539 {
        None => {
            let m541 = f123(v1.clone());
            match m541 {
                None => {
                    None::<List<Adt5>>
                }
                Some(v3) => {
                    Some(List::cons(Adt5::C0(v3, true, f124(v1.clone()), List::<Adt6>::nil(), if f124(v1) {
                        Adt2::C0
                    } else {
                        Adt2::C5
                    }), List::<Adt5>::nil()))
                }
            }
        }
        Some(v2) => {
            Some(match v2 {
                Adt10::C0(_, h540) => {
                    h540
                }
            })
        }
    }
}

pub fn f74(v0: List<Adt8>, v1: Bytes, v2: Option<Bytes>, v3: Adt5) -> R<Option<Bytes>> {
    let m542 = v2;
    match m542 {
        None => {
            let v5: u64 = f125(v0.clone(), match v3.clone() {
                Adt5::C0(h543, _, _, _, _) => {
                    h543
                }
            })?;
            let a544 = v5;
            let a545 = 0u64;
            if nat_eq(a544, a545) {
                Ok(Some(f12(f13(Str::lit("runtime support: impl-complete: ")), f12(f13(Str::lit("`impl ")), f12(v1, f12(f13(Str::lit("` does not state ")), f14(match v3 {
                    Adt5::C0(h546, _, _, _, _) => {
                        h546
                    }
                })))))))
            } else {
                let a547 = v5;
                let a548 = 1u64;
                if nat_eq(a547, a548) {
                    let m550 = f126(v0, match v3.clone() {
                        Adt5::C0(h549, _, _, _, _) => {
                            h549
                        }
                    });
                    match m550 {
                        None => {
                            Ok(None::<Bytes>)
                        }
                        Some(v6) => {
                            let a552 = f127(match v6 {
                                Adt8::C0(h551, _, _, _) => {
                                    h551
                                }
                            })?;
                            let a553 = f127(v3.clone())?;
                            if equal(a552, a553) {
                                Ok(None::<Bytes>)
                            } else {
                                Ok(Some(f12(f13(Str::lit("runtime support: impl-signature: ")), f12(f13(Str::lit("`impl ")), f12(v1, f12(f13(Str::lit("` states ")), f12(f14(match v3 {
                                    Adt5::C0(h554, _, _, _, _) => {
                                        h554
                                    }
                                }), f13(Str::lit(" with a signature other than the trait's")))))))))
                            }
                        }
                    }
                } else {
                    Ok(Some(f12(f13(Str::lit("runtime support: impl-complete: ")), f12(f13(Str::lit("`impl ")), f12(v1, f12(f13(Str::lit("` states ")), f12(f14(match v3 {
                        Adt5::C0(h555, _, _, _, _) => {
                            h555
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

pub fn f75(v0: List<Adt7>, v1: Adt14, v2: List<Adt5>, v3: Bytes, v4: Adt2, v5: Option<Bytes>, v6: Adt8) -> R<Option<Bytes>> {
    let m556 = v5;
    match m556 {
        None => {
            let a560 = f128(v2, {
                let m559 = match v6.clone() {
                    Adt8::C0(h558, _, _, _) => {
                        h558
                    }
                };
                match m559 {
                    Adt5::C0(h557, _, _, _, _) => {
                        h557
                    }
                }
            });
            if bool_not(a560) {
                Ok(Some(f12(f13(Str::lit("runtime support: impl-extra: ")), f12(f13(Str::lit("`impl ")), f12(v3, f12(f13(Str::lit("` states ")), f12(f14({
                    let m563 = match v6 {
                        Adt8::C0(h562, _, _, _) => {
                            h562
                        }
                    };
                    match m563 {
                        Adt5::C0(h561, _, _, _, _) => {
                            h561
                        }
                    }
                }), f13(Str::lit(", which the trait does not")))))))))
            } else {
                if match v6.clone() {
                    Adt8::C0(_, _, h564, _) => {
                        h564
                    }
                } {
                    Ok(Some(f12(f13(Str::lit("runtime support: impl-visibility: ")), f12(f14({
                        let m567 = match v6 {
                            Adt8::C0(h566, _, _, _) => {
                                h566
                            }
                        };
                        match m567 {
                            Adt5::C0(h565, _, _, _, _) => {
                                h565
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

pub fn f76(v0: Result<List<Adt16>, Bytes>, v1: List<Adt6>, v2: Bytes) -> Result<List<Adt16>, Bytes> {
    f129(Fn14::F130(v2), v0, v1)
}

pub fn f77(v0: Fn7, p571: Bytes, p572: List<Bytes>) -> Bytes {
    let m570 = (p571, p572);
    {
        let mut m570 = m570;
        loop {
            let step = {
                let (v1, v2) = m570;
                let m568 = v2;
                match m568.uncons() {
                    None => {
                        Err::<(Bytes, List<Bytes>), Bytes>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Bytes, List<Bytes>), Bytes>(({
                            let c569: Fn7 = v0.clone();
                            c569.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m570 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f78(_: Bytes, v1: Bytes) -> Bytes {
    v1
}

pub fn f79(v0: List<Bytes>, v1: Bytes) -> R<Bytes> {
    let m573 = f80(Fn8::F131(v1), None::<Bytes>, v0)?;
    match m573 {
        None => {
            Ok(Bytes::lit(&[]))
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f80(v0: Fn8, p577: Option<Bytes>, p578: List<Bytes>) -> R<Option<Bytes>> {
    let m576 = (p577, p578);
    {
        let mut m576 = m576;
        loop {
            let step = {
                let (v1, v2) = m576;
                let m574 = v2;
                match m574.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Bytes>), R<Option<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Bytes>), R<Option<Bytes>>>(({
                            let c575: Fn8 = v0.clone();
                            c575.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m576 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f81(v0: Str, v1: Option<Bytes>, v2: Bytes) -> R<Option<Bytes>> {
    let m579 = v1;
    match m579 {
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

pub fn f82(v0: Str, v1: Option<Bytes>, v2: Bytes) -> R<Option<Bytes>> {
    let m580 = v1;
    match m580 {
        None => {
            f32(v0, v2)
        }
        Some(v3) => {
            Ok(Some(v3))
        }
    }
}

pub fn f83(v0: Bytes) -> R<bool> {
    let a582 = 0u64;
    let a583 = {
        let a581 = v0.clone();
        length_bytes(a581)
    };
    if nat_lt(a582, a583) {
        let a587 = {
            let (h584, _) = f132(Fn15::F133(v0.clone()), {
                let a585 = v0.clone();
                length_bytes(a585)
            }, 0u64)?;
            h584
        };
        let a588 = {
            let a586 = v0;
            length_bytes(a586)
        };
        Ok(nat_eq(a587, a588))
    } else {
        Ok(false)
    }
}

pub fn f84() -> List<Bytes> {
    List::cons(f13(Str::lit("abstract")), List::cons(f13(Str::lit("as")), List::cons(f13(Str::lit("async")), List::cons(f13(Str::lit("await")), List::cons(f13(Str::lit("become")), List::cons(f13(Str::lit("box")), List::cons(f13(Str::lit("break")), List::cons(f13(Str::lit("const")), List::cons(f13(Str::lit("continue")), List::cons(f13(Str::lit("crate")), List::cons(f13(Str::lit("do")), List::cons(f13(Str::lit("dyn")), List::cons(f13(Str::lit("else")), List::cons(f13(Str::lit("enum")), List::cons(f13(Str::lit("extern")), List::cons(f13(Str::lit("false")), List::cons(f13(Str::lit("final")), List::cons(f13(Str::lit("fn")), List::cons(f13(Str::lit("for")), List::cons(f13(Str::lit("if")), List::cons(f13(Str::lit("impl")), List::cons(f13(Str::lit("in")), List::cons(f13(Str::lit("let")), List::cons(f13(Str::lit("loop")), List::cons(f13(Str::lit("macro")), List::cons(f13(Str::lit("match")), List::cons(f13(Str::lit("mod")), List::cons(f13(Str::lit("move")), List::cons(f13(Str::lit("mut")), List::cons(f13(Str::lit("override")), List::cons(f13(Str::lit("priv")), List::cons(f13(Str::lit("pub")), List::cons(f13(Str::lit("ref")), List::cons(f13(Str::lit("return")), List::cons(f13(Str::lit("self")), List::cons(f13(Str::lit("Self")), List::cons(f13(Str::lit("static")), List::cons(f13(Str::lit("struct")), List::cons(f13(Str::lit("super")), List::cons(f13(Str::lit("trait")), List::cons(f13(Str::lit("true")), List::cons(f13(Str::lit("try")), List::cons(f13(Str::lit("type")), List::cons(f13(Str::lit("typeof")), List::cons(f13(Str::lit("unsafe")), List::cons(f13(Str::lit("unsized")), List::cons(f13(Str::lit("use")), List::cons(f13(Str::lit("virtual")), List::cons(f13(Str::lit("where")), List::cons(f13(Str::lit("while")), List::cons(f13(Str::lit("yield")), List::cons(f13(Str::lit("_")), List::<Bytes>::nil()))))))))))))))))))))))))))))))))))))))))))))))))))))
}

pub fn f85(v0: Str, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m589 = v1;
    match m589 {
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

pub fn f86(v0: Fn9, p593: Adt14, p594: List<Adt5>) -> Adt14 {
    let m592 = (p593, p594);
    {
        let mut m592 = m592;
        loop {
            let step = {
                let (v1, v2) = m592;
                let m590 = v2;
                match m590.uncons() {
                    None => {
                        Err::<(Adt14, List<Adt5>), Adt14>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt14, List<Adt5>), Adt14>(({
                            let c591: Fn9 = v0.clone();
                            c591.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m592 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f87(v0: Adt14, v1: Adt5) -> Adt14 {
    Adt14::C0(match v0.clone() {
        Adt14::C0(h595, _, _, _, _, _, _, _, _, _) => {
            h595
        }
    }, match v0.clone() {
        Adt14::C0(_, h596, _, _, _, _, _, _, _, _) => {
            h596
        }
    }, match v0.clone() {
        Adt14::C0(_, _, h597, _, _, _, _, _, _, _) => {
            h597
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, h598, _, _, _, _, _, _) => {
            h598
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, h599, _, _, _, _, _) => {
            h599
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, h600, _, _, _, _) => {
            h600
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, h601, _, _, _) => {
            h601
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, h602, _, _) => {
            h602
        }
    }, {
        let a607 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, h603, _) => {
                h603
            }
        };
        let a608 = List::cons(Adt13::C0(match v1.clone() {
            Adt5::C0(h604, _, _, _, _) => {
                h604
            }
        }, {
            let a606 = match v1 {
                Adt5::C0(_, _, _, h605, _) => {
                    h605
                }
            };
            length_list(a606)
        }), List::<Adt13>::nil());
        append_list(a607, a608)
    }, match v0 {
        Adt14::C0(_, _, _, _, _, _, _, _, _, h609) => {
            h609
        }
    })
}

pub fn f88(v0: Fn10, p613: Adt14, p614: List<Adt8>) -> Adt14 {
    let m612 = (p613, p614);
    {
        let mut m612 = m612;
        loop {
            let step = {
                let (v1, v2) = m612;
                let m610 = v2;
                match m610.uncons() {
                    None => {
                        Err::<(Adt14, List<Adt8>), Adt14>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt14, List<Adt8>), Adt14>(({
                            let c611: Fn10 = v0.clone();
                            c611.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m612 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f89(v0: Adt14, v1: Adt8) -> Adt14 {
    Adt14::C0(match v0.clone() {
        Adt14::C0(h615, _, _, _, _, _, _, _, _, _) => {
            h615
        }
    }, match v0.clone() {
        Adt14::C0(_, h616, _, _, _, _, _, _, _, _) => {
            h616
        }
    }, match v0.clone() {
        Adt14::C0(_, _, h617, _, _, _, _, _, _, _) => {
            h617
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, h618, _, _, _, _, _, _) => {
            h618
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, h619, _, _, _, _, _) => {
            h619
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, h620, _, _, _, _) => {
            h620
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, h621, _, _, _) => {
            h621
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, h622, _, _) => {
            h622
        }
    }, {
        let a631 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, h623, _) => {
                h623
            }
        };
        let a632 = List::cons(Adt13::C0({
            let m626 = match v1.clone() {
                Adt8::C0(h625, _, _, _) => {
                    h625
                }
            };
            match m626 {
                Adt5::C0(h624, _, _, _, _) => {
                    h624
                }
            }
        }, {
            let a630 = {
                let m629 = match v1 {
                    Adt8::C0(h628, _, _, _) => {
                        h628
                    }
                };
                match m629 {
                    Adt5::C0(_, _, _, h627, _) => {
                        h627
                    }
                }
            };
            length_list(a630)
        }), List::<Adt13>::nil());
        append_list(a631, a632)
    }, match v0 {
        Adt14::C0(_, _, _, _, _, _, _, _, _, h633) => {
            h633
        }
    })
}

pub fn f90(v0: u64) -> R<Bytes> {
    let v1: Bytes = f134(Str::lit("0"));
    let v2: Bytes = f135({
        let a670 = f136(v0, 10000000000000000000u64);
        let a671 = {
            let a668 = f136(v0, 1000000000000000000u64);
            let a669 = {
                let a666 = f136(v0, 100000000000000000u64);
                let a667 = {
                    let a664 = f136(v0, 10000000000000000u64);
                    let a665 = {
                        let a662 = f136(v0, 1000000000000000u64);
                        let a663 = {
                            let a660 = f136(v0, 100000000000000u64);
                            let a661 = {
                                let a658 = f136(v0, 10000000000000u64);
                                let a659 = {
                                    let a656 = f136(v0, 1000000000000u64);
                                    let a657 = {
                                        let a654 = f136(v0, 100000000000u64);
                                        let a655 = {
                                            let a652 = f136(v0, 10000000000u64);
                                            let a653 = {
                                                let a650 = f136(v0, 1000000000u64);
                                                let a651 = {
                                                    let a648 = f136(v0, 100000000u64);
                                                    let a649 = {
                                                        let a646 = f136(v0, 10000000u64);
                                                        let a647 = {
                                                            let a644 = f136(v0, 1000000u64);
                                                            let a645 = {
                                                                let a642 = f136(v0, 100000u64);
                                                                let a643 = {
                                                                    let a640 = f136(v0, 10000u64);
                                                                    let a641 = {
                                                                        let a638 = f136(v0, 1000u64);
                                                                        let a639 = {
                                                                            let a636 = f136(v0, 100u64);
                                                                            let a637 = {
                                                                                let a634 = f136(v0, 10u64);
                                                                                let a635 = f136(v0, 1u64);
                                                                                append_bytes(a634, a635)
                                                                            };
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
    })?;
    let a672 = v0;
    let a673 = 0u64;
    if nat_eq(a672, a673) {
        Ok(v1)
    } else {
        Ok(v2)
    }
}

pub fn f91(v0: Fn11, p677: Adt17, p678: List<Adt9>) -> R<Adt17> {
    let m676 = (p677, p678);
    {
        let mut m676 = m676;
        loop {
            let step = {
                let (v1, v2) = m676;
                let m674 = v2;
                match m674.uncons() {
                    None => {
                        Err::<(Adt17, List<Adt9>), R<Adt17>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt17, List<Adt9>), R<Adt17>>(({
                            let c675: Fn11 = v0.clone();
                            c675.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m676 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f92(v0: Bytes, v1: Adt17, v2: Adt9) -> R<Adt17> {
    let m680 = match v1.clone() {
        Adt17::C0(_, h679) => {
            h679
        }
    };
    match m680 {
        None => {
            let m681 = f32(Str::lit("a variant name"), f137(v2.clone()))?;
            match m681 {
                None => {
                    let m684 = {
                        let m683 = v2.clone();
                        match m683 {
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
                    match m684 {
                        None => {
                            if f45(match v1.clone() {
                                Adt17::C0(h686, _) => {
                                    h686
                                }
                            }, f137(v2.clone()))? {
                                Ok(Adt17::C0(match v1 {
                                    Adt17::C0(h687, _) => {
                                        h687
                                    }
                                }, Some(f12(f13(Str::lit("runtime support: variant-name: ")), f12(f14(v0), f12(f13(Str::lit(" declares ")), f12(f14(f137(v2)), f13(Str::lit(" twice")))))))))
                            } else {
                                Ok(Adt17::C0(List::cons(f137(v2), match v1 {
                                    Adt17::C0(h688, _) => {
                                        h688
                                    }
                                }), None::<Bytes>))
                            }
                        }
                        Some(v11) => {
                            Ok(Adt17::C0(match v1 {
                                Adt17::C0(h685, _) => {
                                    h685
                                }
                            }, Some(v11)))
                        }
                    }
                }
                Some(v4) => {
                    Ok(Adt17::C0(match v1 {
                        Adt17::C0(h682, _) => {
                            h682
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

pub fn f93(v0: Adt17, v1: Adt9) -> R<Adt17> {
    let m690 = match v0.clone() {
        Adt17::C0(_, h689) => {
            h689
        }
    };
    match m690 {
        None => {
            if f45(match v0.clone() {
                Adt17::C0(h691, _) => {
                    h691
                }
            }, f137(v1.clone()))? {
                Ok(Adt17::C0(match v0 {
                    Adt17::C0(h692, _) => {
                        h692
                    }
                }, Some(f137(v1))))
            } else {
                Ok(Adt17::C0(List::cons(f137(v1), match v0 {
                    Adt17::C0(h693, _) => {
                        h693
                    }
                }), None::<Bytes>))
            }
        }
        Some(_) => {
            Ok(v0)
        }
    }
}

pub fn f94(v0: Adt14, v1: Bytes) -> Option<Adt11> {
    f138(Fn16::F139(v1), None::<Adt11>, match v0 {
        Adt14::C0(_, _, _, h694, _, _, _, _, _, _) => {
            h694
        }
    })
}

pub fn f95(v0: Adt14, v1: Bytes) -> Option<Adt12> {
    f140(Fn17::F141(v1), None::<Adt12>, match v0 {
        Adt14::C0(_, _, _, _, h695, _, _, _, _, _) => {
            h695
        }
    })
}

pub fn f96(v0: Adt14, v1: Bytes) -> Option<Adt13> {
    f142(Fn18::F143(v1), None::<Adt13>, match v0 {
        Adt14::C0(_, _, _, _, _, h696, _, _, _, _) => {
            h696
        }
    })
}

pub fn f97(v0: Bytes) -> Option<u64> {
    let a697 = v0.clone();
    let a698 = f13(Str::lit("Result"));
    if equal(a697, a698) {
        Some(2u64)
    } else {
        let a699 = v0.clone();
        let a700 = f13(Str::lit("Option"));
        if equal(a699, a700) {
            Some(1u64)
        } else {
            let a701 = v0.clone();
            let a702 = f13(Str::lit("AtomicU64"));
            if equal(a701, a702) {
                Some(0u64)
            } else {
                let a703 = v0.clone();
                let a704 = f13(Str::lit("Rc"));
                if equal(a703, a704) {
                    Some(1u64)
                } else {
                    let a705 = v0.clone();
                    let a706 = f13(Str::lit("Vec"));
                    if equal(a705, a706) {
                        Some(1u64)
                    } else {
                        let a707 = v0;
                        let a708 = f13(Str::lit("String"));
                        if equal(a707, a708) {
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

pub fn f98(v0: Bytes) -> bool {
    let a709 = v0.clone();
    let a710 = f13(Str::lit("Rc"));
    if equal(a709, a710) {
        true
    } else {
        let a711 = v0.clone();
        let a712 = f13(Str::lit("Vec"));
        if equal(a711, a712) {
            true
        } else {
            let a713 = v0;
            let a714 = f13(Str::lit("String"));
            equal(a713, a714)
        }
    }
}

pub fn f99(v0: Fn12, p718: Option<Adt16>, p719: List<Adt16>) -> Option<Adt16> {
    let m717 = (p718, p719);
    {
        let mut m717 = m717;
        loop {
            let step = {
                let (v1, v2) = m717;
                let m715 = v2;
                match m715.uncons() {
                    None => {
                        Err::<(Option<Adt16>, List<Adt16>), Option<Adt16>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Adt16>, List<Adt16>), Option<Adt16>>(({
                            let c716: Fn12 = v0.clone();
                            c716.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m717 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f100(v0: Bytes, v1: Option<Adt16>, v2: Adt16) -> Option<Adt16> {
    let m720 = v1;
    match m720 {
        None => {
            if f25(match v2.clone() {
                Adt16::C0(h721, _, _) => {
                    h721
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

pub fn f101(v0: Fn13, p725: bool, p726: List<Bytes>) -> R<bool> {
    let m724 = (p725, p726);
    {
        let mut m724 = m724;
        loop {
            let step = {
                let (v1, v2) = m724;
                let m722 = v2;
                match m722.uncons() {
                    None => {
                        Err::<(bool, List<Bytes>), R<bool>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<Bytes>), R<bool>>(({
                            let c723: Fn13 = v0.clone();
                            c723.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m724 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f102(v0: Bytes, v1: bool, v2: Bytes) -> bool {
    if v1 {
        true
    } else {
        f25(v2, v0)
    }
}

pub fn f103(v0: Adt14, v1: List<Bytes>) -> R<Option<Bytes>> {
    let v2: Bytes = f144(v1.clone());
    let m727 = f95(v0.clone(), v2.clone());
    match m727 {
        None => {
            if if f45(match v0.clone() {
                Adt14::C0(_, _, _, _, _, _, _, _, _, h731) => {
                    h731
                }
            }, v2.clone())? {
                true
            } else {
                let m732 = f94(v0.clone(), v2.clone());
                if match m732 {
                    None => {
                        false
                    }
                    Some(_) => {
                        true
                    }
                } {
                    true
                } else {
                    if f45(match v0 {
                        Adt14::C0(_, _, _, _, _, _, h733, _, _, _) => {
                            h733
                        }
                    }, v2.clone())? {
                        true
                    } else {
                        if f147(v2.clone()) {
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
            let m728 = f145(v1.clone(), 1u64);
            match m728 {
                None => {
                    Ok(Some(f12(f13(Str::lit("runtime support: variant-name: ")), f12(f14(f27(v1, Str::lit("::"))?), f12(f13(Str::lit(" is not a variant of ")), f14(v2))))))
                }
                Some(v4) => {
                    let m730 = f146(match v3 {
                        Adt12::C0(_, h729, _) => {
                            h729
                        }
                    }, v4);
                    match m730 {
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

pub fn f104(v0: Adt14, v1: List<Bytes>) -> bool {
    if match v0 {
        Adt14::C0(h734, _, _, _, _, _, _, _, _, _) => {
            h734
        }
    } {
        let m735 = v1;
        match m735.uncons() {
            None => {
                false
            }
            Some((v2, v3)) => {
                let m736 = v3;
                match m736.uncons() {
                    None => {
                        false
                    }
                    Some((v4, v5)) => {
                        let m737 = v5;
                        match m737.uncons() {
                            None => {
                                f148(v2, v4)
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

pub fn f105(v0: Adt14, v1: List<Bytes>) -> R<Option<u64>> {
    let m738 = v1;
    match m738.uncons() {
        None => {
            Ok(None::<u64>)
        }
        Some((v2, v3)) => {
            let m739 = v3;
            match m739.uncons() {
                None => {
                    let m740 = f149(v0.clone(), v2.clone());
                    match m740 {
                        None => {
                            let m742 = f150(v2.clone());
                            match m742 {
                                None => {
                                    Ok(f151(v0, v2))
                                }
                                Some(v5) => {
                                    Ok(Some(v5))
                                }
                            }
                        }
                        Some(v4) => {
                            Ok(Some(match v4 {
                                Adt13::C0(_, h741) => {
                                    h741
                                }
                            }))
                        }
                    }
                }
                Some((v6, v7)) => {
                    let m743 = v7;
                    match m743.uncons() {
                        None => {
                            let m744 = f152(v2.clone(), v6.clone());
                            match m744 {
                                None => {
                                    f153(v0, v2, v6)
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

pub fn f106(v0: Bytes) -> Option<u64> {
    let a745 = v0.clone();
    let a746 = f13(Str::lit("fetch_add"));
    if equal(a745, a746) {
        Some(2u64)
    } else {
        let a747 = v0.clone();
        let a748 = f13(Str::lit("load"));
        if equal(a747, a748) {
            Some(1u64)
        } else {
            let a749 = v0.clone();
            let a750 = f13(Str::lit("cmp"));
            if equal(a749, a750) {
                Some(1u64)
            } else {
                let a751 = v0.clone();
                let a752 = f13(Str::lit("len"));
                if equal(a751, a752) {
                    Some(0u64)
                } else {
                    let a753 = v0.clone();
                    let a754 = f13(Str::lit("is_empty"));
                    if equal(a753, a754) {
                        Some(0u64)
                    } else {
                        let a755 = v0.clone();
                        let a756 = f13(Str::lit("chars"));
                        if equal(a755, a756) {
                            Some(0u64)
                        } else {
                            let a757 = v0.clone();
                            let a758 = f13(Str::lit("next"));
                            if equal(a757, a758) {
                                Some(0u64)
                            } else {
                                let a759 = v0.clone();
                                let a760 = f13(Str::lit("min"));
                                if equal(a759, a760) {
                                    Some(1u64)
                                } else {
                                    let a761 = v0.clone();
                                    let a762 = f13(Str::lit("max"));
                                    if equal(a761, a762) {
                                        Some(1u64)
                                    } else {
                                        let a763 = v0.clone();
                                        let a764 = f13(Str::lit("push"));
                                        if equal(a763, a764) {
                                            Some(1u64)
                                        } else {
                                            let a765 = v0.clone();
                                            let a766 = f13(Str::lit("pop"));
                                            if equal(a765, a766) {
                                                Some(0u64)
                                            } else {
                                                let a767 = v0.clone();
                                                let a768 = f13(Str::lit("truncate"));
                                                if equal(a767, a768) {
                                                    Some(1u64)
                                                } else {
                                                    let a769 = v0.clone();
                                                    let a770 = f13(Str::lit("push_str"));
                                                    if equal(a769, a770) {
                                                        Some(1u64)
                                                    } else {
                                                        let a771 = v0.clone();
                                                        let a772 = f13(Str::lit("extend_from_slice"));
                                                        if equal(a771, a772) {
                                                            Some(1u64)
                                                        } else {
                                                            let a773 = v0.clone();
                                                            let a774 = f13(Str::lit("iter"));
                                                            if equal(a773, a774) {
                                                                Some(0u64)
                                                            } else {
                                                                let a775 = v0.clone();
                                                                let a776 = f13(Str::lit("rev"));
                                                                if equal(a775, a776) {
                                                                    Some(0u64)
                                                                } else {
                                                                    let a777 = v0.clone();
                                                                    let a778 = f13(Str::lit("get"));
                                                                    if equal(a777, a778) {
                                                                        Some(1u64)
                                                                    } else {
                                                                        let a779 = v0.clone();
                                                                        let a780 = f13(Str::lit("copied"));
                                                                        if equal(a779, a780) {
                                                                            Some(0u64)
                                                                        } else {
                                                                            let a781 = v0.clone();
                                                                            let a782 = f13(Str::lit("ok"));
                                                                            if equal(a781, a782) {
                                                                                Some(0u64)
                                                                            } else {
                                                                                let a783 = v0.clone();
                                                                                let a784 = f13(Str::lit("clone"));
                                                                                if equal(a783, a784) {
                                                                                    Some(0u64)
                                                                                } else {
                                                                                    let a785 = v0.clone();
                                                                                    let a786 = f13(Str::lit("to_string"));
                                                                                    if equal(a785, a786) {
                                                                                        Some(0u64)
                                                                                    } else {
                                                                                        let a787 = v0.clone();
                                                                                        let a788 = f13(Str::lit("as_bytes"));
                                                                                        if equal(a787, a788) {
                                                                                            Some(0u64)
                                                                                        } else {
                                                                                            let a789 = v0.clone();
                                                                                            let a790 = f13(Str::lit("strip_prefix"));
                                                                                            if equal(a789, a790) {
                                                                                                Some(1u64)
                                                                                            } else {
                                                                                                let a791 = v0.clone();
                                                                                                let a792 = f13(Str::lit("starts_with"));
                                                                                                if equal(a791, a792) {
                                                                                                    Some(1u64)
                                                                                                } else {
                                                                                                    let a793 = v0.clone();
                                                                                                    let a794 = f13(Str::lit("split"));
                                                                                                    if equal(a793, a794) {
                                                                                                        Some(1u64)
                                                                                                    } else {
                                                                                                        let a795 = v0.clone();
                                                                                                        let a796 = f13(Str::lit("collect"));
                                                                                                        if equal(a795, a796) {
                                                                                                            Some(0u64)
                                                                                                        } else {
                                                                                                            let a797 = v0;
                                                                                                            let a798 = f13(Str::lit("is_ascii_digit"));
                                                                                                            if equal(a797, a798) {
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

pub fn f107(v0: Bytes) -> bool {
    let a799 = v0.clone();
    let a800 = f13(Str::lit("push"));
    if equal(a799, a800) {
        true
    } else {
        let a801 = v0.clone();
        let a802 = f13(Str::lit("pop"));
        if equal(a801, a802) {
            true
        } else {
            let a803 = v0.clone();
            let a804 = f13(Str::lit("truncate"));
            if equal(a803, a804) {
                true
            } else {
                let a805 = v0.clone();
                let a806 = f13(Str::lit("push_str"));
                if equal(a805, a806) {
                    true
                } else {
                    let a807 = v0.clone();
                    let a808 = f13(Str::lit("extend_from_slice"));
                    if equal(a807, a808) {
                        true
                    } else {
                        let a809 = v0.clone();
                        let a810 = f13(Str::lit("to_string"));
                        if equal(a809, a810) {
                            true
                        } else {
                            let a811 = v0;
                            let a812 = f13(Str::lit("collect"));
                            equal(a811, a812)
                        }
                    }
                }
            }
        }
    }
}

pub fn f108(v0: Adt14, v1: Bytes) -> List<u64> {
    f154(Fn19::F155(v1), List::<u64>::nil(), match v0 {
        Adt14::C0(_, _, _, _, _, _, _, _, h813, _) => {
            h813
        }
    })
}

pub fn f109(v0: List<u64>, v1: u64) -> bool {
    f156(Fn20::F157(v1), false, v0)
}

pub fn f110(v0: Adt14, v1: Adt15, v2: Adt3, v3: Bytes) -> R<Option<Bytes>> {
    let m814 = f158(f159(v1.clone(), v2));
    match m814 {
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
            if f160(v0, f161(match v1 {
                Adt15::C0(h815, _, _, _) => {
                    h815
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

pub fn f111(v0: List<u64>) -> R<Bytes> {
    f162(Fn21::F163, f13(Str::lit("")), v0)
}

pub fn f112(v0: Adt3) -> R<bool> {
    let m816 = v0;
    match m816 {
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
        Adt3::C4(r847, _, _) => {
            let v6 = (*r847).clone();
            let a817 = 13u64;
            let a818 = f164(v6.clone());
            if nat_le(a817, a818) {
                f112(v6)
            } else {
                Ok(false)
            }
        }
        Adt3::C5(r848, _) => {
            let v9 = (*r848).clone();
            let a819 = 13u64;
            let a820 = f164(v9.clone());
            if nat_le(a819, a820) {
                f112(v9)
            } else {
                Ok(false)
            }
        }
        Adt3::C6(r849, _) => {
            let v11 = (*r849).clone();
            let a821 = 13u64;
            let a822 = f164(v11.clone());
            if nat_le(a821, a822) {
                f112(v11)
            } else {
                Ok(false)
            }
        }
        Adt3::C7(r850) => {
            let v13 = (*r850).clone();
            let a823 = 12u64;
            let a824 = f164(v13.clone());
            if nat_le(a823, a824) {
                f112(v13)
            } else {
                Ok(false)
            }
        }
        Adt3::C8(r851, r852) => {
            let v14 = (*r851).clone();
            let v15 = (*r852).clone();
            let a825 = 6u64;
            let a826 = f164(v14.clone());
            if if nat_le(a825, a826) {
                f112(v14)?
            } else {
                false
            } {
                Ok(true)
            } else {
                let a827 = 6u64;
                let a828 = f164(v15.clone());
                if nat_le(a827, a828) {
                    f112(v15)
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
        Adt3::C11(r853, r854) => {
            let v21 = (*r853).clone();
            let v22 = (*r854).clone();
            if f112(v21)? {
                Ok(true)
            } else {
                f112(v22)
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
        Adt3::C18(r855) => {
            let v35 = (*r855).clone();
            f112(v35)
        }
        Adt3::C19(_) => {
            Ok(false)
        }
        Adt3::C20(_, _, _) => {
            Ok(true)
        }
        Adt3::C21(v40, r856, r857) => {
            let v41 = (*r856).clone();
            let v42 = (*r857).clone();
            if if f165(v40, v41.clone()) {
                false
            } else {
                f112(v41)?
            } {
                Ok(true)
            } else {
                let a831 = {
                    let a829 = f166(v40);
                    let a830 = 1u64;
                    nat_add(a829, a830)?
                };
                let a832 = f164(v42.clone());
                if nat_le(a831, a832) {
                    f112(v42)
                } else {
                    Ok(false)
                }
            }
        }
        Adt3::C22(r858) => {
            let v43 = (*r858).clone();
            let a833 = 12u64;
            let a834 = f164(v43.clone());
            if nat_le(a833, a834) {
                f112(v43)
            } else {
                Ok(false)
            }
        }
        Adt3::C23(r859, _) => {
            let v44 = (*r859).clone();
            let a835 = 11u64;
            let a836 = f164(v44.clone());
            if nat_le(a835, a836) {
                f112(v44)
            } else {
                Ok(false)
            }
        }
        Adt3::C24(r860, _) => {
            let v46 = (*r860).clone();
            let a837 = 13u64;
            let a838 = f164(v46.clone());
            if nat_le(a837, a838) {
                f112(v46)
            } else {
                Ok(false)
            }
        }
        Adt3::C25(r861, _, _) => {
            let v48 = (*r861).clone();
            let a839 = 13u64;
            let a840 = f164(v48.clone());
            if nat_le(a839, a840) {
                f112(v48)
            } else {
                Ok(false)
            }
        }
        Adt3::C26(r862) => {
            let v51 = (*r862).clone();
            let a841 = 12u64;
            let a842 = f164(v51.clone());
            if nat_le(a841, a842) {
                f112(v51)
            } else {
                Ok(false)
            }
        }
        Adt3::C27(r863) => {
            let v52 = (*r863).clone();
            let a843 = 12u64;
            let a844 = f164(v52.clone());
            if nat_le(a843, a844) {
                f112(v52)
            } else {
                Ok(false)
            }
        }
        Adt3::C28(r864, _, _, _) => {
            let v53 = (*r864).clone();
            let a845 = 13u64;
            let a846 = f164(v53.clone());
            if nat_le(a845, a846) {
                f112(v53)
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

pub fn f113(v0: Adt2) -> bool {
    let m865 = v0;
    match m865 {
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

pub fn f114(v0: List<Bytes>) -> bool {
    let m866 = v0;
    match m866.uncons() {
        None => {
            false
        }
        Some((v1, v2)) => {
            let m867 = v2;
            match m867.uncons() {
                None => {
                    let m868 = f150(v1);
                    match m868 {
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

pub fn f115(v0: Adt14, v1: List<Bytes>, v2: Adt4) -> R<Option<Bytes>> {
    let m869 = f145(v1.clone(), 1u64);
    match m869 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v3) => {
            let m870 = f95(v0, f144(v1.clone()));
            match m870 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v4) => {
                    let m872 = f146(match v4 {
                        Adt12::C0(_, h871, _) => {
                            h871
                        }
                    }, v3);
                    match m872 {
                        None => {
                            Ok(Some(f12(f13(Str::lit("runtime support: pattern-fields: ")), f12(f14(f27(v1, Str::lit("::"))?), f12(f13(Str::lit(" is matched with ")), f12(f39(f116(v2)?)?, f13(Str::lit(" patterns"))))))))
                        }
                        Some(v5) => {
                            let a873 = f167(v5)?;
                            let a874 = f116(v2.clone())?;
                            if nat_eq(a873, a874) {
                                Ok(None::<Bytes>)
                            } else {
                                Ok(Some(f12(f13(Str::lit("runtime support: pattern-fields: ")), f12(f14(f27(v1, Str::lit("::"))?), f12(f13(Str::lit(" is matched with ")), f12(f39(f116(v2)?)?, f13(Str::lit(" patterns"))))))))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f116(v0: Adt4) -> R<u64> {
    let m875 = v0;
    match m875 {
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
        Adt4::C9(_, r878) => {
            let v12 = (*r878).clone();
            let a876 = f116(v12)?;
            let a877 = 1u64;
            nat_add(a876, a877)
        }
    }
}

pub fn f117(v0: Adt14, v1: List<Bytes>, v2: List<Bytes>) -> R<Option<Bytes>> {
    let m879 = f61(v0, v1.clone());
    match m879 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v3) => {
            f80(Fn8::F168(v3, v1), None::<Bytes>, v2)
        }
    }
}

pub fn f118(v0: List<Adt9>, v1: Bytes) -> Option<List<Bytes>> {
    let m880 = f146(v0, v1);
    match m880 {
        None => {
            None::<List<Bytes>>
        }
        Some(v2) => {
            let m881 = v2;
            match m881 {
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

pub fn f119(v0: List<Bytes>, v1: List<Bytes>, v2: bool, v3: Bytes) -> R<bool> {
    if v2 {
        let a882 = f169(v1, v3.clone())?;
        let a883 = f169(v0, v3)?;
        Ok(nat_eq(a882, a883))
    } else {
        Ok(false)
    }
}

pub fn f120(v0: Adt14, v1: List<Adt6>, v2: u64, v3: bool) -> R<Option<Bytes>> {
    f170(Fn22::F171(v3, v2, v0), None::<Bytes>, v1)
}

pub fn f121(v0: Adt7, v1: Adt14, v2: Option<Bytes>, v3: Bytes) -> Option<Bytes> {
    let m884 = v2;
    match m884 {
        None => {
            let m885 = f122(v1, v3.clone());
            if match m885 {
                None => {
                    let m886 = f123(v3.clone());
                    match m886 {
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
                    Adt7::C0(h887, _) => {
                        h887
                    }
                }, f12(f13(Str::lit(": ")), v3))), f13(Str::lit(" names no declared trait")))))
            }
        }
        Some(v4) => {
            Some(v4)
        }
    }
}

pub fn f122(v0: Adt14, v1: Bytes) -> Option<Adt10> {
    f172(Fn23::F173(v1), None::<Adt10>, match v0 {
        Adt14::C0(_, _, h888, _, _, _, _, _, _, _) => {
            h888
        }
    })
}

pub fn f123(v0: Bytes) -> Option<Bytes> {
    let a889 = v0.clone();
    let a890 = f13(Str::lit("Clone"));
    if equal(a889, a890) {
        Some(f13(Str::lit("clone")))
    } else {
        let a891 = v0;
        let a892 = f13(Str::lit("Drop"));
        if equal(a891, a892) {
            Some(f13(Str::lit("drop")))
        } else {
            None::<Bytes>
        }
    }
}

pub fn f124(v0: Bytes) -> bool {
    let a893 = v0.clone();
    let a894 = f13(Str::lit("Clone"));
    if equal(a893, a894) {
        false
    } else {
        let a895 = v0;
        let a896 = f13(Str::lit("Drop"));
        equal(a895, a896)
    }
}

pub fn f125(v0: List<Adt8>, v1: Bytes) -> R<u64> {
    f174(Fn24::F175(v1), 0u64, v0)
}

pub fn f126(v0: List<Adt8>, v1: Bytes) -> Option<Adt8> {
    f176(Fn25::F177(v1), None::<Adt8>, v0)
}

pub fn f127(v0: Adt5) -> R<Bytes> {
    Ok(f12(match v0.clone() {
        Adt5::C0(h897, _, _, _, _) => {
            h897
        }
    }, f12(f13(Str::lit("|")), f12(if match v0.clone() {
        Adt5::C0(_, h898, _, _, _) => {
            h898
        }
    } {
        f13(Str::lit("r"))
    } else {
        f13(Str::lit("-"))
    }, f12(if match v0.clone() {
        Adt5::C0(_, _, h899, _, _) => {
            h899
        }
    } {
        f13(Str::lit("x"))
    } else {
        f13(Str::lit("-"))
    }, f12(f13(Str::lit("|")), f12(f178(Fn26::F179, f13(Str::lit("")), match v0.clone() {
        Adt5::C0(_, _, _, h900, _) => {
            h900
        }
    })?, f12(f13(Str::lit("|")), f180(match v0 {
        Adt5::C0(_, _, _, _, h901) => {
            h901
        }
    })?))))))))
}

pub fn f128(v0: List<Adt5>, v1: Bytes) -> bool {
    f181(Fn27::F182(v1), false, v0)
}

pub fn f129(v0: Fn14, p905: Result<List<Adt16>, Bytes>, p906: List<Adt6>) -> Result<List<Adt16>, Bytes> {
    let m904 = (p905, p906);
    {
        let mut m904 = m904;
        loop {
            let step = {
                let (v1, v2) = m904;
                let m902 = v2;
                match m902.uncons() {
                    None => {
                        Err::<(Result<List<Adt16>, Bytes>, List<Adt6>), Result<List<Adt16>, Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Result<List<Adt16>, Bytes>, List<Adt6>), Result<List<Adt16>, Bytes>>(({
                            let c903: Fn14 = v0.clone();
                            c903.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m904 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f130(v0: Bytes, v1: Result<List<Adt16>, Bytes>, v2: Adt6) -> Result<List<Adt16>, Bytes> {
    let m907 = v1;
    match m907 {
        Ok(v4) => {
            let m909 = f44(v4.clone(), match v2.clone() {
                Adt6::C0(h908, _, _) => {
                    h908
                }
            });
            match m909 {
                None => {
                    Ok::<List<Adt16>, Bytes>(List::cons(Adt16::C0(match v2.clone() {
                        Adt6::C0(h911, _, _) => {
                            h911
                        }
                    }, match v2.clone() {
                        Adt6::C0(_, h912, _) => {
                            h912
                        }
                    }, match v2 {
                        Adt6::C0(_, _, h913) => {
                            h913
                        }
                    }), v4))
                }
                Some(_) => {
                    Err::<List<Adt16>, Bytes>(f12(f13(Str::lit("runtime support: param-duplicate: ")), f12(f14(match v2 {
                        Adt6::C0(h910, _, _) => {
                            h910
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

pub fn f131(v0: Bytes, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m914 = v1;
    match m914 {
        None => {
            Some(v2)
        }
        Some(v3) => {
            Some({
                let a917 = {
                    let a915 = v3;
                    let a916 = v0;
                    append_bytes(a915, a916)
                };
                let a918 = v2;
                append_bytes(a917, a918)
            })
        }
    }
}

pub fn f132(v0: Fn15, p923: u64, p924: u64) -> R<(u64, bool)> {
    let m922 = (p923, p924);
    {
        let mut m922 = m922;
        loop {
            let step = {
                let (v1, v2) = m922;
                let m919 = v1;
                if m919 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m919 - 1;
                    let m921 = {
                        let c920: Fn15 = v0.clone();
                        c920.apply(v2)?
                    };
                    match m921 {
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
                Ok(next) => { m922 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f133(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f183(v0, v1) {
        Ok(Some({
            let a925 = v1;
            let a926 = 1u64;
            nat_add(a925, a926)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f134(v0: Str) -> Bytes {
    let a927 = v0;
    utf8_encode(a927)
}

pub fn f135(v0: Bytes) -> R<Bytes> {
    let v1: u64 = {
        let (h928, _) = f132(Fn15::F184(v0.clone()), {
            let a929 = v0.clone();
            length_bytes(a929)
        }, 0u64)?;
        h928
    };
    Ok(f185(v0.clone(), v1, {
        let a931 = {
            let a930 = v0;
            length_bytes(a930)
        };
        let a932 = v1;
        nat_sub(a931, a932)
    }))
}

pub fn f136(v0: u64, v1: u64) -> Bytes {
    f185(f134(Str::lit("0123456789")), {
        let a936 = {
            let a933 = v0;
            let a934 = v1;
            let a935 = 0u64;
            nat_quot(a933, a934, a935)
        };
        let a937 = 10u64;
        let a938 = 0u64;
        nat_rem(a936, a937, a938)
    }, 1u64)
}

pub fn f137(v0: Adt9) -> Bytes {
    let m939 = v0;
    match m939 {
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

pub fn f138(v0: Fn16, p943: Option<Adt11>, p944: List<Adt11>) -> Option<Adt11> {
    let m942 = (p943, p944);
    {
        let mut m942 = m942;
        loop {
            let step = {
                let (v1, v2) = m942;
                let m940 = v2;
                match m940.uncons() {
                    None => {
                        Err::<(Option<Adt11>, List<Adt11>), Option<Adt11>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Adt11>, List<Adt11>), Option<Adt11>>(({
                            let c941: Fn16 = v0.clone();
                            c941.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m942 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f139(v0: Bytes, v1: Option<Adt11>, v2: Adt11) -> Option<Adt11> {
    let m945 = v1;
    match m945 {
        None => {
            if f25(match v2.clone() {
                Adt11::C0(h946, _, _) => {
                    h946
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

pub fn f140(v0: Fn17, p950: Option<Adt12>, p951: List<Adt12>) -> Option<Adt12> {
    let m949 = (p950, p951);
    {
        let mut m949 = m949;
        loop {
            let step = {
                let (v1, v2) = m949;
                let m947 = v2;
                match m947.uncons() {
                    None => {
                        Err::<(Option<Adt12>, List<Adt12>), Option<Adt12>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Adt12>, List<Adt12>), Option<Adt12>>(({
                            let c948: Fn17 = v0.clone();
                            c948.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m949 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f141(v0: Bytes, v1: Option<Adt12>, v2: Adt12) -> Option<Adt12> {
    let m952 = v1;
    match m952 {
        None => {
            if f25(match v2.clone() {
                Adt12::C0(h953, _, _) => {
                    h953
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

pub fn f142(v0: Fn18, p957: Option<Adt13>, p958: List<Adt13>) -> Option<Adt13> {
    let m956 = (p957, p958);
    {
        let mut m956 = m956;
        loop {
            let step = {
                let (v1, v2) = m956;
                let m954 = v2;
                match m954.uncons() {
                    None => {
                        Err::<(Option<Adt13>, List<Adt13>), Option<Adt13>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Adt13>, List<Adt13>), Option<Adt13>>(({
                            let c955: Fn18 = v0.clone();
                            c955.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m956 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f143(v0: Bytes, v1: Option<Adt13>, v2: Adt13) -> Option<Adt13> {
    let m959 = v1;
    match m959 {
        None => {
            if f25(match v2.clone() {
                Adt13::C0(h960, _) => {
                    h960
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

pub fn f144(v0: List<Bytes>) -> Bytes {
    let m961 = v0;
    match m961.uncons() {
        None => {
            f13(Str::lit(""))
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f145(p965: List<Bytes>, p966: u64) -> Option<Bytes> {
    let m964 = (p965, p966);
    {
        let mut m964 = m964;
        loop {
            let step = {
                let (v0, v1) = m964;
                let m962 = v0;
                match m962.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        let m963 = v1;
                        if m963 == 0 {
                            Err::<(List<Bytes>, u64), Option<Bytes>>(Some(v2))
                        } else {
                            let v4 = m963 - 1;
                            Ok::<(List<Bytes>, u64), Option<Bytes>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m964 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f146(v0: List<Adt9>, v1: Bytes) -> Option<Adt9> {
    f186(Fn28::F187(v1), None::<Adt9>, v0)
}

pub fn f147(v0: Bytes) -> bool {
    let a967 = v0.clone();
    let a968 = f13(Str::lit("u8"));
    if equal(a967, a968) {
        true
    } else {
        let a969 = v0.clone();
        let a970 = f13(Str::lit("u16"));
        if equal(a969, a970) {
            true
        } else {
            let a971 = v0.clone();
            let a972 = f13(Str::lit("u32"));
            if equal(a971, a972) {
                true
            } else {
                let a973 = v0.clone();
                let a974 = f13(Str::lit("u64"));
                if equal(a973, a974) {
                    true
                } else {
                    let a975 = v0.clone();
                    let a976 = f13(Str::lit("u128"));
                    if equal(a975, a976) {
                        true
                    } else {
                        let a977 = v0.clone();
                        let a978 = f13(Str::lit("i8"));
                        if equal(a977, a978) {
                            true
                        } else {
                            let a979 = v0.clone();
                            let a980 = f13(Str::lit("i16"));
                            if equal(a979, a980) {
                                true
                            } else {
                                let a981 = v0.clone();
                                let a982 = f13(Str::lit("i32"));
                                if equal(a981, a982) {
                                    true
                                } else {
                                    let a983 = v0.clone();
                                    let a984 = f13(Str::lit("i64"));
                                    if equal(a983, a984) {
                                        true
                                    } else {
                                        let a985 = v0;
                                        let a986 = f13(Str::lit("i128"));
                                        equal(a985, a986)
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

pub fn f148(v0: Bytes, v1: Bytes) -> bool {
    let m987 = f188(v0.clone(), v1.clone());
    match m987 {
        None => {
            if f147(v0) {
                f189(f13(Str::lit("usize")), v1)
            } else {
                false
            }
        }
        Some(_) => {
            f189(v0, v1)
        }
    }
}

pub fn f149(v0: Adt14, v1: Bytes) -> Option<Adt13> {
    f142(Fn18::F190(v1), None::<Adt13>, match v0 {
        Adt14::C0(_, _, _, _, _, _, _, h988, _, _) => {
            h988
        }
    })
}

pub fn f150(v0: Bytes) -> Option<u64> {
    let a989 = v0.clone();
    let a990 = f13(Str::lit("Some"));
    if equal(a989, a990) {
        Some(1u64)
    } else {
        let a991 = v0.clone();
        let a992 = f13(Str::lit("Ok"));
        if equal(a991, a992) {
            Some(1u64)
        } else {
            let a993 = v0;
            let a994 = f13(Str::lit("Err"));
            if equal(a993, a994) {
                Some(1u64)
            } else {
                None::<u64>
            }
        }
    }
}

pub fn f151(v0: Adt14, v1: Bytes) -> Option<u64> {
    let m995 = f94(v0, v1);
    match m995 {
        None => {
            None::<u64>
        }
        Some(v2) => {
            Some({
                let a997 = match v2 {
                    Adt11::C0(_, h996, _) => {
                        h996
                    }
                };
                length_list(a997)
            })
        }
    }
}

pub fn f152(v0: Bytes, v1: Bytes) -> Option<u64> {
    let m998 = f188(v0.clone(), v1.clone());
    match m998 {
        None => {
            if f147(v0) {
                f188(f13(Str::lit("usize")), v1)
            } else {
                None::<u64>
            }
        }
        Some(v2) => {
            Some(v2)
        }
    }
}

pub fn f153(v0: Adt14, v1: Bytes, v2: Bytes) -> R<Option<u64>> {
    let m999 = f95(v0, v1);
    match m999 {
        None => {
            Ok(None::<u64>)
        }
        Some(v3) => {
            f191(match v3 {
                Adt12::C0(_, h1000, _) => {
                    h1000
                }
            }, v2)
        }
    }
}

pub fn f154(v0: Fn19, p1004: List<u64>, p1005: List<Adt13>) -> List<u64> {
    let m1003 = (p1004, p1005);
    {
        let mut m1003 = m1003;
        loop {
            let step = {
                let (v1, v2) = m1003;
                let m1001 = v2;
                match m1001.uncons() {
                    None => {
                        Err::<(List<u64>, List<Adt13>), List<u64>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<u64>, List<Adt13>), List<u64>>(({
                            let c1002: Fn19 = v0.clone();
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

pub fn f155(v0: Bytes, v1: List<u64>, v2: Adt13) -> List<u64> {
    if f25(match v2.clone() {
        Adt13::C0(h1006, _) => {
            h1006
        }
    }, v0) {
        let a1008 = v1;
        let a1009 = List::cons(match v2 {
            Adt13::C0(_, h1007) => {
                h1007
            }
        }, List::<u64>::nil());
        append_list(a1008, a1009)
    } else {
        v1
    }
}

pub fn f156(v0: Fn20, p1013: bool, p1014: List<u64>) -> bool {
    let m1012 = (p1013, p1014);
    {
        let mut m1012 = m1012;
        loop {
            let step = {
                let (v1, v2) = m1012;
                let m1010 = v2;
                match m1010.uncons() {
                    None => {
                        Err::<(bool, List<u64>), bool>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<u64>), bool>(({
                            let c1011: Fn20 = v0.clone();
                            c1011.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1012 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f157(v0: u64, v1: bool, v2: u64) -> bool {
    if v1 {
        true
    } else {
        let a1015 = v2;
        let a1016 = v0;
        nat_eq(a1015, a1016)
    }
}

pub fn f158(p1027: Adt2) -> Adt2 {
    let m1026 = p1027;
    {
        let mut m1026 = m1026;
        loop {
            let step = {
                let v0 = m1026;
                let m1017 = v0;
                match m1017 {
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
                    Adt2::C7(r1018) => {
                        let v2 = (*r1018).clone();
                        Ok::<Adt2, Adt2>(v2)
                    }
                    Adt2::C8(r1019) => {
                        let v3 = (*r1019).clone();
                        Ok::<Adt2, Adt2>(v3)
                    }
                    Adt2::C9(r1020, r1021) => {
                        let v4 = (*r1020).clone();
                        let v5 = (*r1021).clone();
                        Err::<Adt2, Adt2>(Adt2::C9(std::rc::Rc::new(v4), std::rc::Rc::new(v5)))
                    }
                    Adt2::C10(v6, r1022) => {
                        let v7 = (*r1022).clone();
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
                    Adt2::C15(r1023) => {
                        let v8 = (*r1023).clone();
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
                    Adt2::C19(r1024, r1025) => {
                        let v9 = (*r1024).clone();
                        let v10 = (*r1025).clone();
                        Err::<Adt2, Adt2>(Adt2::C19(std::rc::Rc::new(v9), std::rc::Rc::new(v10)))
                    }
                }
            };
            match step {
                Ok(next) => { m1026 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f159(v0: Adt15, v1: Adt3) -> Adt2 {
    let m1028 = v1;
    match m1028 {
        Adt3::C0(v2) => {
            let m1030 = f44(match v0 {
                Adt15::C0(_, h1029, _, _) => {
                    h1029
                }
            }, v2);
            match m1030 {
                None => {
                    Adt2::C16
                }
                Some(v3) => {
                    match v3 {
                        Adt16::C0(_, _, h1031) => {
                            h1031
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
        Adt3::C5(r1032, v12) => {
            let v11 = (*r1032).clone();
            f192(f193(v0.clone(), f158(f159(v0, v11))), v12)
        }
        Adt3::C6(_, _) => {
            Adt2::C16
        }
        Adt3::C7(r1033) => {
            let v15 = (*r1033).clone();
            let v16: Adt2 = f159(v0, v15);
            if f194(v16.clone()) {
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

pub fn f160(v0: Adt14, v1: List<Bytes>, v2: Bytes) -> R<bool> {
    f101(Fn13::F195(v2, v0), false, v1)
}

pub fn f161(p1038: List<Adt7>, p1039: u64) -> List<Bytes> {
    let m1037 = (p1038, p1039);
    {
        let mut m1037 = m1037;
        loop {
            let step = {
                let (v0, v1) = m1037;
                let m1034 = v0;
                match m1034.uncons() {
                    None => {
                        Err::<(List<Adt7>, u64), List<Bytes>>(List::<Bytes>::nil())
                    }
                    Some((v2, v3)) => {
                        let m1035 = v1;
                        if m1035 == 0 {
                            match v2 {
                                Adt7::C0(_, h1036) => {
                                    Err::<(List<Adt7>, u64), List<Bytes>>(h1036)
                                }
                            }
                        } else {
                            let v4 = m1035 - 1;
                            Ok::<(List<Adt7>, u64), List<Bytes>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1037 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f162(v0: Fn21, p1043: Bytes, p1044: List<u64>) -> R<Bytes> {
    let m1042 = (p1043, p1044);
    {
        let mut m1042 = m1042;
        loop {
            let step = {
                let (v1, v2) = m1042;
                let m1040 = v2;
                match m1040.uncons() {
                    None => {
                        Err::<(Bytes, List<u64>), R<Bytes>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Bytes, List<u64>), R<Bytes>>(({
                            let c1041: Fn21 = v0.clone();
                            c1041.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1042 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f163(v0: Bytes, v1: u64) -> R<Bytes> {
    let a1045 = v0.clone();
    let a1046 = f13(Str::lit(""));
    if equal(a1045, a1046) {
        f39(v1)
    } else {
        Ok(f12(v0, f12(f13(Str::lit("/")), f39(v1)?)))
    }
}

pub fn f164(v0: Adt3) -> u64 {
    let m1047 = v0;
    match m1047 {
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
            f166(v40)
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

pub fn f165(v0: u64, v1: Adt3) -> bool {
    let a1048 = v0;
    let a1049 = 8u64;
    if if nat_eq(a1048, a1049) {
        f196(v1.clone())
    } else {
        false
    } {
        true
    } else {
        let a1052 = f164(v1);
        let a1053 = {
            let a1050 = f166(v0);
            let a1051 = 5u64;
            if nat_eq(a1050, a1051) {
                6u64
            } else {
                f166(v0)
            }
        };
        nat_lt(a1052, a1053)
    }
}

pub fn f166(v0: u64) -> u64 {
    let a1054 = v0;
    let a1055 = 6u64;
    if nat_eq(a1054, a1055) {
        3u64
    } else {
        let a1056 = v0;
        let a1057 = 5u64;
        if nat_eq(a1056, a1057) {
            4u64
        } else {
            let a1058 = v0;
            let a1059 = 7u64;
            if nat_eq(a1058, a1059) {
                5u64
            } else {
                let a1060 = v0;
                let a1061 = 8u64;
                if nat_eq(a1060, a1061) {
                    5u64
                } else {
                    let a1062 = v0;
                    let a1063 = 9u64;
                    if nat_eq(a1062, a1063) {
                        5u64
                    } else {
                        let a1064 = v0;
                        let a1065 = 10u64;
                        if nat_eq(a1064, a1065) {
                            5u64
                        } else {
                            let a1066 = v0;
                            let a1067 = 11u64;
                            if nat_eq(a1066, a1067) {
                                5u64
                            } else {
                                let a1068 = v0;
                                let a1069 = 0u64;
                                if nat_eq(a1068, a1069) {
                                    9u64
                                } else {
                                    let a1070 = v0;
                                    let a1071 = 1u64;
                                    if nat_eq(a1070, a1071) {
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

pub fn f167(v0: Adt9) -> R<u64> {
    let m1072 = v0;
    match m1072 {
        Adt9::C0(_) => {
            Ok(0u64)
        }
        Adt9::C1(_, v3) => {
            f38(v3)
        }
        Adt9::C2(_, _, v6) => {
            f38(v6)
        }
    }
}

pub fn f168(v0: List<Bytes>, v1: List<Bytes>, v2: Option<Bytes>, v3: Bytes) -> R<Option<Bytes>> {
    let m1073 = v2;
    match m1073 {
        None => {
            if f45(v0, v3.clone())? {
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

pub fn f169(v0: List<Bytes>, v1: Bytes) -> R<u64> {
    f197(Fn29::F198(v1), 0u64, v0)
}

pub fn f170(v0: Fn22, p1077: Option<Bytes>, p1078: List<Adt6>) -> R<Option<Bytes>> {
    let m1076 = (p1077, p1078);
    {
        let mut m1076 = m1076;
        loop {
            let step = {
                let (v1, v2) = m1076;
                let m1074 = v2;
                match m1074.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Adt6>), R<Option<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Adt6>), R<Option<Bytes>>>(({
                            let c1075: Fn22 = v0.clone();
                            c1075.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1076 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f171(v0: bool, v1: u64, v2: Adt14, v3: Option<Bytes>, v4: Adt6) -> R<Option<Bytes>> {
    let m1079 = v3;
    match m1079 {
        None => {
            let m1081 = f32(Str::lit("a parameter name"), match v4.clone() {
                Adt6::C0(h1080, _, _) => {
                    h1080
                }
            })?;
            match m1081 {
                None => {
                    f8(v2, match v4 {
                        Adt6::C0(_, _, h1082) => {
                            h1082
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

pub fn f172(v0: Fn23, p1086: Option<Adt10>, p1087: List<Adt10>) -> Option<Adt10> {
    let m1085 = (p1086, p1087);
    {
        let mut m1085 = m1085;
        loop {
            let step = {
                let (v1, v2) = m1085;
                let m1083 = v2;
                match m1083.uncons() {
                    None => {
                        Err::<(Option<Adt10>, List<Adt10>), Option<Adt10>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Adt10>, List<Adt10>), Option<Adt10>>(({
                            let c1084: Fn23 = v0.clone();
                            c1084.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1085 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f173(v0: Bytes, v1: Option<Adt10>, v2: Adt10) -> Option<Adt10> {
    let m1088 = v1;
    match m1088 {
        None => {
            if f25(match v2.clone() {
                Adt10::C0(h1089, _) => {
                    h1089
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

pub fn f174(v0: Fn24, p1093: u64, p1094: List<Adt8>) -> R<u64> {
    let m1092 = (p1093, p1094);
    {
        let mut m1092 = m1092;
        loop {
            let step = {
                let (v1, v2) = m1092;
                let m1090 = v2;
                match m1090.uncons() {
                    None => {
                        Err::<(u64, List<Adt8>), R<u64>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(u64, List<Adt8>), R<u64>>(({
                            let c1091: Fn24 = v0.clone();
                            c1091.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1092 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f175(v0: Bytes, v1: u64, v2: Adt8) -> R<u64> {
    if f25({
        let m1097 = match v2 {
            Adt8::C0(h1096, _, _, _) => {
                h1096
            }
        };
        match m1097 {
            Adt5::C0(h1095, _, _, _, _) => {
                h1095
            }
        }
    }, v0) {
        let a1098 = v1;
        let a1099 = 1u64;
        nat_add(a1098, a1099)
    } else {
        Ok(v1)
    }
}

pub fn f176(v0: Fn25, p1103: Option<Adt8>, p1104: List<Adt8>) -> Option<Adt8> {
    let m1102 = (p1103, p1104);
    {
        let mut m1102 = m1102;
        loop {
            let step = {
                let (v1, v2) = m1102;
                let m1100 = v2;
                match m1100.uncons() {
                    None => {
                        Err::<(Option<Adt8>, List<Adt8>), Option<Adt8>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Adt8>, List<Adt8>), Option<Adt8>>(({
                            let c1101: Fn25 = v0.clone();
                            c1101.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1102 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f177(v0: Bytes, v1: Option<Adt8>, v2: Adt8) -> Option<Adt8> {
    let m1105 = v1;
    match m1105 {
        None => {
            if f25({
                let m1108 = match v2.clone() {
                    Adt8::C0(h1107, _, _, _) => {
                        h1107
                    }
                };
                match m1108 {
                    Adt5::C0(h1106, _, _, _, _) => {
                        h1106
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

pub fn f178(v0: Fn26, p1112: Bytes, p1113: List<Adt6>) -> R<Bytes> {
    let m1111 = (p1112, p1113);
    {
        let mut m1111 = m1111;
        loop {
            let step = {
                let (v1, v2) = m1111;
                let m1109 = v2;
                match m1109.uncons() {
                    None => {
                        Err::<(Bytes, List<Adt6>), R<Bytes>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Bytes, List<Adt6>), R<Bytes>>(({
                            let c1110: Fn26 = v0.clone();
                            c1110.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1111 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f179(v0: Bytes, v1: Adt6) -> R<Bytes> {
    Ok(f12(v0, f12(match v1.clone() {
        Adt6::C0(h1114, _, _) => {
            h1114
        }
    }, f12(if match v1.clone() {
        Adt6::C0(_, h1115, _) => {
            h1115
        }
    } {
        f13(Str::lit(":m:"))
    } else {
        f13(Str::lit(":-:"))
    }, f12(f180(match v1 {
        Adt6::C0(_, _, h1116) => {
            h1116
        }
    })?, f13(Str::lit(",")))))))
}

pub fn f180(v0: Adt2) -> R<Bytes> {
    let m1117 = v0;
    match m1117 {
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
            Ok(f12(f13(Str::lit("P")), f12(f39(v1)?, f13(Str::lit(";")))))
        }
        Adt2::C7(r1118) => {
            let v2 = (*r1118).clone();
            Ok(f12(f13(Str::lit("R(")), f12(f180(v2)?, f13(Str::lit(")")))))
        }
        Adt2::C8(r1119) => {
            let v3 = (*r1119).clone();
            Ok(f12(f13(Str::lit("M(")), f12(f180(v3)?, f13(Str::lit(")")))))
        }
        Adt2::C9(r1120, r1121) => {
            let v4 = (*r1120).clone();
            let v5 = (*r1121).clone();
            Ok(f12(f13(Str::lit("T(")), f12(f180(v4)?, f12(f13(Str::lit(",")), f12(f180(v5)?, f13(Str::lit(")")))))))
        }
        Adt2::C10(v6, r1122) => {
            let v7 = (*r1122).clone();
            Ok(f12(f13(Str::lit("D")), f12(f14(v6), f12(f13(Str::lit("(")), f12(f180(v7)?, f13(Str::lit(")")))))))
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
        Adt2::C15(r1123) => {
            let v8 = (*r1123).clone();
            Ok(f12(f13(Str::lit("L(")), f12(f180(v8)?, f13(Str::lit(")")))))
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
        Adt2::C19(r1124, r1125) => {
            let v9 = (*r1124).clone();
            let v10 = (*r1125).clone();
            Ok(f12(f180(v9)?, f12(f13(Str::lit(";")), f180(v10)?)))
        }
    }
}

pub fn f181(v0: Fn27, p1129: bool, p1130: List<Adt5>) -> bool {
    let m1128 = (p1129, p1130);
    {
        let mut m1128 = m1128;
        loop {
            let step = {
                let (v1, v2) = m1128;
                let m1126 = v2;
                match m1126.uncons() {
                    None => {
                        Err::<(bool, List<Adt5>), bool>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<Adt5>), bool>(({
                            let c1127: Fn27 = v0.clone();
                            c1127.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1128 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f182(v0: Bytes, v1: bool, v2: Adt5) -> bool {
    if v1 {
        true
    } else {
        f25(match v2 {
            Adt5::C0(h1131, _, _, _, _) => {
                h1131
            }
        }, v0)
    }
}

pub fn f183(v0: Bytes, v1: u64) -> bool {
    let m1132 = f199(v0, v1);
    match m1132 {
        None => {
            false
        }
        Some(v2) => {
            if f200(v2, 97u8, 122u8) {
                true
            } else {
                if f200(v2, 65u8, 90u8) {
                    true
                } else {
                    let a1133 = v2;
                    let a1134 = 95u8;
                    if equal(a1133, a1134) {
                        true
                    } else {
                        let a1135 = 0u64;
                        let a1136 = v1;
                        if nat_lt(a1135, a1136) {
                            f200(v2, 48u8, 57u8)
                        } else {
                            false
                        }
                    }
                }
            }
        }
    }
}

pub fn f184(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f201(v0, v1)
}

pub fn f185(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m1140 = {
        let a1137 = v0;
        let a1138 = v1;
        let a1139 = v2;
        slice_bytes(a1137, a1138, a1139)
    };
    match m1140 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f186(v0: Fn28, p1144: Option<Adt9>, p1145: List<Adt9>) -> Option<Adt9> {
    let m1143 = (p1144, p1145);
    {
        let mut m1143 = m1143;
        loop {
            let step = {
                let (v1, v2) = m1143;
                let m1141 = v2;
                match m1141.uncons() {
                    None => {
                        Err::<(Option<Adt9>, List<Adt9>), Option<Adt9>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Adt9>, List<Adt9>), Option<Adt9>>(({
                            let c1142: Fn28 = v0.clone();
                            c1142.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1143 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f187(v0: Bytes, v1: Option<Adt9>, v2: Adt9) -> Option<Adt9> {
    let m1146 = v1;
    match m1146 {
        None => {
            if f25(f137(v2.clone()), v0) {
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

pub fn f188(v0: Bytes, v1: Bytes) -> Option<u64> {
    let a1147 = v0.clone();
    let a1148 = f13(Str::lit("AtomicU64"));
    if if equal(a1147, a1148) {
        let a1149 = v1.clone();
        let a1150 = f13(Str::lit("new"));
        equal(a1149, a1150)
    } else {
        false
    } {
        Some(1u64)
    } else {
        let a1151 = v0.clone();
        let a1152 = f13(Str::lit("Rc"));
        if if equal(a1151, a1152) {
            let a1153 = v1.clone();
            let a1154 = f13(Str::lit("new"));
            equal(a1153, a1154)
        } else {
            false
        } {
            Some(1u64)
        } else {
            let a1155 = v0.clone();
            let a1156 = f13(Str::lit("Rc"));
            if if equal(a1155, a1156) {
                let a1157 = v1.clone();
                let a1158 = f13(Str::lit("from"));
                equal(a1157, a1158)
            } else {
                false
            } {
                Some(1u64)
            } else {
                let a1159 = v0.clone();
                let a1160 = f13(Str::lit("Rc"));
                if if equal(a1159, a1160) {
                    let a1161 = v1.clone();
                    let a1162 = f13(Str::lit("try_unwrap"));
                    equal(a1161, a1162)
                } else {
                    false
                } {
                    Some(1u64)
                } else {
                    let a1163 = v0.clone();
                    let a1164 = f13(Str::lit("Rc"));
                    if if equal(a1163, a1164) {
                        let a1165 = v1.clone();
                        let a1166 = f13(Str::lit("get_mut"));
                        equal(a1165, a1166)
                    } else {
                        false
                    } {
                        Some(1u64)
                    } else {
                        let a1167 = v0.clone();
                        let a1168 = f13(Str::lit("Vec"));
                        if if equal(a1167, a1168) {
                            let a1169 = v1.clone();
                            let a1170 = f13(Str::lit("new"));
                            equal(a1169, a1170)
                        } else {
                            false
                        } {
                            Some(0u64)
                        } else {
                            let a1171 = v0.clone();
                            let a1172 = f13(Str::lit("Vec"));
                            if if equal(a1171, a1172) {
                                let a1173 = v1.clone();
                                let a1174 = f13(Str::lit("with_capacity"));
                                equal(a1173, a1174)
                            } else {
                                false
                            } {
                                Some(1u64)
                            } else {
                                let a1175 = v0.clone();
                                let a1176 = f13(Str::lit("String"));
                                if if equal(a1175, a1176) {
                                    let a1177 = v1.clone();
                                    let a1178 = f13(Str::lit("new"));
                                    equal(a1177, a1178)
                                } else {
                                    false
                                } {
                                    Some(0u64)
                                } else {
                                    let a1179 = v0.clone();
                                    let a1180 = f13(Str::lit("mem"));
                                    if if equal(a1179, a1180) {
                                        let a1181 = v1.clone();
                                        let a1182 = f13(Str::lit("replace"));
                                        equal(a1181, a1182)
                                    } else {
                                        false
                                    } {
                                        Some(2u64)
                                    } else {
                                        let a1183 = v0.clone();
                                        let a1184 = f13(Str::lit("str"));
                                        if if equal(a1183, a1184) {
                                            let a1185 = v1.clone();
                                            let a1186 = f13(Str::lit("from_utf8"));
                                            equal(a1185, a1186)
                                        } else {
                                            false
                                        } {
                                            Some(1u64)
                                        } else {
                                            let a1187 = v0.clone();
                                            let a1188 = f13(Str::lit("usize"));
                                            if if equal(a1187, a1188) {
                                                let a1189 = v1.clone();
                                                let a1190 = f13(Str::lit("from"));
                                                equal(a1189, a1190)
                                            } else {
                                                false
                                            } {
                                                Some(1u64)
                                            } else {
                                                let a1191 = v0;
                                                let a1192 = f13(Str::lit("usize"));
                                                if if equal(a1191, a1192) {
                                                    let a1193 = v1;
                                                    let a1194 = f13(Str::lit("try_from"));
                                                    equal(a1193, a1194)
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

pub fn f189(v0: Bytes, v1: Bytes) -> bool {
    let a1195 = v0.clone();
    let a1196 = f13(Str::lit("AtomicU64"));
    if if equal(a1195, a1196) {
        let a1197 = v1.clone();
        let a1198 = f13(Str::lit("new"));
        equal(a1197, a1198)
    } else {
        false
    } {
        false
    } else {
        let a1199 = v0.clone();
        let a1200 = f13(Str::lit("Rc"));
        if if equal(a1199, a1200) {
            let a1201 = v1.clone();
            let a1202 = f13(Str::lit("new"));
            equal(a1201, a1202)
        } else {
            false
        } {
            true
        } else {
            let a1203 = v0.clone();
            let a1204 = f13(Str::lit("Rc"));
            if if equal(a1203, a1204) {
                let a1205 = v1.clone();
                let a1206 = f13(Str::lit("from"));
                equal(a1205, a1206)
            } else {
                false
            } {
                true
            } else {
                let a1207 = v0.clone();
                let a1208 = f13(Str::lit("Rc"));
                if if equal(a1207, a1208) {
                    let a1209 = v1.clone();
                    let a1210 = f13(Str::lit("try_unwrap"));
                    equal(a1209, a1210)
                } else {
                    false
                } {
                    true
                } else {
                    let a1211 = v0.clone();
                    let a1212 = f13(Str::lit("Rc"));
                    if if equal(a1211, a1212) {
                        let a1213 = v1.clone();
                        let a1214 = f13(Str::lit("get_mut"));
                        equal(a1213, a1214)
                    } else {
                        false
                    } {
                        true
                    } else {
                        let a1215 = v0.clone();
                        let a1216 = f13(Str::lit("Vec"));
                        if if equal(a1215, a1216) {
                            let a1217 = v1.clone();
                            let a1218 = f13(Str::lit("new"));
                            equal(a1217, a1218)
                        } else {
                            false
                        } {
                            true
                        } else {
                            let a1219 = v0.clone();
                            let a1220 = f13(Str::lit("Vec"));
                            if if equal(a1219, a1220) {
                                let a1221 = v1.clone();
                                let a1222 = f13(Str::lit("with_capacity"));
                                equal(a1221, a1222)
                            } else {
                                false
                            } {
                                true
                            } else {
                                let a1223 = v0.clone();
                                let a1224 = f13(Str::lit("String"));
                                if if equal(a1223, a1224) {
                                    let a1225 = v1.clone();
                                    let a1226 = f13(Str::lit("new"));
                                    equal(a1225, a1226)
                                } else {
                                    false
                                } {
                                    true
                                } else {
                                    let a1227 = v0.clone();
                                    let a1228 = f13(Str::lit("mem"));
                                    if if equal(a1227, a1228) {
                                        let a1229 = v1.clone();
                                        let a1230 = f13(Str::lit("replace"));
                                        equal(a1229, a1230)
                                    } else {
                                        false
                                    } {
                                        false
                                    } else {
                                        let a1231 = v0.clone();
                                        let a1232 = f13(Str::lit("str"));
                                        if if equal(a1231, a1232) {
                                            let a1233 = v1.clone();
                                            let a1234 = f13(Str::lit("from_utf8"));
                                            equal(a1233, a1234)
                                        } else {
                                            false
                                        } {
                                            false
                                        } else {
                                            let a1235 = v0.clone();
                                            let a1236 = f13(Str::lit("usize"));
                                            if if equal(a1235, a1236) {
                                                let a1237 = v1.clone();
                                                let a1238 = f13(Str::lit("from"));
                                                equal(a1237, a1238)
                                            } else {
                                                false
                                            } {
                                                false
                                            } else {
                                                let a1239 = v0;
                                                let a1240 = f13(Str::lit("usize"));
                                                let _ = if equal(a1239, a1240) {
                                                    let a1241 = v1;
                                                    let a1242 = f13(Str::lit("try_from"));
                                                    equal(a1241, a1242)
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

pub fn f190(v0: Bytes, v1: Option<Adt13>, v2: Adt13) -> Option<Adt13> {
    let m1243 = v1;
    match m1243 {
        None => {
            if f25(match v2.clone() {
                Adt13::C0(h1244, _) => {
                    h1244
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

pub fn f191(v0: List<Adt9>, v1: Bytes) -> R<Option<u64>> {
    let m1245 = f146(v0, v1);
    match m1245 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            let m1246 = v2;
            match m1246 {
                Adt9::C0(_) => {
                    Ok(None::<u64>)
                }
                Adt9::C1(_, v5) => {
                    Ok(Some(f38(v5)?))
                }
                Adt9::C2(_, _, _) => {
                    Ok(None::<u64>)
                }
            }
        }
    }
}

pub fn f192(v0: Adt2, v1: u64) -> Adt2 {
    let m1247 = v0;
    match m1247 {
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
        Adt2::C9(r1252, r1253) => {
            let v5 = (*r1252).clone();
            let v6 = (*r1253).clone();
            let a1248 = v1;
            let a1249 = 0u64;
            if nat_eq(a1248, a1249) {
                v5
            } else {
                let a1250 = v1;
                let a1251 = 1u64;
                if nat_eq(a1250, a1251) {
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

pub fn f193(v0: Adt15, v1: Adt2) -> Adt2 {
    let m1254 = v1.clone();
    match m1254 {
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
                Adt15::C0(_, _, h1255, _) => {
                    h1255
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

pub fn f194(v0: Adt2) -> bool {
    let m1256 = v0;
    match m1256 {
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

pub fn f195(v0: Bytes, v1: Adt14, v2: bool, v3: Bytes) -> bool {
    if v2 {
        true
    } else {
        let m1257 = f122(v1, v3.clone());
        match m1257 {
            None => {
                let m1259 = f123(v3);
                match m1259 {
                    None => {
                        false
                    }
                    Some(v5) => {
                        f25(v5, v0)
                    }
                }
            }
            Some(v4) => {
                f181(Fn27::F202(v0), false, match v4 {
                    Adt10::C0(_, h1258) => {
                        h1258
                    }
                })
            }
        }
    }
}

pub fn f196(p1265: Adt3) -> bool {
    let m1264 = p1265;
    {
        let mut m1264 = m1264;
        loop {
            let step = {
                let v0 = m1264;
                let m1260 = v0;
                match m1260 {
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
                    Adt3::C21(v40, _, r1263) => {
                        let v42 = (*r1263).clone();
                        let a1261 = f166(v40);
                        let a1262 = f164(v42.clone());
                        if nat_lt(a1261, a1262) {
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
                Ok(next) => { m1264 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f197(v0: Fn29, p1269: u64, p1270: List<Bytes>) -> R<u64> {
    let m1268 = (p1269, p1270);
    {
        let mut m1268 = m1268;
        loop {
            let step = {
                let (v1, v2) = m1268;
                let m1266 = v2;
                match m1266.uncons() {
                    None => {
                        Err::<(u64, List<Bytes>), R<u64>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(u64, List<Bytes>), R<u64>>(({
                            let c1267: Fn29 = v0.clone();
                            c1267.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1268 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f198(v0: Bytes, v1: u64, v2: Bytes) -> R<u64> {
    if f25(v2, v0) {
        let a1271 = v1;
        let a1272 = 1u64;
        nat_add(a1271, a1272)
    } else {
        Ok(v1)
    }
}

pub fn f199(v0: Bytes, v1: u64) -> Option<u8> {
    let a1273 = v0;
    let a1274 = v1;
    index_bytes(a1273, a1274)
}

pub fn f200(v0: u8, v1: u8, v2: u8) -> bool {
    let m1277 = {
        let a1275 = v0;
        let a1276 = v1;
        fixed_u8::checked_sub(a1275, a1276)
    };
    match m1277 {
        None => {
            false
        }
        Some(_) => {
            let m1280 = {
                let a1278 = v2;
                let a1279 = v0;
                fixed_u8::checked_sub(a1278, a1279)
            };
            match m1280 {
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

pub fn f201(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f203(v0, v1, 48u8) {
        Ok(Some({
            let a1281 = v1;
            let a1282 = 1u64;
            nat_add(a1281, a1282)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f202(v0: Bytes, v1: bool, v2: Adt5) -> bool {
    if v1 {
        true
    } else {
        f25(match v2 {
            Adt5::C0(h1283, _, _, _, _) => {
                h1283
            }
        }, v0)
    }
}

pub fn f203(v0: Bytes, v1: u64, v2: u8) -> bool {
    let m1284 = f199(v0, v1);
    match m1284 {
        None => {
            false
        }
        Some(v3) => {
            let a1285 = v3;
            let a1286 = v2;
            equal(a1285, a1286)
        }
    }
}
