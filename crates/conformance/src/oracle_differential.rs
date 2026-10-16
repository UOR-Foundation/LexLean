//! The differential against the oracle (SPEC.md §17.19).
//!
//! For each root of the corpus ([`crate::oracle_corpus`]) two programs are
//! compared that were written independently: the Rust the oracle generates
//! from the Lean LexLean generated, and what LexLean itself states of the
//! root — its production verdict, its lowered program, and the interpreter's
//! outcome on the seeded inputs of [`crate::differential`], which the Rust
//! backend's differentials (RB-06, SP-11) tie to LexLean's own Rust.
//!
//! The comparison is semantic and never textual. What it compares, in order:
//!
//! 1. *acceptance*: whether each side takes the root, and for a root only the
//!    oracle takes or refuses, the kind of refusal;
//! 2. *contract*: the root's signature, as a structural shape that does not
//!    depend on either side's names or on how a side spells a type, its
//!    failure typing, and how it passes its collections;
//! 3. *behavior*: the observable outcome on every seeded input, in the JSON
//!    form the Rust backend's harnesses print, an overflow being an overflow
//!    whatever the oracle calls it;
//! 4. *resources*: the allocations the oracle's code performs against the
//!    allocation class the production eligibility states for the root, and
//!    that no call panics.
//!
//! Every way the two sides differ is a [`Diff`]. A difference is accepted
//! only when the versioned table of known deliberate differences
//! (`oracle/known-differences.toml`) names it, and a row of the table that no
//! difference of the run matches fails the run: the table is re-asserted,
//! not trusted. A difference that is not in the table is a disagreement, and
//! a disagreement carries both sides' evidence.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use lexlean::calculus::rust::package::{package, Errors, Export, Manifest, Passing, MANIFEST_SPEC};
use lexlean::calculus::rust::Profile;
use lexlean::calculus::{Program, Ty, Value};
use serde::{Deserialize, Serialize};
use serde_json::Value as Json;
use sha2::{Digest, Sha256};

use crate::differential::Case;
use crate::oracle::{self, Oracle, Stage, Unavailable, Verdict};
use crate::oracle_corpus::{self as corpus, Kind};
use crate::oracle_seeds as seeds;
use crate::support::{self, repo_root, P};

// ---- the oracle's intermediate representation ------------------------------

/// A parsed S-expression of the oracle's IR.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Sx {
    Atom(String),
    Text(String),
    List(Vec<Sx>),
}

fn parse_sx(text: &str) -> Result<Vec<Sx>, String> {
    let chars: Vec<char> = text.chars().collect();
    let mut at = 0;
    let mut stack: Vec<Vec<Sx>> = vec![Vec::new()];
    while at < chars.len() {
        let c = chars[at];
        match c {
            c if c.is_whitespace() => at += 1,
            ';' => {
                while at < chars.len() && chars[at] != '\n' {
                    at += 1;
                }
            }
            '(' => {
                stack.push(Vec::new());
                at += 1;
            }
            ')' => {
                let done = stack.pop().ok_or("unbalanced `)`")?;
                stack
                    .last_mut()
                    .ok_or("unbalanced `)`")?
                    .push(Sx::List(done));
                at += 1;
            }
            '"' => {
                at += 1;
                let mut text = String::new();
                while at < chars.len() && chars[at] != '"' {
                    if chars[at] == '\\' && at + 1 < chars.len() {
                        at += 1;
                    }
                    text.push(chars[at]);
                    at += 1;
                }
                if at >= chars.len() {
                    return Err("unterminated string".into());
                }
                at += 1;
                stack.last_mut().ok_or("no list")?.push(Sx::Text(text));
            }
            _ => {
                let start = at;
                while at < chars.len()
                    && !chars[at].is_whitespace()
                    && !matches!(chars[at], '(' | ')')
                {
                    at += 1;
                }
                stack
                    .last_mut()
                    .ok_or("no list")?
                    .push(Sx::Atom(chars[start..at].iter().collect()));
            }
        }
    }
    if stack.len() != 1 {
        return Err("unbalanced `(`".into());
    }
    stack.pop().ok_or_else(|| "empty".to_owned())
}

/// A type of the oracle's IR.
#[derive(Debug, Clone, PartialEq, Eq)]
enum IrTy {
    Unit,
    Bool,
    Nat,
    Int,
    Fixed(String),
    Text,
    Bytes,
    Ordering,
    Option(Box<IrTy>),
    Result(Box<IrTy>, Box<IrTy>),
    List(Box<IrTy>),
    Tuple(Box<IrTy>, Box<IrTy>),
    Named(String),
    /// Lean's `Unit`, which the oracle renders as the empty structure
    /// `PUnit`.
    PUnit,
}

fn ir_type(sx: &Sx) -> Result<IrTy, String> {
    match sx {
        Sx::Atom(name) => Ok(match name.as_str() {
            "Unit" => IrTy::Unit,
            "Bool" => IrTy::Bool,
            "Nat" => IrTy::Nat,
            "Int" => IrTy::Int,
            "String" => IrTy::Text,
            "ByteArray" | "Bytes" => IrTy::Bytes,
            "Ordering" => IrTy::Ordering,
            "Int8" => IrTy::Fixed("i8".into()),
            "Int16" => IrTy::Fixed("i16".into()),
            "Int32" => IrTy::Fixed("i32".into()),
            "Int64" => IrTy::Fixed("i64".into()),
            "UInt8" => IrTy::Fixed("u8".into()),
            "UInt16" => IrTy::Fixed("u16".into()),
            "UInt32" => IrTy::Fixed("u32".into()),
            "UInt64" => IrTy::Fixed("u64".into()),
            other => {
                return Err(format!(
                    "the IR type `{other}` is not one the harness knows"
                ))
            }
        }),
        Sx::List(items) => match items.as_slice() {
            [Sx::Atom(head), argument] if head == "Option" => {
                Ok(IrTy::Option(Box::new(ir_type(argument)?)))
            }
            [Sx::Atom(head), argument] if head == "List" => {
                Ok(IrTy::List(Box::new(ir_type(argument)?)))
            }
            [Sx::Atom(head), left, right] if head == "Tuple" => Ok(IrTy::Tuple(
                Box::new(ir_type(left)?),
                Box::new(ir_type(right)?),
            )),
            [Sx::Atom(head), ok, error] if head == "Result" => Ok(IrTy::Result(
                Box::new(ir_type(ok)?),
                Box::new(ir_type(error)?),
            )),
            [Sx::Atom(head), Sx::Text(name)] if head == "named" && name == "PUnit" => {
                Ok(IrTy::PUnit)
            }
            [Sx::Atom(head), Sx::Text(name)] if head == "named" => Ok(IrTy::Named(name.clone())),
            other => Err(format!(
                "the IR type {other:?} is not one the harness knows"
            )),
        },
        Sx::Text(_) => Err("a string is not a type".into()),
    }
}

/// One constructor of a type the oracle declared: its Lean name and its
/// fields, named.
#[derive(Debug, Clone)]
struct IrCtor {
    name: String,
    fields: Vec<(String, IrTy)>,
}

/// The oracle's exported module: the types it declared and the definitions.
#[derive(Debug, Clone, Default)]
struct Ir {
    types: BTreeMap<String, Vec<IrCtor>>,
    defs: BTreeMap<String, (Vec<(String, IrTy)>, IrTy)>,
}

fn read_ir(text: &str) -> Result<Ir, String> {
    let top = parse_sx(text)?;
    let [Sx::List(module)] = top.as_slice() else {
        return Err("the IR is not one module".into());
    };
    let mut ir = Ir::default();
    for item in module.iter().skip(2) {
        let Sx::List(parts) = item else { continue };
        match parts.as_slice() {
            [Sx::Atom(head), Sx::Text(name), constructors @ ..] if head == "type" => {
                let mut declared = Vec::new();
                for constructor in constructors {
                    let Sx::List(parts) = constructor else {
                        return Err("a constructor is not a list".into());
                    };
                    let [Sx::Atom(tag), Sx::Text(ctor), fields @ ..] = parts.as_slice() else {
                        return Err("a constructor has no name".into());
                    };
                    if tag != "ctor" {
                        return Err(format!("`{tag}` is not a constructor"));
                    }
                    let mut declared_fields = Vec::new();
                    for field in fields {
                        let Sx::List(pair) = field else {
                            return Err("a field is not a list".into());
                        };
                        let [Sx::Atom(field_name), field_type] = pair.as_slice() else {
                            return Err("a field is not (name Type)".into());
                        };
                        declared_fields.push((field_name.clone(), ir_type(field_type)?));
                    }
                    declared.push(IrCtor {
                        name: ctor.clone(),
                        fields: declared_fields,
                    });
                }
                ir.types.insert(name.clone(), declared);
            }
            [Sx::Atom(head), Sx::Atom(name), Sx::List(parameters), result, _body]
                if head == "def" =>
            {
                let mut declared = Vec::new();
                for parameter in parameters {
                    let Sx::List(pair) = parameter else {
                        return Err("a parameter is not a list".into());
                    };
                    let [Sx::Atom(parameter_name), parameter_type] = pair.as_slice() else {
                        return Err("a parameter is not (name Type)".into());
                    };
                    declared.push((parameter_name.clone(), ir_type(parameter_type)?));
                }
                ir.defs.insert(name.clone(), (declared, ir_type(result)?));
            }
            _ => {}
        }
    }
    Ok(ir)
}

fn last_segment(name: &str) -> &str {
    name.rsplit('.').next().unwrap_or(name)
}

// ---- contracts ---------------------------------------------------------------

/// A type as a contract states it: structure only, none of either side's
/// names, so that two spellings of one type are one shape.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum Shape {
    /// The unit type.
    Unit,
    /// Booleans.
    Bool,
    /// Natural numbers in 64 bits.
    Nat,
    /// Mathematical integers.
    Int,
    /// A fixed width, by its Rust name.
    Fixed(String),
    /// Text.
    Text,
    /// Bytes.
    Bytes,
    /// An ordering.
    Ordering,
    /// An option.
    Option(Box<Shape>),
    /// A result.
    Result(Box<Shape>, Box<Shape>),
    /// A list.
    List(Box<Shape>),
    /// A pair.
    Pair(Box<Shape>, Box<Shape>),
    /// A closed data type: its constructors' fields in order.
    Adt(Vec<Vec<Shape>>),
    /// A data type met again inside itself.
    Recursive,
}

fn lexlean_shape(program: &Program, ty: &Ty, open: &mut Vec<u64>) -> Shape {
    match ty {
        Ty::Unit => Shape::Unit,
        Ty::Bool => Shape::Bool,
        Ty::Nat => Shape::Nat,
        Ty::Int => Shape::Int,
        Ty::Fixed { width } => Shape::Fixed(width.name().to_owned()),
        Ty::String => Shape::Text,
        Ty::Bytes => Shape::Bytes,
        Ty::Ordering => Shape::Ordering,
        Ty::Option { value } => Shape::Option(Box::new(lexlean_shape(program, value, open))),
        Ty::Result { ok, error } => Shape::Result(
            Box::new(lexlean_shape(program, ok, open)),
            Box::new(lexlean_shape(program, error, open)),
        ),
        Ty::List { element } => Shape::List(Box::new(lexlean_shape(program, element, open))),
        Ty::Pair { left, right } => Shape::Pair(
            Box::new(lexlean_shape(program, left, open)),
            Box::new(lexlean_shape(program, right, open)),
        ),
        Ty::Adt { index } => {
            if open.contains(index) {
                return Shape::Recursive;
            }
            open.push(*index);
            let shape = Shape::Adt(
                usize::try_from(*index)
                    .ok()
                    .and_then(|at| program.adts.get(at))
                    .map(|adt| {
                        adt.constructors
                            .iter()
                            .map(|fields| {
                                fields
                                    .iter()
                                    .map(|field| lexlean_shape(program, field, open))
                                    .collect()
                            })
                            .collect()
                    })
                    .unwrap_or_default(),
            );
            open.pop();
            shape
        }
        Ty::Fn { .. } => Shape::Recursive,
    }
}

fn ir_shape(ir: &Ir, ty: &IrTy, open: &mut Vec<String>) -> Shape {
    match ty {
        IrTy::Unit | IrTy::PUnit => Shape::Unit,
        IrTy::Bool => Shape::Bool,
        IrTy::Nat => Shape::Nat,
        IrTy::Int => Shape::Int,
        IrTy::Fixed(name) => Shape::Fixed(name.clone()),
        IrTy::Text => Shape::Text,
        IrTy::Bytes => Shape::Bytes,
        IrTy::Ordering => Shape::Ordering,
        IrTy::Option(value) => Shape::Option(Box::new(ir_shape(ir, value, open))),
        IrTy::Result(ok, error) => Shape::Result(
            Box::new(ir_shape(ir, ok, open)),
            Box::new(ir_shape(ir, error, open)),
        ),
        IrTy::List(element) => Shape::List(Box::new(ir_shape(ir, element, open))),
        IrTy::Tuple(left, right) => Shape::Pair(
            Box::new(ir_shape(ir, left, open)),
            Box::new(ir_shape(ir, right, open)),
        ),
        IrTy::Named(name) => {
            if open.contains(name) {
                return Shape::Recursive;
            }
            open.push(name.clone());
            let shape = Shape::Adt(
                ir.types
                    .get(name)
                    .map(|constructors| {
                        constructors
                            .iter()
                            .map(|constructor| {
                                constructor
                                    .fields
                                    .iter()
                                    .map(|(_, field)| ir_shape(ir, field, open))
                                    .collect()
                            })
                            .collect()
                    })
                    .unwrap_or_default(),
            );
            open.pop();
            shape
        }
    }
}

/// A root's contract on one side.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Contract {
    /// The parameters' shapes in order.
    pub parameters: Vec<Shape>,
    /// The result's shape.
    pub result: Shape,
    /// Whether the call can fail with an overflow.
    pub failure: bool,
    /// The parameters the side takes by reference rather than by value, by
    /// position.
    pub borrowed: Vec<usize>,
    /// Whether the side returns its list result into a buffer the caller
    /// passes, answering how much it wrote (the oracle's memory profile).
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub output_buffer: bool,
    /// Where the Rust text of the oracle's signature disagrees with its own
    /// intermediate representation: empty for LexLean, whose signature is
    /// derived from its program.
    pub drift: Vec<String>,
}

