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
    C0(Adt0),
    C1(Adt0),
}

#[derive(Clone)]
pub enum Adt2 {
    C0,
    C1(u64),
}

#[derive(Clone)]
pub enum Adt3 {
    C0((Adt0, u64), List<Adt2>, Adt4),
}

#[derive(Clone)]
pub enum Adt4 {
    C0(u64, u64, u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Fn0 {
    F16,
}

impl Fn0 {
    pub fn apply(&self, p0: (u64, (u64, i64)), p1: i64) -> R<(u64, (u64, i64))> {
        match self {
            Fn0::F16 => f16(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F23(Bytes),
    F24(u64, Bytes),
    F69(Bytes, u64, Bytes, u64),
}

impl Fn1 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn1::F23(k0) => Ok(f23(k0.clone(), p0)),
            Fn1::F24(k0, k1) => f24(*k0, k1.clone(), p0),
            Fn1::F69(k0, k1, k2, k3) => f69(k0.clone(), *k1, k2.clone(), *k3, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F35,
}

impl Fn2 {
    pub fn apply(&self, p0: (u64, (i64, i64)), p1: i64) -> R<(u64, (i64, i64))> {
        match self {
            Fn2::F35 => f35(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F41(List<i64>),
}

impl Fn3 {
    pub fn apply(&self, p0: (List<i64>, List<i64>), p1: List<i64>) -> R<(List<i64>, List<i64>)> {
        match self {
            Fn3::F41(k0) => f41(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F47,
}

impl Fn4 {
    pub fn apply(&self, p0: Adt3) -> R<Option<Adt3>> {
        match self {
            Fn4::F47 => f47(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F52,
}

impl Fn5 {
    pub fn apply(&self, p0: (i64, List<i64>), p1: i64) -> R<(i64, List<i64>)> {
        match self {
            Fn5::F52 => f52(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F63((Adt0, u64)),
}

impl Fn6 {
    pub fn apply(&self, p0: Option<Adt2>, p1: u64) -> Option<Adt2> {
        match self {
            Fn6::F63(k0) => f63(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F68((Adt0, u64)),
}

impl Fn7 {
    pub fn apply(&self, p0: (bool, u64), p1: u64) -> R<(bool, u64)> {
        match self {
            Fn7::F68(k0) => f68(k0.clone(), p0, p1),
        }
    }
}

pub fn f0(v0: Bytes) -> R<Bytes> {
    let m1 = {
        let v1: Bytes = f1(v0)?;
        if f2(v1.clone()) {
            let v2: List<i64> = f3(v1)?;
            Ok::<List<i64>, (bool, bool)>(v2)
        } else {
            Err::<List<i64>, (bool, bool)>((false, false))
        }
    };
    match m1 {
        Ok(v3) => {
            let v4: Adt0 = f4(f5(v3.clone())?);
            let m2 = f6(if f7(f8(f9(), v3)?) {
                Adt1::C0(v4)
            } else {
                Adt1::C1(v4)
            });
            match m2 {
                None => {
                    Ok(Bytes::lit(&[0x68, 0x75, 0x6d, 0x61, 0x6e]))
                }
                Some(v5) => {
                    f10(v5)
                }
            }
        }
        Err(_) => {
            Ok(Bytes::lit(&[0x72, 0x65, 0x66, 0x75, 0x73, 0x65, 0x64]))
        }
    }
}

pub fn f1(v0: Bytes) -> R<Bytes> {
    let v1: u64 = f11(v0.clone())?;
    let v2: u64 = f12(v0.clone(), v1)?;
    Ok(f13(v0, v2, {
        let a3 = v1;
        let a4 = v2;
        nat_sub(a3, a4)
    }))
}

pub fn f2(v0: Bytes) -> bool {
    let a6 = 0u64;
    let a7 = {
        let a5 = v0;
        length_bytes(a5)
    };
    nat_lt(a6, a7)
}

pub fn f3(v0: Bytes) -> R<List<i64>> {
    f14(v0)
}

pub fn f4(v0: u64) -> Adt0 {
    let a8 = v0;
    let a9 = 0u64;
    if nat_eq(a8, a9) {
        Adt0::C0
    } else {
        let a10 = v0;
        let a11 = 1u64;
        if nat_eq(a10, a11) {
            Adt0::C1
        } else {
            Adt0::C2
        }
    }
}

pub fn f5(v0: List<i64>) -> R<u64> {
    let (h12, _) = {
        let (_, h13) = f15(Fn0::F16, (0u64, (0u64, 0i64)), v0)?;
        h13
    };
    Ok(h12)
}

pub fn f6(v0: Adt1) -> Option<Adt0> {
    f17(v0)
}

pub fn f7(v0: Option<u64>) -> bool {
    let m14 = v0;
    match m14 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f8(v0: i64, v1: List<i64>) -> R<Option<u64>> {
    let a16 = {
        let a15 = v1.clone();
        length_list(a15)
    };
    let a17 = 2u64;
    if nat_lt(a16, a17) {
        Ok(None::<u64>)
    } else {
        let m20 = {
            let a18 = f18(v1.clone())?;
            let a19 = v0;
            compare(a18, a19)
        };
        if match m20 {
            Ordering::Less => {
                true
            }
            Ordering::Equal => {
                false
            }
            Ordering::Greater => {
                false
            }
        } {
            Ok(None::<u64>)
        } else {
            Ok(Some(f5(v1)?))
        }
    }
}

pub fn f9() -> i64 {
    2i64
}

pub fn f10(v0: Adt0) -> R<Bytes> {
    let a23 = f19(v0.clone());
    let a24 = {
        let a21 = Bytes::lit(&[0x3a]);
        let a22 = f20(f21(v0)?);
        append_bytes(a21, a22)
    };
    Ok(append_bytes(a23, a24))
}

pub fn f11(v0: Bytes) -> R<u64> {
    let (h25, _) = f22(Fn1::F23(v0.clone()), {
        let a26 = v0.clone();
        length_bytes(a26)
    }, {
        let a27 = v0;
        length_bytes(a27)
    })?;
    Ok(h25)
}

pub fn f12(v0: Bytes, v1: u64) -> R<u64> {
    let (h28, _) = f22(Fn1::F24(v1, v0), v1, 0u64)?;
    Ok(h28)
}

pub fn f13(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m32 = {
        let a29 = v0;
        let a30 = v1;
        let a31 = v2;
        slice_bytes(a29, a30, a31)
    };
    match m32 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f14(v0: Bytes) -> R<List<i64>> {
    let v1: List<i64> = f25(v0)?;
    Ok(v1)
}

pub fn f15(v0: Fn0, v1: (u64, (u64, i64)), v2: List<i64>) -> R<(u64, (u64, i64))> {
    let m33 = v2;
    match m33.uncons() {
        None => {
            Ok(v1)
        }
        Some((v3, v4)) => {
            f15(v0.clone(), {
                let c34: Fn0 = v0;
                c34.apply(v1, v3)?
            }, v4)
        }
    }
}

pub fn f16(v0: (u64, (u64, i64)), v1: i64) -> R<(u64, (u64, i64))> {
    f26(v0, v1)
}

pub fn f17(v0: Adt1) -> Option<Adt0> {
    f27(v0)
}

pub fn f18(v0: List<i64>) -> R<i64> {
    let a36 = {
        let a35 = v0.clone();
        length_list(a35)
    };
    let a37 = 2u64;
    if nat_lt(a36, a37) {
        Ok(0i64)
    } else {
        let a40 = {
            let (h38, _) = f28(v0.clone())?;
            h38
        };
        let a41 = {
            let (_, h39) = f28(v0)?;
            h39
        };
        int_sub(a40, a41)
    }
}

pub fn f19(v0: Adt0) -> Bytes {
    let m42 = v0;
    match m42 {
        Adt0::C0 => {
            Bytes::lit(&[0x6f, 0x75, 0x74, 0x61, 0x67, 0x65])
        }
        Adt0::C1 => {
            Bytes::lit(&[0x62, 0x69, 0x6c, 0x6c, 0x69, 0x6e, 0x67])
        }
        Adt0::C2 => {
            Bytes::lit(&[0x61, 0x63, 0x63, 0x65, 0x73, 0x73])
        }
    }
}

pub fn f20(v0: u64) -> Bytes {
    let a43 = v0;
    let a44 = 0u64;
    if nat_eq(a43, a44) {
        Bytes::lit(&[0x30])
    } else {
        let a45 = v0;
        let a46 = 1u64;
        if nat_eq(a45, a46) {
            Bytes::lit(&[0x31])
        } else {
            let a47 = v0;
            let a48 = 2u64;
            if nat_eq(a47, a48) {
                Bytes::lit(&[0x32])
            } else {
                let a49 = v0;
                let a50 = 3u64;
                if nat_eq(a49, a50) {
                    Bytes::lit(&[0x33])
                } else {
                    Bytes::lit(&[0x2b])
                }
            }
        }
    }
}

pub fn f21(v0: Adt0) -> R<u64> {
    let m51 = f29((v0.clone(), f30(v0)))?;
    match m51 {
        Ok(v1) => {
            let a53 = {
                let (_, h52) = v1;
                h52
            };
            Ok(length_list(a53))
        }
        Err(_) => {
            Ok(0u64)
        }
    }
}

pub fn f22(v0: Fn1, v1: u64, v2: u64) -> R<(u64, bool)> {
    let m54 = v1;
    if m54 == 0 {
        Ok((v2, false))
    } else {
        let v3 = m54 - 1;
        let m56 = {
            let c55: Fn1 = v0.clone();
            c55.apply(v2)?
        };
        match m56 {
            None => {
                Ok((v2, true))
            }
            Some(v4) => {
                f22(v0, v3, v4)
            }
        }
    }
}

pub fn f23(v0: Bytes, v1: u64) -> Option<u64> {
    let a57 = 0u64;
    let a58 = v1;
    if nat_lt(a57, a58) {
        if f31(v0, {
            let a59 = v1;
            let a60 = 1u64;
            nat_sub(a59, a60)
        }) {
            Some({
                let a61 = v1;
                let a62 = 1u64;
                nat_sub(a61, a62)
            })
        } else {
            None::<u64>
        }
    } else {
        None::<u64>
    }
}

pub fn f24(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    let a63 = v2;
    let a64 = v0;
    if nat_lt(a63, a64) {
        if f31(v1, v2) {
            Ok(Some({
                let a65 = v2;
                let a66 = 1u64;
                nat_add(a65, a66)?
            }))
        } else {
            Ok(None::<u64>)
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f25(v0: Bytes) -> R<List<i64>> {
    f32(f33(v0)?)
}

pub fn f26(v0: (u64, (u64, i64)), v1: i64) -> R<(u64, (u64, i64))> {
    let a68 = {
        let (h67, _) = v0;
        h67
    };
    let a69 = 0u64;
    if nat_eq(a68, a69) {
        Ok((1u64, (0u64, v1)))
    } else {
        let m74 = {
            let a72 = {
                let (_, h70) = {
                    let (_, h71) = v0;
                    h71
                };
                h70
            };
            let a73 = v1;
            compare(a72, a73)
        };
        if match m74 {
            Ordering::Less => {
                true
            }
            Ordering::Equal => {
                false
            }
            Ordering::Greater => {
                false
            }
        } {
            Ok(({
                let a76 = {
                    let (h75, _) = v0;
                    h75
                };
                let a77 = 1u64;
                nat_add(a76, a77)?
            }, ({
                let (h78, _) = v0;
                h78
            }, v1)))
        } else {
            Ok(({
                let a80 = {
                    let (h79, _) = v0;
                    h79
                };
                let a81 = 1u64;
                nat_add(a80, a81)?
            }, {
                let (_, h82) = v0;
                h82
            }))
        }
    }
}

pub fn f27(v0: Adt1) -> Option<Adt0> {
    let m83 = v0;
    match m83 {
        Adt1::C0(v1) => {
            Some(v1)
        }
        Adt1::C1(_) => {
            None::<Adt0>
        }
    }
}

pub fn f28(v0: List<i64>) -> R<(i64, i64)> {
    let (_, h84) = f34(Fn2::F35, (0u64, (0i64, 0i64)), v0)?;
    Ok(h84)
}

pub fn f29(v0: (Adt0, u64)) -> R<Result<(u64, List<Adt2>), (bool, bool)>> {
    let v1: (Adt3, bool) = f36(v0.clone())?;
    let m86 = {
        let (_, h85) = v1.clone();
        h85
    };
    if m86 {
        let m90 = f37(v0, {
            let m89 = {
                let (h88, _) = v1.clone();
                h88
            };
            match m89 {
                Adt3::C0(h87, _, _) => {
                    h87
                }
            }
        });
        match m90 {
            Ok(v2) => {
                Ok(Ok::<(u64, List<Adt2>), (bool, bool)>((v2, {
                    let m93 = {
                        let (h92, _) = v1;
                        h92
                    };
                    match m93 {
                        Adt3::C0(_, h91, _) => {
                            h91
                        }
                    }
                })))
            }
            Err(v3) => {
                Ok(Err::<(u64, List<Adt2>), (bool, bool)>(v3))
            }
        }
    } else {
        Ok(Err::<(u64, List<Adt2>), (bool, bool)>((false, false)))
    }
}

pub fn f30(v0: Adt0) -> u64 {
    let m94 = v0;
    match m94 {
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

pub fn f31(v0: Bytes, v1: u64) -> bool {
    let m95 = f38(v0, v1);
    match m95 {
        None => {
            false
        }
        Some(v2) => {
            f39(v2)
        }
    }
}

pub fn f32(v0: List<i64>) -> R<List<i64>> {
    let (h96, _) = f40(Fn3::F41(v0), (List::<i64>::nil(), f42()), f43())?;
    Ok(h96)
}

pub fn f33(v0: Bytes) -> R<List<i64>> {
    f44(v0)
}

pub fn f34(v0: Fn2, v1: (u64, (i64, i64)), v2: List<i64>) -> R<(u64, (i64, i64))> {
    let m97 = v2;
    match m97.uncons() {
        None => {
            Ok(v1)
        }
        Some((v3, v4)) => {
            f34(v0.clone(), {
                let c98: Fn2 = v0;
                c98.apply(v1, v3)?
            }, v4)
        }
    }
}

pub fn f35(v0: (u64, (i64, i64)), v1: i64) -> R<(u64, (i64, i64))> {
    f45(v0, v1)
}

pub fn f36(v0: (Adt0, u64)) -> R<(Adt3, bool)> {
    f46(Fn4::F47, {
        let a100 = {
            let (_, h99) = v0.clone();
            h99
        };
        let a101 = 1u64;
        nat_add(a100, a101)?
    }, f48(v0))
}

pub fn f37(v0: (Adt0, u64), v1: (Adt0, u64)) -> Result<u64, (bool, bool)> {
    let m102 = f49(v0, v1.clone());
    match m102 {
        None => {
            let m103 = f50(v1);
            match m103 {
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

pub fn f38(v0: Bytes, v1: u64) -> Option<u8> {
    let a104 = v0;
    let a105 = v1;
    index_bytes(a104, a105)
}

pub fn f39(v0: u8) -> bool {
    let a106 = v0;
    let a107 = 32u8;
    if if if equal(a106, a107) {
        true
    } else {
        let a108 = v0;
        let a109 = 9u8;
        equal(a108, a109)
    } {
        true
    } else {
        let a110 = v0;
        let a111 = 10u8;
        equal(a110, a111)
    } {
        true
    } else {
        let a112 = v0;
        let a113 = 12u8;
        if equal(a112, a113) {
            true
        } else {
            let a114 = v0;
            let a115 = 13u8;
            equal(a114, a115)
        }
    }
}

pub fn f40(v0: Fn3, v1: (List<i64>, List<i64>), v2: List<List<i64>>) -> R<(List<i64>, List<i64>)> {
    let m116 = v2;
    match m116.uncons() {
        None => {
            Ok(v1)
        }
        Some((v3, v4)) => {
            f40(v0.clone(), {
                let c117: Fn3 = v0;
                c117.apply(v1, v3)?
            }, v4)
        }
    }
}

pub fn f41(v0: List<i64>, v1: (List<i64>, List<i64>), v2: List<i64>) -> R<(List<i64>, List<i64>)> {
    let m119 = {
        let (_, h118) = v1.clone();
        h118
    };
    match m119.uncons() {
        None => {
            Ok(v1)
        }
        Some((v3, v4)) => {
            Ok(({
                let a125 = {
                    let (h120, _) = v1;
                    h120
                };
                let a126 = List::cons({
                    let a123 = v3;
                    let a124 = {
                        let a122 = {
                            let (h121, _) = f51(Fn5::F52, (0i64, v0), v2)?;
                            h121
                        };
                        int_neg(a122)?
                    };
                    int_sub(a123, a124)?
                }, List::<i64>::nil());
                append_list(a125, a126)
            }, v4))
        }
    }
}

pub fn f42() -> List<i64> {
    List::cons(0i64, List::cons(0i64, List::cons(0i64, List::<i64>::nil())))
}

pub fn f43() -> List<List<i64>> {
    List::cons(List::cons(3i64, List::cons(0i64, List::cons(0i64, List::<i64>::nil()))), List::cons(List::cons(0i64, List::cons(3i64, List::cons(0i64, List::<i64>::nil()))), List::cons(List::cons(0i64, List::cons(0i64, List::cons(3i64, List::<i64>::nil()))), List::<List<i64>>::nil())))
}

pub fn f44(v0: Bytes) -> R<List<i64>> {
    Ok(List::cons(f53(if f54(v0.clone(), Bytes::lit(&[0x64, 0x6f, 0x77, 0x6e]))? {
        true
    } else {
        f54(v0.clone(), Bytes::lit(&[0x6f, 0x75, 0x74, 0x61, 0x67, 0x65]))?
    }), List::cons(f53(if f54(v0.clone(), Bytes::lit(&[0x69, 0x6e, 0x76, 0x6f, 0x69, 0x63, 0x65]))? {
        true
    } else {
        f54(v0.clone(), Bytes::lit(&[0x72, 0x65, 0x66, 0x75, 0x6e, 0x64]))?
    }), List::cons(f53(if f54(v0.clone(), Bytes::lit(&[0x70, 0x61, 0x73, 0x73, 0x77, 0x6f, 0x72, 0x64]))? {
        true
    } else {
        f54(v0, Bytes::lit(&[0x6c, 0x6f, 0x67, 0x69, 0x6e]))?
    }), List::<i64>::nil()))))
}

pub fn f45(v0: (u64, (i64, i64)), v1: i64) -> R<(u64, (i64, i64))> {
    let a128 = {
        let (h127, _) = v0;
        h127
    };
    let a129 = 0u64;
    if nat_eq(a128, a129) {
        Ok((1u64, (v1, 0i64)))
    } else {
        let a131 = {
            let (h130, _) = v0;
            h130
        };
        let a132 = 1u64;
        if nat_eq(a131, a132) {
            let m137 = {
                let a135 = {
                    let (h133, _) = {
                        let (_, h134) = v0;
                        h134
                    };
                    h133
                };
                let a136 = v1;
                compare(a135, a136)
            };
            if match m137 {
                Ordering::Less => {
                    true
                }
                Ordering::Equal => {
                    false
                }
                Ordering::Greater => {
                    false
                }
            } {
                Ok((2u64, (v1, {
                    let (h138, _) = {
                        let (_, h139) = v0;
                        h139
                    };
                    h138
                })))
            } else {
                Ok((2u64, ({
                    let (h140, _) = {
                        let (_, h141) = v0;
                        h141
                    };
                    h140
                }, v1)))
            }
        } else {
            let m146 = {
                let a144 = {
                    let (h142, _) = {
                        let (_, h143) = v0;
                        h143
                    };
                    h142
                };
                let a145 = v1;
                compare(a144, a145)
            };
            if match m146 {
                Ordering::Less => {
                    true
                }
                Ordering::Equal => {
                    false
                }
                Ordering::Greater => {
                    false
                }
            } {
                Ok(({
                    let a148 = {
                        let (h147, _) = v0;
                        h147
                    };
                    let a149 = 1u64;
                    nat_add(a148, a149)?
                }, (v1, {
                    let (h150, _) = {
                        let (_, h151) = v0;
                        h151
                    };
                    h150
                })))
            } else {
                Ok(({
                    let a153 = {
                        let (h152, _) = v0;
                        h152
                    };
                    let a154 = 1u64;
                    nat_add(a153, a154)?
                }, ({
                    let (h155, _) = {
                        let (_, h156) = v0;
                        h156
                    };
                    h155
                }, {
                    let m161 = {
                        let a159 = {
                            let (_, h157) = {
                                let (_, h158) = v0;
                                h158
                            };
                            h157
                        };
                        let a160 = v1;
                        compare(a159, a160)
                    };
                    if match m161 {
                        Ordering::Less => {
                            true
                        }
                        Ordering::Equal => {
                            false
                        }
                        Ordering::Greater => {
                            false
                        }
                    } {
                        v1
                    } else {
                        let (_, h162) = {
                            let (_, h163) = v0;
                            h163
                        };
                        h162
                    }
                })))
            }
        }
    }
}

pub fn f46(v0: Fn4, v1: u64, v2: Adt3) -> R<(Adt3, bool)> {
    let m164 = v1;
    if m164 == 0 {
        Ok((v2, false))
    } else {
        let v3 = m164 - 1;
        let m166 = {
            let c165: Fn4 = v0.clone();
            c165.apply(v2.clone())?
        };
        match m166 {
            None => {
                Ok((v2, true))
            }
            Some(v4) => {
                f46(v0, v3, v4)
            }
        }
    }
}

pub fn f47(v0: Adt3) -> R<Option<Adt3>> {
    let m168 = f55(match v0.clone() {
        Adt3::C0(h167, _, _) => {
            h167
        }
    });
    match m168 {
        None => {
            Ok(None::<Adt3>)
        }
        Some(v1) => {
            let m170 = f56(match v0.clone() {
                Adt3::C0(h169, _, _) => {
                    h169
                }
            }, v1.clone());
            match m170 {
                None => {
                    Ok(None::<Adt3>)
                }
                Some(v2) => {
                    Ok(Some(Adt3::C0(v2, {
                        let a172 = match v0.clone() {
                            Adt3::C0(_, h171, _) => {
                                h171
                            }
                        };
                        let a173 = List::cons(v1, List::<Adt2>::nil());
                        append_list(a172, a173)
                    }, Adt4::C0({
                        let a177 = {
                            let m176 = match v0.clone() {
                                Adt3::C0(_, _, h175) => {
                                    h175
                                }
                            };
                            match m176 {
                                Adt4::C0(h174, _, _, _, _, _) => {
                                    h174
                                }
                            }
                        };
                        let a178 = 1u64;
                        nat_add(a177, a178)?
                    }, {
                        let a183 = {
                            let m181 = match v0.clone() {
                                Adt3::C0(_, _, h180) => {
                                    h180
                                }
                            };
                            match m181 {
                                Adt4::C0(_, h179, _, _, _, _) => {
                                    h179
                                }
                            }
                        };
                        let a184 = f57(match v0.clone() {
                            Adt3::C0(h182, _, _) => {
                                h182
                            }
                        })?;
                        nat_add(a183, a184)?
                    }, {
                        let a188 = {
                            let m187 = match v0.clone() {
                                Adt3::C0(_, _, h186) => {
                                    h186
                                }
                            };
                            match m187 {
                                Adt4::C0(_, _, h185, _, _, _) => {
                                    h185
                                }
                            }
                        };
                        let a189 = 1u64;
                        nat_add(a188, a189)?
                    }, {
                        let m192 = match v0.clone() {
                            Adt3::C0(_, _, h191) => {
                                h191
                            }
                        };
                        match m192 {
                            Adt4::C0(_, _, _, h190, _, _) => {
                                h190
                            }
                        }
                    }, {
                        let m195 = match v0.clone() {
                            Adt3::C0(_, _, h194) => {
                                h194
                            }
                        };
                        match m195 {
                            Adt4::C0(_, _, _, _, h193, _) => {
                                h193
                            }
                        }
                    }, {
                        let m198 = match v0 {
                            Adt3::C0(_, _, h197) => {
                                h197
                            }
                        };
                        match m198 {
                            Adt4::C0(_, _, _, _, _, h196) => {
                                h196
                            }
                        }
                    }))))
                }
            }
        }
    }
}

pub fn f48(v0: (Adt0, u64)) -> Adt3 {
    Adt3::C0(f58(v0), List::<Adt2>::nil(), Adt4::C0(0u64, 0u64, 0u64, 0u64, 0u64, 0u64))
}

pub fn f49(v0: (Adt0, u64), v1: (Adt0, u64)) -> Option<u64> {
    let m199 = f50(v1);
    match m199 {
        None => {
            None::<u64>
        }
        Some(v2) => {
            if f59(v0, v2) {
                Some(v2)
            } else {
                None::<u64>
            }
        }
    }
}

pub fn f50(v0: (Adt0, u64)) -> Option<u64> {
    Some({
        let (_, h200) = v0;
        h200
    })
}

pub fn f51(v0: Fn5, v1: (i64, List<i64>), v2: List<i64>) -> R<(i64, List<i64>)> {
    let m201 = v2;
    match m201.uncons() {
        None => {
            Ok(v1)
        }
        Some((v3, v4)) => {
            f51(v0.clone(), {
                let c202: Fn5 = v0;
                c202.apply(v1, v3)?
            }, v4)
        }
    }
}

pub fn f52(v0: (i64, List<i64>), v1: i64) -> R<(i64, List<i64>)> {
    let m204 = {
        let (_, h203) = v0.clone();
        h203
    };
    match m204.uncons() {
        None => {
            Ok(v0)
        }
        Some((v2, v3)) => {
            Ok(({
                let a209 = {
                    let (h205, _) = v0;
                    h205
                };
                let a210 = {
                    let a208 = {
                        let a206 = v1;
                        let a207 = v2;
                        int_mul(a206, a207)?
                    };
                    int_neg(a208)?
                };
                int_sub(a209, a210)?
            }, v3))
        }
    }
}

pub fn f53(v0: bool) -> i64 {
    if v0 {
        1i64
    } else {
        0i64
    }
}

pub fn f54(v0: Bytes, v1: Bytes) -> R<bool> {
    let m211 = f60(v0, v1)?;
    match m211 {
        None => {
            Ok(false)
        }
        Some(_) => {
            Ok(true)
        }
    }
}

pub fn f55(v0: (Adt0, u64)) -> Option<Adt2> {
    let m212 = if f61(v0.clone()) {
        Some(Adt2::C0)
    } else {
        None::<Adt2>
    };
    match m212 {
        None => {
            f62(Fn6::F63(v0.clone()), None::<Adt2>, f64(v0))
        }
        Some(v1) => {
            Some(v1)
        }
    }
}

pub fn f56(v0: (Adt0, u64), v1: Adt2) -> Option<(Adt0, u64)> {
    let m213 = v1;
    match m213 {
        Adt2::C0 => {
            f65(v0)
        }
        Adt2::C1(v2) => {
            f66(v0, v2)
        }
    }
}

pub fn f57(v0: (Adt0, u64)) -> R<u64> {
    let (_, h214) = {
        let v1: (bool, u64) = (f61(v0.clone()), 1u64);
        let (h215, _) = v1;
        if h215 {
            v1
        } else {
            let v2: (bool, u64) = f67(Fn7::F68(v0.clone()), (false, 0u64), f64(v0))?;
            ({
                let (h216, _) = v2;
                h216
            }, {
                let a219 = {
                    let (_, h217) = v1;
                    h217
                };
                let a220 = {
                    let (_, h218) = v2;
                    h218
                };
                nat_add(a219, a220)?
            })
        }
    };
    Ok(h214)
}

pub fn f58(v0: (Adt0, u64)) -> (Adt0, u64) {
    v0
}

pub fn f59(_: (Adt0, u64), v1: u64) -> bool {
    let a221 = v1;
    let a222 = 0u64;
    nat_eq(a221, a222)
}

pub fn f60(v0: Bytes, v1: Bytes) -> R<Option<u64>> {
    let v2: u64 = {
        let a223 = v0.clone();
        length_bytes(a223)
    };
    let v3: u64 = {
        let a224 = v1.clone();
        length_bytes(a224)
    };
    let v4: u64 = {
        let (h225, _) = f22(Fn1::F69(v1.clone(), v3, v0.clone(), v2), {
            let a226 = v2;
            let a227 = 1u64;
            nat_add(a226, a227)?
        }, 0u64)?;
        h225
    };
    if f70(v0, v1, v4) {
        Ok(Some(v4))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f61(v0: (Adt0, u64)) -> bool {
    let a229 = 2u64;
    let a230 = {
        let (_, h228) = v0;
        h228
    };
    nat_lt(a229, a230)
}

pub fn f62(v0: Fn6, v1: Option<Adt2>, v2: List<u64>) -> Option<Adt2> {
    let m231 = v2;
    match m231.uncons() {
        None => {
            v1
        }
        Some((v3, v4)) => {
            f62(v0.clone(), {
                let c232: Fn6 = v0;
                c232.apply(v1, v3)
            }, v4)
        }
    }
}

pub fn f63(v0: (Adt0, u64), v1: Option<Adt2>, v2: u64) -> Option<Adt2> {
    let m233 = v1;
    match m233 {
        None => {
            if f71(v0, v2) {
                Some(Adt2::C1(v2))
            } else {
                None::<Adt2>
            }
        }
        Some(v3) => {
            Some(v3)
        }
    }
}

pub fn f64(_: (Adt0, u64)) -> List<u64> {
    List::cons(2u64, List::cons(1u64, List::<u64>::nil()))
}

pub fn f65(v0: (Adt0, u64)) -> Option<(Adt0, u64)> {
    if f61(v0.clone()) {
        Some(f72(v0))
    } else {
        None::<(Adt0, u64)>
    }
}

pub fn f66(v0: (Adt0, u64), v1: u64) -> Option<(Adt0, u64)> {
    if f71(v0.clone(), v1) {
        Some(f73(v0, v1))
    } else {
        None::<(Adt0, u64)>
    }
}

pub fn f67(v0: Fn7, v1: (bool, u64), v2: List<u64>) -> R<(bool, u64)> {
    let m234 = v2;
    match m234.uncons() {
        None => {
            Ok(v1)
        }
        Some((v3, v4)) => {
            f67(v0.clone(), {
                let c235: Fn7 = v0;
                c235.apply(v1, v3)?
            }, v4)
        }
    }
}

pub fn f68(v0: (Adt0, u64), v1: (bool, u64), v2: u64) -> R<(bool, u64)> {
    let (h236, _) = v1;
    if h236 {
        Ok(v1)
    } else {
        Ok((f71(v0, v2), {
            let a238 = {
                let (_, h237) = v1;
                h237
            };
            let a239 = 1u64;
            nat_add(a238, a239)?
        }))
    }
}

pub fn f69(v0: Bytes, v1: u64, v2: Bytes, v3: u64, v4: u64) -> R<Option<u64>> {
    if f70(v2, v0, v4) {
        Ok(None::<u64>)
    } else {
        let a242 = {
            let a240 = v4;
            let a241 = v1;
            nat_add(a240, a241)?
        };
        let a243 = v3;
        if nat_lt(a242, a243) {
            Ok(Some({
                let a244 = v4;
                let a245 = 1u64;
                nat_add(a244, a245)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f70(v0: Bytes, v1: Bytes, v2: u64) -> bool {
    let m250 = {
        let a247 = v0;
        let a248 = v2;
        let a249 = {
            let a246 = v1.clone();
            length_bytes(a246)
        };
        slice_bytes(a247, a248, a249)
    };
    match m250 {
        None => {
            false
        }
        Some(v3) => {
            let a251 = v3;
            let a252 = v1;
            equal(a251, a252)
        }
    }
}

pub fn f71(v0: (Adt0, u64), v1: u64) -> bool {
    let a254 = v1;
    let a255 = {
        let (_, h253) = v0;
        h253
    };
    if nat_le(a254, a255) {
        let a256 = 0u64;
        let a257 = v1;
        nat_lt(a256, a257)
    } else {
        false
    }
}

pub fn f72(v0: (Adt0, u64)) -> (Adt0, u64) {
    ({
        let (h258, _) = v0;
        h258
    }, {
        let a260 = {
            let (_, h259) = v0;
            h259
        };
        let a261 = 1u64;
        nat_sub(a260, a261)
    })
}

pub fn f73(v0: (Adt0, u64), v1: u64) -> (Adt0, u64) {
    ({
        let (h262, _) = v0;
        h262
    }, {
        let a264 = {
            let (_, h263) = v0;
            h263
        };
        let a265 = v1;
        nat_sub(a264, a265)
    })
}

pub fn run(p0: Bytes) -> R<Bytes> {
    f0(p0)
}
