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

pub fn f0(v0: List<(u64, List<u64>)>, v1: u64) -> R<(List<u64>, (List<u64>, Option<List<u64>>))> {
    Ok((f1(v0.clone(), v1), (f3(v0.clone(), v1)?, f16(v0.clone())?)))
}

pub fn f1(v0: List<(u64, List<u64>)>, v1: u64) -> List<u64> {
    let m1 = f2(v0.clone(), v1);
    match m1 {
        None => {
            List::<u64>::nil()
        }
        Some(v2) => {
            v2.clone()
        }
    }
}

pub fn f2(v0: List<(u64, List<u64>)>, v1: u64) -> Option<List<u64>> {
    let m2 = v0.clone();
    match m2.uncons() {
        None => {
            None::<List<u64>>
        }
        Some((v2, v3)) => {
            let m6 = {
                let a4 = v1;
                let a5 = {
                    let (h3, _) = v2.clone();
                    h3
                };
                compare(a4, a5)
            };
            match m6 {
                Ordering::Less => {
                    None::<List<u64>>
                }
                Ordering::Equal => {
                    Some({
                        let (_, h7) = v2.clone();
                        h7
                    })
                }
                Ordering::Greater => {
                    f2(v3.clone(), v1)
                }
            }
        }
    }
}

pub fn f3(v0: List<(u64, List<u64>)>, v1: u64) -> R<List<u64>> {
    Ok(f4(v0.clone(), {
        let a9 = {
            let a8 = f13(v0.clone(), List::<u64>::nil());
            length_list(a8)
        };
        let a10 = 1u64;
        nat_add(a9, a10)?
    }, List::cons(v1, List::<u64>::nil()), List::cons(v1, List::<u64>::nil())))
}

pub fn f4(v0: List<(u64, List<u64>)>, v1: u64, v2: List<u64>, v3: List<u64>) -> List<u64> {
    let m11 = v1;
    if m11 == 0 {
        v3.clone()
    } else {
        let v4 = m11 - 1;
        let v5: List<u64> = f5(v0.clone(), v3.clone(), v2.clone(), List::<u64>::nil());
        let m12 = v5.clone();
        match m12.uncons() {
            None => {
                v3.clone()
            }
            Some((_, _)) => {
                f4(v0.clone(), v4, v5.clone(), f11(v3.clone(), v5.clone()))
            }
        }
    }
}

pub fn f5(v0: List<(u64, List<u64>)>, v1: List<u64>, v2: List<u64>, v3: List<u64>) -> List<u64> {
    let m13 = v2.clone();
    match m13.uncons() {
        None => {
            v3.clone()
        }
        Some((v4, v5)) => {
            f5(v0.clone(), v1.clone(), v5.clone(), f6(v1.clone(), f7(v0.clone(), v4), v3.clone()))
        }
    }
}

pub fn f6(v0: List<u64>, v1: List<u64>, v2: List<u64>) -> List<u64> {
    let m14 = v1.clone();
    match m14.uncons() {
        None => {
            v2.clone()
        }
        Some((v3, v4)) => {
            f6(v0.clone(), v4.clone(), if f9(v0.clone(), v3) {
                v2.clone()
            } else {
                if f9(v2.clone(), v3) {
                    v2.clone()
                } else {
                    f10(v2.clone(), v3)
                }
            })
        }
    }
}

pub fn f7(v0: List<(u64, List<u64>)>, v1: u64) -> List<u64> {
    let m15 = f8(v0.clone(), v1);
    match m15 {
        None => {
            List::<u64>::nil()
        }
        Some(v2) => {
            v2.clone()
        }
    }
}

