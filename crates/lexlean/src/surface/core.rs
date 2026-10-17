//! The authoring notation of a core module (SPEC.md §17.10, §17.21.8).
//!
//! A core module is a library of kernel terms, and its notation is a readable
//! program over them: an `imports` line, the declarations, and the node table
//! of the shared expression DAG, one statement per node that is not written
//! inside the node that uses it.
//!
//! ```text
//! imports [Init, Lib.Algebra]
//!
//! theorem Lib.zero_add.{u} : @41 := @77 { class: false, … }
//!
//! @12 := Nat
//! @13 := @12 -> @12
//! @14 := fun (n : @12) => Nat.succ n
//! ```
//!
//! A statement `@N := term` defines node `N`, which is the node numbered next
//! in the order nodes finish being read: a node written inside a statement is
//! numbered before the statement's own, so the numbering of a module is
//! exactly the order of its text, and the labels only state it. A node used
//! exactly once, by a node that follows it closely enough for the numbering to
//! come out the same, is written inside its user instead of on a line of its
//! own; every other node is a statement, and is named by its label wherever
//! it is used. Which nodes those are is a function of the module, so every
//! module has one notation, and the notation reads back to the module.

use std::fmt::Write as _;

use crate::artifact::canonical_json::Json;
use crate::ir::core::{
    CoreBinderInfo, CoreDeclaration, CoreLevel, CoreMetaEntry, CoreMetaValue, CoreModule, CoreNode,
    CORE_SPEC_2,
};

use super::doc::{render, Doc};
use super::generic;
use super::lex::quote;
use super::SurfaceError;

/// The width the printer lays a statement out at.
const WIDTH: usize = generic::WIDTH;

/// The most nodes one node is written around: a node whose subtree is already
/// this tall is a statement of its own. It bounds the nesting of what the
/// printer writes, which the parser reads in bounded stack.
const INLINE_HEIGHT: usize = 48;

/// The deepest nesting the parser reads; above [`INLINE_HEIGHT`] so that
/// everything the printer writes is read.
const MAX_DEPTH: usize = generic::MAX_DEPTH;

/// The largest `+ n` of a universe level the parser reads: it allocates a
/// successor per unit.
const MAX_SUCC: usize = 1024;

/// The words that are not names wherever a name is written. A name equal to
/// one is quoted.
const KEYWORDS: [&str; 17] = [
    "fun", "let", "have", "proof", "Sort", "proj", "annot", "const", "max", "imax", "true",
    "false", "bool", "nat", "int", "str", "name",
];

fn error(offset: usize, message: impl Into<String>) -> SurfaceError {
    SurfaceError {
        offset,
        message: message.into(),
    }
}

// --- names -----------------------------------------------------------------

/// Whether a scalar continues a name: a letter or digit, `_ ' ! ? . @ #`, or
/// any scalar outside ASCII that is not white space.
fn name_char(c: char) -> bool {
    c.is_ascii_alphanumeric()
        || matches!(c, '_' | '\'' | '!' | '?' | '.' | '@' | '#')
        || (!c.is_ascii() && !c.is_whitespace())
}

/// Whether a scalar can begin a name.
fn name_start(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_' || (!c.is_ascii() && !c.is_whitespace())
}

/// Whether `text` holds a numeral with a redundant leading zero that the
/// scanner of §12.4 would not take for part of an identifier.
fn has_redundant_zero(text: &str) -> bool {
    let bytes = text.as_bytes();
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index].is_ascii_digit() {
            let start = index;
            while index < bytes.len() && bytes[index].is_ascii_digit() {
                index += 1;
            }
            let continues = start > 0
                && (bytes[start - 1].is_ascii_alphabetic()
                    || matches!(bytes[start - 1], b'_' | b'\''));
            if index - start > 1 && bytes[start] == b'0' && !continues {
                return true;
            }
        } else {
            index += 1;
        }
    }
    false
}

/// Whether `text` is written without quotes.
fn bare(text: &str) -> bool {
    let mut chars = text.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    name_start(first)
        && text.chars().all(name_char)
        && !text.ends_with('.')
        && text != "_"
        && !KEYWORDS.contains(&text)
        && !has_redundant_zero(text)
}

/// A name as the notation spells it.
fn spell(text: &str) -> String {
    if bare(text) {
        text.to_owned()
    } else {
        quote(text)
    }
}

/// A binder's name: the empty name is `_`.
fn binder_name(text: &str) -> String {
    if text.is_empty() {
        "_".to_owned()
    } else {
        spell(text)
    }
}

/// Whether `text` is the canonical spelling of a natural number.
fn canonical_natural(text: &str) -> bool {
    !text.is_empty()
        && text.bytes().all(|byte| byte.is_ascii_digit())
        && (text == "0" || !text.starts_with('0'))
}

/// Whether `text` is the canonical spelling of an integer.
fn canonical_integer(text: &str) -> bool {
    let digits = text.strip_prefix('-').unwrap_or(text);
    canonical_natural(digits) && text != "-0"
}

// --- printing --------------------------------------------------------------

fn write_level(out: &mut String, level: &CoreLevel) {
    match level {
        CoreLevel::Zero => out.push('0'),
        CoreLevel::Param(name) => out.push_str(&spell(name)),
        CoreLevel::Max(pair) | CoreLevel::IMax(pair) => {
            out.push_str(if matches!(level, CoreLevel::Max(_)) {
                "max("
            } else {
                "imax("
            });
            write_level(out, &pair.0);
            out.push_str(", ");
            write_level(out, &pair.1);
            out.push(')');
        }
        CoreLevel::Succ(_) => {
            let mut count = 0usize;
            let mut base = level;
            while let CoreLevel::Succ(inner) = base {
                count += 1;
                base = inner;
            }
            if matches!(base, CoreLevel::Zero) {
                let _ = write!(out, "{count}");
            } else {
                write_level(out, base);
                let _ = write!(out, "+{count}");
            }
        }
    }
}

fn level_text(level: &CoreLevel) -> String {
    let mut out = String::new();
    write_level(&mut out, level);
    out
}

fn levels_text(levels: &[CoreLevel]) -> String {
    if levels.is_empty() {
        return String::new();
    }
    let items: Vec<String> = levels.iter().map(level_text).collect();
    format!(".{{{}}}", items.join(", "))
}

/// A term the printer has built, and where it may stand without parentheses.
enum Term {
    /// A name, a reference, a literal, `proof(…)`: anywhere.
    Atom(Doc),
    /// An application: a head and its arguments.
    App(Doc, Vec<Doc>),
    /// `Sort`, `proj`, and `annot`: the whole of a term, or the domain of an
    /// arrow.
    Prefix(Doc),
    /// A binder form or a `let`: the last thing of a term.
    Low(Doc),
}

fn paren(doc: Doc) -> Doc {
    Doc::concat(vec![Doc::text("("), doc, Doc::text(")")])
}

fn app_doc(head: Doc, args: Vec<Doc>) -> Doc {
    if args.is_empty() {
        return head;
    }
    let mut rest = Vec::with_capacity(args.len() * 2);
    for arg in args {
        rest.push(Doc::Line);
        rest.push(arg);
    }
    Doc::concat(vec![head, Doc::concat(rest).nest(2)]).group()
}

impl Term {
    /// Anywhere a term is read whole.
    fn whole(self) -> Doc {
        match self {
            Self::Atom(doc) | Self::Prefix(doc) | Self::Low(doc) => doc,
            Self::App(head, args) => app_doc(head, args),
        }
    }

    /// As an argument of an application.
    fn argument(self) -> Doc {
        match self {
            Self::Atom(doc) => doc,
            other => paren(other.whole()),
        }
    }

    /// As the domain of an arrow.
    fn domain(self) -> Doc {
        match self {
            Self::Low(doc) => paren(doc),
            other => other.whole(),
        }
    }

    /// As the function of an application: the head and the arguments so far.
    fn head(self) -> (Doc, Vec<Doc>) {
        match self {
            Self::Atom(doc) => (doc, Vec::new()),
            Self::App(head, args) => (head, args),
            other => (paren(other.whole()), Vec::new()),
        }
    }
}

fn binder_doc(info: CoreBinderInfo, name: &str, domain: Term) -> Doc {
    let (open, close) = match info {
        CoreBinderInfo::Explicit => ("(", ")"),
        CoreBinderInfo::Implicit => ("{", "}"),
        CoreBinderInfo::StrictImplicit => ("{{", "}}"),
        CoreBinderInfo::Instance => ("[", "]"),
    };
    Doc::concat(vec![
        Doc::text(format!("{open}{} : ", binder_name(name))),
        domain.whole(),
        Doc::text(close),
    ])
}

