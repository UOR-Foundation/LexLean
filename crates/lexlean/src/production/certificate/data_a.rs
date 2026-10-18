//! Certificate A as kernel-term data (SPEC.md §17.17, `SP-15`).
//!
//! Certificate A of one lowered root states that the lowered program, run from
//! function 0 on the encoded arguments, converges, overflows exactly when the
//! width predicate is false, and otherwise returns the encoding of the root's
//! own Lean denotation. Here the module is built as a native core module: the
//! program literal `__prog`, the encoders `__enc_k`, one width predicate
//! `__fits_k` and one relation theorem `__rel_k` per function of the closure,
//! the boundary validators and entry, then `denote` and `root`. Each is a
//! kernel term whose every argument and universe level is written; a lemma of
//! the preservation library is applied by instantiating its recorded Π-type
//! ([`super::super::kernel`]), the arguments the generator holds from the
//! lowered program (the operator, the operand expressions, the environment)
//! given as hints, and the rest read off the types of the arguments.
//!
//! The derivation follows the lowered program node by node: the proof of a
//! source term is built together with the expression the lowering produced
//! for it, so that the program's own operator or variable number is what a
//! lemma is applied at. A lowering that realizes a construct other than as its
//! rule says leaves a lemma whose statement the kernel finds different from the
//! program's, and the kernel refuses the module (§17.17.6).

use std::collections::{BTreeMap, BTreeSet, HashMap};

use super::super::kernel::{Builder, Given, Open, Term};
use super::super::preserve;
use super::{components, CLocal, Ctx, Gen, Subject, SYNTAX};
use crate::calculus::Expr;
use crate::ir::core::{
    CoreAxiomPolicy, CoreBinderInfo, CoreDeclKind, CoreDeclaration, CoreModule, CoreReducibility,
    CoreSafety, CoreTransparency, CORE_SPEC_2,
};
use crate::ir::semantic::{MemberRef, SemanticTerm, Step};

mod boundary;
mod calls;
mod clauses;
mod encoders;
mod lean;
mod matching;
mod prims;
mod prove;
mod recplan;
mod rows;
mod src;
mod structural;
mod templates;
mod wf;

pub(super) const LIB: &str = "LexLeanPreservation";

/// The kind of a row of the module.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum RowKind {
    Theorem,
    /// A definition; `compiled` is whether the code generator runs over it,
    /// which the differential evaluator needs of `denote` and what it calls.
    Definition {
        compiled: bool,
    },
    /// The compiler's implementation record of a recursive definition: the
    /// same function written with direct recursion, for the code generator,
    /// which has no code for a recursor. The members of one mutual group
    /// name each other.
    Partial,
}

/// One row of the module.
#[derive(Debug, Clone)]
pub(super) struct Row {
    pub(super) name: String,
    pub(super) ty: Term,
    pub(super) value: Term,
    pub(super) kind: RowKind,
    pub(super) axioms: BTreeSet<String>,
    /// The rows of the mutual group the row belongs to, itself included.
    pub(super) group: Option<Vec<String>>,
}

/// A proof of a program expression's relation, with its type
/// `Conv p env e (Rel fits value)`.
#[derive(Debug, Clone, Copy)]
pub(super) struct Pf {
    pub(super) proof: Term,
    pub(super) ty: Term,
}

/// The terms of a function-typed local's companion: the width predicate,
/// function index, captured values, and relation hypothesis of the closure it
/// is bound to.
#[derive(Debug, Clone, Copy)]
pub(super) struct CompTerms {
    /// The type of `relation`, over the placeholders of the callee's own
    /// parameters.
    pub(super) relation_ty: Term,
    pub(super) fits: Term,
    pub(super) index: Term,
    pub(super) captures: Term,
    pub(super) relation: Term,
}

/// A local of a scope: the free variable standing for its value and, for a
/// function-typed one, its companions.
#[derive(Debug, Clone, Copy)]
pub(super) struct LocalTerm {
    pub(super) value: Term,
    pub(super) companion: Option<CompTerms>,
}

/// Where a term is proved: the source context, the terms of its locals, and
/// the evaluation environment the program runs it in.
#[derive(Clone)]
pub(super) struct Scope {
    pub(super) ctx: Ctx,
    pub(super) terms: Vec<LocalTerm>,
    pub(super) env: Term,
    /// The numbered `if` and `match` arms of a well-founded body that enclose
    /// the term, outermost first.
    pub(super) path: Vec<PathStep>,
}

