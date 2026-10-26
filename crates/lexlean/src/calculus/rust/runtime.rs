//! The fixed Rust runtime every rendering carries (SPEC.md §17.16): checked
//! arithmetic, scalar equality and key order, the work counter, and, in
//! `rust-std`, shared strings, byte strings, persistent lists, and the
//! primitives over them.
//!
//! The runtime is printed from four language-1.2 modules (`RuntimeSupport`,
//! `RustRuntime`, `RuntimeStd`, and `RustStd`) and held nowhere as a string;
//! the SHA-256 of the printed text is what every package's provenance records,
//! and generated code reaches it only through [`Item`], so no rendering can
//! name a runtime function that does not exist or call one with the wrong
//! failure mode.

use std::sync::OnceLock;

use super::super::IntKind;
use super::{ast, rt, rt_decode, rt_validate, source, validate};

/// The core runtime text of `items`: `support`, then each item printed.
/// An item whose exported name is a path `module::name` is printed inside
/// `pub mod module { … }` under its last segment, with the consecutive items
/// of one module in one block, so the module's path in the exported name is
/// the runtime item's path (`fixed_u8::checked_add`).
#[must_use]
pub fn assemble(support: &str, items: &[ast::ItemDef]) -> String {
    let mut text = support.to_owned();
    let mut open: Option<String> = None;
    for item in items {
        let (module, shown) = match item {
            ast::ItemDef::Function {
                name: ast::Ident::Export(name),
                ..
            } if name.contains("::") => {
                let (module, last) = name.rsplit_once("::").expect("a path");
                let mut shown = item.clone();
                if let ast::ItemDef::Function { name, .. } = &mut shown {
                    *name = ast::Ident::Export(last.to_owned());
                }
                (Some(module.to_owned()), shown)
            }
            _ => (None, item.clone()),
        };
        if module != open {
            if open.is_some() {
                text.push_str("}\n");
            }
            if let Some(module) = &module {
                text.push_str(&format!("\npub mod {module} {{"));
            }
            open = module;
        }
        text.push('\n');
        let printed = ast::print_item(&shown);
        if open.is_some() {
            for line in printed.lines() {
                text.push_str("    ");
                text.push_str(line);
                text.push('\n');
            }
        } else {
            text.push_str(&printed);
        }
    }
    if open.is_some() {
        text.push_str("}\n");
    }
    text
}

/// The text of a `RuntimeSupport` module: its crate decoded, checked by the
/// validator of `RuntimeSyntax`, and printed.
///
/// # Errors
///
/// Returns the reason the module cannot be decoded, or its crate is not valid.
pub fn support_from(module: &str) -> Result<String, String> {
    let krate = rt_decode::decode(module)?;
    rt_validate::validate(&krate)?;
    Ok(rt::print(&krate))
}

/// The core runtime text of a `RustRuntime` module over the support text
/// `support`: its functions decoded, the crate they make checked by the
/// validator that checks every rendering, and each printed after `support`.
///
/// The runtime crate is the one crate no lowering produces, so the validator
/// runs on it here: its functions fail only with `Err(Overflow)`, own what
/// they return, and allocate nothing.
///
/// # Errors
///
/// Returns the reason the module cannot be decoded, or its crate is not valid.
pub fn core_over(support: &str, module: &str) -> Result<String, String> {
    let source = source::decode(module)?;
    validate::validate(&source.core)
        .map_err(|error| format!("the core runtime is not a valid crate: {error}"))?;
    Ok(assemble(support, &source.core.items))
}

/// The core runtime text of a `RustRuntime` module over the embedded support.
///
/// # Errors
///
/// Returns the reason either module cannot be read, or its crate is not valid.
pub fn core_from(module: &str) -> Result<String, String> {
    core_over(support()?, module)
}

/// The support items both profiles carry: [`support_from`] the embedded module.
///
/// # Errors
///
/// Returns the reason the module cannot be read, or its crate is not valid.
pub fn support() -> Result<&'static str, String> {
    static SUPPORT: OnceLock<Result<String, String>> = OnceLock::new();
    SUPPORT
        .get_or_init(|| support_from(rt_decode::embedded()?))
        .as_ref()
        .map(String::as_str)
        .map_err(Clone::clone)
}

/// The runtime both profiles carry: [`core_from`] the embedded module.
///
/// # Errors
///
/// Returns the reason the module cannot be read, or its crate is not valid.
pub fn core() -> Result<&'static str, String> {
    static CORE: OnceLock<Result<String, String>> = OnceLock::new();
    CORE.get_or_init(|| core_from(source::embedded()?))
        .as_ref()
        .map(String::as_str)
        .map_err(Clone::clone)
}

