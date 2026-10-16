//! The `runtime` suite: RT-01..RT-04, the runtime of generated Lean as
//! native modules and the frozen bytes of languages 1.0 and 1.1 (SPEC.md
//! §17.12, *Runtime modules*).

use std::collections::BTreeSet;

use lexlean::backend::runtime;

use crate::support::{self, repo_root, P};

/// Run the case for one `RT` ID.
///
/// # Panics
///
/// Panics when the case's assertion fails.
pub fn run(id: &str) {
    match id {
        "RT-01" => rt_01(),
        "RT-02" => rt_02(),
        "RT-03" => rt_03(),
        "RT-04" => rt_04(),
        _ => panic!("no runtime case is wired for {id}"),
    }
}

/// The runtime namespaces a generated module once carried a copy of.
const NAMESPACES: [&str; 4] = [
    "LexLeanRuntime",
    "LexLeanCollections",
    "LexLeanModels",
    "LexLeanReasoning",
];

fn disk(path: &str) -> Result<String, String> {
    std::fs::read_to_string(repo_root().join(path).as_std_path())
        .map_err(|error| format!("{path}: {error}"))
}

/// The files below `root` with one of the extensions, skipping build output.
fn files(root: &std::path::Path, extensions: &[&str]) -> Vec<std::path::PathBuf> {
    walkdir::WalkDir::new(root)
        .into_iter()
        .filter_entry(|entry| {
            let name = entry.file_name().to_string_lossy();
            !(entry.file_type().is_dir()
                && (matches!(name.as_ref(), "target" | ".git" | ".lexlean")
                    || support::holds_cache_tag(entry)))
        })
        .flatten()
        .filter(|entry| entry.file_type().is_file())
        .map(walkdir::DirEntry::into_path)
        .filter(|path| {
            path.extension()
                .and_then(|extension| extension.to_str())
                .is_some_and(|extension| extensions.contains(&extension))
        })
        .collect()
}

