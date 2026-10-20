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
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    C9,
}

#[derive(Clone)]
pub enum Fn0 {
    F6,
}

impl Fn0 {
    pub fn apply(&self, p0: (Option<(u64, i64)>, List<u64>), p1: i64) -> (Option<(u64, i64)>, List<u64>) {
        match self {
            Fn0::F6 => f6(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F14(List<i64>),
    F20(List<i64>),
}

impl Fn1 {
    pub fn apply(&self, p0: (List<i64>, List<i64>), p1: List<i64>) -> R<(List<i64>, List<i64>)> {
        match self {
            Fn1::F14(k0) => f14(k0.clone(), p0, p1),
            Fn1::F20(k0) => f20(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F18,
    F19,
}

impl Fn2 {
    pub fn apply(&self, p0: List<i64>, p1: i64) -> R<List<i64>> {
        match self {
            Fn2::F18 => f18(p0, p1),
            Fn2::F19 => Ok(f19(p0, p1)),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F25,
    F26,
}

impl Fn3 {
    pub fn apply(&self, p0: (i64, List<i64>), p1: i64) -> R<(i64, List<i64>)> {
        match self {
            Fn3::F25 => f25(p0, p1),
            Fn3::F26 => f26(p0, p1),
        }
    }
}

pub fn f0(v0: Adt0) -> R<Result<u64, (bool, bool)>> {
    let v1: Adt0 = v0;
    let v2: u64 = f1(v1.clone())?;
    if f2(v1, v2) {
        Ok(Ok::<u64, (bool, bool)>(v2))
    } else {
        Ok(Err::<u64, (bool, bool)>((true, false)))
    }
}

pub fn f1(v0: Adt0) -> R<u64> {
    f3(v0)
}

pub fn f2(v0: Adt0, v1: u64) -> bool {
    let a1 = v1;
    let a2 = f4(v0);
    nat_eq(a1, a2)
}

pub fn f3(v0: Adt0) -> R<u64> {
    let m4 = {
        let (h3, _) = f5(Fn0::F6, (None::<(u64, i64)>, List::cons(0u64, List::cons(1u64, List::cons(2u64, List::cons(3u64, List::cons(4u64, List::cons(5u64, List::cons(6u64, List::cons(7u64, List::cons(8u64, List::cons(9u64, List::<u64>::nil()))))))))))), f7(v0)?);
        h3
    };
    match m4 {
        None => {
            Ok(0u64)
        }
        Some(v1) => {
            let (h5, _) = v1;
            Ok(h5)
        }
    }
}

pub fn f4(v0: Adt0) -> u64 {
    let m6 = v0;
    match m6 {
        Adt0::C0 => {
            0u64
        }
        Adt0::C1 => {
            1u64
        }
        Adt0::C2 => {
            2u64
        }
        Adt0::C3 => {
            3u64
        }
        Adt0::C4 => {
            4u64
        }
        Adt0::C5 => {
            5u64
        }
        Adt0::C6 => {
            6u64
        }
        Adt0::C7 => {
            7u64
        }
        Adt0::C8 => {
            8u64
        }
        Adt0::C9 => {
            9u64
        }
    }
}

pub fn f5(v0: Fn0, p10: (Option<(u64, i64)>, List<u64>), p11: List<i64>) -> (Option<(u64, i64)>, List<u64>) {
    let m9 = (p10, p11);
    {
        let mut m9 = m9;
        loop {
            let step = {
                let (v1, v2) = m9;
                let m7 = v2;
                match m7.uncons() {
                    None => {
                        Err::<((Option<(u64, i64)>, List<u64>), List<i64>), (Option<(u64, i64)>, List<u64>)>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<((Option<(u64, i64)>, List<u64>), List<i64>), (Option<(u64, i64)>, List<u64>)>(({
                            let c8: Fn0 = v0.clone();
                            c8.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m9 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f6(v0: (Option<(u64, i64)>, List<u64>), v1: i64) -> (Option<(u64, i64)>, List<u64>) {
    let m13 = {
        let (_, h12) = v0.clone();
        h12
    };
    match m13.uncons() {
        None => {
            v0
        }
        Some((v2, v3)) => {
            ({
                let m15 = {
                    let (h14, _) = v0;
                    h14
                };
                match m15 {
                    None => {
                        Some((v2, v1))
                    }
                    Some(v4) => {
                        let m19 = {
                            let a17 = {
                                let (_, h16) = v4;
                                h16
                            };
                            let a18 = v1;
                            compare(a17, a18)
                        };
                        if match m19 {
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
                            Some((v2, v1))
                        } else {
                            Some(v4)
                        }
                    }
                }
            }, v3)
        }
    }
}

pub fn f7(v0: Adt0) -> R<List<i64>> {
    f8(f9(f10(f11(f12(v0))?)?)?)
}

pub fn f8(v0: List<i64>) -> R<List<i64>> {
    let (h20, _) = f13(Fn1::F14(v0), (List::<i64>::nil(), f15()), f16())?;
    Ok(h20)
}

pub fn f9(v0: List<i64>) -> R<List<i64>> {
    f17(Fn2::F18, List::<i64>::nil(), v0)
}

pub fn f10(v0: List<i64>) -> R<List<i64>> {
    f17(Fn2::F19, List::<i64>::nil(), v0)
}

pub fn f11(v0: List<i64>) -> R<List<i64>> {
    let (h21, _) = f13(Fn1::F20(v0), (List::<i64>::nil(), f21()), f22())?;
    Ok(h21)
}

pub fn f12(v0: Adt0) -> List<i64> {
    f23(v0)
}

pub fn f13(v0: Fn1, p25: (List<i64>, List<i64>), p26: List<List<i64>>) -> R<(List<i64>, List<i64>)> {
    let m24 = (p25, p26);
    {
        let mut m24 = m24;
        loop {
            let step = {
                let (v1, v2) = m24;
                let m22 = v2;
                match m22.uncons() {
                    None => {
                        Err::<((List<i64>, List<i64>), List<List<i64>>), R<(List<i64>, List<i64>)>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<((List<i64>, List<i64>), List<List<i64>>), R<(List<i64>, List<i64>)>>(({
                            let c23: Fn1 = v0.clone();
                            c23.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m24 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f14(v0: List<i64>, v1: (List<i64>, List<i64>), v2: List<i64>) -> R<(List<i64>, List<i64>)> {
    let m28 = {
        let (_, h27) = v1.clone();
        h27
    };
    match m28.uncons() {
        None => {
            Ok(v1)
        }
        Some((v3, v4)) => {
            Ok(({
                let a34 = {
                    let (h29, _) = v1;
                    h29
                };
                let a35 = List::cons({
                    let a32 = v3;
                    let a33 = {
                        let a31 = {
                            let (h30, _) = f24(Fn3::F25, (0i64, v0), v2)?;
                            h30
                        };
                        int_neg(a31)?
                    };
                    int_sub(a32, a33)?
                }, List::<i64>::nil());
                append_list(a34, a35)
            }, v4))
        }
    }
}

pub fn f15() -> List<i64> {
    List::cons(-6i64, List::cons(-2i64, List::cons(-5i64, List::cons(-5i64, List::cons(-4i64, List::cons(-5i64, List::cons(-6i64, List::cons(-3i64, List::cons(-7i64, List::cons(-6i64, List::<i64>::nil()))))))))))
}

pub fn f16() -> List<List<i64>> {
    List::cons(List::cons(2i64, List::cons(2i64, List::cons(2i64, List::cons(2i64, List::cons(2i64, List::cons(2i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::<i64>::nil())))))))))))), List::cons(List::cons(0i64, List::cons(2i64, List::cons(2i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::<i64>::nil())))))))))))), List::cons(List::cons(2i64, List::cons(2i64, List::cons(0i64, List::cons(2i64, List::cons(2i64, List::cons(0i64, List::cons(2i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::<i64>::nil())))))))))))), List::cons(List::cons(2i64, List::cons(2i64, List::cons(2i64, List::cons(2i64, List::cons(0i64, List::cons(0i64, List::cons(2i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::<i64>::nil())))))))))))), List::cons(List::cons(0i64, List::cons(2i64, List::cons(2i64, List::cons(0i64, List::cons(0i64, List::cons(2i64, List::cons(2i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::<i64>::nil())))))))))))), List::cons(List::cons(2i64, List::cons(0i64, List::cons(2i64, List::cons(2i64, List::cons(0i64, List::cons(2i64, List::cons(2i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::<i64>::nil())))))))))))), List::cons(List::cons(2i64, List::cons(0i64, List::cons(2i64, List::cons(2i64, List::cons(2i64, List::cons(2i64, List::cons(2i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::<i64>::nil())))))))))))), List::cons(List::cons(2i64, List::cons(2i64, List::cons(2i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::<i64>::nil())))))))))))), List::cons(List::cons(2i64, List::cons(2i64, List::cons(2i64, List::cons(2i64, List::cons(2i64, List::cons(2i64, List::cons(2i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::<i64>::nil())))))))))))), List::cons(List::cons(2i64, List::cons(2i64, List::cons(2i64, List::cons(2i64, List::cons(0i64, List::cons(2i64, List::cons(2i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::<i64>::nil())))))))))))), List::<List<i64>>::nil()))))))))))
}