// ---- the two sides -----------------------------------------------------------

/// What LexLean states of an accepted root.
#[derive(Debug, Clone)]
pub struct Accepted {
    /// The targets the root is eligible for.
    pub targets: Vec<String>,
    /// The effects its closure realizes, over its targets.
    pub effects: BTreeSet<String>,
    /// The registry constructs its closure realizes.
    pub constructs: BTreeSet<String>,
    /// The lowered program.
    pub program: Program,
    /// The contract of function 0.
    pub contract: Contract,
    /// The seeded inputs and the interpreter's outcomes.
    pub cases: Vec<Case>,
}

/// LexLean's side of one root.
#[derive(Debug, Clone)]
pub enum LexLeanSide {
    /// LexLean takes the root as a production root.
    Eligible(Box<Accepted>),
    /// LexLean refuses the root, with the diagnostic its fixture records.
    Rejected {
        /// The code.
        code: String,
        /// The message.
        message: String,
    },
}

/// The oracle's side of one root.
#[derive(Debug, Clone)]
pub enum OracleSide {
    /// The oracle generated a crate.
    Generated {
        /// The oracle's IR.
        ir: String,
        /// The crate's files.
        package: BTreeMap<String, Vec<u8>>,
        /// The contract of the root's function.
        contract: Contract,
        /// The Rust text of each parameter's type and of the result.
        signature: (Vec<String>, String),
    },
    /// The oracle refused the root.
    Refused {
        /// The stage that refused.
        stage: Stage,
        /// The kind of refusal.
        kind: String,
        /// The oracle's words.
        message: String,
    },
}

/// One root of the corpus with both sides' statements.
#[derive(Debug, Clone)]
pub struct RootRun {
    /// The corpus it belongs to.
    pub corpus: String,
    /// The root's qualified Lean name.
    pub root: String,
    /// The Lean module it is in.
    pub lean_module: String,
    /// LexLean's side.
    pub lexlean: LexLeanSide,
    /// The oracle's side.
    pub oracle: OracleSide,
}

impl RootRun {
    /// The root's name as the oracle's crate states it.
    #[must_use]
    pub fn function(&self) -> &str {
        last_segment(&self.root)
    }
}

// ---- differences -----------------------------------------------------------

/// What kind of thing differs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiffKind {
    /// Whether a side takes the root.
    Acceptance,
    /// The root's signature, failure typing, or parameter passing.
    Contract,
    /// An observable outcome.
    Behavior,
    /// An intended resource invariant.
    Resource,
}

/// One difference between the sides on one root, with both sides' evidence.
#[derive(Debug, Clone, Serialize)]
pub struct Diff {
    /// The corpus.
    pub corpus: String,
    /// The root.
    pub root: String,
    /// The kind.
    pub kind: DiffKind,
    /// The difference's name, which a row of the table of known differences
    /// states.
    pub key: String,
    /// LexLean's evidence.
    pub lexlean: Json,
    /// The oracle's evidence.
    pub oracle: Json,
}

/// One input and what each side did with it.
#[derive(Debug, Clone, Serialize)]
pub struct Observed {
    /// The Lean terms of the inputs.
    pub arguments: Vec<String>,
    /// The interpreter's observable outcome.
    pub lexlean: Json,
    /// The candidate's observable outcome.
    pub oracle: Json,
    /// The allocations the candidate performed.
    pub allocations: u64,
}

fn sha256_hex(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

// ---- the Rust the oracle generated -----------------------------------------

/// Split `text` at the commas of its top level.
fn split_top(text: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut depth = 0_i32;
    let mut current = String::new();
    for c in text.chars() {
        match c {
            '<' | '(' | '[' => depth += 1,
            '>' | ')' | ']' => depth -= 1,
            ',' if depth == 0 => {
                parts.push(current.trim().to_owned());
                current.clear();
                continue;
            }
            _ => {}
        }
        current.push(c);
    }
    if !current.trim().is_empty() {
        parts.push(current.trim().to_owned());
    }
    parts
}

/// The parameter types and the result type of `pub fn <name>` in `source`,
/// as the oracle wrote them.
fn rust_signature(source: &str, name: &str) -> Result<(Vec<String>, String), String> {
    let marker = format!("pub fn {name}(");
    let start = source
        .find(&marker)
        .ok_or_else(|| format!("the crate has no `pub fn {name}`"))?
        + marker.len();
    let mut depth = 1_i32;
    let mut end = start;
    for (offset, c) in source[start..].char_indices() {
        match c {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth == 0 {
                    end = start + offset;
                    break;
                }
            }
            _ => {}
        }
    }
    let parameters = split_top(&source[start..end])
        .into_iter()
        .map(|parameter| {
            parameter
                .split_once(':')
                .map(|(_, ty)| ty.trim().to_owned())
                .ok_or_else(|| format!("parameter `{parameter}` has no type"))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let rest = &source[end + 1..];
    let result = match rest.trim_start().strip_prefix("->") {
        Some(after) => {
            let brace = after.find('{').ok_or("no body")?;
            after[..brace].trim().to_owned()
        }
        None => "()".to_owned(),
    };
    Ok((parameters, result))
}

fn oracle_contract(
    ir: &Ir,
    name: &str,
    source: &str,
) -> Result<(Contract, (Vec<String>, String)), String> {
    let (parameters, result) = ir
        .defs
        .get(name)
        .ok_or_else(|| format!("the oracle's IR has no definition `{name}`"))?;
    let mut signature = rust_signature(source, name)?;
    let flat = |text: &str| text.replace("crate::", "o::").replace(' ', "");
    // The oracle returns a list into a buffer its caller passes, one more
    // parameter than its IR states (`output: &mut [T]`), and answers the
    // number of elements written, or `OutputTooSmall`: a convention of its
    // memory profile, recorded as such, not a parameter of the root.
    let mut drift = Vec::new();
    let output_buffer = match result {
        IrTy::List(element) if signature.0.len() == parameters.len() + 1 => {
            let buffer = signature.0.pop().expect("the buffer parameter");
            let expected = format!("&mut [{}]", rust_type(element));
            if flat(&expected) != flat(&buffer) {
                drift.push(format!(
                    "output buffer: the IR states `{expected}`, the crate `{buffer}`"
                ));
            }
            if flat(&signature.1) != flat("Result<usize, crate::ComputeError>") {
                drift.push(format!(
                    "output buffer: the crate answers `{}`, not the number of elements written",
                    signature.1
                ));
            }
            true
        }
        _ => false,
    };
    if signature.0.len() != parameters.len() {
        return Err(format!(
            "the IR gives `{name}` {} parameters and the crate {}",
            parameters.len(),
            signature.0.len()
        ));
    }
    // A buffered result is fallible for the buffer alone; whether the call
    // can overflow is then what it does, which the differential runs.
    let failure = !output_buffer && signature.1.ends_with("crate::ComputeError>");
    // The text must say what the IR says: a signature the oracle's own IR
    // does not describe is a finding about the oracle, whichever of the two
    // is wrong.
    for (at, ((_, ty), text)) in parameters.iter().zip(&signature.0).enumerate() {
        let expected = match ty {
            IrTy::List(element) => format!("&[{}]", rust_type(element)),
            other => rust_type(other),
        };
        if flat(&expected) != flat(text) {
            drift.push(format!(
                "parameter {at}: the IR states `{expected}`, the crate `{text}`"
            ));
        }
    }
    let result_text = if failure {
        signature
            .1
            .strip_prefix("Result<")
            .and_then(|rest| rest.strip_suffix(", crate::ComputeError>"))
            .unwrap_or(&signature.1)
            .to_owned()
    } else {
        signature.1.clone()
    };
    if !output_buffer && flat(&rust_type(result)) != flat(&result_text) {
        drift.push(format!(
            "result: the IR states `{}`, the crate `{}`",
            rust_type(result),
            signature.1
        ));
    }
    let borrowed = signature
        .0
        .iter()
        .enumerate()
        .filter(|(_, ty)| ty.starts_with('&'))
        .map(|(at, _)| at)
        .collect();
    Ok((
        Contract {
            parameters: parameters
                .iter()
                .map(|(_, ty)| ir_shape(ir, ty, &mut Vec::new()))
                .collect(),
            result: ir_shape(ir, result, &mut Vec::new()),
            failure,
            borrowed,
            output_buffer,
            drift,
        },
        signature,
    ))
}

/// Whether the oracle's signature returns a list through a buffer its
/// caller passes ([`oracle_contract`] has removed the buffer parameter).
fn buffered(result: &IrTy, signature: &(Vec<String>, String)) -> bool {
    matches!(result, IrTy::List(_))
        && signature.1.replace(' ', "") == "Result<usize,crate::ComputeError>"
}

/// A value of `ty` to fill an output buffer with before the call writes it.
fn filler(ty: &IrTy) -> Result<String, String> {
    Ok(match ty {
        IrTy::Unit => "()".into(),
        IrTy::Bool => "false".into(),
        IrTy::Nat => "0u64".into(),
        IrTy::Int => "0i128".into(),
        IrTy::Fixed(name) => format!("0{name}"),
        IrTy::Tuple(left, right) => format!("({}, {})", filler(left)?, filler(right)?),
        IrTy::Option(_) => "None".into(),
        other => return Err(format!("no filler for a buffered `{}`", rust_type(other))),
    })
}

/// The elements an output buffer holds: more than any case of the corpus
/// writes, so `OutputTooSmall` is never the answer of a call that fits.
const OUTPUT_CAPACITY: usize = 1 << 14;

// ---- rendering calls into the oracle's crate -------------------------------

fn rust_type(ty: &IrTy) -> String {
    match ty {
        IrTy::Unit => "()".into(),
        IrTy::PUnit => "o::PUnit".into(),
        IrTy::Bool => "bool".into(),
        IrTy::Nat => "u64".into(),
        IrTy::Int => "i128".into(),
        IrTy::Fixed(name) => name.clone(),
        IrTy::Text => "alloc::string::String".into(),
        IrTy::Bytes => "alloc::vec::Vec<u8>".into(),
        IrTy::Ordering => "core::cmp::Ordering".into(),
        IrTy::Option(value) => format!("Option<{}>", rust_type(value)),
        IrTy::Result(ok, error) => {
            format!("Result<{}, {}>", rust_type(ok), rust_type(error))
        }
        IrTy::List(element) => format!("alloc::vec::Vec<{}>", rust_type(element)),
        IrTy::Tuple(left, right) => format!("({}, {})", rust_type(left), rust_type(right)),
        IrTy::Named(name) => format!("o::{}", last_segment(name)),
    }
}

fn fixed_value(value: &Value) -> Option<&str> {
    match value {
        Value::U8 { value }
        | Value::U16 { value }
        | Value::U32 { value }
        | Value::U64 { value }
        | Value::I8 { value }
        | Value::I16 { value }
        | Value::I32 { value }
        | Value::I64 { value } => Some(value),
        _ => None,
    }
}

/// A Rust expression building `value` of IR type `ty`.
fn literal(ir: &Ir, ty: &IrTy, value: &Value) -> Result<String, String> {
    Ok(match (ty, value) {
        (IrTy::Unit, Value::Unit) => "()".into(),
        (IrTy::PUnit, Value::Unit) => "o::PUnit {}".into(),
        (IrTy::Bool, Value::Bool { value }) => value.to_string(),
        (IrTy::Nat, Value::Nat { value }) => format!("{value}u64"),
        (IrTy::Fixed(name), value) => {
            let number = fixed_value(value).ok_or("a fixed-width value")?;
            format!("({number}{name})")
        }
        (IrTy::Text, Value::String { value }) => {
            format!("alloc::string::String::from({value:?})")
        }
        (IrTy::Bytes, Value::Bytes { hex }) => {
            let bytes: Vec<String> = (0..hex.len() / 2)
                .map(|at| format!("0x{}u8", &hex[2 * at..2 * at + 2]))
                .collect();
            if bytes.is_empty() {
                "alloc::vec::Vec::<u8>::new()".into()
            } else {
                format!("alloc::vec![{}]", bytes.join(", "))
            }
        }
        (IrTy::Ordering, Value::Ordering { value }) => format!(
            "core::cmp::Ordering::{}",
            match value {
                lexlean::calculus::OrderingValue::Lt => "Less",
                lexlean::calculus::OrderingValue::Eq => "Equal",
                lexlean::calculus::OrderingValue::Gt => "Greater",
            }
        ),
        (IrTy::Option(inner), Value::None) => format!("None::<{}>", rust_type(inner)),
        (IrTy::Option(inner), Value::Some { value }) => {
            format!("Some({})", literal(ir, inner, value)?)
        }
        (IrTy::Result(ok, error), Value::Ok { value }) => format!(
            "Ok::<{}, {}>({})",
            rust_type(ok),
            rust_type(error),
            literal(ir, ok, value)?
        ),
        (IrTy::Result(ok, error), Value::Error { value }) => format!(
            "Err::<{}, {}>({})",
            rust_type(ok),
            rust_type(error),
            literal(ir, error, value)?
        ),
        (IrTy::List(element), Value::List { items }) => {
            if items.is_empty() {
                format!("alloc::vec::Vec::<{}>::new()", rust_type(element))
            } else {
                let items = items
                    .iter()
                    .map(|item| literal(ir, element, item))
                    .collect::<Result<Vec<_>, _>>()?;
                format!("alloc::vec![{}]", items.join(", "))
            }
        }
        (IrTy::Tuple(left_ty, right_ty), Value::Pair { left, right }) => format!(
            "({}, {})",
            literal(ir, left_ty, left)?,
            literal(ir, right_ty, right)?
        ),
        (
            IrTy::Named(name),
            Value::Adt {
                constructor,
                fields,
            },
        ) => {
            let constructors = ir.types.get(name).ok_or("an undeclared type")?;
            let at = usize::try_from(*constructor).map_err(|e| e.to_string())?;
            let declared = constructors.get(at).ok_or("an undeclared constructor")?;
            let mut inits = Vec::new();
            for ((field_name, field_ty), field) in declared.fields.iter().zip(fields) {
                inits.push(format!("{field_name}: {}", literal(ir, field_ty, field)?));
            }
            let body = if inits.is_empty() && constructors.len() > 1 {
                String::new()
            } else {
                format!(" {{ {} }}", inits.join(", "))
            };
            if constructors.len() == 1 {
                format!("o::{}{body}", last_segment(name))
            } else {
                format!(
                    "o::{}::{}{body}",
                    last_segment(name),
                    last_segment(&declared.name)
                )
            }
        }
        (ty, value) => return Err(format!("{value:?} is not a value of {ty:?}")),
    })
}

/// A Rust expression printing the value in `expr` of IR type `ty` in the
/// exact JSON form `Value` serializes to. `expr` is a place the expression
/// may move out of.
fn show(ir: &Ir, ty: &IrTy, expr: &str, depth: usize) -> Result<String, String> {
    let number = |kind: &str| {
        format!(
            "format!(\"{{{{\\\"kind\\\":\\\"{kind}\\\",\\\"value\\\":\\\"{{}}\\\"}}}}\", {expr})"
        )
    };
    let next = depth + 1;
    Ok(match ty {
        IrTy::Unit => format!("{{ let () = {expr}; \"{{\\\"kind\\\":\\\"unit\\\"}}\".to_owned() }}"),
        IrTy::PUnit => format!("{{ let _ = {expr}; \"{{\\\"kind\\\":\\\"unit\\\"}}\".to_owned() }}"),
        IrTy::Bool => {
            format!("format!(\"{{{{\\\"kind\\\":\\\"bool\\\",\\\"value\\\":{{}}}}}}\", {expr})")
        }
        IrTy::Nat => number("nat"),
        IrTy::Int => number("int"),
        IrTy::Fixed(name) => number(name),
        IrTy::Text => format!(
            "format!(\"{{{{\\\"kind\\\":\\\"string\\\",\\\"value\\\":{{}}}}}}\", quote(&{expr}))"
        ),
        IrTy::Bytes => format!(
            "format!(\"{{{{\\\"kind\\\":\\\"bytes\\\",\\\"hex\\\":\\\"{{}}\\\"}}}}\", {expr}.iter().map(|b| format!(\"{{b:02x}}\")).collect::<String>())"
        ),
        IrTy::Ordering => format!(
            "format!(\"{{{{\\\"kind\\\":\\\"ordering\\\",\\\"value\\\":\\\"{{}}\\\"}}}}\", match {expr} {{ core::cmp::Ordering::Less => \"lt\", core::cmp::Ordering::Equal => \"eq\", core::cmp::Ordering::Greater => \"gt\" }})"
        ),
        IrTy::Option(inner) => {
            let name = format!("x{depth}");
            let inner = show(ir, inner, &name, next)?;
            format!("match {expr} {{ None => \"{{\\\"kind\\\":\\\"none\\\"}}\".to_owned(), Some({name}) => format!(\"{{{{\\\"kind\\\":\\\"some\\\",\\\"value\\\":{{}}}}}}\", {inner}) }}")
        }
        IrTy::Result(ok, error) => {
            let name = format!("x{depth}");
            let ok_show = show(ir, ok, &name, next)?;
            let error_show = show(ir, error, &name, next)?;
            format!("match {expr} {{ Ok({name}) => format!(\"{{{{\\\"kind\\\":\\\"ok\\\",\\\"value\\\":{{}}}}}}\", {ok_show}), Err({name}) => format!(\"{{{{\\\"kind\\\":\\\"error\\\",\\\"value\\\":{{}}}}}}\", {error_show}) }}")
        }
        IrTy::List(element) => {
            let name = format!("x{depth}");
            let inner = show(ir, element, &name, next)?;
            format!("{{ let mut parts = Vec::new(); for {name} in {expr} {{ parts.push({inner}); }} format!(\"{{{{\\\"kind\\\":\\\"list\\\",\\\"items\\\":[{{}}]}}}}\", parts.join(\",\")) }}")
        }
        IrTy::Tuple(left, right) => {
            let l = format!("l{depth}");
            let r = format!("r{depth}");
            let left_show = show(ir, left, &l, next)?;
            let right_show = show(ir, right, &r, next)?;
            format!("{{ let ({l}, {r}) = {expr}; format!(\"{{{{\\\"kind\\\":\\\"pair\\\",\\\"left\\\":{{}},\\\"right\\\":{{}}}}}}\", {left_show}, {right_show}) }}")
        }
        IrTy::Named(name) => {
            let constructors = ir.types.get(name).ok_or("an undeclared type")?;
            let type_name = last_segment(name);
            let mut arms = String::new();
            for (at, constructor) in constructors.iter().enumerate() {
                let mut patterns = Vec::new();
                let mut shows = Vec::new();
                for (position, (field_name, field_ty)) in constructor.fields.iter().enumerate() {
                    let held = format!("f{depth}_{position}");
                    patterns.push(format!("{field_name}: {held}"));
                    shows.push(show(ir, field_ty, &held, next)?);
                }
                let path = if constructors.len() == 1 {
                    format!("o::{type_name}")
                } else {
                    format!("o::{type_name}::{}", last_segment(&constructor.name))
                };
                let pattern = if patterns.is_empty() && constructors.len() > 1 {
                    path
                } else {
                    format!("{path} {{ {} }}", patterns.join(", "))
                };
                let fields = if shows.is_empty() {
                    "\"\".to_owned()".to_owned()
                } else {
                    format!("[{}].join(\",\")", shows.join(", "))
                };
                let _ = write!(
                    arms,
                    "{pattern} => format!(\"{{{{\\\"kind\\\":\\\"adt\\\",\\\"constructor\\\":{at},\\\"fields\\\":[{{}}]}}}}\", {fields}), "
                );
            }
            format!("match {expr} {{ {arms} }}")
        }
    })
}

const QUOTE: &str = r#"
fn quote(text: &str) -> String {
    let mut out = String::from("\"");
    for c in text.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}
"#;

/// The counting allocator of the harness: the allocations a call performs
/// are the difference of the counter around it.
const COUNTING: &str = r"
use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicU64, Ordering as Count};

static ALLOCATIONS: AtomicU64 = AtomicU64::new(0);

struct Counting;

#[allow(unsafe_code)]
// SAFETY: every call is forwarded unchanged to the system allocator.
unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ALLOCATIONS.fetch_add(1, Count::SeqCst);
        unsafe { System.alloc(layout) }
    }
    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        unsafe { System.dealloc(pointer, layout) }
    }
    unsafe fn realloc(&self, pointer: *mut u8, layout: Layout, size: usize) -> *mut u8 {
        ALLOCATIONS.fetch_add(1, Count::SeqCst);
        unsafe { System.realloc(pointer, layout, size) }
    }
}

