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
    C0(u64, u64, u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt1 {
    C0,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
}

#[derive(Clone)]
pub enum Adt2 {
    C0(Adt3, List<Adt1>, Adt4),
}

#[derive(Clone)]
pub enum Adt3 {
    C0(Adt0, u64, u64, u64, u64, u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt4 {
    C0(u64, u64, u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Fn0 {
    F5,
}

impl Fn0 {
    pub fn apply(&self, p0: Adt2) -> R<Option<Adt2>> {
        match self {
            Fn0::F5 => f5(p0),
        }
    }
}

pub fn f0(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<u64> {
    let m1 = f1(Adt0::C0(v0, v1, v2, v3, v4, v5))?;
    match m1 {
        Ok(v6) => {
            let a3 = {
                let (_, h2) = v6.clone();
                h2
            };
            Ok(length_list(a3))
        }
        Err(_) => {
            Ok(0u64)
        }
    }
}

pub fn f1(v0: Adt0) -> R<Result<(u64, List<Adt1>), (bool, bool)>> {
    let v1: (Adt2, bool) = f2(v0.clone())?;
    let m5 = {
        let (_, h4) = v1.clone();
        h4
    };
    if m5 {
        let m9 = f3(v0.clone(), {
            let m8 = {
                let (h7, _) = v1.clone();
                h7
            };
            match m8 {
                Adt2::C0(h6, _, _) => {
                    h6
                }
            }
        });
        match m9 {
            Ok(v2) => {
                Ok(Ok::<(u64, List<Adt1>), (bool, bool)>((v2, {
                    let m12 = {
                        let (h11, _) = v1.clone();
                        h11
                    };
                    match m12 {
                        Adt2::C0(_, h10, _) => {
                            h10
                        }
                    }
                })))
            }
            Err(v3) => {
                Ok(Err::<(u64, List<Adt1>), (bool, bool)>(v3))
            }
        }
    } else {
        Ok(Err::<(u64, List<Adt1>), (bool, bool)>((false, false)))
    }
}

pub fn f2(v0: Adt0) -> R<(Adt2, bool)> {
    f4(Fn0::F5, 11u64, f6(v0.clone()))
}

pub fn f3(v0: Adt0, v1: Adt3) -> Result<u64, (bool, bool)> {
    let m13 = f7(v0.clone(), v1.clone());
    match m13 {
        None => {
            let m14 = f8(v1.clone());
            match m14 {
                None => {
                    Err::<u64, (bool, bool)>((false, true))
                }
                Some(_) => {
                    Err::<u64, (bool, bool)>((true, false))
                }
            }
        }
        Some(v2) => {
            Ok::<u64, (bool, bool)>(v2)
        }
    }
}

pub fn f4(v0: Fn0, v1: u64, v2: Adt2) -> R<(Adt2, bool)> {
    let m15 = v1;
    if m15 == 0 {
        Ok((v2.clone(), false))
    } else {
        let v3 = m15 - 1;
        let m17 = {
            let c16: Fn0 = v0.clone();
            c16.apply(v2.clone())?
        };
        match m17 {
            None => {
                Ok((v2.clone(), true))
            }
            Some(v4) => {
                f4(v0.clone(), v3, v4.clone())
            }
        }
    }
}

pub fn f5(v0: Adt2) -> R<Option<Adt2>> {
    let m19 = f9(match v0.clone() {
        Adt2::C0(h18, _, _) => {
            h18
        }
    })?;
    match m19 {
        None => {
            Ok(None::<Adt2>)
        }
        Some(v1) => {
            let m21 = f10(match v0.clone() {
                Adt2::C0(h20, _, _) => {
                    h20
                }
            }, v1.clone())?;
            match m21 {
                None => {
                    Ok(None::<Adt2>)
                }
                Some(v2) => {
                    Ok(Some(Adt2::C0(v2.clone(), {
                        let a23 = match v0.clone() {
                            Adt2::C0(_, h22, _) => {
                                h22
                            }
                        };
                        let a24 = List::cons(v1.clone(), List::<Adt1>::nil());
                        append_list(a23, a24)
                    }, Adt4::C0({
                        let a28 = {
                            let m27 = match v0.clone() {
                                Adt2::C0(_, _, h26) => {
                                    h26
                                }
                            };
                            match m27 {
                                Adt4::C0(h25, _, _, _, _, _) => {
                                    h25
                                }
                            }
                        };
                        let a29 = 1u64;
                        nat_add(a28, a29)?
                    }, {
                        let a34 = {
                            let m32 = match v0.clone() {
                                Adt2::C0(_, _, h31) => {
                                    h31
                                }
                            };
                            match m32 {
                                Adt4::C0(_, h30, _, _, _, _) => {
                                    h30
                                }
                            }
                        };
                        let a35 = f11(match v0.clone() {
                            Adt2::C0(h33, _, _) => {
                                h33
                            }
                        })?;
                        nat_add(a34, a35)?
                    }, {
                        let a39 = {
                            let m38 = match v0.clone() {
                                Adt2::C0(_, _, h37) => {
                                    h37
                                }
                            };
                            match m38 {
                                Adt4::C0(_, _, h36, _, _, _) => {
                                    h36
                                }
                            }
                        };
                        let a40 = 1u64;
                        nat_add(a39, a40)?
                    }, {
                        let m43 = match v0.clone() {
                            Adt2::C0(_, _, h42) => {
                                h42
                            }
                        };
                        match m43 {
                            Adt4::C0(_, _, _, h41, _, _) => {
                                h41
                            }
                        }
                    }, {
                        let m46 = match v0.clone() {
                            Adt2::C0(_, _, h45) => {
                                h45
                            }
                        };
                        match m46 {
                            Adt4::C0(_, _, _, _, h44, _) => {
                                h44
                            }
                        }
                    }, {
                        let m49 = match v0.clone() {
                            Adt2::C0(_, _, h48) => {
                                h48
                            }
                        };
                        match m49 {
                            Adt4::C0(_, _, _, _, _, h47) => {
                                h47
                            }
                        }
                    }))))
                }
            }
        }
    }
}

pub fn f6(v0: Adt0) -> Adt2 {
    Adt2::C0(f12(v0.clone()), List::<Adt1>::nil(), Adt4::C0(0u64, 0u64, 0u64, 0u64, 0u64, 0u64))
}

pub fn f7(v0: Adt0, v1: Adt3) -> Option<u64> {
    let m50 = f8(v1.clone());
    match m50 {
        None => {
            None::<u64>
        }
        Some(v2) => {
            if f13(v0.clone(), v2) {
                Some(v2)
            } else {
                None::<u64>
            }
        }
    }
}

pub fn f8(v0: Adt3) -> Option<u64> {
    Some(match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, h51) => {
            h51
        }
    })
}

pub fn f9(v0: Adt3) -> R<Option<Adt1>> {
    let m55 = {
        let m53 = {
            let m52 = if f14(v0.clone()) {
                Some(Adt1::C0)
            } else {
                None::<Adt1>
            };
            match m52 {
                None => {
                    if f15(v0.clone()) {
                        Some(Adt1::C1)
                    } else {
                        None::<Adt1>
                    }
                }
                Some(v1) => {
                    Some(v1.clone())
                }
            }
        };
        match m53 {
            None => {
                let m54 = if f16(v0.clone()) {
                    Some(Adt1::C2)
                } else {
                    None::<Adt1>
                };
                match m54 {
                    None => {
                        if f17(v0.clone()) {
                            Some(Adt1::C3)
                        } else {
                            None::<Adt1>
                        }
                    }
                    Some(v3) => {
                        Some(v3.clone())
                    }
                }
            }
            Some(v2) => {
                Some(v2.clone())
            }
        }
    };
    match m55 {
        None => {
            let m57 = {
                let m56 = if f18(v0.clone())? {
                    Some(Adt1::C4)
                } else {
                    None::<Adt1>
                };
                match m56 {
                    None => {
                        if f19(v0.clone()) {
                            Some(Adt1::C5)
                        } else {
                            None::<Adt1>
                        }
                    }
                    Some(v5) => {
                        Some(v5.clone())
                    }
                }
            };
            match m57 {
                None => {
                    let m58 = if f20(v0.clone()) {
                        Some(Adt1::C6)
                    } else {
                        None::<Adt1>
                    };
                    match m58 {
                        None => {
                            if f21(v0.clone()) {
                                Ok(Some(Adt1::C7))
                            } else {
                                Ok(None::<Adt1>)
                            }
                        }
                        Some(v7) => {
                            Ok(Some(v7.clone()))
                        }
                    }
                }
                Some(v6) => {
                    Ok(Some(v6.clone()))
                }
            }
        }
        Some(v4) => {
            Ok(Some(v4.clone()))
        }
    }
}

pub fn f10(v0: Adt3, v1: Adt1) -> R<Option<Adt3>> {
    let m59 = v1.clone();
    match m59 {
        Adt1::C0 => {
            Ok(f22(v0.clone()))
        }
        Adt1::C1 => {
            Ok(f23(v0.clone()))
        }
        Adt1::C2 => {
            Ok(f24(v0.clone()))
        }
        Adt1::C3 => {
            Ok(f25(v0.clone()))
        }
        Adt1::C4 => {
            f26(v0.clone())
        }
        Adt1::C5 => {
            Ok(f27(v0.clone()))
        }
        Adt1::C6 => {
            Ok(f28(v0.clone()))
        }
        Adt1::C7 => {
            f29(v0.clone())
        }
    }
}

pub fn f11(v0: Adt3) -> R<u64> {
    let (_, h60) = {
        let v7: (bool, u64) = {
            let v3: (bool, u64) = {
                let v1: (bool, u64) = (f14(v0.clone()), 1u64);
                let (h61, _) = v1;
                if h61 {
                    v1
                } else {
                    let v2: (bool, u64) = (f15(v0.clone()), 1u64);
                    ({
                        let (h62, _) = v2;
                        h62
                    }, {
                        let a65 = {
                            let (_, h63) = v1;
                            h63
                        };
                        let a66 = {
                            let (_, h64) = v2;
                            h64
                        };
                        nat_add(a65, a66)?
                    })
                }
            };
            let (h67, _) = v3;
            if h67 {
                v3
            } else {
                let v6: (bool, u64) = {
                    let v4: (bool, u64) = (f16(v0.clone()), 1u64);
                    let (h68, _) = v4;
                    if h68 {
                        v4
                    } else {
                        let v5: (bool, u64) = (f17(v0.clone()), 1u64);
                        ({
                            let (h69, _) = v5;
                            h69
                        }, {
                            let a72 = {
                                let (_, h70) = v4;
                                h70
                            };
                            let a73 = {
                                let (_, h71) = v5;
                                h71
                            };
                            nat_add(a72, a73)?
                        })
                    }
                };
                ({
                    let (h74, _) = v6;
                    h74
                }, {
                    let a77 = {
                        let (_, h75) = v3;
                        h75
                    };
                    let a78 = {
                        let (_, h76) = v6;
                        h76
                    };
                    nat_add(a77, a78)?
                })
            }
        };
        let (h79, _) = v7;
        if h79 {
            v7
        } else {
            let v14: (bool, u64) = {
                let v10: (bool, u64) = {
                    let v8: (bool, u64) = (f18(v0.clone())?, 1u64);
                    let (h80, _) = v8;
                    if h80 {
                        v8
                    } else {
                        let v9: (bool, u64) = (f19(v0.clone()), 1u64);
                        ({
                            let (h81, _) = v9;
                            h81
                        }, {
                            let a84 = {
                                let (_, h82) = v8;
                                h82
                            };
                            let a85 = {
                                let (_, h83) = v9;
                                h83
                            };
                            nat_add(a84, a85)?
                        })
                    }
                };
                let (h86, _) = v10;
                if h86 {
                    v10
                } else {
                    let v13: (bool, u64) = {
                        let v11: (bool, u64) = (f20(v0.clone()), 1u64);
                        let (h87, _) = v11;
                        if h87 {
                            v11
                        } else {
                            let v12: (bool, u64) = (f21(v0.clone()), 1u64);
                            ({
                                let (h88, _) = v12;
                                h88
                            }, {
                                let a91 = {
                                    let (_, h89) = v11;
                                    h89
                                };
                                let a92 = {
                                    let (_, h90) = v12;
                                    h90
                                };
                                nat_add(a91, a92)?
                            })
                        }
                    };
                    ({
                        let (h93, _) = v13;
                        h93
                    }, {
                        let a96 = {
                            let (_, h94) = v10;
                            h94
                        };
                        let a97 = {
                            let (_, h95) = v13;
                            h95
                        };
                        nat_add(a96, a97)?
                    })
                }
            };
            ({
                let (h98, _) = v14;
                h98
            }, {
                let a101 = {
                    let (_, h99) = v7;
                    h99
                };
                let a102 = {
                    let (_, h100) = v14;
                    h100
                };
                nat_add(a101, a102)?
            })
        }
    };
    Ok(h60)
}

pub fn f12(v0: Adt0) -> Adt3 {
    Adt3::C0(v0.clone(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64)
}

pub fn f13(v0: Adt0, v1: u64) -> bool {
    let a103 = v1;
    let a104 = 3u64;
    if nat_le(a103, a104) {
        let a105 = v1;
        let a106 = 3u64;
        if nat_eq(a105, a106) {
            true
        } else {
            let a110 = if f30(v0.clone()) {
                let a108 = match v0.clone() {
                    Adt0::C0(_, _, _, _, _, h107) => {
                        h107
                    }
                };
                let a109 = 1u64;
                if nat_eq(a108, a109) {
                    if f31(v0.clone()) {
                        f32(v0.clone())
                    } else {
                        false
                    }
                } else {
                    false
                }
            } else {
                false
            };
            bool_not(a110)
        }
    } else {
        false
    }
}

pub fn f14(v0: Adt3) -> bool {
    let a112 = match v0.clone() {
        Adt3::C0(_, h111, _, _, _, _, _, _, _) => {
            h111
        }
    };
    let a113 = 0u64;
    if nat_eq(a112, a113) {
        f31(match v0.clone() {
            Adt3::C0(h114, _, _, _, _, _, _, _, _) => {
                h114
            }
        })
    } else {
        false
    }
}

pub fn f15(v0: Adt3) -> bool {
    let a116 = match v0.clone() {
        Adt3::C0(_, _, h115, _, _, _, _, _, _) => {
            h115
        }
    };
    let a117 = 0u64;
    if nat_eq(a116, a117) {
        f32(match v0.clone() {
            Adt3::C0(h118, _, _, _, _, _, _, _, _) => {
                h118
            }
        })
    } else {
        false
    }
}

pub fn f16(v0: Adt3) -> bool {
    let a120 = match v0.clone() {
        Adt3::C0(_, _, _, h119, _, _, _, _, _) => {
            h119
        }
    };
    let a121 = 0u64;
    if nat_eq(a120, a121) {
        f33(match v0.clone() {
            Adt3::C0(h122, _, _, _, _, _, _, _, _) => {
                h122
            }
        })
    } else {
        false
    }
}

pub fn f17(v0: Adt3) -> bool {
    let a124 = match v0.clone() {
        Adt3::C0(_, _, _, _, h123, _, _, _, _) => {
            h123
        }
    };
    let a125 = 0u64;
    if nat_eq(a124, a125) {
        f34(match v0.clone() {
            Adt3::C0(h126, _, _, _, _, _, _, _, _) => {
                h126
            }
        })
    } else {
        false
    }
}

pub fn f18(v0: Adt3) -> R<bool> {
    let a128 = match v0.clone() {
        Adt3::C0(_, _, _, _, _, h127, _, _, _) => {
            h127
        }
    };
    let a129 = 0u64;
    if nat_eq(a128, a129) {
        let a140 = 2u64;
        let a141 = {
            let a138 = {
                let a135 = {
                    let a132 = match v0.clone() {
                        Adt3::C0(_, h130, _, _, _, _, _, _, _) => {
                            h130
                        }
                    };
                    let a133 = match v0.clone() {
                        Adt3::C0(_, _, h131, _, _, _, _, _, _) => {
                            h131
                        }
                    };
                    nat_add(a132, a133)?
                };
                let a136 = match v0.clone() {
                    Adt3::C0(_, _, _, h134, _, _, _, _, _) => {
                        h134
                    }
                };
                nat_add(a135, a136)?
            };
            let a139 = match v0.clone() {
                Adt3::C0(_, _, _, _, h137, _, _, _, _) => {
                    h137
                }
            };
            nat_add(a138, a139)?
        };
        Ok(nat_le(a140, a141))
    } else {
        Ok(false)
    }
}

pub fn f19(v0: Adt3) -> bool {
    let a143 = match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, h142, _, _) => {
            h142
        }
    };
    let a144 = 0u64;
    if nat_eq(a143, a144) {
        let a146 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, h145, _, _, _) => {
                h145
            }
        };
        let a147 = 1u64;
        if nat_eq(a146, a147) {
            let a151 = {
                let m150 = match v0.clone() {
                    Adt3::C0(h149, _, _, _, _, _, _, _, _) => {
                        h149
                    }
                };
                match m150 {
                    Adt0::C0(_, _, _, _, _, h148) => {
                        h148
                    }
                }
            };
            let a152 = 1u64;
            nat_eq(a151, a152)
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f20(v0: Adt3) -> bool {
    let a154 = match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, h153, _) => {
            h153
        }
    };
    let a155 = 0u64;
    if nat_eq(a154, a155) {
        let a157 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, h156, _, _) => {
                h156
            }
        };
        let a158 = 1u64;
        if nat_eq(a157, a158) {
            f30(match v0.clone() {
                Adt3::C0(h159, _, _, _, _, _, _, _, _) => {
                    h159
                }
            })
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f21(v0: Adt3) -> bool {
    let a161 = match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, h160) => {
            h160
        }
    };
    let a162 = 3u64;
    if nat_lt(a161, a162) {
        let a164 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, h163) => {
                h163
            }
        };
        let a165 = 0u64;
        if if nat_eq(a164, a165) {
            let a167 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, h166, _, _, _) => {
                    h166
                }
            };
            let a168 = 1u64;
            nat_eq(a167, a168)
        } else {
            false
        } {
            true
        } else {
            let a170 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, h169) => {
                    h169
                }
            };
            let a171 = 1u64;
            if if nat_eq(a170, a171) {
                let a173 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h172, _, _) => {
                        h172
                    }
                };
                let a174 = 1u64;
                nat_eq(a173, a174)
            } else {
                false
            } {
                true
            } else {
                let a176 = match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, _, _, h175) => {
                        h175
                    }
                };
                let a177 = 2u64;
                if nat_eq(a176, a177) {
                    let a179 = match v0.clone() {
                        Adt3::C0(_, _, _, _, _, _, _, h178, _) => {
                            h178
                        }
                    };
                    let a180 = 1u64;
                    nat_eq(a179, a180)
                } else {
                    false
                }
            }
        }
    } else {
        false
    }
}

