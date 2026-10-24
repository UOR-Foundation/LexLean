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
}
#[derive(Clone)]
pub struct Bytes(std::rc::Rc<[u8]>);
impl Bytes {
    pub fn lit(octets: &[u8]) -> Bytes { Bytes(std::rc::Rc::from(octets)) }
    pub fn octets(&self) -> &[u8] { &self.0 }
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

pub fn append_bytes(a: Bytes, b: Bytes) -> Bytes {
    tick((a.0.len() + b.0.len()) as u64);
    let mut out = Vec::with_capacity(a.0.len() + b.0.len());
    out.extend_from_slice(&a.0);
    out.extend_from_slice(&b.0);
    Bytes(std::rc::Rc::from(out))
}
pub fn length_bytes(a: Bytes) -> u64 { a.0.len() as u64 }
pub fn length_string(a: Str) -> u64 { chars(&a.0) }
pub fn index_bytes(a: Bytes, i: u64) -> Option<u8> { usize::try_from(i).ok().and_then(|i| a.0.get(i)).copied() }
pub fn slice_bytes(a: Bytes, start: u64, count: u64) -> Option<Bytes> {
    let end = u128::from(start) + u128::from(count);
    if end > a.0.len() as u128 { return None; }
    tick(count);
    Some(Bytes::lit(&a.0[start as usize..end as usize]))
}
pub fn utf8_encode(a: Str) -> Bytes { tick(a.0.len() as u64); Bytes::lit(a.0.as_bytes()) }
pub fn utf8_decode(a: Bytes) -> Option<Str> { tick(a.0.len() as u64); core::str::from_utf8(&a.0).ok().map(Str::lit) }
pub fn compare_bytes(a: Bytes, b: Bytes) -> Ordering { tick(1 + a.0.len().min(b.0.len()) as u64); a.0.cmp(&b.0) }
pub fn split_exact(text: Str, delimiter: Str, maximum: u32) -> Option<List<Str>> {
    if delimiter.0.is_empty() { return None; }
    chars(&text.0);
    let fields: Vec<Str> = text.0.split(&*delimiter.0).map(Str::lit).collect();
    if fields.len() as u128 > u128::from(maximum) { return None; }
    Some(List::onto(fields, List::nil()))
}
pub fn join(texts: List<Str>, delimiter: Str) -> Str {
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

#[derive(Clone)]
pub enum Adt0 {
    C0,
    C1,
    C2,
    C3,
    C4(Adt1),
    C5,
    C6,
    C7,
    C8(std::rc::Rc<Adt0>),
    C9(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
    C10(std::rc::Rc<Adt0>),
    C11(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
    C12(u64),
    C13(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
    C14,
    C15(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
}

#[derive(Clone)]
pub enum Adt1 {
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
pub enum Adt2 {
    C0(Adt0, Adt3),
    C1(u64),
    C2(u64, Adt0, std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C3(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C4(Adt0, std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C5(Adt5, Adt0, std::rc::Rc<Adt2>),
    C6(u64, std::rc::Rc<Adt2>),
    C7(u64, std::rc::Rc<Adt2>),
    C8(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C9(Adt6, std::rc::Rc<Adt2>),
    C10(std::rc::Rc<Adt2>),
    C11(std::rc::Rc<Adt2>),
    C12(std::rc::Rc<Adt2>, u64),
    C13,
    C14(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C15(Adt5, List<u64>, std::rc::Rc<Adt2>),
}

#[derive(Clone)]
pub enum Adt3 {
    C0,
    C1(bool),
    C2(Bytes),
    C3(Bytes),
    C4(Bytes),
    C5(Bytes),
    C6(Bytes),
    C7(Bytes),
    C8(Bytes),
    C9(Bytes),
    C10(Bytes),
    C11(Bytes),
    C12(Bytes),
    C13(Bytes),
    C14(Adt4),
    C15,
    C16(std::rc::Rc<Adt3>),
    C17(std::rc::Rc<Adt3>),
    C18(std::rc::Rc<Adt3>),
    C19(std::rc::Rc<Adt3>),
    C20(std::rc::Rc<Adt3>, std::rc::Rc<Adt3>),
    C21(u64, std::rc::Rc<Adt3>),
    C22(u64, std::rc::Rc<Adt3>),
    C23,
    C24(std::rc::Rc<Adt3>, std::rc::Rc<Adt3>),
}

#[derive(Clone)]
pub enum Adt4 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt5 {
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
    C10,
    C11,
    C12,
    C13,
    C14,
    C15(u64),
}

#[derive(Clone)]
pub enum Adt6 {
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
    C10,
    C11,
    C12,
    C13,
    C14,
    C15,
    C16,
    C17,
    C18,
    C19,
    C20,
    C21,
    C22,
    C23,
    C24,
    C25,
    C26,
    C27,
    C28,
    C29,
    C30,
    C31,
    C32,
    C33,
    C34,
    C35,
    C36,
    C37,
    C38,
    C39,
    C40(Adt1),
    C41(Adt0),
}

#[derive(Clone)]
pub enum Adt7 {
    C0(Bytes, Adt0, List<List<u64>>, Adt0, Adt0, Adt2),
}

#[derive(Clone)]
pub enum Adt8 {
    C0(Bytes),
    C1(Adt0),
    C2(List<Adt5>),
    C3,
}

#[derive(Clone)]
pub enum Adt9 {
    C0(bool, Bytes),
}

#[derive(Clone)]
pub enum Fn0 {
    F24(Bytes),
    F93(Bytes),
    F99(Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn0::F24(k0) => f24(k0.clone(), p0),
            Fn0::F93(k0) => f93(k0.clone(), p0),
            Fn0::F99(k0) => f99(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F32,
}

impl Fn1 {
    pub fn apply(&self, p0: u64, p1: u64) -> R<u64> {
        match self {
            Fn1::F32 => f32(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F89(Bytes),
}

impl Fn2 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn2::F89(k0) => f89(k0.clone(), p0),
        }
    }
}

pub fn f0(v0: Bytes, v1: Adt0, v2: List<List<u64>>, v3: Adt0, v4: Adt0, v5: Adt2) -> R<Result<(), Bytes>> {
    let v6: Adt7 = Adt7::C0(v0, v1, v2, v3, v4, v5);
    let a2 = match v6.clone() {
        Adt7::C0(h1, _, _, _, _, _) => {
            h1
        }
    };
    let a3 = f1(Str::lit("lexlean/target-program/1"));
    if equal(a2, a3) {
        let m5 = f2(v6.clone(), match v6.clone() {
            Adt7::C0(_, h4, _, _, _, _) => {
                h4
            }
        }, 0u64)?;
        match m5 {
            Ok(_) => {
                f3(v6.clone(), match v6 {
                    Adt7::C0(_, _, _, _, _, h6) => {
                        h6
                    }
                }, 0u64)
            }
            Err(v8) => {
                Ok(Err::<(), Bytes>(v8))
            }
        }
    } else {
        Ok(Err::<(), Bytes>(f4(f1(Str::lit("target program has spec ")), f5(match v6 {
            Adt7::C0(h7, _, _, _, _, _) => {
                h7
            }
        }))))
    }
}

pub fn f1(v0: Str) -> Bytes {
    let a8 = v0;
    utf8_encode(a8)
}

pub fn f2(v0: Adt7, p16: Adt0, p17: u64) -> R<Result<(), Bytes>> {
    let m15 = (p16, p17);
    {
        let mut m15 = m15;
        loop {
            let step = {
                let (v1, v2) = m15;
                let m9 = v1;
                match m9 {
                    Adt0::C0 => {
                        Err::<(Adt0, u64), R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt0::C1 => {
                        Err::<(Adt0, u64), R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt0::C2 => {
                        Err::<(Adt0, u64), R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt0::C3 => {
                        Err::<(Adt0, u64), R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt0::C4(_) => {
                        Err::<(Adt0, u64), R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt0::C5 => {
                        Err::<(Adt0, u64), R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt0::C6 => {
                        Err::<(Adt0, u64), R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt0::C7 => {
                        Err::<(Adt0, u64), R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt0::C8(_) => {
                        Err::<(Adt0, u64), R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt0::C9(_, _) => {
                        Err::<(Adt0, u64), R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt0::C10(_) => {
                        Err::<(Adt0, u64), R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt0::C11(_, _) => {
                        Err::<(Adt0, u64), R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt0::C12(_) => {
                        Err::<(Adt0, u64), R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt0::C13(_, _) => {
                        Err::<(Adt0, u64), R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt0::C14 => {
                        Err::<(Adt0, u64), R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt0::C15(r13, r14) => {
                        let v13 = (*r13).clone();
                        let v14 = (*r14).clone();
                        if f6(v13.clone()) {
                            Err::<(Adt0, u64), R<Result<(), Bytes>>>(Ok(Err::<(), Bytes>(f4(f1(Str::lit("ADT ")), f4(f7(v2)?, f1(Str::lit(" has no constructor")))))))
                        } else {
                            let m10 = f8(v0.clone(), v13)?;
                            match m10 {
                                Ok(_) => {
                                    Ok::<(Adt0, u64), R<Result<(), Bytes>>>((v14, {
                                        let a11 = v2;
                                        let a12 = 1u64;
                                        nat_add(a11, a12)?
                                    }))
                                }
                                Err(v16) => {
                                    Err::<(Adt0, u64), R<Result<(), Bytes>>>(Ok(Err::<(), Bytes>(f4(f1(Str::lit("ADT ")), f4(f7(v2)?, f4(f1(Str::lit(": ")), v16))))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m15 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f3(v0: Adt7, p25: Adt2, p26: u64) -> R<Result<(), Bytes>> {
    let m24 = (p25, p26);
    {
        let mut m24 = m24;
        loop {
            let step = {
                let (v1, v2) = m24;
                let m18 = v1;
                match m18 {
                    Adt2::C0(_, _) => {
                        Err::<(Adt2, u64), R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt2::C1(_) => {
                        Err::<(Adt2, u64), R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt2::C2(_, _, _, _) => {
                        Err::<(Adt2, u64), R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt2::C3(_, _, _) => {
                        Err::<(Adt2, u64), R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt2::C4(_, _, _) => {
                        Err::<(Adt2, u64), R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt2::C5(_, _, _) => {
                        Err::<(Adt2, u64), R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt2::C6(_, _) => {
                        Err::<(Adt2, u64), R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt2::C7(_, _) => {
                        Err::<(Adt2, u64), R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt2::C8(_, _) => {
                        Err::<(Adt2, u64), R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt2::C9(_, _) => {
                        Err::<(Adt2, u64), R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt2::C10(_) => {
                        Err::<(Adt2, u64), R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt2::C11(_) => {
                        Err::<(Adt2, u64), R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt2::C12(_, _) => {
                        Err::<(Adt2, u64), R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt2::C13 => {
                        Err::<(Adt2, u64), R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt2::C14(r22, r23) => {
                        let v3 = (*r22).clone();
                        let v4 = (*r23).clone();
                        let m19 = f9(v0.clone(), v3, v2)?;
                        match m19 {
                            Ok(_) => {
                                Ok::<(Adt2, u64), R<Result<(), Bytes>>>((v4, {
                                    let a20 = v2;
                                    let a21 = 1u64;
                                    nat_add(a20, a21)?
                                }))
                            }
                            Err(v6) => {
                                Err::<(Adt2, u64), R<Result<(), Bytes>>>(Ok(Err::<(), Bytes>(v6)))
                            }
                        }
                    }
                    Adt2::C15(_, _, _) => {
                        Err::<(Adt2, u64), R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
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

pub fn f4(v0: Bytes, v1: Bytes) -> Bytes {
    let a27 = v0;
    let a28 = v1;
    append_bytes(a27, a28)
}

pub fn f5(v0: Bytes) -> Bytes {
    f4(f1(Str::lit("`")), f4(v0, f1(Str::lit("`"))))
}

pub fn f6(v0: Adt0) -> bool {
    let a29 = f10(v0);
    let a30 = 14u64;
    nat_eq(a29, a30)
}

pub fn f7(v0: u64) -> R<Bytes> {
    let v1: Bytes = f1(Str::lit("0"));
    let v2: Bytes = f11({
        let a67 = f12(v0, 10000000000000000000u64);
        let a68 = {
            let a65 = f12(v0, 1000000000000000000u64);
            let a66 = {
                let a63 = f12(v0, 100000000000000000u64);
                let a64 = {
                    let a61 = f12(v0, 10000000000000000u64);
                    let a62 = {
                        let a59 = f12(v0, 1000000000000000u64);
                        let a60 = {
                            let a57 = f12(v0, 100000000000000u64);
                            let a58 = {
                                let a55 = f12(v0, 10000000000000u64);
                                let a56 = {
                                    let a53 = f12(v0, 1000000000000u64);
                                    let a54 = {
                                        let a51 = f12(v0, 100000000000u64);
                                        let a52 = {
                                            let a49 = f12(v0, 10000000000u64);
                                            let a50 = {
                                                let a47 = f12(v0, 1000000000u64);
                                                let a48 = {
                                                    let a45 = f12(v0, 100000000u64);
                                                    let a46 = {
                                                        let a43 = f12(v0, 10000000u64);
                                                        let a44 = {
                                                            let a41 = f12(v0, 1000000u64);
                                                            let a42 = {
                                                                let a39 = f12(v0, 100000u64);
                                                                let a40 = {
                                                                    let a37 = f12(v0, 10000u64);
                                                                    let a38 = {
                                                                        let a35 = f12(v0, 1000u64);
                                                                        let a36 = {
                                                                            let a33 = f12(v0, 100u64);
                                                                            let a34 = {
                                                                                let a31 = f12(v0, 10u64);
                                                                                let a32 = f12(v0, 1u64);
                                                                                append_bytes(a31, a32)
                                                                            };
                                                                            append_bytes(a33, a34)
                                                                        };
                                                                        append_bytes(a35, a36)
                                                                    };
                                                                    append_bytes(a37, a38)
                                                                };
                                                                append_bytes(a39, a40)
                                                            };
                                                            append_bytes(a41, a42)
                                                        };
                                                        append_bytes(a43, a44)
                                                    };
                                                    append_bytes(a45, a46)
                                                };
                                                append_bytes(a47, a48)
                                            };
                                            append_bytes(a49, a50)
                                        };
                                        append_bytes(a51, a52)
                                    };
                                    append_bytes(a53, a54)
                                };
                                append_bytes(a55, a56)
                            };
                            append_bytes(a57, a58)
                        };
                        append_bytes(a59, a60)
                    };
                    append_bytes(a61, a62)
                };
                append_bytes(a63, a64)
            };
            append_bytes(a65, a66)
        };
        append_bytes(a67, a68)
    })?;
    let a69 = v0;
    let a70 = 0u64;
    if nat_eq(a69, a70) {
        Ok(v1)
    } else {
        Ok(v2)
    }
}

pub fn f8(v0: Adt7, p76: Adt0) -> R<Result<(), Bytes>> {
    let m75 = p76;
    {
        let mut m75 = m75;
        loop {
            let step = {
                let v1 = m75;
                let m71 = v1;
                match m71 {
                    Adt0::C0 => {
                        Err::<Adt0, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt0::C1 => {
                        Err::<Adt0, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt0::C2 => {
                        Err::<Adt0, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt0::C3 => {
                        Err::<Adt0, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt0::C4(_) => {
                        Err::<Adt0, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt0::C5 => {
                        Err::<Adt0, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt0::C6 => {
                        Err::<Adt0, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt0::C7 => {
                        Err::<Adt0, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt0::C8(_) => {
                        Err::<Adt0, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt0::C9(_, _) => {
                        Err::<Adt0, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt0::C10(_) => {
                        Err::<Adt0, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt0::C11(_, _) => {
                        Err::<Adt0, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt0::C12(_) => {
                        Err::<Adt0, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt0::C13(_, _) => {
                        Err::<Adt0, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt0::C14 => {
                        Err::<Adt0, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt0::C15(r73, r74) => {
                        let v12 = (*r73).clone();
                        let v13 = (*r74).clone();
                        let m72 = f13(v0.clone(), v12)?;
                        match m72 {
                            Ok(_) => {
                                Ok::<Adt0, R<Result<(), Bytes>>>(v13)
                            }
                            Err(v15) => {
                                Err::<Adt0, R<Result<(), Bytes>>>(Ok(Err::<(), Bytes>(v15)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m75 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f9(v0: Adt7, v1: Adt2, v2: u64) -> R<Result<(), Bytes>> {
    let v3: List<u64> = f14(v0.clone(), v2);
    let v4: Adt0 = f15(v0.clone(), v2);
    let v5: Adt0 = f16(v0.clone(), v2);
    let v6: u64 = f17(v3.clone())?;
    let v7: u64 = f18(v4.clone())?;
    let a77 = v6;
    let a78 = v7;
    if nat_eq(a77, a78) {
        if f19(v3.clone()) {
            Ok(Err::<(), Bytes>(f4(f1(Str::lit("function ")), f4(f7(v2)?, f4(f1(Str::lit(": ")), f1(Str::lit("a parameter name repeats")))))))
        } else {
            let m79 = f13(v0.clone(), v4.clone())?;
            match m79 {
                Ok(_) => {
                    let m80 = f13(v0.clone(), v5.clone())?;
                    match m80 {
                        Ok(_) => {
                            let m81 = f20(v0, v1, f21(v3, v4, List::<(u64, Adt0)>::nil()), 0u64, Adt0::C14, Adt0::C14, f1(Str::lit("")), 0u64, List::<Adt5>::nil())?;
                            match m81 {
                                Adt8::C0(v12) => {
                                    Ok(Err::<(), Bytes>(f4(f1(Str::lit("function ")), f4(f7(v2)?, f4(f1(Str::lit(": ")), v12)))))
                                }
                                Adt8::C1(v14) => {
                                    let m82 = f22(v14, v5, f1(Str::lit("the body")))?;
                                    match m82 {
                                        Ok(_) => {
                                            Ok(Ok::<(), Bytes>(()))
                                        }
                                        Err(v15) => {
                                            Ok(Err::<(), Bytes>(f4(f1(Str::lit("function ")), f4(f7(v2)?, f4(f1(Str::lit(": ")), v15)))))
                                        }
                                    }
                                }
                                Adt8::C2(_) => {
                                    Ok(Err::<(), Bytes>(f1(Str::lit("unreachable"))))
                                }
                                Adt8::C3 => {
                                    Ok(Err::<(), Bytes>(f1(Str::lit("unreachable"))))
                                }
                            }
                        }
                        Err(v10) => {
                            Ok(Err::<(), Bytes>(f4(f1(Str::lit("function ")), f4(f7(v2)?, f4(f1(Str::lit(": ")), v10)))))
                        }
                    }
                }
                Err(v8) => {
                    Ok(Err::<(), Bytes>(f4(f1(Str::lit("function ")), f4(f7(v2)?, f4(f1(Str::lit(": ")), v8)))))
                }
            }
        }
    } else {
        Ok(Err::<(), Bytes>(f4(f1(Str::lit("function ")), f4(f7(v2)?, f4(f1(Str::lit(": ")), f4(f7(v6)?, f4(f1(Str::lit(" parameter names for ")), f4(f7(v7)?, f1(Str::lit(" parameter types"))))))))))
    }
}

pub fn f10(v0: Adt0) -> u64 {
    let m83 = v0;
    match m83 {
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
        Adt0::C4(_) => {
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
        Adt0::C8(_) => {
            8u64
        }
        Adt0::C9(_, _) => {
            9u64
        }
        Adt0::C10(_) => {
            10u64
        }
        Adt0::C11(_, _) => {
            11u64
        }
        Adt0::C12(_) => {
            12u64
        }
        Adt0::C13(_, _) => {
            13u64
        }
        Adt0::C14 => {
            14u64
        }
        Adt0::C15(_, _) => {
            15u64
        }
    }
}

pub fn f11(v0: Bytes) -> R<Bytes> {
    let v1: u64 = {
        let (h84, _) = f23(Fn0::F24(v0.clone()), {
            let a85 = v0.clone();
            length_bytes(a85)
        }, 0u64)?;
        h84
    };
    Ok(f25(v0.clone(), v1, {
        let a87 = {
            let a86 = v0;
            length_bytes(a86)
        };
        let a88 = v1;
        nat_sub(a87, a88)
    }))
}

pub fn f12(v0: u64, v1: u64) -> Bytes {
    f25(f1(Str::lit("0123456789")), {
        let a92 = {
            let a89 = v0;
            let a90 = v1;
            let a91 = 0u64;
            nat_quot(a89, a90, a91)
        };
        let a93 = 10u64;
        let a94 = 0u64;
        nat_rem(a92, a93, a94)
    }, 1u64)
}

pub fn f13(v0: Adt7, v1: Adt0) -> R<Result<(), Bytes>> {
    let m95 = v1;
    match m95 {
        Adt0::C0 => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt0::C1 => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt0::C2 => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt0::C3 => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt0::C4(_) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt0::C5 => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt0::C6 => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt0::C7 => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt0::C8(r101) => {
            let v3 = (*r101).clone();
            f13(v0, v3)
        }
        Adt0::C9(r102, r103) => {
            let v4 = (*r102).clone();
            let v5 = (*r103).clone();
            let m96 = f13(v0.clone(), v4)?;
            match m96 {
                Ok(_) => {
                    f13(v0, v5)
                }
                Err(v7) => {
                    Ok(Err::<(), Bytes>(v7))
                }
            }
        }
        Adt0::C10(r104) => {
            let v8 = (*r104).clone();
            f13(v0, v8)
        }
        Adt0::C11(r105, r106) => {
            let v9 = (*r105).clone();
            let v10 = (*r106).clone();
            let m97 = f13(v0.clone(), v9)?;
            match m97 {
                Ok(_) => {
                    f13(v0, v10)
                }
                Err(v12) => {
                    Ok(Err::<(), Bytes>(v12))
                }
            }
        }
        Adt0::C12(v13) => {
            let m98 = f26(v0, v13);
            match m98 {
                None => {
                    Ok(Err::<(), Bytes>(f27(v13)?))
                }
                Some(_) => {
                    Ok(Ok::<(), Bytes>(()))
                }
            }
        }
        Adt0::C13(r107, r108) => {
            let v15 = (*r107).clone();
            let v16 = (*r108).clone();
            if f6(v15.clone()) {
                Ok(Err::<(), Bytes>(f1(Str::lit("a function type takes at least one parameter"))))
            } else {
                let m99 = f13(v0.clone(), v15)?;
                match m99 {
                    Ok(_) => {
                        f13(v0, v16)
                    }
                    Err(v18) => {
                        Ok(Err::<(), Bytes>(v18))
                    }
                }
            }
        }
        Adt0::C14 => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt0::C15(r109, r110) => {
            let v19 = (*r109).clone();
            let v20 = (*r110).clone();
            let m100 = f13(v0.clone(), v19)?;
            match m100 {
                Ok(_) => {
                    f13(v0, v20)
                }
                Err(v22) => {
                    Ok(Err::<(), Bytes>(v22))
                }
            }
        }
    }
}

pub fn f14(v0: Adt7, v1: u64) -> List<u64> {
    let m112 = f28(match v0 {
        Adt7::C0(_, _, h111, _, _, _) => {
            h111
        }
    }, v1);
    match m112 {
        None => {
            List::<u64>::nil()
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f15(v0: Adt7, v1: u64) -> Adt0 {
    let m113 = f29(v0, v1);
    match m113 {
        None => {
            Adt0::C14
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f16(v0: Adt7, v1: u64) -> Adt0 {
    let m115 = f30(match v0 {
        Adt7::C0(_, _, _, _, h114, _) => {
            h114
        }
    }, v1);
    match m115 {
        None => {
            Adt0::C0
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f17(v0: List<u64>) -> R<u64> {
    f31(Fn1::F32, 0u64, v0)
}

pub fn f18(v0: Adt0) -> R<u64> {
    let m116 = v0;
    match m116 {
        Adt0::C0 => {
            Ok(0u64)
        }
        Adt0::C1 => {
            Ok(0u64)
        }
        Adt0::C2 => {
            Ok(0u64)
        }
        Adt0::C3 => {
            Ok(0u64)
        }
        Adt0::C4(_) => {
            Ok(0u64)
        }
        Adt0::C5 => {
            Ok(0u64)
        }
        Adt0::C6 => {
            Ok(0u64)
        }
        Adt0::C7 => {
            Ok(0u64)
        }
        Adt0::C8(_) => {
            Ok(0u64)
        }
        Adt0::C9(_, _) => {
            Ok(0u64)
        }
        Adt0::C10(_) => {
            Ok(0u64)
        }
        Adt0::C11(_, _) => {
            Ok(0u64)
        }
        Adt0::C12(_) => {
            Ok(0u64)
        }
        Adt0::C13(_, _) => {
            Ok(0u64)
        }
        Adt0::C14 => {
            Ok(0u64)
        }
        Adt0::C15(_, r119) => {
            let v12 = (*r119).clone();
            let v13: u64 = f18(v12)?;
            let a117 = 1u64;
            let a118 = v13;
            nat_add(a117, a118)
        }
    }
}

pub fn f19(v0: List<u64>) -> bool {
    let m120 = v0;
    match m120.uncons() {
        None => {
            false
        }
        Some((v1, v2)) => {
            let v3: bool = f19(v2.clone());
            let v4: bool = f33(v2, v1);
            if v4 {
                true
            } else {
                v3
            }
        }
    }
}

pub fn f20(v0: Adt7, v1: Adt2, v2: List<(u64, Adt0)>, v3: u64, v4: Adt0, v5: Adt0, v6: Bytes, v7: u64, v8: List<Adt5>) -> R<Adt8> {
    let m121 = v1;
    match m121 {
        Adt2::C0(v9, v10) => {
            let a122 = v3;
            let a123 = 0u64;
            if nat_eq(a122, a123) {
                let m124 = f13(v0.clone(), v9.clone())?;
                match m124 {
                    Ok(_) => {
                        let m125 = f34(v0, v10, v9.clone(), 0u64)?;
                        match m125 {
                            Ok(_) => {
                                Ok(Adt8::C1(v9))
                            }
                            Err(v13) => {
                                Ok(Adt8::C0(v13))
                            }
                        }
                    }
                    Err(v14) => {
                        Ok(Adt8::C0(v14))
                    }
                }
            } else {
                Ok(Adt8::C0(f1(Str::lit("unreachable"))))
            }
        }
        Adt2::C1(v15) => {
            let a126 = v3;
            let a127 = 0u64;
            if nat_eq(a126, a127) {
                let m128 = f35(v2, v15);
                match m128 {
                    None => {
                        Ok(Adt8::C0(f4(f1(Str::lit("local ")), f4(f7(v15)?, f1(Str::lit(" is unbound"))))))
                    }
                    Some(v16) => {
                        Ok(Adt8::C1(v16))
                    }
                }
            } else {
                Ok(Adt8::C0(f1(Str::lit("unreachable"))))
            }
        }
        Adt2::C2(v17, v18, r233, r234) => {
            let v19 = (*r233).clone();
            let v20 = (*r234).clone();
            let a129 = v3;
            let a130 = 0u64;
            if nat_eq(a129, a130) {
                let m131 = f13(v0.clone(), v18.clone())?;
                match m131 {
                    Ok(_) => {
                        let m132 = f20(v0.clone(), v19, v2.clone(), 0u64, Adt0::C14, Adt0::C14, f1(Str::lit("")), 0u64, List::<Adt5>::nil())?;
                        match m132 {
                            Adt8::C0(v25) => {
                                Ok(Adt8::C0(v25))
                            }
                            Adt8::C1(v22) => {
                                let m133 = f22(v22, v18.clone(), f4(f1(Str::lit("the value bound to ")), f7(v17)?))?;
                                match m133 {
                                    Ok(_) => {
                                        f20(v0, v20, f36(v17, v18, v2), 0u64, Adt0::C14, Adt0::C14, f1(Str::lit("")), 0u64, List::<Adt5>::nil())
                                    }
                                    Err(v24) => {
                                        Ok(Adt8::C0(v24))
                                    }
                                }
                            }
                            Adt8::C2(_) => {
                                Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                            }
                            Adt8::C3 => {
                                Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                            }
                        }
                    }
                    Err(v27) => {
                        Ok(Adt8::C0(v27))
                    }
                }
            } else {
                Ok(Adt8::C0(f1(Str::lit("unreachable"))))
            }
        }
        Adt2::C3(r235, r236, r237) => {
            let v28 = (*r235).clone();
            let v29 = (*r236).clone();
            let v30 = (*r237).clone();
            let a134 = v3;
            let a135 = 0u64;
            if nat_eq(a134, a135) {
                let m136 = f20(v0.clone(), v28, v2.clone(), 0u64, Adt0::C14, Adt0::C14, f1(Str::lit("")), 0u64, List::<Adt5>::nil())?;
                match m136 {
                    Adt8::C0(v42) => {
                        Ok(Adt8::C0(v42))
                    }
                    Adt8::C1(v31) => {
                        let m137 = f22(v31, Adt0::C1, f1(Str::lit("a condition")))?;
                        match m137 {
                            Ok(_) => {
                                let m138 = f20(v0.clone(), v29, v2.clone(), 0u64, Adt0::C14, Adt0::C14, f1(Str::lit("")), 0u64, List::<Adt5>::nil())?;
                                match m138 {
                                    Adt8::C0(v39) => {
                                        Ok(Adt8::C0(v39))
                                    }
                                    Adt8::C1(v33) => {
                                        let m139 = f20(v0, v30, v2, 0u64, Adt0::C14, Adt0::C14, f1(Str::lit("")), 0u64, List::<Adt5>::nil())?;
                                        match m139 {
                                            Adt8::C0(v37) => {
                                                Ok(Adt8::C0(v37))
                                            }
                                            Adt8::C1(v34) => {
                                                let m140 = f22(v34, v33.clone(), f1(Str::lit("the else branch")))?;
                                                match m140 {
                                                    Ok(_) => {
                                                        Ok(Adt8::C1(v33))
                                                    }
                                                    Err(v36) => {
                                                        Ok(Adt8::C0(v36))
                                                    }
                                                }
                                            }
                                            Adt8::C2(_) => {
                                                Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                                            }
                                            Adt8::C3 => {
                                                Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                                            }
                                        }
                                    }
                                    Adt8::C2(_) => {
                                        Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                                    }
                                    Adt8::C3 => {
                                        Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                                    }
                                }
                            }
                            Err(v41) => {
                                Ok(Adt8::C0(v41))
                            }
                        }
                    }
                    Adt8::C2(_) => {
                        Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                    }
                    Adt8::C3 => {
                        Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                    }
                }
            } else {
                Ok(Adt8::C0(f1(Str::lit("unreachable"))))
            }
        }
        Adt2::C4(v44, r238, r239) => {
            let v45 = (*r238).clone();
            let v46 = (*r239).clone();
            let a141 = v3;
            let a142 = 0u64;
            if nat_eq(a141, a142) {
                let m143 = f13(v0.clone(), v44.clone())?;
                match m143 {
                    Ok(_) => {
                        let m144 = f20(v0.clone(), v45, v2.clone(), 0u64, Adt0::C14, Adt0::C14, f1(Str::lit("")), 0u64, List::<Adt5>::nil())?;
                        match m144 {
                            Adt8::C0(v55) => {
                                Ok(Adt8::C0(v55))
                            }
                            Adt8::C1(v48) => {
                                let m145 = f37(v0.clone(), v48.clone())?;
                                match m145 {
                                    Ok(v49) => {
                                        let m146 = f20(v0, v46, v2, 4u64, v44.clone(), v48, f1(Str::lit("")), 0u64, List::<Adt5>::nil())?;
                                        match m146 {
                                            Adt8::C0(v52) => {
                                                Ok(Adt8::C0(v52))
                                            }
                                            Adt8::C1(_) => {
                                                Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                                            }
                                            Adt8::C2(v50) => {
                                                let m147 = f38(v49, v50);
                                                match m147 {
                                                    None => {
                                                        Ok(Adt8::C1(v44))
                                                    }
                                                    Some(v51) => {
                                                        Ok(Adt8::C0(f4(f1(Str::lit("the match is not exhaustive: ")), f4(f39(v51)?, f1(Str::lit(" is not covered"))))))
                                                    }
                                                }
                                            }
                                            Adt8::C3 => {
                                                Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                                            }
                                        }
                                    }
                                    Err(v54) => {
                                        Ok(Adt8::C0(v54))
                                    }
                                }
                            }
                            Adt8::C2(_) => {
                                Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                            }
                            Adt8::C3 => {
                                Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                            }
                        }
                    }
                    Err(v57) => {
                        Ok(Adt8::C0(v57))
                    }
                }
            } else {
                Ok(Adt8::C0(f1(Str::lit("unreachable"))))
            }
        }
        Adt2::C5(v58, v59, r240) => {
            let v60 = (*r240).clone();
            let a148 = v3;
            let a149 = 0u64;
            if nat_eq(a148, a149) {
                let m150 = f13(v0.clone(), v59.clone())?;
                match m150 {
                    Ok(_) => {
                        let m151 = f40(v0.clone(), v58.clone(), v59.clone())?;
                        match m151 {
                            Ok(v62) => {
                                let v63: Bytes = f4(f1(Str::lit("constructor ")), f39(v58)?);
                                let a152 = f41(v60.clone())?;
                                let a153 = f18(v62.clone())?;
                                if nat_eq(a152, a153) {
                                    let m154 = f20(v0, v60, v2, 2u64, v62, Adt0::C14, v63, 0u64, List::<Adt5>::nil())?;
                                    match m154 {
                                        Adt8::C0(v64) => {
                                            Ok(Adt8::C0(v64))
                                        }
                                        Adt8::C1(_) => {
                                            Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                                        }
                                        Adt8::C2(_) => {
                                            Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                                        }
                                        Adt8::C3 => {
                                            Ok(Adt8::C1(v59))
                                        }
                                    }
                                } else {
                                    Ok(Adt8::C0(f4(v63, f4(f1(Str::lit(" takes ")), f4(f7(f18(v62)?)?, f4(f1(Str::lit(" operand(s), received ")), f7(f41(v60)?)?))))))
                                }
                            }
                            Err(v67) => {
                                Ok(Adt8::C0(v67))
                            }
                        }
                    }
                    Err(v68) => {
                        Ok(Adt8::C0(v68))
                    }
                }
            } else {
                Ok(Adt8::C0(f1(Str::lit("unreachable"))))
            }
        }
        Adt2::C6(v69, r241) => {
            let v70 = (*r241).clone();
            let a155 = v3;
            let a156 = 0u64;
            if nat_eq(a155, a156) {
                let m157 = f29(v0.clone(), v69);
                match m157 {
                    None => {
                        Ok(Adt8::C0(f42(v69)?))
                    }
                    Some(v71) => {
                        let v72: Bytes = f4(f1(Str::lit("function ")), f7(v69)?);
                        let a158 = f41(v70.clone())?;
                        let a159 = f18(v71.clone())?;
                        if nat_eq(a158, a159) {
                            let m160 = f20(v0.clone(), v70, v2, 2u64, v71, Adt0::C14, v72, 0u64, List::<Adt5>::nil())?;
                            match m160 {
                                Adt8::C0(v73) => {
                                    Ok(Adt8::C0(v73))
                                }
                                Adt8::C1(_) => {
                                    Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                                }
                                Adt8::C2(_) => {
                                    Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                                }
                                Adt8::C3 => {
                                    Ok(Adt8::C1(f16(v0, v69)))
                                }
                            }
                        } else {
                            Ok(Adt8::C0(f4(v72, f4(f1(Str::lit(" takes ")), f4(f7(f18(v71)?)?, f4(f1(Str::lit(" operand(s), received ")), f7(f41(v70)?)?))))))
                        }
                    }
                }
            } else {
                Ok(Adt8::C0(f1(Str::lit("unreachable"))))
            }
        }
        Adt2::C7(v76, r242) => {
            let v77 = (*r242).clone();
            let a161 = v3;
            let a162 = 0u64;
            if nat_eq(a161, a162) {
                let m163 = f29(v0.clone(), v76);
                match m163 {
                    None => {
                        Ok(Adt8::C0(f42(v76)?))
                    }
                    Some(v78) => {
                        let v79: u64 = f18(v78.clone())?;
                        let v80: u64 = f41(v77.clone())?;
                        let a164 = v79;
                        let a165 = v80;
                        if nat_le(a164, a165) {
                            Ok(Adt8::C0(f4(f1(Str::lit("closure of function ")), f4(f7(v76)?, f4(f1(Str::lit(" captures ")), f4(f7(v80)?, f4(f1(Str::lit(" of its ")), f4(f7(v79)?, f1(Str::lit(" parameters; at least one must remain"))))))))))
                        } else {
                            let m166 = f20(v0.clone(), v77, v2, 3u64, v78, Adt0::C14, f4(f1(Str::lit("closure of function ")), f7(v76)?), 0u64, List::<Adt5>::nil())?;
                            match m166 {
                                Adt8::C0(v82) => {
                                    Ok(Adt8::C0(v82))
                                }
                                Adt8::C1(v81) => {
                                    Ok(Adt8::C1(f43(v81, f16(v0, v76))))
                                }
                                Adt8::C2(_) => {
                                    Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                                }
                                Adt8::C3 => {
                                    Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                                }
                            }
                        }
                    }
                }
            } else {
                Ok(Adt8::C0(f1(Str::lit("unreachable"))))
            }
        }
        Adt2::C8(r243, r244) => {
            let v84 = (*r243).clone();
            let v85 = (*r244).clone();
            let a167 = v3;
            let a168 = 0u64;
            if nat_eq(a167, a168) {
                let m169 = f20(v0.clone(), v84, v2.clone(), 0u64, Adt0::C14, Adt0::C14, f1(Str::lit("")), 0u64, List::<Adt5>::nil())?;
                match m169 {
                    Adt8::C0(v91) => {
                        Ok(Adt8::C0(v91))
                    }
                    Adt8::C1(v86) => {
                        let a170 = f10(v86.clone());
                        let a171 = 13u64;
                        if nat_eq(a170, a171) {
                            let v87: Bytes = f1(Str::lit("an application"));
                            let a172 = f41(v85.clone())?;
                            let a173 = f18(f44(v86.clone()))?;
                            if nat_eq(a172, a173) {
                                let m174 = f20(v0, v85, v2, 2u64, f44(v86.clone()), Adt0::C14, v87, 0u64, List::<Adt5>::nil())?;
                                match m174 {
                                    Adt8::C0(v88) => {
                                        Ok(Adt8::C0(v88))
                                    }
                                    Adt8::C1(_) => {
                                        Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                                    }
                                    Adt8::C2(_) => {
                                        Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                                    }
                                    Adt8::C3 => {
                                        Ok(Adt8::C1(f45(v86)))
                                    }
                                }
                            } else {
                                Ok(Adt8::C0(f4(v87, f4(f1(Str::lit(" takes ")), f4(f7(f18(f44(v86))?)?, f4(f1(Str::lit(" operand(s), received ")), f7(f41(v85)?)?))))))
                            }
                        } else {
                            Ok(Adt8::C0(f1(Str::lit("only a function value can be applied"))))
                        }
                    }
                    Adt8::C2(_) => {
                        Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                    }
                    Adt8::C3 => {
                        Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                    }
                }
            } else {
                Ok(Adt8::C0(f1(Str::lit("unreachable"))))
            }
        }
        Adt2::C9(v93, r245) => {
            let v94 = (*r245).clone();
            let a175 = v3;
            let a176 = 0u64;
            if nat_eq(a175, a176) {
                let m177 = f20(v0, v94, v2, 1u64, Adt0::C14, Adt0::C14, f1(Str::lit("")), 0u64, List::<Adt5>::nil())?;
                match m177 {
                    Adt8::C0(v98) => {
                        Ok(Adt8::C0(v98))
                    }
                    Adt8::C1(v95) => {
                        let m178 = f46(v93, v95)?;
                        match m178 {
                            Ok(v96) => {
                                Ok(Adt8::C1(v96))
                            }
                            Err(v97) => {
                                Ok(Adt8::C0(v97))
                            }
                        }
                    }
                    Adt8::C2(_) => {
                        Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                    }
                    Adt8::C3 => {
                        Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                    }
                }
            } else {
                Ok(Adt8::C0(f1(Str::lit("unreachable"))))
            }
        }
        Adt2::C10(r246) => {
            let v100 = (*r246).clone();
            let a179 = v3;
            let a180 = 0u64;
            if nat_eq(a179, a180) {
                let m181 = f20(v0, v100, v2, 0u64, Adt0::C14, Adt0::C14, f1(Str::lit("")), 0u64, List::<Adt5>::nil())?;
                match m181 {
                    Adt8::C0(v102) => {
                        Ok(Adt8::C0(v102))
                    }
                    Adt8::C1(v101) => {
                        let a182 = f10(v101.clone());
                        let a183 = 11u64;
                        if nat_eq(a182, a183) {
                            Ok(Adt8::C1(f44(v101)))
                        } else {
                            Ok(Adt8::C0(f4(f1(Str::lit("`first` of ")), f47(v101)?)))
                        }
                    }
                    Adt8::C2(_) => {
                        Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                    }
                    Adt8::C3 => {
                        Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                    }
                }
            } else {
                Ok(Adt8::C0(f1(Str::lit("unreachable"))))
            }
        }
        Adt2::C11(r247) => {
            let v104 = (*r247).clone();
            let a184 = v3;
            let a185 = 0u64;
            if nat_eq(a184, a185) {
                let m186 = f20(v0, v104, v2, 0u64, Adt0::C14, Adt0::C14, f1(Str::lit("")), 0u64, List::<Adt5>::nil())?;
                match m186 {
                    Adt8::C0(v106) => {
                        Ok(Adt8::C0(v106))
                    }
                    Adt8::C1(v105) => {
                        let a187 = f10(v105.clone());
                        let a188 = 11u64;
                        if nat_eq(a187, a188) {
                            Ok(Adt8::C1(f45(v105)))
                        } else {
                            Ok(Adt8::C0(f4(f1(Str::lit("`second` of ")), f47(v105)?)))
                        }
                    }
                    Adt8::C2(_) => {
                        Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                    }
                    Adt8::C3 => {
                        Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                    }
                }
            } else {
                Ok(Adt8::C0(f1(Str::lit("unreachable"))))
            }
        }
        Adt2::C12(r248, v109) => {
            let v108 = (*r248).clone();
            let a189 = v3;
            let a190 = 0u64;
            if nat_eq(a189, a190) {
                let m191 = f20(v0.clone(), v108, v2, 0u64, Adt0::C14, Adt0::C14, f1(Str::lit("")), 0u64, List::<Adt5>::nil())?;
                match m191 {
                    Adt8::C0(v115) => {
                        Ok(Adt8::C0(v115))
                    }
                    Adt8::C1(v110) => {
                        let a192 = f10(v110.clone());
                        let a193 = 12u64;
                        if nat_eq(a192, a193) {
                            let v111: u64 = f48(v110);
                            let m194 = f26(v0, v111);
                            match m194 {
                                None => {
                                    Ok(Adt8::C0(f27(v111)?))
                                }
                                Some(v112) => {
                                    let a195 = f18(v112.clone())?;
                                    let a196 = 1u64;
                                    if nat_eq(a195, a196) {
                                        let m197 = f30(v112, 0u64);
                                        match m197 {
                                            None => {
                                                Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                                            }
                                            Some(v113) => {
                                                let m198 = f30(v113, v109);
                                                match m198 {
                                                    None => {
                                                        Ok(Adt8::C0(f4(f1(Str::lit("ADT ")), f4(f7(v111)?, f4(f1(Str::lit(" has no field ")), f7(v109)?)))))
                                                    }
                                                    Some(v114) => {
                                                        Ok(Adt8::C1(v114))
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        Ok(Adt8::C0(f4(f1(Str::lit("`field` requires an ADT with exactly one constructor; ADT ")), f4(f7(v111)?, f4(f1(Str::lit(" has ")), f7(f18(v112)?)?)))))
                                    }
                                }
                            }
                        } else {
                            Ok(Adt8::C0(f1(Str::lit("`field` requires an ADT value"))))
                        }
                    }
                    Adt8::C2(_) => {
                        Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                    }
                    Adt8::C3 => {
                        Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                    }
                }
            } else {
                Ok(Adt8::C0(f1(Str::lit("unreachable"))))
            }
        }
        Adt2::C13 => {
            let a199 = v3;
            let a200 = 1u64;
            if nat_eq(a199, a200) {
                Ok(Adt8::C1(Adt0::C14))
            } else {
                let a201 = v3;
                let a202 = 2u64;
                if nat_eq(a201, a202) {
                    Ok(Adt8::C3)
                } else {
                    let a203 = v3;
                    let a204 = 3u64;
                    if nat_eq(a203, a204) {
                        Ok(Adt8::C1(v4))
                    } else {
                        let a205 = v3;
                        let a206 = 4u64;
                        if nat_eq(a205, a206) {
                            Ok(Adt8::C2(v8))
                        } else {
                            Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                        }
                    }
                }
            }
        }
        Adt2::C14(r249, r250) => {
            let v117 = (*r249).clone();
            let v118 = (*r250).clone();
            let a207 = v3;
            let a208 = 1u64;
            if nat_eq(a207, a208) {
                let m209 = f20(v0.clone(), v117, v2.clone(), 0u64, Adt0::C14, Adt0::C14, f1(Str::lit("")), 0u64, List::<Adt5>::nil())?;
                match m209 {
                    Adt8::C0(v123) => {
                        Ok(Adt8::C0(v123))
                    }
                    Adt8::C1(v119) => {
                        let m210 = f20(v0, v118, v2, 1u64, Adt0::C14, Adt0::C14, f1(Str::lit("")), 0u64, List::<Adt5>::nil())?;
                        match m210 {
                            Adt8::C0(v121) => {
                                Ok(Adt8::C0(v121))
                            }
                            Adt8::C1(v120) => {
                                Ok(Adt8::C1(f49(v119, v120)))
                            }
                            Adt8::C2(_) => {
                                Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                            }
                            Adt8::C3 => {
                                Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                            }
                        }
                    }
                    Adt8::C2(_) => {
                        Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                    }
                    Adt8::C3 => {
                        Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                    }
                }
            } else {
                let a211 = v3;
                let a212 = 2u64;
                if nat_eq(a211, a212) {
                    if f6(v4.clone()) {
                        Ok(Adt8::C3)
                    } else {
                        let m213 = f20(v0.clone(), v117, v2.clone(), 0u64, Adt0::C14, Adt0::C14, f1(Str::lit("")), 0u64, List::<Adt5>::nil())?;
                        match m213 {
                            Adt8::C0(v128) => {
                                Ok(Adt8::C0(v128))
                            }
                            Adt8::C1(v125) => {
                                let m214 = f22(v125, f44(v4.clone()), f4(v6.clone(), f4(f1(Str::lit(" operand ")), f7(v7)?)))?;
                                match m214 {
                                    Ok(_) => {
                                        f20(v0, v118, v2, 2u64, f45(v4), Adt0::C14, v6, {
                                            let a215 = v7;
                                            let a216 = 1u64;
                                            nat_add(a215, a216)?
                                        }, List::<Adt5>::nil())
                                    }
                                    Err(v127) => {
                                        Ok(Adt8::C0(v127))
                                    }
                                }
                            }
                            Adt8::C2(_) => {
                                Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                            }
                            Adt8::C3 => {
                                Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                            }
                        }
                    }
                } else {
                    let a217 = v3;
                    let a218 = 3u64;
                    if nat_eq(a217, a218) {
                        if f6(v4.clone()) {
                            Ok(Adt8::C1(Adt0::C14))
                        } else {
                            let m219 = f20(v0.clone(), v117, v2.clone(), 0u64, Adt0::C14, Adt0::C14, f1(Str::lit("")), 0u64, List::<Adt5>::nil())?;
                            match m219 {
                                Adt8::C0(v133) => {
                                    Ok(Adt8::C0(v133))
                                }
                                Adt8::C1(v130) => {
                                    let m220 = f22(v130, f44(v4.clone()), f4(v6.clone(), f4(f1(Str::lit(" operand ")), f7(v7)?)))?;
                                    match m220 {
                                        Ok(_) => {
                                            f20(v0, v118, v2, 3u64, f45(v4), Adt0::C14, v6, {
                                                let a221 = v7;
                                                let a222 = 1u64;
                                                nat_add(a221, a222)?
                                            }, List::<Adt5>::nil())
                                        }
                                        Err(v132) => {
                                            Ok(Adt8::C0(v132))
                                        }
                                    }
                                }
                                Adt8::C2(_) => {
                                    Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                                }
                                Adt8::C3 => {
                                    Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                                }
                            }
                        }
                    } else {
                        let a223 = v3;
                        let a224 = 4u64;
                        if nat_eq(a223, a224) {
                            let m225 = f20(v0.clone(), v117, v2.clone(), 5u64, v4.clone(), v5.clone(), f1(Str::lit("")), 0u64, v8)?;
                            match m225 {
                                Adt8::C0(v136) => {
                                    Ok(Adt8::C0(v136))
                                }
                                Adt8::C1(_) => {
                                    Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                                }
                                Adt8::C2(v135) => {
                                    f20(v0, v118, v2, 4u64, v4, v5, f1(Str::lit("")), 0u64, v135)
                                }
                                Adt8::C3 => {
                                    Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                                }
                            }
                        } else {
                            Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                        }
                    }
                }
            }
        }
        Adt2::C15(v138, v139, r251) => {
            let v140 = (*r251).clone();
            let a226 = v3;
            let a227 = 5u64;
            if nat_eq(a226, a227) {
                let m228 = f40(v0.clone(), v138.clone(), v5)?;
                match m228 {
                    Ok(v141) => {
                        let v142: u64 = f17(v139.clone())?;
                        let v143: u64 = f18(v141.clone())?;
                        let a229 = v142;
                        let a230 = v143;
                        if nat_eq(a229, a230) {
                            if f19(v139.clone()) {
                                Ok(Adt8::C0(f4(f1(Str::lit("shape ")), f4(f39(v138)?, f1(Str::lit(" binds the same name twice"))))))
                            } else {
                                if f50(v8.clone(), v138.clone()) {
                                    Ok(Adt8::C0(f4(f1(Str::lit("the arm for ")), f4(f39(v138)?, f1(Str::lit(" is unreachable"))))))
                                } else {
                                    let m231 = f20(v0, v140, f21(v139, v141, v2), 0u64, Adt0::C14, Adt0::C14, f1(Str::lit("")), 0u64, List::<Adt5>::nil())?;
                                    match m231 {
                                        Adt8::C0(v147) => {
                                            Ok(Adt8::C0(v147))
                                        }
                                        Adt8::C1(v144) => {
                                            let m232 = f22(v144, v4, f1(Str::lit("an arm")))?;
                                            match m232 {
                                                Ok(_) => {
                                                    Ok(Adt8::C2(f51(v138, v8)))
                                                }
                                                Err(v146) => {
                                                    Ok(Adt8::C0(v146))
                                                }
                                            }
                                        }
                                        Adt8::C2(_) => {
                                            Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                                        }
                                        Adt8::C3 => {
                                            Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                                        }
                                    }
                                }
                            }
                        } else {
                            Ok(Adt8::C0(f4(f1(Str::lit("shape ")), f4(f39(v138)?, f4(f1(Str::lit(" binds ")), f4(f7(v143)?, f4(f1(Str::lit(" value(s), the arm names ")), f7(v142)?)))))))
                        }
                    }
                    Err(v149) => {
                        Ok(Adt8::C0(v149))
                    }
                }
            } else {
                Ok(Adt8::C0(f1(Str::lit("unreachable"))))
            }
        }
    }
}

pub fn f21(v0: List<u64>, v1: Adt0, v2: List<(u64, Adt0)>) -> List<(u64, Adt0)> {
    let m252 = v0;
    match m252.uncons() {
        None => {
            v2
        }
        Some((v3, v4)) => {
            let v5: List<(u64, Adt0)> = f21(v4, f45(v1.clone()), f36(v3, f44(v1.clone()), v2.clone()));
            if f6(v1) {
                v2
            } else {
                v5
            }
        }
    }
}

pub fn f22(v0: Adt0, v1: Adt0, v2: Bytes) -> R<Result<(), Bytes>> {
    let v3: Result<(), Bytes> = Ok::<(), Bytes>(());
    let v4: Result<(), Bytes> = Err::<(), Bytes>(f4(v2, f4(f1(Str::lit(" has type ")), f4(f47(v0.clone())?, f4(f1(Str::lit(", expected ")), f47(v1.clone())?)))));
    if f52(v0, v1) {
        Ok(v3)
    } else {
        Ok(v4)
    }
}

pub fn f23(v0: Fn0, p257: u64, p258: u64) -> R<(u64, bool)> {
    let m256 = (p257, p258);
    {
        let mut m256 = m256;
        loop {
            let step = {
                let (v1, v2) = m256;
                let m253 = v1;
                if m253 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m253 - 1;
                    let m255 = {
                        let c254: Fn0 = v0.clone();
                        c254.apply(v2)?
                    };
                    match m255 {
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
                Ok(next) => { m256 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f24(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f53(v0, v1)
}

pub fn f25(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m262 = {
        let a259 = v0;
        let a260 = v1;
        let a261 = v2;
        slice_bytes(a259, a260, a261)
    };
    match m262 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f26(v0: Adt7, v1: u64) -> Option<Adt0> {
    f30(match v0 {
        Adt7::C0(_, h263, _, _, _, _) => {
            h263
        }
    }, v1)
}

pub fn f27(v0: u64) -> R<Bytes> {
    Ok(f4(f1(Str::lit("ADT ")), f4(f7(v0)?, f1(Str::lit(" is not declared")))))
}

pub fn f28(p267: List<List<u64>>, p268: u64) -> Option<List<u64>> {
    let m266 = (p267, p268);
    {
        let mut m266 = m266;
        loop {
            let step = {
                let (v0, v1) = m266;
                let m264 = v0;
                match m264.uncons() {
                    None => {
                        Err::<(List<List<u64>>, u64), Option<List<u64>>>(None::<List<u64>>)
                    }
                    Some((v2, v3)) => {
                        let m265 = v1;
                        if m265 == 0 {
                            Err::<(List<List<u64>>, u64), Option<List<u64>>>(Some(v2))
                        } else {
                            let v4 = m265 - 1;
                            Ok::<(List<List<u64>>, u64), Option<List<u64>>>((v3, v4))
                        }
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

pub fn f29(v0: Adt7, v1: u64) -> Option<Adt0> {
    f30(match v0 {
        Adt7::C0(_, _, _, h269, _, _) => {
            h269
        }
    }, v1)
}

pub fn f30(p275: Adt0, p276: u64) -> Option<Adt0> {
    let m274 = (p275, p276);
    {
        let mut m274 = m274;
        loop {
            let step = {
                let (v0, v1) = m274;
                let m270 = v0;
                match m270 {
                    Adt0::C0 => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C1 => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C2 => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C3 => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C4(_) => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C5 => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C6 => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C7 => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C8(_) => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C9(_, _) => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C10(_) => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C11(_, _) => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C12(_) => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C13(_, _) => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C14 => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C15(r272, r273) => {
                        let v12 = (*r272).clone();
                        let v13 = (*r273).clone();
                        let m271 = v1;
                        if m271 == 0 {
                            Err::<(Adt0, u64), Option<Adt0>>(Some(v12))
                        } else {
                            let v14 = m271 - 1;
                            Ok::<(Adt0, u64), Option<Adt0>>((v13, v14))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m274 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f31(v0: Fn1, p280: u64, p281: List<u64>) -> R<u64> {
    let m279 = (p280, p281);
    {
        let mut m279 = m279;
        loop {
            let step = {
                let (v1, v2) = m279;
                let m277 = v2;
                match m277.uncons() {
                    None => {
                        Err::<(u64, List<u64>), R<u64>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(u64, List<u64>), R<u64>>(({
                            let c278: Fn1 = v0.clone();
                            c278.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m279 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f32(v0: u64, _: u64) -> R<u64> {
    let a282 = v0;
    let a283 = 1u64;
    nat_add(a282, a283)
}

pub fn f33(v0: List<u64>, v1: u64) -> bool {
    let m284 = v0;
    match m284.uncons() {
        None => {
            false
        }
        Some((v2, v3)) => {
            let v4: bool = f33(v3, v1);
            let a285 = v2;
            let a286 = v1;
            if nat_eq(a285, a286) {
                true
            } else {
                v4
            }
        }
    }
}

pub fn f34(v0: Adt7, v1: Adt3, v2: Adt0, v3: u64) -> R<Result<(), Bytes>> {
    let m287 = v1;
    match m287 {
        Adt3::C0 => {
            let v4: Result<(), Bytes> = Ok::<(), Bytes>(());
            let v5: Result<(), Bytes> = Err::<(), Bytes>(f4(f1(Str::lit("the literal ")), f4(f54(Adt3::C0)?, f4(f1(Str::lit(" does not have type ")), f47(v2.clone())?))));
            let a288 = f10(v2);
            let a289 = 0u64;
            if nat_eq(a288, a289) {
                Ok(v4)
            } else {
                Ok(v5)
            }
        }
        Adt3::C1(v6) => {
            let v7: Result<(), Bytes> = Ok::<(), Bytes>(());
            let v8: Result<(), Bytes> = Err::<(), Bytes>(f4(f1(Str::lit("the literal ")), f4(f54(Adt3::C1(v6))?, f4(f1(Str::lit(" does not have type ")), f47(v2.clone())?))));
            let a290 = f10(v2);
            let a291 = 1u64;
            if nat_eq(a290, a291) {
                Ok(v7)
            } else {
                Ok(v8)
            }
        }
        Adt3::C2(v9) => {
            let v10: Result<(), Bytes> = f55(v9.clone())?;
            let v11: Result<(), Bytes> = Err::<(), Bytes>(f4(f1(Str::lit("the literal ")), f4(f54(Adt3::C2(v9))?, f4(f1(Str::lit(" does not have type ")), f47(v2.clone())?))));
            let a292 = f10(v2);
            let a293 = 2u64;
            if nat_eq(a292, a293) {
                Ok(v10)
            } else {
                Ok(v11)
            }
        }
        Adt3::C3(v12) => {
            let v13: Result<(), Bytes> = f56(v12.clone(), Adt1::C7)?;
            let v14: Result<(), Bytes> = Err::<(), Bytes>(f4(f1(Str::lit("the literal ")), f4(f54(Adt3::C3(v12))?, f4(f1(Str::lit(" does not have type ")), f47(v2.clone())?))));
            let a294 = f10(v2);
            let a295 = 3u64;
            if nat_eq(a294, a295) {
                Ok(v13)
            } else {
                Ok(v14)
            }
        }
        Adt3::C4(v15) => {
            let v16: Result<(), Bytes> = f56(v15.clone(), Adt1::C0)?;
            let v17: Result<(), Bytes> = Err::<(), Bytes>(f4(f1(Str::lit("the literal ")), f4(f54(Adt3::C4(v15))?, f4(f1(Str::lit(" does not have type ")), f47(v2.clone())?))));
            let a296 = f10(v2.clone());
            let a297 = 4u64;
            if if nat_eq(a296, a297) {
                let a298 = f57(f58(v2));
                let a299 = 0u64;
                nat_eq(a298, a299)
            } else {
                false
            } {
                Ok(v16)
            } else {
                Ok(v17)
            }
        }
        Adt3::C5(v18) => {
            let v19: Result<(), Bytes> = f56(v18.clone(), Adt1::C1)?;
            let v20: Result<(), Bytes> = Err::<(), Bytes>(f4(f1(Str::lit("the literal ")), f4(f54(Adt3::C5(v18))?, f4(f1(Str::lit(" does not have type ")), f47(v2.clone())?))));
            let a300 = f10(v2.clone());
            let a301 = 4u64;
            if if nat_eq(a300, a301) {
                let a302 = f57(f58(v2));
                let a303 = 1u64;
                nat_eq(a302, a303)
            } else {
                false
            } {
                Ok(v19)
            } else {
                Ok(v20)
            }
        }
        Adt3::C6(v21) => {
            let v22: Result<(), Bytes> = f56(v21.clone(), Adt1::C2)?;
            let v23: Result<(), Bytes> = Err::<(), Bytes>(f4(f1(Str::lit("the literal ")), f4(f54(Adt3::C6(v21))?, f4(f1(Str::lit(" does not have type ")), f47(v2.clone())?))));
            let a304 = f10(v2.clone());
            let a305 = 4u64;
            if if nat_eq(a304, a305) {
                let a306 = f57(f58(v2));
                let a307 = 2u64;
                nat_eq(a306, a307)
            } else {
                false
            } {
                Ok(v22)
            } else {
                Ok(v23)
            }
        }
        Adt3::C7(v24) => {
            let v25: Result<(), Bytes> = f56(v24.clone(), Adt1::C3)?;
            let v26: Result<(), Bytes> = Err::<(), Bytes>(f4(f1(Str::lit("the literal ")), f4(f54(Adt3::C7(v24))?, f4(f1(Str::lit(" does not have type ")), f47(v2.clone())?))));
            let a308 = f10(v2.clone());
            let a309 = 4u64;
            if if nat_eq(a308, a309) {
                let a310 = f57(f58(v2));
                let a311 = 3u64;
                nat_eq(a310, a311)
            } else {
                false
            } {
                Ok(v25)
            } else {
                Ok(v26)
            }
        }
        Adt3::C8(v27) => {
            let v28: Result<(), Bytes> = f56(v27.clone(), Adt1::C4)?;
            let v29: Result<(), Bytes> = Err::<(), Bytes>(f4(f1(Str::lit("the literal ")), f4(f54(Adt3::C8(v27))?, f4(f1(Str::lit(" does not have type ")), f47(v2.clone())?))));
            let a312 = f10(v2.clone());
            let a313 = 4u64;
            if if nat_eq(a312, a313) {
                let a314 = f57(f58(v2));
                let a315 = 4u64;
                nat_eq(a314, a315)
            } else {
                false
            } {
                Ok(v28)
            } else {
                Ok(v29)
            }
        }
        Adt3::C9(v30) => {
            let v31: Result<(), Bytes> = f56(v30.clone(), Adt1::C5)?;
            let v32: Result<(), Bytes> = Err::<(), Bytes>(f4(f1(Str::lit("the literal ")), f4(f54(Adt3::C9(v30))?, f4(f1(Str::lit(" does not have type ")), f47(v2.clone())?))));
            let a316 = f10(v2.clone());
            let a317 = 4u64;
            if if nat_eq(a316, a317) {
                let a318 = f57(f58(v2));
                let a319 = 5u64;
                nat_eq(a318, a319)
            } else {
                false
            } {
                Ok(v31)
            } else {
                Ok(v32)
            }
        }
        Adt3::C10(v33) => {
            let v34: Result<(), Bytes> = f56(v33.clone(), Adt1::C6)?;
            let v35: Result<(), Bytes> = Err::<(), Bytes>(f4(f1(Str::lit("the literal ")), f4(f54(Adt3::C10(v33))?, f4(f1(Str::lit(" does not have type ")), f47(v2.clone())?))));
            let a320 = f10(v2.clone());
            let a321 = 4u64;
            if if nat_eq(a320, a321) {
                let a322 = f57(f58(v2));
                let a323 = 6u64;
                nat_eq(a322, a323)
            } else {
                false
            } {
                Ok(v34)
            } else {
                Ok(v35)
            }
        }
        Adt3::C11(v36) => {
            let v37: Result<(), Bytes> = f56(v36.clone(), Adt1::C7)?;
            let v38: Result<(), Bytes> = Err::<(), Bytes>(f4(f1(Str::lit("the literal ")), f4(f54(Adt3::C11(v36))?, f4(f1(Str::lit(" does not have type ")), f47(v2.clone())?))));
            let a324 = f10(v2.clone());
            let a325 = 4u64;
            if if nat_eq(a324, a325) {
                let a326 = f57(f58(v2));
                let a327 = 7u64;
                nat_eq(a326, a327)
            } else {
                false
            } {
                Ok(v37)
            } else {
                Ok(v38)
            }
        }
        Adt3::C12(v39) => {
            let v40: Result<(), Bytes> = Ok::<(), Bytes>(());
            let v41: Result<(), Bytes> = Err::<(), Bytes>(f4(f1(Str::lit("the literal ")), f4(f54(Adt3::C12(v39))?, f4(f1(Str::lit(" does not have type ")), f47(v2.clone())?))));
            let a328 = f10(v2);
            let a329 = 5u64;
            if nat_eq(a328, a329) {
                Ok(v40)
            } else {
                Ok(v41)
            }
        }
        Adt3::C13(v42) => {
            let v43: Result<(), Bytes> = f59(v42.clone())?;
            let v44: Result<(), Bytes> = Err::<(), Bytes>(f4(f1(Str::lit("the literal ")), f4(f54(Adt3::C13(v42))?, f4(f1(Str::lit(" does not have type ")), f47(v2.clone())?))));
            let a330 = f10(v2);
            let a331 = 6u64;
            if nat_eq(a330, a331) {
                Ok(v43)
            } else {
                Ok(v44)
            }
        }
        Adt3::C14(v45) => {
            let v46: Result<(), Bytes> = Ok::<(), Bytes>(());
            let v47: Result<(), Bytes> = Err::<(), Bytes>(f4(f1(Str::lit("the literal ")), f4(f54(Adt3::C14(v45))?, f4(f1(Str::lit(" does not have type ")), f47(v2.clone())?))));
            let a332 = f10(v2);
            let a333 = 7u64;
            if nat_eq(a332, a333) {
                Ok(v46)
            } else {
                Ok(v47)
            }
        }
        Adt3::C15 => {
            let v48: Result<(), Bytes> = Ok::<(), Bytes>(());
            let v49: Result<(), Bytes> = Err::<(), Bytes>(f4(f1(Str::lit("the literal ")), f4(f54(Adt3::C15)?, f4(f1(Str::lit(" does not have type ")), f47(v2.clone())?))));
            let a334 = f10(v2);
            let a335 = 8u64;
            if nat_eq(a334, a335) {
                Ok(v48)
            } else {
                Ok(v49)
            }
        }
        Adt3::C16(r361) => {
            let v50 = (*r361).clone();
            let v51: Result<(), Bytes> = f34(v0, v50.clone(), f44(v2.clone()), 0u64)?;
            let v52: Result<(), Bytes> = Err::<(), Bytes>(f4(f1(Str::lit("the literal ")), f4(f54(Adt3::C16(std::rc::Rc::new(v50)))?, f4(f1(Str::lit(" does not have type ")), f47(v2.clone())?))));
            let a336 = f10(v2);
            let a337 = 8u64;
            if nat_eq(a336, a337) {
                Ok(v51)
            } else {
                Ok(v52)
            }
        }
        Adt3::C17(r362) => {
            let v53 = (*r362).clone();
            let v54: Result<(), Bytes> = f34(v0, v53.clone(), f44(v2.clone()), 0u64)?;
            let v55: Result<(), Bytes> = Err::<(), Bytes>(f4(f1(Str::lit("the literal ")), f4(f54(Adt3::C17(std::rc::Rc::new(v53)))?, f4(f1(Str::lit(" does not have type ")), f47(v2.clone())?))));
            let a338 = f10(v2);
            let a339 = 9u64;
            if nat_eq(a338, a339) {
                Ok(v54)
            } else {
                Ok(v55)
            }
        }
        Adt3::C18(r363) => {
            let v56 = (*r363).clone();
            let v57: Result<(), Bytes> = f34(v0, v56.clone(), f45(v2.clone()), 0u64)?;
            let v58: Result<(), Bytes> = Err::<(), Bytes>(f4(f1(Str::lit("the literal ")), f4(f54(Adt3::C18(std::rc::Rc::new(v56)))?, f4(f1(Str::lit(" does not have type ")), f47(v2.clone())?))));
            let a340 = f10(v2);
            let a341 = 9u64;
            if nat_eq(a340, a341) {
                Ok(v57)
            } else {
                Ok(v58)
            }
        }
        Adt3::C19(r364) => {
            let v59 = (*r364).clone();
            let v60: Result<(), Bytes> = f34(v0, v59.clone(), f44(v2.clone()), 1u64)?;
            let v61: Result<(), Bytes> = Err::<(), Bytes>(f4(f1(Str::lit("the literal ")), f4(f54(Adt3::C19(std::rc::Rc::new(v59)))?, f4(f1(Str::lit(" does not have type ")), f47(v2.clone())?))));
            let a342 = f10(v2);
            let a343 = 10u64;
            if nat_eq(a342, a343) {
                Ok(v60)
            } else {
                Ok(v61)
            }
        }
        Adt3::C20(r365, r366) => {
            let v62 = (*r365).clone();
            let v63 = (*r366).clone();
            let v64: Result<(), Bytes> = f34(v0.clone(), v62.clone(), f44(v2.clone()), 0u64)?;
            let v65: Result<(), Bytes> = f34(v0, v63.clone(), f45(v2.clone()), 0u64)?;
            let v68: Result<(), Bytes> = {
                let m344 = v64;
                match m344 {
                    Ok(_) => {
                        v65
                    }
                    Err(v67) => {
                        Err::<(), Bytes>(v67)
                    }
                }
            };
            let v69: Result<(), Bytes> = Err::<(), Bytes>(f4(f1(Str::lit("the literal ")), f4(f54(Adt3::C20(std::rc::Rc::new(v62), std::rc::Rc::new(v63)))?, f4(f1(Str::lit(" does not have type ")), f47(v2.clone())?))));
            let a345 = f10(v2);
            let a346 = 11u64;
            if nat_eq(a345, a346) {
                Ok(v68)
            } else {
                Ok(v69)
            }
        }
        Adt3::C21(v70, r367) => {
            let v71 = (*r367).clone();
            let v77: Result<(), Bytes> = {
                let m347 = f60(v0.clone(), f48(v2.clone()), v70)?;
                match m347 {
                    Ok(v73) => {
                        let v74: Result<(), Bytes> = f34(v0, v71.clone(), v73.clone(), 2u64)?;
                        let v75: Result<(), Bytes> = v74;
                        let v76: Result<(), Bytes> = Err::<(), Bytes>(f4(f1(Str::lit("constructor ")), f4(f7(v70)?, f4(f1(Str::lit(" of ADT ")), f4(f7(f48(v2.clone()))?, f4(f1(Str::lit(" takes ")), f4(f7(f18(v73.clone())?)?, f4(f1(Str::lit(" fields, the literal has ")), f7(f61(v71.clone())?)?))))))));
                        let a348 = f61(v71.clone())?;
                        let a349 = f18(v73)?;
                        if nat_eq(a348, a349) {
                            v75
                        } else {
                            v76
                        }
                    }
                    Err(v72) => {
                        Err::<(), Bytes>(v72)
                    }
                }
            };
            let v78: Result<(), Bytes> = Err::<(), Bytes>(f4(f1(Str::lit("the literal ")), f4(f54(Adt3::C21(v70, std::rc::Rc::new(v71)))?, f4(f1(Str::lit(" does not have type ")), f47(v2.clone())?))));
            let a350 = f10(v2);
            let a351 = 12u64;
            if nat_eq(a350, a351) {
                Ok(v77)
            } else {
                Ok(v78)
            }
        }
        Adt3::C22(_, _) => {
            Ok(Err::<(), Bytes>(f1(Str::lit("a closure has no literal form; build it with `closure`"))))
        }
        Adt3::C23 => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt3::C24(r368, r369) => {
            let v81 = (*r368).clone();
            let v82 = (*r369).clone();
            let v83: Adt0 = {
                let a352 = v3;
                let a353 = 2u64;
                if nat_eq(a352, a353) {
                    f44(v2.clone())
                } else {
                    v2.clone()
                }
            };
            let v84: Adt0 = {
                let a354 = v3;
                let a355 = 2u64;
                if nat_eq(a354, a355) {
                    f45(v2.clone())
                } else {
                    v2.clone()
                }
            };
            let v85: Result<(), Bytes> = f34(v0.clone(), v81, v83, 0u64)?;
            let v86: Result<(), Bytes> = f34(v0, v82, v84, v3)?;
            let a356 = v3;
            let a357 = 0u64;
            if nat_eq(a356, a357) {
                Ok(Ok::<(), Bytes>(()))
            } else {
                let a358 = v3;
                let a359 = 2u64;
                if if nat_eq(a358, a359) {
                    f6(v2)
                } else {
                    false
                } {
                    Ok(Ok::<(), Bytes>(()))
                } else {
                    let m360 = v85;
                    match m360 {
                        Ok(_) => {
                            Ok(v86)
                        }
                        Err(v88) => {
                            Ok(Err::<(), Bytes>(v88))
                        }
                    }
                }
            }
        }
    }
}

pub fn f35(v0: List<(u64, Adt0)>, v1: u64) -> Option<Adt0> {
    let m370 = v0;
    match m370.uncons() {
        None => {
            None::<Adt0>
        }
        Some((v2, v3)) => {
            let v4: Option<Adt0> = f35(v3, v1);
            let a372 = {
                let (h371, _) = v2;
                h371
            };
            let a373 = v1;
            if nat_eq(a372, a373) {
                Some({
                    let (_, h374) = v2;
                    h374
                })
            } else {
                v4
            }
        }
    }
}

pub fn f36(v0: u64, v1: Adt0, v2: List<(u64, Adt0)>) -> List<(u64, Adt0)> {
    List::cons((v0, v1), v2)
}

pub fn f37(v0: Adt7, v1: Adt0) -> R<Result<List<Adt5>, Bytes>> {
    let m375 = v1.clone();
    match m375 {
        Adt0::C0 => {
            Ok(Ok::<List<Adt5>, Bytes>(List::cons(Adt5::C11, List::<Adt5>::nil())))
        }
        Adt0::C1 => {
            Ok(Ok::<List<Adt5>, Bytes>(List::cons(Adt5::C9, List::cons(Adt5::C10, List::<Adt5>::nil()))))
        }
        Adt0::C2 => {
            Ok(Ok::<List<Adt5>, Bytes>(List::cons(Adt5::C6, List::cons(Adt5::C7, List::<Adt5>::nil()))))
        }
        Adt0::C3 => {
            Ok(Err::<List<Adt5>, Bytes>(f4(f1(Str::lit("a value of type ")), f4(f47(v1)?, f1(Str::lit(" cannot be matched"))))))
        }
        Adt0::C4(_) => {
            Ok(Err::<List<Adt5>, Bytes>(f4(f1(Str::lit("a value of type ")), f4(f47(v1)?, f1(Str::lit(" cannot be matched"))))))
        }
        Adt0::C5 => {
            Ok(Err::<List<Adt5>, Bytes>(f4(f1(Str::lit("a value of type ")), f4(f47(v1)?, f1(Str::lit(" cannot be matched"))))))
        }
        Adt0::C6 => {
            Ok(Err::<List<Adt5>, Bytes>(f4(f1(Str::lit("a value of type ")), f4(f47(v1)?, f1(Str::lit(" cannot be matched"))))))
        }
        Adt0::C7 => {
            Ok(Ok::<List<Adt5>, Bytes>(List::cons(Adt5::C12, List::cons(Adt5::C13, List::cons(Adt5::C14, List::<Adt5>::nil())))))
        }
        Adt0::C8(_) => {
            Ok(Ok::<List<Adt5>, Bytes>(List::cons(Adt5::C0, List::cons(Adt5::C1, List::<Adt5>::nil()))))
        }
        Adt0::C9(_, _) => {
            Ok(Ok::<List<Adt5>, Bytes>(List::cons(Adt5::C2, List::cons(Adt5::C3, List::<Adt5>::nil()))))
        }
        Adt0::C10(_) => {
            Ok(Ok::<List<Adt5>, Bytes>(List::cons(Adt5::C4, List::cons(Adt5::C5, List::<Adt5>::nil()))))
        }
        Adt0::C11(_, _) => {
            Ok(Ok::<List<Adt5>, Bytes>(List::cons(Adt5::C8, List::<Adt5>::nil())))
        }
        Adt0::C12(v9) => {
            let m376 = f26(v0, v9);
            match m376 {
                None => {
                    Ok(Err::<List<Adt5>, Bytes>(f27(v9)?))
                }
                Some(v10) => {
                    Ok(Ok::<List<Adt5>, Bytes>(f62(v10, 0u64)?))
                }
            }
        }
        Adt0::C13(_, _) => {
            Ok(Err::<List<Adt5>, Bytes>(f4(f1(Str::lit("a value of type ")), f4(f47(v1)?, f1(Str::lit(" cannot be matched"))))))
        }
        Adt0::C14 => {
            Ok(Err::<List<Adt5>, Bytes>(f4(f1(Str::lit("a value of type ")), f4(f47(v1)?, f1(Str::lit(" cannot be matched"))))))
        }
        Adt0::C15(_, _) => {
            Ok(Err::<List<Adt5>, Bytes>(f4(f1(Str::lit("a value of type ")), f4(f47(v1)?, f1(Str::lit(" cannot be matched"))))))
        }
    }
}

pub fn f38(v0: List<Adt5>, v1: List<Adt5>) -> Option<Adt5> {
    let m377 = v0;
    match m377.uncons() {
        None => {
            None::<Adt5>
        }
        Some((v2, v3)) => {
            let v4: Option<Adt5> = f38(v3, v1.clone());
            let v5: Option<Adt5> = v4;
            let v6: Option<Adt5> = Some(v2.clone());
            if f50(v1, v2) {
                v5
            } else {
                v6
            }
        }
    }
}

pub fn f39(v0: Adt5) -> R<Bytes> {
    let m378 = v0;
    match m378 {
        Adt5::C0 => {
            Ok(f1(Str::lit("None")))
        }
        Adt5::C1 => {
            Ok(f1(Str::lit("Some")))
        }
        Adt5::C2 => {
            Ok(f1(Str::lit("Ok")))
        }
        Adt5::C3 => {
            Ok(f1(Str::lit("Error")))
        }
        Adt5::C4 => {
            Ok(f1(Str::lit("Nil")))
        }
        Adt5::C5 => {
            Ok(f1(Str::lit("Cons")))
        }
        Adt5::C6 => {
            Ok(f1(Str::lit("Zero")))
        }
        Adt5::C7 => {
            Ok(f1(Str::lit("Succ")))
        }
        Adt5::C8 => {
            Ok(f1(Str::lit("Pair")))
        }
        Adt5::C9 => {
            Ok(f1(Str::lit("True")))
        }
        Adt5::C10 => {
            Ok(f1(Str::lit("False")))
        }
        Adt5::C11 => {
            Ok(f1(Str::lit("Unit")))
        }
        Adt5::C12 => {
            Ok(f1(Str::lit("Lt")))
        }
        Adt5::C13 => {
            Ok(f1(Str::lit("Eq")))
        }
        Adt5::C14 => {
            Ok(f1(Str::lit("Gt")))
        }
        Adt5::C15(v1) => {
            Ok(f4(f1(Str::lit("Adt { constructor: ")), f4(f7(v1)?, f1(Str::lit(" }")))))
        }
    }
}

pub fn f40(v0: Adt7, v1: Adt5, v2: Adt0) -> R<Result<Adt0, Bytes>> {
    let m379 = v1.clone();
    match m379 {
        Adt5::C0 => {
            let v3: Result<Adt0, Bytes> = Ok::<Adt0, Bytes>(Adt0::C14);
            let v4: Result<Adt0, Bytes> = Err::<Adt0, Bytes>(f4(f1(Str::lit("shape ")), f4(f39(v1)?, f4(f1(Str::lit(" does not belong to type ")), f47(v2.clone())?))));
            let a380 = f10(v2);
            let a381 = 8u64;
            if nat_eq(a380, a381) {
                Ok(v3)
            } else {
                Ok(v4)
            }
        }
        Adt5::C1 => {
            let v5: Result<Adt0, Bytes> = Ok::<Adt0, Bytes>(Adt0::C15(std::rc::Rc::new(f44(v2.clone())), std::rc::Rc::new(Adt0::C14)));
            let v6: Result<Adt0, Bytes> = Err::<Adt0, Bytes>(f4(f1(Str::lit("shape ")), f4(f39(v1)?, f4(f1(Str::lit(" does not belong to type ")), f47(v2.clone())?))));
            let a382 = f10(v2);
            let a383 = 8u64;
            if nat_eq(a382, a383) {
                Ok(v5)
            } else {
                Ok(v6)
            }
        }
        Adt5::C2 => {
            let v7: Result<Adt0, Bytes> = Ok::<Adt0, Bytes>(Adt0::C15(std::rc::Rc::new(f44(v2.clone())), std::rc::Rc::new(Adt0::C14)));
            let v8: Result<Adt0, Bytes> = Err::<Adt0, Bytes>(f4(f1(Str::lit("shape ")), f4(f39(v1)?, f4(f1(Str::lit(" does not belong to type ")), f47(v2.clone())?))));
            let a384 = f10(v2);
            let a385 = 9u64;
            if nat_eq(a384, a385) {
                Ok(v7)
            } else {
                Ok(v8)
            }
        }
        Adt5::C3 => {
            let v9: Result<Adt0, Bytes> = Ok::<Adt0, Bytes>(Adt0::C15(std::rc::Rc::new(f45(v2.clone())), std::rc::Rc::new(Adt0::C14)));
            let v10: Result<Adt0, Bytes> = Err::<Adt0, Bytes>(f4(f1(Str::lit("shape ")), f4(f39(v1)?, f4(f1(Str::lit(" does not belong to type ")), f47(v2.clone())?))));
            let a386 = f10(v2);
            let a387 = 9u64;
            if nat_eq(a386, a387) {
                Ok(v9)
            } else {
                Ok(v10)
            }
        }
        Adt5::C4 => {
            let v11: Result<Adt0, Bytes> = Ok::<Adt0, Bytes>(Adt0::C14);
            let v12: Result<Adt0, Bytes> = Err::<Adt0, Bytes>(f4(f1(Str::lit("shape ")), f4(f39(v1)?, f4(f1(Str::lit(" does not belong to type ")), f47(v2.clone())?))));
            let a388 = f10(v2);
            let a389 = 10u64;
            if nat_eq(a388, a389) {
                Ok(v11)
            } else {
                Ok(v12)
            }
        }
        Adt5::C5 => {
            let v13: Result<Adt0, Bytes> = Ok::<Adt0, Bytes>(Adt0::C15(std::rc::Rc::new(f44(v2.clone())), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(v2.clone()), std::rc::Rc::new(Adt0::C14)))));
            let v14: Result<Adt0, Bytes> = Err::<Adt0, Bytes>(f4(f1(Str::lit("shape ")), f4(f39(v1)?, f4(f1(Str::lit(" does not belong to type ")), f47(v2.clone())?))));
            let a390 = f10(v2);
            let a391 = 10u64;
            if nat_eq(a390, a391) {
                Ok(v13)
            } else {
                Ok(v14)
            }
        }
        Adt5::C6 => {
            let v15: Result<Adt0, Bytes> = Ok::<Adt0, Bytes>(Adt0::C14);
            let v16: Result<Adt0, Bytes> = Err::<Adt0, Bytes>(f4(f1(Str::lit("shape ")), f4(f39(v1)?, f4(f1(Str::lit(" does not belong to type ")), f47(v2.clone())?))));
            let a392 = f10(v2);
            let a393 = 2u64;
            if nat_eq(a392, a393) {
                Ok(v15)
            } else {
                Ok(v16)
            }
        }
        Adt5::C7 => {
            let v17: Result<Adt0, Bytes> = Ok::<Adt0, Bytes>(Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C14)));
            let v18: Result<Adt0, Bytes> = Err::<Adt0, Bytes>(f4(f1(Str::lit("shape ")), f4(f39(v1)?, f4(f1(Str::lit(" does not belong to type ")), f47(v2.clone())?))));
            let a394 = f10(v2);
            let a395 = 2u64;
            if nat_eq(a394, a395) {
                Ok(v17)
            } else {
                Ok(v18)
            }
        }
        Adt5::C8 => {
            let v19: Result<Adt0, Bytes> = Ok::<Adt0, Bytes>(Adt0::C15(std::rc::Rc::new(f44(v2.clone())), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(f45(v2.clone())), std::rc::Rc::new(Adt0::C14)))));
            let v20: Result<Adt0, Bytes> = Err::<Adt0, Bytes>(f4(f1(Str::lit("shape ")), f4(f39(v1)?, f4(f1(Str::lit(" does not belong to type ")), f47(v2.clone())?))));
            let a396 = f10(v2);
            let a397 = 11u64;
            if nat_eq(a396, a397) {
                Ok(v19)
            } else {
                Ok(v20)
            }
        }
        Adt5::C9 => {
            let v21: Result<Adt0, Bytes> = Ok::<Adt0, Bytes>(Adt0::C14);
            let v22: Result<Adt0, Bytes> = Err::<Adt0, Bytes>(f4(f1(Str::lit("shape ")), f4(f39(v1)?, f4(f1(Str::lit(" does not belong to type ")), f47(v2.clone())?))));
            let a398 = f10(v2);
            let a399 = 1u64;
            if nat_eq(a398, a399) {
                Ok(v21)
            } else {
                Ok(v22)
            }
        }
        Adt5::C10 => {
            let v23: Result<Adt0, Bytes> = Ok::<Adt0, Bytes>(Adt0::C14);
            let v24: Result<Adt0, Bytes> = Err::<Adt0, Bytes>(f4(f1(Str::lit("shape ")), f4(f39(v1)?, f4(f1(Str::lit(" does not belong to type ")), f47(v2.clone())?))));
            let a400 = f10(v2);
            let a401 = 1u64;
            if nat_eq(a400, a401) {
                Ok(v23)
            } else {
                Ok(v24)
            }
        }
        Adt5::C11 => {
            let v25: Result<Adt0, Bytes> = Ok::<Adt0, Bytes>(Adt0::C14);
            let v26: Result<Adt0, Bytes> = Err::<Adt0, Bytes>(f4(f1(Str::lit("shape ")), f4(f39(v1)?, f4(f1(Str::lit(" does not belong to type ")), f47(v2.clone())?))));
            let a402 = f10(v2);
            let a403 = 0u64;
            if nat_eq(a402, a403) {
                Ok(v25)
            } else {
                Ok(v26)
            }
        }
        Adt5::C12 => {
            let v27: Result<Adt0, Bytes> = Ok::<Adt0, Bytes>(Adt0::C14);
            let v28: Result<Adt0, Bytes> = Err::<Adt0, Bytes>(f4(f1(Str::lit("shape ")), f4(f39(v1)?, f4(f1(Str::lit(" does not belong to type ")), f47(v2.clone())?))));
            let a404 = f10(v2);
            let a405 = 7u64;
            if nat_eq(a404, a405) {
                Ok(v27)
            } else {
                Ok(v28)
            }
        }
        Adt5::C13 => {
            let v29: Result<Adt0, Bytes> = Ok::<Adt0, Bytes>(Adt0::C14);
            let v30: Result<Adt0, Bytes> = Err::<Adt0, Bytes>(f4(f1(Str::lit("shape ")), f4(f39(v1)?, f4(f1(Str::lit(" does not belong to type ")), f47(v2.clone())?))));
            let a406 = f10(v2);
            let a407 = 7u64;
            if nat_eq(a406, a407) {
                Ok(v29)
            } else {
                Ok(v30)
            }
        }
        Adt5::C14 => {
            let v31: Result<Adt0, Bytes> = Ok::<Adt0, Bytes>(Adt0::C14);
            let v32: Result<Adt0, Bytes> = Err::<Adt0, Bytes>(f4(f1(Str::lit("shape ")), f4(f39(v1)?, f4(f1(Str::lit(" does not belong to type ")), f47(v2.clone())?))));
            let a408 = f10(v2);
            let a409 = 7u64;
            if nat_eq(a408, a409) {
                Ok(v31)
            } else {
                Ok(v32)
            }
        }
        Adt5::C15(v33) => {
            let v34: Result<Adt0, Bytes> = f60(v0, f48(v2.clone()), v33)?;
            let v35: Result<Adt0, Bytes> = v34;
            let v36: Result<Adt0, Bytes> = Err::<Adt0, Bytes>(f4(f1(Str::lit("shape ")), f4(f39(v1)?, f4(f1(Str::lit(" does not belong to type ")), f47(v2.clone())?))));
            let a410 = f10(v2);
            let a411 = 12u64;
            if nat_eq(a410, a411) {
                Ok(v35)
            } else {
                Ok(v36)
            }
        }
    }
}

pub fn f41(v0: Adt2) -> R<u64> {
    let m412 = v0;
    match m412 {
        Adt2::C0(_, _) => {
            Ok(0u64)
        }
        Adt2::C1(_) => {
            Ok(0u64)
        }
        Adt2::C2(_, _, _, _) => {
            Ok(0u64)
        }
        Adt2::C3(_, _, _) => {
            Ok(0u64)
        }
        Adt2::C4(_, _, _) => {
            Ok(0u64)
        }
        Adt2::C5(_, _, _) => {
            Ok(0u64)
        }
        Adt2::C6(_, _) => {
            Ok(0u64)
        }
        Adt2::C7(_, _) => {
            Ok(0u64)
        }
        Adt2::C8(_, _) => {
            Ok(0u64)
        }
        Adt2::C9(_, _) => {
            Ok(0u64)
        }
        Adt2::C10(_) => {
            Ok(0u64)
        }
        Adt2::C11(_) => {
            Ok(0u64)
        }
        Adt2::C12(_, _) => {
            Ok(0u64)
        }
        Adt2::C13 => {
            Ok(0u64)
        }
        Adt2::C14(_, r415) => {
            let v33 = (*r415).clone();
            let v34: u64 = f41(v33)?;
            let a413 = 1u64;
            let a414 = v34;
            nat_add(a413, a414)
        }
        Adt2::C15(_, _, _) => {
            Ok(0u64)
        }
    }
}

pub fn f42(v0: u64) -> R<Bytes> {
    Ok(f4(f1(Str::lit("function ")), f4(f7(v0)?, f1(Str::lit(" is not declared")))))
}

pub fn f43(v0: Adt0, v1: Adt0) -> Adt0 {
    Adt0::C13(std::rc::Rc::new(v0), std::rc::Rc::new(v1))
}

pub fn f44(v0: Adt0) -> Adt0 {
    let m416 = f63(v0);
    match m416 {
        None => {
            Adt0::C0
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f45(v0: Adt0) -> Adt0 {
    let m417 = f64(v0);
    match m417 {
        None => {
            Adt0::C0
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f46(v0: Adt6, v1: Adt0) -> R<Result<Adt0, Bytes>> {
    let v2: Adt0 = f44(v1.clone());
    let v3: Adt0 = if f6(v1.clone()) {
        Adt0::C0
    } else {
        v2
    };
    let m418 = v0.clone();
    match m418 {
        Adt6::C0 => {
            f65(v0, Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C14)))), v1, Adt0::C2)
        }
        Adt6::C1 => {
            f65(v0, Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C14)))), v1, Adt0::C2)
        }
        Adt6::C2 => {
            f65(v0, Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C14)))), v1, Adt0::C2)
        }
        Adt6::C3 => {
            f65(v0, Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C14)))))), v1, Adt0::C2)
        }
        Adt6::C4 => {
            f65(v0, Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C14)))))), v1, Adt0::C2)
        }
        Adt6::C5 => {
            f65(v0, Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C14)))), v1, Adt0::C1)
        }
        Adt6::C6 => {
            f65(v0, Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C14)))), v1, Adt0::C1)
        }
        Adt6::C7 => {
            f65(v0, Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C14)))), v1, Adt0::C1)
        }
        Adt6::C8 => {
            f65(v0, Adt0::C15(std::rc::Rc::new(Adt0::C3), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C3), std::rc::Rc::new(Adt0::C14)))), v1, Adt0::C3)
        }
        Adt6::C9 => {
            f65(v0, Adt0::C15(std::rc::Rc::new(Adt0::C3), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C3), std::rc::Rc::new(Adt0::C14)))), v1, Adt0::C3)
        }
        Adt6::C10 => {
            f65(v0, Adt0::C15(std::rc::Rc::new(Adt0::C3), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C3), std::rc::Rc::new(Adt0::C14)))), v1, Adt0::C3)
        }
        Adt6::C11 => {
            f65(v0, Adt0::C15(std::rc::Rc::new(Adt0::C3), std::rc::Rc::new(Adt0::C14)), v1, Adt0::C3)
        }
        Adt6::C12 => {
            f65(v0, Adt0::C15(std::rc::Rc::new(Adt0::C3), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C3), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C3), std::rc::Rc::new(Adt0::C14)))))), v1, Adt0::C3)
        }
        Adt6::C13 => {
            f65(v0, Adt0::C15(std::rc::Rc::new(Adt0::C3), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C3), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C3), std::rc::Rc::new(Adt0::C14)))))), v1, Adt0::C3)
        }
        Adt6::C14 => {
            let v4: Result<Adt0, Bytes> = f65(v0.clone(), Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C14)))), v1.clone(), Adt0::C8(std::rc::Rc::new(v3.clone())))?;
            let v5: Result<Adt0, Bytes> = f66(v0, v1)?;
            let a419 = f10(v3);
            let a420 = 4u64;
            if nat_eq(a419, a420) {
                Ok(v4)
            } else {
                Ok(v5)
            }
        }
        Adt6::C15 => {
            let v6: Result<Adt0, Bytes> = f65(v0.clone(), Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C14)))), v1.clone(), Adt0::C8(std::rc::Rc::new(v3.clone())))?;
            let v7: Result<Adt0, Bytes> = f66(v0, v1)?;
            let a421 = f10(v3);
            let a422 = 4u64;
            if nat_eq(a421, a422) {
                Ok(v6)
            } else {
                Ok(v7)
            }
        }
        Adt6::C16 => {
            let v8: Result<Adt0, Bytes> = f65(v0.clone(), Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C14)))), v1.clone(), Adt0::C8(std::rc::Rc::new(v3.clone())))?;
            let v9: Result<Adt0, Bytes> = f66(v0, v1)?;
            let a423 = f10(v3);
            let a424 = 4u64;
            if nat_eq(a423, a424) {
                Ok(v8)
            } else {
                Ok(v9)
            }
        }
        Adt6::C17 => {
            let v12: Result<Adt0, Bytes> = f65(v0.clone(), Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C14)), v1.clone(), Adt0::C8(std::rc::Rc::new(v3.clone())))?;
            let v13: Result<Adt0, Bytes> = f66(v0, v1)?;
            let a427 = f10(v3.clone());
            let a428 = 4u64;
            if if nat_eq(a427, a428) {
                f67(f58(v3))
            } else {
                false
            } {
                Ok(v12)
            } else {
                Ok(v13)
            }
        }
        Adt6::C18 => {
            let v10: Result<Adt0, Bytes> = f65(v0.clone(), Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C14)))), v1.clone(), Adt0::C8(std::rc::Rc::new(v3.clone())))?;
            let v11: Result<Adt0, Bytes> = f66(v0, v1)?;
            let a425 = f10(v3);
            let a426 = 4u64;
            if nat_eq(a425, a426) {
                Ok(v10)
            } else {
                Ok(v11)
            }
        }
        Adt6::C19 => {
            let v14: Result<Adt0, Bytes> = f65(v0.clone(), Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C14)))), v1.clone(), v3.clone())?;
            let v15: Result<Adt0, Bytes> = f66(v0, v1)?;
            let a429 = f10(v3);
            let a430 = 4u64;
            if nat_eq(a429, a430) {
                Ok(v14)
            } else {
                Ok(v15)
            }
        }
        Adt6::C20 => {
            let v16: Result<Adt0, Bytes> = f65(v0.clone(), Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C14)))), v1.clone(), v3.clone())?;
            let v17: Result<Adt0, Bytes> = f66(v0, v1)?;
            let a431 = f10(v3);
            let a432 = 4u64;
            if nat_eq(a431, a432) {
                Ok(v16)
            } else {
                Ok(v17)
            }
        }
        Adt6::C21 => {
            let v18: Result<Adt0, Bytes> = f65(v0.clone(), Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C14)))), v1.clone(), v3.clone())?;
            let v19: Result<Adt0, Bytes> = f66(v0, v1)?;
            let a433 = f10(v3);
            let a434 = 4u64;
            if nat_eq(a433, a434) {
                Ok(v18)
            } else {
                Ok(v19)
            }
        }
        Adt6::C22 => {
            let v20: Result<Adt0, Bytes> = f65(v0.clone(), Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C14)), v1.clone(), v3.clone())?;
            let v21: Result<Adt0, Bytes> = f66(v0, v1)?;
            let a435 = f10(v3);
            let a436 = 4u64;
            if nat_eq(a435, a436) {
                Ok(v20)
            } else {
                Ok(v21)
            }
        }
        Adt6::C23 => {
            let v22: Result<Adt0, Bytes> = f65(v0.clone(), Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C4(Adt1::C2)), std::rc::Rc::new(Adt0::C14)))), v1.clone(), Adt0::C8(std::rc::Rc::new(v3.clone())))?;
            let v23: Result<Adt0, Bytes> = f66(v0, v1)?;
            let a437 = f10(v3);
            let a438 = 4u64;
            if nat_eq(a437, a438) {
                Ok(v22)
            } else {
                Ok(v23)
            }
        }
        Adt6::C24 => {
            let v24: Result<Adt0, Bytes> = f65(v0.clone(), Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C4(Adt1::C2)), std::rc::Rc::new(Adt0::C14)))), v1.clone(), Adt0::C8(std::rc::Rc::new(v3.clone())))?;
            let v25: Result<Adt0, Bytes> = f66(v0, v1)?;
            let a439 = f10(v3);
            let a440 = 4u64;
            if nat_eq(a439, a440) {
                Ok(v24)
            } else {
                Ok(v25)
            }
        }
        Adt6::C25 => {
            let v26: Result<Adt0, Bytes> = f65(v0.clone(), Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C14)))), v1.clone(), Adt0::C1)?;
            let v27: Result<Adt0, Bytes> = f66(v0, v1)?;
            let a441 = f10(v3.clone());
            let a442 = 2u64;
            if if nat_eq(a441, a442) {
                true
            } else {
                let a443 = f10(v3.clone());
                let a444 = 1u64;
                if nat_eq(a443, a444) {
                    true
                } else {
                    let a445 = f10(v3.clone());
                    let a446 = 4u64;
                    if nat_eq(a445, a446) {
                        true
                    } else {
                        let a447 = f10(v3.clone());
                        let a448 = 5u64;
                        if nat_eq(a447, a448) {
                            true
                        } else {
                            let a449 = f10(v3.clone());
                            let a450 = 6u64;
                            if nat_eq(a449, a450) {
                                true
                            } else {
                                let a451 = f10(v3);
                                let a452 = 7u64;
                                nat_eq(a451, a452)
                            }
                        }
                    }
                }
            } {
                Ok(v26)
            } else {
                Ok(v27)
            }
        }
        Adt6::C26 => {
            f65(v0, Adt0::C15(std::rc::Rc::new(Adt0::C1), std::rc::Rc::new(Adt0::C14)), v1, Adt0::C1)
        }
        Adt6::C27 => {
            f65(v0, Adt0::C15(std::rc::Rc::new(Adt0::C1), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C1), std::rc::Rc::new(Adt0::C14)))), v1, Adt0::C1)
        }
        Adt6::C28 => {
            f65(v0, Adt0::C15(std::rc::Rc::new(Adt0::C1), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C1), std::rc::Rc::new(Adt0::C14)))), v1, Adt0::C1)
        }
        Adt6::C29 => {
            let v28: Result<Adt0, Bytes> = f65(v0.clone(), Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C14)))), v1.clone(), v3.clone())?;
            let v29: Result<Adt0, Bytes> = f66(v0, v1)?;
            let a453 = f10(v3.clone());
            let a454 = 10u64;
            if if nat_eq(a453, a454) {
                true
            } else {
                let a455 = f10(v3);
                let a456 = 6u64;
                nat_eq(a455, a456)
            } {
                Ok(v28)
            } else {
                Ok(v29)
            }
        }
        Adt6::C30 => {
            let v30: Result<Adt0, Bytes> = f65(v0.clone(), Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C14)), v1.clone(), Adt0::C2)?;
            let v31: Result<Adt0, Bytes> = f66(v0, v1)?;
            let a457 = f10(v3.clone());
            let a458 = 10u64;
            if if nat_eq(a457, a458) {
                true
            } else {
                let a459 = f10(v3.clone());
                let a460 = 6u64;
                if nat_eq(a459, a460) {
                    true
                } else {
                    let a461 = f10(v3);
                    let a462 = 5u64;
                    nat_eq(a461, a462)
                }
            } {
                Ok(v30)
            } else {
                Ok(v31)
            }
        }
        Adt6::C31 => {
            let v32: Result<Adt0, Bytes> = f65(v0.clone(), Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C14)))), v1.clone(), Adt0::C8(std::rc::Rc::new(f44(v3.clone()))))?;
            let v33: Result<Adt0, Bytes> = f65(v0, Adt0::C15(std::rc::Rc::new(Adt0::C6), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C14)))), v1, Adt0::C8(std::rc::Rc::new(Adt0::C4(Adt1::C0))))?;
            let a463 = f10(v3.clone());
            let a464 = 10u64;
            if nat_eq(a463, a464) {
                Ok(v32)
            } else {
                let a465 = f10(v3.clone());
                let a466 = 6u64;
                if nat_eq(a465, a466) {
                    Ok(v33)
                } else {
                    Ok(Err::<Adt0, Bytes>(f4(f1(Str::lit("primitive Index of ")), f47(v3)?)))
                }
            }
        }
        Adt6::C32 => {
            let v34: Result<Adt0, Bytes> = f65(v0.clone(), Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C14)))))), v1.clone(), Adt0::C8(std::rc::Rc::new(v3.clone())))?;
            let v35: Result<Adt0, Bytes> = f66(v0, v1)?;
            let a467 = f10(v3.clone());
            let a468 = 10u64;
            if if nat_eq(a467, a468) {
                true
            } else {
                let a469 = f10(v3);
                let a470 = 6u64;
                nat_eq(a469, a470)
            } {
                Ok(v34)
            } else {
                Ok(v35)
            }
        }
        Adt6::C33 => {
            f65(v0, Adt0::C15(std::rc::Rc::new(Adt0::C5), std::rc::Rc::new(Adt0::C14)), v1, Adt0::C6)
        }
        Adt6::C34 => {
            f65(v0, Adt0::C15(std::rc::Rc::new(Adt0::C6), std::rc::Rc::new(Adt0::C14)), v1, Adt0::C8(std::rc::Rc::new(Adt0::C5)))
        }
        Adt6::C35 => {
            f65(v0, Adt0::C15(std::rc::Rc::new(Adt0::C6), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C6), std::rc::Rc::new(Adt0::C14)))), v1, Adt0::C7)
        }
        Adt6::C36 => {
            f65(v0, Adt0::C15(std::rc::Rc::new(Adt0::C5), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C5), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C4(Adt1::C2)), std::rc::Rc::new(Adt0::C14)))))), v1, Adt0::C8(std::rc::Rc::new(Adt0::C10(std::rc::Rc::new(Adt0::C5)))))
        }
        Adt6::C37 => {
            f65(v0, Adt0::C15(std::rc::Rc::new(Adt0::C10(std::rc::Rc::new(Adt0::C5))), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C5), std::rc::Rc::new(Adt0::C14)))), v1, Adt0::C5)
        }
        Adt6::C38 => {
            let v36: Result<Adt0, Bytes> = f65(v0.clone(), Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C14)), v1.clone(), Adt0::C5)?;
            let v37: Result<Adt0, Bytes> = f66(v0, v1)?;
            let a471 = f10(v3.clone());
            let a472 = 3u64;
            if if nat_eq(a471, a472) {
                true
            } else {
                let a473 = f10(v3);
                let a474 = 4u64;
                nat_eq(a473, a474)
            } {
                Ok(v36)
            } else {
                Ok(v37)
            }
        }
        Adt6::C39 => {
            let v38: Result<Adt0, Bytes> = f65(v0.clone(), Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C14)))), v1.clone(), Adt0::C7)?;
            let v39: Result<Adt0, Bytes> = f66(v0, v1)?;
            if f68(v3) {
                Ok(v38)
            } else {
                Ok(v39)
            }
        }
        Adt6::C40(v40) => {
            let v41: Result<Adt0, Bytes> = f65(v0.clone(), Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C14)), v1.clone(), Adt0::C8(std::rc::Rc::new(Adt0::C4(v40))))?;
            let v42: Result<Adt0, Bytes> = f66(v0, v1)?;
            let a475 = f10(v3.clone());
            let a476 = 3u64;
            if if nat_eq(a475, a476) {
                true
            } else {
                let a477 = f10(v3);
                let a478 = 4u64;
                nat_eq(a477, a478)
            } {
                Ok(v41)
            } else {
                Ok(v42)
            }
        }
        Adt6::C41(v43) => {
            let v44: u64 = f10(v43.clone());
            let v45: Result<Adt0, Bytes> = f65(v0.clone(), Adt0::C15(std::rc::Rc::new(Adt0::C5), std::rc::Rc::new(Adt0::C14)), v1.clone(), Adt0::C8(std::rc::Rc::new(v43)))?;
            let v46: Result<Adt0, Bytes> = f66(v0, v1)?;
            let a479 = v44;
            let a480 = 3u64;
            if if nat_eq(a479, a480) {
                true
            } else {
                let a481 = v44;
                let a482 = 4u64;
                nat_eq(a481, a482)
            } {
                Ok(v45)
            } else {
                Ok(v46)
            }
        }
    }
}

