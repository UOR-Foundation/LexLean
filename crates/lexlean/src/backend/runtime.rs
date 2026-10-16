//! The runtime of generated Lean (SPEC.md §17.12): the portable primitives,
//! the ordered collections, the model predicates, and the reasoning lemmas a
//! generated module applies.
//!
//! For language 1.2 the runtime is LexLean source: native core modules
//! (§17.10) of the embedded project `library/`, which every generated module
//! that uses a part of it imports, instead of a text the backend splices. Its
//! Lean is generated from that source by the compiler. The definitions were
//! migrated losslessly from the elaborated texts the backend used to splice
//! (`tests/runtime-library/MIGRATION.md`), so each declaration carries its
//! exact axioms, and the verification of a project that imports a runtime
//! module audits them.
//!
//! Languages 1.0 and 1.1 are frozen: their generated output is byte-identical
//! to their released output, which spliced the portable runtime and the
//! decoder of core module 1 as text. Those bytes are the one declared
//! exception to "no handwritten Lean": they are held as data, `frozen/`, read
//! by the 1.0 and 1.1 backends only, and tested against the released output.

use std::collections::BTreeSet;
use std::sync::OnceLock;

use serde::Deserialize;

use crate::code;
use crate::diagnostic::Diagnostic;

/// The runtime's module list, hashed into the language-1.2 compiler
/// semantics with the sources it names.
pub const REGISTRY_PATH: &str = "language/runtime-1.2/library.toml";

/// The registry's tag.
pub const REGISTRY_SPEC: &str = "lexlean/runtime-library/1";

/// The embedded project of the runtime; its native sources are in `src/`.
pub const PROJECT_DIR: &str = "language/runtime-1.2/library";

/// The module prefix of the runtime project.
pub const PREFIX: &str = "LexLeanRuntime";

/// The portable primitives (`LexLeanRuntime`): checked fixed-width
/// arithmetic, byte strings, and decimal text.
pub const PORTABLE: &str = "LexLeanRuntime.Portable";

/// The ordered collections (`LexLeanCollections`).
pub const COLLECTIONS: &str = "LexLeanRuntime.Collections";

/// The model predicates (`LexLeanModels`).
pub const MODELS: &str = "LexLeanRuntime.Models";

/// The reasoning propositions and lemmas (`LexLeanReasoning`).
pub const REASONING: &str = "LexLeanRuntime.Reasoning";

/// The fold laws (`LexLeanLaws`): the bridges and laws `linear_arithmetic`
/// rewrites with and a `law` proof applies.
pub const LAWS: &str = "LexLeanRuntime.Laws";

/// The released bytes of the language-1.1 portable runtime, which a
/// language-1.1 semantic module splices inside its namespace.
pub const FROZEN_PORTABLE_PATH: &str = "language/runtime-1.2/frozen/portable-runtime-1.1.txt";

/// The released bytes of the decoder of core module 1, which a language-1.0
/// or 1.1 core module splices after its header.
pub const FROZEN_CORE_DECODER_PATH: &str = "language/runtime-1.2/frozen/core-decoder-1.txt";

/// The parsed runtime registry.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
struct Registry {
    spec: String,
    toolchain: String,
    /// The runtime's modules in dependency order.
    modules: Vec<String>,
}

/// One runtime module: its Lean, generated from its native source, and the
/// declarations an audit names, with their exact axioms.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeModule {
    /// Its Lean module name.
    pub name: String,
    /// The runtime modules it imports.
    pub imports: Vec<String>,
    /// Its generated Lean.
    pub text: String,
    /// Every declaration a Lean command can name, with its exact axioms,
    /// sorted.
    pub declarations: Vec<(String, Vec<String>)>,
}

fn internal(reason: impl std::fmt::Display) -> Diagnostic {
    Diagnostic::new(code!("LLI9001"), format!("phase runtime: {reason}"))
}

fn embedded_text(path: &str) -> Result<&'static str, String> {
    crate::embedded::FILES
        .iter()
        .find(|(candidate, _)| *candidate == path)
        .and_then(|(_, bytes)| std::str::from_utf8(bytes).ok())
        .ok_or_else(|| format!("embedded `{path}` is missing"))
}

/// The path of a runtime module's native source.
#[must_use]
pub fn native_source_path(module: &str) -> Option<String> {
    let relative = module.strip_prefix(&format!("{PREFIX}."))?;
    Some(format!(
        "{PROJECT_DIR}/src/{}.lex.tex",
        relative.replace('.', "/")
    ))
}

