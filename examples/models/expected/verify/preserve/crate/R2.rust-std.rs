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
}

#[derive(Clone)]
pub enum Adt1 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Fn0 {
    F10,
}

impl Fn0 {
    pub fn apply(&self, p0: (Option<(Adt1, i64)>, List<Adt1>), p1: i64) -> (Option<(Adt1, i64)>, List<Adt1>) {
        match self {
            Fn0::F10 => f10(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F16(List<i64>),
}

impl Fn1 {
    pub fn apply(&self, p0: (List<i64>, List<i64>), p1: List<i64>) -> R<(List<i64>, List<i64>)> {
        match self {
            Fn1::F16(k0) => f16(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F21,
}

impl Fn2 {
    pub fn apply(&self, p0: (i64, List<i64>), p1: i64) -> R<(i64, List<i64>)> {
        match self {
            Fn2::F21 => f21(p0, p1),
        }
    }
}

pub fn f0(v0: Adt0) -> R<Result<u64, (bool, bool)>> {
    let v1: Adt0 = v0;
    if f1(v1.clone()) {
        let v2: u64 = f2(v1)?;
        Ok(Ok::<u64, (bool, bool)>(v2))
    } else {
        Ok(Err::<u64, (bool, bool)>((false, false)))
    }
}

pub fn f1(v0: Adt0) -> bool {
    f3(v0)
}

pub fn f2(v0: Adt0) -> R<u64> {
    f4(v0)
}

pub fn f3(v0: Adt0) -> bool {
    let m1 = v0;
    match m1 {
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
        Adt0::C4 => {
            true
        }
        Adt0::C5 => {
            true
        }
        Adt0::C6 => {
            true
        }
        Adt0::C7 => {
            true
        }
    }
}

pub fn f4(v0: Adt0) -> R<u64> {
    let v1: Adt1 = f5(v0)?;
    let v2: u64 = f6(v1);
    Ok(v2)
}

pub fn f5(v0: Adt0) -> R<Adt1> {
    f7(v0)
}

pub fn f6(v0: Adt1) -> u64 {
    f8(v0)
}

pub fn f7(v0: Adt0) -> R<Adt1> {
    let m3 = {
        let (h2, _) = f9(Fn0::F10, (None::<(Adt1, i64)>, List::cons(Adt1::C0, List::cons(Adt1::C1, List::cons(Adt1::C2, List::<Adt1>::nil())))), f11(v0)?);
        h2
    };
    match m3 {
        None => {
            Ok(Adt1::C0)
        }
        Some(v1) => {
            let (h4, _) = v1;
            Ok(h4)
        }
    }
}

pub fn f8(v0: Adt1) -> u64 {
    if f12(v0.clone()) {
        2u64
    } else {
        if f13(v0) {
            1u64
        } else {
            0u64
        }
    }
}

pub fn f9(v0: Fn0, v1: (Option<(Adt1, i64)>, List<Adt1>), v2: List<i64>) -> (Option<(Adt1, i64)>, List<Adt1>) {
    let m5 = v2;
    match m5.uncons() {
        None => {
            v1
        }
        Some((v3, v4)) => {
            f9(v0.clone(), {
                let c6: Fn0 = v0;
                c6.apply(v1, v3)
            }, v4)
        }
    }
}

pub fn f10(v0: (Option<(Adt1, i64)>, List<Adt1>), v1: i64) -> (Option<(Adt1, i64)>, List<Adt1>) {
    let m8 = {
        let (_, h7) = v0.clone();
        h7
    };
    match m8.uncons() {
        None => {
            v0
        }
        Some((v2, v3)) => {
            ({
                let m10 = {
                    let (h9, _) = v0;
                    h9
                };
                match m10 {
                    None => {
                        Some((v2, v1))
                    }
                    Some(v4) => {
                        let m14 = {
                            let a12 = {
                                let (_, h11) = v4.clone();
                                h11
                            };
                            let a13 = v1;
                            compare(a12, a13)
                        };
                        if match m14 {
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

pub fn f11(v0: Adt0) -> R<List<i64>> {
    let v1: List<i64> = f14(v0);
    let (h15, _) = f15(Fn1::F16(v1), (List::<i64>::nil(), f17()), f18())?;
    Ok(h15)
}

pub fn f12(v0: Adt1) -> bool {
    let m16 = v0;
    match m16 {
        Adt1::C0 => {
            false
        }
        Adt1::C1 => {
            false
        }
        Adt1::C2 => {
            true
        }
    }
}

pub fn f13(v0: Adt1) -> bool {
    let m17 = v0;
    match m17 {
        Adt1::C0 => {
            false
        }
        Adt1::C1 => {
            true
        }
        Adt1::C2 => {
            false
        }
    }
}

pub fn f14(v0: Adt0) -> List<i64> {
    f19(v0)
}

pub fn f15(v0: Fn1, v1: (List<i64>, List<i64>), v2: List<List<i64>>) -> R<(List<i64>, List<i64>)> {
    let m18 = v2;
    match m18.uncons() {
        None => {
            Ok(v1)
        }
        Some((v3, v4)) => {
            f15(v0.clone(), {
                let c19: Fn1 = v0;
                c19.apply(v1, v3)?
            }, v4)
        }
    }
}

pub fn f16(v0: List<i64>, v1: (List<i64>, List<i64>), v2: List<i64>) -> R<(List<i64>, List<i64>)> {
    let m21 = {
        let (_, h20) = v1.clone();
        h20
    };
    match m21.uncons() {
        None => {
            Ok(v1)
        }
        Some((v3, v4)) => {
            Ok(({
                let a27 = {
                    let (h22, _) = v1;
                    h22
                };
                let a28 = List::cons({
                    let a25 = v3;
                    let a26 = {
                        let a24 = {
                            let (h23, _) = f20(Fn2::F21, (0i64, v0), v2)?;
                            h23
                        };
                        int_neg(a24)?
                    };
                    int_sub(a25, a26)?
                }, List::<i64>::nil());
                append_list(a27, a28)
            }, v4))
        }
    }
}

pub fn f17() -> List<i64> {
    List::cons(1i64, List::cons(0i64, List::cons(-1i64, List::<i64>::nil())))
}

pub fn f18() -> List<List<i64>> {
    List::cons(List::cons(-2i64, List::cons(-2i64, List::cons(-4i64, List::<i64>::nil()))), List::cons(List::cons(2i64, List::cons(2i64, List::cons(0i64, List::<i64>::nil()))), List::cons(List::cons(0i64, List::cons(1i64, List::cons(5i64, List::<i64>::nil()))), List::<List<i64>>::nil())))
}

pub fn f19(v0: Adt0) -> List<i64> {
    let m29 = v0;
    match m29 {
        Adt0::C0 => {
            List::cons(0i64, List::cons(0i64, List::cons(0i64, List::<i64>::nil())))
        }
        Adt0::C1 => {
            List::cons(1i64, List::cons(0i64, List::cons(0i64, List::<i64>::nil())))
        }
        Adt0::C2 => {
            List::cons(0i64, List::cons(1i64, List::cons(0i64, List::<i64>::nil())))
        }
        Adt0::C3 => {
            List::cons(1i64, List::cons(1i64, List::cons(0i64, List::<i64>::nil())))
        }
        Adt0::C4 => {
            List::cons(0i64, List::cons(0i64, List::cons(1i64, List::<i64>::nil())))
        }
        Adt0::C5 => {
            List::cons(1i64, List::cons(0i64, List::cons(1i64, List::<i64>::nil())))
        }
        Adt0::C6 => {
            List::cons(0i64, List::cons(1i64, List::cons(1i64, List::<i64>::nil())))
        }
        Adt0::C7 => {
            List::cons(1i64, List::cons(1i64, List::cons(1i64, List::<i64>::nil())))
        }
    }
}

pub fn f20(v0: Fn2, v1: (i64, List<i64>), v2: List<i64>) -> R<(i64, List<i64>)> {
    let m30 = v2;
    match m30.uncons() {
        None => {
            Ok(v1)
        }
        Some((v3, v4)) => {
            f20(v0.clone(), {
                let c31: Fn2 = v0;
                c31.apply(v1, v3)?
            }, v4)
        }
    }
}

pub fn f21(v0: (i64, List<i64>), v1: i64) -> R<(i64, List<i64>)> {
    let m33 = {
        let (_, h32) = v0.clone();
        h32
    };
    match m33.uncons() {
        None => {
            Ok(v0)
        }
        Some((v2, v3)) => {
            Ok(({
                let a38 = {
                    let (h34, _) = v0;
                    h34
                };
                let a39 = {
                    let a37 = {
                        let a35 = v1;
                        let a36 = v2;
                        int_mul(a35, a36)?
                    };
                    int_neg(a37)?
                };
                int_sub(a38, a39)?
            }, v3))
        }
    }
}