pub fn f17(v0: Fn2, p39: List<i64>, p40: List<i64>) -> R<List<i64>> {
    let m38 = (p39, p40);
    {
        let mut m38 = m38;
        loop {
            let step = {
                let (v1, v2) = m38;
                let m36 = v2;
                match m36.uncons() {
                    None => {
                        Err::<(List<i64>, List<i64>), R<List<i64>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<i64>, List<i64>), R<List<i64>>>(({
                            let c37: Fn2 = v0.clone();
                            c37.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m38 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f18(v0: List<i64>, v1: i64) -> R<List<i64>> {
    let a50 = v0;
    let a51 = List::cons({
        let v2: i64 = {
            let a41 = v1;
            let a42 = 4i64;
            let a43 = 0i64;
            int_quot(a41, a42, a43)?
        };
        let m46 = {
            let a44 = v2;
            let a45 = 0i64;
            compare(a44, a45)
        };
        if match m46 {
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
            0i64
        } else {
            let m49 = {
                let a47 = 127i64;
                let a48 = v2;
                compare(a47, a48)
            };
            if match m49 {
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
                127i64
            } else {
                v2
            }
        }
    }, List::<i64>::nil());
    Ok(append_list(a50, a51))
}

pub fn f19(v0: List<i64>, v1: i64) -> List<i64> {
    let a55 = v0;
    let a56 = List::cons({
        let m54 = {
            let a52 = v1;
            let a53 = 0i64;
            compare(a52, a53)
        };
        if match m54 {
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
            0i64
        } else {
            v1
        }
    }, List::<i64>::nil());
    append_list(a55, a56)
}

pub fn f20(v0: List<i64>, v1: (List<i64>, List<i64>), v2: List<i64>) -> R<(List<i64>, List<i64>)> {
    let m58 = {
        let (_, h57) = v1.clone();
        h57
    };
    match m58.uncons() {
        None => {
            Ok(v1)
        }
        Some((v3, v4)) => {
            Ok(({
                let a64 = {
                    let (h59, _) = v1;
                    h59
                };
                let a65 = List::cons({
                    let a62 = v3;
                    let a63 = {
                        let a61 = {
                            let (h60, _) = f24(Fn3::F26, (0i64, v0), v2)?;
                            h60
                        };
                        int_neg(a61)?
                    };
                    int_sub(a62, a63)?
                }, List::<i64>::nil());
                append_list(a64, a65)
            }, v4))
        }
    }
}

pub fn f21() -> List<i64> {
    List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(1i64, List::cons(-2i64, List::cons(3i64, List::cons(-5i64, List::cons(2i64, List::<i64>::nil()))))))))))))
}

pub fn f22() -> List<List<i64>> {
    List::cons(List::cons(4i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::<i64>::nil()))))))), List::cons(List::cons(0i64, List::cons(4i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::<i64>::nil()))))))), List::cons(List::cons(0i64, List::cons(0i64, List::cons(4i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::<i64>::nil()))))))), List::cons(List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(4i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::<i64>::nil()))))))), List::cons(List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(4i64, List::cons(0i64, List::cons(0i64, List::<i64>::nil()))))))), List::cons(List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(4i64, List::cons(0i64, List::<i64>::nil()))))))), List::cons(List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(4i64, List::<i64>::nil()))))))), List::cons(List::cons(3i64, List::cons(-2i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(-1i64, List::<i64>::nil()))))))), List::cons(List::cons(0i64, List::cons(0i64, List::cons(2i64, List::cons(-3i64, List::cons(1i64, List::cons(0i64, List::cons(0i64, List::<i64>::nil()))))))), List::cons(List::cons(-1i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(2i64, List::cons(2i64, List::cons(-2i64, List::<i64>::nil()))))))), List::cons(List::cons(1i64, List::cons(1i64, List::cons(1i64, List::cons(1i64, List::cons(1i64, List::cons(1i64, List::cons(1i64, List::<i64>::nil()))))))), List::cons(List::cons(0i64, List::cons(-4i64, List::cons(0i64, List::cons(2i64, List::cons(0i64, List::cons(3i64, List::cons(0i64, List::<i64>::nil()))))))), List::<List<i64>>::nil()))))))))))))
}

