//! Conformance cases for the language-1.2 text components (SPEC.md §17.10,
//! §17.12).

use std::collections::BTreeSet;

use lexlean::text_laws;

use crate::support::{self, repo_root};

/// Run the case for one language-models ID.
///
/// # Panics
///
/// Panics when the case's assertion fails, and for an ID of this suite whose
/// case is not yet wired — an unimplemented registered capability must not
/// pass silently.
pub fn run(id: &str) {
    match id {
        "LM-13" => lm_13(),
        _ => panic!("no language-models conformance case is wired for {id}"),
    }
}

/// LM-13 (§17.10, §17.12): the text laws are LexLean source.
///
/// The module that states them is one native core module of the embedded
/// project, the compiler embeds it and generates its Lean, and no
/// independently authored Lean implementation of the laws exists in the
/// release tree: the project holds no `.lean` file, no `.lean` file anywhere
/// declares the library's namespace or lies under its module root, and the
/// one-time migration input is absent. The registry lists exactly the module
/// the elaboration imports, so the seam cannot drift from the data.
fn lm_13() {
    let library = text_laws::library().expect("the shipped registry");
    assert_eq!(
        library.modules,
        vec![text_laws::LAWS_MODULE.to_owned()],
        "the registry lists exactly the module the elaboration imports"
    );
    // The laws are LexLean source: no independently authored Lean
    // implementation of them exists in the release tree.
    support::assert_library_has_no_handwritten_lean(text_laws::PREFIX, "tests/text-laws/oracle");
    // The library project is one native module per listed module and
    // nothing else: no source without a module and no module without a
    // source.
    let project = repo_root().join(text_laws::LIBRARY_DIR);
    let mut sources = BTreeSet::new();
    let mut stack = vec![project.as_std_path().to_path_buf()];
    while let Some(directory) = stack.pop() {
        for entry in std::fs::read_dir(&directory).expect("the text library project") {
            let path = entry.expect("an entry").path();
            if path.is_dir() {
                stack.push(path);
                continue;
            }
            let name = path.to_string_lossy().into_owned();
            assert!(
                !name.ends_with(".lean"),
                "{name}: an independently authored Lean file lies in the text library project"
            );
            // Compared as path components: a Windows checkout mixes the
            // separators of the repository root and of the library's
            // relative directory.
            if let Some(relative) = path
                .strip_prefix(project.as_std_path().join("src"))
                .ok()
                .map(|rest| rest.to_string_lossy().replace('\\', "/"))
                .and_then(|rest| rest.strip_suffix(".lex.tex").map(str::to_owned))
            {
                sources.insert(format!(
                    "{}.{}",
                    text_laws::PREFIX,
                    relative.replace('/', ".")
                ));
            }
        }
    }
    let listed: BTreeSet<String> = library.modules.iter().cloned().collect();
    assert_eq!(sources, listed, "the native sources are the listed modules");
    // The registry lists the module and the toolchain pin and nothing else:
    // every declaration and its exact axioms are the native module's own
    // data, read from it and never re-stated here.
    let registry: toml::Table =
        std::fs::read_to_string(repo_root().join(text_laws::LIBRARY_PATH).as_std_path())
            .expect("the registry")
            .parse()
            .expect("the registry is TOML");
    assert_eq!(
        registry.keys().map(String::as_str).collect::<Vec<_>>(),
        ["modules", "spec", "toolchain"],
        "the registry holds no declaration list"
    );
    // Every source is embedded, so it is hashed into the language-1.2
    // semantics and is what the compiler generates the Lean from.
    for module in &library.modules {
        let path = text_laws::native_source_path(module).expect("a path");
        assert!(
            lexlean::embedded::FILES
                .iter()
                .any(|(candidate, _)| *candidate == path),
            "{path} is not embedded in the compiler"
        );
    }
}
