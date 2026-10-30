//! The axiom audit of imported declarations (SPEC.md §4.2, §5.3, §5.4,
//! §18.8, §22.6).
//!
//! An *imported declaration* is the Lean constant a used external entry
//! denotes, when the Lean module that declares it is not one of the pinned
//! toolchain's own libraries. The probe module prints each one's axioms with
//! `#print axioms`, which is what §5.4 defines the axiom audit to be, in the
//! process that already imports it; nothing here starts Lean. This module
//! decides what the printed sets mean: which axioms an import may not carry
//! unless a policy of the document names them, and how an import is
//! described in the attestation.

use std::collections::BTreeSet;

use crate::artifact::canonical_json::Json;
use crate::code;
use crate::diagnostic::Diagnostic;
use crate::link::CheckedProject;

/// The status of an imported declaration (SPEC.md §5.3). The declaration
/// comes from the locked workspace, which the pinned toolchain loads; LexLean
/// did not prove it, so the words `proved`, `verified` and `kernel-checked`
/// are never used for it (§4.2).
pub const IMPORTED_STATUS: &str = "replayed in the pinned environment";

/// Module roots whose declarations are not imports: the libraries the pinned
/// toolchain ships (trusted as the toolchain itself, §5.1 items 2 and 3), and
/// the runtime modules the project generates, whose declarations each pin
/// their axioms exactly (§17.12) and are audited as the project's own.
const OWN_ROOTS: [&str; 5] = ["Init", "Std", "Lean", "Lake", "LexLeanRuntime"];

/// Whether a Lean module is an import in the sense of this module.
#[must_use]
pub fn is_imported_module(module: &str) -> bool {
    let root = module.split('.').next().unwrap_or(module);
    !OWN_ROOTS.contains(&root)
}

/// The axioms an import may not carry unless a policy names them.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Forbidden {
    /// `sorryAx`: a proof hole.
    Sorry,
    /// `Lean.ofReduceBool`: a claim the compiler's evaluator made.
    ReduceBool,
    /// `Lean.trustCompiler`, which `Lean.ofReduceBool` rests on, and the
    /// axiom `native_decide` generates for each use, which the pinned Lean
    /// names `<declaration>._native.native_decide.ax_<n>_<m>`.
    TrustCompiler,
}

impl Forbidden {
    /// The class of an axiom, when it is one of these.
    #[must_use]
    pub fn of(axiom: &str) -> Option<Self> {
        match axiom {
            "sorryAx" => Some(Self::Sorry),
            "Lean.ofReduceBool" => Some(Self::ReduceBool),
            "Lean.trustCompiler" => Some(Self::TrustCompiler),
            _ if axiom.contains("._native.native_decide.") => Some(Self::TrustCompiler),
            _ => None,
        }
    }

    fn what(self) -> &'static str {
        match self {
            Self::Sorry => "a proof hole",
            Self::ReduceBool => "a claim the compiler's evaluator made",
            Self::TrustCompiler => "trust in the compiler",
        }
    }
}

/// One imported declaration the probe audited.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImportedDeclaration {
    /// The qualified entry ID that denotes it.
    pub entry: String,
    /// The Lean module that declares it.
    pub module: String,
    /// Its fully qualified Lean name.
    pub name: String,
    /// The axioms `#print axioms` reported, sorted.
    pub observed: Vec<String>,
}

/// The axioms the document's declarations name in an `allow` or `exact`
/// policy.
#[must_use]
pub fn named_axioms(checked: &CheckedProject) -> BTreeSet<String> {
    let mut named = BTreeSet::new();
    for module in checked.modules.values() {
        let document = &module.document;
        for declaration in document.declarations() {
            named.extend(declaration.policy.axioms().iter().cloned());
        }
        if let Some(core) = &document.core {
            for declaration in &core.declarations {
                named.extend(declaration.policy.axioms().iter().cloned());
            }
        }
        if let Some(semantic) = &document.semantic {
            for declaration in &semantic.declarations {
                named.extend(declaration.axioms().iter().cloned());
            }
        }
    }
    named
}

/// Refuse the first import that depends on a forbidden axiom no policy of the
/// document names, with the diagnostic of that axiom's class.
///
/// The granularity is the document: a policy anywhere in it that names the
/// axiom is enough here, and the declaration that actually depends on the
/// import is held to its own policy afterwards (§22.6), so an import named
/// by one declaration's policy and used by another's is still refused, later
/// and as `LLV7005`.
///
/// # Errors
///
/// `LLV7023` for `sorryAx`, `LLV7024` for `Lean.ofReduceBool` and `LLV7025`
/// for `Lean.trustCompiler` and the axioms of `native_decide`.
pub fn audit(imports: &[ImportedDeclaration], named: &BTreeSet<String>) -> Result<(), Diagnostic> {
    for import in imports {
        for class in [
            Forbidden::Sorry,
            Forbidden::ReduceBool,
            Forbidden::TrustCompiler,
        ] {
            let unnamed: Vec<&str> = import
                .observed
                .iter()
                .map(String::as_str)
                .filter(|axiom| Forbidden::of(axiom) == Some(class) && !named.contains(*axiom))
                .collect();
            let Some(first) = unnamed.first() else {
                continue;
            };
            let message = format!(
                "imported declaration `{}` of entry `{}` (module `{}`) depends on `{first}`, {}, and no axiom policy of the document names it",
                import.name,
                import.entry,
                import.module,
                class.what(),
            );
            let diagnostic = match class {
                Forbidden::Sorry => Diagnostic::new(code!("LLV7023"), message),
                Forbidden::ReduceBool => Diagnostic::new(code!("LLV7024"), message),
                Forbidden::TrustCompiler => Diagnostic::new(code!("LLV7025"), message),
            };
            return Err(diagnostic.with_note(format!(
                "observed axioms of `{}`: [{}]; name each forbidden one in an `\\allowaxioms` or `\\exactaxioms` policy to accept the import",
                import.name,
                import.observed.join(", ")
            )));
        }
    }
    Ok(())
}