#[global_allocator]
static COUNTING: Counting = Counting;

fn allocations() -> u64 {
    ALLOCATIONS.load(Count::SeqCst)
}
";

/// The binary crate that calls the oracle's `function` on each of `cases`
/// and prints, per case, its observable outcome and the allocations the call
/// performed, separated by a tab.
fn oracle_harness(
    ir: &Ir,
    function: &str,
    signature: &(Vec<String>, String),
    cases: &[Case],
) -> Result<String, String> {
    let (parameters, result) = ir
        .defs
        .get(function)
        .ok_or_else(|| format!("no definition `{function}`"))?;
    let fallible = signature.1.ends_with("crate::ComputeError>");
    let result_show = show(ir, result, "value", 0)?;
    // The buffered call yields `Result<Vec<T>, ComputeError>`, shown as an
    // unbuffered fallible list would be.
    let mut out = String::new();
    out.push_str("#![deny(unsafe_code)]\nextern crate alloc;\nuse oracle_crate as o;\n");
    out.push_str(COUNTING);
    out.push_str(QUOTE);
    out.push_str("\nfn run(case: usize) -> (String, u64) {\n    match case {\n");
    for (at, case) in cases.iter().enumerate() {
        let mut binds = String::new();
        let mut passed = Vec::new();
        for (position, ((_, ty), value)) in parameters.iter().zip(&case.values).enumerate() {
            let _ = writeln!(
                binds,
                "            let a{position} = {};",
                literal(ir, ty, value)?
            );
            passed.push(if signature.0[position].starts_with('&') {
                format!("&a{position}[..]")
            } else {
                format!("a{position}")
            });
        }
        let call = if buffered(result, signature) {
            let IrTy::List(element) = result else {
                unreachable!("a buffered result is a list")
            };
            format!(
                "{{ let mut output = alloc::vec![{}; {OUTPUT_CAPACITY}]; o::{function}({}).map(|written| output[..written].to_vec()) }}",
                filler(element)?,
                passed
                    .iter()
                    .cloned()
                    .chain(std::iter::once("&mut output[..]".to_owned()))
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        } else {
            format!("o::{function}({})", passed.join(", "))
        };
        let printing = if fallible {
            format!(
                "Ok(value) => {result_show}, Err(error) => format!(\"{{{{\\\"kind\\\":\\\"overflow\\\",\\\"detail\\\":\\\"{{error:?}}\\\"}}}}\")"
            )
        } else {
            format!("value => {result_show}")
        };
        let _ = write!(
            out,
            "        {at} => {{\n{binds}            let before = allocations();\n            let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {call}));\n            let spent = allocations() - before;\n            match outcome {{\n                Ok(value) => ({{ match value {{ {printing} }} }}, spent),\n                Err(_) => (\"{{\\\"kind\\\":\\\"panic\\\"}}\".to_owned(), spent),\n            }}\n        }}\n",
        );
    }
    out.push_str("        _ => unreachable!(),\n    }\n}\n");
    let _ = write!(
        out,
        "\nfn main() {{\n    let runner = std::thread::Builder::new().stack_size(64 << 20).spawn(|| {{\n        for case in 0..{} {{\n            let (outcome, spent) = run(case);\n            println!(\"{{outcome}}\\t{{spent}}\");\n        }}\n    }}).expect(\"the harness thread starts\");\n    if runner.join().is_err() {{\n        std::process::exit(101);\n    }}\n}}\n",
        cases.len()
    );
    Ok(out)
}

// ---- staging the generated Lean -------------------------------------------

/// LexLean's Lean for a project, compiled by the pinned toolchain, in a
/// directory of the compiled modules.
pub struct Staged {
    _dir: tempfile::TempDir,
    /// The directory of compiled modules (`LEAN_PATH` for the oracle).
    pub library: PathBuf,
}

/// Compile the Lean modules LexLean generates for `project`.
///
/// # Panics
///
/// Panics when the pinned Lean rejects a module LexLean generated: that is a
/// defect of LexLean's generator, not a statement about the oracle.
#[must_use]
pub fn stage(project: &P) -> Staged {
    let rendered = support::rendered(project);
    let dir = tempfile::Builder::new()
        .prefix("lexlean-oracle-lean-")
        .tempdir()
        .expect("tempdir");
    let library = dir.path().join("lib");
    let source = dir.path().join("src");
    std::fs::create_dir_all(&library).expect("lib");
    let compile = |module: &str, text: &str| {
        let relative = module.replace('.', "/");
        let path = source.join(format!("{relative}.lean"));
        std::fs::create_dir_all(path.parent().expect("parent")).expect("source dir");
        std::fs::write(&path, text).expect("source");
        let object = library.join(format!("{relative}.olean"));
        std::fs::create_dir_all(object.parent().expect("parent")).expect("lib dir");
        let ran =
            crate::preservation::pinned_with("lean", dir.path(), std::slice::from_ref(&library))
                .arg(format!("--root={}", source.display()))
                .arg("-o")
                .arg(&object)
                .arg(&path)
                .output()
                .expect("lean runs");
        assert!(
            ran.status.success(),
            "the pinned Lean rejects the module {module} that LexLean generated:\n{}{}",
            String::from_utf8_lossy(&ran.stdout),
            String::from_utf8_lossy(&ran.stderr)
        );
    };
    // The runtime modules the generated modules import (SPEC.md §17.12),
    // first, in their dependency order.
    for runtime in lexlean::backend::runtime::imported_by(
        rendered
            .modules
            .iter()
            .map(|module| module.lean_text.as_str()),
    )
    .expect("the shipped runtime")
    {
        compile(&runtime.name, &runtime.text);
    }
    let names: BTreeSet<&str> = rendered
        .modules
        .iter()
        .map(|module| module.lean_module.as_str())
        .collect();
    let mut remaining: Vec<_> = rendered.modules.iter().collect();
    let mut done: BTreeSet<String> = BTreeSet::new();
    while !remaining.is_empty() {
        let before = remaining.len();
        let mut later = Vec::new();
        for module in remaining {
            let ready = module
                .lean_text
                .lines()
                .filter_map(|line| line.strip_prefix("public import "))
                .map(str::trim)
                .all(|import| !names.contains(import) || done.contains(import));
            if !ready {
                later.push(module);
                continue;
            }
            compile(&module.lean_module, &module.lean_text);
            done.insert(module.lean_module.clone());
        }
        remaining = later;
        assert!(
            remaining.len() < before,
            "the generated modules import each other cyclically"
        );
    }
    Staged { _dir: dir, library }
}

// ---- running the corpus -----------------------------------------------------

/// What the sides did with the seeded inputs of one root.
#[derive(Debug, Clone, Default)]
pub struct Executed {
    /// The oracle's outcome and allocation count on each case.
    pub oracle: Vec<(Json, u64)>,
    /// The outcome of LexLean's own rendering on each case, when it was run.
    pub lexlean: Option<Vec<Json>>,
}

/// The whole run: every root with both sides, and what the oracle's code did.
#[derive(Debug)]
pub struct Run {
    /// Every root of the corpus.
    pub roots: Vec<RootRun>,
    /// The execution of each root the oracle generated a crate for, by root,
    /// or the reason the crate was not built.
    pub executed: BTreeMap<String, Result<Executed, String>>,
    /// The pin the oracle was acquired by.
    pub pin: oracle::Pin,
    /// How long each phase took, in seconds.
    pub timings: BTreeMap<String, f64>,
}

fn module_prefix(project: &P) -> String {
    project
        .read("lexlean.toml")
        .lines()
        .find_map(|line| {
            line.strip_prefix("module_prefix = \"")
                .and_then(|rest| rest.strip_suffix('"'))
        })
        .expect("a module prefix")
        .to_owned()
}

fn effects_of(report: &lexlean::production::RootReport) -> BTreeSet<String> {
    report
        .targets
        .iter()
        .flat_map(|target| target.effects.iter().map(|row| row.effect.clone()))
        .collect()
}

/// LexLean's statement of every eligible root of `project`, with the inputs of
/// the production differential and the boundary inputs of [`seeds::cases`].
pub fn eligible_roots(project: &P) -> Vec<(String, String, Accepted)> {
    use lexlean::production::lower::{linked_modules, lower_root, roots};
    let checked = support::checked_project(project);
    let modules = linked_modules(&checked);
    let limits = support::limits(project);
    let mut cases = crate::differential::cases(project);
    roots(&checked)
        .expect("the eligibility reports")
        .into_iter()
        .map(|root| {
            let lowered = lower_root(&modules, &root.module, &root.name, root.report, &limits)
                .unwrap_or_else(|diagnostic| panic!("{}: {diagnostic:?}", root.report.root));
            let function = lowered
                .program
                .functions
                .first()
                .expect("a root is function 0")
                .clone();
            let contract = Contract {
                parameters: function
                    .types
                    .iter()
                    .map(|ty| lexlean_shape(&lowered.program, ty, &mut Vec::new()))
                    .collect(),
                result: lexlean_shape(&lowered.program, &function.result, &mut Vec::new()),
                failure: lexlean::calculus::rust::fallible_functions(&lowered.program)
                    .expect("a valid program")[0],
                borrowed: Vec::new(),
                output_buffer: false,
                drift: Vec::new(),
            };
            let mut root_cases: Vec<Case> = cases
                .remove(&root.report.root)
                .unwrap_or_default()
                .into_iter()
                .filter(|case| case.function == 0 && case.invalid.is_none())
                .collect();
            // The production differential's seeded inputs, and the boundary
            // inputs the root's own types and literals call for.
            let (parameters, _) = lexlean::production::certificate::root_signature(
                &modules,
                &root.module,
                &root.name,
            )
            .expect("a root signature");
            let types: Vec<lexlean::ir::semantic::SemanticType> = parameters
                .iter()
                .map(|parameter| parameter.r#type.clone())
                .collect();
            let seeds = seeds::cases(
                &modules,
                &root.report.root,
                &lowered.program,
                &types,
                &root_cases,
            );
            root_cases.extend(seeds);
            let lean_module = checked.modules[&root.module].document.lean_module.clone();
            (
                lean_module,
                root.report.root.clone(),
                Accepted {
                    targets: root
                        .report
                        .targets
                        .iter()
                        .map(|t| t.target.clone())
                        .collect(),
                    effects: effects_of(root.report),
                    constructs: root.report.constructs.keys().cloned().collect(),
                    program: lowered.program,
                    contract,
                    cases: root_cases,
                },
            )
        })
        .collect()
}

/// What the boundary seeds of one root amount to.
#[derive(Debug, Clone)]
pub struct SeedAudit {
    /// The root.
    pub root: String,
    /// The inputs of the root that the boundary seeds added to the production
    /// differential's.
    pub boundary: usize,
    /// The constructors of its parameter types that no input builds.
    pub uncovered: Vec<String>,
    /// Its parameter types.
    pub parameters: Vec<lexlean::ir::semantic::SemanticType>,
}

/// For each of `roots`, the eligible roots of `project` as
/// [`eligible_roots`] states them, how many inputs the boundary seeds added
/// and which constructors of its parameter types the inputs `keep` accepts
/// leave unbuilt (no Lean, no oracle). `keep` lets the audit be shown to fail
/// on inputs that leave a constructor out.
#[must_use]
pub fn seed_audit(
    project: &P,
    roots: &[(String, String, Accepted)],
    keep: &dyn Fn(&str, &[Value]) -> bool,
) -> Vec<SeedAudit> {
    use lexlean::production::lower::{linked_modules, roots as eligible};
    let checked = support::checked_project(project);
    let modules = linked_modules(&checked);
    eligible(&checked)
        .expect("the eligibility reports")
        .into_iter()
        .map(|root| {
            let accepted = &roots
                .iter()
                .find(|(_, name, _)| *name == root.report.root)
                .expect("every root is eligible")
                .2;
            let (parameters, _) = lexlean::production::certificate::root_signature(
                &modules,
                &root.module,
                &root.name,
            )
            .expect("a root signature");
            let types: Vec<lexlean::ir::semantic::SemanticType> = parameters
                .iter()
                .map(|parameter| parameter.r#type.clone())
                .collect();
            let inputs: Vec<Vec<Value>> = accepted
                .cases
                .iter()
                .filter(|case| keep(&root.report.root, &case.values))
                .map(|case| case.values.clone())
                .collect();
            SeedAudit {
                root: root.report.root.clone(),
                boundary: accepted
                    .cases
                    .iter()
                    .filter(|case| seeds::is_boundary(case))
                    .count(),
                uncovered: seeds::uncovered(&modules, &types, &inputs),
                parameters: types,
            }
        })
        .collect()
}

/// [`eligible_roots`] of the project of the corpus `name`, stated once per
/// process: stating them is the longest step of the survey and the audits of
/// the boundary inputs need the same ones, so whichever asks first states them
/// and the other waits for it.
pub fn shared_roots(name: &str, project: &P) -> std::sync::Arc<Vec<(String, String, Accepted)>> {
    type Cell = std::sync::Arc<OnceLock<std::sync::Arc<Vec<(String, String, Accepted)>>>>;
    static STATED: std::sync::Mutex<BTreeMap<String, Cell>> =
        std::sync::Mutex::new(BTreeMap::new());
    let cell = STATED
        .lock()
        .expect("the roots' registry")
        .entry(name.to_owned())
        .or_default()
        .clone();
    cell.get_or_init(|| std::sync::Arc::new(eligible_roots(project)))
        .clone()
}

/// The oracle's statement of one root.
fn ask(
    oracle: &Oracle,
    library: &Path,
    module: &str,
    root: &str,
    crate_name: &str,
) -> Result<OracleSide, Unavailable> {
    match oracle.verdict(library, module, root, crate_name)? {
        Verdict::Refused { stage, message } => Ok(OracleSide::Refused {
            stage,
            kind: oracle::refusal_kind(stage, &message),
            message,
        }),
        Verdict::Accepted { ir, package } => {
            let parsed = read_ir(&ir).map_err(|detail| Unavailable::ToolFailed {
                step: "reading the oracle's IR".into(),
                detail,
            })?;
            let source =
                String::from_utf8_lossy(package.get("src/lib.rs").map_or(&[][..], Vec::as_slice))
                    .into_owned();
            let name = last_segment(root);
            let (contract, signature) =
                oracle_contract(&parsed, name, &source).map_err(|detail| {
                    Unavailable::ToolFailed {
                        step: "reading the oracle's crate".into(),
                        detail,
                    }
                })?;
            Ok(OracleSide::Generated {
                ir,
                package,
                contract,
                signature,
            })
        }
    }
}

fn parallel_map<T: Sync, R: Send>(items: &[T], job: impl Fn(&T) -> R + Sync) -> Vec<R> {
    let workers = std::thread::available_parallelism()
        .map_or(1, std::num::NonZeroUsize::get)
        .clamp(1, 4);
    let next = std::sync::atomic::AtomicUsize::new(0);
    let results: std::sync::Mutex<Vec<Option<R>>> =
        std::sync::Mutex::new(items.iter().map(|_| None).collect());
    std::thread::scope(|scope| {
        for _ in 0..workers {
            scope.spawn(|| loop {
                let at = next.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                let Some(item) = items.get(at) else { break };
                let result = job(item);
                results.lock().expect("results")[at] = Some(result);
            });
        }
    });
    results
        .into_inner()
        .expect("results")
        .into_iter()
        .map(|result| result.expect("every job ran"))
        .collect()
}

/// A root put to the oracle: its Lean module, its qualified name, LexLean's
/// statement when it is eligible, and the recorded rejection when it is not.
type Asked = (String, String, Option<Accepted>, Option<(String, String)>);

/// Both sides' statements about every root of every corpus.
///
/// # Errors
///
/// [`Unavailable`] when the oracle cannot be used or fails other than by
/// refusing a program.
pub fn survey(oracle: &Oracle) -> Result<Vec<RootRun>, Unavailable> {
    let corpora = corpus::corpora();
    let per_corpus: Vec<Result<Vec<RootRun>, Unavailable>> = parallel_map(
        &corpora.iter().enumerate().collect::<Vec<_>>(),
        |(index, corpus)| {
            let staged = stage(&corpus.project);
            let mut asked: Vec<Asked> = Vec::new();
            match &corpus.kind {
                Kind::Example | Kind::Probes => {
                    for (module, root, accepted) in
                        shared_roots(&corpus.name, &corpus.project).iter().cloned()
                    {
                        asked.push((module, root, Some(accepted), None));
                    }
                }
                Kind::Rejection(fixture) => {
                    let prefix = module_prefix(&corpus.project);
                    let recorded = corpus::recorded_rejection(fixture);
                    let (_, marked) = corpus::rejection_project(fixture);
                    for (module, root) in marked {
                        asked.push((
                            format!("{prefix}.{module}"),
                            format!("{prefix}.{module}.{root}"),
                            None,
                            Some(recorded.clone()),
                        ));
                    }
                }
            }
            let mut out = Vec::new();
            for (at, (module, root, accepted, rejection)) in asked.into_iter().enumerate() {
                let side = ask(
                    oracle,
                    &staged.library,
                    &module,
                    &root,
                    &format!("oracle-{index}-{at}"),
                )?;
                let lexlean = match (accepted, rejection) {
                    (Some(accepted), _) => LexLeanSide::Eligible(Box::new(accepted)),
                    (None, Some((code, message))) => LexLeanSide::Rejected { code, message },
                    (None, None) => unreachable!("a root is eligible or rejected"),
                };
                out.push(RootRun {
                    corpus: corpus.name.clone(),
                    root,
                    lean_module: module,
                    lexlean,
                    oracle: side,
                });
            }
            Ok(out)
        },
    );
    let mut roots = Vec::new();
    for result in per_corpus {
        roots.extend(result?);
    }
    Ok(roots)
}

fn write_member(dir: &Path, path: &str, files: &BTreeMap<String, Vec<u8>>) {
    for (relative, bytes) in files {
        let file = dir.join(path).join(relative);
        std::fs::create_dir_all(file.parent().expect("parent")).expect("member dir");
        std::fs::write(file, bytes).expect("member file");
    }
}

/// The crate name of the oracle's package of `root`.
fn package_name(package: &BTreeMap<String, Vec<u8>>) -> String {
    let manifest = String::from_utf8_lossy(&package["Cargo.toml"]).into_owned();
    manifest
        .lines()
        .find_map(|line| {
            line.strip_prefix("name = \"")
                .and_then(|rest| rest.strip_suffix('"'))
        })
        .expect("a package name")
        .to_owned()
}

/// The part of cargo's output that says why the build of `member` failed:
/// the diagnostics that point into it.
fn build_errors(stderr: &str, member: &str) -> String {
    let needle = format!("{member}/");
    stderr
        .split("\n\n")
        .filter(|block| block.contains(&needle))
        .take(8)
        .collect::<Vec<_>>()
        .join("\n\n")
}

/// The bound on the build of the generated crates and on one run of one of
/// the generated programs. A crate that loops or a build that hangs is the
/// oracle's finding and ends in a clear failure instead of the job's own
/// time limit; the bounds are generous because a loaded host must not turn
/// them into a flake.
const BUILD_LIMIT: std::time::Duration = std::time::Duration::from_secs(900);
const RUN_LIMIT: std::time::Duration = std::time::Duration::from_secs(120);

/// `cargo` with `arguments` in `dir`, with `dir`'s own target directory and no
/// inherited rustc flags, ended with everything it started when it overruns
/// `limit`.
fn cargo_bounded(
    dir: &Path,
    arguments: &[&str],
    limit: std::time::Duration,
) -> Result<oracle::Ran, String> {
    let mut command =
        std::process::Command::new(std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_owned()));
    command
        .args(arguments)
        .current_dir(dir)
        .env("CARGO_TARGET_DIR", dir.join("target"))
        .env_remove("RUSTFLAGS")
        .env_remove("CARGO_ENCODED_RUSTFLAGS");
    oracle::run(command, limit)
}

/// One run to execute, under a key that is its own: the key names its crate,
/// its harness, and its result, so two runs of the same root (a root and
/// the same root with its oracle crate edited) are two executions that
/// cannot shadow each other.
#[derive(Debug, Clone, Copy)]
pub struct Slot<'a> {
    /// The key the run's result is returned under.
    pub key: &'a str,
    /// The run.
    pub run: &'a RootRun,
}

/// `package`'s manifest with its crate named `name`, which is how a crate of
/// the oracle becomes one member of a workspace beside other crates of the
/// same root.
fn renamed(package: &BTreeMap<String, Vec<u8>>, name: &str) -> BTreeMap<String, Vec<u8>> {
    let mut package = package.clone();
    let manifest = String::from_utf8_lossy(&package["Cargo.toml"]).into_owned();
    let original = package_name(&package);
    let edited = manifest.replacen(
        &format!("name = \"{original}\""),
        &format!("name = \"{name}\""),
        1,
    );
    assert_ne!(edited, manifest, "the crate `{original}` was renamed");
    package.insert("Cargo.toml".to_owned(), edited.into_bytes());
    package
}

/// Build the oracle's crate of each slot in `slots` with its harness in one
/// workspace, run every harness, and return what each printed under its
/// slot's key.
///
/// Every slot has a crate, a harness, and a result of its own: the crate is
/// named for the slot's position, never for the root, so that runs of the
/// same root cannot overwrite one another. Keys that are not distinct are a
/// defect of the caller and panic before anything is built.
///
/// A crate that does not compile is the oracle's finding, reported for its
/// slot; a harness that does not compile is a defect of this module and
/// panics. The build and every run are bounded ([`BUILD_LIMIT`],
/// [`RUN_LIMIT`]): the build overrunning is [`Unavailable::ToolFailed`], a
/// generated program overrunning is the finding of its slot.
///
/// # Errors
///
/// [`Unavailable::ToolFailed`] when the build cannot be run or overruns.
///
/// # Panics
///
/// Panics when two slots share a key, or when a harness, which is this
/// module's, does not build or run.
pub fn execute(
    slots: &[Slot<'_>],
    with_lexlean: bool,
) -> Result<BTreeMap<String, Result<Executed, String>>, Unavailable> {
    use crate::rust_build::{workspace, Built};
    let keys: BTreeSet<&str> = slots.iter().map(|slot| slot.key).collect();
    assert_eq!(
        keys.len(),
        slots.len(),
        "the runs to execute have distinct keys; a shared key would let one run's result stand for another's"
    );
    let dir = tempfile::Builder::new()
        .prefix("lexlean-oracle-run-")
        .tempdir()
        .expect("tempdir");
    let mut members: Vec<String> = Vec::new();
    let mut crate_of: BTreeMap<String, String> = BTreeMap::new();
    let mut harness_of: BTreeMap<String, String> = BTreeMap::new();
    let mut lexlean_of: BTreeMap<String, String> = BTreeMap::new();
    let mut lexlean_built: Vec<Built> = Vec::new();
    let mut failed: BTreeMap<String, Result<Executed, String>> = BTreeMap::new();
    let mut compile_only: BTreeSet<String> = BTreeSet::new();
    for (position, slot) in slots.iter().enumerate() {
        let run = slot.run;
        let key = slot.key.to_owned();
        let crate_name = format!("oracle-s{position}");
        // A root LexLean rejects and the oracle takes has no inputs to run;
        // whether the oracle's crate for it compiles is still evidence.
        if let (LexLeanSide::Rejected { .. }, OracleSide::Generated { package, .. }) =
            (&run.lexlean, &run.oracle)
        {
            write_member(
                dir.path(),
                &format!("o/s{position}"),
                &renamed(package, &crate_name),
            );
            members.push(format!("\"o/s{position}\""));
            crate_of.insert(key.clone(), crate_name);
            compile_only.insert(key);
            continue;
        }
        let (
            LexLeanSide::Eligible(accepted),
            OracleSide::Generated {
                ir,
                package,
                signature,
                ..
            },
        ) = (&run.lexlean, &run.oracle)
        else {
            continue;
        };
        let parsed = read_ir(ir).expect("the IR parsed when the verdict was asked");
        write_member(
            dir.path(),
            &format!("o/s{position}"),
            &renamed(package, &crate_name),
        );
        let harness_name = format!("oh{position}");
        let harness = match oracle_harness(&parsed, run.function(), signature, &accepted.cases) {
            Ok(text) => text,
            Err(reason) => {
                failed.insert(key, Err(format!("the harness cannot be written: {reason}")));
                continue;
            }
        };
        let mut files = BTreeMap::new();
        files.insert(
            "Cargo.toml".to_owned(),
            format!(
                "[package]\nname = \"{harness_name}\"\nversion = \"0.0.0\"\nedition = \"2021\"\npublish = false\n\n[dependencies]\noracle_crate = {{ package = \"{crate_name}\", path = \"../../o/s{position}\" }}\n"
            )
            .into_bytes(),
        );
        files.insert("src/main.rs".to_owned(), harness.into_bytes());
        write_member(dir.path(), &format!("h/{harness_name}"), &files);
        members.push(format!("\"o/s{position}\""));
        members.push(format!("\"h/{harness_name}\""));
        crate_of.insert(key.clone(), crate_name);
        harness_of.insert(key.clone(), harness_name);
        if with_lexlean {
            let profile_name = if accepted.targets.iter().any(|t| t == "rust-core") {
                "rust-core"
            } else {
                "rust-std"
            };
            let profile = Profile::named(profile_name).expect("a profile");
            let lexlean_name = format!("lexlean_{position}");
            let fallible = accepted.contract.failure;
            let manifest = Manifest {
                spec: MANIFEST_SPEC.to_owned(),
                name: lexlean_name.clone(),
                version: "1.0.0".to_owned(),
                profile: profile_name.to_owned(),
                program: accepted.program.clone(),
                exports: vec![Export {
                    function: 0,
                    name: "root".to_owned(),
                    parameters: vec![Passing::Own; accepted.program.functions[0].types.len()],
                    errors: if fallible {
                        Errors::Overflow
                    } else {
                        Errors::None
                    },
                }],
                sources: Vec::new(),
            };
            let rendered = package_of(&manifest, &run.root);
            let arguments: Vec<Vec<Value>> = accepted
                .cases
                .iter()
                .map(|case| case.values.clone())
                .collect();
            let passing = vec![Passing::Own; accepted.program.functions[0].types.len()];
            let calls = [crate::rust_harness::Calls {
                entry: 0,
                function: "root",
                passing: &passing,
                arguments: &arguments,
            }];
            let harness = crate::rust_harness::render_calls(
                &accepted.program,
                profile,
                &lexlean_name,
                &calls,
            )
            .unwrap_or_else(|reason| panic!("{}: {reason}", run.root));
            lexlean_of.insert(key, lexlean_name.clone());
            lexlean_built.push(Built {
                name: lexlean_name,
                files: rendered.files,
                harness,
            });
        }
    }
    // The LexLean packages and their harnesses go through the Rust backend's
    // own workspace writer; the oracle's crates are added to the manifest.
    workspace(dir.path(), &lexlean_built);
    let existing = std::fs::read_to_string(dir.path().join("Cargo.toml")).expect("manifest");
    let mut listed = existing
        .split_once("members = [")
        .and_then(|(_, rest)| rest.split_once(']'))
        .map(|(list, _)| list.trim().to_owned())
        .unwrap_or_default();
    for member in &members {
        if !listed.is_empty() {
            listed.push_str(", ");
        }
        listed.push_str(member);
    }
    std::fs::write(
        dir.path().join("Cargo.toml"),
        format!("[workspace]\nresolver = \"2\"\nmembers = [{listed}]\n"),
    )
    .expect("workspace manifest");
    let build = cargo_bounded(
        dir.path(),
        &["build", "--offline", "--workspace", "--keep-going"],
        BUILD_LIMIT,
    )
    .map_err(|detail| Unavailable::ToolFailed {
        step: "cargo build of the generated crates".to_owned(),
        detail: format!("{detail} (the bound is {} s)", BUILD_LIMIT.as_secs()),
    })?;
    let stderr = build.stderr;
    let broken: BTreeSet<String> = stderr
        .lines()
        .filter_map(|line| {
            line.strip_prefix("error: could not compile `")
                .and_then(|rest| rest.split('`').next())
                .map(str::to_owned)
        })
        .collect();
    let mut out = failed;
    let does_not_compile = |name: &str, position: usize| {
        format!(
            "the oracle's crate `{name}` does not compile:\n{}",
            build_errors(&stderr, &format!("o/s{position}"))
        )
    };
    for (position, slot) in slots.iter().enumerate() {
        let key = slot.key;
        if compile_only.contains(key) {
            let name = &crate_of[key];
            out.insert(
                key.to_owned(),
                if broken.contains(name) {
                    Err(does_not_compile(name, position))
                } else {
                    Ok(Executed::default())
                },
            );
            continue;
        }
        let Some(harness_name) = harness_of.get(key) else {
            continue;
        };
        let name = &crate_of[key];
        if broken.contains(name) {
            out.insert(key.to_owned(), Err(does_not_compile(name, position)));
            continue;
        }
        if let Some(own) = lexlean_of.get(key) {
            assert!(
                !broken.contains(own) && !broken.contains(&format!("run_{own}")),
                "LexLean's package of {} does not build:\n{}",
                slot.run.root,
                build_errors(&stderr, &format!("p/{own}"))
            );
        }
        assert!(
            !broken.contains(harness_name),
            "the oracle harness of {} does not build:\n{}",
            slot.run.root,
            build_errors(&stderr, &format!("h/{harness_name}"))
        );
        let exe = |name: &str| {
            dir.path()
                .join("target/debug")
                .join(format!("{name}{}", std::env::consts::EXE_SUFFIX))
        };
        let ran = match oracle::run(std::process::Command::new(exe(harness_name)), RUN_LIMIT) {
            Ok(ran) => ran,
            Err(reason) => {
                out.insert(
                    key.to_owned(),
                    Err(format!(
                        "the oracle's crate did not finish: {reason} (the bound is {} s)",
                        RUN_LIMIT.as_secs()
                    )),
                );
                continue;
            }
        };
        if ran.status != Some(0) {
            out.insert(
                key.to_owned(),
                Err(format!(
                    "the oracle's crate ended the run with {:?}: {}",
                    ran.status,
                    ran.stderr.lines().last().unwrap_or("")
                )),
            );
            continue;
        }
        let mut oracle_rows = Vec::new();
        for line in ran.stdout.lines() {
            let (json, spent) = line.rsplit_once('\t').expect("a tab separated outcome");
            oracle_rows.push((
                serde_json::from_str(json)
                    .unwrap_or_else(|e| panic!("{}: {e}: {json}", slot.run.root)),
                spent.parse::<u64>().expect("an allocation count"),
            ));
        }
        let lexlean = lexlean_of.get(key).map(|own| {
            let ran = oracle::run(
                std::process::Command::new(exe(&format!("run_{own}"))),
                RUN_LIMIT,
            )
            .unwrap_or_else(|reason| {
                panic!(
                    "LexLean's rendering of {} did not finish: {reason} (the bound is {} s)",
                    slot.run.root,
                    RUN_LIMIT.as_secs()
                )
            });
            assert_eq!(
                ran.status,
                Some(0),
                "LexLean's rendering of {} fails: {}",
                slot.run.root,
                ran.stderr
            );
            ran.stdout
                .lines()
                .filter(|line| !line.starts_with("work "))
                .map(|line| serde_json::from_str(line).expect("json"))
                .collect::<Vec<Json>>()
        });
        out.insert(
            key.to_owned(),
            Ok(Executed {
                oracle: oracle_rows,
                lexlean,
            }),
        );
    }
    Ok(out)
}

fn package_of(manifest: &Manifest, root: &str) -> lexlean::calculus::rust::package::Package {
    package(manifest).unwrap_or_else(|reason| panic!("{root}: {reason}"))
}

// ---- comparing ----------------------------------------------------------------

fn contract_json(contract: &Contract) -> Json {
    serde_json::to_value(contract).expect("a contract serializes")
}

/// The class of a LexLean rejection, from the words of its diagnostic.
fn rejection_class(message: &str) -> &'static str {
    if message.contains("type parameters") {
        "type-parameters"
    } else if message.contains("closure would escape") {
        "closure-escape"
    } else if message.contains("holds a proposition") {
        "proposition"
    } else if message.contains("type universe") {
        "type-universe"
    } else if message.contains("does not fit") {
        "literal-width"
    } else {
        "other"
    }
}

/// What kind of thing an outcome is: a value, an overflow, or a panic.
fn kind_of(outcome: &Json) -> String {
    match outcome.get("kind").and_then(Json::as_str) {
        Some("overflow") => "overflow",
        Some("panic") => "panic",
        _ => "value",
    }
    .to_owned()
}

/// An outcome with the oracle's own name for an overflow removed: an
/// overflow is an overflow, whichever of its errors the oracle raised.
fn normalized(outcome: &Json) -> Json {
    if kind_of(outcome) == "overflow" {
        serde_json::json!({"kind": "overflow"})
    } else {
        outcome.clone()
    }
}

/// Every difference of one root between the sides.
///
/// `executed` is what the oracle's code did on the root's inputs, or the reason
/// it could not be run; `None` for a root the oracle did not generate.
#[must_use]
pub fn differences(run: &RootRun, executed: Option<&Result<Executed, String>>) -> Vec<Diff> {
    let mut out = Vec::new();
    let mut push = |kind: DiffKind, key: String, lexlean: Json, oracle: Json| {
        out.push(Diff {
            corpus: run.corpus.clone(),
            root: run.root.clone(),
            kind,
            key,
            lexlean,
            oracle,
        });
    };
    match (&run.lexlean, &run.oracle) {
        (
            LexLeanSide::Eligible(accepted),
            OracleSide::Refused {
                stage,
                kind,
                message,
            },
        ) => push(
            DiffKind::Acceptance,
            format!("oracle-refuses:{kind}"),
            serde_json::json!({"verdict": "eligible", "targets": accepted.targets}),
            serde_json::json!({"verdict": "refused", "stage": format!("{stage:?}"), "kind": kind, "message": message}),
        ),
        (LexLeanSide::Rejected { code, message }, OracleSide::Generated { package, .. }) => {
            let rustc = match executed {
                Some(Ok(_)) => serde_json::json!({"compiles": true}),
                Some(Err(reason)) => serde_json::json!({
                    "compiles": false,
                    "reason": reason.chars().take(400).collect::<String>(),
                }),
                None => Json::Null,
            };
            push(
                DiffKind::Acceptance,
                format!("oracle-accepts:{}", rejection_class(message)),
                serde_json::json!({"verdict": "rejected", "code": code, "message": message}),
                serde_json::json!({"verdict": "generated", "lib_rs_sha256": sha256_hex(&package["src/lib.rs"]), "rustc": rustc}),
            );
        }
        // Both refuse, and the oracle's reason is recorded: a root LexLean
        // rejects that the oracle also refuses is an agreement on the verdict
        // and a difference in its reason, and the table names the reason of
        // every one, so a refusal for another reason is a disagreement.
        (
            LexLeanSide::Rejected { code, message },
            OracleSide::Refused {
                stage,
                kind,
                message: refusal,
            },
        ) => push(
            DiffKind::Acceptance,
            format!("both-refuse:{kind}"),
            serde_json::json!({"verdict": "rejected", "code": code, "message": message}),
            serde_json::json!({"verdict": "refused", "stage": format!("{stage:?}"), "kind": kind, "message": refusal}),
        ),
        (
            LexLeanSide::Eligible(accepted),
            OracleSide::Generated {
                contract,
                signature,
                package,
                ..
            },
        ) => {
            let evidence = |detail: Json| {
                (
                    serde_json::json!({"contract": contract_json(&accepted.contract), "detail": detail}),
                    serde_json::json!({"contract": contract_json(contract), "signature": {"parameters": signature.0, "result": signature.1}, "lib_rs_sha256": sha256_hex(&package["src/lib.rs"])}),
                )
            };
            if accepted.contract.parameters.len() != contract.parameters.len() {
                let (l, o) = evidence(Json::Null);
                push(DiffKind::Contract, "arity".into(), l, o);
            } else {
                for (at, (left, right)) in accepted
                    .contract
                    .parameters
                    .iter()
                    .zip(&contract.parameters)
                    .enumerate()
                {
                    if left != right {
                        let (l, o) = evidence(serde_json::json!({"parameter": at}));
                        push(DiffKind::Contract, format!("shape:parameter:{at}"), l, o);
                    }
                }
            }
            if accepted.contract.result != contract.result {
                let (l, o) = evidence(Json::Null);
                push(DiffKind::Contract, "shape:result".into(), l, o);
            }
            if accepted.contract.failure != contract.failure {
                let (l, o) = evidence(Json::Null);
                push(
                    DiffKind::Contract,
                    if accepted.contract.failure {
                        "failure-typing:only-lexlean".into()
                    } else {
                        "failure-typing:only-oracle".into()
                    },
                    l,
                    o,
                );
            }
            if !contract.drift.is_empty() {
                let (l, o) = evidence(serde_json::json!({"drift": contract.drift}));
                push(DiffKind::Contract, "signature-drift".into(), l, o);
            }
            if !contract.borrowed.is_empty() {
                let (l, o) = evidence(serde_json::json!({"borrowed": contract.borrowed}));
                push(DiffKind::Contract, "borrowed-parameter".into(), l, o);
            }
            if contract.output_buffer != accepted.contract.output_buffer {
                let (l, o) = evidence(Json::Null);
                push(DiffKind::Contract, "output-buffer".into(), l, o);
            }
            match executed {
                None => {}
                Some(Err(reason)) => push(
                    DiffKind::Behavior,
                    "oracle-crate-does-not-run".into(),
                    Json::Null,
                    serde_json::json!({"reason": reason}),
                ),
                Some(Ok(done)) => {
                    let mut mismatched: BTreeMap<String, Vec<Observed>> = BTreeMap::new();
                    let mut core_allocations: Vec<Observed> = Vec::new();
                    let rust_core = accepted.targets.iter().any(|t| t == "rust-core");
                    for (at, case) in accepted.cases.iter().enumerate() {
                        let expected = lexlean::calculus::rust::observable(&case.outcome)
                            .expect("an observable outcome");
                        let Some((got, allocations)) = done.oracle.get(at) else {
                            continue;
                        };
                        let row = Observed {
                            arguments: case.arguments.clone(),
                            lexlean: expected.clone(),
                            oracle: got.clone(),
                            allocations: *allocations,
                        };
                        if normalized(got) != expected {
                            mismatched
                                .entry(format!("outcome:{}/{}", kind_of(&expected), kind_of(got)))
                                .or_default()
                                .push(row.clone());
                        }
                        if rust_core && *allocations > 0 {
                            core_allocations.push(row);
                        }
                    }
                    if done.oracle.len() != accepted.cases.len() {
                        push(
                            DiffKind::Behavior,
                            "case-count".into(),
                            serde_json::json!({"cases": accepted.cases.len()}),
                            serde_json::json!({"cases": done.oracle.len()}),
                        );
                    }
                    for (key, rows) in mismatched {
                        push(
                            DiffKind::Behavior,
                            key,
                            serde_json::json!({"cases": rows.iter().map(|r| serde_json::json!({"arguments": r.arguments, "outcome": r.lexlean})).collect::<Vec<_>>()}),
                            serde_json::json!({"cases": rows.iter().map(|r| serde_json::json!({"arguments": r.arguments, "outcome": r.oracle})).collect::<Vec<_>>()}),
                        );
                    }
                    if !core_allocations.is_empty() {
                        push(
                            DiffKind::Resource,
                            "allocation:rust-core-root".into(),
                            serde_json::json!({"targets": accepted.targets, "effects": accepted.effects}),
                            serde_json::json!({"allocating_cases": core_allocations.iter().map(|r| serde_json::json!({"arguments": r.arguments, "allocations": r.allocations})).collect::<Vec<_>>()}),
                        );
                    }
                    if let Some(own) = &done.lexlean {
                        let mut wrong = Vec::new();
                        for (at, case) in accepted.cases.iter().enumerate() {
                            let expected = lexlean::calculus::rust::observable(&case.outcome)
                                .expect("an observable outcome");
                            if own.get(at) != Some(&expected) {
                                wrong.push(serde_json::json!({"arguments": case.arguments, "interpreter": expected, "rendering": own.get(at)}));
                            }
                        }
                        if !wrong.is_empty() {
                            push(
                                DiffKind::Behavior,
                                "lexlean-rendering-differs-from-its-interpreter".into(),
                                serde_json::json!({"cases": wrong}),
                                Json::Null,
                            );
                        }
                    }
                }
            }
        }
    }
    out
}

// ---- the tables ------------------------------------------------------------------

/// One row of the table of known deliberate differences.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct KnownDifference {
    /// The row's identifier.
    pub id: String,
    /// The kind of difference.
    pub kind: DiffKind,
    /// The difference's name.
    pub key: String,
    /// Why LexLean and the oracle are meant to differ here.
    pub reason: String,
    /// The roots the difference is known on, exactly.
    pub roots: Vec<String>,
}

