//! The parts of the `bdd` recipe in one CI run (SPEC.md §9.2, §9.4): the
//! committed plan and the harness filters each part runs with.
//!
//! The plan is data, committed as `.github/bdd-parts.json`. It names parts of
//! the `repo-conformance` suite by conformance ID, so that the cases that
//! share a fixture (a verified project, an export, a compiled environment, all
//! built once per test process) run in one part and the fixture is not built
//! once per part. One part, the remainder, names no ID: it runs every test of
//! the suite no other part names, its library tests, doctests, and meta-gate
//! target included, so an ID added to the register runs there until the plan
//! moves it.
//!
//! `cargo xtask bdd` runs `cargo test` with [`CARGO_ARGUMENTS`] and, when
//! [`PART_VARIABLE`] names a part, that part's [`Plan::harness_filters`]:
//! `--exact` and the part's test names, or for the remainder `--exact` and a
//! `--skip` of every named test. The harness selects the tests; no case
//! decides whether it runs. That the parts are disjoint and together are the
//! whole suite is not argued from the filters but proved from the tests the
//! harness lists for each (`cargo xtask check-test-inventory`, RP-13).

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

/// The committed plan, repository-relative.
pub const PLAN_FILE: &str = ".github/bdd-parts.json";

/// The variable that names the part a job runs (§9.4). Unset or empty, the
/// recipe runs the whole suite.
pub const PART_VARIABLE: &str = "LEXLEAN_BDD_PART";

/// The arguments of the `cargo test` that is the whole `bdd` phase (§9.2): the
/// one command the recipe was before its parts, and still is when no part is
/// named.
pub const CARGO_ARGUMENTS: [&str; 3] = ["-p", "repo-conformance", "--all-features"];

/// The test of a conformance ID (§27.8): `RP-13` is `conformance_rp_13`.
#[must_use]
pub fn test_name(id: &str) -> String {
    format!("conformance_{}", id.to_ascii_lowercase().replace('-', "_"))
}

/// The plan: the named parts and the remainder.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Plan {
    /// The name of the part that runs every test no other part names.
    pub remainder: String,
    /// The named parts: each the conformance IDs whose tests it runs.
    pub parts: BTreeMap<String, Vec<String>>,
}

fn is_name(name: &str) -> bool {
    !name.is_empty()
        && name
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
}

fn is_id(id: &str) -> bool {
    let bytes = id.as_bytes();
    bytes.len() == 5
        && bytes[..2].iter().all(u8::is_ascii_uppercase)
        && bytes[2] == b'-'
        && bytes[3..].iter().all(u8::is_ascii_digit)
}

impl Plan {
    /// Parse the plan file's text and check its shape.
    ///
    /// # Errors
    ///
    /// Text that is not the plan's shape, or a plan [`Self::check`] refuses.
    pub fn parse(text: &str) -> Result<Self, String> {
        let value: serde_json::Value =
            serde_json::from_str(text).map_err(|error| format!("{PLAN_FILE}: {error}"))?;
        let object = value
            .as_object()
            .ok_or_else(|| format!("{PLAN_FILE}: an object is required"))?;
        if let Some(unknown) = object
            .keys()
            .find(|key| !["remainder", "parts"].contains(&key.as_str()))
        {
            return Err(format!(
                "{PLAN_FILE}: `{unknown}` is not a field of the plan"
            ));
        }
        let remainder = object
            .get("remainder")
            .and_then(serde_json::Value::as_str)
            .ok_or_else(|| format!("{PLAN_FILE}: `remainder` is a required string"))?
            .to_owned();
        let mut parts = BTreeMap::new();
        for (name, ids) in object
            .get("parts")
            .and_then(serde_json::Value::as_object)
            .ok_or_else(|| format!("{PLAN_FILE}: `parts` is a required object"))?
        {
            let ids: Vec<String> = ids
                .as_array()
                .ok_or_else(|| format!("{PLAN_FILE}: part `{name}` is an array of IDs"))?
                .iter()
                .map(|id| {
                    id.as_str()
                        .map(str::to_owned)
                        .ok_or_else(|| format!("{PLAN_FILE}: part `{name}` holds strings"))
                })
                .collect::<Result<_, _>>()?;
            parts.insert(name.clone(), ids);
        }
        let plan = Self { remainder, parts };
        plan.check()?;
        Ok(plan)
    }

    /// Read the committed plan of a repository.
    ///
    /// # Errors
    ///
    /// A plan file that cannot be read, parsed, or checked.
    pub fn read(root: &Path) -> Result<Self, String> {
        let path = root.join(PLAN_FILE);
        let text = std::fs::read_to_string(&path)
            .map_err(|error| format!("{}: {error}", path.display()))?;
        Self::parse(&text)
    }

