//! The library boundary of the Rust runtime (SPEC.md §17.16): the machine
//! primitives, and every name the runtime text takes from Rust's standard
//! library with what the closed AST does with it.
//!
//! The runtime ([`super::rust::runtime`]) is handwritten text, so nothing but this
//! table stops it from reaching for one more library function, and each one
//! it reaches for is one more meaning the machine has to state. The table
//! closes that surface: [`audit`] reads the text and refuses a name the
//! table does not list, a row the text does not use, and a row stated over
//! anything but a primitive. The scan reads tokens, not Rust syntax, so it
//! sees names that are called, imported, or written as a path or a type;
//! the integer and Boolean operators are primitives of the machine as Rust
//! defines them and are not rows.
//!
//! The module sits beside `rust`, not in it: the renderer's files are what
//! the `rust_renderer` record of LexLean's semantics hashes, and this table
//! changes no rendering.

use std::collections::{BTreeMap, BTreeSet};

/// A machine primitive: an operation whose meaning `RustSemantics` states
/// and that no runtime source defines.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Primitive {
    /// The primitive's name, as a role cites it.
    pub name: &'static str,
    /// What it computes.
    pub meaning: &'static str,
}

/// How a name the runtime text uses is spelled.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Kind {
    /// A path a `use` declaration imports, with its alias if it has one.
    Import,
    /// A path other than an import, with an integer type spelled `<int>`.
    Path,
    /// A method called with `.name(`.
    Method,
    /// A macro invoked or defined.
    Macro,
    /// A capitalized name: a type, a trait, or a constructor.
    Name,
}

impl Kind {
    fn text(self) -> &'static str {
        match self {
            Self::Import => "import",
            Self::Path => "path",
            Self::Method => "method",
            Self::Macro => "macro",
            Self::Name => "name",
        }
    }
}

/// What the closed AST does with a name the runtime text uses.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    /// The name is (part of) the primitive named.
    Primitive(&'static str),
    /// The AST states what the name computes over the primitives named, or
    /// over its own constructs alone when none is named.
    Stated(&'static [&'static str]),
    /// A construct of the closed AST.
    Construct,
}

/// The machine primitives.
pub const PRIMITIVES: &[Primitive] = &[
    Primitive {
        name: "checked_add",
        meaning: "the sum of two integers of one width, or none when it is outside the width",
    },
    Primitive {
        name: "checked_sub",
        meaning: "the difference of two integers of one width, or none when it is outside the width",
    },
    Primitive {
        name: "checked_mul",
        meaning: "the product of two integers of one width, or none when it is outside the width",
    },
    Primitive {
        name: "checked_div",
        meaning: "the quotient rounded toward zero, or none for a zero divisor or a quotient outside the width",
    },
    Primitive {
        name: "checked_rem",
        meaning: "the remainder with the dividend's sign, or none for a zero divisor or a quotient outside the width",
    },
    Primitive {
        name: "checked_neg",
        meaning: "the negation of a signed integer, or none when it is outside the width",
    },
    Primitive {
        name: "checked_shl",
        meaning: "the bits shifted left by an amount, or none when the amount is at least the width",
    },
    Primitive {
        name: "checked_shr",
        meaning: "the bits shifted right (arithmetically when signed) by an amount, or none when the amount is at least the width",
    },
    Primitive {
        name: "convert",
        meaning: "an integer at another width: exact when it widens, none when the value is outside the target width",
    },
    Primitive {
        name: "order",
        meaning: "the order of two integers or two Booleans",
    },
    Primitive {
        name: "work",
        meaning: "the run's work counter: `tick` adds a count of units, `work` reads the total",
    },
    Primitive {
        name: "buffer_new",
        meaning: "an empty growable buffer with room for a count of items",
    },
    Primitive {
        name: "buffer_push",
        meaning: "a buffer its holder owns alone, one item longer at its end",
    },
    Primitive {
        name: "buffer_truncate",
        meaning: "a buffer its holder owns alone, cut to a length",
    },
    Primitive {
        name: "buffer_len",
        meaning: "the number of items of a buffer, a slice, or the bytes of a text",
    },
    Primitive {
        name: "share_copy",
        meaning: "a shared immutable buffer holding a copy of a slice or a text",
    },
    Primitive {
        name: "share_unique",
        meaning: "the value behind a shared holder, mutably, when the holder is its only one",
    },
    Primitive {
        name: "share_release",
        meaning: "the value behind a shared holder, by value, when the holder is its only one",
    },
    Primitive {
        name: "replace",
        meaning: "the value in a place, leaving another value in its stead",
    },
    Primitive {
        name: "utf8_view",
        meaning: "the UTF-8 bytes of a text",
    },
    Primitive {
        name: "utf8_check",
        meaning: "the text whose UTF-8 encoding is a byte slice, or none when it is not UTF-8",
    },
];

