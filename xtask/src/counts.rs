//! `cargo xtask counts [--write]`: carry the counts a bootstrap case reported
//! under `LEXLEAN_COUNTS` into the constants and statements that state them
//! (SPEC.md §17.23, the §31 row, the register, the scenario).
//!
//! A count is rewritten only where the rewrite is unambiguous: the constant's
//! literal must hold the reported value; a number in a statement is rewritten
//! when no other constant of the case holds the same value, or when it is the
//! sum of exactly one pair of the case's counts; prose of §17.23 is rewritten
//! where the number is followed by the same words as in the case's row.
//! Everything else is listed for a reader, never guessed.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::path::{Path, PathBuf};

use repo_conformance::counts::{read_reports, Recorded};

use crate::Fail;

/// A number in a line of prose.
struct Token {
    start: usize,
    end: usize,
    value: usize,
    commas: bool,
}

/// Whether `bytes[at..]` is a comma group: `,` and three digits, no fourth.
fn comma_group(bytes: &[u8], at: usize) -> bool {
    bytes.get(at) == Some(&b',')
        && at + 4 <= bytes.len()
        && bytes[at + 1..at + 4].iter().all(u8::is_ascii_digit)
        && !bytes.get(at + 4).is_some_and(u8::is_ascii_digit)
}

/// The numbers of `line`: digit runs, with comma groups of three, that are not
/// part of a name, an ID, a section number, or a decimal.
fn tokens(line: &str) -> Vec<Token> {
    let bytes = line.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        if !bytes[i].is_ascii_digit() {
            i += 1;
            continue;
        }
        let start = i;
        let mut end = i;
        while end < bytes.len() && bytes[end].is_ascii_digit() {
            end += 1;
        }
        let mut commas = false;
        if end - start <= 3 {
            while comma_group(bytes, end) {
                commas = true;
                end += 4;
            }
        }
        i = end;
        let before = line[..start].chars().next_back();
        let named = before.is_some_and(|c| c.is_alphanumeric() || "_-.§".contains(c));
        let decimal =
            bytes.get(end) == Some(&b'.') && bytes.get(end + 1).is_some_and(u8::is_ascii_digit);
        let joined = bytes
            .get(end)
            .is_some_and(|b| b.is_ascii_alphanumeric() || *b == b'_');
        if named || decimal || joined {
            continue;
        }
        let digits: String = line[start..end]
            .chars()
            .filter(char::is_ascii_digit)
            .collect();
        if let Ok(value) = digits.parse() {
            out.push(Token {
                start,
                end,
                value,
                commas,
            });
        }
    }
    out
}

fn grouped(digits: &str, separator: char) -> String {
    let mut out = String::new();
    for (index, digit) in digits.chars().enumerate() {
        if index > 0 && (digits.len() - index).is_multiple_of(3) {
            out.push(separator);
        }
        out.push(digit);
    }
    out
}

/// `value` with comma groups of three when `commas`.
fn render(value: usize, commas: bool) -> String {
    let digits = value.to_string();
    if commas && digits.len() > 3 {
        grouped(&digits, ',')
    } else {
        digits
    }
}

/// A Rust literal for `value`: grouped by `_` when the old literal was, or
/// when it has five digits or more.
fn rust_literal(value: usize, underscores: bool) -> String {
    let digits = value.to_string();
    if underscores || digits.len() >= 5 {
        grouped(&digits, '_')
    } else {
        digits
    }
}

/// `line` with every number in `map` replaced at once, in the number's own
/// style (a number under 1,000 that grows past it takes commas when the line
/// writes its other thousands with them), and how often each was replaced.
fn rewrite(line: &str, map: &BTreeMap<usize, usize>) -> (String, BTreeMap<usize, usize>) {
    let found = tokens(line);
    let line_commas = found.iter().any(|token| token.commas);
    let mut out = String::new();
    let mut applied = BTreeMap::new();
    let mut last = 0;
    for token in found {
        if let Some(new) = map.get(&token.value) {
            out.push_str(&line[last..token.start]);
            let commas = token.commas || (token.value < 1_000 && line_commas);
            out.push_str(&render(*new, commas));
            last = token.end;
            *applied.entry(token.value).or_insert(0) += 1;
        }
    }
    out.push_str(&line[last..]);
    (out, applied)
}