/// RT-01: the runtime is LexLean source, and no runtime copy remains.
fn rt_01() {
    let modules = runtime::modules().expect("the shipped runtime");
    let names: Vec<&str> = modules.iter().map(|module| module.name.as_str()).collect();
    assert_eq!(
        names,
        [
            runtime::PORTABLE,
            runtime::COLLECTIONS,
            runtime::MODELS,
            runtime::REASONING,
            runtime::LAWS
        ],
        "the registry lists the five runtime modules in dependency order"
    );
    // The registry lists the modules and nothing else.
    let registry: toml::Table = disk(runtime::REGISTRY_PATH)
        .expect("the registry")
        .parse()
        .expect("the registry is TOML");
    assert_eq!(
        registry.keys().map(String::as_str).collect::<Vec<_>>(),
        ["modules", "spec", "toolchain"],
        "the registry holds no declaration list"
    );
    // One native source per module, embedded, and no Lean file in the
    // project.
    let project = repo_root().join(runtime::PROJECT_DIR);
    let mut sources = BTreeSet::new();
    for path in files(project.as_std_path(), &["lean", "tex"]) {
        let shown = path.display().to_string();
        assert!(
            !shown.ends_with(".lean"),
            "{shown}: a Lean file lies in the runtime project"
        );
        if let Some(stem) = path
            .strip_prefix(project.join("src").as_std_path())
            .ok()
            .map(|rest| rest.to_string_lossy().replace('\\', "/"))
            .and_then(|rest| rest.strip_suffix(".lex.tex").map(str::to_owned))
        {
            sources.insert(format!("{}.{}", runtime::PREFIX, stem.replace('/', ".")));
        }
    }
    let listed: BTreeSet<String> = names.iter().map(|name| (*name).to_owned()).collect();
    assert_eq!(sources, listed, "the native sources are the listed modules");
    for module in &names {
        let path = runtime::native_source_path(module).expect("a path");
        assert!(
            lexlean::embedded::FILES
                .iter()
                .any(|(candidate, _)| *candidate == path),
            "{path} is not embedded in the compiler"
        );
    }
    // Every declaration pins its exact axioms; a weakened, missing, or
    // escaping source is refused.
    let declared: usize = modules.iter().map(|module| module.declarations.len()).sum();
    assert!(declared > 0, "the runtime modules state declarations");
    assert_eq!(
        runtime::check_native_sources(&disk),
        Ok(declared),
        "the sources on disk are the embedded runtime"
    );
    // A plant edits the module's data form, whichever form the source is
    // written in (§17.21.8).
    let planted = |path_suffix: &str, from: &str, to: &str| {
        runtime::check_native_sources(&|path| {
            let text = disk(path)?;
            if path.ends_with(path_suffix) {
                let text = support::as_data_text(&text);
                assert!(text.contains(from), "{path} lacks {from:?}");
                // Every occurrence: a generated row's policy is not checked,
                // so a plant of one occurrence could miss the checked rows.
                Ok(text.replace(from, to))
            } else {
                Ok(text)
            }
        })
    };
    let weakened = planted(
        "Portable.lex.tex",
        "\"kind\":\"exact\"",
        "\"kind\":\"allow\"",
    );
    assert!(
        weakened
            .as_ref()
            .is_err_and(|reason| reason.contains("does not pin its axioms exactly")),
        "a weakened policy is refused: {weakened:?}"
    );
    let escaping = planted(
        "Models.lex.tex",
        "\"imports\":[\"Init\"]",
        "\"imports\":[\"Init\",\"LexLeanPreservation.Core\"]",
    );
    assert!(
        escaping
            .as_ref()
            .is_err_and(|reason| reason.contains("neither `Init` nor an earlier runtime module")),
        "an import outside the runtime is refused: {escaping:?}"
    );
    let missing = runtime::check_native_sources(&|path| {
        if path.ends_with("Collections.lex.tex") {
            Err(format!("{path}: absent"))
        } else {
            disk(path)
        }
    });
    assert!(missing.is_err(), "a missing native source is refused");
    // The one-time migration input is not in the release tree.
    for input in [
        "tests/runtime-library/oracle",
        "tests/runtime-library/laws-oracle",
    ] {
        assert!(
            !repo_root().join(input).exists(),
            "{input}: the completed one-time migration input is absent"
        );
    }
    // No native source names a runtime copy inside a module: a runtime
    // constant is `LexLeanRuntime.x`, or a private realization of it
    // `_private.<module>.#<n>.LexLeanRuntime.x`, never `<module>.LexLeanRuntime.x`;
    // a hygienic binder name `x._@.LexLeanRuntime.Collections.…`, and a
    // private name `_private.LexLeanRuntime.Reasoning.#0.…`, name the runtime
    // module they were made in.
    let root = repo_root();
    for directory in ["language", "compiler", "examples", "tests"] {
        for path in files(root.join(directory).as_std_path(), &["tex"]) {
            let text = std::fs::read_to_string(&path).unwrap_or_default();
            // A core module written as data or in the notation (§17.21.8).
            if !text.contains("\\coredata{") && !text.contains("\\begin{coremodule}") {
                continue;
            }
            for namespace in NAMESPACES {
                let needle = format!(".{namespace}.");
                for (at, _) in text.match_indices(&needle) {
                    let before = &text[..at];
                    // The name segment before the namespace: a name ends at a
                    // dot, a JSON quote, or the notation's white space and
                    // brackets.
                    let segment = before
                        .rsplit(|c: char| {
                            c == '.' || c == '"' || c.is_whitespace() || "()[]{},:".contains(c)
                        })
                        .next()
                        .unwrap_or_default();
                    assert!(
                        segment == "_@"
                            || segment == "_private"
                            || (segment.starts_with('#')
                                && segment[1..].bytes().all(|byte| byte.is_ascii_digit())),
                        "{}: a runtime copy `…{}` is named inside a module",
                        path.display(),
                        &text[at - segment.len().min(at)..(at + needle.len() + 16).min(text.len())]
                    );
                }
            }
        }
    }
    // No Lean file declares a runtime namespace but the released output of a
    // language-1.0 or 1.1 project, which spliced the frozen text.
    for path in files(root.as_std_path(), &["lean"]) {
        let text = std::fs::read_to_string(&path).unwrap_or_default();
        if !NAMESPACES.iter().any(|namespace| {
            text.lines()
                .any(|line| line == format!("namespace {namespace}"))
        }) {
            continue;
        }
        let shown = path.display().to_string().replace('\\', "/");
        let project = path
            .ancestors()
            .find(|ancestor| ancestor.join("lexlean.toml").is_file())
            .map(|project| {
                std::fs::read_to_string(project.join("lexlean.toml")).unwrap_or_default()
            });
        let frozen_language = project.is_some_and(|config| !config.contains("language = \"1.2\""));
        assert!(
            shown.contains("/expected/") && frozen_language,
            "{shown}: a Lean file declares a runtime namespace outside the released output of a frozen language"
        );
    }
}

