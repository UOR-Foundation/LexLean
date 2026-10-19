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

pub fn f0(v0: List<(Str, List<Str>)>) -> R<Option<List<Str>>> {
    f1(v0)
}

pub fn f1(v0: List<(Str, List<Str>)>) -> R<Option<List<Str>>> {
    let v1: List<Str> = f6(v0.clone(), List::<Str>::nil());
    Ok(f2(v0, {
        let a2 = {
            let a1 = v1.clone();
            length_list(a1)
        };
        let a3 = 1u64;
        nat_add(a2, a3)?
    }, v1, List::<Str>::nil()))
}

pub fn f2(v0: List<(Str, List<Str>)>, p9: u64, p10: List<Str>, p11: List<Str>) -> Option<List<Str>> {
    let m8 = (p9, (p10, p11));
    {
        let mut m8 = m8;
        loop {
            let step = {
                let (v1, (v2, v3)) = m8;
                let m4 = v1;
                if m4 == 0 {
                    let m5 = v2;
                    match m5.uncons() {
                        None => {
                            Err::<(u64, (List<Str>, List<Str>)), Option<List<Str>>>(Some(f5(v3, List::<Str>::nil())))
                        }
                        Some((_, _)) => {
                            Err::<(u64, (List<Str>, List<Str>)), Option<List<Str>>>(None::<List<Str>>)
                        }
                    }
                } else {
                    let v6 = m4 - 1;
                    let m6 = f3(v0.clone(), v2.clone(), v2.clone());
                    match m6.uncons() {
                        None => {
                            let m7 = v2;
                            match m7.uncons() {
                                None => {
                                    Err::<(u64, (List<Str>, List<Str>)), Option<List<Str>>>(Some(f5(v3, List::<Str>::nil())))
                                }
                                Some((_, _)) => {
                                    Err::<(u64, (List<Str>, List<Str>)), Option<List<Str>>>(None::<List<Str>>)
                                }
                            }
                        }
                        Some((v9, _)) => {
                            Ok::<(u64, (List<Str>, List<Str>)), Option<List<Str>>>((v6, (f12(v2, v9.clone()), List::cons(v9, v3))))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m8 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f3(v0: List<(Str, List<Str>)>, v1: List<Str>, v2: List<Str>) -> List<Str> {
    let m12 = v2;
    match m12.uncons() {
        None => {
            List::<Str>::nil()
        }
        Some((v3, v4)) => {
            if f4(v0.clone(), v3.clone(), v1.clone()) {
                List::cons(v3, f3(v0, v1, v4))
            } else {
                f3(v0, v1, v4)
            }
        }
    }
}

pub fn f4(v0: List<(Str, List<Str>)>, v1: Str, p15: List<Str>) -> bool {
    let m14 = p15;
    {
        let mut m14 = m14;
        loop {
            let step = {
                let v2 = m14;
                let m13 = v2;
                match m13.uncons() {
                    None => {
                        Err::<List<Str>, bool>(true)
                    }
                    Some((v3, v4)) => {
                        if f11(f9(v0.clone(), v3), v1.clone()) {
                            Err::<List<Str>, bool>(false)
                        } else {
                            Ok::<List<Str>, bool>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m14 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f5(p18: List<Str>, p19: List<Str>) -> List<Str> {
    let m17 = (p18, p19);
    {
        let mut m17 = m17;
        loop {
            let step = {
                let (v0, v1) = m17;
                let m16 = v0;
                match m16.uncons() {
                    None => {
                        Err::<(List<Str>, List<Str>), List<Str>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Str>, List<Str>), List<Str>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m17 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f6(p24: List<(Str, List<Str>)>, p25: List<Str>) -> List<Str> {
    let m23 = (p24, p25);
    {
        let mut m23 = m23;
        loop {
            let step = {
                let (v0, v1) = m23;
                let m20 = v0;
                match m20.uncons() {
                    None => {
                        Err::<(List<(Str, List<Str>)>, List<Str>), List<Str>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, List<Str>)>, List<Str>), List<Str>>((v3, f7({
                            let (_, h21) = v2;
                            h21
                        }, f8(v1, {
                            let (h22, _) = v2;
                            h22
                        }))))
                    }
                }
            };
            match step {
                Ok(next) => { m23 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f7(p28: List<Str>, p29: List<Str>) -> List<Str> {
    let m27 = (p28, p29);
    {
        let mut m27 = m27;
        loop {
            let step = {
                let (v0, v1) = m27;
                let m26 = v0;
                match m26.uncons() {
                    None => {
                        Err::<(List<Str>, List<Str>), List<Str>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Str>, List<Str>), List<Str>>((v3, f8(v1, v2)))
                    }
                }
            };
            match step {
                Ok(next) => { m27 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f8(v0: List<Str>, v1: Str) -> List<Str> {
    let m30 = v0.clone();
    match m30.uncons() {
        None => {
            List::cons(v1, List::<Str>::nil())
        }
        Some((v2, v3)) => {
            let m33 = {
                let a31 = v1.clone();
                let a32 = v2.clone();
                compare(a31, a32)
            };
            match m33 {
                Ordering::Less => {
                    List::cons(v1, v0)
                }
                Ordering::Equal => {
                    v0
                }
                Ordering::Greater => {
                    List::cons(v2, f8(v3, v1))
                }
            }
        }
    }
}

pub fn f9(v0: List<(Str, List<Str>)>, v1: Str) -> List<Str> {
    let m34 = f10(v0, v1);
    match m34 {
        None => {
            List::<Str>::nil()
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f10(p42: List<(Str, List<Str>)>, v1: Str) -> Option<List<Str>> {
    let m41 = p42;
    {
        let mut m41 = m41;
        loop {
            let step = {
                let v0 = m41;
                let m35 = v0;
                match m35.uncons() {
                    None => {
                        Err::<List<(Str, List<Str>)>, Option<List<Str>>>(None::<List<Str>>)
                    }
                    Some((v2, v3)) => {
                        let m39 = {
                            let a37 = v1.clone();
                            let a38 = {
                                let (h36, _) = v2;
                                h36
                            };
                            compare(a37, a38)
                        };
                        match m39 {
                            Ordering::Less => {
                                Err::<List<(Str, List<Str>)>, Option<List<Str>>>(None::<List<Str>>)
                            }
                            Ordering::Equal => {
                                Err::<List<(Str, List<Str>)>, Option<List<Str>>>(Some({
                                    let (_, h40) = v2;
                                    h40
                                }))
                            }
                            Ordering::Greater => {
                                Ok::<List<(Str, List<Str>)>, Option<List<Str>>>(v3)
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m41 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f11(p48: List<Str>, v1: Str) -> bool {
    let m47 = p48;
    {
        let mut m47 = m47;
        loop {
            let step = {
                let v0 = m47;
                let m43 = v0;
                match m43.uncons() {
                    None => {
                        Err::<List<Str>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let m46 = {
                            let a44 = v1.clone();
                            let a45 = v2;
                            compare(a44, a45)
                        };
                        match m46 {
                            Ordering::Less => {
                                Err::<List<Str>, bool>(false)
                            }
                            Ordering::Equal => {
                                Err::<List<Str>, bool>(true)
                            }
                            Ordering::Greater => {
                                Ok::<List<Str>, bool>(v3)
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m47 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f12(v0: List<Str>, v1: Str) -> List<Str> {
    let m49 = v0.clone();
    match m49.uncons() {
        None => {
            List::<Str>::nil()
        }
        Some((v2, v3)) => {
            let m52 = {
                let a50 = v1.clone();
                let a51 = v2.clone();
                compare(a50, a51)
            };
            match m52 {
                Ordering::Less => {
                    v0
                }
                Ordering::Equal => {
                    v3
                }
                Ordering::Greater => {
                    List::cons(v2, f12(v3, v1))
                }
            }
        }
    }
}

pub fn f13(p62: List<(Str, List<Str>)>) -> bool {
    let m61 = p62;
    {
        let mut m61 = m61;
        loop {
            let step = {
                let v0 = m61;
                let m53 = v0;
                match m53.uncons() {
                    None => {
                        Err::<List<(Str, List<Str>)>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f14({
                            let (_, h54) = v1;
                            h54
                        }) {
                            let m55 = v2.clone();
                            match m55.uncons() {
                                None => {
                                    Err::<List<(Str, List<Str>)>, bool>(true)
                                }
                                Some((v3, _)) => {
                                    let m60 = {
                                        let a58 = {
                                            let (h56, _) = v1;
                                            h56
                                        };
                                        let a59 = {
                                            let (h57, _) = v3;
                                            h57
                                        };
                                        compare(a58, a59)
                                    };
                                    if match m60 {
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
                                        Ok::<List<(Str, List<Str>)>, bool>(v2)
                                    } else {
                                        Err::<List<(Str, List<Str>)>, bool>(false)
                                    }
                                }
                            }
                        } else {
                            Err::<List<(Str, List<Str>)>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m61 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f14(p69: List<Str>) -> bool {
    let m68 = p69;
    {
        let mut m68 = m68;
        loop {
            let step = {
                let v0 = m68;
                let m63 = v0;
                match m63.uncons() {
                    None => {
                        Err::<List<Str>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let m64 = v2.clone();
                        match m64.uncons() {
                            None => {
                                Err::<List<Str>, bool>(true)
                            }
                            Some((v3, _)) => {
                                let m67 = {
                                    let a65 = v1;
                                    let a66 = v3;
                                    compare(a65, a66)
                                };
                                if match m67 {
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
                                    Ok::<List<Str>, bool>(v2)
                                } else {
                                    Err::<List<Str>, bool>(false)
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m68 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f15(v0: List<(Str, List<Str>)>) -> R<Option<Option<List<Str>>>> {
    if f13(v0.clone()) {
        Ok(Some(f0(v0)?))
    } else {
        Ok(None::<Option<List<Str>>>)
    }
}

pub fn run(p0: List<(Str, List<Str>)>) -> R<Option<Option<List<Str>>>> {
    f15(p0)
}
