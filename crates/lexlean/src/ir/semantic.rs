//! Language-1.1 closed declaration, term, and proof data.
//!
//! This is a source-language IR, not Lean syntax.  Every variant has one
//! fixed Lean lowering and one fixed LaTeX rendering.  The validator is
//! deliberately conservative: unresolved or ambiguous data is rejected
//! before either backend runs.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

pub mod model;
pub mod reasoning;

/// A qualified document member.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MemberRef {
    /// Imported logical module, absent for the current module.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module: Option<String>,
    /// Declaration/member name.
    pub name: String,
}

/// The closed portable integer families admitted by language 1.1.
///
/// `Int` is mathematical and unbounded.  The remaining representations are
/// distinct fixed-width values; their literals are range-checked before a
/// backend is invoked.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SemanticInteger {
    Int,
    Int8,
    Int16,
    Int32,
    Int64,
    #[serde(rename = "uint8")]
    UInt8,
    #[serde(rename = "uint16")]
    UInt16,
    #[serde(rename = "uint32")]
    UInt32,
    #[serde(rename = "uint64")]
    UInt64,
}

/// Closed, backend-independent primitive operations for portable data.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SemanticPrimitive {
    Subtract,
    Multiply,
    Quotient,
    Remainder,
    Negate,
    CheckedConvert,
    CheckedAdd,
    CheckedSubtract,
    CheckedMultiply,
    CheckedNegate,
    CheckedQuotient,
    BitAnd,
    BitOr,
    BitXor,
    BitNot,
    ShiftLeft,
    ShiftRight,
    Append,
    Length,
    Index,
    Slice,
    Utf8Encode,
    Utf8Decode,
    CompareBytes,
    Equal,
    SplitExact,
    Join,
    ParseDecimal,
    FormatDecimal,
    /// Language 1.2 ordered collections, folds, bounded iteration, and
    /// graphs (§17.12).
    MapInsert,
    MapRemove,
    MapLookup,
    MapContains,
    MapSize,
    MapKeys,
    MapValues,
    MapEntries,
    MapFold,
    SetInsert,
    SetRemove,
    SetContains,
    SetSize,
    SetElements,
    SetUnion,
    SetIntersection,
    SetDifference,
    SetFold,
    ListFold,
    Iterate,
    IterateUntil,
    GraphSuccessors,
    GraphReachable,
    GraphTopological,
    /// Language 1.2: the strict order of an ordered key type, as a Boolean.
    LessThan,
}

impl SemanticPrimitive {
    /// Whether the operation exists only in language 1.2.
    #[must_use]
    pub const fn language_1_2(self) -> bool {
        matches!(
            self,
            Self::MapInsert
                | Self::MapRemove
                | Self::MapLookup
                | Self::MapContains
                | Self::MapSize
                | Self::MapKeys
                | Self::MapValues
                | Self::MapEntries
                | Self::MapFold
                | Self::SetInsert
                | Self::SetRemove
                | Self::SetContains
                | Self::SetSize
                | Self::SetElements
                | Self::SetUnion
                | Self::SetIntersection
                | Self::SetDifference
                | Self::SetFold
                | Self::ListFold
                | Self::Iterate
                | Self::IterateUntil
                | Self::GraphSuccessors
                | Self::GraphReachable
                | Self::GraphTopological
                | Self::LessThan
        )
    }

    /// The argument position holding a function, for the collection
    /// combinators that take one.
    #[must_use]
    pub const fn function_argument(self) -> Option<usize> {
        match self {
            Self::MapFold | Self::SetFold | Self::ListFold | Self::Iterate | Self::IterateUntil => {
                Some(0)
            }
            _ => None,
        }
    }
}

impl SemanticInteger {
    fn semantic_type(self) -> SemanticType {
        match self {
            Self::Int => SemanticType::Int,
            Self::Int8 => SemanticType::Int8,
            Self::Int16 => SemanticType::Int16,
            Self::Int32 => SemanticType::Int32,
            Self::Int64 => SemanticType::Int64,
            Self::UInt8 => SemanticType::UInt8,
            Self::UInt16 => SemanticType::UInt16,
            Self::UInt32 => SemanticType::UInt32,
            Self::UInt64 => SemanticType::UInt64,
        }
    }

    fn bounds(self) -> Option<(i128, i128)> {
        Some(match self {
            Self::Int => return None,
            Self::Int8 => (i8::MIN.into(), i8::MAX.into()),
            Self::Int16 => (i16::MIN.into(), i16::MAX.into()),
            Self::Int32 => (i32::MIN.into(), i32::MAX.into()),
            Self::Int64 => (i64::MIN.into(), i64::MAX.into()),
            Self::UInt8 => (0, u8::MAX.into()),
            Self::UInt16 => (0, u16::MAX.into()),
            Self::UInt32 => (0, u32::MAX.into()),
            Self::UInt64 => (0, u64::MAX.into()),
        })
    }
}

fn canonical_integer(value: &str) -> bool {
    if value == "0" {
        return true;
    }
    let digits = value.strip_prefix('-').unwrap_or(value);
    !digits.is_empty()
        && !digits.starts_with('0')
        && digits.bytes().all(|byte| byte.is_ascii_digit())
}

fn check_integer_literal(representation: SemanticInteger, value: &str) -> Result<(), String> {
    if !canonical_integer(value) {
        return Err(format!("noncanonical integer literal `{value}`"));
    }
    if matches!(representation, SemanticInteger::Int) {
        return Ok(());
    }
    let parsed = value
        .parse::<i128>()
        .map_err(|_| format!("integer literal `{value}` is outside the portable parser range"))?;
    let (minimum, maximum) = representation.bounds().expect("fixed integer bounds");
    if (minimum..=maximum).contains(&parsed) {
        Ok(())
    } else {
        Err(format!(
            "integer literal `{value}` is outside {representation:?} [{minimum}, {maximum}]"
        ))
    }
}

/// Closed language-1.1 types.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
pub enum SemanticType {
    /// The first universe of data types.
    Type,
    /// A declaration-bound type parameter.
    Parameter { name: String },
    /// Natural numbers.
    Nat,
    /// Booleans.
    Bool,
    /// Propositions.
    Prop,
    /// Unit.
    Unit,
    /// A mathematical, unbounded integer.
    Int,
    /// A signed eight-bit integer.
    Int8,
    /// A signed sixteen-bit integer.
    Int16,
    /// A signed thirty-two-bit integer.
    Int32,
    /// A signed sixty-four-bit integer.
    Int64,
    /// An unsigned eight-bit integer.
    #[serde(rename = "uint8")]
    UInt8,
    /// An unsigned sixteen-bit integer.
    #[serde(rename = "uint16")]
    UInt16,
    /// An unsigned thirty-two-bit integer.
    #[serde(rename = "uint32")]
    UInt32,
    /// An unsigned sixty-four-bit integer.
    #[serde(rename = "uint64")]
    UInt64,
    /// A Unicode scalar string whose serialized value is valid UTF-8.
    String,
    /// An immutable byte sequence.
    Bytes,
    /// The three-way lexicographic order result.
    Ordering,
    /// A closed optional value.
    Option { value: Box<Self> },
    /// A closed success/error value.
    Result { ok: Box<Self>, error: Box<Self> },
    /// A list.
    List { element: Box<Self> },
    /// A document-defined type.
    Named {
        member: MemberRef,
        arguments: Vec<Self>,
    },
    /// Language 1.2: the binary product of two closed types.
    Product { left: Box<Self>, right: Box<Self> },
    /// Language 1.2: a total function of one or more ordered arguments.
    Function {
        parameters: Vec<Self>,
        result: Box<Self>,
    },
    /// Language 1.2: a finite map over an ordered key type, iterated in
    /// ascending key order.
    Map { key: Box<Self>, value: Box<Self> },
    /// Language 1.2: a finite set over an ordered element type, iterated in
    /// ascending order.
    Set { element: Box<Self> },
    /// Language 1.2 (models): the refusal of a checked model application,
    /// naming the contract predicate that failed.
    ContractViolation,
    /// Language 1.2 (reasoning): why a reasoner returned no verified answer.
    ReasoningFailure,
}

/// One explicit declaration parameter.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticParameter {
    pub name: String,
    pub r#type: SemanticType,
}

/// One ordered structure or class field.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticField {
    pub name: String,
    pub r#type: SemanticType,
}

/// One positional inductive constructor.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticConstructor {
    pub name: String,
    pub fields: Vec<SemanticType>,
}

/// One ordered record/instance assignment.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticAssignment {
    pub field: String,
    pub value: SemanticTerm,
}

/// One exhaustive match branch.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticBranch {
    pub constructor: MemberRef,
    pub binders: Vec<String>,
    pub body: SemanticTerm,
}

/// Closed language-1.1 terms.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
pub enum SemanticTerm {
    Var {
        name: String,
    },
    Nat {
        value: String,
    },
    /// A canonical mathematical or fixed-width integer literal.
    Integer {
        representation: SemanticInteger,
        value: String,
    },
    /// A Unicode string literal. Serde guarantees valid UTF-8.
    String {
        value: String,
    },
    /// A byte sequence represented by canonical lowercase hexadecimal.
    Bytes {
        hex: String,
    },
    /// A closed portable primitive with explicit arguments and result type.
    Primitive {
        operation: SemanticPrimitive,
        arguments: Vec<Self>,
        result: SemanticType,
    },
    Bool {
        value: bool,
    },
    Unit,
    Nil {
        element: SemanticType,
    },
    Cons {
        head: Box<Self>,
        tail: Box<Self>,
    },
    Record {
        r#type: MemberRef,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        type_arguments: Vec<SemanticType>,
        fields: Vec<SemanticAssignment>,
    },
    Constructor {
        constructor: MemberRef,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        type_arguments: Vec<SemanticType>,
        arguments: Vec<Self>,
    },
    /// A deterministically resolved explicit document instance value.
    InstanceValue {
        class: MemberRef,
        arguments: Vec<SemanticType>,
        resolved: MemberRef,
    },
    Project {
        value: Box<Self>,
        field: String,
    },
    Call {
        function: MemberRef,
        /// Language 1.2: explicit type arguments of a generic definition.
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        type_arguments: Vec<SemanticType>,
        arguments: Vec<Self>,
    },
    If {
        condition: Box<Self>,
        then_value: Box<Self>,
        else_value: Box<Self>,
    },
    Match {
        scrutinee: Box<Self>,
        branches: Vec<SemanticBranch>,
    },
    Eq {
        left: Box<Self>,
        right: Box<Self>,
    },
    Le {
        left: Box<Self>,
        right: Box<Self>,
    },
    Lt {
        left: Box<Self>,
        right: Box<Self>,
    },
    Add {
        left: Box<Self>,
        right: Box<Self>,
    },
    Beq {
        left: Box<Self>,
        right: Box<Self>,
    },
    Ble {
        left: Box<Self>,
        right: Box<Self>,
    },
    Blt {
        left: Box<Self>,
        right: Box<Self>,
    },
    And {
        left: Box<Self>,
        right: Box<Self>,
    },
    /// Propositional conjunction, distinct from Boolean conjunction.
    PropAnd {
        left: Box<Self>,
        right: Box<Self>,
    },
    Or {
        left: Box<Self>,
        right: Box<Self>,
    },
    Not {
        value: Box<Self>,
    },
    Implies {
        premise: Box<Self>,
        conclusion: Box<Self>,
    },
    Iff {
        left: Box<Self>,
        right: Box<Self>,
    },
    Forall {
        binder: SemanticParameter,
        body: Box<Self>,
    },
    /// Language 1.2: a typed, nonrecursive local definition. The binder is
    /// in scope only in `body`; it never shadows another local.
    Let {
        binder: SemanticParameter,
        value: Box<Self>,
        body: Box<Self>,
    },
    /// Language 1.2: the pair of two values, of product type.
    Pair {
        left: Box<Self>,
        right: Box<Self>,
    },
    /// Language 1.2: the first component of a pair.
    First {
        value: Box<Self>,
    },
    /// Language 1.2: the second component of a pair.
    Second {
        value: Box<Self>,
    },
    /// Language 1.2: an anonymous function. `captures` lists, sorted and
    /// unique, exactly the enclosing locals the body uses.
    Lambda {
        parameters: Vec<SemanticParameter>,
        captures: Vec<String>,
        body: Box<Self>,
    },
    /// Language 1.2: full application of a function-typed value.
    Apply {
        function: Box<Self>,
        arguments: Vec<Self>,
    },
    /// Language 1.2: a document definition used as a function value.
    FunctionRef {
        function: MemberRef,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        type_arguments: Vec<SemanticType>,
    },
    /// Language 1.2: a finite map of literal keys; linking sorts the entries
    /// into canonical key order and rejects a duplicate key.
    MapLiteral {
        key: SemanticType,
        value: SemanticType,
        entries: Vec<SemanticMapEntry>,
    },
    /// Language 1.2: a finite set of literal elements, canonicalized like a
    /// map literal.
    SetLiteral {
        element: SemanticType,
        elements: Vec<Self>,
    },
    /// Language 1.2: a directed graph `Map node (Set node)` of declared
    /// literal nodes and edges between them.
    GraphLiteral {
        node: SemanticType,
        nodes: Vec<Self>,
        edges: Vec<SemanticEdge>,
    },
    /// Language 1.2 (models): a model applied through the listed runtime
    /// checks of its contract, returning the result or the first refusal.
    CheckedApply {
        model: MemberRef,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        type_arguments: Vec<SemanticType>,
        arguments: Vec<Self>,
        checks: Vec<ModelCheck>,
    },
}

/// One literal map entry.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticMapEntry {
    pub key: SemanticTerm,
    pub value: SemanticTerm,
}

/// One literal directed edge.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticEdge {
    pub source: SemanticTerm,
    pub target: SemanticTerm,
}

/// One proof branch for a fixed cases/induction lowering.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticProofBranch {
    pub constructor: String,
    pub binders: Vec<String>,
    pub proof: Box<SemanticProof>,
}

/// A fixed Boolean comparison reflected into a proposition without using
/// propositional extensionality.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SemanticReflectionComparison {
    NatBeq,
    NatBlt,
}

/// One Nat-valued record projection and its canonical expected literal.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticReflectionField {
    pub field: String,
    pub expected: String,
}

/// Closed, axiom-free Boolean reflection proof shapes.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
pub enum SemanticReflection {
    /// Structural reflection for a Nat-indexed list validator/predicate pair.
    List {
        parameter: String,
        values: String,
        boolean_definition: MemberRef,
        proposition_definition: MemberRef,
        comparison: SemanticReflectionComparison,
    },
    /// Reflection for a finite right-associated conjunction of Nat record
    /// field comparisons.
    Record {
        record: String,
        boolean_definition: MemberRef,
        proposition_definition: MemberRef,
        fields: Vec<SemanticReflectionField>,
    },
}

/// Closed language-1.1 proof forms.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
pub enum SemanticProof {
    Reflexivity,
    Decide,
    /// Language 1.2: closed linear natural-number arithmetic. It introduces
    /// the goal's hypotheses, rewrites the fixed Boolean comparisons
    /// `Nat.beq`, `Nat.blt`, and `Nat.ble` into propositions, and runs the
    /// pinned toolchain's `omega` decision procedure. When it names document
    /// definitions, it first substitutes every hypothesis equating a local
    /// with a value and unfolds exactly those definitions, so a measure over
    /// constructors becomes linear.
    LinearArithmetic {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        definitions: Vec<MemberRef>,
    },
    Simplify {
        definitions: Vec<MemberRef>,
    },
    Constructor {
        branches: Vec<Self>,
    },
    Cases {
        scrutinee: String,
        branches: Vec<SemanticProofBranch>,
    },
    Induction {
        scrutinee: String,
        /// Other theorem parameters generalized before structural induction.
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        generalizing: Vec<String>,
        branches: Vec<SemanticProofBranch>,
    },
    Congruence,
    /// A fixed natural-deduction bridge from executable Boolean validation to
    /// an independent proposition. The backend never invokes `propext`.
    BooleanReflection {
        reflection: SemanticReflection,
    },
    /// Exact application of one prior theorem to checked semantic arguments.
    Apply {
        theorem: MemberRef,
        /// Language 1.2: explicit type arguments of a generic theorem.
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        type_arguments: Vec<SemanticType>,
        arguments: Vec<SemanticTerm>,
    },
}

/// Language-1.2 well-founded recursion evidence (§17.12): a natural-number
/// measure over the parameters and, for each recursive call site in
/// pre-order, a prior theorem whose statement is exactly that call's
/// decrease obligation.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticTermination {
    pub measure: SemanticTerm,
    pub evidence: Vec<MemberRef>,
}

/// Language-1.2 production-root declaration (§17.13): the registered targets
/// the definition must be eligible for, both sorted and unique, and the
/// registered effects its realization may have.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticProduction {
    pub targets: Vec<String>,
    pub effects: Vec<String>,
}

/// Language 1.2 (§17.12, models): the closed role an artifact plays. The
/// bytes may be opaque; their role never is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactRole {
    /// Learned or fitted parameters: an integer tensor.
    Parameters,
    /// A vocabulary: UTF-8 lines.
    Vocabulary,
    /// Examples or measurements: any closed schema.
    Dataset,
    /// A lookup table: an integer tensor.
    Table,
    /// Uninterpreted bytes.
    Binary,
}

/// Language 1.2 (models): the element encoding of an integer tensor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TensorElement {
    Int8,
    Int16,
    Int32,
    Int64,
    #[serde(rename = "uint8")]
    UInt8,
    #[serde(rename = "uint16")]
    UInt16,
    #[serde(rename = "uint32")]
    UInt32,
    #[serde(rename = "uint64")]
    UInt64,
}

/// Language 1.2 (models): how an artifact's bytes decode to its value.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
pub enum ArtifactSchema {
    /// The bytes themselves, as `bytes`.
    Bytes,
    /// Little-endian, row-major integers of one encoding, as nested lists
    /// of mathematical integers, one list level per dimension.
    IntTensor {
        element: TensorElement,
        shape: Vec<u64>,
    },
    /// Valid UTF-8 without carriage returns, every line LF-terminated, as
    /// the list of lines without their terminators.
    Utf8Lines,
}

/// Language 1.2 (models): one interface binder of a contract or a
/// realization.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ModelBinder {
    pub name: String,
    pub r#type: SemanticType,
}

/// Language 1.2 (models): a contract's state interface: the state before
/// a step, the state after it, and their type.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ContractState {
    pub name: String,
    pub next: String,
    pub r#type: SemanticType,
}

/// Language 1.2 (models): the predicates a contract may name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ContractPredicate {
    Invariant,
    Postcondition,
    Precondition,
}

/// Language 1.2 (models): a runtime validator of one contract predicate,
/// linked to it by statement-exact soundness and optional completeness
/// theorems.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ContractValidator {
    pub predicate: ContractPredicate,
    pub validator: MemberRef,
    pub sound: MemberRef,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub complete: Option<MemberRef>,
}

/// Language 1.2 (models): a contract, realization, model, evidence, or
/// function instantiated at explicit type arguments.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ModelUse {
    pub member: MemberRef,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub type_arguments: Vec<SemanticType>,
}

/// Language 1.2 (models): the state a stateful realization threads.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RealizationState {
    pub name: String,
    pub r#type: SemanticType,
    pub initial: SemanticTerm,
}

/// Language 1.2 (models): one guarded rule, tried in declared order.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RealizationRule {
    pub name: String,
    pub guard: SemanticTerm,
    pub action: SemanticTerm,
}

/// Language 1.2 (models): the closed statistical scoring schemes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StatisticalScheme {
    /// Integer linear scores over an integer feature vector, the first
    /// maximal label winning.
    LinearScoring,
}

/// Language 1.2 (models): the closed neural architectures.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum NeuralArchitecture {
    /// Exact integer feed-forward layers over mathematical integers.
    IntegerFeedforward,
}

/// Language 1.2 (models): one exact integer layer.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
pub enum NeuralLayer {
    /// `W v + b` over mathematical integers, `W` of shape `[outputs,
    /// inputs]` and `b` of shape `[outputs]`.
    Dense {
        inputs: u64,
        outputs: u64,
        weights: MemberRef,
        bias: MemberRef,
    },
    /// Negative values become zero.
    Relu,
    /// Truncating division by `2^shift`, then clamping to the closed range.
    Requantize {
        shift: u64,
        minimum: String,
        maximum: String,
    },
}

/// Language 1.2 (models): how a network's final values become its output.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
pub enum NeuralDecoder {
    /// The label paired with the first maximal value.
    Argmax { labels: Vec<SemanticTerm> },
    /// An ordinary term over the final values, bound to `binder`.
    Function { binder: String, body: SemanticTerm },
}

/// Language 1.2 (models): how one composite stage is guarded at run time.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
pub enum CompositeJunction {
    /// The stage has nothing left to establish.
    Unconditional,
    /// A prior theorem states exactly the generated junction obligation.
    Proved { evidence: MemberRef },
    /// The stage's validators run first; a refusal is returned.
    Checked,
}

/// Language 1.2 (models): the closed composition forms.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
pub enum CompositeForm {
    /// Each stage consumes the previous stage's output; one junction per
    /// stage after the first.
    Sequence {
        stages: Vec<ModelUse>,
        junctions: Vec<CompositeJunction>,
    },
    /// Both stages consume the input; the output is their pair.
    Fanout { left: ModelUse, right: ModelUse },
    /// Each stage consumes its own component of a pair input.
    Product { left: ModelUse, right: ModelUse },
    /// A Boolean guard over the input selects one stage.
    Branch {
        guard: SemanticTerm,
        then: ModelUse,
        r#else: ModelUse,
    },
    /// A stateful stage folded over a list from its initial state.
    Scan {
        stage: ModelUse,
        junction: CompositeJunction,
    },
}

/// Language 1.2 (models): the closed realization descriptors. Each
/// elaborates in linking to exactly one ordinary denotation.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
pub enum RealizationDescriptor {
    /// An ordinary term over the interface binders.
    Deterministic { body: SemanticTerm },
    /// The action of the first rule whose guard holds, else the default.
    Rule {
        rules: Vec<RealizationRule>,
        default: SemanticTerm,
    },
    /// Integer linear scores of an integer feature vector.
    Statistical {
        scheme: StatisticalScheme,
        features: SemanticTerm,
        width: u64,
        width_evidence: MemberRef,
        weights: MemberRef,
        bias: MemberRef,
        labels: Vec<SemanticTerm>,
    },
    /// Exact integer layers over an integer encoding of the input.
    Neural {
        architecture: NeuralArchitecture,
        encoder: SemanticTerm,
        width: u64,
        width_evidence: MemberRef,
        layers: Vec<NeuralLayer>,
        decoder: NeuralDecoder,
    },
    /// A typed composition of prior models.
    Composite { form: CompositeForm },
}

/// Language 1.2 (models): one closed evidence claim, discharged by a prior
/// theorem stating exactly the generated obligation.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
pub enum EvidenceClaim {
    /// Agreement on a dataset under a comparison sound for equality.
    DatasetAgreement {
        dataset: MemberRef,
        comparison: MemberRef,
        comparison_sound: MemberRef,
        examples: u64,
        agreements: u64,
        theorem: MemberRef,
    },
    /// Equality with a reference function under the contract's premises.
    EquivalentTo {
        reference: ModelUse,
        theorem: MemberRef,
    },
    /// The initial state satisfies the invariant.
    InitialInvariant { theorem: MemberRef },
    /// A step preserves the invariant.
    PreservesInvariant { theorem: MemberRef },
    /// The realization meets the postcondition under the premises.
    SatisfiesContract { theorem: MemberRef },
}

/// Language 1.2 (models): the runtime checks of a checked application, in
/// strictly sorted order of their names.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ModelCheck {
    InputInvariant,
    OutputInvariant,
    Postcondition,
    Precondition,
}

/// Language 1.2 (reasoning): a logic's invariant and the theorem that its
/// relation preserves it.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LogicInvariant {
    pub predicate: MemberRef,
    pub preserves: MemberRef,
}

/// Language 1.2 (reasoning): a rule's bound variable, ranging over the
/// candidates its term lists for the current state, in list order.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RuleBinding {
    pub name: String,
    pub r#type: SemanticType,
    pub candidates: SemanticTerm,
}

/// Language 1.2 (reasoning): the candidate answer of a reasoner's state, an
/// optional value of the answer type over the state binder.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ReasoningAnswer {
    pub name: String,
    pub r#type: SemanticType,
    pub value: SemanticTerm,
}

/// Language 1.2 (reasoning): the closed search orders.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SearchOrder {
    /// Successors join the back of the frontier.
    BreadthFirst,
    /// Successors join the front of the frontier.
    DepthFirst,
}

/// Language 1.2 (reasoning): the closed strategies. Every bound is a
/// natural-number term over the observation; a missing bound is refused in
/// linking (`LLT4011`), not by the schema, so that an unbounded strategy is
/// named as such.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
pub enum ReasoningStrategy {
    /// Fire the first applicable rule until none applies or the fuel is
    /// spent.
    Forward {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        fuel: Option<SemanticTerm>,
    },
    /// Expand a bounded frontier of states in the given order until a
    /// verified answer is found, the frontier empties, or the fuel is spent.
    Search {
        order: SearchOrder,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        fuel: Option<SemanticTerm>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        frontier: Option<SemanticTerm>,
        #[serde(default, skip_serializing_if = "core::ops::Not::not")]
        deduplicate: bool,
    },
    /// Draw candidate answers from a generator, in order, and return the
    /// first one the verifier's check accepts, checking at most the budget.
    /// The candidates carry no evidence: this is the strategy whose
    /// generator may be anything, a model's output among them, because
    /// nothing it proposes is used unverified.
    GenerateAndVerify {
        generator: SemanticTerm,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        budget: Option<SemanticTerm>,
    },
}

/// Language 1.2 (reasoning): the closed reasoner claims, each discharged by
/// a statement-exact prior theorem.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
pub enum ReasoningClaim {
    /// The observed state satisfies the logic's invariant.
    InitialInvariant { theorem: MemberRef },
    /// The ranking of the observed state is below the fuel, so a forward
    /// reasoner whose every rule decreases the ranking saturates.
    Terminates { theorem: MemberRef },
    /// A predicate relating the observation to every state a run reaches:
    /// it holds of the observed state, and is preserved by the logic's
    /// relation, so it holds of the final state of every run. Under it an
    /// answer may be claimed correct on the states it holds of, whatever the
    /// specification says about the observation.
    ObservationInvariant {
        predicate: MemberRef,
        initial: MemberRef,
        preserved: MemberRef,
    },
    /// The answer term is correct on every state: whatever it extracts meets
    /// the verifier's specification, so the verifier's check is erased and
    /// the unverified answer may be used (the reasoning analogue of an
    /// evidence claim discharging a model's runtime check).
    AnswerCorrect { theorem: MemberRef },
}

/// A closed declaration.
// Declarations are parsed once and held in a module's ordered list, never
// moved in bulk, so the size of the definition variant costs nothing.
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
pub enum SemanticDeclaration {
    Structure {
        name: String,
        type_parameters: Vec<String>,
        parameters: Vec<SemanticParameter>,
        fields: Vec<SemanticField>,
    },
    Class {
        name: String,
        type_parameters: Vec<String>,
        parameters: Vec<SemanticParameter>,
        fields: Vec<SemanticField>,
    },
    Instance {
        name: String,
        class: MemberRef,
        arguments: Vec<SemanticType>,
        priority: u64,
        fields: Vec<SemanticAssignment>,
    },
    Inductive {
        name: String,
        type_parameters: Vec<String>,
        parameters: Vec<SemanticParameter>,
        constructors: Vec<SemanticConstructor>,
        /// Language 1.2: the label of the contiguous mutual group this
        /// inductive belongs to. Absent for a standalone inductive.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        mutual: Option<String>,
    },
    Definition {
        name: String,
        /// Language 1.2: explicit type parameters, instantiated at every use.
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        type_parameters: Vec<String>,
        parameters: Vec<SemanticParameter>,
        result: SemanticType,
        #[serde(skip_serializing_if = "Option::is_none")]
        recursive_argument: Option<String>,
        body: SemanticTerm,
        /// Exact, sorted axiom set admitted by this computational definition.
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        axioms: Vec<String>,
        /// Language 1.2: the definition asserts production eligibility; it
        /// is rejected unless every closure it forms is non-escaping.
        #[serde(default, skip_serializing_if = "core::ops::Not::not")]
        executable: bool,
        /// Language 1.2: the label of the contiguous group of mutually
        /// structurally recursive definitions this definition belongs to.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        mutual: Option<String>,
        /// Language 1.2: well-founded recursion with a measure and one
        /// evidence theorem per recursive call site.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        termination: Option<SemanticTermination>,
        /// Language 1.2: the definition is a production root for the listed
        /// targets, admitting exactly the listed effects (§17.13).
        #[serde(default, skip_serializing_if = "Option::is_none")]
        production: Option<SemanticProduction>,
    },
    Theorem {
        name: String,
        /// Language 1.2: explicit type parameters.
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        type_parameters: Vec<String>,
        parameters: Vec<SemanticParameter>,
        statement: SemanticTerm,
        proof: SemanticProof,
        /// Exact, sorted axiom set. Omission means the empty policy.
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        axioms: Vec<String>,
    },
    /// Language 1.2 (models): a content-addressed external artifact with a
    /// declared role, closed schema, and value type.
    Artifact {
        name: String,
        role: ArtifactRole,
        sha256: String,
        length: u64,
        schema: ArtifactSchema,
        r#type: SemanticType,
        /// Exact, sorted axiom set of its generated declarations.
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        axioms: Vec<String>,
    },
    /// Language 1.2 (models): a specification: typed interfaces, prior
    /// predicates, and linked runtime validators.
    Contract {
        name: String,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        type_parameters: Vec<String>,
        input: ModelBinder,
        output: ModelBinder,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        state: Option<ContractState>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        precondition: Option<MemberRef>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        postcondition: Option<MemberRef>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        invariant: Option<MemberRef>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        validators: Vec<ContractValidator>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        axioms: Vec<String>,
    },
    /// Language 1.2 (models): an implementation whose closed descriptor
    /// elaborates to one ordinary denotation.
    Realization {
        name: String,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        type_parameters: Vec<String>,
        input: ModelBinder,
        output: SemanticType,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        state: Option<RealizationState>,
        descriptor: RealizationDescriptor,
        #[serde(default, skip_serializing_if = "core::ops::Not::not")]
        executable: bool,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        axioms: Vec<String>,
    },
    /// Language 1.2 (models): claims about one realization against one
    /// contract, each discharged by a statement-exact prior theorem.
    Evidence {
        name: String,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        type_parameters: Vec<String>,
        contract: ModelUse,
        realization: ModelUse,
        claims: Vec<EvidenceClaim>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        axioms: Vec<String>,
    },
    /// Language 1.2 (models): a contract bound to a realization of the
    /// identical interface, with its evidence and entry obligations.
    Model {
        name: String,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        type_parameters: Vec<String>,
        contract: ModelUse,
        realization: ModelUse,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        evidence: Vec<ModelUse>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        entry: Vec<MemberRef>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        axioms: Vec<String>,
    },
    /// Language 1.2 (reasoning): a state type with a relation between
    /// states, and optionally an invariant the relation preserves and a
    /// natural-number ranking.
    Logic {
        name: String,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        type_parameters: Vec<String>,
        state: ContractState,
        relation: MemberRef,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        invariant: Option<LogicInvariant>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        ranking: Option<MemberRef>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        axioms: Vec<String>,
    },
    /// Language 1.2 (reasoning): a guarded transition of a logic's state,
    /// sound for its relation.
    InferenceRule {
        name: String,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        type_parameters: Vec<String>,
        logic: ModelUse,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        binding: Option<RuleBinding>,
        guard: SemanticTerm,
        conclusion: SemanticTerm,
        soundness: MemberRef,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        progress: Option<MemberRef>,
        #[serde(default, skip_serializing_if = "core::ops::Not::not")]
        executable: bool,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        axioms: Vec<String>,
    },
    /// Language 1.2 (reasoning): an executable check of a candidate answer
    /// for a subject, sound (and optionally complete) for a specification.
    Verifier {
        name: String,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        type_parameters: Vec<String>,
        subject: ModelBinder,
        candidate: ModelBinder,
        specification: MemberRef,
        check: MemberRef,
        sound: MemberRef,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        complete: Option<MemberRef>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        axioms: Vec<String>,
    },
    /// Language 1.2 (reasoning): a bounded engine that observes an input,
    /// applies rules of one logic under a closed strategy, and returns an
    /// answer its verifier accepts, with the trace that derives it.
    Reasoner {
        name: String,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        type_parameters: Vec<String>,
        /// The logic, observed state, and answer of a rule-based reasoner;
        /// a generate-and-verify reasoner states none of them.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        logic: Option<ModelUse>,
        observation: ModelBinder,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        observe: Option<SemanticTerm>,
        rules: Vec<ModelUse>,
        strategy: ReasoningStrategy,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        answer: Option<ReasoningAnswer>,
        verifier: ModelUse,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        claims: Vec<ReasoningClaim>,
        #[serde(default, skip_serializing_if = "core::ops::Not::not")]
        executable: bool,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        axioms: Vec<String>,
    },
}

impl SemanticDeclaration {
    #[must_use]
    pub fn name(&self) -> &str {
        match self {
            Self::Structure { name, .. }
            | Self::Class { name, .. }
            | Self::Instance { name, .. }
            | Self::Inductive { name, .. }
            | Self::Definition { name, .. }
            | Self::Theorem { name, .. }
            | Self::Artifact { name, .. }
            | Self::Contract { name, .. }
            | Self::Realization { name, .. }
            | Self::Evidence { name, .. }
            | Self::Model { name, .. }
            | Self::Logic { name, .. }
            | Self::InferenceRule { name, .. }
            | Self::Verifier { name, .. }
            | Self::Reasoner { name, .. } => name,
        }
    }

    #[must_use]
    pub const fn kind(&self) -> &'static str {
        match self {
            Self::Structure { .. } => "structure",
            Self::Class { .. } => "class",
            Self::Instance { .. } => "instance",
            Self::Inductive { .. } => "inductive",
            Self::Definition { .. } => "definition",
            Self::Theorem { .. } => "theorem",
            Self::Artifact { .. } => "artifact",
            Self::Contract { .. } => "contract",
            Self::Realization { .. } => "realization",
            Self::Evidence { .. } => "evidence",
            Self::Model { .. } => "model",
            Self::Logic { .. } => "logic",
            Self::InferenceRule { .. } => "inference_rule",
            Self::Verifier { .. } => "verifier",
            Self::Reasoner { .. } => "reasoner",
        }
    }

    #[must_use]
    pub fn axioms(&self) -> &[String] {
        match self {
            Self::Definition { axioms, .. }
            | Self::Theorem { axioms, .. }
            | Self::Artifact { axioms, .. }
            | Self::Contract { axioms, .. }
            | Self::Realization { axioms, .. }
            | Self::Evidence { axioms, .. }
            | Self::Model { axioms, .. }
            | Self::Logic { axioms, .. }
            | Self::InferenceRule { axioms, .. }
            | Self::Verifier { axioms, .. }
            | Self::Reasoner { axioms, .. } => axioms,
            _ => &[],
        }
    }

    #[must_use]
    pub fn axiom_policy_kind(&self) -> &'static str {
        if self.axioms().is_empty() {
            "none"
        } else {
            "exact"
        }
    }
}

/// Positional renaming of every local binder (§17.12 alpha identity).
struct AlphaRenamer {
    next: usize,
    scopes: Vec<(String, String)>,
    types: BTreeMap<String, String>,
}

