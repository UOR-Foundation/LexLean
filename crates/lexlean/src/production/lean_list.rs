//! A list in certificate text (SPEC.md §17.17): a term Lean elaborates
//! without binders, whatever its length.
//!
//! Lean's `[a, b, …]` macro (`Init/Data/List/Notation.lean`) writes a literal
//! of at most 32 elements as `List.cons` applications, and a longer one as
//! `%[… | List.nil]`, which `Init/NotationExtra.lean` splits in halves under
//! `let y := …` binders; `#[…]` is `List.toArray […]` and splits the same way.
//! A witness whose elements are postponed to metavariables does not unify
//! through those binders, so a longer list is written as the `List.cons`
//! chain the macro writes for a short one.

/// The longest list written as a `[…]` literal: Lean expands fewer than 64
/// elements and separators without binders.
pub const LITERAL_MAX: usize = 32;

/// A list term of `items`, separated by `, `.
#[must_use]
pub fn list(items: impl IntoIterator<Item = String>) -> String {
    list_by(items, ", ")
}

/// A list term of `items`, joined by `separator` (a comma, then the
/// whitespace to keep between elements).
#[must_use]
pub fn list_by(items: impl IntoIterator<Item = String>, separator: &str) -> String {
    let items: Vec<String> = items.into_iter().collect();
    if items.len() <= LITERAL_MAX {
        return format!("[{}]", items.join(separator));
    }
    let space = separator.strip_prefix(',').unwrap_or(separator);
    let mut out = String::from("(");
    for item in &items {
        if atomic(item) {
            out.push_str(item);
        } else {
            out.push('(');
            out.push_str(item);
            out.push(')');
        }
        out.push_str(" ::");
        out.push_str(space);
    }
    out.push_str("[])");
    out
}

/// An array term of `items`: `#[…]`, or `List.toArray` of the chain.
#[must_use]
pub fn array(items: impl IntoIterator<Item = String>) -> String {
    let items: Vec<String> = items.into_iter().collect();
    if items.len() <= LITERAL_MAX {
        format!("#[{}]", items.join(", "))
    } else {
        format!("(List.toArray {})", list(items))
    }
}

/// Whether `item` is one term that `::` cannot split: a name, a numeral, or
/// one bracketed group.
fn atomic(item: &str) -> bool {
    let mut chars = item.chars();
    let Some(open) = chars.next() else {
        return false;
    };
    let close = match open {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '⟨' => '⟩',
        _ => {
            return item
                .chars()
                .all(|c| c.is_alphanumeric() || matches!(c, '_' | '.' | '«' | '»' | '\''));
        }
    };
    // The opening bracket closes at the last character, outside any string
    // literal; anything after it would be a further term.
    let mut depth = 0usize;
    let mut in_string = false;
    let mut escaped = false;
    let last = item.chars().count() - 1;
    for (position, c) in item.chars().enumerate() {
        if in_string {
            match (escaped, c) {
                (true, _) => escaped = false,
                (false, '\\') => escaped = true,
                (false, '"') => in_string = false,
                _ => {}
            }
            continue;
        }
        if c == '"' {
            in_string = true;
        } else if c == open {
            depth += 1;
        } else if c == close {
            depth -= 1;
            if depth == 0 {
                return position == last;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::{array, list, list_by, LITERAL_MAX};

    fn numbers(count: usize) -> Vec<String> {
        (0..count).map(|n| n.to_string()).collect()
    }

    #[test]
    fn a_short_list_is_a_literal() {
        assert_eq!(list(Vec::new()), "[]");
        assert_eq!(list(numbers(3)), "[0, 1, 2]");
        assert!(list(numbers(LITERAL_MAX)).starts_with('['));
        assert_eq!(array(numbers(2)), "#[0, 1]");
    }

    #[test]
    fn a_long_list_is_a_cons_chain() {
        let text = list(numbers(LITERAL_MAX + 1));
        assert!(text.starts_with("(0 :: 1 :: "), "{text}");
        assert!(text.ends_with(" :: 32 :: [])"), "{text}");
        assert_eq!(text.matches('[').count(), 1, "{text}");
        assert_eq!(
            array(numbers(LITERAL_MAX + 1)),
            format!("(List.toArray {})", list(numbers(LITERAL_MAX + 1)))
        );
    }

    #[test]
    fn an_element_that_is_not_one_term_is_parenthesized() {
        let mut items: Vec<String> = numbers(LITERAL_MAX);
        items.push("fun x => x".to_owned());
        items.push("(a) (b)".to_owned());
        items.push("(.str \"(\")".to_owned());
        items.push("{ a := 1 }".to_owned());
        let text = list(items);
        assert!(text.contains(":: (fun x => x) ::"), "{text}");
        assert!(text.contains(":: ((a) (b)) ::"), "{text}");
        assert!(text.contains(":: (.str \"(\") ::"), "{text}");
        assert!(text.contains(":: { a := 1 } ::"), "{text}");
    }

    #[test]
    fn the_separator_keeps_its_whitespace() {
        let text = list_by(numbers(LITERAL_MAX + 1), ",\n    ");
        assert!(text.starts_with("(0 ::\n    1 ::\n    "), "{text}");
        assert!(text.ends_with("32 ::\n    [])"), "{text}");
    }
}
