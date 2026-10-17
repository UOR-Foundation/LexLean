//! Certificate B as kernel-term data (SPEC.md §17.17, `SP-16`).
//!
//! Certificate B of one rendering of a root states that the rendered crate
//! simulates the lowered program: the program and the crate are two
//! definitions, `program` and `krate`, whose terms are the target calculus
//! program and the closed Rust AST; `flags` records how the function types the
//! program closes over or applies are called; one theorem `fun<n>` per
//! function derives `FunOK program krate flags n` from the library's
//! correspondence `Corr`, one rule application per construct; `crate_ok`
//! assembles them into `CrateOK`, and `root` is the library's `simulate`
//! applied to it. Here the module is built as a native core module: a row per
//! declaration, each a kernel term whose every argument is written, the rule
//! types read from the native preservation library and instantiated one
//! binder at a time ([`super::super::kernel`]).
//!
//! What the text generator spelled for Lean's elaborator, this generator
//! writes out: the implicit program, crate, and flags of every rule, the
//! context, the judgment and the arguments the aligner holds from the
//! constructs it relates, the witnesses of `FunOK`, and the case analysis on
//! the function index. An `rfl` is `Eq.refl` at the equation the rule states,
//! which the kernel decides by evaluation. The statements are the text
//! generator's, so the kernel term each theorem states is the one the text
//! elaborated to.

// The shape tests of the aligner and of the proof builders end in a refusal
// (`Err`), never in a default realization: a construct they do not match is
// reported, not certified. The lint that denies a catch-all in this module's
// parent guards the lowering of constructs, which these matches are not, and
// the exhaustive match on every construct that matters is the block
// judgment, which `audit-production` holds to name each one.
#![allow(
    clippy::wildcard_enum_match_arm,
    clippy::match_wildcard_for_single_variants
)]

mod align;
mod syntax;

use std::collections::BTreeMap;

pub use align::RULES;

use super::data_a::{RowKind, Writer};
use super::{Gen, Source};
use crate::calculus::rust::ast::{Crate, ItemDef};
use crate::calculus::{Function, Program};
use crate::ir::core::{CoreBinderInfo, CoreLevel, CoreModule, CoreNode};
use crate::production::kernel::Term as K;
use crate::production::lower::{Layout, LIMIT};
use crate::production::preserve;
use align::Aligner;

/// The library module certificate B imports, which imports the rest.
pub const SOUNDNESS_MODULE: &str = "LexLeanPreservation.RustSound";

const RUST: &str = "LexLeanPreservation.Rust";

/// The rows a certificate B module declares, in the order it declares them.
struct Rows {
    functions: Vec<(String, K, K)>,
}

impl<'g, 'a> Aligner<'g, 'a> {
    fn rust_const(&mut self, name: &str) -> K {
        self.w.b.konst(&format!("{RUST}.{name}"))
    }

    fn row(&mut self, name: &str) -> K {
        self.w.own(name)
    }

    fn level_zero() -> CoreLevel {
        CoreLevel::Zero
    }

    /// `Eq.refl` at the equation `equation` states, whose sides the kernel
    /// compares by evaluation.
    fn refl_of(&mut self, equation: K) -> Result<K, String> {
        let (head, parts) = self.w.b.spine(equation);
        let levels = match self.w.b.node(head) {
            CoreNode::Const { n, u } if n == "Eq" && parts.len() == 3 => u.clone(),
            _ => {
                return Err(format!(
                    "a reflexivity proof at {}, which is not an equation",
                    self.w.b.show(equation)
                ));
            }
        };
        let refl = self.w.b.konst_at("Eq.refl", levels);
        Ok(self.w.b.apps(refl, &[parts[0], parts[1]]))
    }