fn meta_entry_text(entry: &CoreMetaEntry) -> Result<String, SurfaceError> {
    let value = match &entry.v {
        CoreMetaValue::Bool { v } => format!("bool {v}"),
        CoreMetaValue::Nat { v } if canonical_natural(v) => format!("nat {v}"),
        CoreMetaValue::Nat { v } => format!("nat {}", quote(v)),
        CoreMetaValue::Int { v } if canonical_integer(v) => format!("int {v}"),
        CoreMetaValue::Int { v } => format!("int {}", quote(v)),
        CoreMetaValue::Str { v } => format!("str {}", quote(v)),
        CoreMetaValue::Name { v } => format!("name {}", spell(v)),
    };
    Ok(format!("{} := {value}", spell(&entry.n)))
}

/// The shape of the printing of one module: which nodes are written inside
/// the node that uses them.
struct Plan {
    inlined: Vec<bool>,
}

/// Check that the module is one the notation can number.
fn check(module: &CoreModule) -> Result<(), SurfaceError> {
    let count = module.nodes.len();
    for (index, node) in module.nodes.iter().enumerate() {
        for child in children(node) {
            if child >= index {
                return Err(error(
                    0,
                    format!("node {index} points to the node {child}, which is not earlier"),
                ));
            }
        }
    }
    let mut previous = None;
    for proof in &module.proof_nodes {
        if *proof >= count || previous.is_some_and(|prior| prior >= *proof) {
            return Err(error(
                0,
                "the proof nodes are not strictly increasing nodes of the module",
            ));
        }
        previous = Some(*proof);
    }
    for declaration in &module.declarations {
        for root in roots(declaration) {
            if root >= count {
                return Err(error(
                    0,
                    format!(
                        "`{}` has a root that is not a node of the module",
                        declaration.name
                    ),
                ));
            }
        }
    }
    Ok(())
}

fn children(node: &CoreNode) -> Vec<usize> {
    match node {
        CoreNode::App { f, x } => vec![*f, *x],
        CoreNode::Lambda { t, v, .. } | CoreNode::Forall { t, v, .. } => vec![*t, *v],
        CoreNode::Let { t, v, body, .. } => vec![*t, *v, *body],
        CoreNode::Projection { x, .. } | CoreNode::Mdata { x, .. } => vec![*x],
        CoreNode::BVar { .. }
        | CoreNode::Sort { .. }
        | CoreNode::Const { .. }
        | CoreNode::Nat { .. }
        | CoreNode::String { .. } => Vec::new(),
    }
}

/// The nodes a declaration states by number.
fn roots(declaration: &CoreDeclaration) -> Vec<usize> {
    let mut roots = vec![declaration.r#type];
    roots.extend(declaration.value);
    if let Some(equations) = &declaration.eqn_info {
        roots.push(equations.r#type);
        roots.push(equations.value);
    }
    if let Some(structure) = declaration
        .inductive
        .as_ref()
        .and_then(|inductive| inductive.structure.as_ref())
    {
        roots.extend(structure.fields.iter().filter_map(|field| field.auto_param));
    }
    roots
}

fn plan(module: &CoreModule) -> Plan {
    let count = module.nodes.len();
    // A declaration's root is a use too, so a node a declaration states and
    // no node uses has one use and no user to be written inside.
    let mut uses = vec![0u32; count];
    for node in &module.nodes {
        for child in children(node) {
            uses[child] = uses[child].saturating_add(1);
        }
    }
    for declaration in &module.declarations {
        for root in roots(declaration) {
            uses[root] = uses[root].saturating_add(1);
        }
    }
    let mut inlined = vec![false; count];
    let mut start = vec![0usize; count];
    let mut height = vec![0usize; count];
    for (index, node) in module.nodes.iter().enumerate() {
        // The block of nodes written inside this one ends right before it, and
        // each child written inside it must end right before the one after.
        let mut expected = index.checked_sub(1);
        let mut tallest = 0;
        for child in children(node).into_iter().rev() {
            if uses[child] == 1 && Some(child) == expected && height[child] < INLINE_HEIGHT {
                inlined[child] = true;
                expected = start[child].checked_sub(1);
                tallest = tallest.max(height[child]);
            }
        }
        start[index] = expected.map_or(0, |end| end + 1);
        height[index] = tallest + 1;
    }
    Plan { inlined }
}

struct Printer<'a> {
    module: &'a CoreModule,
    inlined: Vec<bool>,
    proof: Vec<bool>,
}

/// How a bound variable is written: by the name of its binder when the binder
/// is one the notation can name and no nearer binder has the name (which the
/// reader would take the name for), else by its index. `scope` holds the names
/// of the binders the term is written under, outermost first.
fn variable(scope: &[String], index: usize) -> String {
    if let Some(at) = scope.len().checked_sub(index + 1) {
        let name = &scope[at];
        if bare(name) && !scope[at + 1..].iter().any(|nearer| nearer == name) {
            return name.clone();
        }
    }
    format!("#{index}")
}

impl Printer<'_> {
    /// A child of a node: written inside it, or named by its label.
    fn child(&self, index: usize, scope: &mut Vec<String>) -> Result<Term, SurfaceError> {
        if self.inlined[index] {
            self.term(index, scope)
        } else {
            Ok(Term::Atom(Doc::text(format!("@{index}"))))
        }
    }

    /// The term of a node, with the nodes written inside it. The recursion is
    /// as deep as the nodes written inside one another, which the printer
    /// bounds (`INLINE_HEIGHT`).
    fn term(&self, index: usize, scope: &mut Vec<String>) -> Result<Term, SurfaceError> {
        let node = &self.module.nodes[index];
        let term = match node {
            CoreNode::BVar { i } => Term::Atom(Doc::text(variable(scope, *i))),
            CoreNode::Sort { l } => Term::Prefix(Doc::text(format!("Sort {}", level_text(l)))),
            CoreNode::Const { n, u } => {
                let levels = levels_text(u);
                // A name a binder in scope has is the variable's: the constant
                // of that name is written as a string.
                Term::Atom(Doc::text(
                    if bare(n) && !scope.iter().any(|binder| binder == n) {
                        format!("{n}{levels}")
                    } else {
                        format!("const {}{levels}", quote(n))
                    },
                ))
            }
            CoreNode::App { f, x } => {
                let function = self.child(*f, scope)?;
                let argument = self.child(*x, scope)?;
                let (head, mut args) = function.head();
                args.push(argument.argument());
                Term::App(head, args)
            }
            CoreNode::Lambda { n, b, t, v } => {
                let domain = self.child(*t, scope)?;
                scope.push(n.clone());
                let body = self.child(*v, scope)?;
                scope.pop();
                Term::Low(
                    Doc::concat(vec![
                        Doc::text("fun "),
                        binder_doc(*b, n, domain),
                        Doc::text(" =>"),
                        Doc::Line,
                        body.whole(),
                    ])
                    .group(),
                )
            }
            CoreNode::Forall { n, b, t, v } => {
                let domain = self.child(*t, scope)?;
                scope.push(n.clone());
                let body = self.child(*v, scope)?;
                scope.pop();
                let head = if n.is_empty() && matches!(b, CoreBinderInfo::Explicit) {
                    domain.domain()
                } else {
                    binder_doc(*b, n, domain)
                };
                Term::Low(
                    Doc::concat(vec![head, Doc::text(" ->"), Doc::Line, body.whole()]).group(),
                )
            }
            CoreNode::Let { n, t, v, body, d } => {
                let ty = self.child(*t, scope)?;
                let value = self.child(*v, scope)?;
                scope.push(n.clone());
                let rest = self.child(*body, scope)?;
                scope.pop();
                Term::Low(
                    Doc::concat(vec![
                        Doc::text(format!(
                            "{} {} : ",
                            if *d { "have" } else { "let" },
                            binder_name(n)
                        )),
                        ty.whole(),
                        Doc::text(" := "),
                        value.whole(),
                        Doc::text(";"),
                        Doc::Line,
                        rest.whole(),
                    ])
                    .group(),
                )
            }
            CoreNode::Nat { v } => Term::Atom(Doc::text(if canonical_natural(v) {
                v.clone()
            } else {
                format!("nat {}", quote(v))
            })),
            CoreNode::String { v } => {
                let mut text = String::with_capacity(v.len());
                for scalar in v {
                    text.push(char::from_u32(*scalar).ok_or_else(|| {
                        error(
                            0,
                            format!("node {index}: {scalar} is not a Unicode scalar value"),
                        )
                    })?);
                }
                Term::Atom(Doc::text(quote(&text)))
            }
            CoreNode::Mdata { m, x } => {
                let entries = m
                    .iter()
                    .map(meta_entry_text)
                    .collect::<Result<Vec<_>, _>>()?;
                let inner = self.child(*x, scope)?;
                Term::Prefix(
                    Doc::concat(vec![
                        Doc::text(format!("annot [{}]", entries.join(", "))),
                        Doc::Line,
                        inner.argument(),
                    ])
                    .group(),
                )
            }
            CoreNode::Projection { n, s, i, x } => {
                let inner = self.child(*x, scope)?;
                Term::Prefix(
                    Doc::concat(vec![
                        Doc::text(format!("proj {} {} {i}", spell(s), spell(n))),
                        Doc::Line,
                        inner.argument(),
                    ])
                    .group(),
                )
            }
        };
        Ok(if self.proof[index] {
            Term::Atom(Doc::concat(vec![
                Doc::text("proof("),
                term.whole().nest(2),
                Doc::text(")"),
            ]))
        } else {
            term
        })
    }
}

