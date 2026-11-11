//! The `region_check` proof form (SPEC.md §17.12, *Decision regions*): the
//! lowering of a property of a decision fragment to the data of the runtime
//! module `LexLeanRuntime.Regions`, and the proof the backend emits.
//!
//! A decision fragment is a structure of natural-number fields (the request),
//! a first-match `judge` over it, whose rules conjoin comparisons of one field
//! with one constant, and a Boolean `breaks` definition that conjoins such
//! comparisons with a test of the judge's answer. The theorem states
//! `breaks request = false`.
//!
//! Nothing here is trusted. The lowered data is written in exactly the shapes
//! the backend emits for the module (`Nat.blt`, `Nat.ble`, `Nat.beq`, `!`,
//! `&&` folded from the left, `if b then`), so the module's `breaks` applied to
//! the request is *definitionally* the runtime's `breaks` of the data, and the
//! kernel checks that. A lowering that misread the module produces a theorem
//! Lean refuses, never a false one. The checks here only make a module outside
//! the fragment fail at `check` with a reason instead of at `verify`.

use std::collections::BTreeMap;

use super::{
    MemberRef, SemanticDeclaration, SemanticParameter, SemanticProof, SemanticTerm, SemanticType,
};

/// One comparison of a field with a constant, in the runtime's operator
/// codes: 0 `x < c`, 1 `x ≤ c`, 2 `c < x`, 3 `c ≤ x`, 4 `x = c`, 5 `c = x`,
/// 6 `x ≠ c`, 7 `c ≠ x`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cond {
    pub field: usize,
    pub op: u8,
    pub value: u128,
}

/// One rule of the judge: its conditions and its answer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rule {
    pub when: Vec<Cond>,
    pub outcome: u128,
}

/// One property theorem, lowered.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lowered {
    /// The theorem's one parameter.
    pub request: String,
    /// The request structure's fields, in declaration order.
    pub fields: Vec<String>,
    pub rules: Vec<Rule>,
    /// The judge's `otherwise` answer.
    pub otherwise: u128,
    /// `true` when the property says the answer must be `outcome`, `false`
    /// when it must not be.
    pub always: bool,
    pub when: Vec<Cond>,
    pub outcome: u128,
    /// One closed interval per field: from 0 to the least upper bound the
    /// property's own conditions give that field.
    pub bounds: Vec<u128>,
}

/// The prior declarations of the module, by name.
struct Scope<'a> {
    definitions: BTreeMap<&'a str, (&'a [SemanticParameter], &'a SemanticType, &'a SemanticTerm)>,
    structures: BTreeMap<&'a str, Vec<(&'a str, &'a SemanticType)>>,
}

