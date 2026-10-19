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
    C0(bool, Bytes),
}

#[derive(Clone)]
pub enum Fn0 {
    F42(Bytes),
    F51(Bytes),
    F59(Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn0::F42(k0) => f42(k0.clone(), p0),
            Fn0::F51(k0) => f51(k0.clone(), p0),
            Fn0::F59(k0) => f59(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F45(Bytes),
}

impl Fn1 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn1::F45(k0) => f45(k0.clone(), p0),
        }
    }
}

pub fn f0(v0: Bytes, v1: Adt0, v2: List<List<u64>>, v3: Adt0, v4: Adt0, v5: Adt2, v6: u64, v7: Adt3) -> R<Result<(), Bytes>> {
    let v8: Adt7 = Adt7::C0(v0, v1, v2, v3, v4, v5);
    let m1 = f1(v8.clone(), v6);
    match m1 {
        None => {
            Ok(Err::<(), Bytes>(f2(v6)?))
        }
        Some(v9) => {
            let v10: Result<(), Bytes> = f3(v8, v7.clone(), v9.clone(), 2u64)?;
            let v11: Result<(), Bytes> = Err::<(), Bytes>(f4(f5(Str::lit("function ")), f4(f6(v6)?, f4(f5(Str::lit(" takes ")), f4(f6(f7(v9.clone())?)?, f4(f5(Str::lit(" argument(s), received ")), f6(f8(v7.clone())?)?))))));
            let a2 = f8(v7)?;
            let a3 = f7(v9)?;
            if nat_eq(a2, a3) {
                Ok(v10)
            } else {
                Ok(v11)
            }
        }
    }
}

pub fn f1(v0: Adt7, v1: u64) -> Option<Adt0> {
    f9(match v0 {
        Adt7::C0(_, _, _, h4, _, _) => {
            h4
        }
    }, v1)
}

pub fn f2(v0: u64) -> R<Bytes> {
    Ok(f4(f5(Str::lit("function ")), f4(f6(v0)?, f5(Str::lit(" is not declared")))))
}