fn declaration_text(declaration: &CoreDeclaration) -> Result<String, SurfaceError> {
    let mut members = serde_json::to_value(declaration)
        .map_err(|failure| error(0, failure.to_string()))
        .and_then(|value| {
            Json::parse(value.to_string().as_bytes()).map_err(|failure| error(0, failure))
        })?;
    let kind = match &mut members {
        Json::Obj(object) => {
            let kind = match object.remove("kind") {
                Some(Json::Str(kind)) => kind,
                _ => return Err(error(0, "a declaration has a kind")),
            };
            for key in ["name", "levels", "type", "value"] {
                object.remove(key);
            }
            kind
        }
        _ => return Err(error(0, "a declaration is an object")),
    };
    let mut head = format!("{kind} {}", spell(&declaration.name));
    if !declaration.levels.is_empty() {
        let names: Vec<String> = declaration.levels.iter().map(|name| spell(name)).collect();
        let _ = write!(head, ".{{{}}}", names.join(", "));
    }
    let _ = write!(head, " : @{}", declaration.r#type);
    if let Some(value) = declaration.value {
        let _ = write!(head, " := @{value}");
    }
    let mut doc = vec![Doc::text(head)];
    if matches!(&members, Json::Obj(object) if !object.is_empty()) {
        doc.push(Doc::text(" "));
        doc.push(generic::doc(&members));
    }
    Ok(render(&Doc::concat(doc), WIDTH))
}

/// The notation of a core module: its imports, its declarations, and the node
/// table, ended by a line feed.
///
/// # Errors
///
/// A module the notation does not carry: another schema than
/// `lexlean/core-module/2`, an edge that is not backward, a proof node or a
/// root outside the module, a string literal that is not a scalar value.
pub fn print_module(module: &CoreModule) -> Result<String, SurfaceError> {
    if module.spec != CORE_SPEC_2 {
        return Err(error(
            0,
            format!(
                "the notation names the schema `{CORE_SPEC_2}`, not `{}`",
                module.spec
            ),
        ));
    }
    check(module)?;
    let mut proof = vec![false; module.nodes.len()];
    for node in &module.proof_nodes {
        proof[*node] = true;
    }
    let printer = Printer {
        module,
        inlined: plan(module).inlined,
        proof,
    };
    let imports: Vec<Doc> = module
        .imports
        .iter()
        .map(|name| Doc::text(spell(name)))
        .collect();
    let mut out = render(
        &Doc::concat(vec![
            Doc::text("imports "),
            generic::delimited("[", "]", imports),
        ]),
        WIDTH,
    );
    out.push_str("\n\n");
    for declaration in &module.declarations {
        out.push_str(&declaration_text(declaration)?);
        out.push('\n');
    }
    out.push('\n');
    for index in (0..module.nodes.len()).filter(|index| !printer.inlined[*index]) {
        // A statement is written under no binder: a variable free in it is
        // written by its index.
        let term = printer.term(index, &mut Vec::new())?;
        let statement = Doc::concat(vec![
            Doc::text(format!("@{index} := ")),
            term.whole().nest(2),
        ]);
        out.push_str(&render(&statement, WIDTH));
        out.push('\n');
    }
    Ok(out)
}

/// The notation of the module whose canonical JSON text is `data`.
///
/// # Errors
///
/// Text that is not a core module, or one the notation does not carry.
pub fn print_data(data: &str) -> Result<String, SurfaceError> {
    let module: CoreModule = serde_json::from_str(data)
        .map_err(|failure| error(0, format!("not a core module: {failure}")))?;
    print_module(&module)
}

// --- the canonical data form ------------------------------------------------

fn write_string(out: &mut String, text: &str) {
    out.push_str(&quote(text));
}

fn write_level_json(out: &mut String, level: &CoreLevel) {
    match level {
        CoreLevel::Zero => out.push_str("{\"k\":\"z\"}"),
        CoreLevel::Succ(inner) => {
            out.push_str("{\"a\":");
            write_level_json(out, inner);
            out.push_str(",\"k\":\"s\"}");
        }
        CoreLevel::Max(pair) | CoreLevel::IMax(pair) => {
            out.push_str("{\"a\":[");
            write_level_json(out, &pair.0);
            out.push(',');
            write_level_json(out, &pair.1);
            out.push_str("],\"k\":\"");
            out.push(if matches!(level, CoreLevel::Max(_)) {
                'm'
            } else {
                'i'
            });
            out.push_str("\"}");
        }
        CoreLevel::Param(name) => {
            out.push_str("{\"a\":");
            write_string(out, name);
            out.push_str(",\"k\":\"p\"}");
        }
    }
}

fn binder_letter(info: CoreBinderInfo) -> char {
    match info {
        CoreBinderInfo::Explicit => 'e',
        CoreBinderInfo::Implicit => 'i',
        CoreBinderInfo::StrictImplicit => 's',
        CoreBinderInfo::Instance => 'c',
    }
}

fn write_node_json(out: &mut String, node: &CoreNode) {
    match node {
        CoreNode::BVar { i } => {
            let _ = write!(out, "{{\"i\":{i},\"k\":\"b\"}}");
        }
        CoreNode::Sort { l } => {
            out.push_str("{\"k\":\"s\",\"l\":");
            write_level_json(out, l);
            out.push('}');
        }
        CoreNode::Const { n, u } => {
            out.push_str("{\"k\":\"c\",\"n\":");
            write_string(out, n);
            out.push_str(",\"u\":[");
            for (index, level) in u.iter().enumerate() {
                if index > 0 {
                    out.push(',');
                }
                write_level_json(out, level);
            }
            out.push_str("]}");
        }
        CoreNode::App { f, x } => {
            let _ = write!(out, "{{\"f\":{f},\"k\":\"a\",\"x\":{x}}}");
        }
        CoreNode::Lambda { n, b, t, v } | CoreNode::Forall { n, b, t, v } => {
            let _ = write!(out, "{{\"b\":\"{}\",\"k\":\"", binder_letter(*b));
            out.push(if matches!(node, CoreNode::Lambda { .. }) {
                'l'
            } else {
                'p'
            });
            out.push_str("\",\"n\":");
            write_string(out, n);
            let _ = write!(out, ",\"t\":{t},\"v\":{v}}}");
        }
        CoreNode::Let { n, t, v, body, d } => {
            let _ = write!(out, "{{\"body\":{body},\"d\":{d},\"k\":\"e\",\"n\":");
            write_string(out, n);
            let _ = write!(out, ",\"t\":{t},\"v\":{v}}}");
        }
        CoreNode::Nat { v } => {
            out.push_str("{\"k\":\"n\",\"v\":");
            write_string(out, v);
            out.push('}');
        }
        CoreNode::String { v } => {
            out.push_str("{\"k\":\"t\",\"v\":[");
            for (index, scalar) in v.iter().enumerate() {
                if index > 0 {
                    out.push(',');
                }
                let _ = write!(out, "{scalar}");
            }
            out.push_str("]}");
        }
        CoreNode::Mdata { m, x } => {
            out.push_str("{\"k\":\"d\",\"m\":[");
            for (index, entry) in m.iter().enumerate() {
                if index > 0 {
                    out.push(',');
                }
                out.push_str("{\"n\":");
                write_string(out, &entry.n);
                out.push_str(",\"v\":");
                match &entry.v {
                    CoreMetaValue::Bool { v } => {
                        let _ = write!(out, "{{\"t\":\"b\",\"v\":{v}}}");
                    }
                    CoreMetaValue::Nat { v } => {
                        out.push_str("{\"t\":\"n\",\"v\":");
                        write_string(out, v);
                        out.push('}');
                    }
                    CoreMetaValue::Int { v } => {
                        out.push_str("{\"t\":\"i\",\"v\":");
                        write_string(out, v);
                        out.push('}');
                    }
                    CoreMetaValue::Str { v } => {
                        out.push_str("{\"t\":\"s\",\"v\":");
                        write_string(out, v);
                        out.push('}');
                    }
                    CoreMetaValue::Name { v } => {
                        out.push_str("{\"t\":\"m\",\"v\":");
                        write_string(out, v);
                        out.push('}');
                    }
                }
                out.push('}');
            }
            let _ = write!(out, "],\"x\":{x}}}");
        }
        CoreNode::Projection { n, s, i, x } => {
            let _ = write!(out, "{{\"i\":{i},\"k\":\"j\",\"n\":");
            write_string(out, n);
            out.push_str(",\"s\":");
            write_string(out, s);
            let _ = write!(out, ",\"x\":{x}}}");
        }
    }
}

/// The canonical JSON (§21.7) of a module: the data form every other stage
/// reads. Object keys are in byte order, so the order of the fields below is
/// the alphabet.
///
/// # Errors
///
/// A declaration that does not serialize.
pub fn canonical_json(module: &CoreModule) -> Result<String, SurfaceError> {
    let mut out = String::with_capacity(module.nodes.len() * 40 + 1024);
    out.push_str("{\"declarations\":[");
    for (index, declaration) in module.declarations.iter().enumerate() {
        if index > 0 {
            out.push(',');
        }
        // The value keeps its object keys in byte order, which a derived
        // serializer does not.
        out.push_str(
            &serde_json::to_value(declaration)
                .map_err(|failure| error(0, failure.to_string()))?
                .to_string(),
        );
    }
    out.push_str("],\"imports\":[");
    for (index, name) in module.imports.iter().enumerate() {
        if index > 0 {
            out.push(',');
        }
        write_string(&mut out, name);
    }
    out.push_str("],\"nodes\":[");
    for (index, node) in module.nodes.iter().enumerate() {
        if index > 0 {
            out.push(',');
        }
        write_node_json(&mut out, node);
    }
    out.push_str("],\"proof_nodes\":[");
    for (index, node) in module.proof_nodes.iter().enumerate() {
        if index > 0 {
            out.push(',');
        }
        let _ = write!(out, "{node}");
    }
    out.push_str("],\"spec\":");
    write_string(&mut out, &module.spec);
    out.push('}');
    Ok(out)
}

// --- parsing ----------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Kind {
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBrack,
    RBrack,
    Comma,
    Colon,
    ColonEq,
    Semi,
    Arrow,
    FatArrow,
    Plus,
    Minus,
    /// `.{`, after a name: the universe levels of a constant.
    LevelOpen,
    /// `@N`.
    Ref,
    /// `@name`: a statement a person names, which `fmt` numbers.
    Label,
    /// `#N`.
    BVar,
    /// A numeral.
    Num,
    /// A quoted string.
    Str,
    /// A name, bare.
    Name,
    End,
}

