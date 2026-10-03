//! The realization library (SPEC.md §17.14): every language-1.2 collection
//! and state-threading primitive realized as target functions.
//!
//! A map `K -> V` is realized as its strictly ascending entry list
//! `List (Pair K V)`, a set as its ascending element list, and a graph as a
//! map from node to successor set, exactly the representation the generated
//! Lean runtime `LexLeanCollections` uses (§17.12). Each template transcribes
//! the corresponding runtime definition clause for clause, comparing keys
//! with the calculus `compare` primitive, so a realization neither adds nor
//! reorders work. Templates are monomorphic: an instance is fixed by its
//! type arguments, as production closures are monomorphized (§17.13).

use super::{Arm, Expr, Function, Prim, Shape, Ty};

/// A library template.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Template {
    MapInsert,
    MapRemove,
    MapLookup,
    MapContains,
    MapKeys,
    MapValues,
    MapFold,
    SetInsert,
    SetRemove,
    SetContains,
    SetUnion,
    SetIntersection,
    SetDifference,
    SetFold,
    ListFold,
    Iterate,
    IterateUntil,
    GraphSuccessors,
    GraphReachable,
    GraphTopological,
}

impl Template {
    /// Every template.
    pub const ALL: [Self; 20] = [
        Self::MapInsert,
        Self::MapRemove,
        Self::MapLookup,
        Self::MapContains,
        Self::MapKeys,
        Self::MapValues,
        Self::MapFold,
        Self::SetInsert,
        Self::SetRemove,
        Self::SetContains,
        Self::SetUnion,
        Self::SetIntersection,
        Self::SetDifference,
        Self::SetFold,
        Self::ListFold,
        Self::Iterate,
        Self::IterateUntil,
        Self::GraphSuccessors,
        Self::GraphReachable,
        Self::GraphTopological,
    ];

    /// The template's name, as the realization table and fixtures spell it.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::MapInsert => "map_insert",
            Self::MapRemove => "map_remove",
            Self::MapLookup => "map_lookup",
            Self::MapContains => "map_contains",
            Self::MapKeys => "map_keys",
            Self::MapValues => "map_values",
            Self::MapFold => "map_fold",
            Self::SetInsert => "set_insert",
            Self::SetRemove => "set_remove",
            Self::SetContains => "set_contains",
            Self::SetUnion => "set_union",
            Self::SetIntersection => "set_intersection",
            Self::SetDifference => "set_difference",
            Self::SetFold => "set_fold",
            Self::ListFold => "list_fold",
            Self::Iterate => "iterate",
            Self::IterateUntil => "iterate_until",
            Self::GraphSuccessors => "graph_successors",
            Self::GraphReachable => "graph_reachable",
            Self::GraphTopological => "graph_topological",
        }
    }

    /// The template named `name`.
    #[must_use]
    pub fn named(name: &str) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|template| template.name() == name)
    }

    /// The number of type arguments: the key (or element, or node, or
    /// state) type, then the value type, then the fold state.
    #[must_use]
    pub const fn arity(self) -> usize {
        match self {
            Self::MapInsert
            | Self::MapRemove
            | Self::MapLookup
            | Self::MapContains
            | Self::MapKeys
            | Self::MapValues
            | Self::SetFold
            | Self::ListFold => 2,
            Self::MapFold => 3,
            Self::SetInsert
            | Self::SetRemove
            | Self::SetContains
            | Self::SetUnion
            | Self::SetIntersection
            | Self::SetDifference
            | Self::Iterate
            | Self::IterateUntil
            | Self::GraphSuccessors
            | Self::GraphReachable
            | Self::GraphTopological => 1,
        }
    }

    /// The functions of one instance placed at function index `at`. The
    /// first function is the entry; the rest are its helpers, which the
    /// instance carries with it.
    ///
    /// # Errors
    ///
    /// Returns the reason the type arguments do not fit the template.
    pub fn instantiate(self, types: &[Ty], at: u64) -> Result<Vec<Function>, String> {
        if types.len() != self.arity() {
            return Err(format!(
                "template {} takes {} type argument(s), received {}",
                self.name(),
                self.arity(),
                types.len()
            ));
        }
        let mut builder = Builder {
            at,
            functions: Vec::new(),
        };
        match self {
            Self::MapInsert => {
                builder.map_insert(&types[0], &types[1]);
            }
            Self::MapRemove => {
                builder.map_remove(&types[0], &types[1]);
            }
            Self::MapLookup => {
                builder.map_lookup(&types[0], &types[1]);
            }
            Self::MapContains => {
                builder.map_contains(&types[0], &types[1]);
            }
            Self::MapKeys => {
                builder.map_project(&types[0], &types[1], true);
            }
            Self::MapValues => {
                builder.map_project(&types[0], &types[1], false);
            }
            Self::MapFold => {
                builder.map_fold(&types[0], &types[1], &types[2]);
            }
            Self::SetInsert => {
                builder.set_insert(&types[0]);
            }
            Self::SetRemove => {
                builder.set_remove(&types[0]);
            }
            Self::SetContains => {
                builder.set_contains(&types[0]);
            }
            Self::SetUnion => {
                builder.set_union(&types[0]);
            }
            Self::SetIntersection => {
                builder.set_filter(&types[0], true);
            }
            Self::SetDifference => {
                builder.set_filter(&types[0], false);
            }
            Self::SetFold | Self::ListFold => {
                builder.list_fold(&types[0], &types[1]);
            }
            Self::Iterate => {
                builder.iterate(&types[0]);
            }
            Self::IterateUntil => {
                builder.iterate_until(&types[0]);
            }
            Self::GraphSuccessors => {
                builder.graph_successors(&types[0]);
            }
            Self::GraphReachable => {
                builder.graph_reachable(&types[0]);
            }
            Self::GraphTopological => {
                builder.graph_topological(&types[0]);
            }
        }
        Ok(builder.functions)
    }
}