pub fn f23(v0: Adt0) -> List<i64> {
    let m66 = v0;
    match m66 {
        Adt0::C0 => {
            List::cons(1i64, List::cons(1i64, List::cons(1i64, List::cons(1i64, List::cons(1i64, List::cons(1i64, List::cons(0i64, List::<i64>::nil())))))))
        }
        Adt0::C1 => {
            List::cons(0i64, List::cons(1i64, List::cons(1i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::<i64>::nil())))))))
        }
        Adt0::C2 => {
            List::cons(1i64, List::cons(1i64, List::cons(0i64, List::cons(1i64, List::cons(1i64, List::cons(0i64, List::cons(1i64, List::<i64>::nil())))))))
        }
        Adt0::C3 => {
            List::cons(1i64, List::cons(1i64, List::cons(1i64, List::cons(1i64, List::cons(0i64, List::cons(0i64, List::cons(1i64, List::<i64>::nil())))))))
        }
        Adt0::C4 => {
            List::cons(0i64, List::cons(1i64, List::cons(1i64, List::cons(0i64, List::cons(0i64, List::cons(1i64, List::cons(1i64, List::<i64>::nil())))))))
        }
        Adt0::C5 => {
            List::cons(1i64, List::cons(0i64, List::cons(1i64, List::cons(1i64, List::cons(0i64, List::cons(1i64, List::cons(1i64, List::<i64>::nil())))))))
        }
        Adt0::C6 => {
            List::cons(1i64, List::cons(0i64, List::cons(1i64, List::cons(1i64, List::cons(1i64, List::cons(1i64, List::cons(1i64, List::<i64>::nil())))))))
        }
        Adt0::C7 => {
            List::cons(1i64, List::cons(1i64, List::cons(1i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::cons(0i64, List::<i64>::nil())))))))
        }
        Adt0::C8 => {
            List::cons(1i64, List::cons(1i64, List::cons(1i64, List::cons(1i64, List::cons(1i64, List::cons(1i64, List::cons(1i64, List::<i64>::nil())))))))
        }
        Adt0::C9 => {
            List::cons(1i64, List::cons(1i64, List::cons(1i64, List::cons(1i64, List::cons(0i64, List::cons(1i64, List::cons(1i64, List::<i64>::nil())))))))
        }
    }
}

