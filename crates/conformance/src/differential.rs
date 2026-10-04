//! The differential evaluator of the lowering (SPEC.md §17.17): every
//! production root is run on seeded inputs twice, once by the calculus
//! interpreter on the lowered program and once by Lean on the generated
//! source definition, and every value the interpreter returns must be the
//! value Lean computes. It is evidence, not proof: certificate A proves the
//! same agreement for every input against the calculus's Lean semantics,
//! while this checks the Rust interpreter against Lean on samples.

use std::cmp::Ordering;
use std::collections::BTreeMap;

use lexlean::calculus::{interp, OrderingValue, Outcome, Value};
use lexlean::ir::semantic::SemanticType;
use lexlean::production::certificate::{root_signature, source_type};
use lexlean::production::eligibility::LinkedModule;
use lexlean::production::lower::{linked_modules, lower_root, roots, Lowered};

use crate::support::{self, P};

/// The interpreter's fuel: every sampled input finishes well within it.
const FUEL: u64 = 10_000_000;

/// Inputs per root.
const SAMPLES: u64 = 6;

/// A deterministic generator (xorshift64*), so a failure reproduces.
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;
        self.0.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }

    fn below(&mut self, bound: u64) -> u64 {
        self.next() % bound
    }
}

type Modules<'a> = BTreeMap<String, LinkedModule<'a>>;

/// The source value order of keys, on their calculus encodings: numbers by
/// value, strings by code points, booleans false first, pairs
/// lexicographically.
fn key_order(left: &Value, right: &Value) -> Ordering {
    let number = |text: &str| text.parse::<i128>().expect("a sampled number");
    match (left, right) {
        (Value::Nat { value: a }, Value::Nat { value: b })
        | (Value::Int { value: a }, Value::Int { value: b })
        | (Value::U8 { value: a }, Value::U8 { value: b })
        | (Value::U16 { value: a }, Value::U16 { value: b })
        | (Value::U32 { value: a }, Value::U32 { value: b })
        | (Value::U64 { value: a }, Value::U64 { value: b })
        | (Value::I8 { value: a }, Value::I8 { value: b })
        | (Value::I16 { value: a }, Value::I16 { value: b })
        | (Value::I32 { value: a }, Value::I32 { value: b })
        | (Value::I64 { value: a }, Value::I64 { value: b }) => number(a).cmp(&number(b)),
        (Value::String { value: a }, Value::String { value: b }) => a.chars().cmp(b.chars()),
        (Value::Bool { value: a }, Value::Bool { value: b }) => a.cmp(b),
        (
            Value::Pair {
                left: a1,
                right: a2,
            },
            Value::Pair {
                left: b1,
                right: b2,
            },
        ) => key_order(a1, b1).then_with(|| key_order(a2, b2)),
        _ => panic!("{left:?} and {right:?} are not keys of one type"),
    }
}

fn sorted_unique(mut items: Vec<Value>, key: impl Fn(&Value) -> &Value) -> Vec<Value> {
    items.sort_by(|a, b| key_order(key(a), key(b)));
    items.dedup_by(|a, b| key_order(key(a), key(b)) == Ordering::Equal);
    items
}

fn first_of(value: &Value) -> &Value {
    match value {
        Value::Pair { left, right: _ } => left,
        _ => panic!("a map entry is a pair"),
    }
}

/// A sampled value of `ty`, small enough that most computations stay in
/// range and deep enough to exercise every constructor.
/// Where a sampled scalar lies: anywhere small, or at the top or bottom of
/// its type, the inputs on which an overflow is possible (§17.17).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Extreme {
    Small,
    /// Larger numbers and longer sequences than the small seeded inputs,
    /// so recursion and iteration run through more than a few steps.
    Medium,
    High,
    Low,
    /// Each scalar independently small, at the top, or at the bottom.
    Mixed,
}

