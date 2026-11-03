//! `BS-22` (SPEC.md §17.23, §17.21): the lexing of the authoring notation as
//! language-1.2 source, `LexLeanBoot.SurfaceLex`, held to the byte to
//! `lexlean::surface::lex::{lex, quote}`.
//!
//! `lexText` reads a text and returns its tokens with their byte ranges, or the
//! offset and message of the first lexical error; `quoteText` spells a text as
//! the notation quotes it. Neither recurses: a scan is a loop under
//! `iterate_until`, and a string is a loop over its pieces, joined once at the
//! end.
//!
//! Each planted defect names the one vector that catches it. `BS-05` and
//! `BS-11` each lost a plant when their corpus grew, because a plant caught by
//! "some run of the corpus" is only as good as that corpus.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::OnceLock;

use lexlean::calculus::Value;
use lexlean::surface::lex::{lex, quote, Spanned, Token};
use lexlean::surface::SurfaceError;
use serde_json::Value as Json;

use super::rust_validate::{eligibility_of, rustc_differential};
use super::{list, nat_val, text, Run, Seeded};
use crate::production_export as harness;
use crate::support::{self, repo_root};

const MODULE: &str = "LexLeanBoot.SurfaceLex";
const SOURCE: &str = "bootstrap/src/SurfaceLex.lex.tex";
const ROOTS: [(&str, &[&str]); 2] = [
    ("lexText", &["allocation", "overflow"]),
    ("quoteText", &["allocation", "overflow"]),
];

/// The most a part of a certificate of a root may be, as the row states.
const CERTIFICATE_BUDGET: u64 = 3_145_728;
/// The bytes of each committed source the evaluator reads, and rustc reads: a
/// window from a place the path fixes. The harness writes a text as an array
/// of its bytes, so the total under rustc is held to about a megabyte.
const EVALUATOR_WINDOW: usize = 192;
const RUSTC_WINDOW: usize = 1_024;
/// Seeded edits under the evaluator, and under rustc.
const EVALUATOR_MUTANTS: usize = 600;
const RUSTC_MUTANTS: usize = 2_400;

// ---------------------------------------------------------------- the sorts

/// The constructors of each inductive of the module, in declaration order,
/// read from its text, so that the encoders name a constructor and never
/// number it, and one moved in the module fails here.
fn sorts() -> &'static BTreeMap<String, Vec<String>> {
    static SORTS: OnceLock<BTreeMap<String, Vec<String>>> = OnceLock::new();
    SORTS.get_or_init(|| {
        let source =
            std::fs::read_to_string(repo_root().join(SOURCE).as_std_path()).expect("the module");
        let mut out = BTreeMap::new();
        for block in source.split("inductive{").skip(1) {
            let block = block.split("\n}\n").next().expect("a block");
            let name = block
                .split("name: ")
                .nth(1)
                .and_then(|rest| rest.split(',').next())
                .expect("a sort name")
                .trim()
                .to_owned();
            let constructors: Vec<String> = block
                .split("{name: ")
                .skip(1)
                .map(|rest| {
                    rest.split(',')
                        .next()
                        .expect("a constructor")
                        .trim()
                        .to_owned()
                })
                .collect();
            out.insert(name, constructors);
        }
        out
    })
}

fn adt(sort: &str, constructor: &str, fields: Vec<Value>) -> Value {
    let found = sorts()
        .get(sort)
        .unwrap_or_else(|| panic!("the module has no sort `{sort}`"))
        .iter()
        .position(|name| name == constructor)
        .unwrap_or_else(|| panic!("`{sort}` has no constructor `{constructor}`"));
    Value::Adt {
        constructor: u64::try_from(found).expect("an index"),
        fields,
    }
}

/// A record: a structure is the one constructor of its own sort.
fn record(fields: Vec<Value>) -> Value {
    Value::Adt {
        constructor: 0,
        fields,
    }
}

// ------------------------------------------------------------- the encoders

fn enc_token(token: &Token) -> Value {
    let plain = |name: &str| adt("Token", name, Vec::new());
    match token {
        Token::LBrace => plain("LBrace"),
        Token::RBrace => plain("RBrace"),
        Token::LBrack => plain("LBrack"),
        Token::RBrack => plain("RBrack"),
        Token::LParen => plain("LParen"),
        Token::RParen => plain("RParen"),
        Token::Comma => plain("Comma"),
        Token::Colon => plain("Colon"),
        Token::ColonEq => plain("ColonEq"),
        Token::Scope => plain("Scope"),
        Token::Semi => plain("Semi"),
        Token::Str(value) => adt("Token", "StrToken", vec![text(value)]),
        Token::Word(value) => adt("Token", "WordToken", vec![text(value)]),
    }
}

fn enc_spanned(spanned: &Spanned) -> Value {
    record(vec![
        enc_token(&spanned.token),
        nat_val(spanned.start),
        nat_val(spanned.end),
    ])
}

fn enc_failure(failure: &SurfaceError) -> Value {
    record(vec![nat_val(failure.offset), text(&failure.message)])
}

/// The run of `lexText` on a text, and the oracle's message when it refuses.
fn lex_run(source: &str) -> (Run, Option<String>) {
    let (value, refusal) = match lex(source) {
        Ok(tokens) => (
            Value::Ok {
                value: Box::new(list(tokens.iter().map(enc_spanned).collect())),
            },
            None,
        ),
        Err(failure) => (
            Value::Error {
                value: Box::new(enc_failure(&failure)),
            },
            Some(failure.message),
        ),
    };
    (
        (
            vec![text(source)],
            serde_json::to_value(value).expect("a value is JSON"),
        ),
        refusal,
    )
}

