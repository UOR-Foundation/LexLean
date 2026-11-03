//! The driver of a command (SPEC.md §17.13, §17.16).
//!
//! A command root is a pure step: `step (state : Option Session) (event :
//! LexLeanStd.Host::Event) : Session * LexLeanStd.Host::Request`. Its package
//! carries a `src/main.rs` that calls the step in a loop and carries out each
//! request it returns. The driver is the text of the language-1.2 module
//! `RuntimeHost`, decoded, never evaluated, and printed after the checks here.
//!
//! Program types are nameless (`Adt7`), so this module finds the types of the
//! module `LexLeanStd.Host` by walking the entry's signature against the shape
//! the module states, and refuses any mismatch. A project that wrote its own
//! module of that name is held to the same shape, and its requests to the same
//! effects, so it gains nothing.

use std::collections::BTreeMap;

use super::super::{IntKind, Program, Ty};
use super::package::{Errors, Export, Passing};
use super::{rt, rt_decode, rt_validate};

/// What a type of `LexLeanStd.Host` holds, field by field.
#[derive(Clone)]
enum Want {
    Unit,
    Bool,
    Nat,
    U8,
    I32,
    Str,
    Bytes,
    Option(Box<Want>),
    Result(Box<Want>, Box<Want>),
    List(Box<Want>),
    Pair(Box<Want>, Box<Want>),
    Host(&'static str),
}

fn option(inner: Want) -> Want {
    Want::Option(Box::new(inner))
}

fn result(ok: Want, error: Want) -> Want {
    Want::Result(Box::new(ok), Box::new(error))
}

fn list(element: Want) -> Want {
    Want::List(Box::new(element))
}

fn pair(left: Want, right: Want) -> Want {
    Want::Pair(Box::new(left), Box::new(right))
}

fn failed(ok: Want) -> Want {
    result(ok, Want::Host("Failure"))
}

/// The constructors of the type `name` of `LexLeanStd.Host`, each as the
/// types of its fields, in the order `stdlib/src/Host.lex.tex` declares them.
fn shape(name: &str) -> Option<Vec<Vec<Want>>> {
    use Want::{Bool, Bytes, Host, Nat, Str, Unit, I32, U8};
    Some(match name {
        "FailureKind" | "EntryKind" => vec![vec![]; 4],
        "Failure" => vec![vec![Host("FailureKind"), Bytes]],
        "Entry" => vec![vec![Host("EntryKind"), Nat, option(pair(Nat, Nat))]],
        "Child" => vec![vec![Str, list(Str), Str, list(pair(Str, Str)), Str, Str]],
        "Waited" => vec![vec![list(pair(Nat, option(I32))), Nat]],
        "Start" => vec![vec![
            list(Bytes),
            failed(Bytes),
            failed(Bytes),
            list(pair(Bytes, Bytes)),
            Bool,
        ]],
        "Request" => vec![
            vec![Str],
            vec![Str, Bytes],
            vec![Str],
            vec![Str],
            vec![Str],
            vec![Str],
            vec![Str],
            vec![Str, Str],
            vec![Str],
            vec![Str],
            vec![Host("Child")],
            vec![list(Nat), Nat],
            vec![Nat],
            vec![U8, Bytes, Bytes],
        ],
        "Event" => vec![
            vec![Host("Start")],
            vec![failed(Bytes)],
            vec![failed(Unit)],
            vec![failed(list(Bytes))],
            vec![failed(Host("Entry"))],
            vec![failed(Bytes)],
            vec![failed(pair(Nat, Nat))],
            vec![failed(Host("Waited"))],
        ],
        _ => return None,
    })
}

/// The name the driver module gives each type of `LexLeanStd.Host` it uses.
const ALIASES: [(&str, &str); 9] = [
    ("Request", "Request"),
    ("Event", "Event"),
    ("Failure", "Failure"),
    ("Kind", "FailureKind"),
    ("Entry", "Entry"),
    ("Shape", "EntryKind"),
    ("Spec", "Child"),
    ("Waited", "Waited"),
    ("Start", "Start"),
];

/// The names the driver imports from the package's own runtime.
const RUNTIME: [&str; 5] = ["Bytes", "Str", "List", "R", "Overflow"];

/// A command found in a package: where each type of the host module is.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Command {
    /// The Rust name of the export that is the step.
    pub entry: String,
    /// Whether the step can overflow, so that it returns `R<(S, Request)>`.
    pub fallible: bool,
    /// The program type that is the session.
    session: u64,
    /// The program type of each type of `LexLeanStd.Host` the step reaches.
    hosts: BTreeMap<&'static str, u64>,
}

