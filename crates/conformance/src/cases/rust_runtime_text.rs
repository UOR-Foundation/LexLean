//! The runtime text is printed, never held (SPEC.md §17.17): `RB-21`.
//!
//! The Rust runtime every package carries is the printed crate of four
//! language-1.2 modules. This case holds the `lexlean` crate to it: no string
//! literal of its sources holds the text of a runtime item, however it is
//! spelled, so that no handwritten runtime can come back as a constant beside
//! the printer. The scan is planted: a constant that holds an item, added to
//! a copy of the shipped sources, is found.

use std::path::{Path, PathBuf};

use crate::support::repo_root;

/// The string literals of the Rust source `text`, each with the line it starts
/// on. Comments, character literals, and lifetimes are skipped; the escapes of
/// a string are read as the compiler reads them where they bear on the shape
/// of the text (a newline, a quote, a backslash, a continuation).
fn literals(text: &str) -> Vec<(usize, String)> {
    let chars: Vec<char> = text.chars().collect();
    let mut out = Vec::new();
    let (mut at, mut line) = (0, 1);
    while at < chars.len() {
        let c = chars[at];
        match c {
            '\n' => {
                line += 1;
                at += 1;
            }
            '/' if chars.get(at + 1) == Some(&'/') => {
                while at < chars.len() && chars[at] != '\n' {
                    at += 1;
                }
            }
            '/' if chars.get(at + 1) == Some(&'*') => {
                let mut depth = 0;
                while at < chars.len() {
                    if chars[at] == '/' && chars.get(at + 1) == Some(&'*') {
                        depth += 1;
                        at += 2;
                    } else if chars[at] == '*' && chars.get(at + 1) == Some(&'/') {
                        depth -= 1;
                        at += 2;
                        if depth == 0 {
                            break;
                        }
                    } else {
                        if chars[at] == '\n' {
                            line += 1;
                        }
                        at += 1;
                    }
                }
            }
            'r' if matches!(chars.get(at + 1), Some('"' | '#'))
                && (at == 0 || !(chars[at - 1].is_alphanumeric() || chars[at - 1] == '_')) =>
            {
                let mut hashes = 0;
                let mut cursor = at + 1;
                while chars.get(cursor) == Some(&'#') {
                    hashes += 1;
                    cursor += 1;
                }
                if chars.get(cursor) != Some(&'"') {
                    at += 1;
                    continue;
                }
                cursor += 1;
                let start_line = line;
                let mut body = String::new();
                loop {
                    let Some(&d) = chars.get(cursor) else { break };
                    if d == '"' && (0..hashes).all(|i| chars.get(cursor + 1 + i) == Some(&'#')) {
                        cursor += 1 + hashes;
                        break;
                    }
                    if d == '\n' {
                        line += 1;
                    }
                    body.push(d);
                    cursor += 1;
                }
                out.push((start_line, body));
                at = cursor;
            }
            '"' => {
                let start_line = line;
                let mut body = String::new();
                at += 1;
                while let Some(&d) = chars.get(at) {
                    at += 1;
                    match d {
                        '"' => break,
                        '\\' => {
                            let e = chars.get(at).copied().unwrap_or('\\');
                            at += 1;
                            match e {
                                'n' => body.push('\n'),
                                '"' => body.push('"'),
                                '\\' => body.push('\\'),
                                '\n' => {
                                    line += 1;
                                    while chars.get(at).is_some_and(|w| w.is_whitespace()) {
                                        if chars[at] == '\n' {
                                            line += 1;
                                        }
                                        at += 1;
                                    }
                                }
                                other => {
                                    body.push('\\');
                                    body.push(other);
                                }
                            }
                        }
                        '\n' => {
                            line += 1;
                            body.push('\n');
                        }
                        other => body.push(other),
                    }
                }
                out.push((start_line, body));
            }
            '\'' => {
                // A character literal, or a lifetime.
                if chars.get(at + 1) == Some(&'\\') {
                    at += 2;
                    while at < chars.len() && chars[at] != '\'' {
                        at += 1;
                    }
                    at += 1;
                } else if chars.get(at + 2) == Some(&'\'') {
                    at += 3;
                } else {
                    at += 1;
                }
            }
            _ => at += 1,
        }
    }
    out
}