/// The types, impls, helpers, and number text of the std runtime: the crate of a
/// `RuntimeStd` module decoded, checked by the validator of `RuntimeSyntax`
/// together with the support items it stands on, and printed. The support
/// items are checked with it and not printed: the core runtime already
/// carries them.
///
/// # Errors
///
/// Returns the reason either module cannot be decoded, the module is not of
/// the std profile, or the crate is not valid.
pub fn std_types_from(module: &str) -> Result<String, String> {
    let mut support = rt_decode::decode(rt_decode::embedded()?)?;
    let krate = rt_decode::decode_std(module)?;
    if krate.profile != rt::Profile::Std {
        return Err("the crate of RuntimeStd is not a std crate".to_owned());
    }
    support.profile = rt::Profile::Std;
    support.items.extend(krate.items.iter().cloned());
    rt_validate::validate(&support)?;
    Ok(rt::print(&krate))
}

/// The base the functions of `RustStd` are printed after: [`std_types_from`]
/// the embedded module.
///
/// # Errors
///
/// Returns the reason the module cannot be read, or its crate is not valid.
pub fn std_base() -> Result<&'static str, String> {
    static BASE: OnceLock<Result<String, String>> = OnceLock::new();
    BASE.get_or_init(|| std_types_from(rt_decode::embedded_std()?))
        .as_ref()
        .map(String::as_str)
        .map_err(Clone::clone)
}

/// The std runtime text of a `RustStd` module over `base`: its functions
/// decoded, the crate they make checked by the validator that checks every
/// rendering, and each printed after `base`.
///
/// # Errors
///
/// Returns the reason the module cannot be decoded, or its crate is not valid.
pub fn std_over(base: &str, module: &str) -> Result<String, String> {
    let krate = source::decode_std(module)?;
    validate::validate(&krate)
        .map_err(|error| format!("the std runtime is not a valid crate: {error}"))?;
    Ok(assemble(base, &krate.items))
}

/// The std runtime text of a `RustStd` module over [`std_base`].
///
/// # Errors
///
/// Returns the reason the module cannot be read, or its crate is not valid.
pub fn std_from(module: &str) -> Result<String, String> {
    std_over(std_base()?, module)
}

/// The runtime only `rust-std` carries: [`std_base`] and the functions of the
/// embedded `RustStd` module. Every loop ticks the work counter once per
/// iteration and a bulk copy ticks its length, except the release of a list,
/// whose cells were each counted when built.
///
/// # Errors
///
/// Returns the reason the module cannot be read, or its crate is not valid.
pub fn std() -> Result<&'static str, String> {
    static STD: OnceLock<Result<String, String>> = OnceLock::new();
    STD.get_or_init(|| std_from(source::embedded_std()?))
        .as_ref()
        .map(String::as_str)
        .map_err(Clone::clone)
}

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
    use super::{core, std, Item};

    /// Every item exists in the runtime of every profile that can call it,
    /// with exactly the failure mode the item declares.
    #[test]
    fn every_item_exists_with_its_failure_mode() {
        let core = core().expect("the core runtime");
        let std = std().expect("the std runtime");
        for item in Item::all() {
            let path = item.path();
            let text = if item.heap() { std } else { core };
            if let Some((module, name)) = path.split_once("::") {
                let at = core
                    .find(&format!("pub mod {module} {{"))
                    .unwrap_or_else(|| panic!("{path}: the module is not declared"));
                let body = &core[at..];
                let body = &body[..body.find("\n}\n").expect("the module ends")];
                let declaration = body
                    .lines()
                    .find(|line| line.contains(&format!("pub fn {name}(")))
                    .unwrap_or_else(|| panic!("{path} is not declared"));
                assert!(!declaration.contains("-> R<") && !item.fallible(), "{path}");
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
            (
                "rust_runtime_core",
                core().expect("the core runtime").to_owned(),
            ),
            (
                "rust_runtime_std",
                format!(
                    "{}{}",
                    core().expect("the core runtime"),
                    std().expect("the std runtime")
                ),
            ),
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
            assert!(!core().expect("the core runtime").contains(heap), "{heap}");
        }
    }

    /// Every loop of the runtime counts its iterations, except the release
    /// of a list, whose cells were counted when built.
    #[test]
    fn every_runtime_loop_ticks() {
        for runtime in [
            core().expect("the core runtime"),
            std().expect("the std runtime"),
        ] {
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
