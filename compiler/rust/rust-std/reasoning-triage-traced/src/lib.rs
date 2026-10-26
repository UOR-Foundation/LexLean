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
    C0(u64, u64, u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(Adt0, u64, u64, u64, u64, u64, u64, u64, u64),
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
}

#[derive(Clone)]
pub enum Adt3 {
    C0(u64, u64, u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt4 {
    C0(Adt1, List<Adt2>, Adt3),
}

#[derive(Clone)]
pub enum Fn0 {
    F2,
}

impl Fn0 {
    pub fn apply(&self, p0: Adt4) -> R<Option<Adt4>> {
        match self {
            Fn0::F2 => f2(p0),
        }
    }
}

pub fn f0(v0: Adt0) -> R<(Result<(u64, List<Adt2>), (bool, bool)>, (u64, u64))> {
    let v1: (Adt4, bool) = f41(Fn0::F2, 11u64, f1(v0.clone()))?;
    let v2: Adt4 = {
        let (h1, _) = v1.clone();
        h1
    };
    Ok(({
        let (_, h2) = v1.clone();
        if h2 {
            let m4 = f39(v0, match v2.clone() {
                Adt4::C0(h3, _, _) => {
                    h3
                }
            });
            match m4 {
                Ok(v3) => {
                    Ok::<(u64, List<Adt2>), (bool, bool)>((v3, match v2.clone() {
                        Adt4::C0(_, h5, _) => {
                            h5
                        }
                    }))
                }
                Err(v4) => {
                    Err::<(u64, List<Adt2>), (bool, bool)>(v4)
                }
            }
        } else {
            Err::<(u64, List<Adt2>), (bool, bool)>((false, false))
        }
    }, ({
        let (_, h6) = v1;
        if h6 {
            let a11 = {
                let m9 = match v2.clone() {
                    Adt4::C0(_, _, h8) => {
                        h8
                    }
                };
                match m9 {
                    Adt3::C0(_, h7, _, _, _, _) => {
                        h7
                    }
                }
            };
            let a12 = f3(match v2.clone() {
                Adt4::C0(h10, _, _) => {
                    h10
                }
            })?;
            nat_add(a11, a12)?
        } else {
            let m15 = match v2.clone() {
                Adt4::C0(_, _, h14) => {
                    h14
                }
            };
            match m15 {
                Adt3::C0(_, h13, _, _, _, _) => {
                    h13
                }
            }
        }
    }, {
        let m18 = match v2 {
            Adt4::C0(_, _, h17) => {
                h17
            }
        };
        match m18 {
            Adt3::C0(_, _, h16, _, _, _) => {
                h16
            }
        }
    })))
}

pub fn f1(v0: Adt0) -> Adt4 {
    Adt4::C0(f33(v0), List::<Adt2>::nil(), Adt3::C0(0u64, 0u64, 0u64, 0u64, 0u64, 0u64))
}

pub fn f2(v0: Adt4) -> R<Option<Adt4>> {
    let m20 = f34(match v0.clone() {
        Adt4::C0(h19, _, _) => {
            h19
        }
    })?;
    match m20 {
        None => {
            Ok(None::<Adt4>)
        }
        Some(v1) => {
            let m22 = f35(match v0.clone() {
                Adt4::C0(h21, _, _) => {
                    h21
                }
            }, v1.clone())?;
            match m22 {
                None => {
                    Ok(None::<Adt4>)
                }
                Some(v2) => {
                    Ok(Some(Adt4::C0(v2, {
                        let a24 = match v0.clone() {
                            Adt4::C0(_, h23, _) => {
                                h23
                            }
                        };
                        let a25 = List::cons(v1, List::<Adt2>::nil());
                        append_list(a24, a25)
                    }, Adt3::C0({
                        let a29 = {
                            let m28 = match v0.clone() {
                                Adt4::C0(_, _, h27) => {
                                    h27
                                }
                            };
                            match m28 {
                                Adt3::C0(h26, _, _, _, _, _) => {
                                    h26
                                }
                            }
                        };
                        let a30 = 1u64;
                        nat_add(a29, a30)?
                    }, {
                        let a35 = {
                            let m33 = match v0.clone() {
                                Adt4::C0(_, _, h32) => {
                                    h32
                                }
                            };
                            match m33 {
                                Adt3::C0(_, h31, _, _, _, _) => {
                                    h31
                                }
                            }
                        };
                        let a36 = f3(match v0.clone() {
                            Adt4::C0(h34, _, _) => {
                                h34
                            }
                        })?;
                        nat_add(a35, a36)?
                    }, {
                        let a40 = {
                            let m39 = match v0.clone() {
                                Adt4::C0(_, _, h38) => {
                                    h38
                                }
                            };
                            match m39 {
                                Adt3::C0(_, _, h37, _, _, _) => {
                                    h37
                                }
                            }
                        };
                        let a41 = 1u64;
                        nat_add(a40, a41)?
                    }, {
                        let m44 = match v0.clone() {
                            Adt4::C0(_, _, h43) => {
                                h43
                            }
                        };
                        match m44 {
                            Adt3::C0(_, _, _, h42, _, _) => {
                                h42
                            }
                        }
                    }, {
                        let m47 = match v0.clone() {
                            Adt4::C0(_, _, h46) => {
                                h46
                            }
                        };
                        match m47 {
                            Adt3::C0(_, _, _, _, h45, _) => {
                                h45
                            }
                        }
                    }, {
                        let m50 = match v0 {
                            Adt4::C0(_, _, h49) => {
                                h49
                            }
                        };
                        match m50 {
                            Adt3::C0(_, _, _, _, _, h48) => {
                                h48
                            }
                        }
                    }))))
                }
            }
        }
    }
}

pub fn f3(v0: Adt1) -> R<u64> {
    if f9(v0.clone()) {
        Ok(1u64)
    } else {
        let a65 = if f10(v0.clone()) {
            1u64
        } else {
            let a63 = if f11(v0.clone()) {
                1u64
            } else {
                let a61 = if f12(v0.clone()) {
                    1u64
                } else {
                    let a59 = if f13(v0.clone())? {
                        1u64
                    } else {
                        let a57 = if f14(v0.clone()) {
                            1u64
                        } else {
                            let a55 = if f15(v0.clone()) {
                                1u64
                            } else {
                                let a53 = if f16(v0) {
                                    1u64
                                } else {
                                    let a51 = 0u64;
                                    let a52 = 1u64;
                                    nat_add(a51, a52)?
                                };
                                let a54 = 1u64;
                                nat_add(a53, a54)?
                            };
                            let a56 = 1u64;
                            nat_add(a55, a56)?
                        };
                        let a58 = 1u64;
                        nat_add(a57, a58)?
                    };
                    let a60 = 1u64;
                    nat_add(a59, a60)?
                };
                let a62 = 1u64;
                nat_add(a61, a62)?
            };
            let a64 = 1u64;
            nat_add(a63, a64)?
        };
        let a66 = 1u64;
        nat_add(a65, a66)
    }
}

pub fn f4(v0: Adt0) -> bool {
    let a68 = 380u64;
    let a69 = match v0 {
        Adt0::C0(h67, _, _, _, _, _) => {
            h67
        }
    };
    nat_lt(a68, a69)
}

pub fn f5(v0: Adt0) -> bool {
    let a71 = 90u64;
    let a72 = match v0 {
        Adt0::C0(_, h70, _, _, _, _) => {
            h70
        }
    };
    nat_lt(a71, a72)
}

pub fn f6(v0: Adt0) -> bool {
    let a74 = 20u64;
    let a75 = match v0 {
        Adt0::C0(_, _, h73, _, _, _) => {
            h73
        }
    };
    nat_lt(a74, a75)
}

pub fn f7(v0: Adt0) -> bool {
    let a77 = 12u64;
    let a78 = match v0 {
        Adt0::C0(_, _, _, h76, _, _) => {
            h76
        }
    };
    nat_lt(a77, a78)
}

pub fn f8(v0: Adt0) -> bool {
    let a80 = match v0 {
        Adt0::C0(_, _, _, _, h79, _) => {
            h79
        }
    };
    let a81 = 90u64;
    nat_lt(a80, a81)
}

pub fn f9(v0: Adt1) -> bool {
    let a86 = {
        let a83 = match v0.clone() {
            Adt1::C0(_, h82, _, _, _, _, _, _, _) => {
                h82
            }
        };
        let a84 = 0u64;
        nat_eq(a83, a84)
    };
    let a87 = f4(match v0 {
        Adt1::C0(h85, _, _, _, _, _, _, _, _) => {
            h85
        }
    });
    bool_and(a86, a87)
}

pub fn f10(v0: Adt1) -> bool {
    let a92 = {
        let a89 = match v0.clone() {
            Adt1::C0(_, _, h88, _, _, _, _, _, _) => {
                h88
            }
        };
        let a90 = 0u64;
        nat_eq(a89, a90)
    };
    let a93 = f5(match v0 {
        Adt1::C0(h91, _, _, _, _, _, _, _, _) => {
            h91
        }
    });
    bool_and(a92, a93)
}

pub fn f11(v0: Adt1) -> bool {
    let a98 = {
        let a95 = match v0.clone() {
            Adt1::C0(_, _, _, h94, _, _, _, _, _) => {
                h94
            }
        };
        let a96 = 0u64;
        nat_eq(a95, a96)
    };
    let a99 = f6(match v0 {
        Adt1::C0(h97, _, _, _, _, _, _, _, _) => {
            h97
        }
    });
    bool_and(a98, a99)
}

pub fn f12(v0: Adt1) -> bool {
    let a104 = {
        let a101 = match v0.clone() {
            Adt1::C0(_, _, _, _, h100, _, _, _, _) => {
                h100
            }
        };
        let a102 = 0u64;
        nat_eq(a101, a102)
    };
    let a105 = f7(match v0 {
        Adt1::C0(h103, _, _, _, _, _, _, _, _) => {
            h103
        }
    });
    bool_and(a104, a105)
}

pub fn f13(v0: Adt1) -> R<bool> {
    let a121 = {
        let a107 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h106, _, _, _) => {
                h106
            }
        };
        let a108 = 0u64;
        nat_eq(a107, a108)
    };
    let a122 = {
        let a119 = 2u64;
        let a120 = {
            let a117 = {
                let a114 = {
                    let a111 = match v0.clone() {
                        Adt1::C0(_, h109, _, _, _, _, _, _, _) => {
                            h109
                        }
                    };
                    let a112 = match v0.clone() {
                        Adt1::C0(_, _, h110, _, _, _, _, _, _) => {
                            h110
                        }
                    };
                    nat_add(a111, a112)?
                };
                let a115 = match v0.clone() {
                    Adt1::C0(_, _, _, h113, _, _, _, _, _) => {
                        h113
                    }
                };
                nat_add(a114, a115)?
            };
            let a118 = match v0 {
                Adt1::C0(_, _, _, _, h116, _, _, _, _) => {
                    h116
                }
            };
            nat_add(a117, a118)?
        };
        nat_le(a119, a120)
    };
    Ok(bool_and(a121, a122))
}

