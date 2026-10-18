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
    C4(Adt1),
    C5,
    C6,
    C7,
    C8(std::rc::Rc<Adt0>),
    C9(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
    C10(std::rc::Rc<Adt0>),
    C11(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
    C12(u64),
    C13(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
    C14,
    C15(std::rc::Rc<Adt0>, std::rc::Rc<Adt0>),
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
    C0(Adt0, Adt3),
    C1(u64),
    C2(u64, Adt0, std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C3(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C4(Adt0, std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C5(Adt5, Adt0, std::rc::Rc<Adt2>),
    C6(u64, std::rc::Rc<Adt2>),
    C7(u64, std::rc::Rc<Adt2>),
    C8(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C9(Adt6, std::rc::Rc<Adt2>),
    C10(std::rc::Rc<Adt2>),
    C11(std::rc::Rc<Adt2>),
    C12(std::rc::Rc<Adt2>, u64),
    C13,
    C14(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C15(Adt5, List<u64>, std::rc::Rc<Adt2>),
}

#[derive(Clone)]
pub enum Adt3 {
    C0,
    C1(bool),
    C2(Bytes),
    C3(Bytes),
    C4(Bytes),
    C5(Bytes),
    C6(Bytes),
    C7(Bytes),
    C8(Bytes),
    C9(Bytes),
    C10(Bytes),
    C11(Bytes),
    C12(Bytes),
    C13(Bytes),
    C14(Adt4),
    C15,
    C16(std::rc::Rc<Adt3>),
    C17(std::rc::Rc<Adt3>),
    C18(std::rc::Rc<Adt3>),
    C19(std::rc::Rc<Adt3>),
    C20(std::rc::Rc<Adt3>, std::rc::Rc<Adt3>),
    C21(u64, std::rc::Rc<Adt3>),
    C22(u64, std::rc::Rc<Adt3>),
    C23,
    C24(std::rc::Rc<Adt3>, std::rc::Rc<Adt3>),
}

#[derive(Clone)]
pub enum Adt4 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt5 {
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
    C10,
    C11,
    C12,
    C13,
    C14,
    C15(u64),
}

#[derive(Clone)]
pub enum Adt6 {
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
    C10,
    C11,
    C12,
    C13,
    C14,
    C15,
    C16,
    C17,
    C18,
    C19,
    C20,
    C21,
    C22,
    C23,
    C24,
    C25,
    C26,
    C27,
    C28,
    C29,
    C30,
    C31,
    C32,
    C33,
    C34,
    C35,
    C36,
    C37,
    C38,
    C39,
    C40(Adt1),
    C41(Adt0),
}

#[derive(Clone)]
pub enum Adt7 {
    C0(Bytes, Adt0, List<List<u64>>, Adt0, Adt0, Adt2),
}

#[derive(Clone)]
pub enum Adt8 {
    C0(Bytes),
    C1(Adt0),
    C2(List<Adt5>),
    C3,
}

#[derive(Clone)]
pub enum Adt9 {
    C0(bool, Bytes),
}

#[derive(Clone)]
pub enum Fn0 {
    F24(Bytes),
    F93(Bytes),
    F99(Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn0::F24(k0) => f24(k0.clone(), p0),
            Fn0::F93(k0) => f93(k0.clone(), p0),
            Fn0::F99(k0) => f99(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F32,
}

impl Fn1 {
    pub fn apply(&self, p0: u64, p1: u64) -> R<u64> {
        match self {
            Fn1::F32 => f32(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F89(Bytes),
}

impl Fn2 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn2::F89(k0) => f89(k0.clone(), p0),
        }
    }
}

pub fn f0(v0: Bytes, v1: Adt0, v2: List<List<u64>>, v3: Adt0, v4: Adt0, v5: Adt2) -> R<Result<(), Bytes>> {
    let v6: Adt7 = Adt7::C0(v0, v1, v2, v3, v4, v5);
    let a2 = match v6.clone() {
        Adt7::C0(h1, _, _, _, _, _) => {
            h1
        }
    };
    let a3 = f1(Str::lit("lexlean/target-program/1"));
    if equal(a2, a3) {
        let m5 = f2(v6.clone(), match v6.clone() {
            Adt7::C0(_, h4, _, _, _, _) => {
                h4
            }
        }, 0u64)?;
        match m5 {
            Ok(_) => {
                f3(v6.clone(), match v6 {
                    Adt7::C0(_, _, _, _, _, h6) => {
                        h6
                    }
                }, 0u64)
            }
            Err(v8) => {
                Ok(Err::<(), Bytes>(v8))
            }
        }
    } else {
        Ok(Err::<(), Bytes>(f4(f1(Str::lit("target program has spec ")), f5(match v6 {
            Adt7::C0(h7, _, _, _, _, _) => {
                h7
            }
        }))))
    }
}

pub fn f1(v0: Str) -> Bytes {
    let a8 = v0;
    utf8_encode(a8)
}

pub fn f2(v0: Adt7, v1: Adt0, v2: u64) -> R<Result<(), Bytes>> {
    let m9 = v1;
    match m9 {
        Adt0::C0 => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt0::C1 => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt0::C2 => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt0::C3 => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt0::C4(_) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt0::C5 => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt0::C6 => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt0::C7 => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt0::C8(_) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt0::C9(_, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt0::C10(_) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt0::C11(_, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt0::C12(_) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt0::C13(_, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt0::C14 => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt0::C15(r13, r14) => {
            let v13 = (*r13).clone();
            let v14 = (*r14).clone();
            if f6(v13.clone()) {
                Ok(Err::<(), Bytes>(f4(f1(Str::lit("ADT ")), f4(f7(v2)?, f1(Str::lit(" has no constructor"))))))
            } else {
                let m10 = f8(v0.clone(), v13)?;
                match m10 {
                    Ok(_) => {
                        f2(v0, v14, {
                            let a11 = v2;
                            let a12 = 1u64;
                            nat_add(a11, a12)?
                        })
                    }
                    Err(v16) => {
                        Ok(Err::<(), Bytes>(f4(f1(Str::lit("ADT ")), f4(f7(v2)?, f4(f1(Str::lit(": ")), v16)))))
                    }
                }
            }
        }
    }
}

pub fn f3(v0: Adt7, v1: Adt2, v2: u64) -> R<Result<(), Bytes>> {
    let m15 = v1;
    match m15 {
        Adt2::C0(_, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt2::C1(_) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt2::C2(_, _, _, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt2::C3(_, _, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt2::C4(_, _, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt2::C5(_, _, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt2::C6(_, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt2::C7(_, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt2::C8(_, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt2::C9(_, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt2::C10(_) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt2::C11(_) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt2::C12(_, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt2::C13 => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt2::C14(r19, r20) => {
            let v3 = (*r19).clone();
            let v4 = (*r20).clone();
            let m16 = f9(v0.clone(), v3, v2)?;
            match m16 {
                Ok(_) => {
                    f3(v0, v4, {
                        let a17 = v2;
                        let a18 = 1u64;
                        nat_add(a17, a18)?
                    })
                }
                Err(v6) => {
                    Ok(Err::<(), Bytes>(v6))
                }
            }
        }
        Adt2::C15(_, _, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
    }
}

pub fn f4(v0: Bytes, v1: Bytes) -> Bytes {
    let a21 = v0;
    let a22 = v1;
    append_bytes(a21, a22)
}

pub fn f5(v0: Bytes) -> Bytes {
    f4(f1(Str::lit("`")), f4(v0, f1(Str::lit("`"))))
}

pub fn f6(v0: Adt0) -> bool {
    let a23 = f10(v0);
    let a24 = 14u64;
    nat_eq(a23, a24)
}

pub fn f7(v0: u64) -> R<Bytes> {
    let v1: Bytes = f1(Str::lit("0"));
    let v2: Bytes = f11({
        let a61 = f12(v0, 10000000000000000000u64);
        let a62 = {
            let a59 = f12(v0, 1000000000000000000u64);
            let a60 = {
                let a57 = f12(v0, 100000000000000000u64);
                let a58 = {
                    let a55 = f12(v0, 10000000000000000u64);
                    let a56 = {
                        let a53 = f12(v0, 1000000000000000u64);
                        let a54 = {
                            let a51 = f12(v0, 100000000000000u64);
                            let a52 = {
                                let a49 = f12(v0, 10000000000000u64);
                                let a50 = {
                                    let a47 = f12(v0, 1000000000000u64);
                                    let a48 = {
                                        let a45 = f12(v0, 100000000000u64);
                                        let a46 = {
                                            let a43 = f12(v0, 10000000000u64);
                                            let a44 = {
                                                let a41 = f12(v0, 1000000000u64);
                                                let a42 = {
                                                    let a39 = f12(v0, 100000000u64);
                                                    let a40 = {
                                                        let a37 = f12(v0, 10000000u64);
                                                        let a38 = {
                                                            let a35 = f12(v0, 1000000u64);
                                                            let a36 = {
                                                                let a33 = f12(v0, 100000u64);
                                                                let a34 = {
                                                                    let a31 = f12(v0, 10000u64);
                                                                    let a32 = {
                                                                        let a29 = f12(v0, 1000u64);
                                                                        let a30 = {
                                                                            let a27 = f12(v0, 100u64);
                                                                            let a28 = {
                                                                                let a25 = f12(v0, 10u64);
                                                                                let a26 = f12(v0, 1u64);
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
                };
                append_bytes(a57, a58)
            };
            append_bytes(a59, a60)
        };
        append_bytes(a61, a62)
    })?;
    let a63 = v0;
    let a64 = 0u64;
    if nat_eq(a63, a64) {
        Ok(v1)
    } else {
        Ok(v2)
    }
}

pub fn f8(v0: Adt7, v1: Adt0) -> R<Result<(), Bytes>> {
    let m65 = v1;
    match m65 {
        Adt0::C0 => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt0::C1 => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt0::C2 => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt0::C3 => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt0::C4(_) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt0::C5 => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt0::C6 => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt0::C7 => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt0::C8(_) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt0::C9(_, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt0::C10(_) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt0::C11(_, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt0::C12(_) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt0::C13(_, _) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt0::C14 => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt0::C15(r67, r68) => {
            let v12 = (*r67).clone();
            let v13 = (*r68).clone();
            let m66 = f13(v0.clone(), v12)?;
            match m66 {
                Ok(_) => {
                    f8(v0, v13)
                }
                Err(v15) => {
                    Ok(Err::<(), Bytes>(v15))
                }
            }
        }
    }
}

pub fn f9(v0: Adt7, v1: Adt2, v2: u64) -> R<Result<(), Bytes>> {
    let v3: List<u64> = f14(v0.clone(), v2);
    let v4: Adt0 = f15(v0.clone(), v2);
    let v5: Adt0 = f16(v0.clone(), v2);
    let v6: u64 = f17(v3.clone())?;
    let v7: u64 = f18(v4.clone())?;
    let a69 = v6;
    let a70 = v7;
    if nat_eq(a69, a70) {
        if f19(v3.clone()) {
            Ok(Err::<(), Bytes>(f4(f1(Str::lit("function ")), f4(f7(v2)?, f4(f1(Str::lit(": ")), f1(Str::lit("a parameter name repeats")))))))
        } else {
            let m71 = f13(v0.clone(), v4.clone())?;
            match m71 {
                Ok(_) => {
                    let m72 = f13(v0.clone(), v5.clone())?;
                    match m72 {
                        Ok(_) => {
                            let m73 = f20(v0, v1, f21(v3, v4, List::<(u64, Adt0)>::nil()), 0u64, Adt0::C14, Adt0::C14, f1(Str::lit("")), 0u64, List::<Adt5>::nil())?;
                            match m73 {
                                Adt8::C0(v12) => {
                                    Ok(Err::<(), Bytes>(f4(f1(Str::lit("function ")), f4(f7(v2)?, f4(f1(Str::lit(": ")), v12)))))
                                }
                                Adt8::C1(v14) => {
                                    let m74 = f22(v14, v5, f1(Str::lit("the body")))?;
                                    match m74 {
                                        Ok(_) => {
                                            Ok(Ok::<(), Bytes>(()))
                                        }
                                        Err(v15) => {
                                            Ok(Err::<(), Bytes>(f4(f1(Str::lit("function ")), f4(f7(v2)?, f4(f1(Str::lit(": ")), v15)))))
                                        }
                                    }
                                }
                                Adt8::C2(_) => {
                                    Ok(Err::<(), Bytes>(f1(Str::lit("unreachable"))))
                                }
                                Adt8::C3 => {
                                    Ok(Err::<(), Bytes>(f1(Str::lit("unreachable"))))
                                }
                            }
                        }
                        Err(v10) => {
                            Ok(Err::<(), Bytes>(f4(f1(Str::lit("function ")), f4(f7(v2)?, f4(f1(Str::lit(": ")), v10)))))
                        }
                    }
                }
                Err(v8) => {
                    Ok(Err::<(), Bytes>(f4(f1(Str::lit("function ")), f4(f7(v2)?, f4(f1(Str::lit(": ")), v8)))))
                }
            }
        }
    } else {
        Ok(Err::<(), Bytes>(f4(f1(Str::lit("function ")), f4(f7(v2)?, f4(f1(Str::lit(": ")), f4(f7(v6)?, f4(f1(Str::lit(" parameter names for ")), f4(f7(v7)?, f1(Str::lit(" parameter types"))))))))))
    }
}

pub fn f10(v0: Adt0) -> u64 {
    let m75 = v0;
    match m75 {
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
        Adt0::C4(_) => {
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
        Adt0::C8(_) => {
            8u64
        }
        Adt0::C9(_, _) => {
            9u64
        }
        Adt0::C10(_) => {
            10u64
        }
        Adt0::C11(_, _) => {
            11u64
        }
        Adt0::C12(_) => {
            12u64
        }
        Adt0::C13(_, _) => {
            13u64
        }
        Adt0::C14 => {
            14u64
        }
        Adt0::C15(_, _) => {
            15u64
        }
    }
}

pub fn f11(v0: Bytes) -> R<Bytes> {
    let v1: u64 = {
        let (h76, _) = f23(Fn0::F24(v0.clone()), {
            let a77 = v0.clone();
            length_bytes(a77)
        }, 0u64)?;
        h76
    };
    Ok(f25(v0.clone(), v1, {
        let a79 = {
            let a78 = v0;
            length_bytes(a78)
        };
        let a80 = v1;
        nat_sub(a79, a80)
    }))
}

pub fn f12(v0: u64, v1: u64) -> Bytes {
    f25(f1(Str::lit("0123456789")), {
        let a84 = {
            let a81 = v0;
            let a82 = v1;
            let a83 = 0u64;
            nat_quot(a81, a82, a83)
        };
        let a85 = 10u64;
        let a86 = 0u64;
        nat_rem(a84, a85, a86)
    }, 1u64)
}

pub fn f13(v0: Adt7, v1: Adt0) -> R<Result<(), Bytes>> {
    let m87 = v1;
    match m87 {
        Adt0::C0 => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt0::C1 => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt0::C2 => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt0::C3 => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt0::C4(_) => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt0::C5 => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt0::C6 => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt0::C7 => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt0::C8(r93) => {
            let v3 = (*r93).clone();
            f13(v0, v3)
        }
        Adt0::C9(r94, r95) => {
            let v4 = (*r94).clone();
            let v5 = (*r95).clone();
            let m88 = f13(v0.clone(), v4)?;
            match m88 {
                Ok(_) => {
                    f13(v0, v5)
                }
                Err(v7) => {
                    Ok(Err::<(), Bytes>(v7))
                }
            }
        }
        Adt0::C10(r96) => {
            let v8 = (*r96).clone();
            f13(v0, v8)
        }
        Adt0::C11(r97, r98) => {
            let v9 = (*r97).clone();
            let v10 = (*r98).clone();
            let m89 = f13(v0.clone(), v9)?;
            match m89 {
                Ok(_) => {
                    f13(v0, v10)
                }
                Err(v12) => {
                    Ok(Err::<(), Bytes>(v12))
                }
            }
        }
        Adt0::C12(v13) => {
            let m90 = f26(v0, v13);
            match m90 {
                None => {
                    Ok(Err::<(), Bytes>(f27(v13)?))
                }
                Some(_) => {
                    Ok(Ok::<(), Bytes>(()))
                }
            }
        }
        Adt0::C13(r99, r100) => {
            let v15 = (*r99).clone();
            let v16 = (*r100).clone();
            if f6(v15.clone()) {
                Ok(Err::<(), Bytes>(f1(Str::lit("a function type takes at least one parameter"))))
            } else {
                let m91 = f13(v0.clone(), v15)?;
                match m91 {
                    Ok(_) => {
                        f13(v0, v16)
                    }
                    Err(v18) => {
                        Ok(Err::<(), Bytes>(v18))
                    }
                }
            }
        }
        Adt0::C14 => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt0::C15(r101, r102) => {
            let v19 = (*r101).clone();
            let v20 = (*r102).clone();
            let m92 = f13(v0.clone(), v19)?;
            match m92 {
                Ok(_) => {
                    f13(v0, v20)
                }
                Err(v22) => {
                    Ok(Err::<(), Bytes>(v22))
                }
            }
        }
    }
}

pub fn f14(v0: Adt7, v1: u64) -> List<u64> {
    let m104 = f28(match v0 {
        Adt7::C0(_, _, h103, _, _, _) => {
            h103
        }
    }, v1);
    match m104 {
        None => {
            List::<u64>::nil()
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f15(v0: Adt7, v1: u64) -> Adt0 {
    let m105 = f29(v0, v1);
    match m105 {
        None => {
            Adt0::C14
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f16(v0: Adt7, v1: u64) -> Adt0 {
    let m107 = f30(match v0 {
        Adt7::C0(_, _, _, _, h106, _) => {
            h106
        }
    }, v1);
    match m107 {
        None => {
            Adt0::C0
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f17(v0: List<u64>) -> R<u64> {
    f31(Fn1::F32, 0u64, v0)
}

pub fn f18(v0: Adt0) -> R<u64> {
    let m108 = v0;
    match m108 {
        Adt0::C0 => {
            Ok(0u64)
        }
        Adt0::C1 => {
            Ok(0u64)
        }
        Adt0::C2 => {
            Ok(0u64)
        }
        Adt0::C3 => {
            Ok(0u64)
        }
        Adt0::C4(_) => {
            Ok(0u64)
        }
        Adt0::C5 => {
            Ok(0u64)
        }
        Adt0::C6 => {
            Ok(0u64)
        }
        Adt0::C7 => {
            Ok(0u64)
        }
        Adt0::C8(_) => {
            Ok(0u64)
        }
        Adt0::C9(_, _) => {
            Ok(0u64)
        }
        Adt0::C10(_) => {
            Ok(0u64)
        }
        Adt0::C11(_, _) => {
            Ok(0u64)
        }
        Adt0::C12(_) => {
            Ok(0u64)
        }
        Adt0::C13(_, _) => {
            Ok(0u64)
        }
        Adt0::C14 => {
            Ok(0u64)
        }
        Adt0::C15(_, r111) => {
            let v12 = (*r111).clone();
            let v13: u64 = f18(v12)?;
            let a109 = 1u64;
            let a110 = v13;
            nat_add(a109, a110)
        }
    }
}

pub fn f19(v0: List<u64>) -> bool {
    let m112 = v0;
    match m112.uncons() {
        None => {
            false
        }
        Some((v1, v2)) => {
            let v3: bool = f19(v2.clone());
            let v4: bool = f33(v2, v1);
            if v4 {
                true
            } else {
                v3
            }
        }
    }
}

pub fn f20(v0: Adt7, v1: Adt2, v2: List<(u64, Adt0)>, v3: u64, v4: Adt0, v5: Adt0, v6: Bytes, v7: u64, v8: List<Adt5>) -> R<Adt8> {
    let m113 = v1;
    match m113 {
        Adt2::C0(v9, v10) => {
            let a114 = v3;
            let a115 = 0u64;
            if nat_eq(a114, a115) {
                let m116 = f13(v0.clone(), v9.clone())?;
                match m116 {
                    Ok(_) => {
                        let m117 = f34(v0, v10, v9.clone(), 0u64)?;
                        match m117 {
                            Ok(_) => {
                                Ok(Adt8::C1(v9))
                            }
                            Err(v13) => {
                                Ok(Adt8::C0(v13))
                            }
                        }
                    }
                    Err(v14) => {
                        Ok(Adt8::C0(v14))
                    }
                }
            } else {
                Ok(Adt8::C0(f1(Str::lit("unreachable"))))
            }
        }
        Adt2::C1(v15) => {
            let a118 = v3;
            let a119 = 0u64;
            if nat_eq(a118, a119) {
                let m120 = f35(v2, v15);
                match m120 {
                    None => {
                        Ok(Adt8::C0(f4(f1(Str::lit("local ")), f4(f7(v15)?, f1(Str::lit(" is unbound"))))))
                    }
                    Some(v16) => {
                        Ok(Adt8::C1(v16))
                    }
                }
            } else {
                Ok(Adt8::C0(f1(Str::lit("unreachable"))))
            }
        }
        Adt2::C2(v17, v18, r225, r226) => {
            let v19 = (*r225).clone();
            let v20 = (*r226).clone();
            let a121 = v3;
            let a122 = 0u64;
            if nat_eq(a121, a122) {
                let m123 = f13(v0.clone(), v18.clone())?;
                match m123 {
                    Ok(_) => {
                        let m124 = f20(v0.clone(), v19, v2.clone(), 0u64, Adt0::C14, Adt0::C14, f1(Str::lit("")), 0u64, List::<Adt5>::nil())?;
                        match m124 {
                            Adt8::C0(v25) => {
                                Ok(Adt8::C0(v25))
                            }
                            Adt8::C1(v22) => {
                                let m125 = f22(v22, v18.clone(), f4(f1(Str::lit("the value bound to ")), f7(v17)?))?;
                                match m125 {
                                    Ok(_) => {
                                        f20(v0, v20, f36(v17, v18, v2), 0u64, Adt0::C14, Adt0::C14, f1(Str::lit("")), 0u64, List::<Adt5>::nil())
                                    }
                                    Err(v24) => {
                                        Ok(Adt8::C0(v24))
                                    }
                                }
                            }
                            Adt8::C2(_) => {
                                Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                            }
                            Adt8::C3 => {
                                Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                            }
                        }
                    }
                    Err(v27) => {
                        Ok(Adt8::C0(v27))
                    }
                }
            } else {
                Ok(Adt8::C0(f1(Str::lit("unreachable"))))
            }
        }
        Adt2::C3(r227, r228, r229) => {
            let v28 = (*r227).clone();
            let v29 = (*r228).clone();
            let v30 = (*r229).clone();
            let a126 = v3;
            let a127 = 0u64;
            if nat_eq(a126, a127) {
                let m128 = f20(v0.clone(), v28, v2.clone(), 0u64, Adt0::C14, Adt0::C14, f1(Str::lit("")), 0u64, List::<Adt5>::nil())?;
                match m128 {
                    Adt8::C0(v42) => {
                        Ok(Adt8::C0(v42))
                    }
                    Adt8::C1(v31) => {
                        let m129 = f22(v31, Adt0::C1, f1(Str::lit("a condition")))?;
                        match m129 {
                            Ok(_) => {
                                let m130 = f20(v0.clone(), v29, v2.clone(), 0u64, Adt0::C14, Adt0::C14, f1(Str::lit("")), 0u64, List::<Adt5>::nil())?;
                                match m130 {
                                    Adt8::C0(v39) => {
                                        Ok(Adt8::C0(v39))
                                    }
                                    Adt8::C1(v33) => {
                                        let m131 = f20(v0, v30, v2, 0u64, Adt0::C14, Adt0::C14, f1(Str::lit("")), 0u64, List::<Adt5>::nil())?;
                                        match m131 {
                                            Adt8::C0(v37) => {
                                                Ok(Adt8::C0(v37))
                                            }
                                            Adt8::C1(v34) => {
                                                let m132 = f22(v34, v33.clone(), f1(Str::lit("the else branch")))?;
                                                match m132 {
                                                    Ok(_) => {
                                                        Ok(Adt8::C1(v33))
                                                    }
                                                    Err(v36) => {
                                                        Ok(Adt8::C0(v36))
                                                    }
                                                }
                                            }
                                            Adt8::C2(_) => {
                                                Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                                            }
                                            Adt8::C3 => {
                                                Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                                            }
                                        }
                                    }
                                    Adt8::C2(_) => {
                                        Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                                    }
                                    Adt8::C3 => {
                                        Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                                    }
                                }
                            }
                            Err(v41) => {
                                Ok(Adt8::C0(v41))
                            }
                        }
                    }
                    Adt8::C2(_) => {
                        Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                    }
                    Adt8::C3 => {
                        Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                    }
                }
            } else {
                Ok(Adt8::C0(f1(Str::lit("unreachable"))))
            }
        }
        Adt2::C4(v44, r230, r231) => {
            let v45 = (*r230).clone();
            let v46 = (*r231).clone();
            let a133 = v3;
            let a134 = 0u64;
            if nat_eq(a133, a134) {
                let m135 = f13(v0.clone(), v44.clone())?;
                match m135 {
                    Ok(_) => {
                        let m136 = f20(v0.clone(), v45, v2.clone(), 0u64, Adt0::C14, Adt0::C14, f1(Str::lit("")), 0u64, List::<Adt5>::nil())?;
                        match m136 {
                            Adt8::C0(v55) => {
                                Ok(Adt8::C0(v55))
                            }
                            Adt8::C1(v48) => {
                                let m137 = f37(v0.clone(), v48.clone())?;
                                match m137 {
                                    Ok(v49) => {
                                        let m138 = f20(v0, v46, v2, 4u64, v44.clone(), v48, f1(Str::lit("")), 0u64, List::<Adt5>::nil())?;
                                        match m138 {
                                            Adt8::C0(v52) => {
                                                Ok(Adt8::C0(v52))
                                            }
                                            Adt8::C1(_) => {
                                                Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                                            }
                                            Adt8::C2(v50) => {
                                                let m139 = f38(v49, v50);
                                                match m139 {
                                                    None => {
                                                        Ok(Adt8::C1(v44))
                                                    }
                                                    Some(v51) => {
                                                        Ok(Adt8::C0(f4(f1(Str::lit("the match is not exhaustive: ")), f4(f39(v51)?, f1(Str::lit(" is not covered"))))))
                                                    }
                                                }
                                            }
                                            Adt8::C3 => {
                                                Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                                            }
                                        }
                                    }
                                    Err(v54) => {
                                        Ok(Adt8::C0(v54))
                                    }
                                }
                            }
                            Adt8::C2(_) => {
                                Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                            }
                            Adt8::C3 => {
                                Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                            }
                        }
                    }
                    Err(v57) => {
                        Ok(Adt8::C0(v57))
                    }
                }
            } else {
                Ok(Adt8::C0(f1(Str::lit("unreachable"))))
            }
        }
        Adt2::C5(v58, v59, r232) => {
            let v60 = (*r232).clone();
            let a140 = v3;
            let a141 = 0u64;
            if nat_eq(a140, a141) {
                let m142 = f13(v0.clone(), v59.clone())?;
                match m142 {
                    Ok(_) => {
                        let m143 = f40(v0.clone(), v58.clone(), v59.clone())?;
                        match m143 {
                            Ok(v62) => {
                                let v63: Bytes = f4(f1(Str::lit("constructor ")), f39(v58)?);
                                let a144 = f41(v60.clone())?;
                                let a145 = f18(v62.clone())?;
                                if nat_eq(a144, a145) {
                                    let m146 = f20(v0, v60, v2, 2u64, v62, Adt0::C14, v63, 0u64, List::<Adt5>::nil())?;
                                    match m146 {
                                        Adt8::C0(v64) => {
                                            Ok(Adt8::C0(v64))
                                        }
                                        Adt8::C1(_) => {
                                            Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                                        }
                                        Adt8::C2(_) => {
                                            Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                                        }
                                        Adt8::C3 => {
                                            Ok(Adt8::C1(v59))
                                        }
                                    }
                                } else {
                                    Ok(Adt8::C0(f4(v63, f4(f1(Str::lit(" takes ")), f4(f7(f18(v62)?)?, f4(f1(Str::lit(" operand(s), received ")), f7(f41(v60)?)?))))))
                                }
                            }
                            Err(v67) => {
                                Ok(Adt8::C0(v67))
                            }
                        }
                    }
                    Err(v68) => {
                        Ok(Adt8::C0(v68))
                    }
                }
            } else {
                Ok(Adt8::C0(f1(Str::lit("unreachable"))))
            }
        }
        Adt2::C6(v69, r233) => {
            let v70 = (*r233).clone();
            let a147 = v3;
            let a148 = 0u64;
            if nat_eq(a147, a148) {
                let m149 = f29(v0.clone(), v69);
                match m149 {
                    None => {
                        Ok(Adt8::C0(f42(v69)?))
                    }
                    Some(v71) => {
                        let v72: Bytes = f4(f1(Str::lit("function ")), f7(v69)?);
                        let a150 = f41(v70.clone())?;
                        let a151 = f18(v71.clone())?;
                        if nat_eq(a150, a151) {
                            let m152 = f20(v0.clone(), v70, v2, 2u64, v71, Adt0::C14, v72, 0u64, List::<Adt5>::nil())?;
                            match m152 {
                                Adt8::C0(v73) => {
                                    Ok(Adt8::C0(v73))
                                }
                                Adt8::C1(_) => {
                                    Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                                }
                                Adt8::C2(_) => {
                                    Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                                }
                                Adt8::C3 => {
                                    Ok(Adt8::C1(f16(v0, v69)))
                                }
                            }
                        } else {
                            Ok(Adt8::C0(f4(v72, f4(f1(Str::lit(" takes ")), f4(f7(f18(v71)?)?, f4(f1(Str::lit(" operand(s), received ")), f7(f41(v70)?)?))))))
                        }
                    }
                }
            } else {
                Ok(Adt8::C0(f1(Str::lit("unreachable"))))
            }
        }
        Adt2::C7(v76, r234) => {
            let v77 = (*r234).clone();
            let a153 = v3;
            let a154 = 0u64;
            if nat_eq(a153, a154) {
                let m155 = f29(v0.clone(), v76);
                match m155 {
                    None => {
                        Ok(Adt8::C0(f42(v76)?))
                    }
                    Some(v78) => {
                        let v79: u64 = f18(v78.clone())?;
                        let v80: u64 = f41(v77.clone())?;
                        let a156 = v79;
                        let a157 = v80;
                        if nat_le(a156, a157) {
                            Ok(Adt8::C0(f4(f1(Str::lit("closure of function ")), f4(f7(v76)?, f4(f1(Str::lit(" captures ")), f4(f7(v80)?, f4(f1(Str::lit(" of its ")), f4(f7(v79)?, f1(Str::lit(" parameters; at least one must remain"))))))))))
                        } else {
                            let m158 = f20(v0.clone(), v77, v2, 3u64, v78, Adt0::C14, f4(f1(Str::lit("closure of function ")), f7(v76)?), 0u64, List::<Adt5>::nil())?;
                            match m158 {
                                Adt8::C0(v82) => {
                                    Ok(Adt8::C0(v82))
                                }
                                Adt8::C1(v81) => {
                                    Ok(Adt8::C1(f43(v81, f16(v0, v76))))
                                }
                                Adt8::C2(_) => {
                                    Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                                }
                                Adt8::C3 => {
                                    Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                                }
                            }
                        }
                    }
                }
            } else {
                Ok(Adt8::C0(f1(Str::lit("unreachable"))))
            }
        }
        Adt2::C8(r235, r236) => {
            let v84 = (*r235).clone();
            let v85 = (*r236).clone();
            let a159 = v3;
            let a160 = 0u64;
            if nat_eq(a159, a160) {
                let m161 = f20(v0.clone(), v84, v2.clone(), 0u64, Adt0::C14, Adt0::C14, f1(Str::lit("")), 0u64, List::<Adt5>::nil())?;
                match m161 {
                    Adt8::C0(v91) => {
                        Ok(Adt8::C0(v91))
                    }
                    Adt8::C1(v86) => {
                        let a162 = f10(v86.clone());
                        let a163 = 13u64;
                        if nat_eq(a162, a163) {
                            let v87: Bytes = f1(Str::lit("an application"));
                            let a164 = f41(v85.clone())?;
                            let a165 = f18(f44(v86.clone()))?;
                            if nat_eq(a164, a165) {
                                let m166 = f20(v0, v85, v2, 2u64, f44(v86.clone()), Adt0::C14, v87, 0u64, List::<Adt5>::nil())?;
                                match m166 {
                                    Adt8::C0(v88) => {
                                        Ok(Adt8::C0(v88))
                                    }
                                    Adt8::C1(_) => {
                                        Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                                    }
                                    Adt8::C2(_) => {
                                        Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                                    }
                                    Adt8::C3 => {
                                        Ok(Adt8::C1(f45(v86)))
                                    }
                                }
                            } else {
                                Ok(Adt8::C0(f4(v87, f4(f1(Str::lit(" takes ")), f4(f7(f18(f44(v86))?)?, f4(f1(Str::lit(" operand(s), received ")), f7(f41(v85)?)?))))))
                            }
                        } else {
                            Ok(Adt8::C0(f1(Str::lit("only a function value can be applied"))))
                        }
                    }
                    Adt8::C2(_) => {
                        Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                    }
                    Adt8::C3 => {
                        Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                    }
                }
            } else {
                Ok(Adt8::C0(f1(Str::lit("unreachable"))))
            }
        }
        Adt2::C9(v93, r237) => {
            let v94 = (*r237).clone();
            let a167 = v3;
            let a168 = 0u64;
            if nat_eq(a167, a168) {
                let m169 = f20(v0, v94, v2, 1u64, Adt0::C14, Adt0::C14, f1(Str::lit("")), 0u64, List::<Adt5>::nil())?;
                match m169 {
                    Adt8::C0(v98) => {
                        Ok(Adt8::C0(v98))
                    }
                    Adt8::C1(v95) => {
                        let m170 = f46(v93, v95)?;
                        match m170 {
                            Ok(v96) => {
                                Ok(Adt8::C1(v96))
                            }
                            Err(v97) => {
                                Ok(Adt8::C0(v97))
                            }
                        }
                    }
                    Adt8::C2(_) => {
                        Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                    }
                    Adt8::C3 => {
                        Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                    }
                }
            } else {
                Ok(Adt8::C0(f1(Str::lit("unreachable"))))
            }
        }
        Adt2::C10(r238) => {
            let v100 = (*r238).clone();
            let a171 = v3;
            let a172 = 0u64;
            if nat_eq(a171, a172) {
                let m173 = f20(v0, v100, v2, 0u64, Adt0::C14, Adt0::C14, f1(Str::lit("")), 0u64, List::<Adt5>::nil())?;
                match m173 {
                    Adt8::C0(v102) => {
                        Ok(Adt8::C0(v102))
                    }
                    Adt8::C1(v101) => {
                        let a174 = f10(v101.clone());
                        let a175 = 11u64;
                        if nat_eq(a174, a175) {
                            Ok(Adt8::C1(f44(v101)))
                        } else {
                            Ok(Adt8::C0(f4(f1(Str::lit("`first` of ")), f47(v101)?)))
                        }
                    }
                    Adt8::C2(_) => {
                        Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                    }
                    Adt8::C3 => {
                        Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                    }
                }
            } else {
                Ok(Adt8::C0(f1(Str::lit("unreachable"))))
            }
        }
        Adt2::C11(r239) => {
            let v104 = (*r239).clone();
            let a176 = v3;
            let a177 = 0u64;
            if nat_eq(a176, a177) {
                let m178 = f20(v0, v104, v2, 0u64, Adt0::C14, Adt0::C14, f1(Str::lit("")), 0u64, List::<Adt5>::nil())?;
                match m178 {
                    Adt8::C0(v106) => {
                        Ok(Adt8::C0(v106))
                    }
                    Adt8::C1(v105) => {
                        let a179 = f10(v105.clone());
                        let a180 = 11u64;
                        if nat_eq(a179, a180) {
                            Ok(Adt8::C1(f45(v105)))
                        } else {
                            Ok(Adt8::C0(f4(f1(Str::lit("`second` of ")), f47(v105)?)))
                        }
                    }
                    Adt8::C2(_) => {
                        Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                    }
                    Adt8::C3 => {
                        Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                    }
                }
            } else {
                Ok(Adt8::C0(f1(Str::lit("unreachable"))))
            }
        }
        Adt2::C12(r240, v109) => {
            let v108 = (*r240).clone();
            let a181 = v3;
            let a182 = 0u64;
            if nat_eq(a181, a182) {
                let m183 = f20(v0.clone(), v108, v2, 0u64, Adt0::C14, Adt0::C14, f1(Str::lit("")), 0u64, List::<Adt5>::nil())?;
                match m183 {
                    Adt8::C0(v115) => {
                        Ok(Adt8::C0(v115))
                    }
                    Adt8::C1(v110) => {
                        let a184 = f10(v110.clone());
                        let a185 = 12u64;
                        if nat_eq(a184, a185) {
                            let v111: u64 = f48(v110);
                            let m186 = f26(v0, v111);
                            match m186 {
                                None => {
                                    Ok(Adt8::C0(f27(v111)?))
                                }
                                Some(v112) => {
                                    let a187 = f18(v112.clone())?;
                                    let a188 = 1u64;
                                    if nat_eq(a187, a188) {
                                        let m189 = f30(v112, 0u64);
                                        match m189 {
                                            None => {
                                                Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                                            }
                                            Some(v113) => {
                                                let m190 = f30(v113, v109);
                                                match m190 {
                                                    None => {
                                                        Ok(Adt8::C0(f4(f1(Str::lit("ADT ")), f4(f7(v111)?, f4(f1(Str::lit(" has no field ")), f7(v109)?)))))
                                                    }
                                                    Some(v114) => {
                                                        Ok(Adt8::C1(v114))
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        Ok(Adt8::C0(f4(f1(Str::lit("`field` requires an ADT with exactly one constructor; ADT ")), f4(f7(v111)?, f4(f1(Str::lit(" has ")), f7(f18(v112)?)?)))))
                                    }
                                }
                            }
                        } else {
                            Ok(Adt8::C0(f1(Str::lit("`field` requires an ADT value"))))
                        }
                    }
                    Adt8::C2(_) => {
                        Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                    }
                    Adt8::C3 => {
                        Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                    }
                }
            } else {
                Ok(Adt8::C0(f1(Str::lit("unreachable"))))
            }
        }
        Adt2::C13 => {
            let a191 = v3;
            let a192 = 1u64;
            if nat_eq(a191, a192) {
                Ok(Adt8::C1(Adt0::C14))
            } else {
                let a193 = v3;
                let a194 = 2u64;
                if nat_eq(a193, a194) {
                    Ok(Adt8::C3)
                } else {
                    let a195 = v3;
                    let a196 = 3u64;
                    if nat_eq(a195, a196) {
                        Ok(Adt8::C1(v4))
                    } else {
                        let a197 = v3;
                        let a198 = 4u64;
                        if nat_eq(a197, a198) {
                            Ok(Adt8::C2(v8))
                        } else {
                            Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                        }
                    }
                }
            }
        }
        Adt2::C14(r241, r242) => {
            let v117 = (*r241).clone();
            let v118 = (*r242).clone();
            let a199 = v3;
            let a200 = 1u64;
            if nat_eq(a199, a200) {
                let m201 = f20(v0.clone(), v117, v2.clone(), 0u64, Adt0::C14, Adt0::C14, f1(Str::lit("")), 0u64, List::<Adt5>::nil())?;
                match m201 {
                    Adt8::C0(v123) => {
                        Ok(Adt8::C0(v123))
                    }
                    Adt8::C1(v119) => {
                        let m202 = f20(v0, v118, v2, 1u64, Adt0::C14, Adt0::C14, f1(Str::lit("")), 0u64, List::<Adt5>::nil())?;
                        match m202 {
                            Adt8::C0(v121) => {
                                Ok(Adt8::C0(v121))
                            }
                            Adt8::C1(v120) => {
                                Ok(Adt8::C1(f49(v119, v120)))
                            }
                            Adt8::C2(_) => {
                                Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                            }
                            Adt8::C3 => {
                                Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                            }
                        }
                    }
                    Adt8::C2(_) => {
                        Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                    }
                    Adt8::C3 => {
                        Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                    }
                }
            } else {
                let a203 = v3;
                let a204 = 2u64;
                if nat_eq(a203, a204) {
                    if f6(v4.clone()) {
                        Ok(Adt8::C3)
                    } else {
                        let m205 = f20(v0.clone(), v117, v2.clone(), 0u64, Adt0::C14, Adt0::C14, f1(Str::lit("")), 0u64, List::<Adt5>::nil())?;
                        match m205 {
                            Adt8::C0(v128) => {
                                Ok(Adt8::C0(v128))
                            }
                            Adt8::C1(v125) => {
                                let m206 = f22(v125, f44(v4.clone()), f4(v6.clone(), f4(f1(Str::lit(" operand ")), f7(v7)?)))?;
                                match m206 {
                                    Ok(_) => {
                                        f20(v0, v118, v2, 2u64, f45(v4), Adt0::C14, v6, {
                                            let a207 = v7;
                                            let a208 = 1u64;
                                            nat_add(a207, a208)?
                                        }, List::<Adt5>::nil())
                                    }
                                    Err(v127) => {
                                        Ok(Adt8::C0(v127))
                                    }
                                }
                            }
                            Adt8::C2(_) => {
                                Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                            }
                            Adt8::C3 => {
                                Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                            }
                        }
                    }
                } else {
                    let a209 = v3;
                    let a210 = 3u64;
                    if nat_eq(a209, a210) {
                        if f6(v4.clone()) {
                            Ok(Adt8::C1(Adt0::C14))
                        } else {
                            let m211 = f20(v0.clone(), v117, v2.clone(), 0u64, Adt0::C14, Adt0::C14, f1(Str::lit("")), 0u64, List::<Adt5>::nil())?;
                            match m211 {
                                Adt8::C0(v133) => {
                                    Ok(Adt8::C0(v133))
                                }
                                Adt8::C1(v130) => {
                                    let m212 = f22(v130, f44(v4.clone()), f4(v6.clone(), f4(f1(Str::lit(" operand ")), f7(v7)?)))?;
                                    match m212 {
                                        Ok(_) => {
                                            f20(v0, v118, v2, 3u64, f45(v4), Adt0::C14, v6, {
                                                let a213 = v7;
                                                let a214 = 1u64;
                                                nat_add(a213, a214)?
                                            }, List::<Adt5>::nil())
                                        }
                                        Err(v132) => {
                                            Ok(Adt8::C0(v132))
                                        }
                                    }
                                }
                                Adt8::C2(_) => {
                                    Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                                }
                                Adt8::C3 => {
                                    Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                                }
                            }
                        }
                    } else {
                        let a215 = v3;
                        let a216 = 4u64;
                        if nat_eq(a215, a216) {
                            let m217 = f20(v0.clone(), v117, v2.clone(), 5u64, v4.clone(), v5.clone(), f1(Str::lit("")), 0u64, v8)?;
                            match m217 {
                                Adt8::C0(v136) => {
                                    Ok(Adt8::C0(v136))
                                }
                                Adt8::C1(_) => {
                                    Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                                }
                                Adt8::C2(v135) => {
                                    f20(v0, v118, v2, 4u64, v4, v5, f1(Str::lit("")), 0u64, v135)
                                }
                                Adt8::C3 => {
                                    Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                                }
                            }
                        } else {
                            Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                        }
                    }
                }
            }
        }
        Adt2::C15(v138, v139, r243) => {
            let v140 = (*r243).clone();
            let a218 = v3;
            let a219 = 5u64;
            if nat_eq(a218, a219) {
                let m220 = f40(v0.clone(), v138.clone(), v5)?;
                match m220 {
                    Ok(v141) => {
                        let v142: u64 = f17(v139.clone())?;
                        let v143: u64 = f18(v141.clone())?;
                        let a221 = v142;
                        let a222 = v143;
                        if nat_eq(a221, a222) {
                            if f19(v139.clone()) {
                                Ok(Adt8::C0(f4(f1(Str::lit("shape ")), f4(f39(v138)?, f1(Str::lit(" binds the same name twice"))))))
                            } else {
                                if f50(v8.clone(), v138.clone()) {
                                    Ok(Adt8::C0(f4(f1(Str::lit("the arm for ")), f4(f39(v138)?, f1(Str::lit(" is unreachable"))))))
                                } else {
                                    let m223 = f20(v0, v140, f21(v139, v141, v2), 0u64, Adt0::C14, Adt0::C14, f1(Str::lit("")), 0u64, List::<Adt5>::nil())?;
                                    match m223 {
                                        Adt8::C0(v147) => {
                                            Ok(Adt8::C0(v147))
                                        }
                                        Adt8::C1(v144) => {
                                            let m224 = f22(v144, v4, f1(Str::lit("an arm")))?;
                                            match m224 {
                                                Ok(_) => {
                                                    Ok(Adt8::C2(f51(v138, v8)))
                                                }
                                                Err(v146) => {
                                                    Ok(Adt8::C0(v146))
                                                }
                                            }
                                        }
                                        Adt8::C2(_) => {
                                            Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                                        }
                                        Adt8::C3 => {
                                            Ok(Adt8::C0(f1(Str::lit("unreachable"))))
                                        }
                                    }
                                }
                            }
                        } else {
                            Ok(Adt8::C0(f4(f1(Str::lit("shape ")), f4(f39(v138)?, f4(f1(Str::lit(" binds ")), f4(f7(v143)?, f4(f1(Str::lit(" value(s), the arm names ")), f7(v142)?)))))))
                        }
                    }
                    Err(v149) => {
                        Ok(Adt8::C0(v149))
                    }
                }
            } else {
                Ok(Adt8::C0(f1(Str::lit("unreachable"))))
            }
        }
    }
}

pub fn f21(v0: List<u64>, v1: Adt0, v2: List<(u64, Adt0)>) -> List<(u64, Adt0)> {
    let m244 = v0;
    match m244.uncons() {
        None => {
            v2
        }
        Some((v3, v4)) => {
            let v5: List<(u64, Adt0)> = f21(v4, f45(v1.clone()), f36(v3, f44(v1.clone()), v2.clone()));
            if f6(v1) {
                v2
            } else {
                v5
            }
        }
    }
}

pub fn f22(v0: Adt0, v1: Adt0, v2: Bytes) -> R<Result<(), Bytes>> {
    let v3: Result<(), Bytes> = Ok::<(), Bytes>(());
    let v4: Result<(), Bytes> = Err::<(), Bytes>(f4(v2, f4(f1(Str::lit(" has type ")), f4(f47(v0.clone())?, f4(f1(Str::lit(", expected ")), f47(v1.clone())?)))));
    if f52(v0, v1) {
        Ok(v3)
    } else {
        Ok(v4)
    }
}

pub fn f23(v0: Fn0, v1: u64, v2: u64) -> R<(u64, bool)> {
    let m245 = v1;
    if m245 == 0 {
        Ok((v2, false))
    } else {
        let v3 = m245 - 1;
        let m247 = {
            let c246: Fn0 = v0.clone();
            c246.apply(v2)?
        };
        match m247 {
            None => {
                Ok((v2, true))
            }
            Some(v4) => {
                f23(v0, v3, v4)
            }
        }
    }
}

pub fn f24(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f53(v0, v1)
}

pub fn f25(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m251 = {
        let a248 = v0;
        let a249 = v1;
        let a250 = v2;
        slice_bytes(a248, a249, a250)
    };
    match m251 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f26(v0: Adt7, v1: u64) -> Option<Adt0> {
    f30(match v0 {
        Adt7::C0(_, h252, _, _, _, _) => {
            h252
        }
    }, v1)
}

pub fn f27(v0: u64) -> R<Bytes> {
    Ok(f4(f1(Str::lit("ADT ")), f4(f7(v0)?, f1(Str::lit(" is not declared")))))
}

pub fn f28(v0: List<List<u64>>, v1: u64) -> Option<List<u64>> {
    let m253 = v0;
    match m253.uncons() {
        None => {
            None::<List<u64>>
        }
        Some((v2, v3)) => {
            let m254 = v1;
            if m254 == 0 {
                Some(v2)
            } else {
                let v4 = m254 - 1;
                f28(v3, v4)
            }
        }
    }
}

pub fn f29(v0: Adt7, v1: u64) -> Option<Adt0> {
    f30(match v0 {
        Adt7::C0(_, _, _, h255, _, _) => {
            h255
        }
    }, v1)
}

pub fn f30(v0: Adt0, v1: u64) -> Option<Adt0> {
    let m256 = v0;
    match m256 {
        Adt0::C0 => {
            None::<Adt0>
        }
        Adt0::C1 => {
            None::<Adt0>
        }
        Adt0::C2 => {
            None::<Adt0>
        }
        Adt0::C3 => {
            None::<Adt0>
        }
        Adt0::C4(_) => {
            None::<Adt0>
        }
        Adt0::C5 => {
            None::<Adt0>
        }
        Adt0::C6 => {
            None::<Adt0>
        }
        Adt0::C7 => {
            None::<Adt0>
        }
        Adt0::C8(_) => {
            None::<Adt0>
        }
        Adt0::C9(_, _) => {
            None::<Adt0>
        }
        Adt0::C10(_) => {
            None::<Adt0>
        }
        Adt0::C11(_, _) => {
            None::<Adt0>
        }
        Adt0::C12(_) => {
            None::<Adt0>
        }
        Adt0::C13(_, _) => {
            None::<Adt0>
        }
        Adt0::C14 => {
            None::<Adt0>
        }
        Adt0::C15(r258, r259) => {
            let v12 = (*r258).clone();
            let v13 = (*r259).clone();
            let m257 = v1;
            if m257 == 0 {
                Some(v12)
            } else {
                let v14 = m257 - 1;
                f30(v13, v14)
            }
        }
    }
}

pub fn f31(v0: Fn1, v1: u64, v2: List<u64>) -> R<u64> {
    let m260 = v2;
    match m260.uncons() {
        None => {
            Ok(v1)
        }
        Some((v3, v4)) => {
            f31(v0.clone(), {
                let c261: Fn1 = v0;
                c261.apply(v1, v3)?
            }, v4)
        }
    }
}

pub fn f32(v0: u64, _: u64) -> R<u64> {
    let a262 = v0;
    let a263 = 1u64;
    nat_add(a262, a263)
}

pub fn f33(v0: List<u64>, v1: u64) -> bool {
    let m264 = v0;
    match m264.uncons() {
        None => {
            false
        }
        Some((v2, v3)) => {
            let v4: bool = f33(v3, v1);
            let a265 = v2;
            let a266 = v1;
            if nat_eq(a265, a266) {
                true
            } else {
                v4
            }
        }
    }
}

pub fn f34(v0: Adt7, v1: Adt3, v2: Adt0, v3: u64) -> R<Result<(), Bytes>> {
    let m267 = v1;
    match m267 {
        Adt3::C0 => {
            let v4: Result<(), Bytes> = Ok::<(), Bytes>(());
            let v5: Result<(), Bytes> = Err::<(), Bytes>(f4(f1(Str::lit("the literal ")), f4(f54(Adt3::C0)?, f4(f1(Str::lit(" does not have type ")), f47(v2.clone())?))));
            let a268 = f10(v2);
            let a269 = 0u64;
            if nat_eq(a268, a269) {
                Ok(v4)
            } else {
                Ok(v5)
            }
        }
        Adt3::C1(v6) => {
            let v7: Result<(), Bytes> = Ok::<(), Bytes>(());
            let v8: Result<(), Bytes> = Err::<(), Bytes>(f4(f1(Str::lit("the literal ")), f4(f54(Adt3::C1(v6))?, f4(f1(Str::lit(" does not have type ")), f47(v2.clone())?))));
            let a270 = f10(v2);
            let a271 = 1u64;
            if nat_eq(a270, a271) {
                Ok(v7)
            } else {
                Ok(v8)
            }
        }
        Adt3::C2(v9) => {
            let v10: Result<(), Bytes> = f55(v9.clone())?;
            let v11: Result<(), Bytes> = Err::<(), Bytes>(f4(f1(Str::lit("the literal ")), f4(f54(Adt3::C2(v9))?, f4(f1(Str::lit(" does not have type ")), f47(v2.clone())?))));
            let a272 = f10(v2);
            let a273 = 2u64;
            if nat_eq(a272, a273) {
                Ok(v10)
            } else {
                Ok(v11)
            }
        }
        Adt3::C3(v12) => {
            let v13: Result<(), Bytes> = f56(v12.clone(), Adt1::C7)?;
            let v14: Result<(), Bytes> = Err::<(), Bytes>(f4(f1(Str::lit("the literal ")), f4(f54(Adt3::C3(v12))?, f4(f1(Str::lit(" does not have type ")), f47(v2.clone())?))));
            let a274 = f10(v2);
            let a275 = 3u64;
            if nat_eq(a274, a275) {
                Ok(v13)
            } else {
                Ok(v14)
            }
        }
        Adt3::C4(v15) => {
            let v16: Result<(), Bytes> = f56(v15.clone(), Adt1::C0)?;
            let v17: Result<(), Bytes> = Err::<(), Bytes>(f4(f1(Str::lit("the literal ")), f4(f54(Adt3::C4(v15))?, f4(f1(Str::lit(" does not have type ")), f47(v2.clone())?))));
            let a276 = f10(v2.clone());
            let a277 = 4u64;
            if if nat_eq(a276, a277) {
                let a278 = f57(f58(v2));
                let a279 = 0u64;
                nat_eq(a278, a279)
            } else {
                false
            } {
                Ok(v16)
            } else {
                Ok(v17)
            }
        }
        Adt3::C5(v18) => {
            let v19: Result<(), Bytes> = f56(v18.clone(), Adt1::C1)?;
            let v20: Result<(), Bytes> = Err::<(), Bytes>(f4(f1(Str::lit("the literal ")), f4(f54(Adt3::C5(v18))?, f4(f1(Str::lit(" does not have type ")), f47(v2.clone())?))));
            let a280 = f10(v2.clone());
            let a281 = 4u64;
            if if nat_eq(a280, a281) {
                let a282 = f57(f58(v2));
                let a283 = 1u64;
                nat_eq(a282, a283)
            } else {
                false
            } {
                Ok(v19)
            } else {
                Ok(v20)
            }
        }
        Adt3::C6(v21) => {
            let v22: Result<(), Bytes> = f56(v21.clone(), Adt1::C2)?;
            let v23: Result<(), Bytes> = Err::<(), Bytes>(f4(f1(Str::lit("the literal ")), f4(f54(Adt3::C6(v21))?, f4(f1(Str::lit(" does not have type ")), f47(v2.clone())?))));
            let a284 = f10(v2.clone());
            let a285 = 4u64;
            if if nat_eq(a284, a285) {
                let a286 = f57(f58(v2));
                let a287 = 2u64;
                nat_eq(a286, a287)
            } else {
                false
            } {
                Ok(v22)
            } else {
                Ok(v23)
            }
        }
        Adt3::C7(v24) => {
            let v25: Result<(), Bytes> = f56(v24.clone(), Adt1::C3)?;
            let v26: Result<(), Bytes> = Err::<(), Bytes>(f4(f1(Str::lit("the literal ")), f4(f54(Adt3::C7(v24))?, f4(f1(Str::lit(" does not have type ")), f47(v2.clone())?))));
            let a288 = f10(v2.clone());
            let a289 = 4u64;
            if if nat_eq(a288, a289) {
                let a290 = f57(f58(v2));
                let a291 = 3u64;
                nat_eq(a290, a291)
            } else {
                false
            } {
                Ok(v25)
            } else {
                Ok(v26)
            }
        }
        Adt3::C8(v27) => {
            let v28: Result<(), Bytes> = f56(v27.clone(), Adt1::C4)?;
            let v29: Result<(), Bytes> = Err::<(), Bytes>(f4(f1(Str::lit("the literal ")), f4(f54(Adt3::C8(v27))?, f4(f1(Str::lit(" does not have type ")), f47(v2.clone())?))));
            let a292 = f10(v2.clone());
            let a293 = 4u64;
            if if nat_eq(a292, a293) {
                let a294 = f57(f58(v2));
                let a295 = 4u64;
                nat_eq(a294, a295)
            } else {
                false
            } {
                Ok(v28)
            } else {
                Ok(v29)
            }
        }
        Adt3::C9(v30) => {
            let v31: Result<(), Bytes> = f56(v30.clone(), Adt1::C5)?;
            let v32: Result<(), Bytes> = Err::<(), Bytes>(f4(f1(Str::lit("the literal ")), f4(f54(Adt3::C9(v30))?, f4(f1(Str::lit(" does not have type ")), f47(v2.clone())?))));
            let a296 = f10(v2.clone());
            let a297 = 4u64;
            if if nat_eq(a296, a297) {
                let a298 = f57(f58(v2));
                let a299 = 5u64;
                nat_eq(a298, a299)
            } else {
                false
            } {
                Ok(v31)
            } else {
                Ok(v32)
            }
        }
        Adt3::C10(v33) => {
            let v34: Result<(), Bytes> = f56(v33.clone(), Adt1::C6)?;
            let v35: Result<(), Bytes> = Err::<(), Bytes>(f4(f1(Str::lit("the literal ")), f4(f54(Adt3::C10(v33))?, f4(f1(Str::lit(" does not have type ")), f47(v2.clone())?))));
            let a300 = f10(v2.clone());
            let a301 = 4u64;
            if if nat_eq(a300, a301) {
                let a302 = f57(f58(v2));
                let a303 = 6u64;
                nat_eq(a302, a303)
            } else {
                false
            } {
                Ok(v34)
            } else {
                Ok(v35)
            }
        }
        Adt3::C11(v36) => {
            let v37: Result<(), Bytes> = f56(v36.clone(), Adt1::C7)?;
            let v38: Result<(), Bytes> = Err::<(), Bytes>(f4(f1(Str::lit("the literal ")), f4(f54(Adt3::C11(v36))?, f4(f1(Str::lit(" does not have type ")), f47(v2.clone())?))));
            let a304 = f10(v2.clone());
            let a305 = 4u64;
            if if nat_eq(a304, a305) {
                let a306 = f57(f58(v2));
                let a307 = 7u64;
                nat_eq(a306, a307)
            } else {
                false
            } {
                Ok(v37)
            } else {
                Ok(v38)
            }
        }
        Adt3::C12(v39) => {
            let v40: Result<(), Bytes> = Ok::<(), Bytes>(());
            let v41: Result<(), Bytes> = Err::<(), Bytes>(f4(f1(Str::lit("the literal ")), f4(f54(Adt3::C12(v39))?, f4(f1(Str::lit(" does not have type ")), f47(v2.clone())?))));
            let a308 = f10(v2);
            let a309 = 5u64;
            if nat_eq(a308, a309) {
                Ok(v40)
            } else {
                Ok(v41)
            }
        }
        Adt3::C13(v42) => {
            let v43: Result<(), Bytes> = f59(v42.clone())?;
            let v44: Result<(), Bytes> = Err::<(), Bytes>(f4(f1(Str::lit("the literal ")), f4(f54(Adt3::C13(v42))?, f4(f1(Str::lit(" does not have type ")), f47(v2.clone())?))));
            let a310 = f10(v2);
            let a311 = 6u64;
            if nat_eq(a310, a311) {
                Ok(v43)
            } else {
                Ok(v44)
            }
        }
        Adt3::C14(v45) => {
            let v46: Result<(), Bytes> = Ok::<(), Bytes>(());
            let v47: Result<(), Bytes> = Err::<(), Bytes>(f4(f1(Str::lit("the literal ")), f4(f54(Adt3::C14(v45))?, f4(f1(Str::lit(" does not have type ")), f47(v2.clone())?))));
            let a312 = f10(v2);
            let a313 = 7u64;
            if nat_eq(a312, a313) {
                Ok(v46)
            } else {
                Ok(v47)
            }
        }
        Adt3::C15 => {
            let v48: Result<(), Bytes> = Ok::<(), Bytes>(());
            let v49: Result<(), Bytes> = Err::<(), Bytes>(f4(f1(Str::lit("the literal ")), f4(f54(Adt3::C15)?, f4(f1(Str::lit(" does not have type ")), f47(v2.clone())?))));
            let a314 = f10(v2);
            let a315 = 8u64;
            if nat_eq(a314, a315) {
                Ok(v48)
            } else {
                Ok(v49)
            }
        }
        Adt3::C16(r341) => {
            let v50 = (*r341).clone();
            let v51: Result<(), Bytes> = f34(v0, v50.clone(), f44(v2.clone()), 0u64)?;
            let v52: Result<(), Bytes> = Err::<(), Bytes>(f4(f1(Str::lit("the literal ")), f4(f54(Adt3::C16(std::rc::Rc::new(v50)))?, f4(f1(Str::lit(" does not have type ")), f47(v2.clone())?))));
            let a316 = f10(v2);
            let a317 = 8u64;
            if nat_eq(a316, a317) {
                Ok(v51)
            } else {
                Ok(v52)
            }
        }
        Adt3::C17(r342) => {
            let v53 = (*r342).clone();
            let v54: Result<(), Bytes> = f34(v0, v53.clone(), f44(v2.clone()), 0u64)?;
            let v55: Result<(), Bytes> = Err::<(), Bytes>(f4(f1(Str::lit("the literal ")), f4(f54(Adt3::C17(std::rc::Rc::new(v53)))?, f4(f1(Str::lit(" does not have type ")), f47(v2.clone())?))));
            let a318 = f10(v2);
            let a319 = 9u64;
            if nat_eq(a318, a319) {
                Ok(v54)
            } else {
                Ok(v55)
            }
        }
        Adt3::C18(r343) => {
            let v56 = (*r343).clone();
            let v57: Result<(), Bytes> = f34(v0, v56.clone(), f45(v2.clone()), 0u64)?;
            let v58: Result<(), Bytes> = Err::<(), Bytes>(f4(f1(Str::lit("the literal ")), f4(f54(Adt3::C18(std::rc::Rc::new(v56)))?, f4(f1(Str::lit(" does not have type ")), f47(v2.clone())?))));
            let a320 = f10(v2);
            let a321 = 9u64;
            if nat_eq(a320, a321) {
                Ok(v57)
            } else {
                Ok(v58)
            }
        }
        Adt3::C19(r344) => {
            let v59 = (*r344).clone();
            let v60: Result<(), Bytes> = f34(v0, v59.clone(), f44(v2.clone()), 1u64)?;
            let v61: Result<(), Bytes> = Err::<(), Bytes>(f4(f1(Str::lit("the literal ")), f4(f54(Adt3::C19(std::rc::Rc::new(v59)))?, f4(f1(Str::lit(" does not have type ")), f47(v2.clone())?))));
            let a322 = f10(v2);
            let a323 = 10u64;
            if nat_eq(a322, a323) {
                Ok(v60)
            } else {
                Ok(v61)
            }
        }
        Adt3::C20(r345, r346) => {
            let v62 = (*r345).clone();
            let v63 = (*r346).clone();
            let v64: Result<(), Bytes> = f34(v0.clone(), v62.clone(), f44(v2.clone()), 0u64)?;
            let v65: Result<(), Bytes> = f34(v0, v63.clone(), f45(v2.clone()), 0u64)?;
            let v68: Result<(), Bytes> = {
                let m324 = v64;
                match m324 {
                    Ok(_) => {
                        v65
                    }
                    Err(v67) => {
                        Err::<(), Bytes>(v67)
                    }
                }
            };
            let v69: Result<(), Bytes> = Err::<(), Bytes>(f4(f1(Str::lit("the literal ")), f4(f54(Adt3::C20(std::rc::Rc::new(v62), std::rc::Rc::new(v63)))?, f4(f1(Str::lit(" does not have type ")), f47(v2.clone())?))));
            let a325 = f10(v2);
            let a326 = 11u64;
            if nat_eq(a325, a326) {
                Ok(v68)
            } else {
                Ok(v69)
            }
        }
        Adt3::C21(v70, r347) => {
            let v71 = (*r347).clone();
            let v77: Result<(), Bytes> = {
                let m327 = f60(v0.clone(), f48(v2.clone()), v70)?;
                match m327 {
                    Ok(v73) => {
                        let v74: Result<(), Bytes> = f34(v0, v71.clone(), v73.clone(), 2u64)?;
                        let v75: Result<(), Bytes> = v74;
                        let v76: Result<(), Bytes> = Err::<(), Bytes>(f4(f1(Str::lit("constructor ")), f4(f7(v70)?, f4(f1(Str::lit(" of ADT ")), f4(f7(f48(v2.clone()))?, f4(f1(Str::lit(" takes ")), f4(f7(f18(v73.clone())?)?, f4(f1(Str::lit(" fields, the literal has ")), f7(f61(v71.clone())?)?))))))));
                        let a328 = f61(v71.clone())?;
                        let a329 = f18(v73)?;
                        if nat_eq(a328, a329) {
                            v75
                        } else {
                            v76
                        }
                    }
                    Err(v72) => {
                        Err::<(), Bytes>(v72)
                    }
                }
            };
            let v78: Result<(), Bytes> = Err::<(), Bytes>(f4(f1(Str::lit("the literal ")), f4(f54(Adt3::C21(v70, std::rc::Rc::new(v71)))?, f4(f1(Str::lit(" does not have type ")), f47(v2.clone())?))));
            let a330 = f10(v2);
            let a331 = 12u64;
            if nat_eq(a330, a331) {
                Ok(v77)
            } else {
                Ok(v78)
            }
        }
        Adt3::C22(_, _) => {
            Ok(Err::<(), Bytes>(f1(Str::lit("a closure has no literal form; build it with `closure`"))))
        }
        Adt3::C23 => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt3::C24(r348, r349) => {
            let v81 = (*r348).clone();
            let v82 = (*r349).clone();
            let v83: Adt0 = {
                let a332 = v3;
                let a333 = 2u64;
                if nat_eq(a332, a333) {
                    f44(v2.clone())
                } else {
                    v2.clone()
                }
            };
            let v84: Adt0 = {
                let a334 = v3;
                let a335 = 2u64;
                if nat_eq(a334, a335) {
                    f45(v2.clone())
                } else {
                    v2.clone()
                }
            };
            let v85: Result<(), Bytes> = f34(v0.clone(), v81, v83, 0u64)?;
            let v86: Result<(), Bytes> = f34(v0, v82, v84, v3)?;
            let a336 = v3;
            let a337 = 0u64;
            if nat_eq(a336, a337) {
                Ok(Ok::<(), Bytes>(()))
            } else {
                let a338 = v3;
                let a339 = 2u64;
                if if nat_eq(a338, a339) {
                    f6(v2)
                } else {
                    false
                } {
                    Ok(Ok::<(), Bytes>(()))
                } else {
                    let m340 = v85;
                    match m340 {
                        Ok(_) => {
                            Ok(v86)
                        }
                        Err(v88) => {
                            Ok(Err::<(), Bytes>(v88))
                        }
                    }
                }
            }
        }
    }
}

pub fn f35(v0: List<(u64, Adt0)>, v1: u64) -> Option<Adt0> {
    let m350 = v0;
    match m350.uncons() {
        None => {
            None::<Adt0>
        }
        Some((v2, v3)) => {
            let v4: Option<Adt0> = f35(v3, v1);
            let a352 = {
                let (h351, _) = v2;
                h351
            };
            let a353 = v1;
            if nat_eq(a352, a353) {
                Some({
                    let (_, h354) = v2;
                    h354
                })
            } else {
                v4
            }
        }
    }
}

pub fn f36(v0: u64, v1: Adt0, v2: List<(u64, Adt0)>) -> List<(u64, Adt0)> {
    List::cons((v0, v1), v2)
}

pub fn f37(v0: Adt7, v1: Adt0) -> R<Result<List<Adt5>, Bytes>> {
    let m355 = v1.clone();
    match m355 {
        Adt0::C0 => {
            Ok(Ok::<List<Adt5>, Bytes>(List::cons(Adt5::C11, List::<Adt5>::nil())))
        }
        Adt0::C1 => {
            Ok(Ok::<List<Adt5>, Bytes>(List::cons(Adt5::C9, List::cons(Adt5::C10, List::<Adt5>::nil()))))
        }
        Adt0::C2 => {
            Ok(Ok::<List<Adt5>, Bytes>(List::cons(Adt5::C6, List::cons(Adt5::C7, List::<Adt5>::nil()))))
        }
        Adt0::C3 => {
            Ok(Err::<List<Adt5>, Bytes>(f4(f1(Str::lit("a value of type ")), f4(f47(v1)?, f1(Str::lit(" cannot be matched"))))))
        }
        Adt0::C4(_) => {
            Ok(Err::<List<Adt5>, Bytes>(f4(f1(Str::lit("a value of type ")), f4(f47(v1)?, f1(Str::lit(" cannot be matched"))))))
        }
        Adt0::C5 => {
            Ok(Err::<List<Adt5>, Bytes>(f4(f1(Str::lit("a value of type ")), f4(f47(v1)?, f1(Str::lit(" cannot be matched"))))))
        }
        Adt0::C6 => {
            Ok(Err::<List<Adt5>, Bytes>(f4(f1(Str::lit("a value of type ")), f4(f47(v1)?, f1(Str::lit(" cannot be matched"))))))
        }
        Adt0::C7 => {
            Ok(Ok::<List<Adt5>, Bytes>(List::cons(Adt5::C12, List::cons(Adt5::C13, List::cons(Adt5::C14, List::<Adt5>::nil())))))
        }
        Adt0::C8(_) => {
            Ok(Ok::<List<Adt5>, Bytes>(List::cons(Adt5::C0, List::cons(Adt5::C1, List::<Adt5>::nil()))))
        }
        Adt0::C9(_, _) => {
            Ok(Ok::<List<Adt5>, Bytes>(List::cons(Adt5::C2, List::cons(Adt5::C3, List::<Adt5>::nil()))))
        }
        Adt0::C10(_) => {
            Ok(Ok::<List<Adt5>, Bytes>(List::cons(Adt5::C4, List::cons(Adt5::C5, List::<Adt5>::nil()))))
        }
        Adt0::C11(_, _) => {
            Ok(Ok::<List<Adt5>, Bytes>(List::cons(Adt5::C8, List::<Adt5>::nil())))
        }
        Adt0::C12(v9) => {
            let m356 = f26(v0, v9);
            match m356 {
                None => {
                    Ok(Err::<List<Adt5>, Bytes>(f27(v9)?))
                }
                Some(v10) => {
                    Ok(Ok::<List<Adt5>, Bytes>(f62(v10, 0u64)?))
                }
            }
        }
        Adt0::C13(_, _) => {
            Ok(Err::<List<Adt5>, Bytes>(f4(f1(Str::lit("a value of type ")), f4(f47(v1)?, f1(Str::lit(" cannot be matched"))))))
        }
        Adt0::C14 => {
            Ok(Err::<List<Adt5>, Bytes>(f4(f1(Str::lit("a value of type ")), f4(f47(v1)?, f1(Str::lit(" cannot be matched"))))))
        }
        Adt0::C15(_, _) => {
            Ok(Err::<List<Adt5>, Bytes>(f4(f1(Str::lit("a value of type ")), f4(f47(v1)?, f1(Str::lit(" cannot be matched"))))))
        }
    }
}

pub fn f38(v0: List<Adt5>, v1: List<Adt5>) -> Option<Adt5> {
    let m357 = v0;
    match m357.uncons() {
        None => {
            None::<Adt5>
        }
        Some((v2, v3)) => {
            let v4: Option<Adt5> = f38(v3, v1.clone());
            let v5: Option<Adt5> = v4;
            let v6: Option<Adt5> = Some(v2.clone());
            if f50(v1, v2) {
                v5
            } else {
                v6
            }
        }
    }
}

pub fn f39(v0: Adt5) -> R<Bytes> {
    let m358 = v0;
    match m358 {
        Adt5::C0 => {
            Ok(f1(Str::lit("None")))
        }
        Adt5::C1 => {
            Ok(f1(Str::lit("Some")))
        }
        Adt5::C2 => {
            Ok(f1(Str::lit("Ok")))
        }
        Adt5::C3 => {
            Ok(f1(Str::lit("Error")))
        }
        Adt5::C4 => {
            Ok(f1(Str::lit("Nil")))
        }
        Adt5::C5 => {
            Ok(f1(Str::lit("Cons")))
        }
        Adt5::C6 => {
            Ok(f1(Str::lit("Zero")))
        }
        Adt5::C7 => {
            Ok(f1(Str::lit("Succ")))
        }
        Adt5::C8 => {
            Ok(f1(Str::lit("Pair")))
        }
        Adt5::C9 => {
            Ok(f1(Str::lit("True")))
        }
        Adt5::C10 => {
            Ok(f1(Str::lit("False")))
        }
        Adt5::C11 => {
            Ok(f1(Str::lit("Unit")))
        }
        Adt5::C12 => {
            Ok(f1(Str::lit("Lt")))
        }
        Adt5::C13 => {
            Ok(f1(Str::lit("Eq")))
        }
        Adt5::C14 => {
            Ok(f1(Str::lit("Gt")))
        }
        Adt5::C15(v1) => {
            Ok(f4(f1(Str::lit("Adt { constructor: ")), f4(f7(v1)?, f1(Str::lit(" }")))))
        }
    }
}

pub fn f40(v0: Adt7, v1: Adt5, v2: Adt0) -> R<Result<Adt0, Bytes>> {
    let m359 = v1.clone();
    match m359 {
        Adt5::C0 => {
            let v3: Result<Adt0, Bytes> = Ok::<Adt0, Bytes>(Adt0::C14);
            let v4: Result<Adt0, Bytes> = Err::<Adt0, Bytes>(f4(f1(Str::lit("shape ")), f4(f39(v1)?, f4(f1(Str::lit(" does not belong to type ")), f47(v2.clone())?))));
            let a360 = f10(v2);
            let a361 = 8u64;
            if nat_eq(a360, a361) {
                Ok(v3)
            } else {
                Ok(v4)
            }
        }
        Adt5::C1 => {
            let v5: Result<Adt0, Bytes> = Ok::<Adt0, Bytes>(Adt0::C15(std::rc::Rc::new(f44(v2.clone())), std::rc::Rc::new(Adt0::C14)));
            let v6: Result<Adt0, Bytes> = Err::<Adt0, Bytes>(f4(f1(Str::lit("shape ")), f4(f39(v1)?, f4(f1(Str::lit(" does not belong to type ")), f47(v2.clone())?))));
            let a362 = f10(v2);
            let a363 = 8u64;
            if nat_eq(a362, a363) {
                Ok(v5)
            } else {
                Ok(v6)
            }
        }
        Adt5::C2 => {
            let v7: Result<Adt0, Bytes> = Ok::<Adt0, Bytes>(Adt0::C15(std::rc::Rc::new(f44(v2.clone())), std::rc::Rc::new(Adt0::C14)));
            let v8: Result<Adt0, Bytes> = Err::<Adt0, Bytes>(f4(f1(Str::lit("shape ")), f4(f39(v1)?, f4(f1(Str::lit(" does not belong to type ")), f47(v2.clone())?))));
            let a364 = f10(v2);
            let a365 = 9u64;
            if nat_eq(a364, a365) {
                Ok(v7)
            } else {
                Ok(v8)
            }
        }
        Adt5::C3 => {
            let v9: Result<Adt0, Bytes> = Ok::<Adt0, Bytes>(Adt0::C15(std::rc::Rc::new(f45(v2.clone())), std::rc::Rc::new(Adt0::C14)));
            let v10: Result<Adt0, Bytes> = Err::<Adt0, Bytes>(f4(f1(Str::lit("shape ")), f4(f39(v1)?, f4(f1(Str::lit(" does not belong to type ")), f47(v2.clone())?))));
            let a366 = f10(v2);
            let a367 = 9u64;
            if nat_eq(a366, a367) {
                Ok(v9)
            } else {
                Ok(v10)
            }
        }
        Adt5::C4 => {
            let v11: Result<Adt0, Bytes> = Ok::<Adt0, Bytes>(Adt0::C14);
            let v12: Result<Adt0, Bytes> = Err::<Adt0, Bytes>(f4(f1(Str::lit("shape ")), f4(f39(v1)?, f4(f1(Str::lit(" does not belong to type ")), f47(v2.clone())?))));
            let a368 = f10(v2);
            let a369 = 10u64;
            if nat_eq(a368, a369) {
                Ok(v11)
            } else {
                Ok(v12)
            }
        }
        Adt5::C5 => {
            let v13: Result<Adt0, Bytes> = Ok::<Adt0, Bytes>(Adt0::C15(std::rc::Rc::new(f44(v2.clone())), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(v2.clone()), std::rc::Rc::new(Adt0::C14)))));
            let v14: Result<Adt0, Bytes> = Err::<Adt0, Bytes>(f4(f1(Str::lit("shape ")), f4(f39(v1)?, f4(f1(Str::lit(" does not belong to type ")), f47(v2.clone())?))));
            let a370 = f10(v2);
            let a371 = 10u64;
            if nat_eq(a370, a371) {
                Ok(v13)
            } else {
                Ok(v14)
            }
        }
        Adt5::C6 => {
            let v15: Result<Adt0, Bytes> = Ok::<Adt0, Bytes>(Adt0::C14);
            let v16: Result<Adt0, Bytes> = Err::<Adt0, Bytes>(f4(f1(Str::lit("shape ")), f4(f39(v1)?, f4(f1(Str::lit(" does not belong to type ")), f47(v2.clone())?))));
            let a372 = f10(v2);
            let a373 = 2u64;
            if nat_eq(a372, a373) {
                Ok(v15)
            } else {
                Ok(v16)
            }
        }
        Adt5::C7 => {
            let v17: Result<Adt0, Bytes> = Ok::<Adt0, Bytes>(Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C14)));
            let v18: Result<Adt0, Bytes> = Err::<Adt0, Bytes>(f4(f1(Str::lit("shape ")), f4(f39(v1)?, f4(f1(Str::lit(" does not belong to type ")), f47(v2.clone())?))));
            let a374 = f10(v2);
            let a375 = 2u64;
            if nat_eq(a374, a375) {
                Ok(v17)
            } else {
                Ok(v18)
            }
        }
        Adt5::C8 => {
            let v19: Result<Adt0, Bytes> = Ok::<Adt0, Bytes>(Adt0::C15(std::rc::Rc::new(f44(v2.clone())), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(f45(v2.clone())), std::rc::Rc::new(Adt0::C14)))));
            let v20: Result<Adt0, Bytes> = Err::<Adt0, Bytes>(f4(f1(Str::lit("shape ")), f4(f39(v1)?, f4(f1(Str::lit(" does not belong to type ")), f47(v2.clone())?))));
            let a376 = f10(v2);
            let a377 = 11u64;
            if nat_eq(a376, a377) {
                Ok(v19)
            } else {
                Ok(v20)
            }
        }
        Adt5::C9 => {
            let v21: Result<Adt0, Bytes> = Ok::<Adt0, Bytes>(Adt0::C14);
            let v22: Result<Adt0, Bytes> = Err::<Adt0, Bytes>(f4(f1(Str::lit("shape ")), f4(f39(v1)?, f4(f1(Str::lit(" does not belong to type ")), f47(v2.clone())?))));
            let a378 = f10(v2);
            let a379 = 1u64;
            if nat_eq(a378, a379) {
                Ok(v21)
            } else {
                Ok(v22)
            }
        }
        Adt5::C10 => {
            let v23: Result<Adt0, Bytes> = Ok::<Adt0, Bytes>(Adt0::C14);
            let v24: Result<Adt0, Bytes> = Err::<Adt0, Bytes>(f4(f1(Str::lit("shape ")), f4(f39(v1)?, f4(f1(Str::lit(" does not belong to type ")), f47(v2.clone())?))));
            let a380 = f10(v2);
            let a381 = 1u64;
            if nat_eq(a380, a381) {
                Ok(v23)
            } else {
                Ok(v24)
            }
        }
        Adt5::C11 => {
            let v25: Result<Adt0, Bytes> = Ok::<Adt0, Bytes>(Adt0::C14);
            let v26: Result<Adt0, Bytes> = Err::<Adt0, Bytes>(f4(f1(Str::lit("shape ")), f4(f39(v1)?, f4(f1(Str::lit(" does not belong to type ")), f47(v2.clone())?))));
            let a382 = f10(v2);
            let a383 = 0u64;
            if nat_eq(a382, a383) {
                Ok(v25)
            } else {
                Ok(v26)
            }
        }
        Adt5::C12 => {
            let v27: Result<Adt0, Bytes> = Ok::<Adt0, Bytes>(Adt0::C14);
            let v28: Result<Adt0, Bytes> = Err::<Adt0, Bytes>(f4(f1(Str::lit("shape ")), f4(f39(v1)?, f4(f1(Str::lit(" does not belong to type ")), f47(v2.clone())?))));
            let a384 = f10(v2);
            let a385 = 7u64;
            if nat_eq(a384, a385) {
                Ok(v27)
            } else {
                Ok(v28)
            }
        }
        Adt5::C13 => {
            let v29: Result<Adt0, Bytes> = Ok::<Adt0, Bytes>(Adt0::C14);
            let v30: Result<Adt0, Bytes> = Err::<Adt0, Bytes>(f4(f1(Str::lit("shape ")), f4(f39(v1)?, f4(f1(Str::lit(" does not belong to type ")), f47(v2.clone())?))));
            let a386 = f10(v2);
            let a387 = 7u64;
            if nat_eq(a386, a387) {
                Ok(v29)
            } else {
                Ok(v30)
            }
        }
        Adt5::C14 => {
            let v31: Result<Adt0, Bytes> = Ok::<Adt0, Bytes>(Adt0::C14);
            let v32: Result<Adt0, Bytes> = Err::<Adt0, Bytes>(f4(f1(Str::lit("shape ")), f4(f39(v1)?, f4(f1(Str::lit(" does not belong to type ")), f47(v2.clone())?))));
            let a388 = f10(v2);
            let a389 = 7u64;
            if nat_eq(a388, a389) {
                Ok(v31)
            } else {
                Ok(v32)
            }
        }
        Adt5::C15(v33) => {
            let v34: Result<Adt0, Bytes> = f60(v0, f48(v2.clone()), v33)?;
            let v35: Result<Adt0, Bytes> = v34;
            let v36: Result<Adt0, Bytes> = Err::<Adt0, Bytes>(f4(f1(Str::lit("shape ")), f4(f39(v1)?, f4(f1(Str::lit(" does not belong to type ")), f47(v2.clone())?))));
            let a390 = f10(v2);
            let a391 = 12u64;
            if nat_eq(a390, a391) {
                Ok(v35)
            } else {
                Ok(v36)
            }
        }
    }
}

pub fn f41(v0: Adt2) -> R<u64> {
    let m392 = v0;
    match m392 {
        Adt2::C0(_, _) => {
            Ok(0u64)
        }
        Adt2::C1(_) => {
            Ok(0u64)
        }
        Adt2::C2(_, _, _, _) => {
            Ok(0u64)
        }
        Adt2::C3(_, _, _) => {
            Ok(0u64)
        }
        Adt2::C4(_, _, _) => {
            Ok(0u64)
        }
        Adt2::C5(_, _, _) => {
            Ok(0u64)
        }
        Adt2::C6(_, _) => {
            Ok(0u64)
        }
        Adt2::C7(_, _) => {
            Ok(0u64)
        }
        Adt2::C8(_, _) => {
            Ok(0u64)
        }
        Adt2::C9(_, _) => {
            Ok(0u64)
        }
        Adt2::C10(_) => {
            Ok(0u64)
        }
        Adt2::C11(_) => {
            Ok(0u64)
        }
        Adt2::C12(_, _) => {
            Ok(0u64)
        }
        Adt2::C13 => {
            Ok(0u64)
        }
        Adt2::C14(_, r395) => {
            let v33 = (*r395).clone();
            let v34: u64 = f41(v33)?;
            let a393 = 1u64;
            let a394 = v34;
            nat_add(a393, a394)
        }
        Adt2::C15(_, _, _) => {
            Ok(0u64)
        }
    }
}

pub fn f42(v0: u64) -> R<Bytes> {
    Ok(f4(f1(Str::lit("function ")), f4(f7(v0)?, f1(Str::lit(" is not declared")))))
}

pub fn f43(v0: Adt0, v1: Adt0) -> Adt0 {
    Adt0::C13(std::rc::Rc::new(v0), std::rc::Rc::new(v1))
}

pub fn f44(v0: Adt0) -> Adt0 {
    let m396 = f63(v0);
    match m396 {
        None => {
            Adt0::C0
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f45(v0: Adt0) -> Adt0 {
    let m397 = f64(v0);
    match m397 {
        None => {
            Adt0::C0
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f46(v0: Adt6, v1: Adt0) -> R<Result<Adt0, Bytes>> {
    let v2: Adt0 = f44(v1.clone());
    let v3: Adt0 = if f6(v1.clone()) {
        Adt0::C0
    } else {
        v2
    };
    let m398 = v0.clone();
    match m398 {
        Adt6::C0 => {
            f65(v0, Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C14)))), v1, Adt0::C2)
        }
        Adt6::C1 => {
            f65(v0, Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C14)))), v1, Adt0::C2)
        }
        Adt6::C2 => {
            f65(v0, Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C14)))), v1, Adt0::C2)
        }
        Adt6::C3 => {
            f65(v0, Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C14)))))), v1, Adt0::C2)
        }
        Adt6::C4 => {
            f65(v0, Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C14)))))), v1, Adt0::C2)
        }
        Adt6::C5 => {
            f65(v0, Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C14)))), v1, Adt0::C1)
        }
        Adt6::C6 => {
            f65(v0, Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C14)))), v1, Adt0::C1)
        }
        Adt6::C7 => {
            f65(v0, Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C14)))), v1, Adt0::C1)
        }
        Adt6::C8 => {
            f65(v0, Adt0::C15(std::rc::Rc::new(Adt0::C3), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C3), std::rc::Rc::new(Adt0::C14)))), v1, Adt0::C3)
        }
        Adt6::C9 => {
            f65(v0, Adt0::C15(std::rc::Rc::new(Adt0::C3), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C3), std::rc::Rc::new(Adt0::C14)))), v1, Adt0::C3)
        }
        Adt6::C10 => {
            f65(v0, Adt0::C15(std::rc::Rc::new(Adt0::C3), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C3), std::rc::Rc::new(Adt0::C14)))), v1, Adt0::C3)
        }
        Adt6::C11 => {
            f65(v0, Adt0::C15(std::rc::Rc::new(Adt0::C3), std::rc::Rc::new(Adt0::C14)), v1, Adt0::C3)
        }
        Adt6::C12 => {
            f65(v0, Adt0::C15(std::rc::Rc::new(Adt0::C3), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C3), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C3), std::rc::Rc::new(Adt0::C14)))))), v1, Adt0::C3)
        }
        Adt6::C13 => {
            f65(v0, Adt0::C15(std::rc::Rc::new(Adt0::C3), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C3), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C3), std::rc::Rc::new(Adt0::C14)))))), v1, Adt0::C3)
        }
        Adt6::C14 => {
            let v4: Result<Adt0, Bytes> = f65(v0.clone(), Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C14)))), v1.clone(), Adt0::C8(std::rc::Rc::new(v3.clone())))?;
            let v5: Result<Adt0, Bytes> = f66(v0, v1)?;
            let a399 = f10(v3);
            let a400 = 4u64;
            if nat_eq(a399, a400) {
                Ok(v4)
            } else {
                Ok(v5)
            }
        }
        Adt6::C15 => {
            let v6: Result<Adt0, Bytes> = f65(v0.clone(), Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C14)))), v1.clone(), Adt0::C8(std::rc::Rc::new(v3.clone())))?;
            let v7: Result<Adt0, Bytes> = f66(v0, v1)?;
            let a401 = f10(v3);
            let a402 = 4u64;
            if nat_eq(a401, a402) {
                Ok(v6)
            } else {
                Ok(v7)
            }
        }
        Adt6::C16 => {
            let v8: Result<Adt0, Bytes> = f65(v0.clone(), Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C14)))), v1.clone(), Adt0::C8(std::rc::Rc::new(v3.clone())))?;
            let v9: Result<Adt0, Bytes> = f66(v0, v1)?;
            let a403 = f10(v3);
            let a404 = 4u64;
            if nat_eq(a403, a404) {
                Ok(v8)
            } else {
                Ok(v9)
            }
        }
        Adt6::C17 => {
            let v12: Result<Adt0, Bytes> = f65(v0.clone(), Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C14)), v1.clone(), Adt0::C8(std::rc::Rc::new(v3.clone())))?;
            let v13: Result<Adt0, Bytes> = f66(v0, v1)?;
            let a407 = f10(v3.clone());
            let a408 = 4u64;
            if if nat_eq(a407, a408) {
                f67(f58(v3))
            } else {
                false
            } {
                Ok(v12)
            } else {
                Ok(v13)
            }
        }
        Adt6::C18 => {
            let v10: Result<Adt0, Bytes> = f65(v0.clone(), Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C14)))), v1.clone(), Adt0::C8(std::rc::Rc::new(v3.clone())))?;
            let v11: Result<Adt0, Bytes> = f66(v0, v1)?;
            let a405 = f10(v3);
            let a406 = 4u64;
            if nat_eq(a405, a406) {
                Ok(v10)
            } else {
                Ok(v11)
            }
        }
        Adt6::C19 => {
            let v14: Result<Adt0, Bytes> = f65(v0.clone(), Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C14)))), v1.clone(), v3.clone())?;
            let v15: Result<Adt0, Bytes> = f66(v0, v1)?;
            let a409 = f10(v3);
            let a410 = 4u64;
            if nat_eq(a409, a410) {
                Ok(v14)
            } else {
                Ok(v15)
            }
        }
        Adt6::C20 => {
            let v16: Result<Adt0, Bytes> = f65(v0.clone(), Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C14)))), v1.clone(), v3.clone())?;
            let v17: Result<Adt0, Bytes> = f66(v0, v1)?;
            let a411 = f10(v3);
            let a412 = 4u64;
            if nat_eq(a411, a412) {
                Ok(v16)
            } else {
                Ok(v17)
            }
        }
        Adt6::C21 => {
            let v18: Result<Adt0, Bytes> = f65(v0.clone(), Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C14)))), v1.clone(), v3.clone())?;
            let v19: Result<Adt0, Bytes> = f66(v0, v1)?;
            let a413 = f10(v3);
            let a414 = 4u64;
            if nat_eq(a413, a414) {
                Ok(v18)
            } else {
                Ok(v19)
            }
        }
        Adt6::C22 => {
            let v20: Result<Adt0, Bytes> = f65(v0.clone(), Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C14)), v1.clone(), v3.clone())?;
            let v21: Result<Adt0, Bytes> = f66(v0, v1)?;
            let a415 = f10(v3);
            let a416 = 4u64;
            if nat_eq(a415, a416) {
                Ok(v20)
            } else {
                Ok(v21)
            }
        }
        Adt6::C23 => {
            let v22: Result<Adt0, Bytes> = f65(v0.clone(), Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C4(Adt1::C2)), std::rc::Rc::new(Adt0::C14)))), v1.clone(), Adt0::C8(std::rc::Rc::new(v3.clone())))?;
            let v23: Result<Adt0, Bytes> = f66(v0, v1)?;
            let a417 = f10(v3);
            let a418 = 4u64;
            if nat_eq(a417, a418) {
                Ok(v22)
            } else {
                Ok(v23)
            }
        }
        Adt6::C24 => {
            let v24: Result<Adt0, Bytes> = f65(v0.clone(), Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C4(Adt1::C2)), std::rc::Rc::new(Adt0::C14)))), v1.clone(), Adt0::C8(std::rc::Rc::new(v3.clone())))?;
            let v25: Result<Adt0, Bytes> = f66(v0, v1)?;
            let a419 = f10(v3);
            let a420 = 4u64;
            if nat_eq(a419, a420) {
                Ok(v24)
            } else {
                Ok(v25)
            }
        }
        Adt6::C25 => {
            let v26: Result<Adt0, Bytes> = f65(v0.clone(), Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C14)))), v1.clone(), Adt0::C1)?;
            let v27: Result<Adt0, Bytes> = f66(v0, v1)?;
            let a421 = f10(v3.clone());
            let a422 = 2u64;
            if if nat_eq(a421, a422) {
                true
            } else {
                let a423 = f10(v3.clone());
                let a424 = 1u64;
                if nat_eq(a423, a424) {
                    true
                } else {
                    let a425 = f10(v3.clone());
                    let a426 = 4u64;
                    if nat_eq(a425, a426) {
                        true
                    } else {
                        let a427 = f10(v3.clone());
                        let a428 = 5u64;
                        if nat_eq(a427, a428) {
                            true
                        } else {
                            let a429 = f10(v3.clone());
                            let a430 = 6u64;
                            if nat_eq(a429, a430) {
                                true
                            } else {
                                let a431 = f10(v3);
                                let a432 = 7u64;
                                nat_eq(a431, a432)
                            }
                        }
                    }
                }
            } {
                Ok(v26)
            } else {
                Ok(v27)
            }
        }
        Adt6::C26 => {
            f65(v0, Adt0::C15(std::rc::Rc::new(Adt0::C1), std::rc::Rc::new(Adt0::C14)), v1, Adt0::C1)
        }
        Adt6::C27 => {
            f65(v0, Adt0::C15(std::rc::Rc::new(Adt0::C1), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C1), std::rc::Rc::new(Adt0::C14)))), v1, Adt0::C1)
        }
        Adt6::C28 => {
            f65(v0, Adt0::C15(std::rc::Rc::new(Adt0::C1), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C1), std::rc::Rc::new(Adt0::C14)))), v1, Adt0::C1)
        }
        Adt6::C29 => {
            let v28: Result<Adt0, Bytes> = f65(v0.clone(), Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C14)))), v1.clone(), v3.clone())?;
            let v29: Result<Adt0, Bytes> = f66(v0, v1)?;
            let a433 = f10(v3.clone());
            let a434 = 10u64;
            if if nat_eq(a433, a434) {
                true
            } else {
                let a435 = f10(v3);
                let a436 = 6u64;
                nat_eq(a435, a436)
            } {
                Ok(v28)
            } else {
                Ok(v29)
            }
        }
        Adt6::C30 => {
            let v30: Result<Adt0, Bytes> = f65(v0.clone(), Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C14)), v1.clone(), Adt0::C2)?;
            let v31: Result<Adt0, Bytes> = f66(v0, v1)?;
            let a437 = f10(v3.clone());
            let a438 = 10u64;
            if if nat_eq(a437, a438) {
                true
            } else {
                let a439 = f10(v3.clone());
                let a440 = 6u64;
                if nat_eq(a439, a440) {
                    true
                } else {
                    let a441 = f10(v3);
                    let a442 = 5u64;
                    nat_eq(a441, a442)
                }
            } {
                Ok(v30)
            } else {
                Ok(v31)
            }
        }
        Adt6::C31 => {
            let v32: Result<Adt0, Bytes> = f65(v0.clone(), Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C14)))), v1.clone(), Adt0::C8(std::rc::Rc::new(f44(v3.clone()))))?;
            let v33: Result<Adt0, Bytes> = f65(v0, Adt0::C15(std::rc::Rc::new(Adt0::C6), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C14)))), v1, Adt0::C8(std::rc::Rc::new(Adt0::C4(Adt1::C0))))?;
            let a443 = f10(v3.clone());
            let a444 = 10u64;
            if nat_eq(a443, a444) {
                Ok(v32)
            } else {
                let a445 = f10(v3.clone());
                let a446 = 6u64;
                if nat_eq(a445, a446) {
                    Ok(v33)
                } else {
                    Ok(Err::<Adt0, Bytes>(f4(f1(Str::lit("primitive Index of ")), f47(v3)?)))
                }
            }
        }
        Adt6::C32 => {
            let v34: Result<Adt0, Bytes> = f65(v0.clone(), Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C2), std::rc::Rc::new(Adt0::C14)))))), v1.clone(), Adt0::C8(std::rc::Rc::new(v3.clone())))?;
            let v35: Result<Adt0, Bytes> = f66(v0, v1)?;
            let a447 = f10(v3.clone());
            let a448 = 10u64;
            if if nat_eq(a447, a448) {
                true
            } else {
                let a449 = f10(v3);
                let a450 = 6u64;
                nat_eq(a449, a450)
            } {
                Ok(v34)
            } else {
                Ok(v35)
            }
        }
        Adt6::C33 => {
            f65(v0, Adt0::C15(std::rc::Rc::new(Adt0::C5), std::rc::Rc::new(Adt0::C14)), v1, Adt0::C6)
        }
        Adt6::C34 => {
            f65(v0, Adt0::C15(std::rc::Rc::new(Adt0::C6), std::rc::Rc::new(Adt0::C14)), v1, Adt0::C8(std::rc::Rc::new(Adt0::C5)))
        }
        Adt6::C35 => {
            f65(v0, Adt0::C15(std::rc::Rc::new(Adt0::C6), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C6), std::rc::Rc::new(Adt0::C14)))), v1, Adt0::C7)
        }
        Adt6::C36 => {
            f65(v0, Adt0::C15(std::rc::Rc::new(Adt0::C5), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C5), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C4(Adt1::C2)), std::rc::Rc::new(Adt0::C14)))))), v1, Adt0::C8(std::rc::Rc::new(Adt0::C10(std::rc::Rc::new(Adt0::C5)))))
        }
        Adt6::C37 => {
            f65(v0, Adt0::C15(std::rc::Rc::new(Adt0::C10(std::rc::Rc::new(Adt0::C5))), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(Adt0::C5), std::rc::Rc::new(Adt0::C14)))), v1, Adt0::C5)
        }
        Adt6::C38 => {
            let v36: Result<Adt0, Bytes> = f65(v0.clone(), Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C14)), v1.clone(), Adt0::C5)?;
            let v37: Result<Adt0, Bytes> = f66(v0, v1)?;
            let a451 = f10(v3.clone());
            let a452 = 3u64;
            if if nat_eq(a451, a452) {
                true
            } else {
                let a453 = f10(v3);
                let a454 = 4u64;
                nat_eq(a453, a454)
            } {
                Ok(v36)
            } else {
                Ok(v37)
            }
        }
        Adt6::C39 => {
            let v38: Result<Adt0, Bytes> = f65(v0.clone(), Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C14)))), v1.clone(), Adt0::C7)?;
            let v39: Result<Adt0, Bytes> = f66(v0, v1)?;
            if f68(v3) {
                Ok(v38)
            } else {
                Ok(v39)
            }
        }
        Adt6::C40(v40) => {
            let v41: Result<Adt0, Bytes> = f65(v0.clone(), Adt0::C15(std::rc::Rc::new(v3.clone()), std::rc::Rc::new(Adt0::C14)), v1.clone(), Adt0::C8(std::rc::Rc::new(Adt0::C4(v40))))?;
            let v42: Result<Adt0, Bytes> = f66(v0, v1)?;
            let a455 = f10(v3.clone());
            let a456 = 3u64;
            if if nat_eq(a455, a456) {
                true
            } else {
                let a457 = f10(v3);
                let a458 = 4u64;
                nat_eq(a457, a458)
            } {
                Ok(v41)
            } else {
                Ok(v42)
            }
        }
        Adt6::C41(v43) => {
            let v44: u64 = f10(v43.clone());
            let v45: Result<Adt0, Bytes> = f65(v0.clone(), Adt0::C15(std::rc::Rc::new(Adt0::C5), std::rc::Rc::new(Adt0::C14)), v1.clone(), Adt0::C8(std::rc::Rc::new(v43)))?;
            let v46: Result<Adt0, Bytes> = f66(v0, v1)?;
            let a459 = v44;
            let a460 = 3u64;
            if if nat_eq(a459, a460) {
                true
            } else {
                let a461 = v44;
                let a462 = 4u64;
                nat_eq(a461, a462)
            } {
                Ok(v45)
            } else {
                Ok(v46)
            }
        }
    }
}