fn sample(
    modules: &Modules<'_>,
    rng: &mut Rng,
    ty: &SemanticType,
    depth: u32,
    extreme: Extreme,
) -> Value {
    let small = |rng: &mut Rng, span: u64| rng.below(span).to_string();
    let signed = |rng: &mut Rng, span: u64| {
        let magnitude = i128::from(rng.below(span));
        (if rng.below(2) == 0 {
            magnitude
        } else {
            -magnitude
        })
        .to_string()
    };
    let here = match extreme {
        Extreme::Mixed => [Extreme::Small, Extreme::High, Extreme::Low][rng.below(3) as usize],
        fixed @ (Extreme::Small | Extreme::Medium | Extreme::High | Extreme::Low) => fixed,
    };
    let high = here == Extreme::High;
    let bound = matches!(here, Extreme::High | Extreme::Low);
    let medium = extreme == Extreme::Medium;
    let length = |rng: &mut Rng| {
        if depth >= 3 {
            0
        } else {
            rng.below(if medium { 9 } else { 4 }) as usize
        }
    };
    match ty {
        SemanticType::Nat => Value::Nat {
            value: if bound {
                if high { u64::MAX } else { 0 }.to_string()
            } else {
                small(rng, if medium { 3000 } else { 40 })
            },
        },
        SemanticType::Int => Value::Int {
            value: if bound {
                if high { i64::MAX } else { i64::MIN }.to_string()
            } else {
                signed(rng, if medium { 3000 } else { 60 })
            },
        },
        SemanticType::Int8 => Value::I8 {
            value: if bound {
                if high { i8::MAX } else { i8::MIN }.to_string()
            } else {
                signed(rng, 100)
            },
        },
        SemanticType::Int16 => Value::I16 {
            value: if bound {
                if high { i16::MAX } else { i16::MIN }.to_string()
            } else {
                signed(rng, 300)
            },
        },
        SemanticType::Int32 => Value::I32 {
            value: if bound {
                if high { i32::MAX } else { i32::MIN }.to_string()
            } else {
                signed(rng, 100_000)
            },
        },
        SemanticType::Int64 => Value::I64 {
            value: if bound {
                if high { i64::MAX } else { i64::MIN }.to_string()
            } else {
                signed(rng, 1_000_000)
            },
        },
        SemanticType::UInt8 => Value::U8 {
            value: if bound {
                if high { u8::MAX } else { 0 }.to_string()
            } else {
                small(rng, 256)
            },
        },
        SemanticType::UInt16 => Value::U16 {
            value: if bound {
                if high { u16::MAX } else { 0 }.to_string()
            } else {
                small(rng, 65_536)
            },
        },
        SemanticType::UInt32 => Value::U32 {
            value: if bound {
                if high { u32::MAX } else { 0 }.to_string()
            } else {
                small(rng, 100_000)
            },
        },
        SemanticType::UInt64 => Value::U64 {
            value: if bound {
                if high { u64::MAX } else { 0 }.to_string()
            } else {
                small(rng, 1_000_000)
            },
        },
        SemanticType::Bool => Value::Bool {
            value: rng.below(2) == 0,
        },
        SemanticType::Unit => Value::Unit,
        SemanticType::Ordering => Value::Ordering {
            value: match rng.below(3) {
                0 => OrderingValue::Lt,
                1 => OrderingValue::Eq,
                _ => OrderingValue::Gt,
            },
        },
        SemanticType::String => {
            const ALPHABET: [&str; 8] = ["a", "b", "z", ",", " ", "é", "7", "Σ"];
            let count = rng.below(5);
            Value::String {
                value: (0..count)
                    .map(|_| ALPHABET[rng.below(ALPHABET.len() as u64) as usize])
                    .collect(),
            }
        }
        SemanticType::Bytes => {
            let count = rng.below(5);
            Value::Bytes {
                hex: (0..count)
                    .map(|_| format!("{:02x}", rng.below(256)))
                    .collect(),
            }
        }
        SemanticType::Option { value } => {
            if rng.below(3) == 0 {
                Value::None
            } else {
                Value::Some {
                    value: Box::new(sample(modules, rng, value, depth + 1, extreme)),
                }
            }
        }
        SemanticType::Result { ok, error } => {
            if rng.below(2) == 0 {
                Value::Ok {
                    value: Box::new(sample(modules, rng, ok, depth + 1, extreme)),
                }
            } else {
                Value::Error {
                    value: Box::new(sample(modules, rng, error, depth + 1, extreme)),
                }
            }
        }
        SemanticType::List { element } => {
            let count = length(rng);
            Value::List {
                items: (0..count)
                    .map(|_| sample(modules, rng, element, depth + 1, extreme))
                    .collect(),
            }
        }
        SemanticType::Set { element } => {
            let count = length(rng) + 1;
            let items = (0..count)
                .map(|_| sample(modules, rng, element, depth + 1, extreme))
                .collect();
            Value::List {
                items: sorted_unique(items, |item| item),
            }
        }
        SemanticType::Map { key, value } => {
            let count = length(rng) + 1;
            let items = (0..count)
                .map(|_| Value::Pair {
                    left: Box::new(sample(modules, rng, key, depth + 1, extreme)),
                    right: Box::new(sample(modules, rng, value, depth + 1, extreme)),
                })
                .collect();
            Value::List {
                items: sorted_unique(items, first_of),
            }
        }
        SemanticType::Product { left, right } => Value::Pair {
            left: Box::new(sample(modules, rng, left, depth + 1, extreme)),
            right: Box::new(sample(modules, rng, right, depth + 1, extreme)),
        },
        // A violation is the pair of Booleans it lowers to (§17.12 rule 9).
        SemanticType::ContractViolation => Value::Pair {
            left: Box::new(Value::Bool {
                value: rng.below(2) == 0,
            }),
            right: Box::new(Value::Bool {
                value: rng.below(2) == 0,
            }),
        },
        SemanticType::Named {
            member: _,
            arguments: _,
        } => {
            let (_, constructors) = source_type(modules, ty).expect("a document type");
            // Past the depth bound the constructor with the fewest fields
            // ends the value; each field is then sampled at the bound, so
            // lists stay empty and recursion stops.
            let chosen = if depth >= 3 {
                constructors
                    .iter()
                    .enumerate()
                    .min_by_key(|(_, constructor)| constructor.fields.len())
                    .map(|(index, _)| index)
                    .expect("a constructor")
            } else {
                rng.below(constructors.len() as u64) as usize
            };
            Value::Adt {
                constructor: chosen as u64,
                fields: constructors[chosen]
                    .fields
                    .iter()
                    .map(|field| sample(modules, rng, field, depth + 1, extreme))
                    .collect(),
            }
        }
        SemanticType::Function {
            parameters: _,
            result: _,
        }
        | SemanticType::Type
        | SemanticType::Prop
        | SemanticType::Parameter { name: _ } => panic!("{ty:?} is not a boundary type"),
    }
}