pub fn f24(v0: Fn3, p70: (i64, List<i64>), p71: List<i64>) -> R<(i64, List<i64>)> {
    let m69 = (p70, p71);
    {
        let mut m69 = m69;
        loop {
            let step = {
                let (v1, v2) = m69;
                let m67 = v2;
                match m67.uncons() {
                    None => {
                        Err::<((i64, List<i64>), List<i64>), R<(i64, List<i64>)>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<((i64, List<i64>), List<i64>), R<(i64, List<i64>)>>(({
                            let c68: Fn3 = v0.clone();
                            c68.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m69 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f25(v0: (i64, List<i64>), v1: i64) -> R<(i64, List<i64>)> {
    let m73 = {
        let (_, h72) = v0.clone();
        h72
    };
    match m73.uncons() {
        None => {
            Ok(v0)
        }
        Some((v2, v3)) => {
            Ok(({
                let a78 = {
                    let (h74, _) = v0;
                    h74
                };
                let a79 = {
                    let a77 = {
                        let a75 = v1;
                        let a76 = v2;
                        int_mul(a75, a76)?
                    };
                    int_neg(a77)?
                };
                int_sub(a78, a79)?
            }, v3))
        }
    }
}

pub fn f26(v0: (i64, List<i64>), v1: i64) -> R<(i64, List<i64>)> {
    let m81 = {
        let (_, h80) = v0.clone();
        h80
    };
    match m81.uncons() {
        None => {
            Ok(v0)
        }
        Some((v2, v3)) => {
            Ok(({
                let a86 = {
                    let (h82, _) = v0;
                    h82
                };
                let a87 = {
                    let a85 = {
                        let a83 = v1;
                        let a84 = v2;
                        int_mul(a83, a84)?
                    };
                    int_neg(a85)?
                };
                int_sub(a86, a87)?
            }, v3))
        }
    }
}