/// The text after byte `end` of `line`: at most three words, and never past a
/// table cell's end.
fn following(line: &str, end: usize) -> String {
    let rest = &line[end..];
    let mut words = 0;
    let mut cut = rest.len();
    let mut in_word = false;
    for (index, c) in rest.char_indices() {
        if c == '|' {
            cut = index;
            break;
        }
        if c.is_whitespace() {
            if in_word {
                words += 1;
                in_word = false;
                if words == 3 {
                    cut = index;
                    break;
                }
            }
        } else {
            in_word = true;
        }
    }
    rest[..cut].to_owned()
}

/// One constant whose count moved.
#[derive(Debug, Clone)]
struct Moved {
    id: String,
    file: String,
    name: String,
    old: usize,
    new: usize,
}

/// A constant of a case's source: its value, its line, and its literal.
type Constant = (usize, usize, String);

/// The `usize` constants of a case's source, by name.
fn constants(source: &str) -> BTreeMap<String, Constant> {
    let mut out = BTreeMap::new();
    for (index, line) in source.lines().enumerate() {
        let Some(rest) = line.trim_start().strip_prefix("const ") else {
            continue;
        };
        let Some((name, rest)) = rest.split_once(": usize = ") else {
            continue;
        };
        let Some(literal) = rest.strip_suffix(';') else {
            continue;
        };
        if let Ok(value) = literal.replace('_', "").parse() {
            out.insert(name.to_owned(), (value, index, literal.to_owned()));
        }
    }
    out
}

/// The lines of `text` joined back, keeping a final newline.
fn joined(lines: &[String], text: &str) -> String {
    let mut out = lines.join("\n");
    if text.ends_with('\n') {
        out.push('\n');
    }
    out
}

/// Every count of the report, by its case, file, and constant: the values the
/// constant held and the values observed.
type Seen = BTreeMap<(String, String, String), (BTreeSet<usize>, BTreeSet<usize>)>;

fn seen_of(records: &[Recorded]) -> Seen {
    let mut seen = Seen::new();
    for record in records {
        let entry = seen
            .entry((record.id.clone(), record.file.clone(), record.name.clone()))
            .or_default();
        entry.0.insert(record.expected);
        entry.1.insert(record.observed);
    }
    seen
}

/// The single value of a set the caller knows has one.
fn only(set: &BTreeSet<usize>) -> usize {
    *set.first().expect("a set of one value")
}

/// A case's numbers to rewrite, and each with the words that follow it in
/// the case's §31 row.
type Anchored = (String, BTreeMap<usize, usize>, Vec<(usize, String)>);

/// The numbers of a case to rewrite, and the words that follow each in its row.
type Numbers = (BTreeMap<usize, usize>, Vec<(usize, String)>);

/// The files a write touches, read once.
struct Texts {
    sources: BTreeMap<String, String>,
    spec: String,
    ids: String,
    features: BTreeMap<PathBuf, String>,
}

