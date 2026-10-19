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
    C0(Adt5, List<Adt0>, u64, Option<Adt2>),
}

#[derive(Clone)]
pub enum Adt5 {
    C0(u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt6 {
    C0(Adt1, Adt5, Option<Adt2>),
}

#[derive(Clone)]
pub enum Fn0 {
    F2(u64, u64, Bytes, Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: Adt4) -> R<Option<Adt4>> {
        match self {
            Fn0::F2(k0, k1, k2, k3) => f2(*k0, *k1, k2.clone(), k3.clone(), p0),
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
    F23(u64, u64, Bytes),
}

impl Fn2 {
    pub fn apply(&self, p0: Adt5) -> R<Option<Adt5>> {
        match self {
            Fn2::F23(k0, k1, k2) => f23(*k0, *k1, k2.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F32(Bytes),
}

impl Fn3 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn3::F32(k0) => f32(k0.clone(), p0),
        }
    }
}

pub fn f0(v0: Bytes, v1: Bytes, v2: u64) -> R<Result<List<Adt0>, Adt2>> {
    let v3: u64 = {
        let a1 = v1.clone();
        length_bytes(a1)
    };
    let v4: Adt4 = {
        let (h2, _) = f1(Fn0::F2(v2, v3, v0, v1), {
            let a3 = v3;
            let a4 = 1u64;
            nat_add(a3, a4)?
        }, Adt4::C0(Adt5::C0(0u64, 1u64, 1u64), List::<Adt0>::nil(), 0u64, None::<Adt2>))?;
        h2
    };
    let m6 = match v4.clone() {
        Adt4::C0(_, _, _, h5) => {
            h5
        }
    };
    match m6 {
        None => {
            Ok(Ok::<List<Adt0>, Adt2>(f3(match v4 {
                Adt4::C0(_, h7, _, _) => {
                    h7
                }
            })))
        }
        Some(v5) => {
            Ok(Err::<List<Adt0>, Adt2>(v5))
        }
    }
}

pub fn f1(v0: Fn0, v1: u64, v2: Adt4) -> R<(Adt4, bool)> {
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

pub fn f2(v0: u64, v1: u64, v2: Bytes, v3: Bytes, v4: Adt4) -> R<Option<Adt4>> {
    f4(v2, v3, v1, v0, v4)
}

pub fn f3(v0: List<Adt0>) -> List<Adt0> {
    f5(Fn1::F6, List::<Adt0>::nil(), v0)
}

pub fn f4(v0: Bytes, v1: Bytes, v2: u64, v3: u64, v4: Adt4) -> R<Option<Adt4>> {
    let v5: Adt5 = match v4.clone() {
        Adt4::C0(h11, _, _, _) => {
            h11
        }
    };
    let v6: u64 = match v5.clone() {
        Adt5::C0(h12, _, _) => {
            h12
        }
    };
    let m14 = match v4.clone() {
        Adt4::C0(_, _, _, h13) => {
            h13
        }
    };
    match m14 {
        None => {
            let a15 = v6;
            let a16 = v2;
            if nat_lt(a15, a16) {
                let v8: Adt6 = f7(v0.clone(), v1.clone(), v5.clone())?;
                let m18 = match v8.clone() {
                    Adt6::C0(_, _, h17) => {
                        h17
                    }
                };
                match m18 {
                    None => {
                        let v10: Adt5 = match v8.clone() {
                            Adt6::C0(_, h21, _) => {
                                h21
                            }
                        };
                        let v11: Adt0 = Adt0::C0(match v8 {
                            Adt6::C0(h22, _, _) => {
                                h22
                            }
                        }, v6, match v10.clone() {
                            Adt5::C0(h23, _, _) => {
                                h23
                            }
                        }, match v5.clone() {
                            Adt5::C0(_, h24, _) => {
                                h24
                            }
                        }, match v5 {
                            Adt5::C0(_, _, h25) => {
                                h25
                            }
                        }, match v10.clone() {
                            Adt5::C0(_, h26, _) => {
                                h26
                            }
                        }, match v10.clone() {
                            Adt5::C0(_, _, h27) => {
                                h27
                            }
                        }, f8(v1, v6, {
                            let a29 = match v10.clone() {
                                Adt5::C0(h28, _, _) => {
                                    h28
                                }
                            };
                            let a30 = v6;
                            nat_sub(a29, a30)
                        }));
                        let v12: u64 = {
                            let a32 = match v4.clone() {
                                Adt4::C0(_, _, h31, _) => {
                                    h31
                                }
                            };
                            let a33 = 1u64;
                            nat_add(a32, a33)?
                        };
                        let a34 = v3;
                        let a35 = v12;
                        if nat_lt(a34, a35) {
                            let v13: Adt2 = f9(v0, v3)?;
                            Ok(Some(Adt4::C0(v10, match v4 {
                                Adt4::C0(_, h36, _, _) => {
                                    h36
                                }
                            }, v12, Some(v13))))
                        } else {
                            Ok(Some(Adt4::C0(v10, List::cons(v11, match v4 {
                                Adt4::C0(_, h37, _, _) => {
                                    h37
                                }
                            }), v12, None::<Adt2>)))
                        }
                    }
                    Some(v9) => {
                        Ok(Some(Adt4::C0(v5, match v4.clone() {
                            Adt4::C0(_, h19, _, _) => {
                                h19
                            }
                        }, match v4 {
                            Adt4::C0(_, _, h20, _) => {
                                h20
                            }
                        }, Some(v9))))
                    }
                }
            } else {
                Ok(None::<Adt4>)
            }
        }
        Some(_) => {
            Ok(None::<Adt4>)
        }
    }
}

pub fn f5(v0: Fn1, v1: List<Adt0>, v2: List<Adt0>) -> List<Adt0> {
    let m38 = v2;
    match m38.uncons() {
        None => {
            v1
        }
        Some((v3, v4)) => {
            f5(v0.clone(), {
                let c39: Fn1 = v0;
                c39.apply(v1, v3)
            }, v4)
        }
    }
}

pub fn f6(v0: List<Adt0>, v1: Adt0) -> List<Adt0> {
    List::cons(v1, v0)
}

pub fn f7(v0: Bytes, v1: Bytes, v2: Adt5) -> R<Adt6> {
    let v3: u64 = {
        let a40 = v1.clone();
        length_bytes(a40)
    };
    let v4: u64 = match v2.clone() {
        Adt5::C0(h41, _, _) => {
            h41
        }
    };
    let v5: u8 = f10(v1.clone(), v4);
    let v6: Adt5 = f11(v1.clone(), 0u64, v2.clone())?;
    let v7: Adt5 = f11(v1.clone(), 1u64, v2.clone())?;
    let v8: Adt5 = f11(v1.clone(), 2u64, v2.clone())?;
    let v9: Adt5 = f12(v1.clone(), v2.clone())?;
    let v10: u64 = match v9.clone() {
        Adt5::C0(h42, _, _) => {
            h42
        }
    };
    let v11: u8 = f10(v1.clone(), v10);
    let v12: bool = {
        let a43 = v10;
        let a44 = v3;
        nat_lt(a43, a44)
    };
    let v13: bool = f13(v11);
    let v14: bool = f14(v11, 0u8, 127u8);
    let v15: Adt5 = f11(v1.clone(), 1u64, v9.clone())?;
    let v16: Adt5 = f12(v1.clone(), v9.clone())?;
    let v17: bool = f13(v5);
    let v18: bool = f15(v5);
    let v19: bool = f14(v5, 32u8, 126u8);
    let v20: bool = f14(v5, 0u8, 127u8);
    let v21: bool = {
        let a45 = v5;
        let a46 = 123u8;
        if equal(a45, a46) {
            true
        } else {
            let a47 = v5;
            let a48 = 125u8;
            if equal(a47, a48) {
                true
            } else {
                let a49 = v5;
                let a50 = 40u8;
                if equal(a49, a50) {
                    true
                } else {
                    let a51 = v5;
                    let a52 = 41u8;
                    if equal(a51, a52) {
                        true
                    } else {
                        let a53 = v5;
                        let a54 = 91u8;
                        if equal(a53, a54) {
                            true
                        } else {
                            let a55 = v5;
                            let a56 = 93u8;
                            equal(a55, a56)
                        }
                    }
                }
            }
        }
    };
    let a57 = v5;
    let a58 = 32u8;
    if if equal(a57, a58) {
        true
    } else {
        let a59 = v5;
        let a60 = 10u8;
        equal(a59, a60)
    } {
        Ok(Adt6::C0(Adt1::C6, v6, None::<Adt2>))
    } else {
        let a61 = v5;
        let a62 = 92u8;
        if equal(a61, a62) {
            if if v12 {
                v13
            } else {
                false
            } {
                Ok(Adt6::C0(Adt1::C0, v15, None::<Adt2>))
            } else {
                if if v12 {
                    v14
                } else {
                    false
                } {
                    Ok(Adt6::C0(Adt1::C0, v16, None::<Adt2>))
                } else {
                    let v22: Adt2 = f16(v0, v2, v9.clone());
                    Ok(Adt6::C0(Adt1::C0, v9, Some(v22)))
                }
            }
        } else {
            if v17 {
                Ok(Adt6::C0(Adt1::C1, v7, None::<Adt2>))
            } else {
                if v18 {
                    Ok(Adt6::C0(Adt1::C2, v8, None::<Adt2>))
                } else {
                    if v21 {
                        Ok(Adt6::C0(Adt1::C3, v9, None::<Adt2>))
                    } else {
                        if v19 {
                            Ok(Adt6::C0(Adt1::C4, v9, None::<Adt2>))
                        } else {
                            if v20 {
                                let v23: Adt2 = f17(v0, v1, v2.clone())?;
                                Ok(Adt6::C0(Adt1::C0, v2, Some(v23)))
                            } else {
                                Ok(Adt6::C0(Adt1::C5, v9, None::<Adt2>))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f8(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m66 = {
        let a63 = v0;
        let a64 = v1;
        let a65 = v2;
        slice_bytes(a63, a64, a65)
    };
    match m66 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f9(v0: Bytes, v1: u64) -> R<Adt2> {
    Ok(Adt2::C0(f18(Str::lit("LLS8002")), f19(f19(f19(f18(Str::lit("max_primitive_atoms exceeded: configured ")), f20(v1)?), f18(Str::lit(", scanning "))), v0), None::<Adt3>, None::<Bytes>))
}

pub fn f10(v0: Bytes, v1: u64) -> u8 {
    let m67 = f21(v0, v1);
    match m67 {
        None => {
            0u8
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f11(v0: Bytes, v1: u64, v2: Adt5) -> R<Adt5> {
    let v3: u64 = {
        let a68 = v0.clone();
        length_bytes(a68)
    };
    let (h69, _) = f22(Fn2::F23(v1, v3, v0), {
        let a71 = v3;
        let a72 = match v2.clone() {
            Adt5::C0(h70, _, _) => {
                h70
            }
        };
        nat_sub(a71, a72)
    }, v2)?;
    Ok(h69)
}

pub fn f12(v0: Bytes, v1: Adt5) -> R<Adt5> {
    let v2: u64 = match v1.clone() {
        Adt5::C0(h73, _, _) => {
            h73
        }
    };
    let v3: u64 = f24(v0.clone(), v2);
    let v4: bool = f25(v0, v2, 10u8);
    if v4 {
        Ok(Adt5::C0({
            let a74 = v2;
            let a75 = 1u64;
            nat_add(a74, a75)?
        }, {
            let a77 = match v1 {
                Adt5::C0(_, h76, _) => {
                    h76
                }
            };
            let a78 = 1u64;
            nat_add(a77, a78)?
        }, 1u64))
    } else {
        Ok(Adt5::C0({
            let a79 = v2;
            let a80 = v3;
            nat_add(a79, a80)?
        }, match v1.clone() {
            Adt5::C0(_, h81, _) => {
                h81
            }
        }, {
            let a83 = match v1 {
                Adt5::C0(_, _, h82) => {
                    h82
                }
            };
            let a84 = 1u64;
            nat_add(a83, a84)?
        }))
    }
}

pub fn f13(v0: u8) -> bool {
    let v1: bool = f14(v0, 97u8, 122u8);
    let v2: bool = f14(v0, 65u8, 90u8);
    if v1 {
        true
    } else {
        v2
    }
}

pub fn f14(v0: u8, v1: u8, v2: u8) -> bool {
    let m87 = {
        let a85 = v0;
        let a86 = v1;
        fixed_u8::checked_sub(a85, a86)
    };
    match m87 {
        None => {
            false
        }
        Some(_) => {
            let m90 = {
                let a88 = v2;
                let a89 = v0;
                fixed_u8::checked_sub(a88, a89)
            };
            match m90 {
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

pub fn f15(v0: u8) -> bool {
    f14(v0, 48u8, 57u8)
}

pub fn f16(v0: Bytes, v1: Adt5, v2: Adt5) -> Adt2 {
    let v3: Adt3 = f26(v0, v1, v2);
    Adt2::C0(f18(Str::lit("LLL1004")), f18(Str::lit("a control sequence needs ASCII letters or one ASCII nonletter after the backslash")), Some(v3), None::<Bytes>)
}

pub fn f17(v0: Bytes, v1: Bytes, v2: Adt5) -> R<Adt2> {
    let v3: u8 = f10(v1, match v2.clone() {
        Adt5::C0(h91, _, _) => {
            h91
        }
    });
    let v5: u8 = {
        let m94 = {
            let a92 = v3;
            let a93 = 4u32;
            fixed_u8::shift_right(a92, a93)
        };
        match m94 {
            None => {
                0u8
            }
            Some(v4) => {
                v4
            }
        }
    };
    let v6: u8 = {
        let a95 = v3;
        let a96 = 15u8;
        fixed_u8::bit_and(a95, a96)
    };
    let v7: Bytes = f27(v5);
    let v8: Bytes = f27(v6);
    let v9: Adt5 = Adt5::C0({
        let a98 = match v2.clone() {
            Adt5::C0(h97, _, _) => {
                h97
            }
        };
        let a99 = 1u64;
        nat_add(a98, a99)?
    }, match v2.clone() {
        Adt5::C0(_, h100, _) => {
            h100
        }
    }, {
        let a102 = match v2.clone() {
            Adt5::C0(_, _, h101) => {
                h101
            }
        };
        let a103 = 1u64;
        nat_add(a102, a103)?
    });
    let v10: Adt3 = f26(v0, v2, v9);
    Ok(Adt2::C0(f18(Str::lit("LLL1001")), f19(f19(f18(Str::lit("forbidden scalar U+00")), v7), v8), Some(v10), None::<Bytes>))
}

pub fn f18(v0: Str) -> Bytes {
    let a104 = v0;
    utf8_encode(a104)
}

pub fn f19(v0: Bytes, v1: Bytes) -> Bytes {
    let a105 = v0;
    let a106 = v1;
    append_bytes(a105, a106)
}

pub fn f20(v0: u64) -> R<Bytes> {
    let v1: Bytes = f18(Str::lit("0"));
    let v2: Bytes = f28({
        let a143 = f29(v0, 10000000000000000000u64);
        let a144 = {
            let a141 = f29(v0, 1000000000000000000u64);
            let a142 = {
                let a139 = f29(v0, 100000000000000000u64);
                let a140 = {
                    let a137 = f29(v0, 10000000000000000u64);
                    let a138 = {
                        let a135 = f29(v0, 1000000000000000u64);
                        let a136 = {
                            let a133 = f29(v0, 100000000000000u64);
                            let a134 = {
                                let a131 = f29(v0, 10000000000000u64);
                                let a132 = {
                                    let a129 = f29(v0, 1000000000000u64);
                                    let a130 = {
                                        let a127 = f29(v0, 100000000000u64);
                                        let a128 = {
                                            let a125 = f29(v0, 10000000000u64);
                                            let a126 = {
                                                let a123 = f29(v0, 1000000000u64);
                                                let a124 = {
                                                    let a121 = f29(v0, 100000000u64);
                                                    let a122 = {
                                                        let a119 = f29(v0, 10000000u64);
                                                        let a120 = {
                                                            let a117 = f29(v0, 1000000u64);
                                                            let a118 = {
                                                                let a115 = f29(v0, 100000u64);
                                                                let a116 = {
                                                                    let a113 = f29(v0, 10000u64);
                                                                    let a114 = {
                                                                        let a111 = f29(v0, 1000u64);
                                                                        let a112 = {
                                                                            let a109 = f29(v0, 100u64);
                                                                            let a110 = {
                                                                                let a107 = f29(v0, 10u64);
                                                                                let a108 = f29(v0, 1u64);
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
                                    };
                                    append_bytes(a129, a130)
                                };
                                append_bytes(a131, a132)
                            };
                            append_bytes(a133, a134)
                        };
                        append_bytes(a135, a136)
                    };
                    append_bytes(a137, a138)
                };
                append_bytes(a139, a140)
            };
            append_bytes(a141, a142)
        };
        append_bytes(a143, a144)
    })?;
    let a145 = v0;
    let a146 = 0u64;
    if nat_eq(a145, a146) {
        Ok(v1)
    } else {
        Ok(v2)
    }
}

pub fn f21(v0: Bytes, v1: u64) -> Option<u8> {
    let a147 = v0;
    let a148 = v1;
    index_bytes(a147, a148)
}

pub fn f22(v0: Fn2, v1: u64, v2: Adt5) -> R<(Adt5, bool)> {
    let m149 = v1;
    if m149 == 0 {
        Ok((v2, false))
    } else {
        let v3 = m149 - 1;
        let m151 = {
            let c150: Fn2 = v0.clone();
            c150.apply(v2.clone())?
        };
        match m151 {
            None => {
                Ok((v2, true))
            }
            Some(v4) => {
                f22(v0, v3, v4)
            }
        }
    }
}

pub fn f23(v0: u64, v1: u64, v2: Bytes, v3: Adt5) -> R<Option<Adt5>> {
    f30(v2, v1, v0, v3)
}

pub fn f24(v0: Bytes, v1: u64) -> u64 {
    let v2: u8 = f10(v0, v1);
    let v3: bool = f14(v2, 0u8, 127u8);
    if v3 {
        1u64
    } else {
        let v4: bool = f14(v2, 128u8, 223u8);
        if v4 {
            2u64
        } else {
            let v5: bool = f14(v2, 224u8, 239u8);
            if v5 {
                3u64
            } else {
                4u64
            }
        }
    }
}

pub fn f25(v0: Bytes, v1: u64, v2: u8) -> bool {
    let m152 = f21(v0, v1);
    match m152 {
        None => {
            false
        }
        Some(v3) => {
            let a153 = v3;
            let a154 = v2;
            equal(a153, a154)
        }
    }
}

pub fn f26(v0: Bytes, v1: Adt5, v2: Adt5) -> Adt3 {
    Adt3::C0(v0, match v1.clone() {
        Adt5::C0(h155, _, _) => {
            h155
        }
    }, match v2.clone() {
        Adt5::C0(h156, _, _) => {
            h156
        }
    }, match v1.clone() {
        Adt5::C0(_, h157, _) => {
            h157
        }
    }, match v1 {
        Adt5::C0(_, _, h158) => {
            h158
        }
    }, match v2.clone() {
        Adt5::C0(_, h159, _) => {
            h159
        }
    }, match v2 {
        Adt5::C0(_, _, h160) => {
            h160
        }
    })
}

pub fn f27(v0: u8) -> Bytes {
    let v1: Bytes = {
        let a161 = Str::lit("0123456789ABCDEF");
        utf8_encode(a161)
    };
    let v2: u64 = {
        let a162 = v0;
        let a163 = 0u8;
        if equal(a162, a163) {
            0u64
        } else {
            let a164 = v0;
            let a165 = 1u8;
            if equal(a164, a165) {
                1u64
            } else {
                let a166 = v0;
                let a167 = 2u8;
                if equal(a166, a167) {
                    2u64
                } else {
                    let a168 = v0;
                    let a169 = 3u8;
                    if equal(a168, a169) {
                        3u64
                    } else {
                        let a170 = v0;
                        let a171 = 4u8;
                        if equal(a170, a171) {
                            4u64
                        } else {
                            let a172 = v0;
                            let a173 = 5u8;
                            if equal(a172, a173) {
                                5u64
                            } else {
                                let a174 = v0;
                                let a175 = 6u8;
                                if equal(a174, a175) {
                                    6u64
                                } else {
                                    let a176 = v0;
                                    let a177 = 7u8;
                                    if equal(a176, a177) {
                                        7u64
                                    } else {
                                        let a178 = v0;
                                        let a179 = 8u8;
                                        if equal(a178, a179) {
                                            8u64
                                        } else {
                                            let a180 = v0;
                                            let a181 = 9u8;
                                            if equal(a180, a181) {
                                                9u64
                                            } else {
                                                let a182 = v0;
                                                let a183 = 10u8;
                                                if equal(a182, a183) {
                                                    10u64
                                                } else {
                                                    let a184 = v0;
                                                    let a185 = 11u8;
                                                    if equal(a184, a185) {
                                                        11u64
                                                    } else {
                                                        let a186 = v0;
                                                        let a187 = 12u8;
                                                        if equal(a186, a187) {
                                                            12u64
                                                        } else {
                                                            let a188 = v0;
                                                            let a189 = 13u8;
                                                            if equal(a188, a189) {
                                                                13u64
                                                            } else {
                                                                let a190 = v0;
                                                                let a191 = 14u8;
                                                                if equal(a190, a191) {
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
    f8(v1, v2, 1u64)
}

pub fn f28(v0: Bytes) -> R<Bytes> {
    let v1: u64 = {
        let (h192, _) = f31(Fn3::F32(v0.clone()), {
            let a193 = v0.clone();
            length_bytes(a193)
        }, 0u64)?;
        h192
    };
    Ok(f8(v0.clone(), v1, {
        let a195 = {
            let a194 = v0;
            length_bytes(a194)
        };
        let a196 = v1;
        nat_sub(a195, a196)
    }))
}

pub fn f29(v0: u64, v1: u64) -> Bytes {
    f8(f18(Str::lit("0123456789")), {
        let a200 = {
            let a197 = v0;
            let a198 = v1;
            let a199 = 0u64;
            nat_quot(a197, a198, a199)
        };
        let a201 = 10u64;
        let a202 = 0u64;
        nat_rem(a200, a201, a202)
    }, 1u64)
}

pub fn f30(v0: Bytes, v1: u64, v2: u64, v3: Adt5) -> R<Option<Adt5>> {
    let v4: u64 = match v3.clone() {
        Adt5::C0(h203, _, _) => {
            h203
        }
    };
    let a204 = v4;
    let a205 = v1;
    if nat_lt(a204, a205) {
        let v5: u8 = f10(v0, v4);
        let v6: bool = f33(v2, v5);
        if v6 {
            let a206 = v5;
            let a207 = 10u8;
            if equal(a206, a207) {
                Ok(Some(Adt5::C0({
                    let a208 = v4;
                    let a209 = 1u64;
                    nat_add(a208, a209)?
                }, {
                    let a211 = match v3 {
                        Adt5::C0(_, h210, _) => {
                            h210
                        }
                    };
                    let a212 = 1u64;
                    nat_add(a211, a212)?
                }, 1u64)))
            } else {
                Ok(Some(Adt5::C0({
                    let a213 = v4;
                    let a214 = 1u64;
                    nat_add(a213, a214)?
                }, match v3.clone() {
                    Adt5::C0(_, h215, _) => {
                        h215
                    }
                }, {
                    let a217 = match v3 {
                        Adt5::C0(_, _, h216) => {
                            h216
                        }
                    };
                    let a218 = 1u64;
                    nat_add(a217, a218)?
                })))
            }
        } else {
            Ok(None::<Adt5>)
        }
    } else {
        Ok(None::<Adt5>)
    }
}

pub fn f31(v0: Fn3, v1: u64, v2: u64) -> R<(u64, bool)> {
    let m219 = v1;
    if m219 == 0 {
        Ok((v2, false))
    } else {
        let v3 = m219 - 1;
        let m221 = {
            let c220: Fn3 = v0.clone();
            c220.apply(v2)?
        };
        match m221 {
            None => {
                Ok((v2, true))
            }
            Some(v4) => {
                f31(v0, v3, v4)
            }
        }
    }
}

pub fn f32(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f34(v0, v1)
}

pub fn f33(v0: u64, v1: u8) -> bool {
    let v2: bool = {
        let a222 = v1;
        let a223 = 32u8;
        equal(a222, a223)
    };
    let v3: bool = {
        let a224 = v1;
        let a225 = 10u8;
        equal(a224, a225)
    };
    let v4: bool = f13(v1);
    let v5: bool = f15(v1);
    let a226 = v0;
    let a227 = 0u64;
    if nat_eq(a226, a227) {
        if v2 {
            true
        } else {
            v3
        }
    } else {
        let a228 = v0;
        let a229 = 1u64;
        if nat_eq(a228, a229) {
            v4
        } else {
            v5
        }
    }
}

pub fn f34(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f25(v0, v1, 48u8) {
        Ok(Some({
            let a230 = v1;
            let a231 = 1u64;
            nat_add(a230, a231)?
        }))
    } else {
        Ok(None::<u64>)
    }
}
