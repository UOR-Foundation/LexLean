//! Lexing of the surface notation.
//!
//! A token is a delimiter (`:=` and `::` are one token each), a quoted string, or a word: the longest run of
//! scalars that are neither white space, a delimiter, nor a quote. Everything
//! the notation spells (names, numerals, operators, keywords) is a word; what
//! a word means is decided by the position that reads it.

use super::SurfaceError;

/// One lexical token.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    /// `{`.
    LBrace,
    /// `}`.
    RBrace,
    /// `[`.
    LBrack,
    /// `]`.
    RBrack,
    /// `(`.
    LParen,
    /// `)`.
    RParen,
    /// `,`.
    Comma,
    /// `:`.
    Colon,
    /// `:=`.
    ColonEq,
    /// `::`.
    Scope,
    /// `;`.
    Semi,
    /// A quoted string, escapes decoded.
    Str(String),
    /// A word.
    Word(String),
}

/// A token with the byte range it was read from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Spanned {
    /// The token.
    pub token: Token,
    /// The first byte of the token.
    pub start: usize,
    /// One past the last byte of the token.
    pub end: usize,
}

/// Whether a scalar ends a word.
fn is_delimiter(c: char) -> bool {
    c.is_whitespace() || matches!(c, '{' | '}' | '[' | ']' | '(' | ')' | ',' | ':' | ';' | '"')
}

/// Split `text` into tokens.
///
/// # Errors
///
/// An unterminated string or an invalid escape.
pub fn lex(text: &str) -> Result<Vec<Spanned>, SurfaceError> {
    let bytes = text.as_bytes();
    let mut tokens = Vec::new();
    let mut index = 0usize;
    while index < bytes.len() {
        let rest = &text[index..];
        let Some(c) = rest.chars().next() else {
            break;
        };
        let start = index;
        if c.is_whitespace() {
            index += c.len_utf8();
            continue;
        }
        let token = match c {
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            '[' => Token::LBrack,
            ']' => Token::RBrack,
            '(' => Token::LParen,
            ')' => Token::RParen,
            ',' => Token::Comma,
            ';' => Token::Semi,
            ':' => {
                if rest.starts_with(":=") {
                    index += 1;
                    Token::ColonEq
                } else if rest.starts_with("::") {
                    index += 1;
                    Token::Scope
                } else {
                    Token::Colon
                }
            }
            '"' => {
                let (value, length) = string(rest, start)?;
                index += length - 1;
                Token::Str(value)
            }
            _ => {
                let length = rest
                    .char_indices()
                    .find(|(_, scalar)| is_delimiter(*scalar))
                    .map_or(rest.len(), |(at, _)| at);
                index += length - 1;
                Token::Word(rest[..length].to_owned())
            }
        };
        index += 1;
        tokens.push(Spanned {
            token,
            start,
            end: index,
        });
    }
    Ok(tokens)
}

/// Read the string literal at the start of `rest`; its decoded value and the
/// byte length of the literal.
pub(crate) fn string(rest: &str, offset: usize) -> Result<(String, usize), SurfaceError> {
    let mut value = String::new();
    let mut chars = rest.char_indices();
    chars.next();
    while let Some((at, c)) = chars.next() {
        match c {
            '"' => return Ok((value, at + 1)),
            '\\' => {
                let Some((_, escape)) = chars.next() else {
                    break;
                };
                match escape {
                    '"' => value.push('"'),
                    '\\' => value.push('\\'),
                    '/' => value.push('/'),
                    'b' => value.push('\u{0008}'),
                    'f' => value.push('\u{000C}'),
                    'n' => value.push('\n'),
                    'r' => value.push('\r'),
                    't' => value.push('\t'),
                    'u' => {
                        let mut code = 0u32;
                        for _ in 0..4 {
                            let digit = chars
                                .next()
                                .and_then(|(_, digit)| digit.to_digit(16))
                                .ok_or_else(|| SurfaceError {
                                    offset: offset + at,
                                    message: "a `\\u` escape takes four hexadecimal digits"
                                        .to_owned(),
                                })?;
                            code = code * 16 + digit;
                        }
                        let scalar = char::from_u32(code).ok_or_else(|| SurfaceError {
                            offset: offset + at,
                            message: format!("`\\u{code:04x}` is not a Unicode scalar value"),
                        })?;
                        value.push(scalar);
                    }
                    other => {
                        return Err(SurfaceError {
                            offset: offset + at,
                            message: format!("`\\{other}` is not an escape"),
                        });
                    }
                }
            }
            c if (c as u32) < 0x20 => {
                return Err(SurfaceError {
                    offset: offset + at,
                    message: "a control character must be escaped in a string".to_owned(),
                });
            }
            c => value.push(c),
        }
    }
    Err(SurfaceError {
        offset,
        message: "unterminated string".to_owned(),
    })
}

/// Minimal escaping for a quoted string: the same as canonical JSON (§21.7).
#[must_use]
pub fn quote(text: &str) -> String {
    let mut out = String::with_capacity(text.len() + 2);
    out.push('"');
    for scalar in text.chars() {
        match scalar {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\u{0008}' => out.push_str("\\b"),
            '\u{000C}' => out.push_str("\\f"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

#[cfg(test)]
mod tests {
    use super::{lex, quote, Token};

    fn words(text: &str) -> Vec<Token> {
        lex(text)
            .expect("lexes")
            .into_iter()
            .map(|spanned| spanned.token)
            .collect()
    }

    /// Delimiters split words, `:=` is one token, and white space is dropped.
    #[test]
    fn delimiters_split_words() {
        assert_eq!(
            words("add{left: x, right: 1} := (a -> b);"),
            vec![
                Token::Word("add".into()),
                Token::LBrace,
                Token::Word("left".into()),
                Token::Colon,
                Token::Word("x".into()),
                Token::Comma,
                Token::Word("right".into()),
                Token::Colon,
                Token::Word("1".into()),
                Token::RBrace,
                Token::ColonEq,
                Token::LParen,
                Token::Word("a".into()),
                Token::Word("->".into()),
                Token::Word("b".into()),
                Token::RParen,
                Token::Semi,
            ]
        );
    }

    /// A string decodes its escapes and `quote` is its inverse.
    #[test]
    fn strings_round_trip() {
        for text in [
            "",
            "plain",
            "a \"b\" \\ c",
            "line\nbreak\ttab",
            "\u{1}\u{1f}",
            "é𝔸",
        ] {
            let quoted = quote(text);
            assert_eq!(
                words(&quoted),
                vec![Token::Str(text.to_owned())],
                "{quoted}"
            );
        }
    }

    /// Malformed strings fail with an offset, and never panic.
    #[test]
    fn malformed_strings_are_errors() {
        for text in [
            "\"open",
            "\"bad \\q\"",
            "\"\\u12\"",
            "\"\\ud800\"",
            "\"\n\"",
        ] {
            assert!(lex(text).is_err(), "{text:?}");
        }
    }
}