/// The run of `quoteText` on a text.
fn quote_run(source: &str) -> Run {
    (
        vec![text(source)],
        serde_json::to_value(text(&quote(source))).expect("a value is JSON"),
    )
}

// ------------------------------------------------------------------ vectors

/// One named vector: a text, and the root it is a run of.
struct Vector {
    label: String,
    quoting: bool,
    source: String,
}

impl Vector {
    fn run(&self) -> (Run, Option<String>) {
        if self.quoting {
            (quote_run(&self.source), None)
        } else {
            lex_run(&self.source)
        }
    }
}

/// The scalars `char::is_whitespace` accepts that are not ASCII, one of each
/// width: the oracle splits a word at each.
const WHITE: &[&str] = &[
    "\u{b}", "\u{c}", "\r", "\u{85}", "\u{a0}", "\u{1680}", "\u{2000}", "\u{2003}", "\u{200a}",
    "\u{2028}", "\u{2029}", "\u{202f}", "\u{205f}", "\u{3000}",
];
/// Scalars next to white space that are none.
const LOOKALIKES: &[&str] = &[
    "\u{200b}",
    "\u{180e}",
    "\u{feff}",
    "\u{2060}",
    "\u{1c}",
    "\u{1f}",
    "\u{a1}",
    "\u{1fff}",
    "\u{2010}",
    "\u{2029}\u{2029}",
];

