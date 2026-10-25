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
        let a144 = 30u64;
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
                nat_eq(a150, a151)
            } {
                1u64
            } else {
                let a152 = v0;
                let a153 = 15u64;
                if nat_eq(a152, a153) {
                    3u64
                } else {
                    let a154 = v0;
                    let a155 = 16u64;
                    if nat_eq(a154, a155) {
                        3u64
                    } else {
                        let a156 = v0;
                        let a157 = 20u64;
                        if nat_eq(a156, a157) {
                            3u64
                        } else {
                            let a158 = v0;
                            let a159 = 25u64;
                            if if nat_eq(a158, a159) {
                                true
                            } else {
                                let a160 = v0;
                                let a161 = 29u64;
                                nat_eq(a160, a161)
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
    let a162 = v1;
    let a163 = 8u64;
    if nat_lt(a162, a163) {
        f8(v0, Str::lit(", "))
    } else {
        let a164 = v1;
        let a165 = 100u64;
        if nat_eq(a164, a165) {
            f8(v0, Str::lit(", "))
        } else {
            let a166 = v2;
            let a167 = 0u64;
            if nat_eq(a166, a167) {
                f8(v0, f46(v1))
            } else {
                f8(v0, f47(v1))
            }
        }
    }
}

pub fn f25(v0: Adt20, v1: Adt17) -> R<Adt20> {
    let m168 = v1;
    match m168 {
        Adt17::C0(v2) => {
            f8(v0, {
                let m169 = v2;
                match m169 {
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
        Adt17::C2(r173) => {
            let v4 = (*r173).clone();
            f8(f25(f8(v0, Str::lit("("))?, v4)?, Str::lit(")"))
        }
        Adt17::C3(v5, r174) => {
            let v6 = (*r174).clone();
            f8(f25(f8(v0, {
                let m170 = v5;
                match m170 {
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
        Adt17::C4(v7, v8, r175) => {
            let v9 = (*r175).clone();
            let v10: Adt20 = f11(f8(f11(f8(v0, Str::lit("Adt"))?, v7)?, Str::lit("::C"))?, v8)?;
            let m171 = v9.clone();
            match m171 {
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
        Adt17::C6(r176, r177) => {
            let v22 = (*r176).clone();
            let v23 = (*r177).clone();
            let m172 = v23.clone();
            match m172 {
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
    let m178 = v1;
    match m178 {
        Adt11::C0(v2) => {
            f48(v0, v2)
        }
        Adt11::C1(v3) => {
            f32(v0, v3)
        }
        Adt11::C2(v4, r182) => {
            let v5 = (*r182).clone();
            f8(f26(f8(v0, f49(v4.clone()))?, v5)?, f50(v4))
        }
        Adt11::C3(v6, r183, r184) => {
            let v7 = (*r183).clone();
            let v8 = (*r184).clone();
            let m179 = v6;
            match m179 {
                Adt14::C0 => {
                    f8(f26(f8(f26(f8(v0, Str::lit("Result<"))?, v7)?, Str::lit(", "))?, v8)?, Str::lit(">"))
                }
                Adt14::C1 => {
                    f8(f26(f8(f26(f8(v0, Str::lit("("))?, v7)?, Str::lit(", "))?, v8)?, Str::lit(")"))
                }
            }
        }
        Adt11::C4(v9, v10) => {
            let m180 = v9;
            match m180 {
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
        Adt11::C6(r185, r186) => {
            let v11 = (*r185).clone();
            let v12 = (*r186).clone();
            let m181 = v12.clone();
            match m181 {
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

pub fn f27(v0: Fn0, p191: u64, p192: List<Bytes>) -> (List<Bytes>, bool) {
    let m190 = (p191, p192);
    {
        let mut m190 = m190;
        loop {
            let step = {
                let (v1, v2) = m190;
                let m187 = v1;
                if m187 == 0 {
                    Err::<(u64, List<Bytes>), (List<Bytes>, bool)>((v2, false))
                } else {
                    let v3 = m187 - 1;
                    let m189 = {
                        let c188: Fn0 = v0.clone();
                        c188.apply(v2.clone())
                    };
                    match m189 {
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
                Ok(next) => { m190 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f28(v0: List<Bytes>) -> Option<List<Bytes>> {
    f52(v0)
}

pub fn f29(v0: Fn1, p196: List<Bytes>, p197: List<Bytes>) -> List<Bytes> {
    let m195 = (p196, p197);
    {
        let mut m195 = m195;
        loop {
            let step = {
                let (v1, v2) = m195;
                let m193 = v2;
                match m193.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c194: Fn1 = v0.clone();
                            c194.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m195 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f30(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f31(v0: Adt2) -> u64 {
    let m198 = v0;
    match m198 {
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
        let (_, h199) = {
            let (h200, _) = f44(Fn2::F54(v1.clone()), {
                let a201 = v1;
                length_bytes(a201)
            }, (0u64, f8(v0, Str::lit("\""))?))?;
            h200
        };
        h199
    }, Str::lit("\""))
}

pub fn f34(v0: Adt20, v1: Bytes) -> R<Adt20> {
    f8({
        let (_, h202) = {
            let (h203, _) = f44(Fn2::F55(v1.clone()), {
                let a204 = v1;
                length_bytes(a204)
            }, (0u64, f8(v0, Str::lit("Bytes::lit(&["))?))?;
            h203
        };
        h202
    }, Str::lit("])"))
}

pub fn f35(v0: Adt20, v1: Adt3) -> R<Adt20> {
    f8(v0, {
        let m205 = v1;
        match m205 {
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
        Adt20::C0(_, _, _, _, h206) => {
            h206
        }
    } {
        Ok(v0)
    } else {
        let v2: u64 = {
            let a209 = match v0.clone() {
                Adt20::C0(_, _, h207, _, _) => {
                    h207
                }
            };
            let a210 = {
                let a208 = v1.clone();
                length_bytes(a208)
            };
            nat_add(a209, a210)?
        };
        let a212 = v2;
        let a213 = match v0.clone() {
            Adt20::C0(_, _, _, h211, _) => {
                h211
            }
        };
        if nat_le(a212, a213) {
            Ok(f56(v0.clone(), {
                let a215 = match v0 {
                    Adt20::C0(_, h214, _, _, _) => {
                        h214
                    }
                };
                let a216 = v1;
                append_bytes(a215, a216)
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
    let a217 = v0;
    let a218 = 20u64;
    if nat_eq(a217, a218) {
        Str::lit("checked_add")
    } else {
        let a219 = v0;
        let a220 = 21u64;
        if nat_eq(a219, a220) {
            Str::lit("checked_sub")
        } else {
            let a221 = v0;
            let a222 = 22u64;
            if nat_eq(a221, a222) {
                Str::lit("checked_mul")
            } else {
                let a223 = v0;
                let a224 = 23u64;
                if nat_eq(a223, a224) {
                    Str::lit("checked_quot")
                } else {
                    let a225 = v0;
                    let a226 = 25u64;
                    if nat_eq(a225, a226) {
                        Str::lit("bit_and")
                    } else {
                        let a227 = v0;
                        let a228 = 26u64;
                        if nat_eq(a227, a228) {
                            Str::lit("bit_or")
                        } else {
                            let a229 = v0;
                            let a230 = 27u64;
                            if nat_eq(a229, a230) {
                                Str::lit("bit_xor")
                            } else {
                                let a231 = v0;
                                let a232 = 28u64;
                                if nat_eq(a231, a232) {
                                    Str::lit("bit_not")
                                } else {
                                    let a233 = v0;
                                    let a234 = 29u64;
                                    if nat_eq(a233, a234) {
                                        Str::lit("shift_left")
                                    } else {
                                        let a235 = v0;
                                        let a236 = 30u64;
                                        if nat_eq(a235, a236) {
                                            Str::lit("shift_right")
                                        } else {
                                            let a237 = v0;
                                            let a238 = 31u64;
                                            if nat_eq(a237, a238) {
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
    let a239 = v0;
    let a240 = 0u64;
    if nat_eq(a239, a240) {
        Str::lit("nat_add")
    } else {
        let a241 = v0;
        let a242 = 1u64;
        if nat_eq(a241, a242) {
            Str::lit("nat_sub")
        } else {
            let a243 = v0;
            let a244 = 2u64;
            if nat_eq(a243, a244) {
                Str::lit("nat_mul")
            } else {
                let a245 = v0;
                let a246 = 3u64;
                if nat_eq(a245, a246) {
                    Str::lit("nat_quot")
                } else {
                    let a247 = v0;
                    let a248 = 4u64;
                    if nat_eq(a247, a248) {
                        Str::lit("nat_rem")
                    } else {
                        let a249 = v0;
                        let a250 = 5u64;
                        if nat_eq(a249, a250) {
                            Str::lit("nat_eq")
                        } else {
                            let a251 = v0;
                            let a252 = 6u64;
                            if nat_eq(a251, a252) {
                                Str::lit("nat_le")
                            } else {
                                let a253 = v0;
                                let a254 = 7u64;
                                if nat_eq(a253, a254) {
                                    Str::lit("nat_lt")
                                } else {
                                    let a255 = v0;
                                    let a256 = 8u64;
                                    if nat_eq(a255, a256) {
                                        Str::lit("nat_succ")
                                    } else {
                                        let a257 = v0;
                                        let a258 = 9u64;
                                        if nat_eq(a257, a258) {
                                            Str::lit("int_add")
                                        } else {
                                            let a259 = v0;
                                            let a260 = 10u64;
                                            if nat_eq(a259, a260) {
                                                Str::lit("int_sub")
                                            } else {
                                                let a261 = v0;
                                                let a262 = 11u64;
                                                if nat_eq(a261, a262) {
                                                    Str::lit("int_mul")
                                                } else {
                                                    let a263 = v0;
                                                    let a264 = 12u64;
                                                    if nat_eq(a263, a264) {
                                                        Str::lit("int_neg")
                                                    } else {
                                                        let a265 = v0;
                                                        let a266 = 13u64;
                                                        if nat_eq(a265, a266) {
                                                            Str::lit("int_quot")
                                                        } else {
                                                            let a267 = v0;
                                                            let a268 = 14u64;
                                                            if nat_eq(a267, a268) {
                                                                Str::lit("int_rem")
                                                            } else {
                                                                let a269 = v0;
                                                                let a270 = 15u64;
                                                                if nat_eq(a269, a270) {
                                                                    Str::lit("bool_not")
                                                                } else {
                                                                    let a271 = v0;
                                                                    let a272 = 16u64;
                                                                    if nat_eq(a271, a272) {
                                                                        Str::lit("bool_and")
                                                                    } else {
                                                                        let a273 = v0;
                                                                        let a274 = 17u64;
                                                                        if nat_eq(a273, a274) {
                                                                            Str::lit("bool_or")
                                                                        } else {
                                                                            let a275 = v0;
                                                                            let a276 = 18u64;
                                                                            if nat_eq(a275, a276) {
                                                                                Str::lit("equal")
                                                                            } else {
                                                                                let a277 = v0;
                                                                                let a278 = 19u64;
                                                                                if nat_eq(a277, a278) {
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
    let a279 = v0;
    let a280 = 32u64;
    if nat_eq(a279, a280) {
        Str::lit("append_list")
    } else {
        let a281 = v0;
        let a282 = 33u64;
        if nat_eq(a281, a282) {
            Str::lit("append_bytes")
        } else {
            let a283 = v0;
            let a284 = 34u64;
            if nat_eq(a283, a284) {
                Str::lit("length_list")
            } else {
                let a285 = v0;
                let a286 = 35u64;
                if nat_eq(a285, a286) {
                    Str::lit("length_bytes")
                } else {
                    let a287 = v0;
                    let a288 = 36u64;
                    if nat_eq(a287, a288) {
                        Str::lit("length_string")
                    } else {
                        let a289 = v0;
                        let a290 = 37u64;
                        if nat_eq(a289, a290) {
                            Str::lit("index_list")
                        } else {
                            let a291 = v0;
                            let a292 = 38u64;
                            if nat_eq(a291, a292) {
                                Str::lit("index_bytes")
                            } else {
                                let a293 = v0;
                                let a294 = 39u64;
                                if nat_eq(a293, a294) {
                                    Str::lit("slice_list")
                                } else {
                                    let a295 = v0;
                                    let a296 = 40u64;
                                    if nat_eq(a295, a296) {
                                        Str::lit("slice_bytes")
                                    } else {
                                        let a297 = v0;
                                        let a298 = 41u64;
                                        if nat_eq(a297, a298) {
                                            Str::lit("utf8_encode")
                                        } else {
                                            let a299 = v0;
                                            let a300 = 42u64;
                                            if nat_eq(a299, a300) {
                                                Str::lit("utf8_decode")
                                            } else {
                                                let a301 = v0;
                                                let a302 = 43u64;
                                                if nat_eq(a301, a302) {
                                                    Str::lit("compare_bytes")
                                                } else {
                                                    let a303 = v0;
                                                    let a304 = 44u64;
                                                    if nat_eq(a303, a304) {
                                                        Str::lit("split_exact")
                                                    } else {
                                                        let a305 = v0;
                                                        let a306 = 45u64;
                                                        if nat_eq(a305, a306) {
                                                            Str::lit("join")
                                                        } else {
                                                            let a307 = v0;
                                                            let a308 = 46u64;
                                                            if nat_eq(a307, a308) {
                                                                Str::lit("format_i64")
                                                            } else {
                                                                let a309 = v0;
                                                                let a310 = 48u64;
                                                                if nat_eq(a309, a310) {
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
    let a311 = v0;
    let a312 = 0u64;
    if nat_eq(a311, a312) {
        Str::lit("checked_add")
    } else {
        let a313 = v0;
        let a314 = 1u64;
        if nat_eq(a313, a314) {
            Str::lit("checked_sub")
        } else {
            let a315 = v0;
            let a316 = 2u64;
            if nat_eq(a315, a316) {
                Str::lit("checked_mul")
            } else {
                let a317 = v0;
                let a318 = 3u64;
                if nat_eq(a317, a318) {
                    Str::lit("checked_div")
                } else {
                    let a319 = v0;
                    let a320 = 4u64;
                    if nat_eq(a319, a320) {
                        Str::lit("checked_neg")
                    } else {
                        let a321 = v0;
                        let a322 = 5u64;
                        if nat_eq(a321, a322) {
                            Str::lit("checked_shl")
                        } else {
                            let a323 = v0;
                            let a324 = 6u64;
                            if nat_eq(a323, a324) {
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
    let a325 = v0;
    let a326 = 8u64;
    if nat_eq(a325, a326) {
        Str::lit("core::ops::BitAnd::bitand(")
    } else {
        let a327 = v0;
        let a328 = 9u64;
        if nat_eq(a327, a328) {
            Str::lit("core::ops::BitOr::bitor(")
        } else {
            let a329 = v0;
            let a330 = 10u64;
            if nat_eq(a329, a330) {
                Str::lit("core::ops::BitXor::bitxor(")
            } else {
                let a331 = v0;
                let a332 = 11u64;
                if nat_eq(a331, a332) {
                    Str::lit("core::ops::Not::not(")
                } else {
                    let a333 = v0;
                    let a334 = 12u64;
                    if nat_eq(a333, a334) {
                        Str::lit("core::cmp::PartialEq::eq(&")
                    } else {
                        let a335 = v0;
                        let a336 = 13u64;
                        if nat_eq(a335, a336) {
                            Str::lit("core::cmp::PartialOrd::le(&")
                        } else {
                            let a337 = v0;
                            let a338 = 14u64;
                            if nat_eq(a337, a338) {
                                Str::lit("core::cmp::PartialOrd::lt(&")
                            } else {
                                let a339 = v0;
                                let a340 = 15u64;
                                if nat_eq(a339, a340) {
                                    Str::lit("u64::checked_rem(")
                                } else {
                                    let a341 = v0;
                                    let a342 = 16u64;
                                    if nat_eq(a341, a342) {
                                        Str::lit("{ let (a, b, z): (i64, i64, i64) = (")
                                    } else {
                                        let a343 = v0;
                                        let a344 = 17u64;
                                        if nat_eq(a343, a344) {
                                            Str::lit("u64::checked_add(")
                                        } else {
                                            let a345 = v0;
                                            let a346 = 18u64;
                                            if nat_eq(a345, a346) {
                                                Str::lit("u64::saturating_sub(")
                                            } else {
                                                let a347 = v0;
                                                let a348 = 19u64;
                                                if nat_eq(a347, a348) {
                                                    Str::lit("u64::checked_mul(")
                                                } else {
                                                    let a349 = v0;
                                                    let a350 = 20u64;
                                                    if nat_eq(a349, a350) {
                                                        Str::lit("u64::checked_div(")
                                                    } else {
                                                        let a351 = v0;
                                                        let a352 = 21u64;
                                                        if nat_eq(a351, a352) {
                                                            Str::lit("i64::checked_add(")
                                                        } else {
                                                            let a353 = v0;
                                                            let a354 = 22u64;
                                                            if nat_eq(a353, a354) {
                                                                Str::lit("i64::checked_sub(")
                                                            } else {
                                                                let a355 = v0;
                                                                let a356 = 23u64;
                                                                if nat_eq(a355, a356) {
                                                                    Str::lit("i64::checked_mul(")
                                                                } else {
                                                                    let a357 = v0;
                                                                    let a358 = 24u64;
                                                                    if nat_eq(a357, a358) {
                                                                        Str::lit("i64::checked_neg(")
                                                                    } else {
                                                                        let a359 = v0;
                                                                        let a360 = 25u64;
                                                                        if nat_eq(a359, a360) {
                                                                            Str::lit("{ let (a, b, z): (i64, i64, i64) = (")
                                                                        } else {
                                                                            let a361 = v0;
                                                                            let a362 = 26u64;
                                                                            if nat_eq(a361, a362) {
                                                                                Str::lit("List::length(&")
                                                                            } else {
                                                                                let a363 = v0;
                                                                                let a364 = 27u64;
                                                                                if nat_eq(a363, a364) {
                                                                                    Str::lit("List::append(")
                                                                                } else {
                                                                                    let a365 = v0;
                                                                                    let a366 = 28u64;
                                                                                    if nat_eq(a365, a366) {
                                                                                        Str::lit("List::index(&")
                                                                                    } else {
                                                                                        let a367 = v0;
                                                                                        let a368 = 29u64;
                                                                                        if nat_eq(a367, a368) {
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

pub fn f43(v0: u64) -> Str {
    let a369 = v0;
    let a370 = 16u64;
    if nat_eq(a369, a370) {
        Str::lit("); if b == 0 { z } else { a.wrapping_rem(b) } }")
    } else {
        let a371 = v0;
        let a372 = 25u64;
        if nat_eq(a371, a372) {
            Str::lit("); if b == 0 { Some(z) } else { a.checked_div(b) } }")
        } else {
            let a373 = v0;
            let a374 = 15u64;
            if nat_eq(a373, a374) {
                Str::lit(")")
            } else {
                let a375 = v0;
                let a376 = 20u64;
                if nat_eq(a375, a376) {
                    Str::lit(")")
                } else {
                    let a377 = v0;
                    let a378 = 8u64;
                    if nat_eq(a377, a378) {
                        Str::lit(")")
                    } else {
                        let a379 = v0;
                        let a380 = 9u64;
                        if nat_eq(a379, a380) {
                            Str::lit(")")
                        } else {
                            let a381 = v0;
                            let a382 = 10u64;
                            if nat_eq(a381, a382) {
                                Str::lit(")")
                            } else {
                                let a383 = v0;
                                let a384 = 11u64;
                                if nat_eq(a383, a384) {
                                    Str::lit(")")
                                } else {
                                    let a385 = v0;
                                    let a386 = 12u64;
                                    if nat_eq(a385, a386) {
                                        Str::lit(")")
                                    } else {
                                        let a387 = v0;
                                        let a388 = 13u64;
                                        if nat_eq(a387, a388) {
                                            Str::lit(")")
                                        } else {
                                            let a389 = v0;
                                            let a390 = 14u64;
                                            if nat_eq(a389, a390) {
                                                Str::lit(")")
                                            } else {
                                                let a391 = v0;
                                                let a392 = 17u64;
                                                if nat_eq(a391, a392) {
                                                    Str::lit(")")
                                                } else {
                                                    let a393 = v0;
                                                    let a394 = 18u64;
                                                    if nat_eq(a393, a394) {
                                                        Str::lit(")")
                                                    } else {
                                                        let a395 = v0;
                                                        let a396 = 19u64;
                                                        if nat_eq(a395, a396) {
                                                            Str::lit(")")
                                                        } else {
                                                            let a397 = v0;
                                                            let a398 = 21u64;
                                                            if nat_eq(a397, a398) {
                                                                Str::lit(")")
                                                            } else {
                                                                let a399 = v0;
                                                                let a400 = 22u64;
                                                                if nat_eq(a399, a400) {
                                                                    Str::lit(")")
                                                                } else {
                                                                    let a401 = v0;
                                                                    let a402 = 23u64;
                                                                    if nat_eq(a401, a402) {
                                                                        Str::lit(")")
                                                                    } else {
                                                                        let a403 = v0;
                                                                        let a404 = 24u64;
                                                                        let _ = nat_eq(a403, a404);
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

pub fn f44(v0: Fn2, p409: u64, p410: (u64, Adt20)) -> R<((u64, Adt20), bool)> {
    let m408 = (p409, p410);
    {
        let mut m408 = m408;
        loop {
            let step = {
                let (v1, v2) = m408;
                let m405 = v1;
                if m405 == 0 {
                    Err::<(u64, (u64, Adt20)), R<((u64, Adt20), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m405 - 1;
                    let m407 = {
                        let c406: Fn2 = v0.clone();
                        c406.apply(v2.clone())?
                    };
                    match m407 {
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
                Ok(next) => { m408 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f45(v0: u64, v1: (u64, Adt20)) -> R<Option<(u64, Adt20)>> {
    f59(v0, v1)
}

pub fn f46(v0: u64) -> Str {
    let a411 = v0;
    let a412 = 12u64;
    if nat_eq(a411, a412) {
        Str::lit(", &")
    } else {
        let a413 = v0;
        let a414 = 13u64;
        if nat_eq(a413, a414) {
            Str::lit(", &")
        } else {
            let a415 = v0;
            let a416 = 14u64;
            if nat_eq(a415, a416) {
                Str::lit(", &")
            } else {
                let a417 = v0;
                let a418 = 8u64;
                if nat_eq(a417, a418) {
                    Str::lit(", ")
                } else {
                    let a419 = v0;
                    let a420 = 9u64;
                    if nat_eq(a419, a420) {
                        Str::lit(", ")
                    } else {
                        let a421 = v0;
                        let a422 = 10u64;
                        if nat_eq(a421, a422) {
                            Str::lit(", ")
                        } else {
                            let a423 = v0;
                            let a424 = 11u64;
                            if nat_eq(a423, a424) {
                                Str::lit(", ")
                            } else {
                                let a425 = v0;
                                let a426 = 15u64;
                                if nat_eq(a425, a426) {
                                    Str::lit(", ")
                                } else {
                                    let a427 = v0;
                                    let a428 = 16u64;
                                    if nat_eq(a427, a428) {
                                        Str::lit(", ")
                                    } else {
                                        let a429 = v0;
                                        let a430 = 17u64;
                                        if nat_eq(a429, a430) {
                                            Str::lit(", ")
                                        } else {
                                            let a431 = v0;
                                            let a432 = 18u64;
                                            if nat_eq(a431, a432) {
                                                Str::lit(", ")
                                            } else {
                                                let a433 = v0;
                                                let a434 = 19u64;
                                                if nat_eq(a433, a434) {
                                                    Str::lit(", ")
                                                } else {
                                                    let a435 = v0;
                                                    let a436 = 20u64;
                                                    if nat_eq(a435, a436) {
                                                        Str::lit(", ")
                                                    } else {
                                                        let a437 = v0;
                                                        let a438 = 21u64;
                                                        if nat_eq(a437, a438) {
                                                            Str::lit(", ")
                                                        } else {
                                                            let a439 = v0;
                                                            let a440 = 22u64;
                                                            if nat_eq(a439, a440) {
                                                                Str::lit(", ")
                                                            } else {
                                                                let a441 = v0;
                                                                let a442 = 23u64;
                                                                if nat_eq(a441, a442) {
                                                                    Str::lit(", ")
                                                                } else {
                                                                    let a443 = v0;
                                                                    let a444 = 24u64;
                                                                    if nat_eq(a443, a444) {
                                                                        Str::lit(", ")
                                                                    } else {
                                                                        let a445 = v0;
                                                                        let a446 = 25u64;
                                                                        let _ = nat_eq(a445, a446);
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

pub fn f47(v0: u64) -> Str {
    let a447 = v0;
    let a448 = 15u64;
    if nat_eq(a447, a448) {
        Str::lit(").unwrap_or(")
    } else {
        let a449 = v0;
        let a450 = 16u64;
        if nat_eq(a449, a450) {
            Str::lit(", ")
        } else {
            let a451 = v0;
            let a452 = 20u64;
            if nat_eq(a451, a452) {
                Str::lit(").unwrap_or(")
            } else {
                let a453 = v0;
                let a454 = 25u64;
                let _ = nat_eq(a453, a454);
                Str::lit(", ")
            }
        }
    }
}

pub fn f48(v0: Adt20, v1: Adt12) -> R<Adt20> {
    f8(v0, {
        let m455 = v1;
        match m455 {
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
    let m456 = v0;
    match m456 {
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
    let m457 = v0;
    match m457 {
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
    let a458 = v1;
    let a459 = 0u64;
    if nat_eq(a458, a459) {
        f8(v0, Str::lit("T"))
    } else {
        let a460 = v1;
        let a461 = 1u64;
        if nat_eq(a460, a461) {
            f8(v0, Str::lit("U"))
        } else {
            let a462 = v1;
            let a463 = 2u64;
            if nat_eq(a462, a463) {
                f8(v0, Str::lit("V"))
            } else {
                f11(f8(v0, Str::lit("T"))?, v1)
            }
        }
    }
}

pub fn f52(v0: List<Bytes>) -> Option<List<Bytes>> {
    let m464 = v0.clone();
    match m464.uncons() {
        None => {
            None::<List<Bytes>>
        }
        Some((_, v2)) => {
            let m465 = v2;
            match m465.uncons() {
                None => {
                    None::<List<Bytes>>
                }
                Some((_, _)) => {
                    let v5: (Option<Bytes>, List<Bytes>) = f60(Fn3::F61, (None::<Bytes>, List::<Bytes>::nil()), v0);
                    let m467 = {
                        let (h466, _) = v5;
                        h466
                    };
                    match m467 {
                        None => {
                            Some(f5({
                                let (_, h469) = v5;
                                h469
                            }))
                        }
                        Some(v6) => {
                            Some(f5(List::cons(v6, {
                                let (_, h468) = v5;
                                h468
                            })))
                        }
                    }
                }
            }
        }
    }
}

pub fn f53(v0: Adt2) -> Str {
    let m470 = v0;
    match m470 {
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
    let a472 = 256u64;
    let a473 = {
        let a471 = v1.clone();
        length_bytes(a471)
    };
    if nat_le(a472, a473) {
        Adt20::C0(List::cons(v1, match v0.clone() {
            Adt20::C0(h474, _, _, _, _) => {
                h474
            }
        }), Bytes::lit(&[]), v2, match v0 {
            Adt20::C0(_, _, _, h475, _) => {
                h475
            }
        }, false)
    } else {
        Adt20::C0(match v0.clone() {
            Adt20::C0(h476, _, _, _, _) => {
                h476
            }
        }, v1, v2, match v0 {
            Adt20::C0(_, _, _, h477, _) => {
                h477
            }
        }, false)
    }
}

pub fn f57(v0: Adt20) -> Adt20 {
    Adt20::C0(match v0.clone() {
        Adt20::C0(h478, _, _, _, _) => {
            h478
        }
    }, match v0.clone() {
        Adt20::C0(_, h479, _, _, _) => {
            h479
        }
    }, match v0.clone() {
        Adt20::C0(_, _, h480, _, _) => {
            h480
        }
    }, match v0 {
        Adt20::C0(_, _, _, h481, _) => {
            h481
        }
    }, true)
}

pub fn f58(v0: u64) -> R<Bytes> {
    let v1: Bytes = {
        let a518 = f64(v0, 10000000000000000000u64);
        let a519 = {
            let a516 = f64(v0, 1000000000000000000u64);
            let a517 = {
                let a514 = f64(v0, 100000000000000000u64);
                let a515 = {
                    let a512 = f64(v0, 10000000000000000u64);
                    let a513 = {
                        let a510 = f64(v0, 1000000000000000u64);
                        let a511 = {
                            let a508 = f64(v0, 100000000000000u64);
                            let a509 = {
                                let a506 = f64(v0, 10000000000000u64);
                                let a507 = {
                                    let a504 = f64(v0, 1000000000000u64);
                                    let a505 = {
                                        let a502 = f64(v0, 100000000000u64);
                                        let a503 = {
                                            let a500 = f64(v0, 10000000000u64);
                                            let a501 = {
                                                let a498 = f64(v0, 1000000000u64);
                                                let a499 = {
                                                    let a496 = f64(v0, 100000000u64);
                                                    let a497 = {
                                                        let a494 = f64(v0, 10000000u64);
                                                        let a495 = {
                                                            let a492 = f64(v0, 1000000u64);
                                                            let a493 = {
                                                                let a490 = f64(v0, 100000u64);
                                                                let a491 = {
                                                                    let a488 = f64(v0, 10000u64);
                                                                    let a489 = {
                                                                        let a486 = f64(v0, 1000u64);
                                                                        let a487 = {
                                                                            let a484 = f64(v0, 100u64);
                                                                            let a485 = {
                                                                                let a482 = f64(v0, 10u64);
                                                                                let a483 = f64(v0, 1u64);
                                                                                append_bytes(a482, a483)
                                                                            };
                                                                            append_bytes(a484, a485)
                                                                        };
                                                                        append_bytes(a486, a487)
                                                                    };
                                                                    append_bytes(a488, a489)
                                                                };
                                                                append_bytes(a490, a491)
                                                            };
                                                            append_bytes(a492, a493)
                                                        };
                                                        append_bytes(a494, a495)
                                                    };
                                                    append_bytes(a496, a497)
                                                };
                                                append_bytes(a498, a499)
                                            };
                                            append_bytes(a500, a501)
                                        };
                                        append_bytes(a502, a503)
                                    };
                                    append_bytes(a504, a505)
                                };
                                append_bytes(a506, a507)
                            };
                            append_bytes(a508, a509)
                        };
                        append_bytes(a510, a511)
                    };
                    append_bytes(a512, a513)
                };
                append_bytes(a514, a515)
            };
            append_bytes(a516, a517)
        };
        append_bytes(a518, a519)
    };
    let v2: u64 = {
        let (h520, _) = f65(Fn4::F66(v1.clone()), {
            let a521 = v1.clone();
            length_bytes(a521)
        }, 0u64)?;
        h520
    };
    let a522 = v0;
    let a523 = 0u64;
    if nat_eq(a522, a523) {
        let a524 = Str::lit("0");
        Ok(utf8_encode(a524))
    } else {
        Ok(f67(v1.clone(), v2, {
            let a526 = {
                let a525 = v1;
                length_bytes(a525)
            };
            let a527 = v2;
            nat_sub(a526, a527)
        }))
    }
}

pub fn f59(v0: u64, v1: (u64, Adt20)) -> R<Option<(u64, Adt20)>> {
    let a529 = {
        let (h528, _) = v1.clone();
        h528
    };
    let a530 = v0;
    if nat_lt(a529, a530) {
        Ok(Some(({
            let a532 = {
                let (h531, _) = v1;
                h531
            };
            let a533 = 1u64;
            nat_add(a532, a533)?
        }, f8({
            let (_, h534) = v1;
            h534
        }, Str::lit("    "))?)))
    } else {
        Ok(None::<(u64, Adt20)>)
    }
}

pub fn f60(v0: Fn3, p538: (Option<Bytes>, List<Bytes>), p539: List<Bytes>) -> (Option<Bytes>, List<Bytes>) {
    let m537 = (p538, p539);
    {
        let mut m537 = m537;
        loop {
            let step = {
                let (v1, v2) = m537;
                let m535 = v2;
                match m535.uncons() {
                    None => {
                        Err::<((Option<Bytes>, List<Bytes>), List<Bytes>), (Option<Bytes>, List<Bytes>)>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<((Option<Bytes>, List<Bytes>), List<Bytes>), (Option<Bytes>, List<Bytes>)>(({
                            let c536: Fn3 = v0.clone();
                            c536.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m537 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f61(v0: (Option<Bytes>, List<Bytes>), v1: Bytes) -> (Option<Bytes>, List<Bytes>) {
    f68(v0, v1)
}

pub fn f62(v0: Bytes, v1: (u64, Adt20)) -> R<Option<(u64, Adt20)>> {
    let a542 = {
        let (h540, _) = v1.clone();
        h540
    };
    let a543 = {
        let a541 = v0.clone();
        length_bytes(a541)
    };
    if nat_lt(a542, a543) {
        let v2: (u64, u64) = f69(v0.clone(), {
            let (h544, _) = v1.clone();
            h544
        })?;
        let v3: u64 = {
            let (h545, _) = v2;
            h545
        };
        Ok(Some(({
            let a548 = {
                let (h546, _) = v1.clone();
                h546
            };
            let a549 = {
                let (_, h547) = v2;
                h547
            };
            nat_add(a548, a549)?
        }, {
            let a550 = v3;
            let a551 = 34u64;
            if nat_eq(a550, a551) {
                f8({
                    let (_, h552) = v1;
                    h552
                }, Str::lit("\\\""))?
            } else {
                let a553 = v3;
                let a554 = 92u64;
                if nat_eq(a553, a554) {
                    f8({
                        let (_, h555) = v1;
                        h555
                    }, Str::lit("\\\\"))?
                } else {
                    let a556 = 32u64;
                    let a557 = v3;
                    if if nat_le(a556, a557) {
                        let a558 = v3;
                        let a559 = 126u64;
                        nat_le(a558, a559)
                    } else {
                        false
                    } {
                        f36({
                            let (_, h560) = v1;
                            h560
                        }, f67(v0, {
                            let (h561, _) = v1;
                            h561
                        }, 1u64))?
                    } else {
                        f8(f70(f8({
                            let (_, h562) = v1;
                            h562
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
    let m564 = f71(v0, {
        let (h563, _) = v1.clone();
        h563
    });
    match m564 {
        None => {
            Ok(None::<(u64, Adt20)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a566 = {
                    let (h565, _) = v1.clone();
                    h565
                };
                let a567 = 1u64;
                nat_add(a566, a567)?
            }, f72(f8({
                let a569 = {
                    let (h568, _) = v1;
                    h568
                };
                let a570 = 0u64;
                if nat_eq(a569, a570) {
                    let (_, h571) = v1;
                    h571
                } else {
                    f8({
                        let (_, h572) = v1;
                        h572
                    }, Str::lit(", "))?
                }
            }, Str::lit("0x"))?, v2)?)))
        }
    }
}

pub fn f64(v0: u64, v1: u64) -> Bytes {
    f67(Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39]), {
        let a576 = {
            let a573 = v0;
            let a574 = v1;
            let a575 = 0u64;
            nat_quot(a573, a574, a575)
        };
        let a577 = 10u64;
        let a578 = 0u64;
        nat_rem(a576, a577, a578)
    }, 1u64)
}

pub fn f65(v0: Fn4, p583: u64, p584: u64) -> R<(u64, bool)> {
    let m582 = (p583, p584);
    {
        let mut m582 = m582;
        loop {
            let step = {
                let (v1, v2) = m582;
                let m579 = v1;
                if m579 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m579 - 1;
                    let m581 = {
                        let c580: Fn4 = v0.clone();
                        c580.apply(v2)?
                    };
                    match m581 {
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
                Ok(next) => { m582 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f66(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f73(v0, v1)
}

pub fn f67(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m588 = {
        let a585 = v0;
        let a586 = v1;
        let a587 = v2;
        slice_bytes(a585, a586, a587)
    };
    match m588 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f68(v0: (Option<Bytes>, List<Bytes>), v1: Bytes) -> (Option<Bytes>, List<Bytes>) {
    let m590 = {
        let (h589, _) = v0;
        h589
    };
    match m590 {
        None => {
            (Some(v1), {
                let (_, h591) = v0;
                h591
            })
        }
        Some(v2) => {
            (None::<Bytes>, List::cons({
                let a592 = v2;
                let a593 = v1;
                append_bytes(a592, a593)
            }, {
                let (_, h594) = v0;
                h594
            }))
        }
    }
}

pub fn f69(v0: Bytes, v1: u64) -> R<(u64, u64)> {
    let m595 = f71(v0.clone(), v1);
    match m595 {
        None => {
            Ok((0u64, 1u64))
        }
        Some(v2) => {
            let v4: u64 = {
                let v3: u8 = v2;
                let a640 = {
                    let a616 = {
                        let a604 = {
                            let a598 = {
                                let a596 = v3;
                                let a597 = 128u8;
                                fixed_u8::bit_and(a596, a597)
                            };
                            let a599 = 128u8;
                            if equal(a598, a599) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a605 = {
                            let a602 = {
                                let a600 = v3;
                                let a601 = 64u8;
                                fixed_u8::bit_and(a600, a601)
                            };
                            let a603 = 64u8;
                            if equal(a602, a603) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a604, a605)?
                    };
                    let a617 = {
                        let a614 = {
                            let a608 = {
                                let a606 = v3;
                                let a607 = 32u8;
                                fixed_u8::bit_and(a606, a607)
                            };
                            let a609 = 32u8;
                            if equal(a608, a609) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a615 = {
                            let a612 = {
                                let a610 = v3;
                                let a611 = 16u8;
                                fixed_u8::bit_and(a610, a611)
                            };
                            let a613 = 16u8;
                            if equal(a612, a613) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a614, a615)?
                    };
                    nat_add(a616, a617)?
                };
                let a641 = {
                    let a638 = {
                        let a626 = {
                            let a620 = {
                                let a618 = v3;
                                let a619 = 8u8;
                                fixed_u8::bit_and(a618, a619)
                            };
                            let a621 = 8u8;
                            if equal(a620, a621) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a627 = {
                            let a624 = {
                                let a622 = v3;
                                let a623 = 4u8;
                                fixed_u8::bit_and(a622, a623)
                            };
                            let a625 = 4u8;
                            if equal(a624, a625) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a626, a627)?
                    };
                    let a639 = {
                        let a636 = {
                            let a630 = {
                                let a628 = v3;
                                let a629 = 2u8;
                                fixed_u8::bit_and(a628, a629)
                            };
                            let a631 = 2u8;
                            if equal(a630, a631) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a637 = {
                            let a634 = {
                                let a632 = v3;
                                let a633 = 1u8;
                                fixed_u8::bit_and(a632, a633)
                            };
                            let a635 = 1u8;
                            if equal(a634, a635) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a636, a637)?
                    };
                    nat_add(a638, a639)?
                };
                nat_add(a640, a641)?
            };
            let a642 = v4;
            let a643 = 128u64;
            if nat_lt(a642, a643) {
                Ok((v4, 1u64))
            } else {
                let a644 = v4;
                let a645 = 224u64;
                if nat_lt(a644, a645) {
                    Ok(({
                        let a653 = {
                            let a649 = {
                                let a646 = v4;
                                let a647 = 32u64;
                                let a648 = 0u64;
                                nat_rem(a646, a647, a648)
                            };
                            let a650 = 64u64;
                            nat_mul(a649, a650)?
                        };
                        let a654 = f74(v0, {
                            let a651 = v1;
                            let a652 = 1u64;
                            nat_add(a651, a652)?
                        })?;
                        nat_add(a653, a654)?
                    }, 2u64))
                } else {
                    let a655 = v4;
                    let a656 = 240u64;
                    if nat_lt(a655, a656) {
                        Ok(({
                            let a670 = {
                                let a660 = {
                                    let a657 = v4;
                                    let a658 = 16u64;
                                    let a659 = 0u64;
                                    nat_rem(a657, a658, a659)
                                };
                                let a661 = 4096u64;
                                nat_mul(a660, a661)?
                            };
                            let a671 = {
                                let a668 = {
                                    let a664 = f74(v0.clone(), {
                                        let a662 = v1;
                                        let a663 = 1u64;
                                        nat_add(a662, a663)?
                                    })?;
                                    let a665 = 64u64;
                                    nat_mul(a664, a665)?
                                };
                                let a669 = f74(v0, {
                                    let a666 = v1;
                                    let a667 = 2u64;
                                    nat_add(a666, a667)?
                                })?;
                                nat_add(a668, a669)?
                            };
                            nat_add(a670, a671)?
                        }, 3u64))
                    } else {
                        Ok(({
                            let a691 = {
                                let a675 = {
                                    let a672 = v4;
                                    let a673 = 8u64;
                                    let a674 = 0u64;
                                    nat_rem(a672, a673, a674)
                                };
                                let a676 = 262144u64;
                                nat_mul(a675, a676)?
                            };
                            let a692 = {
                                let a689 = {
                                    let a679 = f74(v0.clone(), {
                                        let a677 = v1;
                                        let a678 = 1u64;
                                        nat_add(a677, a678)?
                                    })?;
                                    let a680 = 4096u64;
                                    nat_mul(a679, a680)?
                                };
                                let a690 = {
                                    let a687 = {
                                        let a683 = f74(v0.clone(), {
                                            let a681 = v1;
                                            let a682 = 2u64;
                                            nat_add(a681, a682)?
                                        })?;
                                        let a684 = 64u64;
                                        nat_mul(a683, a684)?
                                    };
                                    let a688 = f74(v0, {
                                        let a685 = v1;
                                        let a686 = 3u64;
                                        nat_add(a685, a686)?
                                    })?;
                                    nat_add(a687, a688)?
                                };
                                nat_add(a689, a690)?
                            };
                            nat_add(a691, a692)?
                        }, 4u64))
                    }
                }
            }
        }
    }
}

pub fn f70(v0: Adt20, v1: u64) -> R<Adt20> {
    let v2: Bytes = {
        let a705 = f75(v1, 268435456u64);
        let a706 = {
            let a703 = f75(v1, 16777216u64);
            let a704 = {
                let a701 = f75(v1, 1048576u64);
                let a702 = {
                    let a699 = f75(v1, 65536u64);
                    let a700 = {
                        let a697 = f75(v1, 4096u64);
                        let a698 = {
                            let a695 = f75(v1, 256u64);
                            let a696 = {
                                let a693 = f75(v1, 16u64);
                                let a694 = f75(v1, 1u64);
                                append_bytes(a693, a694)
                            };
                            append_bytes(a695, a696)
                        };
                        append_bytes(a697, a698)
                    };
                    append_bytes(a699, a700)
                };
                append_bytes(a701, a702)
            };
            append_bytes(a703, a704)
        };
        append_bytes(a705, a706)
    };
    let v3: u64 = {
        let (h707, _) = f65(Fn4::F76(v2.clone()), 7u64, 0u64)?;
        h707
    };
    f36(v0, f67(v2, v3, {
        let a708 = 8u64;
        let a709 = v3;
        nat_sub(a708, a709)
    }))
}

pub fn f71(v0: Bytes, v1: u64) -> Option<u8> {
    let a710 = v0;
    let a711 = v1;
    index_bytes(a710, a711)
}

pub fn f72(v0: Adt20, v1: u8) -> R<Adt20> {
    let v3: u64 = {
        let v2: u8 = v1;
        let a756 = {
            let a732 = {
                let a720 = {
                    let a714 = {
                        let a712 = v2;
                        let a713 = 128u8;
                        fixed_u8::bit_and(a712, a713)
                    };
                    let a715 = 128u8;
                    if equal(a714, a715) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a721 = {
                    let a718 = {
                        let a716 = v2;
                        let a717 = 64u8;
                        fixed_u8::bit_and(a716, a717)
                    };
                    let a719 = 64u8;
                    if equal(a718, a719) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a720, a721)?
            };
            let a733 = {
                let a730 = {
                    let a724 = {
                        let a722 = v2;
                        let a723 = 32u8;
                        fixed_u8::bit_and(a722, a723)
                    };
                    let a725 = 32u8;
                    if equal(a724, a725) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a731 = {
                    let a728 = {
                        let a726 = v2;
                        let a727 = 16u8;
                        fixed_u8::bit_and(a726, a727)
                    };
                    let a729 = 16u8;
                    if equal(a728, a729) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a730, a731)?
            };
            nat_add(a732, a733)?
        };
        let a757 = {
            let a754 = {
                let a742 = {
                    let a736 = {
                        let a734 = v2;
                        let a735 = 8u8;
                        fixed_u8::bit_and(a734, a735)
                    };
                    let a737 = 8u8;
                    if equal(a736, a737) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a743 = {
                    let a740 = {
                        let a738 = v2;
                        let a739 = 4u8;
                        fixed_u8::bit_and(a738, a739)
                    };
                    let a741 = 4u8;
                    if equal(a740, a741) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a742, a743)?
            };
            let a755 = {
                let a752 = {
                    let a746 = {
                        let a744 = v2;
                        let a745 = 2u8;
                        fixed_u8::bit_and(a744, a745)
                    };
                    let a747 = 2u8;
                    if equal(a746, a747) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a753 = {
                    let a750 = {
                        let a748 = v2;
                        let a749 = 1u8;
                        fixed_u8::bit_and(a748, a749)
                    };
                    let a751 = 1u8;
                    if equal(a750, a751) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a752, a753)?
            };
            nat_add(a754, a755)?
        };
        nat_add(a756, a757)?
    };
    f36(v0, {
        let a758 = f75(v3, 16u64);
        let a759 = f75(v3, 1u64);
        append_bytes(a758, a759)
    })
}

pub fn f73(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m760 = f71(v0, v1);
    match m760 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            let a761 = v2;
            let a762 = 48u8;
            if equal(a761, a762) {
                Ok(Some({
                    let a763 = v1;
                    let a764 = 1u64;
                    nat_add(a763, a764)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f74(v0: Bytes, v1: u64) -> R<u64> {
    let m765 = f71(v0, v1);
    match m765 {
        None => {
            Ok(0u64)
        }
        Some(v2) => {
            let a812 = {
                let v3: u8 = v2;
                let a810 = {
                    let a786 = {
                        let a774 = {
                            let a768 = {
                                let a766 = v3;
                                let a767 = 128u8;
                                fixed_u8::bit_and(a766, a767)
                            };
                            let a769 = 128u8;
                            if equal(a768, a769) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a775 = {
                            let a772 = {
                                let a770 = v3;
                                let a771 = 64u8;
                                fixed_u8::bit_and(a770, a771)
                            };
                            let a773 = 64u8;
                            if equal(a772, a773) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a774, a775)?
                    };
                    let a787 = {
                        let a784 = {
                            let a778 = {
                                let a776 = v3;
                                let a777 = 32u8;
                                fixed_u8::bit_and(a776, a777)
                            };
                            let a779 = 32u8;
                            if equal(a778, a779) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a785 = {
                            let a782 = {
                                let a780 = v3;
                                let a781 = 16u8;
                                fixed_u8::bit_and(a780, a781)
                            };
                            let a783 = 16u8;
                            if equal(a782, a783) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a784, a785)?
                    };
                    nat_add(a786, a787)?
                };
                let a811 = {
                    let a808 = {
                        let a796 = {
                            let a790 = {
                                let a788 = v3;
                                let a789 = 8u8;
                                fixed_u8::bit_and(a788, a789)
                            };
                            let a791 = 8u8;
                            if equal(a790, a791) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a797 = {
                            let a794 = {
                                let a792 = v3;
                                let a793 = 4u8;
                                fixed_u8::bit_and(a792, a793)
                            };
                            let a795 = 4u8;
                            if equal(a794, a795) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a796, a797)?
                    };
                    let a809 = {
                        let a806 = {
                            let a800 = {
                                let a798 = v3;
                                let a799 = 2u8;
                                fixed_u8::bit_and(a798, a799)
                            };
                            let a801 = 2u8;
                            if equal(a800, a801) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a807 = {
                            let a804 = {
                                let a802 = v3;
                                let a803 = 1u8;
                                fixed_u8::bit_and(a802, a803)
                            };
                            let a805 = 1u8;
                            if equal(a804, a805) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a806, a807)?
                    };
                    nat_add(a808, a809)?
                };
                nat_add(a810, a811)?
            };
            let a813 = 64u64;
            let a814 = 0u64;
            Ok(nat_rem(a812, a813, a814))
        }
    }
}

pub fn f75(v0: u64, v1: u64) -> Bytes {
    f67(Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66]), {
        let a818 = {
            let a815 = v0;
            let a816 = v1;
            let a817 = 0u64;
            nat_quot(a815, a816, a817)
        };
        let a819 = 16u64;
        let a820 = 0u64;
        nat_rem(a818, a819, a820)
    }, 1u64)
}

pub fn f76(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f77(v0, v1)
}

pub fn f77(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m821 = f71(v0, v1);
    match m821 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            let a822 = v2;
            let a823 = 48u8;
            if equal(a822, a823) {
                Ok(Some({
                    let a824 = v1;
                    let a825 = 1u64;
                    nat_add(a824, a825)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}