/// RT-02: the frozen bytes are the released output of languages 1.0 and 1.1.
fn rt_02() {
    let portable = runtime::frozen_portable_runtime();
    let decoder = runtime::frozen_core_decoder();
    for (path, embedded) in [
        (runtime::FROZEN_PORTABLE_PATH, portable),
        (runtime::FROZEN_CORE_DECODER_PATH, decoder),
    ] {
        assert!(
            path.starts_with("language/runtime-1.2/frozen/"),
            "{path}: the frozen bytes are held in the named location"
        );
        assert_eq!(
            disk(path).expect("the frozen file"),
            embedded,
            "{path} is embedded as it is on disk"
        );
    }
    // The released language-1.1 output splices the portable runtime, and the
    // released language-1.0 output the decoder of core module 1, verbatim.
    let released_portable =
        disk("examples/semantic-1.1/expected/build/modules/SemanticFixture/Portable.lean")
            .expect("the released language-1.1 module");
    assert!(
        released_portable.contains(portable),
        "the frozen portable runtime is the released language-1.1 output"
    );
    let atlas = repo_root().join("examples/uor-atlas/expected/build/modules");
    let mut released_core: Vec<_> = files(atlas.as_std_path(), &["lean"]);
    released_core.sort();
    let released_core =
        std::fs::read_to_string(released_core.first().expect("a released core module"))
            .expect("the released language-1.0 module");
    assert!(
        released_core.contains(decoder),
        "the frozen decoder of core module 1 is the released language-1.0 output"
    );
    // A one-byte change of either is not the released output.
    let mutated = portable.replacen(
        "@[noinline] public def index ",
        "@[noinline] public def indez ",
        1,
    );
    assert_ne!(mutated, portable, "the plant changes the frozen text");
    assert!(
        !released_portable.contains(&mutated),
        "a mutated portable runtime is refused"
    );
    let mutated = decoder.replacen(
        "namespace LexLeanCore.Runtime",
        "namespace LexLeanCore.Runtimf",
        1,
    );
    assert!(
        !released_core.contains(&mutated),
        "a mutated decoder is refused"
    );
    // The frozen bytes belong to no older identity (RP-10 pins those IDs to
    // the committed locks), and are part of the language-1.2 one: the three
    // IDs recomputed from disk are the compiler's, and with either frozen
    // file changed the 1.0 and 1.1 IDs are the same and the 1.2 ID is not.
    assert!(
        support::spec_text().contains("`language/runtime-1.2/`"),
        "SPEC.md §21.2 excludes the runtime project and the frozen bytes from the language-1.0 and 1.1 IDs"
    );
    let rows = support::normative_rows();
    let borrowed: Vec<(&str, &[u8])> = rows
        .iter()
        .map(|(path, bytes)| (path.as_str(), bytes.as_slice()))
        .collect();
    let (ten, eleven, twelve) = support::semantics_ids(&borrowed);
    assert_eq!(ten, lexlean::compiler_semantics_id_for("1.0"));
    assert_eq!(eleven, lexlean::compiler_semantics_id_for("1.1"));
    assert_eq!(twelve, lexlean::compiler_semantics_id_for("1.2"));
    for path in [
        runtime::FROZEN_PORTABLE_PATH,
        runtime::FROZEN_CORE_DECODER_PATH,
    ] {
        let mut changed = rows.clone();
        let row = changed
            .iter_mut()
            .find(|(candidate, _)| candidate == path)
            .unwrap_or_else(|| panic!("{path} is in the normative tree"));
        row.1.push(b'\n');
        let borrowed: Vec<(&str, &[u8])> = changed
            .iter()
            .map(|(path, bytes)| (path.as_str(), bytes.as_slice()))
            .collect();
        let (changed_ten, changed_eleven, changed_twelve) = support::semantics_ids(&borrowed);
        assert_eq!(
            changed_ten, ten,
            "{path}: the language-1.0 ID does not read it"
        );
        assert_eq!(
            changed_eleven, eleven,
            "{path}: the language-1.1 ID does not read it"
        );
        assert_ne!(
            changed_twelve, twelve,
            "{path}: the language-1.2 ID reads it"
        );
    }
}