struct Walk<'a> {
    program: &'a Program,
    found: BTreeMap<&'static str, u64>,
    claimed: BTreeMap<u64, &'static str>,
}

fn mismatch<T>(what: impl Into<String>) -> Result<T, String> {
    Err(format!("unsupported type: {}", what.into()))
}

impl Walk<'_> {
    fn host(&mut self, name: &'static str, index: u64) -> Result<(), String> {
        if let Some(held) = self.found.get(name) {
            return if *held == index {
                Ok(())
            } else {
                mismatch(format!(
                    "the command's `Host::{name}` is two program types, {held} and {index}"
                ))
            };
        }
        if let Some(other) = self.claimed.get(&index) {
            return mismatch(format!(
                "the command's program type {index} is both `Host::{other}` and `Host::{name}`"
            ));
        }
        self.found.insert(name, index);
        self.claimed.insert(index, name);
        let Some(wanted) = shape(name) else {
            return mismatch(format!("`Host::{name}` is not a type of the host module"));
        };
        let adt = usize::try_from(index)
            .ok()
            .and_then(|at| self.program.adts.get(at))
            .ok_or_else(|| format!("unsupported type: program type {index} is not declared"))?;
        if adt.constructors.len() != wanted.len() {
            return mismatch(format!(
                "the command's `Host::{name}` has {} constructors, not {}",
                adt.constructors.len(),
                wanted.len()
            ));
        }
        for (constructor, (have, want)) in adt.constructors.iter().zip(&wanted).enumerate() {
            if have.len() != want.len() {
                return mismatch(format!(
                    "constructor {constructor} of the command's `Host::{name}` has {} fields, not {}",
                    have.len(),
                    want.len()
                ));
            }
            for (ty, want) in have.iter().zip(want) {
                self.ty(ty, want, name, constructor)?;
            }
        }
        Ok(())
    }

    fn ty(&mut self, ty: &Ty, want: &Want, owner: &str, constructor: usize) -> Result<(), String> {
        let same = match (want, ty) {
            (Want::Unit, Ty::Unit)
            | (Want::Bool, Ty::Bool)
            | (Want::Nat, Ty::Nat)
            | (Want::Str, Ty::String)
            | (Want::Bytes, Ty::Bytes) => true,
            (Want::U8, Ty::Fixed { width }) => *width == IntKind::U8,
            (Want::I32, Ty::Fixed { width }) => *width == IntKind::I32,
            (Want::Option(inner), Ty::Option { value }) => {
                return self.ty(value, inner, owner, constructor)
            }
            (Want::List(inner), Ty::List { element }) => {
                return self.ty(element, inner, owner, constructor)
            }
            (Want::Result(a, b), Ty::Result { ok: good, error }) => {
                self.ty(good, a, owner, constructor)?;
                return self.ty(error, b, owner, constructor);
            }
            (Want::Pair(a, b), Ty::Pair { left, right }) => {
                self.ty(left, a, owner, constructor)?;
                return self.ty(right, b, owner, constructor);
            }
            (Want::Host(name), Ty::Adt { index }) => return self.host(*name, *index),
            _ => false,
        };
        if same {
            Ok(())
        } else {
            mismatch(format!(
                "a field of constructor {constructor} of the command's `Host::{owner}` has another type than the host module states"
            ))
        }
    }
}

/// The command a package's `entry` export is: its signature is the step of a
/// command root, and every type of the host module it reaches has the shape
/// the module states.
///
/// # Errors
///
/// The entry names no export, takes its parameters other than by value, is not
/// of the type `Option S -> Host::Event -> S * Host::Request` for an `S` that
/// is a program type of its own, or reaches a host type of another shape.
pub fn recognize(program: &Program, exports: &[Export], entry: &str) -> Result<Command, String> {
    let Some(export) = exports.iter().find(|export| export.name == entry) else {
        return Err(format!("the entry `{entry}` is not an export"));
    };
    if export.parameters != [Passing::Own, Passing::Own] {
        return mismatch(format!(
            "the entry `{entry}` takes its state and event by value, as a command's step does"
        ));
    }
    let function = usize::try_from(export.function)
        .ok()
        .and_then(|at| program.functions.get(at))
        .ok_or_else(|| format!("the entry `{entry}` names a function that is not declared"))?;
    let shape_error = || {
        mismatch(format!(
            "the entry `{entry}` is not of the type `Option S -> Host::Event -> S * Host::Request`"
        ))
    };
    let [Ty::Option { value: state }, Ty::Adt { index: event }] = function.types.as_slice() else {
        return shape_error();
    };
    let Ty::Pair { left, right } = &function.result else {
        return shape_error();
    };
    let (Ty::Adt { index: session }, Ty::Adt { index: request }) = (state.as_ref(), right.as_ref())
    else {
        return shape_error();
    };
    if left != state {
        return shape_error();
    }
    let mut walk = Walk {
        program,
        found: BTreeMap::new(),
        claimed: BTreeMap::new(),
    };
    walk.host("Event", *event)?;
    walk.host("Request", *request)?;
    if let Some(name) = walk.claimed.get(session) {
        return mismatch(format!(
            "the session of the entry `{entry}` is the program type of `Host::{name}`"
        ));
    }
    if let Some((_, name)) = ALIASES
        .iter()
        .find(|(_, host)| !walk.found.contains_key(*host))
    {
        return mismatch(format!(
            "the entry `{entry}` reaches no `Host::{name}`, which the driver needs"
        ));
    }
    Ok(Command {
        entry: export.name.clone(),
        fallible: export.errors == Errors::Overflow,
        session: *session,
        hosts: walk.found,
    })
}