pub fn f47(v0: Adt0) -> R<Bytes> {
    let m483 = v0;
    match m483 {
        Adt0::C0 => {
            Ok(f1(Str::lit("Unit")))
        }
        Adt0::C1 => {
            Ok(f1(Str::lit("Bool")))
        }
        Adt0::C2 => {
            Ok(f1(Str::lit("Nat")))
        }
        Adt0::C3 => {
            Ok(f1(Str::lit("Int")))
        }
        Adt0::C4(v1) => {
            Ok(f4(f1(Str::lit("Fixed { width: ")), f4(f69(v1), f1(Str::lit(" }")))))
        }
        Adt0::C5 => {
            Ok(f1(Str::lit("String")))
        }
        Adt0::C6 => {
            Ok(f1(Str::lit("Bytes")))
        }
        Adt0::C7 => {
            Ok(f1(Str::lit("Ordering")))
        }
        Adt0::C8(r484) => {
            let v2 = (*r484).clone();
            Ok(f4(f1(Str::lit("Option { value: ")), f4(f47(v2)?, f1(Str::lit(" }")))))
        }
        Adt0::C9(r485, r486) => {
            let v3 = (*r485).clone();
            let v4 = (*r486).clone();
            Ok(f4(f1(Str::lit("Result { ok: ")), f4(f47(v3)?, f4(f1(Str::lit(", error: ")), f4(f47(v4)?, f1(Str::lit(" }")))))))
        }
        Adt0::C10(r487) => {
            let v5 = (*r487).clone();
            Ok(f4(f1(Str::lit("List { element: ")), f4(f47(v5)?, f1(Str::lit(" }")))))
        }
        Adt0::C11(r488, r489) => {
            let v6 = (*r488).clone();
            let v7 = (*r489).clone();
            Ok(f4(f1(Str::lit("Pair { left: ")), f4(f47(v6)?, f4(f1(Str::lit(", right: ")), f4(f47(v7)?, f1(Str::lit(" }")))))))
        }
        Adt0::C12(v8) => {
            Ok(f4(f1(Str::lit("Adt { index: ")), f4(f7(v8)?, f1(Str::lit(" }")))))
        }
        Adt0::C13(r490, r491) => {
            let v9 = (*r490).clone();
            let v10 = (*r491).clone();
            Ok(f4(f1(Str::lit("Fn { parameters: [")), f4(f47(v9)?, f4(f1(Str::lit("], result: ")), f4(f47(v10)?, f1(Str::lit(" }")))))))
        }
        Adt0::C14 => {
            Ok(f1(Str::lit("")))
        }
        Adt0::C15(r492, r493) => {
            let v11 = (*r492).clone();
            let v12 = (*r493).clone();
            let v13: Bytes = f47(v11)?;
            let v14: Bytes = f47(v12.clone())?;
            if f6(v12) {
                Ok(v13)
            } else {
                Ok(f4(v13, f4(f1(Str::lit(", ")), v14)))
            }
        }
    }
}

