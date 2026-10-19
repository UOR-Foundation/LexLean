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
/// A list is a run of cells laid out contiguously, from the newest at the end of a buffer
/// to the oldest at its start, or a cell in front of another list. A `cons` onto a run that
/// owns its buffer alone pushes in place; a `cons` onto any other list is one cell, so
/// building and taking apart a cell is constant work whatever the sharing, and no buffer is
/// written while another list can see it.
pub enum Repr<T> { Nil, Run { buffer: std::rc::Rc<Vec<T>>, len: usize }, Cell(std::rc::Rc<Node<T>>) }
pub struct List<T>(Repr<T>);
impl<T> Clone for List<T> {
    fn clone(&self) -> Self {
        List(match &self.0 {
            Repr::Nil => Repr::Nil,
            Repr::Run { buffer, len } => Repr::Run { buffer: buffer.clone(), len: *len },
            Repr::Cell(cell) => Repr::Cell(cell.clone()),
        })
    }
}
impl<T> List<T> {
    fn take(&mut self) -> Repr<T> { std::mem::replace(&mut self.0, Repr::Nil) }
}
impl<T> Drop for Node<T> {
    fn drop(&mut self) {
        let mut next = match self.tail.take() { Repr::Cell(cell) => Some(cell), _ => None };
        while let Some(cell) = next { // release
            match std::rc::Rc::try_unwrap(cell) {
                Ok(mut node) => next = match node.tail.take() { Repr::Cell(cell) => Some(cell), _ => None },
                Err(_) => break,
            }
        }
    }
}
impl<T: Clone> List<T> {
    pub fn nil() -> Self { List(Repr::Nil) }
    pub fn cons(head: T, mut tail: List<T>) -> Self {
        List(match tail.take() {
            Repr::Nil => {
                let mut buffer = Vec::with_capacity(16);
                buffer.push(head);
                Repr::Run { buffer: std::rc::Rc::new(buffer), len: 1 }
            }
            Repr::Run { mut buffer, len } => {
                if let Some(cells) = std::rc::Rc::get_mut(&mut buffer) {
                    cells.truncate(len);
                    cells.push(head);
                    Repr::Run { buffer, len: len + 1 }
                } else {
                    Repr::Cell(std::rc::Rc::new(Node { head, tail: List(Repr::Run { buffer, len }) }))
                }
            }
            rest => Repr::Cell(std::rc::Rc::new(Node { head, tail: List(rest) })),
        })
    }
    pub fn uncons(&self) -> Option<(T, List<T>)> {
        match &self.0 {
            Repr::Nil => None,
            Repr::Run { buffer, len } => {
                let head = buffer[len - 1].clone();
                Some((head, List(if *len == 1 { Repr::Nil } else { Repr::Run { buffer: buffer.clone(), len: len - 1 } })))
            }
            Repr::Cell(cell) => Some((cell.head.clone(), cell.tail.clone())),
        }
    }
    /// Walk the items in order, ticking one per item, until `each` says to stop: the cells
    /// in front, then the run they end in.
    fn walk(&self, each: &mut dyn FnMut(&T) -> bool) {
        let mut cursor = self;
        while let Repr::Cell(cell) = &cursor.0 { tick(1); if !each(&cell.head) { return; } cursor = &cell.tail; }
        if let Repr::Run { buffer, len } = &cursor.0 {
            for item in buffer[..*len].iter().rev() { tick(1); if !each(item) { return; } }
        }
    }
    fn items(&self) -> Vec<T> {
        let mut out = Vec::new();
        self.walk(&mut |item| { out.push(item.clone()); true });
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
pub fn length_list<T: Clone>(a: List<T>) -> u64 {
    let mut count = 0;
    a.walk(&mut |_| { count += 1; true });
    count
}
pub fn length_bytes(a: Bytes) -> u64 { a.0.len() as u64 }
pub fn length_string(a: Str) -> u64 { chars(&a.0) }
pub fn index_list<T: Clone>(a: List<T>, i: u64) -> Option<T> {
    let mut position = 0;
    let mut found = None;
    a.walk(&mut |item| { if position == i { found = Some(item.clone()); return false; } position += 1; true });
    found
}
pub fn index_bytes(a: Bytes, i: u64) -> Option<u8> { usize::try_from(i).ok().and_then(|i| a.0.get(i)).copied() }
pub fn slice_list<T: Clone>(a: List<T>, start: u64, count: u64) -> Option<List<T>> {
    let end = u128::from(start) + u128::from(count);
    let mut part = Vec::new();
    let mut position: u128 = 0;
    a.walk(&mut |item| {
        if position >= end { return false; }
        if position >= u128::from(start) { part.push(item.clone()); }
        position += 1;
        true
    });
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
    let mut first = true;
    texts.walk(&mut |text| {
        if !first { chars(&delimiter.0); out.push_str(&delimiter.0); }
        first = false;
        chars(&text.0);
        out.push_str(&text.0);
        true
    });
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
    C2,
}

#[derive(Clone)]
pub enum Adt1 {
    C0,
    C1(u64),
}

#[derive(Clone)]
pub enum Adt2 {
    C0((Adt0, u64), List<Adt1>, Adt3),
}

#[derive(Clone)]
pub enum Adt3 {
    C0(u64, u64, u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Fn0 {
    F8,
}

impl Fn0 {
    pub fn apply(&self, p0: Adt2) -> R<Option<Adt2>> {
        match self {
            Fn0::F8 => f8(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F19((Adt0, u64)),
}

impl Fn1 {
    pub fn apply(&self, p0: Option<Adt1>, p1: u64) -> Option<Adt1> {
        match self {
            Fn1::F19(k0) => f19(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F24((Adt0, u64)),
}

impl Fn2 {
    pub fn apply(&self, p0: (bool, u64), p1: u64) -> R<(bool, u64)> {
        match self {
            Fn2::F24(k0) => f24(k0.clone(), p0, p1),
        }
    }
}

pub fn f0(v0: u64) -> R<u64> {
    f1(f2(v0))
}

pub fn f1(v0: Adt0) -> R<u64> {
    let m1 = f3((v0.clone(), f4(v0)))?;
    match m1 {
        Ok(v1) => {
            let a3 = {
                let (_, h2) = v1;
                h2
            };
            Ok(length_list(a3))
        }
        Err(_) => {
            Ok(0u64)
        }
    }
}

pub fn f2(v0: u64) -> Adt0 {
    let a4 = v0;
    let a5 = 0u64;
    if nat_eq(a4, a5) {
        Adt0::C0
    } else {
        let a6 = v0;
        let a7 = 1u64;
        if nat_eq(a6, a7) {
            Adt0::C1
        } else {
            Adt0::C2
        }
    }
}

pub fn f3(v0: (Adt0, u64)) -> R<Result<(u64, List<Adt1>), (bool, bool)>> {
    let v1: (Adt2, bool) = f5(v0.clone())?;
    let m9 = {
        let (_, h8) = v1.clone();
        h8
    };
    if m9 {
        let m13 = f6(v0, {
            let m12 = {
                let (h11, _) = v1.clone();
                h11
            };
            match m12 {
                Adt2::C0(h10, _, _) => {
                    h10
                }
            }
        });
        match m13 {
            Ok(v2) => {
                Ok(Ok::<(u64, List<Adt1>), (bool, bool)>((v2, {
                    let m16 = {
                        let (h15, _) = v1;
                        h15
                    };
                    match m16 {
                        Adt2::C0(_, h14, _) => {
                            h14
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

pub fn f4(v0: Adt0) -> u64 {
    let m17 = v0;
    match m17 {
        Adt0::C0 => {
            3u64
        }
        Adt0::C1 => {
            2u64
        }
        Adt0::C2 => {
            1u64
        }
    }
}

pub fn f5(v0: (Adt0, u64)) -> R<(Adt2, bool)> {
    f7(Fn0::F8, {
        let a19 = {
            let (_, h18) = v0.clone();
            h18
        };
        let a20 = 1u64;
        nat_add(a19, a20)?
    }, f9(v0))
}

pub fn f6(v0: (Adt0, u64), v1: (Adt0, u64)) -> Result<u64, (bool, bool)> {
    let m21 = f10(v0, v1.clone());
    match m21 {
        None => {
            let m22 = f11(v1);
            match m22 {
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

pub fn f7(v0: Fn0, v1: u64, v2: Adt2) -> R<(Adt2, bool)> {
    let m23 = v1;
    if m23 == 0 {
        Ok((v2, false))
    } else {
        let v3 = m23 - 1;
        let m25 = {
            let c24: Fn0 = v0.clone();
            c24.apply(v2.clone())?
        };
        match m25 {
            None => {
                Ok((v2, true))
            }
            Some(v4) => {
                f7(v0, v3, v4)
            }
        }
    }
}

pub fn f8(v0: Adt2) -> R<Option<Adt2>> {
    let m27 = f12(match v0.clone() {
        Adt2::C0(h26, _, _) => {
            h26
        }
    });
    match m27 {
        None => {
            Ok(None::<Adt2>)
        }
        Some(v1) => {
            let m29 = f13(match v0.clone() {
                Adt2::C0(h28, _, _) => {
                    h28
                }
            }, v1.clone());
            match m29 {
                None => {
                    Ok(None::<Adt2>)
                }
                Some(v2) => {
                    Ok(Some(Adt2::C0(v2, {
                        let a31 = match v0.clone() {
                            Adt2::C0(_, h30, _) => {
                                h30
                            }
                        };
                        let a32 = List::cons(v1, List::<Adt1>::nil());
                        append_list(a31, a32)
                    }, Adt3::C0({
                        let a36 = {
                            let m35 = match v0.clone() {
                                Adt2::C0(_, _, h34) => {
                                    h34
                                }
                            };
                            match m35 {
                                Adt3::C0(h33, _, _, _, _, _) => {
                                    h33
                                }
                            }
                        };
                        let a37 = 1u64;
                        nat_add(a36, a37)?
                    }, {
                        let a42 = {
                            let m40 = match v0.clone() {
                                Adt2::C0(_, _, h39) => {
                                    h39
                                }
                            };
                            match m40 {
                                Adt3::C0(_, h38, _, _, _, _) => {
                                    h38
                                }
                            }
                        };
                        let a43 = f14(match v0.clone() {
                            Adt2::C0(h41, _, _) => {
                                h41
                            }
                        })?;
                        nat_add(a42, a43)?
                    }, {
                        let a47 = {
                            let m46 = match v0.clone() {
                                Adt2::C0(_, _, h45) => {
                                    h45
                                }
                            };
                            match m46 {
                                Adt3::C0(_, _, h44, _, _, _) => {
                                    h44
                                }
                            }
                        };
                        let a48 = 1u64;
                        nat_add(a47, a48)?
                    }, {
                        let m51 = match v0.clone() {
                            Adt2::C0(_, _, h50) => {
                                h50
                            }
                        };
                        match m51 {
                            Adt3::C0(_, _, _, h49, _, _) => {
                                h49
                            }
                        }
                    }, {
                        let m54 = match v0.clone() {
                            Adt2::C0(_, _, h53) => {
                                h53
                            }
                        };
                        match m54 {
                            Adt3::C0(_, _, _, _, h52, _) => {
                                h52
                            }
                        }
                    }, {
                        let m57 = match v0 {
                            Adt2::C0(_, _, h56) => {
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

pub fn f9(v0: (Adt0, u64)) -> Adt2 {
    Adt2::C0(f15(v0), List::<Adt1>::nil(), Adt3::C0(0u64, 0u64, 0u64, 0u64, 0u64, 0u64))
}

pub fn f10(v0: (Adt0, u64), v1: (Adt0, u64)) -> Option<u64> {
    let m58 = f11(v1);
    match m58 {
        None => {
            None::<u64>
        }
        Some(v2) => {
            if f16(v0, v2) {
                Some(v2)
            } else {
                None::<u64>
            }
        }
    }
}

pub fn f11(v0: (Adt0, u64)) -> Option<u64> {
    Some({
        let (_, h59) = v0;
        h59
    })
}

pub fn f12(v0: (Adt0, u64)) -> Option<Adt1> {
    let m60 = if f17(v0.clone()) {
        Some(Adt1::C0)
    } else {
        None::<Adt1>
    };
    match m60 {
        None => {
            f18(Fn1::F19(v0.clone()), None::<Adt1>, f20(v0))
        }
        Some(v1) => {
            Some(v1)
        }
    }
}

pub fn f13(v0: (Adt0, u64), v1: Adt1) -> Option<(Adt0, u64)> {
    let m61 = v1;
    match m61 {
        Adt1::C0 => {
            f21(v0)
        }
        Adt1::C1(v2) => {
            f22(v0, v2)
        }
    }
}

pub fn f14(v0: (Adt0, u64)) -> R<u64> {
    let (_, h62) = {
        let v1: (bool, u64) = (f17(v0.clone()), 1u64);
        let (h63, _) = v1;
        if h63 {
            v1
        } else {
            let v2: (bool, u64) = f23(Fn2::F24(v0.clone()), (false, 0u64), f20(v0))?;
            ({
                let (h64, _) = v2;
                h64
            }, {
                let a67 = {
                    let (_, h65) = v1;
                    h65
                };
                let a68 = {
                    let (_, h66) = v2;
                    h66
                };
                nat_add(a67, a68)?
            })
        }
    };
    Ok(h62)
}

pub fn f15(v0: (Adt0, u64)) -> (Adt0, u64) {
    v0
}

pub fn f16(_: (Adt0, u64), v1: u64) -> bool {
    let a69 = v1;
    let a70 = 0u64;
    nat_eq(a69, a70)
}

pub fn f17(v0: (Adt0, u64)) -> bool {
    let a72 = 2u64;
    let a73 = {
        let (_, h71) = v0;
        h71
    };
    nat_lt(a72, a73)
}

pub fn f18(v0: Fn1, v1: Option<Adt1>, v2: List<u64>) -> Option<Adt1> {
    let m74 = v2;
    match m74.uncons() {
        None => {
            v1
        }
        Some((v3, v4)) => {
            f18(v0.clone(), {
                let c75: Fn1 = v0;
                c75.apply(v1, v3)
            }, v4)
        }
    }
}

pub fn f19(v0: (Adt0, u64), v1: Option<Adt1>, v2: u64) -> Option<Adt1> {
    let m76 = v1;
    match m76 {
        None => {
            if f25(v0, v2) {
                Some(Adt1::C1(v2))
            } else {
                None::<Adt1>
            }
        }
        Some(v3) => {
            Some(v3)
        }
    }
}

pub fn f20(_: (Adt0, u64)) -> List<u64> {
    List::cons(2u64, List::cons(1u64, List::<u64>::nil()))
}

pub fn f21(v0: (Adt0, u64)) -> Option<(Adt0, u64)> {
    if f17(v0.clone()) {
        Some(f26(v0))
    } else {
        None::<(Adt0, u64)>
    }
}

pub fn f22(v0: (Adt0, u64), v1: u64) -> Option<(Adt0, u64)> {
    if f25(v0.clone(), v1) {
        Some(f27(v0, v1))
    } else {
        None::<(Adt0, u64)>
    }
}

pub fn f23(v0: Fn2, v1: (bool, u64), v2: List<u64>) -> R<(bool, u64)> {
    let m77 = v2;
    match m77.uncons() {
        None => {
            Ok(v1)
        }
        Some((v3, v4)) => {
            f23(v0.clone(), {
                let c78: Fn2 = v0;
                c78.apply(v1, v3)?
            }, v4)
        }
    }
}

pub fn f24(v0: (Adt0, u64), v1: (bool, u64), v2: u64) -> R<(bool, u64)> {
    let (h79, _) = v1;
    if h79 {
        Ok(v1)
    } else {
        Ok((f25(v0, v2), {
            let a81 = {
                let (_, h80) = v1;
                h80
            };
            let a82 = 1u64;
            nat_add(a81, a82)?
        }))
    }
}

pub fn f25(v0: (Adt0, u64), v1: u64) -> bool {
    let a84 = v1;
    let a85 = {
        let (_, h83) = v0;
        h83
    };
    if nat_le(a84, a85) {
        let a86 = 0u64;
        let a87 = v1;
        nat_lt(a86, a87)
    } else {
        false
    }
}

pub fn f26(v0: (Adt0, u64)) -> (Adt0, u64) {
    ({
        let (h88, _) = v0;
        h88
    }, {
        let a90 = {
            let (_, h89) = v0;
            h89
        };
        let a91 = 1u64;
        nat_sub(a90, a91)
    })
}

pub fn f27(v0: (Adt0, u64), v1: u64) -> (Adt0, u64) {
    ({
        let (h92, _) = v0;
        h92
    }, {
        let a94 = {
            let (_, h93) = v0;
            h93
        };
        let a95 = v1;
        nat_sub(a94, a95)
    })
}