/// The `use` of the package's own crate that a name of the driver module
/// stands for: a type of the host module or the session (an alias of its
/// program type), a name of the runtime, or the step.
fn import(command: &Command, krate: &str, name: &str) -> Result<rt::Item, String> {
    let own = |last: String, alias: Option<&str>| {
        rt::Item::Use(vec![krate.to_owned(), last], alias.map(str::to_owned))
    };
    if name == "entry" {
        return Ok(own(command.entry.clone(), Some("entry")));
    }
    if name == "Session" {
        return Ok(own(format!("Adt{}", command.session), Some("Session")));
    }
    if RUNTIME.contains(&name) {
        return Ok(own(name.to_owned(), None));
    }
    let Some((alias, host)) = ALIASES.iter().find(|(alias, _)| *alias == name) else {
        return Err(format!(
            "the driver imports `{name}`, which is not a name of the host"
        ));
    };
    let index = command
        .hosts
        .get(*host)
        .ok_or_else(|| format!("the command has no `Host::{host}`"))?;
    Ok(own(format!("Adt{index}"), Some(*alias)))
}

/// The crates of the driver module a package uses, by the effects its root
/// admits: the base, the call of the step, and each effect's group, present
/// when admitted and refusing when not.
fn parts(command: &Command, effects: &[String]) -> [&'static str; 4] {
    let admits = |effect: &str| effects.iter().any(|held| held == effect);
    [
        "hostBase",
        if command.fallible {
            "stepFallible"
        } else {
            "stepPlain"
        },
        if admits("files") {
            "filesOn"
        } else {
            "filesOff"
        },
        if admits("process") {
            "processOn"
        } else {
            "processOff"
        },
    ]
}

/// The driver crate of `command` in the package `krate` whose root admits
/// `effects`: the parts of the module, their imports from the host stand-in
/// rewritten to the package's own names and each import kept once.
///
/// # Errors
///
/// The module cannot be decoded, an import names nothing the package has, or
/// the crate is not valid under the validator of `RuntimeSyntax`, which
/// refuses a name that realizes an effect the root does not admit.
pub fn driver(command: &Command, krate: &str, effects: &[String]) -> Result<rt::Crate, String> {
    let module = rt_decode::embedded_host()?;
    let mut items: Vec<rt::Item> = Vec::new();
    for part in parts(command, effects) {
        for item in rt_decode::decode_host(module, part)?.items {
            let item = match item {
                rt::Item::Use(path, None) if path.first().is_some_and(|root| root == "host") => {
                    let [_, name] = path.as_slice() else {
                        return Err(format!(
                            "`use {}` names one item of the host",
                            path.join("::")
                        ));
                    };
                    import(command, krate, name)?
                }
                other => other,
            };
            if matches!(item, rt::Item::Use(..)) && items.contains(&item) {
                continue;
            }
            items.push(item);
        }
    }
    let admitted: Vec<&str> = effects.iter().map(String::as_str).collect();
    let driver = rt::Crate {
        profile: rt::Profile::Std,
        items,
    };
    rt_validate::validate_in(
        &driver,
        rt_validate::Host {
            roots: &[krate],
            effects: &admitted,
        },
    )?;
    Ok(driver)
}

/// The text of `src/main.rs` for `command`: the attributes the scaffold
/// derives, then the driver.
///
/// # Errors
///
/// As [`driver`], and the scaffold or the printer cannot be read.
pub fn main_rs(command: &Command, krate: &str, effects: &[String]) -> Result<String, String> {
    let crate_ = driver(command, krate, effects)?;
    let mut text = super::scaffold::get()?.attributes(super::Profile::Std)?;
    text.push_str(&rt::print(&crate_)?);
    Ok(text)
}
