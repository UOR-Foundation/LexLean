//! The production realization calculus (SPEC.md §17.14): the closed target
//! of production compilation.
//!
//! The calculus is defined by the LexLean modules `compiler/src/TargetSyntax`
//! and `compiler/src/TargetSemantics`, which Lean elaborates and the kernel
//! checks like any other LexLean program; their evaluator is the denotation.
//! This module is the host side of that definition: the closed JSON form of
//! a target program (`lexlean/target-program/1`), its static validity rules,
//! its canonical bytes and content identity, a reference interpreter that
//! mirrors the denotation step for step (so the kernel can confirm every
//! fixture's outcome against it), the emission of a program as a LexLean
//! term, and the reference rendering to Rust. Nothing here defines meaning:
//! where this module and the LexLean evaluator disagree, the evaluator wins,
//! and the fixture theorems make every disagreement a failed verification.

pub mod check;
pub mod interp;
pub mod library;
pub mod realization;
pub mod rust;
pub mod rust_library;
pub mod term;

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::artifact::content_id::Sha256Digest;
use crate::code;
use crate::diagnostic::Diagnostic;
use crate::error::LexLeanError;

fn rejected(reason: &str) -> LexLeanError {
    LexLeanError::from_diagnostic(Diagnostic::new(
        code!("LLB6005"),
        format!("target program: {reason}"),
    ))
}

/// Read a target program, check every static rule, and return its canonical
/// form. Nothing about an invalid program is guessed or repaired: every
/// failure is `LLB6005`.
///
/// # Errors
///
/// `LLB6005` naming the first malformed member or violated rule.
pub fn load(bytes: &[u8]) -> Result<Program, LexLeanError> {
    let program = Program::parse(bytes).map_err(|reason| rejected(&reason))?;
    program.canonical().map_err(|reason| rejected(&reason))
}

/// Render a valid program to a Rust library crate of `profile`.
///
/// # Errors
///
/// `LLB6005` when the program is invalid or has no faithful rendering in
/// `profile`.
pub fn render(program: &Program, profile: rust::Profile) -> Result<String, LexLeanError> {
    rust::render(program, profile).map_err(|reason| rejected(&reason))
}

/// Render a package of a target program: the Rust library crate of its
/// profile with the exported functions its manifest
/// (`lexlean/rust-package/1`) declares, the Cargo manifest, and the
/// provenance (`lexlean/rust-provenance/1`), by relative path.
///
/// # Errors
///
/// `LLB6005` when the manifest is malformed, its program invalid, its
/// interface misstates the program, or its profile cannot render it.
pub fn package(bytes: &[u8]) -> Result<rust::package::Package, LexLeanError> {
    let manifest = rust::package::Manifest::parse(bytes).map_err(|reason| rejected(&reason))?;
    rust::package::package(&manifest).map_err(|reason| rejected(&reason))
}

/// The schema tag of a target program.
pub const PROGRAM_SPEC: &str = "lexlean/target-program/1";

/// A fixed-width integer kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IntKind {
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
}

impl IntKind {
    /// Every kind in declaration order.
    pub const ALL: [Self; 8] = [
        Self::U8,
        Self::U16,
        Self::U32,
        Self::U64,
        Self::I8,
        Self::I16,
        Self::I32,
        Self::I64,
    ];

    /// The bit width.
    #[must_use]
    pub const fn bits(self) -> u32 {
        match self {
            Self::U8 | Self::I8 => 8,
            Self::U16 | Self::I16 => 16,
            Self::U32 | Self::I32 => 32,
            Self::U64 | Self::I64 => 64,
        }
    }

    /// Whether the kind is signed.
    #[must_use]
    pub const fn signed(self) -> bool {
        matches!(self, Self::I8 | Self::I16 | Self::I32 | Self::I64)
    }