pub fn f14(v0: Adt1) -> bool {
    let a136 = {
        let a124 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h123, _, _) => {
                h123
            }
        };
        let a125 = 0u64;
        nat_eq(a124, a125)
    };
    let a137 = {
        let a134 = {
            let a127 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, h126, _, _, _) => {
                    h126
                }
            };
            let a128 = 1u64;
            nat_eq(a127, a128)
        };
        let a135 = {
            let a132 = {
                let m131 = match v0 {
                    Adt1::C0(h130, _, _, _, _, _, _, _, _) => {
                        h130
                    }
                };
                match m131 {
                    Adt0::C0(_, _, _, _, _, h129) => {
                        h129
                    }
                }
            };
            let a133 = 1u64;
            nat_eq(a132, a133)
        };
        bool_and(a134, a135)
    };
    bool_and(a136, a137)
}

pub fn f15(v0: Adt1) -> bool {
    let a147 = {
        let a139 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h138, _) => {
                h138
            }
        };
        let a140 = 0u64;
        nat_eq(a139, a140)
    };
    let a148 = {
        let a145 = {
            let a142 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h141, _, _) => {
                    h141
                }
            };
            let a143 = 1u64;
            nat_eq(a142, a143)
        };
        let a146 = f8(match v0 {
            Adt1::C0(h144, _, _, _, _, _, _, _, _) => {
                h144
            }
        });
        bool_and(a145, a146)
    };
    bool_and(a147, a148)
}

