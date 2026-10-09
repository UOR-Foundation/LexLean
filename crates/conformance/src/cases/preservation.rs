//! Conformance cases for semantic preservation (SPEC.md §17.17).

use std::collections::BTreeSet;
use std::sync::OnceLock;

use lexlean::calculus::Outcome;
use lexlean::production::lower::{linked_modules, lower_root, roots};
use lexlean::production::preserve::{self, audit, audit_tokens, module_text};

use crate::preservation::{self, Mutation, Report};
use crate::support::{self, repo_root, P};

/// The projects whose every production root is certified: the example that
/// declares production roots, the coverage example, and the models example,
/// whose roots together exercise every runtime row of the production
/// registry.
fn certified_projects() -> [(&'static str, P); 3] {
    [
        ("production", P::copy_example("production")),
        (
            "production-coverage",
            P::copy_example("production-coverage"),
        ),
        ("models", P::copy_example("models")),
    ]
}

/// The roots whose closure declares an overflow (§17.13's effect rows
/// over-approximate) that no input of the differential reaches: none of
/// the seeded, medium, top, bottom, and 400 mixed inputs of
/// `differential::cases`. For the first group the operands that can overflow
/// do not depend on the root's parameters (`roseTotal`, `halvings`,
/// `wellFounded`, `natOps`) or the overflow is a sequence, text, or
/// collection of 2^64 elements, which no test can build (`sequences`,
/// `text`, `decimals`, `mapOps`, `setOps`, `graphs`, `stringGraph`); for the
/// model roots no input found overflows their arithmetic over the decoded
/// weights. The list is checked both ways: a root on it that overflows, or a
/// root off it that does not, fails SP-03.
const UNREACHED_OVERFLOW: [&str; 19] = [
    "Production.Main.halvings",
    "Coverage.Colls.mapOps",
    "Coverage.Colls.setOps",
    "Coverage.Colls.graphs",
    "Coverage.Colls.stringGraph",
    "Coverage.Main.natOps",
    "Coverage.Prims.sequences",
    "Coverage.Prims.decimals",
    "Coverage.Prims.text",
    "Coverage.RecRoots.roseTotal",
    "Coverage.RecRoots.syntaxTotal",
    "Coverage.RecRoots.wellFounded",
    "Models.Main.classify",
    "Models.Main.classifyChecked",
    "Models.Main.respond",
    "Models.Main.admitCosts",
    "Models.Main.ledgerPost",
    "Models.Main.ledgerFull",
    "Models.Main.guessChecked",
];

/// What the product reports of a rejected certificate: Lean's verdict on
/// the certificate, never a machine limit, because the certificate is small
/// against `max_file_bytes`. A planted wrong certificate whose proof makes
/// `isDefEq` run out of heartbeats (the overflow plant on
/// `Coverage.Prims.fixedMiddle`) is `LLV7013`, not `LLS8002`, which would
/// blame the project's types for a defect of the lowering.
fn assert_not_a_limit(what: &str, output: &str, text_len: usize) {
    static LIMITS: OnceLock<lexlean::config::Limits> = OnceLock::new();
    let limits = LIMITS.get_or_init(|| support::limits(&P::copy_example("production")));
    assert!(
        lexlean::verify::resource_death("planted", 1, output, text_len as u64, limits).is_none(),
        "{what}: a rejected certificate of {text_len} bytes is reported as an exhausted resource:\n{output}"
    );
}

/// The failure of a planted mutation is targeted: Lean's first error lies in
/// the relation of the function the mutation changed, or, for a function of
/// a library template, whose relation the certificate states where the
/// template is instantiated, in the relation of a caller; not in the library,
/// the environment, or an unrelated function.
fn assert_targeted(plant: &preservation::Planted) {
    assert_not_a_limit(
        &format!("{} {:?}", plant.root, plant.mutation),
        &plant.rejection,
        plant.text_len,
    );
    let declaration = plant.declaration.as_deref().unwrap_or_else(|| {
        panic!(
            "{} {:?}: no declaration:\n{}",
            plant.root, plant.mutation, plant.rejection
        )
    });
    assert!(
        !plant.expected.is_empty(),
        "{} {:?}: function {} ({:?}) has no relation to fail in",
        plant.root,
        plant.mutation,
        plant.function,
        plant.origin
    );
    assert!(
        plant.expected.iter().any(|name| name == declaration),
        "{} {:?} in function {} ({:?}): the error lies in `{declaration}`, not in {:?}:\n{}",
        plant.root,
        plant.mutation,
        plant.function,
        plant.origin,
        plant.expected,
        plant.rejection
    );
}

/// Certify every stress family, assert that the lower bound the lowering
/// refuses on is never above the largest certificate and the floor from
/// field reads never above certificate B, that every certificate stops at
/// the limit it is given, and that programs whose certificates fit the
/// default limits are not refused. Returns how far certificate B went
/// beyond A and E at most.
fn stress_estimates() -> u64 {
    let mut quadratic = 0_u64;
    // Valid programs whose certificates fit the default limits are not
    // refused: `certificates` panics on a refusal.
    for (family, project) in crate::stress::fitting() {
        let certified = preservation::certificates(&project);
        assert!(!certified.is_empty(), "{family} certifies");
        for entry in &certified {
            assert_bounds(&family, entry);
        }
    }
    // Programs that make different certificates large at once are not
    // refused when they fit either. For each mixed shape, the largest scaling
    // whose certificates are all under the default `max_file_bytes` is
    // searched with the limits lifted and then certified under the default
    // ones, so the program sits at the edge of what fits.
    let limit = support::limits(&crate::stress::wide_match_default(2)).max_file_bytes;
    for (index, shape) in crate::stress::mixed_shapes().into_iter().enumerate() {
        let largest = |numerator: usize| -> u64 {
            preservation::certificates(&crate::stress::mixed_lifted(shape.scaled(numerator)))
                .iter()
                .map(|entry| {
                    std::iter::once(&entry.certificate)
                        .chain(entry.renderings.iter().map(|(_, b)| b))
                        .chain(entry.composed.iter().map(|(_, e)| e))
                        .map(|certificate| certificate.text.len() as u64)
                        .max()
                        .unwrap_or(0)
                })
                .max()
                .unwrap_or(0)
        };
        let (mut low, mut high) = (1_usize, 256_usize);
        if largest(high) <= limit {
            low = high;
        } else {
            while high - low > 1 {
                let middle = (low + high) / 2;
                if largest(middle) <= limit {
                    low = middle;
                } else {
                    high = middle;
                }
            }
        }
        let size = largest(low);
        assert!(
            size <= limit && size * 10 >= limit * 6,
            "mixed shape {index} ({shape:?}): the largest scaling that fits has certificates of {size} bytes, which is not near the limit {limit}"
        );
        let certified =
            preservation::certificates(&crate::stress::mixed_default(shape.scaled(low)));
        for entry in &certified {
            assert_bounds(&format!("mixed shape {index} at {low}/256"), entry);
        }
    }
    // Random shapes, none of them used to choose the costs of the bound,
    // however they combine.
    for (index, shape) in crate::stress::random_shapes(24, 0x5eed)
        .into_iter()
        .enumerate()
    {
        for entry in &preservation::certificates(&crate::stress::mixed_lifted(shape)) {
            assert_bounds(&format!("random shape {index} {shape:?}"), entry);
        }
    }
    for (family, project) in crate::stress::families() {
        let certified = preservation::certificates(&project);
        // However large a program makes its certificates, generation stops
        // at the limit it is given: A and E regenerated under half their
        // size are refused with `LLS8002`.
        for (root, a, e) in preservation::halved(&project, &certified) {
            assert_eq!(
                a, "LLS8002",
                "{family}: {root}: certificate A under half its size"
            );
            assert!(
                e.iter().all(|code| code == "LLS8002"),
                "{family}: {root}: certificate E under half its size: {e:?}"
            );
        }
        for entry in certified {
            assert_floor(&family, &entry);
            assert_bounds(&family, &entry);
            let a = entry.certificate.text.len() as u64;
            let e = entry
                .composed
                .iter()
                .map(|(_, certificate)| certificate.text.len() as u64)
                .max()
                .unwrap_or(0);
            for (target, certificate) in &entry.renderings {
                let size = certificate.text.len() as u64;
                quadratic = quadratic.max(size.saturating_sub(a.max(e)));
                // Certificate B stops at the limit it is given, at
                // the size it would have had or below it.
                if family.starts_with("record copy 100") {
                    let krate = lexlean::calculus::rust::lower(
                        &entry.program,
                        lexlean::calculus::rust::Profile::named(target).expect("a Rust profile"),
                    )
                    .expect("a rendering");
                    let refused = lexlean::production::rust_cert::certificate_b(
                        &entry.program,
                        &krate,
                        &certificate.module,
                        size / 2,
                    )
                    .expect_err("a certificate beyond its limit is refused");
                    assert!(
                        refused.starts_with(lexlean::production::lower::LIMIT)
                            && refused.contains("the derivation of a function is at least"),
                        "{family}: certificate B was refused when finished, not while derived: {refused}"
                    );
                }
            }
        }
    }
    quadratic
}

/// The floor the lowering puts under certificate B from the entries of its
/// field reads is no more than any certificate B generated.
fn assert_floor(label: &str, entry: &preservation::Certified) {
    let floor = lexlean::production::lower::record_slots(&entry.program)
        .saturating_mul(lexlean::production::lower::CERTIFICATE_BYTES_PER_SLOT);
    for (target, certificate) in &entry.renderings {
        assert!(
            floor <= certificate.text.len() as u64,
            "{label} {} ({target}): the floor {floor} is above certificate B, {} bytes",
            entry.root,
            certificate.text.len()
        );
    }
}

/// The lower bound the lowering refuses on against `max_file_bytes` before it
/// generates anything is, for each certificate, no more than that certificate
/// (for B and E, no more than the smaller of the targets'), so that the
/// greatest of the three is never above the largest certificate.
fn assert_bounds(label: &str, entry: &preservation::Certified) {
    let bounds = lexlean::production::lower::certificate_lower_bounds(
        &lexlean::production::lower::measure_program(&entry.program),
    );
    let a = entry.certificate.text.len() as u64;
    let smallest = |texts: Vec<u64>| texts.into_iter().min().unwrap_or(u64::MAX);
    let b = smallest(
        entry
            .renderings
            .iter()
            .map(|(_, certificate)| certificate.text.len() as u64)
            .collect(),
    );
    let e = smallest(
        entry
            .composed
            .iter()
            .map(|(_, certificate)| certificate.text.len() as u64)
            .collect(),
    );
    assert!(
        bounds.a <= a && bounds.b <= b && bounds.e <= e,
        "{label}: {}: the lower bounds {bounds:?} are above certificate A ({a}), B ({b}), or E ({e})",
        entry.root
    );
    let (which, greatest) = bounds.greatest();
    assert!(
        greatest
            <= a.max(if b == u64::MAX { 0 } else { b })
                .max(if e == u64::MAX { 0 } else { e }),
        "{label}: {}: the greatest bound is on certificate {which}: {greatest}",
        entry.root
    );
}

/// The words of Lean's syntax and tactics, and nothing else, that a
/// certificate writes bare: the words that are not names of anything a
/// parameter could shadow. Every other bare word of a certificate is a
/// global or a definition, which `assert_no_bare_globals` refuses.
const SYNTAX_WORDS: &[&str] = &[
    "_",
    "all_goals",
    "at",
    "attribute",
    "autoImplicit",
    "by",
    "cases",
    "decreasing_by",
    "def",
    "else",
    "exact",
    "fun",
    "generalizing",
    "have",
    "if",
    "import",
    "in",
    "irreducible",
    "local",
    "match",
    "maxRecDepth",
    "namespace",
    "end",
    "of",
    "open",
    "rw",
    "set_option",
    "simp",
    "structural",
    "subst_vars",
    "termination_by",
    "then",
    "theorem",
    "unfold",
    "with",
    "let",
    "only",
    "first",
    "mutual",
];

/// A certificate refers to globals, and to the definitions it states, only by
/// their full names or by names beginning with two underscores, never by a
/// bare word that a parameter of the root could be named: a bare word of the
/// text is Lean syntax or tactic, the name of a type the backend emits bare
/// (which linking refuses as a binder), a named argument of a library
/// theorem, or one of the source's own names.
fn assert_no_bare_globals(what: &str, text: &str, user: &BTreeSet<String>, prefix_root: &str) {
    use lexlean::production::preserve::{lex, LexemeKind};
    let lexemes = lex(text).expect("a certificate lexes");
    let name = |at: usize| -> Option<String> {
        lexemes.get(at).and_then(|lexeme| match &lexeme.kind {
            LexemeKind::Ident(segments) => Some(
                segments
                    .iter()
                    .map(|segment| segment.text.as_str())
                    .collect::<Vec<_>>()
                    .join("."),
            ),
            LexemeKind::Number
            | LexemeKind::Literal
            | LexemeKind::Command(_)
            | LexemeKind::Symbol(_) => None,
        })
    };
    let symbol = |at: usize, wanted: char| {
        lexemes
            .get(at)
            .is_some_and(|lexeme| lexeme.kind == LexemeKind::Symbol(wanted))
    };
    // The first word on each line, to recognize `set_option` lines.
    let mut line_words: std::collections::BTreeMap<usize, String> = Default::default();
    for (at, lexeme) in lexemes.iter().enumerate() {
        if lexeme.first_on_line {
            if let Some(word) = name(at) {
                line_words.insert(lexeme.line, word);
            }
        }
    }
    let mut bare: BTreeSet<String> = BTreeSet::new();
    for (at, lexeme) in lexemes.iter().enumerate() {
        let LexemeKind::Ident(segments) = &lexeme.kind else {
            continue;
        };
        let first = segments[0].text.as_str();
        // The module a line imports, opens, or names is not a reference.
        let on_module_line = line_words
            .get(&lexeme.line)
            .is_some_and(|word| ["import", "open", "namespace", "end"].contains(&word.as_str()));
        // `.fixed` and `x.1` name a constructor or a field of what the
        // context says, never a global.
        let after_dot = at >= 1 && symbol(at - 1, '.') && lexemes[at - 1].start + 1 == lexeme.start;
        let option = line_words
            .get(&lexeme.line)
            .is_some_and(|word| word == "set_option");
        if first.starts_with("__") || on_module_line || after_dot || option {
            continue;
        }
        // A name of the source that begins a longer name is a local with a
        // projection (`x.1`) or a reference into the module prefix. It is
        // not a global that a local of that name has captured: those begin
        // with a name of the library, which is no component of the source's
        // own names, so a longer name whose second component is neither a
        // number nor a name of the source is refused even when its first is
        // one (a parameter named `LexLeanPreservation` makes
        // `LexLeanPreservation.conv_var` a field of itself).
        if user.contains(first) {
            if segments.len() > 1
                && first != prefix_root
                && !segments[1].text.chars().all(|c| c.is_ascii_digit())
                && !user.contains(segments[1].text.as_str())
            {
                bare.insert(format!(
                    "{} (line {})",
                    name(at).unwrap_or_default(),
                    lexeme.line
                ));
            }
            continue;
        }
        // A name reaching into a namespace begins from the root, or in a
        // namespace of the backend that linking keeps a binder from taking.
        if segments.len() > 1 {
            if first != "_root_" && !lexlean::ir::semantic::is_backend_bare_name(first) {
                bare.insert(format!(
                    "{} (line {})",
                    name(at).unwrap_or_default(),
                    lexeme.line
                ));
            }
            continue;
        }
        let previous = at.checked_sub(1);
        let declared = previous.and_then(&name).is_some_and(|before| {
            before == "def" || before == "theorem" || before == "generalizing"
        });
        let pattern = previous.is_some_and(|before| symbol(before, '|'));
        // `(ek := …)`, a named argument, and the option of `generalizing := false`.
        let named = symbol(at + 1, ':') && symbol(at + 2, '=');
        let after_assignment = at >= 2 && symbol(at - 1, '=') && symbol(at - 2, ':');
        if declared
            || pattern
            || named
            || (after_assignment
                && at >= 3
                && name(at - 3).is_some_and(|word| word == "generalizing"))
            || SYNTAX_WORDS.contains(&first)
            || lexlean::ir::semantic::is_backend_bare_name(first)
        {
            continue;
        }
        bare.insert(format!("{first} (line {})", lexeme.line));
    }
    assert!(
        bare.is_empty(),
        "{what}: names a parameter could shadow: {bare:?}"
    );
}

/// The namespaces a certificate begins names with: the first component of
/// every longer name, after `_root_`, that is neither a local, a name of the
/// source, nor a name the backend keeps a binder from taking. A parameter
/// spelled like one would capture the names that begin with it unless the
/// certificate writes them from the root.
fn referenced_namespaces(text: &str, user: &BTreeSet<String>) -> BTreeSet<String> {
    use lexlean::production::preserve::{lex, LexemeKind};
    let lexemes = lex(text).expect("a certificate lexes");
    let mut words: std::collections::BTreeMap<usize, String> = Default::default();
    for lexeme in &lexemes {
        if lexeme.first_on_line {
            if let LexemeKind::Ident(segments) = &lexeme.kind {
                words.insert(lexeme.line, segments[0].text.clone());
            }
        }
    }
    let chars: Vec<char> = text.chars().collect();
    let mut out = BTreeSet::new();
    for lexeme in &lexemes {
        let LexemeKind::Ident(segments) = &lexeme.kind else {
            continue;
        };
        let skipped = words.get(&lexeme.line).is_some_and(|word| {
            ["import", "open", "namespace", "end", "set_option"].contains(&word.as_str())
        });
        let after_dot = lexeme.start > 0 && chars[lexeme.start - 1] == '.';
        if skipped || after_dot || segments.len() < 2 {
            continue;
        }
        let first = if segments[0].text == "_root_" {
            segments[1].text.as_str()
        } else {
            segments[0].text.as_str()
        };
        if first.starts_with("__")
            || user.contains(first)
            || lexlean::ir::semantic::is_backend_bare_name(first)
        {
            continue;
        }
        out.insert(first.to_owned());
    }
    out
}

/// The first component of a project's module prefix: the root of every
/// reference to one of its declarations, which linking keeps a binder from
/// being named.
fn prefix_root(project: &P) -> String {
    project
        .read("lexlean.toml")
        .lines()
        .find_map(|line| {
            line.strip_prefix("module_prefix = \"")
                .and_then(|rest| rest.strip_suffix('"'))
        })
        .and_then(|prefix| prefix.split('.').next())
        .expect("a module prefix")
        .to_owned()
}

/// The strings of a project's sources that are not keys: the names a user
/// chose.
fn user_names(project: &P) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    // The module prefix is a name of the source: the root of its modules.
    for line in project.read("lexlean.toml").lines() {
        if let Some(prefix) = line
            .strip_prefix("module_prefix = \"")
            .and_then(|rest| rest.strip_suffix('"'))
        {
            out.insert(prefix.to_owned());
        }
    }
    for entry in walkdir::WalkDir::new(project.root.join("src").as_std_path())
        .into_iter()
        .flatten()
    {
        if !entry.file_type().is_file() {
            continue;
        }
        let text = std::fs::read_to_string(entry.path()).expect("a source");
        let parts: Vec<&str> = text.split('"').collect();
        // Odd parts are the strings of the JSON; one followed by a colon is a
        // key.
        for (at, part) in parts.iter().enumerate().skip(1).step_by(2) {
            let key = parts.get(at + 1).is_some_and(|next| next.starts_with(':'));
            // The value of a `kind` or an `operation` is the source's
            // vocabulary, not a name a user chose.
            let vocabulary = parts.get(at - 1).is_some_and(|before| {
                before.ends_with(':')
                    && (parts
                        .get(at - 2)
                        .is_some_and(|k| *k == "kind" || *k == "operation" || *k == "spec"))
            });
            if !key && !vocabulary {
                out.insert((*part).to_owned());
            }
        }
    }
    out
}

