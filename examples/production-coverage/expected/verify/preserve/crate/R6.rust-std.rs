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

pub fn f0(v0: List<(u64, List<u64>)>, v1: u64) -> R<(List<u64>, (List<u64>, Option<List<u64>>))> {
    Ok((f1(v0.clone(), v1), (f3(v0.clone(), v1)?, f16(v0)?)))
}

pub fn f1(v0: List<(u64, List<u64>)>, v1: u64) -> List<u64> {
    let m1 = f2(v0, v1);
    match m1 {
        None => {
            List::<u64>::nil()
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f2(p9: List<(u64, List<u64>)>, v1: u64) -> Option<List<u64>> {
    let m8 = p9;
    {
        let mut m8 = m8;
        loop {
            let step = {
                let v0 = m8;
                let m2 = v0;
                match m2.uncons() {
                    None => {
                        Err::<List<(u64, List<u64>)>, Option<List<u64>>>(None::<List<u64>>)
                    }
                    Some((v2, v3)) => {
                        let m6 = {
                            let a4 = v1;
                            let a5 = {
                                let (h3, _) = v2;
                                h3
                            };
                            compare(a4, a5)
                        };
                        match m6 {
                            Ordering::Less => {
                                Err::<List<(u64, List<u64>)>, Option<List<u64>>>(None::<List<u64>>)
                            }
                            Ordering::Equal => {
                                Err::<List<(u64, List<u64>)>, Option<List<u64>>>(Some({
                                    let (_, h7) = v2;
                                    h7
                                }))
                            }
                            Ordering::Greater => {
                                Ok::<List<(u64, List<u64>)>, Option<List<u64>>>(v3)
                            }
                        }
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

pub fn f3(v0: List<(u64, List<u64>)>, v1: u64) -> R<List<u64>> {
    Ok(f4(v0.clone(), {
        let a11 = {
            let a10 = f13(v0, List::<u64>::nil());
            length_list(a10)
        };
        let a12 = 1u64;
        nat_add(a11, a12)?
    }, List::cons(v1, List::<u64>::nil()), List::cons(v1, List::<u64>::nil())))
}

pub fn f4(v0: List<(u64, List<u64>)>, p16: u64, p17: List<u64>, p18: List<u64>) -> List<u64> {
    let m15 = (p16, (p17, p18));
    {
        let mut m15 = m15;
        loop {
            let step = {
                let (v1, (v2, v3)) = m15;
                let m13 = v1;
                if m13 == 0 {
                    Err::<(u64, (List<u64>, List<u64>)), List<u64>>(v3)
                } else {
                    let v4 = m13 - 1;
                    let v5: List<u64> = f5(v0.clone(), v3.clone(), v2, List::<u64>::nil());
                    let m14 = v5.clone();
                    match m14.uncons() {
                        None => {
                            Err::<(u64, (List<u64>, List<u64>)), List<u64>>(v3)
                        }
                        Some((_, _)) => {
                            Ok::<(u64, (List<u64>, List<u64>)), List<u64>>((v4, (v5.clone(), f11(v3, v5))))
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

pub fn f5(v0: List<(u64, List<u64>)>, v1: List<u64>, p21: List<u64>, p22: List<u64>) -> List<u64> {
    let m20 = (p21, p22);
    {
        let mut m20 = m20;
        loop {
            let step = {
                let (v2, v3) = m20;
                let m19 = v2;
                match m19.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v3)
                    }
                    Some((v4, v5)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>((v5, f6(v1.clone(), f7(v0.clone(), v4), v3)))
                    }
                }
            };
            match step {
                Ok(next) => { m20 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f6(v0: List<u64>, p25: List<u64>, p26: List<u64>) -> List<u64> {
    let m24 = (p25, p26);
    {
        let mut m24 = m24;
        loop {
            let step = {
                let (v1, v2) = m24;
                let m23 = v1;
                match m23.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v2)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>((v4, if f9(v0.clone(), v3) {
                            v2
                        } else {
                            if f9(v2.clone(), v3) {
                                v2
                            } else {
                                f10(v2, v3)
                            }
                        }))
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

pub fn f7(v0: List<(u64, List<u64>)>, v1: u64) -> List<u64> {
    let m27 = f8(v0, v1);
    match m27 {
        None => {
            List::<u64>::nil()
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f8(p35: List<(u64, List<u64>)>, v1: u64) -> Option<List<u64>> {
    let m34 = p35;
    {
        let mut m34 = m34;
        loop {
            let step = {
                let v0 = m34;
                let m28 = v0;
                match m28.uncons() {
                    None => {
                        Err::<List<(u64, List<u64>)>, Option<List<u64>>>(None::<List<u64>>)
                    }
                    Some((v2, v3)) => {
                        let m32 = {
                            let a30 = v1;
                            let a31 = {
                                let (h29, _) = v2;
                                h29
                            };
                            compare(a30, a31)
                        };
                        match m32 {
                            Ordering::Less => {
                                Err::<List<(u64, List<u64>)>, Option<List<u64>>>(None::<List<u64>>)
                            }
                            Ordering::Equal => {
                                Err::<List<(u64, List<u64>)>, Option<List<u64>>>(Some({
                                    let (_, h33) = v2;
                                    h33
                                }))
                            }
                            Ordering::Greater => {
                                Ok::<List<(u64, List<u64>)>, Option<List<u64>>>(v3)
                            }
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

pub fn f9(p41: List<u64>, v1: u64) -> bool {
    let m40 = p41;
    {
        let mut m40 = m40;
        loop {
            let step = {
                let v0 = m40;
                let m36 = v0;
                match m36.uncons() {
                    None => {
                        Err::<List<u64>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let m39 = {
                            let a37 = v1;
                            let a38 = v2;
                            compare(a37, a38)
                        };
                        match m39 {
                            Ordering::Less => {
                                Err::<List<u64>, bool>(false)
                            }
                            Ordering::Equal => {
                                Err::<List<u64>, bool>(true)
                            }
                            Ordering::Greater => {
                                Ok::<List<u64>, bool>(v3)
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m40 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f10(v0: List<u64>, v1: u64) -> List<u64> {
    let m42 = v0.clone();
    match m42.uncons() {
        None => {
            List::cons(v1, List::<u64>::nil())
        }
        Some((v2, v3)) => {
            let m45 = {
                let a43 = v1;
                let a44 = v2;
                compare(a43, a44)
            };
            match m45 {
                Ordering::Less => {
                    List::cons(v1, v0)
                }
                Ordering::Equal => {
                    v0
                }
                Ordering::Greater => {
                    List::cons(v2, f10(v3, v1))
                }
            }
        }
    }
}

pub fn f11(p48: List<u64>, p49: List<u64>) -> List<u64> {
    let m47 = (p48, p49);
    {
        let mut m47 = m47;
        loop {
            let step = {
                let (v0, v1) = m47;
                let m46 = v1;
                match m46.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v0)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>((f12(v0, v2), v3))
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

pub fn f12(v0: List<u64>, v1: u64) -> List<u64> {
    let m50 = v0.clone();
    match m50.uncons() {
        None => {
            List::cons(v1, List::<u64>::nil())
        }
        Some((v2, v3)) => {
            let m53 = {
                let a51 = v1;
                let a52 = v2;
                compare(a51, a52)
            };
            match m53 {
                Ordering::Less => {
                    List::cons(v1, v0)
                }
                Ordering::Equal => {
                    v0
                }
                Ordering::Greater => {
                    List::cons(v2, f12(v3, v1))
                }
            }
        }
    }
}

pub fn f13(p58: List<(u64, List<u64>)>, p59: List<u64>) -> List<u64> {
    let m57 = (p58, p59);
    {
        let mut m57 = m57;
        loop {
            let step = {
                let (v0, v1) = m57;
                let m54 = v0;
                match m54.uncons() {
                    None => {
                        Err::<(List<(u64, List<u64>)>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(u64, List<u64>)>, List<u64>), List<u64>>((v3, f14({
                            let (_, h55) = v2;
                            h55
                        }, f15(v1, {
                            let (h56, _) = v2;
                            h56
                        }))))
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

pub fn f14(p62: List<u64>, p63: List<u64>) -> List<u64> {
    let m61 = (p62, p63);
    {
        let mut m61 = m61;
        loop {
            let step = {
                let (v0, v1) = m61;
                let m60 = v0;
                match m60.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>((v3, f15(v1, v2)))
                    }
                }
            };
            match step {
                Ok(next) => { m61 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f15(v0: List<u64>, v1: u64) -> List<u64> {
    let m64 = v0.clone();
    match m64.uncons() {
        None => {
            List::cons(v1, List::<u64>::nil())
        }
        Some((v2, v3)) => {
            let m67 = {
                let a65 = v1;
                let a66 = v2;
                compare(a65, a66)
            };
            match m67 {
                Ordering::Less => {
                    List::cons(v1, v0)
                }
                Ordering::Equal => {
                    v0
                }
                Ordering::Greater => {
                    List::cons(v2, f15(v3, v1))
                }
            }
        }
    }
}

pub fn f16(v0: List<(u64, List<u64>)>) -> R<Option<List<u64>>> {
    let v1: List<u64> = f21(v0.clone(), List::<u64>::nil());
    Ok(f17(v0, {
        let a69 = {
            let a68 = v1.clone();
            length_list(a68)
        };
        let a70 = 1u64;
        nat_add(a69, a70)?
    }, v1, List::<u64>::nil()))
}

pub fn f17(v0: List<(u64, List<u64>)>, p76: u64, p77: List<u64>, p78: List<u64>) -> Option<List<u64>> {
    let m75 = (p76, (p77, p78));
    {
        let mut m75 = m75;
        loop {
            let step = {
                let (v1, (v2, v3)) = m75;
                let m71 = v1;
                if m71 == 0 {
                    let m72 = v2;
                    match m72.uncons() {
                        None => {
                            Err::<(u64, (List<u64>, List<u64>)), Option<List<u64>>>(Some(f20(v3, List::<u64>::nil())))
                        }
                        Some((_, _)) => {
                            Err::<(u64, (List<u64>, List<u64>)), Option<List<u64>>>(None::<List<u64>>)
                        }
                    }
                } else {
                    let v6 = m71 - 1;
                    let m73 = f18(v0.clone(), v2.clone(), v2.clone());
                    match m73.uncons() {
                        None => {
                            let m74 = v2;
                            match m74.uncons() {
                                None => {
                                    Err::<(u64, (List<u64>, List<u64>)), Option<List<u64>>>(Some(f20(v3, List::<u64>::nil())))
                                }
                                Some((_, _)) => {
                                    Err::<(u64, (List<u64>, List<u64>)), Option<List<u64>>>(None::<List<u64>>)
                                }
                            }
                        }
                        Some((v9, _)) => {
                            Ok::<(u64, (List<u64>, List<u64>)), Option<List<u64>>>((v6, (f27(v2, v9), List::cons(v9, v3))))
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

pub fn f18(v0: List<(u64, List<u64>)>, v1: List<u64>, v2: List<u64>) -> List<u64> {
    let m79 = v2;
    match m79.uncons() {
        None => {
            List::<u64>::nil()
        }
        Some((v3, v4)) => {
            if f19(v0.clone(), v3, v1.clone()) {
                List::cons(v3, f18(v0, v1, v4))
            } else {
                f18(v0, v1, v4)
            }
        }
    }
}

pub fn f19(v0: List<(u64, List<u64>)>, v1: u64, p82: List<u64>) -> bool {
    let m81 = p82;
    {
        let mut m81 = m81;
        loop {
            let step = {
                let v2 = m81;
                let m80 = v2;
                match m80.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v3, v4)) => {
                        if f26(f24(v0.clone(), v3), v1) {
                            Err::<List<u64>, bool>(false)
                        } else {
                            Ok::<List<u64>, bool>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m81 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f20(p85: List<u64>, p86: List<u64>) -> List<u64> {
    let m84 = (p85, p86);
    {
        let mut m84 = m84;
        loop {
            let step = {
                let (v0, v1) = m84;
                let m83 = v0;
                match m83.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m84 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f21(p91: List<(u64, List<u64>)>, p92: List<u64>) -> List<u64> {
    let m90 = (p91, p92);
    {
        let mut m90 = m90;
        loop {
            let step = {
                let (v0, v1) = m90;
                let m87 = v0;
                match m87.uncons() {
                    None => {
                        Err::<(List<(u64, List<u64>)>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(u64, List<u64>)>, List<u64>), List<u64>>((v3, f22({
                            let (_, h88) = v2;
                            h88
                        }, f23(v1, {
                            let (h89, _) = v2;
                            h89
                        }))))
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

pub fn f22(p95: List<u64>, p96: List<u64>) -> List<u64> {
    let m94 = (p95, p96);
    {
        let mut m94 = m94;
        loop {
            let step = {
                let (v0, v1) = m94;
                let m93 = v0;
                match m93.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>((v3, f23(v1, v2)))
                    }
                }
            };
            match step {
                Ok(next) => { m94 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f23(v0: List<u64>, v1: u64) -> List<u64> {
    let m97 = v0.clone();
    match m97.uncons() {
        None => {
            List::cons(v1, List::<u64>::nil())
        }
        Some((v2, v3)) => {
            let m100 = {
                let a98 = v1;
                let a99 = v2;
                compare(a98, a99)
            };
            match m100 {
                Ordering::Less => {
                    List::cons(v1, v0)
                }
                Ordering::Equal => {
                    v0
                }
                Ordering::Greater => {
                    List::cons(v2, f23(v3, v1))
                }
            }
        }
    }
}

pub fn f24(v0: List<(u64, List<u64>)>, v1: u64) -> List<u64> {
    let m101 = f25(v0, v1);
    match m101 {
        None => {
            List::<u64>::nil()
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f25(p109: List<(u64, List<u64>)>, v1: u64) -> Option<List<u64>> {
    let m108 = p109;
    {
        let mut m108 = m108;
        loop {
            let step = {
                let v0 = m108;
                let m102 = v0;
                match m102.uncons() {
                    None => {
                        Err::<List<(u64, List<u64>)>, Option<List<u64>>>(None::<List<u64>>)
                    }
                    Some((v2, v3)) => {
                        let m106 = {
                            let a104 = v1;
                            let a105 = {
                                let (h103, _) = v2;
                                h103
                            };
                            compare(a104, a105)
                        };
                        match m106 {
                            Ordering::Less => {
                                Err::<List<(u64, List<u64>)>, Option<List<u64>>>(None::<List<u64>>)
                            }
                            Ordering::Equal => {
                                Err::<List<(u64, List<u64>)>, Option<List<u64>>>(Some({
                                    let (_, h107) = v2;
                                    h107
                                }))
                            }
                            Ordering::Greater => {
                                Ok::<List<(u64, List<u64>)>, Option<List<u64>>>(v3)
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m108 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f26(p115: List<u64>, v1: u64) -> bool {
    let m114 = p115;
    {
        let mut m114 = m114;
        loop {
            let step = {
                let v0 = m114;
                let m110 = v0;
                match m110.uncons() {
                    None => {
                        Err::<List<u64>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let m113 = {
                            let a111 = v1;
                            let a112 = v2;
                            compare(a111, a112)
                        };
                        match m113 {
                            Ordering::Less => {
                                Err::<List<u64>, bool>(false)
                            }
                            Ordering::Equal => {
                                Err::<List<u64>, bool>(true)
                            }
                            Ordering::Greater => {
                                Ok::<List<u64>, bool>(v3)
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m114 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f27(v0: List<u64>, v1: u64) -> List<u64> {
    let m116 = v0.clone();
    match m116.uncons() {
        None => {
            List::<u64>::nil()
        }
        Some((v2, v3)) => {
            let m119 = {
                let a117 = v1;
                let a118 = v2;
                compare(a117, a118)
            };
            match m119 {
                Ordering::Less => {
                    v0
                }
                Ordering::Equal => {
                    v3
                }
                Ordering::Greater => {
                    List::cons(v2, f27(v3, v1))
                }
            }
        }
    }
}

pub fn f28(p129: List<(u64, List<u64>)>) -> bool {
    let m128 = p129;
    {
        let mut m128 = m128;
        loop {
            let step = {
                let v0 = m128;
                let m120 = v0;
                match m120.uncons() {
                    None => {
                        Err::<List<(u64, List<u64>)>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f29({
                            let (_, h121) = v1;
                            h121
                        }) {
                            let m122 = v2.clone();
                            match m122.uncons() {
                                None => {
                                    Err::<List<(u64, List<u64>)>, bool>(true)
                                }
                                Some((v3, _)) => {
                                    let m127 = {
                                        let a125 = {
                                            let (h123, _) = v1;
                                            h123
                                        };
                                        let a126 = {
                                            let (h124, _) = v3;
                                            h124
                                        };
                                        compare(a125, a126)
                                    };
                                    if match m127 {
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
                                        Ok::<List<(u64, List<u64>)>, bool>(v2)
                                    } else {
                                        Err::<List<(u64, List<u64>)>, bool>(false)
                                    }
                                }
                            }
                        } else {
                            Err::<List<(u64, List<u64>)>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m128 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f29(p136: List<u64>) -> bool {
    let m135 = p136;
    {
        let mut m135 = m135;
        loop {
            let step = {
                let v0 = m135;
                let m130 = v0;
                match m130.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let m131 = v2.clone();
                        match m131.uncons() {
                            None => {
                                Err::<List<u64>, bool>(true)
                            }
                            Some((v3, _)) => {
                                let m134 = {
                                    let a132 = v1;
                                    let a133 = v3;
                                    compare(a132, a133)
                                };
                                if match m134 {
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
                                    Ok::<List<u64>, bool>(v2)
                                } else {
                                    Err::<List<u64>, bool>(false)
                                }
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

pub fn f30(v0: List<(u64, List<u64>)>, v1: u64) -> R<Option<(List<u64>, (List<u64>, Option<List<u64>>))>> {
    if f28(v0.clone()) {
        Ok(Some(f0(v0, v1)?))
    } else {
        Ok(None::<(List<u64>, (List<u64>, Option<List<u64>>))>)
    }
}
