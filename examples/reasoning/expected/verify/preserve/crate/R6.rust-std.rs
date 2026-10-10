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
    C0(List<Adt2>, Option<(u64, Adt2)>, bool, Adt3),
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
    F10,
    F14,
}

impl Fn1 {
    pub fn apply(&self, p0: List<Adt2>, p1: Adt2) -> List<Adt2> {
        match self {
            Fn1::F10 => f10(p0, p1),
            Fn1::F14 => f14(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F21(Adt2),
}

impl Fn2 {
    pub fn apply(&self, p0: List<Adt2>, p1: u64) -> List<Adt2> {
        match self {
            Fn2::F21(k0) => f21(k0.clone(), p0, p1),
        }
    }
}

pub fn f0(v0: u64) -> R<u64> {
    let m1 = f1(v0)?;
    match m1 {
        Ok(v1) => {
            Ok(v1)
        }
        Err(_) => {
            Ok(9u64)
        }
    }
}

pub fn f1(v0: u64) -> R<Result<u64, (bool, bool)>> {
    let m2 = f2(v0)?;
    match m2 {
        Ok(v1) => {
            Ok(Ok::<u64, (bool, bool)>({
                let (h3, _) = v1.clone();
                h3
            }))
        }
        Err(v2) => {
            Ok(Err::<u64, (bool, bool)>(v2))
        }
    }
}

pub fn f2(v0: u64) -> R<Result<(u64, List<Adt0>), (bool, bool)>> {
    let v1: (Adt1, bool) = f3(v0)?;
    let m7 = {
        let m6 = {
            let (h5, _) = v1.clone();
            h5
        };
        match m6 {
            Adt1::C0(_, h4, _, _) => {
                h4
            }
        }
    };
    match m7 {
        None => {
            Ok(Err::<(u64, List<Adt0>), (bool, bool)>(f4({
                let (_, h12) = v1.clone();
                h12
            }, {
                let m15 = {
                    let (h14, _) = v1.clone();
                    h14
                };
                match m15 {
                    Adt1::C0(_, _, h13, _) => {
                        h13
                    }
                }
            })))
        }
        Some(v2) => {
            Ok(Ok::<(u64, List<Adt0>), (bool, bool)>(({
                let (h8, _) = v2.clone();
                h8
            }, {
                let m11 = {
                    let (_, h10) = v2.clone();
                    h10
                };
                match m11 {
                    Adt2::C0(_, h9) => {
                        h9
                    }
                }
            })))
        }
    }
}

pub fn f3(v0: u64) -> R<(Adt1, bool)> {
    f5(Fn0::F6(v0), 8u64, f7(v0))
}

pub fn f4(v0: bool, v1: bool) -> (bool, bool) {
    if if v0 {
        let a16 = v1;
        bool_not(a16)
    } else {
        false
    } {
        (false, true)
    } else {
        (false, false)
    }
}

pub fn f5(v0: Fn0, v1: u64, v2: Adt1) -> R<(Adt1, bool)> {
    let m17 = v1;
    if m17 == 0 {
        Ok((v2.clone(), false))
    } else {
        let v3 = m17 - 1;
        let m19 = {
            let c18: Fn0 = v0.clone();
            c18.apply(v2.clone())?
        };
        match m19 {
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

pub fn f7(v0: u64) -> Adt1 {
    let v1: List<Adt2> = f9(Fn1::F10, List::<Adt2>::nil(), List::cons(Adt2::C0(f11(v0), List::<Adt0>::nil()), List::<Adt2>::nil()));
    Adt1::C0(v1.clone(), None::<(u64, Adt2)>, {
        let a20 = 3u64;
        let a21 = 1u64;
        nat_lt(a20, a21)
    }, Adt3::C0(0u64, 0u64, 0u64, 0u64, 0u64, {
        let a22 = v1.clone();
        length_list(a22)
    }))
}

pub fn f8(v0: u64, v1: Adt1) -> R<Option<Adt1>> {
    let m24 = match v1.clone() {
        Adt1::C0(_, h23, _, _) => {
            h23
        }
    };
    match m24 {
        None => {
            let m26 = match v1.clone() {
                Adt1::C0(h25, _, _, _) => {
                    h25
                }
            };
            match m26.uncons() {
                None => {
                    Ok(None::<Adt1>)
                }
                Some((v3, v4)) => {
                    let m28 = f12(v0, match v3.clone() {
                        Adt2::C0(h27, _) => {
                            h27
                        }
                    })?;
                    match m28 {
                        None => {
                            let v6: List<Adt2> = f13(v3.clone());
                            let v7: List<Adt2> = v6.clone();
                            let v8: List<Adt2> = {
                                let a52 = v7.clone();
                                let a53 = v4.clone();
                                append_list(a52, a53)
                            };
                            let v9: List<Adt2> = f9(Fn1::F14, List::<Adt2>::nil(), v8.clone());
                            Ok(Some(Adt1::C0(v9.clone(), None::<(u64, Adt2)>, if match v1.clone() {
                                Adt1::C0(_, _, h54, _) => {
                                    h54
                                }
                            } {
                                true
                            } else {
                                let a56 = 3u64;
                                let a57 = {
                                    let a55 = v8.clone();
                                    length_list(a55)
                                };
                                nat_lt(a56, a57)
                            }, Adt3::C0({
                                let a61 = {
                                    let m60 = match v1.clone() {
                                        Adt1::C0(_, _, _, h59) => {
                                            h59
                                        }
                                    };
                                    match m60 {
                                        Adt3::C0(h58, _, _, _, _, _) => {
                                            h58
                                        }
                                    }
                                };
                                let a62 = 1u64;
                                nat_add(a61, a62)?
                            }, {
                                let a67 = {
                                    let m65 = match v1.clone() {
                                        Adt1::C0(_, _, _, h64) => {
                                            h64
                                        }
                                    };
                                    match m65 {
                                        Adt3::C0(_, h63, _, _, _, _) => {
                                            h63
                                        }
                                    }
                                };
                                let a68 = f15(match v3.clone() {
                                    Adt2::C0(h66, _) => {
                                        h66
                                    }
                                });
                                nat_add(a67, a68)?
                            }, {
                                let a73 = {
                                    let m71 = match v1.clone() {
                                        Adt1::C0(_, _, _, h70) => {
                                            h70
                                        }
                                    };
                                    match m71 {
                                        Adt3::C0(_, _, h69, _, _, _) => {
                                            h69
                                        }
                                    }
                                };
                                let a74 = {
                                    let a72 = v6.clone();
                                    length_list(a72)
                                };
                                nat_add(a73, a74)?
                            }, {
                                let a78 = {
                                    let m77 = match v1.clone() {
                                        Adt1::C0(_, _, _, h76) => {
                                            h76
                                        }
                                    };
                                    match m77 {
                                        Adt3::C0(_, _, _, h75, _, _) => {
                                            h75
                                        }
                                    }
                                };
                                let a79 = 1u64;
                                nat_add(a78, a79)?
                            }, {
                                let a83 = {
                                    let m82 = match v1.clone() {
                                        Adt1::C0(_, _, _, h81) => {
                                            h81
                                        }
                                    };
                                    match m82 {
                                        Adt3::C0(_, _, _, _, h80, _) => {
                                            h80
                                        }
                                    }
                                };
                                let a84 = 1u64;
                                nat_add(a83, a84)?
                            }, {
                                let a89 = {
                                    let m87 = match v1.clone() {
                                        Adt1::C0(_, _, _, h86) => {
                                            h86
                                        }
                                    };
                                    match m87 {
                                        Adt3::C0(_, _, _, _, _, h85) => {
                                            h85
                                        }
                                    }
                                };
                                let a90 = {
                                    let a88 = v9.clone();
                                    length_list(a88)
                                };
                                if nat_lt(a89, a90) {
                                    let a91 = v9.clone();
                                    length_list(a91)
                                } else {
                                    let m94 = match v1.clone() {
                                        Adt1::C0(_, _, _, h93) => {
                                            h93
                                        }
                                    };
                                    match m94 {
                                        Adt3::C0(_, _, _, _, _, h92) => {
                                            h92
                                        }
                                    }
                                }
                            }))))
                        }
                        Some(v5) => {
                            Ok(Some(Adt1::C0(v4.clone(), Some((v5, v3.clone())), match v1.clone() {
                                Adt1::C0(_, _, h29, _) => {
                                    h29
                                }
                            }, Adt3::C0({
                                let a33 = {
                                    let m32 = match v1.clone() {
                                        Adt1::C0(_, _, _, h31) => {
                                            h31
                                        }
                                    };
                                    match m32 {
                                        Adt3::C0(h30, _, _, _, _, _) => {
                                            h30
                                        }
                                    }
                                };
                                let a34 = 1u64;
                                nat_add(a33, a34)?
                            }, {
                                let m37 = match v1.clone() {
                                    Adt1::C0(_, _, _, h36) => {
                                        h36
                                    }
                                };
                                match m37 {
                                    Adt3::C0(_, h35, _, _, _, _) => {
                                        h35
                                    }
                                }
                            }, {
                                let m40 = match v1.clone() {
                                    Adt1::C0(_, _, _, h39) => {
                                        h39
                                    }
                                };
                                match m40 {
                                    Adt3::C0(_, _, h38, _, _, _) => {
                                        h38
                                    }
                                }
                            }, {
                                let m43 = match v1.clone() {
                                    Adt1::C0(_, _, _, h42) => {
                                        h42
                                    }
                                };
                                match m43 {
                                    Adt3::C0(_, _, _, h41, _, _) => {
                                        h41
                                    }
                                }
                            }, {
                                let a47 = {
                                    let m46 = match v1.clone() {
                                        Adt1::C0(_, _, _, h45) => {
                                            h45
                                        }
                                    };
                                    match m46 {
                                        Adt3::C0(_, _, _, _, h44, _) => {
                                            h44
                                        }
                                    }
                                };
                                let a48 = 1u64;
                                nat_add(a47, a48)?
                            }, {
                                let m51 = match v1.clone() {
                                    Adt1::C0(_, _, _, h50) => {
                                        h50
                                    }
                                };
                                match m51 {
                                    Adt3::C0(_, _, _, _, _, h49) => {
                                        h49
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

pub fn f9(v0: Fn1, v1: List<Adt2>, v2: List<Adt2>) -> List<Adt2> {
    let m95 = v2.clone();
    match m95.uncons() {
        None => {
            v1.clone()
        }
        Some((v3, v4)) => {
            f9(v0.clone(), {
                let c96: Fn1 = v0.clone();
                c96.apply(v1.clone(), v3.clone())
            }, v4.clone())
        }
    }
}

pub fn f10(v0: List<Adt2>, v1: Adt2) -> List<Adt2> {
    let a98 = {
        let a97 = v0.clone();
        length_list(a97)
    };
    let a99 = 3u64;
    if nat_lt(a98, a99) {
        let a100 = v0.clone();
        let a101 = List::cons(v1.clone(), List::<Adt2>::nil());
        append_list(a100, a101)
    } else {
        v0.clone()
    }
}

pub fn f11(v0: u64) -> (u64, u64) {
    (v0, 0u64)
}

pub fn f12(v0: u64, v1: (u64, u64)) -> R<Option<u64>> {
    let m102 = f16(v1);
    match m102 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f17(v0, v2)? {
                Ok(Some(v2))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f13(v0: Adt2) -> List<Adt2> {
    f18(v0.clone())
}

pub fn f14(v0: List<Adt2>, v1: Adt2) -> List<Adt2> {
    let a104 = {
        let a103 = v0.clone();
        length_list(a103)
    };
    let a105 = 3u64;
    if nat_lt(a104, a105) {
        let a106 = v0.clone();
        let a107 = List::cons(v1.clone(), List::<Adt2>::nil());
        append_list(a106, a107)
    } else {
        v0.clone()
    }
}

pub fn f15(v0: (u64, u64)) -> u64 {
    let a108 = f19(v0);
    length_list(a108)
}

pub fn f16(v0: (u64, u64)) -> Option<u64> {
    let a110 = 0u64;
    let a111 = {
        let (_, h109) = v0;
        h109
    };
    if nat_lt(a110, a111) {
        Some({
            let (_, h112) = v0;
            h112
        })
    } else {
        None::<u64>
    }
}

pub fn f17(v0: u64, v1: u64) -> R<bool> {
    let a115 = {
        let a113 = v1;
        let a114 = v1;
        nat_add(a113, a114)?
    };
    let a116 = v0;
    Ok(nat_le(a115, a116))
}

pub fn f18(v0: Adt2) -> List<Adt2> {
    f20(Fn2::F21(v0.clone()), List::<Adt2>::nil(), f19(match v0.clone() {
        Adt2::C0(h117, _) => {
            h117
        }
    }))
}

pub fn f19(v0: (u64, u64)) -> List<u64> {
    let m119 = {
        let v1: u64 = {
            let (h118, _) = v0;
            h118
        };
        let v2: List<u64> = f22(v1);
        if f23(v1, v2.clone()) {
            Ok::<List<u64>, (bool, bool)>(v2.clone())
        } else {
            Err::<List<u64>, (bool, bool)>((true, false))
        }
    };
    match m119 {
        Ok(v3) => {
            v3.clone()
        }
        Err(_) => {
            List::<u64>::nil()
        }
    }
}

pub fn f20(v0: Fn2, v1: List<Adt2>, v2: List<u64>) -> List<Adt2> {
    let m120 = v2.clone();
    match m120.uncons() {
        None => {
            v1.clone()
        }
        Some((v3, v4)) => {
            f20(v0.clone(), {
                let c121: Fn2 = v0.clone();
                c121.apply(v1.clone(), v3)
            }, v4.clone())
        }
    }
}

pub fn f21(v0: Adt2, v1: List<Adt2>, v2: u64) -> List<Adt2> {
    f24(v0.clone(), v1.clone(), v2)
}

pub fn f22(v0: u64) -> List<u64> {
    f25(v0)
}

pub fn f23(_: u64, v1: List<u64>) -> bool {
    let a123 = {
        let a122 = v1.clone();
        length_list(a122)
    };
    let a124 = 4u64;
    nat_le(a123, a124)
}

pub fn f24(v0: Adt2, v1: List<Adt2>, v2: u64) -> List<Adt2> {
    let m126 = f26(match v0.clone() {
        Adt2::C0(h125, _) => {
            h125
        }
    }, Adt0::C0(v2));
    match m126 {
        None => {
            v1.clone()
        }
        Some(v3) => {
            let a130 = v1.clone();
            let a131 = List::cons(Adt2::C0(v3, {
                let a128 = match v0.clone() {
                    Adt2::C0(_, h127) => {
                        h127
                    }
                };
                let a129 = List::cons(Adt0::C0(v2), List::<Adt0>::nil());
                append_list(a128, a129)
            }), List::<Adt2>::nil());
            append_list(a130, a131)
        }
    }
}

pub fn f25(v0: u64) -> List<u64> {
    List::cons({
        let a132 = v0;
        let a133 = 10u64;
        nat_sub(a132, a133)
    }, List::cons({
        let a134 = v0;
        let a135 = 20u64;
        nat_sub(a134, a135)
    }, List::cons({
        let a136 = v0;
        let a137 = 30u64;
        nat_sub(a136, a137)
    }, List::cons({
        let a138 = v0;
        let a139 = 50u64;
        nat_sub(a138, a139)
    }, List::<u64>::nil()))))
}

pub fn f26(v0: (u64, u64), v1: Adt0) -> Option<(u64, u64)> {
    let m140 = v1.clone();
    match m140 {
        Adt0::C0(v2) => {
            f27(v0, v2)
        }
    }
}

pub fn f27(v0: (u64, u64), v1: u64) -> Option<(u64, u64)> {
    if f28(v0, v1) {
        Some(f29(v0, v1))
    } else {
        None::<(u64, u64)>
    }
}

pub fn f28(v0: (u64, u64), v1: u64) -> bool {
    let a142 = {
        let (_, h141) = v0;
        h141
    };
    let a143 = 0u64;
    if nat_eq(a142, a143) {
        let a144 = 0u64;
        let a145 = v1;
        nat_lt(a144, a145)
    } else {
        false
    }
}

pub fn f29(v0: (u64, u64), v1: u64) -> (u64, u64) {
    ({
        let (h146, _) = v0;
        h146
    }, v1)
}