/// Run the task.
///
/// # Errors
///
/// When there is no report, or a file cannot be read or written.
pub fn counts(root: &Path, write: bool) -> Result<(), Fail> {
    let dir = root.join("target/count-report");
    let records = read_reports(&dir).map_err(|error| {
        format!(
            "counts: no report ({error}); run the bootstrap parts under LEXLEAN_COUNTS=fast first"
        )
    })?;
    if records.is_empty() {
        return Err("counts: the report is empty".into());
    }
    let seen = seen_of(&records);
    let mut report = String::new();
    let mut flagged: Vec<String> = Vec::new();
    let mut texts = Texts {
        sources: BTreeMap::new(),
        spec: std::fs::read_to_string(root.join("SPEC.md"))?,
        ids: std::fs::read_to_string(root.join("model/ids.toml"))?,
        features: BTreeMap::new(),
    };
    for entry in std::fs::read_dir(root.join("features/suites"))? {
        let path = entry?.path();
        if path.extension().is_some_and(|ext| ext == "feature") {
            let text = std::fs::read_to_string(&path)?;
            texts.features.insert(path, text);
        }
    }
    let moved = moved_counts(root, &seen, &mut texts, &mut report, &mut flagged)?;
    if moved.is_empty() {
        let _ = writeln!(report, "counts: no count moved");
    }
    let originals = texts.sources.clone();
    for item in &moved {
        write_constant(item, &mut texts, &mut report);
    }
    let mut verification = String::new();
    let mut anchored: Vec<Anchored> = Vec::new();
    let cases: BTreeSet<&str> = moved.iter().map(|item| item.id.as_str()).collect();
    for id in cases {
        let mine: Vec<&Moved> = moved.iter().filter(|item| item.id == id).collect();
        if let Some((map, contexts)) = statements(
            id,
            &mine,
            &constants(&originals[&mine[0].file]),
            &seen,
            &mut texts,
            &mut report,
            &mut flagged,
        ) {
            anchored.push((id.to_owned(), map, contexts));
        }
        let names: Vec<&str> = mine.iter().map(|item| item.name.as_str()).collect();
        let olds: Vec<String> = mine.iter().map(|item| render(item.old, true)).collect();
        let news: Vec<String> = mine.iter().map(|item| render(item.new, true)).collect();
        let _ = writeln!(
            verification,
            "| `{id}` {} | {} | {} |",
            names.join(", "),
            olds.join(", "),
            news.join(", ")
        );
    }
    if !moved.is_empty() {
        let derived: Vec<(usize, usize)> = anchored
            .iter()
            .flat_map(|(_, map, _)| map.iter().map(|(old, new)| (*old, *new)))
            .filter(|(old, _)| !moved.iter().any(|item| item.old == *old))
            .collect();
        let shared = shared_map(root, &moved, &derived)?;
        prose(&anchored, &shared, &mut texts, &mut report, &mut flagged);
    }
    print!("{report}");
    if !verification.is_empty() {
        println!(
            "\nVERIFICATION.md rows for the landing (counts | before | after):\n{verification}"
        );
    }
    if !flagged.is_empty() {
        println!("For a reader:");
        for line in &flagged {
            println!("  {line}");
        }
    }
    if moved.is_empty() {
        return Ok(());
    }
    if write {
        for (file, text) in &texts.sources {
            if originals.get(file) != Some(text) {
                std::fs::write(root.join(file), text)?;
            }
        }
        // SPEC.md and model/ids.toml are projections of the modules of
        // `spec/` (NS-03): the moved counts are written into the register rows
        // and the chapter items they were projected from, then projected.
        write_sources(root, &texts)?;
        for (path, text) in &texts.features {
            std::fs::write(path, text)?;
        }
        println!(
            "\nwritten, with SPEC.md and the model projected again; next: the parts without LEXLEAN_COUNTS"
        );
    } else {
        println!("\n(dry run: --write applies it)");
    }
    Ok(())
}

/// The counts that moved and whose constant still holds the old value.
fn moved_counts(
    root: &Path,
    seen: &Seen,
    texts: &mut Texts,
    report: &mut String,
    flagged: &mut Vec<String>,
) -> Result<Vec<Moved>, Fail> {
    let mut moved = Vec::new();
    for ((id, file, name), (expected, observed)) in seen {
        if !texts.sources.contains_key(file) {
            let source = std::fs::read_to_string(root.join(file))
                .map_err(|error| format!("counts: {file}: {error}"))?;
            texts.sources.insert(file.clone(), source);
        }
        if expected.len() != 1 || observed.len() != 1 {
            flagged.push(format!(
                "{id} {name}: the report holds {expected:?} and observes {observed:?}; the checks of one constant disagree, which is no count to write"
            ));
            continue;
        }
        let (old, new) = (only(expected), only(observed));
        if old == new {
            continue;
        }
        match constants(&texts.sources[file]).get(name).map(|constant| constant.0) {
            Some(value) if value == new => {
                let _ = writeln!(report, "{id} {name}: already {new}");
            }
            Some(value) if value == old => moved.push(Moved {
                id: id.clone(),
                file: file.clone(),
                name: name.clone(),
                old,
                new,
            }),
            Some(value) => flagged.push(format!(
                "{id} {name}: the source holds {value} and the report {old} -> {new}; rerun the case"
            )),
            None => flagged.push(format!(
                "{id} {name}: {file} has no `const {name}: usize = <literal>;`; write {old} -> {new} by hand"
            )),
        }
    }
    Ok(moved)
}

/// Rewrite the constant's literal in its source.
fn write_constant(item: &Moved, texts: &mut Texts, report: &mut String) {
    let source = &texts.sources[&item.file];
    let (_, index, literal) = constants(source)[&item.name].clone();
    let new_literal = rust_literal(item.new, literal.contains('_'));
    let lines: Vec<String> = source
        .lines()
        .enumerate()
        .map(|(at, line)| {
            if at == index {
                line.replacen(&format!("= {literal};"), &format!("= {new_literal};"), 1)
            } else {
                line.to_owned()
            }
        })
        .collect();
    let text = joined(&lines, source);
    texts.sources.insert(item.file.clone(), text);
    let _ = writeln!(
        report,
        "{} {}: {} -> {} ({})",
        item.id, item.name, item.old, item.new, item.file
    );
}