/// Every name the runtime text uses, with its role.
pub const LIBRARY: &[(Kind, &str, Role)] = &[
    (Kind::Import, "core::cmp::Ordering", Role::Construct),
    (
        Kind::Import,
        "core::sync::atomic::AtomicU64",
        Role::Primitive("work"),
    ),
    (
        Kind::Import,
        "core::sync::atomic::Ordering as Memory",
        Role::Primitive("work"),
    ),
    (
        Kind::Path,
        "<int>::BITS",
        Role::Stated(&["checked_shl", "checked_shr"]),
    ),
    (Kind::Path, "<int>::from", Role::Primitive("convert")),
    (Kind::Path, "<int>::try_from", Role::Primitive("convert")),
    (Kind::Path, "AtomicU64::new", Role::Primitive("work")),
    (Kind::Path, "Memory::Relaxed", Role::Primitive("work")),
    (Kind::Path, "Ordering::Equal", Role::Construct),
    (Kind::Path, "Ordering::Greater", Role::Construct),
    (Kind::Path, "Ordering::Less", Role::Construct),
    (Kind::Path, "String::new", Role::Stated(&["buffer_new"])),
    (Kind::Path, "Vec::new", Role::Stated(&["buffer_new"])),
    (
        Kind::Path,
        "Vec::with_capacity",
        Role::Primitive("buffer_new"),
    ),
    (
        Kind::Path,
        "core::str::from_utf8",
        Role::Primitive("utf8_check"),
    ),
    (Kind::Path, "std::mem::replace", Role::Primitive("replace")),
    (Kind::Path, "std::rc::Rc", Role::Construct),
    (
        Kind::Path,
        "std::rc::Rc::from",
        Role::Primitive("share_copy"),
    ),
    (
        Kind::Path,
        "std::rc::Rc::get_mut",
        Role::Primitive("share_unique"),
    ),
    (Kind::Path, "std::rc::Rc::new", Role::Construct),
    (
        Kind::Path,
        "std::rc::Rc::try_unwrap",
        Role::Primitive("share_release"),
    ),
    (Kind::Method, "and_then", Role::Stated(&[])),
    (Kind::Method, "as_bytes", Role::Primitive("utf8_view")),
    (
        Kind::Method,
        "chars",
        Role::Stated(&["utf8_view", "buffer_len"]),
    ),
    (Kind::Method, "checked_add", Role::Primitive("checked_add")),
    (Kind::Method, "checked_div", Role::Primitive("checked_div")),
    (Kind::Method, "checked_mul", Role::Primitive("checked_mul")),
    (Kind::Method, "checked_neg", Role::Primitive("checked_neg")),
    (Kind::Method, "checked_rem", Role::Primitive("checked_rem")),
    (Kind::Method, "checked_sub", Role::Primitive("checked_sub")),
    (Kind::Method, "clone", Role::Construct),
    (Kind::Method, "cmp", Role::Stated(&["order", "buffer_len"])),
    (
        Kind::Method,
        "collect",
        Role::Stated(&["buffer_new", "buffer_push"]),
    ),
    (Kind::Method, "copied", Role::Stated(&[])),
    (
        Kind::Method,
        "extend_from_slice",
        Role::Stated(&["buffer_push", "buffer_len"]),
    ),
    (Kind::Method, "fetch_add", Role::Primitive("work")),
    (Kind::Method, "get", Role::Stated(&["buffer_len"])),
    (Kind::Method, "into_iter", Role::Stated(&["buffer_len"])),
    (Kind::Method, "is_ascii_digit", Role::Stated(&[])),
    (Kind::Method, "is_empty", Role::Stated(&["buffer_len"])),
    (Kind::Method, "iter", Role::Stated(&["buffer_len"])),
    (Kind::Method, "len", Role::Primitive("buffer_len")),
    (Kind::Method, "load", Role::Primitive("work")),
    (Kind::Method, "map", Role::Stated(&[])),
    (Kind::Method, "map_err", Role::Stated(&[])),
    (Kind::Method, "min", Role::Stated(&[])),
    (Kind::Method, "next", Role::Stated(&["buffer_len"])),
    (Kind::Method, "ok", Role::Stated(&[])),
    (Kind::Method, "ok_or", Role::Stated(&[])),
    (
        Kind::Method,
        "parse",
        Role::Stated(&["checked_mul", "checked_add", "checked_neg"]),
    ),
    (Kind::Method, "push", Role::Primitive("buffer_push")),
    (
        Kind::Method,
        "push_str",
        Role::Stated(&["utf8_view", "buffer_push"]),
    ),
    (Kind::Method, "rev", Role::Stated(&["buffer_len"])),
    (
        Kind::Method,
        "saturating_sub",
        Role::Stated(&["checked_sub"]),
    ),
    (
        Kind::Method,
        "split",
        Role::Stated(&["utf8_view", "buffer_len", "share_copy"]),
    ),
    (
        Kind::Method,
        "starts_with",
        Role::Stated(&["utf8_view", "buffer_len"]),
    ),
    (
        Kind::Method,
        "strip_prefix",
        Role::Stated(&["utf8_view", "buffer_len"]),
    ),
    (
        Kind::Method,
        "to_string",
        Role::Stated(&["checked_div", "checked_rem"]),
    ),
    (Kind::Method, "truncate", Role::Primitive("buffer_truncate")),
    (Kind::Method, "unwrap_or", Role::Stated(&[])),
    (Kind::Method, "wrapping_rem", Role::Stated(&["checked_rem"])),
    (Kind::Method, "wrapping_shl", Role::Stated(&["checked_shl"])),
    (Kind::Method, "wrapping_shr", Role::Stated(&["checked_shr"])),
    (Kind::Macro, "macro_rules", Role::Stated(&[])),
    (Kind::Name, "Clone", Role::Construct),
    (Kind::Name, "Copy", Role::Construct),
    (Kind::Name, "Debug", Role::Construct),
    (Kind::Name, "Drop", Role::Construct),
    (Kind::Name, "Eq", Role::Construct),
    (Kind::Name, "Err", Role::Construct),
    (Kind::Name, "FnMut", Role::Stated(&[])),
    (Kind::Name, "None", Role::Construct),
    (Kind::Name, "Ok", Role::Construct),
    (Kind::Name, "Option", Role::Construct),
    (Kind::Name, "PartialEq", Role::Construct),
    (Kind::Name, "Result", Role::Construct),
    (Kind::Name, "Some", Role::Construct),
    (Kind::Name, "Vec", Role::Construct),
];