#[allow(clippy::too_many_lines)]
fn named() -> Vec<Vector> {
    let mut out: Vec<Vector> = Vec::new();
    let mut lexed = |label: &str, source: &str| {
        out.push(Vector {
            label: label.to_owned(),
            quoting: false,
            source: source.to_owned(),
        });
    };

    // Tokens and words.
    lexed("empty", "");
    lexed("white space only", " \n\t \u{a0} ");
    lexed(
        "the oracle's own text",
        "add{left: x, right: 1} := (a -> b);",
    );
    for (label, source) in [
        ("an opening brace", "{"),
        ("a closing brace", "}"),
        ("an opening bracket", "["),
        ("a closing bracket", "]"),
        ("an opening parenthesis", "("),
        ("a closing parenthesis", ")"),
        ("a comma", ","),
        ("a semicolon", ";"),
        ("a colon", ":"),
        ("a colon equals", ":="),
        ("a double colon", "::"),
    ] {
        lexed(label, source);
    }
    for (label, source) in [
        ("a colon and a colon equals", "::="),
        ("a colon equals and a colon", ":=:"),
        ("three colons", ":::"),
        ("two colon equals", ":=:="),
        ("a colon before the end", "a:"),
        ("a colon after a word", "a: b"),
        ("a colon equals inside a word", "a:=b"),
        ("a double colon inside a word", "a::b"),
        ("a colon equals at the end", "x :="),
        ("every delimiter", "{}[](),:;"),
        ("every delimiter and white space", "{ } [ ] ( ) , : ; :: :="),
        ("a word", "word"),
        ("a word of symbols", "->=>+-*/<>=!?#%&|^~@$"),
        (
            "a word with a dot, a quote mark, and a hyphen",
            "a.b'c-d_e/f",
        ),
        ("a number", "1234567890"),
        ("a negative number", "-12"),
        ("a word ended by a comma", "a,b"),
        ("a word ended by a semicolon", "a;b"),
        ("a word ended by a colon", "a:b"),
        ("a word ended by a brace", "a{b}c"),
        ("a word ended by a bracket", "a[b]c"),
        ("a word ended by a parenthesis", "a(b)c"),
        ("a word ended by a quote mark", "a\"b\"c"),
        ("a tab between words", "a\tb"),
        ("a line feed between words", "a\nb"),
        ("a carriage return between words", "a\rb"),
        ("a form feed between words", "a\u{c}b"),
        ("a vertical tab between words", "a\u{b}b"),
        ("a no-break space between words", "a\u{a0}b"),
        ("a next line between words", "a\u{85}b"),
        ("an ogham space mark between words", "a\u{1680}b"),
        ("an en quad between words", "a\u{2000}b"),
        ("a hair space between words", "a\u{200a}b"),
        ("a line separator between words", "a\u{2028}b"),
        ("a paragraph separator between words", "a\u{2029}b"),
        ("a narrow no-break space between words", "a\u{202f}b"),
        ("a medium mathematical space between words", "a\u{205f}b"),
        ("an ideographic space between words", "a\u{3000}b"),
        (
            "a word of scalars beyond ASCII",
            "\u{e9}\u{2115}\u{65e5}\u{1f600}",
        ),
        (
            "a word of a scalar before white space",
            "\u{e9}\u{a0}\u{e9}",
        ),
        (
            "scalars beyond ASCII among delimiters",
            "\u{2115}:\u{1f600},\u{3b1}",
        ),
        ("a control character in a word", "a\u{0}b\u{1}c\u{7f}d"),
        ("a nul alone", "\u{0}"),
    ] {
        lexed(label, source);
    }
    for (index, white) in WHITE.iter().enumerate() {
        lexed(
            &format!("white space {index} around words"),
            &format!("{white}a{white}{white}b{white}"),
        );
    }
    for (index, other) in LOOKALIKES.iter().enumerate() {
        lexed(
            &format!("lookalike {index} among words"),
            &format!("a{other}b {other} {other}c"),
        );
    }

    // Strings.
    for (label, source) in [
        ("an empty string", "\"\""),
        ("a string", "\"abc\""),
        ("a string with a space", "\"a b\""),
        ("a string after tokens", "{ a: \"x\" }"),
        ("a string beyond ASCII", "\"\u{e9}\u{2115}\u{1f600}\""),
        ("a string with a delete", "\"a\u{7f}b\""),
        ("a string with a brace and a colon equals", "\"{:=}\""),
        ("two strings", "\"a\"\"b\""),
        ("a string between words", "a\"b\"c"),
        (
            "a string with a raw high scalar after an escape",
            "\"\\n\u{e9}\"",
        ),
        ("a string with an escape after raw text", "\"ab\\ncd\""),
        ("an escaped quote mark", "\"a\\\"b\""),
        ("an escaped backslash", "\"a\\\\b\""),
        ("an escaped solidus", "\"a\\/b\""),
        ("an escaped backspace", "\"\\b\""),
        ("an escaped form feed", "\"\\f\""),
        ("an escaped line feed", "\"\\n\""),
        ("an escaped carriage return", "\"\\r\""),
        ("an escaped tab", "\"\\t\""),
        ("every escape", "\"\\\"\\\\\\/\\b\\f\\n\\r\\t\""),
        ("an escaped backslash before a quote mark", "\"a\\\\\" b"),
        ("an escape at the end of a string", "\"\\n\""),
        ("a unicode escape", "\"\\u0041\""),
        ("a unicode escape of a lower-case digit", "\"\\u00e9\""),
        ("a unicode escape of an upper-case digit", "\"\\u00E9\""),
        ("a unicode escape of a mixed-case digit", "\"\\uAbCd\""),
        ("a unicode escape of nul", "\"\\u0000\""),
        ("a unicode escape of the delete", "\"\\u007f\""),
        ("a unicode escape of the last ASCII scalar", "\"\\u007e\""),
        (
            "a unicode escape of the first two-byte scalar",
            "\"\\u0080\"",
        ),
        (
            "a unicode escape of the last two-byte scalar",
            "\"\\u07ff\"",
        ),
        (
            "a unicode escape of the first three-byte scalar",
            "\"\\u0800\"",
        ),
        (
            "a unicode escape of the last scalar before the surrogates",
            "\"\\ud7ff\"",
        ),
        (
            "a unicode escape of the first scalar after the surrogates",
            "\"\\ue000\"",
        ),
        ("a unicode escape of the last scalar", "\"\\uffff\""),
        ("a unicode escape of a letter", "\"\\u2115\""),
        ("a unicode escape of a number", "\"\\u1234\""),
        (
            "a unicode escape of each place",
            "\"\\u1000\\u0100\\u0010\\u0001\"",
        ),
        ("a unicode escape among raw text", "\"a\\u00e9b\\u2115c\""),
        ("a unicode escape of a quote mark", "\"\\u0022\""),
        ("two unicode escapes", "\"\\u0041\\u0042\""),
    ] {
        lexed(label, source);
    }

    // Lexical errors.
    for (label, source) in [
        ("an unterminated string", "\"abc"),
        ("an unterminated string after tokens", "{ a: b } \"abc"),
        ("an unterminated string after a word", "word \"abc"),
        ("an unterminated string at the end of the text", "\""),
        (
            "an unterminated string after white space beyond ASCII",
            "\u{a0}\u{3000}\"abc",
        ),
        (
            "an unterminated string after scalars beyond ASCII",
            "\u{e9}\u{2115} \"abc",
        ),
        ("a string ending with a backslash", "\"abc\\"),
        ("a backslash alone in a string", "\"\\"),
        ("an escape at the end of the text", "\"a\\n"),
        ("a quote mark escaped at the end", "\"a\\\""),
        ("an escape that is not one", "\"\\q\""),
        ("an escape of a digit", "\"\\1\""),
        ("an escape of a space", "\"\\ \""),
        ("an escape of a quote mark and a letter", "\"\\\"\\x\""),
        ("an escape of a scalar beyond ASCII", "\"\\\u{e9}\""),
        ("an escape of a three-byte scalar", "\"\\\u{2115}\""),
        ("an escape of a four-byte scalar", "\"\\\u{1f600}\""),
        ("an escape of a line feed", "\"\\\n\""),
        ("an escape of a nul", "\"\\\u{0}\""),
        ("an escape of an upper-case letter", "\"\\N\""),
        ("an escape after tokens", "a b \"\\q\""),
        (
            "an escape after scalars beyond ASCII",
            "\u{e9}\u{2115} \"\\q\"",
        ),
        ("a unicode escape of no digit", "\"\\u\""),
        ("a unicode escape of one digit", "\"\\u1\""),
        ("a unicode escape of two digits", "\"\\u12\""),
        ("a unicode escape of three digits", "\"\\u123\""),
        ("a unicode escape of three digits and the end", "\"\\u123"),
        (
            "a unicode escape of a letter that is no digit",
            "\"\\u12g4\"",
        ),
        ("a unicode escape of a letter first", "\"\\ug234\""),
        ("a unicode escape of a letter last", "\"\\u123g\""),
        ("a unicode escape of a sign", "\"\\u+123\""),
        ("a unicode escape of a hyphen", "\"\\u-123\""),
        (
            "a unicode escape of a scalar beyond ASCII",
            "\"\\u12\u{e9}4\"",
        ),
        ("a unicode escape cut short by a quote mark", "\"\\u12\"4"),
        (
            "a unicode escape of the digits just short of the first lower-case letter",
            "\"\\u000`\"",
        ),
        (
            "a unicode escape of the digits just past the last lower-case letter",
            "\"\\u000g\"",
        ),
        (
            "a unicode escape of the digits just short of the first upper-case letter",
            "\"\\u000@\"",
        ),
        (
            "a unicode escape of the digits just past the last upper-case letter",
            "\"\\u000G\"",
        ),
        (
            "a unicode escape of the character just past nine",
            "\"\\u000:\"",
        ),
        (
            "a unicode escape of the character just short of zero",
            "\"\\u000/\"",
        ),
        ("a first surrogate", "\"\\ud800\""),
        ("a last high surrogate", "\"\\udbff\""),
        ("a first low surrogate", "\"\\udc00\""),
        ("a last surrogate", "\"\\udfff\""),
        ("a surrogate pair", "\"\\ud83d\\ude00\""),
        ("a surrogate in upper case", "\"\\uD800\""),
        ("a surrogate after tokens", "a: \"\\ud800\""),
        ("a surrogate after text", "\"ab\\ud800\""),
        ("a nul in a string", "\"a\u{0}b\""),
        ("a start of heading in a string", "\"\u{1}\""),
        ("a line feed in a string", "\"a\nb\""),
        ("a tab in a string", "\"a\tb\""),
        ("a carriage return in a string", "\"a\rb\""),
        ("a unit separator in a string", "\"\u{1f}\""),
        ("a control character after tokens", "a: \"b\u{1}\""),
        (
            "a control character after scalars beyond ASCII",
            "\u{e9}\u{2115} \"\u{e9}\u{7}\"",
        ),
        ("a control character after an escape", "\"\\n\u{1}\""),
        ("an error after a string", "\"ok\" \"\\q\""),
        (
            "an error after a first error is not read",
            "\"\\q\" \"\\u12\" \"abc",
        ),
        ("an unterminated string after strings", "\"a\" \"b\" \"c"),
    ] {
        lexed(label, source);
    }

    // The quoting of a text.
    let mut quoted = |label: &str, source: &str| {
        out.push(Vector {
            label: label.to_owned(),
            quoting: true,
            source: source.to_owned(),
        });
    };
    quoted("quote the empty text", "");
    quoted("quote a text", "plain");
    quoted("quote a space", " ");
    quoted("quote a quote mark", "a\"b");
    quoted("quote a backslash", "a\\b");
    quoted("quote a backslash and a quote mark", "\\\"");
    quoted("quote a backspace", "\u{8}");
    quoted("quote a form feed", "\u{c}");
    quoted("quote a line feed", "a\nb");
    quoted("quote a carriage return", "\r");
    quoted("quote a tab", "\t");
    quoted("quote the delete", "a\u{7f}b");
    quoted("quote the first scalar not escaped", " ");
    quoted("quote scalars beyond ASCII", "\u{e9}\u{2115}\u{1f600}");
    quoted(
        "quote scalars beyond ASCII among escapes",
        "\u{e9}\n\u{2115}\"\u{1f600}",
    );
    quoted(
        "quote a long text of nothing to escape",
        &"abcdefgh".repeat(200),
    );
    quoted("quote a long text of escapes", &"\n\"\\".repeat(100));
    quoted(
        "quote a text of every control character",
        &(0..32u8).map(char::from).collect::<String>(),
    );
    for value in 0..32u8 {
        quoted(
            &format!("quote U+{value:04X}"),
            &format!("a{}b", char::from(value)),
        );
    }
    quoted(
        "quote a text the notation reads back",
        "add{left: x} := \"y\"\n",
    );
    out
}

