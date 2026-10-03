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
    C0(Adt0, u64, u64, u64, u64, u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt2 {
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
pub enum Adt3 {
    C0(u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt4 {
    C0(Adt1, List<Adt2>, Adt3),
}

#[derive(Clone)]
pub enum Fn0 {
    F2,
}

impl Fn0 {
    pub fn apply(&self, p0: Adt4) -> R<Option<Adt4>> {
        match self {
            Fn0::F2 => f2(p0),
        }
    }
}

pub fn f0(v0: Adt0) -> R<(Result<(u64, List<Adt2>), (bool, bool)>, (u64, u64))> {
    let v1: (Adt4, bool) = f40(Fn0::F2, 11u64, f1(v0.clone()))?;
    let v2: Adt4 = {
        let (h1, _) = v1.clone();
        h1
    };
    Ok(({
        let (_, h2) = v1.clone();
        if h2 {
            let m4 = f38(v0.clone(), match v2.clone() {
                Adt4::C0(h3, _, _) => {
                    h3
                }
            });
            match m4 {
                Ok(v3) => {
                    Ok::<(u64, List<Adt2>), (bool, bool)>((v3, match v2.clone() {
                        Adt4::C0(_, h5, _) => {
                            h5
                        }
                    }))
                }
                Err(v4) => {
                    Err::<(u64, List<Adt2>), (bool, bool)>(v4)
                }
            }
        } else {
            Err::<(u64, List<Adt2>), (bool, bool)>((false, false))
        }
    }, ({
        let m8 = match v2.clone() {
            Adt4::C0(_, _, h7) => {
                h7
            }
        };
        match m8 {
            Adt3::C0(h6, _, _) => {
                h6
            }
        }
    }, {
        let m11 = match v2.clone() {
            Adt4::C0(_, _, h10) => {
                h10
            }
        };
        match m11 {
            Adt3::C0(_, h9, _) => {
                h9
            }
        }
    })))
}

pub fn f1(v0: Adt0) -> Adt4 {
    Adt4::C0(f32(v0.clone()), List::<Adt2>::nil(), Adt3::C0(0u64, 0u64, 0u64))
}

pub fn f2(v0: Adt4) -> R<Option<Adt4>> {
    let m13 = f33(match v0.clone() {
        Adt4::C0(h12, _, _) => {
            h12
        }
    })?;
    match m13 {
        None => {
            Ok(None::<Adt4>)
        }
        Some(v1) => {
            let m15 = f34(match v0.clone() {
                Adt4::C0(h14, _, _) => {
                    h14
                }
            }, v1.clone())?;
            match m15 {
                None => {
                    Ok(None::<Adt4>)
                }
                Some(v2) => {
                    Ok(Some(Adt4::C0(v2.clone(), {
                        let a17 = match v0.clone() {
                            Adt4::C0(_, h16, _) => {
                                h16
                            }
                        };
                        let a18 = List::cons(v1.clone(), List::<Adt2>::nil());
                        append_list(a17, a18)
                    }, Adt3::C0({
                        let a22 = {
                            let m21 = match v0.clone() {
                                Adt4::C0(_, _, h20) => {
                                    h20
                                }
                            };
                            match m21 {
                                Adt3::C0(h19, _, _) => {
                                    h19
                                }
                            }
                        };
                        let a23 = 1u64;
                        nat_add(a22, a23)?
                    }, {
                        let a27 = {
                            let m26 = match v0.clone() {
                                Adt4::C0(_, _, h25) => {
                                    h25
                                }
                            };
                            match m26 {
                                Adt3::C0(_, h24, _) => {
                                    h24
                                }
                            }
                        };
                        let a28 = 1u64;
                        nat_add(a27, a28)?
                    }, {
                        let m31 = match v0.clone() {
                            Adt4::C0(_, _, h30) => {
                                h30
                            }
                        };
                        match m31 {
                            Adt3::C0(_, _, h29) => {
                                h29
                            }
                        }
                    }))))
                }
            }
        }
    }
}

pub fn f3(v0: Adt0) -> bool {
    let a33 = 380u64;
    let a34 = match v0.clone() {
        Adt0::C0(h32, _, _, _, _, _) => {
            h32
        }
    };
    nat_lt(a33, a34)
}

pub fn f4(v0: Adt0) -> bool {
    let a36 = 90u64;
    let a37 = match v0.clone() {
        Adt0::C0(_, h35, _, _, _, _) => {
            h35
        }
    };
    nat_lt(a36, a37)
}

pub fn f5(v0: Adt0) -> bool {
    let a39 = 20u64;
    let a40 = match v0.clone() {
        Adt0::C0(_, _, h38, _, _, _) => {
            h38
        }
    };
    nat_lt(a39, a40)
}

pub fn f6(v0: Adt0) -> bool {
    let a42 = 12u64;
    let a43 = match v0.clone() {
        Adt0::C0(_, _, _, h41, _, _) => {
            h41
        }
    };
    nat_lt(a42, a43)
}

pub fn f7(v0: Adt0) -> bool {
    let a45 = match v0.clone() {
        Adt0::C0(_, _, _, _, h44, _) => {
            h44
        }
    };
    let a46 = 90u64;
    nat_lt(a45, a46)
}

pub fn f8(v0: Adt1) -> bool {
    let a51 = {
        let a48 = match v0.clone() {
            Adt1::C0(_, h47, _, _, _, _, _, _, _) => {
                h47
            }
        };
        let a49 = 0u64;
        nat_eq(a48, a49)
    };
    let a52 = f3(match v0.clone() {
        Adt1::C0(h50, _, _, _, _, _, _, _, _) => {
            h50
        }
    });
    bool_and(a51, a52)
}

pub fn f9(v0: Adt1) -> bool {
    let a57 = {
        let a54 = match v0.clone() {
            Adt1::C0(_, _, h53, _, _, _, _, _, _) => {
                h53
            }
        };
        let a55 = 0u64;
        nat_eq(a54, a55)
    };
    let a58 = f4(match v0.clone() {
        Adt1::C0(h56, _, _, _, _, _, _, _, _) => {
            h56
        }
    });
    bool_and(a57, a58)
}

pub fn f10(v0: Adt1) -> bool {
    let a63 = {
        let a60 = match v0.clone() {
            Adt1::C0(_, _, _, h59, _, _, _, _, _) => {
                h59
            }
        };
        let a61 = 0u64;
        nat_eq(a60, a61)
    };
    let a64 = f5(match v0.clone() {
        Adt1::C0(h62, _, _, _, _, _, _, _, _) => {
            h62
        }
    });
    bool_and(a63, a64)
}

pub fn f11(v0: Adt1) -> bool {
    let a69 = {
        let a66 = match v0.clone() {
            Adt1::C0(_, _, _, _, h65, _, _, _, _) => {
                h65
            }
        };
        let a67 = 0u64;
        nat_eq(a66, a67)
    };
    let a70 = f6(match v0.clone() {
        Adt1::C0(h68, _, _, _, _, _, _, _, _) => {
            h68
        }
    });
    bool_and(a69, a70)
}

pub fn f12(v0: Adt1) -> R<bool> {
    let a86 = {
        let a72 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h71, _, _, _) => {
                h71
            }
        };
        let a73 = 0u64;
        nat_eq(a72, a73)
    };
    let a87 = {
        let a84 = 2u64;
        let a85 = {
            let a82 = {
                let a79 = {
                    let a76 = match v0.clone() {
                        Adt1::C0(_, h74, _, _, _, _, _, _, _) => {
                            h74
                        }
                    };
                    let a77 = match v0.clone() {
                        Adt1::C0(_, _, h75, _, _, _, _, _, _) => {
                            h75
                        }
                    };
                    nat_add(a76, a77)?
                };
                let a80 = match v0.clone() {
                    Adt1::C0(_, _, _, h78, _, _, _, _, _) => {
                        h78
                    }
                };
                nat_add(a79, a80)?
            };
            let a83 = match v0.clone() {
                Adt1::C0(_, _, _, _, h81, _, _, _, _) => {
                    h81
                }
            };
            nat_add(a82, a83)?
        };
        nat_le(a84, a85)
    };
    Ok(bool_and(a86, a87))
}

pub fn f13(v0: Adt1) -> bool {
    let a101 = {
        let a89 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h88, _, _) => {
                h88
            }
        };
        let a90 = 0u64;
        nat_eq(a89, a90)
    };
    let a102 = {
        let a99 = {
            let a92 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, h91, _, _, _) => {
                    h91
                }
            };
            let a93 = 1u64;
            nat_eq(a92, a93)
        };
        let a100 = {
            let a97 = {
                let m96 = match v0.clone() {
                    Adt1::C0(h95, _, _, _, _, _, _, _, _) => {
                        h95
                    }
                };
                match m96 {
                    Adt0::C0(_, _, _, _, _, h94) => {
                        h94
                    }
                }
            };
            let a98 = 1u64;
            nat_eq(a97, a98)
        };
        bool_and(a99, a100)
    };
    bool_and(a101, a102)
}

