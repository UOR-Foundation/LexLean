//! The chapter edit tool (`NS-07`, SPEC.md §24.8, §27.5): change the chapter
//! modules of `spec/` by item, through the authoring SDK, and write nothing
//! unless the regenerated SPEC.md is the intended text.
//!
//! SPEC.md is the projection of the chapter modules (`NS-05`), so a change to
//! its prose is a change to the strings of an item. This does that change by
//! the block the person means (a paragraph, a table, a fence, found by its
//! first line), splits the block into items of at most
//! [`crate::project::ITEM_LINES`] lines, renumbers the module's items, and
//! writes the module with `lexlean::sdk::write_module`, which keeps the
//! module's own header and writes the canonical notation, so there is no
//! separate formatting step. It reads no Markdown back into a module: the text
//! it applies is lines.
//!
//! It is Rust. It can falsify an edit and never establishes one (C9): what
//! establishes it is the comparison of the regenerated SPEC.md with the text
//! the edit was meant to produce, made on a scratch copy before anything is
//! written to the tree.

use std::ops::Range;
use std::path::Path;

use serde_json::{json, Value};

use crate::project::{project, BlockKind, BlockLayout, Projection, ITEM_LINES, SOURCE_DIR};
use crate::ModelError;

/// The new text of one chapter module.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Change {
    /// The chapter module (`Chapter05`).
    pub chapter: String,
    /// The module's text after the edit.
    pub text: String,
}

/// What an edit produced: verified on a scratch copy, and written only by [`apply`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Outcome {
    /// The modules that change.
    pub changes: Vec<Change>,
    /// The SPEC.md the changed modules project to.
    pub spec: String,
    /// What the edit did, a line per hunk or block.
    pub report: Vec<String>,
}

/// What a kind of inserted block is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InsertKind {
    /// A paragraph.
    Paragraph,
    /// A table.
    Table,
}

/// The text an edit is meant to produce.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Intended {
    /// SPEC.md is exactly this.
    Exact(String),
    /// SPEC.md is this text except where a derived item expands: a table
    /// generated from the registers, and the total, are not ported.
    Merged(String),
}

fn refuse(message: String) -> ModelError {
    ModelError::Inconsistent(format!("R1: {message} (NS-07)"))
}

/// An item of a chapter module: its constructor and its arguments.
type ItemJson = (String, Vec<Value>);

fn percent(line: &str) -> String {
    line.replace('%', "\u{ff05}")
}

fn string_list(lines: &[String]) -> Value {
    let mut list = json!({"element": {"kind": "string"}, "kind": "nil"});
    for line in lines.iter().rev() {
        list = json!({
            "head": {"kind": "string", "value": percent(line)},
            "kind": "cons",
            "tail": list,
        });
    }
    list
}

fn definition(name: &str, constructor: &str, arguments: Vec<Value>) -> Value {
    json!({
        "kind": "definition",
        "name": name,
        "parameters": [],
        "result": {"arguments": [], "kind": "named", "member": {"module": "DocRows", "name": "Item"}},
        "body": {
            "kind": "constructor",
            "constructor": {"module": "DocRows", "name": format!("Item.{constructor}")},
            "arguments": arguments,
            "type_arguments": [],
        },
        "executable": true,
    })
}

fn module_path(root: &Path, chapter: &str) -> std::path::PathBuf {
    root.join(SOURCE_DIR).join(format!("{chapter}.lex.tex"))
}

/// A chapter module's items, read through the SDK.
fn read_items(root: &Path, chapter: &str) -> Result<(String, Vec<ItemJson>), ModelError> {
    let path = module_path(root, chapter);
    let text = std::fs::read_to_string(&path).map_err(|e| ModelError::Io(path.clone(), e))?;
    let data = lexlean::sdk::read_module(&text)
        .map_err(|error| refuse(format!("{}: {}", path.display(), error.message)))?;
    let declarations = data["declarations"]
        .as_array()
        .ok_or_else(|| refuse(format!("{}: no declarations", path.display())))?;
    let mut items = Vec::new();
    for declaration in declarations {
        let body = &declaration["body"];
        let name = body["constructor"]["name"]
            .as_str()
            .and_then(|name| name.strip_prefix("Item."))
            .ok_or_else(|| {
                refuse(format!(
                    "{}: an item is not a `DocRows.Item`",
                    path.display()
                ))
            })?;
        let arguments = body["arguments"]
            .as_array()
            .ok_or_else(|| refuse(format!("{}: an item has no arguments", path.display())))?;
        items.push((name.to_owned(), arguments.clone()));
    }
    Ok((text, items))
}