pub fn f22(v0: Adt3) -> Option<Adt3> {
    if f14(v0.clone()) {
        Some(f35(v0.clone()))
    } else {
        None::<Adt3>
    }
}

pub fn f23(v0: Adt3) -> Option<Adt3> {
    if f15(v0.clone()) {
        Some(f36(v0.clone()))
    } else {
        None::<Adt3>
    }
}

pub fn f24(v0: Adt3) -> Option<Adt3> {
    if f16(v0.clone()) {
        Some(f37(v0.clone()))
    } else {
        None::<Adt3>
    }
}

pub fn f25(v0: Adt3) -> Option<Adt3> {
    if f17(v0.clone()) {
        Some(f38(v0.clone()))
    } else {
        None::<Adt3>
    }
}

pub fn f26(v0: Adt3) -> R<Option<Adt3>> {
    if f18(v0.clone())? {
        Ok(Some(f39(v0.clone())))
    } else {
        Ok(None::<Adt3>)
    }
}

pub fn f27(v0: Adt3) -> Option<Adt3> {
    if f19(v0.clone()) {
        Some(f40(v0.clone()))
    } else {
        None::<Adt3>
    }
}

pub fn f28(v0: Adt3) -> Option<Adt3> {
    if f20(v0.clone()) {
        Some(f41(v0.clone()))
    } else {
        None::<Adt3>
    }
}

