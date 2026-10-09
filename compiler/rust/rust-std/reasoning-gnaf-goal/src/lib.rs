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

pub fn f0(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<(u64, (u64, (u64, (u64, u64))))> {
    Ok((f1(v0)?, (f1(v1)?, (f1(v2)?, (f1(v3)?, f1(v4)?)))))
}

pub fn f1(v0: u64) -> R<u64> {
    let a51 = {
        let a41 = {
            let a31 = 2u64;
            let a32 = {
                let a29 = {
                    let a13 = {
                        let a4 = {
                            let a1 = v0;
                            let a2 = 1u64;
                            let a3 = 0u64;
                            nat_quot(a1, a2, a3)
                        };
                        let a5 = 2u64;
                        let a6 = 0u64;
                        nat_rem(a4, a5, a6)
                    };
                    let a14 = {
                        let a10 = {
                            let a7 = v0;
                            let a8 = 2u64;
                            let a9 = 0u64;
                            nat_quot(a7, a8, a9)
                        };
                        let a11 = 2u64;
                        let a12 = 0u64;
                        nat_rem(a10, a11, a12)
                    };
                    nat_add(a13, a14)?
                };
                let a30 = {
                    let a27 = {
                        let a18 = {
                            let a15 = v0;
                            let a16 = 4u64;
                            let a17 = 0u64;
                            nat_quot(a15, a16, a17)
                        };
                        let a19 = 2u64;
                        let a20 = 0u64;
                        nat_rem(a18, a19, a20)
                    };
                    let a28 = {
                        let a24 = {
                            let a21 = v0;
                            let a22 = 8u64;
                            let a23 = 0u64;
                            nat_quot(a21, a22, a23)
                        };
                        let a25 = 2u64;
                        let a26 = 0u64;
                        nat_rem(a24, a25, a26)
                    };
                    nat_add(a27, a28)?
                };
                nat_add(a29, a30)?
            };
            nat_le(a31, a32)
        };
        let a42 = {
            let a39 = {
                let a36 = {
                    let a33 = v0;
                    let a34 = 16u64;
                    let a35 = 0u64;
                    nat_quot(a33, a34, a35)
                };
                let a37 = 2u64;
                let a38 = 0u64;
                nat_rem(a36, a37, a38)
            };
            let a40 = 1u64;
            nat_eq(a39, a40)
        };
        bool_and(a41, a42)
    };
    let a52 = {
        let a49 = {
            let a46 = {
                let a43 = v0;
                let a44 = 32u64;
                let a45 = 0u64;
                nat_quot(a43, a44, a45)
            };
            let a47 = 2u64;
            let a48 = 0u64;
            nat_rem(a46, a47, a48)
        };
        let a50 = 1u64;
        nat_eq(a49, a50)
    };
    if bool_and(a51, a52) {
        Ok(3u64)
    } else {
        let a93 = {
            let a83 = 2u64;
            let a84 = {
                let a81 = {
                    let a65 = {
                        let a56 = {
                            let a53 = v0;
                            let a54 = 1u64;
                            let a55 = 0u64;
                            nat_quot(a53, a54, a55)
                        };
                        let a57 = 2u64;
                        let a58 = 0u64;
                        nat_rem(a56, a57, a58)
                    };
                    let a66 = {
                        let a62 = {
                            let a59 = v0;
                            let a60 = 2u64;
                            let a61 = 0u64;
                            nat_quot(a59, a60, a61)
                        };
                        let a63 = 2u64;
                        let a64 = 0u64;
                        nat_rem(a62, a63, a64)
                    };
                    nat_add(a65, a66)?
                };
                let a82 = {
                    let a79 = {
                        let a70 = {
                            let a67 = v0;
                            let a68 = 4u64;
                            let a69 = 0u64;
                            nat_quot(a67, a68, a69)
                        };
                        let a71 = 2u64;
                        let a72 = 0u64;
                        nat_rem(a70, a71, a72)
                    };
                    let a80 = {
                        let a76 = {
                            let a73 = v0;
                            let a74 = 8u64;
                            let a75 = 0u64;
                            nat_quot(a73, a74, a75)
                        };
                        let a77 = 2u64;
                        let a78 = 0u64;
                        nat_rem(a76, a77, a78)
                    };
                    nat_add(a79, a80)?
                };
                nat_add(a81, a82)?
            };
            nat_le(a83, a84)
        };
        let a94 = {
            let a91 = {
                let a88 = {
                    let a85 = v0;
                    let a86 = 16u64;
                    let a87 = 0u64;
                    nat_quot(a85, a86, a87)
                };
                let a89 = 2u64;
                let a90 = 0u64;
                nat_rem(a88, a89, a90)
            };
            let a92 = 1u64;
            nat_eq(a91, a92)
        };
        if bool_and(a93, a94) {
            Ok(2u64)
        } else {
            let a125 = 2u64;
            let a126 = {
                let a123 = {
                    let a107 = {
                        let a98 = {
                            let a95 = v0;
                            let a96 = 1u64;
                            let a97 = 0u64;
                            nat_quot(a95, a96, a97)
                        };
                        let a99 = 2u64;
                        let a100 = 0u64;
                        nat_rem(a98, a99, a100)
                    };
                    let a108 = {
                        let a104 = {
                            let a101 = v0;
                            let a102 = 2u64;
                            let a103 = 0u64;
                            nat_quot(a101, a102, a103)
                        };
                        let a105 = 2u64;
                        let a106 = 0u64;
                        nat_rem(a104, a105, a106)
                    };
                    nat_add(a107, a108)?
                };
                let a124 = {
                    let a121 = {
                        let a112 = {
                            let a109 = v0;
                            let a110 = 4u64;
                            let a111 = 0u64;
                            nat_quot(a109, a110, a111)
                        };
                        let a113 = 2u64;
                        let a114 = 0u64;
                        nat_rem(a112, a113, a114)
                    };
                    let a122 = {
                        let a118 = {
                            let a115 = v0;
                            let a116 = 8u64;
                            let a117 = 0u64;
                            nat_quot(a115, a116, a117)
                        };
                        let a119 = 2u64;
                        let a120 = 0u64;
                        nat_rem(a118, a119, a120)
                    };
                    nat_add(a121, a122)?
                };
                nat_add(a123, a124)?
            };
            if nat_le(a125, a126) {
                Ok(1u64)
            } else {
                Ok(0u64)
            }
        }
    }
}

pub fn run(p0: u64, p1: u64, p2: u64, p3: u64, p4: u64) -> R<(u64, (u64, (u64, (u64, u64))))> {
    f0(p0, p1, p2, p3, p4)
}