/// Whether `rest` starts with a name followed by one of `after`.
fn named_then(rest: &str, after: &[char]) -> bool {
    let name: String = rest
        .chars()
        .take_while(|c| c.is_alphanumeric() || *c == '_')
        .collect();
    !name.is_empty()
        && !name.starts_with(|c: char| c.is_ascii_digit())
        && rest[name.len()..]
            .chars()
            .next()
            .is_some_and(|c| after.contains(&c))
}

/// The first line of `literal` that is the head of a runtime item as the
/// printer writes it: a function, a struct, an enum, a trait, an impl, or a
/// macro definition, with a name and not a placeholder in the name's place,
/// and not the head of a format string.
fn item_head(literal: &str) -> Option<String> {
    literal.lines().map(str::trim_start).find_map(|line| {
        let line = line.strip_prefix("pub ").unwrap_or(line);
        let held = if let Some(rest) = line.strip_prefix("fn ") {
            named_then(rest, &['(', '<'])
        } else if let Some(rest) = line
            .strip_prefix("struct ")
            .or_else(|| line.strip_prefix("enum "))
            .or_else(|| line.strip_prefix("trait "))
        {
            named_then(rest, &[' ', '(', '<', '{', ';'])
        } else if let Some(rest) = line.strip_prefix("macro_rules! ") {
            named_then(rest, &[' ', '{'])
        } else if let Some(rest) = line.strip_prefix("impl") {
            let rest = rest.trim_start();
            let rest = rest.strip_prefix('<').map_or(rest, |generics| {
                generics
                    .split_once('>')
                    .map_or(generics, |(_, after)| after.trim_start())
            });
            named_then(rest, &[' ', '<', '{'])
        } else {
            false
        };
        // A head with doubled braces is the head of a format string, a
        // printer's template with substitutions, not text that is a runtime
        // item as it stands.
        (held && !line.contains("{{")).then(|| line.trim_end().to_owned())
    })
}

/// Every `.rs` file under `dir`.
fn sources(dir: &Path, out: &mut Vec<PathBuf>) {
    let mut entries: Vec<_> = std::fs::read_dir(dir)
        .unwrap_or_else(|error| panic!("{}: {error}", dir.display()))
        .map(|entry| entry.expect("an entry").path())
        .collect();
    entries.sort();
    for path in entries {
        if path.is_dir() {
            sources(&path, out);
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            out.push(path);
        }
    }
}

/// The literals of the sources of `texts` (a path with its text) that hold a
/// runtime item, each with its path, line, and the head found; and the
/// number of literals examined.
fn scan(texts: &[(String, String)]) -> (Vec<String>, usize) {
    let mut found = Vec::new();
    let mut examined = 0;
    for (path, text) in texts {
        for (line, literal) in literals(text) {
            examined += 1;
            if let Some(head) = item_head(&literal) {
                found.push(format!("{path}:{line}: `{head}`"));
            }
        }
    }
    (found, examined)
}