pub fn f47(v0: Adt0) -> R<Bytes> {
    let m463 = v0;
    match m463 {
        Adt0::C0 => {
            Ok(f1(Str::lit("Unit")))
        }
        Adt0::C1 => {
            Ok(f1(Str::lit("Bool")))
        }
        Adt0::C2 => {
            Ok(f1(Str::lit("Nat")))
        }
        Adt0::C3 => {
            Ok(f1(Str::lit("Int")))
        }
        Adt0::C4(v1) => {
            Ok(f4(f1(Str::lit("Fixed { width: ")), f4(f69(v1), f1(Str::lit(" }")))))
        }
        Adt0::C5 => {
            Ok(f1(Str::lit("String")))
        }
        Adt0::C6 => {
            Ok(f1(Str::lit("Bytes")))
        }
        Adt0::C7 => {
            Ok(f1(Str::lit("Ordering")))
        }
        Adt0::C8(r464) => {
            let v2 = (*r464).clone();
            Ok(f4(f1(Str::lit("Option { value: ")), f4(f47(v2)?, f1(Str::lit(" }")))))
        }
        Adt0::C9(r465, r466) => {
            let v3 = (*r465).clone();
            let v4 = (*r466).clone();
            Ok(f4(f1(Str::lit("Result { ok: ")), f4(f47(v3)?, f4(f1(Str::lit(", error: ")), f4(f47(v4)?, f1(Str::lit(" }")))))))
        }
        Adt0::C10(r467) => {
            let v5 = (*r467).clone();
            Ok(f4(f1(Str::lit("List { element: ")), f4(f47(v5)?, f1(Str::lit(" }")))))
        }
        Adt0::C11(r468, r469) => {
            let v6 = (*r468).clone();
            let v7 = (*r469).clone();
            Ok(f4(f1(Str::lit("Pair { left: ")), f4(f47(v6)?, f4(f1(Str::lit(", right: ")), f4(f47(v7)?, f1(Str::lit(" }")))))))
        }
        Adt0::C12(v8) => {
            Ok(f4(f1(Str::lit("Adt { index: ")), f4(f7(v8)?, f1(Str::lit(" }")))))
        }
        Adt0::C13(r470, r471) => {
            let v9 = (*r470).clone();
            let v10 = (*r471).clone();
            Ok(f4(f1(Str::lit("Fn { parameters: [")), f4(f47(v9)?, f4(f1(Str::lit("], result: ")), f4(f47(v10)?, f1(Str::lit(" }")))))))
        }
        Adt0::C14 => {
            Ok(f1(Str::lit("")))
        }
        Adt0::C15(r472, r473) => {
            let v11 = (*r472).clone();
            let v12 = (*r473).clone();
            let v13: Bytes = f47(v11)?;
            let v14: Bytes = f47(v12.clone())?;
            if f6(v12) {
                Ok(v13)
            } else {
                Ok(f4(v13, f4(f1(Str::lit(", ")), v14)))
            }
        }
    }
}