pub fn f29(v0: Adt3) -> R<Option<Adt3>> {
    if f21(v0.clone()) {
        Ok(Some(f42(v0.clone())?))
    } else {
        Ok(None::<Adt3>)
    }
}

pub fn f30(v0: Adt0) -> bool {
    let a182 = match v0.clone() {
        Adt0::C0(_, _, _, _, h181, _) => {
            h181
        }
    };
    let a183 = 90u64;
    nat_lt(a182, a183)
}

pub fn f31(v0: Adt0) -> bool {
    let a185 = 380u64;
    let a186 = match v0.clone() {
        Adt0::C0(h184, _, _, _, _, _) => {
            h184
        }
    };
    nat_lt(a185, a186)
}

pub fn f32(v0: Adt0) -> bool {
    let a188 = 90u64;
    let a189 = match v0.clone() {
        Adt0::C0(_, h187, _, _, _, _) => {
            h187
        }
    };
    nat_lt(a188, a189)
}

pub fn f33(v0: Adt0) -> bool {
    let a191 = 20u64;
    let a192 = match v0.clone() {
        Adt0::C0(_, _, h190, _, _, _) => {
            h190
        }
    };
    nat_lt(a191, a192)
}

pub fn f34(v0: Adt0) -> bool {
    let a194 = 12u64;
    let a195 = match v0.clone() {
        Adt0::C0(_, _, _, h193, _, _) => {
            h193
        }
    };
    nat_lt(a194, a195)
}

