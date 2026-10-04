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
    C2(u64),
}

#[derive(Clone)]
pub enum Adt1 {
    C0((u64, u64), List<Adt0>),
}

#[derive(Clone)]
pub enum Adt2 {
    C0(List<Adt1>, List<(u64, u64)>, Option<((u64, u64), Adt1)>, bool),
}

#[derive(Clone)]
pub enum Fn0 {
    F25(u64),
}

impl Fn0 {
    pub fn apply(&self, p0: Adt2) -> R<Option<Adt2>> {
        match self {
            Fn0::F25(k0) => f25(*k0, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F18(Adt1),
}

impl Fn1 {
    pub fn apply(&self, p0: List<Adt1>, p1: u64) -> R<List<Adt1>> {
        match self {
            Fn1::F18(k0) => f18(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F21,
}

impl Fn2 {
    pub fn apply(&self, p0: (List<Adt1>, List<(u64, u64)>), p1: Adt1) -> (List<Adt1>, List<(u64, u64)>) {
        match self {
            Fn2::F21 => f21(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F23(u64),
}

impl Fn3 {
    pub fn apply(&self, p0: List<Adt1>, p1: Adt1) -> List<Adt1> {
        match self {
            Fn3::F23(k0) => f23(*k0, p0, p1),
        }
    }
}

pub fn f0(v0: u64) -> R<Result<(u64, u64), (bool, bool)>> {
    let v1: (Adt2, bool) = f31(Fn0::F25(v0), {
        let a1 = v0;
        let a2 = 8u64;
        nat_mul(a1, a2)?
    }, f24(v0)?)?;
    let m6 = {
        let m5 = {
            let (h4, _) = v1.clone();
            h4
        };
        match m5 {
            Adt2::C0(_, _, h3, _) => {
                h3
            }
        }
    };
    match m6 {
        None => {
            Ok(Err::<(u64, u64), (bool, bool)>({
                let a13 = {
                    let (_, h8) = v1.clone();
                    h8
                };
                let a14 = {
                    let a12 = {
                        let m11 = {
                            let (h10, _) = v1.clone();
                            h10
                        };
                        match m11 {
                            Adt2::C0(_, _, _, h9) => {
                                h9
                            }
                        }
                    };
                    bool_not(a12)
                };
                if bool_and(a13, a14) {
                    (false, true)
                } else {
                    (false, false)
                }
            }))
        }
        Some(v2) => {
            Ok(Ok::<(u64, u64), (bool, bool)>({
                let (h7, _) = v2.clone();
                h7
            }))
        }
    }
}

pub fn f1(v0: (u64, u64)) -> bool {
    let a21 = {
        let a16 = {
            let (h15, _) = v0;
            h15
        };
        let a17 = 4u64;
        nat_lt(a16, a17)
    };
    let a22 = {
        let a19 = {
            let (_, h18) = v0;
            h18
        };
        let a20 = 3u64;
        nat_le(a19, a20)
    };
    bool_and(a21, a22)
}

pub fn f2(v0: (u64, u64)) -> (u64, u64) {
    (4u64, {
        let (_, h23) = v0;
        h23
    })
}

pub fn f3(v0: (u64, u64)) -> Option<(u64, u64)> {
    if f1(v0) {
        Some(f2(v0))
    } else {
        None::<(u64, u64)>
    }
}

pub fn f4(v0: (u64, u64)) -> bool {
    let a30 = {
        let a25 = 0u64;
        let a26 = {
            let (_, h24) = v0;
            h24
        };
        nat_lt(a25, a26)
    };
    let a31 = {
        let a28 = {
            let (h27, _) = v0;
            h27
        };
        let a29 = 4u64;
        nat_le(a28, a29)
    };
    bool_and(a30, a31)
}

pub fn f5(v0: (u64, u64)) -> (u64, u64) {
    ({
        let (h32, _) = v0;
        h32
    }, 0u64)
}

pub fn f6(v0: (u64, u64)) -> Option<(u64, u64)> {
    if f4(v0) {
        Some(f5(v0))
    } else {
        None::<(u64, u64)>
    }
}

pub fn f7(v0: (u64, u64), v1: u64) -> R<bool> {
    let a46 = {
        let a34 = v1;
        let a35 = {
            let (h33, _) = v0;
            h33
        };
        nat_le(a34, a35)
    };
    let a47 = {
        let a44 = {
            let a39 = {
                let a37 = {
                    let (_, h36) = v0;
                    h36
                };
                let a38 = v1;
                nat_add(a37, a38)?
            };
            let a40 = 3u64;
            nat_le(a39, a40)
        };
        let a45 = {
            let a42 = {
                let (h41, _) = v0;
                h41
            };
            let a43 = 4u64;
            nat_le(a42, a43)
        };
        bool_and(a44, a45)
    };
    Ok(bool_and(a46, a47))
}

pub fn f8(v0: (u64, u64), v1: u64) -> R<(u64, u64)> {
    Ok(({
        let a49 = {
            let (h48, _) = v0;
            h48
        };
        let a50 = v1;
        nat_sub(a49, a50)
    }, {
        let a52 = {
            let (_, h51) = v0;
            h51
        };
        let a53 = v1;
        nat_add(a52, a53)?
    }))
}

pub fn f9(_: (u64, u64)) -> List<u64> {
    List::cons(1u64, List::cons(2u64, List::cons(3u64, List::<u64>::nil())))
}

pub fn f10(v0: (u64, u64), v1: u64) -> R<Option<(u64, u64)>> {
    if f7(v0, v1)? {
        Ok(Some(f8(v0, v1)?))
    } else {
        Ok(None::<(u64, u64)>)
    }
}

pub fn f11(v0: (u64, u64), v1: Adt0) -> R<Option<(u64, u64)>> {
    let m54 = v1.clone();
    match m54 {
        Adt0::C0 => {
            Ok(f3(v0))
        }
        Adt0::C1 => {
            Ok(f6(v0))
        }
        Adt0::C2(v2) => {
            f10(v0, v2)
        }
    }
}

pub fn f12(_: u64) -> (u64, u64) {
    (0u64, 0u64)
}

pub fn f13(v0: (u64, u64)) -> Option<(u64, u64)> {
    Some(v0)
}

pub fn f14(v0: u64, v1: (u64, u64)) -> bool {
    let a61 = {
        let a56 = {
            let (_, h55) = v1;
            h55
        };
        let a57 = v0;
        nat_eq(a56, a57)
    };
    let a62 = {
        let a59 = {
            let (h58, _) = v1;
            h58
        };
        let a60 = 4u64;
        nat_le(a59, a60)
    };
    bool_and(a61, a62)
}

pub fn f15(v0: u64, v1: (u64, u64)) -> Option<(u64, u64)> {
    let m63 = f13(v1);
    match m63 {
        None => {
            None::<(u64, u64)>
        }
        Some(v2) => {
            let v3: bool = f14(v0, v2);
            if v3 {
                Some(v2)
            } else {
                None::<(u64, u64)>
            }
        }
    }
}

pub fn f16(v0: Adt1) -> R<List<Adt1>> {
    let m65 = f11(match v0.clone() {
        Adt1::C0(h64, _) => {
            h64
        }
    }, Adt0::C0)?;
    match m65 {
        None => {
            Ok(List::<Adt1>::nil())
        }
        Some(v1) => {
            Ok(List::cons(Adt1::C0(v1, {
                let a67 = match v0.clone() {
                    Adt1::C0(_, h66) => {
                        h66
                    }
                };
                let a68 = List::cons(Adt0::C0, List::<Adt0>::nil());
                append_list(a67, a68)
            }), List::<Adt1>::nil()))
        }
    }
}

pub fn f17(v0: Adt1) -> R<List<Adt1>> {
    let m70 = f11(match v0.clone() {
        Adt1::C0(h69, _) => {
            h69
        }
    }, Adt0::C1)?;
    match m70 {
        None => {
            Ok(List::<Adt1>::nil())
        }
        Some(v1) => {
            Ok(List::cons(Adt1::C0(v1, {
                let a72 = match v0.clone() {
                    Adt1::C0(_, h71) => {
                        h71
                    }
                };
                let a73 = List::cons(Adt0::C1, List::<Adt0>::nil());
                append_list(a72, a73)
            }), List::<Adt1>::nil()))
        }
    }
}

pub fn f18(v0: Adt1, v1: List<Adt1>, v2: u64) -> R<List<Adt1>> {
    let m75 = f11(match v0.clone() {
        Adt1::C0(h74, _) => {
            h74
        }
    }, Adt0::C2(v2))?;
    match m75 {
        None => {
            Ok(v1.clone())
        }
        Some(v3) => {
            let a79 = v1.clone();
            let a80 = List::cons(Adt1::C0(v3, {
                let a77 = match v0.clone() {
                    Adt1::C0(_, h76) => {
                        h76
                    }
                };
                let a78 = List::cons(Adt0::C2(v2), List::<Adt0>::nil());
                append_list(a77, a78)
            }), List::<Adt1>::nil());
            Ok(append_list(a79, a80))
        }
    }
}

pub fn f19(v0: Adt1) -> R<List<Adt1>> {
    f27(Fn1::F18(v0.clone()), List::<Adt1>::nil(), f9(match v0.clone() {
        Adt1::C0(h81, _) => {
            h81
        }
    }))
}

pub fn f20(v0: Adt1) -> R<List<Adt1>> {
    let a84 = f16(v0.clone())?;
    let a85 = {
        let a82 = f17(v0.clone())?;
        let a83 = f19(v0.clone())?;
        append_list(a82, a83)
    };
    Ok(append_list(a84, a85))
}

pub fn f21(v0: (List<Adt1>, List<(u64, u64)>), v1: Adt1) -> (List<Adt1>, List<(u64, u64)>) {
    if f29({
        let (_, h86) = v0.clone();
        h86
    }, match v1.clone() {
        Adt1::C0(h87, _) => {
            h87
        }
    }) {
        v0.clone()
    } else {
        ({
            let a89 = {
                let (h88, _) = v0.clone();
                h88
            };
            let a90 = List::cons(v1.clone(), List::<Adt1>::nil());
            append_list(a89, a90)
        }, f30({
            let (_, h91) = v0.clone();
            h91
        }, match v1.clone() {
            Adt1::C0(h92, _) => {
                h92
            }
        }))
    }
}

pub fn f22(v0: List<(u64, u64)>, v1: List<Adt1>) -> (List<Adt1>, List<(u64, u64)>) {
    f28(Fn2::F21, (List::<Adt1>::nil(), v0.clone()), v1.clone())
}

pub fn f23(v0: u64, v1: List<Adt1>, v2: Adt1) -> List<Adt1> {
    let a94 = {
        let a93 = v1.clone();
        length_list(a93)
    };
    let a95 = v0;
    if nat_lt(a94, a95) {
        let a96 = v1.clone();
        let a97 = List::cons(v2.clone(), List::<Adt1>::nil());
        append_list(a96, a97)
    } else {
        v1.clone()
    }
}

pub fn f24(v0: u64) -> R<Adt2> {
    let v1: u64 = {
        let a98 = v0;
        let a99 = v0;
        nat_add(a98, a99)?
    };
    let v2: List<Adt1> = f26(Fn3::F23(v1), List::<Adt1>::nil(), List::cons(Adt1::C0(f12(v0), List::<Adt0>::nil()), List::<Adt1>::nil()));
    Ok(Adt2::C0(v2.clone(), f30(List::<(u64, u64)>::nil(), f12(v0)), None::<((u64, u64), Adt1)>, {
        let a100 = v1;
        let a101 = 1u64;
        nat_lt(a100, a101)
    }))
}

pub fn f25(v0: u64, v1: Adt2) -> R<Option<Adt2>> {
    let m103 = match v1.clone() {
        Adt2::C0(_, _, h102, _) => {
            h102
        }
    };
    match m103 {
        None => {
            let m105 = match v1.clone() {
                Adt2::C0(h104, _, _, _) => {
                    h104
                }
            };
            match m105.uncons() {
                None => {
                    Ok(None::<Adt2>)
                }
                Some((v3, v4)) => {
                    let m107 = f15(v0, match v3.clone() {
                        Adt1::C0(h106, _) => {
                            h106
                        }
                    });
                    match m107 {
                        None => {
                            let v6: List<Adt1> = f20(v3.clone())?;
                            let v7: (List<Adt1>, List<(u64, u64)>) = f22(match v1.clone() {
                                Adt2::C0(_, h110, _, _) => {
                                    h110
                                }
                            }, v6.clone());
                            let v8: List<Adt1> = {
                                let a112 = v4.clone();
                                let a113 = {
                                    let (h111, _) = v7.clone();
                                    h111
                                };
                                append_list(a112, a113)
                            };
                            let v9: List<Adt1> = f26(Fn3::F23({
                                let a114 = v0;
                                let a115 = v0;
                                nat_add(a114, a115)?
                            }), List::<Adt1>::nil(), v8.clone());
                            Ok(Some(Adt2::C0(v9.clone(), {
                                let (_, h116) = v7.clone();
                                h116
                            }, None::<((u64, u64), Adt1)>, {
                                let a123 = match v1.clone() {
                                    Adt2::C0(_, _, _, h117) => {
                                        h117
                                    }
                                };
                                let a124 = {
                                    let a121 = {
                                        let a118 = v0;
                                        let a119 = v0;
                                        nat_add(a118, a119)?
                                    };
                                    let a122 = {
                                        let a120 = v8.clone();
                                        length_list(a120)
                                    };
                                    nat_lt(a121, a122)
                                };
                                bool_or(a123, a124)
                            })))
                        }
                        Some(v5) => {
                            Ok(Some(Adt2::C0(v4.clone(), match v1.clone() {
                                Adt2::C0(_, h108, _, _) => {
                                    h108
                                }
                            }, Some((v5, v3.clone())), match v1.clone() {
                                Adt2::C0(_, _, _, h109) => {
                                    h109
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

pub fn f26(v0: Fn3, v1: List<Adt1>, v2: List<Adt1>) -> List<Adt1> {
    let m125 = v2.clone();
    match m125.uncons() {
        None => {
            v1.clone()
        }
        Some((v3, v4)) => {
            f26(v0.clone(), {
                let c126: Fn3 = v0.clone();
                c126.apply(v1.clone(), v3.clone())
            }, v4.clone())
        }
    }
}

pub fn f27(v0: Fn1, v1: List<Adt1>, v2: List<u64>) -> R<List<Adt1>> {
    let m127 = v2.clone();
    match m127.uncons() {
        None => {
            Ok(v1.clone())
        }
        Some((v3, v4)) => {
            f27(v0.clone(), {
                let c128: Fn1 = v0.clone();
                c128.apply(v1.clone(), v3)?
            }, v4.clone())
        }
    }
}

pub fn f28(v0: Fn2, v1: (List<Adt1>, List<(u64, u64)>), v2: List<Adt1>) -> (List<Adt1>, List<(u64, u64)>) {
    let m129 = v2.clone();
    match m129.uncons() {
        None => {
            v1.clone()
        }
        Some((v3, v4)) => {
            f28(v0.clone(), {
                let c130: Fn2 = v0.clone();
                c130.apply(v1.clone(), v3.clone())
            }, v4.clone())
        }
    }
}

pub fn f29(v0: List<(u64, u64)>, v1: (u64, u64)) -> bool {
    let m131 = v0.clone();
    match m131.uncons() {
        None => {
            false
        }
        Some((v2, v3)) => {
            let m134 = {
                let a132 = v1;
                let a133 = v2;
                compare(a132, a133)
            };
            match m134 {
                Ordering::Less => {
                    false
                }
                Ordering::Equal => {
                    true
                }
                Ordering::Greater => {
                    f29(v3.clone(), v1)
                }
            }
        }
    }
}

pub fn f30(v0: List<(u64, u64)>, v1: (u64, u64)) -> List<(u64, u64)> {
    let m135 = v0.clone();
    match m135.uncons() {
        None => {
            List::cons(v1, List::<(u64, u64)>::nil())
        }
        Some((v2, v3)) => {
            let m138 = {
                let a136 = v1;
                let a137 = v2;
                compare(a136, a137)
            };
            match m138 {
                Ordering::Less => {
                    List::cons(v1, v0.clone())
                }
                Ordering::Equal => {
                    v0.clone()
                }
                Ordering::Greater => {
                    List::cons(v2, f30(v3.clone(), v1))
                }
            }
        }
    }
}

pub fn f31(v0: Fn0, v1: u64, v2: Adt2) -> R<(Adt2, bool)> {
    let m139 = v1;
    if m139 == 0 {
        Ok((v2.clone(), false))
    } else {
        let v3 = m139 - 1;
        let m141 = {
            let c140: Fn0 = v0.clone();
            c140.apply(v2.clone())?
        };
        match m141 {
            None => {
                Ok((v2.clone(), true))
            }
            Some(v4) => {
                f31(v0.clone(), v3, v4.clone())
            }
        }
    }
}

pub fn run(p0: u64) -> R<Result<(u64, u64), (bool, bool)>> {
    f0(p0)
}
