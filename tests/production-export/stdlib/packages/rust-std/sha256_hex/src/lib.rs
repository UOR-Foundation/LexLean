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

pub fn f4(v0: Fn0, v1: u64, v2: (u64, Adt0)) -> R<(u64, Adt0)> {
    let m65 = v1;
    if m65 == 0 {
        Ok(v2)
    } else {
        let v3 = m65 - 1;
        f4(v0.clone(), v3, {
            let c66: Fn0 = v0;
            c66.apply(v2)?
        })
    }
}

pub fn f5(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt0)) -> R<(u64, Adt0)> {
    Ok(({
        let a68 = {
            let (h67, _) = v3.clone();
            h67
        };
        let a69 = 64u64;
        nat_add(a68, a69)?
    }, f9({
        let (_, h70) = v3;
        h70
    }, f10(v0, v1, v2, {
        let (h71, _) = v3;
        h71
    })?)?))
}

pub fn f6() -> Adt0 {
    Adt0::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f7(v0: u32) -> Bytes {
    let a74 = {
        let a72 = v0;
        let a73 = 8u32;
        fixed_u32::bit_and(a72, a73)
    };
    let a75 = 0u32;
    if equal(a74, a75) {
        let a78 = {
            let a76 = v0;
            let a77 = 4u32;
            fixed_u32::bit_and(a76, a77)
        };
        let a79 = 0u32;
        if equal(a78, a79) {
            let a82 = {
                let a80 = v0;
                let a81 = 2u32;
                fixed_u32::bit_and(a80, a81)
            };
            let a83 = 0u32;
            if equal(a82, a83) {
                let a86 = {
                    let a84 = v0;
                    let a85 = 1u32;
                    fixed_u32::bit_and(a84, a85)
                };
                let a87 = 0u32;
                if equal(a86, a87) {
                    Bytes::lit(&[0x30])
                } else {
                    Bytes::lit(&[0x31])
                }
            } else {
                let a90 = {
                    let a88 = v0;
                    let a89 = 1u32;
                    fixed_u32::bit_and(a88, a89)
                };
                let a91 = 0u32;
                if equal(a90, a91) {
                    Bytes::lit(&[0x32])
                } else {
                    Bytes::lit(&[0x33])
                }
            }
        } else {
            let a94 = {
                let a92 = v0;
                let a93 = 2u32;
                fixed_u32::bit_and(a92, a93)
            };
            let a95 = 0u32;
            if equal(a94, a95) {
                let a98 = {
                    let a96 = v0;
                    let a97 = 1u32;
                    fixed_u32::bit_and(a96, a97)
                };
                let a99 = 0u32;
                if equal(a98, a99) {
                    Bytes::lit(&[0x34])
                } else {
                    Bytes::lit(&[0x35])
                }
            } else {
                let a102 = {
                    let a100 = v0;
                    let a101 = 1u32;
                    fixed_u32::bit_and(a100, a101)
                };
                let a103 = 0u32;
                if equal(a102, a103) {
                    Bytes::lit(&[0x36])
                } else {
                    Bytes::lit(&[0x37])
                }
            }
        }
    } else {
        let a106 = {
            let a104 = v0;
            let a105 = 4u32;
            fixed_u32::bit_and(a104, a105)
        };
        let a107 = 0u32;
        if equal(a106, a107) {
            let a110 = {
                let a108 = v0;
                let a109 = 2u32;
                fixed_u32::bit_and(a108, a109)
            };
            let a111 = 0u32;
            if equal(a110, a111) {
                let a114 = {
                    let a112 = v0;
                    let a113 = 1u32;
                    fixed_u32::bit_and(a112, a113)
                };
                let a115 = 0u32;
                if equal(a114, a115) {
                    Bytes::lit(&[0x38])
                } else {
                    Bytes::lit(&[0x39])
                }
            } else {
                let a118 = {
                    let a116 = v0;
                    let a117 = 1u32;
                    fixed_u32::bit_and(a116, a117)
                };
                let a119 = 0u32;
                if equal(a118, a119) {
                    Bytes::lit(&[0x61])
                } else {
                    Bytes::lit(&[0x62])
                }
            }
        } else {
            let a122 = {
                let a120 = v0;
                let a121 = 2u32;
                fixed_u32::bit_and(a120, a121)
            };
            let a123 = 0u32;
            if equal(a122, a123) {
                let a126 = {
                    let a124 = v0;
                    let a125 = 1u32;
                    fixed_u32::bit_and(a124, a125)
                };
                let a127 = 0u32;
                if equal(a126, a127) {
                    Bytes::lit(&[0x63])
                } else {
                    Bytes::lit(&[0x64])
                }
            } else {
                let a130 = {
                    let a128 = v0;
                    let a129 = 1u32;
                    fixed_u32::bit_and(a128, a129)
                };
                let a131 = 0u32;
                if equal(a130, a131) {
                    Bytes::lit(&[0x65])
                } else {
                    Bytes::lit(&[0x66])
                }
            }
        }
    }
}

pub fn f8(v0: u32, v1: u32) -> u32 {
    let m134 = {
        let a132 = v0;
        let a133 = v1;
        fixed_u32::shift_right(a132, a133)
    };
    match m134 {
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
        Adt0::C0(h135, _, _, _, _, _, _, _) => {
            h135
        }
    }, match v0.clone() {
        Adt0::C0(_, h136, _, _, _, _, _, _) => {
            h136
        }
    }, match v0.clone() {
        Adt0::C0(_, _, h137, _, _, _, _, _) => {
            h137
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, h138, _, _, _, _) => {
            h138
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, h139, _, _, _) => {
            h139
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, h140, _, _) => {
            h140
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, h141, _) => {
            h141
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, h142) => {
            h142
        }
    }, v1, 0u64))?;
    Ok(Adt0::C0(f13(match v0.clone() {
        Adt0::C0(h143, _, _, _, _, _, _, _) => {
            h143
        }
    }, match v2.clone() {
        Adt2::C0(h144, _, _, _, _, _, _, _, _, _) => {
            h144
        }
    }), f13(match v0.clone() {
        Adt0::C0(_, h145, _, _, _, _, _, _) => {
            h145
        }
    }, match v2.clone() {
        Adt2::C0(_, h146, _, _, _, _, _, _, _, _) => {
            h146
        }
    }), f13(match v0.clone() {
        Adt0::C0(_, _, h147, _, _, _, _, _) => {
            h147
        }
    }, match v2.clone() {
        Adt2::C0(_, _, h148, _, _, _, _, _, _, _) => {
            h148
        }
    }), f13(match v0.clone() {
        Adt0::C0(_, _, _, h149, _, _, _, _) => {
            h149
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, h150, _, _, _, _, _, _) => {
            h150
        }
    }), f13(match v0.clone() {
        Adt0::C0(_, _, _, _, h151, _, _, _) => {
            h151
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, _, h152, _, _, _, _, _) => {
            h152
        }
    }), f13(match v0.clone() {
        Adt0::C0(_, _, _, _, _, h153, _, _) => {
            h153
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, _, _, h154, _, _, _, _) => {
            h154
        }
    }), f13(match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, h155, _) => {
            h155
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, _, _, _, h156, _, _, _) => {
            h156
        }
    }), f13(match v0 {
        Adt0::C0(_, _, _, _, _, _, _, h157) => {
            h157
        }
    }, match v2 {
        Adt2::C0(_, _, _, _, _, _, _, h158, _, _) => {
            h158
        }
    })))
}

