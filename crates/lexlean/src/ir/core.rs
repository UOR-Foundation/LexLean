//! Closed kernel-term IR used for lossless migrations of already elaborated
//! formal libraries.  The representation is a DAG: every edge points to an
//! earlier node, so decoding is bounded, cycle-free, and traversable by both
//! backends.  It is semantic data, never backend source text.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

/// A universe level in the kernel calculus.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "k", content = "a")]
#[serde(deny_unknown_fields)]
pub enum CoreLevel {
    /// Level zero.
    #[serde(rename = "z")]
    Zero,
    /// Successor.
    #[serde(rename = "s")]
    Succ(Box<CoreLevel>),
    /// Maximum.
    #[serde(rename = "m")]
    Max(Box<(CoreLevel, CoreLevel)>),
    /// Impredicative maximum.
    #[serde(rename = "i")]
    IMax(Box<(CoreLevel, CoreLevel)>),
    /// A declaration universe parameter.
    #[serde(rename = "p")]
    Param(String),
}

/// Binder visibility, preserved from the elaborated kernel expression.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum CoreBinderInfo {
    /// Ordinary explicit binder.
    #[serde(rename = "e")]
    Explicit,
    /// Implicit binder.
    #[serde(rename = "i")]
    Implicit,
    /// Strict implicit binder.
    #[serde(rename = "s")]
    StrictImplicit,
    /// Type-class instance binder.
    #[serde(rename = "c")]
    Instance,
}

/// One node of a closed Calculus-of-Inductive-Constructions expression.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "k")]
#[serde(deny_unknown_fields)]
pub enum CoreNode {
    /// De Bruijn variable.
    #[serde(rename = "b")]
    BVar { i: usize },
    /// Sort.
    #[serde(rename = "s")]
    Sort { l: CoreLevel },
    /// Universe-instantiated global constant.
    #[serde(rename = "c")]
    Const { n: String, u: Vec<CoreLevel> },
    /// Application.
    #[serde(rename = "a")]
    App { f: usize, x: usize },
    /// Lambda.
    #[serde(rename = "l")]
    Lambda {
        n: String,
        b: CoreBinderInfo,
        t: usize,
        v: usize,
    },
    /// Dependent function type.
    #[serde(rename = "p")]
    Forall {
        n: String,
        b: CoreBinderInfo,
        t: usize,
        v: usize,
    },
    /// Let expression.
    #[serde(rename = "e")]
    Let {
        n: String,
        t: usize,
        v: usize,
        body: usize,
        d: bool,
    },
    /// Natural-number literal.
    #[serde(rename = "n")]
    Nat { v: String },
    /// String literal, encoded as Unicode scalar values so source whitespace
    /// and TeX metacharacters never acquire a second interpretation.
    #[serde(rename = "t")]
    String { v: Vec<u32> },
    /// Expression annotation (`Expr.mdata`): kernel-irrelevant, but part of the
    /// elaborated term a downstream elaborator may read.  Core module 2 only.
    #[serde(rename = "d")]
    Mdata { m: Vec<CoreMetaEntry>, x: usize },
    /// Primitive structure projection.
    #[serde(rename = "j")]
    Projection {
        /// Projection function name, used by the readable renderers.
        n: String,
        /// Structure type name used by the kernel projection node.
        s: String,
        /// Zero-based projection index.
        i: usize,
        x: usize,
    },
}

/// The value of one annotation entry.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "t")]
#[serde(deny_unknown_fields)]
pub enum CoreMetaValue {
    /// A Boolean.
    #[serde(rename = "b")]
    Bool { v: bool },
    /// A natural number, in decimal.
    #[serde(rename = "n")]
    Nat { v: String },
    /// An integer, in decimal.
    #[serde(rename = "i")]
    Int { v: String },
    /// A string.
    #[serde(rename = "s")]
    Str { v: String },
    /// A name.
    #[serde(rename = "m")]
    Name { v: String },
}

/// One `key := value` entry of an annotation.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CoreMetaEntry {
    /// The key name.
    pub n: String,
    /// The value.
    pub v: CoreMetaValue,
}

impl CoreNode {
    fn children(&self) -> Vec<usize> {
        match self {
            Self::App { f, x } => vec![*f, *x],
            Self::Lambda { t, v, .. } | Self::Forall { t, v, .. } => vec![*t, *v],
            Self::Let { t, v, body, .. } => vec![*t, *v, *body],
            Self::Projection { x, .. } | Self::Mdata { x, .. } => vec![*x],
            Self::BVar { .. }
            | Self::Sort { .. }
            | Self::Const { .. }
            | Self::Nat { .. }
            | Self::String { .. } => Vec::new(),
        }
    }
}

/// Inductive metadata needed to reproduce the kernel declaration.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CoreStructureField {
    /// Source field name, normally a single name segment.
    pub name: String,
    /// Generated projection declaration.
    pub projection: String,
    /// Embedded parent structure for a subobject field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subobject: Option<String>,
    /// Original field binder visibility.
    pub binder: CoreBinderInfo,
    /// Deprecated Lean auto-parameter expression, retained losslessly when
    /// present in an imported environment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_param: Option<usize>,
}

/// One direct structure parent and its generated projection.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CoreStructureParent {
    /// Parent structure name.
    pub name: String,
    /// Whether the parent is represented by a stored subobject field.
    pub subobject: bool,
    /// Parent projection declaration.
    pub projection: String,
}

/// Elaborator metadata that distinguishes a structure from an ordinary
/// one-constructor inductive.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CoreStructure {
    /// Direct fields in constructor-field order.
    pub fields: Vec<CoreStructureField>,
    /// Direct parents in source order.
    pub parents: Vec<CoreStructureParent>,
}

/// Inductive metadata needed to reproduce the kernel declaration.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CoreInductive {
    /// Fixed parameters.
    pub num_params: usize,
    /// Varying indices.
    pub num_indices: usize,
    /// Constructor declaration names, in kernel order.
    pub constructors: Vec<String>,
    /// Exact structure registration metadata, absent for ordinary
    /// inductives.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub structure: Option<CoreStructure>,
}