/// RT-03: a verification audits the runtime against its pinned axioms.
fn rt_03() {
    // Every committed verification of a project that imports a runtime module
    // printed the axioms of its declarations.
    let audit = disk("examples/collections/expected/verify/audit/output.txt")
        .expect("the committed audit of a project that uses the collections");
    for declaration in ["LexLeanCollections.insertEntry", "LexLeanRuntime.append"] {
        assert!(
            audit.contains(&format!("'{declaration}'")),
            "the audit names the runtime declaration `{declaration}`"
        );
    }
    if !support::lean_backed("RT-03") {
        return;
    }
    // The runtime's own project verifies: the generated Lean of every module
    // elaborates, replays, and observes exactly the axioms its rows pin.
    let project = P::runtime_project();
    project.relock();
    support::verify_ok(&project);
    // A row that pins an axiom its declaration does not use is refused.
    let planted = P::runtime_project();
    planted.edit(
        "src/Portable.lex.tex",
        "\"policy\":{\"axioms\":[],\"kind\":\"exact\"}",
        "\"policy\":{\"axioms\":[\"propext\"],\"kind\":\"exact\"}",
    );
    planted.relock();
    planted.verify_fails_with("LLV7005");
    // And the other direction: a declaration that uses an axiom its row does
    // not pin is refused as well (the verifier compares the sets both ways).
    let unpinned = P::runtime_project();
    unpinned.edit(
        "src/Collections.lex.tex",
        "\"policy\":{\"axioms\":[\"propext\"],\"kind\":\"exact\"}",
        "\"policy\":{\"axioms\":[],\"kind\":\"exact\"}",
    );
    unpinned.relock();
    unpinned.verify_fails_with("LLV7005");
}

/// A string literal of a Rust source.
struct Literal {
    /// Where its opening delimiter is, and where its closing one ends.
    start: usize,
    end: usize,
    /// The line it opens on.
    line: usize,
    /// Its text, with escapes resolved (a raw literal as written).
    text: String,
}

/// A Rust source read once: its string literals, and its code with every
/// literal, character literal and comment blanked, so that a brace, an
/// attribute or a keyword found in it is code.
struct Scan {
    literals: Vec<Literal>,
    code: Vec<u8>,
}

/// Whether the `r` at `index` opens a raw literal: it is not the end of a
/// longer name, apart from the `b` of a raw byte string.
fn opens_raw(bytes: &[u8], index: usize) -> bool {
    let name = |byte: u8| byte.is_ascii_alphanumeric() || byte == b'_';
    match index.checked_sub(1).map(|before| bytes[before]) {
        None => true,
        Some(b'b') => index == 1 || !name(bytes[index - 2]),
        Some(byte) => !name(byte),
    }
}