/// The module's text with `items` as its declarations, numbered in order.
fn write_items(text: &str, items: &[ItemJson]) -> Result<String, ModelError> {
    let declarations: Vec<Value> = items
        .iter()
        .enumerate()
        .map(|(index, (constructor, arguments))| {
            definition(
                &format!("i{:04}", index + 1),
                constructor,
                arguments.clone(),
            )
        })
        .collect();
    let data = json!({"spec": "lexlean/semantic-module/2", "declarations": declarations});
    lexlean::sdk::write_module(text, &data)
        .map_err(|error| refuse(format!("the SDK refuses the module: {}", error.message)))
}

/// The items of a block of `lines`, as `constructor`: the first carries what
/// the block's constructor takes before its lines, the rest continue it.
fn block_items(constructor: &str, head: &[Value], lines: &[String]) -> Vec<ItemJson> {
    let mut items = Vec::new();
    let mut chunks = lines.chunks(ITEM_LINES);
    let first = chunks.next().unwrap_or(&[]);
    let mut arguments = head.to_vec();
    arguments.push(string_list(first));
    items.push((constructor.to_owned(), arguments));
    for chunk in chunks {
        items.push(("cont".to_owned(), vec![string_list(chunk)]));
    }
    items
}

/// The lines of a block of `kind`: no line break, no U+FF05, and the lines
/// the document would read as another structure refused. A paragraph or table
/// holds no empty line (that starts another block), no line that begins a
/// heading (`#`) and none that opens a fence; a fence holds none that closes it.
/// The comparison of SPEC.md cannot see these, because the text is the same
/// either way: only the structure of the module differs.
fn check_lines(lines: &[String], kind: &BlockKind) -> Result<(), ModelError> {
    if lines.is_empty() {
        return Err(refuse(
            "a block is at least one line: an empty block is removed by hand".to_owned(),
        ));
    }
    for line in lines {
        if line.contains('\n') || line.contains('\r') {
            return Err(refuse(format!(
                "`{line:.40}` holds a line break: a block is lines"
            )));
        }
        if line.contains('\u{ff05}') {
            return Err(refuse(format!(
                "`{line:.40}` holds U+FF05, which the chapters reserve to spell a percent sign"
            )));
        }
        if line.starts_with("```") {
            return Err(refuse(format!(
                "`{line:.40}` opens or closes a fence: a fence is a block of its own, and its lines are not edited into another"
            )));
        }
        if !matches!(kind, BlockKind::Fence(_)) {
            if line.is_empty() {
                return Err(refuse(
                    "an empty line starts another block: replace-block takes the lines of one block, and insert-after adds the next".to_owned(),
                ));
            }
            if line.starts_with('#') {
                return Err(refuse(format!(
                    "`{line:.40}` begins a heading, which is not a line of a paragraph or a table"
                )));
            }
        }
    }
    Ok(())
}

fn chapter_name(chapter: &str) -> Result<(), ModelError> {
    let shaped = chapter
        .strip_prefix("Chapter")
        .is_some_and(|digits| digits.len() == 2 && digits.bytes().all(|b| b.is_ascii_digit()));
    if shaped {
        Ok(())
    } else {
        Err(refuse(format!(
            "`{chapter:.40}` is not a chapter module (`Chapter` and two digits)"
        )))
    }
}