/// Checks the runtime of both profiles against [`LIBRARY`] and
/// [`PRIMITIVES`].
///
/// # Errors
///
/// The first discrepancy [`audit`] finds.
pub fn audit_runtime() -> Result<(), String> {
    audit(
        &format!(
            "{}{}",
            super::rust::runtime::CORE,
            super::rust::runtime::STD
        ),
        LIBRARY,
        PRIMITIVES,
    )
}

/// Checks a runtime text against a library table and a primitive set.
///
/// # Errors
///
/// A name the text uses that no row lists; a row listed twice or whose name
/// the text does not use; a role citing a name that is not a primitive; a
/// primitive listed twice or that no role cites.
pub fn audit(
    text: &str,
    library: &[(Kind, &str, Role)],
    primitives: &[Primitive],
) -> Result<(), String> {
    let mut known = BTreeSet::new();
    for primitive in primitives {
        if !known.insert(primitive.name) {
            return Err(format!(
                "the primitive `{}` is listed twice",
                primitive.name
            ));
        }
    }
    let mut rows = BTreeMap::new();
    let mut cited = BTreeSet::new();
    for (kind, name, role) in library {
        if rows.insert((*kind, *name), *role).is_some() {
            return Err(format!("the {} row `{name}` is listed twice", kind.text()));
        }
        let by: &[&str] = match role {
            Role::Primitive(primitive) => std::slice::from_ref(primitive),
            Role::Stated(primitives) => primitives,
            Role::Construct => &[],
        };
        for primitive in by {
            if !known.contains(primitive) {
                return Err(format!(
                    "the {} row `{name}` cites `{primitive}`, which is not a primitive",
                    kind.text()
                ));
            }
            cited.insert(*primitive);
        }
    }
    if let Some(primitive) = known.difference(&cited).next() {
        return Err(format!("the primitive `{primitive}` is cited by no row"));
    }
    let used = uses(text);
    if let Some((kind, name)) = used
        .iter()
        .find(|(kind, name)| !rows.contains_key(&(*kind, name.as_str())))
    {
        return Err(format!(
            "the runtime uses the {} `{name}`, which the library table does not list",
            kind.text()
        ));
    }
    if let Some((kind, name)) = rows
        .keys()
        .find(|(kind, name)| !used.contains(&(*kind, (*name).to_owned())))
    {
        return Err(format!(
            "the {} row `{name}` names nothing the runtime uses",
            kind.text()
        ));
    }
    Ok(())
}