/// The numbers of a case's statements to rewrite: each moved count whose old
/// value no other constant of the case holds, and each sum of one pair of its
/// counts that the row states.
fn number_map(
    id: &str,
    mine: &[&Moved],
    table: &BTreeMap<String, Constant>,
    seen: &Seen,
    row: &str,
    report: &mut String,
    flagged: &mut Vec<String>,
) -> BTreeMap<usize, usize> {
    let mut map = BTreeMap::new();
    for item in mine {
        let others: Vec<&String> = table
            .iter()
            .filter(|(name, (value, _, _))| {
                *name != &item.name
                    && *value == item.old
                    && !mine
                        .iter()
                        .any(|other| &other.name == *name && other.new == item.new)
            })
            .map(|(name, _)| name)
            .collect();
        if others.is_empty() {
            map.insert(item.old, item.new);
        } else {
            flagged.push(format!(
                "{id} {}: {} is also the value of {others:?}; rewrite its statements by hand ({} -> {})",
                item.name, item.old, item.old, item.new
            ));
        }
    }
    let counted: Vec<(&String, usize, usize)> = seen
        .iter()
        .filter(|((case, _, _), (expected, observed))| {
            case == id && expected.len() == 1 && observed.len() == 1
        })
        .map(|((_, _, name), (expected, observed))| (name, only(expected), only(observed)))
        .collect();
    let known: BTreeSet<usize> = table.values().map(|constant| constant.0).collect();
    for token in tokens(row) {
        if known.contains(&token.value) || map.contains_key(&token.value) {
            continue;
        }
        let mut sums = BTreeSet::new();
        let mut pairs = Vec::new();
        for (i, (a, a_old, a_new)) in counted.iter().enumerate() {
            for (b, b_old, b_new) in &counted[i + 1..] {
                // An unchanged explanation can conflict with a moved pair.
                if a_old + b_old == token.value {
                    sums.insert(a_new + b_new);
                    pairs.push(format!("{a} + {b}"));
                }
            }
        }
        match sums.len() {
            0 => {}
            1 => {
                let new = only(&sums);
                if new == token.value {
                    continue;
                }
                let _ = writeln!(
                    report,
                    "{id}: {} is {}: -> {new}",
                    token.value,
                    pairs.join(" or ")
                );
                map.insert(token.value, new);
            }
            _ => flagged.push(format!(
                "{id}: {} is the sum of {}, which move differently; rewrite it by hand",
                token.value,
                pairs.join(", ")
            )),
        }
    }
    map
}

/// Rewrite the numbers of the case's §31 row, register statement, and
/// scenario; return the numbers and the words that follow each in the row.
fn statements(
    id: &str,
    mine: &[&Moved],
    table: &BTreeMap<String, Constant>,
    seen: &Seen,
    texts: &mut Texts,
    report: &mut String,
    flagged: &mut Vec<String>,
) -> Option<Numbers> {
    let row_prefix = format!("| `{id}` |");
    let Some(row) = texts
        .spec
        .lines()
        .find(|line| line.starts_with(&row_prefix))
        .map(str::to_owned)
    else {
        flagged.push(format!("{id}: SPEC.md has no §31 row"));
        return None;
    };
    let map = number_map(id, mine, table, seen, &row, report, flagged);
    let contexts: Vec<(usize, String)> = tokens(&row)
        .iter()
        .filter(|token| map.contains_key(&token.value))
        .map(|token| (token.value, collapsed(&following(&row, token.end))))
        .filter(|(_, words)| words.trim().len() >= 4)
        .collect();
    let (new_row, applied) = rewrite(&row, &map);
    let _ = writeln!(report, "{id}: §31 row: {}", applied_text(&applied, &map));
    for (old, new) in &map {
        if !applied.contains_key(old) {
            let _ = writeln!(report, "{id}: the §31 row does not state {old} (-> {new})");
        }
    }
    let mut lines: Vec<String> = texts.spec.lines().map(str::to_owned).collect();
    if let Some(at) = lines.iter().position(|line| line == &row) {
        lines[at] = new_row;
    }
    texts.spec = joined(&lines, &texts.spec);

    let marker = format!("id = \"{id}\"");
    let mut lines: Vec<String> = texts.ids.lines().map(str::to_owned).collect();
    let statement = lines
        .iter()
        .position(|line| line == &marker)
        .and_then(|at| {
            lines[at..]
                .iter()
                .position(|line| line.starts_with("statement = "))
                .map(|offset| at + offset)
        });
    if let Some(at) = statement {
        let (line, applied) = rewrite(&lines[at], &map);
        lines[at] = line;
        let _ = writeln!(report, "{id}: register: {}", applied_text(&applied, &map));
        texts.ids = joined(&lines, &texts.ids);
    } else {
        flagged.push(format!("{id}: model/ids.toml has no statement"));
    }

    let tag = format!("@{id} ");
    for text in texts.features.values_mut() {
        let mut lines: Vec<String> = text.lines().map(str::to_owned).collect();
        let Some(at) = lines
            .iter()
            .position(|line| line.trim_start().starts_with(&tag))
        else {
            continue;
        };
        if let Some(next) = lines.get(at + 1) {
            let (line, applied) = rewrite(next, &map);
            lines[at + 1] = line;
            let _ = writeln!(report, "{id}: scenario: {}", applied_text(&applied, &map));
        }
        *text = joined(&lines, text);
    }
    Some((map, contexts))
}