fn list(element: &Ty) -> Ty {
    Ty::List {
        element: Box::new(element.clone()),
    }
}

fn pair(left: &Ty, right: &Ty) -> Ty {
    Ty::Pair {
        left: Box::new(left.clone()),
        right: Box::new(right.clone()),
    }
}

fn option(value: &Ty) -> Ty {
    Ty::Option {
        value: Box::new(value.clone()),
    }
}

fn function_type(parameters: &[Ty], result: &Ty) -> Ty {
    Ty::Fn {
        parameters: parameters.to_vec(),
        result: Box::new(result.clone()),
    }
}

fn var(name: u64) -> Expr {
    Expr::Var { name }
}

fn call(function: u64, operands: Vec<Expr>) -> Expr {
    Expr::Call { function, operands }
}

fn prim(operation: Prim, operands: Vec<Expr>) -> Expr {
    Expr::Prim {
        operation,
        operands,
    }
}

fn build(shape: Shape, ty: &Ty, operands: Vec<Expr>) -> Expr {
    Expr::Build {
        shape,
        ty: ty.clone(),
        operands,
    }
}

fn arm(shape: Shape, binders: Vec<u64>, body: Expr) -> Arm {
    Arm {
        shape,
        binders,
        body,
    }
}

fn matching(ty: &Ty, scrutinee: Expr, arms: Vec<Arm>) -> Expr {
    Expr::Match {
        ty: ty.clone(),
        scrutinee: Box::new(scrutinee),
        arms,
    }
}

fn cond(condition: Expr, then_branch: Expr, else_branch: Expr) -> Expr {
    Expr::Cond {
        condition: Box::new(condition),
        then_branch: Box::new(then_branch),
        else_branch: Box::new(else_branch),
    }
}

fn first(value: Expr) -> Expr {
    Expr::First {
        value: Box::new(value),
    }
}

fn second(value: Expr) -> Expr {
    Expr::Second {
        value: Box::new(value),
    }
}

fn apply(target: Expr, operands: Vec<Expr>) -> Expr {
    Expr::Apply {
        target: Box::new(target),
        operands,
    }
}

fn boolean(value: bool) -> Expr {
    build(
        if value { Shape::True } else { Shape::False },
        &Ty::Bool,
        Vec::new(),
    )
}

fn cons(ty: &Ty, head: Expr, tail: Expr) -> Expr {
    build(Shape::Cons, ty, vec![head, tail])
}

fn nil(ty: &Ty) -> Expr {
    build(Shape::Nil, ty, Vec::new())
}

/// `match compare(key, other) with lt => less | eq => equal | gt => greater`.
fn by_order(result: &Ty, key: Expr, other: Expr, less: Expr, equal: Expr, greater: Expr) -> Expr {
    matching(
        result,
        prim(Prim::Compare, vec![key, other]),
        vec![
            arm(Shape::Lt, Vec::new(), less),
            arm(Shape::Eq, Vec::new(), equal),
            arm(Shape::Gt, Vec::new(), greater),
        ],
    )
}

/// The boundary validators (§17.17): Boolean functions an entry calls to
/// decide, at run time, §17.12's invariants of the values it receives. Each
/// is the transcription of the preservation library's `Tpl.valid*Fn`, so a
/// certificate states its index's function equal to the template by `rfl`.
pub mod validators {
    use super::{arm, boolean, call, cond, first, list, matching, pair, prim, second, var};
    use crate::calculus::{Expr, Function, Prim, Shape, Ty};

    fn function(types: Vec<Ty>, body: Expr) -> Function {
        Function {
            parameters: (0..types.len() as u64).collect(),
            types,
            result: Ty::Bool,
            body,
        }
    }