/// One enclosing arm of a well-founded body: the free variable standing for
/// the equation the arm was selected by, and for a `match` the free variables
/// of the arm's fields.
#[derive(Clone)]
pub(super) struct PathStep {
    pub(super) step: Step,
    pub(super) hypothesis: Term,
    pub(super) fields: Vec<Term>,
}

/// One recursive call of a well-founded body: the arm it lies in, its
/// arguments, and the theorem that its arguments decrease the measure.
#[derive(Clone)]
pub(super) struct WfSite {
    pub(super) path: Vec<Step>,
    pub(super) arguments: Vec<SemanticTerm>,
    pub(super) evidence: MemberRef,
}

/// The well-founded definition whose relation is being derived.
#[derive(Clone)]
pub(super) struct WfState {
    pub(super) index: u64,
    pub(super) sites: Vec<WfSite>,
    /// The definition's type arguments, as Lean types.
    pub(super) type_arguments: Vec<Term>,
    /// The free variables of the definition's parameters.
    pub(super) parameters: Vec<Term>,
}

impl Scope {
    pub(super) fn lookup(&self, name: &str) -> Result<(&CLocal, LocalTerm), String> {
        self.ctx
            .locals
            .iter()
            .enumerate()
            .rev()
            .find(|(_, local)| local.local.name == name)
            .map(|(position, local)| (local, self.terms[position]))
            .ok_or_else(|| format!("local `{name}` is unbound"))
    }
}

/// The builder of one certificate A module.
pub(super) struct Writer<'g, 'a> {
    pub(super) gen: &'g mut Gen<'a>,
    pub(super) b: Builder,
    pub(super) name: String,
    lemmas: HashMap<String, (Term, Term)>,
    pub(super) rows: Vec<Row>,
    row_axioms: BTreeMap<String, BTreeSet<String>>,
    /// The declared exact axioms of each declaration of the source, by its
    /// Lean name.
    declared: HashMap<String, Vec<String>>,
    /// `module.__prog`.
    pub(super) prog: Term,
    /// The rendering of each expression of the program, by address.
    exprs: HashMap<*const Expr, Term>,
    /// The signature of each function whose relation has been stated.
    sigs: HashMap<u64, rows::Sig>,
    /// The well-founded definition under derivation.
    pub(super) wf: Option<WfState>,
}

impl<'g, 'a> Writer<'g, 'a> {
    pub(super) fn new(gen: &'g mut Gen<'a>, name: &str) -> Self {
        let mut b = Builder::new();
        let prog = b.konst(&format!("{name}.__prog"));
        let mut declared = HashMap::new();
        for linked in gen.source.modules.values() {
            for declaration in linked.semantic.lowered_declarations() {
                let mut axioms = declaration.axioms().to_vec();
                axioms.sort();
                // A definition a model construct elaborates to carries the
                // construct's declared set as an upper bound; the audit finds
                // none for it, as it finds for any definition that proves
                // nothing, so the exact set is empty.
                let derived = matches!(
                    declaration,
                    crate::ir::semantic::SemanticDeclaration::Definition { .. }
                ) && !linked.semantic.declarations.iter().any(|source| {
                    source.name() == declaration.name()
                        && matches!(
                            source,
                            crate::ir::semantic::SemanticDeclaration::Definition { .. }
                        )
                });
                if derived {
                    axioms.clear();
                }
                declared.insert(
                    format!("{}.{}", linked.lean_module, declaration.name()),
                    axioms,
                );
            }
        }
        Self {
            gen,
            b,
            name: name.to_owned(),
            lemmas: HashMap::new(),
            rows: Vec::new(),
            row_axioms: BTreeMap::new(),
            declared,
            prog,
            exprs: HashMap::new(),
            sigs: HashMap::new(),
            wf: None,
        }
    }

    // --- Names ------------------------------------------------------------

    pub(super) fn ts(&mut self, name: &str) -> Term {
        self.b.konst(&format!("{SYNTAX}.{name}"))
    }

    pub(super) fn lp(&mut self, name: &str) -> Term {
        self.b.konst(&format!("{LIB}.{name}"))
    }

    /// A row of this module, by its short name.
    pub(super) fn own(&mut self, name: &str) -> Term {
        let full = format!("{}.{name}", self.name);
        self.b.konst(&full)
    }

    // --- Lemmas -----------------------------------------------------------