    /// The inclusive range.
    #[must_use]
    pub const fn range(self) -> (i128, i128) {
        match self {
            Self::U8 => (0, u8::MAX as i128),
            Self::U16 => (0, u16::MAX as i128),
            Self::U32 => (0, u32::MAX as i128),
            Self::U64 => (0, u64::MAX as i128),
            Self::I8 => (i8::MIN as i128, i8::MAX as i128),
            Self::I16 => (i16::MIN as i128, i16::MAX as i128),
            Self::I32 => (i32::MIN as i128, i32::MAX as i128),
            Self::I64 => (i64::MIN as i128, i64::MAX as i128),
        }
    }

    /// The spelling shared by the JSON form, the LexLean term, and Rust.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::U8 => "u8",
            Self::U16 => "u16",
            Self::U32 => "u32",
            Self::U64 => "u64",
            Self::I8 => "i8",
            Self::I16 => "i16",
            Self::I32 => "i32",
            Self::I64 => "i64",
        }
    }
}

/// A target type.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum Ty {
    Unit,
    Bool,
    Nat,
    Int,
    Fixed {
        width: IntKind,
    },
    String,
    Bytes,
    Ordering,
    Option {
        value: Box<Ty>,
    },
    Result {
        ok: Box<Ty>,
        error: Box<Ty>,
    },
    List {
        element: Box<Ty>,
    },
    Pair {
        left: Box<Ty>,
        right: Box<Ty>,
    },
    Adt {
        index: u64,
    },
    Fn {
        parameters: Vec<Ty>,
        result: Box<Ty>,
    },
}

/// An ordering value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrderingValue {
    Lt,
    Eq,
    Gt,
}

/// A target value. Integers are decimal strings, so the JSON form is exact.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum Value {
    Unit,
    Bool {
        value: bool,
    },
    Nat {
        value: String,
    },
    Int {
        value: String,
    },
    U8 {
        value: String,
    },
    U16 {
        value: String,
    },
    U32 {
        value: String,
    },
    U64 {
        value: String,
    },
    I8 {
        value: String,
    },
    I16 {
        value: String,
    },
    I32 {
        value: String,
    },
    I64 {
        value: String,
    },
    String {
        value: String,
    },
    Bytes {
        hex: String,
    },
    Ordering {
        value: OrderingValue,
    },
    None,
    Some {
        value: Box<Value>,
    },
    Ok {
        value: Box<Value>,
    },
    Error {
        value: Box<Value>,
    },
    List {
        items: Vec<Value>,
    },
    Pair {
        left: Box<Value>,
        right: Box<Value>,
    },
    Adt {
        constructor: u64,
        fields: Vec<Value>,
    },
    Closure {
        function: u64,
        captures: Vec<Value>,
    },
}

/// A pattern shape, also used to construct values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum Shape {
    None,
    Some,
    Ok,
    Error,
    Nil,
    Cons,
    Zero,
    Succ,
    Pair,
    True,
    False,
    Unit,
    Lt,
    Eq,
    Gt,
    Adt { constructor: u64 },
}

/// A primitive operation.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum Prim {
    NatAdd,
    NatSub,
    NatMul,
    NatQuot,
    NatRem,
    NatEq,
    NatLe,
    NatLt,
    IntAdd,
    IntSub,
    IntMul,
    IntNeg,
    IntQuot,
    IntRem,
    CheckedAdd,
    CheckedSub,
    CheckedMul,
    CheckedNeg,
    CheckedQuot,
    BitAnd,
    BitOr,
    BitXor,
    BitNot,
    ShiftLeft,
    ShiftRight,
    Equal,
    BoolNot,
    BoolAnd,
    BoolOr,
    Append,
    Length,
    Index,
    Slice,
    Utf8Encode,
    Utf8Decode,
    CompareBytes,
    SplitExact,
    Join,
    FormatDecimal,
    Compare,
    Convert { target: IntKind },
    ParseDecimal { target: Ty },
}

/// A match arm.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Arm {
    pub shape: Shape,
    pub binders: Vec<u64>,
    pub body: Expr,
}