/// Every error the lexer can state, by a part of its message, so that a corpus
/// that never reached one fails here and not silently (R2).
const REFUSALS: &[&str] = &[
    "unterminated string",
    "a `\\u` escape takes four hexadecimal digits",
    "is not a Unicode scalar value",
    "is not an escape",
    "a control character must be escaped in a string",
];

// ------------------------------------------------------------------- seeded

/// A whole token or a part of one, of each shape the notation reads.
const PIECES: &[&str] = &[
    "{",
    "}",
    "[",
    "]",
    "(",
    ")",
    ",",
    ":",
    ":=",
    "::",
    ";",
    " ",
    "  ",
    "\n",
    "\t",
    "\r",
    "\u{a0}",
    "\u{2003}",
    "\u{3000}",
    "\u{85}",
    "\u{200b}",
    "word",
    "a-b",
    "x1",
    "->",
    "=>",
    "1",
    "-1",
    "\"",
    "\"\"",
    "\"abc\"",
    "\"a b\"",
    "\"\\n\"",
    "\"\\\"\"",
    "\"\\\\\"",
    "\"\\/\"",
    "\"\\u00e9\"",
    "\"\\u00E9\"",
    "\"\\ud800\"",
    "\"\\udfff\"",
    "\"\\ue000\"",
    "\"\\q\"",
    "\"\\u12\"",
    "\"\\u12g4\"",
    "\\",
    "\\u",
    "\\n",
    "\\u00",
    "\u{e9}",
    "\u{2115}",
    "\u{1f600}",
    "\u{0}",
    "\u{1}",
    "\u{1f}",
    "\u{7f}",
    "\"\u{1}\"",
    "\"\n\"",
    "\"\u{e9}\"",
];

/// One seeded edit of a text, at a scalar boundary: a deletion, an insertion,
/// or a replacement, up to four of them.
fn edited(seeded: &mut Seeded, source: &str) -> String {
    let mut scalars: Vec<char> = source.chars().collect();
    for _ in 0..=seeded.below(4) {
        let at = seeded.below(scalars.len() + 1);
        let piece: Vec<char> = seeded.pick(PIECES).chars().collect();
        match seeded.below(3) {
            0 if at < scalars.len() => {
                scalars.remove(at);
            }
            1 => {
                drop(scalars.splice(at..at, piece));
            }
            _ if at < scalars.len() => {
                drop(scalars.splice(at..=at, piece));
            }
            _ => scalars.extend(piece),
        }
    }
    scalars.into_iter().collect()
}