/// `text` with every run of whitespace one space.
fn collapsed(text: &str) -> String {
    let mut out = String::new();
    let mut space = false;
    for c in text.chars() {
        if c.is_whitespace() {
            space = true;
        } else {
            if space {
                out.push(' ');
                space = false;
            }
            out.push(c);
        }
    }
    if space {
        out.push(' ');
    }
    out
}

/// The numbers every constant holding them moves alike: each moved old
/// value that no constant of any bootstrap case holds unless it moves to the
/// same new value, and each sum of counts (`derived`) that no constant holds.
/// Prose of §17.23 states such a number without naming a case (the committed
/// manifests every case reads, say).
fn shared_map(
    root: &Path,
    moved: &[Moved],
    derived: &[(usize, usize)],
) -> Result<BTreeMap<usize, usize>, Fail> {
    let mut holders: Vec<(String, String, usize)> = Vec::new();
    let dir = root.join("crates/conformance/src/cases/bootstrap");
    let mut paths: Vec<PathBuf> = std::fs::read_dir(&dir)?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .collect();
    paths.push(root.join("crates/conformance/src/cases/bootstrap.rs"));
    for path in paths {
        if path.extension().is_none_or(|ext| ext != "rs") {
            continue;
        }
        let file = path
            .strip_prefix(root)
            .unwrap_or(&path)
            .to_string_lossy()
            .replace('\\', "/");
        let source = std::fs::read_to_string(&path)?;
        for (name, (value, _, _)) in constants(&source) {
            holders.push((file.clone(), name, value));
        }
    }
    let mut out = BTreeMap::new();
    let olds: BTreeSet<usize> = moved
        .iter()
        .map(|item| item.old)
        .chain(derived.iter().map(|(old, _)| *old))
        .collect();
    for old in olds {
        let news: BTreeSet<usize> = moved
            .iter()
            .filter(|item| item.old == old)
            .map(|item| item.new)
            .chain(
                derived
                    .iter()
                    .filter(|(from, _)| *from == old)
                    .map(|(_, new)| *new),
            )
            .collect();
        let still = holders.iter().any(|(file, name, value)| {
            *value == old
                && !moved
                    .iter()
                    .any(|item| &item.file == file && &item.name == name)
        });
        if news.len() == 1 && !still {
            out.insert(old, only(&news));
        }
    }
    Ok(out)
}

/// `old -> new (times)` for each replaced number.
fn applied_text(applied: &BTreeMap<usize, usize>, map: &BTreeMap<usize, usize>) -> String {
    if applied.is_empty() {
        return "nothing".to_owned();
    }
    applied
        .iter()
        .map(|(old, times)| format!("{old} -> {} (x{times})", map[old]))
        .collect::<Vec<_>>()
        .join(", ")
}

/// The text after byte `end` of line `at`, running on over the next lines of
/// its paragraph, with whitespace collapsed: enough to compare with a row's
/// words across the prose's line breaks.
fn after(lines: &[String], at: usize, end: usize) -> String {
    let mut text = lines[at][end..].to_owned();
    for line in lines.iter().skip(at + 1).take(3) {
        if line.trim().is_empty() {
            break;
        }
        text.push(' ');
        text.push_str(line);
    }
    collapsed(&text)
}