impl AlphaRenamer {
    fn bind(&mut self, name: &str) -> String {
        let fresh = format!("_{}", self.next);
        self.next += 1;
        self.scopes.push((name.to_owned(), fresh.clone()));
        fresh
    }

    fn resolve(&self, name: &str) -> String {
        self.scopes
            .iter()
            .rev()
            .find(|(source, _)| source == name)
            .map_or_else(|| name.to_owned(), |(_, fresh)| fresh.clone())
    }

    fn ty(&self, ty: &SemanticType) -> SemanticType {
        substitute_type(
            ty,
            &self
                .types
                .iter()
                .map(|(source, fresh)| {
                    (
                        source.clone(),
                        SemanticType::Parameter {
                            name: fresh.clone(),
                        },
                    )
                })
                .collect(),
        )
    }

    fn term(&mut self, term: &SemanticTerm) -> SemanticTerm {
        let mark = self.scopes.len();
        let out = match term {
            SemanticTerm::Var { name } => SemanticTerm::Var {
                name: self.resolve(name),
            },
            SemanticTerm::Forall { binder, body } => {
                let r#type = self.ty(&binder.r#type);
                let mark = self.scopes.len();
                let name = self.bind(&binder.name);
                let body = Box::new(self.term(body));
                self.scopes.truncate(mark);
                SemanticTerm::Forall {
                    binder: SemanticParameter { name, r#type },
                    body,
                }
            }
            SemanticTerm::Let {
                binder,
                value,
                body,
            } => {
                let value = self.term(value);
                let r#type = self.ty(&binder.r#type);
                let mark = self.scopes.len();
                let name = self.bind(&binder.name);
                let body = Box::new(self.term(body));
                self.scopes.truncate(mark);
                SemanticTerm::Let {
                    binder: SemanticParameter { name, r#type },
                    value: Box::new(value),
                    body,
                }
            }
            SemanticTerm::Lambda {
                parameters,
                captures,
                body,
            } => {
                // Captures are a sorted set of names, so they are sorted
                // again after renaming: alpha-equivalent sources whose
                // original names sort differently must agree.
                let mut captures: Vec<String> =
                    captures.iter().map(|name| self.resolve(name)).collect();
                captures.sort();
                let mark = self.scopes.len();
                let parameters = parameters
                    .iter()
                    .map(|parameter| {
                        let r#type = self.ty(&parameter.r#type);
                        SemanticParameter {
                            name: self.bind(&parameter.name),
                            r#type,
                        }
                    })
                    .collect();
                let body = Box::new(self.term(body));
                self.scopes.truncate(mark);
                SemanticTerm::Lambda {
                    parameters,
                    captures,
                    body,
                }
            }
            SemanticTerm::Match {
                scrutinee,
                branches,
            } => SemanticTerm::Match {
                scrutinee: Box::new(self.term(scrutinee)),
                branches: branches
                    .iter()
                    .map(|branch| {
                        let mark = self.scopes.len();
                        let binders = branch.binders.iter().map(|name| self.bind(name)).collect();
                        let body = self.term(&branch.body);
                        self.scopes.truncate(mark);
                        SemanticBranch {
                            constructor: branch.constructor.clone(),
                            binders,
                            body,
                        }
                    })
                    .collect(),
            },
            // Nodes that bind nothing rename their subterms in evaluation
            // order, which fixes the positional numbering of any binder
            // nested inside them independently of how the node serializes.
            SemanticTerm::Nat { .. }
            | SemanticTerm::Integer { .. }
            | SemanticTerm::String { .. }
            | SemanticTerm::Bytes { .. }
            | SemanticTerm::Bool { .. }
            | SemanticTerm::Unit => term.clone(),
            SemanticTerm::Primitive {
                operation,
                arguments,
                result,
            } => SemanticTerm::Primitive {
                operation: *operation,
                arguments: self.terms(arguments),
                result: self.ty(result),
            },
            SemanticTerm::Nil { element } => SemanticTerm::Nil {
                element: self.ty(element),
            },
            SemanticTerm::Cons { head, tail } => {
                let head = Box::new(self.term(head));
                SemanticTerm::Cons {
                    head,
                    tail: Box::new(self.term(tail)),
                }
            }
            SemanticTerm::Record {
                r#type,
                type_arguments,
                fields,
            } => SemanticTerm::Record {
                r#type: r#type.clone(),
                type_arguments: self.types(type_arguments),
                fields: fields
                    .iter()
                    .map(|field| SemanticAssignment {
                        field: field.field.clone(),
                        value: self.term(&field.value),
                    })
                    .collect(),
            },
            SemanticTerm::Constructor {
                constructor,
                type_arguments,
                arguments,
            } => SemanticTerm::Constructor {
                constructor: constructor.clone(),
                type_arguments: self.types(type_arguments),
                arguments: self.terms(arguments),
            },
            SemanticTerm::InstanceValue {
                class,
                arguments,
                resolved,
            } => SemanticTerm::InstanceValue {
                class: class.clone(),
                arguments: self.types(arguments),
                resolved: resolved.clone(),
            },
            SemanticTerm::Project { value, field } => SemanticTerm::Project {
                value: Box::new(self.term(value)),
                field: field.clone(),
            },
            SemanticTerm::Call {
                function,
                type_arguments,
                arguments,
            } => SemanticTerm::Call {
                function: function.clone(),
                type_arguments: self.types(type_arguments),
                arguments: self.terms(arguments),
            },
            SemanticTerm::If {
                condition,
                then_value,
                else_value,
            } => {
                let condition = Box::new(self.term(condition));
                let then_value = Box::new(self.term(then_value));
                SemanticTerm::If {
                    condition,
                    then_value,
                    else_value: Box::new(self.term(else_value)),
                }
            }
            SemanticTerm::Eq { left, right } => {
                let (left, right) = self.pair(left, right);
                SemanticTerm::Eq { left, right }
            }
            SemanticTerm::Le { left, right } => {
                let (left, right) = self.pair(left, right);
                SemanticTerm::Le { left, right }
            }
            SemanticTerm::Lt { left, right } => {
                let (left, right) = self.pair(left, right);
                SemanticTerm::Lt { left, right }
            }
            SemanticTerm::Add { left, right } => {
                let (left, right) = self.pair(left, right);
                SemanticTerm::Add { left, right }
            }
            SemanticTerm::Beq { left, right } => {
                let (left, right) = self.pair(left, right);
                SemanticTerm::Beq { left, right }
            }
            SemanticTerm::Ble { left, right } => {
                let (left, right) = self.pair(left, right);
                SemanticTerm::Ble { left, right }
            }
            SemanticTerm::Blt { left, right } => {
                let (left, right) = self.pair(left, right);
                SemanticTerm::Blt { left, right }
            }
            SemanticTerm::And { left, right } => {
                let (left, right) = self.pair(left, right);
                SemanticTerm::And { left, right }
            }
            SemanticTerm::PropAnd { left, right } => {
                let (left, right) = self.pair(left, right);
                SemanticTerm::PropAnd { left, right }
            }
            SemanticTerm::Or { left, right } => {
                let (left, right) = self.pair(left, right);
                SemanticTerm::Or { left, right }
            }
            SemanticTerm::Iff { left, right } => {
                let (left, right) = self.pair(left, right);
                SemanticTerm::Iff { left, right }
            }
            SemanticTerm::Pair { left, right } => {
                let (left, right) = self.pair(left, right);
                SemanticTerm::Pair { left, right }
            }
            SemanticTerm::Implies {
                premise,
                conclusion,
            } => {
                let (premise, conclusion) = self.pair(premise, conclusion);
                SemanticTerm::Implies {
                    premise,
                    conclusion,
                }
            }
            SemanticTerm::Not { value } => SemanticTerm::Not {
                value: Box::new(self.term(value)),
            },
            SemanticTerm::First { value } => SemanticTerm::First {
                value: Box::new(self.term(value)),
            },
            SemanticTerm::Second { value } => SemanticTerm::Second {
                value: Box::new(self.term(value)),
            },
            SemanticTerm::Apply {
                function,
                arguments,
            } => {
                let function = Box::new(self.term(function));
                SemanticTerm::Apply {
                    function,
                    arguments: self.terms(arguments),
                }
            }
            SemanticTerm::FunctionRef {
                function,
                type_arguments,
            } => SemanticTerm::FunctionRef {
                function: function.clone(),
                type_arguments: self.types(type_arguments),
            },
            SemanticTerm::MapLiteral {
                key,
                value,
                entries,
            } => SemanticTerm::MapLiteral {
                key: self.ty(key),
                value: self.ty(value),
                entries: entries
                    .iter()
                    .map(|entry| {
                        let key = self.term(&entry.key);
                        SemanticMapEntry {
                            key,
                            value: self.term(&entry.value),
                        }
                    })
                    .collect(),
            },
            SemanticTerm::SetLiteral { element, elements } => SemanticTerm::SetLiteral {
                element: self.ty(element),
                elements: self.terms(elements),
            },
            SemanticTerm::GraphLiteral { node, nodes, edges } => SemanticTerm::GraphLiteral {
                node: self.ty(node),
                nodes: self.terms(nodes),
                edges: edges
                    .iter()
                    .map(|edge| {
                        let source = self.term(&edge.source);
                        SemanticEdge {
                            source,
                            target: self.term(&edge.target),
                        }
                    })
                    .collect(),
            },
            SemanticTerm::CheckedApply {
                model,
                type_arguments,
                arguments,
                checks,
            } => SemanticTerm::CheckedApply {
                model: model.clone(),
                type_arguments: self.types(type_arguments),
                arguments: self.terms(arguments),
                checks: checks.clone(),
            },
        };
        self.scopes.truncate(mark);
        out
    }

    fn terms(&mut self, terms: &[SemanticTerm]) -> Vec<SemanticTerm> {
        terms.iter().map(|term| self.term(term)).collect()
    }

    fn types(&self, types: &[SemanticType]) -> Vec<SemanticType> {
        types.iter().map(|ty| self.ty(ty)).collect()
    }

    fn pair(
        &mut self,
        left: &SemanticTerm,
        right: &SemanticTerm,
    ) -> (Box<SemanticTerm>, Box<SemanticTerm>) {
        let left = Box::new(self.term(left));
        (left, Box::new(self.term(right)))
    }
}

impl SemanticDeclaration {
    /// The alpha identity of a definition (§17.12): the canonical JSON of the
    /// definition with every type parameter, value parameter, and term
    /// binder renamed positionally in binding order. Alpha-equivalent
    /// definitions share it; any other difference changes it.
    #[must_use]
    pub fn alpha_identity(&self) -> Option<crate::artifact::content_id::Sha256Digest> {
        let Self::Definition {
            name,
            type_parameters,
            parameters,
            result,
            recursive_argument,
            body,
            axioms,
            executable,
            mutual,
            termination,
            production,
        } = self
        else {
            return None;
        };
        let mut renamer = AlphaRenamer {
            next: 0,
            scopes: Vec::new(),
            types: BTreeMap::new(),
        };
        let type_parameters = type_parameters
            .iter()
            .enumerate()
            .map(|(index, source)| {
                let fresh = format!("T{index}");
                renamer.types.insert(source.clone(), fresh.clone());
                fresh
            })
            .collect();
        let parameters: Vec<SemanticParameter> = parameters
            .iter()
            .map(|parameter| {
                let r#type = renamer.ty(&parameter.r#type);
                SemanticParameter {
                    name: renamer.bind(&parameter.name),
                    r#type,
                }
            })
            .collect();
        let normalized = Self::Definition {
            name: name.clone(),
            type_parameters,
            recursive_argument: recursive_argument
                .as_ref()
                .map(|argument| renamer.resolve(argument)),
            result: renamer.ty(result),
            body: renamer.term(body),
            termination: termination.as_ref().map(|termination| SemanticTermination {
                measure: renamer.term(&termination.measure),
                evidence: termination.evidence.clone(),
            }),
            parameters,
            axioms: axioms.clone(),
            executable: *executable,
            mutual: mutual.clone(),
            production: production.clone(),
        };
        let text = serde_json::to_string(&normalized).expect("definition serializes");
        let canonical = crate::artifact::canonical_json::Json::parse(text.as_bytes())
            .expect("serialized definition is JSON")
            .to_canonical_string();
        Some(crate::artifact::content_id::Sha256Digest::of(
            canonical.as_bytes(),
        ))
    }
}

/// A complete high-level semantic module.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticModule {
    pub spec: String,
    pub declarations: Vec<SemanticDeclaration>,
    /// Language 1.2 (models): what linking elaborated each declaration to.
    /// Never serialized: it is a function of the declarations, the linked
    /// imports, and the compiler semantics (§17.12, §21.4).
    #[serde(skip)]
    pub(crate) elaboration: model::Elaboration,
}

/// A linking failure with its registered diagnostic code (§26.3).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticFailure {
    /// The registered code.
    pub code: crate::diagnostic::DiagnosticCode,
    /// The reason, naming the construct.
    pub reason: String,
}

impl From<String> for SemanticFailure {
    fn from(reason: String) -> Self {
        Self {
            code: crate::code!("LLT4001"),
            reason,
        }
    }
}

impl std::fmt::Display for SemanticFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.reason)
    }
}

fn type_node_count(ty: &SemanticType) -> u64 {
    1 + match ty {
        SemanticType::Option { value } => type_node_count(value),
        SemanticType::Result { ok, error } => type_node_count(ok) + type_node_count(error),
        SemanticType::List { element } => type_node_count(element),
        SemanticType::Named { arguments, .. } => arguments.iter().map(type_node_count).sum(),
        SemanticType::Product { left, right } => type_node_count(left) + type_node_count(right),
        SemanticType::Function { parameters, result } => {
            parameters.iter().map(type_node_count).sum::<u64>() + type_node_count(result)
        }
        SemanticType::Map { key, value } => type_node_count(key) + type_node_count(value),
        SemanticType::Set { element } => type_node_count(element),
        _ => 0,
    }
}