fn lean_string(text: &str) -> String {
    let mut out = String::from("\"");
    for character in text.chars() {
        match character {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            other => out.push(other),
        }
    }
    out.push('"');
    out
}

/// The Lean term of the source value `value` encodes at type `ty`.
fn lean_term(modules: &Modules<'_>, ty: &SemanticType, value: &Value) -> String {
    let (lean, constructors) = source_type(modules, ty).expect("a closed type");
    let typed = |term: String| format!("({term} : {lean})");
    let element = |ty: &SemanticType, items: &[Value]| {
        typed(format!(
            "[{}]",
            items
                .iter()
                .map(|item| lean_term(modules, ty, item))
                .collect::<Vec<_>>()
                .join(", ")
        ))
    };
    match (ty, value) {
        (SemanticType::Nat, Value::Nat { value })
        | (SemanticType::Int, Value::Int { value })
        | (SemanticType::Int8, Value::I8 { value })
        | (SemanticType::Int16, Value::I16 { value })
        | (SemanticType::Int32, Value::I32 { value })
        | (SemanticType::Int64, Value::I64 { value })
        | (SemanticType::UInt8, Value::U8 { value })
        | (SemanticType::UInt16, Value::U16 { value })
        | (SemanticType::UInt32, Value::U32 { value })
        | (SemanticType::UInt64, Value::U64 { value }) => typed(value.clone()),
        (SemanticType::Bool, Value::Bool { value }) => value.to_string(),
        (SemanticType::Unit, Value::Unit) => "()".to_owned(),
        (SemanticType::Ordering, Value::Ordering { value }) => match value {
            OrderingValue::Lt => "Ordering.lt".to_owned(),
            OrderingValue::Eq => "Ordering.eq".to_owned(),
            OrderingValue::Gt => "Ordering.gt".to_owned(),
        },
        (SemanticType::String, Value::String { value }) => lean_string(value),
        (SemanticType::Bytes, Value::Bytes { hex }) => {
            let octets: Vec<String> = (0..hex.len() / 2)
                .map(|index| {
                    u8::from_str_radix(&hex[2 * index..2 * index + 2], 16)
                        .expect("hex")
                        .to_string()
                })
                .collect();
            format!("(ByteArray.mk #[{}])", octets.join(", "))
        }
        (SemanticType::Option { value: _ }, Value::None) => typed("none".to_owned()),
        (SemanticType::Option { value: inner }, Value::Some { value }) => {
            typed(format!("some {}", lean_term(modules, inner, value)))
        }
        (SemanticType::Result { ok, error: _ }, Value::Ok { value }) => {
            typed(format!("Except.ok {}", lean_term(modules, ok, value)))
        }
        (SemanticType::Result { ok: _, error }, Value::Error { value }) => {
            typed(format!("Except.error {}", lean_term(modules, error, value)))
        }
        (
            SemanticType::List { element: inner } | SemanticType::Set { element: inner },
            Value::List { items },
        ) => element(inner, items),
        (SemanticType::Map { key, value: stored }, Value::List { items }) => element(
            &SemanticType::Product {
                left: key.clone(),
                right: stored.clone(),
            },
            items,
        ),
        (SemanticType::Product { left, right }, Value::Pair { left: a, right: b }) => {
            typed(format!(
                "({}, {})",
                lean_term(modules, left, a),
                lean_term(modules, right, b)
            ))
        }
        (
            SemanticType::Named {
                member: _,
                arguments: _,
            },
            Value::Adt {
                constructor,
                fields,
            },
        ) => {
            let constructor = &constructors[usize::try_from(*constructor).expect("an index")];
            let mut term = constructor.lean.clone();
            for (field, value) in constructor.fields.iter().zip(fields) {
                term.push(' ');
                term.push_str(&lean_term(modules, field, value));
            }
            typed(term)
        }
        _ => panic!("{value:?} is not a value of {lean}"),
    }
}