    /// The type `FunOK` states of function `index`, over the unfolded
    /// definition.
    fn fun_ok_body(&mut self, index: u64) -> Result<K, String> {
        let (module, declaration) = preserve::native_declaration(&format!("{RUST}.FunOK"))?;
        let value = declaration
            .value
            .ok_or("FunOK is declared without a value")?;
        let mut term = self.w.b.import(module, value);
        let f = self.w.nat_u(index);
        for argument in [self.p, self.c, self.a, f] {
            let CoreNode::Lambda { v, .. } = self.w.b.node(term).clone() else {
                return Err("FunOK is not a function of its four arguments".to_owned());
            };
            term = self.w.b.instantiate_beta(v, argument);
        }
        Ok(term)
    }

    /// `Exists.intro` of a witness and the proof of the predicate at it.
    fn exists_intro(
        &mut self,
        statement: K,
        witness: K,
        proof: impl FnOnce(&mut Self, K) -> Result<K, String>,
    ) -> Result<K, String> {
        let (head, parts) = self.w.b.spine(statement);
        let CoreNode::Const { n, u } = self.w.b.node(head).clone() else {
            return Err("an existential is not a constant application".to_owned());
        };
        if n != "Exists" || parts.len() != 2 {
            return Err(format!(
                "`{}` is not an existential",
                self.w.b.show(statement)
            ));
        }
        let CoreNode::Lambda { v, .. } = self.w.b.node(parts[1]).clone() else {
            return Err("the predicate of an existential is not a function".to_owned());
        };
        let at = self.w.b.instantiate_beta(v, witness);
        let inner = proof(self, at)?;
        let intro = self.w.b.konst_at("Exists.intro", u);
        Ok(self.w.b.apps(intro, &[parts[0], parts[1], witness, inner]))
    }

    /// The two halves of a conjunction.
    fn conjuncts(&self, statement: K) -> Result<(K, K), String> {
        let (head, parts) = self.w.b.spine(statement);
        match self.w.b.node(head) {
            CoreNode::Const { n, .. } if n == "And" && parts.len() == 2 => Ok((parts[0], parts[1])),
            _ => Err(format!(
                "`{}` is not a conjunction",
                self.w.b.show(statement)
            )),
        }
    }

    fn and_intro(&mut self, left: K, right: K, proof_left: K, proof_right: K) -> K {
        let intro = self.w.b.konst("And.intro");
        self.w
            .b
            .apps(intro, &[left, right, proof_left, proof_right])
    }

    /// The proof of `FunOK program krate flags index` by a derivation, or by
    /// the emptiness of a parameter's type.
    fn function_proof(&mut self, index: u64) -> Result<K, String> {
        let function = self.function(index)?;
        let (parameters, body) = self
            .krate()
            .items
            .iter()
            .find_map(|item| match item {
                ItemDef::Function {
                    name: crate::calculus::rust::ast::Ident::Function(found),
                    parameters,
                    body,
                    ..
                } if *found == index => Some((parameters, body)),
                _ => None,
            })
            .ok_or_else(|| format!("the crate does not render function {index}"))?;
        let statement = self.fun_ok_body(index)?;
        let fn_term = self.w.function_term(function)?;
        let empty_parameter = function
            .types
            .iter()
            .position(|ty| self.uninh(&self.empty.clone(), ty));
        self.exists_intro(statement, fn_term, |this, at| {
            let (found, rest) = this.conjuncts(at)?;
            let refl = this.refl_of(found)?;
            let (left, right) = this.disjuncts(rest)?;
            let choice = match empty_parameter {
                None => {
                    let proof = this.inhabited_case(index, function, parameters, body, left)?;
                    this.or_inl(left, right, proof)
                }
                Some(position) => {
                    let proof = this.empty_case(function, position, right)?;
                    this.or_inr(left, right, proof)
                }
            };
            Ok(this.and_intro(found, rest, refl, choice))
        })
    }

