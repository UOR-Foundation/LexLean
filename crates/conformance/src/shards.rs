//! The partition of one example's verification into the parts of one CI run
//! (SPEC.md §9.4): the committed plan, the import graph it is checked
//! against, and the checks.
//!
//! The plan is data, committed as `.github/verify-shards.json` and read by
//! `cargo xtask verify-examples` on each part's job and by `cargo xtask
//! ci-shards`, which checks it. It names one project and its shards: each
//! shard the shards it needs (whose `.olean` files it imports), the modules it
//! elaborates, and the modules it replays and audits. The shard named `join`
//! is the one that publishes; it needs every other shard, directly or through
//! another. The checks are the ones the verifier itself would fail on when a
//! part ran, made before any part runs: every module is elaborated by exactly
//! one shard and replayed by exactly one, every import of a module a shard
//! elaborates is elaborated by that shard or one it needs, a module a shard
//! replays is one whose `.olean` it has, and the needs form no cycle.
//!
//! The import graph is read from the example's committed `expected/build/
//! modules/`, the generated Lean the `golden` recipe holds equal to the
//! build: a module's `import` and `public import` lines that name another
//! module of the project.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

/// The committed plan, repository-relative.
pub const PLAN_FILE: &str = ".github/verify-shards.json";

/// The name of the shard that publishes.
pub const JOIN: &str = "join";

/// One shard of the plan.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Shard {
    /// The shards whose output this one imports.
    pub needs: Vec<String>,
    /// The full Lean module names this shard elaborates.
    pub elaborate: Vec<String>,
    /// The full Lean module names this shard replays and audits.
    pub replay: Vec<String>,
}

/// The plan: one project, its shards by name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Plan {
    /// The example directory name under `examples/`.
    pub project: String,
    /// The shards, by name.
    pub shards: BTreeMap<String, Shard>,
}

impl Plan {
    /// Parse the plan file's text.
    ///
    /// # Errors
    ///
    /// Text that is not the plan's shape.
    pub fn parse(text: &str) -> Result<Self, String> {
        let value: serde_json::Value =
            serde_json::from_str(text).map_err(|error| format!("{PLAN_FILE}: {error}"))?;
        let object = value
            .as_object()
            .ok_or_else(|| format!("{PLAN_FILE}: an object is required"))?;
        let project = object
            .get("project")
            .and_then(serde_json::Value::as_str)
            .ok_or_else(|| format!("{PLAN_FILE}: `project` is a required string"))?
            .to_owned();
        let shards_value = object
            .get("shards")
            .and_then(serde_json::Value::as_object)
            .ok_or_else(|| format!("{PLAN_FILE}: `shards` is a required object"))?;
        let strings =
            |shard: &str, value: &serde_json::Value, field: &str| -> Result<Vec<String>, String> {
                value
                    .get(field)
                    .and_then(serde_json::Value::as_array)
                    .ok_or_else(|| {
                        format!("{PLAN_FILE}: shard `{shard}`: `{field}` is a required array")
                    })?
                    .iter()
                    .map(|item| {
                        item.as_str().map(str::to_owned).ok_or_else(|| {
                            format!("{PLAN_FILE}: shard `{shard}`: `{field}` holds strings")
                        })
                    })
                    .collect()
            };
        let mut shards = BTreeMap::new();
        for (name, value) in shards_value {
            if let Some(unknown) = value
                .as_object()
                .map(|fields| {
                    fields
                        .keys()
                        .find(|key| !["needs", "elaborate", "replay"].contains(&key.as_str()))
                })
                .unwrap_or(None)
            {
                return Err(format!(
                    "{PLAN_FILE}: shard `{name}`: `{unknown}` is not a field of a shard"
                ));
            }
            shards.insert(
                name.clone(),
                Shard {
                    needs: strings(name, value, "needs")?,
                    elaborate: strings(name, value, "elaborate")?,
                    replay: strings(name, value, "replay")?,
                },
            );
        }
        if let Some(unknown) = object
            .keys()
            .find(|key| !["project", "shards"].contains(&key.as_str()))
        {
            return Err(format!(
                "{PLAN_FILE}: `{unknown}` is not a field of the plan"
            ));
        }
        Ok(Self { project, shards })
    }

