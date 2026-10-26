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
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt1 {
    C0(Adt0),
    C1(Adt0),
}

#[derive(Clone)]
pub enum Adt2 {
    C0,
    C1(u64),
}

#[derive(Clone)]
pub enum Adt3 {
    C0((Adt0, u64), List<Adt2>, Adt4),
}

#[derive(Clone)]
pub enum Adt4 {
    C0(u64, u64, u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Fn0 {
    F16,
}

impl Fn0 {
    pub fn apply(&self, p0: (u64, (u64, i64)), p1: i64) -> R<(u64, (u64, i64))> {
        match self {
            Fn0::F16 => f16(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F23(Bytes),
    F24(u64, Bytes),
    F69(Bytes, u64, Bytes, u64),
}

impl Fn1 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn1::F23(k0) => Ok(f23(k0.clone(), p0)),
            Fn1::F24(k0, k1) => f24(*k0, k1.clone(), p0),
            Fn1::F69(k0, k1, k2, k3) => f69(k0.clone(), *k1, k2.clone(), *k3, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F35,
}

impl Fn2 {
    pub fn apply(&self, p0: (u64, (i64, i64)), p1: i64) -> R<(u64, (i64, i64))> {
        match self {
            Fn2::F35 => f35(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F41(List<i64>),
}

impl Fn3 {
    pub fn apply(&self, p0: (List<i64>, List<i64>), p1: List<i64>) -> R<(List<i64>, List<i64>)> {
        match self {
            Fn3::F41(k0) => f41(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F47,
}

impl Fn4 {
    pub fn apply(&self, p0: Adt3) -> R<Option<Adt3>> {
        match self {
            Fn4::F47 => f47(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F52,
}

impl Fn5 {
    pub fn apply(&self, p0: (i64, List<i64>), p1: i64) -> R<(i64, List<i64>)> {
        match self {
            Fn5::F52 => f52(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F63((Adt0, u64)),
}

impl Fn6 {
    pub fn apply(&self, p0: Option<Adt2>, p1: u64) -> Option<Adt2> {
        match self {
            Fn6::F63(k0) => f63(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F68((Adt0, u64)),
}

impl Fn7 {
    pub fn apply(&self, p0: (bool, u64), p1: u64) -> R<(bool, u64)> {
        match self {
            Fn7::F68(k0) => f68(k0.clone(), p0, p1),
        }
    }
}

pub fn f0(v0: Bytes) -> R<Bytes> {
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
            let v4: Adt0 = f4(f5(v3.clone())?);
            let m2 = f6(if f7(f8(f9(), v3)?) {
                Adt1::C0(v4)
            } else {
                Adt1::C1(v4)
            });
            match m2 {
                None => {
                    Ok(Bytes::lit(&[0x68, 0x75, 0x6d, 0x61, 0x6e]))
                }
                Some(v5) => {
                    f10(v5)
                }
            }
        }
        Err(_) => {
            Ok(Bytes::lit(&[0x72, 0x65, 0x66, 0x75, 0x73, 0x65, 0x64]))
        }
    }
}

pub fn f1(v0: Bytes) -> R<Bytes> {
    let v1: u64 = f11(v0.clone())?;
    let v2: u64 = f12(v0.clone(), v1)?;
    Ok(f13(v0, v2, {
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
    f14(v0)
}

pub fn f4(v0: u64) -> Adt0 {
    let a8 = v0;
    let a9 = 0u64;
    if nat_eq(a8, a9) {
        Adt0::C0
    } else {
        let a10 = v0;
        let a11 = 1u64;
        if nat_eq(a10, a11) {
            Adt0::C1
        } else {
            Adt0::C2
        }
    }
}

pub fn f5(v0: List<i64>) -> R<u64> {
    let (h12, _) = {
        let (_, h13) = f15(Fn0::F16, (0u64, (0u64, 0i64)), v0)?;
        h13
    };
    Ok(h12)
}

pub fn f6(v0: Adt1) -> Option<Adt0> {
    f17(v0)
}

pub fn f7(v0: Option<u64>) -> bool {
    let m14 = v0;
    match m14 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f8(v0: i64, v1: List<i64>) -> R<Option<u64>> {
    let a16 = {
        let a15 = v1.clone();
        length_list(a15)
    };
    let a17 = 2u64;
    if nat_lt(a16, a17) {
        Ok(None::<u64>)
    } else {
        let m20 = {
            let a18 = f18(v1.clone())?;
            let a19 = v0;
            compare(a18, a19)
        };
        if match m20 {
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
            Ok(Some(f5(v1)?))
        }
    }
}

pub fn f9() -> i64 {
    2i64
}

pub fn f10(v0: Adt0) -> R<Bytes> {
    let a23 = f19(v0.clone());
    let a24 = {
        let a21 = Bytes::lit(&[0x3a]);
        let a22 = f20(f21(v0)?);
        append_bytes(a21, a22)
    };
    Ok(append_bytes(a23, a24))
}

pub fn f11(v0: Bytes) -> R<u64> {
    let (h25, _) = f22(Fn1::F23(v0.clone()), {
        let a26 = v0.clone();
        length_bytes(a26)
    }, {
        let a27 = v0;
        length_bytes(a27)
    })?;
    Ok(h25)
}

pub fn f12(v0: Bytes, v1: u64) -> R<u64> {
    let (h28, _) = f22(Fn1::F24(v1, v0), v1, 0u64)?;
    Ok(h28)
}

pub fn f13(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m32 = {
        let a29 = v0;
        let a30 = v1;
        let a31 = v2;
        slice_bytes(a29, a30, a31)
    };
    match m32 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f14(v0: Bytes) -> R<List<i64>> {
    let v1: List<i64> = f25(v0)?;
    Ok(v1)
}

pub fn f15(v0: Fn0, p36: (u64, (u64, i64)), p37: List<i64>) -> R<(u64, (u64, i64))> {
    let m35 = (p36, p37);
    {
        let mut m35 = m35;
        loop {
            let step = {
                let (v1, v2) = m35;
                let m33 = v2;
                match m33.uncons() {
                    None => {
                        Err::<((u64, (u64, i64)), List<i64>), R<(u64, (u64, i64))>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<((u64, (u64, i64)), List<i64>), R<(u64, (u64, i64))>>(({
                            let c34: Fn0 = v0.clone();
                            c34.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m35 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f16(v0: (u64, (u64, i64)), v1: i64) -> R<(u64, (u64, i64))> {
    f26(v0, v1)
}

pub fn f17(v0: Adt1) -> Option<Adt0> {
    f27(v0)
}

pub fn f18(v0: List<i64>) -> R<i64> {
    let a39 = {
        let a38 = v0.clone();
        length_list(a38)
    };
    let a40 = 2u64;
    if nat_lt(a39, a40) {
        Ok(0i64)
    } else {
        let a43 = {
            let (h41, _) = f28(v0.clone())?;
            h41
        };
        let a44 = {
            let (_, h42) = f28(v0)?;
            h42
        };
        int_sub(a43, a44)
    }
}

pub fn f19(v0: Adt0) -> Bytes {
    let m45 = v0;
    match m45 {
        Adt0::C0 => {
            Bytes::lit(&[0x6f, 0x75, 0x74, 0x61, 0x67, 0x65])
        }
        Adt0::C1 => {
            Bytes::lit(&[0x62, 0x69, 0x6c, 0x6c, 0x69, 0x6e, 0x67])
        }
        Adt0::C2 => {
            Bytes::lit(&[0x61, 0x63, 0x63, 0x65, 0x73, 0x73])
        }
    }
}

pub fn f20(v0: u64) -> Bytes {
    let a46 = v0;
    let a47 = 0u64;
    if nat_eq(a46, a47) {
        Bytes::lit(&[0x30])
    } else {
        let a48 = v0;
        let a49 = 1u64;
        if nat_eq(a48, a49) {
            Bytes::lit(&[0x31])
        } else {
            let a50 = v0;
            let a51 = 2u64;
            if nat_eq(a50, a51) {
                Bytes::lit(&[0x32])
            } else {
                let a52 = v0;
                let a53 = 3u64;
                if nat_eq(a52, a53) {
                    Bytes::lit(&[0x33])
                } else {
                    Bytes::lit(&[0x2b])
                }
            }
        }
    }
}

pub fn f21(v0: Adt0) -> R<u64> {
    let m54 = f29((v0.clone(), f30(v0)))?;
    match m54 {
        Ok(v1) => {
            let a56 = {
                let (_, h55) = v1;
                h55
            };
            Ok(length_list(a56))
        }
        Err(_) => {
            Ok(0u64)
        }
    }
}

pub fn f22(v0: Fn1, p61: u64, p62: u64) -> R<(u64, bool)> {
    let m60 = (p61, p62);
    {
        let mut m60 = m60;
        loop {
            let step = {
                let (v1, v2) = m60;
                let m57 = v1;
                if m57 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m57 - 1;
                    let m59 = {
                        let c58: Fn1 = v0.clone();
                        c58.apply(v2)?
                    };
                    match m59 {
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
                Ok(next) => { m60 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f23(v0: Bytes, v1: u64) -> Option<u64> {
    let a63 = 0u64;
    let a64 = v1;
    if nat_lt(a63, a64) {
        if f31(v0, {
            let a65 = v1;
            let a66 = 1u64;
            nat_sub(a65, a66)
        }) {
            Some({
                let a67 = v1;
                let a68 = 1u64;
                nat_sub(a67, a68)
            })
        } else {
            None::<u64>
        }
    } else {
        None::<u64>
    }
}

pub fn f24(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    let a69 = v2;
    let a70 = v0;
    if nat_lt(a69, a70) {
        if f31(v1, v2) {
            Ok(Some({
                let a71 = v2;
                let a72 = 1u64;
                nat_add(a71, a72)?
            }))
        } else {
            Ok(None::<u64>)
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f25(v0: Bytes) -> R<List<i64>> {
    f32(f33(v0)?)
}

pub fn f26(v0: (u64, (u64, i64)), v1: i64) -> R<(u64, (u64, i64))> {
    let a74 = {
        let (h73, _) = v0;
        h73
    };
    let a75 = 0u64;
    if nat_eq(a74, a75) {
        Ok((1u64, (0u64, v1)))
    } else {
        let m80 = {
            let a78 = {
                let (_, h76) = {
                    let (_, h77) = v0;
                    h77
                };
                h76
            };
            let a79 = v1;
            compare(a78, a79)
        };
        if match m80 {
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
                let a82 = {
                    let (h81, _) = v0;
                    h81
                };
                let a83 = 1u64;
                nat_add(a82, a83)?
            }, ({
                let (h84, _) = v0;
                h84
            }, v1)))
        } else {
            Ok(({
                let a86 = {
                    let (h85, _) = v0;
                    h85
                };
                let a87 = 1u64;
                nat_add(a86, a87)?
            }, {
                let (_, h88) = v0;
                h88
            }))
        }
    }
}

pub fn f27(v0: Adt1) -> Option<Adt0> {
    let m89 = v0;
    match m89 {
        Adt1::C0(v1) => {
            Some(v1)
        }
        Adt1::C1(_) => {
            None::<Adt0>
        }
    }
}

pub fn f28(v0: List<i64>) -> R<(i64, i64)> {
    let (_, h90) = f34(Fn2::F35, (0u64, (0i64, 0i64)), v0)?;
    Ok(h90)
}

pub fn f29(v0: (Adt0, u64)) -> R<Result<(u64, List<Adt2>), (bool, bool)>> {
    let v1: (Adt3, bool) = f36(v0.clone())?;
    let m92 = {
        let (_, h91) = v1.clone();
        h91
    };
    if m92 {
        let m96 = f37(v0, {
            let m95 = {
                let (h94, _) = v1.clone();
                h94
            };
            match m95 {
                Adt3::C0(h93, _, _) => {
                    h93
                }
            }
        });
        match m96 {
            Ok(v2) => {
                Ok(Ok::<(u64, List<Adt2>), (bool, bool)>((v2, {
                    let m99 = {
                        let (h98, _) = v1;
                        h98
                    };
                    match m99 {
                        Adt3::C0(_, h97, _) => {
                            h97
                        }
                    }
                })))
            }
            Err(v3) => {
                Ok(Err::<(u64, List<Adt2>), (bool, bool)>(v3))
            }
        }
    } else {
        Ok(Err::<(u64, List<Adt2>), (bool, bool)>((false, false)))
    }
}

pub fn f30(v0: Adt0) -> u64 {
    let m100 = v0;
    match m100 {
        Adt0::C0 => {
            3u64
        }
        Adt0::C1 => {
            2u64
        }
        Adt0::C2 => {
            1u64
        }
    }
}

pub fn f31(v0: Bytes, v1: u64) -> bool {
    let m101 = f38(v0, v1);
    match m101 {
        None => {
            false
        }
        Some(v2) => {
            f39(v2)
        }
    }
}

pub fn f32(v0: List<i64>) -> R<List<i64>> {
    let (h102, _) = f40(Fn3::F41(v0), (List::<i64>::nil(), f42()), f43())?;
    Ok(h102)
}

pub fn f33(v0: Bytes) -> R<List<i64>> {
    f44(v0)
}

pub fn f34(v0: Fn2, p106: (u64, (i64, i64)), p107: List<i64>) -> R<(u64, (i64, i64))> {
    let m105 = (p106, p107);
    {
        let mut m105 = m105;
        loop {
            let step = {
                let (v1, v2) = m105;
                let m103 = v2;
                match m103.uncons() {
                    None => {
                        Err::<((u64, (i64, i64)), List<i64>), R<(u64, (i64, i64))>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<((u64, (i64, i64)), List<i64>), R<(u64, (i64, i64))>>(({
                            let c104: Fn2 = v0.clone();
                            c104.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m105 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f35(v0: (u64, (i64, i64)), v1: i64) -> R<(u64, (i64, i64))> {
    f45(v0, v1)
}

pub fn f36(v0: (Adt0, u64)) -> R<(Adt3, bool)> {
    f46(Fn4::F47, {
        let a109 = {
            let (_, h108) = v0.clone();
            h108
        };
        let a110 = 1u64;
        nat_add(a109, a110)?
    }, f48(v0))
}

pub fn f37(v0: (Adt0, u64), v1: (Adt0, u64)) -> Result<u64, (bool, bool)> {
    let m111 = f49(v0, v1.clone());
    match m111 {
        None => {
            let m112 = f50(v1);
            match m112 {
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

pub fn f38(v0: Bytes, v1: u64) -> Option<u8> {
    let a113 = v0;
    let a114 = v1;
    index_bytes(a113, a114)
}

pub fn f39(v0: u8) -> bool {
    let a115 = v0;
    let a116 = 32u8;
    if if if equal(a115, a116) {
        true
    } else {
        let a117 = v0;
        let a118 = 9u8;
        equal(a117, a118)
    } {
        true
    } else {
        let a119 = v0;
        let a120 = 10u8;
        equal(a119, a120)
    } {
        true
    } else {
        let a121 = v0;
        let a122 = 12u8;
        if equal(a121, a122) {
            true
        } else {
            let a123 = v0;
            let a124 = 13u8;
            equal(a123, a124)
        }
    }
}

pub fn f40(v0: Fn3, p128: (List<i64>, List<i64>), p129: List<List<i64>>) -> R<(List<i64>, List<i64>)> {
    let m127 = (p128, p129);
    {
        let mut m127 = m127;
        loop {
            let step = {
                let (v1, v2) = m127;
                let m125 = v2;
                match m125.uncons() {
                    None => {
                        Err::<((List<i64>, List<i64>), List<List<i64>>), R<(List<i64>, List<i64>)>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<((List<i64>, List<i64>), List<List<i64>>), R<(List<i64>, List<i64>)>>(({
                            let c126: Fn3 = v0.clone();
                            c126.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m127 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f41(v0: List<i64>, v1: (List<i64>, List<i64>), v2: List<i64>) -> R<(List<i64>, List<i64>)> {
    let m131 = {
        let (_, h130) = v1.clone();
        h130
    };
    match m131.uncons() {
        None => {
            Ok(v1)
        }
        Some((v3, v4)) => {
            Ok(({
                let a137 = {
                    let (h132, _) = v1;
                    h132
                };
                let a138 = List::cons({
                    let a135 = v3;
                    let a136 = {
                        let a134 = {
                            let (h133, _) = f51(Fn5::F52, (0i64, v0), v2)?;
                            h133
                        };
                        int_neg(a134)?
                    };
                    int_sub(a135, a136)?
                }, List::<i64>::nil());
                append_list(a137, a138)
            }, v4))
        }
    }
}

pub fn f42() -> List<i64> {
    List::cons(0i64, List::cons(0i64, List::cons(0i64, List::<i64>::nil())))
}

pub fn f43() -> List<List<i64>> {
    List::cons(List::cons(4i64, List::cons(0i64, List::cons(0i64, List::<i64>::nil()))), List::cons(List::cons(0i64, List::cons(3i64, List::cons(0i64, List::<i64>::nil()))), List::cons(List::cons(0i64, List::cons(0i64, List::cons(2i64, List::<i64>::nil()))), List::<List<i64>>::nil())))
}

pub fn f44(v0: Bytes) -> R<List<i64>> {
    Ok(List::cons(f53(if f54(v0.clone(), Bytes::lit(&[0x64, 0x6f, 0x77, 0x6e]))? {
        true
    } else {
        f54(v0.clone(), Bytes::lit(&[0x6f, 0x75, 0x74, 0x61, 0x67, 0x65]))?
    }), List::cons(f53(if f54(v0.clone(), Bytes::lit(&[0x69, 0x6e, 0x76, 0x6f, 0x69, 0x63, 0x65]))? {
        true
    } else {
        f54(v0.clone(), Bytes::lit(&[0x72, 0x65, 0x66, 0x75, 0x6e, 0x64]))?
    }), List::cons(f53(if f54(v0.clone(), Bytes::lit(&[0x70, 0x61, 0x73, 0x73, 0x77, 0x6f, 0x72, 0x64]))? {
        true
    } else {
        f54(v0, Bytes::lit(&[0x6c, 0x6f, 0x67, 0x69, 0x6e]))?
    }), List::<i64>::nil()))))
}

pub fn f45(v0: (u64, (i64, i64)), v1: i64) -> R<(u64, (i64, i64))> {
    let a140 = {
        let (h139, _) = v0;
        h139
    };
    let a141 = 0u64;
    if nat_eq(a140, a141) {
        Ok((1u64, (v1, 0i64)))
    } else {
        let a143 = {
            let (h142, _) = v0;
            h142
        };
        let a144 = 1u64;
        if nat_eq(a143, a144) {
            let m149 = {
                let a147 = {
                    let (h145, _) = {
                        let (_, h146) = v0;
                        h146
                    };
                    h145
                };
                let a148 = v1;
                compare(a147, a148)
            };
            if match m149 {
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
                    let (h150, _) = {
                        let (_, h151) = v0;
                        h151
                    };
                    h150
                })))
            } else {
                Ok((2u64, ({
                    let (h152, _) = {
                        let (_, h153) = v0;
                        h153
                    };
                    h152
                }, v1)))
            }
        } else {
            let m158 = {
                let a156 = {
                    let (h154, _) = {
                        let (_, h155) = v0;
                        h155
                    };
                    h154
                };
                let a157 = v1;
                compare(a156, a157)
            };
            if match m158 {
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
                    let a160 = {
                        let (h159, _) = v0;
                        h159
                    };
                    let a161 = 1u64;
                    nat_add(a160, a161)?
                }, (v1, {
                    let (h162, _) = {
                        let (_, h163) = v0;
                        h163
                    };
                    h162
                })))
            } else {
                Ok(({
                    let a165 = {
                        let (h164, _) = v0;
                        h164
                    };
                    let a166 = 1u64;
                    nat_add(a165, a166)?
                }, ({
                    let (h167, _) = {
                        let (_, h168) = v0;
                        h168
                    };
                    h167
                }, {
                    let m173 = {
                        let a171 = {
                            let (_, h169) = {
                                let (_, h170) = v0;
                                h170
                            };
                            h169
                        };
                        let a172 = v1;
                        compare(a171, a172)
                    };
                    if match m173 {
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
                        let (_, h174) = {
                            let (_, h175) = v0;
                            h175
                        };
                        h174
                    }
                })))
            }
        }
    }
}

pub fn f46(v0: Fn4, p180: u64, p181: Adt3) -> R<(Adt3, bool)> {
    let m179 = (p180, p181);
    {
        let mut m179 = m179;
        loop {
            let step = {
                let (v1, v2) = m179;
                let m176 = v1;
                if m176 == 0 {
                    Err::<(u64, Adt3), R<(Adt3, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m176 - 1;
                    let m178 = {
                        let c177: Fn4 = v0.clone();
                        c177.apply(v2.clone())?
                    };
                    match m178 {
                        None => {
                            Err::<(u64, Adt3), R<(Adt3, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt3), R<(Adt3, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m179 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f47(v0: Adt3) -> R<Option<Adt3>> {
    let m183 = f55(match v0.clone() {
        Adt3::C0(h182, _, _) => {
            h182
        }
    });
    match m183 {
        None => {
            Ok(None::<Adt3>)
        }
        Some(v1) => {
            let m185 = f56(match v0.clone() {
                Adt3::C0(h184, _, _) => {
                    h184
                }
            }, v1.clone());
            match m185 {
                None => {
                    Ok(None::<Adt3>)
                }
                Some(v2) => {
                    Ok(Some(Adt3::C0(v2, {
                        let a187 = match v0.clone() {
                            Adt3::C0(_, h186, _) => {
                                h186
                            }
                        };
                        let a188 = List::cons(v1, List::<Adt2>::nil());
                        append_list(a187, a188)
                    }, Adt4::C0({
                        let a192 = {
                            let m191 = match v0.clone() {
                                Adt3::C0(_, _, h190) => {
                                    h190
                                }
                            };
                            match m191 {
                                Adt4::C0(h189, _, _, _, _, _) => {
                                    h189
                                }
                            }
                        };
                        let a193 = 1u64;
                        nat_add(a192, a193)?
                    }, {
                        let a198 = {
                            let m196 = match v0.clone() {
                                Adt3::C0(_, _, h195) => {
                                    h195
                                }
                            };
                            match m196 {
                                Adt4::C0(_, h194, _, _, _, _) => {
                                    h194
                                }
                            }
                        };
                        let a199 = f57(match v0.clone() {
                            Adt3::C0(h197, _, _) => {
                                h197
                            }
                        })?;
                        nat_add(a198, a199)?
                    }, {
                        let a203 = {
                            let m202 = match v0.clone() {
                                Adt3::C0(_, _, h201) => {
                                    h201
                                }
                            };
                            match m202 {
                                Adt4::C0(_, _, h200, _, _, _) => {
                                    h200
                                }
                            }
                        };
                        let a204 = 1u64;
                        nat_add(a203, a204)?
                    }, {
                        let m207 = match v0.clone() {
                            Adt3::C0(_, _, h206) => {
                                h206
                            }
                        };
                        match m207 {
                            Adt4::C0(_, _, _, h205, _, _) => {
                                h205
                            }
                        }
                    }, {
                        let m210 = match v0.clone() {
                            Adt3::C0(_, _, h209) => {
                                h209
                            }
                        };
                        match m210 {
                            Adt4::C0(_, _, _, _, h208, _) => {
                                h208
                            }
                        }
                    }, {
                        let m213 = match v0 {
                            Adt3::C0(_, _, h212) => {
                                h212
                            }
                        };
                        match m213 {
                            Adt4::C0(_, _, _, _, _, h211) => {
                                h211
                            }
                        }
                    }))))
                }
            }
        }
    }
}

pub fn f48(v0: (Adt0, u64)) -> Adt3 {
    Adt3::C0(f58(v0), List::<Adt2>::nil(), Adt4::C0(0u64, 0u64, 0u64, 0u64, 0u64, 0u64))
}

pub fn f49(v0: (Adt0, u64), v1: (Adt0, u64)) -> Option<u64> {
    let m214 = f50(v1);
    match m214 {
        None => {
            None::<u64>
        }
        Some(v2) => {
            if f59(v0, v2) {
                Some(v2)
            } else {
                None::<u64>
            }
        }
    }
}

pub fn f50(v0: (Adt0, u64)) -> Option<u64> {
    Some({
        let (_, h215) = v0;
        h215
    })
}

pub fn f51(v0: Fn5, p219: (i64, List<i64>), p220: List<i64>) -> R<(i64, List<i64>)> {
    let m218 = (p219, p220);
    {
        let mut m218 = m218;
        loop {
            let step = {
                let (v1, v2) = m218;
                let m216 = v2;
                match m216.uncons() {
                    None => {
                        Err::<((i64, List<i64>), List<i64>), R<(i64, List<i64>)>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<((i64, List<i64>), List<i64>), R<(i64, List<i64>)>>(({
                            let c217: Fn5 = v0.clone();
                            c217.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m218 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f52(v0: (i64, List<i64>), v1: i64) -> R<(i64, List<i64>)> {
    let m222 = {
        let (_, h221) = v0.clone();
        h221
    };
    match m222.uncons() {
        None => {
            Ok(v0)
        }
        Some((v2, v3)) => {
            Ok(({
                let a227 = {
                    let (h223, _) = v0;
                    h223
                };
                let a228 = {
                    let a226 = {
                        let a224 = v1;
                        let a225 = v2;
                        int_mul(a224, a225)?
                    };
                    int_neg(a226)?
                };
                int_sub(a227, a228)?
            }, v3))
        }
    }
}

pub fn f53(v0: bool) -> i64 {
    if v0 {
        1i64
    } else {
        0i64
    }
}

pub fn f54(v0: Bytes, v1: Bytes) -> R<bool> {
    let m229 = f60(v0, v1)?;
    match m229 {
        None => {
            Ok(false)
        }
        Some(_) => {
            Ok(true)
        }
    }
}

pub fn f55(v0: (Adt0, u64)) -> Option<Adt2> {
    let m230 = if f61(v0.clone()) {
        Some(Adt2::C0)
    } else {
        None::<Adt2>
    };
    match m230 {
        None => {
            f62(Fn6::F63(v0.clone()), None::<Adt2>, f64(v0))
        }
        Some(v1) => {
            Some(v1)
        }
    }
}

pub fn f56(v0: (Adt0, u64), v1: Adt2) -> Option<(Adt0, u64)> {
    let m231 = v1;
    match m231 {
        Adt2::C0 => {
            f65(v0)
        }
        Adt2::C1(v2) => {
            f66(v0, v2)
        }
    }
}

pub fn f57(v0: (Adt0, u64)) -> R<u64> {
    let (_, h232) = {
        let v1: (bool, u64) = (f61(v0.clone()), 1u64);
        let (h233, _) = v1;
        if h233 {
            v1
        } else {
            let v2: (bool, u64) = f67(Fn7::F68(v0.clone()), (false, 0u64), f64(v0))?;
            ({
                let (h234, _) = v2;
                h234
            }, {
                let a237 = {
                    let (_, h235) = v1;
                    h235
                };
                let a238 = {
                    let (_, h236) = v2;
                    h236
                };
                nat_add(a237, a238)?
            })
        }
    };
    Ok(h232)
}

pub fn f58(v0: (Adt0, u64)) -> (Adt0, u64) {
    v0
}

pub fn f59(_: (Adt0, u64), v1: u64) -> bool {
    let a239 = v1;
    let a240 = 0u64;
    nat_eq(a239, a240)
}

pub fn f60(v0: Bytes, v1: Bytes) -> R<Option<u64>> {
    let v2: u64 = {
        let a241 = v0.clone();
        length_bytes(a241)
    };
    let v3: u64 = {
        let a242 = v1.clone();
        length_bytes(a242)
    };
    let v4: u64 = {
        let (h243, _) = f22(Fn1::F69(v1.clone(), v3, v0.clone(), v2), {
            let a244 = v2;
            let a245 = 1u64;
            nat_add(a244, a245)?
        }, 0u64)?;
        h243
    };
    if f70(v0, v1, v4) {
        Ok(Some(v4))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f61(v0: (Adt0, u64)) -> bool {
    let a247 = 2u64;
    let a248 = {
        let (_, h246) = v0;
        h246
    };
    nat_lt(a247, a248)
}

pub fn f62(v0: Fn6, p252: Option<Adt2>, p253: List<u64>) -> Option<Adt2> {
    let m251 = (p252, p253);
    {
        let mut m251 = m251;
        loop {
            let step = {
                let (v1, v2) = m251;
                let m249 = v2;
                match m249.uncons() {
                    None => {
                        Err::<(Option<Adt2>, List<u64>), Option<Adt2>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Adt2>, List<u64>), Option<Adt2>>(({
                            let c250: Fn6 = v0.clone();
                            c250.apply(v1, v3)
                        }, v4))
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

pub fn f63(v0: (Adt0, u64), v1: Option<Adt2>, v2: u64) -> Option<Adt2> {
    let m254 = v1;
    match m254 {
        None => {
            if f71(v0, v2) {
                Some(Adt2::C1(v2))
            } else {
                None::<Adt2>
            }
        }
        Some(v3) => {
            Some(v3)
        }
    }
}

pub fn f64(_: (Adt0, u64)) -> List<u64> {
    List::cons(2u64, List::cons(1u64, List::<u64>::nil()))
}

pub fn f65(v0: (Adt0, u64)) -> Option<(Adt0, u64)> {
    if f61(v0.clone()) {
        Some(f72(v0))
    } else {
        None::<(Adt0, u64)>
    }
}

pub fn f66(v0: (Adt0, u64), v1: u64) -> Option<(Adt0, u64)> {
    if f71(v0.clone(), v1) {
        Some(f73(v0, v1))
    } else {
        None::<(Adt0, u64)>
    }
}

pub fn f67(v0: Fn7, p258: (bool, u64), p259: List<u64>) -> R<(bool, u64)> {
    let m257 = (p258, p259);
    {
        let mut m257 = m257;
        loop {
            let step = {
                let (v1, v2) = m257;
                let m255 = v2;
                match m255.uncons() {
                    None => {
                        Err::<((bool, u64), List<u64>), R<(bool, u64)>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<((bool, u64), List<u64>), R<(bool, u64)>>(({
                            let c256: Fn7 = v0.clone();
                            c256.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m257 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f68(v0: (Adt0, u64), v1: (bool, u64), v2: u64) -> R<(bool, u64)> {
    let (h260, _) = v1;
    if h260 {
        Ok(v1)
    } else {
        Ok((f71(v0, v2), {
            let a262 = {
                let (_, h261) = v1;
                h261
            };
            let a263 = 1u64;
            nat_add(a262, a263)?
        }))
    }
}

pub fn f69(v0: Bytes, v1: u64, v2: Bytes, v3: u64, v4: u64) -> R<Option<u64>> {
    if f70(v2, v0, v4) {
        Ok(None::<u64>)
    } else {
        let a266 = {
            let a264 = v4;
            let a265 = v1;
            nat_add(a264, a265)?
        };
        let a267 = v3;
        if nat_lt(a266, a267) {
            Ok(Some({
                let a268 = v4;
                let a269 = 1u64;
                nat_add(a268, a269)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f70(v0: Bytes, v1: Bytes, v2: u64) -> bool {
    let m274 = {
        let a271 = v0;
        let a272 = v2;
        let a273 = {
            let a270 = v1.clone();
            length_bytes(a270)
        };
        slice_bytes(a271, a272, a273)
    };
    match m274 {
        None => {
            false
        }
        Some(v3) => {
            let a275 = v3;
            let a276 = v1;
            equal(a275, a276)
        }
    }
}

pub fn f71(v0: (Adt0, u64), v1: u64) -> bool {
    let a278 = v1;
    let a279 = {
        let (_, h277) = v0;
        h277
    };
    if nat_le(a278, a279) {
        let a280 = 0u64;
        let a281 = v1;
        nat_lt(a280, a281)
    } else {
        false
    }
}

pub fn f72(v0: (Adt0, u64)) -> (Adt0, u64) {
    ({
        let (h282, _) = v0;
        h282
    }, {
        let a284 = {
            let (_, h283) = v0;
            h283
        };
        let a285 = 1u64;
        nat_sub(a284, a285)
    })
}

pub fn f73(v0: (Adt0, u64), v1: u64) -> (Adt0, u64) {
    ({
        let (h286, _) = v0;
        h286
    }, {
        let a288 = {
            let (_, h287) = v0;
            h287
        };
        let a289 = v1;
        nat_sub(a288, a289)
    })
}

pub fn run(p0: Bytes) -> R<Bytes> {
    f0(p0)
}