    fn disjuncts(&self, statement: K) -> Result<(K, K), String> {
        let (head, parts) = self.w.b.spine(statement);
        match self.w.b.node(head) {
            CoreNode::Const { n, .. } if n == "Or" && parts.len() == 2 => Ok((parts[0], parts[1])),
            _ => Err(format!(
                "`{}` is not a disjunction",
                self.w.b.show(statement)
            )),
        }
    }

    fn or_inl(&mut self, left: K, right: K, proof: K) -> K {
        let intro = self.w.b.konst("Or.inl");
        self.w.b.apps(intro, &[left, right, proof])
    }

    fn or_inr(&mut self, left: K, right: K, proof: K) -> K {
        let intro = self.w.b.konst("Or.inr");
        self.w.b.apps(intro, &[left, right, proof])
    }

    /// The disjunct of a function whose parameters can all be given: its
    /// parameters, result, body, and context, and the derivation.
    fn inhabited_case(
        &mut self,
        index: u64,
        function: &Function,
        parameters: &[(
            crate::calculus::rust::ast::Pat,
            crate::calculus::rust::ast::Type,
        )],
        body: &crate::calculus::rust::ast::Block,
        statement: K,
    ) -> Result<K, String> {
        let patterns: Vec<crate::calculus::rust::ast::Pat> = parameters
            .iter()
            .map(|(pattern, _)| pattern.clone())
            .collect();
        let empty = self.ctx_empty();
        let scope = self.param_ctx(&empty, &function.parameters, &function.types, &patterns)?;
        let fallible = self.fallible_fn(index);
        let derivation = self.bg(
            &scope,
            fallible,
            fallible,
            &function.body,
            &function.result,
            &body.lets,
            &body.tail,
        )?;
        let params = self.w.rs_parameters(parameters);
        let result = self.krate_result(index)?;
        let lets = self.w.rs_lets(&body.lets);
        let tail = self.w.rs_expr(&body.tail);
        let gamma = scope.term();
        let witnesses = [params, result, lets, tail, gamma];
        self.nested_exists(statement, &witnesses, &mut |this, innermost| {
            let (found, rest) = this.conjuncts(innermost)?;
            let first = this.refl_of(found)?;
            let (ctx_eq, corr) = this.conjuncts(rest)?;
            let second = this.refl_of(ctx_eq)?;
            let last = this.and_intro(ctx_eq, corr, second, derivation.proof);
            Ok(this.and_intro(found, rest, first, last))
        })
    }

    fn krate_result(&mut self, index: u64) -> Result<K, String> {
        let result = self
            .krate()
            .items
            .iter()
            .find_map(|item| match item {
                ItemDef::Function {
                    name: crate::calculus::rust::ast::Ident::Function(found),
                    result,
                    ..
                } if *found == index => Some(result),
                _ => None,
            })
            .ok_or_else(|| format!("the crate does not render function {index}"))?;
        Ok(self.w.rs_type(result))
    }

    /// Nested existentials introduced by their witnesses, in order, and the
    /// proof of what is left.
    fn nested_exists(
        &mut self,
        statement: K,
        witnesses: &[K],
        finish: &mut dyn FnMut(&mut Self, K) -> Result<K, String>,
    ) -> Result<K, String> {
        match witnesses.split_first() {
            None => finish(self, statement),
            Some((witness, rest)) => self.exists_intro(statement, *witness, |this, at| {
                this.nested_exists(at, rest, finish)
            }),
        }
    }