pub fn f35(v0: Adt3) -> Adt3 {
    Adt3::C0(match v0.clone() {
        Adt3::C0(h196, _, _, _, _, _, _, _, _) => {
            h196
        }
    }, 1u64, match v0.clone() {
        Adt3::C0(_, _, h197, _, _, _, _, _, _) => {
            h197
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, h198, _, _, _, _, _) => {
            h198
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, h199, _, _, _, _) => {
            h199
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, h200, _, _, _) => {
            h200
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, h201, _, _) => {
            h201
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, h202, _) => {
            h202
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, h203) => {
            h203
        }
    })
}

pub fn f36(v0: Adt3) -> Adt3 {
    Adt3::C0(match v0.clone() {
        Adt3::C0(h204, _, _, _, _, _, _, _, _) => {
            h204
        }
    }, match v0.clone() {
        Adt3::C0(_, h205, _, _, _, _, _, _, _) => {
            h205
        }
    }, 1u64, match v0.clone() {
        Adt3::C0(_, _, _, h206, _, _, _, _, _) => {
            h206
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, h207, _, _, _, _) => {
            h207
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, h208, _, _, _) => {
            h208
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, h209, _, _) => {
            h209
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, h210, _) => {
            h210
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, h211) => {
            h211
        }
    })
}

