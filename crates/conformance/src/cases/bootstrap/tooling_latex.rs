//! `BS-20`: the canonical LaTeX of a tooling module of SPEC.md §17.20 as
//! language-1.2 source (`LexLeanBoot.ToolingLatex`), held to
//! `backend::tooling::latex_text` to the byte.
//!
//! The root takes the tooling module as data (`LexLeanBoot.ToolingAst`), the
//! typings the checker infers (the type parameters of each call and function)
//! and the reserved tokens of the pinned toolchain as arguments, and a
//! capacity for the output buffer. The checker, the decoder of the module,
//! and the source map of the emitted pieces stay in the host.

use lexlean::backend::lean_tokens::LEAN_RESERVED_TOKENS;
use lexlean::backend::tooling::latex_text;
use lexlean::calculus::Value;
use lexlean::ir::tooling::{
    AuthorityPin, BinderInfo, CommandAtom, InvokeArgument, ParameterKind, Sig, SigLevel,
    ToolConstructor, ToolDeclaration, ToolTerm, ToolType, ToolingModule, TypeParameter,
};
use lexlean::ir::tooling_check::typings;
use serde_json::Value as Json;

use super::rust_print::REFUSAL;
use super::rust_validate::{adt, boolean, chain, nat};
use super::{list, option, text, Run};
use crate::support;

// ---------------------------------------------------------------- encoders

fn texts(items: &[String]) -> Value {
    list(items.iter().map(|item| text(item)).collect())
}

/// `BinderKind`: explicit, implicit, instance.
fn enc_binder(info: BinderInfo) -> Value {
    adt(
        match info {
            BinderInfo::Explicit => 0,
            BinderInfo::Implicit => 1,
            BinderInfo::Instance => 2,
        },
        Vec::new(),
    )
}

/// A chain of `Sig` ending in `SgNil` (9) with `SgCons` (10).
fn enc_sigs(items: &[Sig]) -> Value {
    chain(9, 10, items.iter().map(enc_sig).collect())
}

/// `Sig`: const, var, app, arrow, pi, type, optParam, empty, nat, nil, cons.
fn enc_sig(sig: &Sig) -> Value {
    match sig {
        Sig::Const { name } => adt(0, vec![text(name)]),
        Sig::Var { name } => adt(1, vec![text(name)]),
        Sig::App {
            function,
            arguments,
        } => adt(2, vec![enc_sig(function), enc_sigs(arguments)]),
        Sig::Arrow { domain, codomain } => adt(3, vec![enc_sig(domain), enc_sig(codomain)]),
        Sig::Pi {
            name,
            info,
            domain,
            codomain,
        } => adt(
            4,
            vec![
                option(name.as_deref().map(text)),
                enc_binder(*info),
                enc_sig(domain),
                enc_sig(codomain),
            ],
        ),
        Sig::Type { level } => adt(
            5,
            vec![option(level.as_ref().map(|SigLevel::Parameter { name }| text(name)))],
        ),
        Sig::OptParam { domain, default } => adt(6, vec![enc_sig(domain), enc_sig(default)]),
        Sig::EmptyStructure => adt(7, Vec::new()),
        Sig::Nat { value } => adt(8, vec![text(value)]),
    }
}

/// A chain of `Ty` ending in `TyNil` (4) with `TyCons` (5).
fn enc_tys(items: &[ToolType]) -> Value {
    chain(4, 5, items.iter().map(enc_ty).collect())
}

/// `Ty`: const, param, fn, index, nil, cons.
fn enc_ty(ty: &ToolType) -> Value {
    match ty {
        ToolType::Const { name, arguments } => adt(0, vec![text(name), enc_tys(arguments)]),
        ToolType::Parameter { name, arguments } => adt(1, vec![text(name), enc_tys(arguments)]),
        ToolType::Function { parameters, result } => {
            adt(2, vec![enc_tys(parameters), enc_ty(result)])
        }
        ToolType::Index { constructor } => adt(3, vec![text(constructor)]),
    }
}

/// `Pars`: nil (0), cons (1) of a name, a type, and the rest.
fn enc_pars<'a>(items: impl DoubleEndedIterator<Item = (&'a String, &'a ToolType)>) -> Value {
    chain_of(0, 1, items.map(|(name, ty)| vec![text(name), enc_ty(ty)]))
}

/// A chain whose constructor takes `fields` and then the rest.
fn chain_of(nil: u64, cons: u64, items: impl DoubleEndedIterator<Item = Vec<Value>>) -> Value {
    items.rev().fold(adt(nil, Vec::new()), |tail, mut fields| {
        fields.push(tail);
        adt(cons, fields)
    })
}

/// A chain of `Term` ending in `TmNil` (15) with `TmCons` (16).
fn enc_terms(items: &[ToolTerm]) -> Value {
    chain(15, 16, items.iter().map(enc_term).collect())
}