/// Run the case of `RB-21`.
pub fn run(id: &str) {
    assert_eq!(id, "RB-21");
    let root = repo_root();
    let mut paths = Vec::new();
    sources(root.join("crates/lexlean/src").as_std_path(), &mut paths);
    let texts: Vec<(String, String)> = paths
        .iter()
        .map(|path| {
            let shown = path
                .strip_prefix(root.as_std_path())
                .expect("under the root")
                .to_string_lossy()
                .into_owned();
            let text = std::fs::read_to_string(path)
                .unwrap_or_else(|error| panic!("{}: {error}", path.display()));
            (shown, text)
        })
        .collect();

    // A scan that reads nothing finds nothing: it must have read the sources
    // that print the runtime, and strings in them.
    for needed in [
        "crates/lexlean/src/calculus/rust/runtime.rs",
        "crates/lexlean/src/calculus/rust/rt.rs",
        "crates/lexlean/src/calculus/rust/ast.rs",
    ] {
        assert!(
            texts.iter().any(|(path, _)| path == needed),
            "the scan reads `{needed}`"
        );
    }
    let (found, examined) = scan(&texts);
    assert!(
        examined > 1000,
        "the scan examines the string literals of the sources, not {examined}"
    );
    assert!(
        found.is_empty(),
        "the sources hold the text of a runtime item:\n{}",
        found.join("\n")
    );

    // The handwritten runtime that was kept as an oracle is gone; what holds
    // the behavior of the printed runtime is the golden transcripts.
    assert!(
        !root
            .join("tests/rust-runtime-oracle")
            .as_std_path()
            .exists(),
        "tests/rust-runtime-oracle is gone"
    );
    for (file, least) in [("core", 100), ("support", 4), ("std", 30)] {
        let path = format!("tests/rust-runtime-golden/{file}.txt");
        let text = std::fs::read_to_string(root.join(&path).as_std_path())
            .unwrap_or_else(|error| panic!("{path}: {error}"));
        let lines: Vec<&str> = text.lines().collect();
        assert!(lines.len() >= least, "{path} has {} lines", lines.len());
        for line in lines {
            let (label, digest) = line.rsplit_once(' ').expect("a label and a digest");
            assert!(
                !label.is_empty()
                    && digest.len() == 16
                    && digest.chars().all(|c| c.is_ascii_hexdigit()),
                "{path}: `{line}` is a label and a 64-bit digest"
            );
        }
    }

    // The scan is planted: each way a handwritten runtime could come back as
    // a constant is found in a copy of the sources that is otherwise clean.
    let target = "crates/lexlean/src/calculus/rust/runtime.rs";
    let at = texts
        .iter()
        .position(|(path, _)| path == target)
        .expect("runtime.rs");
    for (what, constant) in [
        (
            "a constant that holds a function",
            "pub const TICK: &str = \"pub fn tick(units: u64) { WORK.fetch_add(units, Memory::Relaxed); }\";\n",
        ),
        (
            "a raw string that holds an impl",
            "const SAME: &str = r#\"\nimpl Same for Str {\n    fn same(&self, other: &Self) -> bool { true }\n}\n\"#;\n",
        ),
        (
            "a constant that holds a macro definition",
            "const DECIMAL: &str = \"macro_rules! decimal { ($t:ident) => {}; }\";\n",
        ),
        (
            "a constant that holds a type",
            "const NODE: &str = \"\\n#[derive(Clone)]\\npub struct Node<T> { head: T }\\n\";\n",
        ),
        (
            "a generic function on a line of a longer text",
            "const MORE: &str = \"use std::rc::Rc;\\nfn walk<T>(items: &[T]) {}\\n\";\n",
        ),
    ] {
        let mut planted = texts.clone();
        planted[at].1.push_str(constant);
        let (found, _) = scan(&planted);
        assert_eq!(found.len(), 1, "{what}: {found:?}");
        assert!(found[0].starts_with(target), "{what}: {found:?}");
    }

    // The printers' own templates are not runtime text: a head with a
    // placeholder where the name stands, and text that only mentions a
    // keyword, are not found.
    let mut clean = texts.clone();
    clean[at].1.push_str(
        "const T: &str = \"pub fn {name}({params}) -> {result} {{ {body} }}\";\nconst U: &str = \"the impl of a trait; fn is a keyword\";\n",
    );
    assert!(scan(&clean).0.is_empty(), "templates are not runtime text");
}
