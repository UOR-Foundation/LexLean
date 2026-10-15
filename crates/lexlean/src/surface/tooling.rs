//! The authoring notation of a tooling module (SPEC.md §17.20, §17.21).
//!
//! A tooling module is a typed registry and typed functions over it, not a
//! program in a language of terms, so it has no readable forms of its own: it
//! is written in the generic notation, which is exact for every value, as one
//! object whose `spec` member the language implies. A declaration is a `kind`
//! tagged object (`type{…}`, `call{…}`, `function{…}`, `command{…}`), and what
//! a position means is decided by the decoder that reads the canonical JSON
//! the notation is read to, as for a semantic module.

use std::collections::BTreeMap;

use crate::artifact::canonical_json::Json;

use super::generic::{self, Reader};
use super::lex::Token;
use super::SurfaceError;

/// The notation of a module: its members, without the `spec` the language
/// implies, laid out at the width of the generic notation and ended by a line
/// feed.
///
/// # Errors
///
/// A value that is not a module: an object with a `spec` string.
pub fn print_module(module: &Json) -> Result<String, SurfaceError> {
    let Json::Obj(members) = module else {
        return Err(SurfaceError {
            offset: 0,
            message: "a module is an object".to_owned(),
        });
    };
    if !matches!(members.get("spec"), Some(Json::Str(_))) {
        return Err(SurfaceError {
            offset: 0,
            message: "a module has a `spec`".to_owned(),
        });
    }
    let mut rest = members.clone();
    rest.remove("spec");
    let mut text = generic::print(&Json::Obj(rest));
    text.push('\n');
    Ok(text)
}

/// Read the notation of a module and make the module of it under `spec`.
///
/// # Errors
///
/// Text that is not one object of the generic notation, or that writes the
/// `spec` the language implies.
pub fn parse_module(text: &str, spec: &str) -> Result<Json, SurfaceError> {
    let Json::Obj(mut members) = generic::parse(text)? else {
        return Err(SurfaceError {
            offset: text.find(|c: char| !c.is_whitespace()).unwrap_or(0),
            message: "a module is an object".to_owned(),
        });
    };
    if members.contains_key("spec") {
        return Err(SurfaceError {
            offset: text.find("spec").unwrap_or(0),
            message: "the `spec` of a module is implied by its language and is not written"
                .to_owned(),
        });
    }
    let mut module: BTreeMap<String, Json> = std::mem::take(&mut members);
    module.insert("spec".to_owned(), Json::Str(spec.to_owned()));
    Ok(Json::Obj(module))
}

/// The byte range in `text` of every element of the module's `declarations`
/// array, in order: what the source map relates to a declaration's rendering.
///
/// # Errors
///
/// Text that is not the notation of a module.
pub fn declaration_spans(text: &str) -> Result<Vec<(usize, usize)>, SurfaceError> {
    let mut reader = Reader::new(text)?;
    reader.expect(&Token::LBrace, "`{`")?;
    let mut spans = Vec::new();
    while reader.peek() != Some(&Token::RBrace) {
        let offset = reader.offset();
        let key = match reader.bump() {
            Some(Token::Word(word)) => word,
            Some(Token::Str(text)) => text,
            _ => {
                return Err(SurfaceError {
                    offset,
                    message: "expected a member name".to_owned(),
                });
            }
        };
        reader.expect(&Token::Colon, "`:` after a member name")?;
        if key == "declarations" {
            reader.expect(&Token::LBrack, "`[` after `declarations`")?;
            while reader.peek() != Some(&Token::RBrack) {
                let start = reader.offset();
                generic::value(&mut reader, 2)?;
                spans.push((start, reader.last_end()));
                if !reader.eat(&Token::Comma) {
                    break;
                }
            }
            reader.expect(&Token::RBrack, "`,` or `]`")?;
        } else {
            generic::value(&mut reader, 1)?;
        }
        if !reader.eat(&Token::Comma) {
            break;
        }
    }
    reader.expect(&Token::RBrace, "`,` or `}`")?;
    if !reader.done() {
        return Err(reader.error("unexpected text after the module"));
    }
    Ok(spans)
}

#[cfg(test)]
mod tests {
    use super::{declaration_spans, parse_module, print_module};
    use crate::artifact::canonical_json::Json;

    const SPEC: &str = "lexlean/tooling-module/1";

    fn module() -> Json {
        Json::parse(
            br#"{"authority":{"id":"A","lean_githash":"g","lean_version":"4.32.1"},"declarations":[{"constructors":[{"fields":[],"name":"false"}],"kind":"type","name":"Bool","parameters":[]},{"kind":"call","name":"f"}],"imports":[],"spec":"lexlean/tooling-module/1"}"#,
        )
        .expect("a module")
    }

    /// A module is written without its `spec` and read back to itself, and
    /// each declaration has the span of its own text.
    #[test]
    fn a_module_and_its_declaration_spans() {
        let text = print_module(&module()).expect("prints");
        assert!(!text.contains("spec"), "{text}");
        assert_eq!(parse_module(&text, SPEC).expect("reads"), module());
        let spans = declaration_spans(&text).expect("spans");
        assert_eq!(spans.len(), 2);
        assert!(text[spans[0].0..spans[0].1].starts_with("type{"), "{text}");
        assert!(text[spans[1].0..spans[1].1].starts_with("call{"), "{text}");
    }

    /// What is not the notation of a module is an error, never a panic.
    #[test]
    fn malformed_notation_is_an_error() {
        for text in [
            "",
            "[]",
            "{",
            "{declarations: [type{]}",
            "{spec: x, declarations: []}",
            "{declarations: []} trailing",
            "{declarations: 3, declarations: []}",
        ] {
            assert!(
                parse_module(text, SPEC).is_err() || declaration_spans(text).is_err(),
                "{text:?}"
            );
        }
    }
}
