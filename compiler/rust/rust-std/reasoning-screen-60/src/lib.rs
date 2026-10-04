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
    C0(u64),
}

#[derive(Clone)]
pub enum Adt1 {
    C0((u64, u64), List<Adt0>),
}

#[derive(Clone)]
pub enum Adt2 {
    C0(List<Adt1>, Option<(u64, Adt1)>, bool),
}

#[derive(Clone)]
pub enum Fn0 {
    F15(u64),
}

impl Fn0 {
    pub fn apply(&self, p0: Adt2) -> R<Option<Adt2>> {
        match self {
            Fn0::F15(k0) => f15(*k0, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F11(Adt1),
}

impl Fn1 {
    pub fn apply(&self, p0: List<Adt1>, p1: u64) -> List<Adt1> {
        match self {
            Fn1::F11(k0) => f11(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F13(u64),
}

impl Fn2 {
    pub fn apply(&self, p0: List<Adt1>, p1: Adt1) -> List<Adt1> {
        match self {
            Fn2::F13(k0) => f13(*k0, p0, p1),
        }
    }
}

pub fn f0(v0: u64) -> R<Result<u64, (bool, bool)>> {
    let v1: (Adt2, bool) = f18(Fn0::F15(v0), 8u64, f14(v0))?;
    let m4 = {
        let m3 = {
            let (h2, _) = v1.clone();
            h2
        };
        match m3 {
            Adt2::C0(_, h1, _) => {
                h1
            }
        }
    };
    match m4 {
        None => {
            Ok(Err::<u64, (bool, bool)>({
                let a11 = {
                    let (_, h6) = v1.clone();
                    h6
                };
                let a12 = {
                    let a10 = {
                        let m9 = {
                            let (h8, _) = v1.clone();
                            h8
                        };
                        match m9 {
                            Adt2::C0(_, _, h7) => {
                                h7
                            }
                        }
                    };
                    bool_not(a10)
                };
                if bool_and(a11, a12) {
                    (false, true)
                } else {
                    (false, false)
                }
            }))
        }
        Some(v2) => {
            Ok(Ok::<u64, (bool, bool)>({
                let (h5, _) = v2.clone();
                h5
            }))
        }
    }
}

pub fn f1(v0: (u64, u64), v1: u64) -> bool {
    let a18 = {
        let a14 = {
            let (_, h13) = v0;
            h13
        };
        let a15 = 0u64;
        nat_eq(a14, a15)
    };
    let a19 = {
        let a16 = 0u64;
        let a17 = v1;
        nat_lt(a16, a17)
    };
    bool_and(a18, a19)
}

pub fn f2(v0: (u64, u64), v1: u64) -> (u64, u64) {
    ({
        let (h20, _) = v0;
        h20
    }, v1)
}

pub fn f3(v0: (u64, u64), v1: u64) -> Option<(u64, u64)> {
    if f1(v0, v1) {
        Some(f2(v0, v1))
    } else {
        None::<(u64, u64)>
    }
}

pub fn f4(v0: u64) -> List<u64> {
    let v1: List<u64> = List::cons({
        let a21 = v0;
        let a22 = 10u64;
        nat_sub(a21, a22)
    }, List::cons({
        let a23 = v0;
        let a24 = 20u64;
        nat_sub(a23, a24)
    }, List::cons({
        let a25 = v0;
        let a26 = 30u64;
        nat_sub(a25, a26)
    }, List::cons({
        let a27 = v0;
        let a28 = 50u64;
        nat_sub(a27, a28)
    }, List::<u64>::nil()))));
    let a30 = {
        let a29 = v1.clone();
        length_list(a29)
    };
    let a31 = 4u64;
    if nat_le(a30, a31) {
        v1.clone()
    } else {
        List::<u64>::nil()
    }
}

pub fn f5(v0: (u64, u64)) -> List<u64> {
    f4({
        let (h32, _) = v0;
        h32
    })
}

pub fn f6(v0: (u64, u64), v1: Adt0) -> Option<(u64, u64)> {
    let m33 = v1.clone();
    match m33 {
        Adt0::C0(v2) => {
            f3(v0, v2)
        }
    }
}

pub fn f7(v0: u64) -> (u64, u64) {
    (v0, 0u64)
}

pub fn f8(v0: (u64, u64)) -> Option<u64> {
    let a35 = 0u64;
    let a36 = {
        let (_, h34) = v0;
        h34
    };
    if nat_lt(a35, a36) {
        Some({
            let (_, h37) = v0;
            h37
        })
    } else {
        None::<u64>
    }
}

pub fn f9(v0: u64, v1: u64) -> R<bool> {
    let a40 = {
        let a38 = v1;
        let a39 = v1;
        nat_add(a38, a39)?
    };
    let a41 = v0;
    Ok(nat_le(a40, a41))
}

pub fn f10(v0: u64, v1: (u64, u64)) -> R<Option<u64>> {
    let m42 = f8(v1);
    match m42 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            let v3: bool = f9(v0, v2)?;
            if v3 {
                Ok(Some(v2))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f11(v0: Adt1, v1: List<Adt1>, v2: u64) -> List<Adt1> {
    let m44 = f6(match v0.clone() {
        Adt1::C0(h43, _) => {
            h43
        }
    }, Adt0::C0(v2));
    match m44 {
        None => {
            v1.clone()
        }
        Some(v3) => {
            let a48 = v1.clone();
            let a49 = List::cons(Adt1::C0(v3, {
                let a46 = match v0.clone() {
                    Adt1::C0(_, h45) => {
                        h45
                    }
                };
                let a47 = List::cons(Adt0::C0(v2), List::<Adt0>::nil());
                append_list(a46, a47)
            }), List::<Adt1>::nil());
            append_list(a48, a49)
        }
    }
}

pub fn f12(v0: Adt1) -> List<Adt1> {
    f17(Fn1::F11(v0.clone()), List::<Adt1>::nil(), f5(match v0.clone() {
        Adt1::C0(h50, _) => {
            h50
        }
    }))
}

pub fn f13(v0: u64, v1: List<Adt1>, v2: Adt1) -> List<Adt1> {
    let a52 = {
        let a51 = v1.clone();
        length_list(a51)
    };
    let a53 = v0;
    if nat_lt(a52, a53) {
        let a54 = v1.clone();
        let a55 = List::cons(v2.clone(), List::<Adt1>::nil());
        append_list(a54, a55)
    } else {
        v1.clone()
    }
}

pub fn f14(v0: u64) -> Adt2 {
    let v1: List<Adt1> = f16(Fn2::F13(3u64), List::<Adt1>::nil(), List::cons(Adt1::C0(f7(v0), List::<Adt0>::nil()), List::<Adt1>::nil()));
    Adt2::C0(v1.clone(), None::<(u64, Adt1)>, {
        let a56 = 3u64;
        let a57 = 1u64;
        nat_lt(a56, a57)
    })
}

pub fn f15(v0: u64, v1: Adt2) -> R<Option<Adt2>> {
    let m59 = match v1.clone() {
        Adt2::C0(_, h58, _) => {
            h58
        }
    };
    match m59 {
        None => {
            let m61 = match v1.clone() {
                Adt2::C0(h60, _, _) => {
                    h60
                }
            };
            match m61.uncons() {
                None => {
                    Ok(None::<Adt2>)
                }
                Some((v3, v4)) => {
                    let m63 = f10(v0, match v3.clone() {
                        Adt1::C0(h62, _) => {
                            h62
                        }
                    })?;
                    match m63 {
                        None => {
                            let v6: List<Adt1> = f12(v3.clone());
                            let v7: List<Adt1> = {
                                let a65 = v6.clone();
                                let a66 = v4.clone();
                                append_list(a65, a66)
                            };
                            let v8: List<Adt1> = f16(Fn2::F13(3u64), List::<Adt1>::nil(), v7.clone());
                            Ok(Some(Adt2::C0(v8.clone(), None::<(u64, Adt1)>, {
                                let a71 = match v1.clone() {
                                    Adt2::C0(_, _, h67) => {
                                        h67
                                    }
                                };
                                let a72 = {
                                    let a69 = 3u64;
                                    let a70 = {
                                        let a68 = v7.clone();
                                        length_list(a68)
                                    };
                                    nat_lt(a69, a70)
                                };
                                bool_or(a71, a72)
                            })))
                        }
                        Some(v5) => {
                            Ok(Some(Adt2::C0(v4.clone(), Some((v5, v3.clone())), match v1.clone() {
                                Adt2::C0(_, _, h64) => {
                                    h64
                                }
                            })))
                        }
                    }
                }
            }
        }
        Some(_) => {
            Ok(None::<Adt2>)
        }
    }
}

pub fn f16(v0: Fn2, v1: List<Adt1>, v2: List<Adt1>) -> List<Adt1> {
    let m73 = v2.clone();
    match m73.uncons() {
        None => {
            v1.clone()
        }
        Some((v3, v4)) => {
            f16(v0.clone(), {
                let c74: Fn2 = v0.clone();
                c74.apply(v1.clone(), v3.clone())
            }, v4.clone())
        }
    }
}

pub fn f17(v0: Fn1, v1: List<Adt1>, v2: List<u64>) -> List<Adt1> {
    let m75 = v2.clone();
    match m75.uncons() {
        None => {
            v1.clone()
        }
        Some((v3, v4)) => {
            f17(v0.clone(), {
                let c76: Fn1 = v0.clone();
                c76.apply(v1.clone(), v3)
            }, v4.clone())
        }
    }
}

pub fn f18(v0: Fn0, v1: u64, v2: Adt2) -> R<(Adt2, bool)> {
    let m77 = v1;
    if m77 == 0 {
        Ok((v2.clone(), false))
    } else {
        let v3 = m77 - 1;
        let m79 = {
            let c78: Fn0 = v0.clone();
            c78.apply(v2.clone())?
        };
        match m79 {
            None => {
                Ok((v2.clone(), true))
            }
            Some(v4) => {
                f18(v0.clone(), v3, v4.clone())
            }
        }
    }
}

pub fn run(p0: u64) -> R<Result<u64, (bool, bool)>> {
    f0(p0)
}