/// `oracle/known-differences.toml`.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct KnownDifferences {
    /// The schema tag.
    pub spec: String,
    /// The version of the table, raised whenever a row changes.
    pub version: u64,
    /// The rows.
    pub difference: Vec<KnownDifference>,
}

impl KnownDifferences {
    /// The committed table.
    ///
    /// # Panics
    ///
    /// Panics when the table is not valid.
    #[must_use]
    pub fn committed() -> Self {
        let text = std::fs::read_to_string(
            repo_root()
                .join("oracle/known-differences.toml")
                .as_std_path(),
        )
        .expect("oracle/known-differences.toml");
        let table = Self::parse(&text)
            .unwrap_or_else(|reason| panic!("oracle/known-differences.toml: {reason}"));
        TableVersions::committed()
            .check("known-differences.toml", table.version, &table.digest())
            .unwrap_or_else(|reason| panic!("oracle/known-differences.toml: {reason}"));
        table
    }

    /// The SHA-256 of the table's rows: what a version of the table names.
    #[must_use]
    pub fn digest(&self) -> String {
        rows_digest(
            &self.spec,
            &serde_json::json!({"difference": self.difference}),
        )
    }

    /// Parse and validate a table.
    ///
    /// # Errors
    ///
    /// The first way it is not a table: an unknown field, a wrong schema, a
    /// repeated identifier, a row without a reason or without roots, or a
    /// root repeated within a row.
    pub fn parse(text: &str) -> Result<Self, String> {
        let table: Self = toml::from_str(text).map_err(|error| error.to_string())?;
        if table.spec != "lexlean/oracle-differences/1" {
            return Err(format!("unknown schema `{}`", table.spec));
        }
        if table.version == 0 {
            return Err("the table's version is positive".into());
        }
        let mut ids = BTreeSet::new();
        for row in &table.difference {
            if !ids.insert(row.id.clone()) {
                return Err(format!("`{}` is a row twice", row.id));
            }
            if row.reason.trim().is_empty() {
                return Err(format!("`{}` states no reason", row.id));
            }
            if row.roots.is_empty() {
                return Err(format!(
                    "`{}` names no root: a known difference is known on roots",
                    row.id
                ));
            }
            if row.roots.iter().collect::<BTreeSet<_>>().len() != row.roots.len() {
                return Err(format!("`{}` names a root twice", row.id));
            }
        }
        Ok(table)
    }
}

