//! The fixed Rust runtime every rendering carries (SPEC.md §17.16): checked
//! arithmetic, scalar equality and key order, the work counter, and, in
//! `rust-std`, shared strings, byte strings, persistent lists, and the
//! primitives over them.
//!
//! The runtime is not generated: it is a closed, reviewed text whose
//! SHA-256 every package's provenance records, and generated code reaches it
//! only through [`Item`], so no rendering can name a runtime function that
//! does not exist or call one with the wrong failure mode.

use super::super::IntKind;

/// The runtime both profiles carry.
pub const CORE: &str = r#"use core::cmp::Ordering;
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
"#;

/// The runtime only `rust-std` carries. Every loop ticks the work counter
/// once per iteration and a bulk copy ticks its length, except the release
/// of a list, whose cells were each counted when built.
pub const STD: &str = r#"
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
pub struct List<T>(Option<std::rc::Rc<Node<T>>>);
impl<T> Clone for List<T> { fn clone(&self) -> Self { List(self.0.clone()) } }
impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut next = self.0.take();
        while let Some(cell) = next { // release
            match std::rc::Rc::try_unwrap(cell) {
                Ok(mut node) => next = node.tail.0.take(),
                Err(_) => break,
            }
        }
    }
}
impl<T: Clone> List<T> {
    pub fn nil() -> Self { List(None) }
    pub fn cons(head: T, tail: List<T>) -> Self { List(Some(std::rc::Rc::new(Node { head, tail }))) }
    pub fn uncons(&self) -> Option<(T, List<T>)> { self.0.as_ref().map(|node| (node.head.clone(), node.tail.clone())) }
    fn items(&self) -> Vec<T> {
        let mut out = Vec::new();
        let mut cursor = self.0.as_ref();
        while let Some(node) = cursor { tick(1); out.push(node.head.clone()); cursor = node.tail.0.as_ref(); }
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
pub fn length_list<T>(a: List<T>) -> u64 {
    let mut count = 0;
    let mut cursor = a.0.as_ref();
    while let Some(node) = cursor { tick(1); count += 1; cursor = node.tail.0.as_ref(); }
    count
}
pub fn length_bytes(a: Bytes) -> u64 { a.0.len() as u64 }
pub fn length_string(a: Str) -> u64 { chars(&a.0) }
pub fn index_list<T: Clone>(a: List<T>, i: u64) -> Option<T> {
    let mut position = 0;
    let mut cursor = a.0.as_ref();
    while let Some(node) = cursor {
        tick(1);
        if position == i { return Some(node.head.clone()); }
        position += 1;
        cursor = node.tail.0.as_ref();
    }
    None
}
pub fn index_bytes(a: Bytes, i: u64) -> Option<u8> { usize::try_from(i).ok().and_then(|i| a.0.get(i)).copied() }
pub fn slice_list<T: Clone>(a: List<T>, start: u64, count: u64) -> Option<List<T>> {
    let end = u128::from(start) + u128::from(count);
    let mut part = Vec::new();
    let mut position: u128 = 0;
    let mut cursor = a.0.as_ref();
    while let Some(node) = cursor {
        tick(1);
        if position >= end { break; }
        if position >= u128::from(start) { part.push(node.head.clone()); }
        position += 1;
        cursor = node.tail.0.as_ref();
    }
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
    let mut cursor = texts.0.as_ref();
    let mut first = true;
    while let Some(node) = cursor {
        tick(1);
        if !first { chars(&delimiter.0); out.push_str(&delimiter.0); }
        first = false;
        chars(&node.head.0);
        out.push_str(&node.head.0);
        cursor = node.tail.0.as_ref();
    }
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
"#;

/// A runtime function generated code may call.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Item {
    NatAdd,
    NatSub,
    NatMul,
    NatQuot,
    NatRem,
    NatEq,
    NatLe,
    NatLt,
    NatSucc,
    IntAdd,
    IntSub,
    IntMul,
    IntNeg,
    IntQuot,
    IntRem,
    BoolNot,
    BoolAnd,
    BoolOr,
    Equal,
    Compare,
    CheckedAdd(IntKind),
    CheckedSub(IntKind),
    CheckedMul(IntKind),
    CheckedQuot(IntKind),
    CheckedNeg(IntKind),
    BitAnd(IntKind),
    BitOr(IntKind),
    BitXor(IntKind),
    BitNot(IntKind),
    ShiftLeft(IntKind),
    ShiftRight(IntKind),
    Convert(IntKind),
    AppendList,
    AppendBytes,
    LengthList,
    LengthBytes,
    LengthString,
    IndexList,
    IndexBytes,
    SliceList,
    SliceBytes,
    Utf8Encode,
    Utf8Decode,
    CompareBytes,
    SplitExact,
    Join,
    FormatInt,
    FormatFixed(IntKind),
    ParseInt,
    ParseFixed(IntKind),
}

impl Item {
    /// The path generated code calls.
    #[must_use]
    pub fn path(self) -> String {
        let fixed = |module: &str, kind: IntKind| format!("fixed_{}::{module}", kind.name());
        match self {
            Self::NatAdd => "nat_add".to_owned(),
            Self::NatSub => "nat_sub".to_owned(),
            Self::NatMul => "nat_mul".to_owned(),
            Self::NatQuot => "nat_quot".to_owned(),
            Self::NatRem => "nat_rem".to_owned(),
            Self::NatEq => "nat_eq".to_owned(),
            Self::NatLe => "nat_le".to_owned(),
            Self::NatLt => "nat_lt".to_owned(),
            Self::NatSucc => "nat_succ".to_owned(),
            Self::IntAdd => "int_add".to_owned(),
            Self::IntSub => "int_sub".to_owned(),
            Self::IntMul => "int_mul".to_owned(),
            Self::IntNeg => "int_neg".to_owned(),
            Self::IntQuot => "int_quot".to_owned(),
            Self::IntRem => "int_rem".to_owned(),
            Self::BoolNot => "bool_not".to_owned(),
            Self::BoolAnd => "bool_and".to_owned(),
            Self::BoolOr => "bool_or".to_owned(),
            Self::Equal => "equal".to_owned(),
            Self::Compare => "compare".to_owned(),
            Self::CheckedAdd(kind) => fixed("checked_add", kind),
            Self::CheckedSub(kind) => fixed("checked_sub", kind),
            Self::CheckedMul(kind) => fixed("checked_mul", kind),
            Self::CheckedQuot(kind) => fixed("checked_quot", kind),
            Self::CheckedNeg(kind) => format!("checked_neg_{}", kind.name()),
            Self::BitAnd(kind) => fixed("bit_and", kind),
            Self::BitOr(kind) => fixed("bit_or", kind),
            Self::BitXor(kind) => fixed("bit_xor", kind),
            Self::BitNot(kind) => fixed("bit_not", kind),
            Self::ShiftLeft(kind) => fixed("shift_left", kind),
            Self::ShiftRight(kind) => fixed("shift_right", kind),
            Self::Convert(kind) => fixed("convert", kind),
            Self::AppendList => "append_list".to_owned(),
            Self::AppendBytes => "append_bytes".to_owned(),
            Self::LengthList => "length_list".to_owned(),
            Self::LengthBytes => "length_bytes".to_owned(),
            Self::LengthString => "length_string".to_owned(),
            Self::IndexList => "index_list".to_owned(),
            Self::IndexBytes => "index_bytes".to_owned(),
            Self::SliceList => "slice_list".to_owned(),
            Self::SliceBytes => "slice_bytes".to_owned(),
            Self::Utf8Encode => "utf8_encode".to_owned(),
            Self::Utf8Decode => "utf8_decode".to_owned(),
            Self::CompareBytes => "compare_bytes".to_owned(),
            Self::SplitExact => "split_exact".to_owned(),
            Self::Join => "join".to_owned(),
            Self::FormatInt => "format_i64".to_owned(),
            Self::FormatFixed(kind) => format!("format_{}", kind.name()),
            Self::ParseInt => "parse_int".to_owned(),
            Self::ParseFixed(kind) => format!("parse_{}", kind.name()),
        }
    }

    /// The width a fixed-width function works at.
    #[must_use]
    pub const fn width(self) -> Option<IntKind> {
        match self {
            Self::CheckedAdd(kind)
            | Self::CheckedSub(kind)
            | Self::CheckedMul(kind)
            | Self::CheckedQuot(kind)
            | Self::CheckedNeg(kind)
            | Self::BitAnd(kind)
            | Self::BitOr(kind)
            | Self::BitXor(kind)
            | Self::BitNot(kind)
            | Self::ShiftLeft(kind)
            | Self::ShiftRight(kind)
            | Self::Convert(kind)
            | Self::FormatFixed(kind)
            | Self::ParseFixed(kind) => Some(kind),
            _ => None,
        }
    }

    /// Whether the function can fail with `Overflow`, so its result is
    /// `R<T>` and a call propagates with `?`.
    #[must_use]
    pub const fn fallible(self) -> bool {
        matches!(
            self,
            Self::NatAdd
                | Self::NatMul
                | Self::NatSucc
                | Self::IntAdd
                | Self::IntSub
                | Self::IntMul
                | Self::IntNeg
                | Self::IntQuot
                | Self::ParseInt
        )
    }

    /// Whether the function exists only in `rust-std`, because it takes or
    /// returns heap storage.
    #[must_use]
    pub const fn heap(self) -> bool {
        matches!(
            self,
            Self::AppendList
                | Self::AppendBytes
                | Self::LengthList
                | Self::LengthBytes
                | Self::LengthString
                | Self::IndexList
                | Self::IndexBytes
                | Self::SliceList
                | Self::SliceBytes
                | Self::Utf8Encode
                | Self::Utf8Decode
                | Self::CompareBytes
                | Self::SplitExact
                | Self::Join
                | Self::FormatInt
                | Self::FormatFixed(_)
                | Self::ParseInt
                | Self::ParseFixed(_)
        )
    }

    /// Every runtime function, each fixed width included.
    #[must_use]
    pub fn all() -> Vec<Self> {
        let mut out = vec![
            Self::NatAdd,
            Self::NatSub,
            Self::NatMul,
            Self::NatQuot,
            Self::NatRem,
            Self::NatEq,
            Self::NatLe,
            Self::NatLt,
            Self::NatSucc,
            Self::IntAdd,
            Self::IntSub,
            Self::IntMul,
            Self::IntNeg,
            Self::IntQuot,
            Self::IntRem,
            Self::BoolNot,
            Self::BoolAnd,
            Self::BoolOr,
            Self::Equal,
            Self::Compare,
            Self::AppendList,
            Self::AppendBytes,
            Self::LengthList,
            Self::LengthBytes,
            Self::LengthString,
            Self::IndexList,
            Self::IndexBytes,
            Self::SliceList,
            Self::SliceBytes,
            Self::Utf8Encode,
            Self::Utf8Decode,
            Self::CompareBytes,
            Self::SplitExact,
            Self::Join,
            Self::FormatInt,
            Self::ParseInt,
        ];
        for kind in IntKind::ALL {
            out.extend([
                Self::CheckedAdd(kind),
                Self::CheckedSub(kind),
                Self::CheckedMul(kind),
                Self::CheckedQuot(kind),
                Self::BitAnd(kind),
                Self::BitOr(kind),
                Self::BitXor(kind),
                Self::BitNot(kind),
                Self::ShiftLeft(kind),
                Self::ShiftRight(kind),
                Self::Convert(kind),
                Self::FormatFixed(kind),
                Self::ParseFixed(kind),
            ]);
            if kind.signed() {
                out.push(Self::CheckedNeg(kind));
            }
        }
        out
    }
}

/// The names the runtime defines at the crate root, which no exported name
/// may take.
pub const ROOT_NAMES: &[&str] = &[
    "Ordering",
    "AtomicU64",
    "Memory",
    "Overflow",
    "R",
    "WORK",
    "tick",
    "work",
    "Same",
    "Key",
    "scalar",
    "fixed",
    "decimal",
    "Str",
    "Bytes",
    "Node",
    "List",
    "chars",
    "canonical_decimal",
];

#[cfg(test)]
mod tests {
    use super::{Item, CORE, STD};

    /// Every item exists in the runtime of every profile that can call it,
    /// with exactly the failure mode the item declares.
    #[test]
    fn every_item_exists_with_its_failure_mode() {
        let fixed_body = &CORE[CORE.find("macro_rules! fixed").expect("fixed")..];
        for item in Item::all() {
            let path = item.path();
            let text = if item.heap() { STD } else { CORE };
            if let Some((module, name)) = path.split_once("::") {
                assert!(CORE.contains(&format!("fixed!({module}, ")), "{path}");
                let declaration = fixed_body
                    .lines()
                    .find(|line| line.contains(&format!("pub fn {name}(")))
                    .unwrap_or_else(|| panic!("{path} is not declared"));
                assert!(!declaration.contains("-> R<") && !item.fallible(), "{path}");
            } else if let Some(line) = text.lines().find(|line| {
                line.starts_with("decimal!(")
                    && (line.contains(&format!(", {path},"))
                        || line.contains(&format!(", {path})")))
            }) {
                assert!(!item.fallible(), "{path}: {line}");
                let macro_body = &STD[STD.find("macro_rules! decimal").expect("decimal")..];
                let macro_body = &macro_body[..macro_body.find("\n}\n").expect("end")];
                assert!(!macro_body.contains("-> R<"), "{path}");
            } else {
                let declaration = text
                    .lines()
                    .find(|line| {
                        line.contains(&format!("pub fn {path}("))
                            || line.contains(&format!("pub fn {path}<"))
                    })
                    .unwrap_or_else(|| panic!("{path} is not declared"));
                assert_eq!(
                    declaration.contains("-> R<"),
                    item.fallible(),
                    "{path}: {declaration}"
                );
            }
        }
    }

    /// LexLean's compiler semantics records the SHA-256 of each profile's
    /// runtime, so a change to the runtime is a change to LexLean's
    /// identity: a runtime edited without its record fails here.
    #[test]
    fn the_compiler_semantics_records_the_runtime() {
        let (_, semantics) = crate::embedded::FILES
            .iter()
            .find(|(path, _)| *path == "language/semantics-1.2.toml")
            .expect("the language-1.2 semantics");
        let table: toml::Value = std::str::from_utf8(semantics)
            .expect("UTF-8")
            .parse()
            .expect("TOML");
        for (key, text) in [
            ("rust_runtime_core", CORE.to_owned()),
            ("rust_runtime_std", format!("{CORE}{STD}")),
        ] {
            let digest = crate::artifact::content_id::Sha256Digest::of(text.as_bytes()).to_hex();
            assert_eq!(
                table.get(key).and_then(toml::Value::as_str),
                Some(digest.as_str()),
                "the runtime changed without its record `{key}` in language/semantics-1.2.toml"
            );
        }
    }

    #[test]
    fn the_core_runtime_names_no_heap_type() {
        for heap in ["Vec", "String", "Box", "Rc", "std::", "alloc::", "format!"] {
            assert!(!CORE.contains(heap), "{heap}");
        }
    }

    /// Every loop of the runtime counts its iterations, except the release
    /// of a list, whose cells were counted when built.
    #[test]
    fn every_runtime_loop_ticks() {
        for runtime in [CORE, STD] {
            let lines: Vec<&str> = runtime.lines().collect();
            for (number, line) in lines.iter().enumerate() {
                let trimmed = line.trim_start();
                let looping = trimmed.starts_with("while ")
                    || trimmed.starts_with("for ")
                    || trimmed.starts_with("loop ")
                    || trimmed.contains("{ while ")
                    || trimmed.contains("{ for ");
                if looping && !line.ends_with("// release") {
                    let body = lines[number..(number + 3).min(lines.len())].join("\n");
                    assert!(body.contains("tick("), "an uncounted loop: {line}");
                }
            }
        }
    }
}