/// The blocks of `chapter` whose first line is `first_line`.
fn find<'a>(
    projection: &'a Projection,
    chapter: &str,
    first_line: &str,
) -> Result<&'a BlockLayout, ModelError> {
    chapter_name(chapter)?;
    if !projection
        .layout
        .blocks
        .iter()
        .any(|block| block.chapter == chapter)
        && !projection.chapters.iter().any(|name| name == chapter)
    {
        return Err(refuse(format!(
            "{chapter}: not a chapter module of the specification project"
        )));
    }
    let found: Vec<&BlockLayout> = projection
        .layout
        .blocks
        .iter()
        .filter(|block| {
            block.chapter == chapter && block.lines.first().map(String::as_str) == Some(first_line)
        })
        .collect();
    match found.as_slice() {
        [one] => Ok(one),
        [] => Err(refuse(format!(
            "{chapter}: no block begins with `{first_line:.60}`"
        ))),
        many => Err(refuse(format!(
            "{chapter}: {} blocks begin with `{first_line:.60}` (items {}); name another first line",
            many.len(),
            many.iter()
                .map(|block| format!("{}..{}", block.first + 1, block.last + 1))
                .collect::<Vec<_>>()
                .join(", ")
        ))),
    }
}

fn kind_constructor(kind: &BlockKind) -> &'static str {
    match kind {
        BlockKind::Paragraph => "para",
        BlockKind::Table => "table",
        BlockKind::Fence(_) => "fence",
    }
}

/// `items` with the block replaced by one of `lines`.
fn replace_in(items: &mut Vec<ItemJson>, block: &BlockLayout, lines: &[String]) {
    let head: Vec<Value> = {
        let first = &items[block.first].1;
        first[..first.len() - 1].to_vec()
    };
    let replacement = block_items(kind_constructor(&block.kind), &head, lines);
    items.splice(block.first..=block.last, replacement);
}

/// The lines of SPEC.md, as the projector writes them.
fn lines_of(text: &str) -> Vec<String> {
    text.strip_suffix('\n')
        .unwrap_or(text)
        .split('\n')
        .map(str::to_owned)
        .collect()
}

fn join(lines: &[String]) -> String {
    let mut text = lines.join("\n");
    text.push('\n');
    text
}

/// Project the tree with `changes` written, on a scratch copy, and compare the
/// regenerated SPEC.md with what the edit was meant to produce.
///
/// # Errors
///
/// Refuses naming the first differing line; nothing of the tree is touched.
pub fn verify(
    root: &Path,
    changes: &[Change],
    intended: &Intended,
) -> Result<Projection, ModelError> {
    let scratch = tempfile::Builder::new()
        .prefix("lexlean-spec-edit-")
        .tempdir()
        .map_err(|e| ModelError::Io(root.to_path_buf(), e))?;
    let source = scratch.path().join(SOURCE_DIR);
    std::fs::create_dir_all(&source).map_err(|e| ModelError::Io(source.clone(), e))?;
    let from = root.join(SOURCE_DIR);
    for entry in std::fs::read_dir(&from).map_err(|e| ModelError::Io(from.clone(), e))? {
        let entry = entry.map_err(|e| ModelError::Io(from.clone(), e))?;
        std::fs::copy(entry.path(), source.join(entry.file_name()))
            .map_err(|e| ModelError::Io(entry.path(), e))?;
    }
    for change in changes {
        let path = source.join(format!("{}.lex.tex", change.chapter));
        std::fs::write(&path, &change.text).map_err(|e| ModelError::Io(path, e))?;
        // The SDK writes the canonical notation: formatting is the identity.
        let again = lexlean::sdk::to_notation(&change.text)
            .map_err(|error| refuse(format!("{}: {}", change.chapter, error.message)))?;
        if again != change.text {
            return Err(refuse(format!(
                "{}: the written module is not canonical",
                change.chapter
            )));
        }
    }
    let projection = project(scratch.path())?;
    let regenerated = lines_of(&projection.spec);
    match intended {
        Intended::Exact(text) => {
            let wanted = lines_of(text);
            if let Some(at) = (0..regenerated.len().max(wanted.len()))
                .find(|&at| regenerated.get(at) != wanted.get(at))
            {
                return Err(refuse(format!(
                    "the regenerated SPEC.md differs from the intended text at line {}: `{:.80}` against `{:.80}`",
                    at + 1,
                    regenerated.get(at).map_or("(end)", String::as_str),
                    wanted.get(at).map_or("(end)", String::as_str)
                )));
            }
        }
        Intended::Merged(text) => {
            let wanted = lines_of(text);
            for hunk in hunks(&regenerated, &wanted)? {
                if !within(&projection.layout.derived, &hunk) {
                    return Err(refuse(format!(
                        "the regenerated SPEC.md differs from the merged text outside the derived regions at line {}: `{:.80}` against `{:.80}`",
                        hunk.a.start + 1,
                        regenerated.get(hunk.a.start).map_or("(end)", String::as_str),
                        wanted.get(hunk.b.start).map_or("(end)", String::as_str)
                    )));
                }
            }
        }
    }
    Ok(projection)
}

