//! Calls, function values, and applications (SPEC.md §17.17.4): the relation
//! of a callee applied to the arguments' denotations, and a function-typed
//! argument carried with its companions, the width predicate, closure index,
//! captured values, and relation of the closure it is bound to.

use super::prove::Passed;
use super::{CompTerms, Pf, Scope, Writer, EXPLICIT};
use crate::calculus::Expr;
use crate::ir::semantic::{SemanticTerm, SemanticType};
use crate::production::kernel::{Given, Term};
use crate::production::source::Source;

/// A function-typed argument with its companions.
#[derive(Debug, Clone, Copy)]
pub(super) struct FunctionArgument {
    pub(super) value: Term,
    pub(super) fits: Term,
    pub(super) index: Term,
    pub(super) captures: Term,
    pub(super) relation: Term,
    pub(super) proof: Pf,
    /// The function the program numbered it (unused for a parameter).
    pub(super) number: u64,
}

/// The parameter and result types of a closed function type.
fn function_parts(ty: &SemanticType) -> Option<(Vec<SemanticType>, SemanticType)> {
    match ty {
        SemanticType::Function { parameters, result } => {
            Some((parameters.clone(), result.as_ref().clone()))
        }
        SemanticType::Type
        | SemanticType::Parameter { name: _ }
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
        | SemanticType::Option { value: _ }
        | SemanticType::Result { ok: _, error: _ }
        | SemanticType::List { element: _ }
        | SemanticType::Named {
            member: _,
            arguments: _,
        }
        | SemanticType::Product { left: _, right: _ }
        | SemanticType::Map { key: _, value: _ }
        | SemanticType::Set { element: _ }
        | SemanticType::ContractViolation
        | SemanticType::ReasoningFailure => None,
    }
}