pub fn f48(v0: Adt0) -> u64 {
    let m474 = v0;
    match m474 {
        Adt0::C0 => {
            0u64
        }
        Adt0::C1 => {
            0u64
        }
        Adt0::C2 => {
            0u64
        }
        Adt0::C3 => {
            0u64
        }
        Adt0::C4(_) => {
            0u64
        }
        Adt0::C5 => {
            0u64
        }
        Adt0::C6 => {
            0u64
        }
        Adt0::C7 => {
            0u64
        }
        Adt0::C8(_) => {
            0u64
        }
        Adt0::C9(_, _) => {
            0u64
        }
        Adt0::C10(_) => {
            0u64
        }
        Adt0::C11(_, _) => {
            0u64
        }
        Adt0::C12(v8) => {
            v8
        }
        Adt0::C13(_, _) => {
            0u64
        }
        Adt0::C14 => {
            0u64
        }
        Adt0::C15(_, _) => {
            0u64
        }
    }
}

pub fn f49(v0: Adt0, v1: Adt0) -> Adt0 {
    Adt0::C15(std::rc::Rc::new(v0), std::rc::Rc::new(v1))
}

pub fn f50(v0: List<Adt5>, v1: Adt5) -> bool {
    let m475 = v0;
    match m475.uncons() {
        None => {
            false
        }
        Some((v2, v3)) => {
            let v4: bool = f70(v2, v1.clone());
            let v5: bool = f50(v3, v1);
            if v4 {
                true
            } else {
                v5
            }
        }
    }
}

