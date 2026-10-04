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

pub fn f0(v0: List<((u64, Str), bool)>, v1: (u64, Str), v2: List<(i8, u64)>, v3: i8, v4: List<(bool, u64)>, v5: bool, v6: List<(i64, u64)>, v7: i64, v8: List<(u64, u64)>, v9: u64) -> (Option<bool>, (Option<u64>, (Option<u64>, (Option<u64>, Option<u64>)))) {
    (f1(v0.clone(), v1.clone()), (f2(v2.clone(), v3), (f3(v4.clone(), v5), (f4(v6.clone(), v7), f5(v8.clone(), v9)))))
}

pub fn f1(v0: List<((u64, Str), bool)>, v1: (u64, Str)) -> Option<bool> {
    let m1 = v0.clone();
    match m1.uncons() {
        None => {
            None::<bool>
        }
        Some((v2, v3)) => {
            let m5 = {
                let a3 = v1.clone();
                let a4 = {
                    let (h2, _) = v2.clone();
                    h2
                };
                compare(a3, a4)
            };
            match m5 {
                Ordering::Less => {
                    None::<bool>
                }
                Ordering::Equal => {
                    Some({
                        let (_, h6) = v2.clone();
                        h6
                    })
                }
                Ordering::Greater => {
                    f1(v3.clone(), v1.clone())
                }
            }
        }
    }
}

pub fn f2(v0: List<(i8, u64)>, v1: i8) -> Option<u64> {
    let m7 = v0.clone();
    match m7.uncons() {
        None => {
            None::<u64>
        }
        Some((v2, v3)) => {
            let m11 = {
                let a9 = v1;
                let a10 = {
                    let (h8, _) = v2;
                    h8
                };
                compare(a9, a10)
            };
            match m11 {
                Ordering::Less => {
                    None::<u64>
                }
                Ordering::Equal => {
                    Some({
                        let (_, h12) = v2;
                        h12
                    })
                }
                Ordering::Greater => {
                    f2(v3.clone(), v1)
                }
            }
        }
    }
}

pub fn f3(v0: List<(bool, u64)>, v1: bool) -> Option<u64> {
    let m13 = v0.clone();
    match m13.uncons() {
        None => {
            None::<u64>
        }
        Some((v2, v3)) => {
            let m17 = {
                let a15 = v1;
                let a16 = {
                    let (h14, _) = v2;
                    h14
                };
                compare(a15, a16)
            };
            match m17 {
                Ordering::Less => {
                    None::<u64>
                }
                Ordering::Equal => {
                    Some({
                        let (_, h18) = v2;
                        h18
                    })
                }
                Ordering::Greater => {
                    f3(v3.clone(), v1)
                }
            }
        }
    }
}

pub fn f4(v0: List<(i64, u64)>, v1: i64) -> Option<u64> {
    let m19 = v0.clone();
    match m19.uncons() {
        None => {
            None::<u64>
        }
        Some((v2, v3)) => {
            let m23 = {
                let a21 = v1;
                let a22 = {
                    let (h20, _) = v2;
                    h20
                };
                compare(a21, a22)
            };
            match m23 {
                Ordering::Less => {
                    None::<u64>
                }
                Ordering::Equal => {
                    Some({
                        let (_, h24) = v2;
                        h24
                    })
                }
                Ordering::Greater => {
                    f4(v3.clone(), v1)
                }
            }
        }
    }
}

pub fn f5(v0: List<(u64, u64)>, v1: u64) -> Option<u64> {
    let m25 = v0.clone();
    match m25.uncons() {
        None => {
            None::<u64>
        }
        Some((v2, v3)) => {
            let m29 = {
                let a27 = v1;
                let a28 = {
                    let (h26, _) = v2;
                    h26
                };
                compare(a27, a28)
            };
            match m29 {
                Ordering::Less => {
                    None::<u64>
                }
                Ordering::Equal => {
                    Some({
                        let (_, h30) = v2;
                        h30
                    })
                }
                Ordering::Greater => {
                    f5(v3.clone(), v1)
                }
            }
        }
    }
}