/// `Term`: var, nat, str, char, name, call, construct, field, list, array,
/// let, lambda, apply, if, match, nil, cons, branch nil, branch.
fn enc_term(term: &ToolTerm) -> Value {
    match term {
        ToolTerm::Var { name } => adt(0, vec![text(name)]),
        ToolTerm::Nat { value } => adt(1, vec![text(value)]),
        ToolTerm::String { value } => adt(2, vec![text(value)]),
        ToolTerm::Char { value } => adt(3, vec![nat(u64::from(*value))]),
        ToolTerm::Name { value } => adt(4, vec![text(value)]),
        ToolTerm::Call {
            function,
            type_arguments,
            arguments,
        } => adt(
            5,
            vec![text(function), enc_tys(type_arguments), enc_terms(arguments)],
        ),
        ToolTerm::Construct {
            constructor,
            type_arguments,
            arguments,
        } => adt(
            6,
            vec![
                text(constructor),
                enc_tys(type_arguments),
                enc_terms(arguments),
            ],
        ),
        ToolTerm::Field { value, field } => adt(7, vec![enc_term(value), text(field)]),
        ToolTerm::List { element, elements } => adt(8, vec![enc_ty(element), enc_terms(elements)]),
        ToolTerm::Array { element, elements } => adt(9, vec![enc_ty(element), enc_terms(elements)]),
        ToolTerm::Let {
            binder,
            ty,
            value,
            body,
        } => adt(
            10,
            vec![text(binder), enc_ty(ty), enc_term(value), enc_term(body)],
        ),
        ToolTerm::Lambda { parameters, body } => adt(
            11,
            vec![
                enc_pars(parameters.iter().map(|p| (&p.name, &p.ty))),
                enc_term(body),
            ],
        ),
        ToolTerm::Apply {
            function,
            arguments,
        } => adt(12, vec![enc_term(function), enc_terms(arguments)]),
        ToolTerm::If {
            condition,
            then_branch,
            else_branch,
        } => adt(
            13,
            vec![
                enc_term(condition),
                enc_term(then_branch),
                enc_term(else_branch),
            ],
        ),
        ToolTerm::Match {
            scrutinee,
            branches,
        } => adt(
            14,
            vec![
                enc_term(scrutinee),
                chain_of(
                    17,
                    18,
                    branches.iter().map(|branch| {
                        vec![
                            option(branch.constructor.as_deref().map(text)),
                            texts(&branch.binders),
                            enc_term(&branch.body),
                        ]
                    }),
                ),
            ],
        ),
    }
}

fn enc_pin(pin: &lexlean::ir::tooling::SourcePin) -> Value {
    adt(0, vec![text(&pin.file), text(&pin.sha256)])
}

/// `TypeParam`: a name and its `ParamKind` (type, constructor, index).
fn enc_type_param(parameter: &TypeParameter) -> Value {
    adt(
        0,
        vec![
            text(&parameter.name),
            match &parameter.kind {
                ParameterKind::Type => adt(0, Vec::new()),
                ParameterKind::Constructor => adt(1, Vec::new()),
                ParameterKind::Index { of } => adt(2, vec![text(of)]),
            },
        ],
    )
}

/// `Constructor`: a name, its fields, and the index requirement.
fn enc_constructor(constructor: &ToolConstructor) -> Value {
    adt(
        0,
        vec![
            text(&constructor.name),
            enc_pars(constructor.fields.iter().map(|f| (&f.name, &f.ty))),
            option(
                constructor
                    .requires
                    .as_ref()
                    .map(|r| adt(0, vec![text(&r.parameter), text(&r.constructor)])),
            ),
        ],
    )
}

/// `Decl`: type, call, plumbing, function, command, invoke, evaluate.
fn enc_decl(declaration: &ToolDeclaration) -> Value {
    match declaration {
        ToolDeclaration::Type {
            name,
            parameters,
            constructors,
            opaque,
            alias,
            source,
        } => adt(
            0,
            vec![
                text(name),
                list(parameters.iter().map(enc_type_param).collect()),
                list(constructors.iter().map(enc_constructor).collect()),
                boolean(*opaque),
                option(alias.as_ref().map(enc_ty)),
                enc_pin(source),
            ],
        ),
        ToolDeclaration::Call {
            name,
            signature,
            role,
            source,
        } => adt(
            1,
            vec![text(name), enc_sig(signature), text(role), enc_pin(source)],
        ),
        ToolDeclaration::Plumbing { names } => adt(2, vec![texts(names)]),
        ToolDeclaration::Function {
            name,
            public,
            partial,
            mutual,
            type_parameters,
            parameters,
            result,
            body,
            ..
        } => adt(
            3,
            vec![
                text(name),
                boolean(*public),
                boolean(*partial),
                option(mutual.as_deref().map(text)),
                texts(type_parameters),
                enc_pars(parameters.iter().map(|p| (&p.name, &p.ty))),
                enc_ty(result),
                enc_term(body),
            ],
        ),
        ToolDeclaration::Command {
            name,
            atoms,
            elaborator,
        } => adt(
            4,
            vec![
                text(name),
                list(
                    atoms
                        .iter()
                        .map(|atom| match atom {
                            CommandAtom::Token { text: token } => adt(0, vec![text(token)]),
                            CommandAtom::Ident => adt(1, Vec::new()),
                            CommandAtom::Type => adt(2, Vec::new()),
                        })
                        .collect(),
                ),
                text(elaborator),
            ],
        ),
        ToolDeclaration::Invoke { command, arguments } => adt(
            5,
            vec![
                text(command),
                list(
                    arguments
                        .iter()
                        .map(|argument| match argument {
                            InvokeArgument::Ident { value } => adt(0, vec![text(value)]),
                            InvokeArgument::Type { value } => adt(1, vec![enc_sig(value)]),
                        })
                        .collect(),
                ),
            ],
        ),
        ToolDeclaration::Evaluate {
            function,
            arguments,
        } => adt(6, vec![text(function), enc_terms(arguments)]),
    }
}

