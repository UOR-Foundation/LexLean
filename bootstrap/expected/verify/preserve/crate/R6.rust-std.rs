#![forbid(unsafe_code)]
use core::cmp::Ordering;
use core::sync::atomic::{AtomicU64, Ordering as Memory};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Overflow;
pub type R<T> = Result<T, Overflow>;

static WORK: AtomicU64 = AtomicU64::new(0);
pub fn tick(units: u64) { WORK.fetch_add(units, Memory::Relaxed); }
pub fn work() -> u64 { WORK.load(Memory::Relaxed) }

pub fn nat_add(a: u64, b: u64) -> R<u64> { a.checked_add(b).ok_or(Overflow) }
pub fn nat_sub(a: u64, b: u64) -> u64 { a.saturating_sub(b) }
pub fn nat_mul(a: u64, b: u64) -> R<u64> { a.checked_mul(b).ok_or(Overflow) }
pub fn nat_quot(a: u64, b: u64, z: u64) -> u64 { a.checked_div(b).unwrap_or(z) }
pub fn nat_rem(a: u64, b: u64, z: u64) -> u64 { a.checked_rem(b).unwrap_or(z) }
pub fn nat_eq(a: u64, b: u64) -> bool { a == b }
pub fn nat_le(a: u64, b: u64) -> bool { a <= b }
pub fn nat_lt(a: u64, b: u64) -> bool { a < b }
pub fn nat_succ(a: u64) -> R<u64> { a.checked_add(1).ok_or(Overflow) }
pub fn int_add(a: i64, b: i64) -> R<i64> { a.checked_add(b).ok_or(Overflow) }
pub fn int_sub(a: i64, b: i64) -> R<i64> { a.checked_sub(b).ok_or(Overflow) }
pub fn int_mul(a: i64, b: i64) -> R<i64> { a.checked_mul(b).ok_or(Overflow) }
pub fn int_neg(a: i64) -> R<i64> { a.checked_neg().ok_or(Overflow) }
pub fn int_quot(a: i64, b: i64, z: i64) -> R<i64> { if b == 0 { Ok(z) } else { a.checked_div(b).ok_or(Overflow) } }
pub fn int_rem(a: i64, b: i64, z: i64) -> i64 { if b == 0 { z } else { a.wrapping_rem(b) } }
pub fn bool_not(a: bool) -> bool { !a }
pub fn bool_and(a: bool, b: bool) -> bool { a && b }
pub fn bool_or(a: bool, b: bool) -> bool { a || b }

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

macro_rules! fixed {
    ($m:ident, $t:ident) => {
        pub mod $m {
            pub fn checked_add(a: $t, b: $t) -> Option<$t> { a.checked_add(b) }
            pub fn checked_sub(a: $t, b: $t) -> Option<$t> { a.checked_sub(b) }
            pub fn checked_mul(a: $t, b: $t) -> Option<$t> { a.checked_mul(b) }
            pub fn checked_quot(a: $t, b: $t) -> Option<$t> { a.checked_div(b) }
            pub fn bit_and(a: $t, b: $t) -> $t { a & b }
            pub fn bit_or(a: $t, b: $t) -> $t { a | b }
            pub fn bit_xor(a: $t, b: $t) -> $t { a ^ b }
            pub fn bit_not(a: $t) -> $t { !a }
            pub fn shift_left(a: $t, amount: u32) -> Option<$t> { if amount < $t::BITS { Some(a.wrapping_shl(amount)) } else { None } }
            pub fn shift_right(a: $t, amount: u32) -> Option<$t> { if amount < $t::BITS { Some(a.wrapping_shr(amount)) } else { None } }
            pub fn convert(a: i128) -> Option<$t> { $t::try_from(a).ok() }
        }
    };
}
fixed!(fixed_u8, u8); fixed!(fixed_u16, u16); fixed!(fixed_u32, u32); fixed!(fixed_u64, u64);
fixed!(fixed_i8, i8); fixed!(fixed_i16, i16); fixed!(fixed_i32, i32); fixed!(fixed_i64, i64);
pub fn checked_neg_i8(a: i8) -> Option<i8> { a.checked_neg() }
pub fn checked_neg_i16(a: i16) -> Option<i16> { a.checked_neg() }
pub fn checked_neg_i32(a: i32) -> Option<i32> { a.checked_neg() }
pub fn checked_neg_i64(a: i64) -> Option<i64> { a.checked_neg() }

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
    C0(bool, Adt26, bool, List<Adt25>, bool),
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
    F175(Bytes, u64, Bytes, u64),
}

impl Fn2 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn2::F39(k0) => f39(k0.clone(), p0),
            Fn2::F175(k0, k1, k2, k3) => f175(k0.clone(), *k1, k2.clone(), *k3, p0),
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
    F89(Adt12),
}