/// A text of pieces, one edit away from nothing: an edge text.
fn edge_text(seeded: &mut Seeded) -> String {
    let mut out = String::new();
    for _ in 0..=seeded.below(12) {
        out.push_str(seeded.pick(PIECES));
        if seeded.chance(40) {
            out.push_str(seeded.pick(&[" ", "\n", "  "]));
        }
    }
    out
}

/// The seeded corpus: `count` edits of the texts of the vectors, of windows of
/// the committed sources, and edge texts, as runs of `lexText`, and the same
/// number of runs of `quoteText` on them.
fn mutants(count: usize, bases: &[String]) -> (Vec<Run>, Vec<Run>) {
    let mut seeded = Seeded(0x0B55_0022);
    let mut lexing = Vec::with_capacity(count);
    let mut quoting = Vec::with_capacity(count);
    while lexing.len() < count {
        let source = if seeded.chance(25) {
            edge_text(&mut seeded)
        } else {
            let base = seeded.pick(bases).clone();
            edited(&mut seeded, &base)
        };
        lexing.push(lex_run(&source).0);
        if seeded.chance(40) {
            quoting.push(quote_run(&source));
        }
    }
    (lexing, quoting)
}

/// A hash of a path (FNV-1a), which fixes where a window lies.
fn path_hash(path: &str) -> u64 {
    path.bytes().fold(0xcbf2_9ce4_8422_2325_u64, |hash, byte| {
        (hash ^ u64::from(byte)).wrapping_mul(0x0100_0000_01b3)
    })
}

/// A window of `bytes` bytes of a text, from a place its path fixes, at
/// scalar boundaries.
fn window(path: &str, source: &str, bytes: usize) -> String {
    let mut start = if source.len() > bytes {
        usize::try_from(path_hash(path) % u64::try_from(source.len() - bytes).expect("a length"))
            .expect("an offset")
    } else {
        0
    };
    while !source.is_char_boundary(start) {
        start -= 1;
    }
    let mut end = (start + bytes).min(source.len());
    while !source.is_char_boundary(end) {
        end -= 1;
    }
    source[start..end].to_owned()
}

// ------------------------------------------------------------------- plants

/// A defect planted in the source, and the named vector that catches it.
struct Plant {
    label: &'static str,
    root: usize,
    site: &'static str,
    replacement: &'static str,
    vector: &'static str,
}

