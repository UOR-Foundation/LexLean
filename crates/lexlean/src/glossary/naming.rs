//! The naming scheme of generated glossaries (SPEC.md §13.12): how a Lean
//! declaration name becomes an entry ID, a canonical surface, and the words of
//! a prose form. Every function is pure and ASCII-only, so a name maps to the
//! same strings on every host and in every release that keeps the scheme.

/// The lowercase words of one name component. A word ends at `_`, before an
/// uppercase letter that follows a lowercase letter or digit, and before the
/// last capital of a capital run that a lowercase letter follows (`HAdd` is
/// `h add`, `ENNReal` is `enn real`). A prime is the word `prime`. Digits
/// stay with the word they follow. Characters outside ASCII letters, digits,
/// `_`, and `'` contribute nothing; the extractor refuses such names, so this
/// is only a total function, not a policy.
#[must_use]
pub fn words(component: &str) -> Vec<String> {
    let chars: Vec<char> = component.chars().collect();
    let mut words: Vec<String> = Vec::new();
    let mut current = String::new();
    let flush = |current: &mut String, words: &mut Vec<String>| {
        if !current.is_empty() {
            words.push(std::mem::take(current));
        }
    };
    for (index, &c) in chars.iter().enumerate() {
        match c {
            '_' => flush(&mut current, &mut words),
            '\'' => {
                flush(&mut current, &mut words);
                words.push("prime".to_owned());
            }
            c if c.is_ascii_uppercase() => {
                let previous = index.checked_sub(1).map(|i| chars[i]);
                let next = chars.get(index + 1).copied();
                let after_lower_or_digit =
                    previous.is_some_and(|p| p.is_ascii_lowercase() || p.is_ascii_digit());
                let ends_capital_run = previous.is_some_and(|p| p.is_ascii_uppercase())
                    && next.is_some_and(|n| n.is_ascii_lowercase());
                if after_lower_or_digit || ends_capital_run {
                    flush(&mut current, &mut words);
                }
                current.push(c.to_ascii_lowercase());
            }
            c if c.is_ascii_lowercase() || c.is_ascii_digit() => current.push(c),
            _ => {}
        }
    }
    flush(&mut current, &mut words);
    words
}

/// One entry-ID segment: the words of a component joined by `-`, starting with
/// a letter (`x` is prefixed when the first word starts with a digit and is the
/// whole segment when the component has no word).
fn id_segment(component: &str) -> String {
    let joined = words(component).join("-");
    match joined.chars().next() {
        Some(c) if c.is_ascii_lowercase() => joined,
        Some(_) => format!("x{joined}"),
        None => "x".to_owned(),
    }
}

/// The base entry ID of a Lean name: one segment per component, joined by
/// `.`, so the namespace structure of Lean is the directory structure of the
/// package (`Nat.add_comm` is `nat.add-comm`, stored at
/// `entries/nat/add-comm.toml`).
#[must_use]
pub fn base_entry_id(lean_name: &str) -> String {
    lean_name
        .split('.')
        .map(id_segment)
        .collect::<Vec<_>>()
        .join(".")
}

/// The canonical surface of a Lean name: the words of every component joined
/// in lower camel case (`Nat.add_comm` is `natAddComm`). The result is one
/// math word that begins with a letter, which is renderer-safe (§13.5) and a
/// valid `operator-name` (§13.9).
#[must_use]
pub fn camel_surface(lean_name: &str) -> String {
    let mut out = String::new();
    for word in lean_name.split('.').flat_map(words) {
        if out.is_empty() {
            out.push_str(&word);
        } else {
            let mut letters = word.chars();
            if let Some(first) = letters.next() {
                out.push(first.to_ascii_uppercase());
                out.push_str(letters.as_str());
            }
        }
    }
    match out.chars().next() {
        Some(c) if c.is_ascii_alphabetic() => out,
        Some(_) => format!("x{out}"),
        None => "x".to_owned(),
    }
}

/// The prose words of a Lean name for an adjective or transitive predicate:
/// the words of its last component, lowercase, with a leading `is` dropped
/// when other words remain (`Nat.Prime` is `prime`, `IsUnit` is `unit`).
/// `None` when a word holds a digit, because a prose form cannot (§13.5
/// rule 4), or when nothing is left.
#[must_use]
pub fn prose_words(lean_name: &str) -> Option<String> {
    let component = lean_name.rsplit('.').next()?;
    let mut all = words(component);
    if all.len() > 1 && all.first().is_some_and(|w| w == "is") {
        all.remove(0);
    }
    if all.is_empty()
        || all
            .iter()
            .any(|w| !w.chars().all(|c| c.is_ascii_lowercase()))
    {
        return None;
    }
    Some(all.join(" "))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn words_split_at_every_boundary_kind() {
        assert_eq!(words("add_comm"), ["add", "comm"]);
        assert_eq!(words("addComm"), ["add", "comm"]);
        assert_eq!(words("HAdd"), ["h", "add"]);
        assert_eq!(words("ENNReal"), ["enn", "real"]);
        assert_eq!(words("pow2"), ["pow2"]);
        assert_eq!(words("pow_2"), ["pow", "2"]);
        assert_eq!(words("add_comm'"), ["add", "comm", "prime"]);
        assert_eq!(words("IsUnit"), ["is", "unit"]);
        assert!(words("_").is_empty());
    }

    #[test]
    fn entry_ids_follow_the_namespace() {
        assert_eq!(base_entry_id("Nat.add_comm"), "nat.add-comm");
        assert_eq!(base_entry_id("List.append_assoc"), "list.append-assoc");
        assert_eq!(base_entry_id("Nat"), "nat");
        assert_eq!(base_entry_id("Nat.pow_2"), "nat.pow-2");
        assert_eq!(base_entry_id("Foo._1"), "foo.x1");
        assert_eq!(base_entry_id("Foo._"), "foo.x");
        for name in ["Nat.add_comm", "HAdd.hAdd", "Nat.le.dest", "a'.b"] {
            assert!(
                crate::lexicon::lse::is_entry_id(&base_entry_id(name)),
                "{name}"
            );
        }
    }

    #[test]
    fn surfaces_are_one_safe_word() {
        assert_eq!(camel_surface("Nat.add_comm"), "natAddComm");
        assert_eq!(camel_surface("HAdd.hAdd"), "hAddHAdd");
        assert_eq!(camel_surface("Nat"), "nat");
        assert_eq!(camel_surface("Nat.pow_2"), "natPow2");
        assert_eq!(camel_surface("add_comm'"), "addCommPrime");
        // The two spellings of one name collide, by design: the surface is a
        // reading aid and `\lexeme` selects between the entries.
        assert_eq!(camel_surface("Nat.add_comm"), camel_surface("Nat.addComm"));
    }

    #[test]
    fn prose_needs_letters_only() {
        assert_eq!(prose_words("Nat.Prime").as_deref(), Some("prime"));
        assert_eq!(prose_words("IsUnit").as_deref(), Some("unit"));
        assert_eq!(prose_words("Dvd.dvd").as_deref(), Some("dvd"));
        assert_eq!(prose_words("Nat.lt_irrefl").as_deref(), Some("lt irrefl"));
        assert_eq!(prose_words("Fin2"), None);
        assert_eq!(prose_words("Is"), Some("is".to_owned()));
    }
}