pub fn f14(v0: Adt1) -> bool {
    let a112 = {
        let a104 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h103, _) => {
                h103
            }
        };
        let a105 = 0u64;
        nat_eq(a104, a105)
    };
    let a113 = {
        let a110 = {
            let a107 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h106, _, _) => {
                    h106
                }
            };
            let a108 = 1u64;
            nat_eq(a107, a108)
        };
        let a111 = f7(match v0.clone() {
            Adt1::C0(h109, _, _, _, _, _, _, _, _) => {
                h109
            }
        });
        bool_and(a110, a111)
    };
    bool_and(a112, a113)
}

pub fn f15(v0: Adt1) -> bool {
    let a145 = {
        let a115 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h114) => {
                h114
            }
        };
        let a116 = 3u64;
        nat_lt(a115, a116)
    };
    let a146 = {
        let a143 = {
            let a123 = {
                let a118 = match v0.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h117) => {
                        h117
                    }
                };
                let a119 = 0u64;
                nat_eq(a118, a119)
            };
            let a124 = {
                let a121 = match v0.clone() {
                    Adt1::C0(_, _, _, _, _, h120, _, _, _) => {
                        h120
                    }
                };
                let a122 = 1u64;
                nat_eq(a121, a122)
            };
            bool_and(a123, a124)
        };
        let a144 = {
            let a141 = {
                let a131 = {
                    let a126 = match v0.clone() {
                        Adt1::C0(_, _, _, _, _, _, _, _, h125) => {
                            h125
                        }
                    };
                    let a127 = 1u64;
                    nat_eq(a126, a127)
                };
                let a132 = {
                    let a129 = match v0.clone() {
                        Adt1::C0(_, _, _, _, _, _, h128, _, _) => {
                            h128
                        }
                    };
                    let a130 = 1u64;
                    nat_eq(a129, a130)
                };
                bool_and(a131, a132)
            };
            let a142 = {
                let a139 = {
                    let a134 = match v0.clone() {
                        Adt1::C0(_, _, _, _, _, _, _, _, h133) => {
                            h133
                        }
                    };
                    let a135 = 2u64;
                    nat_eq(a134, a135)
                };
                let a140 = {
                    let a137 = match v0.clone() {
                        Adt1::C0(_, _, _, _, _, _, _, h136, _) => {
                            h136
                        }
                    };
                    let a138 = 1u64;
                    nat_eq(a137, a138)
                };
                bool_and(a139, a140)
            };
            bool_or(a141, a142)
        };
        bool_or(a143, a144)
    };
    bool_and(a145, a146)
}