// ---- the versions of the tables ------------------------------------------------

/// The SHA-256 of a table's rows and schema tag, in a canonical form (JSON with
/// sorted keys): comments, blank lines, and the order of the keys of a row do
/// not change it; a word of a row does.
fn rows_digest(spec: &str, rows: &Json) -> String {
    let canonical = serde_json::json!({"spec": spec, "rows": rows});
    sha256_hex(&serde_json::to_vec(&canonical).expect("rows serialize"))
}

/// One released version of a table and the digest of its rows.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Released {
    /// The version.
    pub version: u64,
    /// The digest of the rows at that version.
    pub rows_sha256: String,
}

/// The history of one table.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TableHistory {
    /// The table's file in `oracle/`.
    pub file: String,
    /// Every version, in order.
    pub history: Vec<Released>,
}

/// `oracle/table-versions.toml`: what each version of each table is.
///
/// A table's `version` used to be a number nobody could be held to. Here it
/// names the digest of its rows, so that a row that changes without a new
/// version, and a version that is raised with no change, both fail.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TableVersions {
    /// The schema tag.
    pub spec: String,
    /// The tables.
    pub table: Vec<TableHistory>,
}

impl TableVersions {
    /// The committed history.
    ///
    /// # Panics
    ///
    /// Panics when the history is not valid.
    #[must_use]
    pub fn committed() -> Self {
        let text =
            std::fs::read_to_string(repo_root().join("oracle/table-versions.toml").as_std_path())
                .expect("oracle/table-versions.toml");
        Self::parse(&text).unwrap_or_else(|reason| panic!("oracle/table-versions.toml: {reason}"))
    }