pub fn f16(v0: Adt1) -> bool {
    let a180 = {
        let a150 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h149) => {
                h149
            }
        };
        let a151 = 3u64;
        nat_lt(a150, a151)
    };
    let a181 = {
        let a178 = {
            let a158 = {
                let a153 = match v0.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h152) => {
                        h152
                    }
                };
                let a154 = 0u64;
                nat_eq(a153, a154)
            };
            let a159 = {
                let a156 = match v0.clone() {
                    Adt1::C0(_, _, _, _, _, h155, _, _, _) => {
                        h155
                    }
                };
                let a157 = 1u64;
                nat_eq(a156, a157)
            };
            bool_and(a158, a159)
        };
        let a179 = {
            let a176 = {
                let a166 = {
                    let a161 = match v0.clone() {
                        Adt1::C0(_, _, _, _, _, _, _, _, h160) => {
                            h160
                        }
                    };
                    let a162 = 1u64;
                    nat_eq(a161, a162)
                };
                let a167 = {
                    let a164 = match v0.clone() {
                        Adt1::C0(_, _, _, _, _, _, h163, _, _) => {
                            h163
                        }
                    };
                    let a165 = 1u64;
                    nat_eq(a164, a165)
                };
                bool_and(a166, a167)
            };
            let a177 = {
                let a174 = {
                    let a169 = match v0.clone() {
                        Adt1::C0(_, _, _, _, _, _, _, _, h168) => {
                            h168
                        }
                    };
                    let a170 = 2u64;
                    nat_eq(a169, a170)
                };
                let a175 = {
                    let a172 = match v0 {
                        Adt1::C0(_, _, _, _, _, _, _, h171, _) => {
                            h171
                        }
                    };
                    let a173 = 1u64;
                    nat_eq(a172, a173)
                };
                bool_and(a174, a175)
            };
            bool_or(a176, a177)
        };
        bool_or(a178, a179)
    };
    bool_and(a180, a181)
}