impl<'a> Scope<'a> {
    fn new(declarations: &'a [SemanticDeclaration]) -> Self {
        let mut definitions = BTreeMap::new();
        let mut structures = BTreeMap::new();
        for declaration in declarations {
            match declaration {
                SemanticDeclaration::Definition {
                    name,
                    type_parameters,
                    parameters,
                    result,
                    body,
                    mutual: None,
                    termination: None,
                    recursive_argument: None,
                    ..
                } if type_parameters.is_empty() => {
                    definitions.insert(name.as_str(), (parameters.as_slice(), result, body));
                }
                SemanticDeclaration::Structure {
                    name,
                    type_parameters,
                    parameters,
                    fields,
                } if type_parameters.is_empty() && parameters.is_empty() => {
                    structures.insert(
                        name.as_str(),
                        fields
                            .iter()
                            .map(|field| (field.name.as_str(), &field.r#type))
                            .collect(),
                    );
                }
                _ => {}
            }
        }
        Self {
            definitions,
            structures,
        }
    }

    /// A local, non-recursive definition of this module.
    fn definition(
        &self,
        member: &MemberRef,
    ) -> Option<(&'a [SemanticParameter], &'a SemanticType, &'a SemanticTerm)> {
        if member.module.is_some() {
            return None;
        }
        self.definitions.get(member.name.as_str()).copied()
    }

    /// A natural-number constant: a literal, or a call of a parameterless
    /// definition whose body is one, followed through such definitions.
    fn constant(&self, term: &SemanticTerm, depth: usize) -> Option<u128> {
        match term {
            SemanticTerm::Nat { value } => value.parse().ok(),
            SemanticTerm::Call {
                function,
                type_arguments,
                arguments,
            } if type_arguments.is_empty() && arguments.is_empty() && depth < 64 => {
                let (parameters, result, body) = self.definition(function)?;
                if !parameters.is_empty() || *result != SemanticType::Nat {
                    return None;
                }
                self.constant(body, depth + 1)
            }
            _ => None,
        }
    }
}

/// The context of one lowering: the request's name and fields.
struct Request<'s, 'a> {
    scope: &'s Scope<'a>,
    name: &'s str,
    fields: &'s [String],
}

impl Request<'_, '_> {
    fn field(&self, term: &SemanticTerm) -> Option<usize> {
        match term {
            SemanticTerm::Project { value, field } => match value.as_ref() {
                SemanticTerm::Var { name } if name == self.name => {
                    self.fields.iter().position(|candidate| candidate == field)
                }
                _ => None,
            },
            _ => None,
        }
    }

    /// One comparison of a field with a constant, either way round.
    fn comparison(&self, term: &SemanticTerm) -> Result<Cond, String> {
        let (left, right, codes) = match term {
            SemanticTerm::Blt { left, right } => (left, right, (0, 2)),
            SemanticTerm::Ble { left, right } => (left, right, (1, 3)),
            SemanticTerm::Beq { left, right } => (left, right, (4, 5)),
            SemanticTerm::Not { value } => match value.as_ref() {
                SemanticTerm::Beq { left, right } => (left, right, (6, 7)),
                _ => return Err("a negation other than `!(a =? b)`".to_owned()),
            },
            _ => {
                return Err(
                    "a conjunct that is not a comparison of a request field with a constant"
                        .to_owned(),
                )
            }
        };
        if let (Some(field), Some(value)) = (self.field(left), self.scope.constant(right, 0)) {
            return Ok(Cond {
                field,
                op: codes.0,
                value,
            });
        }
        if let (Some(value), Some(field)) = (self.scope.constant(left, 0), self.field(right)) {
            return Ok(Cond {
                field,
                op: codes.1,
                value,
            });
        }
        Err("a comparison that is not of a request field with a constant".to_owned())
    }

    /// A conjunction folded from the left, `((c1 && c2) && c3)`, read into its
    /// conjuncts in order. A right operand that is itself a conjunction is one
    /// conjunct, which `comparison` then refuses.
    fn conjuncts<'t>(&self, term: &'t SemanticTerm, out: &mut Vec<&'t SemanticTerm>) {
        match term {
            SemanticTerm::And { left, right } => {
                self.conjuncts(left, out);
                out.push(right);
            }
            other => out.push(other),
        }
    }

    fn conditions(&self, term: &SemanticTerm) -> Result<Vec<Cond>, String> {
        let mut terms = Vec::new();
        self.conjuncts(term, &mut terms);
        terms
            .into_iter()
            .map(|term| self.comparison(term))
            .collect()
    }

    /// `judge request`, the one call the property's test may make.
    fn judge_call<'t>(&self, term: &'t SemanticTerm) -> Option<&'t MemberRef> {
        match term {
            SemanticTerm::Call {
                function,
                type_arguments,
                arguments,
            } if type_arguments.is_empty()
                && matches!(arguments.as_slice(), [SemanticTerm::Var { name }] if name == self.name) =>
            {
                Some(function)
            }
            _ => None,
        }
    }

    /// A rule's test: a call of a one-parameter Boolean definition over the
    /// request whose body is a conjunction of comparisons, or the
    /// conjunction itself.
    fn rule_conditions(&self, term: &SemanticTerm) -> Result<Vec<Cond>, String> {
        if let Some(function) = self.judge_call(term) {
            let (parameters, result, body) = self
                .scope
                .definition(function)
                .ok_or_else(|| format!("rule `{}` is not a local definition", function.name))?;
            let [parameter] = parameters else {
                return Err(format!(
                    "rule `{}` does not take only the request",
                    function.name
                ));
            };
            if *result != SemanticType::Bool {
                return Err(format!("rule `{}` is not Boolean", function.name));
            }
            let inner = Request {
                scope: self.scope,
                name: &parameter.name,
                fields: self.fields,
            };
            return inner
                .conditions(body)
                .map_err(|reason| format!("rule `{}` has {reason}", function.name));
        }
        self.conditions(term)
            .map_err(|reason| format!("a judge test has {reason}"))
    }
}