/// The reserved tokens of the pinned toolchain, as the argument of the root.
fn reserved() -> Value {
    list(LEAN_RESERVED_TOKENS.iter().map(|token| text(token)).collect())
}

/// The typings the checker infers: the type parameters of each call and
/// function, as `Typing` rows.
fn enc_typings(module: &ToolingModule) -> Option<Value> {
    let all = typings(module).ok()?;
    Some(list(
        all.iter()
            .map(|(name, typing)| adt(0, vec![text(name), texts(&typing.type_parameters)]))
            .collect(),
    ))
}

// ------------------------------------------------------------------ oracle

fn outcome(printed: &str, capacity: usize) -> Json {
    serde_json::to_value(if printed.len() > capacity {
        Value::Error {
            value: Box::new(text(REFUSAL)),
        }
    } else {
        Value::Ok {
            value: Box::new(text(printed)),
        }
    })
    .expect("a value is JSON")
}

/// The oracle's text of `module`, or `None` when the oracle refuses or
/// panics on it: the claim is over checked modules.
fn printed(module: &ToolingModule) -> Option<String> {
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let result = std::panic::catch_unwind(|| latex_text(module).ok().map(|rendered| rendered.text));
    std::panic::set_hook(hook);
    result.ok().flatten()
}

/// A run of `renderToolingLatex` with `capacity`.
pub(super) fn run_of(module: &ToolingModule, capacity: Option<usize>) -> Option<Run> {
    let text_out = printed(module)?;
    let capacity = capacity.unwrap_or(text_out.len());
    Some((
        vec![
            nat(capacity as u64),
            reserved(),
            enc_typings(module)?,
            list(module.declarations.iter().map(enc_decl).collect()),
        ],
        outcome(&text_out, capacity),
    ))
}

/// The runs of one module: the exact capacity, one byte short, and room to
/// spare.
fn runs_of(module: &ToolingModule) -> Vec<Run> {
    let Some(text_out) = printed(module) else {
        return Vec::new();
    };
    let length = text_out.len();
    [Some(length), Some(length.saturating_sub(1)), Some(length + 4096)]
        .into_iter()
        .filter_map(|capacity| run_of(module, capacity))
        .collect()
}

// -------------------------------------------------------------- committed

const FIXTURES: [(&str, &str); 3] = [
    ("commands", "ToolingCommands"),
    ("core", "ToolingCore"),
    ("traversal", "ToolingTraversal"),
];

/// The committed tooling modules as the compiler links them, with the
/// committed LaTeX of each.
fn committed() -> Vec<(String, ToolingModule, String)> {
    FIXTURES
        .iter()
        .map(|(fixture, module_name)| {
            let project = support::P::tooling(fixture);
            let inner = lexlean::project::Project::load(&project.root.join("lexlean.toml"))
                .expect("the project loads")
                .with_tooling();
            let (lock, packages) =
                lexlean::lock::compute_lock(&inner, false).expect("the lock computes");
            let checked = lexlean::link::check_project(
                &inner,
                &lexlean::project::Selection::Entrypoints,
                &lock,
                packages,
            )
            .unwrap_or_else(|error| panic!("{fixture}: {error:?}"));
            let module = checked
                .modules
                .values()
                .find_map(|module| module.document.tooling.clone())
                .unwrap_or_else(|| panic!("{fixture}: a tooling module"));
            let golden = std::fs::read_to_string(
                support::repo_root()
                    .join("tests/tooling")
                    .join(fixture)
                    .join("expected/build/modules")
                    .join(module_name)
                    .join("Main.tex")
                    .as_std_path(),
            )
            .unwrap_or_else(|error| panic!("{fixture}: the committed LaTeX: {error}"));
            ((*fixture).to_owned(), module, golden)
        })
        .collect()
}