pub fn f17(v0: Adt1) -> Adt1 {
    Adt1::C0(match v0.clone() {
        Adt1::C0(h182, _, _, _, _, _, _, _, _) => {
            h182
        }
    }, 1u64, match v0.clone() {
        Adt1::C0(_, _, h183, _, _, _, _, _, _) => {
            h183
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h184, _, _, _, _, _) => {
            h184
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h185, _, _, _, _) => {
            h185
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, h186, _, _, _) => {
            h186
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h187, _, _) => {
            h187
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h188, _) => {
            h188
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, _, _, _, _, h189) => {
            h189
        }
    })
}

pub fn f18(v0: Adt1) -> Adt1 {
    Adt1::C0(match v0.clone() {
        Adt1::C0(h190, _, _, _, _, _, _, _, _) => {
            h190
        }
    }, match v0.clone() {
        Adt1::C0(_, h191, _, _, _, _, _, _, _) => {
            h191
        }
    }, 1u64, match v0.clone() {
        Adt1::C0(_, _, _, h192, _, _, _, _, _) => {
            h192
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h193, _, _, _, _) => {
            h193
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, h194, _, _, _) => {
            h194
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h195, _, _) => {
            h195
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h196, _) => {
            h196
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, _, _, _, _, h197) => {
            h197
        }
    })
}

pub fn f19(v0: Adt1) -> Adt1 {
    Adt1::C0(match v0.clone() {
        Adt1::C0(h198, _, _, _, _, _, _, _, _) => {
            h198
        }
    }, match v0.clone() {
        Adt1::C0(_, h199, _, _, _, _, _, _, _) => {
            h199
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h200, _, _, _, _, _, _) => {
            h200
        }
    }, 1u64, match v0.clone() {
        Adt1::C0(_, _, _, _, h201, _, _, _, _) => {
            h201
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, h202, _, _, _) => {
            h202
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h203, _, _) => {
            h203
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h204, _) => {
            h204
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, _, _, _, _, h205) => {
            h205
        }
    })
}

pub fn f20(v0: Adt1) -> Adt1 {
    Adt1::C0(match v0.clone() {
        Adt1::C0(h206, _, _, _, _, _, _, _, _) => {
            h206
        }
    }, match v0.clone() {
        Adt1::C0(_, h207, _, _, _, _, _, _, _) => {
            h207
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h208, _, _, _, _, _, _) => {
            h208
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h209, _, _, _, _, _) => {
            h209
        }
    }, 1u64, match v0.clone() {
        Adt1::C0(_, _, _, _, _, h210, _, _, _) => {
            h210
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h211, _, _) => {
            h211
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h212, _) => {
            h212
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, _, _, _, _, h213) => {
            h213
        }
    })
}

