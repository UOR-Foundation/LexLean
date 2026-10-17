//! The backends of a tooling module (SPEC.md §17.20): canonical Lean and
//! canonical LaTeX, both generated from the one linked value. Nothing is
//! copied from the source: every token of the output is the rendering of a
//! node of the IR, and no member of the IR holds Lean text.

use std::collections::{BTreeMap, BTreeSet};

use crate::backend::semantic::{
    emit_declarations_for, string_literal, tex_escape, DeclarationSource,
};
use crate::backend::Emitter;
use crate::code;
use crate::diagnostic::Diagnostic;
use crate::ir::tooling::{
    BinderInfo, CommandAtom, InvokeArgument, ParameterKind, Sig, ToolConstructor, ToolDeclaration,
    ToolTerm, ToolType, ToolingModule, TypeParameter,
};
use crate::ir::tooling_check::{typings, FnType};
use crate::link::CheckedModule;

const SOURCE: DeclarationSource<'static> = DeclarationSource {
    control: "\\toolingdata{",
    owner: "lexlean.core::toolingdata",
    environment: "toolingmodule",
    notation_owner: "lexlean.core::toolingsource",
    node: "tooling-declaration",
    skip_blank: true,
};

/// A Lean name, with every reserved segment quoted so that its identity is
/// exactly the registered one. Unlike a semantic name it may be an `IO` or a
/// `throwError` constant of the authority.
fn name(full: &str) -> String {
    full.split('.')
        .map(|segment| {
            if crate::backend::lean_tokens::is_reserved(segment) {
                format!("«{segment}»")
            } else {
                segment.to_owned()
            }
        })
        .collect::<Vec<_>>()
        .join(".")
}

/// A constructor written after the dot, which Lean resolves against the
/// type it expects: `.let` in a pattern, whatever the segment.
fn leading(short: &str) -> String {
    format!(".{}", name(short))
}

fn character(value: u32) -> String {
    match char::from_u32(value).expect("checked by the module") {
        '\'' => "'\\''".to_owned(),
        '\\' => "'\\\\'".to_owned(),
        '\n' => "'\\n'".to_owned(),
        '\r' => "'\\r'".to_owned(),
        '\t' => "'\\t'".to_owned(),
        c if c.is_ascii_graphic() || c == ' ' => format!("'{c}'"),
        c => format!("'\\u{:04x}'", u32::from(c)),
    }
}

struct Lean<'a> {
    module: &'a ToolingModule,
    typings: BTreeMap<String, FnType>,
    parameters: BTreeMap<&'a str, &'a [TypeParameter]>,
    calls: BTreeSet<&'a str>,
}

impl<'a> Lean<'a> {
    fn new(module: &'a ToolingModule) -> Result<Self, Diagnostic> {
        let typings = typings(module).map_err(|failure| {
            Diagnostic::new(
                failure.code,
                format!("phase lean-backend: {}", failure.reason),
            )
        })?;
        let parameters = module
            .declarations
            .iter()
            .filter_map(|declaration| match declaration {
                ToolDeclaration::Type {
                    name, parameters, ..
                } => Some((name.as_str(), parameters.as_slice())),
                _ => None,
            })
            .collect();
        let calls = module
            .declarations
            .iter()
            .filter_map(|declaration| match declaration {
                ToolDeclaration::Call { name, .. } => Some(name.as_str()),
                _ => None,
            })
            .collect();
        Ok(Self {
            module,
            typings,
            parameters,
            calls,
        })
    }

    fn ty(&self, ty: &ToolType) -> String {
        match ty {
            ToolType::Const {
                name: head,
                arguments,
            }
            | ToolType::Parameter {
                name: head,
                arguments,
            } => {
                if arguments.is_empty() {
                    name(head)
                } else {
                    format!(
                        "({} {})",
                        name(head),
                        arguments
                            .iter()
                            .map(|argument| self.ty(argument))
                            .collect::<Vec<_>>()
                            .join(" ")
                    )
                }
            }
            ToolType::Function { parameters, result } => format!(
                "({} → {})",
                parameters
                    .iter()
                    .map(|parameter| self.ty(parameter))
                    .collect::<Vec<_>>()
                    .join(" → "),
                self.ty(result)
            ),
            ToolType::Index { constructor } => name(constructor),
        }
    }