fn scan(source: &str) -> Scan {
    let bytes = source.as_bytes();
    let mut code = bytes.to_vec();
    let mut literals = Vec::new();
    let blank = |code: &mut Vec<u8>, from: usize, to: usize| {
        for byte in &mut code[from..to.min(bytes.len())] {
            if *byte != b'\n' {
                *byte = b' ';
            }
        }
    };
    let line_of = |at: usize| source[..at].matches('\n').count() + 1;
    let mut index = 0;
    while index < bytes.len() {
        match bytes[index] {
            b'/' if bytes.get(index + 1) == Some(&b'/') => {
                let start = index;
                while index < bytes.len() && bytes[index] != b'\n' {
                    index += 1;
                }
                blank(&mut code, start, index);
            }
            b'/' if bytes.get(index + 1) == Some(&b'*') => {
                // Block comments nest.
                let start = index;
                let mut depth = 0usize;
                while index < bytes.len() {
                    if bytes[index..].starts_with(b"/*") {
                        depth += 1;
                        index += 2;
                    } else if bytes[index..].starts_with(b"*/") {
                        depth -= 1;
                        index += 2;
                        if depth == 0 {
                            break;
                        }
                    } else {
                        index += 1;
                    }
                }
                blank(&mut code, start, index);
            }
            b'\'' => {
                // A character literal (`'x'`, `'\n'`, `'\u{1F600}'`, `'é'`) or
                // a lifetime.
                let start = index;
                if bytes.get(index + 1) == Some(&b'\\') {
                    index += 3;
                    while index < bytes.len() && bytes[index] != b'\'' {
                        index += 1;
                    }
                    index += 1;
                    blank(&mut code, start, index);
                } else if let Some(next) = source[index + 1..].chars().next() {
                    let width = next.len_utf8();
                    if bytes.get(index + 1 + width) == Some(&b'\'') {
                        index += width + 2;
                        blank(&mut code, start, index);
                    } else {
                        index += 1;
                    }
                } else {
                    index += 1;
                }
            }
            b'r' if matches!(bytes.get(index + 1), Some(b'#' | b'"'))
                && opens_raw(bytes, index) =>
            {
                let start = index;
                let mut hashes = 0;
                let mut at = index + 1;
                while bytes.get(at) == Some(&b'#') {
                    hashes += 1;
                    at += 1;
                }
                if bytes.get(at) != Some(&b'"') {
                    index += 1;
                    continue;
                }
                let closing = format!("\"{}", "#".repeat(hashes));
                let text_start = at + 1;
                let text_end = source[text_start..]
                    .find(&closing)
                    .map_or(bytes.len(), |end| text_start + end);
                index = (text_end + closing.len()).min(bytes.len());
                literals.push(Literal {
                    start,
                    end: index,
                    line: line_of(start),
                    text: source[text_start..text_end].to_owned(),
                });
                blank(&mut code, start, index);
            }
            b'"' => {
                let start = index;
                let mut text = String::new();
                index += 1;
                while index < bytes.len() && bytes[index] != b'"' {
                    if bytes[index] == b'\\' && index + 1 < bytes.len() {
                        match bytes[index + 1] {
                            b'n' => text.push('\n'),
                            b't' => text.push('\t'),
                            b'r' => text.push('\r'),
                            b'0' => text.push('\0'),
                            // A backslash before a line break drops the
                            // break and the white space that opens the next
                            // line: one literal written over several lines.
                            b'\n' | b'\r' => {
                                index += 2;
                                while index < bytes.len() && bytes[index].is_ascii_whitespace() {
                                    index += 1;
                                }
                                continue;
                            }
                            b'x' => {
                                let digits = source.get(index + 2..index + 4).unwrap_or_default();
                                if let Ok(value) = u8::from_str_radix(digits, 16) {
                                    text.push(char::from(value));
                                }
                                index += 4;
                                continue;
                            }
                            b'u' => {
                                let close =
                                    source[index..].find('}').map_or(index + 2, |at| index + at);
                                let digits = source
                                    .get(index + 3..close)
                                    .unwrap_or_default()
                                    .replace('_', "");
                                if let Some(decoded) = u32::from_str_radix(&digits, 16)
                                    .ok()
                                    .and_then(char::from_u32)
                                {
                                    text.push(decoded);
                                }
                                index = close + 1;
                                continue;
                            }
                            other => text.push(char::from(other)),
                        }
                        index += 2;
                    } else {
                        let width = source[index..].chars().next().map_or(1, char::len_utf8);
                        text.push_str(&source[index..index + width]);
                        index += width;
                    }
                }
                index = (index + 1).min(bytes.len());
                literals.push(Literal {
                    start,
                    end: index,
                    line: line_of(start),
                    text,
                });
                blank(&mut code, start, index);
            }
            _ => index += 1,
        }
    }
    Scan { literals, code }
}

/// A source with its test items blanked, each by the line breaks it held so
/// that every other line keeps its number: an item under `#[cfg(test)]` (a
/// `mod`, a function, a `use`), wherever it stands, and the whole file when
/// it opens with `#![cfg(test)]`. A unit test's fixture text is no part of
/// what the compiler emits. What follows a test item is production code and
/// stays, which a cut at the first test module would have dropped.
fn without_test_items(source: &str) -> String {
    let scanned = scan(source);
    let code = &scanned.code;
    let mut blanked = source.as_bytes().to_vec();
    let erase = |blanked: &mut Vec<u8>, from: usize, to: usize| {
        for byte in &mut blanked[from..to] {
            if *byte != b'\n' {
                *byte = b' ';
            }
        }
    };
    let text = String::from_utf8_lossy(code).into_owned();
    if text.contains("#![cfg(test)]") {
        let whole = blanked.len();
        erase(&mut blanked, 0, whole);
    }
    let mut search = 0;
    while let Some(found) = text[search..].find("#[cfg(test)]") {
        let attribute = search + found;
        // The item follows its attributes: up to the first `{` or `;` outside
        // parentheses and brackets, then the matching `}`.
        let mut at = attribute + "#[cfg(test)]".len();
        let mut nesting = 0usize;
        let mut end = code.len();
        while at < code.len() {
            match code[at] {
                b'(' | b'[' => nesting += 1,
                b')' | b']' => nesting = nesting.saturating_sub(1),
                b';' if nesting == 0 => {
                    end = at + 1;
                    break;
                }
                b'{' if nesting == 0 => {
                    let mut depth = 0usize;
                    while at < code.len() {
                        match code[at] {
                            b'{' => depth += 1,
                            b'}' => {
                                depth -= 1;
                                if depth == 0 {
                                    break;
                                }
                            }
                            _ => {}
                        }
                        at += 1;
                    }
                    end = (at + 1).min(code.len());
                    break;
                }
                _ => {}
            }
            at += 1;
        }
        erase(&mut blanked, attribute, end);
        search = end.max(attribute + 1);
    }
    String::from_utf8(blanked).expect("blanking keeps ASCII and whole characters")
}