    /// Read the committed plan of a repository.
    ///
    /// # Errors
    ///
    /// A plan file that cannot be read or parsed.
    pub fn read(root: &Path) -> Result<Self, String> {
        let path = root.join(PLAN_FILE);
        let text = std::fs::read_to_string(&path)
            .map_err(|error| format!("{}: {error}", path.display()))?;
        Self::parse(&text)
    }

    /// The shards a shard needs, directly or through another, in sorted
    /// order.
    ///
    /// # Errors
    ///
    /// A need that names no shard, or a cycle.
    pub fn needs_closure(&self, name: &str) -> Result<BTreeSet<String>, String> {
        let mut closure = BTreeSet::new();
        let mut stack: Vec<(String, Vec<String>)> = vec![(name.to_owned(), vec![name.to_owned()])];
        while let Some((shard, path)) = stack.pop() {
            let shard_plan = self.shards.get(&shard).ok_or_else(|| {
                format!("shard `{shard}` is needed by `{name}` and is not in the plan")
            })?;
            for need in &shard_plan.needs {
                if path.contains(need) {
                    return Err(format!(
                        "the needs of the shards form a cycle through `{need}`: {}",
                        path.join(" -> ")
                    ));
                }
                if closure.insert(need.clone()) {
                    let mut next = path.clone();
                    next.push(need.clone());
                    stack.push((need.clone(), next));
                }
            }
        }
        Ok(closure)
    }

    /// The modules whose `.olean` a shard has: the ones it elaborates and
    /// the ones the shards it needs elaborate.
    ///
    /// # Errors
    ///
    /// What [`Self::needs_closure`] reports.
    pub fn available(&self, name: &str) -> Result<BTreeSet<String>, String> {
        let shard = self
            .shards
            .get(name)
            .ok_or_else(|| format!("shard `{name}` is not in the plan"))?;
        let mut available: BTreeSet<String> = shard.elaborate.iter().cloned().collect();
        for need in self.needs_closure(name)? {
            available.extend(self.shards[&need].elaborate.iter().cloned());
        }
        Ok(available)
    }
}

/// The import graph of an example from its committed generated Lean: each
/// module's full Lean name to the modules of the project it imports.
///
/// # Errors
///
/// A modules directory that cannot be read.
pub fn import_graph(example_dir: &Path) -> Result<BTreeMap<String, Vec<String>>, String> {
    let modules_dir = example_dir.join("expected").join("build").join("modules");
    let mut sources: BTreeMap<String, String> = BTreeMap::new();
    for entry in walkdir::WalkDir::new(&modules_dir).sort_by_file_name() {
        let entry = entry.map_err(|error| format!("{}: {error}", modules_dir.display()))?;
        if !entry.file_type().is_file() || entry.path().extension().is_none_or(|ext| ext != "lean")
        {
            continue;
        }
        let relative = entry
            .path()
            .strip_prefix(&modules_dir)
            .map_err(|error| error.to_string())?
            .with_extension("");
        let name = relative
            .components()
            .map(|component| component.as_os_str().to_string_lossy().into_owned())
            .collect::<Vec<_>>()
            .join(".");
        let text = std::fs::read_to_string(entry.path())
            .map_err(|error| format!("{}: {error}", entry.path().display()))?;
        sources.insert(name, text);
    }
    if sources.is_empty() {
        return Err(format!("{}: no generated modules", modules_dir.display()));
    }
    let mut graph = BTreeMap::new();
    for (name, text) in &sources {
        let mut imports = Vec::new();
        for line in text.lines() {
            let line = line.trim();
            let imported = line
                .strip_prefix("public import ")
                .or_else(|| line.strip_prefix("import "))
                .map(str::trim);
            if let Some(imported) = imported {
                if sources.contains_key(imported) && !imports.contains(&imported.to_owned()) {
                    imports.push(imported.to_owned());
                }
            }
        }
        graph.insert(name.clone(), imports);
    }
    Ok(graph)
}