/// The parenthesized groups of `text`, which must be nothing else but groups
/// separated by single spaces.
fn groups(text: &str) -> Option<Vec<&str>> {
    let mut out = Vec::new();
    let mut depth = 0_usize;
    let mut start = 0;
    for (at, character) in text.char_indices() {
        match character {
            '(' => {
                if depth == 0 {
                    start = at;
                }
                depth += 1;
            }
            ')' => {
                depth = depth.checked_sub(1)?;
                if depth == 0 {
                    out.push(&text[start..=at]);
                }
            }
            ' ' if depth == 0 => {}
            _ if depth == 0 => return None,
            _ => {}
        }
    }
    (depth == 0 && out.join(" ") == text).then_some(out)
}

/// The names bound by the lambdas, existentials, and patterns of a
/// composition's text after its parameters: `fun a b =>`, `∃ a,`, and
/// `⟨a, b⟩ =>`.
fn generated_binders(text: &str) -> Vec<&str> {
    let mut out = Vec::new();
    let mut take = |from: &'static str, to: &'static str, split: &'static [char]| {
        for (at, _) in text.match_indices(from) {
            let tail = &text[at + from.len()..];
            if let Some(end) = tail.find(to) {
                out.extend(
                    tail[..end]
                        .split(|c: char| c == ' ' || split.contains(&c))
                        .filter(|word| !word.is_empty()),
                );
            }
        }
    };
    take("fun ", " =>", &[]);
    take("∃ ", ",", &[]);
    take("| ⟨", "⟩ =>", &[',']);
    out
}