const PLANTS: &[Plant] = &[
    Plant {
        label: "a word ends only at an ASCII space",
        root: 0,
        site: "(if (0 <? (AxiomOutput::whiteLength subject position)) then true else (isDelimiterByte subject position))",
        replacement: "(if (byteIs subject position (32 : UInt8)) then true else (isDelimiterByte subject position))",
        vector: "a tab between words",
    },
    Plant {
        label: "white space is skipped by one byte",
        root: 0,
        site: "(record Lx {pos := (position + white),",
        replacement: "(record Lx {pos := (position + 1),",
        vector: "a no-break space between words",
    },
    Plant {
        label: "a colon equals is a colon",
        root: 0,
        site: "(pushed state (ctor Token.ColonEq) (position + 2))",
        replacement: "(pushed state (ctor Token.Colon) (position + 1))",
        vector: "a colon equals",
    },
    Plant {
        label: "a double colon is a colon",
        root: 0,
        site: "(pushed state (ctor Token.Scope) (position + 2))",
        replacement: "(pushed state (ctor Token.Colon) (position + 1))",
        vector: "a double colon",
    },
    Plant {
        label: "a semicolon is no token",
        root: 0,
        site: "(if (byteIs subject position (59 : UInt8)) then (pushed state (ctor Token.Semi) (position + 1))",
        replacement: "(if false then (pushed state (ctor Token.Semi) (position + 1))",
        vector: "a semicolon",
    },
    Plant {
        label: "a comma does not end a word",
        root: 0,
        site: "(if (byteIs subject position (44 : UInt8)) then true else",
        replacement: "(if false then true else",
        vector: "a word ended by a comma",
    },
    Plant {
        label: "a colon does not end a word",
        root: 0,
        site: "(if (byteIs subject position (58 : UInt8)) then true else",
        replacement: "(if false then true else",
        vector: "a word ended by a colon",
    },
    Plant {
        label: "a quote mark does not end a word",
        root: 0,
        site: "(byteIs subject position (34 : UInt8)))))))))))",
        replacement: "false)))))))))",
        vector: "a word ended by a quote mark",
    },
    Plant {
        label: "the tokens come out last first",
        root: 0,
        site: "(reverseSpans (project final tokens))",
        replacement: "(project final tokens)",
        vector: "the oracle's own text",
    },
    Plant {
        label: "an escaped backslash is dropped",
        root: 0,
        site: "(taken state (position + 2) (lit \"\\\\\"))",
        replacement: "(taken state (position + 2) (lit \"\"))",
        vector: "an escaped backslash",
    },
    Plant {
        label: "an escaped solidus is dropped",
        root: 0,
        site: "(taken state (position + 2) (lit \"/\"))",
        replacement: "(taken state (position + 2) (lit \"\"))",
        vector: "an escaped solidus",
    },
    Plant {
        label: "an escaped backspace is a nul",
        root: 0,
        site: "(byteOf 8)",
        replacement: "(byteOf 0)",
        vector: "an escaped backspace",
    },
    Plant {
        label: "an escaped form feed is a vertical tab",
        root: 0,
        site: "(byteOf 12)",
        replacement: "(byteOf 11)",
        vector: "an escaped form feed",
    },
    Plant {
        label: "an escaped line feed is a space",
        root: 0,
        site: "(byteOf 10)",
        replacement: "(byteOf 32)",
        vector: "an escaped line feed",
    },
    Plant {
        label: "an escaped carriage return is a line feed",
        root: 0,
        site: "(byteOf 13)",
        replacement: "(byteOf 10)",
        vector: "an escaped carriage return",
    },
    Plant {
        label: "an escaped tab is a space",
        root: 0,
        site: "(byteOf 9)",
        replacement: "(byteOf 32)",
        vector: "an escaped tab",
    },
    Plant {
        label: "a upper-case hexadecimal digit is no digit",
        root: 0,
        site: "(value <=? 70)",
        replacement: "false",
        vector: "a unicode escape of an upper-case digit",
    },
    Plant {
        label: "a hexadecimal digit weighs fifteen",
        root: 0,
        site: "(prim multiply a 16 : Nat)",
        replacement: "(prim multiply a 15 : Nat)",
        vector: "a unicode escape of each place",
    },
    Plant {
        label: "the first surrogate is a scalar",
        root: 0,
        site: "(55296 <=? code)",
        replacement: "(55297 <=? code)",
        vector: "a first surrogate",
    },
    Plant {
        label: "the scalar before the surrogates is not one",
        root: 0,
        site: "(55296 <=? code)",
        replacement: "(55295 <=? code)",
        vector: "a unicode escape of the last scalar before the surrogates",
    },
    Plant {
        label: "the last surrogate is a scalar",
        root: 0,
        site: "(code <=? 57343)",
        replacement: "(code <=? 57342)",
        vector: "a last surrogate",
    },
    Plant {
        label: "the scalar after the surrogates is not one",
        root: 0,
        site: "(code <=? 57343)",
        replacement: "(code <=? 57344)",
        vector: "a unicode escape of the first scalar after the surrogates",
    },
    Plant {
        label: "a two-byte scalar ends at 0x7fe",
        root: 0,
        site: "(code <? 2048)",
        replacement: "(code <? 2047)",
        vector: "a unicode escape of the last two-byte scalar",
    },
    Plant {
        label: "the delete is a two-byte scalar",
        root: 0,
        site: "(code <? 128)",
        replacement: "(code <? 127)",
        vector: "a unicode escape of the delete",
    },
    Plant {
        label: "a two-byte scalar leads with 0xc1",
        root: 0,
        site: "(byteOf (192 +",
        replacement: "(byteOf (193 +",
        vector: "a unicode escape of a lower-case digit",
    },
    Plant {
        label: "a three-byte scalar leads with 0xe1",
        root: 0,
        site: "(byteOf (224 +",
        replacement: "(byteOf (225 +",
        vector: "a unicode escape of a letter",
    },
    Plant {
        label: "a surrogate's message names the wrong digit",
        root: 0,
        site: "(hexAbove code 256)",
        replacement: "(hexAbove code 4096)",
        vector: "a first surrogate",
    },
    Plant {
        label: "an unterminated string is found at its second byte",
        root: 0,
        site: "(failed start (lit \"unterminated string\"))",
        replacement: "(failed (start + 1) (lit \"unterminated string\"))",
        vector: "an unterminated string after tokens",
    },
    Plant {
        label: "a short unicode escape is found at its first digit",
        root: 0,
        site: "(halted state position (lit \"a `\\\\u` escape takes four hexadecimal digits\"))",
        replacement: "(halted state (position + 2) (lit \"a `\\\\u` escape takes four hexadecimal digits\"))",
        vector: "a unicode escape of two digits",
    },
    Plant {
        label: "a raw run does not stop at a backslash",
        root: 0,
        site: "(if (value =? 92) then (ctor Option.none [Nat])",
        replacement: "(if false then (ctor Option.none [Nat])",
        vector: "a string with an escape after raw text",
    },
    Plant {
        label: "a raw run does not stop at a quote mark",
        root: 0,
        site: "(if (value =? 34) then (ctor Option.none [Nat])",
        replacement: "(if false then (ctor Option.none [Nat])",
        vector: "a string",
    },
    Plant {
        label: "the space is a control character",
        root: 0,
        site: "(byteNum subject position) <? 32)",
        replacement: "(byteNum subject position) <? 33)",
        vector: "a string with a space",
    },
    Plant {
        label: "the unit separator is no control character",
        root: 0,
        site: "(byteNum subject position) <? 32)",
        replacement: "(byteNum subject position) <? 31)",
        vector: "a unit separator in a string",
    },
    Plant {
        label: "a bad escape names one byte of its scalar",
        root: 0,
        site: "(widthAt subject (position + 1))",
        replacement: "1",
        vector: "an escape of a scalar beyond ASCII",
    },
    Plant {
        label: "a bad escape is found at the character after it",
        root: 0,
        site: "(halted state position (cat (cat (lit \"`\\\\\")",
        replacement: "(halted state (position + 1) (cat (cat (lit \"`\\\\\")",
        vector: "an escape that is not one",
    },
    Plant {
        label: "a quote mark is not escaped when quoted",
        root: 1,
        site: "(if (value =? 34) then (lit \"\\\\\\\"\")",
        replacement: "(if (value =? 34) then (lit \"\\\"\")",
        vector: "quote a quote mark",
    },
    Plant {
        label: "a backslash is not escaped when quoted",
        root: 1,
        site: "(if (value =? 92) then (lit \"\\\\\\\\\")",
        replacement: "(if (value =? 92) then (lit \"\\\\\")",
        vector: "quote a backslash",
    },
    Plant {
        label: "a backspace is written as a b without its backslash",
        root: 1,
        site: "(if (value =? 8) then (lit \"\\\\b\")",
        replacement: "(if (value =? 8) then (lit \"b\")",
        vector: "quote a backspace",
    },
    Plant {
        label: "a form feed is written as an n",
        root: 1,
        site: "(if (value =? 12) then (lit \"\\\\f\")",
        replacement: "(if (value =? 12) then (lit \"\\\\n\")",
        vector: "quote a form feed",
    },
    Plant {
        label: "a line feed is written as an r",
        root: 1,
        site: "(if (value =? 10) then (lit \"\\\\n\")",
        replacement: "(if (value =? 10) then (lit \"\\\\r\")",
        vector: "quote a line feed",
    },
    Plant {
        label: "a carriage return is written as a t",
        root: 1,
        site: "(if (value =? 13) then (lit \"\\\\r\")",
        replacement: "(if (value =? 13) then (lit \"\\\\t\")",
        vector: "quote a carriage return",
    },
    Plant {
        label: "a tab is written as an f",
        root: 1,
        site: "(if (value =? 9) then (lit \"\\\\t\")",
        replacement: "(if (value =? 9) then (lit \"\\\\f\")",
        vector: "quote a tab",
    },
    Plant {
        label: "a control character is written with three digits",
        root: 1,
        site: "(lit \"\\\\u00\")",
        replacement: "(lit \"\\\\u0\")",
        vector: "quote U+0001",
    },
    Plant {
        label: "the unit separator is written raw",
        root: 1,
        site: "(if (if (value <? 32) then true",
        replacement: "(if (if (value <? 31) then true",
        vector: "quote U+001F",
    },
    Plant {
        label: "a control character's low digit is its high digit",
        root: 1,
        site: "(hex2 value)",
        replacement: "(cat (hexAbove value 16) (hexAbove value 16))",
        vector: "quote U+000B",
    },
    Plant {
        label: "the quotation leaves out the text",
        root: 1,
        site: "(joined (project final pieces))",
        replacement: "0x",
        vector: "quote a text",
    },
];

