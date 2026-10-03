//! Production eligibility (SPEC.md §17.13): the closed registry of production
//! targets, effects, and construct dispositions, and the deterministic
//! eligibility report of every language-1.2 production root.
//!
//! Executable status is a checked property, not an implication of being
//! valid LexLean: a module with no production root is never analysed here,
//! and formal-only content coexists with executable roots because only the
//! computational closure of a root is ever inspected.

pub mod eligibility;
pub mod lcnf;

use std::collections::{BTreeMap, BTreeSet};
use std::sync::OnceLock;

use serde::Deserialize;

/// The embedded registry path, hashed into the language-1.2 compiler
/// semantics ID with every other language file.
pub const REGISTRY_PATH: &str = "language/production-1.2.toml";

/// The schema tag of an eligibility report.
pub const REPORT_SPEC: &str = "lexlean/production-eligibility/1";

/// A registered production target (machine profile).
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Target {
    pub id: String,
    pub statement: String,
    /// Whether the target provides heap allocation.
    pub allocation: bool,
    /// The width of the natural-number representation.
    pub natural_bits: u32,
    /// The width of the mathematical-integer representation.
    pub integer_bits: u32,
}

/// A registered effect a root may admit.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Effect {
    pub id: String,
    pub statement: String,
}

/// Whether a construct may be realized.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Disposition {
    /// Realized directly by every target, subject to its effects.
    Runtime,
    /// Never realized: reaching it from a root makes the root ineligible.
    FormalOnly,
    /// A proof-only dependency: recorded and erased, never realized.
    Erased,
}

impl Disposition {
    /// The registry spelling.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Runtime => "runtime",
            Self::FormalOnly => "formal-only",
            Self::Erased => "erased",
        }
    }
}

/// One construct row of the registry.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Construct {
    pub key: String,
    pub disposition: Disposition,
    /// The construct requires heap allocation.
    pub allocation: bool,
    /// The value representations (`nat`, `int`) whose results may exceed
    /// the target width.
    pub overflow: Vec<String>,
    /// The construct recurses to an input-bounded depth.
    pub recursion: bool,
    #[serde(default)]
    pub note: Option<String>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RegistryFile {
    spec: String,
    target: Vec<Target>,
    effect: Vec<Effect>,
    construct: Vec<Construct>,
}

/// The parsed registry.
#[derive(Debug)]
pub struct Registry {
    pub targets: BTreeMap<String, Target>,
    pub effects: BTreeMap<String, Effect>,
    pub constructs: BTreeMap<String, Construct>,
    /// Construct keys in registry order.
    pub order: Vec<String>,
}

fn parse_registry(text: &str) -> Result<Registry, String> {
    let file: RegistryFile =
        toml::from_str(text).map_err(|error| format!("{REGISTRY_PATH}: {error}"))?;
    if file.spec != "lexlean/production/1" {
        return Err(format!("{REGISTRY_PATH}: unsupported spec `{}`", file.spec));
    }
    let mut registry = Registry {
        targets: BTreeMap::new(),
        effects: BTreeMap::new(),
        constructs: BTreeMap::new(),
        order: Vec::new(),
    };
    for target in file.target {
        if target.natural_bits == 0
            || target.natural_bits > 64
            || target.integer_bits == 0
            || target.integer_bits > 64
        {
            return Err(format!(
                "{REGISTRY_PATH}: target `{}` has an unsupported width",
                target.id
            ));
        }
        if registry
            .targets
            .insert(target.id.clone(), target.clone())
            .is_some()
        {
            return Err(format!("{REGISTRY_PATH}: duplicate target `{}`", target.id));
        }
    }
    for effect in file.effect {
        if registry
            .effects
            .insert(effect.id.clone(), effect.clone())
            .is_some()
        {
            return Err(format!("{REGISTRY_PATH}: duplicate effect `{}`", effect.id));
        }
    }
    for construct in file.construct {
        for representation in &construct.overflow {
            if representation != "nat" && representation != "int" {
                return Err(format!(
                    "{REGISTRY_PATH}: construct `{}` names unknown representation `{representation}`",
                    construct.key
                ));
            }
        }
        registry.order.push(construct.key.clone());
        if registry
            .constructs
            .insert(construct.key.clone(), construct.clone())
            .is_some()
        {
            return Err(format!(
                "{REGISTRY_PATH}: duplicate construct `{}`",
                construct.key
            ));
        }
    }
    for required in ["allocation", "overflow", "recursion"] {
        if !registry.effects.contains_key(required) {
            return Err(format!(
                "{REGISTRY_PATH}: the effect `{required}` is not registered"
            ));
        }
    }
    Ok(registry)
}