/// The statement of certificate E, in full. The binders are exactly the
/// root's parameters, then one hypothesis, that the encodings of the
/// arguments are representable; the conclusion is the one §17.17 states for
/// a root with an entry or without, built here from the root's parameters
/// and the function its rendering is invoked through. A statement with
/// another hypothesis, a conjunct, or another conclusion is not equal to it,
/// which a prefix match would not notice.
fn assert_statement(
    name: &str,
    entry: &preservation::Certified,
    rendering: &lexlean::production::certificate::Certificate,
    composed: &lexlean::production::certificate::Certificate,
) {
    let at = composed
        .text
        .find("theorem root ")
        .unwrap_or_else(|| panic!("{name}: `{}` states no root theorem", composed.module));
    assert_eq!(
        composed.text.matches("theorem root ").count(),
        1,
        "{name}: `{}` states one root theorem",
        composed.module
    );
    let rest = &composed.text[at + "theorem root ".len()..];
    let (head, _) = rest
        .split_once(" : ∃ __e_ro, ")
        .unwrap_or_else(|| panic!("{name}: `{}`: no conclusion", composed.module));
    let binders = groups(head).unwrap_or_else(|| {
        panic!(
            "{name}: `{}`: binders are not groups: {head}",
            composed.module
        )
    });
    let parameters = entry.program.functions[0].parameters.len();
    assert_eq!(
        binders.len(),
        parameters + 1,
        "{name}: `{}` binds the {parameters} parameters and one hypothesis: {head}",
        composed.module
    );
    let (hypothesis, own) = binders.split_last().expect("a hypothesis");
    let names: Vec<&str> = own
        .iter()
        .map(|binder| {
            binder
                .trim_start_matches('(')
                .split(" : ")
                .next()
                .expect("a name")
        })
        .collect();
    let list = hypothesis
        .strip_prefix("(__e_hrep : _root_.LexLeanPreservation.Rust.RepresentableL [")
        .and_then(|rest| rest.strip_suffix("])"))
        .unwrap_or_else(|| panic!("{name}: `{}`: hypothesis {hypothesis}", composed.module));
    // The encodings, one for each parameter, each applied to its name.
    let encodings = {
        let mut out = Vec::new();
        let (mut depth, mut start) = (0_usize, 0);
        for (position, character) in list.char_indices() {
            match character {
                '(' | '[' => depth += 1,
                ')' | ']' => depth -= 1,
                ',' if depth == 0 => {
                    out.push(&list[start..position]);
                    start = position + 1;
                }
                _ => {}
            }
        }
        out.push(&list[start..]);
        out.into_iter().map(str::trim).collect::<Vec<_>>()
    };
    assert_eq!(encodings.len(), parameters, "{name}: `{}`", composed.module);
    for (encoding, local) in encodings.iter().zip(&names) {
        assert!(
            encoding.starts_with('(') && encoding.ends_with(&format!(" {local})")),
            "{name}: `{}`: `{encoding}` does not encode `{local}`",
            composed.module
        );
    }
    let fallible = if lexlean::calculus::rust::fallible_functions(&entry.program)
        .expect("a valid program")[entry.entry as usize]
    {
        "Bool.true"
    } else {
        "Bool.false"
    };
    let applied: String = names.iter().map(|local| format!(" {local}")).collect();
    let (a, b) = (&entry.certificate.module, &rendering.module);
    let encoded = encodings.join(", ");
    let lib = "_root_.LexLeanPreservation";
    let conclusion = if entry.entry == 0 {
        format!(
            "∃ __e_ro, {lib}.Rust.RealizesFn {fallible} (_root_.{a}.denote{applied}) __e_ro ∧ {lib}.Rust.RCI _root_.{b}.krate ({lib}.Rust.fnIdent 0) [{encoded}] __e_ro"
        )
    } else {
        format!(
            "∃ __e_ro, {lib}.Rust.RCI _root_.{b}.krate ({lib}.Rust.fnIdent {}) [{encoded}] __e_ro ∧\n    (_root_.{a}.accepts{applied} → {lib}.Rust.RealizesFn {fallible} ({lib}.someObs (_root_.{a}.denote{applied})) __e_ro) ∧\n    (¬ _root_.{a}.accepts{applied} → {lib}.Rust.RealizesFn {fallible} ({lib}.Obs.value _root_.LexLeanTarget.TargetSyntax.Value.none) __e_ro)",
            entry.entry
        )
    };
    // No name the generator binds can be a name the source spells: every
    // binder of the composition other than the root's own parameters begins
    // with two underscores, which no semantic name does.
    for binder in generated_binders(rest) {
        assert!(
            binder == "_" || binder.starts_with("__"),
            "{name}: `{}` binds `{binder}`, which a parameter of that name would capture",
            composed.module
        );
    }
    let statement = format!("{head} : {conclusion} :=\n");
    assert!(
        rest.starts_with(&statement),
        "{name}: `{}` does not state exactly the end-to-end theorem:\n{statement}\nbut\n{}",
        composed.module,
        rest.lines().take(4).collect::<Vec<_>>().join("\n")
    );
}