/// How a declaration participates in generated Lean.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum CoreDeclKind {
    /// Reducible abbreviation.
    #[serde(rename = "abbrev")]
    Abbrev,
    /// Definition.
    #[serde(rename = "definition")]
    Definition,
    /// Proof-bearing theorem.
    #[serde(rename = "theorem")]
    Theorem,
    /// Inductive or structure type former.
    #[serde(rename = "inductive")]
    Inductive,
    /// Constructor generated with its inductive.
    #[serde(rename = "constructor")]
    Constructor,
    /// Recursor generated with its inductive.
    #[serde(rename = "recursor")]
    Recursor,
}

/// Kernel reducibility metadata for a definition.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "kind", content = "height")]
#[serde(deny_unknown_fields)]
pub enum CoreReducibility {
    /// An abbreviation is unfolded before ordinary definitions.
    #[serde(rename = "abbrev")]
    Abbrev,
    /// An ordinary definition whose kernel unfolding hint is opaque.
    #[serde(rename = "opaque")]
    Opaque,
    /// An ordinary definition with its exact dependency height.
    #[serde(rename = "regular")]
    Regular(u32),
}

/// Effective elaborator transparency attached to a declaration.  This is
/// distinct from the kernel definition hints above and is required for exact
/// downstream type-class and elaboration behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum CoreTransparency {
    #[serde(rename = "reducible")]
    Reducible,
    #[serde(rename = "semireducible")]
    Semireducible,
    #[serde(rename = "irreducible")]
    Irreducible,
    #[serde(rename = "implicit-reducible")]
    ImplicitReducible,
}

/// Scope behavior of a registered Lean attribute.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum CoreAttributeKind {
    #[serde(rename = "global")]
    Global,
    #[serde(rename = "local")]
    Local,
    #[serde(rename = "scoped")]
    Scoped,
}

/// Exact instance-extension metadata that is not part of the declaration's
/// kernel type or value.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CoreInstance {
    /// Type-class search priority.
    pub priority: u64,
    /// Attribute scope behavior.
    pub attribute: CoreAttributeKind,
    /// Argument synthesis order computed by the original elaborator.
    pub synthesis_order: Vec<usize>,
}

/// An explicit axiom policy for a native core declaration.  Core modules do
/// not inherit a module-wide default: generated and directly submitted
/// declarations alike carry the policy that verification enforces.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "kind", content = "axioms")]
#[serde(deny_unknown_fields)]
pub enum CoreAxiomPolicy {
    /// The observed axiom set must be empty.
    #[serde(rename = "none")]
    None,
    /// The observed set must be a subset of this list.
    #[serde(rename = "allow")]
    Allow(Vec<String>),
    /// The observed set must equal this list.
    #[serde(rename = "exact")]
    Exact(Vec<String>),
}

impl CoreAxiomPolicy {
    /// The policy kind used in verification records and diagnostics.
    #[must_use]
    pub const fn kind(&self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Allow(_) => "allow",
            Self::Exact(_) => "exact",
        }
    }

    /// The configured axiom names.
    #[must_use]
    pub fn axioms(&self) -> &[String] {
        match self {
            Self::None => &[],
            Self::Allow(names) | Self::Exact(names) => names,
        }
    }

    /// Enforce the policy against the normalized observed set.
    #[must_use]
    pub fn permits(&self, observed: &[String]) -> bool {
        match self {
            Self::None => observed.is_empty(),
            Self::Allow(allowed) => observed.iter().all(|axiom| allowed.contains(axiom)),
            Self::Exact(exact) => {
                let mut sorted = observed.to_vec();
                sorted.sort();
                sorted.dedup();
                sorted == *exact
            }
        }
    }
}

/// The safety of a definition row; a row that omits it is safe.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum CoreSafety {
    /// A definition that is not safe.
    #[serde(rename = "unsafe")]
    Unsafe,
    /// A partial definition, the compiler's implementation record of a
    /// recursive function.
    #[serde(rename = "partial")]
    Partial,
}

/// A declaration tag of the elaborator that a downstream elaboration can
/// observe.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub enum CoreTag {
    /// `protected`: the short name is not introduced by `open`.
    #[serde(rename = "protected")]
    Protected,
    /// An auxiliary recursor (`recOn`, `casesOn`, `brecOn`, ...).
    #[serde(rename = "aux_recursor")]
    AuxRecursor,
    /// `@[elab_as_elim]`.
    #[serde(rename = "elab_as_elim")]
    ElabAsElim,
    /// `@[defeq]`: a theorem proved by definitional unfolding.
    #[serde(rename = "defeq")]
    Defeq,
    /// The backward-compatible form of `@[defeq]`.
    #[serde(rename = "backward_defeq")]
    BackwardDefeq,
    /// A definition produced by recursion compilation.
    #[serde(rename = "recursive")]
    Recursive,
    /// A sparse `casesOn` or a constructor eliminator.
    #[serde(rename = "sparse_cases_on")]
    SparseCasesOn,
}

/// Shape information of a no-confusion declaration.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "kind")]
#[serde(deny_unknown_fields)]
pub enum CoreNoConfusion {
    /// The regular no-confusion lemma of a type.
    #[serde(rename = "regular")]
    Regular {
        arity: usize,
        lhs: usize,
        rhs: usize,
    },
    /// A per-constructor no-confusion lemma.
    #[serde(rename = "per_ctor")]
    PerCtor { arity: usize, fields: usize },
}

/// Parameter structure of one alternative of a matcher.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CoreMatcherAlt {
    /// Actual fields, not counting discriminant equations.
    pub fields: usize,
    /// The overlap assumptions of a splitter.
    pub overlaps: usize,
    /// Whether the alternative has an artificial unit parameter.
    pub unit_thunk: bool,
}

/// The matcher registration of an auxiliary `match` definition.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CoreMatcher {
    /// Parameters before the motive.
    pub num_params: usize,
    /// Discriminants.
    pub num_discrs: usize,
    /// The alternatives, in order.
    pub alts: Vec<CoreMatcherAlt>,
    /// The universe parameter that selects the elimination universe, when
    /// the matcher can eliminate into more than `Prop`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub u_elim_pos: Option<usize>,
    /// For each discriminant, the name of its annotation, if any.
    pub discrs: Vec<Vec<String>>,
    /// Which alternatives may overlap which, by overlapped index.
    pub overlaps: Vec<(usize, Vec<usize>)>,
}