// --- the scan ----------------------------------------------------------------

/// The text with comments removed and every string and character literal
/// emptied, so that a name inside a literal is not a use.
fn code(text: &str) -> String {
    let chars: Vec<char> = text.chars().collect();
    let mut out = String::new();
    let mut at = 0;
    while at < chars.len() {
        match chars[at] {
            '/' if chars.get(at + 1) == Some(&'/') => {
                while at < chars.len() && chars[at] != '\n' {
                    at += 1;
                }
            }
            '"' => {
                at += 1;
                while at < chars.len() && chars[at] != '"' {
                    at += if chars[at] == '\\' { 2 } else { 1 };
                }
                at += 1;
                out.push_str("\"\"");
            }
            '\'' if chars.get(at + 2) == Some(&'\'') => {
                at += 3;
                out.push_str("' '");
            }
            '\'' if chars.get(at + 1) == Some(&'\\') => {
                at += 2;
                while at < chars.len() && chars[at] != '\'' {
                    at += 1;
                }
                at += 1;
                out.push_str("' '");
            }
            other => {
                out.push(other);
                at += 1;
            }
        }
    }
    out
}

fn tokens(text: &str) -> Vec<String> {
    let chars: Vec<char> = code(text).chars().collect();
    let mut out = Vec::new();
    let mut at = 0;
    while at < chars.len() {
        let c = chars[at];
        if c.is_whitespace() {
            at += 1;
        } else if c == '$' || c == '_' || c.is_ascii_alphabetic() {
            let start = at;
            at += 1;
            while at < chars.len() && (chars[at] == '_' || chars[at].is_ascii_alphanumeric()) {
                at += 1;
            }
            out.push(chars[start..at].iter().collect());
        } else if c == ':' && chars.get(at + 1) == Some(&':') {
            out.push("::".to_owned());
            at += 2;
        } else {
            out.push(c.to_string());
            at += 1;
        }
    }
    out
}

