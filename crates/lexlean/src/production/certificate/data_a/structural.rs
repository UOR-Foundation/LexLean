//! Functions that recurse structurally (SPEC.md §17.17.4): the width
//! predicate and relation theorem of each are defined by the recursor of the
//! argument's type, its induction hypotheses standing for the recursive calls,
//! and the predicate has an implementation record for the code generator.

use std::collections::{BTreeMap, HashMap};

use super::super::{CLocal, Ctx, Gen, Hypotheses, Recursion, Subject};
use super::recplan::RecPlan;
use super::rows::{Binder, LocalSet, Sig};
use super::{LocalTerm, RowKind, Scope, Writer, EXPLICIT};
use crate::calculus::Expr;
use crate::ir::core::CoreLevel;
use crate::ir::semantic::{SemanticTerm, SemanticType};
use crate::production::kernel::{Given, Term};
use crate::production::source::{Local, Site};

/// The minor premises of one constructor: the width-predicate minor, the
/// relation minor, and the direct form of each function's, by member slot.
type StructuralMinors = (Term, Term, Vec<(usize, Term)>);

/// One function of a structurally recursive group.
struct Member<'a> {
    index: u64,
    module: String,
    name: String,
    body: &'a SemanticTerm,
    site: Site,
    type_parameters: Vec<(String, SemanticType)>,
    argument: String,
    locals: Vec<CLocal>,
    /// The position of the recursive argument among the locals.
    arg_position: usize,
    /// The position in the plan its argument's type has.
    plan_position: usize,
    /// The binder index of the argument among the width predicate's binders
    /// and the relation's.
    fits_arg: usize,
    rel_arg: usize,
    sig: Sig,
}

/// The hypotheses of one minor premise: for each recursive field of a
/// constructor, the free variable of the field and its hypotheses for the
/// width predicate and the relation, with the position recursed on.
struct Hypothesis {
    field: Term,
    fits: Term,
    rel: Term,
    position: usize,
}