pub fn f48(v0: Adt0) -> u64 {
    let m494 = v0;
    match m494 {
        Adt0::C0 => {
            0u64
        }
        Adt0::C1 => {
            0u64
        }
        Adt0::C2 => {
            0u64
        }
        Adt0::C3 => {
            0u64
        }
        Adt0::C4(_) => {
            0u64
        }
        Adt0::C5 => {
            0u64
        }
        Adt0::C6 => {
            0u64
        }
        Adt0::C7 => {
            0u64
        }
        Adt0::C8(_) => {
            0u64
        }
        Adt0::C9(_, _) => {
            0u64
        }
        Adt0::C10(_) => {
            0u64
        }
        Adt0::C11(_, _) => {
            0u64
        }
        Adt0::C12(v8) => {
            v8
        }
        Adt0::C13(_, _) => {
            0u64
        }
        Adt0::C14 => {
            0u64
        }
        Adt0::C15(_, _) => {
            0u64
        }
    }
}

pub fn f49(v0: Adt0, v1: Adt0) -> Adt0 {
    Adt0::C15(std::rc::Rc::new(v0), std::rc::Rc::new(v1))
}

pub fn f50(v0: List<Adt5>, v1: Adt5) -> bool {
    let m495 = v0;
    match m495.uncons() {
        None => {
            false
        }
        Some((v2, v3)) => {
            let v4: bool = f70(v2, v1.clone());
            let v5: bool = f50(v3, v1);
            if v4 {
                true
            } else {
                v5
            }
        }
    }
}

