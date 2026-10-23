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
    F53(Bytes),
    F54(Bytes),
}

impl Fn2 {
    pub fn apply(&self, p0: (u64, Adt20)) -> R<Option<(u64, Adt20)>> {
        match self {
            Fn2::F45(k0) => f45(*k0, p0),
            Fn2::F53(k0) => f53(k0.clone(), p0),
            Fn2::F54(k0) => f54(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F60,
}

impl Fn3 {
    pub fn apply(&self, p0: (Option<Bytes>, List<Bytes>), p1: Bytes) -> (Option<Bytes>, List<Bytes>) {
        match self {
            Fn3::F60 => f60(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F65(Bytes),
    F75(Bytes),
}

impl Fn4 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn4::F65(k0) => f65(k0.clone(), p0),
            Fn4::F75(k0) => f75(k0.clone(), p0),
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
        let a144 = 26u64;
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
            if nat_eq(a148, a149) {
                1u64
            } else {
                let a150 = v0;
                let a151 = 15u64;
                if nat_eq(a150, a151) {
                    3u64
                } else {
                    let a152 = v0;
                    let a153 = 16u64;
                    if nat_eq(a152, a153) {
                        3u64
                    } else {
                        let a154 = v0;
                        let a155 = 20u64;
                        if nat_eq(a154, a155) {
                            3u64
                        } else {
                            let a156 = v0;
                            let a157 = 25u64;
                            if nat_eq(a156, a157) {
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
    let a158 = v1;
    let a159 = 8u64;
    if nat_lt(a158, a159) {
        f8(v0, Str::lit(", "))
    } else {
        let a160 = v1;
        let a161 = 100u64;
        if nat_eq(a160, a161) {
            f8(v0, Str::lit(", "))
        } else {
            let a162 = v2;
            let a163 = 0u64;
            if nat_eq(a162, a163) {
                f8(v0, f46(v1))
            } else {
                f8(v0, f47(v1))
            }
        }
    }
}

pub fn f25(v0: Adt20, v1: Adt17) -> R<Adt20> {
    let m164 = v1;
    match m164 {
        Adt17::C0(v2) => {
            f8(v0, {
                let m165 = v2;
                match m165 {
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
        Adt17::C2(r169) => {
            let v4 = (*r169).clone();
            f8(f25(f8(v0, Str::lit("("))?, v4)?, Str::lit(")"))
        }
        Adt17::C3(v5, r170) => {
            let v6 = (*r170).clone();
            f8(f25(f8(v0, {
                let m166 = v5;
                match m166 {
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
        Adt17::C4(v7, v8, r171) => {
            let v9 = (*r171).clone();
            let v10: Adt20 = f11(f8(f11(f8(v0, Str::lit("Adt"))?, v7)?, Str::lit("::C"))?, v8)?;
            let m167 = v9.clone();
            match m167 {
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
        Adt17::C6(r172, r173) => {
            let v22 = (*r172).clone();
            let v23 = (*r173).clone();
            let m168 = v23.clone();
            match m168 {
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
    let m174 = v1;
    match m174 {
        Adt11::C0(v2) => {
            f48(v0, v2)
        }
        Adt11::C1(v3) => {
            f32(v0, v3)
        }
        Adt11::C2(v4, r178) => {
            let v5 = (*r178).clone();
            f8(f26(f8(v0, f49(v4.clone()))?, v5)?, f50(v4))
        }
        Adt11::C3(v6, r179, r180) => {
            let v7 = (*r179).clone();
            let v8 = (*r180).clone();
            let m175 = v6;
            match m175 {
                Adt14::C0 => {
                    f8(f26(f8(f26(f8(v0, Str::lit("Result<"))?, v7)?, Str::lit(", "))?, v8)?, Str::lit(">"))
                }
                Adt14::C1 => {
                    f8(f26(f8(f26(f8(v0, Str::lit("("))?, v7)?, Str::lit(", "))?, v8)?, Str::lit(")"))
                }
            }
        }
        Adt11::C4(v9, v10) => {
            let m176 = v9;
            match m176 {
                Adt15::C0 => {
                    f11(f8(v0, Str::lit("Adt"))?, v10)
                }
                Adt15::C1 => {
                    f11(f8(v0, Str::lit("Fn"))?, v10)
                }
            }
        }
        Adt11::C5 => {
            Ok(v0)
        }
        Adt11::C6(r181, r182) => {
            let v11 = (*r181).clone();
            let v12 = (*r182).clone();
            let m177 = v12.clone();
            match m177 {
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

pub fn f27(v0: Fn0, p187: u64, p188: List<Bytes>) -> (List<Bytes>, bool) {
    let m186 = (p187, p188);
    {
        let mut m186 = m186;
        loop {
            let step = {
                let (v1, v2) = m186;
                let m183 = v1;
                if m183 == 0 {
                    Err::<(u64, List<Bytes>), (List<Bytes>, bool)>((v2, false))
                } else {
                    let v3 = m183 - 1;
                    let m185 = {
                        let c184: Fn0 = v0.clone();
                        c184.apply(v2.clone())
                    };
                    match m185 {
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
                Ok(next) => { m186 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f28(v0: List<Bytes>) -> Option<List<Bytes>> {
    f51(v0)
}

pub fn f29(v0: Fn1, p192: List<Bytes>, p193: List<Bytes>) -> List<Bytes> {
    let m191 = (p192, p193);
    {
        let mut m191 = m191;
        loop {
            let step = {
                let (v1, v2) = m191;
                let m189 = v2;
                match m189.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c190: Fn1 = v0.clone();
                            c190.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m191 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f30(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f31(v0: Adt2) -> u64 {
    let m194 = v0;
    match m194 {
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
    f8(v0, f52(v1))
}

pub fn f33(v0: Adt20, v1: Bytes) -> R<Adt20> {
    f8({
        let (_, h195) = {
            let (h196, _) = f44(Fn2::F53(v1.clone()), {
                let a197 = v1;
                length_bytes(a197)
            }, (0u64, f8(v0, Str::lit("\""))?))?;
            h196
        };
        h195
    }, Str::lit("\""))
}

pub fn f34(v0: Adt20, v1: Bytes) -> R<Adt20> {
    f8({
        let (_, h198) = {
            let (h199, _) = f44(Fn2::F54(v1.clone()), {
                let a200 = v1;
                length_bytes(a200)
            }, (0u64, f8(v0, Str::lit("Bytes::lit(&["))?))?;
            h199
        };
        h198
    }, Str::lit("])"))
}

pub fn f35(v0: Adt20, v1: Adt3) -> R<Adt20> {
    f8(v0, {
        let m201 = v1;
        match m201 {
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
        Adt20::C0(_, _, _, _, h202) => {
            h202
        }
    } {
        Ok(v0)
    } else {
        let v2: u64 = {
            let a205 = match v0.clone() {
                Adt20::C0(_, _, h203, _, _) => {
                    h203
                }
            };
            let a206 = {
                let a204 = v1.clone();
                length_bytes(a204)
            };
            nat_add(a205, a206)?
        };
        let a208 = v2;
        let a209 = match v0.clone() {
            Adt20::C0(_, _, _, h207, _) => {
                h207
            }
        };
        if nat_le(a208, a209) {
            Ok(f55(v0.clone(), {
                let a211 = match v0 {
                    Adt20::C0(_, h210, _, _, _) => {
                        h210
                    }
                };
                let a212 = v1;
                append_bytes(a211, a212)
            }, v2))
        } else {
            Ok(f56(v0))
        }
    }
}

pub fn f37(v0: Adt20, v1: u64) -> R<Adt20> {
    f36(v0, f57(v1)?)
}

pub fn f38(v0: u64) -> Str {
    let a213 = v0;
    let a214 = 20u64;
    if nat_eq(a213, a214) {
        Str::lit("checked_add")
    } else {
        let a215 = v0;
        let a216 = 21u64;
        if nat_eq(a215, a216) {
            Str::lit("checked_sub")
        } else {
            let a217 = v0;
            let a218 = 22u64;
            if nat_eq(a217, a218) {
                Str::lit("checked_mul")
            } else {
                let a219 = v0;
                let a220 = 23u64;
                if nat_eq(a219, a220) {
                    Str::lit("checked_quot")
                } else {
                    let a221 = v0;
                    let a222 = 25u64;
                    if nat_eq(a221, a222) {
                        Str::lit("bit_and")
                    } else {
                        let a223 = v0;
                        let a224 = 26u64;
                        if nat_eq(a223, a224) {
                            Str::lit("bit_or")
                        } else {
                            let a225 = v0;
                            let a226 = 27u64;
                            if nat_eq(a225, a226) {
                                Str::lit("bit_xor")
                            } else {
                                let a227 = v0;
                                let a228 = 28u64;
                                if nat_eq(a227, a228) {
                                    Str::lit("bit_not")
                                } else {
                                    let a229 = v0;
                                    let a230 = 29u64;
                                    if nat_eq(a229, a230) {
                                        Str::lit("shift_left")
                                    } else {
                                        let a231 = v0;
                                        let a232 = 30u64;
                                        if nat_eq(a231, a232) {
                                            Str::lit("shift_right")
                                        } else {
                                            let a233 = v0;
                                            let a234 = 31u64;
                                            if nat_eq(a233, a234) {
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
    let a235 = v0;
    let a236 = 0u64;
    if nat_eq(a235, a236) {
        Str::lit("nat_add")
    } else {
        let a237 = v0;
        let a238 = 1u64;
        if nat_eq(a237, a238) {
            Str::lit("nat_sub")
        } else {
            let a239 = v0;
            let a240 = 2u64;
            if nat_eq(a239, a240) {
                Str::lit("nat_mul")
            } else {
                let a241 = v0;
                let a242 = 3u64;
                if nat_eq(a241, a242) {
                    Str::lit("nat_quot")
                } else {
                    let a243 = v0;
                    let a244 = 4u64;
                    if nat_eq(a243, a244) {
                        Str::lit("nat_rem")
                    } else {
                        let a245 = v0;
                        let a246 = 5u64;
                        if nat_eq(a245, a246) {
                            Str::lit("nat_eq")
                        } else {
                            let a247 = v0;
                            let a248 = 6u64;
                            if nat_eq(a247, a248) {
                                Str::lit("nat_le")
                            } else {
                                let a249 = v0;
                                let a250 = 7u64;
                                if nat_eq(a249, a250) {
                                    Str::lit("nat_lt")
                                } else {
                                    let a251 = v0;
                                    let a252 = 8u64;
                                    if nat_eq(a251, a252) {
                                        Str::lit("nat_succ")
                                    } else {
                                        let a253 = v0;
                                        let a254 = 9u64;
                                        if nat_eq(a253, a254) {
                                            Str::lit("int_add")
                                        } else {
                                            let a255 = v0;
                                            let a256 = 10u64;
                                            if nat_eq(a255, a256) {
                                                Str::lit("int_sub")
                                            } else {
                                                let a257 = v0;
                                                let a258 = 11u64;
                                                if nat_eq(a257, a258) {
                                                    Str::lit("int_mul")
                                                } else {
                                                    let a259 = v0;
                                                    let a260 = 12u64;
                                                    if nat_eq(a259, a260) {
                                                        Str::lit("int_neg")
                                                    } else {
                                                        let a261 = v0;
                                                        let a262 = 13u64;
                                                        if nat_eq(a261, a262) {
                                                            Str::lit("int_quot")
                                                        } else {
                                                            let a263 = v0;
                                                            let a264 = 14u64;
                                                            if nat_eq(a263, a264) {
                                                                Str::lit("int_rem")
                                                            } else {
                                                                let a265 = v0;
                                                                let a266 = 15u64;
                                                                if nat_eq(a265, a266) {
                                                                    Str::lit("bool_not")
                                                                } else {
                                                                    let a267 = v0;
                                                                    let a268 = 16u64;
                                                                    if nat_eq(a267, a268) {
                                                                        Str::lit("bool_and")
                                                                    } else {
                                                                        let a269 = v0;
                                                                        let a270 = 17u64;
                                                                        if nat_eq(a269, a270) {
                                                                            Str::lit("bool_or")
                                                                        } else {
                                                                            let a271 = v0;
                                                                            let a272 = 18u64;
                                                                            if nat_eq(a271, a272) {
                                                                                Str::lit("equal")
                                                                            } else {
                                                                                let a273 = v0;
                                                                                let a274 = 19u64;
                                                                                if nat_eq(a273, a274) {
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
    let a275 = v0;
    let a276 = 32u64;
    if nat_eq(a275, a276) {
        Str::lit("append_list")
    } else {
        let a277 = v0;
        let a278 = 33u64;
        if nat_eq(a277, a278) {
            Str::lit("append_bytes")
        } else {
            let a279 = v0;
            let a280 = 34u64;
            if nat_eq(a279, a280) {
                Str::lit("length_list")
            } else {
                let a281 = v0;
                let a282 = 35u64;
                if nat_eq(a281, a282) {
                    Str::lit("length_bytes")
                } else {
                    let a283 = v0;
                    let a284 = 36u64;
                    if nat_eq(a283, a284) {
                        Str::lit("length_string")
                    } else {
                        let a285 = v0;
                        let a286 = 37u64;
                        if nat_eq(a285, a286) {
                            Str::lit("index_list")
                        } else {
                            let a287 = v0;
                            let a288 = 38u64;
                            if nat_eq(a287, a288) {
                                Str::lit("index_bytes")
                            } else {
                                let a289 = v0;
                                let a290 = 39u64;
                                if nat_eq(a289, a290) {
                                    Str::lit("slice_list")
                                } else {
                                    let a291 = v0;
                                    let a292 = 40u64;
                                    if nat_eq(a291, a292) {
                                        Str::lit("slice_bytes")
                                    } else {
                                        let a293 = v0;
                                        let a294 = 41u64;
                                        if nat_eq(a293, a294) {
                                            Str::lit("utf8_encode")
                                        } else {
                                            let a295 = v0;
                                            let a296 = 42u64;
                                            if nat_eq(a295, a296) {
                                                Str::lit("utf8_decode")
                                            } else {
                                                let a297 = v0;
                                                let a298 = 43u64;
                                                if nat_eq(a297, a298) {
                                                    Str::lit("compare_bytes")
                                                } else {
                                                    let a299 = v0;
                                                    let a300 = 44u64;
                                                    if nat_eq(a299, a300) {
                                                        Str::lit("split_exact")
                                                    } else {
                                                        let a301 = v0;
                                                        let a302 = 45u64;
                                                        if nat_eq(a301, a302) {
                                                            Str::lit("join")
                                                        } else {
                                                            let a303 = v0;
                                                            let a304 = 46u64;
                                                            if nat_eq(a303, a304) {
                                                                Str::lit("format_i64")
                                                            } else {
                                                                let a305 = v0;
                                                                let a306 = 48u64;
                                                                if nat_eq(a305, a306) {
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
    let a307 = v0;
    let a308 = 0u64;
    if nat_eq(a307, a308) {
        Str::lit("checked_add")
    } else {
        let a309 = v0;
        let a310 = 1u64;
        if nat_eq(a309, a310) {
            Str::lit("checked_sub")
        } else {
            let a311 = v0;
            let a312 = 2u64;
            if nat_eq(a311, a312) {
                Str::lit("checked_mul")
            } else {
                let a313 = v0;
                let a314 = 3u64;
                if nat_eq(a313, a314) {
                    Str::lit("checked_div")
                } else {
                    let a315 = v0;
                    let a316 = 4u64;
                    if nat_eq(a315, a316) {
                        Str::lit("checked_neg")
                    } else {
                        let a317 = v0;
                        let a318 = 5u64;
                        if nat_eq(a317, a318) {
                            Str::lit("checked_shl")
                        } else {
                            let a319 = v0;
                            let a320 = 6u64;
                            if nat_eq(a319, a320) {
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
    let a321 = v0;
    let a322 = 8u64;
    if nat_eq(a321, a322) {
        Str::lit("core::ops::BitAnd::bitand(")
    } else {
        let a323 = v0;
        let a324 = 9u64;
        if nat_eq(a323, a324) {
            Str::lit("core::ops::BitOr::bitor(")
        } else {
            let a325 = v0;
            let a326 = 10u64;
            if nat_eq(a325, a326) {
                Str::lit("core::ops::BitXor::bitxor(")
            } else {
                let a327 = v0;
                let a328 = 11u64;
                if nat_eq(a327, a328) {
                    Str::lit("core::ops::Not::not(")
                } else {
                    let a329 = v0;
                    let a330 = 12u64;
                    if nat_eq(a329, a330) {
                        Str::lit("core::cmp::PartialEq::eq(&")
                    } else {
                        let a331 = v0;
                        let a332 = 13u64;
                        if nat_eq(a331, a332) {
                            Str::lit("core::cmp::PartialOrd::le(&")
                        } else {
                            let a333 = v0;
                            let a334 = 14u64;
                            if nat_eq(a333, a334) {
                                Str::lit("core::cmp::PartialOrd::lt(&")
                            } else {
                                let a335 = v0;
                                let a336 = 15u64;
                                if nat_eq(a335, a336) {
                                    Str::lit("u64::checked_rem(")
                                } else {
                                    let a337 = v0;
                                    let a338 = 16u64;
                                    if nat_eq(a337, a338) {
                                        Str::lit("{ let (a, b, z): (i64, i64, i64) = (")
                                    } else {
                                        let a339 = v0;
                                        let a340 = 17u64;
                                        if nat_eq(a339, a340) {
                                            Str::lit("u64::checked_add(")
                                        } else {
                                            let a341 = v0;
                                            let a342 = 18u64;
                                            if nat_eq(a341, a342) {
                                                Str::lit("u64::saturating_sub(")
                                            } else {
                                                let a343 = v0;
                                                let a344 = 19u64;
                                                if nat_eq(a343, a344) {
                                                    Str::lit("u64::checked_mul(")
                                                } else {
                                                    let a345 = v0;
                                                    let a346 = 20u64;
                                                    if nat_eq(a345, a346) {
                                                        Str::lit("u64::checked_div(")
                                                    } else {
                                                        let a347 = v0;
                                                        let a348 = 21u64;
                                                        if nat_eq(a347, a348) {
                                                            Str::lit("i64::checked_add(")
                                                        } else {
                                                            let a349 = v0;
                                                            let a350 = 22u64;
                                                            if nat_eq(a349, a350) {
                                                                Str::lit("i64::checked_sub(")
                                                            } else {
                                                                let a351 = v0;
                                                                let a352 = 23u64;
                                                                if nat_eq(a351, a352) {
                                                                    Str::lit("i64::checked_mul(")
                                                                } else {
                                                                    let a353 = v0;
                                                                    let a354 = 24u64;
                                                                    if nat_eq(a353, a354) {
                                                                        Str::lit("i64::checked_neg(")
                                                                    } else {
                                                                        let a355 = v0;
                                                                        let a356 = 25u64;
                                                                        if nat_eq(a355, a356) {
                                                                            Str::lit("{ let (a, b, z): (i64, i64, i64) = (")
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

pub fn f43(v0: u64) -> Str {
    let a357 = v0;
    let a358 = 16u64;
    if nat_eq(a357, a358) {
        Str::lit("); if b == 0 { z } else { a.wrapping_rem(b) } }")
    } else {
        let a359 = v0;
        let a360 = 25u64;
        if nat_eq(a359, a360) {
            Str::lit("); if b == 0 { Some(z) } else { a.checked_div(b) } }")
        } else {
            let a361 = v0;
            let a362 = 15u64;
            if nat_eq(a361, a362) {
                Str::lit(")")
            } else {
                let a363 = v0;
                let a364 = 20u64;
                if nat_eq(a363, a364) {
                    Str::lit(")")
                } else {
                    let a365 = v0;
                    let a366 = 8u64;
                    if nat_eq(a365, a366) {
                        Str::lit(")")
                    } else {
                        let a367 = v0;
                        let a368 = 9u64;
                        if nat_eq(a367, a368) {
                            Str::lit(")")
                        } else {
                            let a369 = v0;
                            let a370 = 10u64;
                            if nat_eq(a369, a370) {
                                Str::lit(")")
                            } else {
                                let a371 = v0;
                                let a372 = 11u64;
                                if nat_eq(a371, a372) {
                                    Str::lit(")")
                                } else {
                                    let a373 = v0;
                                    let a374 = 12u64;
                                    if nat_eq(a373, a374) {
                                        Str::lit(")")
                                    } else {
                                        let a375 = v0;
                                        let a376 = 13u64;
                                        if nat_eq(a375, a376) {
                                            Str::lit(")")
                                        } else {
                                            let a377 = v0;
                                            let a378 = 14u64;
                                            if nat_eq(a377, a378) {
                                                Str::lit(")")
                                            } else {
                                                let a379 = v0;
                                                let a380 = 17u64;
                                                if nat_eq(a379, a380) {
                                                    Str::lit(")")
                                                } else {
                                                    let a381 = v0;
                                                    let a382 = 18u64;
                                                    if nat_eq(a381, a382) {
                                                        Str::lit(")")
                                                    } else {
                                                        let a383 = v0;
                                                        let a384 = 19u64;
                                                        if nat_eq(a383, a384) {
                                                            Str::lit(")")
                                                        } else {
                                                            let a385 = v0;
                                                            let a386 = 21u64;
                                                            if nat_eq(a385, a386) {
                                                                Str::lit(")")
                                                            } else {
                                                                let a387 = v0;
                                                                let a388 = 22u64;
                                                                if nat_eq(a387, a388) {
                                                                    Str::lit(")")
                                                                } else {
                                                                    let a389 = v0;
                                                                    let a390 = 23u64;
                                                                    if nat_eq(a389, a390) {
                                                                        Str::lit(")")
                                                                    } else {
                                                                        let a391 = v0;
                                                                        let a392 = 24u64;
                                                                        let _ = nat_eq(a391, a392);
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

pub fn f44(v0: Fn2, p397: u64, p398: (u64, Adt20)) -> R<((u64, Adt20), bool)> {
    let m396 = (p397, p398);
    {
        let mut m396 = m396;
        loop {
            let step = {
                let (v1, v2) = m396;
                let m393 = v1;
                if m393 == 0 {
                    Err::<(u64, (u64, Adt20)), R<((u64, Adt20), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m393 - 1;
                    let m395 = {
                        let c394: Fn2 = v0.clone();
                        c394.apply(v2.clone())?
                    };
                    match m395 {
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
                Ok(next) => { m396 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f45(v0: u64, v1: (u64, Adt20)) -> R<Option<(u64, Adt20)>> {
    f58(v0, v1)
}

pub fn f46(v0: u64) -> Str {
    let a399 = v0;
    let a400 = 12u64;
    if nat_eq(a399, a400) {
        Str::lit(", &")
    } else {
        let a401 = v0;
        let a402 = 13u64;
        if nat_eq(a401, a402) {
            Str::lit(", &")
        } else {
            let a403 = v0;
            let a404 = 14u64;
            if nat_eq(a403, a404) {
                Str::lit(", &")
            } else {
                let a405 = v0;
                let a406 = 8u64;
                if nat_eq(a405, a406) {
                    Str::lit(", ")
                } else {
                    let a407 = v0;
                    let a408 = 9u64;
                    if nat_eq(a407, a408) {
                        Str::lit(", ")
                    } else {
                        let a409 = v0;
                        let a410 = 10u64;
                        if nat_eq(a409, a410) {
                            Str::lit(", ")
                        } else {
                            let a411 = v0;
                            let a412 = 11u64;
                            if nat_eq(a411, a412) {
                                Str::lit(", ")
                            } else {
                                let a413 = v0;
                                let a414 = 15u64;
                                if nat_eq(a413, a414) {
                                    Str::lit(", ")
                                } else {
                                    let a415 = v0;
                                    let a416 = 16u64;
                                    if nat_eq(a415, a416) {
                                        Str::lit(", ")
                                    } else {
                                        let a417 = v0;
                                        let a418 = 17u64;
                                        if nat_eq(a417, a418) {
                                            Str::lit(", ")
                                        } else {
                                            let a419 = v0;
                                            let a420 = 18u64;
                                            if nat_eq(a419, a420) {
                                                Str::lit(", ")
                                            } else {
                                                let a421 = v0;
                                                let a422 = 19u64;
                                                if nat_eq(a421, a422) {
                                                    Str::lit(", ")
                                                } else {
                                                    let a423 = v0;
                                                    let a424 = 20u64;
                                                    if nat_eq(a423, a424) {
                                                        Str::lit(", ")
                                                    } else {
                                                        let a425 = v0;
                                                        let a426 = 21u64;
                                                        if nat_eq(a425, a426) {
                                                            Str::lit(", ")
                                                        } else {
                                                            let a427 = v0;
                                                            let a428 = 22u64;
                                                            if nat_eq(a427, a428) {
                                                                Str::lit(", ")
                                                            } else {
                                                                let a429 = v0;
                                                                let a430 = 23u64;
                                                                if nat_eq(a429, a430) {
                                                                    Str::lit(", ")
                                                                } else {
                                                                    let a431 = v0;
                                                                    let a432 = 24u64;
                                                                    if nat_eq(a431, a432) {
                                                                        Str::lit(", ")
                                                                    } else {
                                                                        let a433 = v0;
                                                                        let a434 = 25u64;
                                                                        let _ = nat_eq(a433, a434);
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
    let a435 = v0;
    let a436 = 15u64;
    if nat_eq(a435, a436) {
        Str::lit(").unwrap_or(")
    } else {
        let a437 = v0;
        let a438 = 16u64;
        if nat_eq(a437, a438) {
            Str::lit(", ")
        } else {
            let a439 = v0;
            let a440 = 20u64;
            if nat_eq(a439, a440) {
                Str::lit(").unwrap_or(")
            } else {
                let a441 = v0;
                let a442 = 25u64;
                let _ = nat_eq(a441, a442);
                Str::lit(", ")
            }
        }
    }
}

pub fn f48(v0: Adt20, v1: Adt12) -> R<Adt20> {
    f8(v0, {
        let m443 = v1;
        match m443 {
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
    let m444 = v0;
    match m444 {
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
    let m445 = v0;
    match m445 {
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

pub fn f51(v0: List<Bytes>) -> Option<List<Bytes>> {
    let m446 = v0.clone();
    match m446.uncons() {
        None => {
            None::<List<Bytes>>
        }
        Some((_, v2)) => {
            let m447 = v2;
            match m447.uncons() {
                None => {
                    None::<List<Bytes>>
                }
                Some((_, _)) => {
                    let v5: (Option<Bytes>, List<Bytes>) = f59(Fn3::F60, (None::<Bytes>, List::<Bytes>::nil()), v0);
                    let m449 = {
                        let (h448, _) = v5;
                        h448
                    };
                    match m449 {
                        None => {
                            Some(f5({
                                let (_, h451) = v5;
                                h451
                            }))
                        }
                        Some(v6) => {
                            Some(f5(List::cons(v6, {
                                let (_, h450) = v5;
                                h450
                            })))
                        }
                    }
                }
            }
        }
    }
}

pub fn f52(v0: Adt2) -> Str {
    let m452 = v0;
    match m452 {
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

pub fn f53(v0: Bytes, v1: (u64, Adt20)) -> R<Option<(u64, Adt20)>> {
    f61(v0, v1)
}

pub fn f54(v0: Bytes, v1: (u64, Adt20)) -> R<Option<(u64, Adt20)>> {
    f62(v0, v1)
}

pub fn f55(v0: Adt20, v1: Bytes, v2: u64) -> Adt20 {
    let a454 = 256u64;
    let a455 = {
        let a453 = v1.clone();
        length_bytes(a453)
    };
    if nat_le(a454, a455) {
        Adt20::C0(List::cons(v1, match v0.clone() {
            Adt20::C0(h456, _, _, _, _) => {
                h456
            }
        }), Bytes::lit(&[]), v2, match v0 {
            Adt20::C0(_, _, _, h457, _) => {
                h457
            }
        }, false)
    } else {
        Adt20::C0(match v0.clone() {
            Adt20::C0(h458, _, _, _, _) => {
                h458
            }
        }, v1, v2, match v0 {
            Adt20::C0(_, _, _, h459, _) => {
                h459
            }
        }, false)
    }
}

pub fn f56(v0: Adt20) -> Adt20 {
    Adt20::C0(match v0.clone() {
        Adt20::C0(h460, _, _, _, _) => {
            h460
        }
    }, match v0.clone() {
        Adt20::C0(_, h461, _, _, _) => {
            h461
        }
    }, match v0.clone() {
        Adt20::C0(_, _, h462, _, _) => {
            h462
        }
    }, match v0 {
        Adt20::C0(_, _, _, h463, _) => {
            h463
        }
    }, true)
}

pub fn f57(v0: u64) -> R<Bytes> {
    let v1: Bytes = {
        let a500 = f63(v0, 10000000000000000000u64);
        let a501 = {
            let a498 = f63(v0, 1000000000000000000u64);
            let a499 = {
                let a496 = f63(v0, 100000000000000000u64);
                let a497 = {
                    let a494 = f63(v0, 10000000000000000u64);
                    let a495 = {
                        let a492 = f63(v0, 1000000000000000u64);
                        let a493 = {
                            let a490 = f63(v0, 100000000000000u64);
                            let a491 = {
                                let a488 = f63(v0, 10000000000000u64);
                                let a489 = {
                                    let a486 = f63(v0, 1000000000000u64);
                                    let a487 = {
                                        let a484 = f63(v0, 100000000000u64);
                                        let a485 = {
                                            let a482 = f63(v0, 10000000000u64);
                                            let a483 = {
                                                let a480 = f63(v0, 1000000000u64);
                                                let a481 = {
                                                    let a478 = f63(v0, 100000000u64);
                                                    let a479 = {
                                                        let a476 = f63(v0, 10000000u64);
                                                        let a477 = {
                                                            let a474 = f63(v0, 1000000u64);
                                                            let a475 = {
                                                                let a472 = f63(v0, 100000u64);
                                                                let a473 = {
                                                                    let a470 = f63(v0, 10000u64);
                                                                    let a471 = {
                                                                        let a468 = f63(v0, 1000u64);
                                                                        let a469 = {
                                                                            let a466 = f63(v0, 100u64);
                                                                            let a467 = {
                                                                                let a464 = f63(v0, 10u64);
                                                                                let a465 = f63(v0, 1u64);
                                                                                append_bytes(a464, a465)
                                                                            };
                                                                            append_bytes(a466, a467)
                                                                        };
                                                                        append_bytes(a468, a469)
                                                                    };
                                                                    append_bytes(a470, a471)
                                                                };
                                                                append_bytes(a472, a473)
                                                            };
                                                            append_bytes(a474, a475)
                                                        };
                                                        append_bytes(a476, a477)
                                                    };
                                                    append_bytes(a478, a479)
                                                };
                                                append_bytes(a480, a481)
                                            };
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
    let v2: u64 = {
        let (h502, _) = f64(Fn4::F65(v1.clone()), {
            let a503 = v1.clone();
            length_bytes(a503)
        }, 0u64)?;
        h502
    };
    let a504 = v0;
    let a505 = 0u64;
    if nat_eq(a504, a505) {
        let a506 = Str::lit("0");
        Ok(utf8_encode(a506))
    } else {
        Ok(f66(v1.clone(), v2, {
            let a508 = {
                let a507 = v1;
                length_bytes(a507)
            };
            let a509 = v2;
            nat_sub(a508, a509)
        }))
    }
}

pub fn f58(v0: u64, v1: (u64, Adt20)) -> R<Option<(u64, Adt20)>> {
    let a511 = {
        let (h510, _) = v1.clone();
        h510
    };
    let a512 = v0;
    if nat_lt(a511, a512) {
        Ok(Some(({
            let a514 = {
                let (h513, _) = v1;
                h513
            };
            let a515 = 1u64;
            nat_add(a514, a515)?
        }, f8({
            let (_, h516) = v1;
            h516
        }, Str::lit("    "))?)))
    } else {
        Ok(None::<(u64, Adt20)>)
    }
}

pub fn f59(v0: Fn3, p520: (Option<Bytes>, List<Bytes>), p521: List<Bytes>) -> (Option<Bytes>, List<Bytes>) {
    let m519 = (p520, p521);
    {
        let mut m519 = m519;
        loop {
            let step = {
                let (v1, v2) = m519;
                let m517 = v2;
                match m517.uncons() {
                    None => {
                        Err::<((Option<Bytes>, List<Bytes>), List<Bytes>), (Option<Bytes>, List<Bytes>)>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<((Option<Bytes>, List<Bytes>), List<Bytes>), (Option<Bytes>, List<Bytes>)>(({
                            let c518: Fn3 = v0.clone();
                            c518.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m519 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f60(v0: (Option<Bytes>, List<Bytes>), v1: Bytes) -> (Option<Bytes>, List<Bytes>) {
    f67(v0, v1)
}

pub fn f61(v0: Bytes, v1: (u64, Adt20)) -> R<Option<(u64, Adt20)>> {
    let a524 = {
        let (h522, _) = v1.clone();
        h522
    };
    let a525 = {
        let a523 = v0.clone();
        length_bytes(a523)
    };
    if nat_lt(a524, a525) {
        let v2: (u64, u64) = f68(v0.clone(), {
            let (h526, _) = v1.clone();
            h526
        })?;
        let v3: u64 = {
            let (h527, _) = v2;
            h527
        };
        Ok(Some(({
            let a530 = {
                let (h528, _) = v1.clone();
                h528
            };
            let a531 = {
                let (_, h529) = v2;
                h529
            };
            nat_add(a530, a531)?
        }, {
            let a532 = v3;
            let a533 = 34u64;
            if nat_eq(a532, a533) {
                f8({
                    let (_, h534) = v1;
                    h534
                }, Str::lit("\\\""))?
            } else {
                let a535 = v3;
                let a536 = 92u64;
                if nat_eq(a535, a536) {
                    f8({
                        let (_, h537) = v1;
                        h537
                    }, Str::lit("\\\\"))?
                } else {
                    let a538 = 32u64;
                    let a539 = v3;
                    if if nat_le(a538, a539) {
                        let a540 = v3;
                        let a541 = 126u64;
                        nat_le(a540, a541)
                    } else {
                        false
                    } {
                        f36({
                            let (_, h542) = v1;
                            h542
                        }, f66(v0, {
                            let (h543, _) = v1;
                            h543
                        }, 1u64))?
                    } else {
                        f8(f69(f8({
                            let (_, h544) = v1;
                            h544
                        }, Str::lit("\\u{"))?, v3)?, Str::lit("}"))?
                    }
                }
            }
        })))
    } else {
        Ok(None::<(u64, Adt20)>)
    }
}

pub fn f62(v0: Bytes, v1: (u64, Adt20)) -> R<Option<(u64, Adt20)>> {
    let m546 = f70(v0, {
        let (h545, _) = v1.clone();
        h545
    });
    match m546 {
        None => {
            Ok(None::<(u64, Adt20)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a548 = {
                    let (h547, _) = v1.clone();
                    h547
                };
                let a549 = 1u64;
                nat_add(a548, a549)?
            }, f71(f8({
                let a551 = {
                    let (h550, _) = v1;
                    h550
                };
                let a552 = 0u64;
                if nat_eq(a551, a552) {
                    let (_, h553) = v1;
                    h553
                } else {
                    f8({
                        let (_, h554) = v1;
                        h554
                    }, Str::lit(", "))?
                }
            }, Str::lit("0x"))?, v2)?)))
        }
    }
}

pub fn f63(v0: u64, v1: u64) -> Bytes {
    f66(Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39]), {
        let a558 = {
            let a555 = v0;
            let a556 = v1;
            let a557 = 0u64;
            nat_quot(a555, a556, a557)
        };
        let a559 = 10u64;
        let a560 = 0u64;
        nat_rem(a558, a559, a560)
    }, 1u64)
}

pub fn f64(v0: Fn4, p565: u64, p566: u64) -> R<(u64, bool)> {
    let m564 = (p565, p566);
    {
        let mut m564 = m564;
        loop {
            let step = {
                let (v1, v2) = m564;
                let m561 = v1;
                if m561 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m561 - 1;
                    let m563 = {
                        let c562: Fn4 = v0.clone();
                        c562.apply(v2)?
                    };
                    match m563 {
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
                Ok(next) => { m564 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f65(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f72(v0, v1)
}

pub fn f66(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m570 = {
        let a567 = v0;
        let a568 = v1;
        let a569 = v2;
        slice_bytes(a567, a568, a569)
    };
    match m570 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f67(v0: (Option<Bytes>, List<Bytes>), v1: Bytes) -> (Option<Bytes>, List<Bytes>) {
    let m572 = {
        let (h571, _) = v0;
        h571
    };
    match m572 {
        None => {
            (Some(v1), {
                let (_, h573) = v0;
                h573
            })
        }
        Some(v2) => {
            (None::<Bytes>, List::cons({
                let a574 = v2;
                let a575 = v1;
                append_bytes(a574, a575)
            }, {
                let (_, h576) = v0;
                h576
            }))
        }
    }
}

pub fn f68(v0: Bytes, v1: u64) -> R<(u64, u64)> {
    let m577 = f70(v0.clone(), v1);
    match m577 {
        None => {
            Ok((0u64, 1u64))
        }
        Some(v2) => {
            let v4: u64 = {
                let v3: u8 = v2;
                let a622 = {
                    let a598 = {
                        let a586 = {
                            let a580 = {
                                let a578 = v3;
                                let a579 = 128u8;
                                fixed_u8::bit_and(a578, a579)
                            };
                            let a581 = 128u8;
                            if equal(a580, a581) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a587 = {
                            let a584 = {
                                let a582 = v3;
                                let a583 = 64u8;
                                fixed_u8::bit_and(a582, a583)
                            };
                            let a585 = 64u8;
                            if equal(a584, a585) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a586, a587)?
                    };
                    let a599 = {
                        let a596 = {
                            let a590 = {
                                let a588 = v3;
                                let a589 = 32u8;
                                fixed_u8::bit_and(a588, a589)
                            };
                            let a591 = 32u8;
                            if equal(a590, a591) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a597 = {
                            let a594 = {
                                let a592 = v3;
                                let a593 = 16u8;
                                fixed_u8::bit_and(a592, a593)
                            };
                            let a595 = 16u8;
                            if equal(a594, a595) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a596, a597)?
                    };
                    nat_add(a598, a599)?
                };
                let a623 = {
                    let a620 = {
                        let a608 = {
                            let a602 = {
                                let a600 = v3;
                                let a601 = 8u8;
                                fixed_u8::bit_and(a600, a601)
                            };
                            let a603 = 8u8;
                            if equal(a602, a603) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a609 = {
                            let a606 = {
                                let a604 = v3;
                                let a605 = 4u8;
                                fixed_u8::bit_and(a604, a605)
                            };
                            let a607 = 4u8;
                            if equal(a606, a607) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a608, a609)?
                    };
                    let a621 = {
                        let a618 = {
                            let a612 = {
                                let a610 = v3;
                                let a611 = 2u8;
                                fixed_u8::bit_and(a610, a611)
                            };
                            let a613 = 2u8;
                            if equal(a612, a613) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a619 = {
                            let a616 = {
                                let a614 = v3;
                                let a615 = 1u8;
                                fixed_u8::bit_and(a614, a615)
                            };
                            let a617 = 1u8;
                            if equal(a616, a617) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a618, a619)?
                    };
                    nat_add(a620, a621)?
                };
                nat_add(a622, a623)?
            };
            let a624 = v4;
            let a625 = 128u64;
            if nat_lt(a624, a625) {
                Ok((v4, 1u64))
            } else {
                let a626 = v4;
                let a627 = 224u64;
                if nat_lt(a626, a627) {
                    Ok(({
                        let a635 = {
                            let a631 = {
                                let a628 = v4;
                                let a629 = 32u64;
                                let a630 = 0u64;
                                nat_rem(a628, a629, a630)
                            };
                            let a632 = 64u64;
                            nat_mul(a631, a632)?
                        };
                        let a636 = f73(v0, {
                            let a633 = v1;
                            let a634 = 1u64;
                            nat_add(a633, a634)?
                        })?;
                        nat_add(a635, a636)?
                    }, 2u64))
                } else {
                    let a637 = v4;
                    let a638 = 240u64;
                    if nat_lt(a637, a638) {
                        Ok(({
                            let a652 = {
                                let a642 = {
                                    let a639 = v4;
                                    let a640 = 16u64;
                                    let a641 = 0u64;
                                    nat_rem(a639, a640, a641)
                                };
                                let a643 = 4096u64;
                                nat_mul(a642, a643)?
                            };
                            let a653 = {
                                let a650 = {
                                    let a646 = f73(v0.clone(), {
                                        let a644 = v1;
                                        let a645 = 1u64;
                                        nat_add(a644, a645)?
                                    })?;
                                    let a647 = 64u64;
                                    nat_mul(a646, a647)?
                                };
                                let a651 = f73(v0, {
                                    let a648 = v1;
                                    let a649 = 2u64;
                                    nat_add(a648, a649)?
                                })?;
                                nat_add(a650, a651)?
                            };
                            nat_add(a652, a653)?
                        }, 3u64))
                    } else {
                        Ok(({
                            let a673 = {
                                let a657 = {
                                    let a654 = v4;
                                    let a655 = 8u64;
                                    let a656 = 0u64;
                                    nat_rem(a654, a655, a656)
                                };
                                let a658 = 262144u64;
                                nat_mul(a657, a658)?
                            };
                            let a674 = {
                                let a671 = {
                                    let a661 = f73(v0.clone(), {
                                        let a659 = v1;
                                        let a660 = 1u64;
                                        nat_add(a659, a660)?
                                    })?;
                                    let a662 = 4096u64;
                                    nat_mul(a661, a662)?
                                };
                                let a672 = {
                                    let a669 = {
                                        let a665 = f73(v0.clone(), {
                                            let a663 = v1;
                                            let a664 = 2u64;
                                            nat_add(a663, a664)?
                                        })?;
                                        let a666 = 64u64;
                                        nat_mul(a665, a666)?
                                    };
                                    let a670 = f73(v0, {
                                        let a667 = v1;
                                        let a668 = 3u64;
                                        nat_add(a667, a668)?
                                    })?;
                                    nat_add(a669, a670)?
                                };
                                nat_add(a671, a672)?
                            };
                            nat_add(a673, a674)?
                        }, 4u64))
                    }
                }
            }
        }
    }
}

pub fn f69(v0: Adt20, v1: u64) -> R<Adt20> {
    let v2: Bytes = {
        let a687 = f74(v1, 268435456u64);
        let a688 = {
            let a685 = f74(v1, 16777216u64);
            let a686 = {
                let a683 = f74(v1, 1048576u64);
                let a684 = {
                    let a681 = f74(v1, 65536u64);
                    let a682 = {
                        let a679 = f74(v1, 4096u64);
                        let a680 = {
                            let a677 = f74(v1, 256u64);
                            let a678 = {
                                let a675 = f74(v1, 16u64);
                                let a676 = f74(v1, 1u64);
                                append_bytes(a675, a676)
                            };
                            append_bytes(a677, a678)
                        };
                        append_bytes(a679, a680)
                    };
                    append_bytes(a681, a682)
                };
                append_bytes(a683, a684)
            };
            append_bytes(a685, a686)
        };
        append_bytes(a687, a688)
    };
    let v3: u64 = {
        let (h689, _) = f64(Fn4::F75(v2.clone()), 7u64, 0u64)?;
        h689
    };
    f36(v0, f66(v2, v3, {
        let a690 = 8u64;
        let a691 = v3;
        nat_sub(a690, a691)
    }))
}

pub fn f70(v0: Bytes, v1: u64) -> Option<u8> {
    let a692 = v0;
    let a693 = v1;
    index_bytes(a692, a693)
}

pub fn f71(v0: Adt20, v1: u8) -> R<Adt20> {
    let v3: u64 = {
        let v2: u8 = v1;
        let a738 = {
            let a714 = {
                let a702 = {
                    let a696 = {
                        let a694 = v2;
                        let a695 = 128u8;
                        fixed_u8::bit_and(a694, a695)
                    };
                    let a697 = 128u8;
                    if equal(a696, a697) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a703 = {
                    let a700 = {
                        let a698 = v2;
                        let a699 = 64u8;
                        fixed_u8::bit_and(a698, a699)
                    };
                    let a701 = 64u8;
                    if equal(a700, a701) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a702, a703)?
            };
            let a715 = {
                let a712 = {
                    let a706 = {
                        let a704 = v2;
                        let a705 = 32u8;
                        fixed_u8::bit_and(a704, a705)
                    };
                    let a707 = 32u8;
                    if equal(a706, a707) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a713 = {
                    let a710 = {
                        let a708 = v2;
                        let a709 = 16u8;
                        fixed_u8::bit_and(a708, a709)
                    };
                    let a711 = 16u8;
                    if equal(a710, a711) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a712, a713)?
            };
            nat_add(a714, a715)?
        };
        let a739 = {
            let a736 = {
                let a724 = {
                    let a718 = {
                        let a716 = v2;
                        let a717 = 8u8;
                        fixed_u8::bit_and(a716, a717)
                    };
                    let a719 = 8u8;
                    if equal(a718, a719) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a725 = {
                    let a722 = {
                        let a720 = v2;
                        let a721 = 4u8;
                        fixed_u8::bit_and(a720, a721)
                    };
                    let a723 = 4u8;
                    if equal(a722, a723) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a724, a725)?
            };
            let a737 = {
                let a734 = {
                    let a728 = {
                        let a726 = v2;
                        let a727 = 2u8;
                        fixed_u8::bit_and(a726, a727)
                    };
                    let a729 = 2u8;
                    if equal(a728, a729) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a735 = {
                    let a732 = {
                        let a730 = v2;
                        let a731 = 1u8;
                        fixed_u8::bit_and(a730, a731)
                    };
                    let a733 = 1u8;
                    if equal(a732, a733) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a734, a735)?
            };
            nat_add(a736, a737)?
        };
        nat_add(a738, a739)?
    };
    f36(v0, {
        let a740 = f74(v3, 16u64);
        let a741 = f74(v3, 1u64);
        append_bytes(a740, a741)
    })
}

pub fn f72(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m742 = f70(v0, v1);
    match m742 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            let a743 = v2;
            let a744 = 48u8;
            if equal(a743, a744) {
                Ok(Some({
                    let a745 = v1;
                    let a746 = 1u64;
                    nat_add(a745, a746)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f73(v0: Bytes, v1: u64) -> R<u64> {
    let m747 = f70(v0, v1);
    match m747 {
        None => {
            Ok(0u64)
        }
        Some(v2) => {
            let a794 = {
                let v3: u8 = v2;
                let a792 = {
                    let a768 = {
                        let a756 = {
                            let a750 = {
                                let a748 = v3;
                                let a749 = 128u8;
                                fixed_u8::bit_and(a748, a749)
                            };
                            let a751 = 128u8;
                            if equal(a750, a751) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a757 = {
                            let a754 = {
                                let a752 = v3;
                                let a753 = 64u8;
                                fixed_u8::bit_and(a752, a753)
                            };
                            let a755 = 64u8;
                            if equal(a754, a755) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a756, a757)?
                    };
                    let a769 = {
                        let a766 = {
                            let a760 = {
                                let a758 = v3;
                                let a759 = 32u8;
                                fixed_u8::bit_and(a758, a759)
                            };
                            let a761 = 32u8;
                            if equal(a760, a761) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a767 = {
                            let a764 = {
                                let a762 = v3;
                                let a763 = 16u8;
                                fixed_u8::bit_and(a762, a763)
                            };
                            let a765 = 16u8;
                            if equal(a764, a765) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a766, a767)?
                    };
                    nat_add(a768, a769)?
                };
                let a793 = {
                    let a790 = {
                        let a778 = {
                            let a772 = {
                                let a770 = v3;
                                let a771 = 8u8;
                                fixed_u8::bit_and(a770, a771)
                            };
                            let a773 = 8u8;
                            if equal(a772, a773) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a779 = {
                            let a776 = {
                                let a774 = v3;
                                let a775 = 4u8;
                                fixed_u8::bit_and(a774, a775)
                            };
                            let a777 = 4u8;
                            if equal(a776, a777) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a778, a779)?
                    };
                    let a791 = {
                        let a788 = {
                            let a782 = {
                                let a780 = v3;
                                let a781 = 2u8;
                                fixed_u8::bit_and(a780, a781)
                            };
                            let a783 = 2u8;
                            if equal(a782, a783) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a789 = {
                            let a786 = {
                                let a784 = v3;
                                let a785 = 1u8;
                                fixed_u8::bit_and(a784, a785)
                            };
                            let a787 = 1u8;
                            if equal(a786, a787) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a788, a789)?
                    };
                    nat_add(a790, a791)?
                };
                nat_add(a792, a793)?
            };
            let a795 = 64u64;
            let a796 = 0u64;
            Ok(nat_rem(a794, a795, a796))
        }
    }
}

pub fn f74(v0: u64, v1: u64) -> Bytes {
    f66(Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66]), {
        let a800 = {
            let a797 = v0;
            let a798 = v1;
            let a799 = 0u64;
            nat_quot(a797, a798, a799)
        };
        let a801 = 16u64;
        let a802 = 0u64;
        nat_rem(a800, a801, a802)
    }, 1u64)
}

pub fn f75(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f76(v0, v1)
}

pub fn f76(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m803 = f70(v0, v1);
    match m803 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            let a804 = v2;
            let a805 = 48u8;
            if equal(a804, a805) {
                Ok(Some({
                    let a806 = v1;
                    let a807 = 1u64;
                    nat_add(a806, a807)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn run(p0: u64, p1: Adt0) -> R<Result<Bytes, Bytes>> {
    f0(p0, p1)
}
