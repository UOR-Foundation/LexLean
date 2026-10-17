//! The recursors a structural definition is written with (SPEC.md §17.17).
//!
//! A function that recurses structurally is defined here by the recursor of
//! its argument's type, as the kernel has it: a motive for each type of the
//! recursive group (the mutual block's types, then the containers the kernel
//! nests in them), and a minor premise for each constructor, whose induction
//! hypotheses stand for the recursive calls. This module reads the group out
//! of the layout and the source: its positions, their constructors, and which
//! of a constructor's fields are recursive.

use std::collections::BTreeSet;

use super::super::data::{canonical, container_parts, named_member, type_arguments, Aux, AuxKind};
use super::super::{components, Gen};
use super::Writer;
use crate::ir::core::CoreLevel;
use crate::ir::semantic::SemanticType;

/// How a position of a plan is recursed on.
#[derive(Clone)]
pub(super) enum PositionKind {
    /// A document type of the recursive block, by its index in the layout.
    Block { index: u64 },
    /// A container nested in the block.
    Aux(AuxEntry),
    /// A type of Lean's own: `Nat` or a list.
    Builtin,
}

/// A nested container of a group.
#[derive(Clone)]
pub(super) struct AuxEntry {
    pub(super) original: SemanticType,
    pub(super) canonical: SemanticType,
    pub(super) kind: AuxKind,
    pub(super) nested_index: u64,
}

/// One constructor of a position.
#[derive(Clone)]
pub(super) struct RecCtor {
    /// The closed types of its fields.
    pub(super) fields: Vec<SemanticType>,
    /// Per field, the position it recurses on, when it is recursive.
    pub(super) recursive: Vec<Option<usize>>,
}

/// One position of a plan: a type the recursor has a motive for.
#[derive(Clone)]
pub(super) struct Position {
    /// The type as the source writes it.
    pub(super) ty: SemanticType,
    pub(super) kind: PositionKind,
    pub(super) constructors: Vec<RecCtor>,
}

/// The recursors of one recursive group.
#[derive(Clone)]
pub(super) struct RecPlan {
    /// The type arguments the recursor is applied at.
    pub(super) params: Vec<SemanticType>,
    pub(super) positions: Vec<Position>,
    /// The constant of the recursor that has a position as its major
    /// premise.
    pub(super) recursors: Vec<String>,
    /// How the recursor's universe parameters are written for a motive into
    /// `Sort u`.
    pub(super) builtin_list: bool,
}

impl RecPlan {
    /// The universe levels of the recursor at motive sort `u`.
    pub(super) fn levels(&self, motive: CoreLevel) -> Vec<CoreLevel> {
        if self.builtin_list {
            vec![motive, CoreLevel::Zero]
        } else {
            vec![motive]
        }
    }

    /// The position of a type among the plan's, if it has one.
    pub(super) fn position_of(&self, gen: &Gen<'_>, ty: &SemanticType) -> Option<usize> {
        let key = gen.source.type_text(&canonical(ty));
        self.positions.iter().position(|position| {
            let own = match &position.kind {
                PositionKind::Aux(aux) => &aux.canonical,
                PositionKind::Block { .. } | PositionKind::Builtin => &position.ty,
            };
            gen.source.type_text(&canonical(own)) == key
        })
    }
}

impl Writer<'_, '_> {
    /// The plan for recursing structurally on a value of closed type `ty`.
    pub(super) fn rec_plan(&mut self, ty: &SemanticType) -> Result<RecPlan, String> {
        match ty {
            SemanticType::Nat => Ok(RecPlan {
                params: Vec::new(),
                positions: vec![Position {
                    ty: SemanticType::Nat,
                    kind: PositionKind::Builtin,
                    constructors: vec![
                        RecCtor {
                            fields: Vec::new(),
                            recursive: Vec::new(),
                        },
                        RecCtor {
                            fields: vec![SemanticType::Nat],
                            recursive: vec![Some(0)],
                        },
                    ],
                }],
                recursors: vec!["Nat.rec".to_owned()],
                builtin_list: false,
            }),
            SemanticType::List { element } => Ok(RecPlan {
                params: vec![element.as_ref().clone()],
                positions: vec![Position {
                    ty: ty.clone(),
                    kind: PositionKind::Builtin,
                    constructors: vec![
                        RecCtor {
                            fields: Vec::new(),
                            recursive: Vec::new(),
                        },
                        RecCtor {
                            fields: vec![element.as_ref().clone(), ty.clone()],
                            recursive: vec![None, Some(0)],
                        },
                    ],
                }],
                recursors: vec!["List.rec".to_owned()],
                builtin_list: true,
            }),
            SemanticType::Named { .. } => {
                let text = self.gen.source.type_text(ty);
                let index = *self
                    .gen
                    .adts
                    .get(&text)
                    .ok_or_else(|| format!("`{text}` has no ADT in the layout"))?;
                let graph = self.gen.adt_graph()?;
                let component = components(&graph)
                    .into_iter()
                    .find(|component| component.contains(&index))
                    .ok_or("a document type outside every component")?;
                self.group_plan(&component)
            }
            SemanticType::Type
            | SemanticType::Parameter { name: _ }
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
            | SemanticType::Product { left: _, right: _ }
            | SemanticType::Function {
                parameters: _,
                result: _,
            }
            | SemanticType::Map { key: _, value: _ }
            | SemanticType::Set { element: _ }
            | SemanticType::ContractViolation
            | SemanticType::ReasoningFailure => Err(format!(
                "structural recursion on `{}` has no recursor here",
                self.gen.source.type_text(ty)
            )),
        }
    }

