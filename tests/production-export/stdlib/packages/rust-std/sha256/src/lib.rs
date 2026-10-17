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

pub fn f1(v0: Fn0, v1: u64, v2: (u64, Adt0)) -> R<(u64, Adt0)> {
    let m13 = v1;
    if m13 == 0 {
        Ok(v2)
    } else {
        let v3 = m13 - 1;
        f1(v0.clone(), v3, {
            let c14: Fn0 = v0;
            c14.apply(v2)?
        })
    }
}

pub fn f2(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt0)) -> R<(u64, Adt0)> {
    Ok(({
        let a16 = {
            let (h15, _) = v3.clone();
            h15
        };
        let a17 = 64u64;
        nat_add(a16, a17)?
    }, f4({
        let (_, h18) = v3;
        h18
    }, f5(v0, v1, v2, {
        let (h19, _) = v3;
        h19
    })?)?))
}

pub fn f3() -> Adt0 {
    Adt0::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f4(v0: Adt0, v1: Adt1) -> R<Adt0> {
    let v2: Adt2 = f6(Fn1::F7, 64u64, Adt2::C0(match v0.clone() {
        Adt0::C0(h20, _, _, _, _, _, _, _) => {
            h20
        }
    }, match v0.clone() {
        Adt0::C0(_, h21, _, _, _, _, _, _) => {
            h21
        }
    }, match v0.clone() {
        Adt0::C0(_, _, h22, _, _, _, _, _) => {
            h22
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, h23, _, _, _, _) => {
            h23
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, h24, _, _, _) => {
            h24
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, h25, _, _) => {
            h25
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, h26, _) => {
            h26
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, h27) => {
            h27
        }
    }, v1, 0u64))?;
    Ok(Adt0::C0(f8(match v0.clone() {
        Adt0::C0(h28, _, _, _, _, _, _, _) => {
            h28
        }
    }, match v2.clone() {
        Adt2::C0(h29, _, _, _, _, _, _, _, _, _) => {
            h29
        }
    }), f8(match v0.clone() {
        Adt0::C0(_, h30, _, _, _, _, _, _) => {
            h30
        }
    }, match v2.clone() {
        Adt2::C0(_, h31, _, _, _, _, _, _, _, _) => {
            h31
        }
    }), f8(match v0.clone() {
        Adt0::C0(_, _, h32, _, _, _, _, _) => {
            h32
        }
    }, match v2.clone() {
        Adt2::C0(_, _, h33, _, _, _, _, _, _, _) => {
            h33
        }
    }), f8(match v0.clone() {
        Adt0::C0(_, _, _, h34, _, _, _, _) => {
            h34
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, h35, _, _, _, _, _, _) => {
            h35
        }
    }), f8(match v0.clone() {
        Adt0::C0(_, _, _, _, h36, _, _, _) => {
            h36
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, _, h37, _, _, _, _, _) => {
            h37
        }
    }), f8(match v0.clone() {
        Adt0::C0(_, _, _, _, _, h38, _, _) => {
            h38
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, _, _, h39, _, _, _, _) => {
            h39
        }
    }), f8(match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, h40, _) => {
            h40
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, _, _, _, h41, _, _, _) => {
            h41
        }
    }), f8(match v0 {
        Adt0::C0(_, _, _, _, _, _, _, h42) => {
            h42
        }
    }, match v2 {
        Adt2::C0(_, _, _, _, _, _, _, h43, _, _) => {
            h43
        }
    })))
}

