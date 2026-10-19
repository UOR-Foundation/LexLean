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
    C0(u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt2 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32, Adt1, u64),
}

#[derive(Clone)]
pub enum Fn0 {
    F2(Bytes, u64, u64),
}

impl Fn0 {
    pub fn apply(&self, p0: (u64, Adt0)) -> R<(u64, Adt0)> {
        match self {
            Fn0::F2(k0, k1, k2) => f2(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F7,
}

impl Fn1 {
    pub fn apply(&self, p0: Adt2) -> R<Adt2> {
        match self {
            Fn1::F7 => f7(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F23,
}

impl Fn2 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn2::F23 => f23(p0),
        }
    }
}

pub fn f0(v0: Bytes) -> R<Adt0> {
    let v1: u64 = {
        let a1 = v0.clone();
        length_bytes(a1)
    };
    let v2: u64 = {
        let a7 = {
            let a4 = {
                let a2 = v1;
                let a3 = 72u64;
                nat_add(a2, a3)?
            };
            let a5 = 64u64;
            let a6 = 0u64;
            nat_quot(a4, a5, a6)
        };
        let a8 = 64u64;
        nat_mul(a7, a8)?
    };
    let (_, h9) = f1(Fn0::F2(v0, v1, v2), {
        let a10 = v2;
        let a11 = 64u64;
        let a12 = 0u64;
        nat_quot(a10, a11, a12)
    }, (0u64, f3()))?;
    Ok(h9)
}

pub fn f1(v0: Fn0, p16: u64, p17: (u64, Adt0)) -> R<(u64, Adt0)> {
    let m15 = (p16, p17);
    {
        let mut m15 = m15;
        loop {
            let step = {
                let (v1, v2) = m15;
                let m13 = v1;
                if m13 == 0 {
                    Err::<(u64, (u64, Adt0)), R<(u64, Adt0)>>(Ok(v2))
                } else {
                    let v3 = m13 - 1;
                    Ok::<(u64, (u64, Adt0)), R<(u64, Adt0)>>((v3, {
                        let c14: Fn0 = v0.clone();
                        c14.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m15 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f2(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt0)) -> R<(u64, Adt0)> {
    Ok(({
        let a19 = {
            let (h18, _) = v3.clone();
            h18
        };
        let a20 = 64u64;
        nat_add(a19, a20)?
    }, f4({
        let (_, h21) = v3;
        h21
    }, f5(v0, v1, v2, {
        let (h22, _) = v3;
        h22
    })?)?))
}

pub fn f3() -> Adt0 {
    Adt0::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f4(v0: Adt0, v1: Adt1) -> R<Adt0> {
    let v2: Adt2 = f6(Fn1::F7, 64u64, Adt2::C0(match v0.clone() {
        Adt0::C0(h23, _, _, _, _, _, _, _) => {
            h23
        }
    }, match v0.clone() {
        Adt0::C0(_, h24, _, _, _, _, _, _) => {
            h24
        }
    }, match v0.clone() {
        Adt0::C0(_, _, h25, _, _, _, _, _) => {
            h25
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, h26, _, _, _, _) => {
            h26
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, h27, _, _, _) => {
            h27
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, h28, _, _) => {
            h28
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, h29, _) => {
            h29
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, h30) => {
            h30
        }
    }, v1, 0u64))?;
    Ok(Adt0::C0(f8(match v0.clone() {
        Adt0::C0(h31, _, _, _, _, _, _, _) => {
            h31
        }
    }, match v2.clone() {
        Adt2::C0(h32, _, _, _, _, _, _, _, _, _) => {
            h32
        }
    }), f8(match v0.clone() {
        Adt0::C0(_, h33, _, _, _, _, _, _) => {
            h33
        }
    }, match v2.clone() {
        Adt2::C0(_, h34, _, _, _, _, _, _, _, _) => {
            h34
        }
    }), f8(match v0.clone() {
        Adt0::C0(_, _, h35, _, _, _, _, _) => {
            h35
        }
    }, match v2.clone() {
        Adt2::C0(_, _, h36, _, _, _, _, _, _, _) => {
            h36
        }
    }), f8(match v0.clone() {
        Adt0::C0(_, _, _, h37, _, _, _, _) => {
            h37
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, h38, _, _, _, _, _, _) => {
            h38
        }
    }), f8(match v0.clone() {
        Adt0::C0(_, _, _, _, h39, _, _, _) => {
            h39
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, _, h40, _, _, _, _, _) => {
            h40
        }
    }), f8(match v0.clone() {
        Adt0::C0(_, _, _, _, _, h41, _, _) => {
            h41
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, _, _, h42, _, _, _, _) => {
            h42
        }
    }), f8(match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, h43, _) => {
            h43
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, _, _, _, h44, _, _, _) => {
            h44
        }
    }), f8(match v0 {
        Adt0::C0(_, _, _, _, _, _, _, h45) => {
            h45
        }
    }, match v2 {
        Adt2::C0(_, _, _, _, _, _, _, h46, _, _) => {
            h46
        }
    })))
}

pub fn f5(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt1> {
    Ok(Adt1::C0(f9(v0.clone(), v1, v2, v3)?, f9(v0.clone(), v1, v2, {
        let a47 = v3;
        let a48 = 4u64;
        nat_add(a47, a48)?
    })?, f9(v0.clone(), v1, v2, {
        let a49 = v3;
        let a50 = 8u64;
        nat_add(a49, a50)?
    })?, f9(v0.clone(), v1, v2, {
        let a51 = v3;
        let a52 = 12u64;
        nat_add(a51, a52)?
    })?, f9(v0.clone(), v1, v2, {
        let a53 = v3;
        let a54 = 16u64;
        nat_add(a53, a54)?
    })?, f9(v0.clone(), v1, v2, {
        let a55 = v3;
        let a56 = 20u64;
        nat_add(a55, a56)?
    })?, f9(v0.clone(), v1, v2, {
        let a57 = v3;
        let a58 = 24u64;
        nat_add(a57, a58)?
    })?, f9(v0.clone(), v1, v2, {
        let a59 = v3;
        let a60 = 28u64;
        nat_add(a59, a60)?
    })?, f9(v0.clone(), v1, v2, {
        let a61 = v3;
        let a62 = 32u64;
        nat_add(a61, a62)?
    })?, f9(v0.clone(), v1, v2, {
        let a63 = v3;
        let a64 = 36u64;
        nat_add(a63, a64)?
    })?, f9(v0.clone(), v1, v2, {
        let a65 = v3;
        let a66 = 40u64;
        nat_add(a65, a66)?
    })?, f9(v0.clone(), v1, v2, {
        let a67 = v3;
        let a68 = 44u64;
        nat_add(a67, a68)?
    })?, f9(v0.clone(), v1, v2, {
        let a69 = v3;
        let a70 = 48u64;
        nat_add(a69, a70)?
    })?, f9(v0.clone(), v1, v2, {
        let a71 = v3;
        let a72 = 52u64;
        nat_add(a71, a72)?
    })?, f9(v0.clone(), v1, v2, {
        let a73 = v3;
        let a74 = 56u64;
        nat_add(a73, a74)?
    })?, f9(v0, v1, v2, {
        let a75 = v3;
        let a76 = 60u64;
        nat_add(a75, a76)?
    })?))
}

pub fn f6(v0: Fn1, p80: u64, p81: Adt2) -> R<Adt2> {
    let m79 = (p80, p81);
    {
        let mut m79 = m79;
        loop {
            let step = {
                let (v1, v2) = m79;
                let m77 = v1;
                if m77 == 0 {
                    Err::<(u64, Adt2), R<Adt2>>(Ok(v2))
                } else {
                    let v3 = m77 - 1;
                    Ok::<(u64, Adt2), R<Adt2>>((v3, {
                        let c78: Fn1 = v0.clone();
                        c78.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m79 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f7(v0: Adt2) -> R<Adt2> {
    f10(v0)
}

pub fn f8(v0: u32, v1: u32) -> u32 {
    let m84 = {
        let a82 = f11(v0);
        let a83 = f11(v1);
        fixed_u64::checked_add(a82, a83)
    };
    match m84 {
        None => {
            0u32
        }
        Some(v2) => {
            f12(v2)
        }
    }
}

pub fn f9(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u32> {
    let a87 = {
        let a85 = v3;
        let a86 = 8u64;
        nat_add(a85, a86)?
    };
    let a88 = v2;
    if nat_eq(a87, a88) {
        Ok(f13({
            let a91 = {
                let a89 = v1;
                let a90 = 8u64;
                nat_mul(a89, a90)?
            };
            let a92 = 4294967296u64;
            let a93 = 0u64;
            nat_quot(a91, a92, a93)
        }))
    } else {
        let a96 = {
            let a94 = v3;
            let a95 = 4u64;
            nat_add(a94, a95)?
        };
        let a97 = v2;
        if nat_eq(a96, a97) {
            Ok(f13({
                let a100 = {
                    let a98 = v1;
                    let a99 = 8u64;
                    nat_mul(a98, a99)?
                };
                let a101 = 4294967296u64;
                let a102 = 0u64;
                nat_rem(a100, a101, a102)
            }))
        } else {
            Ok(f14(f15(v0.clone(), v1, v3), f15(v0.clone(), v1, {
                let a103 = v3;
                let a104 = 1u64;
                nat_add(a103, a104)?
            }), f15(v0.clone(), v1, {
                let a105 = v3;
                let a106 = 2u64;
                nat_add(a105, a106)?
            }), f15(v0, v1, {
                let a107 = v3;
                let a108 = 3u64;
                nat_add(a107, a108)?
            })))
        }
    }
}

pub fn f10(v0: Adt2) -> R<Adt2> {
    let v1: u32 = f16(match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h109) => {
            h109
        }
    });
    let v2: u32 = f8(match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h110, _, _) => {
            h110
        }
    }, f8(f17(match v0.clone() {
        Adt2::C0(_, _, _, _, h111, _, _, _, _, _) => {
            h111
        }
    }), f8(f18(match v0.clone() {
        Adt2::C0(_, _, _, _, h112, _, _, _, _, _) => {
            h112
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h113, _, _, _, _) => {
            h113
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h114, _, _, _) => {
            h114
        }
    }), f8(v1, {
        let m117 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, h116, _) => {
                h116
            }
        };
        match m117 {
            Adt1::C0(h115, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h115
            }
        }
    }))));
    let v3: u32 = f8(f19(match v0.clone() {
        Adt2::C0(h118, _, _, _, _, _, _, _, _, _) => {
            h118
        }
    }), f20(match v0.clone() {
        Adt2::C0(h119, _, _, _, _, _, _, _, _, _) => {
            h119
        }
    }, match v0.clone() {
        Adt2::C0(_, h120, _, _, _, _, _, _, _, _) => {
            h120
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h121, _, _, _, _, _, _, _) => {
            h121
        }
    }));
    Ok(Adt2::C0(f8(v2, v3), match v0.clone() {
        Adt2::C0(h122, _, _, _, _, _, _, _, _, _) => {
            h122
        }
    }, match v0.clone() {
        Adt2::C0(_, h123, _, _, _, _, _, _, _, _) => {
            h123
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h124, _, _, _, _, _, _, _) => {
            h124
        }
    }, f8(match v0.clone() {
        Adt2::C0(_, _, _, h125, _, _, _, _, _, _) => {
            h125
        }
    }, v2), match v0.clone() {
        Adt2::C0(_, _, _, _, h126, _, _, _, _, _) => {
            h126
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h127, _, _, _, _) => {
            h127
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h128, _, _, _) => {
            h128
        }
    }, f21(match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, h129, _) => {
            h129
        }
    }), {
        let a131 = match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h130) => {
                h130
            }
        };
        let a132 = 1u64;
        nat_add(a131, a132)?
    }))
}

pub fn f11(v0: u32) -> u64 {
    let m134 = {
        let a133 = v0;
        fixed_u64::convert(i128::from(a133))
    };
    match m134 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f12(v0: u64) -> u32 {
    let m138 = {
        let a137 = {
            let a135 = v0;
            let a136 = 4294967295u64;
            fixed_u64::bit_and(a135, a136)
        };
        fixed_u32::convert(i128::from(a137))
    };
    match m138 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f13(v0: u64) -> u32 {
    let (_, h139) = {
        let (_, h140) = f22(Fn2::F23, 32u64, (v0, (1u32, 0u32)));
        h140
    };
    h139
}

pub fn f14(v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
    let a145 = f24(v0, 24u32);
    let a146 = {
        let a143 = f24(v1, 16u32);
        let a144 = {
            let a141 = f24(v2, 8u32);
            let a142 = v3;
            fixed_u32::bit_or(a141, a142)
        };
        fixed_u32::bit_or(a143, a144)
    };
    fixed_u32::bit_or(a145, a146)
}

pub fn f15(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a147 = v2;
    let a148 = v1;
    if nat_lt(a147, a148) {
        let m151 = {
            let a149 = v0;
            let a150 = v2;
            index_bytes(a149, a150)
        };
        match m151 {
            None => {
                0u32
            }
            Some(v3) => {
                let m153 = {
                    let a152 = v3;
                    fixed_u32::convert(i128::from(a152))
                };
                match m153 {
                    None => {
                        0u32
                    }
                    Some(v4) => {
                        v4
                    }
                }
            }
        }
    } else {
        let a154 = v2;
        let a155 = v1;
        if nat_eq(a154, a155) {
            128u32
        } else {
            0u32
        }
    }
}

pub fn f16(v0: u64) -> u32 {
    let a156 = v0;
    let a157 = 32u64;
    if nat_lt(a156, a157) {
        let a158 = v0;
        let a159 = 16u64;
        if nat_lt(a158, a159) {
            let a160 = v0;
            let a161 = 8u64;
            if nat_lt(a160, a161) {
                let a162 = v0;
                let a163 = 4u64;
                if nat_lt(a162, a163) {
                    let a164 = v0;
                    let a165 = 2u64;
                    if nat_lt(a164, a165) {
                        let a166 = v0;
                        let a167 = 1u64;
                        if nat_lt(a166, a167) {
                            1116352408u32
                        } else {
                            1899447441u32
                        }
                    } else {
                        let a168 = v0;
                        let a169 = 3u64;
                        if nat_lt(a168, a169) {
                            3049323471u32
                        } else {
                            3921009573u32
                        }
                    }
                } else {
                    let a170 = v0;
                    let a171 = 6u64;
                    if nat_lt(a170, a171) {
                        let a172 = v0;
                        let a173 = 5u64;
                        if nat_lt(a172, a173) {
                            961987163u32
                        } else {
                            1508970993u32
                        }
                    } else {
                        let a174 = v0;
                        let a175 = 7u64;
                        if nat_lt(a174, a175) {
                            2453635748u32
                        } else {
                            2870763221u32
                        }
                    }
                }
            } else {
                let a176 = v0;
                let a177 = 12u64;
                if nat_lt(a176, a177) {
                    let a178 = v0;
                    let a179 = 10u64;
                    if nat_lt(a178, a179) {
                        let a180 = v0;
                        let a181 = 9u64;
                        if nat_lt(a180, a181) {
                            3624381080u32
                        } else {
                            310598401u32
                        }
                    } else {
                        let a182 = v0;
                        let a183 = 11u64;
                        if nat_lt(a182, a183) {
                            607225278u32
                        } else {
                            1426881987u32
                        }
                    }
                } else {
                    let a184 = v0;
                    let a185 = 14u64;
                    if nat_lt(a184, a185) {
                        let a186 = v0;
                        let a187 = 13u64;
                        if nat_lt(a186, a187) {
                            1925078388u32
                        } else {
                            2162078206u32
                        }
                    } else {
                        let a188 = v0;
                        let a189 = 15u64;
                        if nat_lt(a188, a189) {
                            2614888103u32
                        } else {
                            3248222580u32
                        }
                    }
                }
            }
        } else {
            let a190 = v0;
            let a191 = 24u64;
            if nat_lt(a190, a191) {
                let a192 = v0;
                let a193 = 20u64;
                if nat_lt(a192, a193) {
                    let a194 = v0;
                    let a195 = 18u64;
                    if nat_lt(a194, a195) {
                        let a196 = v0;
                        let a197 = 17u64;
                        if nat_lt(a196, a197) {
                            3835390401u32
                        } else {
                            4022224774u32
                        }
                    } else {
                        let a198 = v0;
                        let a199 = 19u64;
                        if nat_lt(a198, a199) {
                            264347078u32
                        } else {
                            604807628u32
                        }
                    }
                } else {
                    let a200 = v0;
                    let a201 = 22u64;
                    if nat_lt(a200, a201) {
                        let a202 = v0;
                        let a203 = 21u64;
                        if nat_lt(a202, a203) {
                            770255983u32
                        } else {
                            1249150122u32
                        }
                    } else {
                        let a204 = v0;
                        let a205 = 23u64;
                        if nat_lt(a204, a205) {
                            1555081692u32
                        } else {
                            1996064986u32
                        }
                    }
                }
            } else {
                let a206 = v0;
                let a207 = 28u64;
                if nat_lt(a206, a207) {
                    let a208 = v0;
                    let a209 = 26u64;
                    if nat_lt(a208, a209) {
                        let a210 = v0;
                        let a211 = 25u64;
                        if nat_lt(a210, a211) {
                            2554220882u32
                        } else {
                            2821834349u32
                        }
                    } else {
                        let a212 = v0;
                        let a213 = 27u64;
                        if nat_lt(a212, a213) {
                            2952996808u32
                        } else {
                            3210313671u32
                        }
                    }
                } else {
                    let a214 = v0;
                    let a215 = 30u64;
                    if nat_lt(a214, a215) {
                        let a216 = v0;
                        let a217 = 29u64;
                        if nat_lt(a216, a217) {
                            3336571891u32
                        } else {
                            3584528711u32
                        }
                    } else {
                        let a218 = v0;
                        let a219 = 31u64;
                        if nat_lt(a218, a219) {
                            113926993u32
                        } else {
                            338241895u32
                        }
                    }
                }
            }
        }
    } else {
        let a220 = v0;
        let a221 = 48u64;
        if nat_lt(a220, a221) {
            let a222 = v0;
            let a223 = 40u64;
            if nat_lt(a222, a223) {
                let a224 = v0;
                let a225 = 36u64;
                if nat_lt(a224, a225) {
                    let a226 = v0;
                    let a227 = 34u64;
                    if nat_lt(a226, a227) {
                        let a228 = v0;
                        let a229 = 33u64;
                        if nat_lt(a228, a229) {
                            666307205u32
                        } else {
                            773529912u32
                        }
                    } else {
                        let a230 = v0;
                        let a231 = 35u64;
                        if nat_lt(a230, a231) {
                            1294757372u32
                        } else {
                            1396182291u32
                        }
                    }
                } else {
                    let a232 = v0;
                    let a233 = 38u64;
                    if nat_lt(a232, a233) {
                        let a234 = v0;
                        let a235 = 37u64;
                        if nat_lt(a234, a235) {
                            1695183700u32
                        } else {
                            1986661051u32
                        }
                    } else {
                        let a236 = v0;
                        let a237 = 39u64;
                        if nat_lt(a236, a237) {
                            2177026350u32
                        } else {
                            2456956037u32
                        }
                    }
                }
            } else {
                let a238 = v0;
                let a239 = 44u64;
                if nat_lt(a238, a239) {
                    let a240 = v0;
                    let a241 = 42u64;
                    if nat_lt(a240, a241) {
                        let a242 = v0;
                        let a243 = 41u64;
                        if nat_lt(a242, a243) {
                            2730485921u32
                        } else {
                            2820302411u32
                        }
                    } else {
                        let a244 = v0;
                        let a245 = 43u64;
                        if nat_lt(a244, a245) {
                            3259730800u32
                        } else {
                            3345764771u32
                        }
                    }
                } else {
                    let a246 = v0;
                    let a247 = 46u64;
                    if nat_lt(a246, a247) {
                        let a248 = v0;
                        let a249 = 45u64;
                        if nat_lt(a248, a249) {
                            3516065817u32
                        } else {
                            3600352804u32
                        }
                    } else {
                        let a250 = v0;
                        let a251 = 47u64;
                        if nat_lt(a250, a251) {
                            4094571909u32
                        } else {
                            275423344u32
                        }
                    }
                }
            }
        } else {
            let a252 = v0;
            let a253 = 56u64;
            if nat_lt(a252, a253) {
                let a254 = v0;
                let a255 = 52u64;
                if nat_lt(a254, a255) {
                    let a256 = v0;
                    let a257 = 50u64;
                    if nat_lt(a256, a257) {
                        let a258 = v0;
                        let a259 = 49u64;
                        if nat_lt(a258, a259) {
                            430227734u32
                        } else {
                            506948616u32
                        }
                    } else {
                        let a260 = v0;
                        let a261 = 51u64;
                        if nat_lt(a260, a261) {
                            659060556u32
                        } else {
                            883997877u32
                        }
                    }
                } else {
                    let a262 = v0;
                    let a263 = 54u64;
                    if nat_lt(a262, a263) {
                        let a264 = v0;
                        let a265 = 53u64;
                        if nat_lt(a264, a265) {
                            958139571u32
                        } else {
                            1322822218u32
                        }
                    } else {
                        let a266 = v0;
                        let a267 = 55u64;
                        if nat_lt(a266, a267) {
                            1537002063u32
                        } else {
                            1747873779u32
                        }
                    }
                }
            } else {
                let a268 = v0;
                let a269 = 60u64;
                if nat_lt(a268, a269) {
                    let a270 = v0;
                    let a271 = 58u64;
                    if nat_lt(a270, a271) {
                        let a272 = v0;
                        let a273 = 57u64;
                        if nat_lt(a272, a273) {
                            1955562222u32
                        } else {
                            2024104815u32
                        }
                    } else {
                        let a274 = v0;
                        let a275 = 59u64;
                        if nat_lt(a274, a275) {
                            2227730452u32
                        } else {
                            2361852424u32
                        }
                    }
                } else {
                    let a276 = v0;
                    let a277 = 62u64;
                    if nat_lt(a276, a277) {
                        let a278 = v0;
                        let a279 = 61u64;
                        if nat_lt(a278, a279) {
                            2428436474u32
                        } else {
                            2756734187u32
                        }
                    } else {
                        let a280 = v0;
                        let a281 = 63u64;
                        if nat_lt(a280, a281) {
                            3204031479u32
                        } else {
                            3329325298u32
                        }
                    }
                }
            }
        }
    }
}

pub fn f17(v0: u32) -> u32 {
    let a284 = {
        let a282 = f25(v0, 6u32, 26u32);
        let a283 = f25(v0, 11u32, 21u32);
        fixed_u32::bit_xor(a282, a283)
    };
    let a285 = f25(v0, 25u32, 7u32);
    fixed_u32::bit_xor(a284, a285)
}

pub fn f18(v0: u32, v1: u32, v2: u32) -> u32 {
    let a291 = {
        let a286 = v0;
        let a287 = v1;
        fixed_u32::bit_and(a286, a287)
    };
    let a292 = {
        let a289 = {
            let a288 = v0;
            fixed_u32::bit_not(a288)
        };
        let a290 = v2;
        fixed_u32::bit_and(a289, a290)
    };
    fixed_u32::bit_xor(a291, a292)
}

pub fn f19(v0: u32) -> u32 {
    let a295 = {
        let a293 = f25(v0, 2u32, 30u32);
        let a294 = f25(v0, 13u32, 19u32);
        fixed_u32::bit_xor(a293, a294)
    };
    let a296 = f25(v0, 22u32, 10u32);
    fixed_u32::bit_xor(a295, a296)
}

pub fn f20(v0: u32, v1: u32, v2: u32) -> u32 {
    let a305 = {
        let a301 = {
            let a297 = v0;
            let a298 = v1;
            fixed_u32::bit_and(a297, a298)
        };
        let a302 = {
            let a299 = v0;
            let a300 = v2;
            fixed_u32::bit_and(a299, a300)
        };
        fixed_u32::bit_xor(a301, a302)
    };
    let a306 = {
        let a303 = v1;
        let a304 = v2;
        fixed_u32::bit_and(a303, a304)
    };
    fixed_u32::bit_xor(a305, a306)
}

pub fn f21(v0: Adt1) -> Adt1 {
    Adt1::C0(match v0.clone() {
        Adt1::C0(_, h307, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h307
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h308, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h308
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h309, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h309
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h310, _, _, _, _, _, _, _, _, _, _, _) => {
            h310
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, h311, _, _, _, _, _, _, _, _, _, _) => {
            h311
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h312, _, _, _, _, _, _, _, _, _) => {
            h312
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h313, _, _, _, _, _, _, _, _) => {
            h313
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, h314, _, _, _, _, _, _, _) => {
            h314
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, h315, _, _, _, _, _, _) => {
            h315
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, h316, _, _, _, _, _) => {
            h316
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, h317, _, _, _, _) => {
            h317
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, h318, _, _, _) => {
            h318
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h319, _, _) => {
            h319
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h320, _) => {
            h320
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h321) => {
            h321
        }
    }, f8(f26(match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h322, _) => {
            h322
        }
    }), f8(match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, h323, _, _, _, _, _, _) => {
            h323
        }
    }, f8(f27(match v0.clone() {
        Adt1::C0(_, h324, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h324
        }
    }), match v0 {
        Adt1::C0(h325, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h325
        }
    }))))
}

pub fn f22(v0: Fn2, p329: u64, p330: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m328 = (p329, p330);
    {
        let mut m328 = m328;
        loop {
            let step = {
                let (v1, v2) = m328;
                let m326 = v1;
                if m326 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m326 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c327: Fn2 = v0.clone();
                        c327.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m328 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f23(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h331, _) = v0;
        h331
    };
    let v2: u32 = {
        let (h332, _) = {
            let (_, h333) = v0;
            h333
        };
        h332
    };
    let v3: u32 = {
        let (_, h334) = {
            let (_, h335) = v0;
            h335
        };
        h334
    };
    ({
        let a336 = v1;
        let a337 = 2u64;
        let a338 = 0u64;
        nat_quot(a336, a337, a338)
    }, (f24(v2, 1u32), {
        let a342 = {
            let a339 = v1;
            let a340 = 2u64;
            let a341 = 0u64;
            nat_rem(a339, a340, a341)
        };
        let a343 = 1u64;
        if nat_eq(a342, a343) {
            let a344 = v3;
            let a345 = v2;
            fixed_u32::bit_or(a344, a345)
        } else {
            v3
        }
    }))
}

pub fn f24(v0: u32, v1: u32) -> u32 {
    let m348 = {
        let a346 = v0;
        let a347 = v1;
        fixed_u32::shift_left(a346, a347)
    };
    match m348 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f25(v0: u32, v1: u32, v2: u32) -> u32 {
    let a349 = f28(v0, v1);
    let a350 = f24(v0, v2);
    fixed_u32::bit_or(a349, a350)
}

pub fn f26(v0: u32) -> u32 {
    let a353 = {
        let a351 = f25(v0, 17u32, 15u32);
        let a352 = f25(v0, 19u32, 13u32);
        fixed_u32::bit_xor(a351, a352)
    };
    let a354 = f28(v0, 10u32);
    fixed_u32::bit_xor(a353, a354)
}

pub fn f27(v0: u32) -> u32 {
    let a357 = {
        let a355 = f25(v0, 7u32, 25u32);
        let a356 = f25(v0, 18u32, 14u32);
        fixed_u32::bit_xor(a355, a356)
    };
    let a358 = f28(v0, 3u32);
    fixed_u32::bit_xor(a357, a358)
}

pub fn f28(v0: u32, v1: u32) -> u32 {
    let m361 = {
        let a359 = v0;
        let a360 = v1;
        fixed_u32::shift_right(a359, a360)
    };
    match m361 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}