pub fn f6(v0: List<((u64, Str), bool)>) -> bool {
    let m31 = v0.clone();
    match m31.uncons() {
        None => {
            true
        }
        Some((v1, v2)) => {
            if f7({
                let (_, h32) = v1.clone();
                h32
            }) {
                let m33 = v2.clone();
                match m33.uncons() {
                    None => {
                        true
                    }
                    Some((v3, _)) => {
                        let m38 = {
                            let a36 = {
                                let (h34, _) = v1.clone();
                                h34
                            };
                            let a37 = {
                                let (h35, _) = v3.clone();
                                h35
                            };
                            compare(a36, a37)
                        };
                        if match m38 {
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
                            f6(v2.clone())
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

pub fn f7(_: bool) -> bool {
    true
}

pub fn f8(v0: List<(i8, u64)>) -> bool {
    let m39 = v0.clone();
    match m39.uncons() {
        None => {
            true
        }
        Some((v1, v2)) => {
            if f9({
                let (_, h40) = v1;
                h40
            }) {
                let m41 = v2.clone();
                match m41.uncons() {
                    None => {
                        true
                    }
                    Some((v3, _)) => {
                        let m46 = {
                            let a44 = {
                                let (h42, _) = v1;
                                h42
                            };
                            let a45 = {
                                let (h43, _) = v3;
                                h43
                            };
                            compare(a44, a45)
                        };
                        if match m46 {
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
                            f8(v2.clone())
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

pub fn f9(_: u64) -> bool {
    true
}

pub fn f10(v0: List<(bool, u64)>) -> bool {
    let m47 = v0.clone();
    match m47.uncons() {
        None => {
            true
        }
        Some((v1, v2)) => {
            if f9({
                let (_, h48) = v1;
                h48
            }) {
                let m49 = v2.clone();
                match m49.uncons() {
                    None => {
                        true
                    }
                    Some((v3, _)) => {
                        let m54 = {
                            let a52 = {
                                let (h50, _) = v1;
                                h50
                            };
                            let a53 = {
                                let (h51, _) = v3;
                                h51
                            };
                            compare(a52, a53)
                        };
                        if match m54 {
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
                            f10(v2.clone())
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

pub fn f11(v0: List<(i64, u64)>) -> bool {
    let m55 = v0.clone();
    match m55.uncons() {
        None => {
            true
        }
        Some((v1, v2)) => {
            if f9({
                let (_, h56) = v1;
                h56
            }) {
                let m57 = v2.clone();
                match m57.uncons() {
                    None => {
                        true
                    }
                    Some((v3, _)) => {
                        let m62 = {
                            let a60 = {
                                let (h58, _) = v1;
                                h58
                            };
                            let a61 = {
                                let (h59, _) = v3;
                                h59
                            };
                            compare(a60, a61)
                        };
                        if match m62 {
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
                            f11(v2.clone())
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

pub fn f12(v0: List<(u64, u64)>) -> bool {
    let m63 = v0.clone();
    match m63.uncons() {
        None => {
            true
        }
        Some((v1, v2)) => {
            if f9({
                let (_, h64) = v1;
                h64
            }) {
                let m65 = v2.clone();
                match m65.uncons() {
                    None => {
                        true
                    }
                    Some((v3, _)) => {
                        let m70 = {
                            let a68 = {
                                let (h66, _) = v1;
                                h66
                            };
                            let a69 = {
                                let (h67, _) = v3;
                                h67
                            };
                            compare(a68, a69)
                        };
                        if match m70 {
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
                            f12(v2.clone())
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

pub fn f13(v0: List<((u64, Str), bool)>, v1: (u64, Str), v2: List<(i8, u64)>, v3: i8, v4: List<(bool, u64)>, v5: bool, v6: List<(i64, u64)>, v7: i64, v8: List<(u64, u64)>, v9: u64) -> Option<(Option<bool>, (Option<u64>, (Option<u64>, (Option<u64>, Option<u64>))))> {
    if f6(v0.clone()) {
        if f8(v2.clone()) {
            if f10(v4.clone()) {
                if f11(v6.clone()) {
                    if f12(v8.clone()) {
                        Some(f0(v0.clone(), v1.clone(), v2.clone(), v3, v4.clone(), v5, v6.clone(), v7, v8.clone(), v9))
                    } else {
                        None::<(Option<bool>, (Option<u64>, (Option<u64>, (Option<u64>, Option<u64>))))>
                    }
                } else {
                    None::<(Option<bool>, (Option<u64>, (Option<u64>, (Option<u64>, Option<u64>))))>
                }
            } else {
                None::<(Option<bool>, (Option<u64>, (Option<u64>, (Option<u64>, Option<u64>))))>
            }
        } else {
            None::<(Option<bool>, (Option<u64>, (Option<u64>, (Option<u64>, Option<u64>))))>
        }
    } else {
        None::<(Option<bool>, (Option<u64>, (Option<u64>, (Option<u64>, Option<u64>))))>
    }
}
