//! Functions that recurse structurally (SPEC.md §17.17.4): the width
//! predicate and relation theorem of each are defined by the recursor of the
//! argument's type, its induction hypotheses standing for the recursive calls,
//! and the predicate has an implementation record for the code generator.

use std::collections::{BTreeMap, HashMap};

use super::recplan::RecPlan;
use super::rows::{Binder, LocalSet, Sig};
use super::{LocalTerm, Pf, RowKind, Scope, Writer, EXPLICIT};
use crate::calculus::Expr;
use crate::ir::core::CoreLevel;
use crate::ir::semantic::{SemanticTerm, SemanticType};
use crate::production::kernel::{Given, Term};
use crate::production::source::{Local, Site};
use super::super::{CLocal, Ctx, Gen, Hypotheses, Recursion, Subject};

/// One function of a structurally recursive group.
struct Member<'a> {
    index: u64,
    module: String,
    name: String,
    type_arguments: Vec<SemanticType>,
    result: SemanticType,
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
    set: LocalSet,
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
            let Recursion::Structural { argument, mutual: _ } = recursion else {
                return Err(format!("`{module}.{name}` is not structurally recursive"));
            };
            let clocals = Gen::scope(&locals, "");
            let arg_position = clocals
                .iter()
                .position(|local| local.local.name == argument)
                .ok_or_else(|| format!("`{argument}` is not a parameter of `{name}`"))?;
            let set = self.local_set(*index, &clocals)?;
            let denotation = self.denotation_of(&module, &name, &type_arguments, &set)?;
            let sig = self.signature(*index, &clocals, &set, &result, denotation)?;
            self.sigs.insert(*index, sig.clone());
            let width = |local: &CLocal, rel: bool| -> usize {
                if local.companion.is_some() {
                    if rel {
                        5
                    } else {
                        2
                    }
                } else {
                    1
                }
            };
            let fits_arg = clocals[..arg_position].iter().map(|l| width(l, false)).sum();
            let rel_arg = clocals[..arg_position].iter().map(|l| width(l, true)).sum();
            members.push(Member {
                index: *index,
                module,
                name,
                type_arguments,
                result,
                body,
                site,
                type_parameters,
                argument,
                locals: clocals,
                arg_position,
                plan_position: 0,
                fits_arg,
                rel_arg,
                set,
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
        let mut at_position: Vec<Option<usize>> = vec![None; plan.positions.len()];
        for (slot, member) in members.iter().enumerate() {
            if at_position[member.plan_position].replace(slot).is_some() {
                return Err(format!(
                    "two functions of a group recurse on the same type (`{}`)",
                    member.name
                ));
            }
        }
        let one = CoreLevel::Succ(Box::new(CoreLevel::Zero));
        // The motives: a function of the argument into the statement for every
        // other argument.
        let bool = self.b.konst("Bool");
        let mut fits_motives = Vec::new();
        let mut rel_motives = Vec::new();
        let mut domains = Vec::new();
        for position in 0..plan.positions.len() {
            let domain = self.lean_ty(&plan.positions[position].ty)?;
            domains.push(domain);
            let member = &members[at_position[position]
                .ok_or("a nested type of the group has no function recursing on it")?];
            let others_fits: Vec<Binder> = member
                .sig
                .fits_binders
                .iter()
                .enumerate()
                .filter(|(slot, _)| *slot != member.fits_arg)
                .map(|(_, binder)| binder.clone())
                .collect();
            let others_rel: Vec<Binder> = member
                .sig
                .rel_binders
                .iter()
                .enumerate()
                .filter(|(slot, _)| *slot != member.rel_arg)
                .map(|(_, binder)| binder.clone())
                .collect();
            let fits_body = self.pis(&others_fits, bool);
            let rel_body = self.pis(&others_rel, member.sig.statement);
            // The motive is a function of the argument: its body is closed
            // over the member's own argument binder, which it replaces.
            let arg_free = member.sig.fits_binders[member.fits_arg].free;
            let fits_lifted = self.b.lam_over("__v", EXPLICIT, domain, arg_free, fits_body);
            let rel_arg_free = member.sig.rel_binders[member.rel_arg].free;
            let rel_lifted = self.b.lam_over("__v", EXPLICIT, domain, rel_arg_free, rel_body);
            fits_motives.push(fits_lifted);
            rel_motives.push(rel_lifted);
        }
        // The minor premises, position by position, constructor by
        // constructor.
        let mut fits_minors = Vec::new();
        let mut rel_minors = Vec::new();
        let mut direct_minors: Vec<Vec<Term>> = vec![Vec::new(); plan.positions.len()];
        for position in 0..plan.positions.len() {
            let slot = at_position[position].ok_or("a position without a function")?;
            for ctor in 0..plan.positions[position].constructors.len() {
                let (fits_minor, rel_minor, direct) =
                    self.structural_minor(&plan, &members, &at_position, slot, ctor)?;
                fits_minors.push(fits_minor);
                rel_minors.push(rel_minor);
                direct_minors[position].push(direct);
            }
        }
        let mut params = Vec::new();
        for argument in &plan.params {
            params.push(self.lean_ty(argument)?);
        }
        // The rows of each member.
        for member in &members {
            let position = member.plan_position;
            let recursor = self
                .b
                .konst_at(&plan.recursors[position], plan.levels(one.clone()));
            // The predicate.
            let mut fits_args = params.clone();
            fits_args.extend(fits_motives.iter().copied());
            fits_args.extend(fits_minors.iter().copied());
            fits_args.push(member.sig.fits_binders[member.fits_arg].free);
            let applied = self.b.apps(recursor, &fits_args);
            let others: Vec<Term> = member
                .sig
                .fits_binders
                .iter()
                .enumerate()
                .filter(|(slot, _)| *slot != member.fits_arg)
                .map(|(_, binder)| binder.free)
                .collect();
            let body = self.b.apps(applied, &others);
            let value = self.lambdas(&member.sig.fits_binders, body);
            let row_ty = self.pis(&member.sig.fits_binders, bool);
            self.push_row(
                &format!("__fits_{}", member.index),
                RowKind::Definition { compiled: true },
                row_ty,
                value,
            )?;
        }
        for member in &members {
            let position = member.plan_position;
            let recursor = self
                .b
                .konst_at(&plan.recursors[position], plan.levels(CoreLevel::Zero));
            let mut rel_args = params.clone();
            rel_args.extend(rel_motives.iter().copied());
            rel_args.extend(rel_minors.iter().copied());
            rel_args.push(member.sig.rel_binders[member.rel_arg].free);
            let applied = self.b.apps(recursor, &rel_args);
            let others: Vec<Term> = member
                .sig
                .rel_binders
                .iter()
                .enumerate()
                .filter(|(slot, _)| *slot != member.rel_arg)
                .map(|(_, binder)| binder.free)
                .collect();
            let body = self.b.apps(applied, &others);
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
        for member in &members {
            let position = member.plan_position;
            let discriminant = member.sig.fits_binders[member.fits_arg].free;
            let ty = plan.positions[position].ty.clone();
            let minors = direct_minors[position].clone();
            let cases = self.cases_on_fresh_motive(&ty, discriminant, bool, &minors)?;
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

    /// `T.casesOn` at the constant motive `result` whose minors are the
    /// functions of the constructor's fields.
    fn cases_on_fresh_motive(
        &mut self,
        ty: &SemanticType,
        discriminant: Term,
        result: Term,
        minors: &[Term],
    ) -> Result<Term, String> {
        self.cases_on_const(ty, discriminant, result, minors)
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

    /// The minor premises of constructor `ctor` of the position function
    /// `slot` recurses on: for the width predicate, for the relation, and its
    /// direct form for the implementation record.
    #[allow(clippy::too_many_lines)]
    fn structural_minor(
        &mut self,
        plan: &RecPlan,
        members: &[Member<'a>],
        at_position: &[Option<usize>],
        slot: usize,
        ctor: usize,
    ) -> Result<(Term, Term, Term), String> {
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
            _ => {
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
            let (constructor, fields) = self.gen.source.branch(branch, &argument_ty, &member.site)?;
            let position = self.constructor_position(&constructor)?;
            if position == ctor {
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
        // Free variables: the fields, the hypotheses, and the other arguments.
        let mut field_frees = Vec::new();
        let mut field_types = Vec::new();
        for ty in &shape.fields {
            field_frees.push(self.b.free());
            field_types.push(self.lean_ty(ty)?);
        }
        let mut hypotheses: Vec<Hypothesis> = Vec::new();
        let mut hypothesis_info: Vec<(Term, Term, Term, Term)> = Vec::new();
        for (field_index, recursion) in shape.recursive.iter().enumerate() {
            if let Some(target) = recursion {
                let target_member = &members[at_position[*target]
                    .ok_or("a recursive field whose type has no function")?];
                let fits = self.b.free();
                let rel = self.b.free();
                hypotheses.push(Hypothesis {
                    field: field_frees[field_index],
                    fits,
                    rel,
                    position: *target,
                });
                // Their types: the motive of the target at the field.
                let others_fits: Vec<Binder> = target_member
                    .sig
                    .fits_binders
                    .iter()
                    .enumerate()
                    .filter(|(slot, _)| *slot != target_member.fits_arg)
                    .map(|(_, binder)| binder.clone())
                    .collect();
                let others_rel: Vec<Binder> = target_member
                    .sig
                    .rel_binders
                    .iter()
                    .enumerate()
                    .filter(|(slot, _)| *slot != target_member.rel_arg)
                    .map(|(_, binder)| binder.clone())
                    .collect();
                let bool = self.b.konst("Bool");
                let fits_body = self.pis(&others_fits, bool);
                let rel_body = self.pis(&others_rel, target_member.sig.statement);
                let at_fits = target_member.sig.fits_binders[target_member.fits_arg].free;
                let at_rel = target_member.sig.rel_binders[target_member.rel_arg].free;
                let fits_ty = self
                    .b
                    .substitute(fits_body, &[at_fits], &[field_frees[field_index]]);
                let rel_ty = self
                    .b
                    .substitute(rel_body, &[at_rel], &[field_frees[field_index]]);
                hypothesis_info.push((fits, rel, fits_ty, rel_ty));
            }
        }
        // The other arguments of this minor are fresh.
        let fresh = self.local_set(member.index, &member.locals)?;
        // The value of the constructor on the fields.
        let constructed = self.constructor_value_of(
            &constructor,
            &argument_ty,
            &branch.constructor,
            &member.site,
            &field_frees,
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
            .zip(&field_frees)
        {
            ctx_locals.push(CLocal {
                local: Local {
                    name: name.clone(),
                    ty,
                },
                generic,
                companion: None,
            });
            terms.push(LocalTerm {
                value: *free,
                companion: None,
            });
        }
        // The environment: the parameters bound to the encodings.
        let parameters = self.gen.program.functions[member.index as usize]
            .parameters
            .clone();
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
        };
        let program = self.gen.program;
        let top = &program.functions[member.index as usize].body;
        let (match_ty, scrutinee_expr, arms) = match top {
            Expr::Match {
                ty,
                scrutinee,
                arms,
            } => (ty, scrutinee.as_ref(), arms.as_slice()),
            _ => return Err("the lowering of a structural body is not a match".to_owned()),
        };
        let arm = arms
            .get(branch_index)
            .ok_or("a structural match lowered to fewer arms")?;
        let arm_fields: Vec<Term> = {
            let mut out = Vec::new();
            for (field_ty, free) in shape.fields.iter().zip(&field_frees) {
                let enc = self.enc(field_ty)?;
                out.push(self.b.app(enc, *free));
            }
            out
        };
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
        let mut chain = self.arm_hit(
            env,
            scrutinee_encoded,
            arms,
            branch_index,
            &arm_fields,
            arm_env,
            body_pf,
        )?;
        for earlier in (0..branch_index).rev() {
            chain = self.arm_miss(env, scrutinee_encoded, arms, earlier, chain)?;
        }
        let (p, outer) = (self.prog, env);
        let number = match scrutinee_expr {
            Expr::Var { name } => *name,
            _ => return Err("the lowering of a structural scrutinee is not a variable".to_owned()),
        };
        let x = self.nat_u(number);
        let tested = self.apply(
            "conv_var",
            &[("p", p), ("env", outer), ("x", x), ("v", scrutinee_encoded)],
            &[Given::Refl],
        )?;
        let arms_term = self.arms_list(arms)?;
        let match_ty_term = self.ty_term(match_ty);
        let matched = self.apply(
            "conv_match",
            &[("p", p), ("env", outer), ("t", match_ty_term), ("arms", arms_term)],
            &[
                Given::Proof(tested.proof, tested.ty),
                Given::Proof(chain.proof, chain.ty),
            ],
        )?;
        // The statement at this constructor.
        let mut from: Vec<Term> = member.sig.rel_params.clone();
        let mut to: Vec<Term> = Vec::new();
        for (slot_index, param) in member.sig.rel_params.iter().enumerate() {
            let _ = param;
            if slot_index == member.rel_arg {
                to.push(constructed);
            } else {
                to.push(fresh.rel[slot_index].free);
            }
        }
        // `from` and `to` pair the outer binders with this minor's.
        let _ = &mut from;
        let statement = self.b.substitute(member.sig.statement, &from, &to);
        let (_, parts) = self.b.spine(statement);
        let observation = parts.get(3).copied().ok_or("a statement with no observation")?;
        let arguments = parts.get(2).copied().ok_or("a statement with no arguments")?;
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
        // Replace the recursive calls by the hypotheses.
        let mut fits_calls: HashMap<usize, (usize, usize)> = HashMap::new();
        for (slot_index, other) in members.iter().enumerate() {
            fits_calls.insert(other.index as usize, (slot_index, other.fits_arg));
        }
        let hypothesis_fits: HashMap<Term, Term> = hypotheses
            .iter()
            .map(|hypothesis| (hypothesis.field, hypothesis.fits))
            .collect();
        let hypothesis_rel: HashMap<Term, Term> = hypotheses
            .iter()
            .map(|hypothesis| (hypothesis.field, hypothesis.rel))
            .collect();
        let _ = hypothesis_info.len();
        let fits_names: HashMap<String, usize> = members
            .iter()
            .map(|other| (format!("{}.__fits_{}", self.name, other.index), other.fits_arg))
            .collect();
        let rel_names: HashMap<String, usize> = members
            .iter()
            .map(|other| (format!("{}.__rel_{}", self.name, other.index), other.rel_arg))
            .collect();
        let unsafe_map: HashMap<String, Term> = members
            .iter()
            .map(|other| {
                (
                    format!("{}.__fits_{}", self.name, other.index),
                    self.b.konst(&format!(
                        "{}.__fits_{}._unsafe_rec",
                        self.name, other.index
                    )),
                )
            })
            .collect();
        let rewrite_with = |b: &mut crate::production::kernel::Builder,
                            names: &HashMap<String, usize>,
                            by_field: &HashMap<Term, Term>,
                            name: &str,
                            args: &[Term]|
         -> Option<Term> {
            let &arg_slot = names.get(name)?;
            let field = *args.get(arg_slot)?;
            let hypothesis = *by_field.get(&field)?;
            let rest: Vec<Term> = args
                .iter()
                .enumerate()
                .filter(|(position, _)| *position != arg_slot)
                .map(|(_, arg)| *arg)
                .collect();
            Some(b.apps(hypothesis, &rest))
        };
        let fits_rec = self.b.rewrite_apps(fits_term, &mut |b, name, args| {
            rewrite_with(b, &fits_names, &hypothesis_fits, name, args)
        });
        let rel_rec = self.b.rewrite_apps(funrel.proof, &mut |b, name, args| {
            rewrite_with(b, &rel_names, &hypothesis_rel, name, args)
        });
        let others_fits_fresh: Vec<Term> = fresh
            .fits
            .iter()
            .enumerate()
            .filter(|(slot_index, _)| *slot_index != member.fits_arg)
            .map(|(_, binder)| binder.free)
            .collect();
        let others_rel_fresh: Vec<Term> = fresh
            .rel
            .iter()
            .enumerate()
            .filter(|(slot_index, _)| *slot_index != member.rel_arg)
            .map(|(_, binder)| binder.free)
            .collect();
        let _ = (others_fits_fresh, others_rel_fresh);
        // The minors: fields, then hypotheses, then the other arguments.
        let wrap = |writer: &mut Self,
                    body: Term,
                    binders: &[Binder],
                    arg_slot: usize,
                    ihs: &[(Term, Term)]|
         -> Term {
            let mut out = body;
            for (index, binder) in binders.iter().enumerate().rev() {
                if index != arg_slot {
                    out = writer
                        .b
                        .lam_over(&binder.name, EXPLICIT, binder.ty, binder.free, out);
                }
            }
            for (free, ty) in ihs.iter().rev() {
                out = writer.b.lam_over("__ih", EXPLICIT, *ty, *free, out);
            }
            for (index, free) in field_frees.iter().enumerate().rev() {
                out = writer
                    .b
                    .lam_over(&format!("__x{index}"), EXPLICIT, field_types[index], *free, out);
            }
            out
        };
        let fits_ihs: Vec<(Term, Term)> = hypothesis_info
            .iter()
            .map(|(fits, _, fits_ty, _)| (*fits, *fits_ty))
            .collect();
        let rel_ihs: Vec<(Term, Term)> = hypothesis_info
            .iter()
            .map(|(_, rel, _, rel_ty)| (*rel, *rel_ty))
            .collect();
        let fits_minor = wrap(self, fits_rec, &fresh.fits, member.fits_arg, &fits_ihs);
        let rel_minor = wrap(self, rel_rec, &fresh.rel, member.rel_arg, &rel_ihs);
        // The direct form: the fresh arguments are the outer ones, and a
        // recursive call is a call of the implementation record.
        let direct_body = self.b.rewrite_apps(fits_term, &mut |b, name, args| {
            let target = unsafe_map.get(name)?;
            Some(b.apps(*target, args))
        });
        let outer_fits: Vec<Term> = member.sig.fits_binders.iter().map(|b| b.free).collect();
        let fresh_fits: Vec<Term> = fresh.fits.iter().map(|b| b.free).collect();
        let mut from_fits = fresh_fits.clone();
        let mut to_fits = outer_fits.clone();
        // The argument is the constructor over the fields, not an outer
        // binder.
        from_fits.remove(member.fits_arg);
        to_fits.remove(member.fits_arg);
        let direct_body = self.b.substitute(direct_body, &from_fits, &to_fits);
        let mut direct = direct_body;
        for (index, free) in field_frees.iter().enumerate().rev() {
            direct = self
                .b
                .lam_over(&format!("__x{index}"), EXPLICIT, field_types[index], *free, direct);
        }
        Ok((fits_minor, rel_minor, direct))
    }

    /// The value of a constructor on the free variables of its fields.
    fn constructor_value_of(
        &mut self,
        constructor: &crate::production::source::Constructor,
        ty: &SemanticType,
        member: &crate::ir::semantic::MemberRef,
        site: &Site,
        frees: &[Term],
    ) -> Result<Term, String> {
        self.constructed(constructor, ty, member, site, frees)
    }
}