pub fn f51(v0: Adt5, v1: List<Adt5>) -> List<Adt5> {
    List::cons(v0, v1)
}

pub fn f52(v0: Adt0, v1: Adt0) -> bool {
    let m496 = v0;
    match m496 {
        Adt0::C0 => {
            let a497 = f10(v1);
            let a498 = 0u64;
            nat_eq(a497, a498)
        }
        Adt0::C1 => {
            let a499 = f10(v1);
            let a500 = 1u64;
            nat_eq(a499, a500)
        }
        Adt0::C2 => {
            let a501 = f10(v1);
            let a502 = 2u64;
            nat_eq(a501, a502)
        }
        Adt0::C3 => {
            let a503 = f10(v1);
            let a504 = 3u64;
            nat_eq(a503, a504)
        }
        Adt0::C4(v2) => {
            let a513 = f10(v1.clone());
            let a514 = 4u64;
            if nat_eq(a513, a514) {
                let a515 = f57(v2);
                let a516 = f57(f58(v1));
                nat_eq(a515, a516)
            } else {
                false
            }
        }
        Adt0::C5 => {
            let a505 = f10(v1);
            let a506 = 5u64;
            nat_eq(a505, a506)
        }
        Adt0::C6 => {
            let a507 = f10(v1);
            let a508 = 6u64;
            nat_eq(a507, a508)
        }
        Adt0::C7 => {
            let a509 = f10(v1);
            let a510 = 7u64;
            nat_eq(a509, a510)
        }
        Adt0::C8(r533) => {
            let v3 = (*r533).clone();
            let v4: bool = f52(v3, f44(v1.clone()));
            let a517 = f10(v1);
            let a518 = 8u64;
            if nat_eq(a517, a518) {
                v4
            } else {
                false
            }
        }
        Adt0::C9(r534, r535) => {
            let v5 = (*r534).clone();
            let v6 = (*r535).clone();
            let v7: bool = f52(v5, f44(v1.clone()));
            let v8: bool = f52(v6, f45(v1.clone()));
            let a519 = f10(v1);
            let a520 = 9u64;
            if nat_eq(a519, a520) {
                if v7 {
                    v8
                } else {
                    false
                }
            } else {
                false
            }
        }
        Adt0::C10(r536) => {
            let v9 = (*r536).clone();
            let v10: bool = f52(v9, f44(v1.clone()));
            let a521 = f10(v1);
            let a522 = 10u64;
            if nat_eq(a521, a522) {
                v10
            } else {
                false
            }
        }
        Adt0::C11(r537, r538) => {
            let v11 = (*r537).clone();
            let v12 = (*r538).clone();
            let v13: bool = f52(v11, f44(v1.clone()));
            let v14: bool = f52(v12, f45(v1.clone()));
            let a523 = f10(v1);
            let a524 = 11u64;
            if nat_eq(a523, a524) {
                if v13 {
                    v14
                } else {
                    false
                }
            } else {
                false
            }
        }
        Adt0::C12(v15) => {
            let a525 = f10(v1.clone());
            let a526 = 12u64;
            if nat_eq(a525, a526) {
                let a527 = v15;
                let a528 = f48(v1);
                nat_eq(a527, a528)
            } else {
                false
            }
        }
        Adt0::C13(r539, r540) => {
            let v16 = (*r539).clone();
            let v17 = (*r540).clone();
            let v18: bool = f52(v16, f44(v1.clone()));
            let v19: bool = f52(v17, f45(v1.clone()));
            let a529 = f10(v1);
            let a530 = 13u64;
            if nat_eq(a529, a530) {
                if v18 {
                    v19
                } else {
                    false
                }
            } else {
                false
            }
        }
        Adt0::C14 => {
            let a511 = f10(v1);
            let a512 = 14u64;
            nat_eq(a511, a512)
        }
        Adt0::C15(r541, r542) => {
            let v20 = (*r541).clone();
            let v21 = (*r542).clone();
            let v22: bool = f52(v20, f44(v1.clone()));
            let v23: bool = f52(v21, f45(v1.clone()));
            let a531 = f10(v1);
            let a532 = 15u64;
            if nat_eq(a531, a532) {
                if v22 {
                    v23
                } else {
                    false
                }
            } else {
                false
            }
        }
    }
}