    /// The disjunct of a function with a parameter whose type no value
    /// inhabits: never called, so the correspondence says nothing of it.
    fn empty_case(
        &mut self,
        function: &Function,
        position: usize,
        statement: K,
    ) -> Result<K, String> {
        let empty = self.empty.clone();
        let u = {
            let nat = self.w.b.konst("Nat");
            let items: Vec<K> = empty.iter().map(|n| self.w.nat_u(*n)).collect();
            self.w.b.list(nat, &items)
        };
        let t = self.w.ty_term(&function.types[position]);
        let ty_ty = self.w.ts("Ty");
        let items: Vec<K> = function.types.iter().map(|ty| self.w.ty_term(ty)).collect();
        let witnesses = [u, t];
        self.nested_exists(statement, &witnesses, &mut |this, innermost| {
            let (valid, rest) = this.conjuncts(innermost)?;
            let first = this.refl_of(valid)?;
            let (member, uninhabited) = this.conjuncts(rest)?;
            let second = this.refl_of(uninhabited)?;
            let mut proof = {
                let head = this.w.b.konst0("List.Mem.head");
                let after = this.w.b.list(ty_ty, &items[position + 1..]);
                this.w.b.apps(head, &[ty_ty, items[position], after])
            };
            for at in (0..position).rev() {
                let tail = this.w.b.konst0("List.Mem.tail");
                let after = this.w.b.list(ty_ty, &items[at + 1..]);
                proof = this
                    .w
                    .b
                    .apps(tail, &[ty_ty, items[position], items[at], after, proof]);
            }
            let tail_pair = this.and_intro(member, uninhabited, proof, second);
            Ok(this.and_intro(valid, rest, first, tail_pair))
        })
    }

