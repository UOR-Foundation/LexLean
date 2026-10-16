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
            !(entry.file_type().is_dir() && matches!(name.as_ref(), "target" | ".git" | ".lexlean"))
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
    // the committed locks).
    assert!(
        support::spec_text().contains("`language/runtime-1.2/`"),
        "SPEC.md §21.2 excludes the runtime project and the frozen bytes from the language-1.0 and 1.1 IDs"
    );
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
}

/// The string literals of a Rust source, decoded: raw literals as written,
/// ordinary ones with their escapes resolved, with the line each opens on.
fn string_literals(source: &str) -> Vec<(usize, String)> {
    let bytes = source.as_bytes();
    let mut out = Vec::new();
    let mut index = 0;
    while index < bytes.len() {
        match bytes[index] {
            b'/' if bytes.get(index + 1) == Some(&b'/') => {
                while index < bytes.len() && bytes[index] != b'\n' {
                    index += 1;
                }
            }
            b'\'' => {
                // A character literal or a lifetime: skip `'x'`, `'\x'`.
                if bytes.get(index + 2) == Some(&b'\'') {
                    index += 3;
                } else if bytes.get(index + 1) == Some(&b'\\') {
                    index += 2;
                    while index < bytes.len() && bytes[index] != b'\'' {
                        index += 1;
                    }
                    index += 1;
                } else {
                    index += 1;
                }
            }
            b'r' if matches!(bytes.get(index + 1), Some(b'#' | b'"'))
                && (index == 0
                    || !(bytes[index - 1].is_ascii_alphanumeric() || bytes[index - 1] == b'_')) =>
            {
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
                let start = at + 1;
                let end = source[start..]
                    .find(&closing)
                    .map_or(bytes.len(), |end| start + end);
                out.push((
                    source[..index].matches('\n').count() + 1,
                    source[start..end].to_owned(),
                ));
                index = end + closing.len();
            }
            b'"' => {
                let line = source[..index].matches('\n').count() + 1;
                let mut text = String::new();
                index += 1;
                while index < bytes.len() && bytes[index] != b'"' {
                    if bytes[index] == b'\\' && index + 1 < bytes.len() {
                        match bytes[index + 1] {
                            b'n' => text.push('\n'),
                            b't' => text.push('\t'),
                            other => text.push(char::from(other)),
                        }
                        index += 2;
                    } else {
                        let width = source[index..].chars().next().map_or(1, char::len_utf8);
                        text.push_str(&source[index..index + width]);
                        index += width;
                    }
                }
                out.push((line, text));
                index += 1;
            }
            _ => index += 1,
        }
    }
    out
}

/// Whether a text is Lean runtime text: a `namespace` and, within it, a
/// declaration.
fn is_runtime_text(text: &str) -> bool {
    let declaration = |line: &str| {
        let mut rest = line.trim_start();
        if rest.starts_with("@[") {
            rest = rest
                .split_once(']')
                .map_or("", |(_, after)| after)
                .trim_start();
        }
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
        [
            "def ",
            "theorem ",
            "instance ",
            "class ",
            "inductive ",
            "structure ",
            "abbrev ",
        ]
        .iter()
        .any(|keyword| rest.starts_with(keyword))
    };
    text.lines().any(|line| line.starts_with("namespace ")) && text.lines().any(declaration)
}

/// The runtime texts of a Rust source, by line.
fn runtime_texts(source: &str) -> Vec<usize> {
    string_literals(source)
        .into_iter()
        .filter(|(_, text)| is_runtime_text(text))
        .map(|(line, _)| line)
        .collect()
}

/// RT-04: no Lean runtime text remains in the language-1.2 compiler but the
/// decoder of core module 2, which loads native modules.
fn rt_04() {
    let source_root = repo_root().join("crates/lexlean/src");
    let mut found = Vec::new();
    for path in files(source_root.as_std_path(), &["rs"]) {
        let text = std::fs::read_to_string(&path).expect("a compiler source");
        // A unit test's fixture text is no part of what the compiler emits.
        let text = text.split("\n#[cfg(test)]").next().unwrap_or_default();
        let relative = path
            .strip_prefix(source_root.as_std_path())
            .expect("below the source root")
            .to_string_lossy()
            .replace('\\', "/");
        for line in runtime_texts(text) {
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
    assert_eq!(
        found,
        [("backend/core_v2.rs".to_owned(), decoder_line)],
        "the only Lean runtime text in the compiler is the decoder of core module 2"
    );
    // A planted runtime text constant, raw or escaped, is found.
    for plant in [
        "const PLANTED: &str = r#\"\nnamespace LexLeanCollections\n\n@[expose] public def listFold {α σ : Type} (step : σ -> α -> σ) (initial : σ) (values : List α) : σ :=\n  values.foldl step initial\nend LexLeanCollections\n\"#;\n",
        "const PLANTED: &str = \"\\nnamespace LexLeanModels\\npublic def Sound1 : Prop := True\\nend LexLeanModels\\n\";\n",
    ] {
        let semantic = std::fs::read_to_string(source_root.join("backend/semantic.rs").as_std_path())
            .expect("the semantic backend");
        assert_eq!(runtime_texts(&semantic), Vec::<usize>::new(), "the semantic backend carries no runtime text");
        let planted = format!("{semantic}\n{plant}");
        assert_eq!(
            runtime_texts(&planted).len(),
            1,
            "a planted runtime text constant is found: {plant}"
        );
    }
}