pub fn f8(v0: List<(u64, List<u64>)>, v1: u64) -> Option<List<u64>> {
    let m16 = v0.clone();
    match m16.uncons() {
        None => {
            None::<List<u64>>
        }
        Some((v2, v3)) => {
            let m20 = {
                let a18 = v1;
                let a19 = {
                    let (h17, _) = v2.clone();
                    h17
                };
                compare(a18, a19)
            };
            match m20 {
                Ordering::Less => {
                    None::<List<u64>>
                }
                Ordering::Equal => {
                    Some({
                        let (_, h21) = v2.clone();
                        h21
                    })
                }
                Ordering::Greater => {
                    f8(v3.clone(), v1)
                }
            }
        }
    }
}

pub fn f9(v0: List<u64>, v1: u64) -> bool {
    let m22 = v0.clone();
    match m22.uncons() {
        None => {
            false
        }
        Some((v2, v3)) => {
            let m25 = {
                let a23 = v1;
                let a24 = v2;
                compare(a23, a24)
            };
            match m25 {
                Ordering::Less => {
                    false
                }
                Ordering::Equal => {
                    true
                }
                Ordering::Greater => {
                    f9(v3.clone(), v1)
                }
            }
        }
    }
}

pub fn f10(v0: List<u64>, v1: u64) -> List<u64> {
    let m26 = v0.clone();
    match m26.uncons() {
        None => {
            List::cons(v1, List::<u64>::nil())
        }
        Some((v2, v3)) => {
            let m29 = {
                let a27 = v1;
                let a28 = v2;
                compare(a27, a28)
            };
            match m29 {
                Ordering::Less => {
                    List::cons(v1, v0.clone())
                }
                Ordering::Equal => {
                    v0.clone()
                }
                Ordering::Greater => {
                    List::cons(v2, f10(v3.clone(), v1))
                }
            }
        }
    }
}

pub fn f11(v0: List<u64>, v1: List<u64>) -> List<u64> {
    let m30 = v1.clone();
    match m30.uncons() {
        None => {
            v0.clone()
        }
        Some((v2, v3)) => {
            f11(f12(v0.clone(), v2), v3.clone())
        }
    }
}

pub fn f12(v0: List<u64>, v1: u64) -> List<u64> {
    let m31 = v0.clone();
    match m31.uncons() {
        None => {
            List::cons(v1, List::<u64>::nil())
        }
        Some((v2, v3)) => {
            let m34 = {
                let a32 = v1;
                let a33 = v2;
                compare(a32, a33)
            };
            match m34 {
                Ordering::Less => {
                    List::cons(v1, v0.clone())
                }
                Ordering::Equal => {
                    v0.clone()
                }
                Ordering::Greater => {
                    List::cons(v2, f12(v3.clone(), v1))
                }
            }
        }
    }
}

pub fn f13(v0: List<(u64, List<u64>)>, v1: List<u64>) -> List<u64> {
    let m35 = v0.clone();
    match m35.uncons() {
        None => {
            v1.clone()
        }
        Some((v2, v3)) => {
            f13(v3.clone(), f14({
                let (_, h36) = v2.clone();
                h36
            }, f15(v1.clone(), {
                let (h37, _) = v2.clone();
                h37
            })))
        }
    }
}

pub fn f14(v0: List<u64>, v1: List<u64>) -> List<u64> {
    let m38 = v0.clone();
    match m38.uncons() {
        None => {
            v1.clone()
        }
        Some((v2, v3)) => {
            f14(v3.clone(), f15(v1.clone(), v2))
        }
    }
}

pub fn f15(v0: List<u64>, v1: u64) -> List<u64> {
    let m39 = v0.clone();
    match m39.uncons() {
        None => {
            List::cons(v1, List::<u64>::nil())
        }
        Some((v2, v3)) => {
            let m42 = {
                let a40 = v1;
                let a41 = v2;
                compare(a40, a41)
            };
            match m42 {
                Ordering::Less => {
                    List::cons(v1, v0.clone())
                }
                Ordering::Equal => {
                    v0.clone()
                }
                Ordering::Greater => {
                    List::cons(v2, f15(v3.clone(), v1))
                }
            }
        }
    }
}

