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
    C0(Adt1, u64, bool, u64),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(List<Adt2>),
    C1,
    C2,
    C3(Bytes),
    C4(Bytes),
}

#[derive(Clone)]
pub enum Adt2 {
    C0(Bytes, bool),
}

#[derive(Clone)]
pub enum Adt3 {
    C0(u64, u64, bool, u64, List<Adt0>, Option<Bytes>),
}

#[derive(Clone)]
pub enum Adt4 {
    C0(u64, u64, u64, bool),
}

#[derive(Clone)]
pub enum Adt5 {
    C0(u64, u64, bool, bool),
}

#[derive(Clone)]
pub enum Adt6 {
    C0(u64, List<Adt2>, bool, Option<Bytes>),
}

#[derive(Clone)]
pub enum Adt7 {
    C0(u64, bool, bool, bool),
}

#[derive(Clone)]
pub enum Fn0 {
    F2(u64, Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: Adt3) -> R<Option<Adt3>> {
        match self {
            Fn0::F2(k0, k1) => f2(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F6,
}

impl Fn1 {
    pub fn apply(&self, p0: List<Adt0>, p1: Adt0) -> List<Adt0> {
        match self {
            Fn1::F6 => f6(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F19(u64, Bytes),
    F48(Bytes),
    F60(Bytes),
    F66(Bytes),
}

impl Fn2 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn2::F19(k0, k1) => f19(*k0, k1.clone(), p0),
            Fn2::F48(k0) => f48(k0.clone(), p0),
            Fn2::F60(k0) => f60(k0.clone(), p0),
            Fn2::F66(k0) => f66(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F21(u64, Bytes),
}

impl Fn3 {
    pub fn apply(&self, p0: Adt4) -> R<Option<Adt4>> {
        match self {
            Fn3::F21(k0, k1) => f21(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F37(Bytes, u64),
}

impl Fn4 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn4::F37(k0, k1) => f37(k0.clone(), *k1, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F41(u64, Bytes),
    F67(u64, u64, Bytes),
}

impl Fn5 {
    pub fn apply(&self, p0: Adt5) -> R<Option<Adt5>> {
        match self {
            Fn5::F41(k0, k1) => f41(*k0, k1.clone(), p0),
            Fn5::F67(k0, k1, k2) => f67(*k0, *k1, k2.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F57(u64, Bytes),
}

impl Fn6 {
    pub fn apply(&self, p0: (u64, (u64, bool))) -> R<Option<(u64, (u64, bool))>> {
        match self {
            Fn6::F57(k0, k1) => f57(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F59(u64, u64, Bytes),
}

impl Fn7 {
    pub fn apply(&self, p0: (u64, (bool, bool))) -> R<Option<(u64, (bool, bool))>> {
        match self {
            Fn7::F59(k0, k1, k2) => f59(*k0, *k1, k2.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F64(Bytes),
}

impl Fn8 {
    pub fn apply(&self, p0: Adt6) -> R<Option<Adt6>> {
        match self {
            Fn8::F64(k0) => f64(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F75,
}

impl Fn9 {
    pub fn apply(&self, p0: List<Adt2>, p1: Adt2) -> List<Adt2> {
        match self {
            Fn9::F75 => f75(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F83(Bytes),
}

impl Fn10 {
    pub fn apply(&self, p0: Adt7) -> R<Option<Adt7>> {
        match self {
            Fn10::F83(k0) => f83(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F85(u64, u64, Bytes),
}

impl Fn11 {
    pub fn apply(&self, p0: (u64, bool)) -> R<Option<(u64, bool)>> {
        match self {
            Fn11::F85(k0, k1, k2) => f85(*k0, *k1, k2.clone(), p0),
        }
    }
}

pub fn f0(v0: Bytes) -> R<Result<List<Adt0>, Bytes>> {
    let v1: u64 = {
        let a1 = v0.clone();
        length_bytes(a1)
    };
    let v2: Adt3 = {
        let (h2, _) = f1(Fn0::F2(v1, v0), {
            let a3 = v1;
            let a4 = 1u64;
            nat_add(a3, a4)?
        }, Adt3::C0(0u64, 0u64, true, 0u64, List::<Adt0>::nil(), None::<Bytes>))?;
        h2
    };
    let m6 = match v2.clone() {
        Adt3::C0(_, _, _, _, _, h5) => {
            h5
        }
    };
    match m6 {
        None => {
            let v4: List<Adt0> = f3(match v2 {
                Adt3::C0(_, _, _, _, h7, _) => {
                    h7
                }
            });
            Ok(Ok::<List<Adt0>, Bytes>(v4))
        }
        Some(v3) => {
            Ok(Err::<List<Adt0>, Bytes>(v3))
        }
    }
}

pub fn f1(v0: Fn0, v1: u64, v2: Adt3) -> R<(Adt3, bool)> {
    let m8 = v1;
    if m8 == 0 {
        Ok((v2, false))
    } else {
        let v3 = m8 - 1;
        let m10 = {
            let c9: Fn0 = v0.clone();
            c9.apply(v2.clone())?
        };
        match m10 {
            None => {
                Ok((v2, true))
            }
            Some(v4) => {
                f1(v0, v3, v4)
            }
        }
    }
}

pub fn f2(v0: u64, v1: Bytes, v2: Adt3) -> R<Option<Adt3>> {
    f4(v1, v0, v2)
}

pub fn f3(v0: List<Adt0>) -> List<Adt0> {
    f5(Fn1::F6, List::<Adt0>::nil(), v0)
}

pub fn f4(v0: Bytes, v1: u64, v2: Adt3) -> R<Option<Adt3>> {
    let m12 = match v2.clone() {
        Adt3::C0(_, _, _, _, _, h11) => {
            h11
        }
    };
    match m12 {
        None => {
            let v4: Option<Adt3> = f7(v0, match v2.clone() {
                Adt3::C0(h13, _, _, _, _, _) => {
                    h13
                }
            }, match v2.clone() {
                Adt3::C0(_, h14, _, _, _, _) => {
                    h14
                }
            }, match v2.clone() {
                Adt3::C0(_, _, h15, _, _, _) => {
                    h15
                }
            }, match v2.clone() {
                Adt3::C0(_, _, _, h16, _, _) => {
                    h16
                }
            }, match v2.clone() {
                Adt3::C0(_, _, _, _, h17, _) => {
                    h17
                }
            })?;
            let a19 = match v2 {
                Adt3::C0(h18, _, _, _, _, _) => {
                    h18
                }
            };
            let a20 = v1;
            if nat_lt(a19, a20) {
                Ok(v4)
            } else {
                Ok(None::<Adt3>)
            }
        }
        Some(_) => {
            Ok(None::<Adt3>)
        }
    }
}

pub fn f5(v0: Fn1, v1: List<Adt0>, v2: List<Adt0>) -> List<Adt0> {
    let m21 = v2;
    match m21.uncons() {
        None => {
            v1
        }
        Some((v3, v4)) => {
            f5(v0.clone(), {
                let c22: Fn1 = v0;
                c22.apply(v1, v3)
            }, v4)
        }
    }
}

pub fn f6(v0: List<Adt0>, v1: Adt0) -> List<Adt0> {
    List::cons(v1, v0)
}

pub fn f7(v0: Bytes, v1: u64, v2: u64, v3: bool, v4: u64, v5: List<Adt0>) -> R<Option<Adt3>> {
    let v6: u8 = f8(v0.clone(), v1);
    let v7: bool = f9(v0.clone(), {
        let a23 = v1;
        let a24 = 1u64;
        nat_add(a23, a24)?
    }, 45u8);
    let v8: u64 = f10(v0.clone(), v1)?;
    let v9: Adt4 = f11(v0.clone(), {
        let a25 = v1;
        let a26 = 2u64;
        nat_add(a25, a26)?
    }, v2)?;
    let v10: Adt3 = f12(v0.clone(), v6, v1, v2, v3, v4, v5.clone())?;
    let a27 = v6;
    let a28 = 10u8;
    if equal(a27, a28) {
        Ok(Some(Adt3::C0({
            let a29 = v1;
            let a30 = 1u64;
            nat_add(a29, a30)?
        }, {
            let a31 = v2;
            let a32 = 1u64;
            nat_add(a31, a32)?
        }, true, {
            let a33 = v4;
            let a34 = 1u64;
            nat_add(a33, a34)?
        }, v5, None::<Bytes>)))
    } else {
        let a35 = v6;
        let a36 = 32u8;
        if if equal(a35, a36) {
            true
        } else {
            let a37 = v6;
            let a38 = 9u8;
            if equal(a37, a38) {
                true
            } else {
                let a39 = v6;
                let a40 = 13u8;
                equal(a39, a40)
            }
        } {
            Ok(Some(Adt3::C0({
                let a41 = v1;
                let a42 = 1u64;
                nat_add(a41, a42)?
            }, v2, v3, {
                let a43 = v4;
                let a44 = 1u64;
                nat_add(a43, a44)?
            }, v5, None::<Bytes>)))
        } else {
            let m45 = f13(v0.clone(), v1)?;
            match m45 {
                None => {
                    let a46 = v6;
                    let a47 = 45u8;
                    if if equal(a46, a47) {
                        v7
                    } else {
                        false
                    } {
                        Ok(Some(Adt3::C0(v8, v2, v3, {
                            let a48 = v4;
                            let a49 = f16(v0, v1, v8)?;
                            nat_add(a48, a49)?
                        }, v5, None::<Bytes>)))
                    } else {
                        let a50 = v6;
                        let a51 = 47u8;
                        if if equal(a50, a51) {
                            v7
                        } else {
                            false
                        } {
                            if match v9.clone() {
                                Adt4::C0(_, _, _, h52) => {
                                    h52
                                }
                            } {
                                Ok(Some(f15(v1, v2, v3, v4, v5, {
                                    let a53 = Str::lit("a comment is not closed");
                                    utf8_encode(a53)
                                })))
                            } else {
                                Ok(Some(Adt3::C0(match v9.clone() {
                                    Adt4::C0(h54, _, _, _) => {
                                        h54
                                    }
                                }, match v9.clone() {
                                    Adt4::C0(_, h55, _, _) => {
                                        h55
                                    }
                                }, v3, {
                                    let a57 = v4;
                                    let a58 = f16(v0, v1, match v9 {
                                        Adt4::C0(h56, _, _, _) => {
                                            h56
                                        }
                                    })?;
                                    nat_add(a57, a58)?
                                }, v5, None::<Bytes>)))
                            }
                        } else {
                            Ok(Some(v10))
                        }
                    }
                }
                Some(v11) => {
                    let v12: Bytes = f14(v11);
                    Ok(Some(f15(v1, v2, v3, v4, v5, v12)))
                }
            }
        }
    }
}

pub fn f8(v0: Bytes, v1: u64) -> u8 {
    let m59 = f17(v0, v1);
    match m59 {
        None => {
            0u8
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f9(v0: Bytes, v1: u64, v2: u8) -> bool {
    let m60 = f17(v0, v1);
    match m60 {
        None => {
            false
        }
        Some(v3) => {
            let a61 = v3;
            let a62 = v2;
            equal(a61, a62)
        }
    }
}

pub fn f10(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = {
        let a63 = v0.clone();
        length_bytes(a63)
    };
    let (h64, _) = f18(Fn2::F19(v2, v0), {
        let a65 = v2;
        let a66 = v1;
        nat_sub(a65, a66)
    }, v1)?;
    Ok(h64)
}

pub fn f11(v0: Bytes, v1: u64, v2: u64) -> R<Adt4> {
    let v3: u64 = {
        let a67 = v0.clone();
        length_bytes(a67)
    };
    let (h68, _) = f20(Fn3::F21(v3, v0), {
        let a71 = {
            let a69 = v3;
            let a70 = v1;
            nat_sub(a69, a70)
        };
        let a72 = 1u64;
        nat_add(a71, a72)?
    }, Adt4::C0(v1, v2, 1u64, false))?;
    Ok(h68)
}

pub fn f12(v0: Bytes, v1: u8, v2: u64, v3: u64, v4: bool, v5: u64, v6: List<Adt0>) -> R<Adt3> {
    let v7: Adt5 = f22(v0.clone(), {
        let a73 = v2;
        let a74 = 1u64;
        nat_add(a73, a74)?
    }, v3)?;
    let v8: Adt3 = if match v7.clone() {
        Adt5::C0(_, _, _, h75) => {
            h75
        }
    } {
        f15(v2, v3, false, v5, v6.clone(), {
            let a76 = Str::lit("a string is not closed");
            utf8_encode(a76)
        })
    } else {
        f23(match v7.clone() {
            Adt5::C0(h77, _, _, _) => {
                h77
            }
        }, match v7.clone() {
            Adt5::C0(_, h78, _, _) => {
                h78
            }
        }, v5, f16(v0.clone(), v2, match v7 {
            Adt5::C0(h79, _, _, _) => {
                h79
            }
        })?, Adt1::C2, v3, v4, v6.clone())?
    };
    let v10: Adt3 = {
        let m80 = f24(v0.clone(), v2)?;
        match m80 {
            None => {
                f15(v2, v3, false, v5, v6.clone(), {
                    let a81 = Str::lit("a character literal is not closed");
                    utf8_encode(a81)
                })
            }
            Some(v9) => {
                f23(v9, v3, v5, f16(v0.clone(), v2, v9)?, Adt1::C2, v3, v4, v6.clone())?
            }
        }
    };
    let v13: Adt3 = {
        let m82 = f25(v0.clone(), v2)?;
        match m82 {
            Ok(v12) => {
                f23(v12, v3, v5, f16(v0.clone(), v2, v12)?, Adt1::C1, v3, v4, v6.clone())?
            }
            Err(v11) => {
                f15(v2, v3, false, v5, v6.clone(), v11)
            }
        }
    };
    let v14: u64 = f26(v0.clone(), {
        let a83 = v2;
        let a84 = 1u64;
        nat_add(a83, a84)?
    })?;
    let v15: u64 = f27(v0.clone(), {
        let a87 = {
            let a85 = v2;
            let a86 = 1u64;
            nat_add(a85, a86)?
        };
        let a88 = v14;
        nat_add(a87, a88)?
    })?;
    let v16: Adt3 = f23(v15, v3, v5, f16(v0.clone(), v2, v15)?, Adt1::C3(f28(v0.clone(), v2, {
        let a89 = v15;
        let a90 = v2;
        nat_sub(a89, a90)
    })), v3, v4, v6.clone())?;
    let v17: bool = f29(v0.clone(), v2)?;
    let v18: Adt3 = f30(v0.clone(), v2, v3, v4, v5, v6.clone())?;
    let v19: u64 = f31(v0.clone(), v2);
    let v20: Adt3 = f23({
        let a91 = v2;
        let a92 = v19;
        nat_add(a91, a92)?
    }, v3, v5, 1u64, Adt1::C4(f28(v0, v2, v19)), v3, v4, v6)?;
    let v21: bool = f32(v1);
    let a93 = v1;
    let a94 = 34u8;
    if equal(a93, a94) {
        Ok(v8)
    } else {
        let a95 = v1;
        let a96 = 39u8;
        if equal(a95, a96) {
            Ok(v10)
        } else {
            if v21 {
                Ok(v13)
            } else {
                let a97 = v1;
                let a98 = 35u8;
                if if equal(a97, a98) {
                    let a99 = 0u64;
                    let a100 = v14;
                    nat_lt(a99, a100)
                } else {
                    false
                } {
                    Ok(v16)
                } else {
                    if v17 {
                        Ok(v18)
                    } else {
                        Ok(v20)
                    }
                }
            }
        }
    }
}

pub fn f13(v0: Bytes, v1: u64) -> R<Option<u8>> {
    let a102 = v1;
    let a103 = {
        let a101 = v0.clone();
        length_bytes(a101)
    };
    if nat_lt(a102, a103) {
        let v2: u8 = f8(v0.clone(), v1);
        if f33(v2, 0u8, 31u8) {
            Ok(Some(v2))
        } else {
            let a104 = v2;
            let a105 = 127u8;
            if equal(a104, a105) {
                Ok(Some(v2))
            } else {
                let a106 = v2;
                let a107 = 194u8;
                if equal(a106, a107) {
                    if f34(v0.clone(), {
                        let a108 = v1;
                        let a109 = 1u64;
                        nat_add(a108, a109)?
                    }, 128u8, 159u8) {
                        Ok(Some(f8(v0, {
                            let a110 = v1;
                            let a111 = 1u64;
                            nat_add(a110, a111)?
                        })))
                    } else {
                        Ok(None::<u8>)
                    }
                } else {
                    Ok(None::<u8>)
                }
            }
        }
    } else {
        Ok(None::<u8>)
    }
}

pub fn f14(v0: u8) -> Bytes {
    let v2: u8 = {
        let m114 = {
            let a112 = v0;
            let a113 = 4u32;
            fixed_u8::shift_right(a112, a113)
        };
        match m114 {
            None => {
                0u8
            }
            Some(v1) => {
                v1
            }
        }
    };
    let v3: u8 = {
        let a115 = v0;
        let a116 = 15u8;
        fixed_u8::bit_and(a115, a116)
    };
    let v4: Bytes = f35(v2);
    let v5: Bytes = f35(v3);
    let a123 = {
        let a120 = {
            let a117 = Str::lit("a control character U+00");
            utf8_encode(a117)
        };
        let a121 = {
            let a118 = v4;
            let a119 = v5;
            append_bytes(a118, a119)
        };
        append_bytes(a120, a121)
    };
    let a124 = {
        let a122 = Str::lit(" in the text");
        utf8_encode(a122)
    };
    append_bytes(a123, a124)
}

pub fn f15(v0: u64, v1: u64, v2: bool, v3: u64, v4: List<Adt0>, v5: Bytes) -> Adt3 {
    Adt3::C0(v0, v1, v2, v3, v4, Some(v5))
}

pub fn f16(v0: Bytes, v1: u64, v2: u64) -> R<u64> {
    let (_, h125) = {
        let (h126, _) = f36(Fn4::F37(v0, v2), {
            let a127 = v2;
            let a128 = v1;
            nat_sub(a127, a128)
        }, (v1, 0u64))?;
        h126
    };
    Ok(h125)
}

pub fn f17(v0: Bytes, v1: u64) -> Option<u8> {
    let a129 = v0;
    let a130 = v1;
    index_bytes(a129, a130)
}

pub fn f18(v0: Fn2, v1: u64, v2: u64) -> R<(u64, bool)> {
    let m131 = v1;
    if m131 == 0 {
        Ok((v2, false))
    } else {
        let v3 = m131 - 1;
        let m133 = {
            let c132: Fn2 = v0.clone();
            c132.apply(v2)?
        };
        match m133 {
            None => {
                Ok((v2, true))
            }
            Some(v4) => {
                f18(v0, v3, v4)
            }
        }
    }
}

pub fn f19(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f38(v1, v0, v2)
}

pub fn f20(v0: Fn3, v1: u64, v2: Adt4) -> R<(Adt4, bool)> {
    let m134 = v1;
    if m134 == 0 {
        Ok((v2, false))
    } else {
        let v3 = m134 - 1;
        let m136 = {
            let c135: Fn3 = v0.clone();
            c135.apply(v2.clone())?
        };
        match m136 {
            None => {
                Ok((v2, true))
            }
            Some(v4) => {
                f20(v0, v3, v4)
            }
        }
    }
}

pub fn f21(v0: u64, v1: Bytes, v2: Adt4) -> R<Option<Adt4>> {
    f39(v1, v0, v2)
}

pub fn f22(v0: Bytes, v1: u64, v2: u64) -> R<Adt5> {
    let v3: u64 = {
        let a137 = v0.clone();
        length_bytes(a137)
    };
    let (h138, _) = f40(Fn5::F41(v3, v0), {
        let a141 = {
            let a139 = v3;
            let a140 = v1;
            nat_sub(a139, a140)
        };
        let a142 = 1u64;
        nat_add(a141, a142)?
    }, Adt5::C0(v1, v2, false, false))?;
    Ok(h138)
}

pub fn f23(v0: u64, v1: u64, v2: u64, v3: u64, v4: Adt1, v5: u64, v6: bool, v7: List<Adt0>) -> R<Adt3> {
    Ok(Adt3::C0(v0, v1, false, {
        let a143 = v2;
        let a144 = v3;
        nat_add(a143, a144)?
    }, List::cons(Adt0::C0(v4, v5, v6, v2), v7), None::<Bytes>))
}

pub fn f24(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = {
        let a145 = v0.clone();
        length_bytes(a145)
    };
    let v3: u64 = {
        let a146 = v1;
        let a147 = 1u64;
        nat_add(a146, a147)?
    };
    let v4: u8 = f8(v0.clone(), v3);
    let v5: u64 = {
        let a148 = v3;
        let a149 = 1u64;
        nat_add(a148, a149)?
    };
    let v6: u8 = f8(v0.clone(), v5);
    let v7: u64 = {
        let a150 = v5;
        let a151 = f31(v0.clone(), v5);
        nat_add(a150, a151)?
    };
    let v8: Option<u64> = f42(v0.clone(), v7, 2u64)?;
    let v9: Option<u64> = f42(v0.clone(), v7, 4u64)?;
    let v10: u64 = f31(v0.clone(), v3);
    let v11: Option<u64> = {
        let a152 = v3;
        let a153 = v2;
        if nat_lt(a152, a153) {
            let a154 = v4;
            let a155 = 92u8;
            if equal(a154, a155) {
                let a156 = v5;
                let a157 = v2;
                if nat_lt(a156, a157) {
                    let a158 = v6;
                    let a159 = 120u8;
                    if equal(a158, a159) {
                        v8
                    } else {
                        let a160 = v6;
                        let a161 = 117u8;
                        if equal(a160, a161) {
                            v9
                        } else {
                            Some(v7)
                        }
                    }
                } else {
                    None::<u64>
                }
            } else {
                let a162 = v4;
                let a163 = 39u8;
                if equal(a162, a163) {
                    None::<u64>
                } else {
                    let a164 = v4;
                    let a165 = 10u8;
                    if equal(a164, a165) {
                        None::<u64>
                    } else {
                        Some({
                            let a166 = v3;
                            let a167 = v10;
                            nat_add(a166, a167)?
                        })
                    }
                }
            }
        } else {
            None::<u64>
        }
    };
    let m168 = v11;
    match m168 {
        None => {
            Ok(None::<u64>)
        }
        Some(v12) => {
            let v13: bool = f9(v0, v12, 39u8);
            if v13 {
                Ok(Some({
                    let a169 = v12;
                    let a170 = 1u64;
                    nat_add(a169, a170)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f25(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let v2: u8 = f8(v0.clone(), v1);
    let v3: u8 = f8(v0.clone(), {
        let a171 = v1;
        let a172 = 1u64;
        nat_add(a171, a172)?
    });
    let v4: u64 = {
        let a173 = v2;
        let a174 = 48u8;
        if equal(a173, a174) {
            let a175 = v3;
            let a176 = 120u8;
            if equal(a175, a176) {
                16u64
            } else {
                let a177 = v3;
                let a178 = 88u8;
                if equal(a177, a178) {
                    16u64
                } else {
                    let a179 = v3;
                    let a180 = 98u8;
                    if equal(a179, a180) {
                        2u64
                    } else {
                        let a181 = v3;
                        let a182 = 66u8;
                        if equal(a181, a182) {
                            2u64
                        } else {
                            let a183 = v3;
                            let a184 = 111u8;
                            if equal(a183, a184) {
                                8u64
                            } else {
                                let a185 = v3;
                                let a186 = 79u8;
                                if equal(a185, a186) {
                                    8u64
                                } else {
                                    10u64
                                }
                            }
                        }
                    }
                }
            }
        } else {
            10u64
        }
    };
    let v5: Result<u64, Bytes> = f43(v0.clone(), {
        let a187 = v1;
        let a188 = 2u64;
        nat_add(a187, a188)?
    }, v4, true)?;
    let v6: u64 = f44(v0.clone(), v1)?;
    let v9: Result<u64, Bytes> = {
        let m191 = f43(v0.clone(), {
            let a189 = v1;
            let a190 = 1u64;
            nat_add(a189, a190)?
        }, 10u64, false)?;
        match m191 {
            Ok(v8) => {
                f45(v0.clone(), v8)?
            }
            Err(v7) => {
                Err::<u64, Bytes>(v7)
            }
        }
    };
    let a194 = {
        let a192 = v4;
        let a193 = 10u64;
        nat_eq(a192, a193)
    };
    if bool_not(a194) {
        Ok(v5)
    } else {
        let a197 = {
            let a195 = v2;
            let a196 = 48u8;
            equal(a195, a196)
        };
        if if bool_not(a197) {
            let a198 = 0u64;
            let a199 = v1;
            if nat_lt(a198, a199) {
                f9(v0.clone(), {
                    let a200 = v1;
                    let a201 = 1u64;
                    nat_sub(a200, a201)
                }, 46u8)
            } else {
                false
            }
        } else {
            false
        } {
            if if f9(v0.clone(), v6, 95u8) {
                true
            } else {
                if f9(v0.clone(), v6, 101u8) {
                    true
                } else {
                    f9(v0, v6, 69u8)
                }
            } {
                Ok(Err::<u64, Bytes>({
                    let a202 = Str::lit("a field index is followed by what could continue a number");
                    utf8_encode(a202)
                }))
            } else {
                Ok(Ok::<u64, Bytes>(v6))
            }
        } else {
            Ok(v9)
        }
    }
}

pub fn f26(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u8 = f8(v0.clone(), v1);
    let v3: bool = f46(v2);
    let v4: bool = {
        let a203 = v2;
        let a204 = 95u8;
        equal(a203, a204)
    };
    let v5: u64 = f47(v0.clone(), v1)?;
    let a206 = v1;
    let a207 = {
        let a205 = v0;
        length_bytes(a205)
    };
    if nat_lt(a206, a207) {
        if v3 {
            Ok(1u64)
        } else {
            if v4 {
                Ok(1u64)
            } else {
                Ok(v5)
            }
        }
    } else {
        Ok(0u64)
    }
}

pub fn f27(v0: Bytes, v1: u64) -> R<u64> {
    let (h208, _) = f18(Fn2::F48(v0.clone()), {
        let a210 = {
            let a209 = v0;
            length_bytes(a209)
        };
        let a211 = v1;
        nat_sub(a210, a211)
    }, v1)?;
    Ok(h208)
}

pub fn f28(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m215 = {
        let a212 = v0;
        let a213 = v1;
        let a214 = v2;
        slice_bytes(a212, a213, a214)
    };
    match m215 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f29(v0: Bytes, v1: u64) -> R<bool> {
    let v2: u64 = f26(v0.clone(), v1)?;
    let v3: bool = f49(v0, v1)?;
    let a216 = 0u64;
    let a217 = v2;
    if nat_lt(a216, a217) {
        Ok(true)
    } else {
        Ok(v3)
    }
}

pub fn f30(v0: Bytes, v1: u64, v2: u64, v3: bool, v4: u64, v5: List<Adt0>) -> R<Adt3> {
    let m218 = f50(v0.clone(), v1)?;
    match m218 {
        Ok(v7) => {
            let m220 = {
                let (h219, _) = v7.clone();
                h219
            };
            match m220.uncons() {
                None => {
                    Ok(Adt3::C0({
                        let (_, h221) = v7;
                        h221
                    }, v2, false, v4, v5, None::<Bytes>))
                }
                Some((v8, v9)) => {
                    let m222 = v9;
                    match m222.uncons() {
                        None => {
                            let v10: u64 = f51(v0.clone(), {
                                let (_, h223) = v7.clone();
                                h223
                            })?;
                            let v11: Adt5 = f52(v0.clone(), {
                                let a227 = {
                                    let a225 = {
                                        let (_, h224) = v7.clone();
                                        h224
                                    };
                                    let a226 = v10;
                                    nat_add(a225, a226)?
                                };
                                let a228 = 1u64;
                                nat_add(a227, a228)?
                            }, v2, v10)?;
                            let v12: Adt3 = if match v11.clone() {
                                Adt5::C0(_, _, _, h229) => {
                                    h229
                                }
                            } {
                                f15(v1, v2, false, v4, v5.clone(), {
                                    let a230 = Str::lit("a raw string is not closed");
                                    utf8_encode(a230)
                                })
                            } else {
                                f23(match v11.clone() {
                                    Adt5::C0(h231, _, _, _) => {
                                        h231
                                    }
                                }, match v11.clone() {
                                    Adt5::C0(_, h232, _, _) => {
                                        h232
                                    }
                                }, v4, f16(v0.clone(), v1, match v11 {
                                    Adt5::C0(h233, _, _, _) => {
                                        h233
                                    }
                                })?, Adt1::C2, v2, v3, v5.clone())?
                            };
                            let v13: Adt3 = f15(v1, v2, false, v4, v5.clone(), {
                                let a234 = Str::lit("an interpolated string holds code");
                                utf8_encode(a234)
                            });
                            let v14: Adt3 = f23({
                                let (_, h235) = v7.clone();
                                h235
                            }, v2, v4, f16(v0.clone(), v1, {
                                let (_, h236) = v7.clone();
                                h236
                            })?, Adt1::C0({
                                let (h237, _) = v7.clone();
                                h237
                            }), v2, v3, v5)?;
                            let v15: bool = f53(match v8.clone() {
                                Adt2::C0(h238, _) => {
                                    h238
                                }
                            }, {
                                let a239 = Str::lit("!");
                                utf8_encode(a239)
                            });
                            let v16: bool = f9(v0.clone(), {
                                let (_, h240) = v7.clone();
                                h240
                            }, 34u8);
                            let v17: bool = f9(v0, {
                                let a242 = {
                                    let (_, h241) = v7;
                                    h241
                                };
                                let a243 = v10;
                                nat_add(a242, a243)?
                            }, 34u8);
                            let a245 = match v8.clone() {
                                Adt2::C0(_, h244) => {
                                    h244
                                }
                            };
                            if if bool_not(a245) {
                                let a248 = match v8.clone() {
                                    Adt2::C0(h246, _) => {
                                        h246
                                    }
                                };
                                let a249 = {
                                    let a247 = Str::lit("r");
                                    utf8_encode(a247)
                                };
                                if equal(a248, a249) {
                                    v17
                                } else {
                                    false
                                }
                            } else {
                                false
                            } {
                                Ok(v12)
                            } else {
                                let a251 = match v8 {
                                    Adt2::C0(_, h250) => {
                                        h250
                                    }
                                };
                                if if bool_not(a251) {
                                    if v15 {
                                        v16
                                    } else {
                                        false
                                    }
                                } else {
                                    false
                                } {
                                    Ok(v13)
                                } else {
                                    Ok(v14)
                                }
                            }
                        }
                        Some((_, _)) => {
                            f23({
                                let (_, h252) = v7.clone();
                                h252
                            }, v2, v4, f16(v0, v1, {
                                let (_, h253) = v7;
                                h253
                            })?, Adt1::C0({
                                let (h254, _) = v7;
                                h254
                            }), v2, v3, v5)
                        }
                    }
                }
            }
        }
        Err(v6) => {
            Ok(f15(v1, v2, false, v4, v5, v6))
        }
    }
}

pub fn f31(v0: Bytes, v1: u64) -> u64 {
    let v2: u8 = f8(v0, v1);
    let v3: bool = f33(v2, 0u8, 127u8);
    if v3 {
        1u64
    } else {
        let v4: bool = f33(v2, 128u8, 223u8);
        if v4 {
            2u64
        } else {
            let v5: bool = f33(v2, 224u8, 239u8);
            if v5 {
                3u64
            } else {
                4u64
            }
        }
    }
}

pub fn f32(v0: u8) -> bool {
    f33(v0, 48u8, 57u8)
}

pub fn f33(v0: u8, v1: u8, v2: u8) -> bool {
    let m257 = {
        let a255 = v0;
        let a256 = v1;
        fixed_u8::checked_sub(a255, a256)
    };
    match m257 {
        None => {
            false
        }
        Some(_) => {
            let m260 = {
                let a258 = v2;
                let a259 = v0;
                fixed_u8::checked_sub(a258, a259)
            };
            match m260 {
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

pub fn f34(v0: Bytes, v1: u64, v2: u8, v3: u8) -> bool {
    let m261 = f17(v0, v1);
    match m261 {
        None => {
            false
        }
        Some(v4) => {
            f33(v4, v2, v3)
        }
    }
}

pub fn f35(v0: u8) -> Bytes {
    let v1: Bytes = {
        let a262 = Str::lit("0123456789ABCDEF");
        utf8_encode(a262)
    };
    let v2: u64 = {
        let a263 = v0;
        let a264 = 0u8;
        if equal(a263, a264) {
            0u64
        } else {
            let a265 = v0;
            let a266 = 1u8;
            if equal(a265, a266) {
                1u64
            } else {
                let a267 = v0;
                let a268 = 2u8;
                if equal(a267, a268) {
                    2u64
                } else {
                    let a269 = v0;
                    let a270 = 3u8;
                    if equal(a269, a270) {
                        3u64
                    } else {
                        let a271 = v0;
                        let a272 = 4u8;
                        if equal(a271, a272) {
                            4u64
                        } else {
                            let a273 = v0;
                            let a274 = 5u8;
                            if equal(a273, a274) {
                                5u64
                            } else {
                                let a275 = v0;
                                let a276 = 6u8;
                                if equal(a275, a276) {
                                    6u64
                                } else {
                                    let a277 = v0;
                                    let a278 = 7u8;
                                    if equal(a277, a278) {
                                        7u64
                                    } else {
                                        let a279 = v0;
                                        let a280 = 8u8;
                                        if equal(a279, a280) {
                                            8u64
                                        } else {
                                            let a281 = v0;
                                            let a282 = 9u8;
                                            if equal(a281, a282) {
                                                9u64
                                            } else {
                                                let a283 = v0;
                                                let a284 = 10u8;
                                                if equal(a283, a284) {
                                                    10u64
                                                } else {
                                                    let a285 = v0;
                                                    let a286 = 11u8;
                                                    if equal(a285, a286) {
                                                        11u64
                                                    } else {
                                                        let a287 = v0;
                                                        let a288 = 12u8;
                                                        if equal(a287, a288) {
                                                            12u64
                                                        } else {
                                                            let a289 = v0;
                                                            let a290 = 13u8;
                                                            if equal(a289, a290) {
                                                                13u64
                                                            } else {
                                                                let a291 = v0;
                                                                let a292 = 14u8;
                                                                if equal(a291, a292) {
                                                                    14u64
                                                                } else {
                                                                    15u64
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    };
    f28(v1, v2, 1u64)
}

pub fn f36(v0: Fn4, v1: u64, v2: (u64, u64)) -> R<((u64, u64), bool)> {
    let m293 = v1;
    if m293 == 0 {
        Ok((v2, false))
    } else {
        let v3 = m293 - 1;
        let m295 = {
            let c294: Fn4 = v0.clone();
            c294.apply(v2)?
        };
        match m295 {
            None => {
                Ok((v2, true))
            }
            Some(v4) => {
                f36(v0, v3, v4)
            }
        }
    }
}

pub fn f37(v0: Bytes, v1: u64, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    f54(v0, v1, v2)
}

pub fn f38(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let a296 = v2;
    let a297 = v1;
    if nat_lt(a296, a297) {
        let v3: bool = f9(v0, v2, 10u8);
        if v3 {
            Ok(None::<u64>)
        } else {
            Ok(Some({
                let a298 = v2;
                let a299 = 1u64;
                nat_add(a298, a299)?
            }))
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f39(v0: Bytes, v1: u64, v2: Adt4) -> R<Option<Adt4>> {
    let v3: u64 = match v2.clone() {
        Adt4::C0(h300, _, _, _) => {
            h300
        }
    };
    let v4: u64 = match v2.clone() {
        Adt4::C0(_, h301, _, _) => {
            h301
        }
    };
    let v5: u64 = match v2.clone() {
        Adt4::C0(_, _, h302, _) => {
            h302
        }
    };
    let a303 = v5;
    let a304 = 0u64;
    if nat_eq(a303, a304) {
        Ok(None::<Adt4>)
    } else {
        if match v2 {
            Adt4::C0(_, _, _, h305) => {
                h305
            }
        } {
            Ok(None::<Adt4>)
        } else {
            let a306 = v3;
            let a307 = v1;
            if nat_lt(a306, a307) {
                let v6: u8 = f8(v0.clone(), v3);
                let a308 = v6;
                let a309 = 47u8;
                if if equal(a308, a309) {
                    f9(v0.clone(), {
                        let a310 = v3;
                        let a311 = 1u64;
                        nat_add(a310, a311)?
                    }, 45u8)
                } else {
                    false
                } {
                    Ok(Some(Adt4::C0({
                        let a312 = v3;
                        let a313 = 2u64;
                        nat_add(a312, a313)?
                    }, v4, {
                        let a314 = v5;
                        let a315 = 1u64;
                        nat_add(a314, a315)?
                    }, false)))
                } else {
                    let a316 = v6;
                    let a317 = 45u8;
                    if if equal(a316, a317) {
                        f9(v0.clone(), {
                            let a318 = v3;
                            let a319 = 1u64;
                            nat_add(a318, a319)?
                        }, 47u8)
                    } else {
                        false
                    } {
                        Ok(Some(Adt4::C0({
                            let a320 = v3;
                            let a321 = 2u64;
                            nat_add(a320, a321)?
                        }, v4, {
                            let a322 = v5;
                            let a323 = 1u64;
                            nat_sub(a322, a323)
                        }, false)))
                    } else {
                        let a324 = v6;
                        let a325 = 10u8;
                        if equal(a324, a325) {
                            Ok(Some(Adt4::C0({
                                let a326 = v3;
                                let a327 = 1u64;
                                nat_add(a326, a327)?
                            }, {
                                let a328 = v4;
                                let a329 = 1u64;
                                nat_add(a328, a329)?
                            }, v5, false)))
                        } else {
                            Ok(Some(Adt4::C0({
                                let a330 = v3;
                                let a331 = f31(v0, v3);
                                nat_add(a330, a331)?
                            }, v4, v5, false)))
                        }
                    }
                }
            } else {
                Ok(Some(Adt4::C0(v3, v4, v5, true)))
            }
        }
    }
}

pub fn f40(v0: Fn5, v1: u64, v2: Adt5) -> R<(Adt5, bool)> {
    let m332 = v1;
    if m332 == 0 {
        Ok((v2, false))
    } else {
        let v3 = m332 - 1;
        let m334 = {
            let c333: Fn5 = v0.clone();
            c333.apply(v2.clone())?
        };
        match m334 {
            None => {
                Ok((v2, true))
            }
            Some(v4) => {
                f40(v0, v3, v4)
            }
        }
    }
}

pub fn f41(v0: u64, v1: Bytes, v2: Adt5) -> R<Option<Adt5>> {
    f55(v1, v0, v2)
}

pub fn f42(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let v3: (u64, (u64, bool)) = {
        let (h335, _) = f56(Fn6::F57(v2, v0), v2, (v1, (0u64, true)))?;
        h335
    };
    let (_, h336) = {
        let (_, h337) = v3;
        h337
    };
    if h336 {
        Ok(Some({
            let (h338, _) = v3;
            h338
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f43(v0: Bytes, v1: u64, v2: u64, v3: bool) -> R<Result<u64, Bytes>> {
    let v4: u64 = {
        let a339 = v0.clone();
        length_bytes(a339)
    };
    let v5: (u64, (bool, bool)) = {
        let (h340, _) = f58(Fn7::F59(v4, v2, v0), {
            let a343 = {
                let a341 = v4;
                let a342 = v1;
                nat_sub(a341, a342)
            };
            let a344 = 1u64;
            nat_add(a343, a344)?
        }, (v1, (v3, false)))?;
        h340
    };
    let (h345, _) = {
        let (_, h346) = v5;
        h346
    };
    if h345 {
        Ok(Err::<u64, Bytes>({
            let a347 = Str::lit("a number has a digit separator or a radix prefix with no digit after it");
            utf8_encode(a347)
        }))
    } else {
        Ok(Ok::<u64, Bytes>({
            let (h348, _) = v5;
            h348
        }))
    }
}

pub fn f44(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = {
        let a349 = v0.clone();
        length_bytes(a349)
    };
    let (h350, _) = f18(Fn2::F60(v0), {
        let a351 = v2;
        let a352 = v1;
        nat_sub(a351, a352)
    }, v1)?;
    Ok(h350)
}

pub fn f45(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let v2: bool = f9(v0.clone(), v1, 46u8);
    let v3: bool = f9(v0.clone(), {
        let a353 = v1;
        let a354 = 1u64;
        nat_add(a353, a354)?
    }, 46u8);
    let v4: bool = f34(v0.clone(), {
        let a355 = v1;
        let a356 = 1u64;
        nat_add(a355, a356)?
    }, 48u8, 57u8);
    let v5: bool = f9(v0.clone(), v1, 101u8);
    let v6: bool = f9(v0.clone(), v1, 69u8);
    let v7: Result<u64, Bytes> = f61(v0.clone(), {
        let a357 = v1;
        let a358 = 1u64;
        nat_add(a357, a358)?
    }, true)?;
    let v8: Result<u64, Bytes> = f61(v0.clone(), v1, false)?;
    let v11: Result<u64, Bytes> = {
        let m361 = f43(v0.clone(), {
            let a359 = v1;
            let a360 = 1u64;
            nat_add(a359, a360)?
        }, 10u64, false)?;
        match m361 {
            Ok(v10) => {
                f61(v0, v10, false)?
            }
            Err(v9) => {
                Err::<u64, Bytes>(v9)
            }
        }
    };
    if if v2 {
        v3
    } else {
        false
    } {
        Ok(Ok::<u64, Bytes>(v1))
    } else {
        if v2 {
            if v4 {
                Ok(v11)
            } else {
                Ok(v7)
            }
        } else {
            if if v5 {
                true
            } else {
                v6
            } {
                Ok(v8)
            } else {
                Ok(Ok::<u64, Bytes>(v1))
            }
        }
    }
}

pub fn f46(v0: u8) -> bool {
    let v1: bool = f33(v0, 97u8, 122u8);
    let v2: bool = f33(v0, 65u8, 90u8);
    if v1 {
        true
    } else {
        v2
    }
}

pub fn f47(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u8 = f8(v0.clone(), v1);
    let v3: u8 = f8(v0.clone(), {
        let a362 = v1;
        let a363 = 1u64;
        nat_add(a362, a363)?
    });
    let v4: u8 = f8(v0.clone(), {
        let a364 = v1;
        let a365 = 2u64;
        nat_add(a364, a365)?
    });
    let v5: u8 = f8(v0, {
        let a366 = v1;
        let a367 = 3u64;
        nat_add(a366, a367)?
    });
    let a368 = v2;
    let a369 = 206u8;
    if equal(a368, a369) {
        if f33(v3, 145u8, 169u8) {
            let a370 = v3;
            let a371 = 160u8;
            if equal(a370, a371) {
                Ok(0u64)
            } else {
                let a372 = v3;
                let a373 = 163u8;
                if equal(a372, a373) {
                    Ok(0u64)
                } else {
                    Ok(2u64)
                }
            }
        } else {
            if f33(v3, 177u8, 191u8) {
                let a374 = v3;
                let a375 = 187u8;
                if equal(a374, a375) {
                    Ok(0u64)
                } else {
                    Ok(2u64)
                }
            } else {
                Ok(0u64)
            }
        }
    } else {
        let a376 = v2;
        let a377 = 207u8;
        if equal(a376, a377) {
            if f33(v3, 128u8, 187u8) {
                Ok(2u64)
            } else {
                Ok(0u64)
            }
        } else {
            let a378 = v2;
            let a379 = 225u8;
            if equal(a378, a379) {
                if f33(v3, 188u8, 191u8) {
                    if f33(v4, 128u8, 191u8) {
                        let a380 = v3;
                        let a381 = 191u8;
                        if equal(a380, a381) {
                            let a382 = v4;
                            let a383 = 191u8;
                            if equal(a382, a383) {
                                Ok(0u64)
                            } else {
                                Ok(3u64)
                            }
                        } else {
                            Ok(3u64)
                        }
                    } else {
                        Ok(0u64)
                    }
                } else {
                    Ok(0u64)
                }
            } else {
                let a384 = v2;
                let a385 = 226u8;
                if equal(a384, a385) {
                    let a386 = v3;
                    let a387 = 132u8;
                    if equal(a386, a387) {
                        if f33(v4, 128u8, 191u8) {
                            Ok(3u64)
                        } else {
                            Ok(0u64)
                        }
                    } else {
                        let a388 = v3;
                        let a389 = 133u8;
                        if equal(a388, a389) {
                            if f33(v4, 128u8, 143u8) {
                                Ok(3u64)
                            } else {
                                Ok(0u64)
                            }
                        } else {
                            Ok(0u64)
                        }
                    }
                } else {
                    let a390 = v2;
                    let a391 = 240u8;
                    if equal(a390, a391) {
                        let a392 = v3;
                        let a393 = 157u8;
                        if equal(a392, a393) {
                            let a394 = v4;
                            let a395 = 146u8;
                            if equal(a394, a395) {
                                if f33(v5, 156u8, 191u8) {
                                    Ok(4u64)
                                } else {
                                    Ok(0u64)
                                }
                            } else {
                                if f33(v4, 147u8, 149u8) {
                                    if f33(v5, 128u8, 191u8) {
                                        Ok(4u64)
                                    } else {
                                        Ok(0u64)
                                    }
                                } else {
                                    let a396 = v4;
                                    let a397 = 150u8;
                                    if equal(a396, a397) {
                                        if f33(v5, 128u8, 159u8) {
                                            Ok(4u64)
                                        } else {
                                            Ok(0u64)
                                        }
                                    } else {
                                        Ok(0u64)
                                    }
                                }
                            }
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

pub fn f48(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f62(v0, v1)
}

pub fn f49(v0: Bytes, v1: u64) -> R<bool> {
    let v2: bool = f9(v0.clone(), v1, 194u8);
    let v3: bool = f9(v0, {
        let a398 = v1;
        let a399 = 1u64;
        nat_add(a398, a399)?
    }, 171u8);
    if v2 {
        Ok(v3)
    } else {
        Ok(false)
    }
}

pub fn f50(v0: Bytes, v1: u64) -> R<Result<(List<Adt2>, u64), Bytes>> {
    let v2: u64 = {
        let a400 = v0.clone();
        length_bytes(a400)
    };
    let v3: Adt6 = {
        let (h401, _) = f63(Fn8::F64(v0), {
            let a404 = {
                let a402 = v2;
                let a403 = v1;
                nat_sub(a402, a403)
            };
            let a405 = 1u64;
            nat_add(a404, a405)?
        }, Adt6::C0(v1, List::<Adt2>::nil(), false, None::<Bytes>))?;
        h401
    };
    let m407 = match v3.clone() {
        Adt6::C0(_, _, _, h406) => {
            h406
        }
    };
    match m407 {
        None => {
            let v5: List<Adt2> = f65(match v3.clone() {
                Adt6::C0(_, h408, _, _) => {
                    h408
                }
            });
            Ok(Ok::<(List<Adt2>, u64), Bytes>((v5, match v3 {
                Adt6::C0(h409, _, _, _) => {
                    h409
                }
            })))
        }
        Some(v4) => {
            Ok(Err::<(List<Adt2>, u64), Bytes>(v4))
        }
    }
}

pub fn f51(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = {
        let a410 = v0.clone();
        length_bytes(a410)
    };
    let v3: u64 = {
        let (h411, _) = f18(Fn2::F66(v0), {
            let a412 = v2;
            let a413 = v1;
            nat_sub(a412, a413)
        }, v1)?;
        h411
    };
    let a414 = v3;
    let a415 = v1;
    Ok(nat_sub(a414, a415))
}

pub fn f52(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt5> {
    let v4: u64 = {
        let a416 = v0.clone();
        length_bytes(a416)
    };
    let (h417, _) = f40(Fn5::F67(v3, v4, v0), {
        let a420 = {
            let a418 = v4;
            let a419 = v1;
            nat_sub(a418, a419)
        };
        let a421 = 1u64;
        nat_add(a420, a421)?
    }, Adt5::C0(v1, v2, false, false))?;
    Ok(h417)
}

pub fn f53(v0: Bytes, v1: Bytes) -> bool {
    let v2: u64 = {
        let a422 = v0.clone();
        length_bytes(a422)
    };
    let v3: u64 = {
        let a423 = v1.clone();
        length_bytes(a423)
    };
    let a424 = v3;
    let a425 = v2;
    if nat_le(a424, a425) {
        let m431 = {
            let a428 = v0;
            let a429 = {
                let a426 = v2;
                let a427 = v3;
                nat_sub(a426, a427)
            };
            let a430 = v3;
            slice_bytes(a428, a429, a430)
        };
        match m431 {
            None => {
                false
            }
            Some(v4) => {
                let a432 = v4;
                let a433 = v1;
                equal(a432, a433)
            }
        }
    } else {
        false
    }
}

pub fn f54(v0: Bytes, v1: u64, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v3: u64 = {
        let (h434, _) = v2;
        h434
    };
    let v4: u64 = {
        let (_, h435) = v2;
        h435
    };
    let a436 = v3;
    let a437 = v1;
    if nat_lt(a436, a437) {
        let v5: u64 = f31(v0, v3);
        Ok(Some(({
            let a438 = v3;
            let a439 = v5;
            nat_add(a438, a439)?
        }, {
            let a440 = v4;
            let a441 = 1u64;
            nat_add(a440, a441)?
        })))
    } else {
        Ok(None::<(u64, u64)>)
    }
}

pub fn f55(v0: Bytes, v1: u64, v2: Adt5) -> R<Option<Adt5>> {
    let v3: u64 = match v2.clone() {
        Adt5::C0(h442, _, _, _) => {
            h442
        }
    };
    let v4: u64 = match v2.clone() {
        Adt5::C0(_, h443, _, _) => {
            h443
        }
    };
    if match v2.clone() {
        Adt5::C0(_, _, h444, _) => {
            h444
        }
    } {
        Ok(None::<Adt5>)
    } else {
        if match v2 {
            Adt5::C0(_, _, _, h445) => {
                h445
            }
        } {
            Ok(None::<Adt5>)
        } else {
            let a446 = v3;
            let a447 = v1;
            if nat_lt(a446, a447) {
                let v5: u8 = f8(v0.clone(), v3);
                let a448 = v5;
                let a449 = 92u8;
                if equal(a448, a449) {
                    let a452 = {
                        let a450 = v3;
                        let a451 = 1u64;
                        nat_add(a450, a451)?
                    };
                    let a453 = v1;
                    if nat_lt(a452, a453) {
                        let v6: u64 = f31(v0, {
                            let a454 = v3;
                            let a455 = 1u64;
                            nat_add(a454, a455)?
                        });
                        Ok(Some(Adt5::C0({
                            let a458 = {
                                let a456 = v3;
                                let a457 = 1u64;
                                nat_add(a456, a457)?
                            };
                            let a459 = v6;
                            nat_add(a458, a459)?
                        }, v4, false, false)))
                    } else {
                        Ok(Some(Adt5::C0(v1, v4, false, true)))
                    }
                } else {
                    let a460 = v5;
                    let a461 = 34u8;
                    if equal(a460, a461) {
                        Ok(Some(Adt5::C0({
                            let a462 = v3;
                            let a463 = 1u64;
                            nat_add(a462, a463)?
                        }, v4, true, false)))
                    } else {
                        let a464 = v5;
                        let a465 = 10u8;
                        if equal(a464, a465) {
                            Ok(Some(Adt5::C0({
                                let a466 = v3;
                                let a467 = 1u64;
                                nat_add(a466, a467)?
                            }, {
                                let a468 = v4;
                                let a469 = 1u64;
                                nat_add(a468, a469)?
                            }, false, false)))
                        } else {
                            let v7: u64 = f31(v0, v3);
                            Ok(Some(Adt5::C0({
                                let a470 = v3;
                                let a471 = v7;
                                nat_add(a470, a471)?
                            }, v4, false, false)))
                        }
                    }
                }
            } else {
                Ok(Some(Adt5::C0(v3, v4, false, true)))
            }
        }
    }
}

pub fn f56(v0: Fn6, v1: u64, v2: (u64, (u64, bool))) -> R<((u64, (u64, bool)), bool)> {
    let m472 = v1;
    if m472 == 0 {
        Ok((v2, false))
    } else {
        let v3 = m472 - 1;
        let m474 = {
            let c473: Fn6 = v0.clone();
            c473.apply(v2)?
        };
        match m474 {
            None => {
                Ok((v2, true))
            }
            Some(v4) => {
                f56(v0, v3, v4)
            }
        }
    }
}

pub fn f57(v0: u64, v1: Bytes, v2: (u64, (u64, bool))) -> R<Option<(u64, (u64, bool))>> {
    f68(v1.clone(), {
        let a475 = v1;
        length_bytes(a475)
    }, v0, v2)
}

pub fn f58(v0: Fn7, v1: u64, v2: (u64, (bool, bool))) -> R<((u64, (bool, bool)), bool)> {
    let m476 = v1;
    if m476 == 0 {
        Ok((v2, false))
    } else {
        let v3 = m476 - 1;
        let m478 = {
            let c477: Fn7 = v0.clone();
            c477.apply(v2)?
        };
        match m478 {
            None => {
                Ok((v2, true))
            }
            Some(v4) => {
                f58(v0, v3, v4)
            }
        }
    }
}

pub fn f59(v0: u64, v1: u64, v2: Bytes, v3: (u64, (bool, bool))) -> R<Option<(u64, (bool, bool))>> {
    f69(v2, v0, v1, v3)
}

pub fn f60(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f70(v0, v1)
}

pub fn f61(v0: Bytes, v1: u64, v2: bool) -> R<Result<u64, Bytes>> {
    let v3: bool = f9(v0.clone(), v1, 101u8);
    let v4: bool = f9(v0.clone(), v1, 69u8);
    let v5: Result<u64, Bytes> = f71(v0.clone(), v1)?;
    let v6: bool = f29(v0, v1)?;
    if if v3 {
        true
    } else {
        v4
    } {
        Ok(v5)
    } else {
        if if v2 {
            v6
        } else {
            false
        } {
            Ok(Err::<u64, Bytes>({
                let a479 = Str::lit("an identifier follows a decimal point");
                utf8_encode(a479)
            }))
        } else {
            Ok(Ok::<u64, Bytes>(v1))
        }
    }
}

pub fn f62(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f72(v0, v1)?;
    let a480 = 0u64;
    let a481 = v2;
    if nat_lt(a480, a481) {
        Ok(Some({
            let a482 = v1;
            let a483 = v2;
            nat_add(a482, a483)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f63(v0: Fn8, v1: u64, v2: Adt6) -> R<(Adt6, bool)> {
    let m484 = v1;
    if m484 == 0 {
        Ok((v2, false))
    } else {
        let v3 = m484 - 1;
        let m486 = {
            let c485: Fn8 = v0.clone();
            c485.apply(v2.clone())?
        };
        match m486 {
            None => {
                Ok((v2, true))
            }
            Some(v4) => {
                f63(v0, v3, v4)
            }
        }
    }
}

pub fn f64(v0: Bytes, v1: Adt6) -> R<Option<Adt6>> {
    f73(v0, v1)
}

pub fn f65(v0: List<Adt2>) -> List<Adt2> {
    f74(Fn9::F75, List::<Adt2>::nil(), v0)
}

pub fn f66(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f76(v0, v1)
}

pub fn f67(v0: u64, v1: u64, v2: Bytes, v3: Adt5) -> R<Option<Adt5>> {
    f77(v2, v1, v0, v3)
}

pub fn f68(v0: Bytes, v1: u64, v2: u64, v3: (u64, (u64, bool))) -> R<Option<(u64, (u64, bool))>> {
    let v4: u64 = {
        let (h487, _) = v3;
        h487
    };
    let v5: u64 = {
        let (h488, _) = {
            let (_, h489) = v3;
            h489
        };
        h488
    };
    let v6: bool = {
        let (_, h490) = {
            let (_, h491) = v3;
            h491
        };
        h490
    };
    if v6 {
        let a492 = v5;
        let a493 = v2;
        if nat_lt(a492, a493) {
            let a494 = v4;
            let a495 = v1;
            if nat_lt(a494, a495) {
                let v7: u64 = f31(v0, v4);
                Ok(Some(({
                    let a496 = v4;
                    let a497 = v7;
                    nat_add(a496, a497)?
                }, ({
                    let a498 = v5;
                    let a499 = 1u64;
                    nat_add(a498, a499)?
                }, true))))
            } else {
                Ok(Some((v4, ({
                    let a500 = v5;
                    let a501 = 1u64;
                    nat_add(a500, a501)?
                }, false))))
            }
        } else {
            Ok(None::<(u64, (u64, bool))>)
        }
    } else {
        Ok(None::<(u64, (u64, bool))>)
    }
}

pub fn f69(v0: Bytes, v1: u64, v2: u64, v3: (u64, (bool, bool))) -> R<Option<(u64, (bool, bool))>> {
    let v4: u64 = {
        let (h502, _) = v3;
        h502
    };
    let v5: bool = {
        let (h503, _) = {
            let (_, h504) = v3;
            h504
        };
        h503
    };
    let v6: bool = {
        let (_, h505) = {
            let (_, h506) = v3;
            h506
        };
        h505
    };
    if v6 {
        Ok(None::<(u64, (bool, bool))>)
    } else {
        let a507 = v4;
        let a508 = v1;
        if nat_lt(a507, a508) {
            let v7: u8 = f8(v0, v4);
            let a509 = v7;
            let a510 = 95u8;
            if equal(a509, a510) {
                Ok(Some(({
                    let a511 = v4;
                    let a512 = 1u64;
                    nat_add(a511, a512)?
                }, (true, false))))
            } else {
                let v8: bool = f78(v7, v2);
                if v8 {
                    Ok(Some(({
                        let a513 = v4;
                        let a514 = 1u64;
                        nat_add(a513, a514)?
                    }, (false, false))))
                } else {
                    Ok(Some((v4, (v5, true))))
                }
            }
        } else {
            Ok(Some((v4, (v5, true))))
        }
    }
}

pub fn f70(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: bool = f34(v0, v1, 48u8, 57u8);
    if v2 {
        Ok(Some({
            let a515 = v1;
            let a516 = 1u64;
            nat_add(a515, a516)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f71(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let v2: u64 = {
        let a517 = v1;
        let a518 = 1u64;
        nat_add(a517, a518)?
    };
    let v3: bool = f9(v0.clone(), v2, 43u8);
    let v4: bool = f9(v0.clone(), v2, 45u8);
    let v5: u64 = if v3 {
        let a519 = v2;
        let a520 = 1u64;
        nat_add(a519, a520)?
    } else {
        if v4 {
            let a521 = v2;
            let a522 = 1u64;
            nat_add(a521, a522)?
        } else {
            v2
        }
    };
    let v6: bool = f34(v0.clone(), v5, 48u8, 57u8);
    let v7: Result<u64, Bytes> = f43(v0, v5, 10u64, false)?;
    if v6 {
        Ok(v7)
    } else {
        Ok(Err::<u64, Bytes>({
            let a523 = Str::lit("a scientific literal has no exponent digits");
            utf8_encode(a523)
        }))
    }
}

pub fn f72(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u8 = f8(v0.clone(), v1);
    let v3: bool = f46(v2);
    let v4: bool = f32(v2);
    let v5: u64 = f47(v0.clone(), v1)?;
    let v6: u64 = f79(v0.clone(), v1)?;
    let a525 = v1;
    let a526 = {
        let a524 = v0;
        length_bytes(a524)
    };
    if nat_lt(a525, a526) {
        if v3 {
            Ok(1u64)
        } else {
            if v4 {
                Ok(1u64)
            } else {
                let a527 = v2;
                let a528 = 95u8;
                if equal(a527, a528) {
                    Ok(1u64)
                } else {
                    let a529 = v2;
                    let a530 = 39u8;
                    if equal(a529, a530) {
                        Ok(1u64)
                    } else {
                        let a531 = v2;
                        let a532 = 33u8;
                        if equal(a531, a532) {
                            Ok(1u64)
                        } else {
                            let a533 = v2;
                            let a534 = 63u8;
                            if equal(a533, a534) {
                                Ok(1u64)
                            } else {
                                let a535 = 0u64;
                                let a536 = v5;
                                if nat_lt(a535, a536) {
                                    Ok(v5)
                                } else {
                                    Ok(v6)
                                }
                            }
                        }
                    }
                }
            }
        }
    } else {
        Ok(0u64)
    }
}

pub fn f73(v0: Bytes, v1: Adt6) -> R<Option<Adt6>> {
    let v2: bool = f49(v0.clone(), match v1.clone() {
        Adt6::C0(h537, _, _, _) => {
            h537
        }
    })?;
    let v3: Result<(Adt2, u64), Bytes> = f80(v0.clone(), match v1.clone() {
        Adt6::C0(h538, _, _, _) => {
            h538
        }
    })?;
    let v4: u64 = f27(v0.clone(), {
        let a541 = match v1.clone() {
            Adt6::C0(h539, _, _, _) => {
                h539
            }
        };
        let a542 = f26(v0.clone(), match v1.clone() {
            Adt6::C0(h540, _, _, _) => {
                h540
            }
        })?;
        nat_add(a541, a542)?
    })?;
    let v5: Result<(Adt2, u64), Bytes> = Ok::<(Adt2, u64), Bytes>((Adt2::C0(f28(v0.clone(), match v1.clone() {
        Adt6::C0(h543, _, _, _) => {
            h543
        }
    }, {
        let a545 = v4;
        let a546 = match v1.clone() {
            Adt6::C0(h544, _, _, _) => {
                h544
            }
        };
        nat_sub(a545, a546)
    }), false), v4));
    if match v1.clone() {
        Adt6::C0(_, _, h547, _) => {
            h547
        }
    } {
        Ok(None::<Adt6>)
    } else {
        let m548 = if v2 {
            v3
        } else {
            v5
        };
        match m548 {
            Ok(v7) => {
                let v8: bool = f9(v0.clone(), {
                    let (_, h551) = v7.clone();
                    h551
                }, 46u8);
                let v9: bool = f29(v0, {
                    let a553 = {
                        let (_, h552) = v7.clone();
                        h552
                    };
                    let a554 = 1u64;
                    nat_add(a553, a554)?
                })?;
                if if v8 {
                    v9
                } else {
                    false
                } {
                    Ok(Some(Adt6::C0({
                        let a556 = {
                            let (_, h555) = v7;
                            h555
                        };
                        let a557 = 1u64;
                        nat_add(a556, a557)?
                    }, List::cons({
                        let (h558, _) = v7;
                        h558
                    }, match v1 {
                        Adt6::C0(_, h559, _, _) => {
                            h559
                        }
                    }), false, None::<Bytes>)))
                } else {
                    Ok(Some(Adt6::C0({
                        let (_, h560) = v7;
                        h560
                    }, List::cons({
                        let (h561, _) = v7;
                        h561
                    }, match v1 {
                        Adt6::C0(_, h562, _, _) => {
                            h562
                        }
                    }), true, None::<Bytes>)))
                }
            }
            Err(v6) => {
                Ok(Some(Adt6::C0(match v1.clone() {
                    Adt6::C0(h549, _, _, _) => {
                        h549
                    }
                }, match v1 {
                    Adt6::C0(_, h550, _, _) => {
                        h550
                    }
                }, true, Some(v6))))
            }
        }
    }
}

pub fn f74(v0: Fn9, v1: List<Adt2>, v2: List<Adt2>) -> List<Adt2> {
    let m563 = v2;
    match m563.uncons() {
        None => {
            v1
        }
        Some((v3, v4)) => {
            f74(v0.clone(), {
                let c564: Fn9 = v0;
                c564.apply(v1, v3)
            }, v4)
        }
    }
}

pub fn f75(v0: List<Adt2>, v1: Adt2) -> List<Adt2> {
    List::cons(v1, v0)
}

pub fn f76(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: bool = f9(v0, v1, 35u8);
    if v2 {
        Ok(Some({
            let a565 = v1;
            let a566 = 1u64;
            nat_add(a565, a566)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f77(v0: Bytes, v1: u64, v2: u64, v3: Adt5) -> R<Option<Adt5>> {
    let v4: u64 = match v3.clone() {
        Adt5::C0(h567, _, _, _) => {
            h567
        }
    };
    let v5: u64 = match v3.clone() {
        Adt5::C0(_, h568, _, _) => {
            h568
        }
    };
    if match v3.clone() {
        Adt5::C0(_, _, h569, _) => {
            h569
        }
    } {
        Ok(None::<Adt5>)
    } else {
        if match v3 {
            Adt5::C0(_, _, _, h570) => {
                h570
            }
        } {
            Ok(None::<Adt5>)
        } else {
            let a571 = v4;
            let a572 = v1;
            if nat_lt(a571, a572) {
                let v6: u8 = f8(v0.clone(), v4);
                let a573 = v6;
                let a574 = 34u8;
                if equal(a573, a574) {
                    let v7: bool = f81(v0, {
                        let a575 = v4;
                        let a576 = 1u64;
                        nat_add(a575, a576)?
                    }, v2)?;
                    if v7 {
                        Ok(Some(Adt5::C0({
                            let a579 = {
                                let a577 = v4;
                                let a578 = 1u64;
                                nat_add(a577, a578)?
                            };
                            let a580 = v2;
                            nat_add(a579, a580)?
                        }, v5, true, false)))
                    } else {
                        Ok(Some(Adt5::C0({
                            let a581 = v4;
                            let a582 = 1u64;
                            nat_add(a581, a582)?
                        }, v5, false, false)))
                    }
                } else {
                    let a583 = v6;
                    let a584 = 10u8;
                    if equal(a583, a584) {
                        Ok(Some(Adt5::C0({
                            let a585 = v4;
                            let a586 = 1u64;
                            nat_add(a585, a586)?
                        }, {
                            let a587 = v5;
                            let a588 = 1u64;
                            nat_add(a587, a588)?
                        }, false, false)))
                    } else {
                        let v8: u64 = f31(v0, v4);
                        Ok(Some(Adt5::C0({
                            let a589 = v4;
                            let a590 = v8;
                            nat_add(a589, a590)?
                        }, v5, false, false)))
                    }
                }
            } else {
                Ok(Some(Adt5::C0(v4, v5, false, true)))
            }
        }
    }
}

pub fn f78(v0: u8, v1: u64) -> bool {
    let v2: bool = f33(v0, 48u8, 49u8);
    let v3: bool = f33(v0, 48u8, 55u8);
    let v4: bool = f33(v0, 48u8, 57u8);
    let v5: bool = f33(v0, 97u8, 102u8);
    let v6: bool = f33(v0, 65u8, 70u8);
    let a591 = v1;
    let a592 = 2u64;
    if nat_eq(a591, a592) {
        v2
    } else {
        let a593 = v1;
        let a594 = 8u64;
        if nat_eq(a593, a594) {
            v3
        } else {
            let a595 = v1;
            let a596 = 16u64;
            if nat_eq(a595, a596) {
                if v4 {
                    true
                } else {
                    if v5 {
                        true
                    } else {
                        v6
                    }
                }
            } else {
                v4
            }
        }
    }
}

pub fn f79(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u8 = f8(v0.clone(), v1);
    let v3: u8 = f8(v0.clone(), {
        let a597 = v1;
        let a598 = 1u64;
        nat_add(a597, a598)?
    });
    let v4: u8 = f8(v0, {
        let a599 = v1;
        let a600 = 2u64;
        nat_add(a599, a600)?
    });
    let a601 = v2;
    let a602 = 225u8;
    if equal(a601, a602) {
        let a603 = v3;
        let a604 = 181u8;
        if equal(a603, a604) {
            if f33(v4, 162u8, 170u8) {
                Ok(3u64)
            } else {
                Ok(0u64)
            }
        } else {
            Ok(0u64)
        }
    } else {
        let a605 = v2;
        let a606 = 226u8;
        if equal(a605, a606) {
            let a607 = v3;
            let a608 = 130u8;
            if equal(a607, a608) {
                if f33(v4, 128u8, 137u8) {
                    Ok(3u64)
                } else {
                    if f33(v4, 144u8, 156u8) {
                        Ok(3u64)
                    } else {
                        Ok(0u64)
                    }
                }
            } else {
                Ok(0u64)
            }
        } else {
            Ok(0u64)
        }
    }
}

pub fn f80(v0: Bytes, v1: u64) -> R<Result<(Adt2, u64), Bytes>> {
    let v2: u64 = {
        let a609 = v1;
        let a610 = 2u64;
        nat_add(a609, a610)?
    };
    let v3: Adt7 = {
        let (h611, _) = f82(Fn10::F83(v0.clone()), {
            let a615 = {
                let a613 = {
                    let a612 = v0.clone();
                    length_bytes(a612)
                };
                let a614 = v2;
                nat_sub(a613, a614)
            };
            let a616 = 1u64;
            nat_add(a615, a616)?
        }, Adt7::C0(v2, false, false, true))?;
        h611
    };
    let v4: Bytes = f28(v0, v2, {
        let a618 = match v3.clone() {
            Adt7::C0(h617, _, _, _) => {
                h617
            }
        };
        let a619 = v2;
        nat_sub(a618, a619)
    });
    if match v3.clone() {
        Adt7::C0(_, h620, _, _) => {
            h620
        }
    } {
        let a622 = 0u64;
        let a623 = {
            let a621 = v4.clone();
            length_bytes(a621)
        };
        if if nat_lt(a622, a623) {
            match v3.clone() {
                Adt7::C0(_, _, _, h624) => {
                    h624
                }
            }
        } else {
            false
        } {
            Ok(Ok::<(Adt2, u64), Bytes>((Adt2::C0(v4, true), {
                let a626 = match v3 {
                    Adt7::C0(h625, _, _, _) => {
                        h625
                    }
                };
                let a627 = 2u64;
                nat_add(a626, a627)?
            })))
        } else {
            Ok(Err::<(Adt2, u64), Bytes>({
                let a632 = {
                    let a629 = {
                        let a628 = Str::lit("the quoted name `");
                        utf8_encode(a628)
                    };
                    let a630 = v4;
                    append_bytes(a629, a630)
                };
                let a633 = {
                    let a631 = Str::lit("` is not a name");
                    utf8_encode(a631)
                };
                append_bytes(a632, a633)
            }))
        }
    } else {
        Ok(Err::<(Adt2, u64), Bytes>({
            let a634 = Str::lit("a quoted name is not closed");
            utf8_encode(a634)
        }))
    }
}

pub fn f81(v0: Bytes, v1: u64, v2: u64) -> R<bool> {
    let (_, h635) = {
        let (h636, _) = f84(Fn11::F85(v2, v1, v0), v2, (0u64, true))?;
        h636
    };
    Ok(h635)
}

pub fn f82(v0: Fn10, v1: u64, v2: Adt7) -> R<(Adt7, bool)> {
    let m637 = v1;
    if m637 == 0 {
        Ok((v2, false))
    } else {
        let v3 = m637 - 1;
        let m639 = {
            let c638: Fn10 = v0.clone();
            c638.apply(v2.clone())?
        };
        match m639 {
            None => {
                Ok((v2, true))
            }
            Some(v4) => {
                f82(v0, v3, v4)
            }
        }
    }
}

pub fn f83(v0: Bytes, v1: Adt7) -> R<Option<Adt7>> {
    f86(v0.clone(), {
        let a640 = v0;
        length_bytes(a640)
    }, v1)
}

pub fn f84(v0: Fn11, v1: u64, v2: (u64, bool)) -> R<((u64, bool), bool)> {
    let m641 = v1;
    if m641 == 0 {
        Ok((v2, false))
    } else {
        let v3 = m641 - 1;
        let m643 = {
            let c642: Fn11 = v0.clone();
            c642.apply(v2)?
        };
        match m643 {
            None => {
                Ok((v2, true))
            }
            Some(v4) => {
                f84(v0, v3, v4)
            }
        }
    }
}

pub fn f85(v0: u64, v1: u64, v2: Bytes, v3: (u64, bool)) -> R<Option<(u64, bool)>> {
    f87(v2, v1, v0, v3)
}

pub fn f86(v0: Bytes, v1: u64, v2: Adt7) -> R<Option<Adt7>> {
    let v3: u64 = match v2.clone() {
        Adt7::C0(h644, _, _, _) => {
            h644
        }
    };
    let v4: bool = match v2.clone() {
        Adt7::C0(_, _, _, h645) => {
            h645
        }
    };
    if match v2.clone() {
        Adt7::C0(_, h646, _, _) => {
            h646
        }
    } {
        Ok(None::<Adt7>)
    } else {
        if match v2 {
            Adt7::C0(_, _, h647, _) => {
                h647
            }
        } {
            Ok(None::<Adt7>)
        } else {
            let a648 = v3;
            let a649 = v1;
            if nat_lt(a648, a649) {
                let v5: bool = f9(v0.clone(), v3, 10u8);
                if v5 {
                    Ok(Some(Adt7::C0(v3, false, true, v4)))
                } else {
                    let v6: bool = f88(v0.clone(), v3)?;
                    if v6 {
                        Ok(Some(Adt7::C0(v3, true, false, v4)))
                    } else {
                        let v7: u64 = f72(v0.clone(), v3)?;
                        let a650 = 0u64;
                        let a651 = v7;
                        if nat_lt(a650, a651) {
                            Ok(Some(Adt7::C0({
                                let a652 = v3;
                                let a653 = v7;
                                nat_add(a652, a653)?
                            }, false, false, v4)))
                        } else {
                            let v8: bool = f9(v0.clone(), v3, 46u8);
                            if v8 {
                                Ok(Some(Adt7::C0({
                                    let a654 = v3;
                                    let a655 = 1u64;
                                    nat_add(a654, a655)?
                                }, false, false, v4)))
                            } else {
                                let v9: u64 = f31(v0, v3);
                                Ok(Some(Adt7::C0({
                                    let a656 = v3;
                                    let a657 = v9;
                                    nat_add(a656, a657)?
                                }, false, false, false)))
                            }
                        }
                    }
                }
            } else {
                Ok(Some(Adt7::C0(v3, false, true, v4)))
            }
        }
    }
}

pub fn f87(v0: Bytes, v1: u64, v2: u64, v3: (u64, bool)) -> R<Option<(u64, bool)>> {
    let v4: u64 = {
        let (h658, _) = v3;
        h658
    };
    let v5: bool = {
        let (_, h659) = v3;
        h659
    };
    if v5 {
        let a660 = v4;
        let a661 = v2;
        if nat_lt(a660, a661) {
            let v6: bool = f9(v0, {
                let a662 = v1;
                let a663 = v4;
                nat_add(a662, a663)?
            }, 35u8);
            Ok(Some(({
                let a664 = v4;
                let a665 = 1u64;
                nat_add(a664, a665)?
            }, v6)))
        } else {
            Ok(None::<(u64, bool)>)
        }
    } else {
        Ok(None::<(u64, bool)>)
    }
}

pub fn f88(v0: Bytes, v1: u64) -> R<bool> {
    let v2: bool = f9(v0.clone(), v1, 194u8);
    let v3: bool = f9(v0, {
        let a666 = v1;
        let a667 = 1u64;
        nat_add(a666, a667)?
    }, 187u8);
    if v2 {
        Ok(v3)
    } else {
        Ok(false)
    }
}

pub fn run(p0: Bytes) -> R<Result<List<Adt0>, Bytes>> {
    f0(p0)
}