/// The projection-function registration of a structure field.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CoreProjection {
    /// The structure's constructor.
    pub constructor: String,
    /// The number of structure parameters.
    pub num_params: usize,
    /// The field index.
    pub index: usize,
    /// Whether the structure is a class.
    pub from_class: bool,
}

/// A compiler inlining attribute of a definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum CoreInline {
    /// `@[inline]`.
    #[serde(rename = "inline")]
    Inline,
    /// `@[noinline]`.
    #[serde(rename = "noinline")]
    NoInline,
    /// `@[macro_inline]`.
    #[serde(rename = "macro_inline")]
    MacroInline,
    /// `@[inline_if_reduce]`.
    #[serde(rename = "inline_if_reduce")]
    InlineIfReduce,
    /// `@[always_inline]`.
    #[serde(rename = "always_inline")]
    AlwaysInline,
}

/// What the elaborator records of a sparse `casesOn`, a `casesOn` with arms
/// for only some of the constructors of a type. The constant is made by the
/// elaborator's own construction from the type and its constructors, so the
/// Lean backend remakes it that way and refuses a row that is not what the
/// construction makes.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CoreSparse {
    /// The inductive type.
    pub ind_name: String,
    /// The position of the major premise.
    pub major_pos: usize,
    /// The number of arguments of the constant.
    pub arity: usize,
    /// The constructors that have an arm, in the order of the arms.
    pub interesting_ctors: Vec<String>,
}

/// The kind of the `grind` attribute that registers a theorem for
/// E-matching.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum CoreGrind {
    /// `@[grind =]`.
    #[serde(rename = "eq_lhs")]
    EqLhs,
    /// `@[grind =>]` read right to left.
    #[serde(rename = "eq_rhs")]
    EqRhs,
    /// Both sides of an equation.
    #[serde(rename = "eq_both")]
    EqBoth,
    /// The equation read backwards.
    #[serde(rename = "eq_bwd")]
    EqBwd,
    /// `@[grind →]`.
    #[serde(rename = "fwd")]
    Fwd,
    /// `@[grind ←]`.
    #[serde(rename = "bwd")]
    Bwd,
    /// `@[grind ->]` of an equivalence, left to right.
    #[serde(rename = "left_right")]
    LeftRight,
    /// An equivalence, right to left.
    #[serde(rename = "right_left")]
    RightLeft,
    /// `@[grind]`.
    #[serde(rename = "default")]
    Default,
}

/// The registration of a theorem in the default simplification set.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CoreSimp {
    /// Attribute scope behavior.
    pub attribute: CoreAttributeKind,
    /// Simplification priority.
    pub priority: u64,
    /// Whether the lemma is applied after the subterms are simplified.
    pub post: bool,
}

/// The structural-recursion equation data the elaborator keeps for a
/// recursive definition, from which it generates equation lemmas on demand.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CoreEqnInfo {
    /// The position of the recursive argument.
    pub rec_arg_pos: usize,
    /// The definitions of the recursive group.
    pub declarations: Vec<String>,
    /// The number of fixed parameters.
    pub num_fixed: usize,
    /// For each function, the position of each parameter among the fixed ones.
    pub perms: Vec<Vec<Vec<usize>>>,
    /// The dependencies among the fixed parameters.
    pub rev_deps: Vec<Vec<Vec<usize>>>,
    /// The type, as a node.
    pub r#type: usize,
    /// The value, as a node.
    pub value: usize,
    /// The universe parameters.
    pub levels: Vec<String>,
}

/// One declaration linked to exact type and, where applicable, value nodes.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CoreDeclaration {
    /// Generated global name.
    pub name: String,
    /// Universe parameters in declaration order.
    pub levels: Vec<String>,
    /// Declaration kind.
    pub kind: CoreDeclKind,
    /// Exact kernel reducibility metadata for definitions and abbreviations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reducibility: Option<CoreReducibility>,
    /// Effective elaborator transparency.
    pub transparency: CoreTransparency,
    /// Explicit per-declaration axiom policy.
    pub policy: CoreAxiomPolicy,
    /// Type root.
    pub r#type: usize,
    /// Value/proof root for definitions and theorems.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<usize>,
    /// Inductive metadata exactly for inductive declarations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inductive: Option<CoreInductive>,
    /// Whether this type is a type class.
    #[serde(default)]
    pub class: bool,
    /// Instance-extension metadata, when registered as an instance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance: Option<CoreInstance>,
    /// Whether the source compiler generated this environment row. An
    /// inductive-owned row is recreated by its owner and audited in the native
    /// environment. A self-recursive definition that no native declaration
    /// references is retained as import provenance, but is not a safe native
    /// declaration and therefore appears in neither generated backend.
    #[serde(default)]
    pub generated: bool,
    /// The definitions of the mutual group the row belongs to, itself
    /// included. Core module 2 only.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub all: Option<Vec<String>>,
    /// The safety of a definition; omitted for a safe one. Core module 2
    /// only.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub safety: Option<CoreSafety>,
    /// Whether the source environment ran the code generator over this row.
    /// Core module 2 only.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub compiled: bool,
    /// Elaborator tags, sorted and distinct. Core module 2 only.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<CoreTag>,
    /// No-confusion shape. Core module 2 only.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub no_confusion: Option<CoreNoConfusion>,
    /// Matcher registration. Core module 2 only.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matcher: Option<CoreMatcher>,
    /// Projection-function registration. Core module 2 only.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub projection: Option<CoreProjection>,
    /// Simplification-set registration. Core module 2 only.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub simp: Option<CoreSimp>,
    /// Structural-recursion equation data. Core module 2 only.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub eqn_info: Option<CoreEqnInfo>,
    /// The compiler inlining attribute of a definition. Core module 2 only.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inline: Option<CoreInline>,
    /// The record of a sparse `casesOn` definition. Core module 2 only.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sparse: Option<CoreSparse>,
    /// The `grind` attribute of a theorem. Core module 2 only.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grind: Option<CoreGrind>,
}