pub fn f16(v0: List<(u64, List<u64>)>) -> R<Option<List<u64>>> {
    let v1: List<u64> = f21(v0.clone(), List::<u64>::nil());
    Ok(f17(v0.clone(), {
        let a44 = {
            let a43 = v1.clone();
            length_list(a43)
        };
        let a45 = 1u64;
        nat_add(a44, a45)?
    }, v1.clone(), List::<u64>::nil()))
}

pub fn f17(v0: List<(u64, List<u64>)>, v1: u64, v2: List<u64>, v3: List<u64>) -> Option<List<u64>> {
    let m46 = v1;
    if m46 == 0 {
        let m47 = v2.clone();
        match m47.uncons() {
            None => {
                Some(f20(v3.clone(), List::<u64>::nil()))
            }
            Some((_, _)) => {
                None::<List<u64>>
            }
        }
    } else {
        let v6 = m46 - 1;
        let m48 = f18(v0.clone(), v2.clone(), v2.clone());
        match m48.uncons() {
            None => {
                let m49 = v2.clone();
                match m49.uncons() {
                    None => {
                        Some(f20(v3.clone(), List::<u64>::nil()))
                    }
                    Some((_, _)) => {
                        None::<List<u64>>
                    }
                }
            }
            Some((v9, _)) => {
                f17(v0.clone(), v6, f27(v2.clone(), v9), List::cons(v9, v3.clone()))
            }
        }
    }
}

pub fn f18(v0: List<(u64, List<u64>)>, v1: List<u64>, v2: List<u64>) -> List<u64> {
    let m50 = v2.clone();
    match m50.uncons() {
        None => {
            List::<u64>::nil()
        }
        Some((v3, v4)) => {
            if f19(v0.clone(), v3, v1.clone()) {
                List::cons(v3, f18(v0.clone(), v1.clone(), v4.clone()))
            } else {
                f18(v0.clone(), v1.clone(), v4.clone())
            }
        }
    }
}

pub fn f19(v0: List<(u64, List<u64>)>, v1: u64, v2: List<u64>) -> bool {
    let m51 = v2.clone();
    match m51.uncons() {
        None => {
            true
        }
        Some((v3, v4)) => {
            if f26(f24(v0.clone(), v3), v1) {
                false
            } else {
                f19(v0.clone(), v1, v4.clone())
            }
        }
    }
}

pub fn f20(v0: List<u64>, v1: List<u64>) -> List<u64> {
    let m52 = v0.clone();
    match m52.uncons() {
        None => {
            v1.clone()
        }
        Some((v2, v3)) => {
            f20(v3.clone(), List::cons(v2, v1.clone()))
        }
    }
}

pub fn f21(v0: List<(u64, List<u64>)>, v1: List<u64>) -> List<u64> {
    let m53 = v0.clone();
    match m53.uncons() {
        None => {
            v1.clone()
        }
        Some((v2, v3)) => {
            f21(v3.clone(), f22({
                let (_, h54) = v2.clone();
                h54
            }, f23(v1.clone(), {
                let (h55, _) = v2.clone();
                h55
            })))
        }
    }
}

pub fn f22(v0: List<u64>, v1: List<u64>) -> List<u64> {
    let m56 = v0.clone();
    match m56.uncons() {
        None => {
            v1.clone()
        }
        Some((v2, v3)) => {
            f22(v3.clone(), f23(v1.clone(), v2))
        }
    }
}

pub fn f23(v0: List<u64>, v1: u64) -> List<u64> {
    let m57 = v0.clone();
    match m57.uncons() {
        None => {
            List::cons(v1, List::<u64>::nil())
        }
        Some((v2, v3)) => {
            let m60 = {
                let a58 = v1;
                let a59 = v2;
                compare(a58, a59)
            };
            match m60 {
                Ordering::Less => {
                    List::cons(v1, v0.clone())
                }
                Ordering::Equal => {
                    v0.clone()
                }
                Ordering::Greater => {
                    List::cons(v2, f23(v3.clone(), v1))
                }
            }
        }
    }
}