    /// The plan of one component of the document types' graph.
    pub(super) fn group_plan(&mut self, component: &[u64]) -> Result<RecPlan, String> {
        let mut ordered = Vec::new();
        for &index in component {
            let ty = &self.gen.layout.adts[index as usize].ty;
            let member = named_member(ty)?;
            let module = member.module.clone().ok_or("an unanchored document type")?;
            let position = self
                .gen
                .source
                .declaration_position(&module, &member.name)
                .ok_or_else(|| format!("`{}` is not declared in `{module}`", member.name))?;
            ordered.push((position, index));
        }
        ordered.sort_unstable();
        let block: Vec<u64> = ordered.into_iter().map(|(_, index)| index).collect();
        let key = |gen: &Gen<'_>, ty: &SemanticType| gen.source.type_text(&canonical(ty));
        let block_keys: BTreeSet<String> = block
            .iter()
            .map(|&index| key(self.gen, &self.gen.layout.adts[index as usize].ty))
            .collect();
        let mut aux: Vec<Aux> = Vec::new();
        let mut seen: BTreeSet<String> = BTreeSet::new();
        for &index in &block {
            let shape = self
                .gen
                .source
                .document(&self.gen.layout.adts[index as usize].ty)?;
            for fields in &shape.fields {
                for field in fields {
                    self.register_aux(field, &block_keys, &mut aux, &mut seen)?;
                }
            }
        }
        let mut next = 0;
        while next < aux.len() {
            for field in self.aux_field_types(&aux[next])? {
                self.register_aux(&field, &block_keys, &mut aux, &mut seen)?;
            }
            next += 1;
        }
        let first = self.gen.layout.adts[block[0] as usize].ty.clone();
        let first_member = named_member(&first)?;
        let first_name = self.document_name(&first_member)?;
        let mut positions = Vec::new();
        let mut recursors = Vec::new();
        for &index in &block {
            positions.push(Position {
                ty: self.gen.layout.adts[index as usize].ty.clone(),
                kind: PositionKind::Block { index },
                constructors: Vec::new(),
            });
            let member = named_member(&self.gen.layout.adts[index as usize].ty)?;
            recursors.push(format!("{}.rec", self.document_name(&member)?));
        }
        for (position, entry) in aux.iter().enumerate() {
            positions.push(Position {
                ty: entry.canonical.clone(),
                kind: PositionKind::Aux(AuxEntry {
                    original: entry.original.clone(),
                    canonical: entry.canonical.clone(),
                    kind: entry.kind,
                    nested_index: entry.nested_index,
                }),
                constructors: Vec::new(),
            });
            recursors.push(format!("{first_name}.rec_{}", position + 1));
        }
        let mut plan = RecPlan {
            params: type_arguments(&first),
            positions,
            recursors,
            builtin_list: false,
        };
        // The constructors, now that every position is known.
        for position in 0..plan.positions.len() {
            let constructors = self.constructors_of(&plan, position)?;
            plan.positions[position].constructors = constructors;
        }
        for nested in self.gen.nested.values() {
            if component.contains(&nested.group)
                && !aux.iter().any(|entry| entry.nested_index == nested.index)
            {
                return Err(format!(
                    "`{}` is an auxiliary of the group but the kernel nests no such type",
                    self.gen.source.type_text(&nested.ty)
                ));
            }
        }
        Ok(plan)
    }