fn plants(vectors: &[Vector]) {
    let mut missed = Vec::new();
    for plant in PLANTS {
        let vector = vectors
            .iter()
            .find(|vector| vector.label == plant.vector)
            .unwrap_or_else(|| panic!("{}: no vector `{}`", plant.label, plant.vector));
        assert_eq!(
            vector.quoting,
            plant.root == 1,
            "{}: the vector is a run of the root planted",
            plant.label
        );
        let planted = support::P::bootstrap();
        let source = planted.read_raw("src/SurfaceLex.lex.tex");
        assert_eq!(
            source.matches(plant.site).count(),
            1,
            "{}: the plant site occurs once",
            plant.label
        );
        planted.write(
            "src/SurfaceLex.lex.tex",
            &source.replacen(plant.site, plant.replacement, 1),
        );
        let checked = support::checked_project(&planted);
        let low = super::lower(&checked, &planted, MODULE, ROOTS[plant.root].0);
        match super::evaluate(&low, &[vector.run().0], 1).first() {
            Some(first) => eprintln!(
                "BS-22: plant detected: {}: {}",
                plant.label,
                first.chars().take(160).collect::<String>()
            ),
            None => missed.push(plant.label),
        }
    }
    assert!(
        missed.is_empty(),
        "planted defects their own vector does not detect: {missed:?}"
    );
}

fn export(lowered: &BTreeMap<&str, super::Lowered>) {
    let exported = harness::exported("bootstrap");
    let bundle = exported.bundle();
    let mut realized = 0usize;
    for exported_root in &exported.output.result.roots {
        let Some(member) = exported_root
            .root
            .strip_prefix(&format!("{MODULE}."))
            .and_then(|member| lowered.get(member).map(|low| (member, low)))
        else {
            continue;
        };
        let largest = exported_root
            .outputs
            .iter()
            .flat_map(|output| &output.certification.certificates)
            .map(|certificate| certificate.file.byte_length)
            .max()
            .expect("a certificate");
        eprintln!(
            "BS-22: {}: largest certificate part {largest} bytes",
            exported_root.root
        );
        assert!(
            largest <= CERTIFICATE_BUDGET,
            "{}: a certificate part of {largest} bytes is beyond {CERTIFICATE_BUDGET}",
            exported_root.root
        );
        for output in &exported_root.outputs {
            let package: Json = serde_json::from_slice(
                &std::fs::read(
                    bundle
                        .join(&output.package.directory)
                        .join("package.json")
                        .as_std_path(),
                )
                .expect("package.json"),
            )
            .expect("package JSON");
            assert_eq!(
                package["program"],
                serde_json::to_value(&member.1.program).expect("a program is JSON"),
                "{}: the exported package realizes the program the differential ran",
                member.0
            );
            realized += 1;
        }
    }
    assert_eq!(realized, ROOTS.len(), "one certified package per root");
    let packages = harness::tree(&bundle.join("packages"));
    let committed_packages = harness::tree(&harness::fixture_dir("bootstrap").join("packages"));
    assert_eq!(
        packages.keys().collect::<Vec<_>>(),
        committed_packages.keys().collect::<Vec<_>>(),
        "the exported package files are the committed ones"
    );
    for (relative, content) in &packages {
        assert!(
            committed_packages.get(relative) == Some(content),
            "packages/{relative} differs from the committed package"
        );
    }
}