#[derive(Debug, Clone, Copy)]
struct Token {
    kind: Kind,
    start: usize,
    end: usize,
    /// Whether the token is the first character of its line.
    col0: bool,
}

struct Parser<'a> {
    text: &'a str,
    bytes: &'a [u8],
    at: usize,
    cached: Option<(usize, Token, usize)>,
    /// One past the last byte of the last token read.
    last_end: usize,
    nodes: Vec<CoreNode>,
    proof: Vec<usize>,
    /// The names of the binders the term being read is under, outermost
    /// first; the empty name is a binder that has none.
    scope: Vec<String>,
    /// The statements a person named, by the node each defines.
    labels: std::collections::HashMap<String, usize>,
}

type PResult<T> = Result<T, SurfaceError>;

impl<'a> Parser<'a> {
    fn new(text: &'a str) -> Self {
        Self {
            text,
            bytes: text.as_bytes(),
            at: 0,
            cached: None,
            last_end: 0,
            nodes: Vec::new(),
            proof: Vec::new(),
            scope: Vec::new(),
            labels: std::collections::HashMap::new(),
        }
    }

    fn fail<T>(&self, offset: usize, message: impl Into<String>) -> PResult<T> {
        Err(error(offset, message))
    }

    /// The token at `from`, and where the one after it begins.
    fn scan(&self, from: usize) -> PResult<(Token, usize)> {
        let bytes = self.bytes;
        let mut at = from;
        while at < bytes.len() {
            match bytes[at] {
                b' ' | b'\n' => at += 1,
                b'\t' | b'\r' => {
                    return self.fail(at, "a tab or a carriage return is not part of the notation")
                }
                _ => break,
            }
        }
        let col0 = at == 0 || bytes[at - 1] == b'\n';
        let make = |kind, end| {
            (
                Token {
                    kind,
                    start: at,
                    end,
                    col0,
                },
                end,
            )
        };
        let Some(&byte) = bytes.get(at) else {
            return Ok(make(Kind::End, at));
        };
        let next = bytes.get(at + 1).copied();
        Ok(match byte {
            b'(' => make(Kind::LParen, at + 1),
            b')' => make(Kind::RParen, at + 1),
            b'{' => make(Kind::LBrace, at + 1),
            b'}' => make(Kind::RBrace, at + 1),
            b'[' => make(Kind::LBrack, at + 1),
            b']' => make(Kind::RBrack, at + 1),
            b',' => make(Kind::Comma, at + 1),
            b';' => make(Kind::Semi, at + 1),
            b'+' => make(Kind::Plus, at + 1),
            b':' if next == Some(b'=') => make(Kind::ColonEq, at + 2),
            b':' => make(Kind::Colon, at + 1),
            b'-' if next == Some(b'>') => make(Kind::Arrow, at + 2),
            b'-' => make(Kind::Minus, at + 1),
            b'=' if next == Some(b'>') => make(Kind::FatArrow, at + 2),
            b'.' if next == Some(b'{') => make(Kind::LevelOpen, at + 2),
            b'"' => {
                let rest = &self.text[at..];
                let (_, length) = super::lex::string(rest, at)?;
                make(Kind::Str, at + length)
            }
            b'@' if self.text[at + 1..].chars().next().is_some_and(name_start) => {
                let mut end = at + 1;
                for scalar in self.text[at + 1..].chars() {
                    if !name_char(scalar) {
                        break;
                    }
                    end += scalar.len_utf8();
                }
                make(Kind::Label, end)
            }
            b'@' | b'#' => {
                let mut end = at + 1;
                while end < bytes.len() && bytes[end].is_ascii_digit() {
                    end += 1;
                }
                if end == at + 1 {
                    return self.fail(
                        at,
                        "`@` and `#` are followed by a number or, after `@`, a name",
                    );
                }
                if self.text[end..].chars().next().is_some_and(name_char) {
                    return self.fail(end, "a number ends where its text does");
                }
                make(if byte == b'@' { Kind::Ref } else { Kind::BVar }, end)
            }
            b'0'..=b'9' => {
                let mut end = at;
                while end < bytes.len() && bytes[end].is_ascii_digit() {
                    end += 1;
                }
                if self.text[end..].chars().next().is_some_and(name_char) {
                    return self.fail(at, "a name does not begin with a digit; quote it");
                }
                make(Kind::Num, end)
            }
            _ => {
                let first = self.text[at..].chars().next().unwrap_or(' ');
                if !name_start(first) {
                    return self.fail(at, format!("unexpected `{first}`"));
                }
                let mut end = at;
                for scalar in self.text[at..].chars() {
                    if !name_char(scalar) {
                        break;
                    }
                    end += scalar.len_utf8();
                }
                // `Name.{u}`: the dot belongs to the levels.
                if self.bytes[end - 1] == b'.' && self.bytes.get(end) == Some(&b'{') {
                    end -= 1;
                }
                make(Kind::Name, end)
            }
        })
    }