pub fn f16(v0: Adt1) -> Adt1 {
    Adt1::C0(match v0.clone() {
        Adt1::C0(h147, _, _, _, _, _, _, _, _) => {
            h147
        }
    }, 1u64, match v0.clone() {
        Adt1::C0(_, _, h148, _, _, _, _, _, _) => {
            h148
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h149, _, _, _, _, _) => {
            h149
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h150, _, _, _, _) => {
            h150
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, h151, _, _, _) => {
            h151
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h152, _, _) => {
            h152
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h153, _) => {
            h153
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, h154) => {
            h154
        }
    })
}

pub fn f17(v0: Adt1) -> Adt1 {
    Adt1::C0(match v0.clone() {
        Adt1::C0(h155, _, _, _, _, _, _, _, _) => {
            h155
        }
    }, match v0.clone() {
        Adt1::C0(_, h156, _, _, _, _, _, _, _) => {
            h156
        }
    }, 1u64, match v0.clone() {
        Adt1::C0(_, _, _, h157, _, _, _, _, _) => {
            h157
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h158, _, _, _, _) => {
            h158
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, h159, _, _, _) => {
            h159
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h160, _, _) => {
            h160
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h161, _) => {
            h161
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, h162) => {
            h162
        }
    })
}

pub fn f18(v0: Adt1) -> Adt1 {
    Adt1::C0(match v0.clone() {
        Adt1::C0(h163, _, _, _, _, _, _, _, _) => {
            h163
        }
    }, match v0.clone() {
        Adt1::C0(_, h164, _, _, _, _, _, _, _) => {
            h164
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h165, _, _, _, _, _, _) => {
            h165
        }
    }, 1u64, match v0.clone() {
        Adt1::C0(_, _, _, _, h166, _, _, _, _) => {
            h166
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, h167, _, _, _) => {
            h167
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h168, _, _) => {
            h168
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h169, _) => {
            h169
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, h170) => {
            h170
        }
    })
}