    /// `crate_ok`'s value: the case analysis on the function index that
    /// hands each function its theorem and refutes an index past the last.
    fn crate_ok_value(&mut self, count: u64) -> Result<K, String> {
        let nat = self.w.b.konst("Nat");
        let function_ty = self.w.ts("Function");
        let option = self.w.b.konst0("Option");
        let option_function = self.w.b.app(option, function_ty);
        let functions = {
            let head = self.w.ts("Program.functions");
            self.w.b.app(head, self.p)
        };
        let list_ty = self.w.b.list_ty(function_ty);
        let _ = list_ty;
        let index_head = self.w.b.konst("LexLeanRuntime.index");
        let indexable = {
            let head = self.w.b.konst("LexLeanRuntime.instIndexableList");
            self.w.b.app(head, function_ty)
        };
        let list_function = self.w.b.list_ty(function_ty);
        let fun_ok = self.rust_const("FunOK");
        let at = |this: &mut Self, x: K, d: u64| -> K {
            let zero = [CoreLevel::Zero, CoreLevel::Zero, CoreLevel::Zero];
            let add = this.w.b.konst_at("HAdd.hAdd", zero.to_vec());
            let inst = {
                let head = this.w.b.konst_at("instHAdd", vec![CoreLevel::Zero]);
                let add_nat = this.w.b.konst("instAddNat");
                this.w.b.apps(head, &[nat, add_nat])
            };
            let d = this.w.nat_u(d);
            this.w.b.apps(add, &[nat, nat, nat, inst, x, d])
        };
        let statement = |this: &mut Self, x: K, d: u64, fun: K, h: K| -> (K, K, K) {
            // The type of `h` and the conclusion, over `x`, `fun`.
            let position = at(this, x, d);
            let looked = this.w.b.apps(
                index_head,
                &[list_function, function_ty, indexable, functions, position],
            );
            let some = this.w.b.konst0("Option.some");
            let found = this.w.b.apps(some, &[function_ty, fun]);
            let eq = this.w.b.konst1("Eq");
            let hypothesis = this.w.b.apps(eq, &[option_function, looked, found]);
            let conclusion = this.w.b.apps(fun_ok, &[this.p, this.c, this.a, position]);
            let _ = h;
            (hypothesis, conclusion, position)
        };
        // The innermost level: an index past the last function looks up
        // nothing.
        let x = self.w.b.free();
        let fun = self.w.b.free();
        let h = self.w.b.free();
        let (hypothesis, conclusion, _) = statement(self, x, count, fun, h);
        let none = {
            let head = self.w.b.konst0("Option.none");
            self.w.b.app(head, function_ty)
        };
        let some = self.w.b.konst0("Option.some");
        let found = self.w.b.apps(some, &[function_ty, fun]);
        let refuted = {
            // `Option.noConfusion` takes the equality of the carriers and the
            // heterogeneous equality of the options: the hypothesis, which
            // the kernel reads as `none = some fn` by evaluating the lookup
            // past the last function, is the second.
            let head = self
                .w
                .b
                .konst_at("Option.noConfusion", vec![CoreLevel::Zero, CoreLevel::Zero]);
            let type_level = CoreLevel::Succ(Box::new(CoreLevel::Succ(Box::new(CoreLevel::Zero))));
            let refl = self.w.b.konst_at("Eq.refl", vec![type_level]);
            let type0 = self.w.b.type0();
            let carriers = self.w.b.apps(refl, &[type0, function_ty]);
            let heq = {
                let head = self.w.b.konst_at(
                    "heq_of_eq",
                    vec![CoreLevel::Succ(Box::new(CoreLevel::Zero))],
                );
                self.w.b.apps(head, &[option_function, none, found, h])
            };
            self.w.b.apps(
                head,
                &[
                    conclusion,
                    function_ty,
                    none,
                    function_ty,
                    found,
                    carriers,
                    heq,
                ],
            )
        };
        let with_h = self
            .w
            .b
            .lam_over("h", CoreBinderInfo::Explicit, hypothesis, h, refuted);
        let with_fun = self
            .w
            .b
            .lam_over("fn", CoreBinderInfo::Explicit, function_ty, fun, with_h);
        let mut level = self
            .w
            .b
            .lam_over("x", CoreBinderInfo::Explicit, nat, x, with_fun);
        for d in (0..count).rev() {
            let x = self.w.b.free();
            let y = self.w.b.free();
            let fun = self.w.b.free();
            let h = self.w.b.free();
            // The motive: the statement at `y`.
            let (hypothesis_y, conclusion_y, _) = statement(self, y, d, fun, h);
            let pi_h =
                self.w
                    .b
                    .pi_over("h", CoreBinderInfo::Explicit, hypothesis_y, h, conclusion_y);
            let pi_fun = self
                .w
                .b
                .pi_over("fn", CoreBinderInfo::Explicit, function_ty, fun, pi_h);
            let motive = self
                .w
                .b
                .lam_over("y", CoreBinderInfo::Explicit, nat, y, pi_fun);
            // The zero case hands over the theorem of function `d`.
            let fun0 = self.w.b.free();
            let h0 = self.w.b.free();
            let zero = self.w.b.konst("Nat.zero");
            let (hypothesis_0, _, _) = statement(self, zero, d, fun0, h0);
            let theorem = self.row(&format!("fun{d}"));
            let zero_h =
                self.w
                    .b
                    .lam_over("h", CoreBinderInfo::Explicit, hypothesis_0, h0, theorem);
            let zero_case =
                self.w
                    .b
                    .lam_over("fn", CoreBinderInfo::Explicit, function_ty, fun0, zero_h);
            let cases = self.w.b.konst_at("Nat.casesOn", vec![Self::level_zero()]);
            let applied = self.w.b.apps(cases, &[motive, x, zero_case, level]);
            level = self
                .w
                .b
                .lam_over("x", CoreBinderInfo::Explicit, nat, x, applied);
        }
        Ok(level)
    }

    fn crate_ok_type(&mut self) -> K {
        let head = self.rust_const("CrateOK");
        self.w.b.apps(head, &[self.p, self.c, self.a])
    }