    fn sig(&self, sig: &Sig) -> String {
        match sig {
            Sig::Const { name: constant } => name(constant),
            Sig::Var { name: variable } => name(variable),
            Sig::App {
                function,
                arguments,
            } => {
                // A function applied to every argument it takes, the implicit
                // ones included, and a type applied to the instances it carries
                // beyond its stated parameters, are written explicitly.
                let explicit = match function.as_ref() {
                    Sig::Const { name: head } => {
                        self.calls.contains(head.as_str())
                            || self
                                .parameters
                                .get(head.as_str())
                                .is_some_and(|parameters| arguments.len() > parameters.len())
                    }
                    _ => false,
                };
                format!(
                    "({}{} {})",
                    if explicit { "@" } else { "" },
                    self.sig(function),
                    arguments
                        .iter()
                        .map(|argument| self.sig(argument))
                        .collect::<Vec<_>>()
                        .join(" ")
                )
            }
            Sig::Arrow { domain, codomain } => {
                format!("({} → {})", self.sig(domain), self.sig(codomain))
            }
            Sig::Pi {
                name: binder,
                info,
                domain,
                codomain,
            } => {
                let domain = self.sig(domain);
                let binder_text = match (info, binder) {
                    (BinderInfo::Explicit, Some(binder)) => {
                        format!("({} : {domain})", name(binder))
                    }
                    (BinderInfo::Implicit, Some(binder)) => {
                        format!("{{{} : {domain}}}", name(binder))
                    }
                    (BinderInfo::Instance, Some(binder)) => {
                        format!("[{} : {domain}]", name(binder))
                    }
                    (BinderInfo::Instance, None) => format!("[{domain}]"),
                    _ => unreachable!("an unnamed binder is an instance, checked by the module"),
                };
                format!("({binder_text} → {})", self.sig(codomain))
            }
            Sig::Type { level: None } => "Type".to_owned(),
            Sig::Type {
                level: Some(crate::ir::tooling::SigLevel::Parameter { name: level }),
            } => format!("(Type {level})"),
            Sig::OptParam { domain, default } => {
                format!("(optParam {} {})", self.sig(domain), self.sig(default))
            }
            Sig::EmptyStructure => "{ }".to_owned(),
            Sig::Nat { value } => value.clone(),
        }
    }

    /// The named arguments that give a call's or constructor's implicit
    /// parameters, in the callee's own binder names.
    fn named(&self, names: &[String], arguments: &[ToolType]) -> String {
        names
            .iter()
            .zip(arguments)
            .map(|(binder, argument)| format!(" ({} := {})", name(binder), self.ty(argument)))
            .collect()
    }

    fn term(&self, term: &ToolTerm) -> String {
        match term {
            ToolTerm::Var { name: variable } => name(variable),
            ToolTerm::Nat { value } => value.clone(),
            ToolTerm::String { value } => string_literal(value),
            ToolTerm::Char { value } => character(*value),
            ToolTerm::Name { value } => format!("``{}", name(value)),
            ToolTerm::Call {
                function,
                type_arguments,
                arguments,
            } => {
                let typing = self
                    .typings
                    .get(function)
                    .expect("a callee is a registered call or a function, checked by the module");
                format!(
                    "({}{}{})",
                    name(function),
                    self.named(&typing.type_parameters, type_arguments),
                    arguments
                        .iter()
                        .map(|argument| format!(" {}", self.term(argument)))
                        .collect::<String>()
                )
            }
            ToolTerm::Construct {
                constructor,
                type_arguments,
                arguments,
            } => {
                let owner = constructor.rsplit_once('.').map_or("", |(owner, _)| owner);
                let parameters: Vec<String> = self
                    .parameters
                    .get(owner)
                    .map(|parameters| parameters.iter().map(|p| p.name.clone()).collect())
                    .unwrap_or_default();
                format!(
                    "({}{}{})",
                    name(constructor),
                    self.named(&parameters, type_arguments),
                    arguments
                        .iter()
                        .map(|argument| format!(" {}", self.term(argument)))
                        .collect::<String>()
                )
            }
            ToolTerm::Field { value, field } => {
                format!("({}).{}", self.term(value), name(field))
            }
            ToolTerm::List { element, elements } => format!(
                "(([{}] : List {}))",
                elements
                    .iter()
                    .map(|item| self.term(item))
                    .collect::<Vec<_>>()
                    .join(", "),
                self.ty(element)
            ),
            ToolTerm::Array { element, elements } => format!(
                "((#[{}] : Array {}))",
                elements
                    .iter()
                    .map(|item| self.term(item))
                    .collect::<Vec<_>>()
                    .join(", "),
                self.ty(element)
            ),
            ToolTerm::Let {
                binder,
                ty,
                value,
                body,
            } => format!(
                "(let {} : {} := {}; {})",
                name(binder),
                self.ty(ty),
                self.term(value),
                self.term(body)
            ),
            ToolTerm::Lambda { parameters, body } => format!(
                "(fun {} => {})",
                parameters
                    .iter()
                    .map(|p| format!("({} : {})", name(&p.name), self.ty(&p.ty)))
                    .collect::<Vec<_>>()
                    .join(" "),
                self.term(body)
            ),
            ToolTerm::Apply {
                function,
                arguments,
            } => format!(
                "({}{})",
                self.term(function),
                arguments
                    .iter()
                    .map(|argument| format!(" {}", self.term(argument)))
                    .collect::<String>()
            ),
            ToolTerm::If {
                condition,
                then_branch,
                else_branch,
            } => format!(
                "(if {} then {} else {})",
                self.term(condition),
                self.term(then_branch),
                self.term(else_branch)
            ),
            ToolTerm::Match {
                scrutinee,
                branches,
            } => {
                let mut text = format!("(match {} with", self.term(scrutinee));
                for branch in branches {
                    match &branch.constructor {
                        Some(full) => {
                            let short = full.rsplit_once('.').map_or(full.as_str(), |(_, s)| s);
                            text.push_str(&format!(" | {}", leading(short)));
                            for binder in &branch.binders {
                                text.push(' ');
                                text.push_str(&name(binder));
                            }
                        }
                        None => text.push_str(" | _"),
                    }
                    text.push_str(&format!(" => {}", self.term(&branch.body)));
                }
                text.push(')');
                text
            }
        }
    }

