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
    C1,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    C9,
}

#[derive(Clone)]
pub enum Fn0 {
    F4,
}

impl Fn0 {
    pub fn apply(&self, p0: (Option<(u64, i64)>, List<u64>), p1: i64) -> (Option<(u64, i64)>, List<u64>) {
        match self {
            Fn0::F4 => f4(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F12(List<i64>),
    F18(List<i64>),
}

impl Fn1 {
    pub fn apply(&self, p0: (List<i64>, List<i64>), p1: List<i64>) -> R<(List<i64>, List<i64>)> {
        match self {
            Fn1::F12(k0) => f12(k0.clone(), p0, p1),
            Fn1::F18(k0) => f18(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F16,
    F17,
}

impl Fn2 {
    pub fn apply(&self, p0: List<i64>, p1: i64) -> R<List<i64>> {
        match self {
            Fn2::F16 => f16(p0, p1),
            Fn2::F17 => Ok(f17(p0, p1)),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F23,
    F24,
}

impl Fn3 {
    pub fn apply(&self, p0: (i64, List<i64>), p1: i64) -> R<(i64, List<i64>)> {
        match self {
            Fn3::F23 => f23(p0, p1),
            Fn3::F24 => f24(p0, p1),
        }
    }
}

pub fn f0(v0: Adt0) -> R<u64> {
    f1(v0.clone())
}

pub fn f1(v0: Adt0) -> R<u64> {
    f2(v0.clone())
}

pub fn f2(v0: Adt0) -> R<u64> {
    let m2 = {
        let (h1, _) = f3(Fn0::F4, (None::<(u64, i64)>, List::cons(0u64, List::cons(1u64, List::cons(2u64, List::cons(3u64, List::cons(4u64, List::cons(5u64, List::cons(6u64, List::cons(7u64, List::cons(8u64, List::cons(9u64, List::<u64>::nil()))))))))))), f5(v0.clone())?);
        h1
    };
    match m2 {
        None => {
            Ok(0u64)
        }
        Some(v1) => {
            let (h3, _) = v1;
            Ok(h3)
        }
    }
}

pub fn f3(v0: Fn0, v1: (Option<(u64, i64)>, List<u64>), v2: List<i64>) -> (Option<(u64, i64)>, List<u64>) {
    let m4 = v2.clone();
    match m4.uncons() {
        None => {
            v1.clone()
        }
        Some((v3, v4)) => {
            f3(v0.clone(), {
                let c5: Fn0 = v0.clone();
                c5.apply(v1.clone(), v3)
            }, v4.clone())
        }
    }
}

pub fn f4(v0: (Option<(u64, i64)>, List<u64>), v1: i64) -> (Option<(u64, i64)>, List<u64>) {
    let m7 = {
        let (_, h6) = v0.clone();
        h6
    };
    match m7.uncons() {
        None => {
            v0.clone()
        }
        Some((v2, v3)) => {
            ({
                let m9 = {
                    let (h8, _) = v0.clone();
                    h8
                };
                match m9 {
                    None => {
                        Some((v2, v1))
                    }
                    Some(v4) => {
                        let m13 = {
                            let a11 = {
                                let (_, h10) = v4;
                                h10
                            };
                            let a12 = v1;
                            compare(a11, a12)
                        };
                        if match m13 {
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
                            Some((v2, v1))
                        } else {
                            Some(v4)
                        }
                    }
                }
            }, v3.clone())
        }
    }
}

pub fn f5(v0: Adt0) -> R<List<i64>> {
    f6(f7(f8(f9(f10(v0.clone()))?)?)?)
}

pub fn f6(v0: List<i64>) -> R<List<i64>> {
    let (h14, _) = f11(Fn1::F12(v0.clone()), (List::<i64>::nil(), f13()), f14())?;
    Ok(h14)
}

pub fn f7(v0: List<i64>) -> R<List<i64>> {
    f15(Fn2::F16, List::<i64>::nil(), v0.clone())
}

pub fn f8(v0: List<i64>) -> R<List<i64>> {
    f15(Fn2::F17, List::<i64>::nil(), v0.clone())
}

pub fn f9(v0: List<i64>) -> R<List<i64>> {
    let (h15, _) = f11(Fn1::F18(v0.clone()), (List::<i64>::nil(), f19()), f20())?;
    Ok(h15)
}

pub fn f10(v0: Adt0) -> List<i64> {
    f21(v0.clone())
}

pub fn f11(v0: Fn1, v1: (List<i64>, List<i64>), v2: List<List<i64>>) -> R<(List<i64>, List<i64>)> {
    let m16 = v2.clone();
    match m16.uncons() {
        None => {
            Ok(v1.clone())
        }
        Some((v3, v4)) => {
            f11(v0.clone(), {
                let c17: Fn1 = v0.clone();
                c17.apply(v1.clone(), v3.clone())?
            }, v4.clone())
        }
    }
}

pub fn f12(v0: List<i64>, v1: (List<i64>, List<i64>), v2: List<i64>) -> R<(List<i64>, List<i64>)> {
    let m19 = {
        let (_, h18) = v1.clone();
        h18
    };
    match m19.uncons() {
        None => {
            Ok(v1.clone())
        }
        Some((v3, v4)) => {
            Ok(({
                let a25 = {
                    let (h20, _) = v1.clone();
                    h20
                };
                let a26 = List::cons({
                    let a23 = v3;
                    let a24 = {
                        let a22 = {
                            let (h21, _) = f22(Fn3::F23, (0i64, v0.clone()), v2.clone())?;
                            h21
                        };
                        int_neg(a22)?
                    };
                    int_sub(a23, a24)?
                }, List::<i64>::nil());
                append_list(a25, a26)
            }, v4.clone()))
        }
    }
}

pub fn f13() -> List<i64> {
    List::cons(-6i64, List::cons(-2i64, List::cons(-5i64, List::cons(-5i64, List::cons(-4i64, List::cons(-5i64, List::cons(-6i64, List::cons(-3i64, List::cons(-7i64, List::cons(-6i64, List::<i64>::nil()))))))))))
}

pub fn f14() -> List<List<i64>> {
    List::cons(List::cons(2i64, List::cons(2i64, List::cons(2i64, List::cons(2i64, List::cons(2i64, List::cons(2i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::<i64>::nil())))))))))))), List::cons(List::cons(0i64, List::cons(2i64, List::cons(2i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::<i64>::nil())))))))))))), List::cons(List::cons(2i64, List::cons(2i64, List::cons(0i64, List::cons(2i64, List::cons(2i64, List::cons(0i64, List::cons(2i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::<i64>::nil())))))))))))), List::cons(List::cons(2i64, List::cons(2i64, List::cons(2i64, List::cons(2i64, List::cons(0i64, List::cons(0i64, List::cons(2i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::<i64>::nil())))))))))))), List::cons(List::cons(0i64, List::cons(2i64, List::cons(2i64, List::cons(0i64, List::cons(0i64, List::cons(2i64, List::cons(2i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::<i64>::nil())))))))))))), List::cons(List::cons(2i64, List::cons(0i64, List::cons(2i64, List::cons(2i64, List::cons(0i64, List::cons(2i64, List::cons(2i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::<i64>::nil())))))))))))), List::cons(List::cons(2i64, List::cons(0i64, List::cons(2i64, List::cons(2i64, List::cons(2i64, List::cons(2i64, List::cons(2i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::<i64>::nil())))))))))))), List::cons(List::cons(2i64, List::cons(2i64, List::cons(2i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::<i64>::nil())))))))))))), List::cons(List::cons(2i64, List::cons(2i64, List::cons(2i64, List::cons(2i64, List::cons(2i64, List::cons(2i64, List::cons(2i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::<i64>::nil())))))))))))), List::cons(List::cons(2i64, List::cons(2i64, List::cons(2i64, List::cons(2i64, List::cons(0i64, List::cons(2i64, List::cons(2i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::<i64>::nil())))))))))))), List::<List<i64>>::nil()))))))))))
}

