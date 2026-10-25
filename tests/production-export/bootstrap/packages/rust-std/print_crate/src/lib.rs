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
        let a270 = 30u64;
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
                nat_eq(a276, a277)
            } {
                1u64
            } else {
                let a278 = v0;
                let a279 = 15u64;
                if nat_eq(a278, a279) {
                    3u64
                } else {
                    let a280 = v0;
                    let a281 = 16u64;
                    if nat_eq(a280, a281) {
                        3u64
                    } else {
                        let a282 = v0;
                        let a283 = 20u64;
                        if nat_eq(a282, a283) {
                            3u64
                        } else {
                            let a284 = v0;
                            let a285 = 25u64;
                            if if nat_eq(a284, a285) {
                                true
                            } else {
                                let a286 = v0;
                                let a287 = 29u64;
                                nat_eq(a286, a287)
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
    let a288 = v1;
    let a289 = 8u64;
    if nat_lt(a288, a289) {
        f8(v0, Str::lit(", "))
    } else {
        let a290 = v1;
        let a291 = 100u64;
        if nat_eq(a290, a291) {
            f8(v0, Str::lit(", "))
        } else {
            let a292 = v2;
            let a293 = 0u64;
            if nat_eq(a292, a293) {
                f8(v0, f82(v1))
            } else {
                f8(v0, f83(v1))
            }
        }
    }
}

pub fn f58(v0: Adt26, v1: Adt14) -> R<Adt26> {
    let m294 = v1;
    match m294 {
        Adt14::C0(v2) => {
            f8(v0, {
                let m295 = v2;
                match m295 {
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
        Adt14::C2(r299) => {
            let v4 = (*r299).clone();
            f8(f58(f8(v0, Str::lit("("))?, v4)?, Str::lit(")"))
        }
        Adt14::C3(v5, r300) => {
            let v6 = (*r300).clone();
            f8(f58(f8(v0, {
                let m296 = v5;
                match m296 {
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
        Adt14::C4(v7, v8, r301) => {
            let v9 = (*r301).clone();
            let v10: Adt26 = f19(f8(f19(f8(v0, Str::lit("Adt"))?, v7)?, Str::lit("::C"))?, v8)?;
            let m297 = v9.clone();
            match m297 {
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
        Adt14::C6(r302, r303) => {
            let v22 = (*r302).clone();
            let v23 = (*r303).clone();
            let m298 = v23.clone();
            match m298 {
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
    let a304 = v0;
    let a305 = v1;
    if nat_lt(a304, a305) {
        v1
    } else {
        v0
    }
}

pub fn f60(v0: Adt13) -> R<u64> {
    let m306 = v0;
    match m306 {
        Adt13::C0 => {
            Ok(0u64)
        }
        Adt13::C1(_, v2, r307) => {
            let v3 = (*r307).clone();
            Ok(f59(f61(v2)?, f60(v3)?))
        }
    }
}

pub fn f61(v0: Adt2) -> R<u64> {
    let m308 = v0;
    match m308 {
        Adt2::C0(_) => {
            Ok(0u64)
        }
        Adt2::C1(_) => {
            Ok(0u64)
        }
        Adt2::C2(_, r312) => {
            let v6 = (*r312).clone();
            f61(v6)
        }
        Adt2::C3(_, r313, r314) => {
            let v8 = (*r313).clone();
            let v9 = (*r314).clone();
            Ok(f59(f61(v8)?, f61(v9)?))
        }
        Adt2::C4(v1, v2) => {
            let m309 = v1;
            match m309 {
                Adt7::C0 => {
                    Ok(0u64)
                }
                Adt7::C1 => {
                    Ok(0u64)
                }
                Adt7::C2 => {
                    let a310 = v2;
                    let a311 = 1u64;
                    nat_add(a310, a311)
                }
            }
        }
        Adt2::C5 => {
            Ok(0u64)
        }
        Adt2::C6(r315, r316) => {
            let v10 = (*r315).clone();
            let v11 = (*r316).clone();
            Ok(f59(f61(v10)?, f61(v11)?))
        }
    }
}

pub fn f62(v0: Fn4, p321: u64, p322: (u64, (Adt26, bool))) -> R<((u64, (Adt26, bool)), bool)> {
    let m320 = (p321, p322);
    {
        let mut m320 = m320;
        loop {
            let step = {
                let (v1, v2) = m320;
                let m317 = v1;
                if m317 == 0 {
                    Err::<(u64, (u64, (Adt26, bool))), R<((u64, (Adt26, bool)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m317 - 1;
                    let m319 = {
                        let c318: Fn4 = v0.clone();
                        c318.apply(v2.clone())?
                    };
                    match m319 {
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
                Ok(next) => { m320 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f63(v0: u64, v1: Adt13, v2: Adt2, v3: (u64, (Adt26, bool))) -> R<Option<(u64, (Adt26, bool))>> {
    f84(v1, v2, v0, v3)
}

pub fn f64(v0: Fn5, p326: (Option<Bytes>, List<Bytes>), p327: List<Bytes>) -> (Option<Bytes>, List<Bytes>) {
    let m325 = (p326, p327);
    {
        let mut m325 = m325;
        loop {
            let step = {
                let (v1, v2) = m325;
                let m323 = v2;
                match m323.uncons() {
                    None => {
                        Err::<((Option<Bytes>, List<Bytes>), List<Bytes>), (Option<Bytes>, List<Bytes>)>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<((Option<Bytes>, List<Bytes>), List<Bytes>), (Option<Bytes>, List<Bytes>)>(({
                            let c324: Fn5 = v0.clone();
                            c324.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m325 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f65(v0: (Option<Bytes>, List<Bytes>), v1: Bytes) -> (Option<Bytes>, List<Bytes>) {
    f85(v0, v1)
}

pub fn f66(v0: Adt4) -> Str {
    let m328 = v0;
    match m328 {
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
        let a365 = f86(v0, 10000000000000000000u64);
        let a366 = {
            let a363 = f86(v0, 1000000000000000000u64);
            let a364 = {
                let a361 = f86(v0, 100000000000000000u64);
                let a362 = {
                    let a359 = f86(v0, 10000000000000000u64);
                    let a360 = {
                        let a357 = f86(v0, 1000000000000000u64);
                        let a358 = {
                            let a355 = f86(v0, 100000000000000u64);
                            let a356 = {
                                let a353 = f86(v0, 10000000000000u64);
                                let a354 = {
                                    let a351 = f86(v0, 1000000000000u64);
                                    let a352 = {
                                        let a349 = f86(v0, 100000000000u64);
                                        let a350 = {
                                            let a347 = f86(v0, 10000000000u64);
                                            let a348 = {
                                                let a345 = f86(v0, 1000000000u64);
                                                let a346 = {
                                                    let a343 = f86(v0, 100000000u64);
                                                    let a344 = {
                                                        let a341 = f86(v0, 10000000u64);
                                                        let a342 = {
                                                            let a339 = f86(v0, 1000000u64);
                                                            let a340 = {
                                                                let a337 = f86(v0, 100000u64);
                                                                let a338 = {
                                                                    let a335 = f86(v0, 10000u64);
                                                                    let a336 = {
                                                                        let a333 = f86(v0, 1000u64);
                                                                        let a334 = {
                                                                            let a331 = f86(v0, 100u64);
                                                                            let a332 = {
                                                                                let a329 = f86(v0, 10u64);
                                                                                let a330 = f86(v0, 1u64);
                                                                                append_bytes(a329, a330)
                                                                            };
                                                                            append_bytes(a331, a332)
                                                                        };
                                                                        append_bytes(a333, a334)
                                                                    };
                                                                    append_bytes(a335, a336)
                                                                };
                                                                append_bytes(a337, a338)
                                                            };
                                                            append_bytes(a339, a340)
                                                        };
                                                        append_bytes(a341, a342)
                                                    };
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
    let v2: u64 = {
        let (h367, _) = f87(Fn6::F88(v1.clone()), {
            let a368 = v1.clone();
            length_bytes(a368)
        }, 0u64)?;
        h367
    };
    let a369 = v0;
    let a370 = 0u64;
    if nat_eq(a369, a370) {
        let a371 = Str::lit("0");
        Ok(utf8_encode(a371))
    } else {
        Ok(f89(v1.clone(), v2, {
            let a373 = {
                let a372 = v1;
                length_bytes(a372)
            };
            let a374 = v2;
            nat_sub(a373, a374)
        }))
    }
}

pub fn f68(v0: (u64, Adt26), v1: Adt11) -> R<(u64, Adt26)> {
    let v2: Adt26 = {
        let a376 = {
            let (h375, _) = v0.clone();
            h375
        };
        let a377 = 0u64;
        if nat_eq(a376, a377) {
            let (_, h378) = v0.clone();
            h378
        } else {
            f8({
                let (_, h379) = v0.clone();
                h379
            }, Str::lit(", "))?
        }
    };
    Ok(({
        let a381 = {
            let (h380, _) = v0.clone();
            h380
        };
        let a382 = 1u64;
        nat_add(a381, a382)?
    }, {
        let m383 = v1;
        match m383 {
            Adt11::C0 => {
                f19(f8(v2, Str::lit("k"))?, {
                    let (h384, _) = v0;
                    h384
                })?
            }
            Adt11::C1 => {
                f19(f8(v2, Str::lit("k"))?, {
                    let (h385, _) = v0;
                    h385
                })?
            }
            Adt11::C2 => {
                f19(f8(v2, Str::lit("k"))?, {
                    let (h386, _) = v0;
                    h386
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
        let a388 = {
            let (h387, _) = v0.clone();
            h387
        };
        let a389 = 0u64;
        if nat_eq(a388, a389) {
            let (_, h390) = v0.clone();
            h390
        } else {
            f8({
                let (_, h391) = v0.clone();
                h391
            }, Str::lit(", "))?
        }
    };
    Ok(({
        let a393 = {
            let (h392, _) = v0.clone();
            h392
        };
        let a394 = 1u64;
        nat_add(a393, a394)?
    }, {
        let m395 = v1;
        match m395 {
            Adt11::C0 => {
                f19(f8(v2, Str::lit("*k"))?, {
                    let (h396, _) = v0;
                    h396
                })?
            }
            Adt11::C1 => {
                f8(f19(f8(v2, Str::lit("k"))?, {
                    let (h397, _) = v0;
                    h397
                })?, Str::lit(".clone()"))?
            }
            Adt11::C2 => {
                f8(f19(f8(v2, Str::lit("(**k"))?, {
                    let (h398, _) = v0;
                    h398
                })?, Str::lit(").clone()"))?
            }
            Adt11::C3 => {
                f8(v2, Str::lit("()"))?
            }
        }
    }))
}

pub fn f70(v0: u64, v1: u64, v2: (u64, Adt26)) -> R<Option<(u64, Adt26)>> {
    let a400 = {
        let (h399, _) = v2.clone();
        h399
    };
    let a401 = v0;
    if nat_lt(a400, a401) {
        Ok(Some(({
            let a403 = {
                let (h402, _) = v2.clone();
                h402
            };
            let a404 = 1u64;
            nat_add(a403, a404)?
        }, f19(f8({
            let a406 = {
                let (h405, _) = v2.clone();
                h405
            };
            let a407 = 0u64;
            if if nat_eq(a406, a407) {
                let a408 = v1;
                let a409 = 0u64;
                nat_eq(a408, a409)
            } else {
                false
            } {
                let (_, h410) = v2;
                h410
            } else {
                f8({
                    let (_, h411) = v2;
                    h411
                }, Str::lit(", "))?
            }
        }, Str::lit("p"))?, {
            let (h412, _) = v2;
            h412
        })?)))
    } else {
        Ok(None::<(u64, Adt26)>)
    }
}

pub fn f71(v0: Adt4) -> u64 {
    let m413 = v0;
    match m413 {
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
        let (_, h414) = {
            let (h415, _) = f40(Fn3::F90(v1.clone()), {
                let a416 = v1;
                length_bytes(a416)
            }, (0u64, f8(v0, Str::lit("\""))?))?;
            h415
        };
        h414
    }, Str::lit("\""))
}

pub fn f73(v0: Adt26, v1: Bytes) -> R<Adt26> {
    f8({
        let (_, h417) = {
            let (h418, _) = f40(Fn3::F91(v1.clone()), {
                let a419 = v1;
                length_bytes(a419)
            }, (0u64, f8(v0, Str::lit("Bytes::lit(&["))?))?;
            h418
        };
        h417
    }, Str::lit("])"))
}

pub fn f74(v0: Adt26, v1: Adt17) -> R<Adt26> {
    f8(v0, {
        let m420 = v1;
        match m420 {
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
    let a421 = v0;
    let a422 = 20u64;
    if nat_eq(a421, a422) {
        Str::lit("checked_add")
    } else {
        let a423 = v0;
        let a424 = 21u64;
        if nat_eq(a423, a424) {
            Str::lit("checked_sub")
        } else {
            let a425 = v0;
            let a426 = 22u64;
            if nat_eq(a425, a426) {
                Str::lit("checked_mul")
            } else {
                let a427 = v0;
                let a428 = 23u64;
                if nat_eq(a427, a428) {
                    Str::lit("checked_quot")
                } else {
                    let a429 = v0;
                    let a430 = 25u64;
                    if nat_eq(a429, a430) {
                        Str::lit("bit_and")
                    } else {
                        let a431 = v0;
                        let a432 = 26u64;
                        if nat_eq(a431, a432) {
                            Str::lit("bit_or")
                        } else {
                            let a433 = v0;
                            let a434 = 27u64;
                            if nat_eq(a433, a434) {
                                Str::lit("bit_xor")
                            } else {
                                let a435 = v0;
                                let a436 = 28u64;
                                if nat_eq(a435, a436) {
                                    Str::lit("bit_not")
                                } else {
                                    let a437 = v0;
                                    let a438 = 29u64;
                                    if nat_eq(a437, a438) {
                                        Str::lit("shift_left")
                                    } else {
                                        let a439 = v0;
                                        let a440 = 30u64;
                                        if nat_eq(a439, a440) {
                                            Str::lit("shift_right")
                                        } else {
                                            let a441 = v0;
                                            let a442 = 31u64;
                                            if nat_eq(a441, a442) {
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
    let a443 = v0;
    let a444 = 0u64;
    if nat_eq(a443, a444) {
        Str::lit("nat_add")
    } else {
        let a445 = v0;
        let a446 = 1u64;
        if nat_eq(a445, a446) {
            Str::lit("nat_sub")
        } else {
            let a447 = v0;
            let a448 = 2u64;
            if nat_eq(a447, a448) {
                Str::lit("nat_mul")
            } else {
                let a449 = v0;
                let a450 = 3u64;
                if nat_eq(a449, a450) {
                    Str::lit("nat_quot")
                } else {
                    let a451 = v0;
                    let a452 = 4u64;
                    if nat_eq(a451, a452) {
                        Str::lit("nat_rem")
                    } else {
                        let a453 = v0;
                        let a454 = 5u64;
                        if nat_eq(a453, a454) {
                            Str::lit("nat_eq")
                        } else {
                            let a455 = v0;
                            let a456 = 6u64;
                            if nat_eq(a455, a456) {
                                Str::lit("nat_le")
                            } else {
                                let a457 = v0;
                                let a458 = 7u64;
                                if nat_eq(a457, a458) {
                                    Str::lit("nat_lt")
                                } else {
                                    let a459 = v0;
                                    let a460 = 8u64;
                                    if nat_eq(a459, a460) {
                                        Str::lit("nat_succ")
                                    } else {
                                        let a461 = v0;
                                        let a462 = 9u64;
                                        if nat_eq(a461, a462) {
                                            Str::lit("int_add")
                                        } else {
                                            let a463 = v0;
                                            let a464 = 10u64;
                                            if nat_eq(a463, a464) {
                                                Str::lit("int_sub")
                                            } else {
                                                let a465 = v0;
                                                let a466 = 11u64;
                                                if nat_eq(a465, a466) {
                                                    Str::lit("int_mul")
                                                } else {
                                                    let a467 = v0;
                                                    let a468 = 12u64;
                                                    if nat_eq(a467, a468) {
                                                        Str::lit("int_neg")
                                                    } else {
                                                        let a469 = v0;
                                                        let a470 = 13u64;
                                                        if nat_eq(a469, a470) {
                                                            Str::lit("int_quot")
                                                        } else {
                                                            let a471 = v0;
                                                            let a472 = 14u64;
                                                            if nat_eq(a471, a472) {
                                                                Str::lit("int_rem")
                                                            } else {
                                                                let a473 = v0;
                                                                let a474 = 15u64;
                                                                if nat_eq(a473, a474) {
                                                                    Str::lit("bool_not")
                                                                } else {
                                                                    let a475 = v0;
                                                                    let a476 = 16u64;
                                                                    if nat_eq(a475, a476) {
                                                                        Str::lit("bool_and")
                                                                    } else {
                                                                        let a477 = v0;
                                                                        let a478 = 17u64;
                                                                        if nat_eq(a477, a478) {
                                                                            Str::lit("bool_or")
                                                                        } else {
                                                                            let a479 = v0;
                                                                            let a480 = 18u64;
                                                                            if nat_eq(a479, a480) {
                                                                                Str::lit("equal")
                                                                            } else {
                                                                                let a481 = v0;
                                                                                let a482 = 19u64;
                                                                                if nat_eq(a481, a482) {
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
    let a483 = v0;
    let a484 = 32u64;
    if nat_eq(a483, a484) {
        Str::lit("append_list")
    } else {
        let a485 = v0;
        let a486 = 33u64;
        if nat_eq(a485, a486) {
            Str::lit("append_bytes")
        } else {
            let a487 = v0;
            let a488 = 34u64;
            if nat_eq(a487, a488) {
                Str::lit("length_list")
            } else {
                let a489 = v0;
                let a490 = 35u64;
                if nat_eq(a489, a490) {
                    Str::lit("length_bytes")
                } else {
                    let a491 = v0;
                    let a492 = 36u64;
                    if nat_eq(a491, a492) {
                        Str::lit("length_string")
                    } else {
                        let a493 = v0;
                        let a494 = 37u64;
                        if nat_eq(a493, a494) {
                            Str::lit("index_list")
                        } else {
                            let a495 = v0;
                            let a496 = 38u64;
                            if nat_eq(a495, a496) {
                                Str::lit("index_bytes")
                            } else {
                                let a497 = v0;
                                let a498 = 39u64;
                                if nat_eq(a497, a498) {
                                    Str::lit("slice_list")
                                } else {
                                    let a499 = v0;
                                    let a500 = 40u64;
                                    if nat_eq(a499, a500) {
                                        Str::lit("slice_bytes")
                                    } else {
                                        let a501 = v0;
                                        let a502 = 41u64;
                                        if nat_eq(a501, a502) {
                                            Str::lit("utf8_encode")
                                        } else {
                                            let a503 = v0;
                                            let a504 = 42u64;
                                            if nat_eq(a503, a504) {
                                                Str::lit("utf8_decode")
                                            } else {
                                                let a505 = v0;
                                                let a506 = 43u64;
                                                if nat_eq(a505, a506) {
                                                    Str::lit("compare_bytes")
                                                } else {
                                                    let a507 = v0;
                                                    let a508 = 44u64;
                                                    if nat_eq(a507, a508) {
                                                        Str::lit("split_exact")
                                                    } else {
                                                        let a509 = v0;
                                                        let a510 = 45u64;
                                                        if nat_eq(a509, a510) {
                                                            Str::lit("join")
                                                        } else {
                                                            let a511 = v0;
                                                            let a512 = 46u64;
                                                            if nat_eq(a511, a512) {
                                                                Str::lit("format_i64")
                                                            } else {
                                                                let a513 = v0;
                                                                let a514 = 48u64;
                                                                if nat_eq(a513, a514) {
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

pub fn f78(v0: u64) -> Str {
    let a515 = v0;
    let a516 = 0u64;
    if nat_eq(a515, a516) {
        Str::lit("checked_add")
    } else {
        let a517 = v0;
        let a518 = 1u64;
        if nat_eq(a517, a518) {
            Str::lit("checked_sub")
        } else {
            let a519 = v0;
            let a520 = 2u64;
            if nat_eq(a519, a520) {
                Str::lit("checked_mul")
            } else {
                let a521 = v0;
                let a522 = 3u64;
                if nat_eq(a521, a522) {
                    Str::lit("checked_div")
                } else {
                    let a523 = v0;
                    let a524 = 4u64;
                    if nat_eq(a523, a524) {
                        Str::lit("checked_neg")
                    } else {
                        let a525 = v0;
                        let a526 = 5u64;
                        if nat_eq(a525, a526) {
                            Str::lit("checked_shl")
                        } else {
                            let a527 = v0;
                            let a528 = 6u64;
                            if nat_eq(a527, a528) {
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
    let a529 = v0;
    let a530 = 8u64;
    if nat_eq(a529, a530) {
        Str::lit("core::ops::BitAnd::bitand(")
    } else {
        let a531 = v0;
        let a532 = 9u64;
        if nat_eq(a531, a532) {
            Str::lit("core::ops::BitOr::bitor(")
        } else {
            let a533 = v0;
            let a534 = 10u64;
            if nat_eq(a533, a534) {
                Str::lit("core::ops::BitXor::bitxor(")
            } else {
                let a535 = v0;
                let a536 = 11u64;
                if nat_eq(a535, a536) {
                    Str::lit("core::ops::Not::not(")
                } else {
                    let a537 = v0;
                    let a538 = 12u64;
                    if nat_eq(a537, a538) {
                        Str::lit("core::cmp::PartialEq::eq(&")
                    } else {
                        let a539 = v0;
                        let a540 = 13u64;
                        if nat_eq(a539, a540) {
                            Str::lit("core::cmp::PartialOrd::le(&")
                        } else {
                            let a541 = v0;
                            let a542 = 14u64;
                            if nat_eq(a541, a542) {
                                Str::lit("core::cmp::PartialOrd::lt(&")
                            } else {
                                let a543 = v0;
                                let a544 = 15u64;
                                if nat_eq(a543, a544) {
                                    Str::lit("u64::checked_rem(")
                                } else {
                                    let a545 = v0;
                                    let a546 = 16u64;
                                    if nat_eq(a545, a546) {
                                        Str::lit("{ let (a, b, z): (i64, i64, i64) = (")
                                    } else {
                                        let a547 = v0;
                                        let a548 = 17u64;
                                        if nat_eq(a547, a548) {
                                            Str::lit("u64::checked_add(")
                                        } else {
                                            let a549 = v0;
                                            let a550 = 18u64;
                                            if nat_eq(a549, a550) {
                                                Str::lit("u64::saturating_sub(")
                                            } else {
                                                let a551 = v0;
                                                let a552 = 19u64;
                                                if nat_eq(a551, a552) {
                                                    Str::lit("u64::checked_mul(")
                                                } else {
                                                    let a553 = v0;
                                                    let a554 = 20u64;
                                                    if nat_eq(a553, a554) {
                                                        Str::lit("u64::checked_div(")
                                                    } else {
                                                        let a555 = v0;
                                                        let a556 = 21u64;
                                                        if nat_eq(a555, a556) {
                                                            Str::lit("i64::checked_add(")
                                                        } else {
                                                            let a557 = v0;
                                                            let a558 = 22u64;
                                                            if nat_eq(a557, a558) {
                                                                Str::lit("i64::checked_sub(")
                                                            } else {
                                                                let a559 = v0;
                                                                let a560 = 23u64;
                                                                if nat_eq(a559, a560) {
                                                                    Str::lit("i64::checked_mul(")
                                                                } else {
                                                                    let a561 = v0;
                                                                    let a562 = 24u64;
                                                                    if nat_eq(a561, a562) {
                                                                        Str::lit("i64::checked_neg(")
                                                                    } else {
                                                                        let a563 = v0;
                                                                        let a564 = 25u64;
                                                                        if nat_eq(a563, a564) {
                                                                            Str::lit("{ let (a, b, z): (i64, i64, i64) = (")
                                                                        } else {
                                                                            let a565 = v0;
                                                                            let a566 = 26u64;
                                                                            if nat_eq(a565, a566) {
                                                                                Str::lit("List::length(&")
                                                                            } else {
                                                                                let a567 = v0;
                                                                                let a568 = 27u64;
                                                                                if nat_eq(a567, a568) {
                                                                                    Str::lit("List::append(")
                                                                                } else {
                                                                                    let a569 = v0;
                                                                                    let a570 = 28u64;
                                                                                    if nat_eq(a569, a570) {
                                                                                        Str::lit("List::index(&")
                                                                                    } else {
                                                                                        let a571 = v0;
                                                                                        let a572 = 29u64;
                                                                                        if nat_eq(a571, a572) {
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

pub fn f80(v0: u64) -> Str {
    let a573 = v0;
    let a574 = 16u64;
    if nat_eq(a573, a574) {
        Str::lit("); if b == 0 { z } else { a.wrapping_rem(b) } }")
    } else {
        let a575 = v0;
        let a576 = 25u64;
        if nat_eq(a575, a576) {
            Str::lit("); if b == 0 { Some(z) } else { a.checked_div(b) } }")
        } else {
            let a577 = v0;
            let a578 = 15u64;
            if nat_eq(a577, a578) {
                Str::lit(")")
            } else {
                let a579 = v0;
                let a580 = 20u64;
                if nat_eq(a579, a580) {
                    Str::lit(")")
                } else {
                    let a581 = v0;
                    let a582 = 8u64;
                    if nat_eq(a581, a582) {
                        Str::lit(")")
                    } else {
                        let a583 = v0;
                        let a584 = 9u64;
                        if nat_eq(a583, a584) {
                            Str::lit(")")
                        } else {
                            let a585 = v0;
                            let a586 = 10u64;
                            if nat_eq(a585, a586) {
                                Str::lit(")")
                            } else {
                                let a587 = v0;
                                let a588 = 11u64;
                                if nat_eq(a587, a588) {
                                    Str::lit(")")
                                } else {
                                    let a589 = v0;
                                    let a590 = 12u64;
                                    if nat_eq(a589, a590) {
                                        Str::lit(")")
                                    } else {
                                        let a591 = v0;
                                        let a592 = 13u64;
                                        if nat_eq(a591, a592) {
                                            Str::lit(")")
                                        } else {
                                            let a593 = v0;
                                            let a594 = 14u64;
                                            if nat_eq(a593, a594) {
                                                Str::lit(")")
                                            } else {
                                                let a595 = v0;
                                                let a596 = 17u64;
                                                if nat_eq(a595, a596) {
                                                    Str::lit(")")
                                                } else {
                                                    let a597 = v0;
                                                    let a598 = 18u64;
                                                    if nat_eq(a597, a598) {
                                                        Str::lit(")")
                                                    } else {
                                                        let a599 = v0;
                                                        let a600 = 19u64;
                                                        if nat_eq(a599, a600) {
                                                            Str::lit(")")
                                                        } else {
                                                            let a601 = v0;
                                                            let a602 = 21u64;
                                                            if nat_eq(a601, a602) {
                                                                Str::lit(")")
                                                            } else {
                                                                let a603 = v0;
                                                                let a604 = 22u64;
                                                                if nat_eq(a603, a604) {
                                                                    Str::lit(")")
                                                                } else {
                                                                    let a605 = v0;
                                                                    let a606 = 23u64;
                                                                    if nat_eq(a605, a606) {
                                                                        Str::lit(")")
                                                                    } else {
                                                                        let a607 = v0;
                                                                        let a608 = 24u64;
                                                                        let _ = nat_eq(a607, a608);
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
    let a609 = v0;
    let a610 = 12u64;
    if nat_eq(a609, a610) {
        Str::lit(", &")
    } else {
        let a611 = v0;
        let a612 = 13u64;
        if nat_eq(a611, a612) {
            Str::lit(", &")
        } else {
            let a613 = v0;
            let a614 = 14u64;
            if nat_eq(a613, a614) {
                Str::lit(", &")
            } else {
                let a615 = v0;
                let a616 = 8u64;
                if nat_eq(a615, a616) {
                    Str::lit(", ")
                } else {
                    let a617 = v0;
                    let a618 = 9u64;
                    if nat_eq(a617, a618) {
                        Str::lit(", ")
                    } else {
                        let a619 = v0;
                        let a620 = 10u64;
                        if nat_eq(a619, a620) {
                            Str::lit(", ")
                        } else {
                            let a621 = v0;
                            let a622 = 11u64;
                            if nat_eq(a621, a622) {
                                Str::lit(", ")
                            } else {
                                let a623 = v0;
                                let a624 = 15u64;
                                if nat_eq(a623, a624) {
                                    Str::lit(", ")
                                } else {
                                    let a625 = v0;
                                    let a626 = 16u64;
                                    if nat_eq(a625, a626) {
                                        Str::lit(", ")
                                    } else {
                                        let a627 = v0;
                                        let a628 = 17u64;
                                        if nat_eq(a627, a628) {
                                            Str::lit(", ")
                                        } else {
                                            let a629 = v0;
                                            let a630 = 18u64;
                                            if nat_eq(a629, a630) {
                                                Str::lit(", ")
                                            } else {
                                                let a631 = v0;
                                                let a632 = 19u64;
                                                if nat_eq(a631, a632) {
                                                    Str::lit(", ")
                                                } else {
                                                    let a633 = v0;
                                                    let a634 = 20u64;
                                                    if nat_eq(a633, a634) {
                                                        Str::lit(", ")
                                                    } else {
                                                        let a635 = v0;
                                                        let a636 = 21u64;
                                                        if nat_eq(a635, a636) {
                                                            Str::lit(", ")
                                                        } else {
                                                            let a637 = v0;
                                                            let a638 = 22u64;
                                                            if nat_eq(a637, a638) {
                                                                Str::lit(", ")
                                                            } else {
                                                                let a639 = v0;
                                                                let a640 = 23u64;
                                                                if nat_eq(a639, a640) {
                                                                    Str::lit(", ")
                                                                } else {
                                                                    let a641 = v0;
                                                                    let a642 = 24u64;
                                                                    if nat_eq(a641, a642) {
                                                                        Str::lit(", ")
                                                                    } else {
                                                                        let a643 = v0;
                                                                        let a644 = 25u64;
                                                                        let _ = nat_eq(a643, a644);
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

pub fn f83(v0: u64) -> Str {
    let a645 = v0;
    let a646 = 15u64;
    if nat_eq(a645, a646) {
        Str::lit(").unwrap_or(")
    } else {
        let a647 = v0;
        let a648 = 16u64;
        if nat_eq(a647, a648) {
            Str::lit(", ")
        } else {
            let a649 = v0;
            let a650 = 20u64;
            if nat_eq(a649, a650) {
                Str::lit(").unwrap_or(")
            } else {
                let a651 = v0;
                let a652 = 25u64;
                let _ = nat_eq(a651, a652);
                Str::lit(", ")
            }
        }
    }
}

pub fn f84(v0: Adt13, v1: Adt2, v2: u64, v3: (u64, (Adt26, bool))) -> R<Option<(u64, (Adt26, bool))>> {
    let a654 = {
        let (h653, _) = v3.clone();
        h653
    };
    let a655 = v2;
    if nat_lt(a654, a655) {
        Ok(Some(({
            let a657 = {
                let (h656, _) = v3.clone();
                h656
            };
            let a658 = 1u64;
            nat_add(a657, a658)?
        }, if if f93(v0, {
            let (h659, _) = v3.clone();
            h659
        }) {
            true
        } else {
            f94(v1, {
                let (h660, _) = v3.clone();
                h660
            })
        } {
            (f8(f35(f8({
                let (h661, _) = {
                    let (_, h662) = v3.clone();
                    h662
                };
                h661
            }, {
                let (_, h663) = {
                    let (_, h664) = v3;
                    h664
                };
                if h663 {
                    Str::lit(", ")
                } else {
                    Str::lit("<")
                }
            })?, {
                let (h665, _) = v3;
                h665
            })?, Str::lit(": Clone"))?, true)
        } else {
            let (_, h666) = v3;
            h666
        })))
    } else {
        Ok(None::<(u64, (Adt26, bool))>)
    }
}

pub fn f85(v0: (Option<Bytes>, List<Bytes>), v1: Bytes) -> (Option<Bytes>, List<Bytes>) {
    let m668 = {
        let (h667, _) = v0;
        h667
    };
    match m668 {
        None => {
            (Some(v1), {
                let (_, h669) = v0;
                h669
            })
        }
        Some(v2) => {
            (None::<Bytes>, List::cons({
                let a670 = v2;
                let a671 = v1;
                append_bytes(a670, a671)
            }, {
                let (_, h672) = v0;
                h672
            }))
        }
    }
}

pub fn f86(v0: u64, v1: u64) -> Bytes {
    f89(Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39]), {
        let a676 = {
            let a673 = v0;
            let a674 = v1;
            let a675 = 0u64;
            nat_quot(a673, a674, a675)
        };
        let a677 = 10u64;
        let a678 = 0u64;
        nat_rem(a676, a677, a678)
    }, 1u64)
}

pub fn f87(v0: Fn6, p683: u64, p684: u64) -> R<(u64, bool)> {
    let m682 = (p683, p684);
    {
        let mut m682 = m682;
        loop {
            let step = {
                let (v1, v2) = m682;
                let m679 = v1;
                if m679 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m679 - 1;
                    let m681 = {
                        let c680: Fn6 = v0.clone();
                        c680.apply(v2)?
                    };
                    match m681 {
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
                Ok(next) => { m682 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f88(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f95(v0, v1)
}

pub fn f89(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m688 = {
        let a685 = v0;
        let a686 = v1;
        let a687 = v2;
        slice_bytes(a685, a686, a687)
    };
    match m688 {
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
    let a690 = {
        let (h689, _) = v1.clone();
        h689
    };
    let a691 = v0;
    if nat_lt(a690, a691) {
        Ok(Some(({
            let a693 = {
                let (h692, _) = v1;
                h692
            };
            let a694 = 1u64;
            nat_add(a693, a694)?
        }, f8({
            let (_, h695) = v1;
            h695
        }, Str::lit("    "))?)))
    } else {
        Ok(None::<(u64, Adt26)>)
    }
}

pub fn f93(p699: Adt13, v1: u64) -> bool {
    let m698 = p699;
    {
        let mut m698 = m698;
        loop {
            let step = {
                let v0 = m698;
                let m696 = v0;
                match m696 {
                    Adt13::C0 => {
                        Err::<Adt13, bool>(false)
                    }
                    Adt13::C1(_, v3, r697) => {
                        let v4 = (*r697).clone();
                        if f94(v3, v1) {
                            Err::<Adt13, bool>(true)
                        } else {
                            Ok::<Adt13, bool>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m698 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f94(v0: Adt2, v1: u64) -> bool {
    let m700 = v0;
    match m700 {
        Adt2::C0(_) => {
            false
        }
        Adt2::C1(_) => {
            false
        }
        Adt2::C2(_, r704) => {
            let v7 = (*r704).clone();
            f94(v7, v1)
        }
        Adt2::C3(_, r705, r706) => {
            let v9 = (*r705).clone();
            let v10 = (*r706).clone();
            if f94(v9, v1) {
                true
            } else {
                f94(v10, v1)
            }
        }
        Adt2::C4(v2, v3) => {
            let m701 = v2;
            match m701 {
                Adt7::C0 => {
                    false
                }
                Adt7::C1 => {
                    false
                }
                Adt7::C2 => {
                    let a702 = v3;
                    let a703 = v1;
                    nat_eq(a702, a703)
                }
            }
        }
        Adt2::C5 => {
            false
        }
        Adt2::C6(r707, r708) => {
            let v11 = (*r707).clone();
            let v12 = (*r708).clone();
            if f94(v11, v1) {
                true
            } else {
                f94(v12, v1)
            }
        }
    }
}

pub fn f95(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m709 = f98(v0, v1);
    match m709 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            let a710 = v2;
            let a711 = 48u8;
            if equal(a710, a711) {
                Ok(Some({
                    let a712 = v1;
                    let a713 = 1u64;
                    nat_add(a712, a713)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f96(v0: Bytes, v1: (u64, Adt26)) -> R<Option<(u64, Adt26)>> {
    let a716 = {
        let (h714, _) = v1.clone();
        h714
    };
    let a717 = {
        let a715 = v0.clone();
        length_bytes(a715)
    };
    if nat_lt(a716, a717) {
        let v2: (u64, u64) = f99(v0.clone(), {
            let (h718, _) = v1.clone();
            h718
        })?;
        let v3: u64 = {
            let (h719, _) = v2;
            h719
        };
        Ok(Some(({
            let a722 = {
                let (h720, _) = v1.clone();
                h720
            };
            let a723 = {
                let (_, h721) = v2;
                h721
            };
            nat_add(a722, a723)?
        }, {
            let a724 = v3;
            let a725 = 34u64;
            if nat_eq(a724, a725) {
                f8({
                    let (_, h726) = v1;
                    h726
                }, Str::lit("\\\""))?
            } else {
                let a727 = v3;
                let a728 = 92u64;
                if nat_eq(a727, a728) {
                    f8({
                        let (_, h729) = v1;
                        h729
                    }, Str::lit("\\\\"))?
                } else {
                    let a730 = 32u64;
                    let a731 = v3;
                    if if nat_le(a730, a731) {
                        let a732 = v3;
                        let a733 = 126u64;
                        nat_le(a732, a733)
                    } else {
                        false
                    } {
                        f3({
                            let (_, h734) = v1;
                            h734
                        }, f89(v0, {
                            let (h735, _) = v1;
                            h735
                        }, 1u64))?
                    } else {
                        f8(f100(f8({
                            let (_, h736) = v1;
                            h736
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
    let m738 = f98(v0, {
        let (h737, _) = v1.clone();
        h737
    });
    match m738 {
        None => {
            Ok(None::<(u64, Adt26)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a740 = {
                    let (h739, _) = v1.clone();
                    h739
                };
                let a741 = 1u64;
                nat_add(a740, a741)?
            }, f101(f8({
                let a743 = {
                    let (h742, _) = v1;
                    h742
                };
                let a744 = 0u64;
                if nat_eq(a743, a744) {
                    let (_, h745) = v1;
                    h745
                } else {
                    f8({
                        let (_, h746) = v1;
                        h746
                    }, Str::lit(", "))?
                }
            }, Str::lit("0x"))?, v2)?)))
        }
    }
}

pub fn f98(v0: Bytes, v1: u64) -> Option<u8> {
    let a747 = v0;
    let a748 = v1;
    index_bytes(a747, a748)
}

pub fn f99(v0: Bytes, v1: u64) -> R<(u64, u64)> {
    let m749 = f98(v0.clone(), v1);
    match m749 {
        None => {
            Ok((0u64, 1u64))
        }
        Some(v2) => {
            let v4: u64 = {
                let v3: u8 = v2;
                let a794 = {
                    let a770 = {
                        let a758 = {
                            let a752 = {
                                let a750 = v3;
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
                                let a754 = v3;
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
                                let a760 = v3;
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
                                let a764 = v3;
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
                                let a772 = v3;
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
                                let a776 = v3;
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
                                let a782 = v3;
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
                                let a786 = v3;
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
            let a796 = v4;
            let a797 = 128u64;
            if nat_lt(a796, a797) {
                Ok((v4, 1u64))
            } else {
                let a798 = v4;
                let a799 = 224u64;
                if nat_lt(a798, a799) {
                    Ok(({
                        let a807 = {
                            let a803 = {
                                let a800 = v4;
                                let a801 = 32u64;
                                let a802 = 0u64;
                                nat_rem(a800, a801, a802)
                            };
                            let a804 = 64u64;
                            nat_mul(a803, a804)?
                        };
                        let a808 = f102(v0, {
                            let a805 = v1;
                            let a806 = 1u64;
                            nat_add(a805, a806)?
                        })?;
                        nat_add(a807, a808)?
                    }, 2u64))
                } else {
                    let a809 = v4;
                    let a810 = 240u64;
                    if nat_lt(a809, a810) {
                        Ok(({
                            let a824 = {
                                let a814 = {
                                    let a811 = v4;
                                    let a812 = 16u64;
                                    let a813 = 0u64;
                                    nat_rem(a811, a812, a813)
                                };
                                let a815 = 4096u64;
                                nat_mul(a814, a815)?
                            };
                            let a825 = {
                                let a822 = {
                                    let a818 = f102(v0.clone(), {
                                        let a816 = v1;
                                        let a817 = 1u64;
                                        nat_add(a816, a817)?
                                    })?;
                                    let a819 = 64u64;
                                    nat_mul(a818, a819)?
                                };
                                let a823 = f102(v0, {
                                    let a820 = v1;
                                    let a821 = 2u64;
                                    nat_add(a820, a821)?
                                })?;
                                nat_add(a822, a823)?
                            };
                            nat_add(a824, a825)?
                        }, 3u64))
                    } else {
                        Ok(({
                            let a845 = {
                                let a829 = {
                                    let a826 = v4;
                                    let a827 = 8u64;
                                    let a828 = 0u64;
                                    nat_rem(a826, a827, a828)
                                };
                                let a830 = 262144u64;
                                nat_mul(a829, a830)?
                            };
                            let a846 = {
                                let a843 = {
                                    let a833 = f102(v0.clone(), {
                                        let a831 = v1;
                                        let a832 = 1u64;
                                        nat_add(a831, a832)?
                                    })?;
                                    let a834 = 4096u64;
                                    nat_mul(a833, a834)?
                                };
                                let a844 = {
                                    let a841 = {
                                        let a837 = f102(v0.clone(), {
                                            let a835 = v1;
                                            let a836 = 2u64;
                                            nat_add(a835, a836)?
                                        })?;
                                        let a838 = 64u64;
                                        nat_mul(a837, a838)?
                                    };
                                    let a842 = f102(v0, {
                                        let a839 = v1;
                                        let a840 = 3u64;
                                        nat_add(a839, a840)?
                                    })?;
                                    nat_add(a841, a842)?
                                };
                                nat_add(a843, a844)?
                            };
                            nat_add(a845, a846)?
                        }, 4u64))
                    }
                }
            }
        }
    }
}

pub fn f100(v0: Adt26, v1: u64) -> R<Adt26> {
    let v2: Bytes = {
        let a859 = f103(v1, 268435456u64);
        let a860 = {
            let a857 = f103(v1, 16777216u64);
            let a858 = {
                let a855 = f103(v1, 1048576u64);
                let a856 = {
                    let a853 = f103(v1, 65536u64);
                    let a854 = {
                        let a851 = f103(v1, 4096u64);
                        let a852 = {
                            let a849 = f103(v1, 256u64);
                            let a850 = {
                                let a847 = f103(v1, 16u64);
                                let a848 = f103(v1, 1u64);
                                append_bytes(a847, a848)
                            };
                            append_bytes(a849, a850)
                        };
                        append_bytes(a851, a852)
                    };
                    append_bytes(a853, a854)
                };
                append_bytes(a855, a856)
            };
            append_bytes(a857, a858)
        };
        append_bytes(a859, a860)
    };
    let v3: u64 = {
        let (h861, _) = f87(Fn6::F104(v2.clone()), 7u64, 0u64)?;
        h861
    };
    f3(v0, f89(v2, v3, {
        let a862 = 8u64;
        let a863 = v3;
        nat_sub(a862, a863)
    }))
}

pub fn f101(v0: Adt26, v1: u8) -> R<Adt26> {
    let v3: u64 = {
        let v2: u8 = v1;
        let a908 = {
            let a884 = {
                let a872 = {
                    let a866 = {
                        let a864 = v2;
                        let a865 = 128u8;
                        fixed_u8::bit_and(a864, a865)
                    };
                    let a867 = 128u8;
                    if equal(a866, a867) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a873 = {
                    let a870 = {
                        let a868 = v2;
                        let a869 = 64u8;
                        fixed_u8::bit_and(a868, a869)
                    };
                    let a871 = 64u8;
                    if equal(a870, a871) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a872, a873)?
            };
            let a885 = {
                let a882 = {
                    let a876 = {
                        let a874 = v2;
                        let a875 = 32u8;
                        fixed_u8::bit_and(a874, a875)
                    };
                    let a877 = 32u8;
                    if equal(a876, a877) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a883 = {
                    let a880 = {
                        let a878 = v2;
                        let a879 = 16u8;
                        fixed_u8::bit_and(a878, a879)
                    };
                    let a881 = 16u8;
                    if equal(a880, a881) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a882, a883)?
            };
            nat_add(a884, a885)?
        };
        let a909 = {
            let a906 = {
                let a894 = {
                    let a888 = {
                        let a886 = v2;
                        let a887 = 8u8;
                        fixed_u8::bit_and(a886, a887)
                    };
                    let a889 = 8u8;
                    if equal(a888, a889) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a895 = {
                    let a892 = {
                        let a890 = v2;
                        let a891 = 4u8;
                        fixed_u8::bit_and(a890, a891)
                    };
                    let a893 = 4u8;
                    if equal(a892, a893) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a894, a895)?
            };
            let a907 = {
                let a904 = {
                    let a898 = {
                        let a896 = v2;
                        let a897 = 2u8;
                        fixed_u8::bit_and(a896, a897)
                    };
                    let a899 = 2u8;
                    if equal(a898, a899) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a905 = {
                    let a902 = {
                        let a900 = v2;
                        let a901 = 1u8;
                        fixed_u8::bit_and(a900, a901)
                    };
                    let a903 = 1u8;
                    if equal(a902, a903) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a904, a905)?
            };
            nat_add(a906, a907)?
        };
        nat_add(a908, a909)?
    };
    f3(v0, {
        let a910 = f103(v3, 16u64);
        let a911 = f103(v3, 1u64);
        append_bytes(a910, a911)
    })
}

pub fn f102(v0: Bytes, v1: u64) -> R<u64> {
    let m912 = f98(v0, v1);
    match m912 {
        None => {
            Ok(0u64)
        }
        Some(v2) => {
            let a959 = {
                let v3: u8 = v2;
                let a957 = {
                    let a933 = {
                        let a921 = {
                            let a915 = {
                                let a913 = v3;
                                let a914 = 128u8;
                                fixed_u8::bit_and(a913, a914)
                            };
                            let a916 = 128u8;
                            if equal(a915, a916) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a922 = {
                            let a919 = {
                                let a917 = v3;
                                let a918 = 64u8;
                                fixed_u8::bit_and(a917, a918)
                            };
                            let a920 = 64u8;
                            if equal(a919, a920) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a921, a922)?
                    };
                    let a934 = {
                        let a931 = {
                            let a925 = {
                                let a923 = v3;
                                let a924 = 32u8;
                                fixed_u8::bit_and(a923, a924)
                            };
                            let a926 = 32u8;
                            if equal(a925, a926) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a932 = {
                            let a929 = {
                                let a927 = v3;
                                let a928 = 16u8;
                                fixed_u8::bit_and(a927, a928)
                            };
                            let a930 = 16u8;
                            if equal(a929, a930) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a931, a932)?
                    };
                    nat_add(a933, a934)?
                };
                let a958 = {
                    let a955 = {
                        let a943 = {
                            let a937 = {
                                let a935 = v3;
                                let a936 = 8u8;
                                fixed_u8::bit_and(a935, a936)
                            };
                            let a938 = 8u8;
                            if equal(a937, a938) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a944 = {
                            let a941 = {
                                let a939 = v3;
                                let a940 = 4u8;
                                fixed_u8::bit_and(a939, a940)
                            };
                            let a942 = 4u8;
                            if equal(a941, a942) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a943, a944)?
                    };
                    let a956 = {
                        let a953 = {
                            let a947 = {
                                let a945 = v3;
                                let a946 = 2u8;
                                fixed_u8::bit_and(a945, a946)
                            };
                            let a948 = 2u8;
                            if equal(a947, a948) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a954 = {
                            let a951 = {
                                let a949 = v3;
                                let a950 = 1u8;
                                fixed_u8::bit_and(a949, a950)
                            };
                            let a952 = 1u8;
                            if equal(a951, a952) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a953, a954)?
                    };
                    nat_add(a955, a956)?
                };
                nat_add(a957, a958)?
            };
            let a960 = 64u64;
            let a961 = 0u64;
            Ok(nat_rem(a959, a960, a961))
        }
    }
}

pub fn f103(v0: u64, v1: u64) -> Bytes {
    f89(Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66]), {
        let a965 = {
            let a962 = v0;
            let a963 = v1;
            let a964 = 0u64;
            nat_quot(a962, a963, a964)
        };
        let a966 = 16u64;
        let a967 = 0u64;
        nat_rem(a965, a966, a967)
    }, 1u64)
}

pub fn f104(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f105(v0, v1)
}

pub fn f105(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m968 = f98(v0, v1);
    match m968 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            let a969 = v2;
            let a970 = 48u8;
            if equal(a969, a970) {
                Ok(Some({
                    let a971 = v1;
                    let a972 = 1u64;
                    nat_add(a971, a972)?
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