    /// `lessThanE a b`: whether the key order puts `a` strictly first.
    fn less_than(a: Expr, b: Expr) -> Expr {
        matching(
            &Ty::Bool,
            prim(Prim::Compare, vec![a, b]),
            vec![
                arm(Shape::Lt, Vec::new(), boolean(true)),
                arm(Shape::Eq, Vec::new(), boolean(false)),
                arm(Shape::Gt, Vec::new(), boolean(false)),
            ],
        )
    }

    /// `validTrueFn t`: a type without an invariant.
    #[must_use]
    pub fn trivial(ty: &Ty) -> Function {
        function(vec![ty.clone()], boolean(true))
    }

    /// `validListFn this element t`: every element valid.
    #[must_use]
    pub fn list_of(this: u64, element: u64, ty: &Ty) -> Function {
        function(
            vec![list(ty)],
            matching(
                &Ty::Bool,
                var(0),
                vec![
                    arm(Shape::Nil, Vec::new(), boolean(true)),
                    arm(
                        Shape::Cons,
                        vec![1, 2],
                        cond(
                            call(element, vec![var(1)]),
                            call(this, vec![var(2)]),
                            boolean(false),
                        ),
                    ),
                ],
            ),
        )
    }

    /// `validSetFn this k`: the elements strictly ascending.
    #[must_use]
    pub fn set_of(this: u64, key: &Ty) -> Function {
        function(
            vec![list(key)],
            matching(
                &Ty::Bool,
                var(0),
                vec![
                    arm(Shape::Nil, Vec::new(), boolean(true)),
                    arm(
                        Shape::Cons,
                        vec![1, 2],
                        matching(
                            &Ty::Bool,
                            var(2),
                            vec![
                                arm(Shape::Nil, Vec::new(), boolean(true)),
                                arm(
                                    Shape::Cons,
                                    vec![3, 4],
                                    cond(
                                        less_than(var(1), var(3)),
                                        call(this, vec![var(2)]),
                                        boolean(false),
                                    ),
                                ),
                            ],
                        ),
                    ),
                ],
            ),
        )
    }

    /// `validMapFn this value k w`: the keys strictly ascending, every
    /// value valid.
    #[must_use]
    pub fn map_of(this: u64, value: u64, key: &Ty, stored: &Ty) -> Function {
        function(
            vec![list(&pair(key, stored))],
            matching(
                &Ty::Bool,
                var(0),
                vec![
                    arm(Shape::Nil, Vec::new(), boolean(true)),
                    arm(
                        Shape::Cons,
                        vec![1, 2],
                        cond(
                            call(value, vec![second(var(1))]),
                            matching(
                                &Ty::Bool,
                                var(2),
                                vec![
                                    arm(Shape::Nil, Vec::new(), boolean(true)),
                                    arm(
                                        Shape::Cons,
                                        vec![3, 4],
                                        cond(
                                            less_than(first(var(1)), first(var(3))),
                                            call(this, vec![var(2)]),
                                            boolean(false),
                                        ),
                                    ),
                                ],
                            ),
                            boolean(false),
                        ),
                    ),
                ],
            ),
        )
    }

    /// `validOptionFn value t`.
    #[must_use]
    pub fn option_of(value: u64, ty: &Ty) -> Function {
        function(
            vec![Ty::Option {
                value: Box::new(ty.clone()),
            }],
            matching(
                &Ty::Bool,
                var(0),
                vec![
                    arm(Shape::None, Vec::new(), boolean(true)),
                    arm(Shape::Some, vec![1], call(value, vec![var(1)])),
                ],
            ),
        )
    }

    /// `validPairFn left right a b`.
    #[must_use]
    pub fn pair_of(left: u64, right: u64, a: &Ty, b: &Ty) -> Function {
        function(
            vec![pair(a, b)],
            cond(
                call(left, vec![first(var(0))]),
                call(right, vec![second(var(0))]),
                boolean(false),
            ),
        )
    }

    /// `validResultFn ok error a b`.
    #[must_use]
    pub fn result_of(ok: u64, error: u64, a: &Ty, b: &Ty) -> Function {
        function(
            vec![Ty::Result {
                ok: Box::new(a.clone()),
                error: Box::new(b.clone()),
            }],
            matching(
                &Ty::Bool,
                var(0),
                vec![
                    arm(Shape::Ok, vec![1], call(ok, vec![var(1)])),
                    arm(Shape::Error, vec![2], call(error, vec![var(2)])),
                ],
            ),
        )
    }