fn exact(conditions: &[Cond], what: &str) -> Result<(), String> {
    if conditions.iter().any(|condition| condition.op >= 6) {
        return Err(format!(
            "{what} uses `!=`, whose region is not a box; a rule may use only <, <=, >, >= and ="
        ));
    }
    Ok(())
}

/// Lower one theorem whose proof is `region_check`. `declarations` are the
/// module's declarations before the theorem.
pub fn lower(
    declarations: &[SemanticDeclaration],
    theorem: &SemanticDeclaration,
) -> Result<Lowered, String> {
    let SemanticDeclaration::Theorem {
        name,
        type_parameters,
        parameters,
        statement,
        proof: SemanticProof::RegionCheck,
        ..
    } = theorem
    else {
        return Err(
            "internal: region_check lowering of a declaration that is not a region_check theorem"
                .to_owned(),
        );
    };
    let fail = |reason: String| format!("region_check theorem `{name}`: {reason}");
    let scope = Scope::new(declarations);
    if !type_parameters.is_empty() {
        return Err(fail(
            "a decision property has no type parameters".to_owned(),
        ));
    }
    let [parameter] = parameters.as_slice() else {
        return Err(fail(
            "a decision property takes exactly one request".to_owned(),
        ));
    };
    let SemanticType::Named { member, arguments } = &parameter.r#type else {
        return Err(fail("the request is not a structure".to_owned()));
    };
    let structure = (member.module.is_none() && arguments.is_empty())
        .then(|| scope.structures.get(member.name.as_str()))
        .flatten()
        .ok_or_else(|| fail(format!("`{}` is not a local structure", member.name)))?;
    if structure.iter().any(|(_, ty)| **ty != SemanticType::Nat) {
        return Err(fail(format!(
            "every field of `{}` must be Nat",
            member.name
        )));
    }
    let fields: Vec<String> = structure
        .iter()
        .map(|(field, _)| (*field).to_owned())
        .collect();
    let request = Request {
        scope: &scope,
        name: &parameter.name,
        fields: &fields,
    };
    // The statement: `breaks request = false`.
    let breaks = match statement {
        SemanticTerm::Eq { left, right }
            if matches!(right.as_ref(), SemanticTerm::Bool { value: false }) =>
        {
            request.judge_call(left)
        }
        _ => None,
    }
    .ok_or_else(|| fail("the statement is not `breaks request = false`".to_owned()))?;
    let (breaks_parameters, breaks_result, breaks_body) = scope
        .definition(breaks)
        .ok_or_else(|| fail(format!("`{}` is not a local definition", breaks.name)))?;
    let [breaks_parameter] = breaks_parameters else {
        return Err(fail(format!(
            "`{}` does not take only the request",
            breaks.name
        )));
    };
    if *breaks_result != SemanticType::Bool || breaks_parameter.r#type != parameter.r#type {
        return Err(fail(format!(
            "`{}` is not a Boolean definition over the request",
            breaks.name
        )));
    }
    let inner = Request {
        scope: &scope,
        name: &breaks_parameter.name,
        fields: &fields,
    };
    // Its body: `conditions && test`, or the test alone.
    let (conditions, test) = match breaks_body {
        SemanticTerm::And { left, right } => (Some(left.as_ref()), right.as_ref()),
        other => (None, other),
    };
    let when = match conditions {
        Some(conditions) => inner
            .conditions(conditions)
            .map_err(|reason| fail(format!("`{}` has {reason}", breaks.name)))?,
        None => Vec::new(),
    };
    let (always, judged, outcome) = match test {
        SemanticTerm::Not { value } => match value.as_ref() {
            SemanticTerm::Beq { left, right } => (true, left.as_ref(), right.as_ref()),
            _ => (false, test, test),
        },
        SemanticTerm::Beq { left, right } => (false, left.as_ref(), right.as_ref()),
        _ => (false, test, test),
    };
    let judge = inner.judge_call(judged).ok_or_else(|| {
        fail(format!(
            "`{}` does not end with a test `judge request =? answer` or its negation",
            breaks.name
        ))
    })?;
    let outcome = scope
        .constant(outcome, 0)
        .ok_or_else(|| fail("the tested answer is not a constant".to_owned()))?;
    // The judge: `if rule1 request then a1 else if ... else otherwise`.
    let (judge_parameters, judge_result, judge_body) = scope
        .definition(judge)
        .ok_or_else(|| fail(format!("`{}` is not a local definition", judge.name)))?;
    let [judge_parameter] = judge_parameters else {
        return Err(fail(format!(
            "`{}` does not take only the request",
            judge.name
        )));
    };
    if *judge_result != SemanticType::Nat || judge_parameter.r#type != parameter.r#type {
        return Err(fail(format!(
            "`{}` is not a Nat-valued definition over the request",
            judge.name
        )));
    }
    let judging = Request {
        scope: &scope,
        name: &judge_parameter.name,
        fields: &fields,
    };
    let mut rules = Vec::new();
    let mut at = judge_body;
    let otherwise = loop {
        match at {
            SemanticTerm::If {
                condition,
                then_value,
                else_value,
            } => {
                let when = judging
                    .rule_conditions(condition)
                    .map_err(|reason| fail(format!("`{}`: {reason}", judge.name)))?;
                exact(
                    &when,
                    &format!("rule {} of `{}`", rules.len() + 1, judge.name),
                )
                .map_err(fail)?;
                let outcome = scope.constant(then_value, 0).ok_or_else(|| {
                    fail(format!(
                        "rule {} of `{}` does not answer a constant",
                        rules.len() + 1,
                        judge.name
                    ))
                })?;
                rules.push(Rule { when, outcome });
                at = else_value;
            }
            other => {
                break scope.constant(other, 0).ok_or_else(|| {
                    fail(format!(
                        "`{}` is not a chain of `if rule then answer else ...` ending in a constant",
                        judge.name
                    ))
                })?;
            }
        }
    };
    // The box: every field bounded above by the property's own conditions.
    let mut bounds = Vec::with_capacity(fields.len());
    for (index, field) in fields.iter().enumerate() {
        let bound = when
            .iter()
            .filter(|condition| condition.field == index)
            .filter_map(|condition| match condition.op {
                0 => Some(condition.value.saturating_sub(1)),
                1 | 4 | 5 => Some(condition.value),
                _ => None,
            })
            .min()
            .ok_or_else(|| {
                fail(format!(
                    "`{}` does not bound field `{field}` above (with <, <= or =), so its region is not a box",
                    breaks.name
                ))
            })?;
        bounds.push(bound);
    }
    Ok(Lowered {
        request: parameter.name.clone(),
        fields,
        rules,
        otherwise,
        always,
        when,
        outcome,
        bounds,
    })
}

