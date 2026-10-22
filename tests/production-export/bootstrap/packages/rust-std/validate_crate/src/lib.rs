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
    C0(bool, Adt1),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(Adt2, Adt8, Adt9),
    C1(u64, Adt2, Adt2, bool, Adt10, Adt9),
    C2(Adt12, Adt13, Adt2, Adt17, Adt9),
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
}

#[derive(Clone)]
pub enum Adt15 {
    C0,
    C1,
    C2,
    C3,
}

#[derive(Clone)]
pub enum Adt16 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt17 {
    C0(Adt18, Adt9),
    C1(Adt19, Adt12, Adt9),
    C2(Adt20, std::rc::Rc<Adt17>, Adt9),
    C3(Adt21, std::rc::Rc<Adt17>, bool, Adt9),
    C4(Adt12, std::rc::Rc<Adt17>, bool, Adt9),
    C5(Adt23, std::rc::Rc<Adt17>, Adt9),
    C6(std::rc::Rc<Adt17>, std::rc::Rc<Adt17>, Adt9),
    C7(std::rc::Rc<Adt17>, std::rc::Rc<Adt17>, std::rc::Rc<Adt17>, Adt9),
    C8(std::rc::Rc<Adt17>, std::rc::Rc<Adt17>, Adt9),
    C9(std::rc::Rc<Adt17>),
    C10(Adt24, Adt12, Adt9),
    C11(std::rc::Rc<Adt17>, std::rc::Rc<Adt17>),
    C12,
    C13(std::rc::Rc<Adt17>, std::rc::Rc<Adt17>),
    C14(Adt14, bool, Adt2, std::rc::Rc<Adt17>, Adt9, std::rc::Rc<Adt17>),
    C15(Adt14, std::rc::Rc<Adt17>, std::rc::Rc<Adt17>),
    C16(Adt12, std::rc::Rc<Adt17>),
}

#[derive(Clone)]
pub enum Adt18 {
    C0,
    C1,
    C2,
    C3,
    C4(Adt4),
    C5,
    C6,
    C7,
    C8,
}

#[derive(Clone)]
pub enum Adt19 {
    C0,
    C1,
    C2,
    C3,
    C4,
}

#[derive(Clone)]
pub enum Adt20 {
    C0,
    C1,
    C2,
    C3,
}

#[derive(Clone)]
pub enum Adt21 {
    C0(u64),
    C1(Adt22),
    C2(u64),
}

#[derive(Clone)]
pub enum Adt22 {
    C0(u64, Option<Adt4>),
}