    /// A document type's validator: the arm of each constructor checks its
    /// fields that carry an invariant, in field order, with the validator
    /// `fields[c][i]`.
    #[must_use]
    pub fn document(adt: u64, fields: &[Vec<Option<u64>>]) -> Function {
        let mut next = 1;
        let arms = fields
            .iter()
            .enumerate()
            .map(|(constructor, validators)| {
                let binders: Vec<u64> = (0..validators.len() as u64).map(|i| next + i).collect();
                next += validators.len() as u64;
                let body = binders.iter().zip(validators).rev().fold(
                    boolean(true),
                    |rest, (binder, validator)| match validator {
                        Some(validator) => {
                            cond(call(*validator, vec![var(*binder)]), rest, boolean(false))
                        }
                        None => rest,
                    },
                );
                arm(
                    Shape::Adt {
                        constructor: constructor as u64,
                    },
                    binders,
                    body,
                )
            })
            .collect();
        function(
            vec![Ty::Adt { index: adt }],
            matching(&Ty::Bool, var(0), arms),
        )
    }
}

/// Collects one instance's functions. Each method pushes its entry first and
/// returns its index, so a helper's index is known before its caller's body
/// is complete.
struct Builder {
    at: u64,
    functions: Vec<Function>,
}

impl Builder {
    /// Reserve the next function index.
    fn reserve(&mut self) -> u64 {
        let index = self.at + self.functions.len() as u64;
        self.functions.push(Function {
            parameters: Vec::new(),
            types: Vec::new(),
            result: Ty::Unit,
            body: Expr::Var { name: 0 },
        });
        index
    }

    fn define(&mut self, index: u64, types: Vec<Ty>, result: Ty, body: Expr) {
        let position = usize::try_from(index - self.at).expect("a reserved index");
        self.functions[position] = Function {
            parameters: (0..types.len() as u64).collect(),
            types,
            result,
            body,
        };
    }

    /// `insertEntry key value entries`.
    fn map_insert(&mut self, key: &Ty, value: &Ty) -> u64 {
        let this = self.reserve();
        let entry = pair(key, value);
        let entries = list(&entry);
        // 0 entries, 1 key, 2 value; 3 head, 4 tail.
        let new_entry = || build(Shape::Pair, &entry, vec![var(1), var(2)]);
        let body = matching(
            &entries,
            var(0),
            vec![
                arm(
                    Shape::Nil,
                    Vec::new(),
                    cons(&entries, new_entry(), nil(&entries)),
                ),
                arm(
                    Shape::Cons,
                    vec![3, 4],
                    by_order(
                        &entries,
                        var(1),
                        first(var(3)),
                        cons(&entries, new_entry(), var(0)),
                        cons(&entries, new_entry(), var(4)),
                        cons(&entries, var(3), call(this, vec![var(4), var(1), var(2)])),
                    ),
                ),
            ],
        );
        self.define(
            this,
            vec![entries.clone(), key.clone(), value.clone()],
            entries,
            body,
        );
        this
    }

    /// `removeEntry key entries`.
    fn map_remove(&mut self, key: &Ty, value: &Ty) -> u64 {
        let this = self.reserve();
        let entries = list(&pair(key, value));
        let body = matching(
            &entries,
            var(0),
            vec![
                arm(Shape::Nil, Vec::new(), nil(&entries)),
                arm(
                    Shape::Cons,
                    vec![2, 3],
                    by_order(
                        &entries,
                        var(1),
                        first(var(2)),
                        var(0),
                        var(3),
                        cons(&entries, var(2), call(this, vec![var(3), var(1)])),
                    ),
                ),
            ],
        );
        self.define(this, vec![entries.clone(), key.clone()], entries, body);
        this
    }

    /// `lookupEntry key entries`.
    fn map_lookup(&mut self, key: &Ty, value: &Ty) -> u64 {
        let this = self.reserve();
        let entries = list(&pair(key, value));
        let found = option(value);
        let body = matching(
            &found,
            var(0),
            vec![
                arm(
                    Shape::Nil,
                    Vec::new(),
                    build(Shape::None, &found, Vec::new()),
                ),
                arm(
                    Shape::Cons,
                    vec![2, 3],
                    by_order(
                        &found,
                        var(1),
                        first(var(2)),
                        build(Shape::None, &found, Vec::new()),
                        build(Shape::Some, &found, vec![second(var(2))]),
                        call(this, vec![var(3), var(1)]),
                    ),
                ),
            ],
        );
        self.define(this, vec![entries, key.clone()], found, body);
        this
    }

    /// `(lookupEntry key entries).isSome`.
    fn map_contains(&mut self, key: &Ty, value: &Ty) -> u64 {
        let this = self.reserve();
        let lookup = self.map_lookup(key, value);
        let body = matching(
            &Ty::Bool,
            call(lookup, vec![var(0), var(1)]),
            vec![
                arm(Shape::None, Vec::new(), boolean(false)),
                arm(Shape::Some, vec![2], boolean(true)),
            ],
        );
        self.define(
            this,
            vec![list(&pair(key, value)), key.clone()],
            Ty::Bool,
            body,
        );
        this
    }