pub fn f19(v0: Adt1) -> Adt1 {
    Adt1::C0(match v0.clone() {
        Adt1::C0(h171, _, _, _, _, _, _, _, _) => {
            h171
        }
    }, match v0.clone() {
        Adt1::C0(_, h172, _, _, _, _, _, _, _) => {
            h172
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h173, _, _, _, _, _, _) => {
            h173
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h174, _, _, _, _, _) => {
            h174
        }
    }, 1u64, match v0.clone() {
        Adt1::C0(_, _, _, _, _, h175, _, _, _) => {
            h175
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h176, _, _) => {
            h176
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h177, _) => {
            h177
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, h178) => {
            h178
        }
    })
}

pub fn f20(v0: Adt1) -> Adt1 {
    Adt1::C0(match v0.clone() {
        Adt1::C0(h179, _, _, _, _, _, _, _, _) => {
            h179
        }
    }, match v0.clone() {
        Adt1::C0(_, h180, _, _, _, _, _, _, _) => {
            h180
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h181, _, _, _, _, _, _) => {
            h181
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h182, _, _, _, _, _) => {
            h182
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h183, _, _, _, _) => {
            h183
        }
    }, 1u64, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h184, _, _) => {
            h184
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h185, _) => {
            h185
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, h186) => {
            h186
        }
    })
}

pub fn f21(v0: Adt1) -> Adt1 {
    Adt1::C0(match v0.clone() {
        Adt1::C0(h187, _, _, _, _, _, _, _, _) => {
            h187
        }
    }, match v0.clone() {
        Adt1::C0(_, h188, _, _, _, _, _, _, _) => {
            h188
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h189, _, _, _, _, _, _) => {
            h189
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h190, _, _, _, _, _) => {
            h190
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h191, _, _, _, _) => {
            h191
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, h192, _, _, _) => {
            h192
        }
    }, 1u64, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h193, _) => {
            h193
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, h194) => {
            h194
        }
    })
}

pub fn f22(v0: Adt1) -> Adt1 {
    Adt1::C0(match v0.clone() {
        Adt1::C0(h195, _, _, _, _, _, _, _, _) => {
            h195
        }
    }, match v0.clone() {
        Adt1::C0(_, h196, _, _, _, _, _, _, _) => {
            h196
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h197, _, _, _, _, _, _) => {
            h197
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h198, _, _, _, _, _) => {
            h198
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h199, _, _, _, _) => {
            h199
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, h200, _, _, _) => {
            h200
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h201, _, _) => {
            h201
        }
    }, 1u64, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, h202) => {
            h202
        }
    })
}

pub fn f23(v0: Adt1) -> R<Adt1> {
    Ok(Adt1::C0(match v0.clone() {
        Adt1::C0(h203, _, _, _, _, _, _, _, _) => {
            h203
        }
    }, match v0.clone() {
        Adt1::C0(_, h204, _, _, _, _, _, _, _) => {
            h204
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h205, _, _, _, _, _, _) => {
            h205
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h206, _, _, _, _, _) => {
            h206
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h207, _, _, _, _) => {
            h207
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, h208, _, _, _) => {
            h208
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h209, _, _) => {
            h209
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h210, _) => {
            h210
        }
    }, {
        let a212 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h211) => {
                h211
            }
        };
        let a213 = 1u64;
        nat_add(a212, a213)?
    }))
}

pub fn f24(v0: Adt1) -> Option<Adt1> {
    if f8(v0.clone()) {
        Some(f16(v0.clone()))
    } else {
        None::<Adt1>
    }
}

pub fn f25(v0: Adt1) -> Option<Adt1> {
    if f9(v0.clone()) {
        Some(f17(v0.clone()))
    } else {
        None::<Adt1>
    }
}

pub fn f26(v0: Adt1) -> Option<Adt1> {
    if f10(v0.clone()) {
        Some(f18(v0.clone()))
    } else {
        None::<Adt1>
    }
}

pub fn f27(v0: Adt1) -> Option<Adt1> {
    if f11(v0.clone()) {
        Some(f19(v0.clone()))
    } else {
        None::<Adt1>
    }
}

pub fn f28(v0: Adt1) -> R<Option<Adt1>> {
    if f12(v0.clone())? {
        Ok(Some(f20(v0.clone())))
    } else {
        Ok(None::<Adt1>)
    }
}

pub fn f29(v0: Adt1) -> Option<Adt1> {
    if f13(v0.clone()) {
        Some(f21(v0.clone()))
    } else {
        None::<Adt1>
    }
}

pub fn f30(v0: Adt1) -> Option<Adt1> {
    if f14(v0.clone()) {
        Some(f22(v0.clone()))
    } else {
        None::<Adt1>
    }
}

