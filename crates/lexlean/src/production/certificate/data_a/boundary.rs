//! The boundary validators and the entry (SPEC.md §17.17).
//!
//! A root whose parameters carry §17.12's invariant has, besides its own
//! relation, one validator function per type that carries it. The module
//! states four things of each validator `k`: the Boolean function `__valid_k`
//! it computes, the proposition `__inv_k` it decides, the theorem `__viff_k`
//! that the one is the other, and the theorem `__vrel_k` that the program's
//! function `k` returns `__valid_k` of the encoding of its argument. A
//! container's four come from the preservation library's lemmas; a document
//! type's, and those of the containers nested in a recursive group of them,
//! are written by the recursor of the type, one motive per type of the group
//! and one minor premise per constructor, exactly as the group's encoders are.
//! The entry then states what the root returns on an input the validators
//! accept (`entry_accepts`) and on one they refuse (`entry_refuses`).
//!
//! Every equation the text proved by unfolding a definition or by `simp` is
//! here a definitional equality the kernel checks, or a rewriting by
//! `Eq.ndrec` along an equation the validators' decision theorems give.

use std::collections::{BTreeMap, BTreeSet};

use super::super::{checked, components, element_of, option_value, parts, Validation};
use super::recplan::{PositionKind, RecCtor, RecPlan};
use super::rows::Sig;
use super::{Pf, RowKind, Writer, EXPLICIT};
use crate::calculus::{Expr, Function, Shape, Ty};
use crate::ir::core::CoreLevel;
use crate::ir::semantic::SemanticType;
use crate::production::kernel::{Given, Term};
use crate::production::source::Site;

/// One validator of the root's boundary.
#[derive(Clone)]
struct Validator {
    index: u64,
    ty: SemanticType,
    module: String,
    kind: Validation,
    /// The Lean type it validates.
    lean: Term,
    /// The encoder of that type.
    enc: Term,
}

/// The validators of the boundary by function index.
struct Boundary {
    validators: BTreeMap<u64, Validator>,
}

impl Boundary {
    fn get(&self, index: u64) -> Result<&Validator, String> {
        self.validators
            .get(&index)
            .ok_or_else(|| format!("function {index} is not a validator"))
    }
}

/// Which of a validator's four statements a recursor builds.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Family {
    /// `__valid_k`.
    Valid,
    /// `__inv_k`.
    Inv,
    /// `__viff_k`.
    Viff,
    /// `__vrel_k`.
    Vrel,
}

/// How a validator's recursive calls are written.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Mode {
    /// By the induction hypotheses of the recursor.
    Ih,
    /// By the implementation record of the group's other members.
    Unsafe,
}

/// A field of a constructor that its validator checks.
#[derive(Clone)]
struct Check {
    field: usize,
    validator: u64,
}

/// One constructor of a validated type, as its validator reads it.
#[derive(Clone)]
struct CtorSpec {
    fields: Vec<SemanticType>,
    /// Per field, the position of the group it recurses on.
    recursive: Vec<Option<usize>>,
    /// The checked fields, in the order the validator tests them.
    checks: Vec<Check>,
    /// Whether the last check is the validator's result itself, else the
    /// result after the last check is `true`.
    direct: bool,
    /// The arm of the validator's `match` that takes this constructor, if it
    /// matches.
    arm: Option<usize>,
}

/// One validated type of a group.
#[derive(Clone)]
struct GroupPosition {
    validator: Validator,
    ctors: Vec<CtorSpec>,
}

/// The validators defined together: the types of a recursive group, or one
/// document type.
struct Group {
    positions: Vec<GroupPosition>,
    /// The group's recursors; none for a document outside any recursion,
    /// which is written by its eliminator.
    plan: Option<RecPlan>,
}

/// A checked field met while deriving a validator's relation.
#[derive(Clone)]
struct Checked {
    validator: Validator,
    x: Term,
    /// The proof of the validator's relation at `x`.
    proof: Term,
    /// `__valid_c x`.
    valid: Term,
}

/// The terms the variables of a validator's body stand for.
struct Binding {
    vars: Vec<(u64, Term)>,
    /// The encodings of a pair's components, for `first` and `second`.
    pair: Option<(Term, Term)>,
}

