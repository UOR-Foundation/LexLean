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
    C0,
    C1,
    C2(u64),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(List<Adt2>, List<(u64, u64)>, Option<((u64, u64), Adt2)>, bool, Adt3),
}

#[derive(Clone)]
pub enum Adt2 {
    C0((u64, u64), List<Adt0>),
}

#[derive(Clone)]
pub enum Adt3 {
    C0(u64, u64, u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Fn0 {
    F6(u64),
}

impl Fn0 {
    pub fn apply(&self, p0: Adt1) -> R<Option<Adt1>> {
        match self {
            Fn0::F6(k0) => f6(*k0, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F10(u64),
    F16(u64),
}

impl Fn1 {
    pub fn apply(&self, p0: List<Adt2>, p1: Adt2) -> R<List<Adt2>> {
        match self {
            Fn1::F10(k0) => f10(*k0, p0, p1),
            Fn1::F16(k0) => f16(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F24,
}

impl Fn2 {
    pub fn apply(&self, p0: (List<Adt2>, List<(u64, u64)>), p1: Adt2) -> (List<Adt2>, List<(u64, u64)>) {
        match self {
            Fn2::F24 => f24(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F28(Adt2),
}

impl Fn3 {
    pub fn apply(&self, p0: List<Adt2>, p1: u64) -> R<List<Adt2>> {
        match self {
            Fn3::F28(k0) => f28(k0.clone(), p0, p1),
        }
    }
}

pub fn f0(v0: u64) -> R<u64> {
    let m1 = f1(v0)?;
    match m1 {
        Ok(v1) => {
            let (h2, _) = v1;
            Ok(h2)
        }
        Err(_) => {
            Ok(9u64)
        }
    }
}

pub fn f1(v0: u64) -> R<Result<(u64, u64), (bool, bool)>> {
    let m3 = f2(v0)?;
    match m3 {
        Ok(v1) => {
            Ok(Ok::<(u64, u64), (bool, bool)>({
                let (h4, _) = v1;
                h4
            }))
        }
        Err(v2) => {
            Ok(Err::<(u64, u64), (bool, bool)>(v2))
        }
    }
}

pub fn f2(v0: u64) -> R<Result<((u64, u64), List<Adt0>), (bool, bool)>> {
    let v1: (Adt1, bool) = f3(v0)?;
    let m8 = {
        let m7 = {
            let (h6, _) = v1.clone();
            h6
        };
        match m7 {
            Adt1::C0(_, _, h5, _, _) => {
                h5
            }
        }
    };
    match m8 {
        None => {
            Ok(Err::<((u64, u64), List<Adt0>), (bool, bool)>(f4({
                let (_, h13) = v1;
                h13
            }, {
                let m16 = {
                    let (h15, _) = v1;
                    h15
                };
                match m16 {
                    Adt1::C0(_, _, _, h14, _) => {
                        h14
                    }
                }
            })))
        }
        Some(v2) => {
            Ok(Ok::<((u64, u64), List<Adt0>), (bool, bool)>(({
                let (h9, _) = v2;
                h9
            }, {
                let m12 = {
                    let (_, h11) = v2;
                    h11
                };
                match m12 {
                    Adt2::C0(_, h10) => {
                        h10
                    }
                }
            })))
        }
    }
}

pub fn f3(v0: u64) -> R<(Adt1, bool)> {
    f5(Fn0::F6(v0), {
        let a17 = v0;
        let a18 = 8u64;
        nat_mul(a17, a18)?
    }, f7(v0)?)
}

pub fn f4(v0: bool, v1: bool) -> (bool, bool) {
    if if v0 {
        let a19 = v1;
        bool_not(a19)
    } else {
        false
    } {
        (false, true)
    } else {
        (false, false)
    }
}

pub fn f5(v0: Fn0, p24: u64, p25: Adt1) -> R<(Adt1, bool)> {
    let m23 = (p24, p25);
    {
        let mut m23 = m23;
        loop {
            let step = {
                let (v1, v2) = m23;
                let m20 = v1;
                if m20 == 0 {
                    Err::<(u64, Adt1), R<(Adt1, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m20 - 1;
                    let m22 = {
                        let c21: Fn0 = v0.clone();
                        c21.apply(v2.clone())?
                    };
                    match m22 {
                        None => {
                            Err::<(u64, Adt1), R<(Adt1, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt1), R<(Adt1, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m23 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f6(v0: u64, v1: Adt1) -> R<Option<Adt1>> {
    f8(v0, v1)
}

pub fn f7(v0: u64) -> R<Adt1> {
    let v1: List<Adt2> = f9(Fn1::F10(v0), List::<Adt2>::nil(), List::cons(Adt2::C0(f11(v0), List::<Adt0>::nil()), List::<Adt2>::nil()))?;
    Ok(Adt1::C0(v1.clone(), f12(List::<(u64, u64)>::nil(), f11(v0)), None::<((u64, u64), Adt2)>, {
        let a28 = {
            let a26 = v0;
            let a27 = v0;
            nat_add(a26, a27)?
        };
        let a29 = 1u64;
        nat_lt(a28, a29)
    }, Adt3::C0(0u64, 0u64, 0u64, 0u64, 0u64, {
        let a30 = v1;
        length_list(a30)
    })))
}

pub fn f8(v0: u64, v1: Adt1) -> R<Option<Adt1>> {
    let m32 = match v1.clone() {
        Adt1::C0(_, _, h31, _, _) => {
            h31
        }
    };
    match m32 {
        None => {
            let m34 = match v1.clone() {
                Adt1::C0(h33, _, _, _, _) => {
                    h33
                }
            };
            match m34.uncons() {
                None => {
                    Ok(None::<Adt1>)
                }
                Some((v3, v4)) => {
                    let m36 = f13(v0, match v3.clone() {
                        Adt2::C0(h35, _) => {
                            h35
                        }
                    });
                    match m36 {
                        None => {
                            let v6: List<Adt2> = f14(v3.clone())?;
                            let v7: (List<Adt2>, List<(u64, u64)>) = f15(match v1.clone() {
                                Adt1::C0(_, h61, _, _, _) => {
                                    h61
                                }
                            }, v6.clone());
                            let v8: List<Adt2> = {
                                let a63 = v4;
                                let a64 = {
                                    let (h62, _) = v7;
                                    h62
                                };
                                append_list(a63, a64)
                            };
                            let v9: List<Adt2> = f9(Fn1::F16(v0), List::<Adt2>::nil(), v8.clone())?;
                            Ok(Some(Adt1::C0(v9.clone(), {
                                let (_, h65) = v7;
                                h65
                            }, None::<((u64, u64), Adt2)>, if match v1.clone() {
                                Adt1::C0(_, _, _, h66, _) => {
                                    h66
                                }
                            } {
                                true
                            } else {
                                let a70 = {
                                    let a67 = v0;
                                    let a68 = v0;
                                    nat_add(a67, a68)?
                                };
                                let a71 = {
                                    let a69 = v8;
                                    length_list(a69)
                                };
                                nat_lt(a70, a71)
                            }, Adt3::C0({
                                let a75 = {
                                    let m74 = match v1.clone() {
                                        Adt1::C0(_, _, _, _, h73) => {
                                            h73
                                        }
                                    };
                                    match m74 {
                                        Adt3::C0(h72, _, _, _, _, _) => {
                                            h72
                                        }
                                    }
                                };
                                let a76 = 1u64;
                                nat_add(a75, a76)?
                            }, {
                                let a81 = {
                                    let m79 = match v1.clone() {
                                        Adt1::C0(_, _, _, _, h78) => {
                                            h78
                                        }
                                    };
                                    match m79 {
                                        Adt3::C0(_, h77, _, _, _, _) => {
                                            h77
                                        }
                                    }
                                };
                                let a82 = f17(match v3 {
                                    Adt2::C0(h80, _) => {
                                        h80
                                    }
                                })?;
                                nat_add(a81, a82)?
                            }, {
                                let a87 = {
                                    let m85 = match v1.clone() {
                                        Adt1::C0(_, _, _, _, h84) => {
                                            h84
                                        }
                                    };
                                    match m85 {
                                        Adt3::C0(_, _, h83, _, _, _) => {
                                            h83
                                        }
                                    }
                                };
                                let a88 = {
                                    let a86 = v6;
                                    length_list(a86)
                                };
                                nat_add(a87, a88)?
                            }, {
                                let a92 = {
                                    let m91 = match v1.clone() {
                                        Adt1::C0(_, _, _, _, h90) => {
                                            h90
                                        }
                                    };
                                    match m91 {
                                        Adt3::C0(_, _, _, h89, _, _) => {
                                            h89
                                        }
                                    }
                                };
                                let a93 = 1u64;
                                nat_add(a92, a93)?
                            }, {
                                let a97 = {
                                    let m96 = match v1.clone() {
                                        Adt1::C0(_, _, _, _, h95) => {
                                            h95
                                        }
                                    };
                                    match m96 {
                                        Adt3::C0(_, _, _, _, h94, _) => {
                                            h94
                                        }
                                    }
                                };
                                let a98 = 1u64;
                                nat_add(a97, a98)?
                            }, {
                                let a103 = {
                                    let m101 = match v1.clone() {
                                        Adt1::C0(_, _, _, _, h100) => {
                                            h100
                                        }
                                    };
                                    match m101 {
                                        Adt3::C0(_, _, _, _, _, h99) => {
                                            h99
                                        }
                                    }
                                };
                                let a104 = {
                                    let a102 = v9.clone();
                                    length_list(a102)
                                };
                                if nat_lt(a103, a104) {
                                    let a105 = v9;
                                    length_list(a105)
                                } else {
                                    let m108 = match v1 {
                                        Adt1::C0(_, _, _, _, h107) => {
                                            h107
                                        }
                                    };
                                    match m108 {
                                        Adt3::C0(_, _, _, _, _, h106) => {
                                            h106
                                        }
                                    }
                                }
                            }))))
                        }
                        Some(v5) => {
                            Ok(Some(Adt1::C0(v4, match v1.clone() {
                                Adt1::C0(_, h37, _, _, _) => {
                                    h37
                                }
                            }, Some((v5, v3)), match v1.clone() {
                                Adt1::C0(_, _, _, h38, _) => {
                                    h38
                                }
                            }, Adt3::C0({
                                let a42 = {
                                    let m41 = match v1.clone() {
                                        Adt1::C0(_, _, _, _, h40) => {
                                            h40
                                        }
                                    };
                                    match m41 {
                                        Adt3::C0(h39, _, _, _, _, _) => {
                                            h39
                                        }
                                    }
                                };
                                let a43 = 1u64;
                                nat_add(a42, a43)?
                            }, {
                                let m46 = match v1.clone() {
                                    Adt1::C0(_, _, _, _, h45) => {
                                        h45
                                    }
                                };
                                match m46 {
                                    Adt3::C0(_, h44, _, _, _, _) => {
                                        h44
                                    }
                                }
                            }, {
                                let m49 = match v1.clone() {
                                    Adt1::C0(_, _, _, _, h48) => {
                                        h48
                                    }
                                };
                                match m49 {
                                    Adt3::C0(_, _, h47, _, _, _) => {
                                        h47
                                    }
                                }
                            }, {
                                let m52 = match v1.clone() {
                                    Adt1::C0(_, _, _, _, h51) => {
                                        h51
                                    }
                                };
                                match m52 {
                                    Adt3::C0(_, _, _, h50, _, _) => {
                                        h50
                                    }
                                }
                            }, {
                                let a56 = {
                                    let m55 = match v1.clone() {
                                        Adt1::C0(_, _, _, _, h54) => {
                                            h54
                                        }
                                    };
                                    match m55 {
                                        Adt3::C0(_, _, _, _, h53, _) => {
                                            h53
                                        }
                                    }
                                };
                                let a57 = 1u64;
                                nat_add(a56, a57)?
                            }, {
                                let m60 = match v1 {
                                    Adt1::C0(_, _, _, _, h59) => {
                                        h59
                                    }
                                };
                                match m60 {
                                    Adt3::C0(_, _, _, _, _, h58) => {
                                        h58
                                    }
                                }
                            }))))
                        }
                    }
                }
            }
        }
        Some(_) => {
            Ok(None::<Adt1>)
        }
    }
}

pub fn f9(v0: Fn1, p112: List<Adt2>, p113: List<Adt2>) -> R<List<Adt2>> {
    let m111 = (p112, p113);
    {
        let mut m111 = m111;
        loop {
            let step = {
                let (v1, v2) = m111;
                let m109 = v2;
                match m109.uncons() {
                    None => {
                        Err::<(List<Adt2>, List<Adt2>), R<List<Adt2>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt2>, List<Adt2>), R<List<Adt2>>>(({
                            let c110: Fn1 = v0.clone();
                            c110.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m111 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f10(v0: u64, v1: List<Adt2>, v2: Adt2) -> R<List<Adt2>> {
    let a117 = {
        let a114 = v1.clone();
        length_list(a114)
    };
    let a118 = {
        let a115 = v0;
        let a116 = v0;
        nat_add(a115, a116)?
    };
    if nat_lt(a117, a118) {
        let a119 = v1;
        let a120 = List::cons(v2, List::<Adt2>::nil());
        Ok(append_list(a119, a120))
    } else {
        Ok(v1)
    }
}

pub fn f11(_: u64) -> (u64, u64) {
    (0u64, 0u64)
}

pub fn f12(v0: List<(u64, u64)>, v1: (u64, u64)) -> List<(u64, u64)> {
    let m121 = v0.clone();
    match m121.uncons() {
        None => {
            List::cons(v1, List::<(u64, u64)>::nil())
        }
        Some((v2, v3)) => {
            let m124 = {
                let a122 = v1;
                let a123 = v2;
                compare(a122, a123)
            };
            match m124 {
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

pub fn f13(v0: u64, v1: (u64, u64)) -> Option<(u64, u64)> {
    let m125 = f18(v1);
    match m125 {
        None => {
            None::<(u64, u64)>
        }
        Some(v2) => {
            if f19(v0, v2) {
                Some(v2)
            } else {
                None::<(u64, u64)>
            }
        }
    }
}

pub fn f14(v0: Adt2) -> R<List<Adt2>> {
    let a128 = f20(v0.clone())?;
    let a129 = {
        let a126 = f21(v0.clone())?;
        let a127 = f22(v0)?;
        append_list(a126, a127)
    };
    Ok(append_list(a128, a129))
}

pub fn f15(v0: List<(u64, u64)>, v1: List<Adt2>) -> (List<Adt2>, List<(u64, u64)>) {
    f23(Fn2::F24, (List::<Adt2>::nil(), v0), v1)
}

pub fn f16(v0: u64, v1: List<Adt2>, v2: Adt2) -> R<List<Adt2>> {
    let a133 = {
        let a130 = v1.clone();
        length_list(a130)
    };
    let a134 = {
        let a131 = v0;
        let a132 = v0;
        nat_add(a131, a132)?
    };
    if nat_lt(a133, a134) {
        let a135 = v1;
        let a136 = List::cons(v2, List::<Adt2>::nil());
        Ok(append_list(a135, a136))
    } else {
        Ok(v1)
    }
}

pub fn f17(v0: (u64, u64)) -> R<u64> {
    let a140 = 1u64;
    let a141 = {
        let a138 = 1u64;
        let a139 = {
            let a137 = f25(v0);
            length_list(a137)
        };
        nat_add(a138, a139)?
    };
    nat_add(a140, a141)
}

pub fn f18(v0: (u64, u64)) -> Option<(u64, u64)> {
    Some(v0)
}

pub fn f19(v0: u64, v1: (u64, u64)) -> bool {
    let a143 = {
        let (_, h142) = v1;
        h142
    };
    let a144 = v0;
    if nat_eq(a143, a144) {
        let a146 = {
            let (h145, _) = v1;
            h145
        };
        let a147 = 4u64;
        nat_le(a146, a147)
    } else {
        false
    }
}

pub fn f20(v0: Adt2) -> R<List<Adt2>> {
    let m149 = f26(match v0.clone() {
        Adt2::C0(h148, _) => {
            h148
        }
    }, Adt0::C0)?;
    match m149 {
        None => {
            Ok(List::<Adt2>::nil())
        }
        Some(v1) => {
            Ok(List::cons(Adt2::C0(v1, {
                let a151 = match v0 {
                    Adt2::C0(_, h150) => {
                        h150
                    }
                };
                let a152 = List::cons(Adt0::C0, List::<Adt0>::nil());
                append_list(a151, a152)
            }), List::<Adt2>::nil()))
        }
    }
}

pub fn f21(v0: Adt2) -> R<List<Adt2>> {
    let m154 = f26(match v0.clone() {
        Adt2::C0(h153, _) => {
            h153
        }
    }, Adt0::C1)?;
    match m154 {
        None => {
            Ok(List::<Adt2>::nil())
        }
        Some(v1) => {
            Ok(List::cons(Adt2::C0(v1, {
                let a156 = match v0 {
                    Adt2::C0(_, h155) => {
                        h155
                    }
                };
                let a157 = List::cons(Adt0::C1, List::<Adt0>::nil());
                append_list(a156, a157)
            }), List::<Adt2>::nil()))
        }
    }
}

pub fn f22(v0: Adt2) -> R<List<Adt2>> {
    f27(Fn3::F28(v0.clone()), List::<Adt2>::nil(), f25(match v0 {
        Adt2::C0(h158, _) => {
            h158
        }
    }))
}

pub fn f23(v0: Fn2, p162: (List<Adt2>, List<(u64, u64)>), p163: List<Adt2>) -> (List<Adt2>, List<(u64, u64)>) {
    let m161 = (p162, p163);
    {
        let mut m161 = m161;
        loop {
            let step = {
                let (v1, v2) = m161;
                let m159 = v2;
                match m159.uncons() {
                    None => {
                        Err::<((List<Adt2>, List<(u64, u64)>), List<Adt2>), (List<Adt2>, List<(u64, u64)>)>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<((List<Adt2>, List<(u64, u64)>), List<Adt2>), (List<Adt2>, List<(u64, u64)>)>(({
                            let c160: Fn2 = v0.clone();
                            c160.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m161 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f24(v0: (List<Adt2>, List<(u64, u64)>), v1: Adt2) -> (List<Adt2>, List<(u64, u64)>) {
    if f29({
        let (_, h164) = v0.clone();
        h164
    }, match v1.clone() {
        Adt2::C0(h165, _) => {
            h165
        }
    }) {
        v0
    } else {
        ({
            let a167 = {
                let (h166, _) = v0;
                h166
            };
            let a168 = List::cons(v1.clone(), List::<Adt2>::nil());
            append_list(a167, a168)
        }, f12({
            let (_, h169) = v0;
            h169
        }, match v1 {
            Adt2::C0(h170, _) => {
                h170
            }
        }))
    }
}

pub fn f25(_: (u64, u64)) -> List<u64> {
    List::cons(1u64, List::cons(2u64, List::cons(3u64, List::<u64>::nil())))
}

pub fn f26(v0: (u64, u64), v1: Adt0) -> R<Option<(u64, u64)>> {
    let m171 = v1;
    match m171 {
        Adt0::C0 => {
            Ok(f30(v0))
        }
        Adt0::C1 => {
            Ok(f31(v0))
        }
        Adt0::C2(v2) => {
            f32(v0, v2)
        }
    }
}

pub fn f27(v0: Fn3, p175: List<Adt2>, p176: List<u64>) -> R<List<Adt2>> {
    let m174 = (p175, p176);
    {
        let mut m174 = m174;
        loop {
            let step = {
                let (v1, v2) = m174;
                let m172 = v2;
                match m172.uncons() {
                    None => {
                        Err::<(List<Adt2>, List<u64>), R<List<Adt2>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt2>, List<u64>), R<List<Adt2>>>(({
                            let c173: Fn3 = v0.clone();
                            c173.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m174 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f28(v0: Adt2, v1: List<Adt2>, v2: u64) -> R<List<Adt2>> {
    f33(v0, v1, v2)
}

pub fn f29(p182: List<(u64, u64)>, v1: (u64, u64)) -> bool {
    let m181 = p182;
    {
        let mut m181 = m181;
        loop {
            let step = {
                let v0 = m181;
                let m177 = v0;
                match m177.uncons() {
                    None => {
                        Err::<List<(u64, u64)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let m180 = {
                            let a178 = v1;
                            let a179 = v2;
                            compare(a178, a179)
                        };
                        match m180 {
                            Ordering::Less => {
                                Err::<List<(u64, u64)>, bool>(false)
                            }
                            Ordering::Equal => {
                                Err::<List<(u64, u64)>, bool>(true)
                            }
                            Ordering::Greater => {
                                Ok::<List<(u64, u64)>, bool>(v3)
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m181 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f30(v0: (u64, u64)) -> Option<(u64, u64)> {
    if f34(v0) {
        Some(f35(v0))
    } else {
        None::<(u64, u64)>
    }
}

pub fn f31(v0: (u64, u64)) -> Option<(u64, u64)> {
    if f36(v0) {
        Some(f37(v0))
    } else {
        None::<(u64, u64)>
    }
}

pub fn f32(v0: (u64, u64), v1: u64) -> R<Option<(u64, u64)>> {
    if f38(v0, v1)? {
        Ok(Some(f39(v0, v1)?))
    } else {
        Ok(None::<(u64, u64)>)
    }
}

pub fn f33(v0: Adt2, v1: List<Adt2>, v2: u64) -> R<List<Adt2>> {
    let m184 = f26(match v0.clone() {
        Adt2::C0(h183, _) => {
            h183
        }
    }, Adt0::C2(v2))?;
    match m184 {
        None => {
            Ok(v1)
        }
        Some(v3) => {
            let a188 = v1;
            let a189 = List::cons(Adt2::C0(v3, {
                let a186 = match v0 {
                    Adt2::C0(_, h185) => {
                        h185
                    }
                };
                let a187 = List::cons(Adt0::C2(v2), List::<Adt0>::nil());
                append_list(a186, a187)
            }), List::<Adt2>::nil());
            Ok(append_list(a188, a189))
        }
    }
}

pub fn f34(v0: (u64, u64)) -> bool {
    let a191 = {
        let (h190, _) = v0;
        h190
    };
    let a192 = 4u64;
    if nat_lt(a191, a192) {
        let a194 = {
            let (_, h193) = v0;
            h193
        };
        let a195 = 3u64;
        nat_le(a194, a195)
    } else {
        false
    }
}

pub fn f35(v0: (u64, u64)) -> (u64, u64) {
    (4u64, {
        let (_, h196) = v0;
        h196
    })
}

pub fn f36(v0: (u64, u64)) -> bool {
    let a198 = 0u64;
    let a199 = {
        let (_, h197) = v0;
        h197
    };
    if nat_lt(a198, a199) {
        let a201 = {
            let (h200, _) = v0;
            h200
        };
        let a202 = 4u64;
        nat_le(a201, a202)
    } else {
        false
    }
}

pub fn f37(v0: (u64, u64)) -> (u64, u64) {
    ({
        let (h203, _) = v0;
        h203
    }, 0u64)
}

pub fn f38(v0: (u64, u64), v1: u64) -> R<bool> {
    let a205 = v1;
    let a206 = {
        let (h204, _) = v0;
        h204
    };
    if nat_le(a205, a206) {
        let a210 = {
            let a208 = {
                let (_, h207) = v0;
                h207
            };
            let a209 = v1;
            nat_add(a208, a209)?
        };
        let a211 = 3u64;
        if nat_le(a210, a211) {
            let a213 = {
                let (h212, _) = v0;
                h212
            };
            let a214 = 4u64;
            Ok(nat_le(a213, a214))
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f39(v0: (u64, u64), v1: u64) -> R<(u64, u64)> {
    Ok(({
        let a216 = {
            let (h215, _) = v0;
            h215
        };
        let a217 = v1;
        nat_sub(a216, a217)
    }, {
        let a219 = {
            let (_, h218) = v0;
            h218
        };
        let a220 = v1;
        nat_add(a219, a220)?
    }))
}