    fn peek(&mut self) -> PResult<Token> {
        if let Some((from, token, _)) = self.cached {
            if from == self.at {
                return Ok(token);
            }
        }
        let (token, after) = self.scan(self.at)?;
        self.cached = Some((self.at, token, after));
        Ok(token)
    }

    /// The token after the next, not consumed.
    fn peek_second(&mut self) -> PResult<Token> {
        self.peek()?;
        let after = self.cached.map_or(self.at, |(_, _, after)| after);
        Ok(self.scan(after)?.0)
    }

    fn bump(&mut self) -> PResult<Token> {
        let token = self.peek()?;
        self.at = self.cached.map_or(self.at, |(_, _, after)| after);
        self.last_end = token.end;
        Ok(token)
    }

    fn expect(&mut self, kind: Kind, what: &str) -> PResult<Token> {
        let token = self.peek()?;
        if token.kind == kind {
            self.bump()
        } else {
            self.fail(token.start, format!("expected {what}"))
        }
    }

    fn eat(&mut self, kind: Kind) -> PResult<bool> {
        if self.peek()?.kind == kind {
            self.bump()?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn slice(&self, token: Token) -> &'a str {
        &self.text[token.start..token.end]
    }

    fn is_word(&mut self, word: &str) -> PResult<bool> {
        let token = self.peek()?;
        Ok(token.kind == Kind::Name && self.slice(token) == word)
    }

    /// A name: a bare word or a quoted string.
    fn name(&mut self, what: &str) -> PResult<String> {
        let token = self.bump()?;
        match token.kind {
            Kind::Name => {
                let word = self.slice(token);
                if KEYWORDS.contains(&word) {
                    return self.fail(
                        token.start,
                        format!("`{word}` is a keyword; quote it to write a name"),
                    );
                }
                Ok(word.to_owned())
            }
            Kind::Str => self.string(token),
            _ => self.fail(token.start, format!("expected {what}")),
        }
    }

    fn string(&self, token: Token) -> PResult<String> {
        Ok(super::lex::string(&self.text[token.start..], token.start)?.0)
    }

    fn number(&mut self, what: &str) -> PResult<usize> {
        let token = self.expect(Kind::Num, what)?;
        let digits = self.slice(token);
        if digits.len() > 1 && digits.starts_with('0') {
            return self.fail(token.start, "a numeral has no redundant leading zero");
        }
        digits
            .parse()
            .or_else(|_| self.fail(token.start, "the number is too large"))
    }

    fn enter(&self, depth: usize, at: usize) -> PResult<()> {
        if depth > MAX_DEPTH {
            self.fail(at, "nested more deeply than the notation reads")
        } else {
            Ok(())
        }
    }

    // --- levels ---

    fn level(&mut self, depth: usize) -> PResult<CoreLevel> {
        let mut level = self.level_atom(depth)?;
        while self.eat(Kind::Plus)? {
            let at = self.peek()?.start;
            let count = self.number("a number after `+`")?;
            if count > MAX_SUCC {
                return self.fail(
                    at,
                    format!("a level is raised by at most {MAX_SUCC} at once"),
                );
            }
            for _ in 0..count {
                level = CoreLevel::Succ(Box::new(level));
            }
        }
        Ok(level)
    }

    fn level_atom(&mut self, depth: usize) -> PResult<CoreLevel> {
        let token = self.peek()?;
        self.enter(depth, token.start)?;
        match token.kind {
            Kind::Num => {
                let count = self.number("a level")?;
                if count > MAX_SUCC {
                    return self.fail(token.start, format!("a level is at most {MAX_SUCC}"));
                }
                let mut level = CoreLevel::Zero;
                for _ in 0..count {
                    level = CoreLevel::Succ(Box::new(level));
                }
                Ok(level)
            }
            Kind::LParen => {
                self.bump()?;
                let level = self.level(depth + 1)?;
                self.expect(Kind::RParen, "`)`")?;
                Ok(level)
            }
            Kind::Name if matches!(self.slice(token), "max" | "imax") => {
                self.bump()?;
                self.expect(Kind::LParen, "`(` after `max`")?;
                let left = self.level(depth + 1)?;
                self.expect(Kind::Comma, "`,`")?;
                let right = self.level(depth + 1)?;
                self.expect(Kind::RParen, "`)`")?;
                let pair = Box::new((left, right));
                Ok(if self.slice(token) == "max" {
                    CoreLevel::Max(pair)
                } else {
                    CoreLevel::IMax(pair)
                })
            }
            Kind::Name | Kind::Str => Ok(CoreLevel::Param(self.name("a level")?)),
            _ => self.fail(token.start, "expected a universe level"),
        }
    }

    fn levels(&mut self, depth: usize) -> PResult<Vec<CoreLevel>> {
        let mut levels = Vec::new();
        if self.eat(Kind::LevelOpen)? {
            loop {
                levels.push(self.level(depth + 1)?);
                if !self.eat(Kind::Comma)? {
                    break;
                }
            }
            self.expect(Kind::RBrace, "`}` after the levels")?;
        }
        Ok(levels)
    }

    // --- terms ---

    fn push(&mut self, node: CoreNode) -> usize {
        self.nodes.push(node);
        self.nodes.len() - 1
    }

    /// Whether the next tokens begin a binder: `(x :`, `{`, `[`.
    fn at_binder(&mut self) -> PResult<bool> {
        let token = self.peek()?;
        Ok(match token.kind {
            Kind::LBrace | Kind::LBrack => true,
            Kind::LParen => {
                let name = self.peek_second()?;
                if matches!(name.kind, Kind::Name | Kind::Str) {
                    let after = self.scan(name.end)?.0;
                    after.kind == Kind::Colon
                } else {
                    false
                }
            }
            _ => false,
        })
    }

    /// A binder: its visibility, its name, and the node of its type.
    fn binder(&mut self, depth: usize) -> PResult<(CoreBinderInfo, String, usize)> {
        let open = self.bump()?;
        let (info, close) = match open.kind {
            Kind::LParen => (CoreBinderInfo::Explicit, Kind::RParen),
            Kind::LBrack => (CoreBinderInfo::Instance, Kind::RBrack),
            Kind::LBrace => {
                if self.eat(Kind::LBrace)? {
                    (CoreBinderInfo::StrictImplicit, Kind::RBrace)
                } else {
                    (CoreBinderInfo::Implicit, Kind::RBrace)
                }
            }
            _ => return self.fail(open.start, "expected a binder"),
        };
        let name = self.binder_name_token()?;
        self.expect(Kind::Colon, "`:` after the binder's name")?;
        let domain = self.term(depth + 1)?;
        self.expect(close, "the binder's closing bracket")?;
        if info == CoreBinderInfo::StrictImplicit {
            self.expect(Kind::RBrace, "`}}`")?;
        }
        Ok((info, name, domain))
    }

    /// A binder's name: `_` is the empty name.
    fn binder_name_token(&mut self) -> PResult<String> {
        let token = self.peek()?;
        if token.kind == Kind::Name && self.slice(token) == "_" {
            self.bump()?;
            Ok(String::new())
        } else {
            self.name("a binder's name")
        }
    }

    /// Read what `read` reads with a binder called `name` in scope.
    fn under<T>(&mut self, name: &str, read: impl FnOnce(&mut Self) -> PResult<T>) -> PResult<T> {
        self.scope.push(name.to_owned());
        let read = read(self);
        self.scope.pop();
        read
    }

    /// The index of the nearest binder called `word`, if one is in scope.
    fn resolve(&self, word: &str) -> Option<usize> {
        self.scope
            .iter()
            .rev()
            .position(|name| !name.is_empty() && name == word)
    }

    fn term(&mut self, depth: usize) -> PResult<usize> {
        let token = self.peek()?;
        self.enter(depth, token.start)?;
        if token.kind == Kind::Name {
            match self.slice(token) {
                "fun" => {
                    self.bump()?;
                    let (b, n, t) = self.binder(depth)?;
                    self.expect(Kind::FatArrow, "`=>` after the binder")?;
                    let v = self.under(&n, |parser| parser.term(depth + 1))?;
                    return Ok(self.push(CoreNode::Lambda { n, b, t, v }));
                }
                word @ ("let" | "have") => {
                    let d = word == "have";
                    self.bump()?;
                    let n = self.binder_name_token()?;
                    self.expect(Kind::Colon, "`:` after the name")?;
                    let t = self.term(depth + 1)?;
                    self.expect(Kind::ColonEq, "`:=`")?;
                    let v = self.term(depth + 1)?;
                    self.expect(Kind::Semi, "`;` after the value")?;
                    let body = self.under(&n, |parser| parser.term(depth + 1))?;
                    return Ok(self.push(CoreNode::Let { n, t, v, body, d }));
                }
                _ => {}
            }
        }
        if self.at_binder()? {
            let (b, n, t) = self.binder(depth)?;
            self.expect(Kind::Arrow, "`->` after the binder")?;
            let v = self.under(&n, |parser| parser.term(depth + 1))?;
            return Ok(self.push(CoreNode::Forall { n, b, t, v }));
        }
        let head = self.prefix_or_application(depth)?;
        if self.eat(Kind::Arrow)? {
            // The arrow's binder has no name, and the body is under it.
            let v = self.under("", |parser| parser.term(depth + 1))?;
            return Ok(self.push(CoreNode::Forall {
                n: String::new(),
                b: CoreBinderInfo::Explicit,
                t: head,
                v,
            }));
        }
        Ok(head)
    }

    fn prefix_or_application(&mut self, depth: usize) -> PResult<usize> {
        let token = self.peek()?;
        if token.kind == Kind::Name {
            match self.slice(token) {
                "Sort" => {
                    self.bump()?;
                    let l = self.level(depth + 1)?;
                    return Ok(self.push(CoreNode::Sort { l }));
                }
                "proj" => {
                    self.bump()?;
                    let s = self.name("a structure name")?;
                    let n = self.name("a projection name")?;
                    let i = self.number("a field index")?;
                    let x = self.atom(depth + 1)?;
                    return Ok(self.push(CoreNode::Projection { n, s, i, x }));
                }
                "annot" => {
                    self.bump()?;
                    let m = self.annotation()?;
                    let x = self.atom(depth + 1)?;
                    return Ok(self.push(CoreNode::Mdata { m, x }));
                }
                _ => {}
            }
        }
        let mut function = self.atom(depth)?;
        while self.at_atom()? {
            let x = self.atom(depth + 1)?;
            function = self.push(CoreNode::App { f: function, x });
        }
        Ok(function)
    }

    fn annotation(&mut self) -> PResult<Vec<CoreMetaEntry>> {
        self.expect(Kind::LBrack, "`[` after `annot`")?;
        let mut entries = Vec::new();
        while self.peek()?.kind != Kind::RBrack {
            let n = self.name("an entry's key")?;
            self.expect(Kind::ColonEq, "`:=` after the key")?;
            let tag = self.bump()?;
            if tag.kind != Kind::Name {
                return self.fail(tag.start, "expected the type of the value");
            }
            let v = match self.slice(tag) {
                "bool" => {
                    let word = self.bump()?;
                    match (word.kind, self.slice(word)) {
                        (Kind::Name, "true") => CoreMetaValue::Bool { v: true },
                        (Kind::Name, "false") => CoreMetaValue::Bool { v: false },
                        _ => return self.fail(word.start, "expected `true` or `false`"),
                    }
                }
                "nat" => CoreMetaValue::Nat {
                    v: self.digits(false)?,
                },
                "int" => CoreMetaValue::Int {
                    v: self.digits(true)?,
                },
                "str" => {
                    let token = self.expect(Kind::Str, "a string")?;
                    CoreMetaValue::Str {
                        v: self.string(token)?,
                    }
                }
                "name" => CoreMetaValue::Name {
                    v: self.name("a name")?,
                },
                other => {
                    return self.fail(
                        tag.start,
                        format!("`{other}` is not the type of an annotation value"),
                    )
                }
            };
            entries.push(CoreMetaEntry { n, v });
            if !self.eat(Kind::Comma)? {
                break;
            }
        }
        self.expect(Kind::RBrack, "`,` or `]`")?;
        Ok(entries)
    }

    /// The decimal of a natural number or an integer: a numeral, or a string
    /// for a spelling that is not canonical.
    fn digits(&mut self, signed: bool) -> PResult<String> {
        let token = self.peek()?;
        match token.kind {
            Kind::Str => {
                self.bump()?;
                self.string(token)
            }
            Kind::Minus if signed => {
                self.bump()?;
                let number = self.expect(Kind::Num, "a number after `-`")?;
                let digits = self.slice(number);
                if (digits.len() > 1 && digits.starts_with('0')) || digits == "0" {
                    return self.fail(number.start, "a numeral has no redundant leading zero");
                }
                Ok(format!("-{digits}"))
            }
            Kind::Num => {
                self.bump()?;
                let digits = self.slice(token);
                if digits.len() > 1 && digits.starts_with('0') {
                    return self.fail(token.start, "a numeral has no redundant leading zero");
                }
                Ok(digits.to_owned())
            }
            _ => self.fail(token.start, "expected a number"),
        }
    }

    /// Whether the next token begins an atom, on the line of the term it is
    /// an argument of.
    fn at_atom(&mut self) -> PResult<bool> {
        let token = self.peek()?;
        if token.col0 {
            return Ok(false);
        }
        Ok(match token.kind {
            Kind::Ref | Kind::Label | Kind::BVar | Kind::Num | Kind::Str | Kind::LParen => true,
            Kind::Name => !matches!(
                self.slice(token),
                "fun" | "let" | "have" | "Sort" | "proj" | "annot"
            ),
            _ => false,
        })
    }

    fn atom(&mut self, depth: usize) -> PResult<usize> {
        let token = self.bump()?;
        self.enter(depth, token.start)?;
        match token.kind {
            Kind::Ref => {
                let digits = &self.slice(token)[1..];
                let target: usize = digits
                    .parse()
                    .or_else(|_| self.fail(token.start, "the number is too large"))?;
                if target >= self.nodes.len() {
                    return self.fail(
                        token.start,
                        format!("`@{target}` is not a node that has been written"),
                    );
                }
                Ok(target)
            }
            Kind::Label => {
                let name = &self.slice(token)[1..];
                match self.labels.get(name) {
                    Some(target) => Ok(*target),
                    None => self.fail(
                        token.start,
                        format!("no statement written before this is named `@{name}`"),
                    ),
                }
            }
            Kind::BVar => {
                let i: usize = self.slice(token)[1..]
                    .parse()
                    .or_else(|_| self.fail(token.start, "the number is too large"))?;
                Ok(self.push(CoreNode::BVar { i }))
            }
            Kind::Num => {
                let digits = self.slice(token);
                if digits.len() > 1 && digits.starts_with('0') {
                    return self.fail(token.start, "a numeral has no redundant leading zero");
                }
                let v = digits.to_owned();
                Ok(self.push(CoreNode::Nat { v }))
            }
            Kind::Str => {
                let text = self.string(token)?;
                let v = text.chars().map(u32::from).collect();
                Ok(self.push(CoreNode::String { v }))
            }
            Kind::LParen => {
                let inner = self.term(depth + 1)?;
                self.expect(Kind::RParen, "`)`")?;
                Ok(inner)
            }
            Kind::Name => match self.slice(token) {
                "proof" => {
                    self.expect(Kind::LParen, "`(` after `proof`")?;
                    let before = self.nodes.len();
                    let inner = self.term(depth + 1)?;
                    let close = self.expect(Kind::RParen, "`)`")?;
                    if inner + 1 != self.nodes.len() || inner < before {
                        return self.fail(
                            close.start,
                            "`proof` marks a node written inside it, not a reference",
                        );
                    }
                    self.proof.push(inner);
                    Ok(inner)
                }
                "nat" => {
                    let literal = self.expect(Kind::Str, "a quoted number after `nat`")?;
                    let v = self.string(literal)?;
                    Ok(self.push(CoreNode::Nat { v }))
                }
                "const" => {
                    let n = {
                        let token = self.expect(Kind::Str, "a quoted name after `const`")?;
                        self.string(token)?
                    };
                    let u = self.levels(depth)?;
                    Ok(self.push(CoreNode::Const { n, u }))
                }
                word if KEYWORDS.contains(&word) => self.fail(
                    token.start,
                    format!("`{word}` is a keyword; a term that begins with it is written in parentheses"),
                ),
                "_" => self.fail(token.start, "`_` is the name of a binder, not a constant"),
                word => {
                    // A name a binder in scope has is that binder's variable.
                    if let Some(i) = self.resolve(word) {
                        if self.peek()?.kind == Kind::LevelOpen {
                            return self.fail(
                                token.end,
                                format!("`{word}` is a bound variable and has no universe levels"),
                            );
                        }
                        return Ok(self.push(CoreNode::BVar { i }));
                    }
                    let n = word.to_owned();
                    let u = self.levels(depth)?;
                    Ok(self.push(CoreNode::Const { n, u }))
                }
            },
            _ => self.fail(token.start, "expected a term"),
        }
    }

    // --- the module ---

    fn imports(&mut self) -> PResult<Vec<String>> {
        let token = self.peek()?;
        if !self.is_word("imports")? {
            return self.fail(token.start, "a core module begins with its `imports`");
        }
        self.bump()?;
        self.expect(Kind::LBrack, "`[` after `imports`")?;
        let mut imports = Vec::new();
        while self.peek()?.kind != Kind::RBrack {
            imports.push(self.name("a module name")?);
            if !self.eat(Kind::Comma)? {
                break;
            }
        }
        self.expect(Kind::RBrack, "`,` or `]`")?;
        Ok(imports)
    }

    /// The text between the braces at the next token, to its match: strings
    /// are skipped whole.
    fn members(&mut self) -> PResult<Option<(usize, usize)>> {
        let token = self.peek()?;
        if token.kind != Kind::LBrace || token.col0 {
            return Ok(None);
        }
        let mut depth = 0usize;
        let mut at = token.start;
        while at < self.bytes.len() {
            match self.bytes[at] {
                b'"' => {
                    let (_, length) = super::lex::string(&self.text[at..], at)?;
                    at += length;
                    continue;
                }
                b'{' => depth += 1,
                b'}' => {
                    depth -= 1;
                    if depth == 0 {
                        self.at = at + 1;
                        self.last_end = at + 1;
                        return Ok(Some((token.start, at + 1)));
                    }
                }
                _ => {}
            }
            at += 1;
        }
        self.fail(token.start, "the braces of the declaration are not closed")
    }

    fn declaration(&mut self) -> PResult<Pending> {
        let kind = self.bump()?;
        let kind_word = self.slice(kind);
        if !matches!(
            kind_word,
            "abbrev" | "definition" | "theorem" | "inductive" | "constructor" | "recursor"
        ) {
            return self.fail(
                kind.start,
                format!("`{kind_word}` does not begin a declaration or a node"),
            );
        }
        let name = self.name("the declaration's name")?;
        let mut levels = Vec::new();
        if self.eat(Kind::LevelOpen)? {
            loop {
                levels.push(self.name("a universe parameter")?);
                if !self.eat(Kind::Comma)? {
                    break;
                }
            }
            self.expect(Kind::RBrace, "`}` after the universe parameters")?;
        }
        self.expect(Kind::Colon, "`:` and the type")?;
        let r#type = self.root("the type")?;
        let value = if self.eat(Kind::ColonEq)? {
            Some(self.root("the value")?)
        } else {
            None
        };
        let mut object = std::collections::BTreeMap::new();
        if let Some((from, to)) = self.members()? {
            match generic::parse(&self.text[from..to]) {
                Ok(Json::Obj(members)) => object = members,
                Ok(_) => return self.fail(from, "the members of a declaration are an object"),
                Err(failure) => return self.fail(from + failure.offset, failure.message),
            }
        }
        object.insert("kind".to_owned(), Json::Str(kind_word.to_owned()));
        object.insert("name".to_owned(), Json::Str(name));
        object.insert(
            "levels".to_owned(),
            Json::Arr(levels.into_iter().map(Json::Str).collect()),
        );
        Ok(Pending {
            object,
            r#type,
            value,
            start: kind.start,
        })
    }

    /// A declaration whose roots are known: the statements that carry a
    /// person's names have all been read.
    fn finish(&self, pending: Pending) -> PResult<CoreDeclaration> {
        let mut object = pending.object;
        let resolve = |root: Root| -> PResult<usize> {
            match root {
                Root::Id(id) => Ok(id),
                Root::Label(name, at) => self
                    .labels
                    .get(&name)
                    .copied()
                    .ok_or_else(|| error(at, format!("no statement is named `@{name}`"))),
            }
        };
        object.insert(
            "type".to_owned(),
            Json::from_usize(resolve(pending.r#type)?),
        );
        if let Some(value) = pending.value {
            object.insert("value".to_owned(), Json::from_usize(resolve(value)?));
        }
        serde_json::from_str(&Json::Obj(object).to_canonical_string())
            .map_err(|failure| error(pending.start, format!("not a declaration: {failure}")))
    }

    /// A node a declaration states: `@N`, or the name of a statement.
    fn root(&mut self, what: &str) -> PResult<Root> {
        let token = self.bump()?;
        match token.kind {
            Kind::Ref => self.slice(token)[1..]
                .parse()
                .map(Root::Id)
                .or_else(|_| self.fail(token.start, "the number is too large")),
            Kind::Label => Ok(Root::Label(self.slice(token)[1..].to_owned(), token.start)),
            _ => self.fail(token.start, format!("expected `@N`, {what}")),
        }
    }

    fn statement(&mut self) -> PResult<()> {
        let label = self.bump()?;
        // `@N` states the number of the node; `@name` names it, and `fmt`
        // writes the number.
        let target: Result<usize, String> = if label.kind == Kind::Ref {
            Ok(self.slice(label)[1..]
                .parse()
                .or_else(|_| self.fail(label.start, "the number is too large"))?)
        } else {
            Err(self.slice(label)[1..].to_owned())
        };
        self.expect(Kind::ColonEq, "`:=` after the label")?;
        let before = self.nodes.len();
        let defined = self.term(0)?;
        if defined < before {
            return self.fail(
                label.start,
                "a statement defines a node, not another name for one",
            );
        }
        if defined + 1 != self.nodes.len() {
            return self.fail(
                label.start,
                "a statement defines the node its term ends with",
            );
        }
        match target {
            Ok(target) if target != defined => {
                return self.fail(
                    label.start,
                    format!(
                        "this statement defines node {defined}, not node {target}: the label is the number of the node"
                    ),
                );
            }
            Ok(_) => {}
            Err(name) => {
                if self.labels.insert(name.clone(), defined).is_some() {
                    return self.fail(label.start, format!("`@{name}` names two statements"));
                }
            }
        }
        let after = self.peek()?;
        if after.kind != Kind::End && !after.col0 {
            return self.fail(after.start, "a statement ends at the end of its line");
        }
        Ok(())
    }
}

/// A node a declaration states, before the statements that name a node have
/// been read.
enum Root {
    Id(usize),
    Label(String, usize),
}

/// A declaration read up to its roots.
struct Pending {
    object: std::collections::BTreeMap<String, Json>,
    r#type: Root,
    value: Option<Root>,
    start: usize,
}

/// Read the notation of a core module, and the byte range of each of its
/// declarations in the text.
///
/// # Errors
///
/// Text that is not the notation of a module, with the byte offset of the
/// first thing that is not.
pub fn parse_module_with_spans(
    text: &str,
) -> Result<(CoreModule, Vec<(usize, usize)>), SurfaceError> {
    let mut parser = Parser::new(text);
    let imports = parser.imports()?;
    let mut pending = Vec::new();
    let mut spans = Vec::new();
    let mut statements = false;
    loop {
        let token = parser.peek()?;
        if token.kind == Kind::End {
            break;
        }
        if !token.col0 {
            return parser.fail(token.start, "a declaration or a node begins its line");
        }
        if matches!(token.kind, Kind::Ref | Kind::Label) {
            statements = true;
            parser.statement()?;
        } else if statements {
            return parser.fail(token.start, "the declarations come before the nodes");
        } else {
            pending.push(parser.declaration()?);
            spans.push((token.start, parser.last_end));
        }
    }
    let mut declarations = Vec::with_capacity(pending.len());
    for declaration in pending {
        declarations.push(parser.finish(declaration)?);
    }
    Ok((
        CoreModule {
            spec: CORE_SPEC_2.to_owned(),
            imports,
            nodes: parser.nodes,
            proof_nodes: parser.proof,
            declarations,
        },
        spans,
    ))
}

/// Read the notation of a core module.
///
/// # Errors
///
/// Text that is not the notation of a module, with the byte offset of the
/// first thing that is not.
pub fn parse_module(text: &str) -> Result<CoreModule, SurfaceError> {
    parse_module_with_spans(text).map(|(module, _)| module)
}

/// The byte range in `text` of every declaration of the module, in order:
/// what the source map relates to a declaration's rendering.
///
/// # Errors
///
/// Text that is not the notation of a module.
pub fn declaration_spans(text: &str) -> Result<Vec<(usize, usize)>, SurfaceError> {
    parse_module_with_spans(text).map(|(_, spans)| spans)
}

/// The canonical JSON of the module a notation reads to, under `spec`.
///
/// # Errors
///
/// Text that is not the notation of a module, or a `spec` other than
/// `lexlean/core-module/2`.
pub fn data_text(text: &str, spec: &str) -> Result<String, SurfaceError> {
    if spec != CORE_SPEC_2 {
        return Err(error(
            0,
            format!("the notation names the schema `{CORE_SPEC_2}`, not `{spec}`"),
        ));
    }
    canonical_json(&parse_module(text)?)
}

#[cfg(test)]
mod tests {
    use super::{canonical_json, parse_module, print_module};
    use crate::ir::core::{CoreModule, CoreNode, CORE_SPEC_2};

    /// A module of six nodes: a sort, a constant, an application used twice,
    /// a variable, a lambda over them, and an arrow whose two sides are the
    /// shared application.
    fn sample() -> CoreModule {
        let nodes = [
            r#"{"k":"s","l":{"a":{"a":"u","k":"p"},"k":"s"}}"#,
            r#"{"k":"c","n":"Nat","u":[]}"#,
            r#"{"f":1,"k":"a","x":1}"#,
            r#"{"i":0,"k":"b"}"#,
            r#"{"b":"e","k":"l","n":"n","t":1,"v":3}"#,
            r#"{"b":"e","k":"p","n":"","t":2,"v":2}"#,
        ]
        .join(",");
        let definition = r#"{"class":false,"generated":false,"kind":"definition","levels":["u"],"name":"T","policy":{"kind":"none"},"reducibility":{"height":0,"kind":"regular"},"transparency":"semireducible","type":5,"value":4}"#;
        let text = format!(
            "{{\"declarations\":[{definition}],\"imports\":[\"Init\"],\"nodes\":[{nodes}],\"proof_nodes\":[3],\"spec\":\"{CORE_SPEC_2}\"}}"
        );
        serde_json::from_str(&text).expect("a module")
    }

    /// A module reads back to itself, its canonical JSON is the JSON it was
    /// decoded from, and a node used once is written inside its user.
    #[test]
    fn a_module_reads_back_to_itself() {
        let module = sample();
        let text = print_module(&module).expect("prints");
        assert_eq!(parse_module(&text).expect("reads"), module, "{text}");
        assert_eq!(
            print_module(&parse_module(&text).expect("reads")).expect("prints"),
            text
        );
        assert!(
            text.contains("@2 := @1 @1\n"),
            "the shared application is a statement: {text}"
        );
        assert!(
            text.contains("@4 := fun (n : @1) => proof(n)\n"),
            "the variable is written inside the lambda, by its binder's name: {text}"
        );
        let json = canonical_json(&module).expect("writes");
        assert_eq!(
            serde_json::from_str::<CoreModule>(&json).expect("decodes"),
            module
        );
    }

    /// Malformed text is an error at an offset, and never a panic.
    #[test]
    fn malformed_text_is_an_error() {
        for text in [
            "",
            "imports",
            "imports [Init",
            "imports [Init]\n\n@0 := ",
            "imports [Init]\n\n@1 := Nat\n",
            "imports [Init]\n\n@0 := @0\n",
            "imports [Init]\n\n@0 := (Nat\n",
            "imports [Init]\n\n@0 := Nat Nat\n@5 := Nat\n",
            "imports [Init]\n\ntheorem T : @0\n",
            "imports [Init]\n\n@0 := fun x => x\n",
            "imports [Init]\n\n@0 := proof(Nat\n",
            "imports [Init]\n\n@0 := Sort u+99999\n",
            "imports [Init]\n\n@0 := 007\n",
            "imports [Init]\n\n@0 := \"open\n",
            "imports [Init]\n\n@0 := let\n",
        ] {
            assert!(parse_module(text).is_err(), "{text:?}");
        }
    }

    /// What a person writes (names for binders and for statements) reads to
    /// the module its canonical notation reads to, which keeps the names of the
    /// binders and writes the numbers of the statements.
    fn authored(text: &str, canonical: &str) -> CoreModule {
        let module = parse_module(text).unwrap_or_else(|error| panic!("{text}: {error}"));
        assert_eq!(print_module(&module).expect("prints"), canonical);
        assert_eq!(
            parse_module(canonical).expect("reads"),
            module,
            "{canonical}"
        );
        module
    }

    /// A variable is written by the name of its binder, read back to the index
    /// of the nearest binder of that name.
    #[test]
    fn a_variable_is_written_by_its_binders_name() {
        let module = authored(
            "imports [Init]\n\n@nat := Nat\n@k := fun (x : @nat) => fun (y : @nat) => x\n",
            "imports [Init]\n\n\n@0 := Nat\n@3 := fun (x : @0) => fun (y : @0) => x\n",
        );
        assert_eq!(module.nodes[1], CoreNode::BVar { i: 1 });
        // The body of an arrow is under a binder that has no name.
        let module = authored(
            "imports [Init]\n\n@nat := Nat\n@k := fun (x : @nat) => @nat -> x\n",
            "imports [Init]\n\n\n@0 := Nat\n@3 := fun (x : @0) => @0 -> x\n",
        );
        assert_eq!(module.nodes[1], CoreNode::BVar { i: 1 });
        authored(
            "imports [Init]\n\n@nat := Nat\n@k := fun (x : @nat) => let z : @nat := x; z\n",
            "imports [Init]\n\n\n@0 := Nat\n@4 := fun (x : @0) => let z : @0 := x; z\n",
        );
    }

    /// A variable is written by its index when no name can say which binder it
    /// is: the nearer binder has the name, the binder's name is not a word, or
    /// no binder in the statement is the one.
    #[test]
    fn a_variable_no_name_can_say_is_written_by_its_index() {
        authored(
            "imports [Init]\n\n@nat := Nat\n@k := fun (x : @nat) => fun (x : @nat) => #1\n",
            "imports [Init]\n\n\n@0 := Nat\n@3 := fun (x : @0) => fun (x : @0) => #1\n",
        );
        authored(
            "imports [Init]\n\n@nat := Nat\n@k := fun (x : @nat) => fun (x : @nat) => x\n",
            "imports [Init]\n\n\n@0 := Nat\n@3 := fun (x : @0) => fun (x : @0) => x\n",
        );
        authored(
            "imports [Init]\n\n@nat := Nat\n@k := fun (\"fun\" : @nat) => #0\n",
            "imports [Init]\n\n\n@2 := fun (\"fun\" : Nat) => #0\n",
        );
        authored(
            "imports [Init]\n\n@nat := Nat\n@k := fun (_ : @nat) => #0\n",
            "imports [Init]\n\n\n@2 := fun (_ : Nat) => #0\n",
        );
        authored(
            "imports [Init]\n\n@v := #3\n",
            "imports [Init]\n\n\n@0 := #3\n",
        );
    }

    /// A constant that a binder in scope has the name of is written as a
    /// string, and a name no binder has is a constant.
    #[test]
    fn a_constant_a_binder_shadows_is_written_as_a_string() {
        let module = authored(
            "imports [Init]\n\n@k := fun (Nat : Sort 1) => const \"Nat\" Nat\n",
            "imports [Init]\n\n\n@4 := fun (Nat : Sort 1) => const \"Nat\" Nat\n",
        );
        assert_eq!(
            module.nodes[1],
            CoreNode::Const {
                n: "Nat".to_owned(),
                u: Vec::new()
            }
        );
        assert_eq!(module.nodes[2], CoreNode::BVar { i: 0 });
        authored(
            "imports [Init]\n\n@k := fun (x : Sort 1) => Nat x\n",
            "imports [Init]\n\n\n@4 := fun (x : Sort 1) => Nat x\n",
        );
    }

    /// A name that is not defined is an error at its position.
    #[test]
    fn a_name_that_is_not_defined_is_an_error() {
        for text in [
            "imports [Init]\n\n@a := @b\n",
            "imports [Init]\n\n@a := Nat\n@a := Nat\n",
            "imports [Init]\n\ntheorem T : @missing\n",
            "imports [Init]\n\n@a := fun (x : Sort 1) => x.{0}\n",
            "imports [Init]\n\n@a := @a\n",
        ] {
            let failure = parse_module(text).expect_err(text);
            assert!(failure.offset <= text.len(), "{text}: {failure}");
        }
    }
}
