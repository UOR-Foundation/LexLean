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
    C0(List<Adt1>, List<(u64, u64)>, Option<((u64, u64), Adt1)>, bool, Adt3),
}

#[derive(Clone)]
pub enum Adt3 {
    C0(u64, u64, u64, u64, u64, u64),
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

pub fn f0(v0: u64) -> R<(Result<((u64, u64), List<Adt0>), (bool, bool)>, (u64, (u64, (u64, (u64, (u64, u64))))))> {
    let v1: (Adt2, bool) = f32(Fn0::F25(v0), {
        let a1 = v0;
        let a2 = 8u64;
        nat_mul(a1, a2)?
    }, f24(v0)?)?;
    Ok(({
        let m6 = {
            let m5 = {
                let (h4, _) = v1.clone();
                h4
            };
            match m5 {
                Adt2::C0(_, _, h3, _, _) => {
                    h3
                }
            }
        };
        match m6 {
            None => {
                Err::<((u64, u64), List<Adt0>), (bool, bool)>({
                    let a16 = {
                        let (_, h11) = v1.clone();
                        h11
                    };
                    let a17 = {
                        let a15 = {
                            let m14 = {
                                let (h13, _) = v1.clone();
                                h13
                            };
                            match m14 {
                                Adt2::C0(_, _, _, h12, _) => {
                                    h12
                                }
                            }
                        };
                        bool_not(a15)
                    };
                    if bool_and(a16, a17) {
                        (false, true)
                    } else {
                        (false, false)
                    }
                })
            }
            Some(v2) => {
                Ok::<((u64, u64), List<Adt0>), (bool, bool)>(({
                    let (h7, _) = v2.clone();
                    h7
                }, {
                    let m10 = {
                        let (_, h9) = v2.clone();
                        h9
                    };
                    match m10 {
                        Adt1::C0(_, h8) => {
                            h8
                        }
                    }
                }))
            }
        }
    }, ({
        let m22 = {
            let m21 = {
                let (h20, _) = v1.clone();
                h20
            };
            match m21 {
                Adt2::C0(_, _, _, _, h19) => {
                    h19
                }
            }
        };
        match m22 {
            Adt3::C0(h18, _, _, _, _, _) => {
                h18
            }
        }
    }, ({
        let m27 = {
            let m26 = {
                let (h25, _) = v1.clone();
                h25
            };
            match m26 {
                Adt2::C0(_, _, _, _, h24) => {
                    h24
                }
            }
        };
        match m27 {
            Adt3::C0(_, h23, _, _, _, _) => {
                h23
            }
        }
    }, ({
        let m32 = {
            let m31 = {
                let (h30, _) = v1.clone();
                h30
            };
            match m31 {
                Adt2::C0(_, _, _, _, h29) => {
                    h29
                }
            }
        };
        match m32 {
            Adt3::C0(_, _, h28, _, _, _) => {
                h28
            }
        }
    }, ({
        let m37 = {
            let m36 = {
                let (h35, _) = v1.clone();
                h35
            };
            match m36 {
                Adt2::C0(_, _, _, _, h34) => {
                    h34
                }
            }
        };
        match m37 {
            Adt3::C0(_, _, _, h33, _, _) => {
                h33
            }
        }
    }, ({
        let m42 = {
            let m41 = {
                let (h40, _) = v1.clone();
                h40
            };
            match m41 {
                Adt2::C0(_, _, _, _, h39) => {
                    h39
                }
            }
        };
        match m42 {
            Adt3::C0(_, _, _, _, h38, _) => {
                h38
            }
        }
    }, {
        let m47 = {
            let m46 = {
                let (h45, _) = v1.clone();
                h45
            };
            match m46 {
                Adt2::C0(_, _, _, _, h44) => {
                    h44
                }
            }
        };
        match m47 {
            Adt3::C0(_, _, _, _, _, h43) => {
                h43
            }
        }
    })))))))
}

pub fn f1(v0: (u64, u64)) -> bool {
    let a54 = {
        let a49 = {
            let (h48, _) = v0;
            h48
        };
        let a50 = 4u64;
        nat_lt(a49, a50)
    };
    let a55 = {
        let a52 = {
            let (_, h51) = v0;
            h51
        };
        let a53 = 3u64;
        nat_le(a52, a53)
    };
    bool_and(a54, a55)
}

pub fn f2(v0: (u64, u64)) -> (u64, u64) {
    (4u64, {
        let (_, h56) = v0;
        h56
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
    let a63 = {
        let a58 = 0u64;
        let a59 = {
            let (_, h57) = v0;
            h57
        };
        nat_lt(a58, a59)
    };
    let a64 = {
        let a61 = {
            let (h60, _) = v0;
            h60
        };
        let a62 = 4u64;
        nat_le(a61, a62)
    };
    bool_and(a63, a64)
}

pub fn f5(v0: (u64, u64)) -> (u64, u64) {
    ({
        let (h65, _) = v0;
        h65
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
    let a79 = {
        let a67 = v1;
        let a68 = {
            let (h66, _) = v0;
            h66
        };
        nat_le(a67, a68)
    };
    let a80 = {
        let a77 = {
            let a72 = {
                let a70 = {
                    let (_, h69) = v0;
                    h69
                };
                let a71 = v1;
                nat_add(a70, a71)?
            };
            let a73 = 3u64;
            nat_le(a72, a73)
        };
        let a78 = {
            let a75 = {
                let (h74, _) = v0;
                h74
            };
            let a76 = 4u64;
            nat_le(a75, a76)
        };
        bool_and(a77, a78)
    };
    Ok(bool_and(a79, a80))
}

pub fn f8(v0: (u64, u64), v1: u64) -> R<(u64, u64)> {
    Ok(({
        let a82 = {
            let (h81, _) = v0;
            h81
        };
        let a83 = v1;
        nat_sub(a82, a83)
    }, {
        let a85 = {
            let (_, h84) = v0;
            h84
        };
        let a86 = v1;
        nat_add(a85, a86)?
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
    let m87 = v1.clone();
    match m87 {
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
    let a94 = {
        let a89 = {
            let (_, h88) = v1;
            h88
        };
        let a90 = v0;
        nat_eq(a89, a90)
    };
    let a95 = {
        let a92 = {
            let (h91, _) = v1;
            h91
        };
        let a93 = 4u64;
        nat_le(a92, a93)
    };
    bool_and(a94, a95)
}

pub fn f15(v0: u64, v1: (u64, u64)) -> Option<(u64, u64)> {
    let m96 = f13(v1);
    match m96 {
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
    let m98 = f11(match v0.clone() {
        Adt1::C0(h97, _) => {
            h97
        }
    }, Adt0::C0)?;
    match m98 {
        None => {
            Ok(List::<Adt1>::nil())
        }
        Some(v1) => {
            Ok(List::cons(Adt1::C0(v1, {
                let a100 = match v0.clone() {
                    Adt1::C0(_, h99) => {
                        h99
                    }
                };
                let a101 = List::cons(Adt0::C0, List::<Adt0>::nil());
                append_list(a100, a101)
            }), List::<Adt1>::nil()))
        }
    }
}

pub fn f17(v0: Adt1) -> R<List<Adt1>> {
    let m103 = f11(match v0.clone() {
        Adt1::C0(h102, _) => {
            h102
        }
    }, Adt0::C1)?;
    match m103 {
        None => {
            Ok(List::<Adt1>::nil())
        }
        Some(v1) => {
            Ok(List::cons(Adt1::C0(v1, {
                let a105 = match v0.clone() {
                    Adt1::C0(_, h104) => {
                        h104
                    }
                };
                let a106 = List::cons(Adt0::C1, List::<Adt0>::nil());
                append_list(a105, a106)
            }), List::<Adt1>::nil()))
        }
    }
}

pub fn f18(v0: Adt1, v1: List<Adt1>, v2: u64) -> R<List<Adt1>> {
    let m108 = f11(match v0.clone() {
        Adt1::C0(h107, _) => {
            h107
        }
    }, Adt0::C2(v2))?;
    match m108 {
        None => {
            Ok(v1.clone())
        }
        Some(v3) => {
            let a112 = v1.clone();
            let a113 = List::cons(Adt1::C0(v3, {
                let a110 = match v0.clone() {
                    Adt1::C0(_, h109) => {
                        h109
                    }
                };
                let a111 = List::cons(Adt0::C2(v2), List::<Adt0>::nil());
                append_list(a110, a111)
            }), List::<Adt1>::nil());
            Ok(append_list(a112, a113))
        }
    }
}

pub fn f19(v0: Adt1) -> R<List<Adt1>> {
    f28(Fn1::F18(v0.clone()), List::<Adt1>::nil(), f9(match v0.clone() {
        Adt1::C0(h114, _) => {
            h114
        }
    }))
}

pub fn f20(v0: Adt1) -> R<List<Adt1>> {
    let a117 = f16(v0.clone())?;
    let a118 = {
        let a115 = f17(v0.clone())?;
        let a116 = f19(v0.clone())?;
        append_list(a115, a116)
    };
    Ok(append_list(a117, a118))
}

pub fn f21(v0: (List<Adt1>, List<(u64, u64)>), v1: Adt1) -> (List<Adt1>, List<(u64, u64)>) {
    if f30({
        let (_, h119) = v0.clone();
        h119
    }, match v1.clone() {
        Adt1::C0(h120, _) => {
            h120
        }
    }) {
        v0.clone()
    } else {
        ({
            let a122 = {
                let (h121, _) = v0.clone();
                h121
            };
            let a123 = List::cons(v1.clone(), List::<Adt1>::nil());
            append_list(a122, a123)
        }, f31({
            let (_, h124) = v0.clone();
            h124
        }, match v1.clone() {
            Adt1::C0(h125, _) => {
                h125
            }
        }))
    }
}

pub fn f22(v0: List<(u64, u64)>, v1: List<Adt1>) -> (List<Adt1>, List<(u64, u64)>) {
    f29(Fn2::F21, (List::<Adt1>::nil(), v0.clone()), v1.clone())
}

pub fn f23(v0: u64, v1: List<Adt1>, v2: Adt1) -> List<Adt1> {
    let a127 = {
        let a126 = v1.clone();
        length_list(a126)
    };
    let a128 = v0;
    if nat_lt(a127, a128) {
        let a129 = v1.clone();
        let a130 = List::cons(v2.clone(), List::<Adt1>::nil());
        append_list(a129, a130)
    } else {
        v1.clone()
    }
}

pub fn f24(v0: u64) -> R<Adt2> {
    let v1: u64 = {
        let a131 = v0;
        let a132 = v0;
        nat_add(a131, a132)?
    };
    let v2: List<Adt1> = f27(Fn3::F23(v1), List::<Adt1>::nil(), List::cons(Adt1::C0(f12(v0), List::<Adt0>::nil()), List::<Adt1>::nil()));
    Ok(Adt2::C0(v2.clone(), f31(List::<(u64, u64)>::nil(), f12(v0)), None::<((u64, u64), Adt1)>, {
        let a133 = v1;
        let a134 = 1u64;
        nat_lt(a133, a134)
    }, Adt3::C0(0u64, 0u64, 0u64, 0u64, 0u64, {
        let a135 = v2.clone();
        length_list(a135)
    })))
}

pub fn f25(v0: u64, v1: Adt2) -> R<Option<Adt2>> {
    let m137 = match v1.clone() {
        Adt2::C0(_, _, h136, _, _) => {
            h136
        }
    };
    match m137 {
        None => {
            let m139 = match v1.clone() {
                Adt2::C0(h138, _, _, _, _) => {
                    h138
                }
            };
            match m139.uncons() {
                None => {
                    Ok(None::<Adt2>)
                }
                Some((v3, v4)) => {
                    let m141 = f15(v0, match v3.clone() {
                        Adt1::C0(h140, _) => {
                            h140
                        }
                    });
                    match m141 {
                        None => {
                            let v6: List<Adt1> = f20(v3.clone())?;
                            let v7: (List<Adt1>, List<(u64, u64)>) = f22(match v1.clone() {
                                Adt2::C0(_, h166, _, _, _) => {
                                    h166
                                }
                            }, v6.clone());
                            let v8: List<Adt1> = {
                                let a168 = v4.clone();
                                let a169 = {
                                    let (h167, _) = v7.clone();
                                    h167
                                };
                                append_list(a168, a169)
                            };
                            let v9: List<Adt1> = f27(Fn3::F23({
                                let a170 = v0;
                                let a171 = v0;
                                nat_add(a170, a171)?
                            }), List::<Adt1>::nil(), v8.clone());
                            Ok(Some(Adt2::C0(v9.clone(), {
                                let (_, h172) = v7.clone();
                                h172
                            }, None::<((u64, u64), Adt1)>, {
                                let a179 = match v1.clone() {
                                    Adt2::C0(_, _, _, h173, _) => {
                                        h173
                                    }
                                };
                                let a180 = {
                                    let a177 = {
                                        let a174 = v0;
                                        let a175 = v0;
                                        nat_add(a174, a175)?
                                    };
                                    let a178 = {
                                        let a176 = v8.clone();
                                        length_list(a176)
                                    };
                                    nat_lt(a177, a178)
                                };
                                bool_or(a179, a180)
                            }, Adt3::C0({
                                let a184 = {
                                    let m183 = match v1.clone() {
                                        Adt2::C0(_, _, _, _, h182) => {
                                            h182
                                        }
                                    };
                                    match m183 {
                                        Adt3::C0(h181, _, _, _, _, _) => {
                                            h181
                                        }
                                    }
                                };
                                let a185 = 1u64;
                                nat_add(a184, a185)?
                            }, {
                                let a190 = {
                                    let m188 = match v1.clone() {
                                        Adt2::C0(_, _, _, _, h187) => {
                                            h187
                                        }
                                    };
                                    match m188 {
                                        Adt3::C0(_, h186, _, _, _, _) => {
                                            h186
                                        }
                                    }
                                };
                                let a191 = f26(match v3.clone() {
                                    Adt1::C0(h189, _) => {
                                        h189
                                    }
                                })?;
                                nat_add(a190, a191)?
                            }, {
                                let a196 = {
                                    let m194 = match v1.clone() {
                                        Adt2::C0(_, _, _, _, h193) => {
                                            h193
                                        }
                                    };
                                    match m194 {
                                        Adt3::C0(_, _, h192, _, _, _) => {
                                            h192
                                        }
                                    }
                                };
                                let a197 = {
                                    let a195 = v6.clone();
                                    length_list(a195)
                                };
                                nat_add(a196, a197)?
                            }, {
                                let a201 = {
                                    let m200 = match v1.clone() {
                                        Adt2::C0(_, _, _, _, h199) => {
                                            h199
                                        }
                                    };
                                    match m200 {
                                        Adt3::C0(_, _, _, h198, _, _) => {
                                            h198
                                        }
                                    }
                                };
                                let a202 = 1u64;
                                nat_add(a201, a202)?
                            }, {
                                let a206 = {
                                    let m205 = match v1.clone() {
                                        Adt2::C0(_, _, _, _, h204) => {
                                            h204
                                        }
                                    };
                                    match m205 {
                                        Adt3::C0(_, _, _, _, h203, _) => {
                                            h203
                                        }
                                    }
                                };
                                let a207 = 1u64;
                                nat_add(a206, a207)?
                            }, {
                                let a212 = {
                                    let m210 = match v1.clone() {
                                        Adt2::C0(_, _, _, _, h209) => {
                                            h209
                                        }
                                    };
                                    match m210 {
                                        Adt3::C0(_, _, _, _, _, h208) => {
                                            h208
                                        }
                                    }
                                };
                                let a213 = {
                                    let a211 = v9.clone();
                                    length_list(a211)
                                };
                                if nat_lt(a212, a213) {
                                    let a214 = v9.clone();
                                    length_list(a214)
                                } else {
                                    let m217 = match v1.clone() {
                                        Adt2::C0(_, _, _, _, h216) => {
                                            h216
                                        }
                                    };
                                    match m217 {
                                        Adt3::C0(_, _, _, _, _, h215) => {
                                            h215
                                        }
                                    }
                                }
                            }))))
                        }
                        Some(v5) => {
                            Ok(Some(Adt2::C0(v4.clone(), match v1.clone() {
                                Adt2::C0(_, h142, _, _, _) => {
                                    h142
                                }
                            }, Some((v5, v3.clone())), match v1.clone() {
                                Adt2::C0(_, _, _, h143, _) => {
                                    h143
                                }
                            }, Adt3::C0({
                                let a147 = {
                                    let m146 = match v1.clone() {
                                        Adt2::C0(_, _, _, _, h145) => {
                                            h145
                                        }
                                    };
                                    match m146 {
                                        Adt3::C0(h144, _, _, _, _, _) => {
                                            h144
                                        }
                                    }
                                };
                                let a148 = 1u64;
                                nat_add(a147, a148)?
                            }, {
                                let m151 = match v1.clone() {
                                    Adt2::C0(_, _, _, _, h150) => {
                                        h150
                                    }
                                };
                                match m151 {
                                    Adt3::C0(_, h149, _, _, _, _) => {
                                        h149
                                    }
                                }
                            }, {
                                let m154 = match v1.clone() {
                                    Adt2::C0(_, _, _, _, h153) => {
                                        h153
                                    }
                                };
                                match m154 {
                                    Adt3::C0(_, _, h152, _, _, _) => {
                                        h152
                                    }
                                }
                            }, {
                                let m157 = match v1.clone() {
                                    Adt2::C0(_, _, _, _, h156) => {
                                        h156
                                    }
                                };
                                match m157 {
                                    Adt3::C0(_, _, _, h155, _, _) => {
                                        h155
                                    }
                                }
                            }, {
                                let a161 = {
                                    let m160 = match v1.clone() {
                                        Adt2::C0(_, _, _, _, h159) => {
                                            h159
                                        }
                                    };
                                    match m160 {
                                        Adt3::C0(_, _, _, _, h158, _) => {
                                            h158
                                        }
                                    }
                                };
                                let a162 = 1u64;
                                nat_add(a161, a162)?
                            }, {
                                let m165 = match v1.clone() {
                                    Adt2::C0(_, _, _, _, h164) => {
                                        h164
                                    }
                                };
                                match m165 {
                                    Adt3::C0(_, _, _, _, _, h163) => {
                                        h163
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

pub fn f26(v0: (u64, u64)) -> R<u64> {
    let a221 = 1u64;
    let a222 = {
        let a219 = 1u64;
        let a220 = {
            let a218 = f9(v0);
            length_list(a218)
        };
        nat_add(a219, a220)?
    };
    nat_add(a221, a222)
}

pub fn f27(v0: Fn3, v1: List<Adt1>, v2: List<Adt1>) -> List<Adt1> {
    let m223 = v2.clone();
    match m223.uncons() {
        None => {
            v1.clone()
        }
        Some((v3, v4)) => {
            f27(v0.clone(), {
                let c224: Fn3 = v0.clone();
                c224.apply(v1.clone(), v3.clone())
            }, v4.clone())
        }
    }
}

pub fn f28(v0: Fn1, v1: List<Adt1>, v2: List<u64>) -> R<List<Adt1>> {
    let m225 = v2.clone();
    match m225.uncons() {
        None => {
            Ok(v1.clone())
        }
        Some((v3, v4)) => {
            f28(v0.clone(), {
                let c226: Fn1 = v0.clone();
                c226.apply(v1.clone(), v3)?
            }, v4.clone())
        }
    }
}

pub fn f29(v0: Fn2, v1: (List<Adt1>, List<(u64, u64)>), v2: List<Adt1>) -> (List<Adt1>, List<(u64, u64)>) {
    let m227 = v2.clone();
    match m227.uncons() {
        None => {
            v1.clone()
        }
        Some((v3, v4)) => {
            f29(v0.clone(), {
                let c228: Fn2 = v0.clone();
                c228.apply(v1.clone(), v3.clone())
            }, v4.clone())
        }
    }
}

pub fn f30(v0: List<(u64, u64)>, v1: (u64, u64)) -> bool {
    let m229 = v0.clone();
    match m229.uncons() {
        None => {
            false
        }
        Some((v2, v3)) => {
            let m232 = {
                let a230 = v1;
                let a231 = v2;
                compare(a230, a231)
            };
            match m232 {
                Ordering::Less => {
                    false
                }
                Ordering::Equal => {
                    true
                }
                Ordering::Greater => {
                    f30(v3.clone(), v1)
                }
            }
        }
    }
}

pub fn f31(v0: List<(u64, u64)>, v1: (u64, u64)) -> List<(u64, u64)> {
    let m233 = v0.clone();
    match m233.uncons() {
        None => {
            List::cons(v1, List::<(u64, u64)>::nil())
        }
        Some((v2, v3)) => {
            let m236 = {
                let a234 = v1;
                let a235 = v2;
                compare(a234, a235)
            };
            match m236 {
                Ordering::Less => {
                    List::cons(v1, v0.clone())
                }
                Ordering::Equal => {
                    v0.clone()
                }
                Ordering::Greater => {
                    List::cons(v2, f31(v3.clone(), v1))
                }
            }
        }
    }
}

pub fn f32(v0: Fn0, v1: u64, v2: Adt2) -> R<(Adt2, bool)> {
    let m237 = v1;
    if m237 == 0 {
        Ok((v2.clone(), false))
    } else {
        let v3 = m237 - 1;
        let m239 = {
            let c238: Fn0 = v0.clone();
            c238.apply(v2.clone())?
        };
        match m239 {
            None => {
                Ok((v2.clone(), true))
            }
            Some(v4) => {
                f32(v0.clone(), v3, v4.clone())
            }
        }
    }
}

pub fn run(p0: u64) -> R<(Result<((u64, u64), List<Adt0>), (bool, bool)>, (u64, (u64, (u64, (u64, (u64, u64))))))> {
    f0(p0)
}
