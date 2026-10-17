//! The generic notation: every canonical JSON value (§21.7) has exactly one
//! readable spelling, and the spelling parses back to the value.
//!
//! ```text
//! value  = integer | "true" | "false" | string | word
//!        | "[" [ value { "," value } [ "," ] ] "]"
//!        | [ tag ] "{" [ field { "," field } [ "," ] ] "}" ;
//! field  = key ":" value ;
//! key    = word | string ;
//! tag    = word ;
//! ```
//!
//! A word is a string written without quotes when it is a bare name (§17.21);
//! a tagged object `K{…}` is the object that has the member `"kind": "K"`
//! with that member left out. Object members print in a fixed reading order
//! (see [`ordered`]) and parse in any order; no source layout survives printing.

use std::collections::BTreeMap;

use crate::artifact::canonical_json::Json;

use super::doc::{render, Doc};
use super::lex::{lex, quote, Spanned, Token};
use super::SurfaceError;

/// The width the printer lays a value out at.
pub const WIDTH: usize = 100;

/// The deepest nesting the parser reads. The decoder that consumes the
/// canonical text reads 128 levels; this is the bound that keeps the parser
/// itself off the end of the stack on hostile text.
pub const MAX_DEPTH: usize = 256;

/// The members that print first, in this order; every other member follows in
/// key order. The order is for the reader (a declaration's name comes before
/// its body); it carries no meaning, and the parser takes members in any order.
const PRIORITY: [&str; 12] = [
    "name",
    "type_parameters",
    "parameters",
    "type",
    "result",
    "statement",
    "fields",
    "constructors",
    "arguments",
    "body",
    "proof",
    "axioms",
];

/// The members of an object in the order they print.
#[must_use]
pub fn ordered(object: &BTreeMap<String, Json>) -> Vec<(&String, &Json)> {
    let rank = |key: &str| {
        PRIORITY
            .iter()
            .position(|first| *first == key)
            .unwrap_or(PRIORITY.len())
    };
    let mut members: Vec<(&String, &Json)> = object.iter().collect();
    members.sort_by(|(left, _), (right, _)| rank(left).cmp(&rank(right)).then(left.cmp(right)));
    members
}

/// Whether `text` is written as a word: a letter or `_` first, then letters,
/// digits and `_ . ' / -`, and not a word the notation reserves.
#[must_use]
pub fn is_bare(text: &str) -> bool {
    let mut chars = text.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    (first.is_ascii_alphabetic() || first == '_')
        && chars.all(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | '.' | '\'' | '/' | '-'))
        && !matches!(text, "true" | "false")
}

/// A string as the notation spells it.
#[must_use]
pub fn spell(text: &str) -> String {
    if is_bare(text) {
        text.to_owned()
    } else {
        quote(text)
    }
}

/// The document of a value.
#[must_use]
pub fn doc(value: &Json) -> Doc {
    match value {
        Json::Bool(true) => Doc::text("true"),
        Json::Bool(false) => Doc::text("false"),
        Json::Int(number) => Doc::text(number.to_string()),
        Json::Str(text) => Doc::text(spell(text)),
        Json::Arr(items) => delimited("[", "]", items.iter().map(doc).collect()),
        Json::Obj(object) => {
            let tag = object.get("kind").and_then(|kind| match kind {
                Json::Str(kind) if is_bare(kind) => Some(kind.as_str()),
                _ => None,
            });
            let fields: Vec<Doc> = ordered(object)
                .into_iter()
                .filter(|(key, _)| !(tag.is_some() && key.as_str() == "kind"))
                .map(|(key, value)| field(key, doc(value)))
                .collect();
            let open = tag.map_or_else(|| "{".to_owned(), |kind| format!("{kind}{{"));
            delimited(&open, "}", fields)
        }
    }
}

/// `key: value`.
#[must_use]
pub fn field(key: &str, value: Doc) -> Doc {
    Doc::concat(vec![Doc::text(format!("{}: ", spell(key))), value])
}

/// Items between delimiters, on one line when they fit.
#[must_use]
pub fn delimited(open: &str, close: &str, items: Vec<Doc>) -> Doc {
    if items.is_empty() {
        return Doc::text(format!("{open}{close}"));
    }
    Doc::concat(vec![
        Doc::text(open),
        Doc::concat(vec![Doc::Soft, Doc::join(items, ",")]).nest(2),
        Doc::Soft,
        Doc::text(close),
    ])
    .group()
}

/// The generic spelling of a value.
#[must_use]
pub fn print(value: &Json) -> String {
    render(&doc(value), WIDTH)
}

/// A reader over the tokens of one text.
pub struct Reader<'t> {
    /// The text the tokens were read from.
    pub text: &'t str,
    tokens: Vec<Spanned>,
    at: usize,
}