pub fn f37(v0: Adt3) -> Adt3 {
    Adt3::C0(match v0.clone() {
        Adt3::C0(h212, _, _, _, _, _, _, _, _) => {
            h212
        }
    }, match v0.clone() {
        Adt3::C0(_, h213, _, _, _, _, _, _, _) => {
            h213
        }
    }, match v0.clone() {
        Adt3::C0(_, _, h214, _, _, _, _, _, _) => {
            h214
        }
    }, 1u64, match v0.clone() {
        Adt3::C0(_, _, _, _, h215, _, _, _, _) => {
            h215
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, h216, _, _, _) => {
            h216
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, h217, _, _) => {
            h217
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, h218, _) => {
            h218
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, h219) => {
            h219
        }
    })
}

pub fn f38(v0: Adt3) -> Adt3 {
    Adt3::C0(match v0.clone() {
        Adt3::C0(h220, _, _, _, _, _, _, _, _) => {
            h220
        }
    }, match v0.clone() {
        Adt3::C0(_, h221, _, _, _, _, _, _, _) => {
            h221
        }
    }, match v0.clone() {
        Adt3::C0(_, _, h222, _, _, _, _, _, _) => {
            h222
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, h223, _, _, _, _, _) => {
            h223
        }
    }, 1u64, match v0.clone() {
        Adt3::C0(_, _, _, _, _, h224, _, _, _) => {
            h224
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, h225, _, _) => {
            h225
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, h226, _) => {
            h226
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, h227) => {
            h227
        }
    })
}