    /// The universe parameters the registered signatures name.
    fn universes(&self) -> BTreeSet<String> {
        fn walk(sig: &Sig, out: &mut BTreeSet<String>) {
            match sig {
                Sig::Type {
                    level: Some(crate::ir::tooling::SigLevel::Parameter { name }),
                } => {
                    out.insert(name.clone());
                }
                Sig::App {
                    function,
                    arguments,
                } => {
                    walk(function, out);
                    for argument in arguments {
                        walk(argument, out);
                    }
                }
                Sig::Arrow { domain, codomain }
                | Sig::Pi {
                    domain, codomain, ..
                } => {
                    walk(domain, out);
                    walk(codomain, out);
                }
                Sig::OptParam { domain, default } => {
                    walk(domain, out);
                    walk(default, out);
                }
                _ => {}
            }
        }
        let mut out = BTreeSet::new();
        for declaration in &self.module.declarations {
            match declaration {
                ToolDeclaration::Call { signature, .. } => walk(signature, &mut out),
                ToolDeclaration::Invoke { arguments, .. } => {
                    for argument in arguments {
                        if let InvokeArgument::Type { value } = argument {
                            walk(value, &mut out);
                        }
                    }
                }
                _ => {}
            }
        }
        out
    }

    fn declaration(&self, declaration: &ToolDeclaration) -> String {
        match declaration {
            ToolDeclaration::Type { .. }
            | ToolDeclaration::Call { .. }
            | ToolDeclaration::Plumbing { .. } => String::new(),
            ToolDeclaration::Function {
                name: function,
                public,
                partial,
                type_parameters,
                parameters,
                result,
                body,
                ..
            } => {
                let mut text = String::new();
                if *public {
                    text.push_str("public ");
                }
                text.push_str("meta ");
                if *partial {
                    text.push_str("partial ");
                }
                text.push_str("def ");
                text.push_str(&name(function));
                for parameter in type_parameters {
                    text.push_str(&format!(" {{{} : Type}}", name(parameter)));
                }
                for parameter in parameters {
                    text.push_str(&format!(
                        " ({} : {})",
                        name(&parameter.name),
                        self.ty(&parameter.ty)
                    ));
                }
                text.push_str(&format!(
                    " : {} :=\n  {}\n",
                    self.ty(result),
                    self.term(body)
                ));
                text
            }
            ToolDeclaration::Command {
                name: command,
                atoms,
                elaborator,
            } => {
                let mut text = format!("syntax (name := {}) ", name(command));
                for atom in atoms {
                    match atom {
                        CommandAtom::Token { text: token } => {
                            text.push_str(&string_literal(token));
                        }
                        CommandAtom::Ident => text.push_str("ident"),
                        CommandAtom::Type => text.push_str("term"),
                    }
                    text.push(' ');
                }
                text.push_str(&format!(
                    ": command\nattribute [command_elab {}] {}\n",
                    name(command),
                    name(elaborator)
                ));
                text
            }
            ToolDeclaration::Invoke { command, arguments } => {
                let atoms = self
                    .module
                    .declarations
                    .iter()
                    .find_map(|candidate| match candidate {
                        ToolDeclaration::Command {
                            name: registered,
                            atoms,
                            ..
                        } if registered == command => Some(atoms),
                        _ => None,
                    })
                    .expect("an invocation names a registered command, checked by the module");
                let mut arguments = arguments.iter();
                let mut text = String::new();
                for atom in atoms {
                    match atom {
                        CommandAtom::Token { text: token } => text.push_str(token),
                        CommandAtom::Ident | CommandAtom::Type => {
                            match arguments.next().expect("one argument per atom") {
                                InvokeArgument::Ident { value } => text.push_str(&name(value)),
                                InvokeArgument::Type { value } => text.push_str(&self.sig(value)),
                            }
                        }
                    }
                }
                text.push('\n');
                text
            }
            ToolDeclaration::Evaluate {
                function,
                arguments,
            } => format!(
                "#eval {}{}\n",
                name(function),
                arguments
                    .iter()
                    .map(|argument| format!(" {}", self.term(argument)))
                    .collect::<String>()
            ),
        }
    }
}