impl Fn4 {
    pub fn apply(&self, p0: bool, p1: Adt12) -> bool {
        match self {
            Fn4::F89(k0) => f89(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F105,
    F109(Adt12),
}

impl Fn5 {
    pub fn apply(&self, p0: List<Adt12>, p1: Adt12) -> List<Adt12> {
        match self {
            Fn5::F105 => f105(p0, p1),
            Fn5::F109(k0) => f109(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F107,
    F156(Adt12),
}

impl Fn6 {
    pub fn apply(&self, p0: List<(Adt12, List<u64>)>, p1: (Adt12, List<u64>)) -> List<(Adt12, List<u64>)> {
        match self {
            Fn6::F107 => f107(p0, p1),
            Fn6::F156(k0) => f156(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F111(List<Adt12>, List<Adt12>, Adt12),
}

impl Fn7 {
    pub fn apply(&self, p0: Option<Adt12>, p1: Adt12) -> Option<Adt12> {
        match self {
            Fn7::F111(k0, k1, k2) => f111(k0.clone(), k1.clone(), k2.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F113(List<(Adt12, List<u64>)>, List<Adt12>),
}

impl Fn8 {
    pub fn apply(&self, p0: Option<Adt12>, p1: (Adt12, List<u64>)) -> Option<Adt12> {
        match self {
            Fn8::F113(k0, k1) => f113(k0.clone(), k1.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F115(Adt12),
}

impl Fn9 {
    pub fn apply(&self, p0: Option<u64>, p1: (Adt12, u64)) -> Option<u64> {
        match self {
            Fn9::F115(k0) => f115(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F119(u64),
}

impl Fn10 {
    pub fn apply(&self, p0: Option<u64>, p1: (u64, u64)) -> Option<u64> {
        match self {
            Fn10::F119(k0) => f119(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F122(Adt12),
}

impl Fn11 {
    pub fn apply(&self, p0: Option<List<u64>>, p1: (Adt12, List<u64>)) -> Option<List<u64>> {
        match self {
            Fn11::F122(k0) => f122(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F126,
}

impl Fn12 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Adt29) -> Option<Bytes> {
        match self {
            Fn12::F126 => f126(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F145(u64),
}

impl Fn13 {
    pub fn apply(&self, p0: Option<Adt25>, p1: Adt25) -> Option<Adt25> {
        match self {
            Fn13::F145(k0) => f145(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F155(u64),
    F180(List<u64>),
}

impl Fn14 {
    pub fn apply(&self, p0: bool, p1: u64) -> bool {
        match self {
            Fn14::F155(k0) => f155(*k0, p0, p1),
            Fn14::F180(k0) => f180(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F169,
}

impl Fn15 {
    pub fn apply(&self, p0: List<u64>, p1: u64) -> List<u64> {
        match self {
            Fn15::F169 => f169(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F174(Adt29),
}

impl Fn16 {
    pub fn apply(&self, p0: bool, p1: Adt29) -> bool {
        match self {
            Fn16::F174(k0) => f174(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F184(Bytes),
}

impl Fn17 {
    pub fn apply(&self, p0: (bool, List<Bytes>), p1: Bytes) -> (bool, List<Bytes>) {
        match self {
            Fn17::F184(k0) => f184(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F188(Bytes),
}

impl Fn18 {
    pub fn apply(&self, p0: bool, p1: Bytes) -> bool {
        match self {
            Fn18::F188(k0) => f188(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F191,
}

impl Fn19 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn19::F191 => f191(p0, p1),
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
            f24(Adt27::C0(v0, v1, false, v2, false), v5)
        }
        Adt1::C1(v7, v8, v9, v10, v11, _) => {
            let v13: Adt27 = Adt27::C0(v0, v1, false, v2, false);
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
            let v24: Adt27 = Adt27::C0(v0, v1.clone(), v23, v2, false);
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
        Adt27::C0(h143, _, _, _, _) => {
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
                                Adt27::C0(_, h147, _, _, _) => {
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
                                    Adt27::C0(_, h159, _, _, _) => {
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
                        Adt27::C0(_, h224, _, _, _) => {
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
                        Adt27::C0(_, h228, _, _, _) => {
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
        Adt27::C0(h234, _, _, _, _) => {
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
                                            Adt27::C0(_, h261, _, _, _) => {
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
                if match v1.clone() {
                    Adt27::C0(_, _, _, _, h269) => {
                        h269
                    }
                } {
                    false
                } else {
                    match v1.clone() {
                        Adt27::C0(_, _, h270, _, _) => {
                            h270
                        }
                    }
                }
            } else {
                false
            };
            let m271 = v6;
            if m271 {
                Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("arithmetic: the value a fallible function returns, ")), f4(f61(match v1 {
                    Adt27::C0(_, _, _, h272, _) => {
                        h272
                    }
                }, Adt17::C0(v4, v5))?, f1(Str::lit(", is not a fallible value"))))))
            } else {
                let m273 = v4;
                match m273 {
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
                }
            }
        }
        Adt17::C1(v8, v9, v10) => {
            let v11: bool = if v3 {
                if match v1.clone() {
                    Adt27::C0(_, _, _, _, h274) => {
                        h274
                    }
                } {
                    false
                } else {
                    match v1.clone() {
                        Adt27::C0(_, _, h275, _, _) => {
                            h275
                        }
                    }
                }
            } else {
                false
            };
            let m276 = v11;
            if m276 {
                Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("arithmetic: the value a fallible function returns, ")), f4(f61(match v1 {
                    Adt27::C0(_, _, _, h277, _) => {
                        h277
                    }
                }, Adt17::C1(v8, v9, v10))?, f1(Str::lit(", is not a fallible value"))))))
            } else {
                let m278 = v8;
                match m278 {
                    Adt19::C0 => {
                        f63(v0, v9)
                    }
                    Adt19::C1 => {
                        let m279 = f64(v0.clone(), v9)?;
                        match m279 {
                            Ok(_) => {
                                Ok(Ok::<Adt28, Bytes>(v0))
                            }
                            Err(v13) => {
                                Ok(Err::<Adt28, Bytes>(v13))
                            }
                        }
                    }
                    Adt19::C2 => {
                        let m280 = f64(v0.clone(), v9)?;
                        match m280 {
                            Ok(_) => {
                                Ok(Ok::<Adt28, Bytes>(v0))
                            }
                            Err(v15) => {
                                Ok(Err::<Adt28, Bytes>(v15))
                            }
                        }
                    }
                    Adt19::C3 => {
                        let m281 = f64(v0.clone(), v9)?;
                        match m281 {
                            Ok(_) => {
                                Ok(Ok::<Adt28, Bytes>(v0))
                            }
                            Err(v17) => {
                                Ok(Err::<Adt28, Bytes>(v17))
                            }
                        }
                    }
                    Adt19::C4 => {
                        let m282 = f44(v1, f1(Str::lit("a boxed read")));
                        match m282 {
                            Ok(_) => {
                                let m283 = f64(v0.clone(), v9)?;
                                match m283 {
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
        Adt17::C2(v22, r440, v24) => {
            let v23 = (*r440).clone();
            let m284 = v22.clone();
            match m284 {
                Adt20::C0 => {
                    let v25: bool = if v3 {
                        if match v1.clone() {
                            Adt27::C0(_, _, _, _, h285) => {
                                h285
                            }
                        } {
                            false
                        } else {
                            match v1.clone() {
                                Adt27::C0(_, _, h286, _, _) => {
                                    h286
                                }
                            }
                        }
                    } else {
                        false
                    };
                    let m287 = v25;
                    if m287 {
                        Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("arithmetic: the value a fallible function returns, ")), f4(f61(match v1 {
                            Adt27::C0(_, _, _, h288, _) => {
                                h288
                            }
                        }, Adt17::C2(v22, std::rc::Rc::new(v23), v24))?, f1(Str::lit(", is not a fallible value"))))))
                    } else {
                        let m289 = f65(v23.clone());
                        if m289 {
                            Ok(Err::<Adt28, Bytes>(f1(Str::lit("a negated zero test or predecessor: a zero test is negated as its complement, and a predecessor is not a Boolean"))))
                        } else {
                            f47(v0, v1, v23, false)
                        }
                    }
                }
                Adt20::C1 => {
                    let v26: bool = if v3 {
                        if match v1.clone() {
                            Adt27::C0(_, _, _, _, h290) => {
                                h290
                            }
                        } {
                            false
                        } else {
                            match v1.clone() {
                                Adt27::C0(_, _, h291, _, _) => {
                                    h291
                                }
                            }
                        }
                    } else {
                        false
                    };
                    let m292 = v26;
                    if m292 {
                        Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("arithmetic: the value a fallible function returns, ")), f4(f61(match v1 {
                            Adt27::C0(_, _, _, h293, _) => {
                                h293
                            }
                        }, Adt17::C2(v22, std::rc::Rc::new(v23), v24))?, f1(Str::lit(", is not a fallible value"))))))
                    } else {
                        let m294 = f44(v1.clone(), f1(Str::lit("a box")));
                        match m294 {
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
                        if match v1.clone() {
                            Adt27::C0(_, _, _, _, h295) => {
                                h295
                            }
                        } {
                            false
                        } else {
                            match v1.clone() {
                                Adt27::C0(_, _, h296, _, _) => {
                                    h296
                                }
                            }
                        }
                    } else {
                        false
                    };
                    let m297 = v29;
                    if m297 {
                        Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("arithmetic: the value a fallible function returns, ")), f4(f61(match v1 {
                            Adt27::C0(_, _, _, h298, _) => {
                                h298
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
        Adt17::C3(v30, r441, v32, v33) => {
            let v31 = (*r441).clone();
            let m299 = v32;
            if m299 {
                let v34: bool = if v3 {
                    if match v1.clone() {
                        Adt27::C0(_, _, _, _, h300) => {
                            h300
                        }
                    } {
                        false
                    } else {
                        match v1.clone() {
                            Adt27::C0(_, _, h301, _, _) => {
                                h301
                            }
                        }
                    }
                } else {
                    false
                };
                let m302 = v34;
                if m302 {
                    Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("arithmetic: the value a fallible function returns, ")), f4(f61(match v1 {
                        Adt27::C0(_, _, _, h303, _) => {
                            h303
                        }
                    }, Adt17::C3(v30, std::rc::Rc::new(v31), v32, v33))?, f1(Str::lit(", is not a fallible value"))))))
                } else {
                    let m304 = f66(v0, v1.clone(), v31)?;
                    match m304 {
                        Ok(v35) => {
                            let m305 = v30;
                            match m305 {
                                Adt21::C0(v36) => {
                                    let m309 = f20({
                                        let m308 = match v1.clone() {
                                            Adt27::C0(_, h307, _, _, _) => {
                                                h307
                                            }
                                        };
                                        match m308 {
                                            Adt26::C0(_, _, _, h306) => {
                                                h306
                                            }
                                        }
                                    }, v36);
                                    match m309 {
                                        None => {
                                            Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("f")), f4(f10(v36)?, f1(Str::lit(" is not declared"))))))
                                        }
                                        Some(v37) => {
                                            Ok(f67(v35, v1.clone(), v37, v32, if match v1 {
                                                Adt27::C0(_, _, _, _, h310) => {
                                                    h310
                                                }
                                            } {
                                                false
                                            } else {
                                                v3
                                            }, f4(f1(Str::lit("f")), f10(v36)?)))
                                        }
                                    }
                                }
                                Adt21::C1(v38) => {
                                    let v39: Bytes = f68(match v1.clone() {
                                        Adt27::C0(_, _, _, h311, _) => {
                                            h311
                                        }
                                    }, v38.clone());
                                    let m314 = {
                                        let m313 = f69(match v1.clone() {
                                            Adt27::C0(_, _, _, h312, _) => {
                                                h312
                                            }
                                        }, v38.clone());
                                        if m313 {
                                            f44(v1.clone(), f4(f1(Str::lit("the runtime function ")), f30(v39.clone())))
                                        } else {
                                            Ok::<(), Bytes>(())
                                        }
                                    };
                                    match m314 {
                                        Ok(_) => {
                                            Ok(f67(v35, v1.clone(), f70(match v1.clone() {
                                                Adt27::C0(_, _, _, h315, _) => {
                                                    h315
                                                }
                                            }, v38), v32, if match v1 {
                                                Adt27::C0(_, _, _, _, h316) => {
                                                    h316
                                                }
                                            } {
                                                false
                                            } else {
                                                v3
                                            }, f30(v39)))
                                        }
                                        Err(v41) => {
                                            Ok(Err::<Adt28, Bytes>(v41))
                                        }
                                    }
                                }
                            }
                        }
                        Err(v42) => {
                            Ok(Err::<Adt28, Bytes>(v42))
                        }
                    }
                }
            } else {
                let m317 = f66(v0, v1.clone(), v31)?;
                match m317 {
                    Ok(v43) => {
                        let m318 = v30;
                        match m318 {
                            Adt21::C0(v44) => {
                                let m322 = f20({
                                    let m321 = match v1.clone() {
                                        Adt27::C0(_, h320, _, _, _) => {
                                            h320
                                        }
                                    };
                                    match m321 {
                                        Adt26::C0(_, _, _, h319) => {
                                            h319
                                        }
                                    }
                                }, v44);
                                match m322 {
                                    None => {
                                        Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("f")), f4(f10(v44)?, f1(Str::lit(" is not declared"))))))
                                    }
                                    Some(v45) => {
                                        Ok(f67(v43, v1.clone(), v45, v32, if match v1 {
                                            Adt27::C0(_, _, _, _, h323) => {
                                                h323
                                            }
                                        } {
                                            false
                                        } else {
                                            v3
                                        }, f4(f1(Str::lit("f")), f10(v44)?)))
                                    }
                                }
                            }
                            Adt21::C1(v46) => {
                                let v47: Bytes = f68(match v1.clone() {
                                    Adt27::C0(_, _, _, h324, _) => {
                                        h324
                                    }
                                }, v46.clone());
                                let m327 = {
                                    let m326 = f69(match v1.clone() {
                                        Adt27::C0(_, _, _, h325, _) => {
                                            h325
                                        }
                                    }, v46.clone());
                                    if m326 {
                                        f44(v1.clone(), f4(f1(Str::lit("the runtime function ")), f30(v47.clone())))
                                    } else {
                                        Ok::<(), Bytes>(())
                                    }
                                };
                                match m327 {
                                    Ok(_) => {
                                        Ok(f67(v43, v1.clone(), f70(match v1.clone() {
                                            Adt27::C0(_, _, _, h328, _) => {
                                                h328
                                            }
                                        }, v46), v32, if match v1 {
                                            Adt27::C0(_, _, _, _, h329) => {
                                                h329
                                            }
                                        } {
                                            false
                                        } else {
                                            v3
                                        }, f30(v47)))
                                    }
                                    Err(v49) => {
                                        Ok(Err::<Adt28, Bytes>(v49))
                                    }
                                }
                            }
                        }
                    }
                    Err(v50) => {
                        Ok(Err::<Adt28, Bytes>(v50))
                    }
                }
            }
        }
        Adt17::C4(v51, r442, v53, v54) => {
            let v52 = (*r442).clone();
            let m330 = v53;
            if m330 {
                let v55: bool = if v3 {
                    if match v1.clone() {
                        Adt27::C0(_, _, _, _, h331) => {
                            h331
                        }
                    } {
                        false
                    } else {
                        match v1.clone() {
                            Adt27::C0(_, _, h332, _, _) => {
                                h332
                            }
                        }
                    }
                } else {
                    false
                };
                let m333 = v55;
                if m333 {
                    Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("arithmetic: the value a fallible function returns, ")), f4(f61(match v1 {
                        Adt27::C0(_, _, _, h334, _) => {
                            h334
                        }
                    }, Adt17::C4(v51, std::rc::Rc::new(v52), v53, v54))?, f1(Str::lit(", is not a fallible value"))))))
                } else {
                    let m335 = f64(v0.clone(), v51)?;
                    match m335 {
                        Ok(_) => {
                            let m336 = f66(v0, v1, v52)?;
                            match m336 {
                                Ok(v57) => {
                                    Ok(Ok::<Adt28, Bytes>(Adt28::C0(match v57.clone() {
                                        Adt28::C0(h337, _, _, _, _) => {
                                            h337
                                        }
                                    }, match v57.clone() {
                                        Adt28::C0(_, h338, _, _, _) => {
                                            h338
                                        }
                                    }, match v57.clone() {
                                        Adt28::C0(_, _, h339, _, _) => {
                                            h339
                                        }
                                    }, match v57.clone() {
                                        Adt28::C0(_, _, _, h340, _) => {
                                            h340
                                        }
                                    }, if v53 {
                                        true
                                    } else {
                                        match v57 {
                                            Adt28::C0(_, _, _, _, h341) => {
                                                h341
                                            }
                                        }
                                    })))
                                }
                                Err(v58) => {
                                    Ok(Err::<Adt28, Bytes>(v58))
                                }
                            }
                        }
                        Err(v59) => {
                            Ok(Err::<Adt28, Bytes>(v59))
                        }
                    }
                }
            } else {
                let m342 = f64(v0.clone(), v51)?;
                match m342 {
                    Ok(_) => {
                        let m343 = f66(v0, v1, v52)?;
                        match m343 {
                            Ok(v61) => {
                                Ok(Ok::<Adt28, Bytes>(Adt28::C0(match v61.clone() {
                                    Adt28::C0(h344, _, _, _, _) => {
                                        h344
                                    }
                                }, match v61.clone() {
                                    Adt28::C0(_, h345, _, _, _) => {
                                        h345
                                    }
                                }, match v61.clone() {
                                    Adt28::C0(_, _, h346, _, _) => {
                                        h346
                                    }
                                }, match v61.clone() {
                                    Adt28::C0(_, _, _, h347, _) => {
                                        h347
                                    }
                                }, if v53 {
                                    true
                                } else {
                                    match v61 {
                                        Adt28::C0(_, _, _, _, h348) => {
                                            h348
                                        }
                                    }
                                })))
                            }
                            Err(v62) => {
                                Ok(Err::<Adt28, Bytes>(v62))
                            }
                        }
                    }
                    Err(v63) => {
                        Ok(Err::<Adt28, Bytes>(v63))
                    }
                }
            }
        }
        Adt17::C5(v64, r443, v66) => {
            let v65 = (*r443).clone();
            let v67: bool = if v3 {
                if match v1.clone() {
                    Adt27::C0(_, _, _, _, h349) => {
                        h349
                    }
                } {
                    f71(v64.clone())
                } else {
                    false
                }
            } else {
                false
            };
            let m350 = v67;
            if m350 {
                let m355 = f72(v0, Adt27::C0(match v1.clone() {
                    Adt27::C0(h351, _, _, _, _) => {
                        h351
                    }
                }, match v1.clone() {
                    Adt27::C0(_, h352, _, _, _) => {
                        h352
                    }
                }, match v1.clone() {
                    Adt27::C0(_, _, h353, _, _) => {
                        h353
                    }
                }, match v1.clone() {
                    Adt27::C0(_, _, _, h354, _) => {
                        h354
                    }
                }, false), v65.clone())?;
                match m355 {
                    Ok(v68) => {
                        let m356 = f73(v1, v64, f74(v65)?)?;
                        match m356 {
                            Ok(_) => {
                                Ok(Ok::<Adt28, Bytes>(v68))
                            }
                            Err(v70) => {
                                Ok(Err::<Adt28, Bytes>(v70))
                            }
                        }
                    }
                    Err(v71) => {
                        Ok(Err::<Adt28, Bytes>(v71))
                    }
                }
            } else {
                let v72: bool = if v3 {
                    if match v1.clone() {
                        Adt27::C0(_, _, _, _, h357) => {
                            h357
                        }
                    } {
                        false
                    } else {
                        match v1.clone() {
                            Adt27::C0(_, _, h358, _, _) => {
                                h358
                            }
                        }
                    }
                } else {
                    false
                };
                let m359 = v72;
                if m359 {
                    Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("arithmetic: the value a fallible function returns, ")), f4(f61(match v1 {
                        Adt27::C0(_, _, _, h360, _) => {
                            h360
                        }
                    }, Adt17::C5(v64, std::rc::Rc::new(v65), v66))?, f1(Str::lit(", is not a fallible value"))))))
                } else {
                    let m361 = f66(v0, v1.clone(), v65.clone())?;
                    match m361 {
                        Ok(v73) => {
                            let m362 = f73(v1, v64, f74(v65)?)?;
                            match m362 {
                                Ok(_) => {
                                    Ok(Ok::<Adt28, Bytes>(v73))
                                }
                                Err(v75) => {
                                    Ok(Err::<Adt28, Bytes>(v75))
                                }
                            }
                        }
                        Err(v76) => {
                            Ok(Err::<Adt28, Bytes>(v76))
                        }
                    }
                }
            }
        }
        Adt17::C6(r444, r445, v79) => {
            let v77 = (*r444).clone();
            let v78 = (*r445).clone();
            let v80: bool = if v3 {
                if match v1.clone() {
                    Adt27::C0(_, _, _, _, h363) => {
                        h363
                    }
                } {
                    false
                } else {
                    match v1.clone() {
                        Adt27::C0(_, _, h364, _, _) => {
                            h364
                        }
                    }
                }
            } else {
                false
            };
            let m365 = v80;
            if m365 {
                Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("arithmetic: the value a fallible function returns, ")), f4(f61(match v1 {
                    Adt27::C0(_, _, _, h366, _) => {
                        h366
                    }
                }, Adt17::C6(std::rc::Rc::new(v77), std::rc::Rc::new(v78), v79))?, f1(Str::lit(", is not a fallible value"))))))
            } else {
                let m367 = f47(v0, v1.clone(), v77, false)?;
                match m367 {
                    Ok(v81) => {
                        f47(v81, v1, v78, false)
                    }
                    Err(v82) => {
                        Ok(Err::<Adt28, Bytes>(v82))
                    }
                }
            }
        }
        Adt17::C7(r446, r447, r448, _) => {
            let v83 = (*r446).clone();
            let v84 = (*r447).clone();
            let v85 = (*r448).clone();
            let m368 = f47(v0, v1.clone(), v83, false)?;
            match m368 {
                Ok(v87) => {
                    let v88: Adt28 = Adt28::C0(match v87.clone() {
                        Adt28::C0(h369, _, _, _, _) => {
                            h369
                        }
                    }, match v87.clone() {
                        Adt28::C0(_, h370, _, _, _) => {
                            h370
                        }
                    }, match v87.clone() {
                        Adt28::C0(_, _, h371, _, _) => {
                            h371
                        }
                    }, match v87.clone() {
                        Adt28::C0(_, _, _, h372, _) => {
                            h372
                        }
                    }, match v87.clone() {
                        Adt28::C0(_, _, _, _, h373) => {
                            h373
                        }
                    });
                    let m375 = f45(v88, match v1.clone() {
                        Adt27::C0(_, h374, _, _, _) => {
                            h374
                        }
                    }, Adt14::C0(Adt15::C0))?;
                    match m375 {
                        Ok(v89) => {
                            let m376 = f29(v89, v1.clone(), v84, v3)?;
                            match m376 {
                                Ok(v90) => {
                                    let v91: Adt28 = Adt28::C0(match v90.clone() {
                                        Adt28::C0(h377, _, _, _, _) => {
                                            h377
                                        }
                                    }, match v87.clone() {
                                        Adt28::C0(_, h378, _, _, _) => {
                                            h378
                                        }
                                    }, f75(match v87.clone() {
                                        Adt28::C0(_, _, h379, _, _) => {
                                            h379
                                        }
                                    }, match v90.clone() {
                                        Adt28::C0(_, _, h380, _, _) => {
                                            h380
                                        }
                                    }), f76(match v87.clone() {
                                        Adt28::C0(_, _, _, h381, _) => {
                                            h381
                                        }
                                    }, match v90.clone() {
                                        Adt28::C0(_, _, _, h382, _) => {
                                            h382
                                        }
                                    }), match v90 {
                                        Adt28::C0(_, _, _, _, h383) => {
                                            h383
                                        }
                                    });
                                    let v92: Adt28 = Adt28::C0(match v91.clone() {
                                        Adt28::C0(h384, _, _, _, _) => {
                                            h384
                                        }
                                    }, match v87.clone() {
                                        Adt28::C0(_, h385, _, _, _) => {
                                            h385
                                        }
                                    }, match v87.clone() {
                                        Adt28::C0(_, _, h386, _, _) => {
                                            h386
                                        }
                                    }, match v87.clone() {
                                        Adt28::C0(_, _, _, h387, _) => {
                                            h387
                                        }
                                    }, match v91.clone() {
                                        Adt28::C0(_, _, _, _, h388) => {
                                            h388
                                        }
                                    });
                                    let m390 = f45(v92, match v1.clone() {
                                        Adt27::C0(_, h389, _, _, _) => {
                                            h389
                                        }
                                    }, Adt14::C0(Adt15::C0))?;
                                    match m390 {
                                        Ok(v93) => {
                                            let m391 = f29(v93, v1, v85, v3)?;
                                            match m391 {
                                                Ok(v94) => {
                                                    let v95: Adt28 = Adt28::C0(match v94.clone() {
                                                        Adt28::C0(h392, _, _, _, _) => {
                                                            h392
                                                        }
                                                    }, match v87 {
                                                        Adt28::C0(_, h393, _, _, _) => {
                                                            h393
                                                        }
                                                    }, f75(match v91.clone() {
                                                        Adt28::C0(_, _, h394, _, _) => {
                                                            h394
                                                        }
                                                    }, match v94.clone() {
                                                        Adt28::C0(_, _, h395, _, _) => {
                                                            h395
                                                        }
                                                    }), f76(match v91 {
                                                        Adt28::C0(_, _, _, h396, _) => {
                                                            h396
                                                        }
                                                    }, match v94.clone() {
                                                        Adt28::C0(_, _, _, h397, _) => {
                                                            h397
                                                        }
                                                    }), match v94 {
                                                        Adt28::C0(_, _, _, _, h398) => {
                                                            h398
                                                        }
                                                    });
                                                    Ok(Ok::<Adt28, Bytes>(v95))
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
                                Err(v98) => {
                                    Ok(Err::<Adt28, Bytes>(v98))
                                }
                            }
                        }
                        Err(v99) => {
                            Ok(Err::<Adt28, Bytes>(v99))
                        }
                    }
                }
                Err(v100) => {
                    Ok(Err::<Adt28, Bytes>(v100))
                }
            }
        }
        Adt17::C8(r449, r450, _) => {
            let v101 = (*r449).clone();
            let v102 = (*r450).clone();
            let m399 = f47(v0, v1.clone(), v101, false)?;
            match m399 {
                Ok(v104) => {
                    f77(v104.clone(), v1, v104.clone(), match v104 {
                        Adt28::C0(_, h400, _, _, _) => {
                            h400
                        }
                    }, v102, v3)
                }
                Err(v105) => {
                    Ok(Err::<Adt28, Bytes>(v105))
                }
            }
        }
        Adt17::C9(r451) => {
            let v106 = (*r451).clone();
            f29(v0, v1, v106, v3)
        }
        Adt17::C10(v107, v108, v109) => {
            let v110: bool = if v3 {
                if match v1.clone() {
                    Adt27::C0(_, _, _, _, h401) => {
                        h401
                    }
                } {
                    false
                } else {
                    match v1.clone() {
                        Adt27::C0(_, _, h402, _, _) => {
                            h402
                        }
                    }
                }
            } else {
                false
            };
            let m403 = v110;
            if m403 {
                Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("arithmetic: the value a fallible function returns, ")), f4(f61(match v1 {
                    Adt27::C0(_, _, _, h404, _) => {
                        h404
                    }
                }, Adt17::C10(v107, v108, v109))?, f1(Str::lit(", is not a fallible value"))))))
            } else {
                let m405 = v107;
                match m405 {
                    Adt24::C0 => {
                        let m406 = f64(v0.clone(), v108)?;
                        match m406 {
                            Ok(_) => {
                                Ok(Ok::<Adt28, Bytes>(v0))
                            }
                            Err(v112) => {
                                Ok(Err::<Adt28, Bytes>(v112))
                            }
                        }
                    }
                    Adt24::C1 => {
                        let m407 = f64(v0.clone(), v108)?;
                        match m407 {
                            Ok(_) => {
                                Ok(Ok::<Adt28, Bytes>(v0))
                            }
                            Err(v114) => {
                                Ok(Err::<Adt28, Bytes>(v114))
                            }
                        }
                    }
                    Adt24::C2 => {
                        let m408 = f44(v1, f1(Str::lit("a list match")));
                        match m408 {
                            Ok(_) => {
                                let m409 = f64(v0.clone(), v108)?;
                                match m409 {
                                    Ok(_) => {
                                        Ok(Ok::<Adt28, Bytes>(v0))
                                    }
                                    Err(v117) => {
                                        Ok(Err::<Adt28, Bytes>(v117))
                                    }
                                }
                            }
                            Err(v118) => {
                                Ok(Err::<Adt28, Bytes>(v118))
                            }
                        }
                    }
                    Adt24::C3 => {
                        let m410 = f64(v0.clone(), v108)?;
                        match m410 {
                            Ok(_) => {
                                Ok(Ok::<Adt28, Bytes>(v0))
                            }
                            Err(v120) => {
                                Ok(Err::<Adt28, Bytes>(v120))
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
        Adt17::C16(v134, r452) => {
            let v135 = (*r452).clone();
            let v136: bool = if v3 {
                if match v1.clone() {
                    Adt27::C0(_, _, _, _, h411) => {
                        h411
                    }
                } {
                    false
                } else {
                    match v1.clone() {
                        Adt27::C0(_, _, h412, _, _) => {
                            h412
                        }
                    }
                }
            } else {
                false
            };
            let v137: Result<(), Bytes> = {
                let m413 = v136;
                if m413 {
                    f78(match v1.clone() {
                        Adt27::C0(_, _, _, h414, _) => {
                            h414
                        }
                    }, v135.clone())?
                } else {
                    Ok::<(), Bytes>(())
                }
            };
            let m415 = v137;
            match m415 {
                Ok(_) => {
                    let m416 = f63(v0, v134.clone())?;
                    match m416 {
                        Ok(v139) => {
                            let v140: Adt28 = Adt28::C0(match v139.clone() {
                                Adt28::C0(h417, _, _, _, _) => {
                                    h417
                                }
                            }, match v139.clone() {
                                Adt28::C0(_, h418, _, _, _) => {
                                    h418
                                }
                            }, f79(match v139.clone() {
                                Adt28::C0(_, _, h419, _, _) => {
                                    h419
                                }
                            }, v134.clone()), match v139.clone() {
                                Adt28::C0(_, _, _, h420, _) => {
                                    h420
                                }
                            }, match v139.clone() {
                                Adt28::C0(_, _, _, _, h421) => {
                                    h421
                                }
                            });
                            let m426 = f29(v140.clone(), Adt27::C0(match v1.clone() {
                                Adt27::C0(h422, _, _, _, _) => {
                                    h422
                                }
                            }, match v1.clone() {
                                Adt27::C0(_, h423, _, _, _) => {
                                    h423
                                }
                            }, match v1.clone() {
                                Adt27::C0(_, _, h424, _, _) => {
                                    h424
                                }
                            }, match v1 {
                                Adt27::C0(_, _, _, h425, _) => {
                                    h425
                                }
                            }, v136), v135, v136)?;
                            match m426 {
                                Ok(v141) => {
                                    let m430 = f80(match v141.clone() {
                                        Adt28::C0(_, _, h427, _, _) => {
                                            h427
                                        }
                                    }, v134, match v140.clone() {
                                        Adt28::C0(_, _, h428, _, _) => {
                                            h428
                                        }
                                    }, match v139.clone() {
                                        Adt28::C0(_, h429, _, _, _) => {
                                            h429
                                        }
                                    });
                                    match m430 {
                                        None => {
                                            let m434 = f81(match v141.clone() {
                                                Adt28::C0(_, _, _, h431, _) => {
                                                    h431
                                                }
                                            }, match v140 {
                                                Adt28::C0(_, _, _, h432, _) => {
                                                    h432
                                                }
                                            }, match v139.clone() {
                                                Adt28::C0(_, h433, _, _, _) => {
                                                    h433
                                                }
                                            });
                                            match m434 {
                                                None => {
                                                    Ok(Ok::<Adt28, Bytes>(Adt28::C0(match v141.clone() {
                                                        Adt28::C0(h435, _, _, _, _) => {
                                                            h435
                                                        }
                                                    }, match v141.clone() {
                                                        Adt28::C0(_, h436, _, _, _) => {
                                                            h436
                                                        }
                                                    }, match v139.clone() {
                                                        Adt28::C0(_, _, h437, _, _) => {
                                                            h437
                                                        }
                                                    }, match v139 {
                                                        Adt28::C0(_, _, _, h438, _) => {
                                                            h438
                                                        }
                                                    }, match v141 {
                                                        Adt28::C0(_, _, _, _, h439) => {
                                                            h439
                                                        }
                                                    })))
                                                }
                                                Some(v143) => {
                                                    Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("ownership: a component of ")), f4(f30(f31(v143)?), f1(Str::lit(" is moved in the body of a loop that runs again"))))))
                                                }
                                            }
                                        }
                                        Some(v142) => {
                                            Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("ownership: ")), f4(f30(f31(v142)?), f1(Str::lit(" is moved in the body of a loop that runs again"))))))
                                        }
                                    }
                                }
                                Err(v144) => {
                                    Ok(Err::<Adt28, Bytes>(v144))
                                }
                            }
                        }
                        Err(v145) => {
                            Ok(Err::<Adt28, Bytes>(v145))
                        }
                    }
                }
                Err(v146) => {
                    Ok(Err::<Adt28, Bytes>(v146))
                }
            }
        }
    }
}

pub fn f48(p459: List<(Adt12, u64)>, v1: Adt26, p460: Adt17) -> R<Result<List<(Adt12, u64)>, Bytes>> {
    let m458 = (p459, p460);
    {
        let mut m458 = m458;
        loop {
            let step = {
                let (v0, v2) = m458;
                let m453 = v2;
                match m453 {
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
                    Adt17::C14(v40, v41, v42, r456, _, r457) => {
                        let v43 = (*r456).clone();
                        let v45 = (*r457).clone();
                        let v46: List<(Adt12, u64)> = {
                            let m454 = v41;
                            if m454 {
                                f50(v0, v40, v42)
                            } else {
                                v0
                            }
                        };
                        let m455 = f49(v46, v1.clone(), v43, false)?;
                        match m455 {
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
                Ok(next) => { m458 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f49(v0: List<(Adt12, u64)>, v1: Adt26, v2: Adt17, v3: bool) -> R<Result<List<(Adt12, u64)>, Bytes>> {
    let m461 = v2;
    match m461 {
        Adt17::C0(_, _) => {
            Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0))
        }
        Adt17::C1(_, _, _) => {
            Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0))
        }
        Adt17::C2(_, r471, _) => {
            let v10 = (*r471).clone();
            f49(v0, v1, v10, false)
        }
        Adt17::C3(_, r472, _, _) => {
            let v13 = (*r472).clone();
            f82(v0, v1, v13)
        }
        Adt17::C4(v16, r473, v18, _) => {
            let v17 = (*r473).clone();
            let m462 = f83(v0.clone(), v16.clone());
            match m462 {
                None => {
                    Ok(Err::<List<(Adt12, u64)>, Bytes>(f4(f30(f31(v16)?), f1(Str::lit(" is applied without a function type")))))
                }
                Some(v20) => {
                    let m464 = f20(match v1.clone() {
                        Adt26::C0(_, _, h463, _) => {
                            h463
                        }
                    }, v20);
                    match m464 {
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
                            let m465 = v22;
                            if m465 {
                                let m466 = v23;
                                if m466 {
                                    f82(v0, v1, v17)
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
                                f82(v0, v1, v17)
                            }
                        }
                    }
                }
            }
        }
        Adt17::C5(_, r474, _) => {
            let v25 = (*r474).clone();
            f82(v0, v1, v25)
        }
        Adt17::C6(r475, r476, _) => {
            let v27 = (*r475).clone();
            let v28 = (*r476).clone();
            let m467 = f49(v0, v1.clone(), v27, false)?;
            match m467 {
                Ok(v30) => {
                    f49(v30, v1, v28, false)
                }
                Err(v31) => {
                    Ok(Err::<List<(Adt12, u64)>, Bytes>(v31))
                }
            }
        }
        Adt17::C7(r477, r478, r479, _) => {
            let v32 = (*r477).clone();
            let v33 = (*r478).clone();
            let v34 = (*r479).clone();
            let m468 = f49(v0, v1.clone(), v32, false)?;
            match m468 {
                Ok(v36) => {
                    let m469 = f32(v36, v1.clone(), v33, v3)?;
                    match m469 {
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
        Adt17::C8(r480, r481, _) => {
            let v40 = (*r480).clone();
            let v41 = (*r481).clone();
            let m470 = f49(v0, v1.clone(), v40, false)?;
            match m470 {
                Ok(v43) => {
                    f84(v43, v1, v41, v3)
                }
                Err(v44) => {
                    Ok(Err::<List<(Adt12, u64)>, Bytes>(v44))
                }
            }
        }
        Adt17::C9(r482) => {
            let v45 = (*r482).clone();
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
        Adt17::C16(_, r483) => {
            let v63 = (*r483).clone();
            f32(v0, v1, v63, false)
        }
    }
}

pub fn f50(v0: List<(Adt12, u64)>, v1: Adt14, v2: Adt2) -> List<(Adt12, u64)> {
    let m484 = f85(v1);
    match m484 {
        None => {
            v0
        }
        Some(v3) => {
            let m485 = f86(v2);
            match m485 {
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
    if f87(v0, v1, 48u8) {
        Ok(Some({
            let a486 = v1;
            let a487 = 1u64;
            nat_add(a486, a487)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f52(v0: Adt3) -> bool {
    let m488 = v0;
    match m488 {
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
    }
}

pub fn f53(v0: Adt5) -> bool {
    let m489 = v0;
    match m489 {
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

pub fn f54(v0: Fn3, p493: bool, p494: List<Adt11>) -> bool {
    let m492 = (p493, p494);
    {
        let mut m492 = m492;
        loop {
            let step = {
                let (v1, v2) = m492;
                let m490 = v2;
                match m490.uncons() {
                    None => {
                        Err::<(bool, List<Adt11>), bool>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<Adt11>), bool>(({
                            let c491: Fn3 = v0.clone();
                            c491.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m492 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f55(v0: bool, v1: Adt11) -> bool {
    let m495 = v1;
    match m495 {
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
    f88(Fn4::F89(v1), false, v0)
}

pub fn f57(v0: Adt26, v1: u64, v2: u64) -> Option<u64> {
    let m497 = f18(match v0 {
        Adt26::C0(h496, _, _, _) => {
            h496
        }
    }, v1);
    match m497 {
        None => {
            None::<u64>
        }
        Some(v3) => {
            f90(v3, v2)
        }
    }
}

pub fn f58(v0: Adt14) -> R<u64> {
    let m498 = v0;
    match m498 {
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
        Adt14::C6(_, r501) => {
            let v10 = (*r501).clone();
            let v11: u64 = f58(v10)?;
            let a499 = 1u64;
            let a500 = v11;
            nat_add(a499, a500)
        }
    }
}

pub fn f59(v0: Adt14, v1: Adt17) -> Option<Adt12> {
    let m502 = v0;
    match m502 {
        Adt14::C0(_) => {
            None::<Adt12>
        }
        Adt14::C1(_) => {
            None::<Adt12>
        }
        Adt14::C2(r506) => {
            let v4 = (*r506).clone();
            let m503 = v1;
            match m503 {
                Adt17::C0(_, _) => {
                    None::<Adt12>
                }
                Adt17::C1(v7, v8, _) => {
                    let m504 = f91(v7);
                    if m504 {
                        let m505 = f92(v4);
                        if m505 {
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
    let m508 = f56(match v0.clone() {
        Adt28::C0(_, h507, _, _, _) => {
            h507
        }
    }, v1.clone());
    if m508 {
        let m510 = f56(match v0.clone() {
            Adt28::C0(_, _, h509, _, _) => {
                h509
            }
        }, v1.clone());
        if m510 {
            Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("ownership: a component of ")), f4(f30(f31(v1)?), f1(Str::lit(" is moved after it was moved"))))))
        } else {
            let v4: List<u64> = {
                let m512 = f93(match v0.clone() {
                    Adt28::C0(_, _, _, h511, _) => {
                        h511
                    }
                }, v1.clone());
                match m512 {
                    None => {
                        List::<u64>::nil()
                    }
                    Some(v3) => {
                        v3
                    }
                }
            };
            let m513 = f94(v4, 0u64, v2, f30(f31(v1.clone())?))?;
            match m513 {
                Ok(v5) => {
                    Ok(Ok::<Adt28, Bytes>(Adt28::C0(match v0.clone() {
                        Adt28::C0(h514, _, _, _, _) => {
                            h514
                        }
                    }, match v0.clone() {
                        Adt28::C0(_, h515, _, _, _) => {
                            h515
                        }
                    }, match v0.clone() {
                        Adt28::C0(_, _, h516, _, _) => {
                            h516
                        }
                    }, f95(match v0.clone() {
                        Adt28::C0(_, _, _, h517, _) => {
                            h517
                        }
                    }, v1, v5), match v0 {
                        Adt28::C0(_, _, _, _, h518) => {
                            h518
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
    Ok(f96({
        let m520 = f97(Adt30::C0(List::<Adt29>::nil(), List::<Bytes>::nil(), v0), v1)?;
        match m520 {
            Adt30::C0(h519, _, _) => {
                h519
            }
        }
    }))
}

pub fn f62(v0: Adt27, v1: Adt28, v2: Bytes) -> Result<Adt28, Bytes> {
    let m522 = match v0 {
        Adt27::C0(h521, _, _, _, _) => {
            h521
        }
    };
    if m522 {
        Err::<Adt28, Bytes>(f4(f1(Str::lit("hidden allocation: rust-core renders ")), f4(v2, f1(Str::lit(", which requires heap allocation")))))
    } else {
        Ok::<Adt28, Bytes>(v1)
    }
}

pub fn f63(v0: Adt28, v1: Adt12) -> R<Result<Adt28, Bytes>> {
    let m524 = f56(match v0.clone() {
        Adt28::C0(_, h523, _, _, _) => {
            h523
        }
    }, v1.clone());
    if m524 {
        let m526 = f98(match v0.clone() {
            Adt28::C0(_, _, _, h525, _) => {
                h525
            }
        }, v1.clone());
        if m526 {
            Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("ownership: ")), f4(f30(f31(v1)?), f1(Str::lit(" is moved after a component was moved out of it"))))))
        } else {
            let m528 = f56(match v0.clone() {
                Adt28::C0(_, _, h527, _, _) => {
                    h527
                }
            }, v1.clone());
            if m528 {
                Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("ownership: ")), f4(f30(f31(v1)?), f1(Str::lit(" is moved twice"))))))
            } else {
                Ok(Ok::<Adt28, Bytes>(Adt28::C0(match v0.clone() {
                    Adt28::C0(h529, _, _, _, _) => {
                        h529
                    }
                }, match v0.clone() {
                    Adt28::C0(_, h530, _, _, _) => {
                        h530
                    }
                }, List::cons(v1, match v0.clone() {
                    Adt28::C0(_, _, h531, _, _) => {
                        h531
                    }
                }), match v0.clone() {
                    Adt28::C0(_, _, _, h532, _) => {
                        h532
                    }
                }, match v0 {
                    Adt28::C0(_, _, _, _, h533) => {
                        h533
                    }
                })))
            }
        }
    } else {
        Ok(Err::<Adt28, Bytes>(f4(f30(f31(v1)?), f1(Str::lit(" is not bound")))))
    }
}

pub fn f64(v0: Adt28, v1: Adt12) -> R<Result<(), Bytes>> {
    let m535 = f56(match v0.clone() {
        Adt28::C0(_, h534, _, _, _) => {
            h534
        }
    }, v1.clone());
    if m535 {
        let m537 = f56(match v0.clone() {
            Adt28::C0(_, _, h536, _, _) => {
                h536
            }
        }, v1.clone());
        if m537 {
            Ok(Err::<(), Bytes>(f4(f1(Str::lit("ownership: ")), f4(f30(f31(v1)?), f1(Str::lit(" is read after it was moved"))))))
        } else {
            let m539 = f98(match v0 {
                Adt28::C0(_, _, _, h538, _) => {
                    h538
                }
            }, v1.clone());
            if m539 {
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
    let m540 = v0;
    match m540 {
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
            f99(v31)
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

pub fn f66(p546: Adt28, v1: Adt27, p547: Adt17) -> R<Result<Adt28, Bytes>> {
    let m545 = (p546, p547);
    {
        let mut m545 = m545;
        loop {
            let step = {
                let (v0, v2) = m545;
                let m541 = v2;
                match m541 {
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
                    Adt17::C13(r543, r544) => {
                        let v38 = (*r543).clone();
                        let v39 = (*r544).clone();
                        let m542 = f47(v0, v1.clone(), v38, false)?;
                        match m542 {
                            Ok(v40) => {
                                Ok::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>((v40, v39))
                            }
                            Err(v41) => {
                                Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Err::<Adt28, Bytes>(v41)))
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
                Ok(next) => { m545 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f67(v0: Adt28, v1: Adt27, v2: bool, v3: bool, v4: bool, v5: Bytes) -> Result<Adt28, Bytes> {
    let v6: bool = if v4 {
        match v1 {
            Adt27::C0(_, _, h548, _, _) => {
                h548
            }
        }
    } else {
        false
    };
    let m549 = v2;
    if m549 {
        let m550 = v3;
        if m550 {
            Ok::<Adt28, Bytes>(Adt28::C0(match v0.clone() {
                Adt28::C0(h551, _, _, _, _) => {
                    h551
                }
            }, match v0.clone() {
                Adt28::C0(_, h552, _, _, _) => {
                    h552
                }
            }, match v0.clone() {
                Adt28::C0(_, _, h553, _, _) => {
                    h553
                }
            }, match v0 {
                Adt28::C0(_, _, _, h554, _) => {
                    h554
                }
            }, true))
        } else {
            let m555 = v6;
            if m555 {
                Ok::<Adt28, Bytes>(v0)
            } else {
                Err::<Adt28, Bytes>(f4(f1(Str::lit("arithmetic: the fallible call of ")), f4(v5, f1(Str::lit(" does not propagate its overflow")))))
            }
        }
    } else {
        let m556 = v3;
        if m556 {
            Err::<Adt28, Bytes>(f4(f1(Str::lit("arithmetic: the infallible call of ")), f4(v5, f1(Str::lit(" propagates a failure it cannot have")))))
        } else {
            Ok::<Adt28, Bytes>(v0)
        }
    }
}

pub fn f68(v0: List<Adt25>, v1: Adt22) -> Bytes {
    let m558 = f100(v0, match v1.clone() {
        Adt22::C0(h557, _) => {
            h557
        }
    });
    match m558 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            let a560 = match v2.clone() {
                Adt25::C0(_, _, h559, _, _) => {
                    h559
                }
            };
            let a561 = 0u64;
            if nat_eq(a560, a561) {
                match v2 {
                    Adt25::C0(_, h562, _, _, _) => {
                        h562
                    }
                }
            } else {
                let a564 = match v2.clone() {
                    Adt25::C0(_, _, h563, _, _) => {
                        h563
                    }
                };
                let a565 = 1u64;
                if nat_eq(a564, a565) {
                    f4(f1(Str::lit("fixed_")), f4(f101(f102(match v1 {
                        Adt22::C0(_, h566) => {
                            h566
                        }
                    })), f4(f1(Str::lit("::")), match v2 {
                        Adt25::C0(_, h567, _, _, _) => {
                            h567
                        }
                    })))
                } else {
                    f4(match v2 {
                        Adt25::C0(_, h568, _, _, _) => {
                            h568
                        }
                    }, f101(f102(match v1 {
                        Adt22::C0(_, h569) => {
                            h569
                        }
                    })))
                }
            }
        }
    }
}

pub fn f69(v0: List<Adt25>, v1: Adt22) -> bool {
    let m571 = f100(v0, match v1 {
        Adt22::C0(h570, _) => {
            h570
        }
    });
    match m571 {
        None => {
            false
        }
        Some(v2) => {
            match v2 {
                Adt25::C0(_, _, _, _, h572) => {
                    h572
                }
            }
        }
    }
}

pub fn f70(v0: List<Adt25>, v1: Adt22) -> bool {
    let m574 = f100(v0, match v1 {
        Adt22::C0(h573, _) => {
            h573
        }
    });
    match m574 {
        None => {
            false
        }
        Some(v2) => {
            match v2 {
                Adt25::C0(_, _, _, h575, _) => {
                    h575
                }
            }
        }
    }
}

pub fn f71(v0: Adt23) -> bool {
    let m576 = v0;
    match m576 {
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

pub fn f72(p582: Adt28, v1: Adt27, p583: Adt17) -> R<Result<Adt28, Bytes>> {
    let m581 = (p582, p583);
    {
        let mut m581 = m581;
        loop {
            let step = {
                let (v0, v2) = m581;
                let m577 = v2;
                match m577 {
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
                    Adt17::C13(r579, r580) => {
                        let v38 = (*r579).clone();
                        let v39 = (*r580).clone();
                        let m578 = f47(v0, v1.clone(), v38, true)?;
                        match m578 {
                            Ok(v40) => {
                                Ok::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>((v40, v39))
                            }
                            Err(v41) => {
                                Err::<(Adt28, Adt17), R<Result<Adt28, Bytes>>>(Ok(Err::<Adt28, Bytes>(v41)))
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
                Ok(next) => { m581 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f73(v0: Adt27, v1: Adt23, v2: u64) -> R<Result<(), Bytes>> {
    let m584 = v1;
    match m584 {
        Adt23::C0(v3) => {
            f26(v0, v3)
        }
        Adt23::C1 => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt23::C2(v4, v5) => {
            let m585 = f26(v0.clone(), v4)?;
            match m585 {
                Ok(_) => {
                    f26(v0, v5)
                }
                Err(v7) => {
                    Ok(Err::<(), Bytes>(v7))
                }
            }
        }
        Adt23::C3(v8, v9) => {
            let m586 = f26(v0.clone(), v8)?;
            match m586 {
                Ok(_) => {
                    f26(v0, v9)
                }
                Err(v11) => {
                    Ok(Err::<(), Bytes>(v11))
                }
            }
        }
        Adt23::C4(v12, v13) => {
            let m588 = f57(match v0 {
                Adt27::C0(_, h587, _, _, _) => {
                    h587
                }
            }, v12, v13);
            match m588 {
                None => {
                    Ok(Err::<(), Bytes>(f4(f1(Str::lit("Adt")), f4(f10(v12)?, f4(f1(Str::lit("::C")), f4(f10(v13)?, f1(Str::lit(" is not declared"))))))))
                }
                Some(v14) => {
                    let m591 = {
                        let a589 = v14;
                        let a590 = v2;
                        nat_eq(a589, a590)
                    };
                    if m591 {
                        Ok(Ok::<(), Bytes>(()))
                    } else {
                        Ok(Err::<(), Bytes>(f4(f1(Str::lit("Adt")), f4(f10(v12)?, f4(f1(Str::lit("::C")), f4(f10(v13)?, f4(f1(Str::lit(" receives ")), f4(f10(v2)?, f4(f1(Str::lit(" fields of ")), f10(v14)?)))))))))
                    }
                }
            }
        }
        Adt23::C5(v15, v16) => {
            let m593 = f103(match v0 {
                Adt27::C0(_, h592, _, _, _) => {
                    h592
                }
            }, v15, v16);
            match m593 {
                None => {
                    Ok(Err::<(), Bytes>(f4(f1(Str::lit("Fn")), f4(f10(v15)?, f4(f1(Str::lit("::F")), f4(f10(v16)?, f1(Str::lit(" is not declared"))))))))
                }
                Some(v17) => {
                    let m596 = {
                        let a594 = v17;
                        let a595 = v2;
                        nat_eq(a594, a595)
                    };
                    if m596 {
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
            let m597 = f44(v0.clone(), f1(Str::lit("an empty list")));
            match m597 {
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

pub fn f74(v0: Adt17) -> R<u64> {
    let m598 = v0;
    match m598 {
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
        Adt17::C13(_, r601) => {
            let v37 = (*r601).clone();
            let v38: u64 = f74(v37)?;
            let a599 = 1u64;
            let a600 = v38;
            nat_add(a599, a600)
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

pub fn f75(v0: List<Adt12>, v1: List<Adt12>) -> List<Adt12> {
    f104(Fn5::F105, v0, v1)
}

pub fn f76(v0: List<(Adt12, List<u64>)>, v1: List<(Adt12, List<u64>)>) -> List<(Adt12, List<u64>)> {
    f106(Fn6::F107, v0, v1)
}

pub fn f77(v0: Adt28, v1: Adt27, p619: Adt28, v3: List<Adt12>, p620: Adt17, v5: bool) -> R<Result<Adt28, Bytes>> {
    let m618 = (p619, p620);
    {
        let mut m618 = m618;
        loop {
            let step = {
                let (v2, v4) = m618;
                let m602 = v4;
                match m602 {
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
                    Adt17::C15(v49, r616, r617) => {
                        let v50 = (*r616).clone();
                        let v51 = (*r617).clone();
                        let v52: Adt28 = Adt28::C0(match v2.clone() {
                            Adt28::C0(h603, _, _, _, _) => {
                                h603
                            }
                        }, v3.clone(), match v0.clone() {
                            Adt28::C0(_, _, h604, _, _) => {
                                h604
                            }
                        }, match v0.clone() {
                            Adt28::C0(_, _, _, h605, _) => {
                                h605
                            }
                        }, match v2.clone() {
                            Adt28::C0(_, _, _, _, h606) => {
                                h606
                            }
                        });
                        let m608 = f45(v52, match v1.clone() {
                            Adt27::C0(_, h607, _, _, _) => {
                                h607
                            }
                        }, v49)?;
                        match m608 {
                            Ok(v53) => {
                                let m609 = f29(v53, v1.clone(), v50, v5)?;
                                match m609 {
                                    Ok(v54) => {
                                        let v55: Adt28 = Adt28::C0(match v54.clone() {
                                            Adt28::C0(h610, _, _, _, _) => {
                                                h610
                                            }
                                        }, v3.clone(), f75(match v2.clone() {
                                            Adt28::C0(_, _, h611, _, _) => {
                                                h611
                                            }
                                        }, match v54.clone() {
                                            Adt28::C0(_, _, h612, _, _) => {
                                                h612
                                            }
                                        }), f76(match v2 {
                                            Adt28::C0(_, _, _, h613, _) => {
                                                h613
                                            }
                                        }, match v54.clone() {
                                            Adt28::C0(_, _, _, h614, _) => {
                                                h614
                                            }
                                        }), match v54 {
                                            Adt28::C0(_, _, _, _, h615) => {
                                                h615
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
                Ok(next) => { m618 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f78(v0: List<Adt25>, v1: Adt17) -> R<Result<(), Bytes>> {
    let m621 = v1;
    match m621 {
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
        Adt17::C11(_, r622) => {
            let v36 = (*r622).clone();
            f108(v0, v36)
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

pub fn f79(v0: List<Adt12>, v1: Adt12) -> List<Adt12> {
    f104(Fn5::F109(v1), List::<Adt12>::nil(), v0)
}

pub fn f80(v0: List<Adt12>, v1: Adt12, v2: List<Adt12>, v3: List<Adt12>) -> Option<Adt12> {
    f110(Fn7::F111(v2, v3, v1), None::<Adt12>, v0)
}

pub fn f81(v0: List<(Adt12, List<u64>)>, v1: List<(Adt12, List<u64>)>, v2: List<Adt12>) -> Option<Adt12> {
    f112(Fn8::F113(v1, v2), None::<Adt12>, v0)
}

pub fn f82(p628: List<(Adt12, u64)>, v1: Adt26, p629: Adt17) -> R<Result<List<(Adt12, u64)>, Bytes>> {
    let m627 = (p628, p629);
    {
        let mut m627 = m627;
        loop {
            let step = {
                let (v0, v2) = m627;
                let m623 = v2;
                match m623 {
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
                    Adt17::C13(r625, r626) => {
                        let v38 = (*r625).clone();
                        let v39 = (*r626).clone();
                        let m624 = f49(v0, v1.clone(), v38, false)?;
                        match m624 {
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
                Ok(next) => { m627 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f83(v0: List<(Adt12, u64)>, v1: Adt12) -> Option<u64> {
    f114(Fn9::F115(v1), None::<u64>, v0)
}

pub fn f84(p635: List<(Adt12, u64)>, v1: Adt26, p636: Adt17, v3: bool) -> R<Result<List<(Adt12, u64)>, Bytes>> {
    let m634 = (p635, p636);
    {
        let mut m634 = m634;
        loop {
            let step = {
                let (v0, v2) = m634;
                let m630 = v2;
                match m630 {
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
                    Adt17::C15(_, r632, r633) => {
                        let v48 = (*r632).clone();
                        let v49 = (*r633).clone();
                        let m631 = f32(v0, v1.clone(), v48, v3)?;
                        match m631 {
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
                Ok(next) => { m634 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f85(v0: Adt14) -> Option<Adt12> {
    let m637 = v0;
    match m637 {
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

pub fn f86(v0: Adt2) -> Option<u64> {
    let m638 = v0;
    match m638 {
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
            let m639 = f6(v8);
            if m639 {
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

pub fn f87(v0: Bytes, v1: u64, v2: u8) -> bool {
    let m640 = f116(v0, v1);
    match m640 {
        None => {
            false
        }
        Some(v3) => {
            let a641 = v3;
            let a642 = v2;
            equal(a641, a642)
        }
    }
}

pub fn f88(v0: Fn4, p646: bool, p647: List<Adt12>) -> bool {
    let m645 = (p646, p647);
    {
        let mut m645 = m645;
        loop {
            let step = {
                let (v1, v2) = m645;
                let m643 = v2;
                match m643.uncons() {
                    None => {
                        Err::<(bool, List<Adt12>), bool>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<Adt12>), bool>(({
                            let c644: Fn4 = v0.clone();
                            c644.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m645 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f89(v0: Adt12, v1: bool, v2: Adt12) -> bool {
    let v3: bool = f117(v2, v0);
    if v1 {
        true
    } else {
        v3
    }
}

pub fn f90(v0: List<(u64, u64)>, v1: u64) -> Option<u64> {
    f118(Fn10::F119(v1), None::<u64>, v0)
}

pub fn f91(v0: Adt19) -> bool {
    let m648 = v0;
    match m648 {
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

pub fn f92(v0: Adt14) -> bool {
    let m649 = v0;
    match m649 {
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
        Adt14::C6(r650, r651) => {
            let v9 = (*r650).clone();
            let v10 = (*r651).clone();
            let v11: bool = f120(v9);
            let v12: bool = f92(v10);
            if v11 {
                true
            } else {
                v12
            }
        }
    }
}

pub fn f93(v0: List<(Adt12, List<u64>)>, v1: Adt12) -> Option<List<u64>> {
    f121(Fn11::F122(v1), None::<List<u64>>, v0)
}

pub fn f94(p662: List<u64>, p663: u64, p664: Adt14, v3: Bytes) -> R<Result<List<u64>, Bytes>> {
    let m661 = (p662, (p663, p664));
    {
        let mut m661 = m661;
        loop {
            let step = {
                let (v0, (v1, v2)) = m661;
                let m652 = v2;
                match m652 {
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
                    Adt14::C6(r659, r660) => {
                        let v12 = (*r659).clone();
                        let v13 = (*r660).clone();
                        let v14: bool = f120(v12);
                        let m653 = v14;
                        if m653 {
                            Ok::<(List<u64>, (u64, Adt14)), R<Result<List<u64>, Bytes>>>((v0, ({
                                let a654 = v1;
                                let a655 = 1u64;
                                nat_add(a654, a655)?
                            }, v13)))
                        } else {
                            let m656 = f123(v0.clone(), v1);
                            if m656 {
                                Err::<(List<u64>, (u64, Adt14)), R<Result<List<u64>, Bytes>>>(Ok(Err::<List<u64>, Bytes>(f4(f1(Str::lit("ownership: component ")), f4(f10(v1)?, f4(f1(Str::lit(" of ")), f4(v3.clone(), f1(Str::lit(" is moved twice")))))))))
                            } else {
                                Ok::<(List<u64>, (u64, Adt14)), R<Result<List<u64>, Bytes>>>((List::cons(v1, v0), ({
                                    let a657 = v1;
                                    let a658 = 1u64;
                                    nat_add(a657, a658)?
                                }, v13)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m661 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f95(v0: List<(Adt12, List<u64>)>, v1: Adt12, v2: List<u64>) -> List<(Adt12, List<u64>)> {
    List::cons((v1.clone(), v2), f124(v0, v1))
}

pub fn f96(v0: List<Adt29>) -> Bytes {
    let m665 = f125(Fn12::F126, None::<Bytes>, v0);
    match m665 {
        None => {
            f1(Str::lit("a read"))
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f97(v0: Adt30, v1: Adt17) -> R<Adt30> {
    let m666 = v1;
    match m666 {
        Adt17::C0(v2, v3) => {
            Ok(f127(v0, f128(v2.clone()), v3, f129(v2)))
        }
        Adt17::C1(v4, _, v6) => {
            Ok(f130(v0, f131(v4), v6))
        }
        Adt17::C2(v7, r675, v9) => {
            let v8 = (*r675).clone();
            let v10: Adt30 = f130(v0, f132(v7), v9);
            f97(v10, v8)
        }
        Adt17::C3(v11, r676, _, v14) => {
            let v12 = (*r676).clone();
            let v17: Adt30 = {
                let m667 = v11;
                match m667 {
                    Adt21::C0(_) => {
                        f130(v0, f1(Str::lit("call:function")), v14)
                    }
                    Adt21::C1(v16) => {
                        f127(v0.clone(), f4(f1(Str::lit("call:runtime:")), f133(f134(f68(match v0.clone() {
                            Adt30::C0(_, _, h668) => {
                                h668
                            }
                        }, v16.clone()))?)), v14, f135(match v0 {
                            Adt30::C0(_, _, h669) => {
                                h669
                            }
                        }, v16))
                    }
                }
            };
            f97(v17, v12)
        }
        Adt17::C4(_, r677, _, v21) => {
            let v19 = (*r677).clone();
            let v22: Adt30 = f130(v0, f1(Str::lit("apply")), v21);
            f97(v22, v19)
        }
        Adt17::C5(v23, r678, v25) => {
            let v24 = (*r678).clone();
            let v26: Adt30 = f130(f136(v0, v23.clone()), f4(f1(Str::lit("construct:")), f137(v23)), v25);
            f97(v26, v24)
        }
        Adt17::C6(r679, r680, v29) => {
            let v27 = (*r679).clone();
            let v28 = (*r680).clone();
            let v30: Adt30 = f130(v0, f1(Str::lit("construct:pair")), v29);
            let v31: Adt30 = f97(v30, v27)?;
            f97(v31, v28)
        }
        Adt17::C7(r681, r682, r683, v35) => {
            let v32 = (*r681).clone();
            let v33 = (*r682).clone();
            let v34 = (*r683).clone();
            let v36: Adt30 = f130(v0, f1(Str::lit("if")), v35);
            let v37: Adt30 = f97(v36, v32)?;
            let v38: Adt30 = f97(v37, v33)?;
            f97(v38, v34)
        }
        Adt17::C8(r684, r685, v41) => {
            let v39 = (*r684).clone();
            let v40 = (*r685).clone();
            let v42: Adt30 = {
                let m670 = f138(v40.clone());
                if m670 {
                    f130(v0, f1(Str::lit("match:empty")), v41)
                } else {
                    let m671 = f139(v39.clone());
                    if m671 {
                        f130(v0, f1(Str::lit("match:list")), v41)
                    } else {
                        f140(v0, v40.clone(), v41)
                    }
                }
            };
            let v43: Adt30 = f97(v42, v39)?;
            f97(v43, v40)
        }
        Adt17::C9(r686) => {
            let v44 = (*r686).clone();
            f97(v0, v44)
        }
        Adt17::C10(v45, _, v47) => {
            let m672 = f141(v45);
            if m672 {
                Ok(f130(v0, f1(Str::lit("match:nat")), v47))
            } else {
                Ok(v0)
            }
        }
        Adt17::C11(r687, r688) => {
            let v48 = (*r687).clone();
            let v49 = (*r688).clone();
            let v50: Adt30 = f97(v0, v48)?;
            f97(v50, v49)
        }
        Adt17::C12 => {
            Ok(v0)
        }
        Adt17::C13(r689, r690) => {
            let v51 = (*r689).clone();
            let v52 = (*r690).clone();
            let v53: Adt30 = f97(v0, v51)?;
            f97(v53, v52)
        }
        Adt17::C14(v54, v55, v56, r691, v58, r692) => {
            let v57 = (*r691).clone();
            let v59 = (*r692).clone();
            let v60: Bytes = {
                let m673 = f142(v54);
                if m673 {
                    f1(Str::lit("destructure:pair"))
                } else {
                    f1(Str::lit("let"))
                }
            };
            let v61: Adt30 = {
                let m674 = v55;
                if m674 {
                    f143(v0, v56)
                } else {
                    v0
                }
            };
            let v62: Adt30 = f130(v61, v60, v58);
            let v63: Adt30 = f97(v62, v57)?;
            f97(v63, v59)
        }
        Adt17::C15(_, r693, r694) => {
            let v65 = (*r693).clone();
            let v66 = (*r694).clone();
            let v67: Adt30 = f97(v0, v65)?;
            f97(v67, v66)
        }
        Adt17::C16(_, r695) => {
            let v69 = (*r695).clone();
            f97(v0, v69)
        }
    }
}

pub fn f98(v0: List<(Adt12, List<u64>)>, v1: Adt12) -> bool {
    let m696 = f93(v0, v1);
    match m696 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f99(v0: Adt24) -> bool {
    let m697 = v0;
    match m697 {
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

pub fn f100(v0: List<Adt25>, v1: u64) -> Option<Adt25> {
    f144(Fn13::F145(v1), None::<Adt25>, v0)
}

pub fn f101(v0: Adt4) -> Bytes {
    let m698 = v0;
    match m698 {
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

pub fn f102(v0: Option<Adt4>) -> Adt4 {
    let m699 = v0;
    match m699 {
        None => {
            Adt4::C0
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f103(v0: Adt26, v1: u64, v2: u64) -> Option<u64> {
    let m701 = f18(match v0 {
        Adt26::C0(_, h700, _, _) => {
            h700
        }
    }, v1);
    match m701 {
        None => {
            None::<u64>
        }
        Some(v3) => {
            f90(v3, v2)
        }
    }
}

pub fn f104(v0: Fn5, p705: List<Adt12>, p706: List<Adt12>) -> List<Adt12> {
    let m704 = (p705, p706);
    {
        let mut m704 = m704;
        loop {
            let step = {
                let (v1, v2) = m704;
                let m702 = v2;
                match m702.uncons() {
                    None => {
                        Err::<(List<Adt12>, List<Adt12>), List<Adt12>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt12>, List<Adt12>), List<Adt12>>(({
                            let c703: Fn5 = v0.clone();
                            c703.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m704 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f105(v0: List<Adt12>, v1: Adt12) -> List<Adt12> {
    let v2: bool = f56(v0.clone(), v1.clone());
    if v2 {
        v0
    } else {
        List::cons(v1, v0)
    }
}

pub fn f106(v0: Fn6, p710: List<(Adt12, List<u64>)>, p711: List<(Adt12, List<u64>)>) -> List<(Adt12, List<u64>)> {
    let m709 = (p710, p711);
    {
        let mut m709 = m709;
        loop {
            let step = {
                let (v1, v2) = m709;
                let m707 = v2;
                match m707.uncons() {
                    None => {
                        Err::<(List<(Adt12, List<u64>)>, List<(Adt12, List<u64>)>), List<(Adt12, List<u64>)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Adt12, List<u64>)>, List<(Adt12, List<u64>)>), List<(Adt12, List<u64>)>>(({
                            let c708: Fn6 = v0.clone();
                            c708.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m709 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f107(v0: List<(Adt12, List<u64>)>, v1: (Adt12, List<u64>)) -> List<(Adt12, List<u64>)> {
    let m713 = f93(v0.clone(), {
        let (h712, _) = v1.clone();
        h712
    });
    match m713 {
        None => {
            f95(v0, {
                let (h716, _) = v1;
                h716
            }, {
                let (_, h717) = v1;
                h717
            })
        }
        Some(v2) => {
            f95(v0, {
                let (h714, _) = v1;
                h714
            }, f146(v2, {
                let (_, h715) = v1;
                h715
            }))
        }
    }
}

pub fn f108(v0: List<Adt25>, v1: Adt17) -> R<Result<(), Bytes>> {
    let m718 = v1;
    match m718 {
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
        Adt17::C5(v18, r721, _) => {
            let v19 = (*r721).clone();
            let m719 = f71(v18);
            if m719 {
                f147(v0, v19)
            } else {
                Ok(Ok::<(), Bytes>(()))
            }
        }
        Adt17::C6(_, _, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C7(_, r722, r723, _) => {
            let v25 = (*r722).clone();
            let v26 = (*r723).clone();
            let m720 = f78(v0.clone(), v25)?;
            match m720 {
                Ok(_) => {
                    f78(v0, v26)
                }
                Err(v29) => {
                    Ok(Err::<(), Bytes>(v29))
                }
            }
        }
        Adt17::C8(_, r724, _) => {
            let v31 = (*r724).clone();
            f148(v0, v31)
        }
        Adt17::C9(r725) => {
            let v33 = (*r725).clone();
            f78(v0, v33)
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

pub fn f109(v0: Adt12, v1: List<Adt12>, v2: Adt12) -> List<Adt12> {
    let v3: bool = f117(v2.clone(), v0);
    if v3 {
        v1
    } else {
        List::cons(v2, v1)
    }
}

pub fn f110(v0: Fn7, p729: Option<Adt12>, p730: List<Adt12>) -> Option<Adt12> {
    let m728 = (p729, p730);
    {
        let mut m728 = m728;
        loop {
            let step = {
                let (v1, v2) = m728;
                let m726 = v2;
                match m726.uncons() {
                    None => {
                        Err::<(Option<Adt12>, List<Adt12>), Option<Adt12>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Adt12>, List<Adt12>), Option<Adt12>>(({
                            let c727: Fn7 = v0.clone();
                            c727.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m728 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f111(v0: List<Adt12>, v1: List<Adt12>, v2: Adt12, v3: Option<Adt12>, v4: Adt12) -> Option<Adt12> {
    let v5: bool = f117(v4.clone(), v2);
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
        f149(v3, v4)
    } else {
        v3
    }
}

pub fn f112(v0: Fn8, p734: Option<Adt12>, p735: List<(Adt12, List<u64>)>) -> Option<Adt12> {
    let m733 = (p734, p735);
    {
        let mut m733 = m733;
        loop {
            let step = {
                let (v1, v2) = m733;
                let m731 = v2;
                match m731.uncons() {
                    None => {
                        Err::<(Option<Adt12>, List<(Adt12, List<u64>)>), Option<Adt12>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Adt12>, List<(Adt12, List<u64>)>), Option<Adt12>>(({
                            let c732: Fn8 = v0.clone();
                            c732.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m733 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f113(v0: List<(Adt12, List<u64>)>, v1: List<Adt12>, v2: Option<Adt12>, v3: (Adt12, List<u64>)) -> Option<Adt12> {
    let v4: bool = f56(v1, {
        let (h736, _) = v3.clone();
        h736
    });
    let v5: bool = f150(v0, {
        let (h737, _) = v3.clone();
        h737
    }, {
        let (_, h738) = v3;
        h738
    });
    let v6: bool = if v4 {
        !v5
    } else {
        false
    };
    if v6 {
        f149(v2, {
            let (h739, _) = v3;
            h739
        })
    } else {
        v2
    }
}

pub fn f114(v0: Fn9, p743: Option<u64>, p744: List<(Adt12, u64)>) -> Option<u64> {
    let m742 = (p743, p744);
    {
        let mut m742 = m742;
        loop {
            let step = {
                let (v1, v2) = m742;
                let m740 = v2;
                match m740.uncons() {
                    None => {
                        Err::<(Option<u64>, List<(Adt12, u64)>), Option<u64>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<u64>, List<(Adt12, u64)>), Option<u64>>(({
                            let c741: Fn9 = v0.clone();
                            c741.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m742 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f115(v0: Adt12, v1: Option<u64>, v2: (Adt12, u64)) -> Option<u64> {
    let m745 = v1;
    match m745 {
        None => {
            let v4: bool = f117({
                let (h746, _) = v2;
                h746
            }, v0);
            if v4 {
                Some({
                    let (_, h747) = v2;
                    h747
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

pub fn f116(v0: Bytes, v1: u64) -> Option<u8> {
    let a748 = v0;
    let a749 = v1;
    index_bytes(a748, a749)
}

pub fn f117(v0: Adt12, v1: Adt12) -> bool {
    let a750 = f151(v0.clone());
    let a751 = f151(v1.clone());
    if nat_eq(a750, a751) {
        let a752 = f152(v0.clone());
        let a753 = f152(v1.clone());
        if nat_eq(a752, a753) {
            let a754 = f153(v0);
            let a755 = f153(v1);
            equal(a754, a755)
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f118(v0: Fn10, p759: Option<u64>, p760: List<(u64, u64)>) -> Option<u64> {
    let m758 = (p759, p760);
    {
        let mut m758 = m758;
        loop {
            let step = {
                let (v1, v2) = m758;
                let m756 = v2;
                match m756.uncons() {
                    None => {
                        Err::<(Option<u64>, List<(u64, u64)>), Option<u64>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<u64>, List<(u64, u64)>), Option<u64>>(({
                            let c757: Fn10 = v0.clone();
                            c757.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m758 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f119(v0: u64, v1: Option<u64>, v2: (u64, u64)) -> Option<u64> {
    let m761 = v1;
    match m761 {
        None => {
            let a763 = {
                let (h762, _) = v2;
                h762
            };
            let a764 = v0;
            if nat_eq(a763, a764) {
                Some({
                    let (_, h765) = v2;
                    h765
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

pub fn f120(v0: Adt14) -> bool {
    let m766 = v0;
    match m766 {
        Adt14::C0(v1) => {
            let m767 = v1;
            match m767 {
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

pub fn f121(v0: Fn11, p771: Option<List<u64>>, p772: List<(Adt12, List<u64>)>) -> Option<List<u64>> {
    let m770 = (p771, p772);
    {
        let mut m770 = m770;
        loop {
            let step = {
                let (v1, v2) = m770;
                let m768 = v2;
                match m768.uncons() {
                    None => {
                        Err::<(Option<List<u64>>, List<(Adt12, List<u64>)>), Option<List<u64>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<List<u64>>, List<(Adt12, List<u64>)>), Option<List<u64>>>(({
                            let c769: Fn11 = v0.clone();
                            c769.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m770 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f122(v0: Adt12, v1: Option<List<u64>>, v2: (Adt12, List<u64>)) -> Option<List<u64>> {
    let m773 = v1;
    match m773 {
        None => {
            let v4: bool = f117({
                let (h774, _) = v2;
                h774
            }, v0);
            if v4 {
                Some({
                    let (_, h775) = v2;
                    h775
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

pub fn f123(v0: List<u64>, v1: u64) -> bool {
    f154(Fn14::F155(v1), false, v0)
}

pub fn f124(v0: List<(Adt12, List<u64>)>, v1: Adt12) -> List<(Adt12, List<u64>)> {
    f106(Fn6::F156(v1), List::<(Adt12, List<u64>)>::nil(), v0)
}

pub fn f125(v0: Fn12, p779: Option<Bytes>, p780: List<Adt29>) -> Option<Bytes> {
    let m778 = (p779, p780);
    {
        let mut m778 = m778;
        loop {
            let step = {
                let (v1, v2) = m778;
                let m776 = v2;
                match m776.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Adt29>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Adt29>), Option<Bytes>>(({
                            let c777: Fn12 = v0.clone();
                            c777.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m778 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f126(v0: Option<Bytes>, v1: Adt29) -> Option<Bytes> {
    let m781 = v0;
    match m781 {
        None => {
            Some(match v1 {
                Adt29::C0(h782, _, _, _) => {
                    h782
                }
            })
        }
        Some(v2) => {
            let m784 = f157(match v1.clone() {
                Adt29::C0(h783, _, _, _) => {
                    h783
                }
            }, v2.clone());
            if m784 {
                Some(match v1 {
                    Adt29::C0(h785, _, _, _) => {
                        h785
                    }
                })
            } else {
                Some(v2)
            }
        }
    }
}

pub fn f127(v0: Adt30, v1: Bytes, v2: Adt9, v3: Option<Adt4>) -> Adt30 {
    let v4: Adt29 = Adt29::C0(v1, match v2.clone() {
        Adt9::C0(h786, _) => {
            h786
        }
    }, match v2 {
        Adt9::C0(_, h787) => {
            h787
        }
    }, v3);
    let m789 = f158(match v0.clone() {
        Adt30::C0(h788, _, _) => {
            h788
        }
    }, v4.clone());
    if m789 {
        v0
    } else {
        Adt30::C0(List::cons(v4, match v0.clone() {
            Adt30::C0(h790, _, _) => {
                h790
            }
        }), match v0.clone() {
            Adt30::C0(_, h791, _) => {
                h791
            }
        }, match v0 {
            Adt30::C0(_, _, h792) => {
                h792
            }
        })
    }
}

pub fn f128(v0: Adt18) -> Bytes {
    let m793 = v0;
    match m793 {
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
    }
}

pub fn f129(v0: Adt18) -> Option<Adt4> {
    let m794 = v0;
    match m794 {
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
    }
}

pub fn f130(v0: Adt30, v1: Bytes, v2: Adt9) -> Adt30 {
    f127(v0, v1, v2, None::<Adt4>)
}

pub fn f131(v0: Adt19) -> Bytes {
    let m795 = v0;
    match m795 {
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

pub fn f132(v0: Adt20) -> Bytes {
    let m796 = v0;
    match m796 {
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

pub fn f133(v0: Bytes) -> Bytes {
    let m797 = f159(v0.clone(), f1(Str::lit("checked_neg_")));
    if m797 {
        f1(Str::lit("checked_neg"))
    } else {
        let m798 = f159(v0.clone(), f1(Str::lit("format_")));
        if m798 {
            f1(Str::lit("format_decimal"))
        } else {
            let m799 = f159(v0.clone(), f1(Str::lit("parse_")));
            if m799 {
                f1(Str::lit("parse_decimal"))
            } else {
                let m800 = f159(v0.clone(), f1(Str::lit("append_")));
                if m800 {
                    f1(Str::lit("append"))
                } else {
                    let m801 = f159(v0.clone(), f1(Str::lit("length_")));
                    if m801 {
                        f1(Str::lit("length"))
                    } else {
                        let m802 = f159(v0.clone(), f1(Str::lit("index_")));
                        if m802 {
                            f1(Str::lit("index"))
                        } else {
                            let m803 = f159(v0.clone(), f1(Str::lit("slice_")));
                            if m803 {
                                f1(Str::lit("slice"))
                            } else {
                                let m806 = {
                                    let a804 = v0.clone();
                                    let a805 = f1(Str::lit("nat_succ"));
                                    equal(a804, a805)
                                };
                                if m806 {
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

pub fn f134(v0: Bytes) -> R<Bytes> {
    let m807 = f160(v0.clone(), f1(Str::lit("::")))?;
    match m807 {
        None => {
            Ok(v0)
        }
        Some(v1) => {
            Ok(f40(v0.clone(), {
                let a808 = v1;
                let a809 = 2u64;
                nat_add(a808, a809)?
            }, {
                let a813 = {
                    let a810 = v0;
                    length_bytes(a810)
                };
                let a814 = {
                    let a811 = v1;
                    let a812 = 2u64;
                    nat_add(a811, a812)?
                };
                nat_sub(a813, a814)
            }))
        }
    }
}

pub fn f135(v0: List<Adt25>, v1: Adt22) -> Option<Adt4> {
    let m816 = f100(v0, match v1.clone() {
        Adt22::C0(h815, _) => {
            h815
        }
    });
    match m816 {
        None => {
            None::<Adt4>
        }
        Some(v2) => {
            let a818 = match v2 {
                Adt25::C0(_, _, h817, _, _) => {
                    h817
                }
            };
            let a819 = 0u64;
            if nat_eq(a818, a819) {
                None::<Adt4>
            } else {
                Some(f102(match v1 {
                    Adt22::C0(_, h820) => {
                        h820
                    }
                }))
            }
        }
    }
}

pub fn f136(v0: Adt30, v1: Adt23) -> Adt30 {
    let m821 = v1;
    match m821 {
        Adt23::C0(v2) => {
            f143(v0, v2)
        }
        Adt23::C1 => {
            v0
        }
        Adt23::C2(v3, v4) => {
            f143(f143(v0, v3), v4)
        }
        Adt23::C3(v5, v6) => {
            f143(f143(v0, v5), v6)
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
            f143(v0, v11)
        }
    }
}

pub fn f137(v0: Adt23) -> Bytes {
    let m822 = v0;
    match m822 {
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

pub fn f138(v0: Adt17) -> bool {
    let m823 = v0;
    match m823 {
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

pub fn f139(v0: Adt17) -> bool {
    let m824 = v0;
    match m824 {
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
            f161(v31)
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

pub fn f140(p829: Adt30, p830: Adt17, v2: Adt9) -> Adt30 {
    let m828 = (p829, p830);
    {
        let mut m828 = m828;
        loop {
            let step = {
                let (v0, v1) = m828;
                let m825 = v1;
                match m825 {
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
                    Adt17::C15(v46, _, r827) => {
                        let v48 = (*r827).clone();
                        let m826 = f162(v46);
                        match m826 {
                            None => {
                                Ok::<(Adt30, Adt17), Adt30>((v0, v48))
                            }
                            Some(v49) => {
                                Ok::<(Adt30, Adt17), Adt30>((f130(v0, v49, v2.clone()), v48))
                            }
                        }
                    }
                    Adt17::C16(_, _) => {
                        Err::<(Adt30, Adt17), Adt30>(v0)
                    }
                }
            };
            match step {
                Ok(next) => { m828 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f141(v0: Adt24) -> bool {
    let m831 = v0;
    match m831 {
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

pub fn f142(v0: Adt14) -> bool {
    let m832 = v0;
    match m832 {
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

pub fn f143(v0: Adt30, v1: Adt2) -> Adt30 {
    let m833 = v1;
    match m833 {
        Adt2::C0(v2) => {
            f163(v0, f4(f1(Str::lit("type:")), f164(v2)))
        }
        Adt2::C1(_) => {
            f163(v0, f1(Str::lit("type:fixed")))
        }
        Adt2::C2(v4, r834) => {
            let v5 = (*r834).clone();
            let v6: Adt30 = f163(v0, f4(f1(Str::lit("type:")), f165(v4)));
            f143(v6, v5)
        }
        Adt2::C3(v7, r835, r836) => {
            let v8 = (*r835).clone();
            let v9 = (*r836).clone();
            let v10: Adt30 = f163(v0, f4(f1(Str::lit("type:")), f166(v7)));
            let v11: Adt30 = f143(v10, v8);
            f143(v11, v9)
        }
        Adt2::C4(v12, _) => {
            f163(v0, f4(f1(Str::lit("type:")), f167(v12)))
        }
        Adt2::C5 => {
            v0
        }
        Adt2::C6(r837, r838) => {
            let v14 = (*r837).clone();
            let v15 = (*r838).clone();
            let v16: Adt30 = f143(v0, v14);
            f143(v16, v15)
        }
    }
}

pub fn f144(v0: Fn13, p842: Option<Adt25>, p843: List<Adt25>) -> Option<Adt25> {
    let m841 = (p842, p843);
    {
        let mut m841 = m841;
        loop {
            let step = {
                let (v1, v2) = m841;
                let m839 = v2;
                match m839.uncons() {
                    None => {
                        Err::<(Option<Adt25>, List<Adt25>), Option<Adt25>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Adt25>, List<Adt25>), Option<Adt25>>(({
                            let c840: Fn13 = v0.clone();
                            c840.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m841 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f145(v0: u64, v1: Option<Adt25>, v2: Adt25) -> Option<Adt25> {
    let m844 = v1;
    match m844 {
        None => {
            let a846 = match v2.clone() {
                Adt25::C0(h845, _, _, _, _) => {
                    h845
                }
            };
            let a847 = v0;
            if nat_eq(a846, a847) {
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

pub fn f146(v0: List<u64>, v1: List<u64>) -> List<u64> {
    f168(Fn15::F169, v0, v1)
}

pub fn f147(v0: List<Adt25>, p853: Adt17) -> R<Result<(), Bytes>> {
    let m852 = p853;
    {
        let mut m852 = m852;
        loop {
            let step = {
                let v1 = m852;
                let m848 = v1;
                match m848 {
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
                    Adt17::C13(r850, r851) => {
                        let v37 = (*r850).clone();
                        let v38 = (*r851).clone();
                        let m849 = f170(v37.clone());
                        if m849 {
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
                Ok(next) => { m852 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f148(v0: List<Adt25>, p859: Adt17) -> R<Result<(), Bytes>> {
    let m858 = p859;
    {
        let mut m858 = m858;
        loop {
            let step = {
                let v1 = m858;
                let m854 = v1;
                match m854 {
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
                    Adt17::C15(_, r856, r857) => {
                        let v46 = (*r856).clone();
                        let v47 = (*r857).clone();
                        let m855 = f78(v0.clone(), v46)?;
                        match m855 {
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
                Ok(next) => { m858 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f149(v0: Option<Adt12>, v1: Adt12) -> Option<Adt12> {
    let m860 = v0;
    match m860 {
        None => {
            Some(v1)
        }
        Some(v2) => {
            let m861 = f171(v1.clone(), v2.clone());
            if m861 {
                Some(v1)
            } else {
                Some(v2)
            }
        }
    }
}

pub fn f150(v0: List<(Adt12, List<u64>)>, v1: Adt12, v2: List<u64>) -> bool {
    let m862 = f93(v0, v1);
    match m862 {
        None => {
            false
        }
        Some(v3) => {
            let v4: bool = f172(v2.clone(), v3.clone());
            let v5: bool = f172(v3, v2);
            if v4 {
                v5
            } else {
                false
            }
        }
    }
}

pub fn f151(v0: Adt12) -> u64 {
    let m863 = v0;
    match m863 {
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

pub fn f152(v0: Adt12) -> u64 {
    let m864 = v0;
    match m864 {
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

pub fn f153(v0: Adt12) -> Bytes {
    let m865 = v0;
    match m865 {
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

pub fn f154(v0: Fn14, p869: bool, p870: List<u64>) -> bool {
    let m868 = (p869, p870);
    {
        let mut m868 = m868;
        loop {
            let step = {
                let (v1, v2) = m868;
                let m866 = v2;
                match m866.uncons() {
                    None => {
                        Err::<(bool, List<u64>), bool>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<u64>), bool>(({
                            let c867: Fn14 = v0.clone();
                            c867.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m868 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f155(v0: u64, v1: bool, v2: u64) -> bool {
    if v1 {
        true
    } else {
        let a871 = v2;
        let a872 = v0;
        nat_eq(a871, a872)
    }
}

pub fn f156(v0: Adt12, v1: List<(Adt12, List<u64>)>, v2: (Adt12, List<u64>)) -> List<(Adt12, List<u64>)> {
    let v3: bool = f117({
        let (h873, _) = v2.clone();
        h873
    }, v0);
    if v3 {
        v1
    } else {
        List::cons(v2, v1)
    }
}

pub fn f157(v0: Bytes, v1: Bytes) -> bool {
    let a878 = {
        let a874 = v0;
        let a875 = v1;
        compare_bytes(a874, a875)
    };
    let a879 = {
        let a876 = Bytes::lit(&[]);
        let a877 = Bytes::lit(&[0x01]);
        compare_bytes(a876, a877)
    };
    equal(a878, a879)
}

pub fn f158(v0: List<Adt29>, v1: Adt29) -> bool {
    f173(Fn16::F174(v1), false, v0)
}

pub fn f159(v0: Bytes, v1: Bytes) -> bool {
    let m884 = {
        let a881 = v0;
        let a882 = 0u64;
        let a883 = {
            let a880 = v1.clone();
            length_bytes(a880)
        };
        slice_bytes(a881, a882, a883)
    };
    match m884 {
        None => {
            false
        }
        Some(v2) => {
            let a885 = v2;
            let a886 = v1;
            equal(a885, a886)
        }
    }
}

pub fn f160(v0: Bytes, v1: Bytes) -> R<Option<u64>> {
    let v2: u64 = {
        let a887 = v0.clone();
        length_bytes(a887)
    };
    let v3: u64 = {
        let a888 = v1.clone();
        length_bytes(a888)
    };
    let v4: u64 = {
        let (h889, _) = f38(Fn2::F175(v1.clone(), v3, v0.clone(), v2), {
            let a890 = v2;
            let a891 = 1u64;
            nat_add(a890, a891)?
        }, 0u64)?;
        h889
    };
    if f176(v0, v1, v4) {
        Ok(Some(v4))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f161(v0: Adt24) -> bool {
    let m892 = v0;
    match m892 {
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

pub fn f162(v0: Adt14) -> Option<Bytes> {
    let m893 = v0;
    match m893 {
        Adt14::C0(v1) => {
            f177(v1)
        }
        Adt14::C1(_) => {
            None::<Bytes>
        }
        Adt14::C2(_) => {
            None::<Bytes>
        }
        Adt14::C3(v4, _) => {
            Some(f178(v4))
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

pub fn f163(v0: Adt30, v1: Bytes) -> Adt30 {
    Adt30::C0(match v0.clone() {
        Adt30::C0(h894, _, _) => {
            h894
        }
    }, f179(match v0.clone() {
        Adt30::C0(_, h895, _) => {
            h895
        }
    }, v1), match v0 {
        Adt30::C0(_, _, h896) => {
            h896
        }
    })
}

pub fn f164(v0: Adt3) -> Bytes {
    let m897 = v0;
    match m897 {
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
    }
}

pub fn f165(v0: Adt5) -> Bytes {
    let m898 = v0;
    match m898 {
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

pub fn f166(v0: Adt6) -> Bytes {
    let m899 = v0;
    match m899 {
        Adt6::C0 => {
            f1(Str::lit("result"))
        }
        Adt6::C1 => {
            f1(Str::lit("pair"))
        }
    }
}

pub fn f167(v0: Adt7) -> Bytes {
    let m900 = v0;
    match m900 {
        Adt7::C0 => {
            f1(Str::lit("adt"))
        }
        Adt7::C1 => {
            f1(Str::lit("fn"))
        }
    }
}

pub fn f168(v0: Fn15, p904: List<u64>, p905: List<u64>) -> List<u64> {
    let m903 = (p904, p905);
    {
        let mut m903 = m903;
        loop {
            let step = {
                let (v1, v2) = m903;
                let m901 = v2;
                match m901.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>(({
                            let c902: Fn15 = v0.clone();
                            c902.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m903 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f169(v0: List<u64>, v1: u64) -> List<u64> {
    let v2: bool = f123(v0.clone(), v1);
    if v2 {
        v0
    } else {
        List::cons(v1, v0)
    }
}

pub fn f170(v0: Adt17) -> bool {
    let m906 = v0;
    match m906 {
        Adt17::C0(_, _) => {
            false
        }
        Adt17::C1(_, _, _) => {
            false
        }
        Adt17::C2(v6, _, _) => {
            let m907 = v6;
            match m907 {
                Adt20::C0 => {
                    false
                }
                Adt20::C1 => {
                    false
                }
                Adt20::C2 => {
                    false
                }
                Adt20::C3 => {
                    true
                }
            }
        }
        Adt17::C3(_, _, v11, _) => {
            !v11
        }
        Adt17::C4(_, _, v15, _) => {
            !v15
        }
        Adt17::C5(_, _, _) => {
            false
        }
        Adt17::C6(_, _, _) => {
            false
        }
        Adt17::C7(_, _, _, _) => {
            true
        }
        Adt17::C8(_, _, _) => {
            true
        }
        Adt17::C9(_) => {
            true
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
            true
        }
    }
}

pub fn f171(v0: Adt12, v1: Adt12) -> bool {
    let a908 = f151(v0.clone());
    let a909 = f151(v1.clone());
    if nat_eq(a908, a909) {
        let a910 = f151(v0.clone());
        let a911 = 9u64;
        if nat_eq(a910, a911) {
            f157(f153(v0), f153(v1))
        } else {
            let a912 = f152(v0);
            let a913 = f152(v1);
            nat_lt(a912, a913)
        }
    } else {
        let a914 = f151(v0);
        let a915 = f151(v1);
        nat_lt(a914, a915)
    }
}

pub fn f172(v0: List<u64>, v1: List<u64>) -> bool {
    f154(Fn14::F180(v1), true, v0)
}

pub fn f173(v0: Fn16, p919: bool, p920: List<Adt29>) -> bool {
    let m918 = (p919, p920);
    {
        let mut m918 = m918;
        loop {
            let step = {
                let (v1, v2) = m918;
                let m916 = v2;
                match m916.uncons() {
                    None => {
                        Err::<(bool, List<Adt29>), bool>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<Adt29>), bool>(({
                            let c917: Fn16 = v0.clone();
                            c917.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m918 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f174(v0: Adt29, v1: bool, v2: Adt29) -> bool {
    let v3: bool = f181(v2, v0);
    if v1 {
        true
    } else {
        v3
    }
}

pub fn f175(v0: Bytes, v1: u64, v2: Bytes, v3: u64, v4: u64) -> R<Option<u64>> {
    if f176(v2, v0, v4) {
        Ok(None::<u64>)
    } else {
        let a923 = {
            let a921 = v4;
            let a922 = v1;
            nat_add(a921, a922)?
        };
        let a924 = v3;
        if nat_lt(a923, a924) {
            Ok(Some({
                let a925 = v4;
                let a926 = 1u64;
                nat_add(a925, a926)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f176(v0: Bytes, v1: Bytes, v2: u64) -> bool {
    let m931 = {
        let a928 = v0;
        let a929 = v2;
        let a930 = {
            let a927 = v1.clone();
            length_bytes(a927)
        };
        slice_bytes(a928, a929, a930)
    };
    match m931 {
        None => {
            false
        }
        Some(v3) => {
            let a932 = v3;
            let a933 = v1;
            equal(a932, a933)
        }
    }
}

pub fn f177(v0: Adt15) -> Option<Bytes> {
    let m934 = v0;
    match m934 {
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

pub fn f178(v0: Adt16) -> Bytes {
    let m935 = v0;
    match m935 {
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

pub fn f179(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    let m936 = f182(v0.clone(), v1.clone());
    if m936 {
        v0
    } else {
        let v2: (bool, List<Bytes>) = f183(Fn17::F184(v1.clone()), (false, List::<Bytes>::nil()), v0);
        let m938 = {
            let (h937, _) = v2;
            h937
        };
        if m938 {
            f185({
                let (_, h939) = v2;
                h939
            })
        } else {
            f185(List::cons(v1, {
                let (_, h940) = v2;
                h940
            }))
        }
    }
}

pub fn f180(v0: List<u64>, v1: bool, v2: u64) -> bool {
    let v3: bool = f123(v0, v2);
    if v1 {
        v3
    } else {
        false
    }
}

pub fn f181(v0: Adt29, v1: Adt29) -> bool {
    let a943 = match v0.clone() {
        Adt29::C0(h941, _, _, _) => {
            h941
        }
    };
    let a944 = match v1.clone() {
        Adt29::C0(h942, _, _, _) => {
            h942
        }
    };
    if equal(a943, a944) {
        let a947 = match v0.clone() {
            Adt29::C0(_, h945, _, _) => {
                h945
            }
        };
        let a948 = match v1.clone() {
            Adt29::C0(_, h946, _, _) => {
                h946
            }
        };
        if equal(a947, a948) {
            if f186(match v0.clone() {
                Adt29::C0(_, _, h949, _) => {
                    h949
                }
            }, match v1.clone() {
                Adt29::C0(_, _, h950, _) => {
                    h950
                }
            }) {
                f186(match v0 {
                    Adt29::C0(_, _, _, h951) => {
                        h951
                    }
                }, match v1 {
                    Adt29::C0(_, _, _, h952) => {
                        h952
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

pub fn f182(v0: List<Bytes>, v1: Bytes) -> bool {
    f187(Fn18::F188(v1), false, v0)
}

pub fn f183(v0: Fn17, p956: (bool, List<Bytes>), p957: List<Bytes>) -> (bool, List<Bytes>) {
    let m955 = (p956, p957);
    {
        let mut m955 = m955;
        loop {
            let step = {
                let (v1, v2) = m955;
                let m953 = v2;
                match m953.uncons() {
                    None => {
                        Err::<((bool, List<Bytes>), List<Bytes>), (bool, List<Bytes>)>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<((bool, List<Bytes>), List<Bytes>), (bool, List<Bytes>)>(({
                            let c954: Fn17 = v0.clone();
                            c954.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m955 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f184(v0: Bytes, v1: (bool, List<Bytes>), v2: Bytes) -> (bool, List<Bytes>) {
    f189(v0, v1, v2)
}

pub fn f185(v0: List<Bytes>) -> List<Bytes> {
    f190(Fn19::F191, List::<Bytes>::nil(), v0)
}

pub fn f186(v0: Option<Adt4>, v1: Option<Adt4>) -> bool {
    let a958 = f192(v0);
    let a959 = f192(v1);
    equal(a958, a959)
}

pub fn f187(v0: Fn18, p963: bool, p964: List<Bytes>) -> bool {
    let m962 = (p963, p964);
    {
        let mut m962 = m962;
        loop {
            let step = {
                let (v1, v2) = m962;
                let m960 = v2;
                match m960.uncons() {
                    None => {
                        Err::<(bool, List<Bytes>), bool>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<Bytes>), bool>(({
                            let c961: Fn18 = v0.clone();
                            c961.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m962 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f188(v0: Bytes, v1: bool, v2: Bytes) -> bool {
    if v1 {
        true
    } else {
        let a965 = v2;
        let a966 = v0;
        equal(a965, a966)
    }
}

pub fn f189(v0: Bytes, v1: (bool, List<Bytes>), v2: Bytes) -> (bool, List<Bytes>) {
    let v3: bool = {
        let (h967, _) = v1;
        h967
    };
    let v4: List<Bytes> = {
        let (_, h968) = v1;
        h968
    };
    let m969 = v3;
    if m969 {
        (true, List::cons(v2, v4))
    } else {
        let m970 = f157(v0.clone(), v2.clone());
        if m970 {
            (true, List::cons(v2, List::cons(v0, v4)))
        } else {
            (false, List::cons(v2, v4))
        }
    }
}

pub fn f190(v0: Fn19, p974: List<Bytes>, p975: List<Bytes>) -> List<Bytes> {
    let m973 = (p974, p975);
    {
        let mut m973 = m973;
        loop {
            let step = {
                let (v1, v2) = m973;
                let m971 = v2;
                match m971.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c972: Fn19 = v0.clone();
                            c972.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m973 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f191(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f192(v0: Option<Adt4>) -> Bytes {
    let m976 = v0;
    match m976 {
        None => {
            f1(Str::lit("None"))
        }
        Some(v1) => {
            f4(f1(Str::lit("Some(")), f4(f14(v1), f1(Str::lit(")"))))
        }
    }
}
