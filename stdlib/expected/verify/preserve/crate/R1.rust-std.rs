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
    C0(u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt2 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32, Adt1, u64),
}

#[derive(Clone)]
pub enum Fn0 {
    F5(Bytes, u64, u64),
}

impl Fn0 {
    pub fn apply(&self, p0: (u64, Adt0)) -> R<(u64, Adt0)> {
        match self {
            Fn0::F5(k0, k1, k2) => f5(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F12,
}

impl Fn1 {
    pub fn apply(&self, p0: Adt2) -> R<Adt2> {
        match self {
            Fn1::F12 => f12(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F28,
}

impl Fn2 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn2::F28 => f28(p0),
        }
    }
}

pub fn f0(v0: Bytes) -> R<Bytes> {
    Ok(f1(f2(v0)?))
}

pub fn f1(v0: Adt0) -> Bytes {
    let a21 = f3(match v0.clone() {
        Adt0::C0(h1, _, _, _, _, _, _, _) => {
            h1
        }
    });
    let a22 = {
        let a19 = f3(match v0.clone() {
            Adt0::C0(_, h2, _, _, _, _, _, _) => {
                h2
            }
        });
        let a20 = {
            let a17 = f3(match v0.clone() {
                Adt0::C0(_, _, h3, _, _, _, _, _) => {
                    h3
                }
            });
            let a18 = {
                let a15 = f3(match v0.clone() {
                    Adt0::C0(_, _, _, h4, _, _, _, _) => {
                        h4
                    }
                });
                let a16 = {
                    let a13 = f3(match v0.clone() {
                        Adt0::C0(_, _, _, _, h5, _, _, _) => {
                            h5
                        }
                    });
                    let a14 = {
                        let a11 = f3(match v0.clone() {
                            Adt0::C0(_, _, _, _, _, h6, _, _) => {
                                h6
                            }
                        });
                        let a12 = {
                            let a9 = f3(match v0.clone() {
                                Adt0::C0(_, _, _, _, _, _, h7, _) => {
                                    h7
                                }
                            });
                            let a10 = f3(match v0 {
                                Adt0::C0(_, _, _, _, _, _, _, h8) => {
                                    h8
                                }
                            });
                            append_bytes(a9, a10)
                        };
                        append_bytes(a11, a12)
                    };
                    append_bytes(a13, a14)
                };
                append_bytes(a15, a16)
            };
            append_bytes(a17, a18)
        };
        append_bytes(a19, a20)
    };
    append_bytes(a21, a22)
}

pub fn f2(v0: Bytes) -> R<Adt0> {
    let v1: u64 = {
        let a23 = v0.clone();
        length_bytes(a23)
    };
    let v2: u64 = {
        let a29 = {
            let a26 = {
                let a24 = v1;
                let a25 = 72u64;
                nat_add(a24, a25)?
            };
            let a27 = 64u64;
            let a28 = 0u64;
            nat_quot(a26, a27, a28)
        };
        let a30 = 64u64;
        nat_mul(a29, a30)?
    };
    let (_, h31) = f4(Fn0::F5(v0, v1, v2), {
        let a32 = v2;
        let a33 = 64u64;
        let a34 = 0u64;
        nat_quot(a32, a33, a34)
    }, (0u64, f6()))?;
    Ok(h31)
}

pub fn f3(v0: u32) -> Bytes {
    let a63 = f7({
        let a35 = f8(v0, 28u32);
        let a36 = 15u32;
        fixed_u32::bit_and(a35, a36)
    });
    let a64 = {
        let a61 = f7({
            let a37 = f8(v0, 24u32);
            let a38 = 15u32;
            fixed_u32::bit_and(a37, a38)
        });
        let a62 = {
            let a59 = f7({
                let a39 = f8(v0, 20u32);
                let a40 = 15u32;
                fixed_u32::bit_and(a39, a40)
            });
            let a60 = {
                let a57 = f7({
                    let a41 = f8(v0, 16u32);
                    let a42 = 15u32;
                    fixed_u32::bit_and(a41, a42)
                });
                let a58 = {
                    let a55 = f7({
                        let a43 = f8(v0, 12u32);
                        let a44 = 15u32;
                        fixed_u32::bit_and(a43, a44)
                    });
                    let a56 = {
                        let a53 = f7({
                            let a45 = f8(v0, 8u32);
                            let a46 = 15u32;
                            fixed_u32::bit_and(a45, a46)
                        });
                        let a54 = {
                            let a51 = f7({
                                let a47 = f8(v0, 4u32);
                                let a48 = 15u32;
                                fixed_u32::bit_and(a47, a48)
                            });
                            let a52 = f7({
                                let a49 = v0;
                                let a50 = 15u32;
                                fixed_u32::bit_and(a49, a50)
                            });
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
}

pub fn f4(v0: Fn0, p68: u64, p69: (u64, Adt0)) -> R<(u64, Adt0)> {
    let m67 = (p68, p69);
    {
        let mut m67 = m67;
        loop {
            let step = {
                let (v1, v2) = m67;
                let m65 = v1;
                if m65 == 0 {
                    Err::<(u64, (u64, Adt0)), R<(u64, Adt0)>>(Ok(v2))
                } else {
                    let v3 = m65 - 1;
                    Ok::<(u64, (u64, Adt0)), R<(u64, Adt0)>>((v3, {
                        let c66: Fn0 = v0.clone();
                        c66.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m67 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f5(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt0)) -> R<(u64, Adt0)> {
    Ok(({
        let a71 = {
            let (h70, _) = v3.clone();
            h70
        };
        let a72 = 64u64;
        nat_add(a71, a72)?
    }, f9({
        let (_, h73) = v3;
        h73
    }, f10(v0, v1, v2, {
        let (h74, _) = v3;
        h74
    })?)?))
}

pub fn f6() -> Adt0 {
    Adt0::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f7(v0: u32) -> Bytes {
    let a77 = {
        let a75 = v0;
        let a76 = 8u32;
        fixed_u32::bit_and(a75, a76)
    };
    let a78 = 0u32;
    if equal(a77, a78) {
        let a81 = {
            let a79 = v0;
            let a80 = 4u32;
            fixed_u32::bit_and(a79, a80)
        };
        let a82 = 0u32;
        if equal(a81, a82) {
            let a85 = {
                let a83 = v0;
                let a84 = 2u32;
                fixed_u32::bit_and(a83, a84)
            };
            let a86 = 0u32;
            if equal(a85, a86) {
                let a89 = {
                    let a87 = v0;
                    let a88 = 1u32;
                    fixed_u32::bit_and(a87, a88)
                };
                let a90 = 0u32;
                if equal(a89, a90) {
                    Bytes::lit(&[0x30])
                } else {
                    Bytes::lit(&[0x31])
                }
            } else {
                let a93 = {
                    let a91 = v0;
                    let a92 = 1u32;
                    fixed_u32::bit_and(a91, a92)
                };
                let a94 = 0u32;
                if equal(a93, a94) {
                    Bytes::lit(&[0x32])
                } else {
                    Bytes::lit(&[0x33])
                }
            }
        } else {
            let a97 = {
                let a95 = v0;
                let a96 = 2u32;
                fixed_u32::bit_and(a95, a96)
            };
            let a98 = 0u32;
            if equal(a97, a98) {
                let a101 = {
                    let a99 = v0;
                    let a100 = 1u32;
                    fixed_u32::bit_and(a99, a100)
                };
                let a102 = 0u32;
                if equal(a101, a102) {
                    Bytes::lit(&[0x34])
                } else {
                    Bytes::lit(&[0x35])
                }
            } else {
                let a105 = {
                    let a103 = v0;
                    let a104 = 1u32;
                    fixed_u32::bit_and(a103, a104)
                };
                let a106 = 0u32;
                if equal(a105, a106) {
                    Bytes::lit(&[0x36])
                } else {
                    Bytes::lit(&[0x37])
                }
            }
        }
    } else {
        let a109 = {
            let a107 = v0;
            let a108 = 4u32;
            fixed_u32::bit_and(a107, a108)
        };
        let a110 = 0u32;
        if equal(a109, a110) {
            let a113 = {
                let a111 = v0;
                let a112 = 2u32;
                fixed_u32::bit_and(a111, a112)
            };
            let a114 = 0u32;
            if equal(a113, a114) {
                let a117 = {
                    let a115 = v0;
                    let a116 = 1u32;
                    fixed_u32::bit_and(a115, a116)
                };
                let a118 = 0u32;
                if equal(a117, a118) {
                    Bytes::lit(&[0x38])
                } else {
                    Bytes::lit(&[0x39])
                }
            } else {
                let a121 = {
                    let a119 = v0;
                    let a120 = 1u32;
                    fixed_u32::bit_and(a119, a120)
                };
                let a122 = 0u32;
                if equal(a121, a122) {
                    Bytes::lit(&[0x61])
                } else {
                    Bytes::lit(&[0x62])
                }
            }
        } else {
            let a125 = {
                let a123 = v0;
                let a124 = 2u32;
                fixed_u32::bit_and(a123, a124)
            };
            let a126 = 0u32;
            if equal(a125, a126) {
                let a129 = {
                    let a127 = v0;
                    let a128 = 1u32;
                    fixed_u32::bit_and(a127, a128)
                };
                let a130 = 0u32;
                if equal(a129, a130) {
                    Bytes::lit(&[0x63])
                } else {
                    Bytes::lit(&[0x64])
                }
            } else {
                let a133 = {
                    let a131 = v0;
                    let a132 = 1u32;
                    fixed_u32::bit_and(a131, a132)
                };
                let a134 = 0u32;
                if equal(a133, a134) {
                    Bytes::lit(&[0x65])
                } else {
                    Bytes::lit(&[0x66])
                }
            }
        }
    }
}

pub fn f8(v0: u32, v1: u32) -> u32 {
    let m137 = {
        let a135 = v0;
        let a136 = v1;
        fixed_u32::shift_right(a135, a136)
    };
    match m137 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f9(v0: Adt0, v1: Adt1) -> R<Adt0> {
    let v2: Adt2 = f11(Fn1::F12, 64u64, Adt2::C0(match v0.clone() {
        Adt0::C0(h138, _, _, _, _, _, _, _) => {
            h138
        }
    }, match v0.clone() {
        Adt0::C0(_, h139, _, _, _, _, _, _) => {
            h139
        }
    }, match v0.clone() {
        Adt0::C0(_, _, h140, _, _, _, _, _) => {
            h140
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, h141, _, _, _, _) => {
            h141
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, h142, _, _, _) => {
            h142
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, h143, _, _) => {
            h143
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, h144, _) => {
            h144
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, h145) => {
            h145
        }
    }, v1, 0u64))?;
    Ok(Adt0::C0(f13(match v0.clone() {
        Adt0::C0(h146, _, _, _, _, _, _, _) => {
            h146
        }
    }, match v2.clone() {
        Adt2::C0(h147, _, _, _, _, _, _, _, _, _) => {
            h147
        }
    }), f13(match v0.clone() {
        Adt0::C0(_, h148, _, _, _, _, _, _) => {
            h148
        }
    }, match v2.clone() {
        Adt2::C0(_, h149, _, _, _, _, _, _, _, _) => {
            h149
        }
    }), f13(match v0.clone() {
        Adt0::C0(_, _, h150, _, _, _, _, _) => {
            h150
        }
    }, match v2.clone() {
        Adt2::C0(_, _, h151, _, _, _, _, _, _, _) => {
            h151
        }
    }), f13(match v0.clone() {
        Adt0::C0(_, _, _, h152, _, _, _, _) => {
            h152
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, h153, _, _, _, _, _, _) => {
            h153
        }
    }), f13(match v0.clone() {
        Adt0::C0(_, _, _, _, h154, _, _, _) => {
            h154
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, _, h155, _, _, _, _, _) => {
            h155
        }
    }), f13(match v0.clone() {
        Adt0::C0(_, _, _, _, _, h156, _, _) => {
            h156
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, _, _, h157, _, _, _, _) => {
            h157
        }
    }), f13(match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, h158, _) => {
            h158
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, _, _, _, h159, _, _, _) => {
            h159
        }
    }), f13(match v0 {
        Adt0::C0(_, _, _, _, _, _, _, h160) => {
            h160
        }
    }, match v2 {
        Adt2::C0(_, _, _, _, _, _, _, h161, _, _) => {
            h161
        }
    })))
}

pub fn f10(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt1> {
    Ok(Adt1::C0(f14(v0.clone(), v1, v2, v3)?, f14(v0.clone(), v1, v2, {
        let a162 = v3;
        let a163 = 4u64;
        nat_add(a162, a163)?
    })?, f14(v0.clone(), v1, v2, {
        let a164 = v3;
        let a165 = 8u64;
        nat_add(a164, a165)?
    })?, f14(v0.clone(), v1, v2, {
        let a166 = v3;
        let a167 = 12u64;
        nat_add(a166, a167)?
    })?, f14(v0.clone(), v1, v2, {
        let a168 = v3;
        let a169 = 16u64;
        nat_add(a168, a169)?
    })?, f14(v0.clone(), v1, v2, {
        let a170 = v3;
        let a171 = 20u64;
        nat_add(a170, a171)?
    })?, f14(v0.clone(), v1, v2, {
        let a172 = v3;
        let a173 = 24u64;
        nat_add(a172, a173)?
    })?, f14(v0.clone(), v1, v2, {
        let a174 = v3;
        let a175 = 28u64;
        nat_add(a174, a175)?
    })?, f14(v0.clone(), v1, v2, {
        let a176 = v3;
        let a177 = 32u64;
        nat_add(a176, a177)?
    })?, f14(v0.clone(), v1, v2, {
        let a178 = v3;
        let a179 = 36u64;
        nat_add(a178, a179)?
    })?, f14(v0.clone(), v1, v2, {
        let a180 = v3;
        let a181 = 40u64;
        nat_add(a180, a181)?
    })?, f14(v0.clone(), v1, v2, {
        let a182 = v3;
        let a183 = 44u64;
        nat_add(a182, a183)?
    })?, f14(v0.clone(), v1, v2, {
        let a184 = v3;
        let a185 = 48u64;
        nat_add(a184, a185)?
    })?, f14(v0.clone(), v1, v2, {
        let a186 = v3;
        let a187 = 52u64;
        nat_add(a186, a187)?
    })?, f14(v0.clone(), v1, v2, {
        let a188 = v3;
        let a189 = 56u64;
        nat_add(a188, a189)?
    })?, f14(v0, v1, v2, {
        let a190 = v3;
        let a191 = 60u64;
        nat_add(a190, a191)?
    })?))
}

pub fn f11(v0: Fn1, p195: u64, p196: Adt2) -> R<Adt2> {
    let m194 = (p195, p196);
    {
        let mut m194 = m194;
        loop {
            let step = {
                let (v1, v2) = m194;
                let m192 = v1;
                if m192 == 0 {
                    Err::<(u64, Adt2), R<Adt2>>(Ok(v2))
                } else {
                    let v3 = m192 - 1;
                    Ok::<(u64, Adt2), R<Adt2>>((v3, {
                        let c193: Fn1 = v0.clone();
                        c193.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m194 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f12(v0: Adt2) -> R<Adt2> {
    f15(v0)
}

pub fn f13(v0: u32, v1: u32) -> u32 {
    let m199 = {
        let a197 = f16(v0);
        let a198 = f16(v1);
        fixed_u64::checked_add(a197, a198)
    };
    match m199 {
        None => {
            0u32
        }
        Some(v2) => {
            f17(v2)
        }
    }
}

pub fn f14(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u32> {
    let a202 = {
        let a200 = v3;
        let a201 = 8u64;
        nat_add(a200, a201)?
    };
    let a203 = v2;
    if nat_eq(a202, a203) {
        Ok(f18({
            let a206 = {
                let a204 = v1;
                let a205 = 8u64;
                nat_mul(a204, a205)?
            };
            let a207 = 4294967296u64;
            let a208 = 0u64;
            nat_quot(a206, a207, a208)
        }))
    } else {
        let a211 = {
            let a209 = v3;
            let a210 = 4u64;
            nat_add(a209, a210)?
        };
        let a212 = v2;
        if nat_eq(a211, a212) {
            Ok(f18({
                let a215 = {
                    let a213 = v1;
                    let a214 = 8u64;
                    nat_mul(a213, a214)?
                };
                let a216 = 4294967296u64;
                let a217 = 0u64;
                nat_rem(a215, a216, a217)
            }))
        } else {
            Ok(f19(f20(v0.clone(), v1, v3), f20(v0.clone(), v1, {
                let a218 = v3;
                let a219 = 1u64;
                nat_add(a218, a219)?
            }), f20(v0.clone(), v1, {
                let a220 = v3;
                let a221 = 2u64;
                nat_add(a220, a221)?
            }), f20(v0, v1, {
                let a222 = v3;
                let a223 = 3u64;
                nat_add(a222, a223)?
            })))
        }
    }
}

pub fn f15(v0: Adt2) -> R<Adt2> {
    let v1: u32 = f21(match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h224) => {
            h224
        }
    });
    let v2: u32 = f13(match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h225, _, _) => {
            h225
        }
    }, f13(f22(match v0.clone() {
        Adt2::C0(_, _, _, _, h226, _, _, _, _, _) => {
            h226
        }
    }), f13(f23(match v0.clone() {
        Adt2::C0(_, _, _, _, h227, _, _, _, _, _) => {
            h227
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h228, _, _, _, _) => {
            h228
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h229, _, _, _) => {
            h229
        }
    }), f13(v1, {
        let m232 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, h231, _) => {
                h231
            }
        };
        match m232 {
            Adt1::C0(h230, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h230
            }
        }
    }))));
    let v3: u32 = f13(f24(match v0.clone() {
        Adt2::C0(h233, _, _, _, _, _, _, _, _, _) => {
            h233
        }
    }), f25(match v0.clone() {
        Adt2::C0(h234, _, _, _, _, _, _, _, _, _) => {
            h234
        }
    }, match v0.clone() {
        Adt2::C0(_, h235, _, _, _, _, _, _, _, _) => {
            h235
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h236, _, _, _, _, _, _, _) => {
            h236
        }
    }));
    Ok(Adt2::C0(f13(v2, v3), match v0.clone() {
        Adt2::C0(h237, _, _, _, _, _, _, _, _, _) => {
            h237
        }
    }, match v0.clone() {
        Adt2::C0(_, h238, _, _, _, _, _, _, _, _) => {
            h238
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h239, _, _, _, _, _, _, _) => {
            h239
        }
    }, f13(match v0.clone() {
        Adt2::C0(_, _, _, h240, _, _, _, _, _, _) => {
            h240
        }
    }, v2), match v0.clone() {
        Adt2::C0(_, _, _, _, h241, _, _, _, _, _) => {
            h241
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h242, _, _, _, _) => {
            h242
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h243, _, _, _) => {
            h243
        }
    }, f26(match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, h244, _) => {
            h244
        }
    }), {
        let a246 = match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h245) => {
                h245
            }
        };
        let a247 = 1u64;
        nat_add(a246, a247)?
    }))
}

pub fn f16(v0: u32) -> u64 {
    let m249 = {
        let a248 = v0;
        fixed_u64::convert(i128::from(a248))
    };
    match m249 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f17(v0: u64) -> u32 {
    let m253 = {
        let a252 = {
            let a250 = v0;
            let a251 = 4294967295u64;
            fixed_u64::bit_and(a250, a251)
        };
        fixed_u32::convert(i128::from(a252))
    };
    match m253 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f18(v0: u64) -> u32 {
    let (_, h254) = {
        let (_, h255) = f27(Fn2::F28, 32u64, (v0, (1u32, 0u32)));
        h255
    };
    h254
}

pub fn f19(v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
    let a260 = f29(v0, 24u32);
    let a261 = {
        let a258 = f29(v1, 16u32);
        let a259 = {
            let a256 = f29(v2, 8u32);
            let a257 = v3;
            fixed_u32::bit_or(a256, a257)
        };
        fixed_u32::bit_or(a258, a259)
    };
    fixed_u32::bit_or(a260, a261)
}

pub fn f20(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a262 = v2;
    let a263 = v1;
    if nat_lt(a262, a263) {
        let m266 = {
            let a264 = v0;
            let a265 = v2;
            index_bytes(a264, a265)
        };
        match m266 {
            None => {
                0u32
            }
            Some(v3) => {
                let m268 = {
                    let a267 = v3;
                    fixed_u32::convert(i128::from(a267))
                };
                match m268 {
                    None => {
                        0u32
                    }
                    Some(v4) => {
                        v4
                    }
                }
            }
        }
    } else {
        let a269 = v2;
        let a270 = v1;
        if nat_eq(a269, a270) {
            128u32
        } else {
            0u32
        }
    }
}

pub fn f21(v0: u64) -> u32 {
    let a271 = v0;
    let a272 = 32u64;
    if nat_lt(a271, a272) {
        let a273 = v0;
        let a274 = 16u64;
        if nat_lt(a273, a274) {
            let a275 = v0;
            let a276 = 8u64;
            if nat_lt(a275, a276) {
                let a277 = v0;
                let a278 = 4u64;
                if nat_lt(a277, a278) {
                    let a279 = v0;
                    let a280 = 2u64;
                    if nat_lt(a279, a280) {
                        let a281 = v0;
                        let a282 = 1u64;
                        if nat_lt(a281, a282) {
                            1116352408u32
                        } else {
                            1899447441u32
                        }
                    } else {
                        let a283 = v0;
                        let a284 = 3u64;
                        if nat_lt(a283, a284) {
                            3049323471u32
                        } else {
                            3921009573u32
                        }
                    }
                } else {
                    let a285 = v0;
                    let a286 = 6u64;
                    if nat_lt(a285, a286) {
                        let a287 = v0;
                        let a288 = 5u64;
                        if nat_lt(a287, a288) {
                            961987163u32
                        } else {
                            1508970993u32
                        }
                    } else {
                        let a289 = v0;
                        let a290 = 7u64;
                        if nat_lt(a289, a290) {
                            2453635748u32
                        } else {
                            2870763221u32
                        }
                    }
                }
            } else {
                let a291 = v0;
                let a292 = 12u64;
                if nat_lt(a291, a292) {
                    let a293 = v0;
                    let a294 = 10u64;
                    if nat_lt(a293, a294) {
                        let a295 = v0;
                        let a296 = 9u64;
                        if nat_lt(a295, a296) {
                            3624381080u32
                        } else {
                            310598401u32
                        }
                    } else {
                        let a297 = v0;
                        let a298 = 11u64;
                        if nat_lt(a297, a298) {
                            607225278u32
                        } else {
                            1426881987u32
                        }
                    }
                } else {
                    let a299 = v0;
                    let a300 = 14u64;
                    if nat_lt(a299, a300) {
                        let a301 = v0;
                        let a302 = 13u64;
                        if nat_lt(a301, a302) {
                            1925078388u32
                        } else {
                            2162078206u32
                        }
                    } else {
                        let a303 = v0;
                        let a304 = 15u64;
                        if nat_lt(a303, a304) {
                            2614888103u32
                        } else {
                            3248222580u32
                        }
                    }
                }
            }
        } else {
            let a305 = v0;
            let a306 = 24u64;
            if nat_lt(a305, a306) {
                let a307 = v0;
                let a308 = 20u64;
                if nat_lt(a307, a308) {
                    let a309 = v0;
                    let a310 = 18u64;
                    if nat_lt(a309, a310) {
                        let a311 = v0;
                        let a312 = 17u64;
                        if nat_lt(a311, a312) {
                            3835390401u32
                        } else {
                            4022224774u32
                        }
                    } else {
                        let a313 = v0;
                        let a314 = 19u64;
                        if nat_lt(a313, a314) {
                            264347078u32
                        } else {
                            604807628u32
                        }
                    }
                } else {
                    let a315 = v0;
                    let a316 = 22u64;
                    if nat_lt(a315, a316) {
                        let a317 = v0;
                        let a318 = 21u64;
                        if nat_lt(a317, a318) {
                            770255983u32
                        } else {
                            1249150122u32
                        }
                    } else {
                        let a319 = v0;
                        let a320 = 23u64;
                        if nat_lt(a319, a320) {
                            1555081692u32
                        } else {
                            1996064986u32
                        }
                    }
                }
            } else {
                let a321 = v0;
                let a322 = 28u64;
                if nat_lt(a321, a322) {
                    let a323 = v0;
                    let a324 = 26u64;
                    if nat_lt(a323, a324) {
                        let a325 = v0;
                        let a326 = 25u64;
                        if nat_lt(a325, a326) {
                            2554220882u32
                        } else {
                            2821834349u32
                        }
                    } else {
                        let a327 = v0;
                        let a328 = 27u64;
                        if nat_lt(a327, a328) {
                            2952996808u32
                        } else {
                            3210313671u32
                        }
                    }
                } else {
                    let a329 = v0;
                    let a330 = 30u64;
                    if nat_lt(a329, a330) {
                        let a331 = v0;
                        let a332 = 29u64;
                        if nat_lt(a331, a332) {
                            3336571891u32
                        } else {
                            3584528711u32
                        }
                    } else {
                        let a333 = v0;
                        let a334 = 31u64;
                        if nat_lt(a333, a334) {
                            113926993u32
                        } else {
                            338241895u32
                        }
                    }
                }
            }
        }
    } else {
        let a335 = v0;
        let a336 = 48u64;
        if nat_lt(a335, a336) {
            let a337 = v0;
            let a338 = 40u64;
            if nat_lt(a337, a338) {
                let a339 = v0;
                let a340 = 36u64;
                if nat_lt(a339, a340) {
                    let a341 = v0;
                    let a342 = 34u64;
                    if nat_lt(a341, a342) {
                        let a343 = v0;
                        let a344 = 33u64;
                        if nat_lt(a343, a344) {
                            666307205u32
                        } else {
                            773529912u32
                        }
                    } else {
                        let a345 = v0;
                        let a346 = 35u64;
                        if nat_lt(a345, a346) {
                            1294757372u32
                        } else {
                            1396182291u32
                        }
                    }
                } else {
                    let a347 = v0;
                    let a348 = 38u64;
                    if nat_lt(a347, a348) {
                        let a349 = v0;
                        let a350 = 37u64;
                        if nat_lt(a349, a350) {
                            1695183700u32
                        } else {
                            1986661051u32
                        }
                    } else {
                        let a351 = v0;
                        let a352 = 39u64;
                        if nat_lt(a351, a352) {
                            2177026350u32
                        } else {
                            2456956037u32
                        }
                    }
                }
            } else {
                let a353 = v0;
                let a354 = 44u64;
                if nat_lt(a353, a354) {
                    let a355 = v0;
                    let a356 = 42u64;
                    if nat_lt(a355, a356) {
                        let a357 = v0;
                        let a358 = 41u64;
                        if nat_lt(a357, a358) {
                            2730485921u32
                        } else {
                            2820302411u32
                        }
                    } else {
                        let a359 = v0;
                        let a360 = 43u64;
                        if nat_lt(a359, a360) {
                            3259730800u32
                        } else {
                            3345764771u32
                        }
                    }
                } else {
                    let a361 = v0;
                    let a362 = 46u64;
                    if nat_lt(a361, a362) {
                        let a363 = v0;
                        let a364 = 45u64;
                        if nat_lt(a363, a364) {
                            3516065817u32
                        } else {
                            3600352804u32
                        }
                    } else {
                        let a365 = v0;
                        let a366 = 47u64;
                        if nat_lt(a365, a366) {
                            4094571909u32
                        } else {
                            275423344u32
                        }
                    }
                }
            }
        } else {
            let a367 = v0;
            let a368 = 56u64;
            if nat_lt(a367, a368) {
                let a369 = v0;
                let a370 = 52u64;
                if nat_lt(a369, a370) {
                    let a371 = v0;
                    let a372 = 50u64;
                    if nat_lt(a371, a372) {
                        let a373 = v0;
                        let a374 = 49u64;
                        if nat_lt(a373, a374) {
                            430227734u32
                        } else {
                            506948616u32
                        }
                    } else {
                        let a375 = v0;
                        let a376 = 51u64;
                        if nat_lt(a375, a376) {
                            659060556u32
                        } else {
                            883997877u32
                        }
                    }
                } else {
                    let a377 = v0;
                    let a378 = 54u64;
                    if nat_lt(a377, a378) {
                        let a379 = v0;
                        let a380 = 53u64;
                        if nat_lt(a379, a380) {
                            958139571u32
                        } else {
                            1322822218u32
                        }
                    } else {
                        let a381 = v0;
                        let a382 = 55u64;
                        if nat_lt(a381, a382) {
                            1537002063u32
                        } else {
                            1747873779u32
                        }
                    }
                }
            } else {
                let a383 = v0;
                let a384 = 60u64;
                if nat_lt(a383, a384) {
                    let a385 = v0;
                    let a386 = 58u64;
                    if nat_lt(a385, a386) {
                        let a387 = v0;
                        let a388 = 57u64;
                        if nat_lt(a387, a388) {
                            1955562222u32
                        } else {
                            2024104815u32
                        }
                    } else {
                        let a389 = v0;
                        let a390 = 59u64;
                        if nat_lt(a389, a390) {
                            2227730452u32
                        } else {
                            2361852424u32
                        }
                    }
                } else {
                    let a391 = v0;
                    let a392 = 62u64;
                    if nat_lt(a391, a392) {
                        let a393 = v0;
                        let a394 = 61u64;
                        if nat_lt(a393, a394) {
                            2428436474u32
                        } else {
                            2756734187u32
                        }
                    } else {
                        let a395 = v0;
                        let a396 = 63u64;
                        if nat_lt(a395, a396) {
                            3204031479u32
                        } else {
                            3329325298u32
                        }
                    }
                }
            }
        }
    }
}

pub fn f22(v0: u32) -> u32 {
    let a399 = {
        let a397 = f30(v0, 6u32, 26u32);
        let a398 = f30(v0, 11u32, 21u32);
        fixed_u32::bit_xor(a397, a398)
    };
    let a400 = f30(v0, 25u32, 7u32);
    fixed_u32::bit_xor(a399, a400)
}

pub fn f23(v0: u32, v1: u32, v2: u32) -> u32 {
    let a406 = {
        let a401 = v0;
        let a402 = v1;
        fixed_u32::bit_and(a401, a402)
    };
    let a407 = {
        let a404 = {
            let a403 = v0;
            fixed_u32::bit_not(a403)
        };
        let a405 = v2;
        fixed_u32::bit_and(a404, a405)
    };
    fixed_u32::bit_xor(a406, a407)
}

pub fn f24(v0: u32) -> u32 {
    let a410 = {
        let a408 = f30(v0, 2u32, 30u32);
        let a409 = f30(v0, 13u32, 19u32);
        fixed_u32::bit_xor(a408, a409)
    };
    let a411 = f30(v0, 22u32, 10u32);
    fixed_u32::bit_xor(a410, a411)
}

pub fn f25(v0: u32, v1: u32, v2: u32) -> u32 {
    let a420 = {
        let a416 = {
            let a412 = v0;
            let a413 = v1;
            fixed_u32::bit_and(a412, a413)
        };
        let a417 = {
            let a414 = v0;
            let a415 = v2;
            fixed_u32::bit_and(a414, a415)
        };
        fixed_u32::bit_xor(a416, a417)
    };
    let a421 = {
        let a418 = v1;
        let a419 = v2;
        fixed_u32::bit_and(a418, a419)
    };
    fixed_u32::bit_xor(a420, a421)
}

pub fn f26(v0: Adt1) -> Adt1 {
    Adt1::C0(match v0.clone() {
        Adt1::C0(_, h422, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h422
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h423, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h423
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h424, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h424
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h425, _, _, _, _, _, _, _, _, _, _, _) => {
            h425
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, h426, _, _, _, _, _, _, _, _, _, _) => {
            h426
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h427, _, _, _, _, _, _, _, _, _) => {
            h427
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h428, _, _, _, _, _, _, _, _) => {
            h428
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, h429, _, _, _, _, _, _, _) => {
            h429
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, h430, _, _, _, _, _, _) => {
            h430
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, h431, _, _, _, _, _) => {
            h431
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, h432, _, _, _, _) => {
            h432
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, h433, _, _, _) => {
            h433
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h434, _, _) => {
            h434
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h435, _) => {
            h435
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h436) => {
            h436
        }
    }, f13(f31(match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h437, _) => {
            h437
        }
    }), f13(match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, h438, _, _, _, _, _, _) => {
            h438
        }
    }, f13(f32(match v0.clone() {
        Adt1::C0(_, h439, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h439
        }
    }), match v0 {
        Adt1::C0(h440, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h440
        }
    }))))
}

pub fn f27(v0: Fn2, p444: u64, p445: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m443 = (p444, p445);
    {
        let mut m443 = m443;
        loop {
            let step = {
                let (v1, v2) = m443;
                let m441 = v1;
                if m441 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m441 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c442: Fn2 = v0.clone();
                        c442.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m443 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f28(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h446, _) = v0;
        h446
    };
    let v2: u32 = {
        let (h447, _) = {
            let (_, h448) = v0;
            h448
        };
        h447
    };
    let v3: u32 = {
        let (_, h449) = {
            let (_, h450) = v0;
            h450
        };
        h449
    };
    ({
        let a451 = v1;
        let a452 = 2u64;
        let a453 = 0u64;
        nat_quot(a451, a452, a453)
    }, (f29(v2, 1u32), {
        let a457 = {
            let a454 = v1;
            let a455 = 2u64;
            let a456 = 0u64;
            nat_rem(a454, a455, a456)
        };
        let a458 = 1u64;
        if nat_eq(a457, a458) {
            let a459 = v3;
            let a460 = v2;
            fixed_u32::bit_or(a459, a460)
        } else {
            v3
        }
    }))
}

pub fn f29(v0: u32, v1: u32) -> u32 {
    let m463 = {
        let a461 = v0;
        let a462 = v1;
        fixed_u32::shift_left(a461, a462)
    };
    match m463 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f30(v0: u32, v1: u32, v2: u32) -> u32 {
    let a464 = f8(v0, v1);
    let a465 = f29(v0, v2);
    fixed_u32::bit_or(a464, a465)
}

pub fn f31(v0: u32) -> u32 {
    let a468 = {
        let a466 = f30(v0, 17u32, 15u32);
        let a467 = f30(v0, 19u32, 13u32);
        fixed_u32::bit_xor(a466, a467)
    };
    let a469 = f8(v0, 10u32);
    fixed_u32::bit_xor(a468, a469)
}

pub fn f32(v0: u32) -> u32 {
    let a472 = {
        let a470 = f30(v0, 7u32, 25u32);
        let a471 = f30(v0, 18u32, 14u32);
        fixed_u32::bit_xor(a470, a471)
    };
    let a473 = f8(v0, 3u32);
    fixed_u32::bit_xor(a472, a473)
}