pub fn f15(v0: Fn2, v1: List<i64>, v2: List<i64>) -> R<List<i64>> {
    let m27 = v2.clone();
    match m27.uncons() {
        None => {
            Ok(v1.clone())
        }
        Some((v3, v4)) => {
            f15(v0.clone(), {
                let c28: Fn2 = v0.clone();
                c28.apply(v1.clone(), v3)?
            }, v4.clone())
        }
    }
}

pub fn f16(v0: List<i64>, v1: i64) -> R<List<i64>> {
    let a38 = v0.clone();
    let a39 = List::cons({
        let v2: i64 = {
            let a29 = v1;
            let a30 = 4i64;
            let a31 = 0i64;
            int_quot(a29, a30, a31)?
        };
        let m34 = {
            let a32 = v2;
            let a33 = 0i64;
            compare(a32, a33)
        };
        if match m34 {
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
            0i64
        } else {
            let m37 = {
                let a35 = 127i64;
                let a36 = v2;
                compare(a35, a36)
            };
            if match m37 {
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
                127i64
            } else {
                v2
            }
        }
    }, List::<i64>::nil());
    Ok(append_list(a38, a39))
}

pub fn f17(v0: List<i64>, v1: i64) -> List<i64> {
    let a43 = v0.clone();
    let a44 = List::cons({
        let m42 = {
            let a40 = v1;
            let a41 = 0i64;
            compare(a40, a41)
        };
        if match m42 {
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
            0i64
        } else {
            v1
        }
    }, List::<i64>::nil());
    append_list(a43, a44)
}