    /// `entries.map Prod.fst` or `entries.map Prod.snd`.
    fn map_project(&mut self, key: &Ty, value: &Ty, keys: bool) -> u64 {
        let this = self.reserve();
        let element = if keys { key } else { value };
        let result = list(element);
        let projected = if keys { first(var(1)) } else { second(var(1)) };
        let body = matching(
            &result,
            var(0),
            vec![
                arm(Shape::Nil, Vec::new(), nil(&result)),
                arm(
                    Shape::Cons,
                    vec![1, 2],
                    cons(&result, projected, call(this, vec![var(2)])),
                ),
            ],
        );
        self.define(this, vec![list(&pair(key, value))], result, body);
        this
    }

    /// `entries.foldl (fun state entry => step state entry.1 entry.2) initial`.
    fn map_fold(&mut self, key: &Ty, value: &Ty, state: &Ty) -> u64 {
        let this = self.reserve();
        let step = function_type(&[state.clone(), key.clone(), value.clone()], state);
        // 0 step, 1 state, 2 entries; 3 head, 4 tail.
        let body = matching(
            state,
            var(2),
            vec![
                arm(Shape::Nil, Vec::new(), var(1)),
                arm(
                    Shape::Cons,
                    vec![3, 4],
                    call(
                        this,
                        vec![
                            var(0),
                            apply(var(0), vec![var(1), first(var(3)), second(var(3))]),
                            var(4),
                        ],
                    ),
                ),
            ],
        );
        self.define(
            this,
            vec![step, state.clone(), list(&pair(key, value))],
            state.clone(),
            body,
        );
        this
    }

    /// `insertElement key elements`.
    fn set_insert(&mut self, key: &Ty) -> u64 {
        let this = self.reserve();
        let set = list(key);
        let body = matching(
            &set,
            var(0),
            vec![
                arm(Shape::Nil, Vec::new(), cons(&set, var(1), nil(&set))),
                arm(
                    Shape::Cons,
                    vec![2, 3],
                    by_order(
                        &set,
                        var(1),
                        var(2),
                        cons(&set, var(1), var(0)),
                        var(0),
                        cons(&set, var(2), call(this, vec![var(3), var(1)])),
                    ),
                ),
            ],
        );
        self.define(this, vec![set.clone(), key.clone()], set, body);
        this
    }

    /// `removeElement key elements`.
    fn set_remove(&mut self, key: &Ty) -> u64 {
        let this = self.reserve();
        let set = list(key);
        let body = matching(
            &set,
            var(0),
            vec![
                arm(Shape::Nil, Vec::new(), nil(&set)),
                arm(
                    Shape::Cons,
                    vec![2, 3],
                    by_order(
                        &set,
                        var(1),
                        var(2),
                        var(0),
                        var(3),
                        cons(&set, var(2), call(this, vec![var(3), var(1)])),
                    ),
                ),
            ],
        );
        self.define(this, vec![set.clone(), key.clone()], set, body);
        this
    }

    /// `containsElement key elements`.
    fn set_contains(&mut self, key: &Ty) -> u64 {
        let this = self.reserve();
        let body = matching(
            &Ty::Bool,
            var(0),
            vec![
                arm(Shape::Nil, Vec::new(), boolean(false)),
                arm(
                    Shape::Cons,
                    vec![2, 3],
                    by_order(
                        &Ty::Bool,
                        var(1),
                        var(2),
                        boolean(false),
                        boolean(true),
                        call(this, vec![var(3), var(1)]),
                    ),
                ),
            ],
        );
        self.define(this, vec![list(key), key.clone()], Ty::Bool, body);
        this
    }

    /// `right.foldl (fun acc key => insertElement key acc) left`.
    fn set_union(&mut self, key: &Ty) -> u64 {
        let this = self.reserve();
        let insert = self.set_insert(key);
        let set = list(key);
        // 0 left (the accumulator), 1 right; 2 head, 3 tail.
        let body = matching(
            &set,
            var(1),
            vec![
                arm(Shape::Nil, Vec::new(), var(0)),
                arm(
                    Shape::Cons,
                    vec![2, 3],
                    call(this, vec![call(insert, vec![var(0), var(2)]), var(3)]),
                ),
            ],
        );
        self.define(this, vec![set.clone(), set.clone()], set, body);
        this
    }

    /// `left.filter (fun key => containsElement key right)`, or its negation.
    fn set_filter(&mut self, key: &Ty, keep_contained: bool) -> u64 {
        let this = self.reserve();
        let contains = self.set_contains(key);
        let set = list(key);
        // 0 left, 1 right; 2 head, 3 tail.
        let rest = || call(this, vec![var(3), var(1)]);
        let kept = cons(&set, var(2), rest());
        let (when_contained, otherwise) = if keep_contained {
            (kept, rest())
        } else {
            (rest(), kept)
        };
        let body = matching(
            &set,
            var(0),
            vec![
                arm(Shape::Nil, Vec::new(), nil(&set)),
                arm(
                    Shape::Cons,
                    vec![2, 3],
                    cond(
                        call(contains, vec![var(1), var(2)]),
                        when_contained,
                        otherwise,
                    ),
                ),
            ],
        );
        self.define(this, vec![set.clone(), set.clone()], set, body);
        this
    }