impl<'t> Reader<'t> {
    /// Lex `text`.
    ///
    /// # Errors
    ///
    /// A lexical error.
    pub fn new(text: &'t str) -> Result<Self, SurfaceError> {
        Ok(Self {
            text,
            tokens: lex(text)?,
            at: 0,
        })
    }

    /// The next token, not consumed.
    #[must_use]
    pub fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.at).map(|spanned| &spanned.token)
    }

    /// The token `ahead` places after the next, not consumed.
    #[must_use]
    pub fn peek_at(&self, ahead: usize) -> Option<&Token> {
        self.tokens
            .get(self.at + ahead)
            .map(|spanned| &spanned.token)
    }

    /// The byte offset of the next token, or the end of the text.
    #[must_use]
    pub fn offset(&self) -> usize {
        self.tokens
            .get(self.at)
            .map_or(self.text.len(), |spanned| spanned.start)
    }

    /// The byte offset just past the last token consumed.
    #[must_use]
    pub fn last_end(&self) -> usize {
        self.at
            .checked_sub(1)
            .and_then(|last| self.tokens.get(last))
            .map_or(0, |spanned| spanned.end)
    }

    /// Consume and return the next token.
    pub fn bump(&mut self) -> Option<Token> {
        let token = self
            .tokens
            .get(self.at)
            .map(|spanned| spanned.token.clone());
        if token.is_some() {
            self.at += 1;
        }
        token
    }

    /// Whether every token has been consumed.
    #[must_use]
    pub fn done(&self) -> bool {
        self.at >= self.tokens.len()
    }

    /// An error at the next token.
    #[must_use]
    pub fn error(&self, message: impl Into<String>) -> SurfaceError {
        SurfaceError {
            offset: self.offset(),
            message: message.into(),
        }
    }

    /// Consume `expected` or fail.
    ///
    /// # Errors
    ///
    /// The next token is another.
    pub fn expect(&mut self, expected: &Token, what: &str) -> Result<(), SurfaceError> {
        if self.peek() == Some(expected) {
            self.at += 1;
            Ok(())
        } else {
            Err(self.error(format!("expected {what}")))
        }
    }

    /// Consume `expected` when it is next.
    pub fn eat(&mut self, expected: &Token) -> bool {
        if self.peek() == Some(expected) {
            self.at += 1;
            true
        } else {
            false
        }
    }

    /// Consume the word `expected` when it is next.
    pub fn eat_word(&mut self, expected: &str) -> bool {
        if matches!(self.peek(), Some(Token::Word(word)) if word == expected) {
            self.at += 1;
            true
        } else {
            false
        }
    }
}

/// An integer word: an optional `-` and decimal digits.
fn integer(word: &str) -> Option<&str> {
    let digits = word.strip_prefix('-').unwrap_or(word);
    (!digits.is_empty() && digits.bytes().all(|byte| byte.is_ascii_digit())).then_some(word)
}

/// Read one value in the generic notation.
///
/// # Errors
///
/// Text that is not a value.
pub fn value(reader: &mut Reader<'_>, depth: usize) -> Result<Json, SurfaceError> {
    if depth > MAX_DEPTH {
        return Err(reader.error("nested more deeply than the notation reads"));
    }
    let offset = reader.offset();
    match reader.bump() {
        Some(Token::Str(text)) => Ok(Json::Str(text)),
        Some(Token::LBrack) => {
            let mut items = Vec::new();
            while reader.peek() != Some(&Token::RBrack) {
                items.push(value(reader, depth + 1)?);
                if !reader.eat(&Token::Comma) {
                    break;
                }
            }
            reader.expect(&Token::RBrack, "`,` or `]`")?;
            Ok(Json::Arr(items))
        }
        Some(Token::LBrace) => object(reader, None, depth),
        Some(Token::Word(word)) => {
            if word == "true" {
                return Ok(Json::Bool(true));
            }
            if word == "false" {
                return Ok(Json::Bool(false));
            }
            if let Some(number) = integer(&word) {
                return number
                    .parse::<i64>()
                    .map(Json::Int)
                    .map_err(|_| SurfaceError {
                        offset,
                        message: format!("`{word}` does not fit a 64-bit integer"),
                    });
            }
            if reader.peek() == Some(&Token::LBrace) && is_bare(&word) {
                reader.bump();
                return object(reader, Some(word), depth);
            }
            if is_bare(&word) {
                Ok(Json::Str(word))
            } else {
                Err(SurfaceError {
                    offset,
                    message: format!("`{word}` is not a name; quote it to write a string"),
                })
            }
        }
        Some(other) => Err(SurfaceError {
            offset,
            message: format!("expected a value, found {}", describe(&other)),
        }),
        None => Err(reader.error("expected a value, found the end of the text")),
    }
}

