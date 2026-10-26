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
    C0(Adt1, Adt4),
    C1(Adt5, Adt6, Adt4),
    C2(Adt7, std::rc::Rc<Adt0>, Adt4),
    C3(Adt8, std::rc::Rc<Adt0>, bool, Adt4),
    C4(Adt6, std::rc::Rc<Adt0>, bool, Adt4),
    C5(Adt10, std::rc::Rc<Adt0>, Adt4),
    C6(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>, Adt4),
    C7(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>, std::rc::Rc<Adt0>, Adt4),
    C8(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>, Adt4),
    C9(std::rc::Rc<Adt0>),
    C10(Adt16, Adt6, Adt4),
    C11(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
    C12,
    C13(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
    C14(Adt17, bool, Adt11, std::rc::Rc<Adt0>, Adt4, std::rc::Rc<Adt0>),
    C15(Adt17, std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
    C16(Adt6, std::rc::Rc<Adt0>),
}

#[derive(Clone)]
pub enum Adt1 {
    C0,
    C1(bool),
    C2(u64),
    C3(bool, u64),
    C4(Adt2, bool, u64),
    C5(Bytes),
    C6(Bytes),
    C7(Adt3),
    C8,
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
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt4 {
    C0(Bytes, Option<Adt2>),
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
pub enum Adt7 {
    C0,
    C1,
    C2,
    C3,
}

#[derive(Clone)]
pub enum Adt8 {
    C0(u64),
    C1(Adt9),
    C2(u64, Option<Adt2>),
}

#[derive(Clone)]
pub enum Adt9 {
    C0(u64, Option<Adt2>),
}

#[derive(Clone)]
pub enum Adt10 {
    C0(Adt11),
    C1,
    C2(Adt11, Adt11),
    C3(Adt11, Adt11),
    C4(u64, u64),
    C5(u64, u64),
    C6,
    C7(Adt11),
}

#[derive(Clone)]
pub enum Adt11 {
    C0(Adt12),
    C1(Adt2),
    C2(Adt13, std::rc::Rc<Adt11>),
    C3(Adt14, std::rc::Rc<Adt11>, std::rc::Rc<Adt11>),
    C4(Adt15, u64),
    C5,
    C6(std::rc::Rc<Adt11>, std::rc::Rc<Adt11>),
}

#[derive(Clone)]
pub enum Adt12 {
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
pub enum Adt13 {
    C0,
    C1,
    C2,
    C3,
    C4,
}

#[derive(Clone)]
pub enum Adt14 {
    C0,
    C1,
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
    C3,
}

#[derive(Clone)]
pub enum Adt17 {
    C0(Adt18),
    C1(Adt6),
    C2(std::rc::Rc<Adt17>),
    C3(Adt19, std::rc::Rc<Adt17>),
    C4(u64, u64, std::rc::Rc<Adt17>),
    C5,
    C6(std::rc::Rc<Adt17>, std::rc::Rc<Adt17>),
    C7(Adt3),
}

#[derive(Clone)]
pub enum Adt18 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt19 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt20 {
    C0(List<Bytes>, Bytes, u64, u64, bool),
}

#[derive(Clone)]
pub enum Fn0 {
    F28,
}

impl Fn0 {
    pub fn apply(&self, p0: List<Bytes>) -> Option<List<Bytes>> {
        match self {
            Fn0::F28 => f28(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F30,
}

impl Fn1 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn1::F30 => f30(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F45(u64),
    F54(Bytes),
    F55(Bytes),
}

impl Fn2 {
    pub fn apply(&self, p0: (u64, Adt20)) -> R<Option<(u64, Adt20)>> {
        match self {
            Fn2::F45(k0) => f45(*k0, p0),
            Fn2::F54(k0) => f54(k0.clone(), p0),
            Fn2::F55(k0) => f55(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F61,
}

impl Fn3 {
    pub fn apply(&self, p0: (Option<Bytes>, List<Bytes>), p1: Bytes) -> (Option<Bytes>, List<Bytes>) {
        match self {
            Fn3::F61 => f61(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F66(Bytes),
    F76(Bytes),
}

impl Fn4 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn4::F66(k0) => f66(k0.clone(), p0),
            Fn4::F76(k0) => f76(k0.clone(), p0),
        }
    }
}

pub fn f0(v0: u64, v1: Adt0) -> R<Result<Bytes, Bytes>> {
    Ok(f1(f2(f3(v0), v1, 0u64, 100u64, 0u64)?))
}

pub fn f1(v0: Adt20) -> Result<Bytes, Bytes> {
    if match v0.clone() {
        Adt20::C0(_, _, _, _, h1) => {
            h1
        }
    } {
        Err::<Bytes, Bytes>({
            let a2 = Str::lit("output exceeds the capacity of the buffer");
            utf8_encode(a2)
        })
    } else {
        Ok::<Bytes, Bytes>(f4(f5(List::cons(match v0.clone() {
            Adt20::C0(_, h3, _, _, _) => {
                h3
            }
        }, match v0 {
            Adt20::C0(h4, _, _, _, _) => {
                h4
            }
        }))))
    }
}

pub fn f2(v0: Adt20, v1: Adt0, v2: u64, v3: u64, v4: u64) -> R<Adt20> {
    let m5 = v1;
    match m5 {
        Adt0::C0(v5, _) => {
            f6(v0, v5)
        }
        Adt0::C1(v7, v8, _) => {
            let m6 = v7;
            match m6 {
                Adt5::C0 => {
                    f7(v0, v8)
                }
                Adt5::C1 => {
                    f8(f7(v0, v8)?, Str::lit(".clone()"))
                }
                Adt5::C2 => {
                    f7(v0, v8)
                }
                Adt5::C3 => {
                    f7(f8(v0, Str::lit("*"))?, v8)
                }
                Adt5::C4 => {
                    f8(f7(f8(v0, Str::lit("(*"))?, v8)?, Str::lit(").clone()"))
                }
            }
        }
        Adt0::C2(v10, r63, _) => {
            let v11 = (*r63).clone();
            let m7 = v10;
            match m7 {
                Adt7::C0 => {
                    let a8 = f9(v11.clone());
                    let a9 = 0u64;
                    if if nat_eq(a8, a9) {
                        true
                    } else {
                        let a10 = f9(v11.clone());
                        let a11 = 1u64;
                        if nat_eq(a10, a11) {
                            true
                        } else {
                            let a12 = f9(v11.clone());
                            let a13 = 3u64;
                            nat_eq(a12, a13)
                        }
                    } {
                        f8(f2(f8(v0, Str::lit("!("))?, v11, v2, 100u64, 0u64)?, Str::lit(")"))
                    } else {
                        f2(f8(v0, Str::lit("!"))?, v11, v2, 100u64, 0u64)
                    }
                }
                Adt7::C1 => {
                    f8(f2(f8(v0, Str::lit("std::rc::Rc::new("))?, v11, v2, 100u64, 0u64)?, Str::lit(")"))
                }
                Adt7::C2 => {
                    f8(f2(f8(v0, Str::lit("i128::from("))?, v11, v2, 100u64, 0u64)?, Str::lit(")"))
                }
                Adt7::C3 => {
                    f8(f2(f8(v0, Str::lit("Ok("))?, v11, v2, 100u64, 0u64)?, Str::lit(")"))
                }
            }
        }
        Adt0::C3(v13, r64, v15, _) => {
            let v14 = (*r64).clone();
            let v17: u64 = f10(v13.clone());
            let v22: Adt20 = {
                let m14 = v13.clone();
                match m14 {
                    Adt8::C0(v18) => {
                        f8(f11(f8(v0, Str::lit("f"))?, v18)?, Str::lit("("))?
                    }
                    Adt8::C1(v19) => {
                        f8(f12(v0, v19)?, Str::lit("("))?
                    }
                    Adt8::C2(v20, v21) => {
                        f13(v0, v20, v21)?
                    }
                }
            };
            let v23: Adt20 = f2(v22, v14, v2, v17, 0u64)?;
            let v28: Adt20 = {
                let m15 = v13;
                match m15 {
                    Adt8::C0(_) => {
                        f8(v23, Str::lit(")"))?
                    }
                    Adt8::C1(_) => {
                        f8(v23, Str::lit(")"))?
                    }
                    Adt8::C2(v26, _) => {
                        f14(v23, v26)?
                    }
                }
            };
            if v15 {
                f8(v28, Str::lit("?"))
            } else {
                Ok(v28)
            }
        }
        Adt0::C4(v29, r65, v31, _) => {
            let v30 = (*r65).clone();
            let v33: Adt20 = f8(f2(f8(f7(v0, v29)?, Str::lit(".apply("))?, v30, v2, 100u64, 0u64)?, Str::lit(")"))?;
            if v31 {
                f8(v33, Str::lit("?"))
            } else {
                Ok(v33)
            }
        }
        Adt0::C5(v34, r66, _) => {
            let v35 = (*r66).clone();
            let v37: Adt20 = f15(v0, v34.clone())?;
            let a16 = f16(v34.clone());
            let a17 = 1u64;
            if nat_eq(a16, a17) {
                f8(f2(v37, v35, v2, 100u64, 0u64)?, Str::lit(")"))
            } else {
                let a18 = f16(v34);
                let a19 = 2u64;
                if nat_eq(a18, a19) {
                    if f17(v35.clone()) {
                        f8(f2(f8(v37, Str::lit("("))?, v35, v2, 100u64, 0u64)?, Str::lit(")"))
                    } else {
                        Ok(v37)
                    }
                } else {
                    Ok(v37)
                }
            }
        }
        Adt0::C6(r67, r68, _) => {
            let v38 = (*r67).clone();
            let v39 = (*r68).clone();
            f8(f2(f8(f2(f8(v0, Str::lit("("))?, v38, v2, 100u64, 0u64)?, Str::lit(", "))?, v39, v2, 100u64, 0u64)?, Str::lit(")"))
        }
        Adt0::C7(r69, r70, r71, _) => {
            let v41 = (*r69).clone();
            let v42 = (*r70).clone();
            let v43 = (*r71).clone();
            let v45: Adt20 = f2(f8(f2(f8(v0, Str::lit("if "))?, v41, v2, 100u64, 0u64)?, Str::lit(" "))?, v42, v2, 100u64, 0u64)?;
            if f18(v43.clone()) {
                Ok(v45)
            } else {
                f2(f8(v45, Str::lit(" else "))?, v43, v2, 100u64, 0u64)
            }
        }
        Adt0::C8(r72, r73, _) => {
            let v46 = (*r72).clone();
            let v47 = (*r73).clone();
            let a20 = f19(v47.clone());
            let a21 = 15u64;
            if nat_eq(a20, a21) {
                if f20(v46.clone()) {
                    let v49: Adt20 = f2(f8(f2(f8(v0, Str::lit("{ let value = "))?, v46, v2, 100u64, 0u64)?, Str::lit("; match value {\u{a}"))?, v47, v2, 100u64, 0u64)?;
                    f8(f21(v49, v2)?, Str::lit("} }"))
                } else {
                    let v50: Adt20 = f2(f8(f2(f8(v0, Str::lit("match "))?, v46, v2, 100u64, 0u64)?, Str::lit(" {\u{a}"))?, v47, v2, 100u64, 0u64)?;
                    f8(f21(v50, v2)?, Str::lit("}"))
                }
            } else {
                f8(f2(f8(v0, Str::lit("match "))?, v46, v2, 100u64, 0u64)?, Str::lit(" {}"))
            }
        }
        Adt0::C9(r74) => {
            let v51 = (*r74).clone();
            f2(v0, v51, v2, v3, v4)
        }
        Adt0::C10(v52, v53, _) => {
            let m22 = v52;
            match m22 {
                Adt16::C0 => {
                    f8(f7(v0, v53)?, Str::lit(" == 0"))
                }
                Adt16::C1 => {
                    f8(f7(v0, v53)?, Str::lit(" != 0"))
                }
                Adt16::C2 => {
                    f8(f7(v0, v53)?, Str::lit(".uncons()"))
                }
                Adt16::C3 => {
                    f8(f7(v0, v53)?, Str::lit(" - 1"))
                }
            }
        }
        Adt0::C11(r75, r76) => {
            let v55 = (*r75).clone();
            let v56 = (*r76).clone();
            let a23 = f19(v55.clone());
            let a24 = 12u64;
            if if nat_eq(a23, a24) {
                f22(v56.clone())
            } else {
                false
            } {
                f8(v0, Str::lit("{}"))
            } else {
                let v57: Adt20 = f2(f8(v0, Str::lit("{\u{a}"))?, v55, v2, 100u64, 0u64)?;
                let v58: Adt20 = if f22(v56.clone()) {
                    v57
                } else {
                    f8(f2(f21(v57, {
                        let a25 = v2;
                        let a26 = 1u64;
                        nat_add(a25, a26)?
                    })?, v56, {
                        let a27 = v2;
                        let a28 = 1u64;
                        nat_add(a27, a28)?
                    }, 100u64, 0u64)?, Str::lit("\u{a}"))?
                };
                f8(f21(v58, v2)?, Str::lit("}"))
            }
        }
        Adt0::C12 => {
            Ok(v0)
        }
        Adt0::C13(r77, r78) => {
            let v59 = (*r77).clone();
            let v60 = (*r78).clone();
            let a29 = v4;
            let a30 = f23(v3);
            if nat_lt(a29, a30) {
                let v61: Adt20 = f2(v0, v59, v2, 100u64, 0u64)?;
                if f17(v60.clone()) {
                    let a33 = {
                        let a31 = v4;
                        let a32 = 1u64;
                        nat_add(a31, a32)?
                    };
                    let a34 = f23(v3);
                    if nat_lt(a33, a34) {
                        f2(f24(v61, v3, v4)?, v60, v2, v3, {
                            let a35 = v4;
                            let a36 = 1u64;
                            nat_add(a35, a36)?
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
        Adt0::C14(v62, v63, v64, r79, _, r80) => {
            let v65 = (*r79).clone();
            let v67 = (*r80).clone();
            let v68: Adt20 = f25(f8(f21(v0, {
                let a37 = v2;
                let a38 = 1u64;
                nat_add(a37, a38)?
            })?, Str::lit("let "))?, v62)?;
            let v69: Adt20 = if v63 {
                f26(f8(v68, Str::lit(": "))?, v64)?
            } else {
                v68
            };
            let v70: Adt20 = f8(f2(f8(v69, Str::lit(" = "))?, v65, {
                let a39 = v2;
                let a40 = 1u64;
                nat_add(a39, a40)?
            }, 100u64, 0u64)?, Str::lit(";\u{a}"))?;
            f2(v70, v67, v2, 100u64, 0u64)
        }
        Adt0::C15(v71, r81, r82) => {
            let v72 = (*r81).clone();
            let v73 = (*r82).clone();
            let v74: Adt20 = f8(f2(f8(f25(f21(v0, {
                let a41 = v2;
                let a42 = 1u64;
                nat_add(a41, a42)?
            })?, v71)?, Str::lit(" => "))?, v72, {
                let a43 = v2;
                let a44 = 1u64;
                nat_add(a43, a44)?
            }, 100u64, 0u64)?, Str::lit("\u{a}"))?;
            f2(v74, v73, v2, 100u64, 0u64)
        }
        Adt0::C16(v75, r83) => {
            let v76 = (*r83).clone();
            let v77: Adt20 = f7(f8(f21(f8(v0, Str::lit("{\u{a}"))?, {
                let a45 = v2;
                let a46 = 1u64;
                nat_add(a45, a46)?
            })?, Str::lit("let mut "))?, v75.clone())?;
            let v78: Adt20 = f8(f7(f8(v77, Str::lit(" = "))?, v75.clone())?, Str::lit(";\u{a}"))?;
            let v79: Adt20 = f8(f21(v78, {
                let a47 = v2;
                let a48 = 1u64;
                nat_add(a47, a48)?
            })?, Str::lit("loop {\u{a}"))?;
            let v80: Adt20 = f8(f2(f8(f21(v79, {
                let a49 = v2;
                let a50 = 2u64;
                nat_add(a49, a50)?
            })?, Str::lit("let step = "))?, v76, {
                let a51 = v2;
                let a52 = 2u64;
                nat_add(a51, a52)?
            }, 100u64, 0u64)?, Str::lit(";\u{a}"))?;
            let v81: Adt20 = f8(f21(v80, {
                let a53 = v2;
                let a54 = 2u64;
                nat_add(a53, a54)?
            })?, Str::lit("match step {\u{a}"))?;
            let v82: Adt20 = f8(f7(f8(f21(v81, {
                let a55 = v2;
                let a56 = 3u64;
                nat_add(a55, a56)?
            })?, Str::lit("Ok(next) => { "))?, v75)?, Str::lit(" = next; }\u{a}"))?;
            let v83: Adt20 = f8(f21(v82, {
                let a57 = v2;
                let a58 = 3u64;
                nat_add(a57, a58)?
            })?, Str::lit("Err(done) => break done,\u{a}"))?;
            f8(f21(f8(f21(f8(f21(v83, {
                let a59 = v2;
                let a60 = 2u64;
                nat_add(a59, a60)?
            })?, Str::lit("}\u{a}"))?, {
                let a61 = v2;
                let a62 = 1u64;
                nat_add(a61, a62)?
            })?, Str::lit("}\u{a}"))?, v2)?, Str::lit("}"))
        }
    }
}

pub fn f3(v0: u64) -> Adt20 {
    Adt20::C0(List::<Bytes>::nil(), Bytes::lit(&[]), 0u64, v0, false)
}

pub fn f4(v0: List<Bytes>) -> Bytes {
    let m86 = {
        let (h84, _) = f27(Fn0::F28, {
            let a85 = v0.clone();
            length_list(a85)
        }, v0);
        h84
    };
    match m86.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f5(v0: List<Bytes>) -> List<Bytes> {
    f29(Fn1::F30, List::<Bytes>::nil(), v0)
}

pub fn f6(v0: Adt20, v1: Adt1) -> R<Adt20> {
    let m87 = v1;
    match m87 {
        Adt1::C0 => {
            f8(v0, Str::lit("()"))
        }
        Adt1::C1(v2) => {
            if v2 {
                f8(v0, Str::lit("true"))
            } else {
                f8(v0, Str::lit("false"))
            }
        }
        Adt1::C2(v3) => {
            f8(f11(v0, v3)?, Str::lit("u64"))
        }
        Adt1::C3(v4, v5) => {
            if if v4 {
                let a88 = v5;
                let a89 = 9223372036854775808u64;
                nat_eq(a88, a89)
            } else {
                false
            } {
                f8(v0, Str::lit("i64::MIN"))
            } else {
                f8(f11(if v4 {
                    f8(v0, Str::lit("-"))?
                } else {
                    v0
                }, v5)?, Str::lit("i64"))
            }
        }
        Adt1::C4(v6, v7, v8) => {
            if if v7 {
                let a90 = v8;
                let a91 = f31(v6.clone());
                nat_eq(a90, a91)
            } else {
                false
            } {
                f8(f32(v0, v6)?, Str::lit("::MIN"))
            } else {
                f32(f11(if v7 {
                    f8(v0, Str::lit("-"))?
                } else {
                    v0
                }, v8)?, v6)
            }
        }
        Adt1::C5(v9) => {
            f8(f33(f8(v0, Str::lit("Str::lit("))?, v9)?, Str::lit(")"))
        }
        Adt1::C6(v10) => {
            f34(v0, v10)
        }
        Adt1::C7(v11) => {
            f35(v0, v11)
        }
        Adt1::C8 => {
            f8(v0, Str::lit("Overflow"))
        }
    }
}

pub fn f7(v0: Adt20, v1: Adt6) -> R<Adt20> {
    let m92 = v1;
    match m92 {
        Adt6::C0(v2) => {
            f11(f8(v0, Str::lit("v"))?, v2)
        }
        Adt6::C1(v3) => {
            f11(f8(v0, Str::lit("m"))?, v3)
        }
        Adt6::C2(v4) => {
            f11(f8(v0, Str::lit("a"))?, v4)
        }
        Adt6::C3(v5) => {
            f11(f8(v0, Str::lit("c"))?, v5)
        }
        Adt6::C4(v6) => {
            f11(f8(v0, Str::lit("r"))?, v6)
        }
        Adt6::C5(v7) => {
            f11(f8(v0, Str::lit("h"))?, v7)
        }
        Adt6::C6(v8) => {
            f11(f8(v0, Str::lit("k"))?, v8)
        }
        Adt6::C7(v9) => {
            f11(f8(v0, Str::lit("p"))?, v9)
        }
        Adt6::C8(v10) => {
            f11(f8(v0, Str::lit("f"))?, v10)
        }
        Adt6::C9(v11) => {
            f36(v0, v11)
        }
    }
}

pub fn f8(v0: Adt20, v1: Str) -> R<Adt20> {
    f36(v0, {
        let a93 = v1;
        utf8_encode(a93)
    })
}

pub fn f9(v0: Adt0) -> u64 {
    let m94 = v0;
    match m94 {
        Adt0::C0(_, _) => {
            1000u64
        }
        Adt0::C1(_, _, _) => {
            1000u64
        }
        Adt0::C2(_, _, _) => {
            1000u64
        }
        Adt0::C3(_, _, _, _) => {
            1000u64
        }
        Adt0::C4(_, _, _, _) => {
            1000u64
        }
        Adt0::C5(_, _, _) => {
            1000u64
        }
        Adt0::C6(_, _, _) => {
            1000u64
        }
        Adt0::C7(_, _, _, _) => {
            1000u64
        }
        Adt0::C8(_, _, _) => {
            1000u64
        }
        Adt0::C9(_) => {
            1000u64
        }
        Adt0::C10(v31, _, _) => {
            let m95 = v31;
            match m95 {
                Adt16::C0 => {
                    0u64
                }
                Adt16::C1 => {
                    1u64
                }
                Adt16::C2 => {
                    2u64
                }
                Adt16::C3 => {
                    3u64
                }
            }
        }
        Adt0::C11(_, _) => {
            1000u64
        }
        Adt0::C12 => {
            1000u64
        }
        Adt0::C13(_, _) => {
            1000u64
        }
        Adt0::C14(_, _, _, _, _, _) => {
            1000u64
        }
        Adt0::C15(_, _, _) => {
            1000u64
        }
        Adt0::C16(_, _) => {
            1000u64
        }
    }
}

pub fn f10(v0: Adt8) -> u64 {
    let m96 = v0;
    match m96 {
        Adt8::C0(_) => {
            1000u64
        }
        Adt8::C1(_) => {
            1000u64
        }
        Adt8::C2(v3, _) => {
            v3
        }
    }
}

pub fn f11(v0: Adt20, v1: u64) -> R<Adt20> {
    f37(v0, v1)
}

pub fn f12(v0: Adt20, v1: Adt9) -> R<Adt20> {
    let v2: u64 = match v1.clone() {
        Adt9::C0(h97, _) => {
            h97
        }
    };
    let m99 = match v1 {
        Adt9::C0(_, h98) => {
            h98
        }
    };
    match m99 {
        None => {
            f8(v0, {
                let a106 = v2;
                let a107 = 20u64;
                if nat_lt(a106, a107) {
                    f39(v2)
                } else {
                    f40(v2)
                }
            })
        }
        Some(v3) => {
            let a100 = v2;
            let a101 = 24u64;
            if nat_eq(a100, a101) {
                f32(f8(v0, Str::lit("checked_neg_"))?, v3)
            } else {
                let a102 = v2;
                let a103 = 47u64;
                if nat_eq(a102, a103) {
                    f32(f8(v0, Str::lit("format_"))?, v3)
                } else {
                    let a104 = v2;
                    let a105 = 49u64;
                    if nat_eq(a104, a105) {
                        f32(f8(v0, Str::lit("parse_"))?, v3)
                    } else {
                        f8(f8(f32(f8(v0, Str::lit("fixed_"))?, v3)?, Str::lit("::"))?, f38(v2))
                    }
                }
            }
        }
    }
}

pub fn f13(v0: Adt20, v1: u64, v2: Option<Adt2>) -> R<Adt20> {
    let a108 = v1;
    let a109 = 8u64;
    if nat_lt(a108, a109) {
        let m110 = v2;
        match m110 {
            None => {
                Ok(v0)
            }
            Some(v3) => {
                f8(f8(f8(f32(v0, v3)?, Str::lit("::"))?, {
                    let a111 = v1;
                    let a112 = 7u64;
                    if nat_eq(a111, a112) {
                        Str::lit("try_from")
                    } else {
                        f41(v1)
                    }
                })?, Str::lit("("))
            }
        }
    } else {
        f8(v0, f42(v1))
    }
}

pub fn f14(v0: Adt20, v1: u64) -> R<Adt20> {
    let a113 = v1;
    let a114 = 8u64;
    if nat_lt(a113, a114) {
        f8(v0, {
            let a115 = v1;
            let a116 = 7u64;
            if nat_eq(a115, a116) {
                Str::lit(").ok()")
            } else {
                Str::lit(")")
            }
        })
    } else {
        f8(v0, f43(v1))
    }
}

pub fn f15(v0: Adt20, v1: Adt10) -> R<Adt20> {
    let m117 = v1;
    match m117 {
        Adt10::C0(v2) => {
            f8(f26(f8(v0, Str::lit("None::<"))?, v2)?, Str::lit(">"))
        }
        Adt10::C1 => {
            f8(v0, Str::lit("Some("))
        }
        Adt10::C2(v3, v4) => {
            f8(f26(f8(f26(f8(v0, Str::lit("Ok::<"))?, v3)?, Str::lit(", "))?, v4)?, Str::lit(">("))
        }
        Adt10::C3(v5, v6) => {
            f8(f26(f8(f26(f8(v0, Str::lit("Err::<"))?, v5)?, Str::lit(", "))?, v6)?, Str::lit(">("))
        }
        Adt10::C4(v7, v8) => {
            f11(f8(f11(f8(v0, Str::lit("Adt"))?, v7)?, Str::lit("::C"))?, v8)
        }
        Adt10::C5(v9, v10) => {
            f11(f8(f11(f8(v0, Str::lit("Fn"))?, v9)?, Str::lit("::F"))?, v10)
        }
        Adt10::C6 => {
            f8(v0, Str::lit("List::cons("))
        }
        Adt10::C7(v11) => {
            f8(f26(f8(v0, Str::lit("List::<"))?, v11)?, Str::lit(">::nil()"))
        }
    }
}

pub fn f16(v0: Adt10) -> u64 {
    let m118 = v0;
    match m118 {
        Adt10::C0(_) => {
            0u64
        }
        Adt10::C1 => {
            1u64
        }
        Adt10::C2(_, _) => {
            1u64
        }
        Adt10::C3(_, _) => {
            1u64
        }
        Adt10::C4(_, _) => {
            2u64
        }
        Adt10::C5(_, _) => {
            2u64
        }
        Adt10::C6 => {
            1u64
        }
        Adt10::C7(_) => {
            0u64
        }
    }
}

pub fn f17(v0: Adt0) -> bool {
    let a119 = f19(v0);
    let a120 = 13u64;
    nat_eq(a119, a120)
}

pub fn f18(v0: Adt0) -> bool {
    let m121 = v0;
    match m121 {
        Adt0::C0(_, _) => {
            false
        }
        Adt0::C1(_, _, _) => {
            false
        }
        Adt0::C2(_, _, _) => {
            false
        }
        Adt0::C3(_, _, _, _) => {
            false
        }
        Adt0::C4(_, _, _, _) => {
            false
        }
        Adt0::C5(_, _, _) => {
            false
        }
        Adt0::C6(_, _, _) => {
            false
        }
        Adt0::C7(_, _, _, _) => {
            false
        }
        Adt0::C8(_, _, _) => {
            false
        }
        Adt0::C9(_) => {
            false
        }
        Adt0::C10(_, _, _) => {
            false
        }
        Adt0::C11(r124, r125) => {
            let v34 = (*r124).clone();
            let v35 = (*r125).clone();
            let a122 = f19(v34);
            let a123 = 12u64;
            if nat_eq(a122, a123) {
                f22(v35)
            } else {
                false
            }
        }
        Adt0::C12 => {
            false
        }
        Adt0::C13(_, _) => {
            false
        }
        Adt0::C14(_, _, _, _, _, _) => {
            false
        }
        Adt0::C15(_, _, _) => {
            false
        }
        Adt0::C16(_, _) => {
            false
        }
    }
}

pub fn f19(v0: Adt0) -> u64 {
    let m126 = v0;
    match m126 {
        Adt0::C0(_, _) => {
            0u64
        }
        Adt0::C1(_, _, _) => {
            1u64
        }
        Adt0::C2(_, _, _) => {
            2u64
        }
        Adt0::C3(_, _, _, _) => {
            3u64
        }
        Adt0::C4(_, _, _, _) => {
            4u64
        }
        Adt0::C5(_, _, _) => {
            5u64
        }
        Adt0::C6(_, _, _) => {
            6u64
        }
        Adt0::C7(_, _, _, _) => {
            7u64
        }
        Adt0::C8(_, _, _) => {
            8u64
        }
        Adt0::C9(_) => {
            9u64
        }
        Adt0::C10(_, _, _) => {
            10u64
        }
        Adt0::C11(_, _) => {
            11u64
        }
        Adt0::C12 => {
            12u64
        }
        Adt0::C13(_, _) => {
            13u64
        }
        Adt0::C14(_, _, _, _, _, _) => {
            14u64
        }
        Adt0::C15(_, _, _) => {
            15u64
        }
        Adt0::C16(_, _) => {
            16u64
        }
    }
}

pub fn f20(p137: Adt0) -> bool {
    let m136 = p137;
    {
        let mut m136 = m136;
        loop {
            let step = {
                let v0 = m136;
                let m127 = v0;
                match m127 {
                    Adt0::C0(_, _) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C1(_, _, _) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C2(_, _, _) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C3(v9, _, _, _) => {
                        let a128 = f10(v9.clone());
                        let a129 = 16u64;
                        if nat_eq(a128, a129) {
                            Err::<Adt0, bool>(true)
                        } else {
                            let a130 = f10(v9);
                            let a131 = 25u64;
                            Err::<Adt0, bool>(nat_eq(a130, a131))
                        }
                    }
                    Adt0::C4(_, _, _, _) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C5(_, _, _) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C6(_, _, _) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C7(_, _, _, _) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C8(r134, r135, _) => {
                        let v27 = (*r134).clone();
                        let v28 = (*r135).clone();
                        let a132 = f19(v28);
                        let a133 = 15u64;
                        if nat_eq(a132, a133) {
                            Ok::<Adt0, bool>(v27)
                        } else {
                            Err::<Adt0, bool>(false)
                        }
                    }
                    Adt0::C9(_) => {
                        Err::<Adt0, bool>(true)
                    }
                    Adt0::C10(_, _, _) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C11(_, _) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C12 => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C13(_, _) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C14(_, _, _, _, _, _) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C15(_, _, _) => {
                        Err::<Adt0, bool>(false)
                    }
                    Adt0::C16(_, _) => {
                        Err::<Adt0, bool>(true)
                    }
                }
            };
            match step {
                Ok(next) => { m136 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f21(v0: Adt20, v1: u64) -> R<Adt20> {
    let (_, h138) = {
        let (h139, _) = f44(Fn2::F45(v1), v1, (0u64, v0))?;
        h139
    };
    Ok(h138)
}

pub fn f22(v0: Adt0) -> bool {
    let m140 = v0;
    match m140 {
        Adt0::C0(v1, _) => {
            let m141 = v1;
            match m141 {
                Adt1::C0 => {
                    true
                }
                Adt1::C1(_) => {
                    false
                }
                Adt1::C2(_) => {
                    false
                }
                Adt1::C3(_, _) => {
                    false
                }
                Adt1::C4(_, _, _) => {
                    false
                }
                Adt1::C5(_) => {
                    false
                }
                Adt1::C6(_) => {
                    false
                }
                Adt1::C7(_) => {
                    false
                }
                Adt1::C8 => {
                    false
                }
            }
        }
        Adt0::C1(_, _, _) => {
            false
        }
        Adt0::C2(_, _, _) => {
            false
        }
        Adt0::C3(_, _, _, _) => {
            false
        }
        Adt0::C4(_, _, _, _) => {
            false
        }
        Adt0::C5(_, _, _) => {
            false
        }
        Adt0::C6(_, _, _) => {
            false
        }
        Adt0::C7(_, _, _, _) => {
            false
        }
        Adt0::C8(_, _, _) => {
            false
        }
        Adt0::C9(_) => {
            false
        }
        Adt0::C10(_, _, _) => {
            false
        }
        Adt0::C11(_, _) => {
            false
        }
        Adt0::C12 => {
            false
        }
        Adt0::C13(_, _) => {
            false
        }
        Adt0::C14(_, _, _, _, _, _) => {
            false
        }
        Adt0::C15(_, _, _) => {
            false
        }
        Adt0::C16(_, _) => {
            false
        }
    }
}

pub fn f23(v0: u64) -> u64 {
    let a142 = v0;
    let a143 = 8u64;
    if if nat_lt(a142, a143) {
        true
    } else {
        let a144 = 40u64;
        let a145 = v0;
        nat_le(a144, a145)
    } {
        100000u64
    } else {
        let a146 = v0;
        let a147 = 11u64;
        if nat_eq(a146, a147) {
            1u64
        } else {
            let a148 = v0;
            let a149 = 24u64;
            if if nat_eq(a148, a149) {
                true
            } else {
                let a150 = v0;
                let a151 = 26u64;
                if nat_eq(a150, a151) {
                    true
                } else {
                    let a152 = v0;
                    let a153 = 31u64;
                    if nat_eq(a152, a153) {
                        true
                    } else {
                        let a154 = v0;
                        let a155 = 32u64;
                        if nat_eq(a154, a155) {
                            true
                        } else {
                            let a156 = v0;
                            let a157 = 35u64;
                            if nat_eq(a156, a157) {
                                true
                            } else {
                                let a158 = v0;
                                let a159 = 36u64;
                                nat_eq(a158, a159)
                            }
                        }
                    }
                }
            } {
                1u64
            } else {
                let a160 = v0;
                let a161 = 15u64;
                if nat_eq(a160, a161) {
                    3u64
                } else {
                    let a162 = v0;
                    let a163 = 16u64;
                    if nat_eq(a162, a163) {
                        3u64
                    } else {
                        let a164 = v0;
                        let a165 = 20u64;
                        if nat_eq(a164, a165) {
                            3u64
                        } else {
                            let a166 = v0;
                            let a167 = 25u64;
                            if if nat_eq(a166, a167) {
                                true
                            } else {
                                let a168 = v0;
                                let a169 = 29u64;
                                if nat_eq(a168, a169) {
                                    true
                                } else {
                                    let a170 = v0;
                                    let a171 = 34u64;
                                    if nat_eq(a170, a171) {
                                        true
                                    } else {
                                        let a172 = v0;
                                        let a173 = 38u64;
                                        nat_eq(a172, a173)
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

pub fn f24(v0: Adt20, v1: u64, v2: u64) -> R<Adt20> {
    let a174 = v1;
    let a175 = 8u64;
    if nat_lt(a174, a175) {
        f8(v0, Str::lit(", "))
    } else {
        let a176 = v1;
        let a177 = 100u64;
        if nat_eq(a176, a177) {
            f8(v0, Str::lit(", "))
        } else {
            let a178 = v2;
            let a179 = 0u64;
            if nat_eq(a178, a179) {
                f8(v0, f46(v1))
            } else {
                f8(v0, f47(v1))
            }
        }
    }
}

pub fn f25(v0: Adt20, v1: Adt17) -> R<Adt20> {
    let m180 = v1;
    match m180 {
        Adt17::C0(v2) => {
            f8(v0, {
                let m181 = v2;
                match m181 {
                    Adt18::C0 => {
                        Str::lit("_")
                    }
                    Adt18::C1 => {
                        Str::lit("()")
                    }
                    Adt18::C2 => {
                        Str::lit("None")
                    }
                }
            })
        }
        Adt17::C1(v3) => {
            f7(v0, v3)
        }
        Adt17::C2(r185) => {
            let v4 = (*r185).clone();
            f8(f25(f8(v0, Str::lit("("))?, v4)?, Str::lit(")"))
        }
        Adt17::C3(v5, r186) => {
            let v6 = (*r186).clone();
            f8(f25(f8(v0, {
                let m182 = v5;
                match m182 {
                    Adt19::C0 => {
                        Str::lit("Some(")
                    }
                    Adt19::C1 => {
                        Str::lit("Ok(")
                    }
                    Adt19::C2 => {
                        Str::lit("Err(")
                    }
                }
            })?, v6)?, Str::lit(")"))
        }
        Adt17::C4(v7, v8, r187) => {
            let v9 = (*r187).clone();
            let v10: Adt20 = f11(f8(f11(f8(v0, Str::lit("Adt"))?, v7)?, Str::lit("::C"))?, v8)?;
            let m183 = v9.clone();
            match m183 {
                Adt17::C0(_) => {
                    Ok(v10)
                }
                Adt17::C1(_) => {
                    Ok(v10)
                }
                Adt17::C2(_) => {
                    Ok(v10)
                }
                Adt17::C3(_, _) => {
                    Ok(v10)
                }
                Adt17::C4(_, _, _) => {
                    Ok(v10)
                }
                Adt17::C5 => {
                    Ok(v10)
                }
                Adt17::C6(_, _) => {
                    f8(f25(f8(v10, Str::lit("("))?, v9)?, Str::lit(")"))
                }
                Adt17::C7(_) => {
                    Ok(v10)
                }
            }
        }
        Adt17::C5 => {
            Ok(v0)
        }
        Adt17::C6(r188, r189) => {
            let v22 = (*r188).clone();
            let v23 = (*r189).clone();
            let m184 = v23.clone();
            match m184 {
                Adt17::C0(_) => {
                    f25(v0, v22)
                }
                Adt17::C1(_) => {
                    f25(v0, v22)
                }
                Adt17::C2(_) => {
                    f25(v0, v22)
                }
                Adt17::C3(_, _) => {
                    f25(v0, v22)
                }
                Adt17::C4(_, _, _) => {
                    f25(v0, v22)
                }
                Adt17::C5 => {
                    f25(v0, v22)
                }
                Adt17::C6(_, _) => {
                    f25(f8(f25(v0, v22)?, Str::lit(", "))?, v23)
                }
                Adt17::C7(_) => {
                    f25(v0, v22)
                }
            }
        }
        Adt17::C7(v35) => {
            f35(v0, v35)
        }
    }
}

pub fn f26(v0: Adt20, v1: Adt11) -> R<Adt20> {
    let m190 = v1;
    match m190 {
        Adt11::C0(v2) => {
            f48(v0, v2)
        }
        Adt11::C1(v3) => {
            f32(v0, v3)
        }
        Adt11::C2(v4, r194) => {
            let v5 = (*r194).clone();
            f8(f26(f8(v0, f49(v4.clone()))?, v5)?, f50(v4))
        }
        Adt11::C3(v6, r195, r196) => {
            let v7 = (*r195).clone();
            let v8 = (*r196).clone();
            let m191 = v6;
            match m191 {
                Adt14::C0 => {
                    f8(f26(f8(f26(f8(v0, Str::lit("Result<"))?, v7)?, Str::lit(", "))?, v8)?, Str::lit(">"))
                }
                Adt14::C1 => {
                    f8(f26(f8(f26(f8(v0, Str::lit("("))?, v7)?, Str::lit(", "))?, v8)?, Str::lit(")"))
                }
            }
        }
        Adt11::C4(v9, v10) => {
            let m192 = v9;
            match m192 {
                Adt15::C0 => {
                    f11(f8(v0, Str::lit("Adt"))?, v10)
                }
                Adt15::C1 => {
                    f11(f8(v0, Str::lit("Fn"))?, v10)
                }
                Adt15::C2 => {
                    f51(v0, v10)
                }
            }
        }
        Adt11::C5 => {
            Ok(v0)
        }
        Adt11::C6(r197, r198) => {
            let v11 = (*r197).clone();
            let v12 = (*r198).clone();
            let m193 = v12.clone();
            match m193 {
                Adt11::C0(_) => {
                    f26(v0, v11)
                }
                Adt11::C1(_) => {
                    f26(v0, v11)
                }
                Adt11::C2(_, _) => {
                    f26(v0, v11)
                }
                Adt11::C3(_, _, _) => {
                    f26(v0, v11)
                }
                Adt11::C4(_, _) => {
                    f26(v0, v11)
                }
                Adt11::C5 => {
                    f26(v0, v11)
                }
                Adt11::C6(_, _) => {
                    f26(f8(f26(v0, v11)?, Str::lit(", "))?, v12)
                }
            }
        }
    }
}

pub fn f27(v0: Fn0, p203: u64, p204: List<Bytes>) -> (List<Bytes>, bool) {
    let m202 = (p203, p204);
    {
        let mut m202 = m202;
        loop {
            let step = {
                let (v1, v2) = m202;
                let m199 = v1;
                if m199 == 0 {
                    Err::<(u64, List<Bytes>), (List<Bytes>, bool)>((v2, false))
                } else {
                    let v3 = m199 - 1;
                    let m201 = {
                        let c200: Fn0 = v0.clone();
                        c200.apply(v2.clone())
                    };
                    match m201 {
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
                Ok(next) => { m202 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f28(v0: List<Bytes>) -> Option<List<Bytes>> {
    f52(v0)
}

pub fn f29(v0: Fn1, p208: List<Bytes>, p209: List<Bytes>) -> List<Bytes> {
    let m207 = (p208, p209);
    {
        let mut m207 = m207;
        loop {
            let step = {
                let (v1, v2) = m207;
                let m205 = v2;
                match m205.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c206: Fn1 = v0.clone();
                            c206.apply(v1, v3)
                        }, v4))
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

pub fn f30(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f31(v0: Adt2) -> u64 {
    let m210 = v0;
    match m210 {
        Adt2::C0 => {
            0u64
        }
        Adt2::C1 => {
            0u64
        }
        Adt2::C2 => {
            0u64
        }
        Adt2::C3 => {
            0u64
        }
        Adt2::C4 => {
            128u64
        }
        Adt2::C5 => {
            32768u64
        }
        Adt2::C6 => {
            2147483648u64
        }
        Adt2::C7 => {
            9223372036854775808u64
        }
    }
}

pub fn f32(v0: Adt20, v1: Adt2) -> R<Adt20> {
    f8(v0, f53(v1))
}

pub fn f33(v0: Adt20, v1: Bytes) -> R<Adt20> {
    f8({
        let (_, h211) = {
            let (h212, _) = f44(Fn2::F54(v1.clone()), {
                let a213 = v1;
                length_bytes(a213)
            }, (0u64, f8(v0, Str::lit("\""))?))?;
            h212
        };
        h211
    }, Str::lit("\""))
}

pub fn f34(v0: Adt20, v1: Bytes) -> R<Adt20> {
    f8({
        let (_, h214) = {
            let (h215, _) = f44(Fn2::F55(v1.clone()), {
                let a216 = v1;
                length_bytes(a216)
            }, (0u64, f8(v0, Str::lit("Bytes::lit(&["))?))?;
            h215
        };
        h214
    }, Str::lit("])"))
}

pub fn f35(v0: Adt20, v1: Adt3) -> R<Adt20> {
    f8(v0, {
        let m217 = v1;
        match m217 {
            Adt3::C0 => {
                Str::lit("Ordering::Less")
            }
            Adt3::C1 => {
                Str::lit("Ordering::Equal")
            }
            Adt3::C2 => {
                Str::lit("Ordering::Greater")
            }
        }
    })
}

pub fn f36(v0: Adt20, v1: Bytes) -> R<Adt20> {
    if match v0.clone() {
        Adt20::C0(_, _, _, _, h218) => {
            h218
        }
    } {
        Ok(v0)
    } else {
        let v2: u64 = {
            let a221 = match v0.clone() {
                Adt20::C0(_, _, h219, _, _) => {
                    h219
                }
            };
            let a222 = {
                let a220 = v1.clone();
                length_bytes(a220)
            };
            nat_add(a221, a222)?
        };
        let a224 = v2;
        let a225 = match v0.clone() {
            Adt20::C0(_, _, _, h223, _) => {
                h223
            }
        };
        if nat_le(a224, a225) {
            Ok(f56(v0.clone(), {
                let a227 = match v0 {
                    Adt20::C0(_, h226, _, _, _) => {
                        h226
                    }
                };
                let a228 = v1;
                append_bytes(a227, a228)
            }, v2))
        } else {
            Ok(f57(v0))
        }
    }
}

pub fn f37(v0: Adt20, v1: u64) -> R<Adt20> {
    f36(v0, f58(v1)?)
}

pub fn f38(v0: u64) -> Str {
    let a229 = v0;
    let a230 = 20u64;
    if nat_eq(a229, a230) {
        Str::lit("checked_add")
    } else {
        let a231 = v0;
        let a232 = 21u64;
        if nat_eq(a231, a232) {
            Str::lit("checked_sub")
        } else {
            let a233 = v0;
            let a234 = 22u64;
            if nat_eq(a233, a234) {
                Str::lit("checked_mul")
            } else {
                let a235 = v0;
                let a236 = 23u64;
                if nat_eq(a235, a236) {
                    Str::lit("checked_quot")
                } else {
                    let a237 = v0;
                    let a238 = 25u64;
                    if nat_eq(a237, a238) {
                        Str::lit("bit_and")
                    } else {
                        let a239 = v0;
                        let a240 = 26u64;
                        if nat_eq(a239, a240) {
                            Str::lit("bit_or")
                        } else {
                            let a241 = v0;
                            let a242 = 27u64;
                            if nat_eq(a241, a242) {
                                Str::lit("bit_xor")
                            } else {
                                let a243 = v0;
                                let a244 = 28u64;
                                if nat_eq(a243, a244) {
                                    Str::lit("bit_not")
                                } else {
                                    let a245 = v0;
                                    let a246 = 29u64;
                                    if nat_eq(a245, a246) {
                                        Str::lit("shift_left")
                                    } else {
                                        let a247 = v0;
                                        let a248 = 30u64;
                                        if nat_eq(a247, a248) {
                                            Str::lit("shift_right")
                                        } else {
                                            let a249 = v0;
                                            let a250 = 31u64;
                                            if nat_eq(a249, a250) {
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

pub fn f39(v0: u64) -> Str {
    let a251 = v0;
    let a252 = 0u64;
    if nat_eq(a251, a252) {
        Str::lit("nat_add")
    } else {
        let a253 = v0;
        let a254 = 1u64;
        if nat_eq(a253, a254) {
            Str::lit("nat_sub")
        } else {
            let a255 = v0;
            let a256 = 2u64;
            if nat_eq(a255, a256) {
                Str::lit("nat_mul")
            } else {
                let a257 = v0;
                let a258 = 3u64;
                if nat_eq(a257, a258) {
                    Str::lit("nat_quot")
                } else {
                    let a259 = v0;
                    let a260 = 4u64;
                    if nat_eq(a259, a260) {
                        Str::lit("nat_rem")
                    } else {
                        let a261 = v0;
                        let a262 = 5u64;
                        if nat_eq(a261, a262) {
                            Str::lit("nat_eq")
                        } else {
                            let a263 = v0;
                            let a264 = 6u64;
                            if nat_eq(a263, a264) {
                                Str::lit("nat_le")
                            } else {
                                let a265 = v0;
                                let a266 = 7u64;
                                if nat_eq(a265, a266) {
                                    Str::lit("nat_lt")
                                } else {
                                    let a267 = v0;
                                    let a268 = 8u64;
                                    if nat_eq(a267, a268) {
                                        Str::lit("nat_succ")
                                    } else {
                                        let a269 = v0;
                                        let a270 = 9u64;
                                        if nat_eq(a269, a270) {
                                            Str::lit("int_add")
                                        } else {
                                            let a271 = v0;
                                            let a272 = 10u64;
                                            if nat_eq(a271, a272) {
                                                Str::lit("int_sub")
                                            } else {
                                                let a273 = v0;
                                                let a274 = 11u64;
                                                if nat_eq(a273, a274) {
                                                    Str::lit("int_mul")
                                                } else {
                                                    let a275 = v0;
                                                    let a276 = 12u64;
                                                    if nat_eq(a275, a276) {
                                                        Str::lit("int_neg")
                                                    } else {
                                                        let a277 = v0;
                                                        let a278 = 13u64;
                                                        if nat_eq(a277, a278) {
                                                            Str::lit("int_quot")
                                                        } else {
                                                            let a279 = v0;
                                                            let a280 = 14u64;
                                                            if nat_eq(a279, a280) {
                                                                Str::lit("int_rem")
                                                            } else {
                                                                let a281 = v0;
                                                                let a282 = 15u64;
                                                                if nat_eq(a281, a282) {
                                                                    Str::lit("bool_not")
                                                                } else {
                                                                    let a283 = v0;
                                                                    let a284 = 16u64;
                                                                    if nat_eq(a283, a284) {
                                                                        Str::lit("bool_and")
                                                                    } else {
                                                                        let a285 = v0;
                                                                        let a286 = 17u64;
                                                                        if nat_eq(a285, a286) {
                                                                            Str::lit("bool_or")
                                                                        } else {
                                                                            let a287 = v0;
                                                                            let a288 = 18u64;
                                                                            if nat_eq(a287, a288) {
                                                                                Str::lit("equal")
                                                                            } else {
                                                                                let a289 = v0;
                                                                                let a290 = 19u64;
                                                                                if nat_eq(a289, a290) {
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

pub fn f40(v0: u64) -> Str {
    let a291 = v0;
    let a292 = 32u64;
    if nat_eq(a291, a292) {
        Str::lit("append_list")
    } else {
        let a293 = v0;
        let a294 = 33u64;
        if nat_eq(a293, a294) {
            Str::lit("append_bytes")
        } else {
            let a295 = v0;
            let a296 = 34u64;
            if nat_eq(a295, a296) {
                Str::lit("length_list")
            } else {
                let a297 = v0;
                let a298 = 35u64;
                if nat_eq(a297, a298) {
                    Str::lit("length_bytes")
                } else {
                    let a299 = v0;
                    let a300 = 36u64;
                    if nat_eq(a299, a300) {
                        Str::lit("length_string")
                    } else {
                        let a301 = v0;
                        let a302 = 37u64;
                        if nat_eq(a301, a302) {
                            Str::lit("index_list")
                        } else {
                            let a303 = v0;
                            let a304 = 38u64;
                            if nat_eq(a303, a304) {
                                Str::lit("index_bytes")
                            } else {
                                let a305 = v0;
                                let a306 = 39u64;
                                if nat_eq(a305, a306) {
                                    Str::lit("slice_list")
                                } else {
                                    let a307 = v0;
                                    let a308 = 40u64;
                                    if nat_eq(a307, a308) {
                                        Str::lit("slice_bytes")
                                    } else {
                                        let a309 = v0;
                                        let a310 = 41u64;
                                        if nat_eq(a309, a310) {
                                            Str::lit("utf8_encode")
                                        } else {
                                            let a311 = v0;
                                            let a312 = 42u64;
                                            if nat_eq(a311, a312) {
                                                Str::lit("utf8_decode")
                                            } else {
                                                let a313 = v0;
                                                let a314 = 43u64;
                                                if nat_eq(a313, a314) {
                                                    Str::lit("compare_bytes")
                                                } else {
                                                    let a315 = v0;
                                                    let a316 = 44u64;
                                                    if nat_eq(a315, a316) {
                                                        Str::lit("split_exact")
                                                    } else {
                                                        let a317 = v0;
                                                        let a318 = 45u64;
                                                        if nat_eq(a317, a318) {
                                                            Str::lit("join")
                                                        } else {
                                                            let a319 = v0;
                                                            let a320 = 46u64;
                                                            if nat_eq(a319, a320) {
                                                                Str::lit("format_i64")
                                                            } else {
                                                                let a321 = v0;
                                                                let a322 = 48u64;
                                                                if nat_eq(a321, a322) {
                                                                    Str::lit("parse_int")
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

pub fn f41(v0: u64) -> Str {
    let a323 = v0;
    let a324 = 0u64;
    if nat_eq(a323, a324) {
        Str::lit("checked_add")
    } else {
        let a325 = v0;
        let a326 = 1u64;
        if nat_eq(a325, a326) {
            Str::lit("checked_sub")
        } else {
            let a327 = v0;
            let a328 = 2u64;
            if nat_eq(a327, a328) {
                Str::lit("checked_mul")
            } else {
                let a329 = v0;
                let a330 = 3u64;
                if nat_eq(a329, a330) {
                    Str::lit("checked_div")
                } else {
                    let a331 = v0;
                    let a332 = 4u64;
                    if nat_eq(a331, a332) {
                        Str::lit("checked_neg")
                    } else {
                        let a333 = v0;
                        let a334 = 5u64;
                        if nat_eq(a333, a334) {
                            Str::lit("checked_shl")
                        } else {
                            let a335 = v0;
                            let a336 = 6u64;
                            if nat_eq(a335, a336) {
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

pub fn f42(v0: u64) -> Str {
    let a337 = v0;
    let a338 = 30u64;
    if nat_eq(a337, a338) {
        Str::lit("Bytes::append(")
    } else {
        let a339 = v0;
        let a340 = 31u64;
        if nat_eq(a339, a340) {
            Str::lit("Bytes::length(&")
        } else {
            let a341 = v0;
            let a342 = 32u64;
            if nat_eq(a341, a342) {
                Str::lit("Str::length(&")
            } else {
                let a343 = v0;
                let a344 = 33u64;
                if nat_eq(a343, a344) {
                    Str::lit("Bytes::index(&")
                } else {
                    let a345 = v0;
                    let a346 = 34u64;
                    if nat_eq(a345, a346) {
                        Str::lit("Bytes::slice(&")
                    } else {
                        let a347 = v0;
                        let a348 = 35u64;
                        if nat_eq(a347, a348) {
                            Str::lit("Str::utf8_encode(&")
                        } else {
                            let a349 = v0;
                            let a350 = 36u64;
                            if nat_eq(a349, a350) {
                                Str::lit("Bytes::utf8_decode(&")
                            } else {
                                let a351 = v0;
                                let a352 = 37u64;
                                if nat_eq(a351, a352) {
                                    Str::lit("Bytes::compare(&")
                                } else {
                                    let a353 = v0;
                                    let a354 = 38u64;
                                    if nat_eq(a353, a354) {
                                        Str::lit("Str::split_exact(&")
                                    } else {
                                        let a355 = v0;
                                        let a356 = 39u64;
                                        if nat_eq(a355, a356) {
                                            Str::lit("Str::join(")
                                        } else {
                                            let a357 = v0;
                                            let a358 = 8u64;
                                            if nat_eq(a357, a358) {
                                                Str::lit("core::ops::BitAnd::bitand(")
                                            } else {
                                                let a359 = v0;
                                                let a360 = 9u64;
                                                if nat_eq(a359, a360) {
                                                    Str::lit("core::ops::BitOr::bitor(")
                                                } else {
                                                    let a361 = v0;
                                                    let a362 = 10u64;
                                                    if nat_eq(a361, a362) {
                                                        Str::lit("core::ops::BitXor::bitxor(")
                                                    } else {
                                                        let a363 = v0;
                                                        let a364 = 11u64;
                                                        if nat_eq(a363, a364) {
                                                            Str::lit("core::ops::Not::not(")
                                                        } else {
                                                            let a365 = v0;
                                                            let a366 = 12u64;
                                                            if nat_eq(a365, a366) {
                                                                Str::lit("core::cmp::PartialEq::eq(&")
                                                            } else {
                                                                let a367 = v0;
                                                                let a368 = 13u64;
                                                                if nat_eq(a367, a368) {
                                                                    Str::lit("core::cmp::PartialOrd::le(&")
                                                                } else {
                                                                    let a369 = v0;
                                                                    let a370 = 14u64;
                                                                    if nat_eq(a369, a370) {
                                                                        Str::lit("core::cmp::PartialOrd::lt(&")
                                                                    } else {
                                                                        let a371 = v0;
                                                                        let a372 = 15u64;
                                                                        if nat_eq(a371, a372) {
                                                                            Str::lit("u64::checked_rem(")
                                                                        } else {
                                                                            let a373 = v0;
                                                                            let a374 = 16u64;
                                                                            if nat_eq(a373, a374) {
                                                                                Str::lit("{ let (a, b, z): (i64, i64, i64) = (")
                                                                            } else {
                                                                                let a375 = v0;
                                                                                let a376 = 17u64;
                                                                                if nat_eq(a375, a376) {
                                                                                    Str::lit("u64::checked_add(")
                                                                                } else {
                                                                                    let a377 = v0;
                                                                                    let a378 = 18u64;
                                                                                    if nat_eq(a377, a378) {
                                                                                        Str::lit("u64::saturating_sub(")
                                                                                    } else {
                                                                                        let a379 = v0;
                                                                                        let a380 = 19u64;
                                                                                        if nat_eq(a379, a380) {
                                                                                            Str::lit("u64::checked_mul(")
                                                                                        } else {
                                                                                            let a381 = v0;
                                                                                            let a382 = 20u64;
                                                                                            if nat_eq(a381, a382) {
                                                                                                Str::lit("u64::checked_div(")
                                                                                            } else {
                                                                                                let a383 = v0;
                                                                                                let a384 = 21u64;
                                                                                                if nat_eq(a383, a384) {
                                                                                                    Str::lit("i64::checked_add(")
                                                                                                } else {
                                                                                                    let a385 = v0;
                                                                                                    let a386 = 22u64;
                                                                                                    if nat_eq(a385, a386) {
                                                                                                        Str::lit("i64::checked_sub(")
                                                                                                    } else {
                                                                                                        let a387 = v0;
                                                                                                        let a388 = 23u64;
                                                                                                        if nat_eq(a387, a388) {
                                                                                                            Str::lit("i64::checked_mul(")
                                                                                                        } else {
                                                                                                            let a389 = v0;
                                                                                                            let a390 = 24u64;
                                                                                                            if nat_eq(a389, a390) {
                                                                                                                Str::lit("i64::checked_neg(")
                                                                                                            } else {
                                                                                                                let a391 = v0;
                                                                                                                let a392 = 25u64;
                                                                                                                if nat_eq(a391, a392) {
                                                                                                                    Str::lit("{ let (a, b, z): (i64, i64, i64) = (")
                                                                                                                } else {
                                                                                                                    let a393 = v0;
                                                                                                                    let a394 = 26u64;
                                                                                                                    if nat_eq(a393, a394) {
                                                                                                                        Str::lit("List::length(&")
                                                                                                                    } else {
                                                                                                                        let a395 = v0;
                                                                                                                        let a396 = 27u64;
                                                                                                                        if nat_eq(a395, a396) {
                                                                                                                            Str::lit("List::append(")
                                                                                                                        } else {
                                                                                                                            let a397 = v0;
                                                                                                                            let a398 = 28u64;
                                                                                                                            if nat_eq(a397, a398) {
                                                                                                                                Str::lit("List::index(&")
                                                                                                                            } else {
                                                                                                                                let a399 = v0;
                                                                                                                                let a400 = 29u64;
                                                                                                                                if nat_eq(a399, a400) {
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

pub fn f43(v0: u64) -> Str {
    let a401 = v0;
    let a402 = 16u64;
    if nat_eq(a401, a402) {
        Str::lit("); if b == 0 { z } else { a.wrapping_rem(b) } }")
    } else {
        let a403 = v0;
        let a404 = 25u64;
        if nat_eq(a403, a404) {
            Str::lit("); if b == 0 { Some(z) } else { a.checked_div(b) } }")
        } else {
            let a405 = v0;
            let a406 = 15u64;
            if nat_eq(a405, a406) {
                Str::lit(")")
            } else {
                let a407 = v0;
                let a408 = 20u64;
                if nat_eq(a407, a408) {
                    Str::lit(")")
                } else {
                    let a409 = v0;
                    let a410 = 8u64;
                    if nat_eq(a409, a410) {
                        Str::lit(")")
                    } else {
                        let a411 = v0;
                        let a412 = 9u64;
                        if nat_eq(a411, a412) {
                            Str::lit(")")
                        } else {
                            let a413 = v0;
                            let a414 = 10u64;
                            if nat_eq(a413, a414) {
                                Str::lit(")")
                            } else {
                                let a415 = v0;
                                let a416 = 11u64;
                                if nat_eq(a415, a416) {
                                    Str::lit(")")
                                } else {
                                    let a417 = v0;
                                    let a418 = 12u64;
                                    if nat_eq(a417, a418) {
                                        Str::lit(")")
                                    } else {
                                        let a419 = v0;
                                        let a420 = 13u64;
                                        if nat_eq(a419, a420) {
                                            Str::lit(")")
                                        } else {
                                            let a421 = v0;
                                            let a422 = 14u64;
                                            if nat_eq(a421, a422) {
                                                Str::lit(")")
                                            } else {
                                                let a423 = v0;
                                                let a424 = 17u64;
                                                if nat_eq(a423, a424) {
                                                    Str::lit(")")
                                                } else {
                                                    let a425 = v0;
                                                    let a426 = 18u64;
                                                    if nat_eq(a425, a426) {
                                                        Str::lit(")")
                                                    } else {
                                                        let a427 = v0;
                                                        let a428 = 19u64;
                                                        if nat_eq(a427, a428) {
                                                            Str::lit(")")
                                                        } else {
                                                            let a429 = v0;
                                                            let a430 = 21u64;
                                                            if nat_eq(a429, a430) {
                                                                Str::lit(")")
                                                            } else {
                                                                let a431 = v0;
                                                                let a432 = 22u64;
                                                                if nat_eq(a431, a432) {
                                                                    Str::lit(")")
                                                                } else {
                                                                    let a433 = v0;
                                                                    let a434 = 23u64;
                                                                    if nat_eq(a433, a434) {
                                                                        Str::lit(")")
                                                                    } else {
                                                                        let a435 = v0;
                                                                        let a436 = 24u64;
                                                                        let _ = nat_eq(a435, a436);
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

pub fn f44(v0: Fn2, p441: u64, p442: (u64, Adt20)) -> R<((u64, Adt20), bool)> {
    let m440 = (p441, p442);
    {
        let mut m440 = m440;
        loop {
            let step = {
                let (v1, v2) = m440;
                let m437 = v1;
                if m437 == 0 {
                    Err::<(u64, (u64, Adt20)), R<((u64, Adt20), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m437 - 1;
                    let m439 = {
                        let c438: Fn2 = v0.clone();
                        c438.apply(v2.clone())?
                    };
                    match m439 {
                        None => {
                            Err::<(u64, (u64, Adt20)), R<((u64, Adt20), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (u64, Adt20)), R<((u64, Adt20), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m440 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f45(v0: u64, v1: (u64, Adt20)) -> R<Option<(u64, Adt20)>> {
    f59(v0, v1)
}

pub fn f46(v0: u64) -> Str {
    let a443 = v0;
    let a444 = 37u64;
    if nat_eq(a443, a444) {
        Str::lit(", &")
    } else {
        let a445 = v0;
        let a446 = 38u64;
        if nat_eq(a445, a446) {
            Str::lit(", &")
        } else {
            let a447 = v0;
            let a448 = 39u64;
            if nat_eq(a447, a448) {
                Str::lit(", &")
            } else {
                let a449 = v0;
                let a450 = 12u64;
                if nat_eq(a449, a450) {
                    Str::lit(", &")
                } else {
                    let a451 = v0;
                    let a452 = 13u64;
                    if nat_eq(a451, a452) {
                        Str::lit(", &")
                    } else {
                        let a453 = v0;
                        let a454 = 14u64;
                        if nat_eq(a453, a454) {
                            Str::lit(", &")
                        } else {
                            let a455 = v0;
                            let a456 = 8u64;
                            if nat_eq(a455, a456) {
                                Str::lit(", ")
                            } else {
                                let a457 = v0;
                                let a458 = 9u64;
                                if nat_eq(a457, a458) {
                                    Str::lit(", ")
                                } else {
                                    let a459 = v0;
                                    let a460 = 10u64;
                                    if nat_eq(a459, a460) {
                                        Str::lit(", ")
                                    } else {
                                        let a461 = v0;
                                        let a462 = 11u64;
                                        if nat_eq(a461, a462) {
                                            Str::lit(", ")
                                        } else {
                                            let a463 = v0;
                                            let a464 = 15u64;
                                            if nat_eq(a463, a464) {
                                                Str::lit(", ")
                                            } else {
                                                let a465 = v0;
                                                let a466 = 16u64;
                                                if nat_eq(a465, a466) {
                                                    Str::lit(", ")
                                                } else {
                                                    let a467 = v0;
                                                    let a468 = 17u64;
                                                    if nat_eq(a467, a468) {
                                                        Str::lit(", ")
                                                    } else {
                                                        let a469 = v0;
                                                        let a470 = 18u64;
                                                        if nat_eq(a469, a470) {
                                                            Str::lit(", ")
                                                        } else {
                                                            let a471 = v0;
                                                            let a472 = 19u64;
                                                            if nat_eq(a471, a472) {
                                                                Str::lit(", ")
                                                            } else {
                                                                let a473 = v0;
                                                                let a474 = 20u64;
                                                                if nat_eq(a473, a474) {
                                                                    Str::lit(", ")
                                                                } else {
                                                                    let a475 = v0;
                                                                    let a476 = 21u64;
                                                                    if nat_eq(a475, a476) {
                                                                        Str::lit(", ")
                                                                    } else {
                                                                        let a477 = v0;
                                                                        let a478 = 22u64;
                                                                        if nat_eq(a477, a478) {
                                                                            Str::lit(", ")
                                                                        } else {
                                                                            let a479 = v0;
                                                                            let a480 = 23u64;
                                                                            if nat_eq(a479, a480) {
                                                                                Str::lit(", ")
                                                                            } else {
                                                                                let a481 = v0;
                                                                                let a482 = 24u64;
                                                                                if nat_eq(a481, a482) {
                                                                                    Str::lit(", ")
                                                                                } else {
                                                                                    let a483 = v0;
                                                                                    let a484 = 25u64;
                                                                                    let _ = nat_eq(a483, a484);
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

pub fn f47(v0: u64) -> Str {
    let a485 = v0;
    let a486 = 15u64;
    if nat_eq(a485, a486) {
        Str::lit(").unwrap_or(")
    } else {
        let a487 = v0;
        let a488 = 16u64;
        if nat_eq(a487, a488) {
            Str::lit(", ")
        } else {
            let a489 = v0;
            let a490 = 20u64;
            if nat_eq(a489, a490) {
                Str::lit(").unwrap_or(")
            } else {
                let a491 = v0;
                let a492 = 25u64;
                let _ = nat_eq(a491, a492);
                Str::lit(", ")
            }
        }
    }
}

pub fn f48(v0: Adt20, v1: Adt12) -> R<Adt20> {
    f8(v0, {
        let m493 = v1;
        match m493 {
            Adt12::C0 => {
                Str::lit("()")
            }
            Adt12::C1 => {
                Str::lit("bool")
            }
            Adt12::C2 => {
                Str::lit("u64")
            }
            Adt12::C3 => {
                Str::lit("i64")
            }
            Adt12::C4 => {
                Str::lit("Ordering")
            }
            Adt12::C5 => {
                Str::lit("Str")
            }
            Adt12::C6 => {
                Str::lit("Bytes")
            }
            Adt12::C7 => {
                Str::lit("Overflow")
            }
            Adt12::C8 => {
                Str::lit("i128")
            }
        }
    })
}

pub fn f49(v0: Adt13) -> Str {
    let m494 = v0;
    match m494 {
        Adt13::C0 => {
            Str::lit("Option<")
        }
        Adt13::C1 => {
            Str::lit("List<")
        }
        Adt13::C2 => {
            Str::lit("std::rc::Rc<")
        }
        Adt13::C3 => {
            Str::lit("R<")
        }
        Adt13::C4 => {
            Str::lit("&")
        }
    }
}

pub fn f50(v0: Adt13) -> Str {
    let m495 = v0;
    match m495 {
        Adt13::C0 => {
            Str::lit(">")
        }
        Adt13::C1 => {
            Str::lit(">")
        }
        Adt13::C2 => {
            Str::lit(">")
        }
        Adt13::C3 => {
            Str::lit(">")
        }
        Adt13::C4 => {
            Str::lit("")
        }
    }
}

pub fn f51(v0: Adt20, v1: u64) -> R<Adt20> {
    let a496 = v1;
    let a497 = 0u64;
    if nat_eq(a496, a497) {
        f8(v0, Str::lit("T"))
    } else {
        let a498 = v1;
        let a499 = 1u64;
        if nat_eq(a498, a499) {
            f8(v0, Str::lit("U"))
        } else {
            let a500 = v1;
            let a501 = 2u64;
            if nat_eq(a500, a501) {
                f8(v0, Str::lit("V"))
            } else {
                f11(f8(v0, Str::lit("T"))?, v1)
            }
        }
    }
}

pub fn f52(v0: List<Bytes>) -> Option<List<Bytes>> {
    let m502 = v0.clone();
    match m502.uncons() {
        None => {
            None::<List<Bytes>>
        }
        Some((_, v2)) => {
            let m503 = v2;
            match m503.uncons() {
                None => {
                    None::<List<Bytes>>
                }
                Some((_, _)) => {
                    let v5: (Option<Bytes>, List<Bytes>) = f60(Fn3::F61, (None::<Bytes>, List::<Bytes>::nil()), v0);
                    let m505 = {
                        let (h504, _) = v5;
                        h504
                    };
                    match m505 {
                        None => {
                            Some(f5({
                                let (_, h507) = v5;
                                h507
                            }))
                        }
                        Some(v6) => {
                            Some(f5(List::cons(v6, {
                                let (_, h506) = v5;
                                h506
                            })))
                        }
                    }
                }
            }
        }
    }
}

pub fn f53(v0: Adt2) -> Str {
    let m508 = v0;
    match m508 {
        Adt2::C0 => {
            Str::lit("u8")
        }
        Adt2::C1 => {
            Str::lit("u16")
        }
        Adt2::C2 => {
            Str::lit("u32")
        }
        Adt2::C3 => {
            Str::lit("u64")
        }
        Adt2::C4 => {
            Str::lit("i8")
        }
        Adt2::C5 => {
            Str::lit("i16")
        }
        Adt2::C6 => {
            Str::lit("i32")
        }
        Adt2::C7 => {
            Str::lit("i64")
        }
    }
}

pub fn f54(v0: Bytes, v1: (u64, Adt20)) -> R<Option<(u64, Adt20)>> {
    f62(v0, v1)
}

pub fn f55(v0: Bytes, v1: (u64, Adt20)) -> R<Option<(u64, Adt20)>> {
    f63(v0, v1)
}

pub fn f56(v0: Adt20, v1: Bytes, v2: u64) -> Adt20 {
    let a510 = 256u64;
    let a511 = {
        let a509 = v1.clone();
        length_bytes(a509)
    };
    if nat_le(a510, a511) {
        Adt20::C0(List::cons(v1, match v0.clone() {
            Adt20::C0(h512, _, _, _, _) => {
                h512
            }
        }), Bytes::lit(&[]), v2, match v0 {
            Adt20::C0(_, _, _, h513, _) => {
                h513
            }
        }, false)
    } else {
        Adt20::C0(match v0.clone() {
            Adt20::C0(h514, _, _, _, _) => {
                h514
            }
        }, v1, v2, match v0 {
            Adt20::C0(_, _, _, h515, _) => {
                h515
            }
        }, false)
    }
}

pub fn f57(v0: Adt20) -> Adt20 {
    Adt20::C0(match v0.clone() {
        Adt20::C0(h516, _, _, _, _) => {
            h516
        }
    }, match v0.clone() {
        Adt20::C0(_, h517, _, _, _) => {
            h517
        }
    }, match v0.clone() {
        Adt20::C0(_, _, h518, _, _) => {
            h518
        }
    }, match v0 {
        Adt20::C0(_, _, _, h519, _) => {
            h519
        }
    }, true)
}

pub fn f58(v0: u64) -> R<Bytes> {
    let v1: Bytes = {
        let a556 = f64(v0, 10000000000000000000u64);
        let a557 = {
            let a554 = f64(v0, 1000000000000000000u64);
            let a555 = {
                let a552 = f64(v0, 100000000000000000u64);
                let a553 = {
                    let a550 = f64(v0, 10000000000000000u64);
                    let a551 = {
                        let a548 = f64(v0, 1000000000000000u64);
                        let a549 = {
                            let a546 = f64(v0, 100000000000000u64);
                            let a547 = {
                                let a544 = f64(v0, 10000000000000u64);
                                let a545 = {
                                    let a542 = f64(v0, 1000000000000u64);
                                    let a543 = {
                                        let a540 = f64(v0, 100000000000u64);
                                        let a541 = {
                                            let a538 = f64(v0, 10000000000u64);
                                            let a539 = {
                                                let a536 = f64(v0, 1000000000u64);
                                                let a537 = {
                                                    let a534 = f64(v0, 100000000u64);
                                                    let a535 = {
                                                        let a532 = f64(v0, 10000000u64);
                                                        let a533 = {
                                                            let a530 = f64(v0, 1000000u64);
                                                            let a531 = {
                                                                let a528 = f64(v0, 100000u64);
                                                                let a529 = {
                                                                    let a526 = f64(v0, 10000u64);
                                                                    let a527 = {
                                                                        let a524 = f64(v0, 1000u64);
                                                                        let a525 = {
                                                                            let a522 = f64(v0, 100u64);
                                                                            let a523 = {
                                                                                let a520 = f64(v0, 10u64);
                                                                                let a521 = f64(v0, 1u64);
                                                                                append_bytes(a520, a521)
                                                                            };
                                                                            append_bytes(a522, a523)
                                                                        };
                                                                        append_bytes(a524, a525)
                                                                    };
                                                                    append_bytes(a526, a527)
                                                                };
                                                                append_bytes(a528, a529)
                                                            };
                                                            append_bytes(a530, a531)
                                                        };
                                                        append_bytes(a532, a533)
                                                    };
                                                    append_bytes(a534, a535)
                                                };
                                                append_bytes(a536, a537)
                                            };
                                            append_bytes(a538, a539)
                                        };
                                        append_bytes(a540, a541)
                                    };
                                    append_bytes(a542, a543)
                                };
                                append_bytes(a544, a545)
                            };
                            append_bytes(a546, a547)
                        };
                        append_bytes(a548, a549)
                    };
                    append_bytes(a550, a551)
                };
                append_bytes(a552, a553)
            };
            append_bytes(a554, a555)
        };
        append_bytes(a556, a557)
    };
    let v2: u64 = {
        let (h558, _) = f65(Fn4::F66(v1.clone()), {
            let a559 = v1.clone();
            length_bytes(a559)
        }, 0u64)?;
        h558
    };
    let a560 = v0;
    let a561 = 0u64;
    if nat_eq(a560, a561) {
        let a562 = Str::lit("0");
        Ok(utf8_encode(a562))
    } else {
        Ok(f67(v1.clone(), v2, {
            let a564 = {
                let a563 = v1;
                length_bytes(a563)
            };
            let a565 = v2;
            nat_sub(a564, a565)
        }))
    }
}

pub fn f59(v0: u64, v1: (u64, Adt20)) -> R<Option<(u64, Adt20)>> {
    let a567 = {
        let (h566, _) = v1.clone();
        h566
    };
    let a568 = v0;
    if nat_lt(a567, a568) {
        Ok(Some(({
            let a570 = {
                let (h569, _) = v1;
                h569
            };
            let a571 = 1u64;
            nat_add(a570, a571)?
        }, f8({
            let (_, h572) = v1;
            h572
        }, Str::lit("    "))?)))
    } else {
        Ok(None::<(u64, Adt20)>)
    }
}

pub fn f60(v0: Fn3, p576: (Option<Bytes>, List<Bytes>), p577: List<Bytes>) -> (Option<Bytes>, List<Bytes>) {
    let m575 = (p576, p577);
    {
        let mut m575 = m575;
        loop {
            let step = {
                let (v1, v2) = m575;
                let m573 = v2;
                match m573.uncons() {
                    None => {
                        Err::<((Option<Bytes>, List<Bytes>), List<Bytes>), (Option<Bytes>, List<Bytes>)>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<((Option<Bytes>, List<Bytes>), List<Bytes>), (Option<Bytes>, List<Bytes>)>(({
                            let c574: Fn3 = v0.clone();
                            c574.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m575 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f61(v0: (Option<Bytes>, List<Bytes>), v1: Bytes) -> (Option<Bytes>, List<Bytes>) {
    f68(v0, v1)
}

pub fn f62(v0: Bytes, v1: (u64, Adt20)) -> R<Option<(u64, Adt20)>> {
    let a580 = {
        let (h578, _) = v1.clone();
        h578
    };
    let a581 = {
        let a579 = v0.clone();
        length_bytes(a579)
    };
    if nat_lt(a580, a581) {
        let v2: (u64, u64) = f69(v0.clone(), {
            let (h582, _) = v1.clone();
            h582
        })?;
        let v3: u64 = {
            let (h583, _) = v2;
            h583
        };
        Ok(Some(({
            let a586 = {
                let (h584, _) = v1.clone();
                h584
            };
            let a587 = {
                let (_, h585) = v2;
                h585
            };
            nat_add(a586, a587)?
        }, {
            let a588 = v3;
            let a589 = 34u64;
            if nat_eq(a588, a589) {
                f8({
                    let (_, h590) = v1;
                    h590
                }, Str::lit("\\\""))?
            } else {
                let a591 = v3;
                let a592 = 92u64;
                if nat_eq(a591, a592) {
                    f8({
                        let (_, h593) = v1;
                        h593
                    }, Str::lit("\\\\"))?
                } else {
                    let a594 = 32u64;
                    let a595 = v3;
                    if if nat_le(a594, a595) {
                        let a596 = v3;
                        let a597 = 126u64;
                        nat_le(a596, a597)
                    } else {
                        false
                    } {
                        f36({
                            let (_, h598) = v1;
                            h598
                        }, f67(v0, {
                            let (h599, _) = v1;
                            h599
                        }, 1u64))?
                    } else {
                        f8(f70(f8({
                            let (_, h600) = v1;
                            h600
                        }, Str::lit("\\u{"))?, v3)?, Str::lit("}"))?
                    }
                }
            }
        })))
    } else {
        Ok(None::<(u64, Adt20)>)
    }
}

pub fn f63(v0: Bytes, v1: (u64, Adt20)) -> R<Option<(u64, Adt20)>> {
    let m602 = f71(v0, {
        let (h601, _) = v1.clone();
        h601
    });
    match m602 {
        None => {
            Ok(None::<(u64, Adt20)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a604 = {
                    let (h603, _) = v1.clone();
                    h603
                };
                let a605 = 1u64;
                nat_add(a604, a605)?
            }, f72(f8({
                let a607 = {
                    let (h606, _) = v1;
                    h606
                };
                let a608 = 0u64;
                if nat_eq(a607, a608) {
                    let (_, h609) = v1;
                    h609
                } else {
                    f8({
                        let (_, h610) = v1;
                        h610
                    }, Str::lit(", "))?
                }
            }, Str::lit("0x"))?, v2)?)))
        }
    }
}

pub fn f64(v0: u64, v1: u64) -> Bytes {
    f67(Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39]), {
        let a614 = {
            let a611 = v0;
            let a612 = v1;
            let a613 = 0u64;
            nat_quot(a611, a612, a613)
        };
        let a615 = 10u64;
        let a616 = 0u64;
        nat_rem(a614, a615, a616)
    }, 1u64)
}

pub fn f65(v0: Fn4, p621: u64, p622: u64) -> R<(u64, bool)> {
    let m620 = (p621, p622);
    {
        let mut m620 = m620;
        loop {
            let step = {
                let (v1, v2) = m620;
                let m617 = v1;
                if m617 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m617 - 1;
                    let m619 = {
                        let c618: Fn4 = v0.clone();
                        c618.apply(v2)?
                    };
                    match m619 {
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
                Ok(next) => { m620 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f66(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f73(v0, v1)
}

pub fn f67(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m626 = {
        let a623 = v0;
        let a624 = v1;
        let a625 = v2;
        slice_bytes(a623, a624, a625)
    };
    match m626 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f68(v0: (Option<Bytes>, List<Bytes>), v1: Bytes) -> (Option<Bytes>, List<Bytes>) {
    let m628 = {
        let (h627, _) = v0;
        h627
    };
    match m628 {
        None => {
            (Some(v1), {
                let (_, h629) = v0;
                h629
            })
        }
        Some(v2) => {
            (None::<Bytes>, List::cons({
                let a630 = v2;
                let a631 = v1;
                append_bytes(a630, a631)
            }, {
                let (_, h632) = v0;
                h632
            }))
        }
    }
}

pub fn f69(v0: Bytes, v1: u64) -> R<(u64, u64)> {
    let m633 = f71(v0.clone(), v1);
    match m633 {
        None => {
            Ok((0u64, 1u64))
        }
        Some(v2) => {
            let v4: u64 = {
                let v3: u8 = v2;
                let a678 = {
                    let a654 = {
                        let a642 = {
                            let a636 = {
                                let a634 = v3;
                                let a635 = 128u8;
                                fixed_u8::bit_and(a634, a635)
                            };
                            let a637 = 128u8;
                            if equal(a636, a637) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a643 = {
                            let a640 = {
                                let a638 = v3;
                                let a639 = 64u8;
                                fixed_u8::bit_and(a638, a639)
                            };
                            let a641 = 64u8;
                            if equal(a640, a641) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a642, a643)?
                    };
                    let a655 = {
                        let a652 = {
                            let a646 = {
                                let a644 = v3;
                                let a645 = 32u8;
                                fixed_u8::bit_and(a644, a645)
                            };
                            let a647 = 32u8;
                            if equal(a646, a647) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a653 = {
                            let a650 = {
                                let a648 = v3;
                                let a649 = 16u8;
                                fixed_u8::bit_and(a648, a649)
                            };
                            let a651 = 16u8;
                            if equal(a650, a651) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a652, a653)?
                    };
                    nat_add(a654, a655)?
                };
                let a679 = {
                    let a676 = {
                        let a664 = {
                            let a658 = {
                                let a656 = v3;
                                let a657 = 8u8;
                                fixed_u8::bit_and(a656, a657)
                            };
                            let a659 = 8u8;
                            if equal(a658, a659) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a665 = {
                            let a662 = {
                                let a660 = v3;
                                let a661 = 4u8;
                                fixed_u8::bit_and(a660, a661)
                            };
                            let a663 = 4u8;
                            if equal(a662, a663) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a664, a665)?
                    };
                    let a677 = {
                        let a674 = {
                            let a668 = {
                                let a666 = v3;
                                let a667 = 2u8;
                                fixed_u8::bit_and(a666, a667)
                            };
                            let a669 = 2u8;
                            if equal(a668, a669) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a675 = {
                            let a672 = {
                                let a670 = v3;
                                let a671 = 1u8;
                                fixed_u8::bit_and(a670, a671)
                            };
                            let a673 = 1u8;
                            if equal(a672, a673) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a674, a675)?
                    };
                    nat_add(a676, a677)?
                };
                nat_add(a678, a679)?
            };
            let a680 = v4;
            let a681 = 128u64;
            if nat_lt(a680, a681) {
                Ok((v4, 1u64))
            } else {
                let a682 = v4;
                let a683 = 224u64;
                if nat_lt(a682, a683) {
                    Ok(({
                        let a691 = {
                            let a687 = {
                                let a684 = v4;
                                let a685 = 32u64;
                                let a686 = 0u64;
                                nat_rem(a684, a685, a686)
                            };
                            let a688 = 64u64;
                            nat_mul(a687, a688)?
                        };
                        let a692 = f74(v0, {
                            let a689 = v1;
                            let a690 = 1u64;
                            nat_add(a689, a690)?
                        })?;
                        nat_add(a691, a692)?
                    }, 2u64))
                } else {
                    let a693 = v4;
                    let a694 = 240u64;
                    if nat_lt(a693, a694) {
                        Ok(({
                            let a708 = {
                                let a698 = {
                                    let a695 = v4;
                                    let a696 = 16u64;
                                    let a697 = 0u64;
                                    nat_rem(a695, a696, a697)
                                };
                                let a699 = 4096u64;
                                nat_mul(a698, a699)?
                            };
                            let a709 = {
                                let a706 = {
                                    let a702 = f74(v0.clone(), {
                                        let a700 = v1;
                                        let a701 = 1u64;
                                        nat_add(a700, a701)?
                                    })?;
                                    let a703 = 64u64;
                                    nat_mul(a702, a703)?
                                };
                                let a707 = f74(v0, {
                                    let a704 = v1;
                                    let a705 = 2u64;
                                    nat_add(a704, a705)?
                                })?;
                                nat_add(a706, a707)?
                            };
                            nat_add(a708, a709)?
                        }, 3u64))
                    } else {
                        Ok(({
                            let a729 = {
                                let a713 = {
                                    let a710 = v4;
                                    let a711 = 8u64;
                                    let a712 = 0u64;
                                    nat_rem(a710, a711, a712)
                                };
                                let a714 = 262144u64;
                                nat_mul(a713, a714)?
                            };
                            let a730 = {
                                let a727 = {
                                    let a717 = f74(v0.clone(), {
                                        let a715 = v1;
                                        let a716 = 1u64;
                                        nat_add(a715, a716)?
                                    })?;
                                    let a718 = 4096u64;
                                    nat_mul(a717, a718)?
                                };
                                let a728 = {
                                    let a725 = {
                                        let a721 = f74(v0.clone(), {
                                            let a719 = v1;
                                            let a720 = 2u64;
                                            nat_add(a719, a720)?
                                        })?;
                                        let a722 = 64u64;
                                        nat_mul(a721, a722)?
                                    };
                                    let a726 = f74(v0, {
                                        let a723 = v1;
                                        let a724 = 3u64;
                                        nat_add(a723, a724)?
                                    })?;
                                    nat_add(a725, a726)?
                                };
                                nat_add(a727, a728)?
                            };
                            nat_add(a729, a730)?
                        }, 4u64))
                    }
                }
            }
        }
    }
}

pub fn f70(v0: Adt20, v1: u64) -> R<Adt20> {
    let v2: Bytes = {
        let a743 = f75(v1, 268435456u64);
        let a744 = {
            let a741 = f75(v1, 16777216u64);
            let a742 = {
                let a739 = f75(v1, 1048576u64);
                let a740 = {
                    let a737 = f75(v1, 65536u64);
                    let a738 = {
                        let a735 = f75(v1, 4096u64);
                        let a736 = {
                            let a733 = f75(v1, 256u64);
                            let a734 = {
                                let a731 = f75(v1, 16u64);
                                let a732 = f75(v1, 1u64);
                                append_bytes(a731, a732)
                            };
                            append_bytes(a733, a734)
                        };
                        append_bytes(a735, a736)
                    };
                    append_bytes(a737, a738)
                };
                append_bytes(a739, a740)
            };
            append_bytes(a741, a742)
        };
        append_bytes(a743, a744)
    };
    let v3: u64 = {
        let (h745, _) = f65(Fn4::F76(v2.clone()), 7u64, 0u64)?;
        h745
    };
    f36(v0, f67(v2, v3, {
        let a746 = 8u64;
        let a747 = v3;
        nat_sub(a746, a747)
    }))
}

pub fn f71(v0: Bytes, v1: u64) -> Option<u8> {
    let a748 = v0;
    let a749 = v1;
    index_bytes(a748, a749)
}

pub fn f72(v0: Adt20, v1: u8) -> R<Adt20> {
    let v3: u64 = {
        let v2: u8 = v1;
        let a794 = {
            let a770 = {
                let a758 = {
                    let a752 = {
                        let a750 = v2;
                        let a751 = 128u8;
                        fixed_u8::bit_and(a750, a751)
                    };
                    let a753 = 128u8;
                    if equal(a752, a753) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a759 = {
                    let a756 = {
                        let a754 = v2;
                        let a755 = 64u8;
                        fixed_u8::bit_and(a754, a755)
                    };
                    let a757 = 64u8;
                    if equal(a756, a757) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a758, a759)?
            };
            let a771 = {
                let a768 = {
                    let a762 = {
                        let a760 = v2;
                        let a761 = 32u8;
                        fixed_u8::bit_and(a760, a761)
                    };
                    let a763 = 32u8;
                    if equal(a762, a763) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a769 = {
                    let a766 = {
                        let a764 = v2;
                        let a765 = 16u8;
                        fixed_u8::bit_and(a764, a765)
                    };
                    let a767 = 16u8;
                    if equal(a766, a767) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a768, a769)?
            };
            nat_add(a770, a771)?
        };
        let a795 = {
            let a792 = {
                let a780 = {
                    let a774 = {
                        let a772 = v2;
                        let a773 = 8u8;
                        fixed_u8::bit_and(a772, a773)
                    };
                    let a775 = 8u8;
                    if equal(a774, a775) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a781 = {
                    let a778 = {
                        let a776 = v2;
                        let a777 = 4u8;
                        fixed_u8::bit_and(a776, a777)
                    };
                    let a779 = 4u8;
                    if equal(a778, a779) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a780, a781)?
            };
            let a793 = {
                let a790 = {
                    let a784 = {
                        let a782 = v2;
                        let a783 = 2u8;
                        fixed_u8::bit_and(a782, a783)
                    };
                    let a785 = 2u8;
                    if equal(a784, a785) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a791 = {
                    let a788 = {
                        let a786 = v2;
                        let a787 = 1u8;
                        fixed_u8::bit_and(a786, a787)
                    };
                    let a789 = 1u8;
                    if equal(a788, a789) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a790, a791)?
            };
            nat_add(a792, a793)?
        };
        nat_add(a794, a795)?
    };
    f36(v0, {
        let a796 = f75(v3, 16u64);
        let a797 = f75(v3, 1u64);
        append_bytes(a796, a797)
    })
}

pub fn f73(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m798 = f71(v0, v1);
    match m798 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            let a799 = v2;
            let a800 = 48u8;
            if equal(a799, a800) {
                Ok(Some({
                    let a801 = v1;
                    let a802 = 1u64;
                    nat_add(a801, a802)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f74(v0: Bytes, v1: u64) -> R<u64> {
    let m803 = f71(v0, v1);
    match m803 {
        None => {
            Ok(0u64)
        }
        Some(v2) => {
            let a850 = {
                let v3: u8 = v2;
                let a848 = {
                    let a824 = {
                        let a812 = {
                            let a806 = {
                                let a804 = v3;
                                let a805 = 128u8;
                                fixed_u8::bit_and(a804, a805)
                            };
                            let a807 = 128u8;
                            if equal(a806, a807) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a813 = {
                            let a810 = {
                                let a808 = v3;
                                let a809 = 64u8;
                                fixed_u8::bit_and(a808, a809)
                            };
                            let a811 = 64u8;
                            if equal(a810, a811) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a812, a813)?
                    };
                    let a825 = {
                        let a822 = {
                            let a816 = {
                                let a814 = v3;
                                let a815 = 32u8;
                                fixed_u8::bit_and(a814, a815)
                            };
                            let a817 = 32u8;
                            if equal(a816, a817) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a823 = {
                            let a820 = {
                                let a818 = v3;
                                let a819 = 16u8;
                                fixed_u8::bit_and(a818, a819)
                            };
                            let a821 = 16u8;
                            if equal(a820, a821) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a822, a823)?
                    };
                    nat_add(a824, a825)?
                };
                let a849 = {
                    let a846 = {
                        let a834 = {
                            let a828 = {
                                let a826 = v3;
                                let a827 = 8u8;
                                fixed_u8::bit_and(a826, a827)
                            };
                            let a829 = 8u8;
                            if equal(a828, a829) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a835 = {
                            let a832 = {
                                let a830 = v3;
                                let a831 = 4u8;
                                fixed_u8::bit_and(a830, a831)
                            };
                            let a833 = 4u8;
                            if equal(a832, a833) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a834, a835)?
                    };
                    let a847 = {
                        let a844 = {
                            let a838 = {
                                let a836 = v3;
                                let a837 = 2u8;
                                fixed_u8::bit_and(a836, a837)
                            };
                            let a839 = 2u8;
                            if equal(a838, a839) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a845 = {
                            let a842 = {
                                let a840 = v3;
                                let a841 = 1u8;
                                fixed_u8::bit_and(a840, a841)
                            };
                            let a843 = 1u8;
                            if equal(a842, a843) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a844, a845)?
                    };
                    nat_add(a846, a847)?
                };
                nat_add(a848, a849)?
            };
            let a851 = 64u64;
            let a852 = 0u64;
            Ok(nat_rem(a850, a851, a852))
        }
    }
}

pub fn f75(v0: u64, v1: u64) -> Bytes {
    f67(Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66]), {
        let a856 = {
            let a853 = v0;
            let a854 = v1;
            let a855 = 0u64;
            nat_quot(a853, a854, a855)
        };
        let a857 = 16u64;
        let a858 = 0u64;
        nat_rem(a856, a857, a858)
    }, 1u64)
}

pub fn f76(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f77(v0, v1)
}

pub fn f77(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m859 = f71(v0, v1);
    match m859 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            let a860 = v2;
            let a861 = 48u8;
            if equal(a860, a861) {
                Ok(Some({
                    let a862 = v1;
                    let a863 = 1u64;
                    nat_add(a862, a863)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn run(p0: u64, p1: Adt0) -> R<Result<Bytes, Bytes>> {
    f0(p0, p1)
}