    /// `values.foldl step initial`.
    fn list_fold(&mut self, element: &Ty, state: &Ty) -> u64 {
        let this = self.reserve();
        let step = function_type(&[state.clone(), element.clone()], state);
        let body = matching(
            state,
            var(2),
            vec![
                arm(Shape::Nil, Vec::new(), var(1)),
                arm(
                    Shape::Cons,
                    vec![3, 4],
                    call(
                        this,
                        vec![var(0), apply(var(0), vec![var(1), var(3)]), var(4)],
                    ),
                ),
            ],
        );
        self.define(
            this,
            vec![step, state.clone(), list(element)],
            state.clone(),
            body,
        );
        this
    }

    /// `iterate step count state`.
    fn iterate(&mut self, state: &Ty) -> u64 {
        let this = self.reserve();
        let step = function_type(std::slice::from_ref(state), state);
        // 0 step, 1 count, 2 state; 3 predecessor.
        let body = matching(
            state,
            var(1),
            vec![
                arm(Shape::Zero, Vec::new(), var(2)),
                arm(
                    Shape::Succ,
                    vec![3],
                    call(this, vec![var(0), var(3), apply(var(0), vec![var(2)])]),
                ),
            ],
        );
        self.define(
            this,
            vec![step, Ty::Nat, state.clone()],
            state.clone(),
            body,
        );
        this
    }

    /// `iterateUntil step fuel state`.
    fn iterate_until(&mut self, state: &Ty) -> u64 {
        let this = self.reserve();
        let stepped = option(state);
        let step = function_type(std::slice::from_ref(state), &stepped);
        let result = pair(state, &Ty::Bool);
        // 0 step, 1 fuel, 2 state; 3 predecessor, 4 next.
        let finished = |reached: bool| build(Shape::Pair, &result, vec![var(2), boolean(reached)]);
        let body = matching(
            &result,
            var(1),
            vec![
                arm(Shape::Zero, Vec::new(), finished(false)),
                arm(
                    Shape::Succ,
                    vec![3],
                    matching(
                        &result,
                        apply(var(0), vec![var(2)]),
                        vec![
                            arm(Shape::None, Vec::new(), finished(true)),
                            arm(
                                Shape::Some,
                                vec![4],
                                call(this, vec![var(0), var(3), var(4)]),
                            ),
                        ],
                    ),
                ),
            ],
        );
        self.define(this, vec![step, Ty::Nat, state.clone()], result, body);
        this
    }

    /// `(lookupEntry node graph).getD []`.
    fn graph_successors(&mut self, node: &Ty) -> u64 {
        let this = self.reserve();
        let nodes = list(node);
        let lookup = self.map_lookup(node, &nodes);
        let body = matching(
            &nodes,
            call(lookup, vec![var(0), var(1)]),
            vec![
                arm(Shape::None, Vec::new(), nil(&nodes)),
                arm(Shape::Some, vec![2], var(2)),
            ],
        );
        self.define(
            this,
            vec![list(&pair(node, &nodes)), node.clone()],
            nodes,
            body,
        );
        this
    }

    /// `graphNodes graph`: every key and every successor, ascending, as
    /// `graph.foldl (fun acc entry => entry.2.foldl (fun inner node =>
    /// insertElement node inner) (insertElement entry.1 acc)) []`, applied
    /// to the accumulator it is given.
    fn graph_nodes(&mut self, node: &Ty) -> u64 {
        let this = self.reserve();
        let inner = self.reserve();
        let insert = self.set_insert(node);
        let nodes = list(node);
        let graph = list(&pair(node, &nodes));
        // The entry fold: 0 graph, 1 acc; 2 entry, 3 rest.
        let body = matching(
            &nodes,
            var(0),
            vec![
                arm(Shape::Nil, Vec::new(), var(1)),
                arm(
                    Shape::Cons,
                    vec![2, 3],
                    call(
                        this,
                        vec![
                            var(3),
                            call(
                                inner,
                                vec![second(var(2)), call(insert, vec![var(1), first(var(2))])],
                            ),
                        ],
                    ),
                ),
            ],
        );
        self.define(this, vec![graph, nodes.clone()], nodes.clone(), body);
        // The successor fold: 0 successors, 1 acc; 2 node, 3 rest.
        let body = matching(
            &nodes,
            var(0),
            vec![
                arm(Shape::Nil, Vec::new(), var(1)),
                arm(
                    Shape::Cons,
                    vec![2, 3],
                    call(inner, vec![var(3), call(insert, vec![var(1), var(2)])]),
                ),
            ],
        );
        self.define(inner, vec![nodes.clone(), nodes.clone()], nodes, body);
        this
    }