impl CoreDeclaration {
    /// Whether the row uses a field only core module 2 has.
    #[must_use]
    pub fn uses_version_2_fields(&self) -> bool {
        self.all.is_some()
            || self.safety.is_some()
            || self.compiled
            || !self.tags.is_empty()
            || self.no_confusion.is_some()
            || self.matcher.is_some()
            || self.projection.is_some()
            || self.simp.is_some()
            || self.eqn_info.is_some()
            || self.inline.is_some()
            || self.sparse.is_some()
            || self.grind.is_some()
    }
}

/// The tag of the first core-module schema.
pub const CORE_SPEC_1: &str = "lexlean/core-module/1";

/// The tag of the second core-module schema: language 1.2 only, it adds the
/// annotation node and the environment state a downstream elaboration reads.
pub const CORE_SPEC_2: &str = "lexlean/core-module/2";

/// A complete native core module.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CoreModule {
    /// Schema discriminator.
    pub spec: String,
    /// Lean modules containing the foundational constants referenced by the
    /// DAG.  Generated declarations themselves never appear here.
    pub imports: Vec<String>,
    /// Shared expression DAG in child-before-parent order.
    pub nodes: Vec<CoreNode>,
    /// Closed proof nodes.  They remain inline so dependent expected types are
    /// preserved instead of being weakened to a globally inferred helper
    /// type.
    #[serde(default)]
    pub proof_nodes: Vec<usize>,
    /// Declaration rows in dependency order.
    pub declarations: Vec<CoreDeclaration>,
}

impl CoreModule {
    /// Decode and enforce the closed schema invariants of either schema
    /// version. Core module 2 is accepted only under language 1.2.
    pub fn parse(text: &str, language: &str) -> Result<Self, String> {
        let module: Self = serde_json::from_str(text)
            .map_err(|error| format!("invalid core-module JSON: {error}"))?;
        module.validate(language)?;
        CanonicalSyntax::check(text.as_bytes())?;
        Ok(module)
    }

    /// Read a module written in the authoring notation (SPEC.md §17.21.8),
    /// which is language 1.2's, and enforce the closed schema invariants.
    pub fn from_notation(text: &str, language: &str) -> Result<Self, String> {
        if language != crate::LANGUAGE_1_2 {
            return Err(format!(
                "the authoring notation of a coremodule is language 1.2; write `\\coredata` under language {language}"
            ));
        }
        let module = crate::surface::core::parse_module(text).map_err(|failure| {
            format!(
                "invalid core notation: {} (at byte {})",
                failure.message, failure.offset
            )
        })?;
        module.validate(language)?;
        Ok(module)
    }

    /// Decode the module of a native `.lex.tex` source: the one `\coredata`
    /// argument between the module's `coremodule` markers, or the module in
    /// the authoring notation.
    pub fn from_source(source: &str, language: &str) -> Result<Self, String> {
        const MARKER: &str = "\\coredata{";
        if let Some(located) = crate::surface::module::locate(source)
            .filter(|found| found.kind.is(&crate::surface::module::CORE) && found.data.is_none())
        {
            return Self::from_notation(&source[located.body.0..located.body.1], language);
        }
        if source.matches(MARKER).count() != 1 {
            return Err("a native core source holds exactly one core payload".to_owned());
        }
        let start = source
            .find(MARKER)
            .map(|at| at + MARKER.len())
            .ok_or_else(|| "a native core source holds no core payload".to_owned())?;
        let tail = &source[start..];
        let end = tail
            .find("}\n\\end{coremodule}")
            .ok_or_else(|| "a native core source has no closed core module".to_owned())?;
        Self::parse(&tail[..end], language)
    }

    /// The depth of the deepest expression of the DAG: one for a leaf, and
    /// one more than its deepest child for any other node. The edges all
    /// point backwards, so one forward pass computes it without recursion.
    #[must_use]
    pub fn expression_depth(&self) -> u64 {
        let mut depth = vec![0_u64; self.nodes.len()];
        let mut deepest = 0;
        for (index, node) in self.nodes.iter().enumerate() {
            let below = node
                .children()
                .into_iter()
                .filter_map(|child| depth.get(child).copied())
                .max()
                .unwrap_or(0);
            depth[index] = below + 1;
            deepest = deepest.max(below + 1);
        }
        deepest
    }

    /// The schema version the module declares: 1 or 2.
    #[must_use]
    pub fn version(&self) -> u8 {
        if self.spec == CORE_SPEC_2 {
            2
        } else {
            1
        }
    }