fn is_name(token: &str) -> bool {
    token
        .chars()
        .next()
        .is_some_and(|c| c == '$' || c == '_' || c.is_ascii_alphabetic())
}

/// An integer type, or a macro's type variable standing for one.
fn integer(token: &str) -> bool {
    token.starts_with('$')
        || matches!(
            token,
            "u8" | "u16" | "u32" | "u64" | "u128" | "usize" | "i8" | "i16" | "i32" | "i64" | "i128"
        )
}

/// The index of the token that closes the bracket opened at `open`.
fn closing(tokens: &[String], open: usize) -> usize {
    let (left, right) = match tokens[open].as_str() {
        "<" => ("<", ">"),
        "(" => ("(", ")"),
        "[" => ("[", "]"),
        _ => ("{", "}"),
    };
    let mut depth = 0;
    for (at, token) in tokens.iter().enumerate().skip(open) {
        if token == left {
            depth += 1;
        } else if token == right {
            depth -= 1;
            if depth == 0 {
                return at;
            }
        }
    }
    tokens.len()
}

/// What the runtime binds itself.
struct Declared {
    /// Its items, the constructors of its enums, and the generic parameters
    /// of its items.
    names: BTreeSet<String>,
    /// The names its imports bind, whose paths are library paths.
    imported: BTreeSet<String>,
    /// The methods its inherent impls and the impls of its own traits
    /// declare: a call of one is not a library use, while a call of `clone`,
    /// whose trait is the library's, is.
    methods: BTreeSet<String>,
}