pub fn f31(v0: Adt1) -> R<Option<Adt1>> {
    if f15(v0.clone()) {
        Ok(Some(f23(v0.clone())?))
    } else {
        Ok(None::<Adt1>)
    }
}

pub fn f32(v0: Adt0) -> Adt1 {
    Adt1::C0(v0.clone(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64)
}

pub fn f33(v0: Adt1) -> R<Option<Adt2>> {
    if f8(v0.clone()) {
        Ok(Some(Adt2::C0))
    } else {
        if f9(v0.clone()) {
            Ok(Some(Adt2::C1))
        } else {
            if f10(v0.clone()) {
                Ok(Some(Adt2::C2))
            } else {
                if f11(v0.clone()) {
                    Ok(Some(Adt2::C3))
                } else {
                    if f12(v0.clone())? {
                        Ok(Some(Adt2::C4))
                    } else {
                        if f13(v0.clone()) {
                            Ok(Some(Adt2::C5))
                        } else {
                            if f14(v0.clone()) {
                                Ok(Some(Adt2::C6))
                            } else {
                                if f15(v0.clone()) {
                                    Ok(Some(Adt2::C7))
                                } else {
                                    Ok(None::<Adt2>)
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f34(v0: Adt1, v1: Adt2) -> R<Option<Adt1>> {
    let m214 = v1.clone();
    match m214 {
        Adt2::C0 => {
            Ok(f24(v0.clone()))
        }
        Adt2::C1 => {
            Ok(f25(v0.clone()))
        }
        Adt2::C2 => {
            Ok(f26(v0.clone()))
        }
        Adt2::C3 => {
            Ok(f27(v0.clone()))
        }
        Adt2::C4 => {
            f28(v0.clone())
        }
        Adt2::C5 => {
            Ok(f29(v0.clone()))
        }
        Adt2::C6 => {
            Ok(f30(v0.clone()))
        }
        Adt2::C7 => {
            f31(v0.clone())
        }
    }
}

pub fn f35(v0: Adt1) -> R<Option<Adt1>> {
    let m215 = f33(v0.clone())?;
    match m215 {
        None => {
            Ok(None::<Adt1>)
        }
        Some(v1) => {
            f34(v0.clone(), v1.clone())
        }
    }
}

pub fn f36(v0: Adt1) -> Option<u64> {
    Some(match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, h216) => {
            h216
        }
    })
}

pub fn f37(v0: Adt0, v1: Adt1) -> Option<u64> {
    let m217 = f36(v1.clone());
    match m217 {
        None => {
            None::<u64>
        }
        Some(v2) => {
            if f39(v0.clone(), v2) {
                Some(v2)
            } else {
                None::<u64>
            }
        }
    }
}

pub fn f38(v0: Adt0, v1: Adt1) -> Result<u64, (bool, bool)> {
    let m218 = f37(v0.clone(), v1.clone());
    match m218 {
        None => {
            let m219 = f36(v1.clone());
            match m219 {
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

pub fn f39(v0: Adt0, v1: u64) -> bool {
    let a236 = {
        let a220 = v1;
        let a221 = 3u64;
        nat_le(a220, a221)
    };
    let a237 = {
        let a234 = {
            let a222 = v1;
            let a223 = 3u64;
            nat_eq(a222, a223)
        };
        let a235 = {
            let a233 = {
                let a231 = f7(v0.clone());
                let a232 = {
                    let a229 = {
                        let a225 = match v0.clone() {
                            Adt0::C0(_, _, _, _, _, h224) => {
                                h224
                            }
                        };
                        let a226 = 1u64;
                        nat_eq(a225, a226)
                    };
                    let a230 = {
                        let a227 = f3(v0.clone());
                        let a228 = f4(v0.clone());
                        bool_and(a227, a228)
                    };
                    bool_and(a229, a230)
                };
                bool_and(a231, a232)
            };
            bool_not(a233)
        };
        bool_or(a234, a235)
    };
    bool_and(a236, a237)
}

pub fn f40(v0: Fn0, v1: u64, v2: Adt4) -> R<(Adt4, bool)> {
    let m238 = v1;
    if m238 == 0 {
        Ok((v2.clone(), false))
    } else {
        let v3 = m238 - 1;
        let m240 = {
            let c239: Fn0 = v0.clone();
            c239.apply(v2.clone())?
        };
        match m240 {
            None => {
                Ok((v2.clone(), true))
            }
            Some(v4) => {
                f40(v0.clone(), v3, v4.clone())
            }
        }
    }
}

pub fn run(p0: Adt0) -> R<(Result<(u64, List<Adt2>), (bool, bool)>, (u64, u64))> {
    f0(p0)
}