pub fn f10(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt1> {
    Ok(Adt1::C0(f14(v0.clone(), v1, v2, v3)?, f14(v0.clone(), v1, v2, {
        let a159 = v3;
        let a160 = 4u64;
        nat_add(a159, a160)?
    })?, f14(v0.clone(), v1, v2, {
        let a161 = v3;
        let a162 = 8u64;
        nat_add(a161, a162)?
    })?, f14(v0.clone(), v1, v2, {
        let a163 = v3;
        let a164 = 12u64;
        nat_add(a163, a164)?
    })?, f14(v0.clone(), v1, v2, {
        let a165 = v3;
        let a166 = 16u64;
        nat_add(a165, a166)?
    })?, f14(v0.clone(), v1, v2, {
        let a167 = v3;
        let a168 = 20u64;
        nat_add(a167, a168)?
    })?, f14(v0.clone(), v1, v2, {
        let a169 = v3;
        let a170 = 24u64;
        nat_add(a169, a170)?
    })?, f14(v0.clone(), v1, v2, {
        let a171 = v3;
        let a172 = 28u64;
        nat_add(a171, a172)?
    })?, f14(v0.clone(), v1, v2, {
        let a173 = v3;
        let a174 = 32u64;
        nat_add(a173, a174)?
    })?, f14(v0.clone(), v1, v2, {
        let a175 = v3;
        let a176 = 36u64;
        nat_add(a175, a176)?
    })?, f14(v0.clone(), v1, v2, {
        let a177 = v3;
        let a178 = 40u64;
        nat_add(a177, a178)?
    })?, f14(v0.clone(), v1, v2, {
        let a179 = v3;
        let a180 = 44u64;
        nat_add(a179, a180)?
    })?, f14(v0.clone(), v1, v2, {
        let a181 = v3;
        let a182 = 48u64;
        nat_add(a181, a182)?
    })?, f14(v0.clone(), v1, v2, {
        let a183 = v3;
        let a184 = 52u64;
        nat_add(a183, a184)?
    })?, f14(v0.clone(), v1, v2, {
        let a185 = v3;
        let a186 = 56u64;
        nat_add(a185, a186)?
    })?, f14(v0, v1, v2, {
        let a187 = v3;
        let a188 = 60u64;
        nat_add(a187, a188)?
    })?))
}

pub fn f11(v0: Fn1, v1: u64, v2: Adt2) -> R<Adt2> {
    let m189 = v1;
    if m189 == 0 {
        Ok(v2)
    } else {
        let v3 = m189 - 1;
        f11(v0.clone(), v3, {
            let c190: Fn1 = v0;
            c190.apply(v2)?
        })
    }
}

pub fn f12(v0: Adt2) -> R<Adt2> {
    f15(v0)
}

pub fn f13(v0: u32, v1: u32) -> u32 {
    let m193 = {
        let a191 = f16(v0);
        let a192 = f16(v1);
        fixed_u64::checked_add(a191, a192)
    };
    match m193 {
        None => {
            0u32
        }
        Some(v2) => {
            f17(v2)
        }
    }
}

pub fn f14(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u32> {
    let a196 = {
        let a194 = v3;
        let a195 = 8u64;
        nat_add(a194, a195)?
    };
    let a197 = v2;
    if nat_eq(a196, a197) {
        Ok(f18({
            let a200 = {
                let a198 = v1;
                let a199 = 8u64;
                nat_mul(a198, a199)?
            };
            let a201 = 4294967296u64;
            let a202 = 0u64;
            nat_quot(a200, a201, a202)
        }))
    } else {
        let a205 = {
            let a203 = v3;
            let a204 = 4u64;
            nat_add(a203, a204)?
        };
        let a206 = v2;
        if nat_eq(a205, a206) {
            Ok(f18({
                let a209 = {
                    let a207 = v1;
                    let a208 = 8u64;
                    nat_mul(a207, a208)?
                };
                let a210 = 4294967296u64;
                let a211 = 0u64;
                nat_rem(a209, a210, a211)
            }))
        } else {
            Ok(f19(f20(v0.clone(), v1, v3), f20(v0.clone(), v1, {
                let a212 = v3;
                let a213 = 1u64;
                nat_add(a212, a213)?
            }), f20(v0.clone(), v1, {
                let a214 = v3;
                let a215 = 2u64;
                nat_add(a214, a215)?
            }), f20(v0, v1, {
                let a216 = v3;
                let a217 = 3u64;
                nat_add(a216, a217)?
            })))
        }
    }
}

pub fn f15(v0: Adt2) -> R<Adt2> {
    let v1: u32 = f21(match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h218) => {
            h218
        }
    });
    let v2: u32 = f13(match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h219, _, _) => {
            h219
        }
    }, f13(f22(match v0.clone() {
        Adt2::C0(_, _, _, _, h220, _, _, _, _, _) => {
            h220
        }
    }), f13(f23(match v0.clone() {
        Adt2::C0(_, _, _, _, h221, _, _, _, _, _) => {
            h221
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h222, _, _, _, _) => {
            h222
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h223, _, _, _) => {
            h223
        }
    }), f13(v1, {
        let m226 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, h225, _) => {
                h225
            }
        };
        match m226 {
            Adt1::C0(h224, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h224
            }
        }
    }))));
    let v3: u32 = f13(f24(match v0.clone() {
        Adt2::C0(h227, _, _, _, _, _, _, _, _, _) => {
            h227
        }
    }), f25(match v0.clone() {
        Adt2::C0(h228, _, _, _, _, _, _, _, _, _) => {
            h228
        }
    }, match v0.clone() {
        Adt2::C0(_, h229, _, _, _, _, _, _, _, _) => {
            h229
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h230, _, _, _, _, _, _, _) => {
            h230
        }
    }));
    Ok(Adt2::C0(f13(v2, v3), match v0.clone() {
        Adt2::C0(h231, _, _, _, _, _, _, _, _, _) => {
            h231
        }
    }, match v0.clone() {
        Adt2::C0(_, h232, _, _, _, _, _, _, _, _) => {
            h232
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h233, _, _, _, _, _, _, _) => {
            h233
        }
    }, f13(match v0.clone() {
        Adt2::C0(_, _, _, h234, _, _, _, _, _, _) => {
            h234
        }
    }, v2), match v0.clone() {
        Adt2::C0(_, _, _, _, h235, _, _, _, _, _) => {
            h235
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h236, _, _, _, _) => {
            h236
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h237, _, _, _) => {
            h237
        }
    }, f26(match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, h238, _) => {
            h238
        }
    }), {
        let a240 = match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h239) => {
                h239
            }
        };
        let a241 = 1u64;
        nat_add(a240, a241)?
    }))
}

