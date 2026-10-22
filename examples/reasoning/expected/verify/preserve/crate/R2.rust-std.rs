#![forbid(unsafe_code)]
use core::cmp::Ordering;
use core::sync::atomic::{AtomicU64, Ordering as Memory};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Overflow;
pub type R<T> = Result<T, Overflow>;

static WORK: AtomicU64 = AtomicU64::new(0);
pub fn tick(units: u64) { WORK.fetch_add(units, Memory::Relaxed); }
pub fn work() -> u64 { WORK.load(Memory::Relaxed) }


pub trait Same { fn same(&self, other: &Self) -> bool; }
pub trait Key { fn key(&self, other: &Self) -> Ordering; }
macro_rules! scalar {
    ($($t:ty),*) => { $(
        impl Same for $t { fn same(&self, other: &Self) -> bool { self == other } }
        impl Key for $t { fn key(&self, other: &Self) -> Ordering { self.cmp(other) } }
    )* };
}
scalar!(bool, u8, u16, u32, u64, i8, i16, i32, i64);
impl Same for Ordering { fn same(&self, other: &Self) -> bool { self == other } }
impl<A: Key, B: Key> Key for (A, B) {
    fn key(&self, other: &Self) -> Ordering {
        match self.0.key(&other.0) { Ordering::Equal => self.1.key(&other.1), decided => decided }
    }
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

pub fn append_list<T: Clone>(a: List<T>, b: List<T>) -> List<T> { List::onto(a.items(), b) }
pub fn append_bytes(a: Bytes, b: Bytes) -> Bytes {
    tick((a.0.len() + b.0.len()) as u64);
    let mut out = Vec::with_capacity(a.0.len() + b.0.len());
    out.extend_from_slice(&a.0);
    out.extend_from_slice(&b.0);
    Bytes(std::rc::Rc::from(out))
}
pub fn length_list<T: Clone>(a: List<T>) -> u64 {
    let mut count = 0;
    a.walk(&mut |_| { count += 1; true });
    count
}
pub fn length_bytes(a: Bytes) -> u64 { a.0.len() as u64 }
pub fn length_string(a: Str) -> u64 { chars(&a.0) }
pub fn index_list<T: Clone>(a: List<T>, i: u64) -> Option<T> {
    let mut position = 0;
    let mut found = None;
    a.walk(&mut |item| { if position == i { found = Some(item.clone()); return false; } position += 1; true });
    found
}
pub fn index_bytes(a: Bytes, i: u64) -> Option<u8> { usize::try_from(i).ok().and_then(|i| a.0.get(i)).copied() }
pub fn slice_list<T: Clone>(a: List<T>, start: u64, count: u64) -> Option<List<T>> {
    let end = u128::from(start) + u128::from(count);
    let mut part = Vec::new();
    let mut position: u128 = 0;
    a.walk(&mut |item| {
        if position >= end { return false; }
        if position >= u128::from(start) { part.push(item.clone()); }
        position += 1;
        true
    });
    if position < end { None } else { Some(List::onto(part, List::nil())) }
}
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

#[derive(Clone)]
pub enum Adt0 {
    C0(u64, u64, u64, u64, u64, u64),
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
    C0(Adt3, List<Adt1>, Adt4),
}

#[derive(Clone)]
pub enum Adt3 {
    C0(Adt0, u64, u64, u64, u64, u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt4 {
    C0(u64, u64, u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Fn0 {
    F5,
}

impl Fn0 {
    pub fn apply(&self, p0: Adt2) -> R<Option<Adt2>> {
        match self {
            Fn0::F5 => f5(p0),
        }
    }
}

pub fn f0(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<u64> {
    let m1 = f1(Adt0::C0(v0, v1, v2, v3, v4, v5))?;
    match m1 {
        Ok(v6) => {
            let a3 = {
                let (_, h2) = v6;
                h2
            };
            Ok(length_list(a3))
        }
        Err(_) => {
            Ok(0u64)
        }
    }
}

pub fn f1(v0: Adt0) -> R<Result<(u64, List<Adt1>), (bool, bool)>> {
    let v1: (Adt2, bool) = f2(v0.clone())?;
    let m5 = {
        let (_, h4) = v1.clone();
        h4
    };
    if m5 {
        let m9 = f3(v0, {
            let m8 = {
                let (h7, _) = v1.clone();
                h7
            };
            match m8 {
                Adt2::C0(h6, _, _) => {
                    h6
                }
            }
        });
        match m9 {
            Ok(v2) => {
                Ok(Ok::<(u64, List<Adt1>), (bool, bool)>((v2, {
                    let m12 = {
                        let (h11, _) = v1;
                        h11
                    };
                    match m12 {
                        Adt2::C0(_, h10, _) => {
                            h10
                        }
                    }
                })))
            }
            Err(v3) => {
                Ok(Err::<(u64, List<Adt1>), (bool, bool)>(v3))
            }
        }
    } else {
        Ok(Err::<(u64, List<Adt1>), (bool, bool)>((false, false)))
    }
}

pub fn f2(v0: Adt0) -> R<(Adt2, bool)> {
    f4(Fn0::F5, 11u64, f6(v0))
}

pub fn f3(v0: Adt0, v1: Adt3) -> Result<u64, (bool, bool)> {
    let m13 = f7(v0, v1.clone());
    match m13 {
        None => {
            let m14 = f8(v1);
            match m14 {
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

pub fn f4(v0: Fn0, p19: u64, p20: Adt2) -> R<(Adt2, bool)> {
    let m18 = (p19, p20);
    {
        let mut m18 = m18;
        loop {
            let step = {
                let (v1, v2) = m18;
                let m15 = v1;
                if m15 == 0 {
                    Err::<(u64, Adt2), R<(Adt2, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m15 - 1;
                    let m17 = {
                        let c16: Fn0 = v0.clone();
                        c16.apply(v2.clone())?
                    };
                    match m17 {
                        None => {
                            Err::<(u64, Adt2), R<(Adt2, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt2), R<(Adt2, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m18 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f5(v0: Adt2) -> R<Option<Adt2>> {
    let m22 = f9(match v0.clone() {
        Adt2::C0(h21, _, _) => {
            h21
        }
    })?;
    match m22 {
        None => {
            Ok(None::<Adt2>)
        }
        Some(v1) => {
            let m24 = f10(match v0.clone() {
                Adt2::C0(h23, _, _) => {
                    h23
                }
            }, v1.clone())?;
            match m24 {
                None => {
                    Ok(None::<Adt2>)
                }
                Some(v2) => {
                    Ok(Some(Adt2::C0(v2, {
                        let a26 = match v0.clone() {
                            Adt2::C0(_, h25, _) => {
                                h25
                            }
                        };
                        let a27 = List::cons(v1, List::<Adt1>::nil());
                        append_list(a26, a27)
                    }, Adt4::C0({
                        let a31 = {
                            let m30 = match v0.clone() {
                                Adt2::C0(_, _, h29) => {
                                    h29
                                }
                            };
                            match m30 {
                                Adt4::C0(h28, _, _, _, _, _) => {
                                    h28
                                }
                            }
                        };
                        let a32 = 1u64;
                        nat_add(a31, a32)?
                    }, {
                        let a37 = {
                            let m35 = match v0.clone() {
                                Adt2::C0(_, _, h34) => {
                                    h34
                                }
                            };
                            match m35 {
                                Adt4::C0(_, h33, _, _, _, _) => {
                                    h33
                                }
                            }
                        };
                        let a38 = f11(match v0.clone() {
                            Adt2::C0(h36, _, _) => {
                                h36
                            }
                        })?;
                        nat_add(a37, a38)?
                    }, {
                        let a42 = {
                            let m41 = match v0.clone() {
                                Adt2::C0(_, _, h40) => {
                                    h40
                                }
                            };
                            match m41 {
                                Adt4::C0(_, _, h39, _, _, _) => {
                                    h39
                                }
                            }
                        };
                        let a43 = 1u64;
                        nat_add(a42, a43)?
                    }, {
                        let m46 = match v0.clone() {
                            Adt2::C0(_, _, h45) => {
                                h45
                            }
                        };
                        match m46 {
                            Adt4::C0(_, _, _, h44, _, _) => {
                                h44
                            }
                        }
                    }, {
                        let m49 = match v0.clone() {
                            Adt2::C0(_, _, h48) => {
                                h48
                            }
                        };
                        match m49 {
                            Adt4::C0(_, _, _, _, h47, _) => {
                                h47
                            }
                        }
                    }, {
                        let m52 = match v0 {
                            Adt2::C0(_, _, h51) => {
                                h51
                            }
                        };
                        match m52 {
                            Adt4::C0(_, _, _, _, _, h50) => {
                                h50
                            }
                        }
                    }))))
                }
            }
        }
    }
}

pub fn f6(v0: Adt0) -> Adt2 {
    Adt2::C0(f12(v0), List::<Adt1>::nil(), Adt4::C0(0u64, 0u64, 0u64, 0u64, 0u64, 0u64))
}

pub fn f7(v0: Adt0, v1: Adt3) -> Option<u64> {
    let m53 = f8(v1);
    match m53 {
        None => {
            None::<u64>
        }
        Some(v2) => {
            if f13(v0, v2) {
                Some(v2)
            } else {
                None::<u64>
            }
        }
    }
}

pub fn f8(v0: Adt3) -> Option<u64> {
    Some(match v0 {
        Adt3::C0(_, _, _, _, _, _, _, _, h54) => {
            h54
        }
    })
}

pub fn f9(v0: Adt3) -> R<Option<Adt1>> {
    let m58 = {
        let m56 = {
            let m55 = if f14(v0.clone()) {
                Some(Adt1::C0)
            } else {
                None::<Adt1>
            };
            match m55 {
                None => {
                    if f15(v0.clone()) {
                        Some(Adt1::C1)
                    } else {
                        None::<Adt1>
                    }
                }
                Some(v1) => {
                    Some(v1)
                }
            }
        };
        match m56 {
            None => {
                let m57 = if f16(v0.clone()) {
                    Some(Adt1::C2)
                } else {
                    None::<Adt1>
                };
                match m57 {
                    None => {
                        if f17(v0.clone()) {
                            Some(Adt1::C3)
                        } else {
                            None::<Adt1>
                        }
                    }
                    Some(v3) => {
                        Some(v3)
                    }
                }
            }
            Some(v2) => {
                Some(v2)
            }
        }
    };
    match m58 {
        None => {
            let m60 = {
                let m59 = if f18(v0.clone())? {
                    Some(Adt1::C4)
                } else {
                    None::<Adt1>
                };
                match m59 {
                    None => {
                        if f19(v0.clone()) {
                            Some(Adt1::C5)
                        } else {
                            None::<Adt1>
                        }
                    }
                    Some(v5) => {
                        Some(v5)
                    }
                }
            };
            match m60 {
                None => {
                    let m61 = if f20(v0.clone()) {
                        Some(Adt1::C6)
                    } else {
                        None::<Adt1>
                    };
                    match m61 {
                        None => {
                            if f21(v0) {
                                Ok(Some(Adt1::C7))
                            } else {
                                Ok(None::<Adt1>)
                            }
                        }
                        Some(v7) => {
                            Ok(Some(v7))
                        }
                    }
                }
                Some(v6) => {
                    Ok(Some(v6))
                }
            }
        }
        Some(v4) => {
            Ok(Some(v4))
        }
    }
}

pub fn f10(v0: Adt3, v1: Adt1) -> R<Option<Adt3>> {
    let m62 = v1;
    match m62 {
        Adt1::C0 => {
            Ok(f22(v0))
        }
        Adt1::C1 => {
            Ok(f23(v0))
        }
        Adt1::C2 => {
            Ok(f24(v0))
        }
        Adt1::C3 => {
            Ok(f25(v0))
        }
        Adt1::C4 => {
            f26(v0)
        }
        Adt1::C5 => {
            Ok(f27(v0))
        }
        Adt1::C6 => {
            Ok(f28(v0))
        }
        Adt1::C7 => {
            f29(v0)
        }
    }
}

pub fn f11(v0: Adt3) -> R<u64> {
    let (_, h63) = {
        let v7: (bool, u64) = {
            let v3: (bool, u64) = {
                let v1: (bool, u64) = (f14(v0.clone()), 1u64);
                let (h64, _) = v1;
                if h64 {
                    v1
                } else {
                    let v2: (bool, u64) = (f15(v0.clone()), 1u64);
                    ({
                        let (h65, _) = v2;
                        h65
                    }, {
                        let a68 = {
                            let (_, h66) = v1;
                            h66
                        };
                        let a69 = {
                            let (_, h67) = v2;
                            h67
                        };
                        nat_add(a68, a69)?
                    })
                }
            };
            let (h70, _) = v3;
            if h70 {
                v3
            } else {
                let v6: (bool, u64) = {
                    let v4: (bool, u64) = (f16(v0.clone()), 1u64);
                    let (h71, _) = v4;
                    if h71 {
                        v4
                    } else {
                        let v5: (bool, u64) = (f17(v0.clone()), 1u64);
                        ({
                            let (h72, _) = v5;
                            h72
                        }, {
                            let a75 = {
                                let (_, h73) = v4;
                                h73
                            };
                            let a76 = {
                                let (_, h74) = v5;
                                h74
                            };
                            nat_add(a75, a76)?
                        })
                    }
                };
                ({
                    let (h77, _) = v6;
                    h77
                }, {
                    let a80 = {
                        let (_, h78) = v3;
                        h78
                    };
                    let a81 = {
                        let (_, h79) = v6;
                        h79
                    };
                    nat_add(a80, a81)?
                })
            }
        };
        let (h82, _) = v7;
        if h82 {
            v7
        } else {
            let v14: (bool, u64) = {
                let v10: (bool, u64) = {
                    let v8: (bool, u64) = (f18(v0.clone())?, 1u64);
                    let (h83, _) = v8;
                    if h83 {
                        v8
                    } else {
                        let v9: (bool, u64) = (f19(v0.clone()), 1u64);
                        ({
                            let (h84, _) = v9;
                            h84
                        }, {
                            let a87 = {
                                let (_, h85) = v8;
                                h85
                            };
                            let a88 = {
                                let (_, h86) = v9;
                                h86
                            };
                            nat_add(a87, a88)?
                        })
                    }
                };
                let (h89, _) = v10;
                if h89 {
                    v10
                } else {
                    let v13: (bool, u64) = {
                        let v11: (bool, u64) = (f20(v0.clone()), 1u64);
                        let (h90, _) = v11;
                        if h90 {
                            v11
                        } else {
                            let v12: (bool, u64) = (f21(v0), 1u64);
                            ({
                                let (h91, _) = v12;
                                h91
                            }, {
                                let a94 = {
                                    let (_, h92) = v11;
                                    h92
                                };
                                let a95 = {
                                    let (_, h93) = v12;
                                    h93
                                };
                                nat_add(a94, a95)?
                            })
                        }
                    };
                    ({
                        let (h96, _) = v13;
                        h96
                    }, {
                        let a99 = {
                            let (_, h97) = v10;
                            h97
                        };
                        let a100 = {
                            let (_, h98) = v13;
                            h98
                        };
                        nat_add(a99, a100)?
                    })
                }
            };
            ({
                let (h101, _) = v14;
                h101
            }, {
                let a104 = {
                    let (_, h102) = v7;
                    h102
                };
                let a105 = {
                    let (_, h103) = v14;
                    h103
                };
                nat_add(a104, a105)?
            })
        }
    };
    Ok(h63)
}

pub fn f12(v0: Adt0) -> Adt3 {
    Adt3::C0(v0, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64)
}

pub fn f13(v0: Adt0, v1: u64) -> bool {
    let a106 = v1;
    let a107 = 3u64;
    if nat_le(a106, a107) {
        let a108 = v1;
        let a109 = 3u64;
        if nat_eq(a108, a109) {
            true
        } else {
            let a113 = if f30(v0.clone()) {
                let a111 = match v0.clone() {
                    Adt0::C0(_, _, _, _, _, h110) => {
                        h110
                    }
                };
                let a112 = 1u64;
                if nat_eq(a111, a112) {
                    if f31(v0.clone()) {
                        f32(v0)
                    } else {
                        false
                    }
                } else {
                    false
                }
            } else {
                false
            };
            bool_not(a113)
        }
    } else {
        false
    }
}

pub fn f14(v0: Adt3) -> bool {
    let a115 = match v0.clone() {
        Adt3::C0(_, h114, _, _, _, _, _, _, _) => {
            h114
        }
    };
    let a116 = 0u64;
    if nat_eq(a115, a116) {
        f31(match v0 {
            Adt3::C0(h117, _, _, _, _, _, _, _, _) => {
                h117
            }
        })
    } else {
        false
    }
}

pub fn f15(v0: Adt3) -> bool {
    let a119 = match v0.clone() {
        Adt3::C0(_, _, h118, _, _, _, _, _, _) => {
            h118
        }
    };
    let a120 = 0u64;
    if nat_eq(a119, a120) {
        f32(match v0 {
            Adt3::C0(h121, _, _, _, _, _, _, _, _) => {
                h121
            }
        })
    } else {
        false
    }
}

pub fn f16(v0: Adt3) -> bool {
    let a123 = match v0.clone() {
        Adt3::C0(_, _, _, h122, _, _, _, _, _) => {
            h122
        }
    };
    let a124 = 0u64;
    if nat_eq(a123, a124) {
        f33(match v0 {
            Adt3::C0(h125, _, _, _, _, _, _, _, _) => {
                h125
            }
        })
    } else {
        false
    }
}

pub fn f17(v0: Adt3) -> bool {
    let a127 = match v0.clone() {
        Adt3::C0(_, _, _, _, h126, _, _, _, _) => {
            h126
        }
    };
    let a128 = 0u64;
    if nat_eq(a127, a128) {
        f34(match v0 {
            Adt3::C0(h129, _, _, _, _, _, _, _, _) => {
                h129
            }
        })
    } else {
        false
    }
}

pub fn f18(v0: Adt3) -> R<bool> {
    let a131 = match v0.clone() {
        Adt3::C0(_, _, _, _, _, h130, _, _, _) => {
            h130
        }
    };
    let a132 = 0u64;
    if nat_eq(a131, a132) {
        let a143 = 2u64;
        let a144 = {
            let a141 = {
                let a138 = {
                    let a135 = match v0.clone() {
                        Adt3::C0(_, h133, _, _, _, _, _, _, _) => {
                            h133
                        }
                    };
                    let a136 = match v0.clone() {
                        Adt3::C0(_, _, h134, _, _, _, _, _, _) => {
                            h134
                        }
                    };
                    nat_add(a135, a136)?
                };
                let a139 = match v0.clone() {
                    Adt3::C0(_, _, _, h137, _, _, _, _, _) => {
                        h137
                    }
                };
                nat_add(a138, a139)?
            };
            let a142 = match v0 {
                Adt3::C0(_, _, _, _, h140, _, _, _, _) => {
                    h140
                }
            };
            nat_add(a141, a142)?
        };
        Ok(nat_le(a143, a144))
    } else {
        Ok(false)
    }
}

pub fn f19(v0: Adt3) -> bool {
    let a146 = match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, h145, _, _) => {
            h145
        }
    };
    let a147 = 0u64;
    if nat_eq(a146, a147) {
        let a149 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, h148, _, _, _) => {
                h148
            }
        };
        let a150 = 1u64;
        if nat_eq(a149, a150) {
            let a154 = {
                let m153 = match v0 {
                    Adt3::C0(h152, _, _, _, _, _, _, _, _) => {
                        h152
                    }
                };
                match m153 {
                    Adt0::C0(_, _, _, _, _, h151) => {
                        h151
                    }
                }
            };
            let a155 = 1u64;
            nat_eq(a154, a155)
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f20(v0: Adt3) -> bool {
    let a157 = match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, h156, _) => {
            h156
        }
    };
    let a158 = 0u64;
    if nat_eq(a157, a158) {
        let a160 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, h159, _, _) => {
                h159
            }
        };
        let a161 = 1u64;
        if nat_eq(a160, a161) {
            f30(match v0 {
                Adt3::C0(h162, _, _, _, _, _, _, _, _) => {
                    h162
                }
            })
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f21(v0: Adt3) -> bool {
    let a164 = match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, h163) => {
            h163
        }
    };
    let a165 = 3u64;
    if nat_lt(a164, a165) {
        let a167 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, h166) => {
                h166
            }
        };
        let a168 = 0u64;
        if if nat_eq(a167, a168) {
            let a170 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, h169, _, _, _) => {
                    h169
                }
            };
            let a171 = 1u64;
            nat_eq(a170, a171)
        } else {
            false
        } {
            true
        } else {
            let a173 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, h172) => {
                    h172
                }
            };
            let a174 = 1u64;
            if if nat_eq(a173, a174) {
                let a176 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h175, _, _) => {
                        h175
                    }
                };
                let a177 = 1u64;
                nat_eq(a176, a177)
            } else {
                false
            } {
                true
            } else {
                let a179 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, h178) => {
                        h178
                    }
                };
                let a180 = 2u64;
                if nat_eq(a179, a180) {
                    let a182 = match v0 {
                        Adt3::C0(_, _, _, _, _, _, _, h181, _) => {
                            h181
                        }
                    };
                    let a183 = 1u64;
                    nat_eq(a182, a183)
                } else {
                    false
                }
            }
        }
    } else {
        false
    }
}

pub fn f22(v0: Adt3) -> Option<Adt3> {
    if f14(v0.clone()) {
        Some(f35(v0))
    } else {
        None::<Adt3>
    }
}

pub fn f23(v0: Adt3) -> Option<Adt3> {
    if f15(v0.clone()) {
        Some(f36(v0))
    } else {
        None::<Adt3>
    }
}

pub fn f24(v0: Adt3) -> Option<Adt3> {
    if f16(v0.clone()) {
        Some(f37(v0))
    } else {
        None::<Adt3>
    }
}

pub fn f25(v0: Adt3) -> Option<Adt3> {
    if f17(v0.clone()) {
        Some(f38(v0))
    } else {
        None::<Adt3>
    }
}

pub fn f26(v0: Adt3) -> R<Option<Adt3>> {
    if f18(v0.clone())? {
        Ok(Some(f39(v0)))
    } else {
        Ok(None::<Adt3>)
    }
}

pub fn f27(v0: Adt3) -> Option<Adt3> {
    if f19(v0.clone()) {
        Some(f40(v0))
    } else {
        None::<Adt3>
    }
}

pub fn f28(v0: Adt3) -> Option<Adt3> {
    if f20(v0.clone()) {
        Some(f41(v0))
    } else {
        None::<Adt3>
    }
}

pub fn f29(v0: Adt3) -> R<Option<Adt3>> {
    if f21(v0.clone()) {
        Ok(Some(f42(v0)?))
    } else {
        Ok(None::<Adt3>)
    }
}

pub fn f30(v0: Adt0) -> bool {
    let a185 = match v0 {
        Adt0::C0(_, _, _, _, h184, _) => {
            h184
        }
    };
    let a186 = 90u64;
    nat_lt(a185, a186)
}

pub fn f31(v0: Adt0) -> bool {
    let a188 = 380u64;
    let a189 = match v0 {
        Adt0::C0(h187, _, _, _, _, _) => {
            h187
        }
    };
    nat_lt(a188, a189)
}

pub fn f32(v0: Adt0) -> bool {
    let a191 = 90u64;
    let a192 = match v0 {
        Adt0::C0(_, h190, _, _, _, _) => {
            h190
        }
    };
    nat_lt(a191, a192)
}

pub fn f33(v0: Adt0) -> bool {
    let a194 = 20u64;
    let a195 = match v0 {
        Adt0::C0(_, _, h193, _, _, _) => {
            h193
        }
    };
    nat_lt(a194, a195)
}

pub fn f34(v0: Adt0) -> bool {
    let a197 = 12u64;
    let a198 = match v0 {
        Adt0::C0(_, _, _, h196, _, _) => {
            h196
        }
    };
    nat_lt(a197, a198)
}

pub fn f35(v0: Adt3) -> Adt3 {
    Adt3::C0(match v0.clone() {
        Adt3::C0(h199, _, _, _, _, _, _, _, _) => {
            h199
        }
    }, 1u64, match v0.clone() {
        Adt3::C0(_, _, h200, _, _, _, _, _, _) => {
            h200
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, h201, _, _, _, _, _) => {
            h201
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, h202, _, _, _, _) => {
            h202
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, h203, _, _, _) => {
            h203
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, h204, _, _) => {
            h204
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, h205, _) => {
            h205
        }
    }, match v0 {
        Adt3::C0(_, _, _, _, _, _, _, _, h206) => {
            h206
        }
    })
}

pub fn f36(v0: Adt3) -> Adt3 {
    Adt3::C0(match v0.clone() {
        Adt3::C0(h207, _, _, _, _, _, _, _, _) => {
            h207
        }
    }, match v0.clone() {
        Adt3::C0(_, h208, _, _, _, _, _, _, _) => {
            h208
        }
    }, 1u64, match v0.clone() {
        Adt3::C0(_, _, _, h209, _, _, _, _, _) => {
            h209
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, h210, _, _, _, _) => {
            h210
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, h211, _, _, _) => {
            h211
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, h212, _, _) => {
            h212
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, h213, _) => {
            h213
        }
    }, match v0 {
        Adt3::C0(_, _, _, _, _, _, _, _, h214) => {
            h214
        }
    })
}

pub fn f37(v0: Adt3) -> Adt3 {
    Adt3::C0(match v0.clone() {
        Adt3::C0(h215, _, _, _, _, _, _, _, _) => {
            h215
        }
    }, match v0.clone() {
        Adt3::C0(_, h216, _, _, _, _, _, _, _) => {
            h216
        }
    }, match v0.clone() {
        Adt3::C0(_, _, h217, _, _, _, _, _, _) => {
            h217
        }
    }, 1u64, match v0.clone() {
        Adt3::C0(_, _, _, _, h218, _, _, _, _) => {
            h218
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, h219, _, _, _) => {
            h219
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, h220, _, _) => {
            h220
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, h221, _) => {
            h221
        }
    }, match v0 {
        Adt3::C0(_, _, _, _, _, _, _, _, h222) => {
            h222
        }
    })
}

pub fn f38(v0: Adt3) -> Adt3 {
    Adt3::C0(match v0.clone() {
        Adt3::C0(h223, _, _, _, _, _, _, _, _) => {
            h223
        }
    }, match v0.clone() {
        Adt3::C0(_, h224, _, _, _, _, _, _, _) => {
            h224
        }
    }, match v0.clone() {
        Adt3::C0(_, _, h225, _, _, _, _, _, _) => {
            h225
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, h226, _, _, _, _, _) => {
            h226
        }
    }, 1u64, match v0.clone() {
        Adt3::C0(_, _, _, _, _, h227, _, _, _) => {
            h227
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, h228, _, _) => {
            h228
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, h229, _) => {
            h229
        }
    }, match v0 {
        Adt3::C0(_, _, _, _, _, _, _, _, h230) => {
            h230
        }
    })
}

pub fn f39(v0: Adt3) -> Adt3 {
    Adt3::C0(match v0.clone() {
        Adt3::C0(h231, _, _, _, _, _, _, _, _) => {
            h231
        }
    }, match v0.clone() {
        Adt3::C0(_, h232, _, _, _, _, _, _, _) => {
            h232
        }
    }, match v0.clone() {
        Adt3::C0(_, _, h233, _, _, _, _, _, _) => {
            h233
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, h234, _, _, _, _, _) => {
            h234
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, h235, _, _, _, _) => {
            h235
        }
    }, 1u64, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, h236, _, _) => {
            h236
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, h237, _) => {
            h237
        }
    }, match v0 {
        Adt3::C0(_, _, _, _, _, _, _, _, h238) => {
            h238
        }
    })
}

pub fn f40(v0: Adt3) -> Adt3 {
    Adt3::C0(match v0.clone() {
        Adt3::C0(h239, _, _, _, _, _, _, _, _) => {
            h239
        }
    }, match v0.clone() {
        Adt3::C0(_, h240, _, _, _, _, _, _, _) => {
            h240
        }
    }, match v0.clone() {
        Adt3::C0(_, _, h241, _, _, _, _, _, _) => {
            h241
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, h242, _, _, _, _, _) => {
            h242
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, h243, _, _, _, _) => {
            h243
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, h244, _, _, _) => {
            h244
        }
    }, 1u64, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, h245, _) => {
            h245
        }
    }, match v0 {
        Adt3::C0(_, _, _, _, _, _, _, _, h246) => {
            h246
        }
    })
}

pub fn f41(v0: Adt3) -> Adt3 {
    Adt3::C0(match v0.clone() {
        Adt3::C0(h247, _, _, _, _, _, _, _, _) => {
            h247
        }
    }, match v0.clone() {
        Adt3::C0(_, h248, _, _, _, _, _, _, _) => {
            h248
        }
    }, match v0.clone() {
        Adt3::C0(_, _, h249, _, _, _, _, _, _) => {
            h249
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, h250, _, _, _, _, _) => {
            h250
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, h251, _, _, _, _) => {
            h251
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, h252, _, _, _) => {
            h252
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, h253, _, _) => {
            h253
        }
    }, 1u64, match v0 {
        Adt3::C0(_, _, _, _, _, _, _, _, h254) => {
            h254
        }
    })
}

pub fn f42(v0: Adt3) -> R<Adt3> {
    Ok(Adt3::C0(match v0.clone() {
        Adt3::C0(h255, _, _, _, _, _, _, _, _) => {
            h255
        }
    }, match v0.clone() {
        Adt3::C0(_, h256, _, _, _, _, _, _, _) => {
            h256
        }
    }, match v0.clone() {
        Adt3::C0(_, _, h257, _, _, _, _, _, _) => {
            h257
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, h258, _, _, _, _, _) => {
            h258
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, h259, _, _, _, _) => {
            h259
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, h260, _, _, _) => {
            h260
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, h261, _, _) => {
            h261
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, h262, _) => {
            h262
        }
    }, {
        let a264 = match v0 {
            Adt3::C0(_, _, _, _, _, _, _, _, h263) => {
                h263
            }
        };
        let a265 = 1u64;
        nat_add(a264, a265)?
    }))
}
