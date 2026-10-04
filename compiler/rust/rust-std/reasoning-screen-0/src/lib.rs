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
    C0(List<Adt1>, Option<(u64, Adt1)>, bool, Adt3),
}

#[derive(Clone)]
pub enum Adt3 {
    C0(u64, u64, u64, u64, u64, u64),
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

pub fn f0(v0: u64) -> R<(Result<(u64, List<Adt0>), (bool, bool)>, (u64, (u64, (u64, (u64, (u64, u64))))))> {
    let v1: (Adt2, bool) = f19(Fn0::F15(v0), 8u64, f14(v0))?;
    Ok(({
        let m4 = {
            let m3 = {
                let (h2, _) = v1.clone();
                h2
            };
            match m3 {
                Adt2::C0(_, h1, _, _) => {
                    h1
                }
            }
        };
        match m4 {
            None => {
                Err::<(u64, List<Adt0>), (bool, bool)>({
                    let a14 = {
                        let (_, h9) = v1.clone();
                        h9
                    };
                    let a15 = {
                        let a13 = {
                            let m12 = {
                                let (h11, _) = v1.clone();
                                h11
                            };
                            match m12 {
                                Adt2::C0(_, _, h10, _) => {
                                    h10
                                }
                            }
                        };
                        bool_not(a13)
                    };
                    if bool_and(a14, a15) {
                        (false, true)
                    } else {
                        (false, false)
                    }
                })
            }
            Some(v2) => {
                Ok::<(u64, List<Adt0>), (bool, bool)>(({
                    let (h5, _) = v2.clone();
                    h5
                }, {
                    let m8 = {
                        let (_, h7) = v2.clone();
                        h7
                    };
                    match m8 {
                        Adt1::C0(_, h6) => {
                            h6
                        }
                    }
                }))
            }
        }
    }, ({
        let m20 = {
            let m19 = {
                let (h18, _) = v1.clone();
                h18
            };
            match m19 {
                Adt2::C0(_, _, _, h17) => {
                    h17
                }
            }
        };
        match m20 {
            Adt3::C0(h16, _, _, _, _, _) => {
                h16
            }
        }
    }, ({
        let m25 = {
            let m24 = {
                let (h23, _) = v1.clone();
                h23
            };
            match m24 {
                Adt2::C0(_, _, _, h22) => {
                    h22
                }
            }
        };
        match m25 {
            Adt3::C0(_, h21, _, _, _, _) => {
                h21
            }
        }
    }, ({
        let m30 = {
            let m29 = {
                let (h28, _) = v1.clone();
                h28
            };
            match m29 {
                Adt2::C0(_, _, _, h27) => {
                    h27
                }
            }
        };
        match m30 {
            Adt3::C0(_, _, h26, _, _, _) => {
                h26
            }
        }
    }, ({
        let m35 = {
            let m34 = {
                let (h33, _) = v1.clone();
                h33
            };
            match m34 {
                Adt2::C0(_, _, _, h32) => {
                    h32
                }
            }
        };
        match m35 {
            Adt3::C0(_, _, _, h31, _, _) => {
                h31
            }
        }
    }, ({
        let m40 = {
            let m39 = {
                let (h38, _) = v1.clone();
                h38
            };
            match m39 {
                Adt2::C0(_, _, _, h37) => {
                    h37
                }
            }
        };
        match m40 {
            Adt3::C0(_, _, _, _, h36, _) => {
                h36
            }
        }
    }, {
        let m45 = {
            let m44 = {
                let (h43, _) = v1.clone();
                h43
            };
            match m44 {
                Adt2::C0(_, _, _, h42) => {
                    h42
                }
            }
        };
        match m45 {
            Adt3::C0(_, _, _, _, _, h41) => {
                h41
            }
        }
    })))))))
}

pub fn f1(v0: (u64, u64), v1: u64) -> bool {
    let a51 = {
        let a47 = {
            let (_, h46) = v0;
            h46
        };
        let a48 = 0u64;
        nat_eq(a47, a48)
    };
    let a52 = {
        let a49 = 0u64;
        let a50 = v1;
        nat_lt(a49, a50)
    };
    bool_and(a51, a52)
}

pub fn f2(v0: (u64, u64), v1: u64) -> (u64, u64) {
    ({
        let (h53, _) = v0;
        h53
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
        let a54 = v0;
        let a55 = 10u64;
        nat_sub(a54, a55)
    }, List::cons({
        let a56 = v0;
        let a57 = 20u64;
        nat_sub(a56, a57)
    }, List::cons({
        let a58 = v0;
        let a59 = 30u64;
        nat_sub(a58, a59)
    }, List::cons({
        let a60 = v0;
        let a61 = 50u64;
        nat_sub(a60, a61)
    }, List::<u64>::nil()))));
    let a63 = {
        let a62 = v1.clone();
        length_list(a62)
    };
    let a64 = 4u64;
    if nat_le(a63, a64) {
        v1.clone()
    } else {
        List::<u64>::nil()
    }
}

pub fn f5(v0: (u64, u64)) -> List<u64> {
    f4({
        let (h65, _) = v0;
        h65
    })
}

pub fn f6(v0: (u64, u64), v1: Adt0) -> Option<(u64, u64)> {
    let m66 = v1.clone();
    match m66 {
        Adt0::C0(v2) => {
            f3(v0, v2)
        }
    }
}

pub fn f7(v0: u64) -> (u64, u64) {
    (v0, 0u64)
}

pub fn f8(v0: (u64, u64)) -> Option<u64> {
    let a68 = 0u64;
    let a69 = {
        let (_, h67) = v0;
        h67
    };
    if nat_lt(a68, a69) {
        Some({
            let (_, h70) = v0;
            h70
        })
    } else {
        None::<u64>
    }
}

pub fn f9(v0: u64, v1: u64) -> R<bool> {
    let a73 = {
        let a71 = v1;
        let a72 = v1;
        nat_add(a71, a72)?
    };
    let a74 = v0;
    Ok(nat_le(a73, a74))
}

pub fn f10(v0: u64, v1: (u64, u64)) -> R<Option<u64>> {
    let m75 = f8(v1);
    match m75 {
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
    let m77 = f6(match v0.clone() {
        Adt1::C0(h76, _) => {
            h76
        }
    }, Adt0::C0(v2));
    match m77 {
        None => {
            v1.clone()
        }
        Some(v3) => {
            let a81 = v1.clone();
            let a82 = List::cons(Adt1::C0(v3, {
                let a79 = match v0.clone() {
                    Adt1::C0(_, h78) => {
                        h78
                    }
                };
                let a80 = List::cons(Adt0::C0(v2), List::<Adt0>::nil());
                append_list(a79, a80)
            }), List::<Adt1>::nil());
            append_list(a81, a82)
        }
    }
}

pub fn f12(v0: Adt1) -> List<Adt1> {
    f18(Fn1::F11(v0.clone()), List::<Adt1>::nil(), f5(match v0.clone() {
        Adt1::C0(h83, _) => {
            h83
        }
    }))
}

pub fn f13(v0: u64, v1: List<Adt1>, v2: Adt1) -> List<Adt1> {
    let a85 = {
        let a84 = v1.clone();
        length_list(a84)
    };
    let a86 = v0;
    if nat_lt(a85, a86) {
        let a87 = v1.clone();
        let a88 = List::cons(v2.clone(), List::<Adt1>::nil());
        append_list(a87, a88)
    } else {
        v1.clone()
    }
}

pub fn f14(v0: u64) -> Adt2 {
    let v1: List<Adt1> = f17(Fn2::F13(3u64), List::<Adt1>::nil(), List::cons(Adt1::C0(f7(v0), List::<Adt0>::nil()), List::<Adt1>::nil()));
    Adt2::C0(v1.clone(), None::<(u64, Adt1)>, {
        let a89 = 3u64;
        let a90 = 1u64;
        nat_lt(a89, a90)
    }, Adt3::C0(0u64, 0u64, 0u64, 0u64, 0u64, {
        let a91 = v1.clone();
        length_list(a91)
    }))
}

pub fn f15(v0: u64, v1: Adt2) -> R<Option<Adt2>> {
    let m93 = match v1.clone() {
        Adt2::C0(_, h92, _, _) => {
            h92
        }
    };
    match m93 {
        None => {
            let m95 = match v1.clone() {
                Adt2::C0(h94, _, _, _) => {
                    h94
                }
            };
            match m95.uncons() {
                None => {
                    Ok(None::<Adt2>)
                }
                Some((v3, v4)) => {
                    let m97 = f10(v0, match v3.clone() {
                        Adt1::C0(h96, _) => {
                            h96
                        }
                    })?;
                    match m97 {
                        None => {
                            let v6: List<Adt1> = f12(v3.clone());
                            let v7: List<Adt1> = {
                                let a121 = v6.clone();
                                let a122 = v4.clone();
                                append_list(a121, a122)
                            };
                            let v8: List<Adt1> = f17(Fn2::F13(3u64), List::<Adt1>::nil(), v7.clone());
                            Ok(Some(Adt2::C0(v8.clone(), None::<(u64, Adt1)>, {
                                let a127 = match v1.clone() {
                                    Adt2::C0(_, _, h123, _) => {
                                        h123
                                    }
                                };
                                let a128 = {
                                    let a125 = 3u64;
                                    let a126 = {
                                        let a124 = v7.clone();
                                        length_list(a124)
                                    };
                                    nat_lt(a125, a126)
                                };
                                bool_or(a127, a128)
                            }, Adt3::C0({
                                let a132 = {
                                    let m131 = match v1.clone() {
                                        Adt2::C0(_, _, _, h130) => {
                                            h130
                                        }
                                    };
                                    match m131 {
                                        Adt3::C0(h129, _, _, _, _, _) => {
                                            h129
                                        }
                                    }
                                };
                                let a133 = 1u64;
                                nat_add(a132, a133)?
                            }, {
                                let a138 = {
                                    let m136 = match v1.clone() {
                                        Adt2::C0(_, _, _, h135) => {
                                            h135
                                        }
                                    };
                                    match m136 {
                                        Adt3::C0(_, h134, _, _, _, _) => {
                                            h134
                                        }
                                    }
                                };
                                let a139 = f16(match v3.clone() {
                                    Adt1::C0(h137, _) => {
                                        h137
                                    }
                                });
                                nat_add(a138, a139)?
                            }, {
                                let a144 = {
                                    let m142 = match v1.clone() {
                                        Adt2::C0(_, _, _, h141) => {
                                            h141
                                        }
                                    };
                                    match m142 {
                                        Adt3::C0(_, _, h140, _, _, _) => {
                                            h140
                                        }
                                    }
                                };
                                let a145 = {
                                    let a143 = v6.clone();
                                    length_list(a143)
                                };
                                nat_add(a144, a145)?
                            }, {
                                let a149 = {
                                    let m148 = match v1.clone() {
                                        Adt2::C0(_, _, _, h147) => {
                                            h147
                                        }
                                    };
                                    match m148 {
                                        Adt3::C0(_, _, _, h146, _, _) => {
                                            h146
                                        }
                                    }
                                };
                                let a150 = 1u64;
                                nat_add(a149, a150)?
                            }, {
                                let a154 = {
                                    let m153 = match v1.clone() {
                                        Adt2::C0(_, _, _, h152) => {
                                            h152
                                        }
                                    };
                                    match m153 {
                                        Adt3::C0(_, _, _, _, h151, _) => {
                                            h151
                                        }
                                    }
                                };
                                let a155 = 1u64;
                                nat_add(a154, a155)?
                            }, {
                                let a160 = {
                                    let m158 = match v1.clone() {
                                        Adt2::C0(_, _, _, h157) => {
                                            h157
                                        }
                                    };
                                    match m158 {
                                        Adt3::C0(_, _, _, _, _, h156) => {
                                            h156
                                        }
                                    }
                                };
                                let a161 = {
                                    let a159 = v8.clone();
                                    length_list(a159)
                                };
                                if nat_lt(a160, a161) {
                                    let a162 = v8.clone();
                                    length_list(a162)
                                } else {
                                    let m165 = match v1.clone() {
                                        Adt2::C0(_, _, _, h164) => {
                                            h164
                                        }
                                    };
                                    match m165 {
                                        Adt3::C0(_, _, _, _, _, h163) => {
                                            h163
                                        }
                                    }
                                }
                            }))))
                        }
                        Some(v5) => {
                            Ok(Some(Adt2::C0(v4.clone(), Some((v5, v3.clone())), match v1.clone() {
                                Adt2::C0(_, _, h98, _) => {
                                    h98
                                }
                            }, Adt3::C0({
                                let a102 = {
                                    let m101 = match v1.clone() {
                                        Adt2::C0(_, _, _, h100) => {
                                            h100
                                        }
                                    };
                                    match m101 {
                                        Adt3::C0(h99, _, _, _, _, _) => {
                                            h99
                                        }
                                    }
                                };
                                let a103 = 1u64;
                                nat_add(a102, a103)?
                            }, {
                                let m106 = match v1.clone() {
                                    Adt2::C0(_, _, _, h105) => {
                                        h105
                                    }
                                };
                                match m106 {
                                    Adt3::C0(_, h104, _, _, _, _) => {
                                        h104
                                    }
                                }
                            }, {
                                let m109 = match v1.clone() {
                                    Adt2::C0(_, _, _, h108) => {
                                        h108
                                    }
                                };
                                match m109 {
                                    Adt3::C0(_, _, h107, _, _, _) => {
                                        h107
                                    }
                                }
                            }, {
                                let m112 = match v1.clone() {
                                    Adt2::C0(_, _, _, h111) => {
                                        h111
                                    }
                                };
                                match m112 {
                                    Adt3::C0(_, _, _, h110, _, _) => {
                                        h110
                                    }
                                }
                            }, {
                                let a116 = {
                                    let m115 = match v1.clone() {
                                        Adt2::C0(_, _, _, h114) => {
                                            h114
                                        }
                                    };
                                    match m115 {
                                        Adt3::C0(_, _, _, _, h113, _) => {
                                            h113
                                        }
                                    }
                                };
                                let a117 = 1u64;
                                nat_add(a116, a117)?
                            }, {
                                let m120 = match v1.clone() {
                                    Adt2::C0(_, _, _, h119) => {
                                        h119
                                    }
                                };
                                match m120 {
                                    Adt3::C0(_, _, _, _, _, h118) => {
                                        h118
                                    }
                                }
                            }))))
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

pub fn f16(v0: (u64, u64)) -> u64 {
    let a166 = f5(v0);
    length_list(a166)
}

pub fn f17(v0: Fn2, v1: List<Adt1>, v2: List<Adt1>) -> List<Adt1> {
    let m167 = v2.clone();
    match m167.uncons() {
        None => {
            v1.clone()
        }
        Some((v3, v4)) => {
            f17(v0.clone(), {
                let c168: Fn2 = v0.clone();
                c168.apply(v1.clone(), v3.clone())
            }, v4.clone())
        }
    }
}

pub fn f18(v0: Fn1, v1: List<Adt1>, v2: List<u64>) -> List<Adt1> {
    let m169 = v2.clone();
    match m169.uncons() {
        None => {
            v1.clone()
        }
        Some((v3, v4)) => {
            f18(v0.clone(), {
                let c170: Fn1 = v0.clone();
                c170.apply(v1.clone(), v3)
            }, v4.clone())
        }
    }
}

pub fn f19(v0: Fn0, v1: u64, v2: Adt2) -> R<(Adt2, bool)> {
    let m171 = v1;
    if m171 == 0 {
        Ok((v2.clone(), false))
    } else {
        let v3 = m171 - 1;
        let m173 = {
            let c172: Fn0 = v0.clone();
            c172.apply(v2.clone())?
        };
        match m173 {
            None => {
                Ok((v2.clone(), true))
            }
            Some(v4) => {
                f19(v0.clone(), v3, v4.clone())
            }
        }
    }
}

pub fn run(p0: u64) -> R<(Result<(u64, List<Adt0>), (bool, bool)>, (u64, (u64, (u64, (u64, (u64, u64))))))> {
    f0(p0)
}