/// The Lean text of a tooling module and where each declaration's rendering
/// starts in it (a declaration with no output starts where the next one
/// does).
pub struct LeanText {
    /// The module's text.
    pub text: String,
    /// The byte offset at which each declaration's rendering starts.
    pub starts: Vec<usize>,
    /// The byte offset at which the last declaration's rendering ends.
    pub end: usize,
}

/// Render a tooling module as Lean (SPEC.md §17.20): `module`, the fixed
/// imports and options, the universe variables its signatures name, then one
/// rendering per declaration in order, a contiguous mutual group inside one
/// `mutual` block. `imports` are project modules, each imported with all its
/// declarations visible, under `module_prefix` unless it is empty, which
/// names them in full (the extraction driver imports the generated modules of
/// the project it reads, by their own names, §22.10).
///
/// # Errors
///
/// `LLI9001` when the module is not well typed or a rendering contains a
/// comment token.
pub fn lean_text(
    module: &ToolingModule,
    imports: &[String],
    module_prefix: &str,
) -> Result<LeanText, Diagnostic> {
    let lean = Lean::new(module)?;
    let mut text = String::from("module\npublic meta import Lean\n");
    for import in imports {
        let full = if module_prefix.is_empty() {
            import.clone()
        } else {
            format!("{module_prefix}.{import}")
        };
        text.push_str(&format!("import all {}\n", name(&full)));
    }
    text.push_str("set_option autoImplicit false\nset_option linter.unusedVariables false\n");
    let universes = lean.universes();
    if !universes.is_empty() {
        text.push_str(&format!(
            "universe {}\n",
            universes.into_iter().collect::<Vec<_>>().join(" ")
        ));
    }
    let group_of = |index: usize| match module.declarations.get(index) {
        Some(ToolDeclaration::Function {
            mutual: Some(label),
            ..
        }) => Some(label.as_str()),
        _ => None,
    };
    let mut starts = Vec::with_capacity(module.declarations.len());
    for (index, declaration) in module.declarations.iter().enumerate() {
        starts.push(text.len());
        let rendered = lean.declaration(declaration);
        if rendered.is_empty() {
            continue;
        }
        text.push('\n');
        let group = group_of(index);
        if group.is_some() && (index == 0 || group_of(index - 1) != group) {
            text.push_str("mutual\n");
        }
        text.push_str(&rendered);
        if group.is_some() && group_of(index + 1) != group {
            text.push_str("end\n");
        }
    }
    let end = text.len();
    if text.contains("--") || text.contains("/-") {
        return Err(Diagnostic::new(
            code!("LLI9001"),
            "phase lean-backend: a tooling rendering contains a comment token",
        ));
    }
    Ok(LeanText { text, starts, end })
}

/// Render a tooling module as Lean with its source map (§17.20, §20.3).
///
/// # Errors
///
/// `LLI9001` when the source declarations and the rendered ones disagree.
pub fn render_lean(
    checked: &CheckedModule,
    module: &ToolingModule,
    module_prefix: &str,
) -> Result<Emitter, Diagnostic> {
    let rendered = lean_text(module, &module.imports, module_prefix)?;
    emit_declarations_for(
        checked,
        &rendered.text,
        "tooling-lean-module",
        &rendered.starts,
        rendered.end,
        &SOURCE,
    )
}