/// Rewrite the numbers of the prose of §17.23: each a case's row states that
/// the row's words follow there too, and each every holder moves alike
/// (`shared`); then list the moved numbers it still states. Matches are taken
/// on the prose as it was, then applied at once.
fn prose(
    anchored: &[Anchored],
    shared: &BTreeMap<usize, usize>,
    texts: &mut Texts,
    report: &mut String,
    flagged: &mut Vec<String>,
) {
    let mut lines: Vec<String> = texts.spec.lines().map(str::to_owned).collect();
    let Some(start) = lines.iter().position(|line| line.starts_with("### 17.23 ")) else {
        flagged.push("SPEC.md has no §17.23".to_owned());
        return;
    };
    let end = lines[start + 1..]
        .iter()
        .position(|line| line.starts_with("### ") || line.starts_with("## "))
        .map_or(lines.len(), |offset| start + 1 + offset);
    let mut edits: Vec<(usize, usize, usize, String, String)> = Vec::new();
    for at in start..end {
        if lines[at].starts_with('|') {
            continue;
        }
        for token in tokens(&lines[at]) {
            let words = after(&lines, at, token.end);
            let mut by: Option<(usize, String)> = None;
            let mut destinations = BTreeSet::new();
            for (id, map, contexts) in anchored {
                if contexts.iter().any(|(old, context)| {
                    *old == token.value && words.starts_with(context.as_str())
                }) {
                    let new = map[&token.value];
                    destinations.insert(new);
                    by.get_or_insert_with(|| (new, id.clone()));
                }
            }
            // A known anchor conflict must not use the shared-number fallback.
            if destinations.len() > 1 {
                flagged.push(format!(
                    "SPEC.md:{}: {} has matching row contexts with conflicting destinations {destinations:?}; rewrite it by hand",
                    at + 1,
                    token.value
                ));
                continue;
            }
            if by.is_none() {
                if let Some(new) = shared.get(&token.value) {
                    by = Some((*new, "every holder".to_owned()));
                }
            }
            if let Some((new, why)) = by {
                edits.push((at, token.start, token.end, render(new, token.commas), why));
            }
        }
    }
    for (at, start, end, new, why) in edits.iter().rev() {
        let old = lines[*at][*start..*end].to_owned();
        lines[*at].replace_range(*start..*end, new);
        let _ = writeln!(report, "SPEC.md:{}: {old} -> {new} ({why})", at + 1);
    }
    let moved: BTreeMap<usize, (usize, &str)> = anchored
        .iter()
        .flat_map(|(id, map, _)| {
            map.iter()
                .map(move |(old, new)| (*old, (*new, id.as_str())))
        })
        .collect();
    for at in start..end {
        if lines[at].starts_with('|') {
            continue;
        }
        let done: BTreeSet<usize> = edits
            .iter()
            .filter(|edit| edit.0 == at)
            .map(|edit| edit.1)
            .collect();
        for token in tokens(&lines[at]) {
            if done.contains(&token.start) {
                continue;
            }
            if let Some((new, id)) = moved.get(&token.value) {
                flagged.push(format!(
                    "SPEC.md:{} still states {} (-> {new} for {id}?) before \"{}\"",
                    at + 1,
                    token.value,
                    after(&lines, at, token.end)
                        .chars()
                        .take(40)
                        .collect::<String>()
                        .trim()
                ));
            }
        }
    }
    texts.spec = joined(&lines, &texts.spec);
}

/// The statement of each row of a register text, by ID.
fn statements_of(text: &str) -> Result<BTreeMap<String, String>, Fail> {
    let value: toml::Value = text
        .parse()
        .map_err(|error| format!("counts: model/ids.toml: {error}"))?;
    let mut out = BTreeMap::new();
    for row in value
        .get("id")
        .and_then(toml::Value::as_array)
        .ok_or("counts: model/ids.toml has no rows")?
    {
        if let (Some(id), Some(statement)) = (
            row.get("id").and_then(toml::Value::as_str),
            row.get("statement").and_then(toml::Value::as_str),
        ) {
            out.insert(id.to_owned(), statement.to_owned());
        }
    }
    Ok(out)
}