fn conditions_text(conditions: &[Cond]) -> String {
    conditions
        .iter()
        .map(|condition| {
            format!(
                "(LexLeanRegions.Cond.mk {} {} {})",
                condition.field, condition.op, condition.value
            )
        })
        .collect::<Vec<_>>()
        .join(", ")
}

/// The proof the backend emits: one application of `LexLeanRegions.holds`,
/// whose obligation the kernel decides. `request` is the request's Lean
/// spelling; `project` spells one of its fields.
pub fn proof_text(lowered: &Lowered, pad: &str, project: impl Fn(&str) -> String) -> String {
    let rules = lowered
        .rules
        .iter()
        .map(|rule| {
            format!(
                "(LexLeanRegions.Rule.mk [{}] {})",
                conditions_text(&rule.when),
                rule.outcome
            )
        })
        .collect::<Vec<_>>()
        .join(", ");
    let bounds = lowered
        .bounds
        .iter()
        .map(|bound| format!("(0, {bound})"))
        .collect::<Vec<_>>()
        .join(", ");
    let request = lowered
        .fields
        .iter()
        .map(|field| project(field))
        .collect::<Vec<_>>()
        .join(", ");
    format!(
        "{pad}exact LexLeanRegions.holds [{rules}] {} (LexLeanRegions.Property.mk {} [{}] {}) [{bounds}] (by decide +kernel) [{request}] rfl\n",
        lowered.otherwise,
        lowered.always,
        conditions_text(&lowered.when),
        lowered.outcome,
    )
}
