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
    C0(List<Adt2>, List<(u64, u64)>, Option<((u64, u64), Adt2)>, bool, Adt3),
}

#[derive(Clone)]
pub enum Adt2 {
    C0((u64, u64), List<Adt0>),
}

#[derive(Clone)]
pub enum Adt3 {
    C0(u64, u64, u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Fn0 {
    F6(u64),
}

impl Fn0 {
    pub fn apply(&self, p0: Adt1) -> R<Option<Adt1>> {
        match self {
            Fn0::F6(k0) => f6(*k0, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F10(u64),
    F16(u64),
}

impl Fn1 {
    pub fn apply(&self, p0: List<Adt2>, p1: Adt2) -> R<List<Adt2>> {
        match self {
            Fn1::F10(k0) => f10(*k0, p0, p1),
            Fn1::F16(k0) => f16(*k0, p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F24,
}

impl Fn2 {
    pub fn apply(&self, p0: (List<Adt2>, List<(u64, u64)>), p1: Adt2) -> (List<Adt2>, List<(u64, u64)>) {
        match self {
            Fn2::F24 => f24(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F28(Adt2),
}

impl Fn3 {
    pub fn apply(&self, p0: List<Adt2>, p1: u64) -> R<List<Adt2>> {
        match self {
            Fn3::F28(k0) => f28(k0.clone(), p0, p1),
        }
    }
}

pub fn f0(v0: u64) -> R<u64> {
    let m1 = f1(v0)?;
    match m1 {
        Ok(v1) => {
            let (h2, _) = v1;
            Ok(h2)
        }
        Err(_) => {
            Ok(9u64)
        }
    }
}

pub fn f1(v0: u64) -> R<Result<(u64, u64), (bool, bool)>> {
    let m3 = f2(v0)?;
    match m3 {
        Ok(v1) => {
            Ok(Ok::<(u64, u64), (bool, bool)>({
                let (h4, _) = v1.clone();
                h4
            }))
        }
        Err(v2) => {
            Ok(Err::<(u64, u64), (bool, bool)>(v2))
        }
    }
}

pub fn f2(v0: u64) -> R<Result<((u64, u64), List<Adt0>), (bool, bool)>> {
    let v1: (Adt1, bool) = f3(v0)?;
    let m8 = {
        let m7 = {
            let (h6, _) = v1.clone();
            h6
        };
        match m7 {
            Adt1::C0(_, _, h5, _, _) => {
                h5
            }
        }
    };
    match m8 {
        None => {
            Ok(Err::<((u64, u64), List<Adt0>), (bool, bool)>(f4({
                let (_, h13) = v1.clone();
                h13
            }, {
                let m16 = {
                    let (h15, _) = v1.clone();
                    h15
                };
                match m16 {
                    Adt1::C0(_, _, _, h14, _) => {
                        h14
                    }
                }
            })))
        }
        Some(v2) => {
            Ok(Ok::<((u64, u64), List<Adt0>), (bool, bool)>(({
                let (h9, _) = v2.clone();
                h9
            }, {
                let m12 = {
                    let (_, h11) = v2.clone();
                    h11
                };
                match m12 {
                    Adt2::C0(_, h10) => {
                        h10
                    }
                }
            })))
        }
    }
}

pub fn f3(v0: u64) -> R<(Adt1, bool)> {
    f5(Fn0::F6(v0), {
        let a17 = v0;
        let a18 = 8u64;
        nat_mul(a17, a18)?
    }, f7(v0)?)
}

pub fn f4(v0: bool, v1: bool) -> (bool, bool) {
    if if v0 {
        let a19 = v1;
        bool_not(a19)
    } else {
        false
    } {
        (false, true)
    } else {
        (false, false)
    }
}

pub fn f5(v0: Fn0, v1: u64, v2: Adt1) -> R<(Adt1, bool)> {
    let m20 = v1;
    if m20 == 0 {
        Ok((v2.clone(), false))
    } else {
        let v3 = m20 - 1;
        let m22 = {
            let c21: Fn0 = v0.clone();
            c21.apply(v2.clone())?
        };
        match m22 {
            None => {
                Ok((v2.clone(), true))
            }
            Some(v4) => {
                f5(v0.clone(), v3, v4.clone())
            }
        }
    }
}

pub fn f6(v0: u64, v1: Adt1) -> R<Option<Adt1>> {
    f8(v0, v1.clone())
}

pub fn f7(v0: u64) -> R<Adt1> {
    let v1: List<Adt2> = f9(Fn1::F10(v0), List::<Adt2>::nil(), List::cons(Adt2::C0(f11(v0), List::<Adt0>::nil()), List::<Adt2>::nil()))?;
    Ok(Adt1::C0(v1.clone(), f12(List::<(u64, u64)>::nil(), f11(v0)), None::<((u64, u64), Adt2)>, {
        let a25 = {
            let a23 = v0;
            let a24 = v0;
            nat_add(a23, a24)?
        };
        let a26 = 1u64;
        nat_lt(a25, a26)
    }, Adt3::C0(0u64, 0u64, 0u64, 0u64, 0u64, {
        let a27 = v1.clone();
        length_list(a27)
    })))
}

pub fn f8(v0: u64, v1: Adt1) -> R<Option<Adt1>> {
    let m29 = match v1.clone() {
        Adt1::C0(_, _, h28, _, _) => {
            h28
        }
    };
    match m29 {
        None => {
            let m31 = match v1.clone() {
                Adt1::C0(h30, _, _, _, _) => {
                    h30
                }
            };
            match m31.uncons() {
                None => {
                    Ok(None::<Adt1>)
                }
                Some((v3, v4)) => {
                    let m33 = f13(v0, match v3.clone() {
                        Adt2::C0(h32, _) => {
                            h32
                        }
                    });
                    match m33 {
                        None => {
                            let v6: List<Adt2> = f14(v3.clone())?;
                            let v7: (List<Adt2>, List<(u64, u64)>) = f15(match v1.clone() {
                                Adt1::C0(_, h58, _, _, _) => {
                                    h58
                                }
                            }, v6.clone());
                            let v8: List<Adt2> = {
                                let a60 = v4.clone();
                                let a61 = {
                                    let (h59, _) = v7.clone();
                                    h59
                                };
                                append_list(a60, a61)
                            };
                            let v9: List<Adt2> = f9(Fn1::F16(v0), List::<Adt2>::nil(), v8.clone())?;
                            Ok(Some(Adt1::C0(v9.clone(), {
                                let (_, h62) = v7.clone();
                                h62
                            }, None::<((u64, u64), Adt2)>, if match v1.clone() {
                                Adt1::C0(_, _, _, h63, _) => {
                                    h63
                                }
                            } {
                                true
                            } else {
                                let a67 = {
                                    let a64 = v0;
                                    let a65 = v0;
                                    nat_add(a64, a65)?
                                };
                                let a68 = {
                                    let a66 = v8.clone();
                                    length_list(a66)
                                };
                                nat_lt(a67, a68)
                            }, Adt3::C0({
                                let a72 = {
                                    let m71 = match v1.clone() {
                                        Adt1::C0(_, _, _, _, h70) => {
                                            h70
                                        }
                                    };
                                    match m71 {
                                        Adt3::C0(h69, _, _, _, _, _) => {
                                            h69
                                        }
                                    }
                                };
                                let a73 = 1u64;
                                nat_add(a72, a73)?
                            }, {
                                let a78 = {
                                    let m76 = match v1.clone() {
                                        Adt1::C0(_, _, _, _, h75) => {
                                            h75
                                        }
                                    };
                                    match m76 {
                                        Adt3::C0(_, h74, _, _, _, _) => {
                                            h74
                                        }
                                    }
                                };
                                let a79 = f17(match v3.clone() {
                                    Adt2::C0(h77, _) => {
                                        h77
                                    }
                                })?;
                                nat_add(a78, a79)?
                            }, {
                                let a84 = {
                                    let m82 = match v1.clone() {
                                        Adt1::C0(_, _, _, _, h81) => {
                                            h81
                                        }
                                    };
                                    match m82 {
                                        Adt3::C0(_, _, h80, _, _, _) => {
                                            h80
                                        }
                                    }
                                };
                                let a85 = {
                                    let a83 = v6.clone();
                                    length_list(a83)
                                };
                                nat_add(a84, a85)?
                            }, {
                                let a89 = {
                                    let m88 = match v1.clone() {
                                        Adt1::C0(_, _, _, _, h87) => {
                                            h87
                                        }
                                    };
                                    match m88 {
                                        Adt3::C0(_, _, _, h86, _, _) => {
                                            h86
                                        }
                                    }
                                };
                                let a90 = 1u64;
                                nat_add(a89, a90)?
                            }, {
                                let a94 = {
                                    let m93 = match v1.clone() {
                                        Adt1::C0(_, _, _, _, h92) => {
                                            h92
                                        }
                                    };
                                    match m93 {
                                        Adt3::C0(_, _, _, _, h91, _) => {
                                            h91
                                        }
                                    }
                                };
                                let a95 = 1u64;
                                nat_add(a94, a95)?
                            }, {
                                let a100 = {
                                    let m98 = match v1.clone() {
                                        Adt1::C0(_, _, _, _, h97) => {
                                            h97
                                        }
                                    };
                                    match m98 {
                                        Adt3::C0(_, _, _, _, _, h96) => {
                                            h96
                                        }
                                    }
                                };
                                let a101 = {
                                    let a99 = v9.clone();
                                    length_list(a99)
                                };
                                if nat_lt(a100, a101) {
                                    let a102 = v9.clone();
                                    length_list(a102)
                                } else {
                                    let m105 = match v1.clone() {
                                        Adt1::C0(_, _, _, _, h104) => {
                                            h104
                                        }
                                    };
                                    match m105 {
                                        Adt3::C0(_, _, _, _, _, h103) => {
                                            h103
                                        }
                                    }
                                }
                            }))))
                        }
                        Some(v5) => {
                            Ok(Some(Adt1::C0(v4.clone(), match v1.clone() {
                                Adt1::C0(_, h34, _, _, _) => {
                                    h34
                                }
                            }, Some((v5, v3.clone())), match v1.clone() {
                                Adt1::C0(_, _, _, h35, _) => {
                                    h35
                                }
                            }, Adt3::C0({
                                let a39 = {
                                    let m38 = match v1.clone() {
                                        Adt1::C0(_, _, _, _, h37) => {
                                            h37
                                        }
                                    };
                                    match m38 {
                                        Adt3::C0(h36, _, _, _, _, _) => {
                                            h36
                                        }
                                    }
                                };
                                let a40 = 1u64;
                                nat_add(a39, a40)?
                            }, {
                                let m43 = match v1.clone() {
                                    Adt1::C0(_, _, _, _, h42) => {
                                        h42
                                    }
                                };
                                match m43 {
                                    Adt3::C0(_, h41, _, _, _, _) => {
                                        h41
                                    }
                                }
                            }, {
                                let m46 = match v1.clone() {
                                    Adt1::C0(_, _, _, _, h45) => {
                                        h45
                                    }
                                };
                                match m46 {
                                    Adt3::C0(_, _, h44, _, _, _) => {
                                        h44
                                    }
                                }
                            }, {
                                let m49 = match v1.clone() {
                                    Adt1::C0(_, _, _, _, h48) => {
                                        h48
                                    }
                                };
                                match m49 {
                                    Adt3::C0(_, _, _, h47, _, _) => {
                                        h47
                                    }
                                }
                            }, {
                                let a53 = {
                                    let m52 = match v1.clone() {
                                        Adt1::C0(_, _, _, _, h51) => {
                                            h51
                                        }
                                    };
                                    match m52 {
                                        Adt3::C0(_, _, _, _, h50, _) => {
                                            h50
                                        }
                                    }
                                };
                                let a54 = 1u64;
                                nat_add(a53, a54)?
                            }, {
                                let m57 = match v1.clone() {
                                    Adt1::C0(_, _, _, _, h56) => {
                                        h56
                                    }
                                };
                                match m57 {
                                    Adt3::C0(_, _, _, _, _, h55) => {
                                        h55
                                    }
                                }
                            }))))
                        }
                    }
                }
            }
        }
        Some(_) => {
            Ok(None::<Adt1>)
        }
    }
}

pub fn f9(v0: Fn1, v1: List<Adt2>, v2: List<Adt2>) -> R<List<Adt2>> {
    let m106 = v2.clone();
    match m106.uncons() {
        None => {
            Ok(v1.clone())
        }
        Some((v3, v4)) => {
            f9(v0.clone(), {
                let c107: Fn1 = v0.clone();
                c107.apply(v1.clone(), v3.clone())?
            }, v4.clone())
        }
    }
}

pub fn f10(v0: u64, v1: List<Adt2>, v2: Adt2) -> R<List<Adt2>> {
    let a111 = {
        let a108 = v1.clone();
        length_list(a108)
    };
    let a112 = {
        let a109 = v0;
        let a110 = v0;
        nat_add(a109, a110)?
    };
    if nat_lt(a111, a112) {
        let a113 = v1.clone();
        let a114 = List::cons(v2.clone(), List::<Adt2>::nil());
        Ok(append_list(a113, a114))
    } else {
        Ok(v1.clone())
    }
}

pub fn f11(_: u64) -> (u64, u64) {
    (0u64, 0u64)
}

pub fn f12(v0: List<(u64, u64)>, v1: (u64, u64)) -> List<(u64, u64)> {
    let m115 = v0.clone();
    match m115.uncons() {
        None => {
            List::cons(v1, List::<(u64, u64)>::nil())
        }
        Some((v2, v3)) => {
            let m118 = {
                let a116 = v1;
                let a117 = v2;
                compare(a116, a117)
            };
            match m118 {
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

pub fn f13(v0: u64, v1: (u64, u64)) -> Option<(u64, u64)> {
    let m119 = f18(v1);
    match m119 {
        None => {
            None::<(u64, u64)>
        }
        Some(v2) => {
            if f19(v0, v2) {
                Some(v2)
            } else {
                None::<(u64, u64)>
            }
        }
    }
}

pub fn f14(v0: Adt2) -> R<List<Adt2>> {
    let a122 = f20(v0.clone())?;
    let a123 = {
        let a120 = f21(v0.clone())?;
        let a121 = f22(v0.clone())?;
        append_list(a120, a121)
    };
    Ok(append_list(a122, a123))
}

pub fn f15(v0: List<(u64, u64)>, v1: List<Adt2>) -> (List<Adt2>, List<(u64, u64)>) {
    f23(Fn2::F24, (List::<Adt2>::nil(), v0.clone()), v1.clone())
}

pub fn f16(v0: u64, v1: List<Adt2>, v2: Adt2) -> R<List<Adt2>> {
    let a127 = {
        let a124 = v1.clone();
        length_list(a124)
    };
    let a128 = {
        let a125 = v0;
        let a126 = v0;
        nat_add(a125, a126)?
    };
    if nat_lt(a127, a128) {
        let a129 = v1.clone();
        let a130 = List::cons(v2.clone(), List::<Adt2>::nil());
        Ok(append_list(a129, a130))
    } else {
        Ok(v1.clone())
    }
}

pub fn f17(v0: (u64, u64)) -> R<u64> {
    let a134 = 1u64;
    let a135 = {
        let a132 = 1u64;
        let a133 = {
            let a131 = f25(v0);
            length_list(a131)
        };
        nat_add(a132, a133)?
    };
    nat_add(a134, a135)
}

pub fn f18(v0: (u64, u64)) -> Option<(u64, u64)> {
    Some(v0)
}

pub fn f19(v0: u64, v1: (u64, u64)) -> bool {
    let a137 = {
        let (_, h136) = v1;
        h136
    };
    let a138 = v0;
    if nat_eq(a137, a138) {
        let a140 = {
            let (h139, _) = v1;
            h139
        };
        let a141 = 4u64;
        nat_le(a140, a141)
    } else {
        false
    }
}

pub fn f20(v0: Adt2) -> R<List<Adt2>> {
    let m143 = f26(match v0.clone() {
        Adt2::C0(h142, _) => {
            h142
        }
    }, Adt0::C0)?;
    match m143 {
        None => {
            Ok(List::<Adt2>::nil())
        }
        Some(v1) => {
            Ok(List::cons(Adt2::C0(v1, {
                let a145 = match v0.clone() {
                    Adt2::C0(_, h144) => {
                        h144
                    }
                };
                let a146 = List::cons(Adt0::C0, List::<Adt0>::nil());
                append_list(a145, a146)
            }), List::<Adt2>::nil()))
        }
    }
}

pub fn f21(v0: Adt2) -> R<List<Adt2>> {
    let m148 = f26(match v0.clone() {
        Adt2::C0(h147, _) => {
            h147
        }
    }, Adt0::C1)?;
    match m148 {
        None => {
            Ok(List::<Adt2>::nil())
        }
        Some(v1) => {
            Ok(List::cons(Adt2::C0(v1, {
                let a150 = match v0.clone() {
                    Adt2::C0(_, h149) => {
                        h149
                    }
                };
                let a151 = List::cons(Adt0::C1, List::<Adt0>::nil());
                append_list(a150, a151)
            }), List::<Adt2>::nil()))
        }
    }
}

pub fn f22(v0: Adt2) -> R<List<Adt2>> {
    f27(Fn3::F28(v0.clone()), List::<Adt2>::nil(), f25(match v0.clone() {
        Adt2::C0(h152, _) => {
            h152
        }
    }))
}

pub fn f23(v0: Fn2, v1: (List<Adt2>, List<(u64, u64)>), v2: List<Adt2>) -> (List<Adt2>, List<(u64, u64)>) {
    let m153 = v2.clone();
    match m153.uncons() {
        None => {
            v1.clone()
        }
        Some((v3, v4)) => {
            f23(v0.clone(), {
                let c154: Fn2 = v0.clone();
                c154.apply(v1.clone(), v3.clone())
            }, v4.clone())
        }
    }
}

pub fn f24(v0: (List<Adt2>, List<(u64, u64)>), v1: Adt2) -> (List<Adt2>, List<(u64, u64)>) {
    if f29({
        let (_, h155) = v0.clone();
        h155
    }, match v1.clone() {
        Adt2::C0(h156, _) => {
            h156
        }
    }) {
        v0.clone()
    } else {
        ({
            let a158 = {
                let (h157, _) = v0.clone();
                h157
            };
            let a159 = List::cons(v1.clone(), List::<Adt2>::nil());
            append_list(a158, a159)
        }, f12({
            let (_, h160) = v0.clone();
            h160
        }, match v1.clone() {
            Adt2::C0(h161, _) => {
                h161
            }
        }))
    }
}

pub fn f25(_: (u64, u64)) -> List<u64> {
    List::cons(1u64, List::cons(2u64, List::cons(3u64, List::<u64>::nil())))
}

pub fn f26(v0: (u64, u64), v1: Adt0) -> R<Option<(u64, u64)>> {
    let m162 = v1.clone();
    match m162 {
        Adt0::C0 => {
            Ok(f30(v0))
        }
        Adt0::C1 => {
            Ok(f31(v0))
        }
        Adt0::C2(v2) => {
            f32(v0, v2)
        }
    }
}

pub fn f27(v0: Fn3, v1: List<Adt2>, v2: List<u64>) -> R<List<Adt2>> {
    let m163 = v2.clone();
    match m163.uncons() {
        None => {
            Ok(v1.clone())
        }
        Some((v3, v4)) => {
            f27(v0.clone(), {
                let c164: Fn3 = v0.clone();
                c164.apply(v1.clone(), v3)?
            }, v4.clone())
        }
    }
}

pub fn f28(v0: Adt2, v1: List<Adt2>, v2: u64) -> R<List<Adt2>> {
    f33(v0.clone(), v1.clone(), v2)
}

pub fn f29(v0: List<(u64, u64)>, v1: (u64, u64)) -> bool {
    let m165 = v0.clone();
    match m165.uncons() {
        None => {
            false
        }
        Some((v2, v3)) => {
            let m168 = {
                let a166 = v1;
                let a167 = v2;
                compare(a166, a167)
            };
            match m168 {
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

pub fn f30(v0: (u64, u64)) -> Option<(u64, u64)> {
    if f34(v0) {
        Some(f35(v0))
    } else {
        None::<(u64, u64)>
    }
}

pub fn f31(v0: (u64, u64)) -> Option<(u64, u64)> {
    if f36(v0) {
        Some(f37(v0))
    } else {
        None::<(u64, u64)>
    }
}

pub fn f32(v0: (u64, u64), v1: u64) -> R<Option<(u64, u64)>> {
    if f38(v0, v1)? {
        Ok(Some(f39(v0, v1)?))
    } else {
        Ok(None::<(u64, u64)>)
    }
}

pub fn f33(v0: Adt2, v1: List<Adt2>, v2: u64) -> R<List<Adt2>> {
    let m170 = f26(match v0.clone() {
        Adt2::C0(h169, _) => {
            h169
        }
    }, Adt0::C2(v2))?;
    match m170 {
        None => {
            Ok(v1.clone())
        }
        Some(v3) => {
            let a174 = v1.clone();
            let a175 = List::cons(Adt2::C0(v3, {
                let a172 = match v0.clone() {
                    Adt2::C0(_, h171) => {
                        h171
                    }
                };
                let a173 = List::cons(Adt0::C2(v2), List::<Adt0>::nil());
                append_list(a172, a173)
            }), List::<Adt2>::nil());
            Ok(append_list(a174, a175))
        }
    }
}

pub fn f34(v0: (u64, u64)) -> bool {
    let a177 = {
        let (h176, _) = v0;
        h176
    };
    let a178 = 4u64;
    if nat_lt(a177, a178) {
        let a180 = {
            let (_, h179) = v0;
            h179
        };
        let a181 = 3u64;
        nat_le(a180, a181)
    } else {
        false
    }
}

pub fn f35(v0: (u64, u64)) -> (u64, u64) {
    (4u64, {
        let (_, h182) = v0;
        h182
    })
}

pub fn f36(v0: (u64, u64)) -> bool {
    let a184 = 0u64;
    let a185 = {
        let (_, h183) = v0;
        h183
    };
    if nat_lt(a184, a185) {
        let a187 = {
            let (h186, _) = v0;
            h186
        };
        let a188 = 4u64;
        nat_le(a187, a188)
    } else {
        false
    }
}

pub fn f37(v0: (u64, u64)) -> (u64, u64) {
    ({
        let (h189, _) = v0;
        h189
    }, 0u64)
}

pub fn f38(v0: (u64, u64), v1: u64) -> R<bool> {
    let a191 = v1;
    let a192 = {
        let (h190, _) = v0;
        h190
    };
    if nat_le(a191, a192) {
        let a196 = {
            let a194 = {
                let (_, h193) = v0;
                h193
            };
            let a195 = v1;
            nat_add(a194, a195)?
        };
        let a197 = 3u64;
        if nat_le(a196, a197) {
            let a199 = {
                let (h198, _) = v0;
                h198
            };
            let a200 = 4u64;
            Ok(nat_le(a199, a200))
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f39(v0: (u64, u64), v1: u64) -> R<(u64, u64)> {
    Ok(({
        let a202 = {
            let (h201, _) = v0;
            h201
        };
        let a203 = v1;
        nat_sub(a202, a203)
    }, {
        let a205 = {
            let (_, h204) = v0;
            h204
        };
        let a206 = v1;
        nat_add(a205, a206)?
    }))
}