/// One differential case.
#[derive(Debug, Clone)]
pub struct Case {
    /// The root's qualified Lean name.
    pub root: String,
    /// The Lean terms of the inputs.
    pub arguments: Vec<String>,
    /// The inputs as calculus values, which the lowered program and its
    /// crates receive.
    pub values: Vec<Value>,
    /// The interpreter's outcome.
    pub outcome: Outcome,
    /// The function invoked: the root, function 0, or its entry (§17.17).
    pub function: u64,
    /// The parameter whose §17.12 invariant the input was made to break,
    /// which the entry must refuse with `none`.
    pub invalid: Option<usize>,
}

/// `value` of `ty` with one map's or set's members no longer strictly
/// ascending: its first member repeated. `None` when every map and set in
/// the value is empty, so no order can be broken.
fn invalidate(modules: &Modules<'_>, ty: &SemanticType, value: &Value) -> Option<Value> {
    match (ty, value) {
        (
            SemanticType::Map { key: _, value: _ } | SemanticType::Set { element: _ },
            Value::List { items },
        ) => {
            let first = items.first()?.clone();
            let mut items = items.clone();
            items.insert(0, first);
            Some(Value::List { items })
        }
        (SemanticType::Option { value: inner }, Value::Some { value }) => Some(Value::Some {
            value: Box::new(invalidate(modules, inner, value)?),
        }),
        (SemanticType::Result { ok, error: _ }, Value::Ok { value }) => Some(Value::Ok {
            value: Box::new(invalidate(modules, ok, value)?),
        }),
        (SemanticType::Result { ok: _, error }, Value::Error { value }) => Some(Value::Error {
            value: Box::new(invalidate(modules, error, value)?),
        }),
        (SemanticType::List { element }, Value::List { items }) => {
            items.iter().enumerate().find_map(|(position, item)| {
                let broken = invalidate(modules, element, item)?;
                let mut items = items.clone();
                items[position] = broken;
                Some(Value::List { items })
            })
        }
        (SemanticType::Product { left, right }, Value::Pair { left: a, right: b }) => {
            match invalidate(modules, left, a) {
                Some(broken) => Some(Value::Pair {
                    left: Box::new(broken),
                    right: b.clone(),
                }),
                None => Some(Value::Pair {
                    left: a.clone(),
                    right: Box::new(invalidate(modules, right, b)?),
                }),
            }
        }
        (
            SemanticType::Named {
                member: _,
                arguments: _,
            },
            Value::Adt {
                constructor,
                fields,
            },
        ) => {
            let (_, constructors) = source_type(modules, ty).expect("a document type");
            let types = &constructors[*constructor as usize].fields;
            types
                .iter()
                .zip(fields)
                .enumerate()
                .find_map(|(position, (field, item))| {
                    let broken = invalidate(modules, field, item)?;
                    let mut fields = fields.clone();
                    fields[position] = broken;
                    Some(Value::Adt {
                        constructor: *constructor,
                        fields,
                    })
                })
        }
        _ => None,
    }
}

