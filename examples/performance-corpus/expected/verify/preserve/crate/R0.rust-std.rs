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

pub fn f0(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m3 = {
        let a1 = v0.clone();
        let a2 = v1;
        index_bytes(a1, a2)
    };
    match m3 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            let m8 = {
                let a6 = v0.clone();
                let a7 = {
                    let a4 = v1;
                    let a5 = 1u64;
                    nat_add(a4, a5)?
                };
                index_bytes(a6, a7)
            };
            match m8 {
                None => {
                    Ok(None::<u64>)
                }
                Some(v3) => {
                    let m13 = {
                        let a11 = v0.clone();
                        let a12 = {
                            let a9 = v1;
                            let a10 = 2u64;
                            nat_add(a9, a10)?
                        };
                        index_bytes(a11, a12)
                    };
                    match m13 {
                        None => {
                            Ok(None::<u64>)
                        }
                        Some(v4) => {
                            let m18 = {
                                let a16 = v0;
                                let a17 = {
                                    let a14 = v1;
                                    let a15 = 3u64;
                                    nat_add(a14, a15)?
                                };
                                index_bytes(a16, a17)
                            };
                            match m18 {
                                None => {
                                    Ok(None::<u64>)
                                }
                                Some(v5) => {
                                    Ok(Some({
                                        let a213 = {
                                            let a165 = {
                                                let a115 = {
                                                    let a65 = {
                                                        let v6: u8 = v2;
                                                        let a63 = {
                                                            let a39 = {
                                                                let a27 = {
                                                                    let a21 = {
                                                                        let a19 = v6;
                                                                        let a20 = 128u8;
                                                                        fixed_u8::bit_and(a19, a20)
                                                                    };
                                                                    let a22 = 128u8;
                                                                    if equal(a21, a22) {
                                                                        128u64
                                                                    } else {
                                                                        0u64
                                                                    }
                                                                };
                                                                let a28 = {
                                                                    let a25 = {
                                                                        let a23 = v6;
                                                                        let a24 = 64u8;
                                                                        fixed_u8::bit_and(a23, a24)
                                                                    };
                                                                    let a26 = 64u8;
                                                                    if equal(a25, a26) {
                                                                        64u64
                                                                    } else {
                                                                        0u64
                                                                    }
                                                                };
                                                                nat_add(a27, a28)?
                                                            };
                                                            let a40 = {
                                                                let a37 = {
                                                                    let a31 = {
                                                                        let a29 = v6;
                                                                        let a30 = 32u8;
                                                                        fixed_u8::bit_and(a29, a30)
                                                                    };
                                                                    let a32 = 32u8;
                                                                    if equal(a31, a32) {
                                                                        32u64
                                                                    } else {
                                                                        0u64
                                                                    }
                                                                };
                                                                let a38 = {
                                                                    let a35 = {
                                                                        let a33 = v6;
                                                                        let a34 = 16u8;
                                                                        fixed_u8::bit_and(a33, a34)
                                                                    };
                                                                    let a36 = 16u8;
                                                                    if equal(a35, a36) {
                                                                        16u64
                                                                    } else {
                                                                        0u64
                                                                    }
                                                                };
                                                                nat_add(a37, a38)?
                                                            };
                                                            nat_add(a39, a40)?
                                                        };
                                                        let a64 = {
                                                            let a61 = {
                                                                let a49 = {
                                                                    let a43 = {
                                                                        let a41 = v6;
                                                                        let a42 = 8u8;
                                                                        fixed_u8::bit_and(a41, a42)
                                                                    };
                                                                    let a44 = 8u8;
                                                                    if equal(a43, a44) {
                                                                        8u64
                                                                    } else {
                                                                        0u64
                                                                    }
                                                                };
                                                                let a50 = {
                                                                    let a47 = {
                                                                        let a45 = v6;
                                                                        let a46 = 4u8;
                                                                        fixed_u8::bit_and(a45, a46)
                                                                    };
                                                                    let a48 = 4u8;
                                                                    if equal(a47, a48) {
                                                                        4u64
                                                                    } else {
                                                                        0u64
                                                                    }
                                                                };
                                                                nat_add(a49, a50)?
                                                            };
                                                            let a62 = {
                                                                let a59 = {
                                                                    let a53 = {
                                                                        let a51 = v6;
                                                                        let a52 = 2u8;
                                                                        fixed_u8::bit_and(a51, a52)
                                                                    };
                                                                    let a54 = 2u8;
                                                                    if equal(a53, a54) {
                                                                        2u64
                                                                    } else {
                                                                        0u64
                                                                    }
                                                                };
                                                                let a60 = {
                                                                    let a57 = {
                                                                        let a55 = v6;
                                                                        let a56 = 1u8;
                                                                        fixed_u8::bit_and(a55, a56)
                                                                    };
                                                                    let a58 = 1u8;
                                                                    if equal(a57, a58) {
                                                                        1u64
                                                                    } else {
                                                                        0u64
                                                                    }
                                                                };
                                                                nat_add(a59, a60)?
                                                            };
                                                            nat_add(a61, a62)?
                                                        };
                                                        nat_add(a63, a64)?
                                                    };
                                                    let a66 = 16777216u64;
                                                    nat_mul(a65, a66)?
                                                };
                                                let a116 = {
                                                    let a113 = {
                                                        let v7: u8 = v3;
                                                        let a111 = {
                                                            let a87 = {
                                                                let a75 = {
                                                                    let a69 = {
                                                                        let a67 = v7;
                                                                        let a68 = 128u8;
                                                                        fixed_u8::bit_and(a67, a68)
                                                                    };
                                                                    let a70 = 128u8;
                                                                    if equal(a69, a70) {
                                                                        128u64
                                                                    } else {
                                                                        0u64
                                                                    }
                                                                };
                                                                let a76 = {
                                                                    let a73 = {
                                                                        let a71 = v7;
                                                                        let a72 = 64u8;
                                                                        fixed_u8::bit_and(a71, a72)
                                                                    };
                                                                    let a74 = 64u8;
                                                                    if equal(a73, a74) {
                                                                        64u64
                                                                    } else {
                                                                        0u64
                                                                    }
                                                                };
                                                                nat_add(a75, a76)?
                                                            };
                                                            let a88 = {
                                                                let a85 = {
                                                                    let a79 = {
                                                                        let a77 = v7;
                                                                        let a78 = 32u8;
                                                                        fixed_u8::bit_and(a77, a78)
                                                                    };
                                                                    let a80 = 32u8;
                                                                    if equal(a79, a80) {
                                                                        32u64
                                                                    } else {
                                                                        0u64
                                                                    }
                                                                };
                                                                let a86 = {
                                                                    let a83 = {
                                                                        let a81 = v7;
                                                                        let a82 = 16u8;
                                                                        fixed_u8::bit_and(a81, a82)
                                                                    };
                                                                    let a84 = 16u8;
                                                                    if equal(a83, a84) {
                                                                        16u64
                                                                    } else {
                                                                        0u64
                                                                    }
                                                                };
                                                                nat_add(a85, a86)?
                                                            };
                                                            nat_add(a87, a88)?
                                                        };
                                                        let a112 = {
                                                            let a109 = {
                                                                let a97 = {
                                                                    let a91 = {
                                                                        let a89 = v7;
                                                                        let a90 = 8u8;
                                                                        fixed_u8::bit_and(a89, a90)
                                                                    };
                                                                    let a92 = 8u8;
                                                                    if equal(a91, a92) {
                                                                        8u64
                                                                    } else {
                                                                        0u64
                                                                    }
                                                                };
                                                                let a98 = {
                                                                    let a95 = {
                                                                        let a93 = v7;
                                                                        let a94 = 4u8;
                                                                        fixed_u8::bit_and(a93, a94)
                                                                    };
                                                                    let a96 = 4u8;
                                                                    if equal(a95, a96) {
                                                                        4u64
                                                                    } else {
                                                                        0u64
                                                                    }
                                                                };
                                                                nat_add(a97, a98)?
                                                            };
                                                            let a110 = {
                                                                let a107 = {
                                                                    let a101 = {
                                                                        let a99 = v7;
                                                                        let a100 = 2u8;
                                                                        fixed_u8::bit_and(a99, a100)
                                                                    };
                                                                    let a102 = 2u8;
                                                                    if equal(a101, a102) {
                                                                        2u64
                                                                    } else {
                                                                        0u64
                                                                    }
                                                                };
                                                                let a108 = {
                                                                    let a105 = {
                                                                        let a103 = v7;
                                                                        let a104 = 1u8;
                                                                        fixed_u8::bit_and(a103, a104)
                                                                    };
                                                                    let a106 = 1u8;
                                                                    if equal(a105, a106) {
                                                                        1u64
                                                                    } else {
                                                                        0u64
                                                                    }
                                                                };
                                                                nat_add(a107, a108)?
                                                            };
                                                            nat_add(a109, a110)?
                                                        };
                                                        nat_add(a111, a112)?
                                                    };
                                                    let a114 = 65536u64;
                                                    nat_mul(a113, a114)?
                                                };
                                                nat_add(a115, a116)?
                                            };
                                            let a166 = {
                                                let a163 = {
                                                    let v8: u8 = v4;
                                                    let a161 = {
                                                        let a137 = {
                                                            let a125 = {
                                                                let a119 = {
                                                                    let a117 = v8;
                                                                    let a118 = 128u8;
                                                                    fixed_u8::bit_and(a117, a118)
                                                                };
                                                                let a120 = 128u8;
                                                                if equal(a119, a120) {
                                                                    128u64
                                                                } else {
                                                                    0u64
                                                                }
                                                            };
                                                            let a126 = {
                                                                let a123 = {
                                                                    let a121 = v8;
                                                                    let a122 = 64u8;
                                                                    fixed_u8::bit_and(a121, a122)
                                                                };
                                                                let a124 = 64u8;
                                                                if equal(a123, a124) {
                                                                    64u64
                                                                } else {
                                                                    0u64
                                                                }
                                                            };
                                                            nat_add(a125, a126)?
                                                        };
                                                        let a138 = {
                                                            let a135 = {
                                                                let a129 = {
                                                                    let a127 = v8;
                                                                    let a128 = 32u8;
                                                                    fixed_u8::bit_and(a127, a128)
                                                                };
                                                                let a130 = 32u8;
                                                                if equal(a129, a130) {
                                                                    32u64
                                                                } else {
                                                                    0u64
                                                                }
                                                            };
                                                            let a136 = {
                                                                let a133 = {
                                                                    let a131 = v8;
                                                                    let a132 = 16u8;
                                                                    fixed_u8::bit_and(a131, a132)
                                                                };
                                                                let a134 = 16u8;
                                                                if equal(a133, a134) {
                                                                    16u64
                                                                } else {
                                                                    0u64
                                                                }
                                                            };
                                                            nat_add(a135, a136)?
                                                        };
                                                        nat_add(a137, a138)?
                                                    };
                                                    let a162 = {
                                                        let a159 = {
                                                            let a147 = {
                                                                let a141 = {
                                                                    let a139 = v8;
                                                                    let a140 = 8u8;
                                                                    fixed_u8::bit_and(a139, a140)
                                                                };
                                                                let a142 = 8u8;
                                                                if equal(a141, a142) {
                                                                    8u64
                                                                } else {
                                                                    0u64
                                                                }
                                                            };
                                                            let a148 = {
                                                                let a145 = {
                                                                    let a143 = v8;
                                                                    let a144 = 4u8;
                                                                    fixed_u8::bit_and(a143, a144)
                                                                };
                                                                let a146 = 4u8;
                                                                if equal(a145, a146) {
                                                                    4u64
                                                                } else {
                                                                    0u64
                                                                }
                                                            };
                                                            nat_add(a147, a148)?
                                                        };
                                                        let a160 = {
                                                            let a157 = {
                                                                let a151 = {
                                                                    let a149 = v8;
                                                                    let a150 = 2u8;
                                                                    fixed_u8::bit_and(a149, a150)
                                                                };
                                                                let a152 = 2u8;
                                                                if equal(a151, a152) {
                                                                    2u64
                                                                } else {
                                                                    0u64
                                                                }
                                                            };
                                                            let a158 = {
                                                                let a155 = {
                                                                    let a153 = v8;
                                                                    let a154 = 1u8;
                                                                    fixed_u8::bit_and(a153, a154)
                                                                };
                                                                let a156 = 1u8;
                                                                if equal(a155, a156) {
                                                                    1u64
                                                                } else {
                                                                    0u64
                                                                }
                                                            };
                                                            nat_add(a157, a158)?
                                                        };
                                                        nat_add(a159, a160)?
                                                    };
                                                    nat_add(a161, a162)?
                                                };
                                                let a164 = 256u64;
                                                nat_mul(a163, a164)?
                                            };
                                            nat_add(a165, a166)?
                                        };
                                        let a214 = {
                                            let v9: u8 = v5;
                                            let a211 = {
                                                let a187 = {
                                                    let a175 = {
                                                        let a169 = {
                                                            let a167 = v9;
                                                            let a168 = 128u8;
                                                            fixed_u8::bit_and(a167, a168)
                                                        };
                                                        let a170 = 128u8;
                                                        if equal(a169, a170) {
                                                            128u64
                                                        } else {
                                                            0u64
                                                        }
                                                    };
                                                    let a176 = {
                                                        let a173 = {
                                                            let a171 = v9;
                                                            let a172 = 64u8;
                                                            fixed_u8::bit_and(a171, a172)
                                                        };
                                                        let a174 = 64u8;
                                                        if equal(a173, a174) {
                                                            64u64
                                                        } else {
                                                            0u64
                                                        }
                                                    };
                                                    nat_add(a175, a176)?
                                                };
                                                let a188 = {
                                                    let a185 = {
                                                        let a179 = {
                                                            let a177 = v9;
                                                            let a178 = 32u8;
                                                            fixed_u8::bit_and(a177, a178)
                                                        };
                                                        let a180 = 32u8;
                                                        if equal(a179, a180) {
                                                            32u64
                                                        } else {
                                                            0u64
                                                        }
                                                    };
                                                    let a186 = {
                                                        let a183 = {
                                                            let a181 = v9;
                                                            let a182 = 16u8;
                                                            fixed_u8::bit_and(a181, a182)
                                                        };
                                                        let a184 = 16u8;
                                                        if equal(a183, a184) {
                                                            16u64
                                                        } else {
                                                            0u64
                                                        }
                                                    };
                                                    nat_add(a185, a186)?
                                                };
                                                nat_add(a187, a188)?
                                            };
                                            let a212 = {
                                                let a209 = {
                                                    let a197 = {
                                                        let a191 = {
                                                            let a189 = v9;
                                                            let a190 = 8u8;
                                                            fixed_u8::bit_and(a189, a190)
                                                        };
                                                        let a192 = 8u8;
                                                        if equal(a191, a192) {
                                                            8u64
                                                        } else {
                                                            0u64
                                                        }
                                                    };
                                                    let a198 = {
                                                        let a195 = {
                                                            let a193 = v9;
                                                            let a194 = 4u8;
                                                            fixed_u8::bit_and(a193, a194)
                                                        };
                                                        let a196 = 4u8;
                                                        if equal(a195, a196) {
                                                            4u64
                                                        } else {
                                                            0u64
                                                        }
                                                    };
                                                    nat_add(a197, a198)?
                                                };
                                                let a210 = {
                                                    let a207 = {
                                                        let a201 = {
                                                            let a199 = v9;
                                                            let a200 = 2u8;
                                                            fixed_u8::bit_and(a199, a200)
                                                        };
                                                        let a202 = 2u8;
                                                        if equal(a201, a202) {
                                                            2u64
                                                        } else {
                                                            0u64
                                                        }
                                                    };
                                                    let a208 = {
                                                        let a205 = {
                                                            let a203 = v9;
                                                            let a204 = 1u8;
                                                            fixed_u8::bit_and(a203, a204)
                                                        };
                                                        let a206 = 1u8;
                                                        if equal(a205, a206) {
                                                            1u64
                                                        } else {
                                                            0u64
                                                        }
                                                    };
                                                    nat_add(a207, a208)?
                                                };
                                                nat_add(a209, a210)?
                                            };
                                            nat_add(a211, a212)?
                                        };
                                        nat_add(a213, a214)?
                                    }))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