fn declarations(tokens: &[String]) -> Declared {
    let mut names = BTreeSet::new();
    let mut imported = BTreeSet::new();
    for (at, token) in tokens.iter().enumerate() {
        let next = tokens.get(at + 1).map(String::as_str);
        match token.as_str() {
            "fn" | "struct" | "enum" | "trait" | "type" | "static" | "mod" => {
                if let Some(name) = next {
                    names.insert(name.to_owned());
                }
                // Generic parameters follow the name.
                if tokens.get(at + 2).map(String::as_str) == Some("<") {
                    generics(tokens, at + 2, &mut names);
                }
                if token == "enum" {
                    let open = (at..tokens.len()).find(|&i| tokens[i] == "{");
                    if let Some(open) = open {
                        let close = closing(tokens, open);
                        let mut depth = 0;
                        for i in open..close {
                            match tokens[i].as_str() {
                                "{" | "(" | "<" => depth += 1,
                                "}" | ")" | ">" => depth -= 1,
                                name if depth == 1
                                    && is_name(name)
                                    && matches!(tokens[i - 1].as_str(), "{" | ",") =>
                                {
                                    names.insert(name.to_owned());
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
            "macro_rules" => {
                if let Some(name) = tokens.get(at + 2) {
                    names.insert(name.clone());
                }
            }
            "impl" if next == Some("<") => generics(tokens, at + 1, &mut names),
            "use" => {
                let end = (at..tokens.len())
                    .find(|&i| tokens[i] == ";")
                    .unwrap_or(tokens.len());
                for i in at + 1..end {
                    let bound = tokens.get(i + 1).map(String::as_str);
                    if is_name(&tokens[i]) && matches!(bound, Some(";" | "," | "}")) {
                        imported.insert(tokens[i].clone());
                    }
                }
            }
            _ => {}
        }
    }
    let mut methods = BTreeSet::new();
    for (at, token) in tokens.iter().enumerate() {
        if token != "impl" {
            continue;
        }
        let Some(open) = (at..tokens.len()).find(|&i| tokens[i] == "{") else {
            continue;
        };
        let header = &tokens[at..open];
        let own = header
            .iter()
            .position(|token| token == "for")
            .is_none_or(|position| names.contains(&header[position - 1]));
        if own {
            let close = closing(tokens, open);
            for i in open..close {
                if tokens[i] == "fn" {
                    methods.insert(tokens[i + 1].clone());
                }
            }
        }
    }
    Declared {
        names,
        imported,
        methods,
    }
}

fn generics(tokens: &[String], open: usize, names: &mut BTreeSet<String>) {
    let close = closing(tokens, open);
    for i in open + 1..close {
        if is_name(&tokens[i]) && matches!(tokens[i - 1].as_str(), "<" | ",") {
            names.insert(tokens[i].clone());
        }
    }
}

/// Every library name the text uses, by how it is spelled.
fn uses(text: &str) -> BTreeSet<(Kind, String)> {
    let tokens = tokens(text);
    let declared = declarations(&tokens);
    let mut out = BTreeSet::new();
    let mut at = 0;
    while at < tokens.len() {
        let token = tokens[at].as_str();
        let next = tokens.get(at + 1).map(String::as_str);
        let previous = at.checked_sub(1).map(|i| tokens[i].as_str());
        if token == "use" {
            let end = (at..tokens.len())
                .find(|&i| tokens[i] == ";")
                .unwrap_or(tokens.len());
            imports(&tokens[at + 1..end], &mut out);
            at = end + 1;
            continue;
        }
        if !is_name(token) {
            at += 1;
            continue;
        }
        if previous == Some(".") && matches!(next, Some("(" | "::")) {
            if !declared.methods.contains(token) {
                out.insert((Kind::Method, token.to_owned()));
            }
            // A turbofish is part of the call.
            at += 1;
            while tokens.get(at).map(String::as_str) == Some("::") {
                at = if tokens.get(at + 1).map(String::as_str) == Some("<") {
                    closing(&tokens, at + 1) + 1
                } else {
                    at + 2
                };
            }
            continue;
        }
        if next == Some("::") {
            let mut segments = vec![if integer(token) { "<int>" } else { token }.to_owned()];
            let mut end = at;
            while tokens.get(end + 1).map(String::as_str) == Some("::")
                && tokens.get(end + 2).is_some_and(|name| is_name(name))
            {
                segments.push(tokens[end + 2].clone());
                end += 2;
            }
            // A path through a name the runtime declares is the runtime's;
            // one through an imported name is the library's.
            if !declared.names.contains(token) {
                out.insert((Kind::Path, segments.join("::")));
            }
            at = end + 1;
            continue;
        }
        if next == Some("!")
            && (token == "macro_rules"
                || matches!(
                    tokens.get(at + 2).map(String::as_str),
                    Some("(" | "[" | "{")
                ))
        {
            if !declared.names.contains(token) {
                out.insert((Kind::Macro, token.to_owned()));
            }
            at += 1;
            continue;
        }
        if token.starts_with(|c: char| c.is_ascii_uppercase())
            && token != "Self"
            && !declared.names.contains(token)
            && !declared.imported.contains(token)
        {
            out.insert((Kind::Name, token.to_owned()));
        }
        at += 1;
    }
    out
}

/// The paths a `use` declaration imports, braces expanded and each alias
/// kept with its path.
fn imports(tokens: &[String], out: &mut BTreeSet<(Kind, String)>) {
    let mut prefix = Vec::new();
    let mut at = 0;
    while at < tokens.len() {
        match tokens[at].as_str() {
            "::" => at += 1,
            "{" => {
                let close = closing(tokens, at);
                for group in tokens[at + 1..close].split(|token| token == ",") {
                    let mut path = prefix.clone();
                    path.push(group.join(" "));
                    out.insert((Kind::Import, path.join("::")));
                }
                return;
            }
            name => {
                prefix.push(name.to_owned());
                at += 1;
            }
        }
    }
    out.insert((Kind::Import, prefix.join("::")));
}