    /// Parse and validate a history.
    ///
    /// # Errors
    ///
    /// The first way it is not a history: an unknown schema or field, a table
    /// twice, a version that does not follow the one before it, a digest that
    /// is not 64 lowercase hexadecimal digits, or a version with the rows of
    /// the one before (a version that changed nothing).
    pub fn parse(text: &str) -> Result<Self, String> {
        let versions: Self = toml::from_str(text).map_err(|error| error.to_string())?;
        if versions.spec != "lexlean/oracle-table-versions/1" {
            return Err(format!("unknown schema `{}`", versions.spec));
        }
        let mut files = BTreeSet::new();
        for table in &versions.table {
            if !files.insert(&table.file) {
                return Err(format!("`{}` is a table twice", table.file));
            }
            if table.history.is_empty() {
                return Err(format!("`{}` has no version", table.file));
            }
            for (at, released) in table.history.iter().enumerate() {
                if released.version != at as u64 + 1 {
                    return Err(format!(
                        "`{}`: version {} follows {} (versions are 1, 2, 3, ...)",
                        table.file, released.version, at
                    ));
                }
                if released.rows_sha256.len() != 64
                    || !released
                        .rows_sha256
                        .bytes()
                        .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
                {
                    return Err(format!(
                        "`{}` version {}: the digest is not 64 lowercase hexadecimal digits",
                        table.file, released.version
                    ));
                }
                if at > 0 && table.history[at - 1].rows_sha256 == released.rows_sha256 {
                    return Err(format!(
                        "`{}` version {} has the rows of version {}: a version that changed nothing",
                        table.file,
                        released.version,
                        released.version - 1
                    ));
                }
            }
        }
        Ok(versions)
    }

    /// Whether the table `file` at `version` with rows of `digest` is the last
    /// version this history records.
    ///
    /// # Errors
    ///
    /// Says which of the three it is: a table with no history, a version the
    /// history does not end at, or rows that are not those of the version.
    pub fn check(&self, file: &str, version: u64, digest: &str) -> Result<(), String> {
        let table = self
            .table
            .iter()
            .find(|table| table.file == file)
            .ok_or_else(|| format!("oracle/table-versions.toml has no history for {file}"))?;
        let last = table.history.last().expect("a history has a version");
        if version != last.version {
            return Err(format!(
                "the table is at version {version} and oracle/table-versions.toml records it up to version {}; record the rows_sha256 of version {version} (`{digest}`) there, or restore the version",
                last.version
            ));
        }
        if digest != last.rows_sha256 {
            return Err(format!(
                "the rows changed (their digest is `{digest}`) but the table is still at version {version}, which oracle/table-versions.toml records as `{}`; raise `version` and record the new rows_sha256 there",
                last.rows_sha256
            ));
        }
        Ok(())
    }
}

/// How the differences of a run stand against the table.
#[derive(Debug, Clone, Default)]
pub struct Judgement {
    /// Differences no row names: disagreements.
    pub unlisted: Vec<Diff>,
    /// Rows (with a root) that no difference of the run matches.
    pub stale: Vec<(String, String)>,
    /// How many differences a row names.
    pub accepted: usize,
}

/// Judge `diffs` against `table`: every difference must be named by a row
/// that lists its root, and every root a row lists must show the difference.
#[must_use]
pub fn judge(diffs: &[Diff], table: &KnownDifferences) -> Judgement {
    let mut judgement = Judgement::default();
    let mut matched: BTreeSet<(String, String)> = BTreeSet::new();
    for diff in diffs {
        let row = table.difference.iter().find(|row| {
            row.kind == diff.kind && row.key == diff.key && row.roots.contains(&diff.root)
        });
        match row {
            Some(row) => {
                matched.insert((row.id.clone(), diff.root.clone()));
                judgement.accepted += 1;
            }
            None => judgement.unlisted.push(diff.clone()),
        }
    }
    for row in &table.difference {
        for root in &row.roots {
            if !matched.contains(&(row.id.clone(), root.clone())) {
                judgement.stale.push((row.id.clone(), root.clone()));
            }
        }
    }
    judgement
}

/// A row of the table of known non-overlap.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NonOverlap {
    /// The registry construct.
    pub key: String,
    /// `not-runtime` for a construct no runtime value realizes,
    /// `oracle-refuses` for one the oracle refuses on its probe,
    /// `oracle-breaks` for one the oracle takes but whose crate rustc
    /// rejects, and `instantiated` for a construct that no root's closure
    /// holds because it is instantiated before it is realized.
    pub kind: String,
    /// The registry's disposition of the construct, for `not-runtime`.
    #[serde(default)]
    pub disposition: Option<String>,
    /// The probe root whose closure realizes the construct, for
    /// `oracle-refuses`.
    #[serde(default)]
    pub probe: Option<String>,
    /// The kind of refusal the oracle gives the probe (`does-not-compile`
    /// for `oracle-breaks`).
    #[serde(default)]
    pub refusal: Option<String>,
    /// `construct` when the probe realizes this construct alone (besides the
    /// scaffolding every probe has), `closure` when the probe is a root of a
    /// certified example whose closure realizes it among others.
    #[serde(default)]
    pub scope: Option<String>,
    /// Why the construct is outside the overlap.
    pub reason: String,
}