/// The interpreter's outcome within a small fuel, on a thread whose stack
/// holds the recursion that fuel allows.
fn bounded(program: &lexlean::calculus::Program, function: u64, values: &[Value]) -> Outcome {
    std::thread::scope(|scope| {
        std::thread::Builder::new()
            .stack_size(1 << 29)
            .spawn_scoped(scope, || {
                interp::run(program, BOUNDED_FUEL, function, values)
            })
            .expect("a thread")
            .join()
            .expect("the interpreter runs")
    })
}

/// Tries of the search for an input that overflows.
const SEARCH: u64 = 400;

/// The fuel of an input at the bounds of its types.
const BOUNDED_FUEL: u64 = 100_000;

/// Whether a source type can hold a map or a set: the test's own oracle for
/// which parameters an entry must validate, independent of the lowering and
/// of the certificate generator.
fn holds_collection(
    modules: &Modules<'_>,
    ty: &SemanticType,
    seen: &mut std::collections::BTreeSet<String>,
) -> bool {
    match ty {
        SemanticType::Map { key: _, value: _ } | SemanticType::Set { element: _ } => true,
        SemanticType::Option { value: inner } | SemanticType::List { element: inner } => {
            holds_collection(modules, inner, seen)
        }
        SemanticType::Product { left, right }
        | SemanticType::Result {
            ok: left,
            error: right,
        } => holds_collection(modules, left, seen) || holds_collection(modules, right, seen),
        SemanticType::Named {
            member: _,
            arguments: _,
        } => {
            if !seen.insert(format!("{ty:?}")) {
                return false;
            }
            let (_, constructors) = source_type(modules, ty).expect("a document type");
            constructors
                .iter()
                .flat_map(|constructor| constructor.fields.iter())
                .any(|field| holds_collection(modules, field, seen))
        }
        _ => false,
    }
}

/// For each root of `project`, the positions of the parameters whose types
/// can hold a map or a set, which an entry must validate (§17.17), by the
/// test's own oracle.
#[must_use]
pub fn carriers(project: &P) -> BTreeMap<String, Vec<usize>> {
    let checked = support::checked_project(project);
    let modules = linked_modules(&checked);
    roots(&checked)
        .expect("the eligibility reports")
        .into_iter()
        .map(|root| {
            let (parameters, _) =
                root_signature(&modules, &root.module, &root.name).expect("a root signature");
            let positions = parameters
                .iter()
                .enumerate()
                .filter(|(_, parameter)| {
                    holds_collection(
                        &modules,
                        &parameter.r#type,
                        &mut std::collections::BTreeSet::new(),
                    )
                })
                .map(|(position, _)| position)
                .collect();
            (root.report.root.clone(), positions)
        })
        .collect()
}

