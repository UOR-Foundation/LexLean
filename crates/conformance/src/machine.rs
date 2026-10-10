//! The declared Rust machine on the certified roots' crates (SPEC.md
//! §17.17): every crate is printed as a `RustSyntax` term, elaborated by
//! the pinned Lean, and evaluated by `RustSemantics` on the differential's
//! seeded inputs, and each outcome must be the calculus interpreter's on
//! the lowered program. This is evidence that the declared machine and the
//! renderer agree with the calculus on those inputs (`build`), not a proof.

use std::collections::BTreeMap;

use lexlean::calculus::rust::{ast, fallible_functions, lower, Profile};
use lexlean::calculus::Outcome;
use lexlean::production::certificate::render_value;
use lexlean::production::rust_term::crate_term;

use crate::differential::{observed, Case};
use crate::preservation::Certified;

/// One crate of one certified root.
#[derive(Debug, Clone)]
pub struct Rendered {
    /// The root's qualified Lean name.
    pub root: String,
    /// The target the crate realizes.
    pub target: String,
    /// The crate.
    pub krate: ast::Crate,
    /// Whether the entry function returns `R<T>`.
    /// Whether each function's rendering returns `R<T>`.
    pub fallible: Vec<bool>,
}

/// The crate of every certified root in every target it is eligible for.
///
/// # Panics
///
/// Panics when a root's program has no rendering: every eligible root
/// renders in each of its targets.
#[must_use]
pub fn crates(certified: &[Certified]) -> Vec<Rendered> {
    let mut out = Vec::new();
    for entry in certified {
        let fallible = fallible_functions(&entry.program)
            .unwrap_or_else(|reason| panic!("{}: {reason}", entry.root));
        for target in &entry.targets {
            let profile = Profile::named(target)
                .unwrap_or_else(|| panic!("{}: `{target}` is not a profile", entry.root));
            let krate = lower(&entry.program, profile)
                .unwrap_or_else(|reason| panic!("{} on {target}: {reason}", entry.root));
            out.push(Rendered {
                root: entry.root.clone(),
                target: target.clone(),
                krate,
                fallible: fallible.clone(),
            });
        }
    }
    out
}

/// Fuel enough for every seeded case.
const FUEL: u64 = 1_000_000;

const PRINTER: &str = r#"open LexLeanTarget.TargetSyntax LexLeanTarget.RustSemantics in
partial def __show : Value → String
  | .unit => "unit"
  | .bool b => s!"bool {b}"
  | .nat n => s!"nat {n}"
  | .int i => s!"int {i}"
  | .u8 v => s!"u8 {v}"
  | .u16 v => s!"u16 {v}"
  | .u32 v => s!"u32 {v}"
  | .u64 v => s!"u64 {v}"
  | .i8 v => s!"i8 {v}"
  | .i16 v => s!"i16 {v}"
  | .i32 v => s!"i32 {v}"
  | .i64 v => s!"i64 {v}"
  | .string s => "string " ++ s.quote
  | .bytes b => "bytes [" ++ ", ".intercalate (b.toList.map toString) ++ "]"
  | .ordering .less => "ordering less"
  | .ordering .same => "ordering same"
  | .ordering .more => "ordering more"
  | .none => "none"
  | .some v => "some (" ++ __show v ++ ")"
  | .ok v => "ok (" ++ __show v ++ ")"
  | .error v => "error (" ++ __show v ++ ")"
  | .list vs => "list [" ++ ", ".intercalate (vs.map __show) ++ "]"
  | .pair a b => "pair (" ++ __show a ++ ") (" ++ __show b ++ ")"
  | .adt k fs => s!"adt {k} [" ++ ", ".intercalate (fs.map __show) ++ "]"
  | .closure k cs => s!"closure {k} [" ++ ", ".intercalate (cs.map __show) ++ "]"

open LexLeanTarget.TargetSyntax LexLeanTarget.RustSemantics in
def __robs (fallible : Bool) : ROutcome → String
  | .value (.ok v) => if fallible then "value " ++ __show v else "value " ++ __show (.ok v)
  | .value (.error e) => if fallible then "overflow" else "value " ++ __show (.error e)
  | .value v => (if fallible then "ill-typed " else "value ") ++ __show v
  | .raise => "raise"
  | .abort => "abort"
  | .stuck => "stuck"
  | .exhausted => "exhausted"
"#;

/// The module that elaborates every crate and evaluates each case.
///
/// # Panics
///
/// Panics when a case's value has no Lean term.
#[must_use]
pub fn module(rendered: &[Rendered], cases: &BTreeMap<String, Vec<Case>>) -> String {
    let mut text =
        String::from("import LexLeanTarget.RustSemantics\nset_option maxRecDepth 100000\n");
    text.push_str(PRINTER);
    let mut index = 0;
    for (position, crate_) in rendered.iter().enumerate() {
        text.push_str(&format!(
            "\n-- {} on {}\ndef __crate{position} : LexLeanTarget.RustSyntax.Crate := {}\n",
            crate_.root,
            crate_.target,
            crate_term(&crate_.krate)
        ));
        for case in cases.get(&crate_.root).map_or(&[][..], Vec::as_slice) {
            let arguments: Vec<String> = case
                .values
                .iter()
                .map(|value| render_value(value).expect("a value term"))
                .collect();
            text.push_str(&format!(
                "#eval IO.println (\"case {index}: \" ++ __robs {} (LexLeanTarget.RustSemantics.invoke {FUEL} __crate{position} (.generated .function {}) [{}]))\n",
                crate_.fallible[case.function as usize],
                case.function,
                arguments.join(", ")
            ));
            index += 1;
        }
    }
    text
}

/// Compare the module's output with the interpreter: the number of cases
/// that agree, or every disagreement.
///
/// # Errors
///
/// Returns every case whose machine outcome differs from the
/// interpreter's.
pub fn compare(
    output: &str,
    rendered: &[Rendered],
    cases: &BTreeMap<String, Vec<Case>>,
) -> Result<usize, String> {
    let printed: BTreeMap<usize, &str> = output
        .lines()
        .filter_map(|line| {
            let rest = line.strip_prefix("case ")?;
            let (index, text) = rest.split_once(": ")?;
            Some((index.parse().ok()?, text))
        })
        .collect();
    let mut index = 0;
    let mut failures = Vec::new();
    for crate_ in rendered {
        for case in cases.get(&crate_.root).map_or(&[][..], Vec::as_slice) {
            let expected = match &case.outcome {
                Outcome::Exhausted => "exhausted".to_owned(),
                other => observed(other),
            };
            match printed.get(&index) {
                Some(actual) if *actual == expected => {}
                actual => failures.push(format!(
                    "{} on {} case {index}: interpreter {expected}, machine {actual:?}",
                    crate_.root, crate_.target
                )),
            }
            index += 1;
        }
    }
    if failures.is_empty() {
        Ok(index)
    } else {
        Err(failures.join("\n"))
    }
}