#[allow(clippy::too_many_lines)]
pub(super) fn case() {
    let started = std::time::Instant::now();
    let project = support::P::bootstrap();
    let checked_units = project.check_ok();
    assert!(
        checked_units.units.contains_key("SurfaceLex")
            && checked_units.units.contains_key("AxiomOutput"),
        "SurfaceLex and AxiomOutput are linked modules of the project"
    );
    eligibility_of(&project, "SurfaceLex", MODULE, &ROOTS);
    assert_eq!(
        sorts().keys().map(String::as_str).collect::<Vec<_>>(),
        ["Token"],
        "the sorts of the module"
    );

    let vectors = named();
    let labels: BTreeSet<&str> = vectors.iter().map(|vector| vector.label.as_str()).collect();
    assert_eq!(labels.len(), vectors.len(), "each vector has its own label");
    let mut messages = Vec::new();
    let mut lexing: Vec<Run> = Vec::new();
    let mut quoting: Vec<Run> = Vec::new();
    for vector in &vectors {
        let (run, refusal) = vector.run();
        messages.extend(refusal);
        if vector.quoting {
            quoting.push(run);
        } else {
            lexing.push(run);
        }
    }
    for refusal in REFUSALS {
        assert!(
            messages.iter().any(|message| message.contains(refusal)),
            "no vector reaches the error `{refusal}`"
        );
    }
    let accepted = lexing.iter().filter(|run| run.1["kind"] == "ok").count();

    let sources = super::scan::sources();
    let evaluator_windows: Vec<String> = sources
        .iter()
        .map(|(path, source)| window(path, source, EVALUATOR_WINDOW))
        .collect();
    let rustc_windows: Vec<String> = sources
        .iter()
        .map(|(path, source)| window(path, source, RUSTC_WINDOW))
        .collect();
    let normative = support::normative_host();
    let (mut evaluator_lexing, mut evaluator_quoting) = (Vec::new(), Vec::new());
    let (mut rustc_lexing, mut rustc_quoting) = (Vec::new(), Vec::new());
    for source in &evaluator_windows {
        evaluator_lexing.push(lex_run(source).0);
        evaluator_quoting.push(quote_run(source));
    }
    for source in &rustc_windows {
        rustc_lexing.push(lex_run(source).0);
        rustc_quoting.push(quote_run(source));
    }
    let bases: Vec<String> = vectors
        .iter()
        .map(|vector| vector.source.clone())
        .chain(evaluator_windows.iter().cloned())
        .collect();
    let (mut evaluator_mutants, mut evaluator_quoted) = if normative {
        mutants(EVALUATOR_MUTANTS, &bases)
    } else {
        (Vec::new(), Vec::new())
    };
    let (mut rustc_mutants, mut rustc_quoted) = if normative {
        mutants(RUSTC_MUTANTS, &bases)
    } else {
        (Vec::new(), Vec::new())
    };
    eprintln!(
        "BS-22: {} vectors ({} lexed, {} accepted, {} quoted), {} committed sources, {} and {} seeded runs ({:?})",
        vectors.len(),
        lexing.len(),
        accepted,
        quoting.len(),
        sources.len(),
        evaluator_mutants.len(),
        rustc_mutants.len(),
        started.elapsed()
    );

    let checked = support::checked_project(&project);
    let lowered: BTreeMap<&str, super::Lowered> = ROOTS
        .iter()
        .map(|(root, _)| (*root, super::lower(&checked, &project, MODULE, root)))
        .collect();
    if normative {
        let mut lex_corpus: Vec<Run> = lexing.clone();
        lex_corpus.append(&mut evaluator_lexing);
        lex_corpus.append(&mut evaluator_mutants);
        let mut quote_corpus: Vec<Run> = quoting.clone();
        quote_corpus.append(&mut evaluator_quoting);
        quote_corpus.append(&mut evaluator_quoted);
        for (root, corpus) in [("lexText", &lex_corpus), ("quoteText", &quote_corpus)] {
            let disagreements = super::evaluate(&lowered[root], corpus, 5);
            assert!(
                disagreements.is_empty(),
                "{root} and the Rust oracle disagree: {disagreements:?}"
            );
            eprintln!(
                "BS-22: {root}: the evaluator agrees on {} runs ({:?})",
                corpus.len(),
                started.elapsed()
            );
        }
        plants(&vectors);
    } else {
        eprintln!("BS-22: platform-bound host: the evaluator, the seeded runs, and the planted defects run on the normative host");
    }

    let mut compiled_lexing = lexing;
    compiled_lexing.append(&mut rustc_lexing);
    compiled_lexing.append(&mut rustc_mutants);
    let mut compiled_quoting = quoting;
    compiled_quoting.append(&mut rustc_quoting);
    compiled_quoting.append(&mut rustc_quoted);
    let total = rustc_differential(
        "BS-22",
        &[
            ("lexText", &lowered["lexText"], compiled_lexing.as_slice()),
            (
                "quoteText",
                &lowered["quoteText"],
                compiled_quoting.as_slice(),
            ),
        ],
        "oracle",
    );
    eprintln!("BS-22: {total} rustc runs agree ({:?})", started.elapsed());

    if !support::lean_backed("BS-22") {
        return;
    }
    export(&lowered);
    eprintln!("BS-22: passed in {:?}", started.elapsed());
}