pub fn f53(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f71(v0, v1, 48u8) {
        Ok(Some({
            let a543 = v1;
            let a544 = 1u64;
            nat_add(a543, a544)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f54(v0: Adt3) -> R<Bytes> {
    let m545 = v0;
    match m545 {
        Adt3::C0 => {
            Ok(f1(Str::lit("Unit")))
        }
        Adt3::C1(v1) => {
            Ok(f4(f1(Str::lit("Bool { value: ")), f4(f72(v1), f1(Str::lit(" }")))))
        }
        Adt3::C2(v2) => {
            Ok(f4(f1(Str::lit("Nat { value: ")), f4(f73(v2)?, f1(Str::lit(" }")))))
        }
        Adt3::C3(v3) => {
            Ok(f4(f1(Str::lit("Int { value: ")), f4(f73(v3)?, f1(Str::lit(" }")))))
        }
        Adt3::C4(v4) => {
            Ok(f4(f1(Str::lit("U8 { value: ")), f4(f73(v4)?, f1(Str::lit(" }")))))
        }
        Adt3::C5(v5) => {
            Ok(f4(f1(Str::lit("U16 { value: ")), f4(f73(v5)?, f1(Str::lit(" }")))))
        }
        Adt3::C6(v6) => {
            Ok(f4(f1(Str::lit("U32 { value: ")), f4(f73(v6)?, f1(Str::lit(" }")))))
        }
        Adt3::C7(v7) => {
            Ok(f4(f1(Str::lit("U64 { value: ")), f4(f73(v7)?, f1(Str::lit(" }")))))
        }
        Adt3::C8(v8) => {
            Ok(f4(f1(Str::lit("I8 { value: ")), f4(f73(v8)?, f1(Str::lit(" }")))))
        }
        Adt3::C9(v9) => {
            Ok(f4(f1(Str::lit("I16 { value: ")), f4(f73(v9)?, f1(Str::lit(" }")))))
        }
        Adt3::C10(v10) => {
            Ok(f4(f1(Str::lit("I32 { value: ")), f4(f73(v10)?, f1(Str::lit(" }")))))
        }
        Adt3::C11(v11) => {
            Ok(f4(f1(Str::lit("I64 { value: ")), f4(f73(v11)?, f1(Str::lit(" }")))))
        }
        Adt3::C12(v12) => {
            Ok(f4(f1(Str::lit("String { value: ")), f4(f73(v12)?, f1(Str::lit(" }")))))
        }
        Adt3::C13(v13) => {
            Ok(f4(f1(Str::lit("Bytes { hex: ")), f4(f73(v13)?, f1(Str::lit(" }")))))
        }
        Adt3::C14(v14) => {
            Ok(f4(f1(Str::lit("Ordering { value: ")), f4(f74(v14), f1(Str::lit(" }")))))
        }
        Adt3::C15 => {
            Ok(f1(Str::lit("None")))
        }
        Adt3::C16(r546) => {
            let v15 = (*r546).clone();
            Ok(f4(f1(Str::lit("Some { value: ")), f4(f54(v15)?, f1(Str::lit(" }")))))
        }
        Adt3::C17(r547) => {
            let v16 = (*r547).clone();
            Ok(f4(f1(Str::lit("Ok { value: ")), f4(f54(v16)?, f1(Str::lit(" }")))))
        }
        Adt3::C18(r548) => {
            let v17 = (*r548).clone();
            Ok(f4(f1(Str::lit("Error { value: ")), f4(f54(v17)?, f1(Str::lit(" }")))))
        }
        Adt3::C19(r549) => {
            let v18 = (*r549).clone();
            Ok(f4(f1(Str::lit("List { items: [")), f4(f54(v18)?, f1(Str::lit("] }")))))
        }
        Adt3::C20(r550, r551) => {
            let v19 = (*r550).clone();
            let v20 = (*r551).clone();
            Ok(f4(f1(Str::lit("Pair { left: ")), f4(f54(v19)?, f4(f1(Str::lit(", right: ")), f4(f54(v20)?, f1(Str::lit(" }")))))))
        }
        Adt3::C21(v21, r552) => {
            let v22 = (*r552).clone();
            Ok(f4(f1(Str::lit("Adt { constructor: ")), f4(f7(v21)?, f4(f1(Str::lit(", fields: [")), f4(f54(v22)?, f1(Str::lit("] }")))))))
        }
        Adt3::C22(v23, r553) => {
            let v24 = (*r553).clone();
            Ok(f4(f1(Str::lit("Closure { function: ")), f4(f7(v23)?, f4(f1(Str::lit(", captures: [")), f4(f54(v24)?, f1(Str::lit("] }")))))))
        }
        Adt3::C23 => {
            Ok(f1(Str::lit("")))
        }
        Adt3::C24(r554, r555) => {
            let v25 = (*r554).clone();
            let v26 = (*r555).clone();
            let v27: Bytes = f54(v25)?;
            let v28: Bytes = f54(v26.clone())?;
            if f75(v26) {
                Ok(v27)
            } else {
                Ok(f4(v27, f4(f1(Str::lit(", ")), v28)))
            }
        }
    }
}

pub fn f55(v0: Bytes) -> R<Result<(), Bytes>> {
    let m556 = f76(v0.clone())?;
    match m556 {
        Ok(v1) => {
            let v2: bool = f77(match v1.clone() {
                Adt9::C0(_, h557) => {
                    h557
                }
            }, f1(Str::lit("18446744073709551615")));
            if match v1 {
                Adt9::C0(h558, _) => {
                    h558
                }
            } {
                Ok(Err::<(), Bytes>(f4(f1(Str::lit("nat literal ")), f4(v0, f1(Str::lit(" is outside the 64-bit realization"))))))
            } else {
                if v2 {
                    Ok(Ok::<(), Bytes>(()))
                } else {
                    Ok(Err::<(), Bytes>(f4(f1(Str::lit("nat literal ")), f4(v0, f1(Str::lit(" is outside the 64-bit realization"))))))
                }
            }
        }
        Err(v3) => {
            Ok(Err::<(), Bytes>(v3))
        }
    }
}

pub fn f56(v0: Bytes, v1: Adt1) -> R<Result<(), Bytes>> {
    let m559 = f76(v0.clone())?;
    match m559 {
        Ok(v2) => {
            let v3: Adt9 = f78(v1.clone());
            let v4: Adt9 = f79(v1.clone());
            let v5: bool = f80(v3, v2.clone());
            let v6: bool = f80(v2, v4);
            if v5 {
                if v6 {
                    Ok(Ok::<(), Bytes>(()))
                } else {
                    Ok(Err::<(), Bytes>(f4(v0, f4(f1(Str::lit(" is outside ")), f81(v1)))))
                }
            } else {
                Ok(Err::<(), Bytes>(f4(v0, f4(f1(Str::lit(" is outside ")), f81(v1)))))
            }
        }
        Err(v7) => {
            Ok(Err::<(), Bytes>(v7))
        }
    }
}

pub fn f57(v0: Adt1) -> u64 {
    let m560 = v0;
    match m560 {
        Adt1::C0 => {
            0u64
        }
        Adt1::C1 => {
            1u64
        }
        Adt1::C2 => {
            2u64
        }
        Adt1::C3 => {
            3u64
        }
        Adt1::C4 => {
            4u64
        }
        Adt1::C5 => {
            5u64
        }
        Adt1::C6 => {
            6u64
        }
        Adt1::C7 => {
            7u64
        }
    }
}

pub fn f58(v0: Adt0) -> Adt1 {
    let m561 = v0;
    match m561 {
        Adt0::C0 => {
            Adt1::C0
        }
        Adt0::C1 => {
            Adt1::C0
        }
        Adt0::C2 => {
            Adt1::C0
        }
        Adt0::C3 => {
            Adt1::C0
        }
        Adt0::C4(v1) => {
            v1
        }
        Adt0::C5 => {
            Adt1::C0
        }
        Adt0::C6 => {
            Adt1::C0
        }
        Adt0::C7 => {
            Adt1::C0
        }
        Adt0::C8(_) => {
            Adt1::C0
        }
        Adt0::C9(_, _) => {
            Adt1::C0
        }
        Adt0::C10(_) => {
            Adt1::C0
        }
        Adt0::C11(_, _) => {
            Adt1::C0
        }
        Adt0::C12(_) => {
            Adt1::C0
        }
        Adt0::C13(_, _) => {
            Adt1::C0
        }
        Adt0::C14 => {
            Adt1::C0
        }
        Adt0::C15(_, _) => {
            Adt1::C0
        }
    }
}

pub fn f59(v0: Bytes) -> R<Result<(), Bytes>> {
    let a566 = {
        let a563 = {
            let a562 = v0.clone();
            length_bytes(a562)
        };
        let a564 = 2u64;
        let a565 = 0u64;
        nat_rem(a563, a564, a565)
    };
    let a567 = 0u64;
    if nat_eq(a566, a567) {
        if f82(v0.clone())? {
            Ok(Ok::<(), Bytes>(()))
        } else {
            Ok(Err::<(), Bytes>(f4(f1(Str::lit("byte literal `")), f4(v0, f1(Str::lit("` is not even-length lowercase hexadecimal"))))))
        }
    } else {
        Ok(Err::<(), Bytes>(f4(f1(Str::lit("byte literal `")), f4(v0, f1(Str::lit("` is not even-length lowercase hexadecimal"))))))
    }
}

pub fn f60(v0: Adt7, v1: u64, v2: u64) -> R<Result<Adt0, Bytes>> {
    let m568 = f26(v0, v1);
    match m568 {
        None => {
            Ok(Err::<Adt0, Bytes>(f27(v1)?))
        }
        Some(v3) => {
            let m569 = f30(v3, v2);
            match m569 {
                None => {
                    Ok(Err::<Adt0, Bytes>(f4(f1(Str::lit("ADT ")), f4(f7(v1)?, f4(f1(Str::lit(" has no constructor ")), f7(v2)?)))))
                }
                Some(v4) => {
                    Ok(Ok::<Adt0, Bytes>(v4))
                }
            }
        }
    }
}

pub fn f61(v0: Adt3) -> R<u64> {
    let m570 = v0;
    match m570 {
        Adt3::C0 => {
            Ok(0u64)
        }
        Adt3::C1(_) => {
            Ok(0u64)
        }
        Adt3::C2(_) => {
            Ok(0u64)
        }
        Adt3::C3(_) => {
            Ok(0u64)
        }
        Adt3::C4(_) => {
            Ok(0u64)
        }
        Adt3::C5(_) => {
            Ok(0u64)
        }
        Adt3::C6(_) => {
            Ok(0u64)
        }
        Adt3::C7(_) => {
            Ok(0u64)
        }
        Adt3::C8(_) => {
            Ok(0u64)
        }
        Adt3::C9(_) => {
            Ok(0u64)
        }
        Adt3::C10(_) => {
            Ok(0u64)
        }
        Adt3::C11(_) => {
            Ok(0u64)
        }
        Adt3::C12(_) => {
            Ok(0u64)
        }
        Adt3::C13(_) => {
            Ok(0u64)
        }
        Adt3::C14(_) => {
            Ok(0u64)
        }
        Adt3::C15 => {
            Ok(0u64)
        }
        Adt3::C16(_) => {
            Ok(0u64)
        }
        Adt3::C17(_) => {
            Ok(0u64)
        }
        Adt3::C18(_) => {
            Ok(0u64)
        }
        Adt3::C19(_) => {
            Ok(0u64)
        }
        Adt3::C20(_, _) => {
            Ok(0u64)
        }
        Adt3::C21(_, _) => {
            Ok(0u64)
        }
        Adt3::C22(_, _) => {
            Ok(0u64)
        }
        Adt3::C23 => {
            Ok(0u64)
        }
        Adt3::C24(_, r573) => {
            let v26 = (*r573).clone();
            let v27: u64 = f61(v26)?;
            let a571 = 1u64;
            let a572 = v27;
            nat_add(a571, a572)
        }
    }
}

pub fn f62(v0: Adt0, v1: u64) -> R<List<Adt5>> {
    let m574 = v0;
    match m574 {
        Adt0::C0 => {
            Ok(List::<Adt5>::nil())
        }
        Adt0::C1 => {
            Ok(List::<Adt5>::nil())
        }
        Adt0::C2 => {
            Ok(List::<Adt5>::nil())
        }
        Adt0::C3 => {
            Ok(List::<Adt5>::nil())
        }
        Adt0::C4(_) => {
            Ok(List::<Adt5>::nil())
        }
        Adt0::C5 => {
            Ok(List::<Adt5>::nil())
        }
        Adt0::C6 => {
            Ok(List::<Adt5>::nil())
        }
        Adt0::C7 => {
            Ok(List::<Adt5>::nil())
        }
        Adt0::C8(_) => {
            Ok(List::<Adt5>::nil())
        }
        Adt0::C9(_, _) => {
            Ok(List::<Adt5>::nil())
        }
        Adt0::C10(_) => {
            Ok(List::<Adt5>::nil())
        }
        Adt0::C11(_, _) => {
            Ok(List::<Adt5>::nil())
        }
        Adt0::C12(_) => {
            Ok(List::<Adt5>::nil())
        }
        Adt0::C13(_, _) => {
            Ok(List::<Adt5>::nil())
        }
        Adt0::C14 => {
            Ok(List::<Adt5>::nil())
        }
        Adt0::C15(_, r577) => {
            let v13 = (*r577).clone();
            Ok(f51(Adt5::C15(v1), f62(v13, {
                let a575 = v1;
                let a576 = 1u64;
                nat_add(a575, a576)?
            })?))
        }
    }
}

pub fn f63(v0: Adt0) -> Option<Adt0> {
    let m578 = v0;
    match m578 {
        Adt0::C0 => {
            None::<Adt0>
        }
        Adt0::C1 => {
            None::<Adt0>
        }
        Adt0::C2 => {
            None::<Adt0>
        }
        Adt0::C3 => {
            None::<Adt0>
        }
        Adt0::C4(_) => {
            None::<Adt0>
        }
        Adt0::C5 => {
            None::<Adt0>
        }
        Adt0::C6 => {
            None::<Adt0>
        }
        Adt0::C7 => {
            None::<Adt0>
        }
        Adt0::C8(r579) => {
            let v2 = (*r579).clone();
            Some(v2)
        }
        Adt0::C9(r580, _) => {
            let v3 = (*r580).clone();
            Some(v3)
        }
        Adt0::C10(r581) => {
            let v5 = (*r581).clone();
            Some(v5)
        }
        Adt0::C11(r582, _) => {
            let v6 = (*r582).clone();
            Some(v6)
        }
        Adt0::C12(_) => {
            None::<Adt0>
        }
        Adt0::C13(r583, _) => {
            let v9 = (*r583).clone();
            Some(v9)
        }
        Adt0::C14 => {
            None::<Adt0>
        }
        Adt0::C15(r584, _) => {
            let v11 = (*r584).clone();
            Some(v11)
        }
    }
}

pub fn f64(v0: Adt0) -> Option<Adt0> {
    let m585 = v0;
    match m585 {
        Adt0::C0 => {
            None::<Adt0>
        }
        Adt0::C1 => {
            None::<Adt0>
        }
        Adt0::C2 => {
            None::<Adt0>
        }
        Adt0::C3 => {
            None::<Adt0>
        }
        Adt0::C4(_) => {
            None::<Adt0>
        }
        Adt0::C5 => {
            None::<Adt0>
        }
        Adt0::C6 => {
            None::<Adt0>
        }
        Adt0::C7 => {
            None::<Adt0>
        }
        Adt0::C8(_) => {
            None::<Adt0>
        }
        Adt0::C9(_, r586) => {
            let v4 = (*r586).clone();
            Some(v4)
        }
        Adt0::C10(_) => {
            None::<Adt0>
        }
        Adt0::C11(_, r587) => {
            let v7 = (*r587).clone();
            Some(v7)
        }
        Adt0::C12(_) => {
            None::<Adt0>
        }
        Adt0::C13(_, r588) => {
            let v10 = (*r588).clone();
            Some(v10)
        }
        Adt0::C14 => {
            None::<Adt0>
        }
        Adt0::C15(_, r589) => {
            let v12 = (*r589).clone();
            Some(v12)
        }
    }
}

pub fn f65(v0: Adt6, v1: Adt0, v2: Adt0, v3: Adt0) -> R<Result<Adt0, Bytes>> {
    let v4: Result<Adt0, Bytes> = Ok::<Adt0, Bytes>(v3);
    let v5: Result<Adt0, Bytes> = Err::<Adt0, Bytes>(f4(f1(Str::lit("primitive ")), f4(f83(v0)?, f4(f1(Str::lit(" takes ")), f4(f84(v1.clone())?, f4(f1(Str::lit(", received ")), f84(v2.clone())?))))));
    if f52(v2, v1) {
        Ok(v4)
    } else {
        Ok(v5)
    }
}

pub fn f66(v0: Adt6, v1: Adt0) -> R<Result<Adt0, Bytes>> {
    Ok(Err::<Adt0, Bytes>(f4(f1(Str::lit("primitive ")), f4(f83(v0)?, f4(f1(Str::lit(" does not apply to ")), f84(v1)?)))))
}

pub fn f67(v0: Adt1) -> bool {
    let m590 = v0;
    match m590 {
        Adt1::C0 => {
            false
        }
        Adt1::C1 => {
            false
        }
        Adt1::C2 => {
            false
        }
        Adt1::C3 => {
            false
        }
        Adt1::C4 => {
            true
        }
        Adt1::C5 => {
            true
        }
        Adt1::C6 => {
            true
        }
        Adt1::C7 => {
            true
        }
    }
}

pub fn f68(v0: Adt0) -> bool {
    let m591 = v0;
    match m591 {
        Adt0::C0 => {
            false
        }
        Adt0::C1 => {
            true
        }
        Adt0::C2 => {
            true
        }
        Adt0::C3 => {
            true
        }
        Adt0::C4(_) => {
            true
        }
        Adt0::C5 => {
            true
        }
        Adt0::C6 => {
            false
        }
        Adt0::C7 => {
            false
        }
        Adt0::C8(_) => {
            false
        }
        Adt0::C9(_, _) => {
            false
        }
        Adt0::C10(_) => {
            false
        }
        Adt0::C11(r592, r593) => {
            let v2 = (*r592).clone();
            let v3 = (*r593).clone();
            let v4: bool = f68(v2);
            let v5: bool = f68(v3);
            if v4 {
                v5
            } else {
                false
            }
        }
        Adt0::C12(_) => {
            false
        }
        Adt0::C13(_, _) => {
            false
        }
        Adt0::C14 => {
            false
        }
        Adt0::C15(_, _) => {
            false
        }
    }
}

pub fn f69(v0: Adt1) -> Bytes {
    let m594 = v0;
    match m594 {
        Adt1::C0 => {
            f1(Str::lit("U8"))
        }
        Adt1::C1 => {
            f1(Str::lit("U16"))
        }
        Adt1::C2 => {
            f1(Str::lit("U32"))
        }
        Adt1::C3 => {
            f1(Str::lit("U64"))
        }
        Adt1::C4 => {
            f1(Str::lit("I8"))
        }
        Adt1::C5 => {
            f1(Str::lit("I16"))
        }
        Adt1::C6 => {
            f1(Str::lit("I32"))
        }
        Adt1::C7 => {
            f1(Str::lit("I64"))
        }
    }
}

pub fn f70(v0: Adt5, v1: Adt5) -> bool {
    let a595 = f85(v0.clone());
    let a596 = f85(v1.clone());
    if nat_eq(a595, a596) {
        let a597 = f86(v0);
        let a598 = f86(v1);
        nat_eq(a597, a598)
    } else {
        false
    }
}

pub fn f71(v0: Bytes, v1: u64, v2: u8) -> bool {
    let m599 = f87(v0, v1);
    match m599 {
        None => {
            false
        }
        Some(v3) => {
            let a600 = v3;
            let a601 = v2;
            equal(a600, a601)
        }
    }
}

pub fn f72(v0: bool) -> Bytes {
    if v0 {
        f1(Str::lit("true"))
    } else {
        f1(Str::lit("false"))
    }
}

pub fn f73(v0: Bytes) -> R<Bytes> {
    Ok(f4(f1(Str::lit("\"")), f4({
        let (_, h602) = {
            let (h603, _) = f88(Fn2::F89(v0.clone()), {
                let a604 = v0;
                length_bytes(a604)
            }, (0u64, Bytes::lit(&[])))?;
            h603
        };
        h602
    }, f1(Str::lit("\"")))))
}

pub fn f74(v0: Adt4) -> Bytes {
    let m605 = v0;
    match m605 {
        Adt4::C0 => {
            f1(Str::lit("Lt"))
        }
        Adt4::C1 => {
            f1(Str::lit("Eq"))
        }
        Adt4::C2 => {
            f1(Str::lit("Gt"))
        }
    }
}

pub fn f75(v0: Adt3) -> bool {
    let m606 = v0;
    match m606 {
        Adt3::C0 => {
            false
        }
        Adt3::C1(_) => {
            false
        }
        Adt3::C2(_) => {
            false
        }
        Adt3::C3(_) => {
            false
        }
        Adt3::C4(_) => {
            false
        }
        Adt3::C5(_) => {
            false
        }
        Adt3::C6(_) => {
            false
        }
        Adt3::C7(_) => {
            false
        }
        Adt3::C8(_) => {
            false
        }
        Adt3::C9(_) => {
            false
        }
        Adt3::C10(_) => {
            false
        }
        Adt3::C11(_) => {
            false
        }
        Adt3::C12(_) => {
            false
        }
        Adt3::C13(_) => {
            false
        }
        Adt3::C14(_) => {
            false
        }
        Adt3::C15 => {
            false
        }
        Adt3::C16(_) => {
            false
        }
        Adt3::C17(_) => {
            false
        }
        Adt3::C18(_) => {
            false
        }
        Adt3::C19(_) => {
            false
        }
        Adt3::C20(_, _) => {
            false
        }
        Adt3::C21(_, _) => {
            false
        }
        Adt3::C22(_, _) => {
            false
        }
        Adt3::C23 => {
            true
        }
        Adt3::C24(_, _) => {
            false
        }
    }
}

pub fn f76(v0: Bytes) -> R<Result<Adt9, Bytes>> {
    let v1: bool = f71(v0.clone(), 0u64, 45u8);
    let v3: Bytes = {
        let v2: Bytes = f25(v0.clone(), 1u64, {
            let a608 = {
                let a607 = v0.clone();
                length_bytes(a607)
            };
            let a609 = 1u64;
            nat_sub(a608, a609)
        });
        if v1 {
            v2
        } else {
            v0.clone()
        }
    };
    let v4: bool = f90(v3.clone(), v0.clone())?;
    let v5: bool = f77(v3.clone(), f1(Str::lit("170141183460469231731687303715884105727")));
    let v6: bool = f77(v3.clone(), f1(Str::lit("170141183460469231731687303715884105728")));
    if v4 {
        if if v1 {
            v6
        } else {
            v5
        } {
            Ok(Ok::<Adt9, Bytes>(Adt9::C0(v1, v3)))
        } else {
            Ok(Err::<Adt9, Bytes>(f4(f5(v0), f1(Str::lit(" is outside every realized integer")))))
        }
    } else {
        Ok(Err::<Adt9, Bytes>(f4(f5(v0), f1(Str::lit(" is not a canonical decimal")))))
    }
}

pub fn f77(v0: Bytes, v1: Bytes) -> bool {
    let v2: bool = {
        let a612 = {
            let a610 = v0.clone();
            length_bytes(a610)
        };
        let a613 = {
            let a611 = v1.clone();
            length_bytes(a611)
        };
        nat_lt(a612, a613)
    };
    let v3: bool = {
        let a616 = {
            let a614 = v0.clone();
            length_bytes(a614)
        };
        let a617 = {
            let a615 = v1.clone();
            length_bytes(a615)
        };
        nat_eq(a616, a617)
    };
    let v4: bool = f91(v1, v0);
    if v2 {
        true
    } else {
        if v3 {
            let a618 = v4;
            bool_not(a618)
        } else {
            false
        }
    }
}

pub fn f78(v0: Adt1) -> Adt9 {
    let m619 = v0;
    match m619 {
        Adt1::C0 => {
            f92(false, Str::lit("0"))
        }
        Adt1::C1 => {
            f92(false, Str::lit("0"))
        }
        Adt1::C2 => {
            f92(false, Str::lit("0"))
        }
        Adt1::C3 => {
            f92(false, Str::lit("0"))
        }
        Adt1::C4 => {
            f92(true, Str::lit("128"))
        }
        Adt1::C5 => {
            f92(true, Str::lit("32768"))
        }
        Adt1::C6 => {
            f92(true, Str::lit("2147483648"))
        }
        Adt1::C7 => {
            f92(true, Str::lit("9223372036854775808"))
        }
    }
}

pub fn f79(v0: Adt1) -> Adt9 {
    let m620 = v0;
    match m620 {
        Adt1::C0 => {
            f92(false, Str::lit("255"))
        }
        Adt1::C1 => {
            f92(false, Str::lit("65535"))
        }
        Adt1::C2 => {
            f92(false, Str::lit("4294967295"))
        }
        Adt1::C3 => {
            f92(false, Str::lit("18446744073709551615"))
        }
        Adt1::C4 => {
            f92(false, Str::lit("127"))
        }
        Adt1::C5 => {
            f92(false, Str::lit("32767"))
        }
        Adt1::C6 => {
            f92(false, Str::lit("2147483647"))
        }
        Adt1::C7 => {
            f92(false, Str::lit("9223372036854775807"))
        }
    }
}

pub fn f80(v0: Adt9, v1: Adt9) -> bool {
    let v2: bool = f77(match v1.clone() {
        Adt9::C0(_, h621) => {
            h621
        }
    }, match v0.clone() {
        Adt9::C0(_, h622) => {
            h622
        }
    });
    let v3: bool = f77(match v0.clone() {
        Adt9::C0(_, h623) => {
            h623
        }
    }, match v1.clone() {
        Adt9::C0(_, h624) => {
            h624
        }
    });
    if match v0 {
        Adt9::C0(h625, _) => {
            h625
        }
    } {
        if match v1 {
            Adt9::C0(h626, _) => {
                h626
            }
        } {
            v2
        } else {
            true
        }
    } else {
        if match v1 {
            Adt9::C0(h627, _) => {
                h627
            }
        } {
            false
        } else {
            v3
        }
    }
}

pub fn f81(v0: Adt1) -> Bytes {
    let m628 = v0;
    match m628 {
        Adt1::C0 => {
            f1(Str::lit("u8"))
        }
        Adt1::C1 => {
            f1(Str::lit("u16"))
        }
        Adt1::C2 => {
            f1(Str::lit("u32"))
        }
        Adt1::C3 => {
            f1(Str::lit("u64"))
        }
        Adt1::C4 => {
            f1(Str::lit("i8"))
        }
        Adt1::C5 => {
            f1(Str::lit("i16"))
        }
        Adt1::C6 => {
            f1(Str::lit("i32"))
        }
        Adt1::C7 => {
            f1(Str::lit("i64"))
        }
    }
}

pub fn f82(v0: Bytes) -> R<bool> {
    let a632 = {
        let (h629, _) = f23(Fn0::F93(v0.clone()), {
            let a630 = v0.clone();
            length_bytes(a630)
        }, 0u64)?;
        h629
    };
    let a633 = {
        let a631 = v0;
        length_bytes(a631)
    };
    Ok(nat_eq(a632, a633))
}

pub fn f83(v0: Adt6) -> R<Bytes> {
    let m634 = v0;
    match m634 {
        Adt6::C0 => {
            Ok(f1(Str::lit("NatAdd")))
        }
        Adt6::C1 => {
            Ok(f1(Str::lit("NatSub")))
        }
        Adt6::C2 => {
            Ok(f1(Str::lit("NatMul")))
        }
        Adt6::C3 => {
            Ok(f1(Str::lit("NatQuot")))
        }
        Adt6::C4 => {
            Ok(f1(Str::lit("NatRem")))
        }
        Adt6::C5 => {
            Ok(f1(Str::lit("NatEq")))
        }
        Adt6::C6 => {
            Ok(f1(Str::lit("NatLe")))
        }
        Adt6::C7 => {
            Ok(f1(Str::lit("NatLt")))
        }
        Adt6::C8 => {
            Ok(f1(Str::lit("IntAdd")))
        }
        Adt6::C9 => {
            Ok(f1(Str::lit("IntSub")))
        }
        Adt6::C10 => {
            Ok(f1(Str::lit("IntMul")))
        }
        Adt6::C11 => {
            Ok(f1(Str::lit("IntNeg")))
        }
        Adt6::C12 => {
            Ok(f1(Str::lit("IntQuot")))
        }
        Adt6::C13 => {
            Ok(f1(Str::lit("IntRem")))
        }
        Adt6::C14 => {
            Ok(f1(Str::lit("CheckedAdd")))
        }
        Adt6::C15 => {
            Ok(f1(Str::lit("CheckedSub")))
        }
        Adt6::C16 => {
            Ok(f1(Str::lit("CheckedMul")))
        }
        Adt6::C17 => {
            Ok(f1(Str::lit("CheckedNeg")))
        }
        Adt6::C18 => {
            Ok(f1(Str::lit("CheckedQuot")))
        }
        Adt6::C19 => {
            Ok(f1(Str::lit("BitAnd")))
        }
        Adt6::C20 => {
            Ok(f1(Str::lit("BitOr")))
        }
        Adt6::C21 => {
            Ok(f1(Str::lit("BitXor")))
        }
        Adt6::C22 => {
            Ok(f1(Str::lit("BitNot")))
        }
        Adt6::C23 => {
            Ok(f1(Str::lit("ShiftLeft")))
        }
        Adt6::C24 => {
            Ok(f1(Str::lit("ShiftRight")))
        }
        Adt6::C25 => {
            Ok(f1(Str::lit("Equal")))
        }
        Adt6::C26 => {
            Ok(f1(Str::lit("BoolNot")))
        }
        Adt6::C27 => {
            Ok(f1(Str::lit("BoolAnd")))
        }
        Adt6::C28 => {
            Ok(f1(Str::lit("BoolOr")))
        }
        Adt6::C29 => {
            Ok(f1(Str::lit("Append")))
        }
        Adt6::C30 => {
            Ok(f1(Str::lit("Length")))
        }
        Adt6::C31 => {
            Ok(f1(Str::lit("Index")))
        }
        Adt6::C32 => {
            Ok(f1(Str::lit("Slice")))
        }
        Adt6::C33 => {
            Ok(f1(Str::lit("Utf8Encode")))
        }
        Adt6::C34 => {
            Ok(f1(Str::lit("Utf8Decode")))
        }
        Adt6::C35 => {
            Ok(f1(Str::lit("CompareBytes")))
        }
        Adt6::C36 => {
            Ok(f1(Str::lit("SplitExact")))
        }
        Adt6::C37 => {
            Ok(f1(Str::lit("Join")))
        }
        Adt6::C38 => {
            Ok(f1(Str::lit("FormatDecimal")))
        }
        Adt6::C39 => {
            Ok(f1(Str::lit("Compare")))
        }
        Adt6::C40(v1) => {
            Ok(f4(f1(Str::lit("Convert { target: ")), f4(f69(v1), f1(Str::lit(" }")))))
        }
        Adt6::C41(v2) => {
            Ok(f4(f1(Str::lit("ParseDecimal { target: ")), f4(f47(v2)?, f1(Str::lit(" }")))))
        }
    }
}

pub fn f84(v0: Adt0) -> R<Bytes> {
    Ok(f4(f1(Str::lit("[")), f4(f47(v0)?, f1(Str::lit("]")))))
}

pub fn f85(v0: Adt5) -> u64 {
    let m635 = v0;
    match m635 {
        Adt5::C0 => {
            0u64
        }
        Adt5::C1 => {
            1u64
        }
        Adt5::C2 => {
            2u64
        }
        Adt5::C3 => {
            3u64
        }
        Adt5::C4 => {
            4u64
        }
        Adt5::C5 => {
            5u64
        }
        Adt5::C6 => {
            6u64
        }
        Adt5::C7 => {
            7u64
        }
        Adt5::C8 => {
            8u64
        }
        Adt5::C9 => {
            9u64
        }
        Adt5::C10 => {
            10u64
        }
        Adt5::C11 => {
            11u64
        }
        Adt5::C12 => {
            12u64
        }
        Adt5::C13 => {
            13u64
        }
        Adt5::C14 => {
            14u64
        }
        Adt5::C15(_) => {
            15u64
        }
    }
}

pub fn f86(v0: Adt5) -> u64 {
    let m636 = v0;
    match m636 {
        Adt5::C0 => {
            0u64
        }
        Adt5::C1 => {
            0u64
        }
        Adt5::C2 => {
            0u64
        }
        Adt5::C3 => {
            0u64
        }
        Adt5::C4 => {
            0u64
        }
        Adt5::C5 => {
            0u64
        }
        Adt5::C6 => {
            0u64
        }
        Adt5::C7 => {
            0u64
        }
        Adt5::C8 => {
            0u64
        }
        Adt5::C9 => {
            0u64
        }
        Adt5::C10 => {
            0u64
        }
        Adt5::C11 => {
            0u64
        }
        Adt5::C12 => {
            0u64
        }
        Adt5::C13 => {
            0u64
        }
        Adt5::C14 => {
            0u64
        }
        Adt5::C15(v1) => {
            v1
        }
    }
}

pub fn f87(v0: Bytes, v1: u64) -> Option<u8> {
    let a637 = v0;
    let a638 = v1;
    index_bytes(a637, a638)
}

pub fn f88(v0: Fn2, p643: u64, p644: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m642 = (p643, p644);
    {
        let mut m642 = m642;
        loop {
            let step = {
                let (v1, v2) = m642;
                let m639 = v1;
                if m639 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m639 - 1;
                    let m641 = {
                        let c640: Fn2 = v0.clone();
                        c640.apply(v2.clone())?
                    };
                    match m641 {
                        None => {
                            Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m642 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f89(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f94(v0, v1)
}

pub fn f90(v0: Bytes, v1: Bytes) -> R<bool> {
    let a646 = 0u64;
    let a647 = {
        let a645 = v0.clone();
        length_bytes(a645)
    };
    if nat_lt(a646, a647) {
        if f95(v0.clone())? {
            let a648 = v0.clone();
            let a649 = f1(Str::lit("0"));
            if equal(a648, a649) {
                let a652 = {
                    let a650 = v1;
                    let a651 = f1(Str::lit("-0"));
                    equal(a650, a651)
                };
                Ok(bool_not(a652))
            } else {
                let a653 = f71(v0, 0u64, 48u8);
                Ok(bool_not(a653))
            }
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f91(v0: Bytes, v1: Bytes) -> bool {
    let a658 = {
        let a654 = v0;
        let a655 = v1;
        compare_bytes(a654, a655)
    };
    let a659 = {
        let a656 = Bytes::lit(&[]);
        let a657 = Bytes::lit(&[0x01]);
        compare_bytes(a656, a657)
    };
    equal(a658, a659)
}

pub fn f92(v0: bool, v1: Str) -> Adt9 {
    Adt9::C0(v0, f1(v1))
}

pub fn f93(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f96(v0, v1)
}

pub fn f94(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let v2: u64 = {
        let (h660, _) = v1;
        h660
    };
    let v3: u64 = f97(v0.clone(), v2)?;
    let a661 = v3;
    let a662 = 256u64;
    if nat_eq(a661, a662) {
        Ok(None::<(u64, Bytes)>)
    } else {
        let v4: (u64, Bytes) = f98(v0, v2, v3)?;
        Ok(Some(({
            let a664 = v2;
            let a665 = {
                let (h663, _) = v4;
                h663
            };
            nat_add(a664, a665)?
        }, f4({
            let (_, h666) = v1;
            h666
        }, {
            let (_, h667) = v4;
            h667
        }))))
    }
}

pub fn f95(v0: Bytes) -> R<bool> {
    let a671 = {
        let (h668, _) = f23(Fn0::F99(v0.clone()), {
            let a669 = v0.clone();
            length_bytes(a669)
        }, 0u64)?;
        h668
    };
    let a672 = {
        let a670 = v0;
        length_bytes(a670)
    };
    Ok(nat_eq(a671, a672))
}

pub fn f96(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f100(v0, v1)? {
        Ok(Some({
            let a673 = v1;
            let a674 = 1u64;
            nat_add(a673, a674)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f97(v0: Bytes, v1: u64) -> R<u64> {
    let m675 = f87(v0, v1);
    match m675 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a720 = {
                let a696 = {
                    let a684 = {
                        let a678 = {
                            let a676 = v3;
                            let a677 = 128u8;
                            fixed_u8::bit_and(a676, a677)
                        };
                        let a679 = 128u8;
                        if equal(a678, a679) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a685 = {
                        let a682 = {
                            let a680 = v3;
                            let a681 = 64u8;
                            fixed_u8::bit_and(a680, a681)
                        };
                        let a683 = 64u8;
                        if equal(a682, a683) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a684, a685)?
                };
                let a697 = {
                    let a694 = {
                        let a688 = {
                            let a686 = v3;
                            let a687 = 32u8;
                            fixed_u8::bit_and(a686, a687)
                        };
                        let a689 = 32u8;
                        if equal(a688, a689) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a695 = {
                        let a692 = {
                            let a690 = v3;
                            let a691 = 16u8;
                            fixed_u8::bit_and(a690, a691)
                        };
                        let a693 = 16u8;
                        if equal(a692, a693) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a694, a695)?
                };
                nat_add(a696, a697)?
            };
            let a721 = {
                let a718 = {
                    let a706 = {
                        let a700 = {
                            let a698 = v3;
                            let a699 = 8u8;
                            fixed_u8::bit_and(a698, a699)
                        };
                        let a701 = 8u8;
                        if equal(a700, a701) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a707 = {
                        let a704 = {
                            let a702 = v3;
                            let a703 = 4u8;
                            fixed_u8::bit_and(a702, a703)
                        };
                        let a705 = 4u8;
                        if equal(a704, a705) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a706, a707)?
                };
                let a719 = {
                    let a716 = {
                        let a710 = {
                            let a708 = v3;
                            let a709 = 2u8;
                            fixed_u8::bit_and(a708, a709)
                        };
                        let a711 = 2u8;
                        if equal(a710, a711) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a717 = {
                        let a714 = {
                            let a712 = v3;
                            let a713 = 1u8;
                            fixed_u8::bit_and(a712, a713)
                        };
                        let a715 = 1u8;
                        if equal(a714, a715) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a716, a717)?
                };
                nat_add(a718, a719)?
            };
            nat_add(a720, a721)
        }
    }
}

pub fn f98(v0: Bytes, v1: u64, v2: u64) -> R<(u64, Bytes)> {
    let v3: u64 = f97(v0.clone(), {
        let a722 = v1;
        let a723 = 1u64;
        nat_add(a722, a723)?
    })?;
    let a724 = v2;
    let a725 = 34u64;
    if nat_eq(a724, a725) {
        Ok((1u64, f1(Str::lit("\\\""))))
    } else {
        let a726 = v2;
        let a727 = 92u64;
        if nat_eq(a726, a727) {
            Ok((1u64, f1(Str::lit("\\\\"))))
        } else {
            let a728 = v2;
            let a729 = 10u64;
            if nat_eq(a728, a729) {
                Ok((1u64, f1(Str::lit("\\n"))))
            } else {
                let a730 = v2;
                let a731 = 13u64;
                if nat_eq(a730, a731) {
                    Ok((1u64, f1(Str::lit("\\r"))))
                } else {
                    let a732 = v2;
                    let a733 = 9u64;
                    if nat_eq(a732, a733) {
                        Ok((1u64, f1(Str::lit("\\t"))))
                    } else {
                        let a734 = v2;
                        let a735 = 0u64;
                        if nat_eq(a734, a735) {
                            Ok((1u64, f1(Str::lit("\\0"))))
                        } else {
                            let a736 = v2;
                            let a737 = 32u64;
                            if nat_lt(a736, a737) {
                                Ok((1u64, f101(v2)))
                            } else {
                                let a738 = v2;
                                let a739 = 127u64;
                                if nat_eq(a738, a739) {
                                    Ok((1u64, f101(v2)))
                                } else {
                                    let a740 = v2;
                                    let a741 = 194u64;
                                    if nat_eq(a740, a741) {
                                        let a742 = 128u64;
                                        let a743 = v3;
                                        if nat_le(a742, a743) {
                                            let a744 = v3;
                                            let a745 = 159u64;
                                            if nat_le(a744, a745) {
                                                Ok((2u64, f101(v3)))
                                            } else {
                                                Ok((1u64, f25(v0, v1, 1u64)))
                                            }
                                        } else {
                                            Ok((1u64, f25(v0, v1, 1u64)))
                                        }
                                    } else {
                                        Ok((1u64, f25(v0, v1, 1u64)))
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f99(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f102(v0, v1)
}

pub fn f100(v0: Bytes, v1: u64) -> R<bool> {
    let v2: u64 = f97(v0, v1)?;
    let a746 = 47u64;
    let a747 = v2;
    if nat_lt(a746, a747) {
        let a748 = v2;
        let a749 = 58u64;
        if nat_lt(a748, a749) {
            Ok(true)
        } else {
            let a750 = 96u64;
            let a751 = v2;
            if nat_lt(a750, a751) {
                let a752 = v2;
                let a753 = 103u64;
                Ok(nat_lt(a752, a753))
            } else {
                Ok(false)
            }
        }
    } else {
        Ok(false)
    }
}

pub fn f101(v0: u64) -> Bytes {
    f4(f1(Str::lit("\\u{")), f4(f103(v0), f1(Str::lit("}"))))
}

pub fn f102(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f104(v0, v1)? {
        Ok(Some({
            let a754 = v1;
            let a755 = 1u64;
            nat_add(a754, a755)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f103(v0: u64) -> Bytes {
    let v1: Bytes = f105(v0);
    let v2: Bytes = f4(f105({
        let a756 = v0;
        let a757 = 16u64;
        let a758 = 0u64;
        nat_quot(a756, a757, a758)
    }), f105({
        let a759 = v0;
        let a760 = 16u64;
        let a761 = 0u64;
        nat_rem(a759, a760, a761)
    }));
    let a762 = v0;
    let a763 = 16u64;
    if nat_lt(a762, a763) {
        v1
    } else {
        v2
    }
}

pub fn f104(v0: Bytes, v1: u64) -> R<bool> {
    let v2: u64 = f97(v0, v1)?;
    let a764 = 47u64;
    let a765 = v2;
    if nat_lt(a764, a765) {
        let a766 = v2;
        let a767 = 58u64;
        Ok(nat_lt(a766, a767))
    } else {
        Ok(false)
    }
}

pub fn f105(v0: u64) -> Bytes {
    f25(f1(Str::lit("0123456789abcdef")), v0, 1u64)
}
