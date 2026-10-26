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
    C0(bool, Adt1),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(Adt2, Adt8, Adt9),
    C1(u64, Adt2, Adt2, bool, Adt10, Adt9),
    C2(Adt12, Adt13, Adt2, Adt17, Adt9),
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
}

#[derive(Clone)]
pub enum Adt15 {
    C0,
    C1,
    C2,
    C3,
}

#[derive(Clone)]
pub enum Adt16 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt17 {
    C0(Adt18, Adt9),
    C1(Adt19, Adt12, Adt9),
    C2(Adt20, std::rc::Rc<Adt17>, Adt9),
    C3(Adt21, std::rc::Rc<Adt17>, bool, Adt9),
    C4(Adt12, std::rc::Rc<Adt17>, bool, Adt9),
    C5(Adt23, std::rc::Rc<Adt17>, Adt9),
    C6(std::rc::Rc<Adt17>, std::rc::Rc<Adt17>, Adt9),
    C7(std::rc::Rc<Adt17>, std::rc::Rc<Adt17>, std::rc::Rc<Adt17>, Adt9),
    C8(std::rc::Rc<Adt17>, std::rc::Rc<Adt17>, Adt9),
    C9(std::rc::Rc<Adt17>),
    C10(Adt24, Adt12, Adt9),
    C11(std::rc::Rc<Adt17>, std::rc::Rc<Adt17>),
    C12,
    C13(std::rc::Rc<Adt17>, std::rc::Rc<Adt17>),
    C14(Adt14, bool, Adt2, std::rc::Rc<Adt17>, Adt9, std::rc::Rc<Adt17>),
    C15(Adt14, std::rc::Rc<Adt17>, std::rc::Rc<Adt17>),
    C16(Adt12, std::rc::Rc<Adt17>),
}

#[derive(Clone)]
pub enum Adt18 {
    C0,
    C1,
    C2,
    C3,
    C4(Adt4),
    C5,
    C6,
    C7,
    C8,
}

#[derive(Clone)]
pub enum Adt19 {
    C0,
    C1,
    C2,
    C3,
    C4,
}

#[derive(Clone)]
pub enum Adt20 {
    C0,
    C1,
    C2,
    C3,
}

#[derive(Clone)]
pub enum Adt21 {
    C0(u64),
    C1(Adt22),
    C2(u64),
}

#[derive(Clone)]
pub enum Adt22 {
    C0(u64, Option<Adt4>),
}

