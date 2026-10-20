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
    F174(Bytes, u64, Bytes, u64),
}

impl Fn2 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn2::F39(k0) => f39(k0.clone(), p0),
            Fn2::F174(k0, k1, k2, k3) => f174(k0.clone(), *k1, k2.clone(), *k3, p0),
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
    F87(Adt12),
}

impl Fn4 {
    pub fn apply(&self, p0: bool, p1: Adt12) -> bool {
        match self {
            Fn4::F87(k0) => f87(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F103,
    F107(Adt12),
}

impl Fn5 {
    pub fn apply(&self, p0: List<Adt12>, p1: Adt12) -> List<Adt12> {
        match self {
            Fn5::F103 => f103(p0, p1),
            Fn5::F107(k0) => f107(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F105,
    F155(Adt12),
}

impl Fn6 {
    pub fn apply(&self, p0: List<(Adt12, List<u64>)>, p1: (Adt12, List<u64>)) -> List<(Adt12, List<u64>)> {
        match self {
            Fn6::F105 => f105(p0, p1),
            Fn6::F155(k0) => f155(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F109(List<Adt12>, List<Adt12>, Adt12),
}

impl Fn7 {
    pub fn apply(&self, p0: Option<Adt12>, p1: Adt12) -> Option<Adt12> {
        match self {
            Fn7::F109(k0, k1, k2) => f109(k0.clone(), k1.clone(), k2.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F111(List<(Adt12, List<u64>)>, List<Adt12>),
}

impl Fn8 {
    pub fn apply(&self, p0: Option<Adt12>, p1: (Adt12, List<u64>)) -> Option<Adt12> {
        match self {
            Fn8::F111(k0, k1) => f111(k0.clone(), k1.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F113(Adt12),
}

impl Fn9 {
    pub fn apply(&self, p0: Option<u64>, p1: (Adt12, u64)) -> Option<u64> {
        match self {
            Fn9::F113(k0) => f113(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F117(u64),
}

impl Fn10 {
    pub fn apply(&self, p0: Option<u64>, p1: (u64, u64)) -> Option<u64> {
        match self {
            Fn10::F117(k0) => f117(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F120(Adt12),
}

impl Fn11 {
    pub fn apply(&self, p0: Option<List<u64>>, p1: (Adt12, List<u64>)) -> Option<List<u64>> {
        match self {
            Fn11::F120(k0) => f120(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F124,
}

impl Fn12 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Adt29) -> Option<Bytes> {
        match self {
            Fn12::F124 => f124(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F143(u64),
}

impl Fn13 {
    pub fn apply(&self, p0: Option<Adt25>, p1: Adt25) -> Option<Adt25> {
        match self {
            Fn13::F143(k0) => f143(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F154(u64),
    F179(List<u64>),
}

impl Fn14 {
    pub fn apply(&self, p0: bool, p1: u64) -> bool {
        match self {
            Fn14::F154(k0) => f154(*k0, p0, p1),
            Fn14::F179(k0) => f179(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F168,
}

impl Fn15 {
    pub fn apply(&self, p0: List<u64>, p1: u64) -> List<u64> {
        match self {
            Fn15::F168 => f168(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F173(Adt29),
}

impl Fn16 {
    pub fn apply(&self, p0: bool, p1: Adt29) -> bool {
        match self {
            Fn16::F173(k0) => f173(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F183(Bytes),
}

impl Fn17 {
    pub fn apply(&self, p0: (bool, List<Bytes>), p1: Bytes) -> (bool, List<Bytes>) {
        match self {
            Fn17::F183(k0) => f183(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F187(Bytes),
}

impl Fn18 {
    pub fn apply(&self, p0: bool, p1: Bytes) -> bool {
        match self {
            Fn18::F187(k0) => f187(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F190,
}

impl Fn19 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn19::F190 => f190(p0, p1),
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
        Adt17::C2(v22, r413, v24) => {
            let v23 = (*r413).clone();
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
        Adt17::C3(v30, r414, v32, v33) => {
            let v31 = (*r414).clone();
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
                    let m298 = f66(v0, v1.clone(), v31)?;
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
                                Adt21::C1(v38) => {
                                    let v39: Bytes = f68(match v1.clone() {
                                        Adt27::C0(_, _, _, h304) => {
                                            h304
                                        }
                                    }, v38.clone());
                                    let m307 = {
                                        let m306 = f69(match v1.clone() {
                                            Adt27::C0(_, _, _, h305) => {
                                                h305
                                            }
                                        }, v38.clone());
                                        if m306 {
                                            f44(v1.clone(), f4(f1(Str::lit("the runtime function ")), f30(v39.clone())))
                                        } else {
                                            Ok::<(), Bytes>(())
                                        }
                                    };
                                    match m307 {
                                        Ok(_) => {
                                            Ok(f67(v35, v1.clone(), f70(match v1 {
                                                Adt27::C0(_, _, _, h308) => {
                                                    h308
                                                }
                                            }, v38), v32, v3, f30(v39)))
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
                let m309 = f66(v0, v1.clone(), v31)?;
                match m309 {
                    Ok(v43) => {
                        let m310 = v30;
                        match m310 {
                            Adt21::C0(v44) => {
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
                                }, v44);
                                match m314 {
                                    None => {
                                        Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("f")), f4(f10(v44)?, f1(Str::lit(" is not declared"))))))
                                    }
                                    Some(v45) => {
                                        Ok(f67(v43, v1, v45, v32, v3, f4(f1(Str::lit("f")), f10(v44)?)))
                                    }
                                }
                            }
                            Adt21::C1(v46) => {
                                let v47: Bytes = f68(match v1.clone() {
                                    Adt27::C0(_, _, _, h315) => {
                                        h315
                                    }
                                }, v46.clone());
                                let m318 = {
                                    let m317 = f69(match v1.clone() {
                                        Adt27::C0(_, _, _, h316) => {
                                            h316
                                        }
                                    }, v46.clone());
                                    if m317 {
                                        f44(v1.clone(), f4(f1(Str::lit("the runtime function ")), f30(v47.clone())))
                                    } else {
                                        Ok::<(), Bytes>(())
                                    }
                                };
                                match m318 {
                                    Ok(_) => {
                                        Ok(f67(v43, v1.clone(), f70(match v1 {
                                            Adt27::C0(_, _, _, h319) => {
                                                h319
                                            }
                                        }, v46), v32, v3, f30(v47)))
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
        Adt17::C4(v51, r415, v53, v54) => {
            let v52 = (*r415).clone();
            let m320 = v53;
            if m320 {
                let v55: bool = if v3 {
                    match v1.clone() {
                        Adt27::C0(_, _, h321, _) => {
                            h321
                        }
                    }
                } else {
                    false
                };
                let m322 = v55;
                if m322 {
                    Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("arithmetic: the value a fallible function returns, ")), f4(f61(match v1 {
                        Adt27::C0(_, _, _, h323) => {
                            h323
                        }
                    }, Adt17::C4(v51, std::rc::Rc::new(v52), v53, v54))?, f1(Str::lit(", is not a fallible value"))))))
                } else {
                    let m324 = f64(v0.clone(), v51)?;
                    match m324 {
                        Ok(_) => {
                            let m325 = f66(v0, v1, v52)?;
                            match m325 {
                                Ok(v57) => {
                                    Ok(Ok::<Adt28, Bytes>(Adt28::C0(match v57.clone() {
                                        Adt28::C0(h326, _, _, _, _) => {
                                            h326
                                        }
                                    }, match v57.clone() {
                                        Adt28::C0(_, h327, _, _, _) => {
                                            h327
                                        }
                                    }, match v57.clone() {
                                        Adt28::C0(_, _, h328, _, _) => {
                                            h328
                                        }
                                    }, match v57.clone() {
                                        Adt28::C0(_, _, _, h329, _) => {
                                            h329
                                        }
                                    }, if v53 {
                                        true
                                    } else {
                                        match v57 {
                                            Adt28::C0(_, _, _, _, h330) => {
                                                h330
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
                let m331 = f64(v0.clone(), v51)?;
                match m331 {
                    Ok(_) => {
                        let m332 = f66(v0, v1, v52)?;
                        match m332 {
                            Ok(v61) => {
                                Ok(Ok::<Adt28, Bytes>(Adt28::C0(match v61.clone() {
                                    Adt28::C0(h333, _, _, _, _) => {
                                        h333
                                    }
                                }, match v61.clone() {
                                    Adt28::C0(_, h334, _, _, _) => {
                                        h334
                                    }
                                }, match v61.clone() {
                                    Adt28::C0(_, _, h335, _, _) => {
                                        h335
                                    }
                                }, match v61.clone() {
                                    Adt28::C0(_, _, _, h336, _) => {
                                        h336
                                    }
                                }, if v53 {
                                    true
                                } else {
                                    match v61 {
                                        Adt28::C0(_, _, _, _, h337) => {
                                            h337
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
        Adt17::C5(v64, r416, v66) => {
            let v65 = (*r416).clone();
            let v67: bool = if v3 {
                match v1.clone() {
                    Adt27::C0(_, _, h338, _) => {
                        h338
                    }
                }
            } else {
                false
            };
            let m339 = v67;
            if m339 {
                Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("arithmetic: the value a fallible function returns, ")), f4(f61(match v1 {
                    Adt27::C0(_, _, _, h340) => {
                        h340
                    }
                }, Adt17::C5(v64, std::rc::Rc::new(v65), v66))?, f1(Str::lit(", is not a fallible value"))))))
            } else {
                let m341 = f66(v0, v1.clone(), v65.clone())?;
                match m341 {
                    Ok(v68) => {
                        let m342 = f71(v1, v64, f72(v65)?)?;
                        match m342 {
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
            }
        }
        Adt17::C6(r417, r418, v74) => {
            let v72 = (*r417).clone();
            let v73 = (*r418).clone();
            let v75: bool = if v3 {
                match v1.clone() {
                    Adt27::C0(_, _, h343, _) => {
                        h343
                    }
                }
            } else {
                false
            };
            let m344 = v75;
            if m344 {
                Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("arithmetic: the value a fallible function returns, ")), f4(f61(match v1 {
                    Adt27::C0(_, _, _, h345) => {
                        h345
                    }
                }, Adt17::C6(std::rc::Rc::new(v72), std::rc::Rc::new(v73), v74))?, f1(Str::lit(", is not a fallible value"))))))
            } else {
                let m346 = f47(v0, v1.clone(), v72, false)?;
                match m346 {
                    Ok(v76) => {
                        f47(v76, v1, v73, false)
                    }
                    Err(v77) => {
                        Ok(Err::<Adt28, Bytes>(v77))
                    }
                }
            }
        }
        Adt17::C7(r419, r420, r421, _) => {
            let v78 = (*r419).clone();
            let v79 = (*r420).clone();
            let v80 = (*r421).clone();
            let m347 = f47(v0, v1.clone(), v78, false)?;
            match m347 {
                Ok(v82) => {
                    let v83: Adt28 = Adt28::C0(match v82.clone() {
                        Adt28::C0(h348, _, _, _, _) => {
                            h348
                        }
                    }, match v82.clone() {
                        Adt28::C0(_, h349, _, _, _) => {
                            h349
                        }
                    }, match v82.clone() {
                        Adt28::C0(_, _, h350, _, _) => {
                            h350
                        }
                    }, match v82.clone() {
                        Adt28::C0(_, _, _, h351, _) => {
                            h351
                        }
                    }, match v82.clone() {
                        Adt28::C0(_, _, _, _, h352) => {
                            h352
                        }
                    });
                    let m354 = f45(v83, match v1.clone() {
                        Adt27::C0(_, h353, _, _) => {
                            h353
                        }
                    }, Adt14::C0(Adt15::C0))?;
                    match m354 {
                        Ok(v84) => {
                            let m355 = f29(v84, v1.clone(), v79, v3)?;
                            match m355 {
                                Ok(v85) => {
                                    let v86: Adt28 = Adt28::C0(match v85.clone() {
                                        Adt28::C0(h356, _, _, _, _) => {
                                            h356
                                        }
                                    }, match v82.clone() {
                                        Adt28::C0(_, h357, _, _, _) => {
                                            h357
                                        }
                                    }, f73(match v82.clone() {
                                        Adt28::C0(_, _, h358, _, _) => {
                                            h358
                                        }
                                    }, match v85.clone() {
                                        Adt28::C0(_, _, h359, _, _) => {
                                            h359
                                        }
                                    }), f74(match v82.clone() {
                                        Adt28::C0(_, _, _, h360, _) => {
                                            h360
                                        }
                                    }, match v85.clone() {
                                        Adt28::C0(_, _, _, h361, _) => {
                                            h361
                                        }
                                    }), match v85 {
                                        Adt28::C0(_, _, _, _, h362) => {
                                            h362
                                        }
                                    });
                                    let v87: Adt28 = Adt28::C0(match v86.clone() {
                                        Adt28::C0(h363, _, _, _, _) => {
                                            h363
                                        }
                                    }, match v82.clone() {
                                        Adt28::C0(_, h364, _, _, _) => {
                                            h364
                                        }
                                    }, match v82.clone() {
                                        Adt28::C0(_, _, h365, _, _) => {
                                            h365
                                        }
                                    }, match v82.clone() {
                                        Adt28::C0(_, _, _, h366, _) => {
                                            h366
                                        }
                                    }, match v86.clone() {
                                        Adt28::C0(_, _, _, _, h367) => {
                                            h367
                                        }
                                    });
                                    let m369 = f45(v87, match v1.clone() {
                                        Adt27::C0(_, h368, _, _) => {
                                            h368
                                        }
                                    }, Adt14::C0(Adt15::C0))?;
                                    match m369 {
                                        Ok(v88) => {
                                            let m370 = f29(v88, v1, v80, v3)?;
                                            match m370 {
                                                Ok(v89) => {
                                                    let v90: Adt28 = Adt28::C0(match v89.clone() {
                                                        Adt28::C0(h371, _, _, _, _) => {
                                                            h371
                                                        }
                                                    }, match v82 {
                                                        Adt28::C0(_, h372, _, _, _) => {
                                                            h372
                                                        }
                                                    }, f73(match v86.clone() {
                                                        Adt28::C0(_, _, h373, _, _) => {
                                                            h373
                                                        }
                                                    }, match v89.clone() {
                                                        Adt28::C0(_, _, h374, _, _) => {
                                                            h374
                                                        }
                                                    }), f74(match v86 {
                                                        Adt28::C0(_, _, _, h375, _) => {
                                                            h375
                                                        }
                                                    }, match v89.clone() {
                                                        Adt28::C0(_, _, _, h376, _) => {
                                                            h376
                                                        }
                                                    }), match v89 {
                                                        Adt28::C0(_, _, _, _, h377) => {
                                                            h377
                                                        }
                                                    });
                                                    Ok(Ok::<Adt28, Bytes>(v90))
                                                }
                                                Err(v91) => {
                                                    Ok(Err::<Adt28, Bytes>(v91))
                                                }
                                            }
                                        }
                                        Err(v92) => {
                                            Ok(Err::<Adt28, Bytes>(v92))
                                        }
                                    }
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
        Adt17::C8(r422, r423, _) => {
            let v96 = (*r422).clone();
            let v97 = (*r423).clone();
            let m378 = f47(v0, v1.clone(), v96, false)?;
            match m378 {
                Ok(v99) => {
                    f75(v99.clone(), v1, v99.clone(), match v99 {
                        Adt28::C0(_, h379, _, _, _) => {
                            h379
                        }
                    }, v97, v3)
                }
                Err(v100) => {
                    Ok(Err::<Adt28, Bytes>(v100))
                }
            }
        }
        Adt17::C9(r424) => {
            let v101 = (*r424).clone();
            f29(v0, v1, v101, v3)
        }
        Adt17::C10(v102, v103, v104) => {
            let v105: bool = if v3 {
                match v1.clone() {
                    Adt27::C0(_, _, h380, _) => {
                        h380
                    }
                }
            } else {
                false
            };
            let m381 = v105;
            if m381 {
                Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("arithmetic: the value a fallible function returns, ")), f4(f61(match v1 {
                    Adt27::C0(_, _, _, h382) => {
                        h382
                    }
                }, Adt17::C10(v102, v103, v104))?, f1(Str::lit(", is not a fallible value"))))))
            } else {
                let m383 = v102;
                match m383 {
                    Adt24::C0 => {
                        let m384 = f64(v0.clone(), v103)?;
                        match m384 {
                            Ok(_) => {
                                Ok(Ok::<Adt28, Bytes>(v0))
                            }
                            Err(v107) => {
                                Ok(Err::<Adt28, Bytes>(v107))
                            }
                        }
                    }
                    Adt24::C1 => {
                        let m385 = f64(v0.clone(), v103)?;
                        match m385 {
                            Ok(_) => {
                                Ok(Ok::<Adt28, Bytes>(v0))
                            }
                            Err(v109) => {
                                Ok(Err::<Adt28, Bytes>(v109))
                            }
                        }
                    }
                    Adt24::C2 => {
                        let m386 = f44(v1, f1(Str::lit("a list match")));
                        match m386 {
                            Ok(_) => {
                                let m387 = f64(v0.clone(), v103)?;
                                match m387 {
                                    Ok(_) => {
                                        Ok(Ok::<Adt28, Bytes>(v0))
                                    }
                                    Err(v112) => {
                                        Ok(Err::<Adt28, Bytes>(v112))
                                    }
                                }
                            }
                            Err(v113) => {
                                Ok(Err::<Adt28, Bytes>(v113))
                            }
                        }
                    }
                    Adt24::C3 => {
                        let m388 = f64(v0.clone(), v103)?;
                        match m388 {
                            Ok(_) => {
                                Ok(Ok::<Adt28, Bytes>(v0))
                            }
                            Err(v115) => {
                                Ok(Err::<Adt28, Bytes>(v115))
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
        Adt17::C16(v129, r425) => {
            let v130 = (*r425).clone();
            let v131: bool = if v3 {
                match v1.clone() {
                    Adt27::C0(_, _, h389, _) => {
                        h389
                    }
                }
            } else {
                false
            };
            let v132: Result<(), Bytes> = {
                let m390 = v131;
                if m390 {
                    f76(match v1.clone() {
                        Adt27::C0(_, _, _, h391) => {
                            h391
                        }
                    }, v130.clone())?
                } else {
                    Ok::<(), Bytes>(())
                }
            };
            let m392 = v132;
            match m392 {
                Ok(_) => {
                    let m393 = f63(v0, v129.clone())?;
                    match m393 {
                        Ok(v134) => {
                            let v135: Adt28 = Adt28::C0(match v134.clone() {
                                Adt28::C0(h394, _, _, _, _) => {
                                    h394
                                }
                            }, match v134.clone() {
                                Adt28::C0(_, h395, _, _, _) => {
                                    h395
                                }
                            }, f77(match v134.clone() {
                                Adt28::C0(_, _, h396, _, _) => {
                                    h396
                                }
                            }, v129.clone()), match v134.clone() {
                                Adt28::C0(_, _, _, h397, _) => {
                                    h397
                                }
                            }, match v134.clone() {
                                Adt28::C0(_, _, _, _, h398) => {
                                    h398
                                }
                            });
                            let m399 = f29(v135.clone(), v1, v130, false)?;
                            match m399 {
                                Ok(v136) => {
                                    let m403 = f78(match v136.clone() {
                                        Adt28::C0(_, _, h400, _, _) => {
                                            h400
                                        }
                                    }, v129, match v135.clone() {
                                        Adt28::C0(_, _, h401, _, _) => {
                                            h401
                                        }
                                    }, match v134.clone() {
                                        Adt28::C0(_, h402, _, _, _) => {
                                            h402
                                        }
                                    });
                                    match m403 {
                                        None => {
                                            let m407 = f79(match v136.clone() {
                                                Adt28::C0(_, _, _, h404, _) => {
                                                    h404
                                                }
                                            }, match v135 {
                                                Adt28::C0(_, _, _, h405, _) => {
                                                    h405
                                                }
                                            }, match v134.clone() {
                                                Adt28::C0(_, h406, _, _, _) => {
                                                    h406
                                                }
                                            });
                                            match m407 {
                                                None => {
                                                    Ok(Ok::<Adt28, Bytes>(Adt28::C0(match v136.clone() {
                                                        Adt28::C0(h408, _, _, _, _) => {
                                                            h408
                                                        }
                                                    }, match v136.clone() {
                                                        Adt28::C0(_, h409, _, _, _) => {
                                                            h409
                                                        }
                                                    }, match v134.clone() {
                                                        Adt28::C0(_, _, h410, _, _) => {
                                                            h410
                                                        }
                                                    }, match v134 {
                                                        Adt28::C0(_, _, _, h411, _) => {
                                                            h411
                                                        }
                                                    }, match v136 {
                                                        Adt28::C0(_, _, _, _, h412) => {
                                                            h412
                                                        }
                                                    })))
                                                }
                                                Some(v138) => {
                                                    Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("ownership: a component of ")), f4(f30(f31(v138)?), f1(Str::lit(" is moved in the body of a loop that runs again"))))))
                                                }
                                            }
                                        }
                                        Some(v137) => {
                                            Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("ownership: ")), f4(f30(f31(v137)?), f1(Str::lit(" is moved in the body of a loop that runs again"))))))
                                        }
                                    }
                                }
                                Err(v139) => {
                                    Ok(Err::<Adt28, Bytes>(v139))
                                }
                            }
                        }
                        Err(v140) => {
                            Ok(Err::<Adt28, Bytes>(v140))
                        }
                    }
                }
                Err(v141) => {
                    Ok(Err::<Adt28, Bytes>(v141))
                }
            }
        }
    }
}

pub fn f48(p432: List<(Adt12, u64)>, v1: Adt26, p433: Adt17) -> R<Result<List<(Adt12, u64)>, Bytes>> {
    let m431 = (p432, p433);
    {
        let mut m431 = m431;
        loop {
            let step = {
                let (v0, v2) = m431;
                let m426 = v2;
                match m426 {
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
                    Adt17::C14(v40, v41, v42, r429, _, r430) => {
                        let v43 = (*r429).clone();
                        let v45 = (*r430).clone();
                        let v46: List<(Adt12, u64)> = {
                            let m427 = v41;
                            if m427 {
                                f50(v0, v40, v42)
                            } else {
                                v0
                            }
                        };
                        let m428 = f49(v46, v1.clone(), v43, false)?;
                        match m428 {
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
                Ok(next) => { m431 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f49(v0: List<(Adt12, u64)>, v1: Adt26, v2: Adt17, v3: bool) -> R<Result<List<(Adt12, u64)>, Bytes>> {
    let m434 = v2;
    match m434 {
        Adt17::C0(_, _) => {
            Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0))
        }
        Adt17::C1(_, _, _) => {
            Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0))
        }
        Adt17::C2(_, r444, _) => {
            let v10 = (*r444).clone();
            f49(v0, v1, v10, false)
        }
        Adt17::C3(_, r445, _, _) => {
            let v13 = (*r445).clone();
            f80(v0, v1, v13)
        }
        Adt17::C4(v16, r446, v18, _) => {
            let v17 = (*r446).clone();
            let m435 = f81(v0.clone(), v16.clone());
            match m435 {
                None => {
                    Ok(Err::<List<(Adt12, u64)>, Bytes>(f4(f30(f31(v16)?), f1(Str::lit(" is applied without a function type")))))
                }
                Some(v20) => {
                    let m437 = f20(match v1.clone() {
                        Adt26::C0(_, _, h436, _) => {
                            h436
                        }
                    }, v20);
                    match m437 {
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
                            let m438 = v22;
                            if m438 {
                                let m439 = v23;
                                if m439 {
                                    f80(v0, v1, v17)
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
                                f80(v0, v1, v17)
                            }
                        }
                    }
                }
            }
        }
        Adt17::C5(_, r447, _) => {
            let v25 = (*r447).clone();
            f80(v0, v1, v25)
        }
        Adt17::C6(r448, r449, _) => {
            let v27 = (*r448).clone();
            let v28 = (*r449).clone();
            let m440 = f49(v0, v1.clone(), v27, false)?;
            match m440 {
                Ok(v30) => {
                    f49(v30, v1, v28, false)
                }
                Err(v31) => {
                    Ok(Err::<List<(Adt12, u64)>, Bytes>(v31))
                }
            }
        }
        Adt17::C7(r450, r451, r452, _) => {
            let v32 = (*r450).clone();
            let v33 = (*r451).clone();
            let v34 = (*r452).clone();
            let m441 = f49(v0, v1.clone(), v32, false)?;
            match m441 {
                Ok(v36) => {
                    let m442 = f32(v36, v1.clone(), v33, v3)?;
                    match m442 {
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
        Adt17::C8(r453, r454, _) => {
            let v40 = (*r453).clone();
            let v41 = (*r454).clone();
            let m443 = f49(v0, v1.clone(), v40, false)?;
            match m443 {
                Ok(v43) => {
                    f82(v43, v1, v41, v3)
                }
                Err(v44) => {
                    Ok(Err::<List<(Adt12, u64)>, Bytes>(v44))
                }
            }
        }
        Adt17::C9(r455) => {
            let v45 = (*r455).clone();
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
        Adt17::C16(_, r456) => {
            let v63 = (*r456).clone();
            f32(v0, v1, v63, false)
        }
    }
}

pub fn f50(v0: List<(Adt12, u64)>, v1: Adt14, v2: Adt2) -> List<(Adt12, u64)> {
    let m457 = f83(v1);
    match m457 {
        None => {
            v0
        }
        Some(v3) => {
            let m458 = f84(v2);
            match m458 {
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
    if f85(v0, v1, 48u8) {
        Ok(Some({
            let a459 = v1;
            let a460 = 1u64;
            nat_add(a459, a460)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f52(v0: Adt3) -> bool {
    let m461 = v0;
    match m461 {
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
    let m462 = v0;
    match m462 {
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

pub fn f54(v0: Fn3, p466: bool, p467: List<Adt11>) -> bool {
    let m465 = (p466, p467);
    {
        let mut m465 = m465;
        loop {
            let step = {
                let (v1, v2) = m465;
                let m463 = v2;
                match m463.uncons() {
                    None => {
                        Err::<(bool, List<Adt11>), bool>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<Adt11>), bool>(({
                            let c464: Fn3 = v0.clone();
                            c464.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m465 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f55(v0: bool, v1: Adt11) -> bool {
    let m468 = v1;
    match m468 {
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
    f86(Fn4::F87(v1), false, v0)
}

pub fn f57(v0: Adt26, v1: u64, v2: u64) -> Option<u64> {
    let m470 = f18(match v0 {
        Adt26::C0(h469, _, _, _) => {
            h469
        }
    }, v1);
    match m470 {
        None => {
            None::<u64>
        }
        Some(v3) => {
            f88(v3, v2)
        }
    }
}

pub fn f58(v0: Adt14) -> R<u64> {
    let m471 = v0;
    match m471 {
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
        Adt14::C6(_, r474) => {
            let v10 = (*r474).clone();
            let v11: u64 = f58(v10)?;
            let a472 = 1u64;
            let a473 = v11;
            nat_add(a472, a473)
        }
    }
}

pub fn f59(v0: Adt14, v1: Adt17) -> Option<Adt12> {
    let m475 = v0;
    match m475 {
        Adt14::C0(_) => {
            None::<Adt12>
        }
        Adt14::C1(_) => {
            None::<Adt12>
        }
        Adt14::C2(r479) => {
            let v4 = (*r479).clone();
            let m476 = v1;
            match m476 {
                Adt17::C0(_, _) => {
                    None::<Adt12>
                }
                Adt17::C1(v7, v8, _) => {
                    let m477 = f89(v7);
                    if m477 {
                        let m478 = f90(v4);
                        if m478 {
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
    let m481 = f56(match v0.clone() {
        Adt28::C0(_, h480, _, _, _) => {
            h480
        }
    }, v1.clone());
    if m481 {
        let m483 = f56(match v0.clone() {
            Adt28::C0(_, _, h482, _, _) => {
                h482
            }
        }, v1.clone());
        if m483 {
            Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("ownership: a component of ")), f4(f30(f31(v1)?), f1(Str::lit(" is moved after it was moved"))))))
        } else {
            let v4: List<u64> = {
                let m485 = f91(match v0.clone() {
                    Adt28::C0(_, _, _, h484, _) => {
                        h484
                    }
                }, v1.clone());
                match m485 {
                    None => {
                        List::<u64>::nil()
                    }
                    Some(v3) => {
                        v3
                    }
                }
            };
            let m486 = f92(v4, 0u64, v2, f30(f31(v1.clone())?))?;
            match m486 {
                Ok(v5) => {
                    Ok(Ok::<Adt28, Bytes>(Adt28::C0(match v0.clone() {
                        Adt28::C0(h487, _, _, _, _) => {
                            h487
                        }
                    }, match v0.clone() {
                        Adt28::C0(_, h488, _, _, _) => {
                            h488
                        }
                    }, match v0.clone() {
                        Adt28::C0(_, _, h489, _, _) => {
                            h489
                        }
                    }, f93(match v0.clone() {
                        Adt28::C0(_, _, _, h490, _) => {
                            h490
                        }
                    }, v1, v5), match v0 {
                        Adt28::C0(_, _, _, _, h491) => {
                            h491
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
    Ok(f94({
        let m493 = f95(Adt30::C0(List::<Adt29>::nil(), List::<Bytes>::nil(), v0), v1)?;
        match m493 {
            Adt30::C0(h492, _, _) => {
                h492
            }
        }
    }))
}

pub fn f62(v0: Adt27, v1: Adt28, v2: Bytes) -> Result<Adt28, Bytes> {
    let m495 = match v0 {
        Adt27::C0(h494, _, _, _) => {
            h494
        }
    };
    if m495 {
        Err::<Adt28, Bytes>(f4(f1(Str::lit("hidden allocation: rust-core renders ")), f4(v2, f1(Str::lit(", which requires heap allocation")))))
    } else {
        Ok::<Adt28, Bytes>(v1)
    }
}

pub fn f63(v0: Adt28, v1: Adt12) -> R<Result<Adt28, Bytes>> {
    let m497 = f56(match v0.clone() {
        Adt28::C0(_, h496, _, _, _) => {
            h496
        }
    }, v1.clone());
    if m497 {
        let m499 = f96(match v0.clone() {
            Adt28::C0(_, _, _, h498, _) => {
                h498
            }
        }, v1.clone());
        if m499 {
            Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("ownership: ")), f4(f30(f31(v1)?), f1(Str::lit(" is moved after a component was moved out of it"))))))
        } else {
            let m501 = f56(match v0.clone() {
                Adt28::C0(_, _, h500, _, _) => {
                    h500
                }
            }, v1.clone());
            if m501 {
                Ok(Err::<Adt28, Bytes>(f4(f1(Str::lit("ownership: ")), f4(f30(f31(v1)?), f1(Str::lit(" is moved twice"))))))
            } else {
                Ok(Ok::<Adt28, Bytes>(Adt28::C0(match v0.clone() {
                    Adt28::C0(h502, _, _, _, _) => {
                        h502
                    }
                }, match v0.clone() {
                    Adt28::C0(_, h503, _, _, _) => {
                        h503
                    }
                }, List::cons(v1, match v0.clone() {
                    Adt28::C0(_, _, h504, _, _) => {
                        h504
                    }
                }), match v0.clone() {
                    Adt28::C0(_, _, _, h505, _) => {
                        h505
                    }
                }, match v0 {
                    Adt28::C0(_, _, _, _, h506) => {
                        h506
                    }
                })))
            }
        }
    } else {
        Ok(Err::<Adt28, Bytes>(f4(f30(f31(v1)?), f1(Str::lit(" is not bound")))))
    }
}

pub fn f64(v0: Adt28, v1: Adt12) -> R<Result<(), Bytes>> {
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
            Ok(Err::<(), Bytes>(f4(f1(Str::lit("ownership: ")), f4(f30(f31(v1)?), f1(Str::lit(" is read after it was moved"))))))
        } else {
            let m512 = f96(match v0 {
                Adt28::C0(_, _, _, h511, _) => {
                    h511
                }
            }, v1.clone());
            if m512 {
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
    let m513 = v0;
    match m513 {
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
            f97(v31)
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

pub fn f66(p519: Adt28, v1: Adt27, p520: Adt17) -> R<Result<Adt28, Bytes>> {
    let m518 = (p519, p520);
    {
        let mut m518 = m518;
        loop {
            let step = {
                let (v0, v2) = m518;
                let m514 = v2;
                match m514 {
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
                    Adt17::C13(r516, r517) => {
                        let v38 = (*r516).clone();
                        let v39 = (*r517).clone();
                        let m515 = f47(v0, v1.clone(), v38, false)?;
                        match m515 {
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
                Ok(next) => { m518 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f67(v0: Adt28, v1: Adt27, v2: bool, v3: bool, v4: bool, v5: Bytes) -> Result<Adt28, Bytes> {
    let v6: bool = if v4 {
        match v1 {
            Adt27::C0(_, _, h521, _) => {
                h521
            }
        }
    } else {
        false
    };
    let m522 = v2;
    if m522 {
        let m523 = v3;
        if m523 {
            Ok::<Adt28, Bytes>(Adt28::C0(match v0.clone() {
                Adt28::C0(h524, _, _, _, _) => {
                    h524
                }
            }, match v0.clone() {
                Adt28::C0(_, h525, _, _, _) => {
                    h525
                }
            }, match v0.clone() {
                Adt28::C0(_, _, h526, _, _) => {
                    h526
                }
            }, match v0 {
                Adt28::C0(_, _, _, h527, _) => {
                    h527
                }
            }, true))
        } else {
            let m528 = v6;
            if m528 {
                Ok::<Adt28, Bytes>(v0)
            } else {
                Err::<Adt28, Bytes>(f4(f1(Str::lit("arithmetic: the fallible call of ")), f4(v5, f1(Str::lit(" does not propagate its overflow")))))
            }
        }
    } else {
        let m529 = v3;
        if m529 {
            Err::<Adt28, Bytes>(f4(f1(Str::lit("arithmetic: the infallible call of ")), f4(v5, f1(Str::lit(" propagates a failure it cannot have")))))
        } else {
            Ok::<Adt28, Bytes>(v0)
        }
    }
}

pub fn f68(v0: List<Adt25>, v1: Adt22) -> Bytes {
    let m531 = f98(v0, match v1.clone() {
        Adt22::C0(h530, _) => {
            h530
        }
    });
    match m531 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            let a533 = match v2.clone() {
                Adt25::C0(_, _, h532, _, _) => {
                    h532
                }
            };
            let a534 = 0u64;
            if nat_eq(a533, a534) {
                match v2 {
                    Adt25::C0(_, h535, _, _, _) => {
                        h535
                    }
                }
            } else {
                let a537 = match v2.clone() {
                    Adt25::C0(_, _, h536, _, _) => {
                        h536
                    }
                };
                let a538 = 1u64;
                if nat_eq(a537, a538) {
                    f4(f1(Str::lit("fixed_")), f4(f99(f100(match v1 {
                        Adt22::C0(_, h539) => {
                            h539
                        }
                    })), f4(f1(Str::lit("::")), match v2 {
                        Adt25::C0(_, h540, _, _, _) => {
                            h540
                        }
                    })))
                } else {
                    f4(match v2 {
                        Adt25::C0(_, h541, _, _, _) => {
                            h541
                        }
                    }, f99(f100(match v1 {
                        Adt22::C0(_, h542) => {
                            h542
                        }
                    })))
                }
            }
        }
    }
}

pub fn f69(v0: List<Adt25>, v1: Adt22) -> bool {
    let m544 = f98(v0, match v1 {
        Adt22::C0(h543, _) => {
            h543
        }
    });
    match m544 {
        None => {
            false
        }
        Some(v2) => {
            match v2 {
                Adt25::C0(_, _, _, _, h545) => {
                    h545
                }
            }
        }
    }
}

pub fn f70(v0: List<Adt25>, v1: Adt22) -> bool {
    let m547 = f98(v0, match v1 {
        Adt22::C0(h546, _) => {
            h546
        }
    });
    match m547 {
        None => {
            false
        }
        Some(v2) => {
            match v2 {
                Adt25::C0(_, _, _, h548, _) => {
                    h548
                }
            }
        }
    }
}

pub fn f71(v0: Adt27, v1: Adt23, v2: u64) -> R<Result<(), Bytes>> {
    let m549 = v1;
    match m549 {
        Adt23::C0(v3) => {
            f26(v0, v3)
        }
        Adt23::C1 => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt23::C2(v4, v5) => {
            let m550 = f26(v0.clone(), v4)?;
            match m550 {
                Ok(_) => {
                    f26(v0, v5)
                }
                Err(v7) => {
                    Ok(Err::<(), Bytes>(v7))
                }
            }
        }
        Adt23::C3(v8, v9) => {
            let m551 = f26(v0.clone(), v8)?;
            match m551 {
                Ok(_) => {
                    f26(v0, v9)
                }
                Err(v11) => {
                    Ok(Err::<(), Bytes>(v11))
                }
            }
        }
        Adt23::C4(v12, v13) => {
            let m553 = f57(match v0 {
                Adt27::C0(_, h552, _, _) => {
                    h552
                }
            }, v12, v13);
            match m553 {
                None => {
                    Ok(Err::<(), Bytes>(f4(f1(Str::lit("Adt")), f4(f10(v12)?, f4(f1(Str::lit("::C")), f4(f10(v13)?, f1(Str::lit(" is not declared"))))))))
                }
                Some(v14) => {
                    let m556 = {
                        let a554 = v14;
                        let a555 = v2;
                        nat_eq(a554, a555)
                    };
                    if m556 {
                        Ok(Ok::<(), Bytes>(()))
                    } else {
                        Ok(Err::<(), Bytes>(f4(f1(Str::lit("Adt")), f4(f10(v12)?, f4(f1(Str::lit("::C")), f4(f10(v13)?, f4(f1(Str::lit(" receives ")), f4(f10(v2)?, f4(f1(Str::lit(" fields of ")), f10(v14)?)))))))))
                    }
                }
            }
        }
        Adt23::C5(v15, v16) => {
            let m558 = f101(match v0 {
                Adt27::C0(_, h557, _, _) => {
                    h557
                }
            }, v15, v16);
            match m558 {
                None => {
                    Ok(Err::<(), Bytes>(f4(f1(Str::lit("Fn")), f4(f10(v15)?, f4(f1(Str::lit("::F")), f4(f10(v16)?, f1(Str::lit(" is not declared"))))))))
                }
                Some(v17) => {
                    let m561 = {
                        let a559 = v17;
                        let a560 = v2;
                        nat_eq(a559, a560)
                    };
                    if m561 {
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
            let m562 = f44(v0.clone(), f1(Str::lit("an empty list")));
            match m562 {
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

pub fn f72(v0: Adt17) -> R<u64> {
    let m563 = v0;
    match m563 {
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
        Adt17::C13(_, r566) => {
            let v37 = (*r566).clone();
            let v38: u64 = f72(v37)?;
            let a564 = 1u64;
            let a565 = v38;
            nat_add(a564, a565)
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

pub fn f73(v0: List<Adt12>, v1: List<Adt12>) -> List<Adt12> {
    f102(Fn5::F103, v0, v1)
}

pub fn f74(v0: List<(Adt12, List<u64>)>, v1: List<(Adt12, List<u64>)>) -> List<(Adt12, List<u64>)> {
    f104(Fn6::F105, v0, v1)
}

pub fn f75(v0: Adt28, v1: Adt27, p584: Adt28, v3: List<Adt12>, p585: Adt17, v5: bool) -> R<Result<Adt28, Bytes>> {
    let m583 = (p584, p585);
    {
        let mut m583 = m583;
        loop {
            let step = {
                let (v2, v4) = m583;
                let m567 = v4;
                match m567 {
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
                    Adt17::C15(v49, r581, r582) => {
                        let v50 = (*r581).clone();
                        let v51 = (*r582).clone();
                        let v52: Adt28 = Adt28::C0(match v2.clone() {
                            Adt28::C0(h568, _, _, _, _) => {
                                h568
                            }
                        }, v3.clone(), match v0.clone() {
                            Adt28::C0(_, _, h569, _, _) => {
                                h569
                            }
                        }, match v0.clone() {
                            Adt28::C0(_, _, _, h570, _) => {
                                h570
                            }
                        }, match v2.clone() {
                            Adt28::C0(_, _, _, _, h571) => {
                                h571
                            }
                        });
                        let m573 = f45(v52, match v1.clone() {
                            Adt27::C0(_, h572, _, _) => {
                                h572
                            }
                        }, v49)?;
                        match m573 {
                            Ok(v53) => {
                                let m574 = f29(v53, v1.clone(), v50, v5)?;
                                match m574 {
                                    Ok(v54) => {
                                        let v55: Adt28 = Adt28::C0(match v54.clone() {
                                            Adt28::C0(h575, _, _, _, _) => {
                                                h575
                                            }
                                        }, v3.clone(), f73(match v2.clone() {
                                            Adt28::C0(_, _, h576, _, _) => {
                                                h576
                                            }
                                        }, match v54.clone() {
                                            Adt28::C0(_, _, h577, _, _) => {
                                                h577
                                            }
                                        }), f74(match v2 {
                                            Adt28::C0(_, _, _, h578, _) => {
                                                h578
                                            }
                                        }, match v54.clone() {
                                            Adt28::C0(_, _, _, h579, _) => {
                                                h579
                                            }
                                        }), match v54 {
                                            Adt28::C0(_, _, _, _, h580) => {
                                                h580
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
                Ok(next) => { m583 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f76(v0: List<Adt25>, v1: Adt17) -> R<Result<(), Bytes>> {
    let m586 = v1;
    match m586 {
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
        Adt17::C11(_, r587) => {
            let v36 = (*r587).clone();
            f106(v0, v36)
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

pub fn f77(v0: List<Adt12>, v1: Adt12) -> List<Adt12> {
    f102(Fn5::F107(v1), List::<Adt12>::nil(), v0)
}

pub fn f78(v0: List<Adt12>, v1: Adt12, v2: List<Adt12>, v3: List<Adt12>) -> Option<Adt12> {
    f108(Fn7::F109(v2, v3, v1), None::<Adt12>, v0)
}

pub fn f79(v0: List<(Adt12, List<u64>)>, v1: List<(Adt12, List<u64>)>, v2: List<Adt12>) -> Option<Adt12> {
    f110(Fn8::F111(v1, v2), None::<Adt12>, v0)
}

pub fn f80(p593: List<(Adt12, u64)>, v1: Adt26, p594: Adt17) -> R<Result<List<(Adt12, u64)>, Bytes>> {
    let m592 = (p593, p594);
    {
        let mut m592 = m592;
        loop {
            let step = {
                let (v0, v2) = m592;
                let m588 = v2;
                match m588 {
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
                    Adt17::C13(r590, r591) => {
                        let v38 = (*r590).clone();
                        let v39 = (*r591).clone();
                        let m589 = f49(v0, v1.clone(), v38, false)?;
                        match m589 {
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
                Ok(next) => { m592 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f81(v0: List<(Adt12, u64)>, v1: Adt12) -> Option<u64> {
    f112(Fn9::F113(v1), None::<u64>, v0)
}

pub fn f82(p600: List<(Adt12, u64)>, v1: Adt26, p601: Adt17, v3: bool) -> R<Result<List<(Adt12, u64)>, Bytes>> {
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
                    Adt17::C13(_, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C14(_, _, _, _, _, _) => {
                        Err::<(List<(Adt12, u64)>, Adt17), R<Result<List<(Adt12, u64)>, Bytes>>>(Ok(Ok::<List<(Adt12, u64)>, Bytes>(v0)))
                    }
                    Adt17::C15(_, r597, r598) => {
                        let v48 = (*r597).clone();
                        let v49 = (*r598).clone();
                        let m596 = f32(v0, v1.clone(), v48, v3)?;
                        match m596 {
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
                Ok(next) => { m599 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f83(v0: Adt14) -> Option<Adt12> {
    let m602 = v0;
    match m602 {
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

pub fn f84(v0: Adt2) -> Option<u64> {
    let m603 = v0;
    match m603 {
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
            let m604 = f6(v8);
            if m604 {
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

pub fn f85(v0: Bytes, v1: u64, v2: u8) -> bool {
    let m605 = f114(v0, v1);
    match m605 {
        None => {
            false
        }
        Some(v3) => {
            let a606 = v3;
            let a607 = v2;
            equal(a606, a607)
        }
    }
}

pub fn f86(v0: Fn4, p611: bool, p612: List<Adt12>) -> bool {
    let m610 = (p611, p612);
    {
        let mut m610 = m610;
        loop {
            let step = {
                let (v1, v2) = m610;
                let m608 = v2;
                match m608.uncons() {
                    None => {
                        Err::<(bool, List<Adt12>), bool>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<Adt12>), bool>(({
                            let c609: Fn4 = v0.clone();
                            c609.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m610 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f87(v0: Adt12, v1: bool, v2: Adt12) -> bool {
    let v3: bool = f115(v2, v0);
    if v1 {
        true
    } else {
        v3
    }
}

pub fn f88(v0: List<(u64, u64)>, v1: u64) -> Option<u64> {
    f116(Fn10::F117(v1), None::<u64>, v0)
}

pub fn f89(v0: Adt19) -> bool {
    let m613 = v0;
    match m613 {
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

pub fn f90(v0: Adt14) -> bool {
    let m614 = v0;
    match m614 {
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
        Adt14::C6(r615, r616) => {
            let v9 = (*r615).clone();
            let v10 = (*r616).clone();
            let v11: bool = f118(v9);
            let v12: bool = f90(v10);
            if v11 {
                true
            } else {
                v12
            }
        }
    }
}

pub fn f91(v0: List<(Adt12, List<u64>)>, v1: Adt12) -> Option<List<u64>> {
    f119(Fn11::F120(v1), None::<List<u64>>, v0)
}

pub fn f92(p627: List<u64>, p628: u64, p629: Adt14, v3: Bytes) -> R<Result<List<u64>, Bytes>> {
    let m626 = (p627, (p628, p629));
    {
        let mut m626 = m626;
        loop {
            let step = {
                let (v0, (v1, v2)) = m626;
                let m617 = v2;
                match m617 {
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
                    Adt14::C6(r624, r625) => {
                        let v12 = (*r624).clone();
                        let v13 = (*r625).clone();
                        let v14: bool = f118(v12);
                        let m618 = v14;
                        if m618 {
                            Ok::<(List<u64>, (u64, Adt14)), R<Result<List<u64>, Bytes>>>((v0, ({
                                let a619 = v1;
                                let a620 = 1u64;
                                nat_add(a619, a620)?
                            }, v13)))
                        } else {
                            let m621 = f121(v0.clone(), v1);
                            if m621 {
                                Err::<(List<u64>, (u64, Adt14)), R<Result<List<u64>, Bytes>>>(Ok(Err::<List<u64>, Bytes>(f4(f1(Str::lit("ownership: component ")), f4(f10(v1)?, f4(f1(Str::lit(" of ")), f4(v3.clone(), f1(Str::lit(" is moved twice")))))))))
                            } else {
                                Ok::<(List<u64>, (u64, Adt14)), R<Result<List<u64>, Bytes>>>((List::cons(v1, v0), ({
                                    let a622 = v1;
                                    let a623 = 1u64;
                                    nat_add(a622, a623)?
                                }, v13)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m626 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f93(v0: List<(Adt12, List<u64>)>, v1: Adt12, v2: List<u64>) -> List<(Adt12, List<u64>)> {
    List::cons((v1.clone(), v2), f122(v0, v1))
}

pub fn f94(v0: List<Adt29>) -> Bytes {
    let m630 = f123(Fn12::F124, None::<Bytes>, v0);
    match m630 {
        None => {
            f1(Str::lit("a read"))
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f95(v0: Adt30, v1: Adt17) -> R<Adt30> {
    let m631 = v1;
    match m631 {
        Adt17::C0(v2, v3) => {
            Ok(f125(v0, f126(v2.clone()), v3, f127(v2)))
        }
        Adt17::C1(v4, _, v6) => {
            Ok(f128(v0, f129(v4), v6))
        }
        Adt17::C2(v7, r640, v9) => {
            let v8 = (*r640).clone();
            let v10: Adt30 = f128(v0, f130(v7), v9);
            f95(v10, v8)
        }
        Adt17::C3(v11, r641, _, v14) => {
            let v12 = (*r641).clone();
            let v17: Adt30 = {
                let m632 = v11;
                match m632 {
                    Adt21::C0(_) => {
                        f128(v0, f1(Str::lit("call:function")), v14)
                    }
                    Adt21::C1(v16) => {
                        f125(v0.clone(), f4(f1(Str::lit("call:runtime:")), f131(f132(f68(match v0.clone() {
                            Adt30::C0(_, _, h633) => {
                                h633
                            }
                        }, v16.clone()))?)), v14, f133(match v0 {
                            Adt30::C0(_, _, h634) => {
                                h634
                            }
                        }, v16))
                    }
                }
            };
            f95(v17, v12)
        }
        Adt17::C4(_, r642, _, v21) => {
            let v19 = (*r642).clone();
            let v22: Adt30 = f128(v0, f1(Str::lit("apply")), v21);
            f95(v22, v19)
        }
        Adt17::C5(v23, r643, v25) => {
            let v24 = (*r643).clone();
            let v26: Adt30 = f128(f134(v0, v23.clone()), f4(f1(Str::lit("construct:")), f135(v23)), v25);
            f95(v26, v24)
        }
        Adt17::C6(r644, r645, v29) => {
            let v27 = (*r644).clone();
            let v28 = (*r645).clone();
            let v30: Adt30 = f128(v0, f1(Str::lit("construct:pair")), v29);
            let v31: Adt30 = f95(v30, v27)?;
            f95(v31, v28)
        }
        Adt17::C7(r646, r647, r648, v35) => {
            let v32 = (*r646).clone();
            let v33 = (*r647).clone();
            let v34 = (*r648).clone();
            let v36: Adt30 = f128(v0, f1(Str::lit("if")), v35);
            let v37: Adt30 = f95(v36, v32)?;
            let v38: Adt30 = f95(v37, v33)?;
            f95(v38, v34)
        }
        Adt17::C8(r649, r650, v41) => {
            let v39 = (*r649).clone();
            let v40 = (*r650).clone();
            let v42: Adt30 = {
                let m635 = f136(v40.clone());
                if m635 {
                    f128(v0, f1(Str::lit("match:empty")), v41)
                } else {
                    let m636 = f137(v39.clone());
                    if m636 {
                        f128(v0, f1(Str::lit("match:list")), v41)
                    } else {
                        f138(v0, v40.clone(), v41)
                    }
                }
            };
            let v43: Adt30 = f95(v42, v39)?;
            f95(v43, v40)
        }
        Adt17::C9(r651) => {
            let v44 = (*r651).clone();
            f95(v0, v44)
        }
        Adt17::C10(v45, _, v47) => {
            let m637 = f139(v45);
            if m637 {
                Ok(f128(v0, f1(Str::lit("match:nat")), v47))
            } else {
                Ok(v0)
            }
        }
        Adt17::C11(r652, r653) => {
            let v48 = (*r652).clone();
            let v49 = (*r653).clone();
            let v50: Adt30 = f95(v0, v48)?;
            f95(v50, v49)
        }
        Adt17::C12 => {
            Ok(v0)
        }
        Adt17::C13(r654, r655) => {
            let v51 = (*r654).clone();
            let v52 = (*r655).clone();
            let v53: Adt30 = f95(v0, v51)?;
            f95(v53, v52)
        }
        Adt17::C14(v54, v55, v56, r656, v58, r657) => {
            let v57 = (*r656).clone();
            let v59 = (*r657).clone();
            let v60: Bytes = {
                let m638 = f140(v54);
                if m638 {
                    f1(Str::lit("destructure:pair"))
                } else {
                    f1(Str::lit("let"))
                }
            };
            let v61: Adt30 = {
                let m639 = v55;
                if m639 {
                    f141(v0, v56)
                } else {
                    v0
                }
            };
            let v62: Adt30 = f128(v61, v60, v58);
            let v63: Adt30 = f95(v62, v57)?;
            f95(v63, v59)
        }
        Adt17::C15(_, r658, r659) => {
            let v65 = (*r658).clone();
            let v66 = (*r659).clone();
            let v67: Adt30 = f95(v0, v65)?;
            f95(v67, v66)
        }
        Adt17::C16(_, r660) => {
            let v69 = (*r660).clone();
            f95(v0, v69)
        }
    }
}

pub fn f96(v0: List<(Adt12, List<u64>)>, v1: Adt12) -> bool {
    let m661 = f91(v0, v1);
    match m661 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f97(v0: Adt24) -> bool {
    let m662 = v0;
    match m662 {
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

pub fn f98(v0: List<Adt25>, v1: u64) -> Option<Adt25> {
    f142(Fn13::F143(v1), None::<Adt25>, v0)
}

pub fn f99(v0: Adt4) -> Bytes {
    let m663 = v0;
    match m663 {
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

pub fn f100(v0: Option<Adt4>) -> Adt4 {
    let m664 = v0;
    match m664 {
        None => {
            Adt4::C0
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f101(v0: Adt26, v1: u64, v2: u64) -> Option<u64> {
    let m666 = f18(match v0 {
        Adt26::C0(_, h665, _, _) => {
            h665
        }
    }, v1);
    match m666 {
        None => {
            None::<u64>
        }
        Some(v3) => {
            f88(v3, v2)
        }
    }
}

pub fn f102(v0: Fn5, p670: List<Adt12>, p671: List<Adt12>) -> List<Adt12> {
    let m669 = (p670, p671);
    {
        let mut m669 = m669;
        loop {
            let step = {
                let (v1, v2) = m669;
                let m667 = v2;
                match m667.uncons() {
                    None => {
                        Err::<(List<Adt12>, List<Adt12>), List<Adt12>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt12>, List<Adt12>), List<Adt12>>(({
                            let c668: Fn5 = v0.clone();
                            c668.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m669 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f103(v0: List<Adt12>, v1: Adt12) -> List<Adt12> {
    let v2: bool = f56(v0.clone(), v1.clone());
    if v2 {
        v0
    } else {
        List::cons(v1, v0)
    }
}

pub fn f104(v0: Fn6, p675: List<(Adt12, List<u64>)>, p676: List<(Adt12, List<u64>)>) -> List<(Adt12, List<u64>)> {
    let m674 = (p675, p676);
    {
        let mut m674 = m674;
        loop {
            let step = {
                let (v1, v2) = m674;
                let m672 = v2;
                match m672.uncons() {
                    None => {
                        Err::<(List<(Adt12, List<u64>)>, List<(Adt12, List<u64>)>), List<(Adt12, List<u64>)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Adt12, List<u64>)>, List<(Adt12, List<u64>)>), List<(Adt12, List<u64>)>>(({
                            let c673: Fn6 = v0.clone();
                            c673.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m674 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f105(v0: List<(Adt12, List<u64>)>, v1: (Adt12, List<u64>)) -> List<(Adt12, List<u64>)> {
    let m678 = f91(v0.clone(), {
        let (h677, _) = v1.clone();
        h677
    });
    match m678 {
        None => {
            f93(v0, {
                let (h681, _) = v1;
                h681
            }, {
                let (_, h682) = v1;
                h682
            })
        }
        Some(v2) => {
            f93(v0, {
                let (h679, _) = v1;
                h679
            }, f144(v2, {
                let (_, h680) = v1;
                h680
            }))
        }
    }
}

pub fn f106(v0: List<Adt25>, v1: Adt17) -> R<Result<(), Bytes>> {
    let m683 = v1;
    match m683 {
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
        Adt17::C5(v18, r686, _) => {
            let v19 = (*r686).clone();
            let m684 = f145(v18);
            if m684 {
                f146(v0, v19)
            } else {
                Ok(Ok::<(), Bytes>(()))
            }
        }
        Adt17::C6(_, _, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt17::C7(_, r687, r688, _) => {
            let v25 = (*r687).clone();
            let v26 = (*r688).clone();
            let m685 = f76(v0.clone(), v25)?;
            match m685 {
                Ok(_) => {
                    f76(v0, v26)
                }
                Err(v29) => {
                    Ok(Err::<(), Bytes>(v29))
                }
            }
        }
        Adt17::C8(_, r689, _) => {
            let v31 = (*r689).clone();
            f147(v0, v31)
        }
        Adt17::C9(r690) => {
            let v33 = (*r690).clone();
            f76(v0, v33)
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

pub fn f107(v0: Adt12, v1: List<Adt12>, v2: Adt12) -> List<Adt12> {
    let v3: bool = f115(v2.clone(), v0);
    if v3 {
        v1
    } else {
        List::cons(v2, v1)
    }
}

pub fn f108(v0: Fn7, p694: Option<Adt12>, p695: List<Adt12>) -> Option<Adt12> {
    let m693 = (p694, p695);
    {
        let mut m693 = m693;
        loop {
            let step = {
                let (v1, v2) = m693;
                let m691 = v2;
                match m691.uncons() {
                    None => {
                        Err::<(Option<Adt12>, List<Adt12>), Option<Adt12>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Adt12>, List<Adt12>), Option<Adt12>>(({
                            let c692: Fn7 = v0.clone();
                            c692.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m693 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f109(v0: List<Adt12>, v1: List<Adt12>, v2: Adt12, v3: Option<Adt12>, v4: Adt12) -> Option<Adt12> {
    let v5: bool = f115(v4.clone(), v2);
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
        f148(v3, v4)
    } else {
        v3
    }
}

pub fn f110(v0: Fn8, p699: Option<Adt12>, p700: List<(Adt12, List<u64>)>) -> Option<Adt12> {
    let m698 = (p699, p700);
    {
        let mut m698 = m698;
        loop {
            let step = {
                let (v1, v2) = m698;
                let m696 = v2;
                match m696.uncons() {
                    None => {
                        Err::<(Option<Adt12>, List<(Adt12, List<u64>)>), Option<Adt12>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Adt12>, List<(Adt12, List<u64>)>), Option<Adt12>>(({
                            let c697: Fn8 = v0.clone();
                            c697.apply(v1, v3)
                        }, v4))
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

pub fn f111(v0: List<(Adt12, List<u64>)>, v1: List<Adt12>, v2: Option<Adt12>, v3: (Adt12, List<u64>)) -> Option<Adt12> {
    let v4: bool = f56(v1, {
        let (h701, _) = v3.clone();
        h701
    });
    let v5: bool = f149(v0, {
        let (h702, _) = v3.clone();
        h702
    }, {
        let (_, h703) = v3;
        h703
    });
    let v6: bool = if v4 {
        !v5
    } else {
        false
    };
    if v6 {
        f148(v2, {
            let (h704, _) = v3;
            h704
        })
    } else {
        v2
    }
}

pub fn f112(v0: Fn9, p708: Option<u64>, p709: List<(Adt12, u64)>) -> Option<u64> {
    let m707 = (p708, p709);
    {
        let mut m707 = m707;
        loop {
            let step = {
                let (v1, v2) = m707;
                let m705 = v2;
                match m705.uncons() {
                    None => {
                        Err::<(Option<u64>, List<(Adt12, u64)>), Option<u64>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<u64>, List<(Adt12, u64)>), Option<u64>>(({
                            let c706: Fn9 = v0.clone();
                            c706.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m707 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f113(v0: Adt12, v1: Option<u64>, v2: (Adt12, u64)) -> Option<u64> {
    let m710 = v1;
    match m710 {
        None => {
            let v4: bool = f115({
                let (h711, _) = v2;
                h711
            }, v0);
            if v4 {
                Some({
                    let (_, h712) = v2;
                    h712
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

pub fn f114(v0: Bytes, v1: u64) -> Option<u8> {
    let a713 = v0;
    let a714 = v1;
    index_bytes(a713, a714)
}

pub fn f115(v0: Adt12, v1: Adt12) -> bool {
    let a715 = f150(v0.clone());
    let a716 = f150(v1.clone());
    if nat_eq(a715, a716) {
        let a717 = f151(v0.clone());
        let a718 = f151(v1.clone());
        if nat_eq(a717, a718) {
            let a719 = f152(v0);
            let a720 = f152(v1);
            equal(a719, a720)
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f116(v0: Fn10, p724: Option<u64>, p725: List<(u64, u64)>) -> Option<u64> {
    let m723 = (p724, p725);
    {
        let mut m723 = m723;
        loop {
            let step = {
                let (v1, v2) = m723;
                let m721 = v2;
                match m721.uncons() {
                    None => {
                        Err::<(Option<u64>, List<(u64, u64)>), Option<u64>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<u64>, List<(u64, u64)>), Option<u64>>(({
                            let c722: Fn10 = v0.clone();
                            c722.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m723 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f117(v0: u64, v1: Option<u64>, v2: (u64, u64)) -> Option<u64> {
    let m726 = v1;
    match m726 {
        None => {
            let a728 = {
                let (h727, _) = v2;
                h727
            };
            let a729 = v0;
            if nat_eq(a728, a729) {
                Some({
                    let (_, h730) = v2;
                    h730
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

pub fn f118(v0: Adt14) -> bool {
    let m731 = v0;
    match m731 {
        Adt14::C0(v1) => {
            let m732 = v1;
            match m732 {
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

pub fn f119(v0: Fn11, p736: Option<List<u64>>, p737: List<(Adt12, List<u64>)>) -> Option<List<u64>> {
    let m735 = (p736, p737);
    {
        let mut m735 = m735;
        loop {
            let step = {
                let (v1, v2) = m735;
                let m733 = v2;
                match m733.uncons() {
                    None => {
                        Err::<(Option<List<u64>>, List<(Adt12, List<u64>)>), Option<List<u64>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<List<u64>>, List<(Adt12, List<u64>)>), Option<List<u64>>>(({
                            let c734: Fn11 = v0.clone();
                            c734.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m735 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f120(v0: Adt12, v1: Option<List<u64>>, v2: (Adt12, List<u64>)) -> Option<List<u64>> {
    let m738 = v1;
    match m738 {
        None => {
            let v4: bool = f115({
                let (h739, _) = v2;
                h739
            }, v0);
            if v4 {
                Some({
                    let (_, h740) = v2;
                    h740
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

pub fn f121(v0: List<u64>, v1: u64) -> bool {
    f153(Fn14::F154(v1), false, v0)
}

pub fn f122(v0: List<(Adt12, List<u64>)>, v1: Adt12) -> List<(Adt12, List<u64>)> {
    f104(Fn6::F155(v1), List::<(Adt12, List<u64>)>::nil(), v0)
}

pub fn f123(v0: Fn12, p744: Option<Bytes>, p745: List<Adt29>) -> Option<Bytes> {
    let m743 = (p744, p745);
    {
        let mut m743 = m743;
        loop {
            let step = {
                let (v1, v2) = m743;
                let m741 = v2;
                match m741.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Adt29>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Adt29>), Option<Bytes>>(({
                            let c742: Fn12 = v0.clone();
                            c742.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m743 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f124(v0: Option<Bytes>, v1: Adt29) -> Option<Bytes> {
    let m746 = v0;
    match m746 {
        None => {
            Some(match v1 {
                Adt29::C0(h747, _, _, _) => {
                    h747
                }
            })
        }
        Some(v2) => {
            let m749 = f156(match v1.clone() {
                Adt29::C0(h748, _, _, _) => {
                    h748
                }
            }, v2.clone());
            if m749 {
                Some(match v1 {
                    Adt29::C0(h750, _, _, _) => {
                        h750
                    }
                })
            } else {
                Some(v2)
            }
        }
    }
}

pub fn f125(v0: Adt30, v1: Bytes, v2: Adt9, v3: Option<Adt4>) -> Adt30 {
    let v4: Adt29 = Adt29::C0(v1, match v2.clone() {
        Adt9::C0(h751, _) => {
            h751
        }
    }, match v2 {
        Adt9::C0(_, h752) => {
            h752
        }
    }, v3);
    let m754 = f157(match v0.clone() {
        Adt30::C0(h753, _, _) => {
            h753
        }
    }, v4.clone());
    if m754 {
        v0
    } else {
        Adt30::C0(List::cons(v4, match v0.clone() {
            Adt30::C0(h755, _, _) => {
                h755
            }
        }), match v0.clone() {
            Adt30::C0(_, h756, _) => {
                h756
            }
        }, match v0 {
            Adt30::C0(_, _, h757) => {
                h757
            }
        })
    }
}

pub fn f126(v0: Adt18) -> Bytes {
    let m758 = v0;
    match m758 {
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

pub fn f127(v0: Adt18) -> Option<Adt4> {
    let m759 = v0;
    match m759 {
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

pub fn f128(v0: Adt30, v1: Bytes, v2: Adt9) -> Adt30 {
    f125(v0, v1, v2, None::<Adt4>)
}

pub fn f129(v0: Adt19) -> Bytes {
    let m760 = v0;
    match m760 {
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

pub fn f130(v0: Adt20) -> Bytes {
    let m761 = v0;
    match m761 {
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

pub fn f131(v0: Bytes) -> Bytes {
    let m762 = f158(v0.clone(), f1(Str::lit("checked_neg_")));
    if m762 {
        f1(Str::lit("checked_neg"))
    } else {
        let m763 = f158(v0.clone(), f1(Str::lit("format_")));
        if m763 {
            f1(Str::lit("format_decimal"))
        } else {
            let m764 = f158(v0.clone(), f1(Str::lit("parse_")));
            if m764 {
                f1(Str::lit("parse_decimal"))
            } else {
                let m765 = f158(v0.clone(), f1(Str::lit("append_")));
                if m765 {
                    f1(Str::lit("append"))
                } else {
                    let m766 = f158(v0.clone(), f1(Str::lit("length_")));
                    if m766 {
                        f1(Str::lit("length"))
                    } else {
                        let m767 = f158(v0.clone(), f1(Str::lit("index_")));
                        if m767 {
                            f1(Str::lit("index"))
                        } else {
                            let m768 = f158(v0.clone(), f1(Str::lit("slice_")));
                            if m768 {
                                f1(Str::lit("slice"))
                            } else {
                                let m771 = {
                                    let a769 = v0.clone();
                                    let a770 = f1(Str::lit("nat_succ"));
                                    equal(a769, a770)
                                };
                                if m771 {
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

pub fn f132(v0: Bytes) -> R<Bytes> {
    let m772 = f159(v0.clone(), f1(Str::lit("::")))?;
    match m772 {
        None => {
            Ok(v0)
        }
        Some(v1) => {
            Ok(f40(v0.clone(), {
                let a773 = v1;
                let a774 = 2u64;
                nat_add(a773, a774)?
            }, {
                let a778 = {
                    let a775 = v0;
                    length_bytes(a775)
                };
                let a779 = {
                    let a776 = v1;
                    let a777 = 2u64;
                    nat_add(a776, a777)?
                };
                nat_sub(a778, a779)
            }))
        }
    }
}

pub fn f133(v0: List<Adt25>, v1: Adt22) -> Option<Adt4> {
    let m781 = f98(v0, match v1.clone() {
        Adt22::C0(h780, _) => {
            h780
        }
    });
    match m781 {
        None => {
            None::<Adt4>
        }
        Some(v2) => {
            let a783 = match v2 {
                Adt25::C0(_, _, h782, _, _) => {
                    h782
                }
            };
            let a784 = 0u64;
            if nat_eq(a783, a784) {
                None::<Adt4>
            } else {
                Some(f100(match v1 {
                    Adt22::C0(_, h785) => {
                        h785
                    }
                }))
            }
        }
    }
}

pub fn f134(v0: Adt30, v1: Adt23) -> Adt30 {
    let m786 = v1;
    match m786 {
        Adt23::C0(v2) => {
            f141(v0, v2)
        }
        Adt23::C1 => {
            v0
        }
        Adt23::C2(v3, v4) => {
            f141(f141(v0, v3), v4)
        }
        Adt23::C3(v5, v6) => {
            f141(f141(v0, v5), v6)
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
            f141(v0, v11)
        }
    }
}

pub fn f135(v0: Adt23) -> Bytes {
    let m787 = v0;
    match m787 {
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

pub fn f136(v0: Adt17) -> bool {
    let m788 = v0;
    match m788 {
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

pub fn f137(v0: Adt17) -> bool {
    let m789 = v0;
    match m789 {
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
            f160(v31)
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

pub fn f138(p794: Adt30, p795: Adt17, v2: Adt9) -> Adt30 {
    let m793 = (p794, p795);
    {
        let mut m793 = m793;
        loop {
            let step = {
                let (v0, v1) = m793;
                let m790 = v1;
                match m790 {
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
                    Adt17::C15(v46, _, r792) => {
                        let v48 = (*r792).clone();
                        let m791 = f161(v46);
                        match m791 {
                            None => {
                                Ok::<(Adt30, Adt17), Adt30>((v0, v48))
                            }
                            Some(v49) => {
                                Ok::<(Adt30, Adt17), Adt30>((f128(v0, v49, v2.clone()), v48))
                            }
                        }
                    }
                    Adt17::C16(_, _) => {
                        Err::<(Adt30, Adt17), Adt30>(v0)
                    }
                }
            };
            match step {
                Ok(next) => { m793 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f139(v0: Adt24) -> bool {
    let m796 = v0;
    match m796 {
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

pub fn f140(v0: Adt14) -> bool {
    let m797 = v0;
    match m797 {
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

pub fn f141(v0: Adt30, v1: Adt2) -> Adt30 {
    let m798 = v1;
    match m798 {
        Adt2::C0(v2) => {
            f162(v0, f4(f1(Str::lit("type:")), f163(v2)))
        }
        Adt2::C1(_) => {
            f162(v0, f1(Str::lit("type:fixed")))
        }
        Adt2::C2(v4, r799) => {
            let v5 = (*r799).clone();
            let v6: Adt30 = f162(v0, f4(f1(Str::lit("type:")), f164(v4)));
            f141(v6, v5)
        }
        Adt2::C3(v7, r800, r801) => {
            let v8 = (*r800).clone();
            let v9 = (*r801).clone();
            let v10: Adt30 = f162(v0, f4(f1(Str::lit("type:")), f165(v7)));
            let v11: Adt30 = f141(v10, v8);
            f141(v11, v9)
        }
        Adt2::C4(v12, _) => {
            f162(v0, f4(f1(Str::lit("type:")), f166(v12)))
        }
        Adt2::C5 => {
            v0
        }
        Adt2::C6(r802, r803) => {
            let v14 = (*r802).clone();
            let v15 = (*r803).clone();
            let v16: Adt30 = f141(v0, v14);
            f141(v16, v15)
        }
    }
}

pub fn f142(v0: Fn13, p807: Option<Adt25>, p808: List<Adt25>) -> Option<Adt25> {
    let m806 = (p807, p808);
    {
        let mut m806 = m806;
        loop {
            let step = {
                let (v1, v2) = m806;
                let m804 = v2;
                match m804.uncons() {
                    None => {
                        Err::<(Option<Adt25>, List<Adt25>), Option<Adt25>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Adt25>, List<Adt25>), Option<Adt25>>(({
                            let c805: Fn13 = v0.clone();
                            c805.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m806 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f143(v0: u64, v1: Option<Adt25>, v2: Adt25) -> Option<Adt25> {
    let m809 = v1;
    match m809 {
        None => {
            let a811 = match v2.clone() {
                Adt25::C0(h810, _, _, _, _) => {
                    h810
                }
            };
            let a812 = v0;
            if nat_eq(a811, a812) {
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

pub fn f144(v0: List<u64>, v1: List<u64>) -> List<u64> {
    f167(Fn15::F168, v0, v1)
}

pub fn f145(v0: Adt23) -> bool {
    let m813 = v0;
    match m813 {
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

pub fn f146(v0: List<Adt25>, p819: Adt17) -> R<Result<(), Bytes>> {
    let m818 = p819;
    {
        let mut m818 = m818;
        loop {
            let step = {
                let v1 = m818;
                let m814 = v1;
                match m814 {
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
                    Adt17::C13(r816, r817) => {
                        let v37 = (*r816).clone();
                        let v38 = (*r817).clone();
                        let m815 = f169(v37.clone());
                        if m815 {
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
                Ok(next) => { m818 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f147(v0: List<Adt25>, p825: Adt17) -> R<Result<(), Bytes>> {
    let m824 = p825;
    {
        let mut m824 = m824;
        loop {
            let step = {
                let v1 = m824;
                let m820 = v1;
                match m820 {
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
                    Adt17::C15(_, r822, r823) => {
                        let v46 = (*r822).clone();
                        let v47 = (*r823).clone();
                        let m821 = f76(v0.clone(), v46)?;
                        match m821 {
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
                Ok(next) => { m824 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f148(v0: Option<Adt12>, v1: Adt12) -> Option<Adt12> {
    let m826 = v0;
    match m826 {
        None => {
            Some(v1)
        }
        Some(v2) => {
            let m827 = f170(v1.clone(), v2.clone());
            if m827 {
                Some(v1)
            } else {
                Some(v2)
            }
        }
    }
}

pub fn f149(v0: List<(Adt12, List<u64>)>, v1: Adt12, v2: List<u64>) -> bool {
    let m828 = f91(v0, v1);
    match m828 {
        None => {
            false
        }
        Some(v3) => {
            let v4: bool = f171(v2.clone(), v3.clone());
            let v5: bool = f171(v3, v2);
            if v4 {
                v5
            } else {
                false
            }
        }
    }
}

pub fn f150(v0: Adt12) -> u64 {
    let m829 = v0;
    match m829 {
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

pub fn f151(v0: Adt12) -> u64 {
    let m830 = v0;
    match m830 {
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

pub fn f152(v0: Adt12) -> Bytes {
    let m831 = v0;
    match m831 {
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

pub fn f153(v0: Fn14, p835: bool, p836: List<u64>) -> bool {
    let m834 = (p835, p836);
    {
        let mut m834 = m834;
        loop {
            let step = {
                let (v1, v2) = m834;
                let m832 = v2;
                match m832.uncons() {
                    None => {
                        Err::<(bool, List<u64>), bool>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<u64>), bool>(({
                            let c833: Fn14 = v0.clone();
                            c833.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m834 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f154(v0: u64, v1: bool, v2: u64) -> bool {
    if v1 {
        true
    } else {
        let a837 = v2;
        let a838 = v0;
        nat_eq(a837, a838)
    }
}

pub fn f155(v0: Adt12, v1: List<(Adt12, List<u64>)>, v2: (Adt12, List<u64>)) -> List<(Adt12, List<u64>)> {
    let v3: bool = f115({
        let (h839, _) = v2.clone();
        h839
    }, v0);
    if v3 {
        v1
    } else {
        List::cons(v2, v1)
    }
}

pub fn f156(v0: Bytes, v1: Bytes) -> bool {
    let a844 = {
        let a840 = v0;
        let a841 = v1;
        compare_bytes(a840, a841)
    };
    let a845 = {
        let a842 = Bytes::lit(&[]);
        let a843 = Bytes::lit(&[0x01]);
        compare_bytes(a842, a843)
    };
    equal(a844, a845)
}

pub fn f157(v0: List<Adt29>, v1: Adt29) -> bool {
    f172(Fn16::F173(v1), false, v0)
}

pub fn f158(v0: Bytes, v1: Bytes) -> bool {
    let m850 = {
        let a847 = v0;
        let a848 = 0u64;
        let a849 = {
            let a846 = v1.clone();
            length_bytes(a846)
        };
        slice_bytes(a847, a848, a849)
    };
    match m850 {
        None => {
            false
        }
        Some(v2) => {
            let a851 = v2;
            let a852 = v1;
            equal(a851, a852)
        }
    }
}

pub fn f159(v0: Bytes, v1: Bytes) -> R<Option<u64>> {
    let v2: u64 = {
        let a853 = v0.clone();
        length_bytes(a853)
    };
    let v3: u64 = {
        let a854 = v1.clone();
        length_bytes(a854)
    };
    let v4: u64 = {
        let (h855, _) = f38(Fn2::F174(v1.clone(), v3, v0.clone(), v2), {
            let a856 = v2;
            let a857 = 1u64;
            nat_add(a856, a857)?
        }, 0u64)?;
        h855
    };
    if f175(v0, v1, v4) {
        Ok(Some(v4))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f160(v0: Adt24) -> bool {
    let m858 = v0;
    match m858 {
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

pub fn f161(v0: Adt14) -> Option<Bytes> {
    let m859 = v0;
    match m859 {
        Adt14::C0(v1) => {
            f176(v1)
        }
        Adt14::C1(_) => {
            None::<Bytes>
        }
        Adt14::C2(_) => {
            None::<Bytes>
        }
        Adt14::C3(v4, _) => {
            Some(f177(v4))
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

pub fn f162(v0: Adt30, v1: Bytes) -> Adt30 {
    Adt30::C0(match v0.clone() {
        Adt30::C0(h860, _, _) => {
            h860
        }
    }, f178(match v0.clone() {
        Adt30::C0(_, h861, _) => {
            h861
        }
    }, v1), match v0 {
        Adt30::C0(_, _, h862) => {
            h862
        }
    })
}

pub fn f163(v0: Adt3) -> Bytes {
    let m863 = v0;
    match m863 {
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

pub fn f164(v0: Adt5) -> Bytes {
    let m864 = v0;
    match m864 {
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

pub fn f165(v0: Adt6) -> Bytes {
    let m865 = v0;
    match m865 {
        Adt6::C0 => {
            f1(Str::lit("result"))
        }
        Adt6::C1 => {
            f1(Str::lit("pair"))
        }
    }
}

pub fn f166(v0: Adt7) -> Bytes {
    let m866 = v0;
    match m866 {
        Adt7::C0 => {
            f1(Str::lit("adt"))
        }
        Adt7::C1 => {
            f1(Str::lit("fn"))
        }
    }
}

pub fn f167(v0: Fn15, p870: List<u64>, p871: List<u64>) -> List<u64> {
    let m869 = (p870, p871);
    {
        let mut m869 = m869;
        loop {
            let step = {
                let (v1, v2) = m869;
                let m867 = v2;
                match m867.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>(({
                            let c868: Fn15 = v0.clone();
                            c868.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m869 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f168(v0: List<u64>, v1: u64) -> List<u64> {
    let v2: bool = f121(v0.clone(), v1);
    if v2 {
        v0
    } else {
        List::cons(v1, v0)
    }
}

pub fn f169(v0: Adt17) -> bool {
    let m872 = v0;
    match m872 {
        Adt17::C0(_, _) => {
            false
        }
        Adt17::C1(_, _, _) => {
            false
        }
        Adt17::C2(v6, _, _) => {
            let m873 = v6;
            match m873 {
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

pub fn f170(v0: Adt12, v1: Adt12) -> bool {
    let a874 = f150(v0.clone());
    let a875 = f150(v1.clone());
    if nat_eq(a874, a875) {
        let a876 = f150(v0.clone());
        let a877 = 9u64;
        if nat_eq(a876, a877) {
            f156(f152(v0), f152(v1))
        } else {
            let a878 = f151(v0);
            let a879 = f151(v1);
            nat_lt(a878, a879)
        }
    } else {
        let a880 = f150(v0);
        let a881 = f150(v1);
        nat_lt(a880, a881)
    }
}

pub fn f171(v0: List<u64>, v1: List<u64>) -> bool {
    f153(Fn14::F179(v1), true, v0)
}

pub fn f172(v0: Fn16, p885: bool, p886: List<Adt29>) -> bool {
    let m884 = (p885, p886);
    {
        let mut m884 = m884;
        loop {
            let step = {
                let (v1, v2) = m884;
                let m882 = v2;
                match m882.uncons() {
                    None => {
                        Err::<(bool, List<Adt29>), bool>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<Adt29>), bool>(({
                            let c883: Fn16 = v0.clone();
                            c883.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m884 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f173(v0: Adt29, v1: bool, v2: Adt29) -> bool {
    let v3: bool = f180(v2, v0);
    if v1 {
        true
    } else {
        v3
    }
}

pub fn f174(v0: Bytes, v1: u64, v2: Bytes, v3: u64, v4: u64) -> R<Option<u64>> {
    if f175(v2, v0, v4) {
        Ok(None::<u64>)
    } else {
        let a889 = {
            let a887 = v4;
            let a888 = v1;
            nat_add(a887, a888)?
        };
        let a890 = v3;
        if nat_lt(a889, a890) {
            Ok(Some({
                let a891 = v4;
                let a892 = 1u64;
                nat_add(a891, a892)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f175(v0: Bytes, v1: Bytes, v2: u64) -> bool {
    let m897 = {
        let a894 = v0;
        let a895 = v2;
        let a896 = {
            let a893 = v1.clone();
            length_bytes(a893)
        };
        slice_bytes(a894, a895, a896)
    };
    match m897 {
        None => {
            false
        }
        Some(v3) => {
            let a898 = v3;
            let a899 = v1;
            equal(a898, a899)
        }
    }
}

pub fn f176(v0: Adt15) -> Option<Bytes> {
    let m900 = v0;
    match m900 {
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

pub fn f177(v0: Adt16) -> Bytes {
    let m901 = v0;
    match m901 {
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

pub fn f178(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    let m902 = f181(v0.clone(), v1.clone());
    if m902 {
        v0
    } else {
        let v2: (bool, List<Bytes>) = f182(Fn17::F183(v1.clone()), (false, List::<Bytes>::nil()), v0);
        let m904 = {
            let (h903, _) = v2;
            h903
        };
        if m904 {
            f184({
                let (_, h905) = v2;
                h905
            })
        } else {
            f184(List::cons(v1, {
                let (_, h906) = v2;
                h906
            }))
        }
    }
}

pub fn f179(v0: List<u64>, v1: bool, v2: u64) -> bool {
    let v3: bool = f121(v0, v2);
    if v1 {
        v3
    } else {
        false
    }
}

pub fn f180(v0: Adt29, v1: Adt29) -> bool {
    let a909 = match v0.clone() {
        Adt29::C0(h907, _, _, _) => {
            h907
        }
    };
    let a910 = match v1.clone() {
        Adt29::C0(h908, _, _, _) => {
            h908
        }
    };
    if equal(a909, a910) {
        let a913 = match v0.clone() {
            Adt29::C0(_, h911, _, _) => {
                h911
            }
        };
        let a914 = match v1.clone() {
            Adt29::C0(_, h912, _, _) => {
                h912
            }
        };
        if equal(a913, a914) {
            if f185(match v0.clone() {
                Adt29::C0(_, _, h915, _) => {
                    h915
                }
            }, match v1.clone() {
                Adt29::C0(_, _, h916, _) => {
                    h916
                }
            }) {
                f185(match v0 {
                    Adt29::C0(_, _, _, h917) => {
                        h917
                    }
                }, match v1 {
                    Adt29::C0(_, _, _, h918) => {
                        h918
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

pub fn f181(v0: List<Bytes>, v1: Bytes) -> bool {
    f186(Fn18::F187(v1), false, v0)
}

pub fn f182(v0: Fn17, p922: (bool, List<Bytes>), p923: List<Bytes>) -> (bool, List<Bytes>) {
    let m921 = (p922, p923);
    {
        let mut m921 = m921;
        loop {
            let step = {
                let (v1, v2) = m921;
                let m919 = v2;
                match m919.uncons() {
                    None => {
                        Err::<((bool, List<Bytes>), List<Bytes>), (bool, List<Bytes>)>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<((bool, List<Bytes>), List<Bytes>), (bool, List<Bytes>)>(({
                            let c920: Fn17 = v0.clone();
                            c920.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m921 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f183(v0: Bytes, v1: (bool, List<Bytes>), v2: Bytes) -> (bool, List<Bytes>) {
    f188(v0, v1, v2)
}

pub fn f184(v0: List<Bytes>) -> List<Bytes> {
    f189(Fn19::F190, List::<Bytes>::nil(), v0)
}

pub fn f185(v0: Option<Adt4>, v1: Option<Adt4>) -> bool {
    let a924 = f191(v0);
    let a925 = f191(v1);
    equal(a924, a925)
}

pub fn f186(v0: Fn18, p929: bool, p930: List<Bytes>) -> bool {
    let m928 = (p929, p930);
    {
        let mut m928 = m928;
        loop {
            let step = {
                let (v1, v2) = m928;
                let m926 = v2;
                match m926.uncons() {
                    None => {
                        Err::<(bool, List<Bytes>), bool>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<Bytes>), bool>(({
                            let c927: Fn18 = v0.clone();
                            c927.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m928 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f187(v0: Bytes, v1: bool, v2: Bytes) -> bool {
    if v1 {
        true
    } else {
        let a931 = v2;
        let a932 = v0;
        equal(a931, a932)
    }
}

pub fn f188(v0: Bytes, v1: (bool, List<Bytes>), v2: Bytes) -> (bool, List<Bytes>) {
    let v3: bool = {
        let (h933, _) = v1;
        h933
    };
    let v4: List<Bytes> = {
        let (_, h934) = v1;
        h934
    };
    let m935 = v3;
    if m935 {
        (true, List::cons(v2, v4))
    } else {
        let m936 = f156(v0.clone(), v2.clone());
        if m936 {
            (true, List::cons(v2, List::cons(v0, v4)))
        } else {
            (false, List::cons(v2, v4))
        }
    }
}

pub fn f189(v0: Fn19, p940: List<Bytes>, p941: List<Bytes>) -> List<Bytes> {
    let m939 = (p940, p941);
    {
        let mut m939 = m939;
        loop {
            let step = {
                let (v1, v2) = m939;
                let m937 = v2;
                match m937.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c938: Fn19 = v0.clone();
                            c938.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m939 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f190(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f191(v0: Option<Adt4>) -> Bytes {
    let m942 = v0;
    match m942 {
        None => {
            f1(Str::lit("None"))
        }
        Some(v1) => {
            f4(f1(Str::lit("Some(")), f4(f14(v1), f1(Str::lit(")"))))
        }
    }
}