/// `oracle/known-non-overlap.toml`.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NonOverlapTable {
    /// The schema tag.
    pub spec: String,
    /// The version of the table.
    pub version: u64,
    /// The rows.
    pub construct: Vec<NonOverlap>,
}

impl NonOverlapTable {
    /// The committed table.
    ///
    /// # Panics
    ///
    /// Panics when the table is not valid.
    #[must_use]
    pub fn committed() -> Self {
        let text = std::fs::read_to_string(
            repo_root()
                .join("oracle/known-non-overlap.toml")
                .as_std_path(),
        )
        .expect("oracle/known-non-overlap.toml");
        let table = Self::parse(&text)
            .unwrap_or_else(|reason| panic!("oracle/known-non-overlap.toml: {reason}"));
        TableVersions::committed()
            .check("known-non-overlap.toml", table.version, &table.digest())
            .unwrap_or_else(|reason| panic!("oracle/known-non-overlap.toml: {reason}"));
        table
    }

    /// The SHA-256 of the table's rows: what a version of the table names.
    #[must_use]
    pub fn digest(&self) -> String {
        rows_digest(
            &self.spec,
            &serde_json::json!({"construct": self.construct}),
        )
    }

    /// Parse and validate a table.
    ///
    /// # Errors
    ///
    /// The first way it is not a table.
    pub fn parse(text: &str) -> Result<Self, String> {
        let table: Self = toml::from_str(text).map_err(|error| error.to_string())?;
        if table.spec != "lexlean/oracle-overlap/1" {
            return Err(format!("unknown schema `{}`", table.spec));
        }
        if table.version == 0 {
            return Err("the table's version is positive".into());
        }
        let mut keys = BTreeSet::new();
        for row in &table.construct {
            if !keys.insert(row.key.clone()) {
                return Err(format!("`{}` is a row twice", row.key));
            }
            if row.reason.trim().is_empty() {
                return Err(format!("`{}` states no reason", row.key));
            }
            match row.kind.as_str() {
                "not-runtime" => {
                    if row.disposition.is_none() {
                        return Err(format!("`{}` states no disposition", row.key));
                    }
                }
                "oracle-refuses" | "oracle-breaks" | "instantiated" => {
                    if row.probe.is_none() || row.refusal.is_none() || row.scope.is_none() {
                        return Err(format!("`{}` names no probe, refusal, or scope", row.key));
                    }
                }
                other => return Err(format!("`{}` has the unknown kind `{other}`", row.key)),
            }
        }
        Ok(table)
    }
}

/// How the registry's constructs stand against the corpus and the table.
#[derive(Debug, Clone, Default)]
pub struct Coverage {
    /// Runtime constructs realized in the closure of a root the oracle
    /// accepted and rustc compiled: the construct is in the program that was
    /// compared. Whether every input reached it is the seeds' reach, which is
    /// audited and planted against, not proven.
    pub overlapping: BTreeSet<String>,
    /// Constructs a row of the table lists.
    pub listed: BTreeSet<String>,
    /// Problems: a construct in neither, a row that does not hold, a row for
    /// a construct that is exercised.
    pub problems: Vec<String>,
}

/// The registry's constructs and their dispositions.
///
/// # Panics
///
/// Panics when the registry file is not valid.
#[must_use]
pub fn registry() -> BTreeMap<String, String> {
    let text = std::fs::read_to_string(
        repo_root()
            .join("language/production-1.2.toml")
            .as_std_path(),
    )
    .expect("the registry");
    let data: toml::Value = text.parse().expect("the registry parses");
    data["construct"]
        .as_array()
        .expect("constructs")
        .iter()
        .map(|row| {
            (
                row["key"].as_str().expect("key").to_owned(),
                row["disposition"].as_str().expect("disposition").to_owned(),
            )
        })
        .collect()
}

/// Check every construct of `registry` against the corpus run and the table.
#[must_use]
pub fn coverage(
    registry: &BTreeMap<String, String>,
    run: &Run,
    table: &NonOverlapTable,
) -> Coverage {
    let roots = &run.roots;
    let mut out = Coverage::default();
    let by_root: BTreeMap<&str, &RootRun> =
        roots.iter().map(|run| (run.root.as_str(), run)).collect();
    let mut realized: BTreeSet<String> = BTreeSet::new();
    for root in roots {
        if let LexLeanSide::Eligible(accepted) = &root.lexlean {
            realized.extend(accepted.constructs.iter().cloned());
            // A construct overlaps when a root that realizes it in its
            // closure was taken by the oracle, built, and compared: an oracle
            // crate that does not compile is compared with nothing.
            if matches!(root.oracle, OracleSide::Generated { .. })
                && matches!(run.executed.get(&root.root), Some(Ok(_)))
            {
                out.overlapping.extend(accepted.constructs.iter().cloned());
            }
        }
    }
    for row in &table.construct {
        out.listed.insert(row.key.clone());
        let Some(disposition) = registry.get(&row.key) else {
            out.problems.push(format!(
                "`{}` is listed but is not a construct of the registry",
                row.key
            ));
            continue;
        };
        if row.kind == "not-runtime" {
            if Some(disposition) != row.disposition.as_ref() || disposition == "runtime" {
                out.problems.push(format!(
                    "`{}` is listed as {:?}, and the registry's disposition is `{disposition}`",
                    row.key, row.disposition
                ));
            }
            continue;
        }
        if disposition != "runtime" {
            out.problems.push(format!(
                "`{}` is listed as outside the overlap by the oracle but its disposition is `{disposition}`",
                row.key
            ));
            continue;
        }
        if out.overlapping.contains(&row.key) {
            out.problems.push(format!(
                "`{}` is listed as outside the overlap but a root the oracle accepted and compared realizes it",
                row.key
            ));
        }
        let probe = row.probe.as_deref().unwrap_or_default();
        let Some(run_of_probe) = by_root.get(probe) else {
            out.problems.push(format!(
                "`{}`: the probe `{probe}` is not a root of the corpus",
                row.key
            ));
            continue;
        };
        let LexLeanSide::Eligible(accepted) = &run_of_probe.lexlean else {
            out.problems.push(format!(
                "`{}`: LexLean no longer takes the probe `{probe}`",
                row.key
            ));
            continue;
        };
        if row.kind == "instantiated" {
            if realized.contains(&row.key) {
                out.problems.push(format!(
                    "`{}` is listed as instantiated before realization but a root's closure holds it",
                    row.key
                ));
            }
        } else if !accepted.constructs.contains(&row.key) {
            out.problems.push(format!(
                "`{}`: the probe `{probe}` does not realize it",
                row.key
            ));
        }
        let observed = match (&run_of_probe.oracle, run.executed.get(probe)) {
            (OracleSide::Refused { kind, .. }, _) => kind.clone(),
            (OracleSide::Generated { .. }, Some(Err(_))) => "does-not-compile".to_owned(),
            (OracleSide::Generated { .. }, _) => "accepted".to_owned(),
        };
        if Some(&observed) != row.refusal.as_ref() {
            out.problems.push(format!(
                "`{}`: the oracle's answer on `{probe}` is `{observed}`, not `{:?}`",
                row.key, row.refusal
            ));
        }
    }
    for (key, disposition) in registry {
        if !out.listed.contains(key) && !out.overlapping.contains(key) {
            out.problems.push(format!(
                "`{key}` ({disposition}) is exercised by no root the oracle accepts and compares and is in no row of the table of known non-overlap"
            ));
        }
    }
    out
}

// ---- the verdict on the whole ----------------------------------------------------

/// The statement of a run: what a gate may conclude from the oracle.
#[derive(Debug)]
pub enum Validation {
    /// Every root was compared, and every difference is a known one.
    Validated(Summary),
    /// Some difference is not a known one, or a known one is gone: LexLean
    /// and the oracle disagree, and this is the evidence of both sides.
    Disagreement(Vec<Diff>, Vec<(String, String)>),
    /// The oracle could not say anything. This is not a validation.
    Unavailable(Unavailable),
}

/// What a validated run covered.
#[derive(Debug, Clone, Default)]
pub struct Summary {
    /// Roots compared.
    pub roots: usize,
    /// Roots the oracle generated Rust for.
    pub generated: usize,
    /// Roots the oracle refused.
    pub refused: usize,
    /// Cases whose outcomes were compared.
    pub cases: usize,
    /// Differences that rows of the table name.
    pub known_differences: usize,
}

impl Validation {
    /// Whether the oracle validated: only a validated run does.
    #[must_use]
    pub fn validated(&self) -> bool {
        matches!(self, Self::Validated(_))
    }

    /// The evidence of a disagreement as the text a failing gate prints:
    /// each difference with LexLean's evidence and the oracle's.
    #[must_use]
    pub fn report(&self) -> String {
        match self {
            Self::Validated(summary) => format!("validated: {summary:?}"),
            Self::Unavailable(reason) => format!("NOT VALIDATED: {reason}"),
            Self::Disagreement(unlisted, stale) => {
                let mut text = String::from("LexLean and the oracle disagree.\n");
                for diff in unlisted {
                    let _ = writeln!(
                        text,
                        "- {} {:?} `{}`\n  lexlean: {}\n  oracle:  {}",
                        diff.root,
                        diff.kind,
                        diff.key,
                        trim_json(&diff.lexlean),
                        trim_json(&diff.oracle)
                    );
                }
                for (row, root) in stale {
                    let _ = writeln!(
                        text,
                        "- the known difference {row} lists {root}, which no longer shows it"
                    );
                }
                text
            }
        }
    }
}

fn trim_json(value: &Json) -> String {
    let text = value.to_string();
    if text.chars().count() > 900 {
        format!("{}...", text.chars().take(900).collect::<String>())
    } else {
        text
    }
}

/// Judge a whole run against the tables.
///
/// A run over no root, or one in which no case was compared, is
/// unavailable, never validated: a comparison of nothing agrees with
/// everything.
#[must_use]
pub fn validate(run: &Run, table: &KnownDifferences) -> Validation {
    let mut diffs = Vec::new();
    let mut summary = Summary::default();
    for root in &run.roots {
        summary.roots += 1;
        match &root.oracle {
            OracleSide::Generated { .. } => summary.generated += 1,
            OracleSide::Refused { .. } => summary.refused += 1,
        }
        if let LexLeanSide::Eligible(accepted) = &root.lexlean {
            if matches!(root.oracle, OracleSide::Generated { .. }) {
                summary.cases += accepted.cases.len();
            }
        }
        diffs.extend(differences(root, run.executed.get(&root.root)));
    }
    if summary.roots == 0 || summary.generated == 0 || summary.cases == 0 {
        return Validation::Unavailable(Unavailable::Vacuous(format!(
            "{} roots, {} generated by the oracle, {} cases compared",
            summary.roots, summary.generated, summary.cases
        )));
    }
    let judgement = judge(&diffs, table);
    if judgement.unlisted.is_empty() && judgement.stale.is_empty() {
        summary.known_differences = judgement.accepted;
        Validation::Validated(summary)
    } else {
        Validation::Disagreement(judgement.unlisted, judgement.stale)
    }
}

/// The whole run, once per process: the oracle acquired, every corpus
/// staged and asked, and every crate the oracle generated built and run.
pub fn corpus_run() -> &'static Result<Run, Unavailable> {
    static RUN: OnceLock<Result<Run, Unavailable>> = OnceLock::new();
    RUN.get_or_init(|| {
        let start = std::time::Instant::now();
        let oracle = oracle::acquire()?;
        let acquired = start.elapsed().as_secs_f64();
        let roots = survey(oracle)?;
        let surveyed = start.elapsed().as_secs_f64();
        let slots: Vec<Slot<'_>> = roots
            .iter()
            .map(|run| Slot {
                key: &run.root,
                run,
            })
            .collect();
        let executed = execute(&slots, true)?;
        let mut timings = BTreeMap::new();
        timings.insert("acquire".to_owned(), acquired);
        timings.insert("survey".to_owned(), surveyed - acquired);
        timings.insert(
            "execute".to_owned(),
            start.elapsed().as_secs_f64() - surveyed,
        );
        Ok(Run {
            roots,
            executed,
            pin: oracle.pin().clone(),
            timings,
        })
    })
}

/// What a gate does with the oracle's run.
#[derive(Debug)]
pub enum Gate<'a> {
    /// The oracle ran: the case validates against it.
    Run(&'a Run),
    /// The oracle did not run on a host that does not require it: the case
    /// says it did not validate.
    NotValidated(String),
    /// The oracle did not run on the normative host: the case fails.
    Fail(String),
}

/// Decide what the oracle's run means to a gate on a host.
#[must_use]
pub fn decide(normative: bool, run: &Result<Run, Unavailable>) -> Gate<'_> {
    match run {
        Ok(run) => Gate::Run(run),
        Err(reason) if normative => Gate::Fail(format!(
            "the oracle is unavailable ({reason}); that is not a validation, and on the normative host it fails the gate (§17.19)"
        )),
        Err(reason) => Gate::NotValidated(reason.to_string()),
    }
}

/// The run, when this host can have one.
///
/// On the normative host an oracle that cannot be used fails the case: the
/// case would otherwise pass having validated nothing. On any other host the
/// case says it did not validate and runs only its host-independent
/// assertions.
///
/// # Panics
///
/// Panics on the normative host when the oracle is unavailable.
#[must_use]
pub fn gate(id: &str) -> Option<&'static Run> {
    if !support::lean_backed(id) {
        eprintln!("{id}: NOT VALIDATED: this host has no pinned Lean toolchain, so the oracle did not run (§8.3)");
        return None;
    }
    match decide(support::normative_host(), corpus_run()) {
        Gate::Run(run) => Some(run),
        Gate::Fail(reason) => panic!("{id}: {reason}"),
        Gate::NotValidated(reason) => {
            eprintln!("{id}: NOT VALIDATED: {reason}");
            None
        }
    }
}

// ---- planted discrepancies and other renderings ---------------------------------

