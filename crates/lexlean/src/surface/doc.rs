//! A small pretty-printing algebra (Wadler/Oppen): text, concatenation,
//! indentation, a break that is a space or a newline, and groups that stay on
//! one line when they fit.
//!
//! The surface printer builds a [`Doc`] from a value and lays it out at a
//! fixed width, so the canonical spelling of a value is a function of the
//! value alone: no source layout survives `fmt`.

/// A document to lay out.
#[derive(Debug, Clone)]
pub enum Doc {
    /// Nothing.
    Nil,
    /// Text without a newline.
    Text(String),
    /// Documents one after the other.
    Concat(Vec<Doc>),
    /// The document with every newline inside it indented further.
    Nest(usize, Box<Doc>),
    /// A space when the enclosing group is laid out flat, else a newline.
    Line,
    /// Nothing when the enclosing group is laid out flat, else a newline.
    Soft,
    /// A document laid out flat when it fits in the rest of the line.
    Group(Box<Doc>),
}

impl Doc {
    /// Text.
    #[must_use]
    pub fn text(text: impl Into<String>) -> Self {
        Self::Text(text.into())
    }

    /// Documents one after the other.
    #[must_use]
    pub fn concat(parts: Vec<Self>) -> Self {
        Self::Concat(parts)
    }

    /// Indent the newlines of a document.
    #[must_use]
    pub fn nest(self, by: usize) -> Self {
        Self::Nest(by, Box::new(self))
    }

    /// Lay a document out flat when it fits.
    #[must_use]
    pub fn group(self) -> Self {
        Self::Group(Box::new(self))
    }

    /// `items` separated by `separator`, then a [`Doc::Line`].
    #[must_use]
    pub fn join(items: Vec<Self>, separator: &str) -> Self {
        let mut parts = Vec::with_capacity(items.len() * 3);
        let count = items.len();
        for (index, item) in items.into_iter().enumerate() {
            parts.push(item);
            if index + 1 < count {
                parts.push(Self::text(separator));
                parts.push(Self::Line);
            }
        }
        Self::Concat(parts)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Mode {
    Flat,
    Break,
}

/// Whether `rest` (from the work stack) fits in `remaining` columns, up to the
/// first newline it would print in break mode.
fn fits(mut remaining: isize, stack: &[(usize, Mode, &Doc)], first: (usize, Mode, &Doc)) -> bool {
    let mut work: Vec<(usize, Mode, &Doc)> = vec![first];
    let mut below = stack.iter().rev();
    loop {
        if remaining < 0 {
            return false;
        }
        let Some((indent, mode, doc)) = work.pop().or_else(|| below.next().copied()) else {
            return true;
        };
        match doc {
            Doc::Nil => {}
            Doc::Text(text) => {
                remaining -= isize::try_from(text.chars().count()).unwrap_or(isize::MAX);
            }
            Doc::Concat(parts) => {
                for part in parts.iter().rev() {
                    work.push((indent, mode, part));
                }
            }
            Doc::Nest(by, inner) => work.push((indent + by, mode, inner)),
            Doc::Line => match mode {
                Mode::Flat => remaining -= 1,
                Mode::Break => return true,
            },
            Doc::Soft => {
                if mode == Mode::Break {
                    return true;
                }
            }
            Doc::Group(inner) => work.push((indent, Mode::Flat, inner)),
        }
    }
}

/// Lay a document out at `width` columns.
#[must_use]
pub fn render(doc: &Doc, width: usize) -> String {
    let mut out = String::new();
    let mut column: usize = 0;
    let mut stack: Vec<(usize, Mode, &Doc)> = vec![(0, Mode::Break, doc)];
    while let Some((indent, mode, doc)) = stack.pop() {
        match doc {
            Doc::Nil => {}
            Doc::Text(text) => {
                out.push_str(text);
                column += text.chars().count();
            }
            Doc::Concat(parts) => {
                for part in parts.iter().rev() {
                    stack.push((indent, mode, part));
                }
            }
            Doc::Nest(by, inner) => stack.push((indent + by, mode, inner)),
            Doc::Line | Doc::Soft => match mode {
                Mode::Flat => {
                    if matches!(doc, Doc::Line) {
                        out.push(' ');
                        column += 1;
                    }
                }
                Mode::Break => {
                    out.push('\n');
                    out.extend(std::iter::repeat_n(' ', indent));
                    column = indent;
                }
            },
            Doc::Group(inner) => {
                let remaining = isize::try_from(width).unwrap_or(isize::MAX)
                    - isize::try_from(column).unwrap_or(isize::MAX);
                let flat = fits(remaining, &stack, (indent, Mode::Flat, inner));
                stack.push((indent, if flat { Mode::Flat } else { Mode::Break }, inner));
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::{render, Doc};

    fn list(items: &[&str]) -> Doc {
        Doc::concat(vec![
            Doc::text("["),
            Doc::concat(vec![
                Doc::Soft,
                Doc::join(items.iter().map(|item| Doc::text(*item)).collect(), ","),
            ])
            .nest(2),
            Doc::Soft,
            Doc::text("]"),
        ])
        .group()
    }

    /// A group that fits stays on one line; one that does not breaks every
    /// line of its own and no more.
    #[test]
    fn a_group_breaks_only_when_it_does_not_fit() {
        let doc = list(&["a", "bb", "ccc"]);
        assert_eq!(render(&doc, 20), "[a, bb, ccc]");
        assert_eq!(render(&doc, 8), "[\n  a,\n  bb,\n  ccc\n]");
    }

    /// An inner group that fits is flat inside a broken outer group.
    #[test]
    fn nested_groups_break_outermost_first() {
        let doc = Doc::concat(vec![
            Doc::text("["),
            Doc::concat(vec![
                Doc::Soft,
                Doc::join(vec![list(&["x", "y"]), list(&["z"])], ","),
            ])
            .nest(2),
            Doc::Soft,
            Doc::text("]"),
        ])
        .group();
        assert_eq!(render(&doc, 12), "[\n  [x, y],\n  [z]\n]");
    }
}