    /// Enforce the closed schema invariants of either schema version; core
    /// module 2 is accepted only under language 1.2.
    #[allow(clippy::too_many_lines)]
    pub fn validate(&self, language: &str) -> Result<(), String> {
        if self.spec != CORE_SPEC_1 && self.spec != CORE_SPEC_2 {
            return Err(format!("unsupported core-module schema `{}`", self.spec));
        }
        let version = self.version();
        if version == 2 && language != "1.2" {
            return Err(format!(
                "`{CORE_SPEC_2}` requires language 1.2, not `{language}`"
            ));
        }
        let name_ok: fn(&str) -> Result<(), String> = if version == 2 {
            validate_name_v2
        } else {
            validate_name
        };
        let segment_ok: fn(&str) -> Result<(), String> = if version == 2 {
            validate_segment_v2
        } else {
            validate_segment
        };
        if version == 1 {
            if self
                .nodes
                .iter()
                .any(|node| matches!(node, CoreNode::Mdata { .. }))
            {
                return Err(format!(
                    "an annotation node requires `{CORE_SPEC_2}`, not `{CORE_SPEC_1}`"
                ));
            }
            if let Some(row) = self
                .declarations
                .iter()
                .find(|row| row.uses_version_2_fields())
            {
                return Err(format!(
                    "core declaration `{}` carries a field that requires `{CORE_SPEC_2}`",
                    row.name
                ));
            }
        }
        if self.declarations.is_empty() {
            return Err("a core module contains at least one declaration".to_owned());
        }
        let mut imports = BTreeSet::new();
        for import in &self.imports {
            validate_name(import)?;
            if !imports.insert(import) {
                return Err(format!("duplicate core import `{import}`"));
            }
        }
        for (index, node) in self.nodes.iter().enumerate() {
            for child in node.children() {
                if child >= index {
                    return Err(format!(
                        "core node {index} points to non-earlier node {child}"
                    ));
                }
            }
            match node {
                CoreNode::Const { n, .. } => name_ok(n)?,
                CoreNode::Projection { n, s, .. } => {
                    name_ok(n)?;
                    name_ok(s)?;
                }
                CoreNode::Mdata { m, .. } => {
                    for entry in m {
                        name_ok(&entry.n)?;
                        match &entry.v {
                            CoreMetaValue::Nat { v } => {
                                if v.parse::<u128>().is_err() {
                                    return Err(format!("`{v}` is not a natural number"));
                                }
                            }
                            CoreMetaValue::Int { v } => {
                                if v.parse::<i128>().is_err() {
                                    return Err(format!("`{v}` is not an integer"));
                                }
                            }
                            CoreMetaValue::Name { v } => name_ok(v)?,
                            CoreMetaValue::Bool { .. } | CoreMetaValue::Str { .. } => {}
                        }
                    }
                }
                CoreNode::BVar { .. }
                | CoreNode::Sort { .. }
                | CoreNode::App { .. }
                | CoreNode::Lambda { .. }
                | CoreNode::Forall { .. }
                | CoreNode::Let { .. }
                | CoreNode::Nat { .. }
                | CoreNode::String { .. } => {}
            }
        }
        let mut previous = None;
        for proof in &self.proof_nodes {
            if *proof >= self.nodes.len() {
                return Err(format!("core proof node {proof} is out of range"));
            }
            if previous.is_some_and(|prior| prior >= *proof) {
                return Err("core proof nodes are strictly increasing".to_owned());
            }
            previous = Some(*proof);
        }
        let every_name: BTreeSet<&String> = self.declarations.iter().map(|row| &row.name).collect();
        let mut names = BTreeSet::new();
        for declaration in &self.declarations {
            name_ok(&declaration.name)?;
            if !names.insert(&declaration.name) {
                return Err(format!("duplicate core declaration `{}`", declaration.name));
            }
            if declaration.r#type >= self.nodes.len()
                || declaration
                    .value
                    .is_some_and(|value| value >= self.nodes.len())
            {
                return Err(format!(
                    "core declaration `{}` has an out-of-range expression root",
                    declaration.name
                ));
            }
            let has_value = declaration.value.is_some();
            if matches!(
                declaration.kind,
                CoreDeclKind::Abbrev | CoreDeclKind::Definition | CoreDeclKind::Theorem
            ) != has_value
            {
                return Err(format!(
                    "core declaration `{}` has a kind/value mismatch",
                    declaration.name
                ));
            }
            if matches!(declaration.kind, CoreDeclKind::Inductive)
                != declaration.inductive.is_some()
            {
                return Err(format!(
                    "core declaration `{}` has a kind/inductive mismatch",
                    declaration.name
                ));
            }
            if matches!(
                declaration.kind,
                CoreDeclKind::Definition | CoreDeclKind::Abbrev
            ) != declaration.reducibility.is_some()
            {
                return Err(format!(
                    "core declaration `{}` has a kind/reducibility-metadata mismatch",
                    declaration.name
                ));
            }
            if matches!(declaration.kind, CoreDeclKind::Abbrev)
                != matches!(declaration.reducibility, Some(CoreReducibility::Abbrev))
            {
                return Err(format!(
                    "core declaration `{}` has an inconsistent abbreviation hint",
                    declaration.name
                ));
            }
            let mut policy_names = BTreeSet::new();
            for axiom in declaration.policy.axioms() {
                name_ok(axiom)?;
                if !policy_names.insert(axiom) {
                    return Err(format!(
                        "core declaration `{}` repeats axiom `{axiom}` in its policy",
                        declaration.name
                    ));
                }
            }
            for level in &declaration.levels {
                segment_ok(level)?;
            }
            self.validate_version_2_row(declaration, &every_name, name_ok, segment_ok)?;
            if let Some(inductive) = &declaration.inductive {
                if inductive.constructors.is_empty() {
                    return Err(format!(
                        "core inductive `{}` has no constructors",
                        declaration.name
                    ));
                }
                for name in &inductive.constructors {
                    name_ok(name)?;
                    if !names.contains(name)
                        && !self.declarations.iter().any(|row| row.name == *name)
                    {
                        return Err(format!(
                            "core inductive `{}` names absent constructor `{name}`",
                            declaration.name
                        ));
                    }
                }
                if let Some(structure) = &inductive.structure {
                    for field in &structure.fields {
                        name_ok(&field.name)?;
                        name_ok(&field.projection)?;
                        if let Some(subobject) = &field.subobject {
                            name_ok(subobject)?;
                        }
                        if field
                            .auto_param
                            .is_some_and(|node| node >= self.nodes.len())
                        {
                            return Err(format!(
                                "core structure `{}` has an out-of-range auto-parameter root",
                                declaration.name
                            ));
                        }
                    }
                    for parent in &structure.parents {
                        name_ok(&parent.name)?;
                        name_ok(&parent.projection)?;
                    }
                }
            }
        }
        Ok(())
    }
}