    /// `∀ n f, FunSem program krate flags n f` and its proof by `simulate`.
    fn root_row(&mut self) -> Result<(K, K), String> {
        let nat = self.w.b.konst("Nat");
        let n = self.w.b.free();
        let f = self.w.b.free();
        let fun_sem = self.rust_const("FunSem");
        let statement = self.w.b.apps(fun_sem, &[self.p, self.c, self.a, n, f]);
        let simulate = self.rust_const("simulate");
        let crate_ok = self.row("crate_ok");
        let proof = self
            .w
            .b
            .apps(simulate, &[self.p, self.c, self.a, crate_ok, n, f]);
        let inner_ty = self
            .w
            .b
            .pi_over("f", CoreBinderInfo::Explicit, nat, f, statement);
        let ty = self
            .w
            .b
            .pi_over("n", CoreBinderInfo::Explicit, nat, n, inner_ty);
        let inner_value = self
            .w
            .b
            .lam_over("f", CoreBinderInfo::Explicit, nat, f, proof);
        let value = self
            .w
            .b
            .lam_over("n", CoreBinderInfo::Explicit, nat, n, inner_value);
        Ok((ty, value))
    }
}

/// Certificate B of one lowered root's rendering as a native core module
/// (SPEC.md §17.17).
///
/// # Errors
///
/// The reason the aligner cannot relate the crate to the program, or, prefixed
/// with [`LIMIT`], that the certificate would exceed `max_bytes`.
pub fn certificate_b_data(
    program: &Program,
    krate: &Crate,
    module: &str,
    max_bytes: u64,
) -> Result<CoreModule, String> {
    let work = || build(program, krate, module, max_bytes);
    // A derivation nests as deeply as a match has arms, and the kernel terms
    // it is built from are walked recursively: the work has a stack of its
    // own.
    std::thread::scope(|scope| {
        std::thread::Builder::new()
            .stack_size(1 << 30)
            .spawn_scoped(scope, work)
            .map_err(|error| error.to_string())?
            .join()
            .map_err(|_| "certificate B overran its stack".to_owned())?
    })
}

fn build(
    program: &Program,
    krate: &Crate,
    module: &str,
    max_bytes: u64,
) -> Result<CoreModule, String> {
    let modules = BTreeMap::new();
    let layout = Layout {
        functions: Vec::new(),
        adts: Vec::new(),
    };
    let mut generator = Gen {
        source: Source { modules: &modules },
        layout: &layout,
        program,
        instances: BTreeMap::new(),
        lambdas: BTreeMap::new(),
        templates: BTreeMap::new(),
        adts: BTreeMap::new(),
        subjects: BTreeMap::new(),
        found: BTreeMap::new(),
        nested: BTreeMap::new(),
        limit: max_bytes,
        module: module.to_owned(),
    };
    let writer = Writer::new(&mut generator, module);
    let mut aligner = Aligner::new(writer, program, krate, max_bytes, module)?;
    let program_value = aligner.w.program_value()?;
    let crate_value = aligner.w.rs_crate(krate);
    aligner.count_new(program_value);
    aligner.count_new(crate_value);
    let weighed = aligner.weighed();
    if weighed > max_bytes {
        return Err(format!(
            "{LIMIT}the program and crate terms are at least {weighed} bytes, beyond the {max_bytes} bytes of max_file_bytes"
        ));
    }
    let mut rows = Rows {
        functions: Vec::new(),
    };
    for index in 0..program.functions.len() as u64 {
        let proof = aligner
            .function_proof(index)
            .map_err(|reason| match aligner.over.clone() {
                Some(refusal) => refusal,
                None => format!("function {index}: {reason}"),
            })?;
        let fun_ok = aligner.rust_const("FunOK");
        let at = aligner.w.nat_u(index);
        let statement = aligner
            .w
            .b
            .apps(fun_ok, &[aligner.p, aligner.c, aligner.a, at]);
        rows.functions
            .push((format!("fun{index}"), statement, proof));
    }
    let flags_value = aligner.flags()?;
    let count = program.functions.len() as u64;
    let crate_ok_value = aligner.crate_ok_value(count)?;
    let crate_ok_ty = aligner.crate_ok_type();
    let (root_ty, root_value) = aligner.root_row()?;
    let program_ty = aligner.w.ts("Program");
    let crate_ty = aligner.w.rs("Crate");
    let flags_ty = aligner.rust_const("Flags");
    let theorem = RowKind::Theorem;
    let definition = RowKind::Definition { compiled: false };
    aligner
        .w
        .push_row("program", definition, program_ty, program_value)?;
    aligner
        .w
        .push_row("krate", definition, crate_ty, crate_value)?;
    aligner
        .w
        .push_row("flags", definition, flags_ty, flags_value)?;
    for (name, statement, proof) in rows.functions {
        aligner.w.push_row(&name, theorem, statement, proof)?;
    }
    aligner
        .w
        .push_row("crate_ok", theorem, crate_ok_ty, crate_ok_value)?;
    aligner.w.push_row("root", theorem, root_ty, root_value)?;
    aligner.w.finish(vec![SOUNDNESS_MODULE.to_owned()])
}