/// `run` with the source of the oracle's crate replaced by `edit` of it, and
/// the contract read again from the edited source.
///
/// # Errors
///
/// A reason when the root has no oracle crate, when the edit does not apply
/// (a plant that changes nothing plants nothing), or when the edited source
/// has no signature for the root.
pub fn planted(run: &RootRun, edit: &dyn Fn(&str) -> Option<String>) -> Result<RootRun, String> {
    let OracleSide::Generated { ir, package, .. } = &run.oracle else {
        return Err(format!("{} has no oracle crate", run.root));
    };
    let source = String::from_utf8_lossy(&package["src/lib.rs"]).into_owned();
    let edited = edit(&source).ok_or_else(|| format!("the edit does not apply to {}", run.root))?;
    if edited == source {
        return Err(format!("the edit changes nothing in {}", run.root));
    }
    let parsed = read_ir(ir)?;
    let (contract, signature) = oracle_contract(&parsed, run.function(), &edited)?;
    let mut package = package.clone();
    package.insert("src/lib.rs".to_owned(), edited.into_bytes());
    Ok(RootRun {
        oracle: OracleSide::Generated {
            ir: ir.clone(),
            package,
            contract,
            signature,
        },
        ..run.clone()
    })
}

/// The differences of each of `runs`, built and run together: a run whose
/// oracle crate no longer states the contract of its IR or of LexLean is not
/// run, since its harness would not be calling what the root's contract says.
#[must_use]
pub fn evaluate(runs: &[RootRun]) -> Vec<Vec<Diff>> {
    let keys: Vec<String> = runs
        .iter()
        .enumerate()
        .map(|(at, run)| format!("{at}:{}", run.root))
        .collect();
    let runnable: Vec<Slot<'_>> = runs
        .iter()
        .zip(&keys)
        .map(|(run, key)| Slot { key, run })
        .filter(|slot| match (&slot.run.lexlean, &slot.run.oracle) {
            (LexLeanSide::Eligible(accepted), OracleSide::Generated { contract, .. }) => {
                contract.drift.is_empty()
                    && contract.parameters == accepted.contract.parameters
                    && contract.result == accepted.contract.result
            }
            _ => false,
        })
        .collect();
    let executed = execute(&runnable, false)
        .unwrap_or_else(|reason| panic!("the planted runs cannot be executed: {reason}"));
    runs.iter()
        .zip(&keys)
        .map(|(run, key)| differences(run, executed.get(key)))
        .collect()
}

/// What LexLean's rendering of a root, or another rendering of it, did on the
/// root's inputs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rendered {
    /// The printed outcome of each input.
    pub outcomes: Vec<Json>,
    /// The work the rendering's own runtime counted over all of them. This is
    /// the rendering's statement about itself: a rendering that never counts
    /// reports none, so it bounds nothing by itself (see `allocations`).
    pub work: u64,
    /// The allocations a counting allocator, which the harness installs and the
    /// rendering cannot reach, saw over the whole run. This is the measure
    /// that does not depend on what the rendering says about itself.
    pub allocations: u64,
}

/// Build `files` as a crate exporting `root(..)` with the interface of
/// LexLean's package of `accepted`, run it on the root's inputs, and return
/// what it printed.
///
/// # Errors
///
/// The build error when the crate does not provide the interface: a changed
/// width or result type is refused here, by rustc, before any outcome.
///
/// # Panics
///
/// Panics when the harness cannot be written, which is a defect of this
/// module.
pub fn render_candidate(
    accepted: &Accepted,
    profile: &str,
    files: &BTreeMap<String, Vec<u8>>,
) -> Result<Rendered, String> {
    use crate::rust_build::{workspace, Built};
    let program = &accepted.program;
    let passing = vec![Passing::Own; program.functions[0].types.len()];
    let arguments: Vec<Vec<Value>> = accepted
        .cases
        .iter()
        .map(|case| case.values.clone())
        .collect();
    let calls = [crate::rust_harness::Calls {
        entry: 0,
        function: "root",
        passing: &passing,
        arguments: &arguments,
    }];
    let harness = crate::rust_harness::render_calls(
        program,
        Profile::named(profile).expect("a profile"),
        "candidate",
        &calls,
    )
    .expect("the harness is written");
    // The harness is LexLean's own, and the allocator it gets is the one
    // module of it that needs `unsafe`; the candidate crate is another crate
    // and cannot reach either.
    let counted = harness.replacen("#![forbid(unsafe_code)]", "#![deny(unsafe_code)]", 1);
    assert_ne!(counted, harness, "the harness forbids unsafe code");
    let harness = format!(
        "{}\n{COUNTING}\n{}\nfn main() {{\n    harness_main();\n    println!(\"allocations {{}}\", allocations());\n}}\n",
        counted.split_once("\nfn main() {").map_or("", |(head, _)| head),
        "fn harness_main() {\n    let runner = std::thread::Builder::new().stack_size(64 << 20).spawn(harness_runs).expect(\"the harness thread starts\");\n    if runner.join().is_err() {\n        std::process::exit(101);\n    }\n}"
    );
    let dir = tempfile::Builder::new()
        .prefix("lexlean-oracle-candidate-")
        .tempdir()
        .map_err(|error| error.to_string())?;
    workspace(
        dir.path(),
        &[Built {
            name: "candidate".to_owned(),
            files: files.clone(),
            harness,
        }],
    );
    let build = cargo_bounded(
        dir.path(),
        &["build", "--offline", "--workspace"],
        BUILD_LIMIT,
    )
    .map_err(|reason| format!("the candidate's build: {reason}"))?;
    if build.status != Some(0) {
        return Err(build_errors(&build.stderr, "p/candidate"));
    }
    let ran = oracle::run(
        std::process::Command::new(
            dir.path()
                .join("target/debug")
                .join(format!("run_candidate{}", std::env::consts::EXE_SUFFIX)),
        ),
        RUN_LIMIT,
    )
    .map_err(|reason| {
        format!(
            "the candidate did not finish: {reason} (the bound is {} s)",
            RUN_LIMIT.as_secs()
        )
    })?;
    if ran.status != Some(0) {
        return Err(format!("the candidate ended the run with {:?}", ran.status));
    }
    let mut outcomes = Vec::new();
    let mut work = 0;
    let mut allocations = 0;
    for line in ran.stdout.lines() {
        let number = |count: &str| {
            count
                .parse()
                .map_err(|e: std::num::ParseIntError| e.to_string())
        };
        if let Some(count) = line.strip_prefix("work ") {
            work = number(count)?;
        } else if let Some(count) = line.strip_prefix("allocations ") {
            allocations = number(count)?;
        } else {
            outcomes.push(serde_json::from_str(line).map_err(|e| e.to_string())?);
        }
    }
    Ok(Rendered {
        outcomes,
        work,
        allocations,
    })
}

/// LexLean's own package of a root: its `Cargo.toml`, `src/lib.rs`, and
/// provenance, for the profile named.
///
/// # Panics
///
/// Panics when the renderer refuses a root it rendered before.
#[must_use]
pub fn lexlean_package(accepted: &Accepted, profile: &str) -> BTreeMap<String, Vec<u8>> {
    let manifest = Manifest {
        spec: MANIFEST_SPEC.to_owned(),
        name: "candidate".to_owned(),
        version: "1.0.0".to_owned(),
        profile: profile.to_owned(),
        program: accepted.program.clone(),
        exports: vec![Export {
            function: 0,
            name: "root".to_owned(),
            parameters: vec![Passing::Own; accepted.program.functions[0].types.len()],
            errors: if accepted.contract.failure {
                Errors::Overflow
            } else {
                Errors::None
            },
        }],
        sources: Vec::new(),
    };
    package(&manifest)
        .unwrap_or_else(|reason| panic!("LexLean renders the root: {reason}"))
        .files
}

/// Whether a rendering of a root is accepted: it prints, on every input, the
/// outcome the interpreter gives, and does no more work than the baseline.
///
/// # Errors
///
/// The first way it is not: it does not build against the interface, an
/// outcome differs, or it counts more work than `baseline`.
pub fn accept_rendering(
    accepted: &Accepted,
    rendering: &Rendered,
    baseline: &Rendered,
) -> Result<(), String> {
    if rendering.outcomes.len() != accepted.cases.len() {
        return Err(format!(
            "{} outcomes for {} inputs",
            rendering.outcomes.len(),
            accepted.cases.len()
        ));
    }
    for (case, got) in accepted.cases.iter().zip(&rendering.outcomes) {
        let expected = lexlean::calculus::rust::observable(&case.outcome).expect("observable");
        if *got != expected {
            return Err(format!(
                "on {:?} the rendering prints {got} where the interpreter has {expected}",
                case.arguments
            ));
        }
    }
    if rendering.work > baseline.work {
        return Err(format!(
            "the rendering counts {} units of work, more than the {} of LexLean's own",
            rendering.work, baseline.work
        ));
    }
    if rendering.allocations > baseline.allocations {
        return Err(format!(
            "the rendering allocates {} times, more than the {} of LexLean's own",
            rendering.allocations, baseline.allocations
        ));
    }
    Ok(())
}

// ---- host-independent material ---------------------------------------------------

/// The registry constructs each eligible root of `project` realizes, by
/// root, as LexLean's production eligibility states them: no Lean and no
/// oracle.
#[must_use]
pub fn lexlean_constructs(project: &P) -> BTreeMap<String, BTreeSet<String>> {
    let checked = support::checked_project(project);
    lexlean::production::lower::roots(&checked)
        .expect("the eligibility reports")
        .into_iter()
        .map(|root| {
            (
                root.report.root.clone(),
                root.report.constructs.keys().cloned().collect(),
            )
        })
        .collect()
}

/// A root made of a committed target fixture, for the cases that exercise the
/// comparison without the oracle: LexLean's side as the fixture states it, and
/// an oracle side whose contract is LexLean's own and whose crate is `source`.
///
/// # Panics
///
/// Panics when the fixture is not one with its entry at function 0.
#[must_use]
pub fn fixture_root(fixture: &str, source: &str) -> RootRun {
    let path = repo_root().join(format!("compiler/fixtures/{fixture}.json"));
    let fixture: lexlean::calculus::Fixture =
        serde_json::from_slice(&std::fs::read(path.as_std_path()).expect("fixture"))
            .expect("a fixture");
    assert_eq!(fixture.entry, 0, "a fixture whose entry is function 0");
    let function = fixture.program.functions[0].clone();
    let contract = Contract {
        parameters: function
            .types
            .iter()
            .map(|ty| lexlean_shape(&fixture.program, ty, &mut Vec::new()))
            .collect(),
        result: lexlean_shape(&fixture.program, &function.result, &mut Vec::new()),
        failure: lexlean::calculus::rust::fallible_functions(&fixture.program)
            .expect("a valid program")[0],
        borrowed: Vec::new(),
        output_buffer: false,
        drift: Vec::new(),
    };
    let case = Case {
        root: "Fixture.root".to_owned(),
        arguments: Vec::new(),
        values: fixture.arguments.clone(),
        outcome: fixture.expected.clone(),
        function: 0,
        invalid: None,
    };
    let mut package = BTreeMap::new();
    package.insert("src/lib.rs".to_owned(), source.as_bytes().to_vec());
    package.insert(
        "Cargo.toml".to_owned(),
        b"[package]\nname = \"x\"\n".to_vec(),
    );
    RootRun {
        corpus: "fixtures".to_owned(),
        root: "Fixture.root".to_owned(),
        lean_module: "Fixture".to_owned(),
        lexlean: LexLeanSide::Eligible(Box::new(Accepted {
            targets: vec!["rust-core".to_owned(), "rust-std".to_owned()],
            effects: BTreeSet::new(),
            constructs: BTreeSet::new(),
            program: fixture.program,
            contract: contract.clone(),
            cases: vec![case],
        })),
        oracle: OracleSide::Generated {
            ir: String::new(),
            package,
            contract,
            signature: (Vec::new(), String::new()),
        },
    }
}

/// The acquired oracle, when this host can have one: the gate of the cases
/// that need the oracle but not the whole corpus run.
///
/// # Panics
///
/// Panics on the normative host when the oracle is unavailable.
#[must_use]
pub fn oracle_gate(id: &str) -> Option<&'static Oracle> {
    if !support::lean_backed(id) {
        eprintln!("{id}: NOT VALIDATED: this host has no pinned Lean toolchain, so the oracle did not run (§8.3)");
        return None;
    }
    match oracle::acquire() {
        Ok(oracle) => Some(oracle),
        Err(reason) => {
            assert!(
                !support::normative_host(),
                "{id}: the oracle is unavailable ({reason}); that is not a validation, and on the normative host it fails the gate (§17.19)"
            );
            eprintln!("{id}: NOT VALIDATED: {reason}");
            None
        }
    }
}

impl Validation {
    /// Write the evidence of a disagreement to `dir/oracle-evidence.json`:
    /// every difference with LexLean's and the oracle's evidence, and every
    /// stale row. A validated or unavailable run records nothing.
    ///
    /// # Errors
    ///
    /// The I/O error that kept the record from being written.
    pub fn record(&self, dir: &Path) -> std::io::Result<Option<PathBuf>> {
        let Self::Disagreement(unlisted, stale) = self else {
            return Ok(None);
        };
        std::fs::create_dir_all(dir)?;
        let path = dir.join("oracle-evidence.json");
        let text = serde_json::to_string_pretty(&serde_json::json!({
            "spec": "lexlean/oracle-evidence/1",
            "role": oracle::ROLE,
            "disagreements": unlisted,
            "stale": stale.iter().map(|(row, root)| serde_json::json!({"row": row, "root": root})).collect::<Vec<_>>(),
        }))
        .map_err(std::io::Error::other)?;
        std::fs::write(&path, text)?;
        Ok(Some(path))
    }
}

/// Require a run to be validated: a disagreement is recorded (where
/// `LEXLEAN_ORACLE_EVIDENCE` points, else in the oracle's cache) and fails
/// with both sides' evidence; anything else that is not validated fails.
///
/// # Panics
///
/// Panics unless `validation` is validated.
pub fn require(id: &str, validation: &Validation) {
    if validation.validated() {
        return;
    }
    let directory = std::env::var_os("LEXLEAN_ORACLE_EVIDENCE").map_or_else(
        || std::env::temp_dir().join("lexlean-oracle-evidence"),
        PathBuf::from,
    );
    let recorded = validation
        .record(&directory)
        .ok()
        .flatten()
        .map_or_else(String::new, |path| {
            format!("\nrecorded in {}", path.display())
        });
    panic!("{id}: {}{recorded}", validation.report());
}