/// A target expression.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum Expr {
    Value {
        #[serde(rename = "type")]
        ty: Ty,
        value: Value,
    },
    Var {
        name: u64,
    },
    Let {
        name: u64,
        #[serde(rename = "type")]
        ty: Ty,
        bound: Box<Expr>,
        body: Box<Expr>,
    },
    Cond {
        condition: Box<Expr>,
        then_branch: Box<Expr>,
        else_branch: Box<Expr>,
    },
    Match {
        #[serde(rename = "type")]
        ty: Ty,
        scrutinee: Box<Expr>,
        arms: Vec<Arm>,
    },
    Build {
        shape: Shape,
        #[serde(rename = "type")]
        ty: Ty,
        operands: Vec<Expr>,
    },
    Call {
        function: u64,
        operands: Vec<Expr>,
    },
    Closure {
        function: u64,
        captures: Vec<Expr>,
    },
    Apply {
        target: Box<Expr>,
        operands: Vec<Expr>,
    },
    Prim {
        operation: Prim,
        operands: Vec<Expr>,
    },
    First {
        value: Box<Expr>,
    },
    Second {
        value: Box<Expr>,
    },
    Field {
        value: Box<Expr>,
        index: u64,
    },
}

/// An algebraic data type: its constructors' field types.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Adt {
    pub constructors: Vec<Vec<Ty>>,
}

/// A function.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Function {
    pub parameters: Vec<u64>,
    pub types: Vec<Ty>,
    pub result: Ty,
    pub body: Expr,
}

/// A target program.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Program {
    pub spec: String,
    pub adts: Vec<Adt>,
    pub functions: Vec<Function>,
}

impl Program {
    /// Read a program from JSON.
    ///
    /// # Errors
    ///
    /// Returns the reason the bytes are not a `lexlean/target-program/1`
    /// value.
    pub fn parse(bytes: &[u8]) -> Result<Self, String> {
        let program: Self = serde_json::from_slice(bytes)
            .map_err(|error| format!("target program is malformed: {error}"))?;
        if program.spec != PROGRAM_SPEC {
            return Err(format!(
                "target program has spec `{}`, expected `{PROGRAM_SPEC}`",
                program.spec
            ));
        }
        Ok(program)
    }

    /// The canonical form of a valid program: every function's locals
    /// renamed to their first-binding order (parameters first, then binders
    /// in evaluation order), so alpha-equivalent programs are byte-identical.
    /// An invalid program has no canonical form.
    ///
    /// # Errors
    ///
    /// Returns the first violated static rule.
    pub fn canonical(&self) -> Result<Self, String> {
        check::check(self)?;
        let mut out = self.clone();
        for (index, function) in out.functions.iter_mut().enumerate() {
            let mut renamer = Renamer::default();
            let mut scope = Vec::new();
            for parameter in &mut function.parameters {
                *parameter = renamer.bind(*parameter, &mut scope);
            }
            renamer
                .expr(&mut function.body, &mut scope)
                .map_err(|reason| format!("function {index}: {reason}"))?;
        }
        Ok(out)
    }

    /// The canonical file bytes.
    ///
    /// # Panics
    ///
    /// Panics only if `serde_json` produces text the canonical JSON parser
    /// rejects, which would be an internal invariant failure.
    #[must_use]
    pub fn to_file_bytes(&self) -> Vec<u8> {
        let text = serde_json::to_string(self).expect("program serializes");
        crate::artifact::canonical_json::Json::parse(text.as_bytes())
            .expect("program is JSON")
            .to_file_bytes()
    }

    /// The content identity of the canonical form; an invalid program has
    /// none.
    ///
    /// # Errors
    ///
    /// Returns the first violated static rule.
    pub fn id(&self) -> Result<Sha256Digest, String> {
        Ok(Sha256Digest::of(&self.canonical()?.to_file_bytes()))
    }
}

/// Lexically scoped renaming to first-binding order.
#[derive(Default)]
struct Renamer {
    next: u64,
}

impl Renamer {
    fn bind(&mut self, name: u64, scope: &mut Vec<(u64, u64)>) -> u64 {
        let fresh = self.next;
        self.next += 1;
        scope.push((name, fresh));
        fresh
    }

    fn lookup(scope: &[(u64, u64)], name: u64) -> Result<u64, String> {
        scope
            .iter()
            .rev()
            .find(|(source, _)| *source == name)
            .map(|(_, renamed)| *renamed)
            .ok_or_else(|| format!("local {name} is unbound"))
    }