// ----------------------------------------------------------------- mutants

/// Strings that reach every class of the escaper and of the quoting of a
/// string literal.
fn specials() -> Vec<String> {
    [
        "a\\b", "{x}", "50%", "a_b", "x^2", "~", "#", "$", "&", "\u{e9}", "\u{2192}",
        "\u{1d538}", "\u{ab}x\u{bb}", "a\nb", "tab\there", "\u{7f}", "\u{85}", "\u{a0}", "say \"hi\"",
        "ends with a space ", "\u{2003}", "\u{3000}", "\r", "\u{0}", "\u{9f}", "",
        "A.B.C", ".x", "x.", "..", "throwError", "do", "A.throwError.B", "fun", "match",
        "0", "1000000000000000000000",
    ]
    .iter()
    .map(|s| (*s).to_owned())
    .collect()
}

/// A scalar the checker accepts, from every width class of UTF-8.
fn scalars() -> [u32; 14] {
    [0, 9, 10, 13, 32, 39, 92, 126, 127, 233, 8364, 0x10ffff, 0x1d538, 34]
}

enum Seg {
    Key(String),
    Index(usize),
}

fn encode(path: &[Seg]) -> Vec<String> {
    path.iter()
        .map(|segment| match segment {
            Seg::Key(key) => format!("k{key}"),
            Seg::Index(index) => format!("i{index}"),
        })
        .collect()
}

fn leaves(json: &Json, path: &mut Vec<Seg>, out: &mut Vec<Vec<String>>, strings: &mut Vec<String>) {
    match json {
        Json::Object(map) => {
            for (key, value) in map {
                path.push(Seg::Key(key.clone()));
                leaves(value, path, out, strings);
                path.pop();
            }
        }
        Json::Array(items) => {
            for (index, value) in items.iter().enumerate() {
                path.push(Seg::Index(index));
                leaves(value, path, out, strings);
                path.pop();
            }
        }
        Json::String(value) => {
            if !matches!(path.last(), Some(Seg::Key(key)) if key == "kind") {
                out.push(encode(path));
                strings.push(value.clone());
            }
        }
        _ => {}
    }
}

fn at<'a>(json: &'a mut Json, path: &[String]) -> &'a mut Json {
    let mut node = json;
    for segment in path {
        node = match segment.split_at(1) {
            ("k", key) => &mut node[key],
            (_, index) => &mut node[index.parse::<usize>().expect("an index")],
        };
    }
    node
}

/// Every string of `json` that is not a tag, equal to `from`, becomes `to`.
fn rename(json: &mut Json, from: &str, to: &str) {
    match json {
        Json::Object(map) => {
            for (key, value) in map.iter_mut() {
                if key != "kind" {
                    rename(value, from, to);
                }
            }
        }
        Json::Array(items) => items.iter_mut().for_each(|item| rename(item, from, to)),
        Json::String(value) if value == from => *value = to.to_owned(),
        _ => {}
    }
}

/// A module that the checker of SPEC.md §17.20 accepts, from its JSON.
fn accepted(json: &Json) -> Option<ToolingModule> {
    let bytes = serde_json::to_vec(json).ok()?;
    let canonical = lexlean::artifact::canonical_json::Json::parse(&bytes)
        .ok()?
        .to_canonical_string();
    ToolingModule::parse(
        &canonical,
        lexlean::LANGUAGE_1_2,
        AuthorityPin {
            lean_version: lexlean::LEAN_VERSION,
            lean_githash: lexlean::LEAN_GITHASH,
        },
    )
    .ok()
}

/// A seeded mutant of `base` that the checker accepts and the oracle prints,
/// or `None` after a bounded number of attempts.
fn mutant(base: &ToolingModule, seeded: &mut super::Seeded) -> Option<ToolingModule> {
    let specials = specials();
    for _ in 0..40 {
        let mut json = serde_json::to_value(base).expect("a module is JSON");
        let (mut paths, mut strings) = (Vec::new(), Vec::new());
        leaves(&json, &mut Vec::new(), &mut paths, &mut strings);
        if paths.is_empty() {
            return None;
        }
        for _ in 0..=seeded.below(3) {
            let which = seeded.below(paths.len());
            let replacement = if seeded.chance(40) {
                (*seeded.pick(LEAN_RESERVED_TOKENS)).to_owned()
            } else {
                seeded.pick(&specials).clone()
            };
            match seeded.below(4) {
                // a leaf alone
                0 | 1 => *at(&mut json, &paths[which]) = Json::String(replacement),
                // a name everywhere it occurs
                2 => rename(&mut json, &strings[which], &replacement),
                // a flag, a scalar, or an optional label
                _ => flip(&mut json, seeded),
            }
        }
        if let Some(module) = accepted(&json) {
            if printed(&module).is_some() && module != *base {
                return Some(module);
            }
        }
    }
    None
}

