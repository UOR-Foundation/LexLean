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
    C0(Adt1, u64, u64, u64, u64, u64, u64, Bytes),
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
}

#[derive(Clone)]
pub enum Adt2 {
    C0(Bytes, Bytes, Option<Adt3>, Option<Bytes>),
}

#[derive(Clone)]
pub enum Adt3 {
    C0(Bytes, u64, u64, u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt4 {
    C0(Adt5, Option<Adt0>, Option<Adt2>),
}

#[derive(Clone)]
pub enum Adt5 {
    C0(bool, u64, bool, bool, u64, u64),
}

#[derive(Clone)]
pub enum Adt6 {
    C0(u64, u64, bool, bool),
}

#[derive(Clone)]
pub enum Fn0 {
    F2(List<Bytes>, Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: Adt4, p1: Adt0) -> R<Adt4> {
        match self {
            Fn0::F2(k0, k1) => f2(k0.clone(), k1.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F14(Bytes),
}

impl Fn1 {
    pub fn apply(&self, p0: bool, p1: Bytes) -> bool {
        match self {
            Fn1::F14(k0) => f14(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F22(Bytes, bool),
}

impl Fn2 {
    pub fn apply(&self, p0: Adt6) -> R<Option<Adt6>> {
        match self {
            Fn2::F22(k0, k1) => f22(k0.clone(), *k1, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F26(Bytes),
}

impl Fn3 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn3::F26(k0) => f26(k0.clone(), p0),
        }
    }
}

pub fn f0(v0: Bytes, v1: List<Adt0>, v2: List<Bytes>) -> R<Option<Adt2>> {
    let v3: Adt4 = f1(Fn0::F2(v2, v0), Adt4::C0(Adt5::C0(false, 0u64, false, false, 0u64, 0u64), None::<Adt0>, None::<Adt2>), v1)?;
    match v3 {
        Adt4::C0(_, _, h1) => {
            Ok(h1)
        }
    }
}

pub fn f1(v0: Fn0, p5: Adt4, p6: List<Adt0>) -> R<Adt4> {
    let m4 = (p5, p6);
    {
        let mut m4 = m4;
        loop {
            let step = {
                let (v1, v2) = m4;
                let m2 = v2;
                match m2.uncons() {
                    None => {
                        Err::<(Adt4, List<Adt0>), R<Adt4>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(Adt4, List<Adt0>), R<Adt4>>(({
                            let c3: Fn0 = v0.clone();
                            c3.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m4 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f2(v0: List<Bytes>, v1: Bytes, v2: Adt4, v3: Adt0) -> R<Adt4> {
    f3(v1, v0, v2, v3)
}

pub fn f3(v0: Bytes, v1: List<Bytes>, v2: Adt4, v3: Adt0) -> R<Adt4> {
    let m8 = match v2.clone() {
        Adt4::C0(_, _, h7) => {
            h7
        }
    };
    match m8 {
        None => {
            let v5: Adt5 = match v2.clone() {
                Adt4::C0(h9, _, _) => {
                    h9
                }
            };
            let v6: Option<Adt2> = f4(v0, v3.clone(), match v2 {
                Adt4::C0(_, h10, _) => {
                    h10
                }
            }, v5.clone(), v1)?;
            let v7: Adt5 = f5(v5, v3.clone())?;
            Ok(Adt4::C0(v7, Some(v3), v6))
        }
        Some(_) => {
            Ok(v2)
        }
    }
}

pub fn f4(v0: Bytes, v1: Adt0, v2: Option<Adt0>, v3: Adt5, v4: List<Bytes>) -> R<Option<Adt2>> {
    let v5: Bytes = match v1.clone() {
        Adt0::C0(_, _, _, _, _, _, _, h11) => {
            h11
        }
    };
    let v6: u64 = f6(v1.clone());
    let v7: bool = f7(v5.clone(), v4);
    let v8: bool = f8(v5.clone(), 0u64, 48u8);
    let v9: bool = {
        let a13 = 1u64;
        let a14 = {
            let a12 = v5;
            length_bytes(a12)
        };
        nat_lt(a13, a14)
    };
    let v10: bool = {
        let a15 = v6;
        let a16 = 2u64;
        if nat_eq(a15, a16) {
            if v9 {
                v8
            } else {
                false
            }
        } else {
            false
        }
    };
    let v16: bool = {
        let m17 = v2;
        match m17 {
            None => {
                false
            }
            Some(v11) => {
                let v12: u64 = f6(v11.clone());
                let v13: bool = {
                    let a20 = match v11.clone() {
                        Adt0::C0(_, _, h18, _, _, _, _, _) => {
                            h18
                        }
                    };
                    let a21 = match v1.clone() {
                        Adt0::C0(_, h19, _, _, _, _, _, _) => {
                            h19
                        }
                    };
                    nat_eq(a20, a21)
                };
                let v15: bool = {
                    let v14: Bytes = match v11 {
                        Adt0::C0(_, _, _, _, _, _, _, h22) => {
                            h22
                        }
                    };
                    let a23 = v14.clone();
                    let a24 = Bytes::lit(&[0x5f]);
                    if equal(a23, a24) {
                        true
                    } else {
                        let a25 = v14.clone();
                        let a26 = Bytes::lit(&[0x27]);
                        if equal(a25, a26) {
                            true
                        } else {
                            let a27 = v14;
                            let a28 = Bytes::lit(&[0x2d]);
                            equal(a27, a28)
                        }
                    }
                };
                if v13 {
                    let a29 = v12;
                    let a30 = 1u64;
                    if nat_eq(a29, a30) {
                        true
                    } else {
                        let a31 = v12;
                        let a32 = 4u64;
                        if nat_eq(a31, a32) {
                            v15
                        } else {
                            false
                        }
                    }
                } else {
                    false
                }
            }
        }
    };
    let a33 = v6;
    let a34 = 0u64;
    if if nat_eq(a33, a34) {
        v7
    } else {
        false
    } {
        let v17: Adt2 = f9(v0, v1);
        Ok(Some(v17))
    } else {
        if v10 {
            if v16 {
                Ok(None::<Adt2>)
            } else {
                if match v3 {
                    Adt5::C0(_, _, h35, _, _, _) => {
                        h35
                    }
                } {
                    Ok(None::<Adt2>)
                } else {
                    let v18: Adt2 = f10(v0, v1)?;
                    Ok(Some(v18))
                }
            }
        } else {
            Ok(None::<Adt2>)
        }
    }
}

pub fn f5(v0: Adt5, v1: Adt0) -> R<Adt5> {
    let v2: u64 = match v0.clone() {
        Adt5::C0(_, _, _, _, _, h36) => {
            h36
        }
    };
    let v3: u64 = match v0.clone() {
        Adt5::C0(_, h37, _, _, _, _) => {
            h37
        }
    };
    let v4: u64 = match v0.clone() {
        Adt5::C0(_, _, _, _, h38, _) => {
            h38
        }
    };
    let v5: Bytes = match v1.clone() {
        Adt0::C0(_, _, _, _, _, _, _, h39) => {
            h39
        }
    };
    let v6: u64 = f6(v1);
    let v7: bool = {
        let a40 = v6;
        let a41 = 0u64;
        nat_eq(a40, a41)
    };
    let v8: bool = {
        let a42 = v6;
        let a43 = 6u64;
        nat_eq(a42, a43)
    };
    let v9: bool = f11(v5.clone());
    let v10: bool = if v7 {
        v9
    } else {
        false
    };
    let v11: bool = {
        let a44 = v5.clone();
        let a45 = Bytes::lit(&[0x5c, 0x65, 0x6e, 0x64]);
        equal(a44, a45)
    };
    let v12: bool = if v7 {
        v11
    } else {
        false
    };
    let v13: Adt6 = f12(v5.clone(), {
        let a46 = v2;
        let a47 = 0u64;
        nat_eq(a46, a47)
    }, v0.clone())?;
    let v14: Adt5 = if if match v0.clone() {
        Adt5::C0(_, _, h48, _, _, _) => {
            h48
        }
    } {
        false
    } else {
        v12
    } {
        Adt5::C0(match v0.clone() {
            Adt5::C0(h49, _, _, _, _, _) => {
                h49
            }
        }, match v0.clone() {
            Adt5::C0(_, h50, _, _, _, _) => {
                h50
            }
        }, match v0.clone() {
            Adt5::C0(_, _, h51, _, _, _) => {
                h51
            }
        }, false, match v0.clone() {
            Adt5::C0(_, _, _, _, h52, _) => {
                h52
            }
        }, 0u64)
    } else {
        Adt5::C0(match v0.clone() {
            Adt5::C0(h53, _, _, _, _, _) => {
                h53
            }
        }, match v0.clone() {
            Adt5::C0(_, h54, _, _, _, _) => {
                h54
            }
        }, match v13.clone() {
            Adt6::C0(_, _, h55, _) => {
                h55
            }
        }, match v13.clone() {
            Adt6::C0(_, _, _, h56) => {
                h56
            }
        }, match v0.clone() {
            Adt5::C0(_, _, _, _, h57, _) => {
                h57
            }
        }, 2u64)
    };
    let v15: Adt5 = Adt5::C0(match v0.clone() {
        Adt5::C0(h58, _, _, _, _, _) => {
            h58
        }
    }, match v13.clone() {
        Adt6::C0(_, h59, _, _) => {
            h59
        }
    }, match v13.clone() {
        Adt6::C0(_, _, h60, _) => {
            h60
        }
    }, match v13 {
        Adt6::C0(_, _, _, h61) => {
            h61
        }
    }, match v0.clone() {
        Adt5::C0(_, _, _, _, h62, _) => {
            h62
        }
    }, match v0.clone() {
        Adt5::C0(_, _, _, _, _, h63) => {
            h63
        }
    });
    let v16: bool = if v7 {
        let a64 = v5.clone();
        let a65 = Bytes::lit(&[0x5c, 0x62, 0x65, 0x67, 0x69, 0x6e]);
        equal(a64, a65)
    } else {
        false
    };
    let v17: bool = {
        let a66 = v5.clone();
        let a67 = Bytes::lit(&[0x7b]);
        equal(a66, a67)
    };
    let v18: bool = {
        let a68 = v5.clone();
        let a69 = Bytes::lit(&[0x7d]);
        equal(a68, a69)
    };
    let v19: bool = {
        let a70 = v6;
        let a71 = 1u64;
        if nat_eq(a70, a71) {
            let a72 = v5.clone();
            let a73 = Bytes::lit(&[0x73, 0x65, 0x6d, 0x61, 0x6e, 0x74, 0x69, 0x63, 0x6d, 0x6f, 0x64, 0x75, 0x6c, 0x65]);
            if equal(a72, a73) {
                true
            } else {
                let a74 = v5.clone();
                let a75 = Bytes::lit(&[0x74, 0x6f, 0x6f, 0x6c, 0x69, 0x6e, 0x67, 0x6d, 0x6f, 0x64, 0x75, 0x6c, 0x65]);
                if equal(a74, a75) {
                    true
                } else {
                    let a76 = v5;
                    let a77 = Bytes::lit(&[0x63, 0x6f, 0x72, 0x65, 0x6d, 0x6f, 0x64, 0x75, 0x6c, 0x65]);
                    equal(a76, a77)
                }
            }
        } else {
            false
        }
    };
    let v20: u64 = {
        let a78 = v4;
        let a79 = 0u64;
        if nat_eq(a78, a79) {
            if v16 {
                1u64
            } else {
                0u64
            }
        } else {
            let a80 = v4;
            let a81 = 1u64;
            if nat_eq(a80, a81) {
                if v17 {
                    2u64
                } else {
                    0u64
                }
            } else {
                let a82 = v4;
                let a83 = 2u64;
                if nat_eq(a82, a83) {
                    if v19 {
                        3u64
                    } else {
                        0u64
                    }
                } else {
                    let a84 = v4;
                    let a85 = 3u64;
                    if nat_eq(a84, a85) {
                        if v18 {
                            4u64
                        } else {
                            0u64
                        }
                    } else {
                        0u64
                    }
                }
            }
        }
    };
    let v21: bool = {
        let a86 = v20;
        let a87 = 4u64;
        nat_eq(a86, a87)
    };
    let v22: u64 = if v21 {
        0u64
    } else {
        v20
    };
    let v23: Adt5 = if v8 {
        v0.clone()
    } else {
        if v21 {
            Adt5::C0(match v0.clone() {
                Adt5::C0(h88, _, _, _, _, _) => {
                    h88
                }
            }, match v0.clone() {
                Adt5::C0(_, h89, _, _, _, _) => {
                    h89
                }
            }, match v0.clone() {
                Adt5::C0(_, _, h90, _, _, _) => {
                    h90
                }
            }, match v0.clone() {
                Adt5::C0(_, _, _, h91, _, _) => {
                    h91
                }
            }, 0u64, 1u64)
        } else {
            if if match v0.clone() {
                Adt5::C0(h92, _, _, _, _, _) => {
                    h92
                }
            } {
                v17
            } else {
                false
            } {
                Adt5::C0(false, 1u64, match v0.clone() {
                    Adt5::C0(_, _, h93, _, _, _) => {
                        h93
                    }
                }, match v0.clone() {
                    Adt5::C0(_, _, _, h94, _, _) => {
                        h94
                    }
                }, v22, match v0.clone() {
                    Adt5::C0(_, _, _, _, _, h95) => {
                        h95
                    }
                })
            } else {
                Adt5::C0(v10, match v0.clone() {
                    Adt5::C0(_, h96, _, _, _, _) => {
                        h96
                    }
                }, match v0.clone() {
                    Adt5::C0(_, _, h97, _, _, _) => {
                        h97
                    }
                }, match v0.clone() {
                    Adt5::C0(_, _, _, h98, _, _) => {
                        h98
                    }
                }, v22, match v0.clone() {
                    Adt5::C0(_, _, _, _, _, h99) => {
                        h99
                    }
                })
            }
        }
    };
    let a100 = v2;
    let a101 = 1u64;
    if nat_eq(a100, a101) {
        if v8 {
            Ok(v0)
        } else {
            if v10 {
                Ok(Adt5::C0(true, match v0.clone() {
                    Adt5::C0(_, h102, _, _, _, _) => {
                        h102
                    }
                }, match v0.clone() {
                    Adt5::C0(_, _, h103, _, _, _) => {
                        h103
                    }
                }, match v0.clone() {
                    Adt5::C0(_, _, _, h104, _, _) => {
                        h104
                    }
                }, match v0 {
                    Adt5::C0(_, _, _, _, h105, _) => {
                        h105
                    }
                }, 0u64))
            } else {
                Ok(v14)
            }
        }
    } else {
        let a106 = v2;
        let a107 = 2u64;
        if nat_eq(a106, a107) {
            Ok(v14)
        } else {
            let a108 = v3;
            let a109 = 0u64;
            if nat_eq(a108, a109) {
                Ok(v23)
            } else {
                Ok(v15)
            }
        }
    }
}

pub fn f6(v0: Adt0) -> u64 {
    let m111 = match v0 {
        Adt0::C0(h110, _, _, _, _, _, _, _) => {
            h110
        }
    };
    match m111 {
        Adt1::C0 => {
            0u64
        }
        Adt1::C1 => {
            1u64
        }
        Adt1::C2 => {
            2u64
        }
        Adt1::C3 => {
            3u64
        }
        Adt1::C4 => {
            4u64
        }
        Adt1::C5 => {
            5u64
        }
        Adt1::C6 => {
            6u64
        }
    }
}

pub fn f7(v0: Bytes, v1: List<Bytes>) -> bool {
    f13(Fn1::F14(v0), false, v1)
}

pub fn f8(v0: Bytes, v1: u64, v2: u8) -> bool {
    let m112 = f15(v0, v1);
    match m112 {
        None => {
            false
        }
        Some(v3) => {
            let a113 = v3;
            let a114 = v2;
            equal(a113, a114)
        }
    }
}

pub fn f9(v0: Bytes, v1: Adt0) -> Adt2 {
    let v2: Adt3 = f16(v0, v1.clone());
    Adt2::C0(f17(Str::lit("LLL1002")), f18(f18(f18(f19(match v1 {
        Adt0::C0(_, _, _, _, _, _, _, h115) => {
            h115
        }
    }), f17(Str::lit(" is an always-forbidden TeX control ("))), Bytes::lit(&[0xc2, 0xa7])), f17(Str::lit("12.4); LexLean does not expand TeX"))), Some(v2), None::<Bytes>)
}

pub fn f10(v0: Bytes, v1: Adt0) -> R<Adt2> {
    let v2: Adt3 = f16(v0, v1.clone());
    let v3: Bytes = f20(match v1.clone() {
        Adt0::C0(_, _, _, _, _, _, _, h116) => {
            h116
        }
    })?;
    let v4: Bytes = f19(match v1 {
        Adt0::C0(_, _, _, _, _, _, _, h117) => {
            h117
        }
    });
    Ok(Adt2::C0(f17(Str::lit("LLL1003")), f18(f18(f18(f18(f17(Str::lit("numeral ")), v4.clone()), f17(Str::lit(" has a redundant leading zero; the canonical decimal is "))), f19(v3.clone())), f17(Str::lit(""))), Some(v2), Some(f18(f18(f18(f17(Str::lit("replace ")), v4), f17(Str::lit(" with "))), f19(v3)))))
}

pub fn f11(v0: Bytes) -> bool {
    let a118 = v0.clone();
    let a119 = Bytes::lit(&[0x5c, 0x73, 0x65, 0x6d, 0x61, 0x6e, 0x74, 0x69, 0x63, 0x64, 0x61, 0x74, 0x61]);
    if equal(a118, a119) {
        true
    } else {
        let a120 = v0.clone();
        let a121 = Bytes::lit(&[0x5c, 0x74, 0x6f, 0x6f, 0x6c, 0x69, 0x6e, 0x67, 0x64, 0x61, 0x74, 0x61]);
        if equal(a120, a121) {
            true
        } else {
            let a122 = v0;
            let a123 = Bytes::lit(&[0x5c, 0x63, 0x6f, 0x72, 0x65, 0x64, 0x61, 0x74, 0x61]);
            equal(a122, a123)
        }
    }
}

pub fn f12(v0: Bytes, v1: bool, v2: Adt5) -> R<Adt6> {
    let (h124, _) = f21(Fn2::F22(v0.clone(), v1), {
        let a125 = v0;
        length_bytes(a125)
    }, Adt6::C0(0u64, match v2.clone() {
        Adt5::C0(_, h126, _, _, _, _) => {
            h126
        }
    }, match v2.clone() {
        Adt5::C0(_, _, h127, _, _, _) => {
            h127
        }
    }, match v2 {
        Adt5::C0(_, _, _, h128, _, _) => {
            h128
        }
    }))?;
    Ok(h124)
}

pub fn f13(v0: Fn1, p132: bool, p133: List<Bytes>) -> bool {
    let m131 = (p132, p133);
    {
        let mut m131 = m131;
        loop {
            let step = {
                let (v1, v2) = m131;
                let m129 = v2;
                match m129.uncons() {
                    None => {
                        Err::<(bool, List<Bytes>), bool>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(bool, List<Bytes>), bool>(({
                            let c130: Fn1 = v0.clone();
                            c130.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m131 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f14(v0: Bytes, v1: bool, v2: Bytes) -> bool {
    if v1 {
        true
    } else {
        let a134 = v2;
        let a135 = v0;
        equal(a134, a135)
    }
}

pub fn f15(v0: Bytes, v1: u64) -> Option<u8> {
    let a136 = v0;
    let a137 = v1;
    index_bytes(a136, a137)
}

pub fn f16(v0: Bytes, v1: Adt0) -> Adt3 {
    Adt3::C0(v0, match v1.clone() {
        Adt0::C0(_, h138, _, _, _, _, _, _) => {
            h138
        }
    }, match v1.clone() {
        Adt0::C0(_, _, h139, _, _, _, _, _) => {
            h139
        }
    }, match v1.clone() {
        Adt0::C0(_, _, _, h140, _, _, _, _) => {
            h140
        }
    }, match v1.clone() {
        Adt0::C0(_, _, _, _, h141, _, _, _) => {
            h141
        }
    }, match v1.clone() {
        Adt0::C0(_, _, _, _, _, h142, _, _) => {
            h142
        }
    }, match v1 {
        Adt0::C0(_, _, _, _, _, _, h143, _) => {
            h143
        }
    })
}

pub fn f17(v0: Str) -> Bytes {
    let a144 = v0;
    utf8_encode(a144)
}

pub fn f18(v0: Bytes, v1: Bytes) -> Bytes {
    let a145 = v0;
    let a146 = v1;
    append_bytes(a145, a146)
}

pub fn f19(v0: Bytes) -> Bytes {
    f18(f18(f17(Str::lit("`")), v0), f17(Str::lit("`")))
}

pub fn f20(v0: Bytes) -> R<Bytes> {
    let v1: Bytes = f23(v0)?;
    let a148 = {
        let a147 = v1.clone();
        length_bytes(a147)
    };
    let a149 = 0u64;
    if nat_eq(a148, a149) {
        Ok(f17(Str::lit("0")))
    } else {
        Ok(v1)
    }
}

pub fn f21(v0: Fn2, p154: u64, p155: Adt6) -> R<(Adt6, bool)> {
    let m153 = (p154, p155);
    {
        let mut m153 = m153;
        loop {
            let step = {
                let (v1, v2) = m153;
                let m150 = v1;
                if m150 == 0 {
                    Err::<(u64, Adt6), R<(Adt6, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m150 - 1;
                    let m152 = {
                        let c151: Fn2 = v0.clone();
                        c151.apply(v2.clone())?
                    };
                    match m152 {
                        None => {
                            Err::<(u64, Adt6), R<(Adt6, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt6), R<(Adt6, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m153 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f22(v0: Bytes, v1: bool, v2: Adt6) -> R<Option<Adt6>> {
    f24(v0, v1, v2)
}

pub fn f23(v0: Bytes) -> R<Bytes> {
    let v1: u64 = {
        let (h156, _) = f25(Fn3::F26(v0.clone()), {
            let a157 = v0.clone();
            length_bytes(a157)
        }, 0u64)?;
        h156
    };
    Ok(f27(v0.clone(), v1, {
        let a159 = {
            let a158 = v0;
            length_bytes(a158)
        };
        let a160 = v1;
        nat_sub(a159, a160)
    }))
}

pub fn f24(v0: Bytes, v1: bool, v2: Adt6) -> R<Option<Adt6>> {
    let v3: u64 = match v2.clone() {
        Adt6::C0(h161, _, _, _) => {
            h161
        }
    };
    let a163 = v3;
    let a164 = {
        let a162 = v0.clone();
        length_bytes(a162)
    };
    if nat_lt(a163, a164) {
        let v4: u8 = f28(v0, v3);
        let v5: bool = {
            let a165 = v4;
            let a166 = 34u8;
            equal(a165, a166)
        };
        let v6: bool = {
            let a167 = v4;
            let a168 = 92u8;
            equal(a167, a168)
        };
        let v7: bool = if v1 {
            let a169 = v4;
            let a170 = 123u8;
            equal(a169, a170)
        } else {
            false
        };
        let v8: bool = if v1 {
            let a171 = v4;
            let a172 = 125u8;
            equal(a171, a172)
        } else {
            false
        };
        let v9: u64 = match v2.clone() {
            Adt6::C0(_, h173, _, _) => {
                h173
            }
        };
        let v10: bool = match v2.clone() {
            Adt6::C0(_, _, h174, _) => {
                h174
            }
        };
        let v11: bool = match v2 {
            Adt6::C0(_, _, _, h175) => {
                h175
            }
        };
        if v10 {
            if v11 {
                Ok(Some(Adt6::C0({
                    let a176 = v3;
                    let a177 = 1u64;
                    nat_add(a176, a177)?
                }, v9, true, false)))
            } else {
                if v6 {
                    Ok(Some(Adt6::C0({
                        let a178 = v3;
                        let a179 = 1u64;
                        nat_add(a178, a179)?
                    }, v9, true, true)))
                } else {
                    if v5 {
                        Ok(Some(Adt6::C0({
                            let a180 = v3;
                            let a181 = 1u64;
                            nat_add(a180, a181)?
                        }, v9, false, v11)))
                    } else {
                        Ok(Some(Adt6::C0({
                            let a182 = v3;
                            let a183 = 1u64;
                            nat_add(a182, a183)?
                        }, v9, true, v11)))
                    }
                }
            }
        } else {
            if v5 {
                Ok(Some(Adt6::C0({
                    let a184 = v3;
                    let a185 = 1u64;
                    nat_add(a184, a185)?
                }, v9, true, v11)))
            } else {
                if v7 {
                    Ok(Some(Adt6::C0({
                        let a186 = v3;
                        let a187 = 1u64;
                        nat_add(a186, a187)?
                    }, {
                        let a188 = v9;
                        let a189 = 1u64;
                        nat_add(a188, a189)?
                    }, false, v11)))
                } else {
                    if v8 {
                        Ok(Some(Adt6::C0({
                            let a190 = v3;
                            let a191 = 1u64;
                            nat_add(a190, a191)?
                        }, {
                            let a192 = v9;
                            let a193 = 1u64;
                            nat_sub(a192, a193)
                        }, false, v11)))
                    } else {
                        Ok(Some(Adt6::C0({
                            let a194 = v3;
                            let a195 = 1u64;
                            nat_add(a194, a195)?
                        }, v9, false, v11)))
                    }
                }
            }
        }
    } else {
        Ok(None::<Adt6>)
    }
}

pub fn f25(v0: Fn3, p200: u64, p201: u64) -> R<(u64, bool)> {
    let m199 = (p200, p201);
    {
        let mut m199 = m199;
        loop {
            let step = {
                let (v1, v2) = m199;
                let m196 = v1;
                if m196 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m196 - 1;
                    let m198 = {
                        let c197: Fn3 = v0.clone();
                        c197.apply(v2)?
                    };
                    match m198 {
                        None => {
                            Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, u64), R<(u64, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m199 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f26(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f29(v0, v1)
}

pub fn f27(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m205 = {
        let a202 = v0;
        let a203 = v1;
        let a204 = v2;
        slice_bytes(a202, a203, a204)
    };
    match m205 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f28(v0: Bytes, v1: u64) -> u8 {
    let m206 = f15(v0, v1);
    match m206 {
        None => {
            0u8
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f29(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f8(v0, v1, 48u8) {
        Ok(Some({
            let a207 = v1;
            let a208 = 1u64;
            nat_add(a207, a208)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn run(p0: Bytes, p1: List<Adt0>, p2: List<Bytes>) -> R<Option<Adt2>> {
    f0(p0, p1, p2)
}