/// Write the modules of a verified outcome.
///
/// # Errors
///
/// An I/O failure.
pub fn apply(root: &Path, outcome: &Outcome) -> Result<(), ModelError> {
    for change in &outcome.changes {
        let path = module_path(root, &change.chapter);
        std::fs::write(&path, &change.text).map_err(|e| ModelError::Io(path, e))?;
    }
    Ok(())
}

/// Replace the block of `chapter` that begins with `first_line` by `new_lines`.
///
/// # Errors
///
/// A first line that matches no block or two, a line the chapters cannot hold,
/// or a regenerated SPEC.md that is not the intended text.
pub fn replace_block(
    root: &Path,
    chapter: &str,
    first_line: &str,
    new_lines: &[String],
) -> Result<Outcome, ModelError> {
    let current = project(root)?;
    let block = find(&current, chapter, first_line)?;
    check_lines(new_lines, &block.kind)?;
    let (text, mut items) = read_items(root, chapter)?;
    let mut intended = lines_of(&current.spec);
    intended.splice(block.body.clone(), new_lines.iter().cloned());
    replace_in(&mut items, block, new_lines);
    let change = Change {
        chapter: chapter.to_owned(),
        text: write_items(&text, &items)?,
    };
    let projection = verify(
        root,
        std::slice::from_ref(&change),
        &Intended::Exact(join(&intended)),
    )?;
    Ok(Outcome {
        changes: vec![change],
        spec: projection.spec,
        report: vec![format!(
            "replaced the block of {chapter} that begins `{first_line:.50}`"
        )],
    })
}

/// Insert a block of `new_lines` after the block of `chapter` that begins with
/// `first_line`.
///
/// # Errors
///
/// As [`replace_block`].
pub fn insert_after(
    root: &Path,
    chapter: &str,
    first_line: &str,
    kind: InsertKind,
    new_lines: &[String],
) -> Result<Outcome, ModelError> {
    check_lines(new_lines, &BlockKind::Paragraph)?;
    if kind == InsertKind::Table && new_lines.iter().any(|line| !line.starts_with('|')) {
        return Err(refuse(
            "a table is lines that each begin with `|`".to_owned(),
        ));
    }
    let current = project(root)?;
    let block = find(&current, chapter, first_line)?;
    let (text, mut items) = read_items(root, chapter)?;
    let mut intended = lines_of(&current.spec);
    let after = block.body.end + usize::from(matches!(block.kind, BlockKind::Fence(_)));
    let mut inserted = vec![String::new()];
    inserted.extend(new_lines.iter().cloned());
    intended.splice(after..after, inserted);
    let constructor = match kind {
        InsertKind::Paragraph => "para",
        InsertKind::Table => "table",
    };
    items.splice(
        block.last + 1..block.last + 1,
        block_items(constructor, &[], new_lines),
    );
    let change = Change {
        chapter: chapter.to_owned(),
        text: write_items(&text, &items)?,
    };
    let projection = verify(
        root,
        std::slice::from_ref(&change),
        &Intended::Exact(join(&intended)),
    )?;
    Ok(Outcome {
        changes: vec![change],
        spec: projection.spec,
        report: vec![format!(
            "inserted a block after the block of {chapter} that begins `{first_line:.50}`"
        )],
    })
}

/// A run of changed lines: `a` in one text, `b` in the other.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hunk {
    /// The lines of the first text.
    pub a: Range<usize>,
    /// The lines of the second text.
    pub b: Range<usize>,
}