/// A token as a diagnostic names it.
#[must_use]
pub fn describe(token: &Token) -> String {
    match token {
        Token::LBrace => "`{`".to_owned(),
        Token::RBrace => "`}`".to_owned(),
        Token::LBrack => "`[`".to_owned(),
        Token::RBrack => "`]`".to_owned(),
        Token::LParen => "`(`".to_owned(),
        Token::RParen => "`)`".to_owned(),
        Token::Comma => "`,`".to_owned(),
        Token::Colon => "`:`".to_owned(),
        Token::ColonEq => "`:=`".to_owned(),
        Token::Scope => "`::`".to_owned(),
        Token::Semi => "`;`".to_owned(),
        Token::Str(text) => format!("the string {}", quote(text)),
        Token::Word(word) => format!("`{word}`"),
    }
}

/// The members of an object, after its `{`.
fn object(
    reader: &mut Reader<'_>,
    tag: Option<String>,
    depth: usize,
) -> Result<Json, SurfaceError> {
    let mut members: BTreeMap<String, Json> = BTreeMap::new();
    if let Some(tag) = tag {
        members.insert("kind".to_owned(), Json::Str(tag));
    }
    while reader.peek() != Some(&Token::RBrace) {
        let offset = reader.offset();
        let key = match reader.bump() {
            Some(Token::Word(word)) if is_bare(&word) => word,
            Some(Token::Str(text)) => text,
            _ => {
                return Err(SurfaceError {
                    offset,
                    message: "expected a member name".to_owned(),
                });
            }
        };
        reader.expect(&Token::Colon, "`:` after a member name")?;
        let member = value(reader, depth + 1)?;
        if members.insert(key.clone(), member).is_some() {
            return Err(SurfaceError {
                offset,
                message: format!("the member `{key}` is written twice"),
            });
        }
        if !reader.eat(&Token::Comma) {
            break;
        }
    }
    reader.expect(&Token::RBrace, "`,` or `}`")?;
    Ok(Json::Obj(members))
}

/// Parse a whole text as one value of the generic notation.
///
/// # Errors
///
/// Text that is not exactly one value.
pub fn parse(text: &str) -> Result<Json, SurfaceError> {
    let mut reader = Reader::new(text)?;
    let parsed = value(&mut reader, 0)?;
    if !reader.done() {
        return Err(reader.error("unexpected text after the value"));
    }
    Ok(parsed)
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use proptest::prelude::*;

    use super::{parse, print};
    use crate::artifact::canonical_json::Json;

    fn arbitrary() -> impl Strategy<Value = Json> {
        let leaf = prop_oneof![
            any::<bool>().prop_map(Json::Bool),
            any::<i64>().prop_map(Json::Int),
            "[a-zA-Z_][a-zA-Z0-9_.'/-]{0,6}".prop_map(Json::Str),
            ".{0,8}".prop_map(Json::Str),
            "[a-z]{1,6}".prop_map(Json::Str),
        ];
        leaf.prop_recursive(5, 48, 5, |inner| {
            prop_oneof![
                prop::collection::vec(inner.clone(), 0..5).prop_map(Json::Arr),
                prop::collection::btree_map("[a-z_]{1,5}|kind", inner, 0..5)
                    .prop_map(|members: BTreeMap<String, Json>| Json::Obj(members)),
            ]
        })
    }

    proptest! {
        /// Printing a value and parsing the text gives the value back, and
        /// the printed text is its own canonical spelling.
        #[test]
        fn print_then_parse_is_the_identity(value in arbitrary()) {
            let text = print(&value);
            let parsed = parse(&text).map_err(|error| TestCaseError::fail(format!("{error:?}\n{text}")))?;
            prop_assert_eq!(&parsed, &value, "{}", text);
            prop_assert_eq!(print(&parsed), text);
        }
    }

    /// A tagged object drops its `kind` member and members print after it in
    /// key order; any order and a trailing comma parse to the same value.
    #[test]
    fn tagged_objects_and_member_order() {
        let value = parse(r#"add{right: x, left: 1,}"#).expect("parses");
        assert_eq!(print(&value), "add{left: 1, right: x}");
        assert_eq!(parse("add{left: 1, right: x}").expect("parses"), value);
        assert_eq!(
            parse(r#"{kind: "two words", a: []}"#).expect("parses"),
            parse(r#"{a: [], kind: "two words"}"#).expect("parses")
        );
    }

    /// Text that is not a value is an error at an offset, never a panic.
    #[test]
    fn malformed_text_is_an_error() {
        for text in [
            "",
            "[",
            "]",
            "{a}",
            "{a: }",
            "{a: 1 b: 2}",
            "{a: 1, a: 2}",
            "+",
            "1 2",
            "99999999999999999999",
            "{1: 2}",
            "k{kind: x}",
            "(a)",
        ] {
            assert!(parse(text).is_err(), "{text:?}");
        }
    }

    /// The nesting bound turns a hostile text into an error.
    #[test]
    fn deep_nesting_is_an_error() {
        let text = "[".repeat(10_000);
        assert!(parse(&text).is_err());
    }
}