impl<'a> Writer<'_, 'a> {
    /// The validators and entry of a root with invariant-carrying parameters.
    pub(super) fn boundary(
        &mut self,
        sig: &Sig,
        parameters: &[SemanticType],
    ) -> Result<(), String> {
        self.gen.verify_boundary(parameters)?;
        let checks: Vec<(u64, SemanticType, String, Validation)> = self
            .gen
            .checks()
            .into_iter()
            .map(|(index, (ty, module, kind))| (index, ty.clone(), module.to_owned(), kind.clone()))
            .collect();
        let entry = self
            .gen
            .entry_of()
            .map(|(index, validators)| (index, validators.to_vec()));
        if checks.is_empty() && entry.is_none() {
            return Ok(());
        }
        let adt_graph = self.gen.adt_graph()?;
        let mut boundary = Boundary {
            validators: BTreeMap::new(),
        };
        for (index, ty, module, kind) in checks {
            let lean = self.lean_ty(&ty)?;
            let enc = self.enc(&ty)?;
            boundary.validators.insert(
                index,
                Validator {
                    index,
                    ty,
                    module,
                    kind,
                    lean,
                    enc,
                },
            );
        }
        let graph: BTreeMap<u64, BTreeSet<u64>> = boundary
            .validators
            .iter()
            .map(|(index, validator)| (*index, checked(&validator.kind).into_iter().collect()))
            .collect();
        for component in components(&graph) {
            let mut grouped = Vec::new();
            for index in &component {
                let validator = boundary.get(*index)?;
                grouped.push(
                    self.gen
                        .grouped(&validator.ty, &validator.kind, &adt_graph)?,
                );
            }
            let recursive = grouped.iter().any(|grouped| *grouped);
            if (recursive && !grouped.iter().all(|grouped| *grouped))
                || (!recursive && component.len() > 1)
            {
                return Err(format!(
                    "the validators {component:?} call one another outside a recursive group"
                ));
            }
            let mut members = component.clone();
            members.sort_unstable();
            if recursive {
                self.recursive_validators(&boundary, &members)?;
            } else {
                self.plain_validator(&boundary, members[0])?;
            }
        }
        match entry {
            Some((index, validators)) => {
                self.entry_section(&boundary, index, &validators, sig, parameters)
            }
            None => Ok(()),
        }
    }

    // --- Small terms ------------------------------------------------------

    fn bool_ty(&mut self) -> Term {
        self.b.konst("Bool")
    }

    fn true_prop(&mut self) -> Term {
        self.b.konst("True")
    }

    fn iff(&mut self, a: Term, b: Term) -> Term {
        let head = self.b.konst("Iff");
        self.b.apps(head, &[a, b])
    }

    fn and_prop(&mut self, a: Term, b: Term) -> Term {
        let head = self.b.konst("And");
        self.b.apps(head, &[a, b])
    }

    /// `c = true`.
    fn is_true(&mut self, c: Term) -> Term {
        let bool = self.bool_ty();
        let tru = self.tru();
        self.eq(bool, c, tru)
    }

    /// `Eq.refl a` at the type `ty : Type`.
    fn refl(&mut self, ty: Term, a: Term) -> Term {
        let head = self.b.konst1("Eq.refl");
        self.b.apps(head, &[ty, a])
    }

    fn valid_const(&mut self, index: u64) -> Term {
        self.own(&format!("__valid_{index}"))
    }

    fn inv_const(&mut self, index: u64) -> Term {
        self.own(&format!("__inv_{index}"))
    }

    fn viff_const(&mut self, index: u64) -> Term {
        self.own(&format!("__viff_{index}"))
    }

    fn vrel_const(&mut self, index: u64) -> Term {
        self.own(&format!("__vrel_{index}"))
    }

    /// `__valid_k x`.
    fn valid_at(&mut self, index: u64, x: Term) -> Term {
        let head = self.valid_const(index);
        self.b.app(head, x)
    }

    /// `__valid_k x = true ↔ __inv_k x`.
    fn viff_claim(&mut self, index: u64, x: Term) -> Term {
        let valid = self.valid_at(index, x);
        let inv = self.inv_const(index);
        let inv = self.b.app(inv, x);
        let decided = self.is_true(valid);
        self.iff(decided, inv)
    }

    /// `FunRel __prog k [enc x] (Rel true (Value.bool (__valid_k x)))`.
    fn vrel_claim(&mut self, validator: &Validator, x: Term) -> Term {
        let valid = self.valid_at(validator.index, x);
        let ctor = self.ts("Value.bool");
        let w = self.b.app(ctor, valid);
        let tru = self.tru();
        let rel = self.rel(tru, w);
        let value_ty = self.value_ty();
        let encoded = self.b.app(validator.enc, x);
        let list = self.b.list(value_ty, &[encoded]);
        let fun_rel = self.lp("FunRel");
        let index = self.nat_u(validator.index);
        let prog = self.prog;
        self.b.apps(fun_rel, &[prog, index, list, rel])
    }

    /// The statement a family makes of a validated type at `x`.
    fn claim(&mut self, family: Family, position: &GroupPosition, x: Term) -> Term {
        match family {
            Family::Valid => self.bool_ty(),
            Family::Inv => self.b.prop(),
            Family::Viff => self.viff_claim(position.validator.index, x),
            Family::Vrel => self.vrel_claim(&position.validator, x),
        }
    }

    /// The Lean type of a validated type's values and the row `__valid_k`'s
    /// type.
    fn row_type(&mut self, family: Family, validator: &Validator) -> Term {
        match family {
            Family::Valid => {
                let bool = self.bool_ty();
                self.b.arrow(validator.lean, bool)
            }
            Family::Inv => {
                let prop = self.b.prop();
                self.b.arrow(validator.lean, prop)
            }
            Family::Viff | Family::Vrel => {
                let x = self.b.free();
                let claim = if family == Family::Viff {
                    self.viff_claim(validator.index, x)
                } else {
                    self.vrel_claim(validator, x)
                };
                self.b.pi_over("__v", EXPLICIT, validator.lean, x, claim)
            }
        }
    }

    /// One row of a validator's four.
    fn push_family(
        &mut self,
        family: Family,
        validator: &Validator,
        value: Term,
    ) -> Result<(), String> {
        let (name, kind) = match family {
            Family::Valid => (
                format!("__valid_{}", validator.index),
                RowKind::Definition { compiled: true },
            ),
            Family::Inv => (
                format!("__inv_{}", validator.index),
                RowKind::Definition { compiled: false },
            ),
            Family::Viff => (format!("__viff_{}", validator.index), RowKind::Theorem),
            Family::Vrel => (format!("__vrel_{}", validator.index), RowKind::Theorem),
        };
        let ty = self.row_type(family, validator);
        self.push_row(&name, kind, ty, value)
    }

    // --- Validators outside any recursion -----------------------------------

    /// The four rows of a validator whose type is not in a recursive group.
    fn plain_validator(&mut self, boundary: &Boundary, index: u64) -> Result<(), String> {
        let validator = boundary.get(index)?.clone();
        if let Validation::Document { fields: _ } = &validator.kind {
            return self.plain_document(boundary, &validator);
        }
        let [valid, inv, viff, vrel] = self.library_validator(&validator)?;
        self.push_family(Family::Valid, &validator, valid)?;
        self.push_family(Family::Inv, &validator, inv)?;
        self.push_family(Family::Viff, &validator, viff)?;
        self.push_family(Family::Vrel, &validator, vrel)
    }

    /// The type of the validator's parameter in the program.
    fn validator_parameter(&self, index: u64) -> Result<&'a Ty, String> {
        let program = self.gen.program;
        program
            .functions
            .get(index as usize)
            .and_then(|function| function.types.first())
            .ok_or_else(|| format!("the validator {index} has no parameter"))
    }

    /// The hints every template lemma of validator `index` takes alike.
    fn template_hints(&mut self, index: u64) -> Vec<(&'static str, Term)> {
        let fi = self.nat_u(index);
        vec![("p", self.prog), ("fi", fi)]
    }

    /// `(valid, inv, viff, vrel)` of a container or trivial validator, by the
    /// library's definitions, propositions, decision theorems, and templates.
    #[allow(clippy::too_many_lines)]
    fn library_validator(&mut self, validator: &Validator) -> Result<[Term; 4], String> {
        let site = Site {
            module: validator.module.clone(),
            substitution: BTreeMap::new(),
        };
        let parameter = self.validator_parameter(validator.index)?;
        let x = self.b.free();
        let mut hints = self.template_hints(validator.index);
        let applied = match &validator.kind {
            Validation::Trivial => {
                let tru = self.tru();
                let valid = self.b.lam("_", EXPLICIT, validator.lean, tru);
                let truth = self.true_prop();
                let inv = self.b.lam("_", EXPLICIT, validator.lean, truth);
                let head = self.lp("validTrue_inv");
                let viff = self.b.app(head, validator.lean);
                hints.push(("α", validator.lean));
                let t = self.ty_term(parameter);
                hints.push(("t", t));
                let applied = self.apply(
                    "tpl_validTrue",
                    &hints,
                    &[Given::Term(validator.enc), Given::Refl, Given::Term(x)],
                )?;
                (valid, inv, viff, applied)
            }
            Validation::Set => {
                let key = element_of(&validator.ty)?;
                let kappa = self.lean_ty(&key)?;
                let ek = self.enc(&key)?;
                let cmp = self.key_order(&site, &key)?;
                let spec = self.key_spec(&site, &key)?;
                let bundle = self.list_bundle(&validator.ty)?;
                let valid = {
                    let head = self.lp("ascSet");
                    self.b.apps(head, &[kappa, cmp])
                };
                let inv = {
                    let head = self.lp("InvSet");
                    self.b.apps(head, &[kappa, cmp])
                };
                let viff = {
                    let head = self.lp("ascSet_inv");
                    self.b.apps(head, &[kappa, cmp])
                };
                let Ty::List { element } = parameter else {
                    return Err("a set validator over something other than a list".to_owned());
                };
                let t = self.ty_term(element);
                hints.extend([("κ", kappa), ("ek", ek), ("cmp", cmp), ("t", t)]);
                let applied = self.apply(
                    "tpl_validSet",
                    &hints,
                    &[
                        Given::Term(spec.proof),
                        Given::Term(bundle),
                        Given::Refl,
                        Given::Refl,
                        Given::Term(x),
                    ],
                )?;
                (valid, inv, viff, applied)
            }
            Validation::Map { value } => {
                let (key, stored) = parts(&validator.ty)?;
                let kappa = self.lean_ty(&key)?;
                let nu = self.lean_ty(&stored)?;
                let ek = self.enc(&key)?;
                let ev = self.enc(&stored)?;
                let cmp = self.key_order(&site, &key)?;
                let spec = self.key_spec(&site, &key)?;
                let bundle = self.list_bundle(&validator.ty)?;
                let valid_v = self.valid_const(*value);
                let inv_v = self.inv_const(*value);
                let viff_v = self.viff_const(*value);
                let vrel_v = self.vrel_const(*value);
                let valid = {
                    let head = self.lp("ascMap");
                    self.b.apps(head, &[kappa, nu, cmp, valid_v])
                };
                let inv = {
                    let head = self.lp("InvMap");
                    self.b.apps(head, &[kappa, nu, cmp, inv_v])
                };
                let viff = {
                    let head = self.lp("ascMap_inv");
                    self.b.apps(head, &[kappa, nu, cmp, valid_v, inv_v, viff_v])
                };
                let Ty::List { element } = parameter else {
                    return Err("a map validator over something other than a list".to_owned());
                };
                let Ty::Pair { left, right } = element.as_ref() else {
                    return Err("a map validator over entries that are not pairs".to_owned());
                };
                let (tk, tw) = (self.ty_term(left), self.ty_term(right));
                let fv = self.nat_u(*value);
                hints.extend([
                    ("κ", kappa),
                    ("ν", nu),
                    ("ek", ek),
                    ("ev", ev),
                    ("cmp", cmp),
                    ("fv", fv),
                    ("tk", tk),
                    ("tw", tw),
                ]);
                let applied = self.apply(
                    "tpl_validMap",
                    &hints,
                    &[
                        Given::Term(spec.proof),
                        Given::Term(valid_v),
                        Given::Term(bundle),
                        Given::Refl,
                        Given::Term(vrel_v),
                        Given::Refl,
                        Given::Term(x),
                    ],
                )?;
                (valid, inv, viff, applied)
            }
            Validation::List { element } => {
                let inner = element_of(&validator.ty)?;
                let alpha = self.lean_ty(&inner)?;
                let ea = self.enc(&inner)?;
                let bundle = self.list_bundle(&validator.ty)?;
                let valid_e = self.valid_const(*element);
                let inv_e = self.inv_const(*element);
                let viff_e = self.viff_const(*element);
                let vrel_e = self.vrel_const(*element);
                let valid = {
                    let head = self.lp("validList");
                    self.b.apps(head, &[alpha, valid_e])
                };
                let inv = {
                    let head = self.lp("InvList");
                    self.b.apps(head, &[alpha, inv_e])
                };
                let viff = {
                    let head = self.lp("validList_inv");
                    self.b.apps(head, &[alpha, valid_e, inv_e, viff_e])
                };
                let Ty::List { element: stored } = parameter else {
                    return Err("a list validator over something other than a list".to_owned());
                };
                let t = self.ty_term(stored);
                let fv = self.nat_u(*element);
                hints.extend([("α", alpha), ("ea", ea), ("fv", fv), ("t", t)]);
                let applied = self.apply(
                    "tpl_validList",
                    &hints,
                    &[
                        Given::Term(valid_e),
                        Given::Term(bundle),
                        Given::Refl,
                        Given::Term(vrel_e),
                        Given::Refl,
                        Given::Term(x),
                    ],
                )?;
                (valid, inv, viff, applied)
            }
            Validation::Option { value } => {
                let inner = option_value(&validator.ty)?;
                let alpha = self.lean_ty(&inner)?;
                let ea = self.enc(&inner)?;
                let valid_v = self.valid_const(*value);
                let inv_v = self.inv_const(*value);
                let viff_v = self.viff_const(*value);
                let vrel_v = self.vrel_const(*value);
                let valid = {
                    let head = self.lp("validOption");
                    self.b.apps(head, &[alpha, valid_v])
                };
                let inv = {
                    let head = self.lp("InvOption");
                    self.b.apps(head, &[alpha, inv_v])
                };
                let viff = {
                    let head = self.lp("validOption_inv");
                    self.b.apps(head, &[alpha, valid_v, inv_v, viff_v])
                };
                let Ty::Option { value: stored } = parameter else {
                    return Err(
                        "an option validator over something other than an option".to_owned()
                    );
                };
                let t = self.ty_term(stored);
                let fv = self.nat_u(*value);
                hints.extend([("α", alpha), ("ea", ea), ("fv", fv), ("t", t)]);
                let applied = self.apply(
                    "tpl_validOption",
                    &hints,
                    &[
                        Given::Term(valid_v),
                        Given::Term(vrel_v),
                        Given::Refl,
                        Given::Term(x),
                    ],
                )?;
                (valid, inv, viff, applied)
            }
            Validation::Pair { left, right } => {
                let (a, b) = parts(&validator.ty)?;
                let alpha = self.lean_ty(&a)?;
                let beta = self.lean_ty(&b)?;
                let ea = self.enc(&a)?;
                let eb = self.enc(&b)?;
                let (valid_a, valid_b) = (self.valid_const(*left), self.valid_const(*right));
                let (inv_a, inv_b) = (self.inv_const(*left), self.inv_const(*right));
                let (viff_a, viff_b) = (self.viff_const(*left), self.viff_const(*right));
                let (vrel_a, vrel_b) = (self.vrel_const(*left), self.vrel_const(*right));
                let valid = {
                    let head = self.lp("validPair");
                    self.b.apps(head, &[alpha, beta, valid_a, valid_b])
                };
                let inv = {
                    let head = self.lp("InvPair");
                    self.b.apps(head, &[alpha, beta, inv_a, inv_b])
                };
                let viff = {
                    let head = self.lp("validPair_inv");
                    self.b.apps(
                        head,
                        &[alpha, beta, valid_a, valid_b, inv_a, inv_b, viff_a, viff_b],
                    )
                };
                let Ty::Pair {
                    left: first,
                    right: second,
                } = parameter
                else {
                    return Err("a pair validator over something other than a pair".to_owned());
                };
                let (ta, tb) = (self.ty_term(first), self.ty_term(second));
                let (fa, fb) = (self.nat_u(*left), self.nat_u(*right));
                hints.extend([
                    ("α", alpha),
                    ("β", beta),
                    ("ea", ea),
                    ("eb", eb),
                    ("fa", fa),
                    ("fb", fb),
                    ("ta", ta),
                    ("tb", tb),
                ]);
                let applied = self.apply(
                    "tpl_validPair",
                    &hints,
                    &[
                        Given::Term(valid_a),
                        Given::Term(valid_b),
                        Given::Term(vrel_a),
                        Given::Term(vrel_b),
                        Given::Refl,
                        Given::Term(x),
                    ],
                )?;
                (valid, inv, viff, applied)
            }
            Validation::Result { ok, error } => {
                let (ok_ty, error_ty) = parts(&validator.ty)?;
                let alpha = self.lean_ty(&ok_ty)?;
                let epsilon = self.lean_ty(&error_ty)?;
                let ea = self.enc(&ok_ty)?;
                let ee = self.enc(&error_ty)?;
                let (valid_a, valid_e) = (self.valid_const(*ok), self.valid_const(*error));
                let (inv_a, inv_e) = (self.inv_const(*ok), self.inv_const(*error));
                let (viff_a, viff_e) = (self.viff_const(*ok), self.viff_const(*error));
                let (vrel_a, vrel_e) = (self.vrel_const(*ok), self.vrel_const(*error));
                let valid = {
                    let head = self.lp("validExcept");
                    self.b.apps(head, &[epsilon, alpha, valid_e, valid_a])
                };
                let inv = {
                    let head = self.lp("InvExcept");
                    self.b.apps(head, &[epsilon, alpha, inv_e, inv_a])
                };
                let viff = {
                    let head = self.lp("validExcept_inv");
                    self.b.apps(
                        head,
                        &[
                            epsilon, alpha, valid_e, valid_a, inv_e, inv_a, viff_e, viff_a,
                        ],
                    )
                };
                let Ty::Result {
                    ok: stored_ok,
                    error: stored_error,
                } = parameter
                else {
                    return Err("a result validator over something other than a result".to_owned());
                };
                let (ta, te) = (self.ty_term(stored_ok), self.ty_term(stored_error));
                let (fa, fe) = (self.nat_u(*ok), self.nat_u(*error));
                hints.extend([
                    ("α", alpha),
                    ("ε", epsilon),
                    ("ea", ea),
                    ("ee", ee),
                    ("fa", fa),
                    ("fe", fe),
                    ("ta", ta),
                    ("te", te),
                ]);
                let applied = self.apply(
                    "tpl_validResult",
                    &hints,
                    &[
                        Given::Term(valid_e),
                        Given::Term(valid_a),
                        Given::Term(vrel_a),
                        Given::Term(vrel_e),
                        Given::Refl,
                        Given::Term(x),
                    ],
                )?;
                (valid, inv, viff, applied)
            }
            Validation::Document { fields: _ } => {
                return Err("a document validator is not a library validator".to_owned());
            }
        };
        let (valid, inv, viff, relation) = applied;
        let vrel = self
            .b
            .lam_over("__v", EXPLICIT, validator.lean, x, relation.proof);
        Ok([valid, inv, viff, vrel])
    }

    // --- Document types -----------------------------------------------------

    /// The constructors of a document type outside any recursion.
    fn plain_constructors(&self, ty: &SemanticType) -> Result<Vec<RecCtor>, String> {
        let shape = self.gen.source.document(ty)?;
        Ok(shape
            .fields
            .iter()
            .map(|fields| RecCtor {
                fields: fields.clone(),
                recursive: vec![None; fields.len()],
            })
            .collect())
    }

    /// The constructors of a validated type as its validator reads them.
    fn constructor_specs(
        &self,
        validator: &Validator,
        constructors: &[RecCtor],
    ) -> Result<Vec<CtorSpec>, String> {
        let spec = |constructor: &RecCtor,
                    checks: Vec<(usize, u64)>,
                    direct: bool,
                    arm: Option<usize>|
         -> Result<CtorSpec, String> {
            for (field, _) in &checks {
                if *field >= constructor.fields.len() {
                    return Err(format!(
                        "the validator {} checks a field its constructor lacks",
                        validator.index
                    ));
                }
            }
            Ok(CtorSpec {
                fields: constructor.fields.clone(),
                recursive: constructor.recursive.clone(),
                checks: checks
                    .into_iter()
                    .map(|(field, validator)| Check { field, validator })
                    .collect(),
                direct,
                arm,
            })
        };
        let wanted = |count: usize| -> Result<(), String> {
            if constructors.len() == count {
                Ok(())
            } else {
                Err(format!(
                    "the validator {} reads {} constructors, the type has {}",
                    validator.index,
                    count,
                    constructors.len()
                ))
            }
        };
        match &validator.kind {
            Validation::Document { fields } => {
                wanted(fields.len())?;
                constructors
                    .iter()
                    .zip(fields)
                    .enumerate()
                    .map(|(position, (constructor, checks))| {
                        if checks.len() != constructor.fields.len() {
                            return Err(format!(
                                "the validator {} checks another number of fields",
                                validator.index
                            ));
                        }
                        let checks = checks
                            .iter()
                            .enumerate()
                            .filter_map(|(field, check)| check.map(|check| (field, check)))
                            .collect();
                        spec(constructor, checks, false, Some(position))
                    })
                    .collect()
            }
            Validation::List { element } => {
                wanted(2)?;
                Ok(vec![
                    spec(&constructors[0], Vec::new(), true, Some(0))?,
                    spec(
                        &constructors[1],
                        vec![(0, *element), (1, validator.index)],
                        true,
                        Some(1),
                    )?,
                ])
            }
            Validation::Option { value } => {
                wanted(2)?;
                Ok(vec![
                    spec(&constructors[0], Vec::new(), true, Some(0))?,
                    spec(&constructors[1], vec![(0, *value)], true, Some(1))?,
                ])
            }
            Validation::Pair { left, right } => {
                wanted(1)?;
                Ok(vec![spec(
                    &constructors[0],
                    vec![(0, *left), (1, *right)],
                    true,
                    None,
                )?])
            }
            // The program matches the value arm first, the error arm second;
            // the type's constructors are the other way round.
            Validation::Result { ok, error } => {
                wanted(2)?;
                Ok(vec![
                    spec(&constructors[0], vec![(0, *error)], true, Some(1))?,
                    spec(&constructors[1], vec![(0, *ok)], true, Some(0))?,
                ])
            }
            Validation::Trivial | Validation::Set | Validation::Map { value: _ } => Err(format!(
                "the validator of `{}` cannot belong to a recursive group",
                self.gen.source.type_text(&validator.ty)
            )),
        }
    }

    /// A document type's validator outside any recursion: by its eliminator.
    fn plain_document(&mut self, boundary: &Boundary, validator: &Validator) -> Result<(), String> {
        let constructors = self.plain_constructors(&validator.ty)?;
        let ctors = self.constructor_specs(validator, &constructors)?;
        let group = Group {
            positions: vec![GroupPosition {
                validator: validator.clone(),
                ctors,
            }],
            plan: None,
        };
        for family in [Family::Valid, Family::Inv, Family::Viff, Family::Vrel] {
            let values = self.family_values(boundary, family, &group)?;
            self.push_family(family, validator, values[0])?;
        }
        Ok(())
    }

    // --- Recursive groups ---------------------------------------------------

    /// The validators of a recursive group of document types and the
    /// containers nested in them, which are mutually structural.
    #[allow(clippy::too_many_lines)]
    fn recursive_validators(&mut self, boundary: &Boundary, members: &[u64]) -> Result<(), String> {
        let document = members
            .iter()
            .find_map(|index| {
                let validator = boundary.validators.get(index)?;
                matches!(validator.kind, Validation::Document { fields: _ })
                    .then(|| validator.ty.clone())
            })
            .ok_or("a recursive group of validators with no document type")?;
        let plan = self.rec_plan(&document)?;
        let mut positions = Vec::new();
        for entry in &plan.positions {
            let wanted = match &entry.kind {
                PositionKind::Aux(aux) => self.gen.source.type_text(&aux.original),
                PositionKind::Block { index: _ } | PositionKind::Builtin => {
                    self.gen.source.type_text(&entry.ty)
                }
            };
            let validator = members
                .iter()
                .filter_map(|index| boundary.validators.get(index))
                .find(|validator| self.gen.source.type_text(&validator.ty) == wanted)
                .ok_or_else(|| {
                    format!("`{wanted}` is in a recursive group but has no validator")
                })?;
            positions.push(GroupPosition {
                validator: validator.clone(),
                ctors: self.constructor_specs(validator, &entry.constructors)?,
            });
        }
        for member in members {
            if !positions
                .iter()
                .any(|position| position.validator.index == *member)
            {
                return Err(format!(
                    "the validator {member} of a recursive group is at no position of its recursors"
                ));
            }
        }
        // A recursive field is checked by the validator of the position it
        // recurses on.
        for position in &positions {
            for ctor in &position.ctors {
                for check in &ctor.checks {
                    if let Some(target) = ctor.recursive.get(check.field).copied().flatten() {
                        let expected = positions
                            .get(target)
                            .map(|position| position.validator.index);
                        if expected != Some(check.validator) {
                            return Err(format!(
                                "the validator {} checks a recursive field with the validator {}, not the one of its position",
                                position.validator.index, check.validator
                            ));
                        }
                    }
                }
            }
        }
        let group = Group {
            positions,
            plan: Some(plan),
        };
        let position_of = |index: u64| {
            group
                .positions
                .iter()
                .position(|position| position.validator.index == index)
        };
        let mut order: Vec<(u64, usize)> = Vec::new();
        for member in members {
            order.push((
                *member,
                position_of(*member).ok_or("a member outside the group")?,
            ));
        }
        let valid = self.family_values(boundary, Family::Valid, &group)?;
        let mut names = Vec::new();
        for (member, position) in &order {
            let validator = boundary.get(*member)?.clone();
            self.push_family(Family::Valid, &validator, valid[*position])?;
            names.push(format!("__valid_{member}"));
        }
        self.group_rows(&names);
        self.unsafe_records(boundary, &group, &order)?;
        for family in [Family::Inv, Family::Viff, Family::Vrel] {
            let values = self.family_values(boundary, family, &group)?;
            for (member, position) in &order {
                let validator = boundary.get(*member)?.clone();
                self.push_family(family, &validator, values[*position])?;
            }
        }
        for (member, position) in &order {
            let validator = boundary.get(*member)?.clone();
            self.inv_corollary(&group, &validator, *position)?;
        }
        Ok(())
    }

    /// The validators outside a group that its members call, which bring
    /// axioms: the group's records name them all in each record (see
    /// [`Self::unsafe_records`]).
    fn external_axiom_callees(&mut self, group: &Group, order: &[(u64, usize)]) -> Vec<u64> {
        let mut callees = BTreeSet::new();
        for (_, position) in order {
            for ctor in &group.positions[*position].ctors {
                for check in &ctor.checks {
                    if ctor.recursive[check.field].is_none() {
                        callees.insert(check.validator);
                    }
                }
            }
        }
        callees
            .into_iter()
            .filter(|callee| {
                let name = format!("{}.__valid_{callee}", self.name);
                self.constant_axioms(&name)
                    .is_ok_and(|axioms| !axioms.is_empty())
            })
            .collect()
    }

    /// The implementation records of a group's `__valid_k`: direct recursion
    /// by the eliminator of each type, calling the records of the others.
    ///
    /// The records call one another, and Lean computes the axioms of a
    /// definition in a cycle with a cache that a call still in progress cuts
    /// short: which record sees the axioms of a callee that only another
    /// record names depends on the order the declarations are visited in. So
    /// that every record's axioms are the same whatever that order is, each
    /// names (in a `let` the code generator drops) every callee outside the
    /// group that brings axioms, so each record sees them directly.
    fn unsafe_records(
        &mut self,
        boundary: &Boundary,
        group: &Group,
        order: &[(u64, usize)],
    ) -> Result<(), String> {
        let bool = self.bool_ty();
        let external = self.external_axiom_callees(group, order);
        let mut ballast: Option<(Term, Term)> = None;
        for callee in external.iter().rev() {
            let validator = boundary.get(*callee)?;
            let ty = self.b.arrow(validator.lean, bool);
            let constant = self.valid_const(*callee);
            ballast = Some(match ballast {
                None => (ty, constant),
                Some((rest_ty, rest)) => (
                    self.b.pair_ty(ty, rest_ty),
                    self.b.pair(ty, rest_ty, constant, rest),
                ),
            });
        }
        let mut names = Vec::new();
        for (member, position) in order {
            let validator = boundary.get(*member)?.clone();
            let free = self.b.free();
            let one = CoreLevel::Succ(Box::new(CoreLevel::Zero));
            let mut minors = Vec::new();
            for ctor in 0..group.positions[*position].ctors.len() {
                minors.push(self.minor(
                    boundary,
                    Family::Valid,
                    group,
                    *position,
                    ctor,
                    Mode::Unsafe,
                )?);
            }
            let motive = {
                let lifted = self.b.lift(bool, 1, 0);
                self.b.lam("_", EXPLICIT, validator.lean, lifted)
            };
            let mut cases = self.cases_on(&validator.ty, free, motive, one, &minors)?;
            if let Some((ballast_ty, ballast_value)) = ballast {
                cases = self
                    .b
                    .let_node("_callees", ballast_ty, ballast_value, cases, false);
            }
            let value = self
                .b
                .lam_over("__v", EXPLICIT, validator.lean, free, cases);
            let ty = self.row_type(Family::Valid, &validator);
            let name = format!("__valid_{member}._unsafe_rec");
            self.push_row(&name, RowKind::Partial, ty, value)?;
            names.push(name);
        }
        self.group_rows(&names);
        Ok(())
    }

    /// `__vinv_k`: the group's proposition of a nested container is the
    /// library's proposition of the container over the group's.
    fn inv_corollary(
        &mut self,
        group: &Group,
        validator: &Validator,
        position: usize,
    ) -> Result<(), String> {
        let x = self.b.free();
        let inv_k = self.inv_const(validator.index);
        let inv_at = self.b.app(inv_k, x);
        let (library, proof) = match &validator.kind {
            Validation::List { element } => {
                let inner = element_of(&validator.ty)?;
                let alpha = self.lean_ty(&inner)?;
                let inv_e = self.inv_const(*element);
                let head = self.lp("InvList");
                let library = self.b.apps(head, &[alpha, inv_e, x]);
                let nil = self.ctor_value(&group.positions[position], 0, &[])?;
                let nil_inv = self.b.app(inv_k, nil);
                let base = {
                    let head = self.b.konst("Iff.refl");
                    self.b.app(head, nil_inv)
                };
                let (head_x, tail_x) = (self.b.free(), self.b.free());
                let cons = self.ctor_value(&group.positions[position], 1, &[head_x, tail_x])?;
                let cons_inv = self.b.app(inv_k, cons);
                let step_body = {
                    let head = self.b.konst("Iff.refl");
                    self.b.app(head, cons_inv)
                };
                let element_ty = self.lean_ty(&inner)?;
                let list_ty = validator.lean;
                let step = self
                    .b
                    .lam_over("__x1", EXPLICIT, list_ty, tail_x, step_body);
                let step = self.b.lam_over("__x0", EXPLICIT, element_ty, head_x, step);
                let lemma = self.lp("invList_eqs");
                let proof = self.b.apps(lemma, &[alpha, inv_e, inv_k, base, step, x]);
                (library, proof)
            }
            Validation::Option { value } => {
                let inner = option_value(&validator.ty)?;
                let alpha = self.lean_ty(&inner)?;
                let inv_v = self.inv_const(*value);
                let head = self.lp("InvOption");
                let library = self.b.apps(head, &[alpha, inv_v, x]);
                let claim = self.iff(inv_at, library);
                let motive = self.b.lam_over("__v", EXPLICIT, validator.lean, x, claim);
                let proof = self.iff_reflexive_cases(group, validator, position, motive, x)?;
                (library, proof)
            }
            Validation::Pair { left, right } => {
                let (a, b) = parts(&validator.ty)?;
                let alpha = self.lean_ty(&a)?;
                let beta = self.lean_ty(&b)?;
                let (inv_a, inv_b) = (self.inv_const(*left), self.inv_const(*right));
                let head = self.lp("InvPair");
                let library = self.b.apps(head, &[alpha, beta, inv_a, inv_b, x]);
                let claim = self.iff(inv_at, library);
                let motive = self.b.lam_over("__v", EXPLICIT, validator.lean, x, claim);
                let proof = self.iff_reflexive_cases(group, validator, position, motive, x)?;
                (library, proof)
            }
            Validation::Result { ok, error } => {
                let (ok_ty, error_ty) = parts(&validator.ty)?;
                let alpha = self.lean_ty(&ok_ty)?;
                let epsilon = self.lean_ty(&error_ty)?;
                let (inv_a, inv_e) = (self.inv_const(*ok), self.inv_const(*error));
                let head = self.lp("InvExcept");
                let library = self.b.apps(head, &[epsilon, alpha, inv_e, inv_a, x]);
                let claim = self.iff(inv_at, library);
                let motive = self.b.lam_over("__v", EXPLICIT, validator.lean, x, claim);
                let proof = self.iff_reflexive_cases(group, validator, position, motive, x)?;
                (library, proof)
            }
            Validation::Trivial
            | Validation::Set
            | Validation::Map { value: _ }
            | Validation::Document { fields: _ } => return Ok(()),
        };
        let claim = self.iff(inv_at, library);
        let ty = self.b.pi_over("__v", EXPLICIT, validator.lean, x, claim);
        let value = self.b.lam_over("__v", EXPLICIT, validator.lean, x, proof);
        self.push_row(
            &format!("__vinv_{}", validator.index),
            RowKind::Theorem,
            ty,
            value,
        )
    }

    /// `casesOn x` proving `motive x` by `Iff.refl` at each constructor: the
    /// two propositions are the same after unfolding.
    fn iff_reflexive_cases(
        &mut self,
        group: &Group,
        validator: &Validator,
        position: usize,
        motive: Term,
        x: Term,
    ) -> Result<Term, String> {
        let inv_k = self.inv_const(validator.index);
        let mut minors = Vec::new();
        let constructors = group.positions[position].ctors.clone();
        for (ctor, spec) in constructors.iter().enumerate() {
            let mut frees = Vec::new();
            let mut types = Vec::new();
            for field in &spec.fields {
                frees.push(self.b.free());
                types.push(self.lean_ty(field)?);
            }
            let value = self.ctor_value(&group.positions[position], ctor, &frees)?;
            let at = self.b.app(inv_k, value);
            let head = self.b.konst("Iff.refl");
            let mut minor = self.b.app(head, at);
            for (index, free) in frees.iter().enumerate().rev() {
                minor =
                    self.b
                        .lam_over(&format!("__x{index}"), EXPLICIT, types[index], *free, minor);
            }
            minors.push(minor);
        }
        self.cases_on(&validator.ty, x, motive, CoreLevel::Zero, &minors)
    }

    /// The constructor of a validated type applied to its fields.
    fn ctor_value(
        &mut self,
        position: &GroupPosition,
        ctor: usize,
        fields: &[Term],
    ) -> Result<Term, String> {
        let ty = &position.validator.ty;
        match &position.validator.kind {
            Validation::Document { fields: _ } => {
                let SemanticType::Named { member, arguments } = ty else {
                    return Err(
                        "a document validator over a type that is not a document".to_owned()
                    );
                };
                let shape = self.gen.source.document(ty)?;
                let name = shape
                    .names
                    .get(ctor)
                    .ok_or_else(|| format!("the document has no constructor {ctor}"))?;
                let owner = self.document_name(member)?;
                let head = self.b.konst(&format!("{owner}.{name}"));
                let mut all = Vec::new();
                for argument in arguments {
                    all.push(self.lean_ty(argument)?);
                }
                all.extend_from_slice(fields);
                Ok(self.b.apps(head, &all))
            }
            Validation::List { element: _ } => {
                let alpha = self.lean_ty(&element_of(ty)?)?;
                if ctor == 0 {
                    let head = self.b.konst0("List.nil");
                    Ok(self.b.app(head, alpha))
                } else {
                    let head = self.b.konst0("List.cons");
                    let mut all = vec![alpha];
                    all.extend_from_slice(fields);
                    Ok(self.b.apps(head, &all))
                }
            }
            Validation::Option { value: _ } => {
                let alpha = self.lean_ty(&option_value(ty)?)?;
                let name = if ctor == 0 {
                    "Option.none"
                } else {
                    "Option.some"
                };
                let head = self.b.konst0(name);
                let mut all = vec![alpha];
                all.extend_from_slice(fields);
                Ok(self.b.apps(head, &all))
            }
            Validation::Pair { left: _, right: _ } => {
                let (a, b) = parts(ty)?;
                let (alpha, beta) = (self.lean_ty(&a)?, self.lean_ty(&b)?);
                match fields {
                    [first, second] => Ok(self.b.pair(alpha, beta, *first, *second)),
                    _ => Err("a pair of other than two components".to_owned()),
                }
            }
            Validation::Result { ok: _, error: _ } => {
                let (ok, error) = parts(ty)?;
                let (alpha, epsilon) = (self.lean_ty(&ok)?, self.lean_ty(&error)?);
                let name = if ctor == 0 {
                    "Except.error"
                } else {
                    "Except.ok"
                };
                let head = self.b.konst00(name);
                let mut all = vec![epsilon, alpha];
                all.extend_from_slice(fields);
                Ok(self.b.apps(head, &all))
            }
            Validation::Trivial | Validation::Set | Validation::Map { value: _ } => {
                Err("a validator without constructors".to_owned())
            }
        }
    }

    // --- The four families --------------------------------------------------

    /// For each position of the group, `fun v => …`: the statement of
    /// `family` by the group's recursor (or eliminator) at that position.
    fn family_values(
        &mut self,
        boundary: &Boundary,
        family: Family,
        group: &Group,
    ) -> Result<Vec<Term>, String> {
        let one = CoreLevel::Succ(Box::new(CoreLevel::Zero));
        let level = match family {
            Family::Valid | Family::Inv => one,
            Family::Viff | Family::Vrel => CoreLevel::Zero,
        };
        let mut motives = Vec::new();
        for position in &group.positions {
            let free = self.b.free();
            let claim = self.claim(family, position, free);
            motives.push(
                self.b
                    .lam_over("__v", EXPLICIT, position.validator.lean, free, claim),
            );
        }
        let mut minors: Vec<Vec<Term>> = Vec::new();
        for (index, position) in group.positions.iter().enumerate() {
            let mut row = Vec::new();
            for ctor in 0..position.ctors.len() {
                row.push(self.minor(boundary, family, group, index, ctor, Mode::Ih)?);
            }
            minors.push(row);
        }
        let mut values = Vec::new();
        match &group.plan {
            Some(plan) => {
                let mut params = Vec::new();
                for argument in &plan.params {
                    params.push(self.lean_ty(argument)?);
                }
                let all: Vec<Term> = minors.iter().flatten().copied().collect();
                for (index, position) in group.positions.iter().enumerate() {
                    let recursor = plan
                        .recursors
                        .get(index)
                        .ok_or("a position without a recursor")?;
                    let recursor = self.b.konst_at(recursor, plan.levels(level.clone()));
                    let free = self.b.free();
                    let mut args = params.clone();
                    args.extend(motives.iter().copied());
                    args.extend(all.iter().copied());
                    args.push(free);
                    let applied = self.b.apps(recursor, &args);
                    values.push(self.b.lam_over(
                        "__v",
                        EXPLICIT,
                        position.validator.lean,
                        free,
                        applied,
                    ));
                }
            }
            None => {
                for (index, position) in group.positions.iter().enumerate() {
                    let free = self.b.free();
                    let cases = self.cases_on(
                        &position.validator.ty,
                        free,
                        motives[index],
                        level.clone(),
                        &minors[index],
                    )?;
                    values.push(self.b.lam_over(
                        "__v",
                        EXPLICIT,
                        position.validator.lean,
                        free,
                        cases,
                    ));
                }
            }
        }
        Ok(values)
    }

    /// The minor premise of a family for one constructor of one position:
    /// the fields, the induction hypotheses of the recursive ones, then the
    /// statement at the constructor.
    #[allow(clippy::too_many_arguments)]
    fn minor(
        &mut self,
        boundary: &Boundary,
        family: Family,
        group: &Group,
        position: usize,
        ctor: usize,
        mode: Mode,
    ) -> Result<Term, String> {
        let spec = group.positions[position].ctors[ctor].clone();
        let mut xs = Vec::new();
        let mut types = Vec::new();
        for field in &spec.fields {
            xs.push(self.b.free());
            types.push(self.lean_ty(field)?);
        }
        let mut ihs: Vec<Option<Term>> = Vec::new();
        let mut ih_types: Vec<Option<Term>> = Vec::new();
        for (index, recursion) in spec.recursive.iter().enumerate() {
            match (recursion, mode) {
                (Some(target), Mode::Ih) => {
                    ihs.push(Some(self.b.free()));
                    let claim = self.claim(family, &group.positions[*target], xs[index]);
                    ih_types.push(Some(claim));
                }
                (Some(_), Mode::Unsafe) | (None, _) => {
                    ihs.push(None);
                    ih_types.push(None);
                }
            }
        }
        let body = match family {
            Family::Valid | Family::Inv => {
                let mut calls = Vec::new();
                for check in &spec.checks {
                    let x = xs[check.field];
                    let call = match (spec.recursive[check.field], mode) {
                        (Some(_), Mode::Ih) => {
                            ihs[check.field].ok_or("a recursive field without a hypothesis")?
                        }
                        (Some(target), Mode::Unsafe) => {
                            let validator = group.positions[target].validator.index;
                            let head = self.own(&format!("__valid_{validator}._unsafe_rec"));
                            self.b.app(head, x)
                        }
                        (None, _) => {
                            let head = if family == Family::Valid {
                                self.valid_const(check.validator)
                            } else {
                                self.inv_const(check.validator)
                            };
                            self.b.app(head, x)
                        }
                    };
                    calls.push(call);
                }
                if family == Family::Valid {
                    self.valid_chain(&calls, spec.direct)
                } else {
                    self.inv_chain(&calls, spec.direct)
                }
            }
            Family::Viff => self.viff_chain(&spec, &xs, &ihs)?.proof,
            Family::Vrel => self.vrel_body(boundary, group, position, ctor, &xs, &ihs)?,
        };
        let mut minor = body;
        for (index, hypothesis) in ihs.iter().enumerate().rev() {
            if let (Some(free), Some(ty)) = (hypothesis, ih_types[index]) {
                minor = self
                    .b
                    .lam_over(&format!("__ih{index}"), EXPLICIT, ty, *free, minor);
            }
        }
        for (index, free) in xs.iter().enumerate().rev() {
            minor = self
                .b
                .lam_over(&format!("__x{index}"), EXPLICIT, types[index], *free, minor);
        }
        Ok(minor)
    }

    /// The tests of the checks in order: `if c₁ then if c₂ then … true else
    /// false else false`, the last check standing alone when it is the
    /// validator's result.
    fn valid_chain(&mut self, calls: &[Term], direct: bool) -> Term {
        let bool = self.bool_ty();
        let fls = self.fls();
        let (mut out, rest) = match calls.split_last() {
            Some((last, rest)) if direct => (*last, rest),
            _ => (self.tru(), calls),
        };
        for call in rest.iter().rev() {
            out = self.ite(bool, *call, out, fls);
        }
        out
    }

    /// The conjunction of the checks' propositions, as [`Self::valid_chain`].
    fn inv_chain(&mut self, calls: &[Term], direct: bool) -> Term {
        let (mut out, rest) = match calls.split_last() {
            Some((last, rest)) if direct => (*last, rest),
            _ => (self.true_prop(), calls),
        };
        for call in rest.iter().rev() {
            out = self.and_prop(*call, out);
        }
        out
    }

    /// The decision theorem of a constructor: `condAnd_iff` over the checks'
    /// theorems.
    fn viff_chain(
        &mut self,
        spec: &CtorSpec,
        xs: &[Term],
        ihs: &[Option<Term>],
    ) -> Result<Pf, String> {
        let mut leaves = Vec::new();
        for check in &spec.checks {
            let x = xs[check.field];
            let proof = match (spec.recursive[check.field], ihs[check.field]) {
                (Some(_), Some(ih)) => ih,
                _ => {
                    let head = self.viff_const(check.validator);
                    self.b.app(head, x)
                }
            };
            let ty = self.viff_claim(check.validator, x);
            leaves.push(Pf { proof, ty });
        }
        let (mut out, rest) = match leaves.split_last() {
            Some((last, rest)) if spec.direct => (*last, rest),
            _ => (self.apply("trueIff", &[], &[])?, leaves.as_slice()),
        };
        for leaf in rest.iter().rev() {
            out = self.apply(
                "condAnd_iff",
                &[],
                &[
                    Given::Proof(leaf.proof, leaf.ty),
                    Given::Proof(out.proof, out.ty),
                ],
            )?;
        }
        Ok(out)
    }

    // --- The relation of a validator at a constructor -------------------------

    /// The proof that the program's validator, run on the encoding of a
    /// constructor applied to its fields, returns `__valid_k` of it.
    fn vrel_body(
        &mut self,
        boundary: &Boundary,
        group: &Group,
        position: usize,
        ctor: usize,
        xs: &[Term],
        ihs: &[Option<Term>],
    ) -> Result<Term, String> {
        let entry = &group.positions[position];
        let spec = &entry.ctors[ctor];
        let validator = &entry.validator;
        let program = self.gen.program;
        let function = program
            .functions
            .get(validator.index as usize)
            .ok_or_else(|| format!("the program has no validator {}", validator.index))?;
        let value = self.ctor_value(entry, ctor, xs)?;
        let encoded = self.b.app(validator.enc, value);
        let mut fields = Vec::new();
        for (ty, x) in spec.fields.iter().zip(xs) {
            let enc = self.enc(ty)?;
            fields.push(self.b.app(enc, *x));
        }
        let empty = self.empty_env();
        let base = self.extend_env(empty, &function.parameters, &[encoded])?;
        let mut checks = Vec::new();
        for check in &spec.checks {
            let callee = boundary.get(check.validator)?.clone();
            let x = xs[check.field];
            let proof = match (spec.recursive[check.field], ihs[check.field]) {
                (Some(_), Some(ih)) => ih,
                _ => {
                    let head = self.vrel_const(check.validator);
                    self.b.app(head, x)
                }
            };
            let valid = self.valid_at(check.validator, x);
            checks.push(Checked {
                validator: callee,
                x,
                proof,
                valid,
            });
        }
        let parameter = *function
            .parameters
            .first()
            .ok_or("a validator without a parameter")?;
        let expected = self.expected_validator(validator, function)?;
        let body = match spec.arm {
            Some(arm_index) => {
                let Expr::Match {
                    ty: match_ty,
                    scrutinee,
                    arms,
                } = &expected.body
                else {
                    return Err(format!(
                        "the validator {} was lowered to something other than a match",
                        validator.index
                    ));
                };
                let (scrutinee, arms) = (scrutinee.as_ref(), arms.as_slice());
                let arm = arms
                    .get(arm_index)
                    .ok_or("a validator lowered to fewer arms")?;
                let inner = self.extend_env(base, &arm.binders, &fields)?;
                let mut vars: Vec<(u64, Term)> = arm
                    .binders
                    .iter()
                    .copied()
                    .zip(fields.iter().copied())
                    .collect();
                vars.push((parameter, encoded));
                let binding = Binding { vars, pair: None };
                let derived = self.chain_pf(&binding, inner, &arm.body, &checks, spec.direct)?;
                let (_, pre_arms) = self.prepare_arms(arms)?;
                let mut arms_pf =
                    self.arm_hit(base, encoded, &pre_arms, arm_index, &fields, inner, derived)?;
                for earlier in (0..arm_index).rev() {
                    arms_pf = self.arm_miss(base, encoded, &pre_arms, earlier, arms_pf)?;
                }
                let Expr::Var { name } = scrutinee else {
                    return Err(
                        "a validator matches on something other than its argument".to_owned()
                    );
                };
                let name = *name;
                let tested = self.variable_pf(base, name, encoded)?;
                let t = self.ty_term(match_ty);
                let prog = self.prog;
                self.apply(
                    "conv_match",
                    &[("p", prog), ("env", base), ("t", t)],
                    &[
                        Given::Proof(tested.proof, tested.ty),
                        Given::Proof(arms_pf.proof, arms_pf.ty),
                    ],
                )?
            }
            None => {
                if fields.len() != 2 {
                    return Err("a validator without a match over other than a pair".to_owned());
                }
                let binding = Binding {
                    vars: vec![(parameter, encoded)],
                    pair: Some((fields[0], fields[1])),
                };
                self.chain_pf(&binding, base, &expected.body, &checks, spec.direct)?
            }
        };
        let function_term = self.function_term(function)?;
        let tru = self.tru();
        let valid = self.valid_at(validator.index, value);
        let ctor_bool = self.ts("Value.bool");
        let w = self.b.app(ctor_bool, valid);
        let observation = self.rel(tru, w);
        let value_ty = self.value_ty();
        let args = self.b.list(value_ty, &[encoded]);
        let fi = self.nat_u(validator.index);
        let prog = self.prog;
        let introduced = self.apply(
            "funRel_intro",
            &[
                ("p", prog),
                ("fi", fi),
                ("args", args),
                ("o", observation),
                ("fn", function_term),
                ("env", base),
            ],
            &[Given::Refl, Given::Refl, Given::Proof(body.proof, body.ty)],
        )?;
        self.forget_expression(&expected.body);
        Ok(introduced.proof)
    }

    /// The function the lowering builds for a container or document
    /// validator, from the layout's kind and the same builders the lowering
    /// uses. The program's own function is only read for the types of its
    /// parameter: the derivation follows what the source calls for, so a
    /// program that differs is refused by the kernel where the derived body
    /// meets the program's.
    fn expected_validator(
        &self,
        validator: &Validator,
        program_function: &Function,
    ) -> Result<Function, String> {
        use crate::calculus::library::validators;
        let index = validator.index;
        let parameter = program_function
            .types
            .first()
            .ok_or("a validator without a parameter type")?;
        let mismatch =
            || format!("the validator {index} has a parameter type its kind cannot have");
        Ok(match &validator.kind {
            Validation::List { element } => {
                let Ty::List { element: ty } = parameter else {
                    return Err(mismatch());
                };
                validators::list_of(index, *element, ty)
            }
            Validation::Option { value } => {
                let Ty::Option { value: ty } = parameter else {
                    return Err(mismatch());
                };
                validators::option_of(*value, ty)
            }
            Validation::Pair { left, right } => {
                let Ty::Pair { left: a, right: b } = parameter else {
                    return Err(mismatch());
                };
                validators::pair_of(*left, *right, a, b)
            }
            Validation::Result { ok, error } => {
                let Ty::Result { ok: a, error: b } = parameter else {
                    return Err(mismatch());
                };
                validators::result_of(*ok, *error, a, b)
            }
            Validation::Document { fields } => {
                let Ty::Adt { index: adt } = parameter else {
                    return Err(mismatch());
                };
                validators::document(*adt, fields)
            }
            Validation::Trivial | Validation::Set | Validation::Map { value: _ } => {
                return Err(format!(
                    "the validator {index} is not written by a derivation"
                ));
            }
        })
    }

    /// The body the lowering gives an entry: each checked parameter's
    /// validator called on it with the refusal otherwise, around the root's
    /// call, as `some`.
    fn expected_entry(
        &self,
        program_function: &Function,
        validators: &[Option<u64>],
    ) -> Result<Expr, String> {
        let Ty::Option { value: _ } = &program_function.result else {
            return Err("the entry does not return an option".to_owned());
        };
        let outcome = program_function.result.clone();
        let arguments = (0..validators.len() as u64)
            .map(|name| Expr::Var { name })
            .collect();
        let mut body = Expr::Build {
            shape: Shape::Some,
            ty: outcome.clone(),
            operands: vec![Expr::Call {
                function: 0,
                operands: arguments,
            }],
        };
        for (position, validator) in validators.iter().enumerate().rev() {
            if let Some(validator) = validator {
                body = Expr::Cond {
                    condition: Box::new(Expr::Call {
                        function: *validator,
                        operands: vec![Expr::Var {
                            name: position as u64,
                        }],
                    }),
                    then_branch: Box::new(body),
                    else_branch: Box::new(Expr::Build {
                        shape: Shape::None,
                        ty: outcome.clone(),
                        operands: Vec::new(),
                    }),
                };
            }
        }
        Ok(body)
    }

    /// Drop the renderings cached for the parts of an expression that is
    /// about to be freed: the cache is keyed by address, and a later
    /// expression built at a freed address must not meet them.
    fn forget_expression(&mut self, expr: &Expr) {
        self.exprs.remove(&std::ptr::from_ref(expr));
        match expr {
            Expr::Let { bound, body, .. } => {
                self.forget_expression(bound);
                self.forget_expression(body);
            }
            Expr::Cond {
                condition,
                then_branch,
                else_branch,
            } => {
                self.forget_expression(condition);
                self.forget_expression(then_branch);
                self.forget_expression(else_branch);
            }
            Expr::Match {
                scrutinee, arms, ..
            } => {
                self.forget_expression(scrutinee);
                for arm in arms {
                    self.forget_expression(&arm.body);
                }
            }
            Expr::Build { operands, .. }
            | Expr::Call { operands, .. }
            | Expr::Prim { operands, .. } => {
                for operand in operands {
                    self.forget_expression(operand);
                }
            }
            Expr::Closure { captures, .. } => {
                for capture in captures {
                    self.forget_expression(capture);
                }
            }
            Expr::Apply { target, operands } => {
                self.forget_expression(target);
                for operand in operands {
                    self.forget_expression(operand);
                }
            }
            Expr::First { value } | Expr::Second { value } | Expr::Field { value, .. } => {
                self.forget_expression(value);
            }
            Expr::Value { .. } | Expr::Var { .. } => {}
        }
    }

    /// `conv_var` for the variable `name`, bound to `value` in `env`.
    fn variable_pf(&mut self, env: Term, name: u64, value: Term) -> Result<Pf, String> {
        let x = self.nat_u(name);
        let prog = self.prog;
        self.apply(
            "conv_var",
            &[("p", prog), ("env", env), ("x", x), ("v", value)],
            &[Given::Refl],
        )
    }

    /// The proof of an argument of a validator's call: a variable, or a
    /// component of the pair it stands for.
    fn argument_pf(&mut self, binding: &Binding, env: Term, expr: &Expr) -> Result<Pf, String> {
        match expr {
            Expr::Var { name } => {
                let value = binding
                    .vars
                    .iter()
                    .rev()
                    .find(|(number, _)| number == name)
                    .map(|(_, value)| *value)
                    .ok_or_else(|| format!("variable {name} is unbound in a validator"))?;
                self.variable_pf(env, *name, value)
            }
            Expr::First { value } | Expr::Second { value } => {
                let (a, b) = binding
                    .pair
                    .ok_or("a projection in a validator that checks no pair")?;
                let inner = self.argument_pf(binding, env, value)?;
                let lemma = if matches!(expr, Expr::First { value: _ }) {
                    "conv_first"
                } else {
                    "conv_second"
                };
                let prog = self.prog;
                self.apply(
                    lemma,
                    &[("p", prog), ("env", env), ("a", a), ("b", b)],
                    &[Given::Proof(inner.proof, inner.ty)],
                )
            }
            Expr::Value { .. }
            | Expr::Let { .. }
            | Expr::Cond { .. }
            | Expr::Match { .. }
            | Expr::Build { .. }
            | Expr::Call { .. }
            | Expr::Closure { .. }
            | Expr::Apply { .. }
            | Expr::Prim { .. }
            | Expr::Field { .. } => {
                Err("a validator calls with an argument it does not build".to_owned())
            }
        }
    }

    /// `conv_call` of the checked validator on its argument.
    fn call_pf(
        &mut self,
        binding: &Binding,
        env: Term,
        expr: &Expr,
        check: &Checked,
    ) -> Result<Pf, String> {
        let Expr::Call { function, operands } = expr else {
            return Err(format!(
                "the validator was lowered without the call of {}",
                check.validator.index
            ));
        };
        if *function != check.validator.index || operands.len() != 1 {
            return Err(format!(
                "the validator was lowered with another call than {}",
                check.validator.index
            ));
        }
        let argument = self.argument_pf(binding, env, &operands[0])?;
        let list = self.operands(env, &[argument])?;
        let ty = self.vrel_claim(&check.validator, check.x);
        let prog = self.prog;
        self.apply(
            "conv_call",
            &[("p", prog), ("env", env)],
            &[
                Given::Proof(list.proof, list.ty),
                Given::Proof(check.proof, ty),
            ],
        )
    }

    /// A built Boolean.
    fn boolean_pf(&mut self, env: Term, expr: &Expr, value: bool) -> Result<Pf, String> {
        let Expr::Build {
            shape,
            ty,
            operands,
        } = expr
        else {
            return Err("a validator was lowered without the Boolean it returns".to_owned());
        };
        if !operands.is_empty() {
            return Err("a validator builds a Boolean from operands".to_owned());
        }
        let shape = *shape;
        let empty = self.operands(env, &[])?;
        let construct = self.apply(
            if value {
                "construct_true"
            } else {
                "construct_false"
            },
            &[],
            &[],
        )?;
        let shape = self.shape_term(shape);
        let ty = self.ty_term(ty);
        let prog = self.prog;
        self.apply(
            "conv_build",
            &[("p", prog), ("env", env), ("shape", shape), ("t", ty)],
            &[
                Given::Proof(empty.proof, empty.ty),
                Given::Proof(construct.proof, construct.ty),
            ],
        )
    }

    /// The derivation of a constructor's checks: a `cond` per check that is
    /// not the result, each testing the callee's relation.
    fn chain_pf(
        &mut self,
        binding: &Binding,
        env: Term,
        expr: &Expr,
        checks: &[Checked],
        direct: bool,
    ) -> Result<Pf, String> {
        match checks {
            [] => self.boolean_pf(env, expr, true),
            [only] if direct => self.call_pf(binding, env, expr, only),
            [first, rest @ ..] => {
                let Expr::Cond {
                    condition,
                    then_branch,
                    else_branch,
                } = expr
                else {
                    return Err("a validator was lowered without the test of a check".to_owned());
                };
                let (condition, then_branch, else_branch) = (
                    condition.as_ref(),
                    then_branch.as_ref(),
                    else_branch.as_ref(),
                );
                let tested = self.call_pf(binding, env, condition, first)?;
                let continued = self.chain_pf(binding, env, then_branch, rest, direct)?;
                let refused = self.boolean_pf(env, else_branch, false)?;
                let bool = self.bool_ty();
                let (tru, fls) = (self.tru(), self.fls());
                let predicate = self.b.lam("_", EXPLICIT, bool, tru);
                let remaining: Vec<Term> = rest.iter().map(|check| check.valid).collect();
                let rest_valid = self.valid_chain(&remaining, direct);
                let guard = self.b.free();
                let selected = self.ite(bool, guard, rest_valid, fls);
                let wrap = self.ts("Value.bool");
                let wrapped = self.b.app(wrap, selected);
                let value = self.b.lam_over("__c", EXPLICIT, bool, guard, wrapped);
                let on_true = {
                    let ty = self.eq(bool, first.valid, tru);
                    self.b.lam("_", EXPLICIT, ty, continued.proof)
                };
                let on_false = {
                    let ty = self.eq(bool, first.valid, fls);
                    self.b.lam("_", EXPLICIT, ty, refused.proof)
                };
                let then_term = self.expr_term(then_branch)?;
                let else_term = self.expr_term(else_branch)?;
                let prog = self.prog;
                self.apply(
                    "conv_cond",
                    &[
                        ("p", prog),
                        ("env", env),
                        ("t", then_term),
                        ("e", else_term),
                    ],
                    &[
                        Given::Term(predicate),
                        Given::Term(value),
                        Given::Term(first.valid),
                        Given::Proof(tested.proof, tested.ty),
                        Given::Term(on_true),
                        Given::Term(on_false),
                    ],
                )
            }
        }
    }

    // --- The entry ----------------------------------------------------------

    /// `Bool.casesOn x` proving `Eq Bool (shape x) x` by `rfl` at each
    /// value: `shape` is a Boolean expression over `x` that is `x` at
    /// `false` and `true`.
    fn bool_cases_eq(&mut self, x: Term, shape: Term) -> Result<Term, String> {
        let bool = self.bool_ty();
        let free = self.b.free();
        let abstracted = self.b.substitute(shape, &[x], &[free]);
        let claim = self.eq(bool, abstracted, free);
        let motive = self.b.lam_over("x", EXPLICIT, bool, free, claim);
        let (tru, fls) = (self.tru(), self.fls());
        let (on_false, on_true) = (self.refl(bool, fls), self.refl(bool, tru));
        self.cases_on(
            &SemanticType::Bool,
            x,
            motive,
            CoreLevel::Zero,
            &[on_false, on_true],
        )
    }

    /// The fits and value the entry computes over validators' tests
    /// `valid[i]` of the checked parameters, in parameter order.
    fn entry_chain(&mut self, sig: &Sig, tests: &[Term]) -> (Term, Term) {
        let bool = self.bool_ty();
        let value_ty = self.value_ty();
        let (tru, none) = (self.tru(), self.ts("Value.none"));
        let some = self.ts("Value.some");
        let mut fits = sig.fits_applied;
        let mut outcome = self.b.app(some, sig.value);
        for test in tests.iter().rev() {
            fits = self.ite(bool, *test, fits, tru);
            outcome = self.ite(value_ty, *test, outcome, none);
        }
        (fits, outcome)
    }

    /// `cond (fits) (Obs.value outcome) Obs.overflow`.
    fn entry_observation(&mut self, fits: Term, outcome: Term) -> Term {
        let obs = self.lp("Obs");
        let wrap = self.lp("Obs.value");
        let overflow = self.lp("Obs.overflow");
        let valued = self.b.app(wrap, outcome);
        let cond = self.b.konst1("cond");
        self.b.apps(cond, &[obs, fits, valued, overflow])
    }

    /// `Eq Obs observation rhs`.
    fn obs_eq(&mut self, observation: Term, rhs: Term) -> Term {
        let obs = self.lp("Obs");
        self.eq(obs, observation, rhs)
    }

    /// The entry's rows and its three theorems.
    #[allow(clippy::too_many_lines)]
    fn entry_section(
        &mut self,
        boundary: &Boundary,
        entry: u64,
        validators: &[Option<u64>],
        sig: &Sig,
        parameters: &[SemanticType],
    ) -> Result<(), String> {
        if validators.len() != sig.fits_binders.len() || validators.len() != parameters.len() {
            return Err("the entry validates a parameter list of another length".to_owned());
        }
        let program = self.gen.program;
        let function = program
            .functions
            .get(entry as usize)
            .ok_or_else(|| format!("the program has no entry {entry}"))?;
        let frees: Vec<Term> = sig.fits_binders.iter().map(|binder| binder.free).collect();
        // The checked parameters, in order, with their validators.
        let checked: Vec<(usize, u64)> = validators
            .iter()
            .enumerate()
            .filter_map(|(position, validator)| validator.map(|validator| (position, validator)))
            .collect();
        let mut tests = Vec::new();
        let mut accepts_terms = Vec::new();
        for (position, validator) in &checked {
            tests.push(self.valid_at(*validator, frees[*position]));
            let inv = self.inv_const(*validator);
            accepts_terms.push(self.b.app(inv, frees[*position]));
        }
        let (fits, outcome) = self.entry_chain(sig, &tests);
        let mut accepts = self.true_prop();
        for term in accepts_terms.iter().rev() {
            accepts = self.and_prop(*term, accepts);
        }
        let bool = self.bool_ty();
        let value_ty = self.value_ty();
        let obs = self.lp("Obs");
        let prop = self.b.prop();
        // entryFits, entryValue, denoteEntry, accepts.
        let fits_value = self.lambdas(&sig.fits_binders, fits);
        let fits_ty = self.pis(&sig.fits_binders, bool);
        self.push_row(
            "entryFits",
            RowKind::Definition { compiled: true },
            fits_ty,
            fits_value,
        )?;
        let outcome_value = self.lambdas(&sig.fits_binders, outcome);
        let outcome_ty = self.pis(&sig.fits_binders, value_ty);
        self.push_row(
            "entryValue",
            RowKind::Definition { compiled: true },
            outcome_ty,
            outcome_value,
        )?;
        let entry_fits = self.own("entryFits");
        let entry_fits_at = self.b.apps(entry_fits, &frees);
        let entry_value = self.own("entryValue");
        let entry_value_at = self.b.apps(entry_value, &frees);
        let observed = self.entry_observation(entry_fits_at, entry_value_at);
        let denote_value = self.lambdas(&sig.fits_binders, observed);
        let denote_ty = self.pis(&sig.fits_binders, obs);
        self.push_row(
            "denoteEntry",
            RowKind::Definition { compiled: true },
            denote_ty,
            denote_value,
        )?;
        let accepts_value = self.lambdas(&sig.fits_binders, accepts);
        let accepts_ty = self.pis(&sig.fits_binders, prop);
        self.push_row(
            "accepts",
            RowKind::Definition { compiled: false },
            accepts_ty,
            accepts_value,
        )?;
        let denote_entry = self.own("denoteEntry");
        let denote_entry_at = self.b.apps(denote_entry, &frees);
        let accepts_head = self.own("accepts");
        let accepts_at = self.b.apps(accepts_head, &frees);
        // entry.
        let mut encoded = Vec::new();
        for (parameter, free) in parameters.iter().zip(&frees) {
            let enc = self.enc(parameter)?;
            encoded.push(self.b.app(enc, *free));
        }
        let empty = self.empty_env();
        let env = self.extend_env(empty, &function.parameters, &encoded)?;
        let expected_body = self.expected_entry(function, validators)?;
        let mut layers: Vec<(usize, u64, &Expr, &Expr)> = Vec::new();
        let mut cursor = &expected_body;
        for (position, validator) in &checked {
            let Expr::Cond {
                condition,
                then_branch,
                else_branch,
            } = cursor
            else {
                return Err("the entry was lowered without a test of a validator".to_owned());
            };
            let Expr::Call {
                function: callee,
                operands,
            } = condition.as_ref()
            else {
                return Err("the entry tests something other than a validator's call".to_owned());
            };
            if callee != validator || operands.len() != 1 {
                return Err(format!(
                    "the entry was lowered without the call of the validator {validator}"
                ));
            }
            layers.push((
                *position,
                *validator,
                then_branch.as_ref(),
                else_branch.as_ref(),
            ));
            cursor = then_branch.as_ref();
        }
        let Expr::Build {
            shape,
            ty: outcome_ty_program,
            operands,
        } = cursor
        else {
            return Err("the entry was lowered without the root's call".to_owned());
        };
        if operands.len() != 1 {
            return Err("the entry builds its result from another number of operands".to_owned());
        }
        let shape = *shape;
        let mut argument_pfs = Vec::new();
        for (position, number) in function.parameters.iter().enumerate() {
            argument_pfs.push(self.variable_pf(env, *number, encoded[position])?);
        }
        let argument_list = self.operands(env, &argument_pfs)?;
        let prog = self.prog;
        let rel_head = self.own("__rel_0");
        let rel_proof = self.b.apps(rel_head, &sig.rel_params);
        let called = self.apply(
            "conv_call",
            &[("p", prog), ("env", env)],
            &[
                Given::Proof(argument_list.proof, argument_list.ty),
                Given::Proof(rel_proof, sig.statement),
            ],
        )?;
        let call_list = self.operands(env, &[called])?;
        let construct = self.apply("construct_some", &[("a", sig.value)], &[])?;
        let shape_term = self.shape_term(shape);
        let outcome_ty_term = self.ty_term(outcome_ty_program);
        let built = self.apply(
            "conv_build",
            &[
                ("p", prog),
                ("env", env),
                ("shape", shape_term),
                ("t", outcome_ty_term),
            ],
            &[
                Given::Proof(call_list.proof, call_list.ty),
                Given::Proof(construct.proof, construct.ty),
            ],
        )?;
        // The call's width predicate is conjoined with `true` twice more; at
        // each value of the predicate that is the predicate.
        let built_fits = self.fits_of(built)?;
        let equation = self.bool_cases_eq(sig.fits_applied, built_fits)?;
        let mut current = self.apply(
            "Conv.fits_eq",
            &[("f'", sig.fits_applied)],
            &[Given::Proof(built.proof, built.ty), Given::Term(equation)],
        )?;
        let some = self.ts("Value.some");
        let mut fits_inside = sig.fits_applied;
        let mut outcome_inside = self.b.app(some, sig.value);
        let none = self.ts("Value.none");
        let tru = self.tru();
        for (position, validator, then_branch, else_branch) in layers.iter().rev() {
            let callee = boundary.get(*validator)?.clone();
            let x = frees[*position];
            let guard = self.b.free();
            let on_fits = self.ite(bool, guard, fits_inside, tru);
            let predicate = self.b.lam_over("__c", EXPLICIT, bool, guard, on_fits);
            let guard = self.b.free();
            let on_value = self.ite(value_ty, guard, outcome_inside, none);
            let value = self.b.lam_over("__c", EXPLICIT, bool, guard, on_value);
            let test = self.valid_at(*validator, x);
            // The test: the validator's relation at the parameter.
            let argument =
                self.variable_pf(env, function.parameters[*position], encoded[*position])?;
            let list = self.operands(env, &[argument])?;
            let relation_head = self.vrel_const(*validator);
            let relation = self.b.app(relation_head, x);
            let relation_ty = self.vrel_claim(&callee, x);
            let tested = self.apply(
                "conv_call",
                &[("p", prog), ("env", env)],
                &[
                    Given::Proof(list.proof, list.ty),
                    Given::Proof(relation, relation_ty),
                ],
            )?;
            let refused = self.refusal_pf(env, else_branch)?;
            let fls = self.fls();
            let on_true = {
                let ty = self.eq(bool, test, tru);
                self.b.lam("_", EXPLICIT, ty, current.proof)
            };
            let on_false = {
                let ty = self.eq(bool, test, fls);
                self.b.lam("_", EXPLICIT, ty, refused.proof)
            };
            let then_term = self.expr_term(then_branch)?;
            let else_term = self.expr_term(else_branch)?;
            current = self.apply(
                "conv_cond",
                &[
                    ("p", prog),
                    ("env", env),
                    ("t", then_term),
                    ("e", else_term),
                ],
                &[
                    Given::Term(predicate),
                    Given::Term(value),
                    Given::Term(test),
                    Given::Proof(tested.proof, tested.ty),
                    Given::Term(on_true),
                    Given::Term(on_false),
                ],
            )?;
            fits_inside = self.ite(bool, test, fits_inside, tru);
            outcome_inside = self.ite(value_ty, test, outcome_inside, none);
        }
        let function_term = self.function_term(function)?;
        let entry_index = self.nat_u(entry);
        let introduced = self.apply(
            "funRel_intro",
            &[
                ("p", prog),
                ("fi", entry_index),
                ("args", sig.arguments),
                ("o", denote_entry_at),
                ("fn", function_term),
                ("env", env),
            ],
            &[
                Given::Refl,
                Given::Refl,
                Given::Proof(current.proof, current.ty),
            ],
        )?;
        self.forget_expression(&expected_body);
        let run = self.apply(
            "run_of_funRel",
            &[],
            &[Given::Proof(introduced.proof, introduced.ty)],
        )?;
        let run_conv = self.lp("RunConv");
        let statement = self.b.apps(
            run_conv,
            &[prog, entry_index, sig.arguments, denote_entry_at],
        );
        let entry_value_term = self.lambdas(&sig.rel_binders, run.proof);
        let entry_ty = self.pis(&sig.rel_binders, statement);
        self.push_row("entry", RowKind::Theorem, entry_ty, entry_value_term)?;
        // entry_accepts and entry_refuses.
        let denote_head = self.own("denote");
        let denote_at = self.b.apps(denote_head, &frees);
        let some_obs = self.lp("someObs");
        let accepted_obs = self.b.app(some_obs, denote_at);
        let hypothesis = self.b.free();
        let accepts_proof =
            self.entry_accepts_proof(sig, &checked, &frees, &tests, &accepts_terms, hypothesis)?;
        let accepted = self.obs_eq(denote_entry_at, accepted_obs);
        let accepted_pi = self
            .b
            .pi_over("__h", EXPLICIT, accepts_at, hypothesis, accepted);
        let accepted_ty = self.pis(&sig.fits_binders, accepted_pi);
        let accepted_proof =
            self.b
                .lam_over("__h", EXPLICIT, accepts_at, hypothesis, accepts_proof);
        let accepted_value = self.lambdas(&sig.fits_binders, accepted_proof);
        self.push_row(
            "entry_accepts",
            RowKind::Theorem,
            accepted_ty,
            accepted_value,
        )?;
        let hypothesis = self.b.free();
        let refusal =
            self.entry_refuses_proof(sig, &checked, &frees, &tests, &accepts_terms, hypothesis)?;
        let not = self.b.konst("Not");
        let refused_hypothesis = self.b.app(not, accepts_at);
        let value_wrap = self.lp("Obs.value");
        let none_value = self.b.app(value_wrap, none);
        let refused = self.obs_eq(denote_entry_at, none_value);
        let refused_pi = self
            .b
            .pi_over("__h", EXPLICIT, refused_hypothesis, hypothesis, refused);
        let refused_ty = self.pis(&sig.fits_binders, refused_pi);
        let refused_proof =
            self.b
                .lam_over("__h", EXPLICIT, refused_hypothesis, hypothesis, refusal);
        let refused_value = self.lambdas(&sig.fits_binders, refused_proof);
        self.push_row("entry_refuses", RowKind::Theorem, refused_ty, refused_value)
    }

    /// The program's refusal: `build none`.
    fn refusal_pf(&mut self, env: Term, expr: &Expr) -> Result<Pf, String> {
        let Expr::Build {
            shape,
            ty,
            operands,
        } = expr
        else {
            return Err("the entry was lowered without its refusal".to_owned());
        };
        if !operands.is_empty() {
            return Err("the entry's refusal builds from operands".to_owned());
        }
        let shape = *shape;
        let empty = self.operands(env, &[])?;
        let construct = self.apply("construct_none", &[], &[])?;
        let shape = self.shape_term(shape);
        let ty = self.ty_term(ty);
        let prog = self.prog;
        self.apply(
            "conv_build",
            &[("p", prog), ("env", env), ("shape", shape), ("t", ty)],
            &[
                Given::Proof(empty.proof, empty.ty),
                Given::Proof(construct.proof, construct.ty),
            ],
        )
    }

    /// `Eq Obs (cond (fits tests) (value (outcome tests)) overflow) rhs`.
    fn entry_goal(&mut self, sig: &Sig, tests: &[Term], rhs: Term) -> Term {
        let (fits, outcome) = self.entry_chain(sig, tests);
        let observed = self.entry_observation(fits, outcome);
        self.obs_eq(observed, rhs)
    }

    /// The proof that the entry returns the root's observation, as `some`,
    /// when every proposition holds: each test is rewritten to `true` along
    /// the decision theorem, from the base case the width predicate splits.
    fn entry_accepts_proof(
        &mut self,
        sig: &Sig,
        checked: &[(usize, u64)],
        frees: &[Term],
        tests: &[Term],
        invariants: &[Term],
        hypothesis: Term,
    ) -> Result<Term, String> {
        let obs = self.lp("Obs");
        let denote_head = self.own("denote");
        let denote_at = self.b.apps(denote_head, frees);
        let some_obs = self.lp("someObs");
        let rhs = self.b.app(some_obs, denote_at);
        // The base case: every test is `true`.
        let trues: Vec<Term> = tests.iter().map(|_| self.tru()).collect();
        let free = self.b.free();
        let some = self.ts("Value.some");
        let outcome = self.b.app(some, sig.value);
        let unconditional = self.entry_observation(free, outcome);
        let plain = self.entry_observation(free, sig.value);
        let some_plain = self.b.app(some_obs, plain);
        let claim = self.obs_eq(unconditional, some_plain);
        let bool = self.bool_ty();
        let motive = self.b.lam_over("x", EXPLICIT, bool, free, claim);
        let (tru, fls) = (self.tru(), self.fls());
        let overflow_case = {
            let observed = self.entry_observation(fls, outcome);
            self.refl(obs, observed)
        };
        let value_case = {
            let observed = self.entry_observation(tru, outcome);
            self.refl(obs, observed)
        };
        let base = self.cases_on(
            &SemanticType::Bool,
            sig.fits_applied,
            motive,
            CoreLevel::Zero,
            &[overflow_case, value_case],
        )?;
        let mut proof = base;
        let mut callers = trues;
        // The invariants' conjunction, taken apart from the first.
        let mut rest = hypothesis;
        let mut suffixes: Vec<Term> = Vec::new();
        {
            let mut tail = self.true_prop();
            for term in invariants.iter().rev() {
                suffixes.push(tail);
                tail = self.and_prop(*term, tail);
            }
            suffixes.reverse();
        }
        for (index, ((_, validator), test)) in checked.iter().zip(tests).enumerate() {
            let invariant = invariants[index];
            let suffix = suffixes[index];
            let left = {
                let head = self.b.konst("And.left");
                self.b.apps(head, &[invariant, suffix, rest])
            };
            let next = {
                let head = self.b.konst("And.right");
                self.b.apps(head, &[invariant, suffix, rest])
            };
            let viff = {
                let head = self.viff_const(*validator);
                self.b.app(head, frees[checked[index].0])
            };
            let decided = self.is_true(*test);
            let equation = {
                let head = self.b.konst("Iff.mpr");
                self.b.apps(head, &[decided, invariant, viff, left])
            };
            // Rewrite position `index` of the goal from `true` to the test.
            let free = self.b.free();
            let mut moved = callers.clone();
            moved[index] = free;
            let goal = self.entry_goal(sig, &moved, rhs);
            let motive = self.b.lam_over("__b", EXPLICIT, bool, free, goal);
            let symmetric = {
                let head = self.b.konst1("Eq.symm");
                self.b.apps(head, &[bool, *test, tru, equation])
            };
            let rewrite = self.b.konst_at(
                "Eq.ndrec",
                vec![CoreLevel::Zero, CoreLevel::Succ(Box::new(CoreLevel::Zero))],
            );
            proof = self
                .b
                .apps(rewrite, &[bool, tru, motive, proof, *test, symmetric]);
            callers[index] = *test;
            rest = next;
        }
        Ok(proof)
    }

    /// The proof that the entry returns `none` when a proposition fails: for
    /// all Booleans in place of the tests, by cases on each in turn, with the
    /// remaining tests' being irrelevant once one is `false`.
    fn entry_refuses_proof(
        &mut self,
        sig: &Sig,
        checked: &[(usize, u64)],
        frees: &[Term],
        tests: &[Term],
        invariants: &[Term],
        hypothesis: Term,
    ) -> Result<Term, String> {
        let bool = self.bool_ty();
        let fls = self.fls();
        let none = self.ts("Value.none");
        let wrap = self.lp("Obs.value");
        let rhs = self.b.app(wrap, none);
        let obs = self.lp("Obs");
        let count = tests.len();
        let variables: Vec<Term> = (0..count).map(|_| self.b.free()).collect();
        // The conjunction of `b = true` over `b`s.
        let all_true = |writer: &mut Self, bs: &[Term]| -> Term {
            let mut out = writer.true_prop();
            for b in bs.iter().rev() {
                let eq = writer.is_true(*b);
                out = writer.and_prop(eq, out);
            }
            out
        };
        // The proof of `all_true` at `trues`.
        let truth_proof = |writer: &mut Self, length: usize| -> Term {
            let mut out = writer.b.konst("True.intro");
            let eq = {
                let t = writer.tru();
                writer.is_true(t)
            };
            let trues: Vec<Term> = (0..length).map(|_| writer.tru()).collect();
            for index in (0..length).rev() {
                let rest = all_true(writer, &trues[index + 1..]);
                let refl = {
                    let t = writer.tru();
                    writer.refl(bool, t)
                };
                let intro = writer.b.konst("And.intro");
                out = writer.b.apps(intro, &[eq, rest, refl, out]);
            }
            out
        };
        // `refuse(j)`: for the first `j` tests `true` and the rest the
        // variables, the proof of `¬ all_true → goal`.
        let mut next = {
            // j = count: every test is true, so the hypothesis is refuted.
            let trues: Vec<Term> = (0..count).map(|_| self.tru()).collect();
            let goal = self.entry_goal(sig, &trues, rhs);
            let accepted = all_true(self, &trues);
            let not = self.b.konst("Not");
            let refuted = self.b.app(not, accepted);
            let witness = truth_proof(self, count);
            let absurd = self.b.konst_at("absurd", vec![CoreLevel::Zero]);
            let free = self.b.free();
            let body = self.b.apps(absurd, &[accepted, goal, witness, free]);
            self.b.lam_over("__n", EXPLICIT, refuted, free, body)
        };
        for index in (0..count).rev() {
            let mut callers: Vec<Term> = (0..index).map(|_| self.tru()).collect();
            callers.push(variables[index]);
            callers.extend(variables[index + 1..].iter().copied());
            let goal = self.entry_goal(sig, &callers, rhs);
            let accepted = all_true(self, &callers);
            let not = self.b.konst("Not");
            let refuted = self.b.app(not, accepted);
            let claim = self.b.arrow(refuted, goal);
            // The motive abstracts the variable at `index`.
            let abstracted = self
                .b
                .lam_over("__b", EXPLICIT, bool, variables[index], claim);
            // `false`: the goal holds whatever follows.
            let mut at_false = callers.clone();
            at_false[index] = fls;
            let false_accepted = all_true(self, &at_false);
            let false_refuted = self.b.app(not, false_accepted);
            let on_false = {
                let free = self.b.free();
                let value_wrap = self.lp("Obs.value");
                let value = self.b.app(value_wrap, none);
                let refl = self.refl(obs, value);
                self.b.lam_over("__n", EXPLICIT, false_refuted, free, refl)
            };
            let cases = self.cases_on(
                &SemanticType::Bool,
                variables[index],
                abstracted,
                CoreLevel::Zero,
                &[on_false, next],
            )?;
            next = cases;
        }
        // Instantiate the variables at the tests.
        let mut lemma = next;
        for variable in variables.iter().rev() {
            lemma = self.b.lam_over("__b", EXPLICIT, bool, *variable, lemma);
        }
        let applied = self.b.apps(lemma, tests);
        // `¬ all_true tests`, from the hypothesis `¬ accepts`.
        let accepted = all_true(self, tests);
        let hold = self.b.free();
        let mut inv_proof = self.b.konst("True.intro");
        // Build the invariants from the equations, last first.
        let mut suffixes: Vec<Term> = Vec::new();
        {
            let mut tail = self.true_prop();
            for term in invariants.iter().rev() {
                suffixes.push(tail);
                tail = self.and_prop(*term, tail);
            }
            suffixes.reverse();
        }
        // The conjunction of equations, taken apart from the first.
        let mut equations: Vec<Term> = Vec::new();
        let mut cursor = hold;
        for index in 0..count {
            let decided = self.is_true(tests[index]);
            let tail = all_true(self, &tests[index + 1..]);
            let left = {
                let head = self.b.konst("And.left");
                self.b.apps(head, &[decided, tail, cursor])
            };
            let right = {
                let head = self.b.konst("And.right");
                self.b.apps(head, &[decided, tail, cursor])
            };
            equations.push(left);
            cursor = right;
        }
        for index in (0..count).rev() {
            let decided = self.is_true(tests[index]);
            let viff = {
                let head = self.viff_const(checked[index].1);
                self.b.app(head, frees[checked[index].0])
            };
            let invariant = invariants[index];
            let holds = {
                let head = self.b.konst("Iff.mp");
                self.b
                    .apps(head, &[decided, invariant, viff, equations[index]])
            };
            let intro = self.b.konst("And.intro");
            inv_proof = self
                .b
                .apps(intro, &[invariant, suffixes[index], holds, inv_proof]);
        }
        let denied = self.b.app(hypothesis, inv_proof);
        let refutation = self.b.lam_over("__a", EXPLICIT, accepted, hold, denied);
        Ok(self.b.app(applied, refutation))
    }
}