/// Is every changed line of `hunk` inside a derived region? A change is inside
/// when its lines are among the region's; an insertion only when it falls
/// strictly between the region's first and last line, so that a block inserted
/// at an edge (before a table, say) is never excused as regenerated.
fn within(derived: &[Range<usize>], hunk: &Hunk) -> bool {
    derived.iter().any(|region| {
        if hunk.a.is_empty() {
            hunk.a.start > region.start && hunk.a.start < region.end
        } else {
            hunk.a.start >= region.start && hunk.a.end <= region.end
        }
    })
}

/// The most lines two texts may differ by for [`hunks`] to search for the
/// difference. A merged SPEC.md shares nearly everything with the projection
/// it is compared with; a text that shares little is not a merge of it, and
/// the search for its difference costs memory in proportion to the square of
/// the difference.
pub const MAX_DIFFERENCE: usize = 2000;

/// The hunks that turn `a` into `b` (Myers' O(ND) difference over lines).
///
/// # Errors
///
/// Refuses when the texts differ by more than [`MAX_DIFFERENCE`] lines, before
/// the search.
pub fn hunks(a: &[String], b: &[String]) -> Result<Vec<Hunk>, ModelError> {
    let mut prefix = 0;
    while prefix < a.len() && prefix < b.len() && a[prefix] == b[prefix] {
        prefix += 1;
    }
    let mut suffix = 0;
    while suffix < a.len() - prefix
        && suffix < b.len() - prefix
        && a[a.len() - 1 - suffix] == b[b.len() - 1 - suffix]
    {
        suffix += 1;
    }
    let (x, y) = (&a[prefix..a.len() - suffix], &b[prefix..b.len() - suffix]);
    let (n, m) = (x.len(), y.len());
    if n == 0 && m == 0 {
        return Ok(Vec::new());
    }
    if n.abs_diff(m) > MAX_DIFFERENCE {
        return Err(too_different(n, m));
    }
    let max = (n + m).min(MAX_DIFFERENCE);
    let signed = |value: usize| isize::try_from(value).unwrap_or(0);
    let offset = max + 1;
    let index = |k: isize| usize::try_from(k + signed(offset)).unwrap_or(0);
    let mut v = vec![0usize; 2 * max + 3];
    // `trace[d]` is the furthest x on each diagonal after round d - 1, for the
    // diagonals -(d - 1)..=(d - 1): the part of the row that round d reads.
    let mut trace: Vec<Vec<usize>> = Vec::new();
    let mut found = None;
    'search: for d in 0..=max {
        trace.push(if d == 0 {
            Vec::new()
        } else {
            (-(signed(d) - 1)..=signed(d) - 1)
                .map(|k| v[index(k)])
                .collect()
        });
        let mut k = -signed(d);
        while k <= signed(d) {
            let down = k == -signed(d) || (k != signed(d) && v[index(k - 1)] < v[index(k + 1)]);
            let mut xi = if down {
                v[index(k + 1)]
            } else {
                v[index(k - 1)] + 1
            };
            let mut yi = usize::try_from(signed(xi) - k).unwrap_or(0);
            while xi < n && yi < m && x[xi] == y[yi] {
                xi += 1;
                yi += 1;
            }
            v[index(k)] = xi;
            if xi >= n && yi >= m {
                found = Some(d);
                break 'search;
            }
            k += 2;
        }
    }
    let Some(d_final) = found else {
        return Err(too_different(n, m));
    };
    // Walk back through the trace, collecting where a line of `x` was deleted
    // (`true`) or a line of `y` was inserted (`false`).
    let mut changes: Vec<(usize, usize, bool)> = Vec::new();
    let (mut xi, mut yi) = (n, m);
    for d in (1..=d_final).rev() {
        let entering = &trace[d];
        // A diagonal outside the saved window was never written before round
        // `d`, and reads as 0, as in the search.
        let at = |k: isize| {
            let slot = k + signed(d) - 1;
            usize::try_from(slot)
                .ok()
                .and_then(|slot| entering.get(slot).copied())
                .unwrap_or(0)
        };
        let k = signed(xi) - signed(yi);
        let down = k == -signed(d) || (k != signed(d) && at(k - 1) < at(k + 1));
        let previous_k = if down { k + 1 } else { k - 1 };
        let previous_x = at(previous_k);
        let previous_y = usize::try_from(signed(previous_x) - previous_k).unwrap_or(0);
        changes.push((previous_x, previous_y, !down));
        xi = previous_x;
        yi = previous_y;
    }
    changes.reverse();
    // Group consecutive changes into hunks.
    let mut out: Vec<Hunk> = Vec::new();
    for (cx, cy, deleted) in changes {
        let (a_len, b_len) = if deleted { (1, 0) } else { (0, 1) };
        match out.last_mut() {
            Some(last) if last.a.end == cx && last.b.end == cy => {
                last.a.end += a_len;
                last.b.end += b_len;
            }
            _ => out.push(Hunk {
                a: cx..cx + a_len,
                b: cy..cy + b_len,
            }),
        }
    }
    Ok(out
        .into_iter()
        .map(|hunk| Hunk {
            a: hunk.a.start + prefix..hunk.a.end + prefix,
            b: hunk.b.start + prefix..hunk.b.end + prefix,
        })
        .collect())
}