pub fn f21(v0: Adt1) -> Adt1 {
    Adt1::C0(match v0.clone() {
        Adt1::C0(h214, _, _, _, _, _, _, _, _) => {
            h214
        }
    }, match v0.clone() {
        Adt1::C0(_, h215, _, _, _, _, _, _, _) => {
            h215
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h216, _, _, _, _, _, _) => {
            h216
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h217, _, _, _, _, _) => {
            h217
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h218, _, _, _, _) => {
            h218
        }
    }, 1u64, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h219, _, _) => {
            h219
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h220, _) => {
            h220
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, _, _, _, _, h221) => {
            h221
        }
    })
}

pub fn f22(v0: Adt1) -> Adt1 {
    Adt1::C0(match v0.clone() {
        Adt1::C0(h222, _, _, _, _, _, _, _, _) => {
            h222
        }
    }, match v0.clone() {
        Adt1::C0(_, h223, _, _, _, _, _, _, _) => {
            h223
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h224, _, _, _, _, _, _) => {
            h224
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h225, _, _, _, _, _) => {
            h225
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h226, _, _, _, _) => {
            h226
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, h227, _, _, _) => {
            h227
        }
    }, 1u64, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h228, _) => {
            h228
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, _, _, _, _, h229) => {
            h229
        }
    })
}

pub fn f23(v0: Adt1) -> Adt1 {
    Adt1::C0(match v0.clone() {
        Adt1::C0(h230, _, _, _, _, _, _, _, _) => {
            h230
        }
    }, match v0.clone() {
        Adt1::C0(_, h231, _, _, _, _, _, _, _) => {
            h231
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h232, _, _, _, _, _, _) => {
            h232
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h233, _, _, _, _, _) => {
            h233
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h234, _, _, _, _) => {
            h234
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, h235, _, _, _) => {
            h235
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h236, _, _) => {
            h236
        }
    }, 1u64, match v0 {
        Adt1::C0(_, _, _, _, _, _, _, _, h237) => {
            h237
        }
    })
}

pub fn f24(v0: Adt1) -> R<Adt1> {
    Ok(Adt1::C0(match v0.clone() {
        Adt1::C0(h238, _, _, _, _, _, _, _, _) => {
            h238
        }
    }, match v0.clone() {
        Adt1::C0(_, h239, _, _, _, _, _, _, _) => {
            h239
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h240, _, _, _, _, _, _) => {
            h240
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h241, _, _, _, _, _) => {
            h241
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h242, _, _, _, _) => {
            h242
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, h243, _, _, _) => {
            h243
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h244, _, _) => {
            h244
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h245, _) => {
            h245
        }
    }, {
        let a247 = match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h246) => {
                h246
            }
        };
        let a248 = 1u64;
        nat_add(a247, a248)?
    }))
}

pub fn f25(v0: Adt1) -> Option<Adt1> {
    if f9(v0.clone()) {
        Some(f17(v0))
    } else {
        None::<Adt1>
    }
}

pub fn f26(v0: Adt1) -> Option<Adt1> {
    if f10(v0.clone()) {
        Some(f18(v0))
    } else {
        None::<Adt1>
    }
}

pub fn f27(v0: Adt1) -> Option<Adt1> {
    if f11(v0.clone()) {
        Some(f19(v0))
    } else {
        None::<Adt1>
    }
}

pub fn f28(v0: Adt1) -> Option<Adt1> {
    if f12(v0.clone()) {
        Some(f20(v0))
    } else {
        None::<Adt1>
    }
}

pub fn f29(v0: Adt1) -> R<Option<Adt1>> {
    if f13(v0.clone())? {
        Ok(Some(f21(v0)))
    } else {
        Ok(None::<Adt1>)
    }
}

pub fn f30(v0: Adt1) -> Option<Adt1> {
    if f14(v0.clone()) {
        Some(f22(v0))
    } else {
        None::<Adt1>
    }
}

pub fn f31(v0: Adt1) -> Option<Adt1> {
    if f15(v0.clone()) {
        Some(f23(v0))
    } else {
        None::<Adt1>
    }
}

pub fn f32(v0: Adt1) -> R<Option<Adt1>> {
    if f16(v0.clone()) {
        Ok(Some(f24(v0)?))
    } else {
        Ok(None::<Adt1>)
    }
}

pub fn f33(v0: Adt0) -> Adt1 {
    Adt1::C0(v0, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64)
}