    /// `reachableFrom graph ((graphNodes graph).length + 1) [start] [start]`.
    fn graph_reachable(&mut self, node: &Ty) -> u64 {
        let this = self.reserve();
        let rounds = self.reserve();
        let frontier = self.reserve();
        let successors_into = self.reserve();
        let successors = self.graph_successors(node);
        let contains = self.set_contains(node);
        let insert = self.set_insert(node);
        let union = self.set_union(node);
        let all_nodes = self.graph_nodes(node);
        let nodes = list(node);
        let graph = list(&pair(node, &nodes));
        let singleton = || cons(&nodes, var(1), nil(&nodes));
        // graphReachable: 0 graph, 1 start. The bound counts every node,
        // successors without their own entry included, as the Lean
        // rendering's does.
        let bound = prim(
            Prim::NatAdd,
            vec![
                prim(
                    Prim::Length,
                    vec![call(all_nodes, vec![var(0), nil(&nodes)])],
                ),
                Expr::Value {
                    ty: Ty::Nat,
                    value: super::Value::Nat {
                        value: "1".to_owned(),
                    },
                },
            ],
        );
        self.define(
            this,
            vec![graph.clone(), node.clone()],
            nodes.clone(),
            call(rounds, vec![var(0), bound, singleton(), singleton()]),
        );
        // reachableFrom: 0 graph, 1 fuel, 2 frontier, 3 seen; 4 fuel', 5 next, 6 7 its cons.
        let next = call(frontier, vec![var(0), var(3), var(2), nil(&nodes)]);
        let body = matching(
            &nodes,
            var(1),
            vec![
                arm(Shape::Zero, Vec::new(), var(3)),
                arm(
                    Shape::Succ,
                    vec![4],
                    Expr::Let {
                        name: 5,
                        ty: nodes.clone(),
                        bound: Box::new(next),
                        body: Box::new(matching(
                            &nodes,
                            var(5),
                            vec![
                                arm(Shape::Nil, Vec::new(), var(3)),
                                arm(
                                    Shape::Cons,
                                    vec![6, 7],
                                    call(
                                        rounds,
                                        vec![
                                            var(0),
                                            var(4),
                                            var(5),
                                            call(union, vec![var(3), var(5)]),
                                        ],
                                    ),
                                ),
                            ],
                        )),
                    },
                ),
            ],
        );
        self.define(
            rounds,
            vec![graph.clone(), Ty::Nat, nodes.clone(), nodes.clone()],
            nodes.clone(),
            body,
        );
        // The frontier fold: 0 graph, 1 seen, 2 frontier, 3 acc; 4 node, 5 rest.
        let body = matching(
            &nodes,
            var(2),
            vec![
                arm(Shape::Nil, Vec::new(), var(3)),
                arm(
                    Shape::Cons,
                    vec![4, 5],
                    call(
                        frontier,
                        vec![
                            var(0),
                            var(1),
                            var(5),
                            call(
                                successors_into,
                                vec![var(1), call(successors, vec![var(0), var(4)]), var(3)],
                            ),
                        ],
                    ),
                ),
            ],
        );
        self.define(
            frontier,
            vec![graph, nodes.clone(), nodes.clone(), nodes.clone()],
            nodes.clone(),
            body,
        );
        // The successor fold: 0 seen, 1 successors, 2 acc; 3 succ, 4 rest.
        let unseen = cond(
            call(contains, vec![var(0), var(3)]),
            var(2),
            cond(
                call(contains, vec![var(2), var(3)]),
                var(2),
                call(insert, vec![var(2), var(3)]),
            ),
        );
        let body = matching(
            &nodes,
            var(1),
            vec![
                arm(Shape::Nil, Vec::new(), var(2)),
                arm(
                    Shape::Cons,
                    vec![3, 4],
                    call(successors_into, vec![var(0), var(4), unseen]),
                ),
            ],
        );
        self.define(
            successors_into,
            vec![nodes.clone(), nodes.clone(), nodes.clone()],
            nodes,
            body,
        );
        this
    }