/// What a line of Lean declares, when it declares: the text after the
/// declaration keyword, once its attributes and modifiers are read.
fn declaration(line: &str) -> Option<&str> {
    let mut rest = line.trim_start();
    while rest.starts_with("@[") {
        rest = rest
            .split_once(']')
            .map_or("", |(_, after)| after)
            .trim_start();
    }
    loop {
        let before = rest;
        for modifier in [
            "public ",
            "private ",
            "protected ",
            "noncomputable ",
            "partial ",
            "unsafe ",
        ] {
            rest = rest.strip_prefix(modifier).unwrap_or(rest);
        }
        if rest == before {
            break;
        }
    }
    [
        "def ",
        "theorem ",
        "lemma ",
        "instance ",
        "class ",
        "inductive ",
        "structure ",
        "abbrev ",
        "opaque ",
        "axiom ",
    ]
    .iter()
    .find_map(|keyword| rest.strip_prefix(keyword))
}

/// Whether a text opens or closes a runtime namespace, or declares a name
/// under one (`def LexLeanRuntime.x`), with or without a `namespace` line
/// around it. A text that only names a runtime constant, as the Lean a
/// generated module holds does, is not one.
fn declares_under_runtime(text: &str) -> bool {
    let under_runtime = |name: &str| {
        NAMESPACES
            .iter()
            .any(|namespace| name == *namespace || name.starts_with(&format!("{namespace}.")))
    };
    text.lines().map(str::trim_start).any(|line| {
        let namespace = line
            .strip_prefix("namespace ")
            .or_else(|| line.strip_prefix("end "))
            .is_some_and(|name| under_runtime(name.split_whitespace().next().unwrap_or_default()));
        let declared = declaration(line).is_some_and(|rest| {
            under_runtime(
                rest.split(|c: char| c.is_whitespace() || ":({[".contains(c))
                    .next()
                    .unwrap_or_default(),
            )
        });
        namespace || declared
    })
}

/// Whether a text of the compiler's source is Lean runtime text: it declares
/// under a runtime namespace, or it declares something inside a namespace of
/// its own, which is how a runtime module reads whatever it is called.
fn is_runtime_text(text: &str) -> bool {
    declares_under_runtime(text)
        || (text.lines().any(|line| line.starts_with("namespace "))
            && text.lines().any(|line| declaration(line).is_some()))
}

/// The runtime texts of a Rust source, by line: a literal alone, and the
/// literals written one after another (`concat!`, an array of lines, `+`,
/// the arguments of `format!`), read joined with and without a line break
/// between them, because a text can be cut anywhere.
fn runtime_texts(source: &str) -> Vec<usize> {
    let scanned = scan(source);
    let mut found = Vec::new();
    let mut chain: Vec<&Literal> = Vec::new();
    let flush = |chain: &mut Vec<&Literal>, found: &mut Vec<usize>| {
        if let Some(first) = chain.first() {
            let texts: Vec<&str> = chain.iter().map(|literal| literal.text.as_str()).collect();
            if texts.iter().any(|text| is_runtime_text(text))
                || is_runtime_text(&texts.concat())
                || is_runtime_text(&texts.join("\n"))
            {
                found.push(first.line);
            }
        }
        chain.clear();
    };
    for literal in &scanned.literals {
        if let Some(previous) = chain.last() {
            // The same expression: little between the two, and no statement
            // or block boundary.
            let between = &source[previous.end..literal.start];
            if between.len() > 64 || between.contains([';', '{', '}']) {
                flush(&mut chain, &mut found);
            }
        }
        chain.push(literal);
    }
    flush(&mut chain, &mut found);
    found
}

/// What a generated language-1.2 module must be to meet RT-04's statement:
/// it names a runtime constant only if it imports the runtime module that
/// declares it, and it declares nothing under a runtime namespace.
fn generated_module_defects(
    text: &str,
    owners: &std::collections::BTreeMap<String, &'static str>,
) -> Vec<String> {
    let mut defects = Vec::new();
    let imported: BTreeSet<&str> = runtime::imported_by([text])
        .expect("the shipped runtime")
        .into_iter()
        .map(|module| module.name.as_str())
        .collect();
    let mut seen = BTreeSet::new();
    for token in text.split(|c: char| !(c.is_alphanumeric() || matches!(c, '_' | '.' | '\''))) {
        if !NAMESPACES
            .iter()
            .any(|namespace| token.starts_with(&format!("{namespace}.")))
        {
            continue;
        }
        let name = token.trim_end_matches('.');
        if let Some(owner) = owners.get(name) {
            if !imported.contains(owner) && seen.insert(name.to_owned()) {
                defects.push(format!("uses `{name}` without importing `{owner}`"));
            }
        }
    }
    if declares_under_runtime(text) {
        defects.push("declares a runtime namespace or a name under one".to_owned());
    }
    defects
}