    /// The constructors of a position of a plan.
    fn constructors_of(&self, plan: &RecPlan, position: usize) -> Result<Vec<RecCtor>, String> {
        let ctor = |fields: Vec<SemanticType>| -> RecCtor {
            let recursive = fields
                .iter()
                .map(|field| plan.position_of(self.gen, field))
                .collect();
            RecCtor { fields, recursive }
        };
        let entry = &plan.positions[position];
        match &entry.kind {
            PositionKind::Block { index: _ } => {
                let shape = self.gen.source.document(&entry.ty)?;
                Ok(shape
                    .fields
                    .iter()
                    .map(|fields| ctor(fields.clone()))
                    .collect())
            }
            PositionKind::Builtin => Ok(entry.constructors.clone()),
            PositionKind::Aux(aux) => {
                let parts = container_parts(&aux.original)?;
                Ok(match aux.kind {
                    AuxKind::List => vec![
                        ctor(Vec::new()),
                        ctor(vec![parts[0].clone(), aux.original.clone()]),
                    ],
                    AuxKind::Option => vec![ctor(Vec::new()), ctor(vec![parts[0].clone()])],
                    AuxKind::Prod => vec![ctor(parts)],
                    AuxKind::Except => {
                        vec![ctor(vec![parts[0].clone()]), ctor(vec![parts[1].clone()])]
                    }
                })
            }
        }
    }

    /// The field types of an auxiliary's constructors other than itself.
    fn aux_field_types(&self, aux: &Aux) -> Result<Vec<SemanticType>, String> {
        let parts = container_parts(&aux.original)?;
        let expected = match aux.kind {
            AuxKind::List | AuxKind::Option => 1,
            AuxKind::Prod | AuxKind::Except => 2,
        };
        if parts.len() != expected {
            return Err(format!(
                "`{}` is an auxiliary of the wrong shape",
                self.gen.source.type_text(&aux.original)
            ));
        }
        Ok(parts)
    }

    /// Whether a canonical type mentions a type of the block.
    fn mentions_block(&self, ty: &SemanticType, block_keys: &BTreeSet<String>) -> bool {
        match ty {
            SemanticType::Named {
                member: _,
                arguments: _,
            } => block_keys.contains(&self.gen.source.type_text(ty)),
            SemanticType::List { element } | SemanticType::Option { value: element } => {
                self.mentions_block(element, block_keys)
            }
            SemanticType::Product { left, right }
            | SemanticType::Result {
                ok: left,
                error: right,
            } => self.mentions_block(left, block_keys) || self.mentions_block(right, block_keys),
            SemanticType::Set { element: _ }
            | SemanticType::Map { key: _, value: _ }
            | SemanticType::ContractViolation
            | SemanticType::ReasoningFailure
            | SemanticType::Function {
                parameters: _,
                result: _,
            }
            | SemanticType::Type
            | SemanticType::Prop
            | SemanticType::Parameter { name: _ }
            | SemanticType::Nat
            | SemanticType::Bool
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
            | SemanticType::Ordering => false,
        }
    }

    /// Register the container `original` as an auxiliary of the group when it
    /// nests a type of the block and is not yet registered.
    fn register_aux(
        &self,
        original: &SemanticType,
        block_keys: &BTreeSet<String>,
        aux: &mut Vec<Aux>,
        seen: &mut BTreeSet<String>,
    ) -> Result<(), String> {
        let canonical = canonical(original);
        if !self.mentions_block(&canonical, block_keys) {
            return Ok(());
        }
        let kind = match &canonical {
            SemanticType::List { element: _ } => AuxKind::List,
            SemanticType::Option { value: _ } => AuxKind::Option,
            SemanticType::Product { left: _, right: _ } => AuxKind::Prod,
            SemanticType::Result { ok: _, error: _ } => AuxKind::Except,
            SemanticType::Named {
                member: _,
                arguments: _,
            }
            | SemanticType::Set { element: _ }
            | SemanticType::Map { key: _, value: _ }
            | SemanticType::ContractViolation
            | SemanticType::ReasoningFailure
            | SemanticType::Function {
                parameters: _,
                result: _,
            }
            | SemanticType::Type
            | SemanticType::Prop
            | SemanticType::Parameter { name: _ }
            | SemanticType::Nat
            | SemanticType::Bool
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
            | SemanticType::Ordering => return Ok(()),
        };
        let key = self.gen.source.type_text(&canonical);
        if !seen.insert(key) {
            return Ok(());
        }
        let text = self.gen.source.type_text(original);
        let nested_index = self
            .gen
            .nested
            .get(&text)
            .map(|nested| nested.index)
            .ok_or_else(|| {
                format!("`{text}` is nested in a recursive group, but certificate A has no auxiliary encoder for it")
            })?;
        aux.push(Aux {
            original: original.clone(),
            canonical,
            kind,
            nested_index,
        });
        Ok(())
    }
}