pub fn f3(v0: Adt7, v1: Adt3, v2: Adt0, v3: u64) -> R<Result<(), Bytes>> {
    let m5 = v1;
    match m5 {
        Adt3::C0 => {
            let v4: Result<(), Bytes> = Ok::<(), Bytes>(());
            let v5: Result<(), Bytes> = Err::<(), Bytes>(f4(f5(Str::lit("the literal ")), f4(f10(Adt3::C0)?, f4(f5(Str::lit(" does not have type ")), f11(v2.clone())?))));
            let a6 = f12(v2);
            let a7 = 0u64;
            if nat_eq(a6, a7) {
                Ok(v4)
            } else {
                Ok(v5)
            }
        }
        Adt3::C1(v6) => {
            let v7: Result<(), Bytes> = Ok::<(), Bytes>(());
            let v8: Result<(), Bytes> = Err::<(), Bytes>(f4(f5(Str::lit("the literal ")), f4(f10(Adt3::C1(v6))?, f4(f5(Str::lit(" does not have type ")), f11(v2.clone())?))));
            let a8 = f12(v2);
            let a9 = 1u64;
            if nat_eq(a8, a9) {
                Ok(v7)
            } else {
                Ok(v8)
            }
        }
        Adt3::C2(v9) => {
            let v10: Result<(), Bytes> = f13(v9.clone())?;
            let v11: Result<(), Bytes> = Err::<(), Bytes>(f4(f5(Str::lit("the literal ")), f4(f10(Adt3::C2(v9))?, f4(f5(Str::lit(" does not have type ")), f11(v2.clone())?))));
            let a10 = f12(v2);
            let a11 = 2u64;
            if nat_eq(a10, a11) {
                Ok(v10)
            } else {
                Ok(v11)
            }
        }
        Adt3::C3(v12) => {
            let v13: Result<(), Bytes> = f14(v12.clone(), Adt1::C7)?;
            let v14: Result<(), Bytes> = Err::<(), Bytes>(f4(f5(Str::lit("the literal ")), f4(f10(Adt3::C3(v12))?, f4(f5(Str::lit(" does not have type ")), f11(v2.clone())?))));
            let a12 = f12(v2);
            let a13 = 3u64;
            if nat_eq(a12, a13) {
                Ok(v13)
            } else {
                Ok(v14)
            }
        }
        Adt3::C4(v15) => {
            let v16: Result<(), Bytes> = f14(v15.clone(), Adt1::C0)?;
            let v17: Result<(), Bytes> = Err::<(), Bytes>(f4(f5(Str::lit("the literal ")), f4(f10(Adt3::C4(v15))?, f4(f5(Str::lit(" does not have type ")), f11(v2.clone())?))));
            let a14 = f12(v2.clone());
            let a15 = 4u64;
            if if nat_eq(a14, a15) {
                let a16 = f15(f16(v2));
                let a17 = 0u64;
                nat_eq(a16, a17)
            } else {
                false
            } {
                Ok(v16)
            } else {
                Ok(v17)
            }
        }
        Adt3::C5(v18) => {
            let v19: Result<(), Bytes> = f14(v18.clone(), Adt1::C1)?;
            let v20: Result<(), Bytes> = Err::<(), Bytes>(f4(f5(Str::lit("the literal ")), f4(f10(Adt3::C5(v18))?, f4(f5(Str::lit(" does not have type ")), f11(v2.clone())?))));
            let a18 = f12(v2.clone());
            let a19 = 4u64;
            if if nat_eq(a18, a19) {
                let a20 = f15(f16(v2));
                let a21 = 1u64;
                nat_eq(a20, a21)
            } else {
                false
            } {
                Ok(v19)
            } else {
                Ok(v20)
            }
        }
        Adt3::C6(v21) => {
            let v22: Result<(), Bytes> = f14(v21.clone(), Adt1::C2)?;
            let v23: Result<(), Bytes> = Err::<(), Bytes>(f4(f5(Str::lit("the literal ")), f4(f10(Adt3::C6(v21))?, f4(f5(Str::lit(" does not have type ")), f11(v2.clone())?))));
            let a22 = f12(v2.clone());
            let a23 = 4u64;
            if if nat_eq(a22, a23) {
                let a24 = f15(f16(v2));
                let a25 = 2u64;
                nat_eq(a24, a25)
            } else {
                false
            } {
                Ok(v22)
            } else {
                Ok(v23)
            }
        }
        Adt3::C7(v24) => {
            let v25: Result<(), Bytes> = f14(v24.clone(), Adt1::C3)?;
            let v26: Result<(), Bytes> = Err::<(), Bytes>(f4(f5(Str::lit("the literal ")), f4(f10(Adt3::C7(v24))?, f4(f5(Str::lit(" does not have type ")), f11(v2.clone())?))));
            let a26 = f12(v2.clone());
            let a27 = 4u64;
            if if nat_eq(a26, a27) {
                let a28 = f15(f16(v2));
                let a29 = 3u64;
                nat_eq(a28, a29)
            } else {
                false
            } {
                Ok(v25)
            } else {
                Ok(v26)
            }
        }
        Adt3::C8(v27) => {
            let v28: Result<(), Bytes> = f14(v27.clone(), Adt1::C4)?;
            let v29: Result<(), Bytes> = Err::<(), Bytes>(f4(f5(Str::lit("the literal ")), f4(f10(Adt3::C8(v27))?, f4(f5(Str::lit(" does not have type ")), f11(v2.clone())?))));
            let a30 = f12(v2.clone());
            let a31 = 4u64;
            if if nat_eq(a30, a31) {
                let a32 = f15(f16(v2));
                let a33 = 4u64;
                nat_eq(a32, a33)
            } else {
                false
            } {
                Ok(v28)
            } else {
                Ok(v29)
            }
        }
        Adt3::C9(v30) => {
            let v31: Result<(), Bytes> = f14(v30.clone(), Adt1::C5)?;
            let v32: Result<(), Bytes> = Err::<(), Bytes>(f4(f5(Str::lit("the literal ")), f4(f10(Adt3::C9(v30))?, f4(f5(Str::lit(" does not have type ")), f11(v2.clone())?))));
            let a34 = f12(v2.clone());
            let a35 = 4u64;
            if if nat_eq(a34, a35) {
                let a36 = f15(f16(v2));
                let a37 = 5u64;
                nat_eq(a36, a37)
            } else {
                false
            } {
                Ok(v31)
            } else {
                Ok(v32)
            }
        }
        Adt3::C10(v33) => {
            let v34: Result<(), Bytes> = f14(v33.clone(), Adt1::C6)?;
            let v35: Result<(), Bytes> = Err::<(), Bytes>(f4(f5(Str::lit("the literal ")), f4(f10(Adt3::C10(v33))?, f4(f5(Str::lit(" does not have type ")), f11(v2.clone())?))));
            let a38 = f12(v2.clone());
            let a39 = 4u64;
            if if nat_eq(a38, a39) {
                let a40 = f15(f16(v2));
                let a41 = 6u64;
                nat_eq(a40, a41)
            } else {
                false
            } {
                Ok(v34)
            } else {
                Ok(v35)
            }
        }
        Adt3::C11(v36) => {
            let v37: Result<(), Bytes> = f14(v36.clone(), Adt1::C7)?;
            let v38: Result<(), Bytes> = Err::<(), Bytes>(f4(f5(Str::lit("the literal ")), f4(f10(Adt3::C11(v36))?, f4(f5(Str::lit(" does not have type ")), f11(v2.clone())?))));
            let a42 = f12(v2.clone());
            let a43 = 4u64;
            if if nat_eq(a42, a43) {
                let a44 = f15(f16(v2));
                let a45 = 7u64;
                nat_eq(a44, a45)
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
            let v41: Result<(), Bytes> = Err::<(), Bytes>(f4(f5(Str::lit("the literal ")), f4(f10(Adt3::C12(v39))?, f4(f5(Str::lit(" does not have type ")), f11(v2.clone())?))));
            let a46 = f12(v2);
            let a47 = 5u64;
            if nat_eq(a46, a47) {
                Ok(v40)
            } else {
                Ok(v41)
            }
        }
        Adt3::C13(v42) => {
            let v43: Result<(), Bytes> = f17(v42.clone())?;
            let v44: Result<(), Bytes> = Err::<(), Bytes>(f4(f5(Str::lit("the literal ")), f4(f10(Adt3::C13(v42))?, f4(f5(Str::lit(" does not have type ")), f11(v2.clone())?))));
            let a48 = f12(v2);
            let a49 = 6u64;
            if nat_eq(a48, a49) {
                Ok(v43)
            } else {
                Ok(v44)
            }
        }
        Adt3::C14(v45) => {
            let v46: Result<(), Bytes> = Ok::<(), Bytes>(());
            let v47: Result<(), Bytes> = Err::<(), Bytes>(f4(f5(Str::lit("the literal ")), f4(f10(Adt3::C14(v45))?, f4(f5(Str::lit(" does not have type ")), f11(v2.clone())?))));
            let a50 = f12(v2);
            let a51 = 7u64;
            if nat_eq(a50, a51) {
                Ok(v46)
            } else {
                Ok(v47)
            }
        }
        Adt3::C15 => {
            let v48: Result<(), Bytes> = Ok::<(), Bytes>(());
            let v49: Result<(), Bytes> = Err::<(), Bytes>(f4(f5(Str::lit("the literal ")), f4(f10(Adt3::C15)?, f4(f5(Str::lit(" does not have type ")), f11(v2.clone())?))));
            let a52 = f12(v2);
            let a53 = 8u64;
            if nat_eq(a52, a53) {
                Ok(v48)
            } else {
                Ok(v49)
            }
        }
        Adt3::C16(r79) => {
            let v50 = (*r79).clone();
            let v51: Result<(), Bytes> = f3(v0, v50.clone(), f18(v2.clone()), 0u64)?;
            let v52: Result<(), Bytes> = Err::<(), Bytes>(f4(f5(Str::lit("the literal ")), f4(f10(Adt3::C16(std::rc::Rc::new(v50)))?, f4(f5(Str::lit(" does not have type ")), f11(v2.clone())?))));
            let a54 = f12(v2);
            let a55 = 8u64;
            if nat_eq(a54, a55) {
                Ok(v51)
            } else {
                Ok(v52)
            }
        }
        Adt3::C17(r80) => {
            let v53 = (*r80).clone();
            let v54: Result<(), Bytes> = f3(v0, v53.clone(), f18(v2.clone()), 0u64)?;
            let v55: Result<(), Bytes> = Err::<(), Bytes>(f4(f5(Str::lit("the literal ")), f4(f10(Adt3::C17(std::rc::Rc::new(v53)))?, f4(f5(Str::lit(" does not have type ")), f11(v2.clone())?))));
            let a56 = f12(v2);
            let a57 = 9u64;
            if nat_eq(a56, a57) {
                Ok(v54)
            } else {
                Ok(v55)
            }
        }
        Adt3::C18(r81) => {
            let v56 = (*r81).clone();
            let v57: Result<(), Bytes> = f3(v0, v56.clone(), f19(v2.clone()), 0u64)?;
            let v58: Result<(), Bytes> = Err::<(), Bytes>(f4(f5(Str::lit("the literal ")), f4(f10(Adt3::C18(std::rc::Rc::new(v56)))?, f4(f5(Str::lit(" does not have type ")), f11(v2.clone())?))));
            let a58 = f12(v2);
            let a59 = 9u64;
            if nat_eq(a58, a59) {
                Ok(v57)
            } else {
                Ok(v58)
            }
        }
        Adt3::C19(r82) => {
            let v59 = (*r82).clone();
            let v60: Result<(), Bytes> = f3(v0, v59.clone(), f18(v2.clone()), 1u64)?;
            let v61: Result<(), Bytes> = Err::<(), Bytes>(f4(f5(Str::lit("the literal ")), f4(f10(Adt3::C19(std::rc::Rc::new(v59)))?, f4(f5(Str::lit(" does not have type ")), f11(v2.clone())?))));
            let a60 = f12(v2);
            let a61 = 10u64;
            if nat_eq(a60, a61) {
                Ok(v60)
            } else {
                Ok(v61)
            }
        }
        Adt3::C20(r83, r84) => {
            let v62 = (*r83).clone();
            let v63 = (*r84).clone();
            let v64: Result<(), Bytes> = f3(v0.clone(), v62.clone(), f18(v2.clone()), 0u64)?;
            let v65: Result<(), Bytes> = f3(v0, v63.clone(), f19(v2.clone()), 0u64)?;
            let v68: Result<(), Bytes> = {
                let m62 = v64;
                match m62 {
                    Ok(_) => {
                        v65
                    }
                    Err(v67) => {
                        Err::<(), Bytes>(v67)
                    }
                }
            };
            let v69: Result<(), Bytes> = Err::<(), Bytes>(f4(f5(Str::lit("the literal ")), f4(f10(Adt3::C20(std::rc::Rc::new(v62), std::rc::Rc::new(v63)))?, f4(f5(Str::lit(" does not have type ")), f11(v2.clone())?))));
            let a63 = f12(v2);
            let a64 = 11u64;
            if nat_eq(a63, a64) {
                Ok(v68)
            } else {
                Ok(v69)
            }
        }
        Adt3::C21(v70, r85) => {
            let v71 = (*r85).clone();
            let v77: Result<(), Bytes> = {
                let m65 = f20(v0.clone(), f21(v2.clone()), v70)?;
                match m65 {
                    Ok(v73) => {
                        let v74: Result<(), Bytes> = f3(v0, v71.clone(), v73.clone(), 2u64)?;
                        let v75: Result<(), Bytes> = v74;
                        let v76: Result<(), Bytes> = Err::<(), Bytes>(f4(f5(Str::lit("constructor ")), f4(f6(v70)?, f4(f5(Str::lit(" of ADT ")), f4(f6(f21(v2.clone()))?, f4(f5(Str::lit(" takes ")), f4(f6(f7(v73.clone())?)?, f4(f5(Str::lit(" fields, the literal has ")), f6(f8(v71.clone())?)?))))))));
                        let a66 = f8(v71.clone())?;
                        let a67 = f7(v73)?;
                        if nat_eq(a66, a67) {
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
            let v78: Result<(), Bytes> = Err::<(), Bytes>(f4(f5(Str::lit("the literal ")), f4(f10(Adt3::C21(v70, std::rc::Rc::new(v71)))?, f4(f5(Str::lit(" does not have type ")), f11(v2.clone())?))));
            let a68 = f12(v2);
            let a69 = 12u64;
            if nat_eq(a68, a69) {
                Ok(v77)
            } else {
                Ok(v78)
            }
        }
        Adt3::C22(_, _) => {
            Ok(Err::<(), Bytes>(f5(Str::lit("a closure has no literal form; build it with `closure`"))))
        }
        Adt3::C23 => {
            Ok(Ok::<(), Bytes>(()))
        }
        Adt3::C24(r86, r87) => {
            let v81 = (*r86).clone();
            let v82 = (*r87).clone();
            let v83: Adt0 = {
                let a70 = v3;
                let a71 = 2u64;
                if nat_eq(a70, a71) {
                    f18(v2.clone())
                } else {
                    v2.clone()
                }
            };
            let v84: Adt0 = {
                let a72 = v3;
                let a73 = 2u64;
                if nat_eq(a72, a73) {
                    f19(v2.clone())
                } else {
                    v2.clone()
                }
            };
            let v85: Result<(), Bytes> = f3(v0.clone(), v81, v83, 0u64)?;
            let v86: Result<(), Bytes> = f3(v0, v82, v84, v3)?;
            let a74 = v3;
            let a75 = 0u64;
            if nat_eq(a74, a75) {
                Ok(Ok::<(), Bytes>(()))
            } else {
                let a76 = v3;
                let a77 = 2u64;
                if if nat_eq(a76, a77) {
                    f22(v2)
                } else {
                    false
                } {
                    Ok(Ok::<(), Bytes>(()))
                } else {
                    let m78 = v85;
                    match m78 {
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

pub fn f4(v0: Bytes, v1: Bytes) -> Bytes {
    let a88 = v0;
    let a89 = v1;
    append_bytes(a88, a89)
}

pub fn f5(v0: Str) -> Bytes {
    let a90 = v0;
    utf8_encode(a90)
}

pub fn f6(v0: u64) -> R<Bytes> {
    let v1: Bytes = f5(Str::lit("0"));
    let v2: Bytes = f23({
        let a127 = f24(v0, 10000000000000000000u64);
        let a128 = {
            let a125 = f24(v0, 1000000000000000000u64);
            let a126 = {
                let a123 = f24(v0, 100000000000000000u64);
                let a124 = {
                    let a121 = f24(v0, 10000000000000000u64);
                    let a122 = {
                        let a119 = f24(v0, 1000000000000000u64);
                        let a120 = {
                            let a117 = f24(v0, 100000000000000u64);
                            let a118 = {
                                let a115 = f24(v0, 10000000000000u64);
                                let a116 = {
                                    let a113 = f24(v0, 1000000000000u64);
                                    let a114 = {
                                        let a111 = f24(v0, 100000000000u64);
                                        let a112 = {
                                            let a109 = f24(v0, 10000000000u64);
                                            let a110 = {
                                                let a107 = f24(v0, 1000000000u64);
                                                let a108 = {
                                                    let a105 = f24(v0, 100000000u64);
                                                    let a106 = {
                                                        let a103 = f24(v0, 10000000u64);
                                                        let a104 = {
                                                            let a101 = f24(v0, 1000000u64);
                                                            let a102 = {
                                                                let a99 = f24(v0, 100000u64);
                                                                let a100 = {
                                                                    let a97 = f24(v0, 10000u64);
                                                                    let a98 = {
                                                                        let a95 = f24(v0, 1000u64);
                                                                        let a96 = {
                                                                            let a93 = f24(v0, 100u64);
                                                                            let a94 = {
                                                                                let a91 = f24(v0, 10u64);
                                                                                let a92 = f24(v0, 1u64);
                                                                                append_bytes(a91, a92)
                                                                            };
                                                                            append_bytes(a93, a94)
                                                                        };
                                                                        append_bytes(a95, a96)
                                                                    };
                                                                    append_bytes(a97, a98)
                                                                };
                                                                append_bytes(a99, a100)
                                                            };
                                                            append_bytes(a101, a102)
                                                        };
                                                        append_bytes(a103, a104)
                                                    };
                                                    append_bytes(a105, a106)
                                                };
                                                append_bytes(a107, a108)
                                            };
                                            append_bytes(a109, a110)
                                        };
                                        append_bytes(a111, a112)
                                    };
                                    append_bytes(a113, a114)
                                };
                                append_bytes(a115, a116)
                            };
                            append_bytes(a117, a118)
                        };
                        append_bytes(a119, a120)
                    };
                    append_bytes(a121, a122)
                };
                append_bytes(a123, a124)
            };
            append_bytes(a125, a126)
        };
        append_bytes(a127, a128)
    })?;
    let a129 = v0;
    let a130 = 0u64;
    if nat_eq(a129, a130) {
        Ok(v1)
    } else {
        Ok(v2)
    }
}

pub fn f7(v0: Adt0) -> R<u64> {
    let m131 = v0;
    match m131 {
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
        Adt0::C15(_, r134) => {
            let v12 = (*r134).clone();
            let v13: u64 = f7(v12)?;
            let a132 = 1u64;
            let a133 = v13;
            nat_add(a132, a133)
        }
    }
}

pub fn f8(v0: Adt3) -> R<u64> {
    let m135 = v0;
    match m135 {
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
        Adt3::C24(_, r138) => {
            let v26 = (*r138).clone();
            let v27: u64 = f8(v26)?;
            let a136 = 1u64;
            let a137 = v27;
            nat_add(a136, a137)
        }
    }
}

pub fn f9(p144: Adt0, p145: u64) -> Option<Adt0> {
    let m143 = (p144, p145);
    {
        let mut m143 = m143;
        loop {
            let step = {
                let (v0, v1) = m143;
                let m139 = v0;
                match m139 {
                    Adt0::C0 => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C1 => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C2 => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C3 => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C4(_) => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C5 => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C6 => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C7 => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C8(_) => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C9(_, _) => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C10(_) => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C11(_, _) => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C12(_) => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C13(_, _) => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C14 => {
                        Err::<(Adt0, u64), Option<Adt0>>(None::<Adt0>)
                    }
                    Adt0::C15(r141, r142) => {
                        let v12 = (*r141).clone();
                        let v13 = (*r142).clone();
                        let m140 = v1;
                        if m140 == 0 {
                            Err::<(Adt0, u64), Option<Adt0>>(Some(v12))
                        } else {
                            let v14 = m140 - 1;
                            Ok::<(Adt0, u64), Option<Adt0>>((v13, v14))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m143 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f10(v0: Adt3) -> R<Bytes> {
    let m146 = v0;
    match m146 {
        Adt3::C0 => {
            Ok(f5(Str::lit("Unit")))
        }
        Adt3::C1(v1) => {
            Ok(f4(f5(Str::lit("Bool { value: ")), f4(f25(v1), f5(Str::lit(" }")))))
        }
        Adt3::C2(v2) => {
            Ok(f4(f5(Str::lit("Nat { value: ")), f4(f26(v2)?, f5(Str::lit(" }")))))
        }
        Adt3::C3(v3) => {
            Ok(f4(f5(Str::lit("Int { value: ")), f4(f26(v3)?, f5(Str::lit(" }")))))
        }
        Adt3::C4(v4) => {
            Ok(f4(f5(Str::lit("U8 { value: ")), f4(f26(v4)?, f5(Str::lit(" }")))))
        }
        Adt3::C5(v5) => {
            Ok(f4(f5(Str::lit("U16 { value: ")), f4(f26(v5)?, f5(Str::lit(" }")))))
        }
        Adt3::C6(v6) => {
            Ok(f4(f5(Str::lit("U32 { value: ")), f4(f26(v6)?, f5(Str::lit(" }")))))
        }
        Adt3::C7(v7) => {
            Ok(f4(f5(Str::lit("U64 { value: ")), f4(f26(v7)?, f5(Str::lit(" }")))))
        }
        Adt3::C8(v8) => {
            Ok(f4(f5(Str::lit("I8 { value: ")), f4(f26(v8)?, f5(Str::lit(" }")))))
        }
        Adt3::C9(v9) => {
            Ok(f4(f5(Str::lit("I16 { value: ")), f4(f26(v9)?, f5(Str::lit(" }")))))
        }
        Adt3::C10(v10) => {
            Ok(f4(f5(Str::lit("I32 { value: ")), f4(f26(v10)?, f5(Str::lit(" }")))))
        }
        Adt3::C11(v11) => {
            Ok(f4(f5(Str::lit("I64 { value: ")), f4(f26(v11)?, f5(Str::lit(" }")))))
        }
        Adt3::C12(v12) => {
            Ok(f4(f5(Str::lit("String { value: ")), f4(f26(v12)?, f5(Str::lit(" }")))))
        }
        Adt3::C13(v13) => {
            Ok(f4(f5(Str::lit("Bytes { hex: ")), f4(f26(v13)?, f5(Str::lit(" }")))))
        }
        Adt3::C14(v14) => {
            Ok(f4(f5(Str::lit("Ordering { value: ")), f4(f27(v14), f5(Str::lit(" }")))))
        }
        Adt3::C15 => {
            Ok(f5(Str::lit("None")))
        }
        Adt3::C16(r147) => {
            let v15 = (*r147).clone();
            Ok(f4(f5(Str::lit("Some { value: ")), f4(f10(v15)?, f5(Str::lit(" }")))))
        }
        Adt3::C17(r148) => {
            let v16 = (*r148).clone();
            Ok(f4(f5(Str::lit("Ok { value: ")), f4(f10(v16)?, f5(Str::lit(" }")))))
        }
        Adt3::C18(r149) => {
            let v17 = (*r149).clone();
            Ok(f4(f5(Str::lit("Error { value: ")), f4(f10(v17)?, f5(Str::lit(" }")))))
        }
        Adt3::C19(r150) => {
            let v18 = (*r150).clone();
            Ok(f4(f5(Str::lit("List { items: [")), f4(f10(v18)?, f5(Str::lit("] }")))))
        }
        Adt3::C20(r151, r152) => {
            let v19 = (*r151).clone();
            let v20 = (*r152).clone();
            Ok(f4(f5(Str::lit("Pair { left: ")), f4(f10(v19)?, f4(f5(Str::lit(", right: ")), f4(f10(v20)?, f5(Str::lit(" }")))))))
        }
        Adt3::C21(v21, r153) => {
            let v22 = (*r153).clone();
            Ok(f4(f5(Str::lit("Adt { constructor: ")), f4(f6(v21)?, f4(f5(Str::lit(", fields: [")), f4(f10(v22)?, f5(Str::lit("] }")))))))
        }
        Adt3::C22(v23, r154) => {
            let v24 = (*r154).clone();
            Ok(f4(f5(Str::lit("Closure { function: ")), f4(f6(v23)?, f4(f5(Str::lit(", captures: [")), f4(f10(v24)?, f5(Str::lit("] }")))))))
        }
        Adt3::C23 => {
            Ok(f5(Str::lit("")))
        }
        Adt3::C24(r155, r156) => {
            let v25 = (*r155).clone();
            let v26 = (*r156).clone();
            let v27: Bytes = f10(v25)?;
            let v28: Bytes = f10(v26.clone())?;
            if f28(v26) {
                Ok(v27)
            } else {
                Ok(f4(v27, f4(f5(Str::lit(", ")), v28)))
            }
        }
    }
}

pub fn f11(v0: Adt0) -> R<Bytes> {
    let m157 = v0;
    match m157 {
        Adt0::C0 => {
            Ok(f5(Str::lit("Unit")))
        }
        Adt0::C1 => {
            Ok(f5(Str::lit("Bool")))
        }
        Adt0::C2 => {
            Ok(f5(Str::lit("Nat")))
        }
        Adt0::C3 => {
            Ok(f5(Str::lit("Int")))
        }
        Adt0::C4(v1) => {
            Ok(f4(f5(Str::lit("Fixed { width: ")), f4(f29(v1), f5(Str::lit(" }")))))
        }
        Adt0::C5 => {
            Ok(f5(Str::lit("String")))
        }
        Adt0::C6 => {
            Ok(f5(Str::lit("Bytes")))
        }
        Adt0::C7 => {
            Ok(f5(Str::lit("Ordering")))
        }
        Adt0::C8(r158) => {
            let v2 = (*r158).clone();
            Ok(f4(f5(Str::lit("Option { value: ")), f4(f11(v2)?, f5(Str::lit(" }")))))
        }
        Adt0::C9(r159, r160) => {
            let v3 = (*r159).clone();
            let v4 = (*r160).clone();
            Ok(f4(f5(Str::lit("Result { ok: ")), f4(f11(v3)?, f4(f5(Str::lit(", error: ")), f4(f11(v4)?, f5(Str::lit(" }")))))))
        }
        Adt0::C10(r161) => {
            let v5 = (*r161).clone();
            Ok(f4(f5(Str::lit("List { element: ")), f4(f11(v5)?, f5(Str::lit(" }")))))
        }
        Adt0::C11(r162, r163) => {
            let v6 = (*r162).clone();
            let v7 = (*r163).clone();
            Ok(f4(f5(Str::lit("Pair { left: ")), f4(f11(v6)?, f4(f5(Str::lit(", right: ")), f4(f11(v7)?, f5(Str::lit(" }")))))))
        }
        Adt0::C12(v8) => {
            Ok(f4(f5(Str::lit("Adt { index: ")), f4(f6(v8)?, f5(Str::lit(" }")))))
        }
        Adt0::C13(r164, r165) => {
            let v9 = (*r164).clone();
            let v10 = (*r165).clone();
            Ok(f4(f5(Str::lit("Fn { parameters: [")), f4(f11(v9)?, f4(f5(Str::lit("], result: ")), f4(f11(v10)?, f5(Str::lit(" }")))))))
        }
        Adt0::C14 => {
            Ok(f5(Str::lit("")))
        }
        Adt0::C15(r166, r167) => {
            let v11 = (*r166).clone();
            let v12 = (*r167).clone();
            let v13: Bytes = f11(v11)?;
            let v14: Bytes = f11(v12.clone())?;
            if f22(v12) {
                Ok(v13)
            } else {
                Ok(f4(v13, f4(f5(Str::lit(", ")), v14)))
            }
        }
    }
}

pub fn f12(v0: Adt0) -> u64 {
    let m168 = v0;
    match m168 {
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

pub fn f13(v0: Bytes) -> R<Result<(), Bytes>> {
    let m169 = f30(v0.clone())?;
    match m169 {
        Ok(v1) => {
            let v2: bool = f31(match v1.clone() {
                Adt8::C0(_, h170) => {
                    h170
                }
            }, f5(Str::lit("18446744073709551615")));
            if match v1 {
                Adt8::C0(h171, _) => {
                    h171
                }
            } {
                Ok(Err::<(), Bytes>(f4(f5(Str::lit("nat literal ")), f4(v0, f5(Str::lit(" is outside the 64-bit realization"))))))
            } else {
                if v2 {
                    Ok(Ok::<(), Bytes>(()))
                } else {
                    Ok(Err::<(), Bytes>(f4(f5(Str::lit("nat literal ")), f4(v0, f5(Str::lit(" is outside the 64-bit realization"))))))
                }
            }
        }
        Err(v3) => {
            Ok(Err::<(), Bytes>(v3))
        }
    }
}

pub fn f14(v0: Bytes, v1: Adt1) -> R<Result<(), Bytes>> {
    let m172 = f30(v0.clone())?;
    match m172 {
        Ok(v2) => {
            let v3: Adt8 = f32(v1.clone());
            let v4: Adt8 = f33(v1.clone());
            let v5: bool = f34(v3, v2.clone());
            let v6: bool = f34(v2, v4);
            if v5 {
                if v6 {
                    Ok(Ok::<(), Bytes>(()))
                } else {
                    Ok(Err::<(), Bytes>(f4(v0, f4(f5(Str::lit(" is outside ")), f35(v1)))))
                }
            } else {
                Ok(Err::<(), Bytes>(f4(v0, f4(f5(Str::lit(" is outside ")), f35(v1)))))
            }
        }
        Err(v7) => {
            Ok(Err::<(), Bytes>(v7))
        }
    }
}

pub fn f15(v0: Adt1) -> u64 {
    let m173 = v0;
    match m173 {
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

pub fn f16(v0: Adt0) -> Adt1 {
    let m174 = v0;
    match m174 {
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

pub fn f17(v0: Bytes) -> R<Result<(), Bytes>> {
    let a179 = {
        let a176 = {
            let a175 = v0.clone();
            length_bytes(a175)
        };
        let a177 = 2u64;
        let a178 = 0u64;
        nat_rem(a176, a177, a178)
    };
    let a180 = 0u64;
    if nat_eq(a179, a180) {
        if f36(v0.clone())? {
            Ok(Ok::<(), Bytes>(()))
        } else {
            Ok(Err::<(), Bytes>(f4(f5(Str::lit("byte literal `")), f4(v0, f5(Str::lit("` is not even-length lowercase hexadecimal"))))))
        }
    } else {
        Ok(Err::<(), Bytes>(f4(f5(Str::lit("byte literal `")), f4(v0, f5(Str::lit("` is not even-length lowercase hexadecimal"))))))
    }
}

pub fn f18(v0: Adt0) -> Adt0 {
    let m181 = f37(v0);
    match m181 {
        None => {
            Adt0::C0
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f19(v0: Adt0) -> Adt0 {
    let m182 = f38(v0);
    match m182 {
        None => {
            Adt0::C0
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f20(v0: Adt7, v1: u64, v2: u64) -> R<Result<Adt0, Bytes>> {
    let m183 = f39(v0, v1);
    match m183 {
        None => {
            Ok(Err::<Adt0, Bytes>(f40(v1)?))
        }
        Some(v3) => {
            let m184 = f9(v3, v2);
            match m184 {
                None => {
                    Ok(Err::<Adt0, Bytes>(f4(f5(Str::lit("ADT ")), f4(f6(v1)?, f4(f5(Str::lit(" has no constructor ")), f6(v2)?)))))
                }
                Some(v4) => {
                    Ok(Ok::<Adt0, Bytes>(v4))
                }
            }
        }
    }
}

pub fn f21(v0: Adt0) -> u64 {
    let m185 = v0;
    match m185 {
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

pub fn f22(v0: Adt0) -> bool {
    let a186 = f12(v0);
    let a187 = 14u64;
    nat_eq(a186, a187)
}

pub fn f23(v0: Bytes) -> R<Bytes> {
    let v1: u64 = {
        let (h188, _) = f41(Fn0::F42(v0.clone()), {
            let a189 = v0.clone();
            length_bytes(a189)
        }, 0u64)?;
        h188
    };
    Ok(f43(v0.clone(), v1, {
        let a191 = {
            let a190 = v0;
            length_bytes(a190)
        };
        let a192 = v1;
        nat_sub(a191, a192)
    }))
}

pub fn f24(v0: u64, v1: u64) -> Bytes {
    f43(f5(Str::lit("0123456789")), {
        let a196 = {
            let a193 = v0;
            let a194 = v1;
            let a195 = 0u64;
            nat_quot(a193, a194, a195)
        };
        let a197 = 10u64;
        let a198 = 0u64;
        nat_rem(a196, a197, a198)
    }, 1u64)
}

pub fn f25(v0: bool) -> Bytes {
    if v0 {
        f5(Str::lit("true"))
    } else {
        f5(Str::lit("false"))
    }
}

pub fn f26(v0: Bytes) -> R<Bytes> {
    Ok(f4(f5(Str::lit("\"")), f4({
        let (_, h199) = {
            let (h200, _) = f44(Fn1::F45(v0.clone()), {
                let a201 = v0;
                length_bytes(a201)
            }, (0u64, Bytes::lit(&[])))?;
            h200
        };
        h199
    }, f5(Str::lit("\"")))))
}

pub fn f27(v0: Adt4) -> Bytes {
    let m202 = v0;
    match m202 {
        Adt4::C0 => {
            f5(Str::lit("Lt"))
        }
        Adt4::C1 => {
            f5(Str::lit("Eq"))
        }
        Adt4::C2 => {
            f5(Str::lit("Gt"))
        }
    }
}

pub fn f28(v0: Adt3) -> bool {
    let m203 = v0;
    match m203 {
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

pub fn f29(v0: Adt1) -> Bytes {
    let m204 = v0;
    match m204 {
        Adt1::C0 => {
            f5(Str::lit("U8"))
        }
        Adt1::C1 => {
            f5(Str::lit("U16"))
        }
        Adt1::C2 => {
            f5(Str::lit("U32"))
        }
        Adt1::C3 => {
            f5(Str::lit("U64"))
        }
        Adt1::C4 => {
            f5(Str::lit("I8"))
        }
        Adt1::C5 => {
            f5(Str::lit("I16"))
        }
        Adt1::C6 => {
            f5(Str::lit("I32"))
        }
        Adt1::C7 => {
            f5(Str::lit("I64"))
        }
    }
}

pub fn f30(v0: Bytes) -> R<Result<Adt8, Bytes>> {
    let v1: bool = f46(v0.clone(), 0u64, 45u8);
    let v3: Bytes = {
        let v2: Bytes = f43(v0.clone(), 1u64, {
            let a206 = {
                let a205 = v0.clone();
                length_bytes(a205)
            };
            let a207 = 1u64;
            nat_sub(a206, a207)
        });
        if v1 {
            v2
        } else {
            v0.clone()
        }
    };
    let v4: bool = f47(v3.clone(), v0.clone())?;
    let v5: bool = f31(v3.clone(), f5(Str::lit("170141183460469231731687303715884105727")));
    let v6: bool = f31(v3.clone(), f5(Str::lit("170141183460469231731687303715884105728")));
    if v4 {
        if if v1 {
            v6
        } else {
            v5
        } {
            Ok(Ok::<Adt8, Bytes>(Adt8::C0(v1, v3)))
        } else {
            Ok(Err::<Adt8, Bytes>(f4(f48(v0), f5(Str::lit(" is outside every realized integer")))))
        }
    } else {
        Ok(Err::<Adt8, Bytes>(f4(f48(v0), f5(Str::lit(" is not a canonical decimal")))))
    }
}

pub fn f31(v0: Bytes, v1: Bytes) -> bool {
    let v2: bool = {
        let a210 = {
            let a208 = v0.clone();
            length_bytes(a208)
        };
        let a211 = {
            let a209 = v1.clone();
            length_bytes(a209)
        };
        nat_lt(a210, a211)
    };
    let v3: bool = {
        let a214 = {
            let a212 = v0.clone();
            length_bytes(a212)
        };
        let a215 = {
            let a213 = v1.clone();
            length_bytes(a213)
        };
        nat_eq(a214, a215)
    };
    let v4: bool = f49(v1, v0);
    if v2 {
        true
    } else {
        if v3 {
            let a216 = v4;
            bool_not(a216)
        } else {
            false
        }
    }
}

pub fn f32(v0: Adt1) -> Adt8 {
    let m217 = v0;
    match m217 {
        Adt1::C0 => {
            f50(false, Str::lit("0"))
        }
        Adt1::C1 => {
            f50(false, Str::lit("0"))
        }
        Adt1::C2 => {
            f50(false, Str::lit("0"))
        }
        Adt1::C3 => {
            f50(false, Str::lit("0"))
        }
        Adt1::C4 => {
            f50(true, Str::lit("128"))
        }
        Adt1::C5 => {
            f50(true, Str::lit("32768"))
        }
        Adt1::C6 => {
            f50(true, Str::lit("2147483648"))
        }
        Adt1::C7 => {
            f50(true, Str::lit("9223372036854775808"))
        }
    }
}

pub fn f33(v0: Adt1) -> Adt8 {
    let m218 = v0;
    match m218 {
        Adt1::C0 => {
            f50(false, Str::lit("255"))
        }
        Adt1::C1 => {
            f50(false, Str::lit("65535"))
        }
        Adt1::C2 => {
            f50(false, Str::lit("4294967295"))
        }
        Adt1::C3 => {
            f50(false, Str::lit("18446744073709551615"))
        }
        Adt1::C4 => {
            f50(false, Str::lit("127"))
        }
        Adt1::C5 => {
            f50(false, Str::lit("32767"))
        }
        Adt1::C6 => {
            f50(false, Str::lit("2147483647"))
        }
        Adt1::C7 => {
            f50(false, Str::lit("9223372036854775807"))
        }
    }
}

pub fn f34(v0: Adt8, v1: Adt8) -> bool {
    let v2: bool = f31(match v1.clone() {
        Adt8::C0(_, h219) => {
            h219
        }
    }, match v0.clone() {
        Adt8::C0(_, h220) => {
            h220
        }
    });
    let v3: bool = f31(match v0.clone() {
        Adt8::C0(_, h221) => {
            h221
        }
    }, match v1.clone() {
        Adt8::C0(_, h222) => {
            h222
        }
    });
    if match v0 {
        Adt8::C0(h223, _) => {
            h223
        }
    } {
        if match v1 {
            Adt8::C0(h224, _) => {
                h224
            }
        } {
            v2
        } else {
            true
        }
    } else {
        if match v1 {
            Adt8::C0(h225, _) => {
                h225
            }
        } {
            false
        } else {
            v3
        }
    }
}

pub fn f35(v0: Adt1) -> Bytes {
    let m226 = v0;
    match m226 {
        Adt1::C0 => {
            f5(Str::lit("u8"))
        }
        Adt1::C1 => {
            f5(Str::lit("u16"))
        }
        Adt1::C2 => {
            f5(Str::lit("u32"))
        }
        Adt1::C3 => {
            f5(Str::lit("u64"))
        }
        Adt1::C4 => {
            f5(Str::lit("i8"))
        }
        Adt1::C5 => {
            f5(Str::lit("i16"))
        }
        Adt1::C6 => {
            f5(Str::lit("i32"))
        }
        Adt1::C7 => {
            f5(Str::lit("i64"))
        }
    }
}

pub fn f36(v0: Bytes) -> R<bool> {
    let a230 = {
        let (h227, _) = f41(Fn0::F51(v0.clone()), {
            let a228 = v0.clone();
            length_bytes(a228)
        }, 0u64)?;
        h227
    };
    let a231 = {
        let a229 = v0;
        length_bytes(a229)
    };
    Ok(nat_eq(a230, a231))
}

pub fn f37(v0: Adt0) -> Option<Adt0> {
    let m232 = v0;
    match m232 {
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
        Adt0::C8(r233) => {
            let v2 = (*r233).clone();
            Some(v2)
        }
        Adt0::C9(r234, _) => {
            let v3 = (*r234).clone();
            Some(v3)
        }
        Adt0::C10(r235) => {
            let v5 = (*r235).clone();
            Some(v5)
        }
        Adt0::C11(r236, _) => {
            let v6 = (*r236).clone();
            Some(v6)
        }
        Adt0::C12(_) => {
            None::<Adt0>
        }
        Adt0::C13(r237, _) => {
            let v9 = (*r237).clone();
            Some(v9)
        }
        Adt0::C14 => {
            None::<Adt0>
        }
        Adt0::C15(r238, _) => {
            let v11 = (*r238).clone();
            Some(v11)
        }
    }
}

pub fn f38(v0: Adt0) -> Option<Adt0> {
    let m239 = v0;
    match m239 {
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
        Adt0::C9(_, r240) => {
            let v4 = (*r240).clone();
            Some(v4)
        }
        Adt0::C10(_) => {
            None::<Adt0>
        }
        Adt0::C11(_, r241) => {
            let v7 = (*r241).clone();
            Some(v7)
        }
        Adt0::C12(_) => {
            None::<Adt0>
        }
        Adt0::C13(_, r242) => {
            let v10 = (*r242).clone();
            Some(v10)
        }
        Adt0::C14 => {
            None::<Adt0>
        }
        Adt0::C15(_, r243) => {
            let v12 = (*r243).clone();
            Some(v12)
        }
    }
}

pub fn f39(v0: Adt7, v1: u64) -> Option<Adt0> {
    f9(match v0 {
        Adt7::C0(_, h244, _, _, _, _) => {
            h244
        }
    }, v1)
}

pub fn f40(v0: u64) -> R<Bytes> {
    Ok(f4(f5(Str::lit("ADT ")), f4(f6(v0)?, f5(Str::lit(" is not declared")))))
}

pub fn f41(v0: Fn0, p249: u64, p250: u64) -> R<(u64, bool)> {
    let m248 = (p249, p250);
    {
        let mut m248 = m248;
        loop {
            let step = {
                let (v1, v2) = m248;
                let m245 = v1;
                if m245 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m245 - 1;
                    let m247 = {
                        let c246: Fn0 = v0.clone();
                        c246.apply(v2)?
                    };
                    match m247 {
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
                Ok(next) => { m248 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f42(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f52(v0, v1)
}

pub fn f43(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m254 = {
        let a251 = v0;
        let a252 = v1;
        let a253 = v2;
        slice_bytes(a251, a252, a253)
    };
    match m254 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f44(v0: Fn1, p259: u64, p260: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m258 = (p259, p260);
    {
        let mut m258 = m258;
        loop {
            let step = {
                let (v1, v2) = m258;
                let m255 = v1;
                if m255 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m255 - 1;
                    let m257 = {
                        let c256: Fn1 = v0.clone();
                        c256.apply(v2.clone())?
                    };
                    match m257 {
                        None => {
                            Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m258 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f45(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f53(v0, v1)
}

pub fn f46(v0: Bytes, v1: u64, v2: u8) -> bool {
    let m261 = f54(v0, v1);
    match m261 {
        None => {
            false
        }
        Some(v3) => {
            let a262 = v3;
            let a263 = v2;
            equal(a262, a263)
        }
    }
}

pub fn f47(v0: Bytes, v1: Bytes) -> R<bool> {
    let a265 = 0u64;
    let a266 = {
        let a264 = v0.clone();
        length_bytes(a264)
    };
    if nat_lt(a265, a266) {
        if f55(v0.clone())? {
            let a267 = v0.clone();
            let a268 = f5(Str::lit("0"));
            if equal(a267, a268) {
                let a271 = {
                    let a269 = v1;
                    let a270 = f5(Str::lit("-0"));
                    equal(a269, a270)
                };
                Ok(bool_not(a271))
            } else {
                let a272 = f46(v0, 0u64, 48u8);
                Ok(bool_not(a272))
            }
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f48(v0: Bytes) -> Bytes {
    f4(f5(Str::lit("`")), f4(v0, f5(Str::lit("`"))))
}

pub fn f49(v0: Bytes, v1: Bytes) -> bool {
    let a277 = {
        let a273 = v0;
        let a274 = v1;
        compare_bytes(a273, a274)
    };
    let a278 = {
        let a275 = Bytes::lit(&[]);
        let a276 = Bytes::lit(&[0x01]);
        compare_bytes(a275, a276)
    };
    equal(a277, a278)
}

pub fn f50(v0: bool, v1: Str) -> Adt8 {
    Adt8::C0(v0, f5(v1))
}

pub fn f51(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f56(v0, v1)
}

pub fn f52(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f46(v0, v1, 48u8) {
        Ok(Some({
            let a279 = v1;
            let a280 = 1u64;
            nat_add(a279, a280)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f53(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let v2: u64 = {
        let (h281, _) = v1;
        h281
    };
    let v3: u64 = f57(v0.clone(), v2)?;
    let a282 = v3;
    let a283 = 256u64;
    if nat_eq(a282, a283) {
        Ok(None::<(u64, Bytes)>)
    } else {
        let v4: (u64, Bytes) = f58(v0, v2, v3)?;
        Ok(Some(({
            let a285 = v2;
            let a286 = {
                let (h284, _) = v4;
                h284
            };
            nat_add(a285, a286)?
        }, f4({
            let (_, h287) = v1;
            h287
        }, {
            let (_, h288) = v4;
            h288
        }))))
    }
}

pub fn f54(v0: Bytes, v1: u64) -> Option<u8> {
    let a289 = v0;
    let a290 = v1;
    index_bytes(a289, a290)
}

pub fn f55(v0: Bytes) -> R<bool> {
    let a294 = {
        let (h291, _) = f41(Fn0::F59(v0.clone()), {
            let a292 = v0.clone();
            length_bytes(a292)
        }, 0u64)?;
        h291
    };
    let a295 = {
        let a293 = v0;
        length_bytes(a293)
    };
    Ok(nat_eq(a294, a295))
}

pub fn f56(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f60(v0, v1)? {
        Ok(Some({
            let a296 = v1;
            let a297 = 1u64;
            nat_add(a296, a297)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f57(v0: Bytes, v1: u64) -> R<u64> {
    let m298 = f54(v0, v1);
    match m298 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a343 = {
                let a319 = {
                    let a307 = {
                        let a301 = {
                            let a299 = v3;
                            let a300 = 128u8;
                            fixed_u8::bit_and(a299, a300)
                        };
                        let a302 = 128u8;
                        if equal(a301, a302) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a308 = {
                        let a305 = {
                            let a303 = v3;
                            let a304 = 64u8;
                            fixed_u8::bit_and(a303, a304)
                        };
                        let a306 = 64u8;
                        if equal(a305, a306) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a307, a308)?
                };
                let a320 = {
                    let a317 = {
                        let a311 = {
                            let a309 = v3;
                            let a310 = 32u8;
                            fixed_u8::bit_and(a309, a310)
                        };
                        let a312 = 32u8;
                        if equal(a311, a312) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a318 = {
                        let a315 = {
                            let a313 = v3;
                            let a314 = 16u8;
                            fixed_u8::bit_and(a313, a314)
                        };
                        let a316 = 16u8;
                        if equal(a315, a316) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a317, a318)?
                };
                nat_add(a319, a320)?
            };
            let a344 = {
                let a341 = {
                    let a329 = {
                        let a323 = {
                            let a321 = v3;
                            let a322 = 8u8;
                            fixed_u8::bit_and(a321, a322)
                        };
                        let a324 = 8u8;
                        if equal(a323, a324) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a330 = {
                        let a327 = {
                            let a325 = v3;
                            let a326 = 4u8;
                            fixed_u8::bit_and(a325, a326)
                        };
                        let a328 = 4u8;
                        if equal(a327, a328) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a329, a330)?
                };
                let a342 = {
                    let a339 = {
                        let a333 = {
                            let a331 = v3;
                            let a332 = 2u8;
                            fixed_u8::bit_and(a331, a332)
                        };
                        let a334 = 2u8;
                        if equal(a333, a334) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a340 = {
                        let a337 = {
                            let a335 = v3;
                            let a336 = 1u8;
                            fixed_u8::bit_and(a335, a336)
                        };
                        let a338 = 1u8;
                        if equal(a337, a338) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a339, a340)?
                };
                nat_add(a341, a342)?
            };
            nat_add(a343, a344)
        }
    }
}

pub fn f58(v0: Bytes, v1: u64, v2: u64) -> R<(u64, Bytes)> {
    let v3: u64 = f57(v0.clone(), {
        let a345 = v1;
        let a346 = 1u64;
        nat_add(a345, a346)?
    })?;
    let a347 = v2;
    let a348 = 34u64;
    if nat_eq(a347, a348) {
        Ok((1u64, f5(Str::lit("\\\""))))
    } else {
        let a349 = v2;
        let a350 = 92u64;
        if nat_eq(a349, a350) {
            Ok((1u64, f5(Str::lit("\\\\"))))
        } else {
            let a351 = v2;
            let a352 = 10u64;
            if nat_eq(a351, a352) {
                Ok((1u64, f5(Str::lit("\\n"))))
            } else {
                let a353 = v2;
                let a354 = 13u64;
                if nat_eq(a353, a354) {
                    Ok((1u64, f5(Str::lit("\\r"))))
                } else {
                    let a355 = v2;
                    let a356 = 9u64;
                    if nat_eq(a355, a356) {
                        Ok((1u64, f5(Str::lit("\\t"))))
                    } else {
                        let a357 = v2;
                        let a358 = 0u64;
                        if nat_eq(a357, a358) {
                            Ok((1u64, f5(Str::lit("\\0"))))
                        } else {
                            let a359 = v2;
                            let a360 = 32u64;
                            if nat_lt(a359, a360) {
                                Ok((1u64, f61(v2)))
                            } else {
                                let a361 = v2;
                                let a362 = 127u64;
                                if nat_eq(a361, a362) {
                                    Ok((1u64, f61(v2)))
                                } else {
                                    let a363 = v2;
                                    let a364 = 194u64;
                                    if nat_eq(a363, a364) {
                                        let a365 = 128u64;
                                        let a366 = v3;
                                        if nat_le(a365, a366) {
                                            let a367 = v3;
                                            let a368 = 159u64;
                                            if nat_le(a367, a368) {
                                                Ok((2u64, f61(v3)))
                                            } else {
                                                Ok((1u64, f43(v0, v1, 1u64)))
                                            }
                                        } else {
                                            Ok((1u64, f43(v0, v1, 1u64)))
                                        }
                                    } else {
                                        Ok((1u64, f43(v0, v1, 1u64)))
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

pub fn f59(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f62(v0, v1)
}

pub fn f60(v0: Bytes, v1: u64) -> R<bool> {
    let v2: u64 = f57(v0, v1)?;
    let a369 = 47u64;
    let a370 = v2;
    if nat_lt(a369, a370) {
        let a371 = v2;
        let a372 = 58u64;
        if nat_lt(a371, a372) {
            Ok(true)
        } else {
            let a373 = 96u64;
            let a374 = v2;
            if nat_lt(a373, a374) {
                let a375 = v2;
                let a376 = 103u64;
                Ok(nat_lt(a375, a376))
            } else {
                Ok(false)
            }
        }
    } else {
        Ok(false)
    }
}

pub fn f61(v0: u64) -> Bytes {
    f4(f5(Str::lit("\\u{")), f4(f63(v0), f5(Str::lit("}"))))
}

pub fn f62(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f64(v0, v1)? {
        Ok(Some({
            let a377 = v1;
            let a378 = 1u64;
            nat_add(a377, a378)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f63(v0: u64) -> Bytes {
    let v1: Bytes = f65(v0);
    let v2: Bytes = f4(f65({
        let a379 = v0;
        let a380 = 16u64;
        let a381 = 0u64;
        nat_quot(a379, a380, a381)
    }), f65({
        let a382 = v0;
        let a383 = 16u64;
        let a384 = 0u64;
        nat_rem(a382, a383, a384)
    }));
    let a385 = v0;
    let a386 = 16u64;
    if nat_lt(a385, a386) {
        v1
    } else {
        v2
    }
}

pub fn f64(v0: Bytes, v1: u64) -> R<bool> {
    let v2: u64 = f57(v0, v1)?;
    let a387 = 47u64;
    let a388 = v2;
    if nat_lt(a387, a388) {
        let a389 = v2;
        let a390 = 58u64;
        Ok(nat_lt(a389, a390))
    } else {
        Ok(false)
    }
}

pub fn f65(v0: u64) -> Bytes {
    f43(f5(Str::lit("0123456789abcdef")), v0, 1u64)
}