/// The embedded registry.
///
/// # Errors
///
/// Returns the reason the embedded registry is missing or malformed; the
/// conformance suite parses it, so a malformed registry never ships.
pub fn registry() -> Result<&'static Registry, String> {
    static REGISTRY: OnceLock<Result<Registry, String>> = OnceLock::new();
    REGISTRY
        .get_or_init(|| {
            let text = crate::embedded::FILES
                .iter()
                .find(|(path, _)| *path == REGISTRY_PATH)
                .and_then(|(_, bytes)| std::str::from_utf8(bytes).ok())
                .ok_or_else(|| format!("embedded `{REGISTRY_PATH}` is missing"))?;
            parse_registry(text)
        })
        .as_ref()
        .map_err(Clone::clone)
}

fn strictly_ascending(values: &[String]) -> bool {
    values.windows(2).all(|pair| pair[0] < pair[1])
}

/// Check one production-root declaration against the registry: at least one
/// target, targets and effects sorted, unique, and registered.
///
/// # Errors
///
/// Returns the first violated rule.
pub fn check_declaration(name: &str, targets: &[String], effects: &[String]) -> Result<(), String> {
    let registry = registry()?;
    if targets.is_empty() {
        return Err(format!("production root `{name}` names no target"));
    }
    if !strictly_ascending(targets) {
        return Err(format!(
            "production root `{name}` targets are not sorted and unique"
        ));
    }
    if !strictly_ascending(effects) {
        return Err(format!(
            "production root `{name}` effects are not sorted and unique"
        ));
    }
    if let Some(unknown) = targets
        .iter()
        .find(|target| !registry.targets.contains_key(*target))
    {
        return Err(format!(
            "production root `{name}` names unregistered target `{unknown}`"
        ));
    }
    if let Some(unknown) = effects
        .iter()
        .find(|effect| !registry.effects.contains_key(*effect))
    {
        return Err(format!(
            "production root `{name}` names unregistered effect `{unknown}`"
        ));
    }
    Ok(())
}

/// Where a closure member sits relative to its root.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClosureMember {
    /// The instance key: the qualified Lean name, with type arguments when
    /// the declaration is generic.
    pub instance: String,
    /// The qualified Lean name of the declaration.
    pub declaration: String,
    /// The construct key of the declaration.
    pub construct: String,
    /// Canonical type-argument spellings.
    pub type_arguments: Vec<String>,
    /// The shortest call path from the root, root first.
    pub path: Vec<String>,
}

/// One effect of a root on one target with every construct site causing it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EffectRow {
    pub effect: String,
    /// `(construct, instance)` pairs, sorted and unique.
    pub sources: BTreeSet<(String, String)>,
}

/// A natural-number or integer representation crossing a root's boundary,
/// realized in the target's width: a caller supplies only values inside it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoundaryRow {
    /// `parameter <name>` or `result`.
    pub position: String,
    /// `nat` or `int`.
    pub representation: String,
    pub bits: u32,
}

/// The eligibility of one root on one target.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TargetRow {
    pub target: String,
    pub effects: Vec<EffectRow>,
    /// Boundary representations in parameter order, then the result.
    pub boundary: Vec<BoundaryRow>,
}

/// The complete eligibility record of one root.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RootReport {
    /// The qualified Lean name of the root.
    pub root: String,
    pub declared_effects: Vec<String>,
    /// Runtime closure members in instance order.
    pub runtime: Vec<ClosureMember>,
    /// Data types realized by the closure, with their construct keys.
    pub types: BTreeMap<String, String>,
    /// Proof-only dependencies, erased from the runtime closure.
    pub erased: BTreeSet<String>,
    /// Every construct the closure realizes, with the instances using it.
    pub constructs: BTreeMap<String, BTreeSet<String>>,
    pub targets: Vec<TargetRow>,
    /// Language 1.2 (reasoning): the explicit resource account of every
    /// reasoner the closure runs (§17.13).
    pub reasoning: Vec<ReasoningRow>,
}

/// The resources one reasoner in a root's closure accounts for: its
/// strategy and bounds, its rules in priority order, the counters of its
/// ledger, and the generated theorems that bound them (§17.12, §17.13).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReasoningRow {
    /// The qualified Lean name of the reasoner.
    pub reasoner: String,
    /// `forward`, `breadth_first`, `depth_first`, or `generate_and_verify`.
    pub strategy: String,
    /// Whether a search expands each state once.
    pub deduplicate: bool,
    /// The canonical semantic JSON of the iteration bound of a rule-based
    /// reasoner.
    pub fuel: Option<String>,
    /// The canonical semantic JSON of the check budget of a
    /// generate-and-verify reasoner.
    pub budget: Option<String>,
    /// The canonical semantic JSON of the frontier bound of a search.
    pub frontier: Option<String>,
    /// Its rules, qualified, in priority order.
    pub rules: Vec<String>,
    /// The counters of its ledger.
    pub ledger: Vec<String>,
    /// The generated theorems bounding the ledger, qualified.
    pub bounds: Vec<String>,
}

