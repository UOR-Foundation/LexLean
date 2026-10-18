//! The encoders of the document types a program realizes (SPEC.md §17.17.3):
//! `__enc_k` for each type, and for a recursive group of types the auxiliary
//! encoders of the containers nested in them with their bundles.

use super::super::components;
use super::super::data::AuxKind;
use super::recplan::{PositionKind, RecPlan};
use super::{RowKind, Writer, EXPLICIT};
use crate::ir::core::CoreLevel;
use crate::ir::semantic::SemanticType;
use crate::production::kernel::{Given, Term};

/// The short name of the row that encodes a position.
fn row_name(plan: &RecPlan, position: usize) -> Result<String, String> {
    match &plan.positions[position].kind {
        PositionKind::Block { index } => Ok(format!("__enc_{index}")),
        PositionKind::Aux(aux) => Ok(match aux.kind {
            AuxKind::List => format!("__items_{}", aux.nested_index),
            AuxKind::Option | AuxKind::Prod | AuxKind::Except => {
                format!("__aux_{}", aux.nested_index)
            }
        }),
        PositionKind::Builtin => Err("a builtin position has no encoder row".to_owned()),
    }
}

impl<'a> Writer<'_, 'a> {
    /// The encoder rows of every document type of the layout, dependencies
    /// first.
    pub(super) fn encoders(&mut self) -> Result<(), String> {
        let graph = self.gen.adt_graph()?;
        for component in components(&graph) {
            let recursive = component.len() > 1
                || component.iter().any(|index| {
                    self.gen
                        .nested
                        .values()
                        .any(|nested| nested.group == *index)
                        || graph.get(index).is_some_and(|edges| edges.contains(index))
                });
            if recursive {
                self.group_encoders(&component)?;
            } else {
                for index in component {
                    self.plain_encoder(index)?;
                }
            }
        }
        Ok(())
    }

    /// The encoder of a document type outside any recursive group.
    fn plain_encoder(&mut self, index: u64) -> Result<(), String> {
        let ty = self.gen.layout.adts[index as usize].ty.clone();
        let shape = self.gen.source.document(&ty)?;
        let lean = self.lean_ty(&ty)?;
        let value_ty = self.value_ty();
        let adt = self.ts("Value.adt");
        let value = if shape.kind == "inductive" {
            let mut minors = Vec::new();
            for (position, fields) in shape.fields.iter().enumerate() {
                let mut frees = Vec::new();
                let mut types = Vec::new();
                let mut encoded = Vec::new();
                for field in fields {
                    let free = self.b.free();
                    frees.push(free);
                    types.push(self.lean_ty(field)?);
                    let enc = self.enc(field)?;
                    encoded.push(self.b.app(enc, free));
                }
                let list = self.b.list(value_ty, &encoded);
                let tag = self.nat_u(position as u64);
                let mut minor = self.b.apps(adt, &[tag, list]);
                for (position, free) in frees.iter().enumerate().rev() {
                    minor = self.b.lam_over(
                        &format!("__x{position}"),
                        EXPLICIT,
                        types[position],
                        *free,
                        minor,
                    );
                }
                minors.push(minor);
            }
            let free = self.b.free();
            let cases = self.cases_on_const(&ty, free, value_ty, &minors)?;
            self.b.lam_over("__v", EXPLICIT, lean, free, cases)
        } else {
            let free = self.b.free();
            let mut encoded = Vec::new();
            for (name, field) in shape.field_names.iter().zip(&shape.fields[0]) {
                let projected = self.project_term(&ty, name, free)?;
                let enc = self.enc(field)?;
                encoded.push(self.b.app(enc, projected));
            }
            let list = self.b.list(value_ty, &encoded);
            let tag = self.nat_u(0);
            let body = self.b.apps(adt, &[tag, list]);
            self.b.lam_over("__s", EXPLICIT, lean, free, body)
        };
        let row_ty = self.b.arrow(lean, value_ty);
        self.push_row(
            &format!("__enc_{index}"),
            RowKind::Definition { compiled: true },
            row_ty,
            value,
        )
    }

    /// The value one field of a constructor contributes to its encoding:
    /// the recursive value standing for a recursive field, else the field's
    /// own encoding.
    fn field_encoding(
        &mut self,
        plan: &RecPlan,
        field_ty: &SemanticType,
        field: Term,
        recursive: Option<(usize, Term)>,
    ) -> Result<Term, String> {
        match recursive {
            Some((position, value)) => {
                if matches!(
                    &plan.positions[position].kind,
                    PositionKind::Aux(aux) if aux.kind == AuxKind::List
                ) {
                    let wrap = self.ts("Value.list");
                    Ok(self.b.app(wrap, value))
                } else {
                    Ok(value)
                }
            }
            None => {
                let enc = self.enc(field_ty)?;
                Ok(self.b.app(enc, field))
            }
        }
    }

    /// The encoding of the constructor `ctor` of `position`, over its fields
    /// and the recursive values of its recursive fields.
    fn constructor_encoding(
        &mut self,
        plan: &RecPlan,
        position: usize,
        ctor: usize,
        fields: &[Term],
        recursive: &[Option<Term>],
    ) -> Result<Term, String> {
        let entry = &plan.positions[position];
        let shape = &entry.constructors[ctor];
        let mut parts = Vec::new();
        for (field_index, field_ty) in shape.fields.iter().enumerate() {
            let rec = shape.recursive[field_index].zip(recursive[field_index]);
            // The tail of a list auxiliary is the auxiliary itself, whose
            // value is already a list of encodings.
            let is_list_tail = matches!(&entry.kind,
                PositionKind::Aux(aux) if aux.kind == AuxKind::List && field_index == 1);
            if is_list_tail {
                parts.push(recursive[1].ok_or("a list auxiliary whose tail is not recursive")?);
            } else {
                parts.push(self.field_encoding(plan, field_ty, fields[field_index], rec)?);
            }
        }
        let value_ty = self.value_ty();
        match &entry.kind {
            PositionKind::Block { index: _ } => {
                let list = self.b.list(value_ty, &parts);
                let tag = self.nat_u(ctor as u64);
                let adt = self.ts("Value.adt");
                Ok(self.b.apps(adt, &[tag, list]))
            }
            PositionKind::Aux(aux) => match aux.kind {
                AuxKind::List => {
                    if ctor == 0 {
                        let nil = self.b.konst0("List.nil");
                        Ok(self.b.app(nil, value_ty))
                    } else {
                        let cons = self.b.konst0("List.cons");
                        Ok(self.b.apps(cons, &[value_ty, parts[0], parts[1]]))
                    }
                }
                AuxKind::Option => {
                    if ctor == 0 {
                        Ok(self.ts("Value.none"))
                    } else {
                        let some = self.ts("Value.some");
                        Ok(self.b.app(some, parts[0]))
                    }
                }
                AuxKind::Prod => {
                    let pair = self.ts("Value.pair");
                    Ok(self.b.apps(pair, &[parts[0], parts[1]]))
                }
                AuxKind::Except => {
                    let head = self.ts(if ctor == 0 { "Value.error" } else { "Value.ok" });
                    Ok(self.b.app(head, parts[0]))
                }
            },
            PositionKind::Builtin => Err("a builtin position has no encoding".to_owned()),
        }
    }

    /// The result type of the motive of a position: a list of encodings for a
    /// list auxiliary, else an encoding.
    fn result_type(&mut self, plan: &RecPlan, position: usize) -> Term {
        let value_ty = self.value_ty();
        match &plan.positions[position].kind {
            PositionKind::Aux(aux) if aux.kind == AuxKind::List => self.b.list_ty(value_ty),
            PositionKind::Aux(_) | PositionKind::Block { .. } | PositionKind::Builtin => value_ty,
        }
    }

    /// The rows of a recursive group of document types and the containers
    /// nested in them: each position's encoder by the group's recursor, its
    /// implementation record for the code generator, and the bundles of the
    /// list and option containers.
    #[allow(clippy::too_many_lines)]
    fn group_encoders(&mut self, component: &[u64]) -> Result<(), String> {
        let plan = self.group_plan(component)?;
        let one = CoreLevel::Succ(Box::new(CoreLevel::Zero));
        let mut params = Vec::new();
        for argument in &plan.params {
            params.push(self.lean_ty(argument)?);
        }
        // The motives: each position into the type its encoding has.
        let mut domains = Vec::new();
        let mut results = Vec::new();
        let mut motives = Vec::new();
        for position in 0..plan.positions.len() {
            let domain = self.lean_ty(&plan.positions[position].ty)?;
            let result = self.result_type(&plan, position);
            let lifted = self.b.lift(result, 1, 0);
            motives.push(self.b.lam("__v", EXPLICIT, domain, lifted));
            domains.push(domain);
            results.push(result);
        }
        // The minor premises: block constructors in order, then each
        // auxiliary's.
        let mut minors = Vec::new();
        for position in 0..plan.positions.len() {
            for (ctor, shape) in plan.positions[position].constructors.iter().enumerate() {
                let mut frees = Vec::new();
                let mut types = Vec::new();
                for field in &shape.fields {
                    frees.push(self.b.free());
                    types.push(self.lean_ty(field)?);
                }
                let mut hypotheses: Vec<Option<Term>> = Vec::new();
                let mut hypothesis_types = Vec::new();
                for recursion in &shape.recursive {
                    match recursion {
                        Some(target) => {
                            hypotheses.push(Some(self.b.free()));
                            hypothesis_types.push(Some(results[*target]));
                        }
                        None => {
                            hypotheses.push(None);
                            hypothesis_types.push(None);
                        }
                    }
                }
                let body = self.constructor_encoding(&plan, position, ctor, &frees, &hypotheses)?;
                let mut minor = body;
                for (index, hypothesis) in hypotheses.iter().enumerate().rev() {
                    if let (Some(free), Some(ty)) = (hypothesis, hypothesis_types[index]) {
                        minor =
                            self.b
                                .lam_over(&format!("__ih{index}"), EXPLICIT, ty, *free, minor);
                    }
                }
                for (index, free) in frees.iter().enumerate().rev() {
                    minor = self.b.lam_over(
                        &format!("__x{index}"),
                        EXPLICIT,
                        types[index],
                        *free,
                        minor,
                    );
                }
                minors.push(minor);
            }
        }
        // The rows: the group's recursor with each position as the major
        // premise.
        let mut names = Vec::new();
        for position in 0..plan.positions.len() {
            let name = row_name(&plan, position)?;
            let recursor = self
                .b
                .konst_at(&plan.recursors[position], plan.levels(one.clone()));
            let free = self.b.free();
            let mut args = params.clone();
            args.extend(motives.iter().copied());
            args.extend(minors.iter().copied());
            args.push(free);
            let applied = self.b.apps(recursor, &args);
            let value = self
                .b
                .lam_over("__v", EXPLICIT, domains[position], free, applied);
            let row_ty = self.b.arrow(domains[position], results[position]);
            self.push_row(&name, RowKind::Definition { compiled: true }, row_ty, value)?;
            names.push(name);
        }
        self.group_rows(&names);
        // The implementation records: direct recursion by the eliminator of
        // each type, calling the records of the others.
        let unsafe_names: Vec<String> = names
            .iter()
            .map(|name| format!("{name}._unsafe_rec"))
            .collect();
        for position in 0..plan.positions.len() {
            let mut cases_minors = Vec::new();
            for (ctor, shape) in plan.positions[position].constructors.iter().enumerate() {
                let mut frees = Vec::new();
                let mut types = Vec::new();
                for field in &shape.fields {
                    frees.push(self.b.free());
                    types.push(self.lean_ty(field)?);
                }
                let mut calls: Vec<Option<Term>> = Vec::new();
                for (index, recursion) in shape.recursive.iter().enumerate() {
                    match recursion {
                        Some(target) => {
                            let head = self.own(&unsafe_names[*target]);
                            calls.push(Some(self.b.app(head, frees[index])));
                        }
                        None => calls.push(None),
                    }
                }
                let mut minor = self.constructor_encoding(&plan, position, ctor, &frees, &calls)?;
                for (index, free) in frees.iter().enumerate().rev() {
                    minor = self.b.lam_over(
                        &format!("__x{index}"),
                        EXPLICIT,
                        types[index],
                        *free,
                        minor,
                    );
                }
                cases_minors.push(minor);
            }
            let free = self.b.free();
            let cases = self.cases_on_const(
                &plan.positions[position].ty,
                free,
                results[position],
                &cases_minors,
            )?;
            let value = self
                .b
                .lam_over("__v", EXPLICIT, domains[position], free, cases);
            let row_ty = self.b.arrow(domains[position], results[position]);
            self.push_row(&unsafe_names[position], RowKind::Partial, row_ty, value)?;
        }
        self.group_rows(&unsafe_names);
        // The bundles of the containers.
        for position in 0..plan.positions.len() {
            if let PositionKind::Aux(aux) = &plan.positions[position].kind {
                self.bundle(&plan, position, aux.kind, aux.nested_index)?;
            }
        }
        Ok(())
    }

    /// `__L_n` or `__O_n`: the container's encoder as the library's bundle.
    fn bundle(
        &mut self,
        plan: &RecPlan,
        position: usize,
        kind: AuxKind,
        nested_index: u64,
    ) -> Result<(), String> {
        let aux = match &plan.positions[position].kind {
            PositionKind::Aux(aux) => aux.clone(),
            PositionKind::Block { .. } | PositionKind::Builtin => {
                return Err("a bundle of a type that is not a container".to_owned());
            }
        };
        let parts = super::super::data::container_parts(&aux.original)?;
        match kind {
            AuxKind::List => {
                let element = parts[0].clone();
                let alpha = self.lean_ty(&element)?;
                let elem = self.enc(&element)?;
                let items = self.own(&row_name(plan, position)?);
                let ty = {
                    let head = self.lp("ListEnc");
                    self.b.app(head, alpha)
                };
                let applied = self.apply(
                    "ListEnc.mk",
                    &[("α", alpha)],
                    &[
                        Given::Term(elem),
                        Given::Term(items),
                        Given::Refl,
                        Given::Refl,
                    ],
                )?;
                self.push_row(
                    &format!("__L_{nested_index}"),
                    RowKind::Definition { compiled: false },
                    ty,
                    applied.proof,
                )
            }
            AuxKind::Option => {
                let inner = parts[0].clone();
                let alpha = self.lean_ty(&inner)?;
                let elem = self.enc(&inner)?;
                let encoder = self.own(&row_name(plan, position)?);
                let ty = {
                    let head = self.lp("OptEnc");
                    self.b.app(head, alpha)
                };
                let applied = self.apply(
                    "OptEnc.mk",
                    &[("α", alpha)],
                    &[
                        Given::Term(elem),
                        Given::Term(encoder),
                        Given::Refl,
                        Given::Refl,
                    ],
                )?;
                self.push_row(
                    &format!("__O_{nested_index}"),
                    RowKind::Definition { compiled: false },
                    ty,
                    applied.proof,
                )
            }
            AuxKind::Prod | AuxKind::Except => Ok(()),
        }
    }
}