/// RT-04: no Lean runtime text remains in the language-1.2 compiler but the
/// decoder of core module 2, which loads native modules; and what the
/// compiler generates imports the runtime modules it uses and splices none.
fn rt_04() {
    let source_root = repo_root().join("crates/lexlean/src");
    let mut found = Vec::new();
    for path in files(source_root.as_std_path(), &["rs"]) {
        let text = std::fs::read_to_string(&path).expect("a compiler source");
        let text = without_test_items(&text);
        let relative = path
            .strip_prefix(source_root.as_std_path())
            .expect("below the source root")
            .to_string_lossy()
            .replace('\\', "/");
        for line in runtime_texts(&text) {
            found.push((relative.clone(), line));
        }
    }
    let decoder = std::fs::read_to_string(source_root.join("backend/core_v2.rs").as_std_path())
        .expect("the decoder of core module 2");
    let decoder_line = decoder
        .lines()
        .position(|line| line.starts_with("pub const LEAN_CORE_DECODER_V2"))
        .expect("the decoder constant")
        + 1;
    found.sort();
    assert_eq!(
        found,
        [("backend/core_v2.rs".to_owned(), decoder_line)],
        "the only Lean runtime text in the compiler is the decoder of core module 2"
    );
    // A planted runtime text is found wherever and however it is written:
    // raw or escaped, with a namespace line or only a qualified name, split
    // across literals or written over several lines, and in production code
    // that follows a test module.
    let semantic = std::fs::read_to_string(source_root.join("backend/semantic.rs").as_std_path())
        .expect("the semantic backend");
    assert_eq!(
        runtime_texts(&without_test_items(&semantic)),
        Vec::<usize>::new(),
        "the semantic backend carries no runtime text"
    );
    for (shape, plant) in [
        (
            "a raw literal",
            "const PLANTED: &str = r#\"\nnamespace LexLeanCollections\n\n@[expose] public def listFold {α σ : Type} (step : σ -> α -> σ) (initial : σ) (values : List α) : σ :=\n  values.foldl step initial\nend LexLeanCollections\n\"#;\n",
        ),
        (
            "an escaped literal",
            "const PLANTED: &str = \"\\nnamespace LexLeanModels\\npublic def Sound1 : Prop := True\\nend LexLeanModels\\n\";\n",
        ),
        (
            "a declaration under a runtime name and no namespace line",
            "const PLANTED: &str = \"public def LexLeanRuntime.listFold {α : Type} (xs : List α) : Nat := xs.length\\n\";\n",
        ),
        (
            "a theorem under a runtime name",
            "const PLANTED: &str = \"theorem LexLeanReasoning.sound1 : True := trivial\";\n",
        ),
        (
            "a namespace opened and closed with nothing declared in it",
            "const PLANTED: &str = \"namespace LexLeanModels\\nend LexLeanModels\\n\";\n",
        ),
        (
            "literals joined by concat!",
            "const PLANTED: &str = concat!(\n    \"namespace LexLeanCollections\\n\",\n    \"public def listFold : Nat := 0\\n\",\n    \"end LexLeanCollections\\n\"\n);\n",
        ),
        (
            "a name cut inside its namespace",
            "const PLANTED: &str = concat!(\"public def LexLean\", \"Models.Sound1 : Prop := True\");\n",
        ),
        (
            "an array of lines",
            "const PLANTED: [&str; 2] = [\"namespace LexLeanRuntime\", \"public def x : Nat := 0\"];\n",
        ),
        (
            "a format! template",
            "fn planted(name: &str) -> String {\n    format!(\"public def LexLeanReasoning.{name} : Nat := 0\")\n}\n",
        ),
        (
            "a literal continued over lines",
            "const PLANTED: &str = \"namespace LexLeanModels\\n\\\n    public def Sound1 : Prop := True\\n\";\n",
        ),
    ] {
        let planted = format!("{semantic}\n{plant}");
        assert_eq!(
            runtime_texts(&without_test_items(&planted)).len(),
            1,
            "a planted runtime text is found ({shape}): {plant}"
        );
    }
    // Production code after a test module is read: a text planted after the
    // `HEAVY_FRACTION` constant of the verifier, which follows its first test
    // module, is found, and the finding names a line below that module.
    let verifier = std::fs::read_to_string(source_root.join("verify/mod.rs").as_std_path())
        .expect("the verifier");
    let first_test = verifier
        .lines()
        .position(|line| line.starts_with("#[cfg(test)]"))
        .expect("the verifier has a unit test module")
        + 1;
    let after = verifier
        .find("const HEAVY_FRACTION")
        .and_then(|at| verifier[at..].find('\n').map(|end| at + end + 1))
        .expect("the verifier holds HEAVY_FRACTION");
    let at_line = verifier[..after].matches('\n').count() + 1;
    assert!(
        at_line > first_test,
        "HEAVY_FRACTION follows the first unit test module of the verifier"
    );
    let planted = format!(
        "{}const PLANTED: &str = \"public def LexLeanRuntime.listFold : Nat := 0\";\n{}",
        &verifier[..after],
        &verifier[after..]
    );
    assert_eq!(
        runtime_texts(&without_test_items(&planted)),
        [at_line],
        "a runtime text after a test module is found, on its own line"
    );
    // A unit test's fixture is not what the compiler emits: the same text
    // inside a test item is not found, and the production code after it is.
    let in_test = format!(
        "{semantic}\n#[cfg(test)]\nmod planted_tests {{\n    const FIXTURE: &str = \"namespace LexLeanRuntime\\npublic def x : Nat := 0\\n\";\n    fn nested() {{ if true {{ let _ = 1; }} }}\n}}\nconst PRODUCTION: &str = \"namespace LexLeanRuntime\\npublic def y : Nat := 0\\n\";\n"
    );
    let found_after_test = runtime_texts(&without_test_items(&in_test));
    assert_eq!(
        found_after_test.len(),
        1,
        "a test item's fixture is skipped and the production text after it is found"
    );
    // The statement's other half, over what the compiler generates: every
    // committed language-1.2 module imports the runtime modules whose
    // constants it names and declares none, and a module that omits an
    // import, or splices a declaration, is refused.
    let modules = runtime::modules().expect("the shipped runtime");
    let mut owners: std::collections::BTreeMap<String, &'static str> =
        std::collections::BTreeMap::new();
    for module in modules {
        for (name, _) in &module.declarations {
            owners.insert(name.clone(), module.name.as_str());
        }
    }
    assert!(!owners.is_empty(), "the runtime modules declare constants");
    let root = repo_root();
    let mut generated = 0usize;
    let mut using = None;
    for path in files(root.as_std_path(), &["lean"]) {
        let shown = path.display().to_string().replace('\\', "/");
        let is_output = shown.contains("/expected/build/modules/");
        let project = path
            .ancestors()
            .find(|ancestor| ancestor.join("lexlean.toml").is_file());
        let language_1_2 = project.is_some_and(|project| {
            std::fs::read_to_string(project.join("lexlean.toml"))
                .unwrap_or_default()
                .contains("language = \"1.2\"")
        });
        if !is_output || !language_1_2 {
            continue;
        }
        let text = std::fs::read_to_string(&path).expect("a generated module");
        generated += 1;
        assert_eq!(
            generated_module_defects(&text, &owners),
            Vec::<String>::new(),
            "{shown}: a generated language-1.2 module meets the statement"
        );
        if using.is_none()
            && text
                .split(|c: char| !(c.is_alphanumeric() || matches!(c, '_' | '.' | '\'')))
                .any(|token| owners.contains_key(token))
        {
            using = Some(text);
        }
    }
    assert!(
        generated > 0,
        "the repository commits generated language-1.2 modules"
    );
    let using = using.expect("a committed generated module uses a runtime constant");
    let without_import = using
        .lines()
        .filter(|line| {
            !line.starts_with("public import LexLeanRuntime.")
                && !line.starts_with("import LexLeanRuntime.")
        })
        .collect::<Vec<_>>()
        .join("\n");
    assert!(
        !generated_module_defects(&without_import, &owners).is_empty(),
        "a generated module that drops its runtime imports is refused"
    );
    let spliced = format!(
        "{using}\nnamespace LexLeanRuntime\npublic def planted : Nat := 0\nend LexLeanRuntime\n"
    );
    assert!(
        !generated_module_defects(&spliced, &owners).is_empty(),
        "a generated module that declares a runtime namespace is refused"
    );
}