#[derive(Clone)]
pub enum Adt23 {
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
pub enum Adt24 {
    C0,
    C1,
    C2,
    C3,
}

#[derive(Clone)]
pub enum Adt25 {
    C0(u64, Bytes, u64, bool, bool),
}

#[derive(Clone)]
pub enum Adt26 {
    C0(List<(u64, List<(u64, u64)>)>, List<(u64, List<(u64, u64)>)>, List<(u64, bool)>, List<(u64, bool)>),
}

#[derive(Clone)]
pub enum Adt27 {
    C0(bool, Adt26, bool, List<Adt25>, bool),
}

#[derive(Clone)]
pub enum Adt28 {
    C0(List<Adt12>, List<Adt12>, List<Adt12>, List<(Adt12, List<u64>)>, bool),
}

#[derive(Clone)]
pub enum Adt29 {
    C0(Bytes, Bytes, Option<Adt4>, Option<Adt4>),
}

#[derive(Clone)]
pub enum Adt30 {
    C0(List<Adt29>, List<Bytes>, List<Adt25>),
}

#[derive(Clone)]
pub enum Fn0 {
    F35(u64),
}

impl Fn0 {
    pub fn apply(&self, p0: Option<List<(u64, u64)>>, p1: (u64, List<(u64, u64)>)) -> Option<List<(u64, u64)>> {
        match self {
            Fn0::F35(k0) => f35(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F37(u64),
}

impl Fn1 {
    pub fn apply(&self, p0: Option<bool>, p1: (u64, bool)) -> Option<bool> {
        match self {
            Fn1::F37(k0) => f37(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F39(Bytes),
    F184(Bytes, u64, Bytes, u64),
}

impl Fn2 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn2::F39(k0) => f39(k0.clone(), p0),
            Fn2::F184(k0, k1, k2, k3) => f184(k0.clone(), *k1, k2.clone(), *k3, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F56,
}

impl Fn3 {
    pub fn apply(&self, p0: bool, p1: Adt11) -> bool {
        match self {
            Fn3::F56 => f56(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F92(Adt12),
}

impl Fn4 {
    pub fn apply(&self, p0: bool, p1: Adt12) -> bool {
        match self {
            Fn4::F92(k0) => f92(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F113,
    F117(Adt12),
}

impl Fn5 {
    pub fn apply(&self, p0: List<Adt12>, p1: Adt12) -> List<Adt12> {
        match self {
            Fn5::F113 => f113(p0, p1),
            Fn5::F117(k0) => f117(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F115,
    F165(Adt12),
}

impl Fn6 {
    pub fn apply(&self, p0: List<(Adt12, List<u64>)>, p1: (Adt12, List<u64>)) -> List<(Adt12, List<u64>)> {
        match self {
            Fn6::F115 => f115(p0, p1),
            Fn6::F165(k0) => f165(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F119(List<Adt12>, List<Adt12>, Adt12),
}

impl Fn7 {
    pub fn apply(&self, p0: Option<Adt12>, p1: Adt12) -> Option<Adt12> {
        match self {
            Fn7::F119(k0, k1, k2) => f119(k0.clone(), k1.clone(), k2.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F121(List<(Adt12, List<u64>)>, List<Adt12>),
}

impl Fn8 {
    pub fn apply(&self, p0: Option<Adt12>, p1: (Adt12, List<u64>)) -> Option<Adt12> {
        match self {
            Fn8::F121(k0, k1) => f121(k0.clone(), k1.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F123(Adt12),
}

impl Fn9 {
    pub fn apply(&self, p0: Option<u64>, p1: (Adt12, u64)) -> Option<u64> {
        match self {
            Fn9::F123(k0) => f123(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F127(u64),
}

impl Fn10 {
    pub fn apply(&self, p0: Option<u64>, p1: (u64, u64)) -> Option<u64> {
        match self {
            Fn10::F127(k0) => f127(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F130(Adt12),
}

impl Fn11 {
    pub fn apply(&self, p0: Option<List<u64>>, p1: (Adt12, List<u64>)) -> Option<List<u64>> {
        match self {
            Fn11::F130(k0) => f130(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F134,
}

impl Fn12 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Adt29) -> Option<Bytes> {
        match self {
            Fn12::F134 => f134(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F154(u64),
}

impl Fn13 {
    pub fn apply(&self, p0: Option<Adt25>, p1: Adt25) -> Option<Adt25> {
        match self {
            Fn13::F154(k0) => f154(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F164(u64),
    F189(List<u64>),
}

impl Fn14 {
    pub fn apply(&self, p0: bool, p1: u64) -> bool {
        match self {
            Fn14::F164(k0) => f164(*k0, p0, p1),
            Fn14::F189(k0) => f189(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F178,
}

impl Fn15 {
    pub fn apply(&self, p0: List<u64>, p1: u64) -> List<u64> {
        match self {
            Fn15::F178 => f178(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F183(Adt29),
}

impl Fn16 {
    pub fn apply(&self, p0: bool, p1: Adt29) -> bool {
        match self {
            Fn16::F183(k0) => f183(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F193(Bytes),
}

impl Fn17 {
    pub fn apply(&self, p0: (bool, List<Bytes>), p1: Bytes) -> (bool, List<Bytes>) {
        match self {
            Fn17::F193(k0) => f193(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F197(Bytes),
}

impl Fn18 {
    pub fn apply(&self, p0: bool, p1: Bytes) -> bool {
        match self {
            Fn18::F197(k0) => f197(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F200,
}

impl Fn19 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn19::F200 => f200(p0, p1),
        }
    }
}

pub fn f0(v0: Adt0) -> R<Result<(), Bytes>> {
    let v1: List<Adt25> = List::cons(Adt25::C0(0u64, f1(Str::lit("nat_add")), 0u64, true, false), List::cons(Adt25::C0(1u64, f1(Str::lit("nat_sub")), 0u64, false, false), List::cons(Adt25::C0(2u64, f1(Str::lit("nat_mul")), 0u64, true, false), List::cons(Adt25::C0(3u64, f1(Str::lit("nat_quot")), 0u64, false, false), List::cons(Adt25::C0(4u64, f1(Str::lit("nat_rem")), 0u64, false, false), List::cons(Adt25::C0(5u64, f1(Str::lit("nat_eq")), 0u64, false, false), List::cons(Adt25::C0(6u64, f1(Str::lit("nat_le")), 0u64, false, false), List::cons(Adt25::C0(7u64, f1(Str::lit("nat_lt")), 0u64, false, false), List::cons(Adt25::C0(8u64, f1(Str::lit("nat_succ")), 0u64, true, false), List::cons(Adt25::C0(9u64, f1(Str::lit("int_add")), 0u64, true, false), List::<Adt25>::nil()))))))))));
    let v2: List<Adt25> = List::cons(Adt25::C0(10u64, f1(Str::lit("int_sub")), 0u64, true, false), List::cons(Adt25::C0(11u64, f1(Str::lit("int_mul")), 0u64, true, false), List::cons(Adt25::C0(12u64, f1(Str::lit("int_neg")), 0u64, true, false), List::cons(Adt25::C0(13u64, f1(Str::lit("int_quot")), 0u64, true, false), List::cons(Adt25::C0(14u64, f1(Str::lit("int_rem")), 0u64, false, false), List::cons(Adt25::C0(15u64, f1(Str::lit("bool_not")), 0u64, false, false), List::cons(Adt25::C0(16u64, f1(Str::lit("bool_and")), 0u64, false, false), List::cons(Adt25::C0(17u64, f1(Str::lit("bool_or")), 0u64, false, false), List::cons(Adt25::C0(18u64, f1(Str::lit("equal")), 0u64, false, false), List::cons(Adt25::C0(19u64, f1(Str::lit("compare")), 0u64, false, false), v1))))))))));
    let v3: List<Adt25> = List::cons(Adt25::C0(20u64, f1(Str::lit("checked_add")), 1u64, false, false), List::cons(Adt25::C0(21u64, f1(Str::lit("checked_sub")), 1u64, false, false), List::cons(Adt25::C0(22u64, f1(Str::lit("checked_mul")), 1u64, false, false), List::cons(Adt25::C0(23u64, f1(Str::lit("checked_quot")), 1u64, false, false), List::cons(Adt25::C0(24u64, f1(Str::lit("checked_neg_")), 2u64, false, false), List::cons(Adt25::C0(25u64, f1(Str::lit("bit_and")), 1u64, false, false), List::cons(Adt25::C0(26u64, f1(Str::lit("bit_or")), 1u64, false, false), List::cons(Adt25::C0(27u64, f1(Str::lit("bit_xor")), 1u64, false, false), List::cons(Adt25::C0(28u64, f1(Str::lit("bit_not")), 1u64, false, false), List::cons(Adt25::C0(29u64, f1(Str::lit("shift_left")), 1u64, false, false), v2))))))))));
    let v4: List<Adt25> = List::cons(Adt25::C0(30u64, f1(Str::lit("shift_right")), 1u64, false, false), List::cons(Adt25::C0(31u64, f1(Str::lit("convert")), 1u64, false, false), List::cons(Adt25::C0(32u64, f1(Str::lit("append_list")), 0u64, false, true), List::cons(Adt25::C0(33u64, f1(Str::lit("append_bytes")), 0u64, false, true), List::cons(Adt25::C0(34u64, f1(Str::lit("length_list")), 0u64, false, true), List::cons(Adt25::C0(35u64, f1(Str::lit("length_bytes")), 0u64, false, true), List::cons(Adt25::C0(36u64, f1(Str::lit("length_string")), 0u64, false, true), List::cons(Adt25::C0(37u64, f1(Str::lit("index_list")), 0u64, false, true), List::cons(Adt25::C0(38u64, f1(Str::lit("index_bytes")), 0u64, false, true), List::cons(Adt25::C0(39u64, f1(Str::lit("slice_list")), 0u64, false, true), v3))))))))));
    let v5: List<Adt25> = List::cons(Adt25::C0(40u64, f1(Str::lit("slice_bytes")), 0u64, false, true), List::cons(Adt25::C0(41u64, f1(Str::lit("utf8_encode")), 0u64, false, true), List::cons(Adt25::C0(42u64, f1(Str::lit("utf8_decode")), 0u64, false, true), List::cons(Adt25::C0(43u64, f1(Str::lit("compare_bytes")), 0u64, false, true), List::cons(Adt25::C0(44u64, f1(Str::lit("split_exact")), 0u64, false, true), List::cons(Adt25::C0(45u64, f1(Str::lit("join")), 0u64, false, true), List::cons(Adt25::C0(46u64, f1(Str::lit("format_i64")), 0u64, false, true), List::cons(Adt25::C0(47u64, f1(Str::lit("format_")), 2u64, false, true), List::cons(Adt25::C0(48u64, f1(Str::lit("parse_int")), 0u64, true, true), List::cons(Adt25::C0(49u64, f1(Str::lit("parse_")), 2u64, false, true), v4))))))))));
    let m2 = f2(Adt26::C0(List::<(u64, List<(u64, u64)>)>::nil(), List::<(u64, List<(u64, u64)>)>::nil(), List::<(u64, bool)>::nil(), List::<(u64, bool)>::nil()), match v0.clone() {
        Adt0::C0(_, h1) => {
            h1
        }
    })?;
    match m2 {
        Ok(v6) => {
            f3(match v0.clone() {
                Adt0::C0(h3, _) => {
                    h3
                }
            }, v6, v5, match v0 {
                Adt0::C0(_, h4) => {
                    h4
                }
            })
        }
        Err(v7) => {
            Ok(Err::<(), Bytes>(v7))
        }
    }
}

pub fn f1(v0: Str) -> Bytes {
    let a5 = v0;
    utf8_encode(a5)
}

pub fn f2(p38: Adt26, p39: Adt1) -> R<Result<Adt26, Bytes>> {
    let m37 = (p38, p39);
    {
        let mut m37 = m37;
        loop {
            let step = {
                let (v0, v1) = m37;
                let m6 = v1;
                match m6 {
                    Adt1::C0(_, _, _) => {
                        Err::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>(Ok(Ok::<Adt26, Bytes>(v0)))
                    }
                    Adt1::C1(_, _, _, _, _, _) => {
                        Err::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>(Ok(Ok::<Adt26, Bytes>(v0)))
                    }
                    Adt1::C2(_, _, _, _, _) => {
                        Err::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>(Ok(Ok::<Adt26, Bytes>(v0)))
                    }
                    Adt1::C3 => {
                        Err::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>(Ok(Ok::<Adt26, Bytes>(v0)))
                    }
                    Adt1::C4(r35, r36) => {
                        let v16 = (*r35).clone();
                        let v17 = (*r36).clone();
                        let m7 = v16;
                        match m7 {
                            Adt1::C0(v18, v19, _) => {
                                let m8 = v18.clone();
                                match m8 {
                                    Adt2::C0(_) => {
                                        Err::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>(Ok(Err::<Adt26, Bytes>(f4(f1(Str::lit("an enum named ")), f4(f5(v18)?, f1(Str::lit(" is not a declared type")))))))
                                    }
                                    Adt2::C1(_) => {
                                        Err::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>(Ok(Err::<Adt26, Bytes>(f4(f1(Str::lit("an enum named ")), f4(f5(v18)?, f1(Str::lit(" is not a declared type")))))))
                                    }
                                    Adt2::C2(_, _) => {
                                        Err::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>(Ok(Err::<Adt26, Bytes>(f4(f1(Str::lit("an enum named ")), f4(f5(v18)?, f1(Str::lit(" is not a declared type")))))))
                                    }
                                    Adt2::C3(_, _, _) => {
                                        Err::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>(Ok(Err::<Adt26, Bytes>(f4(f1(Str::lit("an enum named ")), f4(f5(v18)?, f1(Str::lit(" is not a declared type")))))))
                                    }
                                    Adt2::C4(v28, v29) => {
                                        let m9 = f6(v28);
                                        if m9 {
                                            let m11 = f7(match v0.clone() {
                                                Adt26::C0(_, h10, _, _) => {
                                                    h10
                                                }
                                            }, v29);
                                            if m11 {
                                                Err::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>(Ok(Err::<Adt26, Bytes>(f4(f1(Str::lit("hygiene: ")), f4(f5(v18)?, f1(Str::lit(" is declared twice")))))))
                                            } else {
                                                Ok::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>((Adt26::C0(match v0.clone() {
                                                    Adt26::C0(h12, _, _, _) => {
                                                        h12
                                                    }
                                                }, List::cons((v29, f8(List::<(u64, u64)>::nil(), v19)?), match v0.clone() {
                                                    Adt26::C0(_, h13, _, _) => {
                                                        h13
                                                    }
                                                }), match v0.clone() {
                                                    Adt26::C0(_, _, h14, _) => {
                                                        h14
                                                    }
                                                }, match v0 {
                                                    Adt26::C0(_, _, _, h15) => {
                                                        h15
                                                    }
                                                }), v17))
                                            }
                                        } else {
                                            let m17 = f7(match v0.clone() {
                                                Adt26::C0(h16, _, _, _) => {
                                                    h16
                                                }
                                            }, v29);
                                            if m17 {
                                                Err::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>(Ok(Err::<Adt26, Bytes>(f4(f1(Str::lit("hygiene: ")), f4(f5(v18)?, f1(Str::lit(" is declared twice")))))))
                                            } else {
                                                Ok::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>((Adt26::C0(List::cons((v29, f8(List::<(u64, u64)>::nil(), v19)?), match v0.clone() {
                                                    Adt26::C0(h18, _, _, _) => {
                                                        h18
                                                    }
                                                }), match v0.clone() {
                                                    Adt26::C0(_, h19, _, _) => {
                                                        h19
                                                    }
                                                }, match v0.clone() {
                                                    Adt26::C0(_, _, h20, _) => {
                                                        h20
                                                    }
                                                }, match v0 {
                                                    Adt26::C0(_, _, _, h21) => {
                                                        h21
                                                    }
                                                }), v17))
                                            }
                                        }
                                    }
                                    Adt2::C5 => {
                                        Err::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>(Ok(Err::<Adt26, Bytes>(f4(f1(Str::lit("an enum named ")), f4(f5(v18)?, f1(Str::lit(" is not a declared type")))))))
                                    }
                                    Adt2::C6(_, _) => {
                                        Err::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>(Ok(Err::<Adt26, Bytes>(f4(f1(Str::lit("an enum named ")), f4(f5(v18)?, f1(Str::lit(" is not a declared type")))))))
                                    }
                                }
                            }
                            Adt1::C1(v32, _, _, v35, _, _) => {
                                let m23 = f9(match v0.clone() {
                                    Adt26::C0(_, _, h22, _) => {
                                        h22
                                    }
                                }, v32);
                                if m23 {
                                    Err::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>(Ok(Err::<Adt26, Bytes>(f4(f1(Str::lit("hygiene: Fn")), f4(f10(v32)?, f1(Str::lit("::apply is declared twice")))))))
                                } else {
                                    Ok::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>((Adt26::C0(match v0.clone() {
                                        Adt26::C0(h24, _, _, _) => {
                                            h24
                                        }
                                    }, match v0.clone() {
                                        Adt26::C0(_, h25, _, _) => {
                                            h25
                                        }
                                    }, List::cons((v32, v35), match v0.clone() {
                                        Adt26::C0(_, _, h26, _) => {
                                            h26
                                        }
                                    }), match v0 {
                                        Adt26::C0(_, _, _, h27) => {
                                            h27
                                        }
                                    }), v17))
                                }
                            }
                            Adt1::C2(v38, _, v40, _, _) => {
                                let m28 = v38;
                                match m28 {
                                    Adt12::C0(_) => {
                                        Ok::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>((v0, v17))
                                    }
                                    Adt12::C1(_) => {
                                        Ok::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>((v0, v17))
                                    }
                                    Adt12::C2(_) => {
                                        Ok::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>((v0, v17))
                                    }
                                    Adt12::C3(_) => {
                                        Ok::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>((v0, v17))
                                    }
                                    Adt12::C4(_) => {
                                        Ok::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>((v0, v17))
                                    }
                                    Adt12::C5(_) => {
                                        Ok::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>((v0, v17))
                                    }
                                    Adt12::C6(_) => {
                                        Ok::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>((v0, v17))
                                    }
                                    Adt12::C7(_) => {
                                        Ok::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>((v0, v17))
                                    }
                                    Adt12::C8(v51) => {
                                        let m30 = f9(match v0.clone() {
                                            Adt26::C0(_, _, _, h29) => {
                                                h29
                                            }
                                        }, v51);
                                        if m30 {
                                            Err::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>(Ok(Err::<Adt26, Bytes>(f4(f1(Str::lit("hygiene: f")), f4(f10(v51)?, f1(Str::lit(" is declared twice")))))))
                                        } else {
                                            Ok::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>((Adt26::C0(match v0.clone() {
                                                Adt26::C0(h31, _, _, _) => {
                                                    h31
                                                }
                                            }, match v0.clone() {
                                                Adt26::C0(_, h32, _, _) => {
                                                    h32
                                                }
                                            }, match v0.clone() {
                                                Adt26::C0(_, _, h33, _) => {
                                                    h33
                                                }
                                            }, List::cons((v51, f11(v40)), match v0 {
                                                Adt26::C0(_, _, _, h34) => {
                                                    h34
                                                }
                                            })), v17))
                                        }
                                    }
                                    Adt12::C9(_) => {
                                        Ok::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>((v0, v17))
                                    }
                                }
                            }
                            Adt1::C3 => {
                                Ok::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>((v0, v17))
                            }
                            Adt1::C4(_, _) => {
                                Ok::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>((v0, v17))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m37 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f3(v0: bool, v1: Adt26, v2: List<Adt25>, p45: Adt1) -> R<Result<(), Bytes>> {
    let m44 = p45;
    {
        let mut m44 = m44;
        loop {
            let step = {
                let v3 = m44;
                let m40 = v3;
                match m40 {
                    Adt1::C0(_, _, _) => {
                        Err::<Adt1, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt1::C1(_, _, _, _, _, _) => {
                        Err::<Adt1, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt1::C2(_, _, _, _, _) => {
                        Err::<Adt1, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt1::C3 => {
                        Err::<Adt1, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt1::C4(r42, r43) => {
                        let v18 = (*r42).clone();
                        let v19 = (*r43).clone();
                        let m41 = f12(v0, v1.clone(), v2.clone(), v18)?;
                        match m41 {
                            Ok(_) => {
                                Ok::<Adt1, R<Result<(), Bytes>>>(v19)
                            }
                            Err(v21) => {
                                Err::<Adt1, R<Result<(), Bytes>>>(Ok(Err::<(), Bytes>(v21)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m44 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f4(v0: Bytes, v1: Bytes) -> Bytes {
    let a46 = v0;
    let a47 = v1;
    append_bytes(a46, a47)
}

pub fn f5(v0: Adt2) -> R<Bytes> {
    let m48 = v0;
    match m48 {
        Adt2::C0(v1) => {
            Ok(f13(v1))
        }
        Adt2::C1(v2) => {
            Ok(f4(f1(Str::lit("Fixed(")), f4(f14(v2), f1(Str::lit(")")))))
        }
        Adt2::C2(v3, r49) => {
            let v4 = (*r49).clone();
            Ok(f4(f15(v3), f4(f1(Str::lit("(")), f4(f5(v4)?, f1(Str::lit(")"))))))
        }
        Adt2::C3(v5, r50, r51) => {
            let v6 = (*r50).clone();
            let v7 = (*r51).clone();
            Ok(f4(f16(v5), f4(f1(Str::lit("(")), f4(f5(v6)?, f4(f1(Str::lit(", ")), f4(f5(v7)?, f1(Str::lit(")"))))))))
        }
        Adt2::C4(v8, v9) => {
            Ok(f4(f17(v8), f4(f1(Str::lit("(")), f4(f10(v9)?, f1(Str::lit(")"))))))
        }
        Adt2::C5 => {
            Ok(f1(Str::lit("unreachable")))
        }
        Adt2::C6(_, _) => {
            Ok(f1(Str::lit("unreachable")))
        }
    }
}

pub fn f6(v0: Adt7) -> bool {
    let m52 = v0;
    match m52 {
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

pub fn f7(v0: List<(u64, List<(u64, u64)>)>, v1: u64) -> bool {
    let m53 = f18(v0, v1);
    match m53 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f8(p57: List<(u64, u64)>, p58: Adt8) -> R<List<(u64, u64)>> {
    let m56 = (p57, p58);
    {
        let mut m56 = m56;
        loop {
            let step = {
                let (v0, v1) = m56;
                let m54 = v1;
                match m54 {
                    Adt8::C0 => {
                        Err::<(List<(u64, u64)>, Adt8), R<List<(u64, u64)>>>(Ok(v0))
                    }
                    Adt8::C1(v2, v3, r55) => {
                        let v4 = (*r55).clone();
                        Ok::<(List<(u64, u64)>, Adt8), R<List<(u64, u64)>>>((List::cons((v2, f19(v3)?), v0), v4))
                    }
                }
            };
            match step {
                Ok(next) => { m56 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f9(v0: List<(u64, bool)>, v1: u64) -> bool {
    let m59 = f20(v0, v1);
    match m59 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f10(v0: u64) -> R<Bytes> {
    let v1: Bytes = f1(Str::lit("0"));
    let v2: Bytes = f21({
        let a96 = f22(v0, 10000000000000000000u64);
        let a97 = {
            let a94 = f22(v0, 1000000000000000000u64);
            let a95 = {
                let a92 = f22(v0, 100000000000000000u64);
                let a93 = {
                    let a90 = f22(v0, 10000000000000000u64);
                    let a91 = {
                        let a88 = f22(v0, 1000000000000000u64);
                        let a89 = {
                            let a86 = f22(v0, 100000000000000u64);
                            let a87 = {
                                let a84 = f22(v0, 10000000000000u64);
                                let a85 = {
                                    let a82 = f22(v0, 1000000000000u64);
                                    let a83 = {
                                        let a80 = f22(v0, 100000000000u64);
                                        let a81 = {
                                            let a78 = f22(v0, 10000000000u64);
                                            let a79 = {
                                                let a76 = f22(v0, 1000000000u64);
                                                let a77 = {
                                                    let a74 = f22(v0, 100000000u64);
                                                    let a75 = {
                                                        let a72 = f22(v0, 10000000u64);
                                                        let a73 = {
                                                            let a70 = f22(v0, 1000000u64);
                                                            let a71 = {
                                                                let a68 = f22(v0, 100000u64);
                                                                let a69 = {
                                                                    let a66 = f22(v0, 10000u64);
                                                                    let a67 = {
                                                                        let a64 = f22(v0, 1000u64);
                                                                        let a65 = {
                                                                            let a62 = f22(v0, 100u64);
                                                                            let a63 = {
                                                                                let a60 = f22(v0, 10u64);
                                                                                let a61 = f22(v0, 1u64);
                                                                                append_bytes(a60, a61)
                                                                            };
                                                                            append_bytes(a62, a63)
                                                                        };
                                                                        append_bytes(a64, a65)
                                                                    };
                                                                    append_bytes(a66, a67)
                                                                };
                                                                append_bytes(a68, a69)
                                                            };
                                                            append_bytes(a70, a71)
                                                        };
                                                        append_bytes(a72, a73)
                                                    };
                                                    append_bytes(a74, a75)
                                                };
                                                append_bytes(a76, a77)
                                            };
                                            append_bytes(a78, a79)
                                        };
                                        append_bytes(a80, a81)
                                    };
                                    append_bytes(a82, a83)
                                };
                                append_bytes(a84, a85)
                            };
                            append_bytes(a86, a87)
                        };
                        append_bytes(a88, a89)
                    };
                    append_bytes(a90, a91)
                };
                append_bytes(a92, a93)
            };
            append_bytes(a94, a95)
        };
        append_bytes(a96, a97)
    })?;
    let a98 = v0;
    let a99 = 0u64;
    if nat_eq(a98, a99) {
        Ok(v1)
    } else {
        Ok(v2)
    }
}

pub fn f11(v0: Adt2) -> bool {
    let m100 = v0;
    match m100 {
        Adt2::C0(_) => {
            false
        }
        Adt2::C1(_) => {
            false
        }
        Adt2::C2(v3, _) => {
            f23(v3)
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

pub fn f12(v0: bool, v1: Adt26, v2: List<Adt25>, v3: Adt1) -> R<Result<(), Bytes>> {
    let m101 = v3;
    match m101 {
        Adt1::C0(_, v5, _) => {
            f24(Adt27::C0(v0, v1, false, v2, false), v5)
        }
        Adt1::C1(v7, v8, v9, v10, v11, _) => {
            let v13: Adt27 = Adt27::C0(v0, v1, false, v2, false);
            let m102 = f25(v13.clone(), v8)?;
            match m102 {
                Ok(_) => {
                    let m103 = f26(v13.clone(), v9)?;
                    match m103 {
                        Ok(_) => {
                            f27(v13, v7, v10, v11)
                        }
                        Err(v16) => {
                            Ok(Err::<(), Bytes>(v16))
                        }
                    }
                }
                Err(v17) => {
                    Ok(Err::<(), Bytes>(v17))
                }
            }
        }
        Adt1::C2(v18, v19, v20, v21, _) => {
            let v23: bool = f11(v20.clone());
            let v24: Adt27 = Adt27::C0(v0, v1.clone(), v23, v2, false);
            let m104 = f26(v24.clone(), v20)?;
            match m104 {
                Ok(_) => {
                    let m105 = f28(v24.clone(), Adt28::C0(List::<Adt12>::nil(), List::<Adt12>::nil(), List::<Adt12>::nil(), List::<(Adt12, List<u64>)>::nil(), false), v19.clone())?;
                    match m105 {
                        Ok(v26) => {
                            let m106 = f29(v26, v24, v21.clone(), true)?;
                            match m106 {
                                Ok(v27) => {
                                    let m109 = if match v27 {
                                        Adt28::C0(_, _, _, _, h107) => {
                                            h107
                                        }
                                    } {
                                        let a108 = v23;
                                        bool_not(a108)
                                    } else {
                                        false
                                    };
                                    if m109 {
                                        Ok(Err::<(), Bytes>(f4(f1(Str::lit("arithmetic: ")), f4(f30(f31(v18)?), f1(Str::lit(" propagates a failure its result type does not carry"))))))
                                    } else {
                                        let m110 = f32(f33(List::<(Adt12, u64)>::nil(), v19), v1, v21, v23)?;
                                        match m110 {
                                            Ok(_) => {
                                                Ok(Ok::<(), Bytes>(()))
                                            }
                                            Err(v29) => {
                                                Ok(Err::<(), Bytes>(v29))
                                            }
                                        }
                                    }
                                }
                                Err(v30) => {
                                    Ok(Err::<(), Bytes>(v30))
                                }
                            }
                        }
                        Err(v31) => {
                            Ok(Err::<(), Bytes>(v31))
                        }
                    }
                }
                Err(v32) => {
                    Ok(Err::<(), Bytes>(v32))
                }
            }
        }
        Adt1::C3 => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt1::C4(_, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
    }
}

pub fn f13(v0: Adt3) -> Bytes {
    let m111 = v0;
    match m111 {
        Adt3::C0 => {
            f1(Str::lit("Unit"))
        }
        Adt3::C1 => {
            f1(Str::lit("Bool"))
        }
        Adt3::C2 => {
            f1(Str::lit("Nat"))
        }
        Adt3::C3 => {
            f1(Str::lit("Int"))
        }
        Adt3::C4 => {
            f1(Str::lit("Ordering"))
        }
        Adt3::C5 => {
            f1(Str::lit("Str"))
        }
        Adt3::C6 => {
            f1(Str::lit("Bytes"))
        }
        Adt3::C7 => {
            f1(Str::lit("Overflow"))
        }
        Adt3::C8 => {
            f1(Str::lit("I128"))
        }
    }
}

pub fn f14(v0: Adt4) -> Bytes {
    let m112 = v0;
    match m112 {
        Adt4::C0 => {
            f1(Str::lit("U8"))
        }
        Adt4::C1 => {
            f1(Str::lit("U16"))
        }
        Adt4::C2 => {
            f1(Str::lit("U32"))
        }
        Adt4::C3 => {
            f1(Str::lit("U64"))
        }
        Adt4::C4 => {
            f1(Str::lit("I8"))
        }
        Adt4::C5 => {
            f1(Str::lit("I16"))
        }
        Adt4::C6 => {
            f1(Str::lit("I32"))
        }
        Adt4::C7 => {
            f1(Str::lit("I64"))
        }
    }
}

pub fn f15(v0: Adt5) -> Bytes {
    let m113 = v0;
    match m113 {
        Adt5::C0 => {
            f1(Str::lit("Option"))
        }
        Adt5::C1 => {
            f1(Str::lit("List"))
        }
        Adt5::C2 => {
            f1(Str::lit("Rc"))
        }
        Adt5::C3 => {
            f1(Str::lit("Fallible"))
        }
        Adt5::C4 => {
            f1(Str::lit("Ref"))
        }
    }
}

pub fn f16(v0: Adt6) -> Bytes {
    let m114 = v0;
    match m114 {
        Adt6::C0 => {
            f1(Str::lit("Result"))
        }
        Adt6::C1 => {
            f1(Str::lit("Pair"))
        }
    }
}

pub fn f17(v0: Adt7) -> Bytes {
    let m115 = v0;
    match m115 {
        Adt7::C0 => {
            f1(Str::lit("Adt"))
        }
        Adt7::C1 => {
            f1(Str::lit("Fn"))
        }
        Adt7::C2 => {
            f1(Str::lit("Param"))
        }
    }
}

pub fn f18(v0: List<(u64, List<(u64, u64)>)>, v1: u64) -> Option<List<(u64, u64)>> {
    f34(Fn0::F35(v1), None::<List<(u64, u64)>>, v0)
}

pub fn f19(v0: Adt2) -> R<u64> {
    let m116 = v0;
    match m116 {
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
        Adt2::C6(_, r119) => {
            let v11 = (*r119).clone();
            let v12: u64 = f19(v11)?;
            let a117 = 1u64;
            let a118 = v12;
            nat_add(a117, a118)
        }
    }
}

pub fn f20(v0: List<(u64, bool)>, v1: u64) -> Option<bool> {
    f36(Fn1::F37(v1), None::<bool>, v0)
}

pub fn f21(v0: Bytes) -> R<Bytes> {
    let v1: u64 = {
        let (h120, _) = f38(Fn2::F39(v0.clone()), {
            let a121 = v0.clone();
            length_bytes(a121)
        }, 0u64)?;
        h120
    };
    Ok(f40(v0.clone(), v1, {
        let a123 = {
            let a122 = v0;
            length_bytes(a122)
        };
        let a124 = v1;
        nat_sub(a123, a124)
    }))
}

pub fn f22(v0: u64, v1: u64) -> Bytes {
    f40(f1(Str::lit("0123456789")), {
        let a128 = {
            let a125 = v0;
            let a126 = v1;
            let a127 = 0u64;
            nat_quot(a125, a126, a127)
        };
        let a129 = 10u64;
        let a130 = 0u64;
        nat_rem(a128, a129, a130)
    }, 1u64)
}

pub fn f23(v0: Adt5) -> bool {
    let m131 = v0;
    match m131 {
        Adt5::C0 => {
            false
        }
        Adt5::C1 => {
            false
        }
        Adt5::C2 => {
            false
        }
        Adt5::C3 => {
            true
        }
        Adt5::C4 => {
            false
        }
    }
}

pub fn f24(v0: Adt27, p136: Adt8) -> R<Result<(), Bytes>> {
    let m135 = p136;
    {
        let mut m135 = m135;
        loop {
            let step = {
                let v1 = m135;
                let m132 = v1;
                match m132 {
                    Adt8::C0 => {
                        Err::<Adt8, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt8::C1(_, v3, r134) => {
                        let v4 = (*r134).clone();
                        let m133 = f25(v0.clone(), v3)?;
                        match m133 {
                            Ok(_) => {
                                Ok::<Adt8, R<Result<(), Bytes>>>(v4)
                            }
                            Err(v6) => {
                                Err::<Adt8, R<Result<(), Bytes>>>(Ok(Err::<(), Bytes>(v6)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m135 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f25(v0: Adt27, p142: Adt2) -> R<Result<(), Bytes>> {
    let m141 = p142;
    {
        let mut m141 = m141;
        loop {
            let step = {
                let v1 = m141;
                let m137 = v1;
                match m137 {
                    Adt2::C0(_) => {
                        Err::<Adt2, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt2::C1(_) => {
                        Err::<Adt2, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt2::C2(_, _) => {
                        Err::<Adt2, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt2::C3(_, _, _) => {
                        Err::<Adt2, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt2::C4(_, _) => {
                        Err::<Adt2, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt2::C5 => {
                        Err::<Adt2, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt2::C6(r139, r140) => {
                        let v11 = (*r139).clone();
                        let v12 = (*r140).clone();
                        let m138 = f26(v0.clone(), v11)?;
                        match m138 {
                            Ok(_) => {
                                Ok::<Adt2, R<Result<(), Bytes>>>(v12)
                            }
                            Err(v14) => {
                                Err::<Adt2, R<Result<(), Bytes>>>(Ok(Err::<(), Bytes>(v14)))
                            }
                        }
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

pub fn f26(v0: Adt27, v1: Adt2) -> R<Result<(), Bytes>> {
    let v2: bool = f41(v1.clone());
    let v3: bool = if match v0.clone() {
        Adt27::C0(h143, _, _, _, _) => {
            h143
        }
    } {
        v2
    } else {
        false
    };
    let m144 = v3;
    if m144 {
        Ok(Err::<(), Bytes>(f4(f1(Str::lit("hidden allocation: rust-core renders the heap type ")), f4(f5(v1)?, f1(Str::lit(", which rust-core does not provide"))))))
    } else {
        f42(v0, v1)
    }
}

pub fn f27(v0: Adt27, v1: u64, v2: bool, p156: Adt10) -> R<Result<(), Bytes>> {
    let m155 = p156;
    {
        let mut m155 = m155;
        loop {
            let step = {
                let v3 = m155;
                let m145 = v3;
                match m145 {
                    Adt10::C0 => {
                        Err::<Adt10, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt10::C1(v4, v5, v6, r154) => {
                        let v7 = (*r154).clone();
                        let m149 = f20({
                            let m148 = match v0.clone() {
                                Adt27::C0(_, h147, _, _, _) => {
                                    h147
                                }
                            };
                            match m148 {
                                Adt26::C0(_, _, _, h146) => {
                                    h146
                                }
                            }
                        }, v4);
                        match m149 {
                            None => {
                                Err::<Adt10, R<Result<(), Bytes>>>(Ok(Err::<(), Bytes>(f4(f1(Str::lit("Fn")), f4(f10(v1)?, f4(f1(Str::lit(" dispatches to undeclared f")), f10(v4)?))))))
                            }
                            Some(v8) => {
                                let v9: bool = if v8 {
                                    !v6
                                } else {
                                    v6
                                };
                                let v10: bool = if v6 {
                                    !v2
                                } else {
                                    false
                                };
                                let m150 = v9;
                                if m150 {
                                    Err::<Adt10, R<Result<(), Bytes>>>(Ok(Err::<(), Bytes>(f4(f1(Str::lit("arithmetic: Fn")), f4(f10(v1)?, f4(f1(Str::lit("::apply misstates the failure of f")), f10(v4)?))))))
                                } else {
                                    let m151 = v10;
                                    if m151 {
                                        Err::<Adt10, R<Result<(), Bytes>>>(Ok(Err::<(), Bytes>(f4(f1(Str::lit("arithmetic: Fn")), f4(f10(v1)?, f4(f1(Str::lit("::apply misstates the failure of f")), f10(v4)?))))))
                                    } else {
                                        let m152 = f43(v5);
                                        if m152 {
                                            let m153 = f44(v0.clone(), f1(Str::lit("a boxed capture")));
                                            match m153 {
                                                Ok(_) => {
                                                    Ok::<Adt10, R<Result<(), Bytes>>>(v7)
                                                }
                                                Err(v12) => {
                                                    Err::<Adt10, R<Result<(), Bytes>>>(Ok(Err::<(), Bytes>(v12)))
                                                }
                                            }
                                        } else {
                                            Ok::<Adt10, R<Result<(), Bytes>>>(v7)
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m155 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f28(v0: Adt27, p163: Adt28, p164: Adt13) -> R<Result<Adt28, Bytes>> {
    let m162 = (p163, p164);
    {
        let mut m162 = m162;
        loop {
            let step = {
                let (v1, v2) = m162;
                let m157 = v2;
                match m157 {
                    Adt13::C0 => {
                        Err::<(Adt28, Adt13), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v1)))
                    }
                    Adt13::C1(v3, v4, r161) => {
                        let v5 = (*r161).clone();
                        let m158 = f26(v0.clone(), v4)?;
                        match m158 {
                            Ok(_) => {
                                let m160 = f45(v1, match v0.clone() {
                                    Adt27::C0(_, h159, _, _, _) => {
                                        h159
                                    }
                                }, v3)?;
                                match m160 {
                                    Ok(v7) => {
                                        Ok::<(Adt28, Adt13), R<Result<Adt28, Bytes>>>((v7, v5))
                                    }
                                    Err(v8) => {
                                        Err::<(Adt28, Adt13), R<Result<Adt28, Bytes>>>(Ok(Err::<Adt28, Bytes>(v8)))
                                    }
                                }
                            }
                            Err(v9) => {
                                Err::<(Adt28, Adt13), R<Result<Adt28, Bytes>>>(Ok(Err::<Adt28, Bytes>(v9)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m162 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f29(v0: Adt28, v1: Adt27, v2: Adt17, v3: bool) -> R<Result<Adt28, Bytes>> {
    let m165 = v2;
    match m165 {
        Adt17::C0(_, _) => {
            Ok(Ok::<Adt28, Bytes>(v0))
        }
        Adt17::C1(_, _, _) => {
            Ok(Ok::<Adt28, Bytes>(v0))
        }
        Adt17::C2(_, _, _) => {
            Ok(Ok::<Adt28, Bytes>(v0))
        }
        Adt17::C3(_, _, _, _) => {
            Ok(Ok::<Adt28, Bytes>(v0))
        }
        Adt17::C4(_, _, _, _) => {
            Ok(Ok::<Adt28, Bytes>(v0))
        }
        Adt17::C5(_, _, _) => {
            Ok(Ok::<Adt28, Bytes>(v0))
        }
        Adt17::C6(_, _, _) => {
            Ok(Ok::<Adt28, Bytes>(v0))
        }
        Adt17::C7(_, _, _, _) => {
            Ok(Ok::<Adt28, Bytes>(v0))
        }
        Adt17::C8(_, _, _) => {
            Ok(Ok::<Adt28, Bytes>(v0))
        }
        Adt17::C9(_) => {
            Ok(Ok::<Adt28, Bytes>(v0))
        }
        Adt17::C10(_, _, _) => {
            Ok(Ok::<Adt28, Bytes>(v0))
        }
        Adt17::C11(r173, r174) => {
            let v37 = (*r173).clone();
            let v38 = (*r174).clone();
            let v39: List<Adt12> = match v0.clone() {
                Adt28::C0(_, h166, _, _, _) => {
                    h166
                }
            };
            let m167 = f46(v0, v1.clone(), v37)?;
            match m167 {
                Ok(v40) => {
                    let m168 = f47(v40, v1, v38, v3)?;
                    match m168 {
                        Ok(v41) => {
                            Ok(Ok::<Adt28, Bytes>(Adt28::C0(match v41.clone() {
                                Adt28::C0(h169, _, _, _, _) => {
                                    h169
                                }
                            }, v39, match v41.clone() {
                                Adt28::C0(_, _, h170, _, _) => {
                                    h170
                                }
                            }, match v41.clone() {
                                Adt28::C0(_, _, _, h171, _) => {
                                    h171
                                }
                            }, match v41 {
                                Adt28::C0(_, _, _, _, h172) => {
                                    h172
                                }
                            })))
                        }
                        Err(v42) => {
                            Ok(Err::<Adt28, Bytes>(v42))
                        }
                    }
                }
                Err(v43) => {
                    Ok(Err::<Adt28, Bytes>(v43))
                }
            }
        }
        Adt17::C12 => {
            Ok(Ok::<Adt28, Bytes>(v0))
        }
        Adt17::C13(_, _) => {
            Ok(Ok::<Adt28, Bytes>(v0))
        }
        Adt17::C14(_, _, _, _, _, _) => {
            Ok(Ok::<Adt28, Bytes>(v0))
        }
        Adt17::C15(_, _, _) => {
            Ok(Ok::<Adt28, Bytes>(v0))
        }
        Adt17::C16(_, _) => {
            Ok(Ok::<Adt28, Bytes>(v0))
        }
    }
}

pub fn f30(v0: Bytes) -> Bytes {
    f4(f1(Str::lit("`")), f4(v0, f1(Str::lit("`"))))
}

pub fn f31(v0: Adt12) -> R<Bytes> {
    let m175 = v0;
    match m175 {
        Adt12::C0(v1) => {
            Ok(f4(f1(Str::lit("v")), f10(v1)?))
        }
        Adt12::C1(v2) => {
            Ok(f4(f1(Str::lit("m")), f10(v2)?))
        }
        Adt12::C2(v3) => {
            Ok(f4(f1(Str::lit("a")), f10(v3)?))
        }
        Adt12::C3(v4) => {
            Ok(f4(f1(Str::lit("c")), f10(v4)?))
        }
        Adt12::C4(v5) => {
            Ok(f4(f1(Str::lit("r")), f10(v5)?))
        }
        Adt12::C5(v6) => {
            Ok(f4(f1(Str::lit("h")), f10(v6)?))
        }
        Adt12::C6(v7) => {
            Ok(f4(f1(Str::lit("k")), f10(v7)?))
        }
        Adt12::C7(v8) => {
            Ok(f4(f1(Str::lit("p")), f10(v8)?))
        }
        Adt12::C8(v9) => {
            Ok(f4(f1(Str::lit("f")), f10(v9)?))
        }
        Adt12::C9(v10) => {
            Ok(v10)
        }
    }
}

pub fn f32(v0: List<(Adt12, u64)>, v1: Adt26, v2: Adt17, v3: bool) -> R<Result<List<(Adt12, u64)>, Bytes>> {
    let m176 = v2;
    match m176 {
        Adt17::C0(_, _) => {
            Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0))
        }
        Adt17::C1(_, _, _) => {
            Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0))
        }
        Adt17::C2(_, _, _) => {
            Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0))
        }
        Adt17::C3(_, _, _, _) => {
            Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0))
        }
        Adt17::C4(_, _, _, _) => {
            Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0))
        }
        Adt17::C5(_, _, _) => {
            Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0))
        }
        Adt17::C6(_, _, _) => {
            Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0))
        }
        Adt17::C7(_, _, _, _) => {
            Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0))
        }
        Adt17::C8(_, _, _) => {
            Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0))
        }
        Adt17::C9(_) => {
            Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0))
        }
        Adt17::C10(_, _, _) => {
            Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0))
        }
        Adt17::C11(r178, r179) => {
            let v37 = (*r178).clone();
            let v38 = (*r179).clone();
            let m177 = f48(v0, v1.clone(), v37)?;
            match m177 {
                Ok(v39) => {
                    f49(v39, v1, v38, v3)
                }
                Err(v40) => {
                    Ok(Err::<List<(Adt12, u64)>, Bytes>(v40))
                }
            }
        }
        Adt17::C12 => {
            Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0))
        }
        Adt17::C13(_, _) => {
            Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0))
        }
        Adt17::C14(_, _, _, _, _, _) => {
            Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0))
        }
        Adt17::C15(_, _, _) => {
            Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0))
        }
        Adt17::C16(_, _) => {
            Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0))
        }
    }
}

pub fn f33(p183: List<(Adt12, u64)>, p184: Adt13) -> List<(Adt12, u64)> {
    let m182 = (p183, p184);
    {
        let mut m182 = m182;
        loop {
            let step = {
                let (v0, v1) = m182;
                let m180 = v1;
                match m180 {
                    Adt13::C0 => {
                        Err::<(List<(Adt12, u64)>, Adt13), List<(Adt12, u64)>>(v0)
                    }
                    Adt13::C1(v2, v3, r181) => {
                        let v4 = (*r181).clone();
                        Ok::<(List<(Adt12, u64)>, Adt13), List<(Adt12, u64)>>((f50(v0, v2, v3), v4))
                    }
                }
            };
            match step {
                Ok(next) => { m182 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f34(v0: Fn0, p188: Option<List<(u64, u64)>>, p189: List<(u64, List<(u64, u64)>)>) -> Option<List<(u64, u64)>> {
    let m187 = (p188, p189);
    {
        let mut m187 = m187;
        loop {
            let step = {
                let (v1, v2) = m187;
                let m185 = v2;
                match m185.uncons() {
                    None => {
                        Err::<(Option<List<(u64, u64)>>, List<(u64, List<(u64, u64)>)>), Option<List<(u64, u64)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<List<(u64, u64)>>, List<(u64, List<(u64, u64)>)>), Option<List<(u64, u64)>>>(({
                            let c186: Fn0 = v0.clone();
                            c186.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m187 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f35(v0: u64, v1: Option<List<(u64, u64)>>, v2: (u64, List<(u64, u64)>)) -> Option<List<(u64, u64)>> {
    let m190 = v1;
    match m190 {
        None => {
            let a192 = {
                let (h191, _) = v2;
                h191
            };
            let a193 = v0;
            if nat_eq(a192, a193) {
                Some({
                    let (_, h194) = v2;
                    h194
                })
            } else {
                None::<List<(u64, u64)>>
            }
        }
        Some(v3) => {
            Some(v3)
        }
    }
}

pub fn f36(v0: Fn1, p198: Option<bool>, p199: List<(u64, bool)>) -> Option<bool> {
    let m197 = (p198, p199);
    {
        let mut m197 = m197;
        loop {
            let step = {
                let (v1, v2) = m197;
                let m195 = v2;
                match m195.uncons() {
                    None => {
                        Err::<(Option<bool>, List<(u64, bool)>), Option<bool>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<bool>, List<(u64, bool)>), Option<bool>>(({
                            let c196: Fn1 = v0.clone();
                            c196.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m197 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f37(v0: u64, v1: Option<bool>, v2: (u64, bool)) -> Option<bool> {
    let m200 = v1;
    match m200 {
        None => {
            let a202 = {
                let (h201, _) = v2;
                h201
            };
            let a203 = v0;
            if nat_eq(a202, a203) {
                Some({
                    let (_, h204) = v2;
                    h204
                })
            } else {
                None::<bool>
            }
        }
        Some(v3) => {
            Some(v3)
        }
    }
}

pub fn f38(v0: Fn2, p209: u64, p210: u64) -> R<(u64, bool)> {
    let m208 = (p209, p210);
    {
        let mut m208 = m208;
        loop {
            let step = {
                let (v1, v2) = m208;
                let m205 = v1;
                if m205 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m205 - 1;
                    let m207 = {
                        let c206: Fn2 = v0.clone();
                        c206.apply(v2)?
                    };
                    match m207 {
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
                Ok(next) => { m208 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f39(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f51(v0, v1)
}

pub fn f40(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m214 = {
        let a211 = v0;
        let a212 = v1;
        let a213 = v2;
        slice_bytes(a211, a212, a213)
    };
    match m214 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f41(v0: Adt2) -> bool {
    let m215 = v0;
    match m215 {
        Adt2::C0(v1) => {
            f52(v1)
        }
        Adt2::C1(_) => {
            false
        }
        Adt2::C2(v3, r217) => {
            let v4 = (*r217).clone();
            let m216 = f53(v3);
            if m216 {
                true
            } else {
                f41(v4)
            }
        }
        Adt2::C3(_, r218, r219) => {
            let v6 = (*r218).clone();
            let v7 = (*r219).clone();
            let v8: bool = f41(v6);
            let v9: bool = f41(v7);
            if v8 {
                true
            } else {
                v9
            }
        }
        Adt2::C4(v10, _) => {
            f54(v10)
        }
        Adt2::C5 => {
            false
        }
        Adt2::C6(_, _) => {
            false
        }
    }
}

pub fn f42(v0: Adt27, v1: Adt2) -> R<Result<(), Bytes>> {
    let m220 = v1;
    match m220 {
        Adt2::C0(_) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt2::C1(_) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt2::C2(_, r232) => {
            let v5 = (*r232).clone();
            f42(v0, v5)
        }
        Adt2::C3(_, r233, r234) => {
            let v7 = (*r233).clone();
            let v8 = (*r234).clone();
            let m221 = f42(v0.clone(), v7)?;
            match m221 {
                Ok(_) => {
                    f42(v0, v8)
                }
                Err(v10) => {
                    Ok(Err::<(), Bytes>(v10))
                }
            }
        }
        Adt2::C4(v11, v12) => {
            let m222 = f54(v11.clone());
            if m222 {
                Ok(Ok::<(), Bytes>(()))
            } else {
                let m223 = f6(v11);
                if m223 {
                    let m227 = f7({
                        let m226 = match v0 {
                            Adt27::C0(_, h225, _, _, _) => {
                                h225
                            }
                        };
                        match m226 {
                            Adt26::C0(_, h224, _, _) => {
                                h224
                            }
                        }
                    }, v12);
                    if m227 {
                        Ok(Ok::<(), Bytes>(()))
                    } else {
                        Ok(Err::<(), Bytes>(f4(f1(Str::lit("type Fn")), f4(f10(v12)?, f1(Str::lit(" is not declared"))))))
                    }
                } else {
                    let m231 = f7({
                        let m230 = match v0 {
                            Adt27::C0(_, h229, _, _, _) => {
                                h229
                            }
                        };
                        match m230 {
                            Adt26::C0(h228, _, _, _) => {
                                h228
                            }
                        }
                    }, v12);
                    if m231 {
                        Ok(Ok::<(), Bytes>(()))
                    } else {
                        Ok(Err::<(), Bytes>(f4(f1(Str::lit("type Adt")), f4(f10(v12)?, f1(Str::lit(" is not declared"))))))
                    }
                }
            }
        }
        Adt2::C5 => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt2::C6(_, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
    }
}

pub fn f43(v0: List<Adt11>) -> bool {
    f55(Fn3::F56, false, v0)
}

pub fn f44(v0: Adt27, v1: Bytes) -> Result<(), Bytes> {
    let m236 = match v0 {
        Adt27::C0(h235, _, _, _, _) => {
            h235
        }
    };
    if m236 {
        Err::<(), Bytes>(f4(f1(Str::lit("hidden allocation: rust-core renders ")), f4(v1, f1(Str::lit(", which requires heap allocation")))))
    } else {
        Ok::<(), Bytes>(())
    }
}

pub fn f45(v0: Adt28, v1: Adt26, v2: Adt14) -> R<Result<Adt28, Bytes>> {
    let m237 = v2;
    match m237 {
        Adt14::C0(_) => {
            Ok(Ok::<Adt28, Bytes>(v0))
        }
        Adt14::C1(v4) => {
            let m239 = f57(match v0.clone() {
                Adt28::C0(h238, _, _, _, _) => {
                    h238
                }
            }, v4.clone());
            if m239 {
                Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("hygiene: ")), f4(f30(f31(v4)?), f1(Str::lit(" is bound twice in one function"))))))
            } else {
                Ok(Ok::<Adt28, Bytes>(Adt28::C0(List::cons(v4.clone(), match v0.clone() {
                    Adt28::C0(h240, _, _, _, _) => {
                        h240
                    }
                }), List::cons(v4, match v0.clone() {
                    Adt28::C0(_, h241, _, _, _) => {
                        h241
                    }
                }), match v0.clone() {
                    Adt28::C0(_, _, h242, _, _) => {
                        h242
                    }
                }, match v0.clone() {
                    Adt28::C0(_, _, _, h243, _) => {
                        h243
                    }
                }, match v0 {
                    Adt28::C0(_, _, _, _, h244) => {
                        h244
                    }
                })))
            }
        }
        Adt14::C2(r250) => {
            let v5 = (*r250).clone();
            f45(v0, v1, v5)
        }
        Adt14::C3(_, r251) => {
            let v7 = (*r251).clone();
            f45(v0, v1, v7)
        }
        Adt14::C4(v8, v9, r252) => {
            let v10 = (*r252).clone();
            let m245 = f58(v1.clone(), v8, v9);
            match m245 {
                None => {
                    Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("pattern Adt")), f4(f10(v8)?, f4(f1(Str::lit("::C")), f4(f10(v9)?, f1(Str::lit(" is not declared"))))))))
                }
                Some(v11) => {
                    let m248 = {
                        let a246 = v11;
                        let a247 = f59(v10.clone())?;
                        nat_eq(a246, a247)
                    };
                    if m248 {
                        f45(v0, v1, v10)
                    } else {
                        Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("pattern Adt")), f4(f10(v8)?, f4(f1(Str::lit("::C")), f4(f10(v9)?, f4(f1(Str::lit(" binds ")), f4(f10(f59(v10)?)?, f4(f1(Str::lit(" fields of ")), f10(v11)?)))))))))
                    }
                }
            }
        }
        Adt14::C5 => {
            Ok(Ok::<Adt28, Bytes>(v0))
        }
        Adt14::C6(r253, r254) => {
            let v12 = (*r253).clone();
            let v13 = (*r254).clone();
            let m249 = f45(v0, v1.clone(), v12)?;
            match m249 {
                Ok(v14) => {
                    f45(v14, v1, v13)
                }
                Err(v15) => {
                    Ok(Err::<Adt28, Bytes>(v15))
                }
            }
        }
    }
}

pub fn f46(p267: Adt28, v1: Adt27, p268: Adt17) -> R<Result<Adt28, Bytes>> {
    let m266 = (p267, p268);
    {
        let mut m266 = m266;
        loop {
            let step = {
                let (v0, v2) = m266;
                let m255 = v2;
                match m255 {
                    Adt17::C0(_, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C1(_, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C2(_, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C3(_, _, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C4(_, _, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C5(_, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C6(_, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C7(_, _, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C8(_, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C9(_) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C10(_, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C11(_, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C12 => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C13(_, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C14(v40, v41, v42, r264, _, r265) => {
                        let v43 = (*r264).clone();
                        let v45 = (*r265).clone();
                        let m257 = {
                            let m256 = v41;
                            if m256 {
                                f26(v1.clone(), v42)?
                            } else {
                                Ok::<(), Bytes>(())
                            }
                        };
                        match m257 {
                            Ok(_) => {
                                let m261 = {
                                    let m258 = f60(v40.clone(), v43.clone());
                                    match m258 {
                                        None => {
                                            f47(v0, v1.clone(), v43, false)?
                                        }
                                        Some(v47) => {
                                            let m259 = v40.clone();
                                            match m259 {
                                                Adt14::C0(_) => {
                                                    f47(v0, v1.clone(), v43, false)?
                                                }
                                                Adt14::C1(_) => {
                                                    f47(v0, v1.clone(), v43, false)?
                                                }
                                                Adt14::C2(r260) => {
                                                    let v50 = (*r260).clone();
                                                    f61(v0, v47, v50)?
                                                }
                                                Adt14::C3(_, _) => {
                                                    f47(v0, v1.clone(), v43, false)?
                                                }
                                                Adt14::C4(_, _, _) => {
                                                    f47(v0, v1.clone(), v43, false)?
                                                }
                                                Adt14::C5 => {
                                                    f47(v0, v1.clone(), v43, false)?
                                                }
                                                Adt14::C6(_, _) => {
                                                    f47(v0, v1.clone(), v43, false)?
                                                }
                                            }
                                        }
                                    }
                                };
                                match m261 {
                                    Ok(v58) => {
                                        let m263 = f45(v58, match v1.clone() {
                                            Adt27::C0(_, h262, _, _, _) => {
                                                h262
                                            }
                                        }, v40)?;
                                        match m263 {
                                            Ok(v59) => {
                                                Ok::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>((v59, v45))
                                            }
                                            Err(v60) => {
                                                Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Err::<Adt28, Bytes>(v60)))
                                            }
                                        }
                                    }
                                    Err(v61) => {
                                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Err::<Adt28, Bytes>(v61)))
                                    }
                                }
                            }
                            Err(v62) => {
                                Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Err::<Adt28, Bytes>(v62)))
                            }
                        }
                    }
                    Adt17::C15(_, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C16(_, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                }
            };
            match step {
                Ok(next) => { m266 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f47(v0: Adt28, v1: Adt27, v2: Adt17, v3: bool) -> R<Result<Adt28, Bytes>> {
    let m269 = v2;
    match m269 {
        Adt17::C0(v4, v5) => {
            let v6: bool = if v3 {
                if match v1.clone() {
                    Adt27::C0(_, _, _, _, h270) => {
                        h270
                    }
                } {
                    false
                } else {
                    match v1.clone() {
                        Adt27::C0(_, _, h271, _, _) => {
                            h271
                        }
                    }
                }
            } else {
                false
            };
            let m272 = v6;
            if m272 {
                Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("arithmetic: the value a fallible function returns, ")), f4(f62(match v1 {
                    Adt27::C0(_, _, _, h273, _) => {
                        h273
                    }
                }, Adt17::C0(v4, v5))?, f1(Str::lit(", is not a fallible value"))))))
            } else {
                let m274 = v4;
                match m274 {
                    Adt18::C0 => {
                        Ok(Ok::<Adt28, Bytes>(v0))
                    }
                    Adt18::C1 => {
                        Ok(Ok::<Adt28, Bytes>(v0))
                    }
                    Adt18::C2 => {
                        Ok(Ok::<Adt28, Bytes>(v0))
                    }
                    Adt18::C3 => {
                        Ok(Ok::<Adt28, Bytes>(v0))
                    }
                    Adt18::C4(_) => {
                        Ok(Ok::<Adt28, Bytes>(v0))
                    }
                    Adt18::C5 => {
                        Ok(f63(v1, v0, f1(Str::lit("a string literal"))))
                    }
                    Adt18::C6 => {
                        Ok(f63(v1, v0, f1(Str::lit("a byte string literal"))))
                    }
                    Adt18::C7 => {
                        Ok(Ok::<Adt28, Bytes>(v0))
                    }
                    Adt18::C8 => {
                        Ok(Err::<Adt28, Bytes>(f1(Str::lit("overflow: the overflow value stands outside the payload of an `Err`"))))
                    }
                }
            }
        }
        Adt17::C1(v8, v9, v10) => {
            let v11: bool = if v3 {
                if match v1.clone() {
                    Adt27::C0(_, _, _, _, h275) => {
                        h275
                    }
                } {
                    false
                } else {
                    match v1.clone() {
                        Adt27::C0(_, _, h276, _, _) => {
                            h276
                        }
                    }
                }
            } else {
                false
            };
            let m277 = v11;
            if m277 {
                Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("arithmetic: the value a fallible function returns, ")), f4(f62(match v1 {
                    Adt27::C0(_, _, _, h278, _) => {
                        h278
                    }
                }, Adt17::C1(v8, v9, v10))?, f1(Str::lit(", is not a fallible value"))))))
            } else {
                let m279 = v8;
                match m279 {
                    Adt19::C0 => {
                        f64(v0, v9)
                    }
                    Adt19::C1 => {
                        let m280 = f65(v0.clone(), v9)?;
                        match m280 {
                            Ok(_) => {
                                Ok(Ok::<Adt28, Bytes>(v0))
                            }
                            Err(v13) => {
                                Ok(Err::<Adt28, Bytes>(v13))
                            }
                        }
                    }
                    Adt19::C2 => {
                        let m281 = f65(v0.clone(), v9)?;
                        match m281 {
                            Ok(_) => {
                                Ok(Ok::<Adt28, Bytes>(v0))
                            }
                            Err(v15) => {
                                Ok(Err::<Adt28, Bytes>(v15))
                            }
                        }
                    }
                    Adt19::C3 => {
                        let m282 = f65(v0.clone(), v9)?;
                        match m282 {
                            Ok(_) => {
                                Ok(Ok::<Adt28, Bytes>(v0))
                            }
                            Err(v17) => {
                                Ok(Err::<Adt28, Bytes>(v17))
                            }
                        }
                    }
                    Adt19::C4 => {
                        let m283 = f44(v1, f1(Str::lit("a boxed read")));
                        match m283 {
                            Ok(_) => {
                                let m284 = f65(v0.clone(), v9)?;
                                match m284 {
                                    Ok(_) => {
                                        Ok(Ok::<Adt28, Bytes>(v0))
                                    }
                                    Err(v20) => {
                                        Ok(Err::<Adt28, Bytes>(v20))
                                    }
                                }
                            }
                            Err(v21) => {
                                Ok(Err::<Adt28, Bytes>(v21))
                            }
                        }
                    }
                }
            }
        }
        Adt17::C2(v22, r444, v24) => {
            let v23 = (*r444).clone();
            let m285 = v22.clone();
            match m285 {
                Adt20::C0 => {
                    let v25: bool = if v3 {
                        if match v1.clone() {
                            Adt27::C0(_, _, _, _, h286) => {
                                h286
                            }
                        } {
                            false
                        } else {
                            match v1.clone() {
                                Adt27::C0(_, _, h287, _, _) => {
                                    h287
                                }
                            }
                        }
                    } else {
                        false
                    };
                    let m288 = v25;
                    if m288 {
                        Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("arithmetic: the value a fallible function returns, ")), f4(f62(match v1 {
                            Adt27::C0(_, _, _, h289, _) => {
                                h289
                            }
                        }, Adt17::C2(v22, std::rc::Rc::new(v23), v24))?, f1(Str::lit(", is not a fallible value"))))))
                    } else {
                        let m290 = f66(v23.clone());
                        if m290 {
                            Ok(Err::<Adt28, Bytes>(f1(Str::lit("a negated zero test or predecessor: a zero test is negated as its complement, and a predecessor is not a Boolean"))))
                        } else {
                            f47(v0, v1, v23, false)
                        }
                    }
                }
                Adt20::C1 => {
                    let v26: bool = if v3 {
                        if match v1.clone() {
                            Adt27::C0(_, _, _, _, h291) => {
                                h291
                            }
                        } {
                            false
                        } else {
                            match v1.clone() {
                                Adt27::C0(_, _, h292, _, _) => {
                                    h292
                                }
                            }
                        }
                    } else {
                        false
                    };
                    let m293 = v26;
                    if m293 {
                        Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("arithmetic: the value a fallible function returns, ")), f4(f62(match v1 {
                            Adt27::C0(_, _, _, h294, _) => {
                                h294
                            }
                        }, Adt17::C2(v22, std::rc::Rc::new(v23), v24))?, f1(Str::lit(", is not a fallible value"))))))
                    } else {
                        let m295 = f44(v1.clone(), f1(Str::lit("a box")));
                        match m295 {
                            Ok(_) => {
                                f47(v0, v1, v23, false)
                            }
                            Err(v28) => {
                                Ok(Err::<Adt28, Bytes>(v28))
                            }
                        }
                    }
                }
                Adt20::C2 => {
                    let v29: bool = if v3 {
                        if match v1.clone() {
                            Adt27::C0(_, _, _, _, h296) => {
                                h296
                            }
                        } {
                            false
                        } else {
                            match v1.clone() {
                                Adt27::C0(_, _, h297, _, _) => {
                                    h297
                                }
                            }
                        }
                    } else {
                        false
                    };
                    let m298 = v29;
                    if m298 {
                        Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("arithmetic: the value a fallible function returns, ")), f4(f62(match v1 {
                            Adt27::C0(_, _, _, h299, _) => {
                                h299
                            }
                        }, Adt17::C2(v22, std::rc::Rc::new(v23), v24))?, f1(Str::lit(", is not a fallible value"))))))
                    } else {
                        f47(v0, v1, v23, false)
                    }
                }
                Adt20::C3 => {
                    f47(v0, v1, v23, false)
                }
            }
        }
        Adt17::C3(v30, r445, v32, v33) => {
            let v31 = (*r445).clone();
            let m300 = v32;
            if m300 {
                let v34: bool = if v3 {
                    if match v1.clone() {
                        Adt27::C0(_, _, _, _, h301) => {
                            h301
                        }
                    } {
                        false
                    } else {
                        match v1.clone() {
                            Adt27::C0(_, _, h302, _, _) => {
                                h302
                            }
                        }
                    }
                } else {
                    false
                };
                let m303 = v34;
                if m303 {
                    Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("arithmetic: the value a fallible function returns, ")), f4(f62(match v1 {
                        Adt27::C0(_, _, _, h304, _) => {
                            h304
                        }
                    }, Adt17::C3(v30, std::rc::Rc::new(v31), v32, v33))?, f1(Str::lit(", is not a fallible value"))))))
                } else {
                    let m305 = f67(v0, v1.clone(), false, v31.clone())?;
                    match m305 {
                        Ok(v35) => {
                            let m306 = v30;
                            match m306 {
                                Adt21::C0(v36) => {
                                    let m310 = f20({
                                        let m309 = match v1.clone() {
                                            Adt27::C0(_, h308, _, _, _) => {
                                                h308
                                            }
                                        };
                                        match m309 {
                                            Adt26::C0(_, _, _, h307) => {
                                                h307
                                            }
                                        }
                                    }, v36);
                                    match m310 {
                                        None => {
                                            Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("f")), f4(f10(v36)?, f1(Str::lit(" is not declared"))))))
                                        }
                                        Some(v37) => {
                                            Ok(f68(v35, v1.clone(), v37, v32, if match v1 {
                                                Adt27::C0(_, _, _, _, h311) => {
                                                    h311
                                                }
                                            } {
                                                false
                                            } else {
                                                v3
                                            }, f4(f1(Str::lit("f")), f10(v36)?)))
                                        }
                                    }
                                }
                                Adt21::C1(v39) => {
                                    let v40: Bytes = f70(match v1.clone() {
                                        Adt27::C0(_, _, _, h313, _) => {
                                            h313
                                        }
                                    }, v39.clone());
                                    let m316 = {
                                        let m315 = f71(match v1.clone() {
                                            Adt27::C0(_, _, _, h314, _) => {
                                                h314
                                            }
                                        }, v39.clone());
                                        if m315 {
                                            f44(v1.clone(), f4(f1(Str::lit("the runtime function ")), f30(v40.clone())))
                                        } else {
                                            Ok::<(), Bytes>(())
                                        }
                                    };
                                    match m316 {
                                        Ok(_) => {
                                            Ok(f68(v35, v1.clone(), f72(match v1.clone() {
                                                Adt27::C0(_, _, _, h317, _) => {
                                                    h317
                                                }
                                            }, v39), v32, if match v1 {
                                                Adt27::C0(_, _, _, _, h318) => {
                                                    h318
                                                }
                                            } {
                                                false
                                            } else {
                                                v3
                                            }, f30(v40)))
                                        }
                                        Err(v42) => {
                                            Ok(Err::<Adt28, Bytes>(v42))
                                        }
                                    }
                                }
                                Adt21::C2(v38) => {
                                    f69(v35, v1.clone(), v38, v31, v32, if match v1 {
                                        Adt27::C0(_, _, _, _, h312) => {
                                            h312
                                        }
                                    } {
                                        false
                                    } else {
                                        v3
                                    })
                                }
                            }
                        }
                        Err(v43) => {
                            Ok(Err::<Adt28, Bytes>(v43))
                        }
                    }
                }
            } else {
                let m319 = f67(v0, v1.clone(), false, v31.clone())?;
                match m319 {
                    Ok(v44) => {
                        let m320 = v30;
                        match m320 {
                            Adt21::C0(v45) => {
                                let m324 = f20({
                                    let m323 = match v1.clone() {
                                        Adt27::C0(_, h322, _, _, _) => {
                                            h322
                                        }
                                    };
                                    match m323 {
                                        Adt26::C0(_, _, _, h321) => {
                                            h321
                                        }
                                    }
                                }, v45);
                                match m324 {
                                    None => {
                                        Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("f")), f4(f10(v45)?, f1(Str::lit(" is not declared"))))))
                                    }
                                    Some(v46) => {
                                        Ok(f68(v44, v1.clone(), v46, v32, if match v1 {
                                            Adt27::C0(_, _, _, _, h325) => {
                                                h325
                                            }
                                        } {
                                            false
                                        } else {
                                            v3
                                        }, f4(f1(Str::lit("f")), f10(v45)?)))
                                    }
                                }
                            }
                            Adt21::C1(v48) => {
                                let v49: Bytes = f70(match v1.clone() {
                                    Adt27::C0(_, _, _, h327, _) => {
                                        h327
                                    }
                                }, v48.clone());
                                let m330 = {
                                    let m329 = f71(match v1.clone() {
                                        Adt27::C0(_, _, _, h328, _) => {
                                            h328
                                        }
                                    }, v48.clone());
                                    if m329 {
                                        f44(v1.clone(), f4(f1(Str::lit("the runtime function ")), f30(v49.clone())))
                                    } else {
                                        Ok::<(), Bytes>(())
                                    }
                                };
                                match m330 {
                                    Ok(_) => {
                                        Ok(f68(v44, v1.clone(), f72(match v1.clone() {
                                            Adt27::C0(_, _, _, h331, _) => {
                                                h331
                                            }
                                        }, v48), v32, if match v1 {
                                            Adt27::C0(_, _, _, _, h332) => {
                                                h332
                                            }
                                        } {
                                            false
                                        } else {
                                            v3
                                        }, f30(v49)))
                                    }
                                    Err(v51) => {
                                        Ok(Err::<Adt28, Bytes>(v51))
                                    }
                                }
                            }
                            Adt21::C2(v47) => {
                                f69(v44, v1.clone(), v47, v31, v32, if match v1 {
                                    Adt27::C0(_, _, _, _, h326) => {
                                        h326
                                    }
                                } {
                                    false
                                } else {
                                    v3
                                })
                            }
                        }
                    }
                    Err(v52) => {
                        Ok(Err::<Adt28, Bytes>(v52))
                    }
                }
            }
        }
        Adt17::C4(v53, r446, v55, v56) => {
            let v54 = (*r446).clone();
            let m333 = v55;
            if m333 {
                let v57: bool = if v3 {
                    if match v1.clone() {
                        Adt27::C0(_, _, _, _, h334) => {
                            h334
                        }
                    } {
                        false
                    } else {
                        match v1.clone() {
                            Adt27::C0(_, _, h335, _, _) => {
                                h335
                            }
                        }
                    }
                } else {
                    false
                };
                let m336 = v57;
                if m336 {
                    Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("arithmetic: the value a fallible function returns, ")), f4(f62(match v1 {
                        Adt27::C0(_, _, _, h337, _) => {
                            h337
                        }
                    }, Adt17::C4(v53, std::rc::Rc::new(v54), v55, v56))?, f1(Str::lit(", is not a fallible value"))))))
                } else {
                    let m338 = f65(v0.clone(), v53)?;
                    match m338 {
                        Ok(_) => {
                            let m339 = f67(v0, v1, false, v54)?;
                            match m339 {
                                Ok(v59) => {
                                    Ok(Ok::<Adt28, Bytes>(Adt28::C0(match v59.clone() {
                                        Adt28::C0(h340, _, _, _, _) => {
                                            h340
                                        }
                                    }, match v59.clone() {
                                        Adt28::C0(_, h341, _, _, _) => {
                                            h341
                                        }
                                    }, match v59.clone() {
                                        Adt28::C0(_, _, h342, _, _) => {
                                            h342
                                        }
                                    }, match v59.clone() {
                                        Adt28::C0(_, _, _, h343, _) => {
                                            h343
                                        }
                                    }, if v55 {
                                        true
                                    } else {
                                        match v59 {
                                            Adt28::C0(_, _, _, _, h344) => {
                                                h344
                                            }
                                        }
                                    })))
                                }
                                Err(v60) => {
                                    Ok(Err::<Adt28, Bytes>(v60))
                                }
                            }
                        }
                        Err(v61) => {
                            Ok(Err::<Adt28, Bytes>(v61))
                        }
                    }
                }
            } else {
                let m345 = f65(v0.clone(), v53)?;
                match m345 {
                    Ok(_) => {
                        let m346 = f67(v0, v1, false, v54)?;
                        match m346 {
                            Ok(v63) => {
                                Ok(Ok::<Adt28, Bytes>(Adt28::C0(match v63.clone() {
                                    Adt28::C0(h347, _, _, _, _) => {
                                        h347
                                    }
                                }, match v63.clone() {
                                    Adt28::C0(_, h348, _, _, _) => {
                                        h348
                                    }
                                }, match v63.clone() {
                                    Adt28::C0(_, _, h349, _, _) => {
                                        h349
                                    }
                                }, match v63.clone() {
                                    Adt28::C0(_, _, _, h350, _) => {
                                        h350
                                    }
                                }, if v55 {
                                    true
                                } else {
                                    match v63 {
                                        Adt28::C0(_, _, _, _, h351) => {
                                            h351
                                        }
                                    }
                                })))
                            }
                            Err(v64) => {
                                Ok(Err::<Adt28, Bytes>(v64))
                            }
                        }
                    }
                    Err(v65) => {
                        Ok(Err::<Adt28, Bytes>(v65))
                    }
                }
            }
        }
        Adt17::C5(v66, r447, v68) => {
            let v67 = (*r447).clone();
            let v69: bool = if v3 {
                if match v1.clone() {
                    Adt27::C0(_, _, _, _, h352) => {
                        h352
                    }
                } {
                    f73(v66.clone())
                } else {
                    false
                }
            } else {
                false
            };
            let m353 = v69;
            if m353 {
                let m358 = f74(v0, Adt27::C0(match v1.clone() {
                    Adt27::C0(h354, _, _, _, _) => {
                        h354
                    }
                }, match v1.clone() {
                    Adt27::C0(_, h355, _, _, _) => {
                        h355
                    }
                }, match v1.clone() {
                    Adt27::C0(_, _, h356, _, _) => {
                        h356
                    }
                }, match v1.clone() {
                    Adt27::C0(_, _, _, h357, _) => {
                        h357
                    }
                }, false), v67.clone())?;
                match m358 {
                    Ok(v70) => {
                        let m359 = f75(v1, v66, f76(v67)?)?;
                        match m359 {
                            Ok(_) => {
                                Ok(Ok::<Adt28, Bytes>(v70))
                            }
                            Err(v72) => {
                                Ok(Err::<Adt28, Bytes>(v72))
                            }
                        }
                    }
                    Err(v73) => {
                        Ok(Err::<Adt28, Bytes>(v73))
                    }
                }
            } else {
                let v74: bool = if v3 {
                    if match v1.clone() {
                        Adt27::C0(_, _, _, _, h360) => {
                            h360
                        }
                    } {
                        false
                    } else {
                        if match v1.clone() {
                            Adt27::C0(_, _, h361, _, _) => {
                                h361
                            }
                        } {
                            let a362 = f77(v66.clone(), v67.clone())?;
                            bool_not(a362)
                        } else {
                            false
                        }
                    }
                } else {
                    false
                };
                let m363 = v74;
                if m363 {
                    Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("arithmetic: the value a fallible function returns, ")), f4(f62(match v1 {
                        Adt27::C0(_, _, _, h364, _) => {
                            h364
                        }
                    }, Adt17::C5(v66, std::rc::Rc::new(v67), v68))?, f1(Str::lit(", is not a fallible value"))))))
                } else {
                    let m365 = f67(v0, v1.clone(), f73(v66.clone()), v67.clone())?;
                    match m365 {
                        Ok(v75) => {
                            let m366 = f75(v1, v66, f76(v67)?)?;
                            match m366 {
                                Ok(_) => {
                                    Ok(Ok::<Adt28, Bytes>(v75))
                                }
                                Err(v77) => {
                                    Ok(Err::<Adt28, Bytes>(v77))
                                }
                            }
                        }
                        Err(v78) => {
                            Ok(Err::<Adt28, Bytes>(v78))
                        }
                    }
                }
            }
        }
        Adt17::C6(r448, r449, v81) => {
            let v79 = (*r448).clone();
            let v80 = (*r449).clone();
            let v82: bool = if v3 {
                if match v1.clone() {
                    Adt27::C0(_, _, _, _, h367) => {
                        h367
                    }
                } {
                    false
                } else {
                    match v1.clone() {
                        Adt27::C0(_, _, h368, _, _) => {
                            h368
                        }
                    }
                }
            } else {
                false
            };
            let m369 = v82;
            if m369 {
                Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("arithmetic: the value a fallible function returns, ")), f4(f62(match v1 {
                    Adt27::C0(_, _, _, h370, _) => {
                        h370
                    }
                }, Adt17::C6(std::rc::Rc::new(v79), std::rc::Rc::new(v80), v81))?, f1(Str::lit(", is not a fallible value"))))))
            } else {
                let m371 = f47(v0, v1.clone(), v79, false)?;
                match m371 {
                    Ok(v83) => {
                        f47(v83, v1, v80, false)
                    }
                    Err(v84) => {
                        Ok(Err::<Adt28, Bytes>(v84))
                    }
                }
            }
        }
        Adt17::C7(r450, r451, r452, _) => {
            let v85 = (*r450).clone();
            let v86 = (*r451).clone();
            let v87 = (*r452).clone();
            let m372 = f47(v0, v1.clone(), v85, false)?;
            match m372 {
                Ok(v89) => {
                    let v90: Adt28 = Adt28::C0(match v89.clone() {
                        Adt28::C0(h373, _, _, _, _) => {
                            h373
                        }
                    }, match v89.clone() {
                        Adt28::C0(_, h374, _, _, _) => {
                            h374
                        }
                    }, match v89.clone() {
                        Adt28::C0(_, _, h375, _, _) => {
                            h375
                        }
                    }, match v89.clone() {
                        Adt28::C0(_, _, _, h376, _) => {
                            h376
                        }
                    }, match v89.clone() {
                        Adt28::C0(_, _, _, _, h377) => {
                            h377
                        }
                    });
                    let m379 = f45(v90, match v1.clone() {
                        Adt27::C0(_, h378, _, _, _) => {
                            h378
                        }
                    }, Adt14::C0(Adt15::C0))?;
                    match m379 {
                        Ok(v91) => {
                            let m380 = f29(v91, v1.clone(), v86, v3)?;
                            match m380 {
                                Ok(v92) => {
                                    let v93: Adt28 = Adt28::C0(match v92.clone() {
                                        Adt28::C0(h381, _, _, _, _) => {
                                            h381
                                        }
                                    }, match v89.clone() {
                                        Adt28::C0(_, h382, _, _, _) => {
                                            h382
                                        }
                                    }, f78(match v89.clone() {
                                        Adt28::C0(_, _, h383, _, _) => {
                                            h383
                                        }
                                    }, match v92.clone() {
                                        Adt28::C0(_, _, h384, _, _) => {
                                            h384
                                        }
                                    }), f79(match v89.clone() {
                                        Adt28::C0(_, _, _, h385, _) => {
                                            h385
                                        }
                                    }, match v92.clone() {
                                        Adt28::C0(_, _, _, h386, _) => {
                                            h386
                                        }
                                    }), match v92 {
                                        Adt28::C0(_, _, _, _, h387) => {
                                            h387
                                        }
                                    });
                                    let v94: Adt28 = Adt28::C0(match v93.clone() {
                                        Adt28::C0(h388, _, _, _, _) => {
                                            h388
                                        }
                                    }, match v89.clone() {
                                        Adt28::C0(_, h389, _, _, _) => {
                                            h389
                                        }
                                    }, match v89.clone() {
                                        Adt28::C0(_, _, h390, _, _) => {
                                            h390
                                        }
                                    }, match v89.clone() {
                                        Adt28::C0(_, _, _, h391, _) => {
                                            h391
                                        }
                                    }, match v93.clone() {
                                        Adt28::C0(_, _, _, _, h392) => {
                                            h392
                                        }
                                    });
                                    let m394 = f45(v94, match v1.clone() {
                                        Adt27::C0(_, h393, _, _, _) => {
                                            h393
                                        }
                                    }, Adt14::C0(Adt15::C0))?;
                                    match m394 {
                                        Ok(v95) => {
                                            let m395 = f29(v95, v1, v87, v3)?;
                                            match m395 {
                                                Ok(v96) => {
                                                    let v97: Adt28 = Adt28::C0(match v96.clone() {
                                                        Adt28::C0(h396, _, _, _, _) => {
                                                            h396
                                                        }
                                                    }, match v89 {
                                                        Adt28::C0(_, h397, _, _, _) => {
                                                            h397
                                                        }
                                                    }, f78(match v93.clone() {
                                                        Adt28::C0(_, _, h398, _, _) => {
                                                            h398
                                                        }
                                                    }, match v96.clone() {
                                                        Adt28::C0(_, _, h399, _, _) => {
                                                            h399
                                                        }
                                                    }), f79(match v93 {
                                                        Adt28::C0(_, _, _, h400, _) => {
                                                            h400
                                                        }
                                                    }, match v96.clone() {
                                                        Adt28::C0(_, _, _, h401, _) => {
                                                            h401
                                                        }
                                                    }), match v96 {
                                                        Adt28::C0(_, _, _, _, h402) => {
                                                            h402
                                                        }
                                                    });
                                                    Ok(Ok::<Adt28, Bytes>(v97))
                                                }
                                                Err(v98) => {
                                                    Ok(Err::<Adt28, Bytes>(v98))
                                                }
                                            }
                                        }
                                        Err(v99) => {
                                            Ok(Err::<Adt28, Bytes>(v99))
                                        }
                                    }
                                }
                                Err(v100) => {
                                    Ok(Err::<Adt28, Bytes>(v100))
                                }
                            }
                        }
                        Err(v101) => {
                            Ok(Err::<Adt28, Bytes>(v101))
                        }
                    }
                }
                Err(v102) => {
                    Ok(Err::<Adt28, Bytes>(v102))
                }
            }
        }
        Adt17::C8(r453, r454, _) => {
            let v103 = (*r453).clone();
            let v104 = (*r454).clone();
            let m403 = f47(v0, v1.clone(), v103, false)?;
            match m403 {
                Ok(v106) => {
                    f80(v106.clone(), v1, v106.clone(), match v106 {
                        Adt28::C0(_, h404, _, _, _) => {
                            h404
                        }
                    }, v104, v3)
                }
                Err(v107) => {
                    Ok(Err::<Adt28, Bytes>(v107))
                }
            }
        }
        Adt17::C9(r455) => {
            let v108 = (*r455).clone();
            f29(v0, v1, v108, v3)
        }
        Adt17::C10(v109, v110, v111) => {
            let v112: bool = if v3 {
                if match v1.clone() {
                    Adt27::C0(_, _, _, _, h405) => {
                        h405
                    }
                } {
                    false
                } else {
                    match v1.clone() {
                        Adt27::C0(_, _, h406, _, _) => {
                            h406
                        }
                    }
                }
            } else {
                false
            };
            let m407 = v112;
            if m407 {
                Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("arithmetic: the value a fallible function returns, ")), f4(f62(match v1 {
                    Adt27::C0(_, _, _, h408, _) => {
                        h408
                    }
                }, Adt17::C10(v109, v110, v111))?, f1(Str::lit(", is not a fallible value"))))))
            } else {
                let m409 = v109;
                match m409 {
                    Adt24::C0 => {
                        let m410 = f65(v0.clone(), v110)?;
                        match m410 {
                            Ok(_) => {
                                Ok(Ok::<Adt28, Bytes>(v0))
                            }
                            Err(v114) => {
                                Ok(Err::<Adt28, Bytes>(v114))
                            }
                        }
                    }
                    Adt24::C1 => {
                        let m411 = f65(v0.clone(), v110)?;
                        match m411 {
                            Ok(_) => {
                                Ok(Ok::<Adt28, Bytes>(v0))
                            }
                            Err(v116) => {
                                Ok(Err::<Adt28, Bytes>(v116))
                            }
                        }
                    }
                    Adt24::C2 => {
                        let m412 = f44(v1, f1(Str::lit("a list match")));
                        match m412 {
                            Ok(_) => {
                                let m413 = f65(v0.clone(), v110)?;
                                match m413 {
                                    Ok(_) => {
                                        Ok(Ok::<Adt28, Bytes>(v0))
                                    }
                                    Err(v119) => {
                                        Ok(Err::<Adt28, Bytes>(v119))
                                    }
                                }
                            }
                            Err(v120) => {
                                Ok(Err::<Adt28, Bytes>(v120))
                            }
                        }
                    }
                    Adt24::C3 => {
                        let m414 = f65(v0.clone(), v110)?;
                        match m414 {
                            Ok(_) => {
                                Ok(Ok::<Adt28, Bytes>(v0))
                            }
                            Err(v122) => {
                                Ok(Err::<Adt28, Bytes>(v122))
                            }
                        }
                    }
                }
            }
        }
        Adt17::C11(_, _) => {
            Ok(Ok::<Adt28, Bytes>(v0))
        }
        Adt17::C12 => {
            Ok(Ok::<Adt28, Bytes>(v0))
        }
        Adt17::C13(_, _) => {
            Ok(Ok::<Adt28, Bytes>(v0))
        }
        Adt17::C14(_, _, _, _, _, _) => {
            Ok(Ok::<Adt28, Bytes>(v0))
        }
        Adt17::C15(_, _, _) => {
            Ok(Ok::<Adt28, Bytes>(v0))
        }
        Adt17::C16(v136, r456) => {
            let v137 = (*r456).clone();
            let v138: bool = if v3 {
                if match v1.clone() {
                    Adt27::C0(_, _, _, _, h415) => {
                        h415
                    }
                } {
                    false
                } else {
                    match v1.clone() {
                        Adt27::C0(_, _, h416, _, _) => {
                            h416
                        }
                    }
                }
            } else {
                false
            };
            let v139: Result<(), Bytes> = {
                let m417 = v138;
                if m417 {
                    f81(match v1.clone() {
                        Adt27::C0(_, _, _, h418, _) => {
                            h418
                        }
                    }, v137.clone())?
                } else {
                    Ok::<(), Bytes>(())
                }
            };
            let m419 = v139;
            match m419 {
                Ok(_) => {
                    let m420 = f64(v0, v136.clone())?;
                    match m420 {
                        Ok(v141) => {
                            let v142: Adt28 = Adt28::C0(match v141.clone() {
                                Adt28::C0(h421, _, _, _, _) => {
                                    h421
                                }
                            }, match v141.clone() {
                                Adt28::C0(_, h422, _, _, _) => {
                                    h422
                                }
                            }, f82(match v141.clone() {
                                Adt28::C0(_, _, h423, _, _) => {
                                    h423
                                }
                            }, v136.clone()), match v141.clone() {
                                Adt28::C0(_, _, _, h424, _) => {
                                    h424
                                }
                            }, match v141.clone() {
                                Adt28::C0(_, _, _, _, h425) => {
                                    h425
                                }
                            });
                            let m430 = f29(v142.clone(), Adt27::C0(match v1.clone() {
                                Adt27::C0(h426, _, _, _, _) => {
                                    h426
                                }
                            }, match v1.clone() {
                                Adt27::C0(_, h427, _, _, _) => {
                                    h427
                                }
                            }, match v1.clone() {
                                Adt27::C0(_, _, h428, _, _) => {
                                    h428
                                }
                            }, match v1 {
                                Adt27::C0(_, _, _, h429, _) => {
                                    h429
                                }
                            }, v138), v137, v138)?;
                            match m430 {
                                Ok(v143) => {
                                    let m434 = f83(match v143.clone() {
                                        Adt28::C0(_, _, h431, _, _) => {
                                            h431
                                        }
                                    }, v136, match v142.clone() {
                                        Adt28::C0(_, _, h432, _, _) => {
                                            h432
                                        }
                                    }, match v141.clone() {
                                        Adt28::C0(_, h433, _, _, _) => {
                                            h433
                                        }
                                    });
                                    match m434 {
                                        None => {
                                            let m438 = f84(match v143.clone() {
                                                Adt28::C0(_, _, _, h435, _) => {
                                                    h435
                                                }
                                            }, match v142 {
                                                Adt28::C0(_, _, _, h436, _) => {
                                                    h436
                                                }
                                            }, match v141.clone() {
                                                Adt28::C0(_, h437, _, _, _) => {
                                                    h437
                                                }
                                            });
                                            match m438 {
                                                None => {
                                                    Ok(Ok::<Adt28, Bytes>(Adt28::C0(match v143.clone() {
                                                        Adt28::C0(h439, _, _, _, _) => {
                                                            h439
                                                        }
                                                    }, match v143.clone() {
                                                        Adt28::C0(_, h440, _, _, _) => {
                                                            h440
                                                        }
                                                    }, match v141.clone() {
                                                        Adt28::C0(_, _, h441, _, _) => {
                                                            h441
                                                        }
                                                    }, match v141 {
                                                        Adt28::C0(_, _, _, h442, _) => {
                                                            h442
                                                        }
                                                    }, match v143 {
                                                        Adt28::C0(_, _, _, _, h443) => {
                                                            h443
                                                        }
                                                    })))
                                                }
                                                Some(v145) => {
                                                    Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("ownership: a component of ")), f4(f30(f31(v145)?), f1(Str::lit(" is moved in the body of a loop that runs again"))))))
                                                }
                                            }
                                        }
                                        Some(v144) => {
                                            Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("ownership: ")), f4(f30(f31(v144)?), f1(Str::lit(" is moved in the body of a loop that runs again"))))))
                                        }
                                    }
                                }
                                Err(v146) => {
                                    Ok(Err::<Adt28, Bytes>(v146))
                                }
                            }
                        }
                        Err(v147) => {
                            Ok(Err::<Adt28, Bytes>(v147))
                        }
                    }
                }
                Err(v148) => {
                    Ok(Err::<Adt28, Bytes>(v148))
                }
            }
        }
    }
}

pub fn f48(p463: List<(Adt12, u64)>, v1: Adt26, p464: Adt17) -> R<Result<List<(Adt12, u64)>, Bytes>> {
    let m462 = (p463, p464);
    {
        let mut m462 = m462;
        loop {
            let step = {
                let (v0, v2) = m462;
                let m457 = v2;
                match m457 {
                    Adt17::C0(_, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C1(_, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C2(_, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C3(_, _, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C4(_, _, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C5(_, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C6(_, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C7(_, _, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C8(_, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C9(_) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C10(_, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C11(_, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C12 => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C13(_, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C14(v40, v41, v42, r460, _, r461) => {
                        let v43 = (*r460).clone();
                        let v45 = (*r461).clone();
                        let v46: List<(Adt12, u64)> = {
                            let m458 = v41;
                            if m458 {
                                f50(v0, v40, v42)
                            } else {
                                v0
                            }
                        };
                        let m459 = f49(v46, v1.clone(), v43, false)?;
                        match m459 {
                            Ok(v47) => {
                                Ok::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>((v47, v45))
                            }
                            Err(v48) => {
                                Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Err::<List<(Adt12, u64)>, Bytes>(v48)))
                            }
                        }
                    }
                    Adt17::C15(_, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C16(_, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                }
            };
            match step {
                Ok(next) => { m462 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f49(v0: List<(Adt12, u64)>, v1: Adt26, v2: Adt17, v3: bool) -> R<Result<List<(Adt12, u64)>, Bytes>> {
    let m465 = v2;
    match m465 {
        Adt17::C0(_, _) => {
            Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0))
        }
        Adt17::C1(_, _, _) => {
            Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0))
        }
        Adt17::C2(_, r475, _) => {
            let v10 = (*r475).clone();
            f49(v0, v1, v10, false)
        }
        Adt17::C3(_, r476, _, _) => {
            let v13 = (*r476).clone();
            f85(v0, v1, v13)
        }
        Adt17::C4(v16, r477, v18, _) => {
            let v17 = (*r477).clone();
            let m466 = f86(v0.clone(), v16.clone());
            match m466 {
                None => {
                    Ok(Err::<List<(Adt12, u64)>, Bytes>(f4(f30(f31(v16)?), f1(Str::lit(" is applied without a function type")))))
                }
                Some(v20) => {
                    let m468 = f20(match v1.clone() {
                        Adt26::C0(_, _, h467, _) => {
                            h467
                        }
                    }, v20);
                    match m468 {
                        None => {
                            Ok(Err::<List<(Adt12, u64)>, Bytes>(f4(f1(Str::lit("Fn")), f4(f10(v20)?, f1(Str::lit("::apply is not declared"))))))
                        }
                        Some(v21) => {
                            let v22: bool = if v21 {
                                !v18
                            } else {
                                v18
                            };
                            let v23: bool = if v3 {
                                v21
                            } else {
                                false
                            };
                            let m469 = v22;
                            if m469 {
                                let m470 = v23;
                                if m470 {
                                    f85(v0, v1, v17)
                                } else {
                                    Ok(Err::<List<(Adt12, u64)>, Bytes>(f4(f1(Str::lit("arithmetic: an application of Fn")), f4(f10(v20)?, f4(f1(Str::lit(" ")), f4(if v18 {
                                        f1(Str::lit("propagates"))
                                    } else {
                                        f1(Str::lit("drops"))
                                    }, f4(f1(Str::lit(" a failure its apply ")), if v21 {
                                        f1(Str::lit("can have"))
                                    } else {
                                        f1(Str::lit("cannot have"))
                                    })))))))
                                }
                            } else {
                                f85(v0, v1, v17)
                            }
                        }
                    }
                }
            }
        }
        Adt17::C5(_, r478, _) => {
            let v25 = (*r478).clone();
            f85(v0, v1, v25)
        }
        Adt17::C6(r479, r480, _) => {
            let v27 = (*r479).clone();
            let v28 = (*r480).clone();
            let m471 = f49(v0, v1.clone(), v27, false)?;
            match m471 {
                Ok(v30) => {
                    f49(v30, v1, v28, false)
                }
                Err(v31) => {
                    Ok(Err::<List<(Adt12, u64)>, Bytes>(v31))
                }
            }
        }
        Adt17::C7(r481, r482, r483, _) => {
            let v32 = (*r481).clone();
            let v33 = (*r482).clone();
            let v34 = (*r483).clone();
            let m472 = f49(v0, v1.clone(), v32, false)?;
            match m472 {
                Ok(v36) => {
                    let m473 = f32(v36, v1.clone(), v33, v3)?;
                    match m473 {
                        Ok(v37) => {
                            f32(v37, v1, v34, v3)
                        }
                        Err(v38) => {
                            Ok(Err::<List<(Adt12, u64)>, Bytes>(v38))
                        }
                    }
                }
                Err(v39) => {
                    Ok(Err::<List<(Adt12, u64)>, Bytes>(v39))
                }
            }
        }
        Adt17::C8(r484, r485, _) => {
            let v40 = (*r484).clone();
            let v41 = (*r485).clone();
            let m474 = f49(v0, v1.clone(), v40, false)?;
            match m474 {
                Ok(v43) => {
                    f87(v43, v1, v41, v3)
                }
                Err(v44) => {
                    Ok(Err::<List<(Adt12, u64)>, Bytes>(v44))
                }
            }
        }
        Adt17::C9(r486) => {
            let v45 = (*r486).clone();
            f32(v0, v1, v45, v3)
        }
        Adt17::C10(_, _, _) => {
            Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0))
        }
        Adt17::C11(_, _) => {
            Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0))
        }
        Adt17::C12 => {
            Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0))
        }
        Adt17::C13(_, _) => {
            Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0))
        }
        Adt17::C14(_, _, _, _, _, _) => {
            Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0))
        }
        Adt17::C15(_, _, _) => {
            Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0))
        }
        Adt17::C16(_, r487) => {
            let v63 = (*r487).clone();
            f32(v0, v1, v63, false)
        }
    }
}

pub fn f50(v0: List<(Adt12, u64)>, v1: Adt14, v2: Adt2) -> List<(Adt12, u64)> {
    let m488 = f88(v1);
    match m488 {
        None => {
            v0
        }
        Some(v3) => {
            let m489 = f89(v2);
            match m489 {
                None => {
                    v0
                }
                Some(v4) => {
                    List::cons((v3, v4), v0)
                }
            }
        }
    }
}

pub fn f51(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f90(v0, v1, 48u8) {
        Ok(Some({
            let a490 = v1;
            let a491 = 1u64;
            nat_add(a490, a491)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f52(v0: Adt3) -> bool {
    let m492 = v0;
    match m492 {
        Adt3::C0 => {
            false
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
            true
        }
        Adt3::C6 => {
            true
        }
        Adt3::C7 => {
            false
        }
        Adt3::C8 => {
            false
        }
    }
}

pub fn f53(v0: Adt5) -> bool {
    let m493 = v0;
    match m493 {
        Adt5::C0 => {
            false
        }
        Adt5::C1 => {
            true
        }
        Adt5::C2 => {
            true
        }
        Adt5::C3 => {
            false
        }
        Adt5::C4 => {
            false
        }
    }
}

pub fn f54(v0: Adt7) -> bool {
    let m494 = v0;
    match m494 {
        Adt7::C0 => {
            false
        }
        Adt7::C1 => {
            false
        }
        Adt7::C2 => {
            true
        }
    }
}

pub fn f55(v0: Fn3, p498: bool, p499: List<Adt11>) -> bool {
    let m497 = (p498, p499);
    {
        let mut m497 = m497;
        loop {
            let step = {
                let (v1, v2) = m497;
                let m495 = v2;
                match m495.uncons() {
                    None => {
                        Err::<(bool, List<Adt11>), bool>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<Adt11>), bool>(({
                            let c496: Fn3 = v0.clone();
                            c496.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m497 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f56(v0: bool, v1: Adt11) -> bool {
    let m500 = v1;
    match m500 {
        Adt11::C0 => {
            v0
        }
        Adt11::C1 => {
            v0
        }
        Adt11::C2 => {
            true
        }
        Adt11::C3 => {
            v0
        }
    }
}

pub fn f57(v0: List<Adt12>, v1: Adt12) -> bool {
    f91(Fn4::F92(v1), false, v0)
}

pub fn f58(v0: Adt26, v1: u64, v2: u64) -> Option<u64> {
    let m502 = f18(match v0 {
        Adt26::C0(h501, _, _, _) => {
            h501
        }
    }, v1);
    match m502 {
        None => {
            None::<u64>
        }
        Some(v3) => {
            f93(v3, v2)
        }
    }
}

pub fn f59(v0: Adt14) -> R<u64> {
    let m503 = v0;
    match m503 {
        Adt14::C0(_) => {
            Ok(0u64)
        }
        Adt14::C1(_) => {
            Ok(0u64)
        }
        Adt14::C2(_) => {
            Ok(0u64)
        }
        Adt14::C3(_, _) => {
            Ok(0u64)
        }
        Adt14::C4(_, _, _) => {
            Ok(0u64)
        }
        Adt14::C5 => {
            Ok(0u64)
        }
        Adt14::C6(_, r506) => {
            let v10 = (*r506).clone();
            let v11: u64 = f59(v10)?;
            let a504 = 1u64;
            let a505 = v11;
            nat_add(a504, a505)
        }
    }
}

pub fn f60(v0: Adt14, v1: Adt17) -> Option<Adt12> {
    let m507 = v0;
    match m507 {
        Adt14::C0(_) => {
            None::<Adt12>
        }
        Adt14::C1(_) => {
            None::<Adt12>
        }
        Adt14::C2(r511) => {
            let v4 = (*r511).clone();
            let m508 = v1;
            match m508 {
                Adt17::C0(_, _) => {
                    None::<Adt12>
                }
                Adt17::C1(v7, v8, _) => {
                    let m509 = f94(v7);
                    if m509 {
                        let m510 = f95(v4);
                        if m510 {
                            Some(v8)
                        } else {
                            None::<Adt12>
                        }
                    } else {
                        None::<Adt12>
                    }
                }
                Adt17::C2(_, _, _) => {
                    None::<Adt12>
                }
                Adt17::C3(_, _, _, _) => {
                    None::<Adt12>
                }
                Adt17::C4(_, _, _, _) => {
                    None::<Adt12>
                }
                Adt17::C5(_, _, _) => {
                    None::<Adt12>
                }
                Adt17::C6(_, _, _) => {
                    None::<Adt12>
                }
                Adt17::C7(_, _, _, _) => {
                    None::<Adt12>
                }
                Adt17::C8(_, _, _) => {
                    None::<Adt12>
                }
                Adt17::C9(_) => {
                    None::<Adt12>
                }
                Adt17::C10(_, _, _) => {
                    None::<Adt12>
                }
                Adt17::C11(_, _) => {
                    None::<Adt12>
                }
                Adt17::C12 => {
                    None::<Adt12>
                }
                Adt17::C13(_, _) => {
                    None::<Adt12>
                }
                Adt17::C14(_, _, _, _, _, _) => {
                    None::<Adt12>
                }
                Adt17::C15(_, _, _) => {
                    None::<Adt12>
                }
                Adt17::C16(_, _) => {
                    None::<Adt12>
                }
            }
        }
        Adt14::C3(_, _) => {
            None::<Adt12>
        }
        Adt14::C4(_, _, _) => {
            None::<Adt12>
        }
        Adt14::C5 => {
            None::<Adt12>
        }
        Adt14::C6(_, _) => {
            None::<Adt12>
        }
    }
}

pub fn f61(v0: Adt28, v1: Adt12, v2: Adt14) -> R<Result<Adt28, Bytes>> {
    let m513 = f57(match v0.clone() {
        Adt28::C0(_, h512, _, _, _) => {
            h512
        }
    }, v1.clone());
    if m513 {
        let m515 = f57(match v0.clone() {
            Adt28::C0(_, _, h514, _, _) => {
                h514
            }
        }, v1.clone());
        if m515 {
            Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("ownership: a component of ")), f4(f30(f31(v1)?), f1(Str::lit(" is moved after it was moved"))))))
        } else {
            let v4: List<u64> = {
                let m517 = f96(match v0.clone() {
                    Adt28::C0(_, _, _, h516, _) => {
                        h516
                    }
                }, v1.clone());
                match m517 {
                    None => {
                        List::<u64>::nil()
                    }
                    Some(v3) => {
                        v3
                    }
                }
            };
            let m518 = f97(v4, 0u64, v2, f30(f31(v1.clone())?))?;
            match m518 {
                Ok(v5) => {
                    Ok(Ok::<Adt28, Bytes>(Adt28::C0(match v0.clone() {
                        Adt28::C0(h519, _, _, _, _) => {
                            h519
                        }
                    }, match v0.clone() {
                        Adt28::C0(_, h520, _, _, _) => {
                            h520
                        }
                    }, match v0.clone() {
                        Adt28::C0(_, _, h521, _, _) => {
                            h521
                        }
                    }, f98(match v0.clone() {
                        Adt28::C0(_, _, _, h522, _) => {
                            h522
                        }
                    }, v1, v5), match v0 {
                        Adt28::C0(_, _, _, _, h523) => {
                            h523
                        }
                    })))
                }
                Err(v6) => {
                    Ok(Err::<Adt28, Bytes>(v6))
                }
            }
        }
    } else {
        Ok(Err::<Adt28, Bytes>(f4(f30(f31(v1)?), f1(Str::lit(" is not bound")))))
    }
}

pub fn f62(v0: List<Adt25>, v1: Adt17) -> R<Bytes> {
    Ok(f99({
        let m525 = f100(Adt30::C0(List::<Adt29>::nil(), List::<Bytes>::nil(), v0), v1)?;
        match m525 {
            Adt30::C0(h524, _, _) => {
                h524
            }
        }
    }))
}

pub fn f63(v0: Adt27, v1: Adt28, v2: Bytes) -> Result<Adt28, Bytes> {
    let m527 = match v0 {
        Adt27::C0(h526, _, _, _, _) => {
            h526
        }
    };
    if m527 {
        Err::<Adt28, Bytes>(f4(f1(Str::lit("hidden allocation: rust-core renders ")), f4(v2, f1(Str::lit(", which requires heap allocation")))))
    } else {
        Ok::<Adt28, Bytes>(v1)
    }
}

pub fn f64(v0: Adt28, v1: Adt12) -> R<Result<Adt28, Bytes>> {
    let m529 = f57(match v0.clone() {
        Adt28::C0(_, h528, _, _, _) => {
            h528
        }
    }, v1.clone());
    if m529 {
        let m531 = f101(match v0.clone() {
            Adt28::C0(_, _, _, h530, _) => {
                h530
            }
        }, v1.clone());
        if m531 {
            Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("ownership: ")), f4(f30(f31(v1)?), f1(Str::lit(" is moved after a component was moved out of it"))))))
        } else {
            let m533 = f57(match v0.clone() {
                Adt28::C0(_, _, h532, _, _) => {
                    h532
                }
            }, v1.clone());
            if m533 {
                Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("ownership: ")), f4(f30(f31(v1)?), f1(Str::lit(" is moved twice"))))))
            } else {
                Ok(Ok::<Adt28, Bytes>(Adt28::C0(match v0.clone() {
                    Adt28::C0(h534, _, _, _, _) => {
                        h534
                    }
                }, match v0.clone() {
                    Adt28::C0(_, h535, _, _, _) => {
                        h535
                    }
                }, List::cons(v1, match v0.clone() {
                    Adt28::C0(_, _, h536, _, _) => {
                        h536
                    }
                }), match v0.clone() {
                    Adt28::C0(_, _, _, h537, _) => {
                        h537
                    }
                }, match v0 {
                    Adt28::C0(_, _, _, _, h538) => {
                        h538
                    }
                })))
            }
        }
    } else {
        Ok(Err::<Adt28, Bytes>(f4(f30(f31(v1)?), f1(Str::lit(" is not bound")))))
    }
}

pub fn f65(v0: Adt28, v1: Adt12) -> R<Result<(), Bytes>> {
    let m540 = f57(match v0.clone() {
        Adt28::C0(_, h539, _, _, _) => {
            h539
        }
    }, v1.clone());
    if m540 {
        let m542 = f57(match v0.clone() {
            Adt28::C0(_, _, h541, _, _) => {
                h541
            }
        }, v1.clone());
        if m542 {
            Ok(Err::<(), Bytes>(f4(f1(Str::lit("ownership: ")), f4(f30(f31(v1)?), f1(Str::lit(" is read after it was moved"))))))
        } else {
            let m544 = f101(match v0 {
                Adt28::C0(_, _, _, h543, _) => {
                    h543
                }
            }, v1.clone());
            if m544 {
                Ok(Err::<(), Bytes>(f4(f1(Str::lit("ownership: ")), f4(f30(f31(v1)?), f1(Str::lit(" is read after a component was moved out of it"))))))
            } else {
                Ok(Ok::<(), Bytes>(()))
            }
        }
    } else {
        Ok(Err::<(), Bytes>(f4(f30(f31(v1)?), f1(Str::lit(" is not bound")))))
    }
}

pub fn f66(v0: Adt17) -> bool {
    let m545 = v0;
    match m545 {
        Adt17::C0(_, _) => {
            false
        }
        Adt17::C1(_, _, _) => {
            false
        }
        Adt17::C2(_, _, _) => {
            false
        }
        Adt17::C3(_, _, _, _) => {
            false
        }
        Adt17::C4(_, _, _, _) => {
            false
        }
        Adt17::C5(_, _, _) => {
            false
        }
        Adt17::C6(_, _, _) => {
            false
        }
        Adt17::C7(_, _, _, _) => {
            false
        }
        Adt17::C8(_, _, _) => {
            false
        }
        Adt17::C9(_) => {
            false
        }
        Adt17::C10(v31, _, _) => {
            f102(v31)
        }
        Adt17::C11(_, _) => {
            false
        }
        Adt17::C12 => {
            false
        }
        Adt17::C13(_, _) => {
            false
        }
        Adt17::C14(_, _, _, _, _, _) => {
            false
        }
        Adt17::C15(_, _, _) => {
            false
        }
        Adt17::C16(_, _) => {
            false
        }
    }
}

pub fn f67(p552: Adt28, v1: Adt27, v2: bool, p553: Adt17) -> R<Result<Adt28, Bytes>> {
    let m551 = (p552, p553);
    {
        let mut m551 = m551;
        loop {
            let step = {
                let (v0, v3) = m551;
                let m546 = v3;
                match m546 {
                    Adt17::C0(_, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C1(_, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C2(_, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C3(_, _, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C4(_, _, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C5(_, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C6(_, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C7(_, _, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C8(_, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C9(_) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C10(_, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C11(_, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C12 => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C13(r549, r550) => {
                        let v39 = (*r549).clone();
                        let v40 = (*r550).clone();
                        let v41: bool = if v2 {
                            f103(v39.clone())
                        } else {
                            false
                        };
                        let m547 = v41;
                        if m547 {
                            Ok::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>((v0, v40))
                        } else {
                            let m548 = f47(v0, v1.clone(), v39, false)?;
                            match m548 {
                                Ok(v42) => {
                                    Ok::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>((v42, v40))
                                }
                                Err(v43) => {
                                    Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Err::<Adt28, Bytes>(v43)))
                                }
                            }
                        }
                    }
                    Adt17::C14(_, _, _, _, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C15(_, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C16(_, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                }
            };
            match step {
                Ok(next) => { m551 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f68(v0: Adt28, v1: Adt27, v2: bool, v3: bool, v4: bool, v5: Bytes) -> Result<Adt28, Bytes> {
    let v6: bool = if v4 {
        match v1 {
            Adt27::C0(_, _, h554, _, _) => {
                h554
            }
        }
    } else {
        false
    };
    let m555 = v2;
    if m555 {
        let m556 = v3;
        if m556 {
            Ok::<Adt28, Bytes>(Adt28::C0(match v0.clone() {
                Adt28::C0(h557, _, _, _, _) => {
                    h557
                }
            }, match v0.clone() {
                Adt28::C0(_, h558, _, _, _) => {
                    h558
                }
            }, match v0.clone() {
                Adt28::C0(_, _, h559, _, _) => {
                    h559
                }
            }, match v0 {
                Adt28::C0(_, _, _, h560, _) => {
                    h560
                }
            }, true))
        } else {
            let m561 = v6;
            if m561 {
                Ok::<Adt28, Bytes>(v0)
            } else {
                Err::<Adt28, Bytes>(f4(f1(Str::lit("arithmetic: the fallible call of ")), f4(v5, f1(Str::lit(" does not propagate its overflow")))))
            }
        }
    } else {
        let m562 = v3;
        if m562 {
            Err::<Adt28, Bytes>(f4(f1(Str::lit("arithmetic: the infallible call of ")), f4(v5, f1(Str::lit(" propagates a failure it cannot have")))))
        } else {
            Ok::<Adt28, Bytes>(v0)
        }
    }
}

pub fn f69(v0: Adt28, v1: Adt27, v2: u64, v3: Adt17, v4: bool, v5: bool) -> R<Result<Adt28, Bytes>> {
    let m565 = {
        let a563 = f76(v3.clone())?;
        let a564 = f104(v2);
        nat_eq(a563, a564)
    };
    if m565 {
        Ok(f68(v0, v1, false, v4, v5, f30(f105(v2))))
    } else {
        Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("machine: ")), f4(f30(f105(v2)), f4(f1(Str::lit(" receives ")), f4(f10(f76(v3)?)?, f4(f1(Str::lit(" operands of ")), f10(f104(v2))?)))))))
    }
}

pub fn f70(v0: List<Adt25>, v1: Adt22) -> Bytes {
    let m567 = f106(v0, match v1.clone() {
        Adt22::C0(h566, _) => {
            h566
        }
    });
    match m567 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            let a569 = match v2.clone() {
                Adt25::C0(_, _, h568, _, _) => {
                    h568
                }
            };
            let a570 = 0u64;
            if nat_eq(a569, a570) {
                match v2 {
                    Adt25::C0(_, h571, _, _, _) => {
                        h571
                    }
                }
            } else {
                let a573 = match v2.clone() {
                    Adt25::C0(_, _, h572, _, _) => {
                        h572
                    }
                };
                let a574 = 1u64;
                if nat_eq(a573, a574) {
                    f4(f1(Str::lit("fixed_")), f4(f107(f108(match v1 {
                        Adt22::C0(_, h575) => {
                            h575
                        }
                    })), f4(f1(Str::lit("::")), match v2 {
                        Adt25::C0(_, h576, _, _, _) => {
                            h576
                        }
                    })))
                } else {
                    f4(match v2 {
                        Adt25::C0(_, h577, _, _, _) => {
                            h577
                        }
                    }, f107(f108(match v1 {
                        Adt22::C0(_, h578) => {
                            h578
                        }
                    })))
                }
            }
        }
    }
}

pub fn f71(v0: List<Adt25>, v1: Adt22) -> bool {
    let m580 = f106(v0, match v1 {
        Adt22::C0(h579, _) => {
            h579
        }
    });
    match m580 {
        None => {
            false
        }
        Some(v2) => {
            match v2 {
                Adt25::C0(_, _, _, _, h581) => {
                    h581
                }
            }
        }
    }
}

pub fn f72(v0: List<Adt25>, v1: Adt22) -> bool {
    let m583 = f106(v0, match v1 {
        Adt22::C0(h582, _) => {
            h582
        }
    });
    match m583 {
        None => {
            false
        }
        Some(v2) => {
            match v2 {
                Adt25::C0(_, _, _, h584, _) => {
                    h584
                }
            }
        }
    }
}

pub fn f73(v0: Adt23) -> bool {
    let m585 = v0;
    match m585 {
        Adt23::C0(_) => {
            false
        }
        Adt23::C1 => {
            false
        }
        Adt23::C2(_, _) => {
            false
        }
        Adt23::C3(_, _) => {
            true
        }
        Adt23::C4(_, _) => {
            false
        }
        Adt23::C5(_, _) => {
            false
        }
        Adt23::C6 => {
            false
        }
        Adt23::C7(_) => {
            false
        }
    }
}

pub fn f74(p591: Adt28, v1: Adt27, p592: Adt17) -> R<Result<Adt28, Bytes>> {
    let m590 = (p591, p592);
    {
        let mut m590 = m590;
        loop {
            let step = {
                let (v0, v2) = m590;
                let m586 = v2;
                match m586 {
                    Adt17::C0(_, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C1(_, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C2(_, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C3(_, _, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C4(_, _, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C5(_, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C6(_, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C7(_, _, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C8(_, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C9(_) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C10(_, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C11(_, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C12 => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C13(r588, r589) => {
                        let v38 = (*r588).clone();
                        let v39 = (*r589).clone();
                        let m587 = f47(v0, v1.clone(), v38, true)?;
                        match m587 {
                            Ok(v40) => {
                                Ok::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>((v40, v39))
                            }
                            Err(v41) => {
                                Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Err::<Adt28, Bytes>(v41)))
                            }
                        }
                    }
                    Adt17::C14(_, _, _, _, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C15(_, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C16(_, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                }
            };
            match step {
                Ok(next) => { m590 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f75(v0: Adt27, v1: Adt23, v2: u64) -> R<Result<(), Bytes>> {
    let m593 = v1;
    match m593 {
        Adt23::C0(v3) => {
            f26(v0, v3)
        }
        Adt23::C1 => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt23::C2(v4, v5) => {
            let m594 = f26(v0.clone(), v4)?;
            match m594 {
                Ok(_) => {
                    f26(v0, v5)
                }
                Err(v7) => {
                    Ok(Err::<(), Bytes>(v7))
                }
            }
        }
        Adt23::C3(v8, v9) => {
            let m595 = f26(v0.clone(), v8)?;
            match m595 {
                Ok(_) => {
                    f26(v0, v9)
                }
                Err(v11) => {
                    Ok(Err::<(), Bytes>(v11))
                }
            }
        }
        Adt23::C4(v12, v13) => {
            let m597 = f58(match v0 {
                Adt27::C0(_, h596, _, _, _) => {
                    h596
                }
            }, v12, v13);
            match m597 {
                None => {
                    Ok(Err::<(), Bytes>(f4(f1(Str::lit("Adt")), f4(f10(v12)?, f4(f1(Str::lit("::C")), f4(f10(v13)?, f1(Str::lit(" is not declared"))))))))
                }
                Some(v14) => {
                    let m600 = {
                        let a598 = v14;
                        let a599 = v2;
                        nat_eq(a598, a599)
                    };
                    if m600 {
                        Ok(Ok::<(), Bytes>(()))
                    } else {
                        Ok(Err::<(), Bytes>(f4(f1(Str::lit("Adt")), f4(f10(v12)?, f4(f1(Str::lit("::C")), f4(f10(v13)?, f4(f1(Str::lit(" receives ")), f4(f10(v2)?, f4(f1(Str::lit(" fields of ")), f10(v14)?)))))))))
                    }
                }
            }
        }
        Adt23::C5(v15, v16) => {
            let m602 = f109(match v0 {
                Adt27::C0(_, h601, _, _, _) => {
                    h601
                }
            }, v15, v16);
            match m602 {
                None => {
                    Ok(Err::<(), Bytes>(f4(f1(Str::lit("Fn")), f4(f10(v15)?, f4(f1(Str::lit("::F")), f4(f10(v16)?, f1(Str::lit(" is not declared"))))))))
                }
                Some(v17) => {
                    let m605 = {
                        let a603 = v17;
                        let a604 = v2;
                        nat_eq(a603, a604)
                    };
                    if m605 {
                        Ok(Ok::<(), Bytes>(()))
                    } else {
                        Ok(Err::<(), Bytes>(f4(f1(Str::lit("Fn")), f4(f10(v15)?, f4(f1(Str::lit("::F")), f4(f10(v16)?, f4(f1(Str::lit(" captures ")), f4(f10(v2)?, f4(f1(Str::lit(" values of ")), f10(v17)?)))))))))
                    }
                }
            }
        }
        Adt23::C6 => {
            Ok(f44(v0, f1(Str::lit("a list cell"))))
        }
        Adt23::C7(v18) => {
            let m606 = f44(v0.clone(), f1(Str::lit("an empty list")));
            match m606 {
                Ok(_) => {
                    f26(v0, v18)
                }
                Err(v20) => {
                    Ok(Err::<(), Bytes>(v20))
                }
            }
        }
    }
}

pub fn f76(v0: Adt17) -> R<u64> {
    let m607 = v0;
    match m607 {
        Adt17::C0(_, _) => {
            Ok(0u64)
        }
        Adt17::C1(_, _, _) => {
            Ok(0u64)
        }
        Adt17::C2(_, _, _) => {
            Ok(0u64)
        }
        Adt17::C3(_, _, _, _) => {
            Ok(0u64)
        }
        Adt17::C4(_, _, _, _) => {
            Ok(0u64)
        }
        Adt17::C5(_, _, _) => {
            Ok(0u64)
        }
        Adt17::C6(_, _, _) => {
            Ok(0u64)
        }
        Adt17::C7(_, _, _, _) => {
            Ok(0u64)
        }
        Adt17::C8(_, _, _) => {
            Ok(0u64)
        }
        Adt17::C9(_) => {
            Ok(0u64)
        }
        Adt17::C10(_, _, _) => {
            Ok(0u64)
        }
        Adt17::C11(_, _) => {
            Ok(0u64)
        }
        Adt17::C12 => {
            Ok(0u64)
        }
        Adt17::C13(_, r610) => {
            let v37 = (*r610).clone();
            let v38: u64 = f76(v37)?;
            let a608 = 1u64;
            let a609 = v38;
            nat_add(a608, a609)
        }
        Adt17::C14(_, _, _, _, _, _) => {
            Ok(0u64)
        }
        Adt17::C15(_, _, _) => {
            Ok(0u64)
        }
        Adt17::C16(_, _) => {
            Ok(0u64)
        }
    }
}

pub fn f77(v0: Adt23, v1: Adt17) -> R<bool> {
    let m611 = v0;
    match m611 {
        Adt23::C0(_) => {
            Ok(false)
        }
        Adt23::C1 => {
            Ok(false)
        }
        Adt23::C2(_, _) => {
            Ok(false)
        }
        Adt23::C3(_, v6) => {
            let v7: bool = f110(v6);
            let v8: bool = f111(v1)?;
            if v7 {
                Ok(v8)
            } else {
                Ok(false)
            }
        }
        Adt23::C4(_, _) => {
            Ok(false)
        }
        Adt23::C5(_, _) => {
            Ok(false)
        }
        Adt23::C6 => {
            Ok(false)
        }
        Adt23::C7(_) => {
            Ok(false)
        }
    }
}

pub fn f78(v0: List<Adt12>, v1: List<Adt12>) -> List<Adt12> {
    f112(Fn5::F113, v0, v1)
}

pub fn f79(v0: List<(Adt12, List<u64>)>, v1: List<(Adt12, List<u64>)>) -> List<(Adt12, List<u64>)> {
    f114(Fn6::F115, v0, v1)
}

pub fn f80(v0: Adt28, v1: Adt27, p629: Adt28, v3: List<Adt12>, p630: Adt17, v5: bool) -> R<Result<Adt28, Bytes>> {
    let m628 = (p629, p630);
    {
        let mut m628 = m628;
        loop {
            let step = {
                let (v2, v4) = m628;
                let m612 = v4;
                match m612 {
                    Adt17::C0(_, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v2)))
                    }
                    Adt17::C1(_, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v2)))
                    }
                    Adt17::C2(_, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v2)))
                    }
                    Adt17::C3(_, _, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v2)))
                    }
                    Adt17::C4(_, _, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v2)))
                    }
                    Adt17::C5(_, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v2)))
                    }
                    Adt17::C6(_, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v2)))
                    }
                    Adt17::C7(_, _, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v2)))
                    }
                    Adt17::C8(_, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v2)))
                    }
                    Adt17::C9(_) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v2)))
                    }
                    Adt17::C10(_, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v2)))
                    }
                    Adt17::C11(_, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v2)))
                    }
                    Adt17::C12 => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v2)))
                    }
                    Adt17::C13(_, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v2)))
                    }
                    Adt17::C14(_, _, _, _, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v2)))
                    }
                    Adt17::C15(v49, r626, r627) => {
                        let v50 = (*r626).clone();
                        let v51 = (*r627).clone();
                        let v52: Adt28 = Adt28::C0(match v2.clone() {
                            Adt28::C0(h613, _, _, _, _) => {
                                h613
                            }
                        }, v3.clone(), match v0.clone() {
                            Adt28::C0(_, _, h614, _, _) => {
                                h614
                            }
                        }, match v0.clone() {
                            Adt28::C0(_, _, _, h615, _) => {
                                h615
                            }
                        }, match v2.clone() {
                            Adt28::C0(_, _, _, _, h616) => {
                                h616
                            }
                        });
                        let m618 = f45(v52, match v1.clone() {
                            Adt27::C0(_, h617, _, _, _) => {
                                h617
                            }
                        }, v49)?;
                        match m618 {
                            Ok(v53) => {
                                let m619 = f29(v53, v1.clone(), v50, v5)?;
                                match m619 {
                                    Ok(v54) => {
                                        let v55: Adt28 = Adt28::C0(match v54.clone() {
                                            Adt28::C0(h620, _, _, _, _) => {
                                                h620
                                            }
                                        }, v3.clone(), f78(match v2.clone() {
                                            Adt28::C0(_, _, h621, _, _) => {
                                                h621
                                            }
                                        }, match v54.clone() {
                                            Adt28::C0(_, _, h622, _, _) => {
                                                h622
                                            }
                                        }), f79(match v2 {
                                            Adt28::C0(_, _, _, h623, _) => {
                                                h623
                                            }
                                        }, match v54.clone() {
                                            Adt28::C0(_, _, _, h624, _) => {
                                                h624
                                            }
                                        }), match v54 {
                                            Adt28::C0(_, _, _, _, h625) => {
                                                h625
                                            }
                                        });
                                        Ok::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>((v55, v51))
                                    }
                                    Err(v56) => {
                                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Err::<Adt28, Bytes>(v56)))
                                    }
                                }
                            }
                            Err(v57) => {
                                Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Err::<Adt28, Bytes>(v57)))
                            }
                        }
                    }
                    Adt17::C16(_, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v2)))
                    }
                }
            };
            match step {
                Ok(next) => { m628 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f81(v0: List<Adt25>, v1: Adt17) -> R<Result<(), Bytes>> {
    let m631 = v1;
    match m631 {
        Adt17::C0(_, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C1(_, _, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C2(_, _, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C3(_, _, _, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C4(_, _, _, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C5(_, _, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C6(_, _, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C7(_, _, _, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C8(_, _, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C9(_) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C10(_, _, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C11(_, r632) => {
            let v36 = (*r632).clone();
            f116(v0, v36)
        }
        Adt17::C12 => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C13(_, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C14(_, _, _, _, _, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C15(_, _, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C16(_, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
    }
}

pub fn f82(v0: List<Adt12>, v1: Adt12) -> List<Adt12> {
    f112(Fn5::F117(v1), List::<Adt12>::nil(), v0)
}

pub fn f83(v0: List<Adt12>, v1: Adt12, v2: List<Adt12>, v3: List<Adt12>) -> Option<Adt12> {
    f118(Fn7::F119(v2, v3, v1), None::<Adt12>, v0)
}

pub fn f84(v0: List<(Adt12, List<u64>)>, v1: List<(Adt12, List<u64>)>, v2: List<Adt12>) -> Option<Adt12> {
    f120(Fn8::F121(v1, v2), None::<Adt12>, v0)
}

pub fn f85(p638: List<(Adt12, u64)>, v1: Adt26, p639: Adt17) -> R<Result<List<(Adt12, u64)>, Bytes>> {
    let m637 = (p638, p639);
    {
        let mut m637 = m637;
        loop {
            let step = {
                let (v0, v2) = m637;
                let m633 = v2;
                match m633 {
                    Adt17::C0(_, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C1(_, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C2(_, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C3(_, _, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C4(_, _, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C5(_, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C6(_, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C7(_, _, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C8(_, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C9(_) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C10(_, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C11(_, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C12 => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C13(r635, r636) => {
                        let v38 = (*r635).clone();
                        let v39 = (*r636).clone();
                        let m634 = f49(v0, v1.clone(), v38, false)?;
                        match m634 {
                            Ok(v40) => {
                                Ok::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>((v40, v39))
                            }
                            Err(v41) => {
                                Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Err::<List<(Adt12, u64)>, Bytes>(v41)))
                            }
                        }
                    }
                    Adt17::C14(_, _, _, _, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C15(_, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C16(_, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                }
            };
            match step {
                Ok(next) => { m637 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f86(v0: List<(Adt12, u64)>, v1: Adt12) -> Option<u64> {
    f122(Fn9::F123(v1), None::<u64>, v0)
}

pub fn f87(p645: List<(Adt12, u64)>, v1: Adt26, p646: Adt17, v3: bool) -> R<Result<List<(Adt12, u64)>, Bytes>> {
    let m644 = (p645, p646);
    {
        let mut m644 = m644;
        loop {
            let step = {
                let (v0, v2) = m644;
                let m640 = v2;
                match m640 {
                    Adt17::C0(_, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C1(_, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C2(_, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C3(_, _, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C4(_, _, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C5(_, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C6(_, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C7(_, _, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C8(_, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C9(_) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C10(_, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C11(_, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C12 => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C13(_, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C14(_, _, _, _, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C15(_, r642, r643) => {
                        let v48 = (*r642).clone();
                        let v49 = (*r643).clone();
                        let m641 = f32(v0, v1.clone(), v48, v3)?;
                        match m641 {
                            Ok(v50) => {
                                Ok::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>((v50, v49))
                            }
                            Err(v51) => {
                                Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Err::<List<(Adt12, u64)>, Bytes>(v51)))
                            }
                        }
                    }
                    Adt17::C16(_, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                }
            };
            match step {
                Ok(next) => { m644 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f88(v0: Adt14) -> Option<Adt12> {
    let m647 = v0;
    match m647 {
        Adt14::C0(_) => {
            None::<Adt12>
        }
        Adt14::C1(v2) => {
            Some(v2)
        }
        Adt14::C2(_) => {
            None::<Adt12>
        }
        Adt14::C3(_, _) => {
            None::<Adt12>
        }
        Adt14::C4(_, _, _) => {
            None::<Adt12>
        }
        Adt14::C5 => {
            None::<Adt12>
        }
        Adt14::C6(_, _) => {
            None::<Adt12>
        }
    }
}

pub fn f89(v0: Adt2) -> Option<u64> {
    let m648 = v0;
    match m648 {
        Adt2::C0(_) => {
            None::<u64>
        }
        Adt2::C1(_) => {
            None::<u64>
        }
        Adt2::C2(_, _) => {
            None::<u64>
        }
        Adt2::C3(_, _, _) => {
            None::<u64>
        }
        Adt2::C4(v8, v9) => {
            let m649 = f6(v8);
            if m649 {
                Some(v9)
            } else {
                None::<u64>
            }
        }
        Adt2::C5 => {
            None::<u64>
        }
        Adt2::C6(_, _) => {
            None::<u64>
        }
    }
}

pub fn f90(v0: Bytes, v1: u64, v2: u8) -> bool {
    let m650 = f124(v0, v1);
    match m650 {
        None => {
            false
        }
        Some(v3) => {
            let a651 = v3;
            let a652 = v2;
            equal(a651, a652)
        }
    }
}

pub fn f91(v0: Fn4, p656: bool, p657: List<Adt12>) -> bool {
    let m655 = (p656, p657);
    {
        let mut m655 = m655;
        loop {
            let step = {
                let (v1, v2) = m655;
                let m653 = v2;
                match m653.uncons() {
                    None => {
                        Err::<(bool, List<Adt12>), bool>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<Adt12>), bool>(({
                            let c654: Fn4 = v0.clone();
                            c654.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m655 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f92(v0: Adt12, v1: bool, v2: Adt12) -> bool {
    let v3: bool = f125(v2, v0);
    if v1 {
        true
    } else {
        v3
    }
}

pub fn f93(v0: List<(u64, u64)>, v1: u64) -> Option<u64> {
    f126(Fn10::F127(v1), None::<u64>, v0)
}

pub fn f94(v0: Adt19) -> bool {
    let m658 = v0;
    match m658 {
        Adt19::C0 => {
            true
        }
        Adt19::C1 => {
            false
        }
        Adt19::C2 => {
            false
        }
        Adt19::C3 => {
            false
        }
        Adt19::C4 => {
            false
        }
    }
}

pub fn f95(v0: Adt14) -> bool {
    let m659 = v0;
    match m659 {
        Adt14::C0(_) => {
            false
        }
        Adt14::C1(_) => {
            false
        }
        Adt14::C2(_) => {
            false
        }
        Adt14::C3(_, _) => {
            false
        }
        Adt14::C4(_, _, _) => {
            false
        }
        Adt14::C5 => {
            false
        }
        Adt14::C6(r660, r661) => {
            let v9 = (*r660).clone();
            let v10 = (*r661).clone();
            let v11: bool = f128(v9);
            let v12: bool = f95(v10);
            if v11 {
                true
            } else {
                v12
            }
        }
    }
}

pub fn f96(v0: List<(Adt12, List<u64>)>, v1: Adt12) -> Option<List<u64>> {
    f129(Fn11::F130(v1), None::<List<u64>>, v0)
}

pub fn f97(p672: List<u64>, p673: u64, p674: Adt14, v3: Bytes) -> R<Result<List<u64>, Bytes>> {
    let m671 = (p672, (p673, p674));
    {
        let mut m671 = m671;
        loop {
            let step = {
                let (v0, (v1, v2)) = m671;
                let m662 = v2;
                match m662 {
                    Adt14::C0(_) => {
                        Err::<(List<u64>, (u64, Adt14)), R<Result<List<u64>, Bytes>>>(Ok(Ok::<List<u64>, Bytes>(v0)))
                    }
                    Adt14::C1(_) => {
                        Err::<(List<u64>, (u64, Adt14)), R<Result<List<u64>, Bytes>>>(Ok(Ok::<List<u64>, Bytes>(v0)))
                    }
                    Adt14::C2(_) => {
                        Err::<(List<u64>, (u64, Adt14)), R<Result<List<u64>, Bytes>>>(Ok(Ok::<List<u64>, Bytes>(v0)))
                    }
                    Adt14::C3(_, _) => {
                        Err::<(List<u64>, (u64, Adt14)), R<Result<List<u64>, Bytes>>>(Ok(Ok::<List<u64>, Bytes>(v0)))
                    }
                    Adt14::C4(_, _, _) => {
                        Err::<(List<u64>, (u64, Adt14)), R<Result<List<u64>, Bytes>>>(Ok(Ok::<List<u64>, Bytes>(v0)))
                    }
                    Adt14::C5 => {
                        Err::<(List<u64>, (u64, Adt14)), R<Result<List<u64>, Bytes>>>(Ok(Ok::<List<u64>, Bytes>(v0)))
                    }
                    Adt14::C6(r669, r670) => {
                        let v12 = (*r669).clone();
                        let v13 = (*r670).clone();
                        let v14: bool = f128(v12);
                        let m663 = v14;
                        if m663 {
                            Ok::<(List<u64>, (u64, Adt14)), R<Result<List<u64>, Bytes>>>((v0, ({
                                let a664 = v1;
                                let a665 = 1u64;
                                nat_add(a664, a665)?
                            }, v13)))
                        } else {
                            let m666 = f131(v0.clone(), v1);
                            if m666 {
                                Err::<(List<u64>, (u64, Adt14)), R<Result<List<u64>, Bytes>>>(Ok(Err::<List<u64>, Bytes>(f4(f1(Str::lit("ownership: component ")), f4(f10(v1)?, f4(f1(Str::lit(" of ")), f4(v3.clone(), f1(Str::lit(" is moved twice")))))))))
                            } else {
                                Ok::<(List<u64>, (u64, Adt14)), R<Result<List<u64>, Bytes>>>((List::cons(v1, v0), ({
                                    let a667 = v1;
                                    let a668 = 1u64;
                                    nat_add(a667, a668)?
                                }, v13)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m671 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f98(v0: List<(Adt12, List<u64>)>, v1: Adt12, v2: List<u64>) -> List<(Adt12, List<u64>)> {
    List::cons((v1.clone(), v2), f132(v0, v1))
}

pub fn f99(v0: List<Adt29>) -> Bytes {
    let m675 = f133(Fn12::F134, None::<Bytes>, v0);
    match m675 {
        None => {
            f1(Str::lit("a read"))
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f100(v0: Adt30, v1: Adt17) -> R<Adt30> {
    let m676 = v1;
    match m676 {
        Adt17::C0(v2, v3) => {
            let m677 = f135(v2.clone());
            if m677 {
                Ok(v0)
            } else {
                Ok(f136(v0, f137(v2.clone()), v3, f138(v2)))
            }
        }
        Adt17::C1(v4, _, v6) => {
            Ok(f139(v0, f140(v4), v6))
        }
        Adt17::C2(v7, r686, v9) => {
            let v8 = (*r686).clone();
            let v10: Adt30 = f139(v0, f141(v7), v9);
            f100(v10, v8)
        }
        Adt17::C3(v11, r687, _, v14) => {
            let v12 = (*r687).clone();
            let v18: Adt30 = {
                let m678 = v11;
                match m678 {
                    Adt21::C0(_) => {
                        f139(v0, f1(Str::lit("call:function")), v14)
                    }
                    Adt21::C1(v17) => {
                        f136(v0.clone(), f4(f1(Str::lit("call:runtime:")), f142(f143(f70(match v0.clone() {
                            Adt30::C0(_, _, h679) => {
                                h679
                            }
                        }, v17.clone()))?)), v14, f144(match v0 {
                            Adt30::C0(_, _, h680) => {
                                h680
                            }
                        }, v17))
                    }
                    Adt21::C2(_) => {
                        f139(v0, f1(Str::lit("call:machine")), v14)
                    }
                }
            };
            f100(v18, v12)
        }
        Adt17::C4(_, r688, _, v22) => {
            let v20 = (*r688).clone();
            let v23: Adt30 = f139(v0, f1(Str::lit("apply")), v22);
            f100(v23, v20)
        }
        Adt17::C5(v24, r689, v26) => {
            let v25 = (*r689).clone();
            let v27: Adt30 = f139(f145(v0, v24.clone()), f4(f1(Str::lit("construct:")), f146(v24)), v26);
            f100(v27, v25)
        }
        Adt17::C6(r690, r691, v30) => {
            let v28 = (*r690).clone();
            let v29 = (*r691).clone();
            let v31: Adt30 = f139(v0, f1(Str::lit("construct:pair")), v30);
            let v32: Adt30 = f100(v31, v28)?;
            f100(v32, v29)
        }
        Adt17::C7(r692, r693, r694, v36) => {
            let v33 = (*r692).clone();
            let v34 = (*r693).clone();
            let v35 = (*r694).clone();
            let v37: Adt30 = f139(v0, f1(Str::lit("if")), v36);
            let v38: Adt30 = f100(v37, v33)?;
            let v39: Adt30 = f100(v38, v34)?;
            f100(v39, v35)
        }
        Adt17::C8(r695, r696, v42) => {
            let v40 = (*r695).clone();
            let v41 = (*r696).clone();
            let v43: Adt30 = {
                let m681 = f147(v41.clone());
                if m681 {
                    f139(v0, f1(Str::lit("match:empty")), v42)
                } else {
                    let m682 = f148(v40.clone());
                    if m682 {
                        f139(v0, f1(Str::lit("match:list")), v42)
                    } else {
                        f149(v0, v41.clone(), v42)
                    }
                }
            };
            let v44: Adt30 = f100(v43, v40)?;
            f100(v44, v41)
        }
        Adt17::C9(r697) => {
            let v45 = (*r697).clone();
            f100(v0, v45)
        }
        Adt17::C10(v46, _, v48) => {
            let m683 = f150(v46);
            if m683 {
                Ok(f139(v0, f1(Str::lit("match:nat")), v48))
            } else {
                Ok(v0)
            }
        }
        Adt17::C11(r698, r699) => {
            let v49 = (*r698).clone();
            let v50 = (*r699).clone();
            let v51: Adt30 = f100(v0, v49)?;
            f100(v51, v50)
        }
        Adt17::C12 => {
            Ok(v0)
        }
        Adt17::C13(r700, r701) => {
            let v52 = (*r700).clone();
            let v53 = (*r701).clone();
            let v54: Adt30 = f100(v0, v52)?;
            f100(v54, v53)
        }
        Adt17::C14(v55, v56, v57, r702, v59, r703) => {
            let v58 = (*r702).clone();
            let v60 = (*r703).clone();
            let v61: Bytes = {
                let m684 = f151(v55);
                if m684 {
                    f1(Str::lit("destructure:pair"))
                } else {
                    f1(Str::lit("let"))
                }
            };
            let v62: Adt30 = {
                let m685 = v56;
                if m685 {
                    f152(v0, v57)
                } else {
                    v0
                }
            };
            let v63: Adt30 = f139(v62, v61, v59);
            let v64: Adt30 = f100(v63, v58)?;
            f100(v64, v60)
        }
        Adt17::C15(_, r704, r705) => {
            let v66 = (*r704).clone();
            let v67 = (*r705).clone();
            let v68: Adt30 = f100(v0, v66)?;
            f100(v68, v67)
        }
        Adt17::C16(_, r706) => {
            let v70 = (*r706).clone();
            f100(v0, v70)
        }
    }
}

pub fn f101(v0: List<(Adt12, List<u64>)>, v1: Adt12) -> bool {
    let m707 = f96(v0, v1);
    match m707 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f102(v0: Adt24) -> bool {
    let m708 = v0;
    match m708 {
        Adt24::C0 => {
            true
        }
        Adt24::C1 => {
            true
        }
        Adt24::C2 => {
            false
        }
        Adt24::C3 => {
            true
        }
    }
}

pub fn f103(v0: Adt17) -> bool {
    let m709 = v0;
    match m709 {
        Adt17::C0(v1, _) => {
            f135(v1)
        }
        Adt17::C1(_, _, _) => {
            false
        }
        Adt17::C2(_, _, _) => {
            false
        }
        Adt17::C3(_, _, _, _) => {
            false
        }
        Adt17::C4(_, _, _, _) => {
            false
        }
        Adt17::C5(_, _, _) => {
            false
        }
        Adt17::C6(_, _, _) => {
            false
        }
        Adt17::C7(_, _, _, _) => {
            false
        }
        Adt17::C8(_, _, _) => {
            false
        }
        Adt17::C9(_) => {
            false
        }
        Adt17::C10(_, _, _) => {
            false
        }
        Adt17::C11(_, _) => {
            false
        }
        Adt17::C12 => {
            false
        }
        Adt17::C13(_, _) => {
            false
        }
        Adt17::C14(_, _, _, _, _, _) => {
            false
        }
        Adt17::C15(_, _, _) => {
            false
        }
        Adt17::C16(_, _) => {
            false
        }
    }
}

pub fn f104(v0: u64) -> u64 {
    let a710 = v0;
    let a711 = 0u64;
    if nat_eq(a710, a711) {
        2u64
    } else {
        let a712 = v0;
        let a713 = 1u64;
        if nat_eq(a712, a713) {
            2u64
        } else {
            let a714 = v0;
            let a715 = 2u64;
            if nat_eq(a714, a715) {
                2u64
            } else {
                let a716 = v0;
                let a717 = 3u64;
                if nat_eq(a716, a717) {
                    2u64
                } else {
                    let a718 = v0;
                    let a719 = 4u64;
                    if nat_eq(a718, a719) {
                        1u64
                    } else {
                        let a720 = v0;
                        let a721 = 5u64;
                        if nat_eq(a720, a721) {
                            2u64
                        } else {
                            let a722 = v0;
                            let a723 = 6u64;
                            if nat_eq(a722, a723) {
                                2u64
                            } else {
                                let a724 = v0;
                                let a725 = 7u64;
                                if nat_eq(a724, a725) {
                                    1u64
                                } else {
                                    let a726 = v0;
                                    let a727 = 8u64;
                                    if nat_eq(a726, a727) {
                                        2u64
                                    } else {
                                        let a728 = v0;
                                        let a729 = 9u64;
                                        if nat_eq(a728, a729) {
                                            2u64
                                        } else {
                                            let a730 = v0;
                                            let a731 = 10u64;
                                            if nat_eq(a730, a731) {
                                                2u64
                                            } else {
                                                let a732 = v0;
                                                let a733 = 11u64;
                                                if nat_eq(a732, a733) {
                                                    1u64
                                                } else {
                                                    let a734 = v0;
                                                    let a735 = 12u64;
                                                    if nat_eq(a734, a735) {
                                                        2u64
                                                    } else {
                                                        let a736 = v0;
                                                        let a737 = 13u64;
                                                        if nat_eq(a736, a737) {
                                                            2u64
                                                        } else {
                                                            let a738 = v0;
                                                            let a739 = 14u64;
                                                            if nat_eq(a738, a739) {
                                                                2u64
                                                            } else {
                                                                let a740 = v0;
                                                                let a741 = 15u64;
                                                                if nat_eq(a740, a741) {
                                                                    3u64
                                                                } else {
                                                                    let a742 = v0;
                                                                    let a743 = 16u64;
                                                                    if nat_eq(a742, a743) {
                                                                        3u64
                                                                    } else {
                                                                        let a744 = v0;
                                                                        let a745 = 17u64;
                                                                        if nat_eq(a744, a745) {
                                                                            2u64
                                                                        } else {
                                                                            let a746 = v0;
                                                                            let a747 = 18u64;
                                                                            if nat_eq(a746, a747) {
                                                                                2u64
                                                                            } else {
                                                                                let a748 = v0;
                                                                                let a749 = 19u64;
                                                                                if nat_eq(a748, a749) {
                                                                                    2u64
                                                                                } else {
                                                                                    let a750 = v0;
                                                                                    let a751 = 20u64;
                                                                                    if nat_eq(a750, a751) {
                                                                                        3u64
                                                                                    } else {
                                                                                        let a752 = v0;
                                                                                        let a753 = 21u64;
                                                                                        if nat_eq(a752, a753) {
                                                                                            2u64
                                                                                        } else {
                                                                                            let a754 = v0;
                                                                                            let a755 = 22u64;
                                                                                            if nat_eq(a754, a755) {
                                                                                                2u64
                                                                                            } else {
                                                                                                let a756 = v0;
                                                                                                let a757 = 23u64;
                                                                                                if nat_eq(a756, a757) {
                                                                                                    2u64
                                                                                                } else {
                                                                                                    let a758 = v0;
                                                                                                    let a759 = 24u64;
                                                                                                    if nat_eq(a758, a759) {
                                                                                                        1u64
                                                                                                    } else {
                                                                                                        let a760 = v0;
                                                                                                        let a761 = 25u64;
                                                                                                        if nat_eq(a760, a761) {
                                                                                                            3u64
                                                                                                        } else {
                                                                                                            let a762 = v0;
                                                                                                            let a763 = 26u64;
                                                                                                            if nat_eq(a762, a763) {
                                                                                                                1u64
                                                                                                            } else {
                                                                                                                let a764 = v0;
                                                                                                                let a765 = 27u64;
                                                                                                                if nat_eq(a764, a765) {
                                                                                                                    2u64
                                                                                                                } else {
                                                                                                                    let a766 = v0;
                                                                                                                    let a767 = 28u64;
                                                                                                                    if nat_eq(a766, a767) {
                                                                                                                        2u64
                                                                                                                    } else {
                                                                                                                        let a768 = v0;
                                                                                                                        let a769 = 29u64;
                                                                                                                        if nat_eq(a768, a769) {
                                                                                                                            3u64
                                                                                                                        } else {
                                                                                                                            let a770 = v0;
                                                                                                                            let a771 = 30u64;
                                                                                                                            if nat_eq(a770, a771) {
                                                                                                                                2u64
                                                                                                                            } else {
                                                                                                                                let a772 = v0;
                                                                                                                                let a773 = 31u64;
                                                                                                                                if nat_eq(a772, a773) {
                                                                                                                                    1u64
                                                                                                                                } else {
                                                                                                                                    let a774 = v0;
                                                                                                                                    let a775 = 32u64;
                                                                                                                                    if nat_eq(a774, a775) {
                                                                                                                                        1u64
                                                                                                                                    } else {
                                                                                                                                        let a776 = v0;
                                                                                                                                        let a777 = 33u64;
                                                                                                                                        if nat_eq(a776, a777) {
                                                                                                                                            2u64
                                                                                                                                        } else {
                                                                                                                                            let a778 = v0;
                                                                                                                                            let a779 = 34u64;
                                                                                                                                            if nat_eq(a778, a779) {
                                                                                                                                                3u64
                                                                                                                                            } else {
                                                                                                                                                let a780 = v0;
                                                                                                                                                let a781 = 35u64;
                                                                                                                                                if nat_eq(a780, a781) {
                                                                                                                                                    1u64
                                                                                                                                                } else {
                                                                                                                                                    let a782 = v0;
                                                                                                                                                    let a783 = 36u64;
                                                                                                                                                    if nat_eq(a782, a783) {
                                                                                                                                                        1u64
                                                                                                                                                    } else {
                                                                                                                                                        let a784 = v0;
                                                                                                                                                        let a785 = 37u64;
                                                                                                                                                        if nat_eq(a784, a785) {
                                                                                                                                                            2u64
                                                                                                                                                        } else {
                                                                                                                                                            let a786 = v0;
                                                                                                                                                            let a787 = 38u64;
                                                                                                                                                            if nat_eq(a786, a787) {
                                                                                                                                                                3u64
                                                                                                                                                            } else {
                                                                                                                                                                let a788 = v0;
                                                                                                                                                                let a789 = 39u64;
                                                                                                                                                                if nat_eq(a788, a789) {
                                                                                                                                                                    2u64
                                                                                                                                                                } else {
                                                                                                                                                                    0u64
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
                        }
                    }
                }
            }
        }
    }
}

pub fn f105(v0: u64) -> Bytes {
    let a790 = v0;
    let a791 = 0u64;
    if nat_eq(a790, a791) {
        f1(Str::lit("checked_add"))
    } else {
        let a792 = v0;
        let a793 = 1u64;
        if nat_eq(a792, a793) {
            f1(Str::lit("checked_sub"))
        } else {
            let a794 = v0;
            let a795 = 2u64;
            if nat_eq(a794, a795) {
                f1(Str::lit("checked_mul"))
            } else {
                let a796 = v0;
                let a797 = 3u64;
                if nat_eq(a796, a797) {
                    f1(Str::lit("checked_div"))
                } else {
                    let a798 = v0;
                    let a799 = 4u64;
                    if nat_eq(a798, a799) {
                        f1(Str::lit("checked_neg"))
                    } else {
                        let a800 = v0;
                        let a801 = 5u64;
                        if nat_eq(a800, a801) {
                            f1(Str::lit("checked_shl"))
                        } else {
                            let a802 = v0;
                            let a803 = 6u64;
                            if nat_eq(a802, a803) {
                                f1(Str::lit("checked_shr"))
                            } else {
                                let a804 = v0;
                                let a805 = 7u64;
                                if nat_eq(a804, a805) {
                                    f1(Str::lit("convert"))
                                } else {
                                    let a806 = v0;
                                    let a807 = 8u64;
                                    if nat_eq(a806, a807) {
                                        f1(Str::lit("bit_and"))
                                    } else {
                                        let a808 = v0;
                                        let a809 = 9u64;
                                        if nat_eq(a808, a809) {
                                            f1(Str::lit("bit_or"))
                                        } else {
                                            let a810 = v0;
                                            let a811 = 10u64;
                                            if nat_eq(a810, a811) {
                                                f1(Str::lit("bit_xor"))
                                            } else {
                                                let a812 = v0;
                                                let a813 = 11u64;
                                                if nat_eq(a812, a813) {
                                                    f1(Str::lit("bit_not"))
                                                } else {
                                                    let a814 = v0;
                                                    let a815 = 12u64;
                                                    if nat_eq(a814, a815) {
                                                        f1(Str::lit("nat_eq"))
                                                    } else {
                                                        let a816 = v0;
                                                        let a817 = 13u64;
                                                        if nat_eq(a816, a817) {
                                                            f1(Str::lit("nat_le"))
                                                        } else {
                                                            let a818 = v0;
                                                            let a819 = 14u64;
                                                            if nat_eq(a818, a819) {
                                                                f1(Str::lit("nat_lt"))
                                                            } else {
                                                                let a820 = v0;
                                                                let a821 = 15u64;
                                                                if nat_eq(a820, a821) {
                                                                    f1(Str::lit("nat_rem"))
                                                                } else {
                                                                    let a822 = v0;
                                                                    let a823 = 16u64;
                                                                    if nat_eq(a822, a823) {
                                                                        f1(Str::lit("int_rem"))
                                                                    } else {
                                                                        let a824 = v0;
                                                                        let a825 = 17u64;
                                                                        if nat_eq(a824, a825) {
                                                                            f1(Str::lit("nat_add"))
                                                                        } else {
                                                                            let a826 = v0;
                                                                            let a827 = 18u64;
                                                                            if nat_eq(a826, a827) {
                                                                                f1(Str::lit("nat_sub"))
                                                                            } else {
                                                                                let a828 = v0;
                                                                                let a829 = 19u64;
                                                                                if nat_eq(a828, a829) {
                                                                                    f1(Str::lit("nat_mul"))
                                                                                } else {
                                                                                    let a830 = v0;
                                                                                    let a831 = 20u64;
                                                                                    if nat_eq(a830, a831) {
                                                                                        f1(Str::lit("nat_quot"))
                                                                                    } else {
                                                                                        let a832 = v0;
                                                                                        let a833 = 21u64;
                                                                                        if nat_eq(a832, a833) {
                                                                                            f1(Str::lit("int_add"))
                                                                                        } else {
                                                                                            let a834 = v0;
                                                                                            let a835 = 22u64;
                                                                                            if nat_eq(a834, a835) {
                                                                                                f1(Str::lit("int_sub"))
                                                                                            } else {
                                                                                                let a836 = v0;
                                                                                                let a837 = 23u64;
                                                                                                if nat_eq(a836, a837) {
                                                                                                    f1(Str::lit("int_mul"))
                                                                                                } else {
                                                                                                    let a838 = v0;
                                                                                                    let a839 = 24u64;
                                                                                                    if nat_eq(a838, a839) {
                                                                                                        f1(Str::lit("int_neg"))
                                                                                                    } else {
                                                                                                        let a840 = v0;
                                                                                                        let a841 = 25u64;
                                                                                                        if nat_eq(a840, a841) {
                                                                                                            f1(Str::lit("int_quot"))
                                                                                                        } else {
                                                                                                            let a842 = v0;
                                                                                                            let a843 = 26u64;
                                                                                                            if nat_eq(a842, a843) {
                                                                                                                f1(Str::lit("list_length"))
                                                                                                            } else {
                                                                                                                let a844 = v0;
                                                                                                                let a845 = 27u64;
                                                                                                                if nat_eq(a844, a845) {
                                                                                                                    f1(Str::lit("list_append"))
                                                                                                                } else {
                                                                                                                    let a846 = v0;
                                                                                                                    let a847 = 28u64;
                                                                                                                    if nat_eq(a846, a847) {
                                                                                                                        f1(Str::lit("list_index"))
                                                                                                                    } else {
                                                                                                                        let a848 = v0;
                                                                                                                        let a849 = 29u64;
                                                                                                                        if nat_eq(a848, a849) {
                                                                                                                            f1(Str::lit("list_slice"))
                                                                                                                        } else {
                                                                                                                            let a850 = v0;
                                                                                                                            let a851 = 30u64;
                                                                                                                            if nat_eq(a850, a851) {
                                                                                                                                f1(Str::lit("bytes_append"))
                                                                                                                            } else {
                                                                                                                                let a852 = v0;
                                                                                                                                let a853 = 31u64;
                                                                                                                                if nat_eq(a852, a853) {
                                                                                                                                    f1(Str::lit("bytes_length"))
                                                                                                                                } else {
                                                                                                                                    let a854 = v0;
                                                                                                                                    let a855 = 32u64;
                                                                                                                                    if nat_eq(a854, a855) {
                                                                                                                                        f1(Str::lit("string_length"))
                                                                                                                                    } else {
                                                                                                                                        let a856 = v0;
                                                                                                                                        let a857 = 33u64;
                                                                                                                                        if nat_eq(a856, a857) {
                                                                                                                                            f1(Str::lit("bytes_index"))
                                                                                                                                        } else {
                                                                                                                                            let a858 = v0;
                                                                                                                                            let a859 = 34u64;
                                                                                                                                            if nat_eq(a858, a859) {
                                                                                                                                                f1(Str::lit("bytes_slice"))
                                                                                                                                            } else {
                                                                                                                                                let a860 = v0;
                                                                                                                                                let a861 = 35u64;
                                                                                                                                                if nat_eq(a860, a861) {
                                                                                                                                                    f1(Str::lit("utf8_encode"))
                                                                                                                                                } else {
                                                                                                                                                    let a862 = v0;
                                                                                                                                                    let a863 = 36u64;
                                                                                                                                                    if nat_eq(a862, a863) {
                                                                                                                                                        f1(Str::lit("utf8_decode"))
                                                                                                                                                    } else {
                                                                                                                                                        let a864 = v0;
                                                                                                                                                        let a865 = 37u64;
                                                                                                                                                        if nat_eq(a864, a865) {
                                                                                                                                                            f1(Str::lit("bytes_compare"))
                                                                                                                                                        } else {
                                                                                                                                                            let a866 = v0;
                                                                                                                                                            let a867 = 38u64;
                                                                                                                                                            if nat_eq(a866, a867) {
                                                                                                                                                                f1(Str::lit("string_split"))
                                                                                                                                                            } else {
                                                                                                                                                                let a868 = v0;
                                                                                                                                                                let a869 = 39u64;
                                                                                                                                                                if nat_eq(a868, a869) {
                                                                                                                                                                    f1(Str::lit("string_join"))
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

pub fn f106(v0: List<Adt25>, v1: u64) -> Option<Adt25> {
    f153(Fn13::F154(v1), None::<Adt25>, v0)
}

pub fn f107(v0: Adt4) -> Bytes {
    let m870 = v0;
    match m870 {
        Adt4::C0 => {
            f1(Str::lit("u8"))
        }
        Adt4::C1 => {
            f1(Str::lit("u16"))
        }
        Adt4::C2 => {
            f1(Str::lit("u32"))
        }
        Adt4::C3 => {
            f1(Str::lit("u64"))
        }
        Adt4::C4 => {
            f1(Str::lit("i8"))
        }
        Adt4::C5 => {
            f1(Str::lit("i16"))
        }
        Adt4::C6 => {
            f1(Str::lit("i32"))
        }
        Adt4::C7 => {
            f1(Str::lit("i64"))
        }
    }
}

pub fn f108(v0: Option<Adt4>) -> Adt4 {
    let m871 = v0;
    match m871 {
        None => {
            Adt4::C0
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f109(v0: Adt26, v1: u64, v2: u64) -> Option<u64> {
    let m873 = f18(match v0 {
        Adt26::C0(_, h872, _, _) => {
            h872
        }
    }, v1);
    match m873 {
        None => {
            None::<u64>
        }
        Some(v3) => {
            f93(v3, v2)
        }
    }
}

pub fn f110(v0: Adt2) -> bool {
    let m874 = v0;
    match m874 {
        Adt2::C0(v1) => {
            let m875 = v1;
            match m875 {
                Adt3::C0 => {
                    false
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
                    true
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

pub fn f111(v0: Adt17) -> R<bool> {
    let m876 = v0;
    match m876 {
        Adt17::C0(_, _) => {
            Ok(false)
        }
        Adt17::C1(_, _, _) => {
            Ok(false)
        }
        Adt17::C2(_, _, _) => {
            Ok(false)
        }
        Adt17::C3(_, _, _, _) => {
            Ok(false)
        }
        Adt17::C4(_, _, _, _) => {
            Ok(false)
        }
        Adt17::C5(_, _, _) => {
            Ok(false)
        }
        Adt17::C6(_, _, _) => {
            Ok(false)
        }
        Adt17::C7(_, _, _, _) => {
            Ok(false)
        }
        Adt17::C8(_, _, _) => {
            Ok(false)
        }
        Adt17::C9(_) => {
            Ok(false)
        }
        Adt17::C10(_, _, _) => {
            Ok(false)
        }
        Adt17::C11(_, _) => {
            Ok(false)
        }
        Adt17::C12 => {
            Ok(false)
        }
        Adt17::C13(r879, r880) => {
            let v36 = (*r879).clone();
            let v37 = (*r880).clone();
            let v38: bool = {
                let a877 = f76(v37)?;
                let a878 = 0u64;
                nat_eq(a877, a878)
            };
            let v39: bool = f103(v36);
            if v38 {
                Ok(v39)
            } else {
                Ok(false)
            }
        }
        Adt17::C14(_, _, _, _, _, _) => {
            Ok(false)
        }
        Adt17::C15(_, _, _) => {
            Ok(false)
        }
        Adt17::C16(_, _) => {
            Ok(false)
        }
    }
}

pub fn f112(v0: Fn5, p884: List<Adt12>, p885: List<Adt12>) -> List<Adt12> {
    let m883 = (p884, p885);
    {
        let mut m883 = m883;
        loop {
            let step = {
                let (v1, v2) = m883;
                let m881 = v2;
                match m881.uncons() {
                    None => {
                        Err::<(List<Adt12>, List<Adt12>), List<Adt12>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt12>, List<Adt12>), List<Adt12>>(({
                            let c882: Fn5 = v0.clone();
                            c882.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m883 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f113(v0: List<Adt12>, v1: Adt12) -> List<Adt12> {
    let v2: bool = f57(v0.clone(), v1.clone());
    if v2 {
        v0
    } else {
        List::cons(v1, v0)
    }
}

pub fn f114(v0: Fn6, p889: List<(Adt12, List<u64>)>, p890: List<(Adt12, List<u64>)>) -> List<(Adt12, List<u64>)> {
    let m888 = (p889, p890);
    {
        let mut m888 = m888;
        loop {
            let step = {
                let (v1, v2) = m888;
                let m886 = v2;
                match m886.uncons() {
                    None => {
                        Err::<(List<(Adt12, List<u64>)>, List<(Adt12, List<u64>)>), List<(Adt12, List<u64>)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Adt12, List<u64>)>, List<(Adt12, List<u64>)>), List<(Adt12, List<u64>)>>(({
                            let c887: Fn6 = v0.clone();
                            c887.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m888 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f115(v0: List<(Adt12, List<u64>)>, v1: (Adt12, List<u64>)) -> List<(Adt12, List<u64>)> {
    let m892 = f96(v0.clone(), {
        let (h891, _) = v1.clone();
        h891
    });
    match m892 {
        None => {
            f98(v0, {
                let (h895, _) = v1;
                h895
            }, {
                let (_, h896) = v1;
                h896
            })
        }
        Some(v2) => {
            f98(v0, {
                let (h893, _) = v1;
                h893
            }, f155(v2, {
                let (_, h894) = v1;
                h894
            }))
        }
    }
}

pub fn f116(v0: List<Adt25>, v1: Adt17) -> R<Result<(), Bytes>> {
    let m897 = v1;
    match m897 {
        Adt17::C0(_, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C1(_, _, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C2(_, _, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C3(_, _, _, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C4(_, _, _, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C5(v18, r900, _) => {
            let v19 = (*r900).clone();
            let m898 = f73(v18);
            if m898 {
                f156(v0, v19)
            } else {
                Ok(Ok::<(), Bytes>(()))
            }
        }
        Adt17::C6(_, _, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C7(_, r901, r902, _) => {
            let v25 = (*r901).clone();
            let v26 = (*r902).clone();
            let m899 = f81(v0.clone(), v25)?;
            match m899 {
                Ok(_) => {
                    f81(v0, v26)
                }
                Err(v29) => {
                    Ok(Err::<(), Bytes>(v29))
                }
            }
        }
        Adt17::C8(_, r903, _) => {
            let v31 = (*r903).clone();
            f157(v0, v31)
        }
        Adt17::C9(r904) => {
            let v33 = (*r904).clone();
            f81(v0, v33)
        }
        Adt17::C10(_, _, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C11(_, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C12 => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C13(_, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C14(_, _, _, _, _, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C15(_, _, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C16(_, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
    }
}

pub fn f117(v0: Adt12, v1: List<Adt12>, v2: Adt12) -> List<Adt12> {
    let v3: bool = f125(v2.clone(), v0);
    if v3 {
        v1
    } else {
        List::cons(v2, v1)
    }
}

pub fn f118(v0: Fn7, p908: Option<Adt12>, p909: List<Adt12>) -> Option<Adt12> {
    let m907 = (p908, p909);
    {
        let mut m907 = m907;
        loop {
            let step = {
                let (v1, v2) = m907;
                let m905 = v2;
                match m905.uncons() {
                    None => {
                        Err::<(Option<Adt12>, List<Adt12>), Option<Adt12>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Adt12>, List<Adt12>), Option<Adt12>>(({
                            let c906: Fn7 = v0.clone();
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

pub fn f119(v0: List<Adt12>, v1: List<Adt12>, v2: Adt12, v3: Option<Adt12>, v4: Adt12) -> Option<Adt12> {
    let v5: bool = f125(v4.clone(), v2);
    let v6: bool = f57(v0, v4.clone());
    let v7: bool = f57(v1, v4.clone());
    let v8: bool = if v5 {
        false
    } else {
        if v6 {
            false
        } else {
            v7
        }
    };
    if v8 {
        f158(v3, v4)
    } else {
        v3
    }
}

pub fn f120(v0: Fn8, p913: Option<Adt12>, p914: List<(Adt12, List<u64>)>) -> Option<Adt12> {
    let m912 = (p913, p914);
    {
        let mut m912 = m912;
        loop {
            let step = {
                let (v1, v2) = m912;
                let m910 = v2;
                match m910.uncons() {
                    None => {
                        Err::<(Option<Adt12>, List<(Adt12, List<u64>)>), Option<Adt12>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Adt12>, List<(Adt12, List<u64>)>), Option<Adt12>>(({
                            let c911: Fn8 = v0.clone();
                            c911.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m912 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f121(v0: List<(Adt12, List<u64>)>, v1: List<Adt12>, v2: Option<Adt12>, v3: (Adt12, List<u64>)) -> Option<Adt12> {
    let v4: bool = f57(v1, {
        let (h915, _) = v3.clone();
        h915
    });
    let v5: bool = f159(v0, {
        let (h916, _) = v3.clone();
        h916
    }, {
        let (_, h917) = v3;
        h917
    });
    let v6: bool = if v4 {
        !v5
    } else {
        false
    };
    if v6 {
        f158(v2, {
            let (h918, _) = v3;
            h918
        })
    } else {
        v2
    }
}

pub fn f122(v0: Fn9, p922: Option<u64>, p923: List<(Adt12, u64)>) -> Option<u64> {
    let m921 = (p922, p923);
    {
        let mut m921 = m921;
        loop {
            let step = {
                let (v1, v2) = m921;
                let m919 = v2;
                match m919.uncons() {
                    None => {
                        Err::<(Option<u64>, List<(Adt12, u64)>), Option<u64>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<u64>, List<(Adt12, u64)>), Option<u64>>(({
                            let c920: Fn9 = v0.clone();
                            c920.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m921 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f123(v0: Adt12, v1: Option<u64>, v2: (Adt12, u64)) -> Option<u64> {
    let m924 = v1;
    match m924 {
        None => {
            let v4: bool = f125({
                let (h925, _) = v2;
                h925
            }, v0);
            if v4 {
                Some({
                    let (_, h926) = v2;
                    h926
                })
            } else {
                None::<u64>
            }
        }
        Some(v3) => {
            Some(v3)
        }
    }
}

pub fn f124(v0: Bytes, v1: u64) -> Option<u8> {
    let a927 = v0;
    let a928 = v1;
    index_bytes(a927, a928)
}

pub fn f125(v0: Adt12, v1: Adt12) -> bool {
    let a929 = f160(v0.clone());
    let a930 = f160(v1.clone());
    if nat_eq(a929, a930) {
        let a931 = f161(v0.clone());
        let a932 = f161(v1.clone());
        if nat_eq(a931, a932) {
            let a933 = f162(v0);
            let a934 = f162(v1);
            equal(a933, a934)
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f126(v0: Fn10, p938: Option<u64>, p939: List<(u64, u64)>) -> Option<u64> {
    let m937 = (p938, p939);
    {
        let mut m937 = m937;
        loop {
            let step = {
                let (v1, v2) = m937;
                let m935 = v2;
                match m935.uncons() {
                    None => {
                        Err::<(Option<u64>, List<(u64, u64)>), Option<u64>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<u64>, List<(u64, u64)>), Option<u64>>(({
                            let c936: Fn10 = v0.clone();
                            c936.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m937 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f127(v0: u64, v1: Option<u64>, v2: (u64, u64)) -> Option<u64> {
    let m940 = v1;
    match m940 {
        None => {
            let a942 = {
                let (h941, _) = v2;
                h941
            };
            let a943 = v0;
            if nat_eq(a942, a943) {
                Some({
                    let (_, h944) = v2;
                    h944
                })
            } else {
                None::<u64>
            }
        }
        Some(v3) => {
            Some(v3)
        }
    }
}

pub fn f128(v0: Adt14) -> bool {
    let m945 = v0;
    match m945 {
        Adt14::C0(v1) => {
            let m946 = v1;
            match m946 {
                Adt15::C0 => {
                    true
                }
                Adt15::C1 => {
                    false
                }
                Adt15::C2 => {
                    false
                }
                Adt15::C3 => {
                    false
                }
            }
        }
        Adt14::C1(_) => {
            false
        }
        Adt14::C2(_) => {
            false
        }
        Adt14::C3(_, _) => {
            false
        }
        Adt14::C4(_, _, _) => {
            false
        }
        Adt14::C5 => {
            false
        }
        Adt14::C6(_, _) => {
            false
        }
    }
}

pub fn f129(v0: Fn11, p950: Option<List<u64>>, p951: List<(Adt12, List<u64>)>) -> Option<List<u64>> {
    let m949 = (p950, p951);
    {
        let mut m949 = m949;
        loop {
            let step = {
                let (v1, v2) = m949;
                let m947 = v2;
                match m947.uncons() {
                    None => {
                        Err::<(Option<List<u64>>, List<(Adt12, List<u64>)>), Option<List<u64>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<List<u64>>, List<(Adt12, List<u64>)>), Option<List<u64>>>(({
                            let c948: Fn11 = v0.clone();
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

pub fn f130(v0: Adt12, v1: Option<List<u64>>, v2: (Adt12, List<u64>)) -> Option<List<u64>> {
    let m952 = v1;
    match m952 {
        None => {
            let v4: bool = f125({
                let (h953, _) = v2;
                h953
            }, v0);
            if v4 {
                Some({
                    let (_, h954) = v2;
                    h954
                })
            } else {
                None::<List<u64>>
            }
        }
        Some(v3) => {
            Some(v3)
        }
    }
}

pub fn f131(v0: List<u64>, v1: u64) -> bool {
    f163(Fn14::F164(v1), false, v0)
}

pub fn f132(v0: List<(Adt12, List<u64>)>, v1: Adt12) -> List<(Adt12, List<u64>)> {
    f114(Fn6::F165(v1), List::<(Adt12, List<u64>)>::nil(), v0)
}

pub fn f133(v0: Fn12, p958: Option<Bytes>, p959: List<Adt29>) -> Option<Bytes> {
    let m957 = (p958, p959);
    {
        let mut m957 = m957;
        loop {
            let step = {
                let (v1, v2) = m957;
                let m955 = v2;
                match m955.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Adt29>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Adt29>), Option<Bytes>>(({
                            let c956: Fn12 = v0.clone();
                            c956.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m957 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f134(v0: Option<Bytes>, v1: Adt29) -> Option<Bytes> {
    let m960 = v0;
    match m960 {
        None => {
            Some(match v1 {
                Adt29::C0(h961, _, _, _) => {
                    h961
                }
            })
        }
        Some(v2) => {
            let m963 = f166(match v1.clone() {
                Adt29::C0(h962, _, _, _) => {
                    h962
                }
            }, v2.clone());
            if m963 {
                Some(match v1 {
                    Adt29::C0(h964, _, _, _) => {
                        h964
                    }
                })
            } else {
                Some(v2)
            }
        }
    }
}

pub fn f135(v0: Adt18) -> bool {
    let m965 = v0;
    match m965 {
        Adt18::C0 => {
            false
        }
        Adt18::C1 => {
            false
        }
        Adt18::C2 => {
            false
        }
        Adt18::C3 => {
            false
        }
        Adt18::C4(_) => {
            false
        }
        Adt18::C5 => {
            false
        }
        Adt18::C6 => {
            false
        }
        Adt18::C7 => {
            false
        }
        Adt18::C8 => {
            true
        }
    }
}

pub fn f136(v0: Adt30, v1: Bytes, v2: Adt9, v3: Option<Adt4>) -> Adt30 {
    let v4: Adt29 = Adt29::C0(v1, match v2.clone() {
        Adt9::C0(h966, _) => {
            h966
        }
    }, match v2 {
        Adt9::C0(_, h967) => {
            h967
        }
    }, v3);
    let m969 = f167(match v0.clone() {
        Adt30::C0(h968, _, _) => {
            h968
        }
    }, v4.clone());
    if m969 {
        v0
    } else {
        Adt30::C0(List::cons(v4, match v0.clone() {
            Adt30::C0(h970, _, _) => {
                h970
            }
        }), match v0.clone() {
            Adt30::C0(_, h971, _) => {
                h971
            }
        }, match v0 {
            Adt30::C0(_, _, h972) => {
                h972
            }
        })
    }
}

pub fn f137(v0: Adt18) -> Bytes {
    let m973 = v0;
    match m973 {
        Adt18::C0 => {
            f1(Str::lit("lit:unit"))
        }
        Adt18::C1 => {
            f1(Str::lit("lit:bool"))
        }
        Adt18::C2 => {
            f1(Str::lit("lit:nat"))
        }
        Adt18::C3 => {
            f1(Str::lit("lit:int"))
        }
        Adt18::C4(_) => {
            f1(Str::lit("lit:fixed"))
        }
        Adt18::C5 => {
            f1(Str::lit("lit:string"))
        }
        Adt18::C6 => {
            f1(Str::lit("lit:bytes"))
        }
        Adt18::C7 => {
            f1(Str::lit("lit:ordering"))
        }
        Adt18::C8 => {
            f1(Str::lit("lit:overflow"))
        }
    }
}

pub fn f138(v0: Adt18) -> Option<Adt4> {
    let m974 = v0;
    match m974 {
        Adt18::C0 => {
            None::<Adt4>
        }
        Adt18::C1 => {
            None::<Adt4>
        }
        Adt18::C2 => {
            None::<Adt4>
        }
        Adt18::C3 => {
            None::<Adt4>
        }
        Adt18::C4(v1) => {
            Some(v1)
        }
        Adt18::C5 => {
            None::<Adt4>
        }
        Adt18::C6 => {
            None::<Adt4>
        }
        Adt18::C7 => {
            None::<Adt4>
        }
        Adt18::C8 => {
            None::<Adt4>
        }
    }
}

pub fn f139(v0: Adt30, v1: Bytes, v2: Adt9) -> Adt30 {
    f136(v0, v1, v2, None::<Adt4>)
}

pub fn f140(v0: Adt19) -> Bytes {
    let m975 = v0;
    match m975 {
        Adt19::C0 => {
            f1(Str::lit("read"))
        }
        Adt19::C1 => {
            f1(Str::lit("read"))
        }
        Adt19::C2 => {
            f1(Str::lit("read"))
        }
        Adt19::C3 => {
            f1(Str::lit("read"))
        }
        Adt19::C4 => {
            f1(Str::lit("box"))
        }
    }
}

pub fn f141(v0: Adt20) -> Bytes {
    let m976 = v0;
    match m976 {
        Adt20::C0 => {
            f1(Str::lit("if"))
        }
        Adt20::C1 => {
            f1(Str::lit("box"))
        }
        Adt20::C2 => {
            f1(Str::lit("widen"))
        }
        Adt20::C3 => {
            f1(Str::lit("succeed"))
        }
    }
}

pub fn f142(v0: Bytes) -> Bytes {
    let m977 = f168(v0.clone(), f1(Str::lit("checked_neg_")));
    if m977 {
        f1(Str::lit("checked_neg"))
    } else {
        let m978 = f168(v0.clone(), f1(Str::lit("format_")));
        if m978 {
            f1(Str::lit("format_decimal"))
        } else {
            let m979 = f168(v0.clone(), f1(Str::lit("parse_")));
            if m979 {
                f1(Str::lit("parse_decimal"))
            } else {
                let m980 = f168(v0.clone(), f1(Str::lit("append_")));
                if m980 {
                    f1(Str::lit("append"))
                } else {
                    let m981 = f168(v0.clone(), f1(Str::lit("length_")));
                    if m981 {
                        f1(Str::lit("length"))
                    } else {
                        let m982 = f168(v0.clone(), f1(Str::lit("index_")));
                        if m982 {
                            f1(Str::lit("index"))
                        } else {
                            let m983 = f168(v0.clone(), f1(Str::lit("slice_")));
                            if m983 {
                                f1(Str::lit("slice"))
                            } else {
                                let m986 = {
                                    let a984 = v0.clone();
                                    let a985 = f1(Str::lit("nat_succ"));
                                    equal(a984, a985)
                                };
                                if m986 {
                                    f1(Str::lit("succ"))
                                } else {
                                    v0
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f143(v0: Bytes) -> R<Bytes> {
    let m987 = f169(v0.clone(), f1(Str::lit("::")))?;
    match m987 {
        None => {
            Ok(v0)
        }
        Some(v1) => {
            Ok(f40(v0.clone(), {
                let a988 = v1;
                let a989 = 2u64;
                nat_add(a988, a989)?
            }, {
                let a993 = {
                    let a990 = v0;
                    length_bytes(a990)
                };
                let a994 = {
                    let a991 = v1;
                    let a992 = 2u64;
                    nat_add(a991, a992)?
                };
                nat_sub(a993, a994)
            }))
        }
    }
}

pub fn f144(v0: List<Adt25>, v1: Adt22) -> Option<Adt4> {
    let m996 = f106(v0, match v1.clone() {
        Adt22::C0(h995, _) => {
            h995
        }
    });
    match m996 {
        None => {
            None::<Adt4>
        }
        Some(v2) => {
            let a998 = match v2 {
                Adt25::C0(_, _, h997, _, _) => {
                    h997
                }
            };
            let a999 = 0u64;
            if nat_eq(a998, a999) {
                None::<Adt4>
            } else {
                Some(f108(match v1 {
                    Adt22::C0(_, h1000) => {
                        h1000
                    }
                }))
            }
        }
    }
}

pub fn f145(v0: Adt30, v1: Adt23) -> Adt30 {
    let m1001 = v1;
    match m1001 {
        Adt23::C0(v2) => {
            f152(v0, v2)
        }
        Adt23::C1 => {
            v0
        }
        Adt23::C2(v3, v4) => {
            f152(f152(v0, v3), v4)
        }
        Adt23::C3(v5, v6) => {
            f152(f152(v0, v5), v6)
        }
        Adt23::C4(_, _) => {
            v0
        }
        Adt23::C5(_, _) => {
            v0
        }
        Adt23::C6 => {
            v0
        }
        Adt23::C7(v11) => {
            f152(v0, v11)
        }
    }
}

pub fn f146(v0: Adt23) -> Bytes {
    let m1002 = v0;
    match m1002 {
        Adt23::C0(_) => {
            f1(Str::lit("none"))
        }
        Adt23::C1 => {
            f1(Str::lit("some"))
        }
        Adt23::C2(_, _) => {
            f1(Str::lit("ok"))
        }
        Adt23::C3(_, _) => {
            f1(Str::lit("err"))
        }
        Adt23::C4(_, _) => {
            f1(Str::lit("adt"))
        }
        Adt23::C5(_, _) => {
            f1(Str::lit("closure"))
        }
        Adt23::C6 => {
            f1(Str::lit("cons"))
        }
        Adt23::C7(_) => {
            f1(Str::lit("nil"))
        }
    }
}

pub fn f147(v0: Adt17) -> bool {
    let m1003 = v0;
    match m1003 {
        Adt17::C0(_, _) => {
            false
        }
        Adt17::C1(_, _, _) => {
            false
        }
        Adt17::C2(_, _, _) => {
            false
        }
        Adt17::C3(_, _, _, _) => {
            false
        }
        Adt17::C4(_, _, _, _) => {
            false
        }
        Adt17::C5(_, _, _) => {
            false
        }
        Adt17::C6(_, _, _) => {
            false
        }
        Adt17::C7(_, _, _, _) => {
            false
        }
        Adt17::C8(_, _, _) => {
            false
        }
        Adt17::C9(_) => {
            false
        }
        Adt17::C10(_, _, _) => {
            false
        }
        Adt17::C11(_, _) => {
            false
        }
        Adt17::C12 => {
            true
        }
        Adt17::C13(_, _) => {
            false
        }
        Adt17::C14(_, _, _, _, _, _) => {
            false
        }
        Adt17::C15(_, _, _) => {
            false
        }
        Adt17::C16(_, _) => {
            false
        }
    }
}

pub fn f148(v0: Adt17) -> bool {
    let m1004 = v0;
    match m1004 {
        Adt17::C0(_, _) => {
            false
        }
        Adt17::C1(_, _, _) => {
            false
        }
        Adt17::C2(_, _, _) => {
            false
        }
        Adt17::C3(_, _, _, _) => {
            false
        }
        Adt17::C4(_, _, _, _) => {
            false
        }
        Adt17::C5(_, _, _) => {
            false
        }
        Adt17::C6(_, _, _) => {
            false
        }
        Adt17::C7(_, _, _, _) => {
            false
        }
        Adt17::C8(_, _, _) => {
            false
        }
        Adt17::C9(_) => {
            false
        }
        Adt17::C10(v31, _, _) => {
            f170(v31)
        }
        Adt17::C11(_, _) => {
            false
        }
        Adt17::C12 => {
            false
        }
        Adt17::C13(_, _) => {
            false
        }
        Adt17::C14(_, _, _, _, _, _) => {
            false
        }
        Adt17::C15(_, _, _) => {
            false
        }
        Adt17::C16(_, _) => {
            false
        }
    }
}

pub fn f149(p1009: Adt30, p1010: Adt17, v2: Adt9) -> Adt30 {
    let m1008 = (p1009, p1010);
    {
        let mut m1008 = m1008;
        loop {
            let step = {
                let (v0, v1) = m1008;
                let m1005 = v1;
                match m1005 {
                    Adt17::C0(_, _) => {
                        Err::<(Adt30, Adt17), Adt30>(v0)
                    }
                    Adt17::C1(_, _, _) => {
                        Err::<(Adt30, Adt17), Adt30>(v0)
                    }
                    Adt17::C2(_, _, _) => {
                        Err::<(Adt30, Adt17), Adt30>(v0)
                    }
                    Adt17::C3(_, _, _, _) => {
                        Err::<(Adt30, Adt17), Adt30>(v0)
                    }
                    Adt17::C4(_, _, _, _) => {
                        Err::<(Adt30, Adt17), Adt30>(v0)
                    }
                    Adt17::C5(_, _, _) => {
                        Err::<(Adt30, Adt17), Adt30>(v0)
                    }
                    Adt17::C6(_, _, _) => {
                        Err::<(Adt30, Adt17), Adt30>(v0)
                    }
                    Adt17::C7(_, _, _, _) => {
                        Err::<(Adt30, Adt17), Adt30>(v0)
                    }
                    Adt17::C8(_, _, _) => {
                        Err::<(Adt30, Adt17), Adt30>(v0)
                    }
                    Adt17::C9(_) => {
                        Err::<(Adt30, Adt17), Adt30>(v0)
                    }
                    Adt17::C10(_, _, _) => {
                        Err::<(Adt30, Adt17), Adt30>(v0)
                    }
                    Adt17::C11(_, _) => {
                        Err::<(Adt30, Adt17), Adt30>(v0)
                    }
                    Adt17::C12 => {
                        Err::<(Adt30, Adt17), Adt30>(v0)
                    }
                    Adt17::C13(_, _) => {
                        Err::<(Adt30, Adt17), Adt30>(v0)
                    }
                    Adt17::C14(_, _, _, _, _, _) => {
                        Err::<(Adt30, Adt17), Adt30>(v0)
                    }
                    Adt17::C15(v46, _, r1007) => {
                        let v48 = (*r1007).clone();
                        let m1006 = f171(v46);
                        match m1006 {
                            None => {
                                Ok::<(Adt30, Adt17), Adt30>((v0, v48))
                            }
                            Some(v49) => {
                                Ok::<(Adt30, Adt17), Adt30>((f139(v0, v49, v2.clone()), v48))
                            }
                        }
                    }
                    Adt17::C16(_, _) => {
                        Err::<(Adt30, Adt17), Adt30>(v0)
                    }
                }
            };
            match step {
                Ok(next) => { m1008 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f150(v0: Adt24) -> bool {
    let m1011 = v0;
    match m1011 {
        Adt24::C0 => {
            true
        }
        Adt24::C1 => {
            true
        }
        Adt24::C2 => {
            false
        }
        Adt24::C3 => {
            false
        }
    }
}

pub fn f151(v0: Adt14) -> bool {
    let m1012 = v0;
    match m1012 {
        Adt14::C0(_) => {
            false
        }
        Adt14::C1(_) => {
            false
        }
        Adt14::C2(_) => {
            true
        }
        Adt14::C3(_, _) => {
            false
        }
        Adt14::C4(_, _, _) => {
            false
        }
        Adt14::C5 => {
            false
        }
        Adt14::C6(_, _) => {
            false
        }
    }
}

pub fn f152(v0: Adt30, v1: Adt2) -> Adt30 {
    let m1013 = v1;
    match m1013 {
        Adt2::C0(v2) => {
            f172(v0, f4(f1(Str::lit("type:")), f173(v2)))
        }
        Adt2::C1(_) => {
            f172(v0, f1(Str::lit("type:fixed")))
        }
        Adt2::C2(v4, r1014) => {
            let v5 = (*r1014).clone();
            let v6: Adt30 = f172(v0, f4(f1(Str::lit("type:")), f174(v4)));
            f152(v6, v5)
        }
        Adt2::C3(v7, r1015, r1016) => {
            let v8 = (*r1015).clone();
            let v9 = (*r1016).clone();
            let v10: Adt30 = f172(v0, f4(f1(Str::lit("type:")), f175(v7)));
            let v11: Adt30 = f152(v10, v8);
            f152(v11, v9)
        }
        Adt2::C4(v12, _) => {
            f172(v0, f4(f1(Str::lit("type:")), f176(v12)))
        }
        Adt2::C5 => {
            v0
        }
        Adt2::C6(r1017, r1018) => {
            let v14 = (*r1017).clone();
            let v15 = (*r1018).clone();
            let v16: Adt30 = f152(v0, v14);
            f152(v16, v15)
        }
    }
}

pub fn f153(v0: Fn13, p1022: Option<Adt25>, p1023: List<Adt25>) -> Option<Adt25> {
    let m1021 = (p1022, p1023);
    {
        let mut m1021 = m1021;
        loop {
            let step = {
                let (v1, v2) = m1021;
                let m1019 = v2;
                match m1019.uncons() {
                    None => {
                        Err::<(Option<Adt25>, List<Adt25>), Option<Adt25>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Adt25>, List<Adt25>), Option<Adt25>>(({
                            let c1020: Fn13 = v0.clone();
                            c1020.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1021 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f154(v0: u64, v1: Option<Adt25>, v2: Adt25) -> Option<Adt25> {
    let m1024 = v1;
    match m1024 {
        None => {
            let a1026 = match v2.clone() {
                Adt25::C0(h1025, _, _, _, _) => {
                    h1025
                }
            };
            let a1027 = v0;
            if nat_eq(a1026, a1027) {
                Some(v2)
            } else {
                None::<Adt25>
            }
        }
        Some(v3) => {
            Some(v3)
        }
    }
}

pub fn f155(v0: List<u64>, v1: List<u64>) -> List<u64> {
    f177(Fn15::F178, v0, v1)
}

pub fn f156(v0: List<Adt25>, p1033: Adt17) -> R<Result<(), Bytes>> {
    let m1032 = p1033;
    {
        let mut m1032 = m1032;
        loop {
            let step = {
                let v1 = m1032;
                let m1028 = v1;
                match m1028 {
                    Adt17::C0(_, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C1(_, _, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C2(_, _, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C3(_, _, _, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C4(_, _, _, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C5(_, _, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C6(_, _, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C7(_, _, _, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C8(_, _, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C9(_) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C10(_, _, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C11(_, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C12 => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C13(r1030, r1031) => {
                        let v37 = (*r1030).clone();
                        let v38 = (*r1031).clone();
                        let m1029 = f179(v37.clone())?;
                        if m1029 {
                            Ok::<Adt17, R<Result<(), Bytes>>>(v38)
                        } else {
                            Err::<Adt17, R<Result<(), Bytes>>>(Ok(Err::<(), Bytes>(f4(f1(Str::lit("arithmetic: the value a fallible loop ends with, ")), f4(f62(v0.clone(), v37)?, f1(Str::lit(", is not a fallible value")))))))
                        }
                    }
                    Adt17::C14(_, _, _, _, _, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C15(_, _, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C16(_, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                }
            };
            match step {
                Ok(next) => { m1032 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f157(v0: List<Adt25>, p1039: Adt17) -> R<Result<(), Bytes>> {
    let m1038 = p1039;
    {
        let mut m1038 = m1038;
        loop {
            let step = {
                let v1 = m1038;
                let m1034 = v1;
                match m1034 {
                    Adt17::C0(_, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C1(_, _, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C2(_, _, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C3(_, _, _, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C4(_, _, _, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C5(_, _, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C6(_, _, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C7(_, _, _, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C8(_, _, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C9(_) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C10(_, _, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C11(_, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C12 => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C13(_, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C14(_, _, _, _, _, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C15(_, r1036, r1037) => {
                        let v46 = (*r1036).clone();
                        let v47 = (*r1037).clone();
                        let m1035 = f81(v0.clone(), v46)?;
                        match m1035 {
                            Ok(_) => {
                                Ok::<Adt17, R<Result<(), Bytes>>>(v47)
                            }
                            Err(v49) => {
                                Err::<Adt17, R<Result<(), Bytes>>>(Ok(Err::<(), Bytes>(v49)))
                            }
                        }
                    }
                    Adt17::C16(_, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                }
            };
            match step {
                Ok(next) => { m1038 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f158(v0: Option<Adt12>, v1: Adt12) -> Option<Adt12> {
    let m1040 = v0;
    match m1040 {
        None => {
            Some(v1)
        }
        Some(v2) => {
            let m1041 = f180(v1.clone(), v2.clone());
            if m1041 {
                Some(v1)
            } else {
                Some(v2)
            }
        }
    }
}

pub fn f159(v0: List<(Adt12, List<u64>)>, v1: Adt12, v2: List<u64>) -> bool {
    let m1042 = f96(v0, v1);
    match m1042 {
        None => {
            false
        }
        Some(v3) => {
            let v4: bool = f181(v2.clone(), v3.clone());
            let v5: bool = f181(v3, v2);
            if v4 {
                v5
            } else {
                false
            }
        }
    }
}

pub fn f160(v0: Adt12) -> u64 {
    let m1043 = v0;
    match m1043 {
        Adt12::C0(_) => {
            0u64
        }
        Adt12::C1(_) => {
            1u64
        }
        Adt12::C2(_) => {
            2u64
        }
        Adt12::C3(_) => {
            3u64
        }
        Adt12::C4(_) => {
            4u64
        }
        Adt12::C5(_) => {
            5u64
        }
        Adt12::C6(_) => {
            6u64
        }
        Adt12::C7(_) => {
            7u64
        }
        Adt12::C8(_) => {
            8u64
        }
        Adt12::C9(_) => {
            9u64
        }
    }
}

pub fn f161(v0: Adt12) -> u64 {
    let m1044 = v0;
    match m1044 {
        Adt12::C0(v1) => {
            v1
        }
        Adt12::C1(v2) => {
            v2
        }
        Adt12::C2(v3) => {
            v3
        }
        Adt12::C3(v4) => {
            v4
        }
        Adt12::C4(v5) => {
            v5
        }
        Adt12::C5(v6) => {
            v6
        }
        Adt12::C6(v7) => {
            v7
        }
        Adt12::C7(v8) => {
            v8
        }
        Adt12::C8(v9) => {
            v9
        }
        Adt12::C9(_) => {
            0u64
        }
    }
}

pub fn f162(v0: Adt12) -> Bytes {
    let m1045 = v0;
    match m1045 {
        Adt12::C0(_) => {
            Bytes::lit(&[])
        }
        Adt12::C1(_) => {
            Bytes::lit(&[])
        }
        Adt12::C2(_) => {
            Bytes::lit(&[])
        }
        Adt12::C3(_) => {
            Bytes::lit(&[])
        }
        Adt12::C4(_) => {
            Bytes::lit(&[])
        }
        Adt12::C5(_) => {
            Bytes::lit(&[])
        }
        Adt12::C6(_) => {
            Bytes::lit(&[])
        }
        Adt12::C7(_) => {
            Bytes::lit(&[])
        }
        Adt12::C8(_) => {
            Bytes::lit(&[])
        }
        Adt12::C9(v10) => {
            v10
        }
    }
}

pub fn f163(v0: Fn14, p1049: bool, p1050: List<u64>) -> bool {
    let m1048 = (p1049, p1050);
    {
        let mut m1048 = m1048;
        loop {
            let step = {
                let (v1, v2) = m1048;
                let m1046 = v2;
                match m1046.uncons() {
                    None => {
                        Err::<(bool, List<u64>), bool>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<u64>), bool>(({
                            let c1047: Fn14 = v0.clone();
                            c1047.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1048 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f164(v0: u64, v1: bool, v2: u64) -> bool {
    if v1 {
        true
    } else {
        let a1051 = v2;
        let a1052 = v0;
        nat_eq(a1051, a1052)
    }
}

pub fn f165(v0: Adt12, v1: List<(Adt12, List<u64>)>, v2: (Adt12, List<u64>)) -> List<(Adt12, List<u64>)> {
    let v3: bool = f125({
        let (h1053, _) = v2.clone();
        h1053
    }, v0);
    if v3 {
        v1
    } else {
        List::cons(v2, v1)
    }
}

pub fn f166(v0: Bytes, v1: Bytes) -> bool {
    let a1058 = {
        let a1054 = v0;
        let a1055 = v1;
        compare_bytes(a1054, a1055)
    };
    let a1059 = {
        let a1056 = Bytes::lit(&[]);
        let a1057 = Bytes::lit(&[0x01]);
        compare_bytes(a1056, a1057)
    };
    equal(a1058, a1059)
}

pub fn f167(v0: List<Adt29>, v1: Adt29) -> bool {
    f182(Fn16::F183(v1), false, v0)
}

pub fn f168(v0: Bytes, v1: Bytes) -> bool {
    let m1064 = {
        let a1061 = v0;
        let a1062 = 0u64;
        let a1063 = {
            let a1060 = v1.clone();
            length_bytes(a1060)
        };
        slice_bytes(a1061, a1062, a1063)
    };
    match m1064 {
        None => {
            false
        }
        Some(v2) => {
            let a1065 = v2;
            let a1066 = v1;
            equal(a1065, a1066)
        }
    }
}

pub fn f169(v0: Bytes, v1: Bytes) -> R<Option<u64>> {
    let v2: u64 = {
        let a1067 = v0.clone();
        length_bytes(a1067)
    };
    let v3: u64 = {
        let a1068 = v1.clone();
        length_bytes(a1068)
    };
    let v4: u64 = {
        let (h1069, _) = f38(Fn2::F184(v1.clone(), v3, v0.clone(), v2), {
            let a1070 = v2;
            let a1071 = 1u64;
            nat_add(a1070, a1071)?
        }, 0u64)?;
        h1069
    };
    if f185(v0, v1, v4) {
        Ok(Some(v4))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f170(v0: Adt24) -> bool {
    let m1072 = v0;
    match m1072 {
        Adt24::C0 => {
            false
        }
        Adt24::C1 => {
            false
        }
        Adt24::C2 => {
            true
        }
        Adt24::C3 => {
            false
        }
    }
}

pub fn f171(v0: Adt14) -> Option<Bytes> {
    let m1073 = v0;
    match m1073 {
        Adt14::C0(v1) => {
            f186(v1)
        }
        Adt14::C1(_) => {
            None::<Bytes>
        }
        Adt14::C2(_) => {
            None::<Bytes>
        }
        Adt14::C3(v4, _) => {
            Some(f187(v4))
        }
        Adt14::C4(_, _, _) => {
            Some(f1(Str::lit("match:adt")))
        }
        Adt14::C5 => {
            None::<Bytes>
        }
        Adt14::C6(_, _) => {
            None::<Bytes>
        }
    }
}

pub fn f172(v0: Adt30, v1: Bytes) -> Adt30 {
    Adt30::C0(match v0.clone() {
        Adt30::C0(h1074, _, _) => {
            h1074
        }
    }, f188(match v0.clone() {
        Adt30::C0(_, h1075, _) => {
            h1075
        }
    }, v1), match v0 {
        Adt30::C0(_, _, h1076) => {
            h1076
        }
    })
}

pub fn f173(v0: Adt3) -> Bytes {
    let m1077 = v0;
    match m1077 {
        Adt3::C0 => {
            f1(Str::lit("unit"))
        }
        Adt3::C1 => {
            f1(Str::lit("bool"))
        }
        Adt3::C2 => {
            f1(Str::lit("nat"))
        }
        Adt3::C3 => {
            f1(Str::lit("int"))
        }
        Adt3::C4 => {
            f1(Str::lit("ordering"))
        }
        Adt3::C5 => {
            f1(Str::lit("string"))
        }
        Adt3::C6 => {
            f1(Str::lit("bytes"))
        }
        Adt3::C7 => {
            f1(Str::lit("overflow"))
        }
        Adt3::C8 => {
            f1(Str::lit("i128"))
        }
    }
}

pub fn f174(v0: Adt5) -> Bytes {
    let m1078 = v0;
    match m1078 {
        Adt5::C0 => {
            f1(Str::lit("option"))
        }
        Adt5::C1 => {
            f1(Str::lit("list"))
        }
        Adt5::C2 => {
            f1(Str::lit("rc"))
        }
        Adt5::C3 => {
            f1(Str::lit("fallible"))
        }
        Adt5::C4 => {
            f1(Str::lit("ref"))
        }
    }
}

pub fn f175(v0: Adt6) -> Bytes {
    let m1079 = v0;
    match m1079 {
        Adt6::C0 => {
            f1(Str::lit("result"))
        }
        Adt6::C1 => {
            f1(Str::lit("pair"))
        }
    }
}

pub fn f176(v0: Adt7) -> Bytes {
    let m1080 = v0;
    match m1080 {
        Adt7::C0 => {
            f1(Str::lit("adt"))
        }
        Adt7::C1 => {
            f1(Str::lit("fn"))
        }
        Adt7::C2 => {
            f1(Str::lit("param"))
        }
    }
}

pub fn f177(v0: Fn15, p1084: List<u64>, p1085: List<u64>) -> List<u64> {
    let m1083 = (p1084, p1085);
    {
        let mut m1083 = m1083;
        loop {
            let step = {
                let (v1, v2) = m1083;
                let m1081 = v2;
                match m1081.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>(({
                            let c1082: Fn15 = v0.clone();
                            c1082.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1083 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f178(v0: List<u64>, v1: u64) -> List<u64> {
    let v2: bool = f131(v0.clone(), v1);
    if v2 {
        v0
    } else {
        List::cons(v1, v0)
    }
}

pub fn f179(v0: Adt17) -> R<bool> {
    let m1086 = v0;
    match m1086 {
        Adt17::C0(_, _) => {
            Ok(false)
        }
        Adt17::C1(_, _, _) => {
            Ok(false)
        }
        Adt17::C2(v6, _, _) => {
            let m1087 = v6;
            match m1087 {
                Adt20::C0 => {
                    Ok(false)
                }
                Adt20::C1 => {
                    Ok(false)
                }
                Adt20::C2 => {
                    Ok(false)
                }
                Adt20::C3 => {
                    Ok(true)
                }
            }
        }
        Adt17::C3(_, _, v11, _) => {
            Ok(!v11)
        }
        Adt17::C4(_, _, v15, _) => {
            Ok(!v15)
        }
        Adt17::C5(v17, r1088, _) => {
            let v18 = (*r1088).clone();
            f77(v17, v18)
        }
        Adt17::C6(_, _, _) => {
            Ok(false)
        }
        Adt17::C7(_, _, _, _) => {
            Ok(true)
        }
        Adt17::C8(_, _, _) => {
            Ok(true)
        }
        Adt17::C9(_) => {
            Ok(true)
        }
        Adt17::C10(_, _, _) => {
            Ok(false)
        }
        Adt17::C11(_, _) => {
            Ok(false)
        }
        Adt17::C12 => {
            Ok(false)
        }
        Adt17::C13(_, _) => {
            Ok(false)
        }
        Adt17::C14(_, _, _, _, _, _) => {
            Ok(false)
        }
        Adt17::C15(_, _, _) => {
            Ok(false)
        }
        Adt17::C16(_, _) => {
            Ok(true)
        }
    }
}

pub fn f180(v0: Adt12, v1: Adt12) -> bool {
    let a1089 = f160(v0.clone());
    let a1090 = f160(v1.clone());
    if nat_eq(a1089, a1090) {
        let a1091 = f160(v0.clone());
        let a1092 = 9u64;
        if nat_eq(a1091, a1092) {
            f166(f162(v0), f162(v1))
        } else {
            let a1093 = f161(v0);
            let a1094 = f161(v1);
            nat_lt(a1093, a1094)
        }
    } else {
        let a1095 = f160(v0);
        let a1096 = f160(v1);
        nat_lt(a1095, a1096)
    }
}

pub fn f181(v0: List<u64>, v1: List<u64>) -> bool {
    f163(Fn14::F189(v1), true, v0)
}

pub fn f182(v0: Fn16, p1100: bool, p1101: List<Adt29>) -> bool {
    let m1099 = (p1100, p1101);
    {
        let mut m1099 = m1099;
        loop {
            let step = {
                let (v1, v2) = m1099;
                let m1097 = v2;
                match m1097.uncons() {
                    None => {
                        Err::<(bool, List<Adt29>), bool>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<Adt29>), bool>(({
                            let c1098: Fn16 = v0.clone();
                            c1098.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1099 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f183(v0: Adt29, v1: bool, v2: Adt29) -> bool {
    let v3: bool = f190(v2, v0);
    if v1 {
        true
    } else {
        v3
    }
}

pub fn f184(v0: Bytes, v1: u64, v2: Bytes, v3: u64, v4: u64) -> R<Option<u64>> {
    if f185(v2, v0, v4) {
        Ok(None::<u64>)
    } else {
        let a1104 = {
            let a1102 = v4;
            let a1103 = v1;
            nat_add(a1102, a1103)?
        };
        let a1105 = v3;
        if nat_lt(a1104, a1105) {
            Ok(Some({
                let a1106 = v4;
                let a1107 = 1u64;
                nat_add(a1106, a1107)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f185(v0: Bytes, v1: Bytes, v2: u64) -> bool {
    let m1112 = {
        let a1109 = v0;
        let a1110 = v2;
        let a1111 = {
            let a1108 = v1.clone();
            length_bytes(a1108)
        };
        slice_bytes(a1109, a1110, a1111)
    };
    match m1112 {
        None => {
            false
        }
        Some(v3) => {
            let a1113 = v3;
            let a1114 = v1;
            equal(a1113, a1114)
        }
    }
}

pub fn f186(v0: Adt15) -> Option<Bytes> {
    let m1115 = v0;
    match m1115 {
        Adt15::C0 => {
            None::<Bytes>
        }
        Adt15::C1 => {
            None::<Bytes>
        }
        Adt15::C2 => {
            Some(f1(Str::lit("match:option")))
        }
        Adt15::C3 => {
            Some(f1(Str::lit("match:ordering")))
        }
    }
}

pub fn f187(v0: Adt16) -> Bytes {
    let m1116 = v0;
    match m1116 {
        Adt16::C0 => {
            f1(Str::lit("match:option"))
        }
        Adt16::C1 => {
            f1(Str::lit("match:result"))
        }
        Adt16::C2 => {
            f1(Str::lit("match:result"))
        }
    }
}

pub fn f188(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    let m1117 = f191(v0.clone(), v1.clone());
    if m1117 {
        v0
    } else {
        let v2: (bool, List<Bytes>) = f192(Fn17::F193(v1.clone()), (false, List::<Bytes>::nil()), v0);
        let m1119 = {
            let (h1118, _) = v2;
            h1118
        };
        if m1119 {
            f194({
                let (_, h1120) = v2;
                h1120
            })
        } else {
            f194(List::cons(v1, {
                let (_, h1121) = v2;
                h1121
            }))
        }
    }
}

pub fn f189(v0: List<u64>, v1: bool, v2: u64) -> bool {
    let v3: bool = f131(v0, v2);
    if v1 {
        v3
    } else {
        false
    }
}

pub fn f190(v0: Adt29, v1: Adt29) -> bool {
    let a1124 = match v0.clone() {
        Adt29::C0(h1122, _, _, _) => {
            h1122
        }
    };
    let a1125 = match v1.clone() {
        Adt29::C0(h1123, _, _, _) => {
            h1123
        }
    };
    if equal(a1124, a1125) {
        let a1128 = match v0.clone() {
            Adt29::C0(_, h1126, _, _) => {
                h1126
            }
        };
        let a1129 = match v1.clone() {
            Adt29::C0(_, h1127, _, _) => {
                h1127
            }
        };
        if equal(a1128, a1129) {
            if f195(match v0.clone() {
                Adt29::C0(_, _, h1130, _) => {
                    h1130
                }
            }, match v1.clone() {
                Adt29::C0(_, _, h1131, _) => {
                    h1131
                }
            }) {
                f195(match v0 {
                    Adt29::C0(_, _, _, h1132) => {
                        h1132
                    }
                }, match v1 {
                    Adt29::C0(_, _, _, h1133) => {
                        h1133
                    }
                })
            } else {
                false
            }
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f191(v0: List<Bytes>, v1: Bytes) -> bool {
    f196(Fn18::F197(v1), false, v0)
}

pub fn f192(v0: Fn17, p1137: (bool, List<Bytes>), p1138: List<Bytes>) -> (bool, List<Bytes>) {
    let m1136 = (p1137, p1138);
    {
        let mut m1136 = m1136;
        loop {
            let step = {
                let (v1, v2) = m1136;
                let m1134 = v2;
                match m1134.uncons() {
                    None => {
                        Err::<((bool, List<Bytes>), List<Bytes>), (bool, List<Bytes>)>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<((bool, List<Bytes>), List<Bytes>), (bool, List<Bytes>)>(({
                            let c1135: Fn17 = v0.clone();
                            c1135.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1136 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f193(v0: Bytes, v1: (bool, List<Bytes>), v2: Bytes) -> (bool, List<Bytes>) {
    f198(v0, v1, v2)
}

pub fn f194(v0: List<Bytes>) -> List<Bytes> {
    f199(Fn19::F200, List::<Bytes>::nil(), v0)
}

pub fn f195(v0: Option<Adt4>, v1: Option<Adt4>) -> bool {
    let a1139 = f201(v0);
    let a1140 = f201(v1);
    equal(a1139, a1140)
}

pub fn f196(v0: Fn18, p1144: bool, p1145: List<Bytes>) -> bool {
    let m1143 = (p1144, p1145);
    {
        let mut m1143 = m1143;
        loop {
            let step = {
                let (v1, v2) = m1143;
                let m1141 = v2;
                match m1141.uncons() {
                    None => {
                        Err::<(bool, List<Bytes>), bool>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<Bytes>), bool>(({
                            let c1142: Fn18 = v0.clone();
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

pub fn f197(v0: Bytes, v1: bool, v2: Bytes) -> bool {
    if v1 {
        true
    } else {
        let a1146 = v2;
        let a1147 = v0;
        equal(a1146, a1147)
    }
}

pub fn f198(v0: Bytes, v1: (bool, List<Bytes>), v2: Bytes) -> (bool, List<Bytes>) {
    let v3: bool = {
        let (h1148, _) = v1;
        h1148
    };
    let v4: List<Bytes> = {
        let (_, h1149) = v1;
        h1149
    };
    let m1150 = v3;
    if m1150 {
        (true, List::cons(v2, v4))
    } else {
        let m1151 = f166(v0.clone(), v2.clone());
        if m1151 {
            (true, List::cons(v2, List::cons(v0, v4)))
        } else {
            (false, List::cons(v2, v4))
        }
    }
}

pub fn f199(v0: Fn19, p1155: List<Bytes>, p1156: List<Bytes>) -> List<Bytes> {
    let m1154 = (p1155, p1156);
    {
        let mut m1154 = m1154;
        loop {
            let step = {
                let (v1, v2) = m1154;
                let m1152 = v2;
                match m1152.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c1153: Fn19 = v0.clone();
                            c1153.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1154 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f200(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f201(v0: Option<Adt4>) -> Bytes {
    let m1157 = v0;
    match m1157 {
        None => {
            f1(Str::lit("None"))
        }
        Some(v1) => {
            f4(f1(Str::lit("Some(")), f4(f14(v1), f1(Str::lit(")"))))
        }
    }
}

pub fn run(p0: Adt0) -> R<Result<(), Bytes>> {
    f0(p0)
}