pub fn f5(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt1> {
    Ok(Adt1::C0(f9(v0.clone(), v1, v2, v3)?, f9(v0.clone(), v1, v2, {
        let a44 = v3;
        let a45 = 4u64;
        nat_add(a44, a45)?
    })?, f9(v0.clone(), v1, v2, {
        let a46 = v3;
        let a47 = 8u64;
        nat_add(a46, a47)?
    })?, f9(v0.clone(), v1, v2, {
        let a48 = v3;
        let a49 = 12u64;
        nat_add(a48, a49)?
    })?, f9(v0.clone(), v1, v2, {
        let a50 = v3;
        let a51 = 16u64;
        nat_add(a50, a51)?
    })?, f9(v0.clone(), v1, v2, {
        let a52 = v3;
        let a53 = 20u64;
        nat_add(a52, a53)?
    })?, f9(v0.clone(), v1, v2, {
        let a54 = v3;
        let a55 = 24u64;
        nat_add(a54, a55)?
    })?, f9(v0.clone(), v1, v2, {
        let a56 = v3;
        let a57 = 28u64;
        nat_add(a56, a57)?
    })?, f9(v0.clone(), v1, v2, {
        let a58 = v3;
        let a59 = 32u64;
        nat_add(a58, a59)?
    })?, f9(v0.clone(), v1, v2, {
        let a60 = v3;
        let a61 = 36u64;
        nat_add(a60, a61)?
    })?, f9(v0.clone(), v1, v2, {
        let a62 = v3;
        let a63 = 40u64;
        nat_add(a62, a63)?
    })?, f9(v0.clone(), v1, v2, {
        let a64 = v3;
        let a65 = 44u64;
        nat_add(a64, a65)?
    })?, f9(v0.clone(), v1, v2, {
        let a66 = v3;
        let a67 = 48u64;
        nat_add(a66, a67)?
    })?, f9(v0.clone(), v1, v2, {
        let a68 = v3;
        let a69 = 52u64;
        nat_add(a68, a69)?
    })?, f9(v0.clone(), v1, v2, {
        let a70 = v3;
        let a71 = 56u64;
        nat_add(a70, a71)?
    })?, f9(v0, v1, v2, {
        let a72 = v3;
        let a73 = 60u64;
        nat_add(a72, a73)?
    })?))
}

pub fn f6(v0: Fn1, v1: u64, v2: Adt2) -> R<Adt2> {
    let m74 = v1;
    if m74 == 0 {
        Ok(v2)
    } else {
        let v3 = m74 - 1;
        f6(v0.clone(), v3, {
            let c75: Fn1 = v0;
            c75.apply(v2)?
        })
    }
}

pub fn f7(v0: Adt2) -> R<Adt2> {
    f10(v0)
}

pub fn f8(v0: u32, v1: u32) -> u32 {
    let m78 = {
        let a76 = f11(v0);
        let a77 = f11(v1);
        fixed_u64::checked_add(a76, a77)
    };
    match m78 {
        None => {
            0u32
        }
        Some(v2) => {
            f12(v2)
        }
    }
}

pub fn f9(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u32> {
    let a81 = {
        let a79 = v3;
        let a80 = 8u64;
        nat_add(a79, a80)?
    };
    let a82 = v2;
    if nat_eq(a81, a82) {
        Ok(f13({
            let a85 = {
                let a83 = v1;
                let a84 = 8u64;
                nat_mul(a83, a84)?
            };
            let a86 = 4294967296u64;
            let a87 = 0u64;
            nat_quot(a85, a86, a87)
        }))
    } else {
        let a90 = {
            let a88 = v3;
            let a89 = 4u64;
            nat_add(a88, a89)?
        };
        let a91 = v2;
        if nat_eq(a90, a91) {
            Ok(f13({
                let a94 = {
                    let a92 = v1;
                    let a93 = 8u64;
                    nat_mul(a92, a93)?
                };
                let a95 = 4294967296u64;
                let a96 = 0u64;
                nat_rem(a94, a95, a96)
            }))
        } else {
            Ok(f14(f15(v0.clone(), v1, v3), f15(v0.clone(), v1, {
                let a97 = v3;
                let a98 = 1u64;
                nat_add(a97, a98)?
            }), f15(v0.clone(), v1, {
                let a99 = v3;
                let a100 = 2u64;
                nat_add(a99, a100)?
            }), f15(v0, v1, {
                let a101 = v3;
                let a102 = 3u64;
                nat_add(a101, a102)?
            })))
        }
    }
}

pub fn f10(v0: Adt2) -> R<Adt2> {
    let v1: u32 = f16(match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h103) => {
            h103
        }
    });
    let v2: u32 = f8(match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h104, _, _) => {
            h104
        }
    }, f8(f17(match v0.clone() {
        Adt2::C0(_, _, _, _, h105, _, _, _, _, _) => {
            h105
        }
    }), f8(f18(match v0.clone() {
        Adt2::C0(_, _, _, _, h106, _, _, _, _, _) => {
            h106
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h107, _, _, _, _) => {
            h107
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h108, _, _, _) => {
            h108
        }
    }), f8(v1, {
        let m111 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, h110, _) => {
                h110
            }
        };
        match m111 {
            Adt1::C0(h109, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h109
            }
        }
    }))));
    let v3: u32 = f8(f19(match v0.clone() {
        Adt2::C0(h112, _, _, _, _, _, _, _, _, _) => {
            h112
        }
    }), f20(match v0.clone() {
        Adt2::C0(h113, _, _, _, _, _, _, _, _, _) => {
            h113
        }
    }, match v0.clone() {
        Adt2::C0(_, h114, _, _, _, _, _, _, _, _) => {
            h114
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h115, _, _, _, _, _, _, _) => {
            h115
        }
    }));
    Ok(Adt2::C0(f8(v2, v3), match v0.clone() {
        Adt2::C0(h116, _, _, _, _, _, _, _, _, _) => {
            h116
        }
    }, match v0.clone() {
        Adt2::C0(_, h117, _, _, _, _, _, _, _, _) => {
            h117
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h118, _, _, _, _, _, _, _) => {
            h118
        }
    }, f8(match v0.clone() {
        Adt2::C0(_, _, _, h119, _, _, _, _, _, _) => {
            h119
        }
    }, v2), match v0.clone() {
        Adt2::C0(_, _, _, _, h120, _, _, _, _, _) => {
            h120
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h121, _, _, _, _) => {
            h121
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h122, _, _, _) => {
            h122
        }
    }, f21(match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, h123, _) => {
            h123
        }
    }), {
        let a125 = match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h124) => {
                h124
            }
        };
        let a126 = 1u64;
        nat_add(a125, a126)?
    }))
}