/// Certificate B as data, its Lean text, and its record, under `max_bytes`.
///
/// # Errors
///
/// As [`certificate_b_data`]; or, prefixed with [`LIMIT`], that the text
/// exceeds `max_bytes`.
pub fn render(
    program: &Program,
    krate: &Crate,
    module: &str,
    max_bytes: u64,
) -> Result<(CoreModule, String, String), String> {
    let data = certificate_b_data(program, krate, module, max_bytes)?;
    let text = crate::backend::core::lean_text_legacy(&data, module)
        .map_err(|diagnostic| diagnostic.message)?;
    let source = super::record_source(module, &data, "B")?;
    // A certificate held as data weighs what it is recorded as, the notation
    // (as certificate A does): the text carries the decoder and the data a
    // second time, escaped, which is the backend's form and not the
    // certificate's size.
    if source.len() as u64 > max_bytes {
        return Err(format!(
            "{LIMIT}the certificate is {} bytes, beyond the {max_bytes} bytes of max_file_bytes",
            source.len()
        ));
    }
    Ok((data, text, source))
}

/// The crates of `crates` as computable definitions `__crate<i> :
/// RustSyntax.Crate` of one native core module, rendered like a certificate: the Rust machine the
/// conformance cases run evaluates a crate by its term, which is the same
/// term certificate B states its theorem about.
///
/// # Errors
///
/// A module that does not validate.
pub fn crates_module(module: &str, crates: &[&Crate]) -> Result<(CoreModule, String), String> {
    let program = Program {
        spec: String::new(),
        adts: Vec::new(),
        functions: Vec::new(),
    };
    let modules = BTreeMap::new();
    let layout = Layout {
        functions: Vec::new(),
        adts: Vec::new(),
    };
    let mut generator = Gen {
        source: Source { modules: &modules },
        layout: &layout,
        program: &program,
        instances: BTreeMap::new(),
        lambdas: BTreeMap::new(),
        templates: BTreeMap::new(),
        adts: BTreeMap::new(),
        subjects: BTreeMap::new(),
        found: BTreeMap::new(),
        nested: BTreeMap::new(),
        limit: u64::MAX,
        module: module.to_owned(),
    };
    let mut writer = Writer::new(&mut generator, module);
    for (position, krate) in crates.iter().enumerate() {
        let value = writer.rs_crate(krate);
        let ty = writer.rs("Crate");
        writer.push_row(
            &format!("__crate{position}"),
            RowKind::Definition { compiled: true },
            ty,
            value,
        )?;
    }
    let data = writer.finish(vec!["LexLeanTarget.RustSyntax".to_owned()])?;
    let text = crate::backend::core::lean_text_legacy(&data, module)
        .map_err(|diagnostic| diagnostic.message)?;
    Ok((data, text))
}