impl CoreModule {
    /// The checks of the fields only core module 2 has.
    fn validate_version_2_row(
        &self,
        declaration: &CoreDeclaration,
        names: &BTreeSet<&String>,
        name_ok: fn(&str) -> Result<(), String>,
        segment_ok: fn(&str) -> Result<(), String>,
    ) -> Result<(), String> {
        let node_ok = |node: usize| node < self.nodes.len();
        let is_definition = matches!(
            declaration.kind,
            CoreDeclKind::Abbrev | CoreDeclKind::Definition
        );
        if declaration.safety.is_some() && !is_definition {
            return Err(format!(
                "core declaration `{}` has a safety but is not a definition",
                declaration.name
            ));
        }
        if declaration.compiled
            && !is_definition
            && !matches!(declaration.kind, CoreDeclKind::Inductive)
        {
            return Err(format!(
                "core declaration `{}` is marked compiled but is neither a definition nor an inductive",
                declaration.name
            ));
        }
        if declaration.tags.windows(2).any(|pair| pair[0] >= pair[1]) {
            return Err(format!(
                "core declaration `{}` has tags that are not strictly ordered",
                declaration.name
            ));
        }
        if let Some(all) = &declaration.all {
            if !all.contains(&declaration.name) {
                return Err(format!(
                    "core declaration `{}` is absent from its own group",
                    declaration.name
                ));
            }
            let mut seen = BTreeSet::new();
            for member in all {
                name_ok(member)?;
                if !seen.insert(member) {
                    return Err(format!(
                        "core declaration `{}` repeats group member `{member}`",
                        declaration.name
                    ));
                }
                if declaration.safety.is_some() && !names.contains(member) {
                    return Err(format!(
                        "core declaration `{}` names the absent group member `{member}`",
                        declaration.name
                    ));
                }
            }
        }
        if declaration.matcher.is_some() && !is_definition {
            return Err(format!(
                "core declaration `{}` has a matcher but is not a definition",
                declaration.name
            ));
        }
        if let Some(matcher) = &declaration.matcher {
            for discriminant in &matcher.discrs {
                if discriminant.len() > 1 {
                    return Err(format!(
                        "core matcher `{}` names a discriminant more than once",
                        declaration.name
                    ));
                }
                for name in discriminant {
                    name_ok(name)?;
                }
            }
        }
        if let Some(projection) = &declaration.projection {
            name_ok(&projection.constructor)?;
        }
        if declaration.simp.is_some() && !matches!(declaration.kind, CoreDeclKind::Theorem) {
            return Err(format!(
                "core declaration `{}` is registered for simplification but is not a theorem",
                declaration.name
            ));
        }
        if let Some(info) = &declaration.eqn_info {
            if !is_definition {
                return Err(format!(
                    "core declaration `{}` has equation data but is not a definition",
                    declaration.name
                ));
            }
            if !node_ok(info.r#type) || !node_ok(info.value) {
                return Err(format!(
                    "core declaration `{}` has an out-of-range equation-data root",
                    declaration.name
                ));
            }
            for name in &info.declarations {
                name_ok(name)?;
            }
            for level in &info.levels {
                segment_ok(level)?;
            }
        }
        if declaration.inline.is_some() && !is_definition {
            return Err(format!(
                "core declaration `{}` has an inlining attribute but is not a definition",
                declaration.name
            ));
        }
        if let Some(sparse) = &declaration.sparse {
            if !is_definition {
                return Err(format!(
                    "core declaration `{}` is a sparse `casesOn` but is not a definition",
                    declaration.name
                ));
            }
            if sparse.interesting_ctors.is_empty() {
                return Err(format!(
                    "core declaration `{}` is a sparse `casesOn` of no constructor",
                    declaration.name
                ));
            }
            name_ok(&sparse.ind_name)?;
            for constructor in &sparse.interesting_ctors {
                name_ok(constructor)?;
            }
        }
        if declaration.grind.is_some() && !matches!(declaration.kind, CoreDeclKind::Theorem) {
            return Err(format!(
                "core declaration `{}` carries a `grind` attribute but is not a theorem",
                declaration.name
            ));
        }
        Ok(())
    }
}

/// Streaming validation of the restricted canonical JSON spelling.  The
/// typed serde pass above owns schema validation; this pass enforces the
/// byte-level rules without allocating a second tree for a foundation model.
struct CanonicalSyntax<'a> {
    bytes: &'a [u8],
    at: usize,
}

impl<'a> CanonicalSyntax<'a> {
    fn check(bytes: &'a [u8]) -> Result<(), String> {
        let mut parser = Self { bytes, at: 0 };
        parser.value()?;
        if parser.at != bytes.len() {
            return Err(
                "core-module JSON is not in canonical byte form: trailing bytes".to_owned(),
            );
        }
        Ok(())
    }

    fn current(&self) -> Option<u8> {
        self.bytes.get(self.at).copied()
    }

    fn take(&mut self, expected: u8) -> Result<(), String> {
        if self.current() != Some(expected) {
            return Err(format!(
                "core-module JSON is not in canonical byte form at byte {}",
                self.at
            ));
        }
        self.at += 1;
        Ok(())
    }

    fn literal(&mut self, value: &[u8]) -> Result<(), String> {
        if !self.bytes[self.at..].starts_with(value) {
            return Err(format!(
                "core-module JSON is not in canonical byte form at byte {}",
                self.at
            ));
        }
        self.at += value.len();
        Ok(())
    }

    fn value(&mut self) -> Result<(), String> {
        match self.current() {
            Some(b'{') => self.object(),
            Some(b'[') => self.array(),
            Some(b'"') => self.string(),
            Some(b't') => self.literal(b"true"),
            Some(b'f') => self.literal(b"false"),
            Some(b'-' | b'0'..=b'9') => self.integer(),
            _ => Err(format!(
                "core-module JSON is not in canonical byte form at byte {}",
                self.at
            )),
        }
    }

    fn object(&mut self) -> Result<(), String> {
        self.take(b'{')?;
        if self.current() == Some(b'}') {
            self.at += 1;
            return Ok(());
        }
        let mut previous: Option<String> = None;
        loop {
            let start = self.at;
            self.string()?;
            let key: String = serde_json::from_slice(&self.bytes[start..self.at])
                .map_err(|error| format!("invalid core-module JSON key: {error}"))?;
            if previous.as_ref().is_some_and(|prior| prior >= &key) {
                return Err(format!(
                    "core-module JSON is not in canonical byte form: object key `{key}` is not strictly ordered"
                ));
            }
            previous = Some(key);
            self.take(b':')?;
            self.value()?;
            match self.current() {
                Some(b',') => self.at += 1,
                Some(b'}') => {
                    self.at += 1;
                    return Ok(());
                }
                _ => {
                    return Err(format!(
                        "core-module JSON is not in canonical byte form at byte {}",
                        self.at
                    ));
                }
            }
        }
    }