/// Certifying both projects takes minutes; the cases share one run.
fn reports() -> &'static Vec<(&'static str, Report)> {
    static REPORTS: OnceLock<Vec<(&'static str, Report)>> = OnceLock::new();
    REPORTS.get_or_init(|| {
        certified_projects()
            .into_iter()
            .map(|(name, project)| (name, preservation::certify(&project, name)))
            .collect()
    })
}

/// Every language-1.2 example.
fn lowered_projects() -> Vec<(String, P)> {
    let mut out = Vec::new();
    let mut names: Vec<String> = std::fs::read_dir(repo_root().join("examples").as_std_path())
        .expect("examples")
        .map(|entry| {
            entry
                .expect("entry")
                .file_name()
                .to_string_lossy()
                .into_owned()
        })
        .collect();
    names.sort();
    for name in names {
        let config = repo_root()
            .join("examples")
            .join(&name)
            .join("lexlean.toml");
        let text = std::fs::read_to_string(config.as_std_path()).unwrap_or_default();
        if text.contains("language = \"1.2\"") {
            out.push((name.clone(), P::copy_example(&name)));
        }
    }
    out
}

/// The runtime rows of the production registry that no root of `roots`
/// exercises: a row counts where a root's report realizes its construct,
/// and `type.parameter` where a root instantiates a generic definition.
fn unexercised(reports: &[&lexlean::production::RootReport]) -> Vec<String> {
    let mut seen: BTreeSet<String> = BTreeSet::new();
    for report in reports {
        seen.extend(report.constructs.keys().cloned());
        if report
            .runtime
            .iter()
            .any(|member| !member.type_arguments.is_empty())
        {
            seen.insert("type.parameter".to_owned());
        }
    }
    let text = std::fs::read_to_string(
        repo_root()
            .join(lexlean::production::REGISTRY_PATH)
            .as_std_path(),
    )
    .expect("the production registry");
    let registry: toml::Value = toml::from_str(&text).expect("the registry parses");
    registry["construct"]
        .as_array()
        .expect("construct rows")
        .iter()
        .filter(|row| row["disposition"].as_str() == Some("runtime"))
        .filter_map(|row| row["key"].as_str())
        .filter(|key| !seen.contains(*key))
        .map(str::to_owned)
        .collect()
}

/// Run one preservation case.
///
/// # Panics
///
/// Panics when the case's claim does not hold.
#[allow(clippy::too_many_lines)]
pub fn run(id: &str) {
    match id {
        // §17.17: the lowering.
        "SP-01" => {
            let mut lowered = 0usize;
            for (name, project) in lowered_projects() {
                let checked = support::checked_project(&project);
                let modules = linked_modules(&checked);
                let limits = support::limits(&project);
                for root in roots(&checked).expect("the eligibility reports") {
                    let first =
                        lower_root(&modules, &root.module, &root.name, root.report, &limits)
                            .unwrap_or_else(|d| panic!("{name}: {}: {d:?}", root.report.root));
                    let second =
                        lower_root(&modules, &root.module, &root.name, root.report, &limits)
                            .expect("lowers again");
                    assert_eq!(
                        first.program.to_file_bytes(),
                        second.program.to_file_bytes(),
                        "{}: two lowerings differ",
                        root.report.root
                    );
                    assert_eq!(
                        first.program.canonical().expect("a valid program"),
                        first.program,
                        "{}: not in first-binding order",
                        root.report.root
                    );
                    assert_eq!(
                        first.layout.functions.len(),
                        first.program.functions.len(),
                        "{}: a function without an origin",
                        root.report.root
                    );
                    assert_eq!(
                        first.layout.adts.len(),
                        first.program.adts.len(),
                        "{}: an ADT without an origin",
                        root.report.root
                    );
                    // A report whose closure omits a member disagrees with
                    // what the lowering reaches.
                    let mut planted = root.report.clone();
                    if planted.runtime.pop().is_some() {
                        let error =
                            lower_root(&modules, &root.module, &root.name, &planted, &limits)
                                .expect_err("a closure disagreement is refused");
                        assert_eq!(error.code.as_str(), "LLI9001", "{error:?}");
                    }
                    lowered += 1;
                }
            }
            assert!(lowered > 0, "the examples declare production roots");
            let semantic = std::fs::read_to_string(
                repo_root()
                    .join(repo_model::exhaustive::SEMANTIC_SOURCE)
                    .as_std_path(),
            )
            .expect("the IR source");
            for path in repo_model::exhaustive::PRESERVATION_SOURCES {
                let text =
                    std::fs::read_to_string(repo_root().join(path).as_std_path()).expect(path);
                repo_model::exhaustive::audit_preservation(path, &text, &semantic)
                    .unwrap_or_else(|report| panic!("{report}"));
                for planted in [
                    "\nfn planted(t: Option<u8>) { if let Some(_) = t {} }\n",
                    "\nfn planted(t: &SemanticType) -> bool { matches!(t, SemanticType::Nat) }\n",
                    "\nfn planted(t: &SemanticType) { match t { SemanticType::Nat => {} _ => {} } }\n",
                ] {
                    let error = repo_model::exhaustive::audit_preservation(
                        path,
                        &format!("{text}{planted}"),
                        &semantic,
                    )
                    .expect_err("a planted default is refused");
                    assert!(error.contains(path), "{error}");
                }
            }
        }
        // §17.17: certificate A.
        "SP-02" => {
            for (name, project) in certified_projects() {
                let certified = preservation::certificates(&project);
                assert!(!certified.is_empty(), "{name} declares production roots");
                for entry in &certified {
                    assert!(
                        !entry.targets.is_empty(),
                        "{}: eligible somewhere",
                        entry.root
                    );
                    assert!(
                        entry.certificate.text.contains("theorem root"),
                        "{}: the certificate states the root theorem",
                        entry.root
                    );
                    // On the corpora the bound is below the largest
                    // certificate.
                    assert_floor(name, entry);
                    assert_bounds(name, entry);
                }
            }
            // The lower bound holds over the stress families too: programs
            // that grow one dimension at a time (let chains, call chains,
            // many parameters, enumerations, nested matches, wide
            // structures, record copies, literals, a generic chain).
            // Certificate B is not bounded from the counts, because its
            // derivations repeat the patterns around a node and grow with
            // the square of a record's arity: it is bounded by the limit it
            // is generated under, which the family of record copies reaches.
            // Certifying a family's deepest program recurses deeply, as the
            // differential's interpreter does.
            let quadratic = std::thread::Builder::new()
                .stack_size(512 << 20)
                .spawn(stress_estimates)
                .expect("a thread")
                .join()
                .expect("the stress families certify");
            assert!(
                quadratic > 100_000,
                "the record-copy family has a certificate B far beyond A and E, which is what B's own limit is for"
            );
            // Two negative fixtures reach the limits of certificate B before
            // the toolchain. A record of 800 fields copied field by field
            // has 640 000 record entries in its field reads, which the
            // lowering refuses by their floor on B; a sum of the 500 fields
            // of a record has 250 000, below that floor and above the limit
            // in its derivation, which stops when it passes the limit.
            // The fixture `certificate-resource-exhausted` is the project of
            // `certificate-heartbeat-rejected` with a limit that holds its
            // largest certificate, so that generation passes, and that its
            // module is a quarter of or more: a change of a certificate that
            // took the limit out of between is reported here, not as a
            // changed hash.
            let between = P::negative("certificate-resource-exhausted");
            let limit = support::limits(&between).max_file_bytes;
            for entry in preservation::certificates(&between) {
                let module = entry.certificate.text.len() as u64;
                let largest = std::iter::once(&entry.certificate)
                    .chain(entry.renderings.iter().map(|(_, b)| b))
                    .chain(entry.composed.iter().map(|(_, e)| e))
                    .map(|certificate| certificate.text.len() as u64)
                    .max()
                    .unwrap_or(0);
                assert!(
                    largest <= limit && limit <= module.saturating_mul(4),
                    "certificate-resource-exhausted: {}: its limit {limit} must lie between its largest certificate, {largest}, and four times its module, {}",
                    entry.root,
                    module * 4
                );
            }
            for fixture in [
                "certificate-size-limit",
                "certificate-generation-limit",
                "certificates-total-limit",
            ] {
                let case =
                    crate::fixtures::load_case(&repo_root().join("tests/negative").join(fixture))
                        .expect("the fixture loads");
                let observed = crate::fixtures::observe(&case).expect("the fixture runs");
                assert_eq!(observed.codes, ["LLS8002"], "{fixture}");
            }
            // A root whose lowered program, or the certificates it implies,
            // would exceed the project's limits is refused before anything is
            // generated from it: a generic chain `g_k<T> = g_{k+1}<(T, T)>`
            // 16 deep has a type of 2^16 nodes (its certificates are
            // at least twice the limit), and `lexlean verify`
            // refuses it with `LLS8002` without starting Lean.
            let case =
                crate::fixtures::load_case(&repo_root().join("tests/negative/lowering-size-limit"))
                    .expect("the fixture loads");
            let observed = crate::fixtures::observe(&case).expect("the fixture runs");
            assert_eq!(observed.codes, ["LLS8002"]);
            // The pinned Lean's heartbeat budgets are lifted by the
            // certificates A and B of a program whose widest match has more
            // than 100 arms, and by no other: the budgets of a wrong
            // certificate of a small program stay what they are, and a wrong
            // certificate of a wide match ends at the wall clock.
            for (arms, lifted) in [(100, false), (101, true)] {
                for entry in preservation::certificates(&crate::stress::wide_match_default(arms)) {
                    let texts = std::iter::once(&entry.certificate)
                        .chain(entry.renderings.iter().map(|(_, b)| b));
                    for certificate in texts {
                        for option in [
                            "set_option maxHeartbeats 0\n",
                            "set_option synthInstance.maxHeartbeats 0\n",
                        ] {
                            assert_eq!(
                                certificate.text.contains(option),
                                lifted,
                                "{arms} arms: `{}` and `{option}`",
                                certificate.module
                            );
                        }
                    }
                }
            }
            // A semantic module is JSON, and the JSON of the front end nests
            // at most 128 levels: a chain of 100 `let`s is read (a family
            // above certifies it) and a chain of 130 is refused, as the
            // module that is not valid JSON of this reader, before any limit
            // of this section.
            let deep = crate::stress::let_chain_default(130);
            let error = deep.check_fails_with("LLT4001");
            assert!(
                error.to_string().contains("recursion limit exceeded"),
                "{error}"
            );
            if !support::lean_backed("SP-02") {
                return;
            }
            // A match on 180 constructors verifies through the whole pipeline
            // (`lexlean verify`'s own path): the extraction record of its
            // named root nests past 128 levels of JSON, certificate B is
            // beyond the pinned Lean's default `synthInstance.maxHeartbeats`
            // from 160 arms and `maxHeartbeats` from 200, and the certificates
            // of a match above 100 arms lift both.
            support::verify_ok(&crate::stress::wide_match_default(180));
            for (name, report) in reports() {
                assert!(!report.certified.is_empty(), "{name}: certified roots");
            }
            let planted =
                preservation::plant(&P::copy_example("production-coverage"), &Mutation::LOWERING);
            let kinds: BTreeSet<Mutation> = planted.iter().map(|plant| plant.mutation).collect();
            assert_eq!(
                kinds,
                Mutation::LOWERING.into_iter().collect(),
                "every mutation is planted somewhere in the coverage example"
            );
            for plant in &planted {
                assert!(
                    !plant.rejection.is_empty(),
                    "{}: the certificate of a program with a {:?} mutation was accepted",
                    plant.root,
                    plant.mutation
                );
                assert_targeted(plant);
            }
        }
        // §17.17: the differential evaluator.
        "SP-03" => {
            let (mut all, mut declared, mut reached) = (0_usize, 0_usize, 0_usize);
            for (name, project) in certified_projects() {
                let cases = crate::differential::cases(&project);
                assert!(
                    cases.values().map(Vec::len).sum::<usize>() > 0,
                    "{name}: seeded cases"
                );
                for case in cases.values().flatten() {
                    assert!(
                        !matches!(case.outcome, Outcome::Exhausted | Outcome::Stuck),
                        "{}: the interpreter stopped on {:?}",
                        case.root,
                        case.arguments
                    );
                }
                // The overflow arm of the statements is exercised: every
                // root whose closure may overflow is run on an input where
                // it does, on the interpreter here and on Lean's `denote`,
                // the declared machine, and rustc in SP-03, SP-07, SP-11,
                // except the roots below, whose declared overflow no input
                // reaches.
                let checked = support::checked_project(&project);
                for root in roots(&checked).expect("the eligibility reports") {
                    all += 1;
                    if !root
                        .report
                        .declared_effects
                        .iter()
                        .any(|effect| effect == "overflow")
                    {
                        continue;
                    }
                    declared += 1;
                    let overflows = cases.get(&root.report.root).is_some_and(|cases| {
                        cases
                            .iter()
                            .any(|case| matches!(case.outcome, Outcome::Overflow { .. }))
                    });
                    reached += usize::from(overflows);
                    let exempt = UNREACHED_OVERFLOW.contains(&root.report.root.as_str());
                    assert!(
                        overflows != exempt,
                        "{name}: `{}` {} (exempt: {exempt})",
                        root.report.root,
                        if overflows {
                            "overflows on a sampled input but is listed as unreached"
                        } else {
                            "has no input on which it overflows"
                        }
                    );
                }
            }
            // The specification states which certified roots rest on the
            // kernel proof alone for the overflow arm, and how many do.
            let spec = std::fs::read_to_string(repo_root().join("SPEC.md").as_std_path())
                .expect("SPEC.md");
            let listed: Vec<&str> = spec
                .split("```unreached-overflow\n")
                .nth(1)
                .and_then(|rest| rest.split("```").next())
                .expect("the specification lists the roots whose overflow no input reaches")
                .lines()
                .collect();
            assert_eq!(
                listed, UNREACHED_OVERFLOW,
                "SPEC.md lists exactly the roots the suite exempts"
            );
            assert_eq!(declared - reached, UNREACHED_OVERFLOW.len());
            let flat = spec.split_whitespace().collect::<Vec<_>>().join(" ");
            assert!(
                flat.contains(&format!(
                    "Of the {all} certified roots of the three examples, {declared} declare an overflow, {reached} of them reach it"
                )),
                "SPEC.md states that of {all} certified roots {declared} declare an overflow and {reached} reach it"
            );
            if !support::lean_backed("SP-03") {
                return;
            }
            for (name, report) in reports() {
                assert!(report.differential > 0, "{name}: cases compared");
                // One altered outcome is a disagreement.
                let mut planted = report.denotes.clone();
                let root = {
                    let case = planted
                        .iter_mut()
                        .flat_map(|(_, _, cases)| cases.iter_mut())
                        .next()
                        .expect("a case");
                    case.outcome = match case.outcome {
                        Outcome::Overflow { steps } => Outcome::Value {
                            value: lexlean::calculus::Value::Unit,
                            steps,
                        },
                        _ => Outcome::Overflow { steps: 0 },
                    };
                    case.root.clone()
                };
                let failures = crate::differential::compare(&report.differential_output, &planted)
                    .expect_err("a planted disagreement is detected");
                assert!(failures.contains(&root), "{failures}");
            }
        }
        // §17.17: the library and the shipped calculus modules.
        "SP-04" => {
            let library = preserve::library().expect("the shipped registry");
            assert!(
                !library.declaration.is_empty(),
                "the registry lists declarations"
            );
            crate::calculus::shipped_modules(repo_root().as_std_path(), false)
                .unwrap_or_else(|reason| panic!("{reason}"));
            // Every declaration the library states is registered: a theorem
            // that is not listed is not audited.
            for module in &library.modules {
                let text = module_text(module).expect("a shipped module");
                let stated: BTreeSet<String> = preservation::library_declarations(text)
                    .into_iter()
                    .map(|name| format!("{module}\u{0}{name}"))
                    .collect();
                let listed: BTreeSet<String> = library
                    .declaration
                    .iter()
                    .filter(|row| &row.module == module)
                    .map(|row| format!("{module}\u{0}{}", row.name))
                    .collect();
                assert_eq!(
                    stated.difference(&listed).collect::<Vec<_>>(),
                    Vec::<&String>::new(),
                    "{module}: declarations the registry does not list"
                );
                assert_eq!(
                    listed.difference(&stated).collect::<Vec<_>>(),
                    Vec::<&String>::new(),
                    "{module}: registry rows with no declaration"
                );
            }
            // The vocabulary SPEC.md quotes equals the library's.
            let spec = std::fs::read_to_string(repo_root().join("SPEC.md").as_std_path())
                .expect("SPEC.md");
            let quoted = repo_model::vocabulary::audit(&spec, &|file| {
                std::fs::read_to_string(
                    repo_root()
                        .join(repo_model::vocabulary::LIBRARY_DIR)
                        .join(file)
                        .as_std_path(),
                )
                .ok()
            })
            .unwrap_or_else(|report| panic!("{report}"));
            assert!(quoted >= 8, "{quoted} quoted declarations");
            let mut environment: BTreeSet<String> = preserve::TARGET_MODULES
                .iter()
                .map(|module| (*module).to_owned())
                .collect();
            for module in &library.modules {
                let text = module_text(module).expect("a shipped module");
                audit_tokens(text, &environment)
                    .unwrap_or_else(|reason| panic!("{module}: {reason}"));
                for planted in [
                    "\ntheorem planted : False := sorry\n",
                    "\naxiom planted : False\n",
                    "\ntheorem planted : 1 = 1 := by native_decide\n",
                    "\nset_option debug.skipKernelTC true\n",
                    "\nimport Mathlib\n",
                    "\ntheorem planted : True := Lean.ofReduceBool _ _ rfl\n",
                ] {
                    audit_tokens(&format!("{text}{planted}"), &environment)
                        .expect_err("a planted escape is refused");
                }
                environment.insert(module.clone());
            }
            if !support::lean_backed("SP-04") {
                return;
            }
            for (name, report) in reports() {
                let certificates: Vec<_> = report
                    .certified
                    .iter()
                    .map(|entry| entry.certificate.clone())
                    .collect();
                audit(&report.audit_output, &certificates)
                    .unwrap_or_else(|reason| panic!("{name}: {reason}"));
                // A declaration whose axioms drift from the registry.
                let first = &library.declaration[0];
                let drifted = report
                    .audit_output
                    .replace(
                        &format!("'{}' does not depend on any axioms", first.name),
                        &format!("'{}' depends on axioms: [sorryAx]", first.name),
                    )
                    .replace(
                        &format!("'{}' depends on axioms: [", first.name),
                        &format!("'{}' depends on axioms: [sorryAx, ", first.name),
                    );
                audit(&drifted, &certificates).expect_err("a drifted declaration is refused");
            }
        }
        // §17.17, §22.8, §22.9: certificate A in verification.
        "SP-05" => {
            // The record states, for each root and target, the function a
            // caller of the crate invokes. It is the function certificate E
            // states of the crate and the crate defines, and it is the
            // boundary entry exactly when E has one.
            let mut entered = 0;
            for example in ["production", "production-coverage", "models"] {
                let base = repo_root()
                    .join("examples")
                    .join(example)
                    .join("expected/verify/preserve");
                let record: serde_json::Value = serde_json::from_slice(
                    &std::fs::read(base.join("preservation.json").as_std_path())
                        .expect("a committed record"),
                )
                .expect("the record is JSON");
                for (index, row) in record["roots"]
                    .as_array()
                    .expect("roots")
                    .iter()
                    .enumerate()
                {
                    for rendering in row["renderings"].as_array().expect("renderings") {
                        let target = rendering["target"].as_str().expect("a target");
                        let function = rendering["entry"]["function"].as_u64().expect("a function");
                        assert_eq!(
                            rendering["entry"]["symbol"].as_str(),
                            Some(format!("f{function}").as_str())
                        );
                        let composed = std::fs::read_to_string(
                            base.join(lexlean::production::preserve::module_path(
                                rendering["composed"]["module"].as_str().expect("a module"),
                            ))
                            .as_std_path(),
                        )
                        .expect("the published certificate E");
                        let invoked: BTreeSet<u64> = composed
                            .match_indices("Rust.fnIdent ")
                            .map(|(at, found)| {
                                composed[at + found.len()..]
                                    .chars()
                                    .take_while(char::is_ascii_digit)
                                    .collect::<String>()
                                    .parse()
                                    .expect("a function number")
                            })
                            .collect();
                        assert_eq!(
                            invoked,
                            BTreeSet::from([function]),
                            "{example} R{index} {target}: E states the function the record names"
                        );
                        assert_eq!(
                            composed.contains("entry_accepts"),
                            function != 0,
                            "{example} R{index} {target}: E has an entry exactly when the record names one"
                        );
                        let krate = std::fs::read_to_string(
                            base.join(format!("crate/R{index}.{target}.rs"))
                                .as_std_path(),
                        )
                        .expect("the published crate");
                        assert!(
                            krate.contains(&format!("fn f{function}(")),
                            "{example} R{index} {target}: the crate defines f{function}"
                        );
                        entered += usize::from(function != 0);
                    }
                }
            }
            assert!(entered > 0, "some certified root has a boundary entry");
            if !support::lean_backed("SP-05") {
                return;
            }
            let project = P::copy_example("production");
            let verified = support::verify_ok(&project);
            let root = verified.root.as_std_path();
            let record_bytes =
                std::fs::read(root.join("preserve/preservation.json")).expect("preservation.json");
            let record: serde_json::Value =
                serde_json::from_slice(&record_bytes).expect("preservation.json is JSON");
            support::assert_schema("preservation", "preservation.json", &record);
            let rows = record["roots"].as_array().expect("roots");
            let checked = support::checked_project(&project);
            assert_eq!(
                rows.len(),
                roots(&checked).expect("the eligibility reports").len(),
                "one certificate per production root"
            );
            for row in rows {
                let module = row["module"].as_str().expect("a module");
                let text = std::fs::read(
                    root.join("preserve")
                        .join(lexlean::production::preserve::module_path(module)),
                )
                .expect("the published certificate");
                assert_eq!(
                    row["sha256"].as_str(),
                    Some(
                        lexlean::artifact::content_id::Sha256Digest::of(&text)
                            .to_hex()
                            .as_str()
                    ),
                    "{module}: the record binds the published certificate"
                );
                // The program the certificates are about and each target's
                // crate are published and bound too.
                let digest = |path: &std::path::Path| {
                    lexlean::artifact::content_id::Sha256Digest::of(
                        &std::fs::read(path).expect("a published file"),
                    )
                    .to_hex()
                };
                let index = rows
                    .iter()
                    .position(|candidate| candidate == row)
                    .expect("the row");
                assert_eq!(
                    row["program"]["sha256"].as_str(),
                    Some(digest(&root.join(format!("preserve/program/R{index}.json"))).as_str()),
                    "{module}: the record binds the published program"
                );
                for rendering in row["renderings"].as_array().expect("renderings") {
                    let target = rendering["target"].as_str().expect("a target");
                    assert_eq!(
                        rendering["crate"]["sha256"].as_str(),
                        Some(
                            digest(&root.join(format!("preserve/crate/R{index}.{target}.rs")))
                                .as_str()
                        ),
                        "{module}: the record binds the published crate in {target}"
                    );
                }
                // Certificates B and E of each target are published and bound
                // the same way.
                for rendering in row["renderings"].as_array().expect("renderings") {
                    for bound in [rendering, &rendering["composed"]] {
                        let module = bound["module"].as_str().expect("a module");
                        let text = std::fs::read(
                            root.join("preserve")
                                .join(lexlean::production::preserve::module_path(module)),
                        )
                        .expect("the published certificate");
                        assert_eq!(
                            bound["sha256"].as_str(),
                            Some(
                                lexlean::artifact::content_id::Sha256Digest::of(&text)
                                    .to_hex()
                                    .as_str()
                            ),
                            "{module}: the record binds the published certificate"
                        );
                    }
                }
            }
            let attestation: serde_json::Value = serde_json::from_slice(
                &std::fs::read(root.join("attestation.json")).expect("attestation"),
            )
            .expect("attestation JSON");
            assert_eq!(
                attestation["preservation"]["sha256"].as_str(),
                Some(
                    lexlean::artifact::content_id::Sha256Digest::of(&record_bytes)
                        .to_hex()
                        .as_str()
                ),
                "the attestation binds preservation.json"
            );
            for (fixture, code) in [
                ("certificate-rejected", "LLV7013"),
                ("certificate-b-rejected", "LLV7015"),
                ("certificate-e-rejected", "LLV7016"),
                ("certificate-heartbeat-rejected", "LLV7013"),
                ("certificate-resource-exhausted", "LLS8002"),
                ("preservation-drift", "LLV7014"),
            ] {
                let case =
                    crate::fixtures::load_case(&repo_root().join("tests/negative").join(fixture))
                        .expect("the fixture loads");
                let observed = crate::fixtures::observe(&case).expect("the fixture runs");
                assert_eq!(observed.codes, [code], "{fixture}");
                assert!(
                    !observed.project.root.join(".lexlean/verified").exists()
                        || std::fs::read_dir(
                            observed
                                .project
                                .root
                                .join(".lexlean/verified")
                                .as_std_path()
                        )
                        .map(|mut entries| entries.next().is_none())
                        .unwrap_or(true),
                    "{fixture}: nothing is published"
                );
            }
        }
        // §17.16, §17.17: the declared Rust machine.
        "SP-07" => {
            for (path, bytes) in crate::rust_source::files() {
                let committed =
                    std::fs::read(repo_root().join(&path).as_std_path()).expect("a source");
                assert_eq!(committed, bytes, "{path} equals its generator");
            }
            assert_eq!(
                crate::calculus::shipped_modules(repo_root().as_std_path(), false)
                    .expect("the shipped modules equal the compiler golden"),
                preserve::TARGET_MODULES.len()
            );
            assert!(preserve::TARGET_MODULES.contains(&"LexLeanTarget.RustSemantics"));
            // The machine declares exactly the renderer's runtime items, with
            // the renderer's failure and heap classes; a width is the
            // constructor's argument.
            let declared = crate::rust_source::item_constructors();
            let mut named: Vec<&str> = Vec::new();
            for item in lexlean::calculus::rust::runtime::Item::all() {
                let term = lexlean::production::rust_term::item(item);
                let spelled = term
                    .trim_start_matches('(')
                    .trim_start_matches('.')
                    .split(' ')
                    .next()
                    .expect("a constructor");
                let (name, fields) = declared
                    .iter()
                    .find(|(name, _)| *name == spelled)
                    .unwrap_or_else(|| panic!("{item:?} (`{term}`) is declared"));
                assert_eq!(fields.len(), usize::from(term.starts_with('(')), "{name}");
                assert_eq!(
                    crate::rust_source::item_classes(name),
                    Some((item.fallible(), item.heap())),
                    "{name}: the machine's classes equal the renderer's"
                );
                if !named.contains(name) {
                    named.push(name);
                }
            }
            let mut order: Vec<&str> = declared.iter().map(|(name, _)| *name).collect();
            named.sort_unstable();
            order.sort_unstable();
            assert_eq!(
                named, order,
                "the machine declares exactly the renderer's items"
            );
            // The machine aborts only for an item that cannot fail
            // (`runItem_abort_infallible`), and the only such item that can
            // overflow is a length of 2^64 elements or more: every other
            // infallible item of the primitive differential, on its boundary
            // values and seeded inputs, has an outcome the calculus does not
            // call an overflow.
            for profile in lexlean::calculus::rust::Profile::ALL {
                let differential = crate::rust_differential::differential(profile);
                let fallible = lexlean::calculus::rust::fallible_functions(&differential.program)
                    .expect("a valid program");
                let mut checked = 0;
                for (function, lists) in differential.arguments.iter().enumerate() {
                    if fallible[function] || differential.names[function].starts_with("length_") {
                        continue;
                    }
                    for list in lists {
                        let outcome = lexlean::calculus::interp::run(
                            &differential.program,
                            1_000_000,
                            function as u64,
                            list,
                        );
                        assert!(
                            !matches!(outcome, Outcome::Overflow { .. }),
                            "{}: the infallible item {} overflows on {list:?}",
                            profile.target(),
                            differential.names[function]
                        );
                        checked += 1;
                    }
                }
                assert!(
                    checked > 500,
                    "{}: {checked} infallible runs",
                    profile.target()
                );
            }
            // A drifted class is reported.
            assert_ne!(
                crate::rust_source::item_classes("natSub"),
                Some((true, false)),
                "natSub cannot fail"
            );
            if !support::lean_backed("SP-07") {
                return;
            }
            for (name, report) in reports() {
                let targets: usize = report
                    .certified
                    .iter()
                    .map(|entry| entry.targets.len())
                    .sum();
                assert_eq!(
                    report.crates.len(),
                    targets,
                    "{name}: one crate per root and target"
                );
                assert!(
                    report.machine > 0,
                    "{name}: the machine evaluated the cases"
                );
            }
        }
        // §17.17: certificate B.
        "SP-08" => {
            // A match has as many arms as its source states, and a limit may
            // allow thousands: the aligner derives 9000 of them, and writes
            // and drops the derivation, on the 2 MiB stack of a thread, so
            // it cannot depend on the depth of the match; and a limit below
            // the size stops it.
            let sizes = std::thread::Builder::new()
                .stack_size(2 << 20)
                .spawn(|| {
                    let project = crate::stress::wide_match(9000);
                    let sizes = preservation::aligned(&project, 64 << 20).expect("B within 64 MiB");
                    let refused = preservation::aligned(&project, 1 << 20)
                        .expect_err("B beyond 1 MiB is refused");
                    (sizes, refused)
                })
                .expect("a thread")
                .join()
                .expect("a match of 9000 arms does not overflow the stack");
            assert!(
                sizes.0.iter().all(|size| *size > 9000 * 100),
                "{:?}",
                sizes.0
            );
            assert!(
                sizes.1.starts_with(lexlean::production::lower::LIMIT),
                "{}",
                sizes.1
            );
            use repo_model::correspondence as corr;
            let read = |path: &str| {
                std::fs::read_to_string(repo_root().join(path).as_std_path()).expect("a source")
            };
            let aligner = read(corr::ALIGNER_SOURCE);
            let correspondence = read(corr::CORRESPONDENCE_SOURCE);
            let soundness = read(corr::SOUNDNESS_SOURCE);
            let calculus = read(corr::CALCULUS_SOURCE);
            corr::audit(&aligner, &correspondence, &soundness, &calculus)
                .expect("the rule set is closed");
            let rules: BTreeSet<String> = lexlean::production::rust_cert::RULES
                .iter()
                .map(|rule| (*rule).to_owned())
                .collect();
            assert_eq!(
                corr::constructors(&correspondence).expect("the constructors"),
                lexlean::production::rust_cert::RULES,
                "the aligner's rules are the correspondence's constructors, in order"
            );
            // Each break of the closure, planted in the repository's own
            // sources, is reported for what it is.
            for (rust, corr_text, sound, expected) in [
                (
                    aligner.replacen("    \"condId\",\n", "", 1),
                    correspondence.clone(),
                    soundness.clone(),
                    "`condId` is not among the aligner's rules",
                ),
                (
                    aligner.clone(),
                    correspondence.clone(),
                    soundness.replacen("| matchUnit", "| matchUnitX", 1),
                    "`matchUnit` is not a case of the soundness theorem",
                ),
                (
                    aligner.replacen(
                        "            Term::Apply { .. } => self.apply(g, fl, false, term, lets, tail),\n",
                        "            _ => self.apply(g, fl, false, term, lets, tail),\n",
                        1,
                    ),
                    correspondence.clone(),
                    soundness.clone(),
                    "wildcard arm",
                ),
            ] {
                assert!(
                    rust != aligner || corr_text != correspondence || sound != soundness,
                    "{expected}: the plant applies"
                );
                let report = corr::audit(&rust, &corr_text, &sound, &calculus)
                    .expect_err("a broken closure is reported");
                assert!(report.contains(expected), "{expected}: {report}");
            }
            // Every renderer fixture certifies in every profile that renders
            // it, from the declared rules only.
            // The roots with a boundary entry (§17.17) are rendered too: a
            // validator dropped from an entry's crate is a crate mutation.
            let entries = preservation::entry_renderings(&preservation::certificates(
                &P::copy_example("production-coverage"),
            ));
            assert!(!entries.is_empty(), "a coverage root has a boundary entry");
            let mut renderings = preservation::fixture_renderings();
            renderings.extend(entries);
            let mut used: BTreeSet<String> = BTreeSet::new();
            for rendering in &renderings {
                used.extend(preservation::rules_used(&rendering.certificate.text));
            }
            assert!(
                used.is_subset(&rules),
                "{:?}",
                used.difference(&rules).collect::<Vec<_>>()
            );
            // With the certified roots' renderings, every rule is used: no
            // rule of the closed set is dead weight the corpus never meets.
            for (_, project) in certified_projects() {
                for entry in preservation::certificates(&project) {
                    for (_, certificate) in &entry.renderings {
                        used.extend(preservation::rules_used(&certificate.text));
                    }
                }
            }
            assert_eq!(
                used,
                rules,
                "rules no rendering uses: {:?}",
                rules.difference(&used).collect::<Vec<_>>()
            );
            // A crate mutated after rendering is refused: the aligner finds
            // no derivation, or Lean rejects the one it writes; and the
            // unmutated derivation never proves the mutated crate.
            let planted = preservation::plant_renderings(&renderings);
            let kinds: BTreeSet<preservation::RustMutation> =
                planted.iter().map(|(plant, _)| plant.mutation).collect();
            assert_eq!(
                kinds,
                preservation::RustMutation::ALL.into_iter().collect(),
                "every crate mutation applies to some rendering"
            );
            // A width change is refused by certificate B below, and also by
            // the renderer's correspondence check, before any crate exists.
            let widened = renderings
                .iter()
                .find_map(|rendering| {
                    preservation::mutate_crate(&rendering.krate, preservation::RustMutation::Width)
                        .map(|mutated| (rendering, mutated))
                })
                .expect("a rendering has a checked operation");
            let elements = lexlean::calculus::rust::realized(
                &widened.0.program,
                lexlean::calculus::rust::Profile::named(&widened.0.target).expect("a profile"),
            )
            .expect("the program is realized");
            lexlean::calculus::rust::validate::correspond(&widened.0.krate, &elements)
                .expect("the rendering corresponds");
            let refusal = lexlean::calculus::rust::validate::correspond(&widened.1, &elements)
                .expect_err("a construct at another width is refused");
            assert!(refusal.contains("works at width"), "{refusal}");
            eprintln!(
                "SP-08: {} renderer-fixture renderings certified",
                renderings.len()
            );
            if !support::lean_backed("SP-08") {
                return;
            }
            for (name, report) in reports() {
                for entry in &report.certified {
                    let targets: Vec<&String> =
                        entry.renderings.iter().map(|(target, _)| target).collect();
                    assert_eq!(
                        targets,
                        entry.targets.iter().collect::<Vec<_>>(),
                        "{name}: {} has certificate B in each target",
                        entry.root
                    );
                    for (_, certificate) in &entry.renderings {
                        assert!(
                            report.audit_output.contains(&certificate.theorem),
                            "{name}: `{}` is audited",
                            certificate.theorem
                        );
                    }
                }
            }
            let checked = preservation::check_renderings(&renderings, planted);
            for rendering in &renderings {
                assert!(
                    checked
                        .audit_output
                        .contains(&rendering.certificate.theorem),
                    "`{}` is audited",
                    rendering.certificate.theorem
                );
            }
            // Every mutation is refused where it was planted: the aligner
            // names the mutated function, or Lean's first error, on the
            // realigned derivation and on the unmutated derivation restated
            // over the mutated crate, lies in the derivation `fun<k>` of
            // that function (the unmutated crate's derivations all compiled
            // above).
            for plant in &checked.planted {
                let what = format!(
                    "{:?} at {}:{} in {} ({})",
                    plant.mutation, plant.function, plant.ordinal, plant.fixture, plant.target
                );
                let expected = format!("fun{}", plant.function);
                assert_eq!(
                    plant.stale_declaration.as_deref(),
                    Some(expected.as_str()),
                    "{what}: the unmutated derivation restated over the mutated crate:\n{}",
                    plant.stale
                );
                match (&plant.unaligned, &plant.realigned) {
                    (Some(reason), None) => assert!(
                        reason.starts_with(&format!("function {}:", plant.function)),
                        "{what}: the aligner's refusal does not name the function: {reason}"
                    ),
                    (None, Some(rejection)) => assert_eq!(
                        plant.realigned_declaration.as_deref(),
                        Some(expected.as_str()),
                        "{what}: the realigned derivation:\n{rejection}"
                    ),
                    (unaligned, realigned) => {
                        panic!("{what}: {unaligned:?} / {realigned:?}")
                    }
                }
            }
            // Each kind is planted at its first, a middle, and its last
            // place; a validation at every validated parameter.
            let kinds: BTreeSet<preservation::RustMutation> =
                checked.planted.iter().map(|plant| plant.mutation).collect();
            assert_eq!(kinds, preservation::RustMutation::ALL.into_iter().collect());
            let validated: usize =
                crate::differential::carriers(&P::copy_example("production-coverage"))
                    .values()
                    .map(Vec::len)
                    .sum();
            assert_eq!(
                checked
                    .planted
                    .iter()
                    .filter(|plant| plant.mutation == preservation::RustMutation::Validation)
                    .count(),
                validated,
                "a validation mutation per validated parameter of an entry crate"
            );
        }
        // §17.12, §17.17: the boundary validators and the entry.
        "SP-10" => {
            use lexlean::calculus::{Expr, Value};
            use lexlean::production::lower::{audit_boundary, Origin, Validation};
            let mut entries = BTreeSet::new();
            let mut invalid = 0;
            for (name, project) in certified_projects() {
                let checked = support::checked_project(&project);
                let modules = linked_modules(&checked);
                let limits = support::limits(&project);
                let oracle = crate::differential::carriers(&project);
                let cases = crate::differential::cases(&project);
                for root in roots(&checked).expect("the eligibility reports") {
                    let lowered =
                        lower_root(&modules, &root.module, &root.name, root.report, &limits)
                            .unwrap_or_else(|d| panic!("{name}: {}: {d:?}", root.report.root));
                    // The lowering audited it already; the audit is the
                    // gate, so it is also run here on what it returned.
                    audit_boundary(&lowered)
                        .unwrap_or_else(|reason| panic!("{}: {reason}", root.report.root));
                    let validators = lowered
                        .layout
                        .functions
                        .iter()
                        .filter(|origin| matches!(origin, Origin::Validator { .. }))
                        .count();
                    assert_eq!(
                        lowered.entry() != 0,
                        validators > 0,
                        "{}: an entry exactly when a validator exists",
                        root.report.root
                    );
                    assert_eq!(
                        lowered.entry() != 0,
                        !oracle[&root.report.root].is_empty(),
                        "{}: an entry exactly when a parameter holds a map or a set",
                        root.report.root
                    );
                    if lowered.entry() == 0 {
                        continue;
                    }
                    entries.insert(root.name.clone());
                    // An entry exists for exactly the roots with a parameter
                    // the test's own oracle says carries the invariant, and
                    // a lowering that skipped a validator is refused by the
                    // certificate generator, which judges the boundary from
                    // the source types: a validated parameter whose check is
                    // omitted, and a validator that checks nothing.
                    let carried = &oracle[&root.report.root];
                    assert!(
                        !carried.is_empty(),
                        "{}: an entry for no carrier",
                        root.report.root
                    );
                    let generate = |lowered: &lexlean::production::lower::Lowered| {
                        lexlean::production::certificate::certificate(
                            &modules,
                            &root.module,
                            &root.name,
                            root.report,
                            lowered,
                            "LexLeanPreserve.Probe",
                            u64::MAX,
                        )
                    };
                    generate(&lowered).expect("the lowering's own boundary is accepted");
                    for position in carried {
                        let skipped = preservation::skip_parameter_validator(&lowered, *position);
                        let refusal = generate(&skipped)
                            .expect_err("a skipped parameter validator is refused");
                        assert!(
                            format!("{refusal:?}").contains(&format!("parameter {position}: "))
                                && format!("{refusal:?}").contains("does not validate"),
                            "{}: {refusal:?}",
                            root.report.root
                        );
                    }
                    for (index, origin) in lowered.layout.functions.iter().enumerate() {
                        if let Origin::Validator { kind, .. } = origin {
                            if *kind == Validation::Trivial {
                                continue;
                            }
                            let trivial = preservation::trivialize_validator(&lowered, index);
                            let refusal = generate(&trivial)
                                .expect_err("a validator that checks nothing is refused");
                            assert!(
                                format!("{refusal:?}").contains("checks nothing")
                                    || format!("{refusal:?}").contains("is trivial"),
                                "{} validator {index}: {refusal:?}",
                                root.report.root
                            );
                        }
                    }
                    // Every validated parameter is a boundary position: a
                    // validator exists only for a type of the signature.
                    for case in cases.get(&root.report.root).map_or(&[][..], Vec::as_slice) {
                        if case.function == 0 {
                            continue;
                        }
                        match (&case.outcome, case.invalid.is_some()) {
                            (lexlean::calculus::Outcome::Value { value, steps: _ }, true) => {
                                assert_eq!(
                                    *value,
                                    Value::None,
                                    "{}: an invalid input is refused with none",
                                    root.report.root
                                );
                                invalid += 1;
                            }
                            (lexlean::calculus::Outcome::Value { value, steps: _ }, false) => {
                                assert!(
                                    matches!(value, Value::Some { .. }),
                                    "{}: a valid input is accepted",
                                    root.report.root
                                );
                            }
                            (lexlean::calculus::Outcome::Overflow { steps: _ }, false) => {}
                            (outcome, invalid) => {
                                panic!("{}: {outcome:?} (invalid {invalid})", root.report.root)
                            }
                        }
                    }
                }
            }
            for root in ["mapOps", "setOps", "graphs", "groveRoot", "stringGraph"] {
                assert!(entries.contains(root), "`{root}` has a boundary entry");
            }
            assert!(
                !entries.contains("natOps"),
                "a root without a map or set has none"
            );
            assert!(invalid >= 5, "{invalid} invalid inputs refused");
            // The recursive tree's validators are one mutual group of
            // structural recursions, as its encoders are, each decided by
            // the library's proposition of its container.
            let coverage = P::copy_example("production-coverage");
            let certified = preservation::certificates(&coverage);
            let grove = certified
                .iter()
                .find(|entry| entry.root.ends_with("groveRoot"))
                .expect("the recursive-tree root is certified");
            for needed in [
                "mutual\ndef __valid_",
                "theorem __viff_",
                "theorem __vrel_",
                "theorem __vinv_",
                "LexLeanPreservation.invList_eqs",
                "theorem entry_accepts",
                "theorem entry_refuses",
                "def accepts",
            ] {
                assert!(
                    grove.certificate.text.contains(needed),
                    "the recursive tree's certificate states `{needed}`"
                );
            }
            // A validator called from inside the program is refused by the
            // lowering's audit, as is an entry the validators do not match.
            let checked = support::checked_project(&coverage);
            let modules = linked_modules(&checked);
            let limits = support::limits(&coverage);
            let report = roots(&checked)
                .expect("the eligibility reports")
                .into_iter()
                .find(|root| root.name == "groveRoot")
                .expect("the root");
            let lowered = lower_root(
                &modules,
                &report.module,
                &report.name,
                report.report,
                &limits,
            )
            .expect("the root lowers");
            let validator = lowered
                .layout
                .functions
                .iter()
                .position(|origin| matches!(origin, Origin::Validator { .. }))
                .expect("a validator") as u64;
            let mut inside = lowered.clone();
            inside.program.functions[0].body = Expr::Call {
                function: validator,
                operands: Vec::new(),
            };
            let reason = audit_boundary(&inside).expect_err("a validator inside the program");
            assert!(reason.contains("runs only at the entry"), "{reason}");
            let mut headless = lowered.clone();
            headless.program.functions.pop();
            headless.layout.functions.pop();
            let reason = audit_boundary(&headless).expect_err("validators with no entry");
            assert!(reason.contains("0 entries"), "{reason}");
            let mut reaching = lowered.clone();
            let first = reaching
                .layout
                .functions
                .iter()
                .position(|origin| matches!(origin, Origin::Validator { .. }))
                .expect("a validator");
            reaching.program.functions[first].body = Expr::Call {
                function: 1,
                operands: Vec::new(),
            };
            let reason = audit_boundary(&reaching).expect_err("a validator reaching the program");
            assert!(reason.contains("references function 1"), "{reason}");
            assert!(
                lowered.layout.functions.iter().any(|origin| matches!(
                    origin,
                    Origin::Validator {
                        kind: Validation::Document { .. },
                        ..
                    }
                )),
                "a document type has a validator"
            );
            if !support::lean_backed("SP-10") {
                return;
            }
            let planted = preservation::plant(&coverage, &Mutation::BOUNDARY);
            let kinds: BTreeSet<Mutation> = planted.iter().map(|plant| plant.mutation).collect();
            assert_eq!(
                kinds,
                Mutation::BOUNDARY.into_iter().collect(),
                "every boundary mutation is planted in the coverage example"
            );
            for plant in &planted {
                assert!(
                    !plant.rejection.is_empty(),
                    "{}: the certificate of a {:?} mutation was accepted",
                    plant.root,
                    plant.mutation
                );
                assert_targeted(plant);
            }
            // Every validated parameter of every entry was planted in turn,
            // and the entry of each was probed with an input breaking that
            // parameter's invariant.
            let carriers = crate::differential::carriers(&coverage);
            let validated: usize = carriers.values().map(Vec::len).sum();
            assert!(validated > 15, "{validated} validated parameters");
            assert_eq!(
                planted
                    .iter()
                    .filter(|plant| plant.mutation == Mutation::Validation)
                    .count(),
                validated,
                "a validation mutation per validated parameter"
            );
            let cases = crate::differential::cases(&coverage);
            for (root, positions) in &carriers {
                for position in positions {
                    assert!(
                        cases.get(root).is_some_and(|cases| cases
                            .iter()
                            .any(|case| case.invalid == Some(*position))),
                        "`{root}`: an input that breaks the invariant of parameter {position}"
                    );
                }
            }
            for (name, report) in reports() {
                for entry in &report.certified {
                    if entry.entry == 0 {
                        continue;
                    }
                    assert!(
                        report.audit_output.contains(&entry.certificate.theorem),
                        "{name}: `{}` is audited",
                        entry.certificate.theorem
                    );
                }
            }
        }
        // §17.16, §17.17: rustc agrees with the declared machine.
        "SP-11" => {
            let mut crates = 0;
            let mut runs = 0;
            let mut entered = 0;
            for (name, project) in certified_projects() {
                let certified = preservation::certificates(&project);
                let run = preservation::rustc_differential(&project, &certified, name);
                eprintln!(
                    "SP-11: {name}: {} crates, {} runs, {} through an entry",
                    run.crates, run.runs, run.entered
                );
                crates += run.crates;
                runs += run.runs;
                entered += run.entered;
            }
            assert!(
                crates > 60 && runs > 400 && entered > 80,
                "{crates} {runs} {entered}"
            );
        }
        // §17.17: certificate E.
        "SP-09" => {
            // Roots whose parameters are spelled like the names the
            // composition binds and like forbidden tokens are valid
            // programs: their certificates E are stated in full, bind no
            // name a parameter could capture, and pass the token audit.
            let mut referenced: BTreeSet<String> = BTreeSet::new();
            for (_, project) in certified_projects() {
                let users = user_names(&project);
                for entry in preservation::certificates(&project) {
                    referenced.extend(referenced_namespaces(&entry.certificate.text, &users));
                    for (_, rendering) in &entry.renderings {
                        referenced.extend(referenced_namespaces(&rendering.text, &users));
                    }
                    for (_, composed) in &entry.composed {
                        referenced.extend(referenced_namespaces(&composed.text, &users));
                    }
                }
            }
            for namespace in [
                "LexLeanPreservation",
                "LexLeanTarget",
                "LexLeanPreserve",
                "Corr",
            ] {
                assert!(
                    referenced.contains(namespace),
                    "the certificates of the corpora begin names with `{namespace}`: {referenced:?}"
                );
            }
            let referenced: Vec<String> = referenced.into_iter().collect();
            let names = crate::stress::names(&referenced);
            let named = preservation::certificates(&names);
            let _staged = preservation::staged(&names, &named);
            // The parameters are spelled like the namespaces of the library:
            // every reference the certificates make to one is written from
            // the root, which the gate sees because it reads the first
            // component of a longer name even when a parameter has it.
            let named_users = user_names(&names);
            let named_root = prefix_root(&names);
            for entry in &named {
                assert_no_bare_globals(
                    &entry.certificate.module,
                    &entry.certificate.text,
                    &named_users,
                    &named_root,
                );
                for (_, composed) in &entry.composed {
                    assert_no_bare_globals(
                        &composed.module,
                        &composed.text,
                        &named_users,
                        &named_root,
                    );
                }
            }
            for entry in &named {
                for ((_, composed), (_, rendering)) in entry.composed.iter().zip(&entry.renderings)
                {
                    assert_statement("names", entry, rendering, composed);
                }
            }
            for (name, project) in certified_projects() {
                let users = user_names(&project);
                let root = prefix_root(&project);
                for entry in preservation::certificates(&project) {
                    assert_no_bare_globals(
                        &entry.certificate.module,
                        &entry.certificate.text,
                        &users,
                        &root,
                    );
                    let targets: Vec<&String> =
                        entry.composed.iter().map(|(target, _)| target).collect();
                    assert_eq!(
                        targets,
                        entry.targets.iter().collect::<Vec<_>>(),
                        "{name}: {} has certificate E in each target",
                        entry.root
                    );
                    for ((_, composed), (_, rendering)) in
                        entry.composed.iter().zip(&entry.renderings)
                    {
                        // E composes exactly this root's A and B.
                        for module in [&entry.certificate.module, &rendering.module] {
                            assert!(
                                composed
                                    .text
                                    .lines()
                                    .any(|line| line == format!("import {module}")),
                                "{name}: `{}` imports `{module}`",
                                composed.module
                            );
                        }
                        assert_eq!(composed.denote, entry.certificate.denote);
                        // E is stated, in full, for the arguments a Rust caller
                        // can pass only: the binders are the root's parameters,
                        // the one hypothesis is that their encodings are
                        // representable, and the conclusion is the two
                        // statements of §17.17 and nothing else.
                        assert_statement(name, &entry, rendering, composed);
                        assert_no_bare_globals(&composed.module, &composed.text, &users, &root);
                        for mutation in preservation::CompositionMutation::ALL {
                            let places = mutation.places(&composed.text);
                            assert!(places > 0, "{mutation:?} applies to `{}`", composed.module);
                            for nth in 0..places {
                                let planted = mutation
                                    .plant(&composed.text, nth)
                                    .expect("a counted place");
                                assert_ne!(planted, composed.text, "{mutation:?} {nth}");
                            }
                        }
                    }
                }
            }
            if !support::lean_backed("SP-09") {
                return;
            }
            support::verify_ok(&names);
            // Declarations spelled like the words the generated-Lean audit
            // forbids verify as well: the audit module names them quoted.
            let declared = crate::stress::declared_names();
            for entry in preservation::certificates(&declared) {
                assert!(
                    entry.certificate.text.contains("«native_decide»"),
                    "certificate A quotes the declaration `native_decide`"
                );
            }
            support::verify_ok(&declared);
            let mut at_bounds = 0;
            for (name, report) in reports() {
                // Each certificate E is applied, in Lean, to arguments a
                // Rust caller can pass, some of them at the bounds of
                // `u64` and `i64`, so a hypothesis that cannot be met or
                // used fails.
                let modules: usize = report
                    .certified
                    .iter()
                    .map(|entry| entry.composed.len())
                    .sum();
                assert!(
                    report.witnessed.0 >= modules,
                    "{name}: {} witnesses for {modules} certificates E",
                    report.witnessed.0
                );
                at_bounds += report.witnessed.1;
                for entry in &report.certified {
                    for (_, composed) in &entry.composed {
                        assert!(
                            report.audit_output.contains(&composed.theorem),
                            "{name}: `{}` is audited",
                            composed.theorem
                        );
                    }
                    let kinds: BTreeSet<preservation::CompositionMutation> = report
                        .composed_plants
                        .iter()
                        .filter(|plant| plant.root == entry.root)
                        .map(|plant| plant.mutation)
                        .collect();
                    assert_eq!(
                        kinds,
                        preservation::CompositionMutation::ALL.into_iter().collect(),
                        "{name}: every composition defect is planted in {}",
                        entry.root
                    );
                }
                for plant in &report.composed_plants {
                    assert_not_a_limit(
                        &format!("{} {:?}", plant.root, plant.mutation),
                        &plant.output,
                        plant.text_len,
                    );
                    assert!(
                        !plant.accepted
                            && plant
                                .declaration
                                .as_deref()
                                .is_some_and(|declaration| declaration
                                    .starts_with(plant.mutation.declaration_prefix())),
                        "{name}: {:?} at {} in {} ({}) is not refused in `{}*`: {:?}\n{}",
                        plant.mutation,
                        plant.nth,
                        plant.root,
                        plant.target,
                        plant.mutation.declaration_prefix(),
                        plant.declaration,
                        plant.output
                    );
                }
            }
            assert!(
                at_bounds > 0,
                "some witness applies E at the bounds of u64 or i64"
            );
        }
        // §17.13, §17.17: the certified examples exercise the registry.
        "SP-06" => {
            let mut held = Vec::new();
            for (_, project) in certified_projects() {
                held.push(support::checked_project(&project));
            }
            let all: Vec<_> = held
                .iter()
                .map(|checked| roots(checked).expect("the reports"))
                .collect();
            let reports: Vec<&lexlean::production::RootReport> =
                all.iter().flatten().map(|root| root.report).collect();
            let missing = unexercised(&reports);
            assert!(
                missing.is_empty(),
                "runtime constructs no certified root exercises: {missing:?}"
            );
            // Withholding the collection roots leaves their constructs
            // unexercised, and the check says so.
            let withheld: Vec<&lexlean::production::RootReport> = reports
                .iter()
                .copied()
                .filter(|report| !report.root.starts_with("Coverage.Colls."))
                .collect();
            let missing = unexercised(&withheld);
            assert!(
                missing.iter().any(|key| key == "primitive.map_insert"),
                "withheld collection roots are reported: {missing:?}"
            );
        }
        _ => panic!("no preservation case is wired for {id}"),
    }
}