fn registry() -> Result<Registry, String> {
    let text = embedded_text(REGISTRY_PATH)?;
    let registry: Registry =
        toml::from_str(text).map_err(|error| format!("{REGISTRY_PATH}: {error}"))?;
    if registry.spec != REGISTRY_SPEC {
        return Err(format!(
            "{REGISTRY_PATH}: spec `{}` is not `{REGISTRY_SPEC}`",
            registry.spec
        ));
    }
    if registry.toolchain != crate::LEAN_TOOLCHAIN {
        return Err(format!(
            "{REGISTRY_PATH}: toolchain `{}` is not the pinned `{}`",
            registry.toolchain,
            crate::LEAN_TOOLCHAIN
        ));
    }
    Ok(registry)
}

/// Build the runtime from its module list and the native sources it names:
/// every module parsed, its imports closed over `Init` and the earlier
/// runtime modules, every declaration pinned to its exact axioms, its Lean
/// generated.
fn build(read: &dyn Fn(&str) -> Result<String, String>) -> Result<Vec<RuntimeModule>, String> {
    let registry = registry()?;
    let mut modules: Vec<RuntimeModule> = Vec::new();
    let mut names = BTreeSet::new();
    for module in &registry.modules {
        let path = native_source_path(module)
            .ok_or_else(|| format!("{REGISTRY_PATH}: `{module}` is outside `{PREFIX}`"))?;
        let source = read(&path)?;
        let core = crate::ir::core::CoreModule::from_source(&source, "1.2")
            .map_err(|reason| format!("{path}: {reason}"))?;
        let mut imports = Vec::new();
        for import in &core.imports {
            if import == "Init" {
                continue;
            }
            if !modules.iter().any(|earlier| earlier.name == *import) {
                return Err(format!(
                    "{path}: imports `{import}`, which is neither `Init` nor an earlier runtime module"
                ));
            }
            imports.push(import.clone());
        }
        let mut declarations = Vec::new();
        for row in &core.declarations {
            if row.generated {
                continue;
            }
            if !names.insert(row.name.clone()) {
                return Err(format!("{path}: `{}` is declared twice", row.name));
            }
            if !matches!(row.policy, crate::ir::core::CoreAxiomPolicy::Exact(_)) {
                return Err(format!(
                    "{path}: `{}` does not pin its axioms exactly",
                    row.name
                ));
            }
            // As for the preservation library (§17.17): only a name of plain
            // identifier segments can be audited by name; a private or escaped
            // auxiliary is covered by every declaration that uses it.
            if crate::ir::core::is_closed_lean_name(&row.name) {
                let mut axioms = row.policy.axioms().to_vec();
                axioms.sort();
                declarations.push((row.name.clone(), axioms));
            }
        }
        declarations.sort();
        let text = crate::backend::core::lean_text(&core)
            .map_err(|diagnostic| format!("{path}: {}", diagnostic.message))?;
        modules.push(RuntimeModule {
            name: module.clone(),
            imports,
            text,
            declarations,
        });
    }
    check_laws(&modules, crate::ir::semantic::laws::registry()?)?;
    Ok(modules)
}

/// The fold-law registry names only declarations of its module: every
/// rewrite and every law is a theorem `linear_arithmetic` or `law` can
/// cite.
fn check_laws(
    modules: &[RuntimeModule],
    registry: &crate::ir::semantic::laws::Registry,
) -> Result<(), String> {
    let path = crate::ir::semantic::laws::REGISTRY_PATH;
    let module = modules
        .iter()
        .find(|module| module.name == registry.module)
        .ok_or_else(|| format!("{path}: `{}` is not a runtime module", registry.module))?;
    for name in registry
        .rewrites
        .iter()
        .chain(registry.laws.iter().map(|law| &law.lean))
    {
        if !module
            .declarations
            .iter()
            .any(|(declared, _)| declared == name)
        {
            return Err(format!(
                "{path}: `{name}` is not a declaration of `{}`",
                registry.module
            ));
        }
    }
    Ok(())
}

/// Check a fold-law registry read from `text` against the shipped runtime:
/// the conformance gate plants defects in the registry this way.
///
/// # Errors
///
/// Returns the first reason the registry is refused.
pub fn check_laws_registry(text: &str) -> Result<(), String> {
    let registry = crate::ir::semantic::laws::check_registry(text)?;
    let modules = modules().map_err(|diagnostic| diagnostic.message)?;
    check_laws(modules, &registry)
}

