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
    C0(List<Adt1>, u64, bool, Option<Bytes>),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(Adt2, u64, bool, u64),
}

#[derive(Clone)]
pub enum Adt2 {
    C0(List<Adt3>),
    C1,
    C2,
    C3(Bytes),
    C4(Bytes),
}

#[derive(Clone)]
pub enum Adt3 {
    C0(Bytes, bool),
}

#[derive(Clone)]
pub enum Adt4 {
    C0(u64, u64, bool, u64, List<Adt1>, Option<Bytes>),
}

#[derive(Clone)]
pub enum Adt5 {
    C0(u64, u64, u64, bool),
}

#[derive(Clone)]
pub enum Adt6 {
    C0(u64, u64, bool, bool),
}

#[derive(Clone)]
pub enum Adt7 {
    C0(u64, List<Adt3>, bool, Option<Bytes>),
}

#[derive(Clone)]
pub enum Adt8 {
    C0(u64, bool, bool, bool),
}

#[derive(Clone)]
pub enum Fn0 {
    F4(List<Bytes>),
}

impl Fn0 {
    pub fn apply(&self, p0: Adt0) -> R<Option<Adt0>> {
        match self {
            Fn0::F4(k0) => f4(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F6(u64, Bytes),
}

impl Fn1 {
    pub fn apply(&self, p0: Adt4) -> R<Option<Adt4>> {
        match self {
            Fn1::F6(k0, k1) => f6(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F9,
}

impl Fn2 {
    pub fn apply(&self, p0: u64, p1: Adt1) -> R<u64> {
        match self {
            Fn2::F9 => f9(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F13,
}

impl Fn3 {
    pub fn apply(&self, p0: List<Adt1>, p1: Adt1) -> List<Adt1> {
        match self {
            Fn3::F13 => f13(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F33,
}

impl Fn4 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Adt3) -> Option<Bytes> {
        match self {
            Fn4::F33 => f33(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F36,
}

impl Fn5 {
    pub fn apply(&self, p0: List<Bytes>, p1: Adt3) -> List<Bytes> {
        match self {
            Fn5::F36 => f36(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F39(Bytes),
}

impl Fn6 {
    pub fn apply(&self, p0: bool, p1: Bytes) -> bool {
        match self {
            Fn6::F39(k0) => f39(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F42(u64, Bytes),
    F76(Bytes),
    F88(Bytes),
    F94(Bytes),
}

impl Fn7 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn7::F42(k0, k1) => f42(*k0, k1.clone(), p0),
            Fn7::F76(k0) => f76(k0.clone(), p0),
            Fn7::F88(k0) => f88(k0.clone(), p0),
            Fn7::F94(k0) => f94(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F44(u64, Bytes),
}

impl Fn8 {
    pub fn apply(&self, p0: Adt5) -> R<Option<Adt5>> {
        match self {
            Fn8::F44(k0, k1) => f44(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F60(Bytes, u64),
}

impl Fn9 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn9::F60(k0, k1) => f60(k0.clone(), *k1, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F63,
}

impl Fn10 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn10::F63 => f63(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F65(Bytes),
}

impl Fn11 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Bytes) -> Option<Bytes> {
        match self {
            Fn11::F65(k0) => f65(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F69(u64, Bytes),
    F95(u64, u64, Bytes),
}

impl Fn12 {
    pub fn apply(&self, p0: Adt6) -> R<Option<Adt6>> {
        match self {
            Fn12::F69(k0, k1) => f69(*k0, k1.clone(), p0),
            Fn12::F95(k0, k1, k2) => f95(*k0, *k1, k2.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F85(u64, Bytes),
}

impl Fn13 {
    pub fn apply(&self, p0: (u64, (u64, bool))) -> R<Option<(u64, (u64, bool))>> {
        match self {
            Fn13::F85(k0, k1) => f85(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F87(u64, u64, Bytes),
}

impl Fn14 {
    pub fn apply(&self, p0: (u64, (bool, bool))) -> R<Option<(u64, (bool, bool))>> {
        match self {
            Fn14::F87(k0, k1, k2) => f87(*k0, *k1, k2.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F92(Bytes),
}

impl Fn15 {
    pub fn apply(&self, p0: Adt7) -> R<Option<Adt7>> {
        match self {
            Fn15::F92(k0) => f92(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F103,
}

impl Fn16 {
    pub fn apply(&self, p0: List<Adt3>, p1: Adt3) -> List<Adt3> {
        match self {
            Fn16::F103 => f103(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F111(Bytes),
}

impl Fn17 {
    pub fn apply(&self, p0: Adt8) -> R<Option<Adt8>> {
        match self {
            Fn17::F111(k0) => f111(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F113(u64, u64, Bytes),
}

impl Fn18 {
    pub fn apply(&self, p0: (u64, bool)) -> R<Option<(u64, bool)>> {
        match self {
            Fn18::F113(k0, k1, k2) => f113(*k0, *k1, k2.clone(), p0),
        }
    }
}

pub fn f0(v0: Bytes, v1: List<Bytes>) -> R<Result<(), Bytes>> {
    let m1 = f1(v0)?;
    match m1 {
        Ok(v3) => {
            let v4: u64 = f2(v3.clone())?;
            let v5: Adt0 = {
                let (h2, _) = f3(Fn0::F4(v1), {
                    let a3 = v4;
                    let a4 = 1u64;
                    nat_add(a3, a4)?
                }, Adt0::C0(v3, 0u64, false, None::<Bytes>))?;
                h2
            };
            let m6 = match v5 {
                Adt0::C0(_, _, _, h5) => {
                    h5
                }
            };
            match m6 {
                None => {
                    Ok(Ok::<(), Bytes>(()))
                }
                Some(v6) => {
                    Ok(Err::<(), Bytes>(v6))
                }
            }
        }
        Err(v2) => {
            Ok(Err::<(), Bytes>(v2))
        }
    }
}

pub fn f1(v0: Bytes) -> R<Result<List<Adt1>, Bytes>> {
    let v1: u64 = {
        let a7 = v0.clone();
        length_bytes(a7)
    };
    let v2: Adt4 = {
        let (h8, _) = f5(Fn1::F6(v1, v0), {
            let a9 = v1;
            let a10 = 1u64;
            nat_add(a9, a10)?
        }, Adt4::C0(0u64, 0u64, true, 0u64, List::<Adt1>::nil(), None::<Bytes>))?;
        h8
    };
    let m12 = match v2.clone() {
        Adt4::C0(_, _, _, _, _, h11) => {
            h11
        }
    };
    match m12 {
        None => {
            let v4: List<Adt1> = f7(match v2 {
                Adt4::C0(_, _, _, _, h13, _) => {
                    h13
                }
            });
            Ok(Ok::<List<Adt1>, Bytes>(v4))
        }
        Some(v3) => {
            Ok(Err::<List<Adt1>, Bytes>(v3))
        }
    }
}

pub fn f2(v0: List<Adt1>) -> R<u64> {
    f8(Fn2::F9, 0u64, v0)
}

pub fn f3(v0: Fn0, v1: u64, v2: Adt0) -> R<(Adt0, bool)> {
    let m14 = v1;
    if m14 == 0 {
        Ok((v2, false))
    } else {
        let v3 = m14 - 1;
        let m16 = {
            let c15: Fn0 = v0.clone();
            c15.apply(v2.clone())?
        };
        match m16 {
            None => {
                Ok((v2, true))
            }
            Some(v4) => {
                f3(v0, v3, v4)
            }
        }
    }
}

pub fn f4(v0: List<Bytes>, v1: Adt0) -> R<Option<Adt0>> {
    f10(v0, v1)
}

pub fn f5(v0: Fn1, v1: u64, v2: Adt4) -> R<(Adt4, bool)> {
    let m17 = v1;
    if m17 == 0 {
        Ok((v2, false))
    } else {
        let v3 = m17 - 1;
        let m19 = {
            let c18: Fn1 = v0.clone();
            c18.apply(v2.clone())?
        };
        match m19 {
            None => {
                Ok((v2, true))
            }
            Some(v4) => {
                f5(v0, v3, v4)
            }
        }
    }
}

pub fn f6(v0: u64, v1: Bytes, v2: Adt4) -> R<Option<Adt4>> {
    f11(v1, v0, v2)
}

pub fn f7(v0: List<Adt1>) -> List<Adt1> {
    f12(Fn3::F13, List::<Adt1>::nil(), v0)
}

pub fn f8(v0: Fn2, v1: u64, v2: List<Adt1>) -> R<u64> {
    let m20 = v2;
    match m20.uncons() {
        None => {
            Ok(v1)
        }
        Some((v3, v4)) => {
            f8(v0.clone(), {
                let c21: Fn2 = v0;
                c21.apply(v1, v3)?
            }, v4)
        }
    }
}

pub fn f9(v0: u64, _: Adt1) -> R<u64> {
    let a22 = v0;
    let a23 = 1u64;
    nat_add(a22, a23)
}

pub fn f10(v0: List<Bytes>, v1: Adt0) -> R<Option<Adt0>> {
    let v2: u64 = match v1.clone() {
        Adt0::C0(_, h24, _, _) => {
            h24
        }
    };
    let v3: bool = match v1.clone() {
        Adt0::C0(_, _, h25, _) => {
            h25
        }
    };
    let m27 = match v1.clone() {
        Adt0::C0(_, _, _, h26) => {
            h26
        }
    };
    match m27 {
        None => {
            let m29 = match v1 {
                Adt0::C0(h28, _, _, _) => {
                    h28
                }
            };
            match m29.uncons() {
                None => {
                    Ok(None::<Adt0>)
                }
                Some((v5, v6)) => {
                    let m31 = match v5 {
                        Adt1::C0(h30, _, _, _) => {
                            h30
                        }
                    };
                    match m31 {
                        Adt2::C0(v7) => {
                            let v8: Adt0 = f14(v7, v6, v2, v3, v0);
                            Ok(Some(v8))
                        }
                        Adt2::C1 => {
                            Ok(Some(Adt0::C0(v6, v2, v3, None::<Bytes>)))
                        }
                        Adt2::C2 => {
                            Ok(Some(Adt0::C0(v6, v2, v3, None::<Bytes>)))
                        }
                        Adt2::C3(v9) => {
                            let v10: Bytes = {
                                let a36 = {
                                    let a33 = {
                                        let a32 = Str::lit("the command `");
                                        utf8_encode(a32)
                                    };
                                    let a34 = v9;
                                    append_bytes(a33, a34)
                                };
                                let a37 = {
                                    let a35 = Str::lit("`");
                                    utf8_encode(a35)
                                };
                                append_bytes(a36, a37)
                            };
                            Ok(Some(Adt0::C0(v6, v2, v3, Some(v10))))
                        }
                        Adt2::C4(v11) => {
                            let a39 = v11.clone();
                            let a40 = {
                                let a38 = Str::lit("@");
                                utf8_encode(a38)
                            };
                            if if equal(a39, a40) {
                                f15(v6.clone())
                            } else {
                                false
                            } {
                                Ok(Some(Adt0::C0(v6, v2, true, None::<Bytes>)))
                            } else {
                                let a42 = v11.clone();
                                let a43 = {
                                    let a41 = Str::lit("[");
                                    utf8_encode(a41)
                                };
                                if equal(a42, a43) {
                                    if v3 {
                                        Ok(Some(Adt0::C0(v6, 1u64, false, None::<Bytes>)))
                                    } else {
                                        let a44 = 0u64;
                                        let a45 = v2;
                                        if nat_lt(a44, a45) {
                                            Ok(Some(Adt0::C0(v6, {
                                                let a46 = v2;
                                                let a47 = 1u64;
                                                nat_add(a46, a47)?
                                            }, false, None::<Bytes>)))
                                        } else {
                                            Ok(Some(Adt0::C0(v6, 0u64, false, None::<Bytes>)))
                                        }
                                    }
                                } else {
                                    let a49 = v11;
                                    let a50 = {
                                        let a48 = Str::lit("]");
                                        utf8_encode(a48)
                                    };
                                    if equal(a49, a50) {
                                        let a51 = 0u64;
                                        let a52 = v2;
                                        if nat_lt(a51, a52) {
                                            Ok(Some(Adt0::C0(v6, {
                                                let a53 = v2;
                                                let a54 = 1u64;
                                                nat_sub(a53, a54)
                                            }, v3, None::<Bytes>)))
                                        } else {
                                            Ok(Some(Adt0::C0(v6, v2, v3, None::<Bytes>)))
                                        }
                                    } else {
                                        Ok(Some(Adt0::C0(v6, v2, v3, None::<Bytes>)))
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Some(_) => {
            Ok(None::<Adt0>)
        }
    }
}

pub fn f11(v0: Bytes, v1: u64, v2: Adt4) -> R<Option<Adt4>> {
    let m56 = match v2.clone() {
        Adt4::C0(_, _, _, _, _, h55) => {
            h55
        }
    };
    match m56 {
        None => {
            let v4: Option<Adt4> = f16(v0, match v2.clone() {
                Adt4::C0(h57, _, _, _, _, _) => {
                    h57
                }
            }, match v2.clone() {
                Adt4::C0(_, h58, _, _, _, _) => {
                    h58
                }
            }, match v2.clone() {
                Adt4::C0(_, _, h59, _, _, _) => {
                    h59
                }
            }, match v2.clone() {
                Adt4::C0(_, _, _, h60, _, _) => {
                    h60
                }
            }, match v2.clone() {
                Adt4::C0(_, _, _, _, h61, _) => {
                    h61
                }
            })?;
            let a63 = match v2 {
                Adt4::C0(h62, _, _, _, _, _) => {
                    h62
                }
            };
            let a64 = v1;
            if nat_lt(a63, a64) {
                Ok(v4)
            } else {
                Ok(None::<Adt4>)
            }
        }
        Some(_) => {
            Ok(None::<Adt4>)
        }
    }
}

pub fn f12(v0: Fn3, v1: List<Adt1>, v2: List<Adt1>) -> List<Adt1> {
    let m65 = v2;
    match m65.uncons() {
        None => {
            v1
        }
        Some((v3, v4)) => {
            f12(v0.clone(), {
                let c66: Fn3 = v0;
                c66.apply(v1, v3)
            }, v4)
        }
    }
}

pub fn f13(v0: List<Adt1>, v1: Adt1) -> List<Adt1> {
    List::cons(v1, v0)
}

pub fn f14(v0: List<Adt3>, v1: List<Adt1>, v2: u64, v3: bool, v4: List<Bytes>) -> Adt0 {
    let m67 = f17(v0.clone());
    match m67 {
        None => {
            let m79 = v0;
            match m79.uncons() {
                None => {
                    Adt0::C0(v1, v2, v3, None::<Bytes>)
                }
                Some((v8, v9)) => {
                    let m80 = v9;
                    match m80.uncons() {
                        None => {
                            let v10: bool = f19(match v8.clone() {
                                Adt3::C0(h81, _) => {
                                    h81
                                }
                            });
                            let v11: bool = {
                                let a83 = match v8.clone() {
                                    Adt3::C0(_, h82) => {
                                        h82
                                    }
                                };
                                if bool_not(a83) {
                                    true
                                } else {
                                    let a84 = 0u64;
                                    let a85 = v2;
                                    nat_lt(a84, a85)
                                }
                            };
                            if if v10 {
                                v11
                            } else {
                                false
                            } {
                                let v12: Bytes = {
                                    let a91 = {
                                        let a88 = {
                                            let a86 = Str::lit("the forbidden token `");
                                            utf8_encode(a86)
                                        };
                                        let a89 = match v8 {
                                            Adt3::C0(h87, _) => {
                                                h87
                                            }
                                        };
                                        append_bytes(a88, a89)
                                    };
                                    let a92 = {
                                        let a90 = Str::lit("`");
                                        utf8_encode(a90)
                                    };
                                    append_bytes(a91, a92)
                                };
                                Adt0::C0(v1, v2, v3, Some(v12))
                            } else {
                                if match v8.clone() {
                                    Adt3::C0(_, h93) => {
                                        h93
                                    }
                                } {
                                    Adt0::C0(v1, v2, v3, None::<Bytes>)
                                } else {
                                    let a96 = match v8.clone() {
                                        Adt3::C0(h94, _) => {
                                            h94
                                        }
                                    };
                                    let a97 = {
                                        let a95 = Str::lit("attribute");
                                        utf8_encode(a95)
                                    };
                                    if if equal(a96, a97) {
                                        f15(v1.clone())
                                    } else {
                                        false
                                    } {
                                        Adt0::C0(v1, v2, true, None::<Bytes>)
                                    } else {
                                        let a100 = match v8.clone() {
                                            Adt3::C0(h98, _) => {
                                                h98
                                            }
                                        };
                                        let a101 = {
                                            let a99 = Str::lit("set_option");
                                            utf8_encode(a99)
                                        };
                                        if equal(a100, a101) {
                                            let v13: Bytes = f20(v1.clone());
                                            let v14: bool = f21(v13.clone());
                                            if v14 {
                                                Adt0::C0(v1, v2, v3, None::<Bytes>)
                                            } else {
                                                let v15: Bytes = {
                                                    let a106 = {
                                                        let a103 = {
                                                            let a102 = Str::lit("the option `");
                                                            utf8_encode(a102)
                                                        };
                                                        let a104 = v13;
                                                        append_bytes(a103, a104)
                                                    };
                                                    let a107 = {
                                                        let a105 = Str::lit("` is not allowed");
                                                        utf8_encode(a105)
                                                    };
                                                    append_bytes(a106, a107)
                                                };
                                                Adt0::C0(v1, v2, v3, Some(v15))
                                            }
                                        } else {
                                            let a110 = match v8 {
                                                Adt3::C0(h108, _) => {
                                                    h108
                                                }
                                            };
                                            let a111 = {
                                                let a109 = Str::lit("import");
                                                utf8_encode(a109)
                                            };
                                            if equal(a110, a111) {
                                                let v16: Bytes = f20(v1.clone());
                                                let v17: bool = f22(v16.clone(), v4);
                                                if v17 {
                                                    Adt0::C0(v1, v2, v3, None::<Bytes>)
                                                } else {
                                                    let v18: Bytes = {
                                                        let a116 = {
                                                            let a113 = {
                                                                let a112 = Str::lit("the import of `");
                                                                utf8_encode(a112)
                                                            };
                                                            let a114 = v16;
                                                            append_bytes(a113, a114)
                                                        };
                                                        let a117 = {
                                                            let a115 = Str::lit("` is not allowed");
                                                            utf8_encode(a115)
                                                        };
                                                        append_bytes(a116, a117)
                                                    };
                                                    Adt0::C0(v1, v2, v3, Some(v18))
                                                }
                                            } else {
                                                Adt0::C0(v1, v2, v3, None::<Bytes>)
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        Some((_, _)) => {
                            Adt0::C0(v1, v2, v3, None::<Bytes>)
                        }
                    }
                }
            }
        }
        Some(v5) => {
            let v6: Bytes = f18(v0);
            let v7: Bytes = {
                let a77 = {
                    let a74 = {
                        let a72 = {
                            let a69 = {
                                let a68 = Str::lit("the forbidden constant `");
                                utf8_encode(a68)
                            };
                            let a70 = v5;
                            append_bytes(a69, a70)
                        };
                        let a73 = {
                            let a71 = Str::lit("` (in `");
                            utf8_encode(a71)
                        };
                        append_bytes(a72, a73)
                    };
                    let a75 = v6;
                    append_bytes(a74, a75)
                };
                let a78 = {
                    let a76 = Str::lit("`)");
                    utf8_encode(a76)
                };
                append_bytes(a77, a78)
            };
            Adt0::C0(v1, v2, v3, Some(v7))
        }
    }
}

pub fn f15(v0: List<Adt1>) -> bool {
    let m118 = v0;
    match m118.uncons() {
        None => {
            false
        }
        Some((v1, _)) => {
            let m120 = match v1 {
                Adt1::C0(h119, _, _, _) => {
                    h119
                }
            };
            match m120 {
                Adt2::C0(_) => {
                    false
                }
                Adt2::C1 => {
                    false
                }
                Adt2::C2 => {
                    false
                }
                Adt2::C3(_) => {
                    false
                }
                Adt2::C4(v5) => {
                    let a122 = v5;
                    let a123 = {
                        let a121 = Str::lit("[");
                        utf8_encode(a121)
                    };
                    equal(a122, a123)
                }
            }
        }
    }
}

pub fn f16(v0: Bytes, v1: u64, v2: u64, v3: bool, v4: u64, v5: List<Adt1>) -> R<Option<Adt4>> {
    let v6: u8 = f23(v0.clone(), v1);
    let v7: bool = f24(v0.clone(), {
        let a124 = v1;
        let a125 = 1u64;
        nat_add(a124, a125)?
    }, 45u8);
    let v8: u64 = f25(v0.clone(), v1)?;
    let v9: Adt5 = f26(v0.clone(), {
        let a126 = v1;
        let a127 = 2u64;
        nat_add(a126, a127)?
    }, v2)?;
    let v10: Adt4 = f27(v0.clone(), v6, v1, v2, v3, v4, v5.clone())?;
    let a128 = v6;
    let a129 = 10u8;
    if equal(a128, a129) {
        Ok(Some(Adt4::C0({
            let a130 = v1;
            let a131 = 1u64;
            nat_add(a130, a131)?
        }, {
            let a132 = v2;
            let a133 = 1u64;
            nat_add(a132, a133)?
        }, true, {
            let a134 = v4;
            let a135 = 1u64;
            nat_add(a134, a135)?
        }, v5, None::<Bytes>)))
    } else {
        let a136 = v6;
        let a137 = 32u8;
        if if equal(a136, a137) {
            true
        } else {
            let a138 = v6;
            let a139 = 9u8;
            if equal(a138, a139) {
                true
            } else {
                let a140 = v6;
                let a141 = 13u8;
                equal(a140, a141)
            }
        } {
            Ok(Some(Adt4::C0({
                let a142 = v1;
                let a143 = 1u64;
                nat_add(a142, a143)?
            }, v2, v3, {
                let a144 = v4;
                let a145 = 1u64;
                nat_add(a144, a145)?
            }, v5, None::<Bytes>)))
        } else {
            let m146 = f28(v0.clone(), v1)?;
            match m146 {
                None => {
                    let a147 = v6;
                    let a148 = 45u8;
                    if if equal(a147, a148) {
                        v7
                    } else {
                        false
                    } {
                        Ok(Some(Adt4::C0(v8, v2, v3, {
                            let a149 = v4;
                            let a150 = f31(v0, v1, v8)?;
                            nat_add(a149, a150)?
                        }, v5, None::<Bytes>)))
                    } else {
                        let a151 = v6;
                        let a152 = 47u8;
                        if if equal(a151, a152) {
                            v7
                        } else {
                            false
                        } {
                            if match v9.clone() {
                                Adt5::C0(_, _, _, h153) => {
                                    h153
                                }
                            } {
                                Ok(Some(f30(v1, v2, v3, v4, v5, {
                                    let a154 = Str::lit("a comment is not closed");
                                    utf8_encode(a154)
                                })))
                            } else {
                                Ok(Some(Adt4::C0(match v9.clone() {
                                    Adt5::C0(h155, _, _, _) => {
                                        h155
                                    }
                                }, match v9.clone() {
                                    Adt5::C0(_, h156, _, _) => {
                                        h156
                                    }
                                }, v3, {
                                    let a158 = v4;
                                    let a159 = f31(v0, v1, match v9 {
                                        Adt5::C0(h157, _, _, _) => {
                                            h157
                                        }
                                    })?;
                                    nat_add(a158, a159)?
                                }, v5, None::<Bytes>)))
                            }
                        } else {
                            Ok(Some(v10))
                        }
                    }
                }
                Some(v11) => {
                    let v12: Bytes = f29(v11);
                    Ok(Some(f30(v1, v2, v3, v4, v5, v12)))
                }
            }
        }
    }
}

pub fn f17(v0: List<Adt3>) -> Option<Bytes> {
    f32(Fn4::F33, None::<Bytes>, v0)
}

pub fn f18(v0: List<Adt3>) -> Bytes {
    let v1: List<Bytes> = f34(f35(Fn5::F36, List::<Bytes>::nil(), v0));
    f37(v1, {
        let a160 = Str::lit(".");
        utf8_encode(a160)
    })
}

pub fn f19(v0: Bytes) -> bool {
    let v1: List<Bytes> = List::cons({
        let a161 = Str::lit("sorry");
        utf8_encode(a161)
    }, List::cons({
        let a162 = Str::lit("admit");
        utf8_encode(a162)
    }, List::cons({
        let a163 = Str::lit("axiom");
        utf8_encode(a163)
    }, List::cons({
        let a164 = Str::lit("opaque");
        utf8_encode(a164)
    }, List::cons({
        let a165 = Str::lit("unsafe");
        utf8_encode(a165)
    }, List::cons({
        let a166 = Str::lit("partial");
        utf8_encode(a166)
    }, List::cons({
        let a167 = Str::lit("native_decide");
        utf8_encode(a167)
    }, List::cons({
        let a168 = Str::lit("kernel");
        utf8_encode(a168)
    }, List::cons({
        let a169 = Str::lit("ofReduceBool");
        utf8_encode(a169)
    }, List::cons({
        let a170 = Str::lit("ofReduceNat");
        utf8_encode(a170)
    }, List::cons({
        let a171 = Str::lit("implemented_by");
        utf8_encode(a171)
    }, List::cons({
        let a172 = Str::lit("extern");
        utf8_encode(a172)
    }, List::cons({
        let a173 = Str::lit("macro");
        utf8_encode(a173)
    }, List::cons({
        let a174 = Str::lit("macro_rules");
        utf8_encode(a174)
    }, List::cons({
        let a175 = Str::lit("syntax");
        utf8_encode(a175)
    }, List::cons({
        let a176 = Str::lit("elab");
        utf8_encode(a176)
    }, List::cons({
        let a177 = Str::lit("elab_rules");
        utf8_encode(a177)
    }, List::cons({
        let a178 = Str::lit("notation");
        utf8_encode(a178)
    }, List::cons({
        let a179 = Str::lit("infix");
        utf8_encode(a179)
    }, List::cons({
        let a180 = Str::lit("infixl");
        utf8_encode(a180)
    }, List::cons({
        let a181 = Str::lit("infixr");
        utf8_encode(a181)
    }, List::cons({
        let a182 = Str::lit("prefix");
        utf8_encode(a182)
    }, List::cons({
        let a183 = Str::lit("postfix");
        utf8_encode(a183)
    }, List::cons({
        let a184 = Str::lit("#eval");
        utf8_encode(a184)
    }, List::cons({
        let a185 = Str::lit("run_cmd");
        utf8_encode(a185)
    }, List::cons({
        let a186 = Str::lit("run_tac");
        utf8_encode(a186)
    }, List::cons({
        let a187 = Str::lit("sorryAx");
        utf8_encode(a187)
    }, List::<Bytes>::nil())))))))))))))))))))))))))));
    f22(v0, v1)
}

pub fn f20(v0: List<Adt1>) -> Bytes {
    let m188 = v0;
    match m188.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v1, _)) => {
            let m190 = match v1 {
                Adt1::C0(h189, _, _, _) => {
                    h189
                }
            };
            match m190 {
                Adt2::C0(v3) => {
                    f18(v3)
                }
                Adt2::C1 => {
                    Bytes::lit(&[])
                }
                Adt2::C2 => {
                    Bytes::lit(&[])
                }
                Adt2::C3(_) => {
                    Bytes::lit(&[])
                }
                Adt2::C4(_) => {
                    Bytes::lit(&[])
                }
            }
        }
    }
}

pub fn f21(v0: Bytes) -> bool {
    let v1: List<Bytes> = List::cons({
        let a191 = Str::lit("autoImplicit");
        utf8_encode(a191)
    }, List::cons({
        let a192 = Str::lit("maxRecDepth");
        utf8_encode(a192)
    }, List::cons({
        let a193 = Str::lit("maxHeartbeats");
        utf8_encode(a193)
    }, List::cons({
        let a194 = Str::lit("synthInstance.maxHeartbeats");
        utf8_encode(a194)
    }, List::cons({
        let a195 = Str::lit("linter.unusedVariables");
        utf8_encode(a195)
    }, List::cons({
        let a196 = Str::lit("linter.unusedSimpArgs");
        utf8_encode(a196)
    }, List::<Bytes>::nil()))))));
    f22(v0, v1)
}

pub fn f22(v0: Bytes, v1: List<Bytes>) -> bool {
    f38(Fn6::F39(v0), false, v1)
}

pub fn f23(v0: Bytes, v1: u64) -> u8 {
    let m197 = f40(v0, v1);
    match m197 {
        None => {
            0u8
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f24(v0: Bytes, v1: u64, v2: u8) -> bool {
    let m198 = f40(v0, v1);
    match m198 {
        None => {
            false
        }
        Some(v3) => {
            let a199 = v3;
            let a200 = v2;
            equal(a199, a200)
        }
    }
}

pub fn f25(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = {
        let a201 = v0.clone();
        length_bytes(a201)
    };
    let (h202, _) = f41(Fn7::F42(v2, v0), {
        let a203 = v2;
        let a204 = v1;
        nat_sub(a203, a204)
    }, v1)?;
    Ok(h202)
}

pub fn f26(v0: Bytes, v1: u64, v2: u64) -> R<Adt5> {
    let v3: u64 = {
        let a205 = v0.clone();
        length_bytes(a205)
    };
    let (h206, _) = f43(Fn8::F44(v3, v0), {
        let a209 = {
            let a207 = v3;
            let a208 = v1;
            nat_sub(a207, a208)
        };
        let a210 = 1u64;
        nat_add(a209, a210)?
    }, Adt5::C0(v1, v2, 1u64, false))?;
    Ok(h206)
}

pub fn f27(v0: Bytes, v1: u8, v2: u64, v3: u64, v4: bool, v5: u64, v6: List<Adt1>) -> R<Adt4> {
    let v7: Adt6 = f45(v0.clone(), {
        let a211 = v2;
        let a212 = 1u64;
        nat_add(a211, a212)?
    }, v3)?;
    let v8: Adt4 = if match v7.clone() {
        Adt6::C0(_, _, _, h213) => {
            h213
        }
    } {
        f30(v2, v3, false, v5, v6.clone(), {
            let a214 = Str::lit("a string is not closed");
            utf8_encode(a214)
        })
    } else {
        f46(match v7.clone() {
            Adt6::C0(h215, _, _, _) => {
                h215
            }
        }, match v7.clone() {
            Adt6::C0(_, h216, _, _) => {
                h216
            }
        }, v5, f31(v0.clone(), v2, match v7 {
            Adt6::C0(h217, _, _, _) => {
                h217
            }
        })?, Adt2::C2, v3, v4, v6.clone())?
    };
    let v10: Adt4 = {
        let m218 = f47(v0.clone(), v2)?;
        match m218 {
            None => {
                f30(v2, v3, false, v5, v6.clone(), {
                    let a219 = Str::lit("a character literal is not closed");
                    utf8_encode(a219)
                })
            }
            Some(v9) => {
                f46(v9, v3, v5, f31(v0.clone(), v2, v9)?, Adt2::C2, v3, v4, v6.clone())?
            }
        }
    };
    let v13: Adt4 = {
        let m220 = f48(v0.clone(), v2)?;
        match m220 {
            Ok(v12) => {
                f46(v12, v3, v5, f31(v0.clone(), v2, v12)?, Adt2::C1, v3, v4, v6.clone())?
            }
            Err(v11) => {
                f30(v2, v3, false, v5, v6.clone(), v11)
            }
        }
    };
    let v14: u64 = f49(v0.clone(), {
        let a221 = v2;
        let a222 = 1u64;
        nat_add(a221, a222)?
    })?;
    let v15: u64 = f50(v0.clone(), {
        let a225 = {
            let a223 = v2;
            let a224 = 1u64;
            nat_add(a223, a224)?
        };
        let a226 = v14;
        nat_add(a225, a226)?
    })?;
    let v16: Adt4 = f46(v15, v3, v5, f31(v0.clone(), v2, v15)?, Adt2::C3(f51(v0.clone(), v2, {
        let a227 = v15;
        let a228 = v2;
        nat_sub(a227, a228)
    })), v3, v4, v6.clone())?;
    let v17: bool = f52(v0.clone(), v2)?;
    let v18: Adt4 = f53(v0.clone(), v2, v3, v4, v5, v6.clone())?;
    let v19: u64 = f54(v0.clone(), v2);
    let v20: Adt4 = f46({
        let a229 = v2;
        let a230 = v19;
        nat_add(a229, a230)?
    }, v3, v5, 1u64, Adt2::C4(f51(v0, v2, v19)), v3, v4, v6)?;
    let v21: bool = f55(v1);
    let a231 = v1;
    let a232 = 34u8;
    if equal(a231, a232) {
        Ok(v8)
    } else {
        let a233 = v1;
        let a234 = 39u8;
        if equal(a233, a234) {
            Ok(v10)
        } else {
            if v21 {
                Ok(v13)
            } else {
                let a235 = v1;
                let a236 = 35u8;
                if if equal(a235, a236) {
                    let a237 = 0u64;
                    let a238 = v14;
                    nat_lt(a237, a238)
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

pub fn f28(v0: Bytes, v1: u64) -> R<Option<u8>> {
    let a240 = v1;
    let a241 = {
        let a239 = v0.clone();
        length_bytes(a239)
    };
    if nat_lt(a240, a241) {
        let v2: u8 = f23(v0.clone(), v1);
        if f56(v2, 0u8, 31u8) {
            Ok(Some(v2))
        } else {
            let a242 = v2;
            let a243 = 127u8;
            if equal(a242, a243) {
                Ok(Some(v2))
            } else {
                let a244 = v2;
                let a245 = 194u8;
                if equal(a244, a245) {
                    if f57(v0.clone(), {
                        let a246 = v1;
                        let a247 = 1u64;
                        nat_add(a246, a247)?
                    }, 128u8, 159u8) {
                        Ok(Some(f23(v0, {
                            let a248 = v1;
                            let a249 = 1u64;
                            nat_add(a248, a249)?
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

pub fn f29(v0: u8) -> Bytes {
    let v2: u8 = {
        let m252 = {
            let a250 = v0;
            let a251 = 4u32;
            fixed_u8::shift_right(a250, a251)
        };
        match m252 {
            None => {
                0u8
            }
            Some(v1) => {
                v1
            }
        }
    };
    let v3: u8 = {
        let a253 = v0;
        let a254 = 15u8;
        fixed_u8::bit_and(a253, a254)
    };
    let v4: Bytes = f58(v2);
    let v5: Bytes = f58(v3);
    let a261 = {
        let a258 = {
            let a255 = Str::lit("a control character U+00");
            utf8_encode(a255)
        };
        let a259 = {
            let a256 = v4;
            let a257 = v5;
            append_bytes(a256, a257)
        };
        append_bytes(a258, a259)
    };
    let a262 = {
        let a260 = Str::lit(" in the text");
        utf8_encode(a260)
    };
    append_bytes(a261, a262)
}

pub fn f30(v0: u64, v1: u64, v2: bool, v3: u64, v4: List<Adt1>, v5: Bytes) -> Adt4 {
    Adt4::C0(v0, v1, v2, v3, v4, Some(v5))
}

pub fn f31(v0: Bytes, v1: u64, v2: u64) -> R<u64> {
    let (_, h263) = {
        let (h264, _) = f59(Fn9::F60(v0, v2), {
            let a265 = v2;
            let a266 = v1;
            nat_sub(a265, a266)
        }, (v1, 0u64))?;
        h264
    };
    Ok(h263)
}

pub fn f32(v0: Fn4, v1: Option<Bytes>, v2: List<Adt3>) -> Option<Bytes> {
    let m267 = v2;
    match m267.uncons() {
        None => {
            v1
        }
        Some((v3, v4)) => {
            f32(v0.clone(), {
                let c268: Fn4 = v0;
                c268.apply(v1, v3)
            }, v4)
        }
    }
}

pub fn f33(v0: Option<Bytes>, v1: Adt3) -> Option<Bytes> {
    let m269 = v0;
    match m269 {
        None => {
            let v3: bool = f61(match v1.clone() {
                Adt3::C0(h270, _) => {
                    h270
                }
            });
            if v3 {
                Some(match v1 {
                    Adt3::C0(h271, _) => {
                        h271
                    }
                })
            } else {
                None::<Bytes>
            }
        }
        Some(v2) => {
            Some(v2)
        }
    }
}

pub fn f34(v0: List<Bytes>) -> List<Bytes> {
    f62(Fn10::F63, List::<Bytes>::nil(), v0)
}

pub fn f35(v0: Fn5, v1: List<Bytes>, v2: List<Adt3>) -> List<Bytes> {
    let m272 = v2;
    match m272.uncons() {
        None => {
            v1
        }
        Some((v3, v4)) => {
            f35(v0.clone(), {
                let c273: Fn5 = v0;
                c273.apply(v1, v3)
            }, v4)
        }
    }
}

pub fn f36(v0: List<Bytes>, v1: Adt3) -> List<Bytes> {
    List::cons(match v1 {
        Adt3::C0(h274, _) => {
            h274
        }
    }, v0)
}

pub fn f37(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m275 = f64(Fn11::F65(v1), None::<Bytes>, v0);
    match m275 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f38(v0: Fn6, v1: bool, v2: List<Bytes>) -> bool {
    let m276 = v2;
    match m276.uncons() {
        None => {
            v1
        }
        Some((v3, v4)) => {
            f38(v0.clone(), {
                let c277: Fn6 = v0;
                c277.apply(v1, v3)
            }, v4)
        }
    }
}

pub fn f39(v0: Bytes, v1: bool, v2: Bytes) -> bool {
    if v1 {
        true
    } else {
        let a278 = v2;
        let a279 = v0;
        equal(a278, a279)
    }
}

pub fn f40(v0: Bytes, v1: u64) -> Option<u8> {
    let a280 = v0;
    let a281 = v1;
    index_bytes(a280, a281)
}

pub fn f41(v0: Fn7, v1: u64, v2: u64) -> R<(u64, bool)> {
    let m282 = v1;
    if m282 == 0 {
        Ok((v2, false))
    } else {
        let v3 = m282 - 1;
        let m284 = {
            let c283: Fn7 = v0.clone();
            c283.apply(v2)?
        };
        match m284 {
            None => {
                Ok((v2, true))
            }
            Some(v4) => {
                f41(v0, v3, v4)
            }
        }
    }
}

pub fn f42(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f66(v1, v0, v2)
}

pub fn f43(v0: Fn8, v1: u64, v2: Adt5) -> R<(Adt5, bool)> {
    let m285 = v1;
    if m285 == 0 {
        Ok((v2, false))
    } else {
        let v3 = m285 - 1;
        let m287 = {
            let c286: Fn8 = v0.clone();
            c286.apply(v2.clone())?
        };
        match m287 {
            None => {
                Ok((v2, true))
            }
            Some(v4) => {
                f43(v0, v3, v4)
            }
        }
    }
}

pub fn f44(v0: u64, v1: Bytes, v2: Adt5) -> R<Option<Adt5>> {
    f67(v1, v0, v2)
}

pub fn f45(v0: Bytes, v1: u64, v2: u64) -> R<Adt6> {
    let v3: u64 = {
        let a288 = v0.clone();
        length_bytes(a288)
    };
    let (h289, _) = f68(Fn12::F69(v3, v0), {
        let a292 = {
            let a290 = v3;
            let a291 = v1;
            nat_sub(a290, a291)
        };
        let a293 = 1u64;
        nat_add(a292, a293)?
    }, Adt6::C0(v1, v2, false, false))?;
    Ok(h289)
}

pub fn f46(v0: u64, v1: u64, v2: u64, v3: u64, v4: Adt2, v5: u64, v6: bool, v7: List<Adt1>) -> R<Adt4> {
    Ok(Adt4::C0(v0, v1, false, {
        let a294 = v2;
        let a295 = v3;
        nat_add(a294, a295)?
    }, List::cons(Adt1::C0(v4, v5, v6, v2), v7), None::<Bytes>))
}

pub fn f47(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = {
        let a296 = v0.clone();
        length_bytes(a296)
    };
    let v3: u64 = {
        let a297 = v1;
        let a298 = 1u64;
        nat_add(a297, a298)?
    };
    let v4: u8 = f23(v0.clone(), v3);
    let v5: u64 = {
        let a299 = v3;
        let a300 = 1u64;
        nat_add(a299, a300)?
    };
    let v6: u8 = f23(v0.clone(), v5);
    let v7: u64 = {
        let a301 = v5;
        let a302 = f54(v0.clone(), v5);
        nat_add(a301, a302)?
    };
    let v8: Option<u64> = f70(v0.clone(), v7, 2u64)?;
    let v9: Option<u64> = f70(v0.clone(), v7, 4u64)?;
    let v10: u64 = f54(v0.clone(), v3);
    let v11: Option<u64> = {
        let a303 = v3;
        let a304 = v2;
        if nat_lt(a303, a304) {
            let a305 = v4;
            let a306 = 92u8;
            if equal(a305, a306) {
                let a307 = v5;
                let a308 = v2;
                if nat_lt(a307, a308) {
                    let a309 = v6;
                    let a310 = 120u8;
                    if equal(a309, a310) {
                        v8
                    } else {
                        let a311 = v6;
                        let a312 = 117u8;
                        if equal(a311, a312) {
                            v9
                        } else {
                            Some(v7)
                        }
                    }
                } else {
                    None::<u64>
                }
            } else {
                let a313 = v4;
                let a314 = 39u8;
                if equal(a313, a314) {
                    None::<u64>
                } else {
                    let a315 = v4;
                    let a316 = 10u8;
                    if equal(a315, a316) {
                        None::<u64>
                    } else {
                        Some({
                            let a317 = v3;
                            let a318 = v10;
                            nat_add(a317, a318)?
                        })
                    }
                }
            }
        } else {
            None::<u64>
        }
    };
    let m319 = v11;
    match m319 {
        None => {
            Ok(None::<u64>)
        }
        Some(v12) => {
            let v13: bool = f24(v0, v12, 39u8);
            if v13 {
                Ok(Some({
                    let a320 = v12;
                    let a321 = 1u64;
                    nat_add(a320, a321)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f48(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let v2: u8 = f23(v0.clone(), v1);
    let v3: u8 = f23(v0.clone(), {
        let a322 = v1;
        let a323 = 1u64;
        nat_add(a322, a323)?
    });
    let v4: u64 = {
        let a324 = v2;
        let a325 = 48u8;
        if equal(a324, a325) {
            let a326 = v3;
            let a327 = 120u8;
            if equal(a326, a327) {
                16u64
            } else {
                let a328 = v3;
                let a329 = 88u8;
                if equal(a328, a329) {
                    16u64
                } else {
                    let a330 = v3;
                    let a331 = 98u8;
                    if equal(a330, a331) {
                        2u64
                    } else {
                        let a332 = v3;
                        let a333 = 66u8;
                        if equal(a332, a333) {
                            2u64
                        } else {
                            let a334 = v3;
                            let a335 = 111u8;
                            if equal(a334, a335) {
                                8u64
                            } else {
                                let a336 = v3;
                                let a337 = 79u8;
                                if equal(a336, a337) {
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
    let v5: Result<u64, Bytes> = f71(v0.clone(), {
        let a338 = v1;
        let a339 = 2u64;
        nat_add(a338, a339)?
    }, v4, true)?;
    let v6: u64 = f72(v0.clone(), v1)?;
    let v9: Result<u64, Bytes> = {
        let m342 = f71(v0.clone(), {
            let a340 = v1;
            let a341 = 1u64;
            nat_add(a340, a341)?
        }, 10u64, false)?;
        match m342 {
            Ok(v8) => {
                f73(v0.clone(), v8)?
            }
            Err(v7) => {
                Err::<u64, Bytes>(v7)
            }
        }
    };
    let a345 = {
        let a343 = v4;
        let a344 = 10u64;
        nat_eq(a343, a344)
    };
    if bool_not(a345) {
        Ok(v5)
    } else {
        let a348 = {
            let a346 = v2;
            let a347 = 48u8;
            equal(a346, a347)
        };
        if if bool_not(a348) {
            let a349 = 0u64;
            let a350 = v1;
            if nat_lt(a349, a350) {
                f24(v0.clone(), {
                    let a351 = v1;
                    let a352 = 1u64;
                    nat_sub(a351, a352)
                }, 46u8)
            } else {
                false
            }
        } else {
            false
        } {
            if if f24(v0.clone(), v6, 95u8) {
                true
            } else {
                if f24(v0.clone(), v6, 101u8) {
                    true
                } else {
                    f24(v0, v6, 69u8)
                }
            } {
                Ok(Err::<u64, Bytes>({
                    let a353 = Str::lit("a field index is followed by what could continue a number");
                    utf8_encode(a353)
                }))
            } else {
                Ok(Ok::<u64, Bytes>(v6))
            }
        } else {
            Ok(v9)
        }
    }
}

pub fn f49(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u8 = f23(v0.clone(), v1);
    let v3: bool = f74(v2);
    let v4: bool = {
        let a354 = v2;
        let a355 = 95u8;
        equal(a354, a355)
    };
    let v5: u64 = f75(v0.clone(), v1)?;
    let a357 = v1;
    let a358 = {
        let a356 = v0;
        length_bytes(a356)
    };
    if nat_lt(a357, a358) {
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

pub fn f50(v0: Bytes, v1: u64) -> R<u64> {
    let (h359, _) = f41(Fn7::F76(v0.clone()), {
        let a361 = {
            let a360 = v0;
            length_bytes(a360)
        };
        let a362 = v1;
        nat_sub(a361, a362)
    }, v1)?;
    Ok(h359)
}

pub fn f51(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m366 = {
        let a363 = v0;
        let a364 = v1;
        let a365 = v2;
        slice_bytes(a363, a364, a365)
    };
    match m366 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f52(v0: Bytes, v1: u64) -> R<bool> {
    let v2: u64 = f49(v0.clone(), v1)?;
    let v3: bool = f77(v0, v1)?;
    let a367 = 0u64;
    let a368 = v2;
    if nat_lt(a367, a368) {
        Ok(true)
    } else {
        Ok(v3)
    }
}

pub fn f53(v0: Bytes, v1: u64, v2: u64, v3: bool, v4: u64, v5: List<Adt1>) -> R<Adt4> {
    let m369 = f78(v0.clone(), v1)?;
    match m369 {
        Ok(v7) => {
            let m371 = {
                let (h370, _) = v7.clone();
                h370
            };
            match m371.uncons() {
                None => {
                    Ok(Adt4::C0({
                        let (_, h372) = v7;
                        h372
                    }, v2, false, v4, v5, None::<Bytes>))
                }
                Some((v8, v9)) => {
                    let m373 = v9;
                    match m373.uncons() {
                        None => {
                            let v10: u64 = f79(v0.clone(), {
                                let (_, h374) = v7.clone();
                                h374
                            })?;
                            let v11: Adt6 = f80(v0.clone(), {
                                let a378 = {
                                    let a376 = {
                                        let (_, h375) = v7.clone();
                                        h375
                                    };
                                    let a377 = v10;
                                    nat_add(a376, a377)?
                                };
                                let a379 = 1u64;
                                nat_add(a378, a379)?
                            }, v2, v10)?;
                            let v12: Adt4 = if match v11.clone() {
                                Adt6::C0(_, _, _, h380) => {
                                    h380
                                }
                            } {
                                f30(v1, v2, false, v4, v5.clone(), {
                                    let a381 = Str::lit("a raw string is not closed");
                                    utf8_encode(a381)
                                })
                            } else {
                                f46(match v11.clone() {
                                    Adt6::C0(h382, _, _, _) => {
                                        h382
                                    }
                                }, match v11.clone() {
                                    Adt6::C0(_, h383, _, _) => {
                                        h383
                                    }
                                }, v4, f31(v0.clone(), v1, match v11 {
                                    Adt6::C0(h384, _, _, _) => {
                                        h384
                                    }
                                })?, Adt2::C2, v2, v3, v5.clone())?
                            };
                            let v13: Adt4 = f30(v1, v2, false, v4, v5.clone(), {
                                let a385 = Str::lit("an interpolated string holds code");
                                utf8_encode(a385)
                            });
                            let v14: Adt4 = f46({
                                let (_, h386) = v7.clone();
                                h386
                            }, v2, v4, f31(v0.clone(), v1, {
                                let (_, h387) = v7.clone();
                                h387
                            })?, Adt2::C0({
                                let (h388, _) = v7.clone();
                                h388
                            }), v2, v3, v5)?;
                            let v15: bool = f81(match v8.clone() {
                                Adt3::C0(h389, _) => {
                                    h389
                                }
                            }, {
                                let a390 = Str::lit("!");
                                utf8_encode(a390)
                            });
                            let v16: bool = f24(v0.clone(), {
                                let (_, h391) = v7.clone();
                                h391
                            }, 34u8);
                            let v17: bool = f24(v0, {
                                let a393 = {
                                    let (_, h392) = v7;
                                    h392
                                };
                                let a394 = v10;
                                nat_add(a393, a394)?
                            }, 34u8);
                            let a396 = match v8.clone() {
                                Adt3::C0(_, h395) => {
                                    h395
                                }
                            };
                            if if bool_not(a396) {
                                let a399 = match v8.clone() {
                                    Adt3::C0(h397, _) => {
                                        h397
                                    }
                                };
                                let a400 = {
                                    let a398 = Str::lit("r");
                                    utf8_encode(a398)
                                };
                                if equal(a399, a400) {
                                    v17
                                } else {
                                    false
                                }
                            } else {
                                false
                            } {
                                Ok(v12)
                            } else {
                                let a402 = match v8 {
                                    Adt3::C0(_, h401) => {
                                        h401
                                    }
                                };
                                if if bool_not(a402) {
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
                            f46({
                                let (_, h403) = v7.clone();
                                h403
                            }, v2, v4, f31(v0, v1, {
                                let (_, h404) = v7;
                                h404
                            })?, Adt2::C0({
                                let (h405, _) = v7;
                                h405
                            }), v2, v3, v5)
                        }
                    }
                }
            }
        }
        Err(v6) => {
            Ok(f30(v1, v2, false, v4, v5, v6))
        }
    }
}

pub fn f54(v0: Bytes, v1: u64) -> u64 {
    let v2: u8 = f23(v0, v1);
    let v3: bool = f56(v2, 0u8, 127u8);
    if v3 {
        1u64
    } else {
        let v4: bool = f56(v2, 128u8, 223u8);
        if v4 {
            2u64
        } else {
            let v5: bool = f56(v2, 224u8, 239u8);
            if v5 {
                3u64
            } else {
                4u64
            }
        }
    }
}

pub fn f55(v0: u8) -> bool {
    f56(v0, 48u8, 57u8)
}

pub fn f56(v0: u8, v1: u8, v2: u8) -> bool {
    let m408 = {
        let a406 = v0;
        let a407 = v1;
        fixed_u8::checked_sub(a406, a407)
    };
    match m408 {
        None => {
            false
        }
        Some(_) => {
            let m411 = {
                let a409 = v2;
                let a410 = v0;
                fixed_u8::checked_sub(a409, a410)
            };
            match m411 {
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

pub fn f57(v0: Bytes, v1: u64, v2: u8, v3: u8) -> bool {
    let m412 = f40(v0, v1);
    match m412 {
        None => {
            false
        }
        Some(v4) => {
            f56(v4, v2, v3)
        }
    }
}

pub fn f58(v0: u8) -> Bytes {
    let v1: Bytes = {
        let a413 = Str::lit("0123456789ABCDEF");
        utf8_encode(a413)
    };
    let v2: u64 = {
        let a414 = v0;
        let a415 = 0u8;
        if equal(a414, a415) {
            0u64
        } else {
            let a416 = v0;
            let a417 = 1u8;
            if equal(a416, a417) {
                1u64
            } else {
                let a418 = v0;
                let a419 = 2u8;
                if equal(a418, a419) {
                    2u64
                } else {
                    let a420 = v0;
                    let a421 = 3u8;
                    if equal(a420, a421) {
                        3u64
                    } else {
                        let a422 = v0;
                        let a423 = 4u8;
                        if equal(a422, a423) {
                            4u64
                        } else {
                            let a424 = v0;
                            let a425 = 5u8;
                            if equal(a424, a425) {
                                5u64
                            } else {
                                let a426 = v0;
                                let a427 = 6u8;
                                if equal(a426, a427) {
                                    6u64
                                } else {
                                    let a428 = v0;
                                    let a429 = 7u8;
                                    if equal(a428, a429) {
                                        7u64
                                    } else {
                                        let a430 = v0;
                                        let a431 = 8u8;
                                        if equal(a430, a431) {
                                            8u64
                                        } else {
                                            let a432 = v0;
                                            let a433 = 9u8;
                                            if equal(a432, a433) {
                                                9u64
                                            } else {
                                                let a434 = v0;
                                                let a435 = 10u8;
                                                if equal(a434, a435) {
                                                    10u64
                                                } else {
                                                    let a436 = v0;
                                                    let a437 = 11u8;
                                                    if equal(a436, a437) {
                                                        11u64
                                                    } else {
                                                        let a438 = v0;
                                                        let a439 = 12u8;
                                                        if equal(a438, a439) {
                                                            12u64
                                                        } else {
                                                            let a440 = v0;
                                                            let a441 = 13u8;
                                                            if equal(a440, a441) {
                                                                13u64
                                                            } else {
                                                                let a442 = v0;
                                                                let a443 = 14u8;
                                                                if equal(a442, a443) {
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
    f51(v1, v2, 1u64)
}

pub fn f59(v0: Fn9, v1: u64, v2: (u64, u64)) -> R<((u64, u64), bool)> {
    let m444 = v1;
    if m444 == 0 {
        Ok((v2, false))
    } else {
        let v3 = m444 - 1;
        let m446 = {
            let c445: Fn9 = v0.clone();
            c445.apply(v2)?
        };
        match m446 {
            None => {
                Ok((v2, true))
            }
            Some(v4) => {
                f59(v0, v3, v4)
            }
        }
    }
}

pub fn f60(v0: Bytes, v1: u64, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    f82(v0, v1, v2)
}

pub fn f61(v0: Bytes) -> bool {
    let a448 = v0.clone();
    let a449 = {
        let a447 = Str::lit("ofReduceBool");
        utf8_encode(a447)
    };
    if equal(a448, a449) {
        true
    } else {
        let a451 = v0.clone();
        let a452 = {
            let a450 = Str::lit("ofReduceNat");
            utf8_encode(a450)
        };
        if equal(a451, a452) {
            true
        } else {
            let a454 = v0;
            let a455 = {
                let a453 = Str::lit("sorryAx");
                utf8_encode(a453)
            };
            equal(a454, a455)
        }
    }
}

pub fn f62(v0: Fn10, v1: List<Bytes>, v2: List<Bytes>) -> List<Bytes> {
    let m456 = v2;
    match m456.uncons() {
        None => {
            v1
        }
        Some((v3, v4)) => {
            f62(v0.clone(), {
                let c457: Fn10 = v0;
                c457.apply(v1, v3)
            }, v4)
        }
    }
}

pub fn f63(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f64(v0: Fn11, v1: Option<Bytes>, v2: List<Bytes>) -> Option<Bytes> {
    let m458 = v2;
    match m458.uncons() {
        None => {
            v1
        }
        Some((v3, v4)) => {
            f64(v0.clone(), {
                let c459: Fn11 = v0;
                c459.apply(v1, v3)
            }, v4)
        }
    }
}

pub fn f65(v0: Bytes, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m460 = v1;
    match m460 {
        None => {
            Some(v2)
        }
        Some(v3) => {
            Some({
                let a463 = {
                    let a461 = v3;
                    let a462 = v0;
                    append_bytes(a461, a462)
                };
                let a464 = v2;
                append_bytes(a463, a464)
            })
        }
    }
}

pub fn f66(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let a465 = v2;
    let a466 = v1;
    if nat_lt(a465, a466) {
        let v3: bool = f24(v0, v2, 10u8);
        if v3 {
            Ok(None::<u64>)
        } else {
            Ok(Some({
                let a467 = v2;
                let a468 = 1u64;
                nat_add(a467, a468)?
            }))
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f67(v0: Bytes, v1: u64, v2: Adt5) -> R<Option<Adt5>> {
    let v3: u64 = match v2.clone() {
        Adt5::C0(h469, _, _, _) => {
            h469
        }
    };
    let v4: u64 = match v2.clone() {
        Adt5::C0(_, h470, _, _) => {
            h470
        }
    };
    let v5: u64 = match v2.clone() {
        Adt5::C0(_, _, h471, _) => {
            h471
        }
    };
    let a472 = v5;
    let a473 = 0u64;
    if nat_eq(a472, a473) {
        Ok(None::<Adt5>)
    } else {
        if match v2 {
            Adt5::C0(_, _, _, h474) => {
                h474
            }
        } {
            Ok(None::<Adt5>)
        } else {
            let a475 = v3;
            let a476 = v1;
            if nat_lt(a475, a476) {
                let v6: u8 = f23(v0.clone(), v3);
                let a477 = v6;
                let a478 = 47u8;
                if if equal(a477, a478) {
                    f24(v0.clone(), {
                        let a479 = v3;
                        let a480 = 1u64;
                        nat_add(a479, a480)?
                    }, 45u8)
                } else {
                    false
                } {
                    Ok(Some(Adt5::C0({
                        let a481 = v3;
                        let a482 = 2u64;
                        nat_add(a481, a482)?
                    }, v4, {
                        let a483 = v5;
                        let a484 = 1u64;
                        nat_add(a483, a484)?
                    }, false)))
                } else {
                    let a485 = v6;
                    let a486 = 45u8;
                    if if equal(a485, a486) {
                        f24(v0.clone(), {
                            let a487 = v3;
                            let a488 = 1u64;
                            nat_add(a487, a488)?
                        }, 47u8)
                    } else {
                        false
                    } {
                        Ok(Some(Adt5::C0({
                            let a489 = v3;
                            let a490 = 2u64;
                            nat_add(a489, a490)?
                        }, v4, {
                            let a491 = v5;
                            let a492 = 1u64;
                            nat_sub(a491, a492)
                        }, false)))
                    } else {
                        let a493 = v6;
                        let a494 = 10u8;
                        if equal(a493, a494) {
                            Ok(Some(Adt5::C0({
                                let a495 = v3;
                                let a496 = 1u64;
                                nat_add(a495, a496)?
                            }, {
                                let a497 = v4;
                                let a498 = 1u64;
                                nat_add(a497, a498)?
                            }, v5, false)))
                        } else {
                            Ok(Some(Adt5::C0({
                                let a499 = v3;
                                let a500 = f54(v0, v3);
                                nat_add(a499, a500)?
                            }, v4, v5, false)))
                        }
                    }
                }
            } else {
                Ok(Some(Adt5::C0(v3, v4, v5, true)))
            }
        }
    }
}

pub fn f68(v0: Fn12, v1: u64, v2: Adt6) -> R<(Adt6, bool)> {
    let m501 = v1;
    if m501 == 0 {
        Ok((v2, false))
    } else {
        let v3 = m501 - 1;
        let m503 = {
            let c502: Fn12 = v0.clone();
            c502.apply(v2.clone())?
        };
        match m503 {
            None => {
                Ok((v2, true))
            }
            Some(v4) => {
                f68(v0, v3, v4)
            }
        }
    }
}

pub fn f69(v0: u64, v1: Bytes, v2: Adt6) -> R<Option<Adt6>> {
    f83(v1, v0, v2)
}

pub fn f70(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let v3: (u64, (u64, bool)) = {
        let (h504, _) = f84(Fn13::F85(v2, v0), v2, (v1, (0u64, true)))?;
        h504
    };
    let (_, h505) = {
        let (_, h506) = v3;
        h506
    };
    if h505 {
        Ok(Some({
            let (h507, _) = v3;
            h507
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f71(v0: Bytes, v1: u64, v2: u64, v3: bool) -> R<Result<u64, Bytes>> {
    let v4: u64 = {
        let a508 = v0.clone();
        length_bytes(a508)
    };
    let v5: (u64, (bool, bool)) = {
        let (h509, _) = f86(Fn14::F87(v4, v2, v0), {
            let a512 = {
                let a510 = v4;
                let a511 = v1;
                nat_sub(a510, a511)
            };
            let a513 = 1u64;
            nat_add(a512, a513)?
        }, (v1, (v3, false)))?;
        h509
    };
    let (h514, _) = {
        let (_, h515) = v5;
        h515
    };
    if h514 {
        Ok(Err::<u64, Bytes>({
            let a516 = Str::lit("a number has a digit separator or a radix prefix with no digit after it");
            utf8_encode(a516)
        }))
    } else {
        Ok(Ok::<u64, Bytes>({
            let (h517, _) = v5;
            h517
        }))
    }
}

pub fn f72(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = {
        let a518 = v0.clone();
        length_bytes(a518)
    };
    let (h519, _) = f41(Fn7::F88(v0), {
        let a520 = v2;
        let a521 = v1;
        nat_sub(a520, a521)
    }, v1)?;
    Ok(h519)
}

pub fn f73(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let v2: bool = f24(v0.clone(), v1, 46u8);
    let v3: bool = f24(v0.clone(), {
        let a522 = v1;
        let a523 = 1u64;
        nat_add(a522, a523)?
    }, 46u8);
    let v4: bool = f57(v0.clone(), {
        let a524 = v1;
        let a525 = 1u64;
        nat_add(a524, a525)?
    }, 48u8, 57u8);
    let v5: bool = f24(v0.clone(), v1, 101u8);
    let v6: bool = f24(v0.clone(), v1, 69u8);
    let v7: Result<u64, Bytes> = f89(v0.clone(), {
        let a526 = v1;
        let a527 = 1u64;
        nat_add(a526, a527)?
    }, true)?;
    let v8: Result<u64, Bytes> = f89(v0.clone(), v1, false)?;
    let v11: Result<u64, Bytes> = {
        let m530 = f71(v0.clone(), {
            let a528 = v1;
            let a529 = 1u64;
            nat_add(a528, a529)?
        }, 10u64, false)?;
        match m530 {
            Ok(v10) => {
                f89(v0, v10, false)?
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

pub fn f74(v0: u8) -> bool {
    let v1: bool = f56(v0, 97u8, 122u8);
    let v2: bool = f56(v0, 65u8, 90u8);
    if v1 {
        true
    } else {
        v2
    }
}

pub fn f75(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u8 = f23(v0.clone(), v1);
    let v3: u8 = f23(v0.clone(), {
        let a531 = v1;
        let a532 = 1u64;
        nat_add(a531, a532)?
    });
    let v4: u8 = f23(v0.clone(), {
        let a533 = v1;
        let a534 = 2u64;
        nat_add(a533, a534)?
    });
    let v5: u8 = f23(v0, {
        let a535 = v1;
        let a536 = 3u64;
        nat_add(a535, a536)?
    });
    let a537 = v2;
    let a538 = 206u8;
    if equal(a537, a538) {
        if f56(v3, 145u8, 169u8) {
            let a539 = v3;
            let a540 = 160u8;
            if equal(a539, a540) {
                Ok(0u64)
            } else {
                let a541 = v3;
                let a542 = 163u8;
                if equal(a541, a542) {
                    Ok(0u64)
                } else {
                    Ok(2u64)
                }
            }
        } else {
            if f56(v3, 177u8, 191u8) {
                let a543 = v3;
                let a544 = 187u8;
                if equal(a543, a544) {
                    Ok(0u64)
                } else {
                    Ok(2u64)
                }
            } else {
                Ok(0u64)
            }
        }
    } else {
        let a545 = v2;
        let a546 = 207u8;
        if equal(a545, a546) {
            if f56(v3, 128u8, 187u8) {
                Ok(2u64)
            } else {
                Ok(0u64)
            }
        } else {
            let a547 = v2;
            let a548 = 225u8;
            if equal(a547, a548) {
                if f56(v3, 188u8, 191u8) {
                    if f56(v4, 128u8, 191u8) {
                        let a549 = v3;
                        let a550 = 191u8;
                        if equal(a549, a550) {
                            let a551 = v4;
                            let a552 = 191u8;
                            if equal(a551, a552) {
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
                let a553 = v2;
                let a554 = 226u8;
                if equal(a553, a554) {
                    let a555 = v3;
                    let a556 = 132u8;
                    if equal(a555, a556) {
                        if f56(v4, 128u8, 191u8) {
                            Ok(3u64)
                        } else {
                            Ok(0u64)
                        }
                    } else {
                        let a557 = v3;
                        let a558 = 133u8;
                        if equal(a557, a558) {
                            if f56(v4, 128u8, 143u8) {
                                Ok(3u64)
                            } else {
                                Ok(0u64)
                            }
                        } else {
                            Ok(0u64)
                        }
                    }
                } else {
                    let a559 = v2;
                    let a560 = 240u8;
                    if equal(a559, a560) {
                        let a561 = v3;
                        let a562 = 157u8;
                        if equal(a561, a562) {
                            let a563 = v4;
                            let a564 = 146u8;
                            if equal(a563, a564) {
                                if f56(v5, 156u8, 191u8) {
                                    Ok(4u64)
                                } else {
                                    Ok(0u64)
                                }
                            } else {
                                if f56(v4, 147u8, 149u8) {
                                    if f56(v5, 128u8, 191u8) {
                                        Ok(4u64)
                                    } else {
                                        Ok(0u64)
                                    }
                                } else {
                                    let a565 = v4;
                                    let a566 = 150u8;
                                    if equal(a565, a566) {
                                        if f56(v5, 128u8, 159u8) {
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

pub fn f76(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f90(v0, v1)
}

pub fn f77(v0: Bytes, v1: u64) -> R<bool> {
    let v2: bool = f24(v0.clone(), v1, 194u8);
    let v3: bool = f24(v0, {
        let a567 = v1;
        let a568 = 1u64;
        nat_add(a567, a568)?
    }, 171u8);
    if v2 {
        Ok(v3)
    } else {
        Ok(false)
    }
}

pub fn f78(v0: Bytes, v1: u64) -> R<Result<(List<Adt3>, u64), Bytes>> {
    let v2: u64 = {
        let a569 = v0.clone();
        length_bytes(a569)
    };
    let v3: Adt7 = {
        let (h570, _) = f91(Fn15::F92(v0), {
            let a573 = {
                let a571 = v2;
                let a572 = v1;
                nat_sub(a571, a572)
            };
            let a574 = 1u64;
            nat_add(a573, a574)?
        }, Adt7::C0(v1, List::<Adt3>::nil(), false, None::<Bytes>))?;
        h570
    };
    let m576 = match v3.clone() {
        Adt7::C0(_, _, _, h575) => {
            h575
        }
    };
    match m576 {
        None => {
            let v5: List<Adt3> = f93(match v3.clone() {
                Adt7::C0(_, h577, _, _) => {
                    h577
                }
            });
            Ok(Ok::<(List<Adt3>, u64), Bytes>((v5, match v3 {
                Adt7::C0(h578, _, _, _) => {
                    h578
                }
            })))
        }
        Some(v4) => {
            Ok(Err::<(List<Adt3>, u64), Bytes>(v4))
        }
    }
}

pub fn f79(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = {
        let a579 = v0.clone();
        length_bytes(a579)
    };
    let v3: u64 = {
        let (h580, _) = f41(Fn7::F94(v0), {
            let a581 = v2;
            let a582 = v1;
            nat_sub(a581, a582)
        }, v1)?;
        h580
    };
    let a583 = v3;
    let a584 = v1;
    Ok(nat_sub(a583, a584))
}

pub fn f80(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt6> {
    let v4: u64 = {
        let a585 = v0.clone();
        length_bytes(a585)
    };
    let (h586, _) = f68(Fn12::F95(v3, v4, v0), {
        let a589 = {
            let a587 = v4;
            let a588 = v1;
            nat_sub(a587, a588)
        };
        let a590 = 1u64;
        nat_add(a589, a590)?
    }, Adt6::C0(v1, v2, false, false))?;
    Ok(h586)
}

pub fn f81(v0: Bytes, v1: Bytes) -> bool {
    let v2: u64 = {
        let a591 = v0.clone();
        length_bytes(a591)
    };
    let v3: u64 = {
        let a592 = v1.clone();
        length_bytes(a592)
    };
    let a593 = v3;
    let a594 = v2;
    if nat_le(a593, a594) {
        let m600 = {
            let a597 = v0;
            let a598 = {
                let a595 = v2;
                let a596 = v3;
                nat_sub(a595, a596)
            };
            let a599 = v3;
            slice_bytes(a597, a598, a599)
        };
        match m600 {
            None => {
                false
            }
            Some(v4) => {
                let a601 = v4;
                let a602 = v1;
                equal(a601, a602)
            }
        }
    } else {
        false
    }
}

pub fn f82(v0: Bytes, v1: u64, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v3: u64 = {
        let (h603, _) = v2;
        h603
    };
    let v4: u64 = {
        let (_, h604) = v2;
        h604
    };
    let a605 = v3;
    let a606 = v1;
    if nat_lt(a605, a606) {
        let v5: u64 = f54(v0, v3);
        Ok(Some(({
            let a607 = v3;
            let a608 = v5;
            nat_add(a607, a608)?
        }, {
            let a609 = v4;
            let a610 = 1u64;
            nat_add(a609, a610)?
        })))
    } else {
        Ok(None::<(u64, u64)>)
    }
}

pub fn f83(v0: Bytes, v1: u64, v2: Adt6) -> R<Option<Adt6>> {
    let v3: u64 = match v2.clone() {
        Adt6::C0(h611, _, _, _) => {
            h611
        }
    };
    let v4: u64 = match v2.clone() {
        Adt6::C0(_, h612, _, _) => {
            h612
        }
    };
    if match v2.clone() {
        Adt6::C0(_, _, h613, _) => {
            h613
        }
    } {
        Ok(None::<Adt6>)
    } else {
        if match v2 {
            Adt6::C0(_, _, _, h614) => {
                h614
            }
        } {
            Ok(None::<Adt6>)
        } else {
            let a615 = v3;
            let a616 = v1;
            if nat_lt(a615, a616) {
                let v5: u8 = f23(v0.clone(), v3);
                let a617 = v5;
                let a618 = 92u8;
                if equal(a617, a618) {
                    let a621 = {
                        let a619 = v3;
                        let a620 = 1u64;
                        nat_add(a619, a620)?
                    };
                    let a622 = v1;
                    if nat_lt(a621, a622) {
                        let v6: u64 = f54(v0, {
                            let a623 = v3;
                            let a624 = 1u64;
                            nat_add(a623, a624)?
                        });
                        Ok(Some(Adt6::C0({
                            let a627 = {
                                let a625 = v3;
                                let a626 = 1u64;
                                nat_add(a625, a626)?
                            };
                            let a628 = v6;
                            nat_add(a627, a628)?
                        }, v4, false, false)))
                    } else {
                        Ok(Some(Adt6::C0(v1, v4, false, true)))
                    }
                } else {
                    let a629 = v5;
                    let a630 = 34u8;
                    if equal(a629, a630) {
                        Ok(Some(Adt6::C0({
                            let a631 = v3;
                            let a632 = 1u64;
                            nat_add(a631, a632)?
                        }, v4, true, false)))
                    } else {
                        let a633 = v5;
                        let a634 = 10u8;
                        if equal(a633, a634) {
                            Ok(Some(Adt6::C0({
                                let a635 = v3;
                                let a636 = 1u64;
                                nat_add(a635, a636)?
                            }, {
                                let a637 = v4;
                                let a638 = 1u64;
                                nat_add(a637, a638)?
                            }, false, false)))
                        } else {
                            let v7: u64 = f54(v0, v3);
                            Ok(Some(Adt6::C0({
                                let a639 = v3;
                                let a640 = v7;
                                nat_add(a639, a640)?
                            }, v4, false, false)))
                        }
                    }
                }
            } else {
                Ok(Some(Adt6::C0(v3, v4, false, true)))
            }
        }
    }
}

pub fn f84(v0: Fn13, v1: u64, v2: (u64, (u64, bool))) -> R<((u64, (u64, bool)), bool)> {
    let m641 = v1;
    if m641 == 0 {
        Ok((v2, false))
    } else {
        let v3 = m641 - 1;
        let m643 = {
            let c642: Fn13 = v0.clone();
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

pub fn f85(v0: u64, v1: Bytes, v2: (u64, (u64, bool))) -> R<Option<(u64, (u64, bool))>> {
    f96(v1.clone(), {
        let a644 = v1;
        length_bytes(a644)
    }, v0, v2)
}

pub fn f86(v0: Fn14, v1: u64, v2: (u64, (bool, bool))) -> R<((u64, (bool, bool)), bool)> {
    let m645 = v1;
    if m645 == 0 {
        Ok((v2, false))
    } else {
        let v3 = m645 - 1;
        let m647 = {
            let c646: Fn14 = v0.clone();
            c646.apply(v2)?
        };
        match m647 {
            None => {
                Ok((v2, true))
            }
            Some(v4) => {
                f86(v0, v3, v4)
            }
        }
    }
}

pub fn f87(v0: u64, v1: u64, v2: Bytes, v3: (u64, (bool, bool))) -> R<Option<(u64, (bool, bool))>> {
    f97(v2, v0, v1, v3)
}

pub fn f88(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f98(v0, v1)
}

pub fn f89(v0: Bytes, v1: u64, v2: bool) -> R<Result<u64, Bytes>> {
    let v3: bool = f24(v0.clone(), v1, 101u8);
    let v4: bool = f24(v0.clone(), v1, 69u8);
    let v5: Result<u64, Bytes> = f99(v0.clone(), v1)?;
    let v6: bool = f52(v0, v1)?;
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
                let a648 = Str::lit("an identifier follows a decimal point");
                utf8_encode(a648)
            }))
        } else {
            Ok(Ok::<u64, Bytes>(v1))
        }
    }
}

pub fn f90(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f100(v0, v1)?;
    let a649 = 0u64;
    let a650 = v2;
    if nat_lt(a649, a650) {
        Ok(Some({
            let a651 = v1;
            let a652 = v2;
            nat_add(a651, a652)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f91(v0: Fn15, v1: u64, v2: Adt7) -> R<(Adt7, bool)> {
    let m653 = v1;
    if m653 == 0 {
        Ok((v2, false))
    } else {
        let v3 = m653 - 1;
        let m655 = {
            let c654: Fn15 = v0.clone();
            c654.apply(v2.clone())?
        };
        match m655 {
            None => {
                Ok((v2, true))
            }
            Some(v4) => {
                f91(v0, v3, v4)
            }
        }
    }
}

pub fn f92(v0: Bytes, v1: Adt7) -> R<Option<Adt7>> {
    f101(v0, v1)
}

pub fn f93(v0: List<Adt3>) -> List<Adt3> {
    f102(Fn16::F103, List::<Adt3>::nil(), v0)
}

pub fn f94(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f104(v0, v1)
}

pub fn f95(v0: u64, v1: u64, v2: Bytes, v3: Adt6) -> R<Option<Adt6>> {
    f105(v2, v1, v0, v3)
}

pub fn f96(v0: Bytes, v1: u64, v2: u64, v3: (u64, (u64, bool))) -> R<Option<(u64, (u64, bool))>> {
    let v4: u64 = {
        let (h656, _) = v3;
        h656
    };
    let v5: u64 = {
        let (h657, _) = {
            let (_, h658) = v3;
            h658
        };
        h657
    };
    let v6: bool = {
        let (_, h659) = {
            let (_, h660) = v3;
            h660
        };
        h659
    };
    if v6 {
        let a661 = v5;
        let a662 = v2;
        if nat_lt(a661, a662) {
            let a663 = v4;
            let a664 = v1;
            if nat_lt(a663, a664) {
                let v7: u64 = f54(v0, v4);
                Ok(Some(({
                    let a665 = v4;
                    let a666 = v7;
                    nat_add(a665, a666)?
                }, ({
                    let a667 = v5;
                    let a668 = 1u64;
                    nat_add(a667, a668)?
                }, true))))
            } else {
                Ok(Some((v4, ({
                    let a669 = v5;
                    let a670 = 1u64;
                    nat_add(a669, a670)?
                }, false))))
            }
        } else {
            Ok(None::<(u64, (u64, bool))>)
        }
    } else {
        Ok(None::<(u64, (u64, bool))>)
    }
}

pub fn f97(v0: Bytes, v1: u64, v2: u64, v3: (u64, (bool, bool))) -> R<Option<(u64, (bool, bool))>> {
    let v4: u64 = {
        let (h671, _) = v3;
        h671
    };
    let v5: bool = {
        let (h672, _) = {
            let (_, h673) = v3;
            h673
        };
        h672
    };
    let v6: bool = {
        let (_, h674) = {
            let (_, h675) = v3;
            h675
        };
        h674
    };
    if v6 {
        Ok(None::<(u64, (bool, bool))>)
    } else {
        let a676 = v4;
        let a677 = v1;
        if nat_lt(a676, a677) {
            let v7: u8 = f23(v0, v4);
            let a678 = v7;
            let a679 = 95u8;
            if equal(a678, a679) {
                Ok(Some(({
                    let a680 = v4;
                    let a681 = 1u64;
                    nat_add(a680, a681)?
                }, (true, false))))
            } else {
                let v8: bool = f106(v7, v2);
                if v8 {
                    Ok(Some(({
                        let a682 = v4;
                        let a683 = 1u64;
                        nat_add(a682, a683)?
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

pub fn f98(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: bool = f57(v0, v1, 48u8, 57u8);
    if v2 {
        Ok(Some({
            let a684 = v1;
            let a685 = 1u64;
            nat_add(a684, a685)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f99(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let v2: u64 = {
        let a686 = v1;
        let a687 = 1u64;
        nat_add(a686, a687)?
    };
    let v3: bool = f24(v0.clone(), v2, 43u8);
    let v4: bool = f24(v0.clone(), v2, 45u8);
    let v5: u64 = if v3 {
        let a688 = v2;
        let a689 = 1u64;
        nat_add(a688, a689)?
    } else {
        if v4 {
            let a690 = v2;
            let a691 = 1u64;
            nat_add(a690, a691)?
        } else {
            v2
        }
    };
    let v6: bool = f57(v0.clone(), v5, 48u8, 57u8);
    let v7: Result<u64, Bytes> = f71(v0, v5, 10u64, false)?;
    if v6 {
        Ok(v7)
    } else {
        Ok(Err::<u64, Bytes>({
            let a692 = Str::lit("a scientific literal has no exponent digits");
            utf8_encode(a692)
        }))
    }
}

pub fn f100(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u8 = f23(v0.clone(), v1);
    let v3: bool = f74(v2);
    let v4: bool = f55(v2);
    let v5: u64 = f75(v0.clone(), v1)?;
    let v6: u64 = f107(v0.clone(), v1)?;
    let a694 = v1;
    let a695 = {
        let a693 = v0;
        length_bytes(a693)
    };
    if nat_lt(a694, a695) {
        if v3 {
            Ok(1u64)
        } else {
            if v4 {
                Ok(1u64)
            } else {
                let a696 = v2;
                let a697 = 95u8;
                if equal(a696, a697) {
                    Ok(1u64)
                } else {
                    let a698 = v2;
                    let a699 = 39u8;
                    if equal(a698, a699) {
                        Ok(1u64)
                    } else {
                        let a700 = v2;
                        let a701 = 33u8;
                        if equal(a700, a701) {
                            Ok(1u64)
                        } else {
                            let a702 = v2;
                            let a703 = 63u8;
                            if equal(a702, a703) {
                                Ok(1u64)
                            } else {
                                let a704 = 0u64;
                                let a705 = v5;
                                if nat_lt(a704, a705) {
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

pub fn f101(v0: Bytes, v1: Adt7) -> R<Option<Adt7>> {
    let v2: bool = f77(v0.clone(), match v1.clone() {
        Adt7::C0(h706, _, _, _) => {
            h706
        }
    })?;
    let v3: Result<(Adt3, u64), Bytes> = f108(v0.clone(), match v1.clone() {
        Adt7::C0(h707, _, _, _) => {
            h707
        }
    })?;
    let v4: u64 = f50(v0.clone(), {
        let a710 = match v1.clone() {
            Adt7::C0(h708, _, _, _) => {
                h708
            }
        };
        let a711 = f49(v0.clone(), match v1.clone() {
            Adt7::C0(h709, _, _, _) => {
                h709
            }
        })?;
        nat_add(a710, a711)?
    })?;
    let v5: Result<(Adt3, u64), Bytes> = Ok::<(Adt3, u64), Bytes>((Adt3::C0(f51(v0.clone(), match v1.clone() {
        Adt7::C0(h712, _, _, _) => {
            h712
        }
    }, {
        let a714 = v4;
        let a715 = match v1.clone() {
            Adt7::C0(h713, _, _, _) => {
                h713
            }
        };
        nat_sub(a714, a715)
    }), false), v4));
    if match v1.clone() {
        Adt7::C0(_, _, h716, _) => {
            h716
        }
    } {
        Ok(None::<Adt7>)
    } else {
        let m717 = if v2 {
            v3
        } else {
            v5
        };
        match m717 {
            Ok(v7) => {
                let v8: bool = f24(v0.clone(), {
                    let (_, h720) = v7.clone();
                    h720
                }, 46u8);
                let v9: bool = f52(v0, {
                    let a722 = {
                        let (_, h721) = v7.clone();
                        h721
                    };
                    let a723 = 1u64;
                    nat_add(a722, a723)?
                })?;
                if if v8 {
                    v9
                } else {
                    false
                } {
                    Ok(Some(Adt7::C0({
                        let a725 = {
                            let (_, h724) = v7;
                            h724
                        };
                        let a726 = 1u64;
                        nat_add(a725, a726)?
                    }, List::cons({
                        let (h727, _) = v7;
                        h727
                    }, match v1 {
                        Adt7::C0(_, h728, _, _) => {
                            h728
                        }
                    }), false, None::<Bytes>)))
                } else {
                    Ok(Some(Adt7::C0({
                        let (_, h729) = v7;
                        h729
                    }, List::cons({
                        let (h730, _) = v7;
                        h730
                    }, match v1 {
                        Adt7::C0(_, h731, _, _) => {
                            h731
                        }
                    }), true, None::<Bytes>)))
                }
            }
            Err(v6) => {
                Ok(Some(Adt7::C0(match v1.clone() {
                    Adt7::C0(h718, _, _, _) => {
                        h718
                    }
                }, match v1 {
                    Adt7::C0(_, h719, _, _) => {
                        h719
                    }
                }, true, Some(v6))))
            }
        }
    }
}

pub fn f102(v0: Fn16, v1: List<Adt3>, v2: List<Adt3>) -> List<Adt3> {
    let m732 = v2;
    match m732.uncons() {
        None => {
            v1
        }
        Some((v3, v4)) => {
            f102(v0.clone(), {
                let c733: Fn16 = v0;
                c733.apply(v1, v3)
            }, v4)
        }
    }
}

pub fn f103(v0: List<Adt3>, v1: Adt3) -> List<Adt3> {
    List::cons(v1, v0)
}

pub fn f104(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: bool = f24(v0, v1, 35u8);
    if v2 {
        Ok(Some({
            let a734 = v1;
            let a735 = 1u64;
            nat_add(a734, a735)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f105(v0: Bytes, v1: u64, v2: u64, v3: Adt6) -> R<Option<Adt6>> {
    let v4: u64 = match v3.clone() {
        Adt6::C0(h736, _, _, _) => {
            h736
        }
    };
    let v5: u64 = match v3.clone() {
        Adt6::C0(_, h737, _, _) => {
            h737
        }
    };
    if match v3.clone() {
        Adt6::C0(_, _, h738, _) => {
            h738
        }
    } {
        Ok(None::<Adt6>)
    } else {
        if match v3 {
            Adt6::C0(_, _, _, h739) => {
                h739
            }
        } {
            Ok(None::<Adt6>)
        } else {
            let a740 = v4;
            let a741 = v1;
            if nat_lt(a740, a741) {
                let v6: u8 = f23(v0.clone(), v4);
                let a742 = v6;
                let a743 = 34u8;
                if equal(a742, a743) {
                    let v7: bool = f109(v0, {
                        let a744 = v4;
                        let a745 = 1u64;
                        nat_add(a744, a745)?
                    }, v2)?;
                    if v7 {
                        Ok(Some(Adt6::C0({
                            let a748 = {
                                let a746 = v4;
                                let a747 = 1u64;
                                nat_add(a746, a747)?
                            };
                            let a749 = v2;
                            nat_add(a748, a749)?
                        }, v5, true, false)))
                    } else {
                        Ok(Some(Adt6::C0({
                            let a750 = v4;
                            let a751 = 1u64;
                            nat_add(a750, a751)?
                        }, v5, false, false)))
                    }
                } else {
                    let a752 = v6;
                    let a753 = 10u8;
                    if equal(a752, a753) {
                        Ok(Some(Adt6::C0({
                            let a754 = v4;
                            let a755 = 1u64;
                            nat_add(a754, a755)?
                        }, {
                            let a756 = v5;
                            let a757 = 1u64;
                            nat_add(a756, a757)?
                        }, false, false)))
                    } else {
                        let v8: u64 = f54(v0, v4);
                        Ok(Some(Adt6::C0({
                            let a758 = v4;
                            let a759 = v8;
                            nat_add(a758, a759)?
                        }, v5, false, false)))
                    }
                }
            } else {
                Ok(Some(Adt6::C0(v4, v5, false, true)))
            }
        }
    }
}

pub fn f106(v0: u8, v1: u64) -> bool {
    let v2: bool = f56(v0, 48u8, 49u8);
    let v3: bool = f56(v0, 48u8, 55u8);
    let v4: bool = f56(v0, 48u8, 57u8);
    let v5: bool = f56(v0, 97u8, 102u8);
    let v6: bool = f56(v0, 65u8, 70u8);
    let a760 = v1;
    let a761 = 2u64;
    if nat_eq(a760, a761) {
        v2
    } else {
        let a762 = v1;
        let a763 = 8u64;
        if nat_eq(a762, a763) {
            v3
        } else {
            let a764 = v1;
            let a765 = 16u64;
            if nat_eq(a764, a765) {
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

pub fn f107(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u8 = f23(v0.clone(), v1);
    let v3: u8 = f23(v0.clone(), {
        let a766 = v1;
        let a767 = 1u64;
        nat_add(a766, a767)?
    });
    let v4: u8 = f23(v0, {
        let a768 = v1;
        let a769 = 2u64;
        nat_add(a768, a769)?
    });
    let a770 = v2;
    let a771 = 225u8;
    if equal(a770, a771) {
        let a772 = v3;
        let a773 = 181u8;
        if equal(a772, a773) {
            if f56(v4, 162u8, 170u8) {
                Ok(3u64)
            } else {
                Ok(0u64)
            }
        } else {
            Ok(0u64)
        }
    } else {
        let a774 = v2;
        let a775 = 226u8;
        if equal(a774, a775) {
            let a776 = v3;
            let a777 = 130u8;
            if equal(a776, a777) {
                if f56(v4, 128u8, 137u8) {
                    Ok(3u64)
                } else {
                    if f56(v4, 144u8, 156u8) {
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

pub fn f108(v0: Bytes, v1: u64) -> R<Result<(Adt3, u64), Bytes>> {
    let v2: u64 = {
        let a778 = v1;
        let a779 = 2u64;
        nat_add(a778, a779)?
    };
    let v3: Adt8 = {
        let (h780, _) = f110(Fn17::F111(v0.clone()), {
            let a784 = {
                let a782 = {
                    let a781 = v0.clone();
                    length_bytes(a781)
                };
                let a783 = v2;
                nat_sub(a782, a783)
            };
            let a785 = 1u64;
            nat_add(a784, a785)?
        }, Adt8::C0(v2, false, false, true))?;
        h780
    };
    let v4: Bytes = f51(v0, v2, {
        let a787 = match v3.clone() {
            Adt8::C0(h786, _, _, _) => {
                h786
            }
        };
        let a788 = v2;
        nat_sub(a787, a788)
    });
    if match v3.clone() {
        Adt8::C0(_, h789, _, _) => {
            h789
        }
    } {
        let a791 = 0u64;
        let a792 = {
            let a790 = v4.clone();
            length_bytes(a790)
        };
        if if nat_lt(a791, a792) {
            match v3.clone() {
                Adt8::C0(_, _, _, h793) => {
                    h793
                }
            }
        } else {
            false
        } {
            Ok(Ok::<(Adt3, u64), Bytes>((Adt3::C0(v4, true), {
                let a795 = match v3 {
                    Adt8::C0(h794, _, _, _) => {
                        h794
                    }
                };
                let a796 = 2u64;
                nat_add(a795, a796)?
            })))
        } else {
            Ok(Err::<(Adt3, u64), Bytes>({
                let a801 = {
                    let a798 = {
                        let a797 = Str::lit("the quoted name `");
                        utf8_encode(a797)
                    };
                    let a799 = v4;
                    append_bytes(a798, a799)
                };
                let a802 = {
                    let a800 = Str::lit("` is not a name");
                    utf8_encode(a800)
                };
                append_bytes(a801, a802)
            }))
        }
    } else {
        Ok(Err::<(Adt3, u64), Bytes>({
            let a803 = Str::lit("a quoted name is not closed");
            utf8_encode(a803)
        }))
    }
}

pub fn f109(v0: Bytes, v1: u64, v2: u64) -> R<bool> {
    let (_, h804) = {
        let (h805, _) = f112(Fn18::F113(v2, v1, v0), v2, (0u64, true))?;
        h805
    };
    Ok(h804)
}

pub fn f110(v0: Fn17, v1: u64, v2: Adt8) -> R<(Adt8, bool)> {
    let m806 = v1;
    if m806 == 0 {
        Ok((v2, false))
    } else {
        let v3 = m806 - 1;
        let m808 = {
            let c807: Fn17 = v0.clone();
            c807.apply(v2.clone())?
        };
        match m808 {
            None => {
                Ok((v2, true))
            }
            Some(v4) => {
                f110(v0, v3, v4)
            }
        }
    }
}

pub fn f111(v0: Bytes, v1: Adt8) -> R<Option<Adt8>> {
    f114(v0.clone(), {
        let a809 = v0;
        length_bytes(a809)
    }, v1)
}

pub fn f112(v0: Fn18, v1: u64, v2: (u64, bool)) -> R<((u64, bool), bool)> {
    let m810 = v1;
    if m810 == 0 {
        Ok((v2, false))
    } else {
        let v3 = m810 - 1;
        let m812 = {
            let c811: Fn18 = v0.clone();
            c811.apply(v2)?
        };
        match m812 {
            None => {
                Ok((v2, true))
            }
            Some(v4) => {
                f112(v0, v3, v4)
            }
        }
    }
}

pub fn f113(v0: u64, v1: u64, v2: Bytes, v3: (u64, bool)) -> R<Option<(u64, bool)>> {
    f115(v2, v1, v0, v3)
}

pub fn f114(v0: Bytes, v1: u64, v2: Adt8) -> R<Option<Adt8>> {
    let v3: u64 = match v2.clone() {
        Adt8::C0(h813, _, _, _) => {
            h813
        }
    };
    let v4: bool = match v2.clone() {
        Adt8::C0(_, _, _, h814) => {
            h814
        }
    };
    if match v2.clone() {
        Adt8::C0(_, h815, _, _) => {
            h815
        }
    } {
        Ok(None::<Adt8>)
    } else {
        if match v2 {
            Adt8::C0(_, _, h816, _) => {
                h816
            }
        } {
            Ok(None::<Adt8>)
        } else {
            let a817 = v3;
            let a818 = v1;
            if nat_lt(a817, a818) {
                let v5: bool = f24(v0.clone(), v3, 10u8);
                if v5 {
                    Ok(Some(Adt8::C0(v3, false, true, v4)))
                } else {
                    let v6: bool = f116(v0.clone(), v3)?;
                    if v6 {
                        Ok(Some(Adt8::C0(v3, true, false, v4)))
                    } else {
                        let v7: u64 = f100(v0.clone(), v3)?;
                        let a819 = 0u64;
                        let a820 = v7;
                        if nat_lt(a819, a820) {
                            Ok(Some(Adt8::C0({
                                let a821 = v3;
                                let a822 = v7;
                                nat_add(a821, a822)?
                            }, false, false, v4)))
                        } else {
                            let v8: bool = f24(v0.clone(), v3, 46u8);
                            if v8 {
                                Ok(Some(Adt8::C0({
                                    let a823 = v3;
                                    let a824 = 1u64;
                                    nat_add(a823, a824)?
                                }, false, false, v4)))
                            } else {
                                let v9: u64 = f54(v0, v3);
                                Ok(Some(Adt8::C0({
                                    let a825 = v3;
                                    let a826 = v9;
                                    nat_add(a825, a826)?
                                }, false, false, false)))
                            }
                        }
                    }
                }
            } else {
                Ok(Some(Adt8::C0(v3, false, true, v4)))
            }
        }
    }
}

pub fn f115(v0: Bytes, v1: u64, v2: u64, v3: (u64, bool)) -> R<Option<(u64, bool)>> {
    let v4: u64 = {
        let (h827, _) = v3;
        h827
    };
    let v5: bool = {
        let (_, h828) = v3;
        h828
    };
    if v5 {
        let a829 = v4;
        let a830 = v2;
        if nat_lt(a829, a830) {
            let v6: bool = f24(v0, {
                let a831 = v1;
                let a832 = v4;
                nat_add(a831, a832)?
            }, 35u8);
            Ok(Some(({
                let a833 = v4;
                let a834 = 1u64;
                nat_add(a833, a834)?
            }, v6)))
        } else {
            Ok(None::<(u64, bool)>)
        }
    } else {
        Ok(None::<(u64, bool)>)
    }
}

pub fn f116(v0: Bytes, v1: u64) -> R<bool> {
    let v2: bool = f24(v0.clone(), v1, 194u8);
    let v3: bool = f24(v0, {
        let a835 = v1;
        let a836 = 1u64;
        nat_add(a835, a836)?
    }, 187u8);
    if v2 {
        Ok(v3)
    } else {
        Ok(false)
    }
}