pub fn f18(v0: List<i64>, v1: (List<i64>, List<i64>), v2: List<i64>) -> R<(List<i64>, List<i64>)> {
    let m46 = {
        let (_, h45) = v1.clone();
        h45
    };
    match m46.uncons() {
        None => {
            Ok(v1.clone())
        }
        Some((v3, v4)) => {
            Ok(({
                let a52 = {
                    let (h47, _) = v1.clone();
                    h47
                };
                let a53 = List::cons({
                    let a50 = v3;
                    let a51 = {
                        let a49 = {
                            let (h48, _) = f22(Fn3::F24, (0i64, v0.clone()), v2.clone())?;
                            h48
                        };
                        int_neg(a49)?
                    };
                    int_sub(a50, a51)?
                }, List::<i64>::nil());
                append_list(a52, a53)
            }, v4.clone()))
        }
    }
}

pub fn f19() -> List<i64> {
    List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(1i64, List::cons(-2i64, List::cons(3i64, List::cons(-5i64, List::cons(2i64, List::<i64>::nil()))))))))))))
}

pub fn f20() -> List<List<i64>> {
    List::cons(List::cons(4i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::<i64>::nil()))))))), List::cons(List::cons(0i64, List::cons(4i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::<i64>::nil()))))))), List::cons(List::cons(0i64, List::cons(0i64, List::cons(4i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::<i64>::nil()))))))), List::cons(List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(4i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::<i64>::nil()))))))), List::cons(List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(4i64, List::cons(0i64, List::cons(0i64, List::<i64>::nil()))))))), List::cons(List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(4i64, List::cons(0i64, List::<i64>::nil()))))))), List::cons(List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(4i64, List::<i64>::nil()))))))), List::cons(List::cons(3i64, List::cons(-2i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(-1i64, List::<i64>::nil()))))))), List::cons(List::cons(0i64, List::cons(0i64, List::cons(2i64, List::cons(-3i64, List::cons(1i64, List::cons(0i64, List::cons(0i64, List::<i64>::nil()))))))), List::cons(List::cons(-1i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(2i64, List::cons(2i64, List::cons(-2i64, List::<i64>::nil()))))))), List::cons(List::cons(1i64, List::cons(1i64, List::cons(1i64, List::cons(1i64, List::cons(1i64, List::cons(1i64, List::cons(1i64, List::<i64>::nil()))))))), List::cons(List::cons(0i64, List::cons(-4i64, List::cons(0i64, List::cons(2i64, List::cons(0i64, List::cons(3i64, List::cons(0i64, List::<i64>::nil()))))))), List::<List<i64>>::nil()))))))))))))
}