/// A certificate B module restated over another crate: every row as it is,
/// but the term of `krate`. The theorems then claim the derivations about a
/// crate they were not written for, which the kernel refuses.
///
/// # Errors
///
/// A module that does not validate.
pub fn restated_over(data: &CoreModule, module: &str, krate: &Crate) -> Result<CoreModule, String> {
    let program = Program {
        spec: String::new(),
        adts: Vec::new(),
        functions: Vec::new(),
    };
    let modules = BTreeMap::new();
    let layout = Layout {
        functions: Vec::new(),
        adts: Vec::new(),
    };
    let mut generator = Gen {
        source: Source { modules: &modules },
        layout: &layout,
        program: &program,
        instances: BTreeMap::new(),
        lambdas: BTreeMap::new(),
        templates: BTreeMap::new(),
        adts: BTreeMap::new(),
        subjects: BTreeMap::new(),
        found: BTreeMap::new(),
        nested: BTreeMap::new(),
        limit: u64::MAX,
        module: module.to_owned(),
    };
    let mut writer = Writer::new(&mut generator, module);
    let own = format!("{module}.krate");
    for row in &data.declarations {
        let ty = writer.b.import(data, row.r#type);
        let value = match row.value {
            Some(_) if row.name == own => writer.rs_crate(krate),
            Some(value) => writer.b.import(data, value),
            None => return Err(format!("the row `{}` has no value", row.name)),
        };
        let kind = match row.kind {
            crate::ir::core::CoreDeclKind::Theorem => RowKind::Theorem,
            _ => RowKind::Definition {
                compiled: row.compiled,
            },
        };
        let short = row
            .name
            .strip_prefix(&format!("{module}."))
            .ok_or_else(|| format!("the row `{}` is outside the module", row.name))?;
        writer.push_row(short, kind, ty, value)?;
    }
    writer.finish(data.imports.clone())
}

/// `data` with its module renamed: every row and every constant of the module
/// under `to` instead of `from`.
pub fn renamed(data: &CoreModule, from: &str, to: &str) -> CoreModule {
    let prefix = format!("{from}.");
    let swap = |name: &str| -> String {
        match name.strip_prefix(&prefix) {
            Some(rest) => format!("{to}.{rest}"),
            None => name.to_owned(),
        }
    };
    let mut out = data.clone();
    for node in &mut out.nodes {
        if let CoreNode::Const { n, .. } = node {
            *n = swap(n);
        }
    }
    for row in &mut out.declarations {
        row.name = swap(&row.name);
        if let Some(all) = &mut row.all {
            for member in all {
                *member = swap(member);
            }
        }
    }
    out
}

/// Run `work` over a writer that has no source: the kernel terms of the Rust
/// syntax need only the builder.
fn with_writer<R>(module: &str, work: impl FnOnce(&mut Writer<'_, '_>) -> R) -> R {
    let program = Program {
        spec: String::new(),
        adts: Vec::new(),
        functions: Vec::new(),
    };
    let modules = BTreeMap::new();
    let layout = Layout {
        functions: Vec::new(),
        adts: Vec::new(),
    };
    let mut generator = Gen {
        source: Source { modules: &modules },
        layout: &layout,
        program: &program,
        instances: BTreeMap::new(),
        lambdas: BTreeMap::new(),
        templates: BTreeMap::new(),
        adts: BTreeMap::new(),
        subjects: BTreeMap::new(),
        found: BTreeMap::new(),
        nested: BTreeMap::new(),
        limit: u64::MAX,
        module: module.to_owned(),
    };
    let mut writer = Writer::new(&mut generator, module);
    work(&mut writer)
}

/// The constructor of `RustSyntax.Item` a runtime item is written as, and the
/// number of arguments it takes.
#[must_use]
pub fn item_constructor(item: crate::calculus::rust::runtime::Item) -> Option<(String, usize)> {
    with_writer("Scratch", |writer| {
        let term = writer.rs_item(item);
        let (head, arguments) = writer.b.spine(term);
        match writer.b.node(head) {
            CoreNode::Const { n, .. } => n
                .strip_prefix("LexLeanTarget.RustSyntax.Item.")
                .map(|name| (name.to_owned(), arguments.len())),
            _ => None,
        }
    })
}