fn term_node_count(term: &SemanticTerm) -> u64 {
    let terms = |values: &[SemanticTerm]| values.iter().map(term_node_count).sum::<u64>();
    let pair =
        |left: &SemanticTerm, right: &SemanticTerm| term_node_count(left) + term_node_count(right);
    1 + match term {
        SemanticTerm::Var { .. }
        | SemanticTerm::Nat { .. }
        | SemanticTerm::Integer { .. }
        | SemanticTerm::String { .. }
        | SemanticTerm::Bytes { .. }
        | SemanticTerm::Bool { .. }
        | SemanticTerm::Unit => 0,
        SemanticTerm::Primitive {
            arguments, result, ..
        } => terms(arguments) + type_node_count(result),
        SemanticTerm::Nil { element } => type_node_count(element),
        SemanticTerm::Cons { head, tail }
        | SemanticTerm::Eq {
            left: head,
            right: tail,
        }
        | SemanticTerm::Le {
            left: head,
            right: tail,
        }
        | SemanticTerm::Lt {
            left: head,
            right: tail,
        }
        | SemanticTerm::Add {
            left: head,
            right: tail,
        }
        | SemanticTerm::Beq {
            left: head,
            right: tail,
        }
        | SemanticTerm::Ble {
            left: head,
            right: tail,
        }
        | SemanticTerm::Blt {
            left: head,
            right: tail,
        }
        | SemanticTerm::And {
            left: head,
            right: tail,
        }
        | SemanticTerm::PropAnd {
            left: head,
            right: tail,
        }
        | SemanticTerm::Or {
            left: head,
            right: tail,
        }
        | SemanticTerm::Implies {
            premise: head,
            conclusion: tail,
        }
        | SemanticTerm::Iff {
            left: head,
            right: tail,
        }
        | SemanticTerm::Pair {
            left: head,
            right: tail,
        } => pair(head, tail),
        SemanticTerm::First { value } | SemanticTerm::Second { value } => term_node_count(value),
        SemanticTerm::Record {
            type_arguments,
            fields,
            ..
        } => {
            type_arguments.iter().map(type_node_count).sum::<u64>()
                + fields
                    .iter()
                    .map(|field| term_node_count(&field.value))
                    .sum::<u64>()
        }
        SemanticTerm::Constructor {
            type_arguments,
            arguments,
            ..
        } => type_arguments.iter().map(type_node_count).sum::<u64>() + terms(arguments),
        SemanticTerm::InstanceValue { arguments, .. } => {
            arguments.iter().map(type_node_count).sum()
        }
        SemanticTerm::Project { value, .. } | SemanticTerm::Not { value } => term_node_count(value),
        SemanticTerm::Call {
            type_arguments,
            arguments,
            ..
        } => type_arguments.iter().map(type_node_count).sum::<u64>() + terms(arguments),
        SemanticTerm::Lambda {
            parameters, body, ..
        } => {
            parameters
                .iter()
                .map(|parameter| type_node_count(&parameter.r#type))
                .sum::<u64>()
                + term_node_count(body)
        }
        SemanticTerm::Apply {
            function,
            arguments,
        } => term_node_count(function) + terms(arguments),
        SemanticTerm::FunctionRef { type_arguments, .. } => {
            type_arguments.iter().map(type_node_count).sum()
        }
        SemanticTerm::MapLiteral {
            key,
            value,
            entries,
        } => {
            type_node_count(key)
                + type_node_count(value)
                + entries
                    .iter()
                    .map(|entry| term_node_count(&entry.key) + term_node_count(&entry.value))
                    .sum::<u64>()
        }
        SemanticTerm::SetLiteral { element, elements } => {
            type_node_count(element) + terms(elements)
        }
        SemanticTerm::GraphLiteral { node, nodes, edges } => {
            type_node_count(node)
                + terms(nodes)
                + edges
                    .iter()
                    .map(|edge| term_node_count(&edge.source) + term_node_count(&edge.target))
                    .sum::<u64>()
        }
        SemanticTerm::If {
            condition,
            then_value,
            else_value,
        } => term_node_count(condition) + term_node_count(then_value) + term_node_count(else_value),
        SemanticTerm::Match {
            scrutinee,
            branches,
        } => {
            term_node_count(scrutinee)
                + branches
                    .iter()
                    .map(|branch| term_node_count(&branch.body))
                    .sum::<u64>()
        }
        SemanticTerm::Forall { binder, body } => {
            type_node_count(&binder.r#type) + term_node_count(body)
        }
        SemanticTerm::Let {
            binder,
            value,
            body,
        } => type_node_count(&binder.r#type) + term_node_count(value) + term_node_count(body),
        SemanticTerm::CheckedApply {
            type_arguments,
            arguments,
            checks,
            ..
        } => {
            type_arguments.iter().map(type_node_count).sum::<u64>()
                + terms(arguments)
                + checks.len() as u64
        }
    }
}

fn proof_node_count(proof: &SemanticProof) -> u64 {
    1 + match proof {
        SemanticProof::Reflexivity
        | SemanticProof::Decide
        | SemanticProof::LinearArithmetic { .. }
        | SemanticProof::Congruence
        | SemanticProof::Simplify { .. }
        | SemanticProof::BooleanReflection { .. } => 0,
        SemanticProof::Constructor { branches } => branches.iter().map(proof_node_count).sum(),
        SemanticProof::Cases { branches, .. } | SemanticProof::Induction { branches, .. } => {
            branches
                .iter()
                .map(|branch| proof_node_count(&branch.proof))
                .sum()
        }
        SemanticProof::Apply { arguments, .. } => arguments.iter().map(term_node_count).sum(),
    }
}

/// Language 1.2 also charges a data declaration's shape, which 1.1's count
/// leaves free: one node per type parameter, per constructor (so a nullary
/// constructor is not free), and for a mutual group label. Language 1.1 keeps
/// its historical count.
fn data_shape_node_count(declaration: &SemanticDeclaration) -> u64 {
    let count = |values: usize| u64::try_from(values).unwrap_or(u64::MAX);
    match declaration {
        SemanticDeclaration::Inductive {
            type_parameters,
            constructors,
            mutual,
            ..
        } => count(type_parameters.len())
            .saturating_add(count(constructors.len()))
            .saturating_add(u64::from(mutual.is_some())),
        SemanticDeclaration::Structure {
            type_parameters, ..
        }
        | SemanticDeclaration::Class {
            type_parameters, ..
        } => count(type_parameters.len()),
        SemanticDeclaration::Instance { .. }
        | SemanticDeclaration::Definition { .. }
        | SemanticDeclaration::Theorem { .. }
        | SemanticDeclaration::Artifact { .. }
        | SemanticDeclaration::Contract { .. }
        | SemanticDeclaration::Realization { .. }
        | SemanticDeclaration::Evidence { .. }
        | SemanticDeclaration::Model { .. }
        | SemanticDeclaration::Logic { .. }
        | SemanticDeclaration::InferenceRule { .. }
        | SemanticDeclaration::Verifier { .. }
        | SemanticDeclaration::Reasoner { .. } => 0,
    }
}

fn declaration_node_count(declaration: &SemanticDeclaration) -> u64 {
    let parameters = |values: &[SemanticParameter]| {
        values
            .iter()
            .map(|parameter| type_node_count(&parameter.r#type))
            .sum::<u64>()
    };
    1 + match declaration {
        SemanticDeclaration::Structure {
            parameters: values,
            fields,
            ..
        }
        | SemanticDeclaration::Class {
            parameters: values,
            fields,
            ..
        } => {
            parameters(values)
                + fields
                    .iter()
                    .map(|field| type_node_count(&field.r#type))
                    .sum::<u64>()
        }
        SemanticDeclaration::Instance {
            arguments, fields, ..
        } => {
            arguments.iter().map(type_node_count).sum::<u64>()
                + fields
                    .iter()
                    .map(|field| term_node_count(&field.value))
                    .sum::<u64>()
        }
        SemanticDeclaration::Inductive {
            parameters: values,
            constructors,
            ..
        } => {
            parameters(values)
                + constructors
                    .iter()
                    .flat_map(|constructor| &constructor.fields)
                    .map(type_node_count)
                    .sum::<u64>()
        }
        SemanticDeclaration::Definition {
            parameters: values,
            result,
            body,
            termination,
            ..
        } => {
            parameters(values)
                + type_node_count(result)
                + term_node_count(body)
                + termination.as_ref().map_or(0, |termination| {
                    term_node_count(&termination.measure) + termination.evidence.len() as u64
                })
        }
        SemanticDeclaration::Theorem {
            parameters: values,
            statement,
            proof,
            ..
        } => parameters(values) + term_node_count(statement) + proof_node_count(proof),
        SemanticDeclaration::Artifact { .. }
        | SemanticDeclaration::Contract { .. }
        | SemanticDeclaration::Realization { .. }
        | SemanticDeclaration::Evidence { .. }
        | SemanticDeclaration::Model { .. } => model::source_node_count(declaration),
        SemanticDeclaration::Logic { .. }
        | SemanticDeclaration::InferenceRule { .. }
        | SemanticDeclaration::Verifier { .. }
        | SemanticDeclaration::Reasoner { .. } => reasoning::source_node_count(declaration),
    }
}

/// The first source member the typed value dropped, as a JSON path.
fn ignored_member(
    source: &serde_json::Value,
    typed: &serde_json::Value,
    path: &str,
) -> Option<String> {
    match (source, typed) {
        (serde_json::Value::Object(source), serde_json::Value::Object(typed)) => {
            // serde's `deny_unknown_fields` rejects an unknown member of a
            // struct-shaped value, so a member the typed value lacks is
            // either a field serialized only when it differs from its
            // default (an empty list or `false`) or an extra member of a
            // unit variant, which serde ignores. A unit variant serializes to
            // its tag alone and so admits no other member at all.
            let unit_variant = typed.len() == 1 && typed.contains_key("kind");
            for (key, value) in source {
                let child = format!("{path}.{key}");
                match typed.get(key) {
                    Some(kept) => {
                        if let Some(found) = ignored_member(value, kept, &child) {
                            return Some(found);
                        }
                    }
                    None => {
                        let default = matches!(value, serde_json::Value::Bool(false))
                            || matches!(value, serde_json::Value::Array(items) if items.is_empty());
                        if unit_variant || !default {
                            return Some(child);
                        }
                    }
                }
            }
            None
        }
        (serde_json::Value::Array(source), serde_json::Value::Array(typed)) => source
            .iter()
            .zip(typed)
            .enumerate()
            .find_map(|(index, (value, kept))| {
                ignored_member(value, kept, &format!("{path}[{index}]"))
            }),
        _ => None,
    }
}

/// The semantic-module schema discriminator a project language requires
/// (§17.12). Language 1.0 has none.
#[must_use]
pub fn semantic_module_spec(language: &str) -> Option<&'static str> {
    match language {
        crate::LANGUAGE_1_1 => Some("lexlean/semantic-module/1"),
        crate::LANGUAGE_1_2 => Some("lexlean/semantic-module/2"),
        _ => None,
    }
}

/// The construct name when `term` itself (not a subterm) exists only in
/// language 1.2. Exhaustive by design: a new variant must state its
/// language here before it compiles.
fn language_1_2_construct(term: &SemanticTerm) -> Option<&'static str> {
    match term {
        SemanticTerm::Let { .. } => Some("let"),
        SemanticTerm::Pair { .. } => Some("pair"),
        SemanticTerm::First { .. } => Some("first"),
        SemanticTerm::Second { .. } => Some("second"),
        SemanticTerm::Lambda { .. } => Some("lambda"),
        SemanticTerm::Apply { .. } => Some("apply"),
        SemanticTerm::FunctionRef { .. } => Some("function_ref"),
        SemanticTerm::MapLiteral { .. } => Some("map_literal"),
        SemanticTerm::SetLiteral { .. } => Some("set_literal"),
        SemanticTerm::GraphLiteral { .. } => Some("graph_literal"),
        SemanticTerm::Primitive { operation, .. } if operation.language_1_2() => {
            Some("collection primitive")
        }
        SemanticTerm::Call { type_arguments, .. } if !type_arguments.is_empty() => {
            Some("call type arguments")
        }
        SemanticTerm::CheckedApply { .. } => Some("checked_apply"),
        SemanticTerm::Var { .. }
        | SemanticTerm::Nat { .. }
        | SemanticTerm::Integer { .. }
        | SemanticTerm::String { .. }
        | SemanticTerm::Bytes { .. }
        | SemanticTerm::Primitive { .. }
        | SemanticTerm::Bool { .. }
        | SemanticTerm::Unit
        | SemanticTerm::Nil { .. }
        | SemanticTerm::Cons { .. }
        | SemanticTerm::Record { .. }
        | SemanticTerm::Constructor { .. }
        | SemanticTerm::InstanceValue { .. }
        | SemanticTerm::Project { .. }
        | SemanticTerm::Call { .. }
        | SemanticTerm::If { .. }
        | SemanticTerm::Match { .. }
        | SemanticTerm::Eq { .. }
        | SemanticTerm::Le { .. }
        | SemanticTerm::Lt { .. }
        | SemanticTerm::Add { .. }
        | SemanticTerm::Beq { .. }
        | SemanticTerm::Ble { .. }
        | SemanticTerm::Blt { .. }
        | SemanticTerm::And { .. }
        | SemanticTerm::PropAnd { .. }
        | SemanticTerm::Or { .. }
        | SemanticTerm::Not { .. }
        | SemanticTerm::Implies { .. }
        | SemanticTerm::Iff { .. }
        | SemanticTerm::Forall { .. } => None,
    }
}

/// Visit `term` and every subterm, parents first.
pub(crate) fn visit_terms(term: &SemanticTerm, visit: &mut impl FnMut(&SemanticTerm)) {
    visit(term);
    match term {
        SemanticTerm::Var { .. }
        | SemanticTerm::Nat { .. }
        | SemanticTerm::Integer { .. }
        | SemanticTerm::String { .. }
        | SemanticTerm::Bytes { .. }
        | SemanticTerm::Bool { .. }
        | SemanticTerm::Unit
        | SemanticTerm::Nil { .. }
        | SemanticTerm::InstanceValue { .. } => {}
        SemanticTerm::Primitive { arguments, .. }
        | SemanticTerm::Constructor { arguments, .. }
        | SemanticTerm::Call { arguments, .. }
        | SemanticTerm::CheckedApply { arguments, .. } => {
            for argument in arguments {
                visit_terms(argument, visit);
            }
        }
        SemanticTerm::Record { fields, .. } => {
            for field in fields {
                visit_terms(&field.value, visit);
            }
        }
        SemanticTerm::Cons { head, tail }
        | SemanticTerm::Eq {
            left: head,
            right: tail,
        }
        | SemanticTerm::Le {
            left: head,
            right: tail,
        }
        | SemanticTerm::Lt {
            left: head,
            right: tail,
        }
        | SemanticTerm::Add {
            left: head,
            right: tail,
        }
        | SemanticTerm::Beq {
            left: head,
            right: tail,
        }
        | SemanticTerm::Ble {
            left: head,
            right: tail,
        }
        | SemanticTerm::Blt {
            left: head,
            right: tail,
        }
        | SemanticTerm::And {
            left: head,
            right: tail,
        }
        | SemanticTerm::PropAnd {
            left: head,
            right: tail,
        }
        | SemanticTerm::Or {
            left: head,
            right: tail,
        }
        | SemanticTerm::Implies {
            premise: head,
            conclusion: tail,
        }
        | SemanticTerm::Iff {
            left: head,
            right: tail,
        }
        | SemanticTerm::Pair {
            left: head,
            right: tail,
        } => {
            visit_terms(head, visit);
            visit_terms(tail, visit);
        }
        SemanticTerm::Project { value, .. }
        | SemanticTerm::Not { value }
        | SemanticTerm::First { value }
        | SemanticTerm::Second { value } => {
            visit_terms(value, visit);
        }
        SemanticTerm::If {
            condition,
            then_value,
            else_value,
        } => {
            visit_terms(condition, visit);
            visit_terms(then_value, visit);
            visit_terms(else_value, visit);
        }
        SemanticTerm::Match {
            scrutinee,
            branches,
        } => {
            visit_terms(scrutinee, visit);
            for branch in branches {
                visit_terms(&branch.body, visit);
            }
        }
        SemanticTerm::Forall { body, .. } => visit_terms(body, visit),
        SemanticTerm::Let { value, body, .. } => {
            visit_terms(value, visit);
            visit_terms(body, visit);
        }
        SemanticTerm::Lambda { body, .. } => visit_terms(body, visit),
        SemanticTerm::Apply {
            function,
            arguments,
        } => {
            visit_terms(function, visit);
            for argument in arguments {
                visit_terms(argument, visit);
            }
        }
        SemanticTerm::FunctionRef { .. } => {}
        SemanticTerm::MapLiteral { entries, .. } => {
            for entry in entries {
                visit_terms(&entry.key, visit);
                visit_terms(&entry.value, visit);
            }
        }
        SemanticTerm::SetLiteral { elements, .. } => {
            for element in elements {
                visit_terms(element, visit);
            }
        }
        SemanticTerm::GraphLiteral { nodes, edges, .. } => {
            for node in nodes {
                visit_terms(node, visit);
            }
            for edge in edges {
                visit_terms(&edge.source, visit);
                visit_terms(&edge.target, visit);
            }
        }
    }
}

/// Visit `term` and every subterm mutably, children first, so a visitor
/// that rewrites a node sees its rewritten children.
fn visit_terms_mut(term: &mut SemanticTerm, visit: &mut impl FnMut(&mut SemanticTerm)) {
    match term {
        SemanticTerm::Var { .. }
        | SemanticTerm::Nat { .. }
        | SemanticTerm::Integer { .. }
        | SemanticTerm::String { .. }
        | SemanticTerm::Bytes { .. }
        | SemanticTerm::Bool { .. }
        | SemanticTerm::Unit
        | SemanticTerm::Nil { .. }
        | SemanticTerm::InstanceValue { .. } => {}
        SemanticTerm::Primitive { arguments, .. }
        | SemanticTerm::Constructor { arguments, .. }
        | SemanticTerm::Call { arguments, .. }
        | SemanticTerm::CheckedApply { arguments, .. } => {
            for argument in arguments {
                visit_terms_mut(argument, visit);
            }
        }
        SemanticTerm::Record { fields, .. } => {
            for field in fields {
                visit_terms_mut(&mut field.value, visit);
            }
        }
        SemanticTerm::Cons { head, tail }
        | SemanticTerm::Eq {
            left: head,
            right: tail,
        }
        | SemanticTerm::Le {
            left: head,
            right: tail,
        }
        | SemanticTerm::Lt {
            left: head,
            right: tail,
        }
        | SemanticTerm::Add {
            left: head,
            right: tail,
        }
        | SemanticTerm::Beq {
            left: head,
            right: tail,
        }
        | SemanticTerm::Ble {
            left: head,
            right: tail,
        }
        | SemanticTerm::Blt {
            left: head,
            right: tail,
        }
        | SemanticTerm::And {
            left: head,
            right: tail,
        }
        | SemanticTerm::PropAnd {
            left: head,
            right: tail,
        }
        | SemanticTerm::Or {
            left: head,
            right: tail,
        }
        | SemanticTerm::Implies {
            premise: head,
            conclusion: tail,
        }
        | SemanticTerm::Iff {
            left: head,
            right: tail,
        }
        | SemanticTerm::Pair {
            left: head,
            right: tail,
        } => {
            visit_terms_mut(head, visit);
            visit_terms_mut(tail, visit);
        }
        SemanticTerm::Project { value, .. }
        | SemanticTerm::Not { value }
        | SemanticTerm::First { value }
        | SemanticTerm::Second { value } => {
            visit_terms_mut(value, visit);
        }
        SemanticTerm::If {
            condition,
            then_value,
            else_value,
        } => {
            visit_terms_mut(condition, visit);
            visit_terms_mut(then_value, visit);
            visit_terms_mut(else_value, visit);
        }
        SemanticTerm::Match {
            scrutinee,
            branches,
        } => {
            visit_terms_mut(scrutinee, visit);
            for branch in branches {
                visit_terms_mut(&mut branch.body, visit);
            }
        }
        SemanticTerm::Forall { body, .. } => visit_terms_mut(body, visit),
        SemanticTerm::Let { value, body, .. } => {
            visit_terms_mut(value, visit);
            visit_terms_mut(body, visit);
        }
        SemanticTerm::Lambda { body, .. } => visit_terms_mut(body, visit),
        SemanticTerm::Apply {
            function,
            arguments,
        } => {
            visit_terms_mut(function, visit);
            for argument in arguments {
                visit_terms_mut(argument, visit);
            }
        }
        SemanticTerm::FunctionRef { .. } => {}
        SemanticTerm::MapLiteral { entries, .. } => {
            for entry in entries {
                visit_terms_mut(&mut entry.key, visit);
                visit_terms_mut(&mut entry.value, visit);
            }
        }
        SemanticTerm::SetLiteral { elements, .. } => {
            for element in elements {
                visit_terms_mut(element, visit);
            }
        }
        SemanticTerm::GraphLiteral { nodes, edges, .. } => {
            for node in nodes {
                visit_terms_mut(node, visit);
            }
            for edge in edges {
                visit_terms_mut(&mut edge.source, visit);
                visit_terms_mut(&mut edge.target, visit);
            }
        }
    }
    visit(term);
}

/// The source spelling of a type in diagnostics: the document's own names,
/// never the compiler's internal representation.
impl std::fmt::Display for SemanticType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let simple = match self {
            Self::Type => "Type",
            Self::Nat => "Nat",
            Self::Bool => "Bool",
            Self::Prop => "Prop",
            Self::Unit => "Unit",
            Self::Int => "Int",
            Self::Int8 => "Int8",
            Self::Int16 => "Int16",
            Self::Int32 => "Int32",
            Self::Int64 => "Int64",
            Self::UInt8 => "UInt8",
            Self::UInt16 => "UInt16",
            Self::UInt32 => "UInt32",
            Self::UInt64 => "UInt64",
            Self::String => "String",
            Self::Bytes => "Bytes",
            Self::Ordering => "Ordering",
            Self::ContractViolation => "ContractViolation",
            Self::ReasoningFailure => "ReasoningFailure",
            Self::Parameter { name } => return f.write_str(name),
            Self::Option { value } => return write!(f, "Option ({value})"),
            Self::Result { ok, error } => return write!(f, "Result ({ok}) ({error})"),
            Self::List { element } => return write!(f, "List ({element})"),
            Self::Product { left, right } => return write!(f, "Prod ({left}) ({right})"),
            Self::Map { key, value } => return write!(f, "Map ({key}) ({value})"),
            Self::Set { element } => return write!(f, "Set ({element})"),
            Self::Function { parameters, result } => {
                f.write_str("(")?;
                for parameter in parameters {
                    write!(f, "({parameter}) -> ")?;
                }
                return write!(f, "({result}))");
            }
            Self::Named { member, arguments } => {
                if let Some(module) = &member.module {
                    write!(f, "{module}.")?;
                }
                f.write_str(&member.name)?;
                for argument in arguments {
                    write!(f, " ({argument})")?;
                }
                return Ok(());
            }
        };
        f.write_str(simple)
    }
}

/// The construct name when `ty` contains a language-1.2-only type form.
fn language_1_2_type(ty: &SemanticType) -> Option<&'static str> {
    match ty {
        SemanticType::Product { .. } => Some("product type"),
        SemanticType::Function { .. } => Some("function type"),
        SemanticType::Map { .. } => Some("map type"),
        SemanticType::Set { .. } => Some("set type"),
        SemanticType::ContractViolation => Some("contract_violation type"),
        SemanticType::ReasoningFailure => Some("reasoning_failure type"),
        SemanticType::Option { value: inner } | SemanticType::List { element: inner } => {
            language_1_2_type(inner)
        }
        SemanticType::Result { ok, error } => {
            language_1_2_type(ok).or_else(|| language_1_2_type(error))
        }
        SemanticType::Named { arguments, .. } => arguments.iter().find_map(language_1_2_type),
        SemanticType::Type
        | SemanticType::Parameter { .. }
        | SemanticType::Nat
        | SemanticType::Bool
        | SemanticType::Prop
        | SemanticType::Unit
        | SemanticType::Int
        | SemanticType::Int8
        | SemanticType::Int16
        | SemanticType::Int32
        | SemanticType::Int64
        | SemanticType::UInt8
        | SemanticType::UInt16
        | SemanticType::UInt32
        | SemanticType::UInt64
        | SemanticType::String
        | SemanticType::Bytes
        | SemanticType::Ordering => None,
    }
}

/// Visit the types a term node itself carries (not those of subterms).
fn term_types(term: &SemanticTerm, visit: &mut impl FnMut(&SemanticType)) {
    match term {
        SemanticTerm::Nil { element } => visit(element),
        SemanticTerm::Primitive { result, .. } => visit(result),
        SemanticTerm::Record { type_arguments, .. }
        | SemanticTerm::Constructor { type_arguments, .. } => {
            type_arguments.iter().for_each(&mut *visit);
        }
        SemanticTerm::InstanceValue { arguments, .. } => arguments.iter().for_each(&mut *visit),
        SemanticTerm::Forall { binder, .. } | SemanticTerm::Let { binder, .. } => {
            visit(&binder.r#type);
        }
        SemanticTerm::Call { type_arguments, .. }
        | SemanticTerm::FunctionRef { type_arguments, .. }
        | SemanticTerm::CheckedApply { type_arguments, .. } => {
            type_arguments.iter().for_each(&mut *visit);
        }
        SemanticTerm::Lambda { parameters, .. } => {
            for parameter in parameters {
                visit(&parameter.r#type);
            }
        }
        SemanticTerm::MapLiteral { key, value, .. } => {
            visit(key);
            visit(value);
        }
        SemanticTerm::SetLiteral { element, .. } => visit(element),
        SemanticTerm::GraphLiteral { node, .. } => visit(node),
        SemanticTerm::Var { .. }
        | SemanticTerm::Nat { .. }
        | SemanticTerm::Integer { .. }
        | SemanticTerm::String { .. }
        | SemanticTerm::Bytes { .. }
        | SemanticTerm::Bool { .. }
        | SemanticTerm::Unit
        | SemanticTerm::Cons { .. }
        | SemanticTerm::Project { .. }
        | SemanticTerm::Apply { .. }
        | SemanticTerm::If { .. }
        | SemanticTerm::Match { .. }
        | SemanticTerm::Eq { .. }
        | SemanticTerm::Le { .. }
        | SemanticTerm::Lt { .. }
        | SemanticTerm::Add { .. }
        | SemanticTerm::Beq { .. }
        | SemanticTerm::Ble { .. }
        | SemanticTerm::Blt { .. }
        | SemanticTerm::And { .. }
        | SemanticTerm::PropAnd { .. }
        | SemanticTerm::Or { .. }
        | SemanticTerm::Not { .. }
        | SemanticTerm::Implies { .. }
        | SemanticTerm::Iff { .. }
        | SemanticTerm::Pair { .. }
        | SemanticTerm::First { .. }
        | SemanticTerm::Second { .. } => {}
    }
}

/// Visit every type a declaration states directly (parameters, fields,
/// constructor fields, instance arguments, and results).
fn declaration_types(declaration: &SemanticDeclaration, visit: &mut impl FnMut(&SemanticType)) {
    let parameters = |values: &[SemanticParameter], visit: &mut dyn FnMut(&SemanticType)| {
        for parameter in values {
            visit(&parameter.r#type);
        }
    };
    match declaration {
        SemanticDeclaration::Structure {
            parameters: values,
            fields,
            ..
        }
        | SemanticDeclaration::Class {
            parameters: values,
            fields,
            ..
        } => {
            parameters(values, visit);
            for field in fields {
                visit(&field.r#type);
            }
        }
        SemanticDeclaration::Instance { arguments, .. } => arguments.iter().for_each(visit),
        SemanticDeclaration::Inductive {
            parameters: values,
            constructors,
            ..
        } => {
            parameters(values, visit);
            for constructor in constructors {
                constructor.fields.iter().for_each(&mut *visit);
            }
        }
        SemanticDeclaration::Definition {
            parameters: values,
            result,
            ..
        } => {
            parameters(values, visit);
            visit(result);
        }
        SemanticDeclaration::Theorem {
            parameters: values, ..
        } => parameters(values, visit),
        SemanticDeclaration::Artifact { .. }
        | SemanticDeclaration::Contract { .. }
        | SemanticDeclaration::Realization { .. }
        | SemanticDeclaration::Evidence { .. }
        | SemanticDeclaration::Model { .. } => model::declaration_types(declaration, visit),
        SemanticDeclaration::Logic { .. }
        | SemanticDeclaration::InferenceRule { .. }
        | SemanticDeclaration::Verifier { .. }
        | SemanticDeclaration::Reasoner { .. } => reasoning::declaration_types(declaration, visit),
    }
}

fn proof_terms(proof: &SemanticProof, visit: &mut impl FnMut(&SemanticTerm)) {
    match proof {
        SemanticProof::Reflexivity
        | SemanticProof::Decide
        | SemanticProof::LinearArithmetic { .. }
        | SemanticProof::Simplify { .. }
        | SemanticProof::Congruence
        | SemanticProof::BooleanReflection { .. } => {}
        SemanticProof::Constructor { branches } => {
            for branch in branches {
                proof_terms(branch, visit);
            }
        }
        SemanticProof::Cases { branches, .. } | SemanticProof::Induction { branches, .. } => {
            for branch in branches {
                proof_terms(&branch.proof, visit);
            }
        }
        SemanticProof::Apply { arguments, .. } => {
            for argument in arguments {
                visit_terms(argument, visit);
            }
        }
    }
}

fn proof_terms_mut(proof: &mut SemanticProof, visit: &mut impl FnMut(&mut SemanticTerm)) {
    match proof {
        SemanticProof::Reflexivity
        | SemanticProof::Decide
        | SemanticProof::LinearArithmetic { .. }
        | SemanticProof::Simplify { .. }
        | SemanticProof::Congruence
        | SemanticProof::BooleanReflection { .. } => {}
        SemanticProof::Constructor { branches } => {
            for branch in branches {
                proof_terms_mut(branch, visit);
            }
        }
        SemanticProof::Cases { branches, .. } | SemanticProof::Induction { branches, .. } => {
            for branch in branches {
                proof_terms_mut(&mut branch.proof, visit);
            }
        }
        SemanticProof::Apply { arguments, .. } => {
            for argument in arguments {
                visit_terms_mut(argument, visit);
            }
        }
    }
}

/// Lean names the backend emits unqualified: built-in types and their
/// constructor owners, the propositional connectives its proofs name, and the
/// runtime namespaces. A binder spelled like one would capture it.
const BACKEND_BARE_NAMES: [&str; 35] = [
    "And",
    "Bool",
    "ByteArray",
    "Except",
    "Iff",
    "Int",
    "Int16",
    "Int32",
    "Int64",
    "Int8",
    "LexLeanCollections",
    "LexLeanModels",
    "LexLeanReasoning",
    "LexLeanRuntime",
    "List",
    "Nat",
    "Option",
    "Ordering",
    "Prod",
    "Prop",
    "Result",
    "String",
    "Type",
    "UInt16",
    "UInt32",
    "UInt64",
    "UInt8",
    "Unit",
    "and_congr",
    "and_true",
    "congr",
    "decide",
    "id",
    "rfl",
    "true_and",
];

fn proof_binders(proof: &SemanticProof, visit: &mut impl FnMut(&str)) {
    match proof {
        SemanticProof::Reflexivity
        | SemanticProof::Decide
        | SemanticProof::Simplify { .. }
        | SemanticProof::Congruence
        | SemanticProof::BooleanReflection { .. }
        | SemanticProof::LinearArithmetic { .. }
        | SemanticProof::Apply { .. } => {}
        SemanticProof::Constructor { branches } => {
            for branch in branches {
                proof_binders(branch, visit);
            }
        }
        SemanticProof::Cases { branches, .. } | SemanticProof::Induction { branches, .. } => {
            for branch in branches {
                branch.binders.iter().for_each(|binder| visit(binder));
                proof_binders(&branch.proof, visit);
            }
        }
    }
}

/// Visit every name a declaration binds locally: type parameters, value
/// parameters, and the binders of its terms and proofs.
fn declaration_binders(declaration: &SemanticDeclaration, visit: &mut impl FnMut(&str)) {
    let (type_parameters, parameters): (&[String], &[SemanticParameter]) = match declaration {
        SemanticDeclaration::Structure {
            type_parameters,
            parameters,
            ..
        }
        | SemanticDeclaration::Class {
            type_parameters,
            parameters,
            ..
        }
        | SemanticDeclaration::Inductive {
            type_parameters,
            parameters,
            ..
        } => (type_parameters, parameters),
        SemanticDeclaration::Definition {
            type_parameters,
            parameters,
            ..
        }
        | SemanticDeclaration::Theorem {
            type_parameters,
            parameters,
            ..
        } => (type_parameters, parameters),
        SemanticDeclaration::Instance { .. } | SemanticDeclaration::Artifact { .. } => (&[], &[]),
        SemanticDeclaration::Contract {
            type_parameters, ..
        }
        | SemanticDeclaration::Realization {
            type_parameters, ..
        }
        | SemanticDeclaration::Evidence {
            type_parameters, ..
        }
        | SemanticDeclaration::Model {
            type_parameters, ..
        }
        | SemanticDeclaration::Logic {
            type_parameters, ..
        }
        | SemanticDeclaration::InferenceRule {
            type_parameters, ..
        }
        | SemanticDeclaration::Verifier {
            type_parameters, ..
        }
        | SemanticDeclaration::Reasoner {
            type_parameters, ..
        } => (type_parameters, &[]),
    };
    model::declaration_binders(declaration, visit);
    reasoning::declaration_binders(declaration, visit);
    type_parameters.iter().for_each(|name| visit(name));
    parameters
        .iter()
        .for_each(|parameter| visit(&parameter.name));
    declaration_terms(declaration, &mut |term| match term {
        SemanticTerm::Match { branches, .. } => {
            for branch in branches {
                branch.binders.iter().for_each(|binder| visit(binder));
            }
        }
        SemanticTerm::Forall { binder, .. } | SemanticTerm::Let { binder, .. } => {
            visit(&binder.name);
        }
        SemanticTerm::Lambda { parameters, .. } => {
            parameters
                .iter()
                .for_each(|parameter| visit(&parameter.name));
        }
        _ => {}
    });
    if let SemanticDeclaration::Theorem { proof, .. } = declaration {
        proof_binders(proof, visit);
    }
}

/// §17.12 binder hygiene (language 1.2): the backend refers to this module's
/// declarations, to built-in Lean names, and to every imported declaration
/// through the project's module prefix without qualification, so no local
/// binder may be spelled like any of them. Lean would otherwise resolve the
/// reference to the binder after linking has accepted the module.
fn check_binder_hygiene(module: &SemanticModule, module_prefix: &str) -> Result<(), String> {
    let declared: BTreeSet<&str> = module
        .declarations
        .iter()
        .map(SemanticDeclaration::name)
        .collect();
    let prefix_root = module_prefix.split('.').next().unwrap_or(module_prefix);
    let mut failure = None;
    for declaration in &module.declarations {
        declaration_binders(declaration, &mut |binder| {
            if failure.is_some() {
                return;
            }
            let captured = if declared.contains(binder) {
                Some("declaration")
            } else if BACKEND_BARE_NAMES.contains(&binder) {
                Some("built-in Lean name")
            } else if binder == prefix_root {
                Some("module prefix")
            } else {
                None
            };
            if let Some(what) = captured {
                failure = Some(format!(
                    "binder `{binder}` in `{}` is spelled like the {what} `{binder}` and would capture it in generated Lean",
                    declaration.name()
                ));
            }
        });
    }
    failure.map_or(Ok(()), Err)
}

/// Visit every term a declaration carries, including proof arguments.
fn declaration_terms(declaration: &SemanticDeclaration, visit: &mut impl FnMut(&SemanticTerm)) {
    match declaration {
        SemanticDeclaration::Structure { .. }
        | SemanticDeclaration::Class { .. }
        | SemanticDeclaration::Inductive { .. } => {}
        SemanticDeclaration::Instance { fields, .. } => {
            for field in fields {
                visit_terms(&field.value, visit);
            }
        }
        SemanticDeclaration::Definition {
            body, termination, ..
        } => {
            visit_terms(body, visit);
            if let Some(termination) = termination {
                visit_terms(&termination.measure, visit);
            }
        }
        SemanticDeclaration::Theorem {
            statement, proof, ..
        } => {
            visit_terms(statement, visit);
            proof_terms(proof, visit);
        }
        SemanticDeclaration::Artifact { .. }
        | SemanticDeclaration::Contract { .. }
        | SemanticDeclaration::Realization { .. }
        | SemanticDeclaration::Evidence { .. }
        | SemanticDeclaration::Model { .. } => model::declaration_terms(declaration, visit),
        SemanticDeclaration::Logic { .. }
        | SemanticDeclaration::InferenceRule { .. }
        | SemanticDeclaration::Verifier { .. }
        | SemanticDeclaration::Reasoner { .. } => reasoning::declaration_terms(declaration, visit),
    }
}

/// Visit every term a declaration carries mutably, including proof
/// arguments.
fn declaration_terms_mut(
    declaration: &mut SemanticDeclaration,
    visit: &mut impl FnMut(&mut SemanticTerm),
) {
    match declaration {
        SemanticDeclaration::Structure { .. }
        | SemanticDeclaration::Class { .. }
        | SemanticDeclaration::Inductive { .. } => {}
        SemanticDeclaration::Instance { fields, .. } => {
            for field in fields {
                visit_terms_mut(&mut field.value, visit);
            }
        }
        SemanticDeclaration::Definition {
            body, termination, ..
        } => {
            visit_terms_mut(body, visit);
            if let Some(termination) = termination {
                visit_terms_mut(&mut termination.measure, visit);
            }
        }
        SemanticDeclaration::Theorem {
            statement, proof, ..
        } => {
            visit_terms_mut(statement, visit);
            proof_terms_mut(proof, visit);
        }
        SemanticDeclaration::Artifact { .. }
        | SemanticDeclaration::Contract { .. }
        | SemanticDeclaration::Realization { .. }
        | SemanticDeclaration::Evidence { .. }
        | SemanticDeclaration::Model { .. } => model::declaration_terms_mut(declaration, visit),
        SemanticDeclaration::Logic { .. }
        | SemanticDeclaration::InferenceRule { .. }
        | SemanticDeclaration::Verifier { .. }
        | SemanticDeclaration::Reasoner { .. } => {
            reasoning::declaration_terms_mut(declaration, visit);
        }
    }
}

fn type_mentions_parameter(ty: &SemanticType, name: &str) -> bool {
    match ty {
        SemanticType::Parameter { name: parameter } => parameter == name,
        SemanticType::Option { value: inner }
        | SemanticType::List { element: inner }
        | SemanticType::Set { element: inner } => type_mentions_parameter(inner, name),
        SemanticType::Result { ok, error }
        | SemanticType::Map {
            key: ok,
            value: error,
        } => type_mentions_parameter(ok, name) || type_mentions_parameter(error, name),
        SemanticType::Product { left, right } => {
            type_mentions_parameter(left, name) || type_mentions_parameter(right, name)
        }
        SemanticType::Named { arguments, .. } => arguments
            .iter()
            .any(|argument| type_mentions_parameter(argument, name)),
        SemanticType::Function { parameters, result } => {
            parameters
                .iter()
                .any(|parameter| type_mentions_parameter(parameter, name))
                || type_mentions_parameter(result, name)
        }
        SemanticType::Type
        | SemanticType::Nat
        | SemanticType::Bool
        | SemanticType::Prop
        | SemanticType::Unit
        | SemanticType::Int
        | SemanticType::Int8
        | SemanticType::Int16
        | SemanticType::Int32
        | SemanticType::Int64
        | SemanticType::UInt8
        | SemanticType::UInt16
        | SemanticType::UInt32
        | SemanticType::UInt64
        | SemanticType::String
        | SemanticType::Bytes
        | SemanticType::Ordering
        | SemanticType::ContractViolation
        | SemanticType::ReasoningFailure => false,
    }
}

fn proof_type_arguments(proof: &SemanticProof, visit: &mut impl FnMut(&SemanticType)) {
    match proof {
        SemanticProof::Reflexivity
        | SemanticProof::LinearArithmetic { .. }
        | SemanticProof::Decide
        | SemanticProof::Simplify { .. }
        | SemanticProof::Congruence
        | SemanticProof::BooleanReflection { .. } => {}
        SemanticProof::Constructor { branches } => {
            for branch in branches {
                proof_type_arguments(branch, visit);
            }
        }
        SemanticProof::Cases { branches, .. } | SemanticProof::Induction { branches, .. } => {
            for branch in branches {
                proof_type_arguments(&branch.proof, visit);
            }
        }
        SemanticProof::Apply { type_arguments, .. } => type_arguments.iter().for_each(visit),
    }
}

/// Whether a declaration mentions its type parameter `name` anywhere: in a
/// parameter, field, or result type, or in a type written in its body,
/// statement, or proof. An unmentioned one lowers as `_name` (§17.12).
pub(crate) fn mentions_type_parameter(declaration: &SemanticDeclaration, name: &str) -> bool {
    let mut found = false;
    let mut visit = |ty: &SemanticType| found = found || type_mentions_parameter(ty, name);
    declaration_types(declaration, &mut visit);
    declaration_terms(declaration, &mut |term| term_types(term, &mut visit));
    if let SemanticDeclaration::Theorem { proof, .. } = declaration {
        proof_type_arguments(proof, &mut visit);
    }
    found
}

impl SemanticModule {
    /// The ordinary declarations this module means, in order: each model
    /// declaration as its elaboration, and each other declaration with every
    /// checked model application elaborated (§17.12). Every backend reads
    /// these.
    #[must_use]
    pub fn lowered_declarations(&self) -> Vec<&SemanticDeclaration> {
        if self.elaboration.lowered.len() == self.declarations.len() {
            self.elaboration.lowered.iter().flatten().collect()
        } else {
            self.declarations.iter().collect()
        }
    }

    /// Every Lean declaration the module generates for its semantic
    /// declarations, in order: each ordinary declaration by its own name, and
    /// each model declaration as the declarations and cross-checks it
    /// elaborates to (§17.12).
    #[must_use]
    pub fn generated_names(&self) -> Vec<String> {
        let mut names = Vec::new();
        for (index, declaration) in self.declarations.iter().enumerate() {
            match self.elaborated(index) {
                Some(lowered) if model::elaborated_construct(declaration).is_some() => {
                    names.extend(lowered.iter().map(|derived| derived.name().to_owned()));
                    names.extend(
                        self.elaboration
                            .theorems(index)
                            .iter()
                            .map(|theorem| theorem.name.clone()),
                    );
                    names.extend(
                        self.elaboration
                            .checks(index)
                            .iter()
                            .map(|check| check.name.clone()),
                    );
                }
                _ => names.push(declaration.name().to_owned()),
            }
        }
        names
    }

    /// The ordinary declarations source declaration `index` means, or
    /// `None` for a module linking never elaborated.
    #[must_use]
    pub fn elaborated(&self, index: usize) -> Option<&[SemanticDeclaration]> {
        (self.elaboration.lowered.len() == self.declarations.len())
            .then(|| self.elaboration.lowered(index))
    }

    /// Does the ordinary source declaration `name` apply a model through a
    /// checked application, so that what is realized is its elaborated copy
    /// (§17.12, §17.13)?
    #[must_use]
    pub fn applies_checked(&self, name: &str) -> bool {
        self.declarations
            .iter()
            .enumerate()
            .any(|(index, declaration)| {
                declaration.name() == name
                    && model::elaborated_construct(declaration).is_none()
                    && self
                        .elaborated(index)
                        .is_some_and(|lowered| lowered != std::slice::from_ref(declaration))
            })
    }

    /// Exact recursive semantic-node count charged to `max_ir_nodes`.
    pub(crate) fn node_count(&self) -> u64 {
        let base: u64 = self
            .declarations
            .iter()
            .map(declaration_node_count)
            .sum::<u64>()
            + self.elaboration.node_count(&self.declarations);
        if semantic_module_spec(crate::LANGUAGE_1_2) == Some(self.spec.as_str()) {
            base + self
                .declarations
                .iter()
                .map(data_shape_node_count)
                .sum::<u64>()
        } else {
            base
        }
    }
}

#[derive(Default)]
struct Environment<'a> {
    /// Whether the module is language-1.2 data (`lexlean/semantic-module/2`).
    language_1_2: bool,
    imports: &'a [String],
    types: BTreeMap<String, TypeInfo>,
    functions: BTreeMap<String, FunctionInfo>,
    instances: BTreeMap<String, MemberRef>,
    proof_rules: BTreeMap<String, Vec<SemanticType>>,
    /// Language 1.2: the explicit type parameters of each theorem.
    proof_type_parameters: BTreeMap<String, Vec<String>>,
    /// Language 1.2: the type parameters of the declaration being checked;
    /// a recursive call must pass exactly these.
    current_type_parameters: Vec<String>,
    /// Language 1.2: the mutual definition group being checked, by member
    /// name and decreasing-argument position.
    recursive_group: BTreeMap<String, usize>,
    /// Language 1.2: the recursive family of that group (§17.12).
    recursive_family: Option<Family>,
    /// Language 1.2: the decreasing parameter type of the member being
    /// checked.
    current_decreasing_type: Option<SemanticType>,
    /// Language 1.2: the well-founded definition being checked, which may be
    /// called only at planned call sites and never referenced as a value.
    /// The well-founded definitions being checked, standalone or one mutual
    /// group, with each member's parameters and measure: calls to them are
    /// recursive, and each call's obligation reads the callee's measure.
    well_founded_group: BTreeMap<String, (Vec<SemanticParameter>, SemanticTerm)>,
    /// Language 1.2: local theorems by name, for termination evidence.
    theorems: BTreeMap<String, (Vec<String>, Vec<SemanticParameter>, SemanticTerm)>,
    /// Language 1.2 (models): every visible artifact, contract,
    /// realization, evidence, and model, by environment key.
    models: model::Models,
    /// Language 1.2 (reasoning): every visible logic, rule, and verifier,
    /// and the generated functions executable code may not reach directly.
    reasoning: reasoning::Reasoning,
    /// Language 1.2 (models): the declaration being checked was elaborated
    /// from a model declaration, so generated binders are admitted.
    derived: bool,
}

#[derive(Clone, Default)]
struct TypeInfo {
    parameters: usize,
    fields: Vec<String>,
    constructors: BTreeMap<String, usize>,
    type_parameters: Vec<String>,
    field_types: Vec<SemanticType>,
    constructor_types: BTreeMap<String, Vec<SemanticType>>,
    class: bool,
    /// Language 1.2: per constructor, which fields are a direct recursive
    /// occurrence of this very type (the structurally smaller positions).
    direct_recursive: BTreeMap<String, Vec<bool>>,
    /// Language 1.2: a recursive occurrence of the group appears nested
    /// under a container, or the type belongs to a mutual group.
    nested_or_mutual: bool,
    /// Language 1.2: the environment keys of every member of this type's
    /// inductive group, itself included.
    group: Vec<String>,
}

#[derive(Clone)]
struct FunctionInfo {
    /// Language 1.2: explicit type parameters, substituted at every use.
    type_parameters: Vec<String>,
    parameters: Vec<SemanticType>,
    result: SemanticType,
    /// Language 1.2: the definition asserts production eligibility.
    executable: bool,
}

fn member_key(member: &MemberRef) -> String {
    match &member.module {
        Some(module) => format!("{module}::{}", member.name),
        None => member.name.clone(),
    }
}

fn member_from_key(key: &str) -> MemberRef {
    match key.split_once("::") {
        Some((module, name)) => MemberRef {
            module: Some(module.to_owned()),
            name: name.to_owned(),
        },
        None => MemberRef {
            module: None,
            name: key.to_owned(),
        },
    }
}

fn qualify_type(ty: &SemanticType, module: &str) -> SemanticType {
    match ty {
        SemanticType::Named { member, arguments } => SemanticType::Named {
            member: MemberRef {
                module: member.module.clone().or_else(|| Some(module.to_owned())),
                name: member.name.clone(),
            },
            arguments: arguments
                .iter()
                .map(|argument| qualify_type(argument, module))
                .collect(),
        },
        SemanticType::List { element } => SemanticType::List {
            element: Box::new(qualify_type(element, module)),
        },
        SemanticType::Option { value } => SemanticType::Option {
            value: Box::new(qualify_type(value, module)),
        },
        SemanticType::Result { ok, error } => SemanticType::Result {
            ok: Box::new(qualify_type(ok, module)),
            error: Box::new(qualify_type(error, module)),
        },
        SemanticType::Product { left, right } => SemanticType::Product {
            left: Box::new(qualify_type(left, module)),
            right: Box::new(qualify_type(right, module)),
        },
        SemanticType::Function { parameters, result } => SemanticType::Function {
            parameters: parameters
                .iter()
                .map(|parameter| qualify_type(parameter, module))
                .collect(),
            result: Box::new(qualify_type(result, module)),
        },
        SemanticType::Map { key, value } => SemanticType::Map {
            key: Box::new(qualify_type(key, module)),
            value: Box::new(qualify_type(value, module)),
        },
        SemanticType::Set { element } => SemanticType::Set {
            element: Box::new(qualify_type(element, module)),
        },
        SemanticType::Type
        | SemanticType::Parameter { .. }
        | SemanticType::Nat
        | SemanticType::Bool
        | SemanticType::Prop
        | SemanticType::Unit
        | SemanticType::Int
        | SemanticType::Int8
        | SemanticType::Int16
        | SemanticType::Int32
        | SemanticType::Int64
        | SemanticType::UInt8
        | SemanticType::UInt16
        | SemanticType::UInt32
        | SemanticType::UInt64
        | SemanticType::String
        | SemanticType::Bytes
        | SemanticType::Ordering
        | SemanticType::ContractViolation
        | SemanticType::ReasoningFailure => ty.clone(),
    }
}

fn type_info<'a>(member: &MemberRef, env: &'a Environment<'_>) -> Option<&'a TypeInfo> {
    env.types.get(&member_key(member))
}

fn function_info<'a>(member: &MemberRef, env: &'a Environment<'_>) -> Option<&'a FunctionInfo> {
    env.functions.get(&member_key(member))
}

fn legal_name(name: &str) -> bool {
    let mut segments = name.split('.');
    segments.all(|segment| {
        let mut chars = segment.chars();
        chars
            .next()
            .is_some_and(|first| first.is_ascii_alphabetic())
            && chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
            && !matches!(
                segment,
                "axiom"
                    | "opaque"
                    | "partial"
                    | "unsafe"
                    | "noncomputable"
                    | "termination_by"
                    | "sorry"
                    | "admit"
            )
    })
}

/// A local binder: a source name, or, only in a declaration linking
/// elaborated from a model (§17.12), a generated name `__<name>`. No source
/// name begins with an underscore, so a generated binder captures nothing.
fn check_binder(name: &str, what: &str, env: &Environment<'_>) -> Result<(), String> {
    match name.strip_prefix("__") {
        Some(rest) if env.derived && legal_name(rest) && !rest.contains('.') => Ok(()),
        _ => check_name(name, what),
    }
}

fn check_name(name: &str, what: &str) -> Result<(), String> {
    if legal_name(name) {
        Ok(())
    } else {
        Err(format!("invalid {what} name `{name}`"))
    }
}

/// The owners of the built-in constructors (`Bool.true`, `Nat.succ`,
/// `List.cons`, `Option.some`, `Result.ok`, `Prod.mk`). A local reference to
/// such a constructor has no module, so in language 1.2 no declaration may
/// take one of these names and make the reference ambiguous.
pub(crate) const BUILTIN_CONSTRUCTOR_OWNERS: [&str; 8] = [
    "Bool",
    "ContractViolation",
    "List",
    "Nat",
    "Option",
    "Prod",
    "ReasoningFailure",
    "Result",
];

fn check_declaration_name(name: &str, env: &Environment<'_>) -> Result<(), String> {
    check_name(name, "declaration")?;
    if env.language_1_2 && BUILTIN_CONSTRUCTOR_OWNERS.contains(&name) {
        return Err(format!(
            "declaration name `{name}` is reserved for the built-in type whose constructors it would shadow"
        ));
    }
    // The generated Lean refers to these namespaces without qualification
    // inside the module's own, so a declaration named like one, or below
    // one, would capture the reference and leave a file Lean cannot accept.
    if env.language_1_2
        && BACKEND_BARE_NAMES.iter().any(|bare| {
            name == *bare
                || name
                    .strip_prefix(*bare)
                    .is_some_and(|rest| rest.starts_with('.'))
        })
    {
        return Err(format!(
            "declaration name `{name}` is reserved: it is, or is below, a namespace the generated Lean refers to"
        ));
    }
    Ok(())
}

fn check_member(member: &MemberRef, env: &Environment<'_>) -> Result<(), String> {
    check_name(&member.name, "member")?;
    if let Some(module) = &member.module {
        check_name(module, "module")?;
        if !env.imports.contains(module) {
            return Err(format!(
                "forward or unavailable module reference `{module}`"
            ));
        }
    }
    Ok(())
}

fn check_type(ty: &SemanticType, env: &Environment<'_>) -> Result<(), String> {
    match ty {
        SemanticType::Type
        | SemanticType::Parameter { .. }
        | SemanticType::Nat
        | SemanticType::Bool
        | SemanticType::Prop
        | SemanticType::Unit
        | SemanticType::Int
        | SemanticType::Int8
        | SemanticType::Int16
        | SemanticType::Int32
        | SemanticType::Int64
        | SemanticType::UInt8
        | SemanticType::UInt16
        | SemanticType::UInt32
        | SemanticType::UInt64
        | SemanticType::String
        | SemanticType::Bytes
        | SemanticType::Ordering
        | SemanticType::ContractViolation
        | SemanticType::ReasoningFailure => Ok(()),
        SemanticType::List { element } | SemanticType::Option { value: element } => {
            check_type(element, env)
        }
        SemanticType::Result { ok, error } => {
            check_type(ok, env)?;
            check_type(error, env)
        }
        SemanticType::Product { left, right } => {
            if !env.language_1_2 {
                return Err(
                    "`product type` is a language-1.2 construct; language 1.1 rejects it"
                        .to_owned(),
                );
            }
            check_type(left, env)?;
            check_type(right, env)
        }
        SemanticType::Function { parameters, result } => {
            if !env.language_1_2 {
                return Err(
                    "`function type` is a language-1.2 construct; language 1.1 rejects it"
                        .to_owned(),
                );
            }
            if parameters.is_empty() {
                return Err("a function type has at least one parameter".to_owned());
            }
            for parameter in parameters {
                check_type(parameter, env)?;
            }
            check_type(result, env)
        }
        SemanticType::Map { key, value } => {
            require_language_1_2(env, "map type")?;
            check_ordered_key(key)?;
            check_type(key, env)?;
            check_type(value, env)
        }
        SemanticType::Set { element } => {
            require_language_1_2(env, "set type")?;
            check_ordered_key(element)?;
            check_type(element, env)
        }
        SemanticType::Named { member, arguments } => {
            check_member(member, env)?;
            for argument in arguments {
                check_type(argument, env)?;
            }
            let info = type_info(member, env)
                .ok_or_else(|| format!("forward or missing type `{}`", member_key(member)))?;
            if info.parameters != arguments.len() {
                return Err(format!(
                    "type `{}` expects {} argument(s), received {}",
                    member_key(member),
                    info.parameters,
                    arguments.len()
                ));
            }
            Ok(())
        }
    }
}

fn check_type_parameters(ty: &SemanticType, allowed: &BTreeSet<String>) -> Result<(), String> {
    match ty {
        SemanticType::Parameter { name } => {
            check_name(name, "type parameter")?;
            if allowed.contains(name) {
                Ok(())
            } else {
                Err(format!("unbound type parameter `{name}`"))
            }
        }
        SemanticType::List { element } | SemanticType::Option { value: element } => {
            check_type_parameters(element, allowed)
        }
        SemanticType::Result { ok, error }
        | SemanticType::Product {
            left: ok,
            right: error,
        } => {
            check_type_parameters(ok, allowed)?;
            check_type_parameters(error, allowed)
        }
        SemanticType::Function { parameters, result } => {
            for parameter in parameters {
                check_type_parameters(parameter, allowed)?;
            }
            check_type_parameters(result, allowed)
        }
        SemanticType::Map { key, value } => {
            check_type_parameters(key, allowed)?;
            check_type_parameters(value, allowed)
        }
        SemanticType::Set { element } => check_type_parameters(element, allowed),
        SemanticType::Named { arguments, .. } => {
            for argument in arguments {
                check_type_parameters(argument, allowed)?;
            }
            Ok(())
        }
        SemanticType::Type
        | SemanticType::Nat
        | SemanticType::Bool
        | SemanticType::Prop
        | SemanticType::Unit
        | SemanticType::Int
        | SemanticType::Int8
        | SemanticType::Int16
        | SemanticType::Int32
        | SemanticType::Int64
        | SemanticType::UInt8
        | SemanticType::UInt16
        | SemanticType::UInt32
        | SemanticType::UInt64
        | SemanticType::String
        | SemanticType::Bytes
        | SemanticType::Ordering
        | SemanticType::ContractViolation
        | SemanticType::ReasoningFailure => Ok(()),
    }
}

/// §17.12: in language 1.2 a type parameter is not also a value binder of
/// the same declaration, which it would capture in the generated binder.
/// Every module-wide spelling it could capture (a declaration, a built-in
/// name, the module prefix) is refused by `check_binder_hygiene` first.
fn check_type_parameter_spelling(
    type_parameters: &[String],
    binders: &BTreeSet<String>,
    env: &Environment<'_>,
) -> Result<(), String> {
    if !env.language_1_2 {
        return Ok(());
    }
    for parameter in type_parameters {
        if binders.contains(parameter) {
            return Err(format!(
                "type parameter `{parameter}` is also bound as a value in the same declaration"
            ));
        }
    }
    Ok(())
}

/// Every value name a term binds: lambda parameters, `let` and quantifier
/// binders, and match pattern binders.
fn bound_names(term: &SemanticTerm, out: &mut BTreeSet<String>) {
    visit_terms(term, &mut |node| match node {
        SemanticTerm::Lambda { parameters, .. } => {
            out.extend(parameters.iter().map(|parameter| parameter.name.clone()));
        }
        SemanticTerm::Let { binder, .. } | SemanticTerm::Forall { binder, .. } => {
            out.insert(binder.name.clone());
        }
        SemanticTerm::Match { branches, .. } => {
            for branch in branches {
                out.extend(branch.binders.iter().cloned());
            }
        }
        _ => {}
    });
}

/// A language-1.2 explicit type argument instantiates a `(T : Type)` binder,
/// so it can never be the universe `Type` itself, at any depth.
fn check_type_argument(argument: &SemanticType, env: &Environment<'_>) -> Result<(), String> {
    check_type(argument, env)?;
    if env.language_1_2 && mentions_universe(argument) {
        return Err(
            "an explicit type argument cannot be the universe `Type`; a type parameter ranges over `Type`"
                .to_owned(),
        );
    }
    Ok(())
}

fn mentions_universe(ty: &SemanticType) -> bool {
    match ty {
        SemanticType::Type => true,
        SemanticType::Option { value: inner } | SemanticType::List { element: inner } => {
            mentions_universe(inner)
        }
        SemanticType::Result { ok, error }
        | SemanticType::Product {
            left: ok,
            right: error,
        } => mentions_universe(ok) || mentions_universe(error),
        SemanticType::Named { arguments, .. } => arguments.iter().any(mentions_universe),
        SemanticType::Function { parameters, result } => {
            parameters.iter().any(mentions_universe) || mentions_universe(result)
        }
        SemanticType::Map { key, value } => mentions_universe(key) || mentions_universe(value),
        SemanticType::Set { element } => mentions_universe(element),
        SemanticType::Parameter { .. }
        | SemanticType::Nat
        | SemanticType::Bool
        | SemanticType::Prop
        | SemanticType::Unit
        | SemanticType::Int
        | SemanticType::Int8
        | SemanticType::Int16
        | SemanticType::Int32
        | SemanticType::Int64
        | SemanticType::UInt8
        | SemanticType::UInt16
        | SemanticType::UInt32
        | SemanticType::UInt64
        | SemanticType::String
        | SemanticType::Bytes
        | SemanticType::Ordering
        | SemanticType::ContractViolation
        | SemanticType::ReasoningFailure => false,
    }
}

fn type_parameter_set(parameters: &[String]) -> Result<BTreeSet<String>, String> {
    let mut out = BTreeSet::new();
    for parameter in parameters {
        check_name(parameter, "type parameter")?;
        if !out.insert(parameter.clone()) {
            return Err(format!("duplicate type parameter `{parameter}`"));
        }
    }
    Ok(out)
}

fn is_structural_domain(ty: &SemanticType, env: &Environment<'_>) -> bool {
    match ty {
        SemanticType::Nat | SemanticType::List { .. } => true,
        SemanticType::Named { member, .. } => type_info(member, env)
            .is_some_and(|info| !info.constructors.is_empty() && info.fields.is_empty()),
        _ => false,
    }
}

fn integer_representation(ty: &SemanticType) -> Option<SemanticInteger> {
    Some(match ty {
        SemanticType::Int => SemanticInteger::Int,
        SemanticType::Int8 => SemanticInteger::Int8,
        SemanticType::Int16 => SemanticInteger::Int16,
        SemanticType::Int32 => SemanticInteger::Int32,
        SemanticType::Int64 => SemanticInteger::Int64,
        SemanticType::UInt8 => SemanticInteger::UInt8,
        SemanticType::UInt16 => SemanticInteger::UInt16,
        SemanticType::UInt32 => SemanticInteger::UInt32,
        SemanticType::UInt64 => SemanticInteger::UInt64,
        _ => return None,
    })
}

fn fixed_integer(ty: &SemanticType) -> bool {
    integer_representation(ty).is_some_and(|kind| !matches!(kind, SemanticInteger::Int))
}

fn signed_integer(ty: &SemanticType) -> bool {
    matches!(
        ty,
        SemanticType::Int
            | SemanticType::Int8
            | SemanticType::Int16
            | SemanticType::Int32
            | SemanticType::Int64
    )
}

fn check_parameters(
    parameters: &[SemanticParameter],
    env: &Environment<'_>,
    type_parameters: &BTreeSet<String>,
) -> Result<BTreeSet<String>, String> {
    let mut names = BTreeSet::new();
    for parameter in parameters {
        check_binder(&parameter.name, "parameter", env)?;
        check_type(&parameter.r#type, env)?;
        check_type_parameters(&parameter.r#type, type_parameters)?;
        if !names.insert(parameter.name.clone()) {
            return Err(format!("duplicate parameter `{}`", parameter.name));
        }
    }
    Ok(names)
}

fn typed_locals(parameters: &[SemanticParameter]) -> BTreeMap<String, SemanticType> {
    parameters
        .iter()
        .map(|parameter| (parameter.name.clone(), parameter.r#type.clone()))
        .collect()
}

fn check_assignments(
    assignments: &[SemanticAssignment],
    expected: &[String],
    locals: &BTreeSet<String>,
    env: &Environment<'_>,
    recursion: Option<(&str, usize, &str)>,
    smaller: &BTreeSet<String>,
) -> Result<(), String> {
    let observed: Vec<String> = assignments.iter().map(|row| row.field.clone()).collect();
    if observed != expected {
        return Err(format!(
            "fields are not the exact ordered set: expected {expected:?}, observed {observed:?}"
        ));
    }
    for assignment in assignments {
        check_name(&assignment.field, "field")?;
        check_term(&assignment.value, locals, env, recursion, smaller)?;
    }
    Ok(())
}

fn constructor_arity(member: &MemberRef, env: &Environment<'_>) -> Option<usize> {
    if member.module.is_none() {
        match member.name.as_str() {
            "Bool.false" | "Bool.true" => return Some(0),
            "Option.none" | "Result.error" | "Result.ok" => {
                // Option.some and both Result constructors are polymorphic;
                // their exact signatures are checked by constructor_signature.
                return Some(usize::from(member.name != "Option.none"));
            }
            "Option.some" => return Some(1),
            name if env.language_1_2 && model::VIOLATIONS.contains(&name) => return Some(0),
            name if env.language_1_2 && reasoning::FAILURES.contains(&name) => return Some(0),
            _ => {}
        }
    }
    env.types.iter().find_map(|(owner, info)| {
        (member_from_key(owner).module == member.module)
            .then(|| info.constructors.get(&member.name).copied())
            .flatten()
    })
}

/// Where a constructor field mentions a member of the inductive group being
/// declared (§17.12 positivity).
#[derive(Clone, Copy, PartialEq, Eq)]
enum Occurrence {
    /// The field does not mention the group.
    Absent,
    /// The field is exactly one group member applied to the declared type
    /// parameters, in order.
    Direct,
    /// A group member occurs strictly positively under `List`, `Option`,
    /// `Result`, or a product.
    Nested,
}

fn mentions_group(ty: &SemanticType, group: &BTreeSet<String>) -> bool {
    match ty {
        SemanticType::Named { member, arguments } => {
            (member.module.is_none() && group.contains(&member.name))
                || arguments
                    .iter()
                    .any(|argument| mentions_group(argument, group))
        }
        SemanticType::List { element: inner } | SemanticType::Option { value: inner } => {
            mentions_group(inner, group)
        }
        SemanticType::Result { ok, error }
        | SemanticType::Product {
            left: ok,
            right: error,
        } => mentions_group(ok, group) || mentions_group(error, group),
        SemanticType::Function { parameters, result } => {
            parameters
                .iter()
                .any(|parameter| mentions_group(parameter, group))
                || mentions_group(result, group)
        }
        SemanticType::Map { key, value } => {
            mentions_group(key, group) || mentions_group(value, group)
        }
        SemanticType::Set { element } => mentions_group(element, group),
        SemanticType::Type
        | SemanticType::Parameter { .. }
        | SemanticType::Nat
        | SemanticType::Bool
        | SemanticType::Prop
        | SemanticType::Unit
        | SemanticType::Int
        | SemanticType::Int8
        | SemanticType::Int16
        | SemanticType::Int32
        | SemanticType::Int64
        | SemanticType::UInt8
        | SemanticType::UInt16
        | SemanticType::UInt32
        | SemanticType::UInt64
        | SemanticType::String
        | SemanticType::Bytes
        | SemanticType::Ordering
        | SemanticType::ContractViolation
        | SemanticType::ReasoningFailure => false,
    }
}

/// Classify one constructor field against the group. Recursive occurrences
/// must be uniform (the declared type parameters, in order) and strictly
/// positive: directly, or nested only under the closed containers. A group
/// member inside another document type's arguments is rejected, because the
/// positivity of that type's parameters is not part of its closed contract.
fn classify_occurrence(
    ty: &SemanticType,
    group: &BTreeSet<String>,
    uniform: &[SemanticType],
    top: bool,
    owner: &str,
) -> Result<Occurrence, String> {
    match ty {
        SemanticType::Named { member, arguments }
            if member.module.is_none() && group.contains(&member.name) =>
        {
            if arguments.as_slice() != uniform {
                return Err(format!(
                    "non-uniform recursive occurrence of `{}` in `{owner}`: its arguments must be the declared type parameters in order",
                    member.name
                ));
            }
            Ok(if top {
                Occurrence::Direct
            } else {
                Occurrence::Nested
            })
        }
        SemanticType::Named { member, arguments } => {
            if arguments
                .iter()
                .any(|argument| mentions_group(argument, group))
            {
                return Err(format!(
                    "positivity violation in `{owner}`: a recursive occurrence inside document type `{}` is not permitted",
                    member_key(member)
                ));
            }
            Ok(Occurrence::Absent)
        }
        SemanticType::Function { .. } | SemanticType::Map { .. } | SemanticType::Set { .. } => {
            if mentions_group(ty, group) {
                return Err(format!(
                    "positivity violation in `{owner}`: a recursive occurrence under a function, map, or set type is not permitted"
                ));
            }
            Ok(Occurrence::Absent)
        }
        SemanticType::List { element: inner } | SemanticType::Option { value: inner } => Ok(
            match classify_occurrence(inner, group, uniform, false, owner)? {
                Occurrence::Absent => Occurrence::Absent,
                Occurrence::Direct | Occurrence::Nested => Occurrence::Nested,
            },
        ),
        SemanticType::Result { ok, error }
        | SemanticType::Product {
            left: ok,
            right: error,
        } => {
            let left = classify_occurrence(ok, group, uniform, false, owner)?;
            let right = classify_occurrence(error, group, uniform, false, owner)?;
            Ok(
                if left == Occurrence::Absent && right == Occurrence::Absent {
                    Occurrence::Absent
                } else {
                    Occurrence::Nested
                },
            )
        }
        SemanticType::Type
        | SemanticType::Parameter { .. }
        | SemanticType::Nat
        | SemanticType::Bool
        | SemanticType::Prop
        | SemanticType::Unit
        | SemanticType::Int
        | SemanticType::Int8
        | SemanticType::Int16
        | SemanticType::Int32
        | SemanticType::Int64
        | SemanticType::UInt8
        | SemanticType::UInt16
        | SemanticType::UInt32
        | SemanticType::UInt64
        | SemanticType::String
        | SemanticType::Bytes
        | SemanticType::Ordering
        | SemanticType::ContractViolation
        | SemanticType::ReasoningFailure => Ok(Occurrence::Absent),
    }
}

/// Whether a value of `ty` can be built from the group members already
/// known to be inhabited. Containers that have an empty value (`List`,
/// `Option`) are always inhabited; every type that is not a group member
/// was itself accepted as inhabited.
fn constructible(
    ty: &SemanticType,
    group: &BTreeSet<String>,
    inhabited: &BTreeSet<String>,
) -> bool {
    match ty {
        SemanticType::Named { member, .. }
            if member.module.is_none() && group.contains(&member.name) =>
        {
            inhabited.contains(&member.name)
        }
        SemanticType::Result { ok, error } => {
            constructible(ok, group, inhabited) || constructible(error, group, inhabited)
        }
        SemanticType::Product { left, right } => {
            constructible(left, group, inhabited) && constructible(right, group, inhabited)
        }
        SemanticType::Function { result, .. } => constructible(result, group, inhabited),
        SemanticType::Map { .. } | SemanticType::Set { .. } => true,
        SemanticType::List { .. }
        | SemanticType::Option { .. }
        | SemanticType::Named { .. }
        | SemanticType::Type
        | SemanticType::Parameter { .. }
        | SemanticType::Nat
        | SemanticType::Bool
        | SemanticType::Prop
        | SemanticType::Unit
        | SemanticType::Int
        | SemanticType::Int8
        | SemanticType::Int16
        | SemanticType::Int32
        | SemanticType::Int64
        | SemanticType::UInt8
        | SemanticType::UInt16
        | SemanticType::UInt32
        | SemanticType::UInt64
        | SemanticType::String
        | SemanticType::Bytes
        | SemanticType::Ordering
        | SemanticType::ContractViolation
        | SemanticType::ReasoningFailure => true,
    }
}

/// One member of a language-1.2 inductive group.
struct InductiveRow<'a> {
    name: &'a str,
    type_parameters: &'a [String],
    parameters: &'a [SemanticParameter],
    constructors: &'a [SemanticConstructor],
}

/// The structurally-smaller flags of one already-validated inductive group
/// and whether it is nested or mutual; shared by local and imported data.
fn recursion_flags(
    rows: &[InductiveRow<'_>],
) -> BTreeMap<String, (BTreeMap<String, Vec<bool>>, bool)> {
    let group: BTreeSet<String> = rows.iter().map(|row| row.name.to_owned()).collect();
    let mut out = BTreeMap::new();
    for row in rows {
        let uniform: Vec<SemanticType> = row
            .type_parameters
            .iter()
            .map(|name| SemanticType::Parameter { name: name.clone() })
            .collect();
        let mut nested = rows.len() > 1;
        let mut flags = BTreeMap::new();
        for constructor in row.constructors {
            let mut direct = Vec::new();
            for field in &constructor.fields {
                let occurrence = classify_occurrence(field, &group, &uniform, true, row.name)
                    .unwrap_or(Occurrence::Absent);
                nested |= occurrence == Occurrence::Nested;
                direct.push(
                    occurrence == Occurrence::Direct
                        && matches!(field, SemanticType::Named { member, .. } if member.name == row.name),
                );
            }
            flags.insert(format!("{}.{}", row.name, constructor.name), direct);
        }
        out.insert(row.name.to_owned(), (flags, nested));
    }
    out
}

/// Validate and register one language-1.2 inductive group (a standalone
/// inductive is a group of one): uniform, strictly positive recursion and a
/// buildable base case for every member, all checked before any backend.
fn register_inductive_group(
    rows: &[InductiveRow<'_>],
    label: Option<&str>,
    env: &mut Environment<'_>,
    generated_names: &mut BTreeSet<String>,
) -> Result<(), String> {
    let first = &rows[0];
    if let Some(label) = label {
        check_name(label, "mutual group")?;
        if rows.len() < 2 {
            return Err(format!(
                "mutual group `{label}` has one member; a standalone inductive omits `mutual`"
            ));
        }
    }
    check_type_parameter_spelling(
        first.type_parameters,
        &rows.iter().map(|row| row.name.to_owned()).collect(),
        env,
    )?;
    let type_parameter_set = type_parameter_set(first.type_parameters)?;
    let mut group = BTreeSet::new();
    for row in rows {
        if !group.insert(row.name.to_owned()) {
            return Err(format!("duplicate generated name `{}`", row.name));
        }
        if row.type_parameters != first.type_parameters {
            return Err(format!(
                "mutual group `{}` members must declare identical type parameters; `{}` differs from `{}`",
                label.unwrap_or(first.name),
                row.name,
                first.name
            ));
        }
        if !row.parameters.is_empty() {
            return Err(format!(
                "inductive `{}` value parameters are not part of a finite data declaration",
                row.name
            ));
        }
        if row.constructors.is_empty() {
            return Err(format!("inductive `{}` has no constructors", row.name));
        }
    }
    // Every member is visible to every constructor field of the group, and
    // to nothing declared earlier: a forward reference outside the group
    // still fails in `check_type`.
    for row in rows {
        env.types.insert(
            row.name.to_owned(),
            TypeInfo {
                parameters: row.type_parameters.len(),
                type_parameters: row.type_parameters.to_vec(),
                constructors: row
                    .constructors
                    .iter()
                    .map(|constructor| {
                        (
                            format!("{}.{}", row.name, constructor.name),
                            constructor.fields.len(),
                        )
                    })
                    .collect(),
                constructor_types: row
                    .constructors
                    .iter()
                    .map(|constructor| {
                        (
                            format!("{}.{}", row.name, constructor.name),
                            constructor.fields.clone(),
                        )
                    })
                    .collect(),
                ..TypeInfo::default()
            },
        );
    }
    let uniform: Vec<SemanticType> = first
        .type_parameters
        .iter()
        .map(|name| SemanticType::Parameter { name: name.clone() })
        .collect();
    for row in rows {
        for constructor in row.constructors {
            check_name(&constructor.name, "constructor")?;
            let full = format!("{}.{}", row.name, constructor.name);
            if !generated_names.insert(full.clone()) {
                return Err(format!("duplicate generated name `{full}`"));
            }
            for field in &constructor.fields {
                check_type(field, env)?;
                check_type_parameters(field, &type_parameter_set)?;
                classify_occurrence(field, &group, &uniform, true, &full)?;
            }
        }
    }
    let mut inhabited = BTreeSet::new();
    loop {
        let before = inhabited.len();
        for row in rows {
            if row.constructors.iter().any(|constructor| {
                constructor
                    .fields
                    .iter()
                    .all(|field| constructible(field, &group, &inhabited))
            }) {
                inhabited.insert(row.name.to_owned());
            }
        }
        if inhabited.len() == before {
            break;
        }
    }
    if let Some(empty) = rows.iter().find(|row| !inhabited.contains(row.name)) {
        return Err(format!(
            "uninhabited recursive cycle: no constructor of inductive `{}` can be built without an existing value of its group",
            empty.name
        ));
    }
    let members: Vec<String> = rows.iter().map(|row| row.name.to_owned()).collect();
    for (name, (direct_recursive, nested_or_mutual)) in recursion_flags(rows) {
        if let Some(info) = env.types.get_mut(&name) {
            info.direct_recursive = direct_recursive;
            info.nested_or_mutual = nested_or_mutual;
            info.group.clone_from(&members);
        }
    }
    Ok(())
}

/// The parameter and result types of a definition at explicit type
/// arguments.
fn instantiate(
    info: &FunctionInfo,
    type_arguments: &[SemanticType],
) -> (Vec<SemanticType>, SemanticType) {
    let map: BTreeMap<String, SemanticType> = info
        .type_parameters
        .iter()
        .cloned()
        .zip(type_arguments.iter().cloned())
        .collect();
    (
        info.parameters
            .iter()
            .map(|parameter| substitute_type(parameter, &map))
            .collect(),
        substitute_type(&info.result, &map),
    )
}

/// Every type written inside `term` mentions only declared type parameters.
fn check_term_type_parameters(term: &SemanticTerm, scope: &BTreeSet<String>) -> Result<(), String> {
    let mut outcome = Ok(());
    visit_terms(term, &mut |node| {
        term_types(node, &mut |ty| {
            if outcome.is_ok() {
                outcome = check_type_parameters(ty, scope);
            }
        });
    });
    outcome
}

/// The locals `term` uses that it does not bind itself.
fn free_locals(term: &SemanticTerm, bound: &mut BTreeSet<String>, out: &mut BTreeSet<String>) {
    let scoped = |names: &[String],
                  inner: &SemanticTerm,
                  bound: &mut BTreeSet<String>,
                  out: &mut BTreeSet<String>| {
        let added: Vec<String> = names
            .iter()
            .filter(|name| bound.insert((*name).clone()))
            .cloned()
            .collect();
        free_locals(inner, bound, out);
        for name in added {
            bound.remove(&name);
        }
    };
    match term {
        SemanticTerm::Var { name } => {
            if !bound.contains(name) {
                out.insert(name.clone());
            }
        }
        SemanticTerm::Forall { binder, body } => {
            scoped(std::slice::from_ref(&binder.name), body, bound, out);
        }
        SemanticTerm::Let {
            binder,
            value,
            body,
        } => {
            free_locals(value, bound, out);
            scoped(std::slice::from_ref(&binder.name), body, bound, out);
        }
        SemanticTerm::Lambda {
            parameters, body, ..
        } => {
            let names: Vec<String> = parameters
                .iter()
                .map(|parameter| parameter.name.clone())
                .collect();
            scoped(&names, body, bound, out);
        }
        SemanticTerm::Match {
            scrutinee,
            branches,
        } => {
            free_locals(scrutinee, bound, out);
            for branch in branches {
                scoped(&branch.binders, &branch.body, bound, out);
            }
        }
        _ => {
            // Every other node binds nothing: recurse into its immediate
            // subterms only (visit_terms would also descend past binders).
            for child in immediate_subterms(term) {
                free_locals(child, bound, out);
            }
        }
    }
}

/// The immediate subterms of a node that binds no local.
fn immediate_subterms(term: &SemanticTerm) -> Vec<&SemanticTerm> {
    match term {
        SemanticTerm::Var { .. }
        | SemanticTerm::Nat { .. }
        | SemanticTerm::Integer { .. }
        | SemanticTerm::String { .. }
        | SemanticTerm::Bytes { .. }
        | SemanticTerm::Bool { .. }
        | SemanticTerm::Unit
        | SemanticTerm::Nil { .. }
        | SemanticTerm::InstanceValue { .. }
        | SemanticTerm::FunctionRef { .. } => Vec::new(),
        SemanticTerm::Primitive { arguments, .. }
        | SemanticTerm::Constructor { arguments, .. }
        | SemanticTerm::Call { arguments, .. }
        | SemanticTerm::CheckedApply { arguments, .. } => arguments.iter().collect(),
        SemanticTerm::Record { fields, .. } => fields.iter().map(|field| &field.value).collect(),
        SemanticTerm::Cons { head, tail } => vec![head, tail],
        SemanticTerm::Eq { left, right }
        | SemanticTerm::Le { left, right }
        | SemanticTerm::Lt { left, right }
        | SemanticTerm::Add { left, right }
        | SemanticTerm::Beq { left, right }
        | SemanticTerm::Ble { left, right }
        | SemanticTerm::Blt { left, right }
        | SemanticTerm::And { left, right }
        | SemanticTerm::PropAnd { left, right }
        | SemanticTerm::Or { left, right }
        | SemanticTerm::Iff { left, right }
        | SemanticTerm::Pair { left, right } => vec![left, right],
        SemanticTerm::Implies {
            premise,
            conclusion,
        } => vec![premise, conclusion],
        SemanticTerm::Project { value, .. }
        | SemanticTerm::Not { value }
        | SemanticTerm::First { value }
        | SemanticTerm::Second { value } => vec![value],
        SemanticTerm::If {
            condition,
            then_value,
            else_value,
        } => vec![condition, then_value, else_value],
        SemanticTerm::Apply {
            function,
            arguments,
        } => std::iter::once(function.as_ref())
            .chain(arguments)
            .collect(),
        SemanticTerm::Forall { body, .. } | SemanticTerm::Lambda { body, .. } => vec![body],
        SemanticTerm::Let { value, body, .. } => vec![value, body],
        SemanticTerm::Match {
            scrutinee,
            branches,
        } => std::iter::once(scrutinee.as_ref())
            .chain(branches.iter().map(|branch| &branch.body))
            .collect(),
        SemanticTerm::MapLiteral { entries, .. } => entries
            .iter()
            .flat_map(|entry| [&entry.key, &entry.value])
            .collect(),
        SemanticTerm::SetLiteral { elements, .. } => elements.iter().collect(),
        SemanticTerm::GraphLiteral { nodes, edges, .. } => nodes
            .iter()
            .chain(edges.iter().flat_map(|edge| [&edge.source, &edge.target]))
            .collect(),
    }
}

/// Whether values of a type can hold a function: a function type itself, or
/// a container, product, or document type that can, through its type
/// arguments or through the field types of its declaration. A type argument
/// counts even where today's declaration does not store it, because the
/// argument is data the value carries by its type.
fn holds_function(
    ty: &SemanticType,
    env: &Environment<'_>,
    visiting: &mut BTreeSet<String>,
) -> bool {
    match ty {
        SemanticType::Function { .. } => true,
        SemanticType::Option { value: inner } | SemanticType::List { element: inner } => {
            holds_function(inner, env, visiting)
        }
        SemanticType::Result { ok, error }
        | SemanticType::Product {
            left: ok,
            right: error,
        } => holds_function(ok, env, visiting) || holds_function(error, env, visiting),
        SemanticType::Named { member, arguments } => {
            if arguments
                .iter()
                .any(|argument| holds_function(argument, env, visiting))
            {
                return true;
            }
            // A recursive or mutual declaration is visited once: a cycle
            // adds no field type that was not already examined.
            if !visiting.insert(member_key(member)) {
                return false;
            }
            type_info(member, env).is_some_and(|info| {
                info.field_types
                    .iter()
                    .chain(info.constructor_types.values().flatten())
                    .any(|field| holds_function(field, env, visiting))
            })
        }
        SemanticType::Map { key, value } => {
            holds_function(key, env, visiting) || holds_function(value, env, visiting)
        }
        SemanticType::Set { element } => holds_function(element, env, visiting),
        SemanticType::Type
        | SemanticType::Parameter { .. }
        | SemanticType::Nat
        | SemanticType::Bool
        | SemanticType::Prop
        | SemanticType::Unit
        | SemanticType::Int
        | SemanticType::Int8
        | SemanticType::Int16
        | SemanticType::Int32
        | SemanticType::Int64
        | SemanticType::UInt8
        | SemanticType::UInt16
        | SemanticType::UInt32
        | SemanticType::UInt64
        | SemanticType::String
        | SemanticType::Bytes
        | SemanticType::Ordering
        | SemanticType::ContractViolation
        | SemanticType::ReasoningFailure => false,
    }
}

/// §17.12 production eligibility of higher-order code: an `executable`
/// definition forms only second-class, non-escaping closures. A closure may
/// be passed directly to an executable definition's function parameter or
/// applied on the spot; it is never returned, stored in data, bound by `let`,
/// or reached through a projection, and every definition it calls is itself
/// executable. No type the definition states or writes in its body can hold
/// a function, so no data value in it carries a closure, and a function
/// parameter is used only where a closure may be.
fn check_executable(
    name: &str,
    parameters: &[SemanticParameter],
    result: &SemanticType,
    body: &SemanticTerm,
    env: &Environment<'_>,
) -> Result<(), String> {
    let holds = |ty: &SemanticType| holds_function(ty, env, &mut BTreeSet::new());
    if holds(result) {
        return Err(if matches!(result, SemanticType::Function { .. }) {
            format!("escaping closure: executable definition `{name}` returns a function")
        } else {
            format!(
                "escaping closure: executable definition `{name}` returns a value of type {result}, which holds a function"
            )
        });
    }
    let mut function_locals = BTreeSet::new();
    for parameter in parameters {
        if let SemanticType::Function {
            parameters: inner,
            result: inner_result,
        } = &parameter.r#type
        {
            if inner.iter().any(holds) || holds(inner_result) {
                return Err(format!(
                    "executable definition `{name}` parameter `{}` is a higher-order function of functions",
                    parameter.name
                ));
            }
            function_locals.insert(parameter.name.clone());
        } else if holds(&parameter.r#type) {
            return Err(format!(
                "escaping closure: executable definition `{name}` parameter `{}` stores a function in data",
                parameter.name
            ));
        }
    }
    let own: Vec<SemanticType> = parameters
        .iter()
        .map(|parameter| parameter.r#type.clone())
        .collect();
    let scope = ExecutableScope {
        name,
        own: &own,
        function_locals: &function_locals,
    };
    check_executable_term(&scope, body, false, env)
}

/// What the escape analysis of one executable definition knows.
struct ExecutableScope<'a> {
    name: &'a str,
    /// The definition's own parameter types, for its recursive calls.
    own: &'a [SemanticType],
    /// Its function-typed parameters, the only function-valued locals.
    function_locals: &'a BTreeSet<String>,
}

fn check_executable_term(
    scope: &ExecutableScope<'_>,
    term: &SemanticTerm,
    closure_position: bool,
    env: &Environment<'_>,
) -> Result<(), String> {
    let name = scope.name;
    if let SemanticTerm::Let { binder, .. } = term {
        if holds_function(&binder.r#type, env, &mut BTreeSet::new()) {
            return Err(format!(
                "escaping closure in executable definition `{name}`: a function is bound by let"
            ));
        }
    }
    let mut written = None;
    term_types(term, &mut |ty| {
        if written.is_none() && holds_function(ty, env, &mut BTreeSet::new()) {
            written = Some(ty.clone());
        }
    });
    if let Some(ty) = written {
        return Err(format!(
            "escaping closure in executable definition `{name}`: the type {ty} written in its body holds a function"
        ));
    }
    match term {
        SemanticTerm::Var { name: local } => {
            if scope.function_locals.contains(local) && !closure_position {
                return Err(format!(
                    "escaping closure in executable definition `{name}`: the function `{local}` is used as a value"
                ));
            }
            Ok(())
        }
        SemanticTerm::Lambda { body, .. } => {
            if !closure_position {
                return Err(format!(
                    "escaping closure in executable definition `{name}`: a lambda may only be passed directly to an executable function parameter or applied"
                ));
            }
            check_executable_term(scope, body, false, env)
        }
        SemanticTerm::FunctionRef { function, .. } => {
            if !closure_position {
                return Err(format!(
                    "escaping closure in executable definition `{name}`: a function reference may only be passed directly to an executable function parameter or applied"
                ));
            }
            if !function_info(function, env).is_some_and(|info| info.executable) {
                return Err(format!(
                    "executable definition `{name}` references non-executable `{}`",
                    member_key(function)
                ));
            }
            Ok(())
        }
        SemanticTerm::Apply {
            function,
            arguments,
        } => {
            match function.as_ref() {
                SemanticTerm::Var { .. }
                | SemanticTerm::Lambda { .. }
                | SemanticTerm::FunctionRef { .. } => {
                    check_executable_term(scope, function, true, env)?;
                }
                _ => {
                    return Err(format!(
                        "executable definition `{name}` applies a function value that is not a parameter, lambda, or function reference"
                    ));
                }
            }
            for argument in arguments {
                check_executable_term(scope, argument, false, env)?;
            }
            Ok(())
        }
        SemanticTerm::Call {
            function,
            arguments,
            ..
        } => {
            let recursive = function.module.is_none() && function.name == name;
            let info = function_info(function, env);
            if !recursive && !info.is_some_and(|info| info.executable) {
                return Err(format!(
                    "executable definition `{name}` calls non-executable `{}`",
                    member_key(function)
                ));
            }
            let callee: &[SemanticType] = if recursive {
                scope.own
            } else {
                info.map_or(&[], |info| info.parameters.as_slice())
            };
            for (index, argument) in arguments.iter().enumerate() {
                let function_parameter =
                    matches!(callee.get(index), Some(SemanticType::Function { .. }));
                check_executable_term(scope, argument, function_parameter, env)?;
            }
            Ok(())
        }
        SemanticTerm::Primitive {
            operation,
            arguments,
            ..
        } => {
            for (index, argument) in arguments.iter().enumerate() {
                check_executable_term(
                    scope,
                    argument,
                    operation.function_argument() == Some(index),
                    env,
                )?;
            }
            Ok(())
        }
        _ => {
            for child in immediate_subterms(term) {
                check_executable_term(scope, child, false, env)?;
            }
            Ok(())
        }
    }
}

/// The recursive family a mutual definition group decreases over (§17.12):
/// the naturals, lists of one element type, or one inductive group with its
/// `List` and `Option` containers.
#[derive(Clone, Debug, PartialEq, Eq)]
enum Family {
    Nat,
    List(SemanticType),
    /// An inductive group at one instantiation of its type parameters.
    Group(BTreeSet<String>, Vec<SemanticType>),
}

/// The containers (`List`, `Option`) under which a group's constructors
/// actually nest a member of the group: exactly those a mutual definition
/// may recurse through.
fn group_containers(group: &BTreeSet<String>, env: &Environment<'_>) -> (bool, bool) {
    let in_group = |ty: &SemanticType| matches!(ty, SemanticType::Named { member, .. } if group.contains(&member_key(member)));
    let (mut list, mut option) = (false, false);
    for key in group {
        let Some(info) = env.types.get(key) else {
            continue;
        };
        for field in info.constructor_types.values().flatten() {
            match field {
                SemanticType::List { element } if in_group(element) => list = true,
                SemanticType::Option { value } if in_group(value) => option = true,
                _ => {}
            }
        }
    }
    (list, option)
}

fn group_key_of(member: &MemberRef, env: &Environment<'_>) -> Option<BTreeSet<String>> {
    let info = type_info(member, env)?;
    if info.constructors.is_empty() || !info.fields.is_empty() {
        return None;
    }
    Some(info.group.iter().cloned().collect())
}

/// The family of one decreasing parameter type.
fn family_of(ty: &SemanticType, env: &Environment<'_>) -> Option<Family> {
    match ty {
        SemanticType::Nat => Some(Family::Nat),
        SemanticType::Named { member, arguments } => {
            group_key_of(member, env).map(|group| Family::Group(group, arguments.clone()))
        }
        SemanticType::List { element } | SemanticType::Option { value: element } => {
            if let SemanticType::Named { member, arguments } = element.as_ref() {
                if let Some(group) = group_key_of(member, env) {
                    let (list, option) = group_containers(&group, env);
                    let nested = match ty {
                        SemanticType::List { .. } => list,
                        _ => option,
                    };
                    return nested.then(|| Family::Group(group, arguments.clone()));
                }
            }
            match ty {
                SemanticType::List { element } => Some(Family::List(element.as_ref().clone())),
                _ => None,
            }
        }
        _ => None,
    }
}

/// Whether a pattern binder of type `ty` belongs to `family` and is
/// therefore a structurally smaller subvalue of the matched argument.
fn in_family(ty: &SemanticType, family: &Family, env: &Environment<'_>) -> bool {
    match (family, ty) {
        (Family::Nat, SemanticType::Nat) => true,
        (Family::List(element), SemanticType::List { element: observed }) => {
            observed.as_ref() == element
        }
        (Family::Group(..), _) => family_of(ty, env).as_ref() == Some(family),
        _ => false,
    }
}

/// Check one definition: signature, scope, body, recursion, typing, and
/// production eligibility. Shared by standalone definitions and members of
/// a mutual group (whose group context is already in `env`).
#[allow(clippy::too_many_lines)]
fn check_definition(
    declaration: &SemanticDeclaration,
    env: &mut Environment<'_>,
) -> Result<(), String> {
    let SemanticDeclaration::Definition {
        name,
        type_parameters,
        parameters,
        result,
        recursive_argument,
        body,
        axioms,
        executable,
        mutual,
        termination,
        production,
    } = declaration
    else {
        return Ok(());
    };
    if axioms.windows(2).any(|pair| pair[0] >= pair[1])
        || axioms.iter().any(|axiom| !legal_name(axiom))
    {
        return Err(format!(
            "definition `{name}` axiom policy is not sorted, unique, and qualified"
        ));
    }
    if !type_parameters.is_empty() {
        require_language_1_2(env, "definition type parameters")?;
    }
    if *executable {
        require_language_1_2(env, "executable definition")?;
    }
    if mutual.is_some() {
        require_language_1_2(env, "mutual definition group")?;
    }
    if let Some(production) = production {
        // The declaration's shape is checked here; whether the root is
        // eligible is decided over its linked closure (§17.13, LLT4005).
        require_language_1_2(env, "production root")?;
        crate::production::check_declaration(name, &production.targets, &production.effects)?;
    }
    if termination.is_some() {
        require_language_1_2(env, "well-founded termination evidence")?;
        if recursive_argument.is_some() {
            return Err(format!(
                "definition `{name}` declares both structural and well-founded recursion"
            ));
        }
    }
    let scope = type_parameter_set(type_parameters)?;
    let mut binders: BTreeSet<String> = parameters
        .iter()
        .map(|parameter| parameter.name.clone())
        .collect();
    bound_names(body, &mut binders);
    check_type_parameter_spelling(type_parameters, &binders, env)?;
    let locals = check_parameters(parameters, env, &scope)?;
    check_type(result, env)?;
    check_type_parameters(result, &scope)?;
    check_term_type_parameters(body, &scope)?;
    env.current_type_parameters.clone_from(type_parameters);
    let info = FunctionInfo {
        type_parameters: type_parameters.clone(),
        parameters: parameters
            .iter()
            .map(|parameter| parameter.r#type.clone())
            .collect(),
        result: result.clone(),
        executable: *executable,
    };
    let recursion = if let Some(argument) = recursive_argument {
        let index = parameters
            .iter()
            .position(|parameter| &parameter.name == argument)
            .ok_or_else(|| {
                format!("definition `{name}` names a missing decreasing argument `{argument}`")
            })?;
        if env.recursive_family.is_none() && !is_structural_domain(&parameters[index].r#type, env) {
            return Err(format!(
                "definition `{name}` decreasing argument `{argument}` is not Nat, List, or a document inductive"
            ));
        }
        // §17.12: one standalone function recurses structurally only over a
        // self-recursive type; a nested or mutual group has no single
        // structural eliminator for it, exactly as for induction. A mutual
        // definition group recurses over such a type as one family.
        if env.recursive_family.is_none() {
            if let SemanticType::Named { member, .. } = &parameters[index].r#type {
                if type_info(member, env).is_some_and(|info| info.nested_or_mutual) {
                    return Err(format!(
                        "definition `{name}` decreases on `{argument}` of the nested or mutual type `{}`; standalone structural recursion requires a self-recursive inductive",
                        member_key(member)
                    ));
                }
            }
        }
        if !matches!(
            body,
            SemanticTerm::Match { scrutinee, .. }
                if matches!(scrutinee.as_ref(), SemanticTerm::Var { name } if name == argument)
        ) {
            return Err(format!(
                "definition `{name}` is recursive but its body is not a top-level match on `{argument}`"
            ));
        }
        Some((name.as_str(), index, argument.as_str()))
    } else {
        None
    };
    // A standalone well-founded definition is a group of one; a mutual
    // group's members are registered by `check_definition_group`.
    let standalone = termination.is_some() && mutual.is_none();
    if let Some(termination) = termination {
        if standalone {
            env.well_founded_group.insert(
                name.clone(),
                (parameters.clone(), termination.measure.clone()),
            );
        }
        check_measure(name, &termination.measure, parameters, env)?;
        // Calls of a well-founded definition are ordinary typed calls; their
        // placement and evidence are checked below.
        env.functions.insert(name.to_owned(), info.clone());
    }
    let checked = check_term(body, &locals, env, recursion, &BTreeSet::new());
    if standalone && checked.is_err() {
        env.well_founded_group.clear();
    }
    checked?;
    env.functions.insert(name.to_owned(), info);
    require_type(
        infer_term(body, &typed_locals(parameters), env)?,
        result,
        &format!("definition `{name}` body"),
    )?;
    if let Some(termination) = termination {
        let outcome = check_well_founded(name, type_parameters, parameters, body, termination, env);
        if standalone {
            env.well_founded_group.clear();
        }
        outcome?;
    }
    if *executable {
        check_executable(name, parameters, result, body, env)?;
    }
    env.current_type_parameters.clear();
    Ok(())
}

/// Validate one contiguous mutual definition group (§17.12): at least two
/// structurally recursive members with identical type parameters, each
/// decreasing on a value of one recursive family, and every call between
/// members passing a structurally smaller family binder.
fn check_definition_group(
    rows: &[&SemanticDeclaration],
    label: &str,
    env: &mut Environment<'_>,
) -> Result<(), String> {
    check_name(label, "mutual group")?;
    if rows.len() < 2 {
        return Err(format!(
            "mutual group `{label}` has one member; a standalone definition omits `mutual`"
        ));
    }
    let well_founded = rows
        .iter()
        .filter(|row| {
            matches!(
                row,
                SemanticDeclaration::Definition {
                    termination: Some(_),
                    ..
                }
            )
        })
        .count();
    if well_founded == rows.len() {
        check_group_call_graph(rows, label)?;
        return check_well_founded_group(rows, label, env);
    }
    if well_founded > 0 {
        return Err(format!(
            "mutual group `{label}` mixes structural and well-founded members"
        ));
    }
    let mut members = BTreeMap::new();
    let mut decreasing_types = Vec::new();
    let mut first_type_parameters: Option<&Vec<String>> = None;
    for row in rows {
        let SemanticDeclaration::Definition {
            name,
            type_parameters,
            parameters,
            result,
            recursive_argument,
            executable,
            ..
        } = row
        else {
            continue;
        };
        if let Some(first) = first_type_parameters {
            if first != type_parameters {
                return Err(format!(
                    "mutual group `{label}` members must declare identical type parameters; `{name}` differs"
                ));
            }
        } else {
            first_type_parameters = Some(type_parameters);
        }
        let argument = recursive_argument.as_ref().ok_or_else(|| {
            format!("mutual group `{label}` member `{name}` names no decreasing argument")
        })?;
        let index = parameters
            .iter()
            .position(|parameter| &parameter.name == argument)
            .ok_or_else(|| {
                format!("definition `{name}` names a missing decreasing argument `{argument}`")
            })?;
        members.insert(name.clone(), index);
        decreasing_types.push(parameters[index].r#type.clone());
        // Every member is callable from every member's body.
        env.functions.insert(
            name.clone(),
            FunctionInfo {
                type_parameters: type_parameters.clone(),
                parameters: parameters
                    .iter()
                    .map(|parameter| parameter.r#type.clone())
                    .collect(),
                result: result.clone(),
                executable: *executable,
            },
        );
    }
    let mut family = None;
    for (row, ty) in rows.iter().zip(&decreasing_types) {
        let observed = family_of(ty, env).ok_or_else(|| {
            format!(
                "mutual group `{label}` member `{}` decreases on a type outside every recursive family",
                row.name()
            )
        })?;
        match &family {
            None => family = Some(observed),
            Some(expected) if *expected == observed => {}
            Some(_) => {
                return Err(format!(
                    "mutual group `{label}` members decrease on different recursive families"
                ));
            }
        }
    }
    // The call graph is checked once the members and their family are
    // known, so a group of the wrong shape reports its shape first.
    check_group_call_graph(rows, label)?;
    env.recursive_group = members;
    env.recursive_family = family;
    let mut outcome = Ok(());
    for (row, ty) in rows.iter().zip(decreasing_types) {
        env.current_decreasing_type = Some(ty);
        outcome = check_definition(row, env);
        if outcome.is_err() {
            break;
        }
    }
    env.recursive_group.clear();
    env.recursive_family = None;
    env.current_decreasing_type = None;
    outcome
}

/// §17.12: the recursion call graph of a mutual group is the calls and
/// references among its members. Every member calls into the group and the
/// graph is strongly connected: a member outside every cycle is not
/// mutually recursive, and Lean refuses its termination clause.
fn check_group_call_graph(rows: &[&SemanticDeclaration], label: &str) -> Result<(), String> {
    let members: BTreeSet<String> = rows.iter().map(|row| row.name().to_owned()).collect();
    let mut edges: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for row in rows {
        let SemanticDeclaration::Definition { name, body, .. } = row else {
            return Err(format!(
                "mutual group `{label}` member `{}` is not a definition",
                row.name()
            ));
        };
        let mut callees = BTreeSet::new();
        visit_terms(body, &mut |term| {
            if let SemanticTerm::Call { function, .. }
            | SemanticTerm::FunctionRef { function, .. } = term
            {
                if function.module.is_none() && members.contains(&function.name) {
                    callees.insert(function.name.clone());
                }
            }
        });
        if callees.is_empty() {
            return Err(format!(
                "mutual group `{label}` member `{name}` calls no member of its group"
            ));
        }
        edges.insert(name.clone(), callees);
    }
    let reach = |forward: bool| {
        let start = rows[0].name().to_owned();
        let mut seen = BTreeSet::from([start.clone()]);
        let mut pending = vec![start];
        while let Some(current) = pending.pop() {
            let next: Vec<String> = if forward {
                edges[&current].iter().cloned().collect()
            } else {
                edges
                    .iter()
                    .filter(|(_, callees)| callees.contains(&current))
                    .map(|(caller, _)| caller.clone())
                    .collect()
            };
            for member in next {
                if seen.insert(member.clone()) {
                    pending.push(member);
                }
            }
        }
        seen
    };
    for (forward, direction) in [(true, "reach"), (false, "be reached from")] {
        let seen = reach(forward);
        if let Some(missing) = members.iter().find(|member| !seen.contains(*member)) {
            return Err(format!(
                "mutual group `{label}` is not strongly connected: `{}` does not {direction} `{missing}`",
                rows[0].name()
            ));
        }
    }
    Ok(())
}

/// A mutual group of well-founded definitions (§17.12 rule 3): identical
/// type parameters, a measure per member, and one evidence theorem per call
/// site whose obligation compares the callee's measure at the call's
/// arguments with the caller's measure.
fn check_well_founded_group(
    rows: &[&SemanticDeclaration],
    label: &str,
    env: &mut Environment<'_>,
) -> Result<(), String> {
    let mut first_type_parameters: Option<&Vec<String>> = None;
    for row in rows {
        let SemanticDeclaration::Definition {
            name,
            type_parameters,
            parameters,
            result,
            executable,
            termination: Some(termination),
            ..
        } = row
        else {
            continue;
        };
        if let Some(first) = first_type_parameters {
            if first != type_parameters {
                return Err(format!(
                    "mutual group `{label}` members must declare identical type parameters; `{name}` differs"
                ));
            }
        } else {
            first_type_parameters = Some(type_parameters);
        }
        env.functions.insert(
            name.clone(),
            FunctionInfo {
                type_parameters: type_parameters.clone(),
                parameters: parameters
                    .iter()
                    .map(|parameter| parameter.r#type.clone())
                    .collect(),
                result: result.clone(),
                executable: *executable,
            },
        );
        env.well_founded_group.insert(
            name.clone(),
            (parameters.clone(), termination.measure.clone()),
        );
    }
    let mut outcome = Ok(());
    for row in rows {
        outcome = check_definition(row, env);
        if outcome.is_err() {
            break;
        }
    }
    env.well_founded_group.clear();
    outcome
}

/// A well-founded measure is a binder-free natural-number term over the
/// parameters that does not mention the definition it measures.
fn check_measure(
    name: &str,
    measure: &SemanticTerm,
    parameters: &[SemanticParameter],
    env: &Environment<'_>,
) -> Result<(), String> {
    let mut binder = false;
    let mut cyclic = false;
    visit_terms(measure, &mut |term| match term {
        SemanticTerm::Let { .. }
        | SemanticTerm::Forall { .. }
        | SemanticTerm::Lambda { .. }
        | SemanticTerm::Match { .. } => binder = true,
        SemanticTerm::Call { function, .. } | SemanticTerm::FunctionRef { function, .. } => {
            cyclic |= function.module.is_none()
                && (function.name == name || env.well_founded_group.contains_key(&function.name));
        }
        _ => {}
    });
    if cyclic {
        return Err(format!(
            "the measure of `{name}` refers to `{name}` itself or a member of its group (cyclic measure)"
        ));
    }
    if binder {
        return Err(format!(
            "the measure of `{name}` binds a local; a measure is binder-free"
        ));
    }
    let names: BTreeSet<String> = parameters
        .iter()
        .map(|parameter| parameter.name.clone())
        .collect();
    check_term(measure, &names, env, None, &BTreeSet::new())?;
    require_type(
        infer_term(measure, &typed_locals(parameters), env)?,
        &SemanticType::Nat,
        &format!("the measure of `{name}`"),
    )
}

/// One enclosing node of a recursive call site, outermost first.
#[derive(Clone, Copy)]
pub(crate) enum Step {
    /// The call lies in the `then` (`true`) or `else` (`false`) arm of the
    /// numbered `if`.
    If { node: usize, polarity: bool },
    /// The call lies in branch `branch` of the numbered `match`.
    Match { node: usize, branch: usize },
}

impl Step {
    /// The numbered node this step lies under.
    pub(crate) const fn node(self) -> usize {
        match self {
            Self::If { node, .. } | Self::Match { node, .. } => node,
        }
    }
}

/// A numbered `if` or `match` of a well-founded body.
pub(crate) enum PlannedNode<'a> {
    If {
        condition: &'a SemanticTerm,
    },
    Match {
        scrutinee: &'a SemanticTerm,
        branches: &'a [SemanticBranch],
    },
}

pub(crate) struct CallSite {
    /// The group member called.
    pub(crate) callee: String,
    pub(crate) path: Vec<Step>,
    pub(crate) arguments: Vec<SemanticTerm>,
}

/// The fixed hypothesis numbering and call-site order of a well-founded
/// body (§17.12): every `if` and `match` is numbered in this pre-order, and
/// both the evidence obligations and the Lean lowering read the same
/// numbering. `addresses` identifies each numbered node within the borrowed
/// body for the lowering; it is never dereferenced.
pub(crate) struct WellFoundedPlan<'a> {
    pub(crate) nodes: Vec<PlannedNode<'a>>,
    pub(crate) addresses: Vec<usize>,
    pub(crate) sites: Vec<CallSite>,
}

impl WellFoundedPlan<'_> {
    /// The match binders a call site is under, outermost first; they are
    /// quantified by its evidence after the definition's parameters.
    pub(crate) fn binders(&self, site: &CallSite) -> Vec<String> {
        let mut out = Vec::new();
        for step in &site.path {
            if let Step::Match { node, branch } = step {
                if let PlannedNode::Match { branches, .. } = &self.nodes[*node] {
                    out.extend(branches[*branch].binders.iter().cloned());
                }
            }
        }
        out
    }
}

fn contains_call(term: &SemanticTerm, group: &BTreeSet<String>) -> bool {
    let mut found = false;
    visit_terms(term, &mut |node| {
        if let SemanticTerm::Call { function, .. } | SemanticTerm::FunctionRef { function, .. } =
            node
        {
            found |= function.module.is_none() && group.contains(&function.name);
        }
    });
    found
}

/// Plan a well-founded body, rejecting a recursive call in any position
/// whose decrease obligation could not be stated over the parameters and
/// the binders of the matches enclosing it.
pub(crate) fn well_founded_plan<'a>(
    name: &str,
    group: &BTreeSet<String>,
    type_parameters: &[String],
    parameters: &[SemanticParameter],
    body: &'a SemanticTerm,
) -> Result<WellFoundedPlan<'a>, String> {
    struct Walk<'n> {
        name: &'n str,
        group: &'n BTreeSet<String>,
        own: Vec<SemanticType>,
        /// The locals an obligation may mention here: the parameters and the
        /// binders of every enclosing match.
        allowed: BTreeSet<String>,
        path: Vec<Step>,
    }
    fn mentions_only(term: &SemanticTerm, allowed: &BTreeSet<String>) -> Option<String> {
        let mut used = BTreeSet::new();
        free_locals(term, &mut BTreeSet::new(), &mut used);
        used.difference(allowed).next().cloned()
    }
    fn walk<'a>(
        term: &'a SemanticTerm,
        state: &mut Walk<'_>,
        plan: &mut WellFoundedPlan<'a>,
    ) -> Result<(), String> {
        let name = state.name;
        let group = state.group;
        match term {
            SemanticTerm::Call {
                function,
                type_arguments,
                arguments,
            } if function.module.is_none() && group.contains(&function.name) => {
                let callee = &function.name;
                if type_arguments.as_slice() != state.own {
                    return Err(format!(
                        "recursive call `{callee}` must pass its own type parameters in order; polymorphic recursion is not permitted"
                    ));
                }
                for argument in arguments {
                    if contains_call(argument, group) {
                        return Err(format!(
                            "a recursive call of `{callee}` is nested in another call's argument"
                        ));
                    }
                    if let Some(local) = mentions_only(argument, &state.allowed) {
                        return Err(format!(
                            "an argument of a recursive call of `{callee}` mentions `{local}`, which is neither a parameter nor an enclosing match binder"
                        ));
                    }
                }
                plan.sites.push(CallSite {
                    callee: callee.clone(),
                    path: state.path.clone(),
                    arguments: arguments.clone(),
                });
                Ok(())
            }
            SemanticTerm::If {
                condition,
                then_value,
                else_value,
            } => {
                let node = plan.nodes.len();
                plan.nodes.push(PlannedNode::If { condition });
                plan.addresses.push(std::ptr::from_ref(term) as usize);
                walk(condition, state, plan)?;
                if contains_call(then_value, group) || contains_call(else_value, group) {
                    if let Some(local) = mentions_only(condition, &state.allowed) {
                        return Err(format!(
                            "a condition enclosing a recursive call of `{name}` mentions `{local}`, which is neither a parameter nor an enclosing match binder"
                        ));
                    }
                }
                for (polarity, arm) in [(true, then_value), (false, else_value)] {
                    state.path.push(Step::If { node, polarity });
                    walk(arm, state, plan)?;
                    state.path.pop();
                }
                Ok(())
            }
            SemanticTerm::Match {
                scrutinee,
                branches,
            } => {
                let node = plan.nodes.len();
                plan.nodes.push(PlannedNode::Match {
                    scrutinee,
                    branches,
                });
                plan.addresses.push(std::ptr::from_ref(term) as usize);
                walk(scrutinee, state, plan)?;
                let encloses = branches
                    .iter()
                    .any(|branch| contains_call(&branch.body, group));
                if encloses {
                    if let Some(local) = mentions_only(scrutinee, &state.allowed) {
                        return Err(format!(
                            "a match enclosing a recursive call of `{name}` inspects `{local}`, which is neither a parameter nor an enclosing match binder"
                        ));
                    }
                }
                for (index, branch) in branches.iter().enumerate() {
                    let added: Vec<&String> = branch.binders.iter().collect();
                    if encloses {
                        for binder in &added {
                            if state.allowed.contains(*binder) {
                                return Err(format!(
                                    "match binder `{binder}` enclosing a recursive call of `{name}` repeats a parameter or an enclosing binder"
                                ));
                            }
                        }
                    }
                    for binder in &added {
                        state.allowed.insert((*binder).clone());
                    }
                    state.path.push(Step::Match {
                        node,
                        branch: index,
                    });
                    walk(&branch.body, state, plan)?;
                    state.path.pop();
                    for binder in &added {
                        state.allowed.remove(*binder);
                    }
                }
                Ok(())
            }
            SemanticTerm::Lambda { .. } | SemanticTerm::Forall { .. }
                if contains_call(term, group) =>
            {
                Err(format!(
                    "a recursive call of `{name}` under a lambda or quantifier has no stated decrease obligation"
                ))
            }
            _ => {
                for child in immediate_subterms(term) {
                    walk(child, state, plan)?;
                }
                Ok(())
            }
        }
    }
    let mut state = Walk {
        name,
        group,
        own: type_parameters
            .iter()
            .map(|parameter| SemanticType::Parameter {
                name: parameter.clone(),
            })
            .collect(),
        allowed: parameters
            .iter()
            .map(|parameter| parameter.name.clone())
            .collect(),
        path: Vec::new(),
    };
    let mut plan = WellFoundedPlan {
        nodes: Vec::new(),
        addresses: Vec::new(),
        sites: Vec::new(),
    };
    walk(body, &mut state, &mut plan)?;
    Ok(plan)
}

fn substitute_locals(term: &SemanticTerm, map: &BTreeMap<String, SemanticTerm>) -> SemanticTerm {
    if let SemanticTerm::Var { name } = term {
        if let Some(value) = map.get(name) {
            return value.clone();
        }
        return term.clone();
    }
    // The measure is binder-free, so substitution never meets a binder.
    let mut value = serde_json::to_value(term).expect("semantic term serializes");
    fn rewrite(value: &mut serde_json::Value, map: &BTreeMap<String, SemanticTerm>) {
        match value {
            serde_json::Value::Array(items) => items.iter_mut().for_each(|item| rewrite(item, map)),
            serde_json::Value::Object(object) => {
                if object.get("kind").and_then(serde_json::Value::as_str) == Some("var") {
                    if let Some(serde_json::Value::String(name)) = object.get("name") {
                        if let Some(replacement) = map.get(name) {
                            *value = serde_json::to_value(replacement).expect("term serializes");
                            return;
                        }
                    }
                }
                object.values_mut().for_each(|child| rewrite(child, map));
            }
            _ => {}
        }
    }
    rewrite(&mut value, map);
    serde_json::from_value(value).expect("substituted term deserializes")
}

/// The value a match branch's pattern denotes, over its binders: the term
/// an evidence hypothesis equates the scrutinee with.
fn pattern_value(scrutinee_type: &SemanticType, branch: &SemanticBranch) -> SemanticTerm {
    let binders: Vec<SemanticTerm> = branch
        .binders
        .iter()
        .map(|binder| SemanticTerm::Var {
            name: binder.clone(),
        })
        .collect();
    match scrutinee_type {
        SemanticType::Bool => SemanticTerm::Bool {
            value: branch.constructor.name == "Bool.true",
        },
        SemanticType::Product { .. } => {
            let mut parts = binders.into_iter();
            SemanticTerm::Pair {
                left: Box::new(parts.next().expect("Prod.mk binds two values")),
                right: Box::new(parts.next().expect("Prod.mk binds two values")),
            }
        }
        _ => SemanticTerm::Constructor {
            constructor: branch.constructor.clone(),
            type_arguments: match scrutinee_type {
                SemanticType::Named { arguments, .. } => arguments.clone(),
                SemanticType::List { element } => vec![element.as_ref().clone()],
                SemanticType::Option { value } => vec![value.as_ref().clone()],
                SemanticType::Result { ok, error } => {
                    vec![ok.as_ref().clone(), error.as_ref().clone()]
                }
                _ => Vec::new(),
            },
            arguments: binders,
        },
    }
}

/// The decrease obligation of one call site and the binders it quantifies:
/// for its enclosing nodes, outermost first, each `if` contributes
/// `c = b` and each `match` contributes `s = pattern`, then
/// `measure(args) < measure`. The evidence theorem's parameters are the
/// definition's parameters followed by the returned binders.
fn decrease_obligation(
    measure: &SemanticTerm,
    callee: (&[SemanticParameter], &SemanticTerm),
    parameters: &[SemanticParameter],
    plan: &WellFoundedPlan<'_>,
    site: &CallSite,
    env: &Environment<'_>,
) -> Result<(Vec<SemanticParameter>, SemanticTerm), String> {
    let (callee_parameters, callee_measure) = callee;
    let map: BTreeMap<String, SemanticTerm> = callee_parameters
        .iter()
        .map(|parameter| parameter.name.clone())
        .zip(site.arguments.iter().cloned())
        .collect();
    let mut typed = typed_locals(parameters);
    let mut binders = Vec::new();
    let mut premises = Vec::new();
    for step in &site.path {
        match (*step, &plan.nodes[step.node()]) {
            (Step::If { polarity, .. }, PlannedNode::If { condition }) => {
                premises.push(SemanticTerm::Eq {
                    left: Box::new((*condition).clone()),
                    right: Box::new(SemanticTerm::Bool { value: polarity }),
                });
            }
            (
                Step::Match { branch, .. },
                PlannedNode::Match {
                    scrutinee,
                    branches,
                },
            ) => {
                let scrutinee_type = infer_term(scrutinee, &typed, env)?.ok_or(
                    "the scrutinee of a match enclosing a recursive call has no known type",
                )?;
                let branch = &branches[branch];
                let types = branch_binder_types(&scrutinee_type, branch, env)?;
                for (binder, ty) in branch.binders.iter().zip(types) {
                    typed.insert(binder.clone(), ty.clone());
                    binders.push(SemanticParameter {
                        name: binder.clone(),
                        r#type: ty,
                    });
                }
                premises.push(SemanticTerm::Eq {
                    left: Box::new((*scrutinee).clone()),
                    right: Box::new(pattern_value(&scrutinee_type, branch)),
                });
            }
            (
                Step::If { .. } | Step::Match { .. },
                PlannedNode::If { .. } | PlannedNode::Match { .. },
            ) => {
                return Err("internal: a well-founded plan step does not name its node".to_owned());
            }
        }
    }
    let mut statement = SemanticTerm::Lt {
        left: Box::new(substitute_locals(callee_measure, &map)),
        right: Box::new(measure.clone()),
    };
    for premise in premises.into_iter().rev() {
        statement = SemanticTerm::Implies {
            premise: Box::new(premise),
            conclusion: Box::new(statement),
        };
    }
    Ok((binders, statement))
}

/// §17.12 well-founded recursion: one statement-exact evidence theorem per
/// call site, in call-site order.
fn check_well_founded(
    name: &str,
    type_parameters: &[String],
    parameters: &[SemanticParameter],
    body: &SemanticTerm,
    termination: &SemanticTermination,
    env: &Environment<'_>,
) -> Result<(), String> {
    let group: BTreeSet<String> = env.well_founded_group.keys().cloned().collect();
    let plan = well_founded_plan(name, &group, type_parameters, parameters, body)?;
    if plan.sites.is_empty() {
        return Err(format!(
            "definition `{name}` declares termination evidence but makes no recursive call"
        ));
    }
    if plan.sites.len() != termination.evidence.len() {
        return Err(format!(
            "definition `{name}` has {} recursive call site(s) but {} evidence theorem(s)",
            plan.sites.len(),
            termination.evidence.len()
        ));
    }
    for (position, (site, evidence)) in plan.sites.iter().zip(&termination.evidence).enumerate() {
        let (callee_parameters, callee_measure) = &env.well_founded_group[&site.callee];
        let (binders, obligation) = decrease_obligation(
            &termination.measure,
            (callee_parameters, callee_measure),
            parameters,
            &plan,
            site,
            env,
        )?;
        let stated = evidence
            .module
            .is_none()
            .then(|| env.theorems.get(&evidence.name))
            .flatten()
            .ok_or_else(|| {
                format!(
                    "evidence `{}` for recursive call {position} of `{name}` is not a prior theorem of this module",
                    member_key(evidence)
                )
            })?;
        let expected_parameters: Vec<SemanticParameter> =
            parameters.iter().cloned().chain(binders).collect();
        if stated.0 != type_parameters || stated.1 != expected_parameters || stated.2 != obligation
        {
            return Err(format!(
                "evidence `{}` for recursive call {position} of `{name}` does not state its decrease obligation exactly",
                evidence.name
            ));
        }
    }
    Ok(())
}

/// Whether `name` is a member of the recursion being checked: the structural
/// definition itself, a member of its mutual group, or the well-founded
/// definition whose body this is.
fn recursive_name(
    name: &str,
    recursion: Option<(&str, usize, &str)>,
    env: &Environment<'_>,
) -> bool {
    recursion.is_some_and(|(own, _, _)| own == name)
        || env.recursive_group.contains_key(name)
        || env.well_founded_group.contains_key(name)
}

/// §17.12: the closed key types with one total canonical order, and so one
/// deterministic iteration order.
fn check_ordered_key(ty: &SemanticType) -> Result<(), String> {
    match ty {
        SemanticType::Nat
        | SemanticType::Int
        | SemanticType::Int8
        | SemanticType::Int16
        | SemanticType::Int32
        | SemanticType::Int64
        | SemanticType::UInt8
        | SemanticType::UInt16
        | SemanticType::UInt32
        | SemanticType::UInt64
        | SemanticType::Bool
        | SemanticType::String => Ok(()),
        SemanticType::Product { left, right } => {
            check_ordered_key(left)?;
            check_ordered_key(right)
        }
        other => Err(format!(
            "type {other} has no canonical order and cannot key a map, set, or graph: its iteration order would be unspecified"
        )),
    }
}

/// The map type of a graph over `node`: each node maps to its successors.
fn graph_type(node: &SemanticType) -> SemanticType {
    SemanticType::Map {
        key: Box::new(node.clone()),
        value: Box::new(SemanticType::Set {
            element: Box::new(node.clone()),
        }),
    }
}

/// A literal key's position in the canonical key order, which is the order
/// of the fixed Lean `Key` instances: integers numerically, `false` before
/// `true`, strings by Unicode scalar sequence, and pairs lexicographically.
#[derive(Clone, Debug, PartialEq, Eq)]
enum KeyOrder {
    Integer(bool, String),
    Bool(bool),
    Text(String),
    Pair(Box<KeyOrder>, Box<KeyOrder>),
}

impl Ord for KeyOrder {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use std::cmp::Ordering;
        match (self, other) {
            (Self::Integer(left_negative, left), Self::Integer(right_negative, right)) => {
                let magnitude = left.len().cmp(&right.len()).then_with(|| left.cmp(right));
                match (left_negative, right_negative) {
                    (false, false) => magnitude,
                    (true, true) => magnitude.reverse(),
                    (true, false) => Ordering::Less,
                    (false, true) => Ordering::Greater,
                }
            }
            (Self::Bool(left), Self::Bool(right)) => left.cmp(right),
            (Self::Text(left), Self::Text(right)) => left.cmp(right),
            (Self::Pair(left_first, left_second), Self::Pair(right_first, right_second)) => {
                left_first
                    .cmp(right_first)
                    .then_with(|| left_second.cmp(right_second))
            }
            // Values of one checked key type never mix variants; the rank
            // only makes the order total.
            _ => self.rank().cmp(&other.rank()),
        }
    }
}

impl PartialOrd for KeyOrder {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl KeyOrder {
    const fn rank(&self) -> u8 {
        match self {
            Self::Integer(..) => 0,
            Self::Bool(_) => 1,
            Self::Text(_) => 2,
            Self::Pair(..) => 3,
        }
    }
}

/// The canonical order of a literal key; a map, set, or graph literal is
/// keyed only by literals, so linking can sort and deduplicate it.
fn literal_key(term: &SemanticTerm) -> Result<KeyOrder, String> {
    match term {
        SemanticTerm::Nat { value } => Ok(KeyOrder::Integer(false, value.clone())),
        SemanticTerm::Integer { value, .. } => Ok(match value.strip_prefix('-') {
            Some(magnitude) => KeyOrder::Integer(true, magnitude.to_owned()),
            None => KeyOrder::Integer(false, value.clone()),
        }),
        SemanticTerm::Bool { value } => Ok(KeyOrder::Bool(*value)),
        SemanticTerm::String { value } => Ok(KeyOrder::Text(value.clone())),
        SemanticTerm::Pair { left, right } => Ok(KeyOrder::Pair(
            Box::new(literal_key(left)?),
            Box::new(literal_key(right)?),
        )),
        _ => Err(
            "a map, set, or graph literal key must be a literal value; build other keys with insert"
                .to_owned(),
        ),
    }
}

/// Sort one map, set, or graph literal into canonical key order, so
/// reordered equivalent source denotes byte-identical linked data. Linking
/// has already checked every literal key, so each has a position.
fn normalize_collection(term: &mut SemanticTerm) {
    let order = |term: &SemanticTerm| literal_key(term).ok();
    match term {
        SemanticTerm::MapLiteral { entries, .. } => entries.sort_by_key(|entry| order(&entry.key)),
        SemanticTerm::SetLiteral { elements, .. } => elements.sort_by_key(order),
        SemanticTerm::GraphLiteral { nodes, edges, .. } => {
            nodes.sort_by_key(order);
            edges.sort_by_key(|edge| (order(&edge.source), order(&edge.target)));
        }
        _ => {}
    }
}

/// Reject a language-1.2-only construct in a language-1.1 module.
fn require_language_1_2(env: &Environment<'_>, construct: &str) -> Result<(), String> {
    if env.language_1_2 {
        Ok(())
    } else {
        Err(format!(
            "`{construct}` is a language-1.2 construct; language 1.1 rejects it"
        ))
    }
}

/// The pattern-binder positions of `constructor` that are structurally
/// smaller values of the matched type: the `List.cons` tail, the `Nat.succ`
/// predecessor, and every field of a document inductive that is a direct,
/// uniform recursive occurrence of that same inductive (§17.12).
fn structurally_smaller_positions(constructor: &MemberRef, env: &Environment<'_>) -> Vec<usize> {
    if constructor.module.is_none() {
        match constructor.name.as_str() {
            "List.cons" => return vec![1],
            "Nat.succ" => return vec![0],
            "List.nil" | "Nat.zero" | "Bool.false" | "Bool.true" | "Option.none"
            | "Option.some" | "Result.error" | "Result.ok" | "Prod.mk" => return Vec::new(),
            _ => {}
        }
    }
    for (owner, info) in &env.types {
        if member_from_key(owner).module != constructor.module {
            continue;
        }
        if let Some(flags) = info.direct_recursive.get(&constructor.name) {
            return flags
                .iter()
                .enumerate()
                .filter_map(|(index, direct)| direct.then_some(index))
                .collect();
        }
    }
    Vec::new()
}

#[allow(clippy::too_many_lines)]
fn check_term(
    term: &SemanticTerm,
    locals: &BTreeSet<String>,
    env: &Environment<'_>,
    recursion: Option<(&str, usize, &str)>,
    smaller: &BTreeSet<String>,
) -> Result<(), String> {
    let pair = |left: &SemanticTerm, right: &SemanticTerm| {
        check_term(left, locals, env, recursion, smaller)?;
        check_term(right, locals, env, recursion, smaller)
    };
    match term {
        SemanticTerm::Var { name } => {
            if locals.contains(name) {
                Ok(())
            } else {
                Err(format!("unbound local `{name}`"))
            }
        }
        SemanticTerm::Nat { value } => {
            if value == "0"
                || (value.bytes().all(|b| b.is_ascii_digit()) && !value.starts_with('0'))
            {
                Ok(())
            } else {
                Err(format!("noncanonical natural literal `{value}`"))
            }
        }
        SemanticTerm::Integer {
            representation,
            value,
        } => check_integer_literal(*representation, value),
        SemanticTerm::String { .. } => Ok(()),
        SemanticTerm::Bytes { hex } => {
            if hex.len() % 2 == 0
                && hex
                    .bytes()
                    .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
            {
                Ok(())
            } else {
                Err(format!(
                    "byte literal `{hex}` must be even-length lowercase hexadecimal"
                ))
            }
        }
        SemanticTerm::Bool { .. } | SemanticTerm::Unit => Ok(()),
        SemanticTerm::Primitive {
            operation,
            arguments,
            result,
        } => {
            if operation.language_1_2() {
                require_language_1_2(env, "collection primitive")?;
            }
            check_type(result, env)?;
            for argument in arguments {
                check_term(argument, locals, env, recursion, smaller)?;
            }
            Ok(())
        }
        SemanticTerm::Nil { element } => check_type(element, env),
        SemanticTerm::Cons { head, tail }
        | SemanticTerm::Eq {
            left: head,
            right: tail,
        }
        | SemanticTerm::Le {
            left: head,
            right: tail,
        }
        | SemanticTerm::Lt {
            left: head,
            right: tail,
        }
        | SemanticTerm::Add {
            left: head,
            right: tail,
        }
        | SemanticTerm::Beq {
            left: head,
            right: tail,
        }
        | SemanticTerm::Ble {
            left: head,
            right: tail,
        }
        | SemanticTerm::Blt {
            left: head,
            right: tail,
        }
        | SemanticTerm::And {
            left: head,
            right: tail,
        }
        | SemanticTerm::PropAnd {
            left: head,
            right: tail,
        }
        | SemanticTerm::Or {
            left: head,
            right: tail,
        }
        | SemanticTerm::Implies {
            premise: head,
            conclusion: tail,
        }
        | SemanticTerm::Iff {
            left: head,
            right: tail,
        } => pair(head, tail),
        SemanticTerm::Not { value } | SemanticTerm::Project { value, .. } => {
            check_term(value, locals, env, recursion, smaller)
        }
        SemanticTerm::Record {
            r#type,
            type_arguments,
            fields,
        } => {
            check_member(r#type, env)?;
            for argument in type_arguments {
                check_type_argument(argument, env)?;
            }
            let info = type_info(r#type, env)
                .ok_or_else(|| format!("record type `{}` is unavailable", member_key(r#type)))?;
            if info.parameters != type_arguments.len() {
                return Err(format!(
                    "record type `{}` expects {} type argument(s), received {}",
                    r#type.name,
                    info.parameters,
                    type_arguments.len()
                ));
            }
            check_assignments(fields, &info.fields, locals, env, recursion, smaller)
        }
        SemanticTerm::Constructor {
            constructor,
            type_arguments,
            arguments,
        } => {
            check_member(constructor, env)?;
            for argument in type_arguments {
                check_type_argument(argument, env)?;
            }
            for argument in arguments {
                check_term(argument, locals, env, recursion, smaller)?;
            }
            if let Some(arity) = constructor_arity(constructor, env) {
                if arity != arguments.len() {
                    return Err(format!(
                        "constructor `{}` expects {arity} argument(s), received {}",
                        constructor.name,
                        arguments.len()
                    ));
                }
            } else if constructor.module.is_none()
                && !matches!(
                    constructor.name.as_str(),
                    "Bool.false"
                        | "Bool.true"
                        | "List.nil"
                        | "List.cons"
                        | "Nat.zero"
                        | "Nat.succ"
                        | "Option.none"
                        | "Option.some"
                        | "Result.error"
                        | "Result.ok"
                )
            {
                return Err(format!("unknown constructor `{}`", constructor.name));
            }
            Ok(())
        }
        SemanticTerm::InstanceValue {
            class,
            arguments,
            resolved,
        } => {
            check_member(class, env)?;
            check_name(&resolved.name, "resolved instance")?;
            if resolved
                .module
                .as_ref()
                .is_some_and(|module| !env.imports.contains(module))
            {
                return Err(format!(
                    "resolved instance `{}` is outside the imported module closure",
                    member_key(resolved)
                ));
            }
            for argument in arguments {
                check_type(argument, env)?;
            }
            let key = format!("{}:{arguments:?}", member_key(class));
            match env.instances.get(&key) {
                Some(instance) if instance == resolved => Ok(()),
                Some(instance) => Err(format!(
                    "instance requirement `{key}` resolves to `{}`, not `{}`",
                    member_key(instance),
                    member_key(resolved)
                )),
                None => Err(format!("missing instance for requirement `{key}`")),
            }
        }
        SemanticTerm::Call {
            function,
            type_arguments,
            arguments,
        } => {
            check_member(function, env)?;
            if !type_arguments.is_empty() {
                require_language_1_2(env, "call type arguments")?;
            }
            for argument in type_arguments {
                check_type_argument(argument, env)?;
            }
            for argument in arguments {
                check_term(argument, locals, env, recursion, smaller)?;
            }
            let recursive_call = if function.module.is_none() {
                match env.recursive_group.get(&function.name) {
                    Some(decreasing) if recursion.is_some() => {
                        Some((function.name.as_str(), *decreasing, ""))
                    }
                    _ => recursion.filter(|(name, _, _)| *name == function.name),
                }
            } else {
                None
            };
            if let Some((self_name, decreasing, _)) = recursive_call {
                // §17.12: no polymorphic recursion, so every executable root
                // has finitely many specializations.
                let own: Vec<SemanticType> = env
                    .current_type_parameters
                    .iter()
                    .map(|name| SemanticType::Parameter { name: name.clone() })
                    .collect();
                if type_arguments != &own {
                    return Err(format!(
                        "recursive call `{self_name}` must pass its own type parameters in order; polymorphic recursion is not permitted"
                    ));
                }
                if arguments.len() <= decreasing {
                    return Err(format!(
                        "recursive call `{self_name}` omits its decreasing argument"
                    ));
                }
                match &arguments[decreasing] {
                    SemanticTerm::Var { name } if smaller.contains(name) => {}
                    _ => {
                        return Err(format!(
                            "recursive call `{self_name}` is not on a structurally smaller value"
                        ));
                    }
                }
            } else {
                let info = function_info(function, env).ok_or_else(|| {
                    format!("forward or missing function `{}`", member_key(function))
                })?;
                if info.type_parameters.len() != type_arguments.len() {
                    return Err(format!(
                        "function `{}` expects {} explicit type argument(s), received {}",
                        member_key(function),
                        info.type_parameters.len(),
                        type_arguments.len()
                    ));
                }
                if info.parameters.len() != arguments.len() {
                    return Err(format!(
                        "function `{}` expects {} argument(s), received {}",
                        member_key(function),
                        info.parameters.len(),
                        arguments.len()
                    ));
                }
            }
            Ok(())
        }
        SemanticTerm::If {
            condition,
            then_value,
            else_value,
        } => {
            check_term(condition, locals, env, recursion, smaller)?;
            pair(then_value, else_value)
        }
        SemanticTerm::Match {
            scrutinee,
            branches,
        } => {
            check_term(scrutinee, locals, env, recursion, smaller)?;
            if branches.is_empty() {
                return Err("match has no branches".to_owned());
            }
            let mut constructors = BTreeSet::new();
            for branch in branches {
                check_member(&branch.constructor, env)?;
                if !constructors.insert(branch.constructor.name.clone()) {
                    return Err(format!(
                        "duplicate match branch `{}`",
                        branch.constructor.name
                    ));
                }
                let expected = match branch.constructor.name.as_str() {
                    "Bool.false" | "Bool.true" | "List.nil" | "Nat.zero" => 0,
                    "List.cons" => 2,
                    "Nat.succ" => 1,
                    "Option.none" => 0,
                    "Option.some" | "Result.error" | "Result.ok" => 1,
                    "Prod.mk" if env.language_1_2 && branch.constructor.module.is_none() => 2,
                    _ => constructor_arity(&branch.constructor, env).ok_or_else(|| {
                        format!("unknown match constructor `{}`", branch.constructor.name)
                    })?,
                };
                if expected != branch.binders.len() {
                    return Err(format!(
                        "match branch `{}` expects {expected} binder(s), received {}",
                        branch.constructor.name,
                        branch.binders.len()
                    ));
                }
                let mut branch_locals = locals.clone();
                for binder in &branch.binders {
                    check_binder(binder, "pattern binder", env)?;
                    if !branch_locals.insert(binder.clone()) {
                        return Err(format!("duplicate or shadowed pattern binder `{binder}`"));
                    }
                }
                let mut branch_smaller = smaller.clone();
                if recursion.is_some_and(|(_, _, argument)| {
                    matches!(scrutinee.as_ref(), SemanticTerm::Var { name } if name == argument)
                }) {
                    if let (Some(family), Some(decreasing)) =
                        (&env.recursive_family, &env.current_decreasing_type)
                    {
                        // §17.12 mutual recursion: every binder of the matched
                        // constructor that belongs to the group's family is a
                        // structurally smaller subvalue.
                        let type_arguments: Vec<SemanticType> =
                            match Some(decreasing) {
                                Some(SemanticType::Named { arguments, .. }) => arguments.clone(),
                                Some(SemanticType::List { element })
                                | Some(SemanticType::Option { value: element }) => {
                                    vec![element.as_ref().clone()]
                                }
                                _ => Vec::new(),
                            };
                        if let Ok(Some((_, fields))) =
                            constructor_signature(&branch.constructor, &type_arguments, env)
                        {
                            for (binder, field) in branch.binders.iter().zip(&fields) {
                                if in_family(field, family, env) {
                                    branch_smaller.insert(binder.clone());
                                }
                            }
                        }
                    } else if env.language_1_2 {
                        for position in
                            structurally_smaller_positions(&branch.constructor, env)
                        {
                            if let Some(binder) = branch.binders.get(position) {
                                branch_smaller.insert(binder.clone());
                            }
                        }
                    } else if let Some(last) = branch.binders.last() {
                        branch_smaller.insert(last.clone());
                    }
                }
                check_term(
                    &branch.body,
                    &branch_locals,
                    env,
                    recursion,
                    &branch_smaller,
                )?;
            }
            let list = BTreeSet::from(["List.cons".to_owned(), "List.nil".to_owned()]);
            let bool_ = BTreeSet::from(["Bool.false".to_owned(), "Bool.true".to_owned()]);
            let nat = BTreeSet::from(["Nat.succ".to_owned(), "Nat.zero".to_owned()]);
            let option = BTreeSet::from(["Option.none".to_owned(), "Option.some".to_owned()]);
            let result = BTreeSet::from(["Result.error".to_owned(), "Result.ok".to_owned()]);
            let product = BTreeSet::from(["Prod.mk".to_owned()]);
            let violation: BTreeSet<String> = model::VIOLATIONS
                .iter()
                .map(|name| (*name).to_owned())
                .collect();
            let failure: BTreeSet<String> = reasoning::FAILURES
                .iter()
                .map(|name| (*name).to_owned())
                .collect();
            let declared = env.types.values().find_map(|info| {
                let set: BTreeSet<String> = info.constructors.keys().cloned().collect();
                (set == constructors).then_some(set)
            });
            if constructors != bool_
                && constructors != list
                && constructors != nat
                && constructors != option
                && constructors != result
                && !(env.language_1_2 && constructors == product)
                && !(env.language_1_2 && constructors == violation)
                && !(env.language_1_2 && constructors == failure)
                && declared.is_none()
            {
                return Err(format!(
                    "nonexhaustive or mixed match branches {constructors:?}"
                ));
            }
            Ok(())
        }
        SemanticTerm::Forall { binder, body } => {
            check_binder(&binder.name, "binder", env)?;
            check_type(&binder.r#type, env)?;
            let mut nested = locals.clone();
            if !nested.insert(binder.name.clone()) {
                return Err(format!("shadowed binder `{}`", binder.name));
            }
            check_term(body, &nested, env, recursion, smaller)
        }
        SemanticTerm::Let {
            binder,
            value,
            body,
        } => {
            check_binder(&binder.name, "let binder", env)?;
            check_type(&binder.r#type, env)?;
            // The bound value is checked in the enclosing scope: a let is
            // never recursive, so its own binder is not visible there.
            check_term(value, locals, env, recursion, smaller)?;
            let mut nested = locals.clone();
            if !nested.insert(binder.name.clone()) {
                return Err(format!("shadowed let binder `{}`", binder.name));
            }
            // A let-bound alias of a structural subvalue is not itself
            // admitted as smaller: termination evidence stays syntactic.
            check_term(body, &nested, env, recursion, smaller)
        }
        SemanticTerm::Pair { left, right } => {
            require_language_1_2(env, "pair")?;
            pair(left, right)
        }
        SemanticTerm::First { value } => {
            require_language_1_2(env, "first")?;
            check_term(value, locals, env, recursion, smaller)
        }
        SemanticTerm::Second { value } => {
            require_language_1_2(env, "second")?;
            check_term(value, locals, env, recursion, smaller)
        }
        SemanticTerm::Lambda {
            parameters,
            captures,
            body,
        } => {
            require_language_1_2(env, "lambda")?;
            if parameters.is_empty() {
                return Err("a lambda binds at least one parameter".to_owned());
            }
            if captures.windows(2).any(|pair| pair[0] >= pair[1]) {
                return Err("lambda captures are strictly sorted and unique".to_owned());
            }
            for capture in captures {
                if !locals.contains(capture) {
                    return Err(format!(
                        "lambda capture `{capture}` is not an enclosing local"
                    ));
                }
            }
            // Only the declared captures and the lambda's own parameters
            // are visible in its body: capture is explicit, never ambient.
            let mut inner: BTreeSet<String> = captures.iter().cloned().collect();
            for parameter in parameters {
                check_binder(&parameter.name, "lambda parameter", env)?;
                check_type(&parameter.r#type, env)?;
                if locals.contains(&parameter.name) || !inner.insert(parameter.name.clone()) {
                    return Err(format!("shadowed lambda parameter `{}`", parameter.name));
                }
            }
            let mut used = BTreeSet::new();
            free_locals(body, &mut BTreeSet::new(), &mut used);
            let bound: BTreeSet<String> = parameters
                .iter()
                .map(|parameter| parameter.name.clone())
                .collect();
            let used: BTreeSet<String> = used.difference(&bound).cloned().collect();
            let declared: BTreeSet<String> = captures.iter().cloned().collect();
            if used != declared {
                let names =
                    |set: &BTreeSet<String>| set.iter().cloned().collect::<Vec<_>>().join(", ");
                return Err(format!(
                    "lambda captures must be exactly the enclosing locals its body uses: declared ({}), used ({})",
                    names(&declared),
                    names(&used)
                ));
            }
            // No member of the recursion being checked (the definition
            // itself, any member of its mutual group, or a well-founded
            // definition) is called or referenced under a lambda, where no
            // decrease is checked.
            let mut recursive = None;
            visit_terms(body, &mut |term| {
                if let SemanticTerm::Call { function, .. }
                | SemanticTerm::FunctionRef { function, .. } = term
                {
                    if recursive.is_none()
                        && function.module.is_none()
                        && recursive_name(&function.name, recursion, env)
                    {
                        recursive = Some(function.name.clone());
                    }
                }
            });
            if let Some(member) = recursive {
                return Err(format!(
                    "recursive call `{member}` inside a lambda is not structurally checked"
                ));
            }
            check_term(body, &inner, env, None, &BTreeSet::new())
        }
        SemanticTerm::Apply {
            function,
            arguments,
        } => {
            require_language_1_2(env, "apply")?;
            if arguments.is_empty() {
                return Err("an application supplies at least one argument".to_owned());
            }
            check_term(function, locals, env, recursion, smaller)?;
            for argument in arguments {
                check_term(argument, locals, env, recursion, smaller)?;
            }
            Ok(())
        }
        SemanticTerm::FunctionRef {
            function,
            type_arguments,
        } => {
            require_language_1_2(env, "function_ref")?;
            check_member(function, env)?;
            if function.module.is_none() && recursive_name(&function.name, recursion, env) {
                return Err(format!(
                    "recursive definition `{}` cannot be referenced as a value of itself",
                    function.name
                ));
            }
            for argument in type_arguments {
                check_type_argument(argument, env)?;
            }
            let info = function_info(function, env)
                .ok_or_else(|| format!("forward or missing function `{}`", member_key(function)))?;
            if info.parameters.is_empty() {
                return Err(format!(
                    "`{}` has no parameters and is not a function value",
                    member_key(function)
                ));
            }
            if info.type_parameters.len() != type_arguments.len() {
                return Err(format!(
                    "function `{}` expects {} explicit type argument(s), received {}",
                    member_key(function),
                    info.type_parameters.len(),
                    type_arguments.len()
                ));
            }
            Ok(())
        }
        SemanticTerm::MapLiteral {
            key,
            value,
            entries,
        } => {
            require_language_1_2(env, "map_literal")?;
            check_ordered_key(key)?;
            check_type(key, env)?;
            check_type(value, env)?;
            let mut seen = BTreeSet::new();
            for entry in entries {
                // A key is checked as a term first: its canonical spelling
                // and range are what make the literal order total.
                check_term(&entry.key, locals, env, recursion, smaller)?;
                let order = literal_key(&entry.key)?;
                if !seen.insert(order) {
                    return Err("duplicate map key in a map literal".to_owned());
                }
                check_term(&entry.value, locals, env, recursion, smaller)?;
            }
            Ok(())
        }
        SemanticTerm::SetLiteral { element, elements } => {
            require_language_1_2(env, "set_literal")?;
            check_ordered_key(element)?;
            check_type(element, env)?;
            let mut seen = BTreeSet::new();
            for item in elements {
                check_term(item, locals, env, recursion, smaller)?;
                if !seen.insert(literal_key(item)?) {
                    return Err("duplicate element in a set literal".to_owned());
                }
            }
            Ok(())
        }
        SemanticTerm::GraphLiteral { node, nodes, edges } => {
            require_language_1_2(env, "graph_literal")?;
            check_ordered_key(node)?;
            check_type(node, env)?;
            let mut declared = BTreeSet::new();
            for item in nodes {
                check_term(item, locals, env, recursion, smaller)?;
                if !declared.insert(literal_key(item)?) {
                    return Err("duplicate node in a graph literal".to_owned());
                }
            }
            let mut seen = BTreeSet::new();
            for edge in edges {
                check_term(&edge.source, locals, env, recursion, smaller)?;
                check_term(&edge.target, locals, env, recursion, smaller)?;
                let source = literal_key(&edge.source)?;
                let target = literal_key(&edge.target)?;
                for (end, order) in [("source", &source), ("target", &target)] {
                    if !declared.contains(order) {
                        return Err(format!(
                            "graph edge {end} is not a declared node of the graph literal"
                        ));
                    }
                }
                if !seen.insert((source, target)) {
                    return Err("duplicate edge in a graph literal".to_owned());
                }
            }
            Ok(())
        }
        // Linking elaborates every checked application before ordinary
        // checking (§17.12, models), so one reaching here is a compiler
        // defect, refused rather than admitted unchecked.
        SemanticTerm::CheckedApply { .. } => {
            Err("internal: a checked model application reached ordinary checking".to_owned())
        }
    }
}

fn substitute_type(
    ty: &SemanticType,
    substitutions: &BTreeMap<String, SemanticType>,
) -> SemanticType {
    match ty {
        SemanticType::Parameter { name } => substitutions
            .get(name)
            .cloned()
            .unwrap_or_else(|| ty.clone()),
        SemanticType::List { element } => SemanticType::List {
            element: Box::new(substitute_type(element, substitutions)),
        },
        SemanticType::Option { value } => SemanticType::Option {
            value: Box::new(substitute_type(value, substitutions)),
        },
        SemanticType::Result { ok, error } => SemanticType::Result {
            ok: Box::new(substitute_type(ok, substitutions)),
            error: Box::new(substitute_type(error, substitutions)),
        },
        SemanticType::Named { member, arguments } => SemanticType::Named {
            member: member.clone(),
            arguments: arguments
                .iter()
                .map(|argument| substitute_type(argument, substitutions))
                .collect(),
        },
        SemanticType::Product { left, right } => SemanticType::Product {
            left: Box::new(substitute_type(left, substitutions)),
            right: Box::new(substitute_type(right, substitutions)),
        },
        SemanticType::Function { parameters, result } => SemanticType::Function {
            parameters: parameters
                .iter()
                .map(|parameter| substitute_type(parameter, substitutions))
                .collect(),
            result: Box::new(substitute_type(result, substitutions)),
        },
        SemanticType::Map { key, value } => SemanticType::Map {
            key: Box::new(substitute_type(key, substitutions)),
            value: Box::new(substitute_type(value, substitutions)),
        },
        SemanticType::Set { element } => SemanticType::Set {
            element: Box::new(substitute_type(element, substitutions)),
        },
        SemanticType::Type
        | SemanticType::Nat
        | SemanticType::Bool
        | SemanticType::Prop
        | SemanticType::Unit
        | SemanticType::Int
        | SemanticType::Int8
        | SemanticType::Int16
        | SemanticType::Int32
        | SemanticType::Int64
        | SemanticType::UInt8
        | SemanticType::UInt16
        | SemanticType::UInt32
        | SemanticType::UInt64
        | SemanticType::String
        | SemanticType::Bytes
        | SemanticType::Ordering
        | SemanticType::ContractViolation
        | SemanticType::ReasoningFailure => ty.clone(),
    }
}

fn substitutions(info: &TypeInfo, arguments: &[SemanticType]) -> BTreeMap<String, SemanticType> {
    info.type_parameters
        .iter()
        .cloned()
        .zip(arguments.iter().cloned())
        .collect()
}

fn require_type(
    observed: Option<SemanticType>,
    expected: &SemanticType,
    context: &str,
) -> Result<(), String> {
    if let Some(observed) = observed {
        if &observed != expected {
            return Err(format!(
                "{context} has type {observed}, expected {expected}"
            ));
        }
    }
    Ok(())
}

fn require_observed(
    observed: &[Option<SemanticType>],
    expected: &[SemanticType],
    operation: SemanticPrimitive,
) -> Result<(), String> {
    if observed.len() != expected.len() {
        return Err(format!(
            "primitive {operation:?} expects {} argument(s), received {}",
            expected.len(),
            observed.len()
        ));
    }
    for (index, (actual, expected)) in observed.iter().zip(expected).enumerate() {
        require_type(
            actual.clone(),
            expected,
            &format!("primitive {operation:?} argument {index}"),
        )?;
    }
    Ok(())
}

fn infer_primitive(
    operation: SemanticPrimitive,
    arguments: &[Option<SemanticType>],
    result: &SemanticType,
) -> Result<SemanticType, String> {
    use SemanticPrimitive as P;
    let first = arguments.first().cloned().flatten().ok_or_else(|| {
        format!("primitive {operation:?} requires a statically typed first argument")
    })?;
    let exact_result = |expected: SemanticType| -> Result<SemanticType, String> {
        if result == &expected {
            Ok(expected)
        } else {
            Err(format!(
                "primitive {operation:?} result is {result}, expected {expected}"
            ))
        }
    };
    match operation {
        P::Subtract | P::Multiply => {
            if !matches!(first, SemanticType::Nat | SemanticType::Int) {
                return Err(format!("primitive {operation:?} requires integer operands"));
            }
            require_observed(arguments, &[first.clone(), first.clone()], operation)?;
            exact_result(first)
        }
        P::Quotient | P::Remainder => {
            if !matches!(first, SemanticType::Nat | SemanticType::Int) {
                return Err(format!("primitive {operation:?} requires integer operands"));
            }
            require_observed(
                arguments,
                &[first.clone(), first.clone(), first.clone()],
                operation,
            )?;
            exact_result(first)
        }
        P::Negate => {
            if !matches!(first, SemanticType::Int) {
                return Err("primitive Negate requires mathematical Int".to_owned());
            }
            require_observed(arguments, core::slice::from_ref(&first), operation)?;
            exact_result(first)
        }
        P::CheckedConvert => {
            if integer_representation(&first).is_none() {
                return Err("primitive CheckedConvert requires an integer input".to_owned());
            }
            let SemanticType::Option { value } = result else {
                return Err("primitive CheckedConvert returns Option of a fixed integer".to_owned());
            };
            if !fixed_integer(value) {
                return Err("primitive CheckedConvert target must be fixed-width".to_owned());
            }
            require_observed(arguments, core::slice::from_ref(&first), operation)?;
            Ok(result.clone())
        }
        P::CheckedAdd | P::CheckedSubtract | P::CheckedMultiply | P::CheckedQuotient => {
            if !fixed_integer(&first) {
                return Err(format!(
                    "primitive {operation:?} requires fixed-width operands"
                ));
            }
            require_observed(arguments, &[first.clone(), first.clone()], operation)?;
            exact_result(SemanticType::Option {
                value: Box::new(first),
            })
        }
        P::CheckedNegate => {
            if !fixed_integer(&first) || !signed_integer(&first) {
                return Err(
                    "primitive CheckedNegate requires a signed fixed-width operand".to_owned(),
                );
            }
            require_observed(arguments, core::slice::from_ref(&first), operation)?;
            exact_result(SemanticType::Option {
                value: Box::new(first),
            })
        }
        P::BitAnd | P::BitOr | P::BitXor => {
            if !fixed_integer(&first) {
                return Err(format!(
                    "primitive {operation:?} requires fixed-width operands"
                ));
            }
            require_observed(arguments, &[first.clone(), first.clone()], operation)?;
            exact_result(first)
        }
        P::BitNot => {
            if !fixed_integer(&first) {
                return Err("primitive BitNot requires a fixed-width operand".to_owned());
            }
            require_observed(arguments, core::slice::from_ref(&first), operation)?;
            exact_result(first)
        }
        P::ShiftLeft | P::ShiftRight => {
            if !fixed_integer(&first) {
                return Err(format!(
                    "primitive {operation:?} requires a fixed-width operand"
                ));
            }
            require_observed(arguments, &[first.clone(), SemanticType::UInt32], operation)?;
            exact_result(SemanticType::Option {
                value: Box::new(first),
            })
        }
        P::Append => {
            if !matches!(first, SemanticType::List { .. } | SemanticType::Bytes) {
                return Err("primitive Append requires lists or bytes".to_owned());
            }
            require_observed(arguments, &[first.clone(), first.clone()], operation)?;
            exact_result(first)
        }
        P::Length => {
            if !matches!(
                first,
                SemanticType::List { .. } | SemanticType::Bytes | SemanticType::String
            ) {
                return Err("primitive Length requires list, bytes, or string".to_owned());
            }
            require_observed(arguments, core::slice::from_ref(&first), operation)?;
            exact_result(SemanticType::Nat)
        }
        P::Index => {
            let element = match &first {
                SemanticType::List { element } => element.as_ref().clone(),
                SemanticType::Bytes => SemanticType::UInt8,
                _ => return Err("primitive Index requires list or bytes".to_owned()),
            };
            require_observed(arguments, &[first, SemanticType::Nat], operation)?;
            exact_result(SemanticType::Option {
                value: Box::new(element),
            })
        }
        P::Slice => {
            if !matches!(first, SemanticType::List { .. } | SemanticType::Bytes) {
                return Err("primitive Slice requires list or bytes".to_owned());
            }
            require_observed(
                arguments,
                &[first.clone(), SemanticType::Nat, SemanticType::Nat],
                operation,
            )?;
            exact_result(SemanticType::Option {
                value: Box::new(first),
            })
        }
        P::Utf8Encode => {
            require_observed(arguments, &[SemanticType::String], operation)?;
            exact_result(SemanticType::Bytes)
        }
        P::Utf8Decode => {
            require_observed(arguments, &[SemanticType::Bytes], operation)?;
            exact_result(SemanticType::Option {
                value: Box::new(SemanticType::String),
            })
        }
        P::CompareBytes => {
            require_observed(
                arguments,
                &[SemanticType::Bytes, SemanticType::Bytes],
                operation,
            )?;
            exact_result(SemanticType::Ordering)
        }
        P::Equal => {
            if !matches!(
                first,
                SemanticType::Nat
                    | SemanticType::Bool
                    | SemanticType::Int8
                    | SemanticType::Int16
                    | SemanticType::Int32
                    | SemanticType::Int64
                    | SemanticType::UInt8
                    | SemanticType::UInt16
                    | SemanticType::UInt32
                    | SemanticType::UInt64
                    | SemanticType::String
                    | SemanticType::Bytes
                    | SemanticType::Ordering
            ) {
                return Err("primitive Equal requires a closed decidable scalar type".to_owned());
            }
            require_observed(arguments, &[first.clone(), first], operation)?;
            exact_result(SemanticType::Bool)
        }
        P::SplitExact => {
            require_observed(
                arguments,
                &[
                    SemanticType::String,
                    SemanticType::String,
                    SemanticType::UInt32,
                ],
                operation,
            )?;
            exact_result(SemanticType::Option {
                value: Box::new(SemanticType::List {
                    element: Box::new(SemanticType::String),
                }),
            })
        }
        P::Join => {
            require_observed(
                arguments,
                &[
                    SemanticType::List {
                        element: Box::new(SemanticType::String),
                    },
                    SemanticType::String,
                ],
                operation,
            )?;
            exact_result(SemanticType::String)
        }
        P::ParseDecimal => {
            require_observed(arguments, &[SemanticType::String], operation)?;
            let SemanticType::Option { value } = result else {
                return Err("primitive ParseDecimal returns Option of an integer".to_owned());
            };
            if integer_representation(value).is_none() {
                return Err("primitive ParseDecimal target must be an integer".to_owned());
            }
            Ok(result.clone())
        }
        P::FormatDecimal => {
            if integer_representation(&first).is_none() {
                return Err("primitive FormatDecimal requires an integer".to_owned());
            }
            require_observed(arguments, core::slice::from_ref(&first), operation)?;
            exact_result(SemanticType::String)
        }
        P::MapInsert
        | P::MapRemove
        | P::MapLookup
        | P::MapContains
        | P::MapSize
        | P::MapKeys
        | P::MapValues
        | P::MapEntries => {
            let SemanticType::Map { key, value } = &first else {
                return Err(format!("primitive {operation:?} requires a map"));
            };
            let (key, value) = (key.as_ref().clone(), value.as_ref().clone());
            let list = |element: SemanticType| SemanticType::List {
                element: Box::new(element),
            };
            match operation {
                P::MapInsert => {
                    require_observed(arguments, &[first.clone(), key, value], operation)?;
                    exact_result(first)
                }
                P::MapRemove => {
                    require_observed(arguments, &[first.clone(), key], operation)?;
                    exact_result(first)
                }
                P::MapLookup => {
                    require_observed(arguments, &[first.clone(), key], operation)?;
                    exact_result(SemanticType::Option {
                        value: Box::new(value),
                    })
                }
                P::MapContains => {
                    require_observed(arguments, &[first.clone(), key], operation)?;
                    exact_result(SemanticType::Bool)
                }
                P::MapSize => {
                    require_observed(arguments, core::slice::from_ref(&first), operation)?;
                    exact_result(SemanticType::Nat)
                }
                P::MapKeys => {
                    require_observed(arguments, core::slice::from_ref(&first), operation)?;
                    exact_result(list(key))
                }
                P::MapValues => {
                    require_observed(arguments, core::slice::from_ref(&first), operation)?;
                    exact_result(list(value))
                }
                _ => {
                    require_observed(arguments, core::slice::from_ref(&first), operation)?;
                    exact_result(list(SemanticType::Product {
                        left: Box::new(key),
                        right: Box::new(value),
                    }))
                }
            }
        }
        P::SetInsert
        | P::SetRemove
        | P::SetContains
        | P::SetSize
        | P::SetElements
        | P::SetUnion
        | P::SetIntersection
        | P::SetDifference => {
            let SemanticType::Set { element } = &first else {
                return Err(format!("primitive {operation:?} requires a set"));
            };
            let element = element.as_ref().clone();
            match operation {
                P::SetInsert | P::SetRemove => {
                    require_observed(arguments, &[first.clone(), element], operation)?;
                    exact_result(first)
                }
                P::SetContains => {
                    require_observed(arguments, &[first.clone(), element], operation)?;
                    exact_result(SemanticType::Bool)
                }
                P::SetSize => {
                    require_observed(arguments, core::slice::from_ref(&first), operation)?;
                    exact_result(SemanticType::Nat)
                }
                P::SetElements => {
                    require_observed(arguments, core::slice::from_ref(&first), operation)?;
                    exact_result(SemanticType::List {
                        element: Box::new(element),
                    })
                }
                _ => {
                    require_observed(arguments, &[first.clone(), first.clone()], operation)?;
                    exact_result(first)
                }
            }
        }
        P::ListFold | P::MapFold | P::SetFold => {
            if arguments.len() != 3 {
                return Err(format!(
                    "primitive {operation:?} expects 3 argument(s) (step, initial state, collection), received {}",
                    arguments.len()
                ));
            }
            let state =
                arguments.get(1).cloned().flatten().ok_or_else(|| {
                    format!("primitive {operation:?} requires a typed initial state")
                })?;
            let collection =
                arguments.get(2).cloned().flatten().ok_or_else(|| {
                    format!("primitive {operation:?} requires a typed collection")
                })?;
            let mut step_parameters = vec![state.clone()];
            match (operation, &collection) {
                (P::ListFold, SemanticType::List { element })
                | (P::SetFold, SemanticType::Set { element }) => {
                    step_parameters.push(element.as_ref().clone());
                }
                (P::MapFold, SemanticType::Map { key, value }) => {
                    step_parameters.push(key.as_ref().clone());
                    step_parameters.push(value.as_ref().clone());
                }
                _ => {
                    return Err(format!(
                        "primitive {operation:?} folds over the wrong collection type {collection}"
                    ));
                }
            }
            let step = SemanticType::Function {
                parameters: step_parameters,
                result: Box::new(state.clone()),
            };
            require_observed(arguments, &[step, state.clone(), collection], operation)?;
            exact_result(state)
        }
        P::Iterate | P::IterateUntil => {
            // The bound is part of the vocabulary: no iteration is unbounded.
            if arguments.len() != 3 {
                return Err(format!(
                    "primitive {operation:?} expects 3 argument(s) (step, natural-number bound, initial state), received {}",
                    arguments.len()
                ));
            }
            let state =
                arguments.get(2).cloned().flatten().ok_or_else(|| {
                    format!("primitive {operation:?} requires a typed initial state")
                })?;
            let stepped = if operation == P::Iterate {
                state.clone()
            } else {
                SemanticType::Option {
                    value: Box::new(state.clone()),
                }
            };
            let step = SemanticType::Function {
                parameters: vec![state.clone()],
                result: Box::new(stepped),
            };
            require_observed(
                arguments,
                &[step, SemanticType::Nat, state.clone()],
                operation,
            )?;
            exact_result(if operation == P::Iterate {
                state
            } else {
                SemanticType::Product {
                    left: Box::new(state),
                    right: Box::new(SemanticType::Bool),
                }
            })
        }
        P::GraphSuccessors | P::GraphReachable | P::GraphTopological => {
            let node = match &first {
                SemanticType::Map { key, value } if matches!(value.as_ref(), SemanticType::Set { element } if element == key) => {
                    key.as_ref().clone()
                }
                _ => {
                    return Err(format!(
                        "primitive {operation:?} requires a graph `Map node (Set node)`"
                    ));
                }
            };
            if operation == P::GraphTopological {
                require_observed(arguments, core::slice::from_ref(&first), operation)?;
                exact_result(SemanticType::Option {
                    value: Box::new(SemanticType::List {
                        element: Box::new(node),
                    }),
                })
            } else {
                require_observed(arguments, &[first.clone(), node.clone()], operation)?;
                exact_result(SemanticType::Set {
                    element: Box::new(node),
                })
            }
        }
        P::LessThan => {
            check_ordered_key(&first).map_err(|_| {
                format!("primitive LessThan requires an ordered key type, not {first}")
            })?;
            require_observed(arguments, &[first.clone(), first.clone()], operation)?;
            exact_result(SemanticType::Bool)
        }
    }
}

fn same_type(
    left: Option<SemanticType>,
    right: Option<SemanticType>,
    context: &str,
) -> Result<Option<SemanticType>, String> {
    match (left, right) {
        (Some(left), Some(right)) if left != right => Err(format!(
            "{context} has incompatible types {left} and {right}"
        )),
        (Some(left), _) => Ok(Some(left)),
        (_, Some(right)) => Ok(Some(right)),
        (None, None) => Ok(None),
    }
}

fn constructor_signature(
    constructor: &MemberRef,
    type_arguments: &[SemanticType],
    env: &Environment<'_>,
) -> Result<Option<(SemanticType, Vec<SemanticType>)>, String> {
    if constructor.module.is_none()
        && (constructor.name == "Option.none" || constructor.name == "Option.some")
    {
        let [value] = type_arguments else {
            return Err(format!(
                "constructor `{}` requires one explicit type argument",
                constructor.name
            ));
        };
        let result = SemanticType::Option {
            value: Box::new(value.clone()),
        };
        let fields = if constructor.name == "Option.some" {
            vec![value.clone()]
        } else {
            Vec::new()
        };
        return Ok(Some((result, fields)));
    }
    if constructor.module.is_none()
        && (constructor.name == "Result.error" || constructor.name == "Result.ok")
    {
        let [ok, error] = type_arguments else {
            return Err(format!(
                "constructor `{}` requires explicit success and error type arguments",
                constructor.name
            ));
        };
        let result = SemanticType::Result {
            ok: Box::new(ok.clone()),
            error: Box::new(error.clone()),
        };
        let fields = if constructor.name == "Result.ok" {
            vec![ok.clone()]
        } else {
            vec![error.clone()]
        };
        return Ok(Some((result, fields)));
    }
    if constructor.module.is_none()
        && (constructor.name == "List.nil" || constructor.name == "List.cons")
    {
        let [element] = type_arguments else {
            return Err(format!(
                "constructor `{}` requires one explicit type argument",
                constructor.name
            ));
        };
        let list = SemanticType::List {
            element: Box::new(element.clone()),
        };
        let fields = if constructor.name == "List.cons" {
            vec![element.clone(), list.clone()]
        } else {
            Vec::new()
        };
        return Ok(Some((list, fields)));
    }
    if constructor.module.is_none() && model::VIOLATIONS.contains(&constructor.name.as_str()) {
        if !type_arguments.is_empty() {
            return Err(format!(
                "constructor `{}` takes no type arguments",
                constructor.name
            ));
        }
        require_language_1_2(env, "contract_violation constructor")?;
        return Ok(Some((SemanticType::ContractViolation, Vec::new())));
    }
    if constructor.module.is_none() && reasoning::FAILURES.contains(&constructor.name.as_str()) {
        if !type_arguments.is_empty() {
            return Err(format!(
                "constructor `{}` takes no type arguments",
                constructor.name
            ));
        }
        require_language_1_2(env, "reasoning_failure constructor")?;
        return Ok(Some((SemanticType::ReasoningFailure, Vec::new())));
    }
    if constructor.module.is_none()
        && (constructor.name == "Nat.zero" || constructor.name == "Nat.succ")
    {
        if !type_arguments.is_empty() {
            return Err(format!(
                "constructor `{}` takes no type arguments",
                constructor.name
            ));
        }
        let fields = if constructor.name == "Nat.succ" {
            vec![SemanticType::Nat]
        } else {
            Vec::new()
        };
        return Ok(Some((SemanticType::Nat, fields)));
    }
    for (owner, info) in &env.types {
        let owner_member = member_from_key(owner);
        if owner_member.module != constructor.module {
            continue;
        }
        if let Some(fields) = info.constructor_types.get(&constructor.name) {
            if info.parameters != type_arguments.len() {
                return Err(format!(
                    "constructor `{}` expects {} type argument(s), received {}",
                    constructor.name,
                    info.parameters,
                    type_arguments.len()
                ));
            }
            let map = substitutions(info, type_arguments);
            let result = SemanticType::Named {
                member: owner_member,
                arguments: type_arguments.to_vec(),
            };
            return Ok(Some((
                result,
                fields
                    .iter()
                    .map(|field| substitute_type(field, &map))
                    .collect(),
            )));
        }
    }
    Ok(None)
}

/// The types a match branch binds for a scrutinee of `scrutinee_type`.
fn branch_binder_types(
    scrutinee_type: &SemanticType,
    branch: &SemanticBranch,
    env: &Environment<'_>,
) -> Result<Vec<SemanticType>, String> {
    Ok(match scrutinee_type {
        SemanticType::Bool => match branch.constructor.name.as_str() {
            "Bool.false" | "Bool.true" => Vec::new(),
            _ => return Err("Bool match uses a non-Bool constructor".to_owned()),
        },
        SemanticType::List { element } => match branch.constructor.name.as_str() {
            "List.nil" => Vec::new(),
            "List.cons" => vec![
                element.as_ref().clone(),
                SemanticType::List {
                    element: element.clone(),
                },
            ],
            _ => return Err("list match uses a non-list constructor".to_owned()),
        },
        SemanticType::Nat => match branch.constructor.name.as_str() {
            "Nat.zero" => Vec::new(),
            "Nat.succ" => vec![SemanticType::Nat],
            _ => return Err("Nat match uses a non-Nat constructor".to_owned()),
        },
        SemanticType::Option { value } => match branch.constructor.name.as_str() {
            "Option.none" => Vec::new(),
            "Option.some" => vec![value.as_ref().clone()],
            _ => return Err("Option match uses a non-Option constructor".to_owned()),
        },
        SemanticType::Result { ok, error } => match branch.constructor.name.as_str() {
            "Result.error" => vec![error.as_ref().clone()],
            "Result.ok" => vec![ok.as_ref().clone()],
            _ => return Err("Result match uses a non-Result constructor".to_owned()),
        },
        SemanticType::ContractViolation => {
            if branch.constructor.module.is_none()
                && model::VIOLATIONS.contains(&branch.constructor.name.as_str())
            {
                Vec::new()
            } else {
                return Err(
                    "contract-violation match uses a constructor other than a ContractViolation one"
                        .to_owned(),
                );
            }
        }
        SemanticType::ReasoningFailure => {
            if branch.constructor.module.is_none()
                && reasoning::FAILURES.contains(&branch.constructor.name.as_str())
            {
                Vec::new()
            } else {
                return Err(
                    "reasoning-failure match uses a constructor other than a ReasoningFailure one"
                        .to_owned(),
                );
            }
        }
        SemanticType::Product { left, right } => match branch.constructor.name.as_str() {
            "Prod.mk" if branch.constructor.module.is_none() => {
                vec![left.as_ref().clone(), right.as_ref().clone()]
            }
            _ => return Err("product match uses a constructor other than `Prod.mk`".to_owned()),
        },
        SemanticType::Named { member, arguments } => {
            let Some((owner, fields)) = constructor_signature(&branch.constructor, arguments, env)?
            else {
                return Err(format!(
                    "unknown match constructor `{}`",
                    branch.constructor.name
                ));
            };
            let expected_owner = SemanticType::Named {
                member: member.clone(),
                arguments: arguments.clone(),
            };
            require_type(Some(owner), &expected_owner, "match constructor")?;
            fields
        }
        other => {
            return Err(format!("cannot pattern match value of type {other}"));
        }
    })
}

#[allow(clippy::too_many_lines)]
fn infer_term(
    term: &SemanticTerm,
    locals: &BTreeMap<String, SemanticType>,
    env: &Environment<'_>,
) -> Result<Option<SemanticType>, String> {
    let infer = |term| infer_term(term, locals, env);
    match term {
        SemanticTerm::Var { name } => locals
            .get(name)
            .cloned()
            .map(Some)
            .ok_or_else(|| format!("unbound typed local `{name}`")),
        SemanticTerm::Nat { .. } => Ok(Some(SemanticType::Nat)),
        SemanticTerm::Integer { representation, .. } => Ok(Some(representation.semantic_type())),
        SemanticTerm::String { .. } => Ok(Some(SemanticType::String)),
        SemanticTerm::Bytes { .. } => Ok(Some(SemanticType::Bytes)),
        SemanticTerm::Primitive {
            operation,
            arguments,
            result,
        } => {
            let argument_types = arguments.iter().map(infer).collect::<Result<Vec<_>, _>>()?;
            infer_primitive(*operation, &argument_types, result).map(Some)
        }
        SemanticTerm::Bool { .. } => Ok(Some(SemanticType::Bool)),
        SemanticTerm::Unit => Ok(Some(SemanticType::Unit)),
        SemanticTerm::Nil { element } => Ok(Some(SemanticType::List {
            element: Box::new(element.clone()),
        })),
        SemanticTerm::Cons { head, tail } => {
            let head = infer(head)?;
            let tail = infer(tail)?;
            match tail {
                Some(SemanticType::List { element }) => {
                    require_type(head, &element, "list head")?;
                    Ok(Some(SemanticType::List { element }))
                }
                Some(other) => Err(format!("list tail has non-list type {other}")),
                None => Ok(None),
            }
        }
        SemanticTerm::Record {
            r#type,
            type_arguments,
            fields,
        } => {
            let info = type_info(r#type, env)
                .ok_or_else(|| format!("missing record type `{}`", member_key(r#type)))?;
            let map = substitutions(info, type_arguments);
            for (assignment, expected) in fields.iter().zip(&info.field_types) {
                require_type(
                    infer(&assignment.value)?,
                    &substitute_type(expected, &map),
                    &format!("record field `{}.{}`", r#type.name, assignment.field),
                )?;
            }
            Ok(Some(SemanticType::Named {
                member: r#type.clone(),
                arguments: type_arguments.clone(),
            }))
        }
        SemanticTerm::Constructor {
            constructor,
            type_arguments,
            arguments,
        } => {
            let Some((result, expected)) = constructor_signature(constructor, type_arguments, env)?
            else {
                return Ok(None);
            };
            for (argument, expected) in arguments.iter().zip(expected) {
                require_type(
                    infer(argument)?,
                    &expected,
                    &format!("constructor `{}` argument", constructor.name),
                )?;
            }
            Ok(Some(result))
        }
        SemanticTerm::InstanceValue {
            class,
            arguments,
            resolved,
        } => {
            let key = format!("{}:{arguments:?}", member_key(class));
            match env.instances.get(&key) {
                Some(instance) if instance == resolved => Ok(Some(SemanticType::Named {
                    member: class.clone(),
                    arguments: arguments.clone(),
                })),
                Some(instance) => Err(format!(
                    "instance requirement `{key}` resolves to `{}`, not `{}`",
                    member_key(instance),
                    member_key(resolved)
                )),
                None => Err(format!("missing instance for requirement `{key}`")),
            }
        }
        SemanticTerm::Project { value, field } => {
            let Some(SemanticType::Named { member, arguments }) = infer(value)? else {
                return Err(format!(
                    "projection `.{field}` requires a known record value"
                ));
            };
            let info = type_info(&member, env)
                .ok_or_else(|| format!("projection owner `{}` is missing", member_key(&member)))?;
            let index = info
                .fields
                .iter()
                .position(|candidate| candidate == field)
                .ok_or_else(|| format!("record `{}` has no field `{field}`", member.name))?;
            Ok(Some(substitute_type(
                &info.field_types[index],
                &substitutions(info, &arguments),
            )))
        }
        SemanticTerm::Call {
            function,
            type_arguments,
            arguments,
        } => {
            let info = function_info(function, env)
                .ok_or_else(|| format!("missing typed function `{}`", member_key(function)))?;
            let (parameters, result) = instantiate(info, type_arguments);
            for (argument, expected) in arguments.iter().zip(&parameters) {
                require_type(
                    infer(argument)?,
                    expected,
                    &format!("function `{}` argument", function.name),
                )?;
            }
            Ok(Some(result))
        }
        SemanticTerm::Lambda {
            parameters, body, ..
        } => {
            let mut nested = locals.clone();
            for parameter in parameters {
                nested.insert(parameter.name.clone(), parameter.r#type.clone());
            }
            Ok(
                infer_term(body, &nested, env)?.map(|result| SemanticType::Function {
                    parameters: parameters
                        .iter()
                        .map(|parameter| parameter.r#type.clone())
                        .collect(),
                    result: Box::new(result),
                }),
            )
        }
        SemanticTerm::Apply {
            function,
            arguments,
        } => match infer(function)? {
            Some(SemanticType::Function { parameters, result }) => {
                if parameters.len() != arguments.len() {
                    return Err(format!(
                        "application expects {} argument(s), received {}",
                        parameters.len(),
                        arguments.len()
                    ));
                }
                for (argument, expected) in arguments.iter().zip(&parameters) {
                    require_type(infer(argument)?, expected, "application argument")?;
                }
                Ok(Some(*result))
            }
            Some(other) => Err(format!(
                "application of a non-function value of type {other}"
            )),
            // An application is checked against its head's function type;
            // with none known, its arity and arguments would go unchecked.
            None => Err("application of a value with no statically known function type".to_owned()),
        },
        SemanticTerm::FunctionRef {
            function,
            type_arguments,
        } => {
            let info = function_info(function, env)
                .ok_or_else(|| format!("missing typed function `{}`", member_key(function)))?;
            let (parameters, result) = instantiate(info, type_arguments);
            Ok(Some(SemanticType::Function {
                parameters,
                result: Box::new(result),
            }))
        }
        SemanticTerm::MapLiteral {
            key,
            value,
            entries,
        } => {
            for entry in entries {
                require_type(infer(&entry.key)?, key, "map literal key")?;
                require_type(infer(&entry.value)?, value, "map literal value")?;
            }
            Ok(Some(SemanticType::Map {
                key: Box::new(key.clone()),
                value: Box::new(value.clone()),
            }))
        }
        SemanticTerm::SetLiteral { element, elements } => {
            for item in elements {
                require_type(infer(item)?, element, "set literal element")?;
            }
            Ok(Some(SemanticType::Set {
                element: Box::new(element.clone()),
            }))
        }
        SemanticTerm::GraphLiteral { node, nodes, edges } => {
            for item in nodes {
                require_type(infer(item)?, node, "graph literal node")?;
            }
            for edge in edges {
                require_type(infer(&edge.source)?, node, "graph edge source")?;
                require_type(infer(&edge.target)?, node, "graph edge target")?;
            }
            Ok(Some(graph_type(node)))
        }
        SemanticTerm::If {
            condition,
            then_value,
            else_value,
        } => {
            require_type(infer(condition)?, &SemanticType::Bool, "if condition")?;
            same_type(infer(then_value)?, infer(else_value)?, "if branches")
        }
        SemanticTerm::Match {
            scrutinee,
            branches,
        } => {
            let scrutinee_type = infer(scrutinee)?;
            let mut result = None;
            for branch in branches {
                let mut branch_locals = locals.clone();
                let Some(scrutinee_type) = &scrutinee_type else {
                    return Ok(None);
                };
                let binder_types = branch_binder_types(scrutinee_type, branch, env)?;
                for (binder, binder_type) in branch.binders.iter().zip(binder_types) {
                    branch_locals.insert(binder.clone(), binder_type);
                }
                result = same_type(
                    result,
                    infer_term(&branch.body, &branch_locals, env)?,
                    "match branches",
                )?;
            }
            Ok(result)
        }
        SemanticTerm::Eq { left, right } => {
            let _ = same_type(infer(left)?, infer(right)?, "equality operands")?;
            Ok(Some(SemanticType::Prop))
        }
        SemanticTerm::Le { left, right } | SemanticTerm::Lt { left, right } => {
            require_type(infer(left)?, &SemanticType::Nat, "order left operand")?;
            require_type(infer(right)?, &SemanticType::Nat, "order right operand")?;
            Ok(Some(SemanticType::Prop))
        }
        SemanticTerm::Add { left, right } => {
            require_type(infer(left)?, &SemanticType::Nat, "addition left operand")?;
            require_type(infer(right)?, &SemanticType::Nat, "addition right operand")?;
            Ok(Some(SemanticType::Nat))
        }
        SemanticTerm::Beq { left, right }
        | SemanticTerm::Ble { left, right }
        | SemanticTerm::Blt { left, right } => {
            require_type(
                infer(left)?,
                &SemanticType::Nat,
                "Boolean comparison left operand",
            )?;
            require_type(
                infer(right)?,
                &SemanticType::Nat,
                "Boolean comparison right operand",
            )?;
            Ok(Some(SemanticType::Bool))
        }
        SemanticTerm::And { left, right } | SemanticTerm::Or { left, right } => {
            require_type(
                infer(left)?,
                &SemanticType::Bool,
                "Boolean connective left operand",
            )?;
            require_type(
                infer(right)?,
                &SemanticType::Bool,
                "Boolean connective right operand",
            )?;
            Ok(Some(SemanticType::Bool))
        }
        SemanticTerm::PropAnd { left, right } => {
            require_type(
                infer(left)?,
                &SemanticType::Prop,
                "propositional conjunction left operand",
            )?;
            require_type(
                infer(right)?,
                &SemanticType::Prop,
                "propositional conjunction right operand",
            )?;
            Ok(Some(SemanticType::Prop))
        }
        SemanticTerm::Not { value } => {
            require_type(
                infer(value)?,
                &SemanticType::Bool,
                "Boolean negation operand",
            )?;
            Ok(Some(SemanticType::Bool))
        }
        SemanticTerm::Implies {
            premise,
            conclusion,
        }
        | SemanticTerm::Iff {
            left: premise,
            right: conclusion,
        } => {
            require_type(
                infer(premise)?,
                &SemanticType::Prop,
                "proposition left operand",
            )?;
            require_type(
                infer(conclusion)?,
                &SemanticType::Prop,
                "proposition right operand",
            )?;
            Ok(Some(SemanticType::Prop))
        }
        SemanticTerm::Forall { binder, body } => {
            let mut nested = locals.clone();
            nested.insert(binder.name.clone(), binder.r#type.clone());
            require_type(
                infer_term(body, &nested, env)?,
                &SemanticType::Prop,
                "forall body",
            )?;
            Ok(Some(SemanticType::Prop))
        }
        SemanticTerm::Let {
            binder,
            value,
            body,
        } => {
            require_type(
                infer(value)?,
                &binder.r#type,
                &format!("let binder `{}`", binder.name),
            )?;
            let mut nested = locals.clone();
            nested.insert(binder.name.clone(), binder.r#type.clone());
            infer_term(body, &nested, env)
        }
        SemanticTerm::Pair { left, right } => match (infer(left)?, infer(right)?) {
            (Some(left), Some(right)) => Ok(Some(SemanticType::Product {
                left: Box::new(left),
                right: Box::new(right),
            })),
            _ => Ok(None),
        },
        SemanticTerm::First { value } | SemanticTerm::Second { value } => match infer(value)? {
            Some(SemanticType::Product { left, right }) => {
                Ok(Some(if matches!(term, SemanticTerm::First { .. }) {
                    *left
                } else {
                    *right
                }))
            }
            Some(other) => Err(format!("pair projection of non-product type {other}")),
            None => Ok(None),
        },
        SemanticTerm::CheckedApply { .. } => {
            Err("internal: a checked model application reached ordinary typing".to_owned())
        }
    }
}

fn check_proof(
    proof: &SemanticProof,
    locals: &BTreeMap<String, SemanticType>,
    env: &Environment<'_>,
) -> Result<(), String> {
    match proof {
        SemanticProof::Reflexivity | SemanticProof::Decide | SemanticProof::Congruence => Ok(()),
        SemanticProof::LinearArithmetic { definitions } => {
            require_language_1_2(env, "linear_arithmetic proof")?;
            if definitions
                .windows(2)
                .any(|pair| member_key(&pair[0]) >= member_key(&pair[1]))
            {
                return Err("linear_arithmetic definitions are not sorted and unique".to_owned());
            }
            for definition in definitions {
                check_member(definition, env)?;
                if function_info(definition, env).is_none() {
                    return Err(format!(
                        "linear_arithmetic unfolds `{}`, which is not a prior document definition",
                        member_key(definition)
                    ));
                }
            }
            Ok(())
        }
        SemanticProof::BooleanReflection { reflection } => {
            check_boolean_reflection(reflection, locals, env)
        }
        SemanticProof::Apply {
            theorem,
            type_arguments,
            arguments,
        } => {
            check_member(theorem, env)?;
            if !type_arguments.is_empty() {
                require_language_1_2(env, "theorem type arguments")?;
            }
            let parameters = env.proof_rules.get(&member_key(theorem)).ok_or_else(|| {
                format!(
                    "proof application names a forward or missing theorem `{}`",
                    member_key(theorem)
                )
            })?;
            let type_parameters = env
                .proof_type_parameters
                .get(&member_key(theorem))
                .cloned()
                .unwrap_or_default();
            if type_parameters.len() != type_arguments.len() {
                return Err(format!(
                    "theorem `{}` expects {} explicit type argument(s), received {}",
                    member_key(theorem),
                    type_parameters.len(),
                    type_arguments.len()
                ));
            }
            for argument in type_arguments {
                check_type_argument(argument, env)?;
            }
            let instantiation: BTreeMap<String, SemanticType> = type_parameters
                .into_iter()
                .zip(type_arguments.iter().cloned())
                .collect();
            let parameters: Vec<SemanticType> = parameters
                .iter()
                .map(|parameter| substitute_type(parameter, &instantiation))
                .collect();
            let parameters = &parameters;
            if parameters.len() != arguments.len() {
                return Err(format!(
                    "theorem `{}` expects {} argument(s), received {}",
                    member_key(theorem),
                    parameters.len(),
                    arguments.len()
                ));
            }
            let local_names = locals.keys().cloned().collect();
            for (argument, expected) in arguments.iter().zip(parameters) {
                check_term(argument, &local_names, env, None, &BTreeSet::new())?;
                require_type(
                    infer_term(argument, locals, env)?,
                    expected,
                    "theorem application argument",
                )?;
            }
            Ok(())
        }
        SemanticProof::Simplify { definitions } => {
            if definitions.is_empty() {
                return Err("simplify requires at least one named definition".to_owned());
            }
            let mut previous = None;
            for definition in definitions {
                check_member(definition, env)?;
                let builtin_bridge = definition.module.is_none()
                    && matches!(
                        definition.name.as_str(),
                        "Bool.and_eq_true" | "Nat.beq_eq" | "Nat.blt_eq"
                    );
                let local_hypothesis = definition.module.is_none()
                    && locals.get(&definition.name) == Some(&SemanticType::Prop);
                if function_info(definition, env).is_none()
                    && !env.proof_rules.contains_key(&member_key(definition))
                    && !builtin_bridge
                    && !local_hypothesis
                {
                    return Err(format!(
                        "simplify names a forward or missing definition `{}`",
                        member_key(definition)
                    ));
                }
                if previous.is_some_and(|prior: &MemberRef| prior >= definition) {
                    return Err("simplify definitions are strictly sorted and unique".to_owned());
                }
                previous = Some(definition);
            }
            Ok(())
        }
        SemanticProof::Constructor { branches } => {
            if branches.is_empty() {
                return Err("constructor proof requires branches".to_owned());
            }
            for branch in branches {
                check_proof(branch, locals, env)?;
            }
            Ok(())
        }
        SemanticProof::Cases {
            scrutinee,
            branches,
        } => check_elimination_proof(scrutinee, branches, false, locals, env),
        SemanticProof::Induction {
            scrutinee,
            generalizing,
            branches,
        } => {
            let mut previous = None;
            for name in generalizing {
                check_name(name, "generalized proof binder")?;
                if name == scrutinee || !locals.contains_key(name) {
                    return Err(format!(
                        "generalized proof binder `{name}` is absent or is the scrutinee"
                    ));
                }
                if previous.is_some_and(|prior: &String| prior >= name) {
                    return Err(
                        "generalized proof binders are strictly sorted and unique".to_owned()
                    );
                }
                previous = Some(name);
            }
            check_elimination_proof(scrutinee, branches, true, locals, env)
        }
    }
}

fn check_reflection_definition(
    definition: &MemberRef,
    env: &Environment<'_>,
) -> Result<(), String> {
    check_member(definition, env)?;
    if function_info(definition, env).is_none() {
        return Err(format!(
            "Boolean reflection names a forward or missing definition `{}`",
            member_key(definition)
        ));
    }
    Ok(())
}

fn check_boolean_reflection(
    reflection: &SemanticReflection,
    locals: &BTreeMap<String, SemanticType>,
    env: &Environment<'_>,
) -> Result<(), String> {
    for reserved in [
        "llAndBridge",
        "llBeqBridge",
        "llBeqRefl",
        "llIH",
        "llRest",
        "llValue",
    ] {
        if locals.contains_key(reserved) {
            return Err(format!(
                "Boolean reflection reserves internal binder `{reserved}`"
            ));
        }
    }
    match reflection {
        SemanticReflection::List {
            parameter,
            values,
            boolean_definition,
            proposition_definition,
            ..
        } => {
            if locals.get(parameter) != Some(&SemanticType::Nat) {
                return Err(format!(
                    "Boolean list reflection parameter `{parameter}` is not an in-scope Nat"
                ));
            }
            if locals.get(values)
                != Some(&SemanticType::List {
                    element: Box::new(SemanticType::Nat),
                })
            {
                return Err(format!(
                    "Boolean list reflection value `{values}` is not an in-scope List Nat"
                ));
            }
            check_reflection_definition(boolean_definition, env)?;
            check_reflection_definition(proposition_definition, env)
        }
        SemanticReflection::Record {
            record,
            boolean_definition,
            proposition_definition,
            fields,
        } => {
            let SemanticType::Named { member, .. } = locals
                .get(record)
                .ok_or_else(|| format!("Boolean record reflection local `{record}` is absent"))?
            else {
                return Err(format!(
                    "Boolean record reflection local `{record}` is not a document structure"
                ));
            };
            let info = type_info(member, env).ok_or_else(|| {
                format!(
                    "Boolean record reflection type `{}` is absent",
                    member_key(member)
                )
            })?;
            if info.class
                || info.fields.is_empty()
                || info.fields.len() != fields.len()
                || info
                    .field_types
                    .iter()
                    .any(|field| field != &SemanticType::Nat)
                || info
                    .fields
                    .iter()
                    .zip(fields)
                    .any(|(expected, field)| expected != &field.field)
            {
                return Err("Boolean record reflection requires every ordered Nat field".to_owned());
            }
            for field in fields {
                check_name(&field.field, "reflected record field")?;
                if field.expected != "0"
                    && (!field.expected.bytes().all(|byte| byte.is_ascii_digit())
                        || field.expected.starts_with('0'))
                {
                    return Err(format!(
                        "Boolean record reflection has noncanonical Nat literal `{}`",
                        field.expected
                    ));
                }
            }
            check_reflection_definition(boolean_definition, env)?;
            check_reflection_definition(proposition_definition, env)
        }
    }
}

fn check_elimination_proof(
    scrutinee: &str,
    branches: &[SemanticProofBranch],
    induction: bool,
    locals: &BTreeMap<String, SemanticType>,
    env: &Environment<'_>,
) -> Result<(), String> {
    let ty = locals
        .get(scrutinee)
        .ok_or_else(|| format!("proof scrutinee `{scrutinee}` is absent"))?;
    let expected: BTreeMap<String, Vec<SemanticType>> = match ty {
        SemanticType::List { element } => BTreeMap::from([
            (
                "cons".to_owned(),
                if induction {
                    vec![element.as_ref().clone(), ty.clone(), SemanticType::Prop]
                } else {
                    vec![element.as_ref().clone(), ty.clone()]
                },
            ),
            ("nil".to_owned(), Vec::new()),
        ]),
        SemanticType::Nat => BTreeMap::from([
            (
                "succ".to_owned(),
                if induction {
                    vec![SemanticType::Nat, SemanticType::Prop]
                } else {
                    vec![SemanticType::Nat]
                },
            ),
            ("zero".to_owned(), Vec::new()),
        ]),
        SemanticType::Named { member, arguments } => {
            let info = type_info(member, env).ok_or_else(|| {
                format!("proof scrutinee `{scrutinee}` has an unavailable inductive type")
            })?;
            if info.constructors.is_empty() || !info.fields.is_empty() {
                return Err(format!(
                    "proof scrutinee `{scrutinee}` does not have an inductive type"
                ));
            }
            if induction && env.language_1_2 && info.nested_or_mutual {
                return Err(format!(
                    "induction on `{scrutinee}` requires a self-recursive inductive without nested or mutual occurrences; use cases"
                ));
            }
            let substitutions = substitutions(info, arguments);
            info.constructor_types
                .iter()
                .map(|(name, fields)| {
                    let mut binders: Vec<SemanticType> = fields
                        .iter()
                        .map(|field| substitute_type(field, &substitutions))
                        .collect();
                    // §17.12: induction binds one hypothesis per direct
                    // recursive field, after the fields, in field order.
                    if induction && env.language_1_2 {
                        let hypotheses = info
                            .direct_recursive
                            .get(name)
                            .map_or(0, |flags| flags.iter().filter(|direct| **direct).count());
                        binders.extend(std::iter::repeat_n(SemanticType::Prop, hypotheses));
                    }
                    (name.rsplit('.').next().unwrap_or(name).to_owned(), binders)
                })
                .collect()
        }
        _ => {
            return Err(format!(
                "proof scrutinee `{scrutinee}` is not Nat, List, or a document inductive"
            ));
        }
    };
    let mut observed = BTreeMap::new();
    for branch in branches {
        check_name(&branch.constructor, "proof constructor")?;
        if observed
            .insert(branch.constructor.clone(), branch.binders.len())
            .is_some()
        {
            return Err(format!("duplicate proof branch `{}`", branch.constructor));
        }
        let expected_binders = expected.get(&branch.constructor).ok_or_else(|| {
            format!(
                "proof branch `{}` is not a constructor of `{scrutinee}`",
                branch.constructor
            )
        })?;
        if branch.binders.len() != expected_binders.len() {
            return Err(format!(
                "proof branch `{}` expects {} binder(s), received {}",
                branch.constructor,
                expected_binders.len(),
                branch.binders.len()
            ));
        }
        let mut nested = locals.clone();
        for (binder, binder_type) in branch.binders.iter().zip(expected_binders) {
            check_name(binder, "proof binder")?;
            if nested.insert(binder.clone(), binder_type.clone()).is_some() {
                return Err(format!("shadowed proof binder `{binder}`"));
            }
        }
        check_proof(&branch.proof, &nested, env)?;
    }
    if observed.keys().collect::<Vec<_>>() != expected.keys().collect::<Vec<_>>() {
        return Err(format!(
            "proof branches are not exhaustive: expected {:?}, observed {:?}",
            expected.keys().collect::<Vec<_>>(),
            observed.keys().collect::<Vec<_>>()
        ));
    }
    Ok(())
}

impl SemanticModule {
    /// Decode canonical JSON and enforce all conservative semantic checks.
    /// `module_prefix` is the project's Lean module prefix, under which the
    /// backend names every imported declaration.
    pub fn parse(
        text: &str,
        language: &str,
        module_prefix: &str,
        imports: &[String],
        imported_modules: &BTreeMap<String, &Self>,
        artifacts: &model::ArtifactStore,
    ) -> Result<Self, SemanticFailure> {
        let module: Self = serde_json::from_str(text)
            .map_err(|error| format!("invalid semantic-module JSON: {error}"))?;
        let canonical =
            crate::artifact::canonical_json::Json::parse(text.as_bytes())?.to_canonical_string();
        if canonical != text {
            return Err("semantic-module JSON is not canonical".to_owned().into());
        }
        // Serde ignores members of a tagged unit variant (`{"kind":"nat",
        // "x":1}`), so the closed schema is enforced here: every source
        // member must survive into the typed value, except an explicitly
        // written default (an empty list or `false`) that serialization
        // omits.
        let source: serde_json::Value = serde_json::from_str(text)
            .map_err(|error| format!("invalid semantic-module JSON: {error}"))?;
        let typed = serde_json::to_value(&module).expect("semantic module serializes");
        if let Some(path) = ignored_member(&source, &typed, "$") {
            return Err(format!(
                "semantic-module JSON member `{path}` is outside the closed schema"
            )
            .into());
        }
        let elaboration = module.validate(
            language,
            module_prefix,
            imports,
            imported_modules,
            artifacts,
        )?;
        let mut module = module;
        module.elaboration = elaboration;
        if language == crate::LANGUAGE_1_2 {
            for declaration in &mut module.declarations {
                declaration_terms_mut(declaration, &mut normalize_collection);
            }
            module.elaboration.normalize(&mut normalize_collection);
        }
        Ok(module)
    }

    /// The first language-1.2-only construct in this module, by kind: a
    /// declaration form, a type, or a term, in declaration order.
    #[must_use]
    pub fn first_language_1_2_construct(&self) -> Option<&'static str> {
        let mut found = None;
        for declaration in &self.declarations {
            if found.is_some() {
                break;
            }
            if let SemanticDeclaration::Inductive {
                mutual: Some(_), ..
            } = declaration
            {
                found = Some("mutual inductive group");
                break;
            }
            if let Some(kind) = model::elaborated_construct(declaration) {
                found = Some(kind);
                break;
            }
            declaration_types(declaration, &mut |ty| {
                if found.is_none() {
                    found = language_1_2_type(ty);
                }
            });
            declaration_terms(declaration, &mut |term| {
                if found.is_none() {
                    found = language_1_2_construct(term);
                }
                if found.is_none() {
                    term_types(term, &mut |ty| {
                        if found.is_none() {
                            found = language_1_2_type(ty);
                        }
                    });
                }
            });
        }
        found
    }

    fn validate(
        &self,
        language: &str,
        module_prefix: &str,
        imports: &[String],
        imported_modules: &BTreeMap<String, &Self>,
        artifacts: &model::ArtifactStore,
    ) -> Result<model::Elaboration, SemanticFailure> {
        // §17.12: the discriminator is routed by the project language, never
        // inferred from the module, so one module cannot mean two things.
        let expected = semantic_module_spec(language)
            .ok_or_else(|| format!("language {language} has no semantic-module schema"))?;
        if self.spec != expected {
            return Err(format!(
                "unsupported semantic-module schema `{}`; language {language} requires `{expected}`",
                self.spec
            ).into());
        }
        if language == crate::LANGUAGE_1_1 {
            if let Some(construct) = self.first_language_1_2_construct() {
                return Err(format!(
                    "`{construct}` is a language-1.2 construct; language 1.1 rejects it"
                )
                .into());
            }
        }
        if self.declarations.is_empty() {
            return Err("a semantic module contains at least one declaration"
                .to_owned()
                .into());
        }
        if language == crate::LANGUAGE_1_2 {
            check_binder_hygiene(self, module_prefix)?;
        }
        let mut env = Environment {
            language_1_2: language == crate::LANGUAGE_1_2,
            imports,
            ..Environment::default()
        };
        #[allow(clippy::type_complexity)]
        let mut imported_flags: BTreeMap<
            String,
            (BTreeMap<String, Vec<bool>>, bool, Vec<String>),
        > = BTreeMap::new();
        for import in imports {
            let Some(module) = imported_modules.get(import) else {
                continue;
            };
            // Imported recursive groups keep their structurally-smaller
            // positions, so recursion over them follows the same rule as
            // recursion over a local group.
            let mut imported_groups: BTreeMap<Option<&str>, Vec<InductiveRow<'_>>> =
                BTreeMap::new();
            // An imported module is visible through its elaboration: every
            // model declaration as the ordinary declarations it means.
            let lowered = module.lowered_declarations();
            for declaration in lowered.iter().copied() {
                if let SemanticDeclaration::Inductive {
                    name,
                    type_parameters,
                    parameters,
                    constructors,
                    mutual,
                } = declaration
                {
                    let row = InductiveRow {
                        name,
                        type_parameters,
                        parameters,
                        constructors,
                    };
                    match mutual {
                        Some(label) => imported_groups.entry(Some(label)).or_default().push(row),
                        None => {
                            for (member, (flags, nested)) in recursion_flags(&[row]) {
                                let key = format!("{import}::{member}");
                                imported_flags.insert(key.clone(), (flags, nested, vec![key]));
                            }
                        }
                    }
                }
            }
            for rows in imported_groups.values() {
                let members: Vec<String> = rows
                    .iter()
                    .map(|row| format!("{import}::{}", row.name))
                    .collect();
                for (member, (flags, nested)) in recursion_flags(rows) {
                    imported_flags.insert(
                        format!("{import}::{member}"),
                        (flags, nested, members.clone()),
                    );
                }
            }
            for declaration in lowered.iter().copied() {
                let key = format!("{import}::{}", declaration.name());
                match declaration {
                    SemanticDeclaration::Structure {
                        type_parameters,
                        fields,
                        ..
                    }
                    | SemanticDeclaration::Class {
                        type_parameters,
                        fields,
                        ..
                    } => {
                        let field_types: Vec<_> = fields
                            .iter()
                            .map(|field| qualify_type(&field.r#type, import))
                            .collect();
                        env.types.insert(
                            key,
                            TypeInfo {
                                parameters: type_parameters.len(),
                                fields: fields.iter().map(|field| field.name.clone()).collect(),
                                constructors: BTreeMap::from([(
                                    format!("{}.mk", declaration.name()),
                                    fields.len(),
                                )]),
                                type_parameters: type_parameters.clone(),
                                field_types: field_types.clone(),
                                constructor_types: BTreeMap::from([(
                                    format!("{}.mk", declaration.name()),
                                    field_types,
                                )]),
                                class: matches!(declaration, SemanticDeclaration::Class { .. }),
                                ..TypeInfo::default()
                            },
                        );
                    }
                    SemanticDeclaration::Inductive {
                        type_parameters,
                        constructors,
                        ..
                    } => {
                        env.types.insert(
                            key,
                            TypeInfo {
                                parameters: type_parameters.len(),
                                fields: Vec::new(),
                                constructors: constructors
                                    .iter()
                                    .map(|constructor| {
                                        (
                                            format!("{}.{}", declaration.name(), constructor.name),
                                            constructor.fields.len(),
                                        )
                                    })
                                    .collect(),
                                type_parameters: type_parameters.clone(),
                                field_types: Vec::new(),
                                constructor_types: constructors
                                    .iter()
                                    .map(|constructor| {
                                        (
                                            format!("{}.{}", declaration.name(), constructor.name),
                                            constructor
                                                .fields
                                                .iter()
                                                .map(|field| qualify_type(field, import))
                                                .collect(),
                                        )
                                    })
                                    .collect(),
                                class: false,
                                ..TypeInfo::default()
                            },
                        );
                    }
                    SemanticDeclaration::Definition {
                        type_parameters,
                        parameters,
                        result,
                        executable,
                        ..
                    } => {
                        env.functions.insert(
                            key,
                            FunctionInfo {
                                type_parameters: type_parameters.clone(),
                                parameters: parameters
                                    .iter()
                                    .map(|parameter| qualify_type(&parameter.r#type, import))
                                    .collect(),
                                result: qualify_type(result, import),
                                executable: *executable,
                            },
                        );
                    }
                    SemanticDeclaration::Instance {
                        name,
                        class,
                        arguments,
                        ..
                    } => {
                        let class = MemberRef {
                            module: class.module.clone().or_else(|| Some(import.clone())),
                            name: class.name.clone(),
                        };
                        let arguments = arguments
                            .iter()
                            .map(|argument| qualify_type(argument, import))
                            .collect::<Vec<_>>();
                        let key = format!("{}:{arguments:?}", member_key(&class));
                        let instance = MemberRef {
                            module: Some(import.clone()),
                            name: name.clone(),
                        };
                        if env.instances.insert(key.clone(), instance).is_some() {
                            return Err(format!(
                                "ambiguous imported instances for requirement `{key}`"
                            )
                            .into());
                        }
                    }
                    SemanticDeclaration::Theorem {
                        type_parameters,
                        parameters,
                        ..
                    } => {
                        env.proof_type_parameters
                            .insert(key.clone(), type_parameters.clone());
                        env.proof_rules.insert(
                            key,
                            parameters
                                .iter()
                                .map(|parameter| qualify_type(&parameter.r#type, import))
                                .collect(),
                        );
                    }
                    SemanticDeclaration::Artifact { .. }
                    | SemanticDeclaration::Contract { .. }
                    | SemanticDeclaration::Realization { .. }
                    | SemanticDeclaration::Evidence { .. }
                    | SemanticDeclaration::Model { .. }
                    | SemanticDeclaration::Logic { .. }
                    | SemanticDeclaration::InferenceRule { .. }
                    | SemanticDeclaration::Verifier { .. }
                    | SemanticDeclaration::Reasoner { .. } => {
                        return Err(format!("internal: imported `{key}` was not elaborated").into());
                    }
                }
            }
            model::register_import(import, &module.elaboration, &mut env.models);
            reasoning::register_import(import, &module.elaboration, &mut env);
        }
        for (key, (flags, nested, members)) in imported_flags {
            if let Some(info) = env.types.get_mut(&key) {
                info.direct_recursive = flags;
                info.nested_or_mutual = nested;
                info.group = members;
            }
        }
        // §17.12: a mutual group is one contiguous run of inductives sharing
        // a label; a label may not reappear after its run ends.
        let mut closed_groups = BTreeSet::new();
        let mut open_group: Option<&str> = None;
        for declaration in &self.declarations {
            let label = match declaration {
                SemanticDeclaration::Inductive {
                    mutual: Some(label),
                    ..
                }
                | SemanticDeclaration::Definition {
                    mutual: Some(label),
                    ..
                } => Some(label.as_str()),
                _ => None,
            };
            if label != open_group {
                if let Some(previous) = open_group {
                    closed_groups.insert(previous);
                }
                if let Some(label) = label {
                    if closed_groups.contains(label) {
                        return Err(format!("mutual group `{label}` is not contiguous").into());
                    }
                }
                open_group = label;
            }
        }
        // A label names one group of one kind. An inductive group and a
        // definition group sharing a label would be admitted together by the
        // first member of either, and the other's members never checked.
        let mut inductive_labels = BTreeSet::new();
        let mut definition_labels = BTreeSet::new();
        for declaration in &self.declarations {
            match declaration {
                SemanticDeclaration::Inductive {
                    mutual: Some(label),
                    ..
                } => {
                    inductive_labels.insert(label.as_str());
                }
                SemanticDeclaration::Definition {
                    mutual: Some(label),
                    ..
                } => {
                    definition_labels.insert(label.as_str());
                }
                _ => {}
            }
        }
        if let Some(label) = inductive_labels.intersection(&definition_labels).next() {
            return Err(format!(
                "mutual label `{label}` names both an inductive group and a definition group"
            )
            .into());
        }
        let mut generated_names = BTreeSet::new();
        let mut registered_groups = BTreeSet::new();
        let mut elaboration = model::Builder::new(self.declarations.len());
        for (index, declaration) in self.declarations.iter().enumerate() {
            let name = declaration.name();
            if let SemanticDeclaration::Inductive {
                mutual: Some(label),
                ..
            }
            | SemanticDeclaration::Definition {
                mutual: Some(label),
                ..
            } = declaration
            {
                // A later member was admitted with the first member.
                if env.language_1_2 && registered_groups.contains(label) {
                    continue;
                }
            }
            check_declaration_name(name, &env)?;
            if !generated_names.insert(name.to_owned()) {
                return Err(format!("duplicate generated name `{name}`").into());
            }
            match declaration {
                SemanticDeclaration::Structure { .. } | SemanticDeclaration::Class { .. } => {
                    register_structure(declaration, &mut env, &mut generated_names)?;
                }
                SemanticDeclaration::Inductive { mutual, .. } if env.language_1_2 => {
                    if let Some(label) = mutual {
                        registered_groups.insert(label.clone());
                    }
                    let rows: Vec<InductiveRow<'_>> = self
                        .declarations
                        .iter()
                        .filter_map(|candidate| match candidate {
                            SemanticDeclaration::Inductive {
                                name: member,
                                type_parameters,
                                parameters,
                                constructors,
                                mutual: member_group,
                            } if (mutual.is_none() && member == name)
                                || (mutual.is_some() && member_group == mutual) =>
                            {
                                Some(InductiveRow {
                                    name: member,
                                    type_parameters,
                                    parameters,
                                    constructors,
                                })
                            }
                            _ => None,
                        })
                        .collect();
                    // Later members' own names are registered here so the
                    // whole group is admitted at once, before any use.
                    for row in rows.iter().skip(1) {
                        check_declaration_name(row.name, &env)?;
                        if !generated_names.insert(row.name.to_owned()) {
                            return Err(format!("duplicate generated name `{}`", row.name).into());
                        }
                    }
                    register_inductive_group(
                        &rows,
                        mutual.as_deref(),
                        &mut env,
                        &mut generated_names,
                    )?;
                }
                SemanticDeclaration::Inductive {
                    type_parameters,
                    parameters,
                    constructors,
                    ..
                } => {
                    if !parameters.is_empty() {
                        return Err(format!(
                            "inductive `{name}` value parameters are not part of a finite data declaration"
                        ).into());
                    }
                    let type_parameter_names = type_parameters.clone();
                    check_type_parameter_spelling(
                        &type_parameter_names,
                        &BTreeSet::from([name.to_owned()]),
                        &env,
                    )?;
                    let type_parameters = type_parameter_set(type_parameters)?;
                    let _ = check_parameters(parameters, &env, &type_parameters)?;
                    if constructors.is_empty() {
                        return Err(format!("inductive `{name}` has no constructors").into());
                    }
                    let mut rows = BTreeMap::new();
                    let mut constructor_types = BTreeMap::new();
                    for constructor in constructors {
                        check_name(&constructor.name, "constructor")?;
                        let full = format!("{name}.{}", constructor.name);
                        if !generated_names.insert(full.clone()) {
                            return Err(format!("duplicate generated name `{full}`").into());
                        }
                        for field in &constructor.fields {
                            check_type(field, &env)?;
                            check_type_parameters(field, &type_parameters)?;
                            if matches!(field, SemanticType::Named { member, .. } if member.module.is_none() && member.name == *name)
                            {
                                return Err(format!(
                                    "recursive inductive payload in `{full}` is not permitted"
                                )
                                .into());
                            }
                        }
                        rows.insert(full.clone(), constructor.fields.len());
                        constructor_types.insert(full, constructor.fields.clone());
                    }
                    env.types.insert(
                        name.to_owned(),
                        TypeInfo {
                            parameters: type_parameters.len(),
                            fields: Vec::new(),
                            constructors: rows,
                            type_parameters: type_parameter_names,
                            field_types: Vec::new(),
                            constructor_types,
                            class: false,
                            ..TypeInfo::default()
                        },
                    );
                }
                SemanticDeclaration::Instance { .. } => {
                    let lowered = model::lower_ordinary(declaration, &env)?;
                    // Only elaborated checked applications bind generated
                    // names; the source binders were checked as source.
                    env.derived = true;
                    let SemanticDeclaration::Instance {
                        class,
                        arguments,
                        priority,
                        fields,
                        ..
                    } = &lowered
                    else {
                        return Err(
                            format!("internal: instance `{name}` lowered to another kind").into(),
                        );
                    };
                    check_member(class, &env)?;
                    if *priority != 1000 {
                        return Err(
                            format!("instance `{name}` priority must be exactly 1000").into()
                        );
                    }
                    for argument in arguments {
                        check_type(argument, &env)?;
                    }
                    let info = type_info(class, &env)
                        .filter(|info| info.class)
                        .cloned()
                        .ok_or_else(|| {
                            format!("instance `{name}` targets a missing or non-class type")
                        })?;
                    if info.parameters != arguments.len() {
                        return Err(format!("instance `{name}` has the wrong class arity").into());
                    }
                    let key = format!("{}:{arguments:?}", member_key(class));
                    if env.instances.contains_key(&key) {
                        return Err(
                            format!("ambiguous duplicate instance for `{}`", class.name).into()
                        );
                    }
                    check_assignments(
                        fields,
                        &info.fields,
                        &BTreeSet::new(),
                        &env,
                        None,
                        &BTreeSet::new(),
                    )?;
                    let map = substitutions(&info, arguments);
                    for (assignment, expected) in fields.iter().zip(&info.field_types) {
                        require_type(
                            infer_term(&assignment.value, &BTreeMap::new(), &env)?,
                            &substitute_type(expected, &map),
                            &format!("instance field `{}.{}`", name, assignment.field),
                        )?;
                    }
                    env.instances.insert(
                        key,
                        MemberRef {
                            module: None,
                            name: name.to_owned(),
                        },
                    );
                    env.derived = false;
                    elaboration.lower(index, vec![lowered.clone()]);
                }
                SemanticDeclaration::Definition {
                    mutual: Some(label),
                    ..
                } if env.language_1_2 => {
                    registered_groups.insert(label.clone());
                    let rows: Vec<&SemanticDeclaration> = self
                        .declarations
                        .iter()
                        .filter(|candidate| {
                            matches!(candidate, SemanticDeclaration::Definition { mutual: Some(other), .. } if other == label)
                        })
                        .collect();
                    for row in rows.iter().skip(1) {
                        check_declaration_name(row.name(), &env)?;
                        if !generated_names.insert(row.name().to_owned()) {
                            return Err(format!("duplicate generated name `{}`", row.name()).into());
                        }
                    }
                    let lowered = rows
                        .iter()
                        .map(|row| model::lower_ordinary(row, &env))
                        .collect::<Result<Vec<_>, _>>()?;
                    env.derived = true;
                    let checked = check_definition_group(
                        &lowered.iter().collect::<Vec<_>>(),
                        label,
                        &mut env,
                    );
                    env.derived = false;
                    checked?;
                    for (position, row) in self.declarations.iter().enumerate() {
                        if let Some(at) = rows
                            .iter()
                            .position(|candidate| std::ptr::eq(*candidate, row))
                        {
                            elaboration.lower(position, vec![lowered[at].clone()]);
                        }
                    }
                }
                SemanticDeclaration::Definition { .. } => {
                    let lowered = model::lower_ordinary(declaration, &env)?;
                    env.derived = true;
                    let checked = check_definition(&lowered, &mut env);
                    env.derived = false;
                    checked?;
                    elaboration.lower(index, vec![lowered]);
                }
                SemanticDeclaration::Theorem { .. } => {
                    let lowered = model::lower_ordinary(declaration, &env)?;
                    env.derived = true;
                    let checked = check_theorem(&lowered, &mut env);
                    env.derived = false;
                    checked?;
                    elaboration.lower(index, vec![lowered]);
                }
                SemanticDeclaration::Artifact { .. }
                | SemanticDeclaration::Contract { .. }
                | SemanticDeclaration::Realization { .. }
                | SemanticDeclaration::Evidence { .. }
                | SemanticDeclaration::Model { .. } => {
                    let lowering = model::check_declaration(
                        declaration,
                        &mut env,
                        artifacts,
                        &mut generated_names,
                    )?;
                    elaboration.record(index, lowering);
                }
                SemanticDeclaration::Logic { .. }
                | SemanticDeclaration::InferenceRule { .. }
                | SemanticDeclaration::Verifier { .. }
                | SemanticDeclaration::Reasoner { .. } => {
                    let lowering = reasoning::check_declaration(
                        declaration,
                        &mut env,
                        artifacts,
                        &mut generated_names,
                    )?;
                    elaboration.record(index, lowering);
                }
            }
        }
        elaboration.finish(
            &self.declarations,
            env.models.locals(),
            env.reasoning.locals(),
        )
    }
}

/// Register one structure or class: its type parameters, fields, and
/// constructor, each generated name unique (§17.12). A reasoner's generated
/// records are registered by the same rule.
fn register_structure(
    declaration: &SemanticDeclaration,
    env: &mut Environment<'_>,
    generated_names: &mut BTreeSet<String>,
) -> Result<(), String> {
    let (SemanticDeclaration::Structure {
        name,
        type_parameters,
        parameters,
        fields,
    }
    | SemanticDeclaration::Class {
        name,
        type_parameters,
        parameters,
        fields,
    }) = declaration
    else {
        return Ok(());
    };
    let name = name.as_str();
    if !parameters.is_empty() {
        return Err(format!(
            "`{name}` value parameters are not part of a finite data declaration"
        ));
    }
    let type_parameter_names = type_parameters.clone();
    check_type_parameter_spelling(
        &type_parameter_names,
        &BTreeSet::from([name.to_owned()]),
        env,
    )?;
    let type_parameters = type_parameter_set(type_parameters)?;
    let _ = check_parameters(parameters, env, &type_parameters)?;
    if fields.is_empty() {
        return Err(format!("`{name}` has no fields"));
    }
    if env.language_1_2 {
        let own = BTreeSet::from([name.to_owned()]);
        if fields
            .iter()
            .any(|field| mentions_group(&field.r#type, &own))
        {
            return Err(format!(
                    "structure or class `{name}` refers to itself; recursive data is declared as an inductive"
                ));
        }
    }
    let mut field_names = Vec::new();
    for field in fields {
        check_name(&field.name, "field")?;
        check_type(&field.r#type, env)?;
        check_type_parameters(&field.r#type, &type_parameters)?;
        if field_names.contains(&field.name) {
            return Err(format!("duplicate field `{}.{}`", name, field.name));
        }
        field_names.push(field.name.clone());
        let generated = format!("{name}.{}", field.name);
        if !generated_names.insert(generated.clone()) {
            return Err(format!("duplicate generated name `{generated}`"));
        }
    }
    let constructor = format!("{name}.mk");
    if !generated_names.insert(constructor.clone()) {
        return Err(format!("duplicate generated name `{constructor}`"));
    }
    env.types.insert(
        name.to_owned(),
        TypeInfo {
            parameters: type_parameters.len(),
            fields: field_names,
            constructors: BTreeMap::from([(format!("{name}.mk"), fields.len())]),
            type_parameters: type_parameter_names,
            field_types: fields.iter().map(|field| field.r#type.clone()).collect(),
            constructor_types: BTreeMap::from([(
                format!("{name}.mk"),
                fields.iter().map(|field| field.r#type.clone()).collect(),
            )]),
            class: matches!(declaration, SemanticDeclaration::Class { .. }),
            ..TypeInfo::default()
        },
    );
    Ok(())
}

/// Check one theorem: signature, statement, and proof; then register it as a
/// prior theorem of the module.
fn check_theorem(
    declaration: &SemanticDeclaration,
    env: &mut Environment<'_>,
) -> Result<(), String> {
    let SemanticDeclaration::Theorem {
        name,
        type_parameters,
        parameters,
        statement,
        proof,
        axioms,
    } = declaration
    else {
        return Ok(());
    };
    if axioms.windows(2).any(|pair| pair[0] >= pair[1])
        || axioms.iter().any(|axiom| !legal_name(axiom))
    {
        return Err(format!(
            "theorem `{name}` axiom policy is not sorted, unique, and qualified"
        ));
    }
    if !type_parameters.is_empty() {
        require_language_1_2(env, "theorem type parameters")?;
    }
    let scope = type_parameter_set(type_parameters)?;
    let mut binders: BTreeSet<String> = parameters
        .iter()
        .map(|parameter| parameter.name.clone())
        .collect();
    bound_names(statement, &mut binders);
    proof_terms(proof, &mut |term| bound_names(term, &mut binders));
    check_type_parameter_spelling(type_parameters, &binders, env)?;
    let locals = check_parameters(parameters, env, &scope)?;
    check_term_type_parameters(statement, &scope)?;
    let mut proof_terms_ok = Ok(());
    proof_terms(proof, &mut |term| {
        if proof_terms_ok.is_ok() {
            proof_terms_ok = check_term_type_parameters(term, &scope);
        }
    });
    proof_terms_ok?;
    check_term(statement, &locals, env, None, &BTreeSet::new())?;
    require_type(
        infer_term(statement, &typed_locals(parameters), env)?,
        &SemanticType::Prop,
        &format!("theorem `{name}` statement"),
    )?;
    check_proof(proof, &typed_locals(parameters), env)?;
    env.theorems.insert(
        name.to_owned(),
        (
            type_parameters.clone(),
            parameters.clone(),
            statement.clone(),
        ),
    );
    env.proof_type_parameters
        .insert(name.to_owned(), type_parameters.clone());
    env.proof_rules.insert(
        name.to_owned(),
        parameters
            .iter()
            .map(|parameter| parameter.r#type.clone())
            .collect(),
    );
    Ok(())
}

#[cfg(test)]
mod application_head_tests {
    use std::collections::BTreeMap;

    use super::{infer_term, Environment, MemberRef, SemanticTerm};

    /// An application whose head has no statically known function type is
    /// refused rather than left unchecked: here the head is a constructor
    /// the environment does not know, whose type inference yields nothing.
    #[test]
    fn an_application_needs_a_known_function_type() {
        let head = SemanticTerm::Constructor {
            constructor: MemberRef {
                module: None,
                name: "Unknown.make".to_owned(),
            },
            type_arguments: Vec::new(),
            arguments: Vec::new(),
        };
        let env = Environment::default();
        assert_eq!(infer_term(&head, &BTreeMap::new(), &env), Ok(None));
        let application = SemanticTerm::Apply {
            function: Box::new(head),
            arguments: vec![SemanticTerm::Nat {
                value: "1".to_owned(),
            }],
        };
        assert_eq!(
            infer_term(&application, &BTreeMap::new(), &env),
            Err("application of a value with no statically known function type".to_owned())
        );
    }
}

#[cfg(test)]
mod alpha_order_tests {
    use std::collections::BTreeMap;

    use super::{AlphaRenamer, SemanticParameter, SemanticTerm, SemanticType};

    fn lambda(name: &str) -> SemanticTerm {
        SemanticTerm::Lambda {
            parameters: vec![SemanticParameter {
                name: name.to_owned(),
                r#type: SemanticType::Nat,
            }],
            captures: Vec::new(),
            body: Box::new(SemanticTerm::Var {
                name: name.to_owned(),
            }),
        }
    }

    fn parameter(term: &SemanticTerm) -> &str {
        let SemanticTerm::Lambda { parameters, .. } = term else {
            panic!("a lambda")
        };
        &parameters[0].name
    }

    /// Binders are numbered in evaluation order, whatever order the node's
    /// serialized members sort in: an application's function (`function`
    /// sorts after `arguments`) and a conditional's branches (`else_value`
    /// sorts before `then_value`) are numbered first.
    #[test]
    fn binders_are_numbered_in_evaluation_order() {
        let mut renamer = AlphaRenamer {
            next: 0,
            scopes: Vec::new(),
            types: BTreeMap::new(),
        };
        let SemanticTerm::Apply {
            function,
            arguments,
        } = renamer.term(&SemanticTerm::Apply {
            function: Box::new(lambda("outer")),
            arguments: vec![lambda("inner")],
        })
        else {
            panic!("an application")
        };
        assert_eq!(
            (parameter(&function), parameter(&arguments[0])),
            ("_0", "_1")
        );
        let SemanticTerm::If {
            then_value,
            else_value,
            ..
        } = renamer.term(&SemanticTerm::If {
            condition: Box::new(SemanticTerm::Bool { value: true }),
            then_value: Box::new(lambda("yes")),
            else_value: Box::new(lambda("no")),
        })
        else {
            panic!("a conditional")
        };
        assert_eq!(
            (parameter(&then_value), parameter(&else_value)),
            ("_2", "_3")
        );
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, BTreeSet};

    use super::{
        check_integer_literal, check_term, infer_primitive, Environment, SemanticInteger,
        SemanticModule, SemanticPrimitive, SemanticTerm, SemanticType,
    };

    const EMPTY_POLICY: &str = r#"{"declarations":[{"kind":"theorem","name":"zero_refl","parameters":[],"proof":{"kind":"reflexivity"},"statement":{"kind":"eq","left":{"kind":"nat","value":"0"},"right":{"kind":"nat","value":"0"}}}],"spec":"lexlean/semantic-module/1"}"#;
    const BOOL_MATCH: &str = r#"{"declarations":[{"axioms":[],"body":{"branches":[{"binders":[],"body":{"kind":"nat","value":"0"},"constructor":{"name":"Bool.false"}},{"binders":[],"body":{"kind":"nat","value":"1"},"constructor":{"name":"Bool.true"}}],"kind":"match","scrutinee":{"kind":"var","name":"value"}},"kind":"definition","name":"boolToNat","parameters":[{"name":"value","type":{"kind":"bool"}}],"result":{"kind":"nat"}}],"spec":"lexlean/semantic-module/1"}"#;

    fn theorem_with_axioms(axioms: &str) -> String {
        format!(
            r#"{{"declarations":[{{"axioms":{axioms},"kind":"theorem","name":"zero_refl","parameters":[],"proof":{{"kind":"reflexivity"}},"statement":{{"kind":"eq","left":{{"kind":"nat","value":"0"}},"right":{{"kind":"nat","value":"0"}}}}}}],"spec":"lexlean/semantic-module/1"}}"#
        )
    }

    #[test]
    fn semantic_bool_match_is_typed_and_exhaustive() {
        SemanticModule::parse(
            BOOL_MATCH,
            "1.1",
            "Test",
            &[],
            &BTreeMap::new(),
            &super::model::ArtifactStore::new(u64::MAX, u64::MAX),
        )
        .expect("both Boolean constructors form a typed exhaustive match");

        let nonexhaustive = BOOL_MATCH.replace(
            r#",{"binders":[],"body":{"kind":"nat","value":"1"},"constructor":{"name":"Bool.true"}}"#,
            "",
        );
        assert!(SemanticModule::parse(
            &nonexhaustive,
            "1.1",
            "Test",
            &[],
            &BTreeMap::new(),
            &super::model::ArtifactStore::new(u64::MAX, u64::MAX)
        )
        .expect_err("one Boolean branch is not exhaustive")
        .to_string()
        .contains("nonexhaustive or mixed match branches"));
    }

    #[test]
    fn semantic_theorem_policy_defaults_to_exact_empty() {
        let module = SemanticModule::parse(
            EMPTY_POLICY,
            "1.1",
            "Test",
            &[],
            &BTreeMap::new(),
            &super::model::ArtifactStore::new(u64::MAX, u64::MAX),
        )
        .expect("omitted policy is exact empty");
        let declaration = module.declarations.first().expect("one theorem");
        assert_eq!(declaration.axiom_policy_kind(), "none");
        assert!(declaration.axioms().is_empty());
    }

    #[test]
    fn semantic_theorem_policy_round_trips_a_nonempty_exact_set() {
        let source = theorem_with_axioms(r#"["Classical.choice","propext"]"#);
        let module = SemanticModule::parse(
            &source,
            "1.1",
            "Test",
            &[],
            &BTreeMap::new(),
            &super::model::ArtifactStore::new(u64::MAX, u64::MAX),
        )
        .expect("sorted exact policy is valid");
        let declaration = module.declarations.first().expect("one theorem");
        assert_eq!(declaration.axiom_policy_kind(), "exact");
        assert_eq!(declaration.axioms(), ["Classical.choice", "propext"]);
        let encoded = serde_json::to_value(declaration).expect("serialize theorem");
        assert_eq!(
            encoded.get("axioms").expect("serialized policy"),
            &serde_json::json!(["Classical.choice", "propext"])
        );
    }

    #[test]
    fn semantic_theorem_policy_rejects_unsorted_duplicate_or_invalid_names() {
        for axioms in [
            r#"["propext","Classical.choice"]"#,
            r#"["propext","propext"]"#,
            r#"["bad-name"]"#,
        ] {
            let error = SemanticModule::parse(
                &theorem_with_axioms(axioms),
                "1.1",
                "Test",
                &[],
                &BTreeMap::new(),
                &super::model::ArtifactStore::new(u64::MAX, u64::MAX),
            )
            .expect_err("invalid exact policy must fail");
            assert!(
                error.reason.contains("not sorted, unique, and qualified"),
                "{error}"
            );
        }
    }

    fn option(value: SemanticType) -> SemanticType {
        SemanticType::Option {
            value: Box::new(value),
        }
    }

    fn typed(types: &[SemanticType]) -> Vec<Option<SemanticType>> {
        types.iter().cloned().map(Some).collect()
    }

    fn accepts(operation: SemanticPrimitive, arguments: &[SemanticType], result: SemanticType) {
        let arguments = typed(arguments);
        assert_eq!(
            infer_primitive(operation, &arguments, &result),
            Ok(result.clone()),
            "{operation:?} must accept its registered signature"
        );
        assert!(
            infer_primitive(operation, &arguments[..arguments.len() - 1], &result).is_err(),
            "{operation:?} must reject the wrong arity"
        );
    }

    #[test]
    fn every_portable_primitive_has_a_positive_and_negative_signature_case() {
        use SemanticPrimitive as P;
        use SemanticType as T;

        for operation in [P::Subtract, P::Multiply] {
            accepts(operation, &[T::Int, T::Int], T::Int);
        }
        for operation in [P::Quotient, P::Remainder] {
            accepts(operation, &[T::Int, T::Int, T::Int], T::Int);
        }
        accepts(P::Negate, &[T::Int], T::Int);
        for operation in [
            P::CheckedAdd,
            P::CheckedSubtract,
            P::CheckedMultiply,
            P::CheckedQuotient,
        ] {
            accepts(operation, &[T::Int64, T::Int64], option(T::Int64));
        }
        accepts(P::CheckedNegate, &[T::Int64], option(T::Int64));
        accepts(P::CheckedConvert, &[T::Int], option(T::UInt64));
        for operation in [P::BitAnd, P::BitOr, P::BitXor] {
            accepts(operation, &[T::UInt64, T::UInt64], T::UInt64);
        }
        accepts(P::BitNot, &[T::UInt64], T::UInt64);
        for operation in [P::ShiftLeft, P::ShiftRight] {
            accepts(operation, &[T::UInt64, T::UInt32], option(T::UInt64));
        }
        accepts(P::Append, &[T::Bytes, T::Bytes], T::Bytes);
        accepts(P::Length, &[T::String], T::Nat);
        accepts(P::Index, &[T::Bytes, T::Nat], option(T::UInt8));
        accepts(P::Slice, &[T::Bytes, T::Nat, T::Nat], option(T::Bytes));
        accepts(P::Utf8Encode, &[T::String], T::Bytes);
        accepts(P::Utf8Decode, &[T::Bytes], option(T::String));
        accepts(P::CompareBytes, &[T::Bytes, T::Bytes], T::Ordering);
        accepts(P::Equal, &[T::Int64, T::Int64], T::Bool);
        accepts(
            P::SplitExact,
            &[T::String, T::String, T::UInt32],
            option(T::List {
                element: Box::new(T::String),
            }),
        );
        accepts(
            P::Join,
            &[
                T::List {
                    element: Box::new(T::String),
                },
                T::String,
            ],
            T::String,
        );
        accepts(P::ParseDecimal, &[T::String], option(T::Int64));
        accepts(P::FormatDecimal, &[T::Int64], T::String);
    }

    #[test]
    fn every_fixed_integer_family_supports_checked_conversion_and_operations() {
        use SemanticPrimitive as P;
        use SemanticType as T;
        let integer_types = [
            T::Int,
            T::Int8,
            T::Int16,
            T::Int32,
            T::Int64,
            T::UInt8,
            T::UInt16,
            T::UInt32,
            T::UInt64,
        ];
        let fixed_types = [
            T::Int8,
            T::Int16,
            T::Int32,
            T::Int64,
            T::UInt8,
            T::UInt16,
            T::UInt32,
            T::UInt64,
        ];
        for source in &integer_types {
            for target in &fixed_types {
                accepts(
                    P::CheckedConvert,
                    core::slice::from_ref(source),
                    option(target.clone()),
                );
            }
        }
        for ty in &fixed_types {
            for operation in [
                P::CheckedAdd,
                P::CheckedSubtract,
                P::CheckedMultiply,
                P::CheckedQuotient,
            ] {
                accepts(operation, &[ty.clone(), ty.clone()], option(ty.clone()));
            }
            for operation in [P::BitAnd, P::BitOr, P::BitXor] {
                accepts(operation, &[ty.clone(), ty.clone()], ty.clone());
            }
            accepts(P::BitNot, core::slice::from_ref(ty), ty.clone());
            for operation in [P::ShiftLeft, P::ShiftRight] {
                accepts(operation, &[ty.clone(), T::UInt32], option(ty.clone()));
            }
            accepts(P::ParseDecimal, &[T::String], option(ty.clone()));
            accepts(P::FormatDecimal, core::slice::from_ref(ty), T::String);
            accepts(P::Equal, &[ty.clone(), ty.clone()], T::Bool);
        }
        for ty in [T::Int8, T::Int16, T::Int32, T::Int64] {
            accepts(
                P::CheckedNegate,
                core::slice::from_ref(&ty),
                option(ty.clone()),
            );
        }
        for ty in [T::UInt8, T::UInt16, T::UInt32, T::UInt64] {
            assert!(infer_primitive(
                P::CheckedNegate,
                &typed(core::slice::from_ref(&ty)),
                &option(ty)
            )
            .is_err());
        }
    }

    #[test]
    fn every_fixed_integer_literal_checks_both_bounds_and_canonical_spelling() {
        use SemanticInteger as I;
        for (representation, minimum, maximum, below, above) in [
            (I::Int8, "-128", "127", "-129", "128"),
            (I::Int16, "-32768", "32767", "-32769", "32768"),
            (
                I::Int32,
                "-2147483648",
                "2147483647",
                "-2147483649",
                "2147483648",
            ),
            (
                I::Int64,
                "-9223372036854775808",
                "9223372036854775807",
                "-9223372036854775809",
                "9223372036854775808",
            ),
            (I::UInt8, "0", "255", "-1", "256"),
            (I::UInt16, "0", "65535", "-1", "65536"),
            (I::UInt32, "0", "4294967295", "-1", "4294967296"),
            (
                I::UInt64,
                "0",
                "18446744073709551615",
                "-1",
                "18446744073709551616",
            ),
        ] {
            assert!(check_integer_literal(representation, minimum).is_ok());
            assert!(check_integer_literal(representation, maximum).is_ok());
            assert!(check_integer_literal(representation, below).is_err());
            assert!(check_integer_literal(representation, above).is_err());
        }
        assert!(check_integer_literal(
            I::Int,
            "-1000000000000000000000000000000000000000000000000000000000000000000"
        )
        .is_ok());
        for value in ["", "-0", "+1", "00", "01", "-01", "1_000", " 1"] {
            assert!(check_integer_literal(I::Int, value).is_err(), "{value:?}");
        }
    }

    #[test]
    fn byte_literals_require_even_lowercase_hexadecimal() {
        let check = |hex: &str| {
            check_term(
                &SemanticTerm::Bytes {
                    hex: hex.to_owned(),
                },
                &BTreeSet::new(),
                &Environment::default(),
                None,
                &BTreeSet::new(),
            )
        };
        for valid in ["", "00", "aabb7fff"] {
            assert!(check(valid).is_ok(), "{valid:?}");
        }
        for invalid in ["0", "A0", "ag", "00ff0"] {
            assert!(check(invalid).is_err(), "{invalid:?}");
        }
    }
}