/// Check a runtime read from `read` instead of the embedded one: the
/// conformance gate plants defects in the sources this way. Returns the
/// number of declarations an audit names.
///
/// # Errors
///
/// Returns the first reason the runtime is refused.
pub fn check_native_sources(
    read: &dyn Fn(&str) -> Result<String, String>,
) -> Result<usize, String> {
    build(read).map(|modules| modules.iter().map(|module| module.declarations.len()).sum())
}

/// The shipped runtime modules in dependency order.
///
/// # Errors
///
/// Returns `LLI9001` when the shipped registry or a native module is missing
/// or malformed.
pub fn modules() -> Result<&'static [RuntimeModule], Diagnostic> {
    static MODULES: OnceLock<Result<Vec<RuntimeModule>, String>> = OnceLock::new();
    MODULES
        .get_or_init(|| build(&|path| embedded_text(path).map(str::to_owned)))
        .as_ref()
        .map(Vec::as_slice)
        .map_err(internal)
}

/// The runtime modules a set of generated Lean texts imports, closed under
/// the runtime's own imports, in dependency order.
///
/// # Errors
///
/// Returns `LLI9001` when the shipped runtime cannot be read.
pub fn imported_by<'a>(
    texts: impl IntoIterator<Item = &'a str>,
) -> Result<Vec<&'static RuntimeModule>, Diagnostic> {
    let modules = modules()?;
    let mut wanted: BTreeSet<&str> = BTreeSet::new();
    for text in texts {
        for line in text.lines() {
            let Some(import) = line
                .strip_prefix("public import ")
                .or_else(|| line.strip_prefix("import "))
                .map(str::trim)
            else {
                continue;
            };
            if let Some(module) = modules.iter().find(|module| module.name == import) {
                wanted.insert(module.name.as_str());
            }
        }
    }
    // The registry is in dependency order, so one pass from the last module
    // to the first closes the set.
    for module in modules.iter().rev() {
        if wanted.contains(module.name.as_str()) {
            for import in &module.imports {
                if let Some(imported) = modules.iter().find(|candidate| candidate.name == *import) {
                    wanted.insert(imported.name.as_str());
                }
            }
        }
    }
    Ok(modules
        .iter()
        .filter(|module| wanted.contains(module.name.as_str()))
        .collect())
}

/// Whether `name` is the Lean module of a shipped runtime module.
#[must_use]
pub fn is_runtime_module(name: &str) -> bool {
    modules().is_ok_and(|modules| modules.iter().any(|module| module.name == name))
}

fn frozen(path: &str) -> &'static str {
    // The frozen bytes are embedded with the rest of `language/`; a build
    // without them is not this compiler (`build.rs` refuses a missing
    // normative directory), so their absence is a broken build, not input.
    embedded_text(path).unwrap_or_else(|reason| panic!("{reason}"))
}

/// The released language-1.1 portable runtime, which a language-1.1
/// semantic module splices inside its namespace (the frozen exception).
#[must_use]
pub fn frozen_portable_runtime() -> &'static str {
    frozen(FROZEN_PORTABLE_PATH)
}

/// The released decoder of core module 1, which a language-1.0 or 1.1 core
/// module splices after its header (the frozen exception).
#[must_use]
pub fn frozen_core_decoder() -> &'static str {
    frozen(FROZEN_CORE_DECODER_PATH)
}

#[cfg(test)]
mod tests {
    #[test]
    fn the_shipped_runtime_is_native_and_pins_every_axiom() {
        let modules = super::modules().expect("the shipped runtime");
        let names: Vec<&str> = modules.iter().map(|module| module.name.as_str()).collect();
        assert_eq!(
            names,
            [
                super::PORTABLE,
                super::COLLECTIONS,
                super::MODELS,
                super::REASONING,
                super::LAWS
            ]
        );
        for imported_by_both in [super::REASONING, super::LAWS] {
            let module = modules
                .iter()
                .find(|module| module.name == imported_by_both)
                .expect("a registered module");
            assert_eq!(module.imports, [super::PORTABLE, super::COLLECTIONS]);
        }
        assert!(modules.iter().all(|module| !module.declarations.is_empty()));
    }

    #[test]
    fn an_importer_brings_the_runtime_modules_it_depends_on() {
        let closure = super::imported_by([
            "module\npublic import Init\npublic import LexLeanRuntime.Reasoning\n",
        ])
        .expect("the shipped runtime");
        let names: Vec<&str> = closure.iter().map(|module| module.name.as_str()).collect();
        assert_eq!(
            names,
            [super::PORTABLE, super::COLLECTIONS, super::REASONING]
        );
    }
}