pub fn f24(v0: List<(u64, List<u64>)>, v1: u64) -> List<u64> {
    let m61 = f25(v0.clone(), v1);
    match m61 {
        None => {
            List::<u64>::nil()
        }
        Some(v2) => {
            v2.clone()
        }
    }
}

pub fn f25(v0: List<(u64, List<u64>)>, v1: u64) -> Option<List<u64>> {
    let m62 = v0.clone();
    match m62.uncons() {
        None => {
            None::<List<u64>>
        }
        Some((v2, v3)) => {
            let m66 = {
                let a64 = v1;
                let a65 = {
                    let (h63, _) = v2.clone();
                    h63
                };
                compare(a64, a65)
            };
            match m66 {
                Ordering::Less => {
                    None::<List<u64>>
                }
                Ordering::Equal => {
                    Some({
                        let (_, h67) = v2.clone();
                        h67
                    })
                }
                Ordering::Greater => {
                    f25(v3.clone(), v1)
                }
            }
        }
    }
}

pub fn f26(v0: List<u64>, v1: u64) -> bool {
    let m68 = v0.clone();
    match m68.uncons() {
        None => {
            false
        }
        Some((v2, v3)) => {
            let m71 = {
                let a69 = v1;
                let a70 = v2;
                compare(a69, a70)
            };
            match m71 {
                Ordering::Less => {
                    false
                }
                Ordering::Equal => {
                    true
                }
                Ordering::Greater => {
                    f26(v3.clone(), v1)
                }
            }
        }
    }
}

pub fn f27(v0: List<u64>, v1: u64) -> List<u64> {
    let m72 = v0.clone();
    match m72.uncons() {
        None => {
            List::<u64>::nil()
        }
        Some((v2, v3)) => {
            let m75 = {
                let a73 = v1;
                let a74 = v2;
                compare(a73, a74)
            };
            match m75 {
                Ordering::Less => {
                    v0.clone()
                }
                Ordering::Equal => {
                    v3.clone()
                }
                Ordering::Greater => {
                    List::cons(v2, f27(v3.clone(), v1))
                }
            }
        }
    }
}

pub fn f28(v0: List<(u64, List<u64>)>) -> bool {
    let m76 = v0.clone();
    match m76.uncons() {
        None => {
            true
        }
        Some((v1, v2)) => {
            if f29({
                let (_, h77) = v1.clone();
                h77
            }) {
                let m78 = v2.clone();
                match m78.uncons() {
                    None => {
                        true
                    }
                    Some((v3, _)) => {
                        let m83 = {
                            let a81 = {
                                let (h79, _) = v1.clone();
                                h79
                            };
                            let a82 = {
                                let (h80, _) = v3.clone();
                                h80
                            };
                            compare(a81, a82)
                        };
                        if match m83 {
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
                            f28(v2.clone())
                        } else {
                            false
                        }
                    }
                }
            } else {
                false
            }
        }
    }
}

pub fn f29(v0: List<u64>) -> bool {
    let m84 = v0.clone();
    match m84.uncons() {
        None => {
            true
        }
        Some((v1, v2)) => {
            let m85 = v2.clone();
            match m85.uncons() {
                None => {
                    true
                }
                Some((v3, _)) => {
                    let m88 = {
                        let a86 = v1;
                        let a87 = v3;
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
                        f29(v2.clone())
                    } else {
                        false
                    }
                }
            }
        }
    }
}

pub fn f30(v0: List<(u64, List<u64>)>, v1: u64) -> R<Option<(List<u64>, (List<u64>, Option<List<u64>>))>> {
    if f28(v0.clone()) {
        Ok(Some(f0(v0.clone(), v1)?))
    } else {
        Ok(None::<(List<u64>, (List<u64>, Option<List<u64>>))>)
    }
}