/// The paths of the flags, characters, and optional labels of `json`.
fn flags(json: &Json, path: &mut Vec<Seg>, out: &mut Vec<Vec<String>>) {
    match json {
        Json::Object(map) => {
            for (key, value) in map {
                path.push(Seg::Key(key.clone()));
                if matches!(key.as_str(), "public" | "partial" | "opaque" | "mutual")
                    || (key == "value" && value.is_number())
                {
                    out.push(encode(path));
                }
                flags(value, path, out);
                path.pop();
            }
        }
        Json::Array(items) => {
            for (index, value) in items.iter().enumerate() {
                path.push(Seg::Index(index));
                flags(value, path, out);
                path.pop();
            }
        }
        _ => {}
    }
}

/// Flip a flag, change a character, or set a mutual label of one declaration
/// or term of `json`.
fn flip(json: &mut Json, seeded: &mut super::Seeded) {
    let mut sites = Vec::new();
    flags(json, &mut Vec::new(), &mut sites);
    if sites.is_empty() {
        return;
    }
    let site = at(json, &sites[seeded.below(sites.len())]);
    match site {
        Json::Bool(value) => *value = !*value,
        Json::Number(_) => *site = Json::from(*seeded.pick(&scalars())),
        _ => *site = Json::String(["a", "group", "G"][seeded.below(3)].to_owned()),
    }
}

// -------------------------------------------------------------- targeted

/// The names of the first declaration of each kind of `module`, which the
/// targeted mutants rename to the strings that stress the quoting of a name
/// and the trimming of a line.
fn titles(module: &ToolingModule) -> Vec<String> {
    let mut seen = std::collections::BTreeSet::new();
    let mut out = Vec::new();
    for declaration in &module.declarations {
        let title = match declaration {
            ToolDeclaration::Type { name, .. }
            | ToolDeclaration::Call { name, .. }
            | ToolDeclaration::Function { name, .. }
            | ToolDeclaration::Command { name, .. } => name.clone(),
            ToolDeclaration::Evaluate { function, .. } => function.clone(),
            ToolDeclaration::Invoke { .. } | ToolDeclaration::Plumbing { .. } => continue,
        };
        if seen.insert(declaration.kind()) {
            out.push(title);
        }
    }
    out
}

/// The modules made from `base` by renaming the first declaration of each
/// kind to each of `names`, and each token of a command to each of `tokens`.
fn targeted(base: &ToolingModule, names: &[&str], tokens: &[&str]) -> Vec<ToolingModule> {
    let json = serde_json::to_value(base).expect("a module is JSON");
    let mut out = Vec::new();
    for title in titles(base) {
        for name in names {
            let mut renamed = json.clone();
            rename(&mut renamed, &title, name);
            out.extend(accepted(&renamed).filter(|module| printed(module).is_some()));
        }
    }
    let (mut paths, mut strings) = (Vec::new(), Vec::new());
    leaves(&json, &mut Vec::new(), &mut paths, &mut strings);
    for (path, string) in paths.iter().zip(&strings) {
        let is_token = path.last().is_some_and(|last| last == "ktext");
        if is_token && !string.is_empty() {
            for token in tokens {
                let mut changed = json.clone();
                *at(&mut changed, path) = Json::String((*token).to_owned());
                out.extend(accepted(&changed).filter(|module| printed(module).is_some()));
            }
        }
    }
    // No committed module has a character term, so a `Char` type and a
    // function that returns one of the scalars that stress the quoting of a
    // character are added to each module that has a string type and a function.
    out.extend(characters(&json));
    out
}

/// The scalars of the character terms of the targeted vectors: the quote and
/// the backslash, the three escaped controls, a printable, the delete and
/// another control, and scalars of two, three, and four bytes.
const CHARS: [u32; 11] = [
    0x27, 0x5c, 0x0a, 0x0d, 0x09, 0x41, 0x7f, 0x01, 0xe9, 0x2192, 0x1_d538,
];

/// The modules made from the module `json` by adding the type `Char` and a
/// function whose body is a character term, one module for each of `CHARS`.
fn characters(json: &Json) -> Vec<ToolingModule> {
    let declarations = json["declarations"].as_array().expect("declarations");
    let string_type = declarations
        .iter()
        .position(|declaration| declaration["kind"] == "type" && declaration["name"] == "String");
    let function = declarations
        .iter()
        .find(|declaration| declaration["kind"] == "function");
    let (Some(string_type), Some(function)) = (string_type, function) else {
        return Vec::new();
    };
    let mut char_type = declarations[string_type].clone();
    char_type["name"] = Json::String("Char".to_owned());
    let name = function["name"].as_str().expect("a function name");
    let prefix = name.rsplit_once('.').map_or("", |(prefix, _)| prefix);
    let mut out = Vec::new();
    for code in CHARS {
        let mut glyph = function.clone();
        glyph["name"] = Json::String(if prefix.is_empty() {
            "glyph".to_owned()
        } else {
            format!("{prefix}.glyph")
        });
        glyph["type_parameters"] = serde_json::json!([]);
        glyph["parameters"] = serde_json::json!([]);
        glyph["result"] = serde_json::json!({"kind": "const", "name": "Char", "arguments": []});
        glyph["body"] = serde_json::json!({"kind": "char", "value": code});
        let mut changed = json.clone();
        let list = changed["declarations"].as_array_mut().expect("declarations");
        list.insert(string_type + 1, char_type.clone());
        list.push(glyph);
        out.extend(accepted(&changed).filter(|module| printed(module).is_some()));
    }
    out
}