    fn array(&mut self) -> Result<(), String> {
        self.take(b'[')?;
        if self.current() == Some(b']') {
            self.at += 1;
            return Ok(());
        }
        loop {
            self.value()?;
            match self.current() {
                Some(b',') => self.at += 1,
                Some(b']') => {
                    self.at += 1;
                    return Ok(());
                }
                _ => {
                    return Err(format!(
                        "core-module JSON is not in canonical byte form at byte {}",
                        self.at
                    ));
                }
            }
        }
    }

    fn string(&mut self) -> Result<(), String> {
        self.take(b'"')?;
        loop {
            match self.current() {
                Some(b'"') => {
                    self.at += 1;
                    return Ok(());
                }
                Some(b'\\') => {
                    self.at += 1;
                    match self.current() {
                        Some(b'"' | b'\\' | b'b' | b'f' | b'n' | b'r' | b't') => self.at += 1,
                        Some(b'u') => {
                            self.at += 1;
                            let end = self.at.checked_add(4).ok_or_else(|| {
                                "core-module JSON escape offset overflow".to_owned()
                            })?;
                            let Some(hex) = self.bytes.get(self.at..end) else {
                                return Err("unterminated core-module JSON escape".to_owned());
                            };
                            if !hex
                                .iter()
                                .all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
                            {
                                return Err("core-module JSON uses a noncanonical Unicode escape"
                                    .to_owned());
                            }
                            let value = hex.iter().fold(0u32, |value, byte| {
                                value * 16
                                    + u32::from(match byte {
                                        b'0'..=b'9' => byte - b'0',
                                        b'a'..=b'f' => byte - b'a' + 10,
                                        _ => 0,
                                    })
                            });
                            if value >= 0x20 || matches!(value, 0x08 | 0x09 | 0x0a | 0x0c | 0x0d) {
                                return Err(
                                    "core-module JSON uses a nonminimal Unicode escape".to_owned()
                                );
                            }
                            self.at = end;
                        }
                        _ => return Err("core-module JSON uses a noncanonical escape".to_owned()),
                    }
                }
                Some(0x00..=0x1f) => {
                    return Err("core-module JSON contains an unescaped control byte".to_owned());
                }
                Some(_) => self.at += 1,
                None => return Err("unterminated core-module JSON string".to_owned()),
            }
        }
    }

    fn integer(&mut self) -> Result<(), String> {
        let negative = self.current() == Some(b'-');
        if negative {
            self.at += 1;
        }
        match self.current() {
            Some(b'0') => {
                self.at += 1;
                if negative || self.current().is_some_and(|byte| byte.is_ascii_digit()) {
                    return Err("core-module JSON uses a noncanonical integer".to_owned());
                }
            }
            Some(b'1'..=b'9') => {
                self.at += 1;
                while self.current().is_some_and(|byte| byte.is_ascii_digit()) {
                    self.at += 1;
                }
            }
            _ => return Err("core-module JSON contains an invalid integer".to_owned()),
        }
        if self
            .current()
            .is_some_and(|byte| matches!(byte, b'.' | b'e' | b'E' | b'+'))
        {
            return Err("core-module JSON contains a non-integer number".to_owned());
        }
        Ok(())
    }
}

fn validate_segment(segment: &str) -> Result<(), String> {
    let mut chars = segment.chars();
    let Some(first) = chars.next() else {
        return Err("empty Lean name segment".to_owned());
    };
    if !(first.is_ascii_alphabetic() || first == '_')
        || !chars.all(|character| character.is_ascii_alphanumeric() || "_?'".contains(character))
    {
        return Err(format!(
            "`{segment}` is not a closed ASCII Lean name segment"
        ));
    }
    Ok(())
}

fn validate_name(name: &str) -> Result<(), String> {
    if name.is_empty() {
        return Err("empty Lean name".to_owned());
    }
    for segment in name.split('.') {
        validate_segment(segment)?;
    }
    Ok(())
}

/// A component of a core-module-2 name: `#` and decimal digits for a numeric
/// component, otherwise a nonempty string without `.`, in which a backslash
/// introduces exactly `\\`, `\d` (a dot), or `\h` (a leading `#`).
fn validate_segment_v2(segment: &str) -> Result<(), String> {
    if segment.is_empty() {
        return Err("empty Lean name segment".to_owned());
    }
    if let Some(digits) = segment.strip_prefix('#') {
        return if !digits.is_empty()
            && digits.bytes().all(|byte| byte.is_ascii_digit())
            && (digits == "0" || !digits.starts_with('0'))
        {
            Ok(())
        } else {
            Err(format!("`{segment}` is not a numeric name component"))
        };
    }
    let mut characters = segment.chars();
    while let Some(character) = characters.next() {
        match character {
            '\\' => {
                if !matches!(characters.next(), Some('\\' | 'd' | 'h')) {
                    return Err(format!("`{segment}` has an unknown name escape"));
                }
            }
            '.' => return Err(format!("`{segment}` holds an unescaped dot")),
            control if control.is_control() => {
                return Err(format!("`{segment}` holds a control character"));
            }
            _ => {}
        }
    }
    Ok(())
}

/// A core-module-2 name: components separated by dots.
fn validate_name_v2(name: &str) -> Result<(), String> {
    if name.is_empty() {
        return Err("empty Lean name".to_owned());
    }
    for segment in name.split('.') {
        validate_segment_v2(segment)?;
    }
    Ok(())
}

/// Whether `name` belongs to the closed ASCII Lean-name grammar used by
/// native core declarations, references, and axiom policies.
#[must_use]
pub fn is_closed_lean_name(name: &str) -> bool {
    validate_name(name).is_ok()
}

#[cfg(test)]
mod canonical_syntax_tests {
    use super::CanonicalSyntax;