/// The eligibility report of one module.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModuleReport {
    pub module: String,
    pub roots: Vec<RootReport>,
}

fn strings(values: impl IntoIterator<Item = String>) -> serde_json::Value {
    serde_json::Value::Array(values.into_iter().map(serde_json::Value::String).collect())
}

impl ModuleReport {
    /// The canonical report value (`lexlean/production-eligibility/1`).
    ///
    /// # Panics
    ///
    /// Panics only if the registry failed to load after analysis succeeded,
    /// which analysis rules out.
    #[must_use]
    pub fn to_value(&self) -> serde_json::Value {
        let registry = registry().expect("analysis loaded the registry");
        let roots = self
            .roots
            .iter()
            .map(|root| {
                let mut value = serde_json::json!({
                    "root": root.root,
                    "declared_effects": strings(root.declared_effects.iter().cloned()),
                    "runtime_closure": root.runtime.iter().map(|member| serde_json::json!({
                        "instance": member.instance,
                        "declaration": member.declaration,
                        "construct": member.construct,
                        "type_arguments": strings(member.type_arguments.iter().cloned()),
                        "path": strings(member.path.iter().cloned()),
                    })).collect::<Vec<_>>(),
                    "types": root.types.iter().map(|(ty, construct)| serde_json::json!({
                        "type": ty,
                        "construct": construct,
                    })).collect::<Vec<_>>(),
                    "erased": strings(root.erased.iter().cloned()),
                    "constructs": root.constructs.iter().map(|(key, instances)| {
                        let disposition = registry
                            .constructs
                            .get(key)
                            .map_or("runtime", |row| row.disposition.as_str());
                        serde_json::json!({
                            "construct": key,
                            "disposition": disposition,
                            "instances": strings(instances.iter().cloned()),
                        })
                    }).collect::<Vec<_>>(),
                    "targets": root.targets.iter().map(|target| serde_json::json!({
                        "target": target.target,
                        "status": "eligible",
                        "boundary": target.boundary.iter().map(|row| serde_json::json!({
                            "position": row.position,
                            "representation": row.representation,
                            "bits": row.bits,
                        })).collect::<Vec<_>>(),
                        "effects": target.effects.iter().map(|effect| serde_json::json!({
                            "effect": effect.effect,
                            "sources": effect.sources.iter().map(|(construct, instance)| serde_json::json!({
                                "construct": construct,
                                "instance": instance,
                            })).collect::<Vec<_>>(),
                        })).collect::<Vec<_>>(),
                    })).collect::<Vec<_>>(),
                });
                if !root.reasoning.is_empty() {
                    value["reasoning"] = root
                        .reasoning
                        .iter()
                        .map(|row| {
                            let mut item = serde_json::json!({
                                "reasoner": row.reasoner,
                                "strategy": row.strategy,
                                "deduplicate": row.deduplicate,
                                "rules": strings(row.rules.iter().cloned()),
                                "ledger": strings(row.ledger.iter().cloned()),
                                "bounds": strings(row.bounds.iter().cloned()),
                            });
                            if let Some(fuel) = &row.fuel {
                                item["fuel"] = serde_json::Value::String(fuel.clone());
                            }
                            if let Some(budget) = &row.budget {
                                item["budget"] = serde_json::Value::String(budget.clone());
                            }
                            if let Some(frontier) = &row.frontier {
                                item["frontier"] = serde_json::Value::String(frontier.clone());
                            }
                            item
                        })
                        .collect();
                }
                value
            })
            .collect::<Vec<_>>();
        serde_json::json!({
            "spec": REPORT_SPEC,
            "module": self.module,
            "roots": roots,
        })
    }

    /// The canonical file bytes of the report.
    ///
    /// # Panics
    ///
    /// Panics only if `serde_json` produces text the canonical JSON parser
    /// rejects, which would be an internal invariant failure.
    #[must_use]
    pub fn to_file_bytes(&self) -> Vec<u8> {
        let text = serde_json::to_string(&self.to_value()).expect("report serializes");
        crate::artifact::canonical_json::Json::parse(text.as_bytes())
            .expect("report is JSON")
            .to_file_bytes()
    }
}