// ------------------------------------------------------------------ plants

pub(super) const PLANTS: [super::Plant; 17] = [
    super::Plant {
        label: "a backslash is escaped as a bare command",
        site: r##"(if (code =? 92) then (putS (second state) "\\textbackslash{}")"##,
        replacement: r##"(if (code =? 92) then (putS (second state) "\\backslash{}")"##,
        root: ROOT,
    },
    super::Plant {
        label: "a brace is not escaped",
        site: r##"(if (code =? 123) then (putS (second state) "\\{")"##,
        replacement: r##"(if (code =? 123) then (putS (second state) "{")"##,
        root: ROOT,
    },
    super::Plant {
        label: "a tilde loses its empty group",
        site: r##"(if (code =? 126) then (putS (second state) "\\~{}")"##,
        replacement: r##"(if (code =? 126) then (putS (second state) "\\~")"##,
        root: ROOT,
    },
    super::Plant {
        label: "a caret loses its empty group",
        site: r##"(if (code =? 94) then (putS (second state) "\\^{}")"##,
        replacement: r##"(if (code =? 94) then (putS (second state) "\\^")"##,
        root: ROOT,
    },
    super::Plant {
        label: "an ampersand is not escaped",
        site: r##"(if (code =? 38) then (putS (second state) "\\&")"##,
        replacement: r##"(if (code =? 38) then (putS (second state) "&")"##,
        root: ROOT,
    },
    super::Plant {
        label: "the hexadecimal of a non-ASCII scalar is in lower case",
        site: r##"(putS (putHex (putS (second state) "<U+") 0x30313233343536373839414243444546 code) ">")"##,
        replacement: r##"(putS (putHex (putS (second state) "<U+") 0x30313233343536373839616263646566 code) ">")"##,
        root: ROOT,
    },
    super::Plant {
        label: "the hexadecimal of a scalar has three digits at least",
        site: "(leadingZero digits position)) 4 0",
        replacement: "(leadingZero digits position)) 5 0",
        root: ROOT,
    },
    super::Plant {
        label: "a character is quoted as a bare quote",
        site: r##"(if (code =? 39) then (putS out "'\\''")"##,
        replacement: r##"(if (code =? 39) then (putS out "'''")"##,
        root: ROOT,
    },
    super::Plant {
        label: "an application of a registered call is not explicit",
        site: r##"then (putS out "(@") else (putS out "(")"##,
        replacement: r##"then (putS out "(") else (putS out "(")"##,
        root: ROOT,
    },
    super::Plant {
        label: "an implicit binder is bracketed as an explicit one",
        site: r##"(putS o1 "{")"##,
        replacement: r##"(putS o1 "(")"##,
        root: ROOT,
    },
    super::Plant {
        label: "an instance binder is bracketed as an explicit one",
        site: r##"BkInstance => (putS (putSig (putS (putName (putS o1 "[")"##,
        replacement: r##"BkInstance => (putS (putSig (putS (putName (putS o1 "(")"##,
        root: ROOT,
    },
    super::Plant {
        label: "a named type argument has no space before its value",
        site: r##"" := ") reserved head 0 true)"##,
        replacement: r##"" :=") reserved head 0 true)"##,
        root: ROOT,
    },
    super::Plant {
        label: "the body of a function is indented by one space",
        site: r##"" :=\n  ""##,
        replacement: r##"" :=\n ""##,
        root: ROOT,
    },
    super::Plant {
        label: "a function has no newline after its body",
        site: r##"(putS (putTerm o4 ctx bodyTerm 0 true) "\n")"##,
        replacement: r##"(putS (putTerm o4 ctx bodyTerm 0 true) "")"##,
        root: ROOT,
    },
    super::Plant {
        label: "a public function is not marked public",
        site: r##"(if isPublic then (putS out "public ") else out)"##,
        replacement: r##"(if isPublic then out else out)"##,
        root: ROOT,
    },
    super::Plant {
        label: "the visibility line swaps public and private",
        site: r##"(if isPublic then "public" else "private")"##,
        replacement: r##"(if isPublic then "private" else "public")"##,
        root: ROOT,
    },
    super::Plant {
        label: "a trimmed line loses one byte of its trailing white space",
        site: "(prim subtract end size : Nat)",
        replacement: "(prim subtract end 1 : Nat)",
        root: ROOT,
    },
];