/// The cases of every root of `project`, by root name.
///
/// Each root is run on seeded small inputs, on inputs at the top and at the
/// bottom of every scalar type and with each parameter in turn at the top,
/// where the overflow arm of the statements is reached (§17.17), and, when
/// it has an entry, through the entry on the same inputs and on inputs that
/// break the invariant of each validated parameter in turn.
#[must_use]
pub fn cases(project: &P) -> BTreeMap<String, Vec<Case>> {
    let checked = support::checked_project(project);
    let modules = linked_modules(&checked);
    let limits = support::limits(project);
    let mut out: BTreeMap<String, Vec<Case>> = BTreeMap::new();
    for root in roots(&checked).expect("the eligibility reports") {
        let lowered: Lowered = lower_root(&modules, &root.module, &root.name, root.report, &limits)
            .unwrap_or_else(|diagnostic| panic!("{}: {diagnostic:?}", root.report.root));
        let (parameters, _) =
            root_signature(&modules, &root.module, &root.name).expect("a root signature");
        let carriers: Vec<bool> = parameters
            .iter()
            .map(|parameter| {
                holds_collection(
                    &modules,
                    &parameter.r#type,
                    &mut std::collections::BTreeSet::new(),
                )
            })
            .collect();
        let entry = lowered.entry();
        let mut seed = root
            .report
            .root
            .bytes()
            .fold(0x9E37_79B9_7F4A_7C15_u64, |hash, byte| {
                (hash ^ u64::from(byte)).wrapping_mul(0x100_0000_01B3)
            });
        let mut next = move || {
            seed = seed.wrapping_add(0x9E37_79B9_7F4A_7C15) | 1;
            Rng(seed)
        };
        // (inputs, whether the inputs are the small seeded ones)
        let mut batches: Vec<(Vec<Value>, bool)> = Vec::new();
        for _ in 0..SAMPLES {
            let mut rng = next();
            batches.push((
                parameters
                    .iter()
                    .map(|parameter| {
                        sample(&modules, &mut rng, &parameter.r#type, 0, Extreme::Small)
                    })
                    .collect(),
                true,
            ));
        }
        for extreme in [
            Extreme::Medium,
            Extreme::Medium,
            Extreme::High,
            Extreme::Low,
        ] {
            let mut rng = next();
            batches.push((
                parameters
                    .iter()
                    .map(|parameter| sample(&modules, &mut rng, &parameter.r#type, 0, extreme))
                    .collect(),
                false,
            ));
        }
        for position in 0..parameters.len() {
            let mut rng = next();
            batches.push((
                parameters
                    .iter()
                    .enumerate()
                    .map(|(at, parameter)| {
                        let extreme = if at == position {
                            Extreme::High
                        } else {
                            Extreme::Small
                        };
                        sample(&modules, &mut rng, &parameter.r#type, 0, extreme)
                    })
                    .collect(),
                false,
            ));
        }
        // A root that may overflow is searched, on inputs whose scalars are
        // each small or at a bound, for one that does, so that the overflow
        // arm of every statement is exercised on it (§17.17).
        if root
            .report
            .declared_effects
            .iter()
            .any(|effect| effect == "overflow")
        {
            let overflows = |values: &Vec<Value>| {
                matches!(
                    bounded(&lowered.program, 0, values),
                    Outcome::Overflow { .. }
                )
            };
            if !batches.iter().any(|(values, _)| overflows(values)) {
                for _ in 0..SEARCH {
                    let mut rng = next();
                    let values: Vec<Value> = parameters
                        .iter()
                        .map(|parameter| {
                            sample(&modules, &mut rng, &parameter.r#type, 0, Extreme::Mixed)
                        })
                        .collect();
                    if overflows(&values) {
                        batches.push((values, false));
                        break;
                    }
                }
            }
        }
        let mut inputs: Vec<(u64, Option<usize>, Vec<Value>, bool)> = Vec::new();
        for (values, small) in &batches {
            // A root with an entry is invoked through the entry alone by a
            // caller, and through function 0 here to state the root's own
            // observation.
            inputs.push((0, None, values.clone(), *small));
            if entry != 0 {
                inputs.push((entry, None, values.clone(), *small));
            }
        }
        if entry != 0 {
            // One input per validated parameter that breaks its invariant,
            // each parameter's value resampled until it has something to
            // break.
            for (values, small) in batches.iter().filter(|(_, small)| *small) {
                for (position, carries) in carriers.iter().enumerate() {
                    if !carries {
                        continue;
                    }
                    let mut broken =
                        invalidate(&modules, &parameters[position].r#type, &values[position]);
                    let mut tries = 0;
                    while broken.is_none() && tries < 64 {
                        let mut rng = next();
                        let fresh = sample(
                            &modules,
                            &mut rng,
                            &parameters[position].r#type,
                            0,
                            Extreme::Small,
                        );
                        broken = invalidate(&modules, &parameters[position].r#type, &fresh);
                        tries += 1;
                    }
                    // A validated parameter whose invariant no input could be
                    // made to break would leave its validator unobserved by
                    // the differentials: that is a failure of the sampling,
                    // not a case to leave out.
                    let broken = broken.unwrap_or_else(|| {
                        panic!(
                            "{}: no sampled `{}` could be made to break its invariant in {tries} tries",
                            root.report.root, parameters[position].name
                        )
                    });
                    let mut values = values.clone();
                    values[position] = broken;
                    inputs.push((entry, Some(position), values, *small));
                }
            }
        }
        for (function, invalid, values, small) in inputs {
            // An input at the bounds that a recursion cannot finish within a
            // small fuel is no case; a small one must finish.
            let outcome = if small {
                interp::run(&lowered.program, FUEL, function, &values)
            } else {
                bounded(&lowered.program, function, &values)
            };
            if !small && matches!(outcome, Outcome::Exhausted) {
                continue;
            }
            out.entry(root.report.root.clone()).or_default().push(Case {
                root: root.report.root.clone(),
                arguments: parameters
                    .iter()
                    .zip(&values)
                    .map(|(parameter, value)| lean_term(&modules, &parameter.r#type, value))
                    .collect(),
                values,
                outcome,
                function,
                invalid,
            });
        }
    }
    out
}

/// Lean's `String.quote` on the printable text the samples contain.
fn quote(text: &str) -> String {
    let mut out = String::from("\"");
    for character in text.chars() {
        match character {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\t' => out.push_str("\\t"),
            other => out.push(other),
        }
    }
    out.push('"');
    out
}

/// A calculus value as the differential module's `__show` prints it.
fn show(value: &Value) -> String {
    let list = |items: &[Value]| items.iter().map(show).collect::<Vec<_>>().join(", ");
    match value {
        Value::Unit => "unit".to_owned(),
        Value::Bool { value } => format!("bool {value}"),
        Value::Nat { value } => format!("nat {value}"),
        Value::Int { value } => format!("int {value}"),
        Value::U8 { value } => format!("u8 {value}"),
        Value::U16 { value } => format!("u16 {value}"),
        Value::U32 { value } => format!("u32 {value}"),
        Value::U64 { value } => format!("u64 {value}"),
        Value::I8 { value } => format!("i8 {value}"),
        Value::I16 { value } => format!("i16 {value}"),
        Value::I32 { value } => format!("i32 {value}"),
        Value::I64 { value } => format!("i64 {value}"),
        Value::String { value } => format!("string {}", quote(value)),
        Value::Bytes { hex } => format!(
            "bytes [{}]",
            (0..hex.len() / 2)
                .map(
                    |index| u8::from_str_radix(&hex[2 * index..2 * index + 2], 16)
                        .expect("hex")
                        .to_string()
                )
                .collect::<Vec<_>>()
                .join(", ")
        ),
        Value::Ordering { value } => match value {
            OrderingValue::Lt => "ordering less".to_owned(),
            OrderingValue::Eq => "ordering same".to_owned(),
            OrderingValue::Gt => "ordering more".to_owned(),
        },
        Value::None => "none".to_owned(),
        Value::Some { value } => format!("some ({})", show(value)),
        Value::Ok { value } => format!("ok ({})", show(value)),
        Value::Error { value } => format!("error ({})", show(value)),
        Value::List { items } => format!("list [{}]", list(items)),
        Value::Pair { left, right } => format!("pair ({}) ({})", show(left), show(right)),
        Value::Adt {
            constructor,
            fields,
        } => format!("adt {constructor} [{}]", list(fields)),
        Value::Closure { function, captures } => {
            format!("closure {function} [{}]", list(captures))
        }
    }
}

/// The interpreter's outcome as the differential module prints the
/// certificate's observation of the same inputs.
#[must_use]
pub fn observed(outcome: &Outcome) -> String {
    match outcome {
        Outcome::Value { value, steps: _ } => format!("value {}", show(value)),
        Outcome::Overflow { steps: _ } => "overflow".to_owned(),
        Outcome::Stuck => "stuck".to_owned(),
        Outcome::Exhausted => "exhausted".to_owned(),
    }
}

/// The printer the differential module defines: the certificate's `Obs`
/// in the form [`observed`] writes the interpreter's.
const PRINTER: &str = r#"open LexLeanTarget.TargetSyntax in
partial def __show : Value → String
  | .unit => "unit"
  | .bool b => s!"bool {b}"
  | .nat n => s!"nat {n}"
  | .int i => s!"int {i}"
  | .u8 v => s!"u8 {v}"
  | .u16 v => s!"u16 {v}"
  | .u32 v => s!"u32 {v}"
  | .u64 v => s!"u64 {v}"
  | .i8 v => s!"i8 {v}"
  | .i16 v => s!"i16 {v}"
  | .i32 v => s!"i32 {v}"
  | .i64 v => s!"i64 {v}"
  | .string s => "string " ++ s.quote
  | .bytes b => "bytes [" ++ ", ".intercalate (b.toList.map toString) ++ "]"
  | .ordering .less => "ordering less"
  | .ordering .same => "ordering same"
  | .ordering .more => "ordering more"
  | .none => "none"
  | .some v => "some (" ++ __show v ++ ")"
  | .ok v => "ok (" ++ __show v ++ ")"
  | .error v => "error (" ++ __show v ++ ")"
  | .list vs => "list [" ++ ", ".intercalate (vs.map __show) ++ "]"
  | .pair a b => "pair (" ++ __show a ++ ") (" ++ __show b ++ ")"
  | .adt k fs => s!"adt {k} [" ++ ", ".intercalate (fs.map __show) ++ "]"
  | .closure k cs => s!"closure {k} [" ++ ", ".intercalate (cs.map __show) ++ "]"

def __obs : LexLeanPreservation.Obs → String
  | .value v => "value " ++ __show v
  | .overflow => "overflow"
  | .stuck => "stuck"
"#;

/// The differential module: it evaluates each certificate's observation
/// on each case's inputs and prints it beside the case's index.
#[must_use]
pub fn module(denotes: &[(String, String, Vec<Case>)]) -> String {
    let mut text = String::new();
    let imports: std::collections::BTreeSet<&String> =
        denotes.iter().map(|(module, _, _)| module).collect();
    for module in imports {
        text.push_str(&format!("import {module}\n"));
    }
    text.push_str(PRINTER);
    let mut index = 0;
    for (_, denote, cases) in denotes {
        for case in cases {
            text.push_str(&format!(
                "#eval IO.println (\"case {index}: \" ++ __obs ({denote} {}))\n",
                case.arguments.join(" ")
            ));
            index += 1;
        }
    }
    text
}

/// Compare the module's output with the interpreter, case by case.
///
/// # Errors
///
/// Returns every case whose printed observation differs.
pub fn compare(output: &str, denotes: &[(String, String, Vec<Case>)]) -> Result<usize, String> {
    let printed: BTreeMap<usize, &str> = output
        .lines()
        .filter_map(|line| {
            let rest = line.strip_prefix("case ")?;
            let (index, text) = rest.split_once(": ")?;
            Some((index.parse().ok()?, text))
        })
        .collect();
    let mut index = 0;
    let mut failures = Vec::new();
    for (_, _, cases) in denotes {
        for case in cases {
            let expected = observed(&case.outcome);
            match printed.get(&index) {
                Some(text) if *text == expected => {}
                Some(text) => failures.push(format!(
                    "{} {:?}: Lean observes `{text}`, the interpreter `{expected}`",
                    case.root, case.arguments
                )),
                None => failures.push(format!(
                    "{} {:?}: Lean printed nothing",
                    case.root, case.arguments
                )),
            }
            index += 1;
        }
    }
    if failures.is_empty() {
        Ok(index)
    } else {
        Err(failures.join("\n"))
    }
}
