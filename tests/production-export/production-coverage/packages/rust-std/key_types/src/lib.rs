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

pub fn f0(v0: List<((u64, Str), bool)>, v1: (u64, Str), v2: List<(i8, u64)>, v3: i8, v4: List<(bool, u64)>, v5: bool, v6: List<(i64, u64)>, v7: i64, v8: List<(u64, u64)>, v9: u64) -> (Option<bool>, (Option<u64>, (Option<u64>, (Option<u64>, Option<u64>)))) {
    (f1(v0, v1), (f2(v2, v3), (f3(v4, v5), (f4(v6, v7), f5(v8, v9)))))
}

pub fn f1(p8: List<((u64, Str), bool)>, v1: (u64, Str)) -> Option<bool> {
    let m7 = p8;
    {
        let mut m7 = m7;
        loop {
            let step = {
                let v0 = m7;
                let m1 = v0;
                match m1.uncons() {
                    None => {
                        Err::<List<((u64, Str), bool)>, Option<bool>>(None::<bool>)
                    }
                    Some((v2, v3)) => {
                        let m5 = {
                            let a3 = v1.clone();
                            let a4 = {
                                let (h2, _) = v2;
                                h2
                            };
                            compare(a3, a4)
                        };
                        match m5 {
                            Ordering::Less => {
                                Err::<List<((u64, Str), bool)>, Option<bool>>(None::<bool>)
                            }
                            Ordering::Equal => {
                                Err::<List<((u64, Str), bool)>, Option<bool>>(Some({
                                    let (_, h6) = v2;
                                    h6
                                }))
                            }
                            Ordering::Greater => {
                                Ok::<List<((u64, Str), bool)>, Option<bool>>(v3)
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m7 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f2(p16: List<(i8, u64)>, v1: i8) -> Option<u64> {
    let m15 = p16;
    {
        let mut m15 = m15;
        loop {
            let step = {
                let v0 = m15;
                let m9 = v0;
                match m9.uncons() {
                    None => {
                        Err::<List<(i8, u64)>, Option<u64>>(None::<u64>)
                    }
                    Some((v2, v3)) => {
                        let m13 = {
                            let a11 = v1;
                            let a12 = {
                                let (h10, _) = v2;
                                h10
                            };
                            compare(a11, a12)
                        };
                        match m13 {
                            Ordering::Less => {
                                Err::<List<(i8, u64)>, Option<u64>>(None::<u64>)
                            }
                            Ordering::Equal => {
                                Err::<List<(i8, u64)>, Option<u64>>(Some({
                                    let (_, h14) = v2;
                                    h14
                                }))
                            }
                            Ordering::Greater => {
                                Ok::<List<(i8, u64)>, Option<u64>>(v3)
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

pub fn f3(p24: List<(bool, u64)>, v1: bool) -> Option<u64> {
    let m23 = p24;
    {
        let mut m23 = m23;
        loop {
            let step = {
                let v0 = m23;
                let m17 = v0;
                match m17.uncons() {
                    None => {
                        Err::<List<(bool, u64)>, Option<u64>>(None::<u64>)
                    }
                    Some((v2, v3)) => {
                        let m21 = {
                            let a19 = v1;
                            let a20 = {
                                let (h18, _) = v2;
                                h18
                            };
                            compare(a19, a20)
                        };
                        match m21 {
                            Ordering::Less => {
                                Err::<List<(bool, u64)>, Option<u64>>(None::<u64>)
                            }
                            Ordering::Equal => {
                                Err::<List<(bool, u64)>, Option<u64>>(Some({
                                    let (_, h22) = v2;
                                    h22
                                }))
                            }
                            Ordering::Greater => {
                                Ok::<List<(bool, u64)>, Option<u64>>(v3)
                            }
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

pub fn f4(p32: List<(i64, u64)>, v1: i64) -> Option<u64> {
    let m31 = p32;
    {
        let mut m31 = m31;
        loop {
            let step = {
                let v0 = m31;
                let m25 = v0;
                match m25.uncons() {
                    None => {
                        Err::<List<(i64, u64)>, Option<u64>>(None::<u64>)
                    }
                    Some((v2, v3)) => {
                        let m29 = {
                            let a27 = v1;
                            let a28 = {
                                let (h26, _) = v2;
                                h26
                            };
                            compare(a27, a28)
                        };
                        match m29 {
                            Ordering::Less => {
                                Err::<List<(i64, u64)>, Option<u64>>(None::<u64>)
                            }
                            Ordering::Equal => {
                                Err::<List<(i64, u64)>, Option<u64>>(Some({
                                    let (_, h30) = v2;
                                    h30
                                }))
                            }
                            Ordering::Greater => {
                                Ok::<List<(i64, u64)>, Option<u64>>(v3)
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m31 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f5(p40: List<(u64, u64)>, v1: u64) -> Option<u64> {
    let m39 = p40;
    {
        let mut m39 = m39;
        loop {
            let step = {
                let v0 = m39;
                let m33 = v0;
                match m33.uncons() {
                    None => {
                        Err::<List<(u64, u64)>, Option<u64>>(None::<u64>)
                    }
                    Some((v2, v3)) => {
                        let m37 = {
                            let a35 = v1;
                            let a36 = {
                                let (h34, _) = v2;
                                h34
                            };
                            compare(a35, a36)
                        };
                        match m37 {
                            Ordering::Less => {
                                Err::<List<(u64, u64)>, Option<u64>>(None::<u64>)
                            }
                            Ordering::Equal => {
                                Err::<List<(u64, u64)>, Option<u64>>(Some({
                                    let (_, h38) = v2;
                                    h38
                                }))
                            }
                            Ordering::Greater => {
                                Ok::<List<(u64, u64)>, Option<u64>>(v3)
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m39 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f6(p50: List<((u64, Str), bool)>) -> bool {
    let m49 = p50;
    {
        let mut m49 = m49;
        loop {
            let step = {
                let v0 = m49;
                let m41 = v0;
                match m41.uncons() {
                    None => {
                        Err::<List<((u64, Str), bool)>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f7({
                            let (_, h42) = v1;
                            h42
                        }) {
                            let m43 = v2.clone();
                            match m43.uncons() {
                                None => {
                                    Err::<List<((u64, Str), bool)>, bool>(true)
                                }
                                Some((v3, _)) => {
                                    let m48 = {
                                        let a46 = {
                                            let (h44, _) = v1;
                                            h44
                                        };
                                        let a47 = {
                                            let (h45, _) = v3;
                                            h45
                                        };
                                        compare(a46, a47)
                                    };
                                    if match m48 {
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
                                        Ok::<List<((u64, Str), bool)>, bool>(v2)
                                    } else {
                                        Err::<List<((u64, Str), bool)>, bool>(false)
                                    }
                                }
                            }
                        } else {
                            Err::<List<((u64, Str), bool)>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m49 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f7(_: bool) -> bool {
    true
}

pub fn f8(p60: List<(i8, u64)>) -> bool {
    let m59 = p60;
    {
        let mut m59 = m59;
        loop {
            let step = {
                let v0 = m59;
                let m51 = v0;
                match m51.uncons() {
                    None => {
                        Err::<List<(i8, u64)>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f9({
                            let (_, h52) = v1;
                            h52
                        }) {
                            let m53 = v2.clone();
                            match m53.uncons() {
                                None => {
                                    Err::<List<(i8, u64)>, bool>(true)
                                }
                                Some((v3, _)) => {
                                    let m58 = {
                                        let a56 = {
                                            let (h54, _) = v1;
                                            h54
                                        };
                                        let a57 = {
                                            let (h55, _) = v3;
                                            h55
                                        };
                                        compare(a56, a57)
                                    };
                                    if match m58 {
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
                                        Ok::<List<(i8, u64)>, bool>(v2)
                                    } else {
                                        Err::<List<(i8, u64)>, bool>(false)
                                    }
                                }
                            }
                        } else {
                            Err::<List<(i8, u64)>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m59 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f9(_: u64) -> bool {
    true
}

pub fn f10(p70: List<(bool, u64)>) -> bool {
    let m69 = p70;
    {
        let mut m69 = m69;
        loop {
            let step = {
                let v0 = m69;
                let m61 = v0;
                match m61.uncons() {
                    None => {
                        Err::<List<(bool, u64)>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f9({
                            let (_, h62) = v1;
                            h62
                        }) {
                            let m63 = v2.clone();
                            match m63.uncons() {
                                None => {
                                    Err::<List<(bool, u64)>, bool>(true)
                                }
                                Some((v3, _)) => {
                                    let m68 = {
                                        let a66 = {
                                            let (h64, _) = v1;
                                            h64
                                        };
                                        let a67 = {
                                            let (h65, _) = v3;
                                            h65
                                        };
                                        compare(a66, a67)
                                    };
                                    if match m68 {
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
                                        Ok::<List<(bool, u64)>, bool>(v2)
                                    } else {
                                        Err::<List<(bool, u64)>, bool>(false)
                                    }
                                }
                            }
                        } else {
                            Err::<List<(bool, u64)>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m69 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f11(p80: List<(i64, u64)>) -> bool {
    let m79 = p80;
    {
        let mut m79 = m79;
        loop {
            let step = {
                let v0 = m79;
                let m71 = v0;
                match m71.uncons() {
                    None => {
                        Err::<List<(i64, u64)>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f9({
                            let (_, h72) = v1;
                            h72
                        }) {
                            let m73 = v2.clone();
                            match m73.uncons() {
                                None => {
                                    Err::<List<(i64, u64)>, bool>(true)
                                }
                                Some((v3, _)) => {
                                    let m78 = {
                                        let a76 = {
                                            let (h74, _) = v1;
                                            h74
                                        };
                                        let a77 = {
                                            let (h75, _) = v3;
                                            h75
                                        };
                                        compare(a76, a77)
                                    };
                                    if match m78 {
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
                                        Ok::<List<(i64, u64)>, bool>(v2)
                                    } else {
                                        Err::<List<(i64, u64)>, bool>(false)
                                    }
                                }
                            }
                        } else {
                            Err::<List<(i64, u64)>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m79 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f12(p90: List<(u64, u64)>) -> bool {
    let m89 = p90;
    {
        let mut m89 = m89;
        loop {
            let step = {
                let v0 = m89;
                let m81 = v0;
                match m81.uncons() {
                    None => {
                        Err::<List<(u64, u64)>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f9({
                            let (_, h82) = v1;
                            h82
                        }) {
                            let m83 = v2.clone();
                            match m83.uncons() {
                                None => {
                                    Err::<List<(u64, u64)>, bool>(true)
                                }
                                Some((v3, _)) => {
                                    let m88 = {
                                        let a86 = {
                                            let (h84, _) = v1;
                                            h84
                                        };
                                        let a87 = {
                                            let (h85, _) = v3;
                                            h85
                                        };
                                        compare(a86, a87)
                                    };
                                    if match m88 {
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
                                        Ok::<List<(u64, u64)>, bool>(v2)
                                    } else {
                                        Err::<List<(u64, u64)>, bool>(false)
                                    }
                                }
                            }
                        } else {
                            Err::<List<(u64, u64)>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m89 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f13(v0: List<((u64, Str), bool)>, v1: (u64, Str), v2: List<(i8, u64)>, v3: i8, v4: List<(bool, u64)>, v5: bool, v6: List<(i64, u64)>, v7: i64, v8: List<(u64, u64)>, v9: u64) -> Option<(Option<bool>, (Option<u64>, (Option<u64>, (Option<u64>, Option<u64>))))> {
    if f6(v0.clone()) {
        if f8(v2.clone()) {
            if f10(v4.clone()) {
                if f11(v6.clone()) {
                    if f12(v8.clone()) {
                        Some(f0(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9))
                    } else {
                        None::<(Option<bool>, (Option<u64>, (Option<u64>, (Option<u64>, Option<u64>))))>
                    }
                } else {
                    None::<(Option<bool>, (Option<u64>, (Option<u64>, (Option<u64>, Option<u64>))))>
                }
            } else {
                None::<(Option<bool>, (Option<u64>, (Option<u64>, (Option<u64>, Option<u64>))))>
            }
        } else {
            None::<(Option<bool>, (Option<u64>, (Option<u64>, (Option<u64>, Option<u64>))))>
        }
    } else {
        None::<(Option<bool>, (Option<u64>, (Option<u64>, (Option<u64>, Option<u64>))))>
    }
}

pub fn run(p0: List<((u64, Str), bool)>, p1: (u64, Str), p2: List<(i8, u64)>, p3: i8, p4: List<(bool, u64)>, p5: bool, p6: List<(i64, u64)>, p7: i64, p8: List<(u64, u64)>, p9: u64) -> Option<(Option<bool>, (Option<u64>, (Option<u64>, (Option<u64>, Option<u64>))))> {
    f13(p0, p1, p2, p3, p4, p5, p6, p7, p8, p9)
}