// -------------------------------------------------------------------- case

const MODULE: &str = "LexLeanBoot.ToolingLatex";
const ROOT: &str = "renderToolingLatex";
const EFFECTS: [&str; 3] = ["allocation", "overflow", "recursion"];

const EVALUATOR_MUTANTS: usize = 60;
const RUSTC_MUTANTS: usize = 200;

fn eligibility(project: &support::P) {
    let built = project.build_ok();
    let build_id = built.build_id.expect("built");
    let report: Json = serde_json::from_slice(
        &std::fs::read(
            project
                .build_dir(&build_id)
                .join("production/LexLeanBoot/ToolingLatex.eligibility.json")
                .as_std_path(),
        )
        .expect("the eligibility report"),
    )
    .expect("the report is JSON");
    let reported = report["roots"].as_array().expect("roots");
    assert_eq!(reported.len(), 1, "one production root");
    let row = &reported[0];
    assert_eq!(
        row["root"],
        format!("{MODULE}.{ROOT}"),
        "the root of the module"
    );
    let declared: Vec<&str> = row["declared_effects"]
        .as_array()
        .expect("declared effects")
        .iter()
        .map(|effect| effect.as_str().expect("an effect"))
        .collect();
    assert_eq!(declared, EFFECTS, "the declared effects");
    let targets = row["targets"].as_array().expect("targets");
    assert_eq!(targets.len(), 1, "one target");
    assert_eq!(targets[0]["target"], "rust-std", "the target");
    assert_eq!(targets[0]["status"], "eligible", "eligible");
    let realized: Vec<&str> = targets[0]["effects"]
        .as_array()
        .expect("effects")
        .iter()
        .map(|effect| effect["effect"].as_str().expect("an effect"))
        .collect();
    assert_eq!(
        realized, EFFECTS,
        "the declaration admits exactly the realized effects"
    );
}

/// Every form of the closed tooling schema that `module` uses, by its tag.
fn forms(module: &ToolingModule, into: &mut std::collections::BTreeSet<String>) {
    fn walk(json: &Json, into: &mut std::collections::BTreeSet<String>) {
        match json {
            Json::Object(map) => {
                for (key, value) in map {
                    if let (true, Json::String(tag)) = (key == "kind", value) {
                        into.insert(tag.clone());
                    }
                    if let (true, Json::String(info)) = (key == "info", value) {
                        into.insert(format!("binder:{info}"));
                    }
                    walk(value, into);
                }
            }
            Json::Array(items) => items.iter().for_each(|item| walk(item, into)),
            _ => {}
        }
    }
    walk(&serde_json::to_value(module).expect("a module is JSON"), into);
}

/// Seeded mutants of `bases`, each at the exact capacity and one byte short.
fn mutants(bases: &[&ToolingModule], seeded: &mut super::Seeded, count: usize) -> Vec<Run> {
    let mut runs = Vec::new();
    let mut made = 0;
    while made < count {
        let base = bases[seeded.below(bases.len())];
        if let Some(module) = mutant(base, seeded) {
            runs.extend(runs_of(&module));
            made += 1;
        }
    }
    runs
}