#[derive(Clone)]
pub enum Adt23 {
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
pub enum Adt24 {
    C0,
    C1,
    C2,
    C3,
}

#[derive(Clone)]
pub enum Adt25 {
    C0(u64, Bytes, u64, bool, bool),
}

#[derive(Clone)]
pub enum Adt26 {
    C0(List<(u64, List<(u64, u64)>)>, List<(u64, List<(u64, u64)>)>, List<(u64, bool)>, List<(u64, bool)>),
}

#[derive(Clone)]
pub enum Adt27 {
    C0(bool, Adt26, bool, List<Adt25>),
}

#[derive(Clone)]
pub enum Adt28 {
    C0(List<Adt12>, List<Adt12>, List<Adt12>, List<(Adt12, List<u64>)>, bool),
}

#[derive(Clone)]
pub enum Adt29 {
    C0(Bytes, Bytes, Option<Adt4>, Option<Adt4>),
}

#[derive(Clone)]
pub enum Adt30 {
    C0(List<Adt29>, List<Bytes>, List<Adt25>),
}

#[derive(Clone)]
pub enum Fn0 {
    F35(u64),
}

impl Fn0 {
    pub fn apply(&self, p0: Option<List<(u64, u64)>>, p1: (u64, List<(u64, u64)>)) -> Option<List<(u64, u64)>> {
        match self {
            Fn0::F35(k0) => f35(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F37(u64),
}

impl Fn1 {
    pub fn apply(&self, p0: Option<bool>, p1: (u64, bool)) -> Option<bool> {
        match self {
            Fn1::F37(k0) => f37(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F39(Bytes),
    F182(Bytes, u64, Bytes, u64),
}

impl Fn2 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn2::F39(k0) => f39(k0.clone(), p0),
            Fn2::F182(k0, k1, k2, k3) => f182(k0.clone(), *k1, k2.clone(), *k3, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F55,
}

impl Fn3 {
    pub fn apply(&self, p0: bool, p1: Adt11) -> bool {
        match self {
            Fn3::F55 => f55(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F90(Adt12),
}

impl Fn4 {
    pub fn apply(&self, p0: bool, p1: Adt12) -> bool {
        match self {
            Fn4::F90(k0) => f90(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F111,
    F115(Adt12),
}

impl Fn5 {
    pub fn apply(&self, p0: List<Adt12>, p1: Adt12) -> List<Adt12> {
        match self {
            Fn5::F111 => f111(p0, p1),
            Fn5::F115(k0) => f115(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F113,
    F163(Adt12),
}

impl Fn6 {
    pub fn apply(&self, p0: List<(Adt12, List<u64>)>, p1: (Adt12, List<u64>)) -> List<(Adt12, List<u64>)> {
        match self {
            Fn6::F113 => f113(p0, p1),
            Fn6::F163(k0) => f163(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F117(List<Adt12>, List<Adt12>, Adt12),
}

impl Fn7 {
    pub fn apply(&self, p0: Option<Adt12>, p1: Adt12) -> Option<Adt12> {
        match self {
            Fn7::F117(k0, k1, k2) => f117(k0.clone(), k1.clone(), k2.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F119(List<(Adt12, List<u64>)>, List<Adt12>),
}

impl Fn8 {
    pub fn apply(&self, p0: Option<Adt12>, p1: (Adt12, List<u64>)) -> Option<Adt12> {
        match self {
            Fn8::F119(k0, k1) => f119(k0.clone(), k1.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F121(Adt12),
}

impl Fn9 {
    pub fn apply(&self, p0: Option<u64>, p1: (Adt12, u64)) -> Option<u64> {
        match self {
            Fn9::F121(k0) => f121(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F125(u64),
}

impl Fn10 {
    pub fn apply(&self, p0: Option<u64>, p1: (u64, u64)) -> Option<u64> {
        match self {
            Fn10::F125(k0) => f125(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F128(Adt12),
}

impl Fn11 {
    pub fn apply(&self, p0: Option<List<u64>>, p1: (Adt12, List<u64>)) -> Option<List<u64>> {
        match self {
            Fn11::F128(k0) => f128(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F132,
}

impl Fn12 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Adt29) -> Option<Bytes> {
        match self {
            Fn12::F132 => f132(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F152(u64),
}

impl Fn13 {
    pub fn apply(&self, p0: Option<Adt25>, p1: Adt25) -> Option<Adt25> {
        match self {
            Fn13::F152(k0) => f152(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F162(u64),
    F187(List<u64>),
}

impl Fn14 {
    pub fn apply(&self, p0: bool, p1: u64) -> bool {
        match self {
            Fn14::F162(k0) => f162(*k0, p0, p1),
            Fn14::F187(k0) => f187(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F176,
}

impl Fn15 {
    pub fn apply(&self, p0: List<u64>, p1: u64) -> List<u64> {
        match self {
            Fn15::F176 => f176(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F181(Adt29),
}

impl Fn16 {
    pub fn apply(&self, p0: bool, p1: Adt29) -> bool {
        match self {
            Fn16::F181(k0) => f181(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F191(Bytes),
}

impl Fn17 {
    pub fn apply(&self, p0: (bool, List<Bytes>), p1: Bytes) -> (bool, List<Bytes>) {
        match self {
            Fn17::F191(k0) => f191(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F195(Bytes),
}

impl Fn18 {
    pub fn apply(&self, p0: bool, p1: Bytes) -> bool {
        match self {
            Fn18::F195(k0) => f195(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F198,
}

impl Fn19 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn19::F198 => f198(p0, p1),
        }
    }
}

pub fn f0(v0: Adt0) -> R<Result<(), Bytes>> {
    let v1: List<Adt25> = List::cons(Adt25::C0(0u64, f1(Str::lit("nat_add")), 0u64, true, false), List::cons(Adt25::C0(1u64, f1(Str::lit("nat_sub")), 0u64, false, false), List::cons(Adt25::C0(2u64, f1(Str::lit("nat_mul")), 0u64, true, false), List::cons(Adt25::C0(3u64, f1(Str::lit("nat_quot")), 0u64, false, false), List::cons(Adt25::C0(4u64, f1(Str::lit("nat_rem")), 0u64, false, false), List::cons(Adt25::C0(5u64, f1(Str::lit("nat_eq")), 0u64, false, false), List::cons(Adt25::C0(6u64, f1(Str::lit("nat_le")), 0u64, false, false), List::cons(Adt25::C0(7u64, f1(Str::lit("nat_lt")), 0u64, false, false), List::cons(Adt25::C0(8u64, f1(Str::lit("nat_succ")), 0u64, true, false), List::cons(Adt25::C0(9u64, f1(Str::lit("int_add")), 0u64, true, false), List::<Adt25>::nil()))))))))));
    let v2: List<Adt25> = List::cons(Adt25::C0(10u64, f1(Str::lit("int_sub")), 0u64, true, false), List::cons(Adt25::C0(11u64, f1(Str::lit("int_mul")), 0u64, true, false), List::cons(Adt25::C0(12u64, f1(Str::lit("int_neg")), 0u64, true, false), List::cons(Adt25::C0(13u64, f1(Str::lit("int_quot")), 0u64, true, false), List::cons(Adt25::C0(14u64, f1(Str::lit("int_rem")), 0u64, false, false), List::cons(Adt25::C0(15u64, f1(Str::lit("bool_not")), 0u64, false, false), List::cons(Adt25::C0(16u64, f1(Str::lit("bool_and")), 0u64, false, false), List::cons(Adt25::C0(17u64, f1(Str::lit("bool_or")), 0u64, false, false), List::cons(Adt25::C0(18u64, f1(Str::lit("equal")), 0u64, false, false), List::cons(Adt25::C0(19u64, f1(Str::lit("compare")), 0u64, false, false), v1))))))))));
    let v3: List<Adt25> = List::cons(Adt25::C0(20u64, f1(Str::lit("checked_add")), 1u64, false, false), List::cons(Adt25::C0(21u64, f1(Str::lit("checked_sub")), 1u64, false, false), List::cons(Adt25::C0(22u64, f1(Str::lit("checked_mul")), 1u64, false, false), List::cons(Adt25::C0(23u64, f1(Str::lit("checked_quot")), 1u64, false, false), List::cons(Adt25::C0(24u64, f1(Str::lit("checked_neg_")), 2u64, false, false), List::cons(Adt25::C0(25u64, f1(Str::lit("bit_and")), 1u64, false, false), List::cons(Adt25::C0(26u64, f1(Str::lit("bit_or")), 1u64, false, false), List::cons(Adt25::C0(27u64, f1(Str::lit("bit_xor")), 1u64, false, false), List::cons(Adt25::C0(28u64, f1(Str::lit("bit_not")), 1u64, false, false), List::cons(Adt25::C0(29u64, f1(Str::lit("shift_left")), 1u64, false, false), v2))))))))));
    let v4: List<Adt25> = List::cons(Adt25::C0(30u64, f1(Str::lit("shift_right")), 1u64, false, false), List::cons(Adt25::C0(31u64, f1(Str::lit("convert")), 1u64, false, false), List::cons(Adt25::C0(32u64, f1(Str::lit("append_list")), 0u64, false, true), List::cons(Adt25::C0(33u64, f1(Str::lit("append_bytes")), 0u64, false, true), List::cons(Adt25::C0(34u64, f1(Str::lit("length_list")), 0u64, false, true), List::cons(Adt25::C0(35u64, f1(Str::lit("length_bytes")), 0u64, false, true), List::cons(Adt25::C0(36u64, f1(Str::lit("length_string")), 0u64, false, true), List::cons(Adt25::C0(37u64, f1(Str::lit("index_list")), 0u64, false, true), List::cons(Adt25::C0(38u64, f1(Str::lit("index_bytes")), 0u64, false, true), List::cons(Adt25::C0(39u64, f1(Str::lit("slice_list")), 0u64, false, true), v3))))))))));
    let v5: List<Adt25> = List::cons(Adt25::C0(40u64, f1(Str::lit("slice_bytes")), 0u64, false, true), List::cons(Adt25::C0(41u64, f1(Str::lit("utf8_encode")), 0u64, false, true), List::cons(Adt25::C0(42u64, f1(Str::lit("utf8_decode")), 0u64, false, true), List::cons(Adt25::C0(43u64, f1(Str::lit("compare_bytes")), 0u64, false, true), List::cons(Adt25::C0(44u64, f1(Str::lit("split_exact")), 0u64, false, true), List::cons(Adt25::C0(45u64, f1(Str::lit("join")), 0u64, false, true), List::cons(Adt25::C0(46u64, f1(Str::lit("format_i64")), 0u64, false, true), List::cons(Adt25::C0(47u64, f1(Str::lit("format_")), 2u64, false, true), List::cons(Adt25::C0(48u64, f1(Str::lit("parse_int")), 0u64, true, true), List::cons(Adt25::C0(49u64, f1(Str::lit("parse_")), 2u64, false, true), v4))))))))));
    let m2 = f2(Adt26::C0(List::<(u64, List<(u64, u64)>)>::nil(), List::<(u64, List<(u64, u64)>)>::nil(), List::<(u64, bool)>::nil(), List::<(u64, bool)>::nil()), match v0.clone() {
        Adt0::C0(_, h1) => {
            h1
        }
    })?;
    match m2 {
        Ok(v6) => {
            f3(match v0.clone() {
                Adt0::C0(h3, _) => {
                    h3
                }
            }, v6, v5, match v0 {
                Adt0::C0(_, h4) => {
                    h4
                }
            })
        }
        Err(v7) => {
            Ok(Err::<(), Bytes>(v7))
        }
    }
}

pub fn f1(v0: Str) -> Bytes {
    let a5 = v0;
    utf8_encode(a5)
}

pub fn f2(p38: Adt26, p39: Adt1) -> R<Result<Adt26, Bytes>> {
    let m37 = (p38, p39);
    {
        let mut m37 = m37;
        loop {
            let step = {
                let (v0, v1) = m37;
                let m6 = v1;
                match m6 {
                    Adt1::C0(_, _, _) => {
                        Err::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>(Ok(Ok::<Adt26, Bytes>(v0)))
                    }
                    Adt1::C1(_, _, _, _, _, _) => {
                        Err::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>(Ok(Ok::<Adt26, Bytes>(v0)))
                    }
                    Adt1::C2(_, _, _, _, _) => {
                        Err::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>(Ok(Ok::<Adt26, Bytes>(v0)))
                    }
                    Adt1::C3 => {
                        Err::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>(Ok(Ok::<Adt26, Bytes>(v0)))
                    }
                    Adt1::C4(r35, r36) => {
                        let v16 = (*r35).clone();
                        let v17 = (*r36).clone();
                        let m7 = v16;
                        match m7 {
                            Adt1::C0(v18, v19, _) => {
                                let m8 = v18.clone();
                                match m8 {
                                    Adt2::C0(_) => {
                                        Err::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>(Ok(Err::<Adt26, Bytes>(f4(f1(Str::lit("an enum named ")), f4(f5(v18)?, f1(Str::lit(" is not a declared type")))))))
                                    }
                                    Adt2::C1(_) => {
                                        Err::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>(Ok(Err::<Adt26, Bytes>(f4(f1(Str::lit("an enum named ")), f4(f5(v18)?, f1(Str::lit(" is not a declared type")))))))
                                    }
                                    Adt2::C2(_, _) => {
                                        Err::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>(Ok(Err::<Adt26, Bytes>(f4(f1(Str::lit("an enum named ")), f4(f5(v18)?, f1(Str::lit(" is not a declared type")))))))
                                    }
                                    Adt2::C3(_, _, _) => {
                                        Err::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>(Ok(Err::<Adt26, Bytes>(f4(f1(Str::lit("an enum named ")), f4(f5(v18)?, f1(Str::lit(" is not a declared type")))))))
                                    }
                                    Adt2::C4(v28, v29) => {
                                        let m9 = f6(v28);
                                        if m9 {
                                            let m11 = f7(match v0.clone() {
                                                Adt26::C0(_, h10, _, _) => {
                                                    h10
                                                }
                                            }, v29);
                                            if m11 {
                                                Err::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>(Ok(Err::<Adt26, Bytes>(f4(f1(Str::lit("hygiene: ")), f4(f5(v18)?, f1(Str::lit(" is declared twice")))))))
                                            } else {
                                                Ok::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>((Adt26::C0(match v0.clone() {
                                                    Adt26::C0(h12, _, _, _) => {
                                                        h12
                                                    }
                                                }, List::cons((v29, f8(List::<(u64, u64)>::nil(), v19)?), match v0.clone() {
                                                    Adt26::C0(_, h13, _, _) => {
                                                        h13
                                                    }
                                                }), match v0.clone() {
                                                    Adt26::C0(_, _, h14, _) => {
                                                        h14
                                                    }
                                                }, match v0 {
                                                    Adt26::C0(_, _, _, h15) => {
                                                        h15
                                                    }
                                                }), v17))
                                            }
                                        } else {
                                            let m17 = f7(match v0.clone() {
                                                Adt26::C0(h16, _, _, _) => {
                                                    h16
                                                }
                                            }, v29);
                                            if m17 {
                                                Err::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>(Ok(Err::<Adt26, Bytes>(f4(f1(Str::lit("hygiene: ")), f4(f5(v18)?, f1(Str::lit(" is declared twice")))))))
                                            } else {
                                                Ok::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>((Adt26::C0(List::cons((v29, f8(List::<(u64, u64)>::nil(), v19)?), match v0.clone() {
                                                    Adt26::C0(h18, _, _, _) => {
                                                        h18
                                                    }
                                                }), match v0.clone() {
                                                    Adt26::C0(_, h19, _, _) => {
                                                        h19
                                                    }
                                                }, match v0.clone() {
                                                    Adt26::C0(_, _, h20, _) => {
                                                        h20
                                                    }
                                                }, match v0 {
                                                    Adt26::C0(_, _, _, h21) => {
                                                        h21
                                                    }
                                                }), v17))
                                            }
                                        }
                                    }
                                    Adt2::C5 => {
                                        Err::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>(Ok(Err::<Adt26, Bytes>(f4(f1(Str::lit("an enum named ")), f4(f5(v18)?, f1(Str::lit(" is not a declared type")))))))
                                    }
                                    Adt2::C6(_, _) => {
                                        Err::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>(Ok(Err::<Adt26, Bytes>(f4(f1(Str::lit("an enum named ")), f4(f5(v18)?, f1(Str::lit(" is not a declared type")))))))
                                    }
                                }
                            }
                            Adt1::C1(v32, _, _, v35, _, _) => {
                                let m23 = f9(match v0.clone() {
                                    Adt26::C0(_, _, h22, _) => {
                                        h22
                                    }
                                }, v32);
                                if m23 {
                                    Err::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>(Ok(Err::<Adt26, Bytes>(f4(f1(Str::lit("hygiene: Fn")), f4(f10(v32)?, f1(Str::lit("::apply is declared twice")))))))
                                } else {
                                    Ok::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>((Adt26::C0(match v0.clone() {
                                        Adt26::C0(h24, _, _, _) => {
                                            h24
                                        }
                                    }, match v0.clone() {
                                        Adt26::C0(_, h25, _, _) => {
                                            h25
                                        }
                                    }, List::cons((v32, v35), match v0.clone() {
                                        Adt26::C0(_, _, h26, _) => {
                                            h26
                                        }
                                    }), match v0 {
                                        Adt26::C0(_, _, _, h27) => {
                                            h27
                                        }
                                    }), v17))
                                }
                            }
                            Adt1::C2(v38, _, v40, _, _) => {
                                let m28 = v38;
                                match m28 {
                                    Adt12::C0(_) => {
                                        Ok::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>((v0, v17))
                                    }
                                    Adt12::C1(_) => {
                                        Ok::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>((v0, v17))
                                    }
                                    Adt12::C2(_) => {
                                        Ok::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>((v0, v17))
                                    }
                                    Adt12::C3(_) => {
                                        Ok::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>((v0, v17))
                                    }
                                    Adt12::C4(_) => {
                                        Ok::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>((v0, v17))
                                    }
                                    Adt12::C5(_) => {
                                        Ok::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>((v0, v17))
                                    }
                                    Adt12::C6(_) => {
                                        Ok::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>((v0, v17))
                                    }
                                    Adt12::C7(_) => {
                                        Ok::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>((v0, v17))
                                    }
                                    Adt12::C8(v51) => {
                                        let m30 = f9(match v0.clone() {
                                            Adt26::C0(_, _, _, h29) => {
                                                h29
                                            }
                                        }, v51);
                                        if m30 {
                                            Err::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>(Ok(Err::<Adt26, Bytes>(f4(f1(Str::lit("hygiene: f")), f4(f10(v51)?, f1(Str::lit(" is declared twice")))))))
                                        } else {
                                            Ok::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>((Adt26::C0(match v0.clone() {
                                                Adt26::C0(h31, _, _, _) => {
                                                    h31
                                                }
                                            }, match v0.clone() {
                                                Adt26::C0(_, h32, _, _) => {
                                                    h32
                                                }
                                            }, match v0.clone() {
                                                Adt26::C0(_, _, h33, _) => {
                                                    h33
                                                }
                                            }, List::cons((v51, f11(v40)), match v0 {
                                                Adt26::C0(_, _, _, h34) => {
                                                    h34
                                                }
                                            })), v17))
                                        }
                                    }
                                    Adt12::C9(_) => {
                                        Ok::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>((v0, v17))
                                    }
                                }
                            }
                            Adt1::C3 => {
                                Ok::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>((v0, v17))
                            }
                            Adt1::C4(_, _) => {
                                Ok::<(Adt26, Adt1), R<Result<Adt26, Bytes>>>((v0, v17))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m37 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f3(v0: bool, v1: Adt26, v2: List<Adt25>, p45: Adt1) -> R<Result<(), Bytes>> {
    let m44 = p45;
    {
        let mut m44 = m44;
        loop {
            let step = {
                let v3 = m44;
                let m40 = v3;
                match m40 {
                    Adt1::C0(_, _, _) => {
                        Err::<Adt1, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt1::C1(_, _, _, _, _, _) => {
                        Err::<Adt1, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt1::C2(_, _, _, _, _) => {
                        Err::<Adt1, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt1::C3 => {
                        Err::<Adt1, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt1::C4(r42, r43) => {
                        let v18 = (*r42).clone();
                        let v19 = (*r43).clone();
                        let m41 = f12(v0, v1.clone(), v2.clone(), v18)?;
                        match m41 {
                            Ok(_) => {
                                Ok::<Adt1, R<Result<(), Bytes>>>(v19)
                            }
                            Err(v21) => {
                                Err::<Adt1, R<Result<(), Bytes>>>(Ok(Err::<(), Bytes>(v21)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m44 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f4(v0: Bytes, v1: Bytes) -> Bytes {
    let a46 = v0;
    let a47 = v1;
    append_bytes(a46, a47)
}

pub fn f5(v0: Adt2) -> R<Bytes> {
    let m48 = v0;
    match m48 {
        Adt2::C0(v1) => {
            Ok(f13(v1))
        }
        Adt2::C1(v2) => {
            Ok(f4(f1(Str::lit("Fixed(")), f4(f14(v2), f1(Str::lit(")")))))
        }
        Adt2::C2(v3, r49) => {
            let v4 = (*r49).clone();
            Ok(f4(f15(v3), f4(f1(Str::lit("(")), f4(f5(v4)?, f1(Str::lit(")"))))))
        }
        Adt2::C3(v5, r50, r51) => {
            let v6 = (*r50).clone();
            let v7 = (*r51).clone();
            Ok(f4(f16(v5), f4(f1(Str::lit("(")), f4(f5(v6)?, f4(f1(Str::lit(", ")), f4(f5(v7)?, f1(Str::lit(")"))))))))
        }
        Adt2::C4(v8, v9) => {
            Ok(f4(f17(v8), f4(f1(Str::lit("(")), f4(f10(v9)?, f1(Str::lit(")"))))))
        }
        Adt2::C5 => {
            Ok(f1(Str::lit("unreachable")))
        }
        Adt2::C6(_, _) => {
            Ok(f1(Str::lit("unreachable")))
        }
    }
}

pub fn f6(v0: Adt7) -> bool {
    let m52 = v0;
    match m52 {
        Adt7::C0 => {
            false
        }
        Adt7::C1 => {
            true
        }
    }
}

pub fn f7(v0: List<(u64, List<(u64, u64)>)>, v1: u64) -> bool {
    let m53 = f18(v0, v1);
    match m53 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f8(p57: List<(u64, u64)>, p58: Adt8) -> R<List<(u64, u64)>> {
    let m56 = (p57, p58);
    {
        let mut m56 = m56;
        loop {
            let step = {
                let (v0, v1) = m56;
                let m54 = v1;
                match m54 {
                    Adt8::C0 => {
                        Err::<(List<(u64, u64)>, Adt8), R<List<(u64, u64)>>>(Ok(v0))
                    }
                    Adt8::C1(v2, v3, r55) => {
                        let v4 = (*r55).clone();
                        Ok::<(List<(u64, u64)>, Adt8), R<List<(u64, u64)>>>((List::cons((v2, f19(v3)?), v0), v4))
                    }
                }
            };
            match step {
                Ok(next) => { m56 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f9(v0: List<(u64, bool)>, v1: u64) -> bool {
    let m59 = f20(v0, v1);
    match m59 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f10(v0: u64) -> R<Bytes> {
    let v1: Bytes = f1(Str::lit("0"));
    let v2: Bytes = f21({
        let a96 = f22(v0, 10000000000000000000u64);
        let a97 = {
            let a94 = f22(v0, 1000000000000000000u64);
            let a95 = {
                let a92 = f22(v0, 100000000000000000u64);
                let a93 = {
                    let a90 = f22(v0, 10000000000000000u64);
                    let a91 = {
                        let a88 = f22(v0, 1000000000000000u64);
                        let a89 = {
                            let a86 = f22(v0, 100000000000000u64);
                            let a87 = {
                                let a84 = f22(v0, 10000000000000u64);
                                let a85 = {
                                    let a82 = f22(v0, 1000000000000u64);
                                    let a83 = {
                                        let a80 = f22(v0, 100000000000u64);
                                        let a81 = {
                                            let a78 = f22(v0, 10000000000u64);
                                            let a79 = {
                                                let a76 = f22(v0, 1000000000u64);
                                                let a77 = {
                                                    let a74 = f22(v0, 100000000u64);
                                                    let a75 = {
                                                        let a72 = f22(v0, 10000000u64);
                                                        let a73 = {
                                                            let a70 = f22(v0, 1000000u64);
                                                            let a71 = {
                                                                let a68 = f22(v0, 100000u64);
                                                                let a69 = {
                                                                    let a66 = f22(v0, 10000u64);
                                                                    let a67 = {
                                                                        let a64 = f22(v0, 1000u64);
                                                                        let a65 = {
                                                                            let a62 = f22(v0, 100u64);
                                                                            let a63 = {
                                                                                let a60 = f22(v0, 10u64);
                                                                                let a61 = f22(v0, 1u64);
                                                                                append_bytes(a60, a61)
                                                                            };
                                                                            append_bytes(a62, a63)
                                                                        };
                                                                        append_bytes(a64, a65)
                                                                    };
                                                                    append_bytes(a66, a67)
                                                                };
                                                                append_bytes(a68, a69)
                                                            };
                                                            append_bytes(a70, a71)
                                                        };
                                                        append_bytes(a72, a73)
                                                    };
                                                    append_bytes(a74, a75)
                                                };
                                                append_bytes(a76, a77)
                                            };
                                            append_bytes(a78, a79)
                                        };
                                        append_bytes(a80, a81)
                                    };
                                    append_bytes(a82, a83)
                                };
                                append_bytes(a84, a85)
                            };
                            append_bytes(a86, a87)
                        };
                        append_bytes(a88, a89)
                    };
                    append_bytes(a90, a91)
                };
                append_bytes(a92, a93)
            };
            append_bytes(a94, a95)
        };
        append_bytes(a96, a97)
    })?;
    let a98 = v0;
    let a99 = 0u64;
    if nat_eq(a98, a99) {
        Ok(v1)
    } else {
        Ok(v2)
    }
}

pub fn f11(v0: Adt2) -> bool {
    let m100 = v0;
    match m100 {
        Adt2::C0(_) => {
            false
        }
        Adt2::C1(_) => {
            false
        }
        Adt2::C2(v3, _) => {
            f23(v3)
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

pub fn f12(v0: bool, v1: Adt26, v2: List<Adt25>, v3: Adt1) -> R<Result<(), Bytes>> {
    let m101 = v3;
    match m101 {
        Adt1::C0(_, v5, _) => {
            f24(Adt27::C0(v0, v1, false, v2), v5)
        }
        Adt1::C1(v7, v8, v9, v10, v11, _) => {
            let v13: Adt27 = Adt27::C0(v0, v1, false, v2);
            let m102 = f25(v13.clone(), v8)?;
            match m102 {
                Ok(_) => {
                    let m103 = f26(v13.clone(), v9)?;
                    match m103 {
                        Ok(_) => {
                            f27(v13, v7, v10, v11)
                        }
                        Err(v16) => {
                            Ok(Err::<(), Bytes>(v16))
                        }
                    }
                }
                Err(v17) => {
                    Ok(Err::<(), Bytes>(v17))
                }
            }
        }
        Adt1::C2(v18, v19, v20, v21, _) => {
            let v23: bool = f11(v20.clone());
            let v24: Adt27 = Adt27::C0(v0, v1.clone(), v23, v2);
            let m104 = f26(v24.clone(), v20)?;
            match m104 {
                Ok(_) => {
                    let m105 = f28(v24.clone(), Adt28::C0(List::<Adt12>::nil(), List::<Adt12>::nil(), List::<Adt12>::nil(), List::<(Adt12, List<u64>)>::nil(), false), v19.clone())?;
                    match m105 {
                        Ok(v26) => {
                            let m106 = f29(v26, v24, v21.clone(), true)?;
                            match m106 {
                                Ok(v27) => {
                                    let m109 = if match v27 {
                                        Adt28::C0(_, _, _, _, h107) => {
                                            h107
                                        }
                                    } {
                                        let a108 = v23;
                                        bool_not(a108)
                                    } else {
                                        false
                                    };
                                    if m109 {
                                        Ok(Err::<(), Bytes>(f4(f1(Str::lit("arithmetic: ")), f4(f30(f31(v18)?), f1(Str::lit(" propagates a failure its result type does not carry"))))))
                                    } else {
                                        let m110 = f32(f33(List::<(Adt12, u64)>::nil(), v19), v1, v21, v23)?;
                                        match m110 {
                                            Ok(_) => {
                                                Ok(Ok::<(), Bytes>(()))
                                            }
                                            Err(v29) => {
                                                Ok(Err::<(), Bytes>(v29))
                                            }
                                        }
                                    }
                                }
                                Err(v30) => {
                                    Ok(Err::<(), Bytes>(v30))
                                }
                            }
                        }
                        Err(v31) => {
                            Ok(Err::<(), Bytes>(v31))
                        }
                    }
                }
                Err(v32) => {
                    Ok(Err::<(), Bytes>(v32))
                }
            }
        }
        Adt1::C3 => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt1::C4(_, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
    }
}

pub fn f13(v0: Adt3) -> Bytes {
    let m111 = v0;
    match m111 {
        Adt3::C0 => {
            f1(Str::lit("Unit"))
        }
        Adt3::C1 => {
            f1(Str::lit("Bool"))
        }
        Adt3::C2 => {
            f1(Str::lit("Nat"))
        }
        Adt3::C3 => {
            f1(Str::lit("Int"))
        }
        Adt3::C4 => {
            f1(Str::lit("Ordering"))
        }
        Adt3::C5 => {
            f1(Str::lit("Str"))
        }
        Adt3::C6 => {
            f1(Str::lit("Bytes"))
        }
        Adt3::C7 => {
            f1(Str::lit("Overflow"))
        }
        Adt3::C8 => {
            f1(Str::lit("I128"))
        }
    }
}

pub fn f14(v0: Adt4) -> Bytes {
    let m112 = v0;
    match m112 {
        Adt4::C0 => {
            f1(Str::lit("U8"))
        }
        Adt4::C1 => {
            f1(Str::lit("U16"))
        }
        Adt4::C2 => {
            f1(Str::lit("U32"))
        }
        Adt4::C3 => {
            f1(Str::lit("U64"))
        }
        Adt4::C4 => {
            f1(Str::lit("I8"))
        }
        Adt4::C5 => {
            f1(Str::lit("I16"))
        }
        Adt4::C6 => {
            f1(Str::lit("I32"))
        }
        Adt4::C7 => {
            f1(Str::lit("I64"))
        }
    }
}

pub fn f15(v0: Adt5) -> Bytes {
    let m113 = v0;
    match m113 {
        Adt5::C0 => {
            f1(Str::lit("Option"))
        }
        Adt5::C1 => {
            f1(Str::lit("List"))
        }
        Adt5::C2 => {
            f1(Str::lit("Rc"))
        }
        Adt5::C3 => {
            f1(Str::lit("Fallible"))
        }
        Adt5::C4 => {
            f1(Str::lit("Ref"))
        }
    }
}

pub fn f16(v0: Adt6) -> Bytes {
    let m114 = v0;
    match m114 {
        Adt6::C0 => {
            f1(Str::lit("Result"))
        }
        Adt6::C1 => {
            f1(Str::lit("Pair"))
        }
    }
}

pub fn f17(v0: Adt7) -> Bytes {
    let m115 = v0;
    match m115 {
        Adt7::C0 => {
            f1(Str::lit("Adt"))
        }
        Adt7::C1 => {
            f1(Str::lit("Fn"))
        }
    }
}

pub fn f18(v0: List<(u64, List<(u64, u64)>)>, v1: u64) -> Option<List<(u64, u64)>> {
    f34(Fn0::F35(v1), None::<List<(u64, u64)>>, v0)
}

pub fn f19(v0: Adt2) -> R<u64> {
    let m116 = v0;
    match m116 {
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
        Adt2::C6(_, r119) => {
            let v11 = (*r119).clone();
            let v12: u64 = f19(v11)?;
            let a117 = 1u64;
            let a118 = v12;
            nat_add(a117, a118)
        }
    }
}

pub fn f20(v0: List<(u64, bool)>, v1: u64) -> Option<bool> {
    f36(Fn1::F37(v1), None::<bool>, v0)
}

pub fn f21(v0: Bytes) -> R<Bytes> {
    let v1: u64 = {
        let (h120, _) = f38(Fn2::F39(v0.clone()), {
            let a121 = v0.clone();
            length_bytes(a121)
        }, 0u64)?;
        h120
    };
    Ok(f40(v0.clone(), v1, {
        let a123 = {
            let a122 = v0;
            length_bytes(a122)
        };
        let a124 = v1;
        nat_sub(a123, a124)
    }))
}

pub fn f22(v0: u64, v1: u64) -> Bytes {
    f40(f1(Str::lit("0123456789")), {
        let a128 = {
            let a125 = v0;
            let a126 = v1;
            let a127 = 0u64;
            nat_quot(a125, a126, a127)
        };
        let a129 = 10u64;
        let a130 = 0u64;
        nat_rem(a128, a129, a130)
    }, 1u64)
}

pub fn f23(v0: Adt5) -> bool {
    let m131 = v0;
    match m131 {
        Adt5::C0 => {
            false
        }
        Adt5::C1 => {
            false
        }
        Adt5::C2 => {
            false
        }
        Adt5::C3 => {
            true
        }
        Adt5::C4 => {
            false
        }
    }
}

pub fn f24(v0: Adt27, p136: Adt8) -> R<Result<(), Bytes>> {
    let m135 = p136;
    {
        let mut m135 = m135;
        loop {
            let step = {
                let v1 = m135;
                let m132 = v1;
                match m132 {
                    Adt8::C0 => {
                        Err::<Adt8, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt8::C1(_, v3, r134) => {
                        let v4 = (*r134).clone();
                        let m133 = f25(v0.clone(), v3)?;
                        match m133 {
                            Ok(_) => {
                                Ok::<Adt8, R<Result<(), Bytes>>>(v4)
                            }
                            Err(v6) => {
                                Err::<Adt8, R<Result<(), Bytes>>>(Ok(Err::<(), Bytes>(v6)))
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

pub fn f25(v0: Adt27, p142: Adt2) -> R<Result<(), Bytes>> {
    let m141 = p142;
    {
        let mut m141 = m141;
        loop {
            let step = {
                let v1 = m141;
                let m137 = v1;
                match m137 {
                    Adt2::C0(_) => {
                        Err::<Adt2, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt2::C1(_) => {
                        Err::<Adt2, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt2::C2(_, _) => {
                        Err::<Adt2, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt2::C3(_, _, _) => {
                        Err::<Adt2, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt2::C4(_, _) => {
                        Err::<Adt2, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt2::C5 => {
                        Err::<Adt2, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt2::C6(r139, r140) => {
                        let v11 = (*r139).clone();
                        let v12 = (*r140).clone();
                        let m138 = f26(v0.clone(), v11)?;
                        match m138 {
                            Ok(_) => {
                                Ok::<Adt2, R<Result<(), Bytes>>>(v12)
                            }
                            Err(v14) => {
                                Err::<Adt2, R<Result<(), Bytes>>>(Ok(Err::<(), Bytes>(v14)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m141 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f26(v0: Adt27, v1: Adt2) -> R<Result<(), Bytes>> {
    let v2: bool = f41(v1.clone());
    let v3: bool = if match v0.clone() {
        Adt27::C0(h143, _, _, _) => {
            h143
        }
    } {
        v2
    } else {
        false
    };
    let m144 = v3;
    if m144 {
        Ok(Err::<(), Bytes>(f4(f1(Str::lit("hidden allocation: rust-core renders the heap type ")), f4(f5(v1)?, f1(Str::lit(", which rust-core does not provide"))))))
    } else {
        f42(v0, v1)
    }
}

pub fn f27(v0: Adt27, v1: u64, v2: bool, p156: Adt10) -> R<Result<(), Bytes>> {
    let m155 = p156;
    {
        let mut m155 = m155;
        loop {
            let step = {
                let v3 = m155;
                let m145 = v3;
                match m145 {
                    Adt10::C0 => {
                        Err::<Adt10, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt10::C1(v4, v5, v6, r154) => {
                        let v7 = (*r154).clone();
                        let m149 = f20({
                            let m148 = match v0.clone() {
                                Adt27::C0(_, h147, _, _) => {
                                    h147
                                }
                            };
                            match m148 {
                                Adt26::C0(_, _, _, h146) => {
                                    h146
                                }
                            }
                        }, v4);
                        match m149 {
                            None => {
                                Err::<Adt10, R<Result<(), Bytes>>>(Ok(Err::<(), Bytes>(f4(f1(Str::lit("Fn")), f4(f10(v1)?, f4(f1(Str::lit(" dispatches to undeclared f")), f10(v4)?))))))
                            }
                            Some(v8) => {
                                let v9: bool = if v8 {
                                    !v6
                                } else {
                                    v6
                                };
                                let v10: bool = if v6 {
                                    !v2
                                } else {
                                    false
                                };
                                let m150 = v9;
                                if m150 {
                                    Err::<Adt10, R<Result<(), Bytes>>>(Ok(Err::<(), Bytes>(f4(f1(Str::lit("arithmetic: Fn")), f4(f10(v1)?, f4(f1(Str::lit("::apply misstates the failure of f")), f10(v4)?))))))
                                } else {
                                    let m151 = v10;
                                    if m151 {
                                        Err::<Adt10, R<Result<(), Bytes>>>(Ok(Err::<(), Bytes>(f4(f1(Str::lit("arithmetic: Fn")), f4(f10(v1)?, f4(f1(Str::lit("::apply misstates the failure of f")), f10(v4)?))))))
                                    } else {
                                        let m152 = f43(v5);
                                        if m152 {
                                            let m153 = f44(v0.clone(), f1(Str::lit("a boxed capture")));
                                            match m153 {
                                                Ok(_) => {
                                                    Ok::<Adt10, R<Result<(), Bytes>>>(v7)
                                                }
                                                Err(v12) => {
                                                    Err::<Adt10, R<Result<(), Bytes>>>(Ok(Err::<(), Bytes>(v12)))
                                                }
                                            }
                                        } else {
                                            Ok::<Adt10, R<Result<(), Bytes>>>(v7)
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m155 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f28(v0: Adt27, p163: Adt28, p164: Adt13) -> R<Result<Adt28, Bytes>> {
    let m162 = (p163, p164);
    {
        let mut m162 = m162;
        loop {
            let step = {
                let (v1, v2) = m162;
                let m157 = v2;
                match m157 {
                    Adt13::C0 => {
                        Err::<(Adt28, Adt13), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v1)))
                    }
                    Adt13::C1(v3, v4, r161) => {
                        let v5 = (*r161).clone();
                        let m158 = f26(v0.clone(), v4)?;
                        match m158 {
                            Ok(_) => {
                                let m160 = f45(v1, match v0.clone() {
                                    Adt27::C0(_, h159, _, _) => {
                                        h159
                                    }
                                }, v3)?;
                                match m160 {
                                    Ok(v7) => {
                                        Ok::<(Adt28, Adt13), R<Result<Adt28, Bytes>>>((v7, v5))
                                    }
                                    Err(v8) => {
                                        Err::<(Adt28, Adt13), R<Result<Adt28, Bytes>>>(Ok(Err::<Adt28, Bytes>(v8)))
                                    }
                                }
                            }
                            Err(v9) => {
                                Err::<(Adt28, Adt13), R<Result<Adt28, Bytes>>>(Ok(Err::<Adt28, Bytes>(v9)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m162 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f29(v0: Adt28, v1: Adt27, v2: Adt17, v3: bool) -> R<Result<Adt28, Bytes>> {
    let m165 = v2;
    match m165 {
        Adt17::C0(_, _) => {
            Ok(Ok::<Adt28, Bytes>(v0))
        }
        Adt17::C1(_, _, _) => {
            Ok(Ok::<Adt28, Bytes>(v0))
        }
        Adt17::C2(_, _, _) => {
            Ok(Ok::<Adt28, Bytes>(v0))
        }
        Adt17::C3(_, _, _, _) => {
            Ok(Ok::<Adt28, Bytes>(v0))
        }
        Adt17::C4(_, _, _, _) => {
            Ok(Ok::<Adt28, Bytes>(v0))
        }
        Adt17::C5(_, _, _) => {
            Ok(Ok::<Adt28, Bytes>(v0))
        }
        Adt17::C6(_, _, _) => {
            Ok(Ok::<Adt28, Bytes>(v0))
        }
        Adt17::C7(_, _, _, _) => {
            Ok(Ok::<Adt28, Bytes>(v0))
        }
        Adt17::C8(_, _, _) => {
            Ok(Ok::<Adt28, Bytes>(v0))
        }
        Adt17::C9(_) => {
            Ok(Ok::<Adt28, Bytes>(v0))
        }
        Adt17::C10(_, _, _) => {
            Ok(Ok::<Adt28, Bytes>(v0))
        }
        Adt17::C11(r173, r174) => {
            let v37 = (*r173).clone();
            let v38 = (*r174).clone();
            let v39: List<Adt12> = match v0.clone() {
                Adt28::C0(_, h166, _, _, _) => {
                    h166
                }
            };
            let m167 = f46(v0, v1.clone(), v37)?;
            match m167 {
                Ok(v40) => {
                    let m168 = f47(v40, v1, v38, v3)?;
                    match m168 {
                        Ok(v41) => {
                            Ok(Ok::<Adt28, Bytes>(Adt28::C0(match v41.clone() {
                                Adt28::C0(h169, _, _, _, _) => {
                                    h169
                                }
                            }, v39, match v41.clone() {
                                Adt28::C0(_, _, h170, _, _) => {
                                    h170
                                }
                            }, match v41.clone() {
                                Adt28::C0(_, _, _, h171, _) => {
                                    h171
                                }
                            }, match v41 {
                                Adt28::C0(_, _, _, _, h172) => {
                                    h172
                                }
                            })))
                        }
                        Err(v42) => {
                            Ok(Err::<Adt28, Bytes>(v42))
                        }
                    }
                }
                Err(v43) => {
                    Ok(Err::<Adt28, Bytes>(v43))
                }
            }
        }
        Adt17::C12 => {
            Ok(Ok::<Adt28, Bytes>(v0))
        }
        Adt17::C13(_, _) => {
            Ok(Ok::<Adt28, Bytes>(v0))
        }
        Adt17::C14(_, _, _, _, _, _) => {
            Ok(Ok::<Adt28, Bytes>(v0))
        }
        Adt17::C15(_, _, _) => {
            Ok(Ok::<Adt28, Bytes>(v0))
        }
        Adt17::C16(_, _) => {
            Ok(Ok::<Adt28, Bytes>(v0))
        }
    }
}

pub fn f30(v0: Bytes) -> Bytes {
    f4(f1(Str::lit("`")), f4(v0, f1(Str::lit("`"))))
}

pub fn f31(v0: Adt12) -> R<Bytes> {
    let m175 = v0;
    match m175 {
        Adt12::C0(v1) => {
            Ok(f4(f1(Str::lit("v")), f10(v1)?))
        }
        Adt12::C1(v2) => {
            Ok(f4(f1(Str::lit("m")), f10(v2)?))
        }
        Adt12::C2(v3) => {
            Ok(f4(f1(Str::lit("a")), f10(v3)?))
        }
        Adt12::C3(v4) => {
            Ok(f4(f1(Str::lit("c")), f10(v4)?))
        }
        Adt12::C4(v5) => {
            Ok(f4(f1(Str::lit("r")), f10(v5)?))
        }
        Adt12::C5(v6) => {
            Ok(f4(f1(Str::lit("h")), f10(v6)?))
        }
        Adt12::C6(v7) => {
            Ok(f4(f1(Str::lit("k")), f10(v7)?))
        }
        Adt12::C7(v8) => {
            Ok(f4(f1(Str::lit("p")), f10(v8)?))
        }
        Adt12::C8(v9) => {
            Ok(f4(f1(Str::lit("f")), f10(v9)?))
        }
        Adt12::C9(v10) => {
            Ok(v10)
        }
    }
}

pub fn f32(v0: List<(Adt12, u64)>, v1: Adt26, v2: Adt17, v3: bool) -> R<Result<List<(Adt12, u64)>, Bytes>> {
    let m176 = v2;
    match m176 {
        Adt17::C0(_, _) => {
            Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0))
        }
        Adt17::C1(_, _, _) => {
            Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0))
        }
        Adt17::C2(_, _, _) => {
            Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0))
        }
        Adt17::C3(_, _, _, _) => {
            Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0))
        }
        Adt17::C4(_, _, _, _) => {
            Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0))
        }
        Adt17::C5(_, _, _) => {
            Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0))
        }
        Adt17::C6(_, _, _) => {
            Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0))
        }
        Adt17::C7(_, _, _, _) => {
            Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0))
        }
        Adt17::C8(_, _, _) => {
            Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0))
        }
        Adt17::C9(_) => {
            Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0))
        }
        Adt17::C10(_, _, _) => {
            Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0))
        }
        Adt17::C11(r178, r179) => {
            let v37 = (*r178).clone();
            let v38 = (*r179).clone();
            let m177 = f48(v0, v1.clone(), v37)?;
            match m177 {
                Ok(v39) => {
                    f49(v39, v1, v38, v3)
                }
                Err(v40) => {
                    Ok(Err::<List<(Adt12, u64)>, Bytes>(v40))
                }
            }
        }
        Adt17::C12 => {
            Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0))
        }
        Adt17::C13(_, _) => {
            Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0))
        }
        Adt17::C14(_, _, _, _, _, _) => {
            Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0))
        }
        Adt17::C15(_, _, _) => {
            Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0))
        }
        Adt17::C16(_, _) => {
            Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0))
        }
    }
}

pub fn f33(p183: List<(Adt12, u64)>, p184: Adt13) -> List<(Adt12, u64)> {
    let m182 = (p183, p184);
    {
        let mut m182 = m182;
        loop {
            let step = {
                let (v0, v1) = m182;
                let m180 = v1;
                match m180 {
                    Adt13::C0 => {
                        Err::<(List<(Adt12, u64)>, Adt13), List<(Adt12, u64)>>(v0)
                    }
                    Adt13::C1(v2, v3, r181) => {
                        let v4 = (*r181).clone();
                        Ok::<(List<(Adt12, u64)>, Adt13), List<(Adt12, u64)>>((f50(v0, v2, v3), v4))
                    }
                }
            };
            match step {
                Ok(next) => { m182 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f34(v0: Fn0, p188: Option<List<(u64, u64)>>, p189: List<(u64, List<(u64, u64)>)>) -> Option<List<(u64, u64)>> {
    let m187 = (p188, p189);
    {
        let mut m187 = m187;
        loop {
            let step = {
                let (v1, v2) = m187;
                let m185 = v2;
                match m185.uncons() {
                    None => {
                        Err::<(Option<List<(u64, u64)>>, List<(u64, List<(u64, u64)>)>), Option<List<(u64, u64)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<List<(u64, u64)>>, List<(u64, List<(u64, u64)>)>), Option<List<(u64, u64)>>>(({
                            let c186: Fn0 = v0.clone();
                            c186.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m187 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f35(v0: u64, v1: Option<List<(u64, u64)>>, v2: (u64, List<(u64, u64)>)) -> Option<List<(u64, u64)>> {
    let m190 = v1;
    match m190 {
        None => {
            let a192 = {
                let (h191, _) = v2;
                h191
            };
            let a193 = v0;
            if nat_eq(a192, a193) {
                Some({
                    let (_, h194) = v2;
                    h194
                })
            } else {
                None::<List<(u64, u64)>>
            }
        }
        Some(v3) => {
            Some(v3)
        }
    }
}

pub fn f36(v0: Fn1, p198: Option<bool>, p199: List<(u64, bool)>) -> Option<bool> {
    let m197 = (p198, p199);
    {
        let mut m197 = m197;
        loop {
            let step = {
                let (v1, v2) = m197;
                let m195 = v2;
                match m195.uncons() {
                    None => {
                        Err::<(Option<bool>, List<(u64, bool)>), Option<bool>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<bool>, List<(u64, bool)>), Option<bool>>(({
                            let c196: Fn1 = v0.clone();
                            c196.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m197 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f37(v0: u64, v1: Option<bool>, v2: (u64, bool)) -> Option<bool> {
    let m200 = v1;
    match m200 {
        None => {
            let a202 = {
                let (h201, _) = v2;
                h201
            };
            let a203 = v0;
            if nat_eq(a202, a203) {
                Some({
                    let (_, h204) = v2;
                    h204
                })
            } else {
                None::<bool>
            }
        }
        Some(v3) => {
            Some(v3)
        }
    }
}

pub fn f38(v0: Fn2, p209: u64, p210: u64) -> R<(u64, bool)> {
    let m208 = (p209, p210);
    {
        let mut m208 = m208;
        loop {
            let step = {
                let (v1, v2) = m208;
                let m205 = v1;
                if m205 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m205 - 1;
                    let m207 = {
                        let c206: Fn2 = v0.clone();
                        c206.apply(v2)?
                    };
                    match m207 {
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
                Ok(next) => { m208 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f39(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f51(v0, v1)
}

pub fn f40(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m214 = {
        let a211 = v0;
        let a212 = v1;
        let a213 = v2;
        slice_bytes(a211, a212, a213)
    };
    match m214 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f41(v0: Adt2) -> bool {
    let m215 = v0;
    match m215 {
        Adt2::C0(v1) => {
            f52(v1)
        }
        Adt2::C1(_) => {
            false
        }
        Adt2::C2(v3, r217) => {
            let v4 = (*r217).clone();
            let m216 = f53(v3);
            if m216 {
                true
            } else {
                f41(v4)
            }
        }
        Adt2::C3(_, r218, r219) => {
            let v6 = (*r218).clone();
            let v7 = (*r219).clone();
            let v8: bool = f41(v6);
            let v9: bool = f41(v7);
            if v8 {
                true
            } else {
                v9
            }
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

pub fn f42(v0: Adt27, v1: Adt2) -> R<Result<(), Bytes>> {
    let m220 = v1;
    match m220 {
        Adt2::C0(_) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt2::C1(_) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt2::C2(_, r231) => {
            let v5 = (*r231).clone();
            f42(v0, v5)
        }
        Adt2::C3(_, r232, r233) => {
            let v7 = (*r232).clone();
            let v8 = (*r233).clone();
            let m221 = f42(v0.clone(), v7)?;
            match m221 {
                Ok(_) => {
                    f42(v0, v8)
                }
                Err(v10) => {
                    Ok(Err::<(), Bytes>(v10))
                }
            }
        }
        Adt2::C4(v11, v12) => {
            let m222 = f6(v11);
            if m222 {
                let m226 = f7({
                    let m225 = match v0 {
                        Adt27::C0(_, h224, _, _) => {
                            h224
                        }
                    };
                    match m225 {
                        Adt26::C0(_, h223, _, _) => {
                            h223
                        }
                    }
                }, v12);
                if m226 {
                    Ok(Ok::<(), Bytes>(()))
                } else {
                    Ok(Err::<(), Bytes>(f4(f1(Str::lit("type Fn")), f4(f10(v12)?, f1(Str::lit(" is not declared"))))))
                }
            } else {
                let m230 = f7({
                    let m229 = match v0 {
                        Adt27::C0(_, h228, _, _) => {
                            h228
                        }
                    };
                    match m229 {
                        Adt26::C0(h227, _, _, _) => {
                            h227
                        }
                    }
                }, v12);
                if m230 {
                    Ok(Ok::<(), Bytes>(()))
                } else {
                    Ok(Err::<(), Bytes>(f4(f1(Str::lit("type Adt")), f4(f10(v12)?, f1(Str::lit(" is not declared"))))))
                }
            }
        }
        Adt2::C5 => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt2::C6(_, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
    }
}

pub fn f43(v0: List<Adt11>) -> bool {
    f54(Fn3::F55, false, v0)
}

pub fn f44(v0: Adt27, v1: Bytes) -> Result<(), Bytes> {
    let m235 = match v0 {
        Adt27::C0(h234, _, _, _) => {
            h234
        }
    };
    if m235 {
        Err::<(), Bytes>(f4(f1(Str::lit("hidden allocation: rust-core renders ")), f4(v1, f1(Str::lit(", which requires heap allocation")))))
    } else {
        Ok::<(), Bytes>(())
    }
}

pub fn f45(v0: Adt28, v1: Adt26, v2: Adt14) -> R<Result<Adt28, Bytes>> {
    let m236 = v2;
    match m236 {
        Adt14::C0(_) => {
            Ok(Ok::<Adt28, Bytes>(v0))
        }
        Adt14::C1(v4) => {
            let m238 = f56(match v0.clone() {
                Adt28::C0(h237, _, _, _, _) => {
                    h237
                }
            }, v4.clone());
            if m238 {
                Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("hygiene: ")), f4(f30(f31(v4)?), f1(Str::lit(" is bound twice in one function"))))))
            } else {
                Ok(Ok::<Adt28, Bytes>(Adt28::C0(List::cons(v4.clone(), match v0.clone() {
                    Adt28::C0(h239, _, _, _, _) => {
                        h239
                    }
                }), List::cons(v4, match v0.clone() {
                    Adt28::C0(_, h240, _, _, _) => {
                        h240
                    }
                }), match v0.clone() {
                    Adt28::C0(_, _, h241, _, _) => {
                        h241
                    }
                }, match v0.clone() {
                    Adt28::C0(_, _, _, h242, _) => {
                        h242
                    }
                }, match v0 {
                    Adt28::C0(_, _, _, _, h243) => {
                        h243
                    }
                })))
            }
        }
        Adt14::C2(r249) => {
            let v5 = (*r249).clone();
            f45(v0, v1, v5)
        }
        Adt14::C3(_, r250) => {
            let v7 = (*r250).clone();
            f45(v0, v1, v7)
        }
        Adt14::C4(v8, v9, r251) => {
            let v10 = (*r251).clone();
            let m244 = f57(v1.clone(), v8, v9);
            match m244 {
                None => {
                    Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("pattern Adt")), f4(f10(v8)?, f4(f1(Str::lit("::C")), f4(f10(v9)?, f1(Str::lit(" is not declared"))))))))
                }
                Some(v11) => {
                    let m247 = {
                        let a245 = v11;
                        let a246 = f58(v10.clone())?;
                        nat_eq(a245, a246)
                    };
                    if m247 {
                        f45(v0, v1, v10)
                    } else {
                        Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("pattern Adt")), f4(f10(v8)?, f4(f1(Str::lit("::C")), f4(f10(v9)?, f4(f1(Str::lit(" binds ")), f4(f10(f58(v10)?)?, f4(f1(Str::lit(" fields of ")), f10(v11)?)))))))))
                    }
                }
            }
        }
        Adt14::C5 => {
            Ok(Ok::<Adt28, Bytes>(v0))
        }
        Adt14::C6(r252, r253) => {
            let v12 = (*r252).clone();
            let v13 = (*r253).clone();
            let m248 = f45(v0, v1.clone(), v12)?;
            match m248 {
                Ok(v14) => {
                    f45(v14, v1, v13)
                }
                Err(v15) => {
                    Ok(Err::<Adt28, Bytes>(v15))
                }
            }
        }
    }
}

pub fn f46(p266: Adt28, v1: Adt27, p267: Adt17) -> R<Result<Adt28, Bytes>> {
    let m265 = (p266, p267);
    {
        let mut m265 = m265;
        loop {
            let step = {
                let (v0, v2) = m265;
                let m254 = v2;
                match m254 {
                    Adt17::C0(_, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C1(_, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C2(_, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C3(_, _, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C4(_, _, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C5(_, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C6(_, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C7(_, _, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C8(_, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C9(_) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C10(_, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C11(_, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C12 => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C13(_, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C14(v40, v41, v42, r263, _, r264) => {
                        let v43 = (*r263).clone();
                        let v45 = (*r264).clone();
                        let m256 = {
                            let m255 = v41;
                            if m255 {
                                f26(v1.clone(), v42)?
                            } else {
                                Ok::<(), Bytes>(())
                            }
                        };
                        match m256 {
                            Ok(_) => {
                                let m260 = {
                                    let m257 = f59(v40.clone(), v43.clone());
                                    match m257 {
                                        None => {
                                            f47(v0, v1.clone(), v43, false)?
                                        }
                                        Some(v47) => {
                                            let m258 = v40.clone();
                                            match m258 {
                                                Adt14::C0(_) => {
                                                    f47(v0, v1.clone(), v43, false)?
                                                }
                                                Adt14::C1(_) => {
                                                    f47(v0, v1.clone(), v43, false)?
                                                }
                                                Adt14::C2(r259) => {
                                                    let v50 = (*r259).clone();
                                                    f60(v0, v47, v50)?
                                                }
                                                Adt14::C3(_, _) => {
                                                    f47(v0, v1.clone(), v43, false)?
                                                }
                                                Adt14::C4(_, _, _) => {
                                                    f47(v0, v1.clone(), v43, false)?
                                                }
                                                Adt14::C5 => {
                                                    f47(v0, v1.clone(), v43, false)?
                                                }
                                                Adt14::C6(_, _) => {
                                                    f47(v0, v1.clone(), v43, false)?
                                                }
                                            }
                                        }
                                    }
                                };
                                match m260 {
                                    Ok(v58) => {
                                        let m262 = f45(v58, match v1.clone() {
                                            Adt27::C0(_, h261, _, _) => {
                                                h261
                                            }
                                        }, v40)?;
                                        match m262 {
                                            Ok(v59) => {
                                                Ok::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>((v59, v45))
                                            }
                                            Err(v60) => {
                                                Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Err::<Adt28, Bytes>(v60)))
                                            }
                                        }
                                    }
                                    Err(v61) => {
                                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Err::<Adt28, Bytes>(v61)))
                                    }
                                }
                            }
                            Err(v62) => {
                                Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Err::<Adt28, Bytes>(v62)))
                            }
                        }
                    }
                    Adt17::C15(_, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C16(_, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                }
            };
            match step {
                Ok(next) => { m265 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f47(v0: Adt28, v1: Adt27, v2: Adt17, v3: bool) -> R<Result<Adt28, Bytes>> {
    let m268 = v2;
    match m268 {
        Adt17::C0(v4, v5) => {
            let v6: bool = if v3 {
                match v1.clone() {
                    Adt27::C0(_, _, h269, _) => {
                        h269
                    }
                }
            } else {
                false
            };
            let m270 = v6;
            if m270 {
                Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("arithmetic: the value a fallible function returns, ")), f4(f61(match v1 {
                    Adt27::C0(_, _, _, h271) => {
                        h271
                    }
                }, Adt17::C0(v4, v5))?, f1(Str::lit(", is not a fallible value"))))))
            } else {
                let m272 = v4;
                match m272 {
                    Adt18::C0 => {
                        Ok(Ok::<Adt28, Bytes>(v0))
                    }
                    Adt18::C1 => {
                        Ok(Ok::<Adt28, Bytes>(v0))
                    }
                    Adt18::C2 => {
                        Ok(Ok::<Adt28, Bytes>(v0))
                    }
                    Adt18::C3 => {
                        Ok(Ok::<Adt28, Bytes>(v0))
                    }
                    Adt18::C4(_) => {
                        Ok(Ok::<Adt28, Bytes>(v0))
                    }
                    Adt18::C5 => {
                        Ok(f62(v1, v0, f1(Str::lit("a string literal"))))
                    }
                    Adt18::C6 => {
                        Ok(f62(v1, v0, f1(Str::lit("a byte string literal"))))
                    }
                    Adt18::C7 => {
                        Ok(Ok::<Adt28, Bytes>(v0))
                    }
                    Adt18::C8 => {
                        Ok(Err::<Adt28, Bytes>(f1(Str::lit("overflow: the overflow value stands outside the payload of an `Err`"))))
                    }
                }
            }
        }
        Adt17::C1(v8, v9, v10) => {
            let v11: bool = if v3 {
                match v1.clone() {
                    Adt27::C0(_, _, h273, _) => {
                        h273
                    }
                }
            } else {
                false
            };
            let m274 = v11;
            if m274 {
                Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("arithmetic: the value a fallible function returns, ")), f4(f61(match v1 {
                    Adt27::C0(_, _, _, h275) => {
                        h275
                    }
                }, Adt17::C1(v8, v9, v10))?, f1(Str::lit(", is not a fallible value"))))))
            } else {
                let m276 = v8;
                match m276 {
                    Adt19::C0 => {
                        f63(v0, v9)
                    }
                    Adt19::C1 => {
                        let m277 = f64(v0.clone(), v9)?;
                        match m277 {
                            Ok(_) => {
                                Ok(Ok::<Adt28, Bytes>(v0))
                            }
                            Err(v13) => {
                                Ok(Err::<Adt28, Bytes>(v13))
                            }
                        }
                    }
                    Adt19::C2 => {
                        let m278 = f64(v0.clone(), v9)?;
                        match m278 {
                            Ok(_) => {
                                Ok(Ok::<Adt28, Bytes>(v0))
                            }
                            Err(v15) => {
                                Ok(Err::<Adt28, Bytes>(v15))
                            }
                        }
                    }
                    Adt19::C3 => {
                        let m279 = f64(v0.clone(), v9)?;
                        match m279 {
                            Ok(_) => {
                                Ok(Ok::<Adt28, Bytes>(v0))
                            }
                            Err(v17) => {
                                Ok(Err::<Adt28, Bytes>(v17))
                            }
                        }
                    }
                    Adt19::C4 => {
                        let m280 = f44(v1, f1(Str::lit("a boxed read")));
                        match m280 {
                            Ok(_) => {
                                let m281 = f64(v0.clone(), v9)?;
                                match m281 {
                                    Ok(_) => {
                                        Ok(Ok::<Adt28, Bytes>(v0))
                                    }
                                    Err(v20) => {
                                        Ok(Err::<Adt28, Bytes>(v20))
                                    }
                                }
                            }
                            Err(v21) => {
                                Ok(Err::<Adt28, Bytes>(v21))
                            }
                        }
                    }
                }
            }
        }
        Adt17::C2(v22, r414, v24) => {
            let v23 = (*r414).clone();
            let m282 = v22.clone();
            match m282 {
                Adt20::C0 => {
                    let v25: bool = if v3 {
                        match v1.clone() {
                            Adt27::C0(_, _, h283, _) => {
                                h283
                            }
                        }
                    } else {
                        false
                    };
                    let m284 = v25;
                    if m284 {
                        Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("arithmetic: the value a fallible function returns, ")), f4(f61(match v1 {
                            Adt27::C0(_, _, _, h285) => {
                                h285
                            }
                        }, Adt17::C2(v22, std::rc::Rc::new(v23), v24))?, f1(Str::lit(", is not a fallible value"))))))
                    } else {
                        let m286 = f65(v23.clone());
                        if m286 {
                            Ok(Err::<Adt28, Bytes>(f1(Str::lit("a negated zero test or predecessor: a zero test is negated as its complement, and a predecessor is not a Boolean"))))
                        } else {
                            f47(v0, v1, v23, false)
                        }
                    }
                }
                Adt20::C1 => {
                    let v26: bool = if v3 {
                        match v1.clone() {
                            Adt27::C0(_, _, h287, _) => {
                                h287
                            }
                        }
                    } else {
                        false
                    };
                    let m288 = v26;
                    if m288 {
                        Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("arithmetic: the value a fallible function returns, ")), f4(f61(match v1 {
                            Adt27::C0(_, _, _, h289) => {
                                h289
                            }
                        }, Adt17::C2(v22, std::rc::Rc::new(v23), v24))?, f1(Str::lit(", is not a fallible value"))))))
                    } else {
                        let m290 = f44(v1.clone(), f1(Str::lit("a box")));
                        match m290 {
                            Ok(_) => {
                                f47(v0, v1, v23, false)
                            }
                            Err(v28) => {
                                Ok(Err::<Adt28, Bytes>(v28))
                            }
                        }
                    }
                }
                Adt20::C2 => {
                    let v29: bool = if v3 {
                        match v1.clone() {
                            Adt27::C0(_, _, h291, _) => {
                                h291
                            }
                        }
                    } else {
                        false
                    };
                    let m292 = v29;
                    if m292 {
                        Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("arithmetic: the value a fallible function returns, ")), f4(f61(match v1 {
                            Adt27::C0(_, _, _, h293) => {
                                h293
                            }
                        }, Adt17::C2(v22, std::rc::Rc::new(v23), v24))?, f1(Str::lit(", is not a fallible value"))))))
                    } else {
                        f47(v0, v1, v23, false)
                    }
                }
                Adt20::C3 => {
                    f47(v0, v1, v23, false)
                }
            }
        }
        Adt17::C3(v30, r415, v32, v33) => {
            let v31 = (*r415).clone();
            let m294 = v32;
            if m294 {
                let v34: bool = if v3 {
                    match v1.clone() {
                        Adt27::C0(_, _, h295, _) => {
                            h295
                        }
                    }
                } else {
                    false
                };
                let m296 = v34;
                if m296 {
                    Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("arithmetic: the value a fallible function returns, ")), f4(f61(match v1 {
                        Adt27::C0(_, _, _, h297) => {
                            h297
                        }
                    }, Adt17::C3(v30, std::rc::Rc::new(v31), v32, v33))?, f1(Str::lit(", is not a fallible value"))))))
                } else {
                    let m298 = f66(v0, v1.clone(), false, v31.clone())?;
                    match m298 {
                        Ok(v35) => {
                            let m299 = v30;
                            match m299 {
                                Adt21::C0(v36) => {
                                    let m303 = f20({
                                        let m302 = match v1.clone() {
                                            Adt27::C0(_, h301, _, _) => {
                                                h301
                                            }
                                        };
                                        match m302 {
                                            Adt26::C0(_, _, _, h300) => {
                                                h300
                                            }
                                        }
                                    }, v36);
                                    match m303 {
                                        None => {
                                            Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("f")), f4(f10(v36)?, f1(Str::lit(" is not declared"))))))
                                        }
                                        Some(v37) => {
                                            Ok(f67(v35, v1, v37, v32, v3, f4(f1(Str::lit("f")), f10(v36)?)))
                                        }
                                    }
                                }
                                Adt21::C1(v39) => {
                                    let v40: Bytes = f69(match v1.clone() {
                                        Adt27::C0(_, _, _, h304) => {
                                            h304
                                        }
                                    }, v39.clone());
                                    let m307 = {
                                        let m306 = f70(match v1.clone() {
                                            Adt27::C0(_, _, _, h305) => {
                                                h305
                                            }
                                        }, v39.clone());
                                        if m306 {
                                            f44(v1.clone(), f4(f1(Str::lit("the runtime function ")), f30(v40.clone())))
                                        } else {
                                            Ok::<(), Bytes>(())
                                        }
                                    };
                                    match m307 {
                                        Ok(_) => {
                                            Ok(f67(v35, v1.clone(), f71(match v1 {
                                                Adt27::C0(_, _, _, h308) => {
                                                    h308
                                                }
                                            }, v39), v32, v3, f30(v40)))
                                        }
                                        Err(v42) => {
                                            Ok(Err::<Adt28, Bytes>(v42))
                                        }
                                    }
                                }
                                Adt21::C2(v38) => {
                                    f68(v35, v1, v38, v31, v32, v3)
                                }
                            }
                        }
                        Err(v43) => {
                            Ok(Err::<Adt28, Bytes>(v43))
                        }
                    }
                }
            } else {
                let m309 = f66(v0, v1.clone(), false, v31.clone())?;
                match m309 {
                    Ok(v44) => {
                        let m310 = v30;
                        match m310 {
                            Adt21::C0(v45) => {
                                let m314 = f20({
                                    let m313 = match v1.clone() {
                                        Adt27::C0(_, h312, _, _) => {
                                            h312
                                        }
                                    };
                                    match m313 {
                                        Adt26::C0(_, _, _, h311) => {
                                            h311
                                        }
                                    }
                                }, v45);
                                match m314 {
                                    None => {
                                        Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("f")), f4(f10(v45)?, f1(Str::lit(" is not declared"))))))
                                    }
                                    Some(v46) => {
                                        Ok(f67(v44, v1, v46, v32, v3, f4(f1(Str::lit("f")), f10(v45)?)))
                                    }
                                }
                            }
                            Adt21::C1(v48) => {
                                let v49: Bytes = f69(match v1.clone() {
                                    Adt27::C0(_, _, _, h315) => {
                                        h315
                                    }
                                }, v48.clone());
                                let m318 = {
                                    let m317 = f70(match v1.clone() {
                                        Adt27::C0(_, _, _, h316) => {
                                            h316
                                        }
                                    }, v48.clone());
                                    if m317 {
                                        f44(v1.clone(), f4(f1(Str::lit("the runtime function ")), f30(v49.clone())))
                                    } else {
                                        Ok::<(), Bytes>(())
                                    }
                                };
                                match m318 {
                                    Ok(_) => {
                                        Ok(f67(v44, v1.clone(), f71(match v1 {
                                            Adt27::C0(_, _, _, h319) => {
                                                h319
                                            }
                                        }, v48), v32, v3, f30(v49)))
                                    }
                                    Err(v51) => {
                                        Ok(Err::<Adt28, Bytes>(v51))
                                    }
                                }
                            }
                            Adt21::C2(v47) => {
                                f68(v44, v1, v47, v31, v32, v3)
                            }
                        }
                    }
                    Err(v52) => {
                        Ok(Err::<Adt28, Bytes>(v52))
                    }
                }
            }
        }
        Adt17::C4(v53, r416, v55, v56) => {
            let v54 = (*r416).clone();
            let m320 = v55;
            if m320 {
                let v57: bool = if v3 {
                    match v1.clone() {
                        Adt27::C0(_, _, h321, _) => {
                            h321
                        }
                    }
                } else {
                    false
                };
                let m322 = v57;
                if m322 {
                    Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("arithmetic: the value a fallible function returns, ")), f4(f61(match v1 {
                        Adt27::C0(_, _, _, h323) => {
                            h323
                        }
                    }, Adt17::C4(v53, std::rc::Rc::new(v54), v55, v56))?, f1(Str::lit(", is not a fallible value"))))))
                } else {
                    let m324 = f64(v0.clone(), v53)?;
                    match m324 {
                        Ok(_) => {
                            let m325 = f66(v0, v1, false, v54)?;
                            match m325 {
                                Ok(v59) => {
                                    Ok(Ok::<Adt28, Bytes>(Adt28::C0(match v59.clone() {
                                        Adt28::C0(h326, _, _, _, _) => {
                                            h326
                                        }
                                    }, match v59.clone() {
                                        Adt28::C0(_, h327, _, _, _) => {
                                            h327
                                        }
                                    }, match v59.clone() {
                                        Adt28::C0(_, _, h328, _, _) => {
                                            h328
                                        }
                                    }, match v59.clone() {
                                        Adt28::C0(_, _, _, h329, _) => {
                                            h329
                                        }
                                    }, if v55 {
                                        true
                                    } else {
                                        match v59 {
                                            Adt28::C0(_, _, _, _, h330) => {
                                                h330
                                            }
                                        }
                                    })))
                                }
                                Err(v60) => {
                                    Ok(Err::<Adt28, Bytes>(v60))
                                }
                            }
                        }
                        Err(v61) => {
                            Ok(Err::<Adt28, Bytes>(v61))
                        }
                    }
                }
            } else {
                let m331 = f64(v0.clone(), v53)?;
                match m331 {
                    Ok(_) => {
                        let m332 = f66(v0, v1, false, v54)?;
                        match m332 {
                            Ok(v63) => {
                                Ok(Ok::<Adt28, Bytes>(Adt28::C0(match v63.clone() {
                                    Adt28::C0(h333, _, _, _, _) => {
                                        h333
                                    }
                                }, match v63.clone() {
                                    Adt28::C0(_, h334, _, _, _) => {
                                        h334
                                    }
                                }, match v63.clone() {
                                    Adt28::C0(_, _, h335, _, _) => {
                                        h335
                                    }
                                }, match v63.clone() {
                                    Adt28::C0(_, _, _, h336, _) => {
                                        h336
                                    }
                                }, if v55 {
                                    true
                                } else {
                                    match v63 {
                                        Adt28::C0(_, _, _, _, h337) => {
                                            h337
                                        }
                                    }
                                })))
                            }
                            Err(v64) => {
                                Ok(Err::<Adt28, Bytes>(v64))
                            }
                        }
                    }
                    Err(v65) => {
                        Ok(Err::<Adt28, Bytes>(v65))
                    }
                }
            }
        }
        Adt17::C5(v66, r417, v68) => {
            let v67 = (*r417).clone();
            let v69: bool = if v3 {
                if match v1.clone() {
                    Adt27::C0(_, _, h338, _) => {
                        h338
                    }
                } {
                    let a339 = f72(v66.clone(), v67.clone())?;
                    bool_not(a339)
                } else {
                    false
                }
            } else {
                false
            };
            let m340 = v69;
            if m340 {
                Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("arithmetic: the value a fallible function returns, ")), f4(f61(match v1 {
                    Adt27::C0(_, _, _, h341) => {
                        h341
                    }
                }, Adt17::C5(v66, std::rc::Rc::new(v67), v68))?, f1(Str::lit(", is not a fallible value"))))))
            } else {
                let m342 = f66(v0, v1.clone(), f73(v66.clone()), v67.clone())?;
                match m342 {
                    Ok(v70) => {
                        let m343 = f74(v1, v66, f75(v67)?)?;
                        match m343 {
                            Ok(_) => {
                                Ok(Ok::<Adt28, Bytes>(v70))
                            }
                            Err(v72) => {
                                Ok(Err::<Adt28, Bytes>(v72))
                            }
                        }
                    }
                    Err(v73) => {
                        Ok(Err::<Adt28, Bytes>(v73))
                    }
                }
            }
        }
        Adt17::C6(r418, r419, v76) => {
            let v74 = (*r418).clone();
            let v75 = (*r419).clone();
            let v77: bool = if v3 {
                match v1.clone() {
                    Adt27::C0(_, _, h344, _) => {
                        h344
                    }
                }
            } else {
                false
            };
            let m345 = v77;
            if m345 {
                Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("arithmetic: the value a fallible function returns, ")), f4(f61(match v1 {
                    Adt27::C0(_, _, _, h346) => {
                        h346
                    }
                }, Adt17::C6(std::rc::Rc::new(v74), std::rc::Rc::new(v75), v76))?, f1(Str::lit(", is not a fallible value"))))))
            } else {
                let m347 = f47(v0, v1.clone(), v74, false)?;
                match m347 {
                    Ok(v78) => {
                        f47(v78, v1, v75, false)
                    }
                    Err(v79) => {
                        Ok(Err::<Adt28, Bytes>(v79))
                    }
                }
            }
        }
        Adt17::C7(r420, r421, r422, _) => {
            let v80 = (*r420).clone();
            let v81 = (*r421).clone();
            let v82 = (*r422).clone();
            let m348 = f47(v0, v1.clone(), v80, false)?;
            match m348 {
                Ok(v84) => {
                    let v85: Adt28 = Adt28::C0(match v84.clone() {
                        Adt28::C0(h349, _, _, _, _) => {
                            h349
                        }
                    }, match v84.clone() {
                        Adt28::C0(_, h350, _, _, _) => {
                            h350
                        }
                    }, match v84.clone() {
                        Adt28::C0(_, _, h351, _, _) => {
                            h351
                        }
                    }, match v84.clone() {
                        Adt28::C0(_, _, _, h352, _) => {
                            h352
                        }
                    }, match v84.clone() {
                        Adt28::C0(_, _, _, _, h353) => {
                            h353
                        }
                    });
                    let m355 = f45(v85, match v1.clone() {
                        Adt27::C0(_, h354, _, _) => {
                            h354
                        }
                    }, Adt14::C0(Adt15::C0))?;
                    match m355 {
                        Ok(v86) => {
                            let m356 = f29(v86, v1.clone(), v81, v3)?;
                            match m356 {
                                Ok(v87) => {
                                    let v88: Adt28 = Adt28::C0(match v87.clone() {
                                        Adt28::C0(h357, _, _, _, _) => {
                                            h357
                                        }
                                    }, match v84.clone() {
                                        Adt28::C0(_, h358, _, _, _) => {
                                            h358
                                        }
                                    }, f76(match v84.clone() {
                                        Adt28::C0(_, _, h359, _, _) => {
                                            h359
                                        }
                                    }, match v87.clone() {
                                        Adt28::C0(_, _, h360, _, _) => {
                                            h360
                                        }
                                    }), f77(match v84.clone() {
                                        Adt28::C0(_, _, _, h361, _) => {
                                            h361
                                        }
                                    }, match v87.clone() {
                                        Adt28::C0(_, _, _, h362, _) => {
                                            h362
                                        }
                                    }), match v87 {
                                        Adt28::C0(_, _, _, _, h363) => {
                                            h363
                                        }
                                    });
                                    let v89: Adt28 = Adt28::C0(match v88.clone() {
                                        Adt28::C0(h364, _, _, _, _) => {
                                            h364
                                        }
                                    }, match v84.clone() {
                                        Adt28::C0(_, h365, _, _, _) => {
                                            h365
                                        }
                                    }, match v84.clone() {
                                        Adt28::C0(_, _, h366, _, _) => {
                                            h366
                                        }
                                    }, match v84.clone() {
                                        Adt28::C0(_, _, _, h367, _) => {
                                            h367
                                        }
                                    }, match v88.clone() {
                                        Adt28::C0(_, _, _, _, h368) => {
                                            h368
                                        }
                                    });
                                    let m370 = f45(v89, match v1.clone() {
                                        Adt27::C0(_, h369, _, _) => {
                                            h369
                                        }
                                    }, Adt14::C0(Adt15::C0))?;
                                    match m370 {
                                        Ok(v90) => {
                                            let m371 = f29(v90, v1, v82, v3)?;
                                            match m371 {
                                                Ok(v91) => {
                                                    let v92: Adt28 = Adt28::C0(match v91.clone() {
                                                        Adt28::C0(h372, _, _, _, _) => {
                                                            h372
                                                        }
                                                    }, match v84 {
                                                        Adt28::C0(_, h373, _, _, _) => {
                                                            h373
                                                        }
                                                    }, f76(match v88.clone() {
                                                        Adt28::C0(_, _, h374, _, _) => {
                                                            h374
                                                        }
                                                    }, match v91.clone() {
                                                        Adt28::C0(_, _, h375, _, _) => {
                                                            h375
                                                        }
                                                    }), f77(match v88 {
                                                        Adt28::C0(_, _, _, h376, _) => {
                                                            h376
                                                        }
                                                    }, match v91.clone() {
                                                        Adt28::C0(_, _, _, h377, _) => {
                                                            h377
                                                        }
                                                    }), match v91 {
                                                        Adt28::C0(_, _, _, _, h378) => {
                                                            h378
                                                        }
                                                    });
                                                    Ok(Ok::<Adt28, Bytes>(v92))
                                                }
                                                Err(v93) => {
                                                    Ok(Err::<Adt28, Bytes>(v93))
                                                }
                                            }
                                        }
                                        Err(v94) => {
                                            Ok(Err::<Adt28, Bytes>(v94))
                                        }
                                    }
                                }
                                Err(v95) => {
                                    Ok(Err::<Adt28, Bytes>(v95))
                                }
                            }
                        }
                        Err(v96) => {
                            Ok(Err::<Adt28, Bytes>(v96))
                        }
                    }
                }
                Err(v97) => {
                    Ok(Err::<Adt28, Bytes>(v97))
                }
            }
        }
        Adt17::C8(r423, r424, _) => {
            let v98 = (*r423).clone();
            let v99 = (*r424).clone();
            let m379 = f47(v0, v1.clone(), v98, false)?;
            match m379 {
                Ok(v101) => {
                    f78(v101.clone(), v1, v101.clone(), match v101 {
                        Adt28::C0(_, h380, _, _, _) => {
                            h380
                        }
                    }, v99, v3)
                }
                Err(v102) => {
                    Ok(Err::<Adt28, Bytes>(v102))
                }
            }
        }
        Adt17::C9(r425) => {
            let v103 = (*r425).clone();
            f29(v0, v1, v103, v3)
        }
        Adt17::C10(v104, v105, v106) => {
            let v107: bool = if v3 {
                match v1.clone() {
                    Adt27::C0(_, _, h381, _) => {
                        h381
                    }
                }
            } else {
                false
            };
            let m382 = v107;
            if m382 {
                Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("arithmetic: the value a fallible function returns, ")), f4(f61(match v1 {
                    Adt27::C0(_, _, _, h383) => {
                        h383
                    }
                }, Adt17::C10(v104, v105, v106))?, f1(Str::lit(", is not a fallible value"))))))
            } else {
                let m384 = v104;
                match m384 {
                    Adt24::C0 => {
                        let m385 = f64(v0.clone(), v105)?;
                        match m385 {
                            Ok(_) => {
                                Ok(Ok::<Adt28, Bytes>(v0))
                            }
                            Err(v109) => {
                                Ok(Err::<Adt28, Bytes>(v109))
                            }
                        }
                    }
                    Adt24::C1 => {
                        let m386 = f64(v0.clone(), v105)?;
                        match m386 {
                            Ok(_) => {
                                Ok(Ok::<Adt28, Bytes>(v0))
                            }
                            Err(v111) => {
                                Ok(Err::<Adt28, Bytes>(v111))
                            }
                        }
                    }
                    Adt24::C2 => {
                        let m387 = f44(v1, f1(Str::lit("a list match")));
                        match m387 {
                            Ok(_) => {
                                let m388 = f64(v0.clone(), v105)?;
                                match m388 {
                                    Ok(_) => {
                                        Ok(Ok::<Adt28, Bytes>(v0))
                                    }
                                    Err(v114) => {
                                        Ok(Err::<Adt28, Bytes>(v114))
                                    }
                                }
                            }
                            Err(v115) => {
                                Ok(Err::<Adt28, Bytes>(v115))
                            }
                        }
                    }
                    Adt24::C3 => {
                        let m389 = f64(v0.clone(), v105)?;
                        match m389 {
                            Ok(_) => {
                                Ok(Ok::<Adt28, Bytes>(v0))
                            }
                            Err(v117) => {
                                Ok(Err::<Adt28, Bytes>(v117))
                            }
                        }
                    }
                }
            }
        }
        Adt17::C11(_, _) => {
            Ok(Ok::<Adt28, Bytes>(v0))
        }
        Adt17::C12 => {
            Ok(Ok::<Adt28, Bytes>(v0))
        }
        Adt17::C13(_, _) => {
            Ok(Ok::<Adt28, Bytes>(v0))
        }
        Adt17::C14(_, _, _, _, _, _) => {
            Ok(Ok::<Adt28, Bytes>(v0))
        }
        Adt17::C15(_, _, _) => {
            Ok(Ok::<Adt28, Bytes>(v0))
        }
        Adt17::C16(v131, r426) => {
            let v132 = (*r426).clone();
            let v133: bool = if v3 {
                match v1.clone() {
                    Adt27::C0(_, _, h390, _) => {
                        h390
                    }
                }
            } else {
                false
            };
            let v134: Result<(), Bytes> = {
                let m391 = v133;
                if m391 {
                    f79(match v1.clone() {
                        Adt27::C0(_, _, _, h392) => {
                            h392
                        }
                    }, v132.clone())?
                } else {
                    Ok::<(), Bytes>(())
                }
            };
            let m393 = v134;
            match m393 {
                Ok(_) => {
                    let m394 = f63(v0, v131.clone())?;
                    match m394 {
                        Ok(v136) => {
                            let v137: Adt28 = Adt28::C0(match v136.clone() {
                                Adt28::C0(h395, _, _, _, _) => {
                                    h395
                                }
                            }, match v136.clone() {
                                Adt28::C0(_, h396, _, _, _) => {
                                    h396
                                }
                            }, f80(match v136.clone() {
                                Adt28::C0(_, _, h397, _, _) => {
                                    h397
                                }
                            }, v131.clone()), match v136.clone() {
                                Adt28::C0(_, _, _, h398, _) => {
                                    h398
                                }
                            }, match v136.clone() {
                                Adt28::C0(_, _, _, _, h399) => {
                                    h399
                                }
                            });
                            let m400 = f29(v137.clone(), v1, v132, false)?;
                            match m400 {
                                Ok(v138) => {
                                    let m404 = f81(match v138.clone() {
                                        Adt28::C0(_, _, h401, _, _) => {
                                            h401
                                        }
                                    }, v131, match v137.clone() {
                                        Adt28::C0(_, _, h402, _, _) => {
                                            h402
                                        }
                                    }, match v136.clone() {
                                        Adt28::C0(_, h403, _, _, _) => {
                                            h403
                                        }
                                    });
                                    match m404 {
                                        None => {
                                            let m408 = f82(match v138.clone() {
                                                Adt28::C0(_, _, _, h405, _) => {
                                                    h405
                                                }
                                            }, match v137 {
                                                Adt28::C0(_, _, _, h406, _) => {
                                                    h406
                                                }
                                            }, match v136.clone() {
                                                Adt28::C0(_, h407, _, _, _) => {
                                                    h407
                                                }
                                            });
                                            match m408 {
                                                None => {
                                                    Ok(Ok::<Adt28, Bytes>(Adt28::C0(match v138.clone() {
                                                        Adt28::C0(h409, _, _, _, _) => {
                                                            h409
                                                        }
                                                    }, match v138.clone() {
                                                        Adt28::C0(_, h410, _, _, _) => {
                                                            h410
                                                        }
                                                    }, match v136.clone() {
                                                        Adt28::C0(_, _, h411, _, _) => {
                                                            h411
                                                        }
                                                    }, match v136 {
                                                        Adt28::C0(_, _, _, h412, _) => {
                                                            h412
                                                        }
                                                    }, match v138 {
                                                        Adt28::C0(_, _, _, _, h413) => {
                                                            h413
                                                        }
                                                    })))
                                                }
                                                Some(v140) => {
                                                    Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("ownership: a component of ")), f4(f30(f31(v140)?), f1(Str::lit(" is moved in the body of a loop that runs again"))))))
                                                }
                                            }
                                        }
                                        Some(v139) => {
                                            Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("ownership: ")), f4(f30(f31(v139)?), f1(Str::lit(" is moved in the body of a loop that runs again"))))))
                                        }
                                    }
                                }
                                Err(v141) => {
                                    Ok(Err::<Adt28, Bytes>(v141))
                                }
                            }
                        }
                        Err(v142) => {
                            Ok(Err::<Adt28, Bytes>(v142))
                        }
                    }
                }
                Err(v143) => {
                    Ok(Err::<Adt28, Bytes>(v143))
                }
            }
        }
    }
}

pub fn f48(p433: List<(Adt12, u64)>, v1: Adt26, p434: Adt17) -> R<Result<List<(Adt12, u64)>, Bytes>> {
    let m432 = (p433, p434);
    {
        let mut m432 = m432;
        loop {
            let step = {
                let (v0, v2) = m432;
                let m427 = v2;
                match m427 {
                    Adt17::C0(_, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C1(_, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C2(_, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C3(_, _, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C4(_, _, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C5(_, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C6(_, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C7(_, _, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C8(_, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C9(_) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C10(_, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C11(_, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C12 => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C13(_, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C14(v40, v41, v42, r430, _, r431) => {
                        let v43 = (*r430).clone();
                        let v45 = (*r431).clone();
                        let v46: List<(Adt12, u64)> = {
                            let m428 = v41;
                            if m428 {
                                f50(v0, v40, v42)
                            } else {
                                v0
                            }
                        };
                        let m429 = f49(v46, v1.clone(), v43, false)?;
                        match m429 {
                            Ok(v47) => {
                                Ok::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>((v47, v45))
                            }
                            Err(v48) => {
                                Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Err::<List<(Adt12, u64)>, Bytes>(v48)))
                            }
                        }
                    }
                    Adt17::C15(_, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C16(_, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                }
            };
            match step {
                Ok(next) => { m432 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f49(v0: List<(Adt12, u64)>, v1: Adt26, v2: Adt17, v3: bool) -> R<Result<List<(Adt12, u64)>, Bytes>> {
    let m435 = v2;
    match m435 {
        Adt17::C0(_, _) => {
            Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0))
        }
        Adt17::C1(_, _, _) => {
            Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0))
        }
        Adt17::C2(_, r445, _) => {
            let v10 = (*r445).clone();
            f49(v0, v1, v10, false)
        }
        Adt17::C3(_, r446, _, _) => {
            let v13 = (*r446).clone();
            f83(v0, v1, v13)
        }
        Adt17::C4(v16, r447, v18, _) => {
            let v17 = (*r447).clone();
            let m436 = f84(v0.clone(), v16.clone());
            match m436 {
                None => {
                    Ok(Err::<List<(Adt12, u64)>, Bytes>(f4(f30(f31(v16)?), f1(Str::lit(" is applied without a function type")))))
                }
                Some(v20) => {
                    let m438 = f20(match v1.clone() {
                        Adt26::C0(_, _, h437, _) => {
                            h437
                        }
                    }, v20);
                    match m438 {
                        None => {
                            Ok(Err::<List<(Adt12, u64)>, Bytes>(f4(f1(Str::lit("Fn")), f4(f10(v20)?, f1(Str::lit("::apply is not declared"))))))
                        }
                        Some(v21) => {
                            let v22: bool = if v21 {
                                !v18
                            } else {
                                v18
                            };
                            let v23: bool = if v3 {
                                v21
                            } else {
                                false
                            };
                            let m439 = v22;
                            if m439 {
                                let m440 = v23;
                                if m440 {
                                    f83(v0, v1, v17)
                                } else {
                                    Ok(Err::<List<(Adt12, u64)>, Bytes>(f4(f1(Str::lit("arithmetic: an application of Fn")), f4(f10(v20)?, f4(f1(Str::lit(" ")), f4(if v18 {
                                        f1(Str::lit("propagates"))
                                    } else {
                                        f1(Str::lit("drops"))
                                    }, f4(f1(Str::lit(" a failure its apply ")), if v21 {
                                        f1(Str::lit("can have"))
                                    } else {
                                        f1(Str::lit("cannot have"))
                                    })))))))
                                }
                            } else {
                                f83(v0, v1, v17)
                            }
                        }
                    }
                }
            }
        }
        Adt17::C5(_, r448, _) => {
            let v25 = (*r448).clone();
            f83(v0, v1, v25)
        }
        Adt17::C6(r449, r450, _) => {
            let v27 = (*r449).clone();
            let v28 = (*r450).clone();
            let m441 = f49(v0, v1.clone(), v27, false)?;
            match m441 {
                Ok(v30) => {
                    f49(v30, v1, v28, false)
                }
                Err(v31) => {
                    Ok(Err::<List<(Adt12, u64)>, Bytes>(v31))
                }
            }
        }
        Adt17::C7(r451, r452, r453, _) => {
            let v32 = (*r451).clone();
            let v33 = (*r452).clone();
            let v34 = (*r453).clone();
            let m442 = f49(v0, v1.clone(), v32, false)?;
            match m442 {
                Ok(v36) => {
                    let m443 = f32(v36, v1.clone(), v33, v3)?;
                    match m443 {
                        Ok(v37) => {
                            f32(v37, v1, v34, v3)
                        }
                        Err(v38) => {
                            Ok(Err::<List<(Adt12, u64)>, Bytes>(v38))
                        }
                    }
                }
                Err(v39) => {
                    Ok(Err::<List<(Adt12, u64)>, Bytes>(v39))
                }
            }
        }
        Adt17::C8(r454, r455, _) => {
            let v40 = (*r454).clone();
            let v41 = (*r455).clone();
            let m444 = f49(v0, v1.clone(), v40, false)?;
            match m444 {
                Ok(v43) => {
                    f85(v43, v1, v41, v3)
                }
                Err(v44) => {
                    Ok(Err::<List<(Adt12, u64)>, Bytes>(v44))
                }
            }
        }
        Adt17::C9(r456) => {
            let v45 = (*r456).clone();
            f32(v0, v1, v45, v3)
        }
        Adt17::C10(_, _, _) => {
            Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0))
        }
        Adt17::C11(_, _) => {
            Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0))
        }
        Adt17::C12 => {
            Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0))
        }
        Adt17::C13(_, _) => {
            Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0))
        }
        Adt17::C14(_, _, _, _, _, _) => {
            Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0))
        }
        Adt17::C15(_, _, _) => {
            Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0))
        }
        Adt17::C16(_, r457) => {
            let v63 = (*r457).clone();
            f32(v0, v1, v63, false)
        }
    }
}

pub fn f50(v0: List<(Adt12, u64)>, v1: Adt14, v2: Adt2) -> List<(Adt12, u64)> {
    let m458 = f86(v1);
    match m458 {
        None => {
            v0
        }
        Some(v3) => {
            let m459 = f87(v2);
            match m459 {
                None => {
                    v0
                }
                Some(v4) => {
                    List::cons((v3, v4), v0)
                }
            }
        }
    }
}

pub fn f51(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f88(v0, v1, 48u8) {
        Ok(Some({
            let a460 = v1;
            let a461 = 1u64;
            nat_add(a460, a461)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f52(v0: Adt3) -> bool {
    let m462 = v0;
    match m462 {
        Adt3::C0 => {
            false
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
            true
        }
        Adt3::C6 => {
            true
        }
        Adt3::C7 => {
            false
        }
        Adt3::C8 => {
            false
        }
    }
}

pub fn f53(v0: Adt5) -> bool {
    let m463 = v0;
    match m463 {
        Adt5::C0 => {
            false
        }
        Adt5::C1 => {
            true
        }
        Adt5::C2 => {
            true
        }
        Adt5::C3 => {
            false
        }
        Adt5::C4 => {
            false
        }
    }
}

pub fn f54(v0: Fn3, p467: bool, p468: List<Adt11>) -> bool {
    let m466 = (p467, p468);
    {
        let mut m466 = m466;
        loop {
            let step = {
                let (v1, v2) = m466;
                let m464 = v2;
                match m464.uncons() {
                    None => {
                        Err::<(bool, List<Adt11>), bool>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<Adt11>), bool>(({
                            let c465: Fn3 = v0.clone();
                            c465.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m466 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f55(v0: bool, v1: Adt11) -> bool {
    let m469 = v1;
    match m469 {
        Adt11::C0 => {
            v0
        }
        Adt11::C1 => {
            v0
        }
        Adt11::C2 => {
            true
        }
        Adt11::C3 => {
            v0
        }
    }
}

pub fn f56(v0: List<Adt12>, v1: Adt12) -> bool {
    f89(Fn4::F90(v1), false, v0)
}

pub fn f57(v0: Adt26, v1: u64, v2: u64) -> Option<u64> {
    let m471 = f18(match v0 {
        Adt26::C0(h470, _, _, _) => {
            h470
        }
    }, v1);
    match m471 {
        None => {
            None::<u64>
        }
        Some(v3) => {
            f91(v3, v2)
        }
    }
}

pub fn f58(v0: Adt14) -> R<u64> {
    let m472 = v0;
    match m472 {
        Adt14::C0(_) => {
            Ok(0u64)
        }
        Adt14::C1(_) => {
            Ok(0u64)
        }
        Adt14::C2(_) => {
            Ok(0u64)
        }
        Adt14::C3(_, _) => {
            Ok(0u64)
        }
        Adt14::C4(_, _, _) => {
            Ok(0u64)
        }
        Adt14::C5 => {
            Ok(0u64)
        }
        Adt14::C6(_, r475) => {
            let v10 = (*r475).clone();
            let v11: u64 = f58(v10)?;
            let a473 = 1u64;
            let a474 = v11;
            nat_add(a473, a474)
        }
    }
}

pub fn f59(v0: Adt14, v1: Adt17) -> Option<Adt12> {
    let m476 = v0;
    match m476 {
        Adt14::C0(_) => {
            None::<Adt12>
        }
        Adt14::C1(_) => {
            None::<Adt12>
        }
        Adt14::C2(r480) => {
            let v4 = (*r480).clone();
            let m477 = v1;
            match m477 {
                Adt17::C0(_, _) => {
                    None::<Adt12>
                }
                Adt17::C1(v7, v8, _) => {
                    let m478 = f92(v7);
                    if m478 {
                        let m479 = f93(v4);
                        if m479 {
                            Some(v8)
                        } else {
                            None::<Adt12>
                        }
                    } else {
                        None::<Adt12>
                    }
                }
                Adt17::C2(_, _, _) => {
                    None::<Adt12>
                }
                Adt17::C3(_, _, _, _) => {
                    None::<Adt12>
                }
                Adt17::C4(_, _, _, _) => {
                    None::<Adt12>
                }
                Adt17::C5(_, _, _) => {
                    None::<Adt12>
                }
                Adt17::C6(_, _, _) => {
                    None::<Adt12>
                }
                Adt17::C7(_, _, _, _) => {
                    None::<Adt12>
                }
                Adt17::C8(_, _, _) => {
                    None::<Adt12>
                }
                Adt17::C9(_) => {
                    None::<Adt12>
                }
                Adt17::C10(_, _, _) => {
                    None::<Adt12>
                }
                Adt17::C11(_, _) => {
                    None::<Adt12>
                }
                Adt17::C12 => {
                    None::<Adt12>
                }
                Adt17::C13(_, _) => {
                    None::<Adt12>
                }
                Adt17::C14(_, _, _, _, _, _) => {
                    None::<Adt12>
                }
                Adt17::C15(_, _, _) => {
                    None::<Adt12>
                }
                Adt17::C16(_, _) => {
                    None::<Adt12>
                }
            }
        }
        Adt14::C3(_, _) => {
            None::<Adt12>
        }
        Adt14::C4(_, _, _) => {
            None::<Adt12>
        }
        Adt14::C5 => {
            None::<Adt12>
        }
        Adt14::C6(_, _) => {
            None::<Adt12>
        }
    }
}

pub fn f60(v0: Adt28, v1: Adt12, v2: Adt14) -> R<Result<Adt28, Bytes>> {
    let m482 = f56(match v0.clone() {
        Adt28::C0(_, h481, _, _, _) => {
            h481
        }
    }, v1.clone());
    if m482 {
        let m484 = f56(match v0.clone() {
            Adt28::C0(_, _, h483, _, _) => {
                h483
            }
        }, v1.clone());
        if m484 {
            Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("ownership: a component of ")), f4(f30(f31(v1)?), f1(Str::lit(" is moved after it was moved"))))))
        } else {
            let v4: List<u64> = {
                let m486 = f94(match v0.clone() {
                    Adt28::C0(_, _, _, h485, _) => {
                        h485
                    }
                }, v1.clone());
                match m486 {
                    None => {
                        List::<u64>::nil()
                    }
                    Some(v3) => {
                        v3
                    }
                }
            };
            let m487 = f95(v4, 0u64, v2, f30(f31(v1.clone())?))?;
            match m487 {
                Ok(v5) => {
                    Ok(Ok::<Adt28, Bytes>(Adt28::C0(match v0.clone() {
                        Adt28::C0(h488, _, _, _, _) => {
                            h488
                        }
                    }, match v0.clone() {
                        Adt28::C0(_, h489, _, _, _) => {
                            h489
                        }
                    }, match v0.clone() {
                        Adt28::C0(_, _, h490, _, _) => {
                            h490
                        }
                    }, f96(match v0.clone() {
                        Adt28::C0(_, _, _, h491, _) => {
                            h491
                        }
                    }, v1, v5), match v0 {
                        Adt28::C0(_, _, _, _, h492) => {
                            h492
                        }
                    })))
                }
                Err(v6) => {
                    Ok(Err::<Adt28, Bytes>(v6))
                }
            }
        }
    } else {
        Ok(Err::<Adt28, Bytes>(f4(f30(f31(v1)?), f1(Str::lit(" is not bound")))))
    }
}

pub fn f61(v0: List<Adt25>, v1: Adt17) -> R<Bytes> {
    Ok(f97({
        let m494 = f98(Adt30::C0(List::<Adt29>::nil(), List::<Bytes>::nil(), v0), v1)?;
        match m494 {
            Adt30::C0(h493, _, _) => {
                h493
            }
        }
    }))
}

pub fn f62(v0: Adt27, v1: Adt28, v2: Bytes) -> Result<Adt28, Bytes> {
    let m496 = match v0 {
        Adt27::C0(h495, _, _, _) => {
            h495
        }
    };
    if m496 {
        Err::<Adt28, Bytes>(f4(f1(Str::lit("hidden allocation: rust-core renders ")), f4(v2, f1(Str::lit(", which requires heap allocation")))))
    } else {
        Ok::<Adt28, Bytes>(v1)
    }
}

pub fn f63(v0: Adt28, v1: Adt12) -> R<Result<Adt28, Bytes>> {
    let m498 = f56(match v0.clone() {
        Adt28::C0(_, h497, _, _, _) => {
            h497
        }
    }, v1.clone());
    if m498 {
        let m500 = f99(match v0.clone() {
            Adt28::C0(_, _, _, h499, _) => {
                h499
            }
        }, v1.clone());
        if m500 {
            Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("ownership: ")), f4(f30(f31(v1)?), f1(Str::lit(" is moved after a component was moved out of it"))))))
        } else {
            let m502 = f56(match v0.clone() {
                Adt28::C0(_, _, h501, _, _) => {
                    h501
                }
            }, v1.clone());
            if m502 {
                Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("ownership: ")), f4(f30(f31(v1)?), f1(Str::lit(" is moved twice"))))))
            } else {
                Ok(Ok::<Adt28, Bytes>(Adt28::C0(match v0.clone() {
                    Adt28::C0(h503, _, _, _, _) => {
                        h503
                    }
                }, match v0.clone() {
                    Adt28::C0(_, h504, _, _, _) => {
                        h504
                    }
                }, List::cons(v1, match v0.clone() {
                    Adt28::C0(_, _, h505, _, _) => {
                        h505
                    }
                }), match v0.clone() {
                    Adt28::C0(_, _, _, h506, _) => {
                        h506
                    }
                }, match v0 {
                    Adt28::C0(_, _, _, _, h507) => {
                        h507
                    }
                })))
            }
        }
    } else {
        Ok(Err::<Adt28, Bytes>(f4(f30(f31(v1)?), f1(Str::lit(" is not bound")))))
    }
}

pub fn f64(v0: Adt28, v1: Adt12) -> R<Result<(), Bytes>> {
    let m509 = f56(match v0.clone() {
        Adt28::C0(_, h508, _, _, _) => {
            h508
        }
    }, v1.clone());
    if m509 {
        let m511 = f56(match v0.clone() {
            Adt28::C0(_, _, h510, _, _) => {
                h510
            }
        }, v1.clone());
        if m511 {
            Ok(Err::<(), Bytes>(f4(f1(Str::lit("ownership: ")), f4(f30(f31(v1)?), f1(Str::lit(" is read after it was moved"))))))
        } else {
            let m513 = f99(match v0 {
                Adt28::C0(_, _, _, h512, _) => {
                    h512
                }
            }, v1.clone());
            if m513 {
                Ok(Err::<(), Bytes>(f4(f1(Str::lit("ownership: ")), f4(f30(f31(v1)?), f1(Str::lit(" is read after a component was moved out of it"))))))
            } else {
                Ok(Ok::<(), Bytes>(()))
            }
        }
    } else {
        Ok(Err::<(), Bytes>(f4(f30(f31(v1)?), f1(Str::lit(" is not bound")))))
    }
}

pub fn f65(v0: Adt17) -> bool {
    let m514 = v0;
    match m514 {
        Adt17::C0(_, _) => {
            false
        }
        Adt17::C1(_, _, _) => {
            false
        }
        Adt17::C2(_, _, _) => {
            false
        }
        Adt17::C3(_, _, _, _) => {
            false
        }
        Adt17::C4(_, _, _, _) => {
            false
        }
        Adt17::C5(_, _, _) => {
            false
        }
        Adt17::C6(_, _, _) => {
            false
        }
        Adt17::C7(_, _, _, _) => {
            false
        }
        Adt17::C8(_, _, _) => {
            false
        }
        Adt17::C9(_) => {
            false
        }
        Adt17::C10(v31, _, _) => {
            f100(v31)
        }
        Adt17::C11(_, _) => {
            false
        }
        Adt17::C12 => {
            false
        }
        Adt17::C13(_, _) => {
            false
        }
        Adt17::C14(_, _, _, _, _, _) => {
            false
        }
        Adt17::C15(_, _, _) => {
            false
        }
        Adt17::C16(_, _) => {
            false
        }
    }
}

pub fn f66(p521: Adt28, v1: Adt27, v2: bool, p522: Adt17) -> R<Result<Adt28, Bytes>> {
    let m520 = (p521, p522);
    {
        let mut m520 = m520;
        loop {
            let step = {
                let (v0, v3) = m520;
                let m515 = v3;
                match m515 {
                    Adt17::C0(_, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C1(_, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C2(_, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C3(_, _, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C4(_, _, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C5(_, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C6(_, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C7(_, _, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C8(_, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C9(_) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C10(_, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C11(_, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C12 => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C13(r518, r519) => {
                        let v39 = (*r518).clone();
                        let v40 = (*r519).clone();
                        let v41: bool = if v2 {
                            f101(v39.clone())
                        } else {
                            false
                        };
                        let m516 = v41;
                        if m516 {
                            Ok::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>((v0, v40))
                        } else {
                            let m517 = f47(v0, v1.clone(), v39, false)?;
                            match m517 {
                                Ok(v42) => {
                                    Ok::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>((v42, v40))
                                }
                                Err(v43) => {
                                    Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Err::<Adt28, Bytes>(v43)))
                                }
                            }
                        }
                    }
                    Adt17::C14(_, _, _, _, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C15(_, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                    Adt17::C16(_, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v0)))
                    }
                }
            };
            match step {
                Ok(next) => { m520 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f67(v0: Adt28, v1: Adt27, v2: bool, v3: bool, v4: bool, v5: Bytes) -> Result<Adt28, Bytes> {
    let v6: bool = if v4 {
        match v1 {
            Adt27::C0(_, _, h523, _) => {
                h523
            }
        }
    } else {
        false
    };
    let m524 = v2;
    if m524 {
        let m525 = v3;
        if m525 {
            Ok::<Adt28, Bytes>(Adt28::C0(match v0.clone() {
                Adt28::C0(h526, _, _, _, _) => {
                    h526
                }
            }, match v0.clone() {
                Adt28::C0(_, h527, _, _, _) => {
                    h527
                }
            }, match v0.clone() {
                Adt28::C0(_, _, h528, _, _) => {
                    h528
                }
            }, match v0 {
                Adt28::C0(_, _, _, h529, _) => {
                    h529
                }
            }, true))
        } else {
            let m530 = v6;
            if m530 {
                Ok::<Adt28, Bytes>(v0)
            } else {
                Err::<Adt28, Bytes>(f4(f1(Str::lit("arithmetic: the fallible call of ")), f4(v5, f1(Str::lit(" does not propagate its overflow")))))
            }
        }
    } else {
        let m531 = v3;
        if m531 {
            Err::<Adt28, Bytes>(f4(f1(Str::lit("arithmetic: the infallible call of ")), f4(v5, f1(Str::lit(" propagates a failure it cannot have")))))
        } else {
            Ok::<Adt28, Bytes>(v0)
        }
    }
}

pub fn f68(v0: Adt28, v1: Adt27, v2: u64, v3: Adt17, v4: bool, v5: bool) -> R<Result<Adt28, Bytes>> {
    let m534 = {
        let a532 = f75(v3.clone())?;
        let a533 = f102(v2);
        nat_eq(a532, a533)
    };
    if m534 {
        Ok(f67(v0, v1, false, v4, v5, f30(f103(v2))))
    } else {
        Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("machine: ")), f4(f30(f103(v2)), f4(f1(Str::lit(" receives ")), f4(f10(f75(v3)?)?, f4(f1(Str::lit(" operands of ")), f10(f102(v2))?)))))))
    }
}

pub fn f69(v0: List<Adt25>, v1: Adt22) -> Bytes {
    let m536 = f104(v0, match v1.clone() {
        Adt22::C0(h535, _) => {
            h535
        }
    });
    match m536 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            let a538 = match v2.clone() {
                Adt25::C0(_, _, h537, _, _) => {
                    h537
                }
            };
            let a539 = 0u64;
            if nat_eq(a538, a539) {
                match v2 {
                    Adt25::C0(_, h540, _, _, _) => {
                        h540
                    }
                }
            } else {
                let a542 = match v2.clone() {
                    Adt25::C0(_, _, h541, _, _) => {
                        h541
                    }
                };
                let a543 = 1u64;
                if nat_eq(a542, a543) {
                    f4(f1(Str::lit("fixed_")), f4(f105(f106(match v1 {
                        Adt22::C0(_, h544) => {
                            h544
                        }
                    })), f4(f1(Str::lit("::")), match v2 {
                        Adt25::C0(_, h545, _, _, _) => {
                            h545
                        }
                    })))
                } else {
                    f4(match v2 {
                        Adt25::C0(_, h546, _, _, _) => {
                            h546
                        }
                    }, f105(f106(match v1 {
                        Adt22::C0(_, h547) => {
                            h547
                        }
                    })))
                }
            }
        }
    }
}

pub fn f70(v0: List<Adt25>, v1: Adt22) -> bool {
    let m549 = f104(v0, match v1 {
        Adt22::C0(h548, _) => {
            h548
        }
    });
    match m549 {
        None => {
            false
        }
        Some(v2) => {
            match v2 {
                Adt25::C0(_, _, _, _, h550) => {
                    h550
                }
            }
        }
    }
}

pub fn f71(v0: List<Adt25>, v1: Adt22) -> bool {
    let m552 = f104(v0, match v1 {
        Adt22::C0(h551, _) => {
            h551
        }
    });
    match m552 {
        None => {
            false
        }
        Some(v2) => {
            match v2 {
                Adt25::C0(_, _, _, h553, _) => {
                    h553
                }
            }
        }
    }
}

pub fn f72(v0: Adt23, v1: Adt17) -> R<bool> {
    let m554 = v0;
    match m554 {
        Adt23::C0(_) => {
            Ok(false)
        }
        Adt23::C1 => {
            Ok(false)
        }
        Adt23::C2(_, _) => {
            Ok(false)
        }
        Adt23::C3(_, v6) => {
            let v7: bool = f107(v6);
            let v8: bool = f108(v1)?;
            if v7 {
                Ok(v8)
            } else {
                Ok(false)
            }
        }
        Adt23::C4(_, _) => {
            Ok(false)
        }
        Adt23::C5(_, _) => {
            Ok(false)
        }
        Adt23::C6 => {
            Ok(false)
        }
        Adt23::C7(_) => {
            Ok(false)
        }
    }
}

pub fn f73(v0: Adt23) -> bool {
    let m555 = v0;
    match m555 {
        Adt23::C0(_) => {
            false
        }
        Adt23::C1 => {
            false
        }
        Adt23::C2(_, _) => {
            false
        }
        Adt23::C3(_, _) => {
            true
        }
        Adt23::C4(_, _) => {
            false
        }
        Adt23::C5(_, _) => {
            false
        }
        Adt23::C6 => {
            false
        }
        Adt23::C7(_) => {
            false
        }
    }
}

pub fn f74(v0: Adt27, v1: Adt23, v2: u64) -> R<Result<(), Bytes>> {
    let m556 = v1;
    match m556 {
        Adt23::C0(v3) => {
            f26(v0, v3)
        }
        Adt23::C1 => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt23::C2(v4, v5) => {
            let m557 = f26(v0.clone(), v4)?;
            match m557 {
                Ok(_) => {
                    f26(v0, v5)
                }
                Err(v7) => {
                    Ok(Err::<(), Bytes>(v7))
                }
            }
        }
        Adt23::C3(v8, v9) => {
            let m558 = f26(v0.clone(), v8)?;
            match m558 {
                Ok(_) => {
                    f26(v0, v9)
                }
                Err(v11) => {
                    Ok(Err::<(), Bytes>(v11))
                }
            }
        }
        Adt23::C4(v12, v13) => {
            let m560 = f57(match v0 {
                Adt27::C0(_, h559, _, _) => {
                    h559
                }
            }, v12, v13);
            match m560 {
                None => {
                    Ok(Err::<(), Bytes>(f4(f1(Str::lit("Adt")), f4(f10(v12)?, f4(f1(Str::lit("::C")), f4(f10(v13)?, f1(Str::lit(" is not declared"))))))))
                }
                Some(v14) => {
                    let m563 = {
                        let a561 = v14;
                        let a562 = v2;
                        nat_eq(a561, a562)
                    };
                    if m563 {
                        Ok(Ok::<(), Bytes>(()))
                    } else {
                        Ok(Err::<(), Bytes>(f4(f1(Str::lit("Adt")), f4(f10(v12)?, f4(f1(Str::lit("::C")), f4(f10(v13)?, f4(f1(Str::lit(" receives ")), f4(f10(v2)?, f4(f1(Str::lit(" fields of ")), f10(v14)?)))))))))
                    }
                }
            }
        }
        Adt23::C5(v15, v16) => {
            let m565 = f109(match v0 {
                Adt27::C0(_, h564, _, _) => {
                    h564
                }
            }, v15, v16);
            match m565 {
                None => {
                    Ok(Err::<(), Bytes>(f4(f1(Str::lit("Fn")), f4(f10(v15)?, f4(f1(Str::lit("::F")), f4(f10(v16)?, f1(Str::lit(" is not declared"))))))))
                }
                Some(v17) => {
                    let m568 = {
                        let a566 = v17;
                        let a567 = v2;
                        nat_eq(a566, a567)
                    };
                    if m568 {
                        Ok(Ok::<(), Bytes>(()))
                    } else {
                        Ok(Err::<(), Bytes>(f4(f1(Str::lit("Fn")), f4(f10(v15)?, f4(f1(Str::lit("::F")), f4(f10(v16)?, f4(f1(Str::lit(" captures ")), f4(f10(v2)?, f4(f1(Str::lit(" values of ")), f10(v17)?)))))))))
                    }
                }
            }
        }
        Adt23::C6 => {
            Ok(f44(v0, f1(Str::lit("a list cell"))))
        }
        Adt23::C7(v18) => {
            let m569 = f44(v0.clone(), f1(Str::lit("an empty list")));
            match m569 {
                Ok(_) => {
                    f26(v0, v18)
                }
                Err(v20) => {
                    Ok(Err::<(), Bytes>(v20))
                }
            }
        }
    }
}

pub fn f75(v0: Adt17) -> R<u64> {
    let m570 = v0;
    match m570 {
        Adt17::C0(_, _) => {
            Ok(0u64)
        }
        Adt17::C1(_, _, _) => {
            Ok(0u64)
        }
        Adt17::C2(_, _, _) => {
            Ok(0u64)
        }
        Adt17::C3(_, _, _, _) => {
            Ok(0u64)
        }
        Adt17::C4(_, _, _, _) => {
            Ok(0u64)
        }
        Adt17::C5(_, _, _) => {
            Ok(0u64)
        }
        Adt17::C6(_, _, _) => {
            Ok(0u64)
        }
        Adt17::C7(_, _, _, _) => {
            Ok(0u64)
        }
        Adt17::C8(_, _, _) => {
            Ok(0u64)
        }
        Adt17::C9(_) => {
            Ok(0u64)
        }
        Adt17::C10(_, _, _) => {
            Ok(0u64)
        }
        Adt17::C11(_, _) => {
            Ok(0u64)
        }
        Adt17::C12 => {
            Ok(0u64)
        }
        Adt17::C13(_, r573) => {
            let v37 = (*r573).clone();
            let v38: u64 = f75(v37)?;
            let a571 = 1u64;
            let a572 = v38;
            nat_add(a571, a572)
        }
        Adt17::C14(_, _, _, _, _, _) => {
            Ok(0u64)
        }
        Adt17::C15(_, _, _) => {
            Ok(0u64)
        }
        Adt17::C16(_, _) => {
            Ok(0u64)
        }
    }
}

pub fn f76(v0: List<Adt12>, v1: List<Adt12>) -> List<Adt12> {
    f110(Fn5::F111, v0, v1)
}

pub fn f77(v0: List<(Adt12, List<u64>)>, v1: List<(Adt12, List<u64>)>) -> List<(Adt12, List<u64>)> {
    f112(Fn6::F113, v0, v1)
}

pub fn f78(v0: Adt28, v1: Adt27, p591: Adt28, v3: List<Adt12>, p592: Adt17, v5: bool) -> R<Result<Adt28, Bytes>> {
    let m590 = (p591, p592);
    {
        let mut m590 = m590;
        loop {
            let step = {
                let (v2, v4) = m590;
                let m574 = v4;
                match m574 {
                    Adt17::C0(_, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v2)))
                    }
                    Adt17::C1(_, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v2)))
                    }
                    Adt17::C2(_, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v2)))
                    }
                    Adt17::C3(_, _, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v2)))
                    }
                    Adt17::C4(_, _, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v2)))
                    }
                    Adt17::C5(_, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v2)))
                    }
                    Adt17::C6(_, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v2)))
                    }
                    Adt17::C7(_, _, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v2)))
                    }
                    Adt17::C8(_, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v2)))
                    }
                    Adt17::C9(_) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v2)))
                    }
                    Adt17::C10(_, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v2)))
                    }
                    Adt17::C11(_, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v2)))
                    }
                    Adt17::C12 => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v2)))
                    }
                    Adt17::C13(_, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v2)))
                    }
                    Adt17::C14(_, _, _, _, _, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v2)))
                    }
                    Adt17::C15(v49, r588, r589) => {
                        let v50 = (*r588).clone();
                        let v51 = (*r589).clone();
                        let v52: Adt28 = Adt28::C0(match v2.clone() {
                            Adt28::C0(h575, _, _, _, _) => {
                                h575
                            }
                        }, v3.clone(), match v0.clone() {
                            Adt28::C0(_, _, h576, _, _) => {
                                h576
                            }
                        }, match v0.clone() {
                            Adt28::C0(_, _, _, h577, _) => {
                                h577
                            }
                        }, match v2.clone() {
                            Adt28::C0(_, _, _, _, h578) => {
                                h578
                            }
                        });
                        let m580 = f45(v52, match v1.clone() {
                            Adt27::C0(_, h579, _, _) => {
                                h579
                            }
                        }, v49)?;
                        match m580 {
                            Ok(v53) => {
                                let m581 = f29(v53, v1.clone(), v50, v5)?;
                                match m581 {
                                    Ok(v54) => {
                                        let v55: Adt28 = Adt28::C0(match v54.clone() {
                                            Adt28::C0(h582, _, _, _, _) => {
                                                h582
                                            }
                                        }, v3.clone(), f76(match v2.clone() {
                                            Adt28::C0(_, _, h583, _, _) => {
                                                h583
                                            }
                                        }, match v54.clone() {
                                            Adt28::C0(_, _, h584, _, _) => {
                                                h584
                                            }
                                        }), f77(match v2 {
                                            Adt28::C0(_, _, _, h585, _) => {
                                                h585
                                            }
                                        }, match v54.clone() {
                                            Adt28::C0(_, _, _, h586, _) => {
                                                h586
                                            }
                                        }), match v54 {
                                            Adt28::C0(_, _, _, _, h587) => {
                                                h587
                                            }
                                        });
                                        Ok::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>((v55, v51))
                                    }
                                    Err(v56) => {
                                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Err::<Adt28, Bytes>(v56)))
                                    }
                                }
                            }
                            Err(v57) => {
                                Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Err::<Adt28, Bytes>(v57)))
                            }
                        }
                    }
                    Adt17::C16(_, _) => {
                        Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Ok::<Adt28, Bytes>(v2)))
                    }
                }
            };
            match step {
                Ok(next) => { m590 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f79(v0: List<Adt25>, v1: Adt17) -> R<Result<(), Bytes>> {
    let m593 = v1;
    match m593 {
        Adt17::C0(_, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C1(_, _, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C2(_, _, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C3(_, _, _, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C4(_, _, _, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C5(_, _, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C6(_, _, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C7(_, _, _, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C8(_, _, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C9(_) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C10(_, _, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C11(_, r594) => {
            let v36 = (*r594).clone();
            f114(v0, v36)
        }
        Adt17::C12 => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C13(_, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C14(_, _, _, _, _, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C15(_, _, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C16(_, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
    }
}

pub fn f80(v0: List<Adt12>, v1: Adt12) -> List<Adt12> {
    f110(Fn5::F115(v1), List::<Adt12>::nil(), v0)
}

pub fn f81(v0: List<Adt12>, v1: Adt12, v2: List<Adt12>, v3: List<Adt12>) -> Option<Adt12> {
    f116(Fn7::F117(v2, v3, v1), None::<Adt12>, v0)
}

pub fn f82(v0: List<(Adt12, List<u64>)>, v1: List<(Adt12, List<u64>)>, v2: List<Adt12>) -> Option<Adt12> {
    f118(Fn8::F119(v1, v2), None::<Adt12>, v0)
}

pub fn f83(p600: List<(Adt12, u64)>, v1: Adt26, p601: Adt17) -> R<Result<List<(Adt12, u64)>, Bytes>> {
    let m599 = (p600, p601);
    {
        let mut m599 = m599;
        loop {
            let step = {
                let (v0, v2) = m599;
                let m595 = v2;
                match m595 {
                    Adt17::C0(_, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C1(_, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C2(_, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C3(_, _, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C4(_, _, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C5(_, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C6(_, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C7(_, _, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C8(_, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C9(_) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C10(_, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C11(_, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C12 => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C13(r597, r598) => {
                        let v38 = (*r597).clone();
                        let v39 = (*r598).clone();
                        let m596 = f49(v0, v1.clone(), v38, false)?;
                        match m596 {
                            Ok(v40) => {
                                Ok::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>((v40, v39))
                            }
                            Err(v41) => {
                                Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Err::<List<(Adt12, u64)>, Bytes>(v41)))
                            }
                        }
                    }
                    Adt17::C14(_, _, _, _, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C15(_, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C16(_, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                }
            };
            match step {
                Ok(next) => { m599 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f84(v0: List<(Adt12, u64)>, v1: Adt12) -> Option<u64> {
    f120(Fn9::F121(v1), None::<u64>, v0)
}

pub fn f85(p607: List<(Adt12, u64)>, v1: Adt26, p608: Adt17, v3: bool) -> R<Result<List<(Adt12, u64)>, Bytes>> {
    let m606 = (p607, p608);
    {
        let mut m606 = m606;
        loop {
            let step = {
                let (v0, v2) = m606;
                let m602 = v2;
                match m602 {
                    Adt17::C0(_, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C1(_, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C2(_, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C3(_, _, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C4(_, _, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C5(_, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C6(_, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C7(_, _, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C8(_, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C9(_) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C10(_, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C11(_, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C12 => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C13(_, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C14(_, _, _, _, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C15(_, r604, r605) => {
                        let v48 = (*r604).clone();
                        let v49 = (*r605).clone();
                        let m603 = f32(v0, v1.clone(), v48, v3)?;
                        match m603 {
                            Ok(v50) => {
                                Ok::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>((v50, v49))
                            }
                            Err(v51) => {
                                Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Err::<List<(Adt12, u64)>, Bytes>(v51)))
                            }
                        }
                    }
                    Adt17::C16(_, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                }
            };
            match step {
                Ok(next) => { m606 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f86(v0: Adt14) -> Option<Adt12> {
    let m609 = v0;
    match m609 {
        Adt14::C0(_) => {
            None::<Adt12>
        }
        Adt14::C1(v2) => {
            Some(v2)
        }
        Adt14::C2(_) => {
            None::<Adt12>
        }
        Adt14::C3(_, _) => {
            None::<Adt12>
        }
        Adt14::C4(_, _, _) => {
            None::<Adt12>
        }
        Adt14::C5 => {
            None::<Adt12>
        }
        Adt14::C6(_, _) => {
            None::<Adt12>
        }
    }
}

pub fn f87(v0: Adt2) -> Option<u64> {
    let m610 = v0;
    match m610 {
        Adt2::C0(_) => {
            None::<u64>
        }
        Adt2::C1(_) => {
            None::<u64>
        }
        Adt2::C2(_, _) => {
            None::<u64>
        }
        Adt2::C3(_, _, _) => {
            None::<u64>
        }
        Adt2::C4(v8, v9) => {
            let m611 = f6(v8);
            if m611 {
                Some(v9)
            } else {
                None::<u64>
            }
        }
        Adt2::C5 => {
            None::<u64>
        }
        Adt2::C6(_, _) => {
            None::<u64>
        }
    }
}

pub fn f88(v0: Bytes, v1: u64, v2: u8) -> bool {
    let m612 = f122(v0, v1);
    match m612 {
        None => {
            false
        }
        Some(v3) => {
            let a613 = v3;
            let a614 = v2;
            equal(a613, a614)
        }
    }
}

pub fn f89(v0: Fn4, p618: bool, p619: List<Adt12>) -> bool {
    let m617 = (p618, p619);
    {
        let mut m617 = m617;
        loop {
            let step = {
                let (v1, v2) = m617;
                let m615 = v2;
                match m615.uncons() {
                    None => {
                        Err::<(bool, List<Adt12>), bool>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<Adt12>), bool>(({
                            let c616: Fn4 = v0.clone();
                            c616.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m617 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f90(v0: Adt12, v1: bool, v2: Adt12) -> bool {
    let v3: bool = f123(v2, v0);
    if v1 {
        true
    } else {
        v3
    }
}

pub fn f91(v0: List<(u64, u64)>, v1: u64) -> Option<u64> {
    f124(Fn10::F125(v1), None::<u64>, v0)
}

pub fn f92(v0: Adt19) -> bool {
    let m620 = v0;
    match m620 {
        Adt19::C0 => {
            true
        }
        Adt19::C1 => {
            false
        }
        Adt19::C2 => {
            false
        }
        Adt19::C3 => {
            false
        }
        Adt19::C4 => {
            false
        }
    }
}

pub fn f93(v0: Adt14) -> bool {
    let m621 = v0;
    match m621 {
        Adt14::C0(_) => {
            false
        }
        Adt14::C1(_) => {
            false
        }
        Adt14::C2(_) => {
            false
        }
        Adt14::C3(_, _) => {
            false
        }
        Adt14::C4(_, _, _) => {
            false
        }
        Adt14::C5 => {
            false
        }
        Adt14::C6(r622, r623) => {
            let v9 = (*r622).clone();
            let v10 = (*r623).clone();
            let v11: bool = f126(v9);
            let v12: bool = f93(v10);
            if v11 {
                true
            } else {
                v12
            }
        }
    }
}

pub fn f94(v0: List<(Adt12, List<u64>)>, v1: Adt12) -> Option<List<u64>> {
    f127(Fn11::F128(v1), None::<List<u64>>, v0)
}

pub fn f95(p634: List<u64>, p635: u64, p636: Adt14, v3: Bytes) -> R<Result<List<u64>, Bytes>> {
    let m633 = (p634, (p635, p636));
    {
        let mut m633 = m633;
        loop {
            let step = {
                let (v0, (v1, v2)) = m633;
                let m624 = v2;
                match m624 {
                    Adt14::C0(_) => {
                        Err::<(List<u64>, (u64, Adt14)), R<Result<List<u64>, Bytes>>>(Ok(Ok::<List<u64>, Bytes>(v0)))
                    }
                    Adt14::C1(_) => {
                        Err::<(List<u64>, (u64, Adt14)), R<Result<List<u64>, Bytes>>>(Ok(Ok::<List<u64>, Bytes>(v0)))
                    }
                    Adt14::C2(_) => {
                        Err::<(List<u64>, (u64, Adt14)), R<Result<List<u64>, Bytes>>>(Ok(Ok::<List<u64>, Bytes>(v0)))
                    }
                    Adt14::C3(_, _) => {
                        Err::<(List<u64>, (u64, Adt14)), R<Result<List<u64>, Bytes>>>(Ok(Ok::<List<u64>, Bytes>(v0)))
                    }
                    Adt14::C4(_, _, _) => {
                        Err::<(List<u64>, (u64, Adt14)), R<Result<List<u64>, Bytes>>>(Ok(Ok::<List<u64>, Bytes>(v0)))
                    }
                    Adt14::C5 => {
                        Err::<(List<u64>, (u64, Adt14)), R<Result<List<u64>, Bytes>>>(Ok(Ok::<List<u64>, Bytes>(v0)))
                    }
                    Adt14::C6(r631, r632) => {
                        let v12 = (*r631).clone();
                        let v13 = (*r632).clone();
                        let v14: bool = f126(v12);
                        let m625 = v14;
                        if m625 {
                            Ok::<(List<u64>, (u64, Adt14)), R<Result<List<u64>, Bytes>>>((v0, ({
                                let a626 = v1;
                                let a627 = 1u64;
                                nat_add(a626, a627)?
                            }, v13)))
                        } else {
                            let m628 = f129(v0.clone(), v1);
                            if m628 {
                                Err::<(List<u64>, (u64, Adt14)), R<Result<List<u64>, Bytes>>>(Ok(Err::<List<u64>, Bytes>(f4(f1(Str::lit("ownership: component ")), f4(f10(v1)?, f4(f1(Str::lit(" of ")), f4(v3.clone(), f1(Str::lit(" is moved twice")))))))))
                            } else {
                                Ok::<(List<u64>, (u64, Adt14)), R<Result<List<u64>, Bytes>>>((List::cons(v1, v0), ({
                                    let a629 = v1;
                                    let a630 = 1u64;
                                    nat_add(a629, a630)?
                                }, v13)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m633 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f96(v0: List<(Adt12, List<u64>)>, v1: Adt12, v2: List<u64>) -> List<(Adt12, List<u64>)> {
    List::cons((v1.clone(), v2), f130(v0, v1))
}

pub fn f97(v0: List<Adt29>) -> Bytes {
    let m637 = f131(Fn12::F132, None::<Bytes>, v0);
    match m637 {
        None => {
            f1(Str::lit("a read"))
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f98(v0: Adt30, v1: Adt17) -> R<Adt30> {
    let m638 = v1;
    match m638 {
        Adt17::C0(v2, v3) => {
            let m639 = f133(v2.clone());
            if m639 {
                Ok(v0)
            } else {
                Ok(f134(v0, f135(v2.clone()), v3, f136(v2)))
            }
        }
        Adt17::C1(v4, _, v6) => {
            Ok(f137(v0, f138(v4), v6))
        }
        Adt17::C2(v7, r648, v9) => {
            let v8 = (*r648).clone();
            let v10: Adt30 = f137(v0, f139(v7), v9);
            f98(v10, v8)
        }
        Adt17::C3(v11, r649, _, v14) => {
            let v12 = (*r649).clone();
            let v18: Adt30 = {
                let m640 = v11;
                match m640 {
                    Adt21::C0(_) => {
                        f137(v0, f1(Str::lit("call:function")), v14)
                    }
                    Adt21::C1(v17) => {
                        f134(v0.clone(), f4(f1(Str::lit("call:runtime:")), f140(f141(f69(match v0.clone() {
                            Adt30::C0(_, _, h641) => {
                                h641
                            }
                        }, v17.clone()))?)), v14, f142(match v0 {
                            Adt30::C0(_, _, h642) => {
                                h642
                            }
                        }, v17))
                    }
                    Adt21::C2(_) => {
                        f137(v0, f1(Str::lit("call:machine")), v14)
                    }
                }
            };
            f98(v18, v12)
        }
        Adt17::C4(_, r650, _, v22) => {
            let v20 = (*r650).clone();
            let v23: Adt30 = f137(v0, f1(Str::lit("apply")), v22);
            f98(v23, v20)
        }
        Adt17::C5(v24, r651, v26) => {
            let v25 = (*r651).clone();
            let v27: Adt30 = f137(f143(v0, v24.clone()), f4(f1(Str::lit("construct:")), f144(v24)), v26);
            f98(v27, v25)
        }
        Adt17::C6(r652, r653, v30) => {
            let v28 = (*r652).clone();
            let v29 = (*r653).clone();
            let v31: Adt30 = f137(v0, f1(Str::lit("construct:pair")), v30);
            let v32: Adt30 = f98(v31, v28)?;
            f98(v32, v29)
        }
        Adt17::C7(r654, r655, r656, v36) => {
            let v33 = (*r654).clone();
            let v34 = (*r655).clone();
            let v35 = (*r656).clone();
            let v37: Adt30 = f137(v0, f1(Str::lit("if")), v36);
            let v38: Adt30 = f98(v37, v33)?;
            let v39: Adt30 = f98(v38, v34)?;
            f98(v39, v35)
        }
        Adt17::C8(r657, r658, v42) => {
            let v40 = (*r657).clone();
            let v41 = (*r658).clone();
            let v43: Adt30 = {
                let m643 = f145(v41.clone());
                if m643 {
                    f137(v0, f1(Str::lit("match:empty")), v42)
                } else {
                    let m644 = f146(v40.clone());
                    if m644 {
                        f137(v0, f1(Str::lit("match:list")), v42)
                    } else {
                        f147(v0, v41.clone(), v42)
                    }
                }
            };
            let v44: Adt30 = f98(v43, v40)?;
            f98(v44, v41)
        }
        Adt17::C9(r659) => {
            let v45 = (*r659).clone();
            f98(v0, v45)
        }
        Adt17::C10(v46, _, v48) => {
            let m645 = f148(v46);
            if m645 {
                Ok(f137(v0, f1(Str::lit("match:nat")), v48))
            } else {
                Ok(v0)
            }
        }
        Adt17::C11(r660, r661) => {
            let v49 = (*r660).clone();
            let v50 = (*r661).clone();
            let v51: Adt30 = f98(v0, v49)?;
            f98(v51, v50)
        }
        Adt17::C12 => {
            Ok(v0)
        }
        Adt17::C13(r662, r663) => {
            let v52 = (*r662).clone();
            let v53 = (*r663).clone();
            let v54: Adt30 = f98(v0, v52)?;
            f98(v54, v53)
        }
        Adt17::C14(v55, v56, v57, r664, v59, r665) => {
            let v58 = (*r664).clone();
            let v60 = (*r665).clone();
            let v61: Bytes = {
                let m646 = f149(v55);
                if m646 {
                    f1(Str::lit("destructure:pair"))
                } else {
                    f1(Str::lit("let"))
                }
            };
            let v62: Adt30 = {
                let m647 = v56;
                if m647 {
                    f150(v0, v57)
                } else {
                    v0
                }
            };
            let v63: Adt30 = f137(v62, v61, v59);
            let v64: Adt30 = f98(v63, v58)?;
            f98(v64, v60)
        }
        Adt17::C15(_, r666, r667) => {
            let v66 = (*r666).clone();
            let v67 = (*r667).clone();
            let v68: Adt30 = f98(v0, v66)?;
            f98(v68, v67)
        }
        Adt17::C16(_, r668) => {
            let v70 = (*r668).clone();
            f98(v0, v70)
        }
    }
}

pub fn f99(v0: List<(Adt12, List<u64>)>, v1: Adt12) -> bool {
    let m669 = f94(v0, v1);
    match m669 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f100(v0: Adt24) -> bool {
    let m670 = v0;
    match m670 {
        Adt24::C0 => {
            true
        }
        Adt24::C1 => {
            true
        }
        Adt24::C2 => {
            false
        }
        Adt24::C3 => {
            true
        }
    }
}

pub fn f101(v0: Adt17) -> bool {
    let m671 = v0;
    match m671 {
        Adt17::C0(v1, _) => {
            f133(v1)
        }
        Adt17::C1(_, _, _) => {
            false
        }
        Adt17::C2(_, _, _) => {
            false
        }
        Adt17::C3(_, _, _, _) => {
            false
        }
        Adt17::C4(_, _, _, _) => {
            false
        }
        Adt17::C5(_, _, _) => {
            false
        }
        Adt17::C6(_, _, _) => {
            false
        }
        Adt17::C7(_, _, _, _) => {
            false
        }
        Adt17::C8(_, _, _) => {
            false
        }
        Adt17::C9(_) => {
            false
        }
        Adt17::C10(_, _, _) => {
            false
        }
        Adt17::C11(_, _) => {
            false
        }
        Adt17::C12 => {
            false
        }
        Adt17::C13(_, _) => {
            false
        }
        Adt17::C14(_, _, _, _, _, _) => {
            false
        }
        Adt17::C15(_, _, _) => {
            false
        }
        Adt17::C16(_, _) => {
            false
        }
    }
}

pub fn f102(v0: u64) -> u64 {
    let a672 = v0;
    let a673 = 0u64;
    if nat_eq(a672, a673) {
        2u64
    } else {
        let a674 = v0;
        let a675 = 1u64;
        if nat_eq(a674, a675) {
            2u64
        } else {
            let a676 = v0;
            let a677 = 2u64;
            if nat_eq(a676, a677) {
                2u64
            } else {
                let a678 = v0;
                let a679 = 3u64;
                if nat_eq(a678, a679) {
                    2u64
                } else {
                    let a680 = v0;
                    let a681 = 4u64;
                    if nat_eq(a680, a681) {
                        1u64
                    } else {
                        let a682 = v0;
                        let a683 = 5u64;
                        if nat_eq(a682, a683) {
                            2u64
                        } else {
                            let a684 = v0;
                            let a685 = 6u64;
                            if nat_eq(a684, a685) {
                                2u64
                            } else {
                                let a686 = v0;
                                let a687 = 7u64;
                                if nat_eq(a686, a687) {
                                    1u64
                                } else {
                                    let a688 = v0;
                                    let a689 = 8u64;
                                    if nat_eq(a688, a689) {
                                        2u64
                                    } else {
                                        let a690 = v0;
                                        let a691 = 9u64;
                                        if nat_eq(a690, a691) {
                                            2u64
                                        } else {
                                            let a692 = v0;
                                            let a693 = 10u64;
                                            if nat_eq(a692, a693) {
                                                2u64
                                            } else {
                                                let a694 = v0;
                                                let a695 = 11u64;
                                                if nat_eq(a694, a695) {
                                                    1u64
                                                } else {
                                                    let a696 = v0;
                                                    let a697 = 12u64;
                                                    if nat_eq(a696, a697) {
                                                        2u64
                                                    } else {
                                                        let a698 = v0;
                                                        let a699 = 13u64;
                                                        if nat_eq(a698, a699) {
                                                            2u64
                                                        } else {
                                                            let a700 = v0;
                                                            let a701 = 14u64;
                                                            if nat_eq(a700, a701) {
                                                                2u64
                                                            } else {
                                                                let a702 = v0;
                                                                let a703 = 15u64;
                                                                if nat_eq(a702, a703) {
                                                                    3u64
                                                                } else {
                                                                    let a704 = v0;
                                                                    let a705 = 16u64;
                                                                    if nat_eq(a704, a705) {
                                                                        3u64
                                                                    } else {
                                                                        let a706 = v0;
                                                                        let a707 = 17u64;
                                                                        if nat_eq(a706, a707) {
                                                                            2u64
                                                                        } else {
                                                                            let a708 = v0;
                                                                            let a709 = 18u64;
                                                                            if nat_eq(a708, a709) {
                                                                                2u64
                                                                            } else {
                                                                                let a710 = v0;
                                                                                let a711 = 19u64;
                                                                                if nat_eq(a710, a711) {
                                                                                    2u64
                                                                                } else {
                                                                                    let a712 = v0;
                                                                                    let a713 = 20u64;
                                                                                    if nat_eq(a712, a713) {
                                                                                        3u64
                                                                                    } else {
                                                                                        let a714 = v0;
                                                                                        let a715 = 21u64;
                                                                                        if nat_eq(a714, a715) {
                                                                                            2u64
                                                                                        } else {
                                                                                            let a716 = v0;
                                                                                            let a717 = 22u64;
                                                                                            if nat_eq(a716, a717) {
                                                                                                2u64
                                                                                            } else {
                                                                                                let a718 = v0;
                                                                                                let a719 = 23u64;
                                                                                                if nat_eq(a718, a719) {
                                                                                                    2u64
                                                                                                } else {
                                                                                                    let a720 = v0;
                                                                                                    let a721 = 24u64;
                                                                                                    if nat_eq(a720, a721) {
                                                                                                        1u64
                                                                                                    } else {
                                                                                                        let a722 = v0;
                                                                                                        let a723 = 25u64;
                                                                                                        if nat_eq(a722, a723) {
                                                                                                            3u64
                                                                                                        } else {
                                                                                                            0u64
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
            }
        }
    }
}

pub fn f103(v0: u64) -> Bytes {
    let a724 = v0;
    let a725 = 0u64;
    if nat_eq(a724, a725) {
        f1(Str::lit("checked_add"))
    } else {
        let a726 = v0;
        let a727 = 1u64;
        if nat_eq(a726, a727) {
            f1(Str::lit("checked_sub"))
        } else {
            let a728 = v0;
            let a729 = 2u64;
            if nat_eq(a728, a729) {
                f1(Str::lit("checked_mul"))
            } else {
                let a730 = v0;
                let a731 = 3u64;
                if nat_eq(a730, a731) {
                    f1(Str::lit("checked_div"))
                } else {
                    let a732 = v0;
                    let a733 = 4u64;
                    if nat_eq(a732, a733) {
                        f1(Str::lit("checked_neg"))
                    } else {
                        let a734 = v0;
                        let a735 = 5u64;
                        if nat_eq(a734, a735) {
                            f1(Str::lit("checked_shl"))
                        } else {
                            let a736 = v0;
                            let a737 = 6u64;
                            if nat_eq(a736, a737) {
                                f1(Str::lit("checked_shr"))
                            } else {
                                let a738 = v0;
                                let a739 = 7u64;
                                if nat_eq(a738, a739) {
                                    f1(Str::lit("convert"))
                                } else {
                                    let a740 = v0;
                                    let a741 = 8u64;
                                    if nat_eq(a740, a741) {
                                        f1(Str::lit("bit_and"))
                                    } else {
                                        let a742 = v0;
                                        let a743 = 9u64;
                                        if nat_eq(a742, a743) {
                                            f1(Str::lit("bit_or"))
                                        } else {
                                            let a744 = v0;
                                            let a745 = 10u64;
                                            if nat_eq(a744, a745) {
                                                f1(Str::lit("bit_xor"))
                                            } else {
                                                let a746 = v0;
                                                let a747 = 11u64;
                                                if nat_eq(a746, a747) {
                                                    f1(Str::lit("bit_not"))
                                                } else {
                                                    let a748 = v0;
                                                    let a749 = 12u64;
                                                    if nat_eq(a748, a749) {
                                                        f1(Str::lit("nat_eq"))
                                                    } else {
                                                        let a750 = v0;
                                                        let a751 = 13u64;
                                                        if nat_eq(a750, a751) {
                                                            f1(Str::lit("nat_le"))
                                                        } else {
                                                            let a752 = v0;
                                                            let a753 = 14u64;
                                                            if nat_eq(a752, a753) {
                                                                f1(Str::lit("nat_lt"))
                                                            } else {
                                                                let a754 = v0;
                                                                let a755 = 15u64;
                                                                if nat_eq(a754, a755) {
                                                                    f1(Str::lit("nat_rem"))
                                                                } else {
                                                                    let a756 = v0;
                                                                    let a757 = 16u64;
                                                                    if nat_eq(a756, a757) {
                                                                        f1(Str::lit("int_rem"))
                                                                    } else {
                                                                        let a758 = v0;
                                                                        let a759 = 17u64;
                                                                        if nat_eq(a758, a759) {
                                                                            f1(Str::lit("nat_add"))
                                                                        } else {
                                                                            let a760 = v0;
                                                                            let a761 = 18u64;
                                                                            if nat_eq(a760, a761) {
                                                                                f1(Str::lit("nat_sub"))
                                                                            } else {
                                                                                let a762 = v0;
                                                                                let a763 = 19u64;
                                                                                if nat_eq(a762, a763) {
                                                                                    f1(Str::lit("nat_mul"))
                                                                                } else {
                                                                                    let a764 = v0;
                                                                                    let a765 = 20u64;
                                                                                    if nat_eq(a764, a765) {
                                                                                        f1(Str::lit("nat_quot"))
                                                                                    } else {
                                                                                        let a766 = v0;
                                                                                        let a767 = 21u64;
                                                                                        if nat_eq(a766, a767) {
                                                                                            f1(Str::lit("int_add"))
                                                                                        } else {
                                                                                            let a768 = v0;
                                                                                            let a769 = 22u64;
                                                                                            if nat_eq(a768, a769) {
                                                                                                f1(Str::lit("int_sub"))
                                                                                            } else {
                                                                                                let a770 = v0;
                                                                                                let a771 = 23u64;
                                                                                                if nat_eq(a770, a771) {
                                                                                                    f1(Str::lit("int_mul"))
                                                                                                } else {
                                                                                                    let a772 = v0;
                                                                                                    let a773 = 24u64;
                                                                                                    if nat_eq(a772, a773) {
                                                                                                        f1(Str::lit("int_neg"))
                                                                                                    } else {
                                                                                                        let a774 = v0;
                                                                                                        let a775 = 25u64;
                                                                                                        if nat_eq(a774, a775) {
                                                                                                            f1(Str::lit("int_quot"))
                                                                                                        } else {
                                                                                                            Bytes::lit(&[])
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
            }
        }
    }
}

pub fn f104(v0: List<Adt25>, v1: u64) -> Option<Adt25> {
    f151(Fn13::F152(v1), None::<Adt25>, v0)
}

pub fn f105(v0: Adt4) -> Bytes {
    let m776 = v0;
    match m776 {
        Adt4::C0 => {
            f1(Str::lit("u8"))
        }
        Adt4::C1 => {
            f1(Str::lit("u16"))
        }
        Adt4::C2 => {
            f1(Str::lit("u32"))
        }
        Adt4::C3 => {
            f1(Str::lit("u64"))
        }
        Adt4::C4 => {
            f1(Str::lit("i8"))
        }
        Adt4::C5 => {
            f1(Str::lit("i16"))
        }
        Adt4::C6 => {
            f1(Str::lit("i32"))
        }
        Adt4::C7 => {
            f1(Str::lit("i64"))
        }
    }
}

pub fn f106(v0: Option<Adt4>) -> Adt4 {
    let m777 = v0;
    match m777 {
        None => {
            Adt4::C0
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f107(v0: Adt2) -> bool {
    let m778 = v0;
    match m778 {
        Adt2::C0(v1) => {
            let m779 = v1;
            match m779 {
                Adt3::C0 => {
                    false
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
                    true
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

pub fn f108(v0: Adt17) -> R<bool> {
    let m780 = v0;
    match m780 {
        Adt17::C0(_, _) => {
            Ok(false)
        }
        Adt17::C1(_, _, _) => {
            Ok(false)
        }
        Adt17::C2(_, _, _) => {
            Ok(false)
        }
        Adt17::C3(_, _, _, _) => {
            Ok(false)
        }
        Adt17::C4(_, _, _, _) => {
            Ok(false)
        }
        Adt17::C5(_, _, _) => {
            Ok(false)
        }
        Adt17::C6(_, _, _) => {
            Ok(false)
        }
        Adt17::C7(_, _, _, _) => {
            Ok(false)
        }
        Adt17::C8(_, _, _) => {
            Ok(false)
        }
        Adt17::C9(_) => {
            Ok(false)
        }
        Adt17::C10(_, _, _) => {
            Ok(false)
        }
        Adt17::C11(_, _) => {
            Ok(false)
        }
        Adt17::C12 => {
            Ok(false)
        }
        Adt17::C13(r783, r784) => {
            let v36 = (*r783).clone();
            let v37 = (*r784).clone();
            let v38: bool = {
                let a781 = f75(v37)?;
                let a782 = 0u64;
                nat_eq(a781, a782)
            };
            let v39: bool = f101(v36);
            if v38 {
                Ok(v39)
            } else {
                Ok(false)
            }
        }
        Adt17::C14(_, _, _, _, _, _) => {
            Ok(false)
        }
        Adt17::C15(_, _, _) => {
            Ok(false)
        }
        Adt17::C16(_, _) => {
            Ok(false)
        }
    }
}

pub fn f109(v0: Adt26, v1: u64, v2: u64) -> Option<u64> {
    let m786 = f18(match v0 {
        Adt26::C0(_, h785, _, _) => {
            h785
        }
    }, v1);
    match m786 {
        None => {
            None::<u64>
        }
        Some(v3) => {
            f91(v3, v2)
        }
    }
}

pub fn f110(v0: Fn5, p790: List<Adt12>, p791: List<Adt12>) -> List<Adt12> {
    let m789 = (p790, p791);
    {
        let mut m789 = m789;
        loop {
            let step = {
                let (v1, v2) = m789;
                let m787 = v2;
                match m787.uncons() {
                    None => {
                        Err::<(List<Adt12>, List<Adt12>), List<Adt12>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt12>, List<Adt12>), List<Adt12>>(({
                            let c788: Fn5 = v0.clone();
                            c788.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m789 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f111(v0: List<Adt12>, v1: Adt12) -> List<Adt12> {
    let v2: bool = f56(v0.clone(), v1.clone());
    if v2 {
        v0
    } else {
        List::cons(v1, v0)
    }
}

pub fn f112(v0: Fn6, p795: List<(Adt12, List<u64>)>, p796: List<(Adt12, List<u64>)>) -> List<(Adt12, List<u64>)> {
    let m794 = (p795, p796);
    {
        let mut m794 = m794;
        loop {
            let step = {
                let (v1, v2) = m794;
                let m792 = v2;
                match m792.uncons() {
                    None => {
                        Err::<(List<(Adt12, List<u64>)>, List<(Adt12, List<u64>)>), List<(Adt12, List<u64>)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Adt12, List<u64>)>, List<(Adt12, List<u64>)>), List<(Adt12, List<u64>)>>(({
                            let c793: Fn6 = v0.clone();
                            c793.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m794 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f113(v0: List<(Adt12, List<u64>)>, v1: (Adt12, List<u64>)) -> List<(Adt12, List<u64>)> {
    let m798 = f94(v0.clone(), {
        let (h797, _) = v1.clone();
        h797
    });
    match m798 {
        None => {
            f96(v0, {
                let (h801, _) = v1;
                h801
            }, {
                let (_, h802) = v1;
                h802
            })
        }
        Some(v2) => {
            f96(v0, {
                let (h799, _) = v1;
                h799
            }, f153(v2, {
                let (_, h800) = v1;
                h800
            }))
        }
    }
}

pub fn f114(v0: List<Adt25>, v1: Adt17) -> R<Result<(), Bytes>> {
    let m803 = v1;
    match m803 {
        Adt17::C0(_, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C1(_, _, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C2(_, _, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C3(_, _, _, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C4(_, _, _, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C5(v18, r806, _) => {
            let v19 = (*r806).clone();
            let m804 = f73(v18);
            if m804 {
                f154(v0, v19)
            } else {
                Ok(Ok::<(), Bytes>(()))
            }
        }
        Adt17::C6(_, _, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C7(_, r807, r808, _) => {
            let v25 = (*r807).clone();
            let v26 = (*r808).clone();
            let m805 = f79(v0.clone(), v25)?;
            match m805 {
                Ok(_) => {
                    f79(v0, v26)
                }
                Err(v29) => {
                    Ok(Err::<(), Bytes>(v29))
                }
            }
        }
        Adt17::C8(_, r809, _) => {
            let v31 = (*r809).clone();
            f155(v0, v31)
        }
        Adt17::C9(r810) => {
            let v33 = (*r810).clone();
            f79(v0, v33)
        }
        Adt17::C10(_, _, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C11(_, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C12 => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C13(_, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C14(_, _, _, _, _, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C15(_, _, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C16(_, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
    }
}

pub fn f115(v0: Adt12, v1: List<Adt12>, v2: Adt12) -> List<Adt12> {
    let v3: bool = f123(v2.clone(), v0);
    if v3 {
        v1
    } else {
        List::cons(v2, v1)
    }
}

pub fn f116(v0: Fn7, p814: Option<Adt12>, p815: List<Adt12>) -> Option<Adt12> {
    let m813 = (p814, p815);
    {
        let mut m813 = m813;
        loop {
            let step = {
                let (v1, v2) = m813;
                let m811 = v2;
                match m811.uncons() {
                    None => {
                        Err::<(Option<Adt12>, List<Adt12>), Option<Adt12>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Adt12>, List<Adt12>), Option<Adt12>>(({
                            let c812: Fn7 = v0.clone();
                            c812.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m813 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f117(v0: List<Adt12>, v1: List<Adt12>, v2: Adt12, v3: Option<Adt12>, v4: Adt12) -> Option<Adt12> {
    let v5: bool = f123(v4.clone(), v2);
    let v6: bool = f56(v0, v4.clone());
    let v7: bool = f56(v1, v4.clone());
    let v8: bool = if v5 {
        false
    } else {
        if v6 {
            false
        } else {
            v7
        }
    };
    if v8 {
        f156(v3, v4)
    } else {
        v3
    }
}

pub fn f118(v0: Fn8, p819: Option<Adt12>, p820: List<(Adt12, List<u64>)>) -> Option<Adt12> {
    let m818 = (p819, p820);
    {
        let mut m818 = m818;
        loop {
            let step = {
                let (v1, v2) = m818;
                let m816 = v2;
                match m816.uncons() {
                    None => {
                        Err::<(Option<Adt12>, List<(Adt12, List<u64>)>), Option<Adt12>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Adt12>, List<(Adt12, List<u64>)>), Option<Adt12>>(({
                            let c817: Fn8 = v0.clone();
                            c817.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m818 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f119(v0: List<(Adt12, List<u64>)>, v1: List<Adt12>, v2: Option<Adt12>, v3: (Adt12, List<u64>)) -> Option<Adt12> {
    let v4: bool = f56(v1, {
        let (h821, _) = v3.clone();
        h821
    });
    let v5: bool = f157(v0, {
        let (h822, _) = v3.clone();
        h822
    }, {
        let (_, h823) = v3;
        h823
    });
    let v6: bool = if v4 {
        !v5
    } else {
        false
    };
    if v6 {
        f156(v2, {
            let (h824, _) = v3;
            h824
        })
    } else {
        v2
    }
}

pub fn f120(v0: Fn9, p828: Option<u64>, p829: List<(Adt12, u64)>) -> Option<u64> {
    let m827 = (p828, p829);
    {
        let mut m827 = m827;
        loop {
            let step = {
                let (v1, v2) = m827;
                let m825 = v2;
                match m825.uncons() {
                    None => {
                        Err::<(Option<u64>, List<(Adt12, u64)>), Option<u64>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<u64>, List<(Adt12, u64)>), Option<u64>>(({
                            let c826: Fn9 = v0.clone();
                            c826.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m827 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f121(v0: Adt12, v1: Option<u64>, v2: (Adt12, u64)) -> Option<u64> {
    let m830 = v1;
    match m830 {
        None => {
            let v4: bool = f123({
                let (h831, _) = v2;
                h831
            }, v0);
            if v4 {
                Some({
                    let (_, h832) = v2;
                    h832
                })
            } else {
                None::<u64>
            }
        }
        Some(v3) => {
            Some(v3)
        }
    }
}

pub fn f122(v0: Bytes, v1: u64) -> Option<u8> {
    let a833 = v0;
    let a834 = v1;
    index_bytes(a833, a834)
}

pub fn f123(v0: Adt12, v1: Adt12) -> bool {
    let a835 = f158(v0.clone());
    let a836 = f158(v1.clone());
    if nat_eq(a835, a836) {
        let a837 = f159(v0.clone());
        let a838 = f159(v1.clone());
        if nat_eq(a837, a838) {
            let a839 = f160(v0);
            let a840 = f160(v1);
            equal(a839, a840)
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f124(v0: Fn10, p844: Option<u64>, p845: List<(u64, u64)>) -> Option<u64> {
    let m843 = (p844, p845);
    {
        let mut m843 = m843;
        loop {
            let step = {
                let (v1, v2) = m843;
                let m841 = v2;
                match m841.uncons() {
                    None => {
                        Err::<(Option<u64>, List<(u64, u64)>), Option<u64>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<u64>, List<(u64, u64)>), Option<u64>>(({
                            let c842: Fn10 = v0.clone();
                            c842.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m843 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f125(v0: u64, v1: Option<u64>, v2: (u64, u64)) -> Option<u64> {
    let m846 = v1;
    match m846 {
        None => {
            let a848 = {
                let (h847, _) = v2;
                h847
            };
            let a849 = v0;
            if nat_eq(a848, a849) {
                Some({
                    let (_, h850) = v2;
                    h850
                })
            } else {
                None::<u64>
            }
        }
        Some(v3) => {
            Some(v3)
        }
    }
}

pub fn f126(v0: Adt14) -> bool {
    let m851 = v0;
    match m851 {
        Adt14::C0(v1) => {
            let m852 = v1;
            match m852 {
                Adt15::C0 => {
                    true
                }
                Adt15::C1 => {
                    false
                }
                Adt15::C2 => {
                    false
                }
                Adt15::C3 => {
                    false
                }
            }
        }
        Adt14::C1(_) => {
            false
        }
        Adt14::C2(_) => {
            false
        }
        Adt14::C3(_, _) => {
            false
        }
        Adt14::C4(_, _, _) => {
            false
        }
        Adt14::C5 => {
            false
        }
        Adt14::C6(_, _) => {
            false
        }
    }
}

pub fn f127(v0: Fn11, p856: Option<List<u64>>, p857: List<(Adt12, List<u64>)>) -> Option<List<u64>> {
    let m855 = (p856, p857);
    {
        let mut m855 = m855;
        loop {
            let step = {
                let (v1, v2) = m855;
                let m853 = v2;
                match m853.uncons() {
                    None => {
                        Err::<(Option<List<u64>>, List<(Adt12, List<u64>)>), Option<List<u64>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<List<u64>>, List<(Adt12, List<u64>)>), Option<List<u64>>>(({
                            let c854: Fn11 = v0.clone();
                            c854.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m855 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f128(v0: Adt12, v1: Option<List<u64>>, v2: (Adt12, List<u64>)) -> Option<List<u64>> {
    let m858 = v1;
    match m858 {
        None => {
            let v4: bool = f123({
                let (h859, _) = v2;
                h859
            }, v0);
            if v4 {
                Some({
                    let (_, h860) = v2;
                    h860
                })
            } else {
                None::<List<u64>>
            }
        }
        Some(v3) => {
            Some(v3)
        }
    }
}

pub fn f129(v0: List<u64>, v1: u64) -> bool {
    f161(Fn14::F162(v1), false, v0)
}

pub fn f130(v0: List<(Adt12, List<u64>)>, v1: Adt12) -> List<(Adt12, List<u64>)> {
    f112(Fn6::F163(v1), List::<(Adt12, List<u64>)>::nil(), v0)
}

pub fn f131(v0: Fn12, p864: Option<Bytes>, p865: List<Adt29>) -> Option<Bytes> {
    let m863 = (p864, p865);
    {
        let mut m863 = m863;
        loop {
            let step = {
                let (v1, v2) = m863;
                let m861 = v2;
                match m861.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Adt29>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Adt29>), Option<Bytes>>(({
                            let c862: Fn12 = v0.clone();
                            c862.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m863 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f132(v0: Option<Bytes>, v1: Adt29) -> Option<Bytes> {
    let m866 = v0;
    match m866 {
        None => {
            Some(match v1 {
                Adt29::C0(h867, _, _, _) => {
                    h867
                }
            })
        }
        Some(v2) => {
            let m869 = f164(match v1.clone() {
                Adt29::C0(h868, _, _, _) => {
                    h868
                }
            }, v2.clone());
            if m869 {
                Some(match v1 {
                    Adt29::C0(h870, _, _, _) => {
                        h870
                    }
                })
            } else {
                Some(v2)
            }
        }
    }
}

pub fn f133(v0: Adt18) -> bool {
    let m871 = v0;
    match m871 {
        Adt18::C0 => {
            false
        }
        Adt18::C1 => {
            false
        }
        Adt18::C2 => {
            false
        }
        Adt18::C3 => {
            false
        }
        Adt18::C4(_) => {
            false
        }
        Adt18::C5 => {
            false
        }
        Adt18::C6 => {
            false
        }
        Adt18::C7 => {
            false
        }
        Adt18::C8 => {
            true
        }
    }
}

pub fn f134(v0: Adt30, v1: Bytes, v2: Adt9, v3: Option<Adt4>) -> Adt30 {
    let v4: Adt29 = Adt29::C0(v1, match v2.clone() {
        Adt9::C0(h872, _) => {
            h872
        }
    }, match v2 {
        Adt9::C0(_, h873) => {
            h873
        }
    }, v3);
    let m875 = f165(match v0.clone() {
        Adt30::C0(h874, _, _) => {
            h874
        }
    }, v4.clone());
    if m875 {
        v0
    } else {
        Adt30::C0(List::cons(v4, match v0.clone() {
            Adt30::C0(h876, _, _) => {
                h876
            }
        }), match v0.clone() {
            Adt30::C0(_, h877, _) => {
                h877
            }
        }, match v0 {
            Adt30::C0(_, _, h878) => {
                h878
            }
        })
    }
}

pub fn f135(v0: Adt18) -> Bytes {
    let m879 = v0;
    match m879 {
        Adt18::C0 => {
            f1(Str::lit("lit:unit"))
        }
        Adt18::C1 => {
            f1(Str::lit("lit:bool"))
        }
        Adt18::C2 => {
            f1(Str::lit("lit:nat"))
        }
        Adt18::C3 => {
            f1(Str::lit("lit:int"))
        }
        Adt18::C4(_) => {
            f1(Str::lit("lit:fixed"))
        }
        Adt18::C5 => {
            f1(Str::lit("lit:string"))
        }
        Adt18::C6 => {
            f1(Str::lit("lit:bytes"))
        }
        Adt18::C7 => {
            f1(Str::lit("lit:ordering"))
        }
        Adt18::C8 => {
            f1(Str::lit("lit:overflow"))
        }
    }
}

pub fn f136(v0: Adt18) -> Option<Adt4> {
    let m880 = v0;
    match m880 {
        Adt18::C0 => {
            None::<Adt4>
        }
        Adt18::C1 => {
            None::<Adt4>
        }
        Adt18::C2 => {
            None::<Adt4>
        }
        Adt18::C3 => {
            None::<Adt4>
        }
        Adt18::C4(v1) => {
            Some(v1)
        }
        Adt18::C5 => {
            None::<Adt4>
        }
        Adt18::C6 => {
            None::<Adt4>
        }
        Adt18::C7 => {
            None::<Adt4>
        }
        Adt18::C8 => {
            None::<Adt4>
        }
    }
}

pub fn f137(v0: Adt30, v1: Bytes, v2: Adt9) -> Adt30 {
    f134(v0, v1, v2, None::<Adt4>)
}

pub fn f138(v0: Adt19) -> Bytes {
    let m881 = v0;
    match m881 {
        Adt19::C0 => {
            f1(Str::lit("read"))
        }
        Adt19::C1 => {
            f1(Str::lit("read"))
        }
        Adt19::C2 => {
            f1(Str::lit("read"))
        }
        Adt19::C3 => {
            f1(Str::lit("read"))
        }
        Adt19::C4 => {
            f1(Str::lit("box"))
        }
    }
}

pub fn f139(v0: Adt20) -> Bytes {
    let m882 = v0;
    match m882 {
        Adt20::C0 => {
            f1(Str::lit("if"))
        }
        Adt20::C1 => {
            f1(Str::lit("box"))
        }
        Adt20::C2 => {
            f1(Str::lit("widen"))
        }
        Adt20::C3 => {
            f1(Str::lit("succeed"))
        }
    }
}

pub fn f140(v0: Bytes) -> Bytes {
    let m883 = f166(v0.clone(), f1(Str::lit("checked_neg_")));
    if m883 {
        f1(Str::lit("checked_neg"))
    } else {
        let m884 = f166(v0.clone(), f1(Str::lit("format_")));
        if m884 {
            f1(Str::lit("format_decimal"))
        } else {
            let m885 = f166(v0.clone(), f1(Str::lit("parse_")));
            if m885 {
                f1(Str::lit("parse_decimal"))
            } else {
                let m886 = f166(v0.clone(), f1(Str::lit("append_")));
                if m886 {
                    f1(Str::lit("append"))
                } else {
                    let m887 = f166(v0.clone(), f1(Str::lit("length_")));
                    if m887 {
                        f1(Str::lit("length"))
                    } else {
                        let m888 = f166(v0.clone(), f1(Str::lit("index_")));
                        if m888 {
                            f1(Str::lit("index"))
                        } else {
                            let m889 = f166(v0.clone(), f1(Str::lit("slice_")));
                            if m889 {
                                f1(Str::lit("slice"))
                            } else {
                                let m892 = {
                                    let a890 = v0.clone();
                                    let a891 = f1(Str::lit("nat_succ"));
                                    equal(a890, a891)
                                };
                                if m892 {
                                    f1(Str::lit("succ"))
                                } else {
                                    v0
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f141(v0: Bytes) -> R<Bytes> {
    let m893 = f167(v0.clone(), f1(Str::lit("::")))?;
    match m893 {
        None => {
            Ok(v0)
        }
        Some(v1) => {
            Ok(f40(v0.clone(), {
                let a894 = v1;
                let a895 = 2u64;
                nat_add(a894, a895)?
            }, {
                let a899 = {
                    let a896 = v0;
                    length_bytes(a896)
                };
                let a900 = {
                    let a897 = v1;
                    let a898 = 2u64;
                    nat_add(a897, a898)?
                };
                nat_sub(a899, a900)
            }))
        }
    }
}

pub fn f142(v0: List<Adt25>, v1: Adt22) -> Option<Adt4> {
    let m902 = f104(v0, match v1.clone() {
        Adt22::C0(h901, _) => {
            h901
        }
    });
    match m902 {
        None => {
            None::<Adt4>
        }
        Some(v2) => {
            let a904 = match v2 {
                Adt25::C0(_, _, h903, _, _) => {
                    h903
                }
            };
            let a905 = 0u64;
            if nat_eq(a904, a905) {
                None::<Adt4>
            } else {
                Some(f106(match v1 {
                    Adt22::C0(_, h906) => {
                        h906
                    }
                }))
            }
        }
    }
}

pub fn f143(v0: Adt30, v1: Adt23) -> Adt30 {
    let m907 = v1;
    match m907 {
        Adt23::C0(v2) => {
            f150(v0, v2)
        }
        Adt23::C1 => {
            v0
        }
        Adt23::C2(v3, v4) => {
            f150(f150(v0, v3), v4)
        }
        Adt23::C3(v5, v6) => {
            f150(f150(v0, v5), v6)
        }
        Adt23::C4(_, _) => {
            v0
        }
        Adt23::C5(_, _) => {
            v0
        }
        Adt23::C6 => {
            v0
        }
        Adt23::C7(v11) => {
            f150(v0, v11)
        }
    }
}

pub fn f144(v0: Adt23) -> Bytes {
    let m908 = v0;
    match m908 {
        Adt23::C0(_) => {
            f1(Str::lit("none"))
        }
        Adt23::C1 => {
            f1(Str::lit("some"))
        }
        Adt23::C2(_, _) => {
            f1(Str::lit("ok"))
        }
        Adt23::C3(_, _) => {
            f1(Str::lit("err"))
        }
        Adt23::C4(_, _) => {
            f1(Str::lit("adt"))
        }
        Adt23::C5(_, _) => {
            f1(Str::lit("closure"))
        }
        Adt23::C6 => {
            f1(Str::lit("cons"))
        }
        Adt23::C7(_) => {
            f1(Str::lit("nil"))
        }
    }
}

pub fn f145(v0: Adt17) -> bool {
    let m909 = v0;
    match m909 {
        Adt17::C0(_, _) => {
            false
        }
        Adt17::C1(_, _, _) => {
            false
        }
        Adt17::C2(_, _, _) => {
            false
        }
        Adt17::C3(_, _, _, _) => {
            false
        }
        Adt17::C4(_, _, _, _) => {
            false
        }
        Adt17::C5(_, _, _) => {
            false
        }
        Adt17::C6(_, _, _) => {
            false
        }
        Adt17::C7(_, _, _, _) => {
            false
        }
        Adt17::C8(_, _, _) => {
            false
        }
        Adt17::C9(_) => {
            false
        }
        Adt17::C10(_, _, _) => {
            false
        }
        Adt17::C11(_, _) => {
            false
        }
        Adt17::C12 => {
            true
        }
        Adt17::C13(_, _) => {
            false
        }
        Adt17::C14(_, _, _, _, _, _) => {
            false
        }
        Adt17::C15(_, _, _) => {
            false
        }
        Adt17::C16(_, _) => {
            false
        }
    }
}

pub fn f146(v0: Adt17) -> bool {
    let m910 = v0;
    match m910 {
        Adt17::C0(_, _) => {
            false
        }
        Adt17::C1(_, _, _) => {
            false
        }
        Adt17::C2(_, _, _) => {
            false
        }
        Adt17::C3(_, _, _, _) => {
            false
        }
        Adt17::C4(_, _, _, _) => {
            false
        }
        Adt17::C5(_, _, _) => {
            false
        }
        Adt17::C6(_, _, _) => {
            false
        }
        Adt17::C7(_, _, _, _) => {
            false
        }
        Adt17::C8(_, _, _) => {
            false
        }
        Adt17::C9(_) => {
            false
        }
        Adt17::C10(v31, _, _) => {
            f168(v31)
        }
        Adt17::C11(_, _) => {
            false
        }
        Adt17::C12 => {
            false
        }
        Adt17::C13(_, _) => {
            false
        }
        Adt17::C14(_, _, _, _, _, _) => {
            false
        }
        Adt17::C15(_, _, _) => {
            false
        }
        Adt17::C16(_, _) => {
            false
        }
    }
}

pub fn f147(p915: Adt30, p916: Adt17, v2: Adt9) -> Adt30 {
    let m914 = (p915, p916);
    {
        let mut m914 = m914;
        loop {
            let step = {
                let (v0, v1) = m914;
                let m911 = v1;
                match m911 {
                    Adt17::C0(_, _) => {
                        Err::<(Adt30, Adt17), Adt30>(v0)
                    }
                    Adt17::C1(_, _, _) => {
                        Err::<(Adt30, Adt17), Adt30>(v0)
                    }
                    Adt17::C2(_, _, _) => {
                        Err::<(Adt30, Adt17), Adt30>(v0)
                    }
                    Adt17::C3(_, _, _, _) => {
                        Err::<(Adt30, Adt17), Adt30>(v0)
                    }
                    Adt17::C4(_, _, _, _) => {
                        Err::<(Adt30, Adt17), Adt30>(v0)
                    }
                    Adt17::C5(_, _, _) => {
                        Err::<(Adt30, Adt17), Adt30>(v0)
                    }
                    Adt17::C6(_, _, _) => {
                        Err::<(Adt30, Adt17), Adt30>(v0)
                    }
                    Adt17::C7(_, _, _, _) => {
                        Err::<(Adt30, Adt17), Adt30>(v0)
                    }
                    Adt17::C8(_, _, _) => {
                        Err::<(Adt30, Adt17), Adt30>(v0)
                    }
                    Adt17::C9(_) => {
                        Err::<(Adt30, Adt17), Adt30>(v0)
                    }
                    Adt17::C10(_, _, _) => {
                        Err::<(Adt30, Adt17), Adt30>(v0)
                    }
                    Adt17::C11(_, _) => {
                        Err::<(Adt30, Adt17), Adt30>(v0)
                    }
                    Adt17::C12 => {
                        Err::<(Adt30, Adt17), Adt30>(v0)
                    }
                    Adt17::C13(_, _) => {
                        Err::<(Adt30, Adt17), Adt30>(v0)
                    }
                    Adt17::C14(_, _, _, _, _, _) => {
                        Err::<(Adt30, Adt17), Adt30>(v0)
                    }
                    Adt17::C15(v46, _, r913) => {
                        let v48 = (*r913).clone();
                        let m912 = f169(v46);
                        match m912 {
                            None => {
                                Ok::<(Adt30, Adt17), Adt30>((v0, v48))
                            }
                            Some(v49) => {
                                Ok::<(Adt30, Adt17), Adt30>((f137(v0, v49, v2.clone()), v48))
                            }
                        }
                    }
                    Adt17::C16(_, _) => {
                        Err::<(Adt30, Adt17), Adt30>(v0)
                    }
                }
            };
            match step {
                Ok(next) => { m914 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f148(v0: Adt24) -> bool {
    let m917 = v0;
    match m917 {
        Adt24::C0 => {
            true
        }
        Adt24::C1 => {
            true
        }
        Adt24::C2 => {
            false
        }
        Adt24::C3 => {
            false
        }
    }
}

pub fn f149(v0: Adt14) -> bool {
    let m918 = v0;
    match m918 {
        Adt14::C0(_) => {
            false
        }
        Adt14::C1(_) => {
            false
        }
        Adt14::C2(_) => {
            true
        }
        Adt14::C3(_, _) => {
            false
        }
        Adt14::C4(_, _, _) => {
            false
        }
        Adt14::C5 => {
            false
        }
        Adt14::C6(_, _) => {
            false
        }
    }
}

pub fn f150(v0: Adt30, v1: Adt2) -> Adt30 {
    let m919 = v1;
    match m919 {
        Adt2::C0(v2) => {
            f170(v0, f4(f1(Str::lit("type:")), f171(v2)))
        }
        Adt2::C1(_) => {
            f170(v0, f1(Str::lit("type:fixed")))
        }
        Adt2::C2(v4, r920) => {
            let v5 = (*r920).clone();
            let v6: Adt30 = f170(v0, f4(f1(Str::lit("type:")), f172(v4)));
            f150(v6, v5)
        }
        Adt2::C3(v7, r921, r922) => {
            let v8 = (*r921).clone();
            let v9 = (*r922).clone();
            let v10: Adt30 = f170(v0, f4(f1(Str::lit("type:")), f173(v7)));
            let v11: Adt30 = f150(v10, v8);
            f150(v11, v9)
        }
        Adt2::C4(v12, _) => {
            f170(v0, f4(f1(Str::lit("type:")), f174(v12)))
        }
        Adt2::C5 => {
            v0
        }
        Adt2::C6(r923, r924) => {
            let v14 = (*r923).clone();
            let v15 = (*r924).clone();
            let v16: Adt30 = f150(v0, v14);
            f150(v16, v15)
        }
    }
}

pub fn f151(v0: Fn13, p928: Option<Adt25>, p929: List<Adt25>) -> Option<Adt25> {
    let m927 = (p928, p929);
    {
        let mut m927 = m927;
        loop {
            let step = {
                let (v1, v2) = m927;
                let m925 = v2;
                match m925.uncons() {
                    None => {
                        Err::<(Option<Adt25>, List<Adt25>), Option<Adt25>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Adt25>, List<Adt25>), Option<Adt25>>(({
                            let c926: Fn13 = v0.clone();
                            c926.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m927 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f152(v0: u64, v1: Option<Adt25>, v2: Adt25) -> Option<Adt25> {
    let m930 = v1;
    match m930 {
        None => {
            let a932 = match v2.clone() {
                Adt25::C0(h931, _, _, _, _) => {
                    h931
                }
            };
            let a933 = v0;
            if nat_eq(a932, a933) {
                Some(v2)
            } else {
                None::<Adt25>
            }
        }
        Some(v3) => {
            Some(v3)
        }
    }
}

pub fn f153(v0: List<u64>, v1: List<u64>) -> List<u64> {
    f175(Fn15::F176, v0, v1)
}

pub fn f154(v0: List<Adt25>, p939: Adt17) -> R<Result<(), Bytes>> {
    let m938 = p939;
    {
        let mut m938 = m938;
        loop {
            let step = {
                let v1 = m938;
                let m934 = v1;
                match m934 {
                    Adt17::C0(_, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C1(_, _, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C2(_, _, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C3(_, _, _, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C4(_, _, _, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C5(_, _, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C6(_, _, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C7(_, _, _, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C8(_, _, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C9(_) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C10(_, _, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C11(_, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C12 => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C13(r936, r937) => {
                        let v37 = (*r936).clone();
                        let v38 = (*r937).clone();
                        let m935 = f177(v37.clone())?;
                        if m935 {
                            Ok::<Adt17, R<Result<(), Bytes>>>(v38)
                        } else {
                            Err::<Adt17, R<Result<(), Bytes>>>(Ok(Err::<(), Bytes>(f4(f1(Str::lit("arithmetic: the value a fallible loop ends with, ")), f4(f61(v0.clone(), v37)?, f1(Str::lit(", is not a fallible value")))))))
                        }
                    }
                    Adt17::C14(_, _, _, _, _, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C15(_, _, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C16(_, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                }
            };
            match step {
                Ok(next) => { m938 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f155(v0: List<Adt25>, p945: Adt17) -> R<Result<(), Bytes>> {
    let m944 = p945;
    {
        let mut m944 = m944;
        loop {
            let step = {
                let v1 = m944;
                let m940 = v1;
                match m940 {
                    Adt17::C0(_, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C1(_, _, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C2(_, _, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C3(_, _, _, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C4(_, _, _, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C5(_, _, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C6(_, _, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C7(_, _, _, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C8(_, _, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C9(_) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C10(_, _, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C11(_, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C12 => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C13(_, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C14(_, _, _, _, _, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                    Adt17::C15(_, r942, r943) => {
                        let v46 = (*r942).clone();
                        let v47 = (*r943).clone();
                        let m941 = f79(v0.clone(), v46)?;
                        match m941 {
                            Ok(_) => {
                                Ok::<Adt17, R<Result<(), Bytes>>>(v47)
                            }
                            Err(v49) => {
                                Err::<Adt17, R<Result<(), Bytes>>>(Ok(Err::<(), Bytes>(v49)))
                            }
                        }
                    }
                    Adt17::C16(_, _) => {
                        Err::<Adt17, R<Result<(), Bytes>>>(Ok(Ok::<(), Bytes>(())))
                    }
                }
            };
            match step {
                Ok(next) => { m944 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f156(v0: Option<Adt12>, v1: Adt12) -> Option<Adt12> {
    let m946 = v0;
    match m946 {
        None => {
            Some(v1)
        }
        Some(v2) => {
            let m947 = f178(v1.clone(), v2.clone());
            if m947 {
                Some(v1)
            } else {
                Some(v2)
            }
        }
    }
}

pub fn f157(v0: List<(Adt12, List<u64>)>, v1: Adt12, v2: List<u64>) -> bool {
    let m948 = f94(v0, v1);
    match m948 {
        None => {
            false
        }
        Some(v3) => {
            let v4: bool = f179(v2.clone(), v3.clone());
            let v5: bool = f179(v3, v2);
            if v4 {
                v5
            } else {
                false
            }
        }
    }
}

pub fn f158(v0: Adt12) -> u64 {
    let m949 = v0;
    match m949 {
        Adt12::C0(_) => {
            0u64
        }
        Adt12::C1(_) => {
            1u64
        }
        Adt12::C2(_) => {
            2u64
        }
        Adt12::C3(_) => {
            3u64
        }
        Adt12::C4(_) => {
            4u64
        }
        Adt12::C5(_) => {
            5u64
        }
        Adt12::C6(_) => {
            6u64
        }
        Adt12::C7(_) => {
            7u64
        }
        Adt12::C8(_) => {
            8u64
        }
        Adt12::C9(_) => {
            9u64
        }
    }
}

pub fn f159(v0: Adt12) -> u64 {
    let m950 = v0;
    match m950 {
        Adt12::C0(v1) => {
            v1
        }
        Adt12::C1(v2) => {
            v2
        }
        Adt12::C2(v3) => {
            v3
        }
        Adt12::C3(v4) => {
            v4
        }
        Adt12::C4(v5) => {
            v5
        }
        Adt12::C5(v6) => {
            v6
        }
        Adt12::C6(v7) => {
            v7
        }
        Adt12::C7(v8) => {
            v8
        }
        Adt12::C8(v9) => {
            v9
        }
        Adt12::C9(_) => {
            0u64
        }
    }
}

pub fn f160(v0: Adt12) -> Bytes {
    let m951 = v0;
    match m951 {
        Adt12::C0(_) => {
            Bytes::lit(&[])
        }
        Adt12::C1(_) => {
            Bytes::lit(&[])
        }
        Adt12::C2(_) => {
            Bytes::lit(&[])
        }
        Adt12::C3(_) => {
            Bytes::lit(&[])
        }
        Adt12::C4(_) => {
            Bytes::lit(&[])
        }
        Adt12::C5(_) => {
            Bytes::lit(&[])
        }
        Adt12::C6(_) => {
            Bytes::lit(&[])
        }
        Adt12::C7(_) => {
            Bytes::lit(&[])
        }
        Adt12::C8(_) => {
            Bytes::lit(&[])
        }
        Adt12::C9(v10) => {
            v10
        }
    }
}

pub fn f161(v0: Fn14, p955: bool, p956: List<u64>) -> bool {
    let m954 = (p955, p956);
    {
        let mut m954 = m954;
        loop {
            let step = {
                let (v1, v2) = m954;
                let m952 = v2;
                match m952.uncons() {
                    None => {
                        Err::<(bool, List<u64>), bool>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<u64>), bool>(({
                            let c953: Fn14 = v0.clone();
                            c953.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m954 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f162(v0: u64, v1: bool, v2: u64) -> bool {
    if v1 {
        true
    } else {
        let a957 = v2;
        let a958 = v0;
        nat_eq(a957, a958)
    }
}

pub fn f163(v0: Adt12, v1: List<(Adt12, List<u64>)>, v2: (Adt12, List<u64>)) -> List<(Adt12, List<u64>)> {
    let v3: bool = f123({
        let (h959, _) = v2.clone();
        h959
    }, v0);
    if v3 {
        v1
    } else {
        List::cons(v2, v1)
    }
}

pub fn f164(v0: Bytes, v1: Bytes) -> bool {
    let a964 = {
        let a960 = v0;
        let a961 = v1;
        compare_bytes(a960, a961)
    };
    let a965 = {
        let a962 = Bytes::lit(&[]);
        let a963 = Bytes::lit(&[0x01]);
        compare_bytes(a962, a963)
    };
    equal(a964, a965)
}

pub fn f165(v0: List<Adt29>, v1: Adt29) -> bool {
    f180(Fn16::F181(v1), false, v0)
}

pub fn f166(v0: Bytes, v1: Bytes) -> bool {
    let m970 = {
        let a967 = v0;
        let a968 = 0u64;
        let a969 = {
            let a966 = v1.clone();
            length_bytes(a966)
        };
        slice_bytes(a967, a968, a969)
    };
    match m970 {
        None => {
            false
        }
        Some(v2) => {
            let a971 = v2;
            let a972 = v1;
            equal(a971, a972)
        }
    }
}

pub fn f167(v0: Bytes, v1: Bytes) -> R<Option<u64>> {
    let v2: u64 = {
        let a973 = v0.clone();
        length_bytes(a973)
    };
    let v3: u64 = {
        let a974 = v1.clone();
        length_bytes(a974)
    };
    let v4: u64 = {
        let (h975, _) = f38(Fn2::F182(v1.clone(), v3, v0.clone(), v2), {
            let a976 = v2;
            let a977 = 1u64;
            nat_add(a976, a977)?
        }, 0u64)?;
        h975
    };
    if f183(v0, v1, v4) {
        Ok(Some(v4))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f168(v0: Adt24) -> bool {
    let m978 = v0;
    match m978 {
        Adt24::C0 => {
            false
        }
        Adt24::C1 => {
            false
        }
        Adt24::C2 => {
            true
        }
        Adt24::C3 => {
            false
        }
    }
}

pub fn f169(v0: Adt14) -> Option<Bytes> {
    let m979 = v0;
    match m979 {
        Adt14::C0(v1) => {
            f184(v1)
        }
        Adt14::C1(_) => {
            None::<Bytes>
        }
        Adt14::C2(_) => {
            None::<Bytes>
        }
        Adt14::C3(v4, _) => {
            Some(f185(v4))
        }
        Adt14::C4(_, _, _) => {
            Some(f1(Str::lit("match:adt")))
        }
        Adt14::C5 => {
            None::<Bytes>
        }
        Adt14::C6(_, _) => {
            None::<Bytes>
        }
    }
}

pub fn f170(v0: Adt30, v1: Bytes) -> Adt30 {
    Adt30::C0(match v0.clone() {
        Adt30::C0(h980, _, _) => {
            h980
        }
    }, f186(match v0.clone() {
        Adt30::C0(_, h981, _) => {
            h981
        }
    }, v1), match v0 {
        Adt30::C0(_, _, h982) => {
            h982
        }
    })
}

pub fn f171(v0: Adt3) -> Bytes {
    let m983 = v0;
    match m983 {
        Adt3::C0 => {
            f1(Str::lit("unit"))
        }
        Adt3::C1 => {
            f1(Str::lit("bool"))
        }
        Adt3::C2 => {
            f1(Str::lit("nat"))
        }
        Adt3::C3 => {
            f1(Str::lit("int"))
        }
        Adt3::C4 => {
            f1(Str::lit("ordering"))
        }
        Adt3::C5 => {
            f1(Str::lit("string"))
        }
        Adt3::C6 => {
            f1(Str::lit("bytes"))
        }
        Adt3::C7 => {
            f1(Str::lit("overflow"))
        }
        Adt3::C8 => {
            f1(Str::lit("i128"))
        }
    }
}

pub fn f172(v0: Adt5) -> Bytes {
    let m984 = v0;
    match m984 {
        Adt5::C0 => {
            f1(Str::lit("option"))
        }
        Adt5::C1 => {
            f1(Str::lit("list"))
        }
        Adt5::C2 => {
            f1(Str::lit("rc"))
        }
        Adt5::C3 => {
            f1(Str::lit("fallible"))
        }
        Adt5::C4 => {
            f1(Str::lit("ref"))
        }
    }
}

pub fn f173(v0: Adt6) -> Bytes {
    let m985 = v0;
    match m985 {
        Adt6::C0 => {
            f1(Str::lit("result"))
        }
        Adt6::C1 => {
            f1(Str::lit("pair"))
        }
    }
}

pub fn f174(v0: Adt7) -> Bytes {
    let m986 = v0;
    match m986 {
        Adt7::C0 => {
            f1(Str::lit("adt"))
        }
        Adt7::C1 => {
            f1(Str::lit("fn"))
        }
    }
}

pub fn f175(v0: Fn15, p990: List<u64>, p991: List<u64>) -> List<u64> {
    let m989 = (p990, p991);
    {
        let mut m989 = m989;
        loop {
            let step = {
                let (v1, v2) = m989;
                let m987 = v2;
                match m987.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>(({
                            let c988: Fn15 = v0.clone();
                            c988.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m989 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f176(v0: List<u64>, v1: u64) -> List<u64> {
    let v2: bool = f129(v0.clone(), v1);
    if v2 {
        v0
    } else {
        List::cons(v1, v0)
    }
}

pub fn f177(v0: Adt17) -> R<bool> {
    let m992 = v0;
    match m992 {
        Adt17::C0(_, _) => {
            Ok(false)
        }
        Adt17::C1(_, _, _) => {
            Ok(false)
        }
        Adt17::C2(v6, _, _) => {
            let m993 = v6;
            match m993 {
                Adt20::C0 => {
                    Ok(false)
                }
                Adt20::C1 => {
                    Ok(false)
                }
                Adt20::C2 => {
                    Ok(false)
                }
                Adt20::C3 => {
                    Ok(true)
                }
            }
        }
        Adt17::C3(_, _, v11, _) => {
            Ok(!v11)
        }
        Adt17::C4(_, _, v15, _) => {
            Ok(!v15)
        }
        Adt17::C5(v17, r994, _) => {
            let v18 = (*r994).clone();
            f72(v17, v18)
        }
        Adt17::C6(_, _, _) => {
            Ok(false)
        }
        Adt17::C7(_, _, _, _) => {
            Ok(true)
        }
        Adt17::C8(_, _, _) => {
            Ok(true)
        }
        Adt17::C9(_) => {
            Ok(true)
        }
        Adt17::C10(_, _, _) => {
            Ok(false)
        }
        Adt17::C11(_, _) => {
            Ok(false)
        }
        Adt17::C12 => {
            Ok(false)
        }
        Adt17::C13(_, _) => {
            Ok(false)
        }
        Adt17::C14(_, _, _, _, _, _) => {
            Ok(false)
        }
        Adt17::C15(_, _, _) => {
            Ok(false)
        }
        Adt17::C16(_, _) => {
            Ok(true)
        }
    }
}

pub fn f178(v0: Adt12, v1: Adt12) -> bool {
    let a995 = f158(v0.clone());
    let a996 = f158(v1.clone());
    if nat_eq(a995, a996) {
        let a997 = f158(v0.clone());
        let a998 = 9u64;
        if nat_eq(a997, a998) {
            f164(f160(v0), f160(v1))
        } else {
            let a999 = f159(v0);
            let a1000 = f159(v1);
            nat_lt(a999, a1000)
        }
    } else {
        let a1001 = f158(v0);
        let a1002 = f158(v1);
        nat_lt(a1001, a1002)
    }
}

pub fn f179(v0: List<u64>, v1: List<u64>) -> bool {
    f161(Fn14::F187(v1), true, v0)
}

pub fn f180(v0: Fn16, p1006: bool, p1007: List<Adt29>) -> bool {
    let m1005 = (p1006, p1007);
    {
        let mut m1005 = m1005;
        loop {
            let step = {
                let (v1, v2) = m1005;
                let m1003 = v2;
                match m1003.uncons() {
                    None => {
                        Err::<(bool, List<Adt29>), bool>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<Adt29>), bool>(({
                            let c1004: Fn16 = v0.clone();
                            c1004.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1005 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f181(v0: Adt29, v1: bool, v2: Adt29) -> bool {
    let v3: bool = f188(v2, v0);
    if v1 {
        true
    } else {
        v3
    }
}

pub fn f182(v0: Bytes, v1: u64, v2: Bytes, v3: u64, v4: u64) -> R<Option<u64>> {
    if f183(v2, v0, v4) {
        Ok(None::<u64>)
    } else {
        let a1010 = {
            let a1008 = v4;
            let a1009 = v1;
            nat_add(a1008, a1009)?
        };
        let a1011 = v3;
        if nat_lt(a1010, a1011) {
            Ok(Some({
                let a1012 = v4;
                let a1013 = 1u64;
                nat_add(a1012, a1013)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f183(v0: Bytes, v1: Bytes, v2: u64) -> bool {
    let m1018 = {
        let a1015 = v0;
        let a1016 = v2;
        let a1017 = {
            let a1014 = v1.clone();
            length_bytes(a1014)
        };
        slice_bytes(a1015, a1016, a1017)
    };
    match m1018 {
        None => {
            false
        }
        Some(v3) => {
            let a1019 = v3;
            let a1020 = v1;
            equal(a1019, a1020)
        }
    }
}

pub fn f184(v0: Adt15) -> Option<Bytes> {
    let m1021 = v0;
    match m1021 {
        Adt15::C0 => {
            None::<Bytes>
        }
        Adt15::C1 => {
            None::<Bytes>
        }
        Adt15::C2 => {
            Some(f1(Str::lit("match:option")))
        }
        Adt15::C3 => {
            Some(f1(Str::lit("match:ordering")))
        }
    }
}

pub fn f185(v0: Adt16) -> Bytes {
    let m1022 = v0;
    match m1022 {
        Adt16::C0 => {
            f1(Str::lit("match:option"))
        }
        Adt16::C1 => {
            f1(Str::lit("match:result"))
        }
        Adt16::C2 => {
            f1(Str::lit("match:result"))
        }
    }
}

pub fn f186(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    let m1023 = f189(v0.clone(), v1.clone());
    if m1023 {
        v0
    } else {
        let v2: (bool, List<Bytes>) = f190(Fn17::F191(v1.clone()), (false, List::<Bytes>::nil()), v0);
        let m1025 = {
            let (h1024, _) = v2;
            h1024
        };
        if m1025 {
            f192({
                let (_, h1026) = v2;
                h1026
            })
        } else {
            f192(List::cons(v1, {
                let (_, h1027) = v2;
                h1027
            }))
        }
    }
}

pub fn f187(v0: List<u64>, v1: bool, v2: u64) -> bool {
    let v3: bool = f129(v0, v2);
    if v1 {
        v3
    } else {
        false
    }
}

pub fn f188(v0: Adt29, v1: Adt29) -> bool {
    let a1030 = match v0.clone() {
        Adt29::C0(h1028, _, _, _) => {
            h1028
        }
    };
    let a1031 = match v1.clone() {
        Adt29::C0(h1029, _, _, _) => {
            h1029
        }
    };
    if equal(a1030, a1031) {
        let a1034 = match v0.clone() {
            Adt29::C0(_, h1032, _, _) => {
                h1032
            }
        };
        let a1035 = match v1.clone() {
            Adt29::C0(_, h1033, _, _) => {
                h1033
            }
        };
        if equal(a1034, a1035) {
            if f193(match v0.clone() {
                Adt29::C0(_, _, h1036, _) => {
                    h1036
                }
            }, match v1.clone() {
                Adt29::C0(_, _, h1037, _) => {
                    h1037
                }
            }) {
                f193(match v0 {
                    Adt29::C0(_, _, _, h1038) => {
                        h1038
                    }
                }, match v1 {
                    Adt29::C0(_, _, _, h1039) => {
                        h1039
                    }
                })
            } else {
                false
            }
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f189(v0: List<Bytes>, v1: Bytes) -> bool {
    f194(Fn18::F195(v1), false, v0)
}

pub fn f190(v0: Fn17, p1043: (bool, List<Bytes>), p1044: List<Bytes>) -> (bool, List<Bytes>) {
    let m1042 = (p1043, p1044);
    {
        let mut m1042 = m1042;
        loop {
            let step = {
                let (v1, v2) = m1042;
                let m1040 = v2;
                match m1040.uncons() {
                    None => {
                        Err::<((bool, List<Bytes>), List<Bytes>), (bool, List<Bytes>)>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<((bool, List<Bytes>), List<Bytes>), (bool, List<Bytes>)>(({
                            let c1041: Fn17 = v0.clone();
                            c1041.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1042 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f191(v0: Bytes, v1: (bool, List<Bytes>), v2: Bytes) -> (bool, List<Bytes>) {
    f196(v0, v1, v2)
}

pub fn f192(v0: List<Bytes>) -> List<Bytes> {
    f197(Fn19::F198, List::<Bytes>::nil(), v0)
}

pub fn f193(v0: Option<Adt4>, v1: Option<Adt4>) -> bool {
    let a1045 = f199(v0);
    let a1046 = f199(v1);
    equal(a1045, a1046)
}

pub fn f194(v0: Fn18, p1050: bool, p1051: List<Bytes>) -> bool {
    let m1049 = (p1050, p1051);
    {
        let mut m1049 = m1049;
        loop {
            let step = {
                let (v1, v2) = m1049;
                let m1047 = v2;
                match m1047.uncons() {
                    None => {
                        Err::<(bool, List<Bytes>), bool>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<Bytes>), bool>(({
                            let c1048: Fn18 = v0.clone();
                            c1048.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1049 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f195(v0: Bytes, v1: bool, v2: Bytes) -> bool {
    if v1 {
        true
    } else {
        let a1052 = v2;
        let a1053 = v0;
        equal(a1052, a1053)
    }
}

pub fn f196(v0: Bytes, v1: (bool, List<Bytes>), v2: Bytes) -> (bool, List<Bytes>) {
    let v3: bool = {
        let (h1054, _) = v1;
        h1054
    };
    let v4: List<Bytes> = {
        let (_, h1055) = v1;
        h1055
    };
    let m1056 = v3;
    if m1056 {
        (true, List::cons(v2, v4))
    } else {
        let m1057 = f164(v0.clone(), v2.clone());
        if m1057 {
            (true, List::cons(v2, List::cons(v0, v4)))
        } else {
            (false, List::cons(v2, v4))
        }
    }
}

pub fn f197(v0: Fn19, p1061: List<Bytes>, p1062: List<Bytes>) -> List<Bytes> {
    let m1060 = (p1061, p1062);
    {
        let mut m1060 = m1060;
        loop {
            let step = {
                let (v1, v2) = m1060;
                let m1058 = v2;
                match m1058.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c1059: Fn19 = v0.clone();
                            c1059.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1060 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f198(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f199(v0: Option<Adt4>) -> Bytes {
    let m1063 = v0;
    match m1063 {
        None => {
            f1(Str::lit("None"))
        }
        Some(v1) => {
            f4(f1(Str::lit("Some(")), f4(f14(v1), f1(Str::lit(")"))))
        }
    }
}

pub fn run(p0: Adt0) -> R<Result<(), Bytes>> {
    f0(p0)
}