    /// The head and type of a library declaration, its type imported from the
    /// native library data.
    pub(super) fn lemma(&mut self, name: &str) -> Result<(Term, Term), String> {
        let full = format!("{LIB}.{name}");
        if let Some(&found) = self.lemmas.get(&full) {
            return Ok(found);
        }
        let (module, declaration) = preserve::native_declaration(&full)?;
        if !declaration.levels.is_empty() {
            return Err(format!(
                "`{full}` has universe parameters, which a certificate does not instantiate"
            ));
        }
        let ty = self.b.import(module, declaration.r#type);
        let head = self.b.konst(&full);
        self.lemmas.insert(full.clone(), (head, ty));
        Ok((head, ty))
    }

    /// A library lemma opened for application with the given hints.
    pub(super) fn open(&mut self, name: &str, hints: &[(&str, Term)]) -> Result<Open, String> {
        let (head, ty) = self.lemma(name)?;
        self.b.open_lemma(name, head, ty, hints)
    }

    /// A library lemma applied to arguments at once.
    pub(super) fn apply(
        &mut self,
        name: &str,
        hints: &[(&str, Term)],
        given: &[Given],
    ) -> Result<Pf, String> {
        let mut open = self.open(name, hints)?;
        for argument in given {
            open.give(&mut self.b, *argument)?;
        }
        let applied = open.finish(&mut self.b)?;
        Ok(Pf {
            proof: applied.term,
            ty: applied.ty,
        })
    }

    /// The axioms a constant brings, as `#print axioms` reports them: the
    /// registry's for a library declaration, the row's own for a row of this
    /// module, the declared exact set for a declaration of the source, and for
    /// the runtime every generated module carries, the set its text depends
    /// on. A constant of Lean's own libraries brings none the certificate
    /// does not already pin through the lemma that mentions it.
    fn constant_axioms(&self, name: &str) -> Result<Vec<String>, String> {
        if let Some(axioms) = self.row_axioms.get(name) {
            return Ok(axioms.iter().cloned().collect());
        }
        if name.starts_with(&format!("{LIB}.")) {
            return preserve::library_axioms(name);
        }
        if let Some(axioms) = self.declared.get(name) {
            return Ok(axioms.clone());
        }
        // The eliminators, constructors, and projections of a declared type.
        let mut owner = name;
        while let Some((parent, _)) = owner.rsplit_once('.') {
            owner = parent;
            if let Some(axioms) = self.declared.get(owner) {
                return Ok(axioms.clone());
            }
        }
        if let Some(rest) = name.strip_prefix("LexLeanRuntime.") {
            return Ok(runtime_axioms(rest));
        }
        if let Some(rest) = name.strip_prefix("LexLeanCollections.") {
            return Ok(collections_axioms(rest));
        }
        Ok(Vec::new())
    }

    /// The axioms of the row whose type and value are `roots`: those of every
    /// constant they mention.
    fn row_axiom_set(&self, roots: &[Term]) -> Result<BTreeSet<String>, String> {
        let mut out = BTreeSet::new();
        let mut seen = std::collections::HashSet::new();
        let mut names = BTreeSet::new();
        let mut stack: Vec<Term> = roots.to_vec();
        while let Some(term) = stack.pop() {
            if !seen.insert(term) {
                continue;
            }
            match self.b.node(term) {
                crate::ir::core::CoreNode::Const { n, .. } => {
                    names.insert(n.clone());
                }
                crate::ir::core::CoreNode::App { f, x } => {
                    stack.push(*f);
                    stack.push(*x);
                }
                crate::ir::core::CoreNode::Lambda { t, v, .. }
                | crate::ir::core::CoreNode::Forall { t, v, .. } => {
                    stack.push(*t);
                    stack.push(*v);
                }
                crate::ir::core::CoreNode::Let { t, v, body, .. } => {
                    stack.push(*t);
                    stack.push(*v);
                    stack.push(*body);
                }
                crate::ir::core::CoreNode::Mdata { x, .. }
                | crate::ir::core::CoreNode::Projection { x, .. } => stack.push(*x),
                crate::ir::core::CoreNode::BVar { .. }
                | crate::ir::core::CoreNode::Sort { .. }
                | crate::ir::core::CoreNode::Nat { .. }
                | crate::ir::core::CoreNode::String { .. } => {}
            }
        }
        for name in names {
            out.extend(self.constant_axioms(&name)?);
        }
        Ok(out)
    }

    /// Add a row, pinning the axioms of what it applied and referenced.
    pub(super) fn push_row(
        &mut self,
        name: &str,
        kind: RowKind,
        ty: Term,
        value: Term,
    ) -> Result<(), String> {
        let full = format!("{}.{name}", self.name);
        if self.b.has_placeholder(ty) || self.b.has_placeholder(value) {
            return Err(format!(
                "the row `{full}` of certificate A has a free variable or placeholder left"
            ));
        }
        if self.b.has_loose(ty, 0) || self.b.has_loose(value, 0) {
            return Err(format!(
                "the row `{full}` of certificate A has a loose bound variable"
            ));
        }
        let axioms = self.row_axiom_set(&[ty, value])?;
        self.row_axioms.insert(full.clone(), axioms.clone());
        self.rows.push(Row {
            name: full,
            ty,
            value,
            kind,
            axioms,
            group: None,
        });
        Ok(())
    }

    /// Name the rows of one mutual group (their short names) as each other's
    /// group.
    pub(super) fn group_rows(&mut self, short_names: &[String]) {
        let full: Vec<String> = short_names
            .iter()
            .map(|name| format!("{}.{name}", self.name))
            .collect();
        for row in &mut self.rows {
            if full.contains(&row.name) {
                row.group = Some(full.clone());
            }
        }
    }

    /// The module.
    pub(super) fn finish(self, imports: Vec<String>) -> Result<CoreModule, String> {
        let heights = self.heights();
        let roots: Vec<Term> = self
            .rows
            .iter()
            .flat_map(|row| [row.ty, row.value])
            .collect();
        let (nodes, mapped) = self.b.compact(&roots);
        let declarations: Vec<CoreDeclaration> = self
            .rows
            .iter()
            .enumerate()
            .map(|(position, row)| {
                declaration(
                    row,
                    heights[&row.name],
                    mapped[position * 2],
                    mapped[position * 2 + 1],
                )
            })
            .collect();
        let module = CoreModule {
            spec: CORE_SPEC_2.to_owned(),
            imports,
            nodes,
            proof_nodes: Vec::new(),
            declarations,
        };
        module.validate(crate::LANGUAGE_1_2)?;
        Ok(module)
    }

    /// The reducibility height of each row: one above the rows it names, so
    /// the kernel unfolds a row before the rows it is stated with.
    fn heights(&self) -> HashMap<String, u32> {
        let mut heights = HashMap::new();
        for row in &self.rows {
            let mut height = 0u32;
            let mut seen = std::collections::HashSet::new();
            let mut stack = vec![row.value, row.ty];
            while let Some(term) = stack.pop() {
                if !seen.insert(term) {
                    continue;
                }
                match self.b.node(term) {
                    crate::ir::core::CoreNode::Const { n, .. } => {
                        if let Some(&below) = heights.get(n) {
                            height = height.max(below + 1);
                        }
                    }
                    crate::ir::core::CoreNode::App { f, x } => {
                        stack.push(*f);
                        stack.push(*x);
                    }
                    crate::ir::core::CoreNode::Lambda { t, v, .. }
                    | crate::ir::core::CoreNode::Forall { t, v, .. } => {
                        stack.push(*t);
                        stack.push(*v);
                    }
                    crate::ir::core::CoreNode::Let { t, v, body, .. } => {
                        stack.push(*t);
                        stack.push(*v);
                        stack.push(*body);
                    }
                    crate::ir::core::CoreNode::Mdata { x, .. }
                    | crate::ir::core::CoreNode::Projection { x, .. } => stack.push(*x),
                    crate::ir::core::CoreNode::BVar { .. }
                    | crate::ir::core::CoreNode::Sort { .. }
                    | crate::ir::core::CoreNode::Nat { .. }
                    | crate::ir::core::CoreNode::String { .. } => {}
                }
            }
            heights.insert(row.name.clone(), height);
        }
        heights
    }
}

fn declaration(row: &Row, height: u32, ty: Term, value: Term) -> CoreDeclaration {
    let (kind, reducibility, compiled, safety) = match row.kind {
        RowKind::Theorem => (CoreDeclKind::Theorem, None, false, None),
        RowKind::Definition { compiled } => (
            CoreDeclKind::Definition,
            Some(CoreReducibility::Regular(height + 1)),
            compiled,
            None,
        ),
        RowKind::Partial => (
            CoreDeclKind::Definition,
            Some(CoreReducibility::Opaque),
            false,
            Some(CoreSafety::Partial),
        ),
    };
    CoreDeclaration {
        all: Some(row.group.clone().unwrap_or_else(|| vec![row.name.clone()])),
        name: row.name.clone(),
        levels: Vec::new(),
        kind,
        reducibility,
        transparency: CoreTransparency::Semireducible,
        policy: CoreAxiomPolicy::Exact(row.axioms.iter().cloned().collect()),
        r#type: ty,
        value: Some(value),
        inductive: None,
        class: false,
        instance: None,
        generated: false,
        safety,
        compiled,
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

impl Gen<'_> {
    /// Certificate A of the root as a native core module (SPEC.md §17.17).
    pub(super) fn module_data(
        &mut self,
        root_module: &str,
        root_name: &str,
        report: &super::RootReport,
        name: &str,
    ) -> Result<CoreModule, String> {
        let graph = self.collect()?;
        self.module = name.to_owned();
        let mut imports: Vec<String> = super::LIBRARY_MODULES
            .iter()
            .map(|module| (*module).to_owned())
            .collect();
        let mut users: Vec<&str> = self
            .source
            .modules
            .values()
            .map(|linked| linked.lean_module)
            .collect();
        users.sort_unstable();
        imports.extend(users.into_iter().map(str::to_owned));
        let mut writer = Writer::new(self, name);
        writer.write(root_module, root_name, report, &graph)?;
        writer.finish(imports)
    }
}

impl Writer<'_, '_> {
    /// The rows in order: the program, the encoders, the relation of each
    /// component, then `denote`, `root`, and the boundary.
    fn write(
        &mut self,
        root_module: &str,
        root_name: &str,
        report: &super::RootReport,
        graph: &BTreeMap<u64, BTreeSet<u64>>,
    ) -> Result<(), String> {
        let program = self.program_value()?;
        let program_ty = self.ts("Program");
        self.push_row(
            "__prog",
            RowKind::Definition { compiled: false },
            program_ty,
            program,
        )?;
        self.encoders()?;
        self.gen.found.clear();
        for component in components(graph) {
            self.component(&component, graph)?;
        }
        self.root(root_module, root_name, report)
    }
}

impl<'a> Gen<'a> {
    pub(super) fn subject_of(&self, index: u64) -> Result<Subject<'a>, String> {
        self.subjects
            .get(&index)
            .cloned()
            .ok_or_else(|| format!("function {index} has no subject"))
    }
}

/// Binder information of an explicit binder, for the lambdas and Π-types of
/// rows.
pub(super) const EXPLICIT: CoreBinderInfo = CoreBinderInfo::Explicit;

/// The axioms of a declaration of the runtime every generated module carries
/// (`backend::semantic`), by its name below `LexLeanRuntime`, as `#print
/// axioms` reports them. The audit module prints the axioms of every row, so
/// a runtime that changes disagrees with this table at the first certificate.
fn runtime_axioms(name: &str) -> Vec<String> {
    let all = || {
        ["Classical.choice", "Quot.sound", "propext"]
            .iter()
            .map(|axiom| (*axiom).to_owned())
            .collect()
    };
    let some = |axioms: &[&str]| axioms.iter().map(|axiom| (*axiom).to_owned()).collect();
    match name {
        "compareBytes" | "join" => some(&["propext"]),
        "instDecimalInt"
        | "instDecimalOfFixedOfToString"
        | "instLengthableString"
        | "splitExact"
        | "utf8Decode" => all(),
        "instFixedInt8" | "instFixedInt16" | "instFixedInt32" | "instFixedInt64"
        | "instFixedUInt8" | "instFixedUInt16" | "instFixedUInt32" | "instFixedUInt64" => {
            some(&["Quot.sound", "propext"])
        }
        _ => Vec::new(),
    }
}

/// The axioms of a declaration of the collections runtime every generated
/// module carries (`backend::semantic`), by its name below
/// `LexLeanCollections`, as `#print axioms` reports them, measured with
/// `Lean.collectAxioms` over a compiled module. Every declaration not named
/// here depends on none. The audit module prints the axioms of every row, so
/// a runtime that changes disagrees with this table at the first certificate.
fn collections_axioms(name: &str) -> Vec<String> {
    let propext = || vec!["propext".to_owned()];
    match name {
        "compareCodes"
        | "compareCodes._f"
        | "compareCodes._sparseCasesOn_1"
        | "compareCodes._sunfold"
        | "compareCodes._unsafe_rec"
        | "compareCodes.match_1"
        | "graphReachable"
        | "instKeyBool"
        | "instKeyProd"
        | "lessThan"
        | "lessThan._sparseCasesOn_1"
        | "lessThan.match_1"
        | "reachableFrom"
        | "reachableFrom._f"
        | "reachableFrom._sparseCasesOn_1"
        | "reachableFrom._sunfold"
        | "reachableFrom._unsafe_rec"
        | "reachableFrom.match_1" => propext(),
        "instKeyString" => ["Classical.choice", "Quot.sound", "propext"]
            .iter()
            .map(|axiom| (*axiom).to_owned())
            .collect(),
        _ => Vec::new(),
    }
}