pub(super) fn case() {
    let started = std::time::Instant::now();
    let project = support::P::bootstrap();
    let checked_units = project.check_ok();
    assert!(
        checked_units.units.contains_key("ToolingLatex")
            && checked_units.units.contains_key("ToolingAst")
            && checked_units.units.contains_key("LexLeanStd.Buffer"),
        "ToolingLatex, ToolingAst, and LexLeanStd.Buffer are linked modules of the project"
    );
    eligibility(&project);

    let committed = committed();
    for (fixture, module, golden) in &committed {
        assert_eq!(
            printed(module).as_deref(),
            Some(golden.as_str()),
            "{fixture}: the oracle's text is the committed LaTeX"
        );
    }
    let bases: Vec<&ToolingModule> = committed.iter().map(|(_, module, _)| module).collect();

    // The vectors: the committed modules and the modules made from them by
    // renaming a declaration or a token to the strings that stress the quoting
    // of a name, the literal of a string, the escaper, and the trimming of a
    // line, each at the exact capacity, one byte short, and with room to spare.
    let names = [
        "throwError",
        "A.do.B",
        "a\nb",
        "a b ",
        "50%_{x}^~#$&\\",
        "\u{e9}\u{2192}\u{1d538}",
        "",
        ".x",
        "x.",
        "\u{ab}\u{bb}",
    ];
    let tokens = [
        "tok ",
        "\t",
        "a\"b\\c",
        "\u{85}",
        "\u{2028}",
        "\u{9f}x",
        "\u{7f}",
        "x\u{a0}",
        "\n",
    ];
    let mut modules: Vec<ToolingModule> = bases.iter().map(|module| (*module).clone()).collect();
    let committed_count = modules.len();
    for base in &bases {
        modules.extend(targeted(base, &names, &tokens));
    }
    let hand = modules.len() - committed_count;
    let vectors: Vec<Run> = modules.iter().flat_map(runs_of).collect();
    let refused = vectors
        .iter()
        .filter(|(_, outcome)| outcome["kind"] == "error")
        .count();
    let mut covered = std::collections::BTreeSet::new();
    for module in &modules {
        forms(module, &mut covered);
    }
    eprintln!(
        "BS-20: {} vectors ({committed_count} committed modules, {hand} targeted, {refused} refused), forms {covered:?} ({:?})",
        vectors.len(),
        started.elapsed()
    );
    // The corpora of the normative host are made here, so that every count the
    // row states is asserted before the first leg runs.
    let mut seeded = super::Seeded(0x0B55_0020);
    let evaluator_mutants = mutants(&bases, &mut seeded, EVALUATOR_MUTANTS);
    let mut large = super::Seeded(0x0B55_0021);
    let rustc_mutants = mutants(&bases, &mut large, RUSTC_MUTANTS);
    eprintln!(
        "BS-20: {} evaluator mutant runs, {} rustc mutant runs ({:?})",
        evaluator_mutants.len(),
        rustc_mutants.len(),
        started.elapsed()
    );

    let checked = support::checked_project(&project);
    let root = super::lower(&checked, &project, MODULE, ROOT);
    super::rust_validate::rustc_differential("BS-20", &[(ROOT, &root, &vectors)], "vectors");
    if !support::normative_host() {
        eprintln!("BS-20: platform-bound host: the evaluator, the seeded corpora, and the planted defects run on the normative host");
        return;
    }
    let disagreements = super::evaluate(&root, &vectors, 5);
    assert!(
        disagreements.is_empty(),
        "the evaluator and the oracle disagree on the vectors: {disagreements:?}"
    );
    let disagreements = super::evaluate(&root, &evaluator_mutants, 5);
    assert!(
        disagreements.is_empty(),
        "the evaluator and the oracle disagree on the seeded mutants: {disagreements:?}"
    );
    super::rust_validate::rustc_differential("BS-20", &[(ROOT, &root, &rustc_mutants)], "seeded");
    eprintln!("BS-20: the legs agree ({:?})", started.elapsed());

    // The gate is armed: each defect planted in the source of the module is
    // caught by the differential on the vectors.
    for plant in &PLANTS {
        let planted = support::P::bootstrap();
        let source = planted.read_raw("src/ToolingLatex.lex.tex");
        assert_eq!(
            source.matches(plant.site).count(),
            1,
            "{}: the plant site occurs once",
            plant.label
        );
        planted.write(
            "src/ToolingLatex.lex.tex",
            &source.replacen(plant.site, plant.replacement, 1),
        );
        let checked = support::checked_project(&planted);
        let low = super::lower(&checked, &planted, MODULE, plant.root);
        let disagreements = super::evaluate(&low, &vectors, 1);
        assert!(
            !disagreements.is_empty(),
            "{}: the planted defect is detected by the differential",
            plant.label
        );
        eprintln!(
            "BS-20: plant detected: {}: {}",
            plant.label, disagreements[0]
        );
    }

    // Under the pinned Lean, the production export certifies the root; its
    // package is the committed one and realizes the program the differential
    // ran.
    if !support::lean_backed("BS-20") {
        return;
    }
    let exported = harness::exported("bootstrap");
    let bundle = exported.bundle();
    let packages = harness::tree(&bundle.join("packages"));
    let committed_packages = harness::tree(&harness::fixture_dir("bootstrap").join("packages"));
    assert_eq!(
        packages.keys().collect::<Vec<_>>(),
        committed_packages.keys().collect::<Vec<_>>(),
        "the exported package files are the committed ones"
    );
    for (relative, content) in &packages {
        assert!(
            committed_packages.get(relative) == Some(content),
            "packages/{relative} differs from the committed package"
        );
    }
    let mut realized = 0usize;
    for exported_root in &exported.output.result.roots {
        if exported_root.root != format!("{MODULE}.{ROOT}") {
            continue;
        }
        for output in &exported_root.outputs {
            let package: Json = serde_json::from_slice(
                &std::fs::read(
                    bundle
                        .join(&output.package.directory)
                        .join("package.json")
                        .as_std_path(),
                )
                .expect("package.json"),
            )
            .expect("package JSON");
            assert_eq!(
                package["program"],
                serde_json::to_value(&root.program).expect("a program is JSON"),
                "{ROOT}: the exported package realizes the program the differential ran"
            );
            realized += 1;
        }
    }
    assert_eq!(realized, 1, "one certified package for the root");
    eprintln!("BS-20: passed in {:?}", started.elapsed());
}

use crate::production_export as harness;