fn too_different(n: usize, m: usize) -> ModelError {
    refuse(format!(
        "the merged text and SPEC.md differ by more than {MAX_DIFFERENCE} lines ({n} against {m} between their common ends): it is not a merge of this specification, so no hunk is searched for"
    ))
}

/// What the hunks ask of one chapter module's items.
enum Edit {
    /// Replace a block (found in the projection) by these lines.
    Replace(BlockLayout, Vec<String>),
    /// Insert a block of this constructor and these lines after a block.
    Insert(BlockLayout, &'static str, Vec<String>),
}

impl Edit {
    /// Where in the module's items the edit acts, and which of two edits at one
    /// position goes first when applied from the last position to the first.
    fn key(&self) -> (usize, u8) {
        match self {
            Self::Replace(block, _) => (block.first, 1),
            Self::Insert(block, _, _) => (block.last + 1, 0),
        }
    }

    fn chapter(&self) -> &str {
        match self {
            Self::Replace(block, _) | Self::Insert(block, _, _) => &block.chapter,
        }
    }
}

/// Port the hunks of a merged SPEC.md that lie outside derived regions into
/// the chapter modules.
///
/// A hunk inside a derived region (a table generated from a register, or the
/// total) is reported as regenerated and not ported. A hunk that spans two
/// blocks, or touches a heading, a blank line or a fence marker, is refused
/// naming what it touches.
///
/// # Errors
///
/// A hunk that cannot be ported, or a regenerated SPEC.md that differs from
/// the merged text outside the derived regions.
pub fn port(root: &Path, merged: &str) -> Result<Outcome, ModelError> {
    let current = project(root)?;
    let before = lines_of(&current.spec);
    let after = lines_of(merged);
    let mut report = Vec::new();
    // The lines of each block that changes, with every hunk applied to them,
    // and the blocks to insert; both keyed so that two hunks of one block are
    // one replacement.
    let mut replaced: std::collections::BTreeMap<(String, usize), (BlockLayout, Vec<String>)> =
        std::collections::BTreeMap::new();
    let mut inserted: Vec<Edit> = Vec::new();
    for found in hunks(&before, &after)?.iter().rev() {
        let mut hunk = found.clone();
        let mut new: Vec<String> = after[hunk.b.clone()].to_vec();
        // The same insertion has two alignments: a paragraph and the blank line
        // after it, or the blank line before it and the paragraph. Take the
        // second, which is the form a block is inserted in.
        if hunk.a.is_empty()
            && hunk.a.start > 0
            && before[hunk.a.start - 1].is_empty()
            && new.first().is_some_and(|line| !line.is_empty())
            && new.last().is_some_and(String::is_empty)
        {
            new.pop();
            new.insert(0, String::new());
            hunk.a = hunk.a.start - 1..hunk.a.start - 1;
        }
        let (start, end) = (hunk.a.start, hunk.a.end);
        if within(&current.layout.derived, &hunk) {
            report.push(format!(
                "lines {}..{} are in a derived region: regenerated, not ported",
                start + 1,
                end + 1
            ));
            continue;
        }
        // A paragraph or table inserted between blocks: the first new line is
        // the blank line that separates it.
        if hunk.a.is_empty() && new.first().is_some_and(String::is_empty) {
            let lines = &new[1..];
            check_lines(lines, &BlockKind::Paragraph).map_err(|error| {
                refuse(format!(
                    "the hunk at line {} inserts more than one paragraph or table, a heading or a fence, or a line a block cannot hold: edit it with insert-after, one block at a time ({error})",
                    start + 1
                ))
            })?;
            let Some(block) =
                current.layout.blocks.iter().find(|b| {
                    b.body.end + usize::from(matches!(b.kind, BlockKind::Fence(_))) == start
                })
            else {
                return Err(refuse(format!(
                    "the hunk at line {} inserts a block that does not follow a paragraph, a table or a fence",
                    start + 1
                )));
            };
            let constructor = if lines.iter().all(|l| l.starts_with('|')) {
                "table"
            } else {
                "para"
            };
            report.push(format!(
                "inserted a block after the block of {} at line {}",
                block.chapter,
                start + 1
            ));
            inserted.push(Edit::Insert(block.clone(), constructor, lines.to_vec()));
            continue;
        }
        // Otherwise the hunk is a change inside one block's lines.
        let touched: Vec<&BlockLayout> = current
            .layout
            .blocks
            .iter()
            .filter(|b| {
                if hunk.a.is_empty() {
                    start >= b.body.start && start <= b.body.end
                } else {
                    start < b.body.end && end > b.body.start
                }
            })
            .collect();
        let inside =
            touched.len() == 1 && start >= touched[0].body.start && end <= touched[0].body.end;
        if !inside {
            // A hunk that touches no block's lines (a blank line, a heading, a
            // fence marker) is named by the blocks on either side of it.
            let around: Vec<&BlockLayout> = if touched.is_empty() {
                current
                    .layout
                    .blocks
                    .iter()
                    .filter(|b| b.body.end + 2 >= start && b.body.start <= end + 2)
                    .collect()
            } else {
                touched.clone()
            };
            let names: Vec<String> = around
                .iter()
                .map(|b| {
                    format!(
                        "{} (`{:.40}`)",
                        b.chapter,
                        b.lines.first().map_or("", String::as_str)
                    )
                })
                .collect();
            return Err(refuse(format!(
                "the hunk at lines {}..{} spans {}; it is not ported",
                start + 1,
                end + 1,
                if touched.is_empty() {
                    format!(
                        "a line outside every paragraph, table and fence (a heading, a blank line or a fence marker) next to {}",
                        if names.is_empty() { "no block".to_owned() } else { names.join(" and ") }
                    )
                } else {
                    names.join(" and ")
                }
            )));
        }
        let block = touched[0];
        let entry = replaced
            .entry((block.chapter.clone(), block.first))
            .or_insert_with(|| (block.clone(), block.lines.clone()));
        let offset = start - block.body.start;
        entry.1.splice(offset..offset + (end - start), new);
        report.push(format!(
            "changed the block of {} at line {}",
            block.chapter,
            start + 1
        ));
    }
    report.reverse();
    let mut edits: Vec<Edit> = Vec::new();
    for (block, lines) in replaced.into_values() {
        check_lines(&lines, &block.kind).map_err(|error| {
            refuse(format!(
                "the hunks that change the block of {} that begins `{:.40}` leave it a block the document cannot hold: {error}",
                block.chapter,
                block.lines.first().map_or("", String::as_str)
            ))
        })?;
        edits.push(Edit::Replace(block, lines));
    }
    edits.extend(inserted);
    // From the last position of a module to the first, so that the positions
    // still to edit hold.
    edits.sort_by_key(|edit| std::cmp::Reverse(edit.key()));
    let mut modules: std::collections::BTreeMap<String, (String, Vec<ItemJson>)> =
        std::collections::BTreeMap::new();
    for edit in &edits {
        let entry = module(&mut modules, root, edit.chapter())?;
        match edit {
            Edit::Replace(block, lines) => replace_in(&mut entry.1, block, lines),
            Edit::Insert(block, constructor, lines) => {
                entry.1.splice(
                    block.last + 1..block.last + 1,
                    block_items(constructor, &[], lines),
                );
            }
        }
    }
    let mut changes = Vec::new();
    for (chapter, (text, items)) in modules {
        changes.push(Change {
            text: write_items(&text, &items)?,
            chapter,
        });
    }
    let projection = verify(root, &changes, &Intended::Merged(merged.to_owned()))?;
    Ok(Outcome {
        changes,
        spec: projection.spec,
        report,
    })
}

fn module<'a>(
    modules: &'a mut std::collections::BTreeMap<String, (String, Vec<ItemJson>)>,
    root: &Path,
    chapter: &str,
) -> Result<&'a mut (String, Vec<ItemJson>), ModelError> {
    if !modules.contains_key(chapter) {
        modules.insert(chapter.to_owned(), read_items(root, chapter)?);
    }
    modules
        .get_mut(chapter)
        .ok_or_else(|| refuse(format!("{chapter}: not read")))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn strings(text: &str) -> Vec<String> {
        text.split_whitespace().map(str::to_owned).collect()
    }

    /// Applying the hunks of `hunks(a, b)` to `a` gives `b`, for texts that
    /// differ at the ends, in the middle, by insertion, deletion and
    /// replacement, and not at all.
    #[test]
    fn the_hunks_turn_one_text_into_the_other() {
        let cases = [
            ("a b c d e", "a b c d e"),
            ("a b c d e", "a x c d e"),
            ("a b c d e", "x a b c d e y"),
            ("a b c d e", "c d"),
            ("", "a b"),
            ("a b", ""),
            ("a b c d e f g h", "a c d x y f g z"),
            ("a a a a", "a a b a a"),
            ("x y z", "p q r"),
        ];
        for (a, b) in cases {
            let (a, b) = (strings(a), strings(b));
            let mut rebuilt: Vec<String> = Vec::new();
            let mut at = 0;
            for hunk in hunks(&a, &b).expect("a small difference") {
                rebuilt.extend_from_slice(&a[at..hunk.a.start]);
                rebuilt.extend_from_slice(&b[hunk.b.clone()]);
                at = hunk.a.end;
            }
            rebuilt.extend_from_slice(&a[at..]);
            assert_eq!(rebuilt, b, "{a:?} to {b:?}");
        }
    }

    /// Neighbouring changes are one hunk, and a common line separates two.
    #[test]
    fn hunks_group_what_is_adjacent() {
        let one = hunks(&strings("a b c d"), &strings("a x y d")).expect("a small difference");
        assert_eq!(one.len(), 1);
        assert_eq!((one[0].a.clone(), one[0].b.clone()), (1..3, 1..3));
        let two = hunks(&strings("a b c d e"), &strings("a x c y e")).expect("a small difference");
        assert_eq!(two.len(), 2);
    }

    /// An insertion is inside a derived region only strictly between its first
    /// and last line; a change is inside when its lines are the region's.
    #[test]
    fn an_insertion_at_the_edge_of_a_region_is_not_inside_it() {
        let region = [10..20];
        let hunk = |a: Range<usize>| Hunk { a, b: 0..1 };
        assert!(!within(&region, &hunk(10..10)), "at the first line");
        assert!(within(&region, &hunk(11..11)));
        assert!(within(&region, &hunk(19..19)));
        assert!(!within(&region, &hunk(20..20)), "at the end");
        assert!(within(&region, &hunk(10..20)), "the whole region changed");
        assert!(
            !within(&region, &hunk(9..11)),
            "a change that starts before it"
        );
    }

    /// Texts that differ by more than the bound are refused before the search,
    /// and a difference of exactly the bound's size is searched.
    #[test]
    fn a_difference_beyond_the_bound_is_refused() {
        let many: Vec<String> = (0..=MAX_DIFFERENCE + 1).map(|n| n.to_string()).collect();
        let error = hunks(&many, &[]).expect_err("beyond the bound");
        assert!(error.to_string().contains("differ by more than"), "{error}");
        let unrelated: Vec<String> = (0..300).map(|n| format!("u{n}")).collect();
        let others: Vec<String> = (0..300).map(|n| format!("v{n}")).collect();
        assert_eq!(hunks(&unrelated, &others).expect("small").len(), 1);
    }
}