    /// `let nodes := graphNodes graph; topological graph (nodes.length + 1) nodes []`.
    fn graph_topological(&mut self, node: &Ty) -> u64 {
        let this = self.reserve();
        let rounds = self.reserve();
        let ready = self.reserve();
        let unreached = self.reserve();
        let reverse = self.reserve();
        let all_nodes = self.graph_nodes(node);
        let successors = self.graph_successors(node);
        let contains = self.set_contains(node);
        let remove = self.set_remove(node);
        let nodes = list(node);
        let graph = list(&pair(node, &nodes));
        let order = option(&nodes);
        // graphTopological: 0 graph; 1 its nodes.
        let bound = prim(
            Prim::NatAdd,
            vec![
                prim(Prim::Length, vec![var(1)]),
                Expr::Value {
                    ty: Ty::Nat,
                    value: super::Value::Nat {
                        value: "1".to_owned(),
                    },
                },
            ],
        );
        self.define(
            this,
            vec![graph.clone()],
            order.clone(),
            Expr::Let {
                name: 1,
                ty: nodes.clone(),
                bound: Box::new(call(all_nodes, vec![var(0), nil(&nodes)])),
                body: Box::new(call(rounds, vec![var(0), bound, var(1), nil(&nodes)])),
            },
        );
        // topological: 0 graph, 1 fuel, 2 remaining, 3 order. Binders are
        // numbered in first-binding order, so the instance is canonical:
        // 4 5 the first finish's cons, 6 fuel', 7 8 the second finish's
        // cons, 9 10 the ready cons.
        let finished = |first: u64| {
            matching(
                &order,
                var(2),
                vec![
                    arm(
                        Shape::Nil,
                        Vec::new(),
                        build(
                            Shape::Some,
                            &order,
                            vec![call(reverse, vec![var(3), nil(&nodes)])],
                        ),
                    ),
                    arm(
                        Shape::Cons,
                        vec![first, first + 1],
                        build(Shape::None, &order, Vec::new()),
                    ),
                ],
            )
        };
        let body = matching(
            &order,
            var(1),
            vec![
                arm(Shape::Zero, Vec::new(), finished(4)),
                arm(
                    Shape::Succ,
                    vec![6],
                    matching(
                        &order,
                        call(ready, vec![var(0), var(2), var(2)]),
                        vec![
                            arm(Shape::Nil, Vec::new(), finished(7)),
                            arm(
                                Shape::Cons,
                                vec![9, 10],
                                call(
                                    rounds,
                                    vec![
                                        var(0),
                                        var(6),
                                        call(remove, vec![var(2), var(9)]),
                                        cons(&nodes, var(9), var(3)),
                                    ],
                                ),
                            ),
                        ],
                    ),
                ),
            ],
        );
        self.define(
            rounds,
            vec![graph.clone(), Ty::Nat, nodes.clone(), nodes.clone()],
            order,
            body,
        );
        // The ready filter: 0 graph, 1 remaining, 2 candidates; 3 node, 4 rest.
        let rest = || call(ready, vec![var(0), var(1), var(4)]);
        let body = matching(
            &nodes,
            var(2),
            vec![
                arm(Shape::Nil, Vec::new(), nil(&nodes)),
                arm(
                    Shape::Cons,
                    vec![3, 4],
                    cond(
                        call(unreached, vec![var(0), var(3), var(1)]),
                        cons(&nodes, var(3), rest()),
                        rest(),
                    ),
                ),
            ],
        );
        self.define(
            ready,
            vec![graph.clone(), nodes.clone(), nodes.clone()],
            nodes.clone(),
            body,
        );
        // `remaining.all (fun other => !containsElement node (graphSuccessors graph other))`:
        // 0 graph, 1 node, 2 others; 3 other, 4 rest.
        let body = matching(
            &Ty::Bool,
            var(2),
            vec![
                arm(Shape::Nil, Vec::new(), boolean(true)),
                arm(
                    Shape::Cons,
                    vec![3, 4],
                    cond(
                        call(
                            contains,
                            vec![call(successors, vec![var(0), var(3)]), var(1)],
                        ),
                        boolean(false),
                        call(unreached, vec![var(0), var(1), var(4)]),
                    ),
                ),
            ],
        );
        self.define(
            unreached,
            vec![graph, node.clone(), nodes.clone()],
            Ty::Bool,
            body,
        );
        // `order.reverse`, onto an accumulator: 0 items, 1 acc; 2 head, 3 tail.
        let body = matching(
            &nodes,
            var(0),
            vec![
                arm(Shape::Nil, Vec::new(), var(1)),
                arm(
                    Shape::Cons,
                    vec![2, 3],
                    call(reverse, vec![var(3), cons(&nodes, var(2), var(1))]),
                ),
            ],
        );
        self.define(reverse, vec![nodes.clone(), nodes.clone()], nodes, body);
        this
    }
}

#[cfg(test)]
mod tests {
    use super::{Template, Ty};
    use crate::calculus::{Program, PROGRAM_SPEC};

    /// Every instance is already in first-binding order, so a lowered
    /// program that places one is canonical as placed.
    #[test]
    fn every_instance_is_canonical() {
        for template in Template::ALL {
            let program = Program {
                spec: PROGRAM_SPEC.to_owned(),
                adts: Vec::new(),
                functions: template
                    .instantiate(&vec![Ty::Nat; template.arity()], 0)
                    .expect("instantiates"),
            };
            assert_eq!(
                program.canonical().expect("valid"),
                program,
                "{} is not in first-binding order",
                template.name()
            );
        }
    }
}
