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
pub enum Fn0 {
    F13(Bytes),
    F14(u64, Bytes),
    F38(Bytes, u64, Bytes, u64),
}

impl Fn0 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn0::F13(k0) => Ok(f13(k0.clone(), p0)),
            Fn0::F14(k0, k1) => f14(*k0, k1.clone(), p0),
            Fn0::F38(k0, k1, k2, k3) => f38(k0.clone(), *k1, k2.clone(), *k3, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F18,
}

impl Fn1 {
    pub fn apply(&self, p0: (u64, (u64, i64)), p1: i64) -> R<(u64, (u64, i64))> {
        match self {
            Fn1::F18 => f18(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F23,
}

impl Fn2 {
    pub fn apply(&self, p0: (u64, (i64, i64)), p1: i64) -> R<(u64, (i64, i64))> {
        match self {
            Fn2::F23 => f23(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F28(List<i64>),
}

impl Fn3 {
    pub fn apply(&self, p0: (List<i64>, List<i64>), p1: List<i64>) -> R<(List<i64>, List<i64>)> {
        match self {
            Fn3::F28(k0) => f28(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F34,
}

impl Fn4 {
    pub fn apply(&self, p0: (i64, List<i64>), p1: i64) -> R<(i64, List<i64>)> {
        match self {
            Fn4::F34 => f34(p0, p1),
        }
    }
}

pub fn f0(v0: Bytes) -> R<u64> {
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
            let m2 = f4(f5(), v3)?;
            match m2 {
                None => {
                    Ok(8u64)
                }
                Some(v4) => {
                    Ok(v4)
                }
            }
        }
        Err(_) => {
            Ok(9u64)
        }
    }
}

pub fn f1(v0: Bytes) -> R<Bytes> {
    let v1: u64 = f6(v0.clone())?;
    let v2: u64 = f7(v0.clone(), v1)?;
    Ok(f8(v0, v2, {
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
    f9(v0)
}

pub fn f4(v0: i64, v1: List<i64>) -> R<Option<u64>> {
    let a9 = {
        let a8 = v1.clone();
        length_list(a8)
    };
    let a10 = 2u64;
    if nat_lt(a9, a10) {
        Ok(None::<u64>)
    } else {
        let m13 = {
            let a11 = f10(v1.clone())?;
            let a12 = v0;
            compare(a11, a12)
        };
        if match m13 {
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
            Ok(Some(f11(v1)?))
        }
    }
}

pub fn f5() -> i64 {
    2i64
}

pub fn f6(v0: Bytes) -> R<u64> {
    let (h14, _) = f12(Fn0::F13(v0.clone()), {
        let a15 = v0.clone();
        length_bytes(a15)
    }, {
        let a16 = v0;
        length_bytes(a16)
    })?;
    Ok(h14)
}

pub fn f7(v0: Bytes, v1: u64) -> R<u64> {
    let (h17, _) = f12(Fn0::F14(v1, v0), v1, 0u64)?;
    Ok(h17)
}

pub fn f8(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m21 = {
        let a18 = v0;
        let a19 = v1;
        let a20 = v2;
        slice_bytes(a18, a19, a20)
    };
    match m21 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f9(v0: Bytes) -> R<List<i64>> {
    let v1: List<i64> = f15(v0)?;
    Ok(v1)
}

pub fn f10(v0: List<i64>) -> R<i64> {
    let a23 = {
        let a22 = v0.clone();
        length_list(a22)
    };
    let a24 = 2u64;
    if nat_lt(a23, a24) {
        Ok(0i64)
    } else {
        let a27 = {
            let (h25, _) = f16(v0.clone())?;
            h25
        };
        let a28 = {
            let (_, h26) = f16(v0)?;
            h26
        };
        int_sub(a27, a28)
    }
}

pub fn f11(v0: List<i64>) -> R<u64> {
    let (h29, _) = {
        let (_, h30) = f17(Fn1::F18, (0u64, (0u64, 0i64)), v0)?;
        h30
    };
    Ok(h29)
}

pub fn f12(v0: Fn0, v1: u64, v2: u64) -> R<(u64, bool)> {
    let m31 = v1;
    if m31 == 0 {
        Ok((v2, false))
    } else {
        let v3 = m31 - 1;
        let m33 = {
            let c32: Fn0 = v0.clone();
            c32.apply(v2)?
        };
        match m33 {
            None => {
                Ok((v2, true))
            }
            Some(v4) => {
                f12(v0, v3, v4)
            }
        }
    }
}

pub fn f13(v0: Bytes, v1: u64) -> Option<u64> {
    let a34 = 0u64;
    let a35 = v1;
    if nat_lt(a34, a35) {
        if f19(v0, {
            let a36 = v1;
            let a37 = 1u64;
            nat_sub(a36, a37)
        }) {
            Some({
                let a38 = v1;
                let a39 = 1u64;
                nat_sub(a38, a39)
            })
        } else {
            None::<u64>
        }
    } else {
        None::<u64>
    }
}

pub fn f14(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    let a40 = v2;
    let a41 = v0;
    if nat_lt(a40, a41) {
        if f19(v1, v2) {
            Ok(Some({
                let a42 = v2;
                let a43 = 1u64;
                nat_add(a42, a43)?
            }))
        } else {
            Ok(None::<u64>)
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f15(v0: Bytes) -> R<List<i64>> {
    f20(f21(v0)?)
}

pub fn f16(v0: List<i64>) -> R<(i64, i64)> {
    let (_, h44) = f22(Fn2::F23, (0u64, (0i64, 0i64)), v0)?;
    Ok(h44)
}

pub fn f17(v0: Fn1, v1: (u64, (u64, i64)), v2: List<i64>) -> R<(u64, (u64, i64))> {
    let m45 = v2;
    match m45.uncons() {
        None => {
            Ok(v1)
        }
        Some((v3, v4)) => {
            f17(v0.clone(), {
                let c46: Fn1 = v0;
                c46.apply(v1, v3)?
            }, v4)
        }
    }
}

pub fn f18(v0: (u64, (u64, i64)), v1: i64) -> R<(u64, (u64, i64))> {
    f24(v0, v1)
}

pub fn f19(v0: Bytes, v1: u64) -> bool {
    let m47 = f25(v0, v1);
    match m47 {
        None => {
            false
        }
        Some(v2) => {
            f26(v2)
        }
    }
}

pub fn f20(v0: List<i64>) -> R<List<i64>> {
    let (h48, _) = f27(Fn3::F28(v0), (List::<i64>::nil(), f29()), f30())?;
    Ok(h48)
}

pub fn f21(v0: Bytes) -> R<List<i64>> {
    f31(v0)
}

pub fn f22(v0: Fn2, v1: (u64, (i64, i64)), v2: List<i64>) -> R<(u64, (i64, i64))> {
    let m49 = v2;
    match m49.uncons() {
        None => {
            Ok(v1)
        }
        Some((v3, v4)) => {
            f22(v0.clone(), {
                let c50: Fn2 = v0;
                c50.apply(v1, v3)?
            }, v4)
        }
    }
}

pub fn f23(v0: (u64, (i64, i64)), v1: i64) -> R<(u64, (i64, i64))> {
    f32(v0, v1)
}

pub fn f24(v0: (u64, (u64, i64)), v1: i64) -> R<(u64, (u64, i64))> {
    let a52 = {
        let (h51, _) = v0;
        h51
    };
    let a53 = 0u64;
    if nat_eq(a52, a53) {
        Ok((1u64, (0u64, v1)))
    } else {
        let m58 = {
            let a56 = {
                let (_, h54) = {
                    let (_, h55) = v0;
                    h55
                };
                h54
            };
            let a57 = v1;
            compare(a56, a57)
        };
        if match m58 {
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
                let a60 = {
                    let (h59, _) = v0;
                    h59
                };
                let a61 = 1u64;
                nat_add(a60, a61)?
            }, ({
                let (h62, _) = v0;
                h62
            }, v1)))
        } else {
            Ok(({
                let a64 = {
                    let (h63, _) = v0;
                    h63
                };
                let a65 = 1u64;
                nat_add(a64, a65)?
            }, {
                let (_, h66) = v0;
                h66
            }))
        }
    }
}

pub fn f25(v0: Bytes, v1: u64) -> Option<u8> {
    let a67 = v0;
    let a68 = v1;
    index_bytes(a67, a68)
}

pub fn f26(v0: u8) -> bool {
    let a69 = v0;
    let a70 = 32u8;
    if if if equal(a69, a70) {
        true
    } else {
        let a71 = v0;
        let a72 = 9u8;
        equal(a71, a72)
    } {
        true
    } else {
        let a73 = v0;
        let a74 = 10u8;
        equal(a73, a74)
    } {
        true
    } else {
        let a75 = v0;
        let a76 = 12u8;
        if equal(a75, a76) {
            true
        } else {
            let a77 = v0;
            let a78 = 13u8;
            equal(a77, a78)
        }
    }
}

pub fn f27(v0: Fn3, v1: (List<i64>, List<i64>), v2: List<List<i64>>) -> R<(List<i64>, List<i64>)> {
    let m79 = v2;
    match m79.uncons() {
        None => {
            Ok(v1)
        }
        Some((v3, v4)) => {
            f27(v0.clone(), {
                let c80: Fn3 = v0;
                c80.apply(v1, v3)?
            }, v4)
        }
    }
}

pub fn f28(v0: List<i64>, v1: (List<i64>, List<i64>), v2: List<i64>) -> R<(List<i64>, List<i64>)> {
    let m82 = {
        let (_, h81) = v1.clone();
        h81
    };
    match m82.uncons() {
        None => {
            Ok(v1)
        }
        Some((v3, v4)) => {
            Ok(({
                let a88 = {
                    let (h83, _) = v1;
                    h83
                };
                let a89 = List::cons({
                    let a86 = v3;
                    let a87 = {
                        let a85 = {
                            let (h84, _) = f33(Fn4::F34, (0i64, v0), v2)?;
                            h84
                        };
                        int_neg(a85)?
                    };
                    int_sub(a86, a87)?
                }, List::<i64>::nil());
                append_list(a88, a89)
            }, v4))
        }
    }
}

pub fn f29() -> List<i64> {
    List::cons(0i64, List::cons(0i64, List::cons(0i64, List::<i64>::nil())))
}

pub fn f30() -> List<List<i64>> {
    List::cons(List::cons(3i64, List::cons(0i64, List::cons(0i64, List::<i64>::nil()))), List::cons(List::cons(0i64, List::cons(3i64, List::cons(0i64, List::<i64>::nil()))), List::cons(List::cons(0i64, List::cons(0i64, List::cons(3i64, List::<i64>::nil()))), List::<List<i64>>::nil())))
}

pub fn f31(v0: Bytes) -> R<List<i64>> {
    Ok(List::cons(f35(if f36(v0.clone(), Bytes::lit(&[0x64, 0x6f, 0x77, 0x6e]))? {
        true
    } else {
        f36(v0.clone(), Bytes::lit(&[0x6f, 0x75, 0x74, 0x61, 0x67, 0x65]))?
    }), List::cons(f35(if f36(v0.clone(), Bytes::lit(&[0x69, 0x6e, 0x76, 0x6f, 0x69, 0x63, 0x65]))? {
        true
    } else {
        f36(v0.clone(), Bytes::lit(&[0x72, 0x65, 0x66, 0x75, 0x6e, 0x64]))?
    }), List::cons(f35(if f36(v0.clone(), Bytes::lit(&[0x70, 0x61, 0x73, 0x73, 0x77, 0x6f, 0x72, 0x64]))? {
        true
    } else {
        f36(v0, Bytes::lit(&[0x6c, 0x6f, 0x67, 0x69, 0x6e]))?
    }), List::<i64>::nil()))))
}

pub fn f32(v0: (u64, (i64, i64)), v1: i64) -> R<(u64, (i64, i64))> {
    let a91 = {
        let (h90, _) = v0;
        h90
    };
    let a92 = 0u64;
    if nat_eq(a91, a92) {
        Ok((1u64, (v1, 0i64)))
    } else {
        let a94 = {
            let (h93, _) = v0;
            h93
        };
        let a95 = 1u64;
        if nat_eq(a94, a95) {
            let m100 = {
                let a98 = {
                    let (h96, _) = {
                        let (_, h97) = v0;
                        h97
                    };
                    h96
                };
                let a99 = v1;
                compare(a98, a99)
            };
            if match m100 {
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
                    let (h101, _) = {
                        let (_, h102) = v0;
                        h102
                    };
                    h101
                })))
            } else {
                Ok((2u64, ({
                    let (h103, _) = {
                        let (_, h104) = v0;
                        h104
                    };
                    h103
                }, v1)))
            }
        } else {
            let m109 = {
                let a107 = {
                    let (h105, _) = {
                        let (_, h106) = v0;
                        h106
                    };
                    h105
                };
                let a108 = v1;
                compare(a107, a108)
            };
            if match m109 {
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
                    let a111 = {
                        let (h110, _) = v0;
                        h110
                    };
                    let a112 = 1u64;
                    nat_add(a111, a112)?
                }, (v1, {
                    let (h113, _) = {
                        let (_, h114) = v0;
                        h114
                    };
                    h113
                })))
            } else {
                Ok(({
                    let a116 = {
                        let (h115, _) = v0;
                        h115
                    };
                    let a117 = 1u64;
                    nat_add(a116, a117)?
                }, ({
                    let (h118, _) = {
                        let (_, h119) = v0;
                        h119
                    };
                    h118
                }, {
                    let m124 = {
                        let a122 = {
                            let (_, h120) = {
                                let (_, h121) = v0;
                                h121
                            };
                            h120
                        };
                        let a123 = v1;
                        compare(a122, a123)
                    };
                    if match m124 {
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
                        let (_, h125) = {
                            let (_, h126) = v0;
                            h126
                        };
                        h125
                    }
                })))
            }
        }
    }
}

pub fn f33(v0: Fn4, v1: (i64, List<i64>), v2: List<i64>) -> R<(i64, List<i64>)> {
    let m127 = v2;
    match m127.uncons() {
        None => {
            Ok(v1)
        }
        Some((v3, v4)) => {
            f33(v0.clone(), {
                let c128: Fn4 = v0;
                c128.apply(v1, v3)?
            }, v4)
        }
    }
}

pub fn f34(v0: (i64, List<i64>), v1: i64) -> R<(i64, List<i64>)> {
    let m130 = {
        let (_, h129) = v0.clone();
        h129
    };
    match m130.uncons() {
        None => {
            Ok(v0)
        }
        Some((v2, v3)) => {
            Ok(({
                let a135 = {
                    let (h131, _) = v0;
                    h131
                };
                let a136 = {
                    let a134 = {
                        let a132 = v1;
                        let a133 = v2;
                        int_mul(a132, a133)?
                    };
                    int_neg(a134)?
                };
                int_sub(a135, a136)?
            }, v3))
        }
    }
}

pub fn f35(v0: bool) -> i64 {
    if v0 {
        1i64
    } else {
        0i64
    }
}

pub fn f36(v0: Bytes, v1: Bytes) -> R<bool> {
    let m137 = f37(v0, v1)?;
    match m137 {
        None => {
            Ok(false)
        }
        Some(_) => {
            Ok(true)
        }
    }
}

pub fn f37(v0: Bytes, v1: Bytes) -> R<Option<u64>> {
    let v2: u64 = {
        let a138 = v0.clone();
        length_bytes(a138)
    };
    let v3: u64 = {
        let a139 = v1.clone();
        length_bytes(a139)
    };
    let v4: u64 = {
        let (h140, _) = f12(Fn0::F38(v1.clone(), v3, v0.clone(), v2), {
            let a141 = v2;
            let a142 = 1u64;
            nat_add(a141, a142)?
        }, 0u64)?;
        h140
    };
    if f39(v0, v1, v4) {
        Ok(Some(v4))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f38(v0: Bytes, v1: u64, v2: Bytes, v3: u64, v4: u64) -> R<Option<u64>> {
    if f39(v2, v0, v4) {
        Ok(None::<u64>)
    } else {
        let a145 = {
            let a143 = v4;
            let a144 = v1;
            nat_add(a143, a144)?
        };
        let a146 = v3;
        if nat_lt(a145, a146) {
            Ok(Some({
                let a147 = v4;
                let a148 = 1u64;
                nat_add(a147, a148)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f39(v0: Bytes, v1: Bytes, v2: u64) -> bool {
    let m153 = {
        let a150 = v0;
        let a151 = v2;
        let a152 = {
            let a149 = v1.clone();
            length_bytes(a149)
        };
        slice_bytes(a150, a151, a152)
    };
    match m153 {
        None => {
            false
        }
        Some(v3) => {
            let a154 = v3;
            let a155 = v1;
            equal(a154, a155)
        }
    }
}

pub fn run(p0: Bytes) -> R<u64> {
    f0(p0)
}