pub fn f51(v0: Adt5, v1: List<Adt5>) -> List<Adt5> {
    List::cons(v0, v1)
}

pub fn f52(v0: Adt0, v1: Adt0) -> bool {
    let m476 = v0;
    match m476 {
        Adt0::C0 => {
            let a477 = f10(v1);
            let a478 = 0u64;
            nat_eq(a477, a478)
        }
        Adt0::C1 => {
            let a479 = f10(v1);
            let a480 = 1u64;
            nat_eq(a479, a480)
        }
        Adt0::C2 => {
            let a481 = f10(v1);
            let a482 = 2u64;
            nat_eq(a481, a482)
        }
        Adt0::C3 => {
            let a483 = f10(v1);
            let a484 = 3u64;
            nat_eq(a483, a484)
        }
        Adt0::C4(v2) => {
            let a493 = f10(v1.clone());
            let a494 = 4u64;
            if nat_eq(a493, a494) {
                let a495 = f57(v2);
                let a496 = f57(f58(v1));
                nat_eq(a495, a496)
            } else {
                false
            }
        }
        Adt0::C5 => {
            let a485 = f10(v1);
            let a486 = 5u64;
            nat_eq(a485, a486)
        }
        Adt0::C6 => {
            let a487 = f10(v1);
            let a488 = 6u64;
            nat_eq(a487, a488)
        }
        Adt0::C7 => {
            let a489 = f10(v1);
            let a490 = 7u64;
            nat_eq(a489, a490)
        }
        Adt0::C8(r513) => {
            let v3 = (*r513).clone();
            let v4: bool = f52(v3, f44(v1.clone()));
            let a497 = f10(v1);
            let a498 = 8u64;
            if nat_eq(a497, a498) {
                v4
            } else {
                false
            }
        }
        Adt0::C9(r514, r515) => {
            let v5 = (*r514).clone();
            let v6 = (*r515).clone();
            let v7: bool = f52(v5, f44(v1.clone()));
            let v8: bool = f52(v6, f45(v1.clone()));
            let a499 = f10(v1);
            let a500 = 9u64;
            if nat_eq(a499, a500) {
                if v7 {
                    v8
                } else {
                    false
                }
            } else {
                false
            }
        }
        Adt0::C10(r516) => {
            let v9 = (*r516).clone();
            let v10: bool = f52(v9, f44(v1.clone()));
            let a501 = f10(v1);
            let a502 = 10u64;
            if nat_eq(a501, a502) {
                v10
            } else {
                false
            }
        }
        Adt0::C11(r517, r518) => {
            let v11 = (*r517).clone();
            let v12 = (*r518).clone();
            let v13: bool = f52(v11, f44(v1.clone()));
            let v14: bool = f52(v12, f45(v1.clone()));
            let a503 = f10(v1);
            let a504 = 11u64;
            if nat_eq(a503, a504) {
                if v13 {
                    v14
                } else {
                    false
                }
            } else {
                false
            }
        }
        Adt0::C12(v15) => {
            let a505 = f10(v1.clone());
            let a506 = 12u64;
            if nat_eq(a505, a506) {
                let a507 = v15;
                let a508 = f48(v1);
                nat_eq(a507, a508)
            } else {
                false
            }
        }
        Adt0::C13(r519, r520) => {
            let v16 = (*r519).clone();
            let v17 = (*r520).clone();
            let v18: bool = f52(v16, f44(v1.clone()));
            let v19: bool = f52(v17, f45(v1.clone()));
            let a509 = f10(v1);
            let a510 = 13u64;
            if nat_eq(a509, a510) {
                if v18 {
                    v19
                } else {
                    false
                }
            } else {
                false
            }
        }
        Adt0::C14 => {
            let a491 = f10(v1);
            let a492 = 14u64;
            nat_eq(a491, a492)
        }
        Adt0::C15(r521, r522) => {
            let v20 = (*r521).clone();
            let v21 = (*r522).clone();
            let v22: bool = f52(v20, f44(v1.clone()));
            let v23: bool = f52(v21, f45(v1.clone()));
            let a511 = f10(v1);
            let a512 = 15u64;
            if nat_eq(a511, a512) {
                if v22 {
                    v23
                } else {
                    false
                }
            } else {
                false
            }
        }
    }
}