pub fn f39(v0: Adt3) -> Adt3 {
    Adt3::C0(match v0.clone() {
        Adt3::C0(h228, _, _, _, _, _, _, _, _) => {
            h228
        }
    }, match v0.clone() {
        Adt3::C0(_, h229, _, _, _, _, _, _, _) => {
            h229
        }
    }, match v0.clone() {
        Adt3::C0(_, _, h230, _, _, _, _, _, _) => {
            h230
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, h231, _, _, _, _, _) => {
            h231
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, h232, _, _, _, _) => {
            h232
        }
    }, 1u64, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, h233, _, _) => {
            h233
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, h234, _) => {
            h234
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, h235) => {
            h235
        }
    })
}

pub fn f40(v0: Adt3) -> Adt3 {
    Adt3::C0(match v0.clone() {
        Adt3::C0(h236, _, _, _, _, _, _, _, _) => {
            h236
        }
    }, match v0.clone() {
        Adt3::C0(_, h237, _, _, _, _, _, _, _) => {
            h237
        }
    }, match v0.clone() {
        Adt3::C0(_, _, h238, _, _, _, _, _, _) => {
            h238
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, h239, _, _, _, _, _) => {
            h239
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, h240, _, _, _, _) => {
            h240
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, h241, _, _, _) => {
            h241
        }
    }, 1u64, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, h242, _) => {
            h242
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, h243) => {
            h243
        }
    })
}

