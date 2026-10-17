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
    C0(Bytes, Option<Bytes>),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(List<Adt2>, List<Bytes>),
    C1(List<Adt2>, List<Bytes>, Bytes, Option<Bytes>),
    C2(Adt0),
}

#[derive(Clone)]
pub enum Adt2 {
    C0(Bytes, List<Bytes>, Option<Bytes>),
}

#[derive(Clone)]
pub enum Fn0 {
    F3,
}

impl Fn0 {
    pub fn apply(&self, p0: Adt1, p1: Bytes) -> R<Adt1> {
        match self {
            Fn0::F3 => f3(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F12,
}

impl Fn1 {
    pub fn apply(&self, p0: Result<List<(Bytes, List<Bytes>)>, Adt0>, p1: Adt2) -> Result<List<(Bytes, List<Bytes>)>, Adt0> {
        match self {
            Fn1::F12 => f12(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F16(Bytes),
    F92(u8, Bytes),
}

impl Fn2 {
    pub fn apply(&self, p0: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
        match self {
            Fn2::F16(k0) => f16(k0.clone(), p0),
            Fn2::F92(k0, k1) => f92(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F23,
}

impl Fn3 {
    pub fn apply(&self, p0: u64, p1: Bytes) -> R<u64> {
        match self {
            Fn3::F23 => f23(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F25,
}

impl Fn4 {
    pub fn apply(&self, p0: u64, p1: Adt2) -> R<u64> {
        match self {
            Fn4::F25 => f25(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F28,
}

impl Fn5 {
    pub fn apply(&self, p0: List<Adt2>, p1: Adt2) -> List<Adt2> {
        match self {
            Fn5::F28 => f28(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F37,
}

impl Fn6 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn6::F37 => f37(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F39(Bytes),
    F63(Bytes),
    F64(u64, Bytes),
    F81(Bytes, Bytes),
    F86(Bytes, u64, Bytes, u64),
    F93(Bytes),
}

impl Fn7 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn7::F39(k0) => f39(k0.clone(), p0),
            Fn7::F63(k0) => f63(k0.clone(), p0),
            Fn7::F64(k0, k1) => f64(*k0, k1.clone(), p0),
            Fn7::F81(k0, k1) => Ok(f81(k0.clone(), k1.clone(), p0)),
            Fn7::F86(k0, k1, k2, k3) => f86(k0.clone(), *k1, k2.clone(), *k3, p0),
            Fn7::F93(k0) => f93(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F72(Bytes),
}

impl Fn8 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn8::F72(k0) => f72(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F84,
}

impl Fn9 {
    pub fn apply(&self, p0: Result<List<Bytes>, Bytes>, p1: Bytes) -> R<Result<List<Bytes>, Bytes>> {
        match self {
            Fn9::F84 => f84(p0, p1),
        }
    }
}

pub fn f0(v0: Bytes, v1: List<Bytes>) -> R<Result<List<(Bytes, List<Bytes>)>, Adt0>> {
    f1(f2(Fn0::F3, Adt1::C0(List::<Adt2>::nil(), v1.clone()), f4(v0)?)?, v1)
}

pub fn f1(v0: Adt1, v1: List<Bytes>) -> R<Result<List<(Bytes, List<Bytes>)>, Adt0>> {
    let m1 = v0;
    match m1 {
        Adt1::C0(v7, _) => {
            let v9: Result<List<(Bytes, List<Bytes>)>, Adt0> = f5(f6(f6(f6(f7(Str::lit("expected ")), f8(f9(v1.clone())?)?), f7(Str::lit(" audit records, found "))), f8(f10(v7.clone())?)?), None::<Bytes>);
            let a2 = f10(v7.clone())?;
            let a3 = f9(v1)?;
            if nat_eq(a2, a3) {
                Ok(f11(Fn1::F12, Ok::<List<(Bytes, List<Bytes>)>, Adt0>(List::<(Bytes, List<Bytes>)>::nil()), f13(v7)))
            } else {
                Ok(v9)
            }
        }
        Adt1::C1(_, _, v5, v6) => {
            Ok(f5(f6(f7(Str::lit("unterminated axiom payload: ")), v5), v6))
        }
        Adt1::C2(v2) => {
            Ok(Err::<List<(Bytes, List<Bytes>)>, Adt0>(v2))
        }
    }
}

pub fn f2(v0: Fn0, v1: Adt1, v2: List<Bytes>) -> R<Adt1> {
    let m4 = v2;
    match m4.uncons() {
        None => {
            Ok(v1)
        }
        Some((v3, v4)) => {
            f2(v0.clone(), {
                let c5: Fn0 = v0;
                c5.apply(v1, v3)?
            }, v4)
        }
    }
}

pub fn f3(v0: Adt1, v1: Bytes) -> R<Adt1> {
    f14(v0, v1)
}

pub fn f4(v0: Bytes) -> R<List<Bytes>> {
    let v1: u64 = {
        let a6 = v0.clone();
        length_bytes(a6)
    };
    let v2: (u64, (u64, List<Bytes>)) = {
        let (h7, _) = f15(Fn2::F16(v0.clone()), v1, (0u64, (0u64, List::<Bytes>::nil())))?;
        h7
    };
    let v3: u64 = {
        let (h8, _) = {
            let (_, h9) = v2.clone();
            h9
        };
        h8
    };
    let v4: List<Bytes> = {
        let (_, h10) = {
            let (_, h11) = v2;
            h11
        };
        h10
    };
    let a12 = v3;
    let a13 = v1;
    if nat_lt(a12, a13) {
        Ok(f17(List::cons(f18(v0, v3, {
            let a14 = v1;
            let a15 = v3;
            nat_sub(a14, a15)
        }), v4)))
    } else {
        Ok(f17(v4))
    }
}

pub fn f5(v0: Bytes, v1: Option<Bytes>) -> Result<List<(Bytes, List<Bytes>)>, Adt0> {
    Err::<List<(Bytes, List<Bytes>)>, Adt0>(f19(v0, v1))
}

pub fn f6(v0: Bytes, v1: Bytes) -> Bytes {
    let a16 = v0;
    let a17 = v1;
    append_bytes(a16, a17)
}

pub fn f7(v0: Str) -> Bytes {
    let a18 = v0;
    utf8_encode(a18)
}

pub fn f8(v0: u64) -> R<Bytes> {
    let v1: Bytes = f7(Str::lit("0"));
    let v2: Bytes = f20({
        let a55 = f21(v0, 10000000000000000000u64);
        let a56 = {
            let a53 = f21(v0, 1000000000000000000u64);
            let a54 = {
                let a51 = f21(v0, 100000000000000000u64);
                let a52 = {
                    let a49 = f21(v0, 10000000000000000u64);
                    let a50 = {
                        let a47 = f21(v0, 1000000000000000u64);
                        let a48 = {
                            let a45 = f21(v0, 100000000000000u64);
                            let a46 = {
                                let a43 = f21(v0, 10000000000000u64);
                                let a44 = {
                                    let a41 = f21(v0, 1000000000000u64);
                                    let a42 = {
                                        let a39 = f21(v0, 100000000000u64);
                                        let a40 = {
                                            let a37 = f21(v0, 10000000000u64);
                                            let a38 = {
                                                let a35 = f21(v0, 1000000000u64);
                                                let a36 = {
                                                    let a33 = f21(v0, 100000000u64);
                                                    let a34 = {
                                                        let a31 = f21(v0, 10000000u64);
                                                        let a32 = {
                                                            let a29 = f21(v0, 1000000u64);
                                                            let a30 = {
                                                                let a27 = f21(v0, 100000u64);
                                                                let a28 = {
                                                                    let a25 = f21(v0, 10000u64);
                                                                    let a26 = {
                                                                        let a23 = f21(v0, 1000u64);
                                                                        let a24 = {
                                                                            let a21 = f21(v0, 100u64);
                                                                            let a22 = {
                                                                                let a19 = f21(v0, 10u64);
                                                                                let a20 = f21(v0, 1u64);
                                                                                append_bytes(a19, a20)
                                                                            };
                                                                            append_bytes(a21, a22)
                                                                        };
                                                                        append_bytes(a23, a24)
                                                                    };
                                                                    append_bytes(a25, a26)
                                                                };
                                                                append_bytes(a27, a28)
                                                            };
                                                            append_bytes(a29, a30)
                                                        };
                                                        append_bytes(a31, a32)
                                                    };
                                                    append_bytes(a33, a34)
                                                };
                                                append_bytes(a35, a36)
                                            };
                                            append_bytes(a37, a38)
                                        };
                                        append_bytes(a39, a40)
                                    };
                                    append_bytes(a41, a42)
                                };
                                append_bytes(a43, a44)
                            };
                            append_bytes(a45, a46)
                        };
                        append_bytes(a47, a48)
                    };
                    append_bytes(a49, a50)
                };
                append_bytes(a51, a52)
            };
            append_bytes(a53, a54)
        };
        append_bytes(a55, a56)
    })?;
    let a57 = v0;
    let a58 = 0u64;
    if nat_eq(a57, a58) {
        Ok(v1)
    } else {
        Ok(v2)
    }
}

pub fn f9(v0: List<Bytes>) -> R<u64> {
    f22(Fn3::F23, 0u64, v0)
}

pub fn f10(v0: List<Adt2>) -> R<u64> {
    f24(Fn4::F25, 0u64, v0)
}

pub fn f11(v0: Fn1, v1: Result<List<(Bytes, List<Bytes>)>, Adt0>, v2: List<Adt2>) -> Result<List<(Bytes, List<Bytes>)>, Adt0> {
    let m59 = v2;
    match m59.uncons() {
        None => {
            v1
        }
        Some((v3, v4)) => {
            f11(v0.clone(), {
                let c60: Fn1 = v0;
                c60.apply(v1, v3)
            }, v4)
        }
    }
}

pub fn f12(v0: Result<List<(Bytes, List<Bytes>)>, Adt0>, v1: Adt2) -> Result<List<(Bytes, List<Bytes>)>, Adt0> {
    f26(v0, v1)
}

pub fn f13(v0: List<Adt2>) -> List<Adt2> {
    f27(Fn5::F28, List::<Adt2>::nil(), v0)
}

pub fn f14(v0: Adt1, v1: Bytes) -> R<Adt1> {
    let m61 = v0.clone();
    match m61 {
        Adt1::C0(v8, v9) => {
            let a64 = 0u64;
            let a65 = {
                let a63 = f32(v1.clone())?;
                length_bytes(a63)
            };
            if nat_eq(a64, a65) {
                Ok(v0)
            } else {
                let m66 = f33(f32(v1.clone())?)?;
                match m66 {
                    None => {
                        Ok(f31(f6(f7(Str::lit("unrecognized audit line: ")), f32(v1)?), f34(v9)))
                    }
                    Some(v10) => {
                        f30(v8, v9.clone(), v10, f34(v9))
                    }
                }
            }
        }
        Adt1::C1(v3, v4, v5, v6) => {
            let m62 = f29(v1)?;
            match m62 {
                None => {
                    Ok(f31(f6(f7(Str::lit("unterminated axiom payload: ")), v5), v6))
                }
                Some(v7) => {
                    f30(v3, v4, f6(f6(v5, f7(Str::lit(" "))), v7), v6)
                }
            }
        }
        Adt1::C2(_) => {
            Ok(v0)
        }
    }
}

pub fn f15(v0: Fn2, v1: u64, v2: (u64, (u64, List<Bytes>))) -> R<((u64, (u64, List<Bytes>)), bool)> {
    let m67 = v1;
    if m67 == 0 {
        Ok((v2, false))
    } else {
        let v3 = m67 - 1;
        let m69 = {
            let c68: Fn2 = v0.clone();
            c68.apply(v2.clone())?
        };
        match m69 {
            None => {
                Ok((v2, true))
            }
            Some(v4) => {
                f15(v0, v3, v4)
            }
        }
    }
}

pub fn f16(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f35(v0, v1)
}

pub fn f17(v0: List<Bytes>) -> List<Bytes> {
    f36(Fn6::F37, List::<Bytes>::nil(), v0)
}

pub fn f18(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m73 = {
        let a70 = v0;
        let a71 = v1;
        let a72 = v2;
        slice_bytes(a70, a71, a72)
    };
    match m73 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f19(v0: Bytes, v1: Option<Bytes>) -> Adt0 {
    Adt0::C0(v0, v1)
}

pub fn f20(v0: Bytes) -> R<Bytes> {
    let v1: u64 = {
        let (h74, _) = f38(Fn7::F39(v0.clone()), {
            let a75 = v0.clone();
            length_bytes(a75)
        }, 0u64)?;
        h74
    };
    Ok(f18(v0.clone(), v1, {
        let a77 = {
            let a76 = v0;
            length_bytes(a76)
        };
        let a78 = v1;
        nat_sub(a77, a78)
    }))
}

pub fn f21(v0: u64, v1: u64) -> Bytes {
    f18(f7(Str::lit("0123456789")), {
        let a82 = {
            let a79 = v0;
            let a80 = v1;
            let a81 = 0u64;
            nat_quot(a79, a80, a81)
        };
        let a83 = 10u64;
        let a84 = 0u64;
        nat_rem(a82, a83, a84)
    }, 1u64)
}

pub fn f22(v0: Fn3, v1: u64, v2: List<Bytes>) -> R<u64> {
    let m85 = v2;
    match m85.uncons() {
        None => {
            Ok(v1)
        }
        Some((v3, v4)) => {
            f22(v0.clone(), {
                let c86: Fn3 = v0;
                c86.apply(v1, v3)?
            }, v4)
        }
    }
}

pub fn f23(v0: u64, _: Bytes) -> R<u64> {
    let a87 = v0;
    let a88 = 1u64;
    nat_add(a87, a88)
}

pub fn f24(v0: Fn4, v1: u64, v2: List<Adt2>) -> R<u64> {
    let m89 = v2;
    match m89.uncons() {
        None => {
            Ok(v1)
        }
        Some((v3, v4)) => {
            f24(v0.clone(), {
                let c90: Fn4 = v0;
                c90.apply(v1, v3)?
            }, v4)
        }
    }
}

pub fn f25(v0: u64, _: Adt2) -> R<u64> {
    let a91 = v0;
    let a92 = 1u64;
    nat_add(a91, a92)
}

pub fn f26(v0: Result<List<(Bytes, List<Bytes>)>, Adt0>, v1: Adt2) -> Result<List<(Bytes, List<Bytes>)>, Adt0> {
    let m93 = v0.clone();
    match m93 {
        Ok(v3) => {
            let v5: Bytes = {
                let m95 = match v1.clone() {
                    Adt2::C0(_, _, h94) => {
                        h94
                    }
                };
                match m95 {
                    None => {
                        Bytes::lit(&[])
                    }
                    Some(v4) => {
                        v4
                    }
                }
            };
            let v6: Result<List<(Bytes, List<Bytes>)>, Adt0> = f5(f6(f7(Str::lit("duplicate record for ")), f40(match v1.clone() {
                Adt2::C0(h96, _, _) => {
                    h96
                }
            })), Some(v5.clone()));
            let v7: Result<List<(Bytes, List<Bytes>)>, Adt0> = f5(f6(f6(f6(f7(Str::lit("expected a record for ")), f40(v5.clone())), f7(Str::lit(", found "))), f40(match v1.clone() {
                Adt2::C0(h97, _, _) => {
                    h97
                }
            })), Some(v5.clone()));
            let a99 = match v1.clone() {
                Adt2::C0(h98, _, _) => {
                    h98
                }
            };
            let a100 = v5;
            if equal(a99, a100) {
                let m103 = f41(match v1.clone() {
                    Adt2::C0(h101, _, _) => {
                        h101
                    }
                }, match v1 {
                    Adt2::C0(_, h102, _) => {
                        h102
                    }
                }, v3);
                match m103 {
                    None => {
                        v6
                    }
                    Some(v8) => {
                        Ok::<List<(Bytes, List<Bytes>)>, Adt0>(v8)
                    }
                }
            } else {
                v7
            }
        }
        Err(_) => {
            v0
        }
    }
}

pub fn f27(v0: Fn5, v1: List<Adt2>, v2: List<Adt2>) -> List<Adt2> {
    let m104 = v2;
    match m104.uncons() {
        None => {
            v1
        }
        Some((v3, v4)) => {
            f27(v0.clone(), {
                let c105: Fn5 = v0;
                c105.apply(v1, v3)
            }, v4)
        }
    }
}

pub fn f28(v0: List<Adt2>, v1: Adt2) -> List<Adt2> {
    List::cons(v1, v0)
}

pub fn f29(v0: Bytes) -> R<Option<Bytes>> {
    let v1: Bytes = f32(v0.clone())?;
    if f42(v0)? {
        let a107 = 0u64;
        let a108 = {
            let a106 = v1.clone();
            length_bytes(a106)
        };
        if nat_eq(a107, a108) {
            Ok(None::<Bytes>)
        } else {
            Ok(Some(v1))
        }
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f30(v0: List<Adt2>, v1: List<Bytes>, v2: Bytes, v3: Option<Bytes>) -> R<Adt1> {
    if f43(v2.clone()) {
        let m109 = f44(v2)?;
        match m109 {
            Ok(v5) => {
                Ok(Adt1::C0(List::cons(Adt2::C0({
                    let (h110, _) = v5;
                    h110
                }, {
                    let (_, h111) = v5;
                    h111
                }, v3), v0), f45(v1)))
            }
            Err(v4) => {
                Ok(f31(v4, v3))
            }
        }
    } else {
        Ok(Adt1::C1(v0, v1, v2, v3))
    }
}

pub fn f31(v0: Bytes, v1: Option<Bytes>) -> Adt1 {
    Adt1::C2(f19(v0, v1))
}

pub fn f32(v0: Bytes) -> R<Bytes> {
    let v1: u64 = f46(v0.clone())?;
    let v2: u64 = f47(v0.clone(), v1)?;
    Ok(f18(v0, v1, {
        let a112 = v2;
        let a113 = v1;
        nat_sub(a112, a113)
    }))
}

pub fn f33(v0: Bytes) -> R<Option<Bytes>> {
    if f48(v0.clone(), f7(Str::lit("'"))) {
        Ok(Some(v0))
    } else {
        let m114 = f49(v0.clone(), f7(Str::lit("info: ")));
        match m114 {
            None => {
                f50(v0)
            }
            Some(v1) => {
                Ok(Some(v1))
            }
        }
    }
}

pub fn f34(v0: List<Bytes>) -> Option<Bytes> {
    let m115 = v0;
    match m115.uncons() {
        None => {
            None::<Bytes>
        }
        Some((v1, _)) => {
            Some(v1)
        }
    }
}

pub fn f35(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v2: u64 = {
        let (h116, _) = v1.clone();
        h116
    };
    let v3: u64 = {
        let (h117, _) = {
            let (_, h118) = v1.clone();
            h118
        };
        h117
    };
    let v4: List<Bytes> = {
        let (_, h119) = {
            let (_, h120) = v1;
            h120
        };
        h119
    };
    let m121 = f51(v0.clone(), v2);
    match m121 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v5) => {
            let a122 = v5;
            let a123 = 10u8;
            if equal(a122, a123) {
                Ok(Some(({
                    let a124 = v2;
                    let a125 = 1u64;
                    nat_add(a124, a125)?
                }, ({
                    let a126 = v2;
                    let a127 = 1u64;
                    nat_add(a126, a127)?
                }, List::cons(f52(f18(v0, v3, {
                    let a128 = v2;
                    let a129 = v3;
                    nat_sub(a128, a129)
                })), v4)))))
            } else {
                Ok(Some(({
                    let a130 = v2;
                    let a131 = 1u64;
                    nat_add(a130, a131)?
                }, (v3, v4))))
            }
        }
    }
}

pub fn f36(v0: Fn6, v1: List<Bytes>, v2: List<Bytes>) -> List<Bytes> {
    let m132 = v2;
    match m132.uncons() {
        None => {
            v1
        }
        Some((v3, v4)) => {
            f36(v0.clone(), {
                let c133: Fn6 = v0;
                c133.apply(v1, v3)
            }, v4)
        }
    }
}

pub fn f37(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f38(v0: Fn7, v1: u64, v2: u64) -> R<(u64, bool)> {
    let m134 = v1;
    if m134 == 0 {
        Ok((v2, false))
    } else {
        let v3 = m134 - 1;
        let m136 = {
            let c135: Fn7 = v0.clone();
            c135.apply(v2)?
        };
        match m136 {
            None => {
                Ok((v2, true))
            }
            Some(v4) => {
                f38(v0, v3, v4)
            }
        }
    }
}

pub fn f39(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f53(v0, v1)
}

pub fn f40(v0: Bytes) -> Bytes {
    f6(f6(f7(Str::lit("`")), v0), f7(Str::lit("`")))
}

pub fn f41(v0: Bytes, v1: List<Bytes>, v2: List<(Bytes, List<Bytes>)>) -> Option<List<(Bytes, List<Bytes>)>> {
    let m137 = v2;
    match m137.uncons() {
        None => {
            Some(List::cons((v0, v1), List::<(Bytes, List<Bytes>)>::nil()))
        }
        Some((v3, v4)) => {
            let a139 = v0.clone();
            let a140 = {
                let (h138, _) = v3.clone();
                h138
            };
            if equal(a139, a140) {
                None::<List<(Bytes, List<Bytes>)>>
            } else {
                if f54(v0.clone(), {
                    let (h141, _) = v3.clone();
                    h141
                }) {
                    Some(List::cons((v0, v1), List::cons(v3, v4)))
                } else {
                    let m142 = f41(v0, v1, v4);
                    match m142 {
                        None => {
                            None::<List<(Bytes, List<Bytes>)>>
                        }
                        Some(v5) => {
                            Some(List::cons(v3, v5))
                        }
                    }
                }
            }
        }
    }
}

pub fn f42(v0: Bytes) -> R<bool> {
    let a143 = 0u64;
    let a144 = f55(v0, 0u64)?;
    Ok(nat_lt(a143, a144))
}

pub fn f43(v0: Bytes) -> bool {
    let m145 = f56(v0.clone(), f7(Str::lit("]")));
    match m145 {
        None => {
            let m146 = f56(v0, f7(Str::lit(" does not depend on any axioms")));
            match m146 {
                None => {
                    false
                }
                Some(_) => {
                    true
                }
            }
        }
        Some(_) => {
            true
        }
    }
}

pub fn f44(v0: Bytes) -> R<Result<(Bytes, List<Bytes>), Bytes>> {
    let m147 = f49(v0.clone(), f7(Str::lit("'")));
    match m147 {
        None => {
            Ok(f57(v0))
        }
        Some(v1) => {
            let m148 = f56(v1.clone(), f7(Str::lit("' does not depend on any axioms")));
            match m148 {
                None => {
                    let m149 = f56(v1, f7(Str::lit("]")));
                    match m149 {
                        None => {
                            Ok(f57(v0))
                        }
                        Some(v5) => {
                            f62(v0, v5)
                        }
                    }
                }
                Some(v2) => {
                    let v3: Result<(Bytes, List<Bytes>), Bytes> = f58(v2.clone(), List::<Bytes>::nil());
                    let v4: Result<(Bytes, List<Bytes>), Bytes> = f59(f60(v2.clone()));
                    if f61(v2)? {
                        Ok(v3)
                    } else {
                        Ok(v4)
                    }
                }
            }
        }
    }
}

pub fn f45(v0: List<Bytes>) -> List<Bytes> {
    let m150 = v0;
    match m150.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((_, v2)) => {
            v2
        }
    }
}

pub fn f46(v0: Bytes) -> R<u64> {
    let (h151, _) = f38(Fn7::F63(v0.clone()), {
        let a152 = v0;
        length_bytes(a152)
    }, 0u64)?;
    Ok(h151)
}

pub fn f47(v0: Bytes, v1: u64) -> R<u64> {
    let (h153, _) = f38(Fn7::F64(v1, v0.clone()), {
        let a154 = v0.clone();
        length_bytes(a154)
    }, {
        let a155 = v0;
        length_bytes(a155)
    })?;
    Ok(h153)
}

pub fn f48(v0: Bytes, v1: Bytes) -> bool {
    let m160 = {
        let a157 = v0;
        let a158 = 0u64;
        let a159 = {
            let a156 = v1.clone();
            length_bytes(a156)
        };
        slice_bytes(a157, a158, a159)
    };
    match m160 {
        None => {
            false
        }
        Some(v2) => {
            let a161 = v2;
            let a162 = v1;
            equal(a161, a162)
        }
    }
}

pub fn f49(v0: Bytes, v1: Bytes) -> Option<Bytes> {
    if f48(v0.clone(), v1.clone()) {
        Some(f18(v0.clone(), {
            let a163 = v1.clone();
            length_bytes(a163)
        }, {
            let a166 = {
                let a164 = v0;
                length_bytes(a164)
            };
            let a167 = {
                let a165 = v1;
                length_bytes(a165)
            };
            nat_sub(a166, a167)
        }))
    } else {
        None::<Bytes>
    }
}

pub fn f50(v0: Bytes) -> R<Option<Bytes>> {
    let m168 = f65(v0, f7(Str::lit(":")))?;
    match m168 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v1) => {
            let m170 = f65({
                let (_, h169) = v1;
                h169
            }, f7(Str::lit(":")))?;
            match m170 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v2) => {
                    let m172 = f65({
                        let (_, h171) = v2;
                        h171
                    }, f7(Str::lit(":")))?;
                    match m172 {
                        None => {
                            Ok(None::<Bytes>)
                        }
                        Some(v3) => {
                            let v4: Bytes = {
                                let (h173, _) = v2;
                                h173
                            };
                            let v5: Bytes = {
                                let (h174, _) = v3;
                                h174
                            };
                            let v6: Bytes = {
                                let (_, h175) = v3;
                                h175
                            };
                            let v7: Option<Bytes> = f66(v5, v6)?;
                            if f67(v4)? {
                                Ok(v7)
                            } else {
                                Ok(None::<Bytes>)
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f51(v0: Bytes, v1: u64) -> Option<u8> {
    let a176 = v0;
    let a177 = v1;
    index_bytes(a176, a177)
}

pub fn f52(v0: Bytes) -> Bytes {
    if f68(v0.clone(), Bytes::lit(&[0x0d])) {
        f18(v0.clone(), 0u64, {
            let a179 = {
                let a178 = v0;
                length_bytes(a178)
            };
            let a180 = 1u64;
            nat_sub(a179, a180)
        })
    } else {
        v0
    }
}

pub fn f53(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f69(v0, v1, 48u8) {
        Ok(Some({
            let a181 = v1;
            let a182 = 1u64;
            nat_add(a181, a182)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f54(v0: Bytes, v1: Bytes) -> bool {
    let a187 = {
        let a183 = v0;
        let a184 = v1;
        compare_bytes(a183, a184)
    };
    let a188 = {
        let a185 = Bytes::lit(&[]);
        let a186 = Bytes::lit(&[0x01]);
        compare_bytes(a185, a186)
    };
    equal(a187, a188)
}

pub fn f55(v0: Bytes, v1: u64) -> R<u64> {
    if if f70(v0.clone(), v1, 9u8, 13u8) {
        true
    } else {
        f69(v0.clone(), v1, 32u8)
    } {
        Ok(1u64)
    } else {
        if f69(v0.clone(), v1, 194u8) {
            if if f69(v0.clone(), {
                let a189 = v1;
                let a190 = 1u64;
                nat_add(a189, a190)?
            }, 133u8) {
                true
            } else {
                f69(v0, {
                    let a191 = v1;
                    let a192 = 1u64;
                    nat_add(a191, a192)?
                }, 160u8)
            } {
                Ok(2u64)
            } else {
                Ok(0u64)
            }
        } else {
            if f69(v0.clone(), v1, 225u8) {
                if if f69(v0.clone(), {
                    let a193 = v1;
                    let a194 = 1u64;
                    nat_add(a193, a194)?
                }, 154u8) {
                    f69(v0, {
                        let a195 = v1;
                        let a196 = 2u64;
                        nat_add(a195, a196)?
                    }, 128u8)
                } else {
                    false
                } {
                    Ok(3u64)
                } else {
                    Ok(0u64)
                }
            } else {
                if f69(v0.clone(), v1, 226u8) {
                    if f69(v0.clone(), {
                        let a197 = v1;
                        let a198 = 1u64;
                        nat_add(a197, a198)?
                    }, 128u8) {
                        if if f70(v0.clone(), {
                            let a199 = v1;
                            let a200 = 2u64;
                            nat_add(a199, a200)?
                        }, 128u8, 138u8) {
                            true
                        } else {
                            if f69(v0.clone(), {
                                let a201 = v1;
                                let a202 = 2u64;
                                nat_add(a201, a202)?
                            }, 168u8) {
                                true
                            } else {
                                if f69(v0.clone(), {
                                    let a203 = v1;
                                    let a204 = 2u64;
                                    nat_add(a203, a204)?
                                }, 169u8) {
                                    true
                                } else {
                                    f69(v0, {
                                        let a205 = v1;
                                        let a206 = 2u64;
                                        nat_add(a205, a206)?
                                    }, 175u8)
                                }
                            }
                        } {
                            Ok(3u64)
                        } else {
                            Ok(0u64)
                        }
                    } else {
                        if if f69(v0.clone(), {
                            let a207 = v1;
                            let a208 = 1u64;
                            nat_add(a207, a208)?
                        }, 129u8) {
                            f69(v0, {
                                let a209 = v1;
                                let a210 = 2u64;
                                nat_add(a209, a210)?
                            }, 159u8)
                        } else {
                            false
                        } {
                            Ok(3u64)
                        } else {
                            Ok(0u64)
                        }
                    }
                } else {
                    if f69(v0.clone(), v1, 227u8) {
                        if if f69(v0.clone(), {
                            let a211 = v1;
                            let a212 = 1u64;
                            nat_add(a211, a212)?
                        }, 128u8) {
                            f69(v0, {
                                let a213 = v1;
                                let a214 = 2u64;
                                nat_add(a213, a214)?
                            }, 128u8)
                        } else {
                            false
                        } {
                            Ok(3u64)
                        } else {
                            Ok(0u64)
                        }
                    } else {
                        Ok(0u64)
                    }
                }
            }
        }
    }
}

pub fn f56(v0: Bytes, v1: Bytes) -> Option<Bytes> {
    if f68(v0.clone(), v1.clone()) {
        Some(f18(v0.clone(), 0u64, {
            let a217 = {
                let a215 = v0;
                length_bytes(a215)
            };
            let a218 = {
                let a216 = v1;
                length_bytes(a216)
            };
            nat_sub(a217, a218)
        }))
    } else {
        None::<Bytes>
    }
}

pub fn f57(v0: Bytes) -> Result<(Bytes, List<Bytes>), Bytes> {
    f59(f6(f7(Str::lit("unrecognized axiom payload: ")), v0))
}

pub fn f58(v0: Bytes, v1: List<Bytes>) -> Result<(Bytes, List<Bytes>), Bytes> {
    Ok::<(Bytes, List<Bytes>), Bytes>((v0, v1))
}

pub fn f59(v0: Bytes) -> Result<(Bytes, List<Bytes>), Bytes> {
    Err::<(Bytes, List<Bytes>), Bytes>(v0)
}

pub fn f60(v0: Bytes) -> Bytes {
    f6(f40(v0), f7(Str::lit(" is not a Lean name")))
}

pub fn f61(v0: Bytes) -> R<bool> {
    let a222 = {
        let (_, h219) = {
            let (h220, _) = f71(Fn8::F72(v0.clone()), {
                let a221 = v0;
                length_bytes(a221)
            }, (0u64, 0u64))?;
            h220
        };
        h219
    };
    let a223 = 1u64;
    Ok(nat_eq(a222, a223))
}

pub fn f62(v0: Bytes, v1: Bytes) -> R<Result<(Bytes, List<Bytes>), Bytes>> {
    let m224 = f73(v1, f7(Str::lit("' depends on axioms: [")))?;
    match m224 {
        None => {
            Ok(f57(v0))
        }
        Some(v2) => {
            let v3: Bytes = {
                let (h225, _) = v2;
                h225
            };
            let v4: Bytes = {
                let (_, h226) = v2;
                h226
            };
            let v5: Result<(Bytes, List<Bytes>), Bytes> = f74(v3.clone(), v4)?;
            let v6: Result<(Bytes, List<Bytes>), Bytes> = f59(f60(v3.clone()));
            if f61(v3)? {
                Ok(v5)
            } else {
                Ok(v6)
            }
        }
    }
}

pub fn f63(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a227 = 0u64;
    let a228 = f55(v0.clone(), v1)?;
    if nat_lt(a227, a228) {
        Ok(Some({
            let a229 = v1;
            let a230 = f55(v0, v1)?;
            nat_add(a229, a230)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f64(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    let a231 = 0u64;
    let a232 = f75(v1.clone(), v2)?;
    if if nat_lt(a231, a232) {
        let a235 = {
            let a233 = v0;
            let a234 = f75(v1.clone(), v2)?;
            nat_add(a233, a234)?
        };
        let a236 = v2;
        nat_le(a235, a236)
    } else {
        false
    } {
        Ok(Some({
            let a237 = v2;
            let a238 = f75(v1, v2)?;
            nat_sub(a237, a238)
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f65(v0: Bytes, v1: Bytes) -> R<Option<(Bytes, Bytes)>> {
    let m239 = f76(v0.clone(), v1.clone())?;
    match m239 {
        None => {
            Ok(None::<(Bytes, Bytes)>)
        }
        Some(v2) => {
            Ok(Some((f18(v0.clone(), 0u64, v2), f18(v0.clone(), {
                let a241 = v2;
                let a242 = {
                    let a240 = v1.clone();
                    length_bytes(a240)
                };
                nat_add(a241, a242)?
            }, {
                let a247 = {
                    let a243 = v0;
                    length_bytes(a243)
                };
                let a248 = {
                    let a245 = v2;
                    let a246 = {
                        let a244 = v1;
                        length_bytes(a244)
                    };
                    nat_add(a245, a246)?
                };
                nat_sub(a247, a248)
            }))))
        }
    }
}

pub fn f66(v0: Bytes, v1: Bytes) -> R<Option<Bytes>> {
    let m249 = f65(v0.clone(), f7(Str::lit("-")))?;
    match m249 {
        None => {
            let v9: Option<Bytes> = f77(v1);
            if f67(v0)? {
                Ok(v9)
            } else {
                Ok(None::<Bytes>)
            }
        }
        Some(v2) => {
            let v3: Bytes = {
                let (h250, _) = v2;
                h250
            };
            let v4: Bytes = {
                let (_, h251) = v2;
                h251
            };
            let m252 = f65(v1, f7(Str::lit(":")))?;
            match m252 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v5) => {
                    let v6: Bytes = {
                        let (h253, _) = v5;
                        h253
                    };
                    let v7: Bytes = {
                        let (_, h254) = v5;
                        h254
                    };
                    let v8: Option<Bytes> = f77(v7);
                    if f67(v3)? {
                        if f67(v4)? {
                            if f67(v6)? {
                                Ok(v8)
                            } else {
                                Ok(None::<Bytes>)
                            }
                        } else {
                            Ok(None::<Bytes>)
                        }
                    } else {
                        Ok(None::<Bytes>)
                    }
                }
            }
        }
    }
}

pub fn f67(v0: Bytes) -> R<bool> {
    f78(f32(v0)?)
}

pub fn f68(v0: Bytes, v1: Bytes) -> bool {
    let v2: u64 = {
        let a255 = v0.clone();
        length_bytes(a255)
    };
    let v3: u64 = {
        let a256 = v1.clone();
        length_bytes(a256)
    };
    let a257 = v3;
    let a258 = v2;
    if nat_le(a257, a258) {
        let m264 = {
            let a261 = v0;
            let a262 = {
                let a259 = v2;
                let a260 = v3;
                nat_sub(a259, a260)
            };
            let a263 = v3;
            slice_bytes(a261, a262, a263)
        };
        match m264 {
            None => {
                false
            }
            Some(v4) => {
                let a265 = v4;
                let a266 = v1;
                equal(a265, a266)
            }
        }
    } else {
        false
    }
}

pub fn f69(v0: Bytes, v1: u64, v2: u8) -> bool {
    let m267 = f51(v0, v1);
    match m267 {
        None => {
            false
        }
        Some(v3) => {
            let a268 = v3;
            let a269 = v2;
            equal(a268, a269)
        }
    }
}

pub fn f70(v0: Bytes, v1: u64, v2: u8, v3: u8) -> bool {
    let m270 = f51(v0, v1);
    match m270 {
        None => {
            false
        }
        Some(v4) => {
            f79(v4, v2, v3)
        }
    }
}

pub fn f71(v0: Fn8, v1: u64, v2: (u64, u64)) -> R<((u64, u64), bool)> {
    let m271 = v1;
    if m271 == 0 {
        Ok((v2, false))
    } else {
        let v3 = m271 - 1;
        let m273 = {
            let c272: Fn8 = v0.clone();
            c272.apply(v2)?
        };
        match m273 {
            None => {
                Ok((v2, true))
            }
            Some(v4) => {
                f71(v0, v3, v4)
            }
        }
    }
}

pub fn f72(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f80(v0, v1)
}

pub fn f73(v0: Bytes, v1: Bytes) -> R<Option<(Bytes, Bytes)>> {
    let v2: u64 = {
        let a274 = v0.clone();
        length_bytes(a274)
    };
    let v3: u64 = {
        let a275 = v1.clone();
        length_bytes(a275)
    };
    let a276 = v2;
    let a277 = v3;
    if nat_lt(a276, a277) {
        Ok(None::<(Bytes, Bytes)>)
    } else {
        let v4: u64 = {
            let (h278, _) = f38(Fn7::F81(v1.clone(), v0.clone()), {
                let a279 = v2;
                let a280 = 1u64;
                nat_add(a279, a280)?
            }, {
                let a281 = v2;
                let a282 = v3;
                nat_sub(a281, a282)
            })?;
            h278
        };
        if f82(v0.clone(), v1, v4) {
            Ok(Some((f18(v0.clone(), 0u64, v4), f18(v0, {
                let a283 = v4;
                let a284 = v3;
                nat_add(a283, a284)?
            }, {
                let a287 = v2;
                let a288 = {
                    let a285 = v4;
                    let a286 = v3;
                    nat_add(a285, a286)?
                };
                nat_sub(a287, a288)
            }))))
        } else {
            Ok(None::<(Bytes, Bytes)>)
        }
    }
}

pub fn f74(v0: Bytes, v1: Bytes) -> R<Result<(Bytes, List<Bytes>), Bytes>> {
    let m289 = f83(Fn9::F84, Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil()), f85(v1, 44u8)?)?;
    match m289 {
        Ok(v3) => {
            let m290 = v3.clone();
            match m290.uncons() {
                None => {
                    Ok(f59(f7(Str::lit("an axiom list cannot be empty"))))
                }
                Some((_, _)) => {
                    Ok(f58(v0, v3))
                }
            }
        }
        Err(v2) => {
            Ok(f59(v2))
        }
    }
}

pub fn f75(v0: Bytes, v1: u64) -> R<u64> {
    let a291 = v1;
    let a292 = 0u64;
    if nat_eq(a291, a292) {
        Ok(0u64)
    } else {
        let a295 = f55(v0.clone(), {
            let a293 = v1;
            let a294 = 1u64;
            nat_sub(a293, a294)
        })?;
        let a296 = 1u64;
        if nat_eq(a295, a296) {
            Ok(1u64)
        } else {
            let a297 = 2u64;
            let a298 = v1;
            if if nat_le(a297, a298) {
                let a301 = f55(v0.clone(), {
                    let a299 = v1;
                    let a300 = 2u64;
                    nat_sub(a299, a300)
                })?;
                let a302 = 2u64;
                nat_eq(a301, a302)
            } else {
                false
            } {
                Ok(2u64)
            } else {
                let a303 = 3u64;
                let a304 = v1;
                if if nat_le(a303, a304) {
                    let a307 = f55(v0, {
                        let a305 = v1;
                        let a306 = 3u64;
                        nat_sub(a305, a306)
                    })?;
                    let a308 = 3u64;
                    nat_eq(a307, a308)
                } else {
                    false
                } {
                    Ok(3u64)
                } else {
                    Ok(0u64)
                }
            }
        }
    }
}

pub fn f76(v0: Bytes, v1: Bytes) -> R<Option<u64>> {
    let v2: u64 = {
        let a309 = v0.clone();
        length_bytes(a309)
    };
    let v3: u64 = {
        let a310 = v1.clone();
        length_bytes(a310)
    };
    let v4: u64 = {
        let (h311, _) = f38(Fn7::F86(v1.clone(), v3, v0.clone(), v2), {
            let a312 = v2;
            let a313 = 1u64;
            nat_add(a312, a313)?
        }, 0u64)?;
        h311
    };
    if f82(v0, v1, v4) {
        Ok(Some(v4))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f77(v0: Bytes) -> Option<Bytes> {
    let m314 = f49(v0, f7(Str::lit(" ")));
    match m314 {
        None => {
            None::<Bytes>
        }
        Some(v1) => {
            f49(v1, f7(Str::lit("info: ")))
        }
    }
}

pub fn f78(v0: Bytes) -> R<bool> {
    let v2: Bytes = {
        let m315 = f49(v0.clone(), f7(Str::lit("+")));
        match m315 {
            None => {
                v0
            }
            Some(v1) => {
                v1
            }
        }
    };
    let v3: bool = f87(f20(v2.clone())?);
    let a317 = 0u64;
    let a318 = {
        let a316 = v2.clone();
        length_bytes(a316)
    };
    if nat_lt(a317, a318) {
        if f88(v2)? {
            Ok(v3)
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f79(v0: u8, v1: u8, v2: u8) -> bool {
    let m321 = {
        let a319 = v0;
        let a320 = v1;
        fixed_u8::checked_sub(a319, a320)
    };
    match m321 {
        None => {
            false
        }
        Some(_) => {
            let m324 = {
                let a322 = v2;
                let a323 = v0;
                fixed_u8::checked_sub(a322, a323)
            };
            match m324 {
                None => {
                    false
                }
                Some(_) => {
                    true
                }
            }
        }
    }
}

pub fn f80(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let a326 = {
        let (_, h325) = v1;
        h325
    };
    let a327 = 2u64;
    if nat_eq(a326, a327) {
        Ok(None::<(u64, u64)>)
    } else {
        let m329 = f51(v0, {
            let (h328, _) = v1;
            h328
        });
        match m329 {
            None => {
                Ok(None::<(u64, u64)>)
            }
            Some(v2) => {
                Ok(Some(({
                    let a331 = {
                        let (h330, _) = v1;
                        h330
                    };
                    let a332 = 1u64;
                    nat_add(a331, a332)?
                }, f89({
                    let (_, h333) = v1;
                    h333
                }, v2))))
            }
        }
    }
}

pub fn f81(v0: Bytes, v1: Bytes, v2: u64) -> Option<u64> {
    f90(v1, v0, v2)
}

pub fn f82(v0: Bytes, v1: Bytes, v2: u64) -> bool {
    let m338 = {
        let a335 = v0;
        let a336 = v2;
        let a337 = {
            let a334 = v1.clone();
            length_bytes(a334)
        };
        slice_bytes(a335, a336, a337)
    };
    match m338 {
        None => {
            false
        }
        Some(v3) => {
            let a339 = v3;
            let a340 = v1;
            equal(a339, a340)
        }
    }
}

pub fn f83(v0: Fn9, v1: Result<List<Bytes>, Bytes>, v2: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m341 = v2;
    match m341.uncons() {
        None => {
            Ok(v1)
        }
        Some((v3, v4)) => {
            f83(v0.clone(), {
                let c342: Fn9 = v0;
                c342.apply(v1, v3)?
            }, v4)
        }
    }
}

pub fn f84(v0: Result<List<Bytes>, Bytes>, v1: Bytes) -> R<Result<List<Bytes>, Bytes>> {
    f91(v0, v1)
}

pub fn f85(v0: Bytes, v1: u8) -> R<List<Bytes>> {
    let v2: u64 = {
        let a343 = v0.clone();
        length_bytes(a343)
    };
    let v3: (u64, (u64, List<Bytes>)) = {
        let (h344, _) = f15(Fn2::F92(v1, v0.clone()), v2, (0u64, (0u64, List::<Bytes>::nil())))?;
        h344
    };
    let v4: u64 = {
        let (h345, _) = {
            let (_, h346) = v3.clone();
            h346
        };
        h345
    };
    Ok(f17(List::cons(f18(v0, v4, {
        let a347 = v2;
        let a348 = v4;
        nat_sub(a347, a348)
    }), {
        let (_, h349) = {
            let (_, h350) = v3;
            h350
        };
        h349
    })))
}

pub fn f86(v0: Bytes, v1: u64, v2: Bytes, v3: u64, v4: u64) -> R<Option<u64>> {
    if f82(v2, v0, v4) {
        Ok(None::<u64>)
    } else {
        let a353 = {
            let a351 = v4;
            let a352 = v1;
            nat_add(a351, a352)?
        };
        let a354 = v3;
        if nat_lt(a353, a354) {
            Ok(Some({
                let a355 = v4;
                let a356 = 1u64;
                nat_add(a355, a356)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f87(v0: Bytes) -> bool {
    let a358 = {
        let a357 = v0.clone();
        length_bytes(a357)
    };
    let a359 = 20u64;
    if nat_lt(a358, a359) {
        true
    } else {
        let a361 = {
            let a360 = v0.clone();
            length_bytes(a360)
        };
        let a362 = 20u64;
        if nat_eq(a361, a362) {
            let a369 = {
                let a367 = {
                    let a363 = v0;
                    let a364 = f7(Str::lit("18446744073709551615"));
                    compare_bytes(a363, a364)
                };
                let a368 = {
                    let a365 = Bytes::lit(&[0x01]);
                    let a366 = Bytes::lit(&[0x00]);
                    compare_bytes(a365, a366)
                };
                equal(a367, a368)
            };
            bool_not(a369)
        } else {
            false
        }
    }
}

pub fn f88(v0: Bytes) -> R<bool> {
    let a373 = {
        let (h370, _) = f38(Fn7::F93(v0.clone()), {
            let a371 = v0.clone();
            length_bytes(a371)
        }, 0u64)?;
        h370
    };
    let a374 = {
        let a372 = v0;
        length_bytes(a372)
    };
    Ok(nat_eq(a373, a374))
}

pub fn f89(v0: u64, v1: u8) -> u64 {
    let a375 = v0;
    let a376 = 2u64;
    if nat_eq(a375, a376) {
        2u64
    } else {
        let a377 = v1;
        let a378 = 46u8;
        if equal(a377, a378) {
            let a379 = v0;
            let a380 = 1u64;
            if nat_eq(a379, a380) {
                0u64
            } else {
                2u64
            }
        } else {
            if f94(v1) {
                1u64
            } else {
                let a381 = v0;
                let a382 = 1u64;
                if nat_eq(a381, a382) {
                    if f95(v1) {
                        1u64
                    } else {
                        2u64
                    }
                } else {
                    2u64
                }
            }
        }
    }
}

pub fn f90(v0: Bytes, v1: Bytes, v2: u64) -> Option<u64> {
    if f82(v0, v1, v2) {
        None::<u64>
    } else {
        let a383 = 0u64;
        let a384 = v2;
        if nat_lt(a383, a384) {
            Some({
                let a385 = v2;
                let a386 = 1u64;
                nat_sub(a385, a386)
            })
        } else {
            None::<u64>
        }
    }
}

pub fn f91(v0: Result<List<Bytes>, Bytes>, v1: Bytes) -> R<Result<List<Bytes>, Bytes>> {
    let m387 = v0.clone();
    match m387 {
        Ok(v3) => {
            let v4: Bytes = f32(v1)?;
            if f61(v4.clone())? {
                let m388 = f96(v4.clone(), v3);
                match m388 {
                    None => {
                        Ok(Err::<List<Bytes>, Bytes>(f6(f7(Str::lit("duplicate axiom name ")), f40(v4))))
                    }
                    Some(v5) => {
                        Ok(Ok::<List<Bytes>, Bytes>(v5))
                    }
                }
            } else {
                Ok(Err::<List<Bytes>, Bytes>(f60(v4)))
            }
        }
        Err(_) => {
            Ok(v0)
        }
    }
}

pub fn f92(v0: u8, v1: Bytes, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f97(v1, v0, v2)
}

pub fn f93(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f98(v0, v1)
}

pub fn f94(v0: u8) -> bool {
    if f79(v0, 65u8, 90u8) {
        true
    } else {
        if f79(v0, 97u8, 122u8) {
            true
        } else {
            let a389 = v0;
            let a390 = 95u8;
            equal(a389, a390)
        }
    }
}

pub fn f95(v0: u8) -> bool {
    if f94(v0) {
        true
    } else {
        if f79(v0, 48u8, 57u8) {
            true
        } else {
            let a391 = v0;
            let a392 = 63u8;
            if equal(a391, a392) {
                true
            } else {
                let a393 = v0;
                let a394 = 39u8;
                equal(a393, a394)
            }
        }
    }
}

pub fn f96(v0: Bytes, v1: List<Bytes>) -> Option<List<Bytes>> {
    let m395 = v1;
    match m395.uncons() {
        None => {
            Some(List::cons(v0, List::<Bytes>::nil()))
        }
        Some((v2, v3)) => {
            let a396 = v0.clone();
            let a397 = v2.clone();
            if equal(a396, a397) {
                None::<List<Bytes>>
            } else {
                if f54(v0.clone(), v2.clone()) {
                    Some(List::cons(v0, List::cons(v2, v3)))
                } else {
                    let m398 = f96(v0, v3);
                    match m398 {
                        None => {
                            None::<List<Bytes>>
                        }
                        Some(v4) => {
                            Some(List::cons(v2, v4))
                        }
                    }
                }
            }
        }
    }
}

pub fn f97(v0: Bytes, v1: u8, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v3: u64 = {
        let (h399, _) = v2.clone();
        h399
    };
    let v4: u64 = {
        let (h400, _) = {
            let (_, h401) = v2.clone();
            h401
        };
        h400
    };
    let v5: List<Bytes> = {
        let (_, h402) = {
            let (_, h403) = v2;
            h403
        };
        h402
    };
    let m404 = f51(v0.clone(), v3);
    match m404 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v6) => {
            let a405 = v6;
            let a406 = v1;
            if equal(a405, a406) {
                Ok(Some(({
                    let a407 = v3;
                    let a408 = 1u64;
                    nat_add(a407, a408)?
                }, ({
                    let a409 = v3;
                    let a410 = 1u64;
                    nat_add(a409, a410)?
                }, List::cons(f18(v0, v4, {
                    let a411 = v3;
                    let a412 = v4;
                    nat_sub(a411, a412)
                }), v5)))))
            } else {
                Ok(Some(({
                    let a413 = v3;
                    let a414 = 1u64;
                    nat_add(a413, a414)?
                }, (v4, v5))))
            }
        }
    }
}

pub fn f98(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f70(v0, v1, 48u8, 57u8) {
        Ok(Some({
            let a415 = v1;
            let a416 = 1u64;
            nat_add(a415, a416)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn run(p0: Bytes, p1: List<Bytes>) -> R<Result<List<(Bytes, List<Bytes>)>, Adt0>> {
    f0(p0, p1)
}
