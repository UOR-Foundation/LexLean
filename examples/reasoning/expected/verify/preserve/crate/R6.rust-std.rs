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
    C0(u64),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(List<Adt2>, Option<(u64, Adt2)>, bool, Adt3),
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
    F10,
    F14,
}

impl Fn1 {
    pub fn apply(&self, p0: List<Adt2>, p1: Adt2) -> List<Adt2> {
        match self {
            Fn1::F10 => f10(p0, p1),
            Fn1::F14 => f14(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F21(Adt2),
}

impl Fn2 {
    pub fn apply(&self, p0: List<Adt2>, p1: u64) -> List<Adt2> {
        match self {
            Fn2::F21(k0) => f21(k0.clone(), p0, p1),
        }
    }
}

pub fn f0(v0: u64) -> R<u64> {
    let m1 = f1(v0)?;
    match m1 {
        Ok(v1) => {
            Ok(v1)
        }
        Err(_) => {
            Ok(9u64)
        }
    }
}

pub fn f1(v0: u64) -> R<Result<u64, (bool, bool)>> {
    let m2 = f2(v0)?;
    match m2 {
        Ok(v1) => {
            Ok(Ok::<u64, (bool, bool)>({
                let (h3, _) = v1;
                h3
            }))
        }
        Err(v2) => {
            Ok(Err::<u64, (bool, bool)>(v2))
        }
    }
}

pub fn f2(v0: u64) -> R<Result<(u64, List<Adt0>), (bool, bool)>> {
    let v1: (Adt1, bool) = f3(v0)?;
    let m7 = {
        let m6 = {
            let (h5, _) = v1.clone();
            h5
        };
        match m6 {
            Adt1::C0(_, h4, _, _) => {
                h4
            }
        }
    };
    match m7 {
        None => {
            Ok(Err::<(u64, List<Adt0>), (bool, bool)>(f4({
                let (_, h12) = v1;
                h12
            }, {
                let m15 = {
                    let (h14, _) = v1;
                    h14
                };
                match m15 {
                    Adt1::C0(_, _, h13, _) => {
                        h13
                    }
                }
            })))
        }
        Some(v2) => {
            Ok(Ok::<(u64, List<Adt0>), (bool, bool)>(({
                let (h8, _) = v2;
                h8
            }, {
                let m11 = {
                    let (_, h10) = v2;
                    h10
                };
                match m11 {
                    Adt2::C0(_, h9) => {
                        h9
                    }
                }
            })))
        }
    }
}

pub fn f3(v0: u64) -> R<(Adt1, bool)> {
    f5(Fn0::F6(v0), 8u64, f7(v0))
}

pub fn f4(v0: bool, v1: bool) -> (bool, bool) {
    if if v0 {
        let a16 = v1;
        bool_not(a16)
    } else {
        false
    } {
        (false, true)
    } else {
        (false, false)
    }
}

pub fn f5(v0: Fn0, p21: u64, p22: Adt1) -> R<(Adt1, bool)> {
    let m20 = (p21, p22);
    {
        let mut m20 = m20;
        loop {
            let step = {
                let (v1, v2) = m20;
                let m17 = v1;
                if m17 == 0 {
                    Err::<(u64, Adt1), R<(Adt1, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m17 - 1;
                    let m19 = {
                        let c18: Fn0 = v0.clone();
                        c18.apply(v2.clone())?
                    };
                    match m19 {
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
                Ok(next) => { m20 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f6(v0: u64, v1: Adt1) -> R<Option<Adt1>> {
    f8(v0, v1)
}

pub fn f7(v0: u64) -> Adt1 {
    let v1: List<Adt2> = f9(Fn1::F10, List::<Adt2>::nil(), List::cons(Adt2::C0(f11(v0), List::<Adt0>::nil()), List::<Adt2>::nil()));
    Adt1::C0(v1.clone(), None::<(u64, Adt2)>, {
        let a23 = 3u64;
        let a24 = 1u64;
        nat_lt(a23, a24)
    }, Adt3::C0(0u64, 0u64, 0u64, 0u64, 0u64, {
        let a25 = v1;
        length_list(a25)
    }))
}

pub fn f8(v0: u64, v1: Adt1) -> R<Option<Adt1>> {
    let m27 = match v1.clone() {
        Adt1::C0(_, h26, _, _) => {
            h26
        }
    };
    match m27 {
        None => {
            let m29 = match v1.clone() {
                Adt1::C0(h28, _, _, _) => {
                    h28
                }
            };
            match m29.uncons() {
                None => {
                    Ok(None::<Adt1>)
                }
                Some((v3, v4)) => {
                    let m31 = f12(v0, match v3.clone() {
                        Adt2::C0(h30, _) => {
                            h30
                        }
                    })?;
                    match m31 {
                        None => {
                            let v6: List<Adt2> = f13(v3.clone());
                            let v7: List<Adt2> = v6.clone();
                            let v8: List<Adt2> = {
                                let a55 = v7;
                                let a56 = v4;
                                append_list(a55, a56)
                            };
                            let v9: List<Adt2> = f9(Fn1::F14, List::<Adt2>::nil(), v8.clone());
                            Ok(Some(Adt1::C0(v9.clone(), None::<(u64, Adt2)>, if match v1.clone() {
                                Adt1::C0(_, _, h57, _) => {
                                    h57
                                }
                            } {
                                true
                            } else {
                                let a59 = 3u64;
                                let a60 = {
                                    let a58 = v8;
                                    length_list(a58)
                                };
                                nat_lt(a59, a60)
                            }, Adt3::C0({
                                let a64 = {
                                    let m63 = match v1.clone() {
                                        Adt1::C0(_, _, _, h62) => {
                                            h62
                                        }
                                    };
                                    match m63 {
                                        Adt3::C0(h61, _, _, _, _, _) => {
                                            h61
                                        }
                                    }
                                };
                                let a65 = 1u64;
                                nat_add(a64, a65)?
                            }, {
                                let a70 = {
                                    let m68 = match v1.clone() {
                                        Adt1::C0(_, _, _, h67) => {
                                            h67
                                        }
                                    };
                                    match m68 {
                                        Adt3::C0(_, h66, _, _, _, _) => {
                                            h66
                                        }
                                    }
                                };
                                let a71 = f15(match v3 {
                                    Adt2::C0(h69, _) => {
                                        h69
                                    }
                                });
                                nat_add(a70, a71)?
                            }, {
                                let a76 = {
                                    let m74 = match v1.clone() {
                                        Adt1::C0(_, _, _, h73) => {
                                            h73
                                        }
                                    };
                                    match m74 {
                                        Adt3::C0(_, _, h72, _, _, _) => {
                                            h72
                                        }
                                    }
                                };
                                let a77 = {
                                    let a75 = v6;
                                    length_list(a75)
                                };
                                nat_add(a76, a77)?
                            }, {
                                let a81 = {
                                    let m80 = match v1.clone() {
                                        Adt1::C0(_, _, _, h79) => {
                                            h79
                                        }
                                    };
                                    match m80 {
                                        Adt3::C0(_, _, _, h78, _, _) => {
                                            h78
                                        }
                                    }
                                };
                                let a82 = 1u64;
                                nat_add(a81, a82)?
                            }, {
                                let a86 = {
                                    let m85 = match v1.clone() {
                                        Adt1::C0(_, _, _, h84) => {
                                            h84
                                        }
                                    };
                                    match m85 {
                                        Adt3::C0(_, _, _, _, h83, _) => {
                                            h83
                                        }
                                    }
                                };
                                let a87 = 1u64;
                                nat_add(a86, a87)?
                            }, {
                                let a92 = {
                                    let m90 = match v1.clone() {
                                        Adt1::C0(_, _, _, h89) => {
                                            h89
                                        }
                                    };
                                    match m90 {
                                        Adt3::C0(_, _, _, _, _, h88) => {
                                            h88
                                        }
                                    }
                                };
                                let a93 = {
                                    let a91 = v9.clone();
                                    length_list(a91)
                                };
                                if nat_lt(a92, a93) {
                                    let a94 = v9;
                                    length_list(a94)
                                } else {
                                    let m97 = match v1 {
                                        Adt1::C0(_, _, _, h96) => {
                                            h96
                                        }
                                    };
                                    match m97 {
                                        Adt3::C0(_, _, _, _, _, h95) => {
                                            h95
                                        }
                                    }
                                }
                            }))))
                        }
                        Some(v5) => {
                            Ok(Some(Adt1::C0(v4, Some((v5, v3)), match v1.clone() {
                                Adt1::C0(_, _, h32, _) => {
                                    h32
                                }
                            }, Adt3::C0({
                                let a36 = {
                                    let m35 = match v1.clone() {
                                        Adt1::C0(_, _, _, h34) => {
                                            h34
                                        }
                                    };
                                    match m35 {
                                        Adt3::C0(h33, _, _, _, _, _) => {
                                            h33
                                        }
                                    }
                                };
                                let a37 = 1u64;
                                nat_add(a36, a37)?
                            }, {
                                let m40 = match v1.clone() {
                                    Adt1::C0(_, _, _, h39) => {
                                        h39
                                    }
                                };
                                match m40 {
                                    Adt3::C0(_, h38, _, _, _, _) => {
                                        h38
                                    }
                                }
                            }, {
                                let m43 = match v1.clone() {
                                    Adt1::C0(_, _, _, h42) => {
                                        h42
                                    }
                                };
                                match m43 {
                                    Adt3::C0(_, _, h41, _, _, _) => {
                                        h41
                                    }
                                }
                            }, {
                                let m46 = match v1.clone() {
                                    Adt1::C0(_, _, _, h45) => {
                                        h45
                                    }
                                };
                                match m46 {
                                    Adt3::C0(_, _, _, h44, _, _) => {
                                        h44
                                    }
                                }
                            }, {
                                let a50 = {
                                    let m49 = match v1.clone() {
                                        Adt1::C0(_, _, _, h48) => {
                                            h48
                                        }
                                    };
                                    match m49 {
                                        Adt3::C0(_, _, _, _, h47, _) => {
                                            h47
                                        }
                                    }
                                };
                                let a51 = 1u64;
                                nat_add(a50, a51)?
                            }, {
                                let m54 = match v1 {
                                    Adt1::C0(_, _, _, h53) => {
                                        h53
                                    }
                                };
                                match m54 {
                                    Adt3::C0(_, _, _, _, _, h52) => {
                                        h52
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

pub fn f9(v0: Fn1, p101: List<Adt2>, p102: List<Adt2>) -> List<Adt2> {
    let m100 = (p101, p102);
    {
        let mut m100 = m100;
        loop {
            let step = {
                let (v1, v2) = m100;
                let m98 = v2;
                match m98.uncons() {
                    None => {
                        Err::<(List<Adt2>, List<Adt2>), List<Adt2>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt2>, List<Adt2>), List<Adt2>>(({
                            let c99: Fn1 = v0.clone();
                            c99.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m100 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f10(v0: List<Adt2>, v1: Adt2) -> List<Adt2> {
    let a104 = {
        let a103 = v0.clone();
        length_list(a103)
    };
    let a105 = 3u64;
    if nat_lt(a104, a105) {
        let a106 = v0;
        let a107 = List::cons(v1, List::<Adt2>::nil());
        append_list(a106, a107)
    } else {
        v0
    }
}

pub fn f11(v0: u64) -> (u64, u64) {
    (v0, 0u64)
}

pub fn f12(v0: u64, v1: (u64, u64)) -> R<Option<u64>> {
    let m108 = f16(v1);
    match m108 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f17(v0, v2)? {
                Ok(Some(v2))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f13(v0: Adt2) -> List<Adt2> {
    f18(v0)
}

pub fn f14(v0: List<Adt2>, v1: Adt2) -> List<Adt2> {
    let a110 = {
        let a109 = v0.clone();
        length_list(a109)
    };
    let a111 = 3u64;
    if nat_lt(a110, a111) {
        let a112 = v0;
        let a113 = List::cons(v1, List::<Adt2>::nil());
        append_list(a112, a113)
    } else {
        v0
    }
}

pub fn f15(v0: (u64, u64)) -> u64 {
    let a114 = f19(v0);
    length_list(a114)
}

pub fn f16(v0: (u64, u64)) -> Option<u64> {
    let a116 = 0u64;
    let a117 = {
        let (_, h115) = v0;
        h115
    };
    if nat_lt(a116, a117) {
        Some({
            let (_, h118) = v0;
            h118
        })
    } else {
        None::<u64>
    }
}

pub fn f17(v0: u64, v1: u64) -> R<bool> {
    let a121 = {
        let a119 = v1;
        let a120 = v1;
        nat_add(a119, a120)?
    };
    let a122 = v0;
    Ok(nat_le(a121, a122))
}

pub fn f18(v0: Adt2) -> List<Adt2> {
    f20(Fn2::F21(v0.clone()), List::<Adt2>::nil(), f19(match v0 {
        Adt2::C0(h123, _) => {
            h123
        }
    }))
}

pub fn f19(v0: (u64, u64)) -> List<u64> {
    let m125 = {
        let v1: u64 = {
            let (h124, _) = v0;
            h124
        };
        let v2: List<u64> = f22(v1);
        if f23(v1, v2.clone()) {
            Ok::<List<u64>, (bool, bool)>(v2)
        } else {
            Err::<List<u64>, (bool, bool)>((true, false))
        }
    };
    match m125 {
        Ok(v3) => {
            v3
        }
        Err(_) => {
            List::<u64>::nil()
        }
    }
}

pub fn f20(v0: Fn2, p129: List<Adt2>, p130: List<u64>) -> List<Adt2> {
    let m128 = (p129, p130);
    {
        let mut m128 = m128;
        loop {
            let step = {
                let (v1, v2) = m128;
                let m126 = v2;
                match m126.uncons() {
                    None => {
                        Err::<(List<Adt2>, List<u64>), List<Adt2>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt2>, List<u64>), List<Adt2>>(({
                            let c127: Fn2 = v0.clone();
                            c127.apply(v1, v3)
                        }, v4))
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

pub fn f21(v0: Adt2, v1: List<Adt2>, v2: u64) -> List<Adt2> {
    f24(v0, v1, v2)
}

pub fn f22(v0: u64) -> List<u64> {
    f25(v0)
}

pub fn f23(_: u64, v1: List<u64>) -> bool {
    let a132 = {
        let a131 = v1;
        length_list(a131)
    };
    let a133 = 4u64;
    nat_le(a132, a133)
}

pub fn f24(v0: Adt2, v1: List<Adt2>, v2: u64) -> List<Adt2> {
    let m135 = f26(match v0.clone() {
        Adt2::C0(h134, _) => {
            h134
        }
    }, Adt0::C0(v2));
    match m135 {
        None => {
            v1
        }
        Some(v3) => {
            let a139 = v1;
            let a140 = List::cons(Adt2::C0(v3, {
                let a137 = match v0 {
                    Adt2::C0(_, h136) => {
                        h136
                    }
                };
                let a138 = List::cons(Adt0::C0(v2), List::<Adt0>::nil());
                append_list(a137, a138)
            }), List::<Adt2>::nil());
            append_list(a139, a140)
        }
    }
}

pub fn f25(v0: u64) -> List<u64> {
    List::cons({
        let a141 = v0;
        let a142 = 10u64;
        nat_sub(a141, a142)
    }, List::cons({
        let a143 = v0;
        let a144 = 20u64;
        nat_sub(a143, a144)
    }, List::cons({
        let a145 = v0;
        let a146 = 30u64;
        nat_sub(a145, a146)
    }, List::cons({
        let a147 = v0;
        let a148 = 50u64;
        nat_sub(a147, a148)
    }, List::<u64>::nil()))))
}

pub fn f26(v0: (u64, u64), v1: Adt0) -> Option<(u64, u64)> {
    let m149 = v1;
    match m149 {
        Adt0::C0(v2) => {
            f27(v0, v2)
        }
    }
}

pub fn f27(v0: (u64, u64), v1: u64) -> Option<(u64, u64)> {
    if f28(v0, v1) {
        Some(f29(v0, v1))
    } else {
        None::<(u64, u64)>
    }
}

pub fn f28(v0: (u64, u64), v1: u64) -> bool {
    let a151 = {
        let (_, h150) = v0;
        h150
    };
    let a152 = 0u64;
    if nat_eq(a151, a152) {
        let a153 = 0u64;
        let a154 = v1;
        nat_lt(a153, a154)
    } else {
        false
    }
}

pub fn f29(v0: (u64, u64), v1: u64) -> (u64, u64) {
    ({
        let (h155, _) = v0;
        h155
    }, v1)
}