/// Check a plan against an import graph. The lines returned describe each
/// shard: what it needs, how many modules it elaborates and replays.
///
/// # Errors
///
/// The first check that fails, named.
pub fn check(plan: &Plan, graph: &BTreeMap<String, Vec<String>>) -> Result<Vec<String>, String> {
    let modules: BTreeSet<&String> = graph.keys().collect();
    let join = plan
        .shards
        .get(JOIN)
        .ok_or_else(|| format!("the plan has no shard named `{JOIN}`, the one that publishes"))?;
    let _ = join;
    let mut elaborated: BTreeMap<&String, &String> = BTreeMap::new();
    let mut replayed: BTreeMap<&String, &String> = BTreeMap::new();
    for (name, shard) in &plan.shards {
        for module in &shard.elaborate {
            if !modules.contains(module) {
                return Err(format!(
                    "shard `{name}` elaborates `{module}`, which is no module of the project"
                ));
            }
            if let Some(other) = elaborated.insert(module, name) {
                return Err(format!(
                    "`{module}` is elaborated by `{other}` and by `{name}`"
                ));
            }
        }
        for module in &shard.replay {
            if !modules.contains(module) {
                return Err(format!(
                    "shard `{name}` replays `{module}`, which is no module of the project"
                ));
            }
            if let Some(other) = replayed.insert(module, name) {
                return Err(format!(
                    "`{module}` is replayed by `{other}` and by `{name}`"
                ));
            }
        }
    }
    for module in &modules {
        if !elaborated.contains_key(module) {
            return Err(format!("`{module}` is elaborated by no shard"));
        }
        if !replayed.contains_key(module) {
            return Err(format!("`{module}` is replayed by no shard"));
        }
    }
    let mut lines = Vec::new();
    for (name, shard) in &plan.shards {
        let available = plan.available(name)?;
        for module in &shard.elaborate {
            for import in &graph[module] {
                if !available.contains(import) {
                    return Err(format!(
                        "shard `{name}` elaborates `{module}`, whose import `{import}` is elaborated by no shard it needs"
                    ));
                }
            }
        }
        for module in &shard.replay {
            if !available.contains(module) {
                return Err(format!(
                    "shard `{name}` replays `{module}`, whose olean comes from no shard it needs"
                ));
            }
            for import in &graph[module] {
                if !available.contains(import) {
                    return Err(format!(
                        "shard `{name}` replays `{module}`, whose import `{import}` is elaborated by no shard it needs"
                    ));
                }
            }
        }
        lines.push(format!(
            "{name}: needs {}; elaborates {}; replays {}",
            if shard.needs.is_empty() {
                "nothing".to_owned()
            } else {
                shard.needs.join(", ")
            },
            shard.elaborate.len(),
            shard.replay.len()
        ));
    }
    let others: BTreeSet<String> = plan
        .shards
        .keys()
        .filter(|name| *name != JOIN)
        .cloned()
        .collect();
    if plan.needs_closure(JOIN)? != others {
        return Err(format!(
            "`{JOIN}` must need every other shard, directly or through another"
        ));
    }
    Ok(lines)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn plan(text: &str) -> Plan {
        Plan::parse(text).expect("parses")
    }

    fn graph() -> BTreeMap<String, Vec<String>> {
        [("P.A", vec![]), ("P.B", vec!["P.A"]), ("P.C", vec!["P.B"])]
            .into_iter()
            .map(|(name, imports)| {
                (
                    name.to_owned(),
                    imports.into_iter().map(str::to_owned).collect(),
                )
            })
            .collect()
    }

    const GOOD: &str = r#"{"project":"p","shards":{
        "one":{"needs":[],"elaborate":["P.A"],"replay":[]},
        "two":{"needs":["one"],"elaborate":["P.B"],"replay":["P.A","P.B"]},
        "join":{"needs":["two"],"elaborate":["P.C"],"replay":["P.C"]}}}"#;

    #[test]
    fn a_plan_that_runs_every_process_once_in_order_passes() {
        let lines = check(&plan(GOOD), &graph()).expect("checks");
        assert_eq!(lines.len(), 3);
        assert_eq!(
            plan(GOOD).needs_closure("join").expect("closure"),
            ["one", "two"].into_iter().map(str::to_owned).collect()
        );
    }

    #[test]
    fn every_defect_of_a_plan_is_named() {
        let cases: [(&str, &str); 7] = [
            (
                r#"{"project":"p","shards":{"one":{"needs":[],"elaborate":["P.A","P.B"],"replay":["P.A","P.B"]},"join":{"needs":["one"],"elaborate":["P.C"],"replay":["P.C","P.A"]}}}"#,
                "replayed by `join` and by `one`",
            ),
            (
                r#"{"project":"p","shards":{"one":{"needs":[],"elaborate":["P.A"],"replay":["P.A"]},"join":{"needs":["one"],"elaborate":["P.C"],"replay":["P.B","P.C"]}}}"#,
                "`P.B` is elaborated by no shard",
            ),
            (
                r#"{"project":"p","shards":{"one":{"needs":[],"elaborate":["P.B"],"replay":["P.B"]},"join":{"needs":["one"],"elaborate":["P.A","P.C"],"replay":["P.A","P.C"]}}}"#,
                "whose import `P.A` is elaborated by no shard it needs",
            ),
            (
                r#"{"project":"p","shards":{"one":{"needs":[],"elaborate":["P.A"],"replay":["P.B"]},"join":{"needs":["one"],"elaborate":["P.B","P.C"],"replay":["P.A","P.C"]}}}"#,
                "whose olean comes from no shard it needs",
            ),
            (
                r#"{"project":"p","shards":{"one":{"needs":["two"],"elaborate":["P.A"],"replay":["P.A"]},"two":{"needs":["one"],"elaborate":["P.B"],"replay":["P.B"]},"join":{"needs":["two"],"elaborate":["P.C"],"replay":["P.C"]}}}"#,
                "form a cycle",
            ),
            (
                r#"{"project":"p","shards":{"one":{"needs":[],"elaborate":["P.A","P.B"],"replay":["P.A","P.B"]},"two":{"needs":[],"elaborate":[],"replay":[]},"join":{"needs":["one"],"elaborate":["P.C"],"replay":["P.C"]}}}"#,
                "must need every other shard",
            ),
            (
                r#"{"project":"p","shards":{"one":{"needs":[],"elaborate":["P.A","P.B","P.C"],"replay":["P.A","P.B","P.C"]}}}"#,
                "no shard named `join`",
            ),
        ];
        for (text, expected) in cases {
            let error = check(&plan(text), &graph()).expect_err(expected);
            assert!(error.contains(expected), "{expected}: {error}");
        }
        assert!(Plan::parse(
            r#"{"project":"p","shards":{"a":{"needs":[],"elaborate":[],"replay":[],"width":2}}}"#
        )
        .is_err());
        assert!(Plan::parse(r#"{"project":"p","shards":{},"extra":1}"#).is_err());
    }

    #[test]
    fn the_import_graph_reads_both_import_forms_of_the_generated_lean() {
        let dir = tempfile::tempdir().expect("tempdir");
        let modules = dir.path().join("expected/build/modules/P");
        std::fs::create_dir_all(&modules).expect("mkdir");
        std::fs::write(
            modules.join("A.lean"),
            "module\npublic import Init\nimport Lean\n",
        )
        .expect("write");
        std::fs::write(
            modules.join("B.lean"),
            "module\npublic import Init\npublic import P.A\nimport Lean\n",
        )
        .expect("write");
        std::fs::write(modules.join("C.lean"), "import P.B\nimport P.A\n").expect("write");
        let graph = import_graph(dir.path()).expect("graph");
        assert_eq!(graph["P.A"], Vec::<String>::new());
        assert_eq!(graph["P.B"], vec!["P.A".to_owned()]);
        assert_eq!(graph["P.C"], vec!["P.B".to_owned(), "P.A".to_owned()]);
    }
}