pub fn f41(v0: Adt3) -> Adt3 {
    Adt3::C0(match v0.clone() {
        Adt3::C0(h244, _, _, _, _, _, _, _, _) => {
            h244
        }
    }, match v0.clone() {
        Adt3::C0(_, h245, _, _, _, _, _, _, _) => {
            h245
        }
    }, match v0.clone() {
        Adt3::C0(_, _, h246, _, _, _, _, _, _) => {
            h246
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, h247, _, _, _, _, _) => {
            h247
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, h248, _, _, _, _) => {
            h248
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, h249, _, _, _) => {
            h249
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, h250, _, _) => {
            h250
        }
    }, 1u64, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, h251) => {
            h251
        }
    })
}

pub fn f42(v0: Adt3) -> R<Adt3> {
    Ok(Adt3::C0(match v0.clone() {
        Adt3::C0(h252, _, _, _, _, _, _, _, _) => {
            h252
        }
    }, match v0.clone() {
        Adt3::C0(_, h253, _, _, _, _, _, _, _) => {
            h253
        }
    }, match v0.clone() {
        Adt3::C0(_, _, h254, _, _, _, _, _, _) => {
            h254
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, h255, _, _, _, _, _) => {
            h255
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, h256, _, _, _, _) => {
            h256
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, h257, _, _, _) => {
            h257
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, h258, _, _) => {
            h258
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, h259, _) => {
            h259
        }
    }, {
        let a261 = match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, _, h260) => {
                h260
            }
        };
        let a262 = 1u64;
        nat_add(a261, a262)?
    }))
}