pub fn f21(v0: Adt0) -> List<i64> {
    let m54 = v0.clone();
    match m54 {
        Adt0::C0 => {
            List::cons(1i64, List::cons(1i64, List::cons(1i64, List::cons(1i64, List::cons(1i64, List::cons(1i64, List::cons(0i64, List::<i64>::nil())))))))
        }
        Adt0::C1 => {
            List::cons(0i64, List::cons(1i64, List::cons(1i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::<i64>::nil())))))))
        }
        Adt0::C2 => {
            List::cons(1i64, List::cons(1i64, List::cons(0i64, List::cons(1i64, List::cons(1i64, List::cons(0i64, List::cons(1i64, List::<i64>::nil())))))))
        }
        Adt0::C3 => {
            List::cons(1i64, List::cons(1i64, List::cons(1i64, List::cons(1i64, List::cons(0i64, List::cons(0i64, List::cons(1i64, List::<i64>::nil())))))))
        }
        Adt0::C4 => {
            List::cons(0i64, List::cons(1i64, List::cons(1i64, List::cons(0i64, List::cons(0i64, List::cons(1i64, List::cons(1i64, List::<i64>::nil())))))))
        }
        Adt0::C5 => {
            List::cons(1i64, List::cons(0i64, List::cons(1i64, List::cons(1i64, List::cons(0i64, List::cons(1i64, List::cons(1i64, List::<i64>::nil())))))))
        }
        Adt0::C6 => {
            List::cons(1i64, List::cons(0i64, List::cons(1i64, List::cons(1i64, List::cons(1i64, List::cons(1i64, List::cons(1i64, List::<i64>::nil())))))))
        }
        Adt0::C7 => {
            List::cons(1i64, List::cons(1i64, List::cons(1i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::<i64>::nil())))))))
        }
        Adt0::C8 => {
            List::cons(1i64, List::cons(1i64, List::cons(1i64, List::cons(1i64, List::cons(1i64, List::cons(1i64, List::cons(1i64, List::<i64>::nil())))))))
        }
        Adt0::C9 => {
            List::cons(1i64, List::cons(1i64, List::cons(1i64, List::cons(1i64, List::cons(0i64, List::cons(1i64, List::cons(1i64, List::<i64>::nil())))))))
        }
    }
}

pub fn f22(v0: Fn3, v1: (i64, List<i64>), v2: List<i64>) -> R<(i64, List<i64>)> {
    let m55 = v2.clone();
    match m55.uncons() {
        None => {
            Ok(v1.clone())
        }
        Some((v3, v4)) => {
            f22(v0.clone(), {
                let c56: Fn3 = v0.clone();
                c56.apply(v1.clone(), v3)?
            }, v4.clone())
        }
    }
}

pub fn f23(v0: (i64, List<i64>), v1: i64) -> R<(i64, List<i64>)> {
    let m58 = {
        let (_, h57) = v0.clone();
        h57
    };
    match m58.uncons() {
        None => {
            Ok(v0.clone())
        }
        Some((v2, v3)) => {
            Ok(({
                let a63 = {
                    let (h59, _) = v0.clone();
                    h59
                };
                let a64 = {
                    let a62 = {
                        let a60 = v1;
                        let a61 = v2;
                        int_mul(a60, a61)?
                    };
                    int_neg(a62)?
                };
                int_sub(a63, a64)?
            }, v3.clone()))
        }
    }
}

pub fn f24(v0: (i64, List<i64>), v1: i64) -> R<(i64, List<i64>)> {
    let m66 = {
        let (_, h65) = v0.clone();
        h65
    };
    match m66.uncons() {
        None => {
            Ok(v0.clone())
        }
        Some((v2, v3)) => {
            Ok(({
                let a71 = {
                    let (h67, _) = v0.clone();
                    h67
                };
                let a72 = {
                    let a70 = {
                        let a68 = v1;
                        let a69 = v2;
                        int_mul(a68, a69)?
                    };
                    int_neg(a70)?
                };
                int_sub(a71, a72)?
            }, v3.clone()))
        }
    }
}