fn constructor_line(constructor: &ToolConstructor, lean: &Lean<'_>) -> String {
    let fields = constructor
        .fields
        .iter()
        .map(|field| format!("({} : {})", field.name, lean.ty(&field.ty)))
        .collect::<Vec<_>>()
        .join(" ");
    let requires = constructor.requires.as_ref().map_or_else(String::new, |r| {
        format!(" [only at {} = {}]", r.parameter, r.constructor)
    });
    format!("{} {fields}{requires}", constructor.name)
}

/// Render a tooling module as canonical LaTeX: one entry per declaration,
/// stating its kind and everything the Lean rendering is made from (the same
/// types, signatures, and terms, spelled by the same printer).
///
/// # Errors
///
/// `LLI9001` when the source declarations and the rendered ones disagree.
pub fn render_latex(
    checked: &CheckedModule,
    module: &ToolingModule,
) -> Result<Emitter, Diagnostic> {
    let lean = Lean::new(module)?;
    let mut text = String::from(
        "\\documentclass[11pt]{article}\n\\usepackage[T1]{fontenc}\n\\usepackage{amsmath,amssymb}\n\\begin{document}\n\\section*{Tooling declarations}\n",
    );
    let mut starts = Vec::with_capacity(module.declarations.len());
    for declaration in &module.declarations {
        starts.push(text.len());
        let (title, lines): (String, Vec<String>) = match declaration {
            ToolDeclaration::Type {
                name: type_name,
                parameters,
                constructors,
                alias,
                opaque,
                source,
            } => {
                let mut lines = vec![format!("Source: {} ({})", source.file, source.sha256)];
                if *opaque {
                    lines.push("Constructors are not pinned (opaque)".to_owned());
                }
                if let Some(alias) = alias {
                    lines.push(format!("Abbreviation of: {}", lean.ty(alias)));
                }
                if !parameters.is_empty() {
                    lines.push(format!(
                        "Parameters: {}",
                        parameters
                            .iter()
                            .map(|p| match &p.kind {
                                ParameterKind::Type => format!("{} : Type", p.name),
                                ParameterKind::Constructor => format!("{} : Type -> Type", p.name),
                                ParameterKind::Index { of } => format!("{} : {of}", p.name),
                            })
                            .collect::<Vec<_>>()
                            .join(", ")
                    ));
                }
                for constructor in constructors {
                    lines.push(format!(
                        "Constructor: {}",
                        constructor_line(constructor, &lean)
                    ));
                }
                (type_name.clone(), lines)
            }
            ToolDeclaration::Call {
                name: call,
                signature,
                role,
                source,
            } => (
                call.clone(),
                vec![
                    format!("Signature: {}", lean.sig(signature)),
                    format!("Role: {role}"),
                    format!("Source: {} ({})", source.file, source.sha256),
                ],
            ),
            ToolDeclaration::Plumbing { names } => (
                "plumbing".to_owned(),
                vec![format!("Names: {}", names.join(", "))],
            ),
            ToolDeclaration::Function {
                name: function,
                public,
                partial,
                mutual,
                ..
            } => {
                let mut lines = vec![lean.declaration(declaration).trim_end().replace('\n', " ")];
                lines.insert(
                    0,
                    format!(
                        "Visibility: {}; recursion: {}{}",
                        if *public { "public" } else { "private" },
                        if *partial { "partial" } else { "total" },
                        mutual
                            .as_ref()
                            .map_or_else(String::new, |label| format!("; mutual group {label}"))
                    ),
                );
                (function.clone(), lines)
            }
            ToolDeclaration::Command { name: command, .. }
            | ToolDeclaration::Invoke { command, .. } => (
                command.clone(),
                vec![lean.declaration(declaration).trim_end().replace('\n', " ")],
            ),
            ToolDeclaration::Evaluate { function, .. } => (
                function.clone(),
                vec![lean.declaration(declaration).trim_end().to_owned()],
            ),
        };
        text.push_str(&format!(
            "\\subsection*{{\\texttt{{{}}}}}\n\\noindent Kind: \\texttt{{{}}}.\\par\n",
            tex_escape(&title),
            declaration.kind()
        ));
        for line in lines {
            text.push_str(&format!(
                "\\noindent \\texttt{{{}}}\\par\n",
                tex_escape(&line)
            ));
        }
    }
    let end = text.len();
    text.push_str("\\end{document}\n");
    emit_declarations_for(
        checked,
        &text,
        "tooling-latex-module",
        &starts,
        end,
        &SOURCE,
    )
}