pub fn f34(v0: Adt1) -> R<Option<Adt2>> {
    if f9(v0.clone()) {
        Ok(Some(Adt2::C0))
    } else {
        if f10(v0.clone()) {
            Ok(Some(Adt2::C1))
        } else {
            if f11(v0.clone()) {
                Ok(Some(Adt2::C2))
            } else {
                if f12(v0.clone()) {
                    Ok(Some(Adt2::C3))
                } else {
                    if f13(v0.clone())? {
                        Ok(Some(Adt2::C4))
                    } else {
                        if f14(v0.clone()) {
                            Ok(Some(Adt2::C5))
                        } else {
                            if f15(v0.clone()) {
                                Ok(Some(Adt2::C6))
                            } else {
                                if f16(v0) {
                                    Ok(Some(Adt2::C7))
                                } else {
                                    Ok(None::<Adt2>)
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f35(v0: Adt1, v1: Adt2) -> R<Option<Adt1>> {
    let m249 = v1;
    match m249 {
        Adt2::C0 => {
            Ok(f25(v0))
        }
        Adt2::C1 => {
            Ok(f26(v0))
        }
        Adt2::C2 => {
            Ok(f27(v0))
        }
        Adt2::C3 => {
            Ok(f28(v0))
        }
        Adt2::C4 => {
            f29(v0)
        }
        Adt2::C5 => {
            Ok(f30(v0))
        }
        Adt2::C6 => {
            Ok(f31(v0))
        }
        Adt2::C7 => {
            f32(v0)
        }
    }
}

pub fn f36(v0: Adt1) -> R<Option<Adt1>> {
    let m250 = f34(v0.clone())?;
    match m250 {
        None => {
            Ok(None::<Adt1>)
        }
        Some(v1) => {
            f35(v0, v1)
        }
    }
}

pub fn f37(v0: Adt1) -> Option<u64> {
    Some(match v0 {
        Adt1::C0(_, _, _, _, _, _, _, _, h251) => {
            h251
        }
    })
}

pub fn f38(v0: Adt0, v1: Adt1) -> Option<u64> {
    let m252 = f37(v1);
    match m252 {
        None => {
            None::<u64>
        }
        Some(v2) => {
            let v3: bool = f40(v0, v2);
            if v3 {
                Some(v2)
            } else {
                None::<u64>
            }
        }
    }
}

pub fn f39(v0: Adt0, v1: Adt1) -> Result<u64, (bool, bool)> {
    let m253 = f38(v0, v1.clone());
    match m253 {
        None => {
            let m254 = f37(v1);
            match m254 {
                None => {
                    Err::<u64, (bool, bool)>((false, true))
                }
                Some(_) => {
                    Err::<u64, (bool, bool)>((true, false))
                }
            }
        }
        Some(v2) => {
            Ok::<u64, (bool, bool)>(v2)
        }
    }
}

pub fn f40(v0: Adt0, v1: u64) -> bool {
    let a271 = {
        let a255 = v1;
        let a256 = 3u64;
        nat_le(a255, a256)
    };
    let a272 = {
        let a269 = {
            let a257 = v1;
            let a258 = 3u64;
            nat_eq(a257, a258)
        };
        let a270 = {
            let a268 = {
                let a266 = f8(v0.clone());
                let a267 = {
                    let a264 = {
                        let a260 = match v0.clone() {
                            Adt0::C0(_, _, _, _, _, h259) => {
                                h259
                            }
                        };
                        let a261 = 1u64;
                        nat_eq(a260, a261)
                    };
                    let a265 = {
                        let a262 = f4(v0.clone());
                        let a263 = f5(v0);
                        bool_and(a262, a263)
                    };
                    bool_and(a264, a265)
                };
                bool_and(a266, a267)
            };
            bool_not(a268)
        };
        bool_or(a269, a270)
    };
    bool_and(a271, a272)
}

pub fn f41(v0: Fn0, p277: u64, p278: Adt4) -> R<(Adt4, bool)> {
    let m276 = (p277, p278);
    {
        let mut m276 = m276;
        loop {
            let step = {
                let (v1, v2) = m276;
                let m273 = v1;
                if m273 == 0 {
                    Err::<(u64, Adt4), R<(Adt4, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m273 - 1;
                    let m275 = {
                        let c274: Fn0 = v0.clone();
                        c274.apply(v2.clone())?
                    };
                    match m275 {
                        None => {
                            Err::<(u64, Adt4), R<(Adt4, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt4), R<(Adt4, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m276 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn run(p0: Adt0) -> R<(Result<(u64, List<Adt2>), (bool, bool)>, (u64, u64))> {
    f0(p0)
}
