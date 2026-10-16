//! The fold laws of language 1.2 (SPEC.md §17.12, *Fold laws*), as linking
//! and the Lean backend read them.
//!
//! The laws are LexLean source: the native runtime module
//! `LexLeanRuntime.Laws`, whose rows pin their axioms like every runtime
//! module's. This registry, `language/runtime-1.2/laws.toml`, names what
//! the compiler uses of it: the ordered rewrite set of `linear_arithmetic`
//! and the laws a `law` proof applies, with the semantic types linking
//! checks an application against. It states no law: Lean checks each
//! application against the module's own statement, and the runtime build
//! refuses a registry that names a declaration the module does not have.

use std::sync::OnceLock;

use serde::Deserialize;

use super::SemanticType;

/// The registry's path; it lies in the language-1.2 partition of the
/// compiler semantics, so it is hashed with the module it names.
pub const REGISTRY_PATH: &str = "language/runtime-1.2/laws.toml";

/// The registry's tag.
pub const REGISTRY_SPEC: &str = "lexlean/runtime-laws/1";

/// One law a `law` proof applies by name.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Law {
    /// Its LexLean name, as a `law` proof writes it.
    pub name: String,
    /// The Lean theorem of the laws module it applies.
    pub lean: String,
    /// The explicit type parameters, in the theorem's binder order.
    pub type_parameters: Vec<String>,
    /// The explicit parameter types, in the theorem's binder order.
    pub parameters: Vec<SemanticType>,
    /// How many premises follow the parameters, each discharged by a prior
    /// theorem.
    pub premises: usize,
}

/// The parsed registry.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Registry {
    spec: String,
    /// The runtime module that declares every name below.
    pub module: String,
    /// The rewrite set of `linear_arithmetic`, in order.
    pub rewrites: Vec<String>,
    /// The laws a `law` proof applies.
    #[serde(rename = "law")]
    pub laws: Vec<Law>,
}

fn parse(text: &str) -> Result<Registry, String> {
    let registry: Registry =
        toml::from_str(text).map_err(|error| format!("{REGISTRY_PATH}: {error}"))?;
    if registry.spec != REGISTRY_SPEC {
        return Err(format!(
            "{REGISTRY_PATH}: spec `{}` is not `{REGISTRY_SPEC}`",
            registry.spec
        ));
    }
    let mut names = std::collections::BTreeSet::new();
    for law in &registry.laws {
        if !names.insert(law.name.as_str()) {
            return Err(format!(
                "{REGISTRY_PATH}: law `{}` is registered twice",
                law.name
            ));
        }
    }
    Ok(registry)
}

/// Check a registry read from `text` instead of the embedded one: the
/// conformance gate plants defects this way.
///
/// # Errors
///
/// Returns the first reason the registry is refused.
pub fn check_registry(text: &str) -> Result<Registry, String> {
    parse(text)
}

/// The shipped registry.
///
/// # Errors
///
/// Returns why the embedded registry is missing or malformed: a broken
/// build, which callers report as an internal error.
pub fn registry() -> Result<&'static Registry, String> {
    static REGISTRY: OnceLock<Result<Registry, String>> = OnceLock::new();
    REGISTRY
        .get_or_init(|| {
            let text = crate::embedded::FILES
                .iter()
                .find(|(path, _)| *path == REGISTRY_PATH)
                .and_then(|(_, bytes)| std::str::from_utf8(bytes).ok())
                .ok_or_else(|| format!("embedded `{REGISTRY_PATH}` is missing"))?;
            parse(text)
        })
        .as_ref()
        .map_err(Clone::clone)
}

/// The registered law named `name`.
///
/// # Errors
///
/// Returns why the registry cannot be read, or that no law has the name.
pub fn law(name: &str) -> Result<&'static Law, String> {
    registry()?
        .laws
        .iter()
        .find(|law| law.name == name)
        .ok_or_else(|| format!("`{name}` is not a registered law"))
}