/// A string as a `.lex.tex` string literal holds it (AGENTS.md, *Editing the
/// specification*).
fn source_string(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for scalar in text
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('%', "\u{ff05}")
        .chars()
    {
        if u32::from(scalar) < 0x20 {
            let _ = write!(out, "\\u{:04x}", u32::from(scalar));
        } else {
            out.push(scalar);
        }
    }
    out
}

/// Carry the rewritten register statements and SPEC.md prose into the modules
/// of `spec/`, then write the projections from them.
fn write_sources(root: &Path, texts: &Texts) -> Result<(), Fail> {
    let before = statements_of(&std::fs::read_to_string(root.join("model/ids.toml"))?)?;
    let after = statements_of(&texts.ids)?;
    let mut modules: BTreeMap<PathBuf, String> = BTreeMap::new();
    for entry in std::fs::read_dir(root.join("spec/src"))? {
        let path = entry?.path();
        let is_register = path
            .file_name()
            .and_then(|name| name.to_str())
            .is_some_and(|name| name.starts_with("Ids") && name.ends_with(".lex.tex"));
        if is_register {
            let text = std::fs::read_to_string(&path)?;
            modules.insert(path, text);
        }
    }
    let mut changed: BTreeSet<PathBuf> = BTreeSet::new();
    for (id, statement) in &after {
        let Some(old) = before.get(id) else { continue };
        if old == statement {
            continue;
        }
        let row = format!("id := \"{id}\",");
        let from = format!("statement := \"{}\"", source_string(old));
        let to = format!("statement := \"{}\"", source_string(statement));
        let mut found = false;
        for (path, text) in &mut modules {
            let Some(at) = text.find(&row) else { continue };
            let end = text[at..].find('\n').map_or(text.len(), |n| at + n);
            if text[at..end].matches(&from).count() != 1 {
                return Err(format!(
                    "counts: the row of {id} in {} does not hold its projected statement",
                    path.display()
                )
                .into());
            }
            let line = text[at..end].replacen(&from, &to, 1);
            text.replace_range(at..end, &line);
            changed.insert(path.clone());
            found = true;
            break;
        }
        if !found {
            return Err(format!("counts: no register module holds the row of {id}").into());
        }
    }
    for path in &changed {
        std::fs::write(path, &modules[path])?;
    }
    let outcome = repo_model::spec_edit::port(root, &texts.spec)?;
    repo_model::spec_edit::apply(root, &outcome)?;
    crate::codegen::check_model(root, true)
}

#[cfg(test)]
mod tests {
    use super::{
        after, collapsed, following, number_map, prose, render, rewrite, rust_literal, tokens,
        Anchored, Moved, Seen, Texts,
    };
    use std::collections::{BTreeMap, BTreeSet};

    fn plan_pairs(values: &[(usize, usize)]) -> (BTreeMap<usize, usize>, Vec<String>) {
        let mut table = BTreeMap::new();
        let mut seen = Seen::new();
        let mut moved = Vec::new();
        for (index, &(old, new)) in values.iter().enumerate() {
            let name = format!("COUNT_{index}");
            table.insert(name.clone(), (old, index, old.to_string()));
            seen.insert(
                ("BS-05".to_owned(), "case.rs".to_owned(), name.clone()),
                (BTreeSet::from([old]), BTreeSet::from([new])),
            );
            if old != new {
                moved.push(Moved {
                    id: "BS-05".to_owned(),
                    file: "case.rs".to_owned(),
                    name,
                    old,
                    new,
                });
            }
        }
        let mine: Vec<&Moved> = moved.iter().collect();
        let mut flagged = Vec::new();
        let map = number_map(
            "BS-05",
            &mine,
            &table,
            &seen,
            "30 vectors",
            &mut String::new(),
            &mut flagged,
        );
        (map, flagged)
    }

    #[test]
    fn pair_planner_rejects_unchanged_and_changed_explanation_collisions() {
        for d_new in [18, 20] {
            let (map, flagged) = plan_pairs(&[(10, 11), (20, 20), (12, 12), (18, d_new)]);
            assert!(!map.contains_key(&30));
            assert_eq!(flagged.len(), 1);
            assert!(flagged[0].contains("which move differently"));
        }
    }

    #[test]
    fn pair_planner_accepts_agreement_but_does_not_plan_unchanged_sums() {
        let (map, flagged) = plan_pairs(&[(10, 11), (20, 20), (12, 12), (18, 19)]);
        assert_eq!(map.get(&30), Some(&31));
        assert!(flagged.is_empty());
        let (map, flagged) = plan_pairs(&[(10, 10), (20, 20), (12, 12), (18, 18)]);
        assert!(map.is_empty());
        assert!(flagged.is_empty());
        let (map, flagged) = plan_pairs(&[(10, 11), (20, 19)]);
        assert!(!map.contains_key(&30));
        assert!(flagged.is_empty());
    }