pub fn f53(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f71(v0, v1, 48u8) {
        Ok(Some({
            let a523 = v1;
            let a524 = 1u64;
            nat_add(a523, a524)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f54(v0: Adt3) -> R<Bytes> {
    let m525 = v0;
    match m525 {
        Adt3::C0 => {
            Ok(f1(Str::lit("Unit")))
        }
        Adt3::C1(v1) => {
            Ok(f4(f1(Str::lit("Bool { value: ")), f4(f72(v1), f1(Str::lit(" }")))))
        }
        Adt3::C2(v2) => {
            Ok(f4(f1(Str::lit("Nat { value: ")), f4(f73(v2)?, f1(Str::lit(" }")))))
        }
        Adt3::C3(v3) => {
            Ok(f4(f1(Str::lit("Int { value: ")), f4(f73(v3)?, f1(Str::lit(" }")))))
        }
        Adt3::C4(v4) => {
            Ok(f4(f1(Str::lit("U8 { value: ")), f4(f73(v4)?, f1(Str::lit(" }")))))
        }
        Adt3::C5(v5) => {
            Ok(f4(f1(Str::lit("U16 { value: ")), f4(f73(v5)?, f1(Str::lit(" }")))))
        }
        Adt3::C6(v6) => {
            Ok(f4(f1(Str::lit("U32 { value: ")), f4(f73(v6)?, f1(Str::lit(" }")))))
        }
        Adt3::C7(v7) => {
            Ok(f4(f1(Str::lit("U64 { value: ")), f4(f73(v7)?, f1(Str::lit(" }")))))
        }
        Adt3::C8(v8) => {
            Ok(f4(f1(Str::lit("I8 { value: ")), f4(f73(v8)?, f1(Str::lit(" }")))))
        }
        Adt3::C9(v9) => {
            Ok(f4(f1(Str::lit("I16 { value: ")), f4(f73(v9)?, f1(Str::lit(" }")))))
        }
        Adt3::C10(v10) => {
            Ok(f4(f1(Str::lit("I32 { value: ")), f4(f73(v10)?, f1(Str::lit(" }")))))
        }
        Adt3::C11(v11) => {
            Ok(f4(f1(Str::lit("I64 { value: ")), f4(f73(v11)?, f1(Str::lit(" }")))))
        }
        Adt3::C12(v12) => {
            Ok(f4(f1(Str::lit("String { value: ")), f4(f73(v12)?, f1(Str::lit(" }")))))
        }
        Adt3::C13(v13) => {
            Ok(f4(f1(Str::lit("Bytes { hex: ")), f4(f73(v13)?, f1(Str::lit(" }")))))
        }
        Adt3::C14(v14) => {
            Ok(f4(f1(Str::lit("Ordering { value: ")), f4(f74(v14), f1(Str::lit(" }")))))
        }
        Adt3::C15 => {
            Ok(f1(Str::lit("None")))
        }
        Adt3::C16(r526) => {
            let v15 = (*r526).clone();
            Ok(f4(f1(Str::lit("Some { value: ")), f4(f54(v15)?, f1(Str::lit(" }")))))
        }
        Adt3::C17(r527) => {
            let v16 = (*r527).clone();
            Ok(f4(f1(Str::lit("Ok { value: ")), f4(f54(v16)?, f1(Str::lit(" }")))))
        }
        Adt3::C18(r528) => {
            let v17 = (*r528).clone();
            Ok(f4(f1(Str::lit("Error { value: ")), f4(f54(v17)?, f1(Str::lit(" }")))))
        }
        Adt3::C19(r529) => {
            let v18 = (*r529).clone();
            Ok(f4(f1(Str::lit("List { items: [")), f4(f54(v18)?, f1(Str::lit("] }")))))
        }
        Adt3::C20(r530, r531) => {
            let v19 = (*r530).clone();
            let v20 = (*r531).clone();
            Ok(f4(f1(Str::lit("Pair { left: ")), f4(f54(v19)?, f4(f1(Str::lit(", right: ")), f4(f54(v20)?, f1(Str::lit(" }")))))))
        }
        Adt3::C21(v21, r532) => {
            let v22 = (*r532).clone();
            Ok(f4(f1(Str::lit("Adt { constructor: ")), f4(f7(v21)?, f4(f1(Str::lit(", fields: [")), f4(f54(v22)?, f1(Str::lit("] }")))))))
        }
        Adt3::C22(v23, r533) => {
            let v24 = (*r533).clone();
            Ok(f4(f1(Str::lit("Closure { function: ")), f4(f7(v23)?, f4(f1(Str::lit(", captures: [")), f4(f54(v24)?, f1(Str::lit("] }")))))))
        }
        Adt3::C23 => {
            Ok(f1(Str::lit("")))
        }
        Adt3::C24(r534, r535) => {
            let v25 = (*r534).clone();
            let v26 = (*r535).clone();
            let v27: Bytes = f54(v25)?;
            let v28: Bytes = f54(v26.clone())?;
            if f75(v26) {
                Ok(v27)
            } else {
                Ok(f4(v27, f4(f1(Str::lit(", ")), v28)))
            }
        }
    }
}

pub fn f55(v0: Bytes) -> R<Result<(), Bytes>> {
    let m536 = f76(v0.clone())?;
    match m536 {
        Ok(v1) => {
            let v2: bool = f77(match v1.clone() {
                Adt9::C0(_, h537) => {
                    h537
                }
            }, f1(Str::lit("18446744073709551615")));
            if match v1 {
                Adt9::C0(h538, _) => {
                    h538
                }
            } {
                Ok(Err::<(), Bytes>(f4(f1(Str::lit("nat literal ")), f4(v0, f1(Str::lit(" is outside the 64-bit realization"))))))
            } else {
                if v2 {
                    Ok(Ok::<(), Bytes>(()))
                } else {
                    Ok(Err::<(), Bytes>(f4(f1(Str::lit("nat literal ")), f4(v0, f1(Str::lit(" is outside the 64-bit realization"))))))
                }
            }
        }
        Err(v3) => {
            Ok(Err::<(), Bytes>(v3))
        }
    }
}

pub fn f56(v0: Bytes, v1: Adt1) -> R<Result<(), Bytes>> {
    let m539 = f76(v0.clone())?;
    match m539 {
        Ok(v2) => {
            let v3: Adt9 = f78(v1.clone());
            let v4: Adt9 = f79(v1.clone());
            let v5: bool = f80(v3, v2.clone());
            let v6: bool = f80(v2, v4);
            if v5 {
                if v6 {
                    Ok(Ok::<(), Bytes>(()))
                } else {
                    Ok(Err::<(), Bytes>(f4(v0, f4(f1(Str::lit(" is outside ")), f81(v1)))))
                }
            } else {
                Ok(Err::<(), Bytes>(f4(v0, f4(f1(Str::lit(" is outside ")), f81(v1)))))
            }
        }
        Err(v7) => {
            Ok(Err::<(), Bytes>(v7))
        }
    }
}

pub fn f57(v0: Adt1) -> u64 {
    let m540 = v0;
    match m540 {
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
        Adt1::C7 => {
            7u64
        }
    }
}

pub fn f58(v0: Adt0) -> Adt1 {
    let m541 = v0;
    match m541 {
        Adt0::C0 => {
            Adt1::C0
        }
        Adt0::C1 => {
            Adt1::C0
        }
        Adt0::C2 => {
            Adt1::C0
        }
        Adt0::C3 => {
            Adt1::C0
        }
        Adt0::C4(v1) => {
            v1
        }
        Adt0::C5 => {
            Adt1::C0
        }
        Adt0::C6 => {
            Adt1::C0
        }
        Adt0::C7 => {
            Adt1::C0
        }
        Adt0::C8(_) => {
            Adt1::C0
        }
        Adt0::C9(_, _) => {
            Adt1::C0
        }
        Adt0::C10(_) => {
            Adt1::C0
        }
        Adt0::C11(_, _) => {
            Adt1::C0
        }
        Adt0::C12(_) => {
            Adt1::C0
        }
        Adt0::C13(_, _) => {
            Adt1::C0
        }
        Adt0::C14 => {
            Adt1::C0
        }
        Adt0::C15(_, _) => {
            Adt1::C0
        }
    }
}

pub fn f59(v0: Bytes) -> R<Result<(), Bytes>> {
    let a546 = {
        let a543 = {
            let a542 = v0.clone();
            length_bytes(a542)
        };
        let a544 = 2u64;
        let a545 = 0u64;
        nat_rem(a543, a544, a545)
    };
    let a547 = 0u64;
    if nat_eq(a546, a547) {
        if f82(v0.clone())? {
            Ok(Ok::<(), Bytes>(()))
        } else {
            Ok(Err::<(), Bytes>(f4(f1(Str::lit("byte literal `")), f4(v0, f1(Str::lit("` is not even-length lowercase hexadecimal"))))))
        }
    } else {
        Ok(Err::<(), Bytes>(f4(f1(Str::lit("byte literal `")), f4(v0, f1(Str::lit("` is not even-length lowercase hexadecimal"))))))
    }
}

pub fn f60(v0: Adt7, v1: u64, v2: u64) -> R<Result<Adt0, Bytes>> {
    let m548 = f26(v0, v1);
    match m548 {
        None => {
            Ok(Err::<Adt0, Bytes>(f27(v1)?))
        }
        Some(v3) => {
            let m549 = f30(v3, v2);
            match m549 {
                None => {
                    Ok(Err::<Adt0, Bytes>(f4(f1(Str::lit("ADT ")), f4(f7(v1)?, f4(f1(Str::lit(" has no constructor ")), f7(v2)?)))))
                }
                Some(v4) => {
                    Ok(Ok::<Adt0, Bytes>(v4))
                }
            }
        }
    }
}

pub fn f61(v0: Adt3) -> R<u64> {
    let m550 = v0;
    match m550 {
        Adt3::C0 => {
            Ok(0u64)
        }
        Adt3::C1(_) => {
            Ok(0u64)
        }
        Adt3::C2(_) => {
            Ok(0u64)
        }
        Adt3::C3(_) => {
            Ok(0u64)
        }
        Adt3::C4(_) => {
            Ok(0u64)
        }
        Adt3::C5(_) => {
            Ok(0u64)
        }
        Adt3::C6(_) => {
            Ok(0u64)
        }
        Adt3::C7(_) => {
            Ok(0u64)
        }
        Adt3::C8(_) => {
            Ok(0u64)
        }
        Adt3::C9(_) => {
            Ok(0u64)
        }
        Adt3::C10(_) => {
            Ok(0u64)
        }
        Adt3::C11(_) => {
            Ok(0u64)
        }
        Adt3::C12(_) => {
            Ok(0u64)
        }
        Adt3::C13(_) => {
            Ok(0u64)
        }
        Adt3::C14(_) => {
            Ok(0u64)
        }
        Adt3::C15 => {
            Ok(0u64)
        }
        Adt3::C16(_) => {
            Ok(0u64)
        }
        Adt3::C17(_) => {
            Ok(0u64)
        }
        Adt3::C18(_) => {
            Ok(0u64)
        }
        Adt3::C19(_) => {
            Ok(0u64)
        }
        Adt3::C20(_, _) => {
            Ok(0u64)
        }
        Adt3::C21(_, _) => {
            Ok(0u64)
        }
        Adt3::C22(_, _) => {
            Ok(0u64)
        }
        Adt3::C23 => {
            Ok(0u64)
        }
        Adt3::C24(_, r553) => {
            let v26 = (*r553).clone();
            let v27: u64 = f61(v26)?;
            let a551 = 1u64;
            let a552 = v27;
            nat_add(a551, a552)
        }
    }
}

pub fn f62(v0: Adt0, v1: u64) -> R<List<Adt5>> {
    let m554 = v0;
    match m554 {
        Adt0::C0 => {
            Ok(List::<Adt5>::nil())
        }
        Adt0::C1 => {
            Ok(List::<Adt5>::nil())
        }
        Adt0::C2 => {
            Ok(List::<Adt5>::nil())
        }
        Adt0::C3 => {
            Ok(List::<Adt5>::nil())
        }
        Adt0::C4(_) => {
            Ok(List::<Adt5>::nil())
        }
        Adt0::C5 => {
            Ok(List::<Adt5>::nil())
        }
        Adt0::C6 => {
            Ok(List::<Adt5>::nil())
        }
        Adt0::C7 => {
            Ok(List::<Adt5>::nil())
        }
        Adt0::C8(_) => {
            Ok(List::<Adt5>::nil())
        }
        Adt0::C9(_, _) => {
            Ok(List::<Adt5>::nil())
        }
        Adt0::C10(_) => {
            Ok(List::<Adt5>::nil())
        }
        Adt0::C11(_, _) => {
            Ok(List::<Adt5>::nil())
        }
        Adt0::C12(_) => {
            Ok(List::<Adt5>::nil())
        }
        Adt0::C13(_, _) => {
            Ok(List::<Adt5>::nil())
        }
        Adt0::C14 => {
            Ok(List::<Adt5>::nil())
        }
        Adt0::C15(_, r557) => {
            let v13 = (*r557).clone();
            Ok(f51(Adt5::C15(v1), f62(v13, {
                let a555 = v1;
                let a556 = 1u64;
                nat_add(a555, a556)?
            })?))
        }
    }
}

pub fn f63(v0: Adt0) -> Option<Adt0> {
    let m558 = v0;
    match m558 {
        Adt0::C0 => {
            None::<Adt0>
        }
        Adt0::C1 => {
            None::<Adt0>
        }
        Adt0::C2 => {
            None::<Adt0>
        }
        Adt0::C3 => {
            None::<Adt0>
        }
        Adt0::C4(_) => {
            None::<Adt0>
        }
        Adt0::C5 => {
            None::<Adt0>
        }
        Adt0::C6 => {
            None::<Adt0>
        }
        Adt0::C7 => {
            None::<Adt0>
        }
        Adt0::C8(r559) => {
            let v2 = (*r559).clone();
            Some(v2)
        }
        Adt0::C9(r560, _) => {
            let v3 = (*r560).clone();
            Some(v3)
        }
        Adt0::C10(r561) => {
            let v5 = (*r561).clone();
            Some(v5)
        }
        Adt0::C11(r562, _) => {
            let v6 = (*r562).clone();
            Some(v6)
        }
        Adt0::C12(_) => {
            None::<Adt0>
        }
        Adt0::C13(r563, _) => {
            let v9 = (*r563).clone();
            Some(v9)
        }
        Adt0::C14 => {
            None::<Adt0>
        }
        Adt0::C15(r564, _) => {
            let v11 = (*r564).clone();
            Some(v11)
        }
    }
}

pub fn f64(v0: Adt0) -> Option<Adt0> {
    let m565 = v0;
    match m565 {
        Adt0::C0 => {
            None::<Adt0>
        }
        Adt0::C1 => {
            None::<Adt0>
        }
        Adt0::C2 => {
            None::<Adt0>
        }
        Adt0::C3 => {
            None::<Adt0>
        }
        Adt0::C4(_) => {
            None::<Adt0>
        }
        Adt0::C5 => {
            None::<Adt0>
        }
        Adt0::C6 => {
            None::<Adt0>
        }
        Adt0::C7 => {
            None::<Adt0>
        }
        Adt0::C8(_) => {
            None::<Adt0>
        }
        Adt0::C9(_, r566) => {
            let v4 = (*r566).clone();
            Some(v4)
        }
        Adt0::C10(_) => {
            None::<Adt0>
        }
        Adt0::C11(_, r567) => {
            let v7 = (*r567).clone();
            Some(v7)
        }
        Adt0::C12(_) => {
            None::<Adt0>
        }
        Adt0::C13(_, r568) => {
            let v10 = (*r568).clone();
            Some(v10)
        }
        Adt0::C14 => {
            None::<Adt0>
        }
        Adt0::C15(_, r569) => {
            let v12 = (*r569).clone();
            Some(v12)
        }
    }
}

pub fn f65(v0: Adt6, v1: Adt0, v2: Adt0, v3: Adt0) -> R<Result<Adt0, Bytes>> {
    let v4: Result<Adt0, Bytes> = Ok::<Adt0, Bytes>(v3);
    let v5: Result<Adt0, Bytes> = Err::<Adt0, Bytes>(f4(f1(Str::lit("primitive ")), f4(f83(v0)?, f4(f1(Str::lit(" takes ")), f4(f84(v1.clone())?, f4(f1(Str::lit(", received ")), f84(v2.clone())?))))));
    if f52(v2, v1) {
        Ok(v4)
    } else {
        Ok(v5)
    }
}

pub fn f66(v0: Adt6, v1: Adt0) -> R<Result<Adt0, Bytes>> {
    Ok(Err::<Adt0, Bytes>(f4(f1(Str::lit("primitive ")), f4(f83(v0)?, f4(f1(Str::lit(" does not apply to ")), f84(v1)?)))))
}

pub fn f67(v0: Adt1) -> bool {
    let m570 = v0;
    match m570 {
        Adt1::C0 => {
            false
        }
        Adt1::C1 => {
            false
        }
        Adt1::C2 => {
            false
        }
        Adt1::C3 => {
            false
        }
        Adt1::C4 => {
            true
        }
        Adt1::C5 => {
            true
        }
        Adt1::C6 => {
            true
        }
        Adt1::C7 => {
            true
        }
    }
}

pub fn f68(v0: Adt0) -> bool {
    let m571 = v0;
    match m571 {
        Adt0::C0 => {
            false
        }
        Adt0::C1 => {
            true
        }
        Adt0::C2 => {
            true
        }
        Adt0::C3 => {
            true
        }
        Adt0::C4(_) => {
            true
        }
        Adt0::C5 => {
            true
        }
        Adt0::C6 => {
            false
        }
        Adt0::C7 => {
            false
        }
        Adt0::C8(_) => {
            false
        }
        Adt0::C9(_, _) => {
            false
        }
        Adt0::C10(_) => {
            false
        }
        Adt0::C11(r572, r573) => {
            let v2 = (*r572).clone();
            let v3 = (*r573).clone();
            let v4: bool = f68(v2);
            let v5: bool = f68(v3);
            if v4 {
                v5
            } else {
                false
            }
        }
        Adt0::C12(_) => {
            false
        }
        Adt0::C13(_, _) => {
            false
        }
        Adt0::C14 => {
            false
        }
        Adt0::C15(_, _) => {
            false
        }
    }
}

pub fn f69(v0: Adt1) -> Bytes {
    let m574 = v0;
    match m574 {
        Adt1::C0 => {
            f1(Str::lit("U8"))
        }
        Adt1::C1 => {
            f1(Str::lit("U16"))
        }
        Adt1::C2 => {
            f1(Str::lit("U32"))
        }
        Adt1::C3 => {
            f1(Str::lit("U64"))
        }
        Adt1::C4 => {
            f1(Str::lit("I8"))
        }
        Adt1::C5 => {
            f1(Str::lit("I16"))
        }
        Adt1::C6 => {
            f1(Str::lit("I32"))
        }
        Adt1::C7 => {
            f1(Str::lit("I64"))
        }
    }
}

pub fn f70(v0: Adt5, v1: Adt5) -> bool {
    let a575 = f85(v0.clone());
    let a576 = f85(v1.clone());
    if nat_eq(a575, a576) {
        let a577 = f86(v0);
        let a578 = f86(v1);
        nat_eq(a577, a578)
    } else {
        false
    }
}

pub fn f71(v0: Bytes, v1: u64, v2: u8) -> bool {
    let m579 = f87(v0, v1);
    match m579 {
        None => {
            false
        }
        Some(v3) => {
            let a580 = v3;
            let a581 = v2;
            equal(a580, a581)
        }
    }
}

pub fn f72(v0: bool) -> Bytes {
    if v0 {
        f1(Str::lit("true"))
    } else {
        f1(Str::lit("false"))
    }
}

pub fn f73(v0: Bytes) -> R<Bytes> {
    Ok(f4(f1(Str::lit("\"")), f4({
        let (_, h582) = {
            let (h583, _) = f88(Fn2::F89(v0.clone()), {
                let a584 = v0;
                length_bytes(a584)
            }, (0u64, Bytes::lit(&[])))?;
            h583
        };
        h582
    }, f1(Str::lit("\"")))))
}

pub fn f74(v0: Adt4) -> Bytes {
    let m585 = v0;
    match m585 {
        Adt4::C0 => {
            f1(Str::lit("Lt"))
        }
        Adt4::C1 => {
            f1(Str::lit("Eq"))
        }
        Adt4::C2 => {
            f1(Str::lit("Gt"))
        }
    }
}

pub fn f75(v0: Adt3) -> bool {
    let m586 = v0;
    match m586 {
        Adt3::C0 => {
            false
        }
        Adt3::C1(_) => {
            false
        }
        Adt3::C2(_) => {
            false
        }
        Adt3::C3(_) => {
            false
        }
        Adt3::C4(_) => {
            false
        }
        Adt3::C5(_) => {
            false
        }
        Adt3::C6(_) => {
            false
        }
        Adt3::C7(_) => {
            false
        }
        Adt3::C8(_) => {
            false
        }
        Adt3::C9(_) => {
            false
        }
        Adt3::C10(_) => {
            false
        }
        Adt3::C11(_) => {
            false
        }
        Adt3::C12(_) => {
            false
        }
        Adt3::C13(_) => {
            false
        }
        Adt3::C14(_) => {
            false
        }
        Adt3::C15 => {
            false
        }
        Adt3::C16(_) => {
            false
        }
        Adt3::C17(_) => {
            false
        }
        Adt3::C18(_) => {
            false
        }
        Adt3::C19(_) => {
            false
        }
        Adt3::C20(_, _) => {
            false
        }
        Adt3::C21(_, _) => {
            false
        }
        Adt3::C22(_, _) => {
            false
        }
        Adt3::C23 => {
            true
        }
        Adt3::C24(_, _) => {
            false
        }
    }
}

pub fn f76(v0: Bytes) -> R<Result<Adt9, Bytes>> {
    let v1: bool = f71(v0.clone(), 0u64, 45u8);
    let v3: Bytes = {
        let v2: Bytes = f25(v0.clone(), 1u64, {
            let a588 = {
                let a587 = v0.clone();
                length_bytes(a587)
            };
            let a589 = 1u64;
            nat_sub(a588, a589)
        });
        if v1 {
            v2
        } else {
            v0.clone()
        }
    };
    let v4: bool = f90(v3.clone(), v0.clone())?;
    let v5: bool = f77(v3.clone(), f1(Str::lit("170141183460469231731687303715884105727")));
    let v6: bool = f77(v3.clone(), f1(Str::lit("170141183460469231731687303715884105728")));
    if v4 {
        if if v1 {
            v6
        } else {
            v5
        } {
            Ok(Ok::<Adt9, Bytes>(Adt9::C0(v1, v3)))
        } else {
            Ok(Err::<Adt9, Bytes>(f4(f5(v0), f1(Str::lit(" is outside every realized integer")))))
        }
    } else {
        Ok(Err::<Adt9, Bytes>(f4(f5(v0), f1(Str::lit(" is not a canonical decimal")))))
    }
}

pub fn f77(v0: Bytes, v1: Bytes) -> bool {
    let v2: bool = {
        let a592 = {
            let a590 = v0.clone();
            length_bytes(a590)
        };
        let a593 = {
            let a591 = v1.clone();
            length_bytes(a591)
        };
        nat_lt(a592, a593)
    };
    let v3: bool = {
        let a596 = {
            let a594 = v0.clone();
            length_bytes(a594)
        };
        let a597 = {
            let a595 = v1.clone();
            length_bytes(a595)
        };
        nat_eq(a596, a597)
    };
    let v4: bool = f91(v1, v0);
    if v2 {
        true
    } else {
        if v3 {
            let a598 = v4;
            bool_not(a598)
        } else {
            false
        }
    }
}

pub fn f78(v0: Adt1) -> Adt9 {
    let m599 = v0;
    match m599 {
        Adt1::C0 => {
            f92(false, Str::lit("0"))
        }
        Adt1::C1 => {
            f92(false, Str::lit("0"))
        }
        Adt1::C2 => {
            f92(false, Str::lit("0"))
        }
        Adt1::C3 => {
            f92(false, Str::lit("0"))
        }
        Adt1::C4 => {
            f92(true, Str::lit("128"))
        }
        Adt1::C5 => {
            f92(true, Str::lit("32768"))
        }
        Adt1::C6 => {
            f92(true, Str::lit("2147483648"))
        }
        Adt1::C7 => {
            f92(true, Str::lit("9223372036854775808"))
        }
    }
}

pub fn f79(v0: Adt1) -> Adt9 {
    let m600 = v0;
    match m600 {
        Adt1::C0 => {
            f92(false, Str::lit("255"))
        }
        Adt1::C1 => {
            f92(false, Str::lit("65535"))
        }
        Adt1::C2 => {
            f92(false, Str::lit("4294967295"))
        }
        Adt1::C3 => {
            f92(false, Str::lit("18446744073709551615"))
        }
        Adt1::C4 => {
            f92(false, Str::lit("127"))
        }
        Adt1::C5 => {
            f92(false, Str::lit("32767"))
        }
        Adt1::C6 => {
            f92(false, Str::lit("2147483647"))
        }
        Adt1::C7 => {
            f92(false, Str::lit("9223372036854775807"))
        }
    }
}

pub fn f80(v0: Adt9, v1: Adt9) -> bool {
    let v2: bool = f77(match v1.clone() {
        Adt9::C0(_, h601) => {
            h601
        }
    }, match v0.clone() {
        Adt9::C0(_, h602) => {
            h602
        }
    });
    let v3: bool = f77(match v0.clone() {
        Adt9::C0(_, h603) => {
            h603
        }
    }, match v1.clone() {
        Adt9::C0(_, h604) => {
            h604
        }
    });
    if match v0 {
        Adt9::C0(h605, _) => {
            h605
        }
    } {
        if match v1 {
            Adt9::C0(h606, _) => {
                h606
            }
        } {
            v2
        } else {
            true
        }
    } else {
        if match v1 {
            Adt9::C0(h607, _) => {
                h607
            }
        } {
            false
        } else {
            v3
        }
    }
}

pub fn f81(v0: Adt1) -> Bytes {
    let m608 = v0;
    match m608 {
        Adt1::C0 => {
            f1(Str::lit("u8"))
        }
        Adt1::C1 => {
            f1(Str::lit("u16"))
        }
        Adt1::C2 => {
            f1(Str::lit("u32"))
        }
        Adt1::C3 => {
            f1(Str::lit("u64"))
        }
        Adt1::C4 => {
            f1(Str::lit("i8"))
        }
        Adt1::C5 => {
            f1(Str::lit("i16"))
        }
        Adt1::C6 => {
            f1(Str::lit("i32"))
        }
        Adt1::C7 => {
            f1(Str::lit("i64"))
        }
    }
}

pub fn f82(v0: Bytes) -> R<bool> {
    let a612 = {
        let (h609, _) = f23(Fn0::F93(v0.clone()), {
            let a610 = v0.clone();
            length_bytes(a610)
        }, 0u64)?;
        h609
    };
    let a613 = {
        let a611 = v0;
        length_bytes(a611)
    };
    Ok(nat_eq(a612, a613))
}

pub fn f83(v0: Adt6) -> R<Bytes> {
    let m614 = v0;
    match m614 {
        Adt6::C0 => {
            Ok(f1(Str::lit("NatAdd")))
        }
        Adt6::C1 => {
            Ok(f1(Str::lit("NatSub")))
        }
        Adt6::C2 => {
            Ok(f1(Str::lit("NatMul")))
        }
        Adt6::C3 => {
            Ok(f1(Str::lit("NatQuot")))
        }
        Adt6::C4 => {
            Ok(f1(Str::lit("NatRem")))
        }
        Adt6::C5 => {
            Ok(f1(Str::lit("NatEq")))
        }
        Adt6::C6 => {
            Ok(f1(Str::lit("NatLe")))
        }
        Adt6::C7 => {
            Ok(f1(Str::lit("NatLt")))
        }
        Adt6::C8 => {
            Ok(f1(Str::lit("IntAdd")))
        }
        Adt6::C9 => {
            Ok(f1(Str::lit("IntSub")))
        }
        Adt6::C10 => {
            Ok(f1(Str::lit("IntMul")))
        }
        Adt6::C11 => {
            Ok(f1(Str::lit("IntNeg")))
        }
        Adt6::C12 => {
            Ok(f1(Str::lit("IntQuot")))
        }
        Adt6::C13 => {
            Ok(f1(Str::lit("IntRem")))
        }
        Adt6::C14 => {
            Ok(f1(Str::lit("CheckedAdd")))
        }
        Adt6::C15 => {
            Ok(f1(Str::lit("CheckedSub")))
        }
        Adt6::C16 => {
            Ok(f1(Str::lit("CheckedMul")))
        }
        Adt6::C17 => {
            Ok(f1(Str::lit("CheckedNeg")))
        }
        Adt6::C18 => {
            Ok(f1(Str::lit("CheckedQuot")))
        }
        Adt6::C19 => {
            Ok(f1(Str::lit("BitAnd")))
        }
        Adt6::C20 => {
            Ok(f1(Str::lit("BitOr")))
        }
        Adt6::C21 => {
            Ok(f1(Str::lit("BitXor")))
        }
        Adt6::C22 => {
            Ok(f1(Str::lit("BitNot")))
        }
        Adt6::C23 => {
            Ok(f1(Str::lit("ShiftLeft")))
        }
        Adt6::C24 => {
            Ok(f1(Str::lit("ShiftRight")))
        }
        Adt6::C25 => {
            Ok(f1(Str::lit("Equal")))
        }
        Adt6::C26 => {
            Ok(f1(Str::lit("BoolNot")))
        }
        Adt6::C27 => {
            Ok(f1(Str::lit("BoolAnd")))
        }
        Adt6::C28 => {
            Ok(f1(Str::lit("BoolOr")))
        }
        Adt6::C29 => {
            Ok(f1(Str::lit("Append")))
        }
        Adt6::C30 => {
            Ok(f1(Str::lit("Length")))
        }
        Adt6::C31 => {
            Ok(f1(Str::lit("Index")))
        }
        Adt6::C32 => {
            Ok(f1(Str::lit("Slice")))
        }
        Adt6::C33 => {
            Ok(f1(Str::lit("Utf8Encode")))
        }
        Adt6::C34 => {
            Ok(f1(Str::lit("Utf8Decode")))
        }
        Adt6::C35 => {
            Ok(f1(Str::lit("CompareBytes")))
        }
        Adt6::C36 => {
            Ok(f1(Str::lit("SplitExact")))
        }
        Adt6::C37 => {
            Ok(f1(Str::lit("Join")))
        }
        Adt6::C38 => {
            Ok(f1(Str::lit("FormatDecimal")))
        }
        Adt6::C39 => {
            Ok(f1(Str::lit("Compare")))
        }
        Adt6::C40(v1) => {
            Ok(f4(f1(Str::lit("Convert { target: ")), f4(f69(v1), f1(Str::lit(" }")))))
        }
        Adt6::C41(v2) => {
            Ok(f4(f1(Str::lit("ParseDecimal { target: ")), f4(f47(v2)?, f1(Str::lit(" }")))))
        }
    }
}

pub fn f84(v0: Adt0) -> R<Bytes> {
    Ok(f4(f1(Str::lit("[")), f4(f47(v0)?, f1(Str::lit("]")))))
}

pub fn f85(v0: Adt5) -> u64 {
    let m615 = v0;
    match m615 {
        Adt5::C0 => {
            0u64
        }
        Adt5::C1 => {
            1u64
        }
        Adt5::C2 => {
            2u64
        }
        Adt5::C3 => {
            3u64
        }
        Adt5::C4 => {
            4u64
        }
        Adt5::C5 => {
            5u64
        }
        Adt5::C6 => {
            6u64
        }
        Adt5::C7 => {
            7u64
        }
        Adt5::C8 => {
            8u64
        }
        Adt5::C9 => {
            9u64
        }
        Adt5::C10 => {
            10u64
        }
        Adt5::C11 => {
            11u64
        }
        Adt5::C12 => {
            12u64
        }
        Adt5::C13 => {
            13u64
        }
        Adt5::C14 => {
            14u64
        }
        Adt5::C15(_) => {
            15u64
        }
    }
}

pub fn f86(v0: Adt5) -> u64 {
    let m616 = v0;
    match m616 {
        Adt5::C0 => {
            0u64
        }
        Adt5::C1 => {
            0u64
        }
        Adt5::C2 => {
            0u64
        }
        Adt5::C3 => {
            0u64
        }
        Adt5::C4 => {
            0u64
        }
        Adt5::C5 => {
            0u64
        }
        Adt5::C6 => {
            0u64
        }
        Adt5::C7 => {
            0u64
        }
        Adt5::C8 => {
            0u64
        }
        Adt5::C9 => {
            0u64
        }
        Adt5::C10 => {
            0u64
        }
        Adt5::C11 => {
            0u64
        }
        Adt5::C12 => {
            0u64
        }
        Adt5::C13 => {
            0u64
        }
        Adt5::C14 => {
            0u64
        }
        Adt5::C15(v1) => {
            v1
        }
    }
}

pub fn f87(v0: Bytes, v1: u64) -> Option<u8> {
    let a617 = v0;
    let a618 = v1;
    index_bytes(a617, a618)
}

pub fn f88(v0: Fn2, v1: u64, v2: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m619 = v1;
    if m619 == 0 {
        Ok((v2, false))
    } else {
        let v3 = m619 - 1;
        let m621 = {
            let c620: Fn2 = v0.clone();
            c620.apply(v2.clone())?
        };
        match m621 {
            None => {
                Ok((v2, true))
            }
            Some(v4) => {
                f88(v0, v3, v4)
            }
        }
    }
}

pub fn f89(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f94(v0, v1)
}

pub fn f90(v0: Bytes, v1: Bytes) -> R<bool> {
    let a623 = 0u64;
    let a624 = {
        let a622 = v0.clone();
        length_bytes(a622)
    };
    if nat_lt(a623, a624) {
        if f95(v0.clone())? {
            let a625 = v0.clone();
            let a626 = f1(Str::lit("0"));
            if equal(a625, a626) {
                let a629 = {
                    let a627 = v1;
                    let a628 = f1(Str::lit("-0"));
                    equal(a627, a628)
                };
                Ok(bool_not(a629))
            } else {
                let a630 = f71(v0, 0u64, 48u8);
                Ok(bool_not(a630))
            }
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f91(v0: Bytes, v1: Bytes) -> bool {
    let a635 = {
        let a631 = v0;
        let a632 = v1;
        compare_bytes(a631, a632)
    };
    let a636 = {
        let a633 = Bytes::lit(&[]);
        let a634 = Bytes::lit(&[0x01]);
        compare_bytes(a633, a634)
    };
    equal(a635, a636)
}

pub fn f92(v0: bool, v1: Str) -> Adt9 {
    Adt9::C0(v0, f1(v1))
}

pub fn f93(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f96(v0, v1)
}

pub fn f94(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let v2: u64 = {
        let (h637, _) = v1;
        h637
    };
    let v3: u64 = f97(v0.clone(), v2)?;
    let a638 = v3;
    let a639 = 256u64;
    if nat_eq(a638, a639) {
        Ok(None::<(u64, Bytes)>)
    } else {
        let v4: (u64, Bytes) = f98(v0, v2, v3)?;
        Ok(Some(({
            let a641 = v2;
            let a642 = {
                let (h640, _) = v4;
                h640
            };
            nat_add(a641, a642)?
        }, f4({
            let (_, h643) = v1;
            h643
        }, {
            let (_, h644) = v4;
            h644
        }))))
    }
}

pub fn f95(v0: Bytes) -> R<bool> {
    let a648 = {
        let (h645, _) = f23(Fn0::F99(v0.clone()), {
            let a646 = v0.clone();
            length_bytes(a646)
        }, 0u64)?;
        h645
    };
    let a649 = {
        let a647 = v0;
        length_bytes(a647)
    };
    Ok(nat_eq(a648, a649))
}

pub fn f96(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f100(v0, v1)? {
        Ok(Some({
            let a650 = v1;
            let a651 = 1u64;
            nat_add(a650, a651)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f97(v0: Bytes, v1: u64) -> R<u64> {
    let m652 = f87(v0, v1);
    match m652 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a697 = {
                let a673 = {
                    let a661 = {
                        let a655 = {
                            let a653 = v3;
                            let a654 = 128u8;
                            fixed_u8::bit_and(a653, a654)
                        };
                        let a656 = 128u8;
                        if equal(a655, a656) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a662 = {
                        let a659 = {
                            let a657 = v3;
                            let a658 = 64u8;
                            fixed_u8::bit_and(a657, a658)
                        };
                        let a660 = 64u8;
                        if equal(a659, a660) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a661, a662)?
                };
                let a674 = {
                    let a671 = {
                        let a665 = {
                            let a663 = v3;
                            let a664 = 32u8;
                            fixed_u8::bit_and(a663, a664)
                        };
                        let a666 = 32u8;
                        if equal(a665, a666) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a672 = {
                        let a669 = {
                            let a667 = v3;
                            let a668 = 16u8;
                            fixed_u8::bit_and(a667, a668)
                        };
                        let a670 = 16u8;
                        if equal(a669, a670) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a671, a672)?
                };
                nat_add(a673, a674)?
            };
            let a698 = {
                let a695 = {
                    let a683 = {
                        let a677 = {
                            let a675 = v3;
                            let a676 = 8u8;
                            fixed_u8::bit_and(a675, a676)
                        };
                        let a678 = 8u8;
                        if equal(a677, a678) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a684 = {
                        let a681 = {
                            let a679 = v3;
                            let a680 = 4u8;
                            fixed_u8::bit_and(a679, a680)
                        };
                        let a682 = 4u8;
                        if equal(a681, a682) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a683, a684)?
                };
                let a696 = {
                    let a693 = {
                        let a687 = {
                            let a685 = v3;
                            let a686 = 2u8;
                            fixed_u8::bit_and(a685, a686)
                        };
                        let a688 = 2u8;
                        if equal(a687, a688) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a694 = {
                        let a691 = {
                            let a689 = v3;
                            let a690 = 1u8;
                            fixed_u8::bit_and(a689, a690)
                        };
                        let a692 = 1u8;
                        if equal(a691, a692) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a693, a694)?
                };
                nat_add(a695, a696)?
            };
            nat_add(a697, a698)
        }
    }
}

pub fn f98(v0: Bytes, v1: u64, v2: u64) -> R<(u64, Bytes)> {
    let v3: u64 = f97(v0.clone(), {
        let a699 = v1;
        let a700 = 1u64;
        nat_add(a699, a700)?
    })?;
    let a701 = v2;
    let a702 = 34u64;
    if nat_eq(a701, a702) {
        Ok((1u64, f1(Str::lit("\\\""))))
    } else {
        let a703 = v2;
        let a704 = 92u64;
        if nat_eq(a703, a704) {
            Ok((1u64, f1(Str::lit("\\\\"))))
        } else {
            let a705 = v2;
            let a706 = 10u64;
            if nat_eq(a705, a706) {
                Ok((1u64, f1(Str::lit("\\n"))))
            } else {
                let a707 = v2;
                let a708 = 13u64;
                if nat_eq(a707, a708) {
                    Ok((1u64, f1(Str::lit("\\r"))))
                } else {
                    let a709 = v2;
                    let a710 = 9u64;
                    if nat_eq(a709, a710) {
                        Ok((1u64, f1(Str::lit("\\t"))))
                    } else {
                        let a711 = v2;
                        let a712 = 0u64;
                        if nat_eq(a711, a712) {
                            Ok((1u64, f1(Str::lit("\\0"))))
                        } else {
                            let a713 = v2;
                            let a714 = 32u64;
                            if nat_lt(a713, a714) {
                                Ok((1u64, f101(v2)))
                            } else {
                                let a715 = v2;
                                let a716 = 127u64;
                                if nat_eq(a715, a716) {
                                    Ok((1u64, f101(v2)))
                                } else {
                                    let a717 = v2;
                                    let a718 = 194u64;
                                    if nat_eq(a717, a718) {
                                        let a719 = 128u64;
                                        let a720 = v3;
                                        if nat_le(a719, a720) {
                                            let a721 = v3;
                                            let a722 = 159u64;
                                            if nat_le(a721, a722) {
                                                Ok((2u64, f101(v3)))
                                            } else {
                                                Ok((1u64, f25(v0, v1, 1u64)))
                                            }
                                        } else {
                                            Ok((1u64, f25(v0, v1, 1u64)))
                                        }
                                    } else {
                                        Ok((1u64, f25(v0, v1, 1u64)))
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

pub fn f99(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f102(v0, v1)
}

pub fn f100(v0: Bytes, v1: u64) -> R<bool> {
    let v2: u64 = f97(v0, v1)?;
    let a723 = 47u64;
    let a724 = v2;
    if nat_lt(a723, a724) {
        let a725 = v2;
        let a726 = 58u64;
        if nat_lt(a725, a726) {
            Ok(true)
        } else {
            let a727 = 96u64;
            let a728 = v2;
            if nat_lt(a727, a728) {
                let a729 = v2;
                let a730 = 103u64;
                Ok(nat_lt(a729, a730))
            } else {
                Ok(false)
            }
        }
    } else {
        Ok(false)
    }
}

pub fn f101(v0: u64) -> Bytes {
    f4(f1(Str::lit("\\u{")), f4(f103(v0), f1(Str::lit("}"))))
}

pub fn f102(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f104(v0, v1)? {
        Ok(Some({
            let a731 = v1;
            let a732 = 1u64;
            nat_add(a731, a732)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f103(v0: u64) -> Bytes {
    let v1: Bytes = f105(v0);
    let v2: Bytes = f4(f105({
        let a733 = v0;
        let a734 = 16u64;
        let a735 = 0u64;
        nat_quot(a733, a734, a735)
    }), f105({
        let a736 = v0;
        let a737 = 16u64;
        let a738 = 0u64;
        nat_rem(a736, a737, a738)
    }));
    let a739 = v0;
    let a740 = 16u64;
    if nat_lt(a739, a740) {
        v1
    } else {
        v2
    }
}

pub fn f104(v0: Bytes, v1: u64) -> R<bool> {
    let v2: u64 = f97(v0, v1)?;
    let a741 = 47u64;
    let a742 = v2;
    if nat_lt(a741, a742) {
        let a743 = v2;
        let a744 = 58u64;
        Ok(nat_lt(a743, a744))
    } else {
        Ok(false)
    }
}

pub fn f105(v0: u64) -> Bytes {
    f25(f1(Str::lit("0123456789abcdef")), v0, 1u64)
}

pub fn run(p0: Bytes, p1: Adt0, p2: List<List<u64>>, p3: Adt0, p4: Adt0, p5: Adt2) -> R<Result<(), Bytes>> {
    f0(p0, p1, p2, p3, p4, p5)
}