pub fn f16(v0: u32) -> u64 {
    let m243 = {
        let a242 = v0;
        fixed_u64::convert(i128::from(a242))
    };
    match m243 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f17(v0: u64) -> u32 {
    let m247 = {
        let a246 = {
            let a244 = v0;
            let a245 = 4294967295u64;
            fixed_u64::bit_and(a244, a245)
        };
        fixed_u32::convert(i128::from(a246))
    };
    match m247 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f18(v0: u64) -> u32 {
    let (_, h248) = {
        let (_, h249) = f27(Fn2::F28, 32u64, (v0, (1u32, 0u32)));
        h249
    };
    h248
}

pub fn f19(v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
    let a254 = f29(v0, 24u32);
    let a255 = {
        let a252 = f29(v1, 16u32);
        let a253 = {
            let a250 = f29(v2, 8u32);
            let a251 = v3;
            fixed_u32::bit_or(a250, a251)
        };
        fixed_u32::bit_or(a252, a253)
    };
    fixed_u32::bit_or(a254, a255)
}

pub fn f20(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a256 = v2;
    let a257 = v1;
    if nat_lt(a256, a257) {
        let m260 = {
            let a258 = v0;
            let a259 = v2;
            index_bytes(a258, a259)
        };
        match m260 {
            None => {
                0u32
            }
            Some(v3) => {
                let m262 = {
                    let a261 = v3;
                    fixed_u32::convert(i128::from(a261))
                };
                match m262 {
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
        let a263 = v2;
        let a264 = v1;
        if nat_eq(a263, a264) {
            128u32
        } else {
            0u32
        }
    }
}

pub fn f21(v0: u64) -> u32 {
    let a265 = v0;
    let a266 = 32u64;
    if nat_lt(a265, a266) {
        let a267 = v0;
        let a268 = 16u64;
        if nat_lt(a267, a268) {
            let a269 = v0;
            let a270 = 8u64;
            if nat_lt(a269, a270) {
                let a271 = v0;
                let a272 = 4u64;
                if nat_lt(a271, a272) {
                    let a273 = v0;
                    let a274 = 2u64;
                    if nat_lt(a273, a274) {
                        let a275 = v0;
                        let a276 = 1u64;
                        if nat_lt(a275, a276) {
                            1116352408u32
                        } else {
                            1899447441u32
                        }
                    } else {
                        let a277 = v0;
                        let a278 = 3u64;
                        if nat_lt(a277, a278) {
                            3049323471u32
                        } else {
                            3921009573u32
                        }
                    }
                } else {
                    let a279 = v0;
                    let a280 = 6u64;
                    if nat_lt(a279, a280) {
                        let a281 = v0;
                        let a282 = 5u64;
                        if nat_lt(a281, a282) {
                            961987163u32
                        } else {
                            1508970993u32
                        }
                    } else {
                        let a283 = v0;
                        let a284 = 7u64;
                        if nat_lt(a283, a284) {
                            2453635748u32
                        } else {
                            2870763221u32
                        }
                    }
                }
            } else {
                let a285 = v0;
                let a286 = 12u64;
                if nat_lt(a285, a286) {
                    let a287 = v0;
                    let a288 = 10u64;
                    if nat_lt(a287, a288) {
                        let a289 = v0;
                        let a290 = 9u64;
                        if nat_lt(a289, a290) {
                            3624381080u32
                        } else {
                            310598401u32
                        }
                    } else {
                        let a291 = v0;
                        let a292 = 11u64;
                        if nat_lt(a291, a292) {
                            607225278u32
                        } else {
                            1426881987u32
                        }
                    }
                } else {
                    let a293 = v0;
                    let a294 = 14u64;
                    if nat_lt(a293, a294) {
                        let a295 = v0;
                        let a296 = 13u64;
                        if nat_lt(a295, a296) {
                            1925078388u32
                        } else {
                            2162078206u32
                        }
                    } else {
                        let a297 = v0;
                        let a298 = 15u64;
                        if nat_lt(a297, a298) {
                            2614888103u32
                        } else {
                            3248222580u32
                        }
                    }
                }
            }
        } else {
            let a299 = v0;
            let a300 = 24u64;
            if nat_lt(a299, a300) {
                let a301 = v0;
                let a302 = 20u64;
                if nat_lt(a301, a302) {
                    let a303 = v0;
                    let a304 = 18u64;
                    if nat_lt(a303, a304) {
                        let a305 = v0;
                        let a306 = 17u64;
                        if nat_lt(a305, a306) {
                            3835390401u32
                        } else {
                            4022224774u32
                        }
                    } else {
                        let a307 = v0;
                        let a308 = 19u64;
                        if nat_lt(a307, a308) {
                            264347078u32
                        } else {
                            604807628u32
                        }
                    }
                } else {
                    let a309 = v0;
                    let a310 = 22u64;
                    if nat_lt(a309, a310) {
                        let a311 = v0;
                        let a312 = 21u64;
                        if nat_lt(a311, a312) {
                            770255983u32
                        } else {
                            1249150122u32
                        }
                    } else {
                        let a313 = v0;
                        let a314 = 23u64;
                        if nat_lt(a313, a314) {
                            1555081692u32
                        } else {
                            1996064986u32
                        }
                    }
                }
            } else {
                let a315 = v0;
                let a316 = 28u64;
                if nat_lt(a315, a316) {
                    let a317 = v0;
                    let a318 = 26u64;
                    if nat_lt(a317, a318) {
                        let a319 = v0;
                        let a320 = 25u64;
                        if nat_lt(a319, a320) {
                            2554220882u32
                        } else {
                            2821834349u32
                        }
                    } else {
                        let a321 = v0;
                        let a322 = 27u64;
                        if nat_lt(a321, a322) {
                            2952996808u32
                        } else {
                            3210313671u32
                        }
                    }
                } else {
                    let a323 = v0;
                    let a324 = 30u64;
                    if nat_lt(a323, a324) {
                        let a325 = v0;
                        let a326 = 29u64;
                        if nat_lt(a325, a326) {
                            3336571891u32
                        } else {
                            3584528711u32
                        }
                    } else {
                        let a327 = v0;
                        let a328 = 31u64;
                        if nat_lt(a327, a328) {
                            113926993u32
                        } else {
                            338241895u32
                        }
                    }
                }
            }
        }
    } else {
        let a329 = v0;
        let a330 = 48u64;
        if nat_lt(a329, a330) {
            let a331 = v0;
            let a332 = 40u64;
            if nat_lt(a331, a332) {
                let a333 = v0;
                let a334 = 36u64;
                if nat_lt(a333, a334) {
                    let a335 = v0;
                    let a336 = 34u64;
                    if nat_lt(a335, a336) {
                        let a337 = v0;
                        let a338 = 33u64;
                        if nat_lt(a337, a338) {
                            666307205u32
                        } else {
                            773529912u32
                        }
                    } else {
                        let a339 = v0;
                        let a340 = 35u64;
                        if nat_lt(a339, a340) {
                            1294757372u32
                        } else {
                            1396182291u32
                        }
                    }
                } else {
                    let a341 = v0;
                    let a342 = 38u64;
                    if nat_lt(a341, a342) {
                        let a343 = v0;
                        let a344 = 37u64;
                        if nat_lt(a343, a344) {
                            1695183700u32
                        } else {
                            1986661051u32
                        }
                    } else {
                        let a345 = v0;
                        let a346 = 39u64;
                        if nat_lt(a345, a346) {
                            2177026350u32
                        } else {
                            2456956037u32
                        }
                    }
                }
            } else {
                let a347 = v0;
                let a348 = 44u64;
                if nat_lt(a347, a348) {
                    let a349 = v0;
                    let a350 = 42u64;
                    if nat_lt(a349, a350) {
                        let a351 = v0;
                        let a352 = 41u64;
                        if nat_lt(a351, a352) {
                            2730485921u32
                        } else {
                            2820302411u32
                        }
                    } else {
                        let a353 = v0;
                        let a354 = 43u64;
                        if nat_lt(a353, a354) {
                            3259730800u32
                        } else {
                            3345764771u32
                        }
                    }
                } else {
                    let a355 = v0;
                    let a356 = 46u64;
                    if nat_lt(a355, a356) {
                        let a357 = v0;
                        let a358 = 45u64;
                        if nat_lt(a357, a358) {
                            3516065817u32
                        } else {
                            3600352804u32
                        }
                    } else {
                        let a359 = v0;
                        let a360 = 47u64;
                        if nat_lt(a359, a360) {
                            4094571909u32
                        } else {
                            275423344u32
                        }
                    }
                }
            }
        } else {
            let a361 = v0;
            let a362 = 56u64;
            if nat_lt(a361, a362) {
                let a363 = v0;
                let a364 = 52u64;
                if nat_lt(a363, a364) {
                    let a365 = v0;
                    let a366 = 50u64;
                    if nat_lt(a365, a366) {
                        let a367 = v0;
                        let a368 = 49u64;
                        if nat_lt(a367, a368) {
                            430227734u32
                        } else {
                            506948616u32
                        }
                    } else {
                        let a369 = v0;
                        let a370 = 51u64;
                        if nat_lt(a369, a370) {
                            659060556u32
                        } else {
                            883997877u32
                        }
                    }
                } else {
                    let a371 = v0;
                    let a372 = 54u64;
                    if nat_lt(a371, a372) {
                        let a373 = v0;
                        let a374 = 53u64;
                        if nat_lt(a373, a374) {
                            958139571u32
                        } else {
                            1322822218u32
                        }
                    } else {
                        let a375 = v0;
                        let a376 = 55u64;
                        if nat_lt(a375, a376) {
                            1537002063u32
                        } else {
                            1747873779u32
                        }
                    }
                }
            } else {
                let a377 = v0;
                let a378 = 60u64;
                if nat_lt(a377, a378) {
                    let a379 = v0;
                    let a380 = 58u64;
                    if nat_lt(a379, a380) {
                        let a381 = v0;
                        let a382 = 57u64;
                        if nat_lt(a381, a382) {
                            1955562222u32
                        } else {
                            2024104815u32
                        }
                    } else {
                        let a383 = v0;
                        let a384 = 59u64;
                        if nat_lt(a383, a384) {
                            2227730452u32
                        } else {
                            2361852424u32
                        }
                    }
                } else {
                    let a385 = v0;
                    let a386 = 62u64;
                    if nat_lt(a385, a386) {
                        let a387 = v0;
                        let a388 = 61u64;
                        if nat_lt(a387, a388) {
                            2428436474u32
                        } else {
                            2756734187u32
                        }
                    } else {
                        let a389 = v0;
                        let a390 = 63u64;
                        if nat_lt(a389, a390) {
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
    let a393 = {
        let a391 = f30(v0, 6u32, 26u32);
        let a392 = f30(v0, 11u32, 21u32);
        fixed_u32::bit_xor(a391, a392)
    };
    let a394 = f30(v0, 25u32, 7u32);
    fixed_u32::bit_xor(a393, a394)
}

pub fn f23(v0: u32, v1: u32, v2: u32) -> u32 {
    let a400 = {
        let a395 = v0;
        let a396 = v1;
        fixed_u32::bit_and(a395, a396)
    };
    let a401 = {
        let a398 = {
            let a397 = v0;
            fixed_u32::bit_not(a397)
        };
        let a399 = v2;
        fixed_u32::bit_and(a398, a399)
    };
    fixed_u32::bit_xor(a400, a401)
}

pub fn f24(v0: u32) -> u32 {
    let a404 = {
        let a402 = f30(v0, 2u32, 30u32);
        let a403 = f30(v0, 13u32, 19u32);
        fixed_u32::bit_xor(a402, a403)
    };
    let a405 = f30(v0, 22u32, 10u32);
    fixed_u32::bit_xor(a404, a405)
}

pub fn f25(v0: u32, v1: u32, v2: u32) -> u32 {
    let a414 = {
        let a410 = {
            let a406 = v0;
            let a407 = v1;
            fixed_u32::bit_and(a406, a407)
        };
        let a411 = {
            let a408 = v0;
            let a409 = v2;
            fixed_u32::bit_and(a408, a409)
        };
        fixed_u32::bit_xor(a410, a411)
    };
    let a415 = {
        let a412 = v1;
        let a413 = v2;
        fixed_u32::bit_and(a412, a413)
    };
    fixed_u32::bit_xor(a414, a415)
}

pub fn f26(v0: Adt1) -> Adt1 {
    Adt1::C0(match v0.clone() {
        Adt1::C0(_, h416, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h416
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h417, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h417
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h418, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h418
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h419, _, _, _, _, _, _, _, _, _, _, _) => {
            h419
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, h420, _, _, _, _, _, _, _, _, _, _) => {
            h420
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h421, _, _, _, _, _, _, _, _, _) => {
            h421
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h422, _, _, _, _, _, _, _, _) => {
            h422
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, h423, _, _, _, _, _, _, _) => {
            h423
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, h424, _, _, _, _, _, _) => {
            h424
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, h425, _, _, _, _, _) => {
            h425
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, h426, _, _, _, _) => {
            h426
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, h427, _, _, _) => {
            h427
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h428, _, _) => {
            h428
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h429, _) => {
            h429
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h430) => {
            h430
        }
    }, f13(f31(match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h431, _) => {
            h431
        }
    }), f13(match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, h432, _, _, _, _, _, _) => {
            h432
        }
    }, f13(f32(match v0.clone() {
        Adt1::C0(_, h433, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h433
        }
    }), match v0 {
        Adt1::C0(h434, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h434
        }
    }))))
}

pub fn f27(v0: Fn2, v1: u64, v2: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m435 = v1;
    if m435 == 0 {
        v2
    } else {
        let v3 = m435 - 1;
        f27(v0.clone(), v3, {
            let c436: Fn2 = v0;
            c436.apply(v2)
        })
    }
}

pub fn f28(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h437, _) = v0;
        h437
    };
    let v2: u32 = {
        let (h438, _) = {
            let (_, h439) = v0;
            h439
        };
        h438
    };
    let v3: u32 = {
        let (_, h440) = {
            let (_, h441) = v0;
            h441
        };
        h440
    };
    ({
        let a442 = v1;
        let a443 = 2u64;
        let a444 = 0u64;
        nat_quot(a442, a443, a444)
    }, (f29(v2, 1u32), {
        let a448 = {
            let a445 = v1;
            let a446 = 2u64;
            let a447 = 0u64;
            nat_rem(a445, a446, a447)
        };
        let a449 = 1u64;
        if nat_eq(a448, a449) {
            let a450 = v3;
            let a451 = v2;
            fixed_u32::bit_or(a450, a451)
        } else {
            v3
        }
    }))
}

pub fn f29(v0: u32, v1: u32) -> u32 {
    let m454 = {
        let a452 = v0;
        let a453 = v1;
        fixed_u32::shift_left(a452, a453)
    };
    match m454 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f30(v0: u32, v1: u32, v2: u32) -> u32 {
    let a455 = f8(v0, v1);
    let a456 = f29(v0, v2);
    fixed_u32::bit_or(a455, a456)
}

pub fn f31(v0: u32) -> u32 {
    let a459 = {
        let a457 = f30(v0, 17u32, 15u32);
        let a458 = f30(v0, 19u32, 13u32);
        fixed_u32::bit_xor(a457, a458)
    };
    let a460 = f8(v0, 10u32);
    fixed_u32::bit_xor(a459, a460)
}

pub fn f32(v0: u32) -> u32 {
    let a463 = {
        let a461 = f30(v0, 7u32, 25u32);
        let a462 = f30(v0, 18u32, 14u32);
        fixed_u32::bit_xor(a461, a462)
    };
    let a464 = f8(v0, 3u32);
    fixed_u32::bit_xor(a463, a464)
}

pub fn run(p0: Bytes) -> R<Bytes> {
    f0(p0)
}
