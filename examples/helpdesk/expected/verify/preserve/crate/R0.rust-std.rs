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
pub enum Fn0 {
    F13(Bytes),
    F14(u64, Bytes),
    F38(Bytes, u64, Bytes, u64),
}

impl Fn0 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn0::F13(k0) => Ok(f13(k0.clone(), p0)),
            Fn0::F14(k0, k1) => f14(*k0, k1.clone(), p0),
            Fn0::F38(k0, k1, k2, k3) => f38(k0.clone(), *k1, k2.clone(), *k3, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F18,
}

impl Fn1 {
    pub fn apply(&self, p0: (u64, (u64, i64)), p1: i64) -> R<(u64, (u64, i64))> {
        match self {
            Fn1::F18 => f18(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F23,
}

impl Fn2 {
    pub fn apply(&self, p0: (u64, (i64, i64)), p1: i64) -> R<(u64, (i64, i64))> {
        match self {
            Fn2::F23 => f23(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F28(List<i64>),
}

impl Fn3 {
    pub fn apply(&self, p0: (List<i64>, List<i64>), p1: List<i64>) -> R<(List<i64>, List<i64>)> {
        match self {
            Fn3::F28(k0) => f28(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F34,
}

impl Fn4 {
    pub fn apply(&self, p0: (i64, List<i64>), p1: i64) -> R<(i64, List<i64>)> {
        match self {
            Fn4::F34 => f34(p0, p1),
        }
    }
}

pub fn f0(v0: Bytes) -> R<u64> {
    let m1 = {
        let v1: Bytes = f1(v0)?;
        if f2(v1.clone()) {
            let v2: List<i64> = f3(v1)?;
            Ok::<List<i64>, (bool, bool)>(v2)
        } else {
            Err::<List<i64>, (bool, bool)>((false, false))
        }
    };
    match m1 {
        Ok(v3) => {
            let m2 = f4(f5(), v3)?;
            match m2 {
                None => {
                    Ok(8u64)
                }
                Some(v4) => {
                    Ok(v4)
                }
            }
        }
        Err(_) => {
            Ok(9u64)
        }
    }
}

pub fn f1(v0: Bytes) -> R<Bytes> {
    let v1: u64 = f6(v0.clone())?;
    let v2: u64 = f7(v0.clone(), v1)?;
    Ok(f8(v0, v2, {
        let a3 = v1;
        let a4 = v2;
        nat_sub(a3, a4)
    }))
}

pub fn f2(v0: Bytes) -> bool {
    let a6 = 0u64;
    let a7 = {
        let a5 = v0;
        length_bytes(a5)
    };
    nat_lt(a6, a7)
}

pub fn f3(v0: Bytes) -> R<List<i64>> {
    f9(v0)
}

pub fn f4(v0: i64, v1: List<i64>) -> R<Option<u64>> {
    let a9 = {
        let a8 = v1.clone();
        length_list(a8)
    };
    let a10 = 2u64;
    if nat_lt(a9, a10) {
        Ok(None::<u64>)
    } else {
        let m13 = {
            let a11 = f10(v1.clone())?;
            let a12 = v0;
            compare(a11, a12)
        };
        if match m13 {
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
            Ok(None::<u64>)
        } else {
            Ok(Some(f11(v1)?))
        }
    }
}

pub fn f5() -> i64 {
    2i64
}

pub fn f6(v0: Bytes) -> R<u64> {
    let (h14, _) = f12(Fn0::F13(v0.clone()), {
        let a15 = v0.clone();
        length_bytes(a15)
    }, {
        let a16 = v0;
        length_bytes(a16)
    })?;
    Ok(h14)
}

pub fn f7(v0: Bytes, v1: u64) -> R<u64> {
    let (h17, _) = f12(Fn0::F14(v1, v0), v1, 0u64)?;
    Ok(h17)
}

pub fn f8(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m21 = {
        let a18 = v0;
        let a19 = v1;
        let a20 = v2;
        slice_bytes(a18, a19, a20)
    };
    match m21 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f9(v0: Bytes) -> R<List<i64>> {
    let v1: List<i64> = f15(v0)?;
    Ok(v1)
}

pub fn f10(v0: List<i64>) -> R<i64> {
    let a23 = {
        let a22 = v0.clone();
        length_list(a22)
    };
    let a24 = 2u64;
    if nat_lt(a23, a24) {
        Ok(0i64)
    } else {
        let a27 = {
            let (h25, _) = f16(v0.clone())?;
            h25
        };
        let a28 = {
            let (_, h26) = f16(v0)?;
            h26
        };
        int_sub(a27, a28)
    }
}

pub fn f11(v0: List<i64>) -> R<u64> {
    let (h29, _) = {
        let (_, h30) = f17(Fn1::F18, (0u64, (0u64, 0i64)), v0)?;
        h30
    };
    Ok(h29)
}

pub fn f12(v0: Fn0, p35: u64, p36: u64) -> R<(u64, bool)> {
    let m34 = (p35, p36);
    {
        let mut m34 = m34;
        loop {
            let step = {
                let (v1, v2) = m34;
                let m31 = v1;
                if m31 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m31 - 1;
                    let m33 = {
                        let c32: Fn0 = v0.clone();
                        c32.apply(v2)?
                    };
                    match m33 {
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
                Ok(next) => { m34 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f13(v0: Bytes, v1: u64) -> Option<u64> {
    let a37 = 0u64;
    let a38 = v1;
    if nat_lt(a37, a38) {
        if f19(v0, {
            let a39 = v1;
            let a40 = 1u64;
            nat_sub(a39, a40)
        }) {
            Some({
                let a41 = v1;
                let a42 = 1u64;
                nat_sub(a41, a42)
            })
        } else {
            None::<u64>
        }
    } else {
        None::<u64>
    }
}

pub fn f14(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    let a43 = v2;
    let a44 = v0;
    if nat_lt(a43, a44) {
        if f19(v1, v2) {
            Ok(Some({
                let a45 = v2;
                let a46 = 1u64;
                nat_add(a45, a46)?
            }))
        } else {
            Ok(None::<u64>)
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f15(v0: Bytes) -> R<List<i64>> {
    f20(f21(v0)?)
}

pub fn f16(v0: List<i64>) -> R<(i64, i64)> {
    let (_, h47) = f22(Fn2::F23, (0u64, (0i64, 0i64)), v0)?;
    Ok(h47)
}

pub fn f17(v0: Fn1, p51: (u64, (u64, i64)), p52: List<i64>) -> R<(u64, (u64, i64))> {
    let m50 = (p51, p52);
    {
        let mut m50 = m50;
        loop {
            let step = {
                let (v1, v2) = m50;
                let m48 = v2;
                match m48.uncons() {
                    None => {
                        Err::<((u64, (u64, i64)), List<i64>), R<(u64, (u64, i64))>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<((u64, (u64, i64)), List<i64>), R<(u64, (u64, i64))>>(({
                            let c49: Fn1 = v0.clone();
                            c49.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m50 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f18(v0: (u64, (u64, i64)), v1: i64) -> R<(u64, (u64, i64))> {
    f24(v0, v1)
}

pub fn f19(v0: Bytes, v1: u64) -> bool {
    let m53 = f25(v0, v1);
    match m53 {
        None => {
            false
        }
        Some(v2) => {
            f26(v2)
        }
    }
}

pub fn f20(v0: List<i64>) -> R<List<i64>> {
    let (h54, _) = f27(Fn3::F28(v0), (List::<i64>::nil(), f29()), f30())?;
    Ok(h54)
}

pub fn f21(v0: Bytes) -> R<List<i64>> {
    f31(v0)
}

pub fn f22(v0: Fn2, p58: (u64, (i64, i64)), p59: List<i64>) -> R<(u64, (i64, i64))> {
    let m57 = (p58, p59);
    {
        let mut m57 = m57;
        loop {
            let step = {
                let (v1, v2) = m57;
                let m55 = v2;
                match m55.uncons() {
                    None => {
                        Err::<((u64, (i64, i64)), List<i64>), R<(u64, (i64, i64))>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<((u64, (i64, i64)), List<i64>), R<(u64, (i64, i64))>>(({
                            let c56: Fn2 = v0.clone();
                            c56.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m57 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f23(v0: (u64, (i64, i64)), v1: i64) -> R<(u64, (i64, i64))> {
    f32(v0, v1)
}

pub fn f24(v0: (u64, (u64, i64)), v1: i64) -> R<(u64, (u64, i64))> {
    let a61 = {
        let (h60, _) = v0;
        h60
    };
    let a62 = 0u64;
    if nat_eq(a61, a62) {
        Ok((1u64, (0u64, v1)))
    } else {
        let m67 = {
            let a65 = {
                let (_, h63) = {
                    let (_, h64) = v0;
                    h64
                };
                h63
            };
            let a66 = v1;
            compare(a65, a66)
        };
        if match m67 {
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
            Ok(({
                let a69 = {
                    let (h68, _) = v0;
                    h68
                };
                let a70 = 1u64;
                nat_add(a69, a70)?
            }, ({
                let (h71, _) = v0;
                h71
            }, v1)))
        } else {
            Ok(({
                let a73 = {
                    let (h72, _) = v0;
                    h72
                };
                let a74 = 1u64;
                nat_add(a73, a74)?
            }, {
                let (_, h75) = v0;
                h75
            }))
        }
    }
}

pub fn f25(v0: Bytes, v1: u64) -> Option<u8> {
    let a76 = v0;
    let a77 = v1;
    index_bytes(a76, a77)
}

pub fn f26(v0: u8) -> bool {
    let a78 = v0;
    let a79 = 32u8;
    if if if equal(a78, a79) {
        true
    } else {
        let a80 = v0;
        let a81 = 9u8;
        equal(a80, a81)
    } {
        true
    } else {
        let a82 = v0;
        let a83 = 10u8;
        equal(a82, a83)
    } {
        true
    } else {
        let a84 = v0;
        let a85 = 12u8;
        if equal(a84, a85) {
            true
        } else {
            let a86 = v0;
            let a87 = 13u8;
            equal(a86, a87)
        }
    }
}

pub fn f27(v0: Fn3, p91: (List<i64>, List<i64>), p92: List<List<i64>>) -> R<(List<i64>, List<i64>)> {
    let m90 = (p91, p92);
    {
        let mut m90 = m90;
        loop {
            let step = {
                let (v1, v2) = m90;
                let m88 = v2;
                match m88.uncons() {
                    None => {
                        Err::<((List<i64>, List<i64>), List<List<i64>>), R<(List<i64>, List<i64>)>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<((List<i64>, List<i64>), List<List<i64>>), R<(List<i64>, List<i64>)>>(({
                            let c89: Fn3 = v0.clone();
                            c89.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m90 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f28(v0: List<i64>, v1: (List<i64>, List<i64>), v2: List<i64>) -> R<(List<i64>, List<i64>)> {
    let m94 = {
        let (_, h93) = v1.clone();
        h93
    };
    match m94.uncons() {
        None => {
            Ok(v1)
        }
        Some((v3, v4)) => {
            Ok(({
                let a100 = {
                    let (h95, _) = v1;
                    h95
                };
                let a101 = List::cons({
                    let a98 = v3;
                    let a99 = {
                        let a97 = {
                            let (h96, _) = f33(Fn4::F34, (0i64, v0), v2)?;
                            h96
                        };
                        int_neg(a97)?
                    };
                    int_sub(a98, a99)?
                }, List::<i64>::nil());
                append_list(a100, a101)
            }, v4))
        }
    }
}

pub fn f29() -> List<i64> {
    List::cons(0i64, List::cons(0i64, List::cons(0i64, List::<i64>::nil())))
}

pub fn f30() -> List<List<i64>> {
    List::cons(List::cons(4i64, List::cons(0i64, List::cons(0i64, List::<i64>::nil()))), List::cons(List::cons(0i64, List::cons(3i64, List::cons(0i64, List::<i64>::nil()))), List::cons(List::cons(0i64, List::cons(0i64, List::cons(2i64, List::<i64>::nil()))), List::<List<i64>>::nil())))
}

pub fn f31(v0: Bytes) -> R<List<i64>> {
    Ok(List::cons(f35(if f36(v0.clone(), Bytes::lit(&[0x64, 0x6f, 0x77, 0x6e]))? {
        true
    } else {
        f36(v0.clone(), Bytes::lit(&[0x6f, 0x75, 0x74, 0x61, 0x67, 0x65]))?
    }), List::cons(f35(if f36(v0.clone(), Bytes::lit(&[0x69, 0x6e, 0x76, 0x6f, 0x69, 0x63, 0x65]))? {
        true
    } else {
        f36(v0.clone(), Bytes::lit(&[0x72, 0x65, 0x66, 0x75, 0x6e, 0x64]))?
    }), List::cons(f35(if f36(v0.clone(), Bytes::lit(&[0x70, 0x61, 0x73, 0x73, 0x77, 0x6f, 0x72, 0x64]))? {
        true
    } else {
        f36(v0, Bytes::lit(&[0x6c, 0x6f, 0x67, 0x69, 0x6e]))?
    }), List::<i64>::nil()))))
}

pub fn f32(v0: (u64, (i64, i64)), v1: i64) -> R<(u64, (i64, i64))> {
    let a103 = {
        let (h102, _) = v0;
        h102
    };
    let a104 = 0u64;
    if nat_eq(a103, a104) {
        Ok((1u64, (v1, 0i64)))
    } else {
        let a106 = {
            let (h105, _) = v0;
            h105
        };
        let a107 = 1u64;
        if nat_eq(a106, a107) {
            let m112 = {
                let a110 = {
                    let (h108, _) = {
                        let (_, h109) = v0;
                        h109
                    };
                    h108
                };
                let a111 = v1;
                compare(a110, a111)
            };
            if match m112 {
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
                Ok((2u64, (v1, {
                    let (h113, _) = {
                        let (_, h114) = v0;
                        h114
                    };
                    h113
                })))
            } else {
                Ok((2u64, ({
                    let (h115, _) = {
                        let (_, h116) = v0;
                        h116
                    };
                    h115
                }, v1)))
            }
        } else {
            let m121 = {
                let a119 = {
                    let (h117, _) = {
                        let (_, h118) = v0;
                        h118
                    };
                    h117
                };
                let a120 = v1;
                compare(a119, a120)
            };
            if match m121 {
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
                Ok(({
                    let a123 = {
                        let (h122, _) = v0;
                        h122
                    };
                    let a124 = 1u64;
                    nat_add(a123, a124)?
                }, (v1, {
                    let (h125, _) = {
                        let (_, h126) = v0;
                        h126
                    };
                    h125
                })))
            } else {
                Ok(({
                    let a128 = {
                        let (h127, _) = v0;
                        h127
                    };
                    let a129 = 1u64;
                    nat_add(a128, a129)?
                }, ({
                    let (h130, _) = {
                        let (_, h131) = v0;
                        h131
                    };
                    h130
                }, {
                    let m136 = {
                        let a134 = {
                            let (_, h132) = {
                                let (_, h133) = v0;
                                h133
                            };
                            h132
                        };
                        let a135 = v1;
                        compare(a134, a135)
                    };
                    if match m136 {
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
                        v1
                    } else {
                        let (_, h137) = {
                            let (_, h138) = v0;
                            h138
                        };
                        h137
                    }
                })))
            }
        }
    }
}

pub fn f33(v0: Fn4, p142: (i64, List<i64>), p143: List<i64>) -> R<(i64, List<i64>)> {
    let m141 = (p142, p143);
    {
        let mut m141 = m141;
        loop {
            let step = {
                let (v1, v2) = m141;
                let m139 = v2;
                match m139.uncons() {
                    None => {
                        Err::<((i64, List<i64>), List<i64>), R<(i64, List<i64>)>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<((i64, List<i64>), List<i64>), R<(i64, List<i64>)>>(({
                            let c140: Fn4 = v0.clone();
                            c140.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m141 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f34(v0: (i64, List<i64>), v1: i64) -> R<(i64, List<i64>)> {
    let m145 = {
        let (_, h144) = v0.clone();
        h144
    };
    match m145.uncons() {
        None => {
            Ok(v0)
        }
        Some((v2, v3)) => {
            Ok(({
                let a150 = {
                    let (h146, _) = v0;
                    h146
                };
                let a151 = {
                    let a149 = {
                        let a147 = v1;
                        let a148 = v2;
                        int_mul(a147, a148)?
                    };
                    int_neg(a149)?
                };
                int_sub(a150, a151)?
            }, v3))
        }
    }
}

pub fn f35(v0: bool) -> i64 {
    if v0 {
        1i64
    } else {
        0i64
    }
}

pub fn f36(v0: Bytes, v1: Bytes) -> R<bool> {
    let m152 = f37(v0, v1)?;
    match m152 {
        None => {
            Ok(false)
        }
        Some(_) => {
            Ok(true)
        }
    }
}

pub fn f37(v0: Bytes, v1: Bytes) -> R<Option<u64>> {
    let v2: u64 = {
        let a153 = v0.clone();
        length_bytes(a153)
    };
    let v3: u64 = {
        let a154 = v1.clone();
        length_bytes(a154)
    };
    let v4: u64 = {
        let (h155, _) = f12(Fn0::F38(v1.clone(), v3, v0.clone(), v2), {
            let a156 = v2;
            let a157 = 1u64;
            nat_add(a156, a157)?
        }, 0u64)?;
        h155
    };
    if f39(v0, v1, v4) {
        Ok(Some(v4))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f38(v0: Bytes, v1: u64, v2: Bytes, v3: u64, v4: u64) -> R<Option<u64>> {
    if f39(v2, v0, v4) {
        Ok(None::<u64>)
    } else {
        let a160 = {
            let a158 = v4;
            let a159 = v1;
            nat_add(a158, a159)?
        };
        let a161 = v3;
        if nat_lt(a160, a161) {
            Ok(Some({
                let a162 = v4;
                let a163 = 1u64;
                nat_add(a162, a163)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f39(v0: Bytes, v1: Bytes, v2: u64) -> bool {
    let m168 = {
        let a165 = v0;
        let a166 = v2;
        let a167 = {
            let a164 = v1.clone();
            length_bytes(a164)
        };
        slice_bytes(a165, a166, a167)
    };
    match m168 {
        None => {
            false
        }
        Some(v3) => {
            let a169 = v3;
            let a170 = v1;
            equal(a169, a170)
        }
    }
}
