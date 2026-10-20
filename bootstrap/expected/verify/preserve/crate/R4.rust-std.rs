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

pub fn f1(v0: Fn0, p12: u64, p13: Adt4) -> R<(Adt4, bool)> {
    let m11 = (p12, p13);
    {
        let mut m11 = m11;
        loop {
            let step = {
                let (v1, v2) = m11;
                let m8 = v1;
                if m8 == 0 {
                    Err::<(u64, Adt4), R<(Adt4, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m8 - 1;
                    let m10 = {
                        let c9: Fn0 = v0.clone();
                        c9.apply(v2.clone())?
                    };
                    match m10 {
                        None => {
                            Err::<(u64, Adt4), R<(Adt4, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt4), R<(Adt4, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m11 = next; }
                Err(done) => break done,
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
        Adt4::C0(h14, _, _, _) => {
            h14
        }
    };
    let v6: u64 = match v5.clone() {
        Adt5::C0(h15, _, _) => {
            h15
        }
    };
    let m17 = match v4.clone() {
        Adt4::C0(_, _, _, h16) => {
            h16
        }
    };
    match m17 {
        None => {
            let a18 = v6;
            let a19 = v2;
            if nat_lt(a18, a19) {
                let v8: Adt6 = f7(v0.clone(), v1.clone(), v5.clone())?;
                let m21 = match v8.clone() {
                    Adt6::C0(_, _, h20) => {
                        h20
                    }
                };
                match m21 {
                    None => {
                        let v10: Adt5 = match v8.clone() {
                            Adt6::C0(_, h24, _) => {
                                h24
                            }
                        };
                        let v11: Adt0 = Adt0::C0(match v8 {
                            Adt6::C0(h25, _, _) => {
                                h25
                            }
                        }, v6, match v10.clone() {
                            Adt5::C0(h26, _, _) => {
                                h26
                            }
                        }, match v5.clone() {
                            Adt5::C0(_, h27, _) => {
                                h27
                            }
                        }, match v5 {
                            Adt5::C0(_, _, h28) => {
                                h28
                            }
                        }, match v10.clone() {
                            Adt5::C0(_, h29, _) => {
                                h29
                            }
                        }, match v10.clone() {
                            Adt5::C0(_, _, h30) => {
                                h30
                            }
                        }, f8(v1, v6, {
                            let a32 = match v10.clone() {
                                Adt5::C0(h31, _, _) => {
                                    h31
                                }
                            };
                            let a33 = v6;
                            nat_sub(a32, a33)
                        }));
                        let v12: u64 = {
                            let a35 = match v4.clone() {
                                Adt4::C0(_, _, h34, _) => {
                                    h34
                                }
                            };
                            let a36 = 1u64;
                            nat_add(a35, a36)?
                        };
                        let a37 = v3;
                        let a38 = v12;
                        if nat_lt(a37, a38) {
                            let v13: Adt2 = f9(v0, v3)?;
                            Ok(Some(Adt4::C0(v10, match v4 {
                                Adt4::C0(_, h39, _, _) => {
                                    h39
                                }
                            }, v12, Some(v13))))
                        } else {
                            Ok(Some(Adt4::C0(v10, List::cons(v11, match v4 {
                                Adt4::C0(_, h40, _, _) => {
                                    h40
                                }
                            }), v12, None::<Adt2>)))
                        }
                    }
                    Some(v9) => {
                        Ok(Some(Adt4::C0(v5, match v4.clone() {
                            Adt4::C0(_, h22, _, _) => {
                                h22
                            }
                        }, match v4 {
                            Adt4::C0(_, _, h23, _) => {
                                h23
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

pub fn f5(v0: Fn1, p44: List<Adt0>, p45: List<Adt0>) -> List<Adt0> {
    let m43 = (p44, p45);
    {
        let mut m43 = m43;
        loop {
            let step = {
                let (v1, v2) = m43;
                let m41 = v2;
                match m41.uncons() {
                    None => {
                        Err::<(List<Adt0>, List<Adt0>), List<Adt0>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt0>, List<Adt0>), List<Adt0>>(({
                            let c42: Fn1 = v0.clone();
                            c42.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m43 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f6(v0: List<Adt0>, v1: Adt0) -> List<Adt0> {
    List::cons(v1, v0)
}

pub fn f7(v0: Bytes, v1: Bytes, v2: Adt5) -> R<Adt6> {
    let v3: u64 = {
        let a46 = v1.clone();
        length_bytes(a46)
    };
    let v4: u64 = match v2.clone() {
        Adt5::C0(h47, _, _) => {
            h47
        }
    };
    let v5: u8 = f10(v1.clone(), v4);
    let v6: Adt5 = f11(v1.clone(), 0u64, v2.clone())?;
    let v7: Adt5 = f11(v1.clone(), 1u64, v2.clone())?;
    let v8: Adt5 = f11(v1.clone(), 2u64, v2.clone())?;
    let v9: Adt5 = f12(v1.clone(), v2.clone())?;
    let v10: u64 = match v9.clone() {
        Adt5::C0(h48, _, _) => {
            h48
        }
    };
    let v11: u8 = f10(v1.clone(), v10);
    let v12: bool = {
        let a49 = v10;
        let a50 = v3;
        nat_lt(a49, a50)
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
        let a51 = v5;
        let a52 = 123u8;
        if equal(a51, a52) {
            true
        } else {
            let a53 = v5;
            let a54 = 125u8;
            if equal(a53, a54) {
                true
            } else {
                let a55 = v5;
                let a56 = 40u8;
                if equal(a55, a56) {
                    true
                } else {
                    let a57 = v5;
                    let a58 = 41u8;
                    if equal(a57, a58) {
                        true
                    } else {
                        let a59 = v5;
                        let a60 = 91u8;
                        if equal(a59, a60) {
                            true
                        } else {
                            let a61 = v5;
                            let a62 = 93u8;
                            equal(a61, a62)
                        }
                    }
                }
            }
        }
    };
    let a63 = v5;
    let a64 = 32u8;
    if if equal(a63, a64) {
        true
    } else {
        let a65 = v5;
        let a66 = 10u8;
        equal(a65, a66)
    } {
        Ok(Adt6::C0(Adt1::C6, v6, None::<Adt2>))
    } else {
        let a67 = v5;
        let a68 = 92u8;
        if equal(a67, a68) {
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
    let m72 = {
        let a69 = v0;
        let a70 = v1;
        let a71 = v2;
        slice_bytes(a69, a70, a71)
    };
    match m72 {
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
    let m73 = f21(v0, v1);
    match m73 {
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
        let a74 = v0.clone();
        length_bytes(a74)
    };
    let (h75, _) = f22(Fn2::F23(v1, v3, v0), {
        let a77 = v3;
        let a78 = match v2.clone() {
            Adt5::C0(h76, _, _) => {
                h76
            }
        };
        nat_sub(a77, a78)
    }, v2)?;
    Ok(h75)
}

pub fn f12(v0: Bytes, v1: Adt5) -> R<Adt5> {
    let v2: u64 = match v1.clone() {
        Adt5::C0(h79, _, _) => {
            h79
        }
    };
    let v3: u64 = f24(v0.clone(), v2);
    let v4: bool = f25(v0, v2, 10u8);
    if v4 {
        Ok(Adt5::C0({
            let a80 = v2;
            let a81 = 1u64;
            nat_add(a80, a81)?
        }, {
            let a83 = match v1 {
                Adt5::C0(_, h82, _) => {
                    h82
                }
            };
            let a84 = 1u64;
            nat_add(a83, a84)?
        }, 1u64))
    } else {
        Ok(Adt5::C0({
            let a85 = v2;
            let a86 = v3;
            nat_add(a85, a86)?
        }, match v1.clone() {
            Adt5::C0(_, h87, _) => {
                h87
            }
        }, {
            let a89 = match v1 {
                Adt5::C0(_, _, h88) => {
                    h88
                }
            };
            let a90 = 1u64;
            nat_add(a89, a90)?
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
    let m93 = {
        let a91 = v0;
        let a92 = v1;
        fixed_u8::checked_sub(a91, a92)
    };
    match m93 {
        None => {
            false
        }
        Some(_) => {
            let m96 = {
                let a94 = v2;
                let a95 = v0;
                fixed_u8::checked_sub(a94, a95)
            };
            match m96 {
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
        Adt5::C0(h97, _, _) => {
            h97
        }
    });
    let v5: u8 = {
        let m100 = {
            let a98 = v3;
            let a99 = 4u32;
            fixed_u8::shift_right(a98, a99)
        };
        match m100 {
            None => {
                0u8
            }
            Some(v4) => {
                v4
            }
        }
    };
    let v6: u8 = {
        let a101 = v3;
        let a102 = 15u8;
        fixed_u8::bit_and(a101, a102)
    };
    let v7: Bytes = f27(v5);
    let v8: Bytes = f27(v6);
    let v9: Adt5 = Adt5::C0({
        let a104 = match v2.clone() {
            Adt5::C0(h103, _, _) => {
                h103
            }
        };
        let a105 = 1u64;
        nat_add(a104, a105)?
    }, match v2.clone() {
        Adt5::C0(_, h106, _) => {
            h106
        }
    }, {
        let a108 = match v2.clone() {
            Adt5::C0(_, _, h107) => {
                h107
            }
        };
        let a109 = 1u64;
        nat_add(a108, a109)?
    });
    let v10: Adt3 = f26(v0, v2, v9);
    Ok(Adt2::C0(f18(Str::lit("LLL1001")), f19(f19(f18(Str::lit("forbidden scalar U+00")), v7), v8), Some(v10), None::<Bytes>))
}

pub fn f18(v0: Str) -> Bytes {
    let a110 = v0;
    utf8_encode(a110)
}

pub fn f19(v0: Bytes, v1: Bytes) -> Bytes {
    let a111 = v0;
    let a112 = v1;
    append_bytes(a111, a112)
}

pub fn f20(v0: u64) -> R<Bytes> {
    let v1: Bytes = f18(Str::lit("0"));
    let v2: Bytes = f28({
        let a149 = f29(v0, 10000000000000000000u64);
        let a150 = {
            let a147 = f29(v0, 1000000000000000000u64);
            let a148 = {
                let a145 = f29(v0, 100000000000000000u64);
                let a146 = {
                    let a143 = f29(v0, 10000000000000000u64);
                    let a144 = {
                        let a141 = f29(v0, 1000000000000000u64);
                        let a142 = {
                            let a139 = f29(v0, 100000000000000u64);
                            let a140 = {
                                let a137 = f29(v0, 10000000000000u64);
                                let a138 = {
                                    let a135 = f29(v0, 1000000000000u64);
                                    let a136 = {
                                        let a133 = f29(v0, 100000000000u64);
                                        let a134 = {
                                            let a131 = f29(v0, 10000000000u64);
                                            let a132 = {
                                                let a129 = f29(v0, 1000000000u64);
                                                let a130 = {
                                                    let a127 = f29(v0, 100000000u64);
                                                    let a128 = {
                                                        let a125 = f29(v0, 10000000u64);
                                                        let a126 = {
                                                            let a123 = f29(v0, 1000000u64);
                                                            let a124 = {
                                                                let a121 = f29(v0, 100000u64);
                                                                let a122 = {
                                                                    let a119 = f29(v0, 10000u64);
                                                                    let a120 = {
                                                                        let a117 = f29(v0, 1000u64);
                                                                        let a118 = {
                                                                            let a115 = f29(v0, 100u64);
                                                                            let a116 = {
                                                                                let a113 = f29(v0, 10u64);
                                                                                let a114 = f29(v0, 1u64);
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
                };
                append_bytes(a145, a146)
            };
            append_bytes(a147, a148)
        };
        append_bytes(a149, a150)
    })?;
    let a151 = v0;
    let a152 = 0u64;
    if nat_eq(a151, a152) {
        Ok(v1)
    } else {
        Ok(v2)
    }
}

pub fn f21(v0: Bytes, v1: u64) -> Option<u8> {
    let a153 = v0;
    let a154 = v1;
    index_bytes(a153, a154)
}

pub fn f22(v0: Fn2, p159: u64, p160: Adt5) -> R<(Adt5, bool)> {
    let m158 = (p159, p160);
    {
        let mut m158 = m158;
        loop {
            let step = {
                let (v1, v2) = m158;
                let m155 = v1;
                if m155 == 0 {
                    Err::<(u64, Adt5), R<(Adt5, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m155 - 1;
                    let m157 = {
                        let c156: Fn2 = v0.clone();
                        c156.apply(v2.clone())?
                    };
                    match m157 {
                        None => {
                            Err::<(u64, Adt5), R<(Adt5, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt5), R<(Adt5, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m158 = next; }
                Err(done) => break done,
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
    let m161 = f21(v0, v1);
    match m161 {
        None => {
            false
        }
        Some(v3) => {
            let a162 = v3;
            let a163 = v2;
            equal(a162, a163)
        }
    }
}

pub fn f26(v0: Bytes, v1: Adt5, v2: Adt5) -> Adt3 {
    Adt3::C0(v0, match v1.clone() {
        Adt5::C0(h164, _, _) => {
            h164
        }
    }, match v2.clone() {
        Adt5::C0(h165, _, _) => {
            h165
        }
    }, match v1.clone() {
        Adt5::C0(_, h166, _) => {
            h166
        }
    }, match v1 {
        Adt5::C0(_, _, h167) => {
            h167
        }
    }, match v2.clone() {
        Adt5::C0(_, h168, _) => {
            h168
        }
    }, match v2 {
        Adt5::C0(_, _, h169) => {
            h169
        }
    })
}

pub fn f27(v0: u8) -> Bytes {
    let v1: Bytes = {
        let a170 = Str::lit("0123456789ABCDEF");
        utf8_encode(a170)
    };
    let v2: u64 = {
        let a171 = v0;
        let a172 = 0u8;
        if equal(a171, a172) {
            0u64
        } else {
            let a173 = v0;
            let a174 = 1u8;
            if equal(a173, a174) {
                1u64
            } else {
                let a175 = v0;
                let a176 = 2u8;
                if equal(a175, a176) {
                    2u64
                } else {
                    let a177 = v0;
                    let a178 = 3u8;
                    if equal(a177, a178) {
                        3u64
                    } else {
                        let a179 = v0;
                        let a180 = 4u8;
                        if equal(a179, a180) {
                            4u64
                        } else {
                            let a181 = v0;
                            let a182 = 5u8;
                            if equal(a181, a182) {
                                5u64
                            } else {
                                let a183 = v0;
                                let a184 = 6u8;
                                if equal(a183, a184) {
                                    6u64
                                } else {
                                    let a185 = v0;
                                    let a186 = 7u8;
                                    if equal(a185, a186) {
                                        7u64
                                    } else {
                                        let a187 = v0;
                                        let a188 = 8u8;
                                        if equal(a187, a188) {
                                            8u64
                                        } else {
                                            let a189 = v0;
                                            let a190 = 9u8;
                                            if equal(a189, a190) {
                                                9u64
                                            } else {
                                                let a191 = v0;
                                                let a192 = 10u8;
                                                if equal(a191, a192) {
                                                    10u64
                                                } else {
                                                    let a193 = v0;
                                                    let a194 = 11u8;
                                                    if equal(a193, a194) {
                                                        11u64
                                                    } else {
                                                        let a195 = v0;
                                                        let a196 = 12u8;
                                                        if equal(a195, a196) {
                                                            12u64
                                                        } else {
                                                            let a197 = v0;
                                                            let a198 = 13u8;
                                                            if equal(a197, a198) {
                                                                13u64
                                                            } else {
                                                                let a199 = v0;
                                                                let a200 = 14u8;
                                                                if equal(a199, a200) {
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
        let (h201, _) = f31(Fn3::F32(v0.clone()), {
            let a202 = v0.clone();
            length_bytes(a202)
        }, 0u64)?;
        h201
    };
    Ok(f8(v0.clone(), v1, {
        let a204 = {
            let a203 = v0;
            length_bytes(a203)
        };
        let a205 = v1;
        nat_sub(a204, a205)
    }))
}

pub fn f29(v0: u64, v1: u64) -> Bytes {
    f8(f18(Str::lit("0123456789")), {
        let a209 = {
            let a206 = v0;
            let a207 = v1;
            let a208 = 0u64;
            nat_quot(a206, a207, a208)
        };
        let a210 = 10u64;
        let a211 = 0u64;
        nat_rem(a209, a210, a211)
    }, 1u64)
}

pub fn f30(v0: Bytes, v1: u64, v2: u64, v3: Adt5) -> R<Option<Adt5>> {
    let v4: u64 = match v3.clone() {
        Adt5::C0(h212, _, _) => {
            h212
        }
    };
    let a213 = v4;
    let a214 = v1;
    if nat_lt(a213, a214) {
        let v5: u8 = f10(v0, v4);
        let v6: bool = f33(v2, v5);
        if v6 {
            let a215 = v5;
            let a216 = 10u8;
            if equal(a215, a216) {
                Ok(Some(Adt5::C0({
                    let a217 = v4;
                    let a218 = 1u64;
                    nat_add(a217, a218)?
                }, {
                    let a220 = match v3 {
                        Adt5::C0(_, h219, _) => {
                            h219
                        }
                    };
                    let a221 = 1u64;
                    nat_add(a220, a221)?
                }, 1u64)))
            } else {
                Ok(Some(Adt5::C0({
                    let a222 = v4;
                    let a223 = 1u64;
                    nat_add(a222, a223)?
                }, match v3.clone() {
                    Adt5::C0(_, h224, _) => {
                        h224
                    }
                }, {
                    let a226 = match v3 {
                        Adt5::C0(_, _, h225) => {
                            h225
                        }
                    };
                    let a227 = 1u64;
                    nat_add(a226, a227)?
                })))
            }
        } else {
            Ok(None::<Adt5>)
        }
    } else {
        Ok(None::<Adt5>)
    }
}

pub fn f31(v0: Fn3, p232: u64, p233: u64) -> R<(u64, bool)> {
    let m231 = (p232, p233);
    {
        let mut m231 = m231;
        loop {
            let step = {
                let (v1, v2) = m231;
                let m228 = v1;
                if m228 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m228 - 1;
                    let m230 = {
                        let c229: Fn3 = v0.clone();
                        c229.apply(v2)?
                    };
                    match m230 {
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
                Ok(next) => { m231 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f32(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f34(v0, v1)
}

pub fn f33(v0: u64, v1: u8) -> bool {
    let v2: bool = {
        let a234 = v1;
        let a235 = 32u8;
        equal(a234, a235)
    };
    let v3: bool = {
        let a236 = v1;
        let a237 = 10u8;
        equal(a236, a237)
    };
    let v4: bool = f13(v1);
    let v5: bool = f15(v1);
    let a238 = v0;
    let a239 = 0u64;
    if nat_eq(a238, a239) {
        if v2 {
            true
        } else {
            v3
        }
    } else {
        let a240 = v0;
        let a241 = 1u64;
        if nat_eq(a240, a241) {
            v4
        } else {
            v5
        }
    }
}

pub fn f34(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f25(v0, v1, 48u8) {
        Ok(Some({
            let a242 = v1;
            let a243 = 1u64;
            nat_add(a242, a243)?
        }))
    } else {
        Ok(None::<u64>)
    }
}