impl<'a> Writer<'_, 'a> {
    /// The relation theorem `__rel_index` applied to `args`, with the type it
    /// states there: the statement the function's signature fixes, written
    /// at the arguments.
    pub(super) fn call_relation(&mut self, index: u64, args: &[Term]) -> Result<Pf, String> {
        let (params, statement) = {
            let sig = self
                .sigs
                .get(&index)
                .ok_or_else(|| format!("function {index} has no signature yet"))?;
            (sig.rel_params.clone(), sig.statement)
        };
        if params.len() != args.len() {
            return Err(format!(
                "function {index}'s relation takes {} arguments, given {}",
                params.len(),
                args.len()
            ));
        }
        let head = self.own(&format!("__rel_{index}"));
        let proof = self.b.apps(head, args);
        let ty = self.b.substitute(statement, &params, args);
        Ok(Pf { proof, ty })
    }

    /// The arguments a callee's relation and width predicate take for one
    /// source argument: the value, and for a function value its companions.
    pub(super) fn argument(
        &mut self,
        argument: &SemanticTerm,
        expr: &'a Expr,
        scope: &Scope,
    ) -> Result<Passed, String> {
        match self.function_argument(argument, expr, scope)? {
            Some(function) => Ok(Passed {
                fits: vec![function.value, function.fits],
                relation: vec![
                    function.value,
                    function.fits,
                    function.index,
                    function.captures,
                    function.relation,
                ],
                proof: function.proof,
            }),
            None => {
                let proof = self.prove(argument, expr, scope)?;
                let value = self.src(argument, scope)?;
                Ok(Passed {
                    fits: vec![value],
                    relation: vec![value],
                    proof,
                })
            }
        }
    }

    /// `fun (p0 : T0) … => body`, over the parameters of a function type,
    /// where `body` is built from their free variables.
    fn over_parameters(
        &mut self,
        parameters: &[SemanticType],
        body: impl FnOnce(&mut Self, &[Term]) -> Result<Term, String>,
    ) -> Result<Term, String> {
        let mut frees = Vec::new();
        let mut types = Vec::new();
        for parameter in parameters {
            frees.push(self.b.free());
            types.push(self.lean_ty(parameter)?);
        }
        let mut out = body(self, &frees)?;
        for position in (0..frees.len()).rev() {
            out = self.b.lam_over(
                &format!("__p{position}"),
                EXPLICIT,
                types[position],
                frees[position],
                out,
            );
        }
        Ok(out)
    }

    /// The companions of a function value at the lambda or function the
    /// program numbered `index`, closing over `captures`.
    fn closure_companions(
        &mut self,
        index: u64,
        parameters: &[SemanticType],
        fits_captures: Vec<Term>,
        relation_captures: Vec<Term>,
    ) -> Result<(Term, Term), String> {
        let fits_head = self.own(&format!("__fits_{index}"));
        let rel_head = self.own(&format!("__rel_{index}"));
        let fits = self.over_parameters(parameters, |writer, frees| {
            let mut all = fits_captures.clone();
            all.extend_from_slice(frees);
            Ok(writer.b.apps(fits_head, &all))
        })?;
        let relation = self.over_parameters(parameters, |writer, frees| {
            let mut all = relation_captures.clone();
            all.extend_from_slice(frees);
            Ok(writer.b.apps(rel_head, &all))
        })?;
        Ok((fits, relation))
    }

    /// A function-typed argument's value and companions; `None` for a
    /// first-order argument.
    pub(super) fn function_argument(
        &mut self,
        argument: &SemanticTerm,
        expr: &'a Expr,
        scope: &Scope,
    ) -> Result<Option<FunctionArgument>, String> {
        let ty = self.infer(argument, scope)?;
        let Some((parameters, _)) = function_parts(&ty) else {
            return Ok(None);
        };
        let (p, env) = (self.prog, scope.env);
        match argument {
            SemanticTerm::Var { name } => {
                let number = match expr {
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
                        return Err(format!("the lowering of `{name}` is not a variable"))
                    }
                };
                let (_, terms) = scope.lookup(name)?;
                let companion = terms
                    .companion
                    .ok_or_else(|| format!("function-typed `{name}` has no companion"))?;
                let closure = self.ts("Value.closure");
                let value = self.b.apps(closure, &[companion.index, companion.captures]);
                let x = self.nat_u(number);
                let proof = self.apply(
                    "conv_var",
                    &[("p", p), ("env", env), ("x", x), ("v", value)],
                    &[Given::Refl],
                )?;
                Ok(Some(FunctionArgument {
                    value: terms.value,
                    fits: companion.fits,
                    index: companion.index,
                    captures: companion.captures,
                    relation: companion.relation,
                    proof,
                    number: 0,
                }))
            }
            SemanticTerm::Lambda {
                parameters: _,
                captures,
                body: _,
            } => {
                let (function, capture_exprs) = match expr {
                    Expr::Closure { function, captures } => (*function, captures),
                    Expr::Value { .. }
                    | Expr::Var { .. }
                    | Expr::Let { .. }
                    | Expr::Cond { .. }
                    | Expr::Match { .. }
                    | Expr::Build { .. }
                    | Expr::Call { .. }
                    | Expr::Apply { .. }
                    | Expr::Prim { .. }
                    | Expr::First { .. }
                    | Expr::Second { .. }
                    | Expr::Field { .. } => {
                        return Err("the lowering of a lambda is not a closure".to_owned())
                    }
                };
                let index = self.gen.next_lambda(scope.ctx.function)?;
                if index != function {
                    return Err(format!(
                        "a lambda numbered {index} lowered to the closure of {function}"
                    ));
                }
                let mut fits_captures = Vec::new();
                let mut relation_captures = Vec::new();
                let mut encoded = Vec::new();
                let mut proofs = Vec::new();
                for (capture, capture_expr) in captures.iter().zip(capture_exprs) {
                    let (local, terms) = scope.lookup(capture)?;
                    let local = local.clone();
                    match terms.companion {
                        Some(companion) => {
                            fits_captures.push(terms.value);
                            fits_captures.push(companion.fits);
                            relation_captures.extend([
                                terms.value,
                                companion.fits,
                                companion.index,
                                companion.captures,
                                companion.relation,
                            ]);
                        }
                        None => {
                            fits_captures.push(terms.value);
                            relation_captures.push(terms.value);
                        }
                    }
                    encoded.push(self.encoded_local(&local, terms)?);
                    let number = match capture_expr {
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
                            return Err("a capture lowered to other than a variable".to_owned())
                        }
                    };
                    let value = *encoded.last().ok_or("a capture")?;
                    let x = self.nat_u(number);
                    proofs.push(self.apply(
                        "conv_var",
                        &[("p", p), ("env", env), ("x", x), ("v", value)],
                        &[Given::Refl],
                    )?);
                }
                let list = self.operands(env, &proofs)?;
                let fi = self.nat_u(index);
                let proof = self.apply(
                    "conv_closure",
                    &[("p", p), ("env", env), ("fi", fi)],
                    &[Given::Proof(list.proof, list.ty)],
                )?;
                let (fits, relation) =
                    self.closure_companions(index, &parameters, fits_captures, relation_captures)?;
                let value_ty = self.value_ty();
                let captures_term = self.b.list(value_ty, &encoded);
                let value = self.src(argument, scope)?;
                Ok(Some(FunctionArgument {
                    value,
                    fits,
                    index: fi,
                    captures: captures_term,
                    relation,
                    proof,
                    number: index,
                }))
            }
            SemanticTerm::FunctionRef {
                function,
                type_arguments,
            } => {
                let module = Source::module_of(function, &scope.ctx.site);
                let closed: Vec<SemanticType> = type_arguments
                    .iter()
                    .map(|argument| self.gen.source.close(argument, &scope.ctx.site))
                    .collect();
                let index = self.gen.instance_index(&module, &function.name, &closed)?;
                let list = self.operands(env, &[])?;
                let fi = self.nat_u(index);
                let proof = self.apply(
                    "conv_closure",
                    &[("p", p), ("env", env), ("fi", fi)],
                    &[Given::Proof(list.proof, list.ty)],
                )?;
                let (fits, relation) =
                    self.closure_companions(index, &parameters, Vec::new(), Vec::new())?;
                let value_ty = self.value_ty();
                let captures_term = self.b.list(value_ty, &[]);
                let value = self.src(argument, scope)?;
                Ok(Some(FunctionArgument {
                    value,
                    fits,
                    index: fi,
                    captures: captures_term,
                    relation,
                    proof,
                    number: index,
                }))
            }
            SemanticTerm::Nat { value: _ }
            | SemanticTerm::Integer {
                representation: _,
                value: _,
            }
            | SemanticTerm::String { value: _ }
            | SemanticTerm::Bytes { hex: _ }
            | SemanticTerm::Primitive {
                operation: _,
                arguments: _,
                result: _,
            }
            | SemanticTerm::Bool { value: _ }
            | SemanticTerm::Unit
            | SemanticTerm::Nil { element: _ }
            | SemanticTerm::Cons { head: _, tail: _ }
            | SemanticTerm::Record {
                r#type: _,
                type_arguments: _,
                fields: _,
            }
            | SemanticTerm::Constructor {
                constructor: _,
                type_arguments: _,
                arguments: _,
            }
            | SemanticTerm::InstanceValue {
                class: _,
                arguments: _,
                resolved: _,
            }
            | SemanticTerm::Project { value: _, field: _ }
            | SemanticTerm::Call {
                function: _,
                type_arguments: _,
                arguments: _,
            }
            | SemanticTerm::If {
                condition: _,
                then_value: _,
                else_value: _,
            }
            | SemanticTerm::Match {
                scrutinee: _,
                branches: _,
            }
            | SemanticTerm::Eq { left: _, right: _ }
            | SemanticTerm::Le { left: _, right: _ }
            | SemanticTerm::Lt { left: _, right: _ }
            | SemanticTerm::Add { left: _, right: _ }
            | SemanticTerm::Beq { left: _, right: _ }
            | SemanticTerm::Ble { left: _, right: _ }
            | SemanticTerm::Blt { left: _, right: _ }
            | SemanticTerm::And { left: _, right: _ }
            | SemanticTerm::PropAnd { left: _, right: _ }
            | SemanticTerm::Or { left: _, right: _ }
            | SemanticTerm::Not { value: _ }
            | SemanticTerm::Implies {
                premise: _,
                conclusion: _,
            }
            | SemanticTerm::Iff { left: _, right: _ }
            | SemanticTerm::Forall { binder: _, body: _ }
            | SemanticTerm::Let {
                binder: _,
                value: _,
                body: _,
            }
            | SemanticTerm::Pair { left: _, right: _ }
            | SemanticTerm::First { value: _ }
            | SemanticTerm::Second { value: _ }
            | SemanticTerm::Apply {
                function: _,
                arguments: _,
            }
            | SemanticTerm::MapLiteral {
                key: _,
                value: _,
                entries: _,
            }
            | SemanticTerm::SetLiteral {
                element: _,
                elements: _,
            }
            | SemanticTerm::GraphLiteral {
                node: _,
                nodes: _,
                edges: _,
            }
            | SemanticTerm::CheckedApply {
                model: _,
                type_arguments: _,
                arguments: _,
                checks: _,
            } => Err(format!(
                "a function value `{}` is neither a parameter, a lambda, nor a function reference",
                super::super::super::eligibility::term_key(argument)
            )),
        }
    }

    /// The proof of applying a function value to arguments.
    pub(super) fn application(
        &mut self,
        function: &SemanticTerm,
        arguments: &[SemanticTerm],
        expr: &'a Expr,
        scope: &Scope,
    ) -> Result<Pf, String> {
        let (target_expr, operand_exprs) = match expr {
            Expr::Apply { target, operands } => (target.as_ref(), operands.as_slice()),
            Expr::Value { .. }
            | Expr::Var { .. }
            | Expr::Let { .. }
            | Expr::Cond { .. }
            | Expr::Match { .. }
            | Expr::Build { .. }
            | Expr::Call { .. }
            | Expr::Closure { .. }
            | Expr::Prim { .. }
            | Expr::First { .. }
            | Expr::Second { .. }
            | Expr::Field { .. } => {
                return Err("the lowering of an application is not an apply".to_owned())
            }
        };
        let (p, env) = (self.prog, scope.env);
        let mut srcs = Vec::new();
        for argument in arguments {
            srcs.push(self.src(argument, scope)?);
        }
        let ty = self.infer(function, scope)?;
        let (parameters, _) = function_parts(&ty).ok_or("an applied non-function")?;
        let (target, relation) = match function {
            SemanticTerm::Var { name } => {
                let number = match target_expr {
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
                        return Err(format!("the lowering of `{name}` is not a variable"))
                    }
                };
                let (_, terms) = scope.lookup(name)?;
                let companion = terms
                    .companion
                    .ok_or_else(|| format!("applied `{name}` has no companion"))?;
                let closure = self.ts("Value.closure");
                let value = self.b.apps(closure, &[companion.index, companion.captures]);
                let x = self.nat_u(number);
                let target = self.apply(
                    "conv_var",
                    &[("p", p), ("env", env), ("x", x), ("v", value)],
                    &[Given::Refl],
                )?;
                let relation = self.relation_applied(companion, &srcs)?;
                (target, relation)
            }
            SemanticTerm::Lambda { .. } | SemanticTerm::FunctionRef { .. } => {
                let passed = self
                    .function_argument(function, target_expr, scope)?
                    .ok_or("a lambda is not a function value")?;
                let callee = passed.number;
                let _ = parameters;
                let mut args = Vec::new();
                args.extend(self.captured_relation_args(function, scope)?);
                args.extend(srcs.iter().copied());
                let relation = self.call_relation(callee, &args)?;
                (passed.proof, relation)
            }
            SemanticTerm::Nat { value: _ }
            | SemanticTerm::Integer {
                representation: _,
                value: _,
            }
            | SemanticTerm::String { value: _ }
            | SemanticTerm::Bytes { hex: _ }
            | SemanticTerm::Primitive {
                operation: _,
                arguments: _,
                result: _,
            }
            | SemanticTerm::Bool { value: _ }
            | SemanticTerm::Unit
            | SemanticTerm::Nil { element: _ }
            | SemanticTerm::Cons { head: _, tail: _ }
            | SemanticTerm::Record {
                r#type: _,
                type_arguments: _,
                fields: _,
            }
            | SemanticTerm::Constructor {
                constructor: _,
                type_arguments: _,
                arguments: _,
            }
            | SemanticTerm::InstanceValue {
                class: _,
                arguments: _,
                resolved: _,
            }
            | SemanticTerm::Project { value: _, field: _ }
            | SemanticTerm::Call {
                function: _,
                type_arguments: _,
                arguments: _,
            }
            | SemanticTerm::If {
                condition: _,
                then_value: _,
                else_value: _,
            }
            | SemanticTerm::Match {
                scrutinee: _,
                branches: _,
            }
            | SemanticTerm::Eq { left: _, right: _ }
            | SemanticTerm::Le { left: _, right: _ }
            | SemanticTerm::Lt { left: _, right: _ }
            | SemanticTerm::Add { left: _, right: _ }
            | SemanticTerm::Beq { left: _, right: _ }
            | SemanticTerm::Ble { left: _, right: _ }
            | SemanticTerm::Blt { left: _, right: _ }
            | SemanticTerm::And { left: _, right: _ }
            | SemanticTerm::PropAnd { left: _, right: _ }
            | SemanticTerm::Or { left: _, right: _ }
            | SemanticTerm::Not { value: _ }
            | SemanticTerm::Implies {
                premise: _,
                conclusion: _,
            }
            | SemanticTerm::Iff { left: _, right: _ }
            | SemanticTerm::Forall { binder: _, body: _ }
            | SemanticTerm::Let {
                binder: _,
                value: _,
                body: _,
            }
            | SemanticTerm::Pair { left: _, right: _ }
            | SemanticTerm::First { value: _ }
            | SemanticTerm::Second { value: _ }
            | SemanticTerm::Apply {
                function: _,
                arguments: _,
            }
            | SemanticTerm::MapLiteral {
                key: _,
                value: _,
                entries: _,
            }
            | SemanticTerm::SetLiteral {
                element: _,
                elements: _,
            }
            | SemanticTerm::GraphLiteral {
                node: _,
                nodes: _,
                edges: _,
            }
            | SemanticTerm::CheckedApply {
                model: _,
                type_arguments: _,
                arguments: _,
                checks: _,
            } => {
                return Err(format!(
                    "the applied `{}` is neither a parameter, a lambda, nor a function reference",
                    super::super::super::eligibility::term_key(function)
                ));
            }
        };
        let mut proofs = Vec::new();
        for (argument, child) in arguments.iter().zip(operand_exprs) {
            proofs.push(self.prove(argument, child, scope)?);
        }
        let list = self.operands(env, &proofs)?;
        self.apply(
            "conv_apply",
            &[("p", p), ("env", env)],
            &[
                Given::Proof(target.proof, target.ty),
                Given::Proof(list.proof, list.ty),
                Given::Proof(relation.proof, relation.ty),
            ],
        )
    }

    /// A function-typed local's relation hypothesis applied to arguments,
    /// with the type it states there.
    fn relation_applied(&mut self, companion: CompTerms, args: &[Term]) -> Result<Pf, String> {
        let mut term = companion.relation;
        let mut ty = companion.relation_ty;
        for arg in args {
            let (_, body) = match self.b.node(ty) {
                crate::ir::core::CoreNode::Forall { t, v, .. } => (*t, *v),
                crate::ir::core::CoreNode::BVar { .. }
                | crate::ir::core::CoreNode::Sort { .. }
                | crate::ir::core::CoreNode::Const { .. }
                | crate::ir::core::CoreNode::App { .. }
                | crate::ir::core::CoreNode::Lambda { .. }
                | crate::ir::core::CoreNode::Let { .. }
                | crate::ir::core::CoreNode::Nat { .. }
                | crate::ir::core::CoreNode::String { .. }
                | crate::ir::core::CoreNode::Mdata { .. }
                | crate::ir::core::CoreNode::Projection { .. } => {
                    return Err("a relation hypothesis applied past its binders".to_owned())
                }
            };
            term = self.b.app(term, *arg);
            ty = self.b.instantiate_beta(body, *arg);
        }
        Ok(Pf { proof: term, ty })
    }

    /// The captured arguments a lambda's relation takes before its
    /// parameters.
    fn captured_relation_args(
        &mut self,
        function: &SemanticTerm,
        scope: &Scope,
    ) -> Result<Vec<Term>, String> {
        match function {
            SemanticTerm::Lambda { captures, .. } => {
                let mut out = Vec::new();
                for capture in captures {
                    let (_, terms) = scope.lookup(capture)?;
                    match terms.companion {
                        Some(companion) => out.extend([
                            terms.value,
                            companion.fits,
                            companion.index,
                            companion.captures,
                            companion.relation,
                        ]),
                        None => out.push(terms.value),
                    }
                }
                Ok(out)
            }
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
            | SemanticTerm::Apply { .. }
            | SemanticTerm::FunctionRef { .. }
            | SemanticTerm::MapLiteral { .. }
            | SemanticTerm::SetLiteral { .. }
            | SemanticTerm::GraphLiteral { .. }
            | SemanticTerm::CheckedApply { .. } => Ok(Vec::new()),
        }
    }
}