    fn exprs(&mut self, exprs: &mut [Expr], scope: &mut Vec<(u64, u64)>) -> Result<(), String> {
        exprs.iter_mut().try_for_each(|expr| self.expr(expr, scope))
    }

    fn expr(&mut self, expr: &mut Expr, scope: &mut Vec<(u64, u64)>) -> Result<(), String> {
        match expr {
            Expr::Value { .. } => Ok(()),
            Expr::Var { name } => {
                *name = Self::lookup(scope, *name)?;
                Ok(())
            }
            Expr::Let {
                name, bound, body, ..
            } => {
                self.expr(bound, scope)?;
                let depth = scope.len();
                *name = self.bind(*name, scope);
                self.expr(body, scope)?;
                scope.truncate(depth);
                Ok(())
            }
            Expr::Cond {
                condition,
                then_branch,
                else_branch,
            } => {
                self.expr(condition, scope)?;
                self.expr(then_branch, scope)?;
                self.expr(else_branch, scope)
            }
            Expr::Match {
                scrutinee, arms, ..
            } => {
                self.expr(scrutinee, scope)?;
                for arm in arms {
                    let depth = scope.len();
                    for binder in &mut arm.binders {
                        *binder = self.bind(*binder, scope);
                    }
                    self.expr(&mut arm.body, scope)?;
                    scope.truncate(depth);
                }
                Ok(())
            }
            Expr::Build { operands, .. }
            | Expr::Call { operands, .. }
            | Expr::Prim { operands, .. } => self.exprs(operands, scope),
            Expr::Closure { captures, .. } => self.exprs(captures, scope),
            Expr::Apply { target, operands } => {
                self.expr(target, scope)?;
                self.exprs(operands, scope)
            }
            Expr::First { value } | Expr::Second { value } | Expr::Field { value, .. } => {
                self.expr(value, scope)
            }
        }
    }
}

/// The outcome of running a program: the denotation's observable result.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum Outcome {
    Value { value: Value, steps: u64 },
    Overflow { steps: u64 },
    Stuck,
    Exhausted,
}

/// A hand-constructed program with an input and its expected outcome.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Fixture {
    pub spec: String,
    pub name: String,
    pub program: Program,
    pub entry: u64,
    pub arguments: Vec<Value>,
    pub fuel: u64,
    pub expected: Outcome,
}

/// The schema tag of a fixture.
pub const FIXTURE_SPEC: &str = "lexlean/target-fixture/1";

impl Fixture {
    /// The canonical file bytes.
    ///
    /// # Panics
    ///
    /// Panics only if `serde_json` produces text the canonical JSON parser
    /// rejects, which would be an internal invariant failure.
    #[must_use]
    pub fn to_file_bytes(&self) -> Vec<u8> {
        let text = serde_json::to_string(self).expect("fixture serializes");
        crate::artifact::canonical_json::Json::parse(text.as_bytes())
            .expect("fixture is JSON")
            .to_file_bytes()
    }
}

/// The fixtures under a directory, sorted by name.
///
/// # Errors
///
/// Returns the first unreadable or malformed fixture.
pub fn fixtures(dir: &std::path::Path) -> Result<BTreeMap<String, Fixture>, String> {
    let mut out = BTreeMap::new();
    let entries = std::fs::read_dir(dir).map_err(|error| format!("{}: {error}", dir.display()))?;
    for entry in entries {
        let path = entry.map_err(|error| error.to_string())?.path();
        if path.extension().is_none_or(|extension| extension != "json") {
            continue;
        }
        let bytes = std::fs::read(&path).map_err(|error| format!("{}: {error}", path.display()))?;
        let fixture: Fixture = serde_json::from_slice(&bytes)
            .map_err(|error| format!("{}: {error}", path.display()))?;
        if fixture.spec != FIXTURE_SPEC {
            return Err(format!("{}: spec `{}`", path.display(), fixture.spec));
        }
        out.insert(fixture.name.clone(), fixture);
    }
    Ok(out)
}