    fn anchor(id: &str, new: usize, context: &str) -> Anchored {
        (
            id.to_owned(),
            BTreeMap::from([(30, new)]),
            vec![(30, context.to_owned())],
        )
    }

    fn plan_prose(anchored: &[Anchored], shared: &BTreeMap<usize, usize>) -> (String, Vec<String>) {
        let mut texts = Texts {
            sources: BTreeMap::new(),
            spec: "### 17.23 Bootstrap\nRun 30 vectors here.\n".to_owned(),
            ids: String::new(),
            features: BTreeMap::new(),
        };
        let mut flagged = Vec::new();
        prose(
            anchored,
            shared,
            &mut texts,
            &mut String::new(),
            &mut flagged,
        );
        (texts.spec, flagged)
    }

    #[test]
    fn prose_planner_rejects_conflicting_anchors_without_shared_fallback() {
        for other in [30, 32] {
            let mut anchored = vec![
                anchor("BS-05", 31, " vectors"),
                anchor("BS-06", other, " vectors here"),
            ];
            for _ in 0..2 {
                for shared in [BTreeMap::new(), BTreeMap::from([(30, 33)])] {
                    let (spec, flagged) = plan_prose(&anchored, &shared);
                    assert!(spec.contains("Run 30 vectors here."));
                    assert!(flagged
                        .iter()
                        .any(|flag| flag.contains("conflicting destinations")));
                }
                anchored.reverse();
            }
        }
    }

    #[test]
    fn prose_planner_accepts_agreeing_anchors_and_unmatched_shared_numbers() {
        let anchored = vec![
            anchor("BS-05", 31, " vectors"),
            anchor("BS-06", 31, " vectors here"),
            anchor("BS-07", 32, " other words"),
        ];
        let (spec, flagged) = plan_prose(&anchored, &BTreeMap::from([(30, 33)]));
        assert!(spec.contains("Run 31 vectors here."));
        assert!(flagged.is_empty());
        let (spec, flagged) = plan_prose(&anchored[2..], &BTreeMap::from([(30, 33)]));
        assert!(spec.contains("Run 33 vectors here."));
        assert!(flagged.is_empty());
    }

    #[test]
    fn prose_words_are_read_across_its_line_breaks_within_a_paragraph() {
        let lines: Vec<String> = ["run 2,927 vectors: 714", "of `printCrate` (546", "", "next"]
            .iter()
            .map(|line| (*line).to_owned())
            .collect();
        let words = after(&lines, 0, "run 2,927".len());
        assert_eq!(words, " vectors: 714 of `printCrate` (546");
        assert!(words.starts_with(&collapsed(" vectors:  714\nof")));
    }

    #[test]
    fn numbers_are_read_with_their_groups_and_not_from_names() {
        let line = "the 2,838 vectors of `BS-05` (§17.16, 0x0B55) are 693 and 2,145, 3522.";
        let values: Vec<usize> = tokens(line).iter().map(|token| token.value).collect();
        assert_eq!(values, vec![2_838, 693, 2_145, 3_522]);
    }

    #[test]
    fn a_rewrite_is_simultaneous_and_keeps_the_style() {
        let map = BTreeMap::from([(693, 714), (714, 800), (2_145, 2_213)]);
        let (line, applied) = rewrite("693 of 714, then 2,145 and 2145", &map);
        assert_eq!(line, "714 of 800, then 2,213 and 2213");
        assert_eq!(applied[&2_145], 2);
        let (grown, _) = rewrite("999 and 1,000", &BTreeMap::from([(999, 1_001)]));
        assert_eq!(grown, "1,001 and 1,000");
    }

    #[test]
    fn literals_and_contexts() {
        assert_eq!(rust_literal(2_989, true), "2_989");
        assert_eq!(rust_literal(3_522, false), "3522");
        assert_eq!(rust_literal(12_000, false), "12_000");
        assert_eq!(render(1_532, true), "1,532");
        assert_eq!(
            following("509 runs of `validateCrate` and more", 3),
            " runs of `validateCrate`"
        );
        assert_eq!(following("66 | rest", 2), " ");
    }
}
