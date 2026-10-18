//! The declared Rust machine on the certified roots' crates (SPEC.md
//! §17.17): every crate is printed as a `RustSyntax` term, elaborated by
//! the pinned Lean, and evaluated by `RustSemantics` on the differential's
//! seeded inputs, and each outcome must be the calculus interpreter's on
//! the lowered program. This is evidence that the declared machine and the
//! renderer agree with the calculus on those inputs (`build`), not a proof.

use std::collections::BTreeMap;

use lexlean::calculus::rust::{ast, fallible_functions, lower, Profile};
use lexlean::calculus::Outcome;
use lexlean::calculus::{OrderingValue, Value};
use lexlean::production::certificate::data_b::crates_module;

use crate::lean_list::{array, list};

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

/// The module that holds every crate as the kernel term certificate B states
/// its theorem about.
pub const CRATES_MODULE: &str = "LexLeanPreserve.MachineCrates";

/// The crates' module, rendered as Lean for the pinned toolchain.
///
/// # Panics
///
/// Panics when a crate has no kernel term.
#[must_use]
pub fn crate_module(rendered: &[Rendered]) -> String {
    let crates: Vec<&ast::Crate> = rendered.iter().map(|crate_| &crate_.krate).collect();
    crates_module(CRATES_MODULE, &crates)
        .expect("the crates have kernel terms")
        .1
}

/// The module that evaluates each case over the crates of [`crate_module`].
///
/// # Panics
///
/// Panics when a case's value has no Lean term.
#[must_use]
pub fn module(rendered: &[Rendered], cases: &BTreeMap<String, Vec<Case>>) -> String {
    let mut text = format!(
        "import LexLeanTarget.RustSemantics\nimport {CRATES_MODULE}\nset_option maxRecDepth 100000\n"
    );
    text.push_str(PRINTER);
    let mut index = 0;
    for (position, crate_) in rendered.iter().enumerate() {
        text.push_str(&format!("\n-- {} on {}\n", crate_.root, crate_.target));
        for case in cases.get(&crate_.root).map_or(&[][..], Vec::as_slice) {
            let arguments: Vec<String> = case
                .values
                .iter()
                .map(|value| render_value(value).expect("a value term"))
                .collect();
            text.push_str(&format!(
                "#eval IO.println (\"case {index}: \" ++ __robs {} (LexLeanTarget.RustSemantics.invoke {FUEL} {CRATES_MODULE}.__crate{position} (.generated .function {}) [{}]))\n",
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

// --- The values the cases pass, as `TargetSyntax.Value` terms -----------------
// A harness input is text the pinned Lean evaluates; it is not a certificate, and
// the compiler's production code writes no Lean source of it.

fn signed(text: &str) -> String {
    if text.starts_with('-') {
        format!("({text})")
    } else {
        text.to_owned()
    }
}

fn bytes_literal(hex: &str) -> Result<String, String> {
    let mut values = Vec::new();
    let digits = hex.as_bytes();
    if !digits.len().is_multiple_of(2) {
        return Err(format!("byte literal `{hex}` has odd length"));
    }
    for pair in digits.chunks(2) {
        let text = std::str::from_utf8(pair).map_err(|_| "a non-ASCII byte literal")?;
        values.push(
            u8::from_str_radix(text, 16)
                .map_err(|_| format!("byte literal `{hex}` is not hexadecimal"))?
                .to_string(),
        );
    }
    Ok(format!("ByteArray.mk {}", array(values)))
}

/// A calculus value as a `TargetSyntax.Value` term.
///
/// # Errors
///
/// Returns the reason a byte literal is malformed.
pub fn render_value(value: &Value) -> Result<String, String> {
    Ok(match value {
        Value::Unit => ".unit".to_owned(),
        Value::Bool { value } => format!("(.bool {value})"),
        Value::Nat { value } => format!("(.nat {value})"),
        Value::Int { value } => format!("(.int {})", signed(value)),
        Value::U8 { value } => format!("(.u8 {value})"),
        Value::U16 { value } => format!("(.u16 {value})"),
        Value::U32 { value } => format!("(.u32 {value})"),
        Value::U64 { value } => format!("(.u64 {value})"),
        Value::I8 { value } => format!("(.i8 {})", signed(value)),
        Value::I16 { value } => format!("(.i16 {})", signed(value)),
        Value::I32 { value } => format!("(.i32 {})", signed(value)),
        Value::I64 { value } => format!("(.i64 {})", signed(value)),
        Value::String { value } => format!("(.string {})", lean_string(value)),
        Value::Bytes { hex } => format!("(.bytes ({}))", bytes_literal(hex)?),
        Value::Ordering { value } => format!(
            "(.ordering .{})",
            match value {
                OrderingValue::Lt => "less",
                OrderingValue::Eq => "same",
                OrderingValue::Gt => "more",
            }
        ),
        Value::None => ".none".to_owned(),
        Value::Some { value } => format!("(.some {})", render_value(value)?),
        Value::Ok { value } => format!("(.ok {})", render_value(value)?),
        Value::Error { value } => format!("(.error {})", render_value(value)?),
        Value::List { items } => format!(
            "(.list {})",
            list(
                items
                    .iter()
                    .map(render_value)
                    .collect::<Result<Vec<_>, _>>()?
            )
        ),
        Value::Pair { left, right } => {
            format!("(.pair {} {})", render_value(left)?, render_value(right)?)
        }
        Value::Adt {
            constructor,
            fields,
        } => format!(
            "(.adt {constructor} {})",
            list(
                fields
                    .iter()
                    .map(render_value)
                    .collect::<Result<Vec<_>, _>>()?
            )
        ),
        Value::Closure { function, captures } => format!(
            "(.closure {function} {})",
            list(
                captures
                    .iter()
                    .map(render_value)
                    .collect::<Result<Vec<_>, _>>()?
            )
        ),
    })
}

/// A string as a Lean literal: control characters and quotes escaped.
fn lean_string(value: &str) -> String {
    use std::fmt::Write as _;
    let mut output = String::from("\"");
    for character in value.chars() {
        match character {
            '\\' => output.push_str("\\\\"),
            '"' => output.push_str("\\\""),
            '\n' => output.push_str("\\n"),
            '\r' => output.push_str("\\r"),
            '\t' => output.push_str("\\t"),
            character if character.is_control() => {
                write!(output, "\\u{:04x}", u32::from(character)).expect("writing to a string");
            }
            character => output.push(character),
        }
    }
    output.push('"');
    output
}