    /// The plan's own rules: every part has a name of lowercase letters,
    /// digits, and hyphens, the remainder is not also a named part, every
    /// named part names at least one ID, every ID is of the register's form,
    /// and no ID is named twice. Whether every ID has a test, and whether the
    /// parts are the whole suite, is the inventory's proof (RP-13).
    ///
    /// # Errors
    ///
    /// The first rule broken, naming the part and the ID.
    pub fn check(&self) -> Result<(), String> {
        if !is_name(&self.remainder) {
            return Err(format!(
                "{PLAN_FILE}: the remainder `{}` is not a part name (lowercase letters, digits, hyphens)",
                self.remainder
            ));
        }
        if self.parts.contains_key(&self.remainder) {
            return Err(format!(
                "{PLAN_FILE}: `{}` is both the remainder and a named part",
                self.remainder
            ));
        }
        let mut owner: BTreeMap<&str, &str> = BTreeMap::new();
        for (name, ids) in &self.parts {
            if !is_name(name) {
                return Err(format!(
                    "{PLAN_FILE}: `{name}` is not a part name (lowercase letters, digits, hyphens)"
                ));
            }
            if ids.is_empty() {
                return Err(format!("{PLAN_FILE}: part `{name}` names no ID"));
            }
            for id in ids {
                if !is_id(id) {
                    return Err(format!(
                        "{PLAN_FILE}: part `{name}` names `{id}`, which is not a conformance ID"
                    ));
                }
                if let Some(first) = owner.insert(id, name) {
                    return Err(format!(
                        "{PLAN_FILE}: `{id}` is named by the parts `{first}` and `{name}`"
                    ));
                }
            }
        }
        Ok(())
    }

    /// Every part's name, the remainder last.
    #[must_use]
    pub fn names(&self) -> Vec<String> {
        let mut names: Vec<String> = self.parts.keys().cloned().collect();
        names.push(self.remainder.clone());
        names
    }

    /// The test names the named parts run, in sorted order.
    #[must_use]
    pub fn named_tests(&self) -> BTreeSet<String> {
        self.parts
            .values()
            .flatten()
            .map(|id| test_name(id))
            .collect()
    }

    /// The harness arguments (after `--`) that select one part: `--exact`
    /// and the part's test names, or for the remainder `--exact` and a
    /// `--skip` of every test a named part runs.
    ///
    /// # Errors
    ///
    /// A part the plan does not have.
    pub fn harness_filters(&self, part: &str) -> Result<Vec<String>, String> {
        let mut filters = vec!["--exact".to_owned()];
        if part == self.remainder {
            for name in self.named_tests() {
                filters.push("--skip".to_owned());
                filters.push(name);
            }
        } else {
            let ids = self.parts.get(part).ok_or_else(|| {
                format!(
                    "{PLAN_FILE} has no part `{part}`; its parts are {}",
                    self.names().join(", ")
                )
            })?;
            let mut names: Vec<String> = ids.iter().map(|id| test_name(id)).collect();
            names.sort();
            filters.extend(names);
        }
        Ok(filters)
    }
}

#[cfg(test)]
mod tests {
    use super::{test_name, Plan};

    const PLAN: &str = r#"{
  "remainder": "rest",
  "parts": {
    "heavy": ["SP-02", "SP-03"],
    "other": ["EX-07"]
  }
}"#;

    #[test]
    fn a_part_runs_its_tests_and_the_remainder_skips_every_named_one() {
        let plan = Plan::parse(PLAN).expect("the plan parses");
        assert_eq!(test_name("RP-13"), "conformance_rp_13");
        assert_eq!(plan.names(), ["heavy", "other", "rest"]);
        assert_eq!(
            plan.harness_filters("heavy").expect("a part"),
            ["--exact", "conformance_sp_02", "conformance_sp_03"]
        );
        assert_eq!(
            plan.harness_filters("rest").expect("the remainder"),
            [
                "--exact",
                "--skip",
                "conformance_ex_07",
                "--skip",
                "conformance_sp_02",
                "--skip",
                "conformance_sp_03"
            ]
        );
        let unknown = plan.harness_filters("none").expect_err("no such part");
        assert!(unknown.contains("heavy, other, rest"), "{unknown}");
    }

    #[test]
    fn every_defect_of_a_plan_is_named() {
        for (text, expected) in [
            (
                PLAN.replace("\"EX-07\"", "\"SP-02\""),
                "`SP-02` is named by the parts `heavy` and `other`",
            ),
            (PLAN.replace("\"EX-07\"", "\"EX7\""), "`EX7`, which is not"),
            (PLAN.replace("\"EX-07\"", ""), "part `other` names no ID"),
            (
                PLAN.replace("\"remainder\": \"rest\"", "\"remainder\": \"other\""),
                "both the remainder and a named part",
            ),
            (PLAN.replace("\"other\"", "\"Other\""), "is not a part name"),
            (
                PLAN.replace("\"parts\"", "\"shards\""),
                "is not a field of the plan",
            ),
        ] {
            let error = Plan::parse(&text).expect_err(expected);
            assert!(error.contains(expected), "{expected}: {error}");
        }
    }
}