pub fn f11(v0: u32) -> u64 {
    let m128 = {
        let a127 = v0;
        fixed_u64::convert(i128::from(a127))
    };
    match m128 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f12(v0: u64) -> u32 {
    let m132 = {
        let a131 = {
            let a129 = v0;
            let a130 = 4294967295u64;
            fixed_u64::bit_and(a129, a130)
        };
        fixed_u32::convert(i128::from(a131))
    };
    match m132 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f13(v0: u64) -> u32 {
    let (_, h133) = {
        let (_, h134) = f22(Fn2::F23, 32u64, (v0, (1u32, 0u32)));
        h134
    };
    h133
}

pub fn f14(v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
    let a139 = f24(v0, 24u32);
    let a140 = {
        let a137 = f24(v1, 16u32);
        let a138 = {
            let a135 = f24(v2, 8u32);
            let a136 = v3;
            fixed_u32::bit_or(a135, a136)
        };
        fixed_u32::bit_or(a137, a138)
    };
    fixed_u32::bit_or(a139, a140)
}

pub fn f15(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a141 = v2;
    let a142 = v1;
    if nat_lt(a141, a142) {
        let m145 = {
            let a143 = v0;
            let a144 = v2;
            index_bytes(a143, a144)
        };
        match m145 {
            None => {
                0u32
            }
            Some(v3) => {
                let m147 = {
                    let a146 = v3;
                    fixed_u32::convert(i128::from(a146))
                };
                match m147 {
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
        let a148 = v2;
        let a149 = v1;
        if nat_eq(a148, a149) {
            128u32
        } else {
            0u32
        }
    }
}

pub fn f16(v0: u64) -> u32 {
    let a150 = v0;
    let a151 = 32u64;
    if nat_lt(a150, a151) {
        let a152 = v0;
        let a153 = 16u64;
        if nat_lt(a152, a153) {
            let a154 = v0;
            let a155 = 8u64;
            if nat_lt(a154, a155) {
                let a156 = v0;
                let a157 = 4u64;
                if nat_lt(a156, a157) {
                    let a158 = v0;
                    let a159 = 2u64;
                    if nat_lt(a158, a159) {
                        let a160 = v0;
                        let a161 = 1u64;
                        if nat_lt(a160, a161) {
                            1116352408u32
                        } else {
                            1899447441u32
                        }
                    } else {
                        let a162 = v0;
                        let a163 = 3u64;
                        if nat_lt(a162, a163) {
                            3049323471u32
                        } else {
                            3921009573u32
                        }
                    }
                } else {
                    let a164 = v0;
                    let a165 = 6u64;
                    if nat_lt(a164, a165) {
                        let a166 = v0;
                        let a167 = 5u64;
                        if nat_lt(a166, a167) {
                            961987163u32
                        } else {
                            1508970993u32
                        }
                    } else {
                        let a168 = v0;
                        let a169 = 7u64;
                        if nat_lt(a168, a169) {
                            2453635748u32
                        } else {
                            2870763221u32
                        }
                    }
                }
            } else {
                let a170 = v0;
                let a171 = 12u64;
                if nat_lt(a170, a171) {
                    let a172 = v0;
                    let a173 = 10u64;
                    if nat_lt(a172, a173) {
                        let a174 = v0;
                        let a175 = 9u64;
                        if nat_lt(a174, a175) {
                            3624381080u32
                        } else {
                            310598401u32
                        }
                    } else {
                        let a176 = v0;
                        let a177 = 11u64;
                        if nat_lt(a176, a177) {
                            607225278u32
                        } else {
                            1426881987u32
                        }
                    }
                } else {
                    let a178 = v0;
                    let a179 = 14u64;
                    if nat_lt(a178, a179) {
                        let a180 = v0;
                        let a181 = 13u64;
                        if nat_lt(a180, a181) {
                            1925078388u32
                        } else {
                            2162078206u32
                        }
                    } else {
                        let a182 = v0;
                        let a183 = 15u64;
                        if nat_lt(a182, a183) {
                            2614888103u32
                        } else {
                            3248222580u32
                        }
                    }
                }
            }
        } else {
            let a184 = v0;
            let a185 = 24u64;
            if nat_lt(a184, a185) {
                let a186 = v0;
                let a187 = 20u64;
                if nat_lt(a186, a187) {
                    let a188 = v0;
                    let a189 = 18u64;
                    if nat_lt(a188, a189) {
                        let a190 = v0;
                        let a191 = 17u64;
                        if nat_lt(a190, a191) {
                            3835390401u32
                        } else {
                            4022224774u32
                        }
                    } else {
                        let a192 = v0;
                        let a193 = 19u64;
                        if nat_lt(a192, a193) {
                            264347078u32
                        } else {
                            604807628u32
                        }
                    }
                } else {
                    let a194 = v0;
                    let a195 = 22u64;
                    if nat_lt(a194, a195) {
                        let a196 = v0;
                        let a197 = 21u64;
                        if nat_lt(a196, a197) {
                            770255983u32
                        } else {
                            1249150122u32
                        }
                    } else {
                        let a198 = v0;
                        let a199 = 23u64;
                        if nat_lt(a198, a199) {
                            1555081692u32
                        } else {
                            1996064986u32
                        }
                    }
                }
            } else {
                let a200 = v0;
                let a201 = 28u64;
                if nat_lt(a200, a201) {
                    let a202 = v0;
                    let a203 = 26u64;
                    if nat_lt(a202, a203) {
                        let a204 = v0;
                        let a205 = 25u64;
                        if nat_lt(a204, a205) {
                            2554220882u32
                        } else {
                            2821834349u32
                        }
                    } else {
                        let a206 = v0;
                        let a207 = 27u64;
                        if nat_lt(a206, a207) {
                            2952996808u32
                        } else {
                            3210313671u32
                        }
                    }
                } else {
                    let a208 = v0;
                    let a209 = 30u64;
                    if nat_lt(a208, a209) {
                        let a210 = v0;
                        let a211 = 29u64;
                        if nat_lt(a210, a211) {
                            3336571891u32
                        } else {
                            3584528711u32
                        }
                    } else {
                        let a212 = v0;
                        let a213 = 31u64;
                        if nat_lt(a212, a213) {
                            113926993u32
                        } else {
                            338241895u32
                        }
                    }
                }
            }
        }
    } else {
        let a214 = v0;
        let a215 = 48u64;
        if nat_lt(a214, a215) {
            let a216 = v0;
            let a217 = 40u64;
            if nat_lt(a216, a217) {
                let a218 = v0;
                let a219 = 36u64;
                if nat_lt(a218, a219) {
                    let a220 = v0;
                    let a221 = 34u64;
                    if nat_lt(a220, a221) {
                        let a222 = v0;
                        let a223 = 33u64;
                        if nat_lt(a222, a223) {
                            666307205u32
                        } else {
                            773529912u32
                        }
                    } else {
                        let a224 = v0;
                        let a225 = 35u64;
                        if nat_lt(a224, a225) {
                            1294757372u32
                        } else {
                            1396182291u32
                        }
                    }
                } else {
                    let a226 = v0;
                    let a227 = 38u64;
                    if nat_lt(a226, a227) {
                        let a228 = v0;
                        let a229 = 37u64;
                        if nat_lt(a228, a229) {
                            1695183700u32
                        } else {
                            1986661051u32
                        }
                    } else {
                        let a230 = v0;
                        let a231 = 39u64;
                        if nat_lt(a230, a231) {
                            2177026350u32
                        } else {
                            2456956037u32
                        }
                    }
                }
            } else {
                let a232 = v0;
                let a233 = 44u64;
                if nat_lt(a232, a233) {
                    let a234 = v0;
                    let a235 = 42u64;
                    if nat_lt(a234, a235) {
                        let a236 = v0;
                        let a237 = 41u64;
                        if nat_lt(a236, a237) {
                            2730485921u32
                        } else {
                            2820302411u32
                        }
                    } else {
                        let a238 = v0;
                        let a239 = 43u64;
                        if nat_lt(a238, a239) {
                            3259730800u32
                        } else {
                            3345764771u32
                        }
                    }
                } else {
                    let a240 = v0;
                    let a241 = 46u64;
                    if nat_lt(a240, a241) {
                        let a242 = v0;
                        let a243 = 45u64;
                        if nat_lt(a242, a243) {
                            3516065817u32
                        } else {
                            3600352804u32
                        }
                    } else {
                        let a244 = v0;
                        let a245 = 47u64;
                        if nat_lt(a244, a245) {
                            4094571909u32
                        } else {
                            275423344u32
                        }
                    }
                }
            }
        } else {
            let a246 = v0;
            let a247 = 56u64;
            if nat_lt(a246, a247) {
                let a248 = v0;
                let a249 = 52u64;
                if nat_lt(a248, a249) {
                    let a250 = v0;
                    let a251 = 50u64;
                    if nat_lt(a250, a251) {
                        let a252 = v0;
                        let a253 = 49u64;
                        if nat_lt(a252, a253) {
                            430227734u32
                        } else {
                            506948616u32
                        }
                    } else {
                        let a254 = v0;
                        let a255 = 51u64;
                        if nat_lt(a254, a255) {
                            659060556u32
                        } else {
                            883997877u32
                        }
                    }
                } else {
                    let a256 = v0;
                    let a257 = 54u64;
                    if nat_lt(a256, a257) {
                        let a258 = v0;
                        let a259 = 53u64;
                        if nat_lt(a258, a259) {
                            958139571u32
                        } else {
                            1322822218u32
                        }
                    } else {
                        let a260 = v0;
                        let a261 = 55u64;
                        if nat_lt(a260, a261) {
                            1537002063u32
                        } else {
                            1747873779u32
                        }
                    }
                }
            } else {
                let a262 = v0;
                let a263 = 60u64;
                if nat_lt(a262, a263) {
                    let a264 = v0;
                    let a265 = 58u64;
                    if nat_lt(a264, a265) {
                        let a266 = v0;
                        let a267 = 57u64;
                        if nat_lt(a266, a267) {
                            1955562222u32
                        } else {
                            2024104815u32
                        }
                    } else {
                        let a268 = v0;
                        let a269 = 59u64;
                        if nat_lt(a268, a269) {
                            2227730452u32
                        } else {
                            2361852424u32
                        }
                    }
                } else {
                    let a270 = v0;
                    let a271 = 62u64;
                    if nat_lt(a270, a271) {
                        let a272 = v0;
                        let a273 = 61u64;
                        if nat_lt(a272, a273) {
                            2428436474u32
                        } else {
                            2756734187u32
                        }
                    } else {
                        let a274 = v0;
                        let a275 = 63u64;
                        if nat_lt(a274, a275) {
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
    let a278 = {
        let a276 = f25(v0, 6u32, 26u32);
        let a277 = f25(v0, 11u32, 21u32);
        fixed_u32::bit_xor(a276, a277)
    };
    let a279 = f25(v0, 25u32, 7u32);
    fixed_u32::bit_xor(a278, a279)
}

pub fn f18(v0: u32, v1: u32, v2: u32) -> u32 {
    let a285 = {
        let a280 = v0;
        let a281 = v1;
        fixed_u32::bit_and(a280, a281)
    };
    let a286 = {
        let a283 = {
            let a282 = v0;
            fixed_u32::bit_not(a282)
        };
        let a284 = v2;
        fixed_u32::bit_and(a283, a284)
    };
    fixed_u32::bit_xor(a285, a286)
}

pub fn f19(v0: u32) -> u32 {
    let a289 = {
        let a287 = f25(v0, 2u32, 30u32);
        let a288 = f25(v0, 13u32, 19u32);
        fixed_u32::bit_xor(a287, a288)
    };
    let a290 = f25(v0, 22u32, 10u32);
    fixed_u32::bit_xor(a289, a290)
}

pub fn f20(v0: u32, v1: u32, v2: u32) -> u32 {
    let a299 = {
        let a295 = {
            let a291 = v0;
            let a292 = v1;
            fixed_u32::bit_and(a291, a292)
        };
        let a296 = {
            let a293 = v0;
            let a294 = v2;
            fixed_u32::bit_and(a293, a294)
        };
        fixed_u32::bit_xor(a295, a296)
    };
    let a300 = {
        let a297 = v1;
        let a298 = v2;
        fixed_u32::bit_and(a297, a298)
    };
    fixed_u32::bit_xor(a299, a300)
}

pub fn f21(v0: Adt1) -> Adt1 {
    Adt1::C0(match v0.clone() {
        Adt1::C0(_, h301, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h301
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h302, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h302
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h303, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h303
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h304, _, _, _, _, _, _, _, _, _, _, _) => {
            h304
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, h305, _, _, _, _, _, _, _, _, _, _) => {
            h305
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h306, _, _, _, _, _, _, _, _, _) => {
            h306
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h307, _, _, _, _, _, _, _, _) => {
            h307
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, h308, _, _, _, _, _, _, _) => {
            h308
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, h309, _, _, _, _, _, _) => {
            h309
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, h310, _, _, _, _, _) => {
            h310
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, h311, _, _, _, _) => {
            h311
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, h312, _, _, _) => {
            h312
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h313, _, _) => {
            h313
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h314, _) => {
            h314
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h315) => {
            h315
        }
    }, f8(f26(match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h316, _) => {
            h316
        }
    }), f8(match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, h317, _, _, _, _, _, _) => {
            h317
        }
    }, f8(f27(match v0.clone() {
        Adt1::C0(_, h318, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h318
        }
    }), match v0 {
        Adt1::C0(h319, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h319
        }
    }))))
}

pub fn f22(v0: Fn2, v1: u64, v2: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m320 = v1;
    if m320 == 0 {
        v2
    } else {
        let v3 = m320 - 1;
        f22(v0.clone(), v3, {
            let c321: Fn2 = v0;
            c321.apply(v2)
        })
    }
}

pub fn f23(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h322, _) = v0;
        h322
    };
    let v2: u32 = {
        let (h323, _) = {
            let (_, h324) = v0;
            h324
        };
        h323
    };
    let v3: u32 = {
        let (_, h325) = {
            let (_, h326) = v0;
            h326
        };
        h325
    };
    ({
        let a327 = v1;
        let a328 = 2u64;
        let a329 = 0u64;
        nat_quot(a327, a328, a329)
    }, (f24(v2, 1u32), {
        let a333 = {
            let a330 = v1;
            let a331 = 2u64;
            let a332 = 0u64;
            nat_rem(a330, a331, a332)
        };
        let a334 = 1u64;
        if nat_eq(a333, a334) {
            let a335 = v3;
            let a336 = v2;
            fixed_u32::bit_or(a335, a336)
        } else {
            v3
        }
    }))
}

pub fn f24(v0: u32, v1: u32) -> u32 {
    let m339 = {
        let a337 = v0;
        let a338 = v1;
        fixed_u32::shift_left(a337, a338)
    };
    match m339 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f25(v0: u32, v1: u32, v2: u32) -> u32 {
    let a340 = f28(v0, v1);
    let a341 = f24(v0, v2);
    fixed_u32::bit_or(a340, a341)
}

pub fn f26(v0: u32) -> u32 {
    let a344 = {
        let a342 = f25(v0, 17u32, 15u32);
        let a343 = f25(v0, 19u32, 13u32);
        fixed_u32::bit_xor(a342, a343)
    };
    let a345 = f28(v0, 10u32);
    fixed_u32::bit_xor(a344, a345)
}

pub fn f27(v0: u32) -> u32 {
    let a348 = {
        let a346 = f25(v0, 7u32, 25u32);
        let a347 = f25(v0, 18u32, 14u32);
        fixed_u32::bit_xor(a346, a347)
    };
    let a349 = f28(v0, 3u32);
    fixed_u32::bit_xor(a348, a349)
}

pub fn f28(v0: u32, v1: u32) -> u32 {
    let m352 = {
        let a350 = v0;
        let a351 = v1;
        fixed_u32::shift_right(a350, a351)
    };
    match m352 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn run(p0: Bytes) -> R<Adt0> {
    f0(p0)
}