impl<'a> Writer<'_, 'a> {
    /// A strongly connected component of structurally recursive functions.
    #[allow(clippy::too_many_lines)]
    pub(super) fn structural_component(&mut self, component: &[u64]) -> Result<(), String> {
        // The members, with the signatures every body is proved against.
        let mut members: Vec<Member<'a>> = Vec::new();
        for index in component {
            let subject = self.gen.subject_of(*index)?;
            let Subject::Definition {
                module,
                name,
                type_arguments,
                locals,
                result,
                body,
                recursion,
                site,
                type_parameters,
            } = subject
            else {
                return Err(format!("function {index} is not a definition"));
            };
            let Recursion::Structural { argument } = recursion else {
                return Err(format!("`{module}.{name}` is not structurally recursive"));
            };
            let clocals = Gen::scope(&locals);
            let arg_position = clocals
                .iter()
                .position(|local| local.local.name == argument)
                .ok_or_else(|| format!("`{argument}` is not a parameter of `{name}`"))?;
            let set = self.local_set(*index, &clocals)?;
            let denotation = self.denotation_of(&module, &name, &type_arguments, &set)?;
            let sig = self.signature(*index, &clocals, &set, &result, denotation)?;
            self.sigs.insert(*index, sig.clone());
            let width = |local: &CLocal, rel: bool| -> usize {
                if local.has_companion {
                    if rel {
                        5
                    } else {
                        2
                    }
                } else {
                    1
                }
            };
            let fits_arg = clocals[..arg_position]
                .iter()
                .map(|l| width(l, false))
                .sum();
            let rel_arg = clocals[..arg_position].iter().map(|l| width(l, true)).sum();
            members.push(Member {
                index: *index,
                module,
                name,
                body,
                site,
                type_parameters,
                argument,
                locals: clocals,
                arg_position,
                plan_position: 0,
                fits_arg,
                rel_arg,
                sig,
            });
        }
        let first_type = members[0].locals[members[0].arg_position].local.ty.clone();
        let plan = self.rec_plan(&first_type)?;
        for member in &mut members {
            let ty = member.locals[member.arg_position].local.ty.clone();
            member.plan_position = plan
                .position_of(self.gen, &ty)
                .ok_or_else(|| format!("`{}` recurses on a type outside its group", member.name))?;
        }
        let mut at_position: Vec<Vec<usize>> = vec![Vec::new(); plan.positions.len()];
        for (slot, member) in members.iter().enumerate() {
            at_position[member.plan_position].push(slot);
        }
        let one = CoreLevel::Succ(Box::new(CoreLevel::Zero));
        let bool = self.b.konst("Bool");
        // The motives: a function of the argument into the statements of the
        // functions that recurse on its type, each for every other argument.
        // Two functions on one type share a motive, which packs their
        // statements as a pair.
        let mut fits_motives = Vec::new();
        let mut rel_motives = Vec::new();
        let mut domains = Vec::new();
        for position in 0..plan.positions.len() {
            let domain = self.lean_ty(&plan.positions[position].ty)?;
            domains.push(domain);
            if at_position[position].is_empty() {
                return Err("a nested type of the group has no function recursing on it".to_owned());
            }
            let fits_tys = self.component_types(&members, &at_position[position], false)?;
            let rel_tys = self.component_types(&members, &at_position[position], true)?;
            let fits_body = self.pack_types(&fits_tys, false);
            let rel_body = self.pack_types(&rel_tys, true);
            let first = &members[at_position[position][0]];
            // The motive is a function of the argument: each statement is
            // closed over its own member's argument binder, which the common
            // bound variable replaces.
            let mut fits_body = fits_body;
            let mut rel_body = rel_body;
            let bound = self.b.free();
            for slot in &at_position[position] {
                let member = &members[*slot];
                fits_body = self.b.substitute(
                    fits_body,
                    &[member.sig.fits_binders[member.fits_arg].free],
                    &[bound],
                );
                rel_body = self.b.substitute(
                    rel_body,
                    &[member.sig.rel_binders[member.rel_arg].free],
                    &[bound],
                );
            }
            let _ = first;
            fits_motives.push(self.b.lam_over("__v", EXPLICIT, domain, bound, fits_body));
            rel_motives.push(self.b.lam_over("__v", EXPLICIT, domain, bound, rel_body));
        }
        // The minor premises, position by position, constructor by
        // constructor.
        let mut fits_minors = Vec::new();
        let mut rel_minors = Vec::new();
        let mut direct_minors: Vec<Vec<Term>> = vec![Vec::new(); members.len()];
        for position in 0..plan.positions.len() {
            for ctor in 0..plan.positions[position].constructors.len() {
                let (fits_minor, rel_minor, direct) =
                    self.structural_minors(&plan, &members, &at_position, position, ctor)?;
                fits_minors.push(fits_minor);
                rel_minors.push(rel_minor);
                for (slot, term) in direct {
                    direct_minors[slot].push(term);
                }
            }
        }
        let mut params = Vec::new();
        for argument in &plan.params {
            params.push(self.lean_ty(argument)?);
        }
        // The rows of each member.
        let mut fits_names = Vec::new();
        for member in &members {
            let position = member.plan_position;
            let siblings = &at_position[position];
            let index = siblings
                .iter()
                .position(|slot| members[*slot].index == member.index)
                .ok_or("a member outside its position")?;
            let recursor = self
                .b
                .konst_at(&plan.recursors[position], plan.levels(one.clone()));
            let mut fits_args = params.clone();
            fits_args.extend(fits_motives.iter().copied());
            fits_args.extend(fits_minors.iter().copied());
            fits_args.push(member.sig.fits_binders[member.fits_arg].free);
            let applied = self.b.apps(recursor, &fits_args);
            let fits_tys = self.component_types_at(
                &members,
                siblings,
                false,
                member.sig.fits_binders[member.fits_arg].free,
            )?;
            let selected = self.unpack(&fits_tys, index, applied, false);
            let others: Vec<Term> = member
                .sig
                .fits_binders
                .iter()
                .enumerate()
                .filter(|(slot, _)| *slot != member.fits_arg)
                .map(|(_, binder)| binder.free)
                .collect();
            let body = self.b.apps(selected, &others);
            let value = self.lambdas(&member.sig.fits_binders, body);
            let row_ty = self.pis(&member.sig.fits_binders, bool);
            fits_names.push(format!("__fits_{}", member.index));
            self.push_row(
                &format!("__fits_{}", member.index),
                RowKind::Definition { compiled: true },
                row_ty,
                value,
            )?;
        }
        self.group_rows(&fits_names);
        for member in &members {
            let position = member.plan_position;
            let siblings = &at_position[position];
            let index = siblings
                .iter()
                .position(|slot| members[*slot].index == member.index)
                .ok_or("a member outside its position")?;
            let recursor = self
                .b
                .konst_at(&plan.recursors[position], plan.levels(CoreLevel::Zero));
            let mut rel_args = params.clone();
            rel_args.extend(rel_motives.iter().copied());
            rel_args.extend(rel_minors.iter().copied());
            rel_args.push(member.sig.rel_binders[member.rel_arg].free);
            let applied = self.b.apps(recursor, &rel_args);
            let rel_tys = self.component_types_at(
                &members,
                siblings,
                true,
                member.sig.rel_binders[member.rel_arg].free,
            )?;
            let selected = self.unpack(&rel_tys, index, applied, true);
            let others: Vec<Term> = member
                .sig
                .rel_binders
                .iter()
                .enumerate()
                .filter(|(slot, _)| *slot != member.rel_arg)
                .map(|(_, binder)| binder.free)
                .collect();
            let body = self.b.apps(selected, &others);
            let value = self.lambdas(&member.sig.rel_binders, body);
            let row_ty = self.pis(&member.sig.rel_binders, member.sig.statement);
            self.push_row(
                &format!("__rel_{}", member.index),
                RowKind::Theorem,
                row_ty,
                value,
            )?;
        }
        // The implementation records of the predicates, by direct recursion.
        let unsafe_names: Vec<String> = members
            .iter()
            .map(|member| format!("__fits_{}._unsafe_rec", member.index))
            .collect();
        for (slot, member) in members.iter().enumerate() {
            let position = member.plan_position;
            let discriminant = member.sig.fits_binders[member.fits_arg].free;
            let ty = plan.positions[position].ty.clone();
            let minors = direct_minors[slot].clone();
            let cases = self.cases_on_const(&ty, discriminant, bool, &minors)?;
            let value = self.lambdas(&member.sig.fits_binders, cases);
            let row_ty = self.pis(&member.sig.fits_binders, bool);
            self.push_row(
                &format!("__fits_{}._unsafe_rec", member.index),
                RowKind::Partial,
                row_ty,
                value,
            )?;
        }
        self.group_rows(&unsafe_names);
        Ok(())
    }

    /// The statements of the functions at a position, over the position's
    /// argument: the type of the other arguments to the result.
    fn component_types(
        &mut self,
        members: &[Member<'a>],
        slots: &[usize],
        relation: bool,
    ) -> Result<Vec<Term>, String> {
        let bool = self.b.konst("Bool");
        let mut out = Vec::new();
        for slot in slots {
            let member = &members[*slot];
            if relation {
                let others: Vec<Binder> = member
                    .sig
                    .rel_binders
                    .iter()
                    .enumerate()
                    .filter(|(position, _)| *position != member.rel_arg)
                    .map(|(_, binder)| binder.clone())
                    .collect();
                out.push(self.pis(&others, member.sig.statement));
            } else {
                let others: Vec<Binder> = member
                    .sig
                    .fits_binders
                    .iter()
                    .enumerate()
                    .filter(|(position, _)| *position != member.fits_arg)
                    .map(|(_, binder)| binder.clone())
                    .collect();
                out.push(self.pis(&others, bool));
            }
        }
        Ok(out)
    }

    /// The types of a position's functions as one: the pair type
    /// `PProd a (PProd b c)`, or for statements `a ∧ b ∧ c`.
    pub(super) fn pack_types(&mut self, tys: &[Term], relation: bool) -> Term {
        let Some((first, rest)) = tys.split_first() else {
            return self.b.konst("PUnit");
        };
        if rest.is_empty() {
            return *first;
        }
        let packed = self.pack_types(rest, relation);
        if relation {
            let and = self.b.konst("And");
            self.b.apps(and, &[*first, packed])
        } else {
            let one = CoreLevel::Succ(Box::new(CoreLevel::Zero));
            let head = self.b.konst_at("PProd", vec![one.clone(), one]);
            self.b.apps(head, &[*first, packed])
        }
    }

    /// The pair of values of types `tys`.
    pub(super) fn pack_values(&mut self, tys: &[Term], values: &[Term], relation: bool) -> Term {
        let (first, rest) = (values[0], &values[1..]);
        if rest.is_empty() {
            return first;
        }
        let packed_ty = self.pack_types(&tys[1..], relation);
        let packed = self.pack_values(&tys[1..], rest, relation);
        if relation {
            let intro = self.b.konst("And.intro");
            self.b.apps(intro, &[tys[0], packed_ty, first, packed])
        } else {
            let one = CoreLevel::Succ(Box::new(CoreLevel::Zero));
            let head = self.b.konst_at("PProd.mk", vec![one.clone(), one]);
            self.b.apps(head, &[tys[0], packed_ty, first, packed])
        }
    }

    /// The component `index` of a packed value of types `tys`.
    pub(super) fn unpack(
        &mut self,
        tys: &[Term],
        index: usize,
        packed: Term,
        relation: bool,
    ) -> Term {
        if tys.len() == 1 {
            return packed;
        }
        let rest_ty = self.pack_types(&tys[1..], relation);
        if index == 0 {
            if relation {
                let head = self.b.konst("And.left");
                self.b.apps(head, &[tys[0], rest_ty, packed])
            } else {
                let one = CoreLevel::Succ(Box::new(CoreLevel::Zero));
                let head = self.b.konst_at("PProd.fst", vec![one.clone(), one]);
                self.b.apps(head, &[tys[0], rest_ty, packed])
            }
        } else {
            let rest = if relation {
                let head = self.b.konst("And.right");
                self.b.apps(head, &[tys[0], rest_ty, packed])
            } else {
                let one = CoreLevel::Succ(Box::new(CoreLevel::Zero));
                let head = self.b.konst_at("PProd.snd", vec![one.clone(), one]);
                self.b.apps(head, &[tys[0], rest_ty, packed])
            };
            self.unpack(&tys[1..], index - 1, rest, relation)
        }
    }

    /// The denotation of a definition instance applied to its locals.
    fn denotation_of(
        &mut self,
        module: &str,
        name: &str,
        type_arguments: &[SemanticType],
        set: &LocalSet,
    ) -> Result<Term, String> {
        let lean_name = self.gen.source.lean_name(
            module,
            &crate::ir::semantic::MemberRef {
                module: Some(module.to_owned()),
                name: name.to_owned(),
            },
        );
        let head = self.b.konst(&lean_name);
        let mut args = Vec::new();
        for argument in type_arguments {
            args.push(self.lean_ty(argument)?);
        }
        args.extend(set.terms.iter().map(|terms| terms.value));
        Ok(self.b.apps(head, &args))
    }

    /// The component types of a position's functions with the argument
    /// replaced by `value`.
    fn component_types_at(
        &mut self,
        members: &[Member<'a>],
        slots: &[usize],
        relation: bool,
        value: Term,
    ) -> Result<Vec<Term>, String> {
        let tys = self.component_types(members, slots, relation)?;
        let mut out = Vec::new();
        for (ty, slot) in tys.into_iter().zip(slots) {
            let member = &members[*slot];
            let at = if relation {
                member.sig.rel_binders[member.rel_arg].free
            } else {
                member.sig.fits_binders[member.fits_arg].free
            };
            out.push(self.b.substitute(ty, &[at], &[value]));
        }
        Ok(out)
    }

    /// The minor premises of constructor `ctor` of a position of the plan:
    /// for the width predicates, for the relations (each packed over the
    /// functions at the position), and the direct form of each function's.
    #[allow(clippy::too_many_lines)]
    fn structural_minors(
        &mut self,
        plan: &RecPlan,
        members: &[Member<'a>],
        at_position: &[Vec<usize>],
        position: usize,
        ctor: usize,
    ) -> Result<StructuralMinors, String> {
        let shape = &plan.positions[position].constructors[ctor];
        let slots = &at_position[position];
        // Free variables: the fields, and a hypothesis for each recursive
        // field.
        let mut field_frees = Vec::new();
        let mut field_types = Vec::new();
        for ty in &shape.fields {
            field_frees.push(self.b.free());
            field_types.push(self.lean_ty(ty)?);
        }
        let mut hypotheses: Vec<Hypothesis> = Vec::new();
        let mut hypothesis_types: Vec<(Term, Term)> = Vec::new();
        for (field_index, recursion) in shape.recursive.iter().enumerate() {
            if let Some(target) = recursion {
                let fits = self.b.free();
                let rel = self.b.free();
                hypotheses.push(Hypothesis {
                    field: field_frees[field_index],
                    fits,
                    rel,
                    position: *target,
                });
                let fits_tys = self.component_types_at(
                    members,
                    &at_position[*target],
                    false,
                    field_frees[field_index],
                )?;
                let rel_tys = self.component_types_at(
                    members,
                    &at_position[*target],
                    true,
                    field_frees[field_index],
                )?;
                hypothesis_types.push((
                    self.pack_types(&fits_tys, false),
                    self.pack_types(&rel_tys, true),
                ));
            }
        }
        let mut fits_inners = Vec::new();
        let mut rel_inners = Vec::new();
        let mut directs = Vec::new();
        let mut constructed_value = None;
        for slot in slots {
            let (fits_inner, rel_inner, direct, constructed) = self.structural_member_minor(
                plan,
                members,
                at_position,
                *slot,
                ctor,
                &field_frees,
                &field_types,
                &hypotheses,
            )?;
            fits_inners.push(fits_inner);
            rel_inners.push(rel_inner);
            directs.push((*slot, direct));
            constructed_value.get_or_insert(constructed);
        }
        let constructed = constructed_value.ok_or("a position with no function")?;
        let fits_tys = self.component_types_at(members, slots, false, constructed)?;
        let rel_tys = self.component_types_at(members, slots, true, constructed)?;
        let fits_body = self.pack_values(&fits_tys, &fits_inners, false);
        let rel_body = self.pack_values(&rel_tys, &rel_inners, true);
        let wrap = |writer: &mut Self, body: Term, relation: bool| -> Term {
            let mut out = body;
            for (hypothesis, tys) in hypotheses.iter().zip(&hypothesis_types).rev() {
                let (free, ty) = if relation {
                    (hypothesis.rel, tys.1)
                } else {
                    (hypothesis.fits, tys.0)
                };
                out = writer.b.lam_over("__ih", EXPLICIT, ty, free, out);
            }
            for (index, free) in field_frees.iter().enumerate().rev() {
                out = writer.b.lam_over(
                    &format!("__x{index}"),
                    EXPLICIT,
                    field_types[index],
                    *free,
                    out,
                );
            }
            out
        };
        let fits_minor = wrap(self, fits_body, false);
        let rel_minor = wrap(self, rel_body, true);
        Ok((fits_minor, rel_minor, directs))
    }

    /// One function's body at a constructor: the width predicate and the
    /// relation, each abstracted over the other arguments, and the direct
    /// form of the predicate over the fields; with the value of the
    /// constructor.
    // The minor's inputs are the plan, the group, and one constructor's
    // fields; bundling them would only rename the same nine values.
    #[allow(clippy::too_many_lines, clippy::too_many_arguments)]
    fn structural_member_minor(
        &mut self,
        plan: &RecPlan,
        members: &[Member<'a>],
        at_position: &[Vec<usize>],
        slot: usize,
        ctor: usize,
        field_frees: &[Term],
        field_types: &[Term],
        hypotheses: &[Hypothesis],
    ) -> Result<(Term, Term, Term, Term), String> {
        let member = &members[slot];
        let shape = &plan.positions[member.plan_position].constructors[ctor];
        // The branch of the source for this constructor.
        let SemanticTerm::Match {
            scrutinee,
            branches,
        } = member.body
        else {
            return Err(format!(
                "structural `{}.{}` is not a top-level match",
                member.module, member.name
            ));
        };
        match scrutinee.as_ref() {
            SemanticTerm::Var { name } if *name == member.argument => {}
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
            | SemanticTerm::Forall { .. }
            | SemanticTerm::Let { .. }
            | SemanticTerm::Pair { .. }
            | SemanticTerm::First { .. }
            | SemanticTerm::Second { .. }
            | SemanticTerm::Lambda { .. }
            | SemanticTerm::Apply { .. }
            | SemanticTerm::FunctionRef { .. }
            | SemanticTerm::MapLiteral { .. }
            | SemanticTerm::SetLiteral { .. }
            | SemanticTerm::GraphLiteral { .. }
            | SemanticTerm::CheckedApply { .. } => {
                return Err(format!(
                    "structural `{}.{}` does not match on `{}`",
                    member.module, member.name, member.argument
                ));
            }
        }
        let argument_ty = member.locals[member.arg_position].local.ty.clone();
        let argument_generic = member.locals[member.arg_position].generic.clone();
        let generic_site = Site {
            module: member.site.module.clone(),
            substitution: BTreeMap::new(),
        };
        let mut chosen = None;
        for (branch_index, branch) in branches.iter().enumerate() {
            let (constructor, fields) =
                self.gen.source.branch(branch, &argument_ty, &member.site)?;
            if self.constructor_position(&constructor)? == ctor {
                chosen = Some((branch_index, branch, constructor, fields));
                break;
            }
        }
        let (branch_index, branch, constructor, fields) = chosen.ok_or_else(|| {
            format!(
                "structural `{}.{}` has no branch for constructor {ctor}",
                member.module, member.name
            )
        })?;
        let (_, generic_fields) =
            self.gen
                .source
                .branch(branch, &argument_generic, &generic_site)?;
        // The other arguments of this minor are fresh.
        let fresh = self.local_set(member.index, &member.locals)?;
        let constructed = self.constructed(
            &constructor,
            &argument_ty,
            &branch.constructor,
            &member.site,
            field_frees,
        )?;
        // The scope: every local but the argument, then the fields.
        let mut ctx_locals: Vec<CLocal> = Vec::new();
        let mut terms: Vec<LocalTerm> = Vec::new();
        for (position, local) in member.locals.iter().enumerate() {
            if position != member.arg_position {
                ctx_locals.push(local.clone());
                terms.push(fresh.terms[position]);
            }
        }
        for ((name, (ty, generic)), free) in branch
            .binders
            .iter()
            .zip(fields.into_iter().zip(generic_fields))
            .zip(field_frees)
        {
            ctx_locals.push(CLocal {
                local: Local {
                    name: name.clone(),
                    ty,
                },
                generic,
                has_companion: false,
            });
            terms.push(LocalTerm {
                value: *free,
                companion: None,
            });
        }
        // The environment: the parameters bound to the encodings.
        let program = self.gen.program;
        let parameters = program.functions[member.index as usize].parameters.clone();
        let mut encoded = Vec::new();
        for (position, local) in member.locals.iter().enumerate() {
            if position == member.arg_position {
                let enc = self.enc(&argument_ty)?;
                encoded.push(self.b.app(enc, constructed));
            } else {
                encoded.push(self.encoded_local(local, fresh.terms[position])?);
            }
        }
        let empty = self.empty_env();
        let env = self.extend_env(empty, &parameters, &encoded)?;
        let scope = Scope {
            ctx: Ctx::new(
                member.site.clone(),
                member.type_parameters.clone(),
                ctx_locals,
                Hypotheses::new(),
                member.index,
            ),
            terms,
            env,
            path: Vec::new(),
        };
        let top = &program.functions[member.index as usize].body;
        let (match_ty, scrutinee_expr, arms) = match top {
            Expr::Match {
                ty,
                scrutinee,
                arms,
            } => (ty, scrutinee.as_ref(), arms.as_slice()),
            Expr::Value { .. }
            | Expr::Var { .. }
            | Expr::Let { .. }
            | Expr::Cond { .. }
            | Expr::Build { .. }
            | Expr::Call { .. }
            | Expr::Closure { .. }
            | Expr::Apply { .. }
            | Expr::Prim { .. }
            | Expr::First { .. }
            | Expr::Second { .. }
            | Expr::Field { .. } => {
                return Err("the lowering of a structural body is not a match".to_owned())
            }
        };
        let arm = arms
            .get(branch_index)
            .ok_or("a structural match lowered to fewer arms")?;
        let mut arm_fields = Vec::new();
        for (field_ty, free) in shape.fields.iter().zip(field_frees) {
            let enc = self.enc(field_ty)?;
            arm_fields.push(self.b.app(enc, *free));
        }
        let arm_env = self.extend_env(env, &arm.binders, &arm_fields)?;
        let arm_scope = Scope {
            env: arm_env,
            ..scope.clone()
        };
        let body_pf = self.prove(&branch.body, &arm.body, &arm_scope)?;
        let scrutinee_encoded = {
            let enc = self.enc(&argument_ty)?;
            self.b.app(enc, constructed)
        };
        let (arms_term, pre_arms) = self.prepare_arms(arms)?;
        let mut chain = self.arm_hit(
            env,
            scrutinee_encoded,
            &pre_arms,
            branch_index,
            &arm_fields,
            arm_env,
            body_pf,
        )?;
        for earlier in (0..branch_index).rev() {
            chain = self.arm_miss(env, scrutinee_encoded, &pre_arms, earlier, chain)?;
        }
        let p = self.prog;
        let number = match scrutinee_expr {
            Expr::Var { name } => *name,
            Expr::Value { .. }
            | Expr::Let { .. }
            | Expr::Cond { .. }
            | Expr::Match { .. }
            | Expr::Build { .. }
            | Expr::Call { .. }
            | Expr::Closure { .. }
            | Expr::Apply { .. }
            | Expr::Prim { .. }
            | Expr::First { .. }
            | Expr::Second { .. }
            | Expr::Field { .. } => {
                return Err("the lowering of a structural scrutinee is not a variable".to_owned())
            }
        };
        let x = self.nat_u(number);
        let tested = self.apply(
            "conv_var",
            &[("p", p), ("env", env), ("x", x), ("v", scrutinee_encoded)],
            &[Given::Refl],
        )?;
        let match_ty_term = self.ty_term(match_ty);
        let matched = self.apply(
            "conv_match",
            &[
                ("p", p),
                ("env", env),
                ("t", match_ty_term),
                ("arms", arms_term),
            ],
            &[
                Given::Proof(tested.proof, tested.ty),
                Given::Proof(chain.proof, chain.ty),
            ],
        )?;
        // The statement at this constructor.
        let to: Vec<Term> = (0..member.sig.rel_params.len())
            .map(|slot_index| {
                if slot_index == member.rel_arg {
                    constructed
                } else {
                    fresh.rel[slot_index].free
                }
            })
            .collect();
        let statement = self
            .b
            .substitute(member.sig.statement, &member.sig.rel_params, &to);
        let (_, parts) = self.b.spine(statement);
        let observation = parts
            .get(3)
            .copied()
            .ok_or("a statement with no observation")?;
        let arguments = parts
            .get(2)
            .copied()
            .ok_or("a statement with no arguments")?;
        let function = self.function_term(&program.functions[member.index as usize])?;
        let fi = self.nat_u(member.index);
        let funrel = self.apply(
            "funRel_intro",
            &[
                ("p", p),
                ("fi", fi),
                ("args", arguments),
                ("o", observation),
                ("fn", function),
                ("env", env),
            ],
            &[
                Given::Refl,
                Given::Refl,
                Given::Proof(matched.proof, matched.ty),
            ],
        )?;
        let fits_term = self.fits_of(matched)?;
        // The recursive calls become the hypotheses (unpacked to the function
        // called) applied to the other arguments of the call.
        let by_field: HashMap<Term, &Hypothesis> = hypotheses
            .iter()
            .map(|hypothesis| (hypothesis.field, hypothesis))
            .collect();
        let called: HashMap<String, (usize, usize, usize, bool)> = {
            let mut out = HashMap::new();
            for other in members {
                let siblings = &at_position[other.plan_position];
                let index = siblings
                    .iter()
                    .position(|slot| members[*slot].index == other.index)
                    .ok_or("a member outside its position")?;
                out.insert(
                    format!("{}.__fits_{}", self.name, other.index),
                    (other.fits_arg, other.plan_position, index, false),
                );
                out.insert(
                    format!("{}.__rel_{}", self.name, other.index),
                    (other.rel_arg, other.plan_position, index, true),
                );
            }
            out
        };
        // The component types of each position, with the free variables that
        // stand for the argument of each function there.
        let mut tys_cache: HashMap<(usize, bool), (Vec<Term>, Vec<Term>)> = HashMap::new();
        for (position, slots) in at_position.iter().enumerate().take(plan.positions.len()) {
            for relation in [false, true] {
                if slots.is_empty() {
                    continue;
                }
                let tys = self.component_types(members, slots, relation)?;
                let frees: Vec<Term> = slots
                    .iter()
                    .map(|slot| {
                        let other = &members[*slot];
                        if relation {
                            other.sig.rel_binders[other.rel_arg].free
                        } else {
                            other.sig.fits_binders[other.fits_arg].free
                        }
                    })
                    .collect();
                tys_cache.insert((position, relation), (tys, frees));
            }
        }
        let replace = |writer: &mut Self, term: Term, relation: bool| -> Term {
            let called = &called;
            let by_field = &by_field;
            let tys_cache = &tys_cache;
            let mut packers: Vec<(Term, Term)> = Vec::new();
            let mut results: HashMap<(Term, usize), Term> = HashMap::new();
            let _ = (&mut packers, &mut results);
            // The unpacking needs the builder, which `rewrite_apps` lends.
            let mut projections: Vec<(Term, usize, usize, bool)> = Vec::new();
            let _ = &mut projections;
            writer.b.rewrite_apps(term, &mut |b, name, args| {
                let &(arg_slot, call_position, index, is_relation) = called.get(name)?;
                if is_relation != relation {
                    return None;
                }
                let field = *args.get(arg_slot)?;
                let hypothesis = by_field.get(&field)?;
                if hypothesis.position != call_position {
                    return None;
                }
                let packed = if relation {
                    hypothesis.rel
                } else {
                    hypothesis.fits
                };
                let (tys, frees) = tys_cache.get(&(call_position, relation))?;
                let at: Vec<Term> = tys
                    .iter()
                    .map(|ty| {
                        let targets = vec![field; frees.len()];
                        b.substitute(*ty, frees, &targets)
                    })
                    .collect();
                let selected = unpack_with(b, &at, index, packed, relation);
                let rest: Vec<Term> = args
                    .iter()
                    .enumerate()
                    .filter(|(position, _)| *position != arg_slot)
                    .map(|(_, arg)| *arg)
                    .collect();
                Some(b.apps(selected, &rest))
            })
        };
        let fits_rec = replace(self, fits_term, false);
        let rel_rec = replace(self, funrel.proof, true);
        // Abstracted over the other arguments, which the motive quantifies.
        let wrap_others = |writer: &mut Self, body: Term, binders: &[Binder], arg_slot: usize| {
            let mut out = body;
            for (index, binder) in binders.iter().enumerate().rev() {
                if index != arg_slot {
                    out = writer
                        .b
                        .lam_over(&binder.name, EXPLICIT, binder.ty, binder.free, out);
                }
            }
            out
        };
        let fits_inner = wrap_others(self, fits_rec, &fresh.fits, member.fits_arg);
        let rel_inner = wrap_others(self, rel_rec, &fresh.rel, member.rel_arg);
        // The direct form: the fresh arguments are the outer ones, and a
        // recursive call is a call of the implementation record.
        let unsafe_map: HashMap<String, String> = members
            .iter()
            .map(|other| {
                (
                    format!("{}.__fits_{}", self.name, other.index),
                    format!("{}.__fits_{}._unsafe_rec", self.name, other.index),
                )
            })
            .collect();
        let direct_body = self.b.rewrite_apps(fits_term, &mut |b, name, args| {
            let target = unsafe_map.get(name)?;
            let head = b.konst(target);
            Some(b.apps(head, args))
        });
        let mut from_fits: Vec<Term> = fresh.fits.iter().map(|b| b.free).collect();
        let mut to_fits: Vec<Term> = member.sig.fits_binders.iter().map(|b| b.free).collect();
        // The argument is the constructor over the fields, not an outer
        // binder.
        from_fits.remove(member.fits_arg);
        to_fits.remove(member.fits_arg);
        let direct_body = self.b.substitute(direct_body, &from_fits, &to_fits);
        let mut direct = direct_body;
        for (index, free) in field_frees.iter().enumerate().rev() {
            direct = self.b.lam_over(
                &format!("__x{index}"),
                EXPLICIT,
                field_types[index],
                *free,
                direct,
            );
        }
        Ok((fits_inner, rel_inner, direct, constructed))
    }
}

/// The component `index` of a packed value of types `tys`, built on the
/// builder alone.
pub(super) fn unpack_with(
    b: &mut crate::production::kernel::Builder,
    tys: &[Term],
    index: usize,
    packed: Term,
    relation: bool,
) -> Term {
    if tys.len() == 1 {
        return packed;
    }
    let rest_ty = pack_types_with(b, &tys[1..], relation);
    let one = CoreLevel::Succ(Box::new(CoreLevel::Zero));
    if index == 0 {
        if relation {
            let head = b.konst("And.left");
            b.apps(head, &[tys[0], rest_ty, packed])
        } else {
            let head = b.konst_at("PProd.fst", vec![one.clone(), one]);
            b.apps(head, &[tys[0], rest_ty, packed])
        }
    } else {
        let rest = if relation {
            let head = b.konst("And.right");
            b.apps(head, &[tys[0], rest_ty, packed])
        } else {
            let head = b.konst_at("PProd.snd", vec![one.clone(), one]);
            b.apps(head, &[tys[0], rest_ty, packed])
        };
        unpack_with(b, &tys[1..], index - 1, rest, relation)
    }
}

/// The pair type of `tys`, built on the builder alone.
pub(super) fn pack_types_with(
    b: &mut crate::production::kernel::Builder,
    tys: &[Term],
    relation: bool,
) -> Term {
    let Some((first, rest)) = tys.split_first() else {
        return b.konst("PUnit");
    };
    if rest.is_empty() {
        return *first;
    }
    let packed = pack_types_with(b, rest, relation);
    if relation {
        let and = b.konst("And");
        b.apps(and, &[*first, packed])
    } else {
        let one = CoreLevel::Succ(Box::new(CoreLevel::Zero));
        let head = b.konst_at("PProd", vec![one.clone(), one]);
        b.apps(head, &[*first, packed])
    }
}

/// The pair of values of types `tys`, built on the builder alone.
pub(super) fn pack_values_with(
    b: &mut crate::production::kernel::Builder,
    tys: &[Term],
    values: &[Term],
    relation: bool,
) -> Term {
    let (first, rest) = (values[0], &values[1..]);
    if rest.is_empty() {
        return first;
    }
    let packed_ty = pack_types_with(b, &tys[1..], relation);
    let packed = pack_values_with(b, &tys[1..], rest, relation);
    if relation {
        let intro = b.konst("And.intro");
        b.apps(intro, &[tys[0], packed_ty, first, packed])
    } else {
        let one = CoreLevel::Succ(Box::new(CoreLevel::Zero));
        let head = b.konst_at("PProd.mk", vec![one.clone(), one]);
        b.apps(head, &[tys[0], packed_ty, first, packed])
    }
}
