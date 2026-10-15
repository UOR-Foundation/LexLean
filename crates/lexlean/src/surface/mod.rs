//! The authoring surface: readable `.lex.tex` notation for the closed module
//! data (SPEC.md §17.21).
//!
//! A module's canonical JSON is the exact, schema-checked data every stage of
//! the compiler consumes; nobody should have to write it. This module is the
//! text a person writes and reads instead, with one rule above all: the
//! notation and the canonical JSON are in bijection. [`generic`] is the part
//! that holds for every JSON value; the sugar of the closed schemas layers on
//! top of it and falls back to it, so no value is ever unspellable and no
//! variant added to a schema can lack a surface.

pub mod core;
pub mod doc;
pub mod generic;
pub mod identity;
pub mod lex;
pub mod module;
pub mod semantic;
pub mod tooling;

/// A surface text that is not well formed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SurfaceError {
    /// The byte offset of the text the error is about.
    pub offset: usize,
    /// What is wrong.
    pub message: String,
}

impl SurfaceError {
    /// The one-based line and column of the offset in `text`.
    #[must_use]
    pub fn line_column(&self, text: &str) -> (usize, usize) {
        let before = &text[..self.offset.min(text.len())];
        let line = before.bytes().filter(|byte| *byte == b'\n').count() + 1;
        let column = before
            .rsplit('\n')
            .next()
            .map_or(0, |last| last.chars().count())
            + 1;
        (line, column)
    }
}

impl std::fmt::Display for SurfaceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} (at byte {})", self.message, self.offset)
    }
}

impl std::error::Error for SurfaceError {}