/// The attestation's `imports` member: one row per imported declaration, in
/// entry order. It is written only when there is one, so a project that
/// imports nothing outside the toolchain keeps the attestation it had.
#[must_use]
pub fn rows(imports: &[ImportedDeclaration]) -> Json {
    Json::Arr(
        imports
            .iter()
            .map(|import| {
                Json::object(vec![
                    ("entry", Json::Str(import.entry.clone())),
                    ("module", Json::Str(import.module.clone())),
                    ("name", Json::Str(import.name.clone())),
                    (
                        "observed",
                        Json::Arr(import.observed.iter().cloned().map(Json::Str).collect()),
                    ),
                    ("status", Json::Str(IMPORTED_STATUS.to_owned())),
                ])
            })
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn import(name: &str, observed: &[&str]) -> ImportedDeclaration {
        ImportedDeclaration {
            entry: "test.ax::e".to_owned(),
            module: "AxHost".to_owned(),
            name: name.to_owned(),
            observed: observed.iter().map(|axiom| (*axiom).to_owned()).collect(),
        }
    }

    fn named(axioms: &[&str]) -> BTreeSet<String> {
        axioms.iter().map(|axiom| (*axiom).to_owned()).collect()
    }

    fn code_of(result: Result<(), Diagnostic>) -> String {
        result.expect_err("refused").code.as_str().to_owned()
    }

    #[test]
    fn the_toolchain_libraries_are_not_imports() {
        for module in [
            "Init",
            "Init.Data.Nat",
            "Std.Data.HashMap",
            "Lean",
            "Lake.Build",
        ] {
            assert!(!is_imported_module(module), "{module}");
        }
        for module in [
            "Mathlib.Order.Basic",
            "Batteries",
            "AxHost",
            "InitX",
            "Atlas.Core",
        ] {
            assert!(is_imported_module(module), "{module}");
        }
    }

    #[test]
    fn each_forbidden_axiom_has_its_own_code() {
        let none = BTreeSet::new();
        assert_eq!(
            code_of(audit(&[import("a", &["sorryAx"])], &none)),
            "LLV7023"
        );
        assert_eq!(
            code_of(audit(&[import("a", &["Lean.ofReduceBool"])], &none)),
            "LLV7024"
        );
        assert_eq!(
            code_of(audit(&[import("a", &["Lean.trustCompiler"])], &none)),
            "LLV7025"
        );
        // The axiom `native_decide` generates, with the hygiene mark Lean
        // prints after it.
        assert_eq!(
            code_of(audit(
                &[import("a", &["a._native.native_decide.ax_1_1\u{271d}"])],
                &none
            )),
            "LLV7025"
        );
    }

    #[test]
    fn the_standard_axioms_are_not_forbidden() {
        let none = BTreeSet::new();
        audit(
            &[import("a", &["Classical.choice", "Quot.sound", "propext"])],
            &none,
        )
        .expect("the three standard axioms are for the declaration's policy, not for this audit");
    }

    #[test]
    fn a_policy_that_names_the_axiom_accepts_the_import() {
        audit(&[import("a", &["sorryAx"])], &named(&["sorryAx"])).expect("named");
        // `Lean.ofReduceBool` rests on `Lean.trustCompiler`: naming the first
        // alone leaves the second, which is refused in its own class.
        let both = import("a", &["Lean.ofReduceBool", "Lean.trustCompiler"]);
        assert_eq!(
            code_of(audit(
                std::slice::from_ref(&both),
                &named(&["Lean.ofReduceBool"])
            )),
            "LLV7025"
        );
        audit(
            &[both],
            &named(&["Lean.ofReduceBool", "Lean.trustCompiler"]),
        )
        .expect("both named");
    }

    #[test]
    fn the_refusal_names_the_entry_the_declaration_and_the_axiom() {
        let diagnostic = audit(&[import("AxHost.viaSorry", &["sorryAx"])], &BTreeSet::new())
            .expect_err("refused");
        for needle in ["test.ax::e", "AxHost.viaSorry", "sorryAx", "AxHost"] {
            assert!(
                diagnostic.message.contains(needle),
                "{}",
                diagnostic.message
            );
        }
    }

    #[test]
    fn the_status_never_claims_a_proof() {
        for word in ["proved", "verified", "kernel-checked"] {
            assert!(!IMPORTED_STATUS.contains(word));
        }
        let row = rows(&[import("a", &[])]).to_canonical_string();
        assert!(row.contains(IMPORTED_STATUS), "{row}");
    }
}