    #[test]
    fn canonical_json_is_checked_without_a_value_tree() {
        assert!(CanonicalSyntax::check(br#"{"a":[0,true,"x\n"],"b":-2}"#).is_ok());
        for rejected in [
            br#"{"b":0,"a":0}"#.as_slice(),
            br#"{"a": 0}"#.as_slice(),
            br#"{"a":"\u0061"}"#.as_slice(),
            br#"{"a":-0}"#.as_slice(),
            br#"{"a":null}"#.as_slice(),
        ] {
            assert!(CanonicalSyntax::check(rejected).is_err());
        }
    }
}

#[cfg(test)]
mod version_2_tests {
    use super::*;
    use crate::artifact::canonical_json::Json;

    fn theorem(name: &str) -> CoreDeclaration {
        CoreDeclaration {
            name: name.to_owned(),
            levels: Vec::new(),
            kind: CoreDeclKind::Theorem,
            reducibility: None,
            transparency: CoreTransparency::Semireducible,
            policy: CoreAxiomPolicy::Exact(Vec::new()),
            r#type: 0,
            value: Some(0),
            inductive: None,
            class: false,
            instance: None,
            generated: false,
            all: None,
            safety: None,
            compiled: false,
            tags: Vec::new(),
            no_confusion: None,
            matcher: None,
            projection: None,
            simp: None,
            eqn_info: None,
            inline: None,
            sparse: None,
            grind: None,
        }
    }

    fn module(spec: &str, declaration: CoreDeclaration) -> String {
        let core = CoreModule {
            spec: spec.to_owned(),
            imports: vec!["Init".to_owned()],
            nodes: vec![CoreNode::Const {
                n: "True".to_owned(),
                u: Vec::new(),
            }],
            proof_nodes: Vec::new(),
            declarations: vec![declaration],
        };
        let text = serde_json::to_string(&core).expect("serializes");
        Json::parse(text.as_bytes())
            .expect("is JSON")
            .to_canonical_string()
    }

    #[test]
    fn version_2_is_language_1_2_only() {
        let text = module(CORE_SPEC_2, theorem("T.t"));
        assert!(CoreModule::parse(&text, "1.2").is_ok());
        for language in ["1.0", "1.1"] {
            let error = CoreModule::parse(&text, language).expect_err("refused");
            assert!(error.contains("requires language 1.2"), "{error}");
        }
    }

    #[test]
    fn version_1_refuses_every_version_2_construct() {
        let mut tagged = theorem("T.t");
        tagged.tags = vec![CoreTag::Protected];
        let text = module(CORE_SPEC_1, tagged);
        let error = CoreModule::parse(&text, "1.0").expect_err("refused");
        assert!(
            error.contains("requires `lexlean/core-module/2`"),
            "{error}"
        );

        let core = CoreModule {
            spec: CORE_SPEC_1.to_owned(),
            imports: vec!["Init".to_owned()],
            nodes: vec![
                CoreNode::Const {
                    n: "True".to_owned(),
                    u: Vec::new(),
                },
                CoreNode::Mdata {
                    m: Vec::new(),
                    x: 0,
                },
            ],
            proof_nodes: Vec::new(),
            declarations: vec![theorem("T.t")],
        };
        let text = Json::parse(serde_json::to_string(&core).expect("serializes").as_bytes())
            .expect("is JSON")
            .to_canonical_string();
        let error = CoreModule::parse(&text, "1.0").expect_err("refused");
        assert!(error.contains("annotation node"), "{error}");
    }

    #[test]
    fn version_2_names_carry_numeric_components_and_escapes() {
        for accepted in [
            "_private.Mod.#0.f.match_1.eq_1",
            "Array.get!",
            "Foo.\\d",
            "Foo.\\h1",
            "a\\\\b.c",
        ] {
            assert!(validate_name_v2(accepted).is_ok(), "{accepted}");
        }
        for rejected in [
            "", "Foo.", ".Foo", "Foo..Bar", "#", "#01", "#x", "Foo.#", "a\\q", "a\\",
        ] {
            assert!(validate_name_v2(rejected).is_err(), "{rejected}");
        }
        // The first schema keeps its closed ASCII grammar.
        assert!(validate_name("_private.Mod.#0.f").is_err());
        assert!(validate_name("Array.get!").is_err());
        let text = module(CORE_SPEC_2, theorem("_private.Mod.#0.f"));
        assert!(CoreModule::parse(&text, "1.2").is_ok());
        let text = module(CORE_SPEC_1, theorem("_private.Mod.#0.f"));
        assert!(CoreModule::parse(&text, "1.0").is_err());
    }

    #[test]
    fn version_2_rows_are_checked_against_their_kind() {
        let mut row = theorem("T.t");
        row.safety = Some(CoreSafety::Partial);
        let text = module(CORE_SPEC_2, row);
        let error = CoreModule::parse(&text, "1.2").expect_err("a theorem has no safety");
        assert!(error.contains("safety"), "{error}");

        let mut row = theorem("T.t");
        row.tags = vec![CoreTag::Recursive, CoreTag::Protected];
        let text = module(CORE_SPEC_2, row);
        let error = CoreModule::parse(&text, "1.2").expect_err("tags are ordered");
        assert!(error.contains("not strictly ordered"), "{error}");

        let mut row = theorem("T.t");
        row.all = Some(vec!["T.u".to_owned()]);
        let text = module(CORE_SPEC_2, row);
        let error = CoreModule::parse(&text, "1.2").expect_err("a row belongs to its group");
        assert!(error.contains("absent from its own group"), "{error}");

        let mut row = theorem("T.t");
        row.simp = Some(CoreSimp {
            attribute: CoreAttributeKind::Global,
            priority: 1000,
            post: true,
        });
        assert!(CoreModule::parse(&module(CORE_SPEC_2, row), "1.2").is_ok());
    }

    #[test]
    fn a_core_source_yields_its_one_payload() {
        let text = module(CORE_SPEC_2, theorem("T.t"));
        let source = format!(
            "\\begin{{lexlean}}{{M}}\n\\title{{t}}\n\\begin{{coremodule}}\n\\coredata{{{text}}}\n\\end{{coremodule}}\n\\end{{lexlean}}\n"
        );
        assert!(CoreModule::from_source(&source, "1.2").is_ok());
        assert!(CoreModule::from_source("\\begin{lexlean}{M}\n\\end{lexlean}\n", "1.2").is_err());
    }
}
