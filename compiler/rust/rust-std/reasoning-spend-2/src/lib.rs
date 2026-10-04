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
pub struct List<T>(Option<std::rc::Rc<Node<T>>>);
impl<T> Clone for List<T> { fn clone(&self) -> Self { List(self.0.clone()) } }
impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut next = self.0.take();
        while let Some(cell) = next { // release
            match std::rc::Rc::try_unwrap(cell) {
                Ok(mut node) => next = node.tail.0.take(),
                Err(_) => break,
            }
        }
    }
}
impl<T: Clone> List<T> {
    pub fn nil() -> Self { List(None) }
    pub fn cons(head: T, tail: List<T>) -> Self { List(Some(std::rc::Rc::new(Node { head, tail }))) }
    pub fn uncons(&self) -> Option<(T, List<T>)> { self.0.as_ref().map(|node| (node.head.clone(), node.tail.clone())) }
    fn items(&self) -> Vec<T> {
        let mut out = Vec::new();
        let mut cursor = self.0.as_ref();
        while let Some(node) = cursor { tick(1); out.push(node.head.clone()); cursor = node.tail.0.as_ref(); }
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
pub fn length_list<T>(a: List<T>) -> u64 {
    let mut count = 0;
    let mut cursor = a.0.as_ref();
    while let Some(node) = cursor { tick(1); count += 1; cursor = node.tail.0.as_ref(); }
    count
}
pub fn length_bytes(a: Bytes) -> u64 { a.0.len() as u64 }
pub fn length_string(a: Str) -> u64 { chars(&a.0) }
pub fn index_list<T: Clone>(a: List<T>, i: u64) -> Option<T> {
    let mut position = 0;
    let mut cursor = a.0.as_ref();
    while let Some(node) = cursor {
        tick(1);
        if position == i { return Some(node.head.clone()); }
        position += 1;
        cursor = node.tail.0.as_ref();
    }
    None
}
pub fn index_bytes(a: Bytes, i: u64) -> Option<u8> { usize::try_from(i).ok().and_then(|i| a.0.get(i)).copied() }
pub fn slice_list<T: Clone>(a: List<T>, start: u64, count: u64) -> Option<List<T>> {
    let end = u128::from(start) + u128::from(count);
    let mut part = Vec::new();
    let mut position: u128 = 0;
    let mut cursor = a.0.as_ref();
    while let Some(node) = cursor {
        tick(1);
        if position >= end { break; }
        if position >= u128::from(start) { part.push(node.head.clone()); }
        position += 1;
        cursor = node.tail.0.as_ref();
    }
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
    let mut cursor = texts.0.as_ref();
    let mut first = true;
    while let Some(node) = cursor {
        tick(1);
        if !first { chars(&delimiter.0); out.push_str(&delimiter.0); }
        first = false;
        chars(&node.head.0);
        out.push_str(&node.head.0);
        cursor = node.tail.0.as_ref();
    }
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
    C1(u64),
}

#[derive(Clone)]
pub enum Fn0 {
    F12,
}

impl Fn0 {
    pub fn apply(&self, p0: (u64, u64)) -> Option<(u64, u64)> {
        match self {
            Fn0::F12 => f12(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F9((u64, u64)),
}

impl Fn1 {
    pub fn apply(&self, p0: Option<Adt0>, p1: u64) -> Option<Adt0> {
        match self {
            Fn1::F9(k0) => f9(*k0, p0, p1),
        }
    }
}

pub fn f0(v0: (u64, u64)) -> R<Result<u64, (bool, bool)>> {
    let v1: ((u64, u64), bool) = f18(Fn0::F12, {
        let a2 = {
            let (_, h1) = v0;
            h1
        };
        let a3 = 4u64;
        nat_add(a2, a3)?
    }, f1(v0)?);
    let (_, h4) = v1;
    if h4 {
        Ok(f16(v0, {
            let (h5, _) = v1;
            h5
        }))
    } else {
        Ok(Err::<u64, (bool, bool)>((false, false)))
    }
}

pub fn f1(v0: (u64, u64)) -> R<(u64, u64)> {
    Ok(({
        let (h6, _) = v0;
        h6
    }, {
        let a8 = {
            let (_, h7) = v0;
            h7
        };
        let a9 = 3u64;
        nat_add(a8, a9)?
    }))
}

pub fn f2(v0: (u64, u64)) -> bool {
    let a11 = 2u64;
    let a12 = {
        let (_, h10) = v0;
        h10
    };
    nat_lt(a11, a12)
}

pub fn f3(v0: (u64, u64)) -> (u64, u64) {
    ({
        let (h13, _) = v0;
        h13
    }, {
        let a15 = {
            let (_, h14) = v0;
            h14
        };
        let a16 = 1u64;
        nat_sub(a15, a16)
    })
}

pub fn f4(v0: (u64, u64)) -> Option<(u64, u64)> {
    if f2(v0) {
        Some(f3(v0))
    } else {
        None::<(u64, u64)>
    }
}

pub fn f5(v0: (u64, u64), v1: u64) -> bool {
    let a22 = {
        let a18 = v1;
        let a19 = {
            let (_, h17) = v0;
            h17
        };
        nat_le(a18, a19)
    };
    let a23 = {
        let a20 = 0u64;
        let a21 = v1;
        nat_lt(a20, a21)
    };
    bool_and(a22, a23)
}

pub fn f6(v0: (u64, u64), v1: u64) -> (u64, u64) {
    ({
        let (h24, _) = v0;
        h24
    }, {
        let a26 = {
            let (_, h25) = v0;
            h25
        };
        let a27 = v1;
        nat_sub(a26, a27)
    })
}

pub fn f7(_: (u64, u64)) -> List<u64> {
    List::cons(2u64, List::cons(1u64, List::<u64>::nil()))
}

pub fn f8(v0: (u64, u64), v1: u64) -> Option<(u64, u64)> {
    if f5(v0, v1) {
        Some(f6(v0, v1))
    } else {
        None::<(u64, u64)>
    }
}

pub fn f9(v0: (u64, u64), v1: Option<Adt0>, v2: u64) -> Option<Adt0> {
    let m28 = v1.clone();
    match m28 {
        None => {
            if f5(v0, v2) {
                Some(Adt0::C1(v2))
            } else {
                None::<Adt0>
            }
        }
        Some(v3) => {
            Some(v3.clone())
        }
    }
}

pub fn f10(v0: (u64, u64)) -> Option<Adt0> {
    let m29 = if f2(v0) {
        Some(Adt0::C0)
    } else {
        None::<Adt0>
    };
    match m29 {
        None => {
            f17(Fn1::F9(v0), None::<Adt0>, f7(v0))
        }
        Some(v1) => {
            Some(v1.clone())
        }
    }
}

pub fn f11(v0: (u64, u64), v1: Adt0) -> Option<(u64, u64)> {
    let m30 = v1.clone();
    match m30 {
        Adt0::C0 => {
            f4(v0)
        }
        Adt0::C1(v2) => {
            f8(v0, v2)
        }
    }
}

pub fn f12(v0: (u64, u64)) -> Option<(u64, u64)> {
    let m31 = f10(v0);
    match m31 {
        None => {
            None::<(u64, u64)>
        }
        Some(v1) => {
            f11(v0, v1.clone())
        }
    }
}

pub fn f13(v0: (u64, u64)) -> Option<u64> {
    Some({
        let (_, h32) = v0;
        h32
    })
}

pub fn f14(_: (u64, u64), v1: u64) -> bool {
    let a33 = v1;
    let a34 = 0u64;
    nat_eq(a33, a34)
}

pub fn f15(v0: (u64, u64), v1: (u64, u64)) -> Option<u64> {
    let m35 = f13(v1);
    match m35 {
        None => {
            None::<u64>
        }
        Some(v2) => {
            let v3: bool = f14(v0, v2);
            if v3 {
                Some(v2)
            } else {
                None::<u64>
            }
        }
    }
}

pub fn f16(v0: (u64, u64), v1: (u64, u64)) -> Result<u64, (bool, bool)> {
    let m36 = f15(v0, v1);
    match m36 {
        None => {
            let m37 = f13(v1);
            match m37 {
                None => {
                    Err::<u64, (bool, bool)>((false, true))
                }
                Some(_) => {
                    Err::<u64, (bool, bool)>((true, false))
                }
            }
        }
        Some(v2) => {
            Ok::<u64, (bool, bool)>(v2)
        }
    }
}

pub fn f17(v0: Fn1, v1: Option<Adt0>, v2: List<u64>) -> Option<Adt0> {
    let m38 = v2.clone();
    match m38.uncons() {
        None => {
            v1.clone()
        }
        Some((v3, v4)) => {
            f17(v0.clone(), {
                let c39: Fn1 = v0.clone();
                c39.apply(v1.clone(), v3)
            }, v4.clone())
        }
    }
}

pub fn f18(v0: Fn0, v1: u64, v2: (u64, u64)) -> ((u64, u64), bool) {
    let m40 = v1;
    if m40 == 0 {
        (v2, false)
    } else {
        let v3 = m40 - 1;
        let m42 = {
            let c41: Fn0 = v0.clone();
            c41.apply(v2)
        };
        match m42 {
            None => {
                (v2, true)
            }
            Some(v4) => {
                f18(v0.clone(), v3, v4)
            }
        }
    }
}

pub fn run(p0: (u64, u64)) -> R<Result<u64, (bool, bool)>> {
    f0(p0)
}
